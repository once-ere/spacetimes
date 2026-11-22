//! sh_superfluid -- stage 3 of the revision of the hypotheses, second version (folder Revision_SM_Higgs of PROJECT
//! spacetimes): the mean-field superfluid of the three fields Psi_LL (the isodoublet, eight components), chi_SH and
//! chi'_SH (the singlets, four components each) by the methods HS, KS and CJT, with the first version of the hypotheses
//! (chi'_SH decoupled) as the comparison.
//!
//! The model.  The Hamiltonian density is the free Dirac Hamiltonian of the three fields (masses m_LL = m^chi_SH =
//! m^chi'_SH = 0.2 in units of the cutoff Lambda_LL = 1, decisions D22 and D31), the scalar and the vector contact terms
//! of the author's ansatz on Psi_LL (couplings G^S_SF, G^V_SF), and the pairing terms (4.2) of the doublet-singlet
//! channels, -G_SH sum_s [(Pi^s_SH)^dag Pi^s_SH + (Pi'^s_SH)^dag Pi'^s_SH] with Pi^s_SH = Psi_LL^T V_s chi_SH,
//! Pi'^s_SH = Psi_LL^T V_s chi'_SH, V_s = (1/2) C gamma^ch (x) e_s.  The methods keep the Wick contractions of the contact
//! terms with the weights of the earlier work (HS with the parameters (1, 1), (0, 1), (0, 0); KS with the exchange in the
//! local-density approximation of the number and scalar densities, those of Psi_LL as matrices in the isospin space and
//! those of the singlets as matrices in the space of the two singlets, decisions D28 and D33; CJT with all contractions).  The
//! engine is sfcore::uniform_n.
//!
//! The potentials.  V^(0)_SH: the author's ansatz (G^S_SF = -8, G^V_SF = 0.8) without a pairing term.  V^(1)_SH: the same
//! with the pairing terms, G_SH = 100.8 (decision D27), in the second version (both singlets) and in the first version
//! (the pairing term of chi'_SH switched off: chi'_SH is a free field, decoupled).
//!
//! The checks (report data/hck_sh_superfluid_checks.txt):
//!   SF-1   the engine reproduces the engine of the earlier work (module uniform) on the earlier V^(2)_SF with the two
//!          singlet fields decoupled, in all five methods;
//!   SF-2   the static response of the normal vacuum in the pairs of Psi_LL with the singlets: the largest eigenvalue is
//!          G_SH / G^crit_SH with G^crit_SH = 4 pi^2 / I1(m), eightfold in the second version (two doublets, real and
//!          imaginary parts) and fourfold in the first; the pseudoscalar pairs have no interaction;
//!   SF-3   the scalar channel of the normal vacuum: the 3 x 3 response of the Dirac masses of the three fields agrees
//!          with the effective couplings and the scalar susceptibilities; it is stable;
//!   SF-4   the other channels of the normal vacuum: the triplet scalar pair coupling of Psi_LL, and every eigenvalue
//!          below 1 outside the doublet-singlet pairs;
//!   SF-5   V^(0)_SH: every start of every method ends in the normal vacuum;
//!   SF-6   V^(1)_SH: the ground state of every method is, in the second version, the custodial Higgs superfluid (both
//!          pair fields, equal lengths, no charged pair or mixing), and in the first version the superfluid of one doublet;
//!   SF-7   in the second version the starts along i tau^2, along a rotated unitary bidoublet and along a generic
//!          bidoublet reach states of equal energy and equal lengths (the symmetry of H4), and the condensate of one
//!          doublet, found by an iteration restricted to the self-energies that keep the number of chi'_SH, is a
//!          stationary state of higher energy;
//!   SF-8   the generators of the global group of H4 that leave each custodial ground state unchanged: dimension 4,
//!          among them one with the spectrum of Q^SM; those of SU(2)_L x U(1)_Y: dimension 1;
//!   SF-9   the static response of each ground state: in the second version four eigenvalues 1 (the Goldstone modes of
//!          the four broken generators), in the first three; all others below 1; the condensate of one doublet is
//!          unstable in the second version (an eigenvalue above 1);
//!   SF-10  the grid: the ground states of HS (1, 1) and CJT of the second version on 24 and 48 radial nodes;
//!   SF-11  the lowest quasi-particle: in the first version the unpaired up component of Psi_LL (energy |M_u|), in the
//!          second version a paired one, with a larger energy;
//!   SF-12  the first version computed by this program (sixteen components, chi'_SH decoupled) equals the first version
//!          computed on the twelve components of Psi_LL and chi_SH alone, and the block of chi'_SH stays empty;
//!   SF-13  V^(1)_SH, both versions: every stationary state other than the ground state (the normal vacuum, the condensate
//!          of one doublet where the iteration stays on it, the scalar condensate of KS and CJT) is unstable towards the
//!          Higgs superfluid: its static response in the doublet-singlet pairs has an eigenvalue above 1.
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_superfluid
//! Output in data/: hck_sh_superfluid_checks.txt and the data files sh_sf_*.csv.  Run time about fifteen minutes on two
//! threads.
mod model;

use model::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{beta, cmat_c, d8, gamma_ch, k_scalar, sigma, t_gen};
use sfcore::methods::{grid, terms, Method, Potential};
use sfcore::quad::gauss_legendre;
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::uniform::Model;
use sfcore::uniform_n::{n_dirs, p_dirs, response_eigs, Dir, FieldBlock, ModelN, SolutionN, TermKindN, TermN};
use std::f64::consts::PI;
use std::time::Instant;

/// the potentials of the revised hypotheses
const V0_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 0.0, gsh2: 0.0 };
const V1_SH_FIRST: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 0.0 };
const V1_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 100.8 };
/// the self-consistency: mixing, tolerance of the residual of the self-energies, maximum of iterations
const MIX: f64 = 0.6;
const TOL: f64 = 1e-12;
const MAXIT: usize = 40000;
/// the step of the central differences of the static response
const H_RESP: f64 = 1e-5;
/// the size below which an element of a self-energy is set to zero during the iteration (see solve_clean)
const FLUSH: f64 = 1e-30;

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

/// I1(M) = int_0^Lambda q^2 dq / sqrt(q^2 + M^2) (closed form)
fn i1(mq: f64) -> f64 {
    0.5 * ((1.0 + mq * mq).sqrt() - mq * mq * (1.0 / mq).asinh())
}
/// the scalar susceptibility of the normal vacuum of an isodoublet (eight components), -dn_S/dM =
/// (2/pi^2) int_0^Lambda q^4 / (q^2 + m^2)^(3/2) dq (Gauss-Legendre, 400 nodes); a singlet has half of it
fn chi_s8(m: f64) -> f64 {
    let (x, w) = gauss_legendre(400);
    let mut s = 0.0;
    for (t, wt) in x.iter().zip(&w) {
        let q = 0.5 * (t + 1.0);
        s += 0.5 * wt * q.powi(4) / (q * q + m * m).powf(1.5);
    }
    2.0 / (PI * PI) * s
}

/// the pair self-energy of a bidoublet of coefficients m[s][a]: sum_{s, a} m_sa K(a, s)^dag
fn bidoublet(m: [[C64; 2]; 2]) -> CMat {
    let mut p = CMat::zeros(N);
    for s in 0..2 {
        for a in 0..2 {
            p = p.add(&k_ds(a, s).dagger().scale(m[s][a]));
        }
    }
    p
}

