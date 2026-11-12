//! Exact matrix arithmetic over the integers and over the Gaussian integers.
//!
//! Every statement about the Dirac matrices of PROJECT spacetimes (reality,
//! anticommutation relations, commutation relations of the generators,
//! projection operators) is an identity between matrices whose entries are
//! small integers.  Such identities are checked here in exact `i64` arithmetic:
//! a check that passes is a proof, not a numerical coincidence.  The release
//! profile keeps overflow checks switched on, so an overflow would abort the
//! program instead of producing a wrong "pass".

/// A dense integer matrix with `n` rows and `m` columns, stored row by row.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct IMat {
    pub n: usize,
    pub m: usize,
    pub a: Vec<i64>,
}

impl IMat {
    /// The `n` x `m` zero matrix.
    pub fn zeros(n: usize, m: usize) -> IMat {
        IMat { n, m, a: vec![0; n * m] }
    }
    /// The `n` x `n` identity matrix.
    pub fn identity(n: usize) -> IMat {
        let mut r = IMat::zeros(n, n);
        for i in 0..n {
            r.a[i * n + i] = 1;
        }
        r
    }
    /// Build a matrix from a slice of rows.
    pub fn from_rows(rows: &[Vec<i64>]) -> IMat {
        let n = rows.len();
        let m = if n > 0 { rows[0].len() } else { 0 };
        let mut r = IMat::zeros(n, m);
        for (i, row) in rows.iter().enumerate() {
            assert_eq!(row.len(), m, "ragged matrix");
            for (j, &x) in row.iter().enumerate() {
                r.a[i * m + j] = x;
            }
        }
        r
    }
    /// Diagonal matrix with the given diagonal.
    pub fn diag(d: &[i64]) -> IMat {
        let n = d.len();
        let mut r = IMat::zeros(n, n);
        for i in 0..n {
            r.a[i * n + i] = d[i];
        }
        r
    }
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> i64 {
        self.a[i * self.m + j]
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, x: i64) {
        self.a[i * self.m + j] = x;
    }
    /// Matrix product `self . b`.
    pub fn mul(&self, b: &IMat) -> IMat {
        assert_eq!(self.m, b.n, "shape mismatch in product");
        let mut r = IMat::zeros(self.n, b.m);
        for i in 0..self.n {
            for k in 0..self.m {
                let x = self.a[i * self.m + k];
                if x == 0 {
                    continue;
                }
                for j in 0..b.m {
                    r.a[i * b.m + j] += x * b.a[k * b.m + j];
                }
            }
        }
        r
    }
    pub fn add(&self, b: &IMat) -> IMat {
        assert!(self.n == b.n && self.m == b.m);
        IMat { n: self.n, m: self.m, a: self.a.iter().zip(&b.a).map(|(x, y)| x + y).collect() }
    }
    pub fn sub(&self, b: &IMat) -> IMat {
        assert!(self.n == b.n && self.m == b.m);
        IMat { n: self.n, m: self.m, a: self.a.iter().zip(&b.a).map(|(x, y)| x - y).collect() }
    }
    /// The matrix multiplied by the integer `c`.
    pub fn scale(&self, c: i64) -> IMat {
        IMat { n: self.n, m: self.m, a: self.a.iter().map(|x| c * x).collect() }
    }
    pub fn neg(&self) -> IMat {
        self.scale(-1)
    }
    pub fn transpose(&self) -> IMat {
        let mut r = IMat::zeros(self.m, self.n);
        for i in 0..self.n {
            for j in 0..self.m {
                r.a[j * self.n + i] = self.a[i * self.m + j];
            }
        }
        r
    }
    pub fn trace(&self) -> i64 {
        assert_eq!(self.n, self.m);
        (0..self.n).map(|i| self.a[i * self.m + i]).sum()
    }
    pub fn is_zero(&self) -> bool {
        self.a.iter().all(|&x| x == 0)
    }
    pub fn is_symmetric(&self) -> bool {
        *self == self.transpose()
    }
    pub fn is_antisymmetric(&self) -> bool {
        *self == self.transpose().neg()
    }
    /// True if every entry is -1, 0 or +1.
    pub fn entries_in_unit_set(&self) -> bool {
        self.a.iter().all(|&x| (-1..=1).contains(&x))
    }
    /// The block matrix [[a, b], [c, d]] built from four blocks of equal shape.
    pub fn block2(a: &IMat, b: &IMat, c: &IMat, d: &IMat) -> IMat {
        let (n, m) = (a.n, a.m);
        assert!(b.n == n && b.m == m && c.n == n && c.m == m && d.n == n && d.m == m);
        let mut r = IMat::zeros(2 * n, 2 * m);
        for i in 0..n {
            for j in 0..m {
                r.set(i, j, a.get(i, j));
                r.set(i, j + m, b.get(i, j));
                r.set(i + n, j, c.get(i, j));
                r.set(i + n, j + m, d.get(i, j));
            }
        }
        r
    }
    /// The sub-block of `rows` x `cols` entries whose upper left corner is (i0, j0).
    pub fn block(&self, i0: usize, j0: usize, rows: usize, cols: usize) -> IMat {
        let mut r = IMat::zeros(rows, cols);
        for i in 0..rows {
            for j in 0..cols {
                r.set(i, j, self.get(i0 + i, j0 + j));
            }
        }
        r
    }
    /// Commutator [self, b] = self.b - b.self.
    pub fn comm(&self, b: &IMat) -> IMat {
        self.mul(b).sub(&b.mul(self))
    }
    /// Anticommutator {self, b} = self.b + b.self.
    pub fn anti(&self, b: &IMat) -> IMat {
        self.mul(b).add(&b.mul(self))
    }
    /// The matrix as one row vector of length n*m (used for rank computations).
    pub fn flatten(&self) -> Vec<i64> {
        self.a.clone()
    }
    /// Product of a list of square matrices, in the order given.
    pub fn product(list: &[&IMat]) -> IMat {
        let mut r = IMat::identity(list[0].n);
        for x in list {
            r = r.mul(x);
        }
        r
    }
}

