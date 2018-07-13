mod token;
mod scanner;
mod error_token;
mod parser;

use errors::LoxError;
use self::token::TokenType;
use self::parser::Parser;
use chunk::Chunk;

#[cfg(test)] mod tests;

pub fn compile(source: &str, mut chunk: &mut Chunk) -> Result<(), LoxError> {
    let mut parser = Parser::init(&source);
    
    parser.advance();
    //expression();
    parser.consume(TokenType::Eof, "expected end of expression");

    end_compiler(&mut parser, &mut chunk);

    if parser.had_error {
        return  Err(LoxError::CompileError("failed to compile!".to_string(), parser.scanner.line));
    }
    Ok(())
}

fn end_compiler(parser: &mut Parser, mut chunk: &mut Chunk) {
    parser.emit_return(&mut chunk);
}
