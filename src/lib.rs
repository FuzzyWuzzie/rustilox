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

    let a = chunk.add_constant(Value::Float(1.2));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(a, 123);

    let b = chunk.add_constant(Value::Float(3.4));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(b, 123);

    chunk.write(OP_ADD, 123);

    let c = chunk.add_constant(Value::Float(5.6));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(c, 123);

    chunk.write(OP_DIVIDE, 123);

    chunk.write(OP_NEGATE, 123);

    chunk.write(OP_RETURN, 123);

    chunk._disassemble_chunk("test");

    {
        let mut vm: VM = VM::init(&mut chunk);
        vm.interpret()?;
    }

    Ok(())
}