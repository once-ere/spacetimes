//! Command `gravity` of the program `deflation_cjt_ks_c` (see main.rs): the author's
//! Einstein-Lovelock equations with Kohn-Sham matter as their source (module `gravity`).
//! (1) The self-consistent evolution of the author's document KOHN_SHAM_GRAVITY is
//! reproduced; (2) the same equations are solved on the deflating spacetimes.
//!
//! Writes data/gravity_reference.csv, gravity_reference_path.csv, gravity_deflating.csv,
//! gravity_deflating_summary.csv, gravity_vacuum_perturbation.csv, gravity_checks.{txt,csv}.

use spacetimes::verify::Report;
use spacetimes_deflation_cjt_ks_c::gravity::{field_equations, Evolution, Geometry, Lovelock, Matter, State};
use spacetimes_deflation_cjt_ks_c::io::{write_csv, write_csv_str, write_report};
use spacetimes_deflation_cjt_ks_c::halves::{binding_threshold, halves_for, Form, Hf4, KF_SCAN};
use spacetimes_deflation_cjt_ks_c::kohn_sham::{ks_solve, saturation, Contact, Fermi, NU};
use spacetimes_deflation_cjt_ks_c::selfint::SelfTables;

const CMD: &str = "deflation_cjt_ks_c gravity";

/// Integrate from the state `s0` to x4 = `x_end` with `steps` Runge-Kutta steps.  Returns the
/// path (every `keep`-th step), the largest relative violation of the constraint, and whether
/// the integration reached the end (false: the linear system for the accelerations degenerated,
/// or a scale factor left the range in which the matter integrals are evaluated).
fn integrate(ev: &Evolution, s0: State, x_end: f64, steps: usize, keep: usize) -> (Vec<State>, f64, bool) {
    let dt = (x_end - s0.x4) / steps as f64;
    let mut s = s0;
    let mut path = vec![s];
    let mut worst: f64 = 0.0;
    for i in 1..=steps {
        match ev.step(&s, dt) {
            Some(n) => s = n,
            None => return (path, worst, false),
        }
        if !(s.a.is_finite() && s.b.is_finite() && s.a > 1e-3 && s.b > 1e-3 && s.a < 1e3 && s.b < 1e3 && s.h_a.abs() < 1e3 && s.h_b.abs() < 1e3) {
            return (path, worst, false);
        }
        if i % keep == 0 || i == steps {
            let (c, scale) = ev.constraint(&s);
            worst = worst.max(c.abs() / scale);
            path.push(s);
        }
    }
    (path, worst, true)
}

/// Integration with a step that follows the rates: dt = 0.004/max(|H_a|, |H_b|, g).  Stops at
/// x4 = x_end ("end"), when a rate exceeds 50 g in magnitude ("runaway": the rates grow without
/// bound in a finite time), or when the two equations for the accelerations stop being
/// independent ("degenerate").  Returns the path (about every `keep`-th step), the largest
/// relative violation of the constraint, and the reason for stopping.
fn integrate_adaptive(ev: &Evolution, s0: State, x_end: f64, g: f64, keep: usize) -> (Vec<State>, f64, &'static str) {
    let mut s = s0;
    let mut path = vec![s];
    let mut worst: f64 = 0.0;
    let mut i = 0usize;
    loop {
        let hmax = s.h_a.abs().max(s.h_b.abs()).max(g);
        if hmax > 50.0 * g {
            path.push(s);
            return (path, worst, "runaway");
        }
        let mut dt = 0.004 / hmax;
        let last = s.x4 + dt >= x_end;
        if last {
            dt = x_end - s.x4;
        }
        match ev.step(&s, dt) {
            Some(n) => s = n,
            None => {
                path.push(s);
                return (path, worst, "degenerate");
            }
        }
        i += 1;
        if i % keep == 0 || last {
            let (c, scale) = ev.constraint(&s);
            worst = worst.max(c.abs() / scale);
            path.push(s);
        }
        if last {
            return (path, worst, "end");
        }
    }
}

/// Whether the perturbed vacuum solution moves away from the rate -g: after two e-foldings of the
/// clock the rate H_b is more negative than at the start.
fn end_faster(ev: &Evolution, ha0: f64, hb0: f64, g: f64) -> bool {
    let (path, _, _) = integrate_adaptive(ev, State { x4: 0.0, a: 1.0, b: 1.0, h_a: ha0, h_b: hb0 }, 2.0 / (3.0 * g), g, 10);
    path.last().unwrap().h_b < hb0
}

