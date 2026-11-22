//! Eigenvalues of a Hermitian block-tridiagonal matrix by counting and bisection (no dense diagonalization).
//!
//! The matrix T has n diagonal blocks D_0..D_{n-1} (Hermitian, b x b) and the upper off-diagonal blocks
//! B_0..B_{n-2} (T_{i,i+1} = B_i, T_{i+1,i} = B_i^dag).  For a real number sigma, the block LDL^dag factorization
//!     S_0 = D_0 - sigma 1,   S_{i+1} = D_{i+1} - sigma 1 - B_i^dag S_i^{-1} B_i
//! writes T - sigma 1 as L diag(S_0, .., S_{n-1}) L^dag with an invertible block-triangular L.  By Sylvester's law of
//! inertia the number of eigenvalues of T below sigma equals the total number of negative eigenvalues of the
//! Hermitian blocks S_i (Haynsworth's additivity of the inertia).  The count is a non-decreasing step function of
//! sigma; bisection on it locates every eigenvalue of an interval to any accuracy.  Cost: O(n b^3) per count.
//! (If an S_i is singular at the chosen sigma -- an event of probability zero -- sigma is shifted by a relative 1e-14.)
use crate::c64::C64;
use crate::cmat::{eigh, CMat};

pub struct BlockTridiag {
    pub b: usize,
    pub diag: Vec<CMat>,
    pub upper: Vec<CMat>,
}

/// inverse of a small square complex matrix (Gauss-Jordan with partial pivoting); None if singular
pub fn small_inverse(a: &CMat) -> Option<CMat> {
    let n = a.rows;
    let mut m = a.clone();
    let mut inv = CMat::identity(n);
    for col in 0..n {
        let mut p = col;
        for i in col..n {
            if m.get(i, col).abs() > m.get(p, col).abs() {
                p = i;
            }
        }
        if m.get(p, col).abs() < 1e-300 {
            return None;
        }
        for k in 0..n {
            let (x, y) = (m.get(col, k), m.get(p, k));
            m.set(col, k, y);
            m.set(p, k, x);
            let (x, y) = (inv.get(col, k), inv.get(p, k));
            inv.set(col, k, y);
            inv.set(p, k, x);
        }
        let pv = m.get(col, col);
        for k in 0..n {
            m.set(col, k, m.get(col, k) / pv);
            inv.set(col, k, inv.get(col, k) / pv);
        }
        for i in 0..n {
            if i != col {
                let fct = m.get(i, col);
                if fct.abs() > 0.0 {
                    for k in 0..n {
                        m.set(i, k, m.get(i, k) - fct * m.get(col, k));
                        inv.set(i, k, inv.get(i, k) - fct * inv.get(col, k));
                    }
                }
            }
        }
    }
    Some(inv)
}

impl BlockTridiag {
    /// the number of eigenvalues below sigma
    pub fn count_below(&self, sigma: f64) -> usize {
        let mut sg = sigma;
        for _attempt in 0..5 {
            if let Some(nneg) = self.try_count(sg) {
                return nneg;
            }
            sg += 1e-14 * (1.0 + sg.abs());
        }
        panic!("count_below: singular blocks at {}", sigma);
    }
    fn try_count(&self, sigma: f64) -> Option<usize> {
        let b = self.b;
        let shift = CMat::identity(b).scale_r(sigma);
        let mut s = self.diag[0].sub(&shift);
        let mut nneg = 0;
        for i in 0..self.diag.len() {
            if i > 0 {
                let bi = &self.upper[i - 1];
                let sinv = small_inverse(&s)?;
                s = self.diag[i].sub(&shift).sub(&bi.dagger().mul(&sinv).mul(bi));
            }
            // symmetrize against rounding and count the negative eigenvalues of the Hermitian block
            let sh = s.add(&s.dagger()).scale_r(0.5);
            let (ev, _) = eigh(&sh);
            if ev.iter().any(|x| x.abs() < 1e-300) {
                return None;
            }
            nneg += ev.iter().filter(|x| **x < 0.0).count();
        }
        Some(nneg)
    }
    /// all eigenvalues in the interval [lo, hi), with their multiplicities, located to the absolute accuracy tol
    pub fn eigenvalues_in(&self, lo: f64, hi: f64, tol: f64) -> Vec<f64> {
        let mut out = vec![];
        let (nlo, nhi) = (self.count_below(lo), self.count_below(hi));
        self.bisect(lo, hi, nlo, nhi, tol, &mut out);
        out
    }
    fn bisect(&self, lo: f64, hi: f64, nlo: usize, nhi: usize, tol: f64, out: &mut Vec<f64>) {
        if nhi == nlo {
            return;
        }
        if hi - lo < tol {
            for _ in nlo..nhi {
                out.push(0.5 * (lo + hi));
            }
            return;
        }
        let mid = 0.5 * (lo + hi);
        let nm = self.count_below(mid);
        self.bisect(lo, mid, nlo, nm, tol, out);
        self.bisect(mid, hi, nm, nhi, tol, out);
    }
    /// the dense matrix (for checks on small cases)
    pub fn dense(&self) -> CMat {
        let (b, n) = (self.b, self.diag.len());
        let mut m = CMat::zeros(b * n);
        for i in 0..n {
            m.put(b * i, b * i, &self.diag[i]);
            if i + 1 < n {
                m.put(b * i, b * (i + 1), &self.upper[i]);
                m.put(b * (i + 1), b * i, &self.upper[i].dagger());
            }
        }
        m
    }
}

/// a complex number from real and imaginary parts (re-exported for the users of this module)
pub fn cplx(re: f64, im: f64) -> C64 {
    C64 { re, im }
}
