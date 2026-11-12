//! Small dense real matrices in double precision.
//!
//! Exact integer arithmetic (module `imat`) proves the algebraic identities.
//! Group elements such as exp(theta Sigma^{AB}) contain cos, sin, cosh and sinh
//! of arbitrary angles and are therefore checked numerically here, to a
//! relative accuracy of about 1e-13.

use crate::imat::IMat;

/// An n x n real matrix stored row by row.
#[derive(Clone, Debug)]
pub struct FMat {
    pub n: usize,
    pub a: Vec<f64>,
}

impl FMat {
    pub fn zeros(n: usize) -> FMat {
        FMat { n, a: vec![0.0; n * n] }
    }
    pub fn identity(n: usize) -> FMat {
        let mut r = FMat::zeros(n);
        for i in 0..n {
            r.a[i * n + i] = 1.0;
        }
        r
    }
    /// Convert an exact integer matrix.
    pub fn from_imat(m: &IMat) -> FMat {
        assert_eq!(m.n, m.m);
        FMat { n: m.n, a: m.a.iter().map(|&x| x as f64).collect() }
    }
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.a[i * self.n + j]
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, x: f64) {
        self.a[i * self.n + j] = x;
    }
    pub fn mul(&self, b: &FMat) -> FMat {
        let n = self.n;
        let mut r = FMat::zeros(n);
        for i in 0..n {
            for k in 0..n {
                let x = self.a[i * n + k];
                if x == 0.0 {
                    continue;
                }
                for j in 0..n {
                    r.a[i * n + j] += x * b.a[k * n + j];
                }
            }
        }
        r
    }
    pub fn add(&self, b: &FMat) -> FMat {
        FMat { n: self.n, a: self.a.iter().zip(&b.a).map(|(x, y)| x + y).collect() }
    }
    pub fn sub(&self, b: &FMat) -> FMat {
        FMat { n: self.n, a: self.a.iter().zip(&b.a).map(|(x, y)| x - y).collect() }
    }
    pub fn scale(&self, c: f64) -> FMat {
        FMat { n: self.n, a: self.a.iter().map(|x| c * x).collect() }
    }
    pub fn transpose(&self) -> FMat {
        let n = self.n;
        let mut r = FMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                r.a[j * n + i] = self.a[i * n + j];
            }
        }
        r
    }
    pub fn trace(&self) -> f64 {
        (0..self.n).map(|i| self.a[i * self.n + i]).sum()
    }
    /// Largest absolute value of an entry.
    pub fn max_abs(&self) -> f64 {
        self.a.iter().fold(0.0, |m, x| m.max(x.abs()))
    }
    /// Matrix exponential by scaling and squaring: exp(M) = (exp(M / 2^s))^(2^s),
    /// with the Taylor series (to machine precision) for the scaled matrix.
    pub fn exp(&self) -> FMat {
        let norm = self.max_abs() * self.n as f64;
        let mut s = 0u32;
        while norm / (1u64 << s) as f64 > 0.25 {
            s += 1;
        }
        let x = self.scale(1.0 / (1u64 << s) as f64);
        let mut term = FMat::identity(self.n);
        let mut sum = FMat::identity(self.n);
        for k in 1..=24 {
            term = term.mul(&x).scale(1.0 / k as f64);
            sum = sum.add(&term);
        }
        for _ in 0..s {
            sum = sum.mul(&sum);
        }
        sum
    }
    /// Inverse by Gauss-Jordan elimination with partial pivoting.
    pub fn inverse(&self) -> Option<FMat> {
        let n = self.n;
        let mut a = self.a.clone();
        let mut inv = FMat::identity(n).a;
        for c in 0..n {
            let mut p = c;
            for r in (c + 1)..n {
                if a[r * n + c].abs() > a[p * n + c].abs() {
                    p = r;
                }
            }
            if a[p * n + c].abs() < 1e-300 {
                return None;
            }
            if p != c {
                for j in 0..n {
                    a.swap(c * n + j, p * n + j);
                    inv.swap(c * n + j, p * n + j);
                }
            }
            let d = a[c * n + c];
            for j in 0..n {
                a[c * n + j] /= d;
                inv[c * n + j] /= d;
            }
            for r in 0..n {
                if r != c {
                    let f = a[r * n + c];
                    if f != 0.0 {
                        for j in 0..n {
                            a[r * n + j] -= f * a[c * n + j];
                            inv[r * n + j] -= f * inv[c * n + j];
                        }
                    }
                }
            }
        }
        Some(FMat { n, a: inv })
    }
}

/// A small deterministic pseudo-random generator (xorshift64*), so that the
/// "random" test angles and momenta are the same on every machine and run.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    /// Uniform in [0, 1).
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }
    /// Uniform in [a, b).
    pub fn range(&mut self, a: f64, b: f64) -> f64 {
        a + (b - a) * self.uniform()
    }
    /// Uniform integer in [lo, hi].
    pub fn int(&mut self, lo: i64, hi: i64) -> i64 {
        lo + (self.next_u64() % ((hi - lo + 1) as u64)) as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exponential_of_a_rotation_generator() {
        // exp(theta [[0, -1], [1, 0]]) = [[cos, -sin], [sin, cos]]
        let th = 0.7f64;
        let mut g = FMat::zeros(2);
        g.set(0, 1, -th);
        g.set(1, 0, th);
        let e = g.exp();
        assert!((e.get(0, 0) - th.cos()).abs() < 1e-14 && (e.get(1, 0) - th.sin()).abs() < 1e-14);
        let inv = e.inverse().unwrap();
        assert!(e.mul(&inv).sub(&FMat::identity(2)).max_abs() < 1e-14);
    }
}
