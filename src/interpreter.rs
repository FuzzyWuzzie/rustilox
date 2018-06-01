use values::Value;
use errors::LoxError;
use compiler::compile;

pub fn interpret(source: &str) -> Result<Value, LoxError> {
    compile(&source)?;

    Ok(Value::Nil)
}