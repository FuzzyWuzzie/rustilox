use std::error::Error;

mod chunk;
mod opcodes;
mod values;

use chunk::Chunk;
use opcodes::*;
use values::Value;

pub fn run() -> Result<(), Box<Error>> {
    let mut chunk:Chunk = Chunk::init();

    let constant = chunk.add_constant(Value::new(1.2));
    chunk.write(OP_CONSTANT, 123);
    chunk.write(constant, 123);

    chunk.write(OP_RETURN, 123);
    chunk.disassemble_chunk("test chunk");

    chunk.free();

    Ok(())
}