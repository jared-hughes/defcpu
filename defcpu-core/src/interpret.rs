use crate::init_mem::{init_mem, InitOpts};
use crate::inst::{DestMROp, OneOp, PlainOneOp, ShrinkOp, TwoOp, WidenOp};
use crate::num_traits::{HasDoubleWidth, HasHalfWidth, UNum, UNum8To64};
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
    /// Number of full steps that have executed.
    pub full_step_count: u64,
}

impl Machine {
    pub fn init(elf_bytes: &[u8], init_opts: InitOpts) -> RResult<Machine> {
        let elf = SimpleElfFile::from_bytes(elf_bytes).map_err(Rerr::ElfParseError)?;
        let mut mem = Memory::new();
        let rsp = init_mem(&mut mem, &elf, init_opts)?;
        let mut regs = Registers {
            regs: [0_u64; 16],
            flags: Flags::new(),
            rip_prev: elf.e_entry,
            rip: elf.e_entry,
        };
        regs.set_reg64(&GPR64::rsp, rsp);
        Ok(Machine {
            regs,
            mem,
            full_step_count: 0,
        })
    }

    pub fn init_with_writers(
        writers: &mut Writers,
        elf_bytes: &[u8],
        init_opts: InitOpts,
    ) -> Option<Machine> {
        match Self::init(elf_bytes, init_opts) {
            Ok(x) => Some(x),
            Err(err) => {
                write!(writers.stderr(), "{}", err).expect("Write to stderr should not fail.");
                None
            }
        }
    }

    /// Return true if the execution loop should stop.
    pub fn full_step(&mut self, writers: &mut Writers) -> bool {
        let s = self.step(writers);
        let should_stop = match s {
            Ok(_) => false,
            Err(Rerr::SysExit(exit_code)) => {
                if exit_code != 0 {
                    write!(writers.stderr(), "exit status {exit_code}")
                        .expect("Write to stderr should not fail.")
                }
                true
            }
            Err(rerr) => {
                write!(writers.stderr(), "Detailed error: {}\n{}", rerr, self.regs)
                    .expect("Write to stderr should not fail.");
                true
            }
        };
        self.full_step_count += 1;
        should_stop
    }

