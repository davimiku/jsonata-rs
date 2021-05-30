use std::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use serde_json::{Number, Value};

pub(crate) struct JSONataNumber(Number);

#[derive(Debug, Clone, Copy)]
enum NType {
    PosInt(u64),
    NegInt(i64),
    Float(f64),
}

impl JSONataNumber {
    pub fn to_value(self) -> Value {
        Value::Number(self.0)
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
            match JSONataNumber::compare_f64_and_u64(-f, (-i) as u64) {
                Some(ordering) => Some(ordering.reverse()),
                None => None,
            }
        } else {
            JSONataNumber::compare_f64_and_u64(f, i as u64)
        }
    }
}

impl From<Number> for NType {
    fn from(n: Number) -> Self {
        if n.is_u64() {
            NType::PosInt(n.as_u64().unwrap())
        } else if n.is_i64() {
            NType::NegInt(n.as_i64().unwrap())
        } else if n.is_f64() {
            NType::Float(n.as_f64().unwrap())
        } else {
            // Not possible, serde_json Number
            // can only be one of those 3 as enforced
            // by its internal (private) enum
            unreachable!()
        }
    }
}

impl From<&Number> for NType {
    fn from(n: &Number) -> Self {
        if n.is_u64() {
            NType::PosInt(n.as_u64().unwrap())
        } else if n.is_i64() {
            NType::NegInt(n.as_i64().unwrap())
        } else if n.is_f64() {
            NType::Float(n.as_f64().unwrap())
        } else {
            // Not possible, serde_json Number
            // can only be one of those 3 as enforced
            // by its internal (private) enum
            unreachable!()
        }
    }
}

impl From<Number> for JSONataNumber {
    fn from(n: Number) -> Self {
        JSONataNumber(n)
    }
}

impl From<&Number> for JSONataNumber {
    fn from(n: &Number) -> Self {
        JSONataNumber(n.clone())
    }
}

impl From<i64> for JSONataNumber {
    fn from(i: i64) -> Self {
        JSONataNumber(i.into())
    }
}

impl From<u64> for JSONataNumber {
    fn from(u: u64) -> Self {
        JSONataNumber(u.into())
    }
}

impl From<f64> for JSONataNumber {
    fn from(f: f64) -> Self {
        JSONataNumber(Number::from_f64(f).unwrap())
    }
}

impl PartialOrd for JSONataNumber {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let a = &self.0;
        let b = &other.0;

        // u64 > u64 ?
        if a.is_u64() && b.is_u64() {
            a.as_u64().partial_cmp(&b.as_u64())

        // i64 > i64 ?
        } else if a.is_i64() && b.is_i64() {
            a.as_i64().partial_cmp(&b.as_i64())

        // u64 > i64 ?
        } else if a.is_u64() && b.is_i64() {
            // a must be between i64::MAX and u64::MAX, so larger than b
            Some(Ordering::Greater)

        // i64 > u64 ?
        } else if a.is_i64() && b.is_u64() {
            // b must be between i64::MAX and u64::MAX, so larger than a
            Some(Ordering::Less)

        // f64 > f64 ?
        } else if a.is_f64() && b.is_f64() {
            a.as_f64().partial_cmp(&b.as_f64())

        // i64 > f64 ?
        } else if a.is_i64() && b.is_f64() {
            match JSONataNumber::compare_f64_and_i64(b.as_f64().unwrap(), a.as_i64().unwrap()) {
                Some(ordering) => Some(ordering.reverse()),
                None => None,
            }

        // f64 > i64 ?
        } else if a.is_f64() && b.is_i64() {
            JSONataNumber::compare_f64_and_i64(a.as_f64().unwrap(), b.as_i64().unwrap())

        // u64 > f64 ?
        } else if a.is_u64() && b.is_f64() {
            match JSONataNumber::compare_f64_and_u64(b.as_f64().unwrap(), a.as_u64().unwrap()) {
                Some(ordering) => Some(ordering.reverse()),
                None => None,
            }

        // f64 > u64 ?
        } else if a.is_f64() && b.is_u64() {
            JSONataNumber::compare_f64_and_u64(a.as_f64().unwrap(), b.as_u64().unwrap())

        // Should be unreachable
        } else {
            unreachable!()
        }
    }
}

impl PartialEq for JSONataNumber {
    fn eq(&self, other: &Self) -> bool {
        let self_type: NType = (&self.0).into();
        let other_type: NType = (&other.0).into();

        match (self_type, other_type) {
            (NType::PosInt(_), NType::NegInt(_)) => false,
            (NType::NegInt(_), NType::PosInt(_)) => false,
            (NType::PosInt(a), NType::PosInt(b)) => a == b,
            (NType::NegInt(a), NType::NegInt(b)) => a == b,
            (NType::Float(a), NType::Float(b)) => a == b,
            (NType::NegInt(i), NType::Float(f)) => (i as f64) == f,
            (NType::PosInt(u), NType::Float(f)) => (u as f64) == f,
            (NType::Float(f), NType::NegInt(i)) => f == (i as f64),
            (NType::Float(f), NType::PosInt(u)) => f == (u as f64),
        }
    }
}

