use std::fmt;

use crate::{
    inst::{RotDir, RotType},
    num_traits::{HasHalfWidth, UNum, UNum8To64},
    num_u1::{
        Sign::{Neg, Pos},
        U1,
    },
};

#[derive(Clone, Copy)]
pub enum ABCDReg {
    Rax,
    Rbx,
    Rcx,
    Rdx,
}
impl ABCDReg {
    pub fn reg_index(&self) -> usize {
        match self {
            ABCDReg::Rax => 0,
            ABCDReg::Rbx => 1,
            ABCDReg::Rcx => 2,
            ABCDReg::Rdx => 3,
        }
    }
}

use QReg::*;
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum QReg {
    Rax,
    Rbx,
    Rcx,
    Rdx,
    Rsi,
    Rdi,
    Rsp,
    Rbp,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
}
impl QReg {
    pub fn reg_index(&self) -> usize {
        match self {
            Rax => 0,
            Rbx => 1,
            Rcx => 2,
            Rdx => 3,
            Rsi => 4,
            Rdi => 5,
            Rsp => 6,
            Rbp => 7,
            R8 => 8,
            R9 => 9,
            R10 => 10,
            R11 => 11,
            R12 => 12,
            R13 => 13,
            R14 => 14,
            R15 => 15,
        }
    }
}

#[derive(Clone, Copy)]
pub enum GPR8 {
    // Low byte (8 bits)
    Low(QReg),
    // High byte of word (8 bits).
    High(ABCDReg),
}
impl GPR8 {
    #![allow(non_upper_case_globals)]
    pub const al: GPR8 = GPR8::Low(Rax);
    pub const bl: GPR8 = GPR8::Low(Rbx);
    pub const cl: GPR8 = GPR8::Low(Rcx);
    pub const dl: GPR8 = GPR8::Low(Rdx);
    pub const ah: GPR8 = GPR8::High(ABCDReg::Rax);
    pub const bh: GPR8 = GPR8::High(ABCDReg::Rbx);
    pub const ch: GPR8 = GPR8::High(ABCDReg::Rcx);
    pub const dh: GPR8 = GPR8::High(ABCDReg::Rdx);
    pub const sil: GPR8 = GPR8::Low(Rsi);
    pub const dil: GPR8 = GPR8::Low(Rdi);
    pub const spl: GPR8 = GPR8::Low(Rsp);
    pub const bpl: GPR8 = GPR8::Low(Rbp);
    pub const r8b: GPR8 = GPR8::Low(R8);
    pub const r9b: GPR8 = GPR8::Low(R9);
    pub const r10b: GPR8 = GPR8::Low(R10);
    pub const r11b: GPR8 = GPR8::Low(R11);
    pub const r12b: GPR8 = GPR8::Low(R12);
    pub const r13b: GPR8 = GPR8::Low(R13);
    pub const r14b: GPR8 = GPR8::Low(R14);
    pub const r15b: GPR8 = GPR8::Low(R15);
}
impl fmt::Display for GPR8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GPR8::Low(Rax) => "%al",
            GPR8::Low(Rbx) => "%bl",
            GPR8::Low(Rcx) => "%cl",
            GPR8::Low(Rdx) => "%dl",
            GPR8::Low(Rsi) => "%sil",
            GPR8::Low(Rdi) => "%dil",
            GPR8::Low(Rsp) => "%spl",
            GPR8::Low(Rbp) => "%bpl",
            GPR8::Low(R8) => "%r8b",
            GPR8::Low(R9) => "%r9b",
            GPR8::Low(R10) => "%r10b",
            GPR8::Low(R11) => "%r11b",
            GPR8::Low(R12) => "%r12b",
            GPR8::Low(R13) => "%r13b",
            GPR8::Low(R14) => "%r14b",
            GPR8::Low(R15) => "%r15b",
            GPR8::High(ABCDReg::Rax) => "%ah",
            GPR8::High(ABCDReg::Rbx) => "%bh",
            GPR8::High(ABCDReg::Rcx) => "%ch",
            GPR8::High(ABCDReg::Rdx) => "%dh",
        };
        write!(f, "{}", s)
    }
}

// Word (16 bits)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GPR16(pub QReg);

