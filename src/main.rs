mod error;
mod scanner;
mod token_type;
mod tokens;
use error::*;
use scanner::*;

use std::env::args;
use std::fs::read_to_string;
use std::io::{self};

fn main() -> Result<(), String> {
    let args: Vec<String> = args().collect();
    println!("args: {:?}", args);
    if args.len() > 2 {
        println!("Usage: lox-ast [script]");
    } else if args.len() == 1 {
        _ = run_file(&args[1]);
    } else {
        run_promt();
    }

    Ok(())
}

fn run_file(path: &String) -> io::Result<()> {
    let buf = read_to_string(path)?;
    match run(buf) {
        Ok(_) => {}
        Err(mut m) => {
            m.report("".to_string());
            std::process::exit(64);
        }
    }

    Ok(())
}

fn run_promt() {
    let stdin = io::stdin();
    for line in stdin.lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            match run(line) {
                Ok(_) => {}
                Err(mut m) => {
                    m.report("".to_string());
                    std::process::exit(65);
                }
            }
        } else {
            break;
        }
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
