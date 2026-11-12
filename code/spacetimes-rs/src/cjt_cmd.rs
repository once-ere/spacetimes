//! The command `spacetimes cjt`: every number of supplement S4 (the effective action of
//! Cornwall, Jackiw and Tomboulis applied to the two models).
//!
//! Output files (data/):
//!   cjt_toy.csv              zero-dimensional test of the method against the exact integral
//!   cjt_pairing_check.csv    paired state: Newton solution, potential, stability, T_c (checks of gap.rs)
//!   cjt_thermo.csv           paired state: gap, free energy, entropy and heat capacity against temperature
//!   cjt_gl.csv, cjt_gl_summary.csv   Ginzburg-Landau coefficients from the potential (checks of supplement S1)
//!   cjt_threshold.csv        two-particle threshold from the gap equation (check of dimer.rs)
//!   cjt_landscape.csv        order-parameter space from the Dirac matrices of both signatures
//!   cjt_normal.csv, cjt_normal_summary.csv, cjt_fock.csv   normal state in a medium
//!   cjt_thermal.csv, cjt_thermal_profile.csv               melting of the vacuum value at finite temperature
//!   cjt_vacuum.csv           critical coupling of a pair condensate in the vacuum
//!
//! The function returns false if any of the built-in consistency requirements fails.

use spacetimes::cjt_normal::{critical_c, fock_energy, i_scalar, Medium, Thermal, NU};
use spacetimes::cjt_pairing::{gap_squares, pair_bubble_curvature, threshold_d, Potential, ShellModel};
use spacetimes::cjt_toy;
use spacetimes::cjt_vacuum::critical_alpha;
use spacetimes::complex::C64;
use spacetimes::gap::{critical_temperature, fermi_grid, solve_gap, Pairing};
use spacetimes::io::{write_csv, write_csv_str};
use spacetimes::kernel::OneParticle;

const PI: f64 = std::f64::consts::PI;
/// zeta(3)
const ZETA3: f64 = 1.202_056_903_159_594_3;

fn cmd(name: &str) -> (&'static str, String) {
    let _ = name;
    ("command", "spacetimes cjt".to_string())
}

/// Part 1: the zero-dimensional test.
fn toy() -> std::io::Result<bool> {
    println!("\n[1] zero-dimensional test: exact <phi^2> against truncations of Gamma_2 and against perturbation theory");
    println!("{:>7} | {:>9} | {:>9} {:>9} {:>9} {:>9} | {:>9} {:>9} {:>9} {:>9}", "lambda", "exact", "2 loops", "3 loops", "4 loops", "5 loops", "pert 1", "pert 2", "pert 3", "pert 4");
    let mut rows = Vec::new();
    let mut ok = true;
    for &l in &[0.01, 0.03, 0.1, 0.3, 0.6, 1.0, 2.0, 3.0, 5.0, 10.0, 30.0] {
        let (z, g) = cjt_toy::exact(l);
        let mut row = vec![l, z, g];
        let sol: Vec<f64> = (2..=5).map(|n| cjt_toy::solve_gap(n, l).unwrap_or(f64::NAN)).collect();
        row.extend_from_slice(&sol);
        let pert: Vec<f64> = (2..=5).map(|n| cjt_toy::perturbative(n, l)).collect();
        row.extend_from_slice(&pert);
        row.push(-z.ln());
        for (i, n) in (2..=5).enumerate() {
            row.push(if sol[i].is_finite() { cjt_toy::effective_action(n, l, sol[i]) } else { f64::NAN });
        }
        println!("{:>7} | {:>9.6} | {:>9.6} {:>9.6} {:>9.6} {:>9.6} | {:>9.4} {:>9.4} {:>9.4} {:>9.4}", l, g, sol[0], sol[1], sol[2], sol[3], pert[0], pert[1], pert[2], pert[3]);
        // the two-loop (Hartree) truncation always has a solution and stays within 12% of the exact value
        ok &= (sol[0] / g - 1.0).abs() < 0.12;
        rows.push(row);
    }
    write_csv("cjt_toy.csv",
        &[("content", "zero-dimensional phi^4 model S = phi^2/2 + lambda phi^4/24: exact propagator G = <phi^2> and Z, solutions of the CJT gap equation with Gamma_2 truncated at 2 to 5 loops, ordinary perturbation theory to the same orders, and the effective action at the stationary point compared with -ln Z".to_string()),
          ("columns", "G_nloop: solution of 1/G = 1 + Sigma_n(G) (NaN: the truncated equation has no real solution); pert_k: perturbation series to order lambda^k; Gamma_nloop: CJT effective action of the truncation at its stationary point".to_string()),
          cmd("toy")],
        &["lambda", "Z_exact", "G_exact", "G_2loop", "G_3loop", "G_4loop", "G_5loop", "pert_1", "pert_2", "pert_3", "pert_4", "minus_lnZ", "Gamma_2loop", "Gamma_3loop", "Gamma_4loop", "Gamma_5loop"], &rows)?;
    Ok(ok)
}

fn pairing_variant(g: f64, m_s: f64, k_f: f64, channel: &str) -> Pairing {
    let base = Pairing { m: 1.0, g, m_s, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f };
    match channel {
        "V" => base,
        "T" => Pairing { phase_sign: -1.0, ..base },
        _ => Pairing { phase: false, ..base },
    }
}

