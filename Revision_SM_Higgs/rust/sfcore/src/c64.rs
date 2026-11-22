//! Complex numbers in double precision (pure Rust).
//! (Copied unchanged in its code from the library t2core of Revision_SuperLoop/test2/rust, where it was written for
//! PROJECT spacetimes; only this note is added.)
use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// A complex number re + i im.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct C64 {
    pub re: f64,
    pub im: f64,
}

/// Shorthand constructor.
pub const fn c(re: f64, im: f64) -> C64 {
    C64 { re, im }
}

impl C64 {
    pub const ZERO: C64 = C64 { re: 0.0, im: 0.0 };
    pub const ONE: C64 = C64 { re: 1.0, im: 0.0 };
    pub const I: C64 = C64 { re: 0.0, im: 1.0 };

    pub fn conj(self) -> C64 {
        c(self.re, -self.im)
    }
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
    pub fn scale(self, a: f64) -> C64 {
        c(self.re * a, self.im * a)
    }
    /// exp(re + i im) = exp(re) (cos im + i sin im)
    pub fn exp(self) -> C64 {
        let r = self.re.exp();
        c(r * self.im.cos(), r * self.im.sin())
    }
    /// cos(theta) + i sin(theta)
    pub fn cis(theta: f64) -> C64 {
        c(theta.cos(), theta.sin())
    }
    /// the principal square root
    pub fn sqrt(self) -> C64 {
        let r = self.abs().sqrt();
        let th = 0.5 * self.arg();
        c(r * th.cos(), r * th.sin())
    }
}

impl Add for C64 {
    type Output = C64;
    fn add(self, o: C64) -> C64 {
        c(self.re + o.re, self.im + o.im)
    }
}
impl Sub for C64 {
    type Output = C64;
    fn sub(self, o: C64) -> C64 {
        c(self.re - o.re, self.im - o.im)
    }
}
impl Mul for C64 {
    type Output = C64;
    fn mul(self, o: C64) -> C64 {
        c(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
}
impl Mul<f64> for C64 {
    type Output = C64;
    fn mul(self, a: f64) -> C64 {
        c(self.re * a, self.im * a)
    }
}
impl Div for C64 {
    type Output = C64;
    fn div(self, o: C64) -> C64 {
        let d = o.norm_sqr();
        c((self.re * o.re + self.im * o.im) / d, (self.im * o.re - self.re * o.im) / d)
    }
}
impl Neg for C64 {
    type Output = C64;
    fn neg(self) -> C64 {
        c(-self.re, -self.im)
    }
}
impl AddAssign for C64 {
    fn add_assign(&mut self, o: C64) {
        self.re += o.re;
        self.im += o.im;
    }
}
impl SubAssign for C64 {
    fn sub_assign(&mut self, o: C64) {
        self.re -= o.re;
        self.im -= o.im;
    }
}
impl MulAssign for C64 {
    fn mul_assign(&mut self, o: C64) {
        *self = *self * o;
    }
}
