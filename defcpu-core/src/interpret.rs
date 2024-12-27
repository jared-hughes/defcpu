use crate::{
    decode_inst::decode_inst, inst::Inst, memory::Memory, parse_elf::SimpleElfFile,
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
            Inst::RexNoop => {}
            Inst::MovImm8(gpr8, imm8) => {
                self.regs.set_reg8(gpr8, imm8);
            }
            Inst::MovImm16(gpr16, imm16) => {
                self.regs.set_reg16(gpr16, imm16);
            }
            Inst::MovImm32(gpr32, imm32) => {
                self.regs.set_reg32(gpr32, imm32);
            }
            Inst::MovImm64(gpr64, imm64) => {
                self.regs.set_reg64(gpr64, imm64);
            }
            Inst::Hlt => {
                eprintln!("{}", self.regs);
                self.halt = true;
            }
        }
    }
}
