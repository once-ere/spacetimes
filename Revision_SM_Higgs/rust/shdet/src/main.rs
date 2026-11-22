//! sh_det -- stage 7 of the revision of the hypotheses, second version (folder Revision_SM_Higgs of PROJECT spacetimes):
//! the determinant interaction that the author chose on 22 November 2026 for the Goldstone boson of the particle number
//! (decision D32 (c) of the task record, with my decisions of interpretation D36), and the mass that it gives that boson.
//!
//! The model is that of stages 3 and 4 (the program sh_superfluid, whose file model.rs this program includes unchanged):
//! the isodoublet Psi_LL (components 0..7, component 2 d + s) and the singlets chi_SH (8..11) and chi'_SH (12..15), all of
//! mass 0.2 (the cutoff is 1), the potential V^(1)_SH (G^S_SF = -8, G^V_SF = 0.8, G_SH = 100.8 for both pairing terms),
//! the methods HS (1, 1), HS (0, 1), HS (0, 0), KS and CJT, and in addition the determinant interaction
//!     -G^det_SH ( eps_st Pi^s_SH Pi'^t_SH + h.c. ) = -G^det_SH ( Pi^u Pi'^d - Pi^d Pi'^u + h.c. ),   eps_ud = 1,
//! with Pi^s_SH = (1/2) Psi^T K^s Psi and Pi'^s_SH = (1/2) Psi^T K'^s Psi (K^s = k_ds(0, s), K'^s = k_ds(1, s) of model.rs).
//! In the engine (sfcore::uniform_n) it is two pair-pair terms c [(Psi^T A Psi)(Psi^T B Psi) + h.c.]: c = -G^det / 4 with
//! (A, B) = (K^u, K'^d) and c = +G^det / 4 with (A, B) = (K^d, K'^u).  Their three Wick contractions are all pair
//! contractions; HS keeps the direct one (weight 1) and no exchange, KS and CJT keep all three (decision D36).
//!
//! What it computes:
//!   * the engine: the pair self-energy of the determinant interaction is the derivative of its energy; the energy is
//!     invariant under SU(2)_L, SU(2)_R and the relative number and changes sign under the rotation by the particle
//!     number through pi/4 (the sign convention G^det >= 0); with G^det = 0 the ground states of stage 4 are reproduced
//!     bit for bit (the engine's new terms change nothing in a model without them);
//!   * for each method and G^det / G_SH = 0, 0.01, 0.1, 0.5 (continuation in G^det from the ground state of stage 4): the
//!     ground state (the custodial Higgs superfluid), the energy of the determinant term, the gap, the static response
//!     in the sector of relative number 0 and in the pairs of relative number +-2, the susceptibilities of the eight
//!     generators (as in stage 4), the scale v_SH, rho, and the masses of the modes by the linear response at a frequency
//!     (module sfcore::rpa_n) in the sector of relative number 0: the boson of the particle number (the pseudo-Goldstone
//!     boson), the amplitude mode and the custodial triplet, identified by the overlap of the null space with the four
//!     neutral directions of the pair self-energy (the common amplitude P_dir, the common phase i P_dir, the relative
//!     amplitude P_trip and the relative phase i P_trip); the mass of the pseudo-Goldstone boson by the relation of Dashen
//!     (that of Gell-Mann, Oakes and Renner), m_G^2 chi^FF = -16 E^det, with chi^FF the susceptibility of the particle
//!     number and E^det the energy per volume of the determinant term (the double commutator of the term with the
//!     particle number is 16 times the term, since the term changes the particle number by 4);
//!   * in HS (0, 0), where only the pairing and the determinant terms act: the closed forms on the grid (the sums of
//!     Section 5.3 of the manuscript of stage 4, with the couplings G_SH + G^det and G_SH - G^det of the four neutral
//!     channels), the scan of G^det / G_SH from 0.001 to the end of the bound state, and the couplings at which the mass
//!     of the pseudo-Goldstone boson reaches d_0 / 2, d_0 and the threshold 2 M_eff of two quasi-particles;
//!   * the grid: 24 and 48 radial nodes.
//!
//! The checks (report data/hck_sh_det_checks.txt):
//!   DT-1  the engine: (a) the pair self-energy of the determinant term is minus twice the derivative of its energy with
//!         respect to kap^* (central differences on random pair matrices); (b) its energy is invariant under random
//!         elements of SU(2)_L, SU(2)_R and of the rotations by the relative number, and the rotation by the particle
//!         number through pi/4 changes its sign and leaves the other terms unchanged; (c) with G^det = 0 the ground state
//!         of every method of stage 4 (data/sh_hg_minimum.csv) is reproduced bit for bit;
//!   DT-2  the ground states with the determinant term: (a) self-consistent; (b) the custodial Higgs superfluid (two equal
//!         singular values of the bidoublet of pair coefficients, no charged component, the coefficient of the vacuum
//!         direction real); (c) the energy falls and d_0 grows with G^det; (d) E^det < 0; (e) in HS (0, 0) d_0 solves the
//!         gap equation with the coupling G_SH + G^det;
//!   DT-3  the static response: in the sector of relative number 0 exactly three eigenvalues 1 (the Goldstone modes of
//!         W^+-, Z) for G^det > 0 and four for G^det = 0, all others below 1; in the pairs of relative number +-2 all
//!         below 1; in HS (0, 0) the four neutral directions are eigenvectors with the eigenvalues of the closed form:
//!         (G - G^det)/(G + G^det) for the common phase, 1 for the relative phase, (G + G^det) S_amp(0) for the common
//!         amplitude and (G - G^det) S_amp(0) for the relative amplitude;
//!   DT-4  the susceptibilities: symmetric, the custodial structure, rho = 1, four positive eigenvalues; in HS (0, 0)
//!         chi^FF = 2 d_0^2 sum_q w / E^3;
//!   DT-5  the linear response: at omega = 0 three singular values vanish for G^det > 0 (four for G^det = 0); below the
//!         lowest transition energy there is a zero whose null space contains the common phase (the pseudo-Goldstone
//!         boson), with multiplicity 1;
//!   DT-6  HS (0, 0): the pseudo-Goldstone boson, the amplitude mode and the triplet lie at the zeros of the closed forms
//!         (G - G^det) S_ph(omega) = 1, omega = d_0, (G - G^det) S_amp(omega) = 1;
//!   DT-7  Dashen: m_G^2 chi^FF / (-16 E^det) -> 1 as G^det -> 0 (the scan of HS (0, 0): the deviation is proportional to
//!         G^det; in every method at G^det / G_SH = 0.01 below 1 per cent);
//!   DT-8  the scan of HS (0, 0): the pseudo-Goldstone boson is found at every point below the end of the bound state;
//!         its mass grows monotonically with G^det; the couplings of the crossings;
//!   DT-9  the grid: 24 and 48 radial nodes give the same mass of the pseudo-Goldstone boson (HS (0, 0)).
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_det
//! Input in data/: sh_couplings.csv (the program sh_quantum) and sh_hg_minimum.csv (the program sh_higgs, for check
//! DT-1c).  Output in data/: hck_sh_det_checks.txt, sh_det_states.csv, sh_det_modes.csv, sh_det_scan.csv,
//! sh_det_values.csv.
#[allow(dead_code)]
#[path = "../../shsuperfluid/src/model.rs"]
mod model;

use model::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{sigma, t_gen};
use sfcore::methods::{grid, Method};
use sfcore::quad::gauss_legendre;
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::rng::Rng;
use sfcore::rpa_n::{kernel_n, ResponseN, SectorN};
use sfcore::uniform_n::{n_dirs, p_dirs, response_eigs, FieldBlock, ModelN, PairPairN, SolutionN};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;
use std::time::Instant;

/// the potential of stages 3 and 4, second version (both pairing terms with G_SH = 100.8, decision D31)
const V1_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 100.8 };
/// the values of G^det_SH / G_SH of the main computation (decision D36: the other couplings are held fixed)
const RATIOS: [f64; 4] = [0.0, 0.01, 0.1, 0.5];
/// the self-consistency, as in stage 4 (the program sh_higgs): mixing, tolerance of the residual of the self-energies,
/// maximum of iterations, and the size below which an element of a self-energy is set to zero after each step
const MIX: f64 = 0.6;
const TOL: f64 = 1e-13;
const MAXIT: usize = 40000;
const FLUSH: f64 = 1e-30;
/// the step of the chemical potentials of the susceptibilities (and twice it, for the Richardson combination), as in
/// stage 4
const H_MU: f64 = 0.005;
/// the step of the central differences of the static response, as in stage 3
const H_RESP: f64 = 1e-5;

fn methods() -> Vec<Method> {
    vec![Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Ks, Method::Cjt]
}
fn mname(m: &Method) -> String {
    match m {
        Method::Hs { s_s, s_v } => format!("HS ({}, {})", s_s, s_v),
        Method::Ks => "KS".into(),
        Method::Cjt => "CJT".into(),
    }
}
fn is_hs00(m: &Method) -> bool {
    matches!(m, Method::Hs { s_s, s_v } if *s_s == 0.0 && *s_v == 0.0)
}

// ================================================================================================ the determinant term

/// the weights (direct, exchange) of the contractions of the determinant interaction in a method (decision D36): HS
/// keeps the direct pair contraction, the one of the channels of Pi^s_SH and Pi'^t_SH in which it decouples the term,
/// and no exchange; KS and CJT keep all three (KS projects only the normal matrix, and this term has no normal
/// contraction)
fn det_weights(method: &Method) -> (f64, f64) {
    match method {
        Method::Hs { .. } => (1.0, 0.0),
        Method::Ks | Method::Cjt => (1.0, 1.0),
    }
}

/// the determinant interaction -G^det (Pi^u Pi'^d - Pi^d Pi'^u + h.c.) as two pair-pair terms of the engine:
/// Pi^u Pi'^d = (1/4)(Psi^T K^u Psi)(Psi^T K'^d Psi), so c = -G^det / 4 for (K^u, K'^d) and c = +G^det / 4 for (K^d, K'^u)
fn det_terms(method: &Method, gdet: f64) -> Vec<PairPairN> {
    if gdet == 0.0 {
        return vec![];
    }
    let (wd, wx) = det_weights(method);
    vec![
        PairPairN { c: -0.25 * gdet, a: k_ds(0, 0), b: k_ds(1, 1), w_d: wd, w_x: wx, label: "det: Pi^u Pi'^d".into() },
        PairPairN { c: 0.25 * gdet, a: k_ds(0, 1), b: k_ds(1, 0), w_d: wd, w_x: wx, label: "det: -Pi^d Pi'^u".into() },
    ]
}

