//! Command-line program of PROJECT spacetimes, Revision_Deflation.
//!
//! Run it from the folder `Revision_Deflation` (the data files are written to
//! `data/` below the current directory; the notebook matrices of the main project
//! are read from `../data/`):
//!
//!     cargo run --release --manifest-path code/deflation-rs/Cargo.toml -- <command>
//!
//! `deflation help` lists the commands.

mod cmd_channels;
mod cmd_matter;
mod cmd_paired;
mod cmd_spectra;
mod cmd_symmetry;

use spacetimes_deflation::io::{write_csv, write_csv_str, write_report};
use spacetimes_deflation::curved::verify_curved;
use spacetimes_deflation::geometry::{verify_geodesics, verify_geometry};
use spacetimes_deflation::metric::{Point, Sig};

/// Folder of the data files of the main project (the matrices extracted from the notebooks).
fn main_data_dir() -> String {
    std::env::var("SPT_MAIN_DATA_DIR").unwrap_or_else(|_| "../data".to_string())
}

/// `matrices`: (1) repeats every check of the real 16x16 Dirac matrices of the main
/// project (they are the matrices of the tangent space of the deflating spacetimes);
/// (2) checks the geometry of the deflating spacetimes; (3) checks the coordinate
/// matrices, the spin connection and the Dirac operators.
/// Writes data/flat_matrix_checks.{txt,csv}, data/deflating_checks.{txt,csv},
/// data/geodesic_1644d.csv, data/geodesic_1671d.csv, data/curvature.csv.
fn cmd_matrices() -> std::io::Result<bool> {
    std::fs::create_dir_all("data")?;
    // ---- (1) the flat matrices, verified again ---------------------------------------
    let nb44 = format!("{}/notebook_matrices_4+4.txt", main_data_dir());
    let nb71 = format!("{}/notebook_matrices_7+1.txt", main_data_dir());
    let (gm, mut flat) = spacetimes::verify::run_all(&nb44, &nb71);
    spacetimes::dirac::verify_dirac_44(&gm, &mut flat);
    spacetimes::dirac::verify_dirac_71(&gm, &mut flat);
    let pair_lines = spacetimes::pairs::run(&gm, &mut flat)?; // also writes data/pair_classification.csv
    std::fs::write("data/pair_classification.txt", pair_lines.join("\n") + "\n")?;
    let (ok_flat, n_flat) = flat.count();
    let all_flat = flat.all_pass();
    flat.add("ALL", "exact", "every check above passed", all_flat);
    write_report(&flat, "flat_matrix_checks.txt", "flat_matrix_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: the checks of the real 16x16 Dirac matrices of the main project, executed again (these matrices are the matrices of the orthonormal frame of the deflating spacetimes); identifiers 44-.. refer to gamma1644, 71-.. to gamma1671", "deflation matrices")?;
    // the routine of the main project wrote data/pair_classification.csv with its own first line; say who called it
    let pc = std::fs::read_to_string("data/pair_classification.csv")?;
    let mut lines: Vec<String> = pc.lines().map(|l| l.to_string()).collect();
    if !lines.is_empty() {
        lines[0] = format!("# {} through the routine pairs::run of the crate spacetimes; command: deflation matrices", spacetimes_deflation::io::generator_line());
    }
    std::fs::write("data/pair_classification.csv", lines.join("\n") + "\n")?;
    println!("flat matrices (tangent space): {} of {} checks passed", ok_flat, n_flat);

    // ---- (2), (3) the deflating spacetimes -----------------------------------------
    let mut rep = spacetimes::verify::Report::new();
    for sig in [Sig::S1644, Sig::S1671] {
        verify_geometry(sig, &mut rep);
        let geo = verify_geodesics(sig, &mut rep);
        let rows: Vec<Vec<f64>> = geo.path.iter().step_by((geo.path.len() / 2000).max(1)).map(|r| r.to_vec()).collect();
        write_csv(&format!("geodesic_{}.csv", sig.tag()),
            &[("content", format!("a geodesic of the deflating {} spacetime with momentum along the deflating coordinate x5 (check {}): g55 = 1, const5 = 0.8, m = 1, P_1 = 0.5, P_5 = 0.3, start at x4 = -2", sig.name(), if sig == Sig::S1671 { "71d-G8" } else { "44d-G9" })),
              ("columns", "affine: affine parameter (proper time for m = 1); x4; u4 = dx4/dlam".to_string()),
              ("command", "deflation matrices".to_string())],
            &["affine", "x4", "u4"], &rows)?;
        verify_curved(&gm, sig, &mut rep);
    }
    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "exact", "every check above passed", all);
    write_report(&rep, "deflating_checks.txt", "deflating_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: checks of the geometry, of the Dirac matrices, of the spin connection and of the Dirac operators of the deflating spacetimes (44d-..: 4+4 with deflating x5, x6, x7; 71d-..: 7+1 with deflating x5, x6, x7)", "deflation matrices")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\ndeflating spacetimes: {} of {} checks passed", ok, n);

    // ---- curvature table for a few sets of rates ---------------------------------------
    let sets: [[f64; 3]; 4] = [[1.0, 1.0, 1.0], [1.0, 2.0, 3.0], [0.5, 0.75, 5.0 / 3.0], [2.0, 2.0, 5.0]];
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for (s, sig) in [Sig::S1644, Sig::S1671].iter().enumerate() {
        for rate in &sets {
            let p = Point { sig: *sig, rate: *rate, b: [1.0; 3] };
            let (ric, r, ein) = p.curvature_closed();
            rows.push(vec![if s == 0 { 1644.0 } else { 1671.0 }, rate[0], rate[1], rate[2], ric[4], ric[5], ric[6], ric[7], r, ein[0], ein[4], ein[5], ein[6], ein[7]]);
        }
    }
    write_csv("curvature.csv",
        &[("content", "curvature of the deflating spacetimes for several sets of rates, from the closed forms of check G5 (mixed components; they do not depend on x4)".to_string()),
          ("columns", "spacetime: 1644 = 4+4, 1671 = 7+1; g55, g66, g77: rates; Ric44, Ric55, Ric66, Ric77: mixed Ricci tensor, components (mu, mu); Rscalar: curvature scalar; Ein00 (= Ein11 = Ein22 = Ein33), Ein44, Ein55, Ein66, Ein77: mixed Einstein tensor, components (mu, mu)".to_string()),
          ("command", "deflation matrices".to_string())],
        &["spacetime", "g55", "g66", "g77", "Ric44", "Ric55", "Ric66", "Ric77", "Rscalar", "Ein00", "Ein44", "Ein55", "Ein66", "Ein77"], &rows)?;
    Ok(all_flat && all)
}

