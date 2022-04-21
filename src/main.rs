// 2 + 2 * 2

mod opcodes {
    pub const END: u8 = 0x00;
    pub const CONST: u8 = 0x01;
    pub const ADD: u8 = 0x02;
    pub const MUL: u8 = 0x03;
}

fn main() {
    let constants = [
        2
    ];

    let program = [
        opcodes::CONST, 0,
        opcodes::CONST, 0,
        opcodes::MUL,
        opcodes::CONST, 0,
        opcodes::ADD,
        opcodes::END,
    ];

    let mut stack: Vec<i32> = Vec::new();

    let mut pc = program.iter().cloned();
    while let Some(opcode) = pc.next() {
        match opcode {
            opcodes::END => break,
            opcodes::CONST => stack.push(*constants.get(pc.next().unwrap() as usize).unwrap()),
            opcodes::ADD => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs.wrapping_add(rhs));
            },
            opcodes::MUL => {
                let rhs = stack.pop().unwrap();
                let lhs = stack.pop().unwrap();
                stack.push(lhs.wrapping_mul(rhs));
            },
            _ => panic!(),
        }
    }

    if let Some(value) = stack.pop() {
        println!("{}", value);
    }
}
