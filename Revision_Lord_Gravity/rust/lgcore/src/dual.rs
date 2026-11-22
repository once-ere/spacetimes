//! Forward-mode automatic differentiation to second order in the four coordinates x^0..x^3 of M31.
//!
//! A `D4` carries a complex value v, its four first partial derivatives d[p] = dv/dx^p and its second partial
//! derivatives dd[p][q] = d^2 v/dx^p dx^q.  The arithmetic follows the rules of differentiation (sum rule, product rule
//! (fg)_p = f_p g + f g_p and (fg)_pq = f_pq g + f_p g_q + f_q g_p + f g_pq, chain rule phi(f)_p = phi'(f) f_p and
//! phi(f)_pq = phi'(f) f_pq + phi''(f) f_p f_q), so a function built from the operations below carries its exact first
//! and second derivatives (exact up to the rounding of double precision): no finite differences are used.  The checks
//! of gauge covariance (program sf_part1) use it for the derivatives of gauge transformations, gauge fields and test
//! spinors; the second derivatives are needed for the field strengths of transformed gauge fields and for the
//! Yang-Mills equations.
//!
//! (Copied unchanged in its code from the library sfcore of the folder Revision_SM_Higgs of PROJECT spacetimes,
//! file rust/sfcore/src/dual.rs; only this note is added.)
use crate::c64::{c, C64};
use crate::cmat::CMat;
use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct D4 {
    pub v: C64,
    pub d: [C64; 4],
    pub dd: [[C64; 4]; 4],
}

impl D4 {
    pub const ZERO: D4 = D4 { v: C64::ZERO, d: [C64::ZERO; 4], dd: [[C64::ZERO; 4]; 4] };
    /// a constant
    pub fn cst(v: C64) -> D4 {
        D4 { v, d: [C64::ZERO; 4], dd: [[C64::ZERO; 4]; 4] }
    }
    pub fn re(x: f64) -> D4 {
        D4::cst(c(x, 0.0))
    }
    /// the coordinate x^p at the point x (value x[p], derivative 1 in the direction p)
    pub fn coord(x: &[f64; 4], p: usize) -> D4 {
        let mut d = [C64::ZERO; 4];
        d[p] = C64::ONE;
        D4 { v: c(x[p], 0.0), d, dd: [[C64::ZERO; 4]; 4] }
    }
    pub fn scale(self, s: C64) -> D4 {
        let mut r = self;
        r.v = r.v * s;
        for p in 0..4 {
            r.d[p] = r.d[p] * s;
            for q in 0..4 {
                r.dd[p][q] = r.dd[p][q] * s;
            }
        }
        r
    }
    /// the complex conjugate (the coordinates are real, so the derivatives are conjugated as well)
    pub fn conj(self) -> D4 {
        let mut r = self;
        r.v = r.v.conj();
        for p in 0..4 {
            r.d[p] = r.d[p].conj();
            for q in 0..4 {
                r.dd[p][q] = r.dd[p][q].conj();
            }
        }
        r
    }
    /// chain rule: phi(self) with phi(v) = fv, phi'(v) = fp, phi''(v) = fpp
    fn chain(self, fv: C64, fp: C64, fpp: C64) -> D4 {
        let mut r = D4::cst(fv);
        for p in 0..4 {
            r.d[p] = self.d[p] * fp;
            for q in 0..4 {
                r.dd[p][q] = self.dd[p][q] * fp + self.d[p] * self.d[q] * fpp;
            }
        }
        r
    }
    pub fn exp(self) -> D4 {
        let e = self.v.exp();
        self.chain(e, e, e)
    }
    /// the principal square root; sqrt' = 1/(2 sqrt), sqrt'' = -1/(4 z sqrt)
    pub fn sqrt(self) -> D4 {
        let s = self.v.sqrt();
        let fp = C64::ONE / (s * 2.0);
        self.chain(s, fp, -(fp / (self.v * 2.0)))
    }
    pub fn sin(self) -> D4 {
        let iz = self * D4::cst(C64::I);
        let (ep, em) = (iz.exp(), (-iz).exp());
        (ep - em) * D4::cst(c(0.0, -0.5))
    }
    pub fn cos(self) -> D4 {
        let iz = self * D4::cst(C64::I);
        let (ep, em) = (iz.exp(), (-iz).exp());
        (ep + em) * D4::cst(c(0.5, 0.0))
    }
    pub fn inv(self) -> D4 {
        let r = C64::ONE / self.v;
        self.chain(r, -(r * r), r * r * r * 2.0)
    }
    pub fn div(self, o: D4) -> D4 {
        self * o.inv()
    }
}

impl Add for D4 {
    type Output = D4;
    fn add(self, o: D4) -> D4 {
        let mut r = self;
        r.v += o.v;
        for p in 0..4 {
            r.d[p] += o.d[p];
            for q in 0..4 {
                r.dd[p][q] += o.dd[p][q];
            }
        }
        r
    }
}
impl Sub for D4 {
    type Output = D4;
    fn sub(self, o: D4) -> D4 {
        self + (-o)
    }
}
impl Neg for D4 {
    type Output = D4;
    fn neg(self) -> D4 {
        self.scale(c(-1.0, 0.0))
    }
}
impl Mul for D4 {
    type Output = D4;
    fn mul(self, o: D4) -> D4 {
        let mut r = D4::cst(self.v * o.v);
        for p in 0..4 {
            r.d[p] = self.d[p] * o.v + self.v * o.d[p];
            for q in 0..4 {
                r.dd[p][q] = self.dd[p][q] * o.v + self.d[p] * o.d[q] + self.d[q] * o.d[p] + self.v * o.dd[p][q];
            }
        }
        r
    }
}

