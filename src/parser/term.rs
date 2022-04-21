use crate::{
    lexer::{TokenType, Tokenizer},
    vm::{opcodes, ChunkBuilder},
};

use super::{factor, ParserResult};

pub fn parse(reader: &mut Tokenizer, builder: &mut ChunkBuilder) -> ParserResult {
    factor::parse(reader, builder)?;

    while let Some(token) = reader.peek() {
        let info = token.info();
        match token.ttype() {
            TokenType::Plus => {
                reader.next().unwrap();
                factor::parse(reader, builder)?;
                builder.instruction(opcodes::ADD, info);
            }
            _ => break,
        }
    }
    Ok(())
}
