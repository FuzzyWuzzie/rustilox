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
        let mut contents: String = String::new();
        loop {
            let c: char = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };

            match c {
                '\n' => { self.line += 1; },
                '"' => { break; },
                _ => {
                    if let Some(ch) = self.advance() {
                        contents.push(ch);
                    }
                }
            };
        }

        if let None = self.chars.peek() {
            return self.error_token("unterminated string");
        }

        self.advance();
        self.make_token(TokenType::String(contents))
    }

    fn number(&mut self, start_char: char) -> Token {
        let mut contents: String = String::new();
        contents.push(start_char);

        loop {
            let c: char = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };

            if !is_digit(c) {
                break;
            }
            
            if let Some(ch) = self.advance() {
                contents.push(ch);
            }
        }

        let has_fractional = match self.chars.peek() {
            Some(c) => *c == '.',
            None => false
        };
        if has_fractional {
            if let Some(ch) = self.advance() {
                contents.push(ch);
            }

            loop {
                let c = match self.chars.peek() {
                    Some(c) => *c,
                    None => break
                };

                if !is_digit(c) {
                    break;
                }

                if let Some(ch) = self.advance() {
                    contents.push(ch);
                }
            }
        }

        self.make_token(TokenType::Number(contents))
    }

    fn identifer(&mut self, start_char: char) -> Token {
        let mut name: String = String::new();
        name.push(start_char);

        loop {
            let c = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };
            if is_digit(c) || is_alpha(c) {
                if let Some(ch) = self.advance() {
                    name.push(ch);
                }
            }
            else {
                break;
            }
        }

        // TODO: optimize using DFA
        self.make_token(match name.as_ref() {
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
            _ => TokenType::Identifier(name)
        })
    }

    fn comment(&mut self) -> Token {
        let mut note: String = String::new();

        loop {
            let c = match self.chars.peek() {
                Some(c) => *c,
                None => break
            };
            if c != '\n' {
                if let Some(ch) = self.advance() {
                    note.push(ch);
                }
            }
            else {
                break;
            }
        }

        self.make_token(TokenType::Comment(note))
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        let c = match self.advance() {
            Some(c) => c,
            None => return self.make_token(TokenType::Eof)
        };

        if is_digit(c) {
            return self.number(c);
        }
        if is_alpha(c) {
            return self.identifer(c);
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