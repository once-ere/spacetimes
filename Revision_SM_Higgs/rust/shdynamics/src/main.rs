//! sh_dynamics -- stage 6 of the revision of the hypotheses, part one (folder Revision_SM_Higgs of PROJECT spacetimes):
//! the dynamics in time of the Higgs superfluid of the second version, with the first version as the comparison, by the
//! time-dependent mean field of each of the five methods (HS (1, 1), HS (0, 1), HS (0, 0), KS, CJT), integrated with
//! CVODE (SUNDIALS 7.8.0 in pure Rust, crate sfsundials).
//!
//! THE EQUATIONS.  A uniform quasi-free state of the sixteen components is given, at every node q of the momentum grid
//! of stage 3 (24 radial nodes times 6 directions: 144 nodes), by its N = 16 occupied quasi-particle vectors, the
//! columns of the 32 x 16 matrix V(q) (at t = 0 the eigenvectors of negative energy of a BdG matrix).  Its generalized
//! density is R(q) = V(q) V(q)^dag, with the blocks C^n(q) = R_11 and C^p(q) = -R_12; the local matrices drho and kap,
//! the energy per volume and the self-energies N and P of the method follow from them as in the static programs of
//! stages 3 and 4 (sfcore::uniform_n::ModelN::local and ::interaction).  The time-dependent mean field is
//!     i dV(q)/dt = H(q; N(t), P(t)) V(q),                                                            (D1)
//! with H(q; N, P) the BdG matrix of the node; it gives i dR(q)/dt = [H(q), R(q)].  The unknowns are the real and the
//! imaginary parts of the 144 x 512 = 73 728 complex entries of the V(q); CVODE integrates (D1) with Adams-Moulton
//! formulas, the fixed-point iteration and the error weights of complex unknowns (their modulus), rtol 1e-12,
//! atol 1e-14 (tighter than 1e-10 and 1e-12, and not slower: CVODE then takes the order 7 instead of 5; the energy per
//! volume is the small difference of the kinetic and the interaction energies, about ten times smaller than either, so
//! that its conservation needs the tighter tolerance).  The nodes are shared by two threads.
//!
//! WHAT IS CONSERVED.  The energy per volume (the self-energies are the derivatives of the energy, so that
//! dE/dt = sum_q w_q tr(H dR/dt) = -i sum_q w_q tr(H [H, R]) = 0), the charge densities of the generators that are
//! symmetries of the energy, and the orthonormality V(q)^dag V(q) = 1 of the vectors (the evolution is unitary).  The
//! symmetries are all eight generators T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot in the second version, and T^1,
//! T^2, T^3, T^3_R, R, F_tot in the first, where chi'_SH is not paired and T^1_R, T^2_R, which mix chi_SH with
//! chi'_SH, do not commute with the pairing term of chi_SH alone.
//!
//! THE RUNS.  From the ground state (N_0, P_0) of each method (the self-consistent solution from the start of stage 3,
//! its phase rotated so that the coefficient d_0 of the vacuum direction P_dir is real and positive):
//!   ground      the ground state itself, which must stay at rest; second version, HS (0, 0) and CJT, 0 <= t <= 100;
//!   amplitude   the ground state of the BdG matrices of (N_0, P_0 + eps d_0 P_dir), eps = 1e-3: the length of the
//!               pair fields displaced along the vacuum direction; five methods, both versions, 0 <= t <= 300;
//!   triplet     the same with P_trip = K'_u^dag + K_d^dag in place of P_dir: the neutral member of the custodial
//!               triplet (the two pair fields displaced oppositely); second version, five methods;
//!   josephson   the ground state evolved with H - mu F_tot, mu = 0.01: the phase of d(t) must turn at the rate
//!               2 mu (each pair carries the particle number 2) with |d(t)| constant; second version, HS (0, 0) and CJT,
//!               0 <= t <= 100;
//!   large       the amplitude kick with eps = 0.3; HS (0, 0), both versions, 0 <= t <= 300.
//! At every output time (every 0.2) the program records the coefficients d(t) of P(t) along P_dir and d_trip(t) along
//! P_trip, the energy per volume and the eight charge densities.
//!
//! THE SPECTRA.  The Fourier transform of a signal s(t_i) (|d(t)| for the amplitude runs, Re d_trip(t) for the triplet
//! runs, its mean removed) with the Hann window h(t) = sin^2(pi t / T) over the whole run of length T,
//!     |S(w)| = | sum_i h(t_i) s(t_i) exp(i w t_i) dt |,
//! scanned for 0.05 <= w <= 3 with the step 0.001; its largest peak is refined by golden sections.  Its half width at
//! half maximum is compared with that of a pure cosine sampled in the same way (the width of the window): a ratio near
//! 1 means an undamped oscillation over the run.
//!
//! The checks (report data/hck_sh_dynamics_checks.txt):
//!   D-1  the integrator: y'' = -y over 100 units against cos t and -sin t;
//!   D-2  the ground state stays at rest: |d(t) - d_0| / d_0 over 0 <= t <= 100, HS (0, 0) and CJT;
//!   D-3  the conservation laws in every run: the energy per volume (relative to |E_0|), the charge densities of the
//!        symmetries, and the orthonormality of the vectors at the end of the run;
//!   D-4  the amplitude mode of HS (0, 1) and HS (0, 0) in the time domain: the peak of |d(t)| lies at d_0, the
//!        frequency that stage 4 found by the linear response, and it has the width of the window (undamped), both
//!        versions;
//!   D-5  second version, HS (0, 1) and HS (0, 0): the triplet oscillates at the frequency of the amplitude mode (the
//!        degeneracy of the custodial singlet and triplet of stage 4);
//!   D-6  the Josephson relation: the phase of d(t) turns at the rate 2 mu, and |d(t)| stays at d_0;
//!   D-7  KS and CJT give the same dynamics (their ground states and their susceptibilities agree, stages 3 and 4);
//!   D-8  in the methods with normal self-energies (HS (1, 1), KS, CJT) the spectra of the amplitude and of the triplet
//!        have a peak; reported against twice the gap of stage 3.
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_dynamics
//! Output in data/: hck_sh_dynamics_checks.txt, sh_dyn_runs.csv, sh_dyn_signals.csv, sh_dyn_spectra.csv,
//! sh_dyn_values.csv.  It reads data/sh_sf_ground.csv (the gaps of stage 3) and data/sh_hg_minimum.csv (the d_0 of
//! stage 4).  Run time about forty minutes on two threads.
#[allow(dead_code)]
#[path = "../../shsuperfluid/src/model.rs"]
mod model;

