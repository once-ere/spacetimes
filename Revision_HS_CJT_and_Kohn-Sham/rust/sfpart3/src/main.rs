//! sf_part3 -- part III of superfluid.md (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes): the effective
//! Lagrangians L^CJT, L^HS, L^KS of the field of the superfluid boson singlet state, their potential parts (the Mexican
//! hat), their kinetic parts, the minima and the breaking of the symmetry, and the expansions SFBSv^X about the vacuum.
//!
//! Build and run (from the folder rust/, after sf_part2, whose data file part2_modes.csv is read for the comparison of
//! the masses):
//!     cargo build --release
//!     ./target/release/sf_part3
//! Output in data/: hck_part3_checks.txt and the data files part3_*.csv.  Run time about two minutes.
mod effpot;
mod kinetic;

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
    let mut curves_out = vec![];
    for m in [Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Ks, Method::Cjt] {
        let tag = m.short();
        let mut model = Model::new(M_LL, 0.0, terms(&m, &v2), grid(24));
        let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let phi0 = k_scalar(3).mul(&sol.p_self).trace().scale(1.0 / 8.0).abs();
        // the phase of the ground state is that of the start (real); the field is Phi0 e3
        let p0 = k_scalar(3).dagger().scale_r(phi0);
        let cv = effpot::curve(&mut rep, &tag, &model, &sol.n_self, phi0, sol.energy, 60);
        let st = kinetic::stiffness(&mut model, &sol.n_self, &p0, sol.energy);
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
        sf_rows.push(vec![tag.clone(), f(phi0), f(sol.energy), f(z), f(y), f(cv.v2_amp), f(cv.v2_nu), f(mh), f(rpa_h), f(mnu), f(rpa_nu), f(st.ff_t), f(st.ff_s), f(st.fi_t), f(st.fi_s), f(st.f3_s)]);
        curves_out.push((tag.clone(), cv.rows.clone()));
    }
    write_csv(&data.join("part3_sfbsv.csv"), &["method", "Phi0", "E0", "Z", "Y", "V2_amplitude", "V2_nonunitary", "M_H", "M_H_RPA", "M_NU", "M_NU_RPA", "fF2_time", "fF2_space", "fI2_time", "fI2_space", "f3_2_space"], &sf_rows);
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
