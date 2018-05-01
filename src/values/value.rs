use std::fmt;
use std::ops;

#[derive(Debug,Clone)]
pub enum Value {
    Null,
    Float(f64)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => write!(f, "null"),
            Value::Float(v) => write!(f, "{}", v)
        }
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Float(v) => Value::Float(-v)
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs:Value) -> Value {
        match self {
            Value::Null => rhs,
            Value::Float(l) => match rhs {
                Value::Null => Value::Float(l),
                Value::Float(r) => Value::Float(l + r)
            }
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs:Value) -> Value {
        match self {
            Value::Null => -rhs,
            Value::Float(l) => match rhs {
                Value::Null => Value::Float(l),
                Value::Float(r) => Value::Float(l - r)
            }
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs:Value) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Float(l) => match rhs {
                Value::Null => Value::Null,
                Value::Float(r) => Value::Float(l * r)
            }
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs:Value) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Float(l) => match rhs {
                Value::Null => Value::Null,
                Value::Float(r) => Value::Float(l / r)
            }
        }
    }
}
