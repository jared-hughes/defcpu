// Every time I switch to `matches!`, I end up switching back.
#![allow(clippy::match_like_matches_macro)]

pub(crate) mod decode_inst;
pub(crate) mod errors;
pub(crate) mod init_mem;
pub(crate) mod inst;
pub(crate) mod inst_prefixes;
pub mod interpret;
pub(crate) mod memory;
pub(crate) mod num_traits;
pub(crate) mod num_u1;
pub(crate) mod parse_elf;
pub mod read_write;
pub(crate) mod registers;

use crate::decode_inst::decode_inst;

use errors::{RResult, Rerr};
use init_mem::init_program_segments;
use interpret::Machine;
use memory::Memory;
use parse_elf::SimpleElfFile;
use read_write::Writers;

pub use init_mem::{InitOpts, InitUnpredictables, SideData};

pub fn interpret_to_streams(elf_bytes: &[u8], init_opts: InitOpts) {
    let mut stdout = std::io::stdout();
    let mut stderr = std::io::stderr();
    let writers = &mut Writers {
        stdout: &mut stdout,
        stderr: &mut stderr,
    };
    let Some(mut machine) = Machine::init_with_writers(writers, elf_bytes, init_opts) else {
        // Expect the `init_with_writers` to log about any problems.
        return;
    };
    let max_steps: u64 = 0xFFFFFFFF;
    let mut step_index = 0;
    while step_index < max_steps {
        let should_stop = machine.full_step(writers);
        if should_stop {
            return;
        }
        step_index += 1;
    }
    write!(
        writers.stderr(),
        "Arbitrary limit of {max_steps} instruction executions exceeded, giving up."
    )
    .expect("Write to stderr should not fail.");
}

pub fn disassemble(input: &[u8]) -> RResult<()> {
    let elf = SimpleElfFile::from_bytes(input).map_err(Rerr::ElfParseError)?;
    let mut mem = Memory::new();
    init_program_segments(&mut mem, &elf);
    for segment in elf.segments {
        if !segment.flags.executable {
            continue;
        }
        disassemble_segment(&mem, segment.p_vaddr, segment.segment_data.len() as u64)?;
    }
    Ok(())
}

fn disassemble_segment(mem: &Memory, v_addr: u64, segment_len: u64) -> RResult<()> {
    let mut i = v_addr;
    while i < v_addr + segment_len {
        let (inst, len) = decode_inst(mem, i)?;
        if len == 0 {
            panic!("Empty instruction.")
        }
        let mut first = true;
        for j in i..i + len {
            if !first {
                print!(" ");
            }
            first = false;
            print!("{:02x}", mem.read_u8(j)?);
        }
        print!("\t");
        i += len;
        println!("{}", inst);
    }
    Ok(())
}
