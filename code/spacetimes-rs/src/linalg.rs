// Provenance: this module is taken from the author's NCSS project (github.com/once-ere/NCSS,
// Revision_1/code/ncss-rs/src/linalg.rs), where it is validated by that project's test suite.
// Changes for PROJECT spacetimes, if any, are marked "[spacetimes]".
//
//! Small dense complex linear algebra needed by the three-flavor modules
//! (`flavor`, `hfb3`): n x n complex matrices stored row-major, products,
//! adjoints, and a cyclic Jacobi eigen-solver for Hermitian matrices.
//!
//! The matrices that occur are tiny (3 x 3 flavor matrices, 6 x 6
//! Bogoliubov-de Gennes matrices, 6 x 6 pair-channel matrices), so the O(n^3)
//! Jacobi method is both the simplest and - because it is backward stable and
//! computes small eigenvector components to high RELATIVE accuracy - the
//! most accurate choice.  No external crate is used.
//!
//! [spacetimes] The modules named above belong to NCSS.  In PROJECT spacetimes this
//! module serves `kernel`, `pairs` and `cjt_pairing`, whose Hermitian matrices have
//! 8 or 16 rows; the code is unchanged.

use crate::complex::C64;

/// Dense n x n complex matrix, row-major.
#[derive(Clone, Debug)]
pub struct CMat {
    /// Dimension.
    pub n: usize,
    /// Entries a[i*n + j] = A_ij.
    pub a: Vec<C64>,
}

impl CMat {
    /// n x n zero matrix.
    pub fn zeros(n: usize) -> Self {
        CMat { n, a: vec![C64::ZERO; n * n] }
    }
    /// n x n identity.
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n);
        for i in 0..n {
            m.a[i * n + i] = C64::ONE;
        }
        m
    }
    /// Real diagonal matrix.
    pub fn diag(d: &[f64]) -> Self {
        let n = d.len();
        let mut m = Self::zeros(n);
        for i in 0..n {
            m.a[i * n + i] = C64::new(d[i], 0.0);
        }
        m
    }
    /// Build from a 3 x 3 array.
    pub fn from3(x: &[[C64; 3]; 3]) -> Self {
        let mut m = Self::zeros(3);
        for i in 0..3 {
            for j in 0..3 {
                m.a[i * 3 + j] = x[i][j];
            }
        }
        m
    }
    /// Convert a 3 x 3 matrix to an array (panics if n != 3).
    pub fn to3(&self) -> [[C64; 3]; 3] {
        assert_eq!(self.n, 3);
        let mut x = [[C64::ZERO; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                x[i][j] = self.a[i * 3 + j];
            }
        }
        x
    }
    /// Element access.
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> C64 {
        self.a[i * self.n + j]
    }
    /// Element assignment.
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: C64) {
        self.a[i * self.n + j] = v;
    }
    /// Matrix product self * b.
    pub fn mul(&self, b: &CMat) -> CMat {
        let n = self.n;
        assert_eq!(n, b.n);
        let mut c = CMat::zeros(n);
        for i in 0..n {
            for k in 0..n {
                let aik = self.a[i * n + k];
                if aik.re == 0.0 && aik.im == 0.0 {
                    continue;
                }
                for j in 0..n {
                    c.a[i * n + j] += aik * b.a[k * n + j];
                }
            }
        }
        c
    }
    /// Hermitian adjoint A^dagger.
    pub fn dagger(&self) -> CMat {
        let n = self.n;
        let mut c = CMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                c.a[j * n + i] = self.a[i * n + j].conj();
            }
        }
        c
    }
    /// Transpose A^T (no conjugation).
    pub fn transpose(&self) -> CMat {
        let n = self.n;
        let mut c = CMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                c.a[j * n + i] = self.a[i * n + j];
            }
        }
        c
    }
    /// Element-wise complex conjugate A^*.
    pub fn conj(&self) -> CMat {
        CMat { n: self.n, a: self.a.iter().map(|z| z.conj()).collect() }
    }
    /// Scale by a real number.
    pub fn scale(&self, s: f64) -> CMat {
        CMat { n: self.n, a: self.a.iter().map(|z| *z * s).collect() }
    }
    /// Sum self + b.
    pub fn add(&self, b: &CMat) -> CMat {
        CMat { n: self.n, a: self.a.iter().zip(b.a.iter()).map(|(x, y)| *x + *y).collect() }
    }
    /// Difference self - b.
    pub fn sub(&self, b: &CMat) -> CMat {
        CMat { n: self.n, a: self.a.iter().zip(b.a.iter()).map(|(x, y)| *x - *y).collect() }
    }
    /// Frobenius norm sqrt(sum |A_ij|^2).
    pub fn fro(&self) -> f64 {
        self.a.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
    }
    /// Frobenius norm of the off-diagonal part.
    pub fn fro_offdiag(&self) -> f64 {
        let n = self.n;
        let mut s = 0.0;
        for i in 0..n {
            for j in 0..n {
                if i != j {
                    s += self.a[i * n + j].norm_sqr();
                }
            }
        }
        s.sqrt()
    }
    /// Trace.
    pub fn trace(&self) -> C64 {
        let mut t = C64::ZERO;
        for i in 0..self.n {
            t += self.a[i * self.n + i];
        }
        t
    }
}

