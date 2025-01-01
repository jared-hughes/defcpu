use defcpu_core::interpret;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(elf: &[u8]) {
    interpret(elf);
}
