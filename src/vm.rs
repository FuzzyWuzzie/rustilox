use std::error;
use std::fmt;

use ::chunk::Chunk;
use ::opcodes::*;
use ::values::Value;

#[derive(Debug)]
pub enum InterpretError {
    _CompileError(String),
    RuntimeError(String)
}

impl error::Error for InterpretError {
    fn description(&self) -> &str {
        match self {
            InterpretError::_CompileError(_) => "Compile error",
            InterpretError::RuntimeError(_) => "Runtime error"
        }
    }
}

impl fmt::Display for InterpretError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpretError::_CompileError(d) => write!(f, "Compile error: {}", d),
            InterpretError::RuntimeError(d) => write!(f, "Runtime error: {}", d),
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

    pub fn free(&mut self) {
        self.stack.resize(0, Value::Null);
    }

    // TODO: move into a closure inside of interpret somehow?
    fn read_byte(&mut self) -> u8 {
        let b = self.chunk.code[self.ip];
        self.ip += 1;
        b
    }

    // TODO: move into a closure inside of interpret somehow?
    // TODO: figure out the fucking borrow checker
    //fn read_constant(&self, loc: u8) -> &Value {
    //    &self.chunk.constants.values[loc as usize]
    //}

    pub fn interpret(&mut self) -> Result<(), InterpretError> {
        //let read_byte = |vm: &mut VM| -> u8 {
        //    let b = vm.chunk.code[vm.ip];
        //    vm.ip += 1;
        //    b
        //};

        //let read_constant = |vm: &mut VM| -> &Value {
        //    let loc = read_byte(vm);
        //    &vm.chunk.constants.values[loc as usize]
        //};

        loop {
            if cfg!(feature = "trace_execution") {
                print!("          ");
                for value in &self.stack {
                    print!("[ {} ]", value);
                }
                println!();
                self.chunk.disassemble_instruction(self.ip);
            }

            let instruction = self.read_byte();//read_byte(self);
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
                    let loc = self.read_byte();
                    let constant = &self.chunk.constants.values[loc as usize];//self.read_constant(loc);//read_constant(self);
                    let new_constant = constant.clone();
                    self.stack.push(new_constant);
                },
                _ => return Err(InterpretError::RuntimeError(format!("unknown opcode {:04}", instruction).to_string()))
            }
        }
    }
}