impl fmt::Display for GPR16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            Rax => "%ax",
            Rbx => "%bx",
            Rcx => "%cx",
            Rdx => "%dx",
            Rsi => "%si",
            Rdi => "%di",
            Rsp => "%sp",
            Rbp => "%bp",
            R8 => "%r8w",
            R9 => "%r9w",
            R10 => "%r10w",
            R11 => "%r11w",
            R12 => "%r12w",
            R13 => "%r13w",
            R14 => "%r14w",
            R15 => "%r15w",
        };
        write!(f, "{}", s)
    }
}
impl GPR16 {
    #![allow(non_upper_case_globals)]
    pub const ax: GPR16 = GPR16(Rax);
    pub const bx: GPR16 = GPR16(Rbx);
    pub const cx: GPR16 = GPR16(Rcx);
    pub const dx: GPR16 = GPR16(Rdx);
    pub const si: GPR16 = GPR16(Rsi);
    pub const di: GPR16 = GPR16(Rdi);
    pub const sp: GPR16 = GPR16(Rsp);
    pub const bp: GPR16 = GPR16(Rbp);
    pub const r8w: GPR16 = GPR16(R8);
    pub const r9w: GPR16 = GPR16(R9);
    pub const r10w: GPR16 = GPR16(R10);
    pub const r11w: GPR16 = GPR16(R11);
    pub const r12w: GPR16 = GPR16(R12);
    pub const r13w: GPR16 = GPR16(R13);
    pub const r14w: GPR16 = GPR16(R14);
    pub const r15w: GPR16 = GPR16(R15);
}

/// Double word (32 bits)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GPR32(pub QReg);
impl GPR32 {
    #![allow(non_upper_case_globals)]
    pub const eax: GPR32 = GPR32(Rax);
    pub const ebx: GPR32 = GPR32(Rbx);
    pub const ecx: GPR32 = GPR32(Rcx);
    pub const edx: GPR32 = GPR32(Rdx);
    pub const esi: GPR32 = GPR32(Rsi);
    pub const edi: GPR32 = GPR32(Rdi);
    pub const esp: GPR32 = GPR32(Rsp);
    pub const ebp: GPR32 = GPR32(Rbp);
    pub const r8d: GPR32 = GPR32(R8);
    pub const r9d: GPR32 = GPR32(R9);
    pub const r10d: GPR32 = GPR32(R10);
    pub const r11d: GPR32 = GPR32(R11);
    pub const r12d: GPR32 = GPR32(R12);
    pub const r13d: GPR32 = GPR32(R13);
    pub const r14d: GPR32 = GPR32(R14);
    pub const r15d: GPR32 = GPR32(R15);
}
impl fmt::Display for GPR32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            Rax => "%eax",
            Rbx => "%ebx",
            Rcx => "%ecx",
            Rdx => "%edx",
            Rsi => "%esi",
            Rdi => "%edi",
            Rsp => "%esp",
            Rbp => "%ebp",
            R8 => "%r8d",
            R9 => "%r9d",
            R10 => "%r10d",
            R11 => "%r11d",
            R12 => "%r12d",
            R13 => "%r13d",
            R14 => "%r14d",
            R15 => "%r15d",
        };
        write!(f, "{}", s)
    }
}

/// Quadword (64 bits)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GPR64(pub QReg);
impl GPR64 {
    #![allow(non_upper_case_globals)]
    pub const rax: GPR64 = GPR64(Rax);
    pub const rbx: GPR64 = GPR64(Rbx);
    pub const rcx: GPR64 = GPR64(Rcx);
    pub const rdx: GPR64 = GPR64(Rdx);
    pub const rsi: GPR64 = GPR64(Rsi);
    pub const rdi: GPR64 = GPR64(Rdi);
    pub const rsp: GPR64 = GPR64(Rsp);
    pub const rbp: GPR64 = GPR64(Rbp);
    pub const r8: GPR64 = GPR64(R8);
    pub const r9: GPR64 = GPR64(R9);
    pub const r10: GPR64 = GPR64(R10);
    pub const r11: GPR64 = GPR64(R11);
    pub const r12: GPR64 = GPR64(R12);
    pub const r13: GPR64 = GPR64(R13);
    pub const r14: GPR64 = GPR64(R14);
    pub const r15: GPR64 = GPR64(R15);
}
impl fmt::Display for GPR64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self.0 {
            Rax => "%rax",
            Rbx => "%rbx",
            Rcx => "%rcx",
            Rdx => "%rdx",
            Rsi => "%rsi",
            Rdi => "%rdi",
            Rsp => "%rsp",
            Rbp => "%rbp",
            R8 => "%r8",
            R9 => "%r9",
            R10 => "%r10",
            R11 => "%r11",
            R12 => "%r12",
            R13 => "%r13",
            R14 => "%r14",
            R15 => "%r15",
        };
        write!(f, "{}", s)
    }
}

