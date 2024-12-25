use defcpu_core::interpret;

fn main() {
    let r = run();
    if let Err(err) = r {
        eprintln!("{}", err);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("Incorrect number of arguments.\n\
        Usage: defcpu filename.elf"
            .to_owned());
    }
    let path = &args[1];
    let contents = std::fs::read(path).map_err(|e| format!("{}", e))?;
    let ret = interpret(&contents).map_err(|e| format!("{:#?}", e))?;
    println!("{}", ret);
    Ok(())
}
