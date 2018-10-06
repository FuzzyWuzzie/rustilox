use super::token::Token;
use super::token::TokenType;
use super::scanner::Scanner;
use chunk::Chunk;
use values::Value;
use opcodes;

pub struct Parser<'a> {
    pub current: Option<Token<'a>>,
    pub previous: Option<Token<'a>>,
    pub chunk: &'a mut Chunk,
    pub scanner: Scanner<'a>,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl<'a> Parser<'a> {
    pub fn init(source: &'a str, chunk: &'a mut Chunk) -> Parser<'a> {
        Parser {
            current: None,
            previous: None,
            chunk,
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

    pub fn expression(&self) {

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

    pub fn emit_byte(&mut self, byte: u8) {
        self.chunk.write(byte, self.previous.unwrap().line);
    }

    pub fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    pub fn emit_return(&mut self) {
        self.emit_byte(opcodes::OP_RETURN);
    }

    pub fn emit_constant(&mut self, value: Value) {
        let constant = self.make_constant(value);
        self.emit_bytes(opcodes::OP_CONSTANT, constant);
    }

    fn make_constant(&mut self, value: Value) -> u8 {
        match self.chunk.add_constant(value) {
            Ok(c) => c,
            Err(e) => {
                self.error_at_current(&format!("{}", e));
                return 0;
            }
        }
    }

    fn grouping(&mut self) {
        self.expression();
        self.consume(TokenType::RightParen, "expect ')' after expression");
    }
}