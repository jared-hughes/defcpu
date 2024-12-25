use std::fmt;

pub(crate) trait Diagnostic {
    fn into_diag(self) -> Diag;
}

#[must_use]
#[derive(Clone)]
pub struct Diag {
    pub(crate) message: String,
    pub(crate) ip: u64,
}

impl fmt::Debug for Diag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "At 0x{:016x}: {}", self.ip, &self.message)
    }
}

macro_rules! def_errors {
    ($(
        pub(crate) struct $Err:ident $def:tt
        msg: $self:ident => $msg:expr;
    )+) => {$(
        pub(crate) struct $Err $def
        impl Diagnostic for $Err {
            fn into_diag($self) -> Diag {
                Diag {
                    ip: $self.ip,
                    message: $msg,
                }
            }
        }
    )+};
}

def_errors! {
pub(crate) struct EmptyELF {
    /// Always 0
    pub(crate) ip: u64,
}
msg: self => format!("ELF file is empty.");
}
