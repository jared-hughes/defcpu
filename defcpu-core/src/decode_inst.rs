use crate::{
    errors::RResult,
    inst::{
        rot_pair, Base32, Base64, DataSize, DisInst, EffAddr, FullInst, Group1Prefix,
        Group1PrefixExec, Index32, Index64,
        Inst::{self, *},
        JumpXor::*,
        OneOp, RotDir, RotType,
        Scale::{self, *},
        TwoOp, RM16, RM32, RM64, RM8, SIDB32, SIDB64,
    },
    inst_prefixes::{
        AddressSizeAttribute::{self, *},
        OperandSizeAttribute::{self, *},
        Prefix,
    },
    memory::Memory,
    registers::{GPR16, GPR32, GPR64, GPR8},
};

struct Lexer<'a> {
    mem: &'a Memory,
    prefix: Prefix,
    i: u64,
    i_start: u64,
    rex_w_mattered: bool,
    rex_r_mattered: bool,
    rex_x_mattered: bool,
    rex_b_mattered: bool,
    rex_presence_mattered: bool,
}

impl<'a> Lexer<'a> {
    fn new(mem: &Memory, i: u64) -> Lexer {
        Lexer {
            mem,
            i,
            i_start: i,
            prefix: Prefix::new(),
            rex_w_mattered: false,
            rex_r_mattered: false,
            rex_x_mattered: false,
            rex_b_mattered: false,
            rex_presence_mattered: false,
        }
    }

    fn len(&self) -> u64 {
        self.i - self.i_start
    }

    fn maybe_remove_rex(&mut self) {
        if !self.should_show_rex() {
            self.prefix.dis_prefix.remove_rex_prefix();
        }
    }

    /// Only use in places where REX.W matters, and 16/32/64 are all valid sizes.
    /// Call only once per instruction decode, since this deletes an operand size prefix, if present.
    fn get_operand_size(&mut self) -> OperandSizeAttribute {
        if let Some(rex) = self.prefix.rex {
            self.rex_w_mattered = true;
            self.maybe_remove_rex();
            if rex.w {
                return Data64;
            }
        }
        self.get_operand_size_no_rexw()
    }

    /// Call only once per instruction decode, since this deletes an operand size prefix, if present.
    fn get_operand_size_no_rexw(&mut self) -> OperandSizeAttribute {
        self.prefix.dis_prefix.remove_last_operand_size_prefix();
        self.prefix.operand_size
    }

    /// This is used in one particular case that supports u16 and u64 but not u32.
    /// So the default is u64. The 0x66 operand-size prefix converts it to u16,
    /// but the REX.W takes precedence in keeping as u64. Since the default
    /// is u64 though, the REX.W (0x48) just undoes the 0x66, so
    /// `66 48 op` is the same as `op`; we don't count the REX.W as mattering,
    /// so it and the operand-size still get printed.
    fn get_operand_size_64_default(&mut self) -> OperandSizeAttribute {
        if let Some(rex) = self.prefix.rex {
            if rex.w {
                return Data64;
            }
        }
        match self.get_operand_size_no_rexw() {
            Data16 => Data16,
            Data32 => Data64,
            Data64 => panic!("Impossible Data64 without REX.W bit."),
        }
    }

    /// Call only once per instruction decode, since this deletes an address size prefix, if present.
    fn get_address_size(&mut self) -> AddressSizeAttribute {
        self.prefix.dis_prefix.remove_last_address_size_prefix();
        self.prefix.address_size
    }

    fn reg8_field_select_r(&mut self, op: u8) -> GPR8 {
        let (reg, rex_r_matters) = reg8_field_select(op, self.prefix.rex.map(|x| x.r));
        self.rex_r_mattered |= rex_r_matters;
        self.rex_presence_mattered |= rex_r_matters;
        self.maybe_remove_rex();
        reg
    }

    fn reg8_field_select_b(&mut self, op: u8) -> GPR8 {
        let (reg, rex_b_matters) = reg8_field_select(op, self.prefix.rex.map(|x| x.b));
        self.rex_b_mattered |= rex_b_matters;
        self.rex_presence_mattered |= rex_b_matters;
        self.maybe_remove_rex();
        reg
    }

    /// Gives the the REX.X bit and marks it as relevant.
    /// Do not use this method if it's possible for REX.X=0 and REX.X=1
    /// to lead to the same behavior in the callee.
    fn get_rex_r_matters(&mut self) -> bool {
        self.rex_r_mattered = true;
        self.maybe_remove_rex();
        self.prefix.rex.map(|r| r.r).unwrap_or(false)
    }

    /// Gives the the REX.X bit and marks it as relevant.
    /// Do not use this method if it's possible for REX.X=0 and REX.X=1
    /// to lead to the same behavior in the callee.
    fn get_rex_x_matters(&mut self) -> bool {
        self.rex_x_mattered = true;
        self.maybe_remove_rex();
        self.prefix.rex.map(|r| r.x).unwrap_or(false)
    }

    /// Gives the the REX.B bit and marks it as relevant.
    /// Do not use this method if it's possible for REX.B=0 and REX.B=1
    /// to lead to the same behavior in the callee.
    fn get_rex_b_matters(&mut self) -> bool {
        self.rex_b_mattered = true;
        self.maybe_remove_rex();
        self.prefix.rex.map(|r| r.b).unwrap_or(false)
    }

    /// Do we want to show the REX in disassembly?
    /// If the REX sets a bit that didn't actually affect anything, then yes.
    /// If the REX's presense doesn't actually affect anything, then yes.
    fn should_show_rex(&self) -> bool {
        if let Some(rex) = self.prefix.rex {
            if rex.w && !self.rex_w_mattered {
                return true;
            }
            if rex.r && !self.rex_r_mattered {
                return true;
            }
            if rex.x && !self.rex_x_mattered {
                return true;
            }
            if rex.b && !self.rex_b_mattered {
                return true;
            }
            let rex_presence_mattered = self.rex_presence_mattered;
            let rex_blank = !rex.w && !rex.r && !rex.x && !rex.b;
            if rex_blank && !rex_presence_mattered {
                return true;
            }
        }
        false
    }

    fn rollback(&mut self) {
        self.i -= 1;
    }

    fn next_u8(&mut self) -> RResult<u8> {
        let out = self.mem.read_u8(self.i)?;
        self.i += 1;
        Ok(out)
    }

    fn next_imm8(&mut self) -> RResult<u8> {
        let out = self.mem.read_u8(self.i)?;
        self.i += 1;
        Ok(out)
    }

    fn next_modrm(&mut self) -> RResult<ModRM> {
        let byte = self.next_u8()?;
        Ok(modrm_decode(byte))
    }

    fn next_i8(&mut self) -> RResult<i8> {
        // i8 and u8 are 2's complement, so this is a no-op.
        Ok(self.next_imm8()? as i8)
    }

    fn next_imm16(&mut self) -> RResult<u16> {
        let out = self.mem.read_u16(self.i)?;
        self.i += 2;
        Ok(out)
    }

    fn next_i16(&mut self) -> RResult<i16> {
        Ok(self.next_imm16()? as i16)
    }

    fn next_imm32(&mut self) -> RResult<u32> {
        let out = self.mem.read_u32(self.i)?;
        self.i += 4;
        Ok(out)
    }

    fn next_i32(&mut self) -> RResult<i32> {
        Ok(self.next_imm32()? as i32)
    }

    fn next_imm64(&mut self) -> RResult<u64> {
        let out = self.mem.read_u64(self.i)?;
        self.i += 8;
        Ok(out)
    }
}

/// Returns instruction together with the number of bytes read.
pub fn decode_inst(mem: &Memory, i: u64) -> RResult<(DisInst, u64)> {
    let mut lex = Lexer::new(mem, i);
    let inst = decode_inst_inner(&mut lex)?;
    let len = lex.len();
    let full_inst = FullInst {
        group1_prefix: lex.prefix.group1_prefix.map(|prefix| {
            let addr_size = lex.get_address_size();
            match prefix {
                // TODO-correctness: How do we know when it's REP instead of REPZ?
                Group1Prefix::Repz => Group1PrefixExec::Repz(addr_size),
                Group1Prefix::Repnz => Group1PrefixExec::Repnz(addr_size),
            }
        }),
        main_inst: inst,
    };
    let dis_inst = DisInst {
        prefix: lex.prefix.dis_prefix,
        inner: full_inst,
    };
    Ok((dis_inst, len))
}

