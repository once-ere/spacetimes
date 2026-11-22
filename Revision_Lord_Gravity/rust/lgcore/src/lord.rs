//! lord -- the matrices and operations of E. A. Lord, "General relativity from gauge invariance", Proc. Camb. Phil.
//! Soc. 69, 423 (1971), built from his own definitions (his equations (1.1) to (1.17) and (4.1) to (4.20)), so that the
//! program lg_lord can check his equations and compare his conventions with those of the project.
//!
//! Conventions (all Lord's, as he states them in his Section 1):
//! * Greek indices mu = 0..3 are upper unless the name of the function says "lower"; the metric is
//!   eta = diag(1, -1, -1, -1), used to raise and lower indices;
//! * sigma^mu = (1, sigma_1, sigma_2, sigma_3) with the Pauli matrices (1.1); epsilon = ((0, 1), (-1, 0)) (1.1);
//! * sigma-bar is defined for any set of four 2x2 matrices by sigma-bar^mu = -epsilon (sigma^mu)^T epsilon (1.2), which is
//!   (1, -sigma_j) for the standard set; for a set changed by an SL(2,C) matrix it is his definition (1.23), (4.3);
//! * sigma^{mu nu} = 1/2 (sigma^mu sigma-bar^nu - sigma^nu sigma-bar^mu) (1.4), (4.9);
//! * gamma^mu = ((0, sigma^mu), (sigma-bar^mu, 0)) and gamma^5 = diag(1, -1) in 2x2 blocks: the Weyl representation
//!   (1.9), (4.5); gamma^{mu nu} = 1/2 (gamma^mu gamma^nu - gamma^nu gamma^mu) (1.15), (4.15);
//! * C = diag(epsilon, -epsilon) (1.14);
//! * the Levi-Civita symbol with lower indices has eps_0123 = +1 (1.6).
//!
//! The functions with the prefix `d_` act on matrices of dual numbers (`DMat`): fields of the four coordinates that carry
//! their exact first and second derivatives, used for the checks of Lord's Section 4 (local transformations).
use crate::c64::{c, C64};
use crate::cmat::CMat;
use crate::dual::{DMat, D4};

/// eta^{mu nu} = eta_{mu nu} = diag(1, -1, -1, -1)
pub fn eta(m: usize, n: usize) -> f64 {
    if m != n {
        0.0
    } else if m == 0 {
        1.0
    } else {
        -1.0
    }
}

/// Lord's sigma^mu (upper index): sigma^0 = 1, sigma^j = the Pauli matrix sigma_j (1.1), written out here from his
/// definition and not taken from the project's matrices, so that the identity of the two is a check
pub fn sig(m: usize) -> CMat {
    let (o, z, i) = (C64::ONE, C64::ZERO, C64::I);
    match m {
        0 => CMat::m2(o, z, z, o),
        1 => CMat::m2(z, o, o, z),
        2 => CMat::m2(z, -i, i, z),
        3 => CMat::m2(o, z, z, -o),
        _ => panic!("sig: index {} out of range", m),
    }
}

/// Lord's epsilon = ((0, 1), (-1, 0)) (1.1)
pub fn eps() -> CMat {
    CMat::m2(C64::ZERO, C64::ONE, -C64::ONE, C64::ZERO)
}

/// sigma-bar of one matrix by Lord's definition (1.2): -epsilon s^T epsilon
pub fn bar(s: &CMat) -> CMat {
    eps().mul(&s.transpose()).mul(&eps()).scale_r(-1.0)
}

/// sigma^{mu nu} of a set of four matrices s[0..4] (1.4): 1/2 (s^mu bar(s^nu) - s^nu bar(s^mu))
pub fn sig_mn(s: &[CMat], m: usize, n: usize) -> CMat {
    s[m].mul(&bar(&s[n])).sub(&s[n].mul(&bar(&s[m]))).scale_r(0.5)
}

