use crate::{
    inst::{EffAddr, FullInst, Inst::*, RM16, RM32, RM64, RM8},
    inst_prefixes::{AddressSizeAttribute::*, OperandSizeAttribute::*, Prefix},
    memory::Memory,
    registers::{GPR16, GPR32, GPR64, GPR8},
};

/// Returns instruction together with the number of bytes read.
pub fn decode_inst(mem: &Memory, i: u64) -> (FullInst, u64) {
    decode_inst_inner(mem, i, Prefix::new())
}

fn decode_inst_inner(mem: &Memory, i: u64, mut prefix: Prefix) -> (FullInst, u64) {
    let b0 = mem.read_u8(i);
    let mut operand_size = prefix.operand_size;
    match b0 {
        0x40..=0x4F => {
            // 0x40..=0x4F REX prefix. Must be immediately followed by opcode byte.
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(prefix.dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_rex(b0));
            (inst, len + 1)
        }
        0x66 => {
            // 0x66 Operand size prefix.
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(prefix.dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_operand_size_prefix());
            (inst, len + 1)
        }
        0x67 => {
            // 0x67 Address size prefix.
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(prefix.dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_address_size_prefix());
            (inst, len + 1)
        }
        0x88 | 0x8A => {
            let modrm = modrm_decode(mem.read_u8(i + 1));
            let (reg, rex_b_matters0) = reg8_field_select(modrm.reg3, prefix.rex.map(|x| x.r));
            let (rm8, rex_b_matters1) = decode_rm8(&modrm, &prefix);
            if let Some(rex) = prefix.rex {
                let rex_b_matters = rex_b_matters0 || rex_b_matters1;
                if !rex.w && !rex.x && !rex.r && rex_b_matters {
                    // REX prefix doesn't need to be printed separately,
                    // since it is already the 'default' prefix for the inst.
                    prefix.dis_prefix.remove_rex_prefix();
                }
            };
            if modrm.mod2 != 0b11 {
                // mod2 == 0b11 means the r/m is a register. Otherwise
                // it is a computed address, 32- or 64- bits.
                prefix.dis_prefix.remove_last_address_size_prefix();
            }
            let inst = match b0 {
                0x88 => MovMR8(rm8, reg),
                0x8A => MovRM8(reg, rm8),
                _ => panic!("Missing match arm in decode_inst_inner."),
            };
            (inst.with_prefix(prefix.dis_prefix), 2)
        }
        0xB0..=0xB7 => {
            // B0+ rb ib; MOV r8, imm8
            // Move imm8 to r8.

            let imm8 = mem.read_u8(i + 1);
            let (reg, rex_b_matters) = reg8_field_select(b0, prefix.rex.map(|r| r.b));
            if let Some(rex) = prefix.rex {
                if !rex.w && !rex.x && !rex.r && rex_b_matters {
                    // REX prefix doesn't need to be printed separately,
                    // since it is already the 'default' prefix for the inst.
                    prefix.dis_prefix.remove_rex_prefix();
                }
            };
            (MovImm8(reg, imm8).with_prefix(prefix.dis_prefix), 2)
        }
        0xB8..=0xBF => {
            // The operand size prefix always changes the shape of the mov
            // instruction, so there is never a need to mention it.
            prefix.dis_prefix.remove_last_operand_size_prefix();
            if let Some(rex) = prefix.rex {
                let no_silly_business = !rex.x && !rex.r;
                let operand_size_influenced = rex.w;
                if operand_size_influenced {
                    operand_size = Data64
                }
                let reg_field_select_influenced = rex.b;
                if no_silly_business && (operand_size_influenced || reg_field_select_influenced) {
                    // REX prefix doesn't need to be printed separately,
                    // since it is already the 'default' prefix for the inst.
                    prefix.dis_prefix.remove_rex_prefix();
                };
            };
            match operand_size {
                Data16 => {
                    // B8+ rw iw; MOV r16, imm16
                    let d0 = mem.read_u8(i + 1) as u16;
                    let d1 = mem.read_u8(i + 2) as u16;
                    let imm16 = (d1 << 8) | d0;
                    let reg = reg16_field_select(b0, prefix.rex.map(|r| r.b).unwrap_or(false));
                    (MovImm16(reg, imm16).with_prefix(prefix.dis_prefix), 3)
                }
                Data32 => {
                    // B8+ rd id; MOV r32, imm32
                    let d0 = mem.read_u8(i + 1) as u32;
                    let d1 = mem.read_u8(i + 2) as u32;
                    let d2 = mem.read_u8(i + 3) as u32;
                    let d3 = mem.read_u8(i + 4) as u32;
                    let imm32 = (d3 << 24) | (d2 << 16) | (d1 << 8) | d0;
                    let reg = reg32_field_select(b0, prefix.rex.map(|r| r.b).unwrap_or(false));
                    (MovImm32(reg, imm32).with_prefix(prefix.dis_prefix), 5)
                }
                Data64 => {
                    // REX.W + B8+ rd io
                    let d0 = mem.read_u8(i + 1) as u64;
                    let d1 = mem.read_u8(i + 2) as u64;
                    let d2 = mem.read_u8(i + 3) as u64;
                    let d3 = mem.read_u8(i + 4) as u64;
                    let d4 = mem.read_u8(i + 5) as u64;
                    let d5 = mem.read_u8(i + 6) as u64;
                    let d6 = mem.read_u8(i + 7) as u64;
                    let d7 = mem.read_u8(i + 8) as u64;
                    let imm64 = (d7 << 56)
                        | (d6 << 48)
                        | (d5 << 40)
                        | (d4 << 32)
                        | (d3 << 24)
                        | (d2 << 16)
                        | (d1 << 8)
                        | d0;
                    let reg = reg64_field_select(b0, prefix.rex.map(|r| r.b).unwrap_or(false));
                    (MovImm64(reg, imm64).with_prefix(prefix.dis_prefix), 9)
                }
            }
        }
        0xC6 => {
            let modrm = modrm_decode(mem.read_u8(i + 1));
            match modrm.reg3 {
                0 => {
                    // C6 /0 ib; MOV r/m8, imm16
                    // REX + C6 /0 ib; MOV r/m8, imm16
                    let modrm = modrm_decode(mem.read_u8(i + 1));
                    let imm8 = mem.read_u8(i + 2);
                    let (rm8, rex_b_matters) = decode_rm8(&modrm, &prefix);
                    if let Some(rex) = prefix.rex {
                        let no_silly_business = !rex.x && !rex.r;
                        if no_silly_business && !rex.w && rex_b_matters {
                            // REX prefix doesn't need to be printed separately,
                            // since it is already the 'default' prefix for the inst.
                            prefix.dis_prefix.remove_rex_prefix();
                        };
                    };
                    (MovMI8(rm8, imm8).with_prefix(prefix.dis_prefix), 3)
                }
                _ => unimplemented!("Opcode {:02X} /{} not yet implemented.", b0, modrm.reg3),
            }
        }
        0xC7 => {
            let modrm = modrm_decode(mem.read_u8(i + 1));
            match modrm.reg3 {
                0 => {
                    // C7 /0
                    prefix.dis_prefix.remove_last_operand_size_prefix();
                    if let Some(rex) = prefix.rex {
                        let no_silly_business = !rex.x && !rex.r;
                        let operand_size_influenced = rex.w;
                        if operand_size_influenced {
                            operand_size = Data64
                        }
                        // no reg field to influence
                        if no_silly_business && operand_size_influenced {
                            // REX prefix doesn't need to be printed separately,
                            // since it is already the 'default' prefix for the inst.
                            prefix.dis_prefix.remove_rex_prefix();
                        };
                    };
                    match operand_size {
                        Data16 => {
                            // C7 /0 iw; MOV r/m16, imm16
                            let modrm = modrm_decode(mem.read_u8(i + 1));
                            let d0 = mem.read_u8(i + 2) as u16;
                            let d1 = mem.read_u8(i + 3) as u16;
                            let imm16 = (d1 << 8) | d0;
                            let rm16 = decode_rm16(&modrm, &prefix);
                            (MovMI16(rm16, imm16).with_prefix(prefix.dis_prefix), 4)
                        }
                        Data32 => {
                            // C7 /0 id; MOV r/m32, imm32
                            let modrm = modrm_decode(mem.read_u8(i + 1));
                            let d0 = mem.read_u8(i + 2) as u32;
                            let d1 = mem.read_u8(i + 3) as u32;
                            let d2 = mem.read_u8(i + 4) as u32;
                            let d3 = mem.read_u8(i + 5) as u32;
                            let imm32 = (d3 << 24) | (d2 << 16) | (d1 << 8) | d0;
                            let rm32 = decode_rm32(&modrm, &prefix);
                            (MovMI32(rm32, imm32).with_prefix(prefix.dis_prefix), 6)
                        }
                        Data64 => {
                            // REX.W + C7 /0 id; MOV r/m64, imm32
                            let modrm = modrm_decode(mem.read_u8(i + 1));
                            let d0 = mem.read_u8(i + 2) as u32;
                            let d1 = mem.read_u8(i + 3) as u32;
                            let d2 = mem.read_u8(i + 4) as u32;
                            let d3 = mem.read_u8(i + 5) as u32;
                            let imm32 = (d3 << 24) | (d2 << 16) | (d1 << 8) | d0;
                            let rm64 = decode_rm64(&modrm, &prefix);
                            (MovMI32s64(rm64, imm32).with_prefix(prefix.dis_prefix), 6)
                        }
                    }
                }
                _ => unimplemented!("Opcode {:02X} /{} not yet implemented.", b0, modrm.reg3),
            }
        }
        0xF4 => {
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(prefix.dis_prefix), 0);
            }
            (Hlt.with_prefix(prefix.dis_prefix), 1)
        }
        _ => unimplemented!("Opcode {:02X} not yet implemented.", b0),
    }
}

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