/// Vol 1: 3.4.3.1 "Status Flags" and 3.4.3.2 "DF Flag".
pub struct Flags {
    /// CF (bit 0): Carry Flag (carry or borrow out of the most-significant bit).
    /// Also used in multiple-precision arithmetic.
    /// Can be modified directly (STC, CLC, CMC) or copied into (BT, BTS, BTR, BTC).
    pub(crate) cf: bool,
    /// PF (bit 2): Parity Flag (least-significant byte (u8) contains an even number of 1 bits).
    pub(crate) pf: bool,
    /// AF (bit 4): Auxiliary Carry Flag (carry or borrow out of bit 3 of the result, for BCD).
    pub(crate) af: bool,
    /// ZF (bit 6): Zero Flag (result is zero)
    pub(crate) zf: bool,
    /// SF (bit 7): Sign Flag (most-significant bit of the result)
    pub(crate) sf: bool,
    /// OF (bit 11): Overflow flag (overflow for two's complement arithmetic)
    pub(crate) of: bool,
    /// DF (bit 10): Direction flag: Controls string instructions (MOVS, CMPS, SCS, LODS, STOS).
    /// When true, auto-decrement (high addresses to low addresses).
    /// WHen false, auto-increment (low addresses to high addresses).
    pub(crate) df: bool,
}

impl Flags {
    pub(crate) fn new() -> Flags {
        Flags {
            cf: false,
            pf: false,
            af: false,
            zf: false,
            sf: false,
            of: false,
            df: false,
        }
    }

    pub(crate) fn to_rflags_u64(&self) -> u64 {
        0x202
            | if self.cf { 1 } else { 0 }
            | if self.pf { 1 << 2 } else { 0 }
            | if self.af { 1 << 4 } else { 0 }
            | if self.zf { 1 << 6 } else { 0 }
            | if self.sf { 1 << 7 } else { 0 }
            | if self.df { 1 << 10 } else { 0 }
            | if self.of { 1 << 11 } else { 0 }
    }

    pub(crate) fn to_rflags_u64_with_rf(&self) -> u64 {
        // TODO: see Vol 3B 19.3.1.1 "Instruction-Breakpoint Exception Condition"
        // for how the resume flag (RF, bit 16) works. For now, we just
        // leave it cleared except on the segfault register printing.
        self.to_rflags_u64() | 0x10000
    }

    /// Set the lower 16 bits of RFLAGS.
    pub(crate) fn set_rflags16(&mut self, val: u16) {
        // TODO: check out the bits other than the 7 flags we treat special
        self.cf = val.bit(0).into();
        self.pf = val.bit(2).into();
        self.af = val.bit(4).into();
        self.zf = val.bit(6).into();
        self.sf = val.bit(7).into();
        self.df = val.bit(10).into();
        self.of = val.bit(11).into();
    }

    pub(crate) fn set_rflags64(&mut self, val: u64) {
        // TODO: check out the bits other than the 7 flags we treat special
        self.set_rflags16(val as u16);
    }

    fn result_flags<U: UNum>(&mut self, result: U) {
        self.pf = result.as_u8().count_ones() % 2 == 0;
        self.zf = result == 0.into();
        self.sf = result.msb_bit().into();
    }

    /// Bitwise AND two numbers, update flags, and return the result.
    pub(crate) fn and<U: UNum>(&mut self, x: U, y: U) -> U {
        let result = x & y;
        self.result_flags(result);
        self.cf = false;
        self.of = false;
        // Behavior of AF is undefined; false is following code.golf CPU.
        self.af = false;
        result
    }

    /// Bitwise OR two numbers, update flags, and return the result.
    pub(crate) fn or<U: UNum>(&mut self, x: U, y: U) -> U {
        let result = x | y;
        self.result_flags(result);
        self.cf = false;
        self.of = false;
        // Behavior of AF is undefined; false is following code.golf CPU.
        self.af = false;
        result
    }

    /// Increment x, update flags except CF, and return the incremented value.
    pub(crate) fn inc<U: UNum>(&mut self, x: U) -> U {
        let cf = self.cf;
        let ret = self.add(x, 1.into());
        self.cf = cf;
        ret
    }

