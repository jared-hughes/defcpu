use std::fmt;

use elf::ParseError;

pub type RResult<T> = Result<T, Rerr>;

// Some of these should be signals, some should be faults. I'm just willy-nillying it right now.
pub enum Rerr {
    ElfParseError(ParseError),
    Hlt,
    SysExit(u8),
    DivideError,
    /// Converting rust string to C-style string went bad. An extra null. See ffi::NulError
    NulError,
    /// Described in "push" docs. Causes a double-fault and logical processer shutdown.
    StackFault,
    WriteOutsideSegment(u64),
    ReadOutsideSegment(u64),
    SegmentNotWriteable(u64),
    SegmentNotReadable(u64),
    NotImplemented(u8),
    NotImplemented2(u8, u8),
    NotImplementedOpext(u8, u8),
    LeaRegInsteadOfAddr,
    UnimplementedSyscall(u32),
    IllegalSyscall(u32),
    Syscall(SyscallError),
}

pub enum SyscallError {
    CapacityExceeded(u64),
    UnknownFileDescriptor(u32),
}

impl fmt::Display for Rerr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ElfParseError(pe) => write!(f, "ELF parse error: {}", pe),
            Self::DivideError => write!(f, "Divide Error #DE."),
            Self::NulError => write!(f, "A Rust string couldn't convert to a C string"),
            Self::StackFault => write!(f, "Stack Fault Exception #SS."),
            Self::SysExit(exit_code) => write!(f, "Clean exit with exit code {exit_code}."),
            Self::Hlt => write!(f, "Hlt (F4) instruction executed."),
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
            Self::LeaRegInsteadOfAddr => {
                write!(
                    f,
                    "Register passed for LEA instead of effective address computation."
                )
            }
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
            Self::IllegalSyscall(fd) => {
                write!(f, "Syscall %eax={fd} is disallowed or does not exit.")
            }
            Self::Syscall(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl fmt::Display for SyscallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CapacityExceeded(size) => {
                write!(f, "[sys_write] Count {size:#x} = {size} exceeds capacity.")
            }
            Self::UnknownFileDescriptor(fd) => {
                write!(f, "[sys_write] Unknown file descriptor fd=%edi={fd}")
            }
        }
    }
}