use model::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{sigma, t_gen};
use sfcore::methods::Method;
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::uniform_n::{ModelN, SolutionN};
use sfsundials::cvode::{self, CvodeSettings, Weights};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

/// the potential of stage 3 in the second version (both pairing terms, decision D31) and in the first version
const V1_SH: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 100.8 };
const V1_SH_FIRST: PotSh = PotSh { gs: -8.0, gv: 0.8, gsh: 100.8, gsh2: 0.0 };
/// the static iteration of stage 3: mixing weight, tolerance, largest number of steps, the size below which an element
/// is set to zero
const MIX: f64 = 0.6;
const TOL: f64 = 1e-13;
const MAXIT: usize = 40000;
const FLUSH: f64 = 1e-30;
/// the tolerances of CVODE
const RTOL: f64 = 1e-12;
const ATOL: f64 = 1e-14;
/// the spacing of the output times, the lengths of the runs, the kicks and the chemical potential of the Josephson runs
const DT: f64 = 0.2;
const T_LONG: f64 = 300.0;
const T_SHORT: f64 = 100.0;
const EPS: f64 = 1e-3;
const EPS_LARGE: f64 = 0.3;
const MU_J: f64 = 0.01;
/// the threads that share the nodes
const THREADS: usize = 2;
/// the real unknowns of one node: 2N rows times N columns, complex
const PER: usize = 2 * (2 * N) * N;

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
/// the same method (Method has no equality; its name identifies it)
fn same(a: &Method, b: &Method) -> bool {
    mname(a) == mname(b)
}
/// the methods in which only the pairing terms act on the Higgs ground state (no normal self-energy)
fn pairing_only(m: &Method) -> bool {
    matches!(m, Method::Hs { s_s, .. } if *s_s == 0.0)
}

/// the self-consistent solution by the iteration of stage 3 (linear mixing with the weight MIX, N made Hermitian and P
/// antisymmetric after each step, elements below FLUSH set to zero)
fn solve_clean(mo: &ModelN, n0: &CMat, p0: &CMat) -> SolutionN {
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
        let res = n2.sub(&nself).max_abs().max(p2.sub(&pself).max_abs());
        if res < TOL || it >= MAXIT {
            return SolutionN { n_self: n2, p_self: p2, drho, kap, energy: ek + ei, e_kin: ek, e_int: ei, iterations: it, residual: res };
        }
        nself = n2.scale_r(MIX).add(&nself.scale_r(1.0 - MIX));
        nself = nself.add(&nself.dagger()).scale_r(0.5);
        flush(&mut nself);
        pself = p2.scale_r(MIX).add(&pself.scale_r(1.0 - MIX));
        pself = pself.sub(&pself.transpose()).scale_r(0.5);
        flush(&mut pself);
        it += 1;
    }
}

/// the direction of the vacuum in the pair self-energy: second version K'_u^dag - K_d^dag (the custodial singlet),
/// first version K_d^dag
fn p_dir(ver: Ver) -> CMat {
    match ver {
        Ver::Second => k_ds(1, 0).dagger().sub(&k_ds(0, 1).dagger()),
        Ver::First => k_ds(0, 1).dagger(),
    }
}
/// the neutral member of the custodial triplet in the pair self-energy: K'_u^dag + K_d^dag (the two pair fields
/// displaced oppositely; orthogonal to the vacuum direction of the second version)
fn p_trip() -> CMat {
    k_ds(1, 0).dagger().add(&k_ds(0, 1).dagger())
}
/// the complex coefficient of the direction dir in a pair self-energy, <dir, P> / <dir, dir>
fn coef(p: &CMat, dir: &CMat) -> C64 {
    dir.dagger().mul(p).trace().scale(1.0 / dir.dagger().mul(dir).trace().re)
}

/// the eight generators of the global group on the sixteen components: T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot
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
/// the generators that are symmetries of the energy: all eight in the second version; in the first T^1_R and T^2_R
/// are not
fn conserved(ver: Ver) -> Vec<usize> {
    match ver {
        Ver::Second => (0..8).collect(),
        Ver::First => vec![0, 1, 2, 5, 6, 7],
    }
}

