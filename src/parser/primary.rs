use crate::{
    lexer::{TokenType, Tokenizer},
    vm::{opcodes, ChunkBuilder},
};

use super::{utils, ParserResult};

pub fn parse(reader: &mut Tokenizer, builder: &mut ChunkBuilder) -> ParserResult {
    let token = utils::take_token(reader)?;
    let info = token.info();
    match token.take_type() {
        TokenType::Integer(value) => {
            let addr = builder.constant(value);
            builder.instruction(opcodes::CONST, info);
            builder.data(addr);
            Ok(())
        }
        TokenType::UnknownCharacter => utils::unknown_character(info),
        _ => utils::unexpected(info),
    }
}