/// the model of a method with the potential V^(1)_SH, the determinant coupling gdet and the common mass m of the three
/// fields, on nr radial nodes
fn model_det(method: &Method, gdet: f64, m: f64, nr: usize) -> ModelN {
    let blocks = vec![
        FieldBlock { off: 0, ncomp: NP, mass: m },
        FieldBlock { off: OFF_CHI, ncomp: NX, mass: m },
        FieldBlock { off: OFF_CHI2, ncomp: NX, mass: m },
    ];
    let mut mo = ModelN::new(blocks, 0.0, terms_sh(method, &V1_SH), grid(nr));
    mo.pair_pair = det_terms(method, gdet);
    mo
}

/// a copy of a model with an external one-body term Psi^dag X Psi (for X = -mu Q: a chemical potential mu of the charge Q)
fn with_ext(mo: &ModelN, x: &CMat) -> ModelN {
    let mut m2 = ModelN::new(mo.blocks.clone(), mo.mu, mo.terms.clone(), mo.grid.clone());
    m2.pair_pair = mo.pair_pair.clone();
    m2.ext_n = Some(x.clone());
    m2
}

/// the energy per volume of the determinant term alone in a state with the local pair matrix kap
fn det_energy(mo: &ModelN, kap: &CMat) -> f64 {
    mo.pair_pair.iter().map(|t| t.energy(kap)).sum()
}

// ================================================================================================ the iteration

/// the self-consistent solution by the iteration of stages 3 and 4 (copied from the program sh_higgs: linear mixing
/// with the weight MIX, N made Hermitian and P antisymmetric after each step, elements below FLUSH set to zero); with
/// `fixed_p` the pair self-energy is held fixed and only the normal self-energy is iterated
fn solve_clean(mo: &ModelN, n0: &CMat, p0: &CMat, fixed_p: bool) -> SolutionN {
    let flush = |m: &mut CMat| {
        for z in m.a.iter_mut() {
            if z.abs() < FLUSH {
                *z = C64::ZERO;
            }
        }
    };
    let mut nself = n0.clone();
    let mut pself = p0.clone();
    let mut it = 0;
    loop {
        let (cn, cp) = mo.states(&nself, &pself);
        let (drho, kap, ek) = mo.local(&cn, &cp);
        let (ei, n2, p2) = mo.interaction(&drho, &kap);
        let res = if fixed_p { n2.sub(&nself).max_abs() } else { n2.sub(&nself).max_abs().max(p2.sub(&pself).max_abs()) };
        if res < TOL || it >= MAXIT {
            let p_out = if fixed_p { pself.clone() } else { p2 };
            return SolutionN { n_self: n2, p_self: p_out, drho, kap, energy: ek + ei, e_kin: ek, e_int: ei, iterations: it, residual: res };
        }
        nself = n2.scale_r(MIX).add(&nself.scale_r(1.0 - MIX));
        nself = nself.add(&nself.dagger()).scale_r(0.5);
        flush(&mut nself);
        if !fixed_p {
            pself = p2.scale_r(MIX).add(&pself.scale_r(1.0 - MIX));
            pself = pself.sub(&pself.transpose()).scale_r(0.5);
            flush(&mut pself);
        }
        it += 1;
    }
}

// ================================================================================================ directions and charges

/// the direction of the vacuum in the pair self-energy, K'_u^dag - K_d^dag (the bidoublet i tau^2 of stage 3; the
/// common amplitude of the two neutral pair fields)
fn p_dir() -> CMat {
    k_ds(1, 0).dagger().sub(&k_ds(0, 1).dagger())
}
/// the direction of the neutral member of the custodial triplet, K'_u^dag + K_d^dag (the two neutral pair fields
/// displaced oppositely: the relative amplitude)
fn p_trip() -> CMat {
    k_ds(1, 0).dagger().add(&k_ds(0, 1).dagger())
}
/// the complex coefficient of a direction in a pair self-energy, <D, P> / <D, D> (inner product tr(A^dag B))
fn coef(p: &CMat, d: &CMat) -> C64 {
    d.dagger().mul(p).trace().scale(1.0 / d.dagger().mul(d).trace().re)
}
/// the pair amplitudes |<Pi^d_SH>| = |tr(K_d kap^T)| / 2 and |<Pi'^u_SH>| = |tr(K'_u kap^T)| / 2
fn phis(kap: &CMat) -> (f64, f64) {
    (0.5 * k_ds(0, 1).mul(&kap.transpose()).trace().abs(), 0.5 * k_ds(1, 0).mul(&kap.transpose()).trace().abs())
}

/// the eight generators of the global group on the sixteen components, in the order T^1, T^2, T^3, T^1_R, T^2_R,
/// T^3_R, R, F_tot (as in stages 3 and 4)
fn generators() -> Vec<CMat> {
    let mut g: Vec<CMat> = (1..=3).map(|a| emb_psi(&t_gen(a))).collect();
    for a in 1..=3 {
        g.push(emb_pair(&sigma(a).scale_r(0.5).kron(&CMat::identity(4))));
    }
    g.push(CMat::block_diag(&CMat::identity(NP), &CMat::identity(NS).scale_r(-1.0)));
    g.push(CMat::identity(N));
    g
}

/// the charge densities <Q_a> = tr(Q_a rho) of a state
fn charges(gens: &[CMat], drho: &CMat) -> Vec<f64> {
    gens.iter().map(|q| q.mul(drho).trace().re).collect()
}

/// the susceptibilities chi^{ab} = d<Q_a>/d mu_b (8 x 8) of a ground state, as in stage 4: the ground states of
/// H - mu Q_b for mu = +-h, +-2h from the ground state, central differences combined by Richardson; returns the matrix
/// and the largest difference of the two steps relative to the largest element
fn susceptibilities(mo: &ModelN, sol: &SolutionN, h: f64) -> (Vec<Vec<f64>>, f64) {
    let g = generators();
    let na = g.len();
    let mut chi = vec![vec![0.0; na]; na];
    let mut dev = 0.0f64;
    let mut scale = 0.0f64;
    for b in 0..na {
        let q_at = |mu: f64| -> Vec<f64> {
            let m2 = with_ext(mo, &g[b].scale_r(-mu));
            let s = solve_clean(&m2, &sol.n_self, &sol.p_self, false);
            charges(&g, &s.drho)
        };
        let (qp1, qm1, qp2, qm2) = (q_at(h), q_at(-h), q_at(2.0 * h), q_at(-2.0 * h));
        for a in 0..na {
            let c1 = (qp1[a] - qm1[a]) / (2.0 * h);
            let c2 = (qp2[a] - qm2[a]) / (4.0 * h);
            chi[a][b] = (4.0 * c1 - c2) / 3.0;
            dev = dev.max((c1 - c2).abs());
            scale = scale.max(chi[a][b].abs());
        }
    }
    (chi, dev / scale.max(1e-300))
}

/// the eigenvalues (ascending) of the symmetric part of chi
fn chi_eigs(chi: &[Vec<f64>]) -> Vec<f64> {
    let k = chi.len();
    let mut m = CMat::zeros(k);
    for i in 0..k {
        for j in 0..k {
            m.set(i, j, c(0.5 * (chi[i][j] + chi[j][i]), 0.0));
        }
    }
    eigh(&m).0
}

// ================================================================================================ closed forms of HS (0, 0)

/// I1(M) = int_0^1 q^2 dq / (q^2 + M^2)^(1/2) (closed form; the cutoff is 1), as in stage 4
fn i1(mq: f64) -> f64 {
    0.5 * ((1.0 + mq * mq).sqrt() - mq * mq * (1.0 / mq).asinh())
}
/// the pairing-only ground state (HS (0, 0)) of the coupling g and the mass m from the closed-form gap equation
/// (g / (4 pi^2)) I1((m^2 + d^2/4)^(1/2)) = 1 (bisection), as in stage 4; here g = G_SH + G^det
fn gap_closed(g: f64, m: f64) -> f64 {
    let fgap = |d: f64| g / (4.0 * PI * PI) * i1((m * m + d * d / 4.0).sqrt()) - 1.0;
    let (mut a, mut b) = (1e-9, 200.0);
    for _ in 0..200 {
        let cc = 0.5 * (a + b);
        if fgap(cc) > 0.0 {
            a = cc;
        } else {
            b = cc;
        }
    }
    0.5 * (a + b)
}

/// the sums of the pairing-only method on the radial nodes of a grid (the six directions carry equal weights and the
/// integrands are isotropic, so the sums over the radial nodes with the weights q^2 w_r / (2 pi^2) are the sums over the
/// grid): with E = (q^2 + M^2)^(1/2), M^2 = m^2 + d^2 / 4,
///   S_ph(omega)  = (1/2) sum w E / (E^2 - omega^2 / 4),
///   S_amp(omega) = (1/2) sum w (q^2 + m^2) / (E (E^2 - omega^2 / 4)),
///   S_3          = sum w / E^3.
/// They are the integrals (G / 4 pi^2) int q^2 dq (...) of (5.3) of stage 4 divided by G.  The gap equation of the
/// coupling g is g S_ph(0) = 1; the common phase, the relative phase, the common amplitude and the relative amplitude of
/// the two neutral pair fields are collective modes where (G - G^det) S_ph, (G + G^det) S_ph, (G + G^det) S_amp and
/// (G - G^det) S_amp equal 1.
struct GridSums {
    q: Vec<f64>,
    w: Vec<f64>,
    m: f64,
    meff: f64,
}
impl GridSums {
    fn new(nr: usize, m: f64, d: f64) -> GridSums {
        let gr = grid(nr);
        let q = gr.kr.clone();
        let w: Vec<f64> = gr.kr.iter().zip(&gr.wr).map(|(k, wr)| k * k * wr / (2.0 * PI * PI)).collect();
        GridSums { q, w, m, meff: (m * m + d * d / 4.0).sqrt() }
    }
    fn e(&self, q: f64) -> f64 {
        (q * q + self.meff * self.meff).sqrt()
    }
    fn s_ph(&self, om: f64) -> f64 {
        self.q.iter().zip(&self.w).map(|(q, w)| {
            let e = self.e(*q);
            0.5 * w * e / (e * e - om * om / 4.0)
        }).sum()
    }
    fn s_amp(&self, om: f64) -> f64 {
        self.q.iter().zip(&self.w).map(|(q, w)| {
            let e = self.e(*q);
            0.5 * w * (q * q + self.m * self.m) / (e * (e * e - om * om / 4.0))
        }).sum()
    }
    fn s3(&self) -> f64 {
        self.q.iter().zip(&self.w).map(|(q, w)| w / self.e(*q).powi(3)).sum()
    }
    /// S_ph and S_amp at the threshold omega = 2 M of the continuum, where E^2 - omega^2 / 4 = q^2: finite, since the
    /// weight w carries the factor q^2 (used with the exact sums: a mode exists below the threshold of the continuum
    /// when its coupling times this value exceeds 1)
    fn s_ph_thr(&self) -> f64 {
        self.q.iter().zip(&self.w).map(|(q, w)| 0.5 * w * self.e(*q) / (q * q)).sum()
    }
    fn s_amp_thr(&self) -> f64 {
        self.q.iter().zip(&self.w).map(|(q, w)| 0.5 * w * (q * q + self.m * self.m) / (self.e(*q) * q * q)).sum()
    }
    /// the lowest energy of two quasi-particles on the grid, 2 E at the smallest radial node (the nodes of Gauss and
    /// Legendre come in descending order, so the smallest is found, not taken from the front)
    fn threshold(&self) -> f64 {
        2.0 * self.e(self.q.iter().cloned().fold(f64::INFINITY, f64::min))
    }
    /// the root omega in (lo, hi) of g S(omega) = 1 with S increasing in omega (bisection)
    fn root(&self, g: f64, amp: bool, lo: f64, hi: f64) -> f64 {
        let fx = |om: f64| g * if amp { self.s_amp(om) } else { self.s_ph(om) } - 1.0;
        let (mut a, mut b) = (lo, hi);
        if fx(a) > 0.0 || fx(b) < 0.0 {
            return f64::NAN;
        }
        for _ in 0..200 {
            let cc = 0.5 * (a + b);
            if fx(cc) > 0.0 {
                b = cc;
            } else {
                a = cc;
            }
        }
        0.5 * (a + b)
    }
}

