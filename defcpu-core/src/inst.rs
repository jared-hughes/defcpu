use std::fmt::{self, LowerHex};

use crate::{
    inst_prefixes::DisassemblyPrefix,
    registers::{Registers, GPR16, GPR32, GPR64, GPR8},
};

use Inst::*;
/// Instructions. Tuple args are in Intel order.
pub enum Inst {
    /// A no-op stemming from REX not being followed by a valid expression.
    RexNoop,
    // 88 /r; MOV r/m8, r8; Move r8 to r/m8.
    MovMR8(RM8, GPR8),
    // 8A /r; MOV r8, r/m8; Move r/m8 to r8.
    MovRM8(GPR8, RM8),
    // B0+ rb ib; MOV r8, imm8; Move imm8 to r8.
    MovImm8(GPR8, u8),
    // B8+ rw iw; MOV r16, imm16; Move imm16 to r16.
    MovImm16(GPR16, u16),
    // B8+ rd id; MOV r32, imm32; Move imm32 to r32.
    MovImm32(GPR32, u32),
    // REX.W + B8+ rd io; MOV r64, imm64; Move imm64 to r64.
    MovImm64(GPR64, u64),
    // C6 /0 ib; MOV r/m8, imm8; Move imm8 to r/m8.
    MovMI8(RM8, u8),
    // C7 /0 iw; MOV r/m16, imm16; Move imm16 to r/m16.
    MovMI16(RM16, u16),
    // C7 /0 id; MOV r/m32, imm32; Move imm32 to r/m32.
    MovMI32(RM32, u32),
    // REX.W + C7 /0 id; MOV r/m64, imm32; Move imm32 sign extended to 64-bits to r/m64.
    MovMI32s64(RM64, u32),
    // F4; HLT
    Hlt,
}

impl Inst {
    pub fn with_prefix(self, prefix: DisassemblyPrefix) -> FullInst {
        FullInst {
            prefix,
            inner: self,
        }
    }
    fn fmt_operands(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RexNoop => Ok(()),
            MovMR8(addr, reg) => write!(f, "{}, {}", reg, addr),
            MovRM8(reg, addr) => write!(f, "{}, {}", addr, reg),
            MovImm8(gpr8, imm8) => write!(f, "${:#x}, {}", imm8, gpr8),
            MovImm16(gpr16, imm16) => write!(f, "${:#x}, {}", imm16, gpr16),
            MovImm32(gpr32, imm32) => write!(f, "${:#x}, {}", imm32, gpr32),
            MovImm64(gpr64, imm64) => write!(f, "${:#x}, {}", imm64, gpr64),
            MovMI8(addr, imm8) => write!(f, "${:#x}, {}", imm8, addr),
            MovMI16(addr, imm16) => write!(f, "${:#x}, {}", imm16, addr),
            MovMI32(addr, imm32) => write!(f, "${:#x}, {}", imm32, addr),
            MovMI32s64(addr, imm32) => write!(f, "${:#x}, {}", imm32, addr),
            Hlt => write!(f, ""),
        }
    }
    fn mnemonic(&self) -> &str {
        match self {
            RexNoop => "",
            MovMR8(_, _) => "mov",
            MovRM8(_, _) => "mov",
            MovImm8(_, _) => "mov",
            MovImm16(_, _) => "mov",
            MovImm32(_, _) => "mov",
            MovMI8(_, _) => "mov",
            MovMI16(_, _) => "mov",
            MovMI32(_, _) => "mov",
            MovMI32s64(_, _) => "mov",
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Scale {
    Scale1,
    Scale2,
    Scale4,
    Scale8,
}
impl From<Scale> for u8 {
    fn from(val: Scale) -> Self {
        match val {
            Scale::Scale1 => 1,
            Scale::Scale2 => 2,
            Scale::Scale4 => 4,
            Scale::Scale8 => 8,
        }
    }
}

// https://stackoverflow.com/a/63607986
struct ReallySigned(i32);
impl LowerHex for ReallySigned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = if f.alternate() { "0x" } else { "" };
        let bare_hex = format!("{:x}", self.0.abs());
        f.pad_integral(self.0 >= 0, prefix, &bare_hex)
    }
}

// TODO p 527
fn format_addr<Base: fmt::Display, Reg: fmt::Display>(
    f: &mut fmt::Formatter,
    disp: Option<i32>,
    base: Option<Base>,
    index: Option<Reg>,
    scale: Scale,
) -> fmt::Result {
    if let Some(disp) = disp {
        if disp != 0 {
            write!(f, "{:#x}", ReallySigned(disp))?;
        }
    }
    write!(f, "(")?;
    // disp(base, index, scale)
    if let Some(base) = base {
        base.fmt(f)?;
    }
    if let Some(index) = index {
        write!(f, ", {}", index)?;
        let scale: u8 = scale.into();
        write!(f, ", {}", scale)?;
    }
    write!(f, ")")
}

#[derive(Clone, Copy)]
pub enum Base32 {
    GPR32(GPR32),
    Eip,
}
impl fmt::Display for Base32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base32::GPR32(gpr32) => gpr32.fmt(f),
            Base32::Eip => write!(f, "%eip"),
        }
    }
}

