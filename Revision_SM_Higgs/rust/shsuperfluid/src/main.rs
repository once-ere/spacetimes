//! sh_superfluid -- stage 3 of the revision of the hypotheses (folder Revision_SM_Higgs of PROJECT spacetimes): the
//! mean-field superfluid of the two fields Psi_LL (the isodoublet, eight components) and chi_SH (the singlet, four
//! components) by the methods HS, KS and CJT.
//!
//! The model.  The Hamiltonian density is the free Dirac Hamiltonian of the two fields (masses m_LL = m^chi_SH = 0.2 in
//! units of the cutoff Lambda_LL = 1, decision D22), the scalar and the vector contact terms of the author's ansatz on
//! Psi_LL (couplings G^S_SF, G^V_SF), and the pairing term (4.2) of the doublet-singlet channel,
//! -G_SH sum_s (Pi^s_SH)^dag Pi^s_SH with Pi^s_SH = Psi_LL^T V_s chi_SH, V_s = (1/2) C gamma^ch (x) e_s.  The methods keep
//! the Wick contractions of the contact terms with the weights of the earlier work (HS with the parameters (1, 1),
//! (0, 1), (0, 0); KS with the exchange in the local-density approximation of the number and scalar densities, those of
//! Psi_LL as matrices in the isospin space, decision D28; CJT with all contractions).  The engine is sfcore::uniform_n,
//! the generalization of the engine of the earlier work to a field of several Dirac fields.
//!
//! The potentials.  V^(0)_SH: the author's ansatz (G^S_SF = -8, G^V_SF = 0.8) without a pairing term.  V^(1)_SH: the same
//! with the pairing term, G_SH = 100.8 (decision D27: the coupling 25.2 of the pairing term of the earlier potential
//! V^(2)_SF transferred to the doublet-singlet channel; the factor 4 comes from the normalization of V_s, so that the
//! pair channel has the same supercriticality as there).
//!
//! The checks (report data/hck_sh_superfluid_checks.txt):
//!   SF-1   the engine reproduces the engine of the earlier work (module uniform) on the earlier V^(2)_SF with the singlet
//!          field decoupled, in all five methods;
//!   SF-2   the static response of the normal vacuum in the pairs of Psi_LL and chi_SH: the largest eigenvalue is
//!          G_SH / G^crit_SH with G^crit_SH = 4 pi^2 / I1(m), fourfold (the two components of the doublet, real and
//!          imaginary parts), and the pseudoscalar channel has no interaction;
//!   SF-3   the scalar channel of the normal vacuum: the 2 x 2 response of the Dirac masses of the two fields agrees
//!          with the effective couplings ((7/8) G^S - (1/2) G^V on Psi_LL, the cross coupling -G_SH/16 of the exchange
//!          contraction of the pairing term) and the scalar susceptibilities; it is stable;
//!   SF-4   the other channels of the normal vacuum (pairs of Psi_LL, of chi_SH, the mixing Psi^dag chi): the triplet
//!          scalar pair coupling agrees with -(1/4) G^S - G^V (KS, CJT) and its HS forms, and every eigenvalue is
//!          below 1: only the Higgs channel can condense;
//!   SF-5   V^(0)_SH: every start of every method ends in the normal vacuum;
//!   SF-6   V^(1)_SH: the ground state of every method is the Higgs superfluid, with no pair of two particles of one
//!          field and no mixing of the two fields;
//!   SF-7   the starts along Phi^u, Phi^d and a generic direction of the doublet reach states of equal energy and
//!          equal length of the doublet (the symmetry SU(2)_L x U(1)_Y);
//!   SF-8   the generators of SU(2)_L x U(1)_Y and of the relative number that leave each Higgs ground state unchanged:
//!          dimension 2, the relative number and one electroweak generator with the spectrum of Q^SM;
//!   SF-9   the static response of each Higgs ground state: three eigenvalues 1 (the Goldstone modes of T^1, T^2 and
//!          T^3 - Y), all others below 1, also in the sector of the pairs of one field and of the mixing;
//!   SF-10  the grid: the ground states of HS (1, 1) and CJT on 24 and 48 radial nodes;
//!   SF-11  the lowest quasi-particle of every Higgs ground state is the unpaired up component of Psi_LL (electric
//!          charge 3/4), with the energy |M_u| at zero momentum (a: the energy; b: the momentum of the minimum).
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_superfluid
//! Output in data/: hck_sh_superfluid_checks.txt and the data files sh_sf_*.csv.  Run time a few minutes (threads).
mod model;

