use crate::{
    decode_inst::decode_inst,
    inst::{Inst, RM16, RM32, RM64, RM8},
    memory::Memory,
    parse_elf::SimpleElfFile,
    registers::Registers,
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
            // Initialize some misc flags to match the code.golf dump.
            rflags: 0x0000000000010202,
            rip: file.e_entry,
        };
        Machine {
            regs,
            mem,
            halt: false,
        }
    }

    pub fn step(&mut self) {
        if self.halt {
            panic!("Unexpected step in a halt state.")
        }
        let (inst, len) = decode_inst(&self.mem, self.regs.rip);
        self.run_inst(inst.inner);
        self.regs.rip += len;
    }

    pub fn run_inst(&mut self, inst: Inst) {
        match inst {
            Inst::NotImplemented(opcode) => {
                panic!("Not yet implemented opcode {opcode:02x}.")
            }
            Inst::NotImplementedOpext(opcode, sub) => {
                panic!("Not yet implemented opcode {opcode:02x} /{sub}.")
            }
            Inst::RexNoop => {}
            Inst::MovMR8(rm8, gpr8) => {
                let val = self.regs.get_reg8(gpr8);
                self.set_rm8(rm8, val);
            }
            Inst::MovMR16(rm16, gpr16) => {
                let val = self.regs.get_reg16(gpr16);
                self.set_rm16(rm16, val);
            }
            Inst::MovMR32(rm32, gpr32) => {
                let val = self.regs.get_reg32(gpr32);
                self.set_rm32(rm32, val);
            }
            Inst::MovMR64(rm64, gpr64) => {
                let val = self.regs.get_reg64(gpr64);
                self.set_rm64(rm64, val);
            }
            Inst::MovRM8(gpr8, rm8) => {
                let val = self.get_rm8(rm8);
                self.regs.set_reg8(gpr8, val);
            }
            Inst::MovRM16(gpr16, rm16) => {
                let val = self.get_rm16(rm16);
                self.regs.set_reg16(gpr16, val);
            }
            Inst::MovRM32(gpr32, rm32) => {
                let val = self.get_rm32(rm32);
                self.regs.set_reg32(gpr32, val);
            }
            Inst::MovRM64(gpr64, rm64) => {
                let val = self.get_rm64(rm64);
                self.regs.set_reg64(gpr64, val);
            }
            Inst::MovOI8(gpr8, imm8) => {
                self.regs.set_reg8(gpr8, imm8);
            }
            Inst::MovOI16(gpr16, imm16) => {
                self.regs.set_reg16(gpr16, imm16);
            }
            Inst::MovOI32(gpr32, imm32) => {
                self.regs.set_reg32(gpr32, imm32);
            }
            Inst::MovOI64(gpr64, imm64) => {
                self.regs.set_reg64(gpr64, imm64);
            }
            Inst::MovMI8(rm8, imm8) => {
                self.set_rm8(rm8, imm8);
            }
            Inst::MovMI16(rm16, imm16) => {
                self.set_rm16(rm16, imm16);
            }
            Inst::MovMI32(rm32, imm32) => {
                self.set_rm32(rm32, imm32);
            }
            Inst::MovMI64(rm64, imm64) => {
                self.set_rm64(rm64, imm64);
            }
            Inst::Hlt => {
                eprintln!("{}", self.regs);
                self.halt = true;
            }
        }
    }

    fn get_rm8(&mut self, addr: RM8) -> u8 {
        match addr {
            RM8::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u8(a)
            }
            RM8::Reg(gpr8) => self.regs.get_reg8(gpr8),
        }
    }

    fn get_rm16(&mut self, addr: RM16) -> u16 {
        match addr {
            RM16::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u16(a)
            }
            RM16::Reg(gpr16) => self.regs.get_reg16(gpr16),
        }
    }

    fn get_rm32(&mut self, addr: RM32) -> u32 {
        match addr {
            RM32::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u32(a)
            }
            RM32::Reg(gpr32) => self.regs.get_reg32(gpr32),
        }
    }

    fn get_rm64(&mut self, addr: RM64) -> u64 {
        match addr {
            RM64::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.read_u64(a)
            }
            RM64::Reg(gpr64) => self.regs.get_reg64(gpr64),
        }
    }

    fn set_rm8(&mut self, addr: RM8, val: u8) {
        match addr {
            RM8::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u8(a, val);
            }
            RM8::Reg(gpr8) => {
                self.regs.set_reg8(gpr8, val);
            }
        }
    }

    fn set_rm16(&mut self, addr: RM16, val: u16) {
        match addr {
            RM16::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u16(a, val);
            }
            RM16::Reg(gpr16) => {
                self.regs.set_reg16(gpr16, val);
            }
        }
    }

    fn set_rm32(&mut self, addr: RM32, val: u32) {
        match addr {
            RM32::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u32(a, val);
            }
            RM32::Reg(gpr32) => {
                self.regs.set_reg32(gpr32, val);
            }
        }
    }

    fn set_rm64(&mut self, addr: RM64, val: u64) {
        match addr {
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
