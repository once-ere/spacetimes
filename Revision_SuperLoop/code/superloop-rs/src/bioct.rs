//! Bi-octonions, octonions, split octonions and quaternions with complex coefficients.
//!
//! Every formula of the folder Revision_SuperLoop rests on the multiplication defined here.
//!
//! * Quaternions.  A quaternion with complex coefficients is q = q0 + q1 i_q + q2 j_q + q3 k_q
//!   (array [q0, q1, q2, q3]); the units obey i_q^2 = j_q^2 = k_q^2 = i_q j_q k_q = -1.  The
//!   complex unit `i` (written `iu` in the code) commutes with every unit.  Quaternionic
//!   conjugation qbar = q0 - q1 i_q - q2 j_q - q3 k_q is complex-LINEAR.
//! * Cayley-Dickson doubling.  A bi-octonion is x = a + b l with quaternions a, b (complex
//!   coefficients) and a new unit l.  The product is
//!       (a + b l)(c + d l) = (a c + s dbar b) + (d a + b cbar) l,
//!   with s = -1 for the (bi-)octonions and s = +1 for the split octonions.  The basis is
//!       e0 = 1, e1 = i_q, e2 = j_q, e3 = k_q, e4 = l, e5 = i_q l, e6 = j_q l, e7 = k_q l
//!   (index A = 0..7); "b l" is the product of the quaternion b with l, i.e. the pair (0, b).
//! * Conjugations.  octonionic conjugate xbar = x^0 e0 - sum_a x^a e_a (complex-linear);
//!   complex conjugate x^* (coefficients conjugated); Hermitian conjugate x^dag = (xbar)^*.
//! * Norm.  N(x) = x xbar = sum_A (x^A)^2 (complex, bilinear; for s = -1).  N(xy) = N(x)N(y).
//!
//! The scalar type is generic: `GInt` (exact Gaussian integers, i128 with overflow checks) for
//! every identity that is checked exactly, and `C64` (double precision) for spectra.

use spacetimes::complex::C64;

/// Exact Gaussian integer re + i im.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct GInt {
    pub re: i128,
    pub im: i128,
}

/// The scalar field of the coefficients: exact Gaussian integers or double-precision complex.
pub trait Sc: Copy + Clone + PartialEq + std::fmt::Debug {
    fn zero() -> Self;
    fn one() -> Self;
    /// the complex unit i
    fn iu() -> Self;
    fn add(self, o: Self) -> Self;
    fn sub(self, o: Self) -> Self;
    fn mul(self, o: Self) -> Self;
    fn neg(self) -> Self {
        Self::zero().sub(self)
    }
    /// complex conjugate
    fn cconj(self) -> Self;
    fn from_ints(re: i64, im: i64) -> Self;
    /// exactly zero (GInt) or |z| < tol (C64)
    fn near_zero(&self, tol: f64) -> bool;
}

impl Sc for GInt {
    fn zero() -> Self {
        GInt { re: 0, im: 0 }
    }
    fn one() -> Self {
        GInt { re: 1, im: 0 }
    }
    fn iu() -> Self {
        GInt { re: 0, im: 1 }
    }
    fn add(self, o: Self) -> Self {
        GInt { re: self.re + o.re, im: self.im + o.im }
    }
    fn sub(self, o: Self) -> Self {
        GInt { re: self.re - o.re, im: self.im - o.im }
    }
    fn mul(self, o: Self) -> Self {
        GInt { re: self.re * o.re - self.im * o.im, im: self.re * o.im + self.im * o.re }
    }
    fn cconj(self) -> Self {
        GInt { re: self.re, im: -self.im }
    }
    fn from_ints(re: i64, im: i64) -> Self {
        GInt { re: re as i128, im: im as i128 }
    }
    fn near_zero(&self, _tol: f64) -> bool {
        self.re == 0 && self.im == 0
    }
}

impl Sc for C64 {
    fn zero() -> Self {
        C64::ZERO
    }
    fn one() -> Self {
        C64::ONE
    }
    fn iu() -> Self {
        C64::I
    }
    fn add(self, o: Self) -> Self {
        self + o
    }
    fn sub(self, o: Self) -> Self {
        self - o
    }
    fn mul(self, o: Self) -> Self {
        self * o
    }
    fn cconj(self) -> Self {
        self.conj()
    }
    fn from_ints(re: i64, im: i64) -> Self {
        C64::new(re as f64, im as f64)
    }
    fn near_zero(&self, tol: f64) -> bool {
        self.abs() < tol
    }
}

