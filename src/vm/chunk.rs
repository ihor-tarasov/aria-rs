use std::{
    iter::{Cloned, Skip},
    slice::Iter,
};

use crate::lexer::TokenInfo;

use super::opcodes;

pub struct Chunk {
    constants: Vec<i64>,
    instructions: Vec<u8>,
    #[allow(dead_code)]
    info: Vec<TokenInfo>,
}

type ProgramCounter<'a> = Cloned<Skip<Iter<'a, u8>>>;

fn read_data(pc: &mut ProgramCounter) -> usize {
    ((pc.next().unwrap() as usize) << 24)
        | ((pc.next().unwrap() as usize) << 16)
        | ((pc.next().unwrap() as usize) << 8)
        | (pc.next().unwrap() as usize)
}

impl Chunk {
    pub fn new(constants: Vec<i64>, instructions: Vec<u8>, info: Vec<TokenInfo>) -> Self {
        Self {
            constants,
            instructions,
            info,
        }
    }

    pub fn run(&self, stack: &mut Vec<i64>, offset: usize) {
        let mut pc = self.instructions.iter().skip(offset).cloned();
        while let Some(opcode) = pc.next() {
            match opcode {
                opcodes::END => break,
                opcodes::CONST => stack.push(*self.constants.get(read_data(&mut pc)).unwrap()),
                opcodes::ADD => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs.wrapping_add(rhs));
                }
                opcodes::MUL => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(lhs.wrapping_mul(rhs));
                }
                _ => panic!("Unknown instruction."),
            }
        }
    }
}
