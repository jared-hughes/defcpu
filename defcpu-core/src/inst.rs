use std::fmt::{self, LowerHex};

use crate::{
    inst_prefixes::DisassemblyPrefix,
    registers::{Registers, GPR16, GPR32, GPR64, GPR8},
};

use Inst::*;
/// Instructions. Tuple args are in Intel order.
/// For valid opcodes, the name of each variant is the mnemonic,
/// followed by the Op/En column then the operand size.
pub enum Inst {
    /// Haven't yet implemented this. May or may not be a valid opcode.
    NotImplemented(u8),
    /// Haven't yet implemented this. May or may not be a valid opcode.
    /// Has an opcode extension: (a,b) represents a with the 3-byte b extension.
    NotImplementedOpext(u8, u8),
    /// A no-op stemming from REX not being followed by a valid expression.
    RexNoop,
    /// 88 /r; MOV r/m8, r8; Move r8 to r/m8.
    MovMR8(RM8, GPR8),
    /// 89 /r; MOV r/m16, r16; Move r16 to r/m16.
    MovMR16(RM16, GPR16),
    /// 89 /r; MOV r/m32, r32; Move r32 to r/m32.
    MovMR32(RM32, GPR32),
    /// REX.W + 89 /r; MOV r/m64, r64; Move r64 to r/m64.
    MovMR64(RM64, GPR64),
    /// 8A /r; MOV r8, r/m8; Move r/m8 to r8.
    MovRM8(GPR8, RM8),
    /// 8B /r; MOV r16, r/m16; Move r/m16 to r16.
    MovRM16(GPR16, RM16),
    /// 8B /r; MOV r32, r/m32; Move r/m32 to r32.
    MovRM32(GPR32, RM32),
    /// REX.W + 8B /r; MOV r64, r/m64; Move r/m64 to r64.
    MovRM64(GPR64, RM64),
    /// B0+ rb ib; MOV r8, imm8; Move imm8 to r8.
    MovOI8(GPR8, u8),
    /// B8+ rw iw; MOV r16, imm16; Move imm16 to r16.
    MovOI16(GPR16, u16),
    /// B8+ rd id; MOV r32, imm32; Move imm32 to r32.
    MovOI32(GPR32, u32),
    /// REX.W + B8+ rd io; MOV r64, imm64; Move imm64 to r64.
    MovOI64(GPR64, u64),
    /// C6 /0 ib; MOV r/m8, imm8; Move imm8 to r/m8.
    MovMI8(RM8, u8),
    /// C7 /0 iw; MOV r/m16, imm16; Move imm16 to r/m16.
    MovMI16(RM16, u16),
    /// C7 /0 id; MOV r/m32, imm32; Move imm32 to r/m32.
    MovMI32(RM32, u32),
    /// REX.W + C7 /0 id; MOV r/m64, imm32; Move imm32 sign extended to 64-bits to r/m64.
    MovMI64(RM64, u64),
    /// F4; HLT
    Hlt,
}

impl Inst {
    pub fn with_prefix(self, prefix: DisassemblyPrefix) -> FullInst {
        FullInst {
            prefix,
            inner: self,
        }
    }
    fn operands(&self) -> String {
        match self {
            NotImplemented(_) | NotImplementedOpext(_, _) | RexNoop => "".to_owned(),
            MovMR8(rm, reg) => format!("{}, {}", reg, rm),
            MovMR16(rm, reg) => format!("{}, {}", reg, rm),
            MovMR32(rm, reg) => format!("{}, {}", reg, rm),
            MovMR64(rm, reg) => format!("{}, {}", reg, rm),
            MovRM8(reg, rm) => format!("{}, {}", rm, reg),
            MovRM16(reg, rm) => format!("{}, {}", rm, reg),
            MovRM32(reg, rm) => format!("{}, {}", rm, reg),
            MovRM64(reg, rm) => format!("{}, {}", rm, reg),
            MovOI8(gpr8, imm8) => format!("${:#x}, {}", imm8, gpr8),
            MovOI16(gpr16, imm16) => format!("${:#x}, {}", imm16, gpr16),
            MovOI32(gpr32, imm32) => format!("${:#x}, {}", imm32, gpr32),
            MovOI64(gpr64, imm64) => format!("${:#x}, {}", imm64, gpr64),
            MovMI8(rm, imm8) => format!("${:#x}, {}", imm8, rm),
            MovMI16(rm, imm16) => format!("${:#x}, {}", imm16, rm),
            MovMI32(rm, imm32) => format!("${:#x}, {}", imm32, rm),
            // TODO: no way this is correct
            MovMI64(rm, imm32) => format!("${:#x}, {}", imm32, rm),
            Hlt => String::new(),
        }
    }
    fn mnemonic(&self) -> &str {
        match self {
            NotImplemented(_) => "(bad)",
            NotImplementedOpext(_, _) => "(bad)",
            RexNoop => "",
            MovMR8(_, _) => "mov",
            MovMR16(_, _) => "mov",
            MovMR32(_, _) => "mov",
            MovMR64(_, _) => "mov",
            MovRM8(_, _) => "mov",
            MovRM16(_, _) => "mov",
            MovRM32(_, _) => "mov",
            MovRM64(_, _) => "mov",
            MovOI8(_, _) => "mov",
            MovOI16(_, _) => "mov",
            MovOI32(_, _) => "mov",
            MovMI8(rm8, _) => {
                if rm8.is_reg() {
                    "mov"
                } else {
                    "movb"
                }
            }
            MovMI16(rm16, _) => {
                if rm16.is_reg() {
                    "mov"
                } else {
                    "movw"
                }
            }
            MovMI32(rm32, _) => {
                if rm32.is_reg() {
                    "mov"
                } else {
                    "movl"
                }
            }
            MovMI64(rm64, _) => {
                if rm64.is_reg() {
                    "mov"
                } else {
                    "movq"
                }
            }
            // movabs just does the same, idk why gdb dumps as movabs.
            MovOI64(_, _) => "movabs",
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
            let operands = self.inner.operands();
            if operands.is_empty() {
                write!(f, "{}", prefix_plus_mnemonic)?;
            } else {
                // The prefix and mnemonic together get a budget of 6 spaces.
                write!(f, "{prefix_plus_mnemonic:6} {operands}")?;
            }
            Ok(())
        } else {
            // Just prefixes, e.g. due to REX no-op.
            write!(f, "{}", self.prefix)
        }
    }
}

