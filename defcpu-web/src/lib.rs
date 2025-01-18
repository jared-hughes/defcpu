use defcpu_core::{
    interpret::Machine, read_write::Writers, InitOpts, InitUnpredictables, SideData,
};
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
    pub fn init(
        elf_bytes: &[u8],
        argv: Vec<String>,
        envp: Vec<String>,
        unp_seed: u64,
    ) -> OuterMachine {
        let mut stdout = Vec::new();
        let mut stderr = Vec::new();
        let mut writers = Writers {
            stdout: &mut stdout,
            stderr: &mut stderr,
        };
        let init_opts = InitOpts {
            side_data: SideData { argv, envp },
            init_unp: InitUnpredictables::Random(unp_seed),
        };
        let machine = Machine::init_with_writers(&mut writers, elf_bytes, init_opts);
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

    #[wasm_bindgen]
    pub fn get_registers_str(&self) -> Vec<u8> {
        match self.machine {
            Some(ref machine) => format!("{}", machine.regs).as_bytes().to_vec(),
            None => Vec::new(),
        }
    }

    #[wasm_bindgen]
    pub fn get_rip(&self) -> u64 {
        match self.machine {
            Some(ref machine) => machine.regs.rip,
            None => 0,
        }
    }

    #[wasm_bindgen]
    pub fn get_full_step_count(&self) -> u64 {
        match self.machine {
            Some(ref machine) => machine.full_step_count,
            None => 0,
        }
    }
}
