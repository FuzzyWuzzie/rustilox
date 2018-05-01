use std::error;
use std::fmt;

#[derive(Debug)]
pub enum InterpretError {
    CompileError(String, usize),
    RuntimeError(String, usize)
}

impl error::Error for InterpretError {
    fn description(&self) -> &str {
        match self {
            InterpretError::CompileError(_, _) => "Compile error",
            InterpretError::RuntimeError(_, _) => "Runtime error"
        }
    }
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpretError::CompileError(d, l) => write!(f, "Compile error: {} on line {}", d, l),
            InterpretError::RuntimeError(d, l) => write!(f, "Runtime error: {} on line {}", d, l),
        }
    }
}