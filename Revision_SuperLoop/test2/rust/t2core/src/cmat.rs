//! Dense complex matrices (pure Rust): products, Kronecker products, the matrix exponential and a
//! Hermitian eigensolver (Householder reduction to a real tridiagonal matrix, then the implicit
//! QL algorithm with Wilkinson shifts; the eigenvectors are accumulated).
use crate::c64::{c, C64};

/// A dense complex matrix with `rows` rows and `cols` columns, stored row by row.
#[derive(Clone, Debug, PartialEq)]
pub struct CMat {
    pub rows: usize,
    pub cols: usize,
    pub a: Vec<C64>,
}

impl CMat {
    pub fn new(rows: usize, cols: usize) -> CMat {
        CMat { rows, cols, a: vec![C64::ZERO; rows * cols] }
    }
    pub fn zeros(n: usize) -> CMat {
        CMat::new(n, n)
    }
    pub fn identity(n: usize) -> CMat {
        let mut m = CMat::zeros(n);
        for i in 0..n {
            m.a[i * n + i] = C64::ONE;
        }
        m
    }
    /// a square matrix from its rows
    pub fn from_rows(rows: &[Vec<C64>]) -> CMat {
        let n = rows.len();
        let mut m = CMat::new(n, rows[0].len());
        for (i, r) in rows.iter().enumerate() {
            assert_eq!(r.len(), m.cols);
            for (j, v) in r.iter().enumerate() {
                m.a[i * m.cols + j] = *v;
            }
        }
        m
    }
    /// a 2 x 2 matrix [[a, b], [cc, d]]
    pub fn m2(a: C64, b: C64, cc: C64, d: C64) -> CMat {
        CMat { rows: 2, cols: 2, a: vec![a, b, cc, d] }
    }
    #[inline]
    pub fn get(&self, i: usize, j: usize) -> C64 {
        self.a[i * self.cols + j]
    }
    #[inline]
    pub fn set(&mut self, i: usize, j: usize, v: C64) {
        self.a[i * self.cols + j] = v;
    }
    pub fn add(&self, o: &CMat) -> CMat {
        assert!(self.rows == o.rows && self.cols == o.cols);
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().zip(&o.a).map(|(x, y)| *x + *y).collect() }
    }
    pub fn sub(&self, o: &CMat) -> CMat {
        assert!(self.rows == o.rows && self.cols == o.cols);
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().zip(&o.a).map(|(x, y)| *x - *y).collect() }
    }
    pub fn scale(&self, s: C64) -> CMat {
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| *x * s).collect() }
    }
    pub fn scale_r(&self, s: f64) -> CMat {
        self.scale(c(s, 0.0))
    }
    pub fn mul(&self, o: &CMat) -> CMat {
        assert_eq!(self.cols, o.rows);
        let mut r = CMat::new(self.rows, o.cols);
        for i in 0..self.rows {
            for k in 0..self.cols {
                let x = self.a[i * self.cols + k];
                if x.re == 0.0 && x.im == 0.0 {
                    continue;
                }
                for j in 0..o.cols {
                    r.a[i * o.cols + j] += x * o.a[k * o.cols + j];
                }
            }
        }
        r
    }
    pub fn matvec(&self, v: &[C64]) -> Vec<C64> {
        assert_eq!(self.cols, v.len());
        (0..self.rows).map(|i| (0..self.cols).fold(C64::ZERO, |s, k| s + self.a[i * self.cols + k] * v[k])).collect()
    }
    pub fn dagger(&self) -> CMat {
        let mut r = CMat::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                r.a[j * self.rows + i] = self.a[i * self.cols + j].conj();
            }
        }
        r
    }
    pub fn transpose(&self) -> CMat {
        let mut r = CMat::new(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                r.a[j * self.rows + i] = self.a[i * self.cols + j];
            }
        }
        r
    }
    pub fn conj(&self) -> CMat {
        CMat { rows: self.rows, cols: self.cols, a: self.a.iter().map(|x| x.conj()).collect() }
    }
    /// Frobenius norm
    pub fn fro(&self) -> f64 {
        self.a.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt()
    }
    pub fn trace(&self) -> C64 {
        (0..self.rows.min(self.cols)).fold(C64::ZERO, |s, i| s + self.get(i, i))
    }
    /// Kronecker product self (x) o: entry ((i1, i2), (j1, j2)) = self[i1][j1] o[i2][j2],
    /// index = i1 * o.rows + i2
    pub fn kron(&self, o: &CMat) -> CMat {
        let mut r = CMat::new(self.rows * o.rows, self.cols * o.cols);
        for i1 in 0..self.rows {
            for j1 in 0..self.cols {
                let x = self.get(i1, j1);
                for i2 in 0..o.rows {
                    for j2 in 0..o.cols {
                        r.set(i1 * o.rows + i2, j1 * o.cols + j2, x * o.get(i2, j2));
                    }
                }
            }
        }
        r
    }
    /// commutator self o - o self
    pub fn comm(&self, o: &CMat) -> CMat {
        self.mul(o).sub(&o.mul(self))
    }
    /// anticommutator self o + o self
    pub fn acomm(&self, o: &CMat) -> CMat {
        self.mul(o).add(&o.mul(self))
    }
    /// the block-diagonal matrix diag(a, b)
    pub fn block_diag(a: &CMat, b: &CMat) -> CMat {
        let mut r = CMat::new(a.rows + b.rows, a.cols + b.cols);
        r.put(0, 0, a);
        r.put(a.rows, a.cols, b);
        r
    }
    /// the 2 x 2 block matrix [[a, b], [cc, d]] of equal square blocks
    pub fn blocks(a: &CMat, b: &CMat, cc: &CMat, d: &CMat) -> CMat {
        let n = a.rows;
        let mut r = CMat::zeros(2 * n);
        r.put(0, 0, a);
        r.put(0, n, b);
        r.put(n, 0, cc);
        r.put(n, n, d);
        r
    }
    /// writes the block `b` with its upper left corner at (i0, j0)
    pub fn put(&mut self, i0: usize, j0: usize, b: &CMat) {
        for i in 0..b.rows {
            for j in 0..b.cols {
                self.set(i0 + i, j0 + j, b.get(i, j));
            }
        }
    }
    /// the sub-block of size (r, cc) at (i0, j0)
    pub fn sub_block(&self, i0: usize, j0: usize, r: usize, cc: usize) -> CMat {
        let mut m = CMat::new(r, cc);
        for i in 0..r {
            for j in 0..cc {
                m.set(i, j, self.get(i0 + i, j0 + j));
            }
        }
        m
    }
    /// matrix exponential by scaling and squaring of a Taylor series (accurate to rounding for
    /// the moderate norms used here)
    pub fn expm(&self) -> CMat {
        let n = self.rows;
        let nrm = self.fro();
        let mut s = 0;
        let mut scale = 1.0;
        while nrm * scale > 0.25 {
            scale *= 0.5;
            s += 1;
        }
        let x = self.scale_r(scale);
        let mut term = CMat::identity(n);
        let mut sum = CMat::identity(n);
        for k in 1..30 {
            term = term.mul(&x).scale_r(1.0 / k as f64);
            sum = sum.add(&term);
            if term.fro() < 1e-18 {
                break;
            }
        }
        for _ in 0..s {
            sum = sum.mul(&sum);
        }
        sum
    }
    /// determinant of a 2 x 2 matrix
    pub fn det2(&self) -> C64 {
        self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0)
    }
    /// inverse of a 2 x 2 matrix
    pub fn inv2(&self) -> CMat {
        let d = self.det2();
        CMat::m2(self.get(1, 1) / d, -self.get(0, 1) / d, -self.get(1, 0) / d, self.get(0, 0) / d)
    }
    /// largest absolute value of an entry
    pub fn max_abs(&self) -> f64 {
        self.a.iter().fold(0.0f64, |m, x| m.max(x.abs()))
    }
}

