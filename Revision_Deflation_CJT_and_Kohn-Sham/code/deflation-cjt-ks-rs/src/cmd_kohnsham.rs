//! Command `kohnsham` of the program `deflation_cjt_ks` (see main.rs): uniform matter of
//! the Dirac field with mass term and self-interaction, by the Kohn-Sham method and by the
//! effective potential of Cornwall, Jackiw and Tomboulis, on flat spacetime and on the
//! deflating spacetimes (modules `kohn_sham`, `cjt_contact`, `selfint`).
//!
//! Writes data/ks_flat_reference.csv, ks_flat_eos.csv, ks_deflating_71.csv,
//! ks_reduced_thresholds.csv, ks_reduced_gas.csv, ks_reduced_saturation.csv, ks_reduced_fixed_density.csv,
//! ks_44c_polarization.csv, cjt_vs_ks.csv, cjt_sea_vacuum.csv, cjt_sea_saturation.csv,
//! unified_thresholds.csv, unified_states.csv, unified_phase_mode.csv, kohnsham_checks.{txt,csv}.

use spacetimes::gamma::Gammas;
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks::cjt_contact::Cjt;
use spacetimes_deflation_cjt_ks::io::{write_csv, write_csv_str, write_report};
use spacetimes_deflation_cjt_ks::kohn_sham::{chemical_potential, fermi_density, global_min, golden_min, ks_solve, ks_state_at_mass, saturation, sound_speed_sq, Contact, CovariantCouplings, Fermi, KsState, TwoHalves, Unified, NU, PI};
use spacetimes_deflation_cjt_ks::selfint::SelfTables;

const CMD: &str = "deflation_cjt_ks kohnsham";
const ONE: [f64; 3] = [1.0; 3];

/// The Fermi momentum of the author's example at its saturation point, as printed in his document.
pub const KF_AUTHOR: f64 = 0.756636150701493;

/// Lowest energy per particle minus m over the Fermi momenta in [0.02, 20] (scan in
/// ln k_F with refinement around every local minimum).  Returns (k_F, E/A - m).
fn lowest_energy_per_particle(f: &Fermi, c: &Contact) -> (f64, f64) {
    let (l, e) = global_min(|l: f64| ks_solve(f, c, l.exp(), &ONE).e_per_particle - c.m, 0.02f64.ln(), 20.0f64.ln(), 240, 1e-11);
    (l.exp(), e)
}

