use crate::{
    decode_inst::decode_inst,
    inst::{Inst, RM16, RM32, RM64, RM8},
    memory::Memory,
    parse_elf::SimpleElfFile,
    read_write::Writers,
    registers::{Flags, Registers, GPR16, GPR32, GPR64, GPR8},
};

pub struct Machine {
    pub regs: Registers,
    pub mem: Memory,
    pub halt: bool,
}

impl Machine {
    pub fn from_elf(file: &SimpleElfFile) -> Machine {
        let mem = Memory::from_segments(&file.segments);
        let regs = Registers {
            regs: [0_u64; 16],
            flags: Flags::new(),
            rip_prev: file.e_entry,
            rip: file.e_entry,
        };
        Machine {
            regs,
            mem,
            halt: false,
        }
    }

    pub fn step(&mut self, writers: &mut Writers) {
        if self.halt {
            panic!("Unexpected step in a halt state.")
        }
        let (inst, len) = decode_inst(&self.mem, self.regs.rip);
        self.regs.rip_prev = self.regs.rip;
        self.regs.rip += len;
        self.run_inst(inst.inner, writers);
    }

    /// Returned value is Some(u64) if rip should jump to that u64.
    pub fn run_inst(&mut self, inst: Inst, writers: &mut Writers) {
        match inst {
            Inst::NotImplemented(opcode) => {
                panic!("Not yet implemented opcode {opcode:02x}.")
            }
            Inst::NotImplemented2(opcode, opcode2) => {
                panic!("Not yet implemented opcode {opcode:02x} {opcode2:02x}.")
            }
            Inst::NotImplementedOpext(opcode, sub) => {
                panic!("Not yet implemented opcode {opcode:02x} /{sub}.")
            }
            Inst::RexNoop => {}
            Inst::Syscall => self.syscall(writers),
            Inst::MovMR8(rm8, gpr8) => {
                let val = self.regs.get_reg8(&gpr8);
                self.set_rm8(&rm8, val);
            }
            Inst::MovMR16(rm16, gpr16) => {
                let val = self.regs.get_reg16(&gpr16);
                self.set_rm16(&rm16, val);
            }
            Inst::MovMR32(rm32, gpr32) => {
                let val = self.regs.get_reg32(&gpr32);
                self.set_rm32(&rm32, val);
            }
            Inst::MovMR64(rm64, gpr64) => {
                let val = self.regs.get_reg64(&gpr64);
                self.set_rm64(&rm64, val);
            }
            Inst::MovRM8(gpr8, rm8) => {
                let val = self.get_rm8(&rm8);
                self.regs.set_reg8(&gpr8, val);
            }
            Inst::MovRM16(gpr16, rm16) => {
                let val = self.get_rm16(&rm16);
                self.regs.set_reg16(&gpr16, val);
            }
            Inst::MovRM32(gpr32, rm32) => {
                let val = self.get_rm32(&rm32);
                self.regs.set_reg32(&gpr32, val);
            }
            Inst::MovRM64(gpr64, rm64) => {
                let val = self.get_rm64(&rm64);
                self.regs.set_reg64(&gpr64, val);
            }
            Inst::MovOI8(gpr8, imm8) => {
                self.regs.set_reg8(&gpr8, imm8);
            }
            Inst::MovOI16(gpr16, imm16) => {
                self.regs.set_reg16(&gpr16, imm16);
            }
            Inst::MovOI32(gpr32, imm32) => {
                self.regs.set_reg32(&gpr32, imm32);
            }
            Inst::MovOI64(gpr64, imm64) => {
                self.regs.set_reg64(&gpr64, imm64);
            }
            Inst::MovMI8(rm8, imm8) => {
                self.set_rm8(&rm8, imm8);
            }
            Inst::MovMI16(rm16, imm16) => {
                self.set_rm16(&rm16, imm16);
            }
            Inst::MovMI32(rm32, imm32) => {
                self.set_rm32(&rm32, imm32);
            }
            Inst::MovMI64(rm64, imm64) => {
                self.set_rm64(&rm64, imm64);
            }
            Inst::Hlt => {
                write!(writers.stderr(), "{}", self.regs).expect("Write should succeed.");
                self.halt = true;
            }
            Inst::IncM8(rm8) => {
                let old = self.get_rm8(&rm8);
                let new = self.regs.flags.add_8(old, 1, false);
                self.set_rm8(&rm8, new);
            }
            Inst::IncM16(rm16) => {
                let old = self.get_rm16(&rm16);
                let new = self.regs.flags.add_16(old, 1, false);
                self.set_rm16(&rm16, new);
            }
            Inst::IncM32(rm32) => {
                let old = self.get_rm32(&rm32);
                let new = self.regs.flags.add_32(old, 1, false);
                self.set_rm32(&rm32, new);
            }
            Inst::IncM64(rm64) => {
                let old = self.get_rm64(&rm64);
                let new = self.regs.flags.add_64(old, 1, false);
                self.set_rm64(&rm64, new);
            }
            Inst::DecM8(rm8) => {
                let old = self.get_rm8(&rm8);
                let new = self.regs.flags.sub_8(old, 1, false);
                self.set_rm8(&rm8, new);
            }
            Inst::DecM16(rm16) => {
                let old = self.get_rm16(&rm16);
                let new = self.regs.flags.sub_16(old, 1, false);
                self.set_rm16(&rm16, new);
            }
            Inst::DecM32(rm32) => {
                let old = self.get_rm32(&rm32);
                let new = self.regs.flags.sub_32(old, 1, false);
                self.set_rm32(&rm32, new);
            }
            Inst::DecM64(rm64) => {
                let old = self.get_rm64(&rm64);
                let new = self.regs.flags.sub_64(old, 1, false);
                self.set_rm64(&rm64, new);
            }
            Inst::AddMI8(rm8, imm8) => {
                let old = self.get_rm8(&rm8);
                let new = self.regs.flags.add_8(old, imm8, true);
                self.set_rm8(&rm8, new);
            }
            Inst::AddMI16(rm16, imm16) => {
                let old = self.get_rm16(&rm16);
                let new = self.regs.flags.add_16(old, imm16, true);
                self.set_rm16(&rm16, new);
            }
            Inst::AddMI32(rm32, imm32) => {
                let old = self.get_rm32(&rm32);
                let new = self.regs.flags.add_32(old, imm32, true);
                self.set_rm32(&rm32, new);
            }
            Inst::AddMI64(rm64, imm64) => {
                let old = self.get_rm64(&rm64);
                let new = self.regs.flags.add_64(old, imm64, true);
                self.set_rm64(&rm64, new);
            }
            Inst::AddMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(&gpr8);
                let old_dest = self.get_rm8(&rm8);
                let new = self.regs.flags.add_8(old_dest, source, true);
                self.set_rm8(&rm8, new);
            }
            Inst::AddMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(&gpr16);
                let old_dest = self.get_rm16(&rm16);
                let new = self.regs.flags.add_16(old_dest, source, true);
                self.set_rm16(&rm16, new);
            }
            Inst::AddMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(&gpr32);
                let old_dest = self.get_rm32(&rm32);
                let new = self.regs.flags.add_32(old_dest, source, true);
                self.set_rm32(&rm32, new);
            }
            Inst::AddMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(&gpr64);
                let old_dest = self.get_rm64(&rm64);
                let new = self.regs.flags.add_64(old_dest, source, true);
                self.set_rm64(&rm64, new);
            }
            Inst::AddRM8(gpr8, rm8) => {
                let source = self.get_rm8(&rm8);
                let old_dest = self.regs.get_reg8(&gpr8);
                let new = self.regs.flags.add_8(old_dest, source, true);
                self.regs.set_reg8(&gpr8, new);
            }
            Inst::AddRM16(gpr16, rm16) => {
                let source = self.get_rm16(&rm16);
                let old_dest = self.regs.get_reg16(&gpr16);
                let new = self.regs.flags.add_16(old_dest, source, true);
                self.regs.set_reg16(&gpr16, new);
            }
            Inst::AddRM32(gpr32, rm32) => {
                let source = self.get_rm32(&rm32);
                let old_dest = self.regs.get_reg32(&gpr32);
                let new = self.regs.flags.add_32(old_dest, source, true);
                self.regs.set_reg32(&gpr32, new);
            }
            Inst::AddRM64(gpr64, rm64) => {
                let source = self.get_rm64(&rm64);
                let old_dest = self.regs.get_reg64(&gpr64);
                let new = self.regs.flags.add_64(old_dest, source, true);
                self.regs.set_reg64(&gpr64, new);
            }
            Inst::SubMI8(rm8, imm8) => {
                let old = self.get_rm8(&rm8);
                let new = self.regs.flags.sub_8(old, imm8, true);
                self.set_rm8(&rm8, new);
            }
            Inst::SubMI16(rm16, imm16) => {
                let old = self.get_rm16(&rm16);
                let new = self.regs.flags.sub_16(old, imm16, true);
                self.set_rm16(&rm16, new);
            }
            Inst::SubMI32(rm32, imm32) => {
                let old = self.get_rm32(&rm32);
                let new = self.regs.flags.sub_32(old, imm32, true);
                self.set_rm32(&rm32, new);
            }
            Inst::SubMI64(rm64, imm64) => {
                let old = self.get_rm64(&rm64);
                let new = self.regs.flags.sub_64(old, imm64, true);
                self.set_rm64(&rm64, new);
            }
            Inst::SubMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(&gpr8);
                let old_dest = self.get_rm8(&rm8);
                let new = self.regs.flags.sub_8(old_dest, source, true);
                self.set_rm8(&rm8, new);
            }
            Inst::SubMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(&gpr16);
                let old_dest = self.get_rm16(&rm16);
                let new = self.regs.flags.sub_16(old_dest, source, true);
                self.set_rm16(&rm16, new);
            }
            Inst::SubMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(&gpr32);
                let old_dest = self.get_rm32(&rm32);
                let new = self.regs.flags.sub_32(old_dest, source, true);
                self.set_rm32(&rm32, new);
            }
            Inst::SubMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(&gpr64);
                let old_dest = self.get_rm64(&rm64);
                let new = self.regs.flags.sub_64(old_dest, source, true);
                self.set_rm64(&rm64, new);
            }
            Inst::SubRM8(gpr8, rm8) => {
                let source = self.get_rm8(&rm8);
                let old_dest = self.regs.get_reg8(&gpr8);
                let new = self.regs.flags.sub_8(old_dest, source, true);
                self.regs.set_reg8(&gpr8, new);
            }
            Inst::SubRM16(gpr16, rm16) => {
                let source = self.get_rm16(&rm16);
                let old_dest = self.regs.get_reg16(&gpr16);
                let new = self.regs.flags.sub_16(old_dest, source, true);
                self.regs.set_reg16(&gpr16, new);
            }
            Inst::SubRM32(gpr32, rm32) => {
                let source = self.get_rm32(&rm32);
                let old_dest = self.regs.get_reg32(&gpr32);
                let new = self.regs.flags.sub_32(old_dest, source, true);
                self.regs.set_reg32(&gpr32, new);
            }
            Inst::SubRM64(gpr64, rm64) => {
                let source = self.get_rm64(&rm64);
                let old_dest = self.regs.get_reg64(&gpr64);
                let new = self.regs.flags.sub_64(old_dest, source, true);
                self.regs.set_reg64(&gpr64, new);
            }
            Inst::CmpMI8(rm8, imm8) => {
                let old = self.get_rm8(&rm8);
                self.regs.flags.sub_8(old, imm8, true);
            }
            Inst::CmpMI16(rm16, imm16) => {
                let old = self.get_rm16(&rm16);
                self.regs.flags.sub_16(old, imm16, true);
            }
            Inst::CmpMI32(rm32, imm32) => {
                let old = self.get_rm32(&rm32);
                self.regs.flags.sub_32(old, imm32, true);
            }
            Inst::CmpMI64(rm64, imm64) => {
                let old = self.get_rm64(&rm64);
                self.regs.flags.sub_64(old, imm64, true);
            }
            Inst::CmpMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(&gpr8);
                let old_dest = self.get_rm8(&rm8);
                self.regs.flags.sub_8(old_dest, source, true);
            }
            Inst::CmpMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(&gpr16);
                let old_dest = self.get_rm16(&rm16);
                self.regs.flags.sub_16(old_dest, source, true);
            }
            Inst::CmpMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(&gpr32);
                let old_dest = self.get_rm32(&rm32);
                self.regs.flags.sub_32(old_dest, source, true);
            }
            Inst::CmpMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(&gpr64);
                let old_dest = self.get_rm64(&rm64);
                self.regs.flags.sub_64(old_dest, source, true);
            }
            Inst::CmpRM8(gpr8, rm8) => {
                let source = self.get_rm8(&rm8);
                let old_dest = self.regs.get_reg8(&gpr8);
                self.regs.flags.sub_8(old_dest, source, true);
            }
            Inst::CmpRM16(gpr16, rm16) => {
                let source = self.get_rm16(&rm16);
                let old_dest = self.regs.get_reg16(&gpr16);
                self.regs.flags.sub_16(old_dest, source, true);
            }
            Inst::CmpRM32(gpr32, rm32) => {
                let source = self.get_rm32(&rm32);
                let old_dest = self.regs.get_reg32(&gpr32);
                self.regs.flags.sub_32(old_dest, source, true);
            }
            Inst::CmpRM64(gpr64, rm64) => {
                let source = self.get_rm64(&rm64);
                let old_dest = self.regs.get_reg64(&gpr64);
                self.regs.flags.sub_64(old_dest, source, true);
            }
            Inst::XorMI8(rm8, imm8) => {
                let old = self.get_rm8(&rm8);
                let new = self.regs.flags.xor_8(old, imm8);
                self.set_rm8(&rm8, new);
            }
            Inst::XorMI16(rm16, imm16) => {
                let old = self.get_rm16(&rm16);
                let new = self.regs.flags.xor_16(old, imm16);
                self.set_rm16(&rm16, new);
            }
            Inst::XorMI32(rm32, imm32) => {
                let old = self.get_rm32(&rm32);
                let new = self.regs.flags.xor_32(old, imm32);
                self.set_rm32(&rm32, new);
            }
            Inst::XorMI64(rm64, imm64) => {
                let old = self.get_rm64(&rm64);
                let new = self.regs.flags.xor_64(old, imm64);
                self.set_rm64(&rm64, new);
            }
            Inst::XorMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(&gpr8);
                let old_dest = self.get_rm8(&rm8);
                let new = self.regs.flags.xor_8(old_dest, source);
                self.set_rm8(&rm8, new);
            }
            Inst::XorMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(&gpr16);
                let old_dest = self.get_rm16(&rm16);
                let new = self.regs.flags.xor_16(old_dest, source);
                self.set_rm16(&rm16, new);
            }
            Inst::XorMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(&gpr32);
                let old_dest = self.get_rm32(&rm32);
                let new = self.regs.flags.xor_32(old_dest, source);
                self.set_rm32(&rm32, new);
            }
            Inst::XorMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(&gpr64);
                let old_dest = self.get_rm64(&rm64);
                let new = self.regs.flags.xor_64(old_dest, source);
                self.set_rm64(&rm64, new);
            }
            Inst::XorRM8(gpr8, rm8) => {
                let source = self.get_rm8(&rm8);
                let old_dest = self.regs.get_reg8(&gpr8);
                let new = self.regs.flags.xor_8(old_dest, source);
                self.regs.set_reg8(&gpr8, new);
            }
            Inst::XorRM16(gpr16, rm16) => {
                let source = self.get_rm16(&rm16);
                let old_dest = self.regs.get_reg16(&gpr16);
                let new = self.regs.flags.xor_16(old_dest, source);
                self.regs.set_reg16(&gpr16, new);
            }
            Inst::XorRM32(gpr32, rm32) => {
                let source = self.get_rm32(&rm32);
                let old_dest = self.regs.get_reg32(&gpr32);
                let new = self.regs.flags.xor_32(old_dest, source);
                self.regs.set_reg32(&gpr32, new);
            }
            Inst::XorRM64(gpr64, rm64) => {
                let source = self.get_rm64(&rm64);
                let old_dest = self.regs.get_reg64(&gpr64);
                let new = self.regs.flags.xor_64(old_dest, source);
                self.regs.set_reg64(&gpr64, new);
            }
            Inst::DivM8(rm8) => {
                let dividend = self.regs.get_reg16(&GPR16::ax);
                let divisor = self.get_rm8(&rm8) as u16;
                if divisor == 0 {
                    divide_error();
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg8(
                    &GPR8::al,
                    // TODO: Verify try_into actually does what I expect
                    // (gives None if the quotient exceeds u8::MAX).
                    quotient.try_into().unwrap_or_else(|_| divide_error()),
                );
                self.regs.set_reg8(&GPR8::ah, remainder as u8);
            }
            Inst::DivM16(rm16) => {
                let dividend = self.regs.get_dx_ax();
                let divisor = self.get_rm16(&rm16) as u32;
                if divisor == 0 {
                    divide_error();
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg16(
                    &GPR16::ax,
                    quotient.try_into().unwrap_or_else(|_| divide_error()),
                );
                self.regs.set_reg16(&GPR16::dx, remainder as u16);
            }
            Inst::DivM32(rm32) => {
                let dividend = self.regs.get_edx_eax();
                let divisor = self.get_rm32(&rm32) as u64;
                if divisor == 0 {
                    divide_error();
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg32(
                    &GPR32::eax,
                    quotient.try_into().unwrap_or_else(|_| divide_error()),
                );
                self.regs.set_reg32(&GPR32::edx, remainder as u32);
            }
            Inst::DivM64(rm64) => {
                let dividend = self.regs.get_rdx_rax();
                let divisor = self.get_rm64(&rm64) as u128;
                if divisor == 0 {
                    divide_error();
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg64(
                    &GPR64::rax,
                    quotient.try_into().unwrap_or_else(|_| divide_error()),
                );
                self.regs.set_reg64(&GPR64::rdx, remainder as u64);
            }
            Inst::JccJo(addr, negate) => {
                if negate.xor(self.regs.flags.of) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJb(addr, negate) => {
                if negate.xor(self.regs.flags.cf) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJe(addr, negate) => {
                if negate.xor(self.regs.flags.zf) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJbe(addr, negate) => {
                if negate.xor(self.regs.flags.cf || self.regs.flags.zf) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJs(addr, negate) => {
                if negate.xor(self.regs.flags.sf) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJp(addr, negate) => {
                if negate.xor(self.regs.flags.pf) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJl(addr, negate) => {
                if negate.xor(self.regs.flags.sf != self.regs.flags.of) {
                    self.regs.rip = addr;
                }
            }
            Inst::JccJle(addr, negate) => {
                if negate.xor(self.regs.flags.zf || (self.regs.flags.sf != self.regs.flags.of)) {
                    self.regs.rip = addr;
                }
            }
            Inst::Jecxz(addr) => {
                if self.regs.get_reg32(&GPR32::ecx) == 0 {
                    self.regs.rip = addr;
                }
            }
            Inst::Jrcxz(addr) => {
                if self.regs.get_reg64(&GPR64::rcx) == 0 {
                    self.regs.rip = addr;
                }
            }
            Inst::JmpD(addr) => {
                self.regs.rip = addr;
            }
            Inst::JmpM64(rm64) => {
                let addr = self.get_rm64(&rm64);
                self.regs.rip = addr;
            }
            Inst::PushM16(rm16) => {
                let val = self.get_rm16(&rm16);
                self.push_16(val)
            }
            Inst::PushM64(rm64) => {
                let val = self.get_rm64(&rm64);
                self.push_64(val)
            }
            Inst::PushI16(imm16) => self.push_16(imm16),
            Inst::PushI64(imm64) => self.push_64(imm64),
            Inst::Pushf16 => {
                let val = self.regs.get_rflags() as u16;
                self.push_16(val);
            }
            Inst::Pushf64 => {
                let val = self.regs.get_rflags();
                self.push_64(val)
            }
            Inst::PopM16(rm16) => {
                let val = self.pop_16();
                self.set_rm16(&rm16, val);
            }
            Inst::PopM64(rm64) => {
                // Note the order is correct here for `pop %rsp`.
                // The increment is before the read value gets placed into %rsp.
                let val = self.pop_64();
                self.set_rm64(&rm64, val);
            }
            Inst::Popf16 => {
                let val = self.pop_16();
                self.regs.flags.set_rflags16(val);
            }
            Inst::Popf64 => {
                let val = self.pop_64();
                self.regs.flags.set_rflags64(val);
            }
        }
    }

    fn pop_16(&mut self) -> u16 {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_add((u16::BITS / 8) as u64)
            .unwrap_or_else(|| stack_fault());
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.read_u16(rsp)
    }

    fn pop_64(&mut self) -> u64 {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_add((u64::BITS / 8) as u64)
            .unwrap_or_else(|| stack_fault());
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.read_u64(rsp)
    }

    fn push_16(&mut self, val: u16) {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_sub((u16::BITS / 8) as u64)
            .unwrap_or_else(|| stack_fault());
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.write_u16(rsp_new, val);
    }

    fn push_64(&mut self, val: u64) {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_sub((u64::BITS / 8) as u64)
            .unwrap_or_else(|| stack_fault());
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.write_u64(rsp_new, val);
    }

    fn syscall(&mut self, writers: &mut Writers) {
        let rax = self.regs.get_reg64(&GPR64::rax);
        self.regs.set_reg64(&GPR64::rcx, self.regs.rip);
        // The r11 register is set while the RF flag is cleared.
        let rflags_clr_rf = self.regs.get_rflags() & !(1 << 16);
        self.regs.set_reg64(&GPR64::r11, rflags_clr_rf);
        let ret = match rax {
            1 => self.sys_write(writers),
            60 => self.sys_exit(),
            _ => panic!("Unimplemented syscall {}", rax),
        };
        self.regs.set_reg64(&GPR64::rax, ret);
    }

    fn sys_write(&mut self, writers: &mut Writers) -> u64 {
        // SYSCALL_DEFINE3(write,
        //     unsigned int, fd,
        //     const char __user *, buf,
        //     size_t, count)
        let fd = self.regs.get_reg32(&GPR32::edi);
        let buf = self.regs.get_reg64(&GPR64::rsi);
        let count = self.regs.get_reg64(&GPR64::rdx);
        let mut buf_out: Vec<u8> = vec![0; count as usize];
        for i in 0..count {
            buf_out[i as usize] = self.mem.read_u8(buf + i);
        }
        match fd {
            1 => writers
                .stdout()
                .write_all(&buf_out)
                .expect("Write should succeed"),
            2 => writers
                .stderr()
                .write_all(&buf_out)
                .expect("Write should succeed"),
            _ => panic!("Unknown file descriptor: {}.", fd),
        }
        count
    }

    fn sys_exit(&mut self) -> u64 {
        self.halt = true;
        0
    }

    fn get_rm8(&mut self, rm8: &RM8) -> u8 {
        match rm8 {
            RM8::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u8(a)
            }
            RM8::Reg(gpr8) => self.regs.get_reg8(gpr8),
        }
    }

    fn get_rm16(&mut self, rm16: &RM16) -> u16 {
        match rm16 {
            RM16::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u16(a)
            }
            RM16::Reg(gpr16) => self.regs.get_reg16(gpr16),
        }
    }

    fn get_rm32(&mut self, rm32: &RM32) -> u32 {
        match rm32 {
            RM32::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u32(a)
            }
            RM32::Reg(gpr32) => self.regs.get_reg32(gpr32),
        }
    }

    fn get_rm64(&mut self, rm64: &RM64) -> u64 {
        match rm64 {
            RM64::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u64(a)
            }
            RM64::Reg(gpr64) => self.regs.get_reg64(gpr64),
        }
    }

    fn set_rm8(&mut self, rm8: &RM8, val: u8) {
        match rm8 {
            RM8::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u8(a, val);
            }
            RM8::Reg(gpr8) => {
                self.regs.set_reg8(gpr8, val);
            }
        }
    }

    fn set_rm16(&mut self, rm16: &RM16, val: u16) {
        match rm16 {
            RM16::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u16(a, val);
            }
            RM16::Reg(gpr16) => {
                self.regs.set_reg16(gpr16, val);
            }
        }
    }

    fn set_rm32(&mut self, rm32: &RM32, val: u32) {
        match rm32 {
            RM32::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u32(a, val);
            }
            RM32::Reg(gpr32) => {
                self.regs.set_reg32(gpr32, val);
            }
        }
    }

    fn set_rm64(&mut self, rm64: &RM64, val: u64) {
        match rm64 {
            RM64::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u64(a, val);
            }
            RM64::Reg(gpr64) => {
                self.regs.set_reg64(gpr64, val);
            }
        }
    }
}

fn stack_fault() -> ! {
    // Described in "push" docs. Causes a double-fault and logical processer shutdown.
    panic!("Stack Fault Exception #SS.");
}

fn divide_error() -> ! {
    panic!("Divide error #DE.");
}
