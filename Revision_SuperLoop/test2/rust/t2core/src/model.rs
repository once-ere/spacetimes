//! The super-loop operators of Test 2 as matrices.
//!
//! Every matrix of the field equations is built here from the bi-quaternion matrix M_LL of the
//! super-loop, so that the programs and the document use one set of definitions:
//!   * sigma_p (p = 0..3) and eps2 = i sigma_2 = M_LL(-e2);
//!   * the frame of R^(1,3)_LL: four Hermitian bi-quaternions f_p (canonically e0, i e1, i e2, i e3),
//!     and the Dirac matrices gamma_LL^p[f] = [[0, M_LL(f_p)], [M_LL(conj f_p), 0]] (chiral basis);
//!   * the Lorentz generators of the super-loop, J_LL^j = e_j/2 and K_LL^j = i e_j/2, on the Dirac
//!     spinor: sgen_LL(uL) = diag(-M_LL(uL)^dag, M_LL(uL)) (the lower block carries the spinor charges);
//!   * the spinor representation of a unit bi-quaternion gL: Sspin_LL(gL) = diag((M_LL(gL)^dag)^-1, M_LL(gL));
//!   * the super-loop form of the Dirac isodoublet: the Weyl isodoublet psi'_LL(y) = eps2^-1 M_LL(y)^T of a
//!     spinor charge y e4, and the bi-octonion (Cayley-Dickson) product of two bi-octonions a + b e4;
//!   * isospin T_LL^iota = 1_4 (x) tau_iota/2, the pair matrices Kch_iota = (C gamma_ch) (x) (eps2 tau_iota)
//!     and Kch'_iota = C (x) (eps2 tau_iota).
use crate::c64::{c, C64};
use crate::cmat::CMat;
use crate::rng::Rng;

/// the Pauli matrices: sigma(0) = unit matrix, sigma(1), sigma(2), sigma(3)
pub fn sigma(p: usize) -> CMat {
    let (o, z, i) = (C64::ONE, C64::ZERO, C64::I);
    match p {
        0 => CMat::m2(o, z, z, o),
        1 => CMat::m2(z, o, o, z),
        2 => CMat::m2(z, -i, i, z),
        3 => CMat::m2(o, z, z, -o),
        _ => panic!("sigma index"),
    }
}

/// eps2 = i sigma_2 = [[0, 1], [-1, 0]]
pub fn eps2() -> CMat {
    CMat::m2(C64::ZERO, C64::ONE, -C64::ONE, C64::ZERO)
}

/// M_LL(b) for the bi-quaternion b = sum_p xi_p e_p
pub fn mq(xi: &[C64; 4]) -> CMat {
    let mut m = sigma(0).scale(xi[0]);
    for j in 1..4 {
        m = m.add(&sigma(j).scale(-C64::I * xi[j]));
    }
    m
}

/// the conjugate bi-quaternion: the signs of the coefficients of e1, e2, e3 change
pub fn bq_conj(xi: &[C64; 4]) -> [C64; 4] {
    [xi[0], -xi[1], -xi[2], -xi[3]]
}

/// the canonical frame of R^(1,3)_LL: f_0 = e0, f_j = i e_j
pub fn canonical_frame() -> [[C64; 4]; 4] {
    let mut f = [[C64::ZERO; 4]; 4];
    f[0][0] = C64::ONE;
    for j in 1..4 {
        f[j][j] = C64::I;
    }
    f
}

/// the Dirac matrices built from a frame f_p: gamma^p[f] = [[0, M_LL(f_p)], [M_LL(conj f_p), 0]]
pub fn gammas_from_frame(f: &[[C64; 4]; 4]) -> Vec<CMat> {
    (0..4)
        .map(|p| {
            let z = CMat::zeros(2);
            CMat::blocks(&z, &mq(&f[p]), &mq(&bq_conj(&f[p])), &z)
        })
        .collect()
}

/// gamma_LL^p (p = 0..3) of the canonical frame (the chiral basis of the manuscript)
pub fn gamma(p: usize) -> CMat {
    gammas_from_frame(&canonical_frame())[p].clone()
}

/// the chirality matrix gamma_ch = i gamma^0 gamma^1 gamma^2 gamma^3
pub fn gamma_ch() -> CMat {
    gamma(0).mul(&gamma(1)).mul(&gamma(2)).mul(&gamma(3)).scale(C64::I)
}

/// the charge-conjugation matrix C_LL = i gamma^2 gamma^0
pub fn cconj() -> CMat {
    gamma(2).mul(&gamma(0)).scale(C64::I)
}

/// alpha^j = gamma^0 gamma^j
pub fn alpha(j: usize) -> CMat {
    gamma(0).mul(&gamma(j))
}

/// beta = gamma^0
pub fn beta() -> CMat {
    gamma(0)
}

