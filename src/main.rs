extern crate rustilox;
extern crate rprompt;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use rustilox::Value;

fn repl() -> Result<Value, Box<Error>> {
    loop {
        let line = rprompt::prompt_reply_stdout("> ")?;
        if line.is_empty() {
            return Ok(Value::Nil);
        }

        rustilox::interpret(&line)?;
    }
}

fn run_file(filename: &str) -> Result<Value, Box<Error>> {
    let mut f = File::open(filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    match rustilox::interpret(&contents) {
        Ok(v) => Ok(v),
        Err(e) => Err(Box::new(e))
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let result: Result<Value, Box<Error>> = match args.len() {
        1 => repl(),
        2 => run_file(&args[1]),
        _ => {
            println!("Usage: rustilox [path]");
            return;
        },
    };

    println!("Result: {:?}", result);
}
