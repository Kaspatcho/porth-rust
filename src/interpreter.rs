use std::process::exit;

#[derive(Clone, Debug)]
pub enum Ops {
    ADD,
    SUB,
    MULT,
    DIV,
    EQUAL,
    LT,
    GT,
    PRINT,
    CLEAR,
    QUIT,
    IF,
    END,
    PUSH,
}

#[derive(Clone)]
pub struct Operation {
    pub op: Ops,
    pub args: Vec<i32>
}

pub struct Interpreter {
    pub stack: Vec<i32>,
    sequence: Vec<Operation>,
    current_index: i32,
}

impl Interpreter {
    pub fn new(sequence: Vec<Operation>) -> Self {
        Self{ stack: vec![], sequence, current_index: 0 }
    }

    pub fn compile(&mut self) {
        while self.current_index < (self.sequence.len() as i32) {
            let element: Operation = self.sequence[self.current_index as usize].clone();
            self.match_actions(element);
            self.current_index += 1;
        }
    }

    fn match_actions(&mut self, element: Operation) {
        let action: &Ops = &element.op;
        match action {
            Ops::ADD =>  { self.ab_op(|a, b| a + b) },
            Ops::SUB =>  { self.ab_op(|a, b| a - b) },
            Ops::MULT =>  { self.ab_op(|a, b| a * b) },
            Ops::DIV =>  { self.ab_op(|a, b| a / b) },
            Ops::EQUAL =>  { self.ab_op(|a, b| (a == b) as i32) },
            Ops::LT =>  { self.ab_op(|a, b| (a < b) as i32) },
            Ops::GT =>  { self.ab_op(|a, b| (a > b) as i32) },
            Ops::IF => { self.jump(&element) }
            Ops::END => {  }
            Ops::PUSH => { self.push(&element) },
            Ops::PRINT => { self.print() },
            Ops::CLEAR => { println!("\x1B[2J\x1B[1;1H") },
            Ops::QUIT => { exit(0) },
            // v => { panic!("{:?} is not implemented yet", v) }
        }
    }

    fn ab_op(&mut self, function: fn(i32, i32) -> i32) {
        assert!(self.stack.len() > 1);
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        self.stack.push(function(a, b));
    }

    fn push(&mut self, element: &Operation) {
        self.stack.push(element.args[0]);
    }

    fn print(&mut self) {
        assert!(self.stack.len() > 0);

        let a = self.stack.pop().unwrap();
        println!("{}", a);
    }

    fn jump(&mut self, element: &Operation) {
        assert!(self.stack.len() > 0);
        let condition: bool = self.stack.pop().unwrap() != 0;
        if !condition {
            self.current_index += element.args[0];
        }
    }
}