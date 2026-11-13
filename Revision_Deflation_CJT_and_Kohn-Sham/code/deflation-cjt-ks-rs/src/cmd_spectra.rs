//! Commands `background`, `spectrum`, `forecast`, `families` and `superfluid` of the program
//! `deflation` (see main.rs): the Einstein-Boltzmann calculations of the main project on a
//! deflating spacetime.  The assumptions (A1, A2 and, for the perturbations, A3 with its
//! two closures "Momentum" and "Pressure") and the routines are described in `spectra_defl`.

use std::time::Instant;

use spacetimes::cosmo::{neutrino_factor, DarkFluid, Params, Step, SuperfluidDm, C_LIGHT, EV, G_NEWTON, H_PLANCK, MPC};
use spacetimes::forecast::{chi2, fisher, invert_spd, solve_spd, to_vec, Config, Fit, Jacobian, Mock, NPAR, PAR_NAMES};
use spacetimes::spectra::{Spectra, Workspace};
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks::cosmo_defl::{grid, growth, rate_bounds, solve_h, GrowthLaw, Model, HUBBLE_TIME_100_YR};
use spacetimes_deflation_cjt_ks::eb_cosmo::{Background, Closure, Deflation, Thermo};
use spacetimes_deflation_cjt_ks::eb_fit::fit;
use spacetimes_deflation_cjt_ks::eb_spectra::{compute, matter_power};
use spacetimes_deflation_cjt_ks::io::{write_csv, write_report};
use spacetimes_deflation_cjt_ks::spectra_defl::{growth_factor, h_at_fixed_theta, jacobian_defl, potentials, rate_derivative};

/// km/s/Mpc per unit of h
const KMS: f64 = 100.0;
/// the Hubble parameter of the fiducial model of the main project
const H_FID: f64 = 0.6736;

fn elapsed(t0: &Instant) -> f64 {
    t0.elapsed().as_secs_f64()
}

/// Numbers of a data file: every line that is not a comment, split at commas (the header row is skipped).
fn read_numbers(path: &str) -> Option<Vec<Vec<f64>>> {
    let text = std::fs::read_to_string(path).ok()?;
    Some(text.lines().filter(|l| !l.starts_with('#')).skip(1).map(|l| l.split(',').map(|c| c.parse().unwrap_or(f64::NAN)).collect()).collect())
}

/// Largest relative deviation |x/y - 1| (absolute where y = 0) between two tables of the same shape;
/// infinite if the shapes differ.  Cells that are not numbers in both tables are skipped.
fn table_deviation(a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
    if a.len() != b.len() || a.iter().zip(b).any(|(r, s)| r.len() != s.len()) {
        return f64::INFINITY;
    }
    let mut dev = 0.0f64;
    for (r, s) in a.iter().zip(b) {
        for (x, y) in r.iter().zip(s) {
            if x.is_nan() && y.is_nan() {
                continue;
            }
            dev = dev.max(if *y != 0.0 { (x / y - 1.0).abs() } else { x.abs() });
        }
    }
    dev
}

/// Are two sets of spectra the same numbers, bit for bit?
fn same_bits(a: &Spectra, b: &Spectra) -> bool {
    let eq = |x: &[f64], y: &[f64]| x.len() == y.len() && x.iter().zip(y).all(|(p, q)| p.to_bits() == q.to_bits());
    eq(&a.tt, &b.tt) && eq(&a.te, &b.te) && eq(&a.ee, &b.ee) && eq(&a.pk, &b.pk) && a.sigma8.to_bits() == b.sigma8.to_bits()
        && a.thermo.theta_star_100.to_bits() == b.thermo.theta_star_100.to_bits() && a.thermo.rs_star.to_bits() == b.thermo.rs_star.to_bits()
}

/// Largest value of |f(l)| for l_lo <= l <= l_hi.
fn max_over(l_lo: usize, l_hi: usize, f: &dyn Fn(usize) -> f64) -> f64 {
    (l_lo..=l_hi).map(|l| f(l).abs()).fold(0.0, f64::max)
}

/// Difference of two models in units of the cosmic variance of the temperature spectrum,
/// (TT_1 - TT_2) / (sqrt(2/(2l+1)) TT_2): the largest value in a range of multipoles, and
/// the root of the sum of the squares over the range (the number of standard deviations by
/// which an ideal temperature measurement of the full sky would tell the two apart).
fn variance_units(a: &Spectra, b: &Spectra, l_lo: usize, l_hi: usize) -> (f64, f64) {
    let f = |l: usize| (a.tt[l] - b.tt[l]) / ((2.0 / (2.0 * l as f64 + 1.0)).sqrt() * b.tt[l]);
    (max_over(l_lo, l_hi, &f), (l_lo..=l_hi).map(|l| f(l) * f(l)).sum::<f64>().sqrt())
}

/// Positions and heights of the local maxima of a spectrum D_l between l_lo and l_hi (parabolic
/// refinement; the routine of the command `spectrum` of the main project).
fn peaks(d: &[f64], l_lo: usize, l_hi: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for l in l_lo.max(3)..l_hi.min(d.len() - 2) {
        if d[l] > d[l - 1] && d[l] >= d[l + 1] {
            let den = d[l - 1] - 2.0 * d[l] + d[l + 1];
            let shift = if den != 0.0 { 0.5 * (d[l - 1] - d[l + 1]) / den } else { 0.0 };
            out.push((l as f64 + shift, d[l] - 0.25 * (d[l - 1] - d[l + 1]) * shift));
        }
    }
    out
}

/// The two closures of the perturbation equations, with the names used in the data files.
const CLOSURES: [(Closure, &str); 2] = [(Closure::Momentum, "M"), (Closure::Pressure, "P")];

/// Three equal rates with the total rate `r` in units of the fiducial Hubble constant, and a closure.
fn deflation(r: f64, closure: Closure) -> Deflation {
    Deflation { closure, ..Deflation::equal_rates(r * H_FID) }
}

/// The two rates that the observations of gravity allow, in units of the fiducial Hubble constant:
/// lunar laser ranging and nucleosynthesis (see `cosmo_defl::rate_bounds`).
fn allowed_rates() -> (f64, f64) {
    let rb = rate_bounds(H_FID);
    (rb.llr / H_FID, rb.bbn / H_FID)
}

fn finish(rep: &mut Report, t0: &Instant, name: &str, title: &str) -> std::io::Result<bool> {
    let (ok, n) = (rep.checks.iter().filter(|c| c.pass).count(), rep.checks.len());
    let all = ok == n;
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(rep, &format!("{}_checks.txt", name), &format!("{}_checks.csv", name), title, &format!("deflation_cjt_ks {}", name))?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\n{}: {} of {} checks passed   ({:.0} s)", name, ok, n, elapsed(t0));
    Ok(all)
}

// =================================================================================================
// background
// =================================================================================================

/// `background`: the thermal history of the three models of the command `background` of the
/// main project while x5, x6, x7 deflate: conformal age, last scattering, drag epoch, sound
/// horizons, angular scales, reionization, helium.  Writes data/background_deflation.csv and
/// background_checks.{txt,csv}.
pub fn run_background() -> std::io::Result<bool> {
    let t0 = Instant::now();
    let mut rep = Report::new();
    let fid = Params::fiducial();
    let variants: [(&str, Params); 3] = [
        ("fiducial LambdaCDM", fid),
        ("fluid dark radiation, N = 0.5", Params { dark: DarkFluid { n_ir: 0.5, step: None }, ..fid }),
        ("stepped fluid (1 boson + 16 fermionic d.o.f.), N_IR = 0.5, z_t = 2e4", Params { dark: DarkFluid { n_ir: 0.5, step: Some(Step::project(2.0e4)) }, ..fid }),
    ];
    let (r_llr, r_bbn) = allowed_rates();
    let rates = [0.0, r_llr, r_bbn];
    let cols = ["model", "gsum_over_H0", "fixed", "H0", "tau0_Mpc", "z_star", "z_rec", "z_drag", "rs_star", "rs_drag", "theta_s_100", "theta_star_100", "z_reio", "Y_He", "GN_star", "GN_early", "age_Gyr"];
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut bits = true;
    println!("{:<70} {:>9} {:>6} {:>8} {:>9} {:>8} {:>8} {:>8} {:>9} {:>9} {:>10} {:>10} {:>7} {:>7} {:>8}", "model", "gsum/H0", "fixed", "H0", "tau0", "z_star", "z_rec", "z_drag", "rs_star", "rs_drag", "100th_s", "100th_*", "z_reio", "Y_He", "GN_star");
    for (iv, (name, p)) in variants.iter().enumerate() {
        // the angle of the model without deflation, which the rows with fixed = 1 keep
        let theta0 = Thermo::new(&Background::new(p, &Deflation::none())).theta_star_100;
        for &r in &rates {
            // fixed = 0: the Hubble constant of the main project; fixed = 1: the Hubble constant that keeps theta_*
            for fixed in [0usize, 1] {
                if r == 0.0 && fixed == 1 {
                    continue;
                }
                let d = deflation(r, Closure::Momentum);
                let h = if fixed == 1 { h_at_fixed_theta(p, &d, theta0) } else { p.h };
                let q = Params { h, ..*p };
                let bg = Background::new(&q, &d);
                let th = Thermo::new(&bg);
                if r == 0.0 {
                    // without deflation: the numbers of the main project, to the last bit
                    let bg0 = spacetimes::cosmo::Background::new(p);
                    let th0 = spacetimes::cosmo::Thermo::new(&bg0);
                    for (x, y) in [(bg.tau0, bg0.tau0), (th.z_star, th0.z_star), (th.z_rec, th0.z_rec), (th.z_drag, th0.z_drag), (th.rs_star, th0.rs_star), (th.rs_drag, th0.rs_drag), (th.theta_s_100, th0.theta_s_100), (th.theta_star_100, th0.theta_star_100), (th.z_reio, th0.z_reio), (th.y_he, th0.y_he)] {
                        bits &= x.to_bits() == y.to_bits();
                    }
                }
                let g_star = bg.s_of_a(1.0 / (1.0 + th.z_star)).exp();
                println!("{:<70} {:>9.6} {:>6} {:>8.4} {:>9.2} {:>8.2} {:>8.2} {:>8.2} {:>9.3} {:>9.3} {:>10.5} {:>10.5} {:>7.3} {:>7.4} {:>8.5}", name, r, fixed, h * KMS, bg.tau0, th.z_star, th.z_rec, th.z_drag, th.rs_star, th.rs_drag, th.theta_s_100, th.theta_star_100, th.z_reio, th.y_he, g_star);
                rows.push(vec![iv as f64, r, fixed as f64, h * KMS, bg.tau0, th.z_star, th.z_rec, th.z_drag, th.rs_star, th.rs_drag, th.theta_s_100, th.theta_star_100, th.z_reio, th.y_he, g_star, bg.s_early().exp(), bg.age() * HUBBLE_TIME_100_YR / 1e9]);
            }
        }
    }
    write_csv("background_deflation.csv",
        &[("content", "thermal history on the deflating spacetime (assumptions A1, A2: Newton's constant G_N(x4) = G_N(today) exp(gsum (x4 - x4_today)), three equal rates) for the three models of the command `background` of the main project".to_string()),
          ("model", "0 = fiducial LambdaCDM; 1 = fluid dark radiation, N = 0.5; 2 = stepped fluid of the model (1 massless boson + 16 fermionic degrees of freedom), N_IR = 0.5, z_t = 2e4".to_string()),
          ("columns", "gsum_over_H0: total rate of deflation in units of 67.36 km/s/Mpc (0, the rate that lunar laser ranging allows, the rate that nucleosynthesis allows); fixed: 0 = the Hubble constant of the main project, 67.36 km/s/Mpc; 1 = the Hubble constant for which the angle theta_* has its value without deflation; H0 in km/s/Mpc; tau0_Mpc: conformal age; z_star, z_rec, z_drag: redshifts of last scattering (optical depth 1), of the maximum of the visibility function, and of the end of the baryon drag; rs_star, rs_drag: sound horizons in Mpc; theta_s_100, theta_star_100: 100 times the angles of the sound horizon at z_rec and at z_star; z_reio; Y_He: helium mass fraction; GN_star, GN_early: Newton's constant at last scattering and in the early universe over its value today; age_Gyr".to_string()),
          ("command", "deflation_cjt_ks background".to_string())],
        &cols, &rows)?;
    rep.add("def-B1", "exact", "thermal history without deflation: for the three models of the command `background` of the main project (fiducial; fluid dark radiation; stepped fluid) the copy of the background and recombination modules gives the numbers of the main project to the last bit: conformal age, redshifts of last scattering, of the maximum of the visibility and of the drag epoch, the two sound horizons, the two angles, the redshift of reionization and the helium fraction", bits);
    // trends
    let row = |iv: usize, r: f64, fixed: usize| rows.iter().find(|x| x[0] == iv as f64 && x[1] == r && x[2] == fixed as f64).unwrap().clone();
    let c = |name: &str| cols.iter().position(|x| *x == name).unwrap();
    let mut trend = true;
    for iv in 0..3 {
        let (a, b, e) = (row(iv, 0.0, 0), row(iv, r_llr, 0), row(iv, r_bbn, 0));
        // weaker gravity in the past: larger sound horizon and angle, later last scattering, less helium, greater age
        for (x, y) in [(&a, &b), (&b, &e)] {
            trend &= y[c("rs_star")] > x[c("rs_star")] && y[c("theta_star_100")] > x[c("theta_star_100")] && y[c("z_star")] < x[c("z_star")] && y[c("Y_He")] < x[c("Y_He")] && y[c("age_Gyr")] > x[c("age_Gyr")] && y[c("GN_star")] < x[c("GN_star")];
        }
        let (bf, ef) = (row(iv, r_llr, 1), row(iv, r_bbn, 1));
        trend &= (bf[c("theta_star_100")] / a[c("theta_star_100")] - 1.0).abs() < 1e-9 && (ef[c("theta_star_100")] / a[c("theta_star_100")] - 1.0).abs() < 1e-9 && ef[c("H0")] < bf[c("H0")] && bf[c("H0")] < a[c("H0")];
    }
    let (a, e, ef) = (row(0, 0.0, 0), row(0, r_bbn, 0), row(0, r_bbn, 1));
    let (s1, s2) = (row(1, r_bbn, 1), row(2, r_bbn, 1));
    rep.add("def-B2", "numerical",
        &format!("thermal history on the deflating spacetime (assumptions A1, A2; data/background_deflation.csv). Gravity was weaker in the past. At the rate that nucleosynthesis allows (gsum = {:.5} H_0) and the Hubble constant of the main project, Newton's constant at last scattering is {:.4} of today's, the sound horizon at last scattering grows from {:.3} to {:.3} Mpc, 100 theta_* from {:.5} to {:.5}, last scattering moves from z_* = {:.2} to {:.2}, the helium fraction falls from {:.5} to {:.5}, and the age grows from {:.3} to {:.3} Gyr; the same directions hold for the two models with dark radiation and at the rate of lunar laser ranging. The Hubble constant that restores the angle is {:.3} km/s/Mpc for the fiducial model, {:.3} with fluid dark radiation (N = 0.5) and {:.3} with the stepped fluid of the model (against {:.3}, {:.3}, {:.3} without deflation at the same angles)", r_bbn, e[c("GN_star")], a[c("rs_star")], e[c("rs_star")], a[c("theta_star_100")], e[c("theta_star_100")], a[c("z_star")], e[c("z_star")], a[c("Y_He")], e[c("Y_He")], a[c("age_Gyr")], e[c("age_Gyr")], ef[c("H0")], s1[c("H0")], s2[c("H0")], row(0, 0.0, 0)[c("H0")], row(1, 0.0, 0)[c("H0")], row(2, 0.0, 0)[c("H0")]),
        trend);
    finish(&mut rep, &t0, "background", "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the thermal history on the deflating spacetime")
}

// =================================================================================================
// spectrum
// =================================================================================================