/// The smallest scalar attraction |G_S| at which matter is self-bound, for a fixed ratio
/// G_V/|G_S| (bisection in ln|G_S|).
fn binding_threshold(f: &Fermi, ratio: f64) -> f64 {
    let bound = |ln_g: f64| {
        let g = ln_g.exp();
        lowest_energy_per_particle(f, &Contact { m: 1.0, gs: -g, gv: ratio * g }).1 < -1e-10
    };
    let (mut lo, mut hi) = (1e-3f64.ln(), 1e4f64.ln());
    assert!(!bound(lo) && bound(hi), "binding_threshold: no threshold in the search interval");
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if bound(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

/// Energy density of matter with the filled states of negative energy (cutoff of `cjt`),
/// at fixed Fermi momentum: minimum over the mass M of
/// e_kin + sea energy + (1/2) G_V rho^2 - (M - m)^2/(2 G_S).  Returns (M, energy density).
fn state_with_sea(f: &Fermi, cjt: &Cjt, kf: f64) -> (f64, f64) {
    let c = cjt.c;
    let e = |mass: f64| {
        let st = ks_state_at_mass(f, &c, kf, &ONE, mass);
        st.eps + cjt.sea_energy(mass)
    };
    let hi = 3.0 * cjt.vacuum_mass().max(c.m) + 1.0;
    let n = 300;
    let mut best = (f64::INFINITY, 0.0);
    for i in 0..=n {
        let x = hi * i as f64 / n as f64;
        let v = e(x);
        if v < best.0 {
            best = (v, x);
        }
    }
    let h = hi / n as f64;
    let m = golden_min(e, (best.1 - h).max(0.0), best.1 + h, 1e-10);
    (m, e(m))
}

fn fmt(x: f64) -> String {
    format!("{:.12e}", x)
}

pub fn run() -> std::io::Result<bool> {
    let t0 = std::time::Instant::now();
    let gm = Gammas::build();
    let tabs = SelfTables::build(&gm);
    let mut rep = Report::new();

    // =====================================================================================
    // 1. flat spacetime, seven space dimensions: the example of the author's document
    // =====================================================================================
    let f7 = Fermi::isotropic(7, NU, 96);
    let hartree = Contact { m: 1.0, gs: -1.0e5, gv: 1.0e4 };
    let st = ks_solve(&f7, &hartree, KF_AUTHOR, &ONE);
    let (ksat, sat) = saturation(&f7, &hartree, 0.3, 1.3);
    let reference: [(&str, f64, f64); 5] = [
        ("mass M of the particles in matter (units of m)", st.mass, 0.348097340146),
        ("density rho", st.rho, 1.388100213e-5),
        ("energy density eps", st.eps, 1.348787233e-5),
        ("energy per particle minus m", st.e_per_particle - 1.0, -0.02832142796),
        ("Fermi momentum at saturation", ksat, KF_AUTHOR),
    ];
    let mut worst: f64 = 0.0;
    let mut rows: Vec<Vec<String>> = Vec::new();
    for (name, here, doc) in reference {
        let rel = (here / doc - 1.0).abs();
        worst = worst.max(rel);
        rows.push(vec![name.to_string(), fmt(here), fmt(doc), format!("{:.2e}", rel)]);
    }
    write_csv_str("ks_flat_reference.csv",
        &[("content", format!("the Kohn-Sham state of uniform matter on flat 7+1 spacetime for the example of the author's document KOHN_SHAM_GRAVITY (m = 1, G_S = -1e5, G_V = +1e4, Fermi momentum {} in all seven space dimensions), computed by this program and compared with the numbers printed in that document (check ks-K1)", KF_AUTHOR)),
          ("columns", "quantity; here: value of this program; document: value printed in the author's document; rel_diff: relative difference".to_string()),
          ("command", CMD.to_string())],
        &["quantity", "here", "document", "rel_diff"], &rows)?;
    rep.add("ks-K1", "numerical", &format!("flat 7+1 spacetime, the example of the author's document (m = 1, G_S = -1e5, G_V = +1e4, k_F = {} in seven space dimensions): this program gives M = {:.12}, rho = {:.9e}, eps = {:.9e}, E/A - m = {:.11}, and finds the saturation point (the minimum of the energy per particle, where the pressure vanishes: p = {:.1e}) at k_F = {:.12}; the document prints 0.348097340146, 1.388100213e-5, 1.348787233e-5, -0.02832142796 and {}. Largest relative difference {:.1e}", KF_AUTHOR, st.mass, st.rho, st.eps, st.e_per_particle - 1.0, sat.p_flat, ksat, KF_AUTHOR, worst), worst < 2e-9);

    // the same couplings with the exchange terms: G_0 and G_1 from the nine couplings
    let mut g9 = [0.0; 9];
    g9[0] = -1.0e5;
    g9[1] = 1.0e4;
    let hf = Contact { m: 1.0, gs: tabs.s71.hf_coupling(1, &g9), gv: tabs.s71.hf_coupling(0, &g9) };
    let (khf, e_hf) = lowest_energy_per_particle(&f7, &hf);
    let sets: [(&str, Contact); 2] = [("hartree", hartree), ("hartree-fock", hf)];
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut thermo: f64 = 0.0;
    for (i, (_, c)) in sets.iter().enumerate() {
        for j in 0..=140 {
            let kf = 0.10 + 0.01 * j as f64;
            let s = ks_solve(&f7, c, kf, &ONE);
            let mu = chemical_potential(c, kf, &s);
            // thermodynamic consistency: the pressure from the momentum integral equals mu rho - eps
            thermo = thermo.max(((mu * s.rho - s.eps) - s.p_flat).abs() / s.eps);
            rows.push(vec![i as f64, kf, s.rho, s.mass, s.eps, s.p_flat, s.e_per_particle - c.m, mu, sound_speed_sq(&f7, c, kf)]);
        }
    }
    write_csv("ks_flat_eos.csv",
        &[("content", format!("equation of state of uniform matter on flat 7+1 spacetime (seven space dimensions, no-sea approximation) for two sets of the mean-field couplings: set 0 = G_S = -1e5, G_V = +1e4, the values that the Kohn-Sham section of the author's document KOHN_SHAM_GRAVITY uses; set 1 = G_S = {}, G_V = {}, the Hartree-Fock couplings G = (1 - F^T) g that the couplings g_0 = -1e5, g_1 = +1e4 of the Lagrangian give (module selfint; the same two numbers are printed in section 4 of the author's document)", hf.gs, hf.gv)),
          ("columns", "set; kF: Fermi momentum; rho: density; M: mass of the particles in matter; eps: energy density; p: pressure; e_bind: energy per particle minus m; mu: chemical potential; cs2: square of the speed of sound dp/d eps (units m = 1)".to_string()),
          ("command", CMD.to_string())],
        &["set", "kF", "rho", "M", "eps", "p", "e_bind", "mu", "cs2"], &rows)?;
    let mu_sat = chemical_potential(&hartree, ksat, &sat);
    rep.add("ks-K2", "numerical", &format!("thermodynamic consistency of the Kohn-Sham state (flat, seven space dimensions): the pressure computed from the momentum integral, <k^2/(7 w)> + Pi, equals mu rho - eps with mu = sqrt(k_F^2 + M^2) + G_V rho along the whole equation of state (141 Fermi momenta for each of the two sets; largest deviation {:.1e} of eps); at saturation the chemical potential equals the energy per particle (theorem of Hugenholtz and Van Hove): mu - E/A = {:.1e}. A remark on the author's example: his document uses G_S = -1e5, G_V = +1e4 in its Kohn-Sham section and calls them the Hartree-Fock couplings G_0, G_1, while the Hartree-Fock couplings of g_0 = -1e5, g_1 = +1e4 are G_0 = {}, G_1 = {} (his section 4 and check 71s-S7 here). For the second pair the lowest energy per particle is E/A - m = {:.6} at k_F = {:.4}: {}", thermo, mu_sat - sat.e_per_particle, hf.gs, hf.gv, e_hf, khf, if e_hf < -1e-9 { "matter is self-bound for these couplings as well" } else { "the doubled vector repulsion removes the binding, and uniform matter is not self-bound. The numbers of the author's Kohn-Sham section therefore belong to G_S = -1e5, G_V = +1e4 taken as the mean-field couplings themselves (as couplings of the Lagrangian with g_2 = .. = g_8 = 0 that would be g_0 = -105341.6, g_1 = +2484.5)" }), thermo < 1e-9 && (mu_sat - sat.e_per_particle).abs() < 1e-9);
    println!("[{:>5.1} s] flat spacetime done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 2. deflating 7+1 spacetime, momenta along all seven space coordinates
    // =====================================================================================
    let ani = Fermi::with_deflating(4, NU, 20, 40, 12);
    let ani_fine = Fermi::with_deflating(4, NU, 28, 56, 16);
    let frac_sets: [(&str, [f64; 3]); 2] = [("equal rates", [1.0 / 3.0; 3]), ("rates in the ratio 1 : 2 : 3", [1.0 / 6.0, 1.0 / 3.0, 0.5])];
    let s_of = |frac: &[f64; 3], tau: f64| [(frac[0] * tau).exp(), (frac[1] * tau).exp(), (frac[2] * tau).exp()];
    // K3: the ellipsoid at tau_d = 0 is the ball; refinement of the quadrature
    let a0 = ks_solve(&ani, &hartree, KF_AUTHOR, &ONE);
    let dev0 = (a0.mass - st.mass).abs().max((a0.eps / st.eps - 1.0).abs()).max((a0.rho / st.rho - 1.0).abs());
    let p_scale = ks_solve(&f7, &hartree, KF_AUTHOR, &ONE).e_kin; // the pressure vanishes at saturation: compare pressures on the scale of the kinetic energy density
    let dev0p = (a0.p_flat - st.p_flat).abs().max((a0.p_defl[0] - st.p_flat).abs()).max((a0.p_defl[2] - st.p_flat).abs()) / p_scale;
    let mut dev_ref: f64 = 0.0;
    for (_, frac) in &frac_sets {
        for tau in [1.5, 3.0, 4.0] {
            let s = s_of(frac, tau);
            let (c1, c2) = (ks_solve(&ani, &hartree, KF_AUTHOR, &s), ks_solve(&ani_fine, &hartree, KF_AUTHOR, &s));
            dev_ref = dev_ref.max((c1.mass - c2.mass).abs() / c2.mass).max((c1.eps / c2.eps - 1.0).abs()).max((c1.p_defl[2] / c2.p_defl[2] - 1.0).abs()).max((c1.p_flat / c2.p_flat - 1.0).abs());
        }
    }
    rep.add("ks-K3", "numerical", &format!("the momentum integral over the ellipsoid of occupied states (four flat and three deflating directions; {} nodes): at the reference time (all stretch factors 1) it reproduces the integral over the ball in seven dimensions, M, rho and eps to {:.1e} and the pressures along flat and deflating directions to {:.1e} of the kinetic energy density; with {} nodes instead, M, eps and the pressures change by at most {:.1e} (relative) at the clock readings 1.5, 3 and 4 for both sets of rates", ani.node_count(), dev0, dev0p, ani_fine.node_count(), dev_ref), dev0 < 1e-9 && dev0p < 1e-9 && dev_ref < 1e-6);

    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut cons: f64 = 0.0;
    let mut half_mass_tau = [f64::NAN; 2];
    let mut unbound_tau = [[f64::NAN; 2]; 2]; // [set][0: dilution, 1: compression]
    let mut heat_max: f64 = 0.0;
    let mut vector_share_end = [0.0; 2];
    for (i, (_, frac)) in frac_sets.iter().enumerate() {
        let mut prev: Option<(f64, KsState)> = None;
        for j in 0..=120 {
            let tau = -2.0 + 0.05 * j as f64;
            let s = s_of(frac, tau);
            let x = ks_solve(&ani, &hartree, KF_AUTHOR, &s);
            // the relaxed state: the ball of the same density, k_F' = k_F exp(tau_d/7)
            let relaxed = ks_solve(&f7, &hartree, KF_AUTHOR * (tau / 7.0).exp(), &ONE);
            heat_max = heat_max.max(x.e_per_particle - relaxed.e_per_particle);
            rows.push(vec![i as f64, tau, s[0], s[1], s[2], x.mass, x.rho, x.rho_s, x.eps, x.p_flat, x.p_defl[0], x.p_defl[1], x.p_defl[2], x.pi, x.e_per_particle - 1.0, relaxed.e_per_particle - 1.0, relaxed.mass]);
            if let Some((tp, xp)) = prev {
                if xp.mass > 0.5 * st.mass && x.mass <= 0.5 * st.mass {
                    half_mass_tau[i] = tp + (tau - tp) * (xp.mass - 0.5 * st.mass) / (xp.mass - x.mass);
                }
                let (ea, eb) = (xp.e_per_particle - 1.0, x.e_per_particle - 1.0);
                if ea * eb < 0.0 {
                    let tz = tp + (tau - tp) * ea / (ea - eb);
                    unbound_tau[i][if tz < 0.0 { 0 } else { 1 }] = tz;
                }
            }
            prev = Some((tau, x));
            if j == 120 {
                vector_share_end[i] = 0.5 * hartree.gv * x.rho * x.rho / x.eps;
            }
        }
        // K4: conservation of energy, d eps/d tau_d = sum_t frac_t (eps + p_t)
        for tau in [-1.0, 0.0, 0.7, 2.0, 3.5] {
            let h = 1e-4;
            let x = ks_solve(&ani, &hartree, KF_AUTHOR, &s_of(frac, tau));
            let d = (ks_solve(&ani, &hartree, KF_AUTHOR, &s_of(frac, tau + h)).eps - ks_solve(&ani, &hartree, KF_AUTHOR, &s_of(frac, tau - h)).eps) / (2.0 * h);
            let want: f64 = (0..3).map(|t| frac[t] * (x.eps + x.p_defl[t])).sum();
            cons = cons.max((d / want - 1.0).abs());
        }
    }
    write_csv("ks_deflating_71.csv",
        &[("content", format!("Kohn-Sham matter on the deflating 7+1 spacetime, momenta along all seven space coordinates, collisionless (every particle keeps its wave numbers): the matter is the saturated matter of the author's example at the reference time (m = 1, G_S = -1e5, G_V = +1e4, k_F = {}); set 0 = equal rates, set 1 = rates in the ratio 1 : 2 : 3 (g55 : g66 : g77)", KF_AUTHOR)),
          ("columns", "set; tau_d = gsum (x4 - x4_ref): deflation clock; s5, s6, s7: stretch factors exp(rate_t (x4 - x4_ref)) of the proper momenta along x5, x6, x7; M: mass of the particles in matter; rho: density (= rho_ref exp(tau_d)); rho_S: scalar density; eps: energy density; p_flat: pressure along each of x0..x3; p5, p6, p7: pressures along x5, x6, x7; Pi: interaction part of the pressure; e_bind: energy per particle minus m; e_bind_relaxed: the same for the state of lowest energy at the same density (a ball of occupied states); M_relaxed: its mass".to_string()),
          ("command", CMD.to_string())],
        &["set", "tau_d", "s5", "s6", "s7", "M", "rho", "rho_S", "eps", "p_flat", "p5", "p6", "p7", "Pi", "e_bind", "e_bind_relaxed", "M_relaxed"], &rows)?;
    rep.add("ks-K4", "numerical", &format!("conservation of energy on the deflating 7+1 spacetime: the Kohn-Sham energy density obeys d eps/dx4 = sum_t rate_t (eps + p_t) with the pressures p_t = <s_t^2 u_t^2/w> + Pi along the deflating coordinates (the covariant divergence of the energy-momentum tensor vanishes); numerical derivative against the right-hand side at the clock readings -1, 0, 0.7, 2 and 3.5, for equal rates and for rates in the ratio 1 : 2 : 3: largest relative deviation {:.1e}", cons), cons < 1e-6);
    rep.add("ks-K5", "numerical", &format!("history of saturated matter while x5, x6, x7 deflate (7+1, all momenta, collisionless; equal rates / rates 1 : 2 : 3): the density grows as exp(tau_d); the mass of the particles falls to half its value at the reference time at tau_d = {:.3} / {:.3}; the matter is self-bound (energy per particle below m) only for {:.3} < tau_d < {:.3} / {:.3} < tau_d < {:.3}; at tau_d = 4 the vector repulsion (1/2) G_V rho^2 is {:.1} / {:.1} per cent of the energy density, which then grows as exp(2 tau_d); the collisionless state lies above the state of lowest energy of the same density by at most {:.3e} m per particle on -2 <= tau_d <= 4 (energy that collisions would turn into heat)", half_mass_tau[0], half_mass_tau[1], unbound_tau[0][0], unbound_tau[0][1], unbound_tau[1][0], unbound_tau[1][1], 100.0 * vector_share_end[0], 100.0 * vector_share_end[1], heat_max),
        half_mass_tau.iter().all(|x| x.is_finite()) && unbound_tau.iter().all(|u| u[0] < 0.0 && u[1] > 0.0) && heat_max >= -1e-12);
    println!("[{:>5.1} s] deflating 7+1 (all momenta) done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 3. the sector of fields that do not depend on x5, x6, x7: D = 4 and D = 3
    // =====================================================================================
    let hf_ratio = hf.gv / -hf.gs;
    let ratios: [(&str, f64); 3] = [("scalar attraction alone (G_V = 0)", 0.0), ("G_V = 0.1 |G_S| (the ratio of the values G_S = -1e5, G_V = +1e4 of the author's document)", 0.1), ("G_V = 0.2025 |G_S| (the ratio of the Hartree-Fock couplings of g_0 = -1e5, g_1 = +1e4)", hf_ratio)];
    let mut thr_rows: Vec<Vec<String>> = Vec::new();
    let mut gas_rows: Vec<Vec<f64>> = Vec::new();
    let mut sat_rows: Vec<Vec<f64>> = Vec::new();
    let mut fix_rows: Vec<Vec<f64>> = Vec::new();
    let mut k6 = true;
    let mut k6_txt: Vec<String> = Vec::new();
    for d in [4usize, 3] {
        let f = Fermi::isotropic(d, NU, 96);
        for (ir, (rname, ratio)) in ratios.iter().enumerate() {
            let g_bind = binding_threshold(&f, *ratio);
            // just above the threshold the minimum is bound, just below it is not
            let above = lowest_energy_per_particle(&f, &Contact { m: 1.0, gs: -1.001 * g_bind, gv: 1.001 * ratio * g_bind });
            let below = lowest_energy_per_particle(&f, &Contact { m: 1.0, gs: -0.999 * g_bind, gv: 0.999 * ratio * g_bind });
            k6 &= above.1 < 0.0 && below.1 >= -1e-10;
            thr_rows.push(vec![d.to_string(), ir.to_string(), rname.to_string(), fmt(*ratio), fmt(g_bind), fmt(above.0), fmt(Cjt::critical_coupling(d, NU, 1.0))]);
            k6_txt.push(format!("D = {}, {}: |G_S| m^{} = {:.4}", d, rname, d - 1, g_bind));
            // saturation along the clock: tau_d = ln(|G_S|/|G_S at the threshold|)
            for j in 0..=120 {
                let tau = 0.05 * j as f64;
                let g = g_bind * tau.exp();
                let c = Contact { m: 1.0, gs: -g, gv: ratio * g };
                let (kf, e) = lowest_energy_per_particle(&f, &c);
                let s = ks_solve(&f, &c, kf, &ONE);
                sat_rows.push(vec![d as f64, ir as f64, tau, g, kf, s.rho, s.mass, e, s.p_flat, sound_speed_sq(&f, &c, kf)]);
            }
            // a gas of fixed density while the couplings grow
            for kf in [0.3, 0.6] {
                let mut prev: Option<(f64, f64)> = None;
                let mut tau_p0 = f64::NAN;
                for j in 0..=160 {
                    let tau = -3.0 + 0.05 * j as f64;
                    let g = g_bind * tau.exp();
                    let c = Contact { m: 1.0, gs: -g, gv: ratio * g };
                    let s = ks_solve(&f, &c, kf, &ONE);
                    if let Some((tp, pp)) = prev {
                        if pp > 0.0 && s.p_flat <= 0.0 && tau_p0.is_nan() {
                            tau_p0 = tp + (tau - tp) * pp / (pp - s.p_flat);
                        }
                    }
                    prev = Some((tau, s.p_flat));
                    fix_rows.push(vec![d as f64, ir as f64, kf, tau, g, s.mass, s.eps, s.p_flat, s.pi, s.e_per_particle - 1.0]);
                }
                gas_rows.push(vec![d as f64, ir as f64, *ratio, kf, g_bind * tau_p0.exp(), tau_p0]);
            }
        }
    }
    write_csv_str("ks_reduced_thresholds.csv",
        &[("content", "fields that do not depend on x5, x6, x7 (D = 4 space dimensions: the quantized sector of 4+4 with the quantized-sector form of the interaction, and the low-energy sector of 7+1; D = 3: fields that do not depend on the hidden coordinate x0 either). The couplings of the reduced Lagrangian grow as exp(tau_d). The smallest scalar attraction at which uniform matter is self-bound (energy per particle below m at some density), in units m = 1, the Fermi momentum of the bound state just above the threshold, and for comparison the critical coupling of the vacuum for a cutoff equal to m".to_string()),
          ("columns", "D; ratio_set: 0 = G_V = 0, 1 = G_V = 0.1 |G_S|, 2 = ratio 0.2025 of the Hartree-Fock couplings of g_1 = -0.1 g_0; description; ratio G_V/|G_S|; GS_abs: |G_S| m^(D-1) at the threshold; kF: Fermi momentum of the bound state; Gc: critical coupling G_c m^(D-1) of the vacuum for Lambda = m".to_string()),
          ("command", CMD.to_string())],
        &["D", "ratio_set", "description", "ratio", "GS_abs", "kF", "Gc"], &thr_rows)?;
    write_csv("ks_reduced_gas.csv",
        &[("content", "fields that do not depend on x5, x6, x7: a gas of fixed Fermi momentum while the couplings grow as exp(tau_d): the coupling and the clock reading (counted from the binding threshold of the same ratio set) at which the pressure of the gas becomes negative".to_string()),
          ("columns", "D; ratio_set: 0 = G_V = 0, 1 = G_V = 0.1 |G_S|, 2 = ratio 0.2025 of the Hartree-Fock couplings of g_1 = -0.1 g_0; ratio G_V/|G_S|; kF; GS_abs: |G_S| m^(D-1) at which p = 0; tau_d: clock reading relative to the binding threshold".to_string()),
          ("command", CMD.to_string())],
        &["D", "ratio_set", "ratio", "kF", "GS_abs", "tau_d"], &gas_rows)?;
    write_csv("ks_reduced_saturation.csv",
        &[("content", "self-bound matter in the sector of fields that do not depend on x5, x6, x7, along the deflation clock: at each clock reading the state of lowest energy per particle (the saturation point, pressure zero) for the couplings of that moment, G_S = -|G_S(threshold)| exp(tau_d), G_V = ratio |G_S|; tau_d is counted from the moment at which matter first becomes self-bound".to_string()),
          ("columns", "D: number of space dimensions with momentum (4 or 3); ratio_set: 0 = G_V = 0, 1 = G_V = 0.1 |G_S|, 2 = ratio 0.2025 of the Hartree-Fock couplings of g_1 = -0.1 g_0; tau_d; GS_abs = |G_S| m^(D-1); kF: Fermi momentum at saturation; rho; M: mass of the particles in matter; e_bind: energy per particle minus m; p: pressure (zero at an interior minimum); cs2: square of the speed of sound".to_string()),
          ("command", CMD.to_string())],
        &["D", "ratio_set", "tau_d", "GS_abs", "kF", "rho", "M", "e_bind", "p", "cs2"], &sat_rows)?;
    write_csv("ks_reduced_fixed_density.csv",
        &[("content", "a gas of fixed density (fixed Fermi momentum) in the sector of fields that do not depend on x5, x6, x7, while the couplings grow with the deflation clock; tau_d is counted from the binding threshold of the same ratio set".to_string()),
          ("columns", "D; ratio_set; kF; tau_d; GS_abs; M; eps; p: pressure along the flat directions; Pi: pressure along the deflating coordinates (interaction part alone); e_bind: energy per particle minus m".to_string()),
          ("command", CMD.to_string())],
        &["D", "ratio_set", "kF", "tau_d", "GS_abs", "M", "eps", "p", "Pi", "e_bind"], &fix_rows)?;
    rep.add("ks-K6", "numerical", &format!("fields that do not depend on x5, x6, x7: the reduced couplings grow as exp(tau_d), and uniform matter becomes self-bound when the scalar attraction reaches a threshold (verified: bound 0.1 per cent above it, not bound 0.1 per cent below it). Thresholds in units m = 1: {}. The deflation therefore turns a gas into self-bound matter at the clock reading tau_bind = ln(|G_S|(threshold)/|G_S|(reference))", k6_txt.join("; ")), k6);
    println!("[{:>5.1} s] reduced sectors done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 4. the covariant form of the interaction on 4+4: two halves
    // =====================================================================================
    let f4 = Fermi::isotropic(4, NU, 96);
    let f4_half = Fermi::isotropic(4, NU / 2.0, 96);
    // couplings: the author's ratio g_1 = -0.1 g_0, strength three times the binding threshold of the quantized-sector form (Hartree-Fock)
    let hf_of = |scale: f64| {
        let mut g = [0.0; 9];
        g[0] = -scale;
        g[1] = 0.1 * scale;
        g
    };
    // Hartree-Fock couplings per unit g_0: G_0/|g_0| and G_1/|g_0| (quantized-sector form)
    let unit = hf_of(1.0);
    let (g0u, g1u) = (tabs.s44q.hf_coupling(1, &unit), tabs.s44q.hf_coupling(0, &unit));
    let thr_q = binding_threshold(&f4, g1u / -g0u) / -g0u; // |g_0| at which the quantized-sector form binds
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut k8 = true;
    let mut k8_txt: Vec<String> = Vec::new();
    let mut k8_pol: Vec<String> = Vec::new();
    for mult in [1.5, 3.0, 6.0] {
        let g = hf_of(mult * thr_q);
        let cov = CovariantCouplings { m: 1.0, gv0: tabs.s44c.hf_coupling(0, &g), gs0: tabs.s44c.hf_coupling(1, &g), g1: tabs.s44c.hf_coupling(2, &g), g0: tabs.s44c.hf_coupling(3, &g) };
        let th = TwoHalves { fermi: &f4_half, c: cov };
        let qform = Contact { m: 1.0, gs: tabs.s44q.hf_coupling(1, &g), gv: tabs.s44q.hf_coupling(0, &g) };
        for kf_tot in [0.4, 0.6, 0.8, 1.0] {
            let rho = fermi_density(4, NU, kf_tot);
            let q = ks_solve(&f4, &qform, kf_tot, &ONE);
            let mut best = (f64::INFINITY, 0.0);
            let mut e_sym = f64::NAN;
            let mut e_pol = f64::NAN;
            for i in 0..=20 {
                let pol = 0.05 * i as f64;
                let s = th.at_polarization(rho, pol);
                k8 &= s.converged;
                if i == 0 {
                    e_sym = s.e_per_particle - 1.0;
                }
                if i == 20 {
                    e_pol = s.e_per_particle - 1.0;
                    // fully polarized: one kind of particle with NU/2 states and the couplings G_V0 + G_1, G_S0 + G_0
                    let single = ks_solve(&f4_half, &Contact { m: 1.0, gs: cov.gs0 + cov.g0, gv: cov.gv0 + cov.g1 }, s.kf[0], &ONE);
                    k8 &= (single.eps / s.eps - 1.0).abs() < 1e-9 && (single.mass - s.mass[0]).abs() < 1e-9;
                }
                if s.e_per_particle < best.0 {
                    best = (s.e_per_particle, pol);
                }
                rows.push(vec![mult, kf_tot, rho, pol, s.mass[0], s.mass[1], s.eps, s.e_per_particle - 1.0, q.e_per_particle - 1.0]);
            }
            if (mult, kf_tot) == (3.0, 0.8) {
                k8_txt.push(format!("for couplings three times the binding threshold of the quantized-sector form and total density of Fermi momentum 0.8: E/A - m = {:+.4} for equal halves, {:+.4} for all particles in one half, and {:+.4} in the quantized-sector form", e_sym, e_pol, q.e_per_particle - 1.0));
            }
            // the state with equal halves is not the state of lowest energy
            k8 &= best.1 > 0.0 && best.0 - 1.0 < e_sym - 1e-9;
            k8_pol.push(format!("{:.2}", best.1));
            let _ = e_pol;
        }
    }
    write_csv("ks_44c_polarization.csv",
        &[("content", format!("the covariant form of the self-interaction on 4+4 in the quantized sector (D = 4): the two halves W = +1 and W = -1 of the field as two kinds of particles with four states each. Couplings g_0 = -|g_0|, g_1 = 0.1 |g_0| with |g_0| = mult times {:.5} (the value at which the quantized-sector form with the same couplings first binds matter); Hartree-Fock couplings of module selfint (check 44c-S7)", thr_q)),
          ("columns", "mult: strength of the couplings; kF_total: Fermi momentum that the total density would have with all eight states filled equally; rho: total density; pol_W = (rho_+ - rho_-)/rho: polarization; M_plus, M_minus: masses of the particles of the two halves; eps: energy density; e_bind: energy per particle minus m; e_bind_q: the same density in the quantized-sector form".to_string()),
          ("command", CMD.to_string())],
        &["mult", "kF_total", "rho", "pol_W", "M_plus", "M_minus", "eps", "e_bind", "e_bind_q"], &rows)?;
    rep.add("ks-K8", "numerical", &format!("covariant form of the interaction on 4+4 (the two halves W = +1 and W = -1 carry opposite scalar and vector charge): for the attractive scalar coupling of the author's example (g_1 = -0.1 g_0) the state with equal halves is never the state of lowest energy; matter polarizes spontaneously. Polarization P = (rho_+ - rho_-)/rho of the state of lowest energy, for couplings 1.5, 3 and 6 times the binding threshold of the quantized-sector form and total densities of Fermi momentum 0.4, 0.6, 0.8, 1.0 (in this order): {} (grid of 0.05). The fully polarized state is the Kohn-Sham state of one kind of particle with four states and the couplings G_S0 + G_0, G_V0 + G_1 (verified to 1e-9). Example: {}", k8_pol.join(", "), k8_txt.join("; ")), k8);
    println!("[{:>5.1} s] covariant form done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 5. the CJT effective potential against the Kohn-Sham state (no sea)
    // =====================================================================================
    let f3 = Fermi::isotropic(3, NU, 96);
    let cases: [(usize, &Fermi, Contact); 5] = [
        (7, &f7, hartree),
        (7, &f7, hf),
        (4, &f4, Contact { m: 1.0, gs: -3.0 * binding_threshold(&f4, 0.1), gv: 0.3 * binding_threshold(&f4, 0.1) }),
        (3, &f3, Contact { m: 1.0, gs: -3.0 * binding_threshold(&f3, 0.1), gv: 0.3 * binding_threshold(&f3, 0.1) }),
        (3, &f3, Contact { m: 0.6, gs: -4.0, gv: 0.0 }),
    ];
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut j1: f64 = 0.0;
    let mut j1_res: f64 = 0.0;
    let mut n_global = 0usize;
    let mut n_states = 0usize;
    let mut n_denser = 0usize;
    let mut j1_missing: Vec<String> = Vec::new();
    for (ic, (d, f, c)) in cases.iter().enumerate() {
        for kf in [0.3, 0.5, 0.7, 0.9, 1.1, 1.4] {
            let ks = ks_solve(f, c, kf, &ONE);
            let mu = chemical_potential(c, kf, &ks);
            let cjt = Cjt { d: *d, nu: NU, c: *c, sea: None };
            // the stationary point of the effective potential nearest to the Kohn-Sham mass
            // (near a limit of metastability two stationary points lie close together: try brackets of several widths)
            let found = [1e-4, 1e-3, 1e-2, 5e-2].iter().find_map(|w| cjt.stationary_near(mu, ks.mass, w * ks.mass + 1e-10));
            let Some(local) = found else {
                j1_missing.push(format!("case {}, k_F = {}", ic, kf));
                continue;
            };
            n_states += 1;
            let scale = ks.e_kin;
            let dev = (local.mass - ks.mass).abs() / c.m.max(1e-300) + (local.rho / ks.rho - 1.0).abs() + (local.eps - ks.eps).abs() / scale + (-local.omega - ks.p_flat).abs() / scale;
            j1 = j1.max(dev);
            let res = cjt.residuals(&local);
            j1_res = j1_res.max(res.0.abs() / mu).max(res.1.abs() / c.m);
            // where the pressure is positive, uniform matter must also be the global minimum
            let mut global_mass = f64::NAN;
            if ks.p_flat > 1e-3 * scale {
                let glob = cjt.stationary(mu);
                global_mass = glob.mass;
                if (glob.mass - ks.mass).abs() < 1e-8 {
                    n_global += 1;
                } else if -glob.omega > ks.p_flat && glob.rho > ks.rho {
                    // another phase has the higher pressure at this chemical potential, and it is denser
                    n_denser += 1;
                }
            }
            rows.push(vec![ic as f64, *d as f64, c.m, c.gs, c.gv, kf, mu, ks.mass, local.mass, global_mass, ks.rho, local.rho, ks.eps, local.eps, ks.p_flat, -local.omega]);
        }
    }
    write_csv("cjt_vs_ks.csv",
        &[("content", "the two methods compared (no-sea approximation): for each Kohn-Sham state (fixed Fermi momentum; gap equation; Gauss-Legendre integrals) the chemical potential is computed and passed to the CJT effective potential Omega(M, V_0; mu) (Simpson integrals), whose stationary point is searched independently".to_string()),
          ("columns", "case; D; m; G_S; G_V; kF; mu; M_ks; M_cjt: stationary point of Omega nearest to M_ks; M_cjt_global: global minimum of Omega (computed where the pressure is positive; empty otherwise); rho_ks; rho_cjt; eps_ks; eps_cjt = Omega + mu rho; p_ks; minus_Omega".to_string()),
          ("command", CMD.to_string())],
        &["case", "D", "m", "G_S", "G_V", "kF", "mu", "M_ks", "M_cjt", "M_cjt_global", "rho_ks", "rho_cjt", "eps_ks", "eps_cjt", "p_ks", "minus_Omega"], &rows)?;
    let n_pos = rows.iter().filter(|r| !r[9].is_nan()).count();
    rep.add("cjt-J1", "numerical", &format!("the Kohn-Sham method and the CJT effective potential agree (no-sea approximation): for {} states (five sets of parameters in D = 7, 4, 3; Fermi momenta 0.3 to 1.4) the stationary point of Omega(M, V_0; mu) at the chemical potential of the Kohn-Sham state has the same mass, density and energy density, and -Omega equals the Kohn-Sham pressure (sum of the four relative deviations at most {:.1e}; residuals of the two stationarity conditions at most {:.1e}). Of the {} states with positive pressure, {} are the global minimum of Omega; in the other {} a denser phase has the higher pressure at the same chemical potential (the Kohn-Sham state of that density is then metastable: a dilute gas above saturated matter){}", n_states, j1, j1_res, n_pos, n_global, n_denser, if j1_missing.is_empty() { String::new() } else { format!("; NOT FOUND: {}", j1_missing.join(", ")) }), j1 < 1e-7 && j1_res < 1e-9 && n_states == 30 && n_global + n_denser == n_pos);
    println!("[{:>5.1} s] CJT against Kohn-Sham done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 6. the CJT effective potential with the Dirac sea
    // =====================================================================================
    // J2: the critical coupling: just above it a massless field acquires a mass, just below it does not
    let mut j2 = true;
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for d in [7usize, 4, 3] {
        let lam = 1.0;
        let gc = Cjt::critical_coupling(d, NU, lam);
        let closed = match d {
            7 => -90.0 * PI.powi(4),
            4 => -3.0 * PI * PI,
            _ => -PI * PI / 2.0,
        };
        j2 &= (gc / closed - 1.0).abs() < 1e-12;
        let m_below = Cjt { d, nu: NU, c: Contact { m: 0.0, gs: 0.98 * gc, gv: 0.0 }, sea: Some(lam) }.vacuum_mass();
        let m_above = Cjt { d, nu: NU, c: Contact { m: 0.0, gs: 1.02 * gc, gv: 0.0 }, sea: Some(lam) }.vacuum_mass();
        j2 &= m_below < 1e-4 && m_above > 1e-2;
        // the mass of the vacuum along the clock, for a massless field and for a bare mass of 5 per cent of the cutoff
        for bare in [0.0, 0.05] {
            for j in 0..=100 {
                let tau = -2.0 + 0.04 * j as f64;
                let gs = gc * tau.exp();
                let cj = Cjt { d, nu: NU, c: Contact { m: bare, gs, gv: 0.0 }, sea: Some(lam) };
                let mv = cj.vacuum_mass();
                rows.push(vec![d as f64, bare, tau, gs, mv, cj.sea_rho_s(mv)]);
            }
        }
    }
    write_csv("cjt_sea_vacuum.csv",
        &[("content", "the vacuum with the filled states of negative energy (cutoff Lambda = 1: all quantities in units of the cutoff): the mass of the fermions in vacuum, the minimum of Omega_vac(M) = -NU Int^Lambda d^Dk/(2 pi)^D [w(M) - w(m)] - (M - m)^2/(2 G_S), as a function of the scalar attraction. In the sector of fields that do not depend on x5, x6, x7 the attraction grows as exp(tau_d); tau_d is counted from the moment at which it reaches the critical coupling G_c".to_string()),
          ("columns", "D: space dimensions; m_bare: bare mass m in units of Lambda; tau_d = ln(G_S/G_c); G_S Lambda^(D-1); M_vac: mass in vacuum in units of Lambda; condensate: scalar density of the sea, <psibar psi> = -NU Int^Lambda M/w, in units of Lambda^D".to_string()),
          ("command", CMD.to_string())],
        &["D", "m_bare", "tau_d", "G_S", "M_vac", "condensate"], &rows)?;
    rep.add("cjt-J2", "numerical", "with the filled states of negative energy (cutoff Lambda) a massless field acquires a mass in vacuum when the scalar attraction exceeds the critical coupling G_c = -(2 pi)^D (D - 1)/(NU A_D Lambda^(D-1)) (A_D: area of the unit sphere): G_c Lambda^6 = -90 pi^4 for D = 7 (the value of the author's document DIRAC16COMPLEX), G_c Lambda^3 = -3 pi^2 for D = 4, G_c Lambda^2 = -pi^2/2 for D = 3; the minimization of the effective potential gives no mass 2 per cent below G_c and a mass 2 per cent above it, for all three D. On the deflating spacetime the reduced coupling crosses G_c at the clock reading tau_c = ln(G_c/G_S(reference))", j2);

    // J3: the author's example with the sea: the bare mass is chosen so that the mass in vacuum is 1
    let lam_max = (90.0 * PI.powi(4) / 1.0e5_f64).powf(1.0 / 6.0);
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut j3 = true;
    let mut j3_txt: Vec<String> = Vec::new();
    for lam in [0.0, 0.3, 0.45, 0.6] {
        let (gs, gv) = (-1.0e5, 1.0e4);
        let (cjt, m_bare) = if lam == 0.0 {
            (Cjt { d: 7, nu: NU, c: hartree, sea: None }, 1.0)
        } else {
            let mb = Cjt::bare_mass_for(7, NU, gs, lam, 1.0);
            (Cjt { d: 7, nu: NU, c: Contact { m: mb, gs, gv }, sea: Some(lam) }, mb)
        };
        let m_vac = cjt.vacuum_mass();
        j3 &= (m_vac - 1.0).abs() < 1e-7;
        // energy per particle relative to the vacuum, as a function of k_F
        let e_vac = cjt.sea_energy(m_vac) - if cjt.c.gs != 0.0 { (m_vac - cjt.c.m).powi(2) / (2.0 * cjt.c.gs) } else { 0.0 };
        let e_of = |kf: f64| {
            let (mass, eps) = state_with_sea(&f7, &cjt, kf);
            (mass, (eps - e_vac) / fermi_density(7, NU, kf) - m_vac)
        };
        let mut best = (f64::INFINITY, 0.0);
        for j in 0..=110 {
            let kf = 0.2 + 0.01 * j as f64;
            let (mass, e) = e_of(kf);
            rows.push(vec![lam, m_bare, kf, mass, e]);
            if e < best.0 {
                best = (e, kf);
            }
        }
        let kf = golden_min(|k| e_of(k).1, best.1 - 0.01, best.1 + 0.01, 1e-9);
        let (mass, e) = e_of(kf);
        j3_txt.push(if lam == 0.0 { format!("without sea: saturation at k_F = {:.4}, M = {:.4}, E/A - M_vac = {:.5}", kf, mass, e) } else { format!("Lambda = {}: bare mass {:.4}, saturation at k_F = {:.4}, M = {:.4}, E/A - M_vac = {:.5}", lam, m_bare, kf, mass, e) });
        if lam == 0.0 {
            j3 &= (kf - KF_AUTHOR).abs() < 1e-6 && (e + 0.02832142796).abs() < 1e-8;
        }
    }
    write_csv("cjt_sea_saturation.csv",
        &[("content", "the author's example (seven space dimensions, G_S = -1e5, G_V = +1e4) with the filled states of negative energy below the cutoff Lambda included (CJT effective potential, one-loop vacuum term); the bare mass m is chosen so that the mass of the fermions in vacuum is 1. Lambda = 0: no-sea approximation".to_string()),
          ("columns", "Lambda: cutoff in units of the vacuum mass; m_bare; kF; M: mass of the particles in matter; e_bind: energy per particle relative to the vacuum, minus the vacuum mass".to_string()),
          ("command", CMD.to_string())],
        &["Lambda", "m_bare", "kF", "M", "e_bind"], &rows)?;
    rep.add("cjt-J3", "numerical", &format!("the Dirac sea and the author's example. The coupling G_S = -1e5 m^-6 of the example exceeds the critical coupling 90 pi^4/Lambda^6 of the vacuum as soon as the cutoff is larger than Lambda = (90 pi^4/1e5)^(1/6) m = {:.4} m: the no-sea approximation presupposes a cutoff below this value. With the sea included (bare mass adjusted so that the vacuum mass stays 1): {}", lam_max, j3_txt.join("; ")), j3);
    println!("[{:>5.1} s] Dirac sea done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 7. contact interaction, bare mass and the Vorton-Scalar together (D = 3)
    // =====================================================================================
    // U1: without contact terms and bare mass the functional is that of the main project
    let mut u1: f64 = 0.0;
    for c in [4.0, 6.0, 10.0] {
        let med = spacetimes::cjt_normal::Medium { c, sea: false };
        let uni = Unified { f_d: 0.0, c, gs: 0.0, gv: 0.0 };
        for kf in [0.3, 0.6, 1.0] {
            let a = uni.state(&f3, kf);
            let x = med.mass(kf);
            u1 = u1.max((a.mass - x).abs()).max((a.eps - med.energy(kf, x)).abs() / a.eps);
        }
    }
    // binding threshold of the unified functional: the smallest C at which matter is self-bound,
    // for fixed f_D and fixed ratios kappa_S = -G_S/C, kappa_V = G_V/C (all three couplings grow as exp(tau_d))
    let lowest = |u: &Unified| -> (f64, f64) {
        let (l, e) = global_min(|l: f64| u.state(&f3, l.exp()).e_per_particle - 1.0, 0.02f64.ln(), 20.0f64.ln(), 120, 1e-9);
        (l.exp(), e)
    };
    let c_bind = |f_d: f64, ks_: f64, kv: f64| -> f64 {
        let bound = |c: f64| lowest(&Unified { f_d, c, gs: -ks_ * c, gv: kv * c }).1 < -1e-9;
        let (mut lo, mut hi) = (0.05, 60.0);
        if bound(lo) || !bound(hi) {
            return f64::NAN;
        }
        for _ in 0..26 {
            let mid = 0.5 * (lo + hi);
            if bound(mid) {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        0.5 * (lo + hi)
    };
    let c_main = spacetimes::cjt_normal::critical_c(false);
    let c0 = c_bind(0.0, 0.0, 0.0);
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for f_d in [0.0, 0.25, 0.5] {
        for ks_ in [0.0, 0.5, 1.0] {
            for kv in [0.0, 0.1, 0.3] {
                let cb = if (f_d, ks_, kv) == (0.0, 0.0, 0.0) { c0 } else { c_bind(f_d, ks_, kv) };
                rows.push(vec![f_d, ks_, kv, cb, (cb / c0).ln()]);
            }
        }
    }
    write_csv("unified_thresholds.csv",
        &[("content", format!("contact interaction, bare Dirac mass and Vorton-Scalar together, ordinary space (D = 3, no-sea approximation, branch connected to the vacuum): the smallest coupling C = g_VS^2 m^2/m_s^2 of the scalar at which uniform matter is self-bound, when a fraction f_D of the vacuum mass of the fermions is a bare Dirac mass and the contact couplings are G_S = -kappa_S C, G_V = kappa_V C (units m = 1). Without bare mass and contact terms the value is that of the main project, {:.6}. All three couplings grow as exp(tau_d), so that the last column is the shift of the clock reading at which matter becomes self-bound", c_main)),
          ("columns", "f_D = m_D/m; kappa_S = -G_S/C; kappa_V = G_V/C; C_bind; dtau = ln(C_bind/C_bind(0, 0, 0)): negative = earlier".to_string()),
          ("command", CMD.to_string())],
        &["f_D", "kappa_S", "kappa_V", "C_bind", "dtau"], &rows)?;
    rep.add("uni-U1", "numerical", &format!("contact interaction, bare mass and Vorton-Scalar together: for f_D = 0 and G_S = G_V = 0 the functional is the one of the main project (masses and energy densities for C = 4, 6, 10 and k_F = 0.3, 0.6, 1.0 agree to {:.1e}), and the smallest C at which matter is self-bound is {:.6} here against {:.6} there", u1, c0, c_main), u1 < 1e-6 && (c0 / c_main - 1.0).abs() < 1e-5);
    // U2: at low density the scalar acts as a contact coupling -C: M = 1 + (G_S - C) rho_S
    let mut u2: f64 = 0.0;
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for (f_d, c, gs, gv) in [(0.0, 5.0, 0.0, 0.0), (0.3, 5.0, 0.0, 0.0), (0.0, 5.0, -2.0, 0.5), (0.3, 5.0, -2.0, 0.5), (0.6, 2.0, -1.0, 0.2)] {
        let uni = Unified { f_d, c, gs, gv };
        for kf in [0.02, 0.05, 0.1, 0.2, 0.4, 0.7, 1.0] {
            let s = uni.state(&f3, kf);
            let slope = (s.mass - 1.0) / s.rho_s;
            if kf <= 0.05 {
                u2 = u2.max((slope / (gs - c) - 1.0).abs());
            }
            rows.push(vec![f_d, c, gs, gv, kf, s.x, s.mass, s.rho, s.rho_s, s.eps, s.e_per_particle - 1.0, slope]);
        }
    }
    write_csv("unified_states.csv",
        &[("content", "states of the functional with bare mass, contact couplings and Vorton-Scalar (D = 3, no-sea approximation, units m = 1): eps = (1 - f_D)^2 (x^2 - 1)^2/(8 C) + e_kin(k_F, M) + (1/2) G_V rho^2 - (M - f_D - (1 - f_D) x)^2/(2 G_S), minimized over x = phi/v_VS and M".to_string()),
          ("columns", "f_D; C; G_S; G_V; kF; x: expectation value of the scalar in units of its vacuum value; M: mass of the particles in matter; rho; rho_S; eps; e_bind: energy per particle minus m; slope = (M - 1)/rho_S, which tends to G_S - C at low density".to_string()),
          ("command", CMD.to_string())],
        &["f_D", "C", "G_S", "G_V", "kF", "x", "M", "rho", "rho_S", "eps", "e_bind", "slope"], &rows)?;
    rep.add("uni-U2", "numerical", &format!("at low density the Vorton-Scalar acts on the mass of the fermions exactly as a scalar contact coupling of strength -C = -g_VS^2/m_s^2 (units m = 1), whatever fraction of the mass is a bare mass: (M - 1)/rho_S tends to G_S - C (five sets of parameters; at k_F <= 0.05 the deviation is at most {:.1e}). The exchange of the radial mode of the scalar and the scalar channel g_0 of the self-interaction are the same interaction at momenta far below m_s", u2), u2 < 2e-2);
    // U3: a bare mass and the phase mode of the scalar.  With Phi = (v + s) exp(i theta)/sqrt(2) the mass term of the
    // fermions is m_D + m_Y exp(i theta chi), of magnitude m(theta) = sqrt(m_D^2 + m_Y^2 + 2 m_D m_Y cos(theta)).
    // The energy of the fermions (Fermi sea and filled states of negative energy) depends on theta through m(theta):
    // E''(0) = -(rho_S + rho_S^sea) m_D m_Y/m, and the phase mode has the mass m_ph^2 = E''(0)/v_VS^2.
    let mut u3: f64 = 0.0;
    let mut u3_edge: f64 = 0.0;
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut u3_txt: Vec<String> = Vec::new();
    for lam in [1.0, 2.0, 4.0] {
        let sea = Cjt { d: 3, nu: NU, c: Contact { m: 1.0, gs: 0.0, gv: 0.0 }, sea: Some(lam) };
        let cond = -sea.sea_rho_s(1.0); // |<psibar psi>| of the vacuum
        // closed form: (NU/(2 pi^2)) [ (Lambda sqrt(Lambda^2 + 1) - asinh(Lambda))/2 ]
        let closed = NU / (2.0 * PI * PI) * 0.5 * (lam * (lam * lam + 1.0).sqrt() - lam.asinh());
        u3 = u3.max((cond / closed - 1.0).abs());
        // In matter the mass of the phase mode falls: m_ph^2(k_F)/m_ph^2(0) = 1 - rho_S(k_F)/|<psibar psi>|.  The ratio
        // would vanish at k_F = Lambda (there the filled states of positive energy cancel those of negative energy
        // below the cutoff), which is the limit of the effective theory; the table stops at k_F = 0.75 Lambda.
        let ratio_half = 1.0 - f3.moments(0.5 * lam, 1.0, &ONE).rho_s / cond;
        let ratio_full = 1.0 - f3.moments(lam, 1.0, &ONE).rho_s / cond;
        u3_edge = u3_edge.max(ratio_full.abs());
        u3_txt.push(format!("Lambda = {} m: |<psibar psi>| = {:.5} m^3, ratio {:.4} at k_F = Lambda/2", lam, cond, ratio_half));
        for f_d in [0.1, 0.3] {
            let m_of = |th: f64| (f_d * f_d + (1.0 - f_d) * (1.0 - f_d) + 2.0 * f_d * (1.0 - f_d) * th.cos()).sqrt();
            for frac in [0.0, 0.25, 0.5, 0.75] {
                let kf = frac * lam;
                let e = |th: f64| {
                    let m = m_of(th);
                    (if kf > 0.0 { f3.moments(kf, m, &ONE).e_kin } else { 0.0 }) + sea.sea_energy(m)
                };
                let h = 2e-3;
                let d2 = (e(h) - 2.0 * e(0.0) + e(-h)) / (h * h);
                let rho_s = if kf > 0.0 { f3.moments(kf, 1.0, &ONE).rho_s } else { 0.0 };
                let formula = (cond - rho_s) * f_d * (1.0 - f_d);
                u3 = u3.max((d2 - formula).abs() / (cond * f_d * (1.0 - f_d)));
                // m_ph^2/(C m_s^2) = E''(0)/(1 - f_D)^2  (units m = 1)
                rows.push(vec![lam, f_d, kf, rho_s, cond, d2, formula, d2 / ((1.0 - f_d) * (1.0 - f_d)), 1.0 - rho_s / cond]);
            }
        }
    }
    write_csv("unified_phase_mode.csv",
        &[("content", "a bare Dirac mass and the phase mode of the Vorton-Scalar (ordinary space, D = 3; fermions of the Fermi sea and of the filled states of negative energy below the cutoff Lambda; units m = 1). The mass term of the fermions is m_D + m_Y exp(i theta chi) for the phase theta of the scalar, of magnitude m(theta); the second derivative of the energy density with respect to theta at theta = 0 is E'' = (|<psibar psi>| - rho_S) m_D m_Y/m, and the mass of the phase mode is m_ph^2 = E''/v_VS^2 = C m_s^2 E''/(1 - f_D)^2. The Fermi momenta are 0, 1/4, 1/2 and 3/4 of the cutoff (the effective theory ends at the cutoff)".to_string()),
          ("columns", "Lambda: cutoff; f_D = m_D/m; kF: Fermi momentum of the matter; rho_S: its scalar density; condensate = |<psibar psi>| of the vacuum; d2E: second derivative of the energy density, numerical; formula: (condensate - rho_S) f_D (1 - f_D); mph2_over_C_ms2: m_ph^2/(C m_s^2); ratio: m_ph^2 in matter divided by m_ph^2 in the vacuum = 1 - rho_S/condensate".to_string()),
          ("command", CMD.to_string())],
        &["Lambda", "f_D", "kF", "rho_S", "condensate", "d2E", "formula", "mph2_over_C_ms2", "ratio"], &rows)?;
    rep.add("uni-U3", "numerical", &format!("a bare Dirac mass gives the phase mode of the Vorton-Scalar a mass. The chiral symmetry U(1)_VS is then broken explicitly; for the phase theta of the scalar the fermions have the mass m(theta) = sqrt(m_D^2 + m_Y^2 + 2 m_D m_Y cos(theta)) (m_Y = g_VS v_VS), and the energy of the filled states gives m_ph^2 v_VS^2 = (|<psibar psi>| - rho_S) m_D m_Y/m, the relation of Gell-Mann, Oakes and Renner with the scalar density rho_S of the matter subtracted (numerical second derivative against this formula, 24 cases, and the closed form of the condensate for D = 3, (nu/(2 pi^2)) [Lambda sqrt(Lambda^2 + m^2) - m^2 asinh(Lambda/m)] m/2: largest deviation {:.1e} of the vacuum value). In units of the couplings of the model, m_ph^2 = C m_s^2 f_D (|<psibar psi>| - rho_S)/((1 - f_D) m^3): on the deflating spacetime m_ph grows as exp(tau_d/2), and its value depends on the cutoff Lambda through the condensate. In matter the mass falls, m_ph^2(k_F)/m_ph^2(0) = 1 - rho_S(k_F)/|<psibar psi>|: {}. The ratio vanishes only at k_F = Lambda ({:.1e}), where the filled states of positive energy cancel those of negative energy below the cutoff: that is the limit of the effective theory and not a prediction", u3, u3_txt.join("; "), u3_edge), u3 < 1e-4 && u3_edge < 1e-6);
    println!("[{:>5.1} s] unified functional done", t0.elapsed().as_secs_f64());

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    write_report(&rep, "kohnsham_checks.txt", "kohnsham_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the Kohn-Sham calculations (ks-..), of the CJT effective potential with contact interaction (cjt-..) and of the functional with the Vorton-Scalar (uni-..)", CMD)?;
    for c in &rep.checks {
        println!("[{}] {:<8} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nkohnsham: {} of {} checks passed", ok, n);
    Ok(all)
}
