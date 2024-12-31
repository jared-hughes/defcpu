use std::fmt;

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
        0x0000000000010202
            | if self.cf { 1 } else { 0 }
            | if self.pf { 1 << 2 } else { 0 }
            | if self.af { 1 << 4 } else { 0 }
            | if self.zf { 1 << 6 } else { 0 }
            | if self.sf { 1 << 7 } else { 0 }
            | if self.df { 1 << 10 } else { 0 }
            | if self.of { 1 << 11 } else { 0 }
    }

    pub(crate) fn add_8(&mut self, x: u8, y: u8, update_cf: bool) -> u8 {
        let (result, carry) = x.overflowing_add(y);
        let carry_out_of_bit_3 = (result >> 3 & 1) ^ (x >> 3 & 1) ^ (y >> 3 & 1);
        if update_cf {
            self.cf = carry;
        }
        self.pf = result.count_ones() % 2 == 0;
        self.af = carry_out_of_bit_3 == 1;
        self.zf = result == 0;
        let bm1 = u8::BITS - 1;
        self.sf = result >> bm1 & 1 == 1;
        self.of = match (x >> bm1 & 1, y >> bm1 & 1, result >> bm1 & 1) {
            // pos + pos = neg
            (0, 0, 1) => true,
            // neg + neg = pos
            (1, 1, 0) => true,
            _ => false,
        };
        result
    }

    pub(crate) fn add_16(&mut self, x: u16, y: u16, update_cf: bool) -> u16 {
        let (result, carry) = x.overflowing_add(y);
        let carry_out_of_bit_3 = (result >> 3 & 1) ^ (x >> 3 & 1) ^ (y >> 3 & 1);
        if update_cf {
            self.cf = carry;
        }
        self.pf = (result as u8).count_ones() % 2 == 0;
        self.af = carry_out_of_bit_3 == 1;
        self.zf = result == 0;
        let bm1 = u16::BITS - 1;
        self.sf = result >> bm1 & 1 == 1;
        self.of = match (x >> bm1 & 1, y >> bm1 & 1, result >> bm1 & 1) {
            // pos + pos = neg
            (0, 0, 1) => true,
            // neg + neg = pos
            (1, 1, 0) => true,
            _ => false,
        };
        result
    }

    pub(crate) fn add_32(&mut self, x: u32, y: u32, update_cf: bool) -> u32 {
        let (result, carry) = x.overflowing_add(y);
        let carry_out_of_bit_3 = (result >> 3 & 1) ^ (x >> 3 & 1) ^ (y >> 3 & 1);
        if update_cf {
            self.cf = carry;
        }
        self.pf = (result as u8).count_ones() % 2 == 0;
        self.af = carry_out_of_bit_3 == 1;
        self.zf = result == 0;
        let bm1 = u32::BITS - 1;
        self.sf = result >> bm1 & 1 == 1;
        self.of = match (x >> bm1 & 1, y >> bm1 & 1, result >> bm1 & 1) {
            // pos + pos = neg
            (0, 0, 1) => true,
            // neg + neg = pos
            (1, 1, 0) => true,
            _ => false,
        };
        result
    }

    pub(crate) fn add_64(&mut self, x: u64, y: u64, update_cf: bool) -> u64 {
        let (result, carry) = x.overflowing_add(y);
        let carry_out_of_bit_3 = (result >> 3 & 1) ^ (x >> 3 & 1) ^ (y >> 3 & 1);
        if update_cf {
            self.cf = carry;
        }
        self.pf = (result as u8).count_ones() % 2 == 0;
        self.af = carry_out_of_bit_3 == 1;
        self.zf = result == 0;
        let bm1 = u64::BITS - 1;
        self.sf = result >> bm1 & 1 == 1;
        self.of = match (x >> bm1 & 1, y >> bm1 & 1, result >> bm1 & 1) {
            // pos + pos = neg
            (0, 0, 1) => true,
            // neg + neg = pos
            (1, 1, 0) => true,
            _ => false,
        };
        result
    }

    pub(crate) fn div_flags(&mut self) {
        // The div and idiv instructions are documented to have undefined effect on flags.
        // On the code.golf CPU, they clear PF, ZF, SF, set AF and preserve CF, OF and other flags
        self.pf = false;
        self.zf = false;
        self.sf = false;
        self.af = true;
    }
}

pub struct Registers {
    /// rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp,
    /// r8, r9, r10, r11, r12, r13, r14, r15
    pub regs: [u64; 16],
    /// flags register
    pub flags: Flags,
    /// instruction pointer register
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

    /// Set the low 32 bits of a register without affecting any other bits.
    pub fn set_reg32(&mut self, gpr32: &GPR32, imm32: u32) {
        let ind = gpr32.0.reg_index();
        self.regs[ind] &= !0xFF_FF_FF_FF;
        self.regs[ind] |= imm32 as u64;
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
        (self.regs[ind] & 0xFF_FF).try_into().unwrap()
    }

    pub fn get_reg32(&self, gpr32: &GPR32) -> u32 {
        let ind = gpr32.0.reg_index();
        (self.regs[ind] & 0xFF_FF_FF_FF).try_into().unwrap()
    }

    pub fn get_reg64(&self, gpr64: &GPR64) -> u64 {
        let ind = gpr64.0.reg_index();
        self.regs[ind]
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

    pub fn get_eip(&self) -> u32 {
        (self.rip & 0xFF_FF_FF_FF).try_into().unwrap()
    }

    pub fn get_rip(&self) -> u64 {
        self.rip
    }

    pub fn get_rflags(&self) -> u64 {
        self.flags.to_rflags_u64()
    }
}

impl fmt::Display for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "(%rip was {:016X})", self.rip)?;
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
        writeln!(f, "Flags ({:016X}):", self.flags.to_rflags_u64())?;
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
