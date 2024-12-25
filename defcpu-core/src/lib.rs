pub(crate) mod mem;
pub(crate) mod parse_elf;

use mem::Memory;
use parse_elf::SimpleElfFile;

pub fn interpret(input: &[u8]) {
    let elf = SimpleElfFile::from_bytes(input).unwrap_or_else(|pe| panic!("{}", pe));
    let mut mem = Memory::from_segments(elf.segments);
    println!("{}", mem);
    println!("Entry byte: 0x{:02x}", mem.read_byte(elf.e_entry));
    mem.write_byte(elf.e_entry, 0x42);
    println!(
        "Overwrote it with 0x42. Now it's 0x{:02x}",
        mem.read_byte(elf.e_entry)
    );
    println!();
    println!("{}", mem);
}