/// the Dirac matrix of one member of a set of Pauli-type matrices (1.9), (4.5): ((0, s), (bar(s), 0))
pub fn gam(s: &CMat) -> CMat {
    let z = CMat::zeros(2);
    CMat::blocks(&z, s, &bar(s), &z)
}

/// Lord's gamma^5 = diag(1, -1) in 2x2 blocks (1.9)
pub fn gam5() -> CMat {
    CMat::block_diag(&CMat::identity(2), &CMat::identity(2).scale_r(-1.0))
}

/// Lord's charge-conjugation matrix C = diag(epsilon, -epsilon) (1.14)
pub fn cc() -> CMat {
    CMat::block_diag(&eps(), &eps().scale_r(-1.0))
}

/// gamma^{mu nu} = 1/2 (gamma^mu gamma^nu - gamma^nu gamma^mu) of a set of four Dirac matrices (1.15)
pub fn gam_mn(g: &[CMat], m: usize, n: usize) -> CMat {
    g[m].comm(&g[n]).scale_r(0.5)
}

/// the Levi-Civita symbol with lower indices, eps_{abcd}, eps_0123 = +1 (1.6): the sign of the permutation
pub fn levi4(a: usize, b: usize, cc_: usize, d: usize) -> f64 {
    let p = [a, b, cc_, d];
    for i in 0..4 {
        for j in i + 1..4 {
            if p[i] == p[j] {
                return 0.0;
            }
        }
    }
    let mut sign = 1.0;
    let mut q = p;
    for i in 0..4 {
        for j in 0..3 - i {
            if q[j] > q[j + 1] {
                q.swap(j, j + 1);
                sign = -sign;
            }
        }
    }
    sign
}

/// the expansion of a 2x2 matrix in the basis sigma^rho: x = sum_rho x_rho sigma^rho with x_rho = 1/2 tr(x sigma^rho)
/// (complex coefficients; real for a Hermitian x)
pub fn coeffs(x: &CMat) -> [C64; 4] {
    let mut r = [C64::ZERO; 4];
    for (rho, item) in r.iter_mut().enumerate() {
        *item = x.mul(&sig(rho)).trace().scale(0.5);
    }
    r
}