    /// Add two numbers, update flags, and return the sum.
    pub(crate) fn add<U: UNum>(&mut self, x: U, y: U) -> U {
        self.cf = false;
        self.adc(x, y)
    }

    /// Add two numbers and CF, update flags, and return the sum.
    pub(crate) fn adc<U: UNum>(&mut self, x: U, y: U) -> U {
        let (x1, carry1) = x.overflowing_add(&U1::from(self.cf).as_u8().into());
        let (result, carry2) = x1.overflowing_add(&y);
        self.result_flags(result);
        self.cf = carry1 | carry2;
        // bit 4:
        // no carry into 4:
        // 0 + 0 = 0
        // 0 + 1 = 1
        // 1 + 0 = 1
        // 1 + 1 = 0 (carry out of 4, into 5)
        // carry into 4:
        // 0 + 0 = 1
        // 0 + 1 = 0 (carry out of 4, into 5)
        // 1 + 0 = 0 (carry out of 4, into 5)
        // 1 + 1 = 1 (carry out of 4, into 5)
        let carry_out_of_bit_3 = x.bit(4) ^ y.bit(4) ^ result.bit(4);
        self.af = carry_out_of_bit_3.into();
        self.of = match (x.sign(), y.sign(), result.sign()) {
            // pos + pos = neg
            (Pos, Pos, Neg) => true,
            // neg + neg = pos
            (Neg, Neg, Pos) => true,
            _ => false,
        };
        result
    }

    /// Decrement x, update flags except CF, and return the decremented value.
    pub(crate) fn dec<U: UNum>(&mut self, x: U) -> U {
        let cf = self.cf;
        let ret = self.sub(x, 1.into());
        self.cf = cf;
        ret
    }

    /// Subtract y from x, update flags, and return the difference.
    pub(crate) fn sub<U: UNum>(&mut self, x: U, y: U) -> U {
        self.cf = false;
        self.sbb(x, y)
    }

    /// Subtract y plus a carry bit from x, update flags, and return the difference.
    /// Note this isn't just `self.add(x, -y)` because CF/OF/AF need
    /// to be different, and negating U::MIN overflows.
    pub(crate) fn sbb<U: UNum>(&mut self, x: U, y: U) -> U {
        let (x1, carry1) = x.overflowing_sub(&U1::from(self.cf).as_u8().into());
        let (result, carry2) = x1.overflowing_sub(&y);
        self.result_flags(result);
        self.cf = carry1 | carry2;
        // bit 4:
        // no borrow into 3:
        // 0 - 0 = 0 (no borrow into 4)
        // 1 - 0 = 1 (no borrow into 4)
        // 1 - 1 = 0 (no borrow into 4)
        // 0 - 1 = 1 (borrow into 4)
        // borrow into 3:
        // 0 - 0 = 1 (borrow into 4)
        // 1 - 1 = 1 (borrow into 4)
        // 0 - 1 = 0 (borrow into 4)
        // 1 - 0 = 0 (no borrow into 4)
        let borrow_into_bit_3 = x.bit(4) ^ y.bit(4) ^ result.bit(4);
        self.af = borrow_into_bit_3.into();
        self.of = match (x.sign(), y.sign(), result.sign()) {
            // pos - neg = neg
            (Pos, Neg, Neg) => true,
            // neg - pos = pos
            (Neg, Pos, Pos) => true,
            _ => false,
        };
        result
    }

    /// XOR two numbers, update flags, and return the XOR.
    pub(crate) fn xor<U: UNum>(&mut self, x: U, y: U) -> U {
        let result = x.bitxor(y);
        self.result_flags(result);
        // Documented: CF and OF cleared, with AF flag undefined.
        // AF flag seems to be cleared on code.golf CPU.
        self.cf = false;
        self.of = false;
        self.af = false;
        result
    }

    pub(crate) fn imul<U: UNum8To64>(&mut self, x: U, y: U) -> U::DoubleWidth {
        let prod = x.widening_signed_mul(&y);
        let cf_of = prod.sign() != prod.truncate_to_half_width().sign();
        self.cf = cf_of;
        self.of = cf_of;
        prod
    }

    pub(crate) fn div_flags(&mut self) {
        // The div and idiv instructions are documented to have undefined effect on flags.
        // On the code.golf CPU, they clear PF, ZF, SF, set AF and preserve CF, OF and other flags
        self.pf = false;
        self.zf = false;
        self.sf = false;
        self.af = true;
    }

