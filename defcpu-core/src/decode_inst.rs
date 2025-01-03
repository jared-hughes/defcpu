use crate::{
    inst::{
        Base32, Base64, EffAddr, FullInst, Index32, Index64,
        Inst::{self, *},
        JumpXor::*,
        Scale::{self, *},
        RM16, RM32, RM64, RM8, SIDB32, SIDB64,
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

    fn next_u8(&mut self) -> u8 {
        let out = self.mem.read_u8(self.i);
        self.i += 1;
        out
    }

    fn next_imm8(&mut self) -> u8 {
        let out = self.mem.read_u8(self.i);
        self.i += 1;
        out
    }

    fn next_modrm(&mut self) -> ModRM {
        let byte = self.next_u8();
        modrm_decode(byte)
    }

    fn next_i8(&mut self) -> i8 {
        // i32 and u32 are 2's complement, so this is a no-op.
        self.next_imm8() as i8
    }

    fn next_imm16(&mut self) -> u16 {
        let out = self.mem.read_u16(self.i);
        self.i += 2;
        out
    }

    fn next_imm32(&mut self) -> u32 {
        let out = self.mem.read_u32(self.i);
        self.i += 4;
        out
    }

    fn next_i16(&mut self) -> i16 {
        self.next_imm16() as i16
    }

    fn next_i32(&mut self) -> i32 {
        self.next_imm32() as i32
    }

    fn next_imm64(&mut self) -> u64 {
        let out = self.mem.read_u64(self.i);
        self.i += 8;
        out
    }
}

/// Returns instruction together with the number of bytes read.
pub fn decode_inst(mem: &Memory, i: u64) -> (FullInst, u64) {
    let mut lex = Lexer::new(mem, i);
    let inst = decode_inst_inner(&mut lex);
    let len = lex.len();
    (inst.with_prefix(lex.prefix.dis_prefix), len)
}

fn decode_inst_inner(lex: &mut Lexer) -> Inst {
    let opcode = lex.next_u8();
    match opcode {
        0x00 | 0x02 => {
            // 00 /r; ADD r/m8, r8; Add r8 to r/m8.
            // 02 /r; ADD r8, r/m8; Add r/m8 to r8.
            let modrm = lex.next_modrm();
            let reg = lex.reg8_field_select_r(modrm.reg3);
            let rm8 = decode_rm8(lex, &modrm);
            match opcode {
                0x00 => AddMR8(rm8, reg),
                0x02 => AddRM8(reg, rm8),
                _ => panic!("Missing match arm in decode_inst_inner."),
            }
        }
        0x01 | 0x03 => {
            let modrm = lex.next_modrm();
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // 01 /r; ADD r/m16, r16; Add r16 to r/m16.
                    // 03 /r; ADD r16, r/m16; Add r/m16 to r16.
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm);
                    match opcode {
                        0x01 => AddMR16(rm16, reg),
                        0x03 => AddRM16(reg, rm16),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data32 => {
                    // 01 /r; ADD r/m32, r32; Add r32 to r/m32.
                    // 03 /r; ADD r32, r/m32; Add r/m32 to r32.
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm);
                    match opcode {
                        0x01 => AddMR32(rm32, reg),
                        0x03 => AddRM32(reg, rm32),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data64 => {
                    // REX.W + 01 /r; ADD r/m64, r64; Add r64 to r/m64.
                    // REX.W + 03 /r; ADD r64, r/m64; Add r/m64 to r64.
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm);
                    match opcode {
                        0x01 => AddMR64(rm64, reg),
                        0x03 => AddRM64(reg, rm64),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
            }
        }
        0x04 => {
            // 04 ib; ADD AL, imm8; Add imm8 to AL.
            let imm8 = lex.next_imm8();
            AddMI8(RM8::Reg(GPR8::al), imm8)
        }
        0x05 => match lex.get_operand_size() {
            Data16 => {
                // 05 iw; ADD AX, imm16; Add imm16 to AX.
                let imm16 = lex.next_imm16();
                AddMI16(RM16::Reg(GPR16::ax), imm16)
            }
            Data32 => {
                // 05 id; ADD EAX, imm32; Add imm32 to EAX.
                let imm32 = lex.next_imm32();
                AddMI32(RM32::Reg(GPR32::eax), imm32)
            }
            Data64 => {
                // REX.W + 05 id; ADD RAX, imm32; Add imm32 sign-extended to 64-bits to RAX.
                let imm32 = lex.next_imm32();
                AddMI64(RM64::Reg(GPR64::rax), imm32 as i32 as u64)
            }
        },
        0x0F => {
            let opcode2 = lex.next_u8();
            match opcode2 {
                0x05 => Syscall,
                0x80..=0x8F => {
                    let dest = match lex.get_operand_size_no_rexw() {
                        Data16 => {
                            let rel_off = lex.next_i16() as u16;
                            // If the operand-size attribute is 16, the upper two bytes of the EIP register
                            // are cleared, resulting in a maximum instruction pointer size of 16 bits.
                            (lex.i as u16).wrapping_add(rel_off) as u64
                        }
                        Data32 => {
                            let rel_off = lex.next_i32() as u64;
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
                _ => NotImplemented2(opcode, opcode2),
            }
        }
        0x28 | 0x2A => {
            // 28 /r; SUB r/m8, r8; Subtract r8 from r/m8.
            // 2A /r; SUB r8, r/m8; Subtract r/m8 from r8.
            let modrm = lex.next_modrm();
            let reg = lex.reg8_field_select_r(modrm.reg3);
            let rm8 = decode_rm8(lex, &modrm);
            match opcode {
                0x28 => SubMR8(rm8, reg),
                0x2A => SubRM8(reg, rm8),
                _ => panic!("Missing match arm in decode_inst_inner."),
            }
        }
        0x29 | 0x2B => {
            let modrm = lex.next_modrm();
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // 29 /r; SUB r/m16, r16; Subtract r16 from r/m16.
                    // 2B /r; SUB r16, r/m16; Subtract r/m16 from r16.
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm);
                    match opcode {
                        0x29 => SubMR16(rm16, reg),
                        0x2B => SubRM16(reg, rm16),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data32 => {
                    // 29 /r; SUB r/m32, r32; Subtract r32 from r/m32.
                    // 2B /r; SUB r32, r/m32; Subtract r/m32 from r32.
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm);
                    match opcode {
                        0x29 => SubMR32(rm32, reg),
                        0x2B => SubRM32(reg, rm32),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data64 => {
                    // REX.W + 29 /r; SUB r/m64, r64; Subtract r64 from r/m64.
                    // REX.W + 2B /r; SUB r64, r/m64; Subtract r/m64 from r64.
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm);
                    match opcode {
                        0x29 => SubMR64(rm64, reg),
                        0x2B => SubRM64(reg, rm64),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
            }
        }
        0x2C => {
            // 2C ib; SUB AL, imm8; Subtract imm8 from AL.
            let imm8 = lex.next_imm8();
            SubMI8(RM8::Reg(GPR8::al), imm8)
        }
        0x2D => match lex.get_operand_size() {
            Data16 => {
                // 2D iw; SUB AX, imm16; Subtract imm16 from AX.
                let imm16 = lex.next_imm16();
                SubMI16(RM16::Reg(GPR16::ax), imm16)
            }
            Data32 => {
                // 2D id; SUB EAX, imm32; Subtract imm32 from EAX.
                let imm32 = lex.next_imm32();
                SubMI32(RM32::Reg(GPR32::eax), imm32)
            }
            Data64 => {
                // REX.W + 2D id; SUB RAX, imm32; Subtract imm32 sign-extended to 64-bits from RAX.
                let imm32 = lex.next_imm32();
                SubMI64(RM64::Reg(GPR64::rax), imm32 as i32 as u64)
            }
        },
        0x34 => {
            // 34 ib; XOR AL, imm8; AL XOR imm8.
            let imm8 = lex.next_imm8();
            XorMI8(RM8::Reg(GPR8::al), imm8)
        }
        0x35 => match lex.get_operand_size() {
            Data16 => {
                // 35 iw; XOR AX, imm16; AX XOR imm16.
                let imm16 = lex.next_imm16();
                XorMI16(RM16::Reg(GPR16::ax), imm16)
            }
            Data32 => {
                // 35 id; XOR EAX, imm32; EAX XOR imm32.
                let imm32 = lex.next_imm32();
                XorMI32(RM32::Reg(GPR32::eax), imm32)
            }
            Data64 => {
                // REX.W + 35 id; XOR RAX, imm32; RAX XOR imm32 (sign-extended).
                let imm32 = lex.next_imm32();
                XorMI64(RM64::Reg(GPR64::rax), imm32 as i32 as u64)
            }
        },
        0x3C => {
            // 3C ib; CMP AL, imm8; Compare imm8 with AL.
            let imm8 = lex.next_imm8();
            CmpMI8(RM8::Reg(GPR8::al), imm8)
        }
        0x3D => match lex.get_operand_size() {
            Data16 => {
                // 3D iw; CMP AX, imm16; Compare imm16 with AX.
                let imm16 = lex.next_imm16();
                CmpMI16(RM16::Reg(GPR16::ax), imm16)
            }
            Data32 => {
                // 3D id; CMP EAX, imm32; Compare imm32 with EAX.
                let imm32 = lex.next_imm32();
                CmpMI32(RM32::Reg(GPR32::eax), imm32)
            }
            Data64 => {
                // REX.W + 3D id; CMP RAX, imm32; Compare imm32 sign-extended to 64-bits with RAX.
                let imm32 = lex.next_imm32();
                CmpMI64(RM64::Reg(GPR64::rax), imm32 as i32 as u64)
            }
        },
        0x30 | 0x32 | 0x38 | 0x3A => {
            // 30 /r; XOR r/m8, r8; r/m8 XOR r8.
            // 32 /r; XOR r8, r/m8; r8 XOR r/m8.
            // 38 /r; CMP r/m8, r8; Compare r8 with r/m8.
            // 3A /r; CMP r8, r/m8; Compare r/m8 with r8.
            let modrm = lex.next_modrm();
            let reg = lex.reg8_field_select_r(modrm.reg3);
            let rm8 = decode_rm8(lex, &modrm);
            match opcode {
                0x30 => XorMR8(rm8, reg),
                0x32 => XorRM8(reg, rm8),
                0x38 => CmpMR8(rm8, reg),
                0x3A => CmpRM8(reg, rm8),
                _ => panic!("Missing match arm in decode_inst_inner."),
            }
        }
        0x31 | 0x33 | 0x39 | 0x3B => {
            let modrm = lex.next_modrm();
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // 31 /r; XOR r/m16, r16; r/m16 XOR r16.
                    // 33 /r; XOR r16, r/m16; r16 XOR r/m16.
                    // 39 /r; CMP r/m16, r16; Compare r16 with r/m16.
                    // 3B /r; CMP r16, r/m16; Compare r/m16 with r16.
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm);
                    match opcode {
                        0x31 => XorMR16(rm16, reg),
                        0x33 => XorRM16(reg, rm16),
                        0x39 => CmpMR16(rm16, reg),
                        0x3B => CmpRM16(reg, rm16),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data32 => {
                    // 31 /r; XOR r/m32, r32; r/m32 XOR r32.
                    // 33 /r; XOR r32, r/m32; r32 XOR r/m32.
                    // 39 /r; CMP r/m32, r32; Compare r32 with r/m32.
                    // 3B /r; CMP r32, r/m32; Compare r/m32 with r32.
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm);
                    match opcode {
                        0x31 => XorMR32(rm32, reg),
                        0x33 => XorRM32(reg, rm32),
                        0x39 => CmpMR32(rm32, reg),
                        0x3B => CmpRM32(reg, rm32),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data64 => {
                    // REX.W + 31 /r; XOR r/m64, r64; r/m64 XOR r64.
                    // REX.W + 33 /r; XOR r64, r/m64; r64 XOR r/m64.
                    // REX.W + 39 /r; CMP r/m64, r64; Compare r64 with r/m64.
                    // REX.W + 3B /r; CMP r64, r/m64; Compare r/m64 with r64.
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm);
                    match opcode {
                        0x31 => XorMR64(rm64, reg),
                        0x33 => XorRM64(reg, rm64),
                        0x39 => CmpMR64(rm64, reg),
                        0x3B => CmpRM64(reg, rm64),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
            }
        }
        0x40..=0x4F => {
            // 0x40..=0x4F REX prefix. Must be immediately followed by opcode byte.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return RexNoop;
            }
            lex.prefix = lex.prefix.with_rex(opcode);
            decode_inst_inner(lex)
        }
        0x50..=0x57 => {
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
        0x66 => {
            // 0x66 Operand size prefix.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return RexNoop;
            }
            lex.prefix = lex.prefix.with_operand_size_prefix();
            decode_inst_inner(lex)
        }
        0x67 => {
            // 0x67 Address size prefix.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return RexNoop;
            }
            lex.prefix = lex.prefix.with_address_size_prefix();
            decode_inst_inner(lex)
        }
        0x68 => {
            // 0x68
            match lex.get_operand_size_64_default() {
                Data16 => {
                    // 68 iw; PUSH imm16; Push imm16.
                    let imm16 = lex.next_imm16();
                    PushI16(imm16)
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    // 68 id; PUSH imm32; Push imm32.
                    let imm32 = lex.next_imm32();
                    PushI64(imm32 as i32 as u64)
                }
            }
        }
        0x6A => {
            // 6A ib; PUSH imm8; Push imm8.
            match lex.get_operand_size_64_default() {
                Data16 => {
                    let imm8 = lex.next_imm8();
                    PushI16(imm8 as i8 as u16)
                }
                Data32 => panic!("Impossible Data32 with 64-bit default."),
                Data64 => {
                    let imm8 = lex.next_imm8();
                    PushI64(imm8 as i8 as u64)
                }
            }
        }
        0x70..=0x7F => {
            let rel_off = lex.next_i8() as u64;
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
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // 80 /0 ib; ADD r/m8, imm8; Add imm8 to r/m8.
                    let rm8 = decode_rm8(lex, &modrm);
                    let imm8 = lex.next_imm8();
                    AddMI8(rm8, imm8)
                }
                5 => {
                    // 80 /5 ib; SUB r/m8, imm8; Subtract imm8 from r/m8.
                    let rm8 = decode_rm8(lex, &modrm);
                    let imm8 = lex.next_imm8();
                    SubMI8(rm8, imm8)
                }
                6 => {
                    // 80 /6 ib; XOR r/m8, imm8; r/m8 XOR imm8.
                    let rm8 = decode_rm8(lex, &modrm);
                    let imm8 = lex.next_imm8();
                    XorMI8(rm8, imm8)
                }
                7 => {
                    // 80 /7 ib; CMP r/m8, imm8; Compare imm8 with r/m8.
                    let rm8 = decode_rm8(lex, &modrm);
                    let imm8 = lex.next_imm8();
                    CmpMI8(rm8, imm8)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0x81 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // 81 /0
                    match lex.get_operand_size() {
                        Data16 => {
                            // 81 /0 iw; ADD r/m16, imm16; Add imm16 to r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm16 = lex.next_imm16();
                            AddMI16(rm16, imm16)
                        }
                        Data32 => {
                            // 81 /0 id; ADD r/m32, imm32; Add imm32 to r/m32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            AddMI32(rm32, imm32)
                        }
                        Data64 => {
                            // REX.W + 81 /0 id; ADD r/m64, imm32; Add imm32 sign-extended to 64-bits to r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            // Sign extend imm32 to u64.
                            AddMI64(rm64, imm32 as i32 as u64)
                        }
                    }
                }
                5 => {
                    // 81 /5
                    match lex.get_operand_size() {
                        Data16 => {
                            // 81 /5 iw; SUB r/m16, imm16; Subtract imm16 from r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm16 = lex.next_imm16();
                            SubMI16(rm16, imm16)
                        }
                        Data32 => {
                            // 81 /5 id; SUB r/m32, imm32; Subtract imm32 from r/m32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            SubMI32(rm32, imm32)
                        }
                        Data64 => {
                            // REX.W + 81 /5 id; SUB r/m64, imm32; Subtract imm32 sign-extended to 64-bits from r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            // Sign extend imm32 to u64.
                            SubMI64(rm64, imm32 as i32 as u64)
                        }
                    }
                }
                6 => {
                    // 81 /6
                    match lex.get_operand_size() {
                        Data16 => {
                            // 81 /6 iw; XOR r/m16, imm16; r/m16 XOR imm16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm16 = lex.next_imm16();
                            XorMI16(rm16, imm16)
                        }
                        Data32 => {
                            // 81 /6 id; XOR r/m32, imm32; r/m32 XOR imm32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            XorMI32(rm32, imm32)
                        }
                        Data64 => {
                            // REX.W + 81 /6 id; XOR r/m64, imm32; r/m64 XOR imm32 (sign-extended).
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            // Sign extend imm32 to u64.
                            XorMI64(rm64, imm32 as i32 as u64)
                        }
                    }
                }
                7 => {
                    // 81 /7
                    match lex.get_operand_size() {
                        Data16 => {
                            // 81 /7 iw; CMP r/m16, imm16; Compare imm16 with r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm16 = lex.next_imm16();
                            CmpMI16(rm16, imm16)
                        }
                        Data32 => {
                            // 81 /7 id; CMP r/m32, imm32; Compare imm32 with r/m32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            CmpMI32(rm32, imm32)
                        }
                        Data64 => {
                            // REX.W + 81 /7 id; CMP r/m64, imm32; Compare imm32 sign-extended to 64-bits with r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            // Sign extend imm32 to u64.
                            CmpMI64(rm64, imm32 as i32 as u64)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0x83 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // 83 /0
                    match lex.get_operand_size() {
                        Data16 => {
                            // 83 /0 ib; ADD r/m16, imm8; Add sign-extended imm8 to r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            AddMI16(rm16, imm8 as i8 as u16)
                        }
                        Data32 => {
                            // 83 /0 ib; ADD r/m32, imm8; Add sign-extended imm8 to r/m32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            AddMI32(rm32, imm8 as i8 as u32)
                        }
                        Data64 => {
                            // REX.W + 83 /0 ib; ADD r/m64, imm8; Add sign-extended imm8 to r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            AddMI64(rm64, imm8 as i8 as u64)
                        }
                    }
                }
                5 => {
                    // 83 /5
                    match lex.get_operand_size() {
                        Data16 => {
                            // 83 /5 ib; SUB r/m16, imm8; Subtract sign-extended imm8 from r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            SubMI16(rm16, imm8 as i8 as u16)
                        }
                        Data32 => {
                            // 83 /5 ib; SUB r/m32, imm8; Subtract sign-extended imm8 from r/m32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            SubMI32(rm32, imm8 as i8 as u32)
                        }
                        Data64 => {
                            // REX.W + 83 /5 ib; SUB r/m64, imm8; Subtract sign-extended imm8 from r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            SubMI64(rm64, imm8 as i8 as u64)
                        }
                    }
                }
                6 => {
                    // 83 /6
                    match lex.get_operand_size() {
                        Data16 => {
                            // 83 /6 ib; XOR r/m16, imm8; r/m16 XOR imm8 (sign-extended).
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            XorMI16(rm16, imm8 as i8 as u16)
                        }
                        Data32 => {
                            // 83 /6 ib; XOR r/m32, imm8; r/m32 XOR imm8 (sign-extended).
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            XorMI32(rm32, imm8 as i8 as u32)
                        }
                        Data64 => {
                            // REX.W + 83 /6 ib; XOR r/m64, imm8; r/m64 XOR imm8 (sign-extended).
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            XorMI64(rm64, imm8 as i8 as u64)
                        }
                    }
                }
                7 => {
                    // 83 /7
                    match lex.get_operand_size() {
                        Data16 => {
                            // 83 /7 ib; CMP r/m16, imm8; Compare sign-extended imm8 with r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            CmpMI16(rm16, imm8 as i8 as u16)
                        }
                        Data32 => {
                            // 83 /7 ib; CMP r/m32, imm8; Compare sign-extended imm8 with r/m32.
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            CmpMI32(rm32, imm8 as i8 as u32)
                        }
                        Data64 => {
                            // REX.W + 83 /7 ib; CMP r/m64, imm8; Compare sign-extended imm8 with r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm8 = lex.next_imm8();
                            CmpMI64(rm64, imm8 as i8 as u64)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0x88 | 0x8A => {
            // 88 /r; MOV r/m8, r8; Move r8 to r/m8.
            // 8A /r; MOV r8, r/m8; Move r/m8 to r8.
            let modrm = lex.next_modrm();
            let reg = lex.reg8_field_select_r(modrm.reg3);
            let rm8 = decode_rm8(lex, &modrm);
            match opcode {
                0x88 => MovMR8(rm8, reg),
                0x8A => MovRM8(reg, rm8),
                _ => panic!("Missing match arm in decode_inst_inner."),
            }
        }
        0x89 | 0x8B => {
            let modrm = lex.next_modrm();
            let rex_r = lex.get_rex_r_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // 8B /r; MOV r16, r/m16; Move r/m16 to r16.
                    // 89 /r; MOV r/m16, r16; Move r16 to r/m16.
                    let reg = reg16_field_select(modrm.reg3, rex_r);
                    let rm16 = decode_rm16(lex, &modrm);
                    match opcode {
                        0x89 => MovMR16(rm16, reg),
                        0x8B => MovRM16(reg, rm16),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data32 => {
                    // 8B /r; MOV r32, r/m32; Move r/m32 to r32.
                    // 89 /r; MOV r/m32, r32; Move r32 to r/m32.
                    let reg = reg32_field_select(modrm.reg3, rex_r);
                    let rm32 = decode_rm32(lex, &modrm);
                    match opcode {
                        0x89 => MovMR32(rm32, reg),
                        0x8B => MovRM32(reg, rm32),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data64 => {
                    // REX.W + 8B /r; MOV r64, r/m64; Move r/m64 to r64.
                    // REX.W + 89 /r; MOV r/m64, r64; Move r64 to r/m64.
                    let reg = reg64_field_select(modrm.reg3, rex_r);
                    let rm64 = decode_rm64(lex, &modrm);
                    match opcode {
                        0x89 => MovMR64(rm64, reg),
                        0x8B => MovRM64(reg, rm64),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
            }
        }
        0x8F => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // 8F /0
                    match lex.get_operand_size_64_default() {
                        Data16 => {
                            // 8F /0; POP r/m16; Pop top of stack into m16; increment stack pointer.
                            let rm16 = decode_rm16(lex, &modrm);
                            PopM16(rm16)
                        }
                        Data32 => panic!("Impossible Data32 with 64-bit default."),
                        Data64 => {
                            // 8F /0; POP r/m64; Pop top of stack into m64; increment stack pointer. Cannot encode 32-bit operand size.
                            let rm64 = decode_rm64(lex, &modrm);
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
        0xB0..=0xB7 => {
            // B0+ rb ib; MOV r8, imm8
            // Move imm8 to r8.
            let imm8 = lex.next_imm8();
            let reg = lex.reg8_field_select_b(opcode);
            MovOI8(reg, imm8)
        }
        0xB8..=0xBF => {
            let rex_b = lex.get_rex_b_matters();
            match lex.get_operand_size() {
                Data16 => {
                    // B8+ rw iw; MOV r16, imm16
                    let imm16 = lex.next_imm16();
                    let reg = reg16_field_select(opcode, rex_b);
                    MovOI16(reg, imm16)
                }
                Data32 => {
                    // B8+ rd id; MOV r32, imm32
                    let imm32 = lex.next_imm32();
                    let reg = reg32_field_select(opcode, rex_b);
                    MovOI32(reg, imm32)
                }
                Data64 => {
                    // REX.W + B8+ rd io
                    let imm64 = lex.next_imm64();
                    let reg = reg64_field_select(opcode, rex_b);
                    MovOI64(reg, imm64)
                }
            }
        }
        0xC6 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // C6 /0 ib; MOV r/m8, imm16
                    let rm8 = decode_rm8(lex, &modrm);
                    let imm8 = lex.next_imm8();
                    MovMI8(rm8, imm8)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xC7 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // C7 /0
                    match lex.get_operand_size() {
                        Data16 => {
                            // C7 /0 iw; MOV r/m16, imm16
                            let rm16 = decode_rm16(lex, &modrm);
                            let imm16 = lex.next_imm16();
                            MovMI16(rm16, imm16)
                        }
                        Data32 => {
                            // C7 /0 id; MOV r/m32, imm32
                            let rm32 = decode_rm32(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            MovMI32(rm32, imm32)
                        }
                        Data64 => {
                            // REX.W + C7 /0 id; MOV r/m64, imm32
                            let rm64 = decode_rm64(lex, &modrm);
                            let imm32 = lex.next_imm32();
                            // Sign extend imm32 to u64.
                            MovMI64(rm64, imm32 as i32 as u64)
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
            let rel_off = lex.next_i8() as u64;
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
            // gdb dis (new cases for 015_jmp.s):
            //   e9 12 34 56 78              jmp    0x78963420
            //   66 e9 12 34                 jmpw   0x342a
            //   48 e9 12 34 56 78           rex.W jmp 0x7896346b
            //   66 48 e9 12 34 56 78        data16 rex.W jmp 0x78963479
            match lex.get_operand_size_no_rexw() {
                Data16 => todo!("Operand-size prefix 0x66 behaves funny with jmp 0xE9. I haven't reverse-engineered it yet."),
                Data32 => {
                    let rel_off = lex.next_i32() as u64;
                    let dest = lex.i.wrapping_add(rel_off);
                    JmpD(dest)
                }
                Data64 => panic!("Impossible Data64 without REX.W bit."),
            }
        }
        0xEB => {
            // EB cb; JMP rel8; Jump short, RIP = RIP + 8-bit displacement sign extended to 64-bits.
            let rel_off = lex.next_i8() as u64;
            let dest = lex.i.wrapping_add(rel_off);
            JmpD(dest)
        }
        0xF4 => {
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return RexNoop;
            }
            Hlt
        }
        0xF6 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                6 => {
                    // F6 /6; DIV r/m8; Unsigned divide AX by r/m8, with result stored in AL := Quotient, AH := Remainder.
                    let rm8 = decode_rm8(lex, &modrm);
                    DivM8(rm8)
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xF7 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                6 => {
                    // F7 /6
                    match lex.get_operand_size() {
                        Data16 => {
                            // F7 /6; DIV r/m16; Unsigned divide DX:AX by r/m16, with result stored in AX := Quotient, DX := Remainder.
                            let rm16 = decode_rm16(lex, &modrm);
                            DivM16(rm16)
                        }
                        Data32 => {
                            // F7 /6; DIV r/m32; Unsigned divide EDX:EAX by r/m32, with result stored in EAX := Quotient, EDX := Remainder.
                            let rm32 = decode_rm32(lex, &modrm);
                            DivM32(rm32)
                        }
                        Data64 => {
                            // REX.W + F7 /6; DIV r/m64; Unsigned divide RDX:RAX by r/m64, with result stored in RAX := Quotient, RDX := Remainder.
                            let rm64 = decode_rm64(lex, &modrm);
                            DivM64(rm64)
                        }
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xFE => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 | 1 => {
                    // FE /0; INC r/m8; Increment r/m byte by 1.
                    // FE /1; DEC r/m8; Decrement r/m byte by 1.
                    let rm8 = decode_rm8(lex, &modrm);
                    match modrm.reg3 {
                        0 => IncM8(rm8),
                        1 => DecM8(rm8),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                _ => {
                    lex.rollback();
                    NotImplementedOpext(opcode, modrm.reg3)
                }
            }
        }
        0xFF => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 | 1 => {
                    // FF /0, FF /1
                    match lex.get_operand_size() {
                        Data16 => {
                            // FF /0; INC r/m16; Increment r/m word by 1.
                            // FF /1; DEC r/m16; Decrement r/m word by 1.
                            let rm16 = decode_rm16(lex, &modrm);
                            match modrm.reg3 {
                                0 => IncM16(rm16),
                                1 => DecM16(rm16),
                                _ => panic!("Missing match arm in decode_inst_inner."),
                            }
                        }
                        Data32 => {
                            // FF /0; INC r/m32; Increment r/m doubleword by 1.
                            // FF /1; DEC r/m32; Decrement r/m doubleword by 1.
                            let rm32 = decode_rm32(lex, &modrm);
                            match modrm.reg3 {
                                0 => IncM32(rm32),
                                1 => DecM32(rm32),
                                _ => panic!("Missing match arm in decode_inst_inner."),
                            }
                        }
                        Data64 => {
                            // REX.W + FF /0; INC r/m64; Increment r/m quadword by 1.
                            // REX.W + FF /1; DEC r/m64; Decrement r/m quadword by 1.
                            let rm64 = decode_rm64(lex, &modrm);
                            match modrm.reg3 {
                                0 => IncM64(rm64),
                                1 => DecM64(rm64),
                                _ => panic!("Missing match arm in decode_inst_inner."),
                            }
                        }
                    }
                }
                4 => {
                    // FF /4; JMP r/m64; Jump near, absolute indirect, RIP = 64-Bit offset from register or memory.
                    let rm64 = decode_rm64(lex, &modrm);
                    JmpM64(rm64)
                }
                6 => {
                    // FF /6
                    match lex.get_operand_size_64_default() {
                        Data16 => {
                            // FF /6; PUSH r/m16; Push r/m16.
                            let rm16 = decode_rm16(lex, &modrm);
                            PushM16(rm16)
                        }
                        Data32 => panic!("Impossible Data32 with 64-bit default."),
                        Data64 => {
                            // FF /6; PUSH r/m64; Push r/m64.
                            let rm64 = decode_rm64(lex, &modrm);
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
    }
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

// r/m8
fn decode_rm8(lex: &mut Lexer, modrm: &ModRM) -> RM8 {
    match modrm.mod2 {
        0b00..=0b10 => RM8::Addr(decode_rm_00_01_10(lex, modrm)),
        0b11 => {
            let reg = lex.reg8_field_select_b(modrm.rm3);
            RM8::Reg(reg)
        }
        _ => panic!("Missing match arm in modrm_decode_addr8."),
    }
}

// r/m16
fn decode_rm16(lex: &mut Lexer, modrm: &ModRM) -> RM16 {
    match modrm.mod2 {
        0b00..=0b10 => RM16::Addr(decode_rm_00_01_10(lex, modrm)),
        0b11 => {
            let rexb = lex.get_rex_b_matters();
            RM16::Reg(reg16_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr16."),
    }
}

// r/m32
fn decode_rm32(lex: &mut Lexer, modrm: &ModRM) -> RM32 {
    match modrm.mod2 {
        0b00..=0b10 => RM32::Addr(decode_rm_00_01_10(lex, modrm)),
        0b11 => {
            let rexb = lex.get_rex_b_matters();
            RM32::Reg(reg32_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr32."),
    }
}

// r/m64
fn decode_rm64(lex: &mut Lexer, modrm: &ModRM) -> RM64 {
    match modrm.mod2 {
        0b00..=0b10 => RM64::Addr(decode_rm_00_01_10(lex, modrm)),
        0b11 => {
            let rex_b = lex.get_rex_b_matters();
            RM64::Reg(reg64_field_select(modrm.rm3, rex_b))
        }
        _ => panic!("Missing match arm in modrm_decode_addr64."),
    }
}

/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
/// The table only provides (address_size, rexb) = (Addr32, false). Rest are guesses.
/// 2.2.1.3 Displacement specifies address_size == Addr64 uses the same encodings,
/// but with addresses 64-bit regs instead of 32-bit. However displacement remains the same size (8/32 bits).
fn decode_rm_00_01_10(lex: &mut Lexer, modrm: &ModRM) -> EffAddr {
    let rex_b = lex.get_rex_b_matters();
    let address_size = lex.get_address_size();
    if modrm.mod2 == 0b00 && modrm.rm3 == 0b101 {
        // Special case: Instead of encoding a plain [ebp] (rexb=0) or [r13] (rexb=1),
        // it encodes as disp32 instead. Since we are in 64-bit mode,
        // it's actually a RIP-relative disp32.
        // See Vol 2A: 2.2.1.6 "RIP-Relative Addressing", and Table 2-7.
        return match address_size {
            Addr32 => EffAddr::EffAddr32(SIDB32 {
                disp: Some(lex.next_i32()),
                base: Some(Base32::Eip),
                index: None,
                scale: Scale1,
            }),
            Addr64 => EffAddr::EffAddr64(SIDB64 {
                disp: Some(lex.next_i32()),
                base: Some(Base64::Rip),
                index: None,
                scale: Scale1,
            }),
        };
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
            let sib = sib_decode(lex.next_u8());
            match lex.prefix.address_size {
                Addr32 => decode_sib_addr32(lex, &sib, modrm),
                Addr64 => decode_sib_addr64(lex, &sib, modrm),
            }
        }
        (Addr32, false, 0b101) => EffAddr::from_base_reg32(GPR32::ebp),
        (Addr64, false, 0b101) => EffAddr::from_base_reg64(GPR64::rbp),
        (Addr32, false, 0b110) => EffAddr::from_base_reg32(GPR32::esi),
        (Addr64, false, 0b110) => EffAddr::from_base_reg64(GPR64::rsi),
        (Addr32, false, 0b111) => EffAddr::from_base_reg32(GPR32::edi),
        (Addr64, false, 0b111) => EffAddr::from_base_reg64(GPR64::rdi),
        // TODO: All of these (true, ) fields are guesses.
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
    match modrm.mod2 {
        0b00 => sidb,
        0b01 => {
            // The `as i32` sign-extends
            sidb.with_disp(Some(lex.next_i8() as i32))
        }
        0b10 => sidb.with_disp(Some(lex.next_i32())),
        _ => panic!("Missing match arm."),
    }
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
fn decode_sib_addr32(lex: &mut Lexer, sib: &SIB, modrm: &ModRM) -> EffAddr {
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
        return EffAddr::EffAddr32(SIDB32 {
            base: None,
            disp: Some(lex.next_i32()),
            index: Some(index_reg),
            scale: sib.scale,
        });
    }
    EffAddr::EffAddr32(SIDB32 {
        base: Some(Base32::GPR32(base_reg)),
        // The callee fills in disp
        disp: None,
        index: Some(index_reg),
        scale: sib.scale,
    })
}

/// 64-bit address mode variation on the 32-bit analog:
/// Vol 2A: Table 2-3. 32-Bit Addressing Forms with the SIB Byte
fn decode_sib_addr64(lex: &mut Lexer, sib: &SIB, modrm: &ModRM) -> EffAddr {
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
        return EffAddr::EffAddr64(SIDB64 {
            base: None,
            // The disp is still 32-bit despite being 64-bit addressing.
            disp: Some(lex.next_i32()),
            index: Some(index_reg),
            scale: sib.scale,
        });
    }
    EffAddr::EffAddr64(SIDB64 {
        base: Some(Base64::GPR64(base_reg)),
        // The callee fills in disp
        disp: None,
        index: Some(index_reg),
        scale: sib.scale,
    })
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