use model::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{beta, d8, k_scalar};
use sfcore::methods::{grid, terms, Method, Potential};
use sfcore::quad::gauss_legendre;
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::uniform::Model;
use sfcore::uniform_n::{n_dirs, p_dirs, response_eigs, Dir, ModelN, SolutionN};
use std::f64::consts::PI;
use std::time::Instant;

/// the potentials of the revised hypotheses
const V0_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 0.0 };
const V1_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8 };
/// the self-consistency: mixing, tolerance of the residual of the self-energies, maximum of iterations
const MIX: f64 = 0.6;
const TOL: f64 = 1e-12;
const MAXIT: usize = 40000;
/// the step of the central differences of the static response
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

/// the starts of the survey: name, N0, P0
fn starts() -> Vec<(&'static str, CMat, CMat)> {
    let bb = CMat::block_diag(&d8(&beta()), &beta());
    let kd = k_sh(1).dagger().scale_r(0.3);
    let ku = k_sh(0).dagger().scale_r(0.3);
    // a generic direction of the doublet: (0.6, 0.8 i), length 1
    let kg = k_sh(0).dagger().scale(c(0.6 * 0.3, 0.0)).add(&k_sh(1).dagger().scale(c(0.0, 0.8 * 0.3)));
    let ktrip = emb_psi(&k_scalar(3).dagger().scale_r(0.3));
    // the scalar pair of two chi_SH particles, chi^T C gamma^ch chi
    let kchi = emb_chi(&sfcore::dirac::cmat_c().mul(&sfcore::dirac::gamma_ch()).dagger().scale_r(0.3));
    // a mixing of the two fields, Psi_u^dag beta chi + h.c. (a charged particle-antiparticle mean field)
    let mut mix = CMat::zeros(N);
    let b = beta();
    for dd in 0..4 {
        for ee in 0..4 {
            let v = b.get(dd, ee).scale(0.3);
            mix.set(2 * dd, NP + ee, v);
            mix.set(NP + ee, 2 * dd, v.conj());
        }
    }
    vec![
        ("normal", CMat::zeros(N), CMat::zeros(N)),
        ("Higgs down", CMat::zeros(N), kd.clone()),
        ("Higgs up", CMat::zeros(N), ku),
        ("Higgs generic", CMat::zeros(N), kg),
        ("pair of Psi", CMat::zeros(N), ktrip),
        ("pair of chi", CMat::zeros(N), kchi),
        ("mass up", bb.scale_r(0.5), CMat::zeros(N)),
        ("mass down", bb.scale_r(-1.0), CMat::zeros(N)),
        ("mass down and Higgs", bb.scale_r(-1.0), kd),
        ("mixing", mix, CMat::zeros(N)),
        ("isospin polarized", tau3_beta(), CMat::zeros(N)),
    ]
}

/// the kind of a stationary state
fn kind_of(d: &Describe) -> String {
    let dn = (d.d[0].norm_sqr() + d.d[1].norm_sqr()).sqrt();
    let charged = d.p_psipsi.max(d.p_chichi).max(d.n_psichi);
    let massive = (d.mass[0] - M_LL).abs().max((d.mass[1] - M_LL).abs()).max((d.mass[2] - M_CHI).abs()) > 1e-6;
    if dn > 1e-6 && d.p_rest < 1e-8 && charged < 1e-8 {
        "Higgs superfluid".into()
    } else if dn > 1e-6 {
        "Higgs superfluid with a charged admixture".into()
    } else if d.p_psipsi > 1e-6 {
        "superfluid of pairs of Psi_LL".into()
    } else if d.p_chichi > 1e-6 {
        "superfluid of pairs of chi_SH".into()
    } else if d.n_psichi > 1e-6 {
        "mixing of the two fields".into()
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
        let sol = mo.solve(&n0, &p0, MIX, TOL, MAXIT);
        let desc = describe(&sol.n_self, &sol.p_self, &sol.drho);
        out.push(State { start: name.into(), kind: kind_of(&desc), sol, desc });
    }
    out.sort_by(|a, b| a.sol.energy.partial_cmp(&b.sol.energy).unwrap());
    out
}