/// Part 2: the paired state.  Checks of gap.rs.
fn pairing_check() -> std::io::Result<bool> {
    println!("\n[2] paired state: fixed-point solution of gap.rs against Newton's method on the CJT potential, value and stability of the potential, critical temperature");
    println!("{:>3} {:>5} {:>5} {:>2} | {:>11} {:>9} | {:>9} {:>8} {:>9} | {:>11} {:>9} {:>7}", "g", "m_s", "k_F", "ch", "gap/E_F", "Newton-1", "V/V_BCS", "s_min-1", "curv_min", "T_c/gap", "Tc2/Tc-1", "unst.");
    let points: [(f64, f64, f64, &str); 12] = [
        (2.0, 0.3, 0.5, "V"), (2.0, 0.3, 1.0, "V"), (2.0, 1.0, 1.0, "V"), (2.0, 0.1, 1.0, "V"),
        (3.0, 1.0, 0.35, "V"), (3.0, 1.0, 0.5, "V"), (3.0, 1.0, 1.0, "V"), (3.0, 0.3, 1.5, "V"),
        (2.0, 0.3, 0.5, "T"), (3.0, 1.0, 0.5, "T"), (2.0, 0.3, 0.5, "S"), (3.0, 1.0, 0.5, "S"),
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut ok = true;
    for &(g, m_s, k_f, ch) in &points {
        let p = pairing_variant(g, m_s, k_f, ch);
        let ef = p.mu() - 1.0;
        // the grid of the command `gap`
        let grid = fermi_grid(k_f, 60.0 * k_f.max(1.0), 11, 8);
        let pot = Potential::new(&p, &grid);
        let start = (2.0 * ef * (-1.0 / p.lambda().max(1e-3)).exp()).max(1e-11 * ef).min(ef);
        let fp = solve_gap(&p, &grid, 0.0, start, 1e-12, 200000);
        // Newton from a function of different height and shape
        let guess: Vec<f64> = pot.k.iter().map(|&k| 2.5 * fp.delta_f / (1.0 + ((k - k_f) / k_f).powi(2))).collect();
        let (d, _, conv) = pot.newton(&guess, 0.0, 1e-13, 300);
        let diff = d.iter().zip(&fp.delta).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max) / fp.delta_f;
        let gap = pot.at_fermi(&d);
        // potential at the solution and for rescaled gap functions: minimum of the parabola through s = 0.998, 1, 1.002
        let v1 = pot.value(&d, 0.0);
        let vs = |s: f64| pot.value(&d.iter().map(|x| s * x).collect::<Vec<f64>>(), 0.0);
        let (va, vb) = (vs(0.998), vs(1.002));
        let s_min = 1.0 - 0.002 * (vb - va) / (2.0 * (va - 2.0 * v1 + vb));
        // weak-coupling value of the condensation energy
        let v_bcs = -NU * p.n0() * gap * gap / 4.0;
        let curv = pot.minimum_curvature(&d, 0.0);
        // critical temperature: linearized gap equation (gap.rs) and second variation of V
        let tc = critical_temperature(&p, &grid, 1e-11 * ef, 5.0 * (ef + 1.0)).unwrap_or(f64::NAN);
        let tc2 = pot.critical_temperature(0.9 * tc, 1.1 * tc, 34);
        let zero = vec![0.0; pot.n()];
        let unstable_below = pot.second_variation_sign(&zero, 0.9 * tc) < 0.0 && pot.minimum_curvature(&zero, 0.9 * tc).is_none();
        let stable_above = pot.minimum_curvature(&zero, 1.1 * tc).is_some();
        println!("{:>3} {:>5} {:>5} {:>2} | {:>11.4e} {:>9.1e} | {:>9.5} {:>8.1e} {:>9.4} | {:>11.5} {:>9.1e} {:>7}", g, m_s, k_f, ch, gap / ef, diff, v1 / v_bcs, s_min - 1.0,
            curv.unwrap_or(f64::NAN), tc / gap, tc2 / tc - 1.0, unstable_below && stable_above);
        ok &= fp.converged && conv && diff < 1e-6 && (s_min - 1.0).abs() < 1e-5 && curv.is_some() && (tc2 / tc - 1.0).abs() < 1e-6 && unstable_below && stable_above;
        // condensation energy within 15% of the weak-coupling formula where the gap is below 10% of E_F
        if gap < 0.1 * ef {
            ok &= (v1 / v_bcs - 1.0).abs() < 0.15;
        }
        rows.push(vec![format!("{}", g), format!("{}", m_s), format!("{}", k_f), ch.to_string(), format!("{:.10e}", ef), format!("{:.6}", p.lambda()),
            format!("{:.10e}", fp.delta_f), format!("{:.10e}", gap), format!("{:.3e}", diff), format!("{:.10e}", v1), format!("{:.10e}", v_bcs), format!("{:.6}", v1 / v_bcs),
            format!("{:.3e}", s_min - 1.0), format!("{:.6}", curv.unwrap_or(f64::NAN)), format!("{:.10e}", tc), format!("{:.10e}", tc2), format!("{:.3e}", tc2 / tc - 1.0),
            format!("{:.6}", tc / gap), format!("{}", (unstable_below && stable_above) as u8)]);
    }
    write_csv_str("cjt_pairing_check.csv",
        &[("content", "paired state at T = 0 for twelve parameter sets of the scan of the command gap (same momentum grid): solution of the gap equation by fixed-point iteration (gap.rs) and by Newton's method on the CJT potential; the potential at the solution; its second variation; the critical temperature from the linearized gap equation (gap.rs) and from the sign change of the second variation of the potential".to_string()),
          ("channel", "V = vector pair along an extra axis (radial + phase mode attract); T = two-form pair (phase mode repels); S = radial mode only".to_string()),
          ("columns", "gap_fixed_point, gap_newton: gap at the Fermi momentum; max_rel_diff: largest difference of the two gap functions divided by the gap; V: potential per unit volume at the solution (units m^4); V_BCS = -nu N(0) gap^2/4 with nu = 8 and N(0) = k_F E_F'/(2 pi^2); s_min_minus_1: position of the minimum of V along the ray s x solution, minus 1; curvature_min: smallest eigenvalue of 1 - Ktilde at the solution (positive: minimum); Tc_linear: gap.rs; Tc_second_variation: this module; unpaired_state_unstable_below_Tc: 1 if the second variation at Delta = 0 is indefinite at 0.9 Tc and positive at 1.1 Tc".to_string()),
          cmd("pairing")],
        &["g", "m_s", "k_F", "channel", "E_F", "lambda", "gap_fixed_point", "gap_newton", "max_rel_diff", "V", "V_BCS", "V_over_V_BCS", "s_min_minus_1", "curvature_min", "Tc_linear", "Tc_second_variation", "Tc_rel_diff", "Tc_over_gap", "unpaired_state_unstable_below_Tc"], &rows)?;
    Ok(ok)
}

