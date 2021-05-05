use io::Write;
use std::env;
use std::fs::File;
use std::io::{self, BufReader, Read};

use lox_rs::scanner;

fn run_file(path: &str) -> std::io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    if let Err(e) = scanner::run(&contents) {
        return Err(e);
    }

    Ok(())
}

fn run_prompt() -> std::io::Result<()> {
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        let stdin = io::stdin();
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                scanner::run(&input)?;
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: lox-rs [script]");
        std::process::exit(64)
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_prompt()
    }
}
