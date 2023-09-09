mod error;
mod scanner;
mod token_type;
mod tokens;
mod perser;
mod expr;

use error::*;
use scanner::*;
use perser::*;

use std::env::args;
use std::fs::read_to_string;
use std::io::{self, stdout, Write};

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => run_promt(),
        2 => run_file(&args[1]).expect("Couldn't run file"),
        _ => {
            println!("Usage: lox-ast [script]");
            std::process::exit(64);
        }
    }

    Ok(())
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(mut m) => {
            m.report("".to_string());
            std::process::exit(65);
        }
    }

    Ok(())
}

fn run_promt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(mut e) => {
                    e.report("".to_string());
                    std::process::exit(65);
                }
            }
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