/// the same sums with the exact integrals (Gauss-Legendre with 2000 nodes on [0, 1]; the integrands are smooth below
/// the threshold), for the statements in the continuum limit of the grid of the momenta
fn exact_sums(m: f64, d: f64) -> GridSums {
    let (x, wl) = gauss_legendre(2000);
    let q: Vec<f64> = x.iter().map(|t| 0.5 * (t + 1.0)).collect();
    let w: Vec<f64> = q.iter().zip(&wl).map(|(k, wt)| k * k * 0.5 * wt / (2.0 * PI * PI)).collect();
    GridSums { q, w, m, meff: (m * m + d * d / 4.0).sqrt() }
}

// ================================================================================================ the static response

/// the self-energy that the contact terms produce from the change of the local matrices caused by a change dP of the
/// pair self-energy (central differences with the step H_RESP): the static response applied to one direction
fn static_apply(mo: &ModelN, nself: &CMat, pself: &CMat, dp: &CMat) -> (CMat, CMat) {
    let h = H_RESP;
    let (_, rp, kp) = mo.energy_of(nself, &pself.add(&dp.scale_r(h)));
    let (_, rm, km) = mo.energy_of(nself, &pself.sub(&dp.scale_r(h)));
    mo.self_energy_lin(&rp.sub(&rm).scale_r(0.5 / h), &kp.sub(&km).scale_r(0.5 / h))
}

/// the static response of a ground state in the two blocks of stage 3: block A (relative number 0: the pairs of Psi_LL
/// with the singlets, the normal blocks of Psi_LL, of each singlet and of their mixing) and block B (relative number
/// +-2: the pairs of two Psi_LL, of two singlets, and the mixing of Psi_LL with the singlets); returns the eigenvalues of
/// block A (descending), the largest of block B, and the largest asymmetry of the matrices
fn static_blocks(mo: &ModelN, sol: &SolutionN) -> (Vec<f64>, f64, f64) {
    let (ps, ss) = (psi_indices(), sing_indices());
    let (c1, c2) = (species_indices(2), species_indices(3));
    let mut block_a = p_dirs(&ps, &ss);
    block_a.extend(n_dirs(&ps, &ps));
    block_a.extend(n_dirs(&c1, &c1));
    block_a.extend(n_dirs(&c2, &c2));
    block_a.extend(n_dirs(&c1, &c2));
    let mut block_b = p_dirs(&ps, &ps);
    block_b.extend(p_dirs(&ss, &ss));
    block_b.extend(n_dirs(&ps, &ss));
    let (ra, a1, a2, _) = response_eigs(mo, &sol.n_self, &sol.p_self, &block_a, H_RESP);
    let (rb, b1, b2, _) = response_eigs(mo, &sol.n_self, &sol.p_self, &block_b, H_RESP);
    (ra, rb[0], a1.max(a2).max(b1).max(b2))
}

// ================================================================================================ the linear response

/// the sector of relative number 0 (as in stage 4): the normal blocks of Psi_LL and of the pair of singlets and the pairs
/// of Psi_LL with the singlets
fn sector_r0() -> SectorN {
    let (ps, ss) = (psi_indices(), sing_indices());
    SectorN::new(N, &[ps.clone(), ss.clone()], &[(ps, ss)])
}
/// the sector of the neutral pairs of the vacuum alone (down with chi_SH, up with chi'_SH), as in stage 4: exact when
/// only the pairing and the determinant terms act (HS (0, 0)): the kernel then reads only the projections onto the
/// pair matrices, the determinant term maps the neutral pairs onto the neutral pairs (its charged part, Pi^u Pi'^d,
/// has no direct contraction in the neutral vacuum, and HS keeps no exchange), and the electric charge keeps the neutral
/// pairs apart from the charged ones
fn sector_vac_pairs() -> SectorN {
    let (up, dn, c1, c2) = (species_indices(0), species_indices(1), species_indices(2), species_indices(3));
    SectorN::new(N, &[], &[(dn, c1), (up, c2)])
}

/// the four neutral field vectors of a sector, normalized: the common amplitude (P_dir, P_dir^dag), the common phase
/// (i P_dir, -i P_dir^dag), the relative amplitude (P_trip, P_trip^dag) and the relative phase (i P_trip, -i P_trip^dag),
/// as amplitudes (dP, dPbar) of e^{-i omega t}: an oscillation P0 (1 + eps cos omega t) of the amplitude has
/// dP = eps P0 / 2, dPbar = eps P0^dag / 2, and an oscillation P0 e^{i eps cos omega t} of the phase dP = i eps P0 / 2,
/// dPbar = -i eps P0^dag / 2
fn neutral_vectors(sec: &SectorN) -> [Vec<C64>; 4] {
    let z = CMat::zeros(N);
    let i = c(0.0, 1.0);
    let norm = |v: Vec<C64>| -> Vec<C64> {
        let s: f64 = v.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        v.iter().map(|x| x.scale(1.0 / s)).collect()
    };
    let (pd, pt) = (p_dir(), p_trip());
    [
        norm(sec.to_vec(&z, &pd, &pd.dagger())),
        norm(sec.to_vec(&z, &pd.scale(i), &pd.dagger().scale(-i))),
        norm(sec.to_vec(&z, &pt, &pt.dagger())),
        norm(sec.to_vec(&z, &pt.scale(i), &pt.dagger().scale(-i))),
    ]
}
const NEUTRAL_NAMES: [&str; 4] = ["common amplitude", "common phase", "relative amplitude", "relative phase"];

/// the squared length of the projection of a unit vector onto the span of a list of vectors (Gram-Schmidt), as in
/// stage 4
fn overlap_span(u: &[C64], vs: &[Vec<C64>]) -> f64 {
    let mut basis: Vec<Vec<C64>> = vec![];
    for v in vs {
        let mut w = v.clone();
        for b in &basis {
            let mut s = C64::ZERO;
            for (x, y) in b.iter().zip(w.iter()) {
                s += x.conj() * *y;
            }
            for (wi, bi) in w.iter_mut().zip(b.iter()) {
                *wi -= *bi * s;
            }
        }
        let nrm: f64 = w.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
        if nrm > 1e-10 {
            basis.push(w.iter().map(|x| x.scale(1.0 / nrm)).collect());
        }
    }
    let mut tot = 0.0;
    for b in &basis {
        let mut s = C64::ZERO;
        for (x, y) in b.iter().zip(u.iter()) {
            s += x.conj() * *y;
        }
        tot += s.norm_sqr();
    }
    tot
}

/// the smallest singular value of 1 - chi(omega) F resolved below the square root of the machine precision (copied
/// from stage 4, where it is explained)
fn sigma_fine(rf: &ResponseN, w: f64) -> f64 {
    let m = rf.chi(w);
    let nv = m.rows;
    let a = CMat::identity(nv).sub(&m);
    let (ev, v) = eigh(&a.dagger().mul(&a));
    let kmin = (0..nv).fold(0, |b, i| if ev[i] < ev[b] { i } else { b });
    let sel: Vec<usize> = (0..nv).filter(|&i| ev[i] < 1e-6 || i == kmin).collect();
    let mut vk = CMat::new(nv, sel.len());
    for (cc, &i) in sel.iter().enumerate() {
        for r in 0..nv {
            vk.set(r, cc, v.get(r, i));
        }
    }
    let av = a.mul(&vk);
    let (e2, _) = eigh(&av.dagger().mul(&av));
    e2.iter().fold(f64::INFINITY, |b, x| b.min(x.max(0.0).sqrt()))
}

/// one collective mode found by the linear response: its frequency, the smallest singular value there, the dimension of
/// the null space (singular values below 1e-6: the multiplicity), and the squared lengths of the projections of the
/// four neutral vectors onto the field vectors of the null space
#[derive(Clone, Debug)]
struct Zero {
    omega: f64,
    sigma: f64,
    mult: usize,
    ov: [f64; 4],
    /// (threshold - omega) / threshold: a zero within 1e-4 of the lowest transition energy is the edge of the continuum
    /// of the grid (the sums over the discrete momenta diverge at each transition energy, so on a grid a zero always
    /// sits just below the threshold; in the continuum it is a bound state only if it stays there as the grid is refined)
    margin: f64,
}

/// the result of the linear response of a state in a sector
#[derive(Clone, Debug)]
struct Modes {
    /// the number of singular values of 1 - chi(0) F below 1e-6 and the smallest singular values at omega = 0
    goldstone: usize,
    sing0: Vec<f64>,
    /// the lowest transition energy of the sector (the threshold of two quasi-particles)
    threshold: f64,
    zeros: Vec<Zero>,
}
impl Modes {
    /// the zero whose null space holds the neutral vector k best (with its squared overlap above 0.5), if any
    fn best(&self, k: usize) -> Option<Zero> {
        self.zeros.iter().filter(|z| z.ov[k] > 0.5).max_by(|a, b| a.ov[k].partial_cmp(&b.ov[k]).unwrap()).cloned()
    }
    /// the same, but only a bound state: a zero below the threshold by more than 1e-4 (relative)
    fn bound(&self, k: usize) -> Option<Zero> {
        self.best(k).filter(|z| z.margin > 1e-4)
    }
}

