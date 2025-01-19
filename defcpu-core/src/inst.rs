use std::fmt::{self, LowerHex};

use crate::{
    inst_prefixes::{
        AddressSizeAttribute::{self, *},
        DisassemblyPrefix,
    },
    registers::{Registers, GPR16, GPR32, GPR64, GPR8},
};

pub use JumpXor::*;
pub enum JumpXor {
    Normal,
    Negate,
}
impl JumpXor {
    pub(crate) fn xor(&self, b: bool) -> bool {
        match self {
            Normal => b,
            Negate => !b,
        }
    }
}

pub enum DataSize {
    Data8,
    Data16,
    Data32,
    Data64,
}
impl DataSize {
    pub fn byte_len(&self) -> u32 {
        match self {
            Self::Data8 => 1,
            Self::Data16 => 2,
            Self::Data32 => 4,
            Self::Data64 => 8,
        }
    }
}

#[derive(Clone, Copy)]
pub enum RotDir {
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum RotType {
    RclRcr,
    RolRor,
}
pub mod rot_pair {
    use super::{RotDir, RotType};

    pub const RCL: (RotType, RotDir) = (RotType::RclRcr, RotDir::Left);
    pub const RCR: (RotType, RotDir) = (RotType::RclRcr, RotDir::Right);
    pub const ROL: (RotType, RotDir) = (RotType::RolRor, RotDir::Left);
    pub const ROR: (RotType, RotDir) = (RotType::RolRor, RotDir::Right);
}

/// Prefixes described in Vol2A: 2.1.1 "Instruction Prefixes"
#[derive(Clone, Copy)]
pub enum Group1Prefix {
    Repz,
    Repnz,
}
impl fmt::Display for Group1Prefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Group1Prefix::Repz => write!(f, "repz"),
            Group1Prefix::Repnz => write!(f, "repnz"),
        }
    }
}

pub enum Group1PrefixExec {
    Repz(AddressSizeAttribute),
    Repnz(AddressSizeAttribute),
}

pub struct FullInst {
    /// The group1_prefix is None if there is no such prefix, or if they
    /// are a mandatory prefix (such as for popcnt or lzcnt).
    /// Having a group1_prefix other than None is undefined except for string and I/O instructions.
    /// Reverse-engineering those undefined behaviors for the code.golf CPU is TODO.
    pub group1_prefix: Option<Group1PrefixExec>,
    pub main_inst: Inst,
}

// Flags aren't updated until after the last iteration to make the operation faster

/// An operation that takes one U and returns a U.
#[derive(PartialEq, Eq)]
pub enum PlainOneOp {
    /// FE /0; INC r/m8; Increment r/m byte by 1.
    /// FF /0; INC r/m16; Increment r/m word by 1.
    /// Note the 40+rw increment isn't encodeable in 64-bit mode.
    /// FF /0; INC r/m32; Increment r/m doubleword by 1.
    /// Note the 40+rd increment isn't encodeable in 64-bit mode.
    /// REX.W + FF /0; INC r/m64; Increment r/m quadword by 1.
    Inc,
    /// FE /1; DEC r/m8; Decrement r/m byte by 1.
    /// FF /1; DEC r/m16; Decrement r/m word by 1.
    /// Note the 48+rw decrement isn't encodeable in 64-bit mode.
    /// FF /1; DEC r/m32; Decrement r/m doubleword by 1.
    /// Note the 48+rd decrement isn't encodeable in 64-bit mode.
    /// REX.W + FF /1; DEC r/m64; Decrement r/m quadword by 1.
    Dec,
    /// F6 /2; NOT r/m8; Reverse each bit of r/m8.
    /// F7 /2; NOT r/m16; Reverse each bit of r/m16.
    /// F7 /2; NOT r/m32; Reverse each bit of r/m32.
    /// REX.W + F7 /2; NOT r/m64; Reverse each bit of r/m64.
    Not,
}

/// An operation that takes a U::DoubleWidth and returns a U and a U.
#[derive(PartialEq, Eq)]
pub enum ShrinkOp {
    /// F6 /6; DIV r/m8; Unsigned divide AX by r/m8, with result stored in AL := Quotient, AH := Remainder.
    /// F7 /6; DIV r/m16; Unsigned divide DX:AX by r/m16, with result stored in AX := Quotient, DX := Remainder.
    /// F7 /6; DIV r/m32; Unsigned divide EDX:EAX by r/m32, with result stored in EAX := Quotient, EDX := Remainder.
    /// REX.W + F7 /6; DIV r/m64; Unsigned divide RDX:RAX by r/m64, with result stored in RAX := Quotient, RDX := Remainder.
    Div,
}

/// An operation that takes a U and a U and returns a U::DoubleWidth.
#[derive(PartialEq, Eq)]
pub enum WidenOp {
    /// F6 /5; IMUL r/m8; AX:= AL ∗ r/m byte.
    /// F7 /5; IMUL r/m16; DX:AX := AX ∗ r/m word.
    /// F7 /5; IMUL r/m32; EDX:EAX := EAX ∗ r/m32.
    /// REX.W + F7 /5; IMUL r/m64; RDX:RAX := RAX ∗ r/m64.
    Imul,
}

#[derive(PartialEq, Eq)]
pub enum OneOp {
    Plain(PlainOneOp),
    Shrink(ShrinkOp),
    Widen(WidenOp),
}
impl OneOp {
    pub(crate) const INC: Self = Self::Plain(PlainOneOp::Inc);
    pub(crate) const DEC: Self = Self::Plain(PlainOneOp::Dec);
    pub(crate) const NOT: Self = Self::Plain(PlainOneOp::Not);
    pub(crate) const DIV: Self = Self::Shrink(ShrinkOp::Div);
    pub(crate) const IMUL: Self = Self::Widen(WidenOp::Imul);
}