/// `spectrum`: (1) the copy of the Einstein-Boltzmann program against the original;
/// (2) the spectra of the microwave background and of matter while x5, x6, x7 deflate, at a
/// fixed angular scale of the sound horizon, for the two closures; (3) what depends on the
/// closure; (4) what the agent supplies, and the numerical checks of the identities;
/// (5) numerical convergence; (6) the helium fraction; (7) the growth of structure.
/// Writes data/cmb_deflation_summary.csv, cmb_deflation_cl.csv, cmb_deflation_pk.csv,
/// cmb_deflation_peaks.csv, cmb_deflation_closure.csv, cmb_deflation_gaps.csv,
/// cmb_deflation_convergence.csv, cmb_deflation_growth.csv, spectrum_checks.{txt,csv}.
pub fn run_spectrum(main_data_dir: &str) -> std::io::Result<bool> {
    let t0 = Instant::now();
    let mut rep = Report::new();
    let fid = Params::fiducial();
    let h0 = fid.h;
    let l_max = 2500;
    let ws = Workspace::new(l_max);
    let none = Deflation::none();

    // ---- (1) without deflation: the copy against the program of the main project -----------------
    let s_main = spacetimes::spectra::compute(&fid, &ws, 1.0);
    println!("[{:>5.0} s] the program of the main project, fiducial model", elapsed(&t0));
    let s_zero = compute(&fid, &none, &ws, 1.0);
    let s_zero_p = compute(&fid, &Deflation { closure: Closure::Pressure, ..none }, &ws, 1.0);
    println!("[{:>5.0} s] its copy without deflation (both closures)", elapsed(&t0));
    let bits = same_bits(&s_main, &s_zero) && same_bits(&s_main, &s_zero_p);
    // the file of the main project, to the digits written there
    let file = read_numbers(&format!("{}/cmb_fiducial_cl.csv", main_data_dir));
    let (file_ok, file_dev) = match &file {
        Some(rows) => {
            let mut dev = 0.0f64;
            for r in rows {
                let l = r[0] as usize;
                if l <= l_max {
                    dev = dev.max((r[1] / s_main.tt[l] - 1.0).abs()).max(((r[2] - s_main.te[l]) / (s_main.tt[l] * s_main.ee[l]).sqrt()).abs()).max((r[3] / s_main.ee[l] - 1.0).abs());
                }
            }
            (rows.len() == l_max - 1 && dev < 1e-9, dev)
        }
        None => (false, f64::NAN),
    };
    rep.add("def-E1", "exact",
        &format!("the Einstein-Boltzmann modules of the revision (copies of four modules of the main project with a listed set of changes, written by code/scripts/vendor_eb.py) reproduce the program of the main project when the rates of deflation are zero, for both closures: the spectra TT, TE and EE for 2 <= l <= {}, the power spectrum of matter at {} wavenumbers, sigma_8, the angle theta_* and the sound horizon agree bit for bit; and the spectra agree with the file data/cmb_fiducial_cl.csv of the main project to the digits written there (largest relative deviation {:.1e})", l_max, s_main.pk.len(), file_dev),
        bits && file_ok);

    // ---- (2) the spectra on the deflating spacetime, at a fixed angle theta_* ----------------------
    let (r_llr, r_bbn) = allowed_rates();
    let rates = [r_llr, 0.01, 0.03, r_bbn, 0.1];
    let theta_fid = s_main.thermo.theta_star_100;
    let mock = Mock { cfg: Config::planck_like(), data: s_main.clone(), tau0: fid.tau_reio };
    let z_star_fid = s_main.thermo.z_star;
    // (rate, parameters, spectra with the closure "Momentum", spectra with the closure "Pressure")
    let mut models: Vec<(f64, Params, Spectra, Spectra)> = Vec::new();
    let scols = ["gsum_over_H0", "gsum_per_yr", "H0", "dH0", "H0_fixed_zstar", "theta_star_100", "z_star", "rs_star", "z_drag", "rs_drag", "GN_star", "GN_early", "Y_He", "age_Gyr", "tau0_Mpc",
                 "sigma_8_M", "sigma_8_P", "max_rel_TT", "cv_max", "cv_rss", "chi2_planck_M", "chi2_planck_P",
                 "closure_cv_low", "closure_cv_high", "closure_over_effect_low", "closure_over_effect_high"];
    let mut srows: Vec<Vec<f64>> = Vec::new();
    let summary = |r: f64, p: &Params, sm: &Spectra, sp: &Spectra| -> Vec<f64> {
        let d = deflation(r, Closure::Momentum);
        let bg = Background::new(p, &d);
        let th = Thermo::new(&bg);
        // the Hubble constant with the redshift of last scattering and the helium fraction held at their values without deflation
        let h_fix = if r == 0.0 { h0 } else { solve_h(&Model::fiducial(r * h0, h0), z_star_fid, spacetimes_deflation_cjt_ks::cosmo_defl::theta_star(&Model::fiducial(0.0, h0), z_star_fid, &|_| 1.0).theta100, &|_| 1.0) };
        let (v_max, v_rss) = variance_units(sm, &s_main, 30, l_max);
        // the two closures against each other, below and above l = 30: largest difference in units of the cosmic
        // variance, and the root of the sum of squares relative to that of the effect of the deflation itself
        let (lo_max, lo_rss) = variance_units(sm, sp, 2, 29);
        let (hi_max, hi_rss) = variance_units(sm, sp, 30, l_max);
        let (_, eff_lo) = variance_units(sm, &s_main, 2, 29);
        let closure = if r == 0.0 { [0.0, 0.0, f64::NAN, f64::NAN] } else { [lo_max, hi_max, lo_rss / eff_lo, hi_rss / v_rss] };
        let mut row = vec![r, r * h0 / HUBBLE_TIME_100_YR, p.h * KMS, (p.h - h0) * KMS, h_fix * KMS, sm.thermo.theta_star_100, sm.thermo.z_star, sm.thermo.rs_star, sm.thermo.z_drag, sm.thermo.rs_drag,
             bg.s_of_a(1.0 / (1.0 + sm.thermo.z_star)).exp(), bg.s_early().exp(), th.y_he, bg.age() * HUBBLE_TIME_100_YR / 1e9, sm.thermo.tau0, sm.sigma8, sp.sigma8,
             max_over(30, l_max, &|l| sm.tt[l] / s_main.tt[l] - 1.0), v_max, v_rss, chi2(&mock, &sm.tt, &sm.te, &sm.ee, p.tau_reio), chi2(&mock, &sp.tt, &sp.te, &sp.ee, p.tau_reio)];
        row.extend_from_slice(&closure);
        row
    };
    srows.push(summary(0.0, &fid, &s_zero, &s_zero_p));
    for &r in &rates {
        let h = h_at_fixed_theta(&fid, &deflation(r, Closure::Momentum), theta_fid);
        let p = Params { h, ..fid };
        let sm = compute(&p, &deflation(r, Closure::Momentum), &ws, 1.0);
        let sp = compute(&p, &deflation(r, Closure::Pressure), &ws, 1.0);
        println!("[{:>5.0} s] gsum = {:.5} H_0: h = {:.5}, 100 theta_* = {:.6}, z_* = {:.3}, sigma_8 = {:.5} (Momentum), {:.5} (Pressure)", elapsed(&t0), r, h, sm.thermo.theta_star_100, sm.thermo.z_star, sm.sigma8, sp.sigma8);
        srows.push(summary(r, &p, &sm, &sp));
        models.push((r, p, sm, sp));
    }
    write_csv("cmb_deflation_summary.csv",
        &[("content", "the models whose spectra are computed: three equal rates of deflation with the total rate gsum, Newton's constant G_N(x4) = G_N(today) exp(gsum (x4 - x4_today)) (assumptions A1, A2), perturbations under assumption A3 with the closure \"Momentum\" (suffix _M) or \"Pressure\" (suffix _P); the Hubble constant of each model is the one for which the angle theta_* of the sound horizon at last scattering has its value without deflation; the densities of baryons and of cold dark matter, the primordial amplitude and tilt and the optical depth are those of the fiducial model of the main project".to_string()),
          ("columns", "gsum_over_H0: gsum in units of 67.36 km/s/Mpc; gsum_per_yr; H0: Hubble constant in km/s/Mpc; dH0: its change; H0_fixed_zstar: the Hubble constant that the same condition gives if the redshift of last scattering and the helium fraction are held at their values without deflation; theta_star_100; z_star; rs_star (Mpc); z_drag; rs_drag (Mpc); GN_star, GN_early: Newton's constant at last scattering and in the early universe over its value today; Y_He: helium fraction (it responds to Newton's constant at nucleosynthesis); age_Gyr; tau0_Mpc: conformal age; sigma_8_M, sigma_8_P; max_rel_TT: largest |TT/TT_0 - 1| for 30 <= l <= 2500 against the model without deflation; cv_max, cv_rss: largest difference of TT in units of the cosmic variance of one multipole, and the root of the sum of squares over 30 <= l <= 2500 (the last three for the closure \"Momentum\"); chi2_planck_M, chi2_planck_P: chi^2 of the model against mock data of the fiducial model for the Planck-like data set of the main project, all other parameters fixed; closure_cv_low, closure_cv_high: largest difference between the temperature spectra of the two closures in units of the cosmic variance of one multipole, for 2 <= l <= 29 and for 30 <= l <= 2500; closure_over_effect_low, closure_over_effect_high: that difference relative to the effect of the deflation itself (roots of the sums of squares over the same ranges)".to_string()),
          ("command", "deflation_cjt_ks spectrum".to_string())],
        &scols, &srows)?;
    // spectra
    let mut crows: Vec<Vec<f64>> = Vec::new();
    for l in 2..=l_max {
        let mut row = vec![l as f64, s_zero.tt[l], s_zero.te[l], s_zero.ee[l]];
        for (_, _, sm, sp) in &models {
            row.extend_from_slice(&[sm.tt[l], sm.te[l], sm.ee[l], sp.tt[l], sp.te[l], sp.ee[l]]);
        }
        crows.push(row);
    }
    let mut chead: Vec<String> = vec!["ell".into(), "TT_0".into(), "TE_0".into(), "EE_0".into()];
    for i in 1..=models.len() {
        for tag in ["M", "P"] {
            chead.extend([format!("TT_{}_{}", i, tag), format!("TE_{}_{}", i, tag), format!("EE_{}_{}", i, tag)]);
        }
    }
    let chead_ref: Vec<&str> = chead.iter().map(|s| s.as_str()).collect();
    let model_list = format!("0: no deflation; 1 to {}: gsum/H_0 = {}", models.len(), models.iter().map(|m| format!("{:.6}", m.0)).collect::<Vec<_>>().join(", "));
    write_csv("cmb_deflation_cl.csv",
        &[("content", "angular power spectra of the microwave background on the deflating spacetime (without gravitational lensing), D_l = l(l+1) C_l/(2 pi) in microkelvin^2, for the models of data/cmb_deflation_summary.csv and the two closures (suffix _M: \"Momentum\", _P: \"Pressure\")".to_string()),
          ("models", model_list.clone()),
          ("command", "deflation_cjt_ks spectrum".to_string())],
        &chead_ref, &crows)?;
    let mut prow: Vec<Vec<f64>> = Vec::new();
    for i in 0..s_zero.k_pk.len() {
        let mut row = vec![s_zero.k_pk[i], s_zero.pk[i]];
        for (_, _, sm, sp) in &models {
            row.push(sm.pk[i]);
            row.push(sp.pk[i]);
        }
        prow.push(row);
    }
    let mut phead: Vec<String> = vec!["k".into(), "P_0".into()];
    for i in 1..=models.len() {
        phead.push(format!("P_{}_M", i));
        phead.push(format!("P_{}_P", i));
    }
    let phead_ref: Vec<&str> = phead.iter().map(|s| s.as_str()).collect();
    write_csv("cmb_deflation_pk.csv",
        &[("content", "linear power spectrum of matter today, P(k) in Mpc^3 at the wavenumbers k in 1/Mpc, for the models of data/cmb_deflation_summary.csv and the two closures (same numbering and suffixes as in data/cmb_deflation_cl.csv)".to_string()),
          ("models", model_list.clone()),
          ("command", "deflation_cjt_ks spectrum".to_string())],
        &phead_ref, &prow)?;
    // the acoustic peaks of the temperature spectrum
    let pk0 = peaks(&s_zero.tt, 100, 1600);
    let mut pkrows: Vec<Vec<f64>> = Vec::new();
    let mut peak_shift = 0.0f64;
    let mut peak_height = vec![0.0f64; models.len()];
    for j in 0..5.min(pk0.len()) {
        let mut row = vec![(j + 1) as f64, pk0[j].0, pk0[j].1];
        for (i, (_, _, sm, _)) in models.iter().enumerate() {
            let pk = peaks(&sm.tt, 100, 1600);
            row.push(pk[j].0);
            row.push(pk[j].1);
            peak_shift = peak_shift.max((pk[j].0 - pk0[j].0).abs());
            peak_height[i] = peak_height[i].max((pk[j].1 / pk0[j].1 - 1.0).abs());
        }
        pkrows.push(row);
    }
    let mut pkhead: Vec<String> = vec!["peak".into(), "ell_0".into(), "Dl_0".into()];
    for i in 1..=models.len() {
        pkhead.push(format!("ell_{}", i));
        pkhead.push(format!("Dl_{}", i));
    }
    let pkhead_ref: Vec<&str> = pkhead.iter().map(|s| s.as_str()).collect();
    write_csv("cmb_deflation_peaks.csv",
        &[("content", "positions and heights (microkelvin^2) of the first five acoustic peaks of the temperature spectrum, for the models of data/cmb_deflation_summary.csv (closure \"Momentum\"; without gravitational lensing)".to_string()),
          ("models", model_list.clone()),
          ("command", "deflation_cjt_ks spectrum".to_string())],
        &pkhead_ref, &pkrows)?;
    let col = |name: &str| -> usize { scols.iter().position(|c| *c == name).unwrap() };
    let theta_dev = srows.iter().map(|r| (r[col("theta_star_100")] / theta_fid - 1.0).abs()).fold(0.0, f64::max);
    let falls = srows.windows(2).all(|w| w[1][col("H0")] < w[0][col("H0")]);
    let under: Vec<f64> = srows.iter().skip(1).map(|r| 1.0 - (h0 * KMS - r[col("H0_fixed_zstar")]) / (h0 * KMS - r[col("H0")])).collect();
    let (under_lo, under_hi) = (under.iter().cloned().fold(f64::INFINITY, f64::min), under.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let i_bbn = 1 + rates.iter().position(|r| *r == r_bbn).unwrap();
    let i_llr = 1 + rates.iter().position(|r| *r == r_llr).unwrap();
    rep.add("def-E2", "numerical",
        &format!("spectra at a fixed angle of the sound horizon (assumptions A1, A2, A3; table data/cmb_deflation_summary.csv). With the full calculation of recombination on the deflating background the Hubble constant that keeps 100 theta_* = {:.5} (kept to {:.0e}) is {:.3}, {:.3}, {:.3}, {:.3}, {:.3} km/s/Mpc for gsum = {:.5}, {}, {}, {:.5}, {} H_0 (H_0 = 67.36 km/s/Mpc): it falls as the rate grows. Last scattering moves from z_* = {:.2} to {:.2} at the largest rate, and the helium fraction falls from {:.5} to {:.5}. If z_* and the helium fraction are held at their values without deflation, the fall of the Hubble constant is underestimated by {:.1} to {:.1} per cent of itself. The acoustic peaks stay where they are (the first five move by at most {:.2} in l); their heights change by at most {:.2} per cent at the rate of lunar laser ranging and {:.1} per cent at the rate of nucleosynthesis (data/cmb_deflation_peaks.csv)", theta_fid, theta_dev, srows[1][col("H0")], srows[2][col("H0")], srows[3][col("H0")], srows[4][col("H0")], srows[5][col("H0")], rates[0], rates[1], rates[2], rates[3], rates[4], srows[0][col("z_star")], srows[5][col("z_star")], srows[0][col("Y_He")], srows[5][col("Y_He")], 100.0 * under_lo, 100.0 * under_hi, peak_shift, 100.0 * peak_height[i_llr - 1], 100.0 * peak_height[i_bbn - 1]),
        theta_dev < 1e-9 && falls && under_lo > 0.0 && under_hi < 0.12 && peak_shift < 1.5);
    rep.add("def-E3", "numerical",
        &format!("what remains in the spectra when the angle is kept. At the rate that lunar laser ranging allows (gsum = {:.5} H_0) the temperature spectrum differs from that of the model without deflation by at most {:.1e} of itself for 30 <= l <= {} ({:.3} of the cosmic variance of a single multipole); chi^2 against Planck-like mock data with all other parameters fixed is {:.2} (closure \"Momentum\") and {:.2} (\"Pressure\"), less than the 3.84 of a deviation of two standard deviations: such data cannot tell this rate from no deflation. At the rate that nucleosynthesis allows (gsum = {:.5} H_0, Newton's constant at last scattering {:.4} of today's) the difference reaches {:.3} of the spectrum and chi^2 = {:.0}: the microwave background is then the stronger test (command `forecast`)", r_llr, srows[i_llr][col("max_rel_TT")], l_max, srows[i_llr][col("cv_max")], srows[i_llr][col("chi2_planck_M")], srows[i_llr][col("chi2_planck_P")], r_bbn, srows[i_bbn][col("GN_star")], srows[i_bbn][col("max_rel_TT")], srows[i_bbn][col("chi2_planck_M")]),
        srows[i_llr][col("chi2_planck_M")] < 3.84 && srows[i_llr][col("chi2_planck_P")] < 3.84 && srows[i_bbn][col("chi2_planck_M")] > 100.0 && srows.windows(2).all(|w| w[1][col("chi2_planck_M")] > w[0][col("chi2_planck_M")]));

    // ---- (3) the closure of the perturbation equations ----------------------------------------------
    let mut cl_stats: Vec<[f64; 9]> = Vec::new();
    let mut clrows: Vec<Vec<f64>> = (2..=l_max).map(|l| vec![l as f64, s_zero.tt[l]]).collect();
    let mut clhead: Vec<String> = vec!["ell".into(), "TT_0".into()];
    for (i, (r, p, sm, sp)) in models.iter().enumerate() {
        // differences between the closures, and the effect of the deflation itself, in units of the cosmic variance
        let (lo_max, lo_rss) = variance_units(sm, sp, 2, 29);
        let (hi_max, hi_rss) = variance_units(sm, sp, 30, l_max);
        let (_, eff_lo) = variance_units(sm, &s_zero, 2, 29);
        let (_, eff_hi) = variance_units(sm, &s_zero, 30, l_max);
        let (cm, cp) = (chi2(&mock, &sm.tt, &sm.te, &sm.ee, p.tau_reio), chi2(&mock, &sp.tt, &sp.te, &sp.ee, p.tau_reio));
        cl_stats.push([*r, lo_max, hi_max, lo_rss / eff_lo, hi_rss / eff_hi, cm, cp, sm.sigma8 / s_zero.sigma8, sp.sigma8 / s_zero.sigma8]);
        for (j, l) in (2..=l_max).enumerate() {
            clrows[j].push(sm.tt[l] - sp.tt[l]);
            clrows[j].push(sm.tt[l] - s_zero.tt[l]);
        }
        clhead.push(format!("closure_{}", i + 1));
        clhead.push(format!("effect_{}", i + 1));
    }
    let clhead_ref: Vec<&str> = clhead.iter().map(|s| s.as_str()).collect();
    write_csv("cmb_deflation_closure.csv",
        &[("content", "dependence of the temperature spectrum on the closure of the perturbation equations (assumption A3): closure_i = D_l^TT with the closure \"Momentum\" minus D_l^TT with the closure \"Pressure\", and for comparison effect_i = the effect of the deflation itself (closure \"Momentum\" minus no deflation, both at the same angle theta_*), in microkelvin^2, for the models i of data/cmb_deflation_summary.csv".to_string()),
          ("models", model_list.clone()),
          ("command", "deflation_cjt_ks spectrum".to_string())],
        &clhead_ref, &clrows)?;
    let (cb, cl) = (cl_stats[i_bbn - 1], cl_stats[i_llr - 1]);
    rep.add("def-E4", "numerical",
        &format!("dependence on the closure of the perturbation equations (\"Momentum\": the agent that holds the extra scale factors carries no momentum density; \"Pressure\": it carries no pressure perturbation; file data/cmb_deflation_closure.csv). For 30 <= l <= {}, the multipoles that the forecast uses, the temperature spectra of the two closures differ by at most {:.1e} and {:.1e} of the cosmic variance of a multipole at gsum = {:.5} and {:.5} H_0, which is {:.1e} and {:.1e} of the effect of the deflation itself (root of the sum of squares); chi^2 against Planck-like mock data is {:.3} against {:.3}, and {:.2} against {:.2}. The spectra at these multipoles are a prediction of assumptions A1 and A2 alone. For l < 30, where the potentials of today act (late integrated Sachs-Wolfe effect), the closures differ by up to {:.3} and {:.3} of the cosmic variance, {:.2} and {:.2} of the effect; and the amplitude sigma_8 of matter is {:.4} of its value without deflation in the closure \"Momentum\" against {:.4} in the closure \"Pressure\" at gsum = {:.5} H_0 ({:.5} against {:.5} at {:.5} H_0). There the result depends on the closure and is not a prediction of the hypotheses", l_max, cl[2], cb[2], r_llr, r_bbn, cl[4], cb[4], cl[5], cl[6], cb[5], cb[6], cl[1], cb[1], cl[3], cb[3], cb[7], cb[8], r_bbn, cl[7], cl[8], r_llr),
        cl_stats.iter().all(|c| c[4] < 0.02 && c[2] < 0.05 && (c[5] / c[6] - 1.0).abs() < 0.01) && (cb[7] - cb[8]).abs() > 1e-3);

    // ---- (4) what the agent supplies; numerical checks of the identities ------------------------------
    let taus = [280.0, 800.0, 3000.0, 9000.0, 13000.0];
    let mut grows: Vec<Vec<f64>> = Vec::new();
    let (mut ident, mut consist) = (0.0f64, 0.0f64);
    let (mut ratio_rec, mut ratio_late) = (0.0f64, 0.0f64);
    // today a = 1 and Hc is the Hubble constant of the model: a gsum/Hc = gsum/H_0(model)
    let ratio_today = models[i_bbn - 1].0 * H_FID / models[i_bbn - 1].1.h;
    let mut coef: [[f64; 2]; 4] = [[f64::INFINITY, 0.0]; 4];
    let mut inside = [f64::INFINITY, 0.0f64];
    {
        let (r, p, _, _) = &models[i_bbn - 1];
        for &k in &[0.005, 0.02, 0.08] {
            for (ci, (closure, _)) in CLOSURES.iter().enumerate() {
                let res = potentials(p, &deflation(*r, *closure), k, &taus, 1e-8);
                for (i, x) in res.iter().enumerate() {
                    let ratio = x.a_gsum / x.hub;
                    let g = [x.shear_gap / (ratio * x.potential.abs()), x.trace_gap / (ratio * x.trace_scale), x.momentum_gap / (ratio * x.momentum_scale)];
                    let c = [x.shear_check.abs() / x.potential.abs(), x.trace_check.abs() / x.trace_scale];
                    ident = ident.max(c[0]).max(c[1]);
                    consist = consist.max((x.difference - x.expression).abs() / (x.difference.abs() + x.expression.abs() + 5e-7));
                    grows.push(vec![k, ci as f64, taus[i], ratio, x.potential, x.difference, x.expression, g[0], g[1], g[2], c[0], c[1]]);
                    if i == 0 {
                        ratio_rec = ratio;
                    }
                    if i == taus.len() - 1 {
                        ratio_late = ratio;
                    }
                    if taus[i] >= 800.0 {
                        // 0: shear gap, Momentum; 1: trace gap, Momentum; 2: momentum gap, Pressure; 3: shear gap, Pressure, times 1 + (k/Hc)^2
                        let list: Vec<(usize, f64)> = if ci == 0 { vec![(0, g[0].abs()), (1, g[1].abs())] } else { vec![(2, g[2].abs()), (3, g[0].abs() * (1.0 + (k / x.hub).powi(2)))] };
                        for (q, v) in list {
                            coef[q][0] = coef[q][0].min(v);
                            coef[q][1] = coef[q][1].max(v);
                        }
                        if ci == 1 && k / x.hub > 20.0 {
                            inside[0] = inside[0].min(g[0].abs());
                            inside[1] = inside[1].max(g[0].abs());
                        }
                    }
                }
            }
        }
    }
    write_csv("cmb_deflation_gaps.csv",
        &[("content", format!("what the agent supplies (assumption A3), at gsum = {:.5} H_0: with a Newton's constant that depends on the time the momentum constraint, the trace equation and the equation of the shear of ordinary space cannot all hold with matter and radiation alone. The gaps are what each equation lacks: momentum gap = 2 k^2 eta' - 8 pi G_N a^2 (rho + P) theta, trace gap = h'' + 2 He h' - 2 k^2 eta + 24 pi G_N a^2 delta P, shear gap = alpha' + 2 He alpha - eta + 12 pi G_N a^2 (rho + P) sigma/k^2, with He = Hc - a gsum/2. The table gives them in units of (a gsum/Hc) times the size of the terms of the equation", r_bbn)),
          ("columns", "k: wavenumber in 1/Mpc; closure: 0 = \"Momentum\", 1 = \"Pressure\"; tau_cf: conformal time in Mpc; a_gsum_over_Hc; potential: eta + alpha', the sum of the two gravitational potentials (unit curvature perturbation); difference: its derivative by a central difference; expression: eta' + alpha'', the quantity that the source of the temperature uses; shear_gap: in units of (a gsum/Hc) |potential|; trace_gap: in units of (a gsum/Hc) (k^2 + Hc^2) |potential|; momentum_gap: in units of (a gsum/Hc) k^2 Hc |potential|; shear_check, trace_check: numerical checks of the two identities from which the program computes the gaps (alpha' and h'' by differences along the solution), relative to |potential| and to (k^2 + Hc^2) |potential|".to_string()),
          ("command", "deflation_cjt_ks spectrum".to_string())],
        &["k", "closure", "tau_cf", "a_gsum_over_Hc", "potential", "difference", "expression", "shear_gap", "trace_gap", "momentum_gap", "shear_check", "trace_check"], &grows)?;
    rep.add("def-E5", "numerical",
        &format!("what the agent supplies (file data/cmb_deflation_gaps.csv; gsum = {:.5} H_0, three wavenumbers, five times). Each closure is a consistent theory: the quantity eta' + alpha'' that the source of the temperature uses equals the derivative of the two potentials along the solution (largest relative difference {:.1e}), and the two identities from which the program computes the gaps hold along the solution to {:.1e}, the accuracy of a numerical derivative. After recombination the gaps are of the first order in the rate: in units of (a gsum/Hc) times the terms of their equations, the closure \"Momentum\" lacks {:.2} to {:.2} in the equation of the shear and {:.2} to {:.2} in the trace equation; the closure \"Pressure\" lacks {:.2} to {:.2} in the momentum constraint, and in the equation of the shear {:.2} to {:.2} times (Hc/k)^2 (far inside the horizon, k > 20 Hc: {:.1e} to {:.1e}): there the closure \"Pressure\" satisfies the equation of the shear as well, and is the closure \"Shear\". The ratio a gsum/Hc = gsum/H is {:.1e} at the first of the five times (recombination), {:.1e} at the last (conformal time {:.0} Mpc) and {:.1e} today: the closure matters only for what happens late", r_bbn, consist, ident, coef[0][0], coef[0][1], coef[1][0], coef[1][1], coef[2][0], coef[2][1], coef[3][0], coef[3][1], inside[0], inside[1], ratio_rec, ratio_late, taus[taus.len() - 1], ratio_today),
        consist < 2e-3 && ident < 2e-3 && coef.iter().all(|c| c[0] > 0.02 && c[1] < 20.0) && inside[1] < 0.02 && ratio_rec < 1e-5 && ratio_late < 0.2);

    // ---- (5) numerical convergence ----------------------------------------------------------------------
    {
        let (r, p, _, _) = &models[2];
        let mut meta: Vec<(&str, String)> = vec![
            ("content", format!("numerical convergence of the spectra on the deflating spacetime, gsum = {} H_0, for the two closures: accuracy setting 1 (used everywhere) relative to setting 2 (all tolerances and wavenumber steps halved)", r)),
            ("columns", "rel_TT = TT_1/TT_2 - 1; rel_EE likewise; dTE = (TE_1 - TE_2)/sqrt(TT_2 EE_2); suffix _M: closure \"Momentum\", _P: \"Pressure\"".to_string()),
        ];
        let mut rows: Vec<Vec<f64>> = (2..=l_max).map(|l| vec![l as f64]).collect();
        let mut worst = [0.0f64; 7];
        for (ci, (closure, tag)) in CLOSURES.iter().enumerate() {
            let d = deflation(*r, *closure);
            let s1 = if ci == 0 { &models[2].2 } else { &models[2].3 };
            let s2 = compute(p, &d, &ws, 2.0);
            println!("[{:>5.0} s] accuracy setting 2 at gsum = {} H_0, closure {}", elapsed(&t0), r, tag);
            let rel_tt = |l: usize| s1.tt[l] / s2.tt[l] - 1.0;
            let rel_ee = |l: usize| s1.ee[l] / s2.ee[l] - 1.0;
            let d_te = |l: usize| (s1.te[l] - s2.te[l]) / (s2.tt[l] * s2.ee[l]).sqrt();
            for (j, l) in (2..=l_max).enumerate() {
                rows[j].extend_from_slice(&[rel_tt(l), d_te(l), rel_ee(l)]);
            }
            let m = [max_over(100, l_max, &rel_tt), max_over(100, l_max, &d_te), max_over(100, l_max, &rel_ee), max_over(30, 99, &rel_tt), max_over(30, 99, &d_te), max_over(30, 99, &rel_ee), (s1.sigma8 / s2.sigma8 - 1.0).abs()];
            for q in 0..7 {
                worst[q] = worst[q].max(m[q]);
            }
            meta.push((if ci == 0 { "maxima_M" } else { "maxima_P" }, format!("100 <= l <= 2500: TT {:.3e}, TE {:.3e}, EE {:.3e}; 30 <= l <= 99: TT {:.3e}, TE {:.3e}, EE {:.3e}; sigma_8: {:.6} (setting 1), {:.6} (setting 2)", m[0], m[1], m[2], m[3], m[4], m[5], s1.sigma8, s2.sigma8)));
        }
        meta.push(("command", "deflation_cjt_ks spectrum".to_string()));
        write_csv("cmb_deflation_convergence.csv", &meta, &["ell", "rel_TT_M", "dTE_M", "rel_EE_M", "rel_TT_P", "dTE_P", "rel_EE_P"], &rows)?;
        // the numbers of the main project, from its file
        let main_conv = read_numbers(&format!("{}/cmb_convergence.csv", main_data_dir));
        let main_max = match &main_conv {
            Some(rows) => {
                let mx = |lo: usize, hi: usize, c: usize| rows.iter().filter(|r| r[0] >= lo as f64 && r[0] <= hi as f64).map(|r| r[c].abs()).fold(0.0, f64::max);
                [mx(100, l_max, 1), mx(100, l_max, 2), mx(100, l_max, 3), mx(30, 99, 1), mx(30, 99, 2), mx(30, 99, 3)]
            }
            None => [f64::NAN; 6],
        };
        rep.add("def-E6", "numerical",
            &format!("numerical convergence on the deflating spacetime (gsum = {} H_0, both closures; file data/cmb_deflation_convergence.csv): halving all tolerances and wavenumber steps changes the spectra by at most {:.1e} (TT), {:.1e} (TE, in units of sqrt(TT EE)), {:.1e} (EE) for 100 <= l <= {}, and by {:.1e}, {:.1e}, {:.1e} for 30 <= l <= 99; sigma_8 changes by {:.1e}. The program of the main project has {:.1e}, {:.1e}, {:.1e} and {:.1e}, {:.1e}, {:.1e} for its fiducial model (its file data/cmb_convergence.csv); the requirements are those of the main project (5e-4 for l >= 100; 2e-3 for TT and 1e-2 for TE and EE below)", r, worst[0], worst[1], worst[2], l_max, worst[3], worst[4], worst[5], worst[6], main_max[0], main_max[1], main_max[2], main_max[3], main_max[4], main_max[5]),
            worst[0] < 5e-4 && worst[1] < 5e-4 && worst[2] < 5e-4 && worst[3] < 2e-3 && worst[4] < 1e-2 && worst[5] < 1e-2 && worst[6] < 1e-4 && main_max.iter().all(|x| x.is_finite()));
    }

    // ---- (6) the helium fraction -----------------------------------------------------------------------
    {
        let (r, p, s, _) = &models[i_bbn - 1];
        let off = Deflation { bbn: false, ..deflation(*r, Closure::Momentum) };
        // the angle depends on the helium fraction through recombination: keep the angle
        let h_off = h_at_fixed_theta(&fid, &off, theta_fid);
        let p_off = Params { h: h_off, ..*p };
        let s_off = compute(&p_off, &off, &ws, 1.0);
        println!("[{:>5.0} s] helium fraction of the model without deflation, gsum = {:.5} H_0", elapsed(&t0), r);
        let (y_on, y_off) = (s.thermo.y_he, s_off.thermo.y_he);
        let dn = 43.0 / 7.0 * (Background::new(p, &deflation(*r, Closure::Momentum)).s_early().exp() - 1.0);
        let rel = max_over(30, l_max, &|l| s.tt[l] / s_off.tt[l] - 1.0);
        let (c_on, c_off) = (chi2(&mock, &s.tt, &s.te, &s.ee, p.tau_reio), chi2(&mock, &s_off.tt, &s_off.te, &s_off.ee, p.tau_reio));
        rep.add("def-E7", "numerical",
            &format!("the helium fraction. Newton's constant at nucleosynthesis was {:.4} of today's at gsum = {:.5} H_0; the slower expansion is equivalent to {:.3} neutrino species, and the helium fraction of standard nucleosynthesis falls from {:.5} to {:.5}. If this is ignored (helium fraction {:.5}), the temperature spectrum changes by at most {:.1e} of itself for 30 <= l <= {}, the Hubble constant at fixed angle by {:.3} km/s/Mpc, and chi^2 against Planck-like mock data (other parameters fixed) from {:.1} to {:.1}", srows[i_bbn][col("GN_early")], r, dn, y_off, y_on, y_off, rel, l_max, (h_off - p.h).abs() * KMS, c_on, c_off),
            y_on < y_off && dn < 0.0 && rel < 0.02);
    }

    // ---- (7) the growth of structure ----------------------------------------------------------------------
    {
        // the growth of the density contrast of matter from a = 0.01 to today, at k = 0.1/Mpc, from the
        // Einstein-Boltzmann equations with each closure, against the equation of that closure for
        // pressureless matter (cosmo_defl::growth, which the command `matter` uses)
        let k = 0.1;
        let ode_growth = |r: f64, h: f64, law: GrowthLaw| -> f64 {
            let m = Model { h, ..Model::fiducial(r * h0, h0) };
            let (a_min, n) = grid(z_star_fid, 4000);
            let hist = m.history(a_min, n, &|_| 1.0);
            growth(&m, &hist, 0.01, &|_| 1.0, law).0
        };
        let laws = [GrowthLaw::Momentum, GrowthLaw::Pressure];
        let g0_eb = growth_factor(&fid, &none, k, 0.01);
        let g0_ode = [ode_growth(0.0, h0, laws[0]), ode_growth(0.0, h0, laws[1])];
        let mut grows: Vec<Vec<f64>> = vec![vec![0.0, h0 * KMS, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]];
        let mut dev = 0.0f64;
        let mut cross = f64::INFINITY;
        for (r, p, sm, sp) in &models {
            let mut row = vec![*r, p.h * KMS];
            for (ci, (closure, _)) in CLOSURES.iter().enumerate() {
                let d = deflation(*r, *closure);
                // at the Hubble constant of the model (fixed angle), and at the Hubble constant held at 67.36 km/s/Mpc
                let (eb, ode) = (growth_factor(p, &d, k, 0.01) / g0_eb, ode_growth(*r, p.h, laws[ci]) / g0_ode[ci]);
                let (eb_h, ode_h) = (growth_factor(&fid, &d, k, 0.01) / g0_eb, ode_growth(*r, h0, laws[ci]) / g0_ode[ci]);
                dev = dev.max((eb / ode - 1.0).abs()).max((eb_h / ode_h - 1.0).abs());
                // the equation of the other closure does not describe this one
                cross = cross.min((eb_h / (ode_growth(*r, h0, laws[1 - ci]) / g0_ode[1 - ci]) - 1.0).abs() / r);
                row.extend_from_slice(&[eb, ode, eb_h, ode_h]);
            }
            row.extend_from_slice(&[sm.sigma8, sm.sigma8 / s_zero.sigma8, sp.sigma8, sp.sigma8 / s_zero.sigma8]);
            grows.push(row);
        }
        grows[0][10] = s_zero.sigma8;
        grows[0][12] = s_zero.sigma8;
        write_csv("cmb_deflation_growth.csv",
            &[("content", "growth of the density contrast of matter (cold dark matter and baryons) from a = 0.01 to today at the wavenumber k = 0.1/Mpc, relative to the model without deflation, for the two closures (suffix _M: \"Momentum\", _P: \"Pressure\"): from the Einstein-Boltzmann equations of the command `spectrum` (growth_EB) and from the growth equation of the closure for pressureless matter that the command `matter` integrates (growth_ODE); and the amplitude sigma_8 of the full calculation for the same primordial amplitude".to_string()),
              ("columns", "gsum_over_H0; H0: Hubble constant of the model in km/s/Mpc (fixed angle theta_*); growth_EB, growth_ODE: at that Hubble constant; growth_EB_fixed_H0, growth_ODE_fixed_H0: with the Hubble constant held at 67.36 km/s/Mpc; sigma_8; sigma_8_ratio: sigma_8 over its value without deflation".to_string()),
              ("command", "deflation_cjt_ks spectrum".to_string())],
            &["gsum_over_H0", "H0", "growth_EB_M", "growth_ODE_M", "growth_EB_fixed_H0_M", "growth_ODE_fixed_H0_M", "growth_EB_P", "growth_ODE_P", "growth_EB_fixed_H0_P", "growth_ODE_fixed_H0_P", "sigma_8_M", "sigma_8_ratio_M", "sigma_8_P", "sigma_8_ratio_P"], &grows)?;
        let (gl, gb) = (&grows[i_llr], &grows[i_bbn]);
        rep.add("def-E8", "numerical",
            &format!("the growth of structure (file data/cmb_deflation_growth.csv). From a = 0.01 to today, at k = 0.1/Mpc, the density contrast of matter in the Einstein-Boltzmann calculation grows by the factor that the growth equation of the same closure for pressureless matter gives: the ratios to the model without deflation agree to {:.1e} for the five rates and both closures, at the Hubble constant of each model and at a fixed Hubble constant; with the equation of the other closure they would disagree by at least {:.2} gsum/H_0. At a fixed Hubble constant the growth is {:.5} (closure \"Momentum\") and {:.5} (\"Pressure\") of its value without deflation at the rate of lunar laser ranging, {:.4} and {:.4} at the rate of nucleosynthesis; at a fixed angle, {:.5} and {:.5}, and {:.4} and {:.4}. The amplitude sigma_8 of the full calculation for the same primordial amplitude is {:.5} and {:.5}, and {:.4} and {:.4}, of its value without deflation ({:.5}): it contains, besides this growth, the change of the scale 8 Mpc/h with the Hubble constant and the evolution before a = 0.01 with a weaker gravity", dev, cross, gl[4], gl[8], gb[4], gb[8], gl[2], gl[6], gb[2], gb[6], gl[11], gl[13], gb[11], gb[13], s_zero.sigma8),
            dev < 1.5e-3 && cross > 0.1);
    }
    finish(&mut rep, &t0, "spectrum", "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the spectra of the microwave background on the deflating spacetime")
}

// =================================================================================================
// forecast: the rate of deflation as a parameter
// =================================================================================================

/// Row of derived numbers for a fitted model on the deflating background `d`.
fn fit_row(head: &[f64], f: &Fit, d: &Deflation, refreshed: f64) -> Vec<f64> {
    let p = &f.params;
    let t = &f.spectra.thermo;
    let om = (p.omega_b + p.omega_c) / (p.h * p.h);
    let bg = Background::new(p, d);
    let mut r = head.to_vec();
    r.extend_from_slice(&[f.chi2, p.omega_b, p.omega_c, p.h, (1e10 * p.a_s).ln(), p.n_s, p.tau_reio, 100.0 * p.h, f.spectra.sigma8,
        f.spectra.sigma8 * (om / 0.3).sqrt(), om, t.rs_drag, t.theta_s_100, t.z_eq, t.y_he, p.n_ur - 3.044 + p.dark_n_uv(), bg.s_of_a(1.0 / (1.0 + t.z_star)).exp(), f.evaluations as f64, refreshed]);
    r
}

const FIT_COLS: [&str; 19] = ["chi2", "omega_b", "omega_c", "h", "ln10A", "n_s", "tau_reio", "H0", "sigma8", "S8", "Omega_m", "rs_drag", "theta_s_100", "z_eq", "Y_He", "dN_bbn", "GN_star", "evaluations", "jacobian_refreshed"];

/// `forecast`: how well can mock data of the microwave background (the two data sets of
/// the main project) bound the rate of deflation, and what happens to the Hubble constant?
/// (1) Fisher matrices with the rate as the seventh parameter, for the two closures;
/// (2) fits of the six parameters of the standard model at fixed rates (profile); (3) the
/// error of using the derivatives of the fiducial point.  Writes data/deflation_fisher.csv,
/// deflation_profile.csv, deflation_chord_check.csv, deflation_residuals.csv,
/// forecast_checks.{txt,csv}.
pub fn run_forecast(main_data_dir: &str) -> std::io::Result<bool> {
    let t0 = Instant::now();
    let mut rep = Report::new();
    let ws = Workspace::new(2500);
    let fid = Params::fiducial();
    let h0 = fid.h;
    let none = Deflation::none();
    let data = compute(&fid, &none, &ws, 1.0);
    let jac0 = jacobian_defl(&fid, &none, &ws);
    println!("[{:>5.0} s] fiducial spectra and their derivatives with respect to the six parameters", elapsed(&t0));
    let v_fid = to_vec(&fid);
    let (r_llr, r_bbn) = allowed_rates();

    // ---- (1) derivative with respect to the rate, and Fisher matrices ----------------------------------
    // gam = gsum/H_100; the rates are positive by hypothesis: one-sided formula of second order with the step e
    let e = 0.002;
    let d_rate: Vec<[Vec<f64>; 3]> = CLOSURES.iter().map(|(c, _)| rate_derivative(&fid, &Deflation { closure: *c, ..none }, 0.0, e, &ws)).collect();
    // the same with half the step (closure "Momentum"), to measure the error of the difference formula
    let d_half = rate_derivative(&fid, &none, 0.0, 0.5 * e, &ws);
    println!("[{:>5.0} s] derivatives with respect to the rate (two closures, two steps)", elapsed(&t0));
    let configs = vec![Config::planck_like(), Config::planck_plus_deep()];
    let mut frows: Vec<Vec<f64>> = Vec::new();
    let mut sigma6_dev = 0.0f64;
    let main_fisher = read_numbers(&format!("{}/dr_fisher.csv", main_data_dir));
    let mut zero_ok = true;
    let mut slopes: Vec<Vec<f64>> = Vec::new();
    let mut mocks: Vec<Mock> = Vec::new();
    let mut step_dev = 0.0f64;
    for (ic, cfg) in configs.into_iter().enumerate() {
        let cfg_name = cfg.name;
        let mock = Mock { cfg, data: data.clone(), tau0: fid.tau_reio };
        // the fiducial model must have chi^2 = 0 exactly
        zero_ok &= chi2(&mock, &data.tt, &data.te, &data.ee, fid.tau_reio).abs() < 1e-9;
        let f6 = fisher(&mock, &jac0, &[]);
        let c6 = invert_spd(&f6).expect("Fisher matrix not positive definite");
        // against the file of the main project
        if let Some(rows) = &main_fisher {
            if let Some(row) = rows.iter().find(|r| r[0] == ic as f64 && r[1] == 0.0) {
                for i in 0..NPAR {
                    sigma6_dev = sigma6_dev.max((c6[i][i].sqrt() / row[4 + i] - 1.0).abs());
                }
            }
        }
        println!("\n=== mock data set '{}' ===", cfg_name);
        println!("LambdaCDM Fisher errors: {}", (0..NPAR).map(|i| format!("{} {:.3e}", PAR_NAMES[i], c6[i][i].sqrt())).collect::<Vec<_>>().join(", "));
        for (ci, (_, tag)) in CLOSURES.iter().enumerate() {
            let f7 = fisher(&mock, &jac0, &[d_rate[ci].clone()]);
            let c7 = invert_spd(&f7).expect("Fisher matrix not positive definite");
            // errors in units of gam = gsum/H_100; reported in units of H_0 = 67.36 km/s/Mpc
            let sigma = c7[6][6].sqrt() / h0;
            // error of the rate if the six parameters are held fixed
            let sigma_fixed = 1.0 / f7[6][6].sqrt() / h0;
            // shift of the six parameters per unit rate at fixed rate (conditional mean): -F6^-1 F_6r
            let rhs: Vec<f64> = (0..NPAR).map(|i| -f7[i][6]).collect();
            let slope = solve_spd(&f6, &rhs).unwrap();
            let mut row = vec![ic as f64, ci as f64, sigma, 1.96 * sigma, sigma_fixed, KMS * slope[2] * h0];
            for i in 0..NPAR {
                row.push(c6[i][i].sqrt());
            }
            for i in 0..NPAR {
                row.push(c7[i][i].sqrt());
            }
            for i in 0..NPAR {
                row.push(slope[i] * h0);
            }
            frows.push(row);
            println!("closure {}: Fisher sigma(gsum/H_0) = {:.5} (six parameters fixed: {:.5}); upper bound at 95 per cent: {:.5}; dH0/d(gsum/H_0) along the degeneracy = {:.3} km/s/Mpc; sigma(H0) with the rate free = {:.3}", tag, sigma, sigma_fixed, 1.96 * sigma, KMS * slope[2] * h0, KMS * c7[2][2].sqrt());
            if ci == 0 {
                slopes.push(slope);
                let fh = fisher(&mock, &jac0, &[d_half.clone()]);
                let ch = invert_spd(&fh).expect("Fisher matrix not positive definite");
                step_dev = step_dev.max((ch[6][6].sqrt() / c7[6][6].sqrt() - 1.0).abs());
            }
        }
        mocks.push(mock);
    }
    let mut fcols: Vec<String> = vec!["config".into(), "closure".into(), "sigma_rate".into(), "bound95_rate".into(), "sigma_rate_fixed".into(), "dH0_drate".into()];
    for pre in ["sigma6_", "sigma7_", "slope_"] {
        for nme in PAR_NAMES {
            fcols.push(format!("{}{}", pre, nme));
        }
    }
    let fcols_ref: Vec<&str> = fcols.iter().map(|s| s.as_str()).collect();
    write_csv("deflation_fisher.csv",
        &[("content", "Fisher-matrix forecasts for the mock data sets of the main project with the total rate of deflation gsum as a seventh parameter (three equal rates; assumptions A1, A2, A3): error of the rate; errors of the six LambdaCDM parameters without (sigma6_) and with (sigma7_) the rate as a free parameter; shift of the best-fit parameters per unit gsum/H_0 at fixed rate (slope_)".to_string()),
          ("config", "0 = Planck-like (f_sky 0.6, l = 30..2500 in T and 30..2000 in E); 1 = Planck-like plus a deep survey of 25 per cent of the sky (1.4 arcmin, 10 microkelvin arcmin)".to_string()),
          ("closure", "0 = \"Momentum\", 1 = \"Pressure\"".to_string()),
          ("columns", "sigma_rate: standard error of gsum/H_0 (H_0 = 67.36 km/s/Mpc); bound95_rate: 1.96 sigma_rate, the upper bound at 95 per cent for a parameter that cannot be negative and whose best value is zero; sigma_rate_fixed: the error if the six parameters are held fixed; dH0_drate: change of the best-fit Hubble constant in km/s/Mpc per unit gsum/H_0".to_string()),
          ("derivative", format!("the derivative of the spectra with respect to gsum/H_100 at zero rate is the one-sided formula of second order with the step {} H_100", e)),
          ("command", "deflation_cjt_ks forecast".to_string())],
        &fcols_ref, &frows)?;
    let fr = |ic: usize, ci: usize| frows.iter().find(|r| r[0] == ic as f64 && r[1] == ci as f64).unwrap().clone();
    rep.add("def-F1", "numerical",
        &format!("the mock data and the Fisher matrices of the main project, computed with the copy of its program: the fiducial model has chi^2 = 0, and the Fisher errors of the six LambdaCDM parameters agree with the file data/dr_fisher.csv of the main project to {:.1e} for both data sets", sigma6_dev),
        zero_ok && main_fisher.is_some() && sigma6_dev < 1e-9);
    rep.add("def-F2", "numerical",
        &format!("forecast for the rate of deflation (Fisher matrix; data/deflation_fisher.csv). With the six LambdaCDM parameters free, Planck-like data determine gsum/H_0 to {:.5} (one standard deviation; closure \"Momentum\") and {:.5} (\"Pressure\"); the upper bound at 95 per cent is gsum < {:.4} H_0 = {:.2e} per year. Adding the deep survey gives {:.5} and a bound of {:.4} H_0. With the six parameters held fixed the error would be {:.5}: most of the effect of the deflation on the spectra can be absorbed by the other parameters. The bound lies between that of lunar laser ranging ({:.5} H_0) and that of nucleosynthesis ({:.4} H_0): {:.1} times weaker than the first and {:.1} times stronger than the second. Halving the step of the derivative changes the error by {:.1e}", fr(0, 0)[2], fr(0, 1)[2], fr(0, 0)[3], fr(0, 0)[3] * h0 / HUBBLE_TIME_100_YR, fr(1, 0)[2], fr(1, 0)[3], fr(0, 0)[4], r_llr, r_bbn, fr(0, 0)[3] / r_llr, r_bbn / fr(0, 0)[3], step_dev),
        (fr(0, 0)[2] / fr(0, 1)[2] - 1.0).abs() < 0.01 && fr(1, 0)[2] < fr(0, 0)[2] && fr(0, 0)[4] < fr(0, 0)[2] && fr(0, 0)[3] > r_llr && fr(0, 0)[3] < r_bbn && step_dev < 0.02);
    rep.add("def-F3", "numerical",
        &format!("the Hubble constant along the degeneracy: per unit gsum/H_0 the best-fit Hubble constant changes by {:.2} km/s/Mpc (Planck-like) and {:.2} km/s/Mpc (with the deep survey), {:.3} and {:.3} km/s/Mpc at the bound of 95 per cent: it FALLS. (At a fixed angle theta_* and fixed densities the command `cosmology` finds {:.2} km/s/Mpc per unit gsum/H_0 at small rates; in the fit the densities and the other parameters move as well.) The error of the Hubble constant grows from {:.3} to {:.3} km/s/Mpc when the rate is free. Deflation cannot raise the Hubble constant inferred from the microwave background", fr(0, 0)[5], fr(1, 0)[5], fr(0, 0)[5] * fr(0, 0)[3], fr(1, 0)[5] * fr(1, 0)[3], {
            let h1 = h_at_fixed_theta(&fid, &deflation(1e-3, Closure::Momentum), data.thermo.theta_star_100);
            KMS * (h1 - h0) / 1e-3
        }, KMS * fr(0, 0)[6 + 2], KMS * fr(0, 0)[6 + NPAR + 2]),
        fr(0, 0)[5] < 0.0 && fr(1, 0)[5] < 0.0 && fr(0, 0)[6 + NPAR + 2] > fr(0, 0)[6 + 2]);

    // ---- (2) profile: fits at fixed rate --------------------------------------------------------------------
    let grid_r = [r_llr, 0.005, 0.01, 0.015, 0.02, 0.03, r_bbn];
    let r_refresh = 0.02;
    let mut prof: Vec<Vec<f64>> = Vec::new();
    let mut chord: Vec<Vec<f64>> = Vec::new();
    let mut resid: Vec<Vec<f64>> = (2..=2500).map(|l| vec![l as f64]).collect();
    let mut resid_cols: Vec<String> = vec!["ell".to_string()];
    let mut refreshed: Option<Jacobian> = None;
    let mut closure_dev = [0.0f64; 2];
    for (ic, mock) in mocks.iter().enumerate() {
        let slope = &slopes[ic];
        let mut v_prev = v_fid;
        let mut r_prev = 0.0;
        let mut jac = jac0.clone();
        let mut is_refreshed = 0.0;
        for &r in &grid_r {
            if r > r_refresh + 1e-12 {
                if let Some(j) = &refreshed {
                    jac = j.clone();
                    is_refreshed = 1.0;
                }
            }
            let d = deflation(r, Closure::Momentum);
            let mut start = v_prev;
            for i in 0..NPAR {
                start[i] += slope[i] * (r - r_prev) * h0;
            }
            let mut f = fit(mock, &jac, &ws, &fid, &d, &start, 8);
            if (r - r_refresh).abs() < 1e-12 {
                // the error of using the derivatives of the fiducial point: recompute them at the end point, continue the fit
                if refreshed.is_none() {
                    refreshed = Some(jacobian_defl(&f.params, &d, &ws));
                }
                let again = fit(mock, refreshed.as_ref().unwrap(), &ws, &fid, &d, &to_vec(&f.params), 8);
                chord.push(vec![ic as f64, r, f.chi2, again.chi2, KMS * f.params.h, KMS * again.params.h, f.params.omega_c, again.params.omega_c, f.params.n_s, again.params.n_s]);
                println!("    derivative refresh at gsum = {} H_0: chi2 {:.3} -> {:.3}, H0 {:.3} -> {:.3}", r, f.chi2, again.chi2, KMS * f.params.h, KMS * again.params.h);
                if again.chi2 < f.chi2 {
                    f = again;
                    is_refreshed = 1.0;
                }
                if ic == 0 {
                    for l in 2..=2500usize {
                        let s = &f.spectra;
                        resid[l - 2].push(s.tt[l] / data.tt[l] - 1.0);
                        resid[l - 2].push((s.te[l] - data.te[l]) / (data.tt[l] * data.ee[l]).sqrt());
                        resid[l - 2].push(s.ee[l] / data.ee[l] - 1.0);
                    }
                    for q in ["TT", "TE", "EE"] {
                        resid_cols.push(format!("fit_{}", q));
                    }
                    // and the model with the same rate at the fixed angle, the other parameters not refitted
                    let hfix = h_at_fixed_theta(&fid, &d, data.thermo.theta_star_100);
                    let s = compute(&Params { h: hfix, ..fid }, &d, &ws, 1.0);
                    for l in 2..=2500usize {
                        resid[l - 2].push(s.tt[l] / data.tt[l] - 1.0);
                        resid[l - 2].push((s.te[l] - data.te[l]) / (data.tt[l] * data.ee[l]).sqrt());
                        resid[l - 2].push(s.ee[l] / data.ee[l] - 1.0);
                    }
                    for q in ["TT", "TE", "EE"] {
                        resid_cols.push(format!("angle_{}", q));
                    }
                }
            }
            println!("[{:>5.0} s] {} gsum = {:.5} H_0: chi2 = {:>8.3}  H0 = {:.3}  omega_c = {:.5}  n_s = {:.4}  sigma8 = {:.4}  ({} computations)", elapsed(&t0), mock.cfg.name, r, f.chi2, KMS * f.params.h, f.params.omega_c, f.params.n_s, f.spectra.sigma8, f.evaluations);
            prof.push(fit_row(&[ic as f64, 0.0, r], &f, &d, is_refreshed));
            // the closure "Pressure" at the same parameters, and refitted, at two rates
            if (r - 0.01).abs() < 1e-12 || (r - r_bbn).abs() < 1e-12 {
                let dp = deflation(r, Closure::Pressure);
                let fp = fit(mock, &jac, &ws, &fid, &dp, &to_vec(&f.params), 8);
                closure_dev[0] = closure_dev[0].max((fp.chi2 - f.chi2).abs() / f.chi2.max(1.0));
                closure_dev[1] = closure_dev[1].max(KMS * (fp.params.h - f.params.h).abs());
                prof.push(fit_row(&[ic as f64, 1.0, r], &fp, &dp, is_refreshed));
                println!("          closure \"Pressure\": chi2 = {:>8.3}  H0 = {:.3}", fp.chi2, KMS * fp.params.h);
            }
            v_prev = to_vec(&f.params);
            r_prev = r;
        }
    }
    let mut pcols = vec!["config", "closure", "gsum_over_H0"];
    pcols.extend_from_slice(&FIT_COLS);
    write_csv("deflation_profile.csv",
        &[("content", "best fits of the six LambdaCDM parameters to mock microwave-background data (fiducial LambdaCDM spectra without deflation, no lensing) for a fixed total rate of deflation gsum (three equal rates; assumptions A1, A2, A3)".to_string()),
          ("config", "0 = Planck-like; 1 = Planck-like plus deep survey".to_string()),
          ("closure", "0 = \"Momentum\", 1 = \"Pressure\"".to_string()),
          ("columns", "gsum_over_H0: the rate in units of 67.36 km/s/Mpc; chi2: minimum chi^2 (0 without deflation by construction); tau_reio: optical depth of reionization; H0 in km/s/Mpc; rs_drag in Mpc; S8 = sigma8 sqrt(Omega_m/0.3); dN_bbn: extra radiation during nucleosynthesis (none here); GN_star: Newton's constant at last scattering over today's; jacobian_refreshed = 1 if the derivatives of the spectra were recomputed at the best fit for gsum = 0.02 H_0".to_string()),
          ("command", "deflation_cjt_ks forecast".to_string())],
        &pcols, &prof)?;
    write_csv("deflation_chord_check.csv",
        &[("content", "effect of recomputing the derivatives of the spectra at the end point of a fit (gsum = 0.02 H_0, closure \"Momentum\") and continuing the fit: chi2 and parameters before (a) and after (b)".to_string()),
          ("command", "deflation_cjt_ks forecast".to_string())],
        &["config", "gsum_over_H0", "chi2_a", "chi2_b", "H0_a", "H0_b", "omega_c_a", "omega_c_b", "n_s_a", "n_s_b"], &chord)?;
    let rc: Vec<&str> = resid_cols.iter().map(|s| s.as_str()).collect();
    write_csv("deflation_residuals.csv",
        &[("content", "spectra on the deflating spacetime at gsum = 0.02 H_0 relative to the fiducial LambdaCDM spectra (Planck-like mock, closure \"Momentum\"): fit_ = the best fit of the six parameters; angle_ = the model with the fiducial densities, amplitude, tilt and optical depth and the Hubble constant that keeps the angle theta_*".to_string()),
          ("columns", "_TT = TT_model/TT_fid - 1; _EE likewise; _TE = (TE_model - TE_fid)/sqrt(TT_fid EE_fid)".to_string()),
          ("command", "deflation_cjt_ks forecast".to_string())],
        &rc, &resid)?;
    // the profile against the Fisher matrix
    let pr = |ic: usize, r: f64| prof.iter().find(|x| x[0] == ic as f64 && x[1] == 0.0 && (x[2] - r).abs() < 1e-12).unwrap().clone();
    let (c_chi2, c_h0) = (3, 10);
    let mut quad: Vec<f64> = Vec::new();
    let mut lin: Vec<f64> = Vec::new();
    for ic in 0..2 {
        for &r in &[r_llr, 0.005, 0.01] {
            quad.push(pr(ic, r)[c_chi2] / (r / fr(ic, 0)[2]).powi(2));
            lin.push((pr(ic, r)[c_h0] - KMS * h0) / (fr(ic, 0)[5] * r));
        }
    }
    let (q_lo, q_hi) = (quad.iter().cloned().fold(f64::INFINITY, f64::min), quad.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let (l_lo, l_hi) = (lin.iter().cloned().fold(f64::INFINITY, f64::min), lin.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    // the rate at which the profile reaches chi^2 = 3.84 (interpolation of sqrt(chi^2), which is nearly linear in the rate)
    let crossing = |ic: usize| -> f64 {
        let pts: Vec<(f64, f64)> = std::iter::once((0.0, 0.0)).chain(grid_r.iter().map(|r| (*r, pr(ic, *r)[c_chi2].sqrt()))).collect();
        for w in pts.windows(2) {
            if w[0].1 <= 3.84f64.sqrt() && w[1].1 > 3.84f64.sqrt() {
                return w[0].0 + (3.84f64.sqrt() - w[0].1) / (w[1].1 - w[0].1) * (w[1].0 - w[0].0);
            }
        }
        f64::NAN
    };
    let (x0, x1) = (crossing(0), crossing(1));
    let falls = (0..2).all(|ic| grid_r.windows(2).all(|w| pr(ic, w[1])[c_h0] < pr(ic, w[0])[c_h0] && pr(ic, w[1])[c_chi2] > pr(ic, w[0])[c_chi2]));
    rep.add("def-F4", "numerical",
        &format!("fits at fixed rate (profile; data/deflation_profile.csv). For gsum up to 0.01 H_0 the minimum chi^2 is {:.2} to {:.2} times (gsum/sigma)^2 with the Fisher error sigma, and the shift of the best-fit Hubble constant {:.2} to {:.2} times the Fisher slope: the Fisher matrix describes the fits. The profile reaches chi^2 = 3.84 at gsum = {:.4} H_0 (Planck-like; Fisher bound {:.4}) and {:.4} H_0 (with the deep survey; {:.4}). At the rate of lunar laser ranging the best fit has chi^2 = {:.3} and H_0 = {:.3} km/s/Mpc; at the rate of nucleosynthesis chi^2 = {:.1} ({:.1} standard deviations) and H_0 = {:.2} km/s/Mpc (Planck-like), chi^2 = {:.1} ({:.1} standard deviations) with the deep survey: the largest rate that nucleosynthesis allows would be excluded by such data. At every rate the best-fit Hubble constant is lower than without deflation", q_lo, q_hi, l_lo, l_hi, x0, fr(0, 0)[3], x1, fr(1, 0)[3], pr(0, r_llr)[c_chi2], pr(0, r_llr)[c_h0], pr(0, r_bbn)[c_chi2], pr(0, r_bbn)[c_chi2].sqrt(), pr(0, r_bbn)[c_h0], pr(1, r_bbn)[c_chi2], pr(1, r_bbn)[c_chi2].sqrt()),
        q_lo > 0.8 && q_hi < 1.25 && l_lo > 0.8 && l_hi < 1.25 && (x0 / fr(0, 0)[3] - 1.0).abs() < 0.15 && (x1 / fr(1, 0)[3] - 1.0).abs() < 0.15 && falls && pr(0, r_bbn)[c_chi2] > 9.0 && pr(1, r_bbn)[c_chi2] > 25.0);
    let chord_dchi = chord.iter().map(|c| (c[2] - c[3]).abs()).fold(0.0, f64::max);
    let chord_dh = chord.iter().map(|c| (c[4] - c[5]).abs()).fold(0.0, f64::max);
    rep.add("def-F5", "numerical",
        &format!("accuracy of the fits. (i) Recomputing the derivatives of the spectra at the best fit for gsum = 0.02 H_0 and continuing the fit changes chi^2 by at most {:.3} and the Hubble constant by at most {:.3} km/s/Mpc (data/deflation_chord_check.csv). (ii) The closure: refitting with the closure \"Pressure\" at gsum = 0.01 H_0 and at the rate of nucleosynthesis changes chi^2 by at most {:.1} per cent of itself (or of 1, where it is smaller than 1) and the best-fit Hubble constant by at most {:.4} km/s/Mpc: the forecast, which uses the multipoles l >= 30, does not depend on the closure", chord_dchi, chord_dh, 100.0 * closure_dev[0], closure_dev[1]),
        chord_dchi < 0.5 && chord_dh < 0.1 && closure_dev[0] < 0.03 && closure_dev[1] < 0.01);
    finish(&mut rep, &t0, "forecast", "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the forecast for the rate of deflation from mock data of the microwave background")
}

// =================================================================================================
// families: the dark-radiation families of the main project on the deflating background
// =================================================================================================

/// One-parameter families of dark-radiation models; N is the density of the extra radiation
/// today in units of the density of one massless neutrino species.  (The families of the
/// command `forecast` of the main project, where the function is private.)
fn dr_family(id: usize, n: f64) -> Params {
    let fid = Params::fiducial();
    match id {
        0 => Params { n_ur: fid.n_ur + n, ..fid },
        1 => Params { dark: DarkFluid { n_ir: n, step: None }, ..fid },
        2 => Params { dark: DarkFluid { n_ir: n, step: Some(Step::project(1.0e4)) }, ..fid },
        3 => Params { dark: DarkFluid { n_ir: n, step: Some(Step::project(2.0e4)) }, ..fid },
        4 => Params { dark: DarkFluid { n_ir: n, step: Some(Step::project(5.0e4)) }, ..fid },
        5 => Params { dark: DarkFluid { n_ir: n, step: Some(Step::wess_zumino(2.0e4)) }, ..fid },
        6 => Params { dark: DarkFluid { n_ir: n, step: Some(Step::project(1.0e6)) }, ..fid },
        _ => panic!("unknown family"),
    }
}

/// Derivative of the spectra of the family `fam` with respect to the density N of its extra
/// radiation at N = 0, for the other parameters `at` on the background `d`; `s0` are the
/// spectra at N = 0.  The formulas of the command `forecast` of the main project, step 0.05:
/// a central difference for free-streaming radiation; for the fluids, whose density cannot be
/// negative, the one-sided formula of second order.
fn n_derivative(fam: usize, at: &Params, d: &Deflation, ws: &Workspace, s0: &Spectra) -> [Vec<f64>; 3] {
    let e = 0.05;
    let with = |n: f64| -> Params {
        let q = dr_family(fam, n);
        Params { n_ur: q.n_ur, dark: q.dark, ..*at }
    };
    if fam == 0 {
        let (a, b) = (compute(&with(e), d, ws, 1.0), compute(&with(-e), d, ws, 1.0));
        let f = |x: &[f64], y: &[f64]| -> Vec<f64> { x.iter().zip(y).map(|(p, q)| (p - q) / (2.0 * e)).collect() };
        [f(&a.tt, &b.tt), f(&a.te, &b.te), f(&a.ee, &b.ee)]
    } else {
        let (s1, s2) = (compute(&with(e), d, ws, 1.0), compute(&with(2.0 * e), d, ws, 1.0));
        let f = |x: &[f64], y: &[f64], z: &[f64]| -> Vec<f64> { (0..x.len()).map(|l| (-3.0 * x[l] + 4.0 * y[l] - z[l]) / (2.0 * e)).collect() };
        [f(&s0.tt, &s1.tt, &s2.tt), f(&s0.te, &s1.te, &s2.te), f(&s0.ee, &s1.ee, &s2.ee)]
    }
}

const DR_FAMILIES: [&str; 7] = [
    "0 = free-streaming extra radiation (N_ur = 3.044 + N)",
    "1 = fluid dark radiation without step",
    "2 = stepped fluid of the model (1 massless boson + 16 fermionic d.o.f., r_step = 15^(1/3)), z_t = 1e4",
    "3 = the same, z_t = 2e4",
    "4 = the same, z_t = 5e4",
    "5 = Wess-Zumino stepped fluid (r_step = (15/7)^(1/3)), z_t = 2e4",
    "6 = stepped fluid of the model, z_t = 1e6 (step after nucleosynthesis, before the scales of the microwave background enter the horizon)",
];

/// `families`: the command `forecast` of the main project (fits of the dark-radiation
/// families to mock microwave-background data) repeated on the deflating spacetime, with
/// the rate that lunar laser ranging allows.  Writes data/dr_deflation_fisher.csv,
/// dr_deflation_profiles.csv, dr_deflation_chord_check.csv, dr_deflation_residuals.csv,
/// dr_deflation_zt_scan.csv, dr_deflation_joint.csv, families_checks.{txt,csv}.  With the
/// argument `quick`: two families, two densities, one data set (a test of the command; its
/// files are written to a temporary folder, not to the folder data).  Run time: about as
/// long as the command of the main project.
pub fn run_families(main_data_dir: &str) -> std::io::Result<bool> {
    let t0 = Instant::now();
    let mut rep = Report::new();
    let quick = std::env::args().nth(2).map(|s| s == "quick").unwrap_or(false);
    if quick && std::env::var("SPT_DATA_DIR").is_err() {
        // a test run must not overwrite the data files of the full run
        let dir = std::env::temp_dir().join("deflation_families_quick");
        println!("quick test: the files are written to {} and not to the folder data", dir.display());
        std::env::set_var("SPT_DATA_DIR", &dir);
    }
    let ws = Workspace::new(2500);
    let fid = Params::fiducial();
    let none = Deflation::none();
    let (r_llr, _) = allowed_rates();
    let dfl = deflation(r_llr, Closure::Momentum);
    // the data: the fiducial LambdaCDM model without deflation, as in the main project
    let data = compute(&fid, &none, &ws, 1.0);
    let jac_fid = jacobian_defl(&fid, &none, &ws);
    println!("[{:>5.0} s] fiducial spectra and their derivatives", elapsed(&t0));
    let n_fam = if quick { 2 } else { DR_FAMILIES.len() };
    let n_grid: Vec<f64> = if quick { vec![0.2, 0.5] } else { vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.8, 1.0] };
    let n_refresh = 0.5;
    let fam_meta = DR_FAMILIES.join("; ");
    let rate_meta = format!("three equal rates of deflation with the total rate gsum = {:.6} H_0 (H_0 = 67.36 km/s/Mpc), the largest that lunar laser ranging allows; assumptions A1, A2, A3 (closure \"Momentum\"; the multipoles used do not depend on the closure)", r_llr);
    let configs = if quick { vec![Config::planck_like()] } else { vec![Config::planck_like(), Config::planck_plus_deep()] };
    let main_prof = read_numbers(&format!("{}/dr_profiles.csv", main_data_dir));
    let main_fish = read_numbers(&format!("{}/dr_fisher.csv", main_data_dir));
    let mut fisher_rows: Vec<Vec<f64>> = Vec::new();
    let mut prof_rows: Vec<Vec<f64>> = Vec::new();
    let mut chord_rows: Vec<Vec<f64>> = Vec::new();
    let mut resid: Vec<Vec<f64>> = (2..=2500).map(|l| vec![l as f64]).collect();
    let mut resid_cols: Vec<String> = vec!["ell".to_string()];
    let mut refreshed_jac: Vec<Option<Jacobian>> = (0..n_fam).map(|_| None).collect();
    let mut zero_ok = true;
    // the derivatives of the spectra on the deflating background: at the best fit of the deflating model without
    // extra radiation (first data set), with respect to the six parameters and to N for every family
    let mut base_fit: Vec<Fit> = Vec::new();
    let mut jac_d: Option<Jacobian> = None;
    let mut d_n: Vec<[Vec<f64>; 3]> = Vec::new();
    // differences to the main project, per configuration: shift of H0, change of chi^2
    let mut shifts: Vec<(f64, f64, f64, f64)> = Vec::new();
    // is the minimum chi^2 lower than in the main project at every N of every family?
    let mut lower_everywhere = true;
    for (ic, cfg) in configs.into_iter().enumerate() {
        let cfg_name = cfg.name;
        let mock = Mock { cfg, data: data.clone(), tau0: fid.tau_reio };
        zero_ok &= chi2(&mock, &data.tt, &data.te, &data.ee, fid.tau_reio).abs() < 1e-9;
        // the deflating model without extra radiation: its best fit to the data
        let f0 = fit(&mock, &jac_fid, &ws, &fid, &dfl, &to_vec(&fid), 8);
        println!("\n=== mock data set '{}' ===", cfg_name);
        println!("[{:>5.0} s] deflating model without extra radiation: chi2 = {:.4}, H0 = {:.4}", elapsed(&t0), f0.chi2, KMS * f0.params.h);
        if jac_d.is_none() {
            let j = jacobian_defl(&f0.params, &dfl, &ws);
            // derivative with respect to N at N = 0 for every family, at the same parameters
            let s0 = compute(&f0.params, &dfl, &ws, 1.0);
            for fam in 0..n_fam {
                d_n.push(n_derivative(fam, &f0.params, &dfl, &ws, &s0));
            }
            jac_d = Some(j);
            println!("[{:>5.0} s] derivatives on the deflating background (six parameters and N for {} families)", elapsed(&t0), n_fam);
        }
        let jac0 = jac_d.clone().unwrap();
        let v0 = to_vec(&f0.params);
        // Fisher matrix of the six parameters on the deflating background
        let f6 = fisher(&mock, &jac0, &[]);
        let c6 = invert_spd(&f6).expect("Fisher matrix not positive definite");
        let (mut dh_lo, mut dh_hi, mut dchi_max, mut dsig_max) = (f64::INFINITY, f64::NEG_INFINITY, 0.0f64, 0.0f64);
        for fam in 0..n_fam {
            let f7 = fisher(&mock, &jac0, &[d_n[fam].clone()]);
            let c7 = invert_spd(&f7).expect("Fisher matrix not positive definite");
            let sigma_n = c7[6][6].sqrt();
            let rhs: Vec<f64> = (0..NPAR).map(|i| -f7[i][6]).collect();
            let slope = solve_spd(&f6, &rhs).unwrap();
            let mut row = vec![ic as f64, fam as f64, sigma_n, KMS * slope[2]];
            for i in 0..NPAR {
                row.push(c6[i][i].sqrt());
            }
            for i in 0..NPAR {
                row.push(c7[i][i].sqrt());
            }
            for i in 0..NPAR {
                row.push(slope[i]);
            }
            fisher_rows.push(row);
            if let Some(rows) = &main_fish {
                if let Some(m) = rows.iter().find(|r| r[0] == ic as f64 && r[1] == fam as f64) {
                    dsig_max = dsig_max.max((sigma_n / m[2] - 1.0).abs());
                }
            }
            println!("family {}: Fisher sigma(N) = {:.4}; dH0/dN along the degeneracy = {:.3} km/s/Mpc; sigma(H0) with N free = {:.3}", fam, sigma_n, KMS * slope[2], KMS * c7[2][2].sqrt());

            // profile: fit at fixed N, continuing from the previous point
            let mut v_prev = v0;
            let mut n_prev = 0.0;
            let mut jac = jac0.clone();
            let mut is_refreshed = 0.0;
            for &n in &n_grid {
                if n > n_refresh + 1e-9 {
                    if let Some(j) = &refreshed_jac[fam] {
                        jac = j.clone();
                        is_refreshed = 1.0;
                    }
                }
                let base = dr_family(fam, n);
                let mut start = v_prev;
                for i in 0..NPAR {
                    start[i] += slope[i] * (n - n_prev);
                }
                let mut fit_n = fit(&mock, &jac, &ws, &base, &dfl, &start, 8);
                if (n - n_refresh).abs() < 1e-9 && !quick {
                    if refreshed_jac[fam].is_none() {
                        refreshed_jac[fam] = Some(jacobian_defl(&fit_n.params, &dfl, &ws));
                    }
                    let j = refreshed_jac[fam].as_ref().unwrap();
                    let again = fit(&mock, j, &ws, &base, &dfl, &to_vec(&fit_n.params), 8);
                    chord_rows.push(vec![ic as f64, fam as f64, n, fit_n.chi2, again.chi2, KMS * fit_n.params.h, KMS * again.params.h, fit_n.params.omega_c, again.params.omega_c, fit_n.params.n_s, again.params.n_s]);
                    println!("    derivative refresh at N = {}: chi2 {:.3} -> {:.3}, H0 {:.3} -> {:.3}", n, fit_n.chi2, again.chi2, KMS * fit_n.params.h, KMS * again.params.h);
                    if again.chi2 < fit_n.chi2 {
                        fit_n = again;
                        is_refreshed = 1.0;
                    }
                }
                println!("[{:>5.0} s] {} family {} N = {:.2}: chi2 = {:>8.3}  H0 = {:.3}  omega_c = {:.5}  n_s = {:.4}  sigma8 = {:.4}  ({} computations)", elapsed(&t0), cfg_name, fam, n, fit_n.chi2, KMS * fit_n.params.h, fit_n.params.omega_c, fit_n.params.n_s, fit_n.spectra.sigma8, fit_n.evaluations);
                prof_rows.push(fit_row(&[ic as f64, fam as f64, n], &fit_n, &dfl, is_refreshed));
                // against the main project: the same family and N without deflation
                if let Some(rows) = &main_prof {
                    if let Some(m) = rows.iter().find(|r| r[0] == ic as f64 && r[1] == fam as f64 && (r[2] - n).abs() < 1e-9) {
                        let dh = KMS * fit_n.params.h - m[10];
                        dh_lo = dh_lo.min(dh);
                        dh_hi = dh_hi.max(dh);
                        dchi_max = dchi_max.max((fit_n.chi2 - m[3]).abs());
                        lower_everywhere &= fit_n.chi2 < m[3];
                    }
                }
                if ic == 0 && (n - n_refresh).abs() < 1e-9 {
                    for l in 2..=2500usize {
                        let s = &fit_n.spectra;
                        resid[l - 2].push(s.tt[l] / data.tt[l] - 1.0);
                        resid[l - 2].push((s.te[l] - data.te[l]) / (data.tt[l] * data.ee[l]).sqrt());
                        resid[l - 2].push(s.ee[l] / data.ee[l] - 1.0);
                    }
                    for q in ["TT", "TE", "EE"] {
                        resid_cols.push(format!("fam{}_{}", fam, q));
                    }
                }
                v_prev = to_vec(&fit_n.params);
                n_prev = n;
            }
            // write what exists so far (a long run should not lose its results)
            let mut cols = vec!["config", "family", "N"];
            cols.extend_from_slice(&FIT_COLS);
            write_csv("dr_deflation_profiles.csv",
                &[("content", "the dark-radiation families of the main project on the deflating spacetime: best fits of the six LambdaCDM parameters to mock microwave-background data (fiducial LambdaCDM spectra without deflation, no lensing) for fixed density N of extra radiation".to_string()),
                  ("deflation", rate_meta.clone()),
                  ("config", "0 = Planck-like (f_sky 0.6, l = 30..2500 in T and 30..2000 in E); 1 = Planck-like plus a deep survey of 25 per cent of the sky (1.4 arcmin, 10 microkelvin arcmin)".to_string()),
                  ("family", fam_meta.clone()),
                  ("N", "density of the extra radiation today in units of one massless neutrino species (for stepped fluids: the value after the step); N = 0 is the deflating model without extra radiation".to_string()),
                  ("columns", "chi2 = minimum chi^2; tau_reio: optical depth of reionization; H0 in km/s/Mpc; rs_drag in Mpc; S8 = sigma8 sqrt(Omega_m/0.3); dN_bbn = extra radiation during nucleosynthesis used for the helium abundance (the response of the helium abundance to Newton's constant at nucleosynthesis is included in Y_He); GN_star: Newton's constant at last scattering over today's; jacobian_refreshed = 1 if the derivatives of the spectra were recomputed at the N = 0.5 best fit of the family".to_string()),
                  ("command", "deflation_cjt_ks families".to_string())],
                &cols, &prof_rows)?;
        }
        shifts.push((dh_lo, dh_hi, dchi_max, dsig_max));
        // the row N = 0 of this data set
        prof_rows.push(fit_row(&[ic as f64, -1.0, 0.0], &f0, &dfl, 0.0));
        base_fit.push(f0);
    }
    let mut cols: Vec<String> = vec!["config".into(), "family".into(), "sigma_N".into(), "dH0_dN".into()];
    for pre in ["sigma6_", "sigma7_", "slope_"] {
        for nme in PAR_NAMES {
            cols.push(format!("{}{}", pre, nme));
        }
    }
    let cols_ref: Vec<&str> = cols.iter().map(|s| s.as_str()).collect();
    write_csv("dr_deflation_fisher.csv",
        &[("content", "the dark-radiation families of the main project on the deflating spacetime: Fisher-matrix forecasts for the mock data sets: errors of the six LambdaCDM parameters without (sigma6_) and with (sigma7_) the density N of extra radiation as a free parameter; error of N; shift of the best-fit parameters per unit N at fixed N (slope_)".to_string()),
          ("deflation", rate_meta.clone()),
          ("config", "0 = Planck-like; 1 = Planck-like plus deep survey".to_string()),
          ("family", fam_meta.clone()),
          ("units", "dH0_dN in km/s/Mpc per unit N; h dimensionless".to_string()),
          ("command", "deflation_cjt_ks families".to_string())],
        &cols_ref, &fisher_rows)?;
    // the complete table of profiles, with the rows N = 0 (family -1)
    {
        let mut cols = vec!["config", "family", "N"];
        cols.extend_from_slice(&FIT_COLS);
        write_csv("dr_deflation_profiles.csv",
            &[("content", "the dark-radiation families of the main project on the deflating spacetime: best fits of the six LambdaCDM parameters to mock microwave-background data (fiducial LambdaCDM spectra without deflation, no lensing) for fixed density N of extra radiation".to_string()),
              ("deflation", rate_meta.clone()),
              ("config", "0 = Planck-like (f_sky 0.6, l = 30..2500 in T and 30..2000 in E); 1 = Planck-like plus a deep survey of 25 per cent of the sky (1.4 arcmin, 10 microkelvin arcmin)".to_string()),
              ("family", format!("{}; -1 = the deflating model without extra radiation (N = 0)", fam_meta)),
              ("N", "density of the extra radiation today in units of one massless neutrino species (for stepped fluids: the value after the step)".to_string()),
              ("columns", "chi2 = minimum chi^2; tau_reio: optical depth of reionization; H0 in km/s/Mpc; rs_drag in Mpc; S8 = sigma8 sqrt(Omega_m/0.3); dN_bbn = extra radiation during nucleosynthesis used for the helium abundance (the response of the helium abundance to Newton's constant at nucleosynthesis is included in Y_He); GN_star: Newton's constant at last scattering over today's; jacobian_refreshed = 1 if the derivatives of the spectra were recomputed at the N = 0.5 best fit of the family".to_string()),
              ("command", "deflation_cjt_ks families".to_string())],
            &cols, &prof_rows)?;
    }
    write_csv("dr_deflation_chord_check.csv",
        &[("content", "the dark-radiation families on the deflating spacetime: effect of recomputing the derivatives of the spectra at the end point of a fit (N = 0.5) and continuing the fit: chi2 and parameters before (a) and after (b)".to_string()),
          ("deflation", rate_meta.clone()),
          ("family", fam_meta.clone()),
          ("command", "deflation_cjt_ks families".to_string())],
        &["config", "family", "N", "chi2_a", "chi2_b", "H0_a", "H0_b", "omega_c_a", "omega_c_b", "n_s_a", "n_s_b"], &chord_rows)?;
    let rc: Vec<&str> = resid_cols.iter().map(|s| s.as_str()).collect();
    write_csv("dr_deflation_residuals.csv",
        &[("content", "the dark-radiation families on the deflating spacetime: best-fit spectra at N = 0.5 (Planck-like mock) relative to the fiducial LambdaCDM spectra without deflation".to_string()),
          ("deflation", rate_meta.clone()),
          ("columns", "famK_TT = TT_model/TT_fid - 1; famK_EE likewise; famK_TE = (TE_model - TE_fid)/sqrt(TT_fid EE_fid)".to_string()),
          ("family", fam_meta.clone()),
          ("command", "deflation_cjt_ks families".to_string())],
        &rc, &resid)?;

    // dependence on the transition redshift at fixed N (Planck-like mock)
    let mut zt_dh = (f64::INFINITY, f64::NEG_INFINITY);
    let mut zt_best = (f64::NAN, f64::INFINITY);
    if !quick {
        let mock = Mock { cfg: Config::planck_like(), data: data.clone(), tau0: fid.tau_reio };
        let n = 0.4;
        let jac0 = jac_d.clone().unwrap();
        let v0 = to_vec(&base_fit[0].params);
        let main_zt = read_numbers(&format!("{}/dr_zt_scan.csv", main_data_dir));
        let mut rows = Vec::new();
        let mut v_prev = v0;
        for i in 0..=12 {
            let lz = 3.0 + 0.25 * i as f64;
            let base = Params { dark: DarkFluid { n_ir: n, step: Some(Step::project(10f64.powf(lz))) }, ..fid };
            let start = if i == 0 { let mut s = v0; s[2] += 0.02; s[1] += 0.004; s } else { v_prev };
            let f = fit(&mock, &jac0, &ws, &base, &dfl, &start, 8);
            println!("[{:>5.0} s] z_t = 10^{:.2}: chi2 = {:.3}, H0 = {:.3}", elapsed(&t0), lz, f.chi2, KMS * f.params.h);
            if let Some(m) = main_zt.as_ref().and_then(|rows| rows.iter().find(|r| (r[0] - lz).abs() < 1e-9)) {
                let dh = KMS * f.params.h - m[9];
                zt_dh = (zt_dh.0.min(dh), zt_dh.1.max(dh));
            }
            if f.chi2 < zt_best.1 {
                zt_best = (lz, f.chi2);
            }
            rows.push(fit_row(&[lz, n], &f, &dfl, 0.0));
            v_prev = to_vec(&f.params);
        }
        let mut cols = vec!["log10_z_t", "N"];
        cols.extend_from_slice(&FIT_COLS);
        write_csv("dr_deflation_zt_scan.csv",
            &[("content", "the stepped fluid of the model (r_step = 15^(1/3)) with N = 0.4 after the step, on the deflating spacetime: best fit to the Planck-like mock as a function of the transition redshift z_t".to_string()),
              ("deflation", rate_meta.clone()),
              ("command", "deflation_cjt_ks families".to_string())],
            &cols, &rows)?;
    }
    // ---- the rate of deflation and the density of extra radiation as free parameters together ----------------
    // Fisher matrices at the fiducial model without deflation with eight parameters: the six of the standard
    // model, the density N of one family, and the rate (derivative: the one-sided formula of the command
    // `forecast`).  From them: the errors, the correlation of N with the rate, the best-fitting N at the rate of
    // lunar laser ranging, and the largest Hubble constant on the contour chi^2 = 3.84 for N >= 0, gsum >= 0.
    let h0 = fid.h;
    let level = 3.84f64;
    let d_rate = rate_derivative(&fid, &none, 0.0, 0.002, &ws);
    let d_n0: Vec<[Vec<f64>; 3]> = (0..n_fam).map(|fam| n_derivative(fam, &fid, &none, &ws, &data)).collect();
    println!("[{:>5.0} s] derivatives at the fiducial model with respect to the rate and to N", elapsed(&t0));
    let mut joint_rows: Vec<Vec<f64>> = Vec::new();
    let (mut sig7_dev, mut schur_dev) = (0.0f64, 0.0f64);
    let mut cross = (f64::INFINITY, f64::NEG_INFINITY);
    for ic in 0..shifts.len() {
        let cfg = if ic == 0 { Config::planck_like() } else { Config::planck_plus_deep() };
        let mock = Mock { cfg, data: data.clone(), tau0: fid.tau_reio };
        let f6 = fisher(&mock, &jac_fid, &[]);
        let c7g = invert_spd(&fisher(&mock, &jac_fid, &[d_rate.clone()])).expect("Fisher matrix not positive definite");
        let sigma_g7 = c7g[6][6].sqrt() / h0;
        for fam in 0..n_fam {
            let c7n = invert_spd(&fisher(&mock, &jac_fid, &[d_n0[fam].clone()])).expect("Fisher matrix not positive definite");
            let sigma_n7 = c7n[6][6].sqrt();
            if let Some(m) = main_fish.as_ref().and_then(|rows| rows.iter().find(|r| r[0] == ic as f64 && r[1] == fam as f64)) {
                sig7_dev = sig7_dev.max((sigma_n7 / m[2] - 1.0).abs());
            }
            let f8 = fisher(&mock, &jac_fid, &[d_n0[fam].clone(), d_rate.clone()]);
            let c8 = invert_spd(&f8).expect("Fisher matrix not positive definite");
            // covariance of (N, gsum/H_0) with the six parameters free, and its inverse (their marginal Fisher matrix)
            let (cnn, cng, cgg) = (c8[6][6], c8[6][7] / h0, c8[7][7] / (h0 * h0));
            let rho = cng / (cnn * cgg).sqrt();
            let det = cnn * cgg - cng * cng;
            let (fnn, fng, fgg) = (cgg / det, -cng / det, cnn / det);
            // 1/fnn is the variance of N at zero rate, 1/fgg that of the rate at N = 0
            schur_dev = schur_dev.max((1.0 / fnn.sqrt() / sigma_n7 - 1.0).abs()).max((1.0 / fgg.sqrt() / sigma_g7 - 1.0).abs());
            // shift of the best-fit Hubble constant per unit N and per unit gsum/H_0
            let slope_n = solve_spd(&f6, &(0..NPAR).map(|i| -f8[i][6]).collect::<Vec<f64>>()).unwrap();
            let slope_g = solve_spd(&f6, &(0..NPAR).map(|i| -f8[i][7]).collect::<Vec<f64>>()).unwrap();
            let (a, b) = (KMS * slope_n[2], KMS * slope_g[2] * h0);
            // the largest a N + b g with chi^2 = fnn N^2 + 2 fng N g + fgg g^2 <= level, N >= 0, g >= 0:
            // the point of the ellipse along C (a, b) if it lies in the quadrant, else a point on an axis
            let (xn, xg) = (cnn * a + cng * b, cng * a + cgg * b);
            let gain_axis = (a * (level / fnn).sqrt()).max(b * (level / fgg).sqrt()).max(0.0);
            let (n_opt, g_opt, gain) = if xn > 0.0 && xg > 0.0 {
                let norm = (level / (a * xn + b * xg)).sqrt();
                (norm * xn, norm * xg, norm * (a * xn + b * xg))
            } else if a * (level / fnn).sqrt() >= gain_axis {
                ((level / fnn).sqrt(), 0.0, gain_axis)
            } else if b * (level / fgg).sqrt() >= gain_axis {
                (0.0, (level / fgg).sqrt(), gain_axis)
            } else {
                (0.0, 0.0, 0.0)
            };
            // the best-fitting N >= 0 at the rate of lunar laser ranging, and its chi^2
            let n_min = (-fng * r_llr / fnn).max(0.0);
            let chi_min = fnn * n_min * n_min + 2.0 * fng * n_min * r_llr + fgg * r_llr * r_llr;
            // the cross term against the fits: chi^2 on the deflating background minus that of the main project at the same N
            let mut ratios = [f64::NAN, f64::NAN];
            for (j, n) in [0.1, 0.2].iter().enumerate() {
                let own = prof_rows.iter().find(|r| r[0] == ic as f64 && r[1] == fam as f64 && (r[2] - n).abs() < 1e-9).map(|r| r[3]);
                let main = main_prof.as_ref().and_then(|rows| rows.iter().find(|r| r[0] == ic as f64 && r[1] == fam as f64 && (r[2] - n).abs() < 1e-9)).map(|r| r[3]);
                if let (Some(x), Some(y)) = (own, main) {
                    ratios[j] = (x - y) / (2.0 * fng * n * r_llr + fgg * r_llr * r_llr);
                    cross = (cross.0.min(ratios[j]), cross.1.max(ratios[j]));
                }
            }
            println!("joint, data set {} family {}: sigma(N) {:.4} -> {:.4}, sigma(gsum/H_0) {:.5} -> {:.5}, correlation {:+.3}; at the rate of lunar laser ranging the best N = {:.4} (chi2 {:.4}); largest H0 gain with chi2 <= 3.84: {:.3} km/s/Mpc at N = {:.3}, gsum = {:.4} H_0 (at zero rate: {:.3}); cross term fits/Fisher {:.2}, {:.2}", ic, fam, sigma_n7, cnn.sqrt(), sigma_g7, cgg.sqrt(), rho, n_min, chi_min, gain, n_opt, g_opt, a * (level / fnn).sqrt(), ratios[0], ratios[1]);
            joint_rows.push(vec![ic as f64, fam as f64, sigma_n7, cnn.sqrt(), sigma_g7, cgg.sqrt(), rho, a, b, n_min, chi_min, a * (level / fnn).sqrt(), gain, n_opt, g_opt, ratios[0], ratios[1]]);
        }
    }
    write_csv("dr_deflation_joint.csv",
        &[("content", "the density N of extra radiation of each family and the total rate of deflation gsum as free parameters together: Fisher matrices with eight parameters (the six of LambdaCDM, N, gsum) at the fiducial model without deflation, for the mock data sets of the main project".to_string()),
          ("config", "0 = Planck-like; 1 = Planck-like plus deep survey".to_string()),
          ("family", fam_meta.clone()),
          ("columns", "sigma_N, sigma_N_joint: error of N with the rate held at zero and with the rate free; sigma_rate, sigma_rate_joint: error of gsum/H_0 (H_0 = 67.36 km/s/Mpc) with N held at zero and with N free; correlation: correlation coefficient of N and gsum; dH0_dN, dH0_drate: change of the best-fit Hubble constant in km/s/Mpc per unit N and per unit gsum/H_0; N_best_llr, chi2_best_llr: the best-fitting N >= 0 and its chi^2 at the largest rate that lunar laser ranging allows; H0_gain_zero_rate: the largest rise of the Hubble constant with chi^2 <= 3.84 at zero rate; H0_gain, N_at_gain, rate_at_gain: the largest rise with chi^2 <= 3.84 for N >= 0 and gsum >= 0, and where it is reached; cross_N0.1, cross_N0.2: (minimum chi^2 of the fit on the deflating background at the rate of lunar laser ranging minus that of the main project at the same N) divided by the prediction of the Fisher matrix".to_string()),
          ("command", "deflation_cjt_ks families".to_string())],
        &["config", "family", "sigma_N", "sigma_N_joint", "sigma_rate", "sigma_rate_joint", "correlation", "dH0_dN", "dH0_drate", "N_best_llr", "chi2_best_llr", "H0_gain_zero_rate", "H0_gain", "N_at_gain", "rate_at_gain", "cross_N0.1", "cross_N0.2"], &joint_rows)?;

    // checks
    rep.add("def-N1", "numerical",
        &format!("the forecast of the main project for its dark-radiation families, repeated on the deflating spacetime at the rate that lunar laser ranging allows (gsum = {:.5} H_0). The data are the spectra of the fiducial model without deflation (chi^2 = 0 for that model). The deflating model without extra radiation fits them with chi^2 = {:.3} and H_0 = {:.3} km/s/Mpc (Planck-like){}", r_llr, base_fit[0].chi2, KMS * base_fit[0].params.h, if base_fit.len() > 1 { format!(", chi^2 = {:.3} and H_0 = {:.3} km/s/Mpc with the deep survey", base_fit[1].chi2, KMS * base_fit[1].params.h) } else { String::new() }),
        zero_ok && base_fit.iter().all(|f| f.chi2 < 3.84 && f.params.h < fid.h));
    if !quick {
        let dh_base: Vec<f64> = base_fit.iter().map(|f| KMS * (f.params.h - fid.h)).collect();
        rep.add("def-N2", "numerical",
            &format!("the families on the deflating background against the main project (its files data/dr_fisher.csv and data/dr_profiles.csv): the Fisher errors of N agree to {:.1} per cent (Planck-like) and {:.1} per cent (with the deep survey) for the {} families. In the fits at fixed N = 0.1 to 1 the best-fit Hubble constant is lower than in the main project by {:.3} to {:.3} km/s/Mpc (Planck-like) and {:.3} to {:.3} km/s/Mpc (deep survey); the model without extra radiation is lower by {:.3} and {:.3} km/s/Mpc. The minimum chi^2 is LOWER than in the main project, by up to {:.2} and {:.2} at N = 1 (check def-N4 explains why). Deflation at this rate lowers every Hubble constant of the main project by about a quarter of a km/s/Mpc", 100.0 * shifts[0].3, 100.0 * shifts[1].3, n_fam, -shifts[0].1, -shifts[0].0, -shifts[1].1, -shifts[1].0, -dh_base[0], -dh_base[1], shifts[0].2, shifts[1].2),
            main_prof.is_some() && main_fish.is_some() && shifts.iter().all(|s| s.3 < 0.03 && s.1 < 0.0 && s.0 > -0.6) && lower_everywhere);
        let chord_dchi = chord_rows.iter().map(|c| (c[3] - c[4]).abs()).fold(0.0, f64::max);
        let chord_dh = chord_rows.iter().map(|c| (c[5] - c[6]).abs()).fold(0.0, f64::max);
        rep.add("def-N3", "numerical",
            &format!("accuracy of the fits on the deflating background: recomputing the derivatives of the spectra at the N = 0.5 best fit of each family and continuing the fit changes chi^2 by at most {:.3} and the Hubble constant by at most {:.3} km/s/Mpc (data/dr_deflation_chord_check.csv). The scan of the transition redshift of the stepped fluid of the model at N = 0.4 (data/dr_deflation_zt_scan.csv) has its best fit at z_t = 10^{:.2} (chi^2 = {:.3}); its Hubble constants are lower than those of the main project by {:.3} to {:.3} km/s/Mpc", chord_dchi, chord_dh, zt_best.0, zt_best.1, -zt_dh.1, -zt_dh.0),
            chord_dchi < 3.0 && chord_dh < 0.5 && zt_dh.1 < 0.0);
    }
    {
        let col = |ic: usize, c: usize| -> (f64, f64) {
            let v: Vec<f64> = joint_rows.iter().filter(|r| r[0] == ic as f64).map(|r| r[c]).collect();
            (v.iter().cloned().fold(f64::INFINITY, f64::min), v.iter().cloned().fold(f64::NEG_INFINITY, f64::max))
        };
        let last = shifts.len() - 1;
        let with_rate = joint_rows.iter().filter(|r| r[14] > 0.0).count();
        let extra = joint_rows.iter().map(|r| r[12] - r[11]).fold(0.0, f64::max);
        let outcome = if with_rate == 0 {
            format!("For every family and both data sets the largest Hubble constant with chi^2 <= 3.84 (N >= 0, gsum >= 0) is reached at ZERO rate (a rise of {:.2} to {:.2} km/s/Mpc, Planck-like): the deflation lowers the Hubble constant faster than it makes room for extra radiation, and it cannot help the dark radiation to raise the Hubble constant", col(0, 11).0, col(0, 11).1)
        } else {
            format!("In {} of the {} cases (family and data set) the largest Hubble constant with chi^2 <= 3.84 (N >= 0, gsum >= 0) is reached at a rate above zero; the rise exceeds that at zero rate by at most {:.3} km/s/Mpc (rise at zero rate: {:.2} to {:.2} km/s/Mpc, Planck-like)", with_rate, joint_rows.len(), extra, col(0, 11).0, col(0, 11).1)
        };
        rep.add("def-N4", "numerical",
            &format!("the rate of deflation and the density N of extra radiation as free parameters together (Fisher matrices with eight parameters at the fiducial model; data/dr_deflation_joint.csv). (i) With the rate held at zero the errors of N are those of the main project (to {:.1e}), and the marginal Fisher matrix of the pair reproduces the errors of each parameter alone (to {:.1e}). (ii) N and the rate are positively correlated for every family: correlation coefficient {:.2} to {:.2} (Planck-like) and {:.2} to {:.2} ({}). Extra radiation makes the early universe expand faster, the weaker gravity of the deflating spacetime makes it expand more slowly, and the two compensate in part. With both free the error of gsum/H_0 grows from {:.4} to between {:.4} and {:.4} (Planck-like), and the error of N by the same factor. (iii) At the largest rate that lunar laser ranging allows the best-fitting density is N = {:.3} to {:.3} instead of zero (chi^2 = {:.3} to {:.3} instead of {:.3}). The difference between the minimum chi^2 of the fits on the deflating background and those of the main project at N = 0.1 and 0.2 is {:.2} to {:.2} times the cross term of the Fisher matrix. (iv) {}", sig7_dev, schur_dev, col(0, 6).0, col(0, 6).1, col(last, 6).0, col(last, 6).1, if last == 1 { "with the deep survey" } else { "the same data set" }, joint_rows[0][4], col(0, 5).0, col(0, 5).1, col(0, 9).0, col(0, 9).1, col(0, 10).0, col(0, 10).1, (r_llr / joint_rows[0][4]).powi(2), cross.0, cross.1, outcome),
            sig7_dev < 1e-8 && schur_dev < 1e-8 && joint_rows.iter().all(|r| r[6] > 0.0 && r[6] < 1.0) && cross.0 > 0.7 && cross.1 < 1.3);
    }
    finish(&mut rep, &t0, "families", "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of the forecast for the dark-radiation families of the main project on the deflating spacetime")
}

// =================================================================================================
// superfluid
// =================================================================================================

/// Wavenumber (1/Mpc) at which the transfer function of thermal warm dark matter of mass
/// `m_kev` falls to 1/2, from the fitting formula of Viel et al. (2005) (the function of
/// the command `superfluid` of the main project, where it is private).
fn wdm_half_mode(m_kev: f64, omega_dm: f64, h: f64) -> f64 {
    let nu: f64 = 1.12;
    let alpha = 0.049 * m_kev.powf(-1.11) * (omega_dm / 0.25).powf(0.11) * (h / 0.7).powf(1.22); // Mpc/h
    (2f64.powf(nu / 5.0) - 1.0).powf(1.0 / (2.0 * nu)) / alpha * h
}

/// Inverse of `wdm_half_mode`: the thermal-relic mass (keV) with the given half-mode wavenumber (1/Mpc).
fn wdm_mass_of_half_mode(k_half: f64, omega_dm: f64, h: f64) -> f64 {
    let nu: f64 = 1.12;
    let kappa = (2f64.powf(nu / 5.0) - 1.0).powf(1.0 / (2.0 * nu));
    let alpha = kappa / (k_half / h);
    (alpha / (0.049 * (omega_dm / 0.25).powf(0.11) * (h / 0.7).powf(1.22))).powf(-1.0 / 1.11)
}

/// First wavenumber at which t2 = P/P_cdm falls below 1/4 (log-linear interpolation).
fn half_mode(ks: &[f64], t2: &[f64]) -> f64 {
    for i in 1..ks.len() {
        if t2[i] < 0.25 && t2[i - 1] >= 0.25 {
            let f = (t2[i - 1] - 0.25) / (t2[i - 1] - t2[i]);
            return (ks[i - 1].ln() + f * (ks[i].ln() - ks[i - 1].ln())).exp();
        }
    }
    f64::NAN
}

/// `superfluid`: the command `superfluid` of the main project (linear growth of structure
/// if the dark matter is a superfluid with first sound instead of cold particles) on the
/// deflating spacetime.  Writes data/sfdm_deflation_transfer.csv, sfdm_deflation_summary.csv,
/// sfdm_deflation_bounds.csv, superfluid_checks.{txt,csv}.
pub fn run_superfluid(main_data_dir: &str) -> std::io::Result<bool> {
    let t0 = Instant::now();
    let mut rep = Report::new();
    let fid = Params::fiducial();
    let pi = std::f64::consts::PI;
    // mean dark-matter density today (the constants of the command of the main project)
    let h100 = 1e5 / MPC; // 100 km/s/Mpc in 1/s
    let rho_dm = 3.0 * h100 * h100 / (8.0 * pi * G_NEWTON) * fid.omega_c; // kg/m^3
    let rho_dm_ev = rho_dm * C_LIGHT * C_LIGHT / EV * 1e-6; // eV/cm^3
    let hbar_c = H_PLANCK * C_LIGHT / (2.0 * pi) / EV * 100.0; // eV cm
    let g_states = 8.0;
    let x0_of_mass = |m_kev: f64| -> f64 {
        let m = 1e3 * m_kev;
        let n = rho_dm_ev / m;
        hbar_c * (6.0 * pi * pi * n / g_states).powf(1.0 / 3.0) / m
    };
    let kpc = MPC / 1e3;
    let r_tf_kpc = |cs0: f64| pi * cs0 * C_LIGHT / (4.0 * pi * G_NEWTON * rho_dm).sqrt() / kpc;
    let omega_nu1 = neutrino_factor() * fid.omega_gamma();
    let omega_dm = fid.omega_c / (fid.h * fid.h);
    let nk = 57;
    let ks: Vec<f64> = (0..nk).map(|i| (0.05f64.ln() + (250f64.ln() - 0.05f64.ln()) * i as f64 / (nk - 1) as f64).exp()).collect();
    let masses = [0.3, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.5]; // keV
    let speeds = [3.0e-12, 1.0e-11, 3.0e-11, 1.0e-10, 1.0e-9, 1.0e-8, 4.5e-8]; // c_s0 / c
    let (r_llr, r_bbn) = allowed_rates();
    let theta_fid = Thermo::new(&Background::new(&fid, &Deflation::none())).theta_star_100;
    // the backgrounds: (label, rate, closure)
    let cases: [(usize, f64, Closure); 4] = [(0, 0.0, Closure::Momentum), (1, r_llr, Closure::Momentum), (2, r_bbn, Closure::Momentum), (3, r_bbn, Closure::Pressure)];
    let interp = |xs: &[f64], khs: &[f64], target: f64| -> f64 {
        for i in 1..xs.len() {
            let (a, b) = (khs[i - 1], khs[i]);
            if a.is_finite() && b.is_finite() && (a - target) * (b - target) <= 0.0 {
                let f = (target.ln() - a.ln()) / (b.ln() - a.ln());
                return (xs[i - 1].ln() + f * (xs[i].ln() - xs[i - 1].ln())).exp();
            }
        }
        f64::NAN
    };
    let mut summary: Vec<Vec<f64>> = Vec::new();
    let mut bounds: Vec<Vec<f64>> = Vec::new();
    let mut trows: Vec<Vec<f64>> = (0..nk).map(|i| vec![ks[i], ks[i] / fid.h]).collect();
    let mut tcols: Vec<String> = vec!["k".into(), "k_h".into()];
    let mut khs: Vec<(Vec<f64>, Vec<f64>)> = Vec::new();
    let mut hs: Vec<f64> = Vec::new();
    let mut g_early: Vec<f64> = Vec::new();
    for (ib, r, closure) in cases.iter() {
        let d = deflation(*r, *closure);
        let h = if *r == 0.0 { fid.h } else { h_at_fixed_theta(&fid, &d, theta_fid) };
        hs.push(h);
        let base = Params { h, ..fid };
        let g_early_b = Background::new(&base, &d).s_early().exp();
        g_early.push(g_early_b);
        let (p_cdm, _) = matter_power(&base, &d, &ks, 1e-5);
        println!("[{:>4.0} s] background {}: gsum = {:.5} H_0, closure {:?}, H0 = {:.3}: cold dark matter reference", elapsed(&t0), ib, r, closure, KMS * h);
        for i in 0..nk {
            trows[i].push(p_cdm[i]);
        }
        tcols.push(format!("P_cdm_b{}", ib));
        let mut run = |fam: usize, par: f64, sf: SuperfluidDm, label: String| -> f64 {
            let p = Params { superfluid_dm: Some(sf), ..base };
            let (pk, _) = matter_power(&p, &d, &ks, 1e-5);
            let t2: Vec<f64> = pk.iter().zip(&p_cdm).map(|(a, b)| a / b).collect();
            let kh = half_mode(&ks, &t2);
            let (_, _, cs2_0) = sf.state(1.0);
            let cs0 = cs2_0.sqrt();
            // the fluid at the time of nucleosynthesis (a = 1e-9): density in units of one neutrino species
            let (ratio_bbn, _, _) = sf.state(1e-9);
            let dn_bbn = fid.omega_c * ratio_bbn * 1e-9 / omega_nu1;
            println!("[{:>4.0} s] background {} {:<24} c_s0 = {:.3e} c   half-mode k = {:>7.2} /Mpc   (thermal relic of {:.2} keV)", elapsed(&t0), ib, label, cs0, kh, wdm_mass_of_half_mode(kh, omega_dm, fid.h));
            summary.push(vec![*ib as f64, fam as f64, par, cs0, cs0 * C_LIGHT, kh, kh / fid.h, wdm_mass_of_half_mode(kh, omega_dm, fid.h), if fam == 1 { r_tf_kpc(cs0) } else { f64::NAN }, dn_bbn, g_early_b]);
            for i in 0..nk {
                trows[i].push(t2[i]);
            }
            tcols.push(format!("{}_b{}", label, ib));
            kh
        };
        let mut kh_fermi = Vec::new();
        for &m in &masses {
            kh_fermi.push(run(0, m, SuperfluidDm::Degenerate { x0: x0_of_mass(m) }, format!("T2_fermi_m{}keV", m)));
        }
        let mut kh_bose = Vec::new();
        for &c in &speeds {
            kh_bose.push(run(1, c, SuperfluidDm::Bose { cs0_sq: c * c }, format!("T2_bose_cs{:.1e}", c)));
        }
        // bounds: the parameter at which the half-mode equals that of the Lyman-alpha limits on thermal warm dark
        // matter; the benchmark wavenumbers are those of the standard model, in which the limits were derived
        for &m_wdm in &[3.5, 5.3] {
            let kt = wdm_half_mode(m_wdm, omega_dm, fid.h);
            let m_min = interp(&masses, &kh_fermi, kt);
            let cs_max = interp(&speeds, &kh_bose, kt);
            println!("background {}: thermal relic of {} keV: half-mode k = {:.1} /Mpc  ->  fermion mass > {:.3} keV;  condensate c_s0 < {:.3e} c, Thomas-Fermi radius < {:.4} kpc", ib, m_wdm, kt, m_min, cs_max, r_tf_kpc(cs_max));
            bounds.push(vec![*ib as f64, m_wdm, kt, kt / fid.h, m_min, x0_of_mass(m_min), x0_of_mass(m_min) * C_LIGHT, cs_max, cs_max * C_LIGHT, r_tf_kpc(cs_max)]);
        }
        khs.push((kh_fermi, kh_bose));
    }
    let bg_meta = format!("0 = no deflation (the main project); 1 = gsum = {:.6} H_0 (lunar laser ranging), closure \"Momentum\"; 2 = gsum = {:.6} H_0 (nucleosynthesis), closure \"Momentum\"; 3 = the same rate, closure \"Pressure\"; three equal rates, assumptions A1, A2, A3; each background has the Hubble constant that keeps the angle theta_* ({:.3}, {:.3}, {:.3}, {:.3} km/s/Mpc)", r_llr, r_bbn, KMS * hs[0], KMS * hs[1], KMS * hs[2], KMS * hs[3]);
    let tref: Vec<&str> = tcols.iter().map(|s| s.as_str()).collect();
    write_csv("sfdm_deflation_transfer.csv",
        &[("content", "linear matter power spectrum today if the dark matter is a superfluid, relative to cold dark matter on the same background: T2 = P/P_cdm, on the deflating spacetime".to_string()),
          ("background", bg_meta.clone()),
          ("models", "fermi: degenerate Fermi gas of 8 internal states and particle mass m (weak pairing; first sound c_s = v_F/sqrt(3)); bose: condensate of bosons with repulsive contact interaction and sound speed c_s0 (in units of c) at the mean density today; suffix _bK: background K".to_string()),
          ("units", "k in 1/Mpc, k_h in h/Mpc (h = 0.6736), P_cdm in Mpc^3".to_string()),
          ("command", "deflation_cjt_ks superfluid".to_string())],
        &tref, &trows)?;
    write_csv("sfdm_deflation_summary.csv",
        &[("content", "half-mode wavenumber (where P/P_cdm first falls to 1/4) of superfluid dark matter on the deflating spacetime".to_string()),
          ("background", bg_meta.clone()),
          ("family", "0 = degenerate Fermi gas, parameter = particle mass in keV; 1 = Bose condensate, parameter = c_s0/c".to_string()),
          ("columns", "cs0: sound speed today at the mean density (units of c); m_thermal_keV: mass of the thermal warm-dark-matter relic with the same half-mode in the standard model (fit of Viel et al. 2005); R_TF_kpc: radius of the Thomas-Fermi core, pi c_s/sqrt(4 pi G rho) with Newton's constant of today (Bose only); dN_bbn: density of the fluid at a = 1e-9 in units of one neutrino species; GN_early: Newton's constant in the early universe over its value today".to_string()),
          ("command", "deflation_cjt_ks superfluid".to_string())],
        &["background", "family", "parameter", "cs0", "cs0_m_per_s", "k_half_Mpc", "k_half_h_Mpc", "m_thermal_keV", "R_TF_kpc", "dN_bbn", "GN_early"], &summary)?;
    write_csv("sfdm_deflation_bounds.csv",
        &[("content", "limits on superfluid dark matter on the deflating spacetime from the requirement that its half-mode wavenumber is not smaller than that of thermal warm dark matter at the Lyman-alpha mass limits (benchmark wavenumbers of the standard model)".to_string()),
          ("background", bg_meta.clone()),
          ("columns", "m_wdm_keV: thermal-relic mass limit used as benchmark; m_fermion_min_keV: smallest mass of the fermions (8 internal states) in the weak-pairing regime; vF0: their Fermi velocity today at the mean density; cs0_max: largest sound speed of a Bose condensate today at the mean density; R_TF_max_kpc: the corresponding largest Thomas-Fermi core radius".to_string()),
          ("mean_dark_matter_density", format!("{:.5e} kg/m^3 = {:.5e} eV/cm^3", rho_dm, rho_dm_ev)),
          ("command", "deflation_cjt_ks superfluid".to_string())],
        &["background", "m_wdm_keV", "k_half_Mpc", "k_half_h_Mpc", "m_fermion_min_keV", "x0_max", "vF0_max_m_per_s", "cs0_max", "cs0_max_m_per_s", "R_TF_max_kpc"], &bounds)?;

    // ---- checks ----
    // (1) without deflation: the files of the main project
    let main_sum = read_numbers(&format!("{}/sfdm_summary.csv", main_data_dir));
    let main_bnd = read_numbers(&format!("{}/sfdm_bounds.csv", main_data_dir));
    let own_sum: Vec<Vec<f64>> = summary.iter().filter(|r| r[0] == 0.0).map(|r| r[1..10].to_vec()).collect();
    let own_bnd: Vec<Vec<f64>> = bounds.iter().filter(|r| r[0] == 0.0).map(|r| r[1..].to_vec()).collect();
    let dev_sum = main_sum.as_ref().map(|m| table_deviation(&own_sum, &m.iter().map(|r| r[0..9].to_vec()).collect::<Vec<_>>())).unwrap_or(f64::INFINITY);
    let dev_bnd = main_bnd.as_ref().map(|m| table_deviation(&own_bnd, m)).unwrap_or(f64::INFINITY);
    rep.add("def-U1", "numerical",
        &format!("superfluid dark matter without deflation: the copy of the Einstein-Boltzmann program reproduces the files data/sfdm_summary.csv and data/sfdm_bounds.csv of the main project (half-mode wavenumbers of {} models; limits on the fermion mass and on the sound speed of a condensate) to the digits written there (largest relative deviations {:.1e} and {:.1e})", masses.len() + speeds.len(), dev_sum, dev_bnd),
        dev_sum < 1e-9 && dev_bnd < 1e-9);
    // (2) the effect of the deflation on the half-mode wavenumbers and on the limits
    let rel = |ib: usize| -> (f64, f64) {
        let mut lo = f64::INFINITY;
        let mut hi = f64::NEG_INFINITY;
        for (a, b) in khs[ib].0.iter().zip(&khs[0].0).chain(khs[ib].1.iter().zip(&khs[0].1)) {
            if a.is_finite() && b.is_finite() {
                lo = lo.min(a / b - 1.0);
                hi = hi.max(a / b - 1.0);
            }
        }
        (lo, hi)
    };
    let (l1, l2, l3) = (rel(1), rel(2), rel(3));
    let b = |ib: usize, m_wdm: f64, c: usize| bounds.iter().find(|r| r[0] == ib as f64 && r[1] == m_wdm).unwrap()[c];
    let closure_dev = khs[2].0.iter().zip(&khs[3].0).chain(khs[2].1.iter().zip(&khs[3].1)).filter(|(a, b)| a.is_finite() && b.is_finite()).map(|(a, b)| (a / b - 1.0).abs()).fold(0.0, f64::max);
    rep.add("def-U2", "numerical",
        &format!("superfluid dark matter on the deflating spacetime (data/sfdm_deflation_summary.csv, sfdm_deflation_bounds.csv). The wavenumber below which the first sound of the fluid suppresses structure (half-mode) changes by {:+.3} to {:+.3} per cent at the rate that lunar laser ranging allows and by {:+.2} to {:+.2} per cent at the rate that nucleosynthesis allows ({} models; each background at the Hubble constant that keeps the angle theta_*). The limits that follow from the Lyman-alpha bound on thermal warm dark matter of 5.3 keV move from m > {:.3} keV (fermions, weak pairing) and c_s0 < {:.3e} c (condensate; Thomas-Fermi radius < {:.3} pc) without deflation to m > {:.3} keV, c_s0 < {:.3e} c ({:.3} pc) at the rate of lunar laser ranging and m > {:.3} keV, c_s0 < {:.3e} c ({:.3} pc) at the rate of nucleosynthesis. The suppression is set while the fluid still has pressure, long before today, when gravity was weaker: the Jeans wavenumber is proportional to the square root of Newton's constant, which in the early universe was {:.4} and {:.4} of today's value at the two rates (square root minus one: {:+.3} and {:+.2} per cent). The two closures give the same half-mode wavenumbers to {:.1e}", 100.0 * l1.0, 100.0 * l1.1, 100.0 * l2.0, 100.0 * l2.1, masses.len() + speeds.len(), b(0, 5.3, 4), b(0, 5.3, 7), 1e3 * b(0, 5.3, 9), b(1, 5.3, 4), b(1, 5.3, 7), 1e3 * b(1, 5.3, 9), b(2, 5.3, 4), b(2, 5.3, 7), 1e3 * b(2, 5.3, 9), g_early[1], g_early[2], 100.0 * (g_early[1].sqrt() - 1.0), 100.0 * (g_early[2].sqrt() - 1.0), closure_dev),
        l1.0.abs() < 0.01 && l1.1.abs() < 0.01 && l2.0.abs() < 0.1 && l2.1.abs() < 0.1 && closure_dev < 2e-3 && l3.0.abs() < 0.1 && bounds.iter().all(|r| r[4].is_finite() && r[7].is_finite()));
    finish(&mut rep, &t0, "superfluid", "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham: checks of superfluid dark matter on the deflating spacetime")
}
