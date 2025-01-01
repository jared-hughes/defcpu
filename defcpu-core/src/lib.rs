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
use memory::Memory;
use parse_elf::SimpleElfFile;
use read_write::Writers;

pub fn interpret_to_streams(input: &[u8]) {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let mut writers = Writers {
        stdout: &mut stdout,
        stderr: &mut stderr,
    };
    interpret(input, &mut writers);
}

pub struct VecOutput {
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
}

pub fn interpret_to_vecs(input: &[u8]) -> VecOutput {
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let mut writers = Writers {
        stdout: &mut stdout,
        stderr: &mut stderr,
    };
    interpret(input, &mut writers);
    VecOutput { stdout, stderr }
}

fn interpret(input: &[u8], writers: &mut Writers) {
    let elf = SimpleElfFile::from_bytes(input).unwrap_or_else(|pe| panic!("{}", pe));
    let mut machine = Machine::from_elf(&elf);
    let max_steps = 100000;
    let mut step_index = 0;
    while step_index < max_steps && !machine.halt {
        machine.step(writers);
        step_index += 1;
    }
}

pub fn disassemble(input: &[u8]) {
    let elf = SimpleElfFile::from_bytes(input).unwrap_or_else(|pe| panic!("{}", pe));
    let mem = Memory::from_segments(&elf.segments);
    for segment in elf.segments {
        if !segment.flags.executable {
            continue;
        }
        disassemble_segment(&mem, segment.p_vaddr, segment.segment_data.len() as u64);
    }
}

fn disassemble_segment(mem: &Memory, v_addr: u64, segment_len: u64) {
    let mut i = v_addr;
    while i < v_addr + segment_len {
        let (inst, len) = decode_inst(mem, i);
        if len == 0 {
            panic!("Empty instruction.")
        }
        let mut first = true;
        for j in i..i + len {
            if !first {
                print!(" ");
            }
            first = false;
            print!("{:02x}", mem.read_u8(j));
        }
        print!("\t");
        i += len;
        println!("{}", inst);
    }
}
