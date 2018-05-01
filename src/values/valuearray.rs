use values::value::Value;

pub struct ValueArray {
    pub capacity: u8,
    pub count: u8,
    pub values: Vec<Value>
}

impl ValueArray {
    pub fn init() -> ValueArray {
        ValueArray {
            count: 0,
            capacity: 0,
            values: Vec::new()
        }
    }

    pub fn build(values: Vec<Value>) -> ValueArray {
        ValueArray {
            capacity: values.len() as u8,
            count: values.len() as u8,
            values
        }
    }

    pub fn write(&mut self, value:Value) {
        if self.capacity < self.count + 1 {
            let old_capacity: usize = self.capacity as usize;
            self.capacity = ::util::grow_capacity(old_capacity) as u8;
            self.values.resize(self.capacity as usize, Value::Nil);
        }

        self.values[self.count as usize] = value;
        self.count += 1;
    }
}