/// An operation that takes a U and a U and returns a U, putting the result in Dest.
/// Each of these have three forms: MR, RM, MI
#[derive(PartialEq, Eq)]
pub enum DestMROp {
    /// 00 /r; ADD r/m8, r8; Add r8 to r/m8.
    /// 01 /r; ADD r/m16, r16; Add r16 to r/m16.
    /// 01 /r; ADD r/m32, r32; Add r32 to r/m32.
    /// REX.W + 01 /r; ADD r/m64, r64; Add r64 to r/m64.
    /// 02 /r; ADD r8, r/m8; Add r/m8 to r8.
    /// 03 /r; ADD r16, r/m16; Add r/m16 to r16.
    /// 03 /r; ADD r32, r/m32; Add r/m32 to r32.
    /// REX.W + 03 /r; ADD r64, r/m64; Add r/m64 to r64.
    /// 04 ib; ADD AL, imm8; Add imm8 to AL.
    /// 80 /0 ib; ADD r/m8, imm8; Add imm8 to r/m8.
    /// 05 iw; ADD AX, imm16; Add imm16 to AX.
    /// 81 /0 iw; ADD r/m16, imm16; Add imm16 to r/m16.
    /// 83 /0 ib; ADD r/m16, imm8; Add sign-extended imm8 to r/m16.
    /// 05 id; ADD EAX, imm32; Add imm32 to EAX.
    /// 81 /0 id; ADD r/m32, imm32; Add imm32 to r/m32.
    /// 83 /0 ib; ADD r/m32, imm8; Add sign-extended imm8 to r/m32.
    /// REX.W + 05 id; ADD RAX, imm32; Add imm32 sign-extended to 64-bits to RAX.
    /// REX.W + 81 /0 id; ADD r/m64, imm32; Add imm32 sign-extended to 64-bits to r/m64.
    /// REX.W + 83 /0 ib; ADD r/m64, imm8; Add sign-extended imm8 to r/m64.
    Add,
    Or,
    Adc,
    Sbb,
    /// 20 /r; AND r/m8, r8; r/m8 AND r8.
    /// 21 /r; AND r/m16, r16; r/m16 AND r16.
    /// 21 /r; AND r/m32, r32; r/m32 AND r32.
    /// REX.W + 21 /r; AND r/m64, r64; r/m64 AND r32.
    /// 22 /r; AND r8, r/m8; r8 AND r/m8.
    /// 23 /r; AND r16, r/m16; r16 AND r/m16.
    /// 23 /r; AND r32, r/m32; r32 AND r/m32.
    /// REX.W + 23 /r; AND r64, r/m64; r64 AND r/m64.
    /// 2C ib; SUB AL, imm8; Subtract imm8 from AL.
    /// 24 ib; AND AL, imm8; AL AND imm8.
    /// 80 /4 ib; AND r/m8, imm8; r/m8 AND imm8.
    /// 25 iw; AND AX, imm16; AX AND imm16.
    /// 81 /4 iw; AND r/m16, imm16; r/m16 AND imm16.
    /// 83 /4 ib; AND r/m16, imm8; r/m16 AND imm8 (sign-extended).
    /// 25 id; AND EAX, imm32; EAX AND imm32.
    /// 81 /4 id; AND r/m32, imm32; r/m32 AND imm32.
    /// 83 /4 ib; AND r/m32, imm8; r/m32 AND imm8 (sign-extended).
    /// REX.W + 25 id; AND RAX, imm32; RAX AND imm32 sign-extended to 64-bits.
    /// REX.W + 81 /4 id; AND r/m64, imm32; r/m64 AND imm32 sign extended to 64-bits.
    /// REX.W + 83 /4 ib; AND r/m64, imm8; r/m64 AND imm8 (sign-extended).
    And,
    /// 28 /r; SUB r/m8, r8; Subtract r8 from r/m8.
    /// 29 /r; SUB r/m16, r16; Subtract r16 from r/m16.
    /// 29 /r; SUB r/m32, r32; Subtract r32 from r/m32.
    /// REX.W + 29 /r; SUB r/m64, r64; Subtract r64 from r/m64.
    /// 2A /r; SUB r8, r/m8; Subtract r/m8 from r8.
    /// 2B /r; SUB r16, r/m16; Subtract r/m16 from r16.
    /// 2B /r; SUB r32, r/m32; Subtract r/m32 from r32.
    /// REX.W + 2B /r; SUB r64, r/m64; Subtract r/m64 from r64.
    /// 80 /5 ib; SUB r/m8, imm8; Subtract imm8 from r/m8.
    /// 2D iw; SUB AX, imm16; Subtract imm16 from AX.
    /// 81 /5 iw; SUB r/m16, imm16; Subtract imm16 from r/m16.
    /// 83 /5 ib; SUB r/m16, imm8; Subtract sign-extended imm8 from r/m16.
    /// 2D id; SUB EAX, imm32; Subtract imm32 from EAX.
    /// 81 /5 id; SUB r/m32, imm32; Subtract imm32 from r/m32.
    /// 83 /5 ib; SUB r/m32, imm8; Subtract sign-extended imm8 from r/m32.
    /// REX.W + 2D id; SUB RAX, imm32; Subtract imm32 sign-extended to 64-bits from RAX.
    /// REX.W + 81 /5 id; SUB r/m64, imm32; Subtract imm32 sign-extended to 64-bits from r/m64.
    /// REX.W + 83 /5 ib; SUB r/m64, imm8; Subtract sign-extended imm8 from r/m64.
    Sub,
    /// 30 /r; XOR r/m8, r8; r/m8 XOR r8.
    /// 31 /r; XOR r/m16, r16; r/m16 XOR r16.
    /// 31 /r; XOR r/m32, r32; r/m32 XOR r32.
    /// REX.W + 31 /r; XOR r/m64, r64; r/m64 XOR r64.
    /// 32 /r; XOR r8, r/m8; r8 XOR r/m8.
    /// 33 /r; XOR r16, r/m16; r16 XOR r/m16.
    /// 33 /r; XOR r32, r/m32; r32 XOR r/m32.
    /// REX.W + 33 /r; XOR r64, r/m64; r64 XOR r/m64.
    /// 34 ib; XOR AL, imm8; AL XOR imm8.
    /// 80 /6 ib; XOR r/m8, imm8; r/m8 XOR imm8.
    /// 35 iw; XOR AX, imm16; AX XOR imm16.
    /// 81 /6 iw; XOR r/m16, imm16; r/m16 XOR imm16.
    /// 83 /6 ib; XOR r/m16, imm8; r/m16 XOR imm8 (sign-extended).
    /// 35 id; XOR EAX, imm32; EAX XOR imm32.
    /// 81 /6 id; XOR r/m32, imm32; r/m32 XOR imm32.
    /// 83 /6 ib; XOR r/m32, imm8; r/m32 XOR imm8 (sign-extended).
    /// REX.W + 35 id; XOR RAX, imm32; RAX XOR imm32 (sign-extended).
    /// REX.W + 81 /6 id; XOR r/m64, imm32; r/m64 XOR imm32 (sign-extended).
    /// REX.W + 83 /6 ib; XOR r/m64, imm8; r/m64 XOR imm8 (sign-extended).
    Xor,
    /// 38 /r; CMP r/m8, r8; Compare r8 with r/m8.
    /// 39 /r; CMP r/m16, r16; Compare r16 with r/m16.
    /// 39 /r; CMP r/m32, r32; Compare r32 with r/m32.
    /// REX.W + 39 /r; CMP r/m64, r64; Compare r64 with r/m64.
    /// 3A /r; CMP r8, r/m8; Compare r/m8 with r8.
    /// 3B /r; CMP r16, r/m16; Compare r/m16 with r16.
    /// 3B /r; CMP r32, r/m32; Compare r/m32 with r32.
    /// REX.W + 3B /r; CMP r64, r/m64; Compare r/m64 with r64.
    /// 3C ib; CMP AL, imm8; Compare imm8 with AL.
    /// 80 /7 ib; CMP r/m8, imm8; Compare imm8 with r/m8.
    /// 3D iw; CMP AX, imm16; Compare imm16 with AX.
    /// 81 /7 iw; CMP r/m16, imm16; Compare imm16 with r/m16.
    /// 83 /7 ib; CMP r/m16, imm8; Compare sign-extended imm8 with r/m16.
    /// 3D id; CMP EAX, imm32; Compare imm32 with EAX.
    /// 81 /7 id; CMP r/m32, imm32; Compare imm32 with r/m32.
    /// 83 /7 ib; CMP r/m32, imm8; Compare sign-extended imm8 with r/m32.
    /// REX.W + 3D id; CMP RAX, imm32; Compare imm32 sign-extended to 64-bits with RAX.
    /// REX.W + 81 /7 id; CMP r/m64, imm32; Compare imm32 sign-extended to 64-bits with r/m64.
    /// REX.W + 83 /7 ib; CMP r/m64, imm8; Compare sign-extended imm8 with r/m64.
    Cmp,
    /// 88 /r; MOV r/m8, r8; Move r8 to r/m8.
    /// 89 /r; MOV r/m16, r16; Move r16 to r/m16.
    /// 89 /r; MOV r/m32, r32; Move r32 to r/m32.
    /// REX.W + 89 /r; MOV r/m64, r64; Move r64 to r/m64.
    /// 8A /r; MOV r8, r/m8; Move r/m8 to r8.
    /// 8B /r; MOV r16, r/m16; Move r/m16 to r16.
    /// 8B /r; MOV r32, r/m32; Move r/m32 to r32.
    /// REX.W + 8B /r; MOV r64, r/m64; Move r/m64 to r64.
    /// C6 /0 ib; MOV r/m8, imm8; Move imm8 to r/m8.
    /// C7 /0 iw; MOV r/m16, imm16; Move imm16 to r/m16.
    /// C7 /0 id; MOV r/m32, imm32; Move imm32 to r/m32.
    /// REX.W + C7 /0 id; MOV r/m64, imm32; Move imm32 sign extended to 64-bits to r/m64.
    Mov,
}

#[derive(PartialEq, Eq)]
pub enum TwoOp {
    Dest(DestMROp),
}
impl TwoOp {
    pub(crate) const ADD: Self = Self::Dest(DestMROp::Add);
    pub(crate) const OR: Self = Self::Dest(DestMROp::Or);
    pub(crate) const ADC: Self = Self::Dest(DestMROp::Adc);
    pub(crate) const SBB: Self = Self::Dest(DestMROp::Sbb);
    pub(crate) const AND: Self = Self::Dest(DestMROp::And);
    pub(crate) const SUB: Self = Self::Dest(DestMROp::Sub);
    pub(crate) const XOR: Self = Self::Dest(DestMROp::Xor);
    pub(crate) const CMP: Self = Self::Dest(DestMROp::Cmp);
    pub(crate) const MOV: Self = Self::Dest(DestMROp::Mov);
}

