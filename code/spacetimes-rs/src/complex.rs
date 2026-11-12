// Provenance: this module is taken from the author's NCSS project (github.com/once-ere/NCSS,
// Revision_1/code/ncss-rs/src/complex.rs), where it is validated by that project's test suite.
// Changes for PROJECT spacetimes, if any, are marked "[spacetimes]".
//
//! A minimal, dependency-free double-precision complex number type.
//!
//! Only the operations needed by the FFT and the Gross-Pitaevskii solver are
//! implemented.  The layout is `#[repr(C)]` (re, im) so arrays of `C64` are
//! contiguous pairs of f64, which is cache friendly for the FFT.

use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// Complex number z = re + i*im.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct C64 {
    pub re: f64,
    pub im: f64,
}

impl C64 {
    /// Construct from real and imaginary parts.
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        C64 { re, im }
    }
    /// The additive identity.
    pub const ZERO: C64 = C64 { re: 0.0, im: 0.0 };
    /// The multiplicative identity.
    pub const ONE: C64 = C64 { re: 1.0, im: 0.0 };
    /// The imaginary unit i.
    pub const I: C64 = C64 { re: 0.0, im: 1.0 };

    /// |z|^2 (no square root, cheap).
    #[inline]
    pub fn norm_sqr(self) -> f64 {
        self.re * self.re + self.im * self.im
    }
    /// |z|.
    #[inline]
    pub fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }
    /// Argument arg(z) in (-pi, pi].
    #[inline]
    pub fn arg(self) -> f64 {
        self.im.atan2(self.re)
    }
    /// Complex conjugate.
    #[inline]
    pub fn conj(self) -> Self {
        C64::new(self.re, -self.im)
    }
    /// exp(z) = e^re (cos im + i sin im).
    #[inline]
    pub fn exp(self) -> Self {
        let r = self.re.exp();
        let (s, c) = self.im.sin_cos();
        C64::new(r * c, r * s)
    }
    /// e^{i theta} for real theta.
    #[inline]
    pub fn cis(theta: f64) -> Self {
        let (s, c) = theta.sin_cos();
        C64::new(c, s)
    }
    /// Multiply by a real scalar.
    #[inline]
    pub fn scale(self, a: f64) -> Self {
        C64::new(self.re * a, self.im * a)
    }
}

impl Add for C64 {
    type Output = C64;
    #[inline]
    fn add(self, o: C64) -> C64 {
        C64::new(self.re + o.re, self.im + o.im)
    }
}
impl Sub for C64 {
    type Output = C64;
    #[inline]
    fn sub(self, o: C64) -> C64 {
        C64::new(self.re - o.re, self.im - o.im)
    }
}
impl Mul for C64 {
    type Output = C64;
    #[inline]
    fn mul(self, o: C64) -> C64 {
        C64::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
}
impl Mul<f64> for C64 {
    type Output = C64;
    #[inline]
    fn mul(self, a: f64) -> C64 {
        C64::new(self.re * a, self.im * a)
    }
}
impl Div for C64 {
    type Output = C64;
    #[inline]
    fn div(self, o: C64) -> C64 {
        let d = o.norm_sqr();
        C64::new(
            (self.re * o.re + self.im * o.im) / d,
            (self.im * o.re - self.re * o.im) / d,
        )
    }
}
impl Neg for C64 {
    type Output = C64;
    #[inline]
    fn neg(self) -> C64 {
        C64::new(-self.re, -self.im)
    }
}
impl AddAssign for C64 {
    #[inline]
    fn add_assign(&mut self, o: C64) {
        self.re += o.re;
        self.im += o.im;
    }
}
impl SubAssign for C64 {
    #[inline]
    fn sub_assign(&mut self, o: C64) {
        self.re -= o.re;
        self.im -= o.im;
    }
}
impl MulAssign for C64 {
    #[inline]
    fn mul_assign(&mut self, o: C64) {
        *self = *self * o;
    }
}
impl MulAssign<f64> for C64 {
    #[inline]
    fn mul_assign(&mut self, a: f64) {
        self.re *= a;
        self.im *= a;
    }
}
