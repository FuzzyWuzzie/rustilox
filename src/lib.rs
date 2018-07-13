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
pub use vm::VM;
pub use chunk::Chunk;

#[cfg(test)] mod tests;