/// `modes`: the solutions of the two Dirac equations on the deflating spacetimes, mode by mode.
/// 7+1: the number of particles that the deflation creates (numerical, compared with the
/// exact Fermi-Dirac law for one direction; anisotropic rates).  4+4: the bounded
/// solutions of modes that depend on the deflating times (standing waves that die out).
/// Writes data/creation_single.csv, creation_anisotropic.csv, creation_history.csv,
/// extinction_profile.csv, extinction_checks.csv, mode_checks.{txt,csv}.
fn cmd_modes() -> std::io::Result<bool> {
    use spacetimes_deflation::modes::{bounded_profile_exact, fermi_dirac_creation, Background, Dirac44, Dirac71, Mode};
    let gm = spacetimes::gamma::Gammas::build();
    let mut rep = spacetimes::verify::Report::new();
    let d71 = Dirac71::new(&gm);
    let d44 = Dirac44::new(&gm);
    let iso = Background { rate: [1.0, 1.0, 1.0], cst: [1.0, 1.0, 1.0] };

    // ---- 7+1, one direction: scan of E_perp/g55 -----------------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut worst: f64 = 0.0;
    let mut worst_unit: f64 = 0.0;
    for k in 1..=12 {
        let eps = 0.25 * k as f64;
        let mode = Mode { m: eps, p: [0.0; 4], kappa: [1.0, 0.0, 0.0] };
        let r = d71.creation(&mode, &iso, 1e-9, 1000.0, 0.05, 0.01, false);
        let exact = fermi_dirac_creation(eps, 1.0);
        worst = worst.max(((r.n_min - exact) / exact).abs()).max(((r.n_max - exact) / exact).abs());
        worst_unit = worst_unit.max(r.unitarity);
        rows.push(vec![eps, r.n_min, r.n_max, exact, r.n_mean / exact - 1.0, r.unitarity, r.steps as f64]);
    }
    write_csv("creation_single.csv",
        &[("content", "7+1 deflating spacetime: number of particles created by the deflation in a mode with one non-zero wavenumber along a deflating coordinate (k_5 = 1, g55 = 1, const5 = 1, p = 0, m = E_perp), per spin state, numerical and exact".to_string()),
          ("columns", "E_perp_over_g55: E_perp/g55; n_min, n_max: smallest and largest eigenvalue of the 8x8 matrix of created particles; n_exact = 1/(exp(2 pi E_perp/g55) + 1); rel_dev = mean/exact - 1; unitarity: deviation of the evolved states from orthonormality; steps: number of integration steps".to_string()),
          ("method", "commutator-free Magnus method of order four from k_d = 1e-9 E_perp to k_d = 1000 E_perp; projection in the first superadiabatic basis (modes.rs)".to_string()),
          ("command", "deflation modes".to_string())],
        &["E_perp_over_g55", "n_min", "n_max", "n_exact", "rel_dev", "unitarity", "steps"], &rows)?;
    rep.add("71d-M1", "numerical", &format!("pair creation by the deflation, one direction: for E_perp/g55 = 0.25, 0.5, .., 3.0 the number of particles created per spin state in a mode that starts empty agrees with 1/(exp(2 pi E_perp/g55) + 1) to a relative deviation of at most {:.1e} (the numbers range from 0.17 to 6.5e-9); the eight spin states are created equally; the evolved states stay orthonormal to {:.1e}", worst, worst_unit), worst < 1e-4 && worst_unit < 1e-9);

    // ---- 7+1, equal rates, general direction -------------------------------------------
    let mode = Mode { m: 1.0, p: [0.0, 0.1, 0.2, 0.3], kappa: [0.5, -0.8, 0.3] };
    let bg = Background { rate: [1.3, 1.3, 1.3], cst: [1.0, 0.7, 2.0] };
    let r = d71.creation(&mode, &bg, 1e-9, 1000.0, 0.05, 0.01, false);
    let exact = fermi_dirac_creation(mode.eps(), 1.3);
    let dev = ((r.n_min - exact) / exact).abs().max(((r.n_max - exact) / exact).abs());
    rep.add("71d-M2", "numerical", &format!("equal rates g55 = g66 = g77 = 1.3 (constants 1, 0.7, 2), mode with m = 1, p = (0, 0.1, 0.2, 0.3), (k_5, k_6, k_7) = (0.5, -0.8, 0.3): the created number per spin state is {:.8e} for all eight states, the law 1/(exp(2 pi E_perp/g55) + 1) gives {:.8e} (relative deviation {:.1e}): with equal rates the result depends neither on the direction of k_t nor on the constants", r.n_mean, exact, dev), dev < 1e-4);

    // ---- 7+1, anisotropic: two directions with rates 1 and 2 ---------------------------
    // delta = (x4 at which q_6 = E_perp) - (x4 at which q_5 = E_perp); k_5 = E_perp fixes the first at x4 = 0
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut far_ok = true;
    let mut far_worst: f64 = 0.0;
    let mut split_max: f64 = 0.0;
    for (g66, label) in [(2.0_f64, 2.0_f64), (1.2, 1.2), (0.5, 0.5)] {
        let bg = Background { rate: [1.0, g66, 1.0], cst: [1.0, 1.0, 1.0] };
        let (fd5, fd6) = (fermi_dirac_creation(1.0, 1.0), fermi_dirac_creation(1.0, g66));
        for k in -24..=32 {
            let delta = 0.25 * k as f64;
            let mode = Mode { m: 1.0, p: [0.0; 4], kappa: [1.0, (-g66 * delta).exp(), 0.0] };
            let r = d71.creation(&mode, &bg, 1e-9, 1000.0, 0.05, 0.01, false);
            // ratio of the other physical wavenumber to E_perp at the moment when the first one reaches E_perp:
            // delta > 0 (x5 first, at x4 = 0): q_6 = exp(-g66 delta); delta < 0 (x6 first, at x4 = delta): q_5 = exp(g55 delta)
            let ratio = if delta >= 0.0 { (-g66 * delta).exp() } else { delta.exp() };
            rows.push(vec![label, delta, ratio, r.n_min, r.n_mean, r.n_max, fd5, fd6]);
            split_max = split_max.max(r.n_max / r.n_min);
            if ratio <= (-3.0f64).exp() * 1.0000001 {
                let fd = if delta > 0.0 { fd5 } else { fd6 };
                let dev = ((r.n_mean - fd) / fd).abs();
                far_worst = far_worst.max(dev);
                far_ok &= dev < 0.02;
            }
        }
    }
    write_csv("creation_anisotropic.csv",
        &[("content", "7+1 deflating spacetime, unequal rates: particles created in a mode with wavenumbers along x5 and x6 (g55 = 1; E_perp = m = 1, p = 0, k_7 = 0, constants 1)".to_string()),
          ("columns", "g66; delay: x4 at which q_6 = E_perp minus x4 at which q_5 = E_perp, in units of 1/g55 (delay > 0: x5 becomes energetic first); ratio: the other physical wavenumber divided by E_perp at the moment when the first one reaches E_perp; n_min, n_mean, n_max: smallest, mean and largest eigenvalue of the 8x8 matrix of created particles; fd5 = 1/(exp(2 pi E_perp/g55) + 1); fd6 = 1/(exp(2 pi E_perp/g66) + 1)".to_string()),
          ("command", "deflation modes".to_string())],
        &["g66", "delay", "ratio", "n_min", "n_mean", "n_max", "fd5", "fd6"], &rows)?;
    rep.add("71d-M3", "numerical", &format!("unequal rates (g55 = 1 with g66 = 2, 1.2 or 0.5), mode with wavenumbers along x5 and x6: whenever the second physical wavenumber is still below exp(-3) E_perp at the moment at which the first reaches E_perp, the mean created number is the Fermi-Dirac law with the rate of the coordinate that becomes energetic first, to within {:.1} per cent; when the two become energetic together the eight spin states are created unequally (largest ratio of the largest to the smallest eigenvalue in the scan: {:.0}) and no closed form is known (data/creation_anisotropic.csv)", 100.0 * far_worst, split_max), far_ok);

    // ---- 7+1: history of the adiabatic particle number ----------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for (id, mode, bg) in [
        (1.0, Mode { m: 1.0, p: [0.0; 4], kappa: [1.0, 0.0, 0.0] }, iso),
        (2.0, Mode { m: 0.5, p: [0.0; 4], kappa: [1.0, 0.0, 0.0] }, iso),
        (3.0, Mode { m: 1.0, p: [0.0; 4], kappa: [1.0, 1.0, 0.0] }, Background { rate: [1.0, 2.0, 1.0], cst: [1.0, 1.0, 1.0] }),
    ] {
        let r = d71.creation(&mode, &bg, 1e-9, 1000.0, 0.05, 0.01, true);
        for h in &r.history {
            rows.push(vec![id, h[0], h[1], h[2]]);
        }
    }
    write_csv("creation_history.csv",
        &[("content", "7+1 deflating spacetime: mean number of particles (per spin state, first superadiabatic basis) in a mode that starts empty, as a function of x4".to_string()),
          ("columns", "case: 1 = E_perp/g55 = 1, one direction; 2 = E_perp/g55 = 0.5, one direction; 3 = E_perp = 1, wavenumbers along x5 (g55 = 1) and x6 (g66 = 2); x4 (the physical wavenumber along x5 equals 1 at x4 = 0); k_d_over_E_perp; n_cr: mean adiabatic particle number".to_string()),
          ("command", "deflation modes".to_string())],
        &["case", "x4", "k_d_over_E_perp", "n_cr"], &rows)?;

    // ---- 4+4: bounded solutions -----------------------------------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut prof_rows: Vec<Vec<f64>> = Vec::new();
    let (mut sw, mut ch, mut amp, mut spread_one, mut spread_all, mut offset_dev, mut prof_dev) = (0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64);
    let cases: Vec<(f64, Mode, Background)> = vec![
        (1.0, Mode { m: 1.0, p: [0.0; 4], kappa: [1.0, 0.0, 0.0] }, iso),
        (2.0, Mode { m: 1.0, p: [0.2, 0.3, -0.4, 0.5], kappa: [0.7, 0.0, 0.0] }, Background { rate: [2.0, 1.0, 1.0], cst: [1.0, 1.0, 1.0] }),
        (3.0, Mode { m: 0.3, p: [0.0, 0.0, 0.0, 0.0], kappa: [0.0, 0.0, 1.0] }, Background { rate: [1.0, 1.0, 0.25], cst: [1.0, 1.0, 1.0] }),
        (4.0, Mode { m: 1.0, p: [0.0, 0.1, 0.2, 0.3], kappa: [0.5, -0.8, 0.3] }, Background { rate: [1.3, 1.3, 1.3], cst: [1.0, 0.7, 2.0] }),
        (5.0, Mode { m: 1.0, p: [0.0, 0.1, 0.2, 0.3], kappa: [0.5, -0.8, 0.3] }, Background { rate: [1.3, 0.7, 2.0], cst: [1.0, 0.5, 2.0] }),
    ];
    for (id, mode, bg) in &cases {
        let r = d44.bounded(mode, bg, 30.0, 1e-9, 0.02, 0.005);
        sw = sw.max(r.standing_wave_dev);
        ch = ch.max(r.charge_ratio);
        spread_all = spread_all.max(r.log_amp_max - r.log_amp_min);
        amp = amp.max((r.log_amp_min - r.s_wkb).abs()).max((r.log_amp_max - r.s_wkb).abs());
        rows.push(vec![*id, mode.eps(), r.x4_turn, r.x4_late, r.s_wkb, r.log_amp_min, r.log_amp_max, r.standing_wave_dev, r.charge_ratio, r.steps as f64]);
        // profile, with the exact form where one rate governs (cases 1, 2, 3: one direction; case 4: equal rates)
        let which = mode.kappa.iter().position(|x| *x != 0.0).unwrap();
        let single = mode.kappa.iter().filter(|x| **x != 0.0).count() == 1 || (bg.rate[0] == bg.rate[1] && bg.rate[1] == bg.rate[2]);
        let rate = bg.rate[which];
        let mu = mode.eps() / rate;
        if single {
            // exact for one governing rate: ln(amplification) - S -> (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi E_perp/rate_t))
            spread_one = spread_one.max(r.log_amp_max - r.log_amp_min);
            let expected = 0.5 * (2.0f64).ln() - 0.5 * (1.0 + (-2.0 * std::f64::consts::PI * mu).exp()).ln();
            offset_dev = offset_dev.max((r.log_amp_min - r.s_wkb - expected).abs());
        }
        // normalization of the exact form: least squares over the oscillatory and the decaying part
        let mut num = 0.0;
        let mut den = 0.0;
        let mut pts: Vec<(f64, f64, f64, f64)> = Vec::new();
        for row in r.profile.iter().step_by(8) {
            let s = row[1] * mode.eps() / rate;
            let ex = if single && s > 1e-3 && s < 25.0 { bounded_profile_exact(s, mu) } else { f64::NAN };
            if ex.is_finite() && s > 0.02 && s < 12.0 {
                num += row[2] * ex;
                den += ex * ex;
            }
            pts.push((row[0], row[1], row[2], ex));
        }
        let c = if den > 0.0 { num / den } else { f64::NAN };
        for (x4, qe, v, ex) in pts {
            let exn = ex * c;
            if exn.is_finite() && qe * mode.eps() / rate > 0.02 && qe * mode.eps() / rate < 12.0 {
                prof_dev = prof_dev.max((v / exn - 1.0).abs());
            }
            prof_rows.push(vec![*id, x4, qe, v, exn]);
        }
    }
    write_csv("extinction_checks.csv",
        &[("content", "4+4 deflating spacetime: the eight bounded solutions of modes that depend on the deflating times".to_string()),
          ("cases", "1: m = 1, p = 0, (k_5, k_6, k_7) = (1,0,0), rates (1,1,1); 2: m = 1, p = (0.2,0.3,-0.4,0.5), (k_5, k_6, k_7) = (0.7,0,0), rates (2,1,1); 3: m = 0.3, (k_5, k_6, k_7) = (0,0,1), rates (1,1,0.25); 4: m = 1, p = (0,0.1,0.2,0.3), (k_5, k_6, k_7) = (0.5,-0.8,0.3), rates (1.3,1.3,1.3), constants (1,0.7,2); 5: the same mode, rates (1.3,0.7,2), constants (1,0.5,2)".to_string()),
          ("columns", "case; E_perp; x4_turn: x4 at which k_d = E_perp; x4_late: x4 at which the backward integration starts (WKB exponent 30 beyond the turning value); S_wkb: integral of sqrt(k_d^2 - E_perp^2) dx4 between them; log_amp_min, log_amp_max: logarithm of the amplification of the eight solutions between x4_late and the early time (k_d = 1e-9 E_perp); standing_wave_dev: relative difference of the Gram matrices of the positive- and negative-energy parts at the early time; charge_ratio: |psihat^dagger W psihat|/|psihat^dagger psihat|; steps".to_string()),
          ("command", "deflation modes".to_string())],
        &["case", "E_perp", "x4_turn", "x4_late", "S_wkb", "log_amp_min", "log_amp_max", "standing_wave_dev", "charge_ratio", "steps"], &rows)?;
    write_csv("extinction_profile.csv",
        &[("content", "4+4 deflating spacetime: |psihat|^2 of one bounded solution as a function of x4, normalized to its maximum, for the five cases of data/extinction_checks.csv".to_string()),
          ("columns", "case; x4; k_d_over_E_perp; norm2: numerical; exact: 2 u_d |K_(1/2 + i E_perp/rate_t)(u_d)|^2 with u_d = k_d/rate_t, scaled to the numerical curve by least squares (cases 1 to 4, where one rate governs; nan where not evaluated)".to_string()),
          ("command", "deflation modes".to_string())],
        &["case", "x4", "k_d_over_E_perp", "norm2", "exact"], &prof_rows)?;
    rep.add("44d-M1", "numerical", &format!("modes of the 4+4 field that depend on the deflating times (five cases: one, two or three non-zero k_t; equal and unequal rates and constants): beyond the turning value of x4 exactly eight of the sixteen solutions stay bounded; integrated backwards to early times they contain positive- and negative-energy parts of equal weight (relative difference of the two Gram matrices at most {:.1e}): they are standing waves in x4, not travelling particles", sw), sw < 1e-5);
    rep.add("44d-M2", "numerical", &format!("the covariant charge psihat^dagger W psihat of every bounded solution vanishes (ratio to psihat^dagger psihat at most {:.1e}), as it must for a conserved quantity of a solution that dies out", ch), ch < 1e-5);
    rep.add("44d-M3", "numerical", &format!("between the turning value and the late x4 every bounded solution falls by exp(-S) times a factor of order one, S = Int sqrt(k_d^2 - E_perp^2) dx4 = 30 being the WKB exponent (largest difference between the logarithm of the factor and S in the five cases: {:.2}). Where one rate governs (cases 1 to 4) the eight solutions fall by the same factor (spread of the logarithms at most {:.1e}) and its logarithm is S + (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi E_perp/rate_t)), as the exact solution requires (largest deviation {:.1e}); with three unequal rates (case 5) the factor depends on the spin state (spread {:.2}). The dependence on a deflating time dies out as exp(-S), and S grows as exp(rate_t x4)", amp, spread_one, offset_dev, spread_all), amp < 1.0 && spread_one < 1e-6 && offset_dev < 5e-3);
    rep.add("44d-M4", "numerical", &format!("where one rate governs (one non-zero k_t, or equal rates) the computed |psihat|^2 equals the exact form 2 u_d |K_(1/2 + i E_perp/rate_t)(u_d)|^2 (modified Bessel function of the order 1/2 + i E_perp/rate_t, u_d = k_d/rate_t) over the range 0.02 < u_d < 12 with a largest relative deviation of {:.1e}", prof_dev), prof_dev < 1e-5);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "mode_checks.txt", "mode_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: checks of the solutions of the Dirac equations on the deflating spacetimes (71d-M..: pair creation on 7+1; 44d-M..: bounded solutions on 4+4)", "deflation modes")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nmodes: {} of {} checks passed", ok, n);
    Ok(all)
}

