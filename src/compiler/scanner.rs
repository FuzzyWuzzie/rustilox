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

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
       (c >= 'a' && c <= 'z')
    || (c >= 'A' && c <= 'Z')
    || c == '_'
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
                ' ' | '\r' | '\t' => {
                    self.advance();
                },
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                _ => return
            };
        }
    }

    fn make_token(&self, token_type: TokenType<'a>) -> Token<'a> {
        Token {
            token_type,
            start: self.start,
            length: self.current - self.start,
            line: self.line
        }
    }

    fn error_token(&self, msg: &str) -> Token {
        Token {
            token_type: TokenType::Error(msg.to_string()),
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

        // actually get the string contents!
        // TODO: remove to optimize!
        let start = match self.source.char_indices().nth(self.start + 1) {
            Some(s) => s.0,
            None => return self.error_token("string underflow")
        };
        let end = match self.source.char_indices().nth(self.current - 1) {
            Some(s) => s.0,
            None => return self.error_token(&format!("string overflow, start: {}, current: {}, len: {}", self.start, self.current, self.source.len()))
        };
        let slice: &str = &self.source[start..end+1];

        self.advance();
        self.make_token(TokenType::String(slice))
    }

    fn number(&mut self) -> Token {
        loop {
            let c: char = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };

            if !is_digit(c) {
                break;
            }
            self.advance();
        }

        let has_fractional = match self.chars.peek() {
            Some(c) => *c == '.',
            None => false
        };
        if has_fractional {
            self.advance();
            loop {
                let c = match self.chars.peek() {
                    Some(c) => *c,
                    None => break
                };

                if !is_digit(c) {
                    break;
                }
                self.advance();
            }
        }

        // extract the bits
        // TODO: remove to optimize!
        let start = match self.source.char_indices().nth(self.start) {
            Some(s) => s.0,
            None => return self.error_token("number underflow")
        };
        let end = match self.source.char_indices().nth(self.current - 1) {
            Some(s) => s.0,
            None => return self.error_token(&format!("number overflow, start: {}, current: {}, len: {}", self.start, self.current, self.source.len()))
        };
        let slice: &str = &self.source[start..end+1];

        self.make_token(TokenType::Number(slice))
    }

    fn check_keyword(&self, start: usize, length: usize, rest: &str, token_type: TokenType<'a>) -> Option<TokenType<'a>> {
        if self.current - self.start == start + length {
            let slice_start = self.source.char_indices().nth(self.start + start).unwrap().0;
            let slice_end = self.source.char_indices().nth(self.current - 1).unwrap().0;
            let slice: &str = &self.source[slice_start..slice_end+1];

            if slice == rest {
                return Some(token_type)
            }
        }
        
        None
    }

    fn identifer(&mut self, start_char: &char) -> Token {
        loop {
            let c = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };
            if is_digit(c) || is_alpha(c) {
                self.advance();
            }
            else {
                break;
            }
        }

        let keyword: Option<TokenType> = match start_char {
            'a' => self.check_keyword(1, 2, "nd", TokenType::And),
            'c' => self.check_keyword(1, 4, "lass", TokenType::Class),
            'e' => self.check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if self.current - self.start > 1 {
                    let second_char = self.source.chars().nth(self.start + 1).unwrap();
                    match second_char {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::False),
                        'o' => self.check_keyword(2, 1, "r", TokenType::For),
                        'u' => self.check_keyword(2, 1, "n", TokenType::Fun),
                        _ => None
                    }
                }
                else {
                    None
                }
            },
            'i' => self.check_keyword(1, 1, "f", TokenType::If),
            'n' => self.check_keyword(1, 2, "il", TokenType::Nil),
            'o' => self.check_keyword(1, 1, "r", TokenType::Or),
            'p' => self.check_keyword(1, 4, "rint", TokenType::Print),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::Return),
            's' => self.check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if self.current - self.start > 1 {
                    let second_char = self.source.chars().nth(self.start + 1).unwrap();
                    match second_char {
                        'h' => self.check_keyword(2, 2, "is", TokenType::This),
                        'r' => self.check_keyword(2, 2, "ue", TokenType::True),
                        _ => None
                    }
                }
                else {
                    None
                }
            },
            'v' => self.check_keyword(1, 2, "ar", TokenType::Var),
            'w' => self.check_keyword(1, 4, "hile", TokenType::While),
            _ => None
        };

        match keyword {
            Some(key) => self.make_token(key),
            None => {
                let slice_start = self.source.char_indices().nth(self.start).unwrap().0;
                let slice_end = self.source.char_indices().nth(self.current - 1).unwrap().0;
                let slice: &str = &self.source[slice_start..slice_end+1];
                self.make_token(TokenType::Identifier(slice))
            }
        }

        /*// extract the bits
        // TODO: remove to optimize!
        let start = match self.source.char_indices().nth(self.start) {
            Some(s) => s.0,
            None => return self.error_token(&format!("identifier underflow, start: {}, current: {}, len: {}", self.start, self.current, self.source.len()))
        };
        let end = match self.source.char_indices().nth(self.current - 1) {
            Some(s) => s.0,
            None => return self.error_token(&format!("identifier overflow, start: {}, current: {}, len: {}", self.start, self.current, self.source.len()))
        };
        let slice: &str = &self.source[start..end+1];

        // TODO: optimize using DFA
        self.make_token(match slice {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier(slice)
        })*/
    }

    fn comment(&mut self) -> Token {
        loop {
            let c = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };
            if c != '\n' {
                self.advance();
            }
            else {
                break;
            }
        }

        let start = match self.source.char_indices().nth(self.start + 2) {
            Some(s) => s.0,
            None => return self.error_token(&format!("comment underflow, start: {}, current: {}, len: {}", self.start, self.current, self.source.len()))
        };
        let end = match self.source.char_indices().nth(self.current - 1) {
            Some(s) => s.0,
            None => return self.error_token(&format!("comment overflow, start: {}, current: {}, len: {}", self.start, self.current, self.source.len()))
        };
        let slice: &str = &self.source[start..end+1];

        self.make_token(TokenType::Comment(slice))
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenType::Eof)
        };

        if is_digit(c) {
            return self.number();
        }
        if is_alpha(c) {
            return self.identifer(&c);
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

            '/' => if self.match_next(&'/') {
                       self.comment()
                   }
                   else {
                       self.make_token(TokenType::Slash)
                   },

            '"' => self.string(),

            _ => self.error_token("unexpected character")
        }
    }
}