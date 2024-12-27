use crate::{
    inst::{Addr, FullInst, Inst::*},
    inst_prefixes::{
        AddressSizeAttribute::{self, *},
        DisassemblyPrefix, OperandSizeAttribute, Prefix, Rex,
    },
    memory::Memory,
    registers::{GPR16, GPR32, GPR64, GPR8},
};

/// Returns instruction together with the number of bytes read.
pub fn decode_inst(mem: &Memory, i: u64) -> (FullInst, u64) {
    decode_inst_inner(mem, i, Prefix::new())
}

fn decode_inst_inner(mem: &Memory, i: u64, prefix: Prefix) -> (FullInst, u64) {
    let b0 = mem.read_byte(i);
    let mut operand_size = OperandSizeAttribute::from_prefix(prefix);
    let address_size = AddressSizeAttribute::from_prefix(prefix);
    let mut dis_prefix = DisassemblyPrefix {
        operand_size: match prefix.operand_size_prefix {
            true => Some(operand_size),
            false => None,
        },
        address_size: match prefix.address_size_prefix {
            true => Some(address_size),
            false => None,
        },
        rex: prefix.rex,
    };
    match b0 {
        0x40..=0x4F => {
            // 0x40..=0x4F REX prefix. Must be immediately followed by opcode byte.
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_rex(b0));
            (inst, len + 1)
        }
        0x66 => {
            // 0x66 Operand size prefix.
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_operand_size_prefix());
            (inst, len + 1)
        }
        0x67 => {
            // 0x67 Address size prefix.
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_address_size_prefix());
            (inst, len + 1)
        }
        0x88 => {
            let modrm = modrm_decode(mem.read_byte(i + 1));
            let (reg, rex_b_matters) = reg8_field_select(modrm.reg3, prefix.rex.map(|x| x.r));
            if let Some(rex) = dis_prefix.rex {
                if !rex.w && !rex.x && !rex.r && rex_b_matters {
                    // REX prefix doesn't need to be printed separately,
                    // since it is already the 'default' prefix for the inst.
                    dis_prefix.rex = None
                }
            };
            let addr = match address_size {
                Addr64 => modrm_addr64(modrm, prefix.rex),
                Addr32 => modrm_addr32(modrm, prefix.rex),
            };
            dis_prefix.address_size = None;
            (MovMR8(addr, reg).with_prefix(dis_prefix), 2)
        }
        0xB0..=0xB7 => {
            // B0+ rb ib; MOV r8, imm8
            // Move imm8 to r8.

            let imm8 = mem.read_byte(i + 1);
            let (reg, rex_b_matters) = reg8_field_select(b0, prefix.rex.map(|r| r.b));
            if let Some(rex) = dis_prefix.rex {
                if !rex.w && !rex.x && !rex.r && rex_b_matters {
                    // REX prefix doesn't need to be printed separately,
                    // since it is already the 'default' prefix for the inst.
                    dis_prefix.rex = None
                }
            };
            (MovImm8(reg, imm8).with_prefix(dis_prefix), 2)
        }
        0xB8..=0xBF => {
            // The operand size prefix always changes the shape of the mov
            // instruction, so there is never a need to mention it.
            dis_prefix.operand_size = None;
            if let Some(rex) = dis_prefix.rex {
                let no_silly_business = !rex.x && !rex.r;
                let operand_size_influenced = rex.w;
                if operand_size_influenced {
                    operand_size = OperandSizeAttribute::Data64
                }
                let reg_field_select_influenced = rex.b;
                if no_silly_business && (operand_size_influenced || reg_field_select_influenced) {
                    // REX prefix doesn't need to be printed separately,
                    // since it is already the 'default' prefix for the inst.
                    dis_prefix.rex = None
                };
            };
            match operand_size {
                OperandSizeAttribute::Data16 => {
                    // B8+ rw iw; MOV r16, imm16
                    let d0 = mem.read_byte(i + 1) as u16;
                    let d1 = mem.read_byte(i + 2) as u16;
                    let imm16 = (d1 << 8) | d0;
                    let reg = reg16_field_select(b0, prefix.rex);
                    (MovImm16(reg, imm16).with_prefix(dis_prefix), 3)
                }
                OperandSizeAttribute::Data32 => {
                    // B8+ rd id; MOV r32, imm32
                    let d0 = mem.read_byte(i + 1) as u32;
                    let d1 = mem.read_byte(i + 2) as u32;
                    let d2 = mem.read_byte(i + 3) as u32;
                    let d3 = mem.read_byte(i + 4) as u32;
                    let imm32 = (d3 << 24) | (d2 << 16) | (d1 << 8) | d0;
                    let reg = reg32_field_select(b0, prefix.rex);
                    (MovImm32(reg, imm32).with_prefix(dis_prefix), 5)
                }
                OperandSizeAttribute::Data64 => {
                    // REX.W + B8+ rd io
                    let d0 = mem.read_byte(i + 1) as u64;
                    let d1 = mem.read_byte(i + 2) as u64;
                    let d2 = mem.read_byte(i + 3) as u64;
                    let d3 = mem.read_byte(i + 4) as u64;
                    let d4 = mem.read_byte(i + 5) as u64;
                    let d5 = mem.read_byte(i + 6) as u64;
                    let d6 = mem.read_byte(i + 7) as u64;
                    let d7 = mem.read_byte(i + 8) as u64;
                    let imm64 = (d7 << 56)
                        | (d6 << 48)
                        | (d5 << 40)
                        | (d4 << 32)
                        | (d3 << 24)
                        | (d2 << 16)
                        | (d1 << 8)
                        | d0;
                    let reg = reg64_field_select(b0, prefix.rex);
                    (MovImm64(reg, imm64).with_prefix(dis_prefix), 9)
                }
            }
        }
        0xF4 => {
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            (Hlt.with_prefix(dis_prefix), 1)
        }
        _ => unimplemented!("Opcode 0x{:02X} not yet implemented.", b0),
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