/// `history`: every coupling-dependent result of the main project as a function of the
/// deflation clock tau_d = gsum (x4 - x4_ref), along which all dimensionless couplings grow
/// as exp(tau_d) (module history.rs).  Writes data/gap_history.csv, dimer_history.csv,
/// medium_history.csv, vacuum_history.csv, thermal_history.csv, events.csv,
/// history_checks.{txt,csv}.
fn cmd_history() -> std::io::Result<bool> {
    use spacetimes::cjt_normal::{critical_c, vacuum_instability, Medium};
    use spacetimes_deflation::history::{abnormal_c, dimer_point, events, gap_history, thermal_point};
    let pi = std::f64::consts::PI;
    let mut rep = spacetimes::verify::Report::new();

    // ---- thresholds, computed again -------------------------------------------------
    let dc = spacetimes::dimer::critical_d();
    let (cc, ccs) = (critical_c(false), critical_c(true));
    rep.add("def-H1", "numerical", &format!("thresholds of the model, computed again: a pair of fermions is bound if D_VS > {:.5}; matter is self-bound if C > {:.4} (3 pi^2/8 = {:.4}) or, with the negative-energy states, C > {:.4} (3 pi^2/5 = {:.4}). On a deflating spacetime D_VS and C grow as exp(gsum x4), so each threshold is passed at a definite x4", dc, cc, 3.0 * pi * pi / 8.0, ccs, 3.0 * pi * pi / 5.0),
        (dc - 1.67981).abs() < 1e-4 && (cc - 3.0 * pi * pi / 8.0).abs() < 2e-3 && (ccs - 3.0 * pi * pi / 5.0).abs() < 2e-3);

    // ---- the gap along the clock ------------------------------------------------------
    let sets: [(f64, f64, f64); 4] = [(1.0, 0.1, 0.5), (2.0, 0.3, 0.5), (3.0, 1.0, 0.5), (4.0, 0.1, 0.2)];
    let (tau_a, tau_b, n) = ((0.25f64).ln(), (16.0f64).ln(), 34usize);
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let (mut tc_dev, mut order_ok, mut mono_ok) = (0.0_f64, true, true);
    let (mut slope_lo, mut slope_hi) = (f64::INFINITY, 0.0_f64);
    for (id, ms, kf) in sets {
        let h = gap_history(1.0, ms, kf, tau_a, tau_b, n, true);
        for i in 0..n {
            let (tau, r) = h[i];
            // d ln(gap_V)/d tau_d by central differences where the neighbours have a gap
            let dln = if i > 0 && i + 1 < n && h[i - 1].1.gap_v > 0.0 && h[i + 1].1.gap_v > 0.0 {
                (h[i + 1].1.gap_v.ln() - h[i - 1].1.gap_v.ln()) / (h[i + 1].0 - h[i - 1].0)
            } else {
                f64::NAN
            };
            let follow = if dln.is_finite() && dln > 0.0 { r.gap_v / dln } else { f64::NAN };
            rows.push(vec![id, ms, kf, tau, r.g, r.alpha, r.alpha / ms, r.g * r.g / (ms * ms), r.lambda_s, r.lambda_p, r.gap_v, r.gap_t, r.gap_s, r.tc_over_gap, dln, follow]);
            if r.gap_v > 0.0 && r.gap_v < 0.05 && r.tc_over_gap.is_finite() {
                tc_dev = tc_dev.max((r.tc_over_gap - 0.567).abs());
            }
            order_ok &= r.gap_v >= r.gap_s && r.gap_s >= r.gap_t;
            if i > 0 {
                mono_ok &= r.gap_v >= h[i - 1].1.gap_v;
            }
            // weak coupling: Delta ~ exp(-1/lambda) and lambda ~ exp(tau_d) give d ln(Delta)/d tau_d = 1/lambda
            let lam = r.lambda_s + r.lambda_p;
            if dln.is_finite() && r.gap_v > 1e-6 && r.gap_v < 0.02 {
                slope_lo = slope_lo.min(dln * lam);
                slope_hi = slope_hi.max(dln * lam);
            }
        }
    }
    write_csv("gap_history.csv",
        &[("content", "the gap of the paired state along the deflation clock tau_d = gsum (x4 - x4_ref): equilibrium gap at the Fermi surface and critical temperature for the couplings of each moment (adiabatic history)".to_string()),
          ("model", "gap equation of the main project (exchange of the radial mode and of the phase mode of the Vorton-Scalar); fermion mass m = 1; g_VS = exp(tau_d/2), i.e. alpha_VS = exp(tau_d)/(4 pi): the reference time is the moment at which g_VS = 1".to_string()),
          ("columns", "set: 1 = (m_s, k_F) = (0.1, 0.5), 2 = (0.3, 0.5), 3 = (1.0, 0.5), 4 = (0.1, 0.2), in units of m; tau_d; g; alpha = g^2/(4 pi); D_VS = alpha m/m_s; C = g^2 m^2/m_s^2; lambda_S, lambda_P: dimensionless pairing strengths of the two modes; gap_V, gap_T, gap_S: gap of the vector channel, of the two-form channel and with the radial mode only, in units of the Fermi kinetic energy E_F (0 = below 1e-7 E_F); Tc_over_gap_V; dln_gap_dtau = d ln(gap_V)/d tau_d; follow = gap_V/(d ln gap_V/d tau_d) in units of E_F: the gap can follow the deflation only if gsum is much smaller than this".to_string()),
          ("command", "deflation history".to_string())],
        &["set", "m_s", "k_F", "tau_d", "g", "alpha", "D_VS", "C", "lambda_S", "lambda_P", "gap_V", "gap_T", "gap_S", "Tc_over_gap_V", "dln_gap_dtau", "follow"], &rows)?;
    rep.add("def-H2", "numerical", &format!("along the clock, wherever the gap is below 0.05 E_F the critical temperature is 0.567 times the gap (largest deviation {:.4}); the gap of the vector channel is at every moment at least as large as the gap with the radial mode alone, and that at least as large as the gap of the two-form channel; the gap never decreases as the deflation proceeds (four sets of m_s/m and k_F/m, 34 moments each)", tc_dev), tc_dev < 0.012 && order_ok && mono_ok);
    rep.add("def-H3", "numerical", &format!("growth of the gap: for a weakly coupled state the gap is proportional to exp(-1/lambda), where lambda = lambda_S + lambda_P is the sum of the pairing strengths of the radial mode and of the phase mode in the vector channel, and lambda grows as exp(tau_d), which gives d ln(gap)/d tau_d = 1/lambda; in the computed histories lambda d ln(gap)/d tau_d lies between {:.2} and {:.2} wherever 1e-6 < gap/E_F < 0.02: the gap grows much faster than the couplings, by one e-folding in the time 1/(gsum/lambda)", slope_lo, slope_hi), slope_lo > 0.6 && slope_hi < 1.6);

    // ---- the bound pair along the clock -----------------------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut ds: Vec<f64> = (0..70).map(|i| 0.2 * (150.0f64).powf(i as f64 / 69.0)).collect();
    for f in [0.9, 0.95, 0.98, 0.99, 1.01, 1.02, 1.04, 1.08, 1.16] {
        ds.push(dc * f);
    }
    ds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pts: Vec<[f64; 4]> = ds.iter().map(|d| dimer_point(*d)).collect();
    for p in &pts {
        rows.push(vec![p[0], (p[0] / dc).ln(), p[1], p[2], p[3]]);
    }
    write_csv("dimer_history.csv",
        &[("content", "two fermions in the Yukawa potential of the radial mode along the deflation clock: D_VS = alpha_VS m/m_s grows as exp(gsum x4)".to_string()),
          ("columns", "D; tau_pair = ln(D/D_c) = gsum (x4 - x4 at which the pair becomes bound); a_ms: scattering length times m_s; bound_states; Eb: binding energy times m/m_s^2 (0 = unbound)".to_string()),
          ("critical_D", format!("{:.6}", dc)),
          ("command", "deflation history".to_string())],
        &["D", "tau_pair", "a_ms", "bound_states", "Eb"], &rows)?;
    let e = |f: f64| dimer_point(dc * f)[3];
    let (e2, e4, e8) = (e(1.02), e(1.04), e(1.08));
    let (x1, x2) = ((e4 / e2).ln() / (2.0f64).ln(), (e8 / e4).ln() / (2.0f64).ln());
    let (am, ap) = (dimer_point(dc * 0.99)[1], dimer_point(dc * 1.01)[1]);
    rep.add("def-H4", "numerical", &format!("the moment at which a pair becomes bound: just below D_c the scattering length is large and negative ({:.1}/m_s at 0.99 D_c), just above large and positive ({:.1}/m_s at 1.01 D_c); the binding energy starts as a power of the distance from the threshold with the exponents {:.2} (between 2 and 4 per cent above D_c) and {:.2} (between 4 and 8 per cent), that is quadratically: E_b ~ (exp(tau_pair) - 1)^2 m_s^2/m. Because E_b vanishes at the threshold, the pair cannot follow the deflation adiabatically at first, however slow the deflation is", am, ap, x1, x2),
        am < -20.0 && ap > 20.0 && (x1 - 2.0).abs() < 0.25 && (x2 - 2.0).abs() < 0.35);

    // ---- normal matter along the clock ---------------------------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let cs: Vec<f64> = (0..60).map(|i| 0.05 * (1200.0f64).powf(i as f64 / 59.0)).collect();
    for kf in [0.2, 0.5, 1.0] {
        for sea in [false, true] {
            for c in &cs {
                let md = Medium { c: *c, sea };
                let st = md.state(kf);
                rows.push(vec![kf, if sea { 1.0 } else { 0.0 }, *c, st.x, st.eps / st.n, st.p, md.sound_speed_sq(kf)]);
            }
        }
    }
    write_csv("medium_history.csv",
        &[("content", "normal matter of fixed density along the deflation clock: C = g_VS^2 m^2/m_s^2 grows as exp(gsum x4)".to_string()),
          ("columns", "k_F (units of m); sea: 1 if the filled negative-energy states are included; C; x = m*/m: fermion mass in the matter; e_per_n: energy per fermion (units of m; below 1 = bound); pressure (units of m^4); cs2: square of the sound speed".to_string()),
          ("command", "deflation history".to_string())],
        &["k_F", "sea", "C", "x", "e_per_n", "pressure", "cs2"], &rows)?;
    // the critical C of abnormal matter for several densities, and the estimate of the main project,
    // k_F = pi m/(2 sqrt(C)), which holds where the transition happens at weak coupling, i.e. at high density
    let mut ab_txt: Vec<String> = Vec::new();
    let mut ab_ok = true;
    let mut prev_c = f64::INFINITY;
    let mut last_ratio = f64::NAN;
    for kf in [0.2, 0.5, 1.0, 2.0] {
        let c = abnormal_c(kf, false).unwrap_or(f64::NAN);
        let est = pi * pi / (4.0 * kf * kf);
        ab_txt.push(format!("k_F = {} m: C = {:.3} ({:.2} times pi^2 m^2/(4 k_F^2))", kf, c, c / est));
        ab_ok &= c.is_finite() && c < prev_c;
        prev_c = c;
        last_ratio = c / est;
    }
    ab_ok &= (last_ratio - 1.0).abs() < 0.05;
    rep.add("def-H5", "numerical", &format!("matter of fixed density loses the fermion mass (the Vorton-Scalar loses its expectation value in the matter) when C passes a critical value that falls with the density: {}. The estimate C = pi^2 m^2/(4 k_F^2) of the main project is reached at high density, where the transition happens at weak coupling; at low density the critical C is larger. On a deflating spacetime every piece of matter becomes abnormal at a definite x4, the denser the earlier", ab_txt.join("; ")), ab_ok);

    // ---- the vacuum potential along the clock ----------------------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut prev = f64::INFINITY;
    let mut dec = true;
    for c in &cs {
        let (xm, xz) = vacuum_instability(*c);
        rows.push(vec![*c, xm, xz]);
        if xz.is_finite() {
            dec &= xz <= prev * 1.0000001;
            prev = xz;
        }
    }
    write_csv("vacuum_history.csv",
        &[("content", "the one-loop potential of the Vorton-Scalar in the vacuum along the deflation clock: where it turns over".to_string()),
          ("columns", "C; x_max: field (units of the vacuum value) at the maximum beyond the vacuum; x_zero: field beyond which the potential lies below its vacuum value (inf = beyond 1e30)".to_string()),
          ("command", "deflation history".to_string())],
        &["C", "x_max", "x_zero"], &rows)?;
    let (_, xz1) = vacuum_instability(1.0);
    let (_, xz10) = vacuum_instability(10.0);
    rep.add("def-H6", "numerical", &format!("the field beyond which the one-loop potential of the Vorton-Scalar lies below its vacuum value decreases monotonically as C grows: {:.1} v_VS at C = 1, {:.2} v_VS at C = 10. As the deflation proceeds the barrier that protects the vacuum moves in towards the vacuum value", xz1, xz10), dec && xz1 > xz10);

    // ---- finite temperature along the clock -------------------------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for ms in [0.3, 1.0, 3.0] {
        for i in 0..10 {
            let g = 0.3 * (4.0f64 / 0.3).powf(i as f64 / 9.0);
            let t = thermal_point(g, ms);
            rows.push(vec![ms, g, g * g / (4.0 * pi), t[0], t[1], t[2], t[3], t[4], t[5], t[6], t[7]]);
        }
    }
    write_csv("thermal_history.csv",
        &[("content", "temperature at which the expectation value of the Vorton-Scalar melts, along the deflation clock (g_VS grows as exp(gsum x4/2))".to_string()),
          ("columns", "m_s (units of m); g; alpha; lambda = g^2 m_s^2/(2 m^2); C; T_high, T_largeN: closed estimates (units of m); T_half, T_zero: temperatures at which the expectation value has fallen to one half and to 0.02 of its vacuum value in the Hartree approximation (units of m); phi_before: expectation value over its vacuum value just below T_zero (a value of order one: a discontinuous transition); valid: 1 = the three preceding numbers describe a melting; 0 = the Hartree potential ceases to exist at 1.05 v_VS before the expectation value has fallen (the one-loop potential turns over too close to the vacuum value, see vacuum_history.csv), no melting temperature can be given (NaN)".to_string()),
          ("command", "deflation history".to_string())],
        &["m_s", "g", "alpha", "lambda", "C", "T_high", "T_largeN", "T_half", "T_zero", "phi_before", "valid"], &rows)?;
    // the melting temperature in units of m falls as the coupling grows (T ~ v ~ m/g), wherever it can be computed
    let mut fall = true;
    let mut n_valid = 0;
    let mut invalid_text = String::new();
    for ms in [0.3, 1.0, 3.0] {
        let sub: Vec<&Vec<f64>> = rows.iter().filter(|r| r[0] == ms && r[10] == 1.0).collect();
        n_valid += sub.len();
        for w in sub.windows(2) {
            fall &= w[1][7] < w[0][7];
        }
        if let Some(first_bad) = rows.iter().find(|r| r[0] == ms && r[10] == 0.0) {
            invalid_text.push_str(&format!(" for m_s/m = {} from g_VS = {:.2} (C = {:.0}) on;", ms, first_bad[1], first_bad[4]));
        }
    }
    let prefix_ok = [0.3, 1.0, 3.0].iter().all(|ms| {
        // the rows that cannot be computed lie at the strong-coupling end of each series
        let flags: Vec<f64> = rows.iter().filter(|r| r[0] == *ms).map(|r| r[10]).collect();
        flags.windows(2).all(|w| w[0] >= w[1])
    });
    rep.add("def-H7", "numerical", &format!("the temperature T_half at which the fermions lose half of their mass, in units of the (constant) fermion mass, falls monotonically as g_VS grows, for m_s/m = 0.3, 1 and 3 ({} of the 30 moments examined; it is of the order of the four-dimensional vacuum value m/g_VS, which falls as exp(-gsum x4/2)): matter of a fixed temperature melts at a definite x4. No melting temperature can be given{} there the Hartree potential ceases to exist at 1.05 v_VS before the expectation value has fallen, because the one-loop potential turns over too close to the vacuum value", n_valid, if invalid_text.is_empty() { " in none of the cases;".to_string() } else { invalid_text }), fall && prefix_ok && n_valid >= 20);

    // ---- the order of events ------------------------------------------------------------------
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut pair_before_instability: Vec<String> = Vec::new();
    for ms in [0.1, 0.3, 1.0, 3.0, 10.0, 30.0] {
        let ev = events(ms, 0.5, 100.0);
        let a_pair = ev.iter().find(|e| e.name.contains("bound pair")).unwrap().alpha;
        let a_inst = ev.iter().find(|e| e.name.contains("3 v_VS")).unwrap().alpha;
        if a_pair < a_inst {
            pair_before_instability.push(format!("{}", ms));
        }
        for e in &ev {
            rows.push(vec![format!("{}", ms), e.name.clone(), e.quantity.clone(), format!("{:.6}", e.critical), format!("{:.6e}", e.alpha), format!("{:.4}", (e.alpha / a_pair).ln())]);
        }
    }
    write_csv_str("events.csv",
        &[("content", "the order of events on a deflating spacetime: values of alpha_VS = g_VS^2/(4 pi) at which the thresholds of the model are passed, for several mass ratios m_s/m; matter of Fermi momentum 0.5 m; cutoff 100 m for the vacuum condensate".to_string()),
          ("columns", "m_s_over_m; event; quantity that passes a critical value; that value; alpha_VS at the event; tau_pair = ln(alpha/alpha at which a pair becomes bound) = gsum (x4 - x4 of that event): negative = earlier".to_string()),
          ("command", "deflation history".to_string())],
        &["m_s_over_m", "event", "quantity", "critical", "alpha", "tau_pair"], &rows)?;
    rep.add("def-H8", "numerical", &format!("order of events: the pair of fermions becomes bound before the one-loop potential of the Vorton-Scalar has fallen below its vacuum value at a field of 3 v_VS only for the mass ratios m_s/m = {} of the six ratios 0.1, 0.3, 1, 3, 10, 30 examined (the condition is 4 pi D_c m/m_s < 4.27, i.e. m_s/m > 4.9): for a light radial mode the deflation destabilizes the vacuum first", pair_before_instability.join(", ")), pair_before_instability == vec!["10".to_string(), "30".to_string()]);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "history_checks.txt", "history_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: checks of the history of the model along the deflation clock (couplings that grow as exp(gsum x4))", "deflation history")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\nhistory: {} of {} checks passed", ok, n);
    Ok(all)
}

