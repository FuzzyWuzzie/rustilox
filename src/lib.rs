// TODO: remove this when finished!
#![allow(dead_code)]

mod chunk;
mod opcodes;
mod values;
mod vm;
mod util;
mod errors;
mod interpreter;
mod compiler;

pub use interpreter::interpret;
pub use values::Value;
pub use errors::LoxError;

#[cfg(test)] mod tests;