fn decode_inst_inner(lex: &mut Lexer) -> RResult<Inst> {
    let opcode = lex.next_u8()?;
    Ok(match opcode {
        //
        // ============ prefixes ============
        //
        0x40..=0x4F => {
            // 0x40..=0x4F REX prefix. Must be immediately followed by opcode byte.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return Ok(RexNoop);
            }
            lex.prefix = lex.prefix.with_rex(opcode);
            decode_inst_inner(lex)?
        }
        0x66 => {
            // 0x66 Operand size prefix.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return Ok(RexNoop);
            }
            lex.prefix = lex.prefix.with_operand_size_prefix();
            decode_inst_inner(lex)?
        }
        0x67 => {
            // 0x67 Address size prefix.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return Ok(RexNoop);
            }
            lex.prefix = lex.prefix.with_address_size_prefix();
            decode_inst_inner(lex)?
        }
        0xF2 => {
            // 0xF2 Repnz/Repne prefix
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return Ok(RexNoop);
            }
            lex.prefix = lex.prefix.with_group1_prefix(Group1Prefix::Repnz);
            decode_inst_inner(lex)?
        }
        0xF3 => {
            // 0xF3 Rep/Repe/Repz prefix
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return Ok(RexNoop);
            }
            lex.prefix = lex.prefix.with_group1_prefix(Group1Prefix::Repz);
            decode_inst_inner(lex)?
        }
        //
        // ============ regular opcodes ============
        //
        0x00 | 0x02 | 0x20 | 0x22 | 0x28 | 0x2A | 0x30 | 0x32 | 0x38 | 0x3A | 0x88 | 0x8A => {
            // Encoding /r, with 8-bit operand size. Proto:
            // 00 /r; ADD r/m8, r8; MR; Valid; Valid; Add r8 to r/m8.
            // 02 /r; ADD r8, r/m8; RM; Valid; Valid; Add r/m8 to r8.
            let modrm = lex.next_modrm()?;
            let reg = lex.reg8_field_select_r(modrm.reg3);
            let rm8 = decode_rm8(lex, &modrm)?;
            let (op, reverse) = match opcode {
                0x00 => (TwoOp::ADD, false),
                0x02 => (TwoOp::ADD, true),
                0x20 => (TwoOp::AND, false),
                0x22 => (TwoOp::AND, true),
                0x28 => (TwoOp::SUB, false),
                0x2A => (TwoOp::SUB, true),
                0x30 => (TwoOp::XOR, false),
                0x32 => (TwoOp::XOR, true),
                0x38 => (TwoOp::CMP, false),
                0x3A => (TwoOp::CMP, true),
                0x88 => (TwoOp::MOV, false),
                0x8A => (TwoOp::MOV, true),
                _ => unreachable!(),
            };
            match reverse {
                false => TwoMRInst8(op, rm8, reg),
                true => TwoRMInst8(op, reg, rm8),
            }
        }
        0x01 | 0x03 | 0x21 | 0x23 | 0x29 | 0x2B | 0x31 | 0x33 | 0x39 | 0x3B | 0x89 | 0x8B => {
            // Encoding /r, with 16/32/64-bit operand size. Proto:
            // 01 /r; ADD r/m16, r16; MR; Valid; Valid; Add r16 to r/m16.
            // 01 /r; ADD r/m32, r32; MR; Valid; Valid; Add r32 to r/m32.
            // REX.W + 01 /r; ADD r/m64, r64; MR; Valid; N.E.; Add r64 to r/m64.
            // 03 /r; ADD r16, r/m16; RM; Valid; Valid; Add r/m16 to r16.
            // 03 /r; ADD r32, r/m32; RM; Valid; Valid; Add r/m32 to r32.
            // REX.W + 03 /r; ADD r64, r/m64; RM; Valid; N.E.; Add r/m64 to r64.
            let modrm = lex.next_modrm()?;
            let rex_r = lex.get_rex_r_matters();
            let (op, reverse) = match opcode {
                0x01 => (TwoOp::ADD, false),
                0x03 => (TwoOp::ADD, true),
                0x21 => (TwoOp::AND, false),
                0x23 => (TwoOp::AND, true),
                0x29 => (TwoOp::SUB, false),
                0x2B => (TwoOp::SUB, true),
                0x31 => (TwoOp::XOR, false),
                0x33 => (TwoOp::XOR, true),
                0x39 => (TwoOp::CMP, false),
                0x3B => (TwoOp::CMP, true),
                0x89 => (TwoOp::MOV, false),
                0x8B => (TwoOp::MOV, true),
                _ => unreachable!(),
            };
            match lex.get_operand_size() {
                Data16 => {
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm)?;
                    match reverse {
                        false => TwoMRInst16(op, rm16, reg),
                        true => TwoRMInst16(op, reg, rm16),
                    }
                }
                Data32 => {
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm)?;
                    match reverse {
                        false => TwoMRInst32(op, rm32, reg),
                        true => TwoRMInst32(op, reg, rm32),
                    }
                }
                Data64 => {
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm)?;
                    match reverse {
                        false => TwoMRInst64(op, rm64, reg),
                        true => TwoRMInst64(op, reg, rm64),
                    }
                }
            }
        }
        0x0F => {
            let opcode2 = lex.next_u8()?;
            match opcode2 {
                0x05 => {
                    // 0F 05
                    Syscall
                }
                0x80..=0x8F => {
                    // 0F 80 ... 0F 8F
                    let dest = match lex.get_operand_size_no_rexw() {
                        Data16 => {
                            let rel_off = lex.next_i16()? as u16;
                            // If the operand-size attribute is 16, the upper two bytes of the EIP register
                            // are cleared, resulting in a maximum instruction pointer size of 16 bits.
                            (lex.i as u16).wrapping_add(rel_off) as u64
                        }
                        Data32 => {
                            let rel_off = lex.next_i32()? as u64;
                            lex.i.wrapping_add(rel_off)
                        }
                        Data64 => panic!("Impossible Data64 without REX.W bit."),
                    };
                    match opcode2 {
                        0x80 => JccJo(dest, Normal),
                        0x81 => JccJo(dest, Negate),
                        0x82 => JccJb(dest, Normal),
                        0x83 => JccJb(dest, Negate),
                        0x84 => JccJe(dest, Normal),
                        0x85 => JccJe(dest, Negate),
                        0x86 => JccJbe(dest, Normal),
                        0x87 => JccJbe(dest, Negate),
                        0x88 => JccJs(dest, Normal),
                        0x89 => JccJs(dest, Negate),
                        0x8A => JccJp(dest, Normal),
                        0x8B => JccJp(dest, Negate),
                        0x8C => JccJl(dest, Normal),
                        0x8D => JccJl(dest, Negate),
                        0x8E => JccJle(dest, Normal),
                        0x8F => JccJle(dest, Negate),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                0xA3 => {
                    // 0F A3
                    let modrm = lex.next_modrm()?;
                    let rex_r = lex.get_rex_r_matters();
                    match lex.get_operand_size() {
                        Data16 => {
                            // 0F A3 /r; BT r/m16, r16; Store selected bit in CF flag.
                            let reg = reg16_field_select(modrm.reg3, rex_r);
                            let rm16 = decode_rm16(lex, &modrm)?;
                            BtMR16(rm16, reg)
                        }
                        Data32 => {
                            // 0F A3 /r; BT r/m32, r32; Store selected bit in CF flag.
                            let reg = reg32_field_select(modrm.reg3, rex_r);
                            let rm32 = decode_rm32(lex, &modrm)?;
                            BtMR32(rm32, reg)
                        }
                        Data64 => {
                            // REX.W + 0F A3 /r; BT r/m64, r64; Store selected bit in CF flag.
                            let reg = reg64_field_select(modrm.reg3, rex_r);
                            let rm64 = decode_rm64(lex, &modrm)?;
                            BtMR64(rm64, reg)
                        }
                    }
                }
                0xAF => {
                    // 0F AF
                    let modrm = lex.next_modrm()?;
                    let rex_r = lex.get_rex_r_matters();
                    match lex.get_operand_size() {
                        Data16 => {
                            // 0F AF /r; IMUL r16, r/m16; word register := word register ∗ r/m16.
                            let reg = reg16_field_select(modrm.reg3, rex_r);
                            let rm16 = decode_rm16(lex, &modrm)?;
                            ImulRM16(reg, rm16)
                        }
                        Data32 => {
                            // 0F AF /r; IMUL r32, r/m32; doubleword register := doubleword register ∗ r/m32.
                            let reg = reg32_field_select(modrm.reg3, rex_r);
                            let rm32 = decode_rm32(lex, &modrm)?;
                            ImulRM32(reg, rm32)
                        }
                        Data64 => {
                            // REX.W + 0F AF /r; IMUL r64, r/m64; Quadword register := Quadword register ∗ r/m64.
                            let reg = reg64_field_select(modrm.reg3, rex_r);
                            let rm64 = decode_rm64(lex, &modrm)?;
                            ImulRM64(reg, rm64)
                        }
                    }
                }
                0xBA => {
                    let modrm = lex.next_modrm()?;
                    match modrm.reg3 {
                        4 => {
                            // 0F BA /4
                            match lex.get_operand_size() {
                                Data16 => {
                                    // 0F BA /4 ib; BT r/m16, imm8; Store selected bit in CF flag.
                                    let rm16 = decode_rm16(lex, &modrm)?;
                                    let imm8 = lex.next_imm8()?;
                                    BtMI16(rm16, imm8)
                                }
                                Data32 => {
                                    // 0F BA /4 ib; BT r/m32, imm8; Store selected bit in CF flag.
                                    let rm32 = decode_rm32(lex, &modrm)?;
                                    let imm8 = lex.next_imm8()?;
                                    BtMI32(rm32, imm8)
                                }
                                Data64 => {
                                    // REX.W + 0F BA /4 ib; BT r/m64, imm8; Store selected bit in CF flag.
                                    let rm64 = decode_rm64(lex, &modrm)?;
                                    let imm8 = lex.next_imm8()?;
                                    BtMI64(rm64, imm8)
                                }
                            }
                        }
                        _ => {
                            lex.rollback();
                            NotImplementedOpext(opcode, modrm.reg3)
                        }
                    }
                }
                _ => NotImplemented2(opcode, opcode2),
            }
        }
        0x04 | 0x24 | 0x2C | 0x34 | 0x3C => {
            // 8-bit immediate, with al as dest operand. Proto:
            // 04 ib; ADD AL, imm8; I; Valid; Valid; Add imm8 to AL.
            let imm8 = lex.next_imm8()?;
            let op = match opcode {
                0x04 => TwoOp::ADD,
                0x24 => TwoOp::AND,
                0x2c => TwoOp::SUB,
                0x34 => TwoOp::XOR,
                0x3C => TwoOp::CMP,
                _ => unreachable!(),
            };
            TwoMIInst8(op, RM8::Reg(GPR8::al), imm8)
        }
        0x05 | 0x25 | 0x2D | 0x35 | 0x3D => {
            // 16/32/64-bit immediate, with ax/eax/rax as dest operand.
            // Note the 64-bit immediate is sign-extended from 32-bit. Proto:
            // 05 iw; ADD AX, imm16; I; Valid; Valid; Add imm16 to AX.
            // 05 id; ADD EAX, imm32; I; Valid; Valid; Add imm32 to EAX.
            // REX.W + 05 id; ADD RAX, imm32; I; Valid; N.E.; Add imm32 sign-extended to 64-bits to RAX.
            let op = match opcode {
                0x05 => TwoOp::ADD,
                0x25 => TwoOp::AND,
                0x2D => TwoOp::SUB,
                0x35 => TwoOp::XOR,
                0x3D => TwoOp::CMP,
                _ => unreachable!(),
            };
            match lex.get_operand_size() {
                Data16 => {
                    let imm16 = lex.next_imm16()?;
                    TwoMIInst16(op, RM16::Reg(GPR16::ax), imm16)
                }
                Data32 => {
                    let imm32 = lex.next_imm32()?;
                    TwoMIInst32(op, RM32::Reg(GPR32::eax), imm32)
                }
                Data64 => {
                    let imm32 = lex.next_imm32()?;
                    TwoMIInst64(op, RM64::Reg(GPR64::rax), imm32 as i32 as u64)
                }
            }
        }
        0x50..=0x57 => {
            // PUSH a 16/64-bit register. TODO-golf: same operand as POP.
            let rex_b = lex.get_rex_b_matters();
            match lex.get_operand_size_64_default() {
                Data16 => {
                    // 50+rw; PUSH r16; Push r16.
                    let reg = reg16_field_select(opcode, rex_b);
                    PushM16(RM16::Reg(reg))
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    // 50+rd; PUSH r64; Push r64.
                    let reg = reg64_field_select(opcode, rex_b);
                    PushM64(RM64::Reg(reg))
                }
            }
        }
        0x58..=0x5F => {
            // POP to a 16/64-bit register. TODO-golf: same operand as PUSH.
            let rex_b = lex.get_rex_b_matters();
            match lex.get_operand_size_64_default() {
                Data16 => {
                    // 58+ rw; POP r16; Pop top of stack into r16; increment stack pointer.
                    let reg = reg16_field_select(opcode, rex_b);
                    PopM16(RM16::Reg(reg))
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    // 58+ rd; POP r64; Pop top of stack into r64; increment stack pointer. Cannot encode 32-bit operand size.
                    let reg = reg64_field_select(opcode, rex_b);
                    PopM64(RM64::Reg(reg))
                }
            }
        }
        0x90..=0x97 => {
            // Exchange a 16/32/64-bit operand with ax/eax/rax.
            let rex_b = lex.get_rex_b_matters();
            if opcode == 0x90 && lex.get_operand_size_no_rexw() != Data16 && !rex_b {
                // XCHG (E)AX, (E)AX (encoded instruction byte is 90H) is an alias
                // for NOP regardless of data size prefixes, including REX.W.
                return Ok(Nop);
            }
            let size = lex.get_operand_size();
            match size {
                Data16 => {
                    // 90+rw; XCHG AX, r16; Exchange r16 with AX.
                    // 90+rw; XCHG r16, AX; Exchange AX with r16.
                    let reg = reg16_field_select(opcode, rex_b);
                    XchgMR16(RM16::Reg(reg), GPR16::ax)
                }
                Data32 => {
                    // 90+rd; XCHG EAX, r32; Exchange r32 with EAX.
                    // 90+rd; XCHG r32, EAX; Exchange EAX with r32.
                    let reg = reg32_field_select(opcode, rex_b);
                    XchgMR32(RM32::Reg(reg), GPR32::eax)
                }
                Data64 => {
                    // REX.W + 90+rd; XCHG RAX, r64; Exchange r64 with RAX.
                    // REX.W + 90+rd; XCHG r64, RAX; Exchange RAX with r64.
                    let reg = reg64_field_select(opcode, rex_b);
                    XchgMR64(RM64::Reg(reg), GPR64::rax)
                }
            }
        }
        0x68 => {
            // PUSH as 16/32
            match lex.get_operand_size_64_default() {
                Data16 => {
                    // 68 iw; PUSH imm16; Push imm16.
                    let imm16 = lex.next_imm16()?;
                    PushI16(imm16)
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    // TODO-correctness: is this implemented right?
                    // 68 id; PUSH imm32; Push imm32.
                    let imm32 = lex.next_imm32()?;
                    PushI64(imm32 as i32 as u64)
                }
            }
        }
        0x69 => {
            // 16/32/64-bit immediate, with a register as dest operand.
            // Note the 64-bit immediate is sign-extended from 32-bit. Proto:
            // 69 /r iw; IMUL r16, r/m16, imm16; word register := r/m16 ∗ immediate word.
            // 69 /r id; IMUL r32, r/m32, imm32; doubleword register := r/m32 ∗ immediate doubleword.
            // REX.W + 69 /r id; IMUL r64, r/m64, imm32; Quadword register := r/m64 ∗ immediate doubleword.
            let modrm = lex.next_modrm()?;
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm)?;
                    let imm16 = lex.next_imm16()?;
                    ImulRMI16(reg, rm16, imm16)
                }
                Data32 => {
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm)?;
                    let imm32 = lex.next_imm32()?;
                    ImulRMI32(reg, rm32, imm32)
                }
                Data64 => {
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm)?;
                    let imm32 = lex.next_imm32()?;
                    // TODO-correctness: verify this is sign-extended from 32.
                    // Disagrees with the REX.W + 69 /r description.
                    ImulRMI64(reg, rm64, imm32 as i32 as u64)
                }
            }
        }
        0x6A => {
            // 16/64-bit immediate, sign-extended from 8-bit. Proto:
            // 6A ib; PUSH imm8; Push imm8.
            match lex.get_operand_size_64_default() {
                Data16 => {
                    let imm8 = lex.next_imm8()?;
                    PushI16(imm8 as i8 as u16)
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    let imm8 = lex.next_imm8()?;
                    PushI64(imm8 as i8 as u64)
                }
            }
        }
        0x6B => {
            // Register and immediate operands, sign-extended from 8-bit. Proto:
            // 6B /r ib; IMUL r16, r/m16, imm8; word register := r/m16 ∗ sign-extended immediate byte.
            // 6B /r ib; IMUL r32, r/m32, imm8; doubleword register := r/m32 ∗ sign-extended immediate byte.
            // REX.W + 6B /r ib; IMUL r64, r/m64, imm8; Quadword register := r/m64 ∗ sign-extended immediate byte.
            let modrm = lex.next_modrm()?;
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    ImulRMI16(reg, rm16, imm8 as i8 as u16)
                }
                Data32 => {
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    ImulRMI32(reg, rm32, imm8 as i8 as u32)
                }
                Data64 => {
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    ImulRMI64(reg, rm64, imm8 as i8 as u64)
                }
            }
        }
        0x70..=0x7F => {
            let rel_off = lex.next_i8()? as u64;
            let dest = lex.i.wrapping_add(rel_off);
            match opcode {
                0x70 => JccJo(dest, Normal),
                0x71 => JccJo(dest, Negate),
                0x72 => JccJb(dest, Normal),
                0x73 => JccJb(dest, Negate),
                0x74 => JccJe(dest, Normal),
                0x75 => JccJe(dest, Negate),
                0x76 => JccJbe(dest, Normal),
                0x77 => JccJbe(dest, Negate),
                0x78 => JccJs(dest, Normal),
                0x79 => JccJs(dest, Negate),
                0x7A => JccJp(dest, Normal),
                0x7B => JccJp(dest, Negate),
                0x7C => JccJl(dest, Normal),
                0x7D => JccJl(dest, Negate),
                0x7E => JccJle(dest, Normal),
                0x7F => JccJle(dest, Negate),
                _ => panic!("Missing match arm in decode_inst_inner."),
            }
        }
        0x80 => {
            // 8-bit immediate, with r/m8 as dest operand.
            // modrm.reg3 is used to determine the instruction. Proto:
            // 80 /0 ib; ADD r/m8, imm8; MI; Add imm8 to r/m8.
            let modrm = lex.next_modrm()?;
            let op = match modrm.reg3 {
                0 => TwoOp::ADD,
                4 => TwoOp::AND,
                5 => TwoOp::SUB,
                6 => TwoOp::XOR,
                7 => TwoOp::CMP,
                _ => {
                    lex.rollback();
                    return Ok(NotImplementedOpext(opcode, modrm.reg3));
                }
            };
            let rm8 = decode_rm8(lex, &modrm)?;
            let imm8 = lex.next_imm8()?;
            TwoMIInst8(op, rm8, imm8)
        }
        0x81 | 0xC7 => {
            // 16/32/64-bit immediate, with r/m as dest operand.
            // modrm.reg3 is used to determine the instruction.
            // Note the 64-bit immediate is sign-extended from 32-bit. Proto:
            // 81 /0 iw; ADD r/m16, imm16; MI; Add imm16 to r/m16.
            // 81 /0 id; ADD r/m32, imm32; MI; Add imm32 to r/m32.
            // REX.W + 81 /0 id; ADD r/m64, imm32; MI; Add imm32 sign-extended to 64-bits to r/m64.
            let modrm = lex.next_modrm()?;
            let op = match (opcode, modrm.reg3) {
                (0x81, 0) => TwoOp::ADD,
                (0x81, 4) => TwoOp::AND,
                (0x81, 5) => TwoOp::SUB,
                (0x81, 6) => TwoOp::XOR,
                (0x81, 7) => TwoOp::CMP,
                (0xC7, 0) => TwoOp::MOV,
                _ => {
                    lex.rollback();
                    return Ok(NotImplementedOpext(opcode, modrm.reg3));
                }
            };
            match lex.get_operand_size() {
                Data16 => {
                    let rm16 = decode_rm16(lex, &modrm)?;
                    let imm16 = lex.next_imm16()?;
                    TwoMIInst16(op, rm16, imm16)
                }
                Data32 => {
                    let rm32 = decode_rm32(lex, &modrm)?;
                    let imm32 = lex.next_imm32()?;
                    TwoMIInst32(op, rm32, imm32)
                }
                Data64 => {
                    let rm64 = decode_rm64(lex, &modrm)?;
                    let imm32 = lex.next_imm32()?;
                    // Sign extend imm32 to u64.
                    TwoMIInst64(op, rm64, imm32 as i32 as u64)
                }
            }
        }
        0x83 => {
            // 16/32/64-bit immediate, with r/m as dest operand.
            // modrm.reg3 is used to determine the instruction.
            // Note each immediate is sign-extended from 8-bit. Proto:
            // 83 /0 ib; ADD r/m16, imm8; MI; Add sign-extended imm8 to r/m16.
            // 83 /0 ib; ADD r/m32, imm8; MI; Add sign-extended imm8 to r/m32.
            // REX.W + 83 /0 ib; ADD r/m64, imm8; MI; Add sign-extended imm8 to r/m64.
            let modrm = lex.next_modrm()?;
            let op = match modrm.reg3 {
                0 => TwoOp::ADD,
                4 => TwoOp::AND,
                5 => TwoOp::SUB,
                6 => TwoOp::XOR,
                7 => TwoOp::CMP,
                _ => {
                    lex.rollback();
                    return Ok(NotImplementedOpext(opcode, modrm.reg3));
                }
            };
            match lex.get_operand_size() {
                Data16 => {
                    let rm16 = decode_rm16(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    TwoMIInst16(op, rm16, imm8 as i8 as u16)
                }
                Data32 => {
                    let rm32 = decode_rm32(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    TwoMIInst32(op, rm32, imm8 as i8 as u32)
                }
                Data64 => {
                    let rm64 = decode_rm64(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    TwoMIInst64(op, rm64, imm8 as i8 as u64)
                }
            }
        }
        0x86 => {
            // TODO-golf: group this with 0x00 | .. | 0x3A.
            // 86 /r; XCHG r/m8, r8; Exchange r8 (byte register) with byte from r/m8.
            // 86 /r; XCHG r8, r/m8; Exchange byte from r/m8 with r8 (byte register).
            let modrm = lex.next_modrm()?;
            let reg = lex.reg8_field_select_r(modrm.reg3);
            let rm8 = decode_rm8(lex, &modrm)?;
            XchgMR8(rm8, reg)
        }
        0x87 => {
            // TODO-golf: group this with 0x01 | .. | 0x8B.
            let modrm = lex.next_modrm()?;
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // 87 /r; XCHG r/m16, r16; Exchange r16 with word from r/m16.
                    // 87 /r; XCHG r16, r/m16; Exchange word from r/m16 with r16.
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm)?;
                    XchgMR16(rm16, reg)
                }
                Data32 => {
                    // 87 /r; XCHG r/m32, r32; Exchange r32 with doubleword from r/m32.
                    // 87 /r; XCHG r32, r/m32; Exchange doubleword from r/m32 with r32.
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm)?;
                    XchgMR32(rm32, reg)
                }
                Data64 => {
                    // REX.W + 87 /r; XCHG r/m64, r64; Exchange r64 with quadword from r/m64.
                    // REX.W + 87 /r; XCHG r64, r/m64; Exchange quadword from r/m64 with r64.
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm)?;
                    XchgMR64(rm64, reg)
                }
            }
        }
        0x8D => {
            // LEA. It's special. Proto:
            // 8D /r; LEA r16,m; RM; Store effective address for m in register r16.
            // 8D /r; LEA r32,m; RM; Store effective address for m in register r32.
            // REX.W + 8D /r; LEA r64,m; RM; Store effective address for m in register r64.
            let modrm = lex.next_modrm()?;
            if modrm.mod2 == 0b11 {
                lex.rollback();
                return Ok(LeaRegInsteadOfAddr);
            };
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    let gpr16 = reg16_field_select(modrm.reg3, rex_r);
                    let eff_addr = decode_rm_00_01_10(lex, &modrm)?;
                    LeaRM16(gpr16, eff_addr)
                }
                Data32 => {
                    let gpr32 = reg32_field_select(modrm.reg3, rex_r);
                    let eff_addr = decode_rm_00_01_10(lex, &modrm)?;
                    LeaRM32(gpr32, eff_addr)
                }
                Data64 => {
                    let gpr64 = reg64_field_select(modrm.reg3, rex_r);
                    let eff_addr = decode_rm_00_01_10(lex, &modrm)?;
                    LeaRM64(gpr64, eff_addr)
                }
            }
        }
        0x8F => {
            // One r/m arg, modrm.reg3 selects the op. Proto:
            // 8F /0; POP r/m16; Pop top of stack into m16; increment stack pointer.
            // 8F /0; POP r/m64; Pop top of stack into m64; increment stack pointer. Cannot encode 32-bit operand size.
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0 => {
                    // 8F /0
                    match lex.get_operand_size_64_default() {
                        Data16 => {
                            let rm16 = decode_rm16(lex, &modrm)?;
                            PopM16(rm16)
                        }
                        Data32 => panic!("Impossible Data32 with 64-bit default."),
                        Data64 => {
                            let rm64 = decode_rm64(lex, &modrm)?;
                            PopM64(rm64)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0x9C => {
            // No args, but depends on operand size.
            match lex.get_operand_size_64_default() {
                Data16 => {
                    // 9C; PUSHF; Push lower 16 bits of EFLAGS.
                    Pushf16
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    // 9C; PUSHFQ; Push RFLAGS.
                    Pushf64
                }
            }
        }
        0x9D => {
            // No args, but depends on operand size.
            match lex.get_operand_size_64_default() {
                Data16 => {
                    // 9D; POPF; Pop top of stack into lower 16 bits of EFLAGS.
                    Popf16
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    // 9D; POPFQ; Pop top of stack and zero-extend into RFLAGS.
                    Popf64
                }
            }
        }
        0xAE => Scas(DataSize::Data8, lex.get_address_size()),
        0xAF => {
            let addr_size = lex.get_address_size();
            match lex.get_operand_size() {
                Data16 => Scas(DataSize::Data16, addr_size),
                Data32 => Scas(DataSize::Data32, addr_size),
                Data64 => Scas(DataSize::Data64, addr_size),
            }
        }
        0xB0..=0xB7 => {
            // B0+ rb ib; MOV r8, imm8
            // Move imm8 to r8.
            let imm8 = lex.next_imm8()?;
            let reg = lex.reg8_field_select_b(opcode);
            MovOI8(reg, imm8)
        }
        0xB8..=0xBF => {
            let rex_b = lex.get_rex_b_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // B8+ rw iw; MOV r16, imm16
                    let imm16 = lex.next_imm16()?;
                    let reg = reg16_field_select(opcode, rex_b);
                    MovOI16(reg, imm16)
                }
                Data32 => {
                    // B8+ rd id; MOV r32, imm32
                    let imm32 = lex.next_imm32()?;
                    let reg = reg32_field_select(opcode, rex_b);
                    MovOI32(reg, imm32)
                }
                Data64 => {
                    // REX.W + B8+ rd io
                    let imm64 = lex.next_imm64()?;
                    let reg = reg64_field_select(opcode, rex_b);
                    MovOI64(reg, imm64)
                }
            }
        }
        0xC0 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0..=3 => {
                    let rm8 = decode_rm8(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    let rp = decode_rot_pair(modrm.reg3);
                    // C0 /0 ib; ROL r/m8, imm8; Rotate 8 bits r/m8 left imm8 times.
                    // C0 /1 ib; ROR r/m8, imm8; Rotate 8 bits r/m16 right imm8 times.
                    // C0 /2 ib; RCL r/m8, imm8; Rotate 9 bits (CF, r/m8) left imm8 times.
                    // C0 /3 ib; RCR r/m8, imm8; Rotate 9 bits (CF, r/m8) right imm8 times.
                    RotateMI8(rp, rm8, imm8)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xC1 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0..=3 => {
                    let rp = decode_rot_pair(modrm.reg3);
                    match lex.get_operand_size() {
                        Data16 => {
                            let rm16 = decode_rm16(lex, &modrm)?;
                            let imm8 = lex.next_imm8()?;
                            // C1 /0 ib; ROL r/m16, imm8; Rotate 16 bits r/m16 left imm8 times.
                            // C1 /1 ib; ROR r/m16, imm8; Rotate 16 bits r/m16 right imm8 times.
                            // C1 /2 ib; RCL r/m16, imm8; Rotate 17 bits (CF, r/m16) left imm8 times.
                            // C1 /3 ib; RCR r/m16, imm8; Rotate 17 bits (CF, r/m16) right imm8 times.
                            RotateMI16(rp, rm16, imm8)
                        }
                        Data32 => {
                            let rm32 = decode_rm32(lex, &modrm)?;
                            let imm8 = lex.next_imm8()?;
                            // C1 /0 ib; ROL r/m32, imm8; Rotate 32 bits r/m32 left imm8 times.
                            // C1 /1 ib; ROR r/m32, imm8; Rotate 32 bits r/m32 right imm8 times.
                            // C1 /2 ib; RCL r/m32, imm8; Rotate 33 bits (CF, r/m32) left imm8 times.
                            // C1 /3 ib; RCR r/m32, imm8; Rotate 33 bits (CF, r/m32) right imm8 times.
                            RotateMI32(rp, rm32, imm8)
                        }
                        Data64 => {
                            let rm64 = decode_rm64(lex, &modrm)?;
                            let imm8 = lex.next_imm8()?;
                            // REX.W + C1 /0 ib; ROL r/m64, imm8; Rotate 64 bits r/m64 left imm8 times. Uses a 6 bit count.
                            // REX.W + C1 /1 ib; ROR r/m64, imm8; Rotate 64 bits r/m64 right imm8 times. Uses a 6 bit count.
                            // REX.W + C1 /2 ib; RCL r/m64, imm8; Rotate 65 bits (CF, r/m64) left imm8 times. Uses a 6 bit count.
                            // REX.W + C1 /3 ib; RCR r/m64, imm8; Rotate 65 bits (CF, r/m64) right imm8 times. Uses a 6 bit count.
                            RotateMI64(rp, rm64, imm8)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xC6 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0 => {
                    // C6 /0 ib; MOV r/m8, imm16
                    let rm8 = decode_rm8(lex, &modrm)?;
                    let imm8 = lex.next_imm8()?;
                    TwoMIInst8(TwoOp::MOV, rm8, imm8)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xD0 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0..=3 => {
                    let rm8 = decode_rm8(lex, &modrm)?;
                    let rp = decode_rot_pair(modrm.reg3);
                    // D0 /0; ROL r/m8, 1; Rotate 8 bits r/m8 left once.
                    // D0 /1; ROR r/m8, 1; Rotate 8 bits r/m8 right once.
                    // D0 /2; RCL r/m8, 1; Rotate 9 bits (CF, r/m8) left once.
                    // D0 /3; RCR r/m8, 1; Rotate 9 bits (CF, r/m8) right once.
                    RotateMI8(rp, rm8, 1)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xD1 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0..=3 => {
                    let rp = decode_rot_pair(modrm.reg3);
                    match lex.get_operand_size() {
                        Data16 => {
                            let rm16 = decode_rm16(lex, &modrm)?;
                            // D1 /0; ROL r/m16, 1; Rotate 16 bits r/m16 left once.
                            // D1 /1; ROR r/m16, 1; Rotate 16 bits r/m16 right once.
                            // D1 /2; RCL r/m16, 1; Rotate 17 bits (CF, r/m16) left once.
                            // D1 /3; RCR r/m16, 1; Rotate 17 bits (CF, r/m16) right once.
                            RotateMI16(rp, rm16, 1)
                        }
                        Data32 => {
                            let rm32 = decode_rm32(lex, &modrm)?;
                            // D1 /0; ROL r/m32, 1; Rotate 32 bits r/m32 left once.
                            // D1 /1; ROR r/m32, 1; Rotate 32 bits r/m32 right once.
                            // D1 /2; RCL r/m32, 1; Rotate 33 bits (CF, r/m32) left once.
                            // D1 /3; RCR r/m32, 1; Rotate 33 bits (CF, r/m32) right once. Uses a 6 bit count.
                            RotateMI32(rp, rm32, 1)
                        }
                        Data64 => {
                            let rm64 = decode_rm64(lex, &modrm)?;
                            // REX.W + D1 /0; ROL r/m64, 1; Rotate 64 bits r/m64 left once. Uses a 6 bit count.
                            // REX.W + D1 /1; ROR r/m64, 1; Rotate 64 bits r/m64 right once. Uses a 6 bit count.
                            // REX.W + D1 /2; RCL r/m64, 1; Rotate 65 bits (CF, r/m64) left once. Uses a 6 bit count.
                            // REX.W + D1 /3; RCR r/m64, 1; Rotate 65 bits (CF, r/m64) right once. Uses a 6 bit count.
                            RotateMI64(rp, rm64, 1)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xD2 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0..=3 => {
                    let rp = decode_rot_pair(modrm.reg3);
                    let rm8 = decode_rm8(lex, &modrm)?;
                    // D2 /0; ROL r/m8, CL; Rotate 8 bits r/m8 left CL times.
                    // D2 /1; ROR r/m8, CL; Rotate 8 bits r/m8 right CL times.
                    // D2 /2; RCL r/m8, CL; Rotate 9 bits (CF, r/m8) left CL times.
                    // D2 /3; RCR r/m8, CL; Rotate 9 bits (CF, r/m8) right CL times.
                    RotateMC8(rp, rm8)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xD3 => {
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0..=3 => {
                    let rp = decode_rot_pair(modrm.reg3);
                    match lex.get_operand_size() {
                        Data16 => {
                            let rm16 = decode_rm16(lex, &modrm)?;
                            // D3 /0; ROL r/m16, CL; Rotate 16 bits r/m16 left CL times.
                            // D3 /1; ROR r/m16, CL; Rotate 16 bits r/m16 right CL times.
                            // D3 /2; RCL r/m16, CL; Rotate 17 bits (CF, r/m16) left CL times.
                            // D3 /3; RCR r/m16, CL; Rotate 17 bits (CF, r/m16) right CL times.
                            RotateMC16(rp, rm16)
                        }
                        Data32 => {
                            let rm32 = decode_rm32(lex, &modrm)?;
                            // D3 /0; ROL r/m32, CL; Rotate 32 bits r/m32 left CL times.
                            // D3 /1; ROR r/m32, CL; Rotate 32 bits r/m32 right CL times.
                            // D3 /2; RCL r/m32, CL; Rotate 33 bits (CF, r/m32) left CL times.
                            // D3 /3; RCR r/m32, CL; Rotate 33 bits (CF, r/m32) right CL times.
                            RotateMC32(rp, rm32)
                        }
                        Data64 => {
                            let rm64 = decode_rm64(lex, &modrm)?;
                            // REX.W + D3 /0; ROL r/m64, CL; Rotate 64 bits r/m64 left CL times. Uses a 6 bit count.
                            // REX.W + D3 /1; ROR r/m64, CL; Rotate 64 bits r/m64 right CL times. Uses a 6 bit count.
                            // REX.W + D3 /2; RCL r/m64, CL; Rotate 65 bits (CF, r/m64) left CL times. Uses a 6 bit count.
                            // REX.W + D3 /3; RCR r/m64, CL; Rotate 65 bits (CF, r/m64) right CL times. Uses a 6 bit count.
                            RotateMC64(rp, rm64)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xE3 => {
            let rel_off = lex.next_i8()? as u64;
            let dest = lex.i.wrapping_add(rel_off);
            match lex.get_address_size() {
                Addr32 => Jecxz(dest),
                Addr64 => Jrcxz(dest),
            }
        }
        0xE9 => {
            // E9 cd; JMP rel32; Jump near, relative, RIP = RIP + 32-bit displacement sign extended to 64-bits.
            // E9 cw; JMP rel16; Jump near, relative, displacement relative to next instruction. Not supported in 64-bit mode.
            //   Note we mention the E9 cw encoding despite "Not supported in 64-bit mode." because it still seems to be parsed.
            //   http://ref.x86asm.net/coder64.html#gen_note_short_near_jmp notes it is implementation-dependent.
            //   It's obtainable with the operand-size prefix 0x66.
            // TODO-correctnesss:
            // gdb dis (new cases for 015_jmp.s):
            //   e9 12 34 56 78              jmp    0x78963420
            //   66 e9 12 34                 jmpw   0x342a
            //   48 e9 12 34 56 78           rex.W jmp 0x7896346b
            //   66 48 e9 12 34 56 78        data16 rex.W jmp 0x78963479
            match lex.get_operand_size_no_rexw() {
                Data16 => todo!("Operand-size prefix 0x66 behaves funny with jmp 0xE9. I haven't reverse-engineered it yet."),
                Data32 => {
                    let rel_off = lex.next_i32()? as u64;
                    let dest = lex.i.wrapping_add(rel_off);
                    JmpD(dest)
                }
                Data64 => panic!("Impossible Data64 without REX.W bit."),
            }
        }
        0xEB => {
            // EB cb; JMP rel8; Jump short, RIP = RIP + 8-bit displacement sign extended to 64-bits.
            let rel_off = lex.next_i8()? as u64;
            let dest = lex.i.wrapping_add(rel_off);
            JmpD(dest)
        }
        0xF4 => {
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return Ok(RexNoop);
            }
            Hlt
        }
        0xF6 | 0xFE => {
            // One 8-bit r/m8 is the only operand encoded. Proto:
            // F6 /2; NOT r/m8; Reverse each bit of r/m8.
            // F6 /5; IMUL r/m8; AX:= AL ∗ r/m byte.
            // F6 /6; DIV r/m8; Unsigned divide AX by r/m8, with result stored in AL := Quotient, AH := Remainder.
            let modrm = lex.next_modrm()?;
            let op = match (opcode, modrm.reg3) {
                (0xF6, 2) => OneOp::NOT,
                (0xF6, 5) => OneOp::IMUL,
                (0xF6, 6) => OneOp::DIV,
                (0xFE, 0) => OneOp::INC,
                (0xFE, 1) => OneOp::DEC,
                _ => {
                    lex.rollback();
                    return Ok(NotImplementedOpext(opcode, modrm.reg3));
                }
            };
            let rm8 = decode_rm8(lex, &modrm)?;
            OneRMInst8(op, rm8)
        }
        0xF7 => {
            // One 16/32/64-bit r/m is the only operand encoded. Proto:
            // F7 /2; NOT r/m16; M; Reverse each bit of r/m16.
            // F7 /2; NOT r/m32; M; Reverse each bit of r/m32.
            // REX.W + F7 /2; NOT r/m64; M; Reverse each bit of r/m64.
            let modrm = lex.next_modrm()?;
            let op = match modrm.reg3 {
                2 => OneOp::NOT,
                5 => OneOp::IMUL,
                6 => OneOp::DIV,
                _ => {
                    lex.rollback();
                    return Ok(NotImplementedOpext(opcode, modrm.reg3));
                }
            };
            match lex.get_operand_size() {
                Data16 => {
                    let rm16 = decode_rm16(lex, &modrm)?;
                    OneRMInst16(op, rm16)
                }
                Data32 => {
                    let rm32 = decode_rm32(lex, &modrm)?;
                    OneRMInst32(op, rm32)
                }
                Data64 => {
                    let rm64 = decode_rm64(lex, &modrm)?;
                    OneRMInst64(op, rm64)
                }
            }
        }
        0xFF => {
            // TODO-golf: untangle this monster case.
            let modrm = lex.next_modrm()?;
            match modrm.reg3 {
                0 | 1 => {
                    let op = match modrm.reg3 {
                        0 => OneOp::INC,
                        1 => OneOp::DEC,
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    };
                    // FF /0, FF /1
                    match lex.get_operand_size() {
                        Data16 => {
                            // FF /0; INC r/m16; Increment r/m word by 1.
                            // FF /1; DEC r/m16; Decrement r/m word by 1.
                            let rm16 = decode_rm16(lex, &modrm)?;
                            OneRMInst16(op, rm16)
                        }
                        Data32 => {
                            // FF /0; INC r/m32; Increment r/m doubleword by 1.
                            // FF /1; DEC r/m32; Decrement r/m doubleword by 1.
                            let rm32 = decode_rm32(lex, &modrm)?;
                            OneRMInst32(op, rm32)
                        }
                        Data64 => {
                            // REX.W + FF /0; INC r/m64; Increment r/m quadword by 1.
                            // REX.W + FF /1; DEC r/m64; Decrement r/m quadword by 1.
                            let rm64 = decode_rm64(lex, &modrm)?;
                            OneRMInst64(op, rm64)
                        }
                    }
                }
                4 => {
                    // FF /4; JMP r/m64; Jump near, absolute indirect, RIP = 64-Bit offset from register or memory.
                    let rm64 = decode_rm64(lex, &modrm)?;
                    JmpM64(rm64)
                }
                6 => {
                    // FF /6
                    match lex.get_operand_size_64_default() {
                        Data16 => {
                            // FF /6; PUSH r/m16; Push r/m16.
                            let rm16 = decode_rm16(lex, &modrm)?;
                            PushM16(rm16)
                        }
                        Data32 => panic!("Impossible Data32 with 64-bit default."),
                        Data64 => {
                            // FF /6; PUSH r/m64; Push r/m64.
                            let rm64 = decode_rm64(lex, &modrm)?;
                            PushM64(rm64)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        opcode => NotImplemented(opcode),
    })
}

#[derive(Debug)]
struct ModRM {
    pub mod2: u8,
    pub reg3: u8,
    pub rm3: u8,
}

fn modrm_decode(modrm: u8) -> ModRM {
    let mod2 = modrm >> 6;
    let reg3 = modrm >> 3 & 0b111;
    let rm3 = modrm & 0b111;
    ModRM { mod2, reg3, rm3 }
}

fn decode_rot_pair(modrm_reg3: u8) -> (RotType, RotDir) {
    match modrm_reg3 {
        0 => rot_pair::ROL,
        1 => rot_pair::ROR,
        2 => rot_pair::RCL,
        3 => rot_pair::RCR,
        _ => panic!("Missing match arm in decode_rot_pair."),
    }
}

// r/m8
fn decode_rm8(lex: &mut Lexer, modrm: &ModRM) -> RResult<RM8> {
    Ok(match modrm.mod2 {
        0b00..=0b10 => RM8::Addr(decode_rm_00_01_10(lex, modrm)?),
        0b11 => {
            let reg = lex.reg8_field_select_b(modrm.rm3);
            RM8::Reg(reg)
        }
        _ => panic!("Missing match arm in modrm_decode_addr8."),
    })
}

// r/m16
fn decode_rm16(lex: &mut Lexer, modrm: &ModRM) -> RResult<RM16> {
    Ok(match modrm.mod2 {
        0b00..=0b10 => RM16::Addr(decode_rm_00_01_10(lex, modrm)?),
        0b11 => {
            let rexb = lex.get_rex_b_matters();
            RM16::Reg(reg16_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr16."),
    })
}

// r/m32
fn decode_rm32(lex: &mut Lexer, modrm: &ModRM) -> RResult<RM32> {
    Ok(match modrm.mod2 {
        0b00..=0b10 => RM32::Addr(decode_rm_00_01_10(lex, modrm)?),
        0b11 => {
            let rexb = lex.get_rex_b_matters();
            RM32::Reg(reg32_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr32."),
    })
}

// r/m64
fn decode_rm64(lex: &mut Lexer, modrm: &ModRM) -> RResult<RM64> {
    Ok(match modrm.mod2 {
        0b00..=0b10 => RM64::Addr(decode_rm_00_01_10(lex, modrm)?),
        0b11 => {
            let rex_b = lex.get_rex_b_matters();
            RM64::Reg(reg64_field_select(modrm.rm3, rex_b))
        }
        _ => panic!("Missing match arm in modrm_decode_addr64."),
    })
}

/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
/// The table only provides (address_size, rexb) = (Addr32, false). Rest are guesses.
/// 2.2.1.3 Displacement specifies address_size == Addr64 uses the same encodings,
/// but with addresses 64-bit regs instead of 32-bit. However displacement remains the same size (8/32 bits).
fn decode_rm_00_01_10(lex: &mut Lexer, modrm: &ModRM) -> RResult<EffAddr> {
    let rex_b = lex.get_rex_b_matters();
    let address_size = lex.get_address_size();
    if modrm.mod2 == 0b00 && modrm.rm3 == 0b101 {
        // Special case: Instead of encoding a plain [ebp] (rexb=0) or [r13] (rexb=1),
        // it encodes as disp32 instead. Since we are in 64-bit mode,
        // it's actually a RIP-relative disp32.
        // See Vol 2A: 2.2.1.6 "RIP-Relative Addressing", and Table 2-7.
        return Ok(match address_size {
            Addr32 => EffAddr::EffAddr32(SIDB32 {
                disp: Some(lex.next_i32()?),
                base: Some(Base32::Eip),
                index: None,
                scale: Scale1,
            }),
            Addr64 => EffAddr::EffAddr64(SIDB64 {
                disp: Some(lex.next_i32()?),
                base: Some(Base64::Rip),
                index: None,
                scale: Scale1,
            }),
        });
    }
    // Here, sidb always has no displacement, so the "d" is a bit misleading.
    let sidb = match (address_size, rex_b, modrm.rm3) {
        (Addr32, false, 0b000) => EffAddr::from_base_reg32(GPR32::eax),
        (Addr64, false, 0b000) => EffAddr::from_base_reg64(GPR64::rax),
        (Addr32, false, 0b001) => EffAddr::from_base_reg32(GPR32::ecx),
        (Addr64, false, 0b001) => EffAddr::from_base_reg64(GPR64::rcx),
        (Addr32, false, 0b010) => EffAddr::from_base_reg32(GPR32::edx),
        (Addr64, false, 0b010) => EffAddr::from_base_reg64(GPR64::rdx),
        (Addr32, false, 0b011) => EffAddr::from_base_reg32(GPR32::ebx),
        (Addr64, false, 0b011) => EffAddr::from_base_reg64(GPR64::rbx),
        (_, _, 0b100) => {
            let sib = sib_decode(lex.next_u8()?);
            match lex.prefix.address_size {
                Addr32 => decode_sib_addr32(lex, &sib, modrm)?,
                Addr64 => decode_sib_addr64(lex, &sib, modrm)?,
            }
        }
        (Addr32, false, 0b101) => EffAddr::from_base_reg32(GPR32::ebp),
        (Addr64, false, 0b101) => EffAddr::from_base_reg64(GPR64::rbp),
        (Addr32, false, 0b110) => EffAddr::from_base_reg32(GPR32::esi),
        (Addr64, false, 0b110) => EffAddr::from_base_reg64(GPR64::rsi),
        (Addr32, false, 0b111) => EffAddr::from_base_reg32(GPR32::edi),
        (Addr64, false, 0b111) => EffAddr::from_base_reg64(GPR64::rdi),
        (Addr32, true, 0b000) => EffAddr::from_base_reg32(GPR32::r8d),
        (Addr64, true, 0b000) => EffAddr::from_base_reg64(GPR64::r8),
        (Addr32, true, 0b001) => EffAddr::from_base_reg32(GPR32::r9d),
        (Addr64, true, 0b001) => EffAddr::from_base_reg64(GPR64::r9),
        (Addr32, true, 0b010) => EffAddr::from_base_reg32(GPR32::r10d),
        (Addr64, true, 0b010) => EffAddr::from_base_reg64(GPR64::r10),
        (Addr32, true, 0b011) => EffAddr::from_base_reg32(GPR32::r11d),
        (Addr64, true, 0b011) => EffAddr::from_base_reg64(GPR64::r11),
        // No r12 because that's sidb.
        (Addr32, true, 0b101) => EffAddr::from_base_reg32(GPR32::r13d),
        (Addr64, true, 0b101) => EffAddr::from_base_reg64(GPR64::r13),
        (Addr32, true, 0b110) => EffAddr::from_base_reg32(GPR32::r14d),
        (Addr64, true, 0b110) => EffAddr::from_base_reg64(GPR64::r14),
        (Addr32, true, 0b111) => EffAddr::from_base_reg32(GPR32::r15d),
        (Addr64, true, 0b111) => EffAddr::from_base_reg64(GPR64::r15),
        _ => panic!("Missing match arm."),
    };
    Ok(match modrm.mod2 {
        0b00 => sidb,
        0b01 => {
            // The `as i32` sign-extends
            sidb.with_disp(Some(lex.next_i8()? as i32))
        }
        0b10 => sidb.with_disp(Some(lex.next_i32()?)),
        _ => panic!("Missing match arm."),
    })
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
struct SIB {
    pub scale: Scale,
    pub index3: u8,
    pub base3: u8,
}

fn sib_decode(sib: u8) -> SIB {
    let ss2 = sib >> 6;
    let scale = match ss2 {
        0b00 => Scale1,
        0b01 => Scale2,
        0b10 => Scale4,
        0b11 => Scale8,
        _ => panic!("Missing match arm."),
    };
    let index3 = sib >> 3 & 0b111;
    let base3 = sib & 0b111;
    SIB {
        scale,
        index3,
        base3,
    }
}

/// Vol 2A: Table 2-3. 32-Bit Addressing Forms with the SIB Byte
fn decode_sib_addr32(lex: &mut Lexer, sib: &SIB, modrm: &ModRM) -> RResult<EffAddr> {
    let index_reg = reg32_field_select(sib.index3, lex.get_rex_x_matters());
    let index_reg = match index_reg {
        GPR32::esp => {
            // 'none' row (REX.X=0, sib.index3=0b100)
            Index32::Eiz
        }
        _ => Index32::GPR32(index_reg),
    };
    let rex_b = lex.get_rex_b_matters();
    let base_reg = reg32_field_select(sib.base3, rex_b);
    if let (0b00, GPR32::ebp) = (modrm.mod2, base_reg) {
        // Very special case: [*] in the table (REX.b=0, sib.base3=0b101)
        // (Vol 2A: Table 2-3 "32-Bit Addressing Forms with the SIB Byte")
        // [*] when the MOD is 0b00 means disp32 with no base.
        return Ok(EffAddr::EffAddr32(SIDB32 {
            base: None,
            disp: Some(lex.next_i32()?),
            index: Some(index_reg),
            scale: sib.scale,
        }));
    }
    Ok(EffAddr::EffAddr32(SIDB32 {
        base: Some(Base32::GPR32(base_reg)),
        // The callee fills in disp
        disp: None,
        index: Some(index_reg),
        scale: sib.scale,
    }))
}

/// 64-bit address mode variation on the 32-bit analog:
/// Vol 2A: Table 2-3. 32-Bit Addressing Forms with the SIB Byte
fn decode_sib_addr64(lex: &mut Lexer, sib: &SIB, modrm: &ModRM) -> RResult<EffAddr> {
    let index_reg = reg64_field_select(sib.index3, lex.get_rex_x_matters());
    let index_reg = match index_reg {
        GPR64::rsp => {
            // 'none' row (REX.X=0, sib.index3=0b100)
            Index64::Riz
        }
        _ => Index64::GPR64(index_reg),
    };
    let rex_b = lex.get_rex_b_matters();
    let base_reg = reg64_field_select(sib.base3, rex_b);
    if let (0b00, GPR64::rbp) = (modrm.mod2, base_reg) {
        // Very special case: [*] in the table (REX.b=0, sib.base3=0b101)
        // (Vol 2A: Table 2-3 "32-Bit Addressing Forms with the SIB Byte")
        // [*] when the MOD is 0b00 means disp32 with no base.
        return Ok(EffAddr::EffAddr64(SIDB64 {
            base: None,
            // The disp is still 32-bit despite being 64-bit addressing.
            disp: Some(lex.next_i32()?),
            index: Some(index_reg),
            scale: sib.scale,
        }));
    }
    Ok(EffAddr::EffAddr64(SIDB64 {
        base: Some(Base64::GPR64(base_reg)),
        // The callee fills in disp
        disp: None,
        index: Some(index_reg),
        scale: sib.scale,
    }))
}

/// Vol 2A: Table 3-1 "Register Codes Associated With +rb, +rw, +rd, +ro.""
/// This is for +rb (byte register).
/// The input boolean is the relevant bit of the Rex:
///   - x: index field of the SIB byte
///   - r: reg field of the ModR/M byte
///   - b: base field of SIB byte, or
///        R/M field of ModR/M byte, or
///        reg field of the opcode
/// Return a pair of the register and a boolean, where
/// the boolean is true iff the REX byte is the "default"
/// This also matches the r8(/r) row of another table, at least when rex = None:
/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
fn reg8_field_select(op: u8, rex: Option<bool>) -> (GPR8, bool) {
    match (rex, op & 0b111) {
        // No REX prefix, or REX prefix with B bit cleared.
        (None, 0b000) => (GPR8::al, false),
        (Some(false), 0b000) => (GPR8::al, false),
        (None, 0b001) => (GPR8::cl, false),
        (Some(false), 0b001) => (GPR8::cl, false),
        (None, 0b010) => (GPR8::dl, false),
        (Some(false), 0b010) => (GPR8::dl, false),
        (None, 0b011) => (GPR8::bl, false),
        (Some(false), 0b011) => (GPR8::bl, false),
        // No REX prefix.
        (None, 0b100) => (GPR8::ah, false),
        (None, 0b101) => (GPR8::ch, false),
        (None, 0b110) => (GPR8::dh, false),
        (None, 0b111) => (GPR8::bh, false),
        // REX prefix with B bit cleared (e.g. 0x40).
        (Some(false), 0b100) => (GPR8::spl, true),
        (Some(false), 0b101) => (GPR8::bpl, true),
        (Some(false), 0b110) => (GPR8::sil, true),
        (Some(false), 0b111) => (GPR8::dil, true),
        // REX prefix with B bit set (e.g. 0x41).
        (Some(true), 0b000) => (GPR8::r8b, true),
        (Some(true), 0b001) => (GPR8::r9b, true),
        (Some(true), 0b010) => (GPR8::r10b, true),
        (Some(true), 0b011) => (GPR8::r11b, true),
        (Some(true), 0b100) => (GPR8::r12b, true),
        (Some(true), 0b101) => (GPR8::r13b, true),
        (Some(true), 0b110) => (GPR8::r14b, true),
        (Some(true), 0b111) => (GPR8::r15b, true),
        _ => panic!("Missing match arm in reg8_field_select."),
    }
}

/// Vol 2A: Table 3-1 "Register Codes Associated With +rb, +rw, +rd, +ro.""
/// This is for +rw (word register).
// No REX prefix is the same as a REX prefix with B bit clear.
fn reg16_field_select(op: u8, rexb: bool) -> GPR16 {
    match (rexb, op & 0b111) {
        // No REX prefix, or REX prefix with B bit cleared (e.g. 0x40).
        (false, 0b000) => GPR16::ax,
        (false, 0b001) => GPR16::cx,
        (false, 0b010) => GPR16::dx,
        (false, 0b011) => GPR16::bx,
        (false, 0b100) => GPR16::sp,
        (false, 0b101) => GPR16::bp,
        (false, 0b110) => GPR16::si,
        (false, 0b111) => GPR16::di,
        // REX prefix with B bit set (e.g. 0x41).
        (true, 0b000) => GPR16::r8w,
        (true, 0b001) => GPR16::r9w,
        (true, 0b010) => GPR16::r10w,
        (true, 0b011) => GPR16::r11w,
        (true, 0b100) => GPR16::r12w,
        (true, 0b101) => GPR16::r13w,
        (true, 0b110) => GPR16::r14w,
        (true, 0b111) => GPR16::r15w,
        _ => panic!("Missing match arm in reg32_field_select."),
    }
}

/// Vol 2A: Table 3-1 "Register Codes Associated With +rb, +rw, +rd, +ro."
/// This is for +rd (dword register).
// No REX prefix is the same as a REX prefix with B bit clear.
fn reg32_field_select(op: u8, rexb: bool) -> GPR32 {
    match (rexb, op & 0b111) {
        // No REX prefix, or REX prefix with B bit cleared (e.g. 0x40).
        (false, 0b000) => GPR32::eax,
        (false, 0b001) => GPR32::ecx,
        (false, 0b010) => GPR32::edx,
        (false, 0b011) => GPR32::ebx,
        (false, 0b100) => GPR32::esp,
        (false, 0b101) => GPR32::ebp,
        (false, 0b110) => GPR32::esi,
        (false, 0b111) => GPR32::edi,
        // REX prefix with B bit set (e.g. 0x41).
        (true, 0b000) => GPR32::r8d,
        (true, 0b001) => GPR32::r9d,
        (true, 0b010) => GPR32::r10d,
        (true, 0b011) => GPR32::r11d,
        (true, 0b100) => GPR32::r12d,
        (true, 0b101) => GPR32::r13d,
        (true, 0b110) => GPR32::r14d,
        (true, 0b111) => GPR32::r15d,
        _ => panic!("Missing match arm in reg32_field_select."),
    }
}

/// Vol 2A: Table 3-1 "Register Codes Associated With +rb, +rw, +rd, +ro."
/// This is for +ro (quadword register).
/// No REX prefix is the same as a REX prefix with B bit clear.
fn reg64_field_select(op: u8, rexb: bool) -> GPR64 {
    match (rexb, op & 0b111) {
        // No REX prefix, or REX prefix with B bit cleared (e.g. 0x40).
        (false, 0b000) => GPR64::rax,
        (false, 0b001) => GPR64::rcx,
        (false, 0b010) => GPR64::rdx,
        (false, 0b011) => GPR64::rbx,
        (false, 0b100) => GPR64::rsp,
        (false, 0b101) => GPR64::rbp,
        (false, 0b110) => GPR64::rsi,
        (false, 0b111) => GPR64::rdi,
        // REX prefix with B bit set (e.g. 0x41).
        (true, 0b000) => GPR64::r8,
        (true, 0b001) => GPR64::r9,
        (true, 0b010) => GPR64::r10,
        (true, 0b011) => GPR64::r11,
        (true, 0b100) => GPR64::r12,
        (true, 0b101) => GPR64::r13,
        (true, 0b110) => GPR64::r14,
        (true, 0b111) => GPR64::r15,
        _ => panic!("Missing match arm in reg64_field_select."),
    }
}