/// Solve the gap equation at temperature t by Newton's method, starting from `start`.
fn gap_at(pot: &Potential, start: &[f64], t: f64) -> Vec<f64> {
    pot.newton(start, t, 1e-12, 200).0
}

/// Part 3: thermodynamics of the paired state, and the Ginzburg-Landau coefficients.
fn thermodynamics() -> std::io::Result<bool> {
    println!("\n[3] thermodynamics of the paired state (g = 3, m_s = 1, k_F = 0.5, vector channel) and Ginzburg-Landau coefficients");
    let p = pairing_variant(3.0, 1.0, 0.5, "V");
    let ef = p.mu() - 1.0;
    let grid = fermi_grid(p.k_f, 60.0, 11, 8);
    let pot = Potential::new(&p, &grid);
    let tc = critical_temperature(&p, &grid, 1e-11 * ef, 5.0).unwrap();
    let d0 = gap_at(&pot, &solve_gap(&p, &grid, 0.0, 0.05 * ef, 1e-10, 100000).delta, 0.0);
    let gap0 = pot.at_fermi(&d0);
    let v0 = pot.value(&d0, 0.0);
    let zero = vec![0.0; pot.n()];
    let mut ok = true;
    let mut rows = Vec::new();
    let ratios = [0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4, 0.45, 0.5, 0.55, 0.6, 0.65, 0.7, 0.75, 0.8, 0.85, 0.9, 0.93, 0.95, 0.97, 0.98, 0.99, 0.995];
    let mut d = d0.clone();
    let mut c_s_near: Vec<(f64, f64)> = Vec::new();
    println!("{:>7} | {:>9} {:>10} {:>9} {:>9}", "T/T_c", "gap/gap0", "V/|V(0)|", "S_s/S_n", "C_s/C_n");
    for &r in &ratios {
        let t = r * tc;
        d = gap_at(&pot, &d, t);
        let gap = pot.at_fermi(&d);
        let v = pot.value(&d, t);
        let (s_s, s_n) = (pot.entropy(&d, t), pot.entropy(&zero, t));
        // heat capacity C = T dS/dT by a central difference; the gap function is recomputed at T (1 +- h)
        let h = 2e-3;
        let (dp, dm) = (gap_at(&pot, &d, t * (1.0 + h)), gap_at(&pot, &d, t * (1.0 - h)));
        let c_s = (pot.entropy(&dp, t * (1.0 + h)) - pot.entropy(&dm, t * (1.0 - h))) / (2.0 * h);
        let c_n = (pot.entropy(&zero, t * (1.0 + h)) - pot.entropy(&zero, t * (1.0 - h))) / (2.0 * h);
        println!("{:>7.3} | {:>9.5} {:>10.6} {:>9.5} {:>9.5}", r, gap / gap0, v / v0.abs(), s_s / s_n, c_s / c_n);
        rows.push(vec![r, t, gap, gap / gap0, v, v / v0.abs(), s_s, s_n, c_s, c_n]);
        if r >= 0.97 {
            c_s_near.push((r, c_s / c_n));
        }
        ok &= gap > 0.0;
    }
    // jump of the heat capacity: linear extrapolation of C_s/C_n from T/T_c = 0.98 and 0.995 to 1
    let (r1, y1) = c_s_near[1];
    let (r2, y2) = c_s_near[3];
    let jump = y2 + (y2 - y1) * (1.0 - r2) / (r2 - r1) - 1.0;
    // slope of the gap near T_c: gap = a T_c sqrt(1 - T/T_c)
    let near = rows.iter().find(|r| (r[0] - 0.98).abs() < 1e-9).unwrap();
    let slope = near[2] / (tc * (1.0 - 0.98f64).sqrt());
    println!("T_c = {:.6e} (= {:.5} gap0);  gap0/T_c = {:.4} (weak coupling: 1.7639);  heat-capacity jump = {:.3} (1.4261);  gap/(T_c sqrt(1 - T/T_c)) at 0.98 T_c = {:.3} (3.063 at T_c)", tc, tc / gap0, gap0 / tc, jump, slope);
    ok &= (gap0 / tc - 1.7639).abs() < 0.03 && (jump - 1.4261).abs() < 0.08;
    write_csv("cjt_thermo.csv",
        &[("content", "paired state against temperature for g = 3, m_s = 1, k_F = 0.5 (units of m), vector channel: gap at the Fermi momentum, CJT potential (free-energy density relative to the unpaired state), entropy and heat capacity of the paired and of the unpaired state".to_string()),
          ("T_c", format!("{:.10e}", tc)), ("gap0", format!("{:.10e}", gap0)), ("V0", format!("{:.10e}", v0)),
          ("gap0_over_Tc", format!("{:.6}", gap0 / tc)), ("heat_capacity_jump", format!("{:.6}", jump)), ("gap_slope_at_0.98", format!("{:.6}", slope)),
          ("columns", "S: entropy density nu Int [ln(1 + e^(-E/T)) + (E/T)/(e^(E/T) + 1)]; C = T dS/dT by central differences with the gap recomputed".to_string()),
          cmd("thermo")],
        &["T_over_Tc", "T", "gap", "gap_over_gap0", "V", "V_over_absV0", "S_paired", "S_unpaired", "C_paired", "C_unpaired"], &rows)?;

    // Ginzburg-Landau coefficients
    println!("\n    Ginzburg-Landau coefficients: V = alpha s^2 + (beta/2) s^4 along Delta(k) = s phi(k)");
    println!("{:>3} {:>5} {:>5} | {:>6} | {:>12} {:>9} | {:>12} {:>9}", "g", "m_s", "k_F", "T/T_c", "alpha", "/S1", "beta", "/S1");
    let mut gl_rows = Vec::new();
    let mut summary = Vec::new();
    for &(g, m_s, k_f) in &[(3.0, 1.0, 0.5), (2.0, 0.3, 1.0), (2.0, 1.0, 1.0)] {
        let p = pairing_variant(g, m_s, k_f, "V");
        let ef = p.mu() - 1.0;
        let grid = fermi_grid(k_f, 60.0 * k_f.max(1.0), 11, 8);
        let pot = Potential::new(&p, &grid);
        let tc = critical_temperature(&p, &grid, 1e-11 * ef, 5.0 * (ef + 1.0)).unwrap();
        let shape = pot.critical_shape(tc);
        let n0 = p.n0();
        let v_f = k_f / p.mu();
        // gradient coefficient
        let k_num = pair_bubble_curvature(&p, &grid, tc);
        let k_s1 = 7.0 * ZETA3 * n0 * v_f * v_f / (96.0 * PI * PI * tc * tc);
        let mut at: Vec<(f64, f64, f64)> = Vec::new();
        for &r in &[0.95, 0.98, 0.99, 1.01, 1.02, 1.05] {
            let t = r * tc;
            // amplitudes small compared with pi T, so that terms beyond s^6 are negligible
            let s_values: Vec<f64> = [0.06, 0.12, 0.18, 0.24, 0.30, 0.36].iter().map(|x| x * t).collect();
            let (alpha, beta) = pot.gl_fit(&shape, t, &s_values);
            // supplement S1: F = a Tr[M M^+] + (b/2) Tr[(M M^+)^2] with Tr 1 = 8:  alpha = 8 a, beta = 8 b
            let alpha_s1 = 4.0 * n0 * r.ln();
            let beta_s1 = 7.0 * ZETA3 * n0 / (2.0 * PI * PI * tc * tc);
            println!("{:>3} {:>5} {:>5} | {:>6.2} | {:>12.5e} {:>9.5} | {:>12.5e} {:>9.5}", g, m_s, k_f, r, alpha, alpha / alpha_s1, beta, beta / beta_s1);
            gl_rows.push(vec![g, m_s, k_f, tc, tc / ef, r, alpha, alpha_s1, alpha / alpha_s1, beta, beta_s1, beta / beta_s1, k_num, k_s1, k_num / k_s1]);
            at.push((r, alpha, beta));
        }
        // values at T_c from the two temperatures next to it (T/T_c = 0.99 and 1.01):
        // slope d alpha/d ln T, to be compared with 4 N(0), and beta(T_c)
        let (lo, hi) = (at[2], at[3]);
        let slope = (hi.1 - lo.1) / (hi.0.ln() - lo.0.ln());
        let beta_c = 0.5 * (lo.2 + hi.2);
        let beta_s1 = 7.0 * ZETA3 * n0 / (2.0 * PI * PI * tc * tc);
        println!("    at T_c:  d alpha/d ln T = {:.5e} = {:.5} x 4 N(0);   beta = {:.5e} = {:.5} x formula of S1;   K = {:.5e} = {:.5} x formula of S1", slope, slope / (4.0 * n0), beta_c, beta_c / beta_s1, k_num, k_num / k_s1);
        summary.push(vec![g, m_s, k_f, tc, tc / ef, p.lambda(), slope, 4.0 * n0, slope / (4.0 * n0), beta_c, beta_s1, beta_c / beta_s1, k_num, k_s1, k_num / k_s1]);
        ok &= (slope / (4.0 * n0) - 1.0).abs() < 0.01 && (beta_c / beta_s1 - 1.0).abs() < 0.015 && (k_num / k_s1 - 1.0).abs() < 0.02;
    }
    write_csv("cjt_gl_summary.csv",
        &[("content", "Ginzburg-Landau coefficients at the critical temperature from the CJT potential, compared with the formulas of supplement S1".to_string()),
          ("columns", "dalpha_dlnT: slope of the quadratic coefficient at T_c (from T/T_c = 0.99 and 1.01), formula 4 N(0); beta_Tc: quartic coefficient at T_c (mean of the values at 0.99 and 1.01 T_c), formula 7 zeta(3) N(0)/(2 pi^2 T_c^2); K: gradient coefficient, formula 7 zeta(3) N(0) v_F^2/(96 pi^2 T_c^2)".to_string()),
          cmd("gl")],
        &["g", "m_s", "k_F", "Tc", "Tc_over_EF", "lambda", "dalpha_dlnT", "four_N0", "alpha_ratio", "beta_Tc", "beta_S1", "beta_ratio", "K", "K_S1", "K_ratio"], &summary)?;
    write_csv("cjt_gl.csv",
        &[("content", "Ginzburg-Landau coefficients extracted from the CJT potential: V = alpha s^2 + (beta/2) s^4 along the ray Delta(k) = s phi(k), phi = critical eigenvector normalized to 1 at the Fermi momentum (fit of V/s^2 by a polynomial of second degree in s^2 at s/T = 0.06 ... 0.36); and the gradient coefficient K from the pair susceptibility at finite pair momentum".to_string()),
          ("formulas_S1", "alpha = 8 a = 4 N(0) ln(T/T_c); beta = 8 b = 7 zeta(3) N(0)/(2 pi^2 T_c^2); K = 7 zeta(3) N(0) v_F^2/(96 pi^2 T_c^2); N(0) = k_F E_F'/(2 pi^2), v_F = k_F/E_F'".to_string()),
          ("note", "away from T_c the coefficients of the potential along a ray of fixed shape differ from the formulas by factors 1 + O(lambda ln(T/T_c)), lambda = N(0) W: for the quartic coefficient the factor is (T_c/T)^2 [1 + 4 (eta(T) - 1)], where eta(T) = 1 - lambda ln(T/T_c) + ... is the eigenvalue of the linearized gap equation (derivation in supplement S4)".to_string()),
          cmd("gl")],
        &["g", "m_s", "k_F", "Tc", "Tc_over_EF", "T_over_Tc", "alpha", "alpha_S1", "alpha_ratio", "beta", "beta_S1", "beta_ratio", "K", "K_S1", "K_ratio"], &gl_rows)?;
    Ok(ok)
}

