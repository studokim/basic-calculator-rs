use evaluator::evaluate;

use crate::parser::parse;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process;
use std::result::Result;

mod evaluator;
mod parser;
mod types;

fn interpret_line(line: String) {
    if line.is_empty() {
        return;
    }
    let (input, parsed_line) = parse(&line).unwrap();
    if !input.is_empty() {
        eprintln!("Parsing error, input remaining {:?}", input);
        return;
    }
    let result = evaluate(parsed_line);
    println!("{:?}", result);
}

fn run_file(filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut input_file_contents = String::new();
    file.read_to_string(&mut input_file_contents)?;
    for line in input_file_contents.lines().by_ref() {
        interpret_line(line.to_string());
    }
    Ok(())
}

fn run_interactive() -> Result<(), Box<dyn Error>> {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input.");
        input = input.trim().to_string();
        if input.is_empty() {
            break;
        }
        interpret_line(input.clone());
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    match env::args().len() {
        1 => run_interactive(),
        2 => {
            let filepath = env::args()
                .next_back()
                .expect("There should be `path-to-program.bc`.");
            run_file(&filepath)
        }
        _ => {
            eprintln!("Provide `path-to-program.bc` or run without arguments to enter REPL.");
            process::exit(1)
        }
    }
}
