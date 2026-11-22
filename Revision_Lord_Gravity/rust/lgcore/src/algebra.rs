//! The bi-quaternions, the bi-octonions, the Lorentz super-loop and the manifold M31.
//!
//! Definitions (the same as in the manuscript superfluid.md, Section 2):
//! * A bi-quaternion is b = sum_{p=0}^{3} xi_p e_p with complex coefficients xi_p (array [xi_0, xi_1, xi_2, xi_3]);
//!   e_0 = 1, e_j^2 = -1 (j = 1, 2, 3), e_1 e_2 = -e_2 e_1 = e_3 and cyclically.  The imaginary unit i of the
//!   coefficients commutes with every unit.
//! * A bi-octonion is x = a + b e_4 with bi-quaternions a and b (Cayley-Dickson form); its eight coefficients are
//!   those of e_0..e_3 (from a) and of e_4, e_5 = e_1 e_4, e_6 = e_2 e_4, e_7 = e_3 e_4 (from b).  The product is
//!       (a + b e_4)(c + d e_4) = (a c - dbar b) + (d a + b cbar) e_4 .
//! * Conjugations: xbar changes the signs of the coefficients of e_1..e_7 (complex-linear); x^* conjugates every
//!   coefficient; x^dag = (xbar)^*.
//! * Complex norm: N(x) = sum_p xi_p(x)^2, with x xbar = N(x) e_0 and N(x y) = N(x) N(y).
//! * Matrix of a bi-quaternion: M(e_0) = sigma_0, M(e_j) = -i sigma_j, extended complex-linearly; M is multiplicative,
//!   det M(b) = N(b) and M(b^dag) = M(b)^dag.
//! * M31: the Hermitian bi-quaternions x = x^0 e_0 + i (x^1 e_1 + x^2 e_2 + x^3 e_3); M(x) = sum_p x^p sigma_p and
//!   N(x) = (x^0)^2 - (x^1)^2 - (x^2)^2 - (x^3)^2 is the Minkowski interval.
//! * A unit bi-quaternion g (N(g) = 1) acts on M31 by x -> g x g^dag; the real 4x4 matrix of this map is
//!   L(g)_{p p'} = (1/2) Re tr(sigma_p M(g) sigma_p' M(g)^dag).
//!
//! (Copied unchanged in its code from the library sfcore of the folder Revision_SM_Higgs of PROJECT spacetimes,
//! file rust/sfcore/src/algebra.rs; only this note is added.)
use crate::c64::{c, C64};
use crate::cmat::CMat;
use crate::dirac::sigma;
use crate::rng::Rng;

/// A bi-quaternion: the complex coefficients of e_0, e_1, e_2, e_3.
pub type BiQ = [C64; 4];

/// the bi-quaternion unit e_p (p = 0..3)
pub fn q_unit(p: usize) -> BiQ {
    let mut q = [C64::ZERO; 4];
    q[p] = C64::ONE;
    q
}

/// the product of two bi-quaternions (complex-bilinear; the quaternion rules for the units)
pub fn q_mul(x: &BiQ, y: &BiQ) -> BiQ {
    let (a0, a1, a2, a3) = (x[0], x[1], x[2], x[3]);
    let (b0, b1, b2, b3) = (y[0], y[1], y[2], y[3]);
    [
        a0 * b0 - a1 * b1 - a2 * b2 - a3 * b3,
        a0 * b1 + a1 * b0 + a2 * b3 - a3 * b2,
        a0 * b2 - a1 * b3 + a2 * b0 + a3 * b1,
        a0 * b3 + a1 * b2 - a2 * b1 + a3 * b0,
    ]
}
pub fn q_add(x: &BiQ, y: &BiQ) -> BiQ {
    [x[0] + y[0], x[1] + y[1], x[2] + y[2], x[3] + y[3]]
}
pub fn q_sub(x: &BiQ, y: &BiQ) -> BiQ {
    [x[0] - y[0], x[1] - y[1], x[2] - y[2], x[3] - y[3]]
}
pub fn q_scale(x: &BiQ, s: C64) -> BiQ {
    [x[0] * s, x[1] * s, x[2] * s, x[3] * s]
}
/// conjugation bbar: the signs of the coefficients of e_1, e_2, e_3 change (complex-linear)
pub fn q_bar(x: &BiQ) -> BiQ {
    [x[0], -x[1], -x[2], -x[3]]
}
/// complex conjugation b^*: every coefficient conjugated
pub fn q_cc(x: &BiQ) -> BiQ {
    [x[0].conj(), x[1].conj(), x[2].conj(), x[3].conj()]
}
/// Hermitian conjugation b^dag = (bbar)^*
pub fn q_dag(x: &BiQ) -> BiQ {
    q_cc(&q_bar(x))
}
/// the complex norm N(b) = sum_p xi_p^2
pub fn q_norm(x: &BiQ) -> C64 {
    x[0] * x[0] + x[1] * x[1] + x[2] * x[2] + x[3] * x[3]
}
/// the polarized norm N(b, b') = sum_p xi_p(b) xi_p(b')
pub fn q_norm2(x: &BiQ, y: &BiQ) -> C64 {
    x[0] * y[0] + x[1] * y[1] + x[2] * y[2] + x[3] * y[3]
}
/// the largest absolute value of a coefficient
pub fn q_max_abs(x: &BiQ) -> f64 {
    x.iter().fold(0.0f64, |m, z| m.max(z.abs()))
}