/// Part 4: the two-particle threshold.
fn threshold() -> std::io::Result<bool> {
    println!("\n[4] threshold of the two-particle bound state from the linearized gap equation in vacuum");
    let d_ode = spacetimes::dimer::critical_d();
    let mut rows = Vec::new();
    let mut ok = true;
    for &m_s in &[0.3, 0.1, 0.03, 0.01, 0.003] {
        let d = threshold_d(m_s);
        println!("    m_s/m = {:>6}:  D_c = {:.5}   (Schroedinger equation in coordinate space, dimer.rs: {:.5})", m_s, d, d_ode);
        rows.push(vec![m_s, d, d_ode, d / d_ode - 1.0]);
    }
    // the non-relativistic limit must agree with dimer.rs
    ok &= (rows[4][1] / d_ode - 1.0).abs() < 3e-3;
    write_csv("cjt_threshold.csv",
        &[("content", "critical value of D = alpha m/m_s at which the linearized gap equation in vacuum (T = 0, mu = m, no Fermi sea) has the eigenvalue 1, for the exchange of the radial mode with the relativistic kernel; compared with the critical coupling of the Yukawa potential from the radial Schroedinger equation (dimer.rs)".to_string()),
          cmd("threshold")],
        &["m_s_over_m", "D_c_gap_equation", "D_c_schroedinger", "relative_difference"], &rows)?;
    Ok(ok)
}

