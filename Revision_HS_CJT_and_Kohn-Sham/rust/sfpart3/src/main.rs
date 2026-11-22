//! sf_part3 -- part III of superfluid.md (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes): the effective
//! Lagrangians L^CJT, L^HS, L^KS of the field of the superfluid boson singlet state, their potential parts (the Mexican
//! hat), their kinetic parts, the minima and the breaking of the symmetry, and the expansions SFBSv^X about the vacuum.
//!
//! Build and run (from the folder rust/, after sf_part2, whose data file part2_modes.csv is read for the comparison of
//! the masses):
//!     cargo build --release
//!     ./target/release/sf_part3
//! Output in data/: hck_part3_checks.txt and the data files part3_*.csv (among them part3_landau.csv and
//! part3_landau_residuals.csv, the accuracy of the Landau form, module landau; part3_ginzburg.csv and
//! part3_ginzburg_regions.csv, the gradient coefficient at T_c and the Ginzburg number, module ginzburg; part3_higgs.csv,
//! the numbers of the superfluid as the Higgs sector of SU(2)_v x U(1)^F_LL, module higgs).  Run time about fifteen
//! minutes.
mod effpot;
mod ginzburg;
mod higgs;
mod kinetic;
mod landau;
mod thermal;

use sfcore::cmat::CMat;
use sfcore::dirac::k_scalar;
use sfcore::methods::{grid, terms, Method, Potential, M_LL};
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::uniform::{DirRule, Grid, Model};
use std::time::Instant;

