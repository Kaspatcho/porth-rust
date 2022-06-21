use std::fs::read_to_string as readfile;
use std::env::args;

mod tests;
mod interpreter;
use interpreter::{ Operation, Interpreter };
mod lexer;
use lexer::parse_program;

fn main() {
    let filename: String = match args().nth(1) {
        Some(s) => { s },
        _ => {
            println!("ERRO! digite o nome de um arquivo");
            return
        }
    };
    let file_text = readfile(filename).unwrap();
    let program: Vec<Operation> = parse_program(file_text);

    let mut i = Interpreter::new(program.clone());

    i.compile();
}
