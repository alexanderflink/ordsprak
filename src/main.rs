use ordsprak::run;
use std::{env, error, fs, process};

#[derive(Debug)]
enum Mode {
    Interactive,
    File(String),
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {}", e);
        process::exit(1)
    }
}

fn try_main() -> Result<(), Box<dyn error::Error>> {
    let args = env::args();

    let mode = parse_arguments(args)?;

    match mode {
        Mode::File(file_path) => {
            let contents: String = fs::read_to_string(&file_path)?;

            run(&contents)
        }
        Mode::Interactive => {
            // start interactive mode
            Ok(())
        }
    }?;

    Ok(())
}

fn parse_arguments(args: env::Args) -> Result<Mode, &'static str> {
    match &args.len() {
        1 => Ok(Mode::Interactive),
        2 => {
            let args: Vec<String> = args.collect();
            let file_path: &String = &args[1];

            Ok(Mode::File(file_path.clone()))
        }
        _ => Err("Expected 0 or 1 arguments."),
    }
}