/// the inverse of a real 4x4 matrix (Gauss-Jordan elimination with partial pivoting)
pub fn inv4(m: &[[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut a = *m;
    let mut b = [[0.0; 4]; 4];
    for (i, row) in b.iter_mut().enumerate() {
        row[i] = 1.0;
    }
    for col in 0..4 {
        let mut piv = col;
        for r in col + 1..4 {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        a.swap(col, piv);
        b.swap(col, piv);
        let d = a[col][col];
        for j in 0..4 {
            a[col][j] /= d;
            b[col][j] /= d;
        }
        for r in 0..4 {
            if r != col {
                let f = a[r][col];
                for j in 0..4 {
                    a[r][j] -= f * a[col][j];
                    b[r][j] -= f * b[col][j];
                }
            }
        }
    }
    b
}

/// the Lorentz matrix Lambda (entry [nu][mu] = Lambda_nu^mu) that Lord's (1.8) assigns to a matrix L of SL(2,C):
/// Lambda_nu^mu L sigma^nu L^dag = sigma^mu.  With L sigma^nu L^dag = sum_rho K^nu_rho sigma^rho (K real, from `coeffs`),
/// (1.8) reads sum_nu Lambda_nu^mu K^nu_rho = delta^mu_rho, that is Lambda^T K = 1 and Lambda = (K^{-1})^T.
pub fn lorentz_of(l: &CMat) -> [[f64; 4]; 4] {
    let mut k = [[0.0; 4]; 4];
    for (nu, row) in k.iter_mut().enumerate() {
        let x = coeffs(&l.mul(&sig(nu)).mul(&l.dagger()));
        for rho in 0..4 {
            row[rho] = x[rho].re;
        }
    }
    let ki = inv4(&k);
    let mut lam = [[0.0; 4]; 4];
    for nu in 0..4 {
        for mu in 0..4 {
            lam[nu][mu] = ki[mu][nu];
        }
    }
    lam
}

/// the polarized determinant form N(A, B) = 1/2 (det(A + B) - det A - det B) of two 2x2 matrices: the Minkowski product
/// of two Hermitian matrices read as points of space-time (N(sigma^mu, sigma^nu) = eta^{mu nu})
pub fn ndet(a: &CMat, b: &CMat) -> C64 {
    (a.add(b).det2() - a.det2() - b.det2()).scale(0.5)
}

// ------------------------------------------------------------------------------------------------------------------
// fields: matrices of dual numbers
// ------------------------------------------------------------------------------------------------------------------

/// the transpose of a matrix of dual numbers (no complex conjugation)
pub fn d_transpose(m: &DMat) -> DMat {
    let mut r = DMat::new(m.cols, m.rows);
    for i in 0..m.rows {
        for j in 0..m.cols {
            r.set(j, i, m.get(i, j));
        }
    }
    r
}

/// a 4x4 matrix of dual numbers from four 2x2 blocks ((a, b), (c, d))
pub fn d_blocks(a: &DMat, b: &DMat, cc_: &DMat, d: &DMat) -> DMat {
    let mut r = DMat::new(4, 4);
    for i in 0..2 {
        for j in 0..2 {
            r.set(i, j, a.get(i, j));
            r.set(i, j + 2, b.get(i, j));
            r.set(i + 2, j, cc_.get(i, j));
            r.set(i + 2, j + 2, d.get(i, j));
        }
    }
    r
}

/// sigma-bar of a field of 2x2 matrices by Lord's definition (1.2), (4.3): -epsilon s^T epsilon
pub fn d_bar(s: &DMat) -> DMat {
    let e = DMat::from_cmat(&eps());
    e.mul(&d_transpose(s)).mul(&e).scale(D4::re(-1.0))
}

/// the Dirac matrix field of a field of Pauli-type matrices (4.5): ((0, s), (bar(s), 0))
pub fn d_gam(s: &DMat) -> DMat {
    let z = DMat::new(2, 2);
    d_blocks(&z, s, &d_bar(s), &z)
}

/// the commutator of two matrices of dual numbers
pub fn d_comm(a: &DMat, b: &DMat) -> DMat {
    a.mul(b).sub(&b.mul(a))
}

/// the trace of a matrix of dual numbers
pub fn d_trace(m: &DMat) -> D4 {
    (0..m.rows).fold(D4::ZERO, |s, i| s + m.get(i, i))
}

/// the largest modulus of the values of the entries of a matrix of dual numbers
pub fn d_max_abs(m: &DMat) -> f64 {
    m.a.iter().fold(0.0, |s, x| s.max(x.v.abs()))
}

/// the largest modulus of the values and of the first derivatives of the entries (a field identity must hold for both)
pub fn d_max_abs1(m: &DMat) -> f64 {
    m.a.iter().fold(0.0, |s, x| {
        let mut t = s.max(x.v.abs());
        for p in 0..4 {
            t = t.max(x.d[p].abs());
        }
        t
    })
}

/// a smooth complex function of the four coordinates at the point x: sum of three terms a_k sin(k_k . x + phi_k) with
/// pseudo-random complex amplitudes of size `amp`, wave vectors of size about 1 and phases (used to build random gauge
/// transformations, gauge fields and test spinors that carry exact derivatives)
pub fn rand_fn(rng: &mut crate::rng::Rng, x: &[f64; 4], amp: f64) -> D4 {
    let mut s = D4::ZERO;
    for _ in 0..3 {
        let a = c(amp * rng.unit(), amp * rng.unit());
        let mut arg = D4::re(std::f64::consts::PI * rng.unit());
        for p in 0..4 {
            arg = arg + D4::coord(x, p) * D4::re(rng.unit());
        }
        s = s + arg.sin().scale(a);
    }
    s
}
