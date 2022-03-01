mod error;
mod scan;

use error::ErrorReporter;
use scan::Scanner;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn run(error_reporter: &mut ErrorReporter, source: String) {
    let mut scanner = Scanner::new(source);
    let scan_result = scanner.scan_tokens();

    match scan_result {
        Ok(tokens) => {
            for token in &tokens {
                println!("token : {}", token.lexeme);
            }
        }
        Err(e) => error_reporter.error(
            e.line,
            &e.r#type.to_string(),
            "occurred while scanning source code",
        ),
    };
}

fn run_file(filename: &str) {
    let mut error_reporter = ErrorReporter::new();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    run(&mut error_reporter, contents);
    if error_reporter.has_error() {
        std::process::exit(65);
    }
}

fn repl_prompt(prompt: &str) {
    print!("{}", prompt);
    let _ = io::stdout().flush();
}

fn run_prompt() {
    let mut error_reporter = ErrorReporter::new();
    let stdin = io::stdin();

    repl_prompt("> ");
    for line in stdin.lock().lines() {
        run(&mut error_reporter, line.unwrap());
        repl_prompt("> ");
        error_reporter.reset();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: glox [script]");
        std::process::exit(64);
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}
