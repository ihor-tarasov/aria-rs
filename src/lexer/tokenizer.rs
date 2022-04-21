use super::{lex, reader::PeekableReader, Token};

pub struct Tokenizer<'a> {
    reader: PeekableReader<'a>,
    token: Option<Token>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        let mut reader = PeekableReader::new(source);
        Self {
            token: lex::lex(&mut reader),
            reader,
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    pub fn next(&mut self) -> Option<Token> {
        std::mem::replace(&mut self.token, lex::lex(&mut self.reader))
    }
}
