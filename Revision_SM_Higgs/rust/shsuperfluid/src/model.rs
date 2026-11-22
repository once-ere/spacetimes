//! The model of the revised hypotheses, second version, on the sixteen components (Psi_LL: components 0..7, component
//! 2 d + s; chi_SH: components 8..11; chi'_SH: components 12..15): the contact terms of the author's ansatz on Psi_LL,
//! the pairing term (4.2) of the doublet-singlet channels, the three methods as weights of the Wick contractions, the
//! Kohn-Sham densities, and the matrices that describe a state (the pair amplitudes of the four channels of the
//! bidoublet, the densities and masses of the four species up, down, chi and chi').
//!
//! The first version of the hypotheses (one singlet) is the same model with the pairing term of chi'_SH switched off
//! (coupling G'_SH = 0): chi'_SH is then a free field, decoupled from the rest, and the states of the first version are
//! those of stage 3 of the first version with an empty block of chi'_SH (check SF-12 of the program).
use sfcore::c64::C64;
use sfcore::cmat::CMat;
use sfcore::dirac::{alpha, beta, cmat_c, d8, gamma_ch, i8, k_scalar, sigma};
use sfcore::methods::{grid, Method};
use sfcore::uniform_n::{FieldBlock, ModelN, TermKindN, TermN};

/// the mass of Psi_LL and the common mass of the two singlets (decisions D22 and D31: all equal), in units of the
/// cutoff Lambda_LL = 1
pub const M_LL: f64 = 0.2;
pub const M_CHI: f64 = 0.2;
/// the number of radial nodes of the standard grid (as in Part II of the earlier work)
pub const NR: usize = 24;
/// the numbers of components of Psi_LL, of one singlet, of the pair of singlets, and of the whole field
pub const NP: usize = 8;
pub const NX: usize = 4;
pub const NS: usize = 2 * NX;
pub const N: usize = NP + NS;
/// the first components of chi_SH and of chi'_SH
pub const OFF_CHI: usize = NP;
pub const OFF_CHI2: usize = NP + NX;

/// the couplings of a potential of the revised hypotheses: the scalar and the vector contact terms of the author's
/// ansatz (on Psi_LL), the coupling G_SH of the pairing term of chi_SH and the coupling of the pairing term of chi'_SH
/// (equal to G_SH in the second version, decision D31; zero in the first version)
#[derive(Clone, Copy, Debug)]
pub struct PotSh {
    pub gs: f64,
    pub gv: f64,
    pub gsh: f64,
    pub gsh2: f64,
}

/// the three Dirac fields of the revised hypotheses, second version
pub fn blocks() -> Vec<FieldBlock> {
    vec![
        FieldBlock { off: 0, ncomp: NP, mass: M_LL },
        FieldBlock { off: OFF_CHI, ncomp: NX, mass: M_CHI },
        FieldBlock { off: OFF_CHI2, ncomp: NX, mass: M_CHI },
    ]
}

/// an 8 x 8 matrix on Psi_LL embedded into the sixteen components
pub fn emb_psi(x8: &CMat) -> CMat {
    CMat::block_diag(x8, &CMat::zeros(NS))
}
/// a 4 x 4 matrix on chi_SH (which = 0) or on chi'_SH (which = 1) embedded into the sixteen components
pub fn emb_sing(which: usize, x4: &CMat) -> CMat {
    let mut m = CMat::zeros(N);
    let o = OFF_CHI + NX * which;
    m.put(o, o, x4);
    m
}
/// an 8 x 8 matrix on the pair of singlets (chi_SH, chi'_SH) (component 4 a + d of the pair) embedded
pub fn emb_pair(x8: &CMat) -> CMat {
    CMat::block_diag(&CMat::zeros(NP), x8)
}

/// the isospin column e_s (2 x 1): s = 0 up, 1 down
pub fn e_iso(s: usize) -> CMat {
    let mut e = CMat::new(2, 1);
    e.a[s] = C64::ONE;
    e
}
/// the projector onto one isospin component, diag(1, 0) (s = 0) or diag(0, 1) (s = 1)
pub fn p_iso(s: usize) -> CMat {
    let mut p = CMat::zeros(2);
    p.set(s, s, C64::ONE);
    p
}