/// the starts of the survey: name, N0, P0
fn starts() -> Vec<(&'static str, CMat, CMat)> {
    let z = C64::ZERO;
    let r = |x: f64| c(x, 0.0);
    let i_ = |x: f64| c(0.0, x);
    let bb = CMat::block_diag(&d8(&beta()), &CMat::block_diag(&beta(), &beta()));
    // one doublet: Phi^d, Phi^u, the generic direction (0.6, 0.8 i) of Phi_SH, and Phi'^d (a doublet of the second
    // field in its charged component)
    let kd = bidoublet([[z, z], [r(0.3), z]]);
    let ku = bidoublet([[r(0.3), z], [z, z]]);
    let kg = bidoublet([[r(0.18), z], [i_(0.24), z]]);
    let kd2 = bidoublet([[z, z], [z, r(0.3)]]);
    // the custodial vacuum 0.3 i tau^2 (Phi^d and Phi'^u), a rotated unitary bidoublet 0.3 ((0.6, 0.8 i), (0.8 i, 0.6)),
    // and a generic bidoublet with two different singular values
    let kc = bidoublet([[z, r(0.3)], [r(-0.3), z]]);
    let kr = bidoublet([[r(0.18), i_(0.24)], [i_(0.24), r(0.18)]]);
    let kgen = bidoublet([[r(0.15), i_(0.09)], [r(-0.06), r(0.21)]]);
    let ktrip = emb_psi(&k_scalar(3).dagger().scale_r(0.3));
    // the scalar pairs of two particles of one singlet, chi^T C gamma^ch chi, and of the two singlets
    let cg = cmat_c().mul(&gamma_ch()).dagger();
    let kchi = emb_sing(0, &cg.scale_r(0.3));
    let kchi2 = emb_sing(1, &cg.scale_r(0.3));
    let mut kcc = CMat::zeros(N);
    kcc.put(OFF_CHI, OFF_CHI2, &cg.scale_r(0.3));
    kcc.put(OFF_CHI2, OFF_CHI, &cg.transpose().scale_r(-0.3));
    // a mixing of Psi_LL with chi_SH, Psi_u^dag beta chi + h.c., and of the two singlets, chi^dag beta chi' + h.c.
    let b = beta();
    let mut mix = CMat::zeros(N);
    let mut mixs = CMat::zeros(N);
    for dd in 0..4 {
        for ee in 0..4 {
            let v = b.get(dd, ee).scale(0.3);
            mix.set(2 * dd, OFF_CHI + ee, v);
            mix.set(OFF_CHI + ee, 2 * dd, v.conj());
            mixs.set(OFF_CHI + dd, OFF_CHI2 + ee, v);
            mixs.set(OFF_CHI2 + ee, OFF_CHI + dd, v.conj());
        }
    }
    let zz = CMat::zeros(N);
    vec![
        ("normal", zz.clone(), zz.clone()),
        ("Higgs down", zz.clone(), kd.clone()),
        ("Higgs up", zz.clone(), ku),
        ("Higgs generic", zz.clone(), kg),
        ("second doublet down", zz.clone(), kd2),
        ("custodial", zz.clone(), kc.clone()),
        ("custodial rotated", zz.clone(), kr),
        ("bidoublet generic", zz.clone(), kgen),
        ("pair of Psi", zz.clone(), ktrip),
        ("pair of chi", zz.clone(), kchi),
        ("pair of chi'", zz.clone(), kchi2),
        ("pair of chi and chi'", zz.clone(), kcc),
        ("mass up", bb.scale_r(0.5), zz.clone()),
        ("mass down", bb.scale_r(-1.0), zz.clone()),
        ("mass down and custodial", bb.scale_r(-1.0), kc),
        ("mixing of Psi and chi", mix, zz.clone()),
        ("mixing of chi and chi'", mixs, zz.clone()),
        ("isospin polarized", tau3_beta(), zz.clone()),
        ("singlets polarized", tau3r_beta(), zz.clone()),
    ]
}

/// the self-consistent solution by the iteration of ModelN::solve of the library (linear mixing of the self-energies with
/// the weight MIX, N made Hermitian and P antisymmetric after each step, the same stopping rule), with two additions:
///   * after each step the elements of the self-energies below FLUSH in size are set to zero.  A channel that no term
///     feeds decays geometrically under the mixing (by the factor 1 - MIX per step); after some hundred steps its
///     elements reach the range where their squares underflow, and the Hermitian eigensolver of sfcore then fails (it
///     returned NaN for the start along Phi'^d in the first version, where chi'_SH is decoupled);
///   * an optional projection of the self-energies after each step onto a subspace that the iteration keeps: used for
///     the condensate of one doublet of the second version, a stationary state that is unstable, so that the rounding
///     errors of the diagonalizations would otherwise carry the iteration to the custodial state.
fn solve_clean(mo: &ModelN, n0: &CMat, p0: &CMat, proj: Option<&dyn Fn(&mut CMat, &mut CMat)>) -> SolutionN {
    let flush = |m: &mut CMat| {
        for z in m.a.iter_mut() {
            if z.abs() < FLUSH {
                *z = C64::ZERO;
            }
        }
    };
    let mut nself = n0.clone();
    let mut pself = p0.clone();
    if let Some(pr) = proj {
        pr(&mut nself, &mut pself);
    }
    let mut it = 0;
    loop {
        let (cn, cp) = mo.states(&nself, &pself);
        let (drho, kap, ek) = mo.local(&cn, &cp);
        let (ei, n2, p2) = mo.interaction(&drho, &kap);
        let res = n2.sub(&nself).max_abs().max(p2.sub(&pself).max_abs());
        if res < TOL || it >= MAXIT {
            return SolutionN { n_self: n2, p_self: p2, drho, kap, energy: ek + ei, e_kin: ek, e_int: ei, iterations: it, residual: res };
        }
        nself = n2.scale_r(MIX).add(&nself.scale_r(1.0 - MIX));
        pself = p2.scale_r(MIX).add(&pself.scale_r(1.0 - MIX));
        nself = nself.add(&nself.dagger()).scale_r(0.5);
        pself = pself.sub(&pself.transpose()).scale_r(0.5);
        flush(&mut nself);
        flush(&mut pself);
        if let Some(pr) = proj {
            pr(&mut nself, &mut pself);
        }
        it += 1;
    }
}

/// the projection onto the self-energies that keep the number of particles of chi'_SH: no pair that contains chi'_SH and
/// no normal block between chi'_SH and the other fields (the subspace of the condensate of one doublet, Phi_SH)
fn keep_chi2_number(n: &mut CMat, p: &mut CMat) {
    let c2 = species_indices(3);
    for i in 0..N {
        for &j in &c2 {
            p.set(i, j, C64::ZERO);
            p.set(j, i, C64::ZERO);
            if !c2.contains(&i) {
                n.set(i, j, C64::ZERO);
                n.set(j, i, C64::ZERO);
            }
        }
    }
}

/// the kind of a stationary state
fn kind_of(d: &Describe) -> String {
    let (s1, s2) = d.singular();
    let charged = d.p_psipsi.max(d.p_ss).max(d.n_psis).max(d.n_chichi2);
    let massive = (d.mass[0] - M_LL).abs().max((d.mass[1] - M_LL).abs()).max((d.mass[2] - M_CHI).abs()).max((d.mass[3] - M_CHI).abs()) > 1e-6;
    if s1 > 1e-6 && d.p_rest < 1e-8 && charged < 1e-8 {
        if s2 > 1e-6 && (s1 - s2).abs() < 1e-6 * s1 {
            "Higgs superfluid, custodial".into()
        } else if s2 < 1e-8 {
            "Higgs superfluid, one doublet".into()
        } else {
            "Higgs superfluid, unequal lengths".into()
        }
    } else if s1 > 1e-6 {
        "Higgs superfluid with a charged admixture".into()
    } else if d.p_psipsi > 1e-6 {
        "superfluid of pairs of Psi_LL".into()
    } else if d.p_ss > 1e-6 {
        "superfluid of pairs of singlets".into()
    } else if d.n_psis > 1e-6 {
        "mixing of Psi_LL with a singlet".into()
    } else if d.n_chichi2 > 1e-6 {
        "mixing of the two singlets".into()
    } else if massive {
        "massive (scalar condensate)".into()
    } else {
        "normal vacuum".into()
    }
}

/// a stationary state found from a start
#[derive(Clone)]
struct State {
    start: String,
    kind: String,
    sol: SolutionN,
    desc: Describe,
}

