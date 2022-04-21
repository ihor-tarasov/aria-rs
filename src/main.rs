use std::io::Write;

use aria_rs::{lexer::Tokenizer, vm::ChunkBuilder, parser};

fn main() {
    loop {
        let mut code = String::new();

        print!("-> ");
        std::io::stdout().flush().unwrap();

        std::io::stdin().read_line(&mut code).unwrap();

        let mut tokenizer = Tokenizer::new(code.as_bytes());
        let mut chunk_builder = ChunkBuilder::new();

        match parser::parse(&mut tokenizer, &mut chunk_builder) {
            Ok(_) => (),
            Err(error) => {
                println!("Parser error: {:?}", error.etype());
                continue;
            }
        }

        let mut stack = Vec::new();

        chunk_builder.build().run(&mut stack, 0);

        if let Some(value) = stack.pop() {
            println!("{}", value);
        }
    }
}