/// a copy of a model with an external one-body term Psi^dag X Psi
fn with_ext(mo: &ModelN, x: &CMat) -> ModelN {
    let mut m2 = ModelN::new(mo.blocks.clone(), mo.mu, mo.terms.clone(), mo.grid.clone());
    m2.ext_n = Some(x.clone());
    m2
}

// ------------------------------------------------------------------------------- the state and the right side of (D1)

/// the occupied quasi-particle vectors (the eigenvectors of negative energy) of the BdG matrices of (N, P) at every
/// node, packed: node n, column k (0..N), row a (0..2N) at the real index n PER + 2 (k 2N + a) and the next one
fn occupied(mo: &ModelN, nself: &CMat, pself: &CMat) -> Vec<f64> {
    let m2 = 2 * N;
    let nn = mo.grid.len();
    let mut y = vec![0.0; nn * PER];
    for n in 0..nn {
        let (ev, v) = eigh(&mo.bdg(n, nself, pself));
        let occ: Vec<usize> = (0..m2).filter(|&k| ev[k] < 0.0).collect();
        assert_eq!(occ.len(), N, "a BdG matrix without N eigenvalues below zero at the node {}", n);
        for (col, &k) in occ.iter().enumerate() {
            for a in 0..m2 {
                let z = v.get(a, k);
                let e = n * PER + 2 * (col * m2 + a);
                y[e] = z.re;
                y[e + 1] = z.im;
            }
        }
    }
    y
}

/// the vectors of one node as complex numbers (column k, row a at k 2N + a)
fn node_vectors(y: &[f64]) -> Vec<C64> {
    (0..PER / 2).map(|e| c(y[2 * e], y[2 * e + 1])).collect()
}

/// C^n = R_11 and C^p = -R_12 of one node from its vectors (R = V V^dag)
fn node_corr(v: &[C64]) -> (CMat, CMat) {
    let m2 = 2 * N;
    let mut cn = CMat::zeros(N);
    let mut cp = CMat::zeros(N);
    for i in 0..N {
        for j in 0..N {
            let (mut s1, mut s2) = (C64::ZERO, C64::ZERO);
            for k in 0..N {
                let vi = v[k * m2 + i];
                s1 += vi * v[k * m2 + j].conj();
                s2 += vi * v[k * m2 + N + j].conj();
            }
            cn.a[i * N + j] = s1;
            cp.a[i * N + j] = -s2;
        }
    }
    (cn, cp)
}

/// what a state gives: the energy per volume (with the external term, if any), the self-energies, the local matrices
struct Fields {
    energy: f64,
    nself: CMat,
    pself: CMat,
    drho: CMat,
}

fn fields(mo: &ModelN, y: &[f64]) -> Fields {
    let nn = mo.grid.len();
    let per = nn.div_ceil(THREADS);
    let mut cns = vec![CMat::zeros(N); nn];
    let mut cps = vec![CMat::zeros(N); nn];
    std::thread::scope(|s| {
        for ((yc, cnc), cpc) in y.chunks(per * PER).zip(cns.chunks_mut(per)).zip(cps.chunks_mut(per)) {
            s.spawn(move || {
                for k in 0..cnc.len() {
                    let (a, b) = node_corr(&node_vectors(&yc[k * PER..(k + 1) * PER]));
                    cnc[k] = a;
                    cpc[k] = b;
                }
            });
        }
    });
    let (drho, kap, ek) = mo.local(&cns, &cps);
    let (ei, nself, pself) = mo.interaction(&drho, &kap);
    Fields { energy: ek + ei, nself, pself, drho }
}

/// the right side of (D1): dV(q)/dt = -i H(q) V(q) at every node
fn rhs(mo: &ModelN, y: &[f64], dy: &mut [f64]) {
    let fl = fields(mo, y);
    let nn = mo.grid.len();
    let per = nn.div_ceil(THREADS);
    let m2 = 2 * N;
    std::thread::scope(|s| {
        for (ci, (yc, dc)) in y.chunks(per * PER).zip(dy.chunks_mut(per * PER)).enumerate() {
            let (ns, ps) = (&fl.nself, &fl.pself);
            s.spawn(move || {
                for k in 0..yc.len() / PER {
                    let h = mo.bdg(ci * per + k, ns, ps);
                    let v = node_vectors(&yc[k * PER..(k + 1) * PER]);
                    for col in 0..N {
                        for a in 0..m2 {
                            let mut sum = C64::ZERO;
                            for b in 0..m2 {
                                sum += h.a[a * m2 + b] * v[col * m2 + b];
                            }
                            // -i sum
                            let e = k * PER + 2 * (col * m2 + a);
                            dc[e] = sum.im;
                            dc[e + 1] = -sum.re;
                        }
                    }
                }
            });
        }
    });
}

/// the largest |V^dag V - 1| over the nodes
fn orthonormality(y: &[f64], nn: usize) -> f64 {
    let m2 = 2 * N;
    let mut worst = 0.0f64;
    for n in 0..nn {
        let v = node_vectors(&y[n * PER..(n + 1) * PER]);
        for i in 0..N {
            for j in 0..N {
                let mut s = C64::ZERO;
                for a in 0..m2 {
                    s += v[i * m2 + a].conj() * v[j * m2 + a];
                }
                let want = if i == j { 1.0 } else { 0.0 };
                worst = worst.max((s - c(want, 0.0)).abs());
            }
        }
    }
    worst
}

