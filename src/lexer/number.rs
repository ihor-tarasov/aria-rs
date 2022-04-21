use super::{reader::PeekableReader, Token, TokenInfo, TokenType};

pub fn lex(reader: &mut PeekableReader) -> Option<Token> {
    let mut acc = Vec::new();
    let start = reader.counter();
    loop {
        let c = match reader.peek() {
            Some(c) => c,
            None => break,
        };

        if c.is_ascii_digit() {
            acc.push(c);
            reader.next();
        } else {
            break;
        }
    }

    Some(Token::new(
        TokenType::Integer(
            std::str::from_utf8(acc.as_slice())
                .unwrap()
                .parse::<i64>()
                .unwrap(),
        ),
        TokenInfo::new(start, acc.len()),
    ))
}
