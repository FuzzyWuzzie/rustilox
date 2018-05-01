use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug,Clone,PartialEq)]
pub enum Value {
    Nil,
    Real(f64),
    _Natural(i64),
    Boolean(bool)
}

use Value::Nil;
use Value::Real;
use Value::_Natural;
use Value::Boolean;

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Nil => write!(f, "Nil"),
            Real(v) => write!(f, "{}", v),
            _Natural(v) => write!(f, "{}", v),
            Boolean(v) => write!(f, "{}", v)
        }
    }
}

impl ops::Neg for Value {
    type Output = Value;

    fn neg(self) -> Self::Output {
        match self {
            Nil => Nil,
            Real(v) => Real(-v),
            _Natural(v) => _Natural(-v),
            Boolean(_) => Nil,
        }
    }
}

impl ops::Add for Value {
    type Output = Value;

    fn add(self, rhs:Value) -> Self::Output {
        match self {
            Nil => rhs,
            Real(l) => match rhs {
                Nil => Real(l),
                Real(r) => Real(l + r),
                _Natural(r) => Real(l + (r as f64)),
                Boolean(_) => Nil
            },
            _Natural(l) => match rhs {
                Nil => _Natural(l),
                Real(r) => Real((l as f64) + r),
                _Natural(r) => _Natural(l + r),
                Boolean(_) => Nil
            }
            Boolean(_) => Nil,
        }
    }
}

impl ops::Sub for Value {
    type Output = Value;

    fn sub(self, rhs:Value) -> Self::Output {
        match self {
            Nil => -rhs,
            Real(l) => match rhs {
                Nil => Real(l),
                Real(r) => Real(l - r),
                _Natural(r) => Real(l - (r as f64)),
                Boolean(_) => Nil
            },
            _Natural(l) => match rhs {
                Nil => _Natural(l),
                Real(r) => Real((l as f64) - r),
                _Natural(r) => _Natural(l - r),
                Boolean(_) => Nil
            }
            Boolean(_) => Nil,
        }
    }
}

impl ops::Mul for Value {
    type Output = Value;

    fn mul(self, rhs:Value) -> Self::Output {
        match self {
            Nil => Nil,
            Real(l) => match rhs {
                Nil => Nil,
                Real(r) => Real(l * r),
                _Natural(r) => Real(l * (r as f64)),
                Boolean(_) => Nil
            },
            _Natural(l) => match rhs {
                Nil => Nil,
                Real(r) => Real((l as f64) * r),
                _Natural(r) => _Natural(l * r),
                Boolean(_) => Nil
            }
            Boolean(_) => Nil,
        }
    }
}

impl ops::Div for Value {
    type Output = Value;

    fn div(self, rhs:Value) -> Self::Output {
        match self {
            Nil => Nil,
            Real(l) => match rhs {
                Nil => Nil,
                Real(r) => Real(l / r),
                _Natural(r) => Real(l / (r as f64)),
                Boolean(_) => Nil
            },
            _Natural(l) => match rhs {
                Nil => Nil,
                Real(r) => Real((l as f64) / r),
                _Natural(r) => _Natural(l / r),
                Boolean(_) => Nil
            }
            Boolean(_) => Nil,
        }
    }
}

impl ops::Not for Value {
    type Output = Option<Value>;

    fn not(self) -> Self::Output {
        match self {
            Boolean(v) => Some(Boolean(!v)),
            _ => None
        }
    }
}

impl cmp::PartialOrd for Value {
    fn partial_cmp(&self, rhs: &Value) -> Option<cmp::Ordering> {
        match (self, rhs) {
            (Nil, Nil) => Some(cmp::Ordering::Equal),
            (Real(a), Real(b)) => Some(a.partial_cmp(b)?),
            (_Natural(a), _Natural(b)) => Some(a.partial_cmp(b)?),
            (Boolean(a), Boolean(b)) => Some(a.partial_cmp(b)?),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_equality() {
        assert_eq!(_Natural(42), _Natural(42));
        assert_ne!(_Natural(-1), _Natural(1));
        assert_eq!(Real(4.2), Real(4.2));
        assert_eq!(Nil, Nil);
        assert_eq!(Boolean(true), Boolean(true));
        assert_ne!(Boolean(true), Boolean(false));
    }

    #[test]
    fn diff_equality() {
        assert_ne!(_Natural(1), Real(1.0));
    }

    #[test]
    fn same_partial_ord() {
        assert!(Real(4.2) > Real(1.5));
        assert!(Boolean(true) > Boolean(false));
    }

    #[test]
    #[should_panic]
    fn diff_partial_ord_panics() {
        assert!(Real(4.2) > Boolean(false));
    }
}