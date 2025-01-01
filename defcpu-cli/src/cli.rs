use defcpu_core::{disassemble, interpret_to_streams};

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
            interpret_to_streams(&contents);
        }
        "dis" => {
            let contents = read_file(&args[2]);
            disassemble(&contents);
        }
        _ => {
            eprintln!("Invalid command.");
            print_usage(&args[0]);
        }
    }
}
