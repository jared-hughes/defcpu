use std::io::Write;

use File::*;
enum File {
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
}

pub(crate) struct VecWriters<'a> {
    pub(crate) stdout: &'a mut Vec<u8>,
    pub(crate) stderr: &'a mut Vec<u8>,
}
impl VecWriters<'_> {
    fn write_all(&mut self, file: File, buf: &[u8]) {
        match file {
            Stdout => self.stdout.extend_from_slice(buf),
            Stderr => self.stderr.extend_from_slice(buf),
        }
    }
}

pub(crate) struct StreamWriters {}
impl StreamWriters {
    fn write_all(&self, file: File, buf: &[u8]) -> Result<(), std::io::Error> {
        match file {
            Stdout => std::io::stdout().write_all(buf),
            Stderr => std::io::stderr().write_all(buf),
        }
    }
}

pub(crate) enum Writers<'a> {
    VecWriters(VecWriters<'a>),
    StreamWriters(StreamWriters),
}
impl Writers<'_> {
    pub(crate) fn write_all(&mut self, fd: u32, buf: &[u8]) {
        let file = File::from_fd(fd);
        match self {
            Writers::VecWriters(vec_output) => vec_output.write_all(file, buf),
            Writers::StreamWriters(stream_output) => stream_output
                .write_all(file, buf)
                .expect("Write fully succeeded."),
        }
    }
}