const GV: f64 = 0.7;
const GF: f64 = 0.45;

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new("sf_part3: checks of part III of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes)");
    let mut vals: Vec<(String, f64)> = vec![];
    let v2 = Potential { gs: 2.8, gv: 0.8, g: 25.2 };
    // the collective modes of part II (method, omega, multiplicity)
    let modes: Vec<(String, f64, i64)> = std::fs::read_to_string(data.join("part2_modes.csv")).expect("run sf_part2 first")
        .lines().skip(1).map(|l| { let v: Vec<&str> = l.split(',').collect(); (v[0].to_string(), v[4].parse().unwrap(), v[5].parse().unwrap()) }).collect();
    let mut sf_rows = vec![];
    let mut landau_rows: Vec<Vec<String>> = vec![];
    let mut th_rows: Vec<Vec<String>> = vec![];
    let mut th_land: Vec<Vec<String>> = vec![];
    let mut th_curves: Vec<Vec<String>> = vec![];
    let mut resid_rows: Vec<Vec<String>> = vec![];
    let mut gz_rows: Vec<Vec<String>> = vec![];
    let mut gz_regions: Vec<Vec<String>> = vec![];
    let mut higgs_rows: Vec<Vec<String>> = vec![];
    landau::check_qp_energies(&mut rep);
    let mut curves_out = vec![];
    for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Ks, Method::Cjt] {
        let tag = m.short();
        let mut model = Model::new(M_LL, 0.0, terms(&m, &v2), grid(24));
        let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let phi0 = k_scalar(3).mul(&sol.p_self).trace().scale(1.0 / 8.0).abs();
        // the phase of the ground state is that of the start (real); the field is Phi0 e3
        let p0 = k_scalar(3).dagger().scale_r(phi0);
        let cv = effpot::curve(&mut rep, &tag, &model, &sol.n_self, phi0, sol.energy, 60);
        // the accuracy of the Landau form (12.2) (Table 12.2, Figure 9)
        let acc = landau::accuracy(&mut rep, &tag, &model, &cv, phi0);
        let st = kinetic::stiffness(&mut model, &sol.n_self, &p0, sol.energy);
        // the free energy at finite temperature (Section 12.4): T_c, Phi0(T), F(Phi; T) and its Landau form
        let lam2: f64 = std::fs::read_to_string(data.join("part2_values.csv")).expect("run sf_part2 first").lines()
            .find(|l| l.starts_with(&format!("p2_lamP_V2_{},", tag))).map(|l| l.split(',').nth(1).unwrap().parse().unwrap()).unwrap();
        let th = thermal::run(&mut rep, &tag, &m, &v2, &model, &sol.n_self, &sol.p_self, phi0, sol.energy, lam2, cv.landau.3 / sol.energy.abs());
        println!("{:<20} T_c {:.8} lambda^P(0) {:.8} g^P {:.6} g^S {:.6} ({:.0} s)", m.name(), th.tc, th.lam0, th.g_p, th.g_s, t0.elapsed().as_secs_f64());
        for (k, v) in [("Tc", th.tc), ("lam0", th.lam0), ("gP", th.g_p), ("gS", th.g_s)] {
            vals.push((format!("p3_th_{}_{}", k, tag), v));
        }
        for p in &th.points {
            th_rows.push(vec![tag.clone(), f(p.tau), f(p.t), f(p.phi0), f(p.mass), f(p.gap), f(p.mass_n), f(p.radius), f(p.phi0 / p.radius), f(p.free), f(p.dentropy),
                              f(p.gapeq), f(p.a), f(p.b), f(p.phi0_landau), format!("{}", p.iterations)]);
        }
        for c in &th.curves {
            th_land.push(vec![tag.clone(), f(c.tau), f(c.res6), f(c.res4)]);
            for r in &c.rows {
                th_curves.push(vec![tag.clone(), f(c.tau), f(r.0), f(r.1), f(r.2)]);
            }
        }
        // ---- the gradient coefficient at T_c and the Ginzburg number (Section 12.5)
        let gz = ginzburg::run(&mut model, th.g_p, th.tc);
        {
            let pi2 = std::f64::consts::PI * std::f64::consts::PI;
            let grid0 = model.grid.clone();
            let ident = CMat::identity(2);
            // P3-G1: the linear response a(0, T) equals the Taylor coefficient of the nonlinear free energy at 0.9 T_c
            let p09 = th.points.iter().find(|p| (p.tau - 0.9).abs() < 1e-12).unwrap();
            let (n09, _) = ginzburg::normal_self_energy(&model, p09.t);
            let a09 = ginzburg::a_twist(&model, &grid0, &n09, p09.t, 0.0, &ident, true).a;
            rep.add(&format!("P3-G1 {}", tag), &format!("{}: at 0.9 T_c the coefficient a(0) of |Phi|^2 from the linear response ({:.10e}) equals the Taylor coefficient a^Tay of the nonlinear free energy of Section 12.4 ({:.10e}) (relative difference)", tag, a09, p09.a),
                    (a09 - p09.a).abs() / p09.a.abs(), 1e-7);
            // P3-G2: a(0, T) = (1 - lambda^P(T)) / (lambda^P(T) g^P) with lambda^P(T) of (12.9)
            let mut worst = 0.0f64;
            for t in [0.9 * th.tc, 0.998 * th.tc, 1.002 * th.tc] {
                let (nn, _) = ginzburg::normal_self_energy(&model, t);
                let mn = ginzburg::dirac_mass(&nn);
                let lam = th.g_p * thermal::i1(&model, mn * mn, t) / pi2;
                let a = ginzburg::a_twist(&model, &grid0, &nn, t, 0.0, &ident, true).a;
                let cl = (1.0 - lam) / (lam * th.g_p);
                worst = worst.max((a - cl).abs() / cl.abs());
            }
            rep.add(&format!("P3-G2 {}", tag), &format!("{}: a(0, T) = (1 - lambda^P(T)) / (lambda^P(T) g^P) at T/T_c = 0.9, 0.998, 1.002 (largest relative difference)", tag), worst, 1e-9);
            rep.add(&format!("P3-G3 {}", tag), &format!("{}: a(0, T_c) vanishes to the accuracy of T_c (|a(0, T_c)| / alpha; alpha = {:.10})", tag, gz.alpha), gz.a_tc.abs() / gz.alpha, 1e-9);
            rep.add(&format!("P3-G4 {}", tag), &format!("{}: at k = 0 no other pair channel is induced (norm of the self-consistent components of P other than (K^iota)^dag; at k = {} it is {:.2e})", tag, ginzburg::K1, gz.perp_k), gz.perp0, 1e-12);
            rep.add(&format!("P3-G5 {}", tag), &format!("{}: the isospin twist gives the gradient coefficient of the phase twist, c = {:.10} (isospin invariance; relative difference)", tag, gz.c), (gz.c_iso / gz.c - 1.0).abs(), 1e-8);
            rep.add_bool(&format!("P3-G6 {}", tag), &format!("{}: c > 0 and the extrapolation in k converges: c(k = 0.01) = {:.10}, c(k = 0.02) = {:.10}, extrapolated {:.10}", tag, gz.c_h, gz.c_2h, gz.c),
                         gz.c > 0.0 && (gz.c_2h - gz.c_h).abs() / gz.c < 1e-3, "");
            if tag == "HS_0_0" {
                rep.add("P3-G7 HS_0_0", &format!("HS_0_0 (no normal self-energy): the twisted states of the nonlinear engine give the gradient coefficient of the linear response with the pair self-energy along (K^3)^dag ({:.10} and {:.10}; relative difference)", gz.c_nonlinear, gz.c_fixed),
                        (gz.c_nonlinear / gz.c_fixed - 1.0).abs(), 1e-5);
            }
            // the two regularizations: cutoff on the relative momentum (sphere) and on the momenta of the charges (lens)
            let (n_c, _) = ginzburg::normal_self_energy(&model, th.tc);
            let (s0, l0) = ginzburg::a_regions(&model, &n_c, th.tc, 0.0);
            let mut sl = vec![];
            for k in [0.005, 0.01, 0.02, 0.04] {
                let (sa, la) = ginzburg::a_regions(&model, &n_c, th.tc, k);
                gz_regions.push(vec![tag.clone(), f(k), f(sa - s0), f(la - l0), f((sa - s0) / (k * k)), f((la - l0) / k)]);
                sl.push((sa - s0, la - l0));
            }
            let (rs, rl) = (sl[1].0 / sl[0].0, sl[1].1 / sl[0].1);
            rep.add_bool(&format!("P3-G9 {}", tag), &format!("{}: with the cutoff on the momenta of the two charges a(k) - a(0) is linear in |k| (ratio {:.5} for k = 0.01 and 0.005, 2 for |k|), with the cutoff on the relative momentum quadratic (ratio {:.6}, 4 for k^2)", tag, rl, rs),
                         (rl / 2.0 - 1.0).abs() < 0.02 && (rs / 4.0 - 1.0).abs() < 1e-3, "");
            // P3-G10: alpha from the central differences of the linear response equals the closed form of (12.9)
            let (al_c, lam_c, mn_c) = ginzburg::alpha_closed(&model, th.g_s, th.g_p, th.tc);
            rep.add(&format!("P3-G10 {}", tag), &format!("{}: alpha = T_c da/dT = {:.10} from central differences of the linear response a(0, T) (Richardson) equals the closed form -T_c lambda'(T_c)/(lambda^2 g^P) = {:.10} (lambda^P(T_c) = {:.12}, Dirac mass of the normal state {:.10} from its scalar self-consistency; relative difference)", tag, gz.alpha, al_c, lam_c, mn_c),
                    (gz.alpha - al_c).abs() / al_c, 1e-9);
            // P3-G11: the two forms of the Ginzburg number (12.13)
            let dc = gz.alpha * gz.alpha / (2.0 * gz.b * gz.tc);
            let gi2 = ginzburg::N_COMP * ginzburg::N_COMP / (256.0 * pi2 * (dc * gz.xi0.powi(3)).powi(2));
            rep.add(&format!("P3-G11 {}", tag), &format!("{}: the two forms of the Ginzburg number (12.13) agree: n^2 b^2 T_c^2/(64 pi^2 alpha c^3) = {:.10} and n^2/(256 pi^2 (Delta C xi0^3)^2) = {:.10}, with Delta C = alpha^2/(2 b T_c) = {:.6e} and xi0 = (c/alpha)^(1/2) = {:.8} (relative difference)", tag, gz.gi, gi2, dc, gz.xi0),
                    (gi2 / gz.gi - 1.0).abs(), 1e-12);
            if tag == "CJT" {
                // P3-G12: the quartic coefficient b at T_c does not depend on the applied fields of its interpolation
                let (_, b2) = ginzburg::taylor_nonlinear_fields(&mut model, th.tc, None, &[0.015, 0.025, 0.035, 0.045]);
                rep.add("P3-G12 CJT", &format!("CJT at T_c: the quartic coefficient b of the free energy at the origin (cubic interpolation of F/Phi^2 in Phi^2 through four states of the nonlinear engine) is positive and does not depend on the applied fields: b = {:.10} from the fields 0.01 to 0.04 and {:.10} from the fields 0.015 to 0.045 (relative difference)", gz.b, b2),
                        if gz.b > 0.0 { (b2 / gz.b - 1.0).abs() } else { f64::INFINITY }, 1e-6);
                let gp8 = Grid::new(24, 1.0, DirRule::Product(8, 8));
                let cp = ginzburg::gradient(&model, &gp8, &n_c, th.tc, &ident, 2.0, ginzburg::K1, true).0;
                let m40 = Model::new(M_LL, 0.0, terms(&m, &v2), grid(40));
                let (n40, _) = ginzburg::normal_self_energy(&m40, th.tc);
                let c40 = ginzburg::gradient(&m40, &m40.grid.clone(), &n40, th.tc, &ident, 2.0, ginzburg::K1, true).0;
                rep.add("P3-G8", &format!("CJT: the gradient coefficient does not depend on the quadrature: six directions and 24 radial nodes {:.12}, 64 directions {:.12}, 40 radial nodes {:.12} (largest relative difference)", gz.c, cp, c40),
                        ((cp - gz.c).abs()).max((c40 - gz.c).abs()) / gz.c, 1e-8);
            }
            println!("{:<20} Ginzburg: alpha {:.8} b {:.8} c {:.8} (fixed {:.8}, T3 {:.6}, twisted states {:.6}) Gi {:.4} xi0 {:.4} ({:.0} s)", m.name(), gz.alpha, gz.b, gz.c, gz.c_fixed, gz.c_t3, gz.c_nonlinear, gz.gi, gz.xi0, t0.elapsed().as_secs_f64());
            gz_rows.push(vec![tag.clone(), f(gz.tc), f(gz.mass_n), f(gz.alpha), f(gz.b), f(gz.c), f(gz.c_iso), f(gz.c_fixed), f(gz.c_t3), f(gz.c_nonlinear), f(gz.c_h), f(gz.c_2h),
                              f(gz.perp0), f(gz.perp_k), f(gz.a_tc), f(gz.gi), f(gz.width), f(gz.xi0), f(gz.phi_g)]);
            for (k, v) in [("alpha", gz.alpha), ("b", gz.b), ("c", gz.c), ("c_fixed", gz.c_fixed), ("c_T3", gz.c_t3), ("c_twisted", gz.c_nonlinear), ("Gi", gz.gi), ("xi0", gz.xi0), ("PhiG", gz.phi_g), ("width", gz.width)] {
                vals.push((format!("p3_gz_{}_{}", k, tag), v));
            }
        }
        rep.add(&format!("P3-6 {}", tag), &format!("{}: the unbroken T^3 has no temporal stiffness in the effective Lagrangian (chi_33 = {:.1e})", tag, st.chi33), st.chi33.abs(), 1e-10);
        // the coefficients of the kinetic part and the masses of SFBSv
        let z = st.chi11 / (2.0 * phi0 * phi0);
        let y = st.chiff / (8.0 * phi0 * phi0) - z;
        let mh = (cv.v2_amp / (2.0 * (z + y))).sqrt();
        let mnu = (cv.v2_nu / (2.0 * z)).sqrt();
        let rpa_h = modes.iter().filter(|r| r.0 == tag && (r.2 == 1 || r.2 == 3)).map(|r| r.1).next().unwrap_or(f64::NAN);
        let rpa_nu = modes.iter().filter(|r| r.0 == tag && (r.2 == 2 || r.2 == 3)).map(|r| r.1).next().unwrap_or(f64::NAN);
        println!("{:<20} Phi0 {:.8} E0 {:.6e} V''amp {:.6e} V''nu {:.6e} Z {:.6e} Y {:.6e} M_H {:.6} (RPA {:.6}) M_NU {:.6} (RPA {:.6}) | f^2 F t/s {:.4e}/{:.4e} I t/s {:.4e}/{:.4e} T3 s {:.3e} ({:.0}s)",
                 m.name(), phi0, sol.energy, cv.v2_amp, cv.v2_nu, z, y, mh, rpa_h, mnu, rpa_nu, st.ff_t, st.ff_s, st.fi_t, st.fi_s, st.f3_s, t0.elapsed().as_secs_f64());
        let (a, b, c, res) = cv.landau;
        for (k, v) in [("Phi0", phi0), ("E0", sol.energy), ("V2amp", cv.v2_amp), ("V2nu", cv.v2_nu), ("Z", z), ("Y", y), ("MH", mh), ("MNU", mnu), ("MH_RPA", rpa_h), ("MNU_RPA", rpa_nu),
                       ("chi11", st.chi11), ("chiFF", st.chiff), ("fF2_t", st.ff_t), ("fF2_s", st.ff_s), ("fI2_t", st.fi_t), ("fI2_s", st.fi_s), ("f32_s", st.f3_s),
                       ("MW", GV * st.chi11.sqrt()), ("MZ", GF * st.chiff.sqrt()), ("landau_a", a), ("landau_b", b), ("landau_c", c), ("landau_res", res), ("b2", -cv.v2_nu / (8.0 * phi0 * phi0)), ("slope", cv.slope_at_min)] {
            vals.push((format!("p3_{}_{}", k, tag), v));
        }
        vals.push((format!("p3_landau_phifit_{}", tag), acc.phi_fit));
        vals.push((format!("p3_landau_depthfit_{}", tag), acc.depth_fit));
        vals.push((format!("p3_landau_curvfit_{}", tag), acc.curv_fit));
        vals.push((format!("p3_landau_signs_{}", tag), acc.sign_changes as f64));
        for &(k, r) in &acc.res_by_k {
            vals.push((format!("p3_landau_resK{}_{}", k, tag), r));
        }
        vals.push((format!("p3_landau_cK4_{}", tag), acc.c_k4));
        for (i, &(top, ra, rb, rc, rr)) in acc.by_range.iter().enumerate() {
            for (k, v) in [("top", top), ("a", ra), ("b", rb), ("c", rc), ("res", rr)] {
                vals.push((format!("p3_landau_rng{}_{}_{}", i + 1, k, tag), v));
            }
        }
        vals.push((format!("p3_a_taylor_{}", tag), acc.a_taylor));
        vals.push((format!("p3_a_taylor_chk_{}", tag), acc.a_taylor_check));
        let mut lrow = vec![tag.clone(), f(phi0), f(sol.energy), f(cv.v2_amp), f(acc.phi_fit), f(acc.depth_fit), f(acc.curv_fit), f(a), f(acc.a_taylor)];
        for &(_, r) in &acc.res_by_k {
            lrow.push(f(r));
        }
        lrow.push(format!("{}", acc.sign_changes));
        lrow.push(f(acc.c_k4));
        for &(top, _, _, rc, rr) in &acc.by_range {
            lrow.push(f(top));
            lrow.push(f(rc));
            lrow.push(f(rr));
        }
        landau_rows.push(lrow);
        for &(x, r2, r3, r4) in &acc.residuals {
            resid_rows.push(vec![tag.clone(), f(x), f(x / phi0), f(r2 / sol.energy.abs()), f(r3 / sol.energy.abs()), f(r4 / sol.energy.abs())]);
        }
        // ---- the superfluid as a Higgs sector (Section 14.6)
        {
            let gap0 = th.points[0].gap;
            let hg = higgs::run(phi0, z, y, rpa_h, rpa_nu, GV, GF, gap0, th.tc);
            rep.add_bool(&format!("P3-H1 {}", tag), &format!("{}: the amplitude mode lies below the threshold of two quasi-particles, M_H = {:.6} < 2 E_gap = {:.6} (M_H / 2 E_gap = {:.4})", tag, rpa_h, 2.0 * gap0, hg.mh_over_2gap), rpa_h < 2.0 * gap0, "");
            rep.add(&format!("P3-H2 {}", tag), &format!("{}: the couplings of h to (d theta_0)^2 and (d theta_1)^2 from finite differences of (12.4) are f_F = {:.8} and f_I^2/f_F = {:.8} (largest relative difference)", tag, hg.f_f, hg.f_i * hg.f_i / hg.f_f),
                    ((hg.g_phase / hg.f_f - 1.0).abs()).max((hg.g_iso / (hg.f_i * hg.f_i / hg.f_f) - 1.0).abs()), 1e-8);
            // P3-H3: the numbers of Table 14.2 recomputed here directly from the susceptibilities chi_11, chi_FF, the masses
            // of the collective modes, the gap and T_c (v = 2 f_I, f_I = chi_11^(1/2), f_F = chi_FF^(1/2)/2, M_W = g^v f_I,
            // M_Z = 2 g^F f_F, lambda = M_H^2/(2 v^2), Gamma/M_H = 3 M_H^2/(32 pi f_F^2)), independently of the module higgs
            {
                let v = 2.0 * st.chi11.sqrt();
                let (f_i, f_f) = (st.chi11.sqrt(), 0.5 * st.chiff.sqrt());
                let (mw, mz) = (GV * f_i, 2.0 * GF * f_f);
                let mine = [v, f_f, f_i, rpa_h / v, rpa_h * rpa_h / (2.0 * v * v), rpa_h / mw, rpa_h / (2.0 * gap0), rpa_nu / rpa_h, f_f / f_i, mw / mz,
                            3.0 * rpa_h * rpa_h / (32.0 * std::f64::consts::PI * f_f * f_f), th.tc / v];
                let theirs = [hg.v_sf, hg.f_f, hg.f_i, hg.mh_over_v, hg.lambda, hg.mh_over_mw, hg.mh_over_2gap, hg.mnu_over_mh, hg.ff_over_fi, hg.mw_over_mz,
                              hg.width_over_mh, hg.tc_over_v];
                let dev = mine.iter().zip(&theirs).map(|(a, b)| (a - b).abs() / b.abs()).fold(0.0, f64::max);
                rep.add(&format!("P3-H3 {}", tag), &format!("{}: the numbers of Table 14.2 recomputed from chi_11, chi_FF, M_H, M_NU, E_gap and T_c by (14.9), (14.10) (v_SF = {:.8}, M_H/v_SF = {:.6}, lambda_SF = {:.5}, Gamma/M_H = {:.5}) = the module higgs (largest relative difference)", tag, v, rpa_h / v, mine[4], mine[10]),
                        dev, 1e-12);
            }
            println!("{:<20} Higgs: v_SF {:.6} M_H/v {:.4} lambda {:.3} M_H/M_W {:.3} M_H/2E_gap {:.4} M_NU/M_H {:.4} f_F/f_I {:.4} Gamma/M_H {:.3}", m.name(), hg.v_sf, hg.mh_over_v, hg.lambda, hg.mh_over_mw, hg.mh_over_2gap, hg.mnu_over_mh, hg.ff_over_fi, hg.width_over_mh);
            higgs_rows.push(vec![tag.clone(), f(hg.v_sf), f(hg.f_f), f(hg.f_i), f(hg.mh), f(hg.mnu), f(hg.mw), f(hg.mz), f(hg.mh_over_v), f(hg.lambda), f(hg.mh_over_mw), f(hg.mh_over_2gap),
                                 f(hg.mnu_over_mh), f(hg.ff_over_fi), f(hg.mw_over_mz), f(hg.width_over_mh), f(hg.tc_over_v)]);
            for (k, v) in [("vSF", hg.v_sf), ("fF", hg.f_f), ("fI", hg.f_i), ("MH_v", hg.mh_over_v), ("lambda", hg.lambda), ("MH_MW", hg.mh_over_mw), ("MH_2gap", hg.mh_over_2gap), ("MNU_MH", hg.mnu_over_mh),
                           ("fF_fI", hg.ff_over_fi), ("MW_MZ", hg.mw_over_mz), ("width_MH", hg.width_over_mh), ("Tc_v", hg.tc_over_v)] {
                vals.push((format!("p3_h_{}_{}", k, tag), v));
            }
        }
        sf_rows.push(vec![tag.clone(), f(phi0), f(sol.energy), f(z), f(y), f(cv.v2_amp), f(cv.v2_nu), f(mh), f(rpa_h), f(mnu), f(rpa_nu), f(st.ff_t), f(st.ff_s), f(st.fi_t), f(st.fi_s), f(st.f3_s)]);
        curves_out.push((tag.clone(), cv.rows.clone()));
    }
    write_csv(&data.join("part3_sfbsv.csv"), &["method", "Phi0", "E0", "Z", "Y", "V2_amplitude", "V2_nonunitary", "M_H", "M_H_RPA", "M_NU", "M_NU_RPA", "fF2_time", "fF2_space", "fI2_time", "fI2_space", "f3_2_space"], &sf_rows);
    write_csv(&data.join("part3_landau.csv"), &["method", "Phi0", "E0", "V2_amplitude", "Phi_L", "V_L_at_Phi_L", "V_L2_at_Phi_L", "a", "a_Taylor",
        "res_K2", "res_K3", "res_K4", "res_K5", "res_K6", "sign_changes_K3", "c_of_K4", "top1", "c_top1", "res_top1", "top2", "c_top2", "res_top2", "top3", "c_top3", "res_top3"], &landau_rows);
    write_csv(&data.join("part3_thermal.csv"), &["method", "tau", "T", "Phi0", "Dirac_mass", "gap", "Dirac_mass_normal", "radius", "Phi0_over_radius", "F_min", "S_minus_S_normal",
        "gap_equation_residual", "a_Taylor", "b_Taylor", "Phi0_Landau", "iterations"], &th_rows);
    write_csv(&data.join("part3_thermal_landau.csv"), &["method", "tau", "res_Landau_over_depth", "res_quartic_over_depth"], &th_land);
    write_csv(&data.join("part3_thermal_curves.csv"), &["method", "tau", "applied_field", "Phi", "F"], &th_curves);
    write_csv(&data.join("part3_ginzburg.csv"), &["method", "T_c", "Dirac_mass_normal", "alpha", "b", "c", "c_isospin_twist", "c_fixed_channels", "c_T3_twist", "c_twisted_states",
        "c_k001", "c_k002", "other_channels_k0", "other_channels_k001", "a_at_Tc", "Gi", "Gi_Tc", "xi0", "Phi_G"], &gz_rows);
    write_csv(&data.join("part3_ginzburg_regions.csv"), &["method", "k", "a_minus_a0_sphere", "a_minus_a0_lens", "sphere_over_k2", "lens_over_k"], &gz_regions);
    // part3_higgs.csv has one row per method and no row of the Standard Model, and the check P3-H4 of that row was
    // removed (21 November 2026): the DiracLike spinor carries no electroweak quantum numbers, so the superfluid is not
    // compared with the Higgs boson of the Standard Model (Sections 14.6 and 17.6 of superfluid.md)
    write_csv(&data.join("part3_higgs.csv"), &["method", "v_SF", "f_F", "f_I", "M_H", "M_NU", "M_W", "M_Z", "M_H_over_v_SF", "lambda_SF", "M_H_over_M_W", "M_H_over_2E_gap",
        "M_NU_over_M_H", "f_F_over_f_I", "M_W_over_M_Z", "Gamma_over_M_H", "T_c_over_v_SF"], &higgs_rows);
    write_csv(&data.join("part3_landau_residuals.csv"), &["method", "Phi", "Phi_over_Phi0", "res_K2_over_depth", "res_K3_over_depth", "res_K4_over_depth"], &resid_rows);
    for (tag, rows) in &curves_out {
        let r: Vec<Vec<String>> = rows.iter().map(|x| vec![f(x.0), f(x.1), f(x.2), f(x.3)]).collect();
        write_csv(&data.join(format!("part3_effpot_{}.csv", tag)), &["applied_field", "Phi3", "V", "Dirac_mass"], &r);
    }
    // the six directions are exact for the twist at second order: compare with a product rule of 6 x 12 directions (HS)
    {
        let m = Method::Hs { s_s: 1.0, s_v: 1.0 };
        let mut m6 = Model::new(M_LL, 0.0, terms(&m, &v2), grid(24));
        let mut mp = Model::new(M_LL, 0.0, terms(&m, &v2), Grid::new(24, 1.0, DirRule::Product(6, 12)));
        let s6 = m6.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let sp = mp.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let t6 = kinetic::twist(&mut m6, &s6.n_self, &s6.p_self, s6.energy, &CMat::identity(2));
        let tp = kinetic::twist(&mut mp, &sp.n_self, &sp.p_self, sp.energy, &CMat::identity(2));
        // the six directions integrate the second-order term of the twist exactly; the terms of fourth order in Q, where
        // they are not exact, are removed by the Richardson extrapolation only up to terms of relative order Q^4
        rep.add("P3-7", &format!("the six directions are exact for the twisted states at second order: phase twist of HS (1, 1), six vs 72 directions ({:.10e}, {:.10e}; relative)", t6, tp), (t6 - tp).abs() / tp.abs(), 2e-6);
    }
    let vrows: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), format!("{:.12e}", v)]).collect();
    write_csv(&data.join("part3_values.csv"), &["name", "value"], &vrows);
    rep.note(&format!("run time {:.1} s", t0.elapsed().as_secs_f64()));
    rep.write(&data.join("hck_part3_checks.txt"));
    let (np, n) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, n, t0.elapsed().as_secs_f64());
    std::process::exit(if np == n { 0 } else { 1 });
}