    pub(crate) fn rotate<U: UNum>(&mut self, rot_pair: &(RotType, RotDir), val: U, imm8: u8) -> U {
        let count_mask = 0x3F;
        let masked_count = imm8 & count_mask;
        match rot_pair {
            (RotType::RolRor, RotDir::Left) => {
                let modded_count = masked_count % 64;
                let new_val = val.rotate_left(modded_count as u32);
                if masked_count != 0 {
                    self.cf = new_val.bit(0).into();
                }
                if masked_count == 1 {
                    self.of = new_val.msb_bit() != val.msb_bit();
                } else {
                    // spec says this is undefined. The code.golf CPU seems to clear it.
                    self.of = false;
                }
                new_val
            }
            (RotType::RolRor, RotDir::Right) => todo!("rotate"),
            (RotType::RclRcr, RotDir::Left) => todo!("rotate"),
            (RotType::RclRcr, RotDir::Right) => todo!("rotate"),
        }
    }
}

pub struct Registers {
    /// rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp,
    /// r8, r9, r10, r11, r12, r13, r14, r15
    pub regs: [u64; 16],
    /// flags register
    pub flags: Flags,
    /// Instruction pointer to the current instruction.
    /// Avoids including the u64 in the encoding of the EffAddr.
    pub rip_prev: u64,
    /// Instruction pointer. During execution of an instruction,
    /// points to the subsequent instruction.
    pub rip: u64,
}

impl Registers {
    /// Set 8 bits in a register without affecting any other bits.
    pub fn set_reg8(&mut self, gpr8: &GPR8, imm8: u8) {
        match gpr8 {
            GPR8::Low(qreg) => {
                let ind = qreg.reg_index();
                self.regs[ind] &= !0xFF;
                self.regs[ind] |= imm8 as u64;
            }
            GPR8::High(abcdreg) => {
                let ind = abcdreg.reg_index();
                self.regs[ind] &= !0xFF_00;
                self.regs[ind] |= (imm8 as u64) << 8;
            }
        }
    }

    /// Set the low 16 bits of a register without affecting any other bits.
    pub fn set_reg16(&mut self, gpr16: &GPR16, imm16: u16) {
        let ind = gpr16.0.reg_index();
        self.regs[ind] &= !0xFF_FF;
        self.regs[ind] |= imm16 as u64;
    }

    /// Set the low 32 bits of a register, and clear the upper bits.
    pub fn set_reg32(&mut self, gpr32: &GPR32, imm32: u32) {
        let ind = gpr32.0.reg_index();
        self.regs[ind] = imm32 as u64;
    }

    /// Set a full register.
    pub fn set_reg64(&mut self, gpr64: &GPR64, imm64: u64) {
        let ind = gpr64.0.reg_index();
        self.regs[ind] = imm64;
    }

    pub fn get_reg8(&self, gpr8: &GPR8) -> u8 {
        match gpr8 {
            GPR8::Low(qreg) => {
                let ind = qreg.reg_index();
                (self.regs[ind] & 0xFF).try_into().unwrap()
            }
            GPR8::High(abcdreg) => {
                let ind = abcdreg.reg_index();
                (self.regs[ind] >> 8 & 0xFF).try_into().unwrap()
            }
        }
    }

    pub fn get_reg16(&self, gpr16: &GPR16) -> u16 {
        let ind = gpr16.0.reg_index();
        self.regs[ind] as u16
    }

    pub fn get_reg32(&self, gpr32: &GPR32) -> u32 {
        let ind = gpr32.0.reg_index();
        self.regs[ind] as u32
    }

    pub fn get_reg64(&self, gpr64: &GPR64) -> u64 {
        let ind = gpr64.0.reg_index();
        self.regs[ind]
    }

    pub fn get_al(&self) -> u8 {
        self.get_reg8(&GPR8::al)
    }

    pub fn get_ax(&self) -> u16 {
        self.get_reg16(&GPR16::ax)
    }

    pub fn get_eax(&self) -> u32 {
        self.get_reg32(&GPR32::eax)
    }

    pub fn get_rax(&self) -> u64 {
        self.get_reg64(&GPR64::rax)
    }

    pub fn get_dx_ax(&self) -> u32 {
        (self.get_reg16(&GPR16::dx) as u32) << 16 | (self.get_reg16(&GPR16::ax) as u32)
    }

    pub fn get_edx_eax(&self) -> u64 {
        (self.get_reg32(&GPR32::edx) as u64) << 32 | (self.get_reg32(&GPR32::eax) as u64)
    }

