//! Command `matter` of the program `deflation_cjt_ks_c` (see main.rs): the dark-matter
//! estimates of the main project on a deflating spacetime.

use spacetimes::cosmo::{C_LIGHT, EV, H_PLANCK};
use spacetimes::dimer::{binding, critical_d, scattering_length};
use spacetimes::verify::Report;
use spacetimes::cosmo::Params;
use spacetimes_deflation_cjt_ks_c::cosmo_defl::{grid, growth, main_project_reference, rate_bounds, GrowthLaw, Model, HUBBLE_TIME_100_YR};
use spacetimes_deflation_cjt_ks_c::eb_cosmo::Deflation;
use spacetimes_deflation_cjt_ks_c::spectra_defl::{h_at_fixed_theta, theta_star_100};
use spacetimes_deflation_cjt_ks_c::io::{write_csv, write_csv_str, write_report};

/// Numbers of a data file of the main project: every line that is not a comment, split
/// at commas, each cell parsed as a number (NaN if it is not one).  The header row is skipped.
fn read_numbers(path: &str) -> Option<Vec<Vec<f64>>> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.lines().filter(|l| !l.starts_with('#')).skip(1).map(|l| l.split(',').map(|c| c.parse().unwrap_or(f64::NAN)).collect()).collect())
}

fn same(a: &[Vec<f64>], b: &[Vec<f64>], tol: f64) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(r, s)| r.len() == s.len() && r.iter().zip(s).all(|(x, y)| (x.is_nan() && y.is_nan()) || (x - y).abs() <= tol * (1.0 + y.abs())))
}

