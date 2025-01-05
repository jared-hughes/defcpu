use defcpu_core::{interpret::Machine, read_write::Writers};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OuterMachine {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    done: bool,
    machine: Option<Machine>,
}
#[wasm_bindgen]
impl OuterMachine {
    #[wasm_bindgen]
    pub fn init(input: &[u8]) -> OuterMachine {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut writers = Writers {
            stdout: &mut stdout,
            stderr: &mut stderr,
        };
        let machine = Machine::from_elf_bytes_with_writers(input, &mut writers);
        OuterMachine {
            stdout,
            stderr,
            done: machine.is_none(),
            machine,
        }
    }

    #[wasm_bindgen]
    pub fn step(&mut self) {
        if self.done {
            return;
        }
        let mut machine = &mut self.machine;
        let mut stdout = &mut self.stdout;
        let mut stderr = &mut self.stderr;
        self.done = match &mut machine {
            Some(ref mut m) => m.full_step(&mut Writers {
                stdout: &mut stdout,
                stderr: &mut stderr,
            }),
            None => true,
        };
    }

    #[wasm_bindgen]
    pub fn is_done(&self) -> bool {
        self.done
    }

    #[wasm_bindgen]
    pub fn get_stdout(&self) -> Vec<u8> {
        self.stdout.clone()
    }
    #[wasm_bindgen]
    pub fn get_stderr(&self) -> Vec<u8> {
        self.stderr.clone()
    }
}
