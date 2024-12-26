use defcpu_core::{disassemble, interpret};

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  cargo run -- run filename.elf   # run");
    eprintln!("  cargo run -- dis filename.elf   # disassemble");
}

fn read_file(path: &str) -> Vec<u8> {
    std::fs::read(path).expect("File should exist.")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        eprintln!("Incorrect number of arguments.");
        print_usage();
        return;
    }
    match args[1].as_str() {
        "run" => {
            let contents = read_file(&args[2]);
            interpret(&contents);
        }
        "dis" => {
            let contents = read_file(&args[2]);
            disassemble(&contents);
        }
        _ => {
            eprintln!("Invalid command.");
            print_usage();
        }
    }
}
