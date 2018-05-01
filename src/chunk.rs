use std::fmt;

use ::opcodes::*;
use ::values::{Value, ValueArray};

pub struct Chunk {
    pub code: Vec<u8>,
    pub count: usize,
    pub capacity: usize,
    pub constants: ValueArray,
    pub lines: Vec<usize>
}

pub struct Instruction<'a> {
    pub chunk: &'a Chunk,
    pub offset: usize
}

impl Chunk {
    pub fn init() -> Chunk {
        Chunk {
            count: 0,
            capacity: 0,
            code: Vec::new(),
            constants: ValueArray::init(),
            lines: Vec::new()
        }
    }

    pub fn build(constants: ValueArray, code: Vec<u8>, lines: Vec<usize>) -> Chunk {
        Chunk {
            count: code.len(),
            capacity: code.len(),
            code,
            constants,
            lines
        }
    }

    pub fn write(&mut self, byte:u8, line:usize) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = ::util::grow_capacity(old_capacity);
            self.code.resize(self.capacity, 0);
            self.lines.resize(self.capacity, 0);
        }

        self.code[self.count] = byte;
        self.lines[self.count] = line;
        self.count += 1;
    }

    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.write(value);
        self.constants.count - 1
    }

    fn simple_instruction(f: &mut fmt::Formatter, name:&str, offset:usize) -> Result<usize, fmt::Error> {
        writeln!(f, "{: >16}", name)?;
        Ok(offset + 1)
    }

    fn constant_instruction(&self, f: &mut fmt::Formatter, name: &str, offset: usize) -> Result<usize, fmt::Error> {
        let constant: u8 = self.code[offset + 1];
        writeln!(f, "{: >16} {:04} '{}'", name, constant, self.constants.values[constant as usize])?;
        Ok(offset + 2)
    }

    pub fn disassemble_instruction(&self, f: &mut fmt::Formatter, offset:usize) -> Result<usize, fmt::Error> {
        write!(f, "{:04} ", offset)?;
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            write!(f, "   | ")?;
        }
        else {
            write!(f, "{:04} ", self.lines[offset])?;
        }

        let instruction = self.code[offset];
        match instruction {
            OP_RETURN => Self::simple_instruction(f, "OP_RETURN", offset),
            OP_CONSTANT => self.constant_instruction(f, "OP_CONSTANT", offset),
            OP_NEGATE => Self::simple_instruction(f, "OP_NEGATE", offset),
            OP_ADD => Self::simple_instruction(f, "OP_ADD", offset),
            OP_SUBTRACT => Self::simple_instruction(f, "OP_SUBTRACT", offset),
            OP_MULTIPLY => Self::simple_instruction(f, "OP_MULTIPLY", offset),
            OP_DIVIDE => Self::simple_instruction(f, "OP_DIVIDE", offset),
            OP_NOT => Self::simple_instruction(f, "OP_NOT", offset),
            OP_EQUAL => Self::simple_instruction(f, "OP_EQUAL", offset),
            OP_NOTEQUAL => Self::simple_instruction(f, "OP_NOTEQUAL", offset),
            OP_GREATER => Self::simple_instruction(f, "OP_GREATER", offset),
            OP_GREATEREQUAL => Self::simple_instruction(f, "OP_GREATEREQUAL", offset),
            OP_LESSER => Self::simple_instruction(f, "OP_LESSER", offset),
            OP_LESSEREQUAL => Self::simple_instruction(f, "OP_LESSEREQUAL", offset),
            _ => {
                writeln!(f, "Unknown opcode {}", instruction)?;
                Ok(offset + 1)
            }
        }
    }

    pub fn get_instruction(&self, offset: usize) -> Instruction {
        Instruction {
            chunk: self,
            offset
        }
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut i:usize = 0;
        while i < self.count {
            i = self.disassemble_instruction(f, i)?;
        }
        
        Ok(())
    }
}

impl<'a> fmt::Display for Instruction<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.chunk.disassemble_instruction(f, self.offset) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}