    fn step(&mut self, writers: &mut Writers) -> RResult<()> {
        let (inst, len) = decode_inst(&self.mem, self.regs.rip)?;
        self.regs.rip_prev = self.regs.rip;
        self.regs.rip += len;
        // rip now points to the instruction to be executed after this one.
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
                        Addr32 => self.regs.get_reg32(&GPR32::ecx) as u64,
                        Addr64 => self.regs.get_reg64(&GPR64::rcx),
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
                            .set_reg32(&GPR32::ecx, (count as u32).wrapping_sub(1)),
                        Addr64 => self.regs.set_reg64(&GPR64::rcx, count.wrapping_sub(1)),
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
            Inst::LeaRegInsteadOfAddr => Err(Rerr::LeaRegInsteadOfAddr)?,
            Inst::RexNoop => {}
            Inst::Nop => {}
            Inst::Syscall => self.syscall(writers)?,
            Inst::Clc => self.regs.flags.cf = false,
            Inst::Stc => self.regs.flags.cf = true,
            Inst::LeaRM16(gpr16, eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.regs.set_reg16(gpr16, a as u16);
            }
            Inst::LeaRM32(gpr32, eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.regs.set_reg32(gpr32, a as u32);
            }
            Inst::LeaRM64(gpr64, eff_addr) => {
                let a = eff_addr.compute(&self.regs);
                self.regs.set_reg64(gpr64, a);
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
            Inst::Hlt => {
                Err(Rerr::Hlt)?;
            }
            Inst::OneRMInst8(OneOp::Plain(op), rm8) => {
                let old = self.get_rm8(rm8)?;
                let new = self.compute_result_onerm(op, old)?;
                self.set_rm8(rm8, new)?;
            }
            Inst::OneRMInst16(OneOp::Plain(op), rm16) => {
                let old = self.get_rm16(rm16)?;
                let new = self.compute_result_onerm(op, old)?;
                self.set_rm16(rm16, new)?;
            }
            Inst::OneRMInst32(OneOp::Plain(op), rm32) => {
                let old = self.get_rm32(rm32)?;
                let new = self.compute_result_onerm(op, old)?;
                self.set_rm32(rm32, new)?;
            }
            Inst::OneRMInst64(OneOp::Plain(op), rm64) => {
                let old = self.get_rm64(rm64)?;
                let new = self.compute_result_onerm(op, old)?;
                self.set_rm64(rm64, new)?;
            }
            Inst::TwoMRInst8(TwoOp::Dest(op), rm8, gpr8) => {
                let source = self.regs.get_reg8(gpr8);
                let old_dest = self.get_rm8(rm8)?;
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.set_rm8(rm8, new)?;
            }
            Inst::TwoRMInst8(TwoOp::Dest(op), gpr8, rm8) => {
                let source = self.get_rm8(rm8)?;
                let old_dest = self.regs.get_reg8(gpr8);
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.regs.set_reg8(gpr8, new);
            }
            Inst::TwoMRInst16(TwoOp::Dest(op), rm16, gpr16) => {
                let source = self.regs.get_reg16(gpr16);
                let old_dest = self.get_rm16(rm16)?;
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.set_rm16(rm16, new)?;
            }
            Inst::TwoRMInst16(TwoOp::Dest(op), gpr16, rm16) => {
                let source = self.get_rm16(rm16)?;
                let old_dest = self.regs.get_reg16(gpr16);
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.regs.set_reg16(gpr16, new);
            }
            Inst::TwoMRInst32(TwoOp::Dest(op), rm32, gpr32) => {
                let source = self.regs.get_reg32(gpr32);
                let old_dest = self.get_rm32(rm32)?;
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.set_rm32(rm32, new)?;
            }
            Inst::TwoRMInst32(TwoOp::Dest(op), gpr32, rm32) => {
                let source = self.get_rm32(rm32)?;
                let old_dest = self.regs.get_reg32(gpr32);
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.regs.set_reg32(gpr32, new);
            }
            Inst::TwoMRInst64(TwoOp::Dest(op), rm64, gpr64) => {
                let source = self.regs.get_reg64(gpr64);
                let old_dest = self.get_rm64(rm64)?;
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.set_rm64(rm64, new)?;
            }
            Inst::TwoRMInst64(TwoOp::Dest(op), gpr64, rm64) => {
                let source = self.get_rm64(rm64)?;
                let old_dest = self.regs.get_reg64(gpr64);
                let new = self.compute_result_tworm(op, old_dest, source)?;
                self.regs.set_reg64(gpr64, new);
            }
            Inst::TwoMIInst8(TwoOp::Dest(op), rm8, source) => {
                let old_dest = self.get_rm8(rm8)?;
                let new = self.compute_result_tworm(op, old_dest, *source)?;
                self.set_rm8(rm8, new)?;
            }
            Inst::TwoMIInst16(TwoOp::Dest(op), rm16, source) => {
                let old_dest = self.get_rm16(rm16)?;
                let new = self.compute_result_tworm(op, old_dest, *source)?;
                self.set_rm16(rm16, new)?;
            }
            Inst::TwoMIInst32(TwoOp::Dest(op), rm32, source) => {
                let old_dest = self.get_rm32(rm32)?;
                let new = self.compute_result_tworm(op, old_dest, *source)?;
                self.set_rm32(rm32, new)?;
            }
            Inst::TwoMIInst64(TwoOp::Dest(op), rm64, source) => {
                let old_dest = self.get_rm64(rm64)?;
                let new = self.compute_result_tworm(op, old_dest, *source)?;
                self.set_rm64(rm64, new)?;
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
                self.regs.flags.cf = val.bit(bit_ind).into()
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
            Inst::OneRMInst8(OneOp::Shrink(op), rm8) => {
                let dividend = self.regs.get_ax();
                let divisor = self.get_rm8(rm8)?;
                let (quotient, remainder) =
                    self.compute_result_shrink_onerm(op, dividend, divisor)?;
                self.regs.set_reg8(&GPR8::al, quotient);
                self.regs.set_reg8(&GPR8::ah, remainder);
            }
            Inst::OneRMInst16(OneOp::Shrink(op), rm16) => {
                let dividend = self.regs.get_dx_ax();
                let divisor = self.get_rm16(rm16)?;
                let (quotient, remainder) =
                    self.compute_result_shrink_onerm(op, dividend, divisor)?;
                self.regs.set_reg16(&GPR16::ax, quotient);
                self.regs.set_reg16(&GPR16::dx, remainder);
            }
            Inst::OneRMInst32(OneOp::Shrink(op), rm32) => {
                let dividend = self.regs.get_edx_eax();
                let divisor = self.get_rm32(rm32)?;
                let (quotient, remainder) =
                    self.compute_result_shrink_onerm(op, dividend, divisor)?;
                self.regs.set_reg32(&GPR32::eax, quotient);
                self.regs.set_reg32(&GPR32::edx, remainder);
            }
            Inst::OneRMInst64(OneOp::Shrink(op), rm64) => {
                let dividend = self.regs.get_rdx_rax();
                let divisor = self.get_rm64(rm64)?;
                let (quotient, remainder) =
                    self.compute_result_shrink_onerm(op, dividend, divisor)?;
                self.regs.set_reg64(&GPR64::rax, quotient);
                self.regs.set_reg64(&GPR64::rdx, remainder);
            }
            // ==============================================================
            Inst::OneRMInst8(OneOp::Widen(op), rm8) => {
                let x = self.regs.get_al();
                let y = self.get_rm8(rm8)?;
                let prod = self.compute_result_widen_onerm(op, x, y)?;
                self.regs.set_ax(prod);
            }
            Inst::OneRMInst16(OneOp::Widen(op), rm16) => {
                let x = self.regs.get_ax();
                let y = self.get_rm16(rm16)?;
                let prod = self.compute_result_widen_onerm(op, x, y)?;
                self.regs.set_dx_ax(prod);
            }
            Inst::OneRMInst32(OneOp::Widen(op), rm32) => {
                let x = self.regs.get_eax();
                let y = self.get_rm32(rm32)?;
                let prod = self.compute_result_widen_onerm(op, x, y)?;
                self.regs.set_edx_eax(prod);
            }
            Inst::OneRMInst64(OneOp::Widen(op), rm64) => {
                let x = self.regs.get_rax();
                let y = self.get_rm64(rm64)?;
                let prod = self.compute_result_widen_onerm(op, x, y)?;
                self.regs.set_rdx_rax(prod);
            }
            Inst::ImulRM16(gpr16, rm16) => {
                let x = self.regs.get_reg16(gpr16);
                let y = self.get_rm16(rm16)?;
                let prod = self.regs.flags.imul(x, y);
                self.regs.set_reg16(gpr16, prod as u16);
            }
            Inst::ImulRM32(gpr32, rm32) => {
                let x = self.regs.get_reg32(gpr32);
                let y = self.get_rm32(rm32)?;
                let prod = self.regs.flags.imul(x, y);
                self.regs.set_reg32(gpr32, prod as u32);
            }
            Inst::ImulRM64(gpr64, rm64) => {
                let x = self.regs.get_reg64(gpr64);
                let y = self.get_rm64(rm64)?;
                let prod = self.regs.flags.imul(x, y);
                self.regs.set_reg64(gpr64, prod as u64);
            }
            Inst::ImulRMI16(gpr16, rm16, imm16) => {
                let x = self.get_rm16(rm16)?;
                let prod = self.regs.flags.imul(x, *imm16);
                self.regs.set_reg16(gpr16, prod as u16);
            }
            Inst::ImulRMI32(gpr32, rm32, imm32) => {
                let x = self.get_rm32(rm32)?;
                let prod = self.regs.flags.imul(x, *imm32);
                self.regs.set_reg32(gpr32, prod as u32);
            }
            Inst::ImulRMI64(gpr64, rm64, imm64) => {
                let x = self.get_rm64(rm64)?;
                let prod = self.regs.flags.imul(x, *imm64);
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
            Inst::Ret => {
                self.regs.rip = self.pop_64()?;
            }
            Inst::CallD(addr) => {
                self.push_64(self.regs.rip)?;
                self.regs.rip = *addr;
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
                        self.regs.flags.sub(a, b);
                    }
                    DataSize::Data16 => {
                        let a = self.regs.get_reg16(&GPR16::ax);
                        let b = self.mem.read_u16(addr)?;
                        self.regs.flags.sub(a, b);
                    }
                    DataSize::Data32 => {
                        let a = self.regs.get_reg32(&GPR32::eax);
                        let b = self.mem.read_u32(addr)?;
                        self.regs.flags.sub(a, b);
                    }
                    DataSize::Data64 => {
                        let a = self.regs.get_reg64(&GPR64::rax);
                        let b = self.mem.read_u64(addr)?;
                        self.regs.flags.sub(a, b);
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
            Inst::RotateMI8(rot_pair, rm8, imm8) => {
                let val = self.get_rm8(rm8)?;
                let new = self.regs.flags.rotate(rot_pair, val, *imm8);
                self.set_rm8(rm8, new)?
            }
            Inst::RotateMI16(rot_pair, rm16, imm8) => {
                let val = self.get_rm16(rm16)?;
                let new = self.regs.flags.rotate(rot_pair, val, *imm8);
                self.set_rm16(rm16, new)?
            }
            Inst::RotateMI32(rot_pair, rm32, imm8) => {
                let val = self.get_rm32(rm32)?;
                let new = self.regs.flags.rotate(rot_pair, val, *imm8);
                self.set_rm32(rm32, new)?
            }
            Inst::RotateMI64(rot_pair, rm64, imm8) => {
                let val = self.get_rm64(rm64)?;
                let new = self.regs.flags.rotate(rot_pair, val, *imm8);
                self.set_rm64(rm64, new)?
            }
            Inst::RotateMC8(rot_pair, rm8) => {
                let val = self.get_rm8(rm8)?;
                let cl = self.regs.get_reg8(&GPR8::cl);
                let new = self.regs.flags.rotate(rot_pair, val, cl);
                self.set_rm8(rm8, new)?
            }
            Inst::RotateMC16(rot_pair, rm16) => {
                let val = self.get_rm16(rm16)?;
                let cl = self.regs.get_reg8(&GPR8::cl);
                let new = self.regs.flags.rotate(rot_pair, val, cl);
                self.set_rm16(rm16, new)?
            }
            Inst::RotateMC32(rot_pair, rm32) => {
                let val = self.get_rm32(rm32)?;
                let cl = self.regs.get_reg8(&GPR8::cl);
                let new = self.regs.flags.rotate(rot_pair, val, cl);
                self.set_rm32(rm32, new)?
            }
            Inst::RotateMC64(rot_pair, rm64) => {
                let val = self.get_rm64(rm64)?;
                let cl = self.regs.get_reg8(&GPR8::cl);
                let new = self.regs.flags.rotate(rot_pair, val, cl);
                self.set_rm64(rm64, new)?
            }
            Inst::XchgMR8(rm8, gpr8) => {
                let x = self.get_rm8(rm8)?;
                let y = self.regs.get_reg8(gpr8);
                // Note the rm8 has to be first because it implicitly
                // needs to calculate the effective address, depending on registers.
                self.set_rm8(rm8, y)?;
                self.regs.set_reg8(gpr8, x);
            }
            Inst::XchgMR16(rm16, gpr16) => {
                let x = self.get_rm16(rm16)?;
                let y = self.regs.get_reg16(gpr16);
                // Note the rm8 has to be first because it implicitly
                // needs to calculate the effective address, depending on registers.
                self.set_rm16(rm16, y)?;
                self.regs.set_reg16(gpr16, x);
            }
            Inst::XchgMR32(rm32, gpr32) => {
                let x = self.get_rm32(rm32)?;
                let y = self.regs.get_reg32(gpr32);
                // Note the rm8 has to be first because it implicitly
                // needs to calculate the effective address, depending on registers.
                self.set_rm32(rm32, y)?;
                self.regs.set_reg32(gpr32, x);
            }
            Inst::XchgMR64(rm64, gpr64) => {
                let x = self.get_rm64(rm64)?;
                let y = self.regs.get_reg64(gpr64);
                // Note the rm8 has to be first because it implicitly
                // needs to calculate the effective address, depending on registers.
                self.set_rm64(rm64, y)?;
                self.regs.set_reg64(gpr64, x);
            }
            Inst::Cwd16 => {
                let s = self.regs.get_ax().sign();
                self.regs.set_reg16(&GPR16::dx, s.fill());
            }
            Inst::Cdq32 => {
                let s = self.regs.get_eax().sign();
                self.regs.set_reg32(&GPR32::edx, s.fill());
            }
            Inst::Cqo64 => {
                let s = self.regs.get_rax().sign();
                self.regs.set_reg64(&GPR64::rdx, s.fill());
            }
            Inst::Cbw16 => {
                let x = self.regs.get_al();
                self.regs.set_ax(x.sign_extend_double_width())
            }
            Inst::Cwde32 => {
                let x = self.regs.get_ax();
                self.regs.set_eax(x.sign_extend_double_width())
            }
            Inst::Cdqe64 => {
                let x = self.regs.get_eax();
                self.regs.set_rax(x.sign_extend_double_width())
            }
            Inst::BswapO32(gpr32) => {
                let x = self.regs.get_reg32(gpr32);
                self.regs.set_reg32(gpr32, x.swap_bytes());
            }
            Inst::BswapO64(gpr64) => {
                let x = self.regs.get_reg64(gpr64);
                self.regs.set_reg64(gpr64, x.swap_bytes());
            }
        }
        Ok(())
    }

    fn compute_result_onerm<U: UNum>(&mut self, op: &PlainOneOp, x: U) -> RResult<U> {
        Ok(match op {
            PlainOneOp::Inc => self.regs.flags.inc(x),
            PlainOneOp::Dec => self.regs.flags.dec(x),
            PlainOneOp::Not => !x,
        })
    }

    fn compute_result_shrink_onerm<U: UNum8To64>(
        &mut self,
        op: &ShrinkOp,
        dividend: U::DoubleWidth,
        divisor: U,
    ) -> RResult<(U, U)> {
        Ok(match op {
            ShrinkOp::Div => {
                if divisor == 0.into() {
                    Err(Rerr::DivideError)?;
                }
                self.regs.flags.div_flags();
                let quotient = dividend / divisor.zero_extend_double_width();
                let remainder = dividend % divisor.zero_extend_double_width();
                // let quotient_u: U
                let quotient_u = quotient
                    .try_into_half_width()
                    .map_err(|_| Rerr::DivideError)?;
                (quotient_u, remainder.truncate_to_half_width())
            }
        })
    }

    fn compute_result_widen_onerm<U: UNum8To64>(
        &mut self,
        op: &WidenOp,
        x: U,
        y: U,
    ) -> RResult<U::DoubleWidth> {
        Ok(match op {
            WidenOp::Imul => self.regs.flags.imul(x, y),
        })
    }

    fn compute_result_tworm<U: UNum>(
        &mut self,
        op: &DestMROp,
        old_dest: U,
        source: U,
    ) -> RResult<U> {
        Ok(match op {
            DestMROp::Mov => source,
            DestMROp::Add => self.regs.flags.add(old_dest, source),
            DestMROp::Adc => self.regs.flags.adc(old_dest, source),
            DestMROp::Sbb => self.regs.flags.sbb(old_dest, source),
            DestMROp::Sub => self.regs.flags.sub(old_dest, source),
            DestMROp::Cmp => {
                self.regs.flags.sub(old_dest, source);
                // Don't change the value.
                old_dest
            }
            DestMROp::And => self.regs.flags.and(old_dest, source),
            DestMROp::Or => self.regs.flags.or(old_dest, source),
            DestMROp::Xor => self.regs.flags.xor(old_dest, source),
        })
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
        let rflags = self.regs.get_rflags();
        self.regs.set_reg64(&GPR64::r11, rflags);
        let ret = match eax {
            0 => 0,
            1 => self.sys_write(writers)?,
            60 => self.sys_exit()?,
            // Full list of allowed syscalls can be obtained by
            // curl https://raw.githubusercontent.com/code-golf/code-golf/refs/heads/master/run-lang.c \
            //  | grep '  ALLOW' | sed -E 's#.*// | \(.*\)##g' | sort -n | tr $'\n' '|'
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19
            | 20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 | 32 | 33 | 34 | 35
            | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 44 | 45 | 46 | 47 | 48 | 49 | 50 | 51 | 52
            | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 61 | 62 | 63 | 67 | 72 | 73 | 74 | 75 | 76
            | 77 | 79 | 80 | 81 | 82 | 83 | 84 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95
            | 96 | 97 | 98 | 99 | 100 | 101 | 102 | 104 | 105 | 106 | 107 | 108 | 109 | 110
            | 111 | 112 | 113 | 114 | 115 | 116 | 117 | 118 | 119 | 120 | 121 | 122 | 123 | 124
            | 125 | 126 | 127 | 128 | 129 | 130 | 131 | 132 | 133 | 136 | 137 | 138 | 139 | 140
            | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 | 151 | 152 | 154 | 157
            | 158 | 160 | 162 | 173 | 186 | 194 | 200 | 201 | 202 | 203 | 204 | 205 | 206 | 207
            | 208 | 209 | 210 | 211 | 213 | 217 | 218 | 219 | 221 | 222 | 223 | 224 | 225 | 226
            | 227 | 228 | 229 | 230 | 231 | 232 | 233 | 234 | 235 | 237 | 238 | 239 | 247 | 251
            | 252 | 253 | 254 | 255 | 257 | 258 | 259 | 260 | 261 | 262 | 263 | 264 | 267 | 268
            | 269 | 270 | 271 | 273 | 274 | 275 | 276 | 277 | 278 | 280 | 281 | 282 | 283 | 284
            | 285 | 286 | 287 | 288 | 289 | 290 | 291 | 292 | 293 | 294 | 295 | 296 | 297 | 299
            | 302 | 303 | 304 | 305 | 306 | 307 | 314 | 315 | 316 | 318 | 319 | 322 | 323 | 324
            | 325 | 326 | 327 | 328 | 329 | 330 | 331 | 332 | 333 | 334 | 424 | 425 | 426 | 427
            | 434 | 435 | 436 | 437 | 438 | 439 | 441 | 447 | 451 | 452 => {
                Err(Rerr::UnimplementedSyscall(eax))?
            }
            _ => Err(Rerr::IllegalSyscall(eax))?,
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

    fn sys_exit(&mut self) -> RResult<u64> {
        let exit_code = self.regs.get_reg8(&GPR8::dil);
        Err(Rerr::SysExit(exit_code))
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
