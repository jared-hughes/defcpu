use defcpu_core::{
    disassemble, gen_unpredictables, interpret_to_streams,
    parse_args::{Hex64, Rand16},
    InitOpts, SideData, Unpredictables,
};

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "defcpu", about = "DefCPU cli. Disassemble or emulate ELFs.")]
enum DefCPUOpt {
    Dis {
        /// Input file, positional.
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
    Run {
        /// Input file, positoinal
        #[structopt(parse(from_os_str))]
        input: PathBuf,

        /// Random seed, for when explicit parameters aren't passed
        #[structopt(long)]
        random_seed: Option<u64>,

        /// 16 bytes of user seeding, given as 32 nibbles.
        #[structopt(long)]
        rand16: Option<Rand16>,

        /// Pointer to the vDSO page.
        #[structopt(long)]
        vdso_ptr: Option<Hex64>,

        /// Pointer to the execfn string "/tmp/asm".
        #[structopt(long)]
        execfn_ptr: Option<Hex64>,

        /// Distance from the nul of the platform string "x86_64" to &argv[0].
        #[structopt(long)]
        platform_offset: Option<Hex64>,

        /// Zeroth argument.
        #[structopt(long, default_value = "/tmp/asm")]
        arg0: String,

        /// Arguments vector.
        #[structopt(long)]
        argv: Option<Vec<String>>,

        /// Environment vector. Can be empty.
        #[structopt(long)]
        envp: Option<Vec<String>>,
    },
}

fn read_file(path: PathBuf) -> Vec<u8> {
    std::fs::read(path).expect("File should exist.")
}

fn main() {
    let opt = DefCPUOpt::from_args();
    match opt {
        DefCPUOpt::Dis { input } => {
            let contents = read_file(input);
            disassemble(&contents).unwrap_or_else(|rerr| eprintln!("{}", rerr));
        }
        // TODO-cli: split this into run-seeded and run-fixed? Nested subcommands?
        DefCPUOpt::Run {
            random_seed,
            rand16,
            vdso_ptr,
            execfn_ptr,
            platform_offset,
            input,
            arg0,
            argv,
            envp,
        } => {
            let contents = read_file(input);
            let mut real_argv = vec![arg0];
            real_argv.extend_from_slice(&argv.unwrap_or(vec![]));
            let side_data = SideData {
                argv: real_argv,
                envp: envp.unwrap_or(vec![]),
            };
            let init_unp = match (random_seed, rand16, vdso_ptr, execfn_ptr, platform_offset) {
                (None, None, None, None, None) => {
                    eprintln!("Must pass either --random-seed or --rand16 --vdso-ptr --execfn-ptr --platform-ptr.");
                    return;
                }
                (Some(random_seed), None, None, None, None) => gen_unpredictables(random_seed),
                (None, Some(rand16), Some(vdso_ptr), Some(execfn_ptr), Some(platform_offset)) => {
                    Unpredictables {
                        random_16_bytes: rand16.0,
                        vdso_ptr: vdso_ptr.0,
                        // TODO-unp: this is slightly wrong
                        stack_top: execfn_ptr.0.next_multiple_of(0x1000),
                        platform_offset: platform_offset.0,
                    }
                }
                (None, _, _, _, _) => {
                    eprintln!("Must pass all four options --rand16 --vdso-ptr --execfn-ptr --platform-ptr, if --random-seed is not passed.");
                    return;
                }
                _ => {
                    eprintln!("Cannot pass both --random-seed and one of the four options --rand16 --vdso-ptr --execfn-ptr --platform-ptr.");
                    return;
                }
            };
            let init_opts = InitOpts {
                side_data,
                init_unp,
            };
            interpret_to_streams(&contents, init_opts);
        }
    }
}
