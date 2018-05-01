use std::error;
use std::fmt;

use ::chunk::Chunk;
use ::opcodes::*;
use ::values::Value;

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

pub struct VM<'a> {
    pub chunk: &'a mut Chunk,
    ip: usize,
    stack: Vec<Value>
}

impl<'a> VM<'a> {
    pub fn init(chunk: &mut Chunk) -> VM {
        VM {
            chunk: chunk,
            ip: 0,
            stack: Vec::new()
        }
    }

    // TODO: move into a closure inside of interpret somehow?
    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }

    // TODO: move into a closure inside of interpret somehow?
    fn read_constant(&mut self) -> &Value {
        let loc = self.read_byte();
        &self.chunk.constants.values[loc as usize]
    }

    fn binary_op(&mut self) -> Result<(Value, Value), InterpretError> {
        let b = match self.stack.pop() {
            Some(v) => v,
            None => return Err(InterpretError::CompileError(format!("no value to pop"), self.chunk.lines[self.ip - 1]))
        };
        let a = match self.stack.pop() {
            Some(v) => v,
            None => return Err(InterpretError::CompileError(format!("no value to pop"), self.chunk.lines[self.ip - 1]))
        };

        Ok((a, b))
    }

    pub fn interpret(&mut self) -> Result<(), InterpretError> {
        loop {
            if cfg!(feature = "trace_execution") {
                print!("          ");
                for value in &self.stack {
                    print!("[ {} ]", value);
                }
                println!();
                self.chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte();
            match instruction {
                OP_RETURN => {
                    let top = match self.stack.pop() {
                        Some(v) => v,
                        None => Value::Null
                    };
                    println!("{}", top);

                    return Ok(());
                },
                OP_CONSTANT => {
                    let new_constant:Value;
                    {
                        let constant = self.read_constant();
                        new_constant = constant.clone();
                    }
                    self.stack.push(new_constant);
                },
                OP_NEGATE => {
                    let top = match self.stack.pop() {
                        Some(v) => v,
                        None => return Err(InterpretError::CompileError(format!("can't negate, no value to pop"), self.chunk.lines[self.ip - 1]))
                    };
                    self.stack.push(-top);
                },
                OP_ADD => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(a + b);
                },
                OP_SUBTRACT => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(a - b);
                },
                OP_MULTIPLY => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(a * b);
                },
                OP_DIVIDE => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(a / b);
                }
                _ => return Err(InterpretError::RuntimeError(format!("unknown opcode {:04}", instruction).to_string(), self.chunk.lines[self.ip - 1]))
            }
        }
    }
}