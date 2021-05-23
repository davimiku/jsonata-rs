//! Define a custom number type JSONataNumber
//! to be PartialEq and PartialOrd

use core::f64;
use std::cmp::Ordering;

use serde_json::Number;

#[derive(Clone, Debug)]
pub(super) enum JSONataNumber {
    NegInt(i64),
    PosInt(u64),
    Float(f64),
}

impl JSONataNumber {
    pub fn new(n: Number) -> Self {
        match (n.is_i64(), n.is_u64(), n.is_f64()) {
            // NegInt -- i64
            (true, false, false) => JSONataNumber::NegInt(n.as_i64().unwrap()),

            // PosInt -- u64
            (false, true, false) => JSONataNumber::PosInt(n.as_u64().unwrap()),

            // Float -- f64
            (false, false, true) => JSONataNumber::Float(n.as_f64().unwrap()),

            // serde_json guarantees this case is unreachable
            (_, _, _) => unreachable!(),
        }
    }
}

impl From<Number> for JSONataNumber {
    fn from(n: Number) -> Self {
        JSONataNumber::new(n)
    }
}

impl From<&Number> for JSONataNumber {
    fn from(n: &Number) -> Self {
        JSONataNumber::new(n.clone())
    }
}

impl From<u64> for JSONataNumber {
    fn from(u: u64) -> Self {
        JSONataNumber::PosInt(u)
    }
}

impl From<i64> for JSONataNumber {
    fn from(i: i64) -> Self {
        JSONataNumber::NegInt(i)
    }
}

impl From<f64> for JSONataNumber {
    fn from(f: f64) -> Self {
        JSONataNumber::Float(f)
    }
}

impl PartialEq for JSONataNumber {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JSONataNumber::PosInt(_), JSONataNumber::NegInt(_)) => false,
            (JSONataNumber::NegInt(_), JSONataNumber::PosInt(_)) => false,

            (JSONataNumber::PosInt(a), JSONataNumber::PosInt(b)) => a == b,
            (JSONataNumber::NegInt(a), JSONataNumber::NegInt(b)) => a == b,
            (JSONataNumber::Float(a), JSONataNumber::Float(b)) => a == b,

            (JSONataNumber::NegInt(i), JSONataNumber::Float(f)) => (*i as f64) == *f,
            (JSONataNumber::PosInt(u), JSONataNumber::Float(f)) => (*u as f64) == *f,
            (JSONataNumber::Float(f), JSONataNumber::NegInt(i)) => *f == (*i as f64),
            (JSONataNumber::Float(f), JSONataNumber::PosInt(u)) => *f == (*u as f64),
        }
    }
}

impl PartialOrd for JSONataNumber {
    /// Implement comparison for numbers that may be u64, i64, or f64.
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            // Same types
            (JSONataNumber::NegInt(a), JSONataNumber::NegInt(b)) => a.partial_cmp(b),
            (JSONataNumber::PosInt(a), JSONataNumber::PosInt(b)) => a.partial_cmp(b),
            (JSONataNumber::Float(a), JSONataNumber::Float(b)) => a.partial_cmp(b),

            // Integers
            (JSONataNumber::NegInt(_), JSONataNumber::PosInt(_)) => Some(Ordering::Less),
            (JSONataNumber::PosInt(_), JSONataNumber::NegInt(_)) => Some(Ordering::Greater),

            // Floats and positive ints
            (JSONataNumber::Float(f), JSONataNumber::PosInt(u)) => {
                JSONataNumber::compare_f64_and_u64(*f, *u)
            }
            (JSONataNumber::PosInt(u), JSONataNumber::Float(f)) => {
                match JSONataNumber::compare_f64_and_u64(*f, *u) {
                    Some(ordering) => Some(ordering.reverse()),
                    None => None,
                }
            }

            // Floats and negative ints
            (JSONataNumber::Float(f), JSONataNumber::NegInt(i)) => {
                JSONataNumber::compare_f64_and_i64(*f, *i)
            }
            (JSONataNumber::NegInt(i), JSONataNumber::Float(f)) => {
                match JSONataNumber::compare_f64_and_i64(*f, *i) {
                    Some(ordering) => Some(ordering.reverse()),
                    None => None,
                }
            }
        }
    }
}

impl JSONataNumber {
    fn compare_f64_and_u64(f: f64, u: u64) -> Option<Ordering> {
        if u < (f64::MAX as u64) {
            f.partial_cmp(&(u as f64))
        } else if u < (f.floor() as u64) {
            Some(Ordering::Greater)
        } else if u > (f.ceil() as u64) {
            Some(Ordering::Less)
        } else {
            None
        }
    }

    fn compare_f64_and_i64(f: f64, i: i64) -> Option<Ordering> {
        if i < 0 {
            match JSONataNumber::compare_f64_and_u64(-f, -i as u64) {
                Some(ordering) => Some(ordering.reverse()),
                None => None,
            }
        } else {
            // This case is impossible as i64 only exist in NegInt variants
            JSONataNumber::compare_f64_and_u64(f, i as u64)
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn equals() {
        assert!(JSONataNumber::from(1_u64) == JSONataNumber::from(1_u64));

        assert!(!(JSONataNumber::from(1_u64) == JSONataNumber::from(2_u64)));
        assert!(!(JSONataNumber::from(1_u64) == JSONataNumber::from(-1_i64)));

        assert!(JSONataNumber::from(1_u64) == JSONataNumber::from(1.0));
        assert!(JSONataNumber::from(-1_i64) == JSONataNumber::from(-1.0));

        assert!(!(JSONataNumber::from(1_u64) == JSONataNumber::from(1.1)));
        assert!(!(JSONataNumber::from(-1_i64) == JSONataNumber::from(-1.1)));
    }

    #[test]
    fn greater_than() {
        assert!(JSONataNumber::from(2_u64) > JSONataNumber::from(1_u64));
        assert!(JSONataNumber::from(-1_i64) > JSONataNumber::from(-2_i64));
        assert!(JSONataNumber::from(2.1) > JSONataNumber::from(1.1));

        assert!(JSONataNumber::from(2_u64) > JSONataNumber::from(-2_i64));
        assert!(!(JSONataNumber::from(-2_i64) > JSONataNumber::from(2_u64)));

        assert!(JSONataNumber::from(2_u64) > JSONataNumber::from(1.1));
        assert!(!(JSONataNumber::from(2_u64) > JSONataNumber::from(2.1)));

        assert!(JSONataNumber::from(-2_i64) > JSONataNumber::from(-2.1));
        assert!(!(JSONataNumber::from(-2_i64) > JSONataNumber::from(-1.1)));

        assert!(JSONataNumber::from(i64::MIN + 1) > JSONataNumber::from(f64::MIN + 1.0));
        assert!(!(JSONataNumber::from(f64::MIN + 1.0) > JSONataNumber::from(i64::MIN + 1)));
    }
}
