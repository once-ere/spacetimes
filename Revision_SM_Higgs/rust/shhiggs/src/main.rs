//! sh_higgs -- stage 4 of the revision of the hypotheses, second version (folder Revision_SM_Higgs of PROJECT spacetimes;
//! the former stages 4 and 5, merged): the effective potential of the pair fields of the Higgs superfluid, the
//! susceptibilities of the superfluid to the chemical potentials of the generators of the symmetry group, the scale of
//! the breaking and the masses of the gauge bosons, the parameter rho, and the mass of the amplitude mode, compared with
//! the Standard Model; the first version of the hypotheses (one singlet) is computed alongside as the comparison.
//!
//! The model is that of stage 3 (the program sh_superfluid, whose file model.rs this program includes unchanged): the
//! isodoublet Psi_LL (components 0..7) and the singlets chi_SH (8..11) and chi'_SH (12..15), all of mass 0.2 (the cutoff
//! is 1), the potential V^(1)_SH (G^S_SF = -8, G^V_SF = 0.8, G_SH = 100.8 for both pairing terms; in the first version
//! the pairing term of chi'_SH is switched off) and the methods HS (1, 1), HS (0, 1), HS (0, 0), KS and CJT.
//!
//! What it computes, for each method and each version:
//!   * the ground state, from the start of stage 3 (second version: the custodial start 0.3 i tau^2; first version:
//!     the start along Phi^d), with the phase of its pair self-energy rotated away (a rotation by the particle number);
//!   * the effective potential V(phi) along the direction of the vacuum: the energy of the lowest state with the pair
//!     amplitude phi = |<Pi^d_SH>| (in the second version also = |<Pi'^u_SH>|).  By a Lagrange multiplier such a state is
//!     the ground state of the BdG matrix with the pair self-energy d P_dir, P_dir = K'_u^dag - K_d^dag (second version)
//!     or K_d^dag (first version), for some d, with the normal self-energy made self-consistent.  The curve is computed
//!     for 33 values of d from 0 to 1.6 d_0 (d_0: the ground state), and V'(phi_0), V''(phi_0) by central differences in
//!     d (parametric formula; two steps and their Richardson combination);
//!   * the susceptibilities chi^{ab} = d<Q_a>/d mu_b of the ground state to the chemical potentials mu_b of the eight
//!     generators T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot: the ground states of H - mu_b Q_b are computed for
//!     mu_b = +-h, +-2h (h = 0.005) and the charge densities <Q_a> = tr(Q_a rho) are differentiated (central differences,
//!     Richardson).  By the theorem of Hellmann and Feynman d E / d mu_a = -<Q_a> at a stationary state, so chi^{ab} is
//!     also -d^2 E / d mu_a d mu_b and must be symmetric; the row and the column of the hypercharge
//!     Y = T^3_R + R / 4 follow by linearity.  The number of nonzero eigenvalues of chi (on the generators that are
//!     symmetries of the version) is the number of broken generators.  The mass matrix of (W^1, W^2, W^3, B) is
//!     M^2_{ab} = g_a g_b chi^{ab} with the couplings of the input file data/sh_couplings.csv (stage 2); the scale
//!     v_SH = 2 (chi^{11})^(1/2) (from M_W = g v_SH / 2); rho from the eigenvalues; the massless photon; the conversion to
//!     GeV by v_SH = v = 246.22 GeV.  The chemical potentials are the time components of the gauge fields: the normal
//!     vacuum, whose spectrum has a gap, does not respond to them at all, so the artifact of the sharp cutoff in the
//!     response to the space components (remark R10 of stage 3) does not enter;
//!   * the mass of the amplitude mode by the effective Lagrangian, M_EFT = (V''(phi_0) phi_0^2 / (4 chi^{33}))^(1/2)
//!     (the kinetic coefficient of the pair fields from the phase stiffness: Phi^d has the charge -1/2 and Phi'^u the
//!     charge +1/2 of T^3), and, in HS (0, 1) and HS (0, 0), where it is a bound state below the continuum of two paired
//!     quasi-particles, by the linear response at a frequency (module sfcore::rpa_n): the zero of the smallest singular
//!     value of 1 - chi(omega) F in the sector of relative number 0, identified by the overlap of the amplitude of the
//!     vacuum direction with the null space there (whose dimension is the multiplicity of the mode).
//! A scan of G_SH / G^crit_SH and of the mass m (HS (0, 0), where only the pairing terms act) gives the ratio of the
//! amplitude mode to v_SH and rho over a range of couplings, in both versions.
//!
//! The checks (report data/hck_sh_higgs_checks.txt):
//!   H-1   the state with the fixed pair self-energy d_0 P_dir is the ground state: its energy, and the pair self-energy
//!         that its pair amplitude produces equals d_0 P_dir (self-consistency), ten computations;
//!   H-2   phi_0 is a minimum of V: V'(phi_0) vanishes to the order of the steps, V''(phi_0) > 0, the two steps agree;
//!         in the second version |<Pi^d_SH>| = |<Pi'^u_SH>| along the whole curve;
//!   H-3   the Mexican hat: V(0) = 0 (the normal vacuum), V decreases from phi = 0 to phi_0 and increases beyond it;
//!   H-4   the susceptibilities: symmetric (Hellmann-Feynman), the two steps agree, KS equals CJT; the number of broken
//!         generators (4 in the second version among the eight generators, 3 in the first among T^1, T^2, T^3, R, F_tot);
//!         the unbroken generators of stage 3 (T^a + T^a_R and R in the second version) have zero susceptibility;
//!         the custodial structure chi(T^a) = chi(T^a_R) = -chi(T^a, T^a_R) in the second version; the row of
//!         Q = T^3 + Y vanishes in both versions (the photon is massless);
//!   H-5   the mass matrix: one zero eigenvalue with the eigenvector of the photon, and a double eigenvalue g^2 chi^{11};
//!   H-6   rho from the eigenvalues equals chi^{11} / chi^{33}; in the second version rho = 1;
//!   H-7   the amplitude mode by the linear response (HS (0, 1), HS (0, 0)): at omega = 0 exactly four singular values
//!         vanish in the second version and three in the first (the Goldstone modes); below the threshold of two
//!         quasi-particles there is a zero whose null space contains the amplitude of the vacuum direction; HS (0, 1)
//!         and HS (0, 0) give the same frequency;
//!   H-8   the scan: the closed-form gap equation gives the self-consistent pair self-energy; the amplitude mode is found
//!         in every point; the point of the standard couplings reproduces the main computation;
//!   H-9   the grid: 24 and 48 radial nodes give the same v_SH (HS (0, 0), CJT) and the same amplitude mode (HS (0, 0)).
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_higgs
//! Output in data/: hck_sh_higgs_checks.txt and the data files sh_hg_*.csv.  Run time about half an hour on two threads.
#[allow(dead_code)]
#[path = "../../shsuperfluid/src/model.rs"]
mod model;

use model::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{sigma, t_gen};
use sfcore::methods::{grid, Method};
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::rpa_n::{kernel_n, ResponseN, SectorN};
use sfcore::uniform_n::{FieldBlock, ModelN, SolutionN};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;
use std::time::Instant;

/// the potential of stage 3 in the second version (both pairing terms, decision D31) and in the first version
const V1_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 100.8 };
const V1_SH_FIRST: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 0.0 };
/// the self-consistency: mixing, tolerance of the residual of the self-energies, maximum of iterations, and the size
/// below which an element of a self-energy is set to zero after each step (as in stage 3: a channel that no term feeds
/// decays geometrically, and the squares of its elements would underflow in the eigensolver)
const MIX: f64 = 0.6;
const TOL: f64 = 1e-13;
const MAXIT: usize = 40000;
const FLUSH: f64 = 1e-30;
/// the step of the chemical potentials (and twice it, for the Richardson combination)
const H_MU: f64 = 0.005;

