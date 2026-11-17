//! Exact linear algebra on integer matrices: products, commutators, spans and ranks.
//!
//! Two kinds of span are used.  `ExactSpan` is exact over the rationals (fraction-free
//! elimination with integer rows divided by their gcd); it is used for the Lie closures, so
//! every closure dimension of this folder is exact.  `ModSpan` works modulo the prime
//! p = 2^61 - 1 and is used for the large linear systems: a rank modulo p is a LOWER bound for
//! the rank over the rationals (vectors independent modulo p are independent over Q), so a
//! nullity computed from it is an UPPER bound; every such nullity in this folder is matched by
//! the same number of explicit, exactly verified solutions.

pub type IM = Vec<Vec<i128>>;

pub fn zeros(n: usize) -> IM {
    vec![vec![0i128; n]; n]
}

pub fn mmul(a: &IM, b: &IM) -> IM {
    let n = a.len();
    let k = b.len();
    let m = b[0].len();
    let mut c = vec![vec![0i128; m]; n];
    for i in 0..n {
        for l in 0..k {
            let x = a[i][l];
            if x == 0 {
                continue;
            }
            for j in 0..m {
                c[i][j] += x * b[l][j];
            }
        }
    }
    c
}

pub fn madd(a: &IM, b: &IM) -> IM {
    a.iter().zip(b.iter()).map(|(r, s)| r.iter().zip(s.iter()).map(|(x, y)| x + y).collect()).collect()
}

pub fn msub(a: &IM, b: &IM) -> IM {
    a.iter().zip(b.iter()).map(|(r, s)| r.iter().zip(s.iter()).map(|(x, y)| x - y).collect()).collect()
}

pub fn transpose(a: &IM) -> IM {
    let n = a.len();
    let m = a[0].len();
    let mut t = vec![vec![0i128; n]; m];
    for i in 0..n {
        for j in 0..m {
            t[j][i] = a[i][j];
        }
    }
    t
}

/// commutator AB - BA
pub fn comm(a: &IM, b: &IM) -> IM {
    msub(&mmul(a, b), &mmul(b, a))
}

pub fn is_zero(a: &IM) -> bool {
    a.iter().all(|r| r.iter().all(|x| *x == 0))
}

pub fn flatten(a: &IM) -> Vec<i128> {
    a.iter().flat_map(|r| r.iter().cloned()).collect()
}

/// The span of integer vectors modulo a prime, built incrementally.
pub struct ModSpan {
    p: u128,
    rows: Vec<(usize, Vec<u128>)>, // (pivot column, row normalized to pivot 1)
}

fn modp(x: i128, p: u128) -> u128 {
    let r = x.rem_euclid(p as i128);
    r as u128
}

fn powmod(mut a: u128, mut e: u128, p: u128) -> u128 {
    let mut r = 1u128;
    a %= p;
    while e > 0 {
        if e & 1 == 1 {
            r = r * a % p;
        }
        a = a * a % p;
        e >>= 1;
    }
    r
}

impl ModSpan {
    /// span modulo p = 2^61 - 1
    pub fn new() -> ModSpan {
        ModSpan { p: (1u128 << 61) - 1, rows: Vec::new() }
    }
    pub fn rank(&self) -> usize {
        self.rows.len()
    }
    /// reduce v against the span; returns the residual (mod p)
    fn reduce(&self, v: &[i128]) -> Vec<u128> {
        let p = self.p;
        let mut w: Vec<u128> = v.iter().map(|&x| modp(x, p)).collect();
        for (piv, r) in &self.rows {
            let f = w[*piv];
            if f != 0 {
                for j in 0..w.len() {
                    if r[j] != 0 {
                        w[j] = (w[j] + p - (f * r[j] % p)) % p;
                    }
                }
            }
        }
        w
    }
    /// insert v; returns true if v was independent of the span
    pub fn insert(&mut self, v: &[i128]) -> bool {
        let p = self.p;
        let mut w = self.reduce(v);
        let piv = match w.iter().position(|&x| x != 0) {
            Some(k) => k,
            None => return false,
        };
        let inv = powmod(w[piv], p - 2, p);
        for x in w.iter_mut() {
            *x = *x * inv % p;
        }
        // keep the rows fully reduced at the new pivot column
        for (_, r) in self.rows.iter_mut() {
            let f = r[piv];
            if f != 0 {
                for j in 0..r.len() {
                    if w[j] != 0 {
                        r[j] = (r[j] + p - (f * w[j] % p)) % p;
                    }
                }
            }
        }
        self.rows.push((piv, w));
        true
    }
}

/// Rank of a list of integer vectors (mod 2^61 - 1).
pub fn rank(vs: &[Vec<i128>]) -> usize {
    let mut s = ModSpan::new();
    for v in vs {
        s.insert(v);
    }
    s.rank()
}