/// The 2x2 matrix M(b) = xi_0 sigma_0 - i sum_j xi_j sigma_j of a bi-quaternion.
pub fn mll(b: &BiQ) -> CMat {
    let mi = c(0.0, -1.0);
    sigma(0).scale(b[0]).add(&sigma(1).scale(mi * b[1])).add(&sigma(2).scale(mi * b[2])).add(&sigma(3).scale(mi * b[3]))
}
/// The bi-quaternion of a 2x2 matrix (the inverse of mll): xi_0 = tr(m)/2, xi_j = (i/2) tr(sigma_j m).
pub fn biq_of(m: &CMat) -> BiQ {
    let half = 0.5;
    let x0 = m.trace().scale(half);
    let mut q = [x0, C64::ZERO, C64::ZERO, C64::ZERO];
    for j in 1..4 {
        q[j] = c(0.0, half) * sigma(j).mul(m).trace();
    }
    q
}

/// A bi-octonion x = a + b e_4.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BiO {
    pub a: BiQ,
    pub b: BiQ,
}

impl BiO {
    pub const ZERO: BiO = BiO { a: [C64::ZERO; 4], b: [C64::ZERO; 4] };
    /// the unit e_p (p = 0..7): e_0..e_3 in a; e_4 = (0, e_0), e_{4+j} = e_j e_4 = (0, e_j)
    pub fn unit(p: usize) -> BiO {
        let mut x = BiO::ZERO;
        if p < 4 {
            x.a[p] = C64::ONE;
        } else {
            x.b[p - 4] = C64::ONE;
        }
        x
    }
    /// from the eight coefficients (of e_0..e_7)
    pub fn from_coeffs(cf: &[C64; 8]) -> BiO {
        BiO { a: [cf[0], cf[1], cf[2], cf[3]], b: [cf[4], cf[5], cf[6], cf[7]] }
    }
    /// the eight coefficients
    pub fn coeffs(&self) -> [C64; 8] {
        [self.a[0], self.a[1], self.a[2], self.a[3], self.b[0], self.b[1], self.b[2], self.b[3]]
    }
    /// the Cayley-Dickson product (a + b e4)(c + d e4) = (a c - dbar b) + (d a + b cbar) e4
    pub fn mul(&self, y: &BiO) -> BiO {
        let first = q_sub(&q_mul(&self.a, &y.a), &q_mul(&q_bar(&y.b), &self.b));
        let second = q_add(&q_mul(&y.b, &self.a), &q_mul(&self.b, &q_bar(&y.a)));
        BiO { a: first, b: second }
    }
    pub fn add(&self, y: &BiO) -> BiO {
        BiO { a: q_add(&self.a, &y.a), b: q_add(&self.b, &y.b) }
    }
    pub fn sub(&self, y: &BiO) -> BiO {
        BiO { a: q_sub(&self.a, &y.a), b: q_sub(&self.b, &y.b) }
    }
    pub fn scale(&self, s: C64) -> BiO {
        BiO { a: q_scale(&self.a, s), b: q_scale(&self.b, s) }
    }
    /// xbar: (a + b e4)bar = abar - b e4 (the coefficients of e_1..e_7 change sign)
    pub fn bar(&self) -> BiO {
        BiO { a: q_bar(&self.a), b: q_scale(&self.b, c(-1.0, 0.0)) }
    }
    /// x^*: every coefficient conjugated
    pub fn cc(&self) -> BiO {
        BiO { a: q_cc(&self.a), b: q_cc(&self.b) }
    }
    /// x^dag = (xbar)^*
    pub fn dag(&self) -> BiO {
        self.bar().cc()
    }
    /// the complex norm N(x) = sum of the squares of the eight coefficients
    pub fn norm(&self) -> C64 {
        q_norm(&self.a) + q_norm(&self.b)
    }
    /// the associator (x y) z - x (y z)
    pub fn assoc(&self, y: &BiO, z: &BiO) -> BiO {
        self.mul(y).mul(z).sub(&self.mul(&y.mul(z)))
    }
    /// the largest absolute value of a coefficient
    pub fn max_abs(&self) -> f64 {
        q_max_abs(&self.a).max(q_max_abs(&self.b))
    }
}