/// the coefficient matrix V_s = (1/2) C gamma^ch (x) e_s (8 x 4) of the pair operators Pi^s_SH = Psi_LL^T V_s chi_SH and
/// Pi'^s_SH = Psi_LL^T V_s chi'_SH
pub fn v_sh(s: usize) -> CMat {
    cmat_c().mul(&gamma_ch()).kron(&e_iso(s)).scale_r(0.5)
}
/// the 16 x 16 antisymmetric pair matrix of the pair of Psi_LL with the singlet `which` (0: chi_SH, 1: chi'_SH),
/// K = ((0, V_s), (-V_s^T, 0)) in the blocks of the two fields, for which Psi^T K Psi = 2 Pi^s_SH (or 2 Pi'^s_SH) with Psi
/// the whole field; the pairing term -(G/4) :(Psi^T K Psi)^dag (Psi^T K Psi): of the engine is then -G (Pi^s)^dag Pi^s
pub fn k_ds(which: usize, s: usize) -> CMat {
    let v = v_sh(s);
    let o = OFF_CHI + NX * which;
    let mut k = CMat::zeros(N);
    k.put(0, o, &v);
    k.put(o, 0, &v.transpose().scale_r(-1.0));
    k
}
/// the pair matrices of the first pair field, Phi_SH (with chi_SH)
pub fn k_sh(s: usize) -> CMat {
    k_ds(0, s)
}

/// the Kohn-Sham densities of the revised hypotheses (decision D28, extended to the second version): the number and the
/// scalar density of Psi_LL as 2 x 2 matrices in the isospin space (1_4 (x) sigma_a and beta (x) sigma_a, a = 0..3: eight
/// real densities, which rotate among themselves under SU(2)_L), and those of the pair of singlets as 2 x 2 matrices in
/// the space of the two singlets (sigma_a (x) 1_4 and sigma_a (x) beta: eight densities, which rotate among themselves
/// under SU(2)_R); the exchange contraction of KS is projected onto their span, so that the method keeps both symmetries.
/// In the first version the densities of chi'_SH and of the mixing of the two singlets vanish in every state, and the
/// projection is that of the first version.
pub fn ks_basis_species() -> Vec<CMat> {
    let b = beta();
    let mut v = vec![];
    for a in 0..4 {
        v.push(emb_psi(&CMat::identity(4).kron(&sigma(a))));
        v.push(emb_psi(&b.kron(&sigma(a))));
    }
    for a in 0..4 {
        v.push(emb_pair(&sigma(a).kron(&CMat::identity(4))));
        v.push(emb_pair(&sigma(a).kron(&b)));
    }
    v
}
/// the Kohn-Sham densities of the earlier work, summed over the isospin (the span of 1_8 and beta on Psi_LL), with the
/// number and scalar densities of each singlet: used only by check SF-1, where the engine is compared with the engine of
/// the earlier work
pub fn ks_basis_isospin_summed() -> Vec<CMat> {
    let b = beta();
    vec![
        emb_psi(&CMat::identity(8)),
        emb_psi(&d8(&b)),
        emb_sing(0, &CMat::identity(4)),
        emb_sing(0, &b),
        emb_sing(1, &CMat::identity(4)),
        emb_sing(1, &b),
    ]
}

/// the weights (Hartree, exchange, pair) of the scalar and the vector term and (pair, exchange) of the pairing term, and
/// whether the exchange is projected (KS), for a method (Section 9.2 of the earlier work)
pub fn weights(method: &Method) -> ((f64, f64, f64), (f64, f64, f64), (f64, f64), bool) {
    match method {
        Method::Hs { s_s, s_v } => ((*s_s, 0.0, 1.0 - s_s), (*s_v, 0.0, 1.0 - s_v), (1.0, 0.0), false),
        Method::Ks => ((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0), true),
        Method::Cjt => ((1.0, 1.0, 1.0), (1.0, 1.0, 1.0), (1.0, 1.0), false),
    }
}

fn bil(nu: f64, x: &CMat, w: (f64, f64, f64), ks: &Option<Vec<CMat>>, label: &str) -> TermN {
    TermN { kind: TermKindN::Bilinear { nu, x: x.clone(), y: x.clone() }, w_h: w.0, w_f: w.1, w_p: w.2, ks_basis: ks.clone(), label: label.into() }
}

