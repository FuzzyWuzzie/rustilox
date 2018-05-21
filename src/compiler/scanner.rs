use std::str::Chars;
use std::iter::Peekable;

use super::token::Token;
use super::token::TokenType;

#[derive(Debug)]
pub struct Scanner<'a> {
    pub source: &'a String,
    pub chars: Peekable<Chars<'a>>,
    pub start: usize,
    pub current: usize,
    pub line: usize
}

impl<'a> Scanner<'a> {
    pub fn init(source: &'a String) -> Self {
        Scanner {
            source,
            chars: source.chars().peekable(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn skip_whitespace(&mut self) {
        loop {
            let next: char = match self.chars.peek() {
                Some(c) => *c,
                None => return
            };

            match next {
                ' ' | '\r' | 't' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' => {
                    match self.chars.nth(self.current + 1) {
                        Some(n) => {
                            if n == '/' {
                                while let Some(v) = self.advance() {
                                    if v == '\n' {
                                        return;
                                    }
                                }
                            }
                            else {
                                return;
                            }
                        },
                        None => return
                    };
                }
                _ => return
            };
        }
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line
        }
    }

    fn error_token(&self, msg: &str) -> Token {
        Token {
            token_type: TokenType::Error(String::from(msg)),
            start: self.start,
            length: msg.len(),
            line: self.line
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.chars.next()
    }

    fn match_next(&mut self, c: &char) -> bool {
        let matches: bool = match self.chars.peek() {
            Some(n) => n == c,
            None => false
        };

        if matches {
            self.advance();
        }
        matches
    }

    fn string(&mut self) -> Token {
        loop {
            let c: char = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };

            match c {
                '\n' => { self.line += 1; },
                '"' => { break; },
                _ => { self.advance(); }
            };
        }

        if let None = self.chars.peek() {
            return self.error_token("unterminated string");
        }

        self.advance();
        self.make_token(TokenType::String)
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> Token {
        loop {
            let c: char = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };

            if !Self::is_digit(c) {
                break;
            }
            self.advance();
        }

        // look for a fractional part
        let has_dot: bool = match self.chars.peek() {
            Some(c) => *c == '.',
            None => false
        };
        let has_fractional = match self.chars.nth(self.current + 1) {
            Some(c) => has_dot && Self::is_digit(c),
            None => false
        };

        if has_fractional {
            // consume the '.'
            self.advance();

            loop {
                let c = match self.chars.peek() {
                    Some(c) => *c,
                    None => break
                };
                if Self::is_digit(c) {
                    self.advance();
                }
                else {
                    break;
                }
            }
        }

        self.make_token(TokenType::Number)
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenType::Eof)
        };

        if Self::is_digit(c) {
            return self.number();
        }

        match c {
            '(' => self.make_token(TokenType::LeftParen),
            ')' => self.make_token(TokenType::RightParen),
            '{' => self.make_token(TokenType::LeftBrace),
            '}' => self.make_token(TokenType::RightBrace),
            ';' => self.make_token(TokenType::Semicolon),
            ',' => self.make_token(TokenType::Comma),
            '.' => self.make_token(TokenType::Dot),
            '-' => self.make_token(TokenType::Minus),
            '+' => self.make_token(TokenType::Plus),
            '/' => self.make_token(TokenType::Slash),
            '*' => self.make_token(TokenType::Star),

            '!' => if self.match_next(&'=') {
                       self.make_token(TokenType::BangEqual)
                   }
                   else {
                       self.make_token(TokenType::Bang)
                   },
            '=' => if self.match_next(&'=') {
                       self.make_token(TokenType::EqualEqual)
                   }
                   else {
                       self.make_token(TokenType::Equal)
                   },
            '<' => if self.match_next(&'=') {
                       self.make_token(TokenType::LessEqual)
                   }
                   else {
                       self.make_token(TokenType::Less)
                   },
            '>' => if self.match_next(&'=') {
                       self.make_token(TokenType::GreaterEqual)
                   }
                   else {
                       self.make_token(TokenType::Greater)
                   },

            '"' => self.string(),

            _ => self.error_token("unexpected character")
        }
    }
}