#![warn(
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::nursery,
    clippy::suspicious,
    clippy::style,
)]
#![allow(
    clippy::semicolon_inside_block,
    clippy::just_underscores_and_digits,
)]

use std::ops::*;
use std::fmt::{self, Display, Formatter};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Float {
    pub mantissa: [Digit; 15],
    pub exponent: i8,
    pub sign: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digit(u8);

#[macro_export]
macro_rules! float {
    (($m: expr) e $e: expr) => {
        Float {
            mantissa: $crate::to_digits($m),
            exponent: $e,
            sign: false
        }
    };
    (- ($m: expr) e $e: expr) => {
        Float {
            mantissa: $crate::to_digits($m),
            exponent: $e,
            sign: true
        }
    };
}

pub const fn to_digits(mut d: u64) -> [Digit; 15] {
    let mut v: [Digit; 15] = [Digit(0); 15];
    let mut i = 15;

    while d != 0 {
        v[i-1] = Digit((d % 10) as u8);
        d /= 10;
        i -= 1;
    }

    v
}

pub fn from_digits(d: [Digit; 15]) -> u64 {
    d.into_iter().fold(0, |a, e| a * 10 + e.0 as u64)
}

impl Display for Float {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.sign {
            write!(f, "-")?;
        }
        write!(f, "{}e{}",
            self.mantissa.iter().enumerate().skip_while(|(i, a)| *i != 14 && a.0 == 0).map(|(_, a)| (b'0' + a.0) as char).collect::<Vec<char>>().into_iter().collect::<String>(),
            self.exponent
        )
    }
}

impl Add for Float {
    type Output = Option<Self>;
    fn add(self, rhs: Self) -> Option<Self> {
        let exponent = self.exponent.min(rhs.exponent);
        let lhs_sh = (self.exponent - exponent) as usize;
        let rhs_sh = ( rhs.exponent - exponent) as usize;

        let mut mantissa = [Digit(0); 15];
        let mut carry = 0;
        let mut reduced = 0;
        let mut i = 15;
        for (l, r) in shift(self.mantissa, lhs_sh).into_iter().zip(shift(rhs.mantissa, rhs_sh)).rev() {
            let a = l.0 + r.0 + carry;
            carry = a / 10;

            if a % 10 != 0 || i != 15 {
                i -= 1;
                mantissa[i] = Digit(a % 10);
            } else {
                reduced += 1;
            }
        }

        while carry != 0 {
            mantissa.rotate_right(1);
            mantissa[0] = Digit(carry % 10);
            carry /= 10;
            reduced += 1;
        }

        Some(Self {
            mantissa,
            exponent: check_exponent(exponent.checked_add(reduced)?)?,
            sign: self.sign ^ rhs.sign
        })
    }
}

impl Mul for Float {
    type Output = Option<Self>;
    fn mul(self, rhs: Self) -> Option<Self> {
        let mut mantissa = [Digit(0); 15];
        let mut carry = 0;
        let mut reduced = 0;
        let mut i = 15;
        let l = from_digits(self.mantissa);
        for r in rhs.mantissa.into_iter().rev() {
            let a = l * r.0 as u64 + carry;
            carry = a / 10;

            if a % 10 != 0 || i != 15 {
                i -= 1;
                mantissa[i] = Digit((a % 10) as u8);
            } else {
                reduced += 1;
            }
        }

        while carry != 0 {
            mantissa.rotate_right(1);
            mantissa[0] = Digit((carry % 10) as u8);
            carry /= 10;
            reduced += 1;
        }

        Some(Self {
            mantissa,
            exponent: check_exponent(check_exponent(self.exponent.checked_add(rhs.exponent)?)?.checked_add(reduced as i8)?)?,
            sign: self.sign ^ rhs.sign
        })
    }
}

const fn check_exponent(e: i8) -> Option<i8> {
    if e < -99 || e > 99 {
        return None;
    }

    Some(e)
}

fn shift(m: [Digit; 15], by: usize) -> [Digit; 15] {
    let mut r = [Digit(0); 15];

    for (i, d) in m.into_iter().enumerate() {
        if let Some(i) = i.checked_sub(by) {
            r[i] = d;
        } else {
            continue;
        }
    }

    r
}
