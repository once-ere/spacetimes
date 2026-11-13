//! Command `forms` of the program `deflation_cjt_ks_c` (see main.rs): uniform matter and
//! the vacuum of the 4+4 model in the two forms of its self-interaction, the covariant form
//! C (the definition in this folder) and the quantized-sector form Q (the alternative),
//! for the same couplings of the Lagrangian (module `halves`).
//!
//! The fields are those of the quantized sector (they do not depend on x5, x6, x7): D = 4
//! space dimensions with momentum, or D = 3 if they do not depend on the hidden coordinate
//! x0 either.  On the deflating spacetime every coupling of the reduced Lagrangian grows as
//! exp(tau_d) (tau_d: the deflation clock), so that a scan in the strength of the couplings
//! at fixed ratios is a history along the clock.
//!
//! Writes data/forms_couplings.csv, forms_energy_vs_polarization.csv, forms_thresholds.csv,
//! forms_saturation.csv, forms_fixed_density.csv, forms_gas_events.csv, forms_cjt_vs_ks.csv,
//! forms_vacuum.csv, forms_vacuum_critical.csv, forms_unified_thresholds.csv,
//! forms_reversed_sign.csv, forms_checks.{txt,csv}.

use spacetimes::gamma::Gammas;
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks_c::cjt_contact::Cjt;
use spacetimes_deflation_cjt_ks_c::halves::{binding_threshold, Form, Halves, HalvesCouplings, Hf4, HState, KF_SCAN};
use crate::cmd_forms_b::run_part2;
use spacetimes_deflation_cjt_ks_c::io::{write_csv, write_csv_str, write_report};
use spacetimes_deflation_cjt_ks_c::kohn_sham::{global_min, ks_solve, Contact, Fermi, NU};
use spacetimes_deflation_cjt_ks_c::selfint::{reduce_couplings, SelfTables};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub(crate) const CMD: &str = "deflation_cjt_ks_c forms";
const ONE: [f64; 3] = [1.0; 3];

/// The sets of couplings of the Lagrangian: (tag, description, direction of (g_0, g_1) for the strength g = |g_0|).
/// All other couplings g_2 .. g_8 are zero, so that g_0eff = g_0 and g_1eff = g_1.
pub const SETS: [(&str, &str, f64, f64); 2] = [
    ("A", "scalar attraction alone: g_0 = -g, all other couplings zero", -1.0, 0.0),
    ("B", "the ratio of the author's example: g_0 = -g, g_1 = +0.1 g, all other couplings zero", -1.0, 0.1),
];

/// The nine couplings of the set number `set` at the strength g.
pub(crate) fn couplings9(set: usize, g: f64) -> [f64; 9] {
    let mut c = [0.0; 9];
    c[0] = SETS[set].2 * g;
    c[1] = SETS[set].3 * g;
    c
}

/// Apply f to 0..n on all processors; the results are returned in the order of the index.
pub(crate) fn par_map<T: Send, F: Fn(usize) -> T + Sync>(n: usize, f: F) -> Vec<T> {
    let next = AtomicUsize::new(0);
    let out: Vec<Mutex<Option<T>>> = (0..n).map(|_| Mutex::new(None)).collect();
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1).min(n.max(1));
    std::thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                if i >= n {
                    break;
                }
                let v = f(i);
                *out[i].lock().unwrap() = Some(v);
            });
        }
    });
    out.into_iter().map(|m| m.into_inner().unwrap().expect("par_map: a task did not finish")).collect()
}

/// The two halves in d dimensions for the form, the set and the strength g (units m = 1).
pub(crate) fn halves_of(tabs: &SelfTables, d: usize, form: Form, set: usize, g: f64) -> Halves {
    let hf = Hf4::from_tables(tabs, &couplings9(set, g));
    Halves { d, nu_half: NU / 2.0, c: HalvesCouplings::new(form, 1.0, &hf) }
}

/// The state of lowest energy per particle over kf_total in [0.05, 5] (scan `KF_SCAN` of the library): (kf_total, state).
pub(crate) fn lowest(h: &Halves) -> (f64, HState) {
    h.lowest_energy_per_particle(KF_SCAN.0, KF_SCAN.1, KF_SCAN.2).expect("forms: no state found")
}