#[derive(Clone)]
pub struct SIDB32 {
    pub(crate) disp: Option<i32>,
    pub(crate) base: Option<Base32>,
    pub(crate) index: Option<GPR32>,
    pub(crate) scale: Scale,
}
impl fmt::Display for SIDB32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_addr(f, self.disp, self.base, self.index, self.scale)
    }
}

#[derive(Clone, Copy)]
pub enum Base64 {
    GPR64(GPR64),
    Rip,
}
impl fmt::Display for Base64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Base64::GPR64(gpr64) => gpr64.fmt(f),
            Base64::Rip => write!(f, "%rip"),
        }
    }
}

#[derive(Clone)]
pub struct SIDB64 {
    pub(crate) disp: Option<i32>,
    pub(crate) base: Option<Base64>,
    pub(crate) index: Option<GPR64>,
    pub(crate) scale: Scale,
}
impl fmt::Display for SIDB64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_addr(f, self.disp, self.base, self.index, self.scale)
    }
}

pub enum EffAddr {
    // E.g. [eax], which is (%eax) in ATT
    EffAddr32(SIDB32),
    // E.g. [rax], which is (%rax) in ATT
    EffAddr64(SIDB64),
}
impl fmt::Display for EffAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EffAddr32(sidb) => write!(f, "{}", sidb),
            Self::EffAddr64(sidb) => write!(f, "{}", sidb),
        }
    }
}
impl EffAddr {
    pub fn from_base_reg32(base: GPR32) -> EffAddr {
        EffAddr::EffAddr32(SIDB32 {
            disp: None,
            base: Some(Base32::GPR32(base)),
            index: None,
            scale: Scale::Scale1,
        })
    }

    pub fn from_base_reg64(base: GPR64) -> EffAddr {
        EffAddr::EffAddr64(SIDB64 {
            disp: None,
            base: Some(Base64::GPR64(base)),
            index: None,
            scale: Scale::Scale1,
        })
    }

    pub fn with_disp(&self, disp: Option<i32>) -> Self {
        match self {
            EffAddr::EffAddr32(sidb32) => {
                let mut sidb = sidb32.clone();
                sidb.disp = disp;
                EffAddr::EffAddr32(sidb)
            }
            EffAddr::EffAddr64(sidb64) => {
                let mut sidb = sidb64.clone();
                sidb.disp = disp;
                EffAddr::EffAddr64(sidb)
            }
        }
    }

    // Vol1 3.7.5.1 "Specifying an Offset in 64-Bit Mode" p88
    // seems to imply that displacement is only 8/16/32 bits,
    // base and index arealways 64 bits.
    // TODO: Somehow RIP (64-bit) + Displacement (32-bit) is also possible.
    pub fn compute(&self, regs: &Registers) -> u64 {
        match self {
            EffAddr::EffAddr32(sidb) => {
                let disp = sidb.disp.unwrap_or(0);
                let base = sidb
                    .base
                    .map(|b| match b {
                        Base32::GPR32(gpr32) => regs.get_reg32(gpr32),
                        Base32::Eip => regs.get_eip(),
                    })
                    .unwrap_or(0);
                let scaled_index = sidb
                    .index
                    .map(|i| regs.get_reg32(i) * (sidb.scale as u32))
                    .unwrap_or(0);
                // TODO: Not sure how to interpret the wrapping here. May be something closer to
                //  (base as u64) + (scaled_index as u64) + (disp as u64)
                base.wrapping_add(scaled_index).wrapping_add(disp as u32) as u64
            }
            EffAddr::EffAddr64(sidb) => {
                let disp = sidb.disp.unwrap_or(0);
                let base = sidb
                    .base
                    .map(|b| match b {
                        Base64::GPR64(gpr64) => regs.get_reg64(gpr64),
                        Base64::Rip => regs.get_rip(),
                    })
                    .unwrap_or(0);
                let scaled_index = sidb
                    .index
                    .map(|i| regs.get_reg64(i) * (sidb.scale as u64))
                    .unwrap_or(0);
                // The `as u64` sign-extends an i32 to u64.
                base.wrapping_add(scaled_index).wrapping_add(disp as u64)
            }
        }
    }
}

pub enum RM8 {
    Addr(EffAddr),
    Reg(GPR8),
}
impl fmt::Display for RM8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RM8::Addr(addr) => addr.fmt(f),
            RM8::Reg(gpr8) => gpr8.fmt(f),
        }
    }
}

pub enum RM16 {
    Addr(EffAddr),
    Reg(GPR16),
}
impl fmt::Display for RM16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RM16::Addr(addr) => addr.fmt(f),
            RM16::Reg(gpr16) => gpr16.fmt(f),
        }
    }
}

pub enum RM32 {
    Addr(EffAddr),
    Reg(GPR32),
}
impl fmt::Display for RM32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RM32::Addr(addr) => addr.fmt(f),
            RM32::Reg(gpr32) => gpr32.fmt(f),
        }
    }
}

pub enum RM64 {
    Addr(EffAddr),
    Reg(GPR64),
}
impl fmt::Display for RM64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RM64::Addr(addr) => addr.fmt(f),
            RM64::Reg(gpr64) => gpr64.fmt(f),
        }
    }
}