impl Neg for JSONataNumber {
    type Output = JSONataNumber;

    fn neg(self) -> Self::Output {
        let self_type: NType = (&self.0).into();
        match self_type {
            NType::NegInt(i) => JSONataNumber::from(-i),
            NType::PosInt(u) => JSONataNumber::from(-(u as i64)),
            NType::Float(f) => JSONataNumber::from(-f),
        }
    }
}

impl Add for JSONataNumber {
    type Output = JSONataNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let self_type: NType = (&self.0).into();
        let rhs_type: NType = (&rhs.0).into();
        match (self_type, rhs_type) {
            (NType::NegInt(a), NType::NegInt(b)) => (a + b).into(),
            (NType::PosInt(a), NType::PosInt(b)) => (a + b).into(),
            (NType::Float(a), NType::Float(b)) => (a + b).into(),

            (NType::NegInt(i), NType::PosInt(u)) => (i + u as i64).into(),
            (NType::PosInt(u), NType::NegInt(i)) => (i + u as i64).into(),

            (NType::NegInt(i), NType::Float(f)) => (f + i as f64).into(),
            (NType::PosInt(u), NType::Float(f)) => (f + u as f64).into(),
            (NType::Float(f), NType::NegInt(i)) => (f + i as f64).into(),
            (NType::Float(f), NType::PosInt(u)) => (f + u as f64).into(),
        }
    }
}

impl Sub for JSONataNumber {
    type Output = JSONataNumber;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for JSONataNumber {
    type Output = JSONataNumber;

    fn mul(self, rhs: Self) -> Self::Output {
        let self_type: NType = (&self.0).into();
        let rhs_type: NType = (&rhs.0).into();
        match (self_type, rhs_type) {
            (NType::NegInt(a), NType::NegInt(b)) => (a * b).into(),
            (NType::PosInt(a), NType::PosInt(b)) => (a * b).into(),
            (NType::Float(a), NType::Float(b)) => (a * b).into(),

            (NType::NegInt(i), NType::PosInt(u)) => (i * u as i64).into(),
            (NType::PosInt(u), NType::NegInt(i)) => (i * u as i64).into(),

            (NType::NegInt(i), NType::Float(f)) => (f * i as f64).into(),
            (NType::PosInt(u), NType::Float(f)) => (f * u as f64).into(),
            (NType::Float(f), NType::NegInt(i)) => (f * i as f64).into(),
            (NType::Float(f), NType::PosInt(u)) => (f * u as f64).into(),
        }
    }
}

impl Div for JSONataNumber {
    type Output = JSONataNumber;

    fn div(self, rhs: Self) -> Self::Output {
        let self_type: NType = (&self.0).into();
        let rhs_type: NType = (&rhs.0).into();

        match (self_type, rhs_type) {
            (NType::NegInt(a), NType::NegInt(b)) => ((a / b) as f64).into(),
            (NType::PosInt(a), NType::PosInt(b)) => ((a / b) as f64).into(),
            (NType::Float(a), NType::Float(b)) => ((a / b) as f64).into(),

            (NType::NegInt(i), NType::PosInt(u)) => ((i / u as i64) as f64).into(),
            (NType::PosInt(u), NType::NegInt(i)) => ((i / u as i64) as f64).into(),

            (NType::NegInt(i), NType::Float(f)) => (f / i as f64).into(),
            (NType::PosInt(u), NType::Float(f)) => (f / u as f64).into(),
            (NType::Float(f), NType::NegInt(i)) => (f / i as f64).into(),
            (NType::Float(f), NType::PosInt(u)) => (f / u as f64).into(),
        }
    }
}

impl Rem for JSONataNumber {
    type Output = JSONataNumber;

    fn rem(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn eq() {
        let cases: Vec<(JSONataNumber, JSONataNumber)> = vec![
            (1_u64.into(), 1_u64.into()),
            (1_u64.into(), 1.0.into()),
            (1.0.into(), 1_u64.into()),
            (1.0.into(), 1.0.into()),
            ((-1_i64).into(), (-1_i64).into()),
            ((-1_i64).into(), (-1.0).into()),
            ((-1.0).into(), (-1_i64).into()),
        ];

        for (a, b) in cases {
            assert!(a == b);
        }
    }

    #[test]
    fn ne() {
        let cases: Vec<(JSONataNumber, JSONataNumber)> = vec![
            (1_u64.into(), 2_u64.into()),
            (1_u64.into(), (-1_i64).into()),
            (1_u64.into(), 1.1.into()),
            ((-1_i64).into(), (-1.1).into()),
        ];
        for (a, b) in cases {
            assert!(a != b);
        }
    }
}