/// the distinct states (same kind and energy within 1e-11)
fn distinct(states: &[State]) -> Vec<(State, Vec<String>)> {
    let mut out: Vec<(State, Vec<String>)> = vec![];
    for s in states {
        if let Some(e) = out.iter_mut().find(|e| e.0.kind == s.kind && (e.0.sol.energy - s.sol.energy).abs() < 1e-11) {
            e.1.push(s.start.clone());
        } else {
            out.push((s.clone(), vec![s.start.clone()]));
        }
    }
    out
}

/// the 2 x 2 response of the Dirac masses (M_Psi, M_chi) of the normal vacuum: a change of the mass of field r by the
/// self-energy h beta_r changes the local matrices; the self-energies that the contact terms produce from that change,
/// projected onto beta_Psi and beta_chi, are the column r
fn scalar_response(mo: &ModelN, h: f64) -> [[f64; 2]; 2] {
    let bpsi = emb_psi(&d8(&beta()));
    let bchi = emb_chi(&beta());
    let bs = [bpsi, bchi];
    let z = CMat::zeros(N);
    let mut s = [[0.0; 2]; 2];
    for r in 0..2 {
        let (_, rp, kp) = mo.energy_of(&bs[r].scale_r(h), &z);
        let (_, rm, km) = mo.energy_of(&bs[r].scale_r(-h), &z);
        let (n2, _) = mo.self_energy_lin(&rp.sub(&rm).scale_r(0.5 / h), &kp.sub(&km).scale_r(0.5 / h));
        for k in 0..2 {
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

/// the directions of the five blocks of the self-energies
fn dirs_blocks() -> Vec<(&'static str, Vec<Dir>)> {
    let (ps, cs) = (psi_indices(), chi_indices());
    let mut ndiag = n_dirs(&ps, &ps);
    ndiag.extend(n_dirs(&cs, &cs));
    vec![
        ("pairs Psi chi", p_dirs(&ps, &cs)),
        ("pairs Psi Psi", p_dirs(&ps, &ps)),
        ("pairs chi chi", p_dirs(&cs, &cs)),
        ("normal, Psi and chi", ndiag),
        ("normal, mixing Psi chi", n_dirs(&ps, &cs)),
    ]
}

/// the stabilizer of a state (N, P) in the real span of the generators T^1, T^2, T^3, Y, R: the real coefficients c
/// with [X_c, N] = 0 and X_c P + P X_c^T = 0 (X_c = sum c_a X_a); returns an orthonormal basis of the null space
fn stabilizer(nself: &CMat, pself: &CMat) -> Vec<Vec<f64>> {
    let gens = generators();
    let na = gens.len();
    let mut cols: Vec<Vec<f64>> = vec![];
    for x in &gens {
        let a = x.comm(nself);
        let b = x.mul(pself).add(&pself.mul(&x.transpose()));
        let mut v = vec![];
        for z in a.a.iter().chain(b.a.iter()) {
            v.push(z.re);
            v.push(z.im);
        }
        cols.push(v);
    }
    // A^T A (na x na), its null space
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

/// the generators T^1, T^2, T^3, Y and the relative number R on the twelve components
fn generators() -> Vec<CMat> {
    use sfcore::dirac::t_gen;
    let mut g: Vec<CMat> = (1..=3).map(|a| emb_psi(&t_gen(a))).collect();
    g.push(CMat::identity(N).scale_r(0.25));
    g.push(CMat::block_diag(&CMat::identity(NP), &CMat::identity(NX).scale_r(-1.0)));
    g
}

/// the results of one method for one potential
struct MethodResult {
    method: Method,
    pot_name: String,
    // normal vacuum
    lam_higgs: f64,
    mult_higgs: usize,
    lam_pseudo: f64,
    smat: [[f64; 2]; 2],
    lam_s: f64,
    lam_trip: f64,
    lam_blocks: Vec<(String, f64, f64)>,
    // states
    states: Vec<(State, Vec<String>)>,
    higgs_starts: Vec<(String, f64, f64)>,
    /// the Higgs ground state reached from the start along Phi^d (the reference state of the tables), when the ground
    /// state is a Higgs superfluid and this state has its energy
    ground: Option<State>,
    // the ground state, if it is a Higgs superfluid: response, stabilizer, gap
    resp: Option<(usize, f64, f64, f64)>,
    stab: Option<((usize, usize), Vec<f64>)>,
    gap: Option<(f64, f64)>,
    sym_dev: f64,
    chi_min: f64,
    seconds: f64,
}

fn run_method(method: Method, pname: &str, pot: PotSh) -> MethodResult {
    let t0 = Instant::now();
    let mo = model(&method, &pot, NR);
    let z = CMat::zeros(N);
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
        if name == "pairs Psi chi" {
            lam_higgs = re[0];
            mult_higgs = re.iter().filter(|x| (*x - re[0]).abs() < 1e-8 * (1.0 + re[0].abs())).count();
        }
        lam_blocks.push((name.to_string(), re[0], 0.0));
    }
    let lam_pseudo = pair_channel_response(&mo, &{
        // the pseudoscalar doublet-singlet pair matrix: ((0, W), (-W^T, 0)) with W = (1/2) C (x) e_down
        let w = sfcore::dirac::cmat_c().kron(&e_iso(1)).scale_r(0.5);
        let mut k = CMat::zeros(N);
        k.put(0, NP, &w);
        k.put(NP, 0, &w.transpose().scale_r(-1.0));
        k
    }, H_RESP);
    let smat = scalar_response(&mo, H_RESP);
    let tr = smat[0][0] + smat[1][1];
    let det = smat[0][0] * smat[1][1] - smat[0][1] * smat[1][0];
    let lam_s = 0.5 * (tr + (tr * tr - 4.0 * det).max(0.0).sqrt());
    let lam_trip = pair_channel_response(&mo, &emb_psi(&k_scalar(3)), H_RESP);
    // the survey
    let all = survey(&mo);
    let states = distinct(&all);
    let higgs_starts: Vec<(String, f64, f64)> = all
        .iter()
        .filter(|s| s.start.starts_with("Higgs"))
        .map(|s| (s.start.clone(), s.sol.energy, (s.desc.d[0].norm_sqr() + s.desc.d[1].norm_sqr()).sqrt()))
        .collect();
    let mut resp = None;
    let mut stab = None;
    let mut gap = None;
    let mut ground = None;
    let g = &states[0].0;
    let gd = all.iter().find(|s| s.start == "Higgs down").unwrap();
    if g.kind == "Higgs superfluid" && gd.kind == g.kind && (gd.sol.energy - g.sol.energy).abs() < 1e-11 * g.sol.energy.abs() {
        // the ground state reached from the start along Phi^d (the states of the three Higgs starts are equivalent)
        ground = Some(gd.clone());
        let (ps, cs) = (psi_indices(), chi_indices());
        let mut block_a = p_dirs(&ps, &cs);
        block_a.extend(n_dirs(&ps, &ps));
        block_a.extend(n_dirs(&cs, &cs));
        let mut block_b = p_dirs(&ps, &ps);
        block_b.extend(p_dirs(&cs, &cs));
        block_b.extend(n_dirs(&ps, &cs));
        let (ra, a1, a2, a3) = response_eigs(&mo, &gd.sol.n_self, &gd.sol.p_self, &block_a, H_RESP);
        let (rb, b1, b2, b3) = response_eigs(&mo, &gd.sol.n_self, &gd.sol.p_self, &block_b, H_RESP);
        sym_dev = sym_dev.max(a1).max(a2).max(b1).max(b2);
        chi_min = chi_min.min(a3).min(b3);
        let n_one = ra.iter().filter(|x| (*x - 1.0).abs() < 1e-6).count();
        let max_other_a = ra.iter().filter(|x| (*x - 1.0).abs() >= 1e-6).fold(f64::NEG_INFINITY, |s, x| s.max(*x));
        let dev_one = ra.iter().filter(|x| (*x - 1.0).abs() < 1e-6).fold(0.0f64, |s, x| s.max((x - 1.0).abs()));
        resp = Some((n_one, dev_one, max_other_a, rb[0]));
        // the stabilizer of each Higgs state
        let mut dims = vec![];
        let mut qdev: Vec<f64> = vec![];
        for s in all.iter().filter(|s| s.start.starts_with("Higgs")) {
            let st = stabilizer(&s.sol.n_self, &s.sol.p_self);
            dims.push(st.len());
            // the electroweak generator of the stabilizer: the combination with no relative number; its spectrum,
            // normalized so that its trace is that of Q (3), against the spectrum of Q^SM
            let gens = generators();
            let mut x = CMat::zeros(N);
            let mut ok = false;
            if st.len() == 2 {
                let (a, b) = (&st[0], &st[1]);
                // the combination a b[4] - b a[4] has no R component
                let w: Vec<f64> = (0..5).map(|i| a[i] * b[4] - b[i] * a[4]).collect();
                for i in 0..4 {
                    x = x.add(&gens[i].scale_r(w[i]));
                }
                let tr = x.trace().re;
                if tr.abs() > 1e-12 {
                    x = x.scale_r(3.0 / tr);
                    ok = true;
                }
            }
            if ok {
                let ev = eigh(&x).0;
                let mut want = vec![-0.25; 4];
                want.extend(vec![0.25; 4]);
                want.extend(vec![0.75; 4]);
                qdev.push(ev.iter().zip(&want).fold(0.0f64, |m, (p, q)| m.max((p - q).abs())));
            } else {
                qdev.push(f64::INFINITY);
            }
        }
        stab = Some(((dims.iter().cloned().min().unwrap_or(0), dims.iter().cloned().max().unwrap_or(0)), qdev));
        gap = Some(mo.gap(&gd.sol.n_self, &gd.sol.p_self));
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
        higgs_starts,
        ground,
        resp,
        stab,
        gap,
        sym_dev,
        chi_min,
        seconds: t0.elapsed().as_secs_f64(),
    }
}

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new("sh_superfluid: checks of the mean-field superfluid of the revised hypotheses (stage 3, Revision_SM_Higgs, PROJECT spacetimes)");
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
        // the singlet block stays empty
        d1k = d1k.max(sn.kap.sub_block(8, 8, 4, 4).max_abs()).max(sn.drho.sub_block(8, 8, 4, 4).max_abs());
        vals.push((format!("sf1_energy_{}", m.short()), sn.energy));
    }
    rep.add("SF-1a", "the energy of the earlier engine (module uniform) and of the new engine (module uniform_n, the singlet field decoupled) on the earlier V^(2) from the polar start, five methods", d1, 1e-13);
    rep.add("SF-1b", "the local matrices of the two engines agree, and the singlet block of the new engine stays empty", d1k, 1e-12);
    println!("SF-1 done ({:.0} s)", t0.elapsed().as_secs_f64());

    // ------------------------------------------------------------------ the methods, in parallel
    let mut jobs: Vec<(Method, &str, PotSh)> = vec![];
    for m in methods() {
        jobs.push((m, "V0_SH", V0_SH));
        jobs.push((m, "V1_SH", V1_SH));
    }
    let results: Vec<MethodResult> = std::thread::scope(|sc| {
        let hs: Vec<_> = jobs.iter().map(|(m, n, p)| {
            let (m, n, p) = (*m, *n, *p);
            sc.spawn(move || run_method(m, n, p))
        }).collect();
        hs.into_iter().map(|h| h.join().unwrap()).collect()
    });
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
    let mut d2 = 0.0f64;
    let mut d2m = true;
    let mut d2p = 0.0f64;
    let mut d3 = 0.0f64;
    let mut ok3 = true;
    let mut d4 = 0.0f64;
    let mut ok4 = true;
    let mut ok4_detail = vec![];
    for r in &results {
        let want = if r.pot_name == "V1_SH" { V1_SH.gsh / gcrit } else { 0.0 };
        d2 = d2.max((r.lam_higgs - want).abs());
        if want > 0.0 {
            d2m &= r.mult_higgs == 4;
        }
        d2p = d2p.max(r.lam_pseudo.abs());
        let pot = if r.pot_name == "V1_SH" { V1_SH } else { V0_SH };
        let (gpp, gpx) = match r.method {
            Method::Hs { s_s, .. } => (s_s * pot.gs, 0.0),
            _ => (0.875 * pot.gs - 0.5 * pot.gv, -pot.gsh / 16.0),
        };
        let sw = [[-gpp * chi8, -gpx * chi4], [-gpx * chi8, 0.0]];
        for a in 0..2 {
            for b in 0..2 {
                d3 = d3.max((r.smat[a][b] - sw[a][b]).abs());
            }
        }
        ok3 &= r.lam_s < 1.0;
        let gtrip = match r.method {
            Method::Hs { s_s, s_v } => -0.25 * (1.0 - s_s) * pot.gs - (1.0 - s_v) * pot.gv,
            _ => -0.25 * pot.gs - pot.gv,
        };
        d4 = d4.max((r.lam_trip - gtrip / gcrit_ll).abs());
        for (name, lam, _) in &r.lam_blocks {
            if name != "pairs Psi chi" {
                ok4 &= *lam < 1.0;
                ok4_detail.push(format!("{} {} {}: {:.4}", r.pot_name, mname(&r.method), name, lam));
            }
        }
    }
    rep.add("SF-2a", &format!("normal vacuum, pairs of Psi_LL and chi_SH: the largest eigenvalue of the static response is G_SH / G^crit_SH with G^crit_SH = 4 pi^2 / I1(m) = {:.10} (V^(1)_SH: {:.10}; V^(0)_SH: 0), five methods", gcrit, V1_SH.gsh / gcrit), d2, 1e-9);
    rep.add_bool("SF-2b", "the eigenvalue G_SH / G^crit_SH is fourfold (the two components of the doublet, each with its real and imaginary part)", d2m, "V^(1)_SH, five methods");
    rep.add("SF-2c", "the pseudoscalar doublet-singlet pairs (matrix C (x) e_s) have no interaction in the normal vacuum (response 0)", d2p, 1e-10);
    rep.add("SF-3a", &format!("normal vacuum, scalar channel: the 2 x 2 response of the masses of Psi_LL and chi_SH agrees with the effective couplings (7/8 G^S - 1/2 G^V on Psi_LL in KS and CJT, s_S G^S in HS; the cross coupling -G_SH/16 in KS and CJT) and the susceptibilities chi_S = {:.10} (Psi_LL), {:.10} (chi_SH)", chi8, chi4), d3, 1e-9);
    rep.add_bool("SF-3b", "the scalar channel of the normal vacuum is stable (largest eigenvalue of the 2 x 2 response below 1) for both potentials and all methods", ok3, &results.iter().map(|r| format!("{} {}: {:.4}", r.pot_name, mname(&r.method), r.lam_s)).collect::<Vec<_>>().join("; "));
    rep.add("SF-4a", "normal vacuum, scalar triplet pairs of Psi_LL (K^3): the response agrees with g_P / G^crit_LL, g_P = -(1/4) G^S - G^V (KS, CJT) or -(1/4)(1 - s_S) G^S - (1 - s_V) G^V (HS)", d4, 1e-9);
    rep.add_bool("SF-4b", "normal vacuum: every eigenvalue of the static response in the pairs of Psi_LL, the pairs of chi_SH, the normal self-energies of each field and the mixing of the two fields is below 1, for both potentials and all methods: only the doublet-singlet pairs can condense", ok4, &ok4_detail.join("; "));

    let sdev = results.iter().fold(0.0f64, |m, r| m.max(r.sym_dev));
    let cmin = results.iter().fold(f64::INFINITY, |m, r| m.min(r.chi_min));
    rep.add_bool("SF-4c", "the eigenvalues of the static response are those of a symmetric matrix: the response of the densities to the self-energies and the kernel of the contact terms are symmetric in an orthonormal basis (largest asymmetry below 1e-7), and the response of the densities is negative semidefinite (smallest eigenvalue of its negative above -1e-9)", sdev < 1e-7 && cmin > -1e-9, &format!("largest asymmetry {:.1e}; smallest eigenvalue {:.1e}", sdev, cmin));

    // ------------------------------------------------------------------ SF-5 to SF-11: the states
    let ok5 = results.iter().filter(|r| r.pot_name == "V0_SH").all(|r| r.states.len() == 1 && r.states[0].0.kind == "normal vacuum");
    rep.add_bool("SF-5", "V^(0)_SH: every start of every method ends in the normal vacuum", ok5, &results.iter().filter(|r| r.pot_name == "V0_SH").map(|r| format!("{}: {} state(s)", mname(&r.method), r.states.len())).collect::<Vec<_>>().join("; "));
    let v1: Vec<&MethodResult> = results.iter().filter(|r| r.pot_name == "V1_SH").collect();
    let ok6 = v1.iter().all(|r| r.states[0].0.kind == "Higgs superfluid" && r.ground.is_some());
    rep.add_bool("SF-6", "V^(1)_SH: the ground state of every method is the Higgs superfluid (no pair of two particles of one field, no mixing of the two fields), the stationary state of lowest energy among those reached from the eleven starts", ok6, &v1.iter().map(|r| format!("{}: {} (E = {:.6e})", mname(&r.method), r.states[0].0.kind, r.states[0].0.sol.energy)).collect::<Vec<_>>().join("; "));
    let mut d7 = 0.0f64;
    for r in &v1 {
        let e0 = r.higgs_starts[0].1;
        let l0 = r.higgs_starts[0].2;
        for (_, e, l) in &r.higgs_starts {
            d7 = d7.max((e - e0).abs() / e0.abs()).max((l - l0).abs() / l0);
        }
    }
    rep.add("SF-7", "the starts along Phi^d, Phi^u and the generic direction (0.6, 0.8 i) reach states of equal energy and equal length of the doublet (relative deviation; the symmetry SU(2)_L x U(1)_Y)", d7, 1e-9);
    let ok8 = v1.iter().all(|r| match &r.stab {
        Some((dims, qdev)) => *dims == (2, 2) && qdev.iter().all(|x| *x < 1e-8),
        None => false,
    });
    rep.add_bool("SF-8", "the generators of SU(2)_L x U(1)_Y and of the relative number that leave each Higgs ground state unchanged form a space of dimension 2: the relative number and one electroweak generator whose spectrum is that of Q^SM (3/4, -1/4, 1/4, each fourfold); this for the three Higgs starts of every method", ok8, &v1.iter().map(|r| format!("{}: {:?}", mname(&r.method), r.stab.as_ref().map(|s| s.0))).collect::<Vec<_>>().join("; "));
    let ok9 = v1.iter().all(|r| match r.resp {
        Some((n1, dev1, maxa, maxb)) => n1 == 3 && dev1 < 1e-6 && maxa < 1.0 && maxb < 1.0,
        None => false,
    });
    rep.add_bool("SF-9", "the static response of each Higgs ground state: in the sector of relative number 0 (doublet-singlet pairs and normal self-energies of each field) exactly three eigenvalues equal 1 (the Goldstone modes of the three broken generators) and all others are below 1; in the sector of relative number +-2 (pairs of one field, mixing) all are below 1: the ground state is stable and no charged condensate grows", ok9, &v1.iter().map(|r| match r.resp {
        Some((n1, dev1, maxa, maxb)) => format!("{}: {} eigenvalues 1 (to {:.1e}), largest other {:.5}, largest of the charged sector {:.5}", mname(&r.method), n1, dev1, maxa, maxb),
        None => format!("{}: no Higgs ground state", mname(&r.method)),
    }).collect::<Vec<_>>().join("; "));

    // SF-10: the grid
    let mut d10 = 0.0f64;
    for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Cjt] {
        let a = model(&m, &V1_SH, NR).solve(&CMat::zeros(N), &k_sh(1).dagger().scale_r(0.3), MIX, TOL, MAXIT);
        let b = model(&m, &V1_SH, 2 * NR).solve(&CMat::zeros(N), &k_sh(1).dagger().scale_r(0.3), MIX, TOL, MAXIT);
        let (da, db) = (describe(&a.n_self, &a.p_self, &a.drho), describe(&b.n_self, &b.p_self, &b.drho));
        d10 = d10.max((a.energy - b.energy).abs() / b.energy.abs()).max((da.d[1].abs() - db.d[1].abs()).abs() / db.d[1].abs());
        vals.push((format!("sf10_energy_48_{}", m.short()), b.energy));
    }
    rep.add("SF-10", &format!("the Higgs ground states of HS (1, 1) and CJT on {} and {} radial nodes: relative change of the energy and of |D_d|", NR, 2 * NR), d10, 1e-6);

    // SF-11: the lowest quasi-particle
    let mut d11 = 0.0f64;
    let mut q11 = 0.0f64;
    for r in &v1 {
        match (r.gap, &r.ground) {
            (Some((g, q)), Some(gs)) => {
                d11 = d11.max((g - gs.desc.mass[0].abs()).abs());
                q11 = q11.max(q);
            }
            _ => d11 = f64::INFINITY,
        }
    }
    rep.add("SF-11a", "the lowest quasi-particle of every Higgs ground state is the unpaired up component of Psi_LL (electric charge 3/4): the gap equals its Dirac mass |M_u|", d11, 1e-9);
    rep.add("SF-11b", "the minimum of the quasi-particle energy lies at zero momentum (the momentum found by the search, whose resolution near a quadratic minimum is about 1e-8)", q11, 1e-6);

    // ------------------------------------------------------------------ data files
    let mut eff = vec![];
    for r in &results {
        let mut row = vec![r.pot_name.clone(), mname(&r.method), f(r.lam_higgs), r.mult_higgs.to_string(), f(r.lam_pseudo),
            f(r.smat[0][0]), f(r.smat[0][1]), f(r.smat[1][0]), f(r.smat[1][1]), f(r.lam_s), f(r.lam_trip)];
        for (_, lam, _) in &r.lam_blocks {
            row.push(f(*lam));
        }
        eff.push(row);
    }
    write_csv(&data.join("sh_sf_normal_vacuum.csv"), &["potential", "method", "lam_higgs", "mult_higgs", "lam_pseudoscalar",
        "S_PsiPsi", "S_Psichi", "S_chiPsi", "S_chichi", "lam_scalar", "lam_triplet",
        "lam_max_pairs_Psi_chi", "lam_max_pairs_Psi_Psi", "lam_max_pairs_chi_chi", "lam_max_normal", "lam_max_mixing"], &eff);
    let mut st = vec![];
    for r in &results {
        for (i, (s, starts)) in r.states.iter().enumerate() {
            let d = &s.desc;
            st.push(vec![r.pot_name.clone(), mname(&r.method), i.to_string(), s.kind.clone(), starts.join(" / "),
                format!("{:.12e}", s.sol.energy), f((d.d[0].norm_sqr() + d.d[1].norm_sqr()).sqrt()), format!("{:.1e}", d.p_rest),
                format!("{:.1e}", d.p_psipsi), format!("{:.1e}", d.p_chichi), format!("{:.1e}", d.n_psichi), format!("{:.1e}", d.n_updown),
                f(d.mass[0]), f(d.mass[1]), f(d.mass[2]), format!("{:.6e}", d.ns[0]), format!("{:.6e}", d.ns[1]), format!("{:.6e}", d.ns[2]),
                format!("{:.6e}", d.n[0]), format!("{:.6e}", d.n[1]), format!("{:.6e}", d.n[2]), s.sol.iterations.to_string(), format!("{:.1e}", s.sol.residual)]);
        }
    }
    write_csv(&data.join("sh_sf_states.csv"), &["potential", "method", "rank", "kind", "starts", "energy", "abs_D", "P_rest",
        "P_PsiPsi", "P_chichi", "N_Psichi", "N_updown", "M_up", "M_down", "M_chi", "nS_up", "nS_down", "nS_chi", "n_up", "n_down", "n_chi", "iterations", "residual"], &st);
    let mut gr = vec![];
    for r in &v1 {
        let s = match &r.ground {
            Some(g) => g,
            None => continue,
        };
        let d = &s.desc;
        let (n1, dev1, maxa, maxb) = r.resp.unwrap_or((0, f64::NAN, f64::NAN, f64::NAN));
        let (g, qg) = r.gap.unwrap_or((f64::NAN, f64::NAN));
        gr.push(vec![mname(&r.method), s.kind.clone(), format!("{:.12e}", s.sol.energy), f(d.d[1].abs()), f(d.mass[0]), f(d.mass[1]), f(d.mass[2]),
            format!("{:.6e}", d.ns[0] - d.ns[1]), f(g), f(qg), n1.to_string(), format!("{:.1e}", dev1), f(maxa), f(maxb)]);
        vals.push((format!("ground_energy_{}", r.method.short()), s.sol.energy));
        vals.push((format!("ground_absD_{}", r.method.short()), d.d[1].abs()));
        vals.push((format!("ground_Mu_{}", r.method.short()), d.mass[0]));
        vals.push((format!("ground_Md_{}", r.method.short()), d.mass[1]));
        vals.push((format!("ground_Mchi_{}", r.method.short()), d.mass[2]));
        vals.push((format!("ground_dnS_{}", r.method.short()), d.ns[0] - d.ns[1]));
        vals.push((format!("ground_gap_{}", r.method.short()), g));
    }
    write_csv(&data.join("sh_sf_ground.csv"), &["method", "kind", "energy", "abs_D_down", "M_up", "M_down", "M_chi", "nS_up_minus_nS_down",
        "gap", "q_gap", "eigenvalues_1", "dev_from_1", "largest_other_R0", "largest_R2"], &gr);
    let vrows: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), format!("{:.12e}", v)]).collect();
    write_csv(&data.join("sh_sf_values.csv"), &["name", "value"], &vrows);
    rep.note(&format!("run time {:.1} s; the ten jobs (five methods, two potentials) ran in parallel, the longest {:.1} s", t0.elapsed().as_secs_f64(), results.iter().fold(0.0f64, |m, r| m.max(r.seconds))));
    rep.write(&data.join("hck_sh_superfluid_checks.txt"));
    let (np, nn) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, nn, t0.elapsed().as_secs_f64());
    let _ = C64::ZERO;
    std::process::exit(if np == nn { 0 } else { 1 });
}
