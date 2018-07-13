mod token;
mod scanner;
mod error_token;

use errors::LoxError;
use self::token::Token;
use self::token::TokenType;
use self::scanner::Scanner;
use chunk::Chunk;

#[cfg(test)] mod tests;

struct Parser<'a> {
    current: Option<Token<'a>>,
    previous: Option<Token<'a>>,
}

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), LoxError> {
    let mut scanner: Scanner = Scanner::init(&source);
    let mut parser = Parser {
        current: None,
        previous: None
    };

    advance(&mut parser, &mut scanner);
    //expression();
    //consume(TOKEN_EOF, "Expected end of expression");

    Ok(())
}

fn advance<'p, 's: 'p>(parser: &'p mut Parser<'p>, scanner: &'s mut Scanner<'s>) {
    parser.previous = parser.current;

    loop {
        let tok = scanner.scan_token();
        parser.current = Some(tok);

        if let TokenType::Error(e) = tok.token_type {
            error_at_current(tok.start);
        }
    }
}

fn error_at_current(start: usize) {
    
}