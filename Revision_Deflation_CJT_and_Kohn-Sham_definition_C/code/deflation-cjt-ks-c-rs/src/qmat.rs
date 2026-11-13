//! Exact matrices whose entries are complex numbers with rational real and
//! imaginary parts (`Qc` of `rat.rs`).
//!
//! The real 16x16 Dirac matrices of the two notebooks have integer entries.  On the
//! deflating spacetimes they are multiplied by the inverse scale factors 1/b_t and
//! combined with the rates of deflation; at the point x4 = 0, and for rational rates
//! and constants, every matrix that occurs has entries in Q + iQ.  All matrix
//! identities of the provenance "dirac matrices deflating" that are marked "exact"
//! are verified with this type.

use crate::rat::{Qc, Rat};
use spacetimes::imat::IMat;

/// A dense rows x cols matrix over Q + iQ, stored row by row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QMat {
    pub rows: usize,
    pub cols: usize,
    pub a: Vec<Qc>,
}

impl QMat {
    pub fn zeros(rows: usize, cols: usize) -> QMat {
        QMat { rows, cols, a: vec![Qc::ZERO; rows * cols] }
    }

    pub fn identity(n: usize) -> QMat {
        let mut m = QMat::zeros(n, n);
        for i in 0..n {
            m.a[i * n + i] = Qc::ONE;
        }
        m
    }

    /// The integer matrix `m` as an exact complex-rational matrix.
    pub fn from_imat(m: &IMat) -> QMat {
        // IMat stores its numbers of rows and columns in the fields `n` and `m`.
        let mut q = QMat::zeros(m.n, m.m);
        for i in 0..m.n {
            for j in 0..m.m {
                q.a[i * m.m + j] = Qc::int(m.get(i, j) as i128);
            }
        }
        q
    }

    pub fn get(&self, i: usize, j: usize) -> Qc {
        self.a[i * self.cols + j]
    }

    pub fn set(&mut self, i: usize, j: usize, x: Qc) {
        self.a[i * self.cols + j] = x;
    }

    pub fn mul(&self, b: &QMat) -> QMat {
        assert_eq!(self.cols, b.rows, "QMat::mul: shapes do not match");
        let mut c = QMat::zeros(self.rows, b.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let x = self.a[i * self.cols + k];
                if x.is_zero() {
                    continue;
                }
                for j in 0..b.cols {
                    let y = b.a[k * b.cols + j];
                    if !y.is_zero() {
                        c.a[i * b.cols + j] = c.a[i * b.cols + j] + x * y;
                    }
                }
            }
        }
        c
    }

    pub fn add(&self, b: &QMat) -> QMat {
        assert_eq!((self.rows, self.cols), (b.rows, b.cols));
        QMat { rows: self.rows, cols: self.cols, a: self.a.iter().zip(&b.a).map(|(x, y)| *x + *y).collect() }
    }

    pub fn sub(&self, b: &QMat) -> QMat {
        assert_eq!((self.rows, self.cols), (b.rows, b.cols));
        QMat { rows: self.rows, cols: self.cols, a: self.a.iter().zip(&b.a).map(|(x, y)| *x - *y).collect() }
    }

    /// Multiplication of every entry by the complex-rational number `c`.
    pub fn scale(&self, c: Qc) -> QMat {
        QMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| *x * c).collect() }
    }

    /// Multiplication of every entry by the rational number `r`.
    pub fn scale_rat(&self, r: Rat) -> QMat {
        self.scale(Qc::real(r))
    }

    /// Multiplication by the imaginary unit.
    pub fn times_i(&self) -> QMat {
        self.scale(Qc::I)
    }

    pub fn neg(&self) -> QMat {
        self.scale(Qc::int(-1))
    }

    pub fn transpose(&self) -> QMat {
        let mut t = QMat::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                t.a[j * self.rows + i] = self.a[i * self.cols + j];
            }
        }
        t
    }

    /// Hermitian conjugate.
    pub fn dagger(&self) -> QMat {
        let mut t = self.transpose();
        for x in t.a.iter_mut() {
            *x = x.conj();
        }
        t
    }

    pub fn trace(&self) -> Qc {
        let mut s = Qc::ZERO;
        for i in 0..self.rows.min(self.cols) {
            s = s + self.get(i, i);
        }
        s
    }

    pub fn is_zero(&self) -> bool {
        self.a.iter().all(|x| x.is_zero())
    }

    /// True if every entry is real.
    pub fn is_real(&self) -> bool {
        self.a.iter().all(|x| x.im.is_zero())
    }

    pub fn is_hermitian(&self) -> bool {
        *self == self.dagger()
    }

    pub fn is_antihermitian(&self) -> bool {
        self.add(&self.dagger()).is_zero()
    }

    /// Commutator [self, b].
    pub fn comm(&self, b: &QMat) -> QMat {
        self.mul(b).sub(&b.mul(self))
    }

    /// Anticommutator {self, b}.
    pub fn anti(&self, b: &QMat) -> QMat {
        self.mul(b).add(&b.mul(self))
    }

    /// The sub-matrix of `rows` x `cols` entries whose upper left corner is (i0, j0).
    pub fn block(&self, i0: usize, j0: usize, rows: usize, cols: usize) -> QMat {
        let mut b = QMat::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                b.a[i * cols + j] = self.get(i0 + i, j0 + j);
            }
        }
        b
    }

    /// True if self = c * identity for the complex-rational number `c`.
    pub fn is_multiple_of_identity(&self, c: Qc) -> bool {
        self.sub(&QMat::identity(self.rows).scale(c)).is_zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn products_and_conjugates() {
        // sigma_1 sigma_2 = i sigma_3, exactly
        let s1 = QMat { rows: 2, cols: 2, a: vec![Qc::ZERO, Qc::ONE, Qc::ONE, Qc::ZERO] };
        let s2 = QMat { rows: 2, cols: 2, a: vec![Qc::ZERO, -Qc::I, Qc::I, Qc::ZERO] };
        let s3 = QMat { rows: 2, cols: 2, a: vec![Qc::ONE, Qc::ZERO, Qc::ZERO, -Qc::ONE] };
        assert_eq!(s1.mul(&s2), s3.times_i());
        assert!(s2.is_hermitian() && !s2.is_real());
        assert!(s1.anti(&s2).is_zero());
        assert_eq!(s1.comm(&s2), s3.times_i().scale(Qc::int(2)));
        assert!(s1.mul(&s1).is_multiple_of_identity(Qc::ONE));
        assert_eq!(s3.trace(), Qc::ZERO);
    }

    #[test]
    fn integer_matrices_are_embedded_exactly() {
        let m = IMat::from_rows(&[vec![1, -2], vec![3, 4]]);
        let q = QMat::from_imat(&m);
        assert_eq!(q.get(1, 0), Qc::int(3));
        assert_eq!(q.scale_rat(Rat::new(1, 2)).get(0, 1), Qc::real(Rat::new(-1, 1)));
        assert_eq!(q.block(1, 0, 1, 2).a, vec![Qc::int(3), Qc::int(4)]);
    }
}
