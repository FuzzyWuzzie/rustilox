mod token;
mod scanner;

use errors::LoxError;
use self::token::TokenType;
use self::scanner::Scanner;
use chunk::Chunk;

#[cfg(test)] mod tests;

pub fn compile(source: &str, chunk: &mut Chunk) -> Result<(), LoxError> {
    let mut scanner: Scanner = Scanner::init(&source);

    

    Ok(())
}