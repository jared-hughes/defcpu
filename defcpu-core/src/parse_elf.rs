use std::fmt;

use elf::endian::LittleEndian;
use elf::file::Class;
use elf::segment::ProgramHeader;
use elf::{ElfBytes, ParseError};

// reference `man elf` (5), `readelf -l`

const ET_EXEC: u16 = 2;

const PT_LOAD: u32 = 1;

/// Executable
const PF_X: u32 = 1;
/// Writeable
const PF_W: u32 = 2;
/// Readable
const PF_R: u32 = 4;

#[derive(Debug, Clone, Copy)]
pub struct PermissionFlags {
    pub executable: bool,
    pub writeable: bool,
    pub readable: bool,
}

impl fmt::Display for PermissionFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.readable { 'R' } else { ' ' })?;
        write!(f, "{}", if self.writeable { 'W' } else { ' ' })?;
        write!(f, "{}", if self.executable { 'E' } else { ' ' })
    }
}

#[derive(Debug)]
pub struct LoadSegment<'a> {
    pub flags: PermissionFlags,
    /// Virtual address is what the software sees.
    pub p_vaddr: u64,
    /// Bytes to fill in.
    pub segment_data: &'a [u8],
}

impl<'a> LoadSegment<'a> {
    pub fn from_phdr(
        phdr: &ProgramHeader,
        file: &ElfBytes<'a, LittleEndian>,
    ) -> Result<LoadSegment<'a>, ParseError> {
        if phdr.p_type != PT_LOAD {
            panic!(
                "Unexpected p_type = {}. Expected PT_LOAD ({}).",
                phdr.p_type, PT_LOAD
            )
        }
        if phdr.p_paddr != phdr.p_vaddr {
            // I'm 50% sure we could just ignore physical addresses for a
            // software-only implementation. Leaving this restriction for now.
            panic!("Physical address differs from virtual address. Giving up.")
        }
        let segment_data = file.segment_data(phdr)?;

        Ok(LoadSegment {
            p_vaddr: phdr.p_vaddr,
            flags: PermissionFlags {
                executable: phdr.p_flags & PF_X != 0,
                writeable: phdr.p_flags & PF_W != 0,
                readable: phdr.p_flags & PF_R != 0,
            },
            segment_data,
        })
    }
}

#[derive(Debug)]
pub struct SimpleElfFile<'a> {
    pub e_entry: u64,
    pub segments: Vec<LoadSegment<'a>>,
}

impl<'a> SimpleElfFile<'a> {
    pub fn from_bytes(slice: &'a [u8]) -> Result<SimpleElfFile<'a>, ParseError> {
        // Assume Little Endian.
        // TODO verify it complains about Big Endian.
        let file = ElfBytes::<LittleEndian>::minimal_parse(slice).expect("Minimal parse");

        if file.ehdr.class != Class::ELF64 {
            panic!("Invalid class (ELF32?). Expected ELF64.")
        }
        if file.ehdr.e_type != ET_EXEC {
            panic!(
                "Unexpected e_type = {}. Expected executable ({}).",
                file.ehdr.e_type, ET_EXEC
            );
        }
        if file.ehdr.e_entry == 0 {
            panic!("Missing entry point (e_entry = 0).")
        }

        let mut out = SimpleElfFile {
            e_entry: file.ehdr.e_entry,
            segments: vec![],
        };

        // The segment table is a bunch of program headers, which describe segments.
        let segment_table = file.segments().expect("Found no segments.");
        for phdr in segment_table.iter() {
            let segment = LoadSegment::from_phdr(&phdr, &file)?;
            out.segments.push(segment);
        }
        // TODO: complain if two segments overlap in a page.
        Ok(out)
    }
}