/// the two versions of the revised hypotheses
#[derive(Clone, Copy, PartialEq, Debug)]
enum Ver {
    Second,
    First,
}
impl Ver {
    fn pot(&self) -> PotSh {
        match self {
            Ver::Second => V1_SH,
            Ver::First => V1_SH_FIRST,
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Ver::Second => "second",
            Ver::First => "first",
        }
    }
}

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
/// the methods in which only the pairing terms act on the Higgs ground state (no normal self-energy)
fn pairing_only(m: &Method) -> bool {
    matches!(m, Method::Hs { s_s, .. } if *s_s == 0.0)
}

/// I1(M) = int_0^1 q^2 dq / sqrt(q^2 + M^2) (closed form; the cutoff is 1)
fn i1(mq: f64) -> f64 {
    0.5 * ((1.0 + mq * mq).sqrt() - mq * mq * (1.0 / mq).asinh())
}

/// the model of a method and a potential with the mass m of all three fields on nr radial nodes
fn model_m(method: &Method, pot: &PotSh, m: f64, nr: usize) -> ModelN {
    let blocks = vec![
        FieldBlock { off: 0, ncomp: NP, mass: m },
        FieldBlock { off: OFF_CHI, ncomp: NX, mass: m },
        FieldBlock { off: OFF_CHI2, ncomp: NX, mass: m },
    ];
    ModelN::new(blocks, 0.0, terms_sh(method, pot), grid(nr))
}

/// a copy of a model with an external one-body term Psi^dag X Psi (X Hermitian): for X = -mu Q the Hamiltonian
/// H - mu Q of a chemical potential mu of the charge Q
fn with_ext(mo: &ModelN, x: &CMat) -> ModelN {
    let mut m2 = ModelN::new(mo.blocks.clone(), mo.mu, mo.terms.clone(), mo.grid.clone());
    m2.ext_n = Some(x.clone());
    m2
}

/// the self-consistent solution by the iteration of stage 3 (linear mixing with the weight MIX, N made Hermitian and P
/// antisymmetric after each step, elements below FLUSH set to zero); with `fixed_p` the pair self-energy is held fixed and
/// only the normal self-energy is iterated (the constrained states of the effective potential)
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

/// the direction of the vacuum in the pair self-energy: second version K'_u^dag - K_d^dag (the bidoublet i tau^2 of
/// stage 3, D_d = -D'_u), first version K_d^dag (the condensate of Phi^d)
fn p_dir(ver: Ver) -> CMat {
    match ver {
        Ver::Second => k_ds(1, 0).dagger().sub(&k_ds(0, 1).dagger()),
        Ver::First => k_ds(0, 1).dagger(),
    }
}
/// the complex coefficient d of the vacuum direction in a pair self-energy, d = <P_dir, P> / <P_dir, P_dir>
fn vac_coef(p: &CMat, ver: Ver) -> C64 {
    let pd = p_dir(ver);
    pd.dagger().mul(p).trace().scale(1.0 / pd.dagger().mul(&pd).trace().re)
}
/// the pair amplitudes |<Pi^d_SH>| = |tr(K_d kap^T)| / 2 and |<Pi'^u_SH>| = |tr(K'_u kap^T)| / 2
fn phis(kap: &CMat) -> (f64, f64) {
    (0.5 * k_ds(0, 1).mul(&kap.transpose()).trace().abs(), 0.5 * k_ds(1, 0).mul(&kap.transpose()).trace().abs())
}
/// the pair amplitude phi of the vacuum direction: |<Pi^d_SH>| (equal to |<Pi'^u_SH>| in the second version)
fn phi_of(kap: &CMat, ver: Ver) -> f64 {
    let (a, b) = phis(kap);
    match ver {
        Ver::Second => 0.5 * (a + b),
        Ver::First => a,
    }
}

/// the state with the fixed pair self-energy d P_dir and the normal self-energy made self-consistent (from n0)
fn fixed_d(mo: &ModelN, ver: Ver, d: f64, n0: &CMat) -> SolutionN {
    solve_clean(mo, n0, &p_dir(ver).scale_r(d), true)
}

/// V'(phi_0) and V''(phi_0) by central differences in d with the step h (parametric: V' = E_d / phi_d,
/// V'' = (E_dd - V' phi_dd) / phi_d^2)
fn curvature(mo: &ModelN, ver: Ver, d0: f64, n0: &CMat, h: f64) -> (f64, f64) {
    let s0 = fixed_d(mo, ver, d0, n0);
    let sp = fixed_d(mo, ver, d0 + h, n0);
    let sm = fixed_d(mo, ver, d0 - h, n0);
    let (p0, pp, pm) = (phi_of(&s0.kap, ver), phi_of(&sp.kap, ver), phi_of(&sm.kap, ver));
    let (e_d, e_dd) = ((sp.energy - sm.energy) / (2.0 * h), (sp.energy - 2.0 * s0.energy + sm.energy) / (h * h));
    let (p_d, p_dd) = ((pp - pm) / (2.0 * h), (pp - 2.0 * p0 + pm) / (h * h));
    let v1 = e_d / p_d;
    (v1, (e_dd - v1 * p_dd) / (p_d * p_d))
}

/// the eight generators of the global group of the hypotheses on the sixteen components, in the order T^1, T^2, T^3,
/// T^1_R, T^2_R, T^3_R, R, F_tot (as in stage 3)
fn generators() -> Vec<CMat> {
    let mut g: Vec<CMat> = (1..=3).map(|a| emb_psi(&t_gen(a))).collect();
    for a in 1..=3 {
        g.push(emb_pair(&sigma(a).scale_r(0.5).kron(&CMat::identity(4))));
    }
    g.push(CMat::block_diag(&CMat::identity(NP), &CMat::identity(NS).scale_r(-1.0)));
    g.push(CMat::identity(N));
    g
}
const GEN_NAMES: [&str; 8] = ["T1", "T2", "T3", "T1R", "T2R", "T3R", "R", "F"];
/// the coefficients of the hypercharge Y = T^3_R + R / 4 and of the electric charge Q = T^3 + Y in the eight generators
const Y_COEF: [f64; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.25, 0.0];
const Q_COEF: [f64; 8] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.25, 0.0];

/// the charge densities <Q_a> = tr(Q_a rho) of a state (rho measured from the normal vacuum)
fn charges(gens: &[CMat], drho: &CMat) -> Vec<f64> {
    gens.iter().map(|q| q.mul(drho).trace().re).collect()
}

/// the susceptibilities chi^{ab} = d<Q_a>/d mu_b of a ground state (8 x 8, chi[a][b]): the ground states of H - mu Q_b
/// for mu = +-h, +-2h, from the ground state; central differences combined by Richardson; returns the matrix and the
/// largest difference of the two steps relative to the largest element
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

/// the bilinear form chi between two combinations of the generators
fn chi_form(chi: &[Vec<f64>], u: &[f64], v: &[f64]) -> f64 {
    let mut s = 0.0;
    for a in 0..u.len() {
        for b in 0..v.len() {
            s += u[a] * chi[a][b] * v[b];
        }
    }
    s
}
fn unit8(a: usize) -> Vec<f64> {
    let mut e = vec![0.0; 8];
    e[a] = 1.0;
    e
}

/// the eigenvalues of the symmetric part of chi restricted to a list of generators (ascending)
fn chi_eigs(chi: &[Vec<f64>], idx: &[usize]) -> Vec<f64> {
    let k = idx.len();
    let mut m = CMat::zeros(k);
    for (i, &a) in idx.iter().enumerate() {
        for (j, &b) in idx.iter().enumerate() {
            m.set(i, j, c(0.5 * (chi[a][b] + chi[b][a]), 0.0));
        }
    }
    eigh(&m).0
}