/// Quaternion with complex coefficients [q0, q1, q2, q3] = q0 + q1 i_q + q2 j_q + q3 k_q.
pub type Quat<T> = [T; 4];

/// Quaternion product.
pub fn qmul<T: Sc>(p: &Quat<T>, q: &Quat<T>) -> Quat<T> {
    let (a0, a1, a2, a3) = (p[0], p[1], p[2], p[3]);
    let (b0, b1, b2, b3) = (q[0], q[1], q[2], q[3]);
    [
        a0.mul(b0).sub(a1.mul(b1)).sub(a2.mul(b2)).sub(a3.mul(b3)),
        a0.mul(b1).add(a1.mul(b0)).add(a2.mul(b3)).sub(a3.mul(b2)),
        a0.mul(b2).sub(a1.mul(b3)).add(a2.mul(b0)).add(a3.mul(b1)),
        a0.mul(b3).add(a1.mul(b2)).sub(a2.mul(b1)).add(a3.mul(b0)),
    ]
}

/// Quaternionic conjugate (complex-linear).
pub fn qconj<T: Sc>(p: &Quat<T>) -> Quat<T> {
    [p[0], p[1].neg(), p[2].neg(), p[3].neg()]
}

/// Complex conjugate of the coefficients.
pub fn qcc<T: Sc>(p: &Quat<T>) -> Quat<T> {
    [p[0].cconj(), p[1].cconj(), p[2].cconj(), p[3].cconj()]
}

pub fn qadd<T: Sc>(p: &Quat<T>, q: &Quat<T>) -> Quat<T> {
    [p[0].add(q[0]), p[1].add(q[1]), p[2].add(q[2]), p[3].add(q[3])]
}

pub fn qsub<T: Sc>(p: &Quat<T>, q: &Quat<T>) -> Quat<T> {
    [p[0].sub(q[0]), p[1].sub(q[1]), p[2].sub(q[2]), p[3].sub(q[3])]
}

pub fn qscale<T: Sc>(p: &Quat<T>, s: T) -> Quat<T> {
    [p[0].mul(s), p[1].mul(s), p[2].mul(s), p[3].mul(s)]
}

pub fn qzero<T: Sc>() -> Quat<T> {
    [T::zero(); 4]
}

/// Complex norm of a quaternion, N(q) = q qbar = sum q_i^2 (= det of the 2x2 matrix).
pub fn qnorm<T: Sc>(p: &Quat<T>) -> T {
    p[0].mul(p[0]).add(p[1].mul(p[1])).add(p[2].mul(p[2])).add(p[3].mul(p[3]))
}

/// A bi-octonion (or split octonion) x = sum_A c[A] e_A, A = 0..7.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bi<T: Sc> {
    pub c: [T; 8],
}

impl<T: Sc> Bi<T> {
    pub fn zero() -> Self {
        Bi { c: [T::zero(); 8] }
    }
    pub fn one() -> Self {
        Self::unit(0)
    }
    /// basis element e_A
    pub fn unit(a: usize) -> Self {
        let mut c = [T::zero(); 8];
        c[a] = T::one();
        Bi { c }
    }
    /// a + b l from two quaternions
    pub fn from_quats(a: &Quat<T>, b: &Quat<T>) -> Self {
        Bi { c: [a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]] }
    }
    /// (a, b) with x = a + b l
    pub fn quats(&self) -> (Quat<T>, Quat<T>) {
        ([self.c[0], self.c[1], self.c[2], self.c[3]], [self.c[4], self.c[5], self.c[6], self.c[7]])
    }
    pub fn add(&self, o: &Self) -> Self {
        let mut c = self.c;
        for k in 0..8 {
            c[k] = c[k].add(o.c[k]);
        }
        Bi { c }
    }
    pub fn sub(&self, o: &Self) -> Self {
        let mut c = self.c;
        for k in 0..8 {
            c[k] = c[k].sub(o.c[k]);
        }
        Bi { c }
    }
    pub fn scale(&self, s: T) -> Self {
        let mut c = self.c;
        for k in 0..8 {
            c[k] = c[k].mul(s);
        }
        Bi { c }
    }
    pub fn neg(&self) -> Self {
        self.scale(T::one().neg())
    }
    /// Cayley-Dickson product with sign s (-1: bi-octonions; +1: split octonions).
    pub fn mul_s(&self, y: &Self, s: i64) -> Self {
        let (a, b) = self.quats();
        let (c, d) = y.quats();
        let dbar_b = qmul(&qconj(&d), &b);
        let first = if s < 0 { qsub(&qmul(&a, &c), &dbar_b) } else { qadd(&qmul(&a, &c), &dbar_b) };
        let second = qadd(&qmul(&d, &a), &qmul(&b, &qconj(&c)));
        Self::from_quats(&first, &second)
    }
    /// bi-octonion product
    pub fn mul(&self, y: &Self) -> Self {
        self.mul_s(y, -1)
    }
    /// octonionic conjugate xbar (complex-linear)
    pub fn conj(&self) -> Self {
        let mut c = self.c;
        for k in 1..8 {
            c[k] = c[k].neg();
        }
        Bi { c }
    }
    /// complex conjugate x^*
    pub fn cc(&self) -> Self {
        let mut c = self.c;
        for k in 0..8 {
            c[k] = c[k].cconj();
        }
        Bi { c }
    }
    /// Hermitian conjugate x^dag = (xbar)^*
    pub fn dag(&self) -> Self {
        self.conj().cc()
    }
    /// complex norm N(x) = sum_A (x^A)^2 = x xbar (bi-octonions)
    pub fn norm(&self) -> T {
        let mut s = T::zero();
        for k in 0..8 {
            s = s.add(self.c[k].mul(self.c[k]));
        }
        s
    }
    pub fn is_zero(&self, tol: f64) -> bool {
        self.c.iter().all(|z| z.near_zero(tol))
    }
    /// commutator xy - yx
    pub fn comm(&self, y: &Self) -> Self {
        self.mul(y).sub(&y.mul(self))
    }
    /// associator (xy)z - x(yz)
    pub fn assoc(&self, y: &Self, z: &Self) -> Self {
        self.mul(y).mul(z).sub(&self.mul(&y.mul(z)))
    }
}

