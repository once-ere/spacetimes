//! Second part of the command `forms` (see cmd_forms.rs): the effective potential of
//! Cornwall, Jackiw and Tomboulis for the two halves, the vacuum with the filled states of
//! negative energy, the functional with the Vorton-Scalar, and couplings of the opposite sign.

use crate::cmd_forms::{couplings9, halves_of, kind_of, par_map, CMD, SETS};
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks_c::cjt_contact::{simpson, Cjt};
use spacetimes_deflation_cjt_ks_c::halves::{CjtHalves, Form, HalvesCouplings, Hf4, UnifiedHalves};
use spacetimes_deflation_cjt_ks_c::io::{write_csv, write_csv_str};
use spacetimes_deflation_cjt_ks_c::kohn_sham::{global_min, sphere_area, Fermi, Unified, NU, PI};
use spacetimes_deflation_cjt_ks_c::selfint::SelfTables;

/// NU Int_{|k| < lam} d^Dk/(2 pi)^D k^2/(k^2 + M^2)^(3/2): minus the derivative of the scalar
/// density of the filled states of negative energy (all NU states) with respect to the mass.
fn sea_slope(d: usize, lam: f64, mass: f64) -> f64 {
    let pref = NU * sphere_area(d) / (2.0 * PI).powi(d as i32);
    pref * simpson(|k| if k == 0.0 { 0.0 } else { k.powi(d as i32 + 1) / (k * k + mass * mass).powf(1.5) }, 0.0, lam, 4000)
}

