//! The matrices of the spinor field: Pauli matrices, the Dirac matrices of the chiral basis, the 8x8 matrices of the
//! Dirac isodoublet Psi_LL, the generators of GEE, the pair matrices and the free Hamiltonian.
//!
//! Index convention of the eight components of Psi_LL (as in superfluid.md): component number (counted from 0)
//! = 2 d + s, with the Dirac index d = 0..3 (d = 0, 1: chirality -1, the block Psi^-; d = 2, 3: chirality +1, the block
//! Psi^+) and the isospin index s = 0 (up), 1 (down).  A Dirac matrix A (4x4) acts on Psi_LL as the Kronecker product
//! A (x) 1_2 (function `d8`), an isospin matrix t (2x2) as 1_4 (x) t (function `i8`).
//!
//! (Copied unchanged in its code from the library sfcore of the folder Revision_SM_Higgs of PROJECT spacetimes,
//! file rust/sfcore/src/dirac.rs; only this note is added.)
use crate::c64::{c, C64};
use crate::cmat::CMat;

/// the 2x2 unit matrix sigma_0 (p = 0) and the Pauli matrices sigma_1, sigma_2, sigma_3 (p = 1, 2, 3)
pub fn sigma(p: usize) -> CMat {
    let (o, z, i) = (C64::ONE, C64::ZERO, C64::I);
    match p {
        0 => CMat::m2(o, z, z, o),
        1 => CMat::m2(z, o, o, z),
        2 => CMat::m2(z, -i, i, z),
        3 => CMat::m2(o, z, z, -o),
        _ => panic!("sigma: index {} out of range", p),
    }
}
/// sigma-bar: sigma_0 for p = 0 and -sigma_j for p = j
pub fn sigma_bar(p: usize) -> CMat {
    if p == 0 {
        sigma(0)
    } else {
        sigma(p).scale_r(-1.0)
    }
}
/// E = i sigma_2 = ((0, 1), (-1, 0)): E^{-1} = E^T = -E
pub fn eps2() -> CMat {
    CMat::m2(C64::ZERO, C64::ONE, -C64::ONE, C64::ZERO)
}
/// the Levi-Civita symbol of three indices in 0..2 (for the indices 1..3 of the text, shifted by one)
pub fn levi(a: usize, b: usize, cc: usize) -> f64 {
    if a == b || b == cc || a == cc {
        return 0.0;
    }
    if (a, b, cc) == (0, 1, 2) || (a, b, cc) == (1, 2, 0) || (a, b, cc) == (2, 0, 1) {
        1.0
    } else {
        -1.0
    }
}

/// the Dirac matrices of the canonical frame in the chiral basis: gamma^0 = ((0, 1), (1, 0)),
/// gamma^j = ((0, sigma_j), (-sigma_j, 0)) in 2x2 blocks
pub fn gamma(p: usize) -> CMat {
    let z = CMat::zeros(2);
    CMat::blocks(&z, &sigma(p), &sigma_bar(p), &z)
}
/// the chirality matrix i gamma^0 gamma^1 gamma^2 gamma^3 (= diag(-1, -1, 1, 1), check A of the program sf_part1)
pub fn gamma_ch() -> CMat {
    gamma(0).mul(&gamma(1)).mul(&gamma(2)).mul(&gamma(3)).scale(C64::I)
}
/// the charge-conjugation matrix C = i gamma^2 gamma^0 (= diag(E, -E))
pub fn cmat_c() -> CMat {
    gamma(2).mul(&gamma(0)).scale(C64::I)
}
/// the Dirac matrices of a frame f_p from the matrices M(f_p) and M(fbar_p): ((0, M(f_p)), (M(fbar_p), 0))
pub fn gamma_frame(mf: &CMat, mfbar: &CMat) -> CMat {
    let z = CMat::zeros(2);
    CMat::blocks(&z, mf, mfbar, &z)
}
/// alpha^j = gamma^0 gamma^j (4x4)
pub fn alpha(j: usize) -> CMat {
    gamma(0).mul(&gamma(j))
}
/// beta = gamma^0 (4x4)
pub fn beta() -> CMat {
    gamma(0)
}
/// the 16 Dirac matrices: 1, gamma^p, sigma^{p p'} (p < p'), gamma^p gamma^ch, gamma^ch, with their names
pub fn dirac_basis16() -> Vec<(String, CMat)> {
    let mut v: Vec<(String, CMat)> = vec![("1".into(), CMat::identity(4))];
    for p in 0..4 {
        v.push((format!("gamma^{}", p), gamma(p)));
    }
    for p in 0..4 {
        for q in (p + 1)..4 {
            let s = gamma(p).mul(&gamma(q)).sub(&gamma(q).mul(&gamma(p))).scale(c(0.0, 0.5));
            v.push((format!("sigma^{}{}", p, q), s));
        }
    }
    for p in 0..4 {
        v.push((format!("gamma^{} gamma^ch", p), gamma(p).mul(&gamma_ch())));
    }
    v.push(("gamma^ch".into(), gamma_ch()));
    v
}

/// a Dirac matrix on the eight components: A (x) 1_2
pub fn d8(a4: &CMat) -> CMat {
    a4.kron(&CMat::identity(2))
}
/// an isospin matrix on the eight components: 1_4 (x) t
pub fn i8(t2: &CMat) -> CMat {
    CMat::identity(4).kron(t2)
}
/// the isospin generators T^iota = 1_4 (x) tau_iota / 2 (iota = 1, 2, 3)
pub fn t_gen(iota: usize) -> CMat {
    i8(&sigma(iota).scale_r(0.5))
}
/// the generators of SU(2)_v on an isospin vector (adjoint representation): (t^iota)_{a b} = -i eps_{iota a b}
/// (iota = 1, 2, 3; a, b = 0..2)
pub fn adj_gen(iota: usize) -> CMat {
    let mut m = CMat::zeros(3);
    for a in 0..3 {
        for b in 0..3 {
            m.set(a, b, c(0.0, -levi(iota - 1, a, b)));
        }
    }
    m
}
/// the general pair matrix K[Gamma, t] = C Gamma (x) E t on the eight components
pub fn k_general(gamma4: &CMat, t2: &CMat) -> CMat {
    cmat_c().mul(gamma4).kron(&eps2().mul(t2))
}
/// the scalar (0+) pair matrices K^iota = C gamma^ch (x) E tau_iota
pub fn k_scalar(iota: usize) -> CMat {
    k_general(&gamma_ch(), &sigma(iota))
}
/// the pseudoscalar (0-) pair matrices K'^iota = C (x) E tau_iota
pub fn k_pseudo(iota: usize) -> CMat {
    k_general(&CMat::identity(4), &sigma(iota))
}
/// the spinor matrix of a unit bi-quaternion, S(g) = diag(M(gtilde), M(g)) with M(gtilde) = (M(g)^dag)^{-1}
pub fn s_spin(mg: &CMat) -> CMat {
    CMat::block_diag(&mg.dagger().inv2(), mg)
}
/// the action of a Lorentz generator on the Dirac index, s(u) = diag(-M(u)^dag, M(u))
pub fn s_gen(mu: &CMat) -> CMat {
    CMat::block_diag(&mu.dagger().scale_r(-1.0), mu)
}
/// the free Hamiltonian on a plane wave of momentum q, h(q) = sum_j alpha^j q_j + beta m, on the eight components
pub fn h_free(q: &[f64; 3], m: f64) -> CMat {
    let mut h = beta().scale_r(m);
    for j in 0..3 {
        h = h.add(&alpha(j + 1).scale_r(q[j]));
    }
    d8(&h)
}
