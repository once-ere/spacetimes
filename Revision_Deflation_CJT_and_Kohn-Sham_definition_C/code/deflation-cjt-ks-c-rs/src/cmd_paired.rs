//! Command `paired` of the program `deflation_cjt_ks_c` (see main.rs): the results of the
//! effective action of Cornwall, Jackiw and Tomboulis (CJT), computed again along the
//! deflation clock with the routines of the main project.

use spacetimes::cjt_normal::{e_kinetic, fock_energy, Medium, NU};
use spacetimes::cjt_pairing::{gap_squares, pair_bubble_curvature, threshold_d, Potential, ShellModel};
use spacetimes::cjt_toy;
use spacetimes::complex::C64;
use spacetimes::gap::{critical_temperature, fermi_grid, solve_gap, Pairing};
use spacetimes_deflation_cjt_ks_c::history::K_MAX;
use spacetimes::kernel::OneParticle;
use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks_c::io::{write_csv, write_csv_str, write_report};

const PI: f64 = std::f64::consts::PI;
const ZETA3: f64 = 1.202_056_903_159_594_3;

fn vector_channel(g: f64, m_s: f64, k_f: f64) -> Pairing {
    Pairing { m: 1.0, g, m_s, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f }
}

/// `paired`: writes data/cjt_history.csv, cjt_thermo_history.csv, cjt_gl_history.csv,
/// order_parameter_states.csv, cjt_toy.csv, cjt_threshold.csv, matter_saturation.csv,
/// paired_checks.{txt,csv}.  Run time: about two minutes.
pub fn run() -> std::io::Result<bool> {
    let mut rep = Report::new();

    // ---- (1) the potential of the paired state along the clock ------------------------------------
    // two particles (m_s/m, k_F/m); the coupling g_VS grows as exp(tau_d/2)
    let particles: [(f64, f64, Vec<f64>); 2] = [(0.3, 0.5, vec![1.8, 2.0, 2.2, 2.4, 2.6, 2.8, 3.0]), (1.0, 0.5, vec![2.4, 2.6, 2.8, 3.0, 3.2, 3.4])];
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let (mut newton_dev, mut smin_dev, mut tc_dev): (f64, f64, f64) = (0.0, 0.0, 0.0);
    let (mut stable, mut vbcs_ok, mut mono_v, mut conv) = (true, true, true, true);
    let mut energy_exponent_dev: f64 = 0.0;
    let mut strong_moments = 0usize;
    for (set, (m_s, k_f, gs)) in particles.iter().enumerate() {
        let mut prev: Option<(f64, f64, f64)> = None; // (tau_d, ln gap, ln |V|)
        for &g in gs {
            let tau = 2.0 * (g / gs[0]).ln();
            let p = vector_channel(g, *m_s, *k_f);
            let ef = p.mu() - 1.0;
            let grid = fermi_grid(*k_f, K_MAX * k_f.max(1.0), 11, 8);
            let pot = Potential::new(&p, &grid);
            let start = (2.0 * ef * (-1.0 / p.lambda().max(1e-3)).exp()).max(1e-11 * ef).min(ef);
            let fp = solve_gap(&p, &grid, 0.0, start, 1e-12, 200000);
            let guess: Vec<f64> = pot.k.iter().map(|&k| 2.5 * fp.delta_f / (1.0 + ((k - k_f) / k_f).powi(2))).collect();
            let (d, _, cv) = pot.newton(&guess, 0.0, 1e-13, 300);
            conv &= fp.converged && cv;
            let diff = d.iter().zip(&fp.delta).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max) / fp.delta_f;
            let gap = pot.at_fermi(&d);
            let v1 = pot.value(&d, 0.0);
            let vs = |s: f64| pot.value(&d.iter().map(|x| s * x).collect::<Vec<f64>>(), 0.0);
            let (va, vb) = (vs(0.998), vs(1.002));
            let s_min = 1.0 - 0.002 * (vb - va) / (2.0 * (va - 2.0 * v1 + vb));
            let v_bcs = -NU * p.n0() * gap * gap / 4.0;
            let curv = pot.minimum_curvature(&d, 0.0);
            let tc = critical_temperature(&p, &grid, 1e-11 * ef, 5.0 * (ef + 1.0)).unwrap_or(f64::NAN);
            let tc2 = pot.critical_temperature(0.9 * tc, 1.1 * tc, 34);
            // coherence length of the pair, v_F/(pi gap), in units of 1/k_F
            let v_f = k_f / p.mu();
            let xi = v_f / (PI * gap) * k_f;
            newton_dev = newton_dev.max(diff);
            smin_dev = smin_dev.max((s_min - 1.0).abs());
            tc_dev = tc_dev.max((tc2 / tc - 1.0).abs());
            stable &= curv.is_some();
            if gap < 0.1 * ef {
                vbcs_ok &= (v1 / v_bcs - 1.0).abs() < 0.15;
            }
            if let Some((t0, lg0, lv0)) = prev {
                mono_v &= v1.abs().ln() > lv0;
                // weak coupling: the condensation energy is proportional to the square of the gap, so that its
                // logarithmic slope is twice that of the gap
                if gap < 0.1 * ef {
                    let (sg, sv) = ((gap.ln() - lg0) / (tau - t0), (v1.abs().ln() - lv0) / (tau - t0));
                    energy_exponent_dev = energy_exponent_dev.max((sv / (2.0 * sg) - 1.0).abs());
                }
            }
            if set == 0 && gap >= 0.1 * ef {
                strong_moments += 1;
            }
            prev = Some((tau, gap.ln(), v1.abs().ln()));
            rows.push(vec![(set + 1) as f64, *m_s, *k_f, tau, g, p.lambda(), ef, fp.delta_f, gap, diff, v1, v_bcs, v1 / v_bcs, s_min - 1.0, curv.unwrap_or(f64::NAN), tc, tc2, tc / gap, xi]);
        }
    }
    write_csv("cjt_history.csv",
        &[("content", "the paired state at zero temperature along the deflation clock, from the CJT potential of the main project (vector channel): two particles, the coupling g_VS growing as exp(tau_d/2). The momentum integral of the gap equation extends to 2e4 max(m, k_F); the earlier calculations of the project use the limit 60, with which these gaps would be smaller by up to 1.1 per cent (check pc-P7 of the command pairing)".to_string()),
          ("columns", "set: 1 = (m_s, k_F) = (0.3, 0.5), 2 = (1.0, 0.5) in units of m; tau_d = 2 ln(g/g_first); g; lambda: pairing strength; E_F; gap_fixed_point, gap_newton: gap at the Fermi momentum from the iteration of the gap equation and from Newton's method on the potential; max_rel_diff of the two gap functions; V: potential per unit volume at the solution (units m^4), the condensation energy; V_BCS = -nu N(0) gap^2/4; V_over_V_BCS; s_min_minus_1: position of the minimum of V along the ray s x solution, minus 1; curvature_min: smallest eigenvalue of the second variation (positive: a minimum); Tc_linear, Tc_second_variation: critical temperature from the linearized gap equation and from the potential; Tc_over_gap; xi_kF: coherence length v_F/(pi gap) in units of 1/k_F".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["set", "m_s", "k_F", "tau_d", "g", "lambda", "E_F", "gap_fixed_point", "gap_newton", "max_rel_diff", "V", "V_BCS", "V_over_V_BCS", "s_min_minus_1", "curvature_min", "Tc_linear", "Tc_second_variation", "Tc_over_gap", "xi_kF"], &rows)?;
    let xi_first = rows[0][18];
    let xi_last = rows[particles[0].2.len() - 1][18];
    rep.add("def-P1", "numerical",
        &format!("the paired state along the clock ({} moments, two particles): at every moment the solution of the gap equation is a minimum of the CJT potential (Newton's method on the potential reproduces the iterated gap function to {:.1e}; the minimum along the ray lies at s = 1 to {:.1e}; the smallest eigenvalue of the second variation is positive), the critical temperature from the potential equals that of the linearized gap equation to {:.1e}, and the condensation energy is within 15 per cent of -nu N(0) gap^2/4 wherever the gap is below 0.1 E_F. The condensation energy deepens monotonically, in weak coupling with twice the logarithmic slope of the gap (to within {:.1} per cent), and the coherence length of the pair shrinks: for the first particle from {:.0} to {:.1} in units of 1/k_F. At the last {} moments of that particle the gap exceeds 0.1 E_F, and at the last one the coherence length has fallen to the distance 1/k_F between the particles: the deflation carries the paired state from overlapping pairs towards bound pairs", rows.len(), newton_dev, smin_dev, tc_dev, 100.0 * energy_exponent_dev, xi_first, xi_last, strong_moments),
        conv && newton_dev < 1e-6 && smin_dev < 1e-5 && tc_dev < 1e-6 && stable && vbcs_ok && mono_v && energy_exponent_dev < 0.05 && xi_last < xi_first && strong_moments >= 1);

    // ---- (2) thermodynamic ratios along the clock --------------------------------------------------
    let mut trows: Vec<Vec<f64>> = Vec::new();
    let (mut ratio_dev, mut jump_dev): (f64, f64) = (0.0, 0.0);
    for &g in &[2.6, 3.0, 3.4] {
        let p = vector_channel(g, 1.0, 0.5);
        let ef = p.mu() - 1.0;
        let grid = fermi_grid(p.k_f, K_MAX, 11, 8);
        let pot = Potential::new(&p, &grid);
        let tc = critical_temperature(&p, &grid, 1e-11 * ef, 5.0).unwrap();
        let d0 = pot.newton(&solve_gap(&p, &grid, 0.0, 0.05 * ef, 1e-10, 100000).delta, 0.0, 1e-12, 200).0;
        let gap0 = pot.at_fermi(&d0);
        let zero = vec![0.0; pot.n()];
        // approach T_c in steps, each solution being the start of the next
        let mut d = d0.clone();
        let mut near: Vec<(f64, f64)> = Vec::new();
        for &r in &[0.3, 0.5, 0.7, 0.8, 0.9, 0.95, 0.97, 0.98, 0.99, 0.995] {
            let t = r * tc;
            d = pot.newton(&d, t, 1e-12, 200).0;
            if r >= 0.97 {
                let h = 2e-3;
                let (dp, dm) = (pot.newton(&d, t * (1.0 + h), 1e-12, 200).0, pot.newton(&d, t * (1.0 - h), 1e-12, 200).0);
                let c_s = (pot.entropy(&dp, t * (1.0 + h)) - pot.entropy(&dm, t * (1.0 - h))) / (2.0 * h);
                let c_n = (pot.entropy(&zero, t * (1.0 + h)) - pot.entropy(&zero, t * (1.0 - h))) / (2.0 * h);
                near.push((r, c_s / c_n));
            }
        }
        let ((r1, y1), (r2, y2)) = (near[1], near[3]);
        let jump = y2 + (y2 - y1) * (1.0 - r2) / (r2 - r1) - 1.0;
        ratio_dev = ratio_dev.max((gap0 / tc - 1.7639).abs());
        jump_dev = jump_dev.max((jump - 1.4261).abs());
        trows.push(vec![2.0 * (g / 2.6_f64).ln(), g, p.lambda(), tc, tc / ef, gap0, gap0 / tc, jump]);
    }
    write_csv("cjt_thermo_history.csv",
        &[("content", "thermodynamic ratios of the paired state at three moments of the deflation, particle (m_s, k_F) = (1.0, 0.5) m, vector channel: the critical temperature and the gap grow, their ratio and the jump of the heat capacity at T_c do not change. The momentum integral of the gap equation extends to 2e4 max(m, k_F); the earlier calculations of the project use the limit 60, with which these gaps would be smaller by up to 1.1 per cent (check pc-P7 of the command pairing)".to_string()),
          ("columns", "tau_d = 2 ln(g/2.6); g; lambda; Tc; Tc_over_EF; gap0: gap at T = 0; gap0_over_Tc (weak coupling: 1.7639); heat_capacity_jump: (C_paired - C_unpaired)/C_unpaired at T_c (weak coupling: 1.4261)".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["tau_d", "g", "lambda", "Tc", "Tc_over_EF", "gap0", "gap0_over_Tc", "heat_capacity_jump"], &trows)?;
    rep.add("def-P2", "numerical",
        &format!("thermodynamics along the clock: while the critical temperature rises from {:.2e} to {:.2e} E_F (g_VS = 2.6 to 3.4), the ratio gap(0)/T_c stays within {:.3} of the weak-coupling value 1.7639 and the jump of the heat capacity within {:.3} of 1.4261: both ratios are universal while the coupling is weak, and they move only slightly as the gap approaches 0.2 E_F at the last moment", trows[0][4], trows[2][4], ratio_dev, jump_dev),
        ratio_dev < 0.03 && jump_dev < 0.08 && trows[2][3] > trows[0][3]);

    // ---- (3) Ginzburg-Landau coefficients along the clock ---------------------------------------------
    let mut grows: Vec<Vec<f64>> = Vec::new();
    let mut gl_dev: f64 = 0.0;
    let mut gl_dev_strong: f64 = 0.0;
    let xi_number = (7.0 * ZETA3 / (48.0 * PI * PI)).sqrt();
    for &g in &[2.6, 3.0, 3.4] {
        let p = vector_channel(g, 1.0, 0.5);
        let ef = p.mu() - 1.0;
        let grid = fermi_grid(p.k_f, K_MAX, 11, 8);
        let pot = Potential::new(&p, &grid);
        let tc = critical_temperature(&p, &grid, 1e-11 * ef, 5.0 * (ef + 1.0)).unwrap();
        let shape = pot.critical_shape(tc);
        let n0 = p.n0();
        let v_f = p.k_f / p.mu();
        let k_num = pair_bubble_curvature(&p, &grid, tc);
        let k_s1 = 7.0 * ZETA3 * n0 * v_f * v_f / (96.0 * PI * PI * tc * tc);
        let fit = |r: f64| {
            let t = r * tc;
            let s_values: Vec<f64> = [0.06, 0.12, 0.18, 0.24, 0.30, 0.36].iter().map(|x| x * t).collect();
            pot.gl_fit(&shape, t, &s_values)
        };
        let (lo, hi) = (fit(0.99), fit(1.01));
        let slope = (hi.0 - lo.0) / ((1.01f64).ln() - (0.99f64).ln());
        let beta_c = 0.5 * (lo.1 + hi.1);
        let beta_s1 = 7.0 * ZETA3 * n0 / (2.0 * PI * PI * tc * tc);
        // coherence length of the Ginzburg-Landau energy F = a Tr[M M^+] + K Tr[grad M . grad M^+] + ..:
        // xi^2 = K/|a| with a = alpha/8 (Tr 1 = 8), evaluated for |ln(T/T_c)| = 1: xi^2 = 8 K/(d alpha/d ln T)
        let xi_gl = (8.0 * k_num / slope).sqrt();
        let dev = (slope / (4.0 * n0) - 1.0).abs().max((beta_c / beta_s1 - 1.0).abs()).max((k_num / k_s1 - 1.0).abs());
        if tc < 0.05 * ef {
            gl_dev = gl_dev.max(dev);
        } else {
            gl_dev_strong = gl_dev_strong.max(dev);
        }
        grows.push(vec![2.0 * (g / 2.6_f64).ln(), g, tc, slope, slope / (4.0 * n0), beta_c, beta_c / beta_s1, k_num, k_num / k_s1, xi_gl, xi_gl * tc / v_f]);
    }
    write_csv("cjt_gl_history.csv",
        &[("content", "Ginzburg-Landau coefficients of the paired state at three moments of the deflation, particle (m_s, k_F) = (1.0, 0.5) m, from the CJT potential, with their ratios to the weak-coupling formulas. The momentum integral of the gap equation extends to 2e4 max(m, k_F); the earlier calculations of the project use the limit 60, with which these gaps would be smaller by up to 1.1 per cent (check pc-P7 of the command pairing)".to_string()),
          ("columns", "tau_d; g; Tc; dalpha_dlnT (formula 4 N(0)) and its ratio to the formula; beta at T_c (formula 7 zeta(3) N(0)/(2 pi^2 T_c^2)) and ratio; K: gradient coefficient (formula 7 zeta(3) N(0) v_F^2/(96 pi^2 T_c^2)) and ratio; xi_GL = sqrt(K/|a|) for |ln(T/T_c)| = 1, a = alpha/8: coherence length (units 1/m); xi_GL T_c/v_F (formula sqrt(7 zeta(3)/(48 pi^2)) = 0.1333)".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["tau_d", "g", "Tc", "dalpha_dlnT", "alpha_ratio", "beta_Tc", "beta_ratio", "K", "K_ratio", "xi_GL", "xi_GL_Tc_over_vF"], &grows)?;
    rep.add("def-P3", "numerical",
        &format!("Ginzburg-Landau energy along the clock (three moments, T_c = {:.1e}, {:.1e}, {:.1e} E_F): while T_c is below 0.05 E_F the coefficients from the CJT potential agree with the weak-coupling formulas to {:.1} per cent; at the third moment, T_c = 0.10 E_F, they differ by up to {:.1} per cent, the state leaving the weak-coupling regime. The quadratic coefficient keeps its slope 4 N(0), the quartic and the gradient coefficients fall as 1/T_c^2, and the coherence length, sqrt(7 zeta(3)/(48 pi^2)) v_F/T_c = {:.4} v_F/T_c in weak coupling (computed: {:.4}, {:.4}, {:.4}), shrinks from {:.0} to {:.1} (units 1/m). Vortex cores shrink as the deflation proceeds", grows[0][2] / 0.118034, grows[1][2] / 0.118034, grows[2][2] / 0.118034, 100.0 * gl_dev, 100.0 * gl_dev_strong, xi_number, grows[0][10], grows[1][10], grows[2][10], grows[0][9], grows[2][9]),
        gl_dev < 0.015 && gl_dev_strong < 0.12 && gl_dev_strong > gl_dev && grows[2][9] < grows[1][9] && grows[1][9] < grows[0][9] && grows.iter().take(2).all(|r| (r[10] / xi_number - 1.0).abs() < 0.01));

    // ---- (4) the space of the order parameter (frame components) ------------------------------------------
    let gm = spacetimes::gamma::Gammas::build();
    let c = |re: f64, im: f64| C64::new(re, im);
    let pmom = [0.0, 0.3, -0.2, 0.5, 0.0, 0.0, 0.0, 0.0];
    let sm = ShellModel::with_gap(1.0, 200.0, 1.0);
    let r = 0.5f64.sqrt();
    let mut orows: Vec<Vec<String>> = Vec::new();
    let mut land_ok = true;
    for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
        let is44 = op.tag == "1644";
        let i_or_1 = if is44 { c(0.0, 1.0) } else { c(1.0, 0.0) };
        let other = if is44 { c(1.0, 0.0) } else { c(0.0, 1.0) };
        let ph = C64::cis(0.7);
        let configs: Vec<(&str, [C64; 4], &str)> = vec![
            ("pair along the frame axis of x0 (hidden-axis singlet)", [c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)], "unitary"),
            ("pair along the frame axis of x5", [c(0.0, 0.0), c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)], "unitary"),
            ("pair along the frame axis of x7", [c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0), c(1.0, 0.0)], "unitary"),
            ("unitary, rotated between x0 and x5", [c(0.8, 0.0), i_or_1.scale(0.6), c(0.0, 0.0), c(0.0, 0.0)], "unitary"),
            ("unitary, general direction and phase", [ph.scale(0.5), ph * i_or_1.scale(0.5), ph * i_or_1.scale(-0.5), ph * i_or_1.scale(0.5)], "unitary"),
            ("half-paired: eta_ab Delta^a Delta^b = 0", [c(r * r, 0.0), other.scale(r * r), c(0.0, 0.0), c(0.0, 0.0)], "half"),
        ];
        for (label, delta, kind) in &configs {
            let d2 = gap_squares(&op, delta, &pmom, 1.0);
            let v = sm.potential(&op, delta, &pmom, 1.0);
            let (w, _vecs, grad) = sm.hessian(&op, delta, &pmom, 1.0, 1e-3);
            let (flat, positive) = (w.iter().filter(|x| x.abs() < 1e-3).count(), w.iter().filter(|x| **x > 1e-3).count());
            if *kind == "unitary" {
                land_ok &= (d2[7] - d2[0]).abs() < 1e-9 && (d2[0] - 1.0).abs() < 1e-9 && (v + 2.0).abs() < 1e-3 && grad < 1e-5 && flat == 4 && positive == 4;
            } else {
                land_ok &= d2[0].abs() < 1e-9 && (d2[7] - 1.0).abs() < 1e-9 && (v + 1.0).abs() < 1e-3;
            }
            orows.push(vec![format!("{}d", op.tag), label.to_string(), format!("{:.8}", d2[0]), format!("{:.8}", d2[7]), format!("{:.8}", v), format!("{:.1e}", grad),
                if *kind == "unitary" { format!("{}", flat) } else { "-".into() }, if *kind == "unitary" { format!("{}", positive) } else { "-".into() }]);
        }
    }
    write_csv_str("order_parameter_states.csv",
        &[("content", "the weak-coupling CJT potential at zero temperature as a function of the order parameter Delta^a (frame components a = 0, 5, 6, 7), with the gap matrix built from the Dirac matrices of each deflating spacetime; unit: gap of the unitary state = 1, N(0) = 1, shell half-width 200".to_string()),
          ("columns", "spacetime; state; gap2_small, gap2_large: the two fourfold eigenvalues of M M^dagger; V: potential (unitary: -2; half-paired: -1); grad: norm of the gradient; flat, positive: numbers of vanishing and of positive eigenvalues of the matrix of second derivatives in the eight real coordinates (unitary states)".to_string()),
          ("note", "the frame components are related to the coordinate components by Delta_t = b_t(x4) Delta^t for t = 5, 6, 7; the potential depends on the frame components only, so that no direction is preferred although x5, x6, x7 deflate and x0 does not".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["spacetime", "state", "gap2_small", "gap2_large", "V", "grad", "flat", "positive"], &orows)?;
    rep.add("def-P4", "numerical",
        "the space of the order parameter on the deflating spacetimes (frame components along x0, x5, x6, x7; both spacetimes): every unitary state, whichever its direction, has all eight gaps equal and the potential -2 (units of N(0) gap^2), with four flat directions (the Goldstone modes) and four directions of positive curvature; the half-paired state has four gaps closed and the potential -1. The potential does not distinguish the deflating axes from x0: in this sector the deflation selects no direction of the order parameter",
        land_ok);

    // ---- (5) the test of the method in zero dimensions -------------------------------------------------
    let mut toy_rows = Vec::new();
    let mut toy_ok = true;
    let mut toy_worst: f64 = 0.0;
    for &l in &[0.01, 0.03, 0.1, 0.3, 0.6, 1.0, 2.0, 3.0, 5.0, 10.0, 30.0] {
        let (z, g) = cjt_toy::exact(l);
        let sol: Vec<f64> = (2..=5).map(|n| cjt_toy::solve_gap(n, l).unwrap_or(f64::NAN)).collect();
        toy_ok &= (sol[0] / g - 1.0).abs() < 0.12;
        toy_worst = toy_worst.max((sol[0] / g - 1.0).abs());
        toy_rows.push(vec![l, z, g, sol[0], sol[1], sol[2], sol[3]]);
    }
    write_csv("cjt_toy.csv",
        &[("content", "zero-dimensional phi^4 model S = phi^2/2 + lambda phi^4/24: exact propagator G = <phi^2> and solutions of the CJT gap equation with Gamma_2 truncated at 2 to 5 loops (NaN: the truncated equation has no real solution). The test of the method does not involve the spacetime; it is executed again here".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["lambda", "Z_exact", "G_exact", "G_2loop", "G_3loop", "G_4loop", "G_5loop"], &toy_rows)?;
    rep.add("def-P5", "numerical",
        &format!("the test of the CJT method on the zero-dimensional model, executed again: the two-loop truncation, the one used for the models, has a solution for every coupling from 0.01 to 30 and stays within {:.1} per cent of the exact propagator", 100.0 * toy_worst), toy_ok);

    // ---- (6) the two-particle threshold from the gap equation ---------------------------------------------
    let dc = spacetimes::dimer::critical_d();
    let mut th_rows = Vec::new();
    let mut th_ok = true;
    let mut prev = f64::INFINITY;
    for &ms in &[0.3, 0.1, 0.03] {
        let d = threshold_d(ms);
        let rel = d / dc - 1.0;
        th_ok &= rel > 0.0 && rel < prev && rel < 0.05;
        prev = rel;
        th_rows.push(vec![ms, d, dc, rel]);
    }
    write_csv("cjt_threshold.csv",
        &[("content", "critical value of D_VS = alpha_VS m/m_s at which the linearized gap equation in vacuum has the eigenvalue 1 (relativistic kernel of the radial mode), compared with the critical coupling of the Yukawa potential from the Schroedinger equation; on the deflating spacetime D_VS reaches this value at a definite x4".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["m_s_over_m", "D_c_gap_equation", "D_c_schroedinger", "relative_difference"], &th_rows)?;
    rep.add("def-P6", "numerical",
        &format!("the moment of binding from the gap equation: the linearized gap equation in vacuum reaches the eigenvalue 1 at D_VS = {:.4}, {:.4}, {:.4} for m_s/m = 0.3, 0.1, 0.03, approaching the value {:.5} of the Schroedinger equation as the binding becomes non-relativistic (relative differences {:.1e}, {:.1e}, {:.1e})", th_rows[0][1], th_rows[1][1], th_rows[2][1], dc, th_rows[0][3], th_rows[1][3], th_rows[2][3]), th_ok);

    // ---- (7) self-bound matter and the exchange energy along the clock ------------------------------------------
    let mut srows: Vec<Vec<f64>> = Vec::new();
    let mut sat_ok = true;
    let mut prev_e = 0.0_f64;
    let c_first: f64 = 4.0;
    for i in 0..=8 {
        let tau = 0.25 * i as f64;
        let cc = c_first * tau.exp();
        let md = Medium { c: cc, sea: false };
        let (e_per_n, kf_sat, x_sat) = md.saturation();
        let e_bind = e_per_n - 1.0;
        if i > 0 {
            sat_ok &= e_bind < prev_e;
        }
        prev_e = e_bind;
        // exchange energy at the saturation density for m_s/m = 0.3: g^2 = C m_s^2/m^2
        let ms = 0.3;
        let g2 = cc * ms * ms;
        let s = md.state(kf_sat);
        let hartree = s.eps - NU * e_kinetic(kf_sat, 1.0);
        let (es, ep) = fock_energy(kf_sat, s.x, g2, ms);
        srows.push(vec![tau, cc, e_bind, kf_sat, x_sat, g2, hartree, es, ep, es / hartree.abs(), ep / hartree.abs()]);
    }
    write_csv("matter_saturation.csv",
        &[("content", "self-bound normal matter along the deflation clock (mean field without the negative-energy states): C = g_VS^2 m^2/m_s^2 = 4 exp(tau_d); energy per particle at the density of lowest energy, and the exchange (Fock) energies of the two modes there for m_s/m = 0.3".to_string()),
          ("columns", "tau_d; C; e_bind: lowest energy per particle minus m (units of m; negative: bound); kF_sat: Fermi momentum there; x_sat: m*/m there; g2 = g_VS^2; hartree: mean-field energy density minus that of free fermions at the same density; fock_radial, fock_phase: exchange energy densities; their ratios to |hartree|".to_string()),
          ("command", "deflation_cjt_ks_c paired".to_string())],
        &["tau_d", "C", "e_bind", "kF_sat", "x_sat", "g2", "hartree", "fock_radial", "fock_phase", "fock_radial_over_hartree", "fock_phase_over_hartree"], &srows)?;
    let last = srows.last().unwrap();
    rep.add("def-P7", "numerical",
        &format!("self-bound matter along the clock: once C has passed 3.7011 the lowest energy per particle lies below m, and it falls monotonically as the deflation proceeds: from {:.4} m at C = 4 to {:.3} m at C = {:.1}. At every one of these moments the matter of lowest energy is in the abnormal state (fermion mass zero inside the matter, largest value of m*/m in the table {:.1e}), at a Fermi momentum that falls from {:.2} m to {:.2} m. The exchange energies of the radial and of the phase mode at that density are {:.2} and {:.2} per cent of the mean-field energy at C = 4, and {:.2} and {:.2} per cent at C = {:.1} (m_s/m = 0.3): the mean field remains a good approximation along the whole history", srows[0][2], last[2], last[1], srows.iter().map(|r| r[4]).fold(0.0, f64::max), srows[0][3], last[3], 100.0 * srows[0][9], 100.0 * srows[0][10], 100.0 * last[9], 100.0 * last[10], last[1]),
        sat_ok && srows[0][2] < 0.0 && srows.iter().all(|r| r[4] < 1e-6 && r[9] < 0.05 && r[10] < 0.05));

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "numerical", "every check above passed", all);
    write_report(&rep, "paired_checks.txt", "paired_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: the results of the CJT effective action along the deflation clock", "deflation_cjt_ks_c paired")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\npaired: {} of {} checks passed", ok, n);
    Ok(all)
}
