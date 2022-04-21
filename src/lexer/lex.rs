use super::{lex_number, lex_single, reader::PeekableReader, Token, TokenInfo};

pub fn lex(reader: &mut PeekableReader) -> Option<Token> {
    let mut c = reader.peek()?;

    while c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
        reader.next();
        c = reader.peek()?;
    }

    if c.is_ascii_digit() {
        return lex_number::lex(reader);
    }

    Some(Token::new(
        lex_single::lex(reader)?,
        TokenInfo::new(reader.counter() - 1, 1),
    ))
}
