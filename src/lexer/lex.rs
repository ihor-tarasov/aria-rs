use super::{number, single, reader::PeekableReader, Token, TokenInfo};

pub fn lex(reader: &mut PeekableReader) -> Option<Token> {
    let mut c = reader.peek()?;

    while c == b' ' || c == b'\t' || c == b'\r' || c == b'\n' {
        reader.next();
        c = reader.peek()?;
    }

    if c.is_ascii_digit() {
        return number::lex(reader);
    }

    Some(Token::new(
        single::lex(reader)?,
        TokenInfo::new(reader.counter() - 1, 1),
    ))
}
