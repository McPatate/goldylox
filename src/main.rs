mod error;
mod scan;

use error::ErrorReporter;
use scan::Scanner;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn run(error_manager: &mut ErrorReporter, source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("token : {}", token.value);
    }
    error_manager.error(
        15,
        "YouSuck",
        "you are the worst human being on this planet",
    );
}

fn run_file(filename: &str) {
    let mut error_manager = ErrorReporter::new();
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    run(&mut error_manager, contents);
    if error_manager.has_error() {
        std::process::exit(65);
    }
}

fn repl_prompt(prompt: &str) {
    print!("{}", prompt);
    let _ = io::stdout().flush();
}

fn run_prompt() {
    let mut error_manager = ErrorReporter::new();
    let stdin = io::stdin();

    repl_prompt("> ");
    for line in stdin.lock().lines() {
        run(&mut error_manager, line.unwrap());
        repl_prompt("> ");
        error_manager.reset();
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
