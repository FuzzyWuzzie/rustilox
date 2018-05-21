use super::opcodes::*;
use super::chunk::Chunk;
use super::vm::VM;  
use super::values::Value;
use super::values::ValueArray;

#[test]
fn test_instruction_at_a_time() {
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

    let mut vm: VM = VM::init(&chunk);
    let result = vm.evaluate().expect("evaluate");
    assert_eq!(result, Value::Boolean(false));
}

#[test]
fn test_instruction_at_a_block() {
    let chunk:Chunk = Chunk::build(
        ValueArray::build(vec![Value::Real(1.2), Value::Real(3.4), Value::Real(5.6), Value::Real(0.0)]),
        vec![OP_CONSTANT, 0, OP_CONSTANT, 1, OP_ADD, OP_CONSTANT, 2, OP_DIVIDE, OP_NEGATE, OP_CONSTANT, 3, OP_LESSER, OP_NOT, OP_RETURN],
        vec![123,123,123,123,123,123,123,123,124,124,124,124,125,125]
    );

    let mut vm: VM = VM::init(&chunk);
    let result = vm.evaluate().expect("evaluate");
    assert_eq!(result, Value::Boolean(false));
}