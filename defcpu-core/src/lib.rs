pub(crate) mod decode_inst;
pub(crate) mod interpret;
pub(crate) mod memory;
pub(crate) mod parse_elf;
pub(crate) mod registers;

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