/// Part 5: the order-parameter space, from the Dirac matrices of both signatures.
fn landscape() -> std::io::Result<bool> {
    println!("\n[5] order-parameter space (weak-coupling potential, gap of the unitary state = 1, N(0) = 1)");
    let gm = spacetimes::gamma::Gammas::build();
    let c = |re: f64, im: f64| C64::new(re, im);
    let p = [0.0, 0.3, -0.2, 0.5, 0.0, 0.0, 0.0, 0.0];
    let sm = ShellModel::with_gap(1.0, 200.0, 1.0);
    let r = 0.5f64.sqrt();
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut ok = true;
    for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
        let is44 = op.tag == "1644";
        // (label, Delta^0, Delta^5, Delta^6, Delta^7, expected kind)
        let i_or_1 = if is44 { c(0.0, 1.0) } else { c(1.0, 0.0) }; // relative phase of a unitary state between axis 0 and a second axis
        let other = if is44 { c(1.0, 0.0) } else { c(0.0, 1.0) }; // the relative phase that makes eta_ab Delta^a Delta^b vanish
        let configs: Vec<(&str, [C64; 4])> = vec![
            ("hidden-axis singlet", [c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)]),
            ("along x5", [c(0.0, 0.0), c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)]),
            ("unitary, rotated between x0 and x5", [c(0.8, 0.0), i_or_1.scale(0.6), c(0.0, 0.0), c(0.0, 0.0)]),
            ("unitary, general direction and phase", {
                let ph = C64::cis(0.7);
                [ph.scale(0.5), ph * i_or_1.scale(0.5), ph * i_or_1.scale(-0.5), ph * i_or_1.scale(0.5)]
            }),
            ("half-paired: eta_ab Delta^a Delta^b = 0; larger gap 1", [c(r * r, 0.0), other.scale(r * r), c(0.0, 0.0), c(0.0, 0.0)]),
            ("the phases of the other signature's unitary state", [c(0.8, 0.0), other.scale(0.6), c(0.0, 0.0), c(0.0, 0.0)]),
        ];
        println!("  signature tag {}:", op.tag);
        for (label, delta) in &configs {
            let d2 = gap_squares(&op, delta, &p, 1.0);
            let v = sm.potential(&op, delta, &p, 1.0);
            let (w, vecs, grad) = sm.hessian(&op, delta, &p, 1.0, 1e-3);
            let unitary = (d2[7] - d2[0]).abs() < 1e-9;
            let half = d2[0].abs() < 1e-9;
            let (mut hess_txt, mut flat_txt, mut remark) = ("-".to_string(), "-".to_string(), String::new());
            let (mut flat, mut positive) = (0usize, 0usize);
            if unitary {
                // the potential is smooth here: eigenvalues of the matrix of second derivatives
                flat = w.iter().filter(|x| x.abs() < 1e-3).count();
                positive = w.iter().filter(|x| **x > 1e-3).count();
                let names = ["Re D0", "Im D0", "Re D5", "Im D5", "Re D6", "Im D6", "Re D7", "Im D7"];
                let mut flat_coords = Vec::new();
                for i in 0..8 {
                    let wt: f64 = (0..8).filter(|&e| w[e].abs() < 1e-3).map(|e| vecs.get(i, e).norm_sqr()).sum();
                    if wt > 0.5 {
                        flat_coords.push(names[i]);
                    }
                }
                hess_txt = w.iter().map(|x| format!("{:.4}", x)).collect::<Vec<_>>().join(" ");
                // for a state that is not along one axis the flat directions are combinations of the coordinates
                flat_txt = if label.starts_with("hidden") || label.starts_with("along") { flat_coords.join(" ") } else { "(combinations)".to_string() };
                ok &= (d2[0] - 1.0).abs() < 1e-9 && (v + 2.0).abs() < 1e-3 && grad < 1e-5 && flat == 4 && positive == 4;
            } else if half {
                // the potential contains d^2 ln d of the vanishing gap and is not twice
                // differentiable: use differences along chosen directions instead
                let scaled = |s: f64| -> [C64; 4] { [delta[0].scale(s), delta[1].scale(s), delta[2].scale(s), delta[3].scale(s)] };
                let (up, down) = (sm.potential(&op, &scaled(1.02), &p, 1.0) - v, sm.potential(&op, &scaled(0.98), &p, 1.0) - v);
                // open the second gap: add a small component that makes eta_ab Delta^a Delta^b non-zero
                let mut opened = *delta;
                opened[0] = opened[0] + c(0.05, 0.0);
                let open = sm.potential(&op, &opened, &p, 1.0) - v;
                remark = format!("V changes by {:+.2e} / {:+.2e} when Delta is multiplied by 1.02 / 0.98, and by {:+.2e} when 0.05 is added to Delta^0 (second gap opened)", up, down, open);
                ok &= (d2[7] - 1.0).abs() < 1e-9 && (v + 1.0).abs() < 1e-3 && grad < 1e-4 && up > 0.0 && down > 0.0 && open < 0.0;
            } else {
                remark = "not stationary".to_string();
                ok &= (d2[7] - d2[0]) > 0.1 && v > -2.0 + 1e-3 && grad > 1e-3;
            }
            println!("    {:<56} gaps^2 = {:.4} (x4), {:.4} (x4)   V = {:+.5}   |grad| = {:.1e}   {}", label, d2[0].abs(), d2[7], v, grad,
                if unitary { format!("second derivatives: {} flat, {} positive ({:.4}); flat: {}", flat, positive, w[7], flat_txt) } else { remark.clone() });
            rows.push(vec![op.tag.to_string(), label.to_string(),
                format!("({:+.4}{:+.4}i; {:+.4}{:+.4}i; {:+.4}{:+.4}i; {:+.4}{:+.4}i)", delta[0].re, delta[0].im, delta[1].re, delta[1].im, delta[2].re, delta[2].im, delta[3].re, delta[3].im),
                format!("{:.8}", d2[0].abs()), format!("{:.8}", d2[7]), format!("{:.8}", v), format!("{:.3e}", grad), format!("{}", flat), format!("{}", positive),
                hess_txt, flat_txt, remark]);
        }
    }
    write_csv_str("cjt_landscape.csv",
        &[("content", "the CJT potential at T = 0 in weak coupling as a function of the order parameter Delta^a (a = 0, 5, 6, 7), evaluated with the gap matrix built from the Dirac matrices of each signature; unit: gap of the unitary state = 1, density of states per internal state N(0) = 1, shell half-width 200".to_string()),
          ("columns", "gap2_small, gap2_large: the two fourfold eigenvalues of M M^dagger; V: potential (unitary state: -2; half-paired state: -1); grad: norm of the gradient in the eight real coordinates; for unitary states: flat, positive = numbers of eigenvalues of the matrix of second derivatives that vanish (|.| < 1e-3) or are positive, hessian = its eight eigenvalues, flat_coordinates = the coordinates that span the flat directions (D0 = Delta^0 etc.); remark: for the half-paired state, where the potential is not twice differentiable, changes of V along chosen directions".to_string()),
          cmd("landscape")],
        &["signature", "state", "Delta", "gap2_small", "gap2_large", "V", "grad", "flat", "positive", "hessian", "flat_coordinates", "remark"], &rows)?;
    Ok(ok)
}