/// the scalar and the vector terms of the author's ansatz on Psi_LL (embedded into the sixteen components)
fn sv_terms(method: &Method, gs: f64, gv: f64, ks: &Option<Vec<CMat>>) -> Vec<TermN> {
    let (ws, wv, _, _) = weights(method);
    let mut t = vec![];
    if gs != 0.0 {
        t.push(bil(0.5 * gs, &emb_psi(&d8(&beta())), ws, ks, "scalar"));
    }
    if gv != 0.0 {
        t.push(bil(0.5 * gv, &emb_psi(&CMat::identity(8)), wv, ks, "vector, time component"));
        for j in 1..4 {
            t.push(bil(-0.5 * gv, &emb_psi(&d8(&alpha(j))), wv, ks, &format!("vector, space component {}", j)));
        }
    }
    t
}

/// the contact terms of a potential of the revised hypotheses with the weights of a method: the scalar and the vector
/// term on Psi_LL, and the pairing terms (4.2) of the doublet-singlet channels,
/// -G_SH sum_s (Pi^s_SH)^dag Pi^s_SH - G'_SH sum_s (Pi'^s_SH)^dag Pi'^s_SH
pub fn terms_sh(method: &Method, pot: &PotSh) -> Vec<TermN> {
    let (_, _, wpair, ks) = weights(method);
    let basis = if ks { Some(ks_basis_species()) } else { None };
    let mut t = sv_terms(method, pot.gs, pot.gv, &basis);
    for (which, g) in [(0usize, pot.gsh), (1usize, pot.gsh2)] {
        if g != 0.0 {
            for s in 0..2 {
                t.push(TermN {
                    kind: TermKindN::Pairing { g, k: k_ds(which, s) },
                    w_h: 0.0,
                    w_f: wpair.1,
                    w_p: wpair.0,
                    ks_basis: basis.clone(),
                    label: format!("pairing {} with {}", if s == 0 { "up" } else { "down" }, if which == 0 { "chi" } else { "chi'" }),
                });
            }
        }
    }
    t
}

/// the contact terms of the earlier potential (g: the coupling of the three triplet pairing terms of Psi_LL) embedded
/// into the sixteen components, with the Kohn-Sham densities summed over the isospin: the model of the earlier work with
/// two free, decoupled singlet fields (check SF-1)
pub fn terms_earlier(method: &Method, gs: f64, gv: f64, g: f64) -> Vec<TermN> {
    let (_, _, wpair, ks) = weights(method);
    let basis = if ks { Some(ks_basis_isospin_summed()) } else { None };
    let mut t = sv_terms(method, gs, gv, &basis);
    if g != 0.0 {
        for i in 1..4 {
            t.push(TermN {
                kind: TermKindN::Pairing { g, k: emb_psi(&k_scalar(i)) },
                w_h: 0.0,
                w_f: wpair.1,
                w_p: wpair.0,
                ks_basis: basis.clone(),
                label: format!("pairing {}", i),
            });
        }
    }
    t
}

/// the model of a potential and a method on the standard grid of nr radial nodes
pub fn model(method: &Method, pot: &PotSh, nr: usize) -> ModelN {
    ModelN::new(blocks(), 0.0, terms_sh(method, pot), grid(nr))
}

/// the components of the four species: up (Psi_LL, s = 0), down (Psi_LL, s = 1), chi, chi'
pub fn species_indices(sp: usize) -> Vec<usize> {
    match sp {
        0 => (0..NP).filter(|i| i % 2 == 0).collect(),
        1 => (0..NP).filter(|i| i % 2 == 1).collect(),
        2 => (OFF_CHI..OFF_CHI2).collect(),
        _ => (OFF_CHI2..N).collect(),
    }
}
pub fn psi_indices() -> Vec<usize> {
    (0..NP).collect()
}
/// the components of the pair of singlets (chi_SH and chi'_SH)
pub fn sing_indices() -> Vec<usize> {
    (NP..N).collect()
}

