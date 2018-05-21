use std::error;
use std::fmt;

#[derive(Debug)]
pub enum LoxError {
    InterpetError(String, usize),
    CompileError(String, usize),
    RuntimeError(String, usize),
    ReplError(String),
    NotImplemented,
}

impl error::Error for LoxError {
    fn description(&self) -> &str {
        match self {
            LoxError::InterpetError(_, _) => "Interpret error",
            LoxError::CompileError(_, _) => "Compile error",
            LoxError::RuntimeError(_, _) => "Runtime error",
            LoxError::ReplError(_) => "Repl error",
            LoxError::NotImplemented => "Not implemented error"
        }
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::InterpetError(d, l) => write!(f, "Interpret error: {} on line {}", d, l),
            LoxError::CompileError(d, l) => write!(f, "Compile error: {} on line {}", d, l),
            LoxError::RuntimeError(d, l) => write!(f, "Runtime error: {} on line {}", d, l),
            LoxError::ReplError(d) => write!(f, "Repl error: {}", d),
            LoxError::NotImplemented => write!(f, "Not implemented!")
        }
    }
}