/// A matrix over the Gaussian integers Z[i]: `re + i im`.
///
/// Needed for the 7+1 spacetime only, where the chirality matrix is `i` times a
/// real matrix.  Twice a chiral projector is a Gaussian-integer matrix.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GMat {
    pub re: IMat,
    pub im: IMat,
}

impl GMat {
    pub fn from_real(re: &IMat) -> GMat {
        GMat { re: re.clone(), im: IMat::zeros(re.n, re.m) }
    }
    /// `i` times a real matrix.
    pub fn i_times(m: &IMat) -> GMat {
        GMat { re: IMat::zeros(m.n, m.m), im: m.clone() }
    }
    pub fn mul(&self, b: &GMat) -> GMat {
        GMat {
            re: self.re.mul(&b.re).sub(&self.im.mul(&b.im)),
            im: self.re.mul(&b.im).add(&self.im.mul(&b.re)),
        }
    }
    pub fn add(&self, b: &GMat) -> GMat {
        GMat { re: self.re.add(&b.re), im: self.im.add(&b.im) }
    }
    pub fn sub(&self, b: &GMat) -> GMat {
        GMat { re: self.re.sub(&b.re), im: self.im.sub(&b.im) }
    }
    pub fn scale(&self, c: i64) -> GMat {
        GMat { re: self.re.scale(c), im: self.im.scale(c) }
    }
    /// Hermitian conjugate (complex conjugate of the transpose).
    pub fn dagger(&self) -> GMat {
        GMat { re: self.re.transpose(), im: self.im.transpose().neg() }
    }
    pub fn is_zero(&self) -> bool {
        self.re.is_zero() && self.im.is_zero()
    }
    pub fn is_real(&self) -> bool {
        self.im.is_zero()
    }
}

