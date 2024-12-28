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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
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

pub struct Registers {
    /// rax, rbx, rcx, rdx, rsi, rdi, rsp, rbp,
    /// r8, r9, r10, r11, r12, r13, r14, r15
    pub regs: [u64; 16],
    /// flags register
    // TODO: consider just tossing this and using a bunch of bits. Would be easier.
    pub rflags: u64,
    /// instruction pointer register
    pub rip: u64,
}

impl Registers {
    /// Set 8 bits in a register without affecting any other bits.
    pub fn set_reg8(&mut self, gpr8: GPR8, imm8: u8) {
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
    pub fn set_reg16(&mut self, gpr16: GPR16, imm16: u16) {
        let ind = gpr16.0.reg_index();
        self.regs[ind] &= !0xFF_FF;
        self.regs[ind] |= imm16 as u64;
    }

    /// Set the low 32 bits of a register without affecting any other bits.
    pub fn set_reg32(&mut self, gpr32: GPR32, imm32: u32) {
        let ind = gpr32.0.reg_index();
        self.regs[ind] &= !0xFF_FF_FF_FF;
        self.regs[ind] |= imm32 as u64;
    }

    /// Set a full register.
    pub fn set_reg64(&mut self, gpr64: GPR64, imm64: u64) {
        let ind = gpr64.0.reg_index();
        self.regs[ind] = imm64;
    }

    pub fn get_reg8(&self, gpr8: GPR8) -> u8 {
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

    pub fn get_reg16(&self, gpr16: GPR16) -> u16 {
        let ind = gpr16.0.reg_index();
        (self.regs[ind] & 0xFF_FF).try_into().unwrap()
    }

    pub fn get_reg32(&self, gpr32: GPR32) -> u32 {
        let ind = gpr32.0.reg_index();
        (self.regs[ind] & 0xFF_FF_FF_FF).try_into().unwrap()
    }

    pub fn get_reg64(&self, gpr64: GPR64) -> u64 {
        let ind = gpr64.0.reg_index();
        self.regs[ind]
    }

    pub fn get_eip(&self) -> u32 {
        (self.rip & 0xFF_FF_FF_FF).try_into().unwrap()
    }

    pub fn get_rip(&self) -> u64 {
        self.rip
    }
}

// Ref https://en.wikipedia.org/wiki/FLAGS_register.
const CF_MASK: u64 = 0x0001;
const PF_MASK: u64 = 0x0004;
const ZF_MASK: u64 = 0x0040;
const SF_MASK: u64 = 0x0080;
const DF_MASK: u64 = 0x0400;
const OF_MASK: u64 = 0x0800;

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
        writeln!(f, "Flags ({:016X}):", self.rflags)?;
        write!(
            f,
            "    Carry     = {}",
            if self.rflags & CF_MASK == 0 {
                "0 (no carry)    "
            } else {
                "1 (carry)       "
            }
        )?;
        writeln!(
            f,
            "   Zero   = {}",
            if self.rflags & ZF_MASK == 0 {
                "0 (isn't zero)"
            } else {
                "1 (is zero)"
            }
        )?;
        write!(
            f,
            "    Overflow  = {}",
            if self.rflags & OF_MASK == 0 {
                "0 (no overflow) "
            } else {
                "1 (overflow)    "
            }
        )?;
        writeln!(
            f,
            "   Sign   = {}",
            if self.rflags & SF_MASK == 0 {
                "0 (positive)"
            } else {
                "1 (negative)"
            }
        )?;
        write!(
            f,
            "    Direction = {}",
            if self.rflags & DF_MASK == 0 {
                "0 (up)          "
            } else {
                "1 (down)        "
            }
        )?;
        writeln!(
            f,
            "   Parity = {}",
            if self.rflags & PF_MASK == 0 {
                "0 (odd)"
            } else {
                "1 (even)"
            }
        )
    }
}
