/*!
Work with numbers.

This module provides big integer types.
*/

use std::str::FromStr;

// Define:
// - `BigInt`
// - `NonNegBigInt`
pub use num_bigint::{
    BigInt,
    BigUint as NonNegBigInt,
};

use crate::reg_exp::*;

/// Allows separating a number into commas for every 3 digits,
/// such as `10,000`.
/// 
/// # Example
/// 
/// ```
/// use rialight_util::number::CommaSeparated;
/// assert_eq!("1,000,000", 1_000_000i64.comma_separated());
/// ```
pub trait CommaSeparated {
    fn comma_separated(&self) -> String;
}

impl CommaSeparated for NonNegBigInt {
    fn comma_separated(&self) -> String {
        let s = self.to_string();
        let m = s.len() % 3;
        let mut r = String::new();
        for (i, digit) in s.char_indices() {
            if i != 0 && i % 3 == m {
                r.push(',');
            }
            r.push(digit);
        }
        r
    }
}

impl CommaSeparated for BigInt {
    fn comma_separated(&self) -> String {
        let neg = self.sign() == num_bigint::Sign::Minus;
        let n = if neg { -self } else { self.clone() };
        (if neg { "-" } else { "" }).to_owned() + &n.to_biguint().unwrap().comma_separated()
    }
}

impl CommaSeparated for i128 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for u128 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for isize {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for usize {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for i64 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for u64 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for i32 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for u32 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for i16 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for u16 {
    fn comma_separated(&self) -> String {
        BigInt::from(*self).comma_separated()
    }
}

impl CommaSeparated for f64 {
    fn comma_separated(&self) -> String {
        if self.is_infinite() || self.is_nan() {
            return self.to_string();
        }
        let s = &self.to_string();
        let mut split = reg_exp!(r"\.").split(s);
        let i = split.next().unwrap();
        let d = split.next();
        let d = if d.is_none() { "".to_owned() } else { ".".to_owned() + d.unwrap() };
        BigInt::from_str(i).unwrap().comma_separated() + &d
    }
}

impl CommaSeparated for f32 {
    fn comma_separated(&self) -> String {
        f64::from(*self).comma_separated()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn comma() {
        assert_eq!("1,000,000.5", 1_000_000.5f64.comma_separated());
        assert_eq!("-1,000,000.5", (-1_000_000.5f64).comma_separated());
    }
}