/// the coefficients of the rotation generator J_LL^j = e_j/2
pub fn jgen(j: usize) -> [C64; 4] {
    let mut x = [C64::ZERO; 4];
    x[j] = c(0.5, 0.0);
    x
}

/// the coefficients of the boost generator K_LL^j = i e_j/2
pub fn kgen(j: usize) -> [C64; 4] {
    let mut x = [C64::ZERO; 4];
    x[j] = c(0.0, 0.5);
    x
}

/// a Lorentz generator uL (an imaginary bi-quaternion) on the Dirac spinor:
/// S(uL) = diag(-M_LL(uL)^dag, M_LL(uL))
pub fn spin_gen(u: &[C64; 4]) -> CMat {
    let m = mq(u);
    CMat::block_diag(&m.dagger().scale_r(-1.0), &m)
}

/// the spinor representation of a unit bi-quaternion with the 2 x 2 matrix mg = M_LL(gL):
/// S_LL(gL) = diag((mg^dag)^-1, mg)  (4 x 4)
pub fn spin_rep(mg: &CMat) -> CMat {
    CMat::block_diag(&mg.dagger().inv2(), mg)
}

/// coordinates x^p = tr(sigma_p X)/2 of a Hermitian 2 x 2 matrix X
pub fn coords(x: &CMat) -> [f64; 4] {
    let mut r = [0.0; 4];
    for p in 0..4 {
        r[p] = 0.5 * sigma(p).mul(x).trace().re;
    }
    r
}

/// the real 4 x 4 Lorentz matrix Lmat(gL): entry (p, q) = Re tr(sigma_p mg sigma_q mg^dag)/2
pub fn lmat(mg: &CMat) -> [[f64; 4]; 4] {
    let mut l = [[0.0; 4]; 4];
    for q in 0..4 {
        let y = mg.mul(&sigma(q)).mul(&mg.dagger());
        let x = coords(&y);
        for p in 0..4 {
            l[p][q] = x[p];
        }
    }
    l
}

/// the vector action V_LL(uL): the real 4 x 4 matrix of M(x) -> M(uL) M(x) + M(x) M(uL)^dag
pub fn vec_action(u: &[C64; 4]) -> [[f64; 4]; 4] {
    let mu = mq(u);
    let mut v = [[0.0; 4]; 4];
    for q in 0..4 {
        let y = mu.mul(&sigma(q)).add(&sigma(q).mul(&mu.dagger()));
        let x = coords(&y);
        for p in 0..4 {
            v[p][q] = x[p];
        }
    }
    v
}

/// a 4 x 4 Dirac matrix lifted to the Dirac isodoublet: A (x) 1_2
pub fn lift4(a: &CMat) -> CMat {
    a.kron(&CMat::identity(2))
}

/// an isospin 2 x 2 matrix lifted to the Dirac isodoublet: 1_4 (x) B
pub fn lift2(b: &CMat) -> CMat {
    CMat::identity(4).kron(b)
}

/// the isospin generators T_LL^iota = 1_4 (x) tau_iota/2 (iota = 1, 2, 3), 8 x 8
pub fn tiso(iota: usize) -> CMat {
    lift2(&sigma(iota).scale_r(0.5))
}

/// the scalar pair matrices Kch_iota = (C_LL gamma_ch) (x) (eps2 tau_iota), 8 x 8
pub fn kpair(iota: usize) -> CMat {
    cconj().mul(&gamma_ch()).kron(&eps2().mul(&sigma(iota)))
}

/// the pseudoscalar pair matrices Kch'_iota = C_LL (x) (eps2 tau_iota), 8 x 8
pub fn kpair_ps(iota: usize) -> CMat {
    cconj().kron(&eps2().mul(&sigma(iota)))
}

/// the pair potential P_LL = sum_iota d^iota Kch_iota^dag (8 x 8)
pub fn pair_potential(d: &[C64; 3]) -> CMat {
    let mut p = CMat::zeros(8);
    for i in 0..3 {
        p = p.add(&kpair(i + 1).dagger().scale(d[i]));
    }
    p
}

/// a random unit bi-quaternion, as its matrix M_LL(gL) in SL(2,C)
pub fn rand_sl2c(rng: &mut Rng) -> CMat {
    loop {
        let m = CMat::m2(c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()));
        let d = m.det2();
        if d.abs() > 0.1 {
            return m.scale(C64::ONE / d.sqrt());
        }
    }
}

/// the SU(2) matrix exp(i sum_iota th_iota tau_iota/2)
pub fn su2(th: &[f64; 3]) -> CMat {
    let mut x = CMat::zeros(2);
    for i in 0..3 {
        x = x.add(&sigma(i + 1).scale(c(0.0, 0.5 * th[i])));
    }
    x.expm()
}