/// the collective modes of a state in a sector below the lowest transition energy: the zeros of the smallest singular
/// value of 1 - chi(omega) F (the scan of stage 4: 600 equal steps from omega_lo to the threshold and 25 more that
/// approach it geometrically, every local minimum of the scan refined by golden sections on sigma_fine and kept when
/// the refined singular value lies below 1e-6), with the overlaps of their null spaces with the four neutral vectors.
/// Every local minimum is refined, as in the scan of stage 4: near a zero the singular value rises like
/// c |omega - omega_0| with a slope c that can be of order one, so a scan point a step away (about 0.003) can lie above
/// any fixed depth; the first run of this program refined only the minima below 1e-3 and missed real zeros (entry E66
/// of the task record)
fn find_modes(mo: &ModelN, nself: &CMat, pself: &CMat, sec: &SectorN, omega_lo: f64, threads: usize, scan: bool) -> Modes {
    let resp = ResponseN::new(mo, nself, pself, sec, threads);
    let fk = kernel_n(mo, sec);
    let rf = resp.with_kernel(&fk);
    let (goldstone, sing0) = rf.count_small_singular(0.0, 1e-6);
    if !scan {
        return Modes { goldstone, sing0: sing0.iter().take(8).cloned().collect(), threshold: resp.threshold, zeros: vec![] };
    }
    let nvec = neutral_vectors(sec);
    let omega_max = resp.threshold * (1.0 - 1e-9);
    let poles: Vec<f64> = rf.buckets.iter().map(|b| b.omega).collect();
    let near_pole = |w: f64| poles.iter().any(|p| (w - p).abs() < 1e-7 * (1.0 + p));
    let sig = |w: f64| rf.smallest_singular(w).0;
    let nscan = 600;
    let mut ws: Vec<f64> = (0..=nscan).map(|i| omega_lo + (omega_max - omega_lo) * i as f64 / nscan as f64).collect();
    for j in 4..=28 {
        ws.push(omega_max - (omega_max - omega_lo) * 10f64.powf(-(j as f64) / 4.0));
    }
    ws.sort_by(|x, y| x.partial_cmp(y).unwrap());
    ws.dedup_by(|x, y| (*x - *y).abs() < 1e-12);
    let ws: Vec<f64> = ws.into_iter().filter(|w| !near_pole(*w)).collect();
    let ss: Vec<f64> = ws.iter().map(|w| sig(*w)).collect();
    let mut zeros: Vec<Zero> = vec![];
    for i in 1..ws.len() - 1 {
        if !(ss[i] <= ss[i - 1] && ss[i] <= ss[i + 1]) {
            continue;
        }
        let sigf = |w: f64| sigma_fine(&rf, w);
        let (mut a, mut b) = (ws[i - 1], ws[i + 1]);
        let gr = 0.5 * (5f64.sqrt() - 1.0);
        let (mut x1, mut x2) = (b - gr * (b - a), a + gr * (b - a));
        let (mut f1, mut f2) = (sigf(x1), sigf(x2));
        for _ in 0..75 {
            if f1 < f2 {
                b = x2;
                x2 = x1;
                f2 = f1;
                x1 = b - gr * (b - a);
                f1 = sigf(x1);
            } else {
                a = x1;
                x1 = x2;
                f1 = f2;
                x2 = a + gr * (b - a);
                f2 = sigf(x2);
            }
        }
        let wm = 0.5 * (a + b);
        let smin = sigma_fine(&rf, wm);
        if smin >= 1e-6 {
            continue;
        }
        // the null space at wm, mapped to the field amplitudes by F
        let m = rf.chi(wm);
        let nv = m.rows;
        let am = CMat::identity(nv).sub(&m);
        let (ev, vecs) = eigh(&am.dagger().mul(&am));
        let mut fields = vec![];
        for k in 0..nv {
            if ev[k].max(0.0).sqrt() < 1e-6 {
                let mut fu = vec![C64::ZERO; nv];
                for r in 0..nv {
                    let mut s = C64::ZERO;
                    for cc in 0..nv {
                        s += fk.get(r, cc) * vecs.get(cc, k);
                    }
                    fu[r] = s;
                }
                fields.push(fu);
            }
        }
        let ov = [0, 1, 2, 3].map(|k| overlap_span(&nvec[k], &fields));
        if !zeros.iter().any(|z| (z.omega - wm).abs() < 1e-9 * (1.0 + wm)) {
            zeros.push(Zero { omega: wm, sigma: smin, mult: fields.len(), ov, margin: (resp.threshold - wm) / resp.threshold });
        }
    }
    Modes { goldstone, sing0: sing0.iter().take(8).cloned().collect(), threshold: resp.threshold, zeros }
}

// ================================================================================================ one state

/// everything computed for one method at one value of G^det
#[derive(Clone)]
struct StateRes {
    method: Method,
    ratio: f64,
    gdet: f64,
    sol: SolutionN,
    /// the complex coefficient of the vacuum direction in the pair self-energy (before the rotation of its phase) and d_0
    dcoef: C64,
    d0: f64,
    phi: (f64, f64),
    e_det: f64,
    desc: Describe,
    gap: f64,
    /// the static response: the eigenvalues of block A (descending), the largest of block B, the asymmetry
    resp_a: Vec<f64>,
    resp_b: f64,
    resp_asym: f64,
    chi: Vec<Vec<f64>>,
    chi_dev: f64,
    modes: Option<Modes>,
    /// HS (0, 0): the static response of the four neutral directions: (coefficient along the direction, norm of the rest)
    neutral_static: Option<[(f64, f64); 4]>,
    seconds: f64,
}

/// the ground state of a method at a value of G^det from a start (N0, P0), rotated so that the coefficient of the
/// vacuum direction is real and positive, and all that is computed for it
fn run_state(method: Method, ratio: f64, n0: &CMat, p0: &CMat, with_modes: bool, threads: usize) -> StateRes {
    let t0 = Instant::now();
    let gdet = ratio * V1_SH.gsh;
    let mo = model_det(&method, gdet, M_LL, NR);
    let sol0 = solve_clean(&mo, n0, p0, false);
    let dcoef = coef(&sol0.p_self, &p_dir());
    let d0 = dcoef.abs();
    // the rotation by the particle number that makes the coefficient real and positive (P -> e^{2 i theta} P,
    // kap -> e^{-2 i theta} kap in the engine's convention); with G^det > 0 the phase is already 0 or pi (DT-2b)
    let ph = dcoef.scale(1.0 / d0).conj();
    let sol = SolutionN { p_self: sol0.p_self.scale(ph), kap: sol0.kap.scale(ph.conj()), ..sol0.clone() };
    let desc = describe(&sol.n_self, &sol.p_self, &sol.drho);
    let e_det = det_energy(&mo, &sol.kap);
    let gap = mo.gap(&sol.n_self, &sol.p_self).0;
    let (resp_a, resp_b, resp_asym) = static_blocks(&mo, &sol);
    let (chi, chi_dev) = susceptibilities(&mo, &sol, H_MU);
    let neutral_static = if is_hs00(&method) {
        let i = c(0.0, 1.0);
        let dirs = [p_dir(), p_dir().scale(i), p_trip(), p_trip().scale(i)];
        let mut out = [(0.0, 0.0); 4];
        for (k, d) in dirs.iter().enumerate() {
            let (_, p2) = static_apply(&mo, &sol.n_self, &sol.p_self, d);
            let cf = coef(&p2, d);
            let rest = p2.sub(&d.scale(cf)).fro() / d.fro();
            // the direction is an eigenvector when the rest vanishes; the eigenvalue is the real coefficient
            out[k] = (cf.re, rest.max(cf.im.abs()));
        }
        Some(out)
    } else {
        None
    };
    // the linear response: at G^det = 0 only the count of the Goldstone modes at omega = 0 (the modes of stage 4); for
    // G^det > 0 also the scan of the frequencies below the threshold
    let modes = if with_modes { Some(find_modes(&mo, &sol.n_self, &sol.p_self, &sector_r0(), 0.002, threads, ratio > 0.0)) } else { None };
    StateRes {
        method,
        ratio,
        gdet,
        phi: phis(&sol.kap),
        sol,
        dcoef,
        d0,
        e_det,
        desc,
        gap,
        resp_a,
        resp_b,
        resp_asym,
        chi,
        chi_dev,
        modes,
        neutral_static,
        seconds: t0.elapsed().as_secs_f64(),
    }
}

/// the continuation of one method over the ratios of RATIOS: the ratio 0 from the start of stage 3 (the custodial start
/// 0.3 P_dir, as in stage 4), every further ratio from the self-energies of the state before it
fn run_method(method: Method, threads: usize) -> Vec<StateRes> {
    let z = CMat::zeros(N);
    let mut out: Vec<StateRes> = vec![];
    let (mut n0, mut p0) = (z.clone(), p_dir().scale_r(0.3));
    for &r in RATIOS.iter() {
        let st = run_state(method, r, &n0, &p0, true, threads);
        println!(
            "{} G_det/G_SH {}: d0 {:.8} E {:.8e} E_det {:.4e} pseudo-Goldstone {} ({:.0} s)",
            mname(&method),
            r,
            st.d0,
            st.sol.energy,
            st.e_det,
            st.modes.as_ref().and_then(|m| m.best(1)).map(|z| format!("{:.6}", z.omega)).unwrap_or("-".into()),
            st.seconds
        );
        n0 = st.sol.n_self.clone();
        p0 = st.sol.p_self.clone();
        out.push(st);
    }
    out
}

// ================================================================================================ the scan of HS (0, 0)

/// one point of the scan of HS (0, 0) at G^det / G_SH = r (mass m, nr radial nodes): the ground state from the closed
/// form (the fixed-d state at the d_0 of the gap equation with G_SH + G^det, whose self-consistency is checked), the
/// energy of the determinant term, the susceptibility of the particle number from its closed form, and the modes in the
/// sector of the neutral pairs of the vacuum (exact in this method), with the closed forms on the same grid
struct ScanPt {
    r: f64,
    gdet: f64,
    d0: f64,
    /// the relative difference of the pair self-energy that the engine produces from the state and d_0 P_dir
    sc_dev: f64,
    meff: f64,
    e_det: f64,
    chi_ff: f64,
    v: f64,
    /// the modes by the linear response: the pseudo-Goldstone boson (common phase), the amplitude mode and the neutral
    /// member of the triplet, and the Goldstone count at omega = 0
    w_g: f64,
    w_h: f64,
    w_t: f64,
    gold: usize,
    mult_g: usize,
    /// the same from the closed forms on the grid, and the threshold 2 E at the smallest node
    wc_g: f64,
    wc_t: f64,
    thr: f64,
    /// the Dashen mass (-16 E^det / chi^FF)^(1/2)
    w_dashen: f64,
    /// the pseudo-Goldstone boson and the neutral member of the triplet in the continuum limit (exact integrals; NaN
    /// where not bound), and the margin (threshold - omega) / threshold of the zero of the pseudo-Goldstone boson
    wx_g: f64,
    wx_t: f64,
    margin_g: f64,
}

