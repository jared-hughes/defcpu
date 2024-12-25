pub(crate) mod diag;

use diag::{Diag, Diagnostic};

pub fn interpret(input: &[u8]) -> Result<u64, Diag> {
    if !input.is_empty() {
        Ok(input[0] as u64)
    } else {
        Err((diag::EmptyELF { ip: 0 }).into_diag())
    }
}
