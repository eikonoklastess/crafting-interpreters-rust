use crate::error::ErrorReporter;
use crate::scanner::Scanner;
use std::env::args;
use std::error::Error;
use std::fs::read_to_string;
use std::io::{self, BufRead, Write};

mod error;
mod scanner;
mod token;

fn main() -> Result<(), Box<dyn Error>> {
    let mut reporter = ErrorReporter::new();
    let args: Vec<String> = args().skip(1).collect();
    println!("args: {:?}", args);
    if args.len() > 1 {
        println!("Usage: rlox [script]");
        if reporter.had_error {
            std::process::exit(64);
        }
    } else if args.len() == 1 {
        match run_file(&args[0], &mut reporter) {
            Ok(_) => println!("File processed successfully"),
            Err(e) => eprintln!("Error processing file: {}", e),
        }
    } else {
        run_prompt(&mut reporter)?;
    }

    Ok(())
}

fn run_file(path: &str, reporter: &mut ErrorReporter) -> Result<(), Box<dyn Error>> {
    let source = read_to_string(path)?;
    run(&source, reporter)?;
    Ok(())
}

fn run_prompt(reporter: &mut ErrorReporter) -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let line = line.trim_end();
                if !line.is_empty() {
                    run(&line, reporter)?;
                    reporter.reset()
                }
            }
            Err(e) => return Err(Box::new(e)),
        }
    }
    Ok(())
}

fn run(source: &str, reporter: &mut ErrorReporter) -> Result<(), Box<dyn Error>> {
    let scanner = &mut Scanner::new(source);
    let tokens = scanner.scan_tokens(reporter)?;
    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
