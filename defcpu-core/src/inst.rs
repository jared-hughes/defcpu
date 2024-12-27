use std::fmt;

use crate::{
    inst_prefixes::DisassemblyPrefix,
    registers::{GPR16, GPR32, GPR64, GPR8},
};

use Inst::*;
/// Instructions. Tuple args are in Intel order.
pub enum Inst {
    /// A no-op stemming from REX not being followed by a valid expression.
    RexNoop,
    // 88 /r; MOV r/m8, r8; Move r8 to r/m8.
    MovMR8(Addr, GPR8),
    // 8A /r; MOV r8, r/m8; Move r/m8 to r8.
    MovRM8(GPR8, Addr),
    // B0+ rb ib; MOV r8, imm8; Move imm8 to r8.
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
    pub fn with_prefix(self, prefix: DisassemblyPrefix) -> FullInst {
        FullInst {
            prefix,
            inner: self,
        }
    }
    fn fmt_operands(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RexNoop => Ok(()),
            MovMR8(addr, reg) => write!(f, "{}, {}", reg, addr),
            MovRM8(reg, addr) => write!(f, "{}, {}", addr, reg),
            MovImm8(gpr8, imm8) => write!(f, "$0x{:x}, {}", imm8, gpr8),
            MovImm16(gpr16, imm16) => write!(f, "$0x{:x}, {}", imm16, gpr16),
            MovImm32(gpr32, imm32) => write!(f, "$0x{:x}, {}", imm32, gpr32),
            MovImm64(gpr64, imm64) => write!(f, "$0x{:x}, {}", imm64, gpr64),
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

use Addr::*;
pub enum Addr {
    Reg32(GPR32),
    Reg64(GPR64),
}
impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Reg32(gpr32) => write!(f, "({})", gpr32),
            Reg64(gpr64) => write!(f, "({})", gpr64),
        }
    }
}
