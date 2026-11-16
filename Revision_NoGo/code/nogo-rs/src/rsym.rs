//! Dense real square matrices and the cyclic Jacobi method for real symmetric
//! matrices.
//!
//! Where real matrices suffice.  On the 7+1 spacetime the Dirac matrices
//! gamma1671[A] are real (integer) matrices, so the Hamiltonian matrices alpha^j_NG,
//! beta_NG and every pairing matrix C^(+-) gamma^K are real; with a real pair
//! amplitude the 32x32 Bogoliubov-de Gennes matrix is real and symmetric and this
//! module diagonalises it.  (Complex Hermitian matrices, needed for complex pair
//! amplitudes and for the 4+4 spacetime, are diagonalised by spacetimes::linalg::eigh.)
//! The module is also used for the real symmetric Killing-form matrices of the
//! command `octonions`.
//!
//! The Jacobi method (Numerical Recipes, Sec. 11.1) annihilates the off-diagonal
//! entries by plane rotations until their Frobenius norm is below 1e-15 of the
//! norm of the matrix; it is backward stable and needs no external library.

use spacetimes::imat::IMat;

/// Dense n x n real matrix, row-major: a[i*n + j] = A_ij.
#[derive(Clone, Debug)]
pub struct RMat {
    pub n: usize,
    pub a: Vec<f64>,
}

#[allow(dead_code)]
impl RMat {
    /// The n x n zero matrix.
    pub fn zeros(n: usize) -> RMat {
        RMat { n, a: vec![0.0; n * n] }
    }
    /// The n x n identity matrix.
    pub fn identity(n: usize) -> RMat {
        let mut r = RMat::zeros(n);
        for i in 0..n {
            r.a[i * n + i] = 1.0;
        }
        r
    }
    /// A square integer matrix as a real matrix.
    pub fn from_imat(m: &IMat) -> RMat {
        assert_eq!(m.n, m.m, "not square");
        RMat { n: m.n, a: m.a.iter().map(|&x| x as f64).collect() }
    }
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> f64 {
        self.a[i * self.n + j]
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, x: f64) {
        self.a[i * self.n + j] = x;
    }
    /// Matrix product.
    pub fn mul(&self, b: &RMat) -> RMat {
        let n = self.n;
        assert_eq!(n, b.n);
        let mut r = RMat::zeros(n);
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
    pub fn add(&self, b: &RMat) -> RMat {
        RMat { n: self.n, a: self.a.iter().zip(&b.a).map(|(x, y)| x + y).collect() }
    }
    pub fn sub(&self, b: &RMat) -> RMat {
        RMat { n: self.n, a: self.a.iter().zip(&b.a).map(|(x, y)| x - y).collect() }
    }
    pub fn scale(&self, c: f64) -> RMat {
        RMat { n: self.n, a: self.a.iter().map(|x| c * x).collect() }
    }
    pub fn transpose(&self) -> RMat {
        let n = self.n;
        let mut r = RMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                r.a[j * n + i] = self.a[i * n + j];
            }
        }
        r
    }
    /// The 2n x 2n block matrix [[a, b], [c, d]] of four n x n blocks.
    pub fn block2(a: &RMat, b: &RMat, c: &RMat, d: &RMat) -> RMat {
        let n = a.n;
        let mut r = RMat::zeros(2 * n);
        for i in 0..n {
            for j in 0..n {
                r.a[i * 2 * n + j] = a.a[i * n + j];
                r.a[i * 2 * n + n + j] = b.a[i * n + j];
                r.a[(n + i) * 2 * n + j] = c.a[i * n + j];
                r.a[(n + i) * 2 * n + n + j] = d.a[i * n + j];
            }
        }
        r
    }
    /// Matrix times vector.
    pub fn matvec(&self, v: &[f64]) -> Vec<f64> {
        let n = self.n;
        (0..n).map(|i| (0..n).map(|j| self.a[i * n + j] * v[j]).sum()).collect()
    }
    /// Largest absolute entry.
    pub fn max_abs(&self) -> f64 {
        self.a.iter().fold(0.0f64, |m, x| m.max(x.abs()))
    }
    /// Column j as a vector.
    pub fn col(&self, j: usize) -> Vec<f64> {
        (0..self.n).map(|i| self.a[i * self.n + j]).collect()
    }
}

/// Eigenvalues (ascending) and orthonormal eigenvectors (the COLUMNS of the second
/// result) of a real symmetric matrix, by the cyclic Jacobi method.  Only the
/// upper triangle of the input is used.
pub fn eigh(m: &RMat) -> (Vec<f64>, RMat) {
    let n = m.n;
    let mut a = m.a.clone();
    for i in 0..n {
        for j in (i + 1)..n {
            a[j * n + i] = a[i * n + j];
        }
    }
    let mut v = RMat::identity(n).a;
    let norm: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-300);
    for _sweep in 0..100 {
        let mut off = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                off += a[i * n + j] * a[i * n + j];
            }
        }
        if off.sqrt() <= 1e-15 * norm {
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                let apq = a[p * n + q];
                if apq.abs() <= 1e-300 {
                    continue;
                }
                // rotation angle: theta = (a_qq - a_pp) / (2 a_pq), t = tan(phi) (the smaller root)
                let theta = (a[q * n + q] - a[p * n + p]) / (2.0 * apq);
                let t = theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt());
                let c = 1.0 / (t * t + 1.0).sqrt();
                let s = t * c;
                // A <- A J  (columns p, q), with J_pp = J_qq = c, J_pq = s, J_qp = -s
                for k in 0..n {
                    let akp = a[k * n + p];
                    let akq = a[k * n + q];
                    a[k * n + p] = c * akp - s * akq;
                    a[k * n + q] = s * akp + c * akq;
                }
                // A <- J^T A  (rows p, q)
                for k in 0..n {
                    let apk = a[p * n + k];
                    let aqk = a[q * n + k];
                    a[p * n + k] = c * apk - s * aqk;
                    a[q * n + k] = s * apk + c * aqk;
                }
                // V <- V J
                for k in 0..n {
                    let vkp = v[k * n + p];
                    let vkq = v[k * n + q];
                    v[k * n + p] = c * vkp - s * vkq;
                    v[k * n + q] = s * vkp + c * vkq;
                }
            }
        }
    }
    // sort ascending
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| a[i * n + i].partial_cmp(&a[j * n + j]).unwrap());
    let w: Vec<f64> = idx.iter().map(|&i| a[i * n + i]).collect();
    let mut vs = RMat::zeros(n);
    for (newj, &oldj) in idx.iter().enumerate() {
        for k in 0..n {
            vs.a[k * n + newj] = v[k * n + oldj];
        }
    }
    (w, vs)
}
