use defcpu_core::{
    disassemble, interpret_to_streams, InitOpts, InitUnpredictables, SideData, Unpredictables,
};

use std::{fmt, path::PathBuf, str::FromStr};
use structopt::StructOpt;

#[derive(Debug)]
struct Rand16([u8; 16]);

#[derive(Debug)]
enum Rand16Err {
    FromHex(hex::FromHexError),
    WrongLen(usize),
    BadFromSlice(std::array::TryFromSliceError),
}
impl fmt::Display for Rand16Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rand16Err::FromHex(e) => write!(f, "{e}"),
            Rand16Err::WrongLen(e) => write!(f, "Need 32 hex digits, but got {}", 2 * e),
            Rand16Err::BadFromSlice(e) => write!(f, "{e}"),
        }
    }
}
impl FromStr for Rand16 {
    type Err = Rand16Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let y = hex::decode(s).map_err(Rand16Err::FromHex)?;
        if y.len() != 16 {
            return Err(Rand16Err::WrongLen(y.len()));
        }
        let tup: &[u8; 16] = y[..].try_into().map_err(Rand16Err::BadFromSlice)?;
        Ok(Rand16(*tup))
    }
}

#[derive(Debug)]
struct Hex64(u64);

#[derive(Debug)]
enum Hex64Err {
    Missing0x,
    ParseInt(std::num::ParseIntError),
}
impl fmt::Display for Hex64Err {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Hex64Err::ParseInt(e) => write!(f, "{e}"),
            Hex64Err::Missing0x => write!(f, "Missing 0x prefix."),
        }
    }
}
impl FromStr for Hex64 {
    type Err = Hex64Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digs = s.strip_prefix("0x").ok_or(Hex64Err::Missing0x)?;
        let num = u64::from_str_radix(digs, 16).map_err(Hex64Err::ParseInt)?;
        Ok(Hex64(num))
    }
}

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

        /// Arguments vector. Remember to pass /tmp/asm as first arg.
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
            argv,
            envp,
        } => {
            let contents = read_file(input);
            let side_data = SideData {
                argv: argv.unwrap_or(vec!["/tmp/asm".to_string()]),
                envp: envp.unwrap_or(vec![]),
            };
            let init_unp = match (random_seed, rand16, vdso_ptr, execfn_ptr, platform_offset) {
                (None, None, None, None, None) => {
                    eprintln!("Must pass either --random-seed or --rand16 --vdso-ptr --execfn-ptr --platform-ptr.");
                    return;
                }
                (Some(random_seed), None, None, None, None) => {
                    InitUnpredictables::Random(random_seed)
                }
                (None, Some(rand16), Some(vdso_ptr), Some(execfn_ptr), Some(platform_offset)) => {
                    InitUnpredictables::Fixed(Unpredictables {
                        random_16_bytes: rand16.0,
                        vdso_ptr: vdso_ptr.0,
                        // TODO-unp: this is slightly wrong
                        stack_top: execfn_ptr.0.next_multiple_of(0x1000),
                        argv0_to_platform_offset: platform_offset.0,
                    })
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