/// `quench`: can the paired state follow the deflation?  Time-dependent mean-field
/// dynamics of the order parameter while the pairing strength grows as exp(gsum x4)
/// (module quench.rs).  Writes data/quench_history.csv, quench_summary.csv,
/// quench_checks.{txt,csv}.
fn cmd_quench() -> std::io::Result<bool> {
    use spacetimes_deflation::quench::{delta_eq, ramp, Shell};
    let mut rep = spacetimes::verify::Report::new();
    let sh = Shell::new(2000, 8.0);
    let (li, lf) = (0.15_f64, 0.5_f64);
    let rates = [1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1];
    let mut hist: Vec<Vec<f64>> = Vec::new();
    let mut summ: Vec<Vec<f64>> = Vec::new();
    let mut len_dev: f64 = 0.0;
    let mut mins: Vec<f64> = Vec::new();
    let mut catch_a: Vec<f64> = Vec::new();
    let mut nex: Vec<f64> = Vec::new();
    for gsum in rates {
        let (h, dev) = ramp(&sh, li, lf, gsum, 0.025, 800);
        len_dev = len_dev.max(dev);
        let a = |s: &spacetimes_deflation::quench::Sample| gsum / (s.lambda * s.delta_eq);
        for s in &h {
            hist.push(vec![gsum, s.tau, s.lambda, s.delta, s.delta_eq, s.delta / s.delta_eq, s.n_ex, a(s)]);
        }
        let ratio = |s: &spacetimes_deflation::quench::Sample| s.delta / s.delta_eq;
        let imin = (0..h.len()).min_by(|i, j| ratio(&h[*i]).partial_cmp(&ratio(&h[*j])).unwrap()).unwrap();
        let catch = h.iter().skip(imin).find(|s| ratio(s) > 0.9);
        let (ctau, ca) = match catch {
            Some(s) => (s.tau, a(s)),
            None => (f64::NAN, f64::NAN),
        };
        // mean of the ratio over the last tenth of the run (it oscillates)
        let tail = &h[h.len() - h.len() / 10..];
        let fin = tail.iter().map(|s| ratio(s)).sum::<f64>() / tail.len() as f64;
        let last = h.last().unwrap();
        summ.push(vec![gsum, a(&h[0]), a(last), ratio(&h[imin]), h[imin].tau, ctau, ca, fin, last.n_ex]);
        mins.push(ratio(&h[imin]));
        catch_a.push(ca);
        nex.push(last.n_ex);
    }
    write_csv("quench_history.csv",
        &[("content", "the order parameter of the paired state while the pairing strength grows as lambda = 0.15 exp(tau_d), tau_d = gsum (x4 - x4_i), up to lambda = 0.5; time-dependent mean-field dynamics of the shell model (quench.rs), started in equilibrium".to_string()),
          ("units", "energies in units of the half-width E_sh of the shell; gsum in units of E_sh".to_string()),
          ("columns", "gsum; tau_d; lambda; gap_abs: |Delta| of the evolving state; gap_eq: equilibrium gap for the coupling of the moment, 1/sinh(1/lambda); ratio = gap_abs/gap_eq; n_ex: density of broken pairs; Xi_d = gsum/(lambda gap_eq): the parameter that decides whether the state can follow".to_string()),
          ("command", "deflation quench".to_string())],
        &["gsum", "tau_d", "lambda", "gap_abs", "gap_eq", "ratio", "n_ex", "Xi_d"], &hist)?;
    write_csv("quench_summary.csv",
        &[("content", "summary of data/quench_history.csv, one row per rate of deflation".to_string()),
          ("columns", "gsum (units of E_sh); Xi_d_start, Xi_d_end: Xi_d = gsum/(lambda gap_eq) at the start and at the end; ratio_min: smallest gap_abs/gap_eq during the run; tau_d_min: clock reading there; tau_d_catch, Xi_d_catch: clock reading and Xi_d at which gap_abs/gap_eq first exceeds 0.9 after the minimum (nan: never); ratio_final: mean of gap_abs/gap_eq over the last tenth of the run; n_ex_final: density of broken pairs at the end".to_string()),
          ("command", "deflation quench".to_string())],
        &["gsum", "Xi_d_start", "Xi_d_end", "ratio_min", "tau_d_min", "tau_d_catch", "Xi_d_catch", "ratio_final", "n_ex_final"], &summ)?;

    // checks
    rep.add("def-Q1", "numerical", &format!("the equations of motion conserve |2F|^2 + (1 - 2n)^2 = 1 for every pair: largest deviation at the end of the seven runs {:.1e}", len_dev), len_dev < 1e-8);
    // convergence: the run with gsum = 1e-3 on a grid twice as fine in energy and in time
    let sh2 = Shell::new(4000, 8.0);
    let (h2, _) = ramp(&sh2, li, lf, 1e-3, 0.0125, 800);
    let min2 = h2.iter().map(|s| s.delta / s.delta_eq).fold(f64::INFINITY, f64::min);
    let nex2 = h2.last().unwrap().n_ex;
    let (dmin, dnex) = ((min2 - mins[2]).abs(), (nex2 / nex[2] - 1.0).abs());
    rep.add("def-Q2", "numerical", &format!("convergence: with twice as many energies (4000) and half the time step the run with gsum = 1e-3 E_sh gives the smallest ratio {:.5} (against {:.5}) and the final density of broken pairs {:.4e} (against {:.4e})", min2, mins[2], nex2, nex[2]), dmin < 2e-3 && dnex < 0.03);
    let mono = mins.windows(2).all(|w| w[1] < w[0]);
    rep.add("def-Q3", "numerical", &format!("the faster the deflation, the further the order parameter falls behind: the smallest value of |Delta|/Delta_eq is {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3}, {:.3} for gsum/E_sh = 1e-4, 3e-4, 1e-3, 3e-3, 1e-2, 3e-2, 1e-1 (Xi_d at the start: 0.26 to 262)", mins[0], mins[1], mins[2], mins[3], mins[4], mins[5], mins[6]), mono && mins[0] > 0.9 && mins[6] < 0.2);
    let ca: Vec<f64> = catch_a.iter().cloned().filter(|x| x.is_finite()).collect();
    let (ca_lo, ca_hi) = (ca.iter().cloned().fold(f64::INFINITY, f64::min), ca.iter().cloned().fold(0.0, f64::max));
    rep.add("def-Q4", "numerical", &format!("the order parameter catches up with the equilibrium gap of the moment (to within 10 per cent) when Xi_d = gsum/(lambda Delta_eq) has fallen to a value between {:.2} and {:.2} (six runs; in the fastest run, gsum = 0.1 E_sh, it does not catch up before lambda = 0.5): the paired state follows the deflation if and only if gsum is small compared with lambda times the gap", ca_lo, ca_hi), ca.len() == 6 && ca_lo > 0.1 && ca_hi < 1.0);
    let nmono = nex.windows(2).take(5).all(|w| w[1] > w[0]);
    rep.add("def-Q5", "numerical", &format!("the lag leaves broken pairs behind: their density at lambda = 0.5 is {:.1e}, {:.1e}, {:.1e}, {:.1e}, {:.1e}, {:.1e} (units of E_sh times the density of states) for gsum/E_sh = 1e-4 to 3e-2, growing with the rate; for the equilibrium state it is zero", nex[0], nex[1], nex[2], nex[3], nex[4], nex[5]), nmono && nex[0] < 1e-4);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "quench_checks.txt", "quench_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: checks of the time-dependent dynamics of the paired state under a coupling that grows as exp(gsum x4)", "deflation quench")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    let _ = delta_eq(0.3);
    println!("\nquench: {} of {} checks passed", ok, n);
    Ok(all)
}

