//! The model of the revised hypotheses on the twelve components (Psi_LL: components 0..7, component 2 d + s; chi_SH:
//! components 8..11): the contact terms of the author's ansatz on Psi_LL, the pairing term (4.2) of the doublet-singlet
//! channel, the three methods as weights of the Wick contractions, the Kohn-Sham densities, and the matrices that
//! describe a state (the pair amplitudes of the two channels of the Higgs doublet, the densities and masses of the three
//! species up, down and chi).
use sfcore::c64::C64;
use sfcore::cmat::CMat;
use sfcore::dirac::{alpha, beta, cmat_c, d8, gamma_ch, i8, k_scalar, sigma};
use sfcore::methods::{grid, Method};
use sfcore::uniform_n::{FieldBlock, ModelN, TermKindN, TermN};

/// the mass of Psi_LL and the mass of chi_SH (decision D22: equal), in units of the cutoff Lambda_LL = 1
pub const M_LL: f64 = 0.2;
pub const M_CHI: f64 = 0.2;
/// the number of radial nodes of the standard grid (as in Part II of the earlier work)
pub const NR: usize = 24;
pub const NP: usize = 8;
pub const NX: usize = 4;
pub const N: usize = 12;

/// the couplings of a potential of the revised hypotheses: the scalar and the vector contact terms of the author's
/// ansatz (on Psi_LL) and the coupling G_SH of the pairing term (4.2)
#[derive(Clone, Copy, Debug)]
pub struct PotSh {
    pub gs: f64,
    pub gv: f64,
    pub gsh: f64,
}

/// the two Dirac fields of the revised hypotheses
pub fn blocks() -> Vec<FieldBlock> {
    vec![FieldBlock { off: 0, ncomp: NP, mass: M_LL }, FieldBlock { off: NP, ncomp: NX, mass: M_CHI }]
}

