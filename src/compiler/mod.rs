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
    scanner: Scanner<'a>,
    had_error: bool,
    panic_mode: bool,
}

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), LoxError> {
    //let mut scanner: Scanner = Scanner::init(&source);
    let mut parser = Parser::init(&source);

    
    parser.advance();
    //expression();
    parser.consume(TokenType::Eof, "expected end of expression");

    if parser.had_error {
        return  Err(LoxError::CompileError("failed to compile!".to_string(), parser.scanner.line));
    }
    Ok(())
}

impl<'a> Parser<'a> {
    pub fn init(source: &'a str) -> Parser<'a> {
        Parser {
            current: None,
            previous: None,
            scanner: Scanner::init(&source),
            had_error: false,
            panic_mode: false
        }
    }

    pub fn advance(&mut self) {
        self.previous = self.current;

        loop {
            self.current = Some(self.scanner.scan_token());
            if let TokenType::Error(e) = self.current.unwrap().token_type {
                self.error_at_current(&format!("error: {}", e));
            }
            else {
                break;
            }
        }
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &str) {
        if self.current.unwrap().token_type == token_type {
            self.advance();
            return;
        }

        self.error_at_current(msg);
    }

    fn error_at_current(&mut self, msg: &str) {
        let token = self.current.unwrap();
        self.error_at(&token, msg);
    }

    fn error_at(&mut self, token: &Token, msg: &str) {
        // suppress errors if we're already panicking
        if self.panic_mode {
            return;
        }
        self.panic_mode = true;

        eprint!("[line {}] Error", token.line);

        match token.token_type {
            TokenType::Eof => eprint!(" at end"),
            TokenType::Error(_) => (),
            _ => eprint!(" at pos '{}'", token.start),
        }

        eprintln!(": {}", msg);
        self.had_error = true;
    }
}