use Inst::*;
/// Instructions. Tuple args are in Intel order.
/// For valid opcodes, the name of each variant is the mnemonic,
/// followed by the Op/En column then the operand size.
pub enum Inst {
    /// Haven't yet implemented this. May or may not be a valid opcode.
    NotImplemented(u8),
    /// Haven't yet implemented this 2-byte opcode.
    NotImplemented2(u8, u8),
    /// Haven't yet implemented this. May or may not be a valid opcode.
    /// Has an opcode extension: (a,b) represents a with the 3-byte b extension.
    NotImplementedOpext(u8, u8),
    /// A LEA instruction with a register instead of an effective address computation.
    LeaRegInsteadOfAddr,
    /// A no-op stemming from REX not being followed by a valid expression.
    RexNoop,
    /// NP 90; NOP; One byte no-operation instruction.
    Nop,
    /// F8; CLC; Clear CF flag.
    Clc,
    /// F9; STC; Set CF flag.
    Stc,
    /// 0F 05; SYSCALL; Fast call to privilege level 0 system procedures.
    Syscall,
    OneRMInst8(OneOp, RM8),
    OneRMInst16(OneOp, RM16),
    OneRMInst32(OneOp, RM32),
    OneRMInst64(OneOp, RM64),
    TwoRMInst8(TwoOp, GPR8, RM8),
    TwoRMInst16(TwoOp, GPR16, RM16),
    TwoRMInst32(TwoOp, GPR32, RM32),
    TwoRMInst64(TwoOp, GPR64, RM64),
    TwoMRInst8(TwoOp, RM8, GPR8),
    TwoMRInst16(TwoOp, RM16, GPR16),
    TwoMRInst32(TwoOp, RM32, GPR32),
    TwoMRInst64(TwoOp, RM64, GPR64),
    TwoMIInst8(TwoOp, RM8, u8),
    TwoMIInst16(TwoOp, RM16, u16),
    TwoMIInst32(TwoOp, RM32, u32),
    TwoMIInst64(TwoOp, RM64, u64),
    /// 8D /r; LEA r16,m; Store effective address for m in register r16.
    LeaRM16(GPR16, EffAddr),
    /// 8D /r; LEA r32,m; Store effective address for m in register r32.
    LeaRM32(GPR32, EffAddr),
    /// REX.W + 8D /r; LEA r64,m; Store effective address for m in register r64.
    LeaRM64(GPR64, EffAddr),
    /// B0+ rb ib; MOV r8, imm8; Move imm8 to r8.
    MovOI8(GPR8, u8),
    /// B8+ rw iw; MOV r16, imm16; Move imm16 to r16.
    MovOI16(GPR16, u16),
    /// B8+ rd id; MOV r32, imm32; Move imm32 to r32.
    MovOI32(GPR32, u32),
    /// REX.W + B8+ rd io; MOV r64, imm64; Move imm64 to r64.
    MovOI64(GPR64, u64),
    /// F4; HLT
    Hlt,
    /// 0F A3 /r; BT r/m16, r16; Store selected bit in CF flag.
    BtMR16(RM16, GPR16),
    /// 0F A3 /r; BT r/m32, r32; Store selected bit in CF flag.
    BtMR32(RM32, GPR32),
    /// REX.W + 0F A3 /r; BT r/m64, r64; Store selected bit in CF flag.
    BtMR64(RM64, GPR64),
    /// 0F BA /4 ib; BT r/m16, imm8; Store selected bit in CF flag.
    BtMI16(RM16, u8),
    /// 0F BA /4 ib; BT r/m32, imm8; Store selected bit in CF flag.
    BtMI32(RM32, u8),
    /// REX.W + 0F BA /4 ib; BT r/m64, imm8; Store selected bit in CF flag.
    BtMI64(RM64, u8),
    /// 0F AF /r; IMUL r16, r/m16; word register := word register ∗ r/m16.
    ImulRM16(GPR16, RM16),
    /// 0F AF /r; IMUL r32, r/m32; doubleword register := doubleword register ∗ r/m32.
    ImulRM32(GPR32, RM32),
    /// REX.W + 0F AF /r; IMUL r64, r/m64; Quadword register := Quadword register ∗ r/m64.
    ImulRM64(GPR64, RM64),
    /// 6B /r ib; IMUL r16, r/m16, imm8; word register := r/m16 ∗ sign-extended immediate byte.
    /// 69 /r iw; IMUL r16, r/m16, imm16; word register := r/m16 ∗ immediate word.
    ImulRMI16(GPR16, RM16, u16),
    /// 6B /r ib; IMUL r32, r/m32, imm8; doubleword register := r/m32 ∗ sign-extended immediate byte.
    /// 69 /r id; IMUL r32, r/m32, imm32; doubleword register := r/m32 ∗ immediate doubleword.
    ImulRMI32(GPR32, RM32, u32),
    /// REX.W + 6B /r ib; IMUL r64, r/m64, imm8; Quadword register := r/m64 ∗ sign-extended immediate byte.
    /// REX.W + 69 /r id; IMUL r64, r/m64, imm32; Quadword register := r/m64 ∗ immediate doubleword.
    ImulRMI64(GPR64, RM64, u64),
    /// C0 /0 ib; ROL r/m8, imm8; Rotate 8 bits r/m8 left imm8 times.
    /// D0 /0; ROL r/m8, 1; Rotate 8 bits r/m8 left once.
    /// C0 /1 ib; ROR r/m8, imm8; Rotate 8 bits r/m16 right imm8 times.
    /// D0 /1; ROR r/m8, 1; Rotate 8 bits r/m8 right once.
    /// C0 /2 ib; RCL r/m8, imm8; Rotate 9 bits (CF, r/m8) left imm8 times.
    /// D0 /2; RCL r/m8, 1; Rotate 9 bits (CF, r/m8) left once.
    /// C0 /3 ib; RCR r/m8, imm8; Rotate 9 bits (CF, r/m8) right imm8 times.
    /// D0 /3; RCR r/m8, 1; Rotate 9 bits (CF, r/m8) right once.
    RotateMI8((RotType, RotDir), RM8, u8),
    /// C1 /0 ib; ROL r/m16, imm8; Rotate 16 bits r/m16 left imm8 times.
    /// D1 /0; ROL r/m16, 1; Rotate 16 bits r/m16 left once.
    /// C1 /1 ib; ROR r/m16, imm8; Rotate 16 bits r/m16 right imm8 times.
    /// D1 /1; ROR r/m16, 1; Rotate 16 bits r/m16 right once.
    /// C1 /2 ib; RCL r/m16, imm8; Rotate 17 bits (CF, r/m16) left imm8 times.
    /// D1 /2; RCL r/m16, 1; Rotate 17 bits (CF, r/m16) left once.
    /// C1 /3 ib; RCR r/m16, imm8; Rotate 17 bits (CF, r/m16) right imm8 times.
    /// D1 /3; RCR r/m16, 1; Rotate 17 bits (CF, r/m16) right once.
    RotateMI16((RotType, RotDir), RM16, u8),
    /// C1 /0 ib; ROL r/m32, imm8; Rotate 32 bits r/m32 left imm8 times.
    /// D1 /0; ROL r/m32, 1; Rotate 32 bits r/m32 left once.
    /// C1 /1 ib; ROR r/m32, imm8; Rotate 32 bits r/m32 right imm8 times.
    /// D1 /1; ROR r/m32, 1; Rotate 32 bits r/m32 right once.
    /// C1 /2 ib; RCL r/m32, imm8; Rotate 33 bits (CF, r/m32) left imm8 times.
    /// D1 /2; RCL r/m32, 1; Rotate 33 bits (CF, r/m32) left once.
    /// C1 /3 ib; RCR r/m32, imm8; Rotate 33 bits (CF, r/m32) right imm8 times.
    /// D1 /3; RCR r/m32, 1; Rotate 33 bits (CF, r/m32) right once. Uses a 6 bit count.
    RotateMI32((RotType, RotDir), RM32, u8),
    /// REX.W + C1 /0 ib; ROL r/m64, imm8; Rotate 64 bits r/m64 left imm8 times. Uses a 6 bit count.
    /// REX.W + D1 /0; ROL r/m64, 1; Rotate 64 bits r/m64 left once. Uses a 6 bit count.
    /// REX.W + C1 /1 ib; ROR r/m64, imm8; Rotate 64 bits r/m64 right imm8 times. Uses a 6 bit count.
    /// REX.W + D1 /1; ROR r/m64, 1; Rotate 64 bits r/m64 right once. Uses a 6 bit count.
    /// REX.W + C1 /2 ib; RCL r/m64, imm8; Rotate 65 bits (CF, r/m64) left imm8 times. Uses a 6 bit count.
    /// REX.W + D1 /2; RCL r/m64, 1; Rotate 65 bits (CF, r/m64) left once. Uses a 6 bit count.
    /// REX.W + C1 /3 ib; RCR r/m64, imm8; Rotate 65 bits (CF, r/m64) right imm8 times. Uses a 6 bit count.
    /// REX.W + D1 /3; RCR r/m64, 1; Rotate 65 bits (CF, r/m64) right once. Uses a 6 bit count.
    RotateMI64((RotType, RotDir), RM64, u8),
    /// D2 /0; ROL r/m8, CL; Rotate 8 bits r/m8 left CL times.
    /// D2 /1; ROR r/m8, CL; Rotate 8 bits r/m8 right CL times.
    /// D2 /2; RCL r/m8, CL; Rotate 9 bits (CF, r/m8) left CL times.
    /// D2 /3; RCR r/m8, CL; Rotate 9 bits (CF, r/m8) right CL times.
    RotateMC8((RotType, RotDir), RM8),
    /// D3 /0; ROL r/m16, CL; Rotate 16 bits r/m16 left CL times.
    /// D3 /1; ROR r/m16, CL; Rotate 16 bits r/m16 right CL times.
    /// D3 /2; RCL r/m16, CL; Rotate 17 bits (CF, r/m16) left CL times.
    /// D3 /3; RCR r/m16, CL; Rotate 17 bits (CF, r/m16) right CL times.
    RotateMC16((RotType, RotDir), RM16),
    /// D3 /0; ROL r/m32, CL; Rotate 32 bits r/m32 left CL times.
    /// D3 /1; ROR r/m32, CL; Rotate 32 bits r/m32 right CL times.
    /// D3 /2; RCL r/m32, CL; Rotate 33 bits (CF, r/m32) left CL times.
    /// D3 /3; RCR r/m32, CL; Rotate 33 bits (CF, r/m32) right CL times.
    RotateMC32((RotType, RotDir), RM32),
    /// REX.W + D3 /0; ROL r/m64, CL; Rotate 64 bits r/m64 left CL times. Uses a 6 bit count.
    /// REX.W + D3 /1; ROR r/m64, CL; Rotate 64 bits r/m64 right CL times. Uses a 6 bit count.
    /// REX.W + D3 /2; RCL r/m64, CL; Rotate 65 bits (CF, r/m64) left CL times. Uses a 6 bit count.
    /// REX.W + D3 /3; RCR r/m64, CL; Rotate 65 bits (CF, r/m64) right CL times. Uses a 6 bit count.
    RotateMC64((RotType, RotDir), RM64),
    BswapO32(GPR32),
    BswapO64(GPR64),
    /// 70 cb; JO rel8; Jump short if overflow (OF=1).
    /// 0F 80 cd; JO rel32; Jump near if overflow (OF=1).
    /// 71 cb; JNO rel8; Jump short if not overflow (OF=0).
    /// 0F 81 cd; JNO rel32; Jump near if not overflow (OF=0).
    JccJo(u64, JumpXor),
    /// 72 cb; JB rel8; Jump short if below (CF=1).
    /// 72 cb; JC rel8; Jump short if carry (CF=1).
    /// 72 cb; JNAE rel8; Jump short if not above or equal (CF=1).
    /// 0F 82 cd; JB rel32; Jump near if below (CF=1).
    /// 0F 82 cd; JC rel32; Jump near if carry (CF=1).
    /// 0F 82 cd; JNAE rel32; Jump near if not above or equal (CF=1).
    /// 73 cb; JAE rel8; Jump short if above or equal (CF=0).
    /// 73 cb; JNB rel8; Jump short if not below (CF=0).
    /// 73 cb; JNC rel8; Jump short if not carry (CF=0).
    /// 0F 83 cd; JAE rel32; Jump near if above or equal (CF=0).
    /// 0F 83 cd; JNB rel32; Jump near if not below (CF=0).
    /// 0F 83 cd; JNC rel32; Jump near if not carry (CF=0).
    JccJb(u64, JumpXor),
    /// 74 cb; JE rel8; Jump short if equal (ZF=1).
    /// 74 cb; JZ rel8; Jump short if zero (ZF=1).
    /// 0F 84 cd; JE rel32; Jump near if equal (ZF=1).
    /// 0F 84 cd; JZ rel32; Jump near if 0 (ZF=1).
    /// 0F 84 cd; JZ rel32; Jump near if 0 (ZF=1).
    /// 75 cb; JNE rel8; Jump short if not equal (ZF=0).
    /// 75 cb; JNZ rel8; Jump short if not zero (ZF=0).
    /// 0F 85 cd; JNE rel32; Jump near if not equal (ZF=0).
    /// 0F 85 cd; JNZ rel32; Jump near if not zero (ZF=0).
    JccJe(u64, JumpXor),
    /// 76 cb; JBE rel8; Jump short if below or equal (CF=1 or ZF=1).
    /// 76 cb; JNA rel8; Jump short if not above (CF=1 or ZF=1).
    /// 0F 86 cd; JBE rel32; Jump near if below or equal (CF=1 or ZF=1).
    /// 0F 86 cd; JNA rel32; Jump near if not above (CF=1 or ZF=1).
    /// 77 cb; JA rel8; Jump short if above (CF=0 and ZF=0).
    /// 77 cb; JNBE rel8; Jump short if not below or equal (CF=0 and ZF=0).
    /// 0F 87 cd; JA rel32; Jump near if above (CF=0 and ZF=0).
    /// 0F 87 cd; JNBE rel32; Jump near if not below or equal (CF=0 and ZF=0).
    JccJbe(u64, JumpXor),
    /// 78 cb; JS rel8; Jump short if sign (SF=1).
    /// 0F 88 cd; JS rel32; Jump near if sign (SF=1).
    /// 79 cb; JNS rel8; Jump short if not sign (SF=0).
    /// 0F 89 cd; JNS rel32; Jump near if not sign (SF=0).
    JccJs(u64, JumpXor),
    /// 7A cb; JPE rel8; Jump short if parity even (PF=1).
    /// 7A cb; JP rel8; Jump short if parity (PF=1).
    /// 0F 8A cd; JPE rel32; Jump near if parity even (PF=1).
    /// 0F 8A cd; JP rel32; Jump near if parity (PF=1).
    /// 7B cb; JNP rel8; Jump short if not parity (PF=0).
    /// 7B cb; JPO rel8; Jump short if parity odd (PF=0).
    /// 0F 8B cd; JNP rel32; Jump near if not parity (PF=0).
    /// 0F 8B cd; JPO rel32; Jump near if parity odd (PF=0).
    JccJp(u64, JumpXor),
    /// 7C cb; JL rel8; Jump short if less (SF≠ OF).
    /// 7C cb; JNGE rel8; Jump short if not greater or equal (SF≠ OF).
    /// 0F 8C cd; JL rel32; Jump near if less (SF≠ OF).
    /// 0F 8C cd; JNGE rel32; Jump near if not greater or equal (SF≠ OF).
    /// 7D cb; JGE rel8; Jump short if greater or equal (SF=OF).
    /// 7D cb; JNL rel8; Jump short if not less (SF=OF).
    /// 0F 8D cd; JGE rel32; Jump near if greater or equal (SF=OF).
    /// 0F 8D cd; JNL rel32; Jump near if not less (SF=OF).
    JccJl(u64, JumpXor),
    /// 7E cb; JLE rel8; Jump short if less or equal (ZF=1 or SF≠ OF).
    /// 7E cb; JNG rel8; Jump short if not greater (ZF=1 or SF≠ OF).
    /// 0F 8E cd; JLE rel32; Jump near if less or equal (ZF=1 or SF≠ OF).
    /// 0F 8E cd; JNG rel32; Jump near if not greater (ZF=1 or SF≠ OF).
    /// 7F cb; JG rel8; Jump short if greater (ZF=0 and SF=OF).
    /// 7F cb; JNLE rel8; Jump short if not less or equal (ZF=0 and SF=OF).
    /// 0F 8F cd; JG rel32; Jump near if greater (ZF=0 and SF=OF).
    /// 0F 8F cd; JNLE rel32; Jump near if not less or equal (ZF=0 and SF=OF).
    JccJle(u64, JumpXor),
    /// E3 cb; JECXZ rel8; Jump short if ECX register is 0.
    Jecxz(u64),
    /// E3 cb; JRCXZ rel8; Jump short if RCX register is 0.
    Jrcxz(u64),
    Ret,
    // TODO-golf: merge CallD and JmpD. These are relative.
    /// E8 cd; CALL rel32; Call near, relative, displacement relative to next instruction. 32-bit displacement sign extended to 64-bits in 64-bit mode.
    CallD(u64),
    /// EB cb; JMP rel8; Jump short, RIP = RIP + 8-bit displacement sign extended to 64-bits.
    /// E9 cd; JMP rel32; Jump near, relative, RIP = RIP + 32-bit displacement sign extended to 64-bits.
    JmpD(u64),
    /// FF /4; JMP r/m64; Jump near, absolute indirect, RIP = 64-Bit offset from register or memory.
    JmpM64(RM64),
    // FF /5; JMP m16:16; Jump far, absolute indirect, address given in m16:16.
    // FF /5; JMP m16:32; Jump far, absolute indirect, address given in m16:32.
    // REX.W FF /5; JMP m16:64; Jump far, absolute indirect, address given in m16:64.
    // TODO: Memory-indirect far jump (ljmp). I'm not messing around with CS currently.
    // JmpM(...),
    /// 8F /0; POP r/m16; Pop top of stack into m16; increment stack pointer.
    /// 58+ rw; POP r16; Pop top of stack into r16; increment stack pointer.
    PopM16(RM16),
    /// 8F /0; POP r/m64; Pop top of stack into m64; increment stack pointer. Cannot encode 32-bit operand size.
    /// 58+ rd; POP r64; Pop top of stack into r64; increment stack pointer. Cannot encode 32-bit operand size.
    PopM64(RM64),
    // 9D; POPF; Pop top of stack into lower 16 bits of EFLAGS.
    Popf16,
    // 9D; POPFQ; Pop top of stack and zero-extend into RFLAGS.
    Popf64,
    /// FF /6; PUSH r/m16; Push r/m16.
    /// 50+rw; PUSH r16; Push r16.
    PushM16(RM16),
    /// FF /6; PUSH r/m64; Push r/m64.
    /// 50+rd; PUSH r64; Push r64.
    PushM64(RM64),
    /// Any of the following, with a 0x66 operand-size prefix.
    /// 6A ib; PUSH imm8; Push imm8.
    /// 68 iw; PUSH imm16; Push imm16.
    /// 68 id; PUSH imm32; Push imm32.
    PushI16(u16),
    /// 6A ib; PUSH imm8; Push imm8.
    /// 68 iw; PUSH imm16; Push imm16.
    /// 68 id; PUSH imm32; Push imm32.
    PushI64(u64),
    /// 9C; PUSHF; Push lower 16 bits of EFLAGS.
    Pushf16,
    /// 9C; PUSHFQ; Push RFLAGS.
    Pushf64,
    /// AE; SCAS m8; Compare AL with byte at ES:(E)DI or RDI, then set status flags.
    /// AE; SCASB; Compare AL with byte at ES:(E)DI or RDI then set status flags.
    /// AF; SCAS m16; Compare AX with word at ES:(E)DI or RDI, then set status flags.
    /// AF; SCASW; Compare AX with word at ES:(E)DI or RDI then set status flags.
    /// AF; SCAS m32; Compare EAX with doubleword at ES(E)DI or RDI then set status flags.
    /// AF; SCASD; Compare EAX with doubleword at ES:(E)DI or RDI then set status flags.
    /// REX.W + AF; SCAS m64; Compare RAX with quadword at RDI or EDI then set status flags.
    /// REX.W + AF; SCASQ; Compare RAX with quadword at RDI or EDI then set status flags.
    /// Note the e.g. m32 form is "explicit-operand form". You put in a register,
    /// but it only represents the %al/%ax/%eax/%rax of the same size.
    Scas(DataSize, AddressSizeAttribute),
    /// 86 /r; XCHG r/m8, r8; Exchange r8 (byte register) with byte from r/m8.
    /// 86 /r; XCHG r8, r/m8; Exchange byte from r/m8 with r8 (byte register).
    XchgMR8(RM8, GPR8),
    /// 90+rw; XCHG AX, r16; Exchange r16 with AX.
    /// 90+rw; XCHG r16, AX; Exchange AX with r16.
    /// 87 /r; XCHG r/m16, r16; Exchange r16 with word from r/m16.
    /// 87 /r; XCHG r16, r/m16; Exchange word from r/m16 with r16.
    XchgMR16(RM16, GPR16),
    /// 90+rd; XCHG EAX, r32; Exchange r32 with EAX.
    /// 90+rd; XCHG r32, EAX; Exchange EAX with r32.
    /// 87 /r; XCHG r/m32, r32; Exchange r32 with doubleword from r/m32.
    /// 87 /r; XCHG r32, r/m32; Exchange doubleword from r/m32 with r32.
    XchgMR32(RM32, GPR32),
    /// REX.W + 90+rd; XCHG RAX, r64; Exchange r64 with RAX.
    /// REX.W + 90+rd; XCHG r64, RAX; Exchange RAX with r64.
    /// REX.W + 87 /r; XCHG r/m64, r64; Exchange r64 with quadword from r/m64.
    /// REX.W + 87 /r; XCHG r64, r/m64; Exchange quadword from r/m64 with r64.
    XchgMR64(RM64, GPR64),
    /// 99; CWD; DX:AX := sign-extend of AX. aka 'cwtd'
    Cwd16,
    /// 99; CDQ; EDX:EAX := sign-extend of EAX. aka 'cltd'
    Cdq32,
    /// REX.W + 99; CQO; RDX:RAX:= sign-extend of RAX. aka 'cqto'
    Cqo64,
    /// 98; CBW; ZO; AX := sign-extend of AL. aka 'cbtw'
    Cbw16,
    /// 98; CWDE; ZO; EAX := sign-extend of AX. aka 'cwtl'
    Cwde32,
    /// REX.W + 98; CDQE; ZO; RAX := sign-extend of EAX. aka 'cltq'
    Cdqe64,
}

