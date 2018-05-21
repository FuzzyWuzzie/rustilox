mod token;
mod scanner;

use self::token::TokenType;
use self::scanner::Scanner;

pub fn compile(source: &String) {
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

        if let TokenType::Eof = token.token_type {
            break;
        }
    }
}