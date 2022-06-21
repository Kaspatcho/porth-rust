use super::interpreter::{ Operation, Ops };

fn match_operations(token: &str) -> Operation {
    let (op, args) =  match token {
        "+" => { (Ops::ADD, vec![]) },
        "-" => { (Ops::SUB, vec![]) },
        "/" => { (Ops::DIV, vec![]) },
        "*" => { (Ops::MULT, vec![]) },
        "=" => { (Ops::EQUAL, vec![]) },
        "<" => { (Ops::LT, vec![]) },
        ">" => { (Ops::GT, vec![]) },
        "print" => { (Ops::PRINT, vec![]) },
        "quit" => { (Ops::QUIT, vec![]) },
        "clear" => { (Ops::CLEAR, vec![]) },
        "if" => { (Ops::IF, vec![]) },
        "end" => { (Ops::END, vec![]) },
        "while" => { (Ops::WHILE, vec![]) },
        "do" => { (Ops::DO, vec![]) },
        "dup" => { (Ops::DUP, vec![]) },
        v if matches!(v.parse::<i32>(), Ok(_)) => {
            (Ops::PUSH, vec![v.parse::<i32>().unwrap()])
        },
        v => { panic!("{:?} is not implemented yet", v) }
    };

    Operation{ op, args }
}

pub fn parse_program(program: String) -> Vec<Operation> {
    let mut vec: Vec<Operation> = Vec::new();
    let tokens = program.split_whitespace();
    let mut block_opening: Vec<usize> = vec![];
    let mut blocks: Vec<(usize, usize)> = vec![];

    for (index, mut token) in tokens.enumerate() {
        token = token.trim();
        if token.is_empty() { continue }
        let operation: Operation = match_operations(token);

        match &operation.op {
            Ops::IF | Ops::WHILE => { block_opening.push(index) },
            Ops::DO => {
                let whille_index = block_opening.last().unwrap();
                vec[*whille_index].args.push(index as i32);
            },
            Ops::END => {
                let opening = block_opening.pop().unwrap();
                blocks.push((opening, index));
            },
            _ => {}
        }

        vec.push(operation);
    }

    for (open, close) in blocks {
        let operation: &Operation = &vec[open];
        match operation.op {
            Ops::WHILE => {
                let doo_index = *operation.args.last().unwrap();
                let doo: &mut Operation = &mut vec[doo_index as usize];
                doo.args.push((close - doo_index as usize) as i32);
                vec[close].args.push((open as i32) - (close as i32));
                continue;
            },
            _ => {}
        };

        vec[open].args.push((close - open) as i32);
    }

    return vec;
}