/// A bi-octonion with small random integer coefficients, real and imaginary parts in -k..k: its products are computed
/// without rounding in double precision, so identities are checked exactly.
pub fn random_int_bio(rng: &mut Rng, k: i64) -> BiO {
    let mut cf = [C64::ZERO; 8];
    for z in cf.iter_mut() {
        let re = ((rng.next_u64() % (2 * k as u64 + 1)) as i64 - k) as f64;
        let im = ((rng.next_u64() % (2 * k as u64 + 1)) as i64 - k) as f64;
        *z = c(re, im);
    }
    BiO::from_coeffs(&cf)
}

/// The point of M31 with the coordinates (x^0, x^1, x^2, x^3): the bi-quaternion x^0 e_0 + i sum_j x^j e_j.
pub fn point(x: &[f64; 4]) -> BiQ {
    [c(x[0], 0.0), c(0.0, x[1]), c(0.0, x[2]), c(0.0, x[3])]
}
/// The coordinates of a Hermitian bi-quaternion, x^0 = Re xi_0 and x^j = Im xi_j (and the largest deviation from
/// Hermiticity, |Im xi_0| and |Re xi_j|).
pub fn coords(x: &BiQ) -> ([f64; 4], f64) {
    let dev = x[0].im.abs().max(x[1].re.abs()).max(x[2].re.abs()).max(x[3].re.abs());
    ([x[0].re, x[1].im, x[2].im, x[3].im], dev)
}

/// The real 4x4 matrix of the Lorentz transformation x -> g x g^dag of M31, from the matrix mg = M(g):
/// L_{p p'} = (1/2) Re tr(sigma_p M sigma_p' M^dag).
pub fn lorentz_matrix(mg: &CMat) -> [[f64; 4]; 4] {
    let md = mg.dagger();
    let mut l = [[0.0; 4]; 4];
    for p in 0..4 {
        for q in 0..4 {
            l[p][q] = 0.5 * sigma(p).mul(mg).mul(&sigma(q)).mul(&md).trace().re;
        }
    }
    l
}

/// exp(X) of a traceless complex 2x2 matrix in closed form: X^2 = s^2 sigma_0 with s^2 = -det X, so
/// exp(X) = cosh(s) sigma_0 + (sinh(s)/s) X (the series sinh(s)/s = 1 + s^2/6 + ... for small s).
pub fn exp_traceless2(x: &CMat) -> CMat {
    let s2 = -x.det2();
    let s = s2.sqrt();
    let (ch, shs) = if s.abs() < 1e-6 {
        (C64::ONE + s2 * 0.5 + s2 * s2 * (1.0 / 24.0), C64::ONE + s2 * (1.0 / 6.0) + s2 * s2 * (1.0 / 120.0))
    } else {
        let e = s.exp();
        let ei = (-s).exp();
        ((e + ei) * 0.5, (e - ei) * 0.5 / s)
    };
    CMat::identity(2).scale(ch).add(&x.scale(shs))
}

/// The matrix M(u) of the Lorentz generator u = sum_j (th_rot_j J^j + th_boost_j K^j), J^j = e_j/2, K^j = i e_j/2:
/// M(u) = sum_j (-i th_rot_j + th_boost_j) sigma_j / 2.
pub fn lorentz_generator(th_rot: &[f64; 3], th_boost: &[f64; 3]) -> CMat {
    let mut m = CMat::zeros(2);
    for j in 0..3 {
        m = m.add(&sigma(j + 1).scale(c(0.5 * th_boost[j], -0.5 * th_rot[j])));
    }
    m
}

/// A pseudo-random unit bi-quaternion, as its matrix M(g) = exp(M(u)) with a random Lorentz generator u (rotation
/// angles and rapidities uniform in -amp..amp).
pub fn random_unit_biq(rng: &mut Rng, amp: f64) -> CMat {
    let r = [amp * rng.unit(), amp * rng.unit(), amp * rng.unit()];
    let b = [amp * rng.unit(), amp * rng.unit(), amp * rng.unit()];
    exp_traceless2(&lorentz_generator(&r, &b))
}

/// A pseudo-random SU(2) matrix exp(i a.tau/2) with a uniform in -amp..amp (an isospin rotation).
pub fn random_su2(rng: &mut Rng, amp: f64) -> CMat {
    let a = [amp * rng.unit(), amp * rng.unit(), amp * rng.unit()];
    let mut x = CMat::zeros(2);
    for j in 0..3 {
        x = x.add(&sigma(j + 1).scale(c(0.0, 0.5 * a[j])));
    }
    exp_traceless2(&x)
}

/// The real orthogonal 3x3 matrix R(U) of an SU(2) matrix U: U tau_k U^dag = sum_k' R_{k'k} tau_k'
/// (R_{k'k} = (1/2) tr(tau_k' U tau_k U^dag)).
pub fn rotation_of_su2(u: &CMat) -> [[f64; 3]; 3] {
    let ud = u.dagger();
    let mut r = [[0.0; 3]; 3];
    for kp in 0..3 {
        for k in 0..3 {
            r[kp][k] = 0.5 * sigma(kp + 1).mul(u).mul(&sigma(k + 1)).mul(&ud).trace().re;
        }
    }
    r
}