/// the survey of the starts for a model, sorted by energy
fn survey(mo: &ModelN) -> Vec<State> {
    let mut out = vec![];
    for (name, n0, p0) in starts() {
        let sol = solve_clean(mo, &n0, &p0, None);
        let desc = describe(&sol.n_self, &sol.p_self, &sol.drho);
        out.push(State { start: name.into(), kind: kind_of(&desc), sol, desc });
    }
    out.sort_by(|a, b| a.sol.energy.partial_cmp(&b.sol.energy).unwrap());
    out
}

/// the distinct states: two states are the same when they are of the same kind and their energies agree to 1e-11 of
/// their size or to 1e-14, whichever is larger.  The absolute floor 1e-14 is far above the rounding errors of the energy
/// of the normal vacuum (about 1e-17: the energy is measured from the normal vacuum, so it is a difference of integrals of
/// size 0.1) and far below the differences between distinct states (above 1e-6 here); without it the normal vacua reached
/// from different starts would count as different states because their energies, all zero, differ by rounding.
fn distinct(states: &[State]) -> Vec<(State, Vec<String>)> {
    let mut out: Vec<(State, Vec<String>)> = vec![];
    for s in states {
        if let Some(e) = out
            .iter_mut()
            .find(|e| e.0.kind == s.kind && (e.0.sol.energy - s.sol.energy).abs() < (1e-11 * s.sol.energy.abs()).max(1e-14))
        {
            e.1.push(s.start.clone());
        } else {
            out.push((s.clone(), vec![s.start.clone()]));
        }
    }
    out
}

/// the 3 x 3 response of the Dirac masses (M_Psi, M_chi, M_chi') of the normal vacuum: a change of the mass of field r by
/// the self-energy h beta_r changes the local matrices; the self-energies that the contact terms produce from that change,
/// projected onto beta_Psi, beta_chi and beta_chi', are the column r
fn scalar_response(mo: &ModelN, h: f64) -> [[f64; 3]; 3] {
    let bs = [emb_psi(&d8(&beta())), emb_sing(0, &beta()), emb_sing(1, &beta())];
    let z = CMat::zeros(N);
    let mut s = [[0.0; 3]; 3];
    for r in 0..3 {
        let (_, rp, kp) = mo.energy_of(&bs[r].scale_r(h), &z);
        let (_, rm, km) = mo.energy_of(&bs[r].scale_r(-h), &z);
        let (n2, _) = mo.self_energy_lin(&rp.sub(&rm).scale_r(0.5 / h), &kp.sub(&km).scale_r(0.5 / h));
        for k in 0..3 {
            let nrm = bs[k].mul(&bs[k]).trace().re;
            s[k][r] = bs[k].mul(&n2).trace().re / nrm;
        }
    }
    s
}

/// the response of the normal vacuum along one pair direction K^dag: the coefficient of K^dag in the pair self-energy
/// that the contact terms produce from the pair amplitude caused by the pair self-energy h K^dag, divided by h
fn pair_channel_response(mo: &ModelN, k: &CMat, h: f64) -> f64 {
    let z = CMat::zeros(N);
    let kd = k.dagger();
    let (_, rp, kp) = mo.energy_of(&z, &kd.scale_r(h));
    let (_, rm, km) = mo.energy_of(&z, &kd.scale_r(-h));
    let (_, p2) = mo.self_energy_lin(&rp.sub(&rm).scale_r(0.5 / h), &kp.sub(&km).scale_r(0.5 / h));
    kd.dagger().mul(&p2).trace().re / kd.dagger().mul(&kd).trace().re
}

/// the directions of the six blocks of the self-energies
fn dirs_blocks() -> Vec<(&'static str, Vec<Dir>)> {
    let (ps, ss) = (psi_indices(), sing_indices());
    let (c1, c2) = (species_indices(2), species_indices(3));
    let mut ndiag = n_dirs(&ps, &ps);
    ndiag.extend(n_dirs(&c1, &c1));
    ndiag.extend(n_dirs(&c2, &c2));
    vec![
        ("pairs Psi with singlets", p_dirs(&ps, &ss)),
        ("pairs Psi Psi", p_dirs(&ps, &ps)),
        ("pairs of singlets", p_dirs(&ss, &ss)),
        ("normal, each field", ndiag),
        ("normal, mixing of the singlets", n_dirs(&c1, &c2)),
        ("normal, mixing Psi with singlets", n_dirs(&ps, &ss)),
    ]
}

/// the eight generators of the global group of H4 on the sixteen components, in the order T^1, T^2, T^3, T^1_R, T^2_R,
/// T^3_R, R, F_tot, and the hypercharge Y = T^3_R + R/4 (index 8)
fn generators() -> Vec<CMat> {
    let mut g: Vec<CMat> = (1..=3).map(|a| emb_psi(&t_gen(a))).collect();
    for a in 1..=3 {
        g.push(emb_pair(&sigma(a).scale_r(0.5).kron(&CMat::identity(4))));
    }
    let r = CMat::block_diag(&CMat::identity(NP), &CMat::identity(NS).scale_r(-1.0));
    g.push(r.clone());
    g.push(CMat::identity(N));
    g.push(g[5].add(&r.scale_r(0.25)));
    g
}

/// the stabilizer of a state (N, P) in the real span of a list of generators: the real coefficients c with [X_c, N] = 0
/// and X_c P + P X_c^T = 0 (X_c = sum c_a X_a); returns an orthonormal basis of the null space
fn stabilizer(nself: &CMat, pself: &CMat, gens: &[CMat]) -> Vec<Vec<f64>> {
    let na = gens.len();
    let mut cols: Vec<Vec<f64>> = vec![];
    for x in gens {
        let a = x.comm(nself);
        let b = x.mul(pself).add(&pself.mul(&x.transpose()));
        let mut v = vec![];
        for z in a.a.iter().chain(b.a.iter()) {
            v.push(z.re);
            v.push(z.im);
        }
        cols.push(v);
    }
    let mut ata = CMat::zeros(na);
    for i in 0..na {
        for j in 0..na {
            let s: f64 = cols[i].iter().zip(&cols[j]).map(|(x, y)| x * y).sum();
            ata.set(i, j, c(s, 0.0));
        }
    }
    let scale = (0..na).map(|i| ata.get(i, i).re).fold(0.0f64, f64::max).max(1e-300);
    let (ev, v) = eigh(&ata);
    let mut out = vec![];
    for j in 0..na {
        // the eigenvalues counted as zero are at the level of the rounding of eigh (about 1e-16 of the scale); the others
        // are of the order of the scale (the squared pair self-energy)
        if ev[j].abs() < 1e-9 * scale {
            out.push((0..na).map(|i| v.get(i, j).re).collect());
        }
    }
    out
}

/// the distance of Q^SM = T^3 + T^3_R + R/4 from the span of a stabilizer (an orthonormal list of coefficient vectors in
/// the order T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot): the length of the part of the unit vector of Q^SM outside the
/// span (zero when the state keeps the electric charge)
fn q_outside(st: &[Vec<f64>]) -> f64 {
    let mut q = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.25, 0.0];
    let nq = q.iter().map(|x| x * x).sum::<f64>().sqrt();
    for x in q.iter_mut() {
        *x /= nq;
    }
    let mut rest = q.clone();
    for v in st {
        let cf: f64 = v.iter().zip(&q).map(|(a, b)| a * b).sum();
        for (r, a) in rest.iter_mut().zip(v) {
            *r -= cf * a;
        }
    }
    rest.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// the results of one method for one potential