/// The Mersenne prime 2^61 - 1 used for rank computations modulo a prime.
pub const RANK_PRIME: u64 = (1u64 << 61) - 1;

/// Rank of an integer matrix (given as a list of rows) over the finite field
/// F_p, p = 2^61 - 1, by Gaussian elimination.
///
/// Why modulo a prime: exact elimination over the integers produces minors as
/// large as 2^500 for the 256 x 256 matrices that occur here, far beyond any
/// machine integer.  Modulo p every number stays below 2^61.
///
/// What it proves: a set of integer vectors that is linearly dependent over the
/// rationals is also dependent modulo p (clear denominators, divide by common
/// factors of p), so
///
/// ```text
/// rank over F_p  <=  rank over Q .
/// ```
///
/// Hence (i) if `rank` returns the number of rows, the rows are linearly
/// independent over Q (and R): a proof.  (ii) If `rank` returns r and r is
/// smaller than the number of columns n, the solution space of the homogeneous
/// system has dimension AT MOST n - r over Q; to prove that it is exactly n - r
/// the callers exhibit n - r independent solutions explicitly.
pub fn rank(rows: &[Vec<i64>]) -> usize {
    let p = RANK_PRIME as u128;
    let nr = rows.len();
    if nr == 0 {
        return 0;
    }
    let nc = rows[0].len();
    // reduce every entry to the range 0..p
    let mut a: Vec<Vec<u64>> = rows
        .iter()
        .map(|r| r.iter().map(|&x| (((x as i128) % (p as i128) + p as i128) % (p as i128)) as u64).collect())
        .collect();
    let mulmod = |x: u64, y: u64| -> u64 { ((x as u128 * y as u128) % p) as u64 };
    // modular inverse by Fermat's little theorem: x^(p-2) mod p
    let inv = |x: u64| -> u64 {
        let mut result = 1u64;
        let mut base = x;
        let mut e = RANK_PRIME - 2;
        while e > 0 {
            if e & 1 == 1 {
                result = mulmod(result, base);
            }
            base = mulmod(base, base);
            e >>= 1;
        }
        result
    };
    let mut rank = 0usize;
    let mut row = 0usize;
    for col in 0..nc {
        if row >= nr {
            break;
        }
        let piv = (row..nr).find(|&r| a[r][col] != 0);
        let pr = match piv {
            Some(pr) => pr,
            None => continue,
        };
        a.swap(row, pr);
        let pinv = inv(a[row][col]);
        for r in (row + 1)..nr {
            if a[r][col] == 0 {
                continue;
            }
            let f = mulmod(a[r][col], pinv);
            for c in col..nc {
                let t = mulmod(f, a[row][c]);
                a[r][c] = (a[r][c] + RANK_PRIME - t) % RANK_PRIME;
            }
        }
        row += 1;
        rank += 1;
    }
    rank
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn product_and_transpose() {
        let a = IMat::from_rows(&[vec![1, 2], vec![3, 4]]);
        let b = IMat::from_rows(&[vec![0, 1], vec![1, 0]]);
        assert_eq!(a.mul(&b), IMat::from_rows(&[vec![2, 1], vec![4, 3]]));
        assert_eq!(a.transpose(), IMat::from_rows(&[vec![1, 3], vec![2, 4]]));
        assert_eq!(a.trace(), 5);
    }
    #[test]
    fn rank_examples() {
        assert_eq!(rank(&[vec![1, 2, 3], vec![2, 4, 6], vec![1, 0, 1]]), 2);
        assert_eq!(rank(&[vec![2, 0], vec![0, 3]]), 2);
        assert_eq!(rank(&[vec![0, 0], vec![0, 0]]), 0);
    }
    #[test]
    fn gaussian_arithmetic() {
        // (i E)(i E) = -E
        let e = IMat::identity(2);
        let ie = GMat::i_times(&e);
        assert_eq!(ie.mul(&ie), GMat::from_real(&e.neg()));
        assert_eq!(ie.dagger(), ie.scale(-1));
    }
}
