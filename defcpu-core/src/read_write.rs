use File::*;
pub(crate) enum File {
    Stdout,
    Stderr,
}
impl File {
    pub(crate) fn from_fd(fd: u32) -> File {
        match fd {
            1 => Stdout,
            2 => Stderr,
            _ => panic!("Unknown fd: {}", fd),
        }
    }

    // TODO: This is probably incredibly inefficient.
    // I don't understand Rust I/O at all.
    pub(crate) fn write_byte(&self, byte: u8) {
        match self {
            Stdout => print!("{}", byte as char),
            Stderr => eprint!("{}", byte as char),
        }
    }
}
