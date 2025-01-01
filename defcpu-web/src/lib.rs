use defcpu_core::interpret_to_vecs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct VecOutputJS {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

#[wasm_bindgen]
impl VecOutputJS {
    #[wasm_bindgen]
    pub fn get_stdout(&self) -> Vec<u8> {
        self.stdout.clone()
    }
    #[wasm_bindgen]
    pub fn get_stderr(&self) -> Vec<u8> {
        self.stderr.clone()
    }
}

#[wasm_bindgen]
pub fn run(elf: &[u8]) -> VecOutputJS {
    let k = interpret_to_vecs(elf);
    VecOutputJS {
        stdout: k.stdout,
        stderr: k.stderr,
    }
}
