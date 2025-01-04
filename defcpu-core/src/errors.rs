use std::fmt;

use elf::ParseError;

pub type RResult<T> = Result<T, Rerr>;

pub enum Rerr {
    ElfParseError(ParseError),
    DivideError,
    /// Described in "push" docs. Causes a double-fault and logical processer shutdown.
    StackFault,
    WriteOutsideSegment(u64),
    ReadOutsideSegment(u64),
    SegmentNotWriteable(u64),
    SegmentNotReadable(u64),
    NotImplemented(u8),
    NotImplemented2(u8, u8),
    NotImplementedOpext(u8, u8),
    UnimplementedSyscall(u32),
    UnknownFileDescriptor(u32),
}

impl fmt::Display for Rerr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElfParseError(pe) => write!(f, "Error when parsing ELF: {}", pe),
            Self::DivideError => write!(f, "Divide Error #DE."),
            Self::StackFault => write!(f, "Stack Fault Exception #SS."),
            Self::WriteOutsideSegment(addr) => write!(
                f,
                "Segmentation fault: write to address {addr:#016X} outside every segment."
            ),
            Self::ReadOutsideSegment(addr) => write!(
                f,
                "Segmentation fault: read of address {addr:#016X} outside every segment."
            ),
            Self::SegmentNotWriteable(addr) => write!(
                f,
                "Segmentation fault: address {addr:#016X} is not writeable."
            ),
            Self::SegmentNotReadable(addr) => write!(
                f,
                "Segmentation fault: address {addr:#016X} is not readable."
            ),
            Self::NotImplemented(opcode) => {
                write!(f, "Not yet implemented opcode {opcode:02x}.")
            }
            Self::NotImplemented2(opcode, opcode2) => {
                write!(f, "Not yet implemented opcode {opcode:02x} {opcode2:02x}.")
            }
            Self::NotImplementedOpext(opcode, sub) => {
                write!(f, "Not yet implemented opcode {opcode:02x} /{sub}.")
            }
            Self::UnimplementedSyscall(eax) => {
                write!(f, "Not yet implemented syscall %eax={eax}.")
            }
            Self::UnknownFileDescriptor(fd) => {
                write!(f, "[sys_write] Unknown file descriptor fd=%edi={fd}")
            }
        }
    }
}