    pub fn get_rdx_rax(&self) -> u128 {
        (self.get_reg64(&GPR64::rdx) as u128) << 64 | (self.get_reg64(&GPR64::rax) as u128)
    }

    pub fn set_ax(&mut self, val: u16) {
        self.set_reg16(&GPR16::ax, val);
    }

    pub fn set_eax(&mut self, val: u32) {
        self.set_reg32(&GPR32::eax, val);
    }

    pub fn set_rax(&mut self, val: u64) {
        self.set_reg64(&GPR64::rax, val);
    }

    pub fn set_dx_ax(&mut self, val: u32) {
        self.set_reg16(&GPR16::dx, (val >> 16) as u16);
        self.set_reg16(&GPR16::ax, val as u16);
    }

    pub fn set_edx_eax(&mut self, val: u64) {
        self.set_reg32(&GPR32::edx, (val >> 32) as u32);
        self.set_reg32(&GPR32::eax, val as u32);
    }

    pub fn set_rdx_rax(&mut self, val: u128) {
        self.set_reg64(&GPR64::rdx, (val >> 64) as u64);
        self.set_reg64(&GPR64::rax, val as u64);
    }

    pub fn get_rflags(&self) -> u64 {
        self.flags.to_rflags_u64()
    }

    pub(crate) fn offset_reg32_by_df(&mut self, gpr32: &GPR32, scale: u32) {
        let val = self.get_reg32(gpr32);
        let new_val = match self.flags.df {
            false => val.wrapping_add(scale),
            true => val.wrapping_sub(scale),
        };
        self.set_reg32(gpr32, new_val);
    }

    pub(crate) fn offset_reg64_by_df(&mut self, gpr64: &GPR64, scale: u64) {
        let val = self.get_reg64(gpr64);
        let new_val = match self.flags.df {
            false => val.wrapping_add(scale),
            true => val.wrapping_sub(scale),
        };
        self.set_reg64(gpr64, new_val);
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "(%rip was {:016X})", self.rip_prev)?;
        writeln!(f, "Registers:")?;
        writeln!(
            f,
            "    %rax = {:016X}        %r8  = {:016X}",
            self.regs[0], self.regs[8]
        )?;
        writeln!(
            f,
            "    %rbx = {:016X}        %r9  = {:016X}",
            self.regs[1], self.regs[9]
        )?;
        writeln!(
            f,
            "    %rcx = {:016X}        %r10 = {:016X}",
            self.regs[2], self.regs[10]
        )?;
        writeln!(
            f,
            "    %rdx = {:016X}        %r11 = {:016X}",
            self.regs[3], self.regs[11]
        )?;
        writeln!(
            f,
            "    %rsi = {:016X}        %r12 = {:016X}",
            self.regs[4], self.regs[12]
        )?;
        writeln!(
            f,
            "    %rdi = {:016X}        %r13 = {:016X}",
            self.regs[5], self.regs[13]
        )?;
        writeln!(
            f,
            "    %rsp = {:016X}        %r14 = {:016X}",
            self.regs[6], self.regs[14]
        )?;
        writeln!(
            f,
            "    %rbp = {:016X}        %r15 = {:016X}",
            self.regs[7], self.regs[15]
        )?;
        writeln!(f, "Flags ({:016X}):", self.flags.to_rflags_u64_with_rf())?;
        write!(
            f,
            "    Carry     = {}",
            if !self.flags.cf {
                "0 (no carry)    "
            } else {
                "1 (carry)       "
            }
        )?;
        writeln!(
            f,
            "   Zero   = {}",
            if !self.flags.zf {
                "0 (isn't zero)"
            } else {
                "1 (is zero)"
            }
        )?;
        write!(
            f,
            "    Overflow  = {}",
            if !self.flags.of {
                "0 (no overflow) "
            } else {
                "1 (overflow)    "
            }
        )?;
        writeln!(
            f,
            "   Sign   = {}",
            if !self.flags.sf {
                "0 (positive)"
            } else {
                "1 (negative)"
            }
        )?;
        write!(
            f,
            "    Direction = {}",
            if !self.flags.df {
                "0 (up)          "
            } else {
                "1 (down)        "
            }
        )?;
        write!(
            f,
            "   Parity = {}",
            if !self.flags.pf {
                "0 (odd)"
            } else {
                "1 (even)"
            }
        )
    }
}
