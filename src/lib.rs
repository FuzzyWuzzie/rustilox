use std::error::Error;

mod chunk;
mod opcodes;
mod values;
mod vm;
mod util;

use chunk::Chunk;
use opcodes::*;
use values::Value;
use vm::VM;

pub fn run() -> Result<(), Box<Error>> {
    let mut chunk:Chunk = Chunk::init();

    let a = chunk.add_constant(Value::Real(1.2));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(a, 123);

    let b = chunk.add_constant(Value::Real(3.4));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(b, 123);

    chunk.write(OP_ADD, 123);

    let c = chunk.add_constant(Value::Real(5.6));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(c, 123);

    chunk.write(OP_DIVIDE, 123);

    chunk.write(OP_NEGATE, 123);

    let d = chunk.add_constant(Value::Real(0.0));
    chunk.write(OP_CONSTANT, 124);
    chunk.write(d, 124);

    chunk.write(OP_LESSER, 124);
    chunk.write(OP_NOT, 124);

    chunk.write(OP_RETURN, 124);

    println!("== test ==\n{}==========", chunk);
    {
        let mut vm: VM = VM::init(&mut chunk);
        vm.interpret()?;
    }

    Ok(())
}