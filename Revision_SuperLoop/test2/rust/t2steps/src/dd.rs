//! Real and complex numbers with twice the precision of f64 ("double-double": a number is the unevaluated sum
//! hi + lo of two f64 with |lo| at most half a unit in the last place of hi; about 31 significant decimal digits).
//!
//! WHY.  The module `formulas` computes, for one step of a linear multistep formula, the factor by which the formula
//! multiplies an oscillation, and compares it with the exact factor exp(z).  The two differ by a relative amount
//! eps of 1e-9 to 1e-16.  Computed in f64, the difference of two numbers that agree to 16 digits has no correct
//! digit left; computed here, it has at least 14.  Only the four operations and the exponential function of a small
//! argument are needed.
//!
//! The algorithms are the classical error-free transformations: `two_sum` (Knuth) returns the rounded sum of two f64
//! and its exact rounding error; `two_prod` returns the rounded product and its exact rounding error, by the fused
//! multiply-add `f64::mul_add`, which rounds a*b + c only once.
use std::ops::{Add, Div, Mul, Neg, Sub};
use t2core::c64::C64;

/// A real number hi + lo.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Dd {
    pub hi: f64,
    pub lo: f64,
}

/// the rounded sum s of a and b and the exact error e: a + b = s + e
#[inline]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let bb = s - a;
    let e = (a - (s - bb)) + (b - bb);
    (s, e)
}

/// the same for |a| >= |b|
#[inline]
fn quick_two_sum(a: f64, b: f64) -> (f64, f64) {
    let s = a + b;
    let e = b - (s - a);
    (s, e)
}

/// the rounded product p of a and b and the exact error e: a b = p + e
#[inline]
fn two_prod(a: f64, b: f64) -> (f64, f64) {
    let p = a * b;
    let e = a.mul_add(b, -p);
    (p, e)
}

impl Dd {
    pub const ZERO: Dd = Dd { hi: 0.0, lo: 0.0 };
    pub const ONE: Dd = Dd { hi: 1.0, lo: 0.0 };
    pub fn from_f64(x: f64) -> Dd {
        Dd { hi: x, lo: 0.0 }
    }
    pub fn to_f64(self) -> f64 {
        self.hi + self.lo
    }
    pub fn abs(self) -> Dd {
        if self.hi < 0.0 {
            -self
        } else {
            self
        }
    }
}

impl Neg for Dd {
    type Output = Dd;
    fn neg(self) -> Dd {
        Dd { hi: -self.hi, lo: -self.lo }
    }
}

impl Add for Dd {
    type Output = Dd;
    fn add(self, o: Dd) -> Dd {
        let (s, e) = two_sum(self.hi, o.hi);
        let (t, f) = two_sum(self.lo, o.lo);
        let (s, e) = quick_two_sum(s, e + t);
        let (hi, lo) = quick_two_sum(s, e + f);
        Dd { hi, lo }
    }
}

impl Sub for Dd {
    type Output = Dd;
    fn sub(self, o: Dd) -> Dd {
        self + (-o)
    }
}

impl Mul for Dd {
    type Output = Dd;
    fn mul(self, o: Dd) -> Dd {
        let (p, e) = two_prod(self.hi, o.hi);
        let e = e + (self.hi * o.lo + self.lo * o.hi);
        let (hi, lo) = quick_two_sum(p, e);
        Dd { hi, lo }
    }
}

impl Div for Dd {
    type Output = Dd;
    fn div(self, o: Dd) -> Dd {
        // three steps of long division: each quotient digit is an f64
        let q1 = self.hi / o.hi;
        let r = self - o * Dd::from_f64(q1);
        let q2 = r.hi / o.hi;
        let r = r - o * Dd::from_f64(q2);
        let q3 = r.hi / o.hi;
        let (s, e) = quick_two_sum(q1, q2);
        Dd { hi: s, lo: e } + Dd::from_f64(q3)
    }
}

/// A complex number re + i im with double-double parts.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cdd {
    pub re: Dd,
    pub im: Dd,
}

