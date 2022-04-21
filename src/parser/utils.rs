use crate::lexer::{Token, TokenInfo, Tokenizer};

use super::{ParserError, ParserErrorType, ParserResult};

pub fn take_token(reader: &mut Tokenizer) -> Result<Token, ParserError> {
    match reader.next() {
        Some(token) => Ok(token),
        None => Err(ParserError::new(
            ParserErrorType::UnexpectedEndOfFile,
            TokenInfo::new(0, 0),
        )),
    }
}

pub fn unexpected(info: TokenInfo) -> ParserResult {
    Err(ParserError::new(ParserErrorType::UnexpectedToken, info))
}

pub fn unknown_character(info: TokenInfo) -> ParserResult {
    Err(ParserError::new(ParserErrorType::UnknownCharacter, info))
}
