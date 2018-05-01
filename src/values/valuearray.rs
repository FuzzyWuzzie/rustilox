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

    fn grow_capacity(capacity:u8) -> u8 {
        if capacity < 8 {
            8
        }
        else {
            capacity * 2
        }
    }

    pub fn write(&mut self, value:Value) {
        if self.capacity < self.count + 1 {
            let old_capacity = self.capacity;
            self.capacity = Self::grow_capacity(old_capacity);
            self.values.resize(self.capacity as usize, Value::Null);
        }

        self.values[self.count as usize] = value;
        self.count += 1;
    }
}