/// Part 6: the normal state in a medium.
fn normal_state() -> std::io::Result<bool> {
    println!("\n[6] normal state in a medium: mass of the fermions, energy per particle, self-bound matter");
    let mut ok = true;
    let c_crit = [critical_c(false), critical_c(true)];
    println!("    smallest C = g^2 m^2/m_s^2 with self-bound matter: {:.5} without the sea term (3 pi^2/8 = {:.5} for the abnormal state); {:.5} with it (3 pi^2/5 = {:.5})",
        c_crit[0], 3.0 * PI * PI / 8.0, c_crit[1], 3.0 * PI * PI / 5.0);
    ok &= c_crit[0] <= 3.0 * PI * PI / 8.0 + 1e-4 && c_crit[1] <= 3.0 * PI * PI / 5.0 + 1e-4;
    // profiles
    let mut prof = Vec::new();
    for &c in &[1.0, 2.0, 3.5, 5.0, 8.0] {
        for (is, &sea) in [false, true].iter().enumerate() {
            let md = Medium { c, sea };
            for i in 0..=70 {
                let kf = 0.03 * (4.0f64 / 0.03).powf(i as f64 / 70.0);
                let s = md.state(kf);
                let free = kf * kf / (3.0 * (1.0 + kf * kf));
                prof.push(vec![c, is as f64, kf, s.x, s.n, s.eps / s.n - 1.0, s.p, md.sound_speed_sq(kf), free]);
            }
        }
    }
    write_csv("cjt_normal.csv",
        &[("content", "normal (unpaired) state at T = 0 from the field equation of the Vorton-Scalar in the medium: mass of the fermions, energy per particle, pressure and sound speed against the Fermi momentum".to_string()),
          ("columns", "C = g^2 m^2/m_s^2; sea = 1 if the renormalized energy of the negative-energy states is included; kF in units of m; x = m*/m; n in units of m^3; e_bind = energy per particle minus m, in units of m (negative: bound); P in units of m^4; cs2 = dP/d eps (negative: mechanically unstable); cs2_free = k_F^2/(3 (m^2 + k_F^2))".to_string()),
          cmd("normal")],
        &["C", "sea", "kF", "x", "n", "e_bind", "P", "cs2", "cs2_free"], &prof)?;
    // summary
    println!("{:>6} {:>4} | {:>10} {:>8} {:>7} | {:>9} {:>8} | {:>9}", "C", "sea", "E/N - m", "k_F,sat", "x_sat", "k_F(x=0)", "x before", "k_F(cs2<0)");
    let mut rows = Vec::new();
    for &c in &[0.5, 1.0, 2.0, 3.0, 3.5, 4.0, 5.0, 6.0, 8.0, 12.0, 20.0, 35.5] {
        for (is, &sea) in [false, true].iter().enumerate() {
            let md = Medium { c, sea };
            let (e, kf_sat, x_sat) = md.saturation();
            let bound = e < 1.0 - 1e-9;
            // onset of the abnormal state and of the mechanical instability on a fine grid
            let (mut k_abn, mut x_before, mut k_spin) = (f64::NAN, f64::NAN, f64::NAN);
            let mut last_x = 1.0;
            for i in 0..=2000 {
                let kf = 0.01 * (10.0f64 / 0.01).powf(i as f64 / 2000.0);
                let x = md.mass(kf);
                if x == 0.0 && k_abn.is_nan() {
                    k_abn = kf;
                    x_before = last_x;
                }
                if x > 0.0 && !k_abn.is_nan() {
                    // the massive state returned: not a transition yet (cannot happen for a monotonic x(k_F))
                    k_abn = f64::NAN;
                }
                last_x = x;
                if k_spin.is_nan() && i % 10 == 0 && md.sound_speed_sq(kf) < 0.0 {
                    k_spin = kf;
                }
            }
            println!("{:>6} {:>4} | {:>10.5} {:>8.4} {:>7.4} | {:>9.4} {:>8.4} | {:>9.4}", c, is, if bound { e - 1.0 } else { 0.0 }, if bound { kf_sat } else { f64::NAN }, if bound { x_sat } else { f64::NAN }, k_abn, x_before, k_spin);
            rows.push(vec![c, is as f64, bound as u8 as f64, if bound { e - 1.0 } else { 0.0 }, if bound { kf_sat } else { f64::NAN }, if bound { x_sat } else { f64::NAN }, k_abn, x_before, k_spin]);
        }
    }
    write_csv("cjt_normal_summary.csv",
        &[("content", "normal state at T = 0: is matter self-bound, at which density does the Vorton-Scalar lose its expectation value, where does the mechanical instability begin".to_string()),
          ("critical_C_without_sea", format!("{:.6}", c_crit[0])), ("critical_C_with_sea", format!("{:.6}", c_crit[1])),
          ("columns", "bound = 1 if the lowest energy per particle is below m; e_bind = that energy minus m (units of m); kF_sat, x_sat: Fermi momentum and m*/m there; kF_abnormal: smallest Fermi momentum at which the stable state has m* = 0; x_before: m*/m just below that density (a value well above 0 means a discontinuous transition); kF_spinodal: smallest Fermi momentum with dP/dn < 0".to_string()),
          cmd("normal")],
        &["C", "sea", "bound", "e_bind", "kF_sat", "x_sat", "kF_abnormal", "x_before", "kF_spinodal"], &rows)?;
    // exchange energy
    println!("    exchange (Fock) energy to first order, relative to the mean-field energy shift:");
    let mut frows = Vec::new();
    for &(c, m_s) in &[(2.0, 0.3), (2.0, 1.0), (2.0, 3.0), (5.0, 1.0)] {
        let g2 = c * m_s * m_s;
        for &kf in &[0.1, 0.3, 0.6, 1.0] {
            let md = Medium { c, sea: false };
            let s = md.state(kf);
            // mean-field (Hartree) energy shift: energy density minus that of free fermions of mass m
            let hartree = s.eps - NU * spacetimes::cjt_normal::e_kinetic(kf, 1.0);
            let (es, ep) = fock_energy(kf, s.x, g2, m_s);
            let rho_s = NU * s.x * i_scalar(kf, s.x);
            println!("      C = {:>3}, m_s = {:>3}, g^2 = {:>5.2}, k_F = {:>4}: mean field {:+.4e}, exchange radial {:+.4e} ({:+.1}%), phase {:+.4e} ({:+.1}%)", c, m_s, g2, kf, hartree, es, 100.0 * es / hartree.abs(), ep, 100.0 * ep / hartree.abs());
            frows.push(vec![c, m_s, g2, kf, s.x, s.n, rho_s, hartree, es, ep, es / hartree.abs(), ep / hartree.abs()]);
            ok &= es > 0.0 && ep > 0.0;
        }
    }
    write_csv("cjt_fock.csv",
        &[("content", "exchange (Fock) part of the two-loop Gamma_2 for the normal state, evaluated to first order with the mean-field solution (static exchange): energy densities in units of m^4".to_string()),
          ("columns", "hartree: mean-field energy density minus that of free fermions of mass m at the same density (negative: attraction); fock_radial, fock_phase: exchange energies of the two modes (positive); the last two columns are the ratios to |hartree|".to_string()),
          cmd("normal")],
        &["C", "m_s", "g2", "kF", "x", "n", "rho_s", "hartree", "fock_radial", "fock_phase", "fock_radial_over_hartree", "fock_phase_over_hartree"], &frows)?;
    Ok(ok)
}