struct MethodResult {
    method: Method,
    pot_name: String,
    // normal vacuum
    lam_higgs: f64,
    mult_higgs: usize,
    lam_pseudo: f64,
    smat: [[f64; 3]; 3],
    lam_s: f64,
    lam_trip: f64,
    lam_blocks: Vec<(String, f64)>,
    // states
    states: Vec<(State, Vec<String>)>,
    all: Vec<State>,
    /// the ground state reached from the reference start (custodial in the second version, Phi^d in the first), when it
    /// has the energy of the lowest state
    ground: Option<State>,
    /// the static response of the ground state: (eigenvalues equal to 1, deviation from 1, largest other in the sector
    /// R = 0, largest in the sector R = +-2)
    resp: Option<(usize, f64, f64, f64)>,
    /// the static response of the stationary states other than the ground state (aligned with `states`; None for the
    /// ground state and for V^(0)_SH): the largest eigenvalue in the pairs of Psi_LL with the singlets and the largest in
    /// the normal self-energies of the fields
    lam_states: Vec<Option<(f64, f64)>>,
    /// the condensate of one doublet in the second version: the largest eigenvalue of its static response, its energy
    lam_one_doublet: Option<f64>,
    e_one_doublet: Option<f64>,
    /// the stabilizer of the ground state: (dimension in the global group, dimension in SU(2)_L x U(1)_Y, the length of
    /// the part of the unit vector of Q^SM outside it)
    stab: Option<(usize, usize, f64)>,
    gap: Option<(f64, f64)>,
    sym_dev: f64,
    chi_min: f64,
    seconds: f64,
}

fn run_method(method: Method, pname: &str, pot: PotSh) -> MethodResult {
    let t0 = Instant::now();
    let mo = model(&method, &pot, NR);
    let z = CMat::zeros(N);
    let second = pot.gsh2 != 0.0;
    // the static response of the normal vacuum, block by block
    let mut lam_blocks = vec![];
    let mut lam_higgs = 0.0;
    let mut mult_higgs = 0;
    let mut sym_dev = 0.0f64;
    let mut chi_min = f64::INFINITY;
    for (name, dirs) in dirs_blocks() {
        let (re, a_chi, a_k, amin) = response_eigs(&mo, &z, &z, &dirs, H_RESP);
        sym_dev = sym_dev.max(a_chi).max(a_k);
        chi_min = chi_min.min(amin);
        if name == "pairs Psi with singlets" {
            lam_higgs = re[0];
            mult_higgs = re.iter().filter(|x| (*x - re[0]).abs() < 1e-8 * (1.0 + re[0].abs())).count();
        }
        lam_blocks.push((name.to_string(), re[0]));
    }
    // the pseudoscalar doublet-singlet pair matrix: ((0, W), (-W^T, 0)) with W = (1/2) C (x) e_down, with each singlet
    let mut lam_pseudo = 0.0f64;
    for which in 0..2 {
        let w = cmat_c().kron(&e_iso(1)).scale_r(0.5);
        let o = OFF_CHI + NX * which;
        let mut k = CMat::zeros(N);
        k.put(0, o, &w);
        k.put(o, 0, &w.transpose().scale_r(-1.0));
        lam_pseudo = lam_pseudo.max(pair_channel_response(&mo, &k, H_RESP).abs());
    }
    let smat = scalar_response(&mo, H_RESP);
    // the largest eigenvalue of the 3 x 3 response S = G X (G the symmetric matrix of the effective couplings, X the
    // diagonal matrix of the scalar susceptibilities of the three fields): the eigenvalues of S are those of the symmetric
    // matrix X^(1/2) G X^(1/2) = X^(1/2) S X^(-1/2), whose two triangles are averaged before the Hermitian eigenproblem
    let lam_s = {
        let xs = [chi_s8(M_LL), 0.5 * chi_s8(M_CHI), 0.5 * chi_s8(M_CHI)];
        let mut sym = CMat::zeros(3);
        for k in 0..3 {
            for r in 0..3 {
                let a = xs[k].sqrt() * smat[k][r] / xs[r].sqrt();
                let b = xs[r].sqrt() * smat[r][k] / xs[k].sqrt();
                sym.set(k, r, c(0.5 * (a + b), 0.0));
            }
        }
        eigh(&sym).0[2]
    };
    let lam_trip = pair_channel_response(&mo, &emb_psi(&k_scalar(3)), H_RESP);
    // the survey
    let all = survey(&mo);
    let states = distinct(&all);
    let mut resp = None;
    let mut stab = None;
    let mut gap = None;
    let mut ground = None;
    let mut lam_one_doublet = None;
    let mut e_one_doublet = None;
    let g = &states[0].0;
    let (ref_start, ref_kind) = if second { ("custodial", "Higgs superfluid, custodial") } else { ("Higgs down", "Higgs superfluid, one doublet") };
    let gd = all.iter().find(|s| s.start == ref_start).unwrap();
    if g.kind == ref_kind && gd.kind == g.kind && (gd.sol.energy - g.sol.energy).abs() < 1e-11 * g.sol.energy.abs() {
        ground = Some(gd.clone());
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
        let (ra, a1, a2, a3) = response_eigs(&mo, &gd.sol.n_self, &gd.sol.p_self, &block_a, H_RESP);
        let (rb, b1, b2, b3) = response_eigs(&mo, &gd.sol.n_self, &gd.sol.p_self, &block_b, H_RESP);
        sym_dev = sym_dev.max(a1).max(a2).max(b1).max(b2);
        chi_min = chi_min.min(a3).min(b3);
        let n_one = ra.iter().filter(|x| (*x - 1.0).abs() < 1e-6).count();
        let max_other_a = ra.iter().filter(|x| (*x - 1.0).abs() >= 1e-6).fold(f64::NEG_INFINITY, |s, x| s.max(*x));
        let dev_one = ra.iter().filter(|x| (*x - 1.0).abs() < 1e-6).fold(0.0f64, |s, x| s.max((x - 1.0).abs()));
        resp = Some((n_one, dev_one, max_other_a, rb[0]));
        // the stabilizer in the global group and in SU(2)_L x U(1)_Y
        let gens = generators();
        let st8 = stabilizer(&gd.sol.n_self, &gd.sol.p_self, &gens[0..8]);
        let gy: Vec<CMat> = vec![gens[0].clone(), gens[1].clone(), gens[2].clone(), gens[8].clone()];
        let st4 = stabilizer(&gd.sol.n_self, &gd.sol.p_self, &gy);
        stab = Some((st8.len(), st4.len(), q_outside(&st8)));
        gap = Some(mo.gap(&gd.sol.n_self, &gd.sol.p_self));
        // in the second version: the condensate of one doublet, from the start along Phi^d by the iteration restricted
        // to the self-energies that keep the number of chi'_SH, is a stationary state of higher energy; its energy and
        // the largest eigenvalue of its static response in the pairs of Psi with the singlets
        if second {
            let p0 = bidoublet([[C64::ZERO, C64::ZERO], [c(0.3, 0.0), C64::ZERO]]);
            let od = solve_clean(&mo, &CMat::zeros(N), &p0, Some(&keep_chi2_number));
            let dd = describe(&od.n_self, &od.p_self, &od.drho);
            if kind_of(&dd) == "Higgs superfluid, one doublet" && od.residual < TOL {
                let (ro, _, _, _) = response_eigs(&mo, &od.n_self, &od.p_self, &p_dirs(&ps, &ss), H_RESP);
                lam_one_doublet = Some(ro[0]);
                e_one_doublet = Some(od.energy);
            }
        }
    }
    // the static response of the other stationary states of V^(1)_SH: the largest eigenvalue in the pairs of Psi_LL with
    // the singlets (above 1: the state is unstable towards the Higgs superfluid) and the largest in the normal
    // self-energies of the fields, the directions without pairs (all below 1: a local minimum among the states without
    // pairs)
    let mut lam_states: Vec<Option<(f64, f64)>> = vec![None; states.len()];
    if pot.gsh != 0.0 {
        let (ps, ss) = (psi_indices(), sing_indices());
        let (c1, c2) = (species_indices(2), species_indices(3));
        let mut nd = n_dirs(&ps, &ps);
        nd.extend(n_dirs(&c1, &c1));
        nd.extend(n_dirs(&c2, &c2));
        nd.extend(n_dirs(&c1, &c2));
        for (i, (s, _)) in states.iter().enumerate().skip(1) {
            let (rp, a1, a2, a3) = response_eigs(&mo, &s.sol.n_self, &s.sol.p_self, &p_dirs(&ps, &ss), H_RESP);
            let (rn, b1, b2, b3) = response_eigs(&mo, &s.sol.n_self, &s.sol.p_self, &nd, H_RESP);
            sym_dev = sym_dev.max(a1).max(a2).max(b1).max(b2);
            chi_min = chi_min.min(a3).min(b3);
            lam_states[i] = Some((rp[0], rn[0]));
        }
    }
    MethodResult {
        method,
        pot_name: pname.into(),
        lam_higgs,
        mult_higgs,
        lam_pseudo,
        smat,
        lam_s,
        lam_trip,
        lam_blocks,
        states,
        all,
        ground,
        resp,
        lam_states,
        lam_one_doublet,
        e_one_doublet,
        stab,
        gap,
        sym_dev,
        chi_min,
        seconds: t0.elapsed().as_secs_f64(),
    }
}

