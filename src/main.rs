use std::env::args;
use std::fs::read_to_string as readfile;

mod interpreter;
mod tests;
use interpreter::{Interpreter, Operation};
mod lexer;
use lexer::parse_program;

fn main() {
    let filename: String = match args().nth(1) {
        Some(s) => s,
        _ => {
            println!("ERRO! digite o nome de um arquivo");
            return;
        }
    };
    let file_text = readfile(filename).unwrap();
    let program: Vec<Operation> = parse_program(file_text);

    let mut i = Interpreter::new(program.clone());

    i.compile();
}
