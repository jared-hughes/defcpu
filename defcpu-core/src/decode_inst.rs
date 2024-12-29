use crate::{
    inst::{
        Base32, Base64, EffAddr, FullInst, Index32, Index64,
        Inst::{self, *},
        Scale::{self, *},
        RM16, RM32, RM64, RM8, SIDB32, SIDB64,
    },
    inst_prefixes::{AddressSizeAttribute::*, OperandSizeAttribute::*, Prefix},
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

    fn next_i32(&mut self) -> i32 {
        // i32 and u32 are 2's complement, so this is a no-op.
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
    let mut operand_size = lex.prefix.operand_size;
    let opcode = lex.next_u8();
    match opcode {
        0x40..=0x4F => {
            // 0x40..=0x4F REX prefix. Must be immediately followed by opcode byte.
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return RexNoop;
            }
            lex.prefix = lex.prefix.with_rex(opcode);
            decode_inst_inner(lex)
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
        0x88 | 0x8A => {
            let modrm = lex.next_modrm();
            let (reg, rex_r_matters) = reg8_field_select(modrm.reg3, lex.prefix.rex.map(|x| x.r));
            lex.rex_r_mattered |= rex_r_matters;
            lex.rex_presence_mattered |= rex_r_matters;
            let rm8 = decode_rm8(lex, &modrm);
            lex.maybe_remove_rex();
            match opcode {
                0x88 => MovMR8(rm8, reg),
                0x8A => MovRM8(reg, rm8),
                _ => panic!("Missing match arm in decode_inst_inner."),
            }
        }
        0x89 | 0x8B => {
            let modrm = lex.next_modrm();
            if let Some(rex) = lex.prefix.rex {
                if rex.w {
                    operand_size = Data64
                }
                lex.rex_w_mattered = true;
                lex.rex_b_mattered = true;
                lex.rex_r_mattered = true;
                lex.maybe_remove_rex();
            };
            match operand_size {
                Data16 => {
                    lex.prefix.dis_prefix.remove_last_operand_size_prefix();
                    // 8B /r; MOV r16, r/m16; Move r/m16 to r16.
                    let reg = reg16_field_select(
                        modrm.reg3,
                        lex.prefix.rex.map(|x| x.r).unwrap_or(false),
                    );
                    let rm16 = decode_rm16(lex, &modrm);
                    match opcode {
                        0x89 => MovMR16(rm16, reg),
                        0x8B => MovRM16(reg, rm16),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data32 => {
                    lex.prefix.dis_prefix.remove_last_operand_size_prefix();
                    // 8B /r; MOV r32, r/m32; Move r/m32 to r32.
                    let reg = reg32_field_select(
                        modrm.reg3,
                        lex.prefix.rex.map(|x| x.r).unwrap_or(false),
                    );
                    let rm32 = decode_rm32(lex, &modrm);
                    match opcode {
                        0x89 => MovMR32(rm32, reg),
                        0x8B => MovRM32(reg, rm32),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
                Data64 => {
                    // REX.W + 8B /r; MOV r64, r/m64; Move r/m64 to r64.
                    let reg = reg64_field_select(
                        modrm.reg3,
                        lex.prefix.rex.map(|x| x.r).unwrap_or(false),
                    );
                    let rm64 = decode_rm64(lex, &modrm);
                    match opcode {
                        0x89 => MovMR64(rm64, reg),
                        0x8B => MovRM64(reg, rm64),
                        _ => panic!("Missing match arm in decode_inst_inner."),
                    }
                }
            }
        }
        0xB0..=0xB7 => {
            // B0+ rb ib; MOV r8, imm8
            // Move imm8 to r8.
            let imm8 = lex.next_imm8();
            let (reg, rex_b_matters) = reg8_field_select(opcode, lex.prefix.rex.map(|r| r.b));
            lex.rex_b_mattered |= rex_b_matters;
            lex.rex_presence_mattered |= rex_b_matters;
            lex.maybe_remove_rex();
            MovOI8(reg, imm8)
        }
        0xB8..=0xBF => {
            if let Some(rex) = lex.prefix.rex {
                if rex.w {
                    operand_size = Data64
                }
                lex.rex_w_mattered = true;
                lex.rex_b_mattered = true;
                lex.maybe_remove_rex();
            };
            match operand_size {
                Data16 => {
                    // B8+ rw iw; MOV r16, imm16
                    lex.prefix.dis_prefix.remove_last_operand_size_prefix();
                    let imm16 = lex.next_imm16();
                    let reg =
                        reg16_field_select(opcode, lex.prefix.rex.map(|r| r.b).unwrap_or(false));
                    MovOI16(reg, imm16)
                }
                Data32 => {
                    // B8+ rd id; MOV r32, imm32
                    lex.prefix.dis_prefix.remove_last_operand_size_prefix();
                    let imm32 = lex.next_imm32();
                    let reg =
                        reg32_field_select(opcode, lex.prefix.rex.map(|r| r.b).unwrap_or(false));
                    MovOI32(reg, imm32)
                }
                Data64 => {
                    // REX.W + B8+ rd io
                    let imm64 = lex.next_imm64();
                    let reg =
                        reg64_field_select(opcode, lex.prefix.rex.map(|r| r.b).unwrap_or(false));
                    MovOI64(reg, imm64)
                }
            }
        }
        0xC6 => {
            let modrm = lex.next_modrm();
            match modrm.reg3 {
                0 => {
                    // C6 /0 ib; MOV r/m8, imm16
                    // REX + C6 /0 ib; MOV r/m8, imm16
                    let rm8 = decode_rm8(lex, &modrm);
                    let imm8 = lex.next_imm8();
                    lex.maybe_remove_rex();
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
                    lex.prefix.dis_prefix.remove_last_operand_size_prefix();
                    if let Some(rex) = lex.prefix.rex {
                        if rex.w {
                            operand_size = Data64
                        }
                        lex.rex_w_mattered = true;
                        lex.maybe_remove_rex();
                    }
                    match operand_size {
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
        0xF4 => {
            if lex.prefix.rex.is_some() {
                lex.rollback();
                return RexNoop;
            }
            Hlt
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
            let rexb_opt = lex.prefix.rex.map(|r| r.b);
            let (reg, rex_b_matters) = reg8_field_select(modrm.rm3, rexb_opt);
            lex.rex_b_mattered |= rex_b_matters;
            lex.rex_presence_mattered |= rex_b_matters;
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
            let rexb = lex.prefix.rex.map(|r| r.b).unwrap_or(false);
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
            let rexb = lex.prefix.rex.map(|r| r.b).unwrap_or(false);
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
            let rexb = lex.prefix.rex.map(|r| r.b).unwrap_or(false);
            RM64::Reg(reg64_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr64."),
    }
}

/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
/// The table only provides (address_size, rexb) = (Addr32, false). Rest are guesses.
/// 2.2.1.3 Displacement specifies address_size == Addr64 uses the same encodings,
/// but with addresses 64-bit regs instead of 32-bit. However displacement remains the same size (8/32 bits).
fn decode_rm_00_01_10(lex: &mut Lexer, modrm: &ModRM) -> EffAddr {
    let rexb = lex.prefix.rex.map(|r| r.b).unwrap_or(false);
    lex.rex_b_mattered = true;
    // Address size matters, so we don't need to show it.
    lex.prefix.dis_prefix.remove_last_address_size_prefix();
    let address_size = lex.prefix.address_size;
    if modrm.mod2 == 0b00 && (rexb, modrm.rm3) == (false, 0b101) {
        // Special case: Instead of encoding a plain [ebp],
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
    let sidb = match (address_size, rexb, modrm.rm3) {
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
    // TODO: Does REX.B, REX.W, REX factor in?
    // If so, use reg32_field_select.
    let rex_x = lex.prefix.rex.map(|r| r.x).unwrap_or(false);
    lex.rex_x_mattered = true;
    let index_reg = match (rex_x, sib.index3) {
        (false, 0b000) => Index32::GPR32(GPR32::eax),
        (false, 0b001) => Index32::GPR32(GPR32::ecx),
        (false, 0b010) => Index32::GPR32(GPR32::edx),
        (false, 0b011) => Index32::GPR32(GPR32::ebx),
        (false, 0b100) => {
            // 'none'
            Index32::Eiz
        }
        (false, 0b101) => Index32::GPR32(GPR32::ebp),
        (false, 0b110) => Index32::GPR32(GPR32::esi),
        (false, 0b111) => Index32::GPR32(GPR32::edi),
        (true, 0b000) => Index32::GPR32(GPR32::r8d),
        (true, 0b001) => Index32::GPR32(GPR32::r9d),
        (true, 0b010) => Index32::GPR32(GPR32::r10d),
        (true, 0b011) => Index32::GPR32(GPR32::r11d),
        (true, 0b100) => Index32::GPR32(GPR32::r12d),
        (true, 0b101) => Index32::GPR32(GPR32::r13d),
        (true, 0b110) => Index32::GPR32(GPR32::r14d),
        (true, 0b111) => Index32::GPR32(GPR32::r15d),
        _ => panic!("Missing match arm."),
    };
    let rex_b = lex.prefix.rex.map(|r| r.b).unwrap_or(false);
    let base_reg = match (rex_b, sib.base3) {
        (false, 0b000) => GPR32::eax,
        (false, 0b001) => GPR32::ecx,
        (false, 0b010) => GPR32::edx,
        (false, 0b011) => GPR32::ebx,
        (false, 0b100) => GPR32::esp,
        (false, 0b101) => match modrm.mod2 {
            0b00 => {
                // Very special case: [*] in the table
                // (Vol 2A: Table 2-3 "32-Bit Addressing Forms with the SIB Byte")
                // [*] when the MOD is 0b00 means disp32 with no base.
                return EffAddr::EffAddr32(SIDB32 {
                    base: None,
                    disp: Some(lex.next_i32()),
                    index: Some(index_reg),
                    scale: sib.scale,
                });
            }
            // If the mod is 01 or 10, the base is [ebp], and the mod
            // bits will automatically leads to adding a disp8 or disp32.
            0b01 | 0b10 => GPR32::ebp,
            _ => panic!("Missing match arm."),
        },
        (false, 0b110) => GPR32::esi,
        (false, 0b111) => GPR32::edi,
        (true, 0b000) => GPR32::r8d,
        (true, 0b001) => GPR32::r9d,
        (true, 0b010) => GPR32::r10d,
        (true, 0b011) => GPR32::r11d,
        (true, 0b100) => GPR32::r12d,
        // TODO: not sure if the (true, 0b101) case should be the same.
        (true, 0b101) => GPR32::r13d,
        (true, 0b110) => GPR32::r14d,
        (true, 0b111) => GPR32::r15d,
        _ => panic!("Missing match arm."),
    };
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
    // TODO: Does REX.B, REX.W, REX factor in?
    // If so, use reg32_field_select.
    let rex_x = lex.prefix.rex.map(|r| r.x).unwrap_or(false);
    lex.rex_x_mattered = true;
    let index_reg = match (rex_x, sib.index3) {
        (false, 0b000) => Index64::GPR64(GPR64::rax),
        (false, 0b001) => Index64::GPR64(GPR64::rcx),
        (false, 0b010) => Index64::GPR64(GPR64::rdx),
        (false, 0b011) => Index64::GPR64(GPR64::rbx),
        (false, 0b100) => {
            // 'none'
            Index64::Riz
        }
        (false, 0b101) => Index64::GPR64(GPR64::rbp),
        (false, 0b110) => Index64::GPR64(GPR64::rsi),
        (false, 0b111) => Index64::GPR64(GPR64::rdi),
        (true, 0b000) => Index64::GPR64(GPR64::r8),
        (true, 0b001) => Index64::GPR64(GPR64::r9),
        (true, 0b010) => Index64::GPR64(GPR64::r10),
        (true, 0b011) => Index64::GPR64(GPR64::r11),
        (true, 0b100) => Index64::GPR64(GPR64::r12),
        (true, 0b101) => Index64::GPR64(GPR64::r13),
        (true, 0b110) => Index64::GPR64(GPR64::r14),
        (true, 0b111) => Index64::GPR64(GPR64::r15),
        _ => panic!("Missing match arm."),
    };
    let rex_b = lex.prefix.rex.map(|r| r.b).unwrap_or(false);
    let base_reg = match (rex_b, sib.base3) {
        (false, 0b000) => GPR64::rax,
        (false, 0b001) => GPR64::rcx,
        (false, 0b010) => GPR64::rdx,
        (false, 0b011) => GPR64::rbx,
        (false, 0b100) => GPR64::rsp,
        (false, 0b101) => match modrm.mod2 {
            0b00 => {
                // Very special case: [*] in the table
                // (Vol 2A: Table 2-3 "32-Bit Addressing Forms with the SIB Byte")
                // [*] when the MOD is 0b00 means disp32 with no base.
                return EffAddr::EffAddr64(SIDB64 {
                    base: None,
                    disp: Some(lex.next_i32()),
                    index: Some(index_reg),
                    scale: sib.scale,
                });
            }
            // If the mod is 01 or 10, the base is [ebp], and the mod
            // bits will automatically leads to adding a disp8 or disp32.
            0b01 | 0b10 => GPR64::rbp,
            _ => panic!("Missing match arm."),
        },
        (false, 0b110) => GPR64::rsi,
        (false, 0b111) => GPR64::rdi,
        (true, 0b000) => GPR64::r8,
        (true, 0b001) => GPR64::r9,
        (true, 0b010) => GPR64::r10,
        (true, 0b011) => GPR64::r11,
        (true, 0b100) => GPR64::r12,
        // TODO: not sure if the (true, 0b101) case should be the same.
        (true, 0b101) => GPR64::r13,
        (true, 0b110) => GPR64::r14,
        (true, 0b111) => GPR64::r15,
        _ => panic!("Missing match arm."),
    };
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