/// the model of the first version on the twelve components of Psi_LL and chi_SH alone (the program of stage 3 of the
/// first version): the terms of the sixteen-component model of V^(1)_SH of the first version cut to the first twelve
/// components, with the Kohn-Sham densities of the first version (those of Psi_LL and the number and scalar densities of
/// chi_SH)
fn model12(method: &Method) -> ModelN {
    let cut = |m: &CMat| m.sub_block(0, 0, NP + NX, NP + NX);
    let b = beta();
    let mut ks12 = vec![];
    for a in 0..4 {
        ks12.push(cut(&emb_psi(&CMat::identity(4).kron(&sigma(a)))));
        ks12.push(cut(&emb_psi(&b.kron(&sigma(a)))));
    }
    ks12.push(cut(&emb_sing(0, &CMat::identity(4))));
    ks12.push(cut(&emb_sing(0, &b)));
    let t16 = terms_sh(method, &V1_SH_FIRST);
    let t12: Vec<TermN> = t16
        .iter()
        .map(|t| TermN {
            kind: match &t.kind {
                TermKindN::Bilinear { nu, x, y } => TermKindN::Bilinear { nu: *nu, x: cut(x), y: cut(y) },
                TermKindN::Pairing { g, k } => TermKindN::Pairing { g: *g, k: cut(k) },
            },
            w_h: t.w_h,
            w_f: t.w_f,
            w_p: t.w_p,
            ks_basis: t.ks_basis.as_ref().map(|_| ks12.clone()),
            label: t.label.clone(),
        })
        .collect();
    let blocks12 = vec![FieldBlock { off: 0, ncomp: NP, mass: M_LL }, FieldBlock { off: NP, ncomp: NX, mass: M_CHI }];
    ModelN::new(blocks12, 0.0, t12, grid(NR))
}

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sh_superfluid: checks of the mean-field superfluid of the revised hypotheses, second version, with the first \
         version as the comparison (stage 3, Revision_SM_Higgs, PROJECT spacetimes)",
    );
    let mut vals: Vec<(String, f64)> = vec![];

    // ------------------------------------------------------------------ SF-1: the engine against the earlier engine
    let v2 = Potential { gs: 2.8, gv: 0.8, g: 25.2 };
    let mut d1 = 0.0f64;
    let mut d1k = 0.0f64;
    for m in methods() {
        let old = Model::new(M_LL, 0.0, terms(&m, &v2), grid(NR));
        let so = old.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), MIX, TOL, MAXIT);
        let new = ModelN::new(blocks(), 0.0, terms_earlier(&m, v2.gs, v2.gv, v2.g), grid(NR));
        let sn = new.solve(&CMat::zeros(N), &emb_psi(&k_scalar(3).dagger().scale_r(0.3)), MIX, TOL, MAXIT);
        d1 = d1.max((so.energy - sn.energy).abs());
        let kn = sn.kap.sub_block(0, 0, 8, 8);
        let rn = sn.drho.sub_block(0, 0, 8, 8);
        d1k = d1k.max(kn.sub(&so.kap).max_abs()).max(rn.sub(&so.drho).max_abs());
        // the blocks of the singlets stay empty
        d1k = d1k.max(sn.kap.sub_block(NP, NP, NS, NS).max_abs()).max(sn.drho.sub_block(NP, NP, NS, NS).max_abs());
        vals.push((format!("sf1_energy_{}", m.short()), sn.energy));
    }
    rep.add(
        "SF-1a",
        "the energy of the earlier engine (module uniform) and of the new engine (module uniform_n, the two singlet fields \
         decoupled) on the earlier V^(2) from the polar start, five methods",
        d1,
        1e-13,
    );
    rep.add("SF-1b", "the local matrices of the two engines agree, and the blocks of the singlets stay empty", d1k, 1e-12);
    println!("SF-1 done ({:.0} s)", t0.elapsed().as_secs_f64());

    // ------------------------------------------------------------------ the methods, in parallel (two at a time)
    let mut jobs: Vec<(Method, &str, PotSh)> = vec![];
    for m in methods() {
        jobs.push((m, "V0_SH", V0_SH));
        jobs.push((m, "V1_SH first version", V1_SH_FIRST));
        jobs.push((m, "V1_SH", V1_SH));
    }
    let nthreads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2).max(1);
    let next = std::sync::atomic::AtomicUsize::new(0);
    let slots: Vec<std::sync::Mutex<Option<MethodResult>>> = jobs.iter().map(|_| std::sync::Mutex::new(None)).collect();
    std::thread::scope(|sc| {
        for _ in 0..nthreads {
            sc.spawn(|| loop {
                let i = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                if i >= jobs.len() {
                    break;
                }
                let (m, n, p) = jobs[i];
                let r = run_method(m, n, p);
                println!("  {} {} done ({:.0} s)", n, mname(&m), r.seconds);
                *slots[i].lock().unwrap() = Some(r);
            });
        }
    });
    let results: Vec<MethodResult> = slots.into_iter().map(|s| s.into_inner().unwrap().unwrap()).collect();
    println!("methods done ({:.0} s)", t0.elapsed().as_secs_f64());

    // ------------------------------------------------------------------ SF-2 to SF-4: the normal vacuum
    let gcrit = 4.0 * PI * PI / i1(M_LL);
    let gcrit_ll = PI * PI / i1(M_LL);
    vals.push(("G_crit_SH".into(), gcrit));
    vals.push(("G_crit_LL".into(), gcrit_ll));
    let chi8 = chi_s8(M_LL);
    let chi4 = 0.5 * chi_s8(M_CHI);
    vals.push(("chi_S_Psi".into(), chi8));
    vals.push(("chi_S_chi".into(), chi4));
    let pot_of = |n: &str| match n {
        "V0_SH" => V0_SH,
        "V1_SH first version" => V1_SH_FIRST,
        _ => V1_SH,
    };
    let (mut d2, mut d2m, mut d2p) = (0.0f64, true, 0.0f64);
    let mut d2m_detail = vec![];
    let (mut d3, mut ok3) = (0.0f64, true);
    let (mut d4, mut ok4) = (0.0f64, true);
    let mut ok4_detail = vec![];
    for r in &results {
        let pot = pot_of(&r.pot_name);
        let want = pot.gsh / gcrit;
        d2 = d2.max((r.lam_higgs - want).abs());
        if want > 0.0 {
            let wm = if pot.gsh2 != 0.0 { 8 } else { 4 };
            d2m &= r.mult_higgs == wm;
            d2m_detail.push(format!("{} {}: {}", r.pot_name, mname(&r.method), r.mult_higgs));
        }
        d2p = d2p.max(r.lam_pseudo);
        let (gpp, gpx, gpx2) = match r.method {
            Method::Hs { s_s, .. } => (s_s * pot.gs, 0.0, 0.0),
            _ => (0.875 * pot.gs - 0.5 * pot.gv, -pot.gsh / 16.0, -pot.gsh2 / 16.0),
        };
        let sw = [[-gpp * chi8, -gpx * chi4, -gpx2 * chi4], [-gpx * chi8, 0.0, 0.0], [-gpx2 * chi8, 0.0, 0.0]];
        for a in 0..3 {
            for b in 0..3 {
                d3 = d3.max((r.smat[a][b] - sw[a][b]).abs());
            }
        }
        ok3 &= r.lam_s < 1.0;
        let gtrip = match r.method {
            Method::Hs { s_s, s_v } => -0.25 * (1.0 - s_s) * pot.gs - (1.0 - s_v) * pot.gv,
            _ => -0.25 * pot.gs - pot.gv,
        };
        d4 = d4.max((r.lam_trip - gtrip / gcrit_ll).abs());
        for (name, lam) in &r.lam_blocks {
            if name != "pairs Psi with singlets" {
                ok4 &= *lam < 1.0;
                ok4_detail.push(format!("{} {} {}: {:.4}", r.pot_name, mname(&r.method), name, lam));
            }
        }
    }
    rep.add(
        "SF-2a",
        &format!(
            "normal vacuum, pairs of Psi_LL with the singlets: the largest eigenvalue of the static response is G_SH / \
             G^crit_SH with G^crit_SH = 4 pi^2 / I1(m) = {:.10} (V^(1)_SH, both versions: {:.10}; V^(0)_SH: 0), five methods",
            gcrit,
            V1_SH.gsh / gcrit
        ),
        d2,
        1e-9,
    );
    rep.add_bool(
        "SF-2b",
        "the eigenvalue G_SH / G^crit_SH is eightfold in the second version (two doublets, each component with its real and \
         imaginary part) and fourfold in the first version",
        d2m,
        &d2m_detail.join("; "),
    );
    rep.add(
        "SF-2c",
        "the pseudoscalar doublet-singlet pairs (matrix C (x) e_s, with either singlet) have no interaction in the normal \
         vacuum (response 0)",
        d2p,
        1e-10,
    );
    rep.add(
        "SF-3a",
        &format!(
            "normal vacuum, scalar channel: the 3 x 3 response of the masses of Psi_LL, chi_SH and chi'_SH agrees with the \
             effective couplings (7/8 G^S - 1/2 G^V on Psi_LL in KS and CJT, s_S G^S in HS; the cross couplings -G_SH/16 of \
             each paired singlet in KS and CJT) and the susceptibilities chi_S = {:.10} (Psi_LL), {:.10} (a singlet)",
            chi8, chi4
        ),
        d3,
        1e-9,
    );
    rep.add_bool(
        "SF-3b",
        "the scalar channel of the normal vacuum is stable (largest eigenvalue of the 3 x 3 response below 1) for all \
         potentials and methods",
        ok3,
        &results.iter().map(|r| format!("{} {}: {:.4}", r.pot_name, mname(&r.method), r.lam_s)).collect::<Vec<_>>().join("; "),
    );
    rep.add(
        "SF-4a",
        "normal vacuum, scalar triplet pairs of Psi_LL (K^3): the response agrees with g_P / G^crit_LL, g_P = -(1/4) G^S - \
         G^V (KS, CJT) or -(1/4)(1 - s_S) G^S - (1 - s_V) G^V (HS)",
        d4,
        1e-9,
    );
    rep.add_bool(
        "SF-4b",
        "normal vacuum: every eigenvalue of the static response in the pairs of Psi_LL, the pairs of the singlets, the \
         normal self-energies of each field and the mixings is below 1, for all potentials and methods: only the \
         doublet-singlet pairs can condense",
        ok4,
        &ok4_detail.join("; "),
    );
    let sdev = results.iter().fold(0.0f64, |m, r| m.max(r.sym_dev));
    let cmin = results.iter().fold(f64::INFINITY, |m, r| m.min(r.chi_min));
    rep.add_bool(
        "SF-4c",
        "the eigenvalues of the static response are those of a symmetric matrix: the response of the densities to the \
         self-energies and the kernel of the contact terms are symmetric in an orthonormal basis (largest asymmetry below \
         1e-7), and the response of the densities is negative semidefinite (smallest eigenvalue of its negative above -1e-9)",
        sdev < 1e-7 && cmin > -1e-9,
        &format!("largest asymmetry {:.1e}; smallest eigenvalue {:.1e}", sdev, cmin),
    );

    // ------------------------------------------------------------------ SF-5 to SF-11: the states
    let v0: Vec<&MethodResult> = results.iter().filter(|r| r.pot_name == "V0_SH").collect();
    let ok5 = v0.iter().all(|r| r.states.len() == 1 && r.states[0].0.kind == "normal vacuum");
    rep.add_bool(
        "SF-5",
        &format!("V^(0)_SH: every start of every method ends in the normal vacuum ({} starts)", starts().len()),
        ok5,
        &v0.iter().map(|r| format!("{}: {} state(s)", mname(&r.method), r.states.len())).collect::<Vec<_>>().join("; "),
    );
    let w2: Vec<&MethodResult> = results.iter().filter(|r| r.pot_name == "V1_SH").collect();
    let w1: Vec<&MethodResult> = results.iter().filter(|r| r.pot_name == "V1_SH first version").collect();
    let ok6 = w2.iter().all(|r| r.states[0].0.kind == "Higgs superfluid, custodial" && r.ground.is_some());
    rep.add_bool(
        "SF-6a",
        "V^(1)_SH, second version: the ground state of every method is the custodial Higgs superfluid (both pair fields, \
         two equal singular values of the bidoublet, no charged pair, no mixing), the stationary state of lowest energy \
         among those reached from the starts",
        ok6,
        &w2.iter().map(|r| format!("{}: {} (E = {:.6e})", mname(&r.method), r.states[0].0.kind, r.states[0].0.sol.energy)).collect::<Vec<_>>().join("; "),
    );
    let ok6b = w1.iter().all(|r| r.states[0].0.kind == "Higgs superfluid, one doublet" && r.ground.is_some());
    rep.add_bool(
        "SF-6b",
        "V^(1)_SH, first version: the ground state of every method is the superfluid of one doublet (Phi_SH alone), as in \
         stage 3 of the first version",
        ok6b,
        &w1.iter().map(|r| format!("{}: {} (E = {:.6e})", mname(&r.method), r.states[0].0.kind, r.states[0].0.sol.energy)).collect::<Vec<_>>().join("; "),
    );
    // SF-7: the starts of the second version, and the condensate of one doublet
    let mut d7 = 0.0f64;
    let mut det7a = vec![];
    for r in &w2 {
        let pick = |n: &str| r.all.iter().find(|s| s.start == n).unwrap();
        let refs = pick("custodial");
        let (e0, (s1, s2)) = (refs.sol.energy, refs.desc.singular());
        for n in ["custodial rotated", "bidoublet generic"] {
            let s = pick(n);
            let (t1, t2) = s.desc.singular();
            d7 = d7.max((s.sol.energy - e0).abs() / e0.abs()).max((t1 - s1).abs() / s1).max((t2 - s2).abs() / s1);
        }
        // the starts along one doublet, without the restriction: where they end
        let ends: Vec<String> = ["Higgs down", "Higgs up", "Higgs generic", "second doublet down"]
            .iter()
            .map(|n| {
                let s = pick(n);
                let k = match s.kind.as_str() {
                    "Higgs superfluid, custodial" => "custodial",
                    "Higgs superfluid, one doublet" => "one doublet",
                    _ => "other",
                };
                format!("{} {}", n, k)
            })
            .collect();
        det7a.push(format!("{}: {}", mname(&r.method), ends.join(", ")));
    }
    rep.add(
        "SF-7a",
        "second version: the starts along i tau^2, along a rotated unitary bidoublet and along a generic bidoublet reach \
         states of equal energy and equal singular values (relative deviation; the symmetry of H4)",
        d7,
        1e-9,
    );
    let mut ok7 = true;
    let mut det7 = vec![];
    for r in &w2 {
        let e0 = r.states[0].0.sol.energy;
        match r.e_one_doublet {
            Some(e1) => {
                ok7 &= e1 > e0;
                det7.push(format!("{}: one doublet {:.6e}, custodial {:.6e}", mname(&r.method), e1, e0));
            }
            None => {
                ok7 = false;
                det7.push(format!("{}: no condensate of one doublet", mname(&r.method)));
            }
        }
    }
    rep.add_bool(
        "SF-7b",
        &format!(
            "second version: the condensate of one doublet (Phi_SH alone), found by the iteration restricted to the \
             self-energies that keep the number of chi'_SH, is a stationary state of higher energy than the custodial one \
             (without the restriction the starts along one doublet end as follows: {})",
            det7a.join("; ")
        ),
        ok7,
        &det7.join("; "),
    );
    // SF-8: the stabilizers
    let ok8 = w2.iter().all(|r| match r.stab {
        Some((d8_, d4_, qd)) => d8_ == 4 && d4_ == 1 && qd < 1e-9,
        None => false,
    }) && w1.iter().all(|r| match r.stab {
        Some((d8_, d4_, qd)) => d8_ == 3 && d4_ == 1 && qd < 1e-9,
        None => false,
    });
    rep.add_bool(
        "SF-8",
        "the generators of the global group of H4 (T^a, T^a_R, R, F_tot) that leave the custodial ground state of each \
         method unchanged form a space of dimension 4 that contains Q^SM = T^3 + T^3_R + R/4; those of SU(2)_L x U(1)_Y a \
         space of dimension 1 (the photon stays massless); the ground state of the first version keeps a space of \
         dimension 3 that contains Q^SM",
        ok8,
        &results
            .iter()
            .filter(|r| r.pot_name != "V0_SH")
            .map(|r| format!("{} {}: {:?}", r.pot_name, mname(&r.method), r.stab.map(|s| (s.0, s.1))))
            .collect::<Vec<_>>()
            .join("; "),
    );
    // SF-9: the static response of the ground states
    let ok9 = w2.iter().all(|r| match r.resp {
        Some((n1, dev1, maxa, maxb)) => n1 == 4 && dev1 < 1e-6 && maxa < 1.0 && maxb < 1.0,
        None => false,
    }) && w1.iter().all(|r| match r.resp {
        Some((n1, dev1, maxa, maxb)) => n1 == 3 && dev1 < 1e-6 && maxa < 1.0 && maxb < 1.0,
        None => false,
    });
    let resp_detail = |r: &MethodResult| match r.resp {
        Some((n1, dev1, maxa, maxb)) => format!(
            "{} {}: {} eigenvalues 1 (to {:.1e}), largest other {:.5}, largest of the sector R = +-2 {:.5}",
            r.pot_name,
            mname(&r.method),
            n1,
            dev1,
            maxa,
            maxb
        ),
        None => format!("{} {}: no ground state", r.pot_name, mname(&r.method)),
    };
    rep.add_bool(
        "SF-9a",
        "the static response of each ground state: in the sector of relative number 0 exactly four eigenvalues equal 1 in \
         the second version (the Goldstone modes of the three broken generators of SU(2)_L x U(1)_Y and of F_tot) and \
         three in the first; all others below 1; in the sector of relative number +-2 all below 1: the ground states are \
         stable and no charged condensate grows",
        ok9,
        &results.iter().filter(|r| r.pot_name != "V0_SH").map(|r| resp_detail(r)).collect::<Vec<_>>().join("; "),
    );
    let ok9b = w2.iter().all(|r| r.lam_one_doublet.map_or(false, |l| l > 1.0 + 1e-6));
    rep.add_bool(
        "SF-9b",
        "second version: the condensate of one doublet is unstable (the largest eigenvalue of its static response in the \
         pairs of Psi_LL with the singlets is above 1, the direction of the second doublet)",
        ok9b,
        &w2.iter().map(|r| format!("{}: {:?}", mname(&r.method), r.lam_one_doublet.map(|l| (l * 1e6).round() / 1e6))).collect::<Vec<_>>().join("; "),
    );
    // SF-10: the grid
    let mut d10 = 0.0f64;
    let p0c = bidoublet([[C64::ZERO, c(0.3, 0.0)], [c(-0.3, 0.0), C64::ZERO]]);
    for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Cjt] {
        let a = solve_clean(&model(&m, &V1_SH, NR), &CMat::zeros(N), &p0c, None);
        let b = solve_clean(&model(&m, &V1_SH, 2 * NR), &CMat::zeros(N), &p0c, None);
        let (da, db) = (describe(&a.n_self, &a.p_self, &a.drho), describe(&b.n_self, &b.p_self, &b.drho));
        d10 = d10.max((a.energy - b.energy).abs() / b.energy.abs()).max((da.singular().0 - db.singular().0).abs() / db.singular().0);
        vals.push((format!("sf10_energy_48_{}", m.short()), b.energy));
    }
    rep.add(
        "SF-10",
        &format!(
            "the custodial ground states of HS (1, 1) and CJT on {} and {} radial nodes: relative change of the energy and of \
             the singular value of the bidoublet",
            NR,
            2 * NR
        ),
        d10,
        1e-6,
    );
    // SF-11: the lowest quasi-particle
    let mut d11 = 0.0f64;
    let mut q11 = 0.0f64;
    for r in &w1 {
        match (r.gap, &r.ground) {
            (Some((g, q)), Some(gs)) => {
                d11 = d11.max((g - gs.desc.mass[0].abs()).abs());
                q11 = q11.max(q);
            }
            _ => d11 = f64::INFINITY,
        }
    }
    rep.add(
        "SF-11a",
        "first version: the lowest quasi-particle of every Higgs ground state is the unpaired up component of Psi_LL \
         (electric charge 3/4): the gap equals its Dirac mass |M_u|",
        d11,
        1e-9,
    );
    rep.add("SF-11b", "first version: the minimum of the quasi-particle energy lies at zero momentum", q11, 1e-6);
    let mut ok11 = true;
    let mut det11 = vec![];
    for (r2, r1) in w2.iter().zip(&w1) {
        match (r2.gap, r1.gap, &r2.ground) {
            (Some((g2, q2)), Some((g1, _)), Some(gs)) => {
                let mmin = gs.desc.mass.iter().fold(f64::INFINITY, |m, x| m.min(x.abs()));
                ok11 &= g2 > g1 && g2 > mmin;
                det11.push(format!("{}: gap {:.6} at |q| = {:.3} (first version {:.6}; smallest |M| {:.6})", mname(&r2.method), g2, q2, g1, mmin));
            }
            _ => ok11 = false,
        }
    }
    rep.add_bool(
        "SF-11c",
        "second version: every constituent is paired; the gap of every custodial ground state is larger than that of the \
         first version and larger than the smallest Dirac mass of the four species",
        ok11,
        &det11.join("; "),
    );
    // SF-12: the first version on twelve components
    let mut d12 = 0.0f64;
    let mut d12b = 0.0f64;
    for r in &w1 {
        let g = match &r.ground {
            Some(g) => g,
            None => {
                d12 = f64::INFINITY;
                continue;
            }
        };
        let mo12 = model12(&r.method);
        let p0 = k_sh(1).dagger().scale_r(0.3).sub_block(0, 0, NP + NX, NP + NX);
        let s12 = solve_clean(&mo12, &CMat::zeros(NP + NX), &p0, None);
        d12 = d12.max((s12.energy - g.sol.energy).abs() / g.sol.energy.abs());
        d12 = d12.max(s12.kap.sub(&g.sol.kap.sub_block(0, 0, NP + NX, NP + NX)).max_abs());
        d12 = d12.max(s12.drho.sub(&g.sol.drho.sub_block(0, 0, NP + NX, NP + NX)).max_abs());
        // the rows and columns of chi'_SH stay empty
        let c2 = species_indices(3);
        for i in 0..N {
            for &j in &c2 {
                for m in [&g.sol.kap, &g.sol.drho, &g.sol.n_self, &g.sol.p_self] {
                    d12b = d12b.max(m.get(i, j).abs()).max(m.get(j, i).abs());
                }
            }
        }
        vals.push((format!("sf12_energy12_{}", r.method.short()), s12.energy));
    }
    rep.add(
        "SF-12a",
        "the first version computed here (sixteen components, the pairing of chi'_SH switched off) equals the first version \
         computed on the twelve components of Psi_LL and chi_SH alone: relative difference of the energies and largest \
         difference of the local matrices, five methods",
        d12,
        1e-10,
    );
    rep.add("SF-12b", "in the first version the rows and columns of chi'_SH of the local matrices and self-energies stay empty", d12b, 1e-12);
    // SF-13: the other stationary states of V^(1)_SH are unstable towards the Higgs superfluid
    let mut ok13 = true;
    let mut det13 = vec![];
    for r in results.iter().filter(|r| r.pot_name != "V0_SH") {
        for (i, (s, _)) in r.states.iter().enumerate().skip(1) {
            match r.lam_states[i] {
                Some((lp, ln)) => {
                    ok13 &= lp > 1.0 + 1e-6;
                    det13.push(format!("{} {} {}: {:.6} (normal self-energies {:.6})", r.pot_name, mname(&r.method), s.kind, lp, ln));
                }
                None => ok13 = false,
            }
        }
    }
    rep.add_bool(
        "SF-13",
        "V^(1)_SH, both versions: every stationary state other than the ground state is unstable towards the Higgs \
         superfluid: the largest eigenvalue of its static response in the pairs of Psi_LL with the singlets is above 1 (in \
         parentheses the largest eigenvalue in the normal self-energies of the fields)",
        ok13,
        &det13.join("; "),
    );

    // ------------------------------------------------------------------ data files
    let mut eff = vec![];
    for r in &results {
        let mut row = vec![
            r.pot_name.clone(),
            mname(&r.method),
            f(r.lam_higgs),
            r.mult_higgs.to_string(),
            f(r.lam_pseudo),
            f(r.smat[0][0]),
            f(r.smat[0][1]),
            f(r.smat[0][2]),
            f(r.smat[1][0]),
            f(r.smat[2][0]),
            f(r.lam_s),
            f(r.lam_trip),
        ];
        for (_, lam) in &r.lam_blocks {
            row.push(f(*lam));
        }
        eff.push(row);
    }
    write_csv(
        &data.join("sh_sf_normal_vacuum.csv"),
        &[
            "potential",
            "method",
            "lam_higgs",
            "mult_higgs",
            "lam_pseudoscalar",
            "S_PsiPsi",
            "S_Psichi",
            "S_Psichi2",
            "S_chiPsi",
            "S_chi2Psi",
            "lam_scalar",
            "lam_triplet",
            "lam_max_pairs_Psi_singlets",
            "lam_max_pairs_Psi_Psi",
            "lam_max_pairs_singlets",
            "lam_max_normal",
            "lam_max_mixing_singlets",
            "lam_max_mixing_Psi_singlets",
        ],
        &eff,
    );
    let mut st = vec![];
    for r in &results {
        for (i, (s, starts)) in r.states.iter().enumerate() {
            let d = &s.desc;
            let (s1, s2) = d.singular();
            st.push(vec![
                r.pot_name.clone(),
                mname(&r.method),
                i.to_string(),
                s.kind.clone(),
                starts.join(" / "),
                format!("{:.12e}", s.sol.energy),
                f(s1),
                f(s2),
                format!("{:.1e}", d.p_rest),
                format!("{:.1e}", d.p_psipsi),
                format!("{:.1e}", d.p_ss),
                format!("{:.1e}", d.n_psis),
                format!("{:.1e}", d.n_chichi2),
                format!("{:.1e}", d.n_updown),
                f(d.mass[0]),
                f(d.mass[1]),
                f(d.mass[2]),
                f(d.mass[3]),
                format!("{:.6e}", d.ns[0]),
                format!("{:.6e}", d.ns[1]),
                format!("{:.6e}", d.ns[2]),
                format!("{:.6e}", d.ns[3]),
                format!("{:.6e}", d.n[0]),
                format!("{:.6e}", d.n[1]),
                format!("{:.6e}", d.n[2]),
                format!("{:.6e}", d.n[3]),
                s.sol.iterations.to_string(),
                format!("{:.1e}", s.sol.residual),
                r.lam_states[i].map_or("-".to_string(), |x| f(x.0)),
                r.lam_states[i].map_or("-".to_string(), |x| f(x.1)),
            ]);
        }
    }
    write_csv(
        &data.join("sh_sf_states.csv"),
        &[
            "potential", "method", "rank", "kind", "starts", "energy", "sing1", "sing2", "P_rest", "P_PsiPsi", "P_singlets",
            "N_Psi_singlets", "N_chichi2", "N_updown", "M_up", "M_down", "M_chi", "M_chi2", "nS_up", "nS_down", "nS_chi",
            "nS_chi2", "n_up", "n_down", "n_chi", "n_chi2", "iterations", "residual", "lam_pairs_ds", "lam_normal",
        ],
        &st,
    );
    let mut gr = vec![];
    for r in w1.iter().chain(w2.iter()) {
        let s = match &r.ground {
            Some(g) => g,
            None => continue,
        };
        let d = &s.desc;
        let (s1, s2) = d.singular();
        let (n1, dev1, maxa, maxb) = r.resp.unwrap_or((0, f64::NAN, f64::NAN, f64::NAN));
        let (g, qg) = r.gap.unwrap_or((f64::NAN, f64::NAN));
        let ver = if r.pot_name == "V1_SH" { "second" } else { "first" };
        gr.push(vec![
            ver.to_string(),
            mname(&r.method),
            s.kind.clone(),
            format!("{:.12e}", s.sol.energy),
            f(s1),
            f(s2),
            f(d.mass[0]),
            f(d.mass[1]),
            f(d.mass[2]),
            f(d.mass[3]),
            format!("{:.6e}", d.ns[0] - d.ns[1]),
            format!("{:.6e}", d.ns[2] - d.ns[3]),
            f(g),
            f(qg),
            n1.to_string(),
            format!("{:.1e}", dev1),
            f(maxa),
            f(maxb),
            r.lam_one_doublet.map_or("-".to_string(), f),
            r.e_one_doublet.map_or("-".to_string(), |e| format!("{:.12e}", e)),
        ]);
        let tag = format!("{}_{}", if ver == "second" { "v2" } else { "v1" }, r.method.short());
        vals.push((format!("ground_energy_{}", tag), s.sol.energy));
        vals.push((format!("ground_sing1_{}", tag), s1));
        vals.push((format!("ground_sing2_{}", tag), s2));
        vals.push((format!("ground_Mu_{}", tag), d.mass[0]));
        vals.push((format!("ground_Md_{}", tag), d.mass[1]));
        vals.push((format!("ground_Mchi_{}", tag), d.mass[2]));
        vals.push((format!("ground_Mchi2_{}", tag), d.mass[3]));
        vals.push((format!("ground_dnS_{}", tag), d.ns[0] - d.ns[1]));
        vals.push((format!("ground_gap_{}", tag), g));
        if let Some(l) = r.lam_one_doublet {
            vals.push((format!("one_doublet_lam_{}", tag), l));
        }
    }
    write_csv(
        &data.join("sh_sf_ground.csv"),
        &[
            "version", "method", "kind", "energy", "sing1", "sing2", "M_up", "M_down", "M_chi", "M_chi2", "nS_up_minus_nS_down",
            "nS_chi_minus_nS_chi2", "gap", "q_gap", "eigenvalues_1", "dev_from_1", "largest_other_R0", "largest_R2",
            "lam_one_doublet", "energy_one_doublet",
        ],
        &gr,
    );
    let vrows: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), format!("{:.12e}", v)]).collect();
    write_csv(&data.join("sh_sf_values.csv"), &["name", "value"], &vrows);
    rep.note(&format!(
        "run time {:.1} s on {} threads; the fifteen jobs (five methods, three potentials) ran in parallel, the longest {:.1} s",
        t0.elapsed().as_secs_f64(),
        nthreads,
        results.iter().fold(0.0f64, |m, r| m.max(r.seconds))
    ));
    rep.write(&data.join("hck_sh_superfluid_checks.txt"));
    let (np, nn) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, nn, t0.elapsed().as_secs_f64());
    std::process::exit(if np == nn { 0 } else { 1 });
}
