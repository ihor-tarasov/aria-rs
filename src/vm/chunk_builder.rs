use std::collections::HashMap;

use crate::lexer::TokenInfo;

use super::Chunk;

pub struct ChunkBuilder {
    instructions: Vec<u8>,
    info: Vec<TokenInfo>,
    constants: Vec<i64>,
    constants_map: HashMap<i64, u32>,
}

impl ChunkBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            info: Vec::new(),
            constants: Vec::new(),
            constants_map: HashMap::new(),
        }
    }

    pub fn instruction(&mut self, opcode: u8, info: TokenInfo) {
        self.instructions.push(opcode);
        self.info.push(info);
    }

    pub fn data(&mut self, data: u32) {
        self.instructions.push((data >> 24) as u8);
        self.instructions.push((data >> 16) as u8);
        self.instructions.push((data >> 8) as u8);
        self.instructions.push(data as u8);
        for _ in 0..4 {
            self.info.push(TokenInfo::new(0, 0));
        }
    }

    pub fn constant(&mut self, value: i64) -> u32 {
        match self.constants_map.get(&value) {
            Some(result) => *result,
            None => {
                self.constants.push(value);
                self.constants_map
                    .insert(value, self.constants.len() as u32 - 1);
                self.constants.len() as u32 - 1
            }
        }
    }

    pub fn build(self) -> Chunk {
        Chunk::new(self.constants, self.instructions, self.info)
    }
}