/// `matter`: (1) the estimates of the main project that do not involve the coupling
/// (degeneracy, condensation), computed again; (2) the self-interaction of bound pairs
/// with the coupling of the moment; (3) the resonance in the scattering of two fermions
/// at the moment at which the pair becomes bound; (4) the growth of structure when
/// gravity strengthens with the deflation; (5) how fast each quantity drifts; (6) the
/// quanta that the deflation itself creates.
/// Writes data/dm_halo_estimates.csv, dm_selfinteraction.csv, dm_resonance.csv,
/// growth_deflation.csv, dm_drift.csv, deflation_temperature.csv, matter_checks.{txt,csv}.
pub fn run(main_data_dir: &str) -> std::io::Result<bool> {
    let mut rep = Report::new();
    let pi = std::f64::consts::PI;
    let hbar = H_PLANCK / (2.0 * pi);
    let m_sun = 1.988_47e30; // kg
    let pc: f64 = 3.085_677_581_491_367_3e16; // m
    let kev = 1e3 * EV / (C_LIGHT * C_LIGHT); // kg
    let g_states = 8.0;
    let d_c = critical_d();

    // ---- (1) estimates without the coupling --------------------------------------------------
    let systems: [(&str, f64, f64); 3] = [("dwarf spheroidal galaxy, core", 0.1, 10.0), ("Milky Way at the Sun", 0.0105, 156.0), ("cluster of galaxies, core", 0.003, 1000.0)];
    let mut rows = Vec::new();
    for (i, (_name, rho_sun, sig_kms)) in systems.iter().enumerate() {
        let rho = rho_sun * m_sun / pc.powi(3);
        let sig: f64 = sig_kms * 1e3;
        let rho_gev = rho * C_LIGHT * C_LIGHT / EV * 1e-9 * 1e-6;
        let m_deg = (6.0 * pi * pi * hbar.powi(3) * rho / (g_states * sig.powi(3))).powf(0.25) / kev;
        let m_bmax = (rho * (2.0 * pi).powf(1.5) * hbar.powi(3) / (2.612 * sig.powi(3))).powf(0.25) / kev * 1e3;
        let m15 = 1.5 * kev;
        let rho_deg = g_states * m15.powi(4) * sig.powi(3) / (6.0 * pi * pi * hbar.powi(3));
        let rho_deg_sun = rho_deg / (m_sun / pc.powi(3));
        let v_f = hbar / m15 * (6.0 * pi * pi * rho / (g_states * m15)).powf(1.0 / 3.0);
        rows.push(vec![i as f64, *rho_sun, rho_gev, *sig_kms, m_deg, m_bmax, rho_deg_sun, rho_deg_sun / rho_sun, v_f / 1e3, v_f / sig]);
    }
    write_csv("dm_halo_estimates.csv",
        &[("content", "could the particles of the model be degenerate (fermions) or Bose-condensed (bound pairs) in observed dark-matter halos?  These estimates contain the masses, the densities and the velocities only; the masses do not change on a deflating spacetime, so the numbers are those of the main project".to_string()),
          ("systems", "0 = dwarf spheroidal galaxy, core (0.1 Msun/pc^3, 10 km/s); 1 = Milky Way at the Sun (0.0105 Msun/pc^3 = 0.4 GeV/cm^3, 156 km/s); 2 = cluster of galaxies, core (0.003 Msun/pc^3, 1000 km/s)".to_string()),
          ("columns", "m_deg_keV: fermions with 8 internal states are degenerate (v_F >= sigma) only if lighter than this; mB_max_eV: bosons are condensed (n lambda^3 >= 2.612 at k T = m sigma^2) only if lighter than this; rho_deg_1p5keV: density (Msun/pc^3) at which fermions of 1.5 keV would be degenerate at this velocity dispersion, and its ratio to the actual density; vF_1p5keV: their Fermi velocity at the actual density (km/s) and its ratio to sigma".to_string()),
          ("formulas", "m_deg = [6 pi^2 hbar^3 rho/(g sigma^3)]^(1/4), g = 8; m_B,max = [rho (2 pi)^(3/2) hbar^3/(2.612 sigma^3)]^(1/4); v_F = (hbar/m)(6 pi^2 rho/(g m))^(1/3)".to_string()),
          ("command", "deflation_cjt_ks_c matter".to_string())],
        &["system", "rho_Msun_pc3", "rho_GeV_cm3", "sigma_km_s", "m_deg_keV", "mB_max_eV", "rho_deg_1p5keV_Msun_pc3", "rho_deg_over_rho", "vF_1p5keV_km_s", "vF_over_sigma"], &rows)?;
    let main_halo = read_numbers(&format!("{}/dm_halo_estimates.csv", main_data_dir));
    let halo_same = main_halo.as_ref().map(|m| same(m, &rows, 1e-9)).unwrap_or(false);
    rep.add("def-D1", "numerical",
        &format!("the estimates of degeneracy and of condensation in halos contain no coupling: fermions with eight internal states are degenerate in the core of a dwarf galaxy only below {:.3} keV, bound pairs are condensed there only below {:.0} eV. Computed again; every number agrees with the file of the main project (the masses do not change along the deflation)", rows[0][4], rows[0][5]),
        halo_same);

    // ---- (2) self-interaction of bound pairs with the coupling of the moment ----------------------
    let hbar_c_cm = hbar * C_LIGHT / EV * 100.0; // eV cm
    let gram_ev = 1e-3 * C_LIGHT * C_LIGHT / EV; // eV
    let kappa = 1.0 / (hbar_c_cm * hbar_c_cm) / gram_ev; // eV^-3 per (cm^2/g)
    let mut rows2 = Vec::new();
    let mut today: Vec<Vec<f64>> = Vec::new();
    for &alpha_today in &[0.01, 0.1, 1.0, 4.0] {
        for tau in [-2.0_f64, -1.0, -0.5, 0.0, 0.5, 1.0] {
            let alpha = alpha_today * tau.exp();
            // pairs of mass 2m with the scattering length 1/m_s: sigma/m_B = 8 pi/(2 m m_s^2) <= kappa, binding needs alpha m/m_s >= D_c
            let m_min = (4.0 * pi * d_c * d_c / (alpha * alpha * kappa)).powf(1.0 / 3.0); // eV
            let m_s = alpha * m_min / d_c;
            let a_fm = hbar_c_cm / m_s * 1e13;
            let (rho_sun, sig_kms): (f64, f64) = (0.1, 10.0);
            let rho = rho_sun * m_sun / pc.powi(3);
            let sig: f64 = sig_kms * 1e3;
            let m_b = 2.0 * m_min * EV / (C_LIGHT * C_LIGHT);
            let nl3 = rho * (2.0 * pi).powf(1.5) * hbar.powi(3) / (m_b.powi(4) * sig.powi(3));
            rows2.push(vec![alpha_today, tau, alpha, m_min / 1e6, m_s / 1e6, a_fm, nl3]);
            if tau == 0.0 {
                today.push(vec![alpha, m_min / 1e6, m_s / 1e6, a_fm, nl3]);
            }
        }
    }
    write_csv("dm_selfinteraction.csv",
        &[("content", "smallest fermion mass for which a gas of bound pairs satisfies sigma/m <= 1 cm^2/g, if the pair-pair scattering length is the range 1/m_s of the binding potential and the potential binds (alpha m/m_s >= D_c), evaluated with the coupling of the moment alpha = alpha_today exp(tau_d), tau_d = gsum (x4 - x4_today)".to_string()),
          ("formula", "m_min = [4 pi D_c^2/(alpha^2 kappa)]^(1/3), kappa = 1 cm^2/g in natural units; m_s = alpha m_min/D_c; last column: phase-space density n lambda^3 of the pairs in the core of a dwarf spheroidal galaxy (condensation needs 2.612)".to_string()),
          ("constants", format!("kappa = {:.5e} eV^-3; D_c = {:.5}", kappa, d_c)),
          ("note", "the rows with tau_d = 0 are the table of the main project. For a given particle (m and m_s fixed, they do not change) the cross-section of the pairs, 8 pi/m_s^2, does not change either; what changes is whether the pair is bound".to_string()),
          ("command", "deflation_cjt_ks_c matter".to_string())],
        &["alpha_today", "tau_d", "alpha", "m_min_MeV", "m_s_MeV", "range_fm", "phase_space_density_dwarf"], &rows2)?;
    let main_si = read_numbers(&format!("{}/dm_selfinteraction.csv", main_data_dir));
    let si_same = main_si.as_ref().map(|m| same(m, &today, 1e-9)).unwrap_or(false);
    rep.add("def-D2", "numerical",
        &format!("self-interaction of bound pairs: with the coupling of today the table of the main project is reproduced (smallest fermion mass {:.0} MeV for alpha_VS = 0.1). The smallest mass falls as alpha_VS^(-2/3), that is, as exp(-2 tau_d/3) along the deflation. For a given particle the masses m and m_s are constant, hence so is the cross-section 8 pi/m_s^2 of the pairs; the deflation changes whether the pair exists: it is bound only after D_VS = alpha_VS m/m_s has grown to {:.5}", today[1][1], d_c),
        si_same);

    // ---- (3) resonance at the moment of binding ---------------------------------------------------
    // example particle: the row alpha = 0.1 of the table (m = 918 MeV, m_s = 54.7 MeV): D = D_c today.  Let the clock run.
    let (m_ev, ms_ev) = (today[1][1] * 1e6, today[1][2] * 1e6);
    let mut rrows: Vec<Vec<f64>> = Vec::new();
    let vels = [10.0_f64, 156.0, 1000.0]; // km/s, relative velocity taken equal to the dispersion
    let mut peak = [0.0_f64; 3];
    let mut far = [0.0_f64; 3];
    let mut window = [0.0_f64; 3];
    let n_tau = 2001;
    for i in 0..n_tau {
        let tau = -1.0 + 2.0 * i as f64 / (n_tau - 1) as f64;
        let d = d_c * tau.exp();
        let a = scattering_length(d) / ms_ev; // eV^-1
        let mut row = vec![tau, d, a * ms_ev];
        for (j, v) in vels.iter().enumerate() {
            let k = 0.5 * m_ev * (v * 1e3 / C_LIGHT); // reduced mass m/2 times relative velocity
            let sigma = 4.0 * pi * a * a / (1.0 + k * k * a * a); // eV^-2
            let s_over_m = sigma / m_ev / kappa; // cm^2/g
            row.push(s_over_m);
            peak[j] = peak[j].max(s_over_m);
            if i == 0 {
                far[j] = s_over_m;
            }
            if s_over_m > 100.0 {
                window[j] += 2.0 / (n_tau - 1) as f64;
            }
        }
        rrows.push(row);
    }
    write_csv("dm_resonance.csv",
        &[("content", "scattering of two unbound fermions of the model at low energy while the deflation carries D_VS = alpha_VS m/m_s through the value D_c at which the pair becomes bound: s-wave cross-section per mass, sigma/m = 4 pi a^2/[(1 + k^2 a^2) m], with the scattering length a(D_VS) of the Yukawa potential (routine dimer::scattering_length of the main project) and k = m v/2".to_string()),
          ("particle", format!("m = {:.1} MeV, m_s = {:.2} MeV (the row alpha_VS = 0.1 of data/dm_selfinteraction.csv); tau_pair = 0 is the moment of binding", m_ev / 1e6, ms_ev / 1e6)),
          ("columns", "tau_pair = gsum (x4 - x4_binding) = ln(D_VS/D_c); D = D_VS = D_c exp(tau_pair); a_ms: scattering length in units of 1/m_s; sigma/m in cm^2/g at the relative velocities 10, 156 and 1000 km/s (effective-range corrections neglected)".to_string()),
          ("command", "deflation_cjt_ks_c matter".to_string())],
        &["tau_pair", "D", "a_ms", "sigma_over_m_v10", "sigma_over_m_v156", "sigma_over_m_v1000"], &rrows)?;
    // unitarity limit 4 pi/k^2 at v = 10 km/s
    let k10 = 0.5 * m_ev * (10.0e3 / C_LIGHT);
    let unit10 = 4.0 * pi / (k10 * k10) / m_ev / kappa;
    rep.add("def-D3", "numerical",
        &format!("the moment at which the pair becomes bound is a zero-energy resonance of the scattering of two fermions: the scattering length diverges at D_VS = D_c. For a particle with m = {:.0} MeV and m_s = {:.1} MeV the cross-section per mass at the relative velocities 10, 156 and 1000 km/s rises from {:.1e}, {:.1e}, {:.1e} cm^2/g one unit of the deflation clock before the binding to the peak values {:.1e}, {:.1e}, {:.1e} cm^2/g (limit of unitarity at 10 km/s: {:.1e}); it exceeds 100 cm^2/g during an interval of the clock of {:.3}, {:.3}, {:.3} around the moment of binding. A deflating universe in which the dark matter is made of these fermions passes through an epoch of strong, velocity-dependent self-interaction", m_ev / 1e6, ms_ev / 1e6, far[0], far[1], far[2], peak[0], peak[1], peak[2], unit10, window[0], window[1], window[2]),
        peak[0] > 1e3 * far[0] && peak[0] <= unit10 * 1.0001 && peak[0] > peak[1] && peak[1] > peak[2] && window[0] >= window[2] && window[0] < 0.5);

    // ---- (4) growth of structure ---------------------------------------------------------------------
    // The density contrast of pressureless matter, for the two closures of the perturbation equations (assumption A3;
    // `cosmo_defl::growth`), and, for comparison, for the four-dimensional equation that an earlier version used.
    let h0 = 0.6736_f64;
    let (z_star, _, _) = main_project_reference();
    let none = |_s: f64| 1.0;
    let (a_min, n) = grid(z_star, 4000);
    // the bounds on the rate, in units of H_0, and H_0 times the age of the model without deflation:
    // the same numbers that the command `cosmology` reports (cosmo_defl::rate_bounds)
    let rb = rate_bounds(h0);
    let (llr, bbn, gf20, t0_h0) = (rb.llr / h0, rb.bbn / h0, rb.gfermi_20 / h0, rb.age * h0);
    let mut ratios: Vec<f64> = vec![0.0, 1e-3, llr, 1e-2, 3e-2, bbn, 0.1, 0.3];
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // the Hubble constant that keeps the angular scale of the sound horizon: the full calculation of recombination on
    // the deflating background, as in the command `cosmology` (data/hubble_deflation.csv)
    let fid = Params::fiducial();
    let theta_fid = theta_star_100(&fid, &Deflation::none());
    let laws = [GrowthLaw::Momentum, GrowthLaw::Pressure, GrowthLaw::FourDimensional];
    let mut grows: Vec<Vec<f64>> = Vec::new();
    let mut refs = [(0.0, 0.0); 3];
    for r in &ratios {
        let base = Model::fiducial(r * h0, h0);
        let h = if *r == 0.0 { h0 } else { h_at_fixed_theta(&fid, &Deflation::equal_rates(r * h0), theta_fid) };
        let mdl = Model { h, ..base };
        let (hist_h, hist) = (base.history(a_min, n, &none), mdl.history(a_min, n, &none));
        let mut row = vec![*r, h];
        for (i, law) in laws.iter().enumerate() {
            // (i) the Hubble constant held at 67.36 km/s/Mpc: the effect of the deflation on the growth alone
            let (d_h, f_h) = growth(&base, &hist_h, 0.01, &none, *law);
            // (ii) the Hubble constant that keeps the angular scale of the sound horizon
            let (d, f) = growth(&mdl, &hist, 0.01, &none, *law);
            if *r == 0.0 {
                refs[i] = (d, f);
            }
            let (d_ref, f_ref) = refs[i];
            row.extend_from_slice(&[d, f, d / d_ref, f / f_ref, d * f / (d_ref * f_ref), d_h / d_ref, f_h / f_ref]);
        }
        grows.push(row);
    }
    let mut gcols: Vec<String> = vec!["gsum_over_H0".into(), "h".into()];
    for tag in ["M", "P", "N4"] {
        for c in ["growth", "f_growth", "growth_ratio", "f_growth_ratio", "f_sigma8_ratio", "growth_ratio_fixed_h", "f_growth_ratio_fixed_h"] {
            gcols.push(format!("{}_{}", c, tag));
        }
    }
    let gcols_ref: Vec<&str> = gcols.iter().map(|s| s.as_str()).collect();
    write_csv("growth_deflation.csv",
        &[("content", "linear growth of the density contrast delta_m of pressureless matter in ordinary space on the deflating spacetime (assumptions A1, A2, A3), from a = 0.01 to today. With a Newton's constant that depends on the time the growth depends on the closure of the perturbation equations (A3). Closure \"Pressure\" (suffix _P; also the closure \"Shear\" far inside the horizon): delta_m'' + (2 H - gsum/2) delta_m' = 4 pi G_N(x4) rho_m delta_m, primes for derivatives with respect to x4. Closure \"Momentum\" (suffix _M): in conformal time, delta_m'' + [(2 He' + R)/(2 He)] delta_m' = [(Hc - a gsum)/(2 He)] g delta_m, with Hc = a H, He = Hc - a gsum/2, g = 8 pi G_N(x4) a^2 rho_m and R = g + (4/3) 8 pi G_N(x4) a^2 rho_r. Suffix _N4: the four-dimensional equation delta_m'' + 2 H delta_m' = 4 pi G_N(x4) rho_m delta_m, which is not a consequence of the eight-dimensional equations and is listed only because an earlier version of this file contained it".to_string()),
          ("columns", "gsum_over_H0; h: the Hubble constant that keeps the angular scale of the sound horizon (as in data/hubble_deflation.csv); then for each of the three equations: growth = delta_m at a = 1 for delta_m = a + 2 a_eq/3 at a = 0.01; f_growth = d ln delta_m/d ln a at a = 1; growth_ratio, f_growth_ratio, f_sigma8_ratio: ratios to the model without deflation (f_sigma8_ratio is the ratio of the products growth times f_growth); growth_ratio_fixed_h, f_growth_ratio_fixed_h: the same ratios if the Hubble constant is held at 67.36 km/s/Mpc instead".to_string()),
          ("command", "deflation_cjt_ks_c matter".to_string())],
        &gcols_ref, &grows)?;
    // columns of the law i: 2 + 7 i + (0 growth, 1 f, 2 growth ratio, 3 f ratio, 4 f sigma8 ratio, 5 growth ratio at fixed h, 6 f ratio at fixed h)
    let at = |r: f64| grows.iter().find(|x| (x[0] - r).abs() < 1e-12).unwrap().clone();
    let col = |law: usize, q: usize| 2 + 7 * law + q;
    let slope = |law: usize, q: usize| (at(1e-3)[col(law, q)] - 1.0) / 1e-3;
    let mono = |law: usize, q: usize, up: bool| grows.windows(2).all(|w| if up { w[1][col(law, q)] > w[0][col(law, q)] } else { w[1][col(law, q)] < w[0][col(law, q)] });
    rep.add("def-D4", "numerical",
        &format!("growth of structure on the deflating spacetime (A1, A2, A3; data/growth_deflation.csv). The growth depends on the closure of the perturbation equations at the first order in the rate. (i) At a fixed Hubble constant the amplitude of the density contrast today (sigma_8 for a given primordial amplitude) falls with the rate in both closures: d ln sigma_8/d(gsum/H_0) = {:.3} (closure \"Momentum\") and {:.3} (closure \"Pressure\"). (ii) The Hubble constant that keeps the angular scale of the sound horizon is lower, the cosmological term takes over later, and matter clusters for longer. With that Hubble constant the net change is d ln sigma_8/d(gsum/H_0) = {:+.3} (\"Momentum\") and {:+.3} (\"Pressure\"): {:+.3} and {:+.3} per cent at the rate that lunar laser ranging allows, {:+.2} and {:+.2} per cent at the rate that nucleosynthesis allows. The growth rate f_growth = d ln delta_m/d ln a today rises in both closures, by the factors {:.4} and {:.4} (lunar laser ranging), {:.4} and {:.4} (nucleosynthesis). Common to both closures is that, at a fixed angle, the amplitude does not fall and the growth rate rises; how large these changes are, and whether the amplitude changes at all, depends on which perturbations the agent carries that holds the extra scale factors, and is not a prediction of the hypotheses. (iii) The four-dimensional equation delta_m'' + 2 H delta_m' = 4 pi G_N(x4) rho_m delta_m, which an earlier version of this command integrated, is neither closure: it gives d ln sigma_8/d(gsum/H_0) = {:.3} at a fixed Hubble constant and {:+.3} at a fixed angle",
            slope(0, 5), slope(1, 5), slope(0, 2), slope(1, 2), 100.0 * (at(llr)[col(0, 2)] - 1.0), 100.0 * (at(llr)[col(1, 2)] - 1.0), 100.0 * (at(bbn)[col(0, 2)] - 1.0), 100.0 * (at(bbn)[col(1, 2)] - 1.0),
            at(llr)[col(0, 3)], at(llr)[col(1, 3)], at(bbn)[col(0, 3)], at(bbn)[col(1, 3)], slope(2, 5), slope(2, 2)),
        mono(0, 5, false) && mono(1, 5, false) && slope(0, 5) < slope(2, 5) && slope(2, 5) < slope(1, 5) && slope(1, 5) < 0.0 && slope(0, 2) < slope(1, 2)
            && slope(0, 2) > -0.02 && mono(1, 2, true) && at(bbn)[col(0, 3)] > 1.0 && at(bbn)[col(1, 3)] > at(bbn)[col(0, 3)] && at(llr)[col(0, 3)] > 1.0);

    // ---- (5) drift of the quantities -----------------------------------------------------------------
    // logarithmic derivatives with respect to tau_d (every quantity depends on tau_d through D or alpha ~ exp(tau_d))
    let dlog = |f: &dyn Fn(f64) -> f64, d: f64| -> f64 {
        let e = 1e-3;
        (f(d * (1.0 + e)).abs().ln() - f(d * (1.0 - e)).abs().ln()) / ((1.0 + e).ln() - (1.0 - e).ln())
    };
    let mut drows: Vec<Vec<String>> = Vec::new();
    let bounds = [("lunar laser ranging (A2)", llr), ("nucleosynthesis, G_N (A2)", bbn), ("nucleosynthesis, G_Fermi (HV), 20 per cent", gf20)];
    let mut push = |q: &str, at: &str, n: f64| {
        let mut row = vec![q.to_string(), at.to_string(), format!("{:.3}", n)];
        for (_, b) in bounds.iter() {
            row.push(format!("{:.2e}", n * b * t0_h0));
        }
        drows.push(row);
    };
    push("alpha_VS, G_VS, D_VS, C", "any", 1.0);
    push("g_VS", "any", 0.5);
    push("fermion mass m, mass m_s of the radial mode", "any", 0.0);
    for d in [0.5, 1.0, 1.5] {
        push("scattering length of two fermions (no bound state)", &format!("D_VS = {}", d), dlog(&|x| scattering_length(x), d));
    }
    for d in [1.8, 2.0, 3.0, 5.0, 10.0] {
        push("binding energy of the pair", &format!("D_VS = {}", d), dlog(&|x| binding(x).unwrap_or(1e-300), d));
    }
    for (d, ms) in [(5.0_f64, 0.2_f64), (10.0, 0.1)] {
        // rest mass 2m - E_b, E_b = eps(D) m_s^2/m, in units of m: 2 - ms^2 eps(D)
        push("rest mass of the bound pair", &format!("D_VS = {}, m_s/m = {}", d, ms), dlog(&|x| 2.0 - ms * ms * binding(x).unwrap_or(0.0), d));
    }
    for lam in [0.15, 0.2, 0.3] {
        push("gap and critical temperature of the paired state (weak coupling)", &format!("pairing strength lambda = {}", lam), 1.0 / lam);
    }
    push("cross-section of two fermions at low energy (Born regime, D_VS << 1)", "D_VS = 0.05", 2.0 * dlog(&|x| scattering_length(x), 0.05));
    write_csv_str("dm_drift.csv",
        &[("content", format!("how fast the quantities of the model drift on a deflating spacetime: n = d ln(quantity)/d tau_d with tau_d = gsum x4, and the relative change n gsum t_0 over the age of the universe at the largest rates that the three bounds allow (gsum = {:.2e}, {:.4} and {:.3} H_0), with the age t_0 of the model without deflation (H_0 t_0 = {:.4})", llr, bbn, gf20, t0_h0)),
          ("columns", "quantity; where evaluated; n; change over the age at the bound from lunar laser ranging (assumption A2); at the bound from nucleosynthesis through G_N (A2); at the bound from nucleosynthesis through the Fermi constant (assumption HV, 20 per cent)".to_string()),
          ("command", "deflation_cjt_ks_c matter".to_string())],
        &["quantity", "at", "n", "change_LLR", "change_BBN_G", "change_BBN_GF"], &drows)?;
    let n_a = dlog(&|x| scattering_length(x), 1.5);
    let n_e = dlog(&|x| binding(x).unwrap_or(1e-300), 1.8);
    rep.add("def-D5", "numerical",
        &format!("drift of the quantities (table data/dm_drift.csv): the couplings change by {:.1e} of themselves over the age of the universe at the rate that lunar laser ranging allows and by {:.2} at the rate that nucleosynthesis allows; quantities near the threshold of binding move much faster (scattering length at D_VS = 1.5: n = {:.1}; binding energy at D_VS = 1.8: n = {:.1}), and the gap of a weakly paired state moves as 1/lambda (n = 5 for lambda = 0.2). The masses do not move", llr * t0_h0, bbn * t0_h0, n_a, n_e),
        n_a > 5.0 && n_e > 10.0);

    // ---- (6) the quanta that the deflation creates -------------------------------------------------------
    // temperature of the creation spectrum: k T = hbar rate_t/(2 pi)
    let hbar_ev_s = hbar / EV; // eV s
    let year = 3.155_76e7; // s
    let k_b_ev = 8.617_333_262e-5; // eV/K
    let mut trows: Vec<Vec<String>> = Vec::new();
    let h0_per_yr = h0 / HUBBLE_TIME_100_YR;
    let mut t_llr = 0.0;
    // the largest of the three bounds: temperature there, and hbar gsum in units of a mass of 1 keV
    let (mut t_max, mut gsum_over_kev) = (0.0_f64, 0.0_f64);
    for (name, r) in [("lunar laser ranging (A2)", llr), ("nucleosynthesis, G_N (A2)", bbn), ("nucleosynthesis, G_Fermi (HV), 20 per cent", gf20), ("gsum = H_0", 1.0)] {
        let rate_per_s = r * h0_per_yr / year / 3.0; // each of three equal rates
        let e = hbar_ev_s * rate_per_s; // eV
        let t = e / (2.0 * pi) / k_b_ev;
        if name.starts_with("lunar") {
            t_llr = t;
        }
        if !name.starts_with("gsum") && t > t_max {
            t_max = t;
            gsum_over_kev = 3.0 * e / 1e3;
        }
        // suppression of the creation of fermions of 1 keV and of 1 GeV: exp(-2 pi m/rate); print the exponent
        trows.push(vec![name.to_string(), format!("{:.3e}", r), format!("{:.3e}", e), format!("{:.3e}", t), format!("{:.2e}", 2.0 * pi * 1e3 / e), format!("{:.2e}", 2.0 * pi * 1e9 / e)]);
    }
    write_csv_str("deflation_temperature.csv",
        &[("content", "7+1: the creation of quanta by the deflation has the spectrum of a temperature k T = hbar rate_t/(2 pi) (modes that depend on the deflating coordinate x_t). The table gives that temperature for three equal rates at the largest total rates that the bounds allow, and the exponent 2 pi m/rate_t that suppresses the creation of particles of mass m".to_string()),
          ("columns", "bound; gsum/H_0; hbar rate_t in eV; T in kelvin; exponent for m = 1 keV; exponent for m = 1 GeV (the number of created particles per mode is exp(-exponent))".to_string()),
          ("command", "deflation_cjt_ks_c matter".to_string())],
        &["bound", "gsum_over_H0", "hbar_rate_eV", "T_K", "exponent_1keV", "exponent_1GeV"], &trows)?;
    rep.add("def-D6", "numerical",
        &format!("the quanta that the deflation creates (7+1; modes that depend on a deflating coordinate): at the present rate that lunar laser ranging allows the temperature of the spectrum is {:.1e} K; at the largest rate that any of the three bounds of the cosmological part allows (gsum = {:.3} H_0) it is {:.1e} K, and hbar gsum is then {:.1e} of a mass of 1 keV. The creation of massive particles is suppressed by exp(-2 pi m/rate_t) with an exponent above 1e37 for every mass above 1 keV: no particles are created today. Creation matters only for an epoch in which the rate was comparable with the masses", t_llr, gf20.max(bbn).max(llr), t_max, gsum_over_kev),
        t_llr < 1e-28 && t_max < 1e-30 && gsum_over_kev < 1e-36);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "matter_checks.txt", "matter_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: checks of the dark-matter estimates on the deflating spacetime", "deflation_cjt_ks_c matter")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nmatter: {} of {} checks passed", ok, n);
    Ok(all)
}
