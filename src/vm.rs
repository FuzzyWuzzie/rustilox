use std::cmp;

use chunk::Chunk;
use opcodes::*;
use values::Value;
use errors::LoxError;

pub struct VM {
    pub chunk: Box<Chunk>,
    pub ip: usize,
    pub stack: Vec<Value>
}

impl VM {
    pub fn init() -> VM {
        VM {
            chunk: Box::new(Chunk::init()),
            ip: 0,
            stack: Vec::new()
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.chunk.code[self.ip - 1]
    }

    fn read_constant(&mut self) -> &Value {
        let loc = self.read_byte();
        &self.chunk.constants.values[loc as usize]
    }

    fn binary_op(&mut self) -> Result<(Value, Value), LoxError> {
        let b = match self.stack.pop() {
            Some(v) => v,
            None => return Err(LoxError::CompileError("stack underflow".to_string(), self.chunk.lines[self.ip - 1]))
        };
        let a = match self.stack.pop() {
            Some(v) => v,
            None => return Err(LoxError::CompileError("stack underflow".to_string(), self.chunk.lines[self.ip - 1]))
        };

        Ok((a, b))
    }

    pub fn evaluate(&mut self) -> Result<Value, LoxError> {
        if self.chunk.count == 0 {
            return Err(LoxError::InterpetError("Chunk is empty".to_string(), 0));
        }

        loop {
            if cfg!(feature = "trace_execution") {
                print!("          ");
                for value in &self.stack {
                    print!("[ {} ]", value);
                }
                println!();
                print!("{}", self.chunk.get_instruction(self.ip));
            }

            let instruction = self.read_byte();
            match instruction {
                OP_RETURN => {
                    let top = match self.stack.pop() {
                        Some(v) => v,
                        None => Value::Nil
                    };
                    println!("{}", top);

                    return Ok(top);
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
                        None => return Err(LoxError::RuntimeError("stack underflow".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                    match -top {
                        Some(v) => self.stack.push(v),
                        None => return Err(LoxError::RuntimeError("can't negate a non-numeric value".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_ADD => {
                    let (a, b) = self.binary_op()?;
                    match a + b {
                        Some(v) => self.stack.push(v),
                        None => return Err(LoxError::RuntimeError("can't add values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_SUBTRACT => {
                    let (a, b) = self.binary_op()?;
                    match a - b {
                        Some(v) => self.stack.push(v),
                        None => return Err(LoxError::RuntimeError("can't subtract values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_MULTIPLY => {
                    let (a, b) = self.binary_op()?;
                    match a * b {
                        Some(v) => self.stack.push(v),
                        None => return Err(LoxError::RuntimeError("can't multiply values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_DIVIDE => {
                    let (a, b) = self.binary_op()?;
                    match a / b {
                        Some(v) => self.stack.push(v),
                        None => return Err(LoxError::RuntimeError("can't divide values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_NOT => {
                    let top = match self.stack.pop() {
                        Some(v) => v,
                        None => return Err(LoxError::RuntimeError("stack underflow".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                    match !top {
                        Some(v) => self.stack.push(v),
                        None => return Err(LoxError::RuntimeError("can't ! a non-boolean value".to_string(), self.chunk.lines[self.ip - 1]))
                    };
                },

                OP_EQUAL => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(Value::Boolean(a == b));
                },
                OP_NOTEQUAL => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(Value::Boolean(a != b));
                },
                OP_GREATER => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Greater => Value::Boolean(true),
                            _ => Value::Boolean(false)
                        },
                        None => return Err(LoxError::RuntimeError("can't compare values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    });
                },
                OP_GREATEREQUAL => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Less => Value::Boolean(false),
                            _ => Value::Boolean(true)
                        },
                        None => return Err(LoxError::RuntimeError("can't compare values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    });
                },
                OP_LESSER => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Less => Value::Boolean(true),
                            _ => Value::Boolean(false)
                        },
                        None => return Err(LoxError::RuntimeError("can't compare values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    });
                },
                OP_LESSEREQUAL => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Greater => Value::Boolean(false),
                            _ => Value::Boolean(true)
                        },
                        None => return Err(LoxError::RuntimeError("can't compare values of differing types".to_string(), self.chunk.lines[self.ip - 1]))
                    });
                },
                
                _ => return Err(LoxError::CompileError(format!("unknown opcode {:04}", instruction).to_string(), self.chunk.lines[self.ip - 1]))
            }
        }
    }
}