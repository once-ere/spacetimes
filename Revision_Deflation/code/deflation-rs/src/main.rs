//! Command-line program of PROJECT spacetimes, Revision_Deflation.
//!
//! Run it from the folder `Revision_Deflation` (the data files are written to
//! `data/` below the current directory; the notebook matrices of the main project
//! are read from `../data/`):
//!
//!     cargo run --release --manifest-path code/deflation-rs/Cargo.toml -- <command>
//!
//! `deflation help` lists the commands.

use spacetimes_deflation::io::{write_csv, write_report};
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
            &[("content", format!("a geodesic of the deflating {} spacetime with momentum along the deflating coordinate x5 (check {}): g55 = 1, const5 = 0.8, M = 1, P_1 = 0.5, P_5 = 0.3, start at x4 = -2", sig.name(), if sig == Sig::S1671 { "71d-G8" } else { "44d-G9" })),
              ("columns", "lam: affine parameter (proper time for M = 1); x4; u4 = dx4/dlam".to_string()),
              ("command", "deflation matrices".to_string())],
            &["lam", "x4", "u4"], &rows)?;
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
          ("columns", "spacetime: 1644 = 4+4, 1671 = 7+1; g55, g66, g77: rates; Ric44, Ric55, Ric66, Ric77: mixed Ricci tensor R^mu_mu; R: curvature scalar; G00 (= G11 = G22 = G33), G44, G55, G66, G77: mixed Einstein tensor G^mu_mu".to_string()),
          ("command", "deflation matrices".to_string())],
        &["spacetime", "g55", "g66", "g77", "Ric44", "Ric55", "Ric66", "Ric77", "R", "G00", "G44", "G55", "G66", "G77"], &rows)?;
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

    // ---- 7+1, one direction: scan of eps/g55 -----------------------------------------
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
        &[("content", "7+1 deflating spacetime: number of particles created by the deflation in a mode with one non-zero wavenumber along a deflating coordinate (kappa_5 = 1, g55 = 1, const5 = 1, p = 0, m = eps), per spin state, numerical and exact".to_string()),
          ("columns", "eps_over_g55: eps/g55; n_min, n_max: smallest and largest eigenvalue of the 8x8 matrix of created particles; n_exact = 1/(exp(2 pi eps/g55) + 1); rel_dev = mean/exact - 1; unitarity: deviation of the evolved states from orthonormality; steps: number of integration steps".to_string()),
          ("method", "commutator-free Magnus method of order four from Q = 1e-9 eps to Q = 1000 eps; projection in the first superadiabatic basis (modes.rs)".to_string()),
          ("command", "deflation modes".to_string())],
        &["eps_over_g55", "n_min", "n_max", "n_exact", "rel_dev", "unitarity", "steps"], &rows)?;
    rep.add("71d-M1", "numerical", &format!("pair creation by the deflation, one direction: for eps/g55 = 0.25, 0.5, .., 3.0 the number of particles created per spin state in a mode that starts empty agrees with 1/(exp(2 pi eps/g55) + 1) to a relative deviation of at most {:.1e} (the numbers range from 0.17 to 6.5e-9); the eight spin states are created equally; the evolved states stay orthonormal to {:.1e}", worst, worst_unit), worst < 1e-4 && worst_unit < 1e-9);

    // ---- 7+1, equal rates, general direction -------------------------------------------
    let mode = Mode { m: 1.0, p: [0.0, 0.1, 0.2, 0.3], kappa: [0.5, -0.8, 0.3] };
    let bg = Background { rate: [1.3, 1.3, 1.3], cst: [1.0, 0.7, 2.0] };
    let r = d71.creation(&mode, &bg, 1e-9, 1000.0, 0.05, 0.01, false);
    let exact = fermi_dirac_creation(mode.eps(), 1.3);
    let dev = ((r.n_min - exact) / exact).abs().max(((r.n_max - exact) / exact).abs());
    rep.add("71d-M2", "numerical", &format!("equal rates g55 = g66 = g77 = 1.3 (constants 1, 0.7, 2), mode with m = 1, p = (0, 0.1, 0.2, 0.3), kappa = (0.5, -0.8, 0.3): the created number per spin state is {:.8e} for all eight states, the law 1/(exp(2 pi eps/g55) + 1) gives {:.8e} (relative deviation {:.1e}): with equal rates the result depends neither on the direction of kappa nor on the constants", r.n_mean, exact, dev), dev < 1e-4);

    // ---- 7+1, anisotropic: two directions with rates 1 and 2 ---------------------------
    // delta = (x4 at which q_6 = eps) - (x4 at which q_5 = eps); kappa_5 = eps fixes the first at x4 = 0
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
            // ratio of the other physical wavenumber to eps at the moment when the first one reaches eps:
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
        &[("content", "7+1 deflating spacetime, unequal rates: particles created in a mode with wavenumbers along x5 and x6 (g55 = 1; eps = m = 1, p = 0, kappa_7 = 0, constants 1)".to_string()),
          ("columns", "g66; delta: x4 at which q_6 = eps minus x4 at which q_5 = eps, in units of 1/g55 (delta > 0: x5 becomes energetic first); ratio: the other physical wavenumber divided by eps at the moment when the first one reaches eps; n_min, n_mean, n_max: smallest, mean and largest eigenvalue of the 8x8 matrix of created particles; fd5 = 1/(exp(2 pi eps/g55) + 1); fd6 = 1/(exp(2 pi eps/g66) + 1)".to_string()),
          ("command", "deflation modes".to_string())],
        &["g66", "delta", "ratio", "n_min", "n_mean", "n_max", "fd5", "fd6"], &rows)?;
    rep.add("71d-M3", "numerical", &format!("unequal rates (g55 = 1 with g66 = 2, 1.2 or 0.5), mode with wavenumbers along x5 and x6: whenever the second physical wavenumber is still below exp(-3) eps at the moment at which the first reaches eps, the mean created number is the Fermi-Dirac law with the rate of the coordinate that becomes energetic first, to within {:.1} per cent; when the two become energetic together the eight spin states are created unequally (largest ratio of the largest to the smallest eigenvalue in the scan: {:.0}) and no closed form is known (data/creation_anisotropic.csv)", 100.0 * far_worst, split_max), far_ok);

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
          ("columns", "case: 1 = eps/g55 = 1, one direction; 2 = eps/g55 = 0.5, one direction; 3 = eps = 1, wavenumbers along x5 (g55 = 1) and x6 (g66 = 2); x4 (the physical wavenumber along x5 equals 1 at x4 = 0); Q_over_eps; n: mean adiabatic particle number".to_string()),
          ("command", "deflation modes".to_string())],
        &["case", "x4", "Q_over_eps", "n"], &rows)?;

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
            // exact for one governing rate: ln(amplification) - S -> (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi mu))
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
          ("cases", "1: m = 1, p = 0, kappa = (1,0,0), rates (1,1,1); 2: m = 1, p = (0.2,0.3,-0.4,0.5), kappa = (0.7,0,0), rates (2,1,1); 3: m = 0.3, kappa = (0,0,1), rates (1,1,0.25); 4: m = 1, p = (0,0.1,0.2,0.3), kappa = (0.5,-0.8,0.3), rates (1.3,1.3,1.3), constants (1,0.7,2); 5: the same mode, rates (1.3,0.7,2), constants (1,0.5,2)".to_string()),
          ("columns", "case; eps; x4_turn: x4 at which Q = eps; x4_late: x4 at which the backward integration starts (WKB exponent 30 beyond the turning value); S_wkb: integral of sqrt(Q^2 - eps^2) dx4 between them; log_amp_min, log_amp_max: logarithm of the amplification of the eight solutions between x4_late and the early time (Q = 1e-9 eps); standing_wave_dev: relative difference of the Gram matrices of the positive- and negative-energy parts at the early time; charge_ratio: |psihat^dagger W psihat|/|psihat^dagger psihat|; steps".to_string()),
          ("command", "deflation modes".to_string())],
        &["case", "eps", "x4_turn", "x4_late", "S_wkb", "log_amp_min", "log_amp_max", "standing_wave_dev", "charge_ratio", "steps"], &rows)?;
    write_csv("extinction_profile.csv",
        &[("content", "4+4 deflating spacetime: |psihat|^2 of one bounded solution as a function of x4, normalized to its maximum, for the five cases of data/extinction_checks.csv".to_string()),
          ("columns", "case; x4; Q_over_eps; norm2: numerical; exact: 2 s |K_(1/2 + i mu)(s)|^2 with s = Q/rate, mu = eps/rate, scaled to the numerical curve by least squares (cases 1 to 4, where one rate governs; nan where not evaluated)".to_string()),
          ("command", "deflation modes".to_string())],
        &["case", "x4", "Q_over_eps", "norm2", "exact"], &prof_rows)?;
    rep.add("44d-M1", "numerical", &format!("modes of the 4+4 field that depend on the deflating times (five cases: one, two or three non-zero kappa_t; equal and unequal rates and constants): beyond the turning value of x4 exactly eight of the sixteen solutions stay bounded; integrated backwards to early times they contain positive- and negative-energy parts of equal weight (relative difference of the two Gram matrices at most {:.1e}): they are standing waves in x4, not travelling particles", sw), sw < 1e-5);
    rep.add("44d-M2", "numerical", &format!("the covariant charge psihat^dagger W psihat of every bounded solution vanishes (ratio to psihat^dagger psihat at most {:.1e}), as it must for a conserved quantity of a solution that dies out", ch), ch < 1e-5);
    rep.add("44d-M3", "numerical", &format!("between the turning value and the late x4 every bounded solution falls by exp(-S) times a factor of order one, S = Int sqrt(Q^2 - eps^2) dx4 = 30 being the WKB exponent (largest difference between the logarithm of the factor and S in the five cases: {:.2}). Where one rate governs (cases 1 to 4) the eight solutions fall by the same factor (spread of the logarithms at most {:.1e}) and its logarithm is S + (1/2) ln 2 - (1/2) ln(1 + exp(-2 pi eps/rate_t)), as the exact solution requires (largest deviation {:.1e}); with three unequal rates (case 5) the factor depends on the spin state (spread {:.2}). The dependence on a deflating time dies out as exp(-S), and S grows as exp(rate_t x4)", amp, spread_one, offset_dev, spread_all), amp < 1.0 && spread_one < 1e-6 && offset_dev < 5e-3);
    rep.add("44d-M4", "numerical", &format!("where one rate governs (one non-zero kappa_t, or equal rates) the computed |psihat|^2 equals the exact form 2 s |K_(1/2 + i mu)(s)|^2 (modified Bessel function of the order 1/2 + i eps/rate_t, s = Q/rate_t) over the range 0.02 < s < 12 with a largest relative deviation of {:.1e}", prof_dev), prof_dev < 1e-5);

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

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let result = match cmd {
        "matrices" => cmd_matrices(),
        "modes" => cmd_modes(),
        _ => {
            println!("deflation: PROJECT spacetimes, Revision_Deflation (x5, x6, x7 deflate exponentially)\n");
            println!("commands (run from the folder Revision_Deflation):");
            println!("  matrices   checks of the flat Dirac matrices (again), of the geometry of the deflating spacetimes,");
            println!("             of the coordinate matrices, the spin connection and the Dirac operators");
            println!("  modes      solutions of the Dirac equations mode by mode: pair creation by the deflation (7+1),");
            println!("             bounded solutions and their extinction (4+4)");
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