/// `cosmology`: the expansion of ordinary space on a deflating spacetime, the bounds on
/// the rate of deflation, and the Hubble constant (module cosmo_defl.rs).  Writes
/// data/hubble_deflation.csv, deflation_bounds.csv, hubble_mass_loss.csv,
/// cosmology_checks.{txt,csv}.
fn cmd_cosmology() -> std::io::Result<bool> {
    use spacetimes_deflation::cosmo_defl::{main_project_reference, rate_bounds, solve_h, theta_star, Model, HUBBLE_TIME_100_YR};
    let mut rep = spacetimes::verify::Report::new();
    let h0 = 0.6736_f64;
    let kms = 100.0; // H_100 in km/s/Mpc
    // the locally measured Hubble constant with which the main project compares (its manuscripts,
    // Sec. XI: 73.50 +- 0.81 km/s/Mpc, the consensus value of the local distance network)
    let (h_local, h_local_err) = (73.50_f64, 0.81_f64);
    let gap_local = h_local - kms * h0; // what separates the model without deflation from it
    let (z_star, theta_main, rs_main) = main_project_reference();
    let none = |_s: f64| 1.0;
    let m0 = Model::fiducial(0.0, h0);
    let d0 = theta_star(&m0, z_star, &none);
    rep.add("def-C1", "numerical", &format!("without deflation the background integrals of this module agree with the main project (its recombination calculation gives z_* = {:.2}): 100 theta_* = {:.5} here against {:.5}, sound horizon {:.3} Mpc against {:.3} Mpc; age {:.3} Gyr", z_star, d0.theta100, theta_main, d0.rs * 2997.92458, rs_main, d0.age * HUBBLE_TIME_100_YR / 1e9),
        (d0.theta100 / theta_main - 1.0).abs() < 2e-4 && (d0.rs * 2997.92458 / rs_main - 1.0).abs() < 2e-4);
    let theta = d0.theta100;

    // ---- the bounds on the rate ------------------------------------------------------
    // lunar laser ranging, nucleosynthesis (Newton's constant; Fermi constant under the assumption HV): the
    // measured values and the derivation are documented with `rate_bounds` in cosmo_defl.rs, which the
    // command `matter` uses as well
    let rb = rate_bounds(h0);
    let (llr, gam_llr, gam_bbn, gam_gfermi_20, gam_gfermi_10) = (rb.llr_per_yr, rb.llr, rb.bbn, rb.gfermi_20, rb.gfermi_10);
    // atomic clocks: (d alpha/dt)/alpha = (1.0 +- 1.1) e-18 per year; two-sigma upper value 3.2e-18 per year.  This bounds the
    // rate only if the electromagnetic field, too, propagates along x5, x6, x7 (then alpha ~ exp(gsum x4)); the ratio of the
    // proton mass to the electron mass, also measured by the clocks, does not change in the model at all.
    let clocks = (1.0 + 2.0 * 1.1) * 1e-18;
    let gam_clock = clocks * HUBBLE_TIME_100_YR;
    // the author's hypothesis H2 (the superfluid state is the Higgs field): the electroweak scale v_EW is then the scale of the
    // pair condensate in the vacuum, a function of alpha_VS, which grows as exp(gsum x4).  With n_EW = d ln v_EW/d ln alpha_VS the
    // masses of the ordinary fermions change at the rate n_EW gsum, while the proton mass (mostly due to the strong interaction)
    // does not: |d ln mu_pe/dt| = n_EW gsum.  Atomic clocks: (d mu_pe/dt)/mu_pe = (-8 +- 36) e-18 per year; largest absolute
    // value at two sigma: 8.0e-17 per year.  The bound is given for n_EW = 1 and scales as 1/n_EW.
    let clocks_mu = (8.0 + 2.0 * 36.0) * 1e-18;
    let gam_mu = clocks_mu * HUBBLE_TIME_100_YR;
    // n_EW in the scale-invariant ladder equation (the test case of the main project, S4 Sec. 7.2): with
    // rho^2 = (alpha_VS/alpha_c - 1)/4 the infrared scale of the condensate obeys rho ln(cutoff/scale) = atan(1/(2 rho)).
    let ln_ratio = |x: f64| { let rho = (0.25 * (x - 1.0)).sqrt(); (1.0 / (2.0 * rho)).atan() / rho };
    let n_ew = |x: f64| { let rho = (0.25 * (x - 1.0)).sqrt(); (1.0 + 4.0 * rho * rho) * (1.0 / (2.0 * rho)).atan() / (8.0 * rho.powi(3)) + 1.0 / (4.0 * rho * rho) };
    let mut ew_rows: Vec<Vec<f64>> = Vec::new();
    let mut ew_dev = 0.0_f64;
    for x in [1.05, 1.1, 1.25, 1.5, 2.0, 3.0, 5.0, 10.0] {
        // finite-difference derivative of -ln(cutoff/scale) with respect to ln(alpha_VS)
        let e = 1e-5;
        let fd = -(ln_ratio(x * (1.0 + e)) - ln_ratio(x * (1.0 - e))) / ((1.0 + e).ln() - (1.0 - e).ln());
        ew_dev = ew_dev.max((fd / n_ew(x) - 1.0).abs());
        ew_rows.push(vec![x, ln_ratio(x), n_ew(x), clocks_mu / n_ew(x), gam_mu / h0 / n_ew(x)]);
    }
    // the ratio alpha_VS/alpha_c at which n_EW = 1 (bisection)
    let (mut lo1, mut hi1) = (1.5_f64, 10.0_f64);
    for _ in 0..60 { let mid = 0.5 * (lo1 + hi1); if n_ew(mid) > 1.0 { lo1 = mid } else { hi1 = mid } }
    let x_one = 0.5 * (lo1 + hi1);
    write_csv("higgs_scale_exponent.csv",
        &[("content", "the author's hypothesis H2 on a deflating spacetime: if the Higgs field is the pair condensate, the electroweak scale v_EW is the scale of that condensate and depends on alpha_VS, which grows as exp(gsum x4). Table: n_EW = d ln v_EW/d ln alpha_VS in the scale-invariant ladder equation (massless fermions and bosons; the test case of the supplement S4 of the main project, Sec. 7.2), where rho_c^2 = (alpha_VS/alpha_c - 1)/4 and rho_c ln(cutoff/scale) = atan(1/(2 rho_c))".to_string()),
          ("columns", "alpha_over_alpha_c; ln_cutoff_over_scale; n_EW; gsum_max_per_yr and gsum_max_over_H0: the bound on the rate of deflation from the constancy of the ratio mu_pe of the proton mass to the electron mass, 8.0e-17 per year (two sigma), divided by n_EW".to_string()),
          ("command", "deflation cosmology".to_string())],
        &["alpha_over_alpha_c", "ln_cutoff_over_scale", "n_EW", "gsum_max_per_yr", "gsum_max_over_H0"], &ew_rows)?;

    // ---- the Hubble constant as a function of the rate -----------------------------------
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut ratios: Vec<f64> = vec![0.0, 1e-3, 3e-3, 1e-2, 3e-2, 0.1, 0.3];
    ratios.push(gam_llr / h0);
    ratios.push(gam_bbn / h0);
    ratios.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // Two calculations.  (a) The integrals of the module cosmo_defl with the redshift of last scattering held at its value
    // without deflation and the helium fraction unchanged: fast, and the only one available when the dark matter loses rest
    // mass (below).  (b) The full calculation of recombination on the deflating background, with the helium fraction that
    // follows from Newton's constant at nucleosynthesis (modules eb_cosmo and spectra_defl: the thermal history of the main
    // project with Newton's constant of the moment).  (b) is the result; (a) is tabulated beside it.
    let fid = spacetimes::cosmo::Params::fiducial();
    let none_defl = spacetimes_deflation::eb_cosmo::Deflation::none();
    let theta_full = spacetimes_deflation::spectra_defl::theta_star_100(&fid, &none_defl);
    let mut hs: Vec<(f64, f64, f64)> = Vec::new();
    for r in &ratios {
        let gam = r * h0;
        let base = Model::fiducial(gam, h0);
        let h_fix = solve_h(&base, z_star, theta, &none);
        let dfl = spacetimes_deflation::eb_cosmo::Deflation::equal_rates(gam);
        let h = if gam == 0.0 { h0 } else { spacetimes_deflation::spectra_defl::h_at_fixed_theta(&fid, &dfl, theta_full) };
        let bg = spacetimes_deflation::eb_cosmo::Background::new(&spacetimes::cosmo::Params { h, ..fid }, &dfl);
        let th = spacetimes_deflation::eb_cosmo::Thermo::new(&bg);
        rows.push(vec![*r, gam / HUBBLE_TIME_100_YR, h, kms * (h - h0), bg.s_early().exp(), bg.s_of_a(1.0 / (1.0 + th.z_star)).exp(), th.rs_star, bg.age() * HUBBLE_TIME_100_YR / 1e9, th.z_star, th.y_he, h_fix, kms * (h_fix - h0)]);
        hs.push((*r, h, h_fix));
    }
    write_csv("hubble_deflation.csv",
        &[("content", "the Hubble constant inferred from the angular scale of the sound horizon when x5, x6, x7 deflate at a constant total rate gsum throughout cosmic history (assumptions A1, A2 of cosmo_defl.rs; three equal rates); densities omega_b, omega_c and 100 theta_* held at the values of the model without deflation; recombination and the helium fraction computed on the deflating background (the thermal history of the main project with Newton's constant of the moment)".to_string()),
          ("columns", "gsum_over_H0: gsum divided by the Hubble constant of the model without deflation (67.36 km/s/Mpc); gsum_per_yr; h; dH0: change of the inferred Hubble constant in km/s/Mpc; GN_early: Newton's constant in the early universe over its value today, exp(-gsum t_0); GN_star: the same at last scattering; rs_Mpc: sound horizon at last scattering; age_Gyr; z_star: redshift of last scattering; Y_He: helium mass fraction; h_fixed_zstar, dH0_fixed_zstar: the Hubble constant and its change if the redshift of last scattering and the helium fraction are held at their values without deflation (the approximation of the module cosmo_defl)".to_string()),
          ("bounds", format!("lunar laser ranging (two sigma): gsum < {:.3e} per year = {:.3e} H_0; nucleosynthesis (G_N(nucleosynthesis)/G_N(today) > 0.94, two sigma): gsum < {:.3e} per year = {:.4} H_0", llr, gam_llr / h0, gam_bbn / HUBBLE_TIME_100_YR, gam_bbn / h0)),
          ("command", "deflation cosmology".to_string())],
        &["gsum_over_H0", "gsum_per_yr", "h", "dH0", "GN_early", "GN_star", "rs_Mpc", "age_Gyr", "z_star", "Y_He", "h_fixed_zstar", "dH0_fixed_zstar"], &rows)?;
    let mono = hs.windows(2).all(|w| w[1].1 < w[0].1 && w[1].2 < w[0].2);
    let h_at = |r: f64| hs.iter().find(|x| (x.0 - r).abs() < 1e-12).unwrap().1;
    let h_fix_at = |r: f64| hs.iter().find(|x| (x.0 - r).abs() < 1e-12).unwrap().2;
    let slope = (h_at(1e-3) - h0) / h0 / 1e-3;
    let slope_fix = (h_fix_at(1e-3) - h0) / h0 / 1e-3;
    // by how much the approximation (a) underestimates the fall
    let under: Vec<f64> = hs.iter().filter(|x| x.0 > 0.0).map(|x| 1.0 - (h0 - x.2) / (h0 - x.1)).collect();
    let (under_lo, under_hi) = (under.iter().cloned().fold(f64::INFINITY, f64::min), under.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let z_at = |r: f64| rows.iter().find(|x| (x[0] - r).abs() < 1e-12).unwrap()[8];
    rep.add("def-C2", "numerical", &format!("with deflation at a constant rate, gravity was weaker in the past, the early universe expanded more slowly at a given density, and the sound horizon is larger; holding 100 theta_*, omega_b and omega_c fixed, the inferred Hubble constant FALLS monotonically with the rate: d ln H_0/d(gsum/H_0) = {:.3} at small rates; H_0 = {:.2}, {:.2}, {:.2} km/s/Mpc for gsum = 0.01, 0.03, 0.1 H_0 (67.36 without deflation). Deflation moves the Hubble constant away from the locally measured value. These numbers come from the full calculation of recombination on the deflating background (last scattering moves from z_* = {:.2} to {:.2} at gsum = 0.1 H_0, and the helium fraction falls). If z_* and the helium fraction are held at their values without deflation, the slope is {:.3} and the values are {:.2}, {:.2}, {:.2} km/s/Mpc: that approximation underestimates the fall by {:.1} to {:.1} per cent", slope, kms * h_at(1e-2), kms * h_at(3e-2), kms * h_at(0.1), z_at(0.0), z_at(0.1), slope_fix, kms * h_fix_at(1e-2), kms * h_fix_at(3e-2), kms * h_fix_at(0.1), 100.0 * under_lo, 100.0 * under_hi),
        mono && slope < 0.0 && slope < slope_fix && under_lo > 0.0 && under_hi < 0.12);
    let (h_llr, h_bbn) = (h_at(gam_llr / h0), h_at(gam_bbn / h0));
    rep.add("def-C3", "numerical", &format!("bounds on the present rate of deflation if gravity is eight-dimensional (G_N grows at the rate gsum): lunar laser ranging, (dG_N/dt)/G_N = (7.1 +- 7.6) 10^-14 per year, gives gsum < {:.2e} per year = {:.2e} H_0 at two sigma; nucleosynthesis, G_N(nucleosynthesis)/G_N(today) > 0.94 at two sigma, gives gsum < {:.2e} per year = {:.3} H_0 for a rate that was constant since then. At these rates the inferred Hubble constant changes by {:.3} and {:.2} km/s/Mpc, in the direction away from the local measurement, {:.2} +- {:.2} km/s/Mpc. Over the age t_0 of the model without deflation (H_0 t_0 = {:.4}) the deflation clock has advanced by gsum t_0 = {:.1e} at the first of these rates", llr, gam_llr / h0, gam_bbn / HUBBLE_TIME_100_YR, gam_bbn / h0, kms * (h_llr - h0), kms * (h_bbn - h0), h_local, h_local_err, rb.age * h0, gam_llr * rb.age),
        h_llr < h0 && h_bbn < h_llr && kms * (h0 - h_llr) < h_local_err);

    let rows: Vec<Vec<String>> = vec![
        vec!["lunar laser ranging".into(), "(dG_N/dt)/G_N = (7.1 +- 7.6) e-14 per year (68%)".into(), "G_N = G_8/(volume of the extra coordinates): (dG_N/dt)/G_N = gsum".into(), format!("{:.3e}", llr), format!("{:.3e}", gam_llr / h0), "gravity is eight-dimensional (A2)".into()],
        vec!["primordial nucleosynthesis".into(), "G_N(nucleosynthesis)/G_N(today) = 0.99 +0.06 -0.05 (two sigma)".into(), "G_N(nucleosynthesis)/G_N(today) = exp(-gsum t_0) > 0.94".into(), format!("{:.3e}", gam_bbn / HUBBLE_TIME_100_YR), format!("{:.4}", gam_bbn / h0), "A2, and a rate that is constant from nucleosynthesis until today".into()],
        vec!["primordial nucleosynthesis, weak interaction".into(), "G_Fermi at t = 1 s within 20 per cent (10 per cent) of today's value, fermion masses held fixed".into(), "G_Fermi = 1/(sqrt(2) v^2) with the four-dimensional vacuum value v ~ exp(-gsum x4/2): G_Fermi(1 s)/G_Fermi(today) = exp(-gsum t_0) > 0.8 (0.9); all masses constant".into(), format!("{:.3e} ({:.3e})", gam_gfermi_20 / HUBBLE_TIME_100_YR, gam_gfermi_10 / HUBBLE_TIME_100_YR), format!("{:.3} ({:.3})", gam_gfermi_20 / h0, gam_gfermi_10 / h0), "the assumption HV: the four-dimensional vacuum value of the Vorton-Scalar sets the electroweak scale, as that of the Higgs field does (not the author's hypothesis H2, in which the Higgs field is the pair condensate); a rate that is constant from nucleosynthesis until today; no assumption about gravity".into()],
        vec!["atomic clocks, mass ratio".into(), "(d mu_pe/dt)/mu_pe = (-8 +- 36) e-18 per year (68%), mu_pe = ratio of the proton mass to the electron mass".into(), "under H2 the electroweak scale is the scale of the pair condensate, v_EW ~ exp(n_EW gsum x4) with n_EW = d ln v_EW/d ln alpha_VS; the electron mass follows v_EW, the proton mass does not: |d ln mu_pe/dt| = n_EW gsum".into(), format!("{:.3e}", clocks_mu), format!("{:.3e}", gam_mu / h0), "the author's hypothesis H2 (the superfluid state is the Higgs field), for n_EW = 1; the bound scales as 1/n_EW (data/higgs_scale_exponent.csv)".into()],
        vec!["atomic clocks".into(), "(d alpha/dt)/alpha = (1.0 +- 1.1) e-18 per year (68%), alpha = fine-structure constant".into(), "alpha ~ exp(gsum x4) if the electromagnetic field propagates along x5, x6, x7".into(), format!("{:.3e}", clocks), format!("{:.3e}", gam_clock / h0), "an assumption OUTSIDE the model, which contains no electromagnetic field: stated to show how strongly such an extension would be constrained".into()],
    ];
    rep.add("def-C5", "numerical", &format!("bounds that do not rest on gravity: (i) under the assumption HV (the vacuum value of the Vorton-Scalar sets the electroweak scale) the Fermi constant grows at the rate gsum while every mass stays constant; nucleosynthesis with fixed masses allows G_Fermi at t = 1 s to differ by 10 to 20 per cent from today's value (Scherrer and Spergel 1993), which gives gsum < {:.3} to {:.3} H_0, weaker than the bound from Newton's constant; (ii) in the model as it stands (without H2 and without HV) no mass of ordinary matter depends on the deflation, and the measured constancy of the ratio of the proton mass to the electron mass does not bound the rate; (iii) the fine-structure constant would bound it, gsum < {:.1e} per year = {:.1e} H_0, only if the electromagnetic field propagated along the deflating coordinates, which is not part of the model", gam_gfermi_10 / h0, gam_gfermi_20 / h0, clocks, gam_clock / h0),
        gam_gfermi_10 < gam_gfermi_20 && gam_gfermi_10 > gam_bbn && gam_clock < gam_llr);
    rep.add("def-C6", "numerical", &format!("the author's hypothesis H2 (the superfluid state is the Higgs field) on a deflating spacetime: the electroweak scale is then the scale of the pair condensate, which depends on alpha_VS, and alpha_VS grows as exp(gsum x4). With n_EW = d ln v_EW/d ln alpha_VS the electron mass changes at the rate n_EW gsum and the proton mass does not; atomic clocks (ratio of the two masses constant to 8.0e-17 per year at two sigma) then give gsum < {:.1e} per year / n_EW = {:.2e} H_0 / n_EW, a bound {:.0} times stronger than that of lunar laser ranging for n_EW = 1, and one that needs no assumption about gravity; over the age of the universe the deflation clock can then have advanced by at most {:.1e}/n_EW. In the scale-invariant ladder equation of the main project n_EW = (1 + 4 rho_c^2) atan(1/(2 rho_c))/(8 rho_c^3) + 1/(4 rho_c^2) with rho_c^2 = (alpha_VS/alpha_c - 1)/4 (agreement with the numerical derivative: {:.1e}): n_EW = {:.1}, {:.2}, {:.2}, {:.2} for alpha_VS/alpha_c = 1.25, 2, 3, 5; it diverges at the critical coupling and exceeds 1 for alpha_VS < {:.2} alpha_c. For the model itself (massive fermions, massive radial mode) n_EW has not been computed", clocks_mu, gam_mu / h0, gam_llr / gam_mu, gam_mu * rb.age, ew_dev, n_ew(1.25), n_ew(2.0), n_ew(3.0), n_ew(5.0), x_one),
        ew_dev < 1e-6 && gam_clock < gam_mu && gam_mu < gam_llr && n_ew(2.0) > 1.0 && n_ew(5.0) < 1.0);
    write_csv_str("deflation_bounds.csv",
        &[("content", "upper bounds (two sigma) on the present total rate of deflation gsum = g55 + g66 + g77, each under the stated assumption".to_string()),
          ("columns", "observation; measured value; relation to gsum; bound on gsum per year; bound in units of H_0 = 67.36 km/s/Mpc; assumption".to_string()),
          ("sources", "Hofmann and Mueller, Class. Quantum Grav. 35, 035015 (2018); Alvey, Sabti, Escudero and Fairbairn, Eur. Phys. J. C 80, 148 (2020); Scherrer and Spergel, Phys. Rev. D 47, 4774 (1993); Lange et al., Phys. Rev. Lett. 126, 011102 (2021)".to_string()),
          ("command", "deflation cosmology".to_string())],
        &["observation", "measured", "relation", "gsum_max_per_yr", "gsum_max_over_H0", "assumption"], &rows)?;

    // ---- dark matter of bound pairs that lose rest mass as the coupling grows --------------------
    // mass of a pair: 2 m - E_b, E_b = eps(D) m_s^2/m;  mass(s) = [1 - (m_s/m)^2 eps(D_0 e^s)/2] / [1 - (m_s/m)^2 eps(D_0)/2]
    let table: Vec<(f64, f64)> = (0..=400).map(|i| {
        let d = 1.0 * (30.0f64).powf(i as f64 / 400.0);
        (d.ln(), spacetimes::dimer::binding(d).unwrap_or(0.0))
    }).collect();
    let eps_of = |d: f64| -> f64 {
        let l = d.max(1.0).min(30.0).ln();
        let x = (l - table[0].0) / (table[400].0 - table[0].0) * 400.0;
        let i = (x.floor() as usize).min(399);
        let f = x - i as f64;
        table[i].1 * (1.0 - f) + table[i + 1].1 * f
    };
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut gain_max: f64 = 0.0;
    // examples with alpha_VS = D m_s/m <= 1 today, so that the binding energy (at most alpha^2 m/4) stays well below the
    // rest mass 2m and the non-relativistic formula for it can be used.
    // (These runs use the approximation (a): last scattering at the redshift of the model without deflation.  The program
    // for the full thermal history has no dark matter of varying rest mass; by def-C2 the approximation underestimates a
    // fall of the Hubble constant by a few per cent of itself.)
    let mut bind_frac_max: f64 = 0.0;
    for (d_today, ms) in [(2.5_f64, 0.2_f64), (5.0, 0.2), (4.0, 0.1), (10.0, 0.1)] {
        bind_frac_max = bind_frac_max.max(0.5 * ms * ms * eps_of(d_today));
        for (r, gravity) in [(gam_llr / h0, true), (gam_bbn / h0, true), (0.03, false), (0.1, false), (0.3, false), (1.0, false)] {
            let mass = |s: f64| (1.0 - 0.5 * ms * ms * eps_of(d_today * s.exp())) / (1.0 - 0.5 * ms * ms * eps_of(d_today));
            let base = Model { gravity, ..Model::fiducial(r * h0, h0) };
            // omega_c is the density inferred at early times; today's is omega_c * mass(0)/mass(early) -> normalize so that the EARLY density is fixed:
            // rho_c(a) = omega_c_early a^-3 mass(s)/mass(s_early); with mass(0) = 1 this is omega_c_early * mass(s)/mass_early
            let s_early = theta_star(&base, z_star, &mass).g_early.ln();
            let m_early = mass(s_early);
            let mass_n = |s: f64| mass(s) / m_early;
            let h = solve_h(&base, z_star, theta, &mass_n);
            rows.push(vec![d_today, ms, d_today * ms, r, if gravity { 1.0 } else { 0.0 }, 1.0 / m_early, h, kms * (h - h0)]);
            if !gravity {
                gain_max = gain_max.max(kms * (h - h0));
            }
        }
    }
    write_csv("hubble_mass_loss.csv",
        &[("content", "dark matter made of bound pairs on a deflating spacetime: the pair loses rest mass as its binding energy grows with the coupling; inferred Hubble constant with the dark-matter density at early times, omega_b and 100 theta_* held fixed".to_string()),
          ("columns", "D_today = alpha_VS m/m_s today; m_s_over_m; alpha_today; gsum_over_H0; gravity: 1 = gravity feels the deflation (assumptions A1, A2), 0 = constant G_N, the deflation acts on the dark sector only; mass_ratio: rest mass of a pair today over its mass in the early universe; h; dH0 (km/s/Mpc)".to_string()),
          ("note", "rows with gravity = 1 use the largest rates that lunar laser ranging and nucleosynthesis allow".to_string()),
          ("command", "deflation cosmology".to_string())],
        &["D_today", "m_s_over_m", "alpha_today", "gsum_over_H0", "gravity", "mass_ratio", "h", "dH0"], &rows)?;
    let grav: Vec<f64> = rows.iter().filter(|r| r[4] == 1.0).map(|r| r[7]).collect();
    let (grav_lo, grav_hi) = (grav.iter().cloned().fold(f64::INFINITY, f64::min), grav.iter().cloned().fold(f64::NEG_INFINITY, f64::max));
    let loss_max = rows.iter().filter(|r| r[4] == 0.0).map(|r| 1.0 - r[5]).fold(0.0, f64::max);
    rep.add("def-C4", "numerical", &format!("dark matter of bound pairs loses rest mass while the coupling grows, because the binding energy increases (four examples with alpha_VS = 0.4 to 1 today; binding energy at most {:.1} per cent of the rest mass). If gravity feels the deflation, the inferred Hubble constant changes by between {:.2} and {:.2} km/s/Mpc at the rates that lunar laser ranging and nucleosynthesis allow: it falls, the weaker gravity of the past outweighing the loss of mass. (The runs with a varying rest mass hold the redshift of last scattering and the helium fraction at their values without deflation, which by def-C2 underestimates a fall by about six per cent of itself.) If Newton's constant is held fixed and the deflation acts on the dark sector alone, rates between 0.03 and 1 H_0 change the inferred Hubble constant by at most +{:.2} km/s/Mpc, for a loss of up to {:.1} per cent of the rest mass (data/hubble_mass_loss.csv). The loss of rest mass is the only effect of the deflation that raises the inferred Hubble constant; in these examples it supplies less than a tenth of the {:.1} km/s/Mpc that separate {:.2} from the local value {:.2}", 100.0 * bind_frac_max, grav_lo, grav_hi, gain_max, 100.0 * loss_max, gap_local, kms * h0, h_local),
        grav_hi < 0.0 && gain_max > 0.0 && gain_max < 0.1 * gap_local && bind_frac_max < 0.2);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "cosmology_checks.txt", "cosmology_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: checks of the cosmological calculations on the deflating spacetime", "deflation cosmology")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\ncosmology: {} of {} checks passed", ok, n);
    Ok(all)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let result = match cmd {
        "matrices" => cmd_matrices(),
        "modes" => cmd_modes(),
        "history" => cmd_history(),
        "quench" => cmd_quench(),
        "cosmology" => cmd_cosmology(),
        "symmetry" => cmd_symmetry::run(),
        "channels" => cmd_channels::run(&main_data_dir()),
        "matter" => cmd_matter::run(&main_data_dir()),
        "paired" => cmd_paired::run(),
        "background" => cmd_spectra::run_background(),
        "spectrum" => cmd_spectra::run_spectrum(&main_data_dir()),
        "forecast" => cmd_spectra::run_forecast(&main_data_dir()),
        "families" => cmd_spectra::run_families(&main_data_dir()),
        "superfluid" => cmd_spectra::run_superfluid(&main_data_dir()),
        _ => {
            println!("deflation: PROJECT spacetimes, Revision_Deflation (x5, x6, x7 deflate exponentially)\n");
            println!("commands (run from the folder Revision_Deflation):");
            println!("  matrices   checks of the flat Dirac matrices (again), of the geometry of the deflating spacetimes,");
            println!("             of the coordinate matrices, the spin connection and the Dirac operators");
            println!("  modes      solutions of the Dirac equations mode by mode: pair creation by the deflation (7+1),");
            println!("             bounded solutions and their extinction (4+4)");
            println!("  history    the couplings and every threshold of the model along the deflation clock:");
            println!("             gap, bound pair, normal matter, vacuum potential, melting, order of events");
            println!("  quench     time-dependent dynamics of the paired state while the coupling grows: can it follow?");
            println!("  cosmology  expansion of ordinary space on the deflating spacetime, bounds on the rate, the Hubble constant");
            println!("  channels   exchange of the Vorton-Scalar: amplitudes in the pair channels (compared with the main project),");
            println!("             the field of a static source, modes of the Vorton-Scalar, creation of quanta, bounded solutions");
            println!("  matter     dark matter on the deflating spacetime: degeneracy, self-interaction, the resonance at the moment");
            println!("             of binding, growth of structure, drift of the quantities, quanta created by the deflation");
            println!("  paired     the results of the effective action of Cornwall, Jackiw and Tomboulis along the clock: potential");
            println!("             of the paired state, thermodynamics, Ginzburg-Landau coefficients, order-parameter space, matter");
            println!("  symmetry   isometries of the deflating spacetimes, internal symmetry of the fields that do not depend on");
            println!("             the extra coordinates, zero modes on a vortex line, exponents of the couplings");
            println!("  background thermal history (recombination, sound horizons, helium) on the deflating spacetime, for the three");
            println!("             models of the command `background` of the main project");
            println!("  spectrum   spectra of the microwave background and of matter on the deflating spacetime (the Einstein-Boltzmann");
            println!("             program of the main project with Newton's constant of the moment), for two closures of the");
            println!("             perturbation equations; what depends on the closure; convergence; growth of structure");
            println!("  forecast   mock data of the microwave background: how well they bound the rate of deflation (Fisher matrix,");
            println!("             fits at fixed rate), and what happens to the Hubble constant");
            println!("  families   the forecast of the main project for its dark-radiation families, repeated on the deflating");
            println!("             spacetime at the rate that lunar laser ranging allows (about one hour; `families quick`: a test)");
            println!("  superfluid superfluid dark matter (first sound) on the deflating spacetime: suppression of structure, limits");
            Ok(true)
        }
    };
    match result {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("deflation {}: at least one check FAILED", cmd);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("deflation {}: {}", cmd, e);
            std::process::exit(2);
        }
    }
}