/// the number projector and the beta matrix of a species (16 x 16)
pub fn species_mats(sp: usize) -> (CMat, CMat) {
    let b = beta();
    match sp {
        0 | 1 => (emb_psi(&i8(&p_iso(sp))), emb_psi(&b.kron(&p_iso(sp)))),
        2 => (emb_sing(0, &CMat::identity(4)), emb_sing(0, &b)),
        _ => (emb_sing(1, &CMat::identity(4)), emb_sing(1, &b)),
    }
}

/// the description of a state: the pair self-energy coefficients of the four channels of the bidoublet,
/// m[s][a] = the coefficient of K(a, s)^dag in P (s = up, down; a = chi, chi'), the norm of the rest of P, the norms of the
/// blocks of P and N that are charged (pairs Psi Psi and of two singlets, mixing Psi^dag chi, mixing chi^dag chi'), the
/// difference of the two isospin components in the normal self-energy, the Dirac masses and the scalar and number
/// densities of the four species
#[derive(Clone, Debug)]
pub struct Describe {
    pub m: [[C64; 2]; 2],
    pub p_rest: f64,
    pub p_psipsi: f64,
    pub p_ss: f64,
    pub n_psis: f64,
    pub n_chichi2: f64,
    pub n_updown: f64,
    pub mass: [f64; 4],
    pub ns: [f64; 4],
    pub n: [f64; 4],
}

impl Describe {
    /// the two singular values of the bidoublet of pair coefficients (descending): equal for a vacuum that keeps the
    /// custodial SU(2)_V, the second zero for a condensate of one doublet only
    pub fn singular(&self) -> (f64, f64) {
        let mm = CMat::m2(self.m[0][0], self.m[0][1], self.m[1][0], self.m[1][1]);
        let (ev, _) = sfcore::cmat::eigh(&mm.dagger().mul(&mm));
        (ev[1].max(0.0).sqrt(), ev[0].max(0.0).sqrt())
    }
}

pub fn describe(nself: &CMat, pself: &CMat, drho: &CMat) -> Describe {
    // m_sa = <K(a, s)^dag, P> / <K(a, s)^dag, K(a, s)^dag> (the four matrices are orthogonal)
    let mut m = [[C64::ZERO; 2]; 2];
    let mut rest = pself.clone();
    for s in 0..2 {
        for a in 0..2 {
            let kd = k_ds(a, s).dagger();
            let nrm = kd.dagger().mul(&kd).trace().re;
            m[s][a] = kd.dagger().mul(pself).trace().scale(1.0 / nrm);
            rest = rest.sub(&kd.scale(m[s][a]));
        }
    }
    let blocknorm = |x: &CMat, a: &[usize], b: &[usize]| -> f64 {
        let mut s = 0.0;
        for &i in a {
            for &j in b {
                s += x.get(i, j).norm_sqr();
            }
        }
        s.sqrt()
    };
    let (ps, ss) = (psi_indices(), sing_indices());
    let (up, dn, c1, c2) = (species_indices(0), species_indices(1), species_indices(2), species_indices(3));
    let mut mass = [0.0; 4];
    let mut ns = [0.0; 4];
    let mut n = [0.0; 4];
    for sp in 0..4 {
        let (pn, pb) = species_mats(sp);
        let m0 = if sp < 2 { M_LL } else { M_CHI };
        mass[sp] = m0 + pb.mul(nself).trace().re / 4.0;
        ns[sp] = pb.mul(drho).trace().re;
        n[sp] = pn.mul(drho).trace().re;
    }
    Describe {
        m,
        p_rest: rest.fro(),
        p_psipsi: blocknorm(pself, &ps, &ps),
        p_ss: blocknorm(pself, &ss, &ss),
        n_psis: blocknorm(nself, &ps, &ss),
        n_chichi2: blocknorm(nself, &c1, &c2),
        n_updown: blocknorm(nself, &up, &dn),
        mass,
        ns,
        n,
    }
}

/// the Pauli matrix sigma_3 on the isospin index, embedded (used for the isospin-polarized start)
pub fn tau3_beta() -> CMat {
    emb_psi(&beta().kron(&sigma(3)))
}
/// sigma_3 in the space of the two singlets times beta, embedded (used for the start that polarizes the singlets)
pub fn tau3r_beta() -> CMat {
    emb_pair(&sigma(3).kron(&beta()))
}
