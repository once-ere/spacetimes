//! Exact rational arithmetic and exact linear algebra.
//!
//! Every structural statement of this program (an algebra identity, a dimension,
//! the existence or the non-existence of a solution of a linear system) is proved
//! in EXACT arithmetic: the entries are rational numbers p/q with 128-bit integer
//! numerator and denominator, always reduced (gcd(p, q) = 1, q > 0).  The crate is
//! compiled with overflow checks, so an overflow would stop the program instead of
//! producing a wrong result; with the small integer entries of the octonion
//! structure constants and of the Dirac matrices no overflow occurs.
//!
//! Linear algebra by Gauss-Jordan elimination over the rationals:
//! * `rref`        reduced row echelon form, returns the pivot columns;
//! * `rank`        rank of a matrix;
//! * `null_space`  a basis of { x : A x = 0 };
//! * `solve`       the solutions of A x = b (None if the system is inconsistent);
//! * `char_poly`   characteristic polynomial det(lambda 1 - M) by the
//!                 Faddeev-LeVerrier recursion (exact in rational arithmetic).

use std::fmt;

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// A rational number n/d, reduced, with d > 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Q {
    pub n: i128,
    pub d: i128,
}

impl Q {
    /// n/d reduced (panics if d = 0).
    pub fn new(n: i128, d: i128) -> Q {
        assert!(d != 0, "zero denominator");
        let g = gcd(n, d).max(1);
        let s = if d < 0 { -1 } else { 1 };
        Q { n: s * n / g, d: s * d / g }
    }
    pub const ZERO: Q = Q { n: 0, d: 1 };
    pub const ONE: Q = Q { n: 1, d: 1 };
    /// An integer as a rational.
    pub fn int(n: i64) -> Q {
        Q { n: n as i128, d: 1 }
    }
    pub fn is_zero(self) -> bool {
        self.n == 0
    }
    pub fn add(self, o: Q) -> Q {
        if self.d == o.d {
            return Q::new(self.n + o.n, self.d);
        }
        Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    pub fn sub(self, o: Q) -> Q {
        self.add(o.neg())
    }
    pub fn mul(self, o: Q) -> Q {
        if self.n == 0 || o.n == 0 {
            return Q::ZERO;
        }
        // cross-reduce first to keep the numbers small
        let g1 = gcd(self.n, o.d).max(1);
        let g2 = gcd(o.n, self.d).max(1);
        Q::new((self.n / g1) * (o.n / g2), (self.d / g2) * (o.d / g1))
    }
    pub fn div(self, o: Q) -> Q {
        assert!(o.n != 0, "division by zero");
        self.mul(Q { n: o.d, d: o.n }.normal())
    }
    fn normal(self) -> Q {
        Q::new(self.n, self.d)
    }
    pub fn neg(self) -> Q {
        Q { n: -self.n, d: self.d }
    }
    pub fn to_f64(self) -> f64 {
        self.n as f64 / self.d as f64
    }
}

impl fmt::Display for Q {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.d == 1 {
            write!(f, "{}", self.n)
        } else {
            write!(f, "{}/{}", self.n, self.d)
        }
    }
}

/// A rational matrix as a list of rows.
pub type QMat = Vec<Vec<Q>>;

/// Bring `a` (rows x cols) to reduced row echelon form in place; return the
/// pivot columns in increasing order.
pub fn rref(a: &mut QMat) -> Vec<usize> {
    let rows = a.len();
    if rows == 0 {
        return vec![];
    }
    let cols = a[0].len();
    let mut pivots = Vec::new();
    let mut r = 0;
    for c in 0..cols {
        if r >= rows {
            break;
        }
        // find a row with a nonzero entry in column c
        let mut p = None;
        for i in r..rows {
            if !a[i][c].is_zero() {
                p = Some(i);
                break;
            }
        }
        let p = match p {
            Some(p) => p,
            None => continue,
        };
        a.swap(r, p);
        let inv = Q::ONE.div(a[r][c]);
        for j in c..cols {
            a[r][j] = a[r][j].mul(inv);
        }
        for i in 0..rows {
            if i != r && !a[i][c].is_zero() {
                let f = a[i][c];
                for j in c..cols {
                    if !a[r][j].is_zero() {
                        a[i][j] = a[i][j].sub(f.mul(a[r][j]));
                    }
                }
            }
        }
        pivots.push(c);
        r += 1;
    }
    pivots
}