/// Jacobiator of the commutator: [[x,y],z] + [[y,z],x] + [[z,x],y].
pub fn jacobiator<T: Sc>(x: &Bi<T>, y: &Bi<T>, z: &Bi<T>) -> Bi<T> {
    x.comm(y).comm(z).add(&y.comm(z).comm(x)).add(&z.comm(x).comm(y))
}

/// The Malcev identity, linearized form, for the commutator algebra:
///   J(x,y,[w,z]) + J(w,y,[x,z]) - [J(x,y,z), w] - [J(w,y,z), x]  (must vanish).
pub fn malcev_defect<T: Sc>(x: &Bi<T>, y: &Bi<T>, z: &Bi<T>, w: &Bi<T>) -> Bi<T> {
    jacobiator(x, y, &w.comm(z))
        .add(&jacobiator(w, y, &x.comm(z)))
        .sub(&jacobiator(x, y, z).comm(w))
        .sub(&jacobiator(w, y, z).comm(x))
}

/// A small deterministic random generator (SplitMix64); no crate is needed.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed)
    }
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// uniform integer in [-k, k]
    pub fn int(&mut self, k: i64) -> i64 {
        (self.next_u64() % (2 * k as u64 + 1)) as i64 - k
    }
    /// uniform real in [0, 1)
    pub fn unif(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// random bi-octonion with Gaussian-integer coefficients in [-k, k] + i[-k, k]
    pub fn bi<T: Sc>(&mut self, k: i64) -> Bi<T> {
        let mut c = [T::zero(); 8];
        for z in c.iter_mut() {
            *z = T::from_ints(self.int(k), self.int(k));
        }
        Bi { c }
    }
}

/// Real coordinates of a bi-octonion: [Re c0..Re c7, Im c0..Im c7] (exact).
pub fn real_coords(x: &Bi<GInt>) -> [i128; 16] {
    let mut r = [0i128; 16];
    for k in 0..8 {
        r[k] = x.c[k].re;
        r[8 + k] = x.c[k].im;
    }
    r
}

/// The bi-octonion with real coordinates r (inverse of `real_coords`).
pub fn from_real_coords(r: &[i128]) -> Bi<GInt> {
    let mut c = [GInt::zero(); 8];
    for k in 0..8 {
        c[k] = GInt { re: r[k], im: r[8 + k] };
    }
    Bi { c }
}

/// The real 16x16 integer matrix of a real-linear map F of the bi-octonions
/// (columns = images of the real basis e_A, i e_A).
pub fn real_matrix<F: Fn(&Bi<GInt>) -> Bi<GInt>>(f: F) -> Vec<Vec<i128>> {
    let mut m = vec![vec![0i128; 16]; 16];
    for col in 0..16 {
        let mut r = [0i128; 16];
        r[col] = 1;
        let img = real_coords(&f(&from_real_coords(&r)));
        for row in 0..16 {
            m[row][col] = img[row];
        }
    }
    m
}