/// the sector of relative number 0: the normal blocks of Psi_LL and of the pair of singlets and the pairs of Psi_LL
/// with the singlets (in the first version the block of chi'_SH is inactive: chi'_SH is free and its pairs have no
/// interaction)
fn sector_r0(ver: Ver) -> SectorN {
    let ps = psi_indices();
    match ver {
        Ver::Second => {
            let ss = sing_indices();
            SectorN::new(N, &[ps.clone(), ss.clone()], &[(ps, ss)])
        }
        Ver::First => {
            let cs = species_indices(2);
            SectorN::new(N, &[ps.clone(), cs.clone()], &[(ps, cs)])
        }
    }
}
/// the sector of the neutral pairs of the vacuum alone (down with chi_SH, and in the second version up with chi'_SH):
/// exact for the amplitude mode when only the pairing terms act (HS (0, 0)), because the kernel then reads only the
/// projections onto the pair matrices and these pairs do not mix with the charged ones
fn sector_vac_pairs(ver: Ver) -> SectorN {
    let (up, dn, c1, c2) = (species_indices(0), species_indices(1), species_indices(2), species_indices(3));
    match ver {
        Ver::Second => SectorN::new(N, &[], &[(dn, c1), (up, c2)]),
        Ver::First => SectorN::new(N, &[], &[(dn, c1)]),
    }
}

/// the amplitude of the vacuum direction in the field amplitudes of a sector: (dP, dPbar) = (P_dir, P_dir^dag),
/// normalized
fn amp_vec(sec: &SectorN, ver: Ver) -> Vec<C64> {
    let z = CMat::zeros(N);
    let pd = p_dir(ver);
    let v = sec.to_vec(&z, &pd, &pd.dagger());
    let s: f64 = v.iter().map(|x| x.norm_sqr()).sum::<f64>().sqrt();
    v.iter().map(|x| x.scale(1.0 / s)).collect()
}

/// the squared length of the projection of a unit vector onto the span of a list of vectors (Gram-Schmidt)
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

/// the result of the search for the amplitude mode
#[derive(Clone, Debug)]
struct AmpMode {
    omega: f64,
    sigma: f64,
    /// the squared length of the projection of the amplitude of the vacuum direction onto the field vectors of the null
    /// space at omega
    overlap: f64,
    /// the dimension of the null space at omega (singular values below 1e-6): the multiplicity of the mode
    mult: usize,
    goldstone: usize,
    sing0: Vec<f64>,
    /// the lowest transition energy whose residue acts on the mode: the lowest omega_c among the buckets c with
    /// |A_c F v|^2 + |B_c F v|^2, summed over the right null vectors v of 1 - chi(omega) F at the mode, above 1e-12 of
    /// |A_c F|^2 + |B_c F|^2 (Frobenius norms): a fraction of the bucket's own size, so that the small weights of the
    /// smallest momenta of the grid do not hide a coupling; a coupling that a symmetry forbids gives a fraction at the
    /// level of the rounding
    threshold: f64,
    /// the lowest transition energy in the whole sector (in the first version the unpaired fields contribute from
    /// 2 m_LL on; they do not act on the mode)
    threshold_sector: f64,
    /// the largest residue fraction among the buckets below `threshold` (rounding level when they do not act)
    frac_below: f64,
}

/// the smallest singular value of A = 1 - chi(omega) F, resolved below the square root of the machine precision.
/// The eigenvalues of A^dag A give the singular values only to about (machine precision)^(1/2) |A|, which limits the
/// location of a zero to about 1e-8. Here the eigenvectors V of A^dag A whose eigenvalues lie below 1e-6 (or the one of
/// the smallest eigenvalue, if none does) span the near-null space to the machine precision times |A|^2 over the gap to
/// the other eigenvalues; the singular values of the small matrix A V then follow from (A V)^dag (A V), whose norm is
/// below 1e-6, so that the smallest one is resolved to about 1e-11.
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

