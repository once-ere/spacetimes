//! Exact rational numbers and exact complex-rational numbers.
//!
//! PURPOSE.  The statements about the geometry of the deflating spacetimes and about
//! their Dirac matrices are identities between rational expressions in the deflation
//! rates and in the constants of the metric, once they are evaluated at the point
//! x4 = 0 (every other point is reached from it by an isometry, see `metric.rs`).
//! With rational rates and constants they can therefore be verified by exact
//! arithmetic, with no rounding at all.  This module supplies that arithmetic.
//!
//! `Rat` is a fraction of two 128-bit integers, always reduced, with a positive
//! denominator.  The program is compiled with overflow checks, so that an overflow
//! stops the program instead of producing a wrong result.
//! `Qc` is a complex number whose real and imaginary parts are `Rat`.

use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Greatest common divisor of two non-negative integers.
fn gcd(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a.abs()
}

/// A rational number num/den, reduced, den > 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rat {
    pub num: i128,
    pub den: i128,
}

impl Rat {
    pub const ZERO: Rat = Rat { num: 0, den: 1 };
    pub const ONE: Rat = Rat { num: 1, den: 1 };

    /// The fraction n/d (d must not be zero).
    pub fn new(n: i128, d: i128) -> Rat {
        assert!(d != 0, "Rat: zero denominator");
        let g = gcd(n.abs(), d.abs()).max(1);
        let s = if d < 0 { -1 } else { 1 };
        Rat { num: s * n / g, den: s * d / g }
    }

    /// The integer n.
    pub fn int(n: i128) -> Rat {
        Rat { num: n, den: 1 }
    }

    pub fn is_zero(self) -> bool {
        self.num == 0
    }

    /// 1/self (self must not be zero).
    pub fn recip(self) -> Rat {
        Rat::new(self.den, self.num)
    }

    pub fn to_f64(self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

impl Add for Rat {
    type Output = Rat;
    fn add(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den + o.num * self.den, self.den * o.den)
    }
}
impl Sub for Rat {
    type Output = Rat;
    fn sub(self, o: Rat) -> Rat {
        Rat::new(self.num * o.den - o.num * self.den, self.den * o.den)
    }
}
impl Mul for Rat {
    type Output = Rat;
    fn mul(self, o: Rat) -> Rat {
        Rat::new(self.num * o.num, self.den * o.den)
    }
}
impl Div for Rat {
    type Output = Rat;
    fn div(self, o: Rat) -> Rat {
        self * o.recip()
    }
}
impl Neg for Rat {
    type Output = Rat;
    fn neg(self) -> Rat {
        Rat { num: -self.num, den: self.den }
    }
}
impl fmt::Display for Rat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.den == 1 {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

/// A complex number with rational real and imaginary parts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Qc {
    pub re: Rat,
    pub im: Rat,
}

impl Qc {
    pub const ZERO: Qc = Qc { re: Rat::ZERO, im: Rat::ZERO };
    pub const ONE: Qc = Qc { re: Rat::ONE, im: Rat::ZERO };
    pub const I: Qc = Qc { re: Rat::ZERO, im: Rat::ONE };

    pub fn real(r: Rat) -> Qc {
        Qc { re: r, im: Rat::ZERO }
    }
    pub fn int(n: i128) -> Qc {
        Qc::real(Rat::int(n))
    }
    pub fn conj(self) -> Qc {
        Qc { re: self.re, im: -self.im }
    }
    pub fn is_zero(self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
}
impl Add for Qc {
    type Output = Qc;
    fn add(self, o: Qc) -> Qc {
        Qc { re: self.re + o.re, im: self.im + o.im }
    }
}
impl Sub for Qc {
    type Output = Qc;
    fn sub(self, o: Qc) -> Qc {
        Qc { re: self.re - o.re, im: self.im - o.im }
    }
}
impl Mul for Qc {
    type Output = Qc;
    fn mul(self, o: Qc) -> Qc {
        Qc { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
}
impl Neg for Qc {
    type Output = Qc;
    fn neg(self) -> Qc {
        Qc { re: -self.re, im: -self.im }
    }
}

/// The arithmetic that the geometry needs, so that the same routines run with exact
/// rationals (proofs at x4 = 0) and with floating-point numbers (any point).
pub trait Field: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self> + Neg<Output = Self> + PartialEq {
    fn zero() -> Self;
    fn one() -> Self;
    /// The integer n as an element of the field.
    fn from_int(n: i64) -> Self;
    fn to_f64(self) -> f64;
}

impl Field for Rat {
    fn zero() -> Rat {
        Rat::ZERO
    }
    fn one() -> Rat {
        Rat::ONE
    }
    fn from_int(n: i64) -> Rat {
        Rat::int(n as i128)
    }
    fn to_f64(self) -> f64 {
        Rat::to_f64(self)
    }
}

impl Field for f64 {
    fn zero() -> f64 {
        0.0
    }
    fn one() -> f64 {
        1.0
    }
    fn from_int(n: i64) -> f64 {
        n as f64
    }
    fn to_f64(self) -> f64 {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rational_arithmetic_is_exact() {
        let a = Rat::new(6, -4); // = -3/2, reduced, positive denominator
        assert_eq!((a.num, a.den), (-3, 2));
        let b = Rat::new(5, 7);
        assert_eq!(a + b, Rat::new(-11, 14));
        assert_eq!(a - b, Rat::new(-31, 14));
        assert_eq!(a * b, Rat::new(-15, 14));
        assert_eq!(a / b, Rat::new(-21, 10));
        assert_eq!(-a, Rat::new(3, 2));
        assert_eq!(a.recip(), Rat::new(-2, 3));
        // (1/3 + 1/6) * 2 = 1 exactly
        assert_eq!((Rat::new(1, 3) + Rat::new(1, 6)) * Rat::int(2), Rat::ONE);
    }

    #[test]
    fn complex_rational_arithmetic_is_exact() {
        let z = Qc { re: Rat::new(1, 2), im: Rat::new(-1, 3) };
        // z * conj(z) = 1/4 + 1/9 = 13/36
        assert_eq!(z * z.conj(), Qc::real(Rat::new(13, 36)));
        assert_eq!(Qc::I * Qc::I, -Qc::ONE);
        assert_eq!(z - z, Qc::ZERO);
    }
}