// ------------------------------------------------------------------------------------------------- one run

/// the record of one run
struct Run {
    name: String,
    ver: Ver,
    method: Method,
    kind: &'static str,
    eps: f64,
    d0: f64,
    ts: Vec<f64>,
    d: Vec<C64>,
    dtrip: Vec<C64>,
    energy: Vec<f64>,
    charges: Vec<Vec<f64>>,
    ortho: f64,
    stats: String,
    seconds: f64,
}

/// integrates (D1) from the vectors y0 with the model mo to t_end and records the observables at every output time
#[allow(clippy::too_many_arguments)]
fn run(name: &str, mo: Arc<ModelN>, y0: Vec<f64>, t_end: f64, ver: Ver, method: Method, kind: &'static str, eps: f64, d0: f64) -> Run {
    let t0 = Instant::now();
    let n_out = (t_end / DT).round() as usize;
    let touts: Vec<f64> = (0..=n_out).map(|i| i as f64 * DT).collect();
    let mr = mo.clone();
    let f: cvode::Rhs = Box::new(move |_t, y, dy| {
        rhs(&mr, y, dy);
        0
    });
    let set = CvodeSettings::adams(RTOL, ATOL, Weights::Complex);
    let gens = generators();
    let (pd, pt) = (p_dir(ver), p_trip());
    let (mut ts, mut d, mut dtrip, mut energy, mut charges) = (vec![], vec![], vec![], vec![], vec![]);
    let mut last = y0.clone();
    let st = cvode::integrate(&y0, 0.0, &touts, &set, f, |_, t, y| {
        let fl = fields(&mo, y);
        ts.push(t);
        d.push(coef(&fl.pself, &pd));
        dtrip.push(coef(&fl.pself, &pt));
        energy.push(fl.energy);
        charges.push(gens.iter().map(|q| q.mul(&fl.drho).trace().re).collect::<Vec<f64>>());
        last.copy_from_slice(y);
        true
    })
    .unwrap_or_else(|e| panic!("CVODE failed in the run {}: {}", name, e));
    let ortho = orthonormality(&last, mo.grid.len());
    let r = Run {
        name: name.to_string(),
        ver,
        method,
        kind,
        eps,
        d0,
        ts,
        d,
        dtrip,
        energy,
        charges,
        ortho,
        stats: st.describe(),
        seconds: t0.elapsed().as_secs_f64(),
    };
    println!("run {}: {} ({:.0} s)", name, r.stats, r.seconds);
    r
}

// ------------------------------------------------------------------------------------------------- the spectra

/// |S(w)| of the samples s (spacing dt) with the Hann window over the whole record
fn spectrum_at(s: &[f64], dt: f64, w: f64) -> f64 {
    let tt = (s.len() - 1) as f64 * dt;
    let (mut re, mut im) = (0.0, 0.0);
    for (i, &x) in s.iter().enumerate() {
        let t = i as f64 * dt;
        let h = (PI * t / tt).sin().powi(2);
        re += h * x * (w * t).cos();
        im += h * x * (w * t).sin();
    }
    (re * re + im * im).sqrt() * dt
}

/// the largest peak of |S(w)| for w_lo <= w <= w_hi: (its frequency, its height, its half width at half maximum)
fn peak(s: &[f64], dt: f64, w_lo: f64, w_hi: f64) -> (f64, f64, f64) {
    let step = 0.001;
    let nw = ((w_hi - w_lo) / step).round() as usize;
    let mut best = (w_lo, 0.0);
    for i in 0..=nw {
        let w = w_lo + i as f64 * step;
        let a = spectrum_at(s, dt, w);
        if a > best.1 {
            best = (w, a);
        }
    }
    // golden sections on -|S| in [w - step, w + step]
    let gr = 0.5 * (5f64.sqrt() - 1.0);
    let (mut a, mut b) = (best.0 - step, best.0 + step);
    let (mut x1, mut x2) = (b - gr * (b - a), a + gr * (b - a));
    let (mut f1, mut f2) = (spectrum_at(s, dt, x1), spectrum_at(s, dt, x2));
    for _ in 0..60 {
        if f1 > f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = b - gr * (b - a);
            f1 = spectrum_at(s, dt, x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = a + gr * (b - a);
            f2 = spectrum_at(s, dt, x2);
        }
    }
    let wp = 0.5 * (a + b);
    let hp = spectrum_at(s, dt, wp);
    // the half width at half maximum: the nearest points on both sides where |S| = hp / 2 (steps of 1e-4, then bisection)
    let half = |dir: f64| -> f64 {
        let mut w = wp;
        loop {
            let w2 = w + dir * 1e-4;
            if (w2 - wp).abs() > 0.5 || spectrum_at(s, dt, w2) < 0.5 * hp {
                let (mut lo, mut hi) = (w, w2);
                for _ in 0..40 {
                    let m = 0.5 * (lo + hi);
                    if spectrum_at(s, dt, m) > 0.5 * hp {
                        lo = m;
                    } else {
                        hi = m;
                    }
                }
                return (0.5 * (lo + hi) - wp).abs();
            }
            w = w2;
        }
    };
    (wp, hp, 0.5 * (half(-1.0) + half(1.0)))
}