pub fn run() -> std::io::Result<bool> {
    let t0 = std::time::Instant::now();
    let mut rep = Report::new();
    let hartree = Contact { m: 1.0, gs: -1.0e5, gv: 1.0e4 };
    let kf_sat = 0.756636150701493;

    // =====================================================================================
    // 1. the components of the field equations against the polynomials printed in the author's document
    // =====================================================================================
    let geo34 = Geometry { n_a: 3, n_b: 4 };
    let mut dev: f64 = 0.0;
    for (ha, hb) in [(0.37_f64, -0.21_f64), (0.05, 0.0091437124), (-1.3, 0.4), (0.02, 0.9)] {
        let h = [ha, hb];
        let only = |k: usize| {
            let mut alpha = [0.0; 3];
            alpha[k] = 1.0;
            field_equations(&geo34, &Lovelock { alpha, lambda_bar: 0.0, kappa: 0.0 }, &h)
        };
        let mut cmp = |x: f64, y: f64| dev = dev.max((x - y).abs() / (1.0 + y.abs()));
        let e = only(0);
        cmp(e[0].c0, 3.0 * (ha * ha + 4.0 * ha * hb + 2.0 * hb * hb));
        cmp(e[1].ca, 2.0);
        cmp(e[1].cb, 4.0);
        cmp(e[1].c0, 3.0 * ha * ha + 8.0 * ha * hb + 10.0 * hb * hb);
        cmp(e[2].ca, 3.0);
        cmp(e[2].cb, 3.0);
        cmp(e[2].c0, 6.0 * ha * ha + 9.0 * ha * hb + 6.0 * hb * hb);
        let e = only(1);
        cmp(e[0].c0, -12.0 * hb * (4.0 * ha.powi(3) + 18.0 * ha * ha * hb + 12.0 * ha * hb * hb + hb.powi(3)));
        cmp(e[1].ca, -32.0 * ha * hb - 48.0 * hb * hb);
        cmp(e[1].cb, -16.0 * ha * ha - 96.0 * ha * hb - 48.0 * hb * hb);
        cmp(e[1].c0, -32.0 * ha.powi(3) * hb - 136.0 * ha * ha * hb * hb - 192.0 * ha * hb.powi(3) - 60.0 * hb.powi(4));
        cmp(e[2].ca, -12.0 * ha * ha - 72.0 * ha * hb - 36.0 * hb * hb);
        cmp(e[2].cb, -36.0 * ha * ha - 72.0 * ha * hb - 12.0 * hb * hb);
        cmp(e[2].c0, -12.0 * ha.powi(4) - 108.0 * ha.powi(3) * hb - 180.0 * ha * ha * hb * hb - 108.0 * ha * hb.powi(3) - 12.0 * hb.powi(4));
        let e = only(2);
        cmp(e[0].c0, 360.0 * ha * ha * hb.powi(3) * (4.0 * ha + 3.0 * hb));
        cmp(e[1].ca, 72.0 * hb * hb * (8.0 * ha * hb + 2.0 * hb * hb));
        cmp(e[1].cb, 72.0 * hb * hb * (12.0 * ha * ha + 8.0 * ha * hb));
        cmp(e[1].c0, 72.0 * hb * hb * (8.0 * ha.powi(3) * hb + 19.0 * ha * ha * hb * hb + 8.0 * ha * hb.powi(3)));
        cmp(e[2].ca, 72.0 * ha * hb * (9.0 * ha * hb + 6.0 * hb * hb));
        cmp(e[2].cb, 72.0 * ha * hb * (6.0 * ha * ha + 9.0 * ha * hb));
        cmp(e[2].c0, 72.0 * ha * hb * (9.0 * ha.powi(3) * hb + 17.0 * ha * ha * hb * hb + 9.0 * ha * hb.powi(3)));
    }
    rep.add("gr-G1", "numerical", &format!("the Einstein-Lovelock tensors of a spacetime with two scale factors, computed from the pair curvatures K_(4i) = H_i' + H_i^2, K_(ij) = H_i H_j by E^(k)m_m = (-1)^(k+1) k! 2^(k-1) times the sum over the sets of 2k directions other than m of the sum over their divisions into pairs of the product of the pair curvatures: for 3 + 4 directions (the cosmological spacetime of the author's document KOHN_SHAM_GRAVITY) the nine components E^(1), E^(2), E^(3) (time, ordinary space, auxiliary space), each with its coefficients of H_a' and H_b', equal the 21 polynomials printed in that document at four pairs of rates (largest deviation {:.1e})", dev), dev < 1e-12);

    // =====================================================================================
    // 2. the self-consistent evolution of the author's document, reproduced
    // =====================================================================================
    // Fermi sea: the "flat" group of four directions is the auxiliary space (scale factor b), the group of three is ordinary space (a)
    let sea34 = Fermi::with_isotropic_three(4, NU, 32, 64);
    let matter34 = || Matter { fermi: &sea34, flat_is_a: false, c: hartree, kf: kf_sat, zero_mode: false };
    // the conservation law at the point of the author's document: a = 1 + 0.3 x + 0.05 x^2, b = 1/(1 + 0.2 x), x = 0.7
    let (a_of, b_of) = (|x: f64| 1.0 + 0.3 * x + 0.05 * x * x, |x: f64| 1.0 / (1.0 + 0.2 * x));
    let (x, h) = (0.7, 1e-4);
    let m34 = matter34();
    let src = m34.source(a_of(x), b_of(x));
    let deps = (m34.source(a_of(x + h), b_of(x + h)).eps - m34.source(a_of(x - h), b_of(x - h)).eps) / (2.0 * h);
    let (ha, hb) = ((0.3 + 0.1 * x) / a_of(x), -0.2 / (1.0 + 0.2 * x));
    let cons = (deps + 3.0 * ha * (src.eps + src.p_a) + 4.0 * hb * (src.eps + src.p_b)) / deps;
    let cases: [(&str, [f64; 3], [f64; 5]); 2] = [
        ("Einstein", [1.0, 0.0, 0.0], [0.0091437124, 1.5817387, 1.1975586, 0.011877999, 0.0074191078]),
        ("Einstein-Lovelock (alpha_2 = 20, alpha_3 = 500)", [1.0, 20.0, 500.0], [0.014980704, 1.4773515, 1.2760841, 0.010175914, 0.0084887647]),
    ];
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut path_rows: Vec<Vec<f64>> = Vec::new();
    let mut worst_rel: f64 = 0.0;
    let mut worst_con: f64 = 0.0;
    let mut g2 = true;
    for (ic, (name, alpha, want)) in cases.iter().enumerate() {
        let ev = Evolution { geo: geo34, love: Lovelock { alpha: *alpha, lambda_bar: 0.0, kappa: 1000.0 }, matter: Some(matter34()) };
        let Some(hb0) = ev.solve_constraint_for_hb(1.0, 1.0, 0.05, 0.0, 0.2, 0.01) else {
            g2 = false;
            continue;
        };
        let (path, con, done) = integrate(&ev, State { x4: 0.0, a: 1.0, b: 1.0, h_a: 0.05, h_b: hb0 }, 20.0, 8000, 80);
        g2 &= done;
        let end = *path.last().unwrap();
        let got = [hb0, end.a, end.b, end.h_a, end.h_b];
        let names = ["H_b(0)", "a(20)", "b(20)", "H_a(20)", "H_b(20)"];
        for i in 0..5 {
            let rel = (got[i] / want[i] - 1.0).abs();
            worst_rel = worst_rel.max(rel);
            rows.push(vec![name.to_string(), names[i].to_string(), format!("{:.10e}", got[i]), format!("{:.10e}", want[i]), format!("{:.1e}", rel)]);
        }
        worst_con = worst_con.max(con);
        rows.push(vec![name.to_string(), "largest relative violation of the constraint".to_string(), format!("{:.2e}", con), "".to_string(), "".to_string()]);
        for s in &path {
            let src = ev.source(s.a, s.b);
            path_rows.push(vec![ic as f64, s.x4, s.a, s.b, s.h_a, s.h_b, src.eps, src.p_a, src.p_b, src.mass]);
        }
    }
    write_csv_str("gravity_reference.csv",
        &[("content", "the self-consistent evolution of the author's document KOHN_SHAM_GRAVITY (section 16.3), computed again by this program: gravity with three ordinary and four auxiliary space coordinates driven by Kohn-Sham matter (m = 1, G_S = -1e5, G_V = +1e4, comoving Fermi momentum of saturation), kappa = 1000, LambdaBar = 0, H_a(0) = 0.05, a(0) = b(0) = 1, x4 from 0 to 20 (the time is called x0 in that document); classical Runge-Kutta method with 8000 steps (check gr-G2)".to_string()),
          ("columns", "gravity; quantity; here: this program; document: the value printed in the author's document; rel_diff".to_string()),
          ("command", CMD.to_string())],
        &["gravity", "quantity", "here", "document", "rel_diff"], &rows)?;
    write_csv("gravity_reference_path.csv",
        &[("content", "the two evolutions of gravity_reference.csv as functions of the time".to_string()),
          ("columns", "case: 0 = Einstein, 1 = Einstein-Lovelock (alpha_2 = 20, alpha_3 = 500); x4: time; a, b: scale factors of ordinary and auxiliary space; H_a, H_b: their rates; eps: energy density of the matter; p_a, p_b: its pressures; M: mass of the particles in matter".to_string()),
          ("command", CMD.to_string())],
        &["case", "x4", "a", "b", "H_a", "H_b", "eps", "p_a", "p_b", "M"], &path_rows)?;
    rep.add("gr-G2", "numerical", &format!("the self-consistent evolution of the author's document is reproduced: with his data (kappa = 1000, LambdaBar = 0, H_a(0) = 0.05, a(0) = b(0) = 1, Kohn-Sham matter of the saturation density) the constraint gives H_b(0), and the integration to x4 = 20 gives a, b, H_a, H_b; for Einstein gravity and for Einstein-Lovelock gravity with alpha_2 = 20, alpha_3 = 500 the ten numbers agree with the printed ones to {:.1e} (relative); the constraint is preserved to {:.1e}. The conservation law eps' + 3 H_a (eps + p_a) + 4 H_b (eps + p_b) = 0 holds at the test point of the document (a = 1 + 0.3 x + 0.05 x^2, b = 1/(1 + 0.2 x), x = 0.7) with the relative residual {:.1e}", worst_rel, worst_con, cons), g2 && worst_rel < 2e-7 && worst_con < 1e-9 && cons.abs() < 1e-6);
    println!("[{:>5.1} s] reference evolution done", t0.elapsed().as_secs_f64());

    // =====================================================================================
    // 3. the deflating spacetimes
    // =====================================================================================
    let geo = Geometry { n_a: 4, n_b: 3 };
    let g = 0.01; // the rate of deflation g55 = g66 = g77, in units of m
    let tuned = |kappa: f64| Lovelock { alpha: [1.0, 1.0 / (4.0 * g * g), 0.0], lambda_bar: 3.0 * g * g, kappa };
    // G3: vacuum solution, and the linear system at the vacuum point
    let fe = field_equations(&geo, &tuned(0.0), &[0.0, -g]);
    let vac = fe.iter().map(|e| e.c0.abs()).fold(0.0, f64::max) / (g * g);
    let diag = (fe[1].ca + 6.0).abs() + fe[1].cb.abs() + fe[2].ca.abs() + (fe[2].cb - 2.0).abs();
    // without the Gauss-Bonnet term the exactly deflating spacetime needs a source: kappa eps = 3 g^2, kappa p_a = -6 g^2, kappa p_b = -3 g^2
    let ein = field_equations(&geo, &Lovelock { alpha: [1.0, 0.0, 0.0], lambda_bar: 0.0, kappa: 0.0 }, &[0.0, -g]);
    let agent = [ein[0].c0 / (g * g), -ein[1].c0 / (g * g), -ein[2].c0 / (g * g)];
    rep.add("gr-G3", "numerical", &format!("the deflating spacetime in the author's Einstein-Lovelock gravity (four flat coordinates x0..x3 with H_a = 0, three deflating coordinates with equal rates, H_b = -g55): (i) it solves the three field equations without matter for LambdaBar = 3 g55^2 and alpha_2 = 1/(4 g55^2) (residuals {:.1e} g55^2; alpha_3 is arbitrary because E^(3) vanishes), in agreement with the exact checks 44d-G10 and 71d-G10; (ii) at this solution the two equations for the accelerations are -6 H_a' + .. and +2 H_b' + .. (deviation of the matrix of coefficients from diag(-6, 2): {:.1e}): the evolution is well defined; (iii) in Einstein gravity without the Gauss-Bonnet term and without LambdaBar the same spacetime needs a source with kappa eps = {:.1} g55^2, kappa p_a = {:.1} g55^2, kappa p_b = {:.1} g55^2, that is p_a = -2 eps along the flat coordinates and p_b = -eps along the deflating ones. Kohn-Sham matter cannot be this source: its pressure along a coordinate on which the fields do not depend is Pi = (1/2) G_V rho^2 + (M - m)^2/(2 G_S), and Pi = -eps would require e_kin + G_V rho^2 = 0", vac, diag, agent[0], agent[1], agent[2]),
        vac < 1e-10 && diag < 1e-10 && (agent[0] - 3.0).abs() < 1e-10 && (agent[1] + 6.0).abs() < 1e-10 && (agent[2] + 3.0).abs() < 1e-10);

    // matter: (0) 7+1, all momenta (saturated matter of the author's example at b = 1);
    //         (1) fields that do not depend on x5, x6, x7, one kind of particle with eight states and the mean-field
    //             couplings G_S, G_V (7+1: low-energy sector): D = 4, couplings three times the binding threshold for
    //             G_V = 0.1 |G_S|, at its saturation density;
    //         (2) 4+4, covariant form (the definition): the couplings of the Lagrangian g_0 = -g, g_1 = +0.1 g at three
    //             times the binding threshold of this form, saturated state;
    //         (3) 4+4, quantized-sector form (the alternative): the same, at three times the threshold of that form.
    let sea7 = Fermi::with_isotropic_three(4, NU, 32, 64);
    let sea4 = Fermi::isotropic(4, NU, 96);
    let sea4_half = Fermi::isotropic(4, NU / 2.0, 96);
    let c4 = Contact { m: 1.0, gs: -3.0 * 85.65040946294, gv: 0.3 * 85.65040946294 };
    let (kf4, sat4) = saturation(&sea4, &c4, 0.3, 2.0);
    let eps7 = ks_solve(&Fermi::isotropic(7, NU, 96), &hartree, kf_sat, &[1.0; 3]).eps;
    // the two forms of 4+4 for the same couplings of the Lagrangian (module halves)
    let tabs = SelfTables::build(&spacetimes::gamma::Gammas::build());
    let mut g9 = [0.0; 9];
    g9[0] = -1.0;
    g9[1] = 0.1;
    let hf_unit = Hf4::from_tables(&tabs, &g9);
    // (contact couplings of the one kind of particle that the saturated state is, number of states, Fermi momentum, energy density, description of the state, strength g)
    let mut form_matter: Vec<(Contact, bool, f64, f64, String, f64)> = Vec::new();
    let mut g6 = true;
    let mut g6_dev: f64 = 0.0;
    let mut g6_txt: Vec<String> = Vec::new();
    for form in Form::both() {
        let g_thr = binding_threshold(4, form, &hf_unit);
        let h = halves_for(4, form, &hf_unit, 3.0 * g_thr);
        let (_, st) = h.lowest_energy_per_particle(KF_SCAN.0, KF_SCAN.1, KF_SCAN.2).expect("gravity: no saturated state");
        // the saturated state is matter of one kind of particle: one half filled (four states, couplings G_Ssame, G_Vsame)
        // or equal halves (eight states, couplings G_Stot, G_Vtot)
        let (c, half) = if st.pol > 1.0 - 1e-6 {
            (h.polarized_contact(), true)
        } else if st.pol < 1e-6 {
            (h.unpolarized_contact(), false)
        } else {
            g6 = false;
            (h.unpolarized_contact(), false)
        };
        // the same state by the Kohn-Sham routines for one kind of particle
        let ks = ks_solve(if half { &sea4_half } else { &sea4 }, &c, st.kf[0], &[1.0; 3]);
        g6_dev = g6_dev.max((ks.eps / st.eps - 1.0).abs()).max((ks.mass - st.mass[0]).abs()).max((ks.p_flat - st.p_flat).abs() / st.eps).max((ks.pi - st.p_int).abs() / st.eps);
        g6_txt.push(format!("{}: threshold g m^3 = {:.4}; at three times that strength the saturated state has {} (Fermi momentum of the filled states {:.5} m, mass {:.5} m, E/N - m = {:+.5} m, eps = {:.6e} m^5, interaction pressure Pi = {:+.6e} m^5); as matter of one kind: {} states, G_S = {:.4}, G_V = {:.4}", form.name(), g_thr, if half { "one half filled" } else { "equal halves" }, st.kf[0], st.mass[0], st.e_per_particle - 1.0, st.eps, st.p_int, if half { 4 } else { 8 }, c.gs, c.gv));
        form_matter.push((c, half, st.kf[0], st.eps, format!("g = {:.4}, {}", 3.0 * g_thr, if half { "one half filled" } else { "equal halves" }), 3.0 * g_thr));
    }
    rep.add("gr-G6", "numerical", &format!("the matter of the two forms of the 4+4 model as a source of gravity (fields that do not depend on x5, x6, x7, D = 4; couplings of the Lagrangian g_0 = -g, g_1 = +0.1 g; state of lowest energy per particle over all densities and polarizations, module halves). {}. The same states computed by the Kohn-Sham routines for matter of one kind of particle (module kohn_sham, other integrals) agree in mass, energy density, pressure and interaction pressure to {:.1e}. In an evolution without collisions every particle keeps its state, so that a half that is empty stays empty: the source is then matter of one kind of particle throughout", g6_txt.join(". "), g6_dev), g6 && g6_dev < 1e-9);
    // (name, energy density at the start, zero mode, Fermi sea, couplings, Fermi momentum)
    let matters: Vec<(String, f64, bool, &Fermi, Contact, f64)> = vec![
        ("7+1, momenta along all seven space coordinates".to_string(), eps7, false, &sea7, hartree, kf_sat),
        ("fields that do not depend on x5, x6, x7: one kind of particle with eight states and the mean-field couplings G_S, G_V (7+1: low-energy sector)".to_string(), sat4.eps, true, &sea4, c4, kf4),
        (format!("4+4, covariant form (the definition), fields that do not depend on x5, x6, x7: {}", form_matter[0].4), form_matter[0].3, true, if form_matter[0].1 { &sea4_half } else { &sea4 }, form_matter[0].0, form_matter[0].2),
        (format!("4+4, quantized-sector form (the alternative), fields that do not depend on x5, x6, x7: {}", form_matter[1].4), form_matter[1].3, true, if form_matter[1].1 { &sea4_half } else { &sea4 }, form_matter[1].0, form_matter[1].2),
    ];
    let short = ["7+1, all momenta", "sector without momenta along x5, x6, x7, mean-field couplings", "4+4 covariant form", "4+4 quantized-sector form"];
    let n_runs = matters.len() * 4;
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut sum_rows: Vec<Vec<String>> = Vec::new();
    let mut g4 = true;
    let mut worst_con4: f64 = 0.0;
    let mut txt: Vec<String> = Vec::new();
    let mut n_runaway = 0usize;
    // gravity models: 0 = tuned Einstein-Lovelock (the deflating spacetime is its vacuum), 1 = Einstein gravity with matter alone
    for (im, (mname, eps0, zero_mode, fermi, contact, kf_m)) in matters.iter().enumerate() {
        for (ig, gname) in ["Einstein-Lovelock gravity with LambdaBar = 3 g55^2, alpha_2 = 1/(4 g55^2)", "Einstein gravity, matter as the only source"].iter().enumerate() {
            for eta in [1e-3, 1e-1] {
                // loading eta = kappa eps(0)/(3 g^2)
                let kappa = eta * 3.0 * g * g / eps0;
                let love = if ig == 0 { tuned(kappa) } else { Lovelock { alpha: [1.0, 0.0, 0.0], lambda_bar: 0.0, kappa } };
                let ev = Evolution { geo, love, matter: Some(Matter { fermi: *fermi, flat_is_a: true, c: *contact, kf: *kf_m, zero_mode: *zero_mode }) };
                // initial data: a = b = 1, H_a = 0, H_b from the constraint (the root nearest to -g55; in Einstein gravity -sqrt(kappa eps/3))
                let guess = if ig == 0 { -g } else { -(kappa * eps0 / 3.0).sqrt() };
                let Some(hb0) = ev.solve_constraint_for_hb(1.0, 1.0, 0.0, -5.0 * g, 0.0, guess) else {
                    g4 = false;
                    continue;
                };
                // evolve for six e-foldings of the clock of the undisturbed deflation, tau_ref = 3 g55 x4
                let x_end = 6.0 / (3.0 * g);
                let (path, con, why) = integrate_adaptive(&ev, State { x4: 0.0, a: 1.0, b: 1.0, h_a: 0.0, h_b: hb0 }, x_end, g, 10);
                worst_con4 = worst_con4.max(con);
                for s in &path {
                    let src = ev.source(s.a, s.b);
                    rows.push(vec![im as f64, ig as f64, eta, s.x4, 3.0 * g * s.x4, s.a, s.b, s.h_a / g, s.h_b / g, -3.0 * s.b.ln(), src.eps / eps0, src.p_a / eps0, src.p_b / eps0, src.mass, love.kappa * src.eps / (3.0 * g * g)]);
                }
                let end = *path.last().unwrap();
                sum_rows.push(vec![mname.to_string(), gname.to_string(), format!("{:e}", eta), format!("{:.6}", hb0 / g), why.to_string(), format!("{:.4}", 3.0 * g * end.x4), format!("{:.6}", end.a), format!("{:.6e}", end.b), format!("{:.6}", end.h_a / g), format!("{:.6}", end.h_b / g), format!("{:.2e}", con)]);
                txt.push(format!("[{}; {}; loading {:e}] H_b(0) = {:.5} g55; {} tau_ref = {:.2}: a = {:.4}, b = {:.3e}, H_a = {:+.4} g55, H_b = {:+.4} g55", short[im], if ig == 0 { "tuned Einstein-Lovelock" } else { "Einstein" }, eta, hb0 / g, if why == "end" { "at" } else { "the rates run away; at" }, 3.0 * g * end.x4, end.a, end.b, end.h_a / g, end.h_b / g));
                n_runaway += usize::from(why == "runaway");
                g4 &= why != "degenerate";
            }
        }
        println!("[{:>5.1} s] deflating spacetime, matter {} done", t0.elapsed().as_secs_f64(), im);
    }
    write_csv("gravity_deflating.csv",
        &[("content", format!("the deflating spacetime with Kohn-Sham matter as a source of gravity: evolution of the scale factor a of the four flat coordinates x0..x3 and of the scale factor b of the three deflating coordinates (equal rates), from the initial data a = b = 1, H_a = 0 and H_b given by the constraint; rate of the undisturbed deflation g55 = {} m. matter 0: 7+1, momenta along all seven space coordinates, saturated matter of the author's example (m = 1, G_S = -1e5, G_V = +1e4); matter 1: fields that do not depend on x5, x6, x7, one kind of particle with eight states (D = 4; G_S = {:.3}, G_V = {:.3}, Fermi momentum {:.5} of saturation); matter 2: the 4+4 model in its covariant form (the definition), couplings of the Lagrangian g_0 = -g, g_1 = +0.1 g at three times the binding threshold of this form, saturated state ({}); matter 3: the same for the quantized-sector form (the alternative; {}). gravity 0: Einstein-Lovelock with LambdaBar = 3 g55^2 and alpha_2 = 1/(4 g55^2), for which the deflating spacetime without matter is an exact solution; gravity 1: Einstein gravity with the matter as the only source. loading = kappa eps(0)/(3 g55^2)", g, c4.gs, c4.gv, kf4, form_matter[0].4, form_matter[1].4)),
          ("columns", "matter; gravity; loading; x4 (units 1/m); tau_ref = 3 g55 x4: clock of the undisturbed deflation; a; b; H_a/g55; H_b/g55; tau_d = -3 ln b: the deflation clock actually reached; eps/eps(0); p_a/eps(0); p_b/eps(0); M: mass of the particles in matter; kappa eps/(3 g55^2)".to_string()),
          ("command", CMD.to_string())],
        &["matter", "gravity", "loading", "x4", "tau_ref", "a", "b", "Ha_over_g", "Hb_over_g", "tau_d", "eps_rel", "pa_rel", "pb_rel", "M", "kappa_eps"], &rows)?;
    write_csv_str("gravity_deflating_summary.csv",
        &[("content", "summary of the evolutions of gravity_deflating.csv: initial rate from the constraint and the state at the end of the integration (six e-foldings of the clock of the undisturbed deflation, or the point at which the integration stopped)".to_string()),
          ("columns", "matter; gravity; loading = kappa eps(0)/(3 g55^2); Hb0_over_g: initial H_b/g55; stop: `end` if the integration reached tau_ref = 6, `runaway` if a rate exceeded 50 g55 before; tau_ref_end; a_end; b_end; Ha_end_over_g; Hb_end_over_g; constraint: largest relative violation of the constraint".to_string()),
          ("command", CMD.to_string())],
        &["matter", "gravity", "loading", "Hb0_over_g", "stop", "tau_ref_end", "a_end", "b_end", "Ha_end_over_g", "Hb_end_over_g", "constraint"], &sum_rows)?;
    rep.add("gr-G4", "numerical", &format!("the deflating spacetime with Kohn-Sham matter as a source (rate g55 = {} m; initial data a = b = 1, H_a = 0, H_b from the constraint; loading = kappa eps(0)/(3 g55^2)): matter makes the three coordinates deflate FASTER than exponentially and makes the four flat coordinates EXPAND; in {} of the {} evolutions the rates exceed 50 g55 before the clock of the undisturbed deflation reads 6 (they grow without bound in a finite time). The constraint is preserved along every evolution to {:.1e} (relative). Results: {}", g, n_runaway, n_runs, worst_con4, txt.join(" | ")), g4 && worst_con4 < 1e-7);

    // G5: is the deflating vacuum solution itself stable?  Perturb it without matter.
    let ev0 = Evolution { geo, love: tuned(0.0), matter: None };
    let mut vac_txt: Vec<String> = Vec::new();
    let mut vac_rows: Vec<Vec<f64>> = Vec::new();
    let mut g5 = true;
    let mut worst_con5: f64 = 0.0;
    let mut n_away = 0usize;
    // (a) slower deflation: is there initial data at all?
    // (the constraint is a polynomial of degree four in H_a; for H_b = -g55 its roots are 0 (twice) and (6 +- sqrt(20)) g55 = 1.53 g55, 10.5 g55)
    let none = ev0.solve_constraint_for_ha(1.0, 1.0, -0.999 * g, -g, g, 0.0);
    let far = ev0.solve_constraint_for_ha(1.0, 1.0, -g, 1.2 * g, 2.0 * g, 1.5 * g).unwrap_or(f64::NAN);
    g5 &= (far / g - (6.0 - 20f64.sqrt())).abs() < 1e-9;
    g5 &= none.is_none();
    // (b) faster deflation by 0.1 per cent: two roots H_a = +-g sqrt(delta/8) to lowest order
    for (iv, sign) in [1.0, -1.0].iter().enumerate() {
        let hb0 = -1.001 * g;
        let lo_hi = if *sign > 0.0 { (1e-9 * g, 2.0 * g) } else { (-2.0 * g, -1e-9 * g) };
        let Some(ha0) = ev0.solve_constraint_for_ha(1.0, 1.0, hb0, lo_hi.0, lo_hi.1, sign * g * (0.001f64 / 8.0).sqrt()) else {
            g5 = false;
            continue;
        };
        g5 &= (ha0 / (sign * g * (0.001f64 / 8.0).sqrt()) - 1.0).abs() < 0.04;
        n_away += usize::from(end_faster(&ev0, ha0, hb0, g));
        let (path, con, why) = integrate_adaptive(&ev0, State { x4: 0.0, a: 1.0, b: 1.0, h_a: ha0, h_b: hb0 }, 6.0 / (3.0 * g), g, 10);
        worst_con5 = worst_con5.max(con);
        for s in &path {
            vac_rows.push(vec![iv as f64, s.x4, 3.0 * g * s.x4, s.a, s.b, s.h_a / g, s.h_b / g]);
        }
        let end = *path.last().unwrap();
        vac_txt.push(format!("H_a(0) = {:+.5} g55: {} tau_ref = {:.2}, H_a = {:+.5} g55, H_b = {:+.5} g55", ha0 / g, if why == "end" { "at" } else { "the rates run away; at" }, 3.0 * g * end.x4, end.h_a / g, end.h_b / g));
    }
    write_csv("gravity_vacuum_perturbation.csv",
        &[("content", format!("the deflating vacuum solution of the tuned Einstein-Lovelock gravity (LambdaBar = 3 g55^2, alpha_2 = 1/(4 g55^2), g55 = {}), perturbed without matter: initial rate H_b = -1.001 g55 and H_a from the constraint (two roots of opposite sign)", g)),
          ("columns", "case: 0 = positive root H_a(0), 1 = negative root; x4; tau_ref = 3 g55 x4; a; b; H_a/g55; H_b/g55".to_string()),
          ("command", CMD.to_string())],
        &["case", "x4", "tau_ref", "a", "b", "Ha_over_g", "Hb_over_g"], &vac_rows)?;
    rep.add("gr-G5", "numerical", &format!("the deflating vacuum solution of the tuned Einstein-Lovelock gravity, perturbed without matter. Near the solution the constraint reads -6 g55 dH_b - 48 H_a^2 = kappa eps (to lowest order; dH_b = H_b + g55): (a) for a deflation that is SLOWER than g55 there are no initial data without matter near the solution (no root H_a between -g55 and +g55 for H_b = -0.999 g55; the constraint is a polynomial of degree four in H_a whose roots for H_b = -g55 are 0, 0 and (6 +- sqrt(20)) g55, the two last ones far from the deflating solution); (b) for H_b = -1.001 g55 the constraint has the two roots H_a = +-g55 sqrt(0.001/8) = +-0.01118 g55 to lowest order (found to 4 per cent), and in both cases the deflation does not return to the rate g55 but becomes faster and faster: {}. The exactly exponential deflation is therefore an UNSTABLE solution of the gravity in which it is a vacuum solution: it lies on the edge of the allowed initial data, and every allowed perturbation, with or without matter, grows. Constraint preserved to {:.1e}", vac_txt.join("; "), worst_con5), g5 && worst_con5 < 1e-7 && n_away == 2);

    let (ok, n) = rep.count();
    let all = rep.all_pass();
    write_report(&rep, "gravity_checks.txt", "gravity_checks.csv",
        "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: checks of the Einstein-Lovelock equations with Kohn-Sham matter (gr-..)", CMD)?;
    for c in &rep.checks {
        println!("[{}] {:<8} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\ngravity: {} of {} checks passed", ok, n);
    Ok(all)
}
