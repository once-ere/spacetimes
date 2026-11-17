//! A small fermionic Fock space: the states of n fermion modes (n at most 20) as complex vectors of length 2^n.
//!
//! The basis state with the bit pattern b (bit k set = mode k occupied) is |b> = (c_0^dag)^{b_0} (c_1^dag)^{b_1} ...
//! (c_{n-1}^dag)^{b_{n-1}} |0>, the creation operators taken in the order of increasing k.  With this order (the
//! Jordan-Wigner order) the annihilation operator c_k acts on |b> with bit k set as
//!     c_k |b> = (-1)^(number of occupied modes with index below k) |b with bit k cleared>,
//! and c_k^dag acts on |b> with bit k clear with the same sign, giving |b with bit k set>.  These operators satisfy the
//! canonical anticommutation relations {c_k, c_l^dag} = 1 for k = l and 0 otherwise, {c_k, c_l} = 0 (check SB-4 of the
//! program verifies the consequences it uses).
//!
//! Operators are applied to vectors; nothing is stored as a matrix except where a program asks for the dense matrix of
//! an operator on a small space (n = 8: 256 x 256).
use t2core::c64::C64;
use t2core::cmat::{eigh, CMat};
use t2core::rng::Rng;

/// the sign (-1)^(number of set bits of b below the bit k)
#[inline]
fn jw_sign(b: usize, k: usize) -> f64 {
    if (b & ((1usize << k) - 1)).count_ones() % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

/// c_k v
pub fn ann(v: &[C64], k: usize) -> Vec<C64> {
    let mut out = vec![C64::ZERO; v.len()];
    let bit = 1usize << k;
    for (b, &x) in v.iter().enumerate() {
        if b & bit != 0 && (x.re != 0.0 || x.im != 0.0) {
            out[b ^ bit] += x.scale(jw_sign(b, k));
        }
    }
    out
}

/// c_k^dag v
pub fn cre(v: &[C64], k: usize) -> Vec<C64> {
    let mut out = vec![C64::ZERO; v.len()];
    let bit = 1usize << k;
    for (b, &x) in v.iter().enumerate() {
        if b & bit == 0 && (x.re != 0.0 || x.im != 0.0) {
            out[b | bit] += x.scale(jw_sign(b, k));
        }
    }
    out
}

/// out += coef c_k^dag c_l v   (a hopping term; k = l gives the occupation number of mode k)
pub fn hop_add(v: &[C64], k: usize, l: usize, coef: C64, out: &mut [C64]) {
    let (bk, bl) = (1usize << k, 1usize << l);
    for (b, &x) in v.iter().enumerate() {
        if b & bl == 0 || (x.re == 0.0 && x.im == 0.0) {
            continue;
        }
        let s1 = jw_sign(b, l);
        let b1 = b ^ bl;
        if b1 & bk != 0 {
            continue;
        }
        let s2 = jw_sign(b1, k);
        out[b1 | bk] += (coef * x).scale(s1 * s2);
    }
}

/// out += coef c_k^dag c_l^dag v   (creation of a pair; zero for k = l)
pub fn pair_cre_add(v: &[C64], k: usize, l: usize, coef: C64, out: &mut [C64]) {
    if k == l {
        return;
    }
    let (bk, bl) = (1usize << k, 1usize << l);
    for (b, &x) in v.iter().enumerate() {
        if b & bl != 0 || (x.re == 0.0 && x.im == 0.0) {
            continue;
        }
        let s1 = jw_sign(b, l);
        let b1 = b | bl;
        if b1 & bk != 0 {
            continue;
        }
        let s2 = jw_sign(b1, k);
        out[b1 | bk] += (coef * x).scale(s1 * s2);
    }
}

/// out += coef c_l c_k v   (annihilation of a pair, the Hermitian conjugate of c_k^dag c_l^dag; zero for k = l)
pub fn pair_ann_add(v: &[C64], k: usize, l: usize, coef: C64, out: &mut [C64]) {
    if k == l {
        return;
    }
    let (bk, bl) = (1usize << k, 1usize << l);
    for (b, &x) in v.iter().enumerate() {
        if b & bk == 0 || (x.re == 0.0 && x.im == 0.0) {
            continue;
        }
        let s1 = jw_sign(b, k);
        let b1 = b ^ bk;
        if b1 & bl == 0 {
            continue;
        }
        let s2 = jw_sign(b1, l);
        out[b1 ^ bl] += (coef * x).scale(s1 * s2);
    }
}

/// sum_k w_k c_k^dag v: creates a fermion in the state with the components w (on the modes offset .. offset + w.len())
pub fn cre_state(v: &[C64], w: &[C64], offset: usize) -> Vec<C64> {
    let mut out = vec![C64::ZERO; v.len()];
    for (k, &wk) in w.iter().enumerate() {
        if wk.abs() < 1e-300 {
            continue;
        }
        let t = cre(v, offset + k);
        for (o, x) in out.iter_mut().zip(t) {
            *o += wk * x;
        }
    }
    out
}

/// sum_k conj(w_k) c_k v: removes a fermion from the state with the components w
pub fn ann_state(v: &[C64], w: &[C64], offset: usize) -> Vec<C64> {
    let mut out = vec![C64::ZERO; v.len()];
    for (k, &wk) in w.iter().enumerate() {
        if wk.abs() < 1e-300 {
            continue;
        }
        let t = ann(v, offset + k);
        for (o, x) in out.iter_mut().zip(t) {
            *o += wk.conj() * x;
        }
    }
    out
}

/// <a|b>
pub fn dot(a: &[C64], b: &[C64]) -> C64 {
    let mut s = C64::ZERO;
    for (x, y) in a.iter().zip(b) {
        s += x.conj() * *y;
    }
    s
}

pub fn norm(a: &[C64]) -> f64 {
    a.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}

/// a + s b
pub fn axpy(a: &[C64], s: C64, b: &[C64]) -> Vec<C64> {
    a.iter().zip(b).map(|(x, y)| *x + s * *y).collect()
}

/// the dense matrix of a linear operator on a space of small dimension (column j = the operator applied to basis state j)
pub fn dense(dim: usize, op: &dyn Fn(&[C64]) -> Vec<C64>) -> CMat {
    let mut m = CMat::zeros(dim);
    let mut e = vec![C64::ZERO; dim];
    for j in 0..dim {
        e[j] = C64::ONE;
        let col = op(&e);
        for i in 0..dim {
            m.set(i, j, col[i]);
        }
        e[j] = C64::ZERO;
    }
    m
}

/// The lowest eigenvalue and its eigenvector of a Hermitian operator, by the Lanczos method with full
/// reorthogonalization: the Krylov vectors v_1, H v_1, ... are made orthonormal one after the other (Gram-Schmidt,
/// applied twice), the operator restricted to their span is a real symmetric tridiagonal matrix, and the eigenvector
/// of its lowest eigenvalue, expanded in the Krylov vectors, approximates the ground state.  The start vector is
/// pseudo-random (fixed seed).  Returns (eigenvalue, normalized eigenvector, residual norm |H x - E x|).
pub fn lanczos_ground(op: &dyn Fn(&[C64]) -> Vec<C64>, dim: usize, iters: usize, seed: u64) -> (f64, Vec<C64>, f64) {
    let mut rng = Rng(seed);
    let mut v: Vec<C64> = (0..dim).map(|_| C64 { re: rng.unit(), im: rng.unit() }).collect();
    let n0 = norm(&v);
    for x in v.iter_mut() {
        *x = x.scale(1.0 / n0);
    }
    let mut basis: Vec<Vec<C64>> = vec![v];
    let mut alpha: Vec<f64> = vec![];
    let mut beta: Vec<f64> = vec![];
    for it in 0..iters {
        let mut w = op(&basis[it]);
        let a = dot(&basis[it], &w).re;
        alpha.push(a);
        for _ in 0..2 {
            for b in &basis {
                let ov = dot(b, &w);
                w = axpy(&w, -ov, b);
            }
        }
        let nb = norm(&w);
        if nb < 1e-13 || it + 1 == iters {
            break;
        }
        beta.push(nb);
        basis.push(w.iter().map(|x| x.scale(1.0 / nb)).collect());
    }
    let k = alpha.len();
    let mut t = CMat::zeros(k);
    for i in 0..k {
        t.set(i, i, C64 { re: alpha[i], im: 0.0 });
        if i + 1 < k {
            t.set(i, i + 1, C64 { re: beta[i], im: 0.0 });
            t.set(i + 1, i, C64 { re: beta[i], im: 0.0 });
        }
    }
    let (ev, vecs) = eigh(&t);
    let mut x = vec![C64::ZERO; dim];
    for i in 0..k {
        let coef = vecs.get(i, 0);
        for (xi, bi) in x.iter_mut().zip(&basis[i]) {
            *xi += coef * *bi;
        }
    }
    let nx = norm(&x);
    for xi in x.iter_mut() {
        *xi = xi.scale(1.0 / nx);
    }
    let hx = op(&x);
    let res = norm(&axpy(&hx, C64 { re: -ev[0], im: 0.0 }, &x));
    (ev[0], x, res)
}
