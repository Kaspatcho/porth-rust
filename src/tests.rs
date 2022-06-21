#[cfg(test)]
use crate::{Operation, parse_program, Interpreter};
#[cfg(test)]
use crate::interpreter::Ops;

#[test]
fn test_raw_program() {
    let program: Vec<Operation> = vec![
        Operation{ op: Ops::PUSH, args: vec![1] },
        Operation{ op: Ops::PUSH, args: vec![1] },
        Operation{ op: Ops::ADD, args: vec![]},
    ];

    let mut i = Interpreter::new(program.clone());

    i.compile();

    assert_eq!(i.stack, vec![2]);
}

#[test]
fn test_lexer() {
    let test = String::from("2 2 *");
    let program: Vec<Operation> = parse_program(test);

    let mut i = Interpreter::new(program.clone());

    i.compile();

    assert_eq!(i.stack, vec![4]);
}

#[test]
fn test_raw_if() {
    let program: Vec<Operation> = vec![
        Operation{ op: Ops::PUSH, args: vec![1] },
        Operation{ op: Ops::PUSH, args: vec![1] },
        Operation{ op: Ops::ADD, args: vec![]},
        Operation{ op: Ops::PUSH, args: vec![3]},
        Operation{ op: Ops::EQUAL, args: vec![]},
        Operation{ op: Ops::IF, args: vec![3]},
        Operation{ op: Ops::PUSH, args: vec![2]},
        Operation{ op: Ops::PRINT, args: vec![]},
        Operation{ op: Ops::END, args: vec![]},
        Operation{ op: Ops::PUSH, args: vec![1]},
    ];

    let mut i = Interpreter::new(program.clone());

    i.compile();
    assert_eq!(i.stack, vec![1]);
}

#[test]
fn test_lexer_if() {
    let test = String::from("1 1 +\n4 = if\n\t4\nend\n1\n");
    let program: Vec<Operation> = parse_program(test);

    let mut i = Interpreter::new(program.clone());

    i.compile();

    assert_eq!(i.stack, vec![1]);
}