impl Inst {
    fn operands(&self) -> String {
        match self {
            NotImplemented(_)
            | NotImplemented2(_, _)
            | NotImplementedOpext(_, _)
            | LeaRegInsteadOfAddr
            | RexNoop
            | Syscall
            | Ret
            | Stc
            | Clc
            | Popf16
            | Popf64
            | Pushf16
            | Pushf64
            | Cwd16
            | Cdq32
            | Cqo64
            | Cbw16
            | Cwde32
            | Cdqe64 => String::new(),
            Nop => String::new(),
            LeaRM16(gpr16, eff_addr) => format!("{}, {}", eff_addr, gpr16),
            LeaRM32(gpr32, eff_addr) => format!("{}, {}", eff_addr, gpr32),
            LeaRM64(gpr64, eff_addr) => format!("{}, {}", eff_addr, gpr64),
            XchgMR8(rm8, gpr8) => {
                format!("{}, {}", gpr8, rm8)
            }
            XchgMR16(rm16, gpr16) | BtMR16(rm16, gpr16) => {
                format!("{}, {}", gpr16, rm16)
            }
            XchgMR32(rm32, gpr32) | BtMR32(rm32, gpr32) => {
                format!("{}, {}", gpr32, rm32)
            }
            XchgMR64(rm64, gpr64) | BtMR64(rm64, gpr64) => {
                format!("{}, {}", gpr64, rm64)
            }
            ImulRM16(gpr16, rm16) => {
                format!("{}, {}", rm16, gpr16)
            }
            ImulRM32(gpr32, rm32) => {
                format!("{}, {}", rm32, gpr32)
            }
            ImulRM64(gpr64, rm64) => {
                format!("{}, {}", rm64, gpr64)
            }
            TwoMRInst8(_, rm8, gpr8) => format!("{}, {}", gpr8, rm8),
            TwoMRInst16(_, rm16, gpr16) => format!("{}, {}", gpr16, rm16),
            TwoMRInst32(_, rm32, gpr32) => format!("{}, {}", gpr32, rm32),
            TwoMRInst64(_, rm64, gpr64) => format!("{}, {}", gpr64, rm64),
            TwoRMInst8(_, gpr8, rm8) => format!("{}, {}", rm8, gpr8),
            TwoRMInst16(_, gpr16, rm16) => format!("{}, {}", rm16, gpr16),
            TwoRMInst32(_, gpr32, rm32) => format!("{}, {}", rm32, gpr32),
            TwoRMInst64(_, gpr64, rm64) => format!("{}, {}", rm64, gpr64),
            MovOI8(gpr8, imm8) => format!("${:#x}, {}", imm8, gpr8),
            MovOI16(gpr16, imm16) => format!("${:#x}, {}", imm16, gpr16),
            MovOI32(gpr32, imm32) => format!("${:#x}, {}", imm32, gpr32),
            MovOI64(gpr64, imm64) => format!("${:#x}, {}", imm64, gpr64),
            TwoMIInst8(_, rm8, imm8) => {
                format!("${:#x}, {}", imm8, rm8)
            }
            TwoMIInst16(_, rm16, imm16) => {
                format!("${:#x}, {}", imm16, rm16)
            }
            TwoMIInst32(_, rm32, imm32) => {
                format!("${:#x}, {}", imm32, rm32)
            }
            TwoMIInst64(_, rm64, imm64) => {
                format!("${:#x}, {}", imm64, rm64)
            }
            Hlt => String::new(),
            OneRMInst8(_, rm8) => format!("{}", rm8),
            OneRMInst16(_, rm16) | PushM16(rm16) | PopM16(rm16) => {
                format!("{}", rm16)
            }
            OneRMInst32(_, rm32) => {
                format!("{}", rm32)
            }
            OneRMInst64(_, rm64) | PushM64(rm64) | PopM64(rm64) => {
                format!("{}", rm64)
            }
            ImulRMI16(gpr16, rm16, imm16) => format!("${imm16:#x}, {rm16}, {gpr16}"),
            ImulRMI32(gpr32, rm32, imm32) => format!("${imm32:#x}, {rm32}, {gpr32}"),
            ImulRMI64(gpr64, rm64, imm64) => format!("${imm64:#x}, {rm64}, {gpr64}"),
            JccJo(imm64, _)
            | JccJb(imm64, _)
            | JccJe(imm64, _)
            | JccJbe(imm64, _)
            | JccJs(imm64, _)
            | JccJp(imm64, _)
            | JccJl(imm64, _)
            | JccJle(imm64, _)
            | Jecxz(imm64)
            | Jrcxz(imm64)
            | JmpD(imm64)
            | CallD(imm64) => format!("{:#x}", imm64),
            JmpM64(rm64) => format!("*{}", rm64),
            PushI16(imm16) => format!("${:#x}", imm16),
            PushI64(imm64) => format!("${:#x}", imm64),
            // gdb picked the worst possible disassembly smh.
            Scas(DataSize::Data8, Addr32) => "%es:(%edi), %al".to_owned(),
            Scas(DataSize::Data16, Addr32) => "%es:(%edi), %ax".to_owned(),
            Scas(DataSize::Data32, Addr32) => "%es:(%edi), %eax".to_owned(),
            Scas(DataSize::Data64, Addr32) => "%es:(%edi), %rax".to_owned(),
            Scas(DataSize::Data8, Addr64) => "%es:(%rdi), %al".to_owned(),
            Scas(DataSize::Data16, Addr64) => "%es:(%rdi), %ax".to_owned(),
            Scas(DataSize::Data32, Addr64) => "%es:(%rdi), %eax".to_owned(),
            Scas(DataSize::Data64, Addr64) => "%es:(%rdi), %rax".to_owned(),
            BtMI16(rm16, imm8) => format!("${:#x}, {}", imm8, rm16),
            BtMI32(rm32, imm8) => format!("${:#x}, {}", imm8, rm32),
            BtMI64(rm64, imm8) => format!("${:#x}, {}", imm8, rm64),
            RotateMI8(_, rm8, imm8) => {
                if *imm8 == 1 {
                    format!("{}", rm8)
                } else {
                    format!("${:#x}, {}", imm8, rm8)
                }
            }
            RotateMI16(_, rm16, imm8) => {
                if *imm8 == 1 {
                    format!("{}", rm16)
                } else {
                    format!("${:#x}, {}", imm8, rm16)
                }
            }
            RotateMI32(_, rm32, imm8) => {
                if *imm8 == 1 {
                    format!("{}", rm32)
                } else {
                    format!("${:#x}, {}", imm8, rm32)
                }
            }
            RotateMI64(_, rm64, imm8) => {
                if *imm8 == 1 {
                    format!("{}", rm64)
                } else {
                    format!("${:#x}, {}", imm8, rm64)
                }
            }
            RotateMC8(_, rm8) => format!("%cl, {}", rm8),
            RotateMC16(_, rm16) => format!("%cl, {}", rm16),
            RotateMC32(_, rm32) => format!("%cl, {}", rm32),
            RotateMC64(_, rm64) => format!("%cl, {}", rm64),
            BswapO32(gpr32) => format!("{gpr32}"),
            BswapO64(gpr64) => format!("{gpr64}"),
        }
    }
    fn mnemonic(&self) -> &str {
        match self {
            NotImplemented(_)
            | NotImplemented2(_, _)
            | NotImplementedOpext(_, _)
            | LeaRegInsteadOfAddr => "(bad)",
            RexNoop => "",
            Nop => "nop",
            Syscall => "syscall",
            Stc => "stc",
            Clc => "clc",
            Ret => "ret",
            // Ref https://sourceware.org/binutils/docs/as/i386_002dMnemonics.html.
            Cwd16 => "cwtd",
            Cdq32 => "cltd",
            Cqo64 => "cqto",
            Cbw16 => "cbtw",
            Cwde32 => "cwtl",
            Cdqe64 => "cltq",
            LeaRM16(_, _) | LeaRM32(_, _) | LeaRM64(_, _) => "lea",
            TwoRMInst8(TwoOp::MOV, _, _)
            | TwoRMInst16(TwoOp::MOV, _, _)
            | TwoRMInst32(TwoOp::MOV, _, _)
            | TwoRMInst64(TwoOp::MOV, _, _)
            | TwoMRInst8(TwoOp::MOV, _, _)
            | TwoMRInst16(TwoOp::MOV, _, _)
            | TwoMRInst32(TwoOp::MOV, _, _)
            | TwoMRInst64(TwoOp::MOV, _, _)
            | MovOI8(_, _)
            | MovOI16(_, _)
            | MovOI32(_, _) => "mov",
            TwoMIInst8(TwoOp::MOV, rm8, _) => rm8.either("mov", "movb"),
            TwoMIInst16(TwoOp::MOV, rm16, _) => rm16.either("mov", "movw"),
            TwoMIInst32(TwoOp::MOV, rm32, _) => rm32.either("mov", "movl"),
            TwoMIInst64(TwoOp::MOV, rm64, _) => rm64.either("mov", "movq"),
            // movabs just does the same, idk why gdb dumps as movabs.
            MovOI64(_, _) => "movabs",
            Hlt => "hlt",
            OneRMInst8(OneOp::INC, rm8) => rm8.either("inc", "incb"),
            OneRMInst16(OneOp::INC, rm16) => rm16.either("inc", "incw"),
            OneRMInst32(OneOp::INC, rm32) => rm32.either("inc", "incl"),
            OneRMInst64(OneOp::INC, rm64) => rm64.either("inc", "incq"),
            OneRMInst8(OneOp::DEC, rm8) => rm8.either("dec", "decb"),
            OneRMInst16(OneOp::DEC, rm16) => rm16.either("dec", "decw"),
            OneRMInst32(OneOp::DEC, rm32) => rm32.either("dec", "decl"),
            OneRMInst64(OneOp::DEC, rm64) => rm64.either("dec", "decq"),
            OneRMInst8(OneOp::NOT, rm8) => rm8.either("not", "notb"),
            OneRMInst16(OneOp::NOT, rm16) => rm16.either("not", "notw"),
            OneRMInst32(OneOp::NOT, rm32) => rm32.either("not", "notl"),
            OneRMInst64(OneOp::NOT, rm64) => rm64.either("not", "notq"),
            OneRMInst8(OneOp::DIV, rm8) => rm8.either("div", "divb"),
            OneRMInst16(OneOp::DIV, rm16) => rm16.either("div", "divw"),
            OneRMInst32(OneOp::DIV, rm32) => rm32.either("div", "divl"),
            OneRMInst64(OneOp::DIV, rm64) => rm64.either("div", "divq"),
            OneRMInst8(OneOp::IMUL, rm8) => rm8.either("imul", "imulb"),
            OneRMInst16(OneOp::IMUL, rm16) => rm16.either("imul", "imulw"),
            OneRMInst32(OneOp::IMUL, rm32) => rm32.either("imul", "imull"),
            OneRMInst64(OneOp::IMUL, rm64) => rm64.either("imul", "imulq"),
            TwoMIInst8(TwoOp::ADD, rm8, _) => rm8.either("add", "addb"),
            TwoMIInst16(TwoOp::ADD, rm16, _) => rm16.either("add", "addw"),
            TwoMIInst32(TwoOp::ADD, rm32, _) => rm32.either("add", "addl"),
            TwoMIInst64(TwoOp::ADD, rm64, _) => rm64.either("add", "addq"),
            TwoRMInst8(TwoOp::ADD, _, _)
            | TwoRMInst16(TwoOp::ADD, _, _)
            | TwoRMInst32(TwoOp::ADD, _, _)
            | TwoRMInst64(TwoOp::ADD, _, _)
            | TwoMRInst8(TwoOp::ADD, _, _)
            | TwoMRInst16(TwoOp::ADD, _, _)
            | TwoMRInst32(TwoOp::ADD, _, _)
            | TwoMRInst64(TwoOp::ADD, _, _) => "add",
            TwoMIInst8(TwoOp::OR, rm8, _) => rm8.either("or", "orb"),
            TwoMIInst16(TwoOp::OR, rm16, _) => rm16.either("or", "orw"),
            TwoMIInst32(TwoOp::OR, rm32, _) => rm32.either("or", "orl"),
            TwoMIInst64(TwoOp::OR, rm64, _) => rm64.either("or", "orq"),
            TwoRMInst8(TwoOp::OR, _, _)
            | TwoRMInst16(TwoOp::OR, _, _)
            | TwoRMInst32(TwoOp::OR, _, _)
            | TwoRMInst64(TwoOp::OR, _, _)
            | TwoMRInst8(TwoOp::OR, _, _)
            | TwoMRInst16(TwoOp::OR, _, _)
            | TwoMRInst32(TwoOp::OR, _, _)
            | TwoMRInst64(TwoOp::OR, _, _) => "or",
            TwoMIInst8(TwoOp::ADC, rm8, _) => rm8.either("adc", "adcb"),
            TwoMIInst16(TwoOp::ADC, rm16, _) => rm16.either("adc", "adcw"),
            TwoMIInst32(TwoOp::ADC, rm32, _) => rm32.either("adc", "adcl"),
            TwoMIInst64(TwoOp::ADC, rm64, _) => rm64.either("adc", "adcq"),
            TwoRMInst8(TwoOp::ADC, _, _)
            | TwoRMInst16(TwoOp::ADC, _, _)
            | TwoRMInst32(TwoOp::ADC, _, _)
            | TwoRMInst64(TwoOp::ADC, _, _)
            | TwoMRInst8(TwoOp::ADC, _, _)
            | TwoMRInst16(TwoOp::ADC, _, _)
            | TwoMRInst32(TwoOp::ADC, _, _)
            | TwoMRInst64(TwoOp::ADC, _, _) => "adc",
            TwoMIInst8(TwoOp::SBB, rm8, _) => rm8.either("sbb", "sbbb"),
            TwoMIInst16(TwoOp::SBB, rm16, _) => rm16.either("sbb", "sbbw"),
            TwoMIInst32(TwoOp::SBB, rm32, _) => rm32.either("sbb", "sbbl"),
            TwoMIInst64(TwoOp::SBB, rm64, _) => rm64.either("sbb", "sbbq"),
            TwoRMInst8(TwoOp::SBB, _, _)
            | TwoRMInst16(TwoOp::SBB, _, _)
            | TwoRMInst32(TwoOp::SBB, _, _)
            | TwoRMInst64(TwoOp::SBB, _, _)
            | TwoMRInst8(TwoOp::SBB, _, _)
            | TwoMRInst16(TwoOp::SBB, _, _)
            | TwoMRInst32(TwoOp::SBB, _, _)
            | TwoMRInst64(TwoOp::SBB, _, _) => "sbb",
            TwoMIInst8(TwoOp::AND, rm8, _) => rm8.either("and", "andb"),
            TwoMIInst16(TwoOp::AND, rm16, _) => rm16.either("and", "andw"),
            TwoMIInst32(TwoOp::AND, rm32, _) => rm32.either("and", "andl"),
            TwoMIInst64(TwoOp::AND, rm64, _) => rm64.either("and", "andq"),
            TwoMRInst8(TwoOp::AND, _, _)
            | TwoMRInst16(TwoOp::AND, _, _)
            | TwoMRInst32(TwoOp::AND, _, _)
            | TwoMRInst64(TwoOp::AND, _, _)
            | TwoRMInst8(TwoOp::AND, _, _)
            | TwoRMInst16(TwoOp::AND, _, _)
            | TwoRMInst32(TwoOp::AND, _, _)
            | TwoRMInst64(TwoOp::AND, _, _) => "and",
            TwoMIInst8(TwoOp::SUB, rm8, _) => rm8.either("sub", "subb"),
            TwoMIInst16(TwoOp::SUB, rm16, _) => rm16.either("sub", "subw"),
            TwoMIInst32(TwoOp::SUB, rm32, _) => rm32.either("sub", "subl"),
            TwoMIInst64(TwoOp::SUB, rm64, _) => rm64.either("sub", "subq"),
            TwoMRInst8(TwoOp::SUB, _, _)
            | TwoMRInst16(TwoOp::SUB, _, _)
            | TwoMRInst32(TwoOp::SUB, _, _)
            | TwoMRInst64(TwoOp::SUB, _, _)
            | TwoRMInst8(TwoOp::SUB, _, _)
            | TwoRMInst16(TwoOp::SUB, _, _)
            | TwoRMInst32(TwoOp::SUB, _, _)
            | TwoRMInst64(TwoOp::SUB, _, _) => "sub",
            TwoMIInst8(TwoOp::CMP, rm8, _) => rm8.either("cmp", "cmpb"),
            TwoMIInst16(TwoOp::CMP, rm16, _) => rm16.either("cmp", "cmpw"),
            TwoMIInst32(TwoOp::CMP, rm32, _) => rm32.either("cmp", "cmpl"),
            TwoMIInst64(TwoOp::CMP, rm64, _) => rm64.either("cmp", "cmpq"),
            TwoMRInst8(TwoOp::CMP, _, _)
            | TwoMRInst16(TwoOp::CMP, _, _)
            | TwoMRInst32(TwoOp::CMP, _, _)
            | TwoMRInst64(TwoOp::CMP, _, _)
            | TwoRMInst8(TwoOp::CMP, _, _)
            | TwoRMInst16(TwoOp::CMP, _, _)
            | TwoRMInst32(TwoOp::CMP, _, _)
            | TwoRMInst64(TwoOp::CMP, _, _) => "cmp",
            TwoMRInst8(TwoOp::XOR, _, _)
            | TwoMRInst16(TwoOp::XOR, _, _)
            | TwoMRInst32(TwoOp::XOR, _, _)
            | TwoMRInst64(TwoOp::XOR, _, _)
            | TwoRMInst8(TwoOp::XOR, _, _)
            | TwoRMInst16(TwoOp::XOR, _, _)
            | TwoRMInst32(TwoOp::XOR, _, _)
            | TwoRMInst64(TwoOp::XOR, _, _) => "xor",
            TwoMIInst8(TwoOp::XOR, rm8, _) => rm8.either("xor", "xorb"),
            TwoMIInst16(TwoOp::XOR, rm16, _) => rm16.either("xor", "xorw"),
            TwoMIInst32(TwoOp::XOR, rm32, _) => rm32.either("xor", "xorl"),
            TwoMIInst64(TwoOp::XOR, rm64, _) => rm64.either("xor", "xorq"),
            ImulRM16(_, _)
            | ImulRM32(_, _)
            | ImulRM64(_, _)
            | ImulRMI16(_, _, _)
            | ImulRMI32(_, _, _)
            | ImulRMI64(_, _, _) => "imul",
            RotateMI8((RotType::RolRor, RotDir::Left), rm8, _)
            | RotateMC8((RotType::RolRor, RotDir::Left), rm8) => rm8.either("rol", "rolb"),
            RotateMI16((RotType::RolRor, RotDir::Left), rm16, _)
            | RotateMC16((RotType::RolRor, RotDir::Left), rm16) => rm16.either("rol", "rolw"),
            RotateMI32((RotType::RolRor, RotDir::Left), rm32, _)
            | RotateMC32((RotType::RolRor, RotDir::Left), rm32) => rm32.either("rol", "roll"),
            RotateMI64((RotType::RolRor, RotDir::Left), rm64, _)
            | RotateMC64((RotType::RolRor, RotDir::Left), rm64) => rm64.either("rol", "rolq"),
            RotateMI8((RotType::RolRor, RotDir::Right), rm8, _)
            | RotateMC8((RotType::RolRor, RotDir::Right), rm8) => rm8.either("ror", "rorb"),
            RotateMI16((RotType::RolRor, RotDir::Right), rm16, _)
            | RotateMC16((RotType::RolRor, RotDir::Right), rm16) => rm16.either("ror", "rorw"),
            RotateMI32((RotType::RolRor, RotDir::Right), rm32, _)
            | RotateMC32((RotType::RolRor, RotDir::Right), rm32) => rm32.either("ror", "rorl"),
            RotateMI64((RotType::RolRor, RotDir::Right), rm64, _)
            | RotateMC64((RotType::RolRor, RotDir::Right), rm64) => rm64.either("ror", "rorq"),
            RotateMI8((RotType::RclRcr, RotDir::Left), rm8, _)
            | RotateMC8((RotType::RclRcr, RotDir::Left), rm8) => rm8.either("rcl", "rclb"),
            RotateMI16((RotType::RclRcr, RotDir::Left), rm16, _)
            | RotateMC16((RotType::RclRcr, RotDir::Left), rm16) => rm16.either("rcl", "rclw"),
            RotateMI32((RotType::RclRcr, RotDir::Left), rm32, _)
            | RotateMC32((RotType::RclRcr, RotDir::Left), rm32) => rm32.either("rcl", "rcll"),
            RotateMI64((RotType::RclRcr, RotDir::Left), rm64, _)
            | RotateMC64((RotType::RclRcr, RotDir::Left), rm64) => rm64.either("rcl", "rclq"),
            RotateMI8((RotType::RclRcr, RotDir::Right), rm8, _)
            | RotateMC8((RotType::RclRcr, RotDir::Right), rm8) => rm8.either("rcr", "rcrb"),
            RotateMI16((RotType::RclRcr, RotDir::Right), rm16, _)
            | RotateMC16((RotType::RclRcr, RotDir::Right), rm16) => rm16.either("rcr", "rcrw"),
            RotateMI32((RotType::RclRcr, RotDir::Right), rm32, _)
            | RotateMC32((RotType::RclRcr, RotDir::Right), rm32) => rm32.either("rcr", "rcrl"),
            RotateMI64((RotType::RclRcr, RotDir::Right), rm64, _)
            | RotateMC64((RotType::RclRcr, RotDir::Right), rm64) => rm64.either("rcr", "rcrq"),
            BswapO32(_) | BswapO64(_) => "bswap",
            JccJo(_, Normal) => "jo",
            JccJo(_, Negate) => "jno",
            JccJb(_, Normal) => "jb",
            JccJb(_, Negate) => "jae",
            JccJe(_, Normal) => "je",
            JccJe(_, Negate) => "jne",
            JccJbe(_, Normal) => "jbe",
            JccJbe(_, Negate) => "ja",
            JccJs(_, Normal) => "js",
            JccJs(_, Negate) => "jns",
            JccJp(_, Normal) => "jp",
            JccJp(_, Negate) => "jnp",
            JccJl(_, Normal) => "jl",
            JccJl(_, Negate) => "jge",
            JccJle(_, Normal) => "jle",
            JccJle(_, Negate) => "jg",
            Jecxz(_) => "jecxz",
            Jrcxz(_) => "jrcxz",
            JmpD(_) | JmpM64(_) => "jmp",
            CallD(_) => "call",
            PopM16(rm16) => rm16.either("pop", "popw"),
            PopM64(_rm64) => "pop",
            Popf16 => "popfw",
            Popf64 => "popf",
            PushM16(rm16) => rm16.either("push", "pushw"),
            PushM64(_rm64) => "push",
            PushI16(_) => "pushw",
            PushI64(_) => "push",
            Pushf16 => "pushfw",
            Pushf64 => "pushf",
            Scas(_, _) => "scas",
            BtMR16(_, _) => "bt",
            BtMR32(_, _) => "bt",
            BtMR64(_, _) => "bt",
            BtMI16(rm16, _) => rm16.either("bt", "btw"),
            BtMI32(rm32, _) => rm32.either("bt", "btl"),
            BtMI64(rm64, _) => rm64.either("bt", "btq"),
            XchgMR8(_, _) | XchgMR16(_, _) | XchgMR32(_, _) | XchgMR64(_, _) => "xchg",
        }
    }
}

