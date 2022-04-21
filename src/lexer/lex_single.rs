use super::{reader::PeekableReader, TokenType};

pub fn lex(reader: &mut PeekableReader) -> Option<TokenType> {
    match reader.next()? {
        b'+' => Some(TokenType::Plus),
        b'*' => Some(TokenType::Asterisk),
        _ => Some(TokenType::UnknownCharacter),
    }
}