/// the amplitude mode by the linear response at a frequency in a sector: the zeros of the smallest singular value of
/// 1 - chi(omega) F below omega_max (scan of the frequencies, refined by golden sections); among them the one whose null
/// space contains the amplitude of the vacuum direction best. The scan takes 600 equal steps from 0.01 to omega_max and
/// 25 more frequencies that approach omega_max geometrically (omega_max - (omega_max - 0.01) 10^(-j/4), j = 4 to 28), so
/// that a mode close to the threshold is not missed; it skips the frequencies within 1e-7 (relative) of a transition
/// energy. Every local minimum of the scan whose singular value lies below `deep` is refined.
fn amplitude_mode(
    mo: &ModelN,
    sol_n: &CMat,
    sol_p: &CMat,
    sec: &SectorN,
    ver: Ver,
    omega_max: f64,
    deep: f64,
    threads: usize,
) -> AmpMode {
    let resp = ResponseN::new(mo, sol_n, sol_p, sec, threads);
    let fk = kernel_n(mo, sec);
    let rf = resp.with_kernel(&fk);
    let (goldstone, sing0) = rf.count_small_singular(0.0, 1e-6);
    let amp = amp_vec(sec, ver);
    let poles: Vec<f64> = rf.buckets.iter().map(|b| b.omega).collect();
    let near_pole = |w: f64| poles.iter().any(|p| (w - p).abs() < 1e-7 * (1.0 + p));
    let sig = |w: f64| rf.smallest_singular(w).0;
    let nscan = 600;
    let w0 = 0.01;
    let mut ws: Vec<f64> = (0..=nscan).map(|i| w0 + (omega_max - w0) * i as f64 / nscan as f64).collect();
    for j in 4..=28 {
        ws.push(omega_max - (omega_max - w0) * 10f64.powf(-(j as f64) / 4.0));
    }
    ws.sort_by(|x, y| x.partial_cmp(y).unwrap());
    ws.dedup_by(|x, y| (*x - *y).abs() < 1e-12);
    let ws: Vec<f64> = ws.into_iter().filter(|w| !near_pole(*w)).collect();
    let ss: Vec<f64> = ws.iter().map(|w| sig(*w)).collect();
    let mut best = AmpMode {
        omega: f64::NAN,
        sigma: f64::INFINITY,
        overlap: 0.0,
        mult: 0,
        goldstone,
        sing0: sing0.iter().take(8).cloned().collect(),
        threshold: f64::NAN,
        threshold_sector: resp.threshold,
        frac_below: 0.0,
    };
    for i in 1..ws.len() - 1 {
        if !(ss[i] <= ss[i - 1] && ss[i] <= ss[i + 1] && ss[i] < deep) {
            continue;
        }
        // golden sections on the fine singular value (sigma_fine), which is V-shaped at a zero (the minimum of the
        // branches c_k |omega - omega_H|)
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
        // the null space at wm: the right singular vectors of 1 - chi F with singular values below 1e-6, mapped to the
        // field amplitudes by F
        let m = rf.chi(wm);
        let nv = m.rows;
        let am = CMat::identity(nv).sub(&m);
        let (ev, vecs) = eigh(&am.dagger().mul(&am));
        let mut fields = vec![];
        let mut nulls = vec![];
        let smin = sigma_fine(&rf, wm);
        for k in 0..nv {
            let sv = ev[k].max(0.0).sqrt();
            if sv < 1e-6 {
                nulls.push((0..nv).map(|r| vecs.get(r, k)).collect::<Vec<C64>>());
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
        let ov = overlap_span(&amp, &fields);
        if smin < 1e-6 && ov > best.overlap {
            // the residues of the buckets on the null space, |A_c F v|^2 + |B_c F v|^2 summed over the null vectors v,
            // as a fraction of the size |A_c F|^2 + |B_c F|^2 of the bucket
            let frac: Vec<f64> = rf
                .buckets
                .iter()
                .map(|bk| {
                    let mut size = 0.0;
                    for r in 0..bk.a.rows {
                        for cc in 0..bk.a.cols {
                            size += bk.a.get(r, cc).norm_sqr() + bk.b.get(r, cc).norm_sqr();
                        }
                    }
                    let res = nulls
                        .iter()
                        .map(|nv_| {
                            let mut s = 0.0;
                            for r in 0..bk.a.rows {
                                let (mut ya, mut yb) = (C64::ZERO, C64::ZERO);
                                for cc in 0..bk.a.cols {
                                    ya += bk.a.get(r, cc) * nv_[cc];
                                    yb += bk.b.get(r, cc) * nv_[cc];
                                }
                                s += ya.norm_sqr() + yb.norm_sqr();
                            }
                            s
                        })
                        .sum::<f64>();
                    if size > 0.0 {
                        res / size
                    } else {
                        0.0
                    }
                })
                .collect();
            best.threshold = rf
                .buckets
                .iter()
                .zip(frac.iter())
                .filter(|(_, r)| **r > 1e-12)
                .map(|(bk, _)| bk.omega)
                .fold(f64::INFINITY, f64::min);
            // the largest fraction among the buckets below that threshold (zero up to the rounding if they do not act)
            best.frac_below = rf
                .buckets
                .iter()
                .zip(frac.iter())
                .filter(|(bk, _)| bk.omega < best.threshold)
                .map(|(_, r)| *r)
                .fold(0.0f64, f64::max);
            best.omega = wm;
            best.sigma = smin;
            best.overlap = ov;
            best.mult = fields.len();
        }
    }
    best
}

/// the couplings of stage 2: name -> value
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

/// the results of one method in one version
struct MethodRes {
    method: Method,
    ver: Ver,
    d0: f64,
    e0: f64,
    phi0: f64,
    phi_pair_dev: f64,
    masses: [f64; 4],
    h1: (f64, f64),
    curve: Vec<(f64, f64, f64, f64, usize)>,
    v1: f64,
    v2: f64,
    v2_dev: f64,
    v1_rel: f64,
    chi: Vec<Vec<f64>>,
    chi_dev: f64,
    amp: Option<AmpMode>,
    seconds: f64,
}

fn run_method(method: Method, ver: Ver) -> MethodRes {
    let t0 = Instant::now();
    let mo = model(&method, &ver.pot(), NR);
    let z = CMat::zeros(N);
    // the ground state from the start of stage 3
    // (second version: 0.3 (K'_u^dag - K_d^dag), the custodial start 0.3 i tau^2; first version: 0.3 K_d^dag)
    let sol = solve_clean(&mo, &z, &p_dir(ver).scale_r(0.3), false);
    let dc = vac_coef(&sol.p_self, ver);
    let d0 = dc.abs();
    let desc = describe(&sol.n_self, &sol.p_self, &sol.drho);
    // rotate the phase of the pair self-energy away (a rotation by the particle number: P -> e^{2 i theta} P)
    let ph = dc.scale(1.0 / d0).conj();
    let sol_r = SolutionN { p_self: sol.p_self.scale(ph), kap: sol.kap.scale(ph.conj()), ..sol.clone() };
    // H-1: the fixed-d state at d_0
    let sf = fixed_d(&mo, ver, d0, &sol.n_self);
    let p_int = mo.interaction(&sf.drho, &sf.kap).2;
    let dint = vac_coef(&p_int, ver).abs();
    let h1 = ((sf.energy - sol.energy).abs() / sol.energy.abs(), (dint - d0).abs() / d0);
    let phif = phi_of(&sf.kap, ver);
    // the curve, and in the second version the equality of the two pair amplitudes along it
    let mut curve = vec![];
    let mut n_prev = CMat::zeros(N);
    let mut phi_pair_dev = 0.0f64;
    for k in 0..=32 {
        let s = 0.05 * k as f64;
        let st = fixed_d(&mo, ver, s * d0, &n_prev);
        n_prev = st.n_self.clone();
        let (pa, pb) = phis(&st.kap);
        if ver == Ver::Second {
            phi_pair_dev = phi_pair_dev.max((pa - pb).abs() / phif);
        }
        curve.push((s, s * d0, phi_of(&st.kap, ver), st.energy, st.iterations));
    }
    // the curvature at the minimum
    let (_, v2a) = curvature(&mo, ver, d0, &sol.n_self, 0.02);
    let (v1b, v2b) = curvature(&mo, ver, d0, &sol.n_self, 0.01);
    let v2 = (4.0 * v2b - v2a) / 3.0;
    let v2_dev = (v2a - v2b).abs() / v2b.abs();
    let v1_rel = v1b.abs() * phif / sol.energy.abs();
    // the susceptibilities
    let (chi, chi_dev) = susceptibilities(&mo, &sol_r, H_MU);
    // the amplitude mode where only the pairing terms act on it
    let amp = if pairing_only(&method) {
        let meff = (M_LL * M_LL + d0 * d0 / 4.0).sqrt();
        Some(amplitude_mode(&mo, &sol_r.n_self, &sol_r.p_self, &sector_r0(ver), ver, 2.0 * meff - 1e-6, 0.05, 1))
    } else {
        None
    };
    MethodRes {
        method,
        ver,
        d0,
        e0: sol.energy,
        phi0: phif,
        phi_pair_dev,
        masses: desc.mass,
        h1,
        curve,
        v1: v1b,
        v2,
        v2_dev,
        v1_rel,
        chi,
        chi_dev,
        amp,
        seconds: t0.elapsed().as_secs_f64(),
    }
}

/// the pairing-only ground state (HS (0, 0)) of the coupling g and the mass m from the closed-form gap equation
/// (g / (4 pi^2)) I1((m^2 + D^2/4)^(1/2)) = 1 (bisection)
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

/// one point of the scan (HS (0, 0)): the ground state from the closed form, the self-consistency of the engine, the
/// susceptibilities chi^{11}, chi^{33}, v_SH, rho and the amplitude mode (sector of the neutral pairs of the vacuum)
fn scan_point(ver: Ver, m: f64, r: f64, nr: usize) -> Vec<f64> {
    let method = Method::Hs { s_s: 0.0, s_v: 0.0 };
    let gc = 4.0 * PI * PI / i1(m);
    let pot = match ver {
        Ver::Second => PotSh { gs: -8.0, gv: 0.8, gsh: r * gc, gsh2: r * gc },
        Ver::First => PotSh { gs: -8.0, gv: 0.8, gsh: r * gc, gsh2: 0.0 },
    };
    let mo = model_m(&method, &pot, m, nr);
    let d0 = gap_closed(pot.gsh, m);
    let z = CMat::zeros(N);
    let sf = fixed_d(&mo, ver, d0, &z);
    let p_int = mo.interaction(&sf.drho, &sf.kap).2;
    let dint = vac_coef(&p_int, ver).abs();
    let sol = SolutionN { n_self: sf.n_self.clone(), p_self: p_dir(ver).scale_r(d0), ..sf.clone() };
    // chi^{11} and chi^{33} by the charge response (the other elements are not needed here)
    let g = generators();
    let col = |b: usize| -> Vec<f64> {
        let q_at = |mu: f64| -> Vec<f64> {
            let m2 = with_ext(&mo, &g[b].scale_r(-mu));
            let s = solve_clean(&m2, &sol.n_self, &sol.p_self, false);
            charges(&g, &s.drho)
        };
        let (qp1, qm1, qp2, qm2) = (q_at(H_MU), q_at(-H_MU), q_at(2.0 * H_MU), q_at(-2.0 * H_MU));
        (0..g.len()).map(|a| (4.0 * (qp1[a] - qm1[a]) / (2.0 * H_MU) - (qp2[a] - qm2[a]) / (4.0 * H_MU)) / 3.0).collect()
    };
    let chi11 = col(0)[0];
    let chi33 = col(2)[2];
    let meff = (m * m + d0 * d0 / 4.0).sqrt();
    // the sector of the neutral pairs of the vacuum has no transition below 2 M_eff, so every local minimum is refined
    let am = amplitude_mode(&mo, &sol.n_self, &sol.p_self, &sector_vac_pairs(ver), ver, 2.0 * meff - 1e-6, f64::INFINITY, 1);
    let v = 2.0 * chi11.sqrt();
    vec![m, r, pot.gsh, d0, (dint - d0).abs() / d0, meff, chi11, chi33, v, chi11 / chi33, am.omega, am.omega / v, 2.0 * meff, am.sigma, am.overlap, am.mult as f64]
}

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sh_higgs: checks of the effective potential, the masses of the gauge bosons and the amplitude mode of the Higgs \
         superfluid (stage 4 of the revision, second version with the first as the comparison, Revision_SM_Higgs, PROJECT \
         spacetimes)",
    );
    let cpl = read_couplings(&data.join("sh_couplings.csv"));
    let (g, gp, v_gev, mh_over_v) = (cpl["g"], cpl["g_prime"], cpl["v"], cpl["m_H_over_v"]);
    let cos2 = g * g / (g * g + gp * gp);

    // ------------------------------------------------------------------ the five methods in both versions, two at a time
    let mut jobs = vec![];
    for ver in [Ver::Second, Ver::First] {
        for m in methods() {
            jobs.push((m, ver));
        }
    }
    let mut results: Vec<MethodRes> = vec![];
    for chunk in jobs.chunks(2) {
        let part: Vec<MethodRes> = std::thread::scope(|sc| {
            let hs: Vec<_> = chunk
                .iter()
                .map(|&(m, ver)| sc.spawn(move || run_method(m, ver)))
                .collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for r in part {
            println!(
                "{} {}: d0 {:.6} V'' {:.6} chi11 {:.6e} chi33 {:.6e} ({:.0} s)",
                r.ver.name(),
                mname(&r.method),
                r.d0,
                r.v2,
                r.chi[0][0],
                r.chi[2][2],
                r.seconds
            );
            results.push(r);
        }
    }
    let longest = results.iter().map(|r| r.seconds).fold(0.0f64, f64::max);
    let find = |ver: Ver, m: Method| results.iter().find(|r| r.ver == ver && mname(&r.method) == mname(&m)).unwrap();

    // ------------------------------------------------------------------ H-1 to H-3: the effective potential
    let h1 = results.iter().map(|r| r.h1.0.max(r.h1.1)).fold(0.0f64, f64::max);
    rep.add(
        "H-1",
        "the state with the fixed pair self-energy d_0 P_dir and a self-consistent normal self-energy is the ground state: \
         its energy (relative), and the pair self-energy that its pair amplitude produces equals d_0 P_dir (relative; \
         self-consistency), five methods, both versions",
        h1,
        1e-9,
    );
    let v1max = results.iter().map(|r| r.v1_rel).fold(0.0f64, f64::max);
    let v2pos = results.iter().all(|r| r.v2 > 0.0);
    let v2dev = results.iter().map(|r| r.v2_dev).fold(0.0f64, f64::max);
    rep.add(
        "H-2a",
        "phi_0 is a stationary point of V: V'(phi_0) phi_0 / |E_0| from central differences with the step 0.01 in d (zero \
         up to the error of the differences, of the order of the step squared), five methods, both versions",
        v1max,
        1e-3,
    );
    rep.add_bool(
        "H-2b",
        "V''(phi_0) > 0 in every method and both versions: phi_0 is a minimum of the effective potential",
        v2pos,
        &results.iter().map(|r| format!("{} {}: {:.4}", r.ver.name(), mname(&r.method), r.v2)).collect::<Vec<_>>().join("; "),
    );
    rep.add(
        "H-2c",
        "V''(phi_0) from the steps 0.02 and 0.01 in d agree to the order of the steps (relative difference; the stated \
         value is their Richardson combination)",
        v2dev,
        2e-3,
    );
    let ppd = results.iter().filter(|r| r.ver == Ver::Second).map(|r| r.phi_pair_dev).fold(0.0f64, f64::max);
    rep.add(
        "H-2d",
        "second version: along the whole curve the two pair amplitudes are equal, |<Pi^d_SH>| = |<Pi'^u_SH>| (difference \
         relative to phi_0): the curve moves both pair fields together, along the custodial direction",
        ppd,
        1e-9,
    );
    let mut hat_ok = true;
    let mut hat_detail = vec![];
    for r in &results {
        let cv = &r.curve;
        let ok0 = cv[0].3.abs() < 1e-14 && cv[0].2.abs() < 1e-14;
        let kmin = (0..cv.len()).fold(0, |b, i| if cv[i].3 < cv[b].3 { i } else { b });
        let dec = (1..=kmin).all(|i| cv[i].3 < cv[i - 1].3);
        let inc = (kmin + 1..cv.len()).all(|i| cv[i].3 > cv[i - 1].3);
        let at_one = (cv[kmin].0 - 1.0).abs() < 1e-12;
        let phi_mono = (1..cv.len()).all(|i| cv[i].2 > cv[i - 1].2);
        hat_ok &= ok0 && dec && inc && at_one && phi_mono;
        hat_detail.push(format!("{} {}: V(0) = {:.1e}, minimum at d = {:.2} d_0", r.ver.name(), mname(&r.method), cv[0].3, cv[kmin].0));
    }
    rep.add_bool(
        "H-3",
        "the Mexican hat along the vacuum direction: V = 0 at phi = 0 (the normal vacuum), phi grows with d, V decreases \
         from phi = 0 to the minimum, which is at d = d_0 among the 33 points, and increases beyond it, five methods, both \
         versions",
        hat_ok,
        &hat_detail.join("; "),
    );

    // ------------------------------------------------------------------ H-4: the susceptibilities
    let mut asym = 0.0f64;
    let mut chidev = 0.0f64;
    let mut rank_ok = true;
    let mut rank_detail = vec![];
    let mut unbroken = 0.0f64;
    let mut cust = 0.0f64;
    let mut q_row = 0.0f64;
    for r in &results {
        let ch = &r.chi;
        let s = ch[0][0].abs();
        for a in 0..8 {
            for b in 0..8 {
                asym = asym.max((ch[a][b] - ch[b][a]).abs() / s);
            }
        }
        chidev = chidev.max(r.chi_dev);
        // the broken generators: the nonzero eigenvalues of chi on the generators that are symmetries of the version
        let idx: Vec<usize> = match r.ver {
            Ver::Second => (0..8).collect(),
            Ver::First => vec![0, 1, 2, 6, 7],
        };
        let ev = chi_eigs(ch, &idx);
        let top = ev.iter().fold(0.0f64, |m, x| m.max(x.abs()));
        let nbroken = ev.iter().filter(|x| x.abs() > 1e-6 * top).count();
        let want = if r.ver == Ver::Second { 4 } else { 3 };
        rank_ok &= nbroken == want && ev.iter().all(|x| *x > -1e-9 * top);
        rank_detail.push(format!("{} {}: {}", r.ver.name(), mname(&r.method), nbroken));
        // the photon: the charge Q = T^3 + Y does not respond
        let qc = Q_COEF.to_vec();
        for a in 0..8 {
            q_row = q_row.max(chi_form(ch, &qc, &unit8(a)).abs() / s);
        }
        if r.ver == Ver::Second {
            // the unbroken generators of stage 3: T^a + T^a_R and R
            for a in 0..3 {
                let mut u = unit8(a);
                u[a + 3] = 1.0;
                for b in 0..8 {
                    unbroken = unbroken.max(chi_form(ch, &u, &unit8(b)).abs() / s);
                }
            }
            for b in 0..8 {
                unbroken = unbroken.max(chi_form(ch, &unit8(6), &unit8(b)).abs() / s);
            }
            // the custodial structure: chi(T^a, T^a) = chi(T^a_R, T^a_R) = -chi(T^a, T^a_R), all a equal
            for a in 0..3 {
                cust = cust
                    .max((ch[a][a] - ch[0][0]).abs() / s)
                    .max((ch[a + 3][a + 3] - ch[0][0]).abs() / s)
                    .max((ch[a][a + 3] + ch[0][0]).abs() / s);
            }
        }
    }
    rep.add(
        "H-4a",
        "the susceptibilities chi^{ab} = d<Q_a>/d mu_b are symmetric (largest |chi^{ab} - chi^{ba}| relative to chi^{11}), as \
         the theorem of Hellmann and Feynman requires (chi^{ab} = -d^2 E / d mu_a d mu_b), eight generators, five methods, \
         both versions",
        asym,
        1e-6,
    );
    rep.add(
        "H-4b",
        "the steps 0.005 and 0.01 of the chemical potentials give the same susceptibilities to the order of the steps \
         squared (largest difference relative to the largest element; the stated values are their Richardson combination)",
        chidev,
        1e-3,
    );
    let mut kscj = 0.0f64;
    for ver in [Ver::Second, Ver::First] {
        let (ks, cj) = (find(ver, Method::Ks), find(ver, Method::Cjt));
        for a in 0..8 {
            for b in 0..8 {
                kscj = kscj.max((ks.chi[a][b] - cj.chi[a][b]).abs() / cj.chi[0][0]);
            }
        }
    }
    rep.add(
        "H-4c",
        "KS and CJT give the same susceptibilities (relative to chi^{11}; the perturbed states keep their local normal \
         matrix in the span of the Kohn-Sham densities), both versions",
        kscj,
        1e-6,
    );
    rep.add_bool(
        "H-4d",
        "the number of broken generators, the nonzero eigenvalues of chi (above 1e-6 of the largest; none negative): four \
         among the eight generators in the second version (the three of SU(2)_L x U(1)_Y that the photon does not keep, \
         and F_tot), three among T^1, T^2, T^3, R, F_tot in the first version",
        rank_ok,
        &rank_detail.join("; "),
    );
    rep.add(
        "H-4e",
        "second version: the generators that stage 3 found unbroken, T^a + T^a_R (a = 1, 2, 3) and R, have zero \
         susceptibility with every generator (relative to chi^{11}), five methods",
        unbroken,
        1e-6,
    );
    rep.add(
        "H-4f",
        "second version: the custodial structure chi(T^a, T^a) = chi(T^a_R, T^a_R) = -chi(T^a, T^a_R), equal for a = 1, 2, \
         3 (relative to chi^{11}), five methods",
        cust,
        1e-6,
    );
    rep.add(
        "H-4g",
        "the electric charge Q = T^3 + T^3_R + R/4 does not respond: chi(Q, X_b) = 0 for the eight generators (relative to \
         chi^{11}); the photon is massless, five methods, both versions",
        q_row,
        1e-6,
    );

    // ------------------------------------------------------------------ H-5, H-6: the masses of the gauge bosons
    let mut h5a = 0.0f64;
    let mut h5b = 0.0f64;
    let mut h6 = 0.0f64;
    let mut rho2 = 0.0f64;
    let mut mass_rows = vec![];
    let yc = Y_COEF.to_vec();
    for r in &results {
        let ch = &r.chi;
        // the 4 x 4 susceptibilities of (T^1, T^2, T^3, Y)
        let basis: Vec<Vec<f64>> = vec![unit8(0), unit8(1), unit8(2), yc.clone()];
        let mut c4 = [[0.0; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                c4[a][b] = 0.5 * (chi_form(ch, &basis[a], &basis[b]) + chi_form(ch, &basis[b], &basis[a]));
            }
        }
        let cp = [g, g, g, gp];
        let mut m2 = CMat::zeros(4);
        for a in 0..4 {
            for b in 0..4 {
                m2.set(a, b, c(cp[a] * cp[b] * c4[a][b], 0.0));
            }
        }
        let (ev, vecs) = eigh(&m2);
        let scale = ev[3].abs();
        let nrm = (g * g + gp * gp).sqrt();
        let phv = [0.0, 0.0, gp / nrm, g / nrm];
        let mut o = C64::ZERO;
        for i in 0..4 {
            o += vecs.get(i, 0).conj().scale(phv[i]);
        }
        h5a = h5a.max(ev[0].abs() / scale).max(1.0 - o.norm_sqr());
        let mw2 = g * g * c4[0][0];
        h5b = h5b.max((ev[1] - mw2).abs() / mw2).max((ev[2] - mw2).abs() / mw2);
        let mz2 = ev[3];
        let rho_ev = 0.5 * (ev[1] + ev[2]) / (mz2 * cos2);
        let rho = c4[0][0] / c4[2][2];
        h6 = h6.max((rho_ev - rho).abs() / rho);
        if r.ver == Ver::Second {
            rho2 = rho2.max((rho - 1.0).abs());
        }
        let vsh = 2.0 * c4[0][0].sqrt();
        let lam_gev = v_gev / vsh;
        let meft = (r.v2 * r.phi0 * r.phi0 / (4.0 * c4[2][2])).sqrt();
        let (mrpa, mrpa_gev) = match &r.amp {
            Some(a) => (a.omega, a.omega * lam_gev),
            None => (f64::NAN, f64::NAN),
        };
        mass_rows.push((
            r.ver,
            r.method,
            vec![
                vsh,
                lam_gev,
                mw2.sqrt() * lam_gev,
                mz2.sqrt() * lam_gev,
                rho,
                meft,
                meft * lam_gev,
                meft / vsh,
                mrpa,
                mrpa_gev,
                mrpa / vsh,
                r.masses[0] * lam_gev,
                r.masses[1] * lam_gev,
                r.masses[2] * lam_gev,
                r.masses[3] * lam_gev,
                c4[0][0],
                c4[2][2],
                c4[3][3],
                c4[2][3],
                ch[7][7],
            ],
        ));
    }
    rep.add(
        "H-5a",
        "the mass matrix g_a g_b chi^{ab} of (W^1, W^2, W^3, B) has one zero eigenvalue (relative to the largest) whose \
         eigenvector is the photon (0, 0, g', g)/(g^2 + g'^2)^(1/2) (one minus the squared overlap), five methods, both \
         versions",
        h5a,
        1e-6,
    );
    rep.add("H-5b", "its other eigenvalues: M_W^2 = g^2 chi^{11} twice (relative), five methods, both versions", h5b, 1e-6);
    rep.add(
        "H-6a",
        "rho from the eigenvalues, M_W^2 / (M_Z^2 cos^2 theta_W), equals chi^{11} / chi^{33} (relative), five methods, both \
         versions",
        h6,
        1e-9,
    );
    rep.add(
        "H-6b",
        "second version: rho = chi^{11} / chi^{33} = 1 (the custodial symmetry of the vacuum), five methods",
        rho2,
        1e-6,
    );

    // ------------------------------------------------------------------ H-7: the amplitude mode by the linear response
    let mut gold_ok = true;
    let mut gold_detail = vec![];
    let mut amp_ok = true;
    let mut amp_detail = vec![];
    let mut h7c = 0.0f64;
    let mut h7d = 0.0f64;
    let mut thr_dev = 0.0f64;
    for ver in [Ver::Second, Ver::First] {
        let r01 = find(ver, Method::Hs { s_s: 0.0, s_v: 1.0 });
        let r00 = find(ver, Method::Hs { s_s: 0.0, s_v: 0.0 });
        let a01 = r01.amp.clone().unwrap();
        let a00 = r00.amp.clone().unwrap();
        for (r, a) in [(r01, &a01), (r00, &a00)] {
            h7d = h7d.max((a.omega - r.d0).abs() / r.d0);
            // the closed form of the threshold, 2 M_eff = (4 m_LL^2 + d_0^2)^(1/2) (the bucket of the smallest momentum
            // of the grid lies slightly above it)
            thr_dev = thr_dev.max((a.threshold - (4.0 * M_LL * M_LL + r.d0 * r.d0).sqrt()) / a.threshold);
        }
        let want = if ver == Ver::Second { 4 } else { 3 };
        for (nm, a) in [("HS (0, 1)", &a01), ("HS (0, 0)", &a00)] {
            gold_ok &= a.goldstone == want && a.sing0[want] > 1e-3;
            gold_detail.push(format!(
                "{} {}: {} below 1e-6, the next {:.2e}",
                ver.name(),
                nm,
                a.goldstone,
                a.sing0[want]
            ));
            amp_ok &= a.omega.is_finite() && a.sigma < 1e-6 && a.overlap > 0.999 && a.omega < a.threshold && a.frac_below < 1e-20;
            amp_detail.push(format!(
                "{} {}: omega {:.10} (the lowest transition that acts on it {:.6}, the lowest of the sector {:.6}, the \
                 largest residue fraction below the former {:.1e}), singular value {:.1e}, multiplicity {}, overlap {:.9}",
                ver.name(),
                nm,
                a.omega,
                a.threshold,
                a.threshold_sector,
                a.frac_below,
                a.sigma,
                a.mult,
                a.overlap
            ));
        }
        h7c = h7c.max((a01.omega - a00.omega).abs() / a00.omega);
    }
    rep.add_bool(
        "H-7a",
        "at omega = 0 exactly four singular values of 1 - chi(0) F vanish in the sector of relative number 0 in the second \
         version and three in the first (the Goldstone modes of the broken generators), and the next is not small, HS (0, 1) \
         and HS (0, 0)",
        gold_ok,
        &gold_detail.join("; "),
    );
    rep.add_bool(
        "H-7b",
        "below the lowest transition energy that acts on it (the lowest omega_c whose residue A_c F, B_c F on the null space \
         exceeds 1e-12 of the size of the bucket) there is a frequency at which 1 - chi(omega) F is singular (smallest \
         singular value below 1e-6) and whose null space contains the amplitude of the vacuum direction (squared length of \
         its projection above 0.999), HS (0, 1) and HS (0, 0), both versions; in the first version the sector also holds the \
         transitions of the unpaired fields from 2 m_LL on, whose residues on the null space are at the level of the \
         rounding (fraction below 1e-20): they do not act on the mode",
        amp_ok,
        &amp_detail.join("; "),
    );
    rep.add(
        "H-7c",
        "HS (0, 1) and HS (0, 0) give the same amplitude mode (relative difference of the frequencies), both versions: the \
         vector term of HS (0, 1), which acts on the number density and the currents, does not couple to it",
        h7c,
        1e-10,
    );
    rep.add(
        "H-7d",
        "the amplitude mode lies at the pair self-energy, omega_H = d_0 (relative difference), HS (0, 1) and HS (0, 0), both \
         versions: the analogue of the amplitude mode of the theory of Bardeen, Cooper and Schrieffer at twice the gap \
         parameter, here d_0 / 2",
        h7d,
        1e-10,
    );
    rep.add(
        "H-7e",
        "the lowest transition energy that acts on the mode is the threshold of two paired quasi-particles, \
         2 M_eff = (4 m_LL^2 + d_0^2)^(1/2) (relative excess, of the order of the smallest momentum of the grid squared; \
         never below it), HS (0, 1) and HS (0, 0), both versions",
        thr_dev.abs(),
        1e-4,
    );

    // ------------------------------------------------------------------ the scan (HS (0, 0)), both versions
    let ms_scan = [0.1, 0.2];
    let rs_scan = [1.02, 1.05, 1.1, 1.183842814, 1.3, 1.5, 2.0, 3.0];
    let mut sjobs = vec![];
    for ver in [Ver::Second, Ver::First] {
        for &m in &ms_scan {
            for &r in &rs_scan {
                sjobs.push((ver, m, r));
            }
        }
    }
    let scan: Vec<(Ver, Vec<f64>)> = sjobs
        .chunks(2)
        .flat_map(|ch| {
            std::thread::scope(|sc| {
                let hs: Vec<_> = ch.iter().map(|&(ver, m, r)| sc.spawn(move || (ver, scan_point(ver, m, r, NR)))).collect();
                hs.into_iter().map(|h| h.join().unwrap()).collect::<Vec<_>>()
            })
        })
        .collect();
    let h8 = scan.iter().map(|(_, row)| row[4]).fold(0.0f64, f64::max);
    rep.add(
        "H-8a",
        "the scan (HS (0, 0), 16 points in each version): the pair self-energy d_0 of the closed-form gap equation \
         (G_SH / 4 pi^2) I1((m^2 + d_0^2/4)^(1/2)) = 1 is self-consistent in the engine (relative)",
        h8,
        1e-9,
    );
    let scan_amp_ok = scan.iter().all(|(_, row)| row[10].is_finite() && row[13] < 1e-6 && row[14] > 0.999);
    rep.add_bool(
        "H-8b",
        "in every point of the scan the amplitude mode is found below the threshold of two quasi-particles (smallest \
         singular value below 1e-6, squared length of the projection of the amplitude of the vacuum direction onto the null \
         space above 0.999)",
        scan_amp_ok,
        &format!("{} points", scan.len()),
    );
    let mut h8c = 0.0f64;
    for ver in [Ver::Second, Ver::First] {
        let std_row = &scan.iter().find(|(v, row)| *v == ver && (row[0] - 0.2).abs() < 1e-12 && (row[1] - 1.183842814).abs() < 1e-9).unwrap().1;
        let r00 = find(ver, Method::Hs { s_s: 0.0, s_v: 0.0 });
        let a00 = r00.amp.as_ref().unwrap();
        h8c = h8c
            .max((std_row[3] - r00.d0).abs() / r00.d0)
            .max((std_row[8] - 2.0 * r00.chi[0][0].sqrt()).abs() / std_row[8])
            .max((std_row[10] - a00.omega).abs() / a00.omega);
    }
    rep.add(
        "H-8c",
        "the scan point m = 0.2, G_SH = 100.8 reproduces the main computation of HS (0, 0) in both versions: d_0, v_SH and the \
         amplitude mode (the latter from the sector of the neutral pairs of the vacuum alone; relative)",
        h8c,
        1e-6,
    );
    let h8d = scan.iter().map(|(_, row)| (row[10] - row[3]).abs() / row[3]).fold(0.0f64, f64::max);
    rep.add(
        "H-8d",
        "in every point of the scan the amplitude mode lies at the pair self-energy, omega_H = d_0 (relative difference)",
        h8d,
        1e-9,
    );

    // ------------------------------------------------------------------ H-9: the grid
    let row48 = scan_point(Ver::Second, 0.2, 1.183842814, 48);
    let std2 = &scan.iter().find(|(v, row)| *v == Ver::Second && (row[0] - 0.2).abs() < 1e-12 && (row[1] - 1.183842814).abs() < 1e-9).unwrap().1;
    let mo48 = model(&Method::Cjt, &V1_SH, 48);
    let sol48 = solve_clean(&mo48, &CMat::zeros(N), &p_dir(Ver::Second).scale_r(0.3), false);
    let dc48 = vac_coef(&sol48.p_self, Ver::Second);
    let ph48 = dc48.scale(1.0 / dc48.abs()).conj();
    let sol48r = SolutionN { p_self: sol48.p_self.scale(ph48), kap: sol48.kap.scale(ph48.conj()), ..sol48.clone() };
    let chi11_48 = {
        let gq = generators();
        let q_at = |mu: f64| -> f64 {
            let m2 = with_ext(&mo48, &gq[0].scale_r(-mu));
            let s = solve_clean(&m2, &sol48r.n_self, &sol48r.p_self, false);
            gq[0].mul(&s.drho).trace().re
        };
        let c1 = (q_at(H_MU) - q_at(-H_MU)) / (2.0 * H_MU);
        let c2 = (q_at(2.0 * H_MU) - q_at(-2.0 * H_MU)) / (4.0 * H_MU);
        (4.0 * c1 - c2) / 3.0
    };
    let cj2 = find(Ver::Second, Method::Cjt);
    let h9 = ((row48[8] - std2[8]).abs() / std2[8])
        .max((row48[10] - std2[10]).abs() / std2[10])
        .max(((2.0 * chi11_48.sqrt()) - 2.0 * cj2.chi[0][0].sqrt()).abs() / (2.0 * cj2.chi[0][0].sqrt()));
    rep.add(
        "H-9",
        "24 and 48 radial nodes give the same v_SH (HS (0, 0) and CJT) and the same amplitude mode (HS (0, 0)) (relative), \
         second version",
        h9,
        1e-6,
    );

    // ------------------------------------------------------------------ the data files
    let mut rows = vec![];
    for r in &results {
        for (s, d, phi, e, it) in &r.curve {
            rows.push(vec![r.ver.name().into(), mname(&r.method), f(*s), f(*d), f(*phi), f(*e), it.to_string()]);
        }
    }
    write_csv(&data.join("sh_hg_potential.csv"), &["version", "method", "d_over_d0", "d", "phi", "energy", "iterations"], &rows);
    let mut rows = vec![];
    for r in &results {
        rows.push(vec![
            r.ver.name().into(),
            mname(&r.method),
            f(r.d0),
            f(r.phi0),
            f(r.e0),
            f(r.v1),
            f(r.v2),
            f(r.v2_dev),
            f(r.masses[0]),
            f(r.masses[1]),
            f(r.masses[2]),
            f(r.masses[3]),
        ]);
    }
    write_csv(
        &data.join("sh_hg_minimum.csv"),
        &["version", "method", "d0", "phi0", "energy", "V1_at_phi0", "V2_at_phi0", "V2_step_difference", "M_up", "M_down", "M_chi", "M_chi2"],
        &rows,
    );
    // the susceptibilities: the upper triangle of the 8 x 8 matrix (symmetrized)
    let mut hdr: Vec<String> = vec!["version".into(), "method".into()];
    for a in 0..8 {
        for b in a..8 {
            hdr.push(format!("chi_{}_{}", GEN_NAMES[a], GEN_NAMES[b]));
        }
    }
    let mut rows = vec![];
    for r in &results {
        let mut row = vec![r.ver.name().to_string(), mname(&r.method)];
        for a in 0..8 {
            for b in a..8 {
                row.push(f(0.5 * (r.chi[a][b] + r.chi[b][a])));
            }
        }
        rows.push(row);
    }
    let hdr_ref: Vec<&str> = hdr.iter().map(|s| s.as_str()).collect();
    write_csv(&data.join("sh_hg_susceptibility.csv"), &hdr_ref, &rows);
    let mut rows = vec![];
    for (ver, m, vals) in &mass_rows {
        let mut row = vec![ver.name().to_string(), mname(m)];
        row.extend(vals.iter().map(|x| f(*x)));
        rows.push(row);
    }
    write_csv(
        &data.join("sh_hg_masses.csv"),
        &[
            "version",
            "method",
            "v_SH",
            "Lambda_GeV",
            "M_W_GeV",
            "M_Z_GeV",
            "rho",
            "M_H_EFT",
            "M_H_EFT_GeV",
            "M_H_EFT_over_v_SH",
            "M_H_RPA",
            "M_H_RPA_GeV",
            "M_H_RPA_over_v_SH",
            "M_up_GeV",
            "M_down_GeV",
            "M_chi_GeV",
            "M_chi2_GeV",
            "chi_T1T1",
            "chi_T3T3",
            "chi_YY",
            "chi_T3Y",
            "chi_FF",
        ],
        &rows,
    );
    let mut rows = vec![];
    for (ver, row) in &scan {
        let mut rr = vec![ver.name().to_string()];
        rr.extend(row.iter().map(|x| f(*x)));
        rows.push(rr);
    }
    write_csv(
        &data.join("sh_hg_scan.csv"),
        &[
            "version",
            "m",
            "G_over_Gcrit",
            "G_SH",
            "d0",
            "selfconsistency",
            "M_eff",
            "chi_T1T1",
            "chi_T3T3",
            "v_SH",
            "rho",
            "M_H_RPA",
            "M_H_RPA_over_v_SH",
            "threshold_2M_eff",
            "singular_value",
            "amplitude_overlap",
            "multiplicity",
        ],
        &rows,
    );
    let mut amps = vec![];
    for ver in [Ver::Second, Ver::First] {
        for mm in [Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }] {
            let a = find(ver, mm).amp.clone().unwrap();
            let tag = format!("{}_{}", ver.name(), mname(&mm).replace(' ', "").replace('(', "").replace(')', "").replace(',', "_"));
            amps.push(vec![format!("amp_omega_{}", tag), f(a.omega)]);
            amps.push(vec![format!("amp_threshold_{}", tag), f(a.threshold)]);
            amps.push(vec![format!("amp_threshold_sector_{}", tag), f(a.threshold_sector)]);
            amps.push(vec![format!("amp_residue_fraction_below_{}", tag), f(a.frac_below)]);
            amps.push(vec![format!("amp_sigma_{}", tag), f(a.sigma)]);
            amps.push(vec![format!("amp_omega_minus_d0_rel_{}", tag), f((a.omega - find(ver, mm).d0) / find(ver, mm).d0)]);
            amps.push(vec![format!("amp_multiplicity_{}", tag), format!("{}", a.mult)]);
            amps.push(vec![format!("amp_overlap_{}", tag), f(a.overlap)]);
            amps.push(vec![format!("goldstone_count_{}", tag), format!("{}", a.goldstone)]);
            amps.push(vec![format!("goldstone_next_singular_{}", tag), f(a.sing0[if ver == Ver::Second { 4 } else { 3 }])]);
        }
    }
    let minr = |ver: Ver| scan.iter().filter(|(v, _)| *v == ver).map(|(_, row)| row[11]).fold(f64::INFINITY, f64::min);
    let maxr = |ver: Ver| scan.iter().filter(|(v, _)| *v == ver).map(|(_, row)| row[11]).fold(0.0f64, f64::max);
    let mut vals: Vec<Vec<String>> = vec![
        vec!["g".into(), f(g)],
        vec!["g_prime".into(), f(gp)],
        vec!["v_GeV".into(), f(v_gev)],
        vec!["m_H_over_v_SM".into(), f(mh_over_v)],
        vec!["scan_min_M_H_over_v_SH_second".into(), f(minr(Ver::Second))],
        vec!["scan_max_M_H_over_v_SH_second".into(), f(maxr(Ver::Second))],
        vec!["scan_min_M_H_over_v_SH_first".into(), f(minr(Ver::First))],
        vec!["scan_max_M_H_over_v_SH_first".into(), f(maxr(Ver::First))],
        vec!["grid48_HS00_v_SH".into(), f(row48[8])],
        vec!["grid48_HS00_omega_H".into(), f(row48[10])],
        vec!["grid48_CJT_chi_T1T1".into(), f(chi11_48)],
    ];
    vals.extend(amps);
    write_csv(&data.join("sh_hg_values.csv"), &["name", "value"], &vals);
    rep.note(&format!(
        "run time {:.0} s on 2 threads; the longest of the ten computations (five methods, two versions) {:.0} s",
        t0.elapsed().as_secs_f64(),
        longest
    ));
    rep.write(&data.join("hck_sh_higgs_checks.txt"));
    let (np, nt) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, nt, t0.elapsed().as_secs_f64());
    std::process::exit(if np == nt { 0 } else { 1 });
}