/// Eigen-decomposition of a Hermitian matrix by the cyclic Jacobi method.
///
/// For every pivot (p, q), p < q, with a_pq = |a_pq| e^{i phi} != 0 the
/// unitary plane rotation
///     P_pp = P_qq = c,  P_pq = s e^{i phi},  P_qp = - s e^{-i phi},
///     cot(2 theta) = (a_qq - a_pp) / (2 |a_pq|),  t = tan(theta) (smaller root),
/// annihilates a_pq in A <- P^dagger A P.  Sweeps are repeated until the
/// off-diagonal Frobenius norm is below 1e-15 of the full norm (quadratic
/// convergence: 5-8 sweeps for n <= 9).
///
/// Returns (eigenvalues in ASCENDING order, V) with the eigenvectors in the
/// COLUMNS of V, i.e. A V = V diag(w), V^dagger V = 1.  Only the Hermitian
/// part of the input is used (the strict lower triangle is ignored and
/// replaced by the conjugate of the upper triangle).
pub fn eigh(m: &CMat) -> (Vec<f64>, CMat) {
    let n = m.n;
    let mut a = m.clone();
    // enforce exact Hermiticity
    for i in 0..n {
        a.a[i * n + i] = C64::new(a.a[i * n + i].re, 0.0);
        for j in (i + 1)..n {
            a.a[j * n + i] = a.a[i * n + j].conj();
        }
    }
    let mut v = CMat::identity(n);
    let total = a.fro().max(1e-300);
    for _sweep in 0..60 {
        if a.fro_offdiag() <= 1e-15 * total {
            break;
        }
        for p in 0..n {
            for q in (p + 1)..n {
                let apq = a.a[p * n + q];
                let r = apq.abs();
                if r <= 1e-300 {
                    continue;
                }
                let app = a.a[p * n + p].re;
                let aqq = a.a[q * n + q].re;
                // skip rotations that cannot change the diagonal in f64
                if r <= 1e-18 * (app.abs() + aqq.abs()) && r <= 1e-17 * total {
                    a.a[p * n + q] = C64::ZERO;
                    a.a[q * n + p] = C64::ZERO;
                    continue;
                }
                let tau = (aqq - app) / (2.0 * r);
                let t = if tau >= 0.0 { 1.0 / (tau + (1.0 + tau * tau).sqrt()) } else { -1.0 / (-tau + (1.0 + tau * tau).sqrt()) };
                let c = 1.0 / (1.0 + t * t).sqrt();
                let s = t * c;
                let ph = C64::new(apq.re / r, apq.im / r); // e^{i phi}
                let sp = ph * s; //  s e^{ i phi}
                let sm = ph.conj() * s; //  s e^{-i phi}
                // columns: A <- A P
                for k in 0..n {
                    let akp = a.a[k * n + p];
                    let akq = a.a[k * n + q];
                    a.a[k * n + p] = akp * c - sm * akq;
                    a.a[k * n + q] = sp * akp + akq * c;
                }
                // rows: A <- P^dagger A
                for k in 0..n {
                    let apk = a.a[p * n + k];
                    let aqk = a.a[q * n + k];
                    a.a[p * n + k] = apk * c - sp * aqk;
                    a.a[q * n + k] = sm * apk + aqk * c;
                }
                // the pivot is zero by construction; remove rounding residue
                a.a[p * n + q] = C64::ZERO;
                a.a[q * n + p] = C64::ZERO;
                a.a[p * n + p] = C64::new(a.a[p * n + p].re, 0.0);
                a.a[q * n + q] = C64::new(a.a[q * n + q].re, 0.0);
                // eigenvectors: V <- V P
                for k in 0..n {
                    let vkp = v.a[k * n + p];
                    let vkq = v.a[k * n + q];
                    v.a[k * n + p] = vkp * c - sm * vkq;
                    v.a[k * n + q] = sp * vkp + vkq * c;
                }
            }
        }
    }
    // sort ascending
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&i, &j| a.a[i * n + i].re.partial_cmp(&a.a[j * n + j].re).unwrap());
    let w: Vec<f64> = idx.iter().map(|&i| a.a[i * n + i].re).collect();
    let mut vs = CMat::zeros(n);
    for (jn, &jo) in idx.iter().enumerate() {
        for k in 0..n {
            vs.a[k * n + jn] = v.a[k * n + jo];
        }
    }
    (w, vs)
}

