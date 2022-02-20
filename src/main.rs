mod scan;

use scan::Scanner;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};

fn run(source: String) {
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("token : {}", token.value);
    }
}

fn run_file(filename: &str) {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    run(contents);
}

fn repl_prompt(prompt: &str) {
    print!("{}", prompt);
    let _ = io::stdout().flush();
}

fn run_prompt() {
    repl_prompt("> ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        run(line.unwrap());
        repl_prompt("> ");
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