/// Rank of a rational matrix.
pub fn rank(a: &QMat) -> usize {
    let mut b = a.clone();
    rref(&mut b).len()
}

/// A basis of the null space { x : A x = 0 } of an (rows x ncols) matrix.
pub fn null_space(a: &QMat, ncols: usize) -> Vec<Vec<Q>> {
    let mut b = a.clone();
    let piv = rref(&mut b);
    let free: Vec<usize> = (0..ncols).filter(|c| !piv.contains(c)).collect();
    let mut basis = Vec::new();
    for &f in &free {
        let mut x = vec![Q::ZERO; ncols];
        x[f] = Q::ONE;
        for (r, &pc) in piv.iter().enumerate() {
            x[pc] = b[r][f].neg();
        }
        basis.push(x);
    }
    basis
}

/// Solutions of A x = b: Some((particular solution, dimension of the kernel)) if
/// the system is consistent, None if it is inconsistent.
pub fn solve(a: &QMat, b: &[Q]) -> Option<(Vec<Q>, usize)> {
    let ncols = if a.is_empty() { 0 } else { a[0].len() };
    let mut aug: QMat = a.iter().zip(b).map(|(r, &bi)| {
        let mut row = r.clone();
        row.push(bi);
        row
    }).collect();
    let piv = rref(&mut aug);
    if piv.contains(&ncols) {
        return None; // a pivot in the right-hand-side column: 0 = 1
    }
    let mut x = vec![Q::ZERO; ncols];
    for (r, &pc) in piv.iter().enumerate() {
        x[pc] = aug[r][ncols];
    }
    Some((x, ncols - piv.len()))
}

/// Matrix product of square rational matrices.
pub fn qmul(a: &QMat, b: &QMat) -> QMat {
    let n = a.len();
    let m = b[0].len();
    let k = b.len();
    let mut c = vec![vec![Q::ZERO; m]; n];
    for i in 0..n {
        for l in 0..k {
            if a[i][l].is_zero() {
                continue;
            }
            for j in 0..m {
                if !b[l][j].is_zero() {
                    c[i][j] = c[i][j].add(a[i][l].mul(b[l][j]));
                }
            }
        }
    }
    c
}

/// Characteristic polynomial det(lambda 1 - M) = sum_k c[k] lambda^k (c[n] = 1)
/// of a square rational matrix, by the Faddeev-LeVerrier recursion
///     M_1 = M,  c_{n-1} = -tr M_1,
///     M_k = M (M_{k-1} + c_{n-k+1} 1),  c_{n-k} = -tr(M_k)/k.
pub fn char_poly(m: &QMat) -> Vec<Q> {
    let n = m.len();
    let mut c = vec![Q::ZERO; n + 1];
    c[n] = Q::ONE;
    let mut mk = m.clone();
    for k in 1..=n {
        if k > 1 {
            let mut t = mk.clone();
            for i in 0..n {
                t[i][i] = t[i][i].add(c[n - k + 1]);
            }
            mk = qmul(m, &t);
        }
        let mut tr = Q::ZERO;
        for i in 0..n {
            tr = tr.add(mk[i][i]);
        }
        c[n - k] = tr.neg().div(Q::int(k as i64));
    }
    c
}

/// Human-readable polynomial sum_k c[k] x^k (highest power first).
pub fn poly_string(c: &[Q]) -> String {
    let mut s = String::new();
    for k in (0..c.len()).rev() {
        if c[k].is_zero() {
            continue;
        }
        let term = match k {
            0 => format!("{}", c[k]),
            1 => format!("{} x", c[k]),
            _ => format!("{} x^{}", c[k], k),
        };
        if !s.is_empty() {
            s += " + ";
        }
        s += &term;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rational_basics() {
        let a = Q::new(2, 4);
        assert_eq!(a, Q::new(1, 2));
        assert_eq!(a.add(Q::new(1, 3)), Q::new(5, 6));
        assert_eq!(a.mul(Q::new(2, 3)), Q::new(1, 3));
        assert_eq!(a.div(Q::new(1, 4)), Q::int(2));
    }
    #[test]
    fn charpoly_rotation() {
        // rotation generator [[0,-1],[1,0]]: lambda^2 + 1
        let m = vec![vec![Q::ZERO, Q::int(-1)], vec![Q::ONE, Q::ZERO]];
        assert_eq!(char_poly(&m), vec![Q::ONE, Q::ZERO, Q::ONE]);
    }
}