/// the real rotation R(U) of the vector representation: U tau_iota U^dag = sum_kappa tau_kappa R[kappa][iota]
pub fn adjoint(u: &CMat) -> [[f64; 3]; 3] {
    let mut r = [[0.0; 3]; 3];
    for i in 0..3 {
        let y = u.mul(&sigma(i + 1)).mul(&u.dagger());
        for k in 0..3 {
            r[k][i] = 0.5 * sigma(k + 1).mul(&y).trace().re;
        }
    }
    r
}

/// the Levi-Civita symbol eps3 for indices 0, 1, 2
pub fn eps3(i: usize, j: usize, k: usize) -> f64 {
    if i == j || j == k || i == k {
        return 0.0;
    }
    let perm = [(0, 1, 2), (1, 2, 0), (2, 0, 1)];
    if perm.contains(&(i, j, k)) {
        1.0
    } else {
        -1.0
    }
}

/// the metric diag(1, -1, -1, -1)
pub fn eta(p: usize) -> f64 {
    if p == 0 {
        1.0
    } else {
        -1.0
    }
}

// ------------------------------------------------------------------ the super-loop form (check GS-14)

/// the coefficients xi_0..xi_3 of the bi-quaternion b with M_LL(b) = m (M_LL is one-to-one):
/// m = xi_0 sigma_0 - i sum_j xi_j sigma_j, so xi_0 = (m00 + m11)/2, xi_1 = i (m01 + m10)/2,
/// xi_2 = (m10 - m01)/2, xi_3 = i (m00 - m11)/2
pub fn bq_from(m: &CMat) -> [C64; 4] {
    let half = c(0.5, 0.0);
    let ih = c(0.0, 0.5);
    [
        (m.get(0, 0) + m.get(1, 1)) * half,
        (m.get(0, 1) + m.get(1, 0)) * ih,
        (m.get(1, 0) - m.get(0, 1)) * half,
        (m.get(0, 0) - m.get(1, 1)) * ih,
    ]
}

/// the product of two bi-quaternions (through M_LL, which is multiplicative)
pub fn bq_mul(x: &[C64; 4], y: &[C64; 4]) -> [C64; 4] {
    bq_from(&mq(x).mul(&mq(y)))
}

/// the complex conjugate of a bi-quaternion: every coefficient is replaced by its complex conjugate
pub fn bq_star(x: &[C64; 4]) -> [C64; 4] {
    [x[0].conj(), x[1].conj(), x[2].conj(), x[3].conj()]
}

/// the Hermitian conjugate x^dag = (conj x)^*
pub fn bq_dag(x: &[C64; 4]) -> [C64; 4] {
    bq_star(&bq_conj(x))
}

/// the Weyl isodoublet of the spinor charge y e4: psi'_LL(y) = eps2^-1 M_LL(y)^T, a 2 x 2 matrix whose
/// row is the Weyl index and whose column is the isospin index (up, down)
pub fn psi_prime(y: &[C64; 4]) -> CMat {
    eps2().inv2().mul(&mq(y).transpose())
}

/// the Dirac isodoublet built from two spinor charges: the upper (gamma_ch = -1) block is psi'_LL(y),
/// the lower (gamma_ch = +1) block is psi'_LL(b); component 2(a-1)+alpha holds the Dirac index a and the
/// isospin index alpha, so components 1..4 are the rows of psi'_LL(y) and 5..8 the rows of psi'_LL(b)
pub fn dirac_from_charges(y: &[C64; 4], b: &[C64; 4]) -> Vec<C64> {
    let py = psi_prime(y);
    let pb = psi_prime(b);
    let mut v = Vec::with_capacity(8);
    for w in 0..2 {
        for a in 0..2 {
            v.push(py.get(w, a));
        }
    }
    for w in 0..2 {
        for a in 0..2 {
            v.push(pb.get(w, a));
        }
    }
    v
}

/// the Cayley-Dickson product of two bi-octonions x = a + b e4 and y = cq + d e4 (a, b, cq, d bi-quaternions):
/// (a + b e4)(cq + d e4) = (a cq - conj(d) b) + (d a + b conj(cq)) e4, the rule of the folder Revision_SuperLoop
pub fn bioct_mul(x: &([C64; 4], [C64; 4]), y: &([C64; 4], [C64; 4])) -> ([C64; 4], [C64; 4]) {
    let (a, b) = x;
    let (cq, d) = y;
    let ac = bq_mul(a, cq);
    let db = bq_mul(&bq_conj(d), b);
    let da = bq_mul(d, a);
    let bc = bq_mul(b, &bq_conj(cq));
    let mut first = [C64::ZERO; 4];
    let mut second = [C64::ZERO; 4];
    for k in 0..4 {
        first[k] = ac[k] - db[k];
        second[k] = da[k] + bc[k];
    }
    (first, second)
}