/// Eigenvalues (ascending) and eigenvectors (the columns of the returned matrix) of a complex
/// Hermitian matrix.  Householder reduction to a real symmetric tridiagonal matrix (the phases of
/// the subdiagonal are removed by a diagonal unitary transformation), then the implicit QL
/// algorithm with Wilkinson shifts; cost about 9 n^3 complex operations.
pub fn eigh(m: &CMat) -> (Vec<f64>, CMat) {
    let n = m.rows;
    assert_eq!(n, m.cols);
    if n == 0 {
        return (vec![], CMat::zeros(0));
    }
    // ---------------------------------------------------------------- Householder reduction
    // work on a copy, column-major access through get/set; q accumulates the transformations
    let mut a = m.clone();
    // symmetrize numerically: a = (a + a^dag)/2
    for i in 0..n {
        for j in i..n {
            let v = (a.get(i, j) + a.get(j, i).conj()).scale(0.5);
            a.set(i, j, v);
            a.set(j, i, v.conj());
        }
    }
    let mut q = CMat::identity(n);
    let mut vv = vec![C64::ZERO; n];
    let mut p = vec![C64::ZERO; n];
    let mut w = vec![C64::ZERO; n];
    for k in 0..n.saturating_sub(2) {
        // x = a[k+1.., k]
        let mut xnorm2 = 0.0;
        for i in (k + 1)..n {
            xnorm2 += a.get(i, k).norm_sqr();
        }
        let xnorm = xnorm2.sqrt();
        if xnorm < 1e-300 {
            continue;
        }
        let x0 = a.get(k + 1, k);
        let phase = if x0.abs() > 0.0 { x0.scale(1.0 / x0.abs()) } else { C64::ONE };
        let alpha = -(phase.scale(xnorm));
        // v = x - alpha e1, normalized
        for i in 0..n {
            vv[i] = C64::ZERO;
        }
        for i in (k + 1)..n {
            vv[i] = a.get(i, k);
        }
        vv[k + 1] = vv[k + 1] - alpha;
        let vn = vv.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        if vn < 1e-300 {
            continue;
        }
        for i in (k + 1)..n {
            vv[i] = vv[i].scale(1.0 / vn);
        }
        // H = I - 2 v v^dag ; A <- H A H.  With p = A v, K = v^dag p (real), w = p - K v:
        // H A H = A - 2 v w^dag - 2 w v^dag
        for i in 0..n {
            p[i] = C64::ZERO;
        }
        for i in k..n {
            let mut s = C64::ZERO;
            for j in (k + 1)..n {
                s += a.get(i, j) * vv[j];
            }
            p[i] = s;
        }
        let mut kk = C64::ZERO;
        for i in (k + 1)..n {
            kk += vv[i].conj() * p[i];
        }
        for i in 0..n {
            w[i] = p[i] - vv[i] * kk;
        }
        // rows and columns before k are not touched (v and w vanish there)
        for i in k..n {
            for j in k..n {
                let upd = (vv[i] * w[j].conj() + w[i] * vv[j].conj()).scale(2.0);
                if upd.re != 0.0 || upd.im != 0.0 {
                    let cur = a.get(i, j);
                    a.set(i, j, cur - upd);
                }
            }
        }
        // q <- q H
        for i in 0..n {
            let mut s = C64::ZERO;
            for j in (k + 1)..n {
                s += q.get(i, j) * vv[j];
            }
            for j in (k + 1)..n {
                let cur = q.get(i, j);
                q.set(i, j, cur - s * vv[j].conj().scale(2.0));
            }
        }
    }
    // tridiagonal: diagonal d (real), subdiagonal e_c (complex); remove the phases with a
    // diagonal unitary D: T_real = D^dag T D, q <- q D
    let mut d = vec![0.0; n];
    let mut e = vec![0.0; n];
    let mut ph = vec![C64::ONE; n];
    for i in 0..n {
        d[i] = a.get(i, i).re;
    }
    for i in 1..n {
        let sub = a.get(i, i - 1); // T[i][i-1]
        // choose ph[i] so that conj(ph[i]) sub ph[i-1] is real and >= 0
        let r = sub.abs();
        if r > 0.0 {
            let z = sub * ph[i - 1];
            ph[i] = z.scale(1.0 / r);
        } else {
            ph[i] = ph[i - 1];
        }
        e[i] = r;
    }
    for i in 0..n {
        for j in 0..n {
            let cur = q.get(i, j);
            q.set(i, j, cur * ph[j]);
        }
    }
    // ---------------------------------------------------------------- implicit QL (tqli)
    // e[i] is the subdiagonal element between i-1 and i; shift to the tqli convention
    for i in 1..n {
        e[i - 1] = e[i];
    }
    e[n - 1] = 0.0;
    for l in 0..n {
        let mut iter = 0;
        loop {
            let mut mm = l;
            while mm < n - 1 {
                let dd = d[mm].abs() + d[mm + 1].abs();
                if e[mm].abs() <= f64::EPSILON * dd {
                    break;
                }
                mm += 1;
            }
            if mm == l {
                break;
            }
            iter += 1;
            if iter > 60 {
                break;
            }
            let mut g = (d[l + 1] - d[l]) / (2.0 * e[l]);
            let mut r = g.hypot(1.0);
            g = d[mm] - d[l] + e[l] / (g + if g >= 0.0 { r.abs() } else { -r.abs() });
            let (mut s, mut cc, mut pp) = (1.0f64, 1.0f64, 0.0f64);
            let mut i = mm as isize - 1;
            let mut underflow = false;
            while i >= l as isize {
                let iu = i as usize;
                let f = s * e[iu];
                let b = cc * e[iu];
                r = f.hypot(g);
                e[iu + 1] = r;
                if r == 0.0 {
                    d[iu + 1] -= pp;
                    e[mm] = 0.0;
                    underflow = true;
                    break;
                }
                s = f / r;
                cc = g / r;
                g = d[iu + 1] - pp;
                r = (d[iu] - g) * s + 2.0 * cc * b;
                pp = s * r;
                d[iu + 1] = g + pp;
                g = cc * r - b;
                // rotate the eigenvector columns iu, iu+1
                for k in 0..n {
                    let z1 = q.get(k, iu + 1);
                    let z0 = q.get(k, iu);
                    q.set(k, iu + 1, z0.scale(s) + z1.scale(cc));
                    q.set(k, iu, z0.scale(cc) - z1.scale(s));
                }
                i -= 1;
            }
            if underflow {
                continue;
            }
            d[l] -= pp;
            e[l] = g;
            e[mm] = 0.0;
        }
    }
    // sort ascending
    let mut idx: Vec<usize> = (0..n).collect();
    idx.sort_by(|&x, &y| d[x].partial_cmp(&d[y]).unwrap());
    let mut vals = vec![0.0; n];
    let mut vecs = CMat::zeros(n);
    for (newc, &oldc) in idx.iter().enumerate() {
        vals[newc] = d[oldc];
        for k in 0..n {
            vecs.set(k, newc, q.get(k, oldc));
        }
    }
    (vals, vecs)
}
