mod factor;
mod primary;
mod result;
mod term;

pub mod utils;

pub use result::ParserError;
pub use result::ParserErrorType;
pub use result::ParserResult;

use crate::lexer::TokenInfo;
use crate::lexer::Tokenizer;
use crate::vm::opcodes;
use crate::vm::ChunkBuilder;

pub fn parse(reader: &mut Tokenizer, builder: &mut ChunkBuilder) -> ParserResult {
    if reader.peek().is_some() {
        term::parse(reader, builder)?;
    }

    match reader.next() {
        Some(token) => utils::parser_unexpected(token.info()),
        None => {
            builder.instruction(opcodes::END, TokenInfo::new(0, 0));
            Ok(())
        }
    }
}