/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
/// I don't have a source for how the rex bit B comes in either.
fn modrm_addr32(modrm: ModRM, rex: Option<Rex>) -> Addr {
    let b = rex.map(|x| x.b).unwrap_or(false);
    match modrm.mod2 {
        0b00 => match (b, modrm.rm3) {
            (false, 0b000) => Addr::Reg32(GPR32::eax),
            (false, 0b001) => Addr::Reg32(GPR32::ecx),
            (false, 0b010) => Addr::Reg32(GPR32::edx),
            (false, 0b011) => Addr::Reg32(GPR32::ebx),
            (false, 0b100) => todo!("[--][--] not yet implemented"),
            (false, 0b101) => todo!("disp32 not yet implemented"),
            (false, 0b110) => Addr::Reg32(GPR32::esi),
            (false, 0b111) => Addr::Reg32(GPR32::edi),
            // TODO: All of these (true, ) fields are guesses.
            (true, 0b000) => Addr::Reg32(GPR32::r8d),
            (true, 0b001) => Addr::Reg32(GPR32::r9d),
            (true, 0b010) => Addr::Reg32(GPR32::r10d),
            (true, 0b011) => Addr::Reg32(GPR32::r11d),
            (true, 0b100) => Addr::Reg32(GPR32::r12d),
            (true, 0b101) => Addr::Reg32(GPR32::r13d),
            (true, 0b110) => Addr::Reg32(GPR32::r14d),
            (true, 0b111) => Addr::Reg32(GPR32::r15d),
            _ => panic!("Missing match arm in modrm_decode_addr32."),
        },
        #[allow(clippy::manual_range_patterns)]
        0b01 | 0b10 | 0b11 => todo!("ModR/M addressing mode not yet implemented."),
        _ => panic!("Missing match arm in modrm_decode_addr32."),
    }
}

/// I don't have a source for this. Just guessing based on the addr32 table.
/// Vol 2A: Table 2-2. "32-Bit Addressing Forms with the ModR/M Byte"
/// I don't have a source for how the rex bit B comes in either.
fn modrm_addr64(modrm: ModRM, rex: Option<Rex>) -> Addr {
    let b = rex.map(|x| x.b).unwrap_or(false);
    match modrm.mod2 {
        0b00 => match (b, modrm.rm3) {
            (false, 0b000) => Addr::Reg64(GPR64::rax),
            (false, 0b001) => Addr::Reg64(GPR64::rcx),
            (false, 0b010) => Addr::Reg64(GPR64::rdx),
            (false, 0b011) => Addr::Reg64(GPR64::rbx),
            (false, 0b100) => todo!("[--][--] not yet implemented"),
            (false, 0b101) => todo!("disp32 not yet implemented"),
            (false, 0b110) => Addr::Reg64(GPR64::rsi),
            (false, 0b111) => Addr::Reg64(GPR64::rdi),
            // TODO: All of these (true, ) fields are guesses.
            (true, 0b000) => Addr::Reg64(GPR64::r8),
            (true, 0b001) => Addr::Reg64(GPR64::r9),
            (true, 0b010) => Addr::Reg64(GPR64::r10),
            (true, 0b011) => Addr::Reg64(GPR64::r11),
            (true, 0b100) => Addr::Reg64(GPR64::r12),
            (true, 0b101) => Addr::Reg64(GPR64::r13),
            (true, 0b110) => Addr::Reg64(GPR64::r14),
            (true, 0b111) => Addr::Reg64(GPR64::r15),
            _ => panic!("Missing match arm in modrm_decode_addr64."),
        },
        #[allow(clippy::manual_range_patterns)]
        0b01 | 0b10 | 0b11 => todo!("ModR/M addressing mode not yet implemented."),
        _ => panic!("Missing match arm in modrm_decode_addr64."),
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
fn reg16_field_select(op: u8, rex: Option<Rex>) -> GPR16 {
    // No REX prefix is the same as a REX prefix with B bit clear.
    let rexb = rex.map(|x| x.b).unwrap_or(false);
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
fn reg32_field_select(op: u8, rex: Option<Rex>) -> GPR32 {
    // No REX prefix is the same as a REX prefix with B bit clear.
    let rexb = rex.map(|x| x.b).unwrap_or(false);
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
fn reg64_field_select(op: u8, rex: Option<Rex>) -> GPR64 {
    // No REX prefix is the same as a REX prefix with B bit clear.
    let rexb = rex.map(|x| x.b).unwrap_or(false);
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