/// Singular values (descending) of a complex n x n matrix, computed as the
/// square roots of the eigenvalues of A^dagger A.  For a complex SYMMETRIC
/// matrix (a pair field Delta = Delta^T) these are the Takagi values d_a in
/// Delta = V diag(d) V^T.
pub fn singular_values(m: &CMat) -> Vec<f64> {
    let h = m.dagger().mul(m);
    let (w, _) = eigh(&h);
    let mut s: Vec<f64> = w.iter().map(|x| x.max(0.0).sqrt()).collect();
    s.reverse();
    s
}

/// Solve the real n x n linear system A x = b by Gaussian elimination with
/// partial pivoting (used by the Anderson accelerator).  Returns None if the
/// matrix is numerically singular.
pub fn solve_real(a: &[f64], b: &[f64], n: usize) -> Option<Vec<f64>> {
    let mut m = a.to_vec();
    let mut x = b.to_vec();
    for col in 0..n {
        let mut piv = col;
        for r in (col + 1)..n {
            if m[r * n + col].abs() > m[piv * n + col].abs() {
                piv = r;
            }
        }
        if m[piv * n + col].abs() < 1e-300 {
            return None;
        }
        if piv != col {
            for c in 0..n {
                m.swap(col * n + c, piv * n + c);
            }
            x.swap(col, piv);
        }
        let d = m[col * n + col];
        for r in (col + 1)..n {
            let f = m[r * n + col] / d;
            if f == 0.0 {
                continue;
            }
            for c in col..n {
                m[r * n + c] -= f * m[col * n + c];
            }
            x[r] -= f * x[col];
        }
    }
    for col in (0..n).rev() {
        let mut s = x[col];
        for c in (col + 1)..n {
            s -= m[col * n + c] * x[c];
        }
        x[col] = s / m[col * n + col];
    }
    Some(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic pseudo-random Hermitian test matrix.
    fn test_matrix(n: usize, seed: u64) -> CMat {
        // [spacetimes] the NCSS test used crate::gpe::Rng (normal deviates); the gpe module is
        // not part of this crate, so uniform deviates of crate::fmat::Rng are used instead.
        let mut r = crate::fmat::Rng(seed.wrapping_mul(0x9E3779B97F4A7C15) | 1);
        let mut m = CMat::zeros(n);
        for i in 0..n {
            m.set(i, i, C64::new(r.range(-1.0, 1.0), 0.0));
            for j in (i + 1)..n {
                let z = C64::new(r.range(-1.0, 1.0), r.range(-1.0, 1.0));
                m.set(i, j, z);
                m.set(j, i, z.conj());
            }
        }
        m
    }

    #[test]
    fn jacobi_diagonalizes_hermitian_matrices() {
        for &n in &[2usize, 3, 6, 9] {
            let m = test_matrix(n, 1234 + n as u64);
            let (w, v) = eigh(&m);
            // A V = V W
            let av = m.mul(&v);
            let vw = v.mul(&CMat::diag(&w));
            assert!(av.sub(&vw).fro() < 1e-12 * m.fro(), "n={n}: residual {}", av.sub(&vw).fro());
            // V unitary
            let vv = v.dagger().mul(&v);
            assert!(vv.sub(&CMat::identity(n)).fro() < 1e-13, "n={n}: unitarity");
            // ascending and trace preserved
            for k in 1..n {
                assert!(w[k] >= w[k - 1]);
            }
            let tr: f64 = w.iter().sum();
            assert!((tr - m.trace().re).abs() < 1e-12);
        }
    }

    #[test]
    fn bdg_two_by_two_matches_closed_form() {
        // H = [[xi, D],[D*, -xi]] has eigenvalues +-sqrt(xi^2+|D|^2) and the
        // positive-energy projector has (1,2) element D/(2E).
        let (xi, d) = (0.37, C64::new(0.21, -0.4));
        let mut h = CMat::zeros(2);
        h.set(0, 0, C64::new(xi, 0.0));
        h.set(1, 1, C64::new(-xi, 0.0));
        h.set(0, 1, d);
        h.set(1, 0, d.conj());
        let (w, v) = eigh(&h);
        let e = (xi * xi + d.norm_sqr()).sqrt();
        assert!((w[1] - e).abs() < 1e-15 && (w[0] + e).abs() < 1e-15);
        let k = v.get(0, 1) * v.get(1, 1).conj();
        assert!((k - d * (0.5 / e)).abs() < 1e-15);
    }

    #[test]
    fn linear_solver() {
        let a = [4.0, 1.0, 2.0, 1.0, 3.0, 0.0, 2.0, 0.0, 5.0];
        let x0 = [1.0, -2.0, 0.5];
        let b: Vec<f64> = (0..3).map(|i| (0..3).map(|j| a[i * 3 + j] * x0[j]).sum()).collect();
        let x = solve_real(&a, &b, 3).unwrap();
        for i in 0..3 {
            assert!((x[i] - x0[i]).abs() < 1e-13);
        }
    }
}
