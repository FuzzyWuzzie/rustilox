mod token;
mod scanner;

use errors::LoxError;
use self::token::TokenType;
use self::scanner::Scanner;

pub fn compile(source: &String) -> Result<(), LoxError> {
    let mut scanner: Scanner = Scanner::init(&source);

    let mut line: usize = 0;
    loop {
        let token = scanner.scan_token();
        if token.line != line {
            print!("{:04} ", token.line);
            line = token.line;
        }
        else {
            print!("   | ");
        }
        println!("{}", token.token_type);

        match token.token_type {
            TokenType::Eof => break,
            TokenType::Error(msg) => return Err(LoxError::InterpetError(msg, scanner.line)),
            _ => ()
        }
    }

    Ok(())
}