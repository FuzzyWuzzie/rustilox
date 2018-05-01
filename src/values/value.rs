use std::fmt;
use std::ops;

#[derive(Debug,Clone)]
pub enum Value {
    Null,
    Real(f64),
    _Natural(i64)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Value::Null => write!(f, "null"),
            Value::Real(v) => write!(f, "{}", v),
            Value::_Natural(v) => write!(f, "{}", v)
        }
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Real(v) => Value::Real(-v),
            Value::_Natural(v) => Value::_Natural(-v)
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs:Value) -> Value {
        match self {
            Value::Null => rhs,
            Value::Real(l) => match rhs {
                Value::Null => Value::Real(l),
                Value::Real(r) => Value::Real(l + r),
                Value::_Natural(r) => Value::Real(l + (r as f64))
            },
            Value::_Natural(l) => match rhs {
                Value::Null => Value::_Natural(l),
                Value::Real(r) => Value::Real((l as f64) + r),
                Value::_Natural(r) => Value::_Natural(l + r)
            }
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs:Value) -> Value {
        match self {
            Value::Null => -rhs,
            Value::Real(l) => match rhs {
                Value::Null => Value::Real(l),
                Value::Real(r) => Value::Real(l - r),
                Value::_Natural(r) => Value::Real(l - (r as f64))
            },
            Value::_Natural(l) => match rhs {
                Value::Null => Value::_Natural(l),
                Value::Real(r) => Value::Real((l as f64) - r),
                Value::_Natural(r) => Value::_Natural(l - r)
            }
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs:Value) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Real(l) => match rhs {
                Value::Null => Value::Null,
                Value::Real(r) => Value::Real(l * r),
                Value::_Natural(r) => Value::Real(l * (r as f64))
            },
            Value::_Natural(l) => match rhs {
                Value::Null => Value::Null,
                Value::Real(r) => Value::Real((l as f64) * r),
                Value::_Natural(r) => Value::_Natural(l * r)
            }
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs:Value) -> Value {
        match self {
            Value::Null => Value::Null,
            Value::Real(l) => match rhs {
                Value::Null => Value::Null,
                Value::Real(r) => Value::Real(l / r),
                Value::_Natural(r) => Value::Real(l / (r as f64))
            },
            Value::_Natural(l) => match rhs {
                Value::Null => Value::Null,
                Value::Real(r) => Value::Real((l as f64) / r),
                Value::_Natural(r) => Value::_Natural(l / r)
            }
        }
    }
}