fn scan_point(r: f64, m: f64, nr: usize) -> ScanPt {
    let method = Method::Hs { s_s: 0.0, s_v: 0.0 };
    let gdet = r * V1_SH.gsh;
    let mo = model_det(&method, gdet, m, nr);
    let d0 = gap_closed(V1_SH.gsh + gdet, m);
    let z = CMat::zeros(N);
    let sf = solve_clean(&mo, &z, &p_dir().scale_r(d0), true);
    let p_int = mo.interaction(&sf.drho, &sf.kap).2;
    let sc_dev = p_int.sub(&p_dir().scale_r(d0)).fro() / p_dir().scale_r(d0).fro();
    let e_det = det_energy(&mo, &sf.kap);
    let gs = GridSums::new(nr, m, d0);
    let chi_ff = 2.0 * d0 * d0 * gs.s3();
    let v = 0.5 * chi_ff.sqrt();
    let md = find_modes(&mo, &sf.n_self, &p_dir().scale_r(d0), &sector_vac_pairs(), 0.002, 1, true);
    let thr = gs.threshold();
    let (gm, gp) = (V1_SH.gsh - gdet, V1_SH.gsh + gdet);
    let _ = gp;
    let wc_g = if gdet > 0.0 { gs.root(gm, false, 1e-12, thr * (1.0 - 1e-12)) } else { 0.0 };
    let wc_t = gs.root(gm, true, d0, thr * (1.0 - 1e-12));
    let zg = md.best(1);
    ScanPt {
        r,
        gdet,
        d0,
        sc_dev,
        meff: gs.meff,
        e_det,
        chi_ff,
        v,
        w_g: zg.as_ref().map(|z| z.omega).unwrap_or(f64::NAN),
        mult_g: zg.as_ref().map(|z| z.mult).unwrap_or(0),
        w_h: md.best(0).map(|z| z.omega).unwrap_or(f64::NAN),
        w_t: md.best(2).map(|z| z.omega).unwrap_or(f64::NAN),
        gold: md.goldstone,
        wc_g,
        wc_t,
        thr,
        w_dashen: (-16.0 * e_det / chi_ff).max(0.0).sqrt(),
        wx_g: wg_exact(r, m).0,
        wx_t: wt_exact(r, m),
        margin_g: zg.as_ref().map(|z| z.margin).unwrap_or(f64::NAN),
    }
}

/// HS (0, 0) in the continuum limit (exact integrals): the mass of the pseudo-Goldstone boson at G^det / G_SH = r, or NaN
/// when (G - G^det) S_ph(omega) stays below 1 up to the threshold (the boson has merged into the continuum)
fn wg_exact(r: f64, m: f64) -> (f64, f64, f64) {
    let gdet = r * V1_SH.gsh;
    let d0 = gap_closed(V1_SH.gsh + gdet, m);
    let ex = exact_sums(m, d0);
    let thr = 2.0 * ex.meff;
    let w = if gdet > 0.0 { ex.root(V1_SH.gsh - gdet, false, 1e-12, thr * (1.0 - 1e-12)) } else { 0.0 };
    (w, d0, thr)
}
/// HS (0, 0) in the continuum limit: (G - G^det) S_ph(2 M) - 1 and (G - G^det) S_amp(2 M) - 1 at G^det / G_SH = r; the
/// pseudo-Goldstone boson (the triplet) is a bound state below the threshold of two quasi-particles while the first
/// (the second) is positive, because S_ph and S_amp increase with omega up to the threshold
fn thr_conditions(r: f64, m: f64) -> (f64, f64) {
    let gdet = r * V1_SH.gsh;
    let d0 = gap_closed(V1_SH.gsh + gdet, m);
    let ex = exact_sums(m, d0);
    let gm = V1_SH.gsh - gdet;
    (gm * ex.s_ph_thr() - 1.0, gm * ex.s_amp_thr() - 1.0)
}
/// the position of the neutral member of the triplet in the continuum limit (NaN when it is not bound)
fn wt_exact(r: f64, m: f64) -> f64 {
    let gdet = r * V1_SH.gsh;
    let d0 = gap_closed(V1_SH.gsh + gdet, m);
    let ex = exact_sums(m, d0);
    if thr_conditions(r, m).1 <= 0.0 {
        return f64::NAN;
    }
    ex.root(V1_SH.gsh - gdet, true, d0, 2.0 * ex.meff * (1.0 - 1e-12))
}

/// the coupling ratio r in (lo, hi) at which a function of r changes sign (bisection)
fn bisect_r<Fx: Fn(f64) -> f64>(fx: Fx, lo: f64, hi: f64) -> f64 {
    let (mut a, mut b) = (lo, hi);
    let fa = fx(a);
    for _ in 0..80 {
        let cc = 0.5 * (a + b);
        if (fx(cc) > 0.0) == (fa > 0.0) {
            a = cc;
        } else {
            b = cc;
        }
    }
    0.5 * (a + b)
}

// ================================================================================================ inputs

/// the couplings of stage 2: name -> value (as in stage 4)
fn read_couplings(p: &Path) -> HashMap<String, f64> {
    let s = std::fs::read_to_string(p).unwrap_or_else(|_| panic!("missing {} (run sh_quantum first)", p.display()));
    let mut m = HashMap::new();
    for line in s.lines().skip(1) {
        let v: Vec<&str> = line.split(',').collect();
        if v.len() >= 3 {
            if let Ok(x) = v[2].parse::<f64>() {
                m.insert(v[0].to_string(), x);
            }
        }
    }
    m
}
/// the fields of one line of a CSV file, with the fields in double quotes unquoted (as in stage 4)
fn csv_fields(line: &str) -> Vec<String> {
    let mut out = vec![];
    let mut cur = String::new();
    let mut quoted = false;
    for ch in line.chars() {
        match ch {
            '"' => quoted = !quoted,
            ',' if !quoted => out.push(std::mem::take(&mut cur)),
            _ => cur.push(ch),
        }
    }
    out.push(cur);
    out
}
/// the ground states of stage 4 (data/sh_hg_minimum.csv, second version): method -> (d0, energy) as written
fn read_minimum(p: &Path) -> HashMap<String, (String, String)> {
    let s = std::fs::read_to_string(p).unwrap_or_else(|_| panic!("missing {} (run sh_higgs first)", p.display()));
    let mut lines = s.lines();
    let hdr = csv_fields(lines.next().unwrap());
    let col = |n: &str| hdr.iter().position(|h| h == n).unwrap();
    let (cv, cm, cd, ce) = (col("version"), col("method"), col("d0"), col("energy"));
    let mut out = HashMap::new();
    for line in lines {
        let v = csv_fields(line);
        if v.len() >= hdr.len() && v[cv] == "second" {
            out.insert(v[cm].clone(), (v[cd].clone(), v[ce].clone()));
        }
    }
    out
}

// ================================================================================================ random helpers

/// a random complex antisymmetric 16 x 16 matrix with entries in [-a, a]
fn rand_antisym(rng: &mut Rng, a: f64) -> CMat {
    let mut m = CMat::zeros(N);
    for i in 0..N {
        for j in (i + 1)..N {
            let z = c(a * rng.unit(), a * rng.unit());
            m.set(i, j, z);
            m.set(j, i, -z);
        }
    }
    m
}
/// a random element of SU(2): a0 + i (a . sigma) with a random unit quaternion (a0, a)
fn rand_su2(rng: &mut Rng) -> CMat {
    let mut q = [rng.unit(), rng.unit(), rng.unit(), rng.unit()];
    let n = q.iter().map(|x| x * x).sum::<f64>().sqrt();
    for x in q.iter_mut() {
        *x /= n;
    }
    let mut u = CMat::identity(2).scale_r(q[0]);
    for a in 1..4 {
        u = u.add(&sigma(a).scale(c(0.0, q[a])));
    }
    u
}

// ================================================================================================ main

