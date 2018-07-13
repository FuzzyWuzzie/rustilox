use super::token::Token;
use super::token::TokenType;
use super::scanner::Scanner;
use chunk::Chunk;
use opcodes;

pub struct Parser<'a> {
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub scanner: Scanner<'a>,
    pub had_error: bool,
    pub panic_mode: bool,
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

    pub fn emit_return(&self, mut chunk: &mut Chunk) {
        self.emit_byte(&mut chunk, opcodes::OP_RETURN);
    }

    pub fn emit_byte(&self, chunk: &mut Chunk, byte: u8) {
        chunk.write(byte, self.previous.unwrap().line);
    }

    pub fn emit_bytes(&self, mut chunk: &mut Chunk, byte1: u8, byte2: u8) {
        self.emit_byte(&mut chunk, byte1);
        self.emit_byte(&mut chunk, byte2);
    }
}