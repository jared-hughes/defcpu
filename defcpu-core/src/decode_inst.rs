use crate::{
    memory::Memory,
    registers::{GPR32, GPR8},
};

use Inst::*;
pub enum Inst {
    MovImm8(GPR8, u8),
    MovImm32(GPR32, u32),
    Hlt,
}

/// Returns instruction together with the number of bytes read.
pub fn decode_inst(mem: &Memory, i: u64) -> (Inst, u64) {
    let b0 = mem.read_byte(i);
    match b0 {
        0xB0..=0xB7 => {
            let imm8 = mem.read_byte(i + 1);
            let reg = reg8_field_select(b0);
            (MovImm8(reg, imm8), 2)
        }
        0xB8..=0xBF => {
            // little-endian
            let d0 = mem.read_byte(i + 1) as u32;
            let d1 = mem.read_byte(i + 2) as u32;
            let d2 = mem.read_byte(i + 3) as u32;
            let d3 = mem.read_byte(i + 4) as u32;
            let imm32 = (d3 << 24) | (d2 << 16) | (d1 << 8) | d0;
            let reg = reg32_field_select(b0);
            (MovImm32(reg, imm32), 5)
        }
        0xF4 => (Hlt, 1),
        _ => panic!("Opcode 0x{:02X} not yet implemented.", b0),
    }
}

/// ref Table 3-1. of Vol. 2A.
/// Register Codes Associated With +rb, +rw, +rd, +ro.
/// This is for +rb with no REX prefix.
fn reg8_field_select(op: u8) -> GPR8 {
    match op & 0b111 {
        0b000 => GPR8::al,
        0b001 => GPR8::cl,
        0b010 => GPR8::dl,
        0b011 => GPR8::bl,
        0b100 => GPR8::ah,
        0b101 => GPR8::ch,
        0b110 => GPR8::dh,
        0b111 => GPR8::bh,
        _ => panic!("Mask failed in reg8_field_select."),
    }
}

/// ref Table 3-1. of Vol. 2A.
/// Register Codes Associated With +rb, +rw, +rd, +ro.
/// This is for +rd with no REX prefix.
fn reg32_field_select(op: u8) -> GPR32 {
    match op & 0b111 {
        0b000 => GPR32::eax,
        0b001 => GPR32::ecx,
        0b010 => GPR32::edx,
        0b011 => GPR32::ebx,
        0b100 => GPR32::esp,
        0b101 => GPR32::ebp,
        0b110 => GPR32::esi,
        0b111 => GPR32::edi,
        _ => panic!("Mask failed in reg32_field_select."),
    }
}
