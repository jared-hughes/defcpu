use crate::{
    decode_inst::{decode_inst, Inst},
    memory::Memory,
    parse_elf::SimpleElfFile,
    registers::{Registers, GPR32, GPR8},
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
        self.run_inst(inst);
        self.regs.rip += len;
    }

    pub fn run_inst(&mut self, inst: Inst) {
        match inst {
            Inst::MovImm8(gpr8, imm8) => match gpr8 {
                GPR8::Low(qreg) => {
                    let ind = qreg.reg_index();
                    self.regs.regs[ind] &= !0xFF;
                    self.regs.regs[ind] |= imm8 as u64;
                }
                GPR8::High(abcdreg) => {
                    let ind = abcdreg.reg_index();
                    self.regs.regs[ind] &= !0xFF_00;
                    self.regs.regs[ind] |= (imm8 as u64) << 8;
                }
            },
            Inst::MovImm32(GPR32(qreg), imm32) => {
                let ind = qreg.reg_index();
                self.regs.regs[ind] &= !0xFF_FF_FF_FF;
                self.regs.regs[ind] |= imm32 as u64;
            }
            Inst::Hlt => {
                eprintln!("{}", self.regs);
                self.halt = true;
            }
        }
    }
}
