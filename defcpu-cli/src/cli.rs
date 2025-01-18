use defcpu_core::{disassemble, interpret_to_streams, InitOpts, InitUnpredictables, SideData};

fn print_usage(arg0: &str) {
    eprintln!("Usage:");
    eprintln!("  {} run filename.elf   # run", arg0);
    eprintln!("  {} dis filename.elf   # disassemble", arg0);
}

fn read_file(path: &str) -> Vec<u8> {
    std::fs::read(path).expect("File should exist.")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Incorrect number of arguments.");
        print_usage(&args[0]);
        return;
    }
    match args[1].as_str() {
        "run" => {
            let contents = read_file(&args[2]);
            // TODO-cli: pass in argv (after `--`?).
            // TODO-cli: some way to configure environment variables?
            let side_data = SideData {
                argv: vec!["/tmp/asm".to_string()],
                envp: vec![],
            };
            let init_opts = InitOpts {
                side_data,
                // TODO-cli: allow seeding the random gen.
                init_unp: InitUnpredictables::Random(123),
            };
            interpret_to_streams(&contents, init_opts);
        }
        "dis" => {
            let contents = read_file(&args[2]);
            disassemble(&contents).unwrap_or_else(|rerr| eprintln!("{}", rerr));
        }
        _ => {
            eprintln!("Invalid command.");
            print_usage(&args[0]);
        }
    }
}
