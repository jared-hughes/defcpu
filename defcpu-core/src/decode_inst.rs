use std::fmt;

use crate::{
    memory::Memory,
    registers::{GPR16, GPR32, GPR64, GPR8},
};

use Inst::*;
pub enum Inst {
    /// A no-op stemming from REX not being followed by a valid expression.
    RexNoop,
    // REX + B0+ rb ib; MOV r81, imm8; Move imm8 to r8.
    MovImm8(GPR8, u8),
    // B8+ rw iw; MOV r16, imm16; Move imm16 to r16.
    MovImm16(GPR16, u16),
    // B8+ rd id; MOV r32, imm32; Move imm32 to r32.
    MovImm32(GPR32, u32),
    // REX.W + B8+ rd io; MOV r64, imm64; Move imm64 to r64.
    MovImm64(GPR64, u64),
    // F4; HLT
    Hlt,
}
impl Inst {
    fn with_prefix(self, prefix: DisassemblyPrefix) -> FullInst {
        FullInst {
            prefix,
            inner: self,
        }
    }
}

impl Inst {
    fn fmt_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RexNoop => Ok(()),
            MovImm8(gpr8, imm8) => write!(f, "$0x{:x}, %{}", imm8, gpr8),
            MovImm16(gpr16, imm16) => write!(f, "$0x{:x}, %{}", imm16, gpr16),
            MovImm32(gpr32, imm32) => write!(f, "$0x{:x}, %{}", imm32, gpr32),
            MovImm64(gpr64, imm64) => write!(f, "$0x{:x}, %{}", imm64, gpr64),
            Hlt => write!(f, ""),
        }
    }
    fn mnemonic(&self) -> &str {
        match self {
            RexNoop => "",
            MovImm8(_, _) => "mov",
            MovImm16(_, _) => "mov",
            MovImm32(_, _) => "mov",
            // movabs just does the same, idk why gdb dumps as movabs.
            MovImm64(_, _) => "movabs",
            Hlt => "hlt",
        }
    }
}

pub struct FullInst {
    /// The prefix is only encoded for disassembly.
    prefix: DisassemblyPrefix,
    /// The inner instruction is needed for disassembly and execution.
    pub inner: Inst,
}
impl fmt::Display for FullInst {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mnem = self.inner.mnemonic();
        if !mnem.is_empty() {
            let prefix = format!("{}", self.prefix);
            let prefix_plus_mnemonic = if !prefix.is_empty() {
                format!("{} {}", prefix, mnem)
            } else {
                mnem.to_string()
            };
            // The prefix and mnemonic together get a budget of 6 spaces.
            write!(f, "{:6} ", prefix_plus_mnemonic)?;
            self.inner.fmt_operands(f)
        } else {
            // Just prefixes, e.g. due to REX no-op.
            write!(f, "{}", self.prefix)
        }
    }
}

struct DisassemblyPrefix {
    pub address_size: Option<AddressSizeAttribute>,
    pub operand_size: Option<OperandSizeAttribute>,
    pub rex: Option<Rex>,
}
impl fmt::Display for DisassemblyPrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut some = false;
        if let Some(s) = &self.address_size {
            s.fmt(f)?;
            some = true;
        }
        if let Some(s) = &self.operand_size {
            if some {
                write!(f, " ")?;
            }
            s.fmt(f)?;
            some = true;
        }
        if let Some(rex) = &self.rex {
            if some {
                write!(f, " ")?;
            }
            rex.fmt(f)?;
        }
        Ok(())
    }
}

/// There are three relevant encodings of instructions
///   A: Just has a ModR/M byte.
///      - Vol 2A: Figure 2-4. "Memory Addressing Without an SIB Byte; REX.X Not Used"
///      - Vol 2A: Figure 2-5. "Register-Register Addressing (No Memory Operand); REX.X Not Used"
///   B: Has a ModR/M byte and a SIB byte.
///      - Vol 2A: Figure 2-6. "Memory Addressing With a SIB Byte"
///   C: Has no ModR/M byte, but the lower 3 bits of the opcode are a reg field.
///      - Vol 2A: Figure 2-7. "Register Operand Coded in Opcode Byte; REX.X & REX.R Not Used"
#[derive(Clone, Copy)]
struct Prefix {
    /// true if the Operand-Size Prefix 0x66 is present
    operand_size_prefix: bool,
    /// true if the Address-Size Prefix 0x67 is present.
    address_size_prefix: bool,
    /// If Some(Rex), a REX prefix is present.
    rex: Option<Rex>,
}

#[derive(Clone, Copy)]
struct Rex {
    /// If false, operand size is determined by CS.D
    /// If true forces a 64 Bit Operand Size
    w: bool,
    /// An extra bit extended to the MSB of the reg field of the ModR/M byte,
    /// when present (encodings A and B), ignored otherwise.
    #[allow(unused)]
    r: bool,
    /// An extra bit extended to the MSB of the index field of the SIB byte,
    /// when present (encoding B), ignored otherwise.
    #[allow(unused)]
    x: bool,
    /// An extra bit extended to the MSB of the:
    ///   - Encoding A: R/M field of ModR/M byte.
    ///   - Encoding B: base field of SIB byte.
    ///   - Encoding C: reg field of the opcode.
    b: bool,
}
impl Rex {}
impl fmt::Display for Rex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rex")?;
        if self.w || self.r || self.x || self.b {
            write!(f, ".")?;
        };
        if self.w {
            write!(f, "W")?;
        }
        if self.r {
            write!(f, "R")?;
        }
        if self.x {
            write!(f, "X")?;
        }
        if self.b {
            write!(f, "B")?;
        }
        Ok(())
    }
}