/// A matrix of D4 entries, stored row by row.
#[derive(Clone, Debug)]
pub struct DMat {
    pub rows: usize,
    pub cols: usize,
    pub a: Vec<D4>,
}

impl DMat {
    pub fn new(rows: usize, cols: usize) -> DMat {
        DMat { rows, cols, a: vec![D4::ZERO; rows * cols] }
    }
    pub fn from_cmat(m: &CMat) -> DMat {
        DMat { rows: m.rows, cols: m.cols, a: m.a.iter().map(|&z| D4::cst(z)).collect() }
    }
    pub fn get(&self, i: usize, j: usize) -> D4 {
        self.a[i * self.cols + j]
    }
    pub fn set(&mut self, i: usize, j: usize, v: D4) {
        self.a[i * self.cols + j] = v;
    }
    pub fn add(&self, o: &DMat) -> DMat {
        DMat { rows: self.rows, cols: self.cols, a: self.a.iter().zip(&o.a).map(|(x, y)| *x + *y).collect() }
    }
    pub fn sub(&self, o: &DMat) -> DMat {
        DMat { rows: self.rows, cols: self.cols, a: self.a.iter().zip(&o.a).map(|(x, y)| *x - *y).collect() }
    }
    pub fn scale(&self, s: D4) -> DMat {
        DMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| *x * s).collect() }
    }
    pub fn mul(&self, o: &DMat) -> DMat {
        assert_eq!(self.cols, o.rows);
        let mut r = DMat::new(self.rows, o.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let x = self.get(i, k);
                for j in 0..o.cols {
                    let idx = i * o.cols + j;
                    r.a[idx] = r.a[idx] + x * o.get(k, j);
                }
            }
        }
        r
    }
    /// the conjugate transpose
    pub fn dagger(&self) -> DMat {
        let mut r = DMat::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                r.set(j, i, self.get(i, j).conj());
            }
        }
        r
    }
    /// the matrix of the values
    pub fn value(&self) -> CMat {
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| x.v).collect() }
    }
    /// the matrix of the partial derivatives d/dx^p
    pub fn deriv(&self, p: usize) -> CMat {
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| x.d[p]).collect() }
    }
    /// the matrix of the second partial derivatives d^2/dx^p dx^q
    pub fn deriv2(&self, p: usize, q: usize) -> CMat {
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| x.dd[p][q]).collect() }
    }
    /// the matrix of the D4 whose value is the derivative d/dx^p of this matrix and whose first derivatives are the
    /// second derivatives of this matrix (its second derivatives, which would need third derivatives, are set to zero:
    /// the result may be differentiated once more, not twice)
    pub fn partial(&self, p: usize) -> DMat {
        let a = self
            .a
            .iter()
            .map(|x| {
                let mut r = D4::cst(x.d[p]);
                for q in 0..4 {
                    r.d[q] = x.dd[p][q];
                }
                r
            })
            .collect();
        DMat { rows: self.rows, cols: self.cols, a }
    }
    /// the Kronecker product self (x) o (index i1 * o.rows + i2, as CMat::kron)
    pub fn kron(&self, o: &DMat) -> DMat {
        let mut r = DMat::new(self.rows * o.rows, self.cols * o.cols);
        for i1 in 0..self.rows {
            for j1 in 0..self.cols {
                for i2 in 0..o.rows {
                    for j2 in 0..o.cols {
                        r.set(i1 * o.rows + i2, j1 * o.cols + j2, self.get(i1, j1) * o.get(i2, j2));
                    }
                }
            }
        }
        r
    }
    /// the block-diagonal matrix diag(a, b)
    pub fn block_diag(a: &DMat, b: &DMat) -> DMat {
        let mut r = DMat::new(a.rows + b.rows, a.cols + b.cols);
        for i in 0..a.rows {
            for j in 0..a.cols {
                r.set(i, j, a.get(i, j));
            }
        }
        for i in 0..b.rows {
            for j in 0..b.cols {
                r.set(a.rows + i, a.cols + j, b.get(i, j));
            }
        }
        r
    }
    /// determinant and inverse of a 2x2 matrix
    pub fn det2(&self) -> D4 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }
    pub fn inv2(&self) -> DMat {
        let di = self.det2().inv();
        let mut r = DMat::new(2, 2);
        r.set(0, 0, self.get(1, 1) * di);
        r.set(0, 1, -(self.get(0, 1) * di));
        r.set(1, 0, -(self.get(1, 0) * di));
        r.set(1, 1, self.get(0, 0) * di);
        r
    }
    /// exp(X) of a traceless 2x2 matrix, cosh(s) 1 + (sinh(s)/s) X with s^2 = -det X; both coefficients are even
    /// functions of s, so the branch of the square root does not matter (s must not be close to 0)
    pub fn exp_traceless2(&self) -> DMat {
        let s = (-self.det2()).sqrt();
        let (ep, em) = (s.exp(), (-s).exp());
        let ch = (ep + em) * D4::re(0.5);
        let shs = (ep - em) * D4::re(0.5) * s.inv();
        let mut r = self.scale(shs);
        for i in 0..2 {
            let x = r.get(i, i) + ch;
            r.set(i, i, x);
        }
        r
    }
}

/// A column of D4 entries times a matrix: (m v)_i = sum_j m_ij v_j.
pub fn dmatvec(m: &DMat, v: &[D4]) -> Vec<D4> {
    (0..m.rows).map(|i| (0..m.cols).fold(D4::ZERO, |s, j| s + m.get(i, j) * v[j])).collect()
}