/// the signal of a run with its mean removed: |d(t)| (amplitude, large) or Re d_trip(t) (triplet)
fn signal(r: &Run) -> Vec<f64> {
    let s: Vec<f64> = if r.kind == "triplet" { r.dtrip.iter().map(|z| z.re).collect() } else { r.d.iter().map(|z| z.abs()).collect() };
    let mean = s.iter().sum::<f64>() / s.len() as f64;
    s.iter().map(|x| x - mean).collect()
}

/// the analysis of a run: (peak frequency, height, half width, the half width of a pure cosine at that frequency
/// sampled in the same way, the ratio of the rms of the signal in the last sixth of the run to that in [T/12, T/4])
fn analyse(r: &Run) -> (f64, f64, f64, f64, f64) {
    let s = signal(r);
    let (wp, hp, hw) = peak(&s, DT, 0.05, 3.0);
    let pure: Vec<f64> = r.ts.iter().map(|t| (wp * t).cos()).collect();
    let pm = pure.iter().sum::<f64>() / pure.len() as f64;
    let pure: Vec<f64> = pure.iter().map(|x| x - pm).collect();
    let (_, _, hw0) = peak(&pure, DT, (wp - 0.05).max(0.01), wp + 0.05);
    let n = s.len();
    let rms = |a: usize, b: usize| (s[a..b].iter().map(|x| x * x).sum::<f64>() / (b - a) as f64).sqrt();
    let ratio = rms(5 * n / 6, n) / rms(n / 12, n / 4);
    (wp, hp, hw, hw0, ratio)
}

/// the stage-3 gaps (version, method) -> gap, of the Higgs superfluid of each version
fn read_gaps(p: &Path) -> HashMap<(String, String), f64> {
    let s = std::fs::read_to_string(p).unwrap_or_else(|_| panic!("missing {} (run sh_superfluid first)", p.display()));
    let mut lines = s.lines();
    let head: Vec<&str> = lines.next().unwrap().split(',').collect();
    let col = |name: &str| head.iter().position(|h| *h == name).unwrap();
    let (cv, cm, ck, cg) = (col("version"), col("method"), col("kind"), col("gap"));
    let mut out = HashMap::new();
    for line in lines {
        // split on the commas outside quotes
        let mut fields = vec![];
        let mut cur = String::new();
        let mut quoted = false;
        for ch in line.chars() {
            match ch {
                '"' => quoted = !quoted,
                ',' if !quoted => {
                    fields.push(cur.clone());
                    cur.clear();
                }
                _ => cur.push(ch),
            }
        }
        fields.push(cur);
        if fields.len() > cg && fields[ck].starts_with("Higgs superfluid") {
            if let Ok(g) = fields[cg].parse::<f64>() {
                out.insert((fields[cv].clone(), fields[cm].clone()), g);
            }
        }
    }
    out
}