impl Cdd {
    pub const ZERO: Cdd = Cdd { re: Dd::ZERO, im: Dd::ZERO };
    pub const ONE: Cdd = Cdd { re: Dd::ONE, im: Dd::ZERO };
    pub fn from_c64(z: C64) -> Cdd {
        Cdd { re: Dd::from_f64(z.re), im: Dd::from_f64(z.im) }
    }
    pub fn from_f64(x: f64) -> Cdd {
        Cdd { re: Dd::from_f64(x), im: Dd::ZERO }
    }
    pub fn real(x: Dd) -> Cdd {
        Cdd { re: x, im: Dd::ZERO }
    }
    pub fn to_c64(self) -> C64 {
        C64 { re: self.re.to_f64(), im: self.im.to_f64() }
    }
    /// the modulus, as an f64 (used only to decide when an iteration or a series has converged)
    pub fn abs_f64(self) -> f64 {
        self.re.to_f64().hypot(self.im.to_f64())
    }
    pub fn scale(self, x: Dd) -> Cdd {
        Cdd { re: self.re * x, im: self.im * x }
    }
    /// the exponential function, by its power series sum_n z^n/n!.  For |z| <= 1 the terms decrease from the start
    /// and no digit is lost; larger arguments are halved until |z| <= 1/2 and the result is squared as often.
    pub fn exp(self) -> Cdd {
        let mut z = self;
        let mut halvings = 0;
        while z.abs_f64() > 0.5 {
            z = z.scale(Dd::from_f64(0.5));
            halvings += 1;
        }
        let mut sum = Cdd::ONE;
        let mut term = Cdd::ONE;
        for n in 1..80 {
            term = (term * z).scale(Dd::ONE / Dd::from_f64(n as f64));
            sum = sum + term;
            if term.abs_f64() < 1e-34 * sum.abs_f64() {
                break;
            }
        }
        for _ in 0..halvings {
            sum = sum * sum;
        }
        sum
    }
}

impl Neg for Cdd {
    type Output = Cdd;
    fn neg(self) -> Cdd {
        Cdd { re: -self.re, im: -self.im }
    }
}

impl Add for Cdd {
    type Output = Cdd;
    fn add(self, o: Cdd) -> Cdd {
        Cdd { re: self.re + o.re, im: self.im + o.im }
    }
}

impl Sub for Cdd {
    type Output = Cdd;
    fn sub(self, o: Cdd) -> Cdd {
        Cdd { re: self.re - o.re, im: self.im - o.im }
    }
}

impl Mul for Cdd {
    type Output = Cdd;
    fn mul(self, o: Cdd) -> Cdd {
        Cdd { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
}

impl Div for Cdd {
    type Output = Cdd;
    fn div(self, o: Cdd) -> Cdd {
        let d = o.re * o.re + o.im * o.im;
        Cdd { re: (self.re * o.re + self.im * o.im) / d, im: (self.im * o.re - self.re * o.im) / d }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use t2core::c64::c;

    /// (1 + 2^-80) - 1 = 2^-80 exactly, which f64 alone loses; 1/3 * 3 = 1 to 31 digits
    #[test]
    fn extra_digits_are_kept() {
        let tiny = Dd::from_f64(2f64.powi(-80));
        let d = (Dd::ONE + tiny) - Dd::ONE;
        assert_eq!(d.to_f64(), 2f64.powi(-80));
        let third = Dd::ONE / Dd::from_f64(3.0);
        let back = third * Dd::from_f64(3.0) - Dd::ONE;
        assert!(back.to_f64().abs() < 1e-31, "1/3 * 3 - 1 = {:e}", back.to_f64());
    }

    /// exp(z) exp(-z) = 1 to 30 digits, and exp agrees with f64 to the precision of f64
    #[test]
    fn exponential() {
        for &z in &[c(0.0, 0.11), c(0.3, -0.7), c(-1.5, 2.5), c(1e-3, 1e-4)] {
            let e = Cdd::from_c64(z).exp();
            let p = e * (-Cdd::from_c64(z)).exp() - Cdd::ONE;
            assert!(p.abs_f64() < 1e-29, "exp(z) exp(-z) - 1 = {:e} at z = {:?}", p.abs_f64(), z);
            let f = z.exp();
            assert!((e.to_c64() - f).abs() < 4e-16 * f.abs(), "exp differs from f64 at {:?}", z);
        }
    }

    /// a quotient of complex numbers, multiplied back
    #[test]
    fn division() {
        let a = Cdd::from_c64(c(0.3, -1.1));
        let b = Cdd::from_c64(c(-2.0, 0.7));
        let q = a / b;
        assert!((q * b - a).abs_f64() < 1e-30);
    }
}
