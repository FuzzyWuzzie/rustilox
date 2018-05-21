use values::Value;
use errors::LoxError;
use compiler::compile;

pub fn interpret(source: &String) -> Result<Value, LoxError> {
    compile(&source);

    Err(LoxError::NotImplemented)
}