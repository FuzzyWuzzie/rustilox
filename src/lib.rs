use std::error::Error;

pub mod chunk;
pub mod opcodes;
pub mod values;
pub mod vm;
pub mod util;
pub mod errors;

use chunk::Chunk;
use opcodes::*;
use values::{Value, ValueArray};
use vm::VM;

pub fn run() -> Result<(), Box<Error>> {
    // pre-build
    let chunk:Chunk = Chunk::build(
        ValueArray::build(vec![Value::Real(1.2), Value::Real(3.4), Value::Real(5.6), Value::Real(0.0)]),
        //vec![OP_CONSTANT, 0, OP_CONSTANT, 1, OP_ADD, OP_CONSTANT, 2, OP_DIVIDE, OP_NEGATE, OP_CONSTANT, 3, OP_LESSER, OP_NOT, OP_RETURN],
        // hard mode:
        vec![1, 0, 1, 1, 3, 1, 2, 6, 2, 1, 3, 12, 7, 0],
        vec![123,123,123,123,123,123,123,123,124,124,124,124,125,125]
    );

    // or build instuction at a time
    //let mut chunk:Chunk = Chunk::init();
    //let a = chunk.add_constant(Value::Real(1.2));
    //chunk.write(OP_CONSTANT, 123);
    //chunk.write(a, 123);
    //let b = chunk.add_constant(Value::Real(3.4));
    //chunk.write(OP_CONSTANT, 123);
    //chunk.write(b, 123);
    //chunk.write(OP_ADD, 123);
    //let c = chunk.add_constant(Value::Real(5.6));
    //chunk.write(OP_CONSTANT, 123);
    //chunk.write(c, 123);
    //chunk.write(OP_DIVIDE, 123);
    //chunk.write(OP_NEGATE, 123);
    //let d = chunk.add_constant(Value::Real(0.0));
    //chunk.write(OP_CONSTANT, 124);
    //chunk.write(d, 124);
    //chunk.write(OP_LESSER, 124);
    //chunk.write(OP_NOT, 124);
    //chunk.write(OP_RETURN, 124);

    // effectively:
    // let a = -((1.2 + 3.4) / 5.6)
    // let b = a < 0.0;
    // return !b;

    println!("== test ==\n{}==========", chunk);
    
    print!("!(-((1.2 + 3.4) / 5.6) < 0.0): ");
    let mut vm: VM = VM::init(&chunk);
    vm.interpret()?;

    Ok(())
}