/// The smallest C at which matter of the functional with the Vorton-Scalar is self-bound,
/// for the fraction f_d of bare mass and the contact couplings g_0 = -kappa0 C, g_1 = +kappa1 C
/// (bisection in ln C on [0.05, 60]).  Returns (C, kf_total, state just above the threshold).
fn unified_threshold(tabs: &SelfTables, form: Form, f_d: f64, kappa0: f64, kappa1: f64) -> Option<(f64, f64, spacetimes_deflation_cjt_ks_c::halves::UnifiedHalvesState)> {
    let functional = |c: f64| {
        let mut g9 = [0.0; 9];
        g9[0] = -kappa0 * c;
        g9[1] = kappa1 * c;
        UnifiedHalves { form, f_d, c, hf: Hf4::from_tables(tabs, &g9) }
    };
    let lowest = |c: f64| functional(c).lowest_energy_per_particle(0.05, 8.0, 24);
    // bound: a first look at 13 densities decides most cases; the full search follows otherwise
    let bound = |c: f64| {
        let f = functional(c);
        for i in 0..=12 {
            let kf = (0.25f64.ln() + (4.0f64.ln() - 0.25f64.ln()) * i as f64 / 12.0).exp();
            if f.state(UnifiedHalves::density_of_kf_total(kf)).map_or(false, |s| s.e_per_particle - 1.0 < -1e-9) {
                return true;
            }
        }
        lowest(c).map_or(false, |(_, s)| s.e_per_particle - 1.0 < -1e-9)
    };
    let (mut lo, mut hi) = (0.05f64.ln(), 60.0f64.ln());
    if bound(lo.exp()) || !bound(hi.exp()) {
        return None;
    }
    for _ in 0..20 {
        let mid = 0.5 * (lo + hi);
        if bound(mid.exp()) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let c = (0.5 * (lo + hi)).exp();
    lowest(1.001 * c).map(|(kf, s)| (c, kf, s))
}

pub(crate) fn run_part2(tabs: &SelfTables, rep: &mut Report, cases: &[(usize, usize, Form)], thr: &[f64], t0: std::time::Instant) -> std::io::Result<bool> {
    // =====================================================================================
    // 6. the effective potential (CJT) of two halves against the Kohn-Sham state
    // =====================================================================================
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut f6 = true;
    let (mut dev, mut res_max, mut dmu_max): (f64, f64, f64) = (0.0, 0.0, 0.0);
    let mut counts = [0usize; 3]; // equal halves, unequal halves, one half
    let mut n_empty_stable = 0usize;
    let mut missing: Vec<String> = Vec::new();
    for (i, &(d, is, form)) in cases.iter().enumerate() {
        for mult in [1.5, 3.0] {
            let h = halves_of(tabs, d, form, is, mult * thr[i]);
            let cj = CjtHalves { d, nu_half: NU / 2.0, c: h.c, sea: None };
            for kf_tot in [0.3, 0.5, 0.7, 0.9, 1.1] {
                let Some(st) = h.lowest_at_density(h.density_of_kf_total(kf_tot)) else {
                    f6 = false;
                    continue;
                };
                let mu = st.mu[0];
                let kind = if st.pol < 1e-6 {
                    0
                } else if st.pol > 1.0 - 1e-6 {
                    2
                } else {
                    1
                };
                counts[kind] += 1;
                if kind == 2 {
                    // an empty half stays empty if its lowest state lies above the chemical potential
                    n_empty_stable += usize::from(st.mu[1] >= mu);
                } else {
                    dmu_max = dmu_max.max((st.mu[1] / mu - 1.0).abs());
                }
                let start = [st.mass[0] * 1.003, st.mass[1] * 0.998];
                let Some(found) = cj.stationary_near(mu, &start, &st.v) else {
                    missing.push(format!("D = {}, set {}, form {}, {} times, k_F = {}", d, SETS[is].0, form.tag(), mult, kf_tot));
                    continue;
                };
                let scale = st.eps.abs();
                let dv = (found.mass[0] - st.mass[0]).abs().max((found.mass[1] - st.mass[1]).abs()) + ((found.n[0] + found.n[1]) / st.n_total() - 1.0).abs() + (found.eps - st.eps).abs() / scale + (-found.omega - st.p_flat).abs() / scale;
                dev = dev.max(dv);
                res_max = res_max.max(found.residual);
                rows.push(vec![d as f64, is as f64, form.code(), mult, kf_tot, mu, st.pol, st.mass[0], found.mass[0], st.mass[1], found.mass[1], st.n_total(), found.n[0] + found.n[1], st.eps, found.eps, st.p_flat, -found.omega, found.residual]);
            }
        }
    }
    write_csv("forms_cjt_vs_ks.csv",
        &[("content", "the 4+4 model in its two forms: the Kohn-Sham method and the effective potential of Cornwall, Jackiw and Tomboulis compared for the two halves of the field (no-sea approximation). For each Kohn-Sham state (state of lowest energy at a given total density; closed forms of the momentum integrals; solution of the coupled gap equations) the chemical potential is computed and passed to the effective potential Omega(M_+, M_-, V_+, V_-; mu) (Simpson integrals), whose stationary point is searched by Newton's method from displaced start values. Strength of the couplings: mult times the binding threshold of the same form, set and dimension".to_string()),
          ("columns", "D; set: 0 = A, 1 = B; form: 0 = C, 1 = Q; mult; kF_total; mu: chemical potential; pol: polarization of the Kohn-Sham state; Mp_ks, Mp_cjt: mass of the particles of the half W = +1 by the two methods; Mm_ks, Mm_cjt: the same for the half W = -1; n_ks, n_cjt: total density; eps_ks, eps_cjt = Omega + mu n: energy density; p_ks: pressure; minus_Omega; residual: largest residual of the four stationarity conditions (units m = 1)".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "mult", "kF_total", "mu", "pol", "Mp_ks", "Mp_cjt", "Mm_ks", "Mm_cjt", "n_ks", "n_cjt", "eps_ks", "eps_cjt", "p_ks", "minus_Omega", "residual"], &rows)?;
    let n_states = rows.len();
    rep.add("fm-F6", "numerical", &format!("the Kohn-Sham method and the CJT effective potential agree for the two halves, in both forms: for {} states (two dimensions, two sets, two forms, strengths 1.5 and 3 times the respective binding threshold, total densities of the Fermi momenta 0.3 to 1.1 m; {} with equal halves, {} with unequal halves, {} with one half filled) the stationary point of Omega(M_+, M_-, V_+, V_-; mu) at the chemical potential of the Kohn-Sham state, found by Newton's method from displaced start values, has the same two masses, the same density and energy density, and -Omega equals the Kohn-Sham pressure (sum of the deviations at most {:.1e}; residuals of the four stationarity conditions at most {:.1e}). In the states with unequal halves the two chemical potentials agree to {:.1e} (relative): the polarization is the equilibrium one. In all {} states with one half filled the lowest state of the empty half lies above the chemical potential{}", n_states, counts[0], counts[1], counts[2], dev, res_max, dmu_max, n_empty_stable, if missing.is_empty() { String::new() } else { format!("; NOT FOUND: {}", missing.join(", ")) }), f6 && missing.is_empty() && dev < 1e-6 && res_max < 1e-10 && dmu_max < 1e-8 && n_empty_stable == counts[2] && n_states == 80);
    println!("[{:>6.1} s] CJT against Kohn-Sham done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 7. the vacuum with the filled states of negative energy
    // =====================================================================================
    let lam = 1.0;
    let mut f7 = true;
    let mut f7_txt: Vec<String> = Vec::new();
    // (a) a massless field: just below and just above the critical strength
    let mut crit_rows: Vec<Vec<String>> = Vec::new();
    let mut vac_cases: Vec<(usize, usize, Form, f64, f64)> = Vec::new(); // (d, set, form, bare mass, critical strength of the massless field)
    for &d in &[4usize, 3] {
        let gc = Cjt::critical_coupling(d, NU, lam);
        for is in 0..SETS.len() {
            let hf1 = Hf4::from_tables(tabs, &couplings9(is, 1.0));
            // the more attractive scalar coupling per unit strength is G_S^HF for both sets
            let g_crit = gc / hf1.gs_hf;
            f7 &= hf1.gs_hf < 0.0 && hf1.gs_c > hf1.gs_hf;
            for form in Form::both() {
                let vac = |g: f64, bare: f64| CjtHalves { d, nu_half: NU / 2.0, c: HalvesCouplings::new(form, bare, &hf1.scaled(g)), sea: Some(lam) }.vacuum();
                let below = vac(0.98 * g_crit, 0.0);
                let above = vac(1.02 * g_crit, 0.0);
                let (mb, ma) = (below[0].0, above[0].0);
                f7 &= mb[0].abs() < 1e-5 && mb[1].abs() < 1e-5 && ma[0].abs() > 1e-2;
                f7 &= match form {
                    Form::C => (ma[0] + ma[1]).abs() < 1e-7,
                    Form::Q => (ma[0] - ma[1]).abs() < 1e-7,
                };
                crit_rows.push(vec![d.to_string(), SETS[is].0.to_string(), form.tag().to_string(), format!("{:.10e}", gc), format!("{:.10e}", g_crit), format!("{:.6e}", mb[0]), format!("{:.6e}", mb[1]), format!("{:.10e}", ma[0]), format!("{:.10e}", ma[1]), above.len().to_string()]);
                for bare in [0.0, 0.05] {
                    vac_cases.push((d, is, form, bare, g_crit));
                }
            }
        }
        f7_txt.push(format!("D = {}: G_c Lambda^{} = {:.6}", d, d - 1, gc));
    }
    // (b) along the clock, tau_d = ln(g/g_crit), for a massless field and for the bare mass 0.05 Lambda
    let n_tau = 101usize;
    let vac_rows_nested: Vec<Vec<Vec<f64>>> = par_map(vac_cases.len(), |i| {
        let (d, is, form, bare, g_crit) = vac_cases[i];
        let hf1 = Hf4::from_tables(tabs, &couplings9(is, 1.0));
        (0..n_tau)
            .map(|j| {
                let tau = -1.0 + 0.02 * j as f64;
                let g = g_crit * tau.exp();
                let v = CjtHalves { d, nu_half: NU / 2.0, c: HalvesCouplings::new(form, bare, &hf1.scaled(g)), sea: Some(lam) }.vacuum();
                let (ms, om) = v[0];
                // by the symmetry between the halves the solutions come in pairs (M_+, M_-) and (M_-, M_+): the larger mass first
                let (hi, lo) = if ms[0] >= ms[1] { (ms[0], ms[1]) } else { (ms[1], ms[0]) };
                vec![d as f64, is as f64, form.code(), bare, tau, g, hi, lo, om, v.len() as f64]
            })
            .collect()
    });
    // (c) with a bare mass the covariant form has a sharp transition: the strength at which the two masses separate
    let mut tc_rows: Vec<String> = Vec::new();
    let mut dev_lin: f64 = 0.0;
    for (i, &(d, is, form, bare, g_crit)) in vac_cases.iter().enumerate() {
        if bare == 0.0 {
            continue;
        }
        let rws = &vac_rows_nested[i];
        let split = |r: &Vec<f64>| (r[6] - r[7]).abs() > 1e-6;
        match form {
            Form::Q => f7 &= rws.iter().all(|r| !split(r)),
            Form::C => {
                let hf1 = Hf4::from_tables(tabs, &couplings9(is, 1.0));
                let Some(j) = rws.iter().position(split) else {
                    f7 = false;
                    continue;
                };
                // refine by bisection in ln g between the readings j - 1 and j
                let separated = |ln_g: f64| {
                    let v = CjtHalves { d, nu_half: NU / 2.0, c: HalvesCouplings::new(Form::C, bare, &hf1.scaled(ln_g.exp())), sea: Some(lam) }.vacuum();
                    (v[0].0[0] - v[0].0[1]).abs() > 1e-6
                };
                let (mut lo, mut hi) = (rws[j - 1][5].ln(), rws[j][5].ln());
                for _ in 0..30 {
                    let mid = 0.5 * (lo + hi);
                    if separated(mid) {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                let g_split = (0.5 * (lo + hi)).exp();
                // the linear criterion: 1 + D_S NU Int k^2/w^3 = 0 at the common mass Mbar of the two halves,
                // Mbar = m_bare - T_S NU Int Mbar/w
                let lin = |ln_g: f64| {
                    let g = ln_g.exp();
                    let c = HalvesCouplings::new(Form::C, bare, &hf1.scaled(g));
                    let one = Cjt { d, nu: NU, c: spacetimes_deflation_cjt_ks_c::kohn_sham::Contact { m: bare, gs: 0.0, gv: 0.0 }, sea: Some(lam) };
                    // common mass: fixed point of M = m_bare + T_S s(M), s(M) = -NU Int M/w  (T_S > 0 here: a contraction for these strengths)
                    let mut mbar = bare;
                    for _ in 0..200 {
                        mbar = bare + c.ts * one.sea_rho_s(mbar);
                    }
                    1.0 + c.ds * sea_slope(d, lam, mbar)
                };
                let (mut a, mut b) = ((0.5 * g_crit).ln(), (4.0 * g_crit).ln());
                if lin(a) * lin(b) < 0.0 {
                    for _ in 0..50 {
                        let mid = 0.5 * (a + b);
                        if lin(a) * lin(mid) <= 0.0 {
                            b = mid;
                        } else {
                            a = mid;
                        }
                    }
                    let g_lin = (0.5 * (a + b)).exp();
                    dev_lin = dev_lin.max((g_split / g_lin - 1.0).abs());
                    tc_rows.push(format!("D = {}, set {}: tau_d = {:.4} (linear criterion {:.4})", d, SETS[is].0, (g_split / g_crit).ln(), (g_lin / g_crit).ln()));
                } else {
                    f7 = false;
                }
            }
        }
    }
    let vac_rows: Vec<Vec<f64>> = vac_rows_nested.into_iter().flatten().collect();
    write_csv_str("forms_vacuum_critical.csv",
        &[("content", "the 4+4 model in its two forms: the vacuum of a field without mass term with the filled states of negative energy (cutoff Lambda = 1: all quantities in units of the cutoff), 2 per cent below and 2 per cent above the critical strength g_crit = G_c/(G_S^HF per unit strength), where G_c = -(2 pi)^D (D - 1)/(NU A_D Lambda^(D-1)) is the critical coupling. Above it the field acquires a mass by its own self-interaction: in the quantized-sector form the same mass in both halves (the mass term psibar psi of the positive Lagrangian), in the covariant form masses of opposite sign in the two halves (the mass term psitilde psi of the covariant Lagrangian)".to_string()),
          ("columns", "D; set; form; Gc: G_c Lambda^(D-1); g_crit; Mp_below, Mm_below: masses of the two halves at 0.98 g_crit; Mp_above, Mm_above: at 1.02 g_crit (the solution of lowest Omega); n_solutions: number of solutions of the two gap equations of the vacuum at 1.02 g_crit".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "Gc", "g_crit", "Mp_below", "Mm_below", "Mp_above", "Mm_above", "n_solutions"], &crit_rows)?;
    write_csv("forms_vacuum.csv",
        &[("content", "the 4+4 model in its two forms: the masses of the two halves of the field in vacuum with the filled states of negative energy (cutoff Lambda = 1), along the deflation clock counted from the moment at which the strength of the couplings reaches the critical strength of a massless field (file forms_vacuum_critical.csv), for a field without mass term and for the bare mass 0.05 Lambda. The row gives the solution of the two gap equations of the vacuum with the lowest Omega_vac = sum over the halves of -(NU/2) Int^Lambda [w(M) - w(m)] - muS^2/(2 T_S) - dS^2/(2 D_S)".to_string()),
          ("columns", "D; set: 0 = A, 1 = B; form: 0 = C (covariant), 1 = Q (quantized-sector); m_bare; tau_d = ln(g/g_crit); g; M_large, M_small: the two masses, the larger first (by the symmetry between the halves the assignment to W = +1 and W = -1 is free; a negative value is a mass term of the opposite sign); Omega: Omega_vac; n_solutions: number of solutions of the two gap equations".to_string()),
          ("command", CMD.to_string())],
        &["D", "set", "form", "m_bare", "tau_d", "g", "M_large", "M_small", "Omega", "n_solutions"], &vac_rows)?;
    rep.add("fm-F7", "numerical", &format!("the vacuum of the two forms with the filled states of negative energy (cutoff Lambda). The gap equations of the vacuum are (M_+ + M_-)/2 - m = T_S (s_+ + s_-) and (M_+ - M_-)/2 = D_S (s_+ - s_-) with s_+- = -(NU/2) Int^Lambda M_+-/w. (a) Without mass term (m = 0) and for an attractive scalar coupling (sets A and B: G_S^HF < 0 < G_Sc) a field acquires a mass when |G_S^HF| exceeds the critical coupling |G_c| ({}), in both forms at the same strength of the couplings: no mass 2 per cent below it, a mass 2 per cent above it. In the quantized-sector form the two halves acquire the SAME mass (M_+ = M_-: the mass term of the positive Lagrangian); in the covariant form they acquire masses of OPPOSITE sign (M_+ = -M_-: the mass term -m psitilde psi of the covariant Lagrangian), verified to 1e-7. (b) With the bare mass 0.05 Lambda the quantized-sector form passes smoothly through the critical strength (the two masses stay equal at all 101 clock readings), whereas the covariant form has a sharp transition at which the two masses separate: {}; the strength of the transition agrees with the linear criterion 1 + D_S NU Int^Lambda d^Dk/(2 pi)^D k^2/w^3 = 0 at the common mass to {:.1e}", f7_txt.join(", "), tc_rows.join("; "), dev_lin), f7 && dev_lin < 1e-4 && tc_rows.len() == 4);
    println!("[{:>6.1} s] vacuum done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 8. bare mass, self-interaction and Vorton-Scalar together (D = 3)
    // =====================================================================================
    // entries: (f_D, kappa0, kappa1); contact couplings g_0 = -kappa0 C, g_1 = +kappa1 C
    let mut entries: Vec<(Form, f64, f64, f64)> = Vec::new();
    for f_d in [0.0, 0.25, 0.5] {
        for (k0, k1) in [(0.0, 0.0), (1.0, 0.0), (1.0, 0.1)] {
            for form in Form::both() {
                entries.push((form, f_d, k0, k1));
            }
        }
    }
    let uni = par_map(entries.len(), |i| {
        let (form, f_d, k0, k1) = entries[i];
        unified_threshold(tabs, form, f_d, k0, k1)
    });
    let c_main = spacetimes::cjt_normal::critical_c(false);
    let mut f8 = true;
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut dev_one: f64 = 0.0;
    let mut f8_txt: Vec<String> = Vec::new();
    let f3 = Fermi::isotropic(3, NU, 96);
    let c0: Vec<f64> = entries.iter().zip(&uni).filter(|(e, _)| (e.1, e.2, e.3) == (0.0, 0.0, 0.0)).map(|(_, u)| u.map_or(f64::NAN, |x| x.0)).collect();
    f8 &= c0.len() == 2 && (c0[0] / c_main - 1.0).abs() < 1e-4 && (c0[1] / c_main - 1.0).abs() < 1e-4;
    for (e, u) in entries.iter().zip(&uni) {
        let (form, f_d, k0, k1) = *e;
        let Some((c, kf, st)) = u else {
            f8 = false;
            rows.push(vec![format!("{}", f_d), format!("{}", k0), format!("{}", k1), form.tag().to_string(), "NaN".to_string(), "NaN".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string(), "".to_string()]);
            continue;
        };
        let dtau = (c / c0[0]).ln();
        // matter of one kind with the same couplings (module kohn_sham), where the state has equal halves
        let mut g9 = [0.0; 9];
        g9[0] = -k0;
        g9[1] = k1;
        let hf1 = Hf4::from_tables(tabs, &g9);
        let hc = HalvesCouplings::new(form, 1.0, &hf1);
        let mut one_kind = f64::NAN;
        if st.halves.pol < 1e-6 {
            let lowest = |cc: f64| global_min(|l: f64| Unified { f_d, c: cc, gs: hc.ts * cc, gv: hc.tv * cc }.state(&f3, l.exp()).e_per_particle - 1.0, 0.05f64.ln(), 8.0f64.ln(), 120, 1e-9).1;
            let (mut lo, mut hi) = (0.05f64.ln(), 60.0f64.ln());
            if lowest(lo.exp()) >= -1e-9 && lowest(hi.exp()) < -1e-9 {
                for _ in 0..20 {
                    let mid = 0.5 * (lo + hi);
                    if lowest(mid.exp()) < -1e-9 {
                        hi = mid;
                    } else {
                        lo = mid;
                    }
                }
                one_kind = (0.5 * (lo + hi)).exp();
                dev_one = dev_one.max((c / one_kind - 1.0).abs());
            }
        }
        rows.push(vec![format!("{}", f_d), format!("{}", k0), format!("{}", k1), form.tag().to_string(), format!("{:.8e}", c), format!("{:.6}", dtau), format!("{:.6}", kf), format!("{:.6}", st.x), format!("{:.6}", st.halves.pol), format!("{:.6}", st.halves.mass[0]), format!("{:.6}", st.halves.mass[1]), format!("{:.8e}", one_kind)]);
        if k0 > 0.0 {
            f8_txt.push(format!("f_D = {}, kappa_0 = {}, kappa_1 = {}, form {}: C = {:.4} (shift {:+.3}; {}, x = {:.3})", f_d, k0, k1, form.tag(), c, dtau, kind_of(st.halves.pol), st.x));
        }
    }
    write_csv_str("forms_unified_thresholds.csv",
        &[("content", format!("the 4+4 model in its two forms: bare Dirac mass, self-interaction and Vorton-Scalar together, ordinary space (D = 3, no-sea approximation): the smallest coupling C = g_VS^2 m^2/m_s^2 of the scalar at which uniform matter is self-bound, when a fraction f_D of the vacuum mass of the fermions is a bare Dirac mass and the couplings of the self-interaction are g_0 = -kappa_0 C, g_1 = +kappa_1 C (all other couplings zero; units m = 1). The Vorton-Scalar shifts the masses of both halves alike; the self-interaction acts between the halves with the sign of its form. Without bare mass and self-interaction the value is {:.6} (= 3 pi^2/8). All couplings grow as exp(tau_d), so that dtau is the shift of the clock reading at which matter becomes self-bound", c_main)),
          ("columns", "f_D = m_D/m; kappa_0 = -g_0/C; kappa_1 = g_1/C; form: C = covariant (the definition), Q = quantized-sector (the alternative); C_bind; dtau = ln(C_bind/C_bind(0, 0, 0)): negative = earlier; kF_total, x = phi/v_VS, pol, M_plus, M_minus: the state 0.1 per cent above the threshold; C_one_kind: the threshold computed with the functional of module kohn_sham for matter of one kind with the couplings T_S, T_V (given where the state has equal halves)".to_string()),
          ("command", CMD.to_string())],
        &["f_D", "kappa_0", "kappa_1", "form", "C_bind", "dtau", "kF_total", "x", "pol", "M_plus", "M_minus", "C_one_kind"], &rows)?;
    rep.add("fm-F8", "numerical", &format!("bare Dirac mass, self-interaction and Vorton-Scalar together, in the two forms (D = 3, no-sea approximation): the smallest C at which matter is self-bound, for f_D = 0, 0.25, 0.5 and the contact couplings g_0 = -kappa_0 C, g_1 = +kappa_1 C. Without self-interaction both forms give the threshold of the model with the Vorton-Scalar alone, C = {:.5} and {:.5} against 3 pi^2/8 = {:.5}. Where the bound state has equal halves, the threshold equals the one of the functional for matter of one kind (module kohn_sham) with the couplings T_S, T_V of the form (largest relative difference {:.1e}). With self-interaction: {}", c0[0], c0[1], c_main, dev_one, f8_txt.join("; ")), f8 && dev_one < 2e-4);
    println!("[{:>6.1} s] functional with the Vorton-Scalar done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 9. scalar coupling of the opposite sign
    // =====================================================================================
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut f9 = true;
    let mut f9_txt: Vec<String> = Vec::new();
    for (d, g) in [(4usize, 100.0), (3usize, 15.0)] {
        let mut g9 = [0.0; 9];
        g9[0] = g;
        let hf = Hf4::from_tables(tabs, &g9);
        for form in Form::both() {
            let h = spacetimes_deflation_cjt_ks_c::halves::Halves { d, nu_half: NU / 2.0, c: HalvesCouplings::new(form, 1.0, &hf) };
            let mut es: Vec<f64> = Vec::new();
            for kf_tot in [0.25, 0.5, 1.0, 2.0, 4.0, 8.0] {
                match h.lowest_at_density(h.density_of_kf_total(kf_tot)) {
                    Some(s) => {
                        rows.push(vec![d as f64, form.code(), g, kf_tot, s.n_total(), s.pol, s.mass[0], s.mass[1], s.e_per_particle - 1.0, h.c.tv, h.c.ts, h.c.dv, h.c.ds]);
                        es.push(s.e_per_particle - 1.0);
                    }
                    None => f9 = false,
                }
            }
            if es.len() == 6 {
                match form {
                    // covariant form: the energy per particle falls without bound at high density
                    Form::C => f9 &= es[5] < es[4] && es[4] < es[3] && es[5] < -1.0 && h.c.tv < 0.0,
                    // quantized-sector form: the energy per particle rises
                    Form::Q => f9 &= es[5] > es[4] && es[4] > es[3] && es[3] > 0.0,
                }
                f9_txt.push(format!("D = {}, g_0 = +{} m^-{}, form {}: E/N - m = {:+.3}, {:+.3}, {:+.3}, {:+.3} m at k_F,total = 1, 2, 4, 8 m", d, g, d - 1, form.tag(), es[2], es[3], es[4], es[5]));
            }
        }
    }
    write_csv("forms_reversed_sign.csv",
        &[("content", "the 4+4 model in its two forms for a scalar coupling of the opposite sign, g_0 = +g with all other couplings zero: energy per particle of uniform matter (state of lowest energy with respect to the polarization; no-sea approximation) at six total densities. In the covariant form particles of different halves then attract each other, the halves stay equal, and the coupling T_V = G_Vc = -g/16 of the square of the total density is negative: the energy per particle falls without bound as the density grows (the contact model has no state of lowest energy; it needs a cutoff). In the quantized-sector form the scalar coupling repels and the energy per particle rises".to_string()),
          ("columns", "D; form: 0 = C, 1 = Q; g = g_0 m^(D-1); kF_total; n: total density; pol; M_plus, M_minus; e_bind: energy per particle minus m; T_V, T_S, D_V, D_S: the four mean-field couplings of the form (units m = 1)".to_string()),
          ("command", CMD.to_string())],
        &["D", "form", "g", "kF_total", "n", "pol", "M_plus", "M_minus", "e_bind", "T_V", "T_S", "D_V", "D_S"], &rows)?;
    rep.add("fm-F9", "numerical", &format!("a scalar coupling of the opposite sign, g_0 > 0 alone. In the covariant form particles of different halves attract each other; the coupling of the square of the total density is T_V = G_Vc = -g_0/16 < 0, and the energy per particle of uniform matter falls without bound as the density grows: the contact model then has no saturated state without a cutoff. In the quantized-sector form the same coupling repels (T_S = 15 g_0/16 > 0) and the energy per particle rises with the density. {}", f9_txt.join("; ")), f9);
    println!("[{:>6.1} s] opposite sign done", t0.elapsed().as_secs_f64());
    Ok(true)
}
