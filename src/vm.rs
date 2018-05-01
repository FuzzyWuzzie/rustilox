use std::cmp;

use ::chunk::Chunk;
use ::opcodes::*;
use ::values::Value;
use ::errors::InterpretError;

pub struct VM<'a> {
    pub chunk: &'a Chunk,
    ip: usize,
    stack: Vec<Value>
}

impl<'a> VM<'a> {
    pub fn init(chunk: &Chunk) -> VM {
        VM {
            chunk: chunk,
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

    fn binary_op(&mut self) -> Result<(Value, Value), InterpretError> {
        let b = match self.stack.pop() {
            Some(v) => v,
            None => return Err(InterpretError::CompileError(format!("stack underflow"), self.chunk.lines[self.ip - 1]))
        };
        let a = match self.stack.pop() {
            Some(v) => v,
            None => return Err(InterpretError::CompileError(format!("stack underflow"), self.chunk.lines[self.ip - 1]))
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
                        None => return Err(InterpretError::RuntimeError(format!("stack underflow"), self.chunk.lines[self.ip - 1]))
                    };
                    match -top {
                        Some(v) => self.stack.push(v),
                        None => return Err(InterpretError::RuntimeError(format!("can't negate a non-numeric value"), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_ADD => {
                    let (a, b) = self.binary_op()?;
                    match a + b {
                        Some(v) => self.stack.push(v),
                        None => return Err(InterpretError::RuntimeError(format!("can't add values of differing types"), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_SUBTRACT => {
                    let (a, b) = self.binary_op()?;
                    match a - b {
                        Some(v) => self.stack.push(v),
                        None => return Err(InterpretError::RuntimeError(format!("can't subtract values of differing types"), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_MULTIPLY => {
                    let (a, b) = self.binary_op()?;
                    match a * b {
                        Some(v) => self.stack.push(v),
                        None => return Err(InterpretError::RuntimeError(format!("can't multiply values of differing types"), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_DIVIDE => {
                    let (a, b) = self.binary_op()?;
                    match a / b {
                        Some(v) => self.stack.push(v),
                        None => return Err(InterpretError::RuntimeError(format!("can't divide values of differing types"), self.chunk.lines[self.ip - 1]))
                    };
                },
                OP_NOT => {
                    let top = match self.stack.pop() {
                        Some(v) => v,
                        None => return Err(InterpretError::RuntimeError(format!("stack underflow"), self.chunk.lines[self.ip - 1]))
                    };
                    match !top {
                        Some(v) => self.stack.push(v),
                        None => return Err(InterpretError::RuntimeError(format!("can't ! a non-boolean value"), self.chunk.lines[self.ip - 1]))
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
                        None => return Err(InterpretError::RuntimeError(format!("can't compare values of differing types"), self.chunk.lines[self.ip - 1]))
                    });
                },
                OP_GREATEREQUAL => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Less => Value::Boolean(false),
                            _ => Value::Boolean(true)
                        },
                        None => return Err(InterpretError::RuntimeError(format!("can't compare values of differing types"), self.chunk.lines[self.ip - 1]))
                    });
                },
                OP_LESSER => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Less => Value::Boolean(true),
                            _ => Value::Boolean(false)
                        },
                        None => return Err(InterpretError::RuntimeError(format!("can't compare values of differing types"), self.chunk.lines[self.ip - 1]))
                    });
                },
                OP_LESSEREQUAL => {
                    let (a, b) = self.binary_op()?;
                    self.stack.push(match a.partial_cmp(&b) {
                        Some(o) => match o {
                            cmp::Ordering::Greater => Value::Boolean(false),
                            _ => Value::Boolean(true)
                        },
                        None => return Err(InterpretError::RuntimeError(format!("can't compare values of differing types"), self.chunk.lines[self.ip - 1]))
                    });
                },
                
                _ => return Err(InterpretError::CompileError(format!("unknown opcode {:04}", instruction).to_string(), self.chunk.lines[self.ip - 1]))
            }
        }
    }
}