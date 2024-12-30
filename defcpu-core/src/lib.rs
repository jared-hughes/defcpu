// Every time I switch to `matches!`, I end up switching back.
#![allow(clippy::match_like_matches_macro)]

pub(crate) mod decode_inst;
pub(crate) mod inst;
pub(crate) mod inst_prefixes;
pub(crate) mod interpret;
pub(crate) mod memory;
pub(crate) mod parse_elf;
pub(crate) mod read_write;
pub(crate) mod registers;
use crate::decode_inst::decode_inst;

use interpret::Machine;
use parse_elf::SimpleElfFile;

pub fn interpret(input: &[u8]) {
    let elf = SimpleElfFile::from_bytes(input).unwrap_or_else(|pe| panic!("{}", pe));
    let mut machine = Machine::from_elf(&elf);
    let max_steps = 100000;
    let mut step_index = 0;
    while step_index < max_steps && !machine.halt {
        machine.step();
        step_index += 1;
    }
}

pub fn disassemble(input: &[u8]) {
    let elf = SimpleElfFile::from_bytes(input).unwrap_or_else(|pe| panic!("{}", pe));
    let machine = Machine::from_elf(&elf);
    let max_steps = 1000;
    let mut step_index = 0;
    let mut i = machine.regs.rip;
    while step_index < max_steps {
        if machine.mem.read_u8(i) == 0 {
            break;
        }
        let (inst, len) = decode_inst(&machine.mem, i);
        let mut first = true;
        for j in i..i + len {
            if !first {
                print!(" ");
            }
            first = false;
            print!("{:02x}", machine.mem.read_u8(j));
        }
        print!("\t");
        i += len;
        println!("{}", inst);
        step_index += 1;
    }
}