pub struct DisInst {
    /// The prefix is only encoded for disassembly.
    pub prefix: DisassemblyPrefix,
    /// The inner instruction is needed for disassembly and execution.
    pub inner: FullInst,
}
impl fmt::Display for DisInst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let main_inst = &self.inner.main_inst;
        let mnem = main_inst.mnemonic();
        if !mnem.is_empty() {
            let prefix = format!("{}", self.prefix);
            let prefix_plus_mnemonic = if !prefix.is_empty() {
                format!("{} {}", prefix, mnem)
            } else {
                mnem.to_string()
            };
            let operands = main_inst.operands();
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
            | (Some(Base64::GPR64(GPR64::r12)), Some(Index64::Riz), Scale1)
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
                        Base32::GPR32(gpr32) => regs.get_reg32(&gpr32),
                        Base32::Eip => regs.rip as u32,
                    })
                    .unwrap_or(0);
                let scaled_index = match sidb.index {
                    Some(Index32::GPR32(gpr32)) => regs.get_reg32(&gpr32) * (sidb.scale as u32),
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
                        Base64::GPR64(gpr64) => regs.get_reg64(&gpr64),
                        Base64::Rip => regs.rip,
                    })
                    .unwrap_or(0);
                let scale: u8 = sidb.scale.into();
                let scaled_index = match sidb.index {
                    Some(Index64::GPR64(gpr64)) => regs.get_reg64(&gpr64) * (scale as u64),
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
    fn either<'a>(&self, if_reg: &'a str, if_addr: &'a str) -> &'a str {
        match self {
            RM8::Reg(_) => if_reg,
            RM8::Addr(_) => if_addr,
        }
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
    fn either<'a>(&self, if_reg: &'a str, if_addr: &'a str) -> &'a str {
        match self {
            RM16::Reg(_) => if_reg,
            RM16::Addr(_) => if_addr,
        }
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
    fn either<'a>(&self, if_reg: &'a str, if_addr: &'a str) -> &'a str {
        match self {
            RM32::Reg(_) => if_reg,
            RM32::Addr(_) => if_addr,
        }
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
    fn either<'a>(&self, if_reg: &'a str, if_addr: &'a str) -> &'a str {
        match self {
            RM64::Reg(_) => if_reg,
            RM64::Addr(_) => if_addr,
        }
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