fn main() {
    let t_start = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sh_dynamics: checks of the dynamics in time of the Higgs superfluid by the time-dependent mean field of the five \
         methods (stage 6 of the revision, part one, second version with the first as the comparison, Revision_SM_Higgs, \
         PROJECT spacetimes)",
    );
    let gaps = read_gaps(&data.join("sh_sf_ground.csv"));

    // ------------------------------------------------------------------ D-1: the integrator
    {
        let touts: Vec<f64> = (0..=1000).map(|i| i as f64 * 0.1).collect();
        let f: cvode::Rhs = Box::new(|_t, y, dy| {
            dy[0] = y[1];
            dy[1] = -y[0];
            0
        });
        let set = CvodeSettings::adams(1e-12, 1e-14, Weights::Real);
        let mut err = 0.0f64;
        cvode::integrate(&[1.0, 0.0], 0.0, &touts, &set, f, |_, t, y| {
            err = err.max((y[0] - t.cos()).abs()).max((y[1] + t.sin()).abs());
            true
        })
        .expect("CVODE failed on the test");
        rep.add(
            "D-1",
            "the integrator (CVODE, Adams-Moulton, rtol 1e-12, atol 1e-14): y'' = -y from (1, 0) over 0 <= t <= 100 against \
             (cos t, -sin t) (largest absolute deviation at the 1001 output times)",
            err,
            1e-8,
        );
    }

    // ------------------------------------------------------------------ the ground states and the runs
    let mut runs: Vec<Run> = vec![];
    let mut ground: HashMap<(String, String), (f64, f64)> = HashMap::new();
    for ver in [Ver::Second, Ver::First] {
        for method in methods() {
            let mo = Arc::new(model(&method, &ver.pot(), NR));
            let sol = solve_clean(&mo, &CMat::zeros(N), &p_dir(ver).scale_r(0.3));
            let dc = coef(&sol.p_self, &p_dir(ver));
            let ph = dc.scale(1.0 / dc.abs()).conj();
            let p0 = sol.p_self.scale(ph);
            let n0 = sol.n_self.clone();
            let d0 = dc.abs();
            ground.insert((ver.name().into(), mname(&method)), (d0, sol.energy));
            println!("{} {}: d0 {:.10} E {:.10e} ({} iterations)", ver.name(), mname(&method), d0, sol.energy, sol.iterations);
            let tag = format!("{}_{}", ver.name(), mname(&method).replace(' ', "").replace('(', "").replace(')', "").replace(',', "_"));
            let pd = p_dir(ver);
            // the ground state at rest
            if ver == Ver::Second && (same(&method, &Method::Hs { s_s: 0.0, s_v: 0.0 }) || same(&method, &Method::Cjt)) {
                let y0 = occupied(&mo, &n0, &p0);
                runs.push(run(&format!("ground_{}", tag), mo.clone(), y0, T_SHORT, ver, method, "ground", 0.0, d0));
                // the Josephson run: the same state evolved with H - mu F_tot
                let mj = Arc::new(with_ext(&mo, &CMat::identity(N).scale_r(-MU_J)));
                let y0 = occupied(&mo, &n0, &p0);
                runs.push(run(&format!("josephson_{}", tag), mj, y0, T_SHORT, ver, method, "josephson", 0.0, d0));
            }
            // the amplitude kick
            let y0 = occupied(&mo, &n0, &p0.add(&pd.scale_r(EPS * d0)));
            runs.push(run(&format!("amplitude_{}", tag), mo.clone(), y0, T_LONG, ver, method, "amplitude", EPS, d0));
            // the triplet kick (second version)
            if ver == Ver::Second {
                let y0 = occupied(&mo, &n0, &p0.add(&p_trip().scale_r(EPS * d0)));
                runs.push(run(&format!("triplet_{}", tag), mo.clone(), y0, T_LONG, ver, method, "triplet", EPS, d0));
            }
            // the large kick (HS (0, 0))
            if same(&method, &Method::Hs { s_s: 0.0, s_v: 0.0 }) {
                let y0 = occupied(&mo, &n0, &p0.add(&pd.scale_r(EPS_LARGE * d0)));
                runs.push(run(&format!("large_{}", tag), mo.clone(), y0, T_LONG, ver, method, "large", EPS_LARGE, d0));
            }
        }
    }

    // ------------------------------------------------------------------ D-2: the ground state stays at rest
    let mut d2 = 0.0f64;
    for r in runs.iter().filter(|r| r.kind == "ground") {
        for z in &r.d {
            d2 = d2.max((*z - c(r.d0, 0.0)).abs() / r.d0);
        }
    }
    rep.add(
        "D-2",
        "the ground state stays at rest: the largest |d(t) - d_0| / d_0 over 0 <= t <= 100, second version, HS (0, 0) and CJT",
        d2,
        1e-7,
    );

    // ------------------------------------------------------------------ D-3: the conservation laws
    let (mut de, mut dq, mut dort) = (0.0f64, 0.0f64, 0.0f64);
    let mut worst_e = String::new();
    for r in &runs {
        let e0 = r.energy[0];
        let scale = r.energy.iter().map(|x| x.abs()).fold(0.0f64, f64::max).max(1e-300);
        let dev = r.energy.iter().map(|e| (e - e0).abs()).fold(0.0f64, f64::max) / scale;
        if dev > de {
            de = dev;
            worst_e = r.name.clone();
        }
        for &a in &conserved(r.ver) {
            let q0 = r.charges[0][a];
            for q in &r.charges {
                dq = dq.max((q[a] - q0).abs());
            }
        }
        dort = dort.max(r.ortho);
    }
    rep.add(
        "D-3a",
        &format!(
            "the energy per volume is conserved in every run (largest |E(t) - E(0)| relative to the largest |E(t)| of the run; \
             the largest in {}); the energy per volume is the difference of its kinetic and interaction parts, each about ten \
             times larger, and its drift follows the loss of orthonormality of D-3c that the integration error causes (the \
             exact flow conserves it: check WD-1)",
            worst_e
        ),
        de,
        1e-5,
    );
    rep.add(
        "D-3b",
        "the charge densities of the symmetries are conserved in every run (all eight generators in the second version; T^1, \
         T^2, T^3, T^3_R, R, F_tot in the first; largest absolute change)",
        dq,
        1e-9,
    );
    rep.add(
        "D-3c",
        "the evolution is unitary: the largest |V(q)^dag V(q) - 1| over the nodes at the end of every run",
        dort,
        1e-7,
    );

    // ------------------------------------------------------------------ the spectra
    let mut ana: HashMap<String, (f64, f64, f64, f64, f64)> = HashMap::new();
    for r in runs.iter().filter(|r| r.kind == "amplitude" || r.kind == "triplet" || r.kind == "large") {
        let a = analyse(r);
        println!("{}: peak {:.8} height {:.3e} hwhm {:.5} (window {:.5}) envelope {:.4}", r.name, a.0, a.1, a.2, a.3, a.4);
        ana.insert(r.name.clone(), a);
    }
    let find = |kind: &str, ver: Ver, m: Method| -> &Run { runs.iter().find(|r| r.kind == kind && r.ver == ver && same(&r.method, &m)).unwrap() };

    // ------------------------------------------------------------------ D-4: the amplitude mode of the pairing-only methods
    let (mut d4, mut d4w) = (0.0f64, 0.0f64);
    let mut det4 = vec![];
    for ver in [Ver::Second, Ver::First] {
        for m in [Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }] {
            let r = find("amplitude", ver, m);
            let a = ana[&r.name];
            d4 = d4.max((a.0 - r.d0).abs() / r.d0);
            d4w = d4w.max(a.2 / a.3);
            det4.push(format!("{} {}: peak {:.8} against d_0 = {:.8}, width ratio {:.4}", ver.name(), mname(&m), a.0, r.d0, a.2 / a.3));
        }
    }
    rep.add(
        "D-4a",
        &format!(
            "the amplitude mode in the time domain: the peak of the spectrum of |d(t)| after the kick eps = 1e-3 lies at d_0, the \
             frequency of the linear response of stage 4 (relative), HS (0, 1) and HS (0, 0), both versions [{}]",
            det4.join("; ")
        ),
        d4,
        1e-4,
    );
    rep.add(
        "D-4b",
        "the amplitude mode is undamped over the run: the half width of its peak equals that of a pure cosine sampled in the \
         same way, the width of the window (largest ratio minus 1), HS (0, 1) and HS (0, 0), both versions",
        d4w - 1.0,
        0.02,
    );

    // ------------------------------------------------------------------ D-5: the triplet of the pairing-only methods
    let mut d5 = 0.0f64;
    for m in [Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }] {
        let wa = ana[&find("amplitude", Ver::Second, m).name].0;
        let wt = ana[&find("triplet", Ver::Second, m).name].0;
        d5 = d5.max((wt - wa).abs() / wa);
    }
    rep.add(
        "D-5",
        "second version, HS (0, 1) and HS (0, 0): the neutral member of the custodial triplet oscillates at the frequency of the \
         amplitude mode (relative difference of the peaks), the degeneracy of stage 4",
        d5,
        1e-4,
    );

    // ------------------------------------------------------------------ D-6: the Josephson relation
    let (mut d6, mut d6m) = (0.0f64, 0.0f64);
    let mut det6 = vec![];
    for r in runs.iter().filter(|r| r.kind == "josephson") {
        // the unwrapped phase of d(t), fitted by a straight line
        let mut ph = vec![];
        let mut prev = 0.0;
        let mut turns = 0.0;
        for (i, z) in r.d.iter().enumerate() {
            let a = z.arg();
            if i > 0 {
                let mut dd = a - prev;
                while dd > PI {
                    dd -= 2.0 * PI;
                    turns -= 2.0 * PI;
                }
                while dd < -PI {
                    dd += 2.0 * PI;
                    turns += 2.0 * PI;
                }
            }
            prev = a;
            ph.push(a + turns);
        }
        let n = ph.len() as f64;
        let (st, sp) = (r.ts.iter().sum::<f64>(), ph.iter().sum::<f64>());
        let stt = r.ts.iter().map(|t| t * t).sum::<f64>();
        let stp = r.ts.iter().zip(ph.iter()).map(|(t, p)| t * p).sum::<f64>();
        let slope = (n * stp - st * sp) / (n * stt - st * st);
        d6 = d6.max((slope.abs() - 2.0 * MU_J).abs() / (2.0 * MU_J));
        let mdev = r.d.iter().map(|z| (z.abs() - r.d0).abs()).fold(0.0f64, f64::max) / r.d0;
        d6m = d6m.max(mdev);
        det6.push(format!("{} {}: rate {:.12} (2 mu = {}), largest | |d| - d_0 | / d_0 {:.1e}", r.ver.name(), mname(&r.method), slope, 2.0 * MU_J, mdev));
    }
    rep.add(
        "D-6a",
        &format!(
            "the Josephson relation: with H - mu F_tot (mu = 0.01) the phase of d(t) turns at the rate 2 mu (relative difference of \
             the modulus of the fitted rate; each pair carries the particle number 2), second version, HS (0, 0) and CJT [{}]",
            det6.join("; ")
        ),
        d6,
        1e-6,
    );
    rep.add("D-6b", "in the same runs |d(t)| stays at d_0 (largest relative deviation)", d6m, 1e-7);

    // ------------------------------------------------------------------ D-7: KS and CJT
    let mut d7 = 0.0f64;
    for ver in [Ver::Second, Ver::First] {
        let (a, b) = (find("amplitude", ver, Method::Ks), find("amplitude", ver, Method::Cjt));
        for (x, y) in a.d.iter().zip(b.d.iter()) {
            d7 = d7.max((*x - *y).abs() / a.d0);
        }
    }
    let (a, b) = (find("triplet", Ver::Second, Method::Ks), find("triplet", Ver::Second, Method::Cjt));
    for (x, y) in a.dtrip.iter().zip(b.dtrip.iter()) {
        d7 = d7.max((*x - *y).abs() / a.d0);
    }
    rep.add(
        "D-7",
        "KS and CJT give the same dynamics: the largest |d_KS(t) - d_CJT(t)| / d_0 in the amplitude runs of both versions and of \
         d_trip(t) in the triplet runs",
        d7,
        1e-6,
    );

    // ------------------------------------------------------------------ D-8: the other methods
    let mut ok8 = true;
    let mut det8 = vec![];
    for ver in [Ver::Second, Ver::First] {
        for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Ks, Method::Cjt] {
            let r = find("amplitude", ver, m);
            let a = ana[&r.name];
            let g2 = 2.0 * gaps[&(ver.name().to_string(), mname(&m))];
            ok8 &= a.0.is_finite() && a.1 > 0.0;
            let mut s = format!("{} {}: amplitude peak {:.6} (2 E_gap {:.6}, d_0 {:.6}), width ratio {:.3}", ver.name(), mname(&m), a.0, g2, r.d0, a.2 / a.3);
            if ver == Ver::Second {
                let t = ana[&find("triplet", ver, m).name];
                ok8 &= t.0.is_finite() && t.1 > 0.0;
                s.push_str(&format!(", triplet peak {:.6}, width ratio {:.3}", t.0, t.2 / t.3));
            }
            det8.push(s);
        }
    }
    rep.add_bool(
        "D-8",
        "in the methods with normal self-energies (HS (1, 1), KS, CJT) the spectra of the amplitude runs (both versions) and of \
         the triplet runs (second version) have a peak; reported with twice the gap of stage 3 and d_0",
        ok8,
        &det8.join("; "),
    );

    // ------------------------------------------------------------------ the data files
    let mut rows = vec![];
    for r in &runs {
        let (wp, hp, hw, hw0, env) = ana.get(&r.name).cloned().unwrap_or((f64::NAN, f64::NAN, f64::NAN, f64::NAN, f64::NAN));
        let e0 = r.energy[0];
        let scale = r.energy.iter().map(|x| x.abs()).fold(0.0f64, f64::max).max(1e-300);
        let edev = r.energy.iter().map(|e| (e - e0).abs()).fold(0.0f64, f64::max) / scale;
        let mut qdev = 0.0f64;
        for &a in &conserved(r.ver) {
            for q in &r.charges {
                qdev = qdev.max((q[a] - r.charges[0][a]).abs());
            }
        }
        let g2 = 2.0 * gaps[&(r.ver.name().to_string(), mname(&r.method))];
        rows.push(vec![
            r.name.clone(),
            r.ver.name().into(),
            mname(&r.method),
            r.kind.into(),
            f(r.eps),
            f(*r.ts.last().unwrap()),
            f(r.d0),
            f(g2),
            f(wp),
            f(hp),
            f(hw),
            f(hw0),
            f(env),
            f(edev),
            f(qdev),
            f(r.ortho),
            f(r.seconds),
            r.stats.clone(),
        ]);
    }
    write_csv(
        &data.join("sh_dyn_runs.csv"),
        &[
            "run", "version", "method", "kind", "eps", "t_end", "d0", "two_E_gap", "omega_peak", "peak_height", "hwhm", "hwhm_window",
            "envelope_ratio", "energy_drift", "charge_drift", "orthonormality", "seconds", "cvode",
        ],
        &rows,
    );
    // the signals every 1.0 (every fifth output time)
    let mut rows = vec![];
    for r in &runs {
        for i in (0..r.ts.len()).step_by(5) {
            rows.push(vec![r.name.clone(), f(r.ts[i]), f(r.d[i].re), f(r.d[i].im), f(r.dtrip[i].re), f(r.dtrip[i].im), f(r.energy[i])]);
        }
    }
    write_csv(&data.join("sh_dyn_signals.csv"), &["run", "t", "re_d", "im_d", "re_dtrip", "im_dtrip", "energy"], &rows);
    // the spectra for the figures (0.05 <= w <= 2.5, step 0.005)
    let mut rows = vec![];
    for r in runs.iter().filter(|r| ana.contains_key(&r.name)) {
        let s = signal(r);
        for i in 0..=490 {
            let w = 0.05 + 0.005 * i as f64;
            rows.push(vec![r.name.clone(), f(w), f(spectrum_at(&s, DT, w))]);
        }
    }
    write_csv(&data.join("sh_dyn_spectra.csv"), &["run", "omega", "abs_S"], &rows);
    let mut vals: Vec<Vec<String>> = vec![
        vec!["rtol".into(), f(RTOL)],
        vec!["atol".into(), f(ATOL)],
        vec!["dt_output".into(), f(DT)],
        vec!["t_long".into(), f(T_LONG)],
        vec!["t_short".into(), f(T_SHORT)],
        vec!["eps".into(), f(EPS)],
        vec!["eps_large".into(), f(EPS_LARGE)],
        vec!["mu_josephson".into(), f(MU_J)],
    ];
    for ((v, m), (d0, e)) in &ground {
        let tag = format!("{}_{}", v, m.replace(' ', "").replace('(', "").replace(')', "").replace(',', "_"));
        vals.push(vec![format!("d0_{}", tag), f(*d0)]);
        vals.push(vec![format!("energy_{}", tag), f(*e)]);
    }
    vals.sort();
    write_csv(&data.join("sh_dyn_values.csv"), &["name", "value"], &vals);

    let total = t_start.elapsed().as_secs_f64();
    let longest = runs.iter().map(|r| r.seconds).fold(0.0f64, f64::max);
    rep.note(&format!(
        "run time {:.0} s on {} threads; {} runs, the longest {:.0} s; {}",
        total,
        THREADS,
        runs.len(),
        longest,
        CvodeSettings::adams(RTOL, ATOL, Weights::Complex).describe()
    ));
    let _ = GEN_NAMES;
    let _ = pairing_only;
    rep.write(&data.join("hck_sh_dynamics_checks.txt"));
    let (np, nt) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, nt, total);
    if np != nt {
        std::process::exit(1);
    }
}
