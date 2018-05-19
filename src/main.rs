extern crate rustylox;

use std::env;

use rustylox::errors;
use rustylox::repl;
use rustylox::values;

fn main() {
    let args: Vec<String> = env::args().collect();

    let result: Result<values::Value, errors::LoxError> = match args.len() {
        1 => repl::repl(),
        _ => Err(errors::LoxError::ReplError("Usage: rustylox [path]".to_string())),
    };

    println!("Result: {:?}", result);
}