/// Part 7: finite temperature.
fn thermal() -> std::io::Result<bool> {
    println!("\n[7] finite temperature: melting of the vacuum value of the Vorton-Scalar (fermions at one loop, bosons in the Hartree approximation)");
    println!("{:>5} {:>5} {:>8} {:>7} | {:>9} {:>9} {:>9} | {:>9} {:>9} | {:>8}", "g", "m_s/m", "lambda", "C", "T_hi/v", "T_N/v", "T_half/v", "phi-/v", "phi+/v", "T_half/m");
    let mut rows = Vec::new();
    let mut prof = Vec::new();
    let mut ok = true;
    for &(g, m_s) in &[(0.1, 1.0), (0.3, 1.0), (0.5, 0.5), (1.0, 3.0), (1.0, 1.0), (2.0, 1.0), (3.0, 1.0)] {
        let th = Thermal { g, lambda: 0.5 * g * g * m_s * m_s };
        let v = th.v();
        let c = g * g / (m_s * m_s);
        let (t_half, below, above) = th.melting();
        println!("{:>5} {:>5} {:>8.4} {:>7.3} | {:>9.5} {:>9.5} {:>9.5} | {:>9.4} {:>9.4} | {:>8.4}", g, m_s, th.lambda, c, th.t_high() / v, th.t_large_n() / v, t_half / v, below, above, t_half);
        rows.push(vec![g, m_s, th.lambda, c, th.t_high() / v, th.t_large_n() / v, t_half / v, below, above, t_half, th.t_high(), th.t_large_n()]);
        // the three estimates of the melting temperature agree within 25%
        ok &= (t_half / th.t_high() - 1.0).abs() < 0.25 && (t_half / th.t_large_n() - 1.0).abs() < 0.25;
        for i in 1..=80 {
            let t = 1.6 * th.t_high() * i as f64 / 80.0;
            if let Some((phi, _, ms2, mt2)) = th.global_minimum(t) {
                prof.push(vec![g, m_s, t / v, t / th.t_high(), phi / v, ms2.sqrt() / m_s, mt2.sqrt() / m_s]);
            }
        }
    }
    write_csv("cjt_thermal.csv",
        &[("content", "temperature at which the expectation value of the Vorton-Scalar melts and the fermions lose their mass, at zero net density: two closed estimates and the Hartree approximation of the CJT potential".to_string()),
          ("columns", "lambda = g^2 m_s^2/(2 m^2); C = g^2 m^2/m_s^2; T_high = sqrt(12 [lambda v^2 + nu g^2 m^2/(8 pi^2)]/(4 lambda + nu g^2)): critical temperature of the one-loop high-temperature expansion (also the temperature above which the symmetric state exists in the Hartree approximation); T_largeN: the same with 2 lambda in place of 4 lambda; T_half: temperature at which the expectation value of the state of lowest Hartree potential falls below v/2; phi_below, phi_above: expectation value in units of v at 0.998 and 1.002 T_half (a large difference means a discontinuous transition); _over_v: in units of the vacuum value v = m/g; _over_m: in units of the fermion mass".to_string()),
          cmd("thermal")],
        &["g", "m_s_over_m", "lambda", "C", "T_high_over_v", "T_largeN_over_v", "T_half_over_v", "phi_below", "phi_above", "T_half_over_m", "T_high_over_m", "T_largeN_over_m"], &rows)?;
    write_csv("cjt_thermal_profile.csv",
        &[("content", "Hartree approximation at finite temperature: the state of lowest potential as a function of temperature, for the seven sets of couplings of cjt_thermal.csv".to_string()),
          ("columns", "T_over_v; T_over_Thigh; phi_over_v (= fermion mass divided by its vacuum value); Ms_over_ms, Mth_over_ms: masses of the radial and of the phase mode in units of the vacuum mass of the radial mode (in the exact theory the phase mode is massless wherever phi > 0; its non-zero value is the known defect of the Hartree approximation)".to_string()),
          cmd("thermal")],
        &["g", "m_s_over_m", "T_over_v", "T_over_Thigh", "phi_over_v", "Ms_over_ms", "Mth_over_ms"], &prof)?;
    Ok(ok)
}