// TODO: test what happens with chains like 0x66 + 0x67 + REX + 0x67 + REX
// Reference says only a single REX can influence, but I'm not sure about
// the other prefixes.
impl Prefix {
    pub fn new() -> Prefix {
        Prefix {
            operand_size_prefix: false,
            address_size_prefix: false,
            rex: None,
        }
    }

    pub fn with_operand_size_prefix(self) -> Prefix {
        let mut p = self;
        p.operand_size_prefix = true;
        p
    }

    pub fn with_address_size_prefix(self) -> Prefix {
        let mut p = self;
        p.address_size_prefix = true;
        p
    }

    pub fn with_rex(self, rex: u8) -> Prefix {
        let mut p = self;
        p.rex = Some(Rex {
            w: rex & 0b1000 != 0,
            r: rex & 0b0100 != 0,
            x: rex & 0b0010 != 0,
            b: rex & 0b0001 != 0,
        });
        p
    }
}

/// Vol 1. 3.6.1 "Operand Size and Address Size in 64-Bit Mode"
/// Address size is 64 by default, but 32 if the Address-Size Prefix 0x67 is present.
#[derive(Clone, Copy)]
enum AddressSizeAttribute {
    Addr64,
    Addr32,
}
impl AddressSizeAttribute {
    fn from_prefix(p: Prefix) -> AddressSizeAttribute {
        if p.address_size_prefix {
            AddressSizeAttribute::Addr32
        } else {
            AddressSizeAttribute::Addr64
        }
    }
}
impl fmt::Display for AddressSizeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressSizeAttribute::Addr64 => write!(f, "addr64"),
            AddressSizeAttribute::Addr32 => write!(f, "addr32"),
        }
    }
}

/// Vol 1. 3.6.1 "Operand Size and Address Size in 64-Bit Mode"
/// Operand size is 32 by default, but 64 if the REX.W prefix is present,
/// else 16 if the Operand-Size Prefix 0x66 is present. The REX.W takes precedence.
#[derive(Clone, Copy)]
enum OperandSizeAttribute {
    Data64,
    Data32,
    Data16,
}
impl OperandSizeAttribute {
    fn from_prefix(p: Prefix) -> OperandSizeAttribute {
        if p.rex.map(|x| x.w).unwrap_or(false) {
            OperandSizeAttribute::Data64
        } else if p.operand_size_prefix {
            OperandSizeAttribute::Data16
        } else {
            OperandSizeAttribute::Data32
        }
    }
}
impl fmt::Display for OperandSizeAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OperandSizeAttribute::Data64 => write!(f, "data64"),
            OperandSizeAttribute::Data32 => write!(f, "data32"),
            OperandSizeAttribute::Data16 => write!(f, "data16"),
        }
    }
}

/// Returns instruction together with the number of bytes read.
pub fn decode_inst(mem: &Memory, i: u64) -> (FullInst, u64) {
    decode_inst_inner(mem, i, Prefix::new())
}

fn decode_inst_inner(mem: &Memory, i: u64, prefix: Prefix) -> (FullInst, u64) {
    let b0 = mem.read_byte(i);
    let operand_size = OperandSizeAttribute::from_prefix(prefix);
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
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_rex(b0));
            (inst, len + 1)
        }
        0x66 => {
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_operand_size_prefix());
            (inst, len + 1)
        }
        0x67 => {
            if prefix.rex.is_some() {
                return (RexNoop.with_prefix(dis_prefix), 0);
            }
            let (inst, len) = decode_inst_inner(mem, i + 1, prefix.with_address_size_prefix());
            (inst, len + 1)
        }
        0xB0..=0xB7 => {
            // B0+ rb ib; MOV r8, imm8
            // Move imm8 to r8.

            let imm8 = mem.read_byte(i + 1);
            let (reg, rex_b_matters) = reg8_field_select(b0, prefix.rex);
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
            // Each is the only `mov` of its size, so no need to mention it in disassembly.
            dis_prefix.operand_size = None;
            if let Some(rex) = dis_prefix.rex {
                let no_silly_business = !rex.x && !rex.r;
                let operand_size_influenced = rex.w;
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
        _ => panic!("Opcode 0x{:02X} not yet implemented.", b0),
    }
}

/// Vol 2A: Table 3-1 "Register Codes Associated With +rb, +rw, +rd, +ro.""
/// This is for +rb (byte register).
/// Return a pair of the register and a boolean, where
/// the boolean is true iff the REX byte is the "default"
fn reg8_field_select(op: u8, rex: Option<Rex>) -> (GPR8, bool) {
    match (rex.map(|x| x.b), op & 0b111) {
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

/// Table 3-1. of Vol. 2A. "Register Codes Associated With +rb, +rw, +rd, +ro."
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

/// Table 3-1. of Vol. 2A. "Register Codes Associated With +rb, +rw, +rd, +ro."
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
