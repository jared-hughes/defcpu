use crate::{
    decode_inst::decode_inst,
    errors::{RResult, Rerr},
    inst::{DataSize, FullInst, Group1PrefixExec, Inst, RM16, RM32, RM64, RM8},
    inst_prefixes::AddressSizeAttribute::*,
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

    pub fn step(&mut self, writers: &mut Writers) -> RResult<()> {
        if self.halt {
            panic!("Unexpected step in a halt state.")
        }
        let (inst, len) = decode_inst(&self.mem, self.regs.rip)?;
        self.regs.rip_prev = self.regs.rip;
        self.regs.rip += len;
        self.run_full_inst(inst.inner, writers)?;
        Ok(())
    }

    fn run_full_inst(&mut self, full_inst: FullInst, writers: &mut Writers) -> RResult<()> {
        let FullInst {
            group1_prefix,
            main_inst,
        } = full_inst;
        match group1_prefix {
            Some(Group1PrefixExec::Repz(addr_size)) | Some(Group1PrefixExec::Repnz(addr_size)) => {
                loop {
                    let count = match addr_size {
                        Addr32 => self.regs.get_reg32(&GPR32::edi) as u64,
                        Addr64 => self.regs.get_reg64(&GPR64::rdi),
                    };
                    // TODO: verify this is right. The documented pseudocode has
                    // two exits to the loop, since there's a break inside a while loop.
                    if count == 0 {
                        break;
                    }
                    self.run_inst(&main_inst, writers)?;
                    match addr_size {
                        Addr32 => self
                            .regs
                            .set_reg32(&GPR32::edi, (count as u32).wrapping_sub(1)),
                        Addr64 => self.regs.set_reg64(&GPR64::rdi, count.wrapping_sub(1)),
                    };
                    if count == 1 {
                        // The register was just decremented to 0.
                        break;
                    }
                    let should_break = match group1_prefix {
                        Some(Group1PrefixExec::Repz(_)) => !self.regs.flags.zf,
                        Some(Group1PrefixExec::Repnz(_)) => self.regs.flags.zf,
                        _ => panic!("Missing match arm in run_full_inst."),
                    };
                    if should_break {
                        break;
                    }
                }
                // let count_reg =
            }
            None => self.run_inst(&main_inst, writers)?,
        }
        Ok(())
    }

    fn run_inst(&mut self, inst: &Inst, writers: &mut Writers) -> RResult<()> {
        match inst {
            Inst::NotImplemented(opcode) => Err(Rerr::NotImplemented(*opcode))?,
            Inst::NotImplemented2(opcode, opcode2) => {
                Err(Rerr::NotImplemented2(*opcode, *opcode2))?
            }
            Inst::NotImplementedOpext(opcode, sub) => {
                Err(Rerr::NotImplementedOpext(*opcode, *sub))?
            }
            Inst::RexNoop => {}
            Inst::Syscall => self.syscall(writers)?,
            Inst::MovMR8(rm8, gpr8) => {
                let val = self.regs.get_reg8(gpr8);
                self.set_rm8(rm8, val)?;
            }
            Inst::MovMR16(rm16, gpr16) => {
                let val = self.regs.get_reg16(gpr16);
                self.set_rm16(rm16, val)?;
            }
            Inst::MovMR32(rm32, gpr32) => {
                let val = self.regs.get_reg32(gpr32);
                self.set_rm32(rm32, val)?;
            }
            Inst::MovMR64(rm64, gpr64) => {
                let val = self.regs.get_reg64(gpr64);
                self.set_rm64(rm64, val)?;
            }
            Inst::MovRM8(gpr8, rm8) => {
                let val = self.get_rm8(rm8)?;
                self.regs.set_reg8(gpr8, val);
            }
            Inst::MovRM16(gpr16, rm16) => {
                let val = self.get_rm16(rm16)?;
                self.regs.set_reg16(gpr16, val);
            }
            Inst::MovRM32(gpr32, rm32) => {
                let val = self.get_rm32(rm32)?;
                self.regs.set_reg32(gpr32, val);
            }
            Inst::MovRM64(gpr64, rm64) => {
                let val = self.get_rm64(rm64)?;
                self.regs.set_reg64(gpr64, val);
            }
            Inst::MovOI8(gpr8, imm8) => {
                self.regs.set_reg8(gpr8, *imm8);
            }
            Inst::MovOI16(gpr16, imm16) => {
                self.regs.set_reg16(gpr16, *imm16);
            }
            Inst::MovOI32(gpr32, imm32) => {
                self.regs.set_reg32(gpr32, *imm32);
            }
            Inst::MovOI64(gpr64, imm64) => {
                self.regs.set_reg64(gpr64, *imm64);
            }
            Inst::MovMI8(rm8, imm8) => {
                self.set_rm8(rm8, *imm8)?;
            }
            Inst::MovMI16(rm16, imm16) => {
                self.set_rm16(rm16, *imm16)?;
            }
            Inst::MovMI32(rm32, imm32) => {
                self.set_rm32(rm32, *imm32)?;
            }
            Inst::MovMI64(rm64, imm64) => {
                self.set_rm64(rm64, *imm64)?;
            }
            Inst::Hlt => {
                write!(writers.stderr(), "{}", self.regs).expect("Write should succeed.");
                self.halt = true;
            }
            Inst::IncM8(rm8) => {
                let old = self.get_rm8(rm8)?;
                let new = self.regs.flags.add_8(old, 1, false);
                self.set_rm8(rm8, new)?;
            }
            Inst::IncM16(rm16) => {
                let old = self.get_rm16(rm16)?;
                let new = self.regs.flags.add_16(old, 1, false);
                self.set_rm16(rm16, new)?;
            }
            Inst::IncM32(rm32) => {
                let old = self.get_rm32(rm32)?;
                let new = self.regs.flags.add_32(old, 1, false);
                self.set_rm32(rm32, new)?;
            }
            Inst::IncM64(rm64) => {
                let old = self.get_rm64(rm64)?;
                let new = self.regs.flags.add_64(old, 1, false);
                self.set_rm64(rm64, new)?;
            }
            Inst::DecM8(rm8) => {
                let old = self.get_rm8(rm8)?;
                let new = self.regs.flags.sub_8(old, 1, false);
                self.set_rm8(rm8, new)?;
            }
            Inst::DecM16(rm16) => {
                let old = self.get_rm16(rm16)?;
                let new = self.regs.flags.sub_16(old, 1, false);
                self.set_rm16(rm16, new)?;
            }
            Inst::DecM32(rm32) => {
                let old = self.get_rm32(rm32)?;
                let new = self.regs.flags.sub_32(old, 1, false);
                self.set_rm32(rm32, new)?;
            }
            Inst::DecM64(rm64) => {
                let old = self.get_rm64(rm64)?;
                let new = self.regs.flags.sub_64(old, 1, false);
                self.set_rm64(rm64, new)?;
            }
            Inst::AddMI8(rm8, imm8) => {
                let old = self.get_rm8(rm8)?;
                let new = self.regs.flags.add_8(old, *imm8, true);
                self.set_rm8(rm8, new)?;
            }
            Inst::AddMI16(rm16, imm16) => {
                let old = self.get_rm16(rm16)?;
                let new = self.regs.flags.add_16(old, *imm16, true);
                self.set_rm16(rm16, new)?;
            }
            Inst::AddMI32(rm32, imm32) => {
                let old = self.get_rm32(rm32)?;
                let new = self.regs.flags.add_32(old, *imm32, true);
                self.set_rm32(rm32, new)?;
            }
            Inst::AddMI64(rm64, imm64) => {
                let old = self.get_rm64(rm64)?;
                let new = self.regs.flags.add_64(old, *imm64, true);
                self.set_rm64(rm64, new)?;
            }
            Inst::AddMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(gpr8);
                let old_dest = self.get_rm8(rm8)?;
                let new = self.regs.flags.add_8(old_dest, source, true);
                self.set_rm8(rm8, new)?;
            }
            Inst::AddMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(gpr16);
                let old_dest = self.get_rm16(rm16)?;
                let new = self.regs.flags.add_16(old_dest, source, true);
                self.set_rm16(rm16, new)?;
            }
            Inst::AddMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(gpr32);
                let old_dest = self.get_rm32(rm32)?;
                let new = self.regs.flags.add_32(old_dest, source, true);
                self.set_rm32(rm32, new)?;
            }
            Inst::AddMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(gpr64);
                let old_dest = self.get_rm64(rm64)?;
                let new = self.regs.flags.add_64(old_dest, source, true);
                self.set_rm64(rm64, new)?;
            }
            Inst::AddRM8(gpr8, rm8) => {
                let source = self.get_rm8(rm8)?;
                let old_dest = self.regs.get_reg8(gpr8);
                let new = self.regs.flags.add_8(old_dest, source, true);
                self.regs.set_reg8(gpr8, new);
            }
            Inst::AddRM16(gpr16, rm16) => {
                let source = self.get_rm16(rm16)?;
                let old_dest = self.regs.get_reg16(gpr16);
                let new = self.regs.flags.add_16(old_dest, source, true);
                self.regs.set_reg16(gpr16, new);
            }
            Inst::AddRM32(gpr32, rm32) => {
                let source = self.get_rm32(rm32)?;
                let old_dest = self.regs.get_reg32(gpr32);
                let new = self.regs.flags.add_32(old_dest, source, true);
                self.regs.set_reg32(gpr32, new);
            }
            Inst::AddRM64(gpr64, rm64) => {
                let source = self.get_rm64(rm64)?;
                let old_dest = self.regs.get_reg64(gpr64);
                let new = self.regs.flags.add_64(old_dest, source, true);
                self.regs.set_reg64(gpr64, new);
            }
            Inst::SubMI8(rm8, imm8) => {
                let old = self.get_rm8(rm8)?;
                let new = self.regs.flags.sub_8(old, *imm8, true);
                self.set_rm8(rm8, new)?;
            }
            Inst::SubMI16(rm16, imm16) => {
                let old = self.get_rm16(rm16)?;
                let new = self.regs.flags.sub_16(old, *imm16, true);
                self.set_rm16(rm16, new)?;
            }
            Inst::SubMI32(rm32, imm32) => {
                let old = self.get_rm32(rm32)?;
                let new = self.regs.flags.sub_32(old, *imm32, true);
                self.set_rm32(rm32, new)?;
            }
            Inst::SubMI64(rm64, imm64) => {
                let old = self.get_rm64(rm64)?;
                let new = self.regs.flags.sub_64(old, *imm64, true);
                self.set_rm64(rm64, new)?;
            }
            Inst::SubMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(gpr8);
                let old_dest = self.get_rm8(rm8)?;
                let new = self.regs.flags.sub_8(old_dest, source, true);
                self.set_rm8(rm8, new)?;
            }
            Inst::SubMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(gpr16);
                let old_dest = self.get_rm16(rm16)?;
                let new = self.regs.flags.sub_16(old_dest, source, true);
                self.set_rm16(rm16, new)?;
            }
            Inst::SubMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(gpr32);
                let old_dest = self.get_rm32(rm32)?;
                let new = self.regs.flags.sub_32(old_dest, source, true);
                self.set_rm32(rm32, new)?;
            }
            Inst::SubMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(gpr64);
                let old_dest = self.get_rm64(rm64)?;
                let new = self.regs.flags.sub_64(old_dest, source, true);
                self.set_rm64(rm64, new)?;
            }
            Inst::SubRM8(gpr8, rm8) => {
                let source = self.get_rm8(rm8)?;
                let old_dest = self.regs.get_reg8(gpr8);
                let new = self.regs.flags.sub_8(old_dest, source, true);
                self.regs.set_reg8(gpr8, new);
            }
            Inst::SubRM16(gpr16, rm16) => {
                let source = self.get_rm16(rm16)?;
                let old_dest = self.regs.get_reg16(gpr16);
                let new = self.regs.flags.sub_16(old_dest, source, true);
                self.regs.set_reg16(gpr16, new);
            }
            Inst::SubRM32(gpr32, rm32) => {
                let source = self.get_rm32(rm32)?;
                let old_dest = self.regs.get_reg32(gpr32);
                let new = self.regs.flags.sub_32(old_dest, source, true);
                self.regs.set_reg32(gpr32, new);
            }
            Inst::SubRM64(gpr64, rm64) => {
                let source = self.get_rm64(rm64)?;
                let old_dest = self.regs.get_reg64(gpr64);
                let new = self.regs.flags.sub_64(old_dest, source, true);
                self.regs.set_reg64(gpr64, new);
            }
            Inst::CmpMI8(rm8, imm8) => {
                let old = self.get_rm8(rm8)?;
                self.regs.flags.sub_8(old, *imm8, true);
            }
            Inst::CmpMI16(rm16, imm16) => {
                let old = self.get_rm16(rm16)?;
                self.regs.flags.sub_16(old, *imm16, true);
            }
            Inst::CmpMI32(rm32, imm32) => {
                let old = self.get_rm32(rm32)?;
                self.regs.flags.sub_32(old, *imm32, true);
            }
            Inst::CmpMI64(rm64, imm64) => {
                let old = self.get_rm64(rm64)?;
                self.regs.flags.sub_64(old, *imm64, true);
            }
            Inst::CmpMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(gpr8);
                let old_dest = self.get_rm8(rm8)?;
                self.regs.flags.sub_8(old_dest, source, true);
            }
            Inst::CmpMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(gpr16);
                let old_dest = self.get_rm16(rm16)?;
                self.regs.flags.sub_16(old_dest, source, true);
            }
            Inst::CmpMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(gpr32);
                let old_dest = self.get_rm32(rm32)?;
                self.regs.flags.sub_32(old_dest, source, true);
            }
            Inst::CmpMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(gpr64);
                let old_dest = self.get_rm64(rm64)?;
                self.regs.flags.sub_64(old_dest, source, true);
            }
            Inst::CmpRM8(gpr8, rm8) => {
                let source = self.get_rm8(rm8)?;
                let old_dest = self.regs.get_reg8(gpr8);
                self.regs.flags.sub_8(old_dest, source, true);
            }
            Inst::CmpRM16(gpr16, rm16) => {
                let source = self.get_rm16(rm16)?;
                let old_dest = self.regs.get_reg16(gpr16);
                self.regs.flags.sub_16(old_dest, source, true);
            }
            Inst::CmpRM32(gpr32, rm32) => {
                let source = self.get_rm32(rm32)?;
                let old_dest = self.regs.get_reg32(gpr32);
                self.regs.flags.sub_32(old_dest, source, true);
            }
            Inst::CmpRM64(gpr64, rm64) => {
                let source = self.get_rm64(rm64)?;
                let old_dest = self.regs.get_reg64(gpr64);
                self.regs.flags.sub_64(old_dest, source, true);
            }
            Inst::XorMI8(rm8, imm8) => {
                let old = self.get_rm8(rm8)?;
                let new = self.regs.flags.xor_8(old, *imm8);
                self.set_rm8(rm8, new)?;
            }
            Inst::XorMI16(rm16, imm16) => {
                let old = self.get_rm16(rm16)?;
                let new = self.regs.flags.xor_16(old, *imm16);
                self.set_rm16(rm16, new)?;
            }
            Inst::XorMI32(rm32, imm32) => {
                let old = self.get_rm32(rm32)?;
                let new = self.regs.flags.xor_32(old, *imm32);
                self.set_rm32(rm32, new)?;
            }
            Inst::XorMI64(rm64, imm64) => {
                let old = self.get_rm64(rm64)?;
                let new = self.regs.flags.xor_64(old, *imm64);
                self.set_rm64(rm64, new)?;
            }
            Inst::XorMR8(rm8, gpr8) => {
                let source = self.regs.get_reg8(gpr8);
                let old_dest = self.get_rm8(rm8)?;
                let new = self.regs.flags.xor_8(old_dest, source);
                self.set_rm8(rm8, new)?;
            }
            Inst::XorMR16(rm16, gpr16) => {
                let source = self.regs.get_reg16(gpr16);
                let old_dest = self.get_rm16(rm16)?;
                let new = self.regs.flags.xor_16(old_dest, source);
                self.set_rm16(rm16, new)?;
            }
            Inst::XorMR32(rm32, gpr32) => {
                let source = self.regs.get_reg32(gpr32);
                let old_dest = self.get_rm32(rm32)?;
                let new = self.regs.flags.xor_32(old_dest, source);
                self.set_rm32(rm32, new)?;
            }
            Inst::XorMR64(rm64, gpr64) => {
                let source = self.regs.get_reg64(gpr64);
                let old_dest = self.get_rm64(rm64)?;
                let new = self.regs.flags.xor_64(old_dest, source);
                self.set_rm64(rm64, new)?;
            }
            Inst::XorRM8(gpr8, rm8) => {
                let source = self.get_rm8(rm8)?;
                let old_dest = self.regs.get_reg8(gpr8);
                let new = self.regs.flags.xor_8(old_dest, source);
                self.regs.set_reg8(gpr8, new);
            }
            Inst::XorRM16(gpr16, rm16) => {
                let source = self.get_rm16(rm16)?;
                let old_dest = self.regs.get_reg16(gpr16);
                let new = self.regs.flags.xor_16(old_dest, source);
                self.regs.set_reg16(gpr16, new);
            }
            Inst::XorRM32(gpr32, rm32) => {
                let source = self.get_rm32(rm32)?;
                let old_dest = self.regs.get_reg32(gpr32);
                let new = self.regs.flags.xor_32(old_dest, source);
                self.regs.set_reg32(gpr32, new);
            }
            Inst::XorRM64(gpr64, rm64) => {
                let source = self.get_rm64(rm64)?;
                let old_dest = self.regs.get_reg64(gpr64);
                let new = self.regs.flags.xor_64(old_dest, source);
                self.regs.set_reg64(gpr64, new);
            }
            Inst::BtMR16(rm16, gpr16) => {
                let ind = self.regs.get_reg16(gpr16);
                let bit_ind = ind % 16;
                let byte_ind = ((u16::BITS / 8) as u16) * (ind / 16);
                let val = self.get_rm16_with_offset(rm16, byte_ind as u64)?;
                self.regs.flags.cf = 1 == (val >> bit_ind) & 1
            }
            Inst::BtMR32(rm32, gpr32) => {
                let ind = self.regs.get_reg32(gpr32);
                let bit_ind = ind % 32;
                let byte_ind = (u32::BITS / 8) * (ind / 32);
                let val = self.get_rm32_with_offset(rm32, byte_ind as u64)?;
                self.regs.flags.cf = 1 == (val >> bit_ind) & 1
            }
            Inst::BtMR64(rm64, gpr64) => {
                let ind = self.regs.get_reg64(gpr64);
                let bit_ind = ind % 64;
                let byte_ind = ((u64::BITS / 8) as u64) * (ind / 64);
                let val = self.get_rm64_with_offset(rm64, byte_ind)?;
                self.regs.flags.cf = 1 == (val >> bit_ind) & 1
            }
            Inst::BtMI16(rm16, imm8) => {
                let bit_ind = imm8 % 16;
                let val = self.get_rm16(rm16)?;
                self.regs.flags.cf = 1 == (val >> bit_ind) & 1
            }
            Inst::BtMI32(rm32, imm8) => {
                let bit_ind = imm8 % 32;
                let val = self.get_rm32(rm32)?;
                self.regs.flags.cf = 1 == (val >> bit_ind) & 1
            }
            Inst::BtMI64(rm64, imm8) => {
                let bit_ind = imm8 % 64;
                let val = self.get_rm64(rm64)?;
                self.regs.flags.cf = 1 == (val >> bit_ind) & 1
            }
            Inst::DivM8(rm8) => {
                let dividend = self.regs.get_reg16(&GPR16::ax);
                let divisor = self.get_rm8(rm8)? as u16;
                if divisor == 0 {
                    Err(Rerr::DivideError)?;
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg8(
                    &GPR8::al,
                    // TODO: Verify try_into actually does what I expect
                    // (gives None if the quotient exceeds u8::MAX).
                    quotient.try_into().map_err(|_| Rerr::DivideError)?,
                );
                self.regs.set_reg8(&GPR8::ah, remainder as u8);
            }
            Inst::DivM16(rm16) => {
                let dividend = self.regs.get_dx_ax();
                let divisor = self.get_rm16(rm16)? as u32;
                if divisor == 0 {
                    Err(Rerr::DivideError)?;
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg16(
                    &GPR16::ax,
                    quotient.try_into().map_err(|_| Rerr::DivideError)?,
                );
                self.regs.set_reg16(&GPR16::dx, remainder as u16);
            }
            Inst::DivM32(rm32) => {
                let dividend = self.regs.get_edx_eax();
                let divisor = self.get_rm32(rm32)? as u64;
                if divisor == 0 {
                    Err(Rerr::DivideError)?;
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg32(
                    &GPR32::eax,
                    quotient.try_into().map_err(|_| Rerr::DivideError)?,
                );
                self.regs.set_reg32(&GPR32::edx, remainder as u32);
            }
            Inst::DivM64(rm64) => {
                let dividend = self.regs.get_rdx_rax();
                let divisor = self.get_rm64(rm64)? as u128;
                if divisor == 0 {
                    Err(Rerr::DivideError)?;
                }
                self.regs.flags.div_flags();
                let quotient = dividend.div_euclid(divisor);
                let remainder = dividend.rem_euclid(divisor);
                self.regs.set_reg64(
                    &GPR64::rax,
                    quotient.try_into().map_err(|_| Rerr::DivideError)?,
                );
                self.regs.set_reg64(&GPR64::rdx, remainder as u64);
            }
            Inst::NotM8(rm8) => {
                let val = self.get_rm8(rm8)?;
                self.set_rm8(rm8, !val)?;
            }
            Inst::NotM16(rm16) => {
                let val = self.get_rm16(rm16)?;
                self.set_rm16(rm16, !val)?;
            }
            Inst::NotM32(rm32) => {
                let val = self.get_rm32(rm32)?;
                self.set_rm32(rm32, !val)?;
            }
            Inst::NotM64(rm64) => {
                let val = self.get_rm64(rm64)?;
                self.set_rm64(rm64, !val)?;
            }
            Inst::ImulM8(rm8) => {
                let x = self.regs.get_reg8(&GPR8::al);
                let y = self.get_rm8(rm8)?;
                let prod = self.regs.flags.imul_8(x, y);
                self.regs.set_reg16(&GPR16::ax, prod);
            }
            Inst::ImulM16(rm16) => {
                let x = self.regs.get_reg16(&GPR16::ax);
                let y = self.get_rm16(rm16)?;
                let prod = self.regs.flags.imul_16(x, y);
                self.regs.set_dx_ax(prod);
            }
            Inst::ImulM32(rm32) => {
                let x = self.regs.get_reg32(&GPR32::eax);
                let y = self.get_rm32(rm32)?;
                let prod = self.regs.flags.imul_32(x, y);
                self.regs.set_edx_eax(prod);
            }
            Inst::ImulM64(rm64) => {
                let x = self.regs.get_reg64(&GPR64::rax);
                let y = self.get_rm64(rm64)?;
                let prod = self.regs.flags.imul_64(x, y);
                self.regs.set_rdx_rax(prod);
            }
            Inst::ImulRM16(gpr16, rm16) => {
                let x = self.regs.get_reg16(gpr16);
                let y = self.get_rm16(rm16)?;
                let prod = self.regs.flags.imul_16(x, y);
                self.regs.set_reg16(gpr16, prod as u16);
            }
            Inst::ImulRM32(gpr32, rm32) => {
                let x = self.regs.get_reg32(gpr32);
                let y = self.get_rm32(rm32)?;
                let prod = self.regs.flags.imul_32(x, y);
                self.regs.set_reg32(gpr32, prod as u32);
            }
            Inst::ImulRM64(gpr64, rm64) => {
                let x = self.regs.get_reg64(gpr64);
                let y = self.get_rm64(rm64)?;
                let prod = self.regs.flags.imul_64(x, y);
                self.regs.set_reg64(gpr64, prod as u64);
            }
            Inst::ImulRMI16(gpr16, rm16, imm16) => {
                let x = self.get_rm16(rm16)?;
                let prod = self.regs.flags.imul_16(x, *imm16);
                self.regs.set_reg16(gpr16, prod as u16);
            }
            Inst::ImulRMI32(gpr32, rm32, imm32) => {
                let x = self.get_rm32(rm32)?;
                let prod = self.regs.flags.imul_32(x, *imm32);
                self.regs.set_reg32(gpr32, prod as u32);
            }
            Inst::ImulRMI64(gpr64, rm64, imm64) => {
                let x = self.get_rm64(rm64)?;
                let prod = self.regs.flags.imul_64(x, *imm64);
                self.regs.set_reg64(gpr64, prod as u64);
            }
            Inst::JccJo(addr, negate) => {
                if negate.xor(self.regs.flags.of) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJb(addr, negate) => {
                if negate.xor(self.regs.flags.cf) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJe(addr, negate) => {
                if negate.xor(self.regs.flags.zf) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJbe(addr, negate) => {
                if negate.xor(self.regs.flags.cf || self.regs.flags.zf) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJs(addr, negate) => {
                if negate.xor(self.regs.flags.sf) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJp(addr, negate) => {
                if negate.xor(self.regs.flags.pf) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJl(addr, negate) => {
                if negate.xor(self.regs.flags.sf != self.regs.flags.of) {
                    self.regs.rip = *addr;
                }
            }
            Inst::JccJle(addr, negate) => {
                if negate.xor(self.regs.flags.zf || (self.regs.flags.sf != self.regs.flags.of)) {
                    self.regs.rip = *addr;
                }
            }
            Inst::Jecxz(addr) => {
                if self.regs.get_reg32(&GPR32::ecx) == 0 {
                    self.regs.rip = *addr;
                }
            }
            Inst::Jrcxz(addr) => {
                if self.regs.get_reg64(&GPR64::rcx) == 0 {
                    self.regs.rip = *addr;
                }
            }
            Inst::JmpD(addr) => {
                self.regs.rip = *addr;
            }
            Inst::JmpM64(rm64) => {
                let addr = self.get_rm64(rm64)?;
                self.regs.rip = addr;
            }
            Inst::PushM16(rm16) => {
                let val = self.get_rm16(rm16)?;
                self.push_16(val)?
            }
            Inst::PushM64(rm64) => {
                let val = self.get_rm64(rm64)?;
                self.push_64(val)?
            }
            Inst::PushI16(imm16) => self.push_16(*imm16)?,
            Inst::PushI64(imm64) => self.push_64(*imm64)?,
            Inst::Pushf16 => {
                let val = self.regs.get_rflags() as u16;
                self.push_16(val)?;
            }
            Inst::Pushf64 => {
                let val = self.regs.get_rflags();
                self.push_64(val)?
            }
            Inst::PopM16(rm16) => {
                let val = self.pop_16()?;
                self.set_rm16(rm16, val)?;
            }
            Inst::PopM64(rm64) => {
                // Note the order is correct here for `pop %rsp`.
                // The increment is before the read value gets placed into %rsp.
                let val = self.pop_64()?;
                self.set_rm64(rm64, val)?;
            }
            Inst::Popf16 => {
                let val = self.pop_16()?;
                self.regs.flags.set_rflags16(val);
            }
            Inst::Popf64 => {
                let val = self.pop_64()?;
                self.regs.flags.set_rflags64(val);
            }
            Inst::Scas(data_size, addr_size) => {
                let addr = match addr_size {
                    Addr32 => self.regs.get_reg32(&GPR32::edi) as u64,
                    Addr64 => self.regs.get_reg64(&GPR64::rdi),
                };
                match data_size {
                    DataSize::Data8 => {
                        let a = self.regs.get_reg8(&GPR8::al);
                        let b = self.mem.read_u8(addr)?;
                        self.regs.flags.sub_8(a, b, true);
                    }
                    DataSize::Data16 => {
                        let a = self.regs.get_reg16(&GPR16::ax);
                        let b = self.mem.read_u16(addr)?;
                        self.regs.flags.sub_16(a, b, true);
                    }
                    DataSize::Data32 => {
                        let a = self.regs.get_reg32(&GPR32::eax);
                        let b = self.mem.read_u32(addr)?;
                        self.regs.flags.sub_32(a, b, true);
                    }
                    DataSize::Data64 => {
                        let a = self.regs.get_reg64(&GPR64::rax);
                        let b = self.mem.read_u64(addr)?;
                        self.regs.flags.sub_64(a, b, true);
                    }
                }
                match addr_size {
                    Addr32 => self
                        .regs
                        .offset_reg32_by_df(&GPR32::edi, data_size.byte_len()),
                    Addr64 => self
                        .regs
                        .offset_reg64_by_df(&GPR64::rdi, data_size.byte_len().into()),
                }
            }
        }
        Ok(())
    }

    fn pop_16(&mut self) -> RResult<u16> {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_add((u16::BITS / 8) as u64)
            .ok_or(Rerr::StackFault)?;
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.read_u16(rsp)
    }

    fn pop_64(&mut self) -> RResult<u64> {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_add((u64::BITS / 8) as u64)
            .ok_or(Rerr::StackFault)?;
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.read_u64(rsp)
    }

    fn push_16(&mut self, val: u16) -> RResult<()> {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_sub((u16::BITS / 8) as u64)
            .ok_or(Rerr::StackFault)?;
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.write_u16(rsp_new, val)?;
        Ok(())
    }

    fn push_64(&mut self, val: u64) -> RResult<()> {
        let rsp = self.regs.get_reg64(&GPR64::rsp);
        let rsp_new = rsp
            .checked_sub((u64::BITS / 8) as u64)
            .ok_or(Rerr::StackFault)?;
        self.regs.set_reg64(&GPR64::rsp, rsp_new);
        self.mem.write_u64(rsp_new, val)?;
        Ok(())
    }

    fn syscall(&mut self, writers: &mut Writers) -> RResult<()> {
        let eax = self.regs.get_reg32(&GPR32::eax);
        self.regs.set_reg64(&GPR64::rcx, self.regs.rip);
        // The r11 register is set while the RF flag is cleared.
        let rflags_clr_rf = self.regs.get_rflags() & !(1 << 16);
        self.regs.set_reg64(&GPR64::r11, rflags_clr_rf);
        let ret = match eax {
            1 => self.sys_write(writers)?,
            60 => self.sys_exit(),
            _ => Err(Rerr::UnimplementedSyscall(eax))?,
        };
        self.regs.set_reg64(&GPR64::rax, ret);
        Ok(())
    }

    fn sys_write(&mut self, writers: &mut Writers) -> RResult<u64> {
        // SYSCALL_DEFINE3(write,
        //     unsigned int, fd,
        //     const char __user *, buf,
        //     size_t, count)
        let fd = self.regs.get_reg32(&GPR32::edi);
        let buf = self.regs.get_reg64(&GPR64::rsi);
        let count = self.regs.get_reg64(&GPR64::rdx);
        let mut buf_out: Vec<u8> = vec![0; count as usize];
        for i in 0..count {
            buf_out[i as usize] = self.mem.read_u8(buf + i)?;
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
            _ => Err(Rerr::UnknownFileDescriptor(fd))?,
        }
        Ok(count)
    }

    fn sys_exit(&mut self) -> u64 {
        self.halt = true;
        0
    }

    fn get_rm8(&mut self, rm8: &RM8) -> RResult<u8> {
        self.get_rm8_with_offset(rm8, 0)
    }

    fn get_rm16(&mut self, rm16: &RM16) -> RResult<u16> {
        self.get_rm16_with_offset(rm16, 0)
    }

    fn get_rm32(&mut self, rm32: &RM32) -> RResult<u32> {
        self.get_rm32_with_offset(rm32, 0)
    }

    fn get_rm64(&mut self, rm64: &RM64) -> RResult<u64> {
        self.get_rm64_with_offset(rm64, 0)
    }

    fn get_rm8_with_offset(&mut self, rm8: &RM8, byte_offset: u64) -> RResult<u8> {
        match rm8 {
            RM8::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs) + byte_offset;
                self.mem.read_u8(a)
            }
            RM8::Reg(gpr8) => Ok(self.regs.get_reg8(gpr8)),
        }
    }

    fn get_rm16_with_offset(&mut self, rm16: &RM16, byte_offset: u64) -> RResult<u16> {
        match rm16 {
            RM16::Addr(eff_addr) => {
                // TODO: Test what happens when adding byte_offset
                // wraps around. Should it wrap around at u32::MAX
                // in 32-bit address mode? This is relevant for the `bt` instruction.
                let a = eff_addr.compute(&self.regs) + byte_offset;
                self.mem.read_u16(a)
            }
            RM16::Reg(gpr16) => Ok(self.regs.get_reg16(gpr16)),
        }
    }

    fn get_rm32_with_offset(&mut self, rm32: &RM32, byte_offset: u64) -> RResult<u32> {
        match rm32 {
            RM32::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs) + byte_offset;
                self.mem.read_u32(a)
            }
            RM32::Reg(gpr32) => Ok(self.regs.get_reg32(gpr32)),
        }
    }

    fn get_rm64_with_offset(&mut self, rm64: &RM64, byte_offset: u64) -> RResult<u64> {
        match rm64 {
            RM64::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs) + byte_offset;
                self.mem.read_u64(a)
            }
            RM64::Reg(gpr64) => Ok(self.regs.get_reg64(gpr64)),
        }
    }

    fn set_rm8(&mut self, rm8: &RM8, val: u8) -> RResult<()> {
        match rm8 {
            RM8::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u8(a, val)?;
            }
            RM8::Reg(gpr8) => {
                self.regs.set_reg8(gpr8, val);
            }
        }
        Ok(())
    }

    fn set_rm16(&mut self, rm16: &RM16, val: u16) -> RResult<()> {
        match rm16 {
            RM16::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u16(a, val)?;
            }
            RM16::Reg(gpr16) => {
                self.regs.set_reg16(gpr16, val);
            }
        }
        Ok(())
    }

    fn set_rm32(&mut self, rm32: &RM32, val: u32) -> RResult<()> {
        match rm32 {
            RM32::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u32(a, val)?;
            }
            RM32::Reg(gpr32) => {
                self.regs.set_reg32(gpr32, val);
            }
        }
        Ok(())
    }

    fn set_rm64(&mut self, rm64: &RM64, val: u64) -> RResult<()> {
        match rm64 {
            RM64::Addr(eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.mem.write_u64(a, val)?;
            }
            RM64::Reg(gpr64) => {
                self.regs.set_reg64(gpr64, val);
            }
        }
        Ok(())
    }
}