// r/m8. The bool is true when REX.b matters.
fn decode_rm8(modrm: &ModRM, prefix: &Prefix) -> (RM8, bool) {
    match modrm.mod2 {
        0b00 => (RM8::Addr(decode_rm_00(modrm, prefix)), false),
        0b01 | 0b10 => todo!("ModR/M addressing mode not yet implemented."),
        0b11 => {
            let rexb_opt = prefix.rex.map(|r| r.b);
            let (reg, rex_b_matters) = reg8_field_select(modrm.rm3, rexb_opt);
            (RM8::Reg(reg), rex_b_matters)
        }
        _ => panic!("Missing match arm in modrm_decode_addr8."),
    }
}

// r/m16
fn decode_rm16(modrm: &ModRM, prefix: &Prefix) -> RM16 {
    match modrm.mod2 {
        0b00 => RM16::Addr(decode_rm_00(modrm, prefix)),
        0b01 | 0b10 => todo!("ModR/M addressing mode not yet implemented."),
        0b11 => {
            let rexb = prefix.rex.map(|r| r.b).unwrap_or(false);
            RM16::Reg(reg16_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr16."),
    }
}

// r/m32
fn decode_rm32(modrm: &ModRM, prefix: &Prefix) -> RM32 {
    match modrm.mod2 {
        0b00 => RM32::Addr(decode_rm_00(modrm, prefix)),
        0b01 | 0b10 => todo!("ModR/M addressing mode not yet implemented."),
        0b11 => {
            let rexb = prefix.rex.map(|r| r.b).unwrap_or(false);
            RM32::Reg(reg32_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr32."),
    }
}

// r/m64
fn decode_rm64(modrm: &ModRM, prefix: &Prefix) -> RM64 {
    match modrm.mod2 {
        0b00 => RM64::Addr(decode_rm_00(modrm, prefix)),
        0b01 | 0b10 => todo!("ModR/M addressing mode not yet implemented."),
        0b11 => {
            let rexb = prefix.rex.map(|r| r.b).unwrap_or(false);
            RM64::Reg(reg64_field_select(modrm.rm3, rexb))
        }
        _ => panic!("Missing match arm in modrm_decode_addr64."),
    }
}

/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
/// The table only provides (address_size, rexb) = (Addr32, false). Rest are guesses.
fn decode_rm_00(modrm: &ModRM, prefix: &Prefix) -> EffAddr {
    let rexb = prefix.rex.map(|r| r.b).unwrap_or(false);
    let address_size = prefix.address_size;
    match (address_size, rexb, modrm.rm3) {
        (Addr32, false, 0b000) => EffAddr::Reg32(GPR32::eax),
        (Addr32, false, 0b001) => EffAddr::Reg32(GPR32::ecx),
        (Addr32, false, 0b010) => EffAddr::Reg32(GPR32::edx),
        (Addr32, false, 0b011) => EffAddr::Reg32(GPR32::ebx),
        (Addr32, false, 0b100) => todo!("[--][--] not yet implemented"),
        (Addr32, false, 0b101) => todo!("disp32 not yet implemented"),
        (Addr32, false, 0b110) => EffAddr::Reg32(GPR32::esi),
        (Addr32, false, 0b111) => EffAddr::Reg32(GPR32::edi),
        (Addr64, false, 0b000) => EffAddr::Reg64(GPR64::rax),
        (Addr64, false, 0b001) => EffAddr::Reg64(GPR64::rcx),
        (Addr64, false, 0b010) => EffAddr::Reg64(GPR64::rdx),
        (Addr64, false, 0b011) => EffAddr::Reg64(GPR64::rbx),
        (Addr64, false, 0b100) => todo!("[--][--] not yet implemented"),
        (Addr64, false, 0b101) => todo!("disp64 not yet implemented"),
        (Addr64, false, 0b110) => EffAddr::Reg64(GPR64::rsi),
        (Addr64, false, 0b111) => EffAddr::Reg64(GPR64::rdi),
        // TODO: All of these (true, ) fields are guesses.
        (Addr32, true, 0b000) => EffAddr::Reg32(GPR32::r8d),
        (Addr32, true, 0b001) => EffAddr::Reg32(GPR32::r9d),
        (Addr32, true, 0b010) => EffAddr::Reg32(GPR32::r10d),
        (Addr32, true, 0b011) => EffAddr::Reg32(GPR32::r11d),
        (Addr32, true, 0b100) => EffAddr::Reg32(GPR32::r12d),
        (Addr32, true, 0b101) => EffAddr::Reg32(GPR32::r13d),
        (Addr32, true, 0b110) => EffAddr::Reg32(GPR32::r14d),
        (Addr32, true, 0b111) => EffAddr::Reg32(GPR32::r15d),
        (Addr64, true, 0b000) => EffAddr::Reg64(GPR64::r8),
        (Addr64, true, 0b001) => EffAddr::Reg64(GPR64::r9),
        (Addr64, true, 0b010) => EffAddr::Reg64(GPR64::r10),
        (Addr64, true, 0b011) => EffAddr::Reg64(GPR64::r11),
        (Addr64, true, 0b100) => EffAddr::Reg64(GPR64::r12),
        (Addr64, true, 0b101) => EffAddr::Reg64(GPR64::r13),
        (Addr64, true, 0b110) => EffAddr::Reg64(GPR64::r14),
        (Addr64, true, 0b111) => EffAddr::Reg64(GPR64::r15),
        _ => panic!("Missing match arm in modrm_decode_addr32."),
    }
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
