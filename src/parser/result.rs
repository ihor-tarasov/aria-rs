use crate::lexer::TokenInfo;

#[derive(Debug, Clone, Copy)]
pub enum ParserErrorType {
    UnexpectedToken,
    UnexpectedEndOfFile,
    UnknownCharacter,
}

pub struct ParserError {
    etype: ParserErrorType,
    info: TokenInfo,
}

impl ParserError {
    pub fn new(etype: ParserErrorType, info: TokenInfo) -> Self {
        Self { etype, info }
    }

    pub fn etype(&self) -> ParserErrorType {
        self.etype
    }

    pub fn info(&self) -> TokenInfo {
        self.info
    }
}

pub type ParserResult = Result<(), ParserError>;