/// Part 8: the vacuum.
fn vacuum() -> std::io::Result<bool> {
    println!("\n[8] vacuum: critical coupling alpha_c = g^2/(4 pi) for a pair condensate without a medium (ladder approximation); pi/2 = {:.5}", 0.5 * PI);
    println!("{:>7} | {:>9} {:>9} {:>9} {:>9} {:>9} {:>9}", "m_s/m", "L=10", "L=30", "L=100", "L=1e3", "L=1e4", "L=1e6");
    let mut rows = Vec::new();
    let mut ok = true;
    for &m_s in &[0.3, 1.0, 3.0] {
        let mut line = format!("{:>7} |", m_s);
        let mut last = f64::MAX;
        for &cut in &[10.0, 30.0, 100.0, 1e3, 1e4, 1e6] {
            let (a, resid) = critical_alpha(m_s, cut);
            line += &format!(" {:>9.4}", a);
            rows.push(vec![m_s, cut, a, (4.0 * PI * a).sqrt(), resid]);
            ok &= resid < 1e-6 && a > 0.5 * PI && a < last;
            last = a;
        }
        println!("{}", line);
    }
    write_csv("cjt_vacuum.csv",
        &[("content", "critical coupling of the linearized ladder (rainbow) Schwinger-Dyson equation for the self-energy generated by a vacuum condensate of the vector pair channel: exchange of the radial mode (mass m_s) and of the massless phase mode, fermion mass m = 1, momentum cutoff Lambda".to_string()),
          ("columns", "alpha_c = g_c^2/(4 pi); g_c; residual of the eigenvalue iteration; for Lambda -> infinity alpha_c -> pi/2".to_string()),
          cmd("vacuum")],
        &["m_s_over_m", "cutoff_over_m", "alpha_c", "g_c", "residual"], &rows)?;
    Ok(ok)
}

/// Run all parts, or the one named by the third command-line argument.
pub fn run() -> std::io::Result<bool> {
    let which = std::env::args().nth(2).unwrap_or_else(|| "all".to_string());
    let t0 = std::time::Instant::now();
    let mut ok = true;
    let mut part = |name: &str, f: fn() -> std::io::Result<bool>| -> std::io::Result<()> {
        if which == "all" || which == name {
            let good = f()?;
            println!("    [{}: {}; {:.0} s]", name, if good { "all requirements met" } else { "A REQUIREMENT FAILED" }, t0.elapsed().as_secs_f64());
            ok &= good;
        }
        Ok(())
    };
    part("toy", toy)?;
    part("pairing", pairing_check)?;
    part("thermo", thermodynamics)?;
    part("threshold", threshold)?;
    part("landscape", landscape)?;
    part("normal", normal_state)?;
    part("thermal", thermal)?;
    part("vacuum", vacuum)?;
    Ok(ok)
}