use Scale::*;
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
            Scale1 => 1,
            Scale2 => 2,
            Scale4 => 4,
            Scale8 => 8,
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
    if base.is_some() || index.is_some() {
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
        write!(f, ")")?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
pub enum Base32 {
    // EBP is actually unencodable afaict
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

#[derive(Clone, Copy)]
pub enum Index32 {
    // ESP is actually unencodable afaict
    GPR32(GPR32),
    // Eiz is just a pseudo-register that GNU uses to represent zero
    Eiz,
}
impl fmt::Display for Index32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Index32::GPR32(gpr32) => gpr32.fmt(f),
            Index32::Eiz => write!(f, "%eiz"),
        }
    }
}

#[derive(Clone)]
pub struct SIDB32 {
    pub(crate) disp: Option<i32>,
    pub(crate) base: Option<Base32>,
    pub(crate) index: Option<Index32>,
    pub(crate) scale: Scale,
}
impl fmt::Display for SIDB32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let index = match (self.base, self.index, self.scale) {
            (Some(Base32::GPR32(GPR32::esp)), Some(Index32::Eiz), Scale1) => {
                // Special case: We don't want want to show
                // (%esp, %eiz, 1) because the only way to encode it is via %eiz.
                // We do show other cases, like (, %eiz, 1) and (%ebp, %eiz, 1) because
                // they can be encoded without the %eiz; a plain (%ebp)
                // has a distinct bit representation.
                None
            }
            _ => self.index,
        };
        format_addr(f, self.disp, self.base, index, self.scale)
    }
}

#[derive(Clone, Copy)]
pub enum Base64 {
    // RBP is actually unencodable afaict
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

#[derive(Clone, Copy)]
pub enum Index64 {
    // RSP is actually unencodable afaict
    GPR64(GPR64),
    // Riz is just a pseudo-register that GNU uses to represent zero
    Riz,
}
impl fmt::Display for Index64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Index64::GPR64(gpr64) => gpr64.fmt(f),
            Index64::Riz => write!(f, "%riz"),
        }
    }
}

#[derive(Clone)]
pub struct SIDB64 {
    pub(crate) disp: Option<i32>,
    pub(crate) base: Option<Base64>,
    pub(crate) index: Option<Index64>,
    pub(crate) scale: Scale,
}
impl fmt::Display for SIDB64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let index = match (self.base, self.index, self.scale) {
            (Some(Base64::GPR64(GPR64::rsp)), Some(Index64::Riz), Scale1)
            | (None, Some(Index64::Riz), Scale1) => {
                // Special case: We don't want want to show
                // (%rsp, %riz, 1) or (, %riz, 1) because the only way to
                // encode them is via %riz.
                // We do show other cases, like (%rbp, %riz, 1) because
                // they can be encoded without the %riz; a plain (%rbp)
                // has a distinct bit representation.
                None
            }
            _ => self.index,
        };
        format_addr(f, self.disp, self.base, index, self.scale)
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
            scale: Scale1,
        })
    }

    pub fn from_base_reg64(base: GPR64) -> EffAddr {
        EffAddr::EffAddr64(SIDB64 {
            disp: None,
            base: Some(Base64::GPR64(base)),
            index: None,
            scale: Scale1,
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
                let scaled_index = match sidb.index {
                    Some(Index32::GPR32(i)) => regs.get_reg32(i) * (sidb.scale as u32),
                    Some(Index32::Eiz) => 0,
                    None => 0,
                };
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
                let scaled_index = match sidb.index {
                    Some(Index64::GPR64(i)) => regs.get_reg64(i) * (sidb.scale as u64),
                    Some(Index64::Riz) => 0,
                    None => 0,
                };
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
impl RM8 {
    fn is_reg(&self) -> bool {
        matches!(self, RM8::Reg(_))
    }
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
impl RM16 {
    fn is_reg(&self) -> bool {
        matches!(self, RM16::Reg(_))
    }
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
impl RM32 {
    fn is_reg(&self) -> bool {
        matches!(self, RM32::Reg(_))
    }
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
impl RM64 {
    fn is_reg(&self) -> bool {
        matches!(self, RM64::Reg(_))
    }
}
impl fmt::Display for RM64 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RM64::Addr(addr) => addr.fmt(f),
            RM64::Reg(gpr64) => gpr64.fmt(f),
        }
    }
}