fn main() {
    let t_start = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sh_det: checks of the determinant interaction and of the mass of the boson of the particle number (stage 7 of \
         the revision, the author's decision D32 (c), Revision_SM_Higgs, PROJECT spacetimes)",
    );
    let cpl = read_couplings(&data.join("sh_couplings.csv"));
    let (gcpl, gp, v_gev) = (cpl["g"], cpl["g_prime"], cpl["v"]);
    let _ = (gcpl, gp);
    let minimum = read_minimum(&data.join("sh_hg_minimum.csv"));
    let mut vals: Vec<(String, String)> = vec![];

    // ------------------------------------------------------------------ DT-1: the engine
    let mut rng = Rng(0x2026_1122_0D32_0C36);
    let gdet_t = 10.0;
    let mut dev_a = 0.0f64;
    let mut dev_b = 0.0f64;
    let mut dev_b_sign = 0.0f64;
    let mut dev_b_rest = 0.0f64;
    for method in [Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Cjt] {
        let mo = model_det(&method, gdet_t, M_LL, NR);
        for _ in 0..4 {
            let kap = rand_antisym(&mut rng, 0.05);
            // (a) the derivative: for a random antisymmetric direction X, dE/dh at h = 0 of E(kap + h X) equals
            // -Re tr(P X^dag) with P the pair self-energy of the determinant terms
            let x = rand_antisym(&mut rng, 1.0);
            let h = 1e-5;
            let ep = det_energy(&mo, &kap.add(&x.scale_r(h)));
            let em = det_energy(&mo, &kap.sub(&x.scale_r(h)));
            let num = (ep - em) / (2.0 * h);
            let p = mo.pair_conj_lin(&kap.conj());
            let ana = -p.mul(&x.dagger()).trace().re;
            dev_a = dev_a.max((num - ana).abs() / ana.abs().max(1e-300));
            // (b) invariance: kap -> U kap U^T for U in SU(2)_L (1_4 (x) u on Psi_LL), SU(2)_R (u (x) 1_4 on the pair of
            // singlets) and the rotations by the relative number; the rotation by the particle number through pi/4,
            // kap -> i kap, changes the sign of the determinant energy and leaves the other terms unchanged
            let e0 = det_energy(&mo, &kap);
            let ul = CMat::block_diag(&CMat::identity(4).kron(&rand_su2(&mut rng)), &CMat::identity(NS));
            let ur = CMat::block_diag(&CMat::identity(NP), &rand_su2(&mut rng).kron(&CMat::identity(4)));
            let th = PI * rng.unit();
            let mut urel = CMat::zeros(N);
            for k in 0..N {
                urel.set(k, k, if k < NP { c(th.cos(), th.sin()) } else { c(th.cos(), -th.sin()) });
            }
            for u in [&ul, &ur, &urel] {
                let k2 = u.mul(&kap).mul(&u.transpose());
                dev_b = dev_b.max((det_energy(&mo, &k2) - e0).abs() / e0.abs());
            }
            let ki = kap.scale(c(0.0, 1.0));
            dev_b_sign = dev_b_sign.max((det_energy(&mo, &ki) + e0).abs() / e0.abs());
            // the other terms: the pairing terms depend on |tr(K kap^T)|^2 and the normal contractions; with drho = 0
            // their energy at i kap equals that at kap
            let mut mo_terms = model_det(&method, 0.0, M_LL, NR);
            mo_terms.pair_pair.clear();
            let drho = CMat::zeros(N);
            let (e_t0, _, _) = mo_terms.interaction(&drho, &kap);
            let (e_t1, _, _) = mo_terms.interaction(&drho, &ki);
            dev_b_rest = dev_b_rest.max((e_t1 - e_t0).abs() / e_t0.abs().max(1e-300));
        }
    }
    rep.add(
        "DT-1a",
        "the pair self-energy of the determinant interaction is minus twice the derivative of its energy with respect to \
         kap^* (the convention of the engine: the first variation of the energy is -Re tr(P d kap^dag)): central \
         differences with the step 1e-5 along random antisymmetric directions at random pair matrices, HS and CJT \
         (relative difference)",
        dev_a,
        1e-7,
    );
    rep.add(
        "DT-1b",
        "the energy of the determinant interaction is invariant under random elements of SU(2)_L (on Psi_LL), of SU(2)_R \
         (on the pair of singlets) and of the rotations by the relative number (kap -> U kap U^T), HS and CJT (relative \
         difference)",
        dev_b,
        1e-12,
    );
    rep.add(
        "DT-1c",
        "the rotation by the particle number through pi/4 (kap -> i kap) changes the sign of the energy of the \
         determinant interaction (relative difference of E(i kap) and -E(kap)) and leaves the energy of the other terms \
         unchanged (relative difference at drho = 0; the two deviations in the note that follows): the convention \
         G^det >= 0 loses no generality, HS and CJT",
        dev_b_sign.max(dev_b_rest),
        1e-12,
    );
    rep.note(&format!("  DT-1c: sign {:.2e}, other terms {:.2e}", dev_b_sign, dev_b_rest));

    // ------------------------------------------------------------------ the five methods, two at a time
    let jobs = methods();
    let schedule: [(usize, Option<usize>); 3] = [(3, Some(4)), (0, Some(1)), (2, None)];
    let mut res: Vec<Option<Vec<StateRes>>> = (0..jobs.len()).map(|_| None).collect();
    for &(ja, jb) in &schedule {
        let idx: Vec<usize> = match jb {
            Some(b) => vec![ja, b],
            None => vec![ja],
        };
        let threads = if idx.len() == 1 { 2 } else { 1 };
        let part: Vec<Vec<StateRes>> = std::thread::scope(|sc| {
            let hs: Vec<_> = idx.iter().map(|&j| {
                let m = jobs[j];
                sc.spawn(move || run_method(m, threads))
            }).collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for (j, r) in idx.into_iter().zip(part) {
            res[j] = Some(r);
        }
    }
    let res: Vec<Vec<StateRes>> = res.into_iter().map(|r| r.unwrap()).collect();
    let all: Vec<&StateRes> = res.iter().flatten().collect();

    // DT-1d: the ratio 0 reproduces the ground states of stage 4 bit for bit
    let mut repro_ok = true;
    let mut repro_detail = vec![];
    for r in res.iter() {
        let s0 = &r[0];
        let name = mname(&s0.method);
        let (d_str, e_str) = minimum.get(&name).cloned().unwrap_or_default();
        let ok = f(s0.d0) == d_str && f(s0.sol.energy) == e_str;
        repro_ok &= ok;
        repro_detail.push(format!("{}: d0 {} energy {}", name, if f(s0.d0) == d_str { "identical" } else { "DIFFERENT" }, if f(s0.sol.energy) == e_str { "identical" } else { "DIFFERENT" }));
    }
    rep.add_bool(
        "DT-1d",
        "with G^det = 0 the program reproduces the ground state of every method of stage 4 bit for bit (d_0 and the energy \
         written with 17 digits, compared with data/sh_hg_minimum.csv): the pair-pair terms added to the engine change \
         nothing in a model without them",
        repro_ok,
        &repro_detail.join("; "),
    );

    // ------------------------------------------------------------------ DT-2: the ground states
    let pos: Vec<&&StateRes> = all.iter().filter(|s| s.ratio > 0.0).collect();
    let res_max = all.iter().map(|s| s.sol.residual).fold(0.0f64, f64::max);
    rep.add(
        "DT-2a",
        "every ground state with the determinant interaction is self-consistent (the largest residual of the self-energies), \
         five methods, G^det / G_SH = 0, 0.01, 0.1, 0.5",
        res_max,
        1e-12,
    );
    let mut cust = 0.0f64;
    let mut charged = 0.0f64;
    let mut phase = 0.0f64;
    for s in &all {
        let (s1, s2) = s.desc.singular();
        cust = cust.max((s1 - s2).abs() / s1);
        charged = charged.max(s.desc.p_psipsi).max(s.desc.p_ss).max(s.desc.n_psis).max(s.desc.n_chichi2).max(s.desc.n_updown);
        // the coefficient of the vacuum direction before the rotation: real (phase 0 or pi) when G^det > 0
        if s.ratio > 0.0 {
            phase = phase.max(s.dcoef.im.abs() / s.d0);
        }
    }
    rep.add(
        "DT-2b",
        "every ground state is the custodial Higgs superfluid: the bidoublet of the pair coefficients has two equal singular \
         values (relative difference; the largest of the three measures), the blocks that carry charge or break the custodial \
         symmetry vanish (norm), and with G^det > 0 the coefficient of the vacuum direction is real before any rotation \
         (|Im| / |d|): the determinant term fixes the phase that the particle number rotated freely",
        cust.max(charged).max(phase),
        1e-9,
    );
    rep.note(&format!("  DT-2b: singular values {:.2e}, charged blocks {:.2e}, phase {:.2e}", cust, charged, phase));
    let mut mono = true;
    let mut mono_detail = vec![];
    for r in res.iter() {
        for k in 1..r.len() {
            mono &= r[k].sol.energy < r[k - 1].sol.energy && r[k].d0 > r[k - 1].d0;
        }
        mono_detail.push(format!(
            "{}: E {} d0 {}",
            mname(&r[0].method),
            r.iter().map(|s| format!("{:.6e}", s.sol.energy)).collect::<Vec<_>>().join(" > "),
            r.iter().map(|s| format!("{:.6}", s.d0)).collect::<Vec<_>>().join(" < ")
        ));
    }
    rep.add_bool(
        "DT-2c",
        "the energy of the ground state falls and d_0 grows with G^det in every method (the determinant term adds to the \
         attraction of the vacuum channel)",
        mono,
        &mono_detail.join("; "),
    );
    let edet_neg = pos.iter().all(|s| s.e_det < 0.0);
    rep.add_bool(
        "DT-2d",
        "the energy of the determinant term in the ground state is negative (G^det > 0)",
        edet_neg,
        &pos.iter().map(|s| format!("{} {}: {:.4e}", mname(&s.method), s.ratio, s.e_det)).collect::<Vec<_>>().join("; "),
    );
    let mut gap_dev = 0.0f64;
    for s in all.iter().filter(|s| is_hs00(&s.method)) {
        let dc = gap_closed(V1_SH.gsh + s.gdet, M_LL);
        gap_dev = gap_dev.max((s.d0 - dc).abs() / dc);
    }
    rep.add(
        "DT-2e",
        "HS (0, 0): d_0 solves the closed-form gap equation with the coupling G_SH + G^det, (G_SH + G^det) I1(M_eff) / (4 pi^2) = \
         1 (relative difference; the grid of 24 radial nodes against the exact integral)",
        gap_dev,
        1e-9,
    );

    // ------------------------------------------------------------------ DT-3: the static response
    let mut ones_ok = true;
    let mut ones_detail = vec![];
    let mut max_other = f64::NEG_INFINITY;
    let mut max_b = f64::NEG_INFINITY;
    let mut asym = 0.0f64;
    for s in &all {
        let n1 = s.resp_a.iter().filter(|x| (*x - 1.0).abs() < 1e-6).count();
        let want = if s.ratio > 0.0 { 3 } else { 4 };
        ones_ok &= n1 == want;
        let mo = s.resp_a.iter().filter(|x| (*x - 1.0).abs() >= 1e-6).fold(f64::NEG_INFINITY, |a, x| a.max(*x));
        max_other = max_other.max(mo);
        max_b = max_b.max(s.resp_b);
        asym = asym.max(s.resp_asym);
        ones_detail.push(format!("{} {}: {} eigenvalues 1, the next {:.6}", mname(&s.method), s.ratio, n1, mo));
    }
    rep.add_bool(
        "DT-3a",
        "the static response in the sector of relative number 0 has exactly three eigenvalues 1 (within 1e-6) for G^det > 0, \
         the Goldstone modes of the three generators of SU(2)_L x U(1)_Y that the vacuum breaks, and four for G^det = 0 \
         (with the mode of the particle number); every other eigenvalue lies below 1, and so does every eigenvalue in the \
         pairs of relative number +-2: the ground state is a local minimum",
        ones_ok && max_other < 1.0 && max_b < 1.0,
        &format!("{}; the largest other eigenvalue {:.6}, the largest in the pairs of relative number +-2 {:.6}; asymmetry of the matrices {:.1e}", ones_detail.join("; "), max_other, max_b, asym),
    );
    let mut ns_dev = 0.0f64;
    let mut ns_rest = 0.0f64;
    let mut ns_detail = vec![];
    for s in all.iter().filter(|s| is_hs00(&s.method)) {
        let ns = s.neutral_static.unwrap();
        let gs = GridSums::new(NR, M_LL, s.d0);
        let (gm, gp) = (V1_SH.gsh - s.gdet, V1_SH.gsh + s.gdet);
        let want = [gp * gs.s_amp(0.0), gm * gs.s_ph(0.0), gm * gs.s_amp(0.0), gp * gs.s_ph(0.0)];
        for k in 0..4 {
            ns_dev = ns_dev.max((ns[k].0 - want[k]).abs());
            ns_rest = ns_rest.max(ns[k].1);
        }
        ns_detail.push(format!(
            "ratio {}: common amplitude {:.10} (closed form {:.10}), common phase {:.10} ({:.10} = (G - G^det)/(G + G^det) {:.10}), relative amplitude {:.10} ({:.10}), relative phase {:.10} ({:.10})",
            s.ratio, ns[0].0, want[0], ns[1].0, want[1], gm / gp, ns[2].0, want[2], ns[3].0, want[3]
        ));
    }
    rep.add(
        "DT-3b",
        "HS (0, 0): the four neutral directions of the pair self-energy are eigenvectors of the static response (the norm of \
         the rest of the response, in the detail) with the eigenvalues of the closed form: (G + G^det) S_amp(0) (common \
         amplitude), (G - G^det) S_ph(0) = (G - G^det)/(G + G^det) (common phase, the mode of the particle number), \
         (G - G^det) S_amp(0) (relative amplitude, the triplet) and (G + G^det) S_ph(0) = 1 (relative phase, a Goldstone \
         mode); largest absolute difference",
        ns_dev,
        1e-8,
    );
    rep.note(&format!("  DT-3b: the largest rest {:.2e}; {}", ns_rest, ns_detail.join("; ")));

    // ------------------------------------------------------------------ DT-4: the susceptibilities
    let mut sym = 0.0f64;
    let mut cust_chi = 0.0f64;
    let mut rho_dev = 0.0f64;
    let mut rank_ok = true;
    let mut chiff_dev = 0.0f64;
    let mut chi_step = 0.0f64;
    for s in &all {
        let ch = &s.chi;
        let sc = ch.iter().flatten().fold(0.0f64, |a, x| a.max(x.abs()));
        for a in 0..8 {
            for b in 0..8 {
                sym = sym.max((ch[a][b] - ch[b][a]).abs() / sc);
            }
        }
        for a in 0..3 {
            cust_chi = cust_chi.max((ch[a][a] - ch[a + 3][a + 3]).abs() / sc).max((ch[a][a] + ch[a][a + 3]).abs() / sc).max((ch[a][a] - ch[0][0]).abs() / sc);
        }
        rho_dev = rho_dev.max((ch[0][0] / ch[2][2] - 1.0).abs());
        let ev = chi_eigs(ch);
        let npos = ev.iter().filter(|x| **x > 1e-6 * sc).count();
        let nzero = ev.iter().filter(|x| x.abs() <= 1e-6 * sc).count();
        rank_ok &= npos == 4 && nzero == 4;
        chi_step = chi_step.max(s.chi_dev);
        if is_hs00(&s.method) {
            let gs = GridSums::new(NR, M_LL, s.d0);
            chiff_dev = chiff_dev.max((ch[7][7] - 2.0 * s.d0 * s.d0 * gs.s3()).abs() / ch[7][7]);
        }
    }
    rep.add(
        "DT-4a",
        "the susceptibilities are symmetric (Hellmann and Feynman) and have the custodial structure chi(T^a) = chi(T^a_R) = \
         -chi(T^a, T^a_R), equal for a = 1, 2, 3; the largest relative deviation (the two steps of the chemical potential \
         differ by the relative amount in the detail)",
        sym.max(cust_chi),
        1e-7,
    );
    rep.note(&format!("  DT-4a: symmetry {:.2e}, custodial {:.2e}, the two steps {:.2e}", sym, cust_chi, chi_step));
    rep.add(
        "DT-4b",
        "rho_SH = chi^11 / chi^33 = 1 in every method and at every G^det: the determinant interaction keeps the custodial \
         symmetry (largest |rho - 1|)",
        rho_dev,
        1e-7,
    );
    rep.add_bool(
        "DT-4c",
        "the susceptibility matrix of the eight generators has exactly four positive eigenvalues and four zero ones (below \
         1e-6 of its largest element) in every method and at every G^det: the stiffness of the particle number stays (its \
         phase is still that of the condensate), the explicit breaking gives the mode a mass, not the generator a zero \
         susceptibility",
        rank_ok,
        "",
    );
    rep.add(
        "DT-4d",
        "HS (0, 0): the susceptibility of the particle number has the closed form chi^FF = 2 d_0^2 sum_q w / E^3 (the stiffness \
         of the phase of the pairs on the grid; relative difference)",
        chiff_dev,
        1e-6,
    );

    // ------------------------------------------------------------------ DT-5: the linear response
    let mut gold_ok = true;
    let mut gold_detail = vec![];
    let mut pg_ok = true;
    for s in &all {
        let md = s.modes.as_ref().unwrap();
        let want = if s.ratio > 0.0 { 3 } else { 4 };
        gold_ok &= md.goldstone == want;
        let zg = md.best(1);
        if (s.ratio - 0.01).abs() < 1e-12 {
            pg_ok &= md.bound(1).map(|z| z.mult == 1).unwrap_or(false);
        }
        gold_detail.push(format!(
            "{} {}: {} zero singular values at omega = 0 (the next {:.2e}); threshold {:.6}; pseudo-Goldstone {}",
            mname(&s.method),
            s.ratio,
            md.goldstone,
            md.sing0.get(md.goldstone).cloned().unwrap_or(f64::NAN),
            md.threshold,
            zg.map(|z| format!("omega {:.10} sigma {:.1e} multiplicity {} overlap {:.6} margin {:.1e}", z.omega, z.sigma, z.mult, z.ov[1], z.margin)).unwrap_or("none below the threshold".into())
        ));
    }
    rep.add_bool(
        "DT-5a",
        "the linear response in the sector of relative number 0: at omega = 0 exactly three singular values of 1 - chi(0) F \
         vanish (below 1e-6) for G^det > 0 and four for G^det = 0, in every method",
        gold_ok,
        &gold_detail.join("; "),
    );
    rep.add_bool(
        "DT-5b",
        "at G^det / G_SH = 0.01, in every method, below the lowest transition energy of the sector (by more than 1e-4, \
         relative) there is a zero of 1 - chi(omega) F whose null space contains the common phase of the two neutral pair \
         fields (squared overlap above 0.5), of multiplicity 1: the boson of the particle number has become a massive \
         pseudo-Goldstone boson, a custodial singlet (the larger couplings in the detail of DT-5a and in \
         data/sh_det_modes.csv)",
        pg_ok,
        "",
    );

    // ------------------------------------------------------------------ DT-6: HS (0, 0) closed forms
    let mut cf_dev = 0.0f64;
    let mut cf_detail = vec![];
    for s in all.iter().filter(|s| is_hs00(&s.method) && s.ratio > 0.0) {
        let md = s.modes.as_ref().unwrap();
        let gs = GridSums::new(NR, M_LL, s.d0);
        let thr = gs.threshold();
        let gm = V1_SH.gsh - s.gdet;
        let wg = gs.root(gm, false, 1e-12, thr * (1.0 - 1e-12));
        let wt = gs.root(gm, true, s.d0, thr * (1.0 - 1e-12));
        let zg = md.best(1).map(|z| z.omega).unwrap_or(f64::NAN);
        let zh = md.best(0).map(|z| z.omega).unwrap_or(f64::NAN);
        let zt = md.best(2).map(|z| z.omega).unwrap_or(f64::NAN);
        let mut d = (zg - wg).abs() / wg;
        d = d.max((zh - s.d0).abs() / s.d0);
        if wt.is_finite() || zt.is_finite() {
            d = d.max((zt - wt).abs() / wt);
        }
        cf_dev = cf_dev.max(if d.is_finite() { d } else { f64::INFINITY });
        cf_detail.push(format!(
            "ratio {}: pseudo-Goldstone {:.10} (closed form {:.10}), amplitude {:.10} (d_0 {:.10}), triplet {:.10} (closed form {:.10}), threshold {:.10}",
            s.ratio, zg, wg, zh, s.d0, zt, wt, thr
        ));
    }
    rep.add(
        "DT-6",
        "HS (0, 0): the linear response of the whole sector of relative number 0 puts the pseudo-Goldstone boson at the root \
         of (G - G^det) S_ph(omega) = 1, the amplitude mode at omega = d_0 (the closed form of stage 4 with the coupling \
         G + G^det) and the neutral member of the triplet at the root of (G - G^det) S_amp(omega) = 1, the sums on the same \
         grid (largest relative difference)",
        cf_dev,
        1e-9,
    );
    rep.note(&format!("  DT-6: {}", cf_detail.join("; ")));

    // ------------------------------------------------------------------ the scan of HS (0, 0)
    let scan_r: Vec<f64> = vec![0.001, 0.002, 0.005, 0.01, 0.02, 0.05, 0.1, 0.15, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
    let scan: Vec<ScanPt> = std::thread::scope(|sc| {
        let hs: Vec<_> = scan_r.chunks((scan_r.len() + 1) / 2).map(|ch| {
            sc.spawn(move || ch.iter().map(|&r| scan_point(r, M_LL, NR)).collect::<Vec<_>>())
        }).collect();
        hs.into_iter().flat_map(|h| h.join().unwrap()).collect()
    });
    // the couplings at which the pseudo-Goldstone boson reaches d_0 / 2, d_0 and the threshold (continuum limit)
    let r_half = bisect_r(|r| { let (w, d0, _) = wg_exact(r, M_LL); w - 0.5 * d0 }, 1e-6, 0.7);
    let r_cross = bisect_r(|r| { let (w, d0, _) = wg_exact(r, M_LL); w - d0 }, 0.05, 0.7);
    // the ends of the bound states in the continuum limit: the pseudo-Goldstone boson and the triplet reach the threshold
    // of two quasi-particles where (G - G^det) S_ph(2 M) = 1 and (G - G^det) S_amp(2 M) = 1
    let r_end = bisect_r(|r| thr_conditions(r, M_LL).0, 0.3, 0.99);
    let r_trip_end = bisect_r(|r| thr_conditions(r, M_LL).1, 1e-6, 0.5);
    println!(
        "crossings: m_G = d0/2 at r {:.6}, m_G = d0 at r {:.6}, the boson reaches the threshold at r {:.6}, the triplet at r {:.6}",
        r_half, r_cross, r_end, r_trip_end
    );
    // DT-7: Dashen
    let mut dash_lin = 0.0f64;
    let mut dash_detail = vec![];
    for p in scan.iter().filter(|p| p.r <= 0.01) {
        let dev = (p.w_g * p.w_g / (p.w_dashen * p.w_dashen) - 1.0).abs();
        dash_lin = dash_lin.max(dev / p.r);
        dash_detail.push(format!("r {}: (m_G / m_Dashen)^2 - 1 = {:.3e} ({:.4} r)", p.r, p.w_g * p.w_g / (p.w_dashen * p.w_dashen) - 1.0, (p.w_g * p.w_g / (p.w_dashen * p.w_dashen) - 1.0) / p.r));
    }
    let mut dash_all = 0.0f64;
    let mut dash_all_detail = vec![];
    for s in all.iter().filter(|s| (s.ratio - 0.01).abs() < 1e-12) {
        let zg = s.modes.as_ref().unwrap().best(1).map(|z| z.omega).unwrap_or(f64::NAN);
        let wd = (-16.0 * s.e_det / s.chi[7][7]).sqrt();
        let dev = (zg * zg / (wd * wd) - 1.0).abs();
        dash_all = dash_all.max(if dev.is_finite() { dev } else { f64::INFINITY });
        dash_all_detail.push(format!("{}: {:.3e}", mname(&s.method), zg * zg / (wd * wd) - 1.0));
    }
    rep.add(
        "DT-7a",
        "the relation of Dashen (of Gell-Mann, Oakes and Renner), m_G^2 chi^FF = -16 E^det, holds to first order in G^det: in \
         the scan of HS (0, 0) the relative difference of the squared masses of the linear response and of the relation, \
         divided by G^det / G_SH, stays bounded as G^det -> 0 (r = 0.001 to 0.01; the largest quotient)",
        dash_lin,
        1.0,
    );
    rep.note(&format!("  DT-7a: {}", dash_detail.join("; ")));
    rep.add(
        "DT-7b",
        "in every method at G^det / G_SH = 0.01 the squared mass of the pseudo-Goldstone boson of the linear response and that \
         of the relation of Dashen, -16 E^det / chi^FF with the energy of the determinant term and the susceptibility of the \
         particle number of the same ground state, differ by less than 1 per cent",
        dash_all,
        1e-2,
    );
    rep.note(&format!("  DT-7b: {}", dash_all_detail.join("; ")));
    // DT-8: the scan
    let mut scan_ok = true;
    let mut scan_dev = 0.0f64;
    let mut prev = 0.0f64;
    let mut scan_sc = 0.0f64;
    for p in &scan {
        let found = p.w_g.is_finite();
        let below = p.wc_g.is_finite();
        // the sector of the neutral pairs of the vacuum holds one Goldstone mode, the relative phase of the two neutral
        // pair fields (the one absorbed by Z); the charged Goldstone modes live in the charged pairs, outside it
        scan_ok &= found == below && p.gold == 1 && (!found || p.mult_g == 1);
        if found {
            scan_ok &= p.w_g > prev;
            prev = p.w_g;
            scan_dev = scan_dev.max((p.w_g - p.wc_g).abs() / p.wc_g);
        }
        scan_sc = scan_sc.max(p.sc_dev);
    }
    rep.add_bool(
        "DT-8a",
        "the scan of HS (0, 0) (G^det / G_SH from 0.001 to 0.7), in the sector of the neutral pairs of the vacuum: at every \
         point one Goldstone mode at omega = 0 (the relative phase, absorbed by Z; the charged ones lie outside this sector), \
         and the pseudo-Goldstone boson (multiplicity 1) is found by the linear response exactly where the closed form has a \
         root below the threshold; its mass grows monotonically with G^det",
        scan_ok,
        &format!("the largest relative difference from the closed form {:.1e}; the self-consistency of the closed-form states {:.1e}", scan_dev, scan_sc),
    );
    rep.add(
        "DT-8b",
        "the scan of HS (0, 0): the mass of the pseudo-Goldstone boson of the linear response equals the root of the closed \
         form on the grid (largest relative difference)",
        scan_dev,
        1e-9,
    );
    rep.note(&format!(
        "  the crossings (HS (0, 0), exact integrals): m_G = d_0 / 2 at G^det / G_SH = {:.6}, m_G = d_0 at {:.6}, the boson reaches the threshold 2 M_eff at {:.6}; the custodial triplet reaches it at {:.6}",
        r_half, r_cross, r_end, r_trip_end
    ));
    // DT-9: the grid
    let p24 = scan_point(0.1, M_LL, NR);
    let p48 = scan_point(0.1, M_LL, 48);
    let (wex, _, _) = wg_exact(0.1, M_LL);
    let gdev = (p24.w_g - p48.w_g).abs() / p48.w_g;
    rep.add(
        "DT-9",
        "the grid: 24 and 48 radial nodes give the same mass of the pseudo-Goldstone boson, HS (0, 0) at G^det / G_SH = 0.1 \
         (relative difference; the continuum limit of the closed form in the detail)",
        gdev,
        1e-6,
    );
    rep.note(&format!("  DT-9: 24 nodes {:.12}, 48 nodes {:.12}, exact integrals {:.12}", p24.w_g, p48.w_g, wex));

    // ------------------------------------------------------------------ the data files
    let mut rows_states = vec![];
    let mut rows_modes = vec![];
    for s in &all {
        let ch = &s.chi;
        let v = 2.0 * ch[0][0].sqrt();
        let lam_gev = v_gev / v;
        let md = s.modes.as_ref().unwrap();
        let wd = if s.ratio > 0.0 { (-16.0 * s.e_det / ch[7][7]).sqrt() } else { 0.0 };
        // the modes that are bound states (below the threshold by more than 1e-4, relative); at G^det = 0 the boson of
        // the particle number is the massless Goldstone mode and the frequencies were not scanned
        let zg = md.bound(1).map(|z| z.omega).unwrap_or(if s.ratio > 0.0 { f64::NAN } else { 0.0 });
        let zh = md.bound(0).map(|z| z.omega).unwrap_or(f64::NAN);
        let zt = md.bound(2).map(|z| z.omega).unwrap_or(f64::NAN);
        rows_states.push(vec![
            mname(&s.method),
            f(s.ratio),
            f(s.gdet),
            f(s.d0),
            f(0.5 * (s.phi.0 + s.phi.1)),
            f(s.sol.energy),
            f(s.e_det),
            f(s.gap),
            f(s.desc.mass[0]),
            f(s.desc.mass[2]),
            f(ch[0][0]),
            f(ch[2][2]),
            f(ch[7][7]),
            f(v),
            f(lam_gev),
            f(ch[0][0] / ch[2][2]),
            f(md.threshold),
            f(zg),
            f(wd),
            f(zh),
            f(zt),
            f(zg * lam_gev),
            f(wd * lam_gev),
            f(zh * lam_gev),
            f(zt * lam_gev),
            f(zg / v),
            f(md.threshold * lam_gev),
            format!("{}", md.goldstone),
            format!("{}", s.sol.iterations),
            f(s.seconds),
        ]);
        for z in &md.zeros {
            rows_modes.push(vec![
                mname(&s.method),
                f(s.ratio),
                f(z.omega),
                f(z.sigma),
                format!("{}", z.mult),
                f(z.ov[0]),
                f(z.ov[1]),
                f(z.ov[2]),
                f(z.ov[3]),
                NEUTRAL_NAMES[(0..4).fold(0, |b, k| if z.ov[k] > z.ov[b] { k } else { b })].to_string(),
                f(z.margin),
                f(md.threshold),
            ]);
        }
    }
    write_csv(
        &data.join("sh_det_states.csv"),
        &[
            "method", "G_det_over_G_SH", "G_det", "d0", "phi0", "energy", "E_det", "gap", "M_psi", "M_chi", "chi_T1T1", "chi_T3T3", "chi_FF",
            "v_SH", "Lambda_GeV", "rho", "threshold", "m_G_RPA", "m_G_Dashen", "M_H_RPA", "M_trip_RPA", "m_G_RPA_GeV", "m_G_Dashen_GeV",
            "M_H_RPA_GeV", "M_trip_RPA_GeV", "m_G_over_v_SH", "threshold_GeV", "goldstone_count", "iterations", "seconds",
        ],
        &rows_states,
    );
    write_csv(
        &data.join("sh_det_modes.csv"),
        &["method", "G_det_over_G_SH", "omega", "sigma", "multiplicity", "ov_common_amplitude", "ov_common_phase", "ov_relative_amplitude", "ov_relative_phase", "largest", "margin", "threshold"],
        &rows_modes,
    );
    let mut rows_scan = vec![];
    for p in scan.iter().chain([&p48]) {
        let lam = v_gev / p.v;
        rows_scan.push(vec![
            f(p.r),
            f(p.gdet),
            format!("{}", if std::ptr::eq(p, &p48) { 48 } else { NR }),
            f(p.d0),
            f(p.meff),
            f(p.e_det),
            f(p.chi_ff),
            f(p.v),
            f(lam),
            f(p.w_g),
            f(p.wc_g),
            f(p.w_dashen),
            f(p.w_h),
            f(p.w_t),
            f(p.wc_t),
            f(p.thr),
            f(p.w_g * lam),
            f(p.w_dashen * lam),
            f(p.w_h * lam),
            f(p.w_t * lam),
            f(p.thr * lam),
            f(p.w_g / p.d0),
            f(p.w_h / p.v),
            format!("{}", p.gold),
            f(p.wx_g),
            f(p.wx_t),
            f(p.margin_g),
            f(p.wx_g * lam),
        ]);
    }
    write_csv(
        &data.join("sh_det_scan.csv"),
        &[
            "G_det_over_G_SH", "G_det", "radial_nodes", "d0", "M_eff", "E_det", "chi_FF", "v_SH", "Lambda_GeV", "m_G_RPA", "m_G_closed",
            "m_G_Dashen", "M_H_RPA", "M_trip_RPA", "M_trip_closed", "threshold", "m_G_GeV", "m_G_Dashen_GeV", "M_H_GeV", "M_trip_GeV",
            "threshold_GeV", "m_G_over_d0", "M_H_over_v_SH", "goldstone_count", "m_G_continuum", "M_trip_continuum", "margin_G",
            "m_G_continuum_GeV",
        ],
        &rows_scan,
    );
    vals.push(("r_half".into(), f(r_half)));
    vals.push(("r_cross".into(), f(r_cross)));
    vals.push(("r_end".into(), f(r_end)));
    vals.push(("r_trip_end".into(), f(r_trip_end)));
    vals.push(("w_g_24".into(), f(p24.w_g)));
    vals.push(("w_g_48".into(), f(p48.w_g)));
    vals.push(("w_g_exact_0.1".into(), f(wex)));
    vals.push(("dashen_lin_max".into(), f(dash_lin)));
    vals.push(("v_gev".into(), f(v_gev)));
    let rows_vals: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), v.clone()]).collect();
    write_csv(&data.join("sh_det_values.csv"), &["name", "value"], &rows_vals);

    let longest = all.iter().map(|s| s.seconds).fold(0.0f64, f64::max);
    rep.note(&format!(
        "  run time {:.0} s on 2 threads; the longest single state (ground state, static response, susceptibilities, linear response) {:.0} s",
        t_start.elapsed().as_secs_f64(),
        longest
    ));
    rep.write(&data.join("hck_sh_det_checks.txt"));
    let (np, nt) = rep.counts();
    println!("sh_det: {} of {} checks pass", np, nt);
}
