use crate::{
    lexer::{TokenType, Tokenizer},
    vm::{opcodes, ChunkBuilder},
};

use super::{primary, ParserResult};

pub fn parse(reader: &mut Tokenizer, builder: &mut ChunkBuilder) -> ParserResult {
    primary::parse(reader, builder)?;

    while let Some(token) = reader.peek() {
        let info = token.info();
        match token.ttype() {
            TokenType::Asterisk => {
                reader.next().unwrap();
                primary::parse(reader, builder)?;
                builder.instruction(opcodes::MUL, info);
            }
            _ => break,
        }
    }
    Ok(())
}
