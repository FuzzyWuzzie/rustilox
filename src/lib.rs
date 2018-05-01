use std::error::Error;

mod chunk;
mod opcodes;
mod values;
mod vm;

use chunk::Chunk;
use opcodes::*;
use values::Value;
use vm::VM;

pub fn run() -> Result<(), Box<Error>> {
    let mut chunk:Chunk = Chunk::init();

    let constant = chunk.add_constant(Value::Float(1.2));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(constant, 123);

    chunk.write(OP_RETURN, 123);

    {
        let mut vm: VM = VM::init(&mut chunk);
        vm.interpret()?;
        vm.free();
    }

    chunk.free();
    Ok(())
}