/// an 8 x 8 matrix on Psi_LL embedded into the twelve components
pub fn emb_psi(x8: &CMat) -> CMat {
    CMat::block_diag(x8, &CMat::zeros(NX))
}
/// a 4 x 4 matrix on chi_SH embedded into the twelve components
pub fn emb_chi(x4: &CMat) -> CMat {
    CMat::block_diag(&CMat::zeros(NP), x4)
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

/// the coefficient matrix V_s = (1/2) C gamma^ch (x) e_s (8 x 4) of the pair operator Pi^s_SH = Psi_LL^T V_s chi_SH
pub fn v_sh(s: usize) -> CMat {
    cmat_c().mul(&gamma_ch()).kron(&e_iso(s)).scale_r(0.5)
}
/// the 12 x 12 antisymmetric pair matrix K_s = ((0, V_s), (-V_s^T, 0)), for which Psi^T K_s Psi = 2 Pi^s_SH (Psi the
/// whole field); the pairing term -(G/4) :(Psi^T K_s Psi)^dag (Psi^T K_s Psi): of the engine is then -G (Pi^s)^dag Pi^s
pub fn k_sh(s: usize) -> CMat {
    let v = v_sh(s);
    let mut k = CMat::zeros(N);
    k.put(0, NP, &v);
    k.put(NP, 0, &v.transpose().scale_r(-1.0));
    k
}

/// the Kohn-Sham densities of the revised hypotheses (decision D28): the number density and the scalar density of
/// Psi_LL as 2 x 2 matrices in the isospin space (1_4 (x) sigma_a and beta (x) sigma_a, a = 0, 1, 2, 3: eight real
/// densities, which rotate among themselves under SU(2)_L, so that the method keeps the symmetry), and the number and the
/// scalar density of chi_SH; the exchange contraction of KS is projected onto their span.  The condensate treats the up
/// and the down components differently, so the densities summed over the isospin (the Kohn-Sham densities of the earlier
/// work) would not suffice.
pub fn ks_basis_species() -> Vec<CMat> {
    let b = beta();
    let mut v = vec![];
    for a in 0..4 {
        v.push(emb_psi(&CMat::identity(4).kron(&sigma(a))));
        v.push(emb_psi(&b.kron(&sigma(a))));
    }
    v.push(emb_chi(&CMat::identity(4)));
    v.push(emb_chi(&b));
    v
}
/// the Kohn-Sham densities of the earlier work, summed over the isospin (the span of 1_8 and beta on Psi_LL), with those
/// of chi_SH: used only by check SF-1, where the engine is compared with the engine of the earlier work
pub fn ks_basis_isospin_summed() -> Vec<CMat> {
    let b = beta();
    vec![emb_psi(&CMat::identity(8)), emb_psi(&d8(&b)), emb_chi(&CMat::identity(4)), emb_chi(&b)]
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

/// the scalar and the vector terms of the author's ansatz on Psi_LL (embedded into the twelve components)
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
/// term on Psi_LL, and the pairing term (4.2) of the doublet-singlet channel, -G_SH sum_s (Pi^s_SH)^dag Pi^s_SH
pub fn terms_sh(method: &Method, pot: &PotSh) -> Vec<TermN> {
    let (_, _, wpair, ks) = weights(method);
    let basis = if ks { Some(ks_basis_species()) } else { None };
    let mut t = sv_terms(method, pot.gs, pot.gv, &basis);
    if pot.gsh != 0.0 {
        for s in 0..2 {
            t.push(TermN {
                kind: TermKindN::Pairing { g: pot.gsh, k: k_sh(s) },
                w_h: 0.0,
                w_f: wpair.1,
                w_p: wpair.0,
                ks_basis: basis.clone(),
                label: format!("pairing {}", if s == 0 { "up" } else { "down" }),
            });
        }
    }
    t
}

/// the contact terms of the earlier potential (g: the coupling of the three triplet pairing terms of Psi_LL) embedded
/// into the twelve components, with the Kohn-Sham densities summed over the isospin: the model of the earlier work with a
/// free, decoupled singlet field (check SF-1)
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

/// the components of the three species: up (Psi_LL, s = 0), down (Psi_LL, s = 1), chi
pub fn species_indices(sp: usize) -> Vec<usize> {
    match sp {
        0 => (0..NP).filter(|i| i % 2 == 0).collect(),
        1 => (0..NP).filter(|i| i % 2 == 1).collect(),
        _ => (NP..N).collect(),
    }
}
pub fn psi_indices() -> Vec<usize> {
    (0..NP).collect()
}
pub fn chi_indices() -> Vec<usize> {
    (NP..N).collect()
}

/// the number projector and the beta matrix of a species (12 x 12)
pub fn species_mats(sp: usize) -> (CMat, CMat) {
    let b = beta();
    match sp {
        0 | 1 => (emb_psi(&i8(&p_iso(sp))), emb_psi(&b.kron(&p_iso(sp)))),
        _ => (emb_chi(&CMat::identity(4)), emb_chi(&b)),
    }
}

/// the description of a state: the pair self-energy coefficients D_s of the two components of the Higgs channel
/// (P = sum_s D_s K_s^dag + rest), the norm of the rest of P, the norms of the blocks of P and N that carry a relative
/// number (pairs Psi Psi and chi chi, mixing Psi^dag chi), the Dirac masses and the scalar and number densities of the
/// three species
#[derive(Clone, Debug)]
pub struct Describe {
    pub d: [C64; 2],
    pub p_rest: f64,
    pub p_psipsi: f64,
    pub p_chichi: f64,
    pub n_psichi: f64,
    pub n_updown: f64,
    pub mass: [f64; 3],
    pub ns: [f64; 3],
    pub n: [f64; 3],
}

pub fn describe(nself: &CMat, pself: &CMat, drho: &CMat) -> Describe {
    // D_s = <K_s^dag, P> / <K_s^dag, K_s^dag> (the K_s^dag are orthogonal)
    let mut d = [C64::ZERO; 2];
    let mut rest = pself.clone();
    for s in 0..2 {
        let kd = k_sh(s).dagger();
        let nrm = kd.dagger().mul(&kd).trace().re;
        d[s] = kd.dagger().mul(pself).trace().scale(1.0 / nrm);
        rest = rest.sub(&kd.scale(d[s]));
    }
    let blocknorm = |m: &CMat, a: &[usize], b: &[usize]| -> f64 {
        let mut s = 0.0;
        for &i in a {
            for &j in b {
                s += m.get(i, j).norm_sqr();
            }
        }
        s.sqrt()
    };
    let (ps, cs) = (psi_indices(), chi_indices());
    let (up, dn) = (species_indices(0), species_indices(1));
    let mut mass = [0.0; 3];
    let mut ns = [0.0; 3];
    let mut n = [0.0; 3];
    for sp in 0..3 {
        let (pn, pb) = species_mats(sp);
        let m0 = if sp < 2 { M_LL } else { M_CHI };
        mass[sp] = m0 + pb.mul(nself).trace().re / 4.0;
        ns[sp] = pb.mul(drho).trace().re;
        n[sp] = pn.mul(drho).trace().re;
    }
    Describe {
        d,
        p_rest: rest.fro(),
        p_psipsi: blocknorm(pself, &ps, &ps),
        p_chichi: blocknorm(pself, &cs, &cs),
        n_psichi: blocknorm(nself, &ps, &cs),
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