fn gcd(mut a: i128, mut b: i128) -> i128 {
    a = a.abs();
    b = b.abs();
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// The span of integer vectors over the rationals (exact): rows in echelon form, each an
/// integer vector divided by the gcd of its entries.
pub struct ExactSpan {
    rows: Vec<(usize, Vec<i128>)>,
}

impl ExactSpan {
    pub fn new() -> ExactSpan {
        ExactSpan { rows: Vec::new() }
    }
    pub fn rank(&self) -> usize {
        self.rows.len()
    }
    fn reduce(&self, v: &[i128]) -> Vec<i128> {
        let mut w = v.to_vec();
        for (piv, r) in &self.rows {
            let f = w[*piv];
            if f != 0 {
                let a = r[*piv];
                for j in 0..w.len() {
                    w[j] = a * w[j] - f * r[j];
                }
                let g = w.iter().fold(0i128, |acc, &x| gcd(acc, x));
                if g > 1 {
                    for x in w.iter_mut() {
                        *x /= g;
                    }
                }
            }
        }
        w
    }
    /// insert v; returns true if v was independent of the span (exact over Q)
    pub fn insert(&mut self, v: &[i128]) -> bool {
        let w = self.reduce(v);
        match w.iter().position(|&x| x != 0) {
            None => false,
            Some(piv) => {
                self.rows.push((piv, w));
                true
            }
        }
    }
}

/// Dimension of the Lie algebra generated (under commutators) by integer matrices `gens`,
/// exact over the rationals; returns (dimension, basis matrices).
pub fn lie_closure(gens: &[IM]) -> (usize, Vec<IM>) {
    let mut span = ExactSpan::new();
    let mut basis: Vec<IM> = Vec::new();
    for g in gens {
        if span.insert(&flatten(g)) {
            basis.push(g.clone());
        }
    }
    let mut i = 0;
    while i < basis.len() {
        let mut j = 0;
        while j < i {
            let c = comm(&basis[i], &basis[j]);
            if !is_zero(&c) && span.insert(&flatten(&c)) {
                basis.push(c);
            }
            j += 1;
        }
        i += 1;
    }
    (basis.len(), basis)
}

/// Signature (n_plus, n_minus, n_zero) of a real symmetric matrix given exactly (integers),
/// computed by an exact LDL^T (Lagrange) reduction over the rationals with i128 numerators.
pub fn signature(m: &IM) -> (usize, usize, usize) {
    // symmetric Gaussian elimination with rational arithmetic (num/den kept in i128)
    let n = m.len();
    let mut a: Vec<Vec<(i128, i128)>> = m.iter().map(|r| r.iter().map(|&x| (x, 1i128)).collect()).collect();
    fn norm(x: (i128, i128)) -> (i128, i128) {
        let (mut p, mut q) = x;
        if q < 0 {
            p = -p;
            q = -q;
        }
        let g = gcd(p.abs(), q);
        if g > 1 {
            (p / g, q / g)
        } else {
            (p, q)
        }
    }
    fn gcd(mut a: i128, mut b: i128) -> i128 {
        while b != 0 {
            let t = a % b;
            a = b;
            b = t;
        }
        a.abs().max(1)
    }
    fn sub(x: (i128, i128), y: (i128, i128)) -> (i128, i128) {
        norm((x.0 * y.1 - y.0 * x.1, x.1 * y.1))
    }
    fn mul(x: (i128, i128), y: (i128, i128)) -> (i128, i128) {
        norm((x.0 * y.0, x.1 * y.1))
    }
    fn div(x: (i128, i128), y: (i128, i128)) -> (i128, i128) {
        norm((x.0 * y.1, x.1 * y.0))
    }
    let (mut pos, mut neg, mut zero) = (0, 0, 0);
    let mut active: Vec<usize> = (0..n).collect();
    while !active.is_empty() {
        // find a non-zero diagonal pivot; if none, make one with a congruence x_i -> x_i + x_j
        let mut piv = active.iter().cloned().find(|&i| a[i][i].0 != 0);
        if piv.is_none() {
            let mut found = None;
            'outer: for &i in &active {
                for &j in &active {
                    if i != j && a[i][j].0 != 0 {
                        found = Some((i, j));
                        break 'outer;
                    }
                }
            }
            match found {
                None => {
                    zero += active.len();
                    break;
                }
                Some((i, j)) => {
                    // row/column i += row/column j
                    for k in 0..n {
                        a[i][k] = norm((a[i][k].0 * a[j][k].1 + a[j][k].0 * a[i][k].1, a[i][k].1 * a[j][k].1));
                    }
                    for k in 0..n {
                        a[k][i] = norm((a[k][i].0 * a[k][j].1 + a[k][j].0 * a[k][i].1, a[k][i].1 * a[k][j].1));
                    }
                    piv = Some(i);
                }
            }
        }
        let p = piv.unwrap();
        let d = a[p][p];
        if d.0 > 0 {
            pos += 1;
        } else {
            neg += 1;
        }
        let others: Vec<usize> = active.iter().cloned().filter(|&i| i != p).collect();
        for &i in &others {
            let f = div(a[i][p], d);
            if f.0 == 0 {
                continue;
            }
            for &j in &others {
                a[i][j] = sub(a[i][j], mul(f, a[p][j]));
            }
        }
        for &i in &others {
            a[i][p] = (0, 1);
            a[p][i] = (0, 1);
        }
        active = others;
    }
    (pos, neg, zero)
}