/// The smallest strength g at which matter is self-bound (library routine `binding_threshold`).
fn threshold(tabs: &SelfTables, d: usize, form: Form, set: usize) -> f64 {
    let g = binding_threshold(d, form, &Hf4::from_tables(tabs, &couplings9(set, 1.0)));
    assert!(g.is_finite(), "forms: no binding threshold in the search interval ({:?}, D = {}, set {})", form, d, SETS[set].0);
    g
}

/// The same threshold for matter of ONE kind of particle with `nu` states and the couplings
/// G_S = cs g, G_V = cv g, by the routines of module `kohn_sham` (Gauss-Legendre integrals).
fn threshold_one_kind(d: usize, nu: f64, cs: f64, cv: f64) -> f64 {
    let f = Fermi::isotropic(d, nu, 96);
    let bound = |ln_g: f64| {
        let g = ln_g.exp();
        let c = Contact { m: 1.0, gs: cs * g, gv: cv * g };
        global_min(|l: f64| ks_solve(&f, &c, l.exp(), &ONE).e_per_particle - 1.0, KF_SCAN.0.ln(), KF_SCAN.1.ln(), 240, 1e-11).1 < -1e-10
    };
    let (mut lo, mut hi) = (1e-2f64.ln(), 1e5f64.ln());
    if bound(lo) || !bound(hi) {
        return f64::NAN;
    }
    for _ in 0..48 {
        let mid = 0.5 * (lo + hi);
        if bound(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

pub(crate) fn kind_of(pol: f64) -> &'static str {
    if pol < 1e-6 {
        "equal halves"
    } else if pol > 1.0 - 1e-6 {
        "one half"
    } else {
        "unequal halves"
    }
}

pub fn run() -> std::io::Result<bool> {
    let t0 = std::time::Instant::now();
    let gm = Gammas::build();
    let tabs = SelfTables::build(&gm);
    let mut rep = Report::new();
    let dims = [4usize, 3];

    // =====================================================================================
    // 1. the couplings of the two forms
    // =====================================================================================
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut f1 = true;
    let mut f1_txt: Vec<String> = Vec::new();
    for (is, (tag, desc, _, _)) in SETS.iter().enumerate() {
        let g9 = couplings9(is, 1.0);
        let hf = Hf4::from_tables(&tabs, &g9);
        let closed = Hf4::from_effective(&reduce_couplings(&g9));
        f1 &= (hf.gs_hf - closed.gs_hf).abs() < 1e-14 && (hf.gv_hf - closed.gv_hf).abs() < 1e-14 && (hf.gs_c - closed.gs_c).abs() < 1e-14 && (hf.gv_c - closed.gv_c).abs() < 1e-14;
        // the quantized-sector form: the couplings of the total densities are those of the exact table 44q
        f1 &= (hf.gs_hf - tabs.s44q.hf_coupling(1, &g9)).abs() < 1e-14 && (hf.gv_hf - tabs.s44q.hf_coupling(0, &g9)).abs() < 1e-14 && (hf.gs_c - tabs.s44q.hf_coupling(3, &g9)).abs() < 1e-14 && (hf.gv_c - tabs.s44q.hf_coupling(2, &g9)).abs() < 1e-14;
        for form in Form::both() {
            let c = HalvesCouplings::new(form, 1.0, &hf);
            rows.push(vec![tag.to_string(), form.tag().to_string(), desc.to_string(), format!("{}", c.tv), format!("{}", c.ts), format!("{}", c.dv), format!("{}", c.ds), format!("{}", c.a_v()), format!("{}", c.a_s()), format!("{}", c.b_v()), format!("{}", c.b_s())]);
        }
        f1_txt.push(format!("set {} ({}): G_S^HF = {} g, G_V^HF = {} g, G_Sc = {} g, G_Vc = {} g", tag, desc, hf.gs_hf, hf.gv_hf, hf.gs_c, hf.gv_c));
    }
    write_csv_str("forms_couplings.csv",
        &[("content", "the mean-field couplings of the two halves W = +1 and W = -1 of the 4+4 field for the sets of couplings of the Lagrangian used by the command `forms`, per unit of the strength g = |g_0|: E_int = (1/2)[T_V n^2 + T_S n_S^2 + D_V n_W^2 + D_S n_SW^2] with n = n_+ + n_-, n_W = n_+ - n_-, n_S = s_+ + s_-, n_SW = s_+ - s_- (number and scalar densities of the two halves); equivalently E_int = (1/2) a_V (n_+^2 + n_-^2) + b_V n_+ n_- + (1/2) a_S (s_+^2 + s_-^2) + b_S s_+ s_- (checks 44c-S7, 44q-S7, 44-S13, fm-F1)".to_string()),
          ("columns", "set; form: C = covariant form (the definition), Q = quantized-sector form (the alternative); description of the set; T_V, T_S: couplings of the total densities; D_V, D_S: couplings of the differences; a_V = T_V + D_V, a_S = T_S + D_S: couplings between particles of the same half (the same in both forms); b_V = T_V - D_V, b_S = T_S - D_S: couplings between particles of different halves (opposite in the two forms); all in units of g".to_string()),
          ("command", CMD.to_string())],
        &["set", "form", "description", "T_V", "T_S", "D_V", "D_S", "a_V", "a_S", "b_V", "b_S"], &rows)?;

    // the solver of two halves against the Kohn-Sham routines for matter of one kind (other integrals, other root search)
    let mut dev_eq: f64 = 0.0;
    let mut dev_one: f64 = 0.0;
    let mut dev_mu: f64 = 0.0;
    let mut dev_thermo: f64 = 0.0;
    let mut n_cmp = 0usize;
    for &d in &dims {
        let (f8, f4) = (Fermi::isotropic(d, NU, 96), Fermi::isotropic(d, NU / 2.0, 96));
        for is in 0..SETS.len() {
            for form in Form::both() {
                for g in if d == 4 { [40.0, 160.0] } else { [6.0, 24.0] } {
                    let h = halves_of(&tabs, d, form, is, g);
                    for kf in [0.3, 0.6, 0.9] {
                        // equal halves: the symmetric solution
                        let ks = ks_solve(&f8, &h.unpolarized_contact(), kf, &ONE);
                        if let Some(sym) = h.solutions(&[kf, kf]).iter().find(|s| (s[0] - s[1]).abs() < 1e-9 && (s[0] - ks.mass).abs() < 1e-7) {
                            let st = h.state_at(&[kf, kf], sym, 1);
                            dev_eq = dev_eq.max((st.mass[0] - ks.mass).abs()).max((st.eps / ks.eps - 1.0).abs()).max((st.p_flat - ks.p_flat).abs() / ks.eps.abs());
                        } else {
                            f1 = false;
                        }
                        // one half
                        let ks1 = ks_solve(&f4, &h.polarized_contact(), kf, &ONE);
                        match h.solve(&[kf, 0.0]) {
                            Some(st) => dev_one = dev_one.max((st.mass[0] - ks1.mass).abs()).max((st.eps / ks1.eps - 1.0).abs()).max((st.p_flat - ks1.p_flat).abs() / ks1.eps.abs()),
                            None => f1 = false,
                        }
                        // unequal halves: chemical potentials and pressure
                        let k2 = [kf, 0.8 * kf];
                        if let Some(st) = h.solve(&k2) {
                            let unit = h.unit_density();
                            let dk = 1e-5;
                            for i in 0..2 {
                                let (mut kp, mut km) = (k2, k2);
                                kp[i] += dk;
                                km[i] -= dk;
                                if let (Some(a), Some(b)) = (h.solve(&kp), h.solve(&km)) {
                                    let dn = unit * (kp[i].powi(d as i32) - km[i].powi(d as i32));
                                    dev_mu = dev_mu.max(((a.eps - b.eps) / dn / st.mu[i] - 1.0).abs());
                                }
                            }
                            dev_thermo = dev_thermo.max((st.mu[0] * st.n[0] + st.mu[1] * st.n[1] - st.eps - st.p_flat).abs() / st.eps.abs());
                        } else {
                            f1 = false;
                        }
                        n_cmp += 1;
                    }
                }
            }
        }
    }
    rep.add("fm-F1", "numerical", &format!("the mean-field couplings and the solver of two halves. (i) The four couplings G_S^HF = (15 g_0eff - 8 g_1eff - 8 g_7eff - g_8eff)/16, G_V^HF = (-g_0eff + 22 g_1eff - 6 g_7eff + g_8eff)/16, G_Sc = (-g_0eff - 2 g_1eff + 2 g_7eff + g_8eff)/16, G_Vc = -(g_0eff + g_8eff)/16 computed from these closed forms equal those of the exact coefficient tables (checks 44c-S7, 44q-S7) for the sets of couplings used here: {}. Covariant form C: T_V = G_Vc, T_S = G_Sc (total densities), D_V = G_V^HF, D_S = G_S^HF (differences); quantized-sector form Q: the two pairs exchanged. (ii) The solver of the coupled gap equations (closed forms of the momentum integrals; all solutions) against the Kohn-Sham routines for matter of one kind of particle (Gauss-Legendre integrals), for {} sets of form, dimension, couplings and Fermi momentum: with equal halves the symmetric solution is matter of eight states with the couplings T_S, T_V (mass, energy density, pressure agree to {:.1e}); with one half filled it is matter of four states with the couplings a_S = T_S + D_S, a_V = T_V + D_V ({:.1e}). (iii) With unequal halves (k_- = 0.8 k_+) the chemical potentials mu_+- = sqrt(k_+-^2 + M_+-^2) + V_+- are the derivatives of the energy density with respect to the densities of the halves (central differences: {:.1e}), and the pressure from the momentum integrals equals mu_+ n_+ + mu_- n_- - eps ({:.1e})", f1_txt.join("; "), n_cmp, dev_eq, dev_one, dev_mu, dev_thermo), f1 && dev_eq < 1e-9 && dev_one < 1e-9 && dev_mu < 1e-6 && dev_thermo < 1e-11);
    println!("[{:>6.1} s] couplings and solver checked", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 2. the binding thresholds
    // =====================================================================================
    // cases: (d, set, form)
    let mut cases: Vec<(usize, usize, Form)> = Vec::new();
    for &d in &dims {
        for is in 0..SETS.len() {
            for form in Form::both() {
                cases.push((d, is, form));
            }
        }
    }
    let thr: Vec<f64> = par_map(cases.len(), |i| threshold(&tabs, cases[i].0, cases[i].2, cases[i].1));
    let thr_of = |d: usize, is: usize, form: Form| -> f64 { thr[cases.iter().position(|c| *c == (d, is, form)).unwrap()] };
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut f2 = true;
    let mut f2_txt: Vec<String> = Vec::new();
    let mut dev_one_kind: f64 = 0.0;
    for (i, &(d, is, form)) in cases.iter().enumerate() {
        let g = thr[i];
        let (kf_a, above) = lowest(&halves_of(&tabs, d, form, is, 1.001 * g));
        let (_, below) = lowest(&halves_of(&tabs, d, form, is, 0.999 * g));
        f2 &= above.e_per_particle - 1.0 < 0.0 && below.e_per_particle - 1.0 >= -1e-10;
        let h = halves_of(&tabs, d, form, is, 1.0);
        // the threshold of matter of one kind that corresponds to the kind of state found
        let one_kind = if above.pol < 1e-6 {
            threshold_one_kind(d, NU, h.c.ts, h.c.tv)
        } else if above.pol > 1.0 - 1e-6 {
            threshold_one_kind(d, NU / 2.0, h.c.a_s(), h.c.a_v())
        } else {
            f64::NAN
        };
        if one_kind.is_finite() {
            dev_one_kind = dev_one_kind.max((g / one_kind - 1.0).abs());
        }
        // the cutoff below which this strength stays below the critical coupling of the vacuum (most attractive of T_S, D_S)
        let most = h.c.ts.min(h.c.ds) * g;
        let gc1 = Cjt::critical_coupling(d, NU, 1.0);
        let cut = if most < 0.0 { (gc1 / most).powf(1.0 / (d as f64 - 1.0)) } else { f64::INFINITY };
        rows.push(vec![d.to_string(), SETS[is].0.to_string(), form.tag().to_string(), format!("{:.10e}", g), format!("{:.10e}", kf_a), format!("{:.6}", above.pol), kind_of(above.pol).to_string(), format!("{:.10e}", above.mass[0]), format!("{:.10e}", above.mass[1]), format!("{:.10e}", one_kind), format!("{:.10e}", most), format!("{:.10e}", gc1), format!("{:.10e}", cut)]);
        f2_txt.push(format!("D = {}, set {}, form {}: g m^{} = {:.4} ({}; k_F,total = {:.4}, M_+ = {:.4}, M_- = {:.4})", d, SETS[is].0, form.tag(), d - 1, g, kind_of(above.pol), kf_a, above.mass[0], above.mass[1]));
    }
    let ratio_txt: Vec<String> = dims.iter().flat_map(|&d| (0..SETS.len()).map(move |is| (d, is))).map(|(d, is)| format!("D = {}, set {}: {:.3}", d, SETS[is].0, thr_of(d, is, Form::C) / thr_of(d, is, Form::Q))).collect();
    write_csv_str("forms_thresholds.csv",
        &[("content", "the 4+4 model in its two forms, fields that do not depend on x5, x6, x7 (D = 4), or on x0 either (D = 3): the smallest strength g = |g_0| of the couplings of the Lagrangian at which uniform matter is self-bound (its energy per particle lies below m at some density; no-sea approximation; state of lowest energy with respect to the polarization of the two halves), in units m = 1, and the state 0.1 per cent above that strength. On the deflating spacetime the strength grows as exp(tau_d): the threshold is reached at a definite clock reading".to_string()),
          ("columns", "D; set: A = g_0 = -g alone, B = g_0 = -g and g_1 = +0.1 g; form: C = covariant (the definition), Q = quantized-sector (the alternative); g_threshold: g m^(D-1); kF_total: Fermi momentum that the density of the bound state would have with equal halves; pol: polarization (n_+ - n_-)/n; kind: equal halves, one half, or unequal halves; M_plus, M_minus: masses of the particles of the two halves (for an empty half: the mass that a particle of that half would have); g_one_kind: the threshold computed for matter of one kind of particle by the routines of module kohn_sham (eight states with T_S, T_V for equal halves; four states with a_S, a_V for one half); G_most: the more attractive of the couplings T_S, D_S at the threshold; Gc: critical coupling of the vacuum for a cutoff equal to m; cutoff_max: the cutoff (in units of m) below which G_most stays below the critical coupling".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "g_threshold", "kF_total", "pol", "kind", "M_plus", "M_minus", "g_one_kind", "G_most", "Gc", "cutoff_max"], &rows)?;
    rep.add("fm-F2", "numerical", &format!("the binding thresholds of the two forms for the same couplings of the Lagrangian (fields that do not depend on x5, x6, x7; no-sea approximation; state of lowest energy with respect to the polarization): the smallest strength g = |g_0| at which matter is self-bound, verified in each case (bound 0.1 per cent above, not bound 0.1 per cent below): {}. Where the bound state has equal halves or one half filled, the threshold equals the one computed for matter of one kind of particle by the routines of module kohn_sham (largest relative difference {:.1e}). Ratio of the threshold of the covariant form to that of the quantized-sector form: {}", f2_txt.join("; "), dev_one_kind, ratio_txt.join("; ")), f2 && dev_one_kind < 1e-6);
    println!("[{:>6.1} s] thresholds done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 3. the energy as a function of the polarization
    // =====================================================================================
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut f3 = true;
    let mut f3_pol: Vec<String> = Vec::new();
    let mut n_c_pol = 0usize;
    let mut n_c_all = 0usize;
    let mut n_q_pol = 0usize;
    let mut f3_ex = String::new();
    for &d in &dims {
        for is in 0..SETS.len() {
            let g_q = thr_of(d, is, Form::Q);
            for mult in [1.5, 3.0, 6.0] {
                let mut line: Vec<String> = Vec::new();
                for kf_tot in [0.4, 0.6, 0.8, 1.0] {
                    let mut e_low = [f64::NAN; 2];
                    let mut p_low = [f64::NAN; 2];
                    for (iform, form) in Form::both().iter().enumerate() {
                        let h = halves_of(&tabs, d, *form, is, mult * g_q);
                        let n = h.density_of_kf_total(kf_tot);
                        let e0 = h.at_polarization(n, 0.0).map(|s| s.e_per_particle);
                        for i in 0..=40 {
                            let pol = 0.025 * i as f64;
                            match h.at_polarization(n, pol) {
                                Some(s) => rows.push(vec![d as f64, is as f64, form.code(), mult, kf_tot, n, pol, s.mass[0], s.mass[1], s.eps, s.e_per_particle - 1.0, s.e_per_particle - e0.unwrap_or(f64::NAN), s.n_solutions as f64]),
                                None => f3 = false,
                            }
                        }
                        match h.lowest_at_density(n) {
                            Some(s) => {
                                e_low[iform] = s.e_per_particle - 1.0;
                                p_low[iform] = s.pol;
                            }
                            None => f3 = false,
                        }
                    }
                    n_c_all += 1;
                    n_c_pol += usize::from(p_low[0] > 1e-6);
                    n_q_pol += usize::from(p_low[1] > 1e-6);
                    line.push(format!("{:.2}", p_low[0]));
                    if (d, is, mult, kf_tot) == (4, 1, 3.0, 0.8) {
                        f3_ex = format!("D = 4, set B, three times the threshold of the quantized-sector form, total density of the Fermi momentum 0.8 m: energy per particle minus m = {:+.4} m in the covariant form (polarization {:.2}) and {:+.4} m in the quantized-sector form (polarization {:.2})", e_low[0], p_low[0], e_low[1], p_low[1]);
                    }
                }
                f3_pol.push(format!("D = {}, set {}, {} times: {}", d, SETS[is].0, mult, line.join(", ")));
            }
        }
    }
    write_csv("forms_energy_vs_polarization.csv",
        &[("content", "the 4+4 model in its two forms: energy of uniform matter of a given total density as a function of the polarization pol = (n_+ - n_-)/n of the two halves W = +1 and W = -1 of the field (four internal states each); at each polarization the solution of the two gap equations of lowest energy. Strength of the couplings: mult times the strength at which the quantized-sector form first binds matter (file forms_thresholds.csv), the same couplings of the Lagrangian for both forms".to_string()),
          ("columns", "D; set: 0 = A (g_0 = -g alone), 1 = B (g_0 = -g, g_1 = +0.1 g); form: 0 = C (covariant, the definition), 1 = Q (quantized-sector, the alternative); mult; kF_total: Fermi momentum that the total density would have with equal halves; n: total density; pol; M_plus, M_minus: masses of the particles of the two halves; eps: energy density; e_bind: energy per particle minus m; e_rel: energy per particle minus its value at pol = 0; n_solutions: number of solutions of the gap equations with non-negative masses (units m = 1)".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "mult", "kF_total", "n", "pol", "M_plus", "M_minus", "eps", "e_bind", "e_rel", "n_solutions"], &rows)?;
    rep.add("fm-F3", "numerical", &format!("the polarization of the two halves. For the same couplings of the Lagrangian (sets A and B, attractive scalar coupling g_0 < 0), 1.5, 3 and 6 times the strength at which the quantized-sector form first binds matter, and total densities of the Fermi momenta 0.4, 0.6, 0.8, 1.0 (48 cases): in the covariant form a particle is attracted by the particles of its own half and repelled by those of the other, and the state of lowest energy is polarized in {} of the {} cases; in the quantized-sector form it is polarized in {} cases. Polarization (n_+ - n_-)/n of the state of lowest energy in the covariant form (four densities in the order given): {}. Example: {}", n_c_pol, n_c_all, n_q_pol, f3_pol.join("; "), f3_ex), f3);
    println!("[{:>6.1} s] polarization done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 4. saturated matter along the deflation clock
    // =====================================================================================
    let n_tau = 121usize;
    let sat: Vec<Vec<Vec<f64>>> = par_map(cases.len(), |i| {
        let (d, is, form) = cases[i];
        let mut out: Vec<Vec<f64>> = Vec::with_capacity(n_tau);
        for j in 0..n_tau {
            let tau = 0.05 * j as f64;
            let g = thr[i] * tau.exp();
            let h = halves_of(&tabs, d, form, is, g);
            let (kf, s) = lowest(&h);
            out.push(vec![d as f64, is as f64, form.code(), tau, g, kf, s.n_total(), s.pol, s.mass[0], s.mass[1], s.e_per_particle - 1.0, s.p_flat, s.p_int, s.eps, s.mu[0], s.mu[1]]);
        }
        out
    });
    let mut f4 = true;
    let mut f4_txt: Vec<String> = Vec::new();
    let mut p_max: f64 = 0.0;
    for (i, &(d, is, form)) in cases.iter().enumerate() {
        let rws = &sat[i];
        // the energy per particle falls monotonically along the clock; the pressure vanishes at saturation
        let mut mono = true;
        for j in 1..n_tau {
            mono &= rws[j][10] <= rws[j - 1][10] + 1e-10;
            if j >= 2 {
                p_max = p_max.max(rws[j][11].abs() / rws[j][13].abs());
            }
        }
        f4 &= mono;
        let at = |tau: f64| &rws[(tau / 0.05).round() as usize];
        let kinds: Vec<&str> = [0.5, 1.0, 2.0, 4.0].iter().map(|&t| kind_of(at(t)[7])).collect();
        f4_txt.push(format!("D = {}, set {}, form {}: E/N - m = {:+.4} m at tau_d = 1 and {:+.4} m at tau_d = 3 (mass of the particles of the filled half or halves {:.3} m and {:.3} m); state at tau_d = 0.5, 1, 2, 4: {}", d, SETS[is].0, form.tag(), at(1.0)[10], at(3.0)[10], at(1.0)[8], at(3.0)[8], kinds.join(", ")));
    }
    let sat_rows: Vec<Vec<f64>> = sat.into_iter().flatten().collect();
    write_csv("forms_saturation.csv",
        &[("content", "the 4+4 model in its two forms: self-bound matter along the deflation clock. At each clock reading tau_d the couplings of the reduced Lagrangian are g = g_threshold exp(tau_d) (g_threshold: the strength at which matter of that form, set and dimension first binds, file forms_thresholds.csv), and the row gives the state of lowest energy per particle over all densities and polarizations (the saturation point; no-sea approximation)".to_string()),
          ("columns", "D; set: 0 = A, 1 = B; form: 0 = C (covariant, the definition), 1 = Q (quantized-sector, the alternative); tau_d; g = |g_0| m^(D-1); kF_total: Fermi momentum that the density would have with equal halves; n: total density; pol: polarization (n_+ - n_-)/n; M_plus, M_minus: masses of the particles of the two halves; e_bind: energy per particle minus m; p: pressure (zero at an interior minimum); Pi: interaction pressure, the pressure along the deflating coordinates; eps: energy density; mu_plus, mu_minus: chemical potentials of the two halves (for an empty half the energy of its lowest state)".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "tau_d", "g", "kF_total", "n", "pol", "M_plus", "M_minus", "e_bind", "p", "Pi", "eps", "mu_plus", "mu_minus"], &sat_rows)?;
    rep.add("fm-F4", "numerical", &format!("self-bound matter along the deflation clock in the two forms (clock counted from the moment at which matter of the respective form first binds; 121 clock readings from 0 to 6 in each of the 8 cases): the energy per particle of the saturated state falls monotonically, and its pressure vanishes (largest |p|/eps after the first two readings: {:.1e}). {}", p_max, f4_txt.join("; ")), f4 && p_max < 1e-6);
    println!("[{:>6.1} s] saturation along the clock done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 5. matter of fixed density while the couplings grow
    // =====================================================================================
    let kfs = [0.3, 0.6];
    let fixed: Vec<(Vec<Vec<f64>>, Vec<Vec<f64>>)> = par_map(cases.len(), |i| {
        let (d, is, form) = cases[i];
        let mut rows: Vec<Vec<f64>> = Vec::new();
        let mut events: Vec<Vec<f64>> = Vec::new();
        for &kf_tot in &kfs {
            let mut prev: Option<(f64, HState)> = None;
            let (mut tau_pol, mut tau_p0, mut tau_full) = (f64::NAN, f64::NAN, f64::NAN);
            for j in 0..=160 {
                let tau = -3.0 + 0.05 * j as f64;
                let g = thr[i] * tau.exp();
                let h = halves_of(&tabs, d, form, is, g);
                let Some(s) = h.lowest_at_density(h.density_of_kf_total(kf_tot)) else { continue };
                if let Some((tp, sp)) = prev {
                    if sp.pol <= 1e-6 && s.pol > 1e-6 && tau_pol.is_nan() {
                        tau_pol = 0.5 * (tp + tau);
                    }
                    if sp.pol < 1.0 - 1e-6 && s.pol >= 1.0 - 1e-6 && tau_full.is_nan() {
                        tau_full = 0.5 * (tp + tau);
                    }
                    if sp.p_flat > 0.0 && s.p_flat <= 0.0 && tau_p0.is_nan() {
                        tau_p0 = tp + (tau - tp) * sp.p_flat / (sp.p_flat - s.p_flat);
                    }
                }
                rows.push(vec![d as f64, is as f64, form.code(), kf_tot, tau, g, s.pol, s.mass[0], s.mass[1], s.eps, s.p_flat, s.p_int, s.e_per_particle - 1.0]);
                prev = Some((tau, s));
            }
            events.push(vec![d as f64, is as f64, form.code(), kf_tot, tau_pol, tau_full, tau_p0, thr[i] * tau_p0.exp()]);
        }
        (rows, events)
    });
    let mut fix_rows: Vec<Vec<f64>> = Vec::new();
    let mut ev_rows: Vec<Vec<f64>> = Vec::new();
    for (r, e) in fixed {
        fix_rows.extend(r);
        ev_rows.extend(e);
    }
    write_csv("forms_fixed_density.csv",
        &[("content", "the 4+4 model in its two forms: matter of a fixed total density (fixed number of particles per unit of the volume of the coordinates in which they move) while the couplings grow with the deflation clock, g = g_threshold exp(tau_d); at each clock reading the state of lowest energy with respect to the polarization of the two halves".to_string()),
          ("columns", "D; set: 0 = A, 1 = B; form: 0 = C, 1 = Q; kF_total: Fermi momentum of the density with equal halves; tau_d: clock reading counted from the binding threshold of the same form, set and dimension; g; pol; M_plus, M_minus; eps: energy density; p: pressure along the coordinates in which the particles move; Pi: interaction pressure (the pressure along the deflating coordinates); e_bind: energy per particle minus m".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "kF_total", "tau_d", "g", "pol", "M_plus", "M_minus", "eps", "p", "Pi", "e_bind"], &fix_rows)?;
    write_csv("forms_gas_events.csv",
        &[("content", "the events in the history of matter of fixed density of file forms_fixed_density.csv: the clock readings (counted from the binding threshold of the same form, set and dimension) at which the two halves first become unequal, at which one half is empty, and at which the pressure along the coordinates of the motion becomes negative. NaN: the event does not happen for -3 <= tau_d <= 5".to_string()),
          ("columns", "D; set: 0 = A, 1 = B; form: 0 = C, 1 = Q; kF_total; tau_polarized: the halves become unequal (resolution 0.05); tau_one_half: one half is empty (resolution 0.05); tau_p0: the pressure vanishes; g_p0: the strength g at that moment".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "kF_total", "tau_polarized", "tau_one_half", "tau_p0", "g_p0"], &ev_rows)?;
    let ev_txt: Vec<String> = ev_rows.iter().filter(|r| r[0] == 4.0).map(|r| format!("set {}, form {}, k_F,total = {}: halves unequal from tau_d = {:.2}, pressure negative from tau_d = {:.2}", SETS[r[1] as usize].0, if r[2] == 0.0 { "C" } else { "Q" }, r[3], r[4], r[6])).collect();
    let f5 = ev_rows.iter().all(|r| r[6].is_finite()) && ev_rows.iter().filter(|r| r[2] == 0.0).all(|r| r[4].is_finite());
    rep.add("fm-F5", "numerical", &format!("matter of fixed density while the couplings grow with the deflation clock (clock counted from the binding threshold of the respective form; total densities of the Fermi momenta 0.3 m and 0.6 m; D = 4 and D = 3; sets A and B). In the covariant form the matter first polarizes (the two halves become unequal) and its pressure along the coordinates of the motion becomes negative later or at the same time; in the quantized-sector form the pressure becomes negative while the halves are still equal wherever \"nan\" stands for the polarization below. For D = 4: {}", ev_txt.join("; ")), f5);
    println!("[{:>6.1} s] fixed density done", t0.elapsed().as_secs_f64());

    // the rest of the command is in the second part of this file
    let ok_rest = run_part2(&tabs, &mut rep, &cases, &thr, t0)?;

    let (ok, n) = rep.count();
    let all = rep.all_pass() && ok_rest;
    write_report(&rep, "forms_checks.txt", "forms_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: checks of the calculations for the two forms of the self-interaction on the 4+4 spacetime, C = covariant form (the definition) and Q = quantized-sector form (the alternative): uniform matter, effective potential, vacuum, Vorton-Scalar (fm-..)", CMD)?;
    for c in &rep.checks {
        println!("[{}] {:<8} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nforms: {} of {} checks passed", ok, n);
    Ok(all)
}

