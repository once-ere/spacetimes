//! sf_part4 -- part IV of superfluid.md (folder Revision_HS_CJT_and_Kohn-Sham of PROJECT spacetimes): the mass matrix
//! of the gauge bosons in the vacua of the superfluid, its diagonalization, the surviving U(1) and its massless gauge boson, the zeugon (not the photon)
//! (Section 14); the dynamics of the uniform superfluid (the time-dependent mean field (6.6), integrated with CVODE) and
//! of the gauge fields of the effective theory (CVODE and IDA) (Section 15).
//!
//! Build and run (from the folder rust/, after sf_part2 and sf_part3, whose data files part2_modes.csv,
//! part2_values.csv and part3_values.csv are read for the comparisons):
//!     cargo build --release
//!     ./target/release/sf_part4
//! Output in data/: hck_part4_checks.txt and the data files part4_*.csv.  Run time a few minutes.
mod mass;
mod signal;
mod spectral;
mod tdhfb;
mod waves;

use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::{beta, d8, h_free, k_scalar};
use sfcore::linalg::eigvals;
use sfcore::methods::{grid, terms, Method, Potential, M_LL};
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::rpa::{kernel, Response};
use sfcore::uniform::Model;
use sfsundials::cvode::{self, CvodeSettings, Weights};
use sfsundials::ida::{self, IdaSettings};
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;
use std::sync::Arc;
use std::time::Instant;

const GV: f64 = 0.7;
const GF: f64 = 0.45;

fn threads() -> usize {
    std::thread::available_parallelism().map(|n| n.get()).unwrap_or(2).min(8)
}

fn read_values(p: &Path) -> HashMap<String, f64> {
    std::fs::read_to_string(p)
        .unwrap_or_else(|_| panic!("missing {}: run sf_part2 and sf_part3 first", p.display()))
        .lines()
        .skip(1)
        .filter_map(|l| {
            let v: Vec<&str> = l.split(',').collect();
            if v.len() == 2 {
                Some((v[0].to_string(), v[1].parse().ok()?))
            } else {
                None
            }
        })
        .collect()
}

fn methods() -> Vec<Method> {
    vec![Method::Hs { s_s: 1.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 1.0 }, Method::Hs { s_s: 0.0, s_v: 0.0 }, Method::Ks, Method::Cjt]
}

fn v2() -> Potential {
    Potential { gs: 2.8, gv: 0.8, g: 25.2 }
}

fn e9(x: f64) -> String {
    format!("{:.9e}", x)
}

/// P4-0: the integrators on problems with known solutions
fn integrator_tests(rep: &mut Report) {
    // (a) CVODE, Adams, complex weights: z' = i omega z, z(0) = 1
    let om = 2.0;
    let touts: Vec<f64> = (0..=50).map(|i| i as f64).collect();
    let f: cvode::Rhs = Box::new(move |_t, y, dy| {
        dy[0] = -om * y[1];
        dy[1] = om * y[0];
        0
    });
    let mut err: f64 = 0.0;
    let set = CvodeSettings::adams(1e-10, 1e-14, Weights::Complex);
    let st = cvode::integrate(&[1.0, 0.0], 0.0, &touts, &set, f, |_, t, y| {
        err = err.max((y[0] - (om * t).cos()).abs().max((y[1] - (om * t).sin()).abs()));
        true
    })
    .expect("CVODE test");
    rep.add("P4-0a", &format!("CVODE (Adams, complex weights, rtol 1e-10) integrates z' = 2 i z to t = 50 ({}): largest error", st.describe()), err, 1e-7);
    // (b) CVODE, BDF, real weights, same problem
    let f: cvode::Rhs = Box::new(move |_t, y, dy| {
        dy[0] = -om * y[1];
        dy[1] = om * y[0];
        0
    });
    let mut err: f64 = 0.0;
    let set = CvodeSettings::bdf(1e-10, 1e-14, Weights::Real);
    let st = cvode::integrate(&[1.0, 0.0], 0.0, &touts, &set, f, |_, t, y| {
        err = err.max((y[0] - (om * t).cos()).abs().max((y[1] - (om * t).sin()).abs()));
        true
    })
    .expect("CVODE test");
    rep.add("P4-0b", &format!("CVODE (BDF, real weights, rtol 1e-10) on the same problem ({}): largest error", st.describe()), err, 1e-6);
    // (c) IDA: y1' = y3, y2' = -y1, 0 = y3 - y2 (y3 algebraic); exact y1 = sin t, y2 = y3 = cos t
    let id = [1.0, 1.0, 0.0];
    let touts: Vec<f64> = (0..=20).map(|i| i as f64).collect();
    for (k, with_jac) in [(0, true), (1, false)] {
        let res: ida::Residual = Box::new(|_t, y, yp, r| {
            r[0] = yp[0] - y[2];
            r[1] = yp[1] + y[0];
            r[2] = y[2] - y[1];
            0
        });
        let jac: Option<ida::DenseJac> = if with_jac {
            Some(Box::new(|_t, cj, _y, _yp, m| {
                // column-major 3 x 3
                m[0] = cj; // dF0/dy0
                m[3 + 1] = cj; // dF1/dy1
                m[1] = 1.0; // dF1/dy0
                m[6] = -1.0; // dF0/dy2
                m[6 + 2] = 1.0; // dF2/dy2
                m[3 + 2] = -1.0; // dF2/dy1
                0
            }))
        } else {
            None
        };
        let mut err: f64 = 0.0;
        let set = IdaSettings::new(1e-10, 1e-14);
        let st = ida::integrate(&[0.0, 1.0, 1.0], &[1.0, 0.0, 0.0], &id, 0.0, &touts, &set, res, jac, |_, t, y, _| {
            err = err.max((y[0] - t.sin()).abs()).max((y[1] - t.cos()).abs()).max((y[2] - t.cos()).abs());
            true
        })
        .expect("IDA test");
        rep.add(
            &format!("P4-0{}", ["c", "d"][k]),
            &format!("IDA (rtol 1e-10, {}) integrates the index-one system y1' = y3, y2' = -y1, 0 = y3 - y2 to t = 20 ({}): largest error", if with_jac { "exact matrix" } else { "matrix by difference quotients" }, st.describe()),
            err,
            1e-6,
        );
    }
    // (e) IDA corrects an inconsistent algebraic initial value (y3(0) = 0.3) by IDACalcIC
    let res: ida::Residual = Box::new(|_t, y, yp, r| {
        r[0] = yp[0] - y[2];
        r[1] = yp[1] + y[0];
        r[2] = y[2] - y[1];
        0
    });
    let mut set = IdaSettings::new(1e-10, 1e-14);
    set.calc_ic = true;
    let mut first = vec![];
    let mut err: f64 = 0.0;
    let st = ida::integrate(&[0.0, 1.0, 0.3], &[0.3, 0.0, 0.0], &id, 0.0, &touts, &set, res, None, |i, t, y, yp| {
        if i == 0 {
            first = vec![y[2], yp[0]];
        }
        err = err.max((y[0] - t.sin()).abs()).max((y[1] - t.cos()).abs());
        true
    })
    .expect("IDA test");
    rep.add("P4-0e", &format!("IDACalcIC corrects the algebraic initial value y3(0) = 0.3 to {:.12} and y1'(0) to {:.12} (exact 1, 1; largest change {:.3}); then the largest error to t = 20", first[0], first[1], st.ic_change), err.max((first[0] - 1.0).abs()).max((first[1] - 1.0).abs()), 1e-6);
}

/// the field of a pair self-energy, Phi^iota = tr(K^iota P)/8
fn field(p: &CMat) -> [C64; 3] {
    [k_scalar(1).mul(p).trace().scale(0.125), k_scalar(2).mul(p).trace().scale(0.125), k_scalar(3).mul(p).trace().scale(0.125)]
}

fn pair_of(phi: &[C64; 3]) -> CMat {
    let mut p = CMat::zeros(8);
    for i in 0..3 {
        p = p.add(&k_scalar(i + 1).dagger().scale(phi[i]));
    }
    p
}

/// the static susceptibility chi^ab = d<Psi^dag q_a Psi>/d mu_b of the self-consistent state reached from (n0, p0):
/// central differences at mu_b = +-h, +-2h with the Richardson extrapolation (the method of Section 11.1), with the
/// self-consistency solved to 1e-13 (at most 8000 iterations); returns the matrix and the largest final residual
fn chi_matrix(model: &mut Model, n0: &CMat, p0: &CMat, h: f64) -> ([[f64; 4]; 4], f64) {
    let qs = sfcore::uniform::gauge_charges();
    let mut chi = [[0.0; 4]; 4];
    let mut resmax: f64 = 0.0;
    for b in 0..4 {
        let mut dn = [[0.0; 4]; 2];
        for (k, step) in [h, 2.0 * h].iter().enumerate() {
            let mut r = vec![];
            for sgn in [1.0, -1.0] {
                let x = qs[b].scale_r(-sgn * step);
                model.ext_n = Some(x.clone());
                let s = model.solve(&n0.add(&x), p0, 0.6, 1e-13, 8000);
                resmax = resmax.max(s.residual);
                r.push(s.drho);
            }
            model.ext_n = None;
            for a in 0..4 {
                dn[k][a] = (qs[a].mul(&r[0]).trace().re - qs[a].mul(&r[1]).trace().re) / (2.0 * step);
            }
        }
        for a in 0..4 {
            chi[a][b] = (4.0 * dn[0][a] - dn[1][a]) / 3.0;
        }
    }
    (chi, resmax)
}

/// Section 14: the mass matrix of the vacuum Phi_0 e^{i pi/5} e_1 (the condensate in the channel K^1 with a phase) from
/// the microscopic susceptibilities and from the effective Lagrangian; the vacuum Phi_0 e^{i pi/5} (1, 1, 1)/sqrt 3 from
/// the effective Lagrangian; the field Phi_0 (e_3 + i e_1)/sqrt 2
fn section14(rep: &mut Report, vals: &mut Vec<(String, f64)>, p3: &HashMap<String, f64>, data: &Path) {
    let mut rows = vec![];
    let th = PI / 5.0;
    let gs = [GV, GV, GV, GF];
    for m in methods() {
        let tag = m.short();
        let t1 = Instant::now();
        let mut model = Model::new(M_LL, 0.0, terms(&m, &v2()), grid(24));
        let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let phi0 = field(&sol.p_self)[2].abs();
        let e0 = sol.energy;
        // the vacuum in the channel K^1 with the phase e^{i pi/5}
        let ph_r: [C64; 3] = [C64::cis(th).scale(phi0), C64::ZERO, C64::ZERO];
        let sr = model.solve(&sol.n_self, &pair_of(&ph_r), 0.6, 1e-14, 40000);
        let ph_s = field(&sr.p_self);
        let dir_err = (0..3).map(|i| (ph_s[i] - ph_r[i]).abs()).fold(0.0, f64::max) / phi0;
        rep.add(&format!("P4-M0 {}", tag), &format!("{}: the vacuum Phi_0 e^(i pi/5) e_1 is a self-consistent state with the ground-state energy (E = {:.12e}, E0 = {:.12e}; relative difference, plus the change of its field relative to Phi_0)", tag, sr.energy, e0), ((sr.energy - e0) / e0).abs() + dir_err, 1e-10);
        // the 4 x 4 susceptibility matrix and the mass matrix (11.1)
        let (chi, resmax) = chi_matrix(&mut model, &sr.n_self, &sr.p_self, 2e-3);
        let chiff_r = chi[3][3];
        let asym = (0..4).flat_map(|a| (0..4).map(move |b| (a, b))).map(|(a, b)| (chi[a][b] - chi[b][a]).abs()).fold(0.0, f64::max);
        let mut mm = [[0.0; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                mm[a][b] = gs[a] * gs[b] * 0.5 * (chi[a][b] + chi[b][a]);
            }
        }
        let (ev, vecs) = mass::jacobi(mm);
        let chi11 = p3[&format!("p3_chi11_{}", tag)];
        let chiff = p3[&format!("p3_chiFF_{}", tag)];
        let (lw, lz) = (GV * GV * chi11, GF * GF * chiff);
        let mut expect = [0.0, lw, lw, lz];
        expect.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let zeugon = mass::column(&vecs, 0);
        let ov_zeugon = mass::overlap(&zeugon, &[1.0, 0.0, 0.0, 0.0]);
        let zcol = (0..4).find(|&k| (ev[k] - lz).abs() < 1e-3 * lz).unwrap_or(3);
        let ov_z = mass::overlap(&mass::column(&vecs, zcol), &[0.0, 0.0, 0.0, 1.0]);
        rep.add(&format!("P4-M1 {}", tag), &format!("{}: the susceptibility matrix of the vacuum along e_1 is symmetric (largest asymmetry relative to chi_FF; largest residual of the self-consistency {:.1e})", tag, resmax), asym / chiff_r, 1e-7);
        rep.add(&format!("P4-M2 {}", tag), &format!("{}: the smallest eigenvalue of the mass matrix g_a g_b chi^ab of this vacuum vanishes: the zeugon is massless ({:.3e}, relative to M_Z^2 = {:.6e})", tag, ev[0], lz), ev[0].abs() / lz, 1e-7);
        rep.add(&format!("P4-M3 {}", tag), &format!("{}: its eigenvector is (1, 0, 0, 0): the zeugon is W^1, the gauge field of the generator T^1 that leaves the condensate along e_1 unchanged (1 minus the overlap)", tag), 1.0 - ov_zeugon, 1e-9);
        let dev = (1..4).map(|k| (ev[k] - expect[k]).abs() / expect[k]).fold(0.0, f64::max);
        rep.add(&format!("P4-M4 {}", tag), &format!("{}: the other eigenvalues are M_W^2, M_W^2, M_Z^2 of the vacuum along e_3 ({:.9e}, {:.9e}, {:.9e} against {:.9e}, {:.9e}, {:.9e}; relative) and the Z is B (1 minus overlap {:.1e})", tag, ev[1], ev[2], ev[3], expect[1], expect[2], expect[3], 1.0 - ov_z), dev + (1.0 - ov_z), 1e-6);
        // the effective Lagrangian (14.3) at this vacuum
        let z = p3[&format!("p3_Z_{}", tag)];
        let y = p3[&format!("p3_Y_{}", tag)];
        let ml = mass::mass_matrix_l(z, y, &ph_r, GV, GF);
        let dl = (0..4).flat_map(|a| (0..4).map(move |b| (a, b))).map(|(a, b)| (ml[a][b] - mm[a][b]).abs()).fold(0.0, f64::max);
        rep.add(&format!("P4-M5 {}", tag), &format!("{}: the mass matrix (14.3) of the effective Lagrangian L^X at this vacuum equals the microscopic one (largest difference relative to M_Z^2)", tag), dl / lz, 1e-6);
        // the vacuum along n = (1, 1, 1)/sqrt 3 with the phase e^{i pi/5}, from (14.3): the zeugon is n.W
        let s3 = 1.0 / 3f64.sqrt();
        let ph_n: [C64; 3] = [C64::cis(th).scale(phi0 * s3), C64::cis(th).scale(phi0 * s3), C64::cis(th).scale(phi0 * s3)];
        let mn = mass::mass_matrix_l(z, y, &ph_n, GV, GF);
        let (evn, vn) = mass::jacobi(mn);
        let ovn = mass::overlap(&mass::column(&vn, 0), &[s3, s3, s3, 0.0]);
        let devn = (1..4).map(|k| (evn[k] - expect[k]).abs() / expect[k]).fold(0.0, f64::max);
        rep.add(&format!("P4-M6 {}", tag), &format!("{}: the mass matrix (14.3) of L^X at the vacuum Phi_0 e^(i pi/5) (1,1,1)/sqrt3 has the eigenvalues 0 ({:.1e}), M_W^2, M_W^2, M_Z^2, and the massless field is n.W = (W^1 + W^2 + W^3)/sqrt3 (sum of 1 minus the overlap, the zero eigenvalue relative to M_Z^2 and the relative deviations)", tag, evn[0]), (1.0 - ovn) + evn[0].abs() / lz + devn, 1e-12);
        println!("{:<20} Phi0 {:.8} vacuum along e1: eigenvalues {:.3e} {:.9e} {:.9e} {:.9e} | zeugon overlap 1-{:.1e} | residual {:.1e} ({:.0} s)", m.name(), phi0, ev[0], ev[1], ev[2], ev[3], 1.0 - ov_zeugon, resmax, t1.elapsed().as_secs_f64());
        for (k, v) in [("ev0", ev[0]), ("ev1", ev[1]), ("ev2", ev[2]), ("ev3", ev[3]), ("MW", ev[1].sqrt()), ("MZ", ev[3].sqrt()), ("A_overlap_defect", 1.0 - ov_zeugon), ("chi11_e1", chi[0][0]), ("chi22_e1", chi[1][1]), ("chiFF_e1", chi[3][3])] {
            vals.push((format!("p4_mass_{}_{}", k, tag), v));
        }
        rows.push(vec![tag.clone(), f(phi0), f(ev[0]), f(ev[1]), f(ev[2]), f(ev[3]), f(lw), f(lz), f(1.0 - ov_zeugon), f(1.0 - ov_z), f(chi[0][0]), f(chi[1][1]), f(chi[2][2]), f(chi[3][3]), f(dl / lz)]);
    }
    write_csv(&data.join("part4_massmatrix.csv"), &["method", "Phi0", "eigenvalue_0", "eigenvalue_1", "eigenvalue_2", "eigenvalue_3", "MW2_e3", "MZ2_e3", "A_overlap_defect", "Z_overlap_defect", "chi_11", "chi_22", "chi_33", "chi_FF", "L_minus_micro"], &rows);
    // the polar vacuum along e_3 from (14.3): {0, 2 Z Phi0^2 g_v^2 (twice), 8 (Z + Y) Phi0^2 g_F^2}
    let tag = "CJT";
    let z = p3[&format!("p3_Z_{}", tag)];
    let y = p3[&format!("p3_Y_{}", tag)];
    let phi0 = p3[&format!("p3_Phi0_{}", tag)];
    let ml = mass::mass_matrix_l(z, y, &[C64::ZERO, C64::ZERO, c(phi0, 0.0)], GV, GF);
    let (evl, _) = mass::jacobi(ml);
    let exp = [0.0, 2.0 * z * phi0 * phi0 * GV * GV, 2.0 * z * phi0 * phi0 * GV * GV, 8.0 * (z + y) * phi0 * phi0 * GF * GF];
    let d = (0..4).map(|k| (evl[k] - exp[k]).abs()).fold(0.0, f64::max) / exp[3];
    rep.add("P4-M7", &format!("the mass matrix (14.3) of L^CJT at Phi_0 e_3 has the eigenvalues 0, 2 Z Phi_0^2 g_v^2 (twice), 8 (Z + Y) Phi_0^2 g_F^2 of (13.7) ({:.3e}, {:.9e}, {:.9e}, {:.9e}; largest difference relative to M_Z^2)", evl[0], evl[1], evl[2], evl[3]), d, 1e-13);
    // the field Phi_0 (e_3 + i e_1)/sqrt 2: one massless combination, cos(th') W^2 - sin(th') B with tan th' = g_v/(2 g_F)
    let r2 = 1.0 / 2f64.sqrt();
    let phn = [c(0.0, phi0 * r2), C64::ZERO, c(phi0 * r2, 0.0)];
    let mn = mass::mass_matrix_l(z, y, &phn, GV, GF);
    let (evn, vn) = mass::jacobi(mn);
    let thp = (GV / (2.0 * GF)).atan();
    let ov = mass::overlap(&mass::column(&vn, 0), &[0.0, thp.cos(), 0.0, -thp.sin()]);
    rep.add("P4-M8", &format!("for the field Phi_0 (e_3 + i e_1)/sqrt2 (not a vacuum) the mass matrix (14.3) of L^CJT has one zero eigenvalue ({:.2e}; next {:.6e}), whose eigenvector is cos(th') W^2 - sin(th') B with tan th' = g_v/(2 g_F), th' = {:.6} degrees (1 minus the overlap, plus the zero eigenvalue relative to the next)", evn[0], evn[1], thp.to_degrees()), (1.0 - ov) + evn[0].abs() / evn[1], 1e-12);
    vals.push(("p4_mixing_angle_nonpolar_deg".into(), thp.to_degrees()));
    vals.push(("p4_charge_nonpolar".into(), GV * 2.0 * GF / (GV * GV + 4.0 * GF * GF).sqrt()));
    for k in 0..4 {
        vals.push((format!("p4_nonpolar_ev{}", k), evn[k]));
    }
}

/// the growth rate gamma of the normal vacuum: the eigenvalue of chi(i gamma) F with the largest real part falls through 1;
/// returns gamma, that eigenvalue at gamma -> 0 (the static lambda^P), and the largest modulus of the imaginary part of the
/// eigenvalues next to 1 at the root (the eigenvalue is real)
fn rpa_growth(model: &Model) -> (f64, f64, f64) {
    let z = CMat::zeros(8);
    let resp = Response::new(model, &z, &z);
    let rk = resp.with_kernel(&kernel(model));
    let eig = |g: f64| eigvals(&rk.chi_complex(c(0.0, g)));
    let lam = |g: f64| -> f64 { eig(g).iter().map(|e| e.re).fold(f64::NEG_INFINITY, f64::max) };
    let lam0 = lam(1e-12);
    let (mut a, mut b) = (0.0, 0.5);
    while lam(b) > 1.0 {
        b *= 2.0;
    }
    for _ in 0..60 {
        let mid = 0.5 * (a + b);
        if lam(mid) > 1.0 {
            a = mid;
        } else {
            b = mid;
        }
    }
    let g = 0.5 * (a + b);
    let im = eig(g).iter().filter(|e| (e.re - 1.0).abs() < 1e-6).map(|e| e.im.abs()).fold(0.0, f64::max);
    (g, lam0, im)
}

/// one integration of (6.6) from y0 to the output times; returns the observables (full ones at every `full_every`-th
/// output) and the description of the counters
fn run_td(model: &Arc<Model>, y0: &[f64], touts: &[f64], rtol: f64, atol: f64, full_every: usize, polar: bool, mut frame: impl FnMut(f64, &[f64])) -> (Vec<tdhfb::Obs>, String) {
    let opp = tdhfb::opposite_nodes(model);
    let nth = threads();
    let m2 = model.clone();
    let f: cvode::Rhs = Box::new(move |_t, y, dy| {
        tdhfb::rhs(&m2, nth, polar, y, dy);
        0
    });
    let set = CvodeSettings::adams(rtol, atol, Weights::Complex);
    let mut obs = vec![];
    let st = cvode::integrate(y0, 0.0, touts, &set, f, |i, t, y| {
        obs.push(tdhfb::observe(model, &opp, t, y, i % full_every == 0 || i + 1 == touts.len()));
        frame(t, y);
        true
    })
    .expect("CVODE failed on the time-dependent mean field");
    (obs, st.describe())
}

fn max_full(obs: &[tdhfb::Obs], key: fn(&tdhfb::Obs) -> f64) -> f64 {
    obs.iter().map(key).fold(0.0, f64::max)
}

/// Section 15.1: the time-dependent mean field of the five methods
fn section15_tdhfb(rep: &mut Report, vals: &mut Vec<(String, f64)>, p2: &HashMap<String, f64>, data: &Path) {
    let modes: Vec<(String, f64, i64)> = std::fs::read_to_string(data.join("part2_modes.csv"))
        .expect("run sf_part2 first")
        .lines()
        .skip(1)
        .map(|l| {
            let v: Vec<&str> = l.split(',').collect();
            (v[0].to_string(), v[4].parse().unwrap(), v[5].parse().unwrap())
        })
        .collect();
    let mut summary = vec![];
    for m in methods() {
        let tag = m.short();
        let t1 = Instant::now();
        let model = Model::new(M_LL, 0.0, terms(&m, &v2()), grid(24));
        let sol = model.solve(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.3), 0.6, 1e-14, 40000);
        let (n0, p0) = (sol.n_self.clone(), sol.p_self.clone());
        let phi0 = field(&p0)[2].abs();
        let e0 = sol.energy;
        // (a) the growth rate of the unstable pair mode of the normal vacuum, by the linear response
        let (g_rpa, lam0, im_root) = rpa_growth(&model);
        let lam_p2 = p2[&format!("p2_lamP_V2_{}", tag)];
        rep.add(&format!("P4-T1 {}", tag), &format!("{}: the static eigenvalue of the pair channel of the normal vacuum from the response at imaginary frequency equals lambda^P of Part II ({:.12}, {:.12}; relative)", tag, lam0, lam_p2), ((lam0 - lam_p2) / lam_p2).abs(), 1e-8);
        rep.add(&format!("P4-T1b {}", tag), &format!("{}: at the growth rate gamma = {:.9} the eigenvalues of chi(i gamma) F next to 1 are real (largest modulus of their imaginary parts)", tag, g_rpa), im_root, 1e-10);
        let model = Arc::new(model);
        // (b) the growth of the condensate from the normal vacuum with the seed 1e-7 (K^3)^dag
        let tg: Vec<f64> = (0..=1000).map(|i| i as f64 * 0.25).collect();
        let y0 = tdhfb::state_of(&model, &CMat::zeros(8), &k_scalar(3).dagger().scale_r(1e-7));
        let mut frames = vec![];
        let is_cjt = tag == "CJT";
        let nd = model.grid.dirs.len();
        let (og, dg) = run_td(&model, &y0, &tg, 1e-10, 1e-15, 40, true, |t, y| {
            if is_cjt && ((t * 2.0).round() - t * 2.0).abs() < 1e-9 {
                // the frames of the video: the pair amplitude |C^p(q)| and the occupation of the upper band along +e_3
                for ir in 0..model.grid.kr.len() {
                    let n = ir * nd + 4;
                    let r = tdhfb::node_matrix(y, n);
                    let cp = r.sub_block(0, 8, 8, 8);
                    let cn = r.sub_block(0, 0, 8, 8);
                    let (q, _) = model.grid.node(n);
                    let eps = (q[2] * q[2] + M_LL * M_LL).sqrt();
                    let vac = CMat::identity(8).sub(&h_free(&q, M_LL).scale_r(1.0 / eps)).scale_r(0.5);
                    let upper = CMat::identity(8).sub(&vac).mul(&cn).trace().re;
                    frames.push(vec![f(t), f(q[2]), e9(cp.fro() / 8f64.sqrt()), e9(upper)]);
                }
            }
        });
        let amp: Vec<f64> = og.iter().map(|o| o.phi[2].abs()).collect();
        let (mut tw, mut lw) = (vec![], vec![]);
        for (o, a) in og.iter().zip(&amp) {
            if *a > 3e-5 * phi0 && *a < 3e-3 * phi0 {
                tw.push(o.t);
                lw.push(a.ln());
            }
            if *a >= 3e-3 * phi0 {
                break;
            }
        }
        let (g_td, _) = signal::line_fit(&tw, &lw);
        rep.add(&format!("P4-T2 {}", tag), &format!("{}: the condensate grows from the normal vacuum at the rate of the unstable mode of the linear response, gamma = {:.9} (fit of ln|Phi^3| on {} outputs, {:.1} <= t <= {:.1}), gamma_RPA = {:.9}; relative", tag, g_td, tw.len(), tw.first().unwrap_or(&0.0), tw.last().unwrap_or(&0.0), g_rpa), ((g_td - g_rpa) / g_rpa).abs(), 2e-5);
        let ed = og.iter().map(|o| (o.energy - og[0].energy).abs()).fold(0.0, f64::max) / e0.abs();
        let qd = og.iter().map(|o| o.charges.iter().map(|x| x.abs()).fold(0.0, f64::max)).fold(0.0, f64::max);
        let pur = max_full(&og, |o| o.purity).max(max_full(&og, |o| o.hermiticity)).max(max_full(&og, |o| o.nambu));
        let side = og.iter().map(|o| o.phi[0].abs().max(o.phi[1].abs())).fold(0.0, f64::max);
        let phmax = amp.iter().cloned().fold(0.0, f64::max);
        rep.add(&format!("P4-T3 {}", tag), &format!("{}: growth run in the polar sector to t = 250 ({}): the energy per volume is conserved (largest change relative to |E0|)", tag, dg), ed, 1e-6);
        rep.add(&format!("P4-T4 {}", tag), &format!("{}: the four charge densities stay zero (largest modulus)", tag), qd, 1e-12);
        rep.add(&format!("P4-T5 {}", tag), &format!("{}: R stays a Hermitian projector with R_22(q) = 1 - R_11(-q)^* (largest deviation, every 10 time units)", tag), pur, 1e-7);
        rep.add(&format!("P4-T6 {}", tag), &format!("{}: the pair stays in the channel K^3, the state in the polar sector (largest |Phi^1|, |Phi^2| relative to the largest |Phi^3| = {:.6})", tag, phmax), side / phmax, 1e-12);
        // the stability of the polar pulse train: the full right side, with the seed eps i (K^1)^dag (eps = 1e-11, 1e-13) added to the seed
        // of the growth run; s(t) = (|Phi^1|^2 + |Phi^2|^2)^(1/2)/Phi_0 measures the departure from the polar sector.  In
        // the linear stage the departure grows with the condensate (all three pair channels of the normal vacuum are
        // unstable); after the saturation its largest value in each pulse, e_k, grows by a factor mu per pulse if the
        // pulse train is unstable.  The pulses are the intervals between consecutive maxima of |Phi^3| of the polar run.
        let pk_t: Vec<f64> = {
            let mut v = vec![];
            for i in 1..og.len() - 1 {
                let (a0, a1, a2) = (og[i - 1].phi[2].abs(), og[i].phi[2].abs(), og[i + 1].phi[2].abs());
                if a1 > a0 && a1 >= a2 && a1 > 0.3 * phi0 {
                    v.push(og[i].t);
                }
            }
            v
        };
        let seed = |eps: f64| tdhfb::state_of(&model, &CMat::zeros(8), &k_scalar(3).dagger().scale_r(1e-7).add(&k_scalar(1).dagger().scale(c(0.0, eps))));
        let off = |o: &tdhfb::Obs| (o.phi[0].norm_sqr() + o.phi[1].norm_sqr()).sqrt() / phi0;
        let envelope = |os: &[tdhfb::Obs]| -> Vec<f64> {
            (0..pk_t.len().saturating_sub(1)).map(|k| os.iter().filter(|o| o.t >= pk_t[k] && o.t < pk_t[k + 1]).map(off).fold(0.0, f64::max)).collect()
        };
        // The absolute tolerance of the two seeded runs is 1e-18, far below the off-polar entries of R of the smaller
        // seed (of order 1e-13).  With 1e-15 the integrator controlled those entries only to about one per cent of
        // their size, and in HS(0,0) the two seeded runs departed from the factor 100 by 1.8e-3 after the passage near
        // the normal vacuum (the minimum of |Phi^3| at t = 58); see Section 17.4 of the manuscript.
        let atol_seeded = 1e-18;
        let (os1, ds1) = run_td(&model, &seed(1e-11), &tg, 1e-10, atol_seeded, 1000, false, |_, _| {});
        let s1: Vec<f64> = os1.iter().map(off).collect();
        let es1 = os1.iter().map(|o| (o.energy - os1[0].energy).abs()).fold(0.0, f64::max) / e0.abs();
        rep.add(&format!("P4-T13 {}", tag), &format!("{}: the run with the full right side and the non-unitary seed 1e-11 i (K^1)^dag ({}) conserves the energy (largest change relative to |E0|)", tag, ds1), es1, 1e-6);
        let (os2, _) = run_td(&model, &seed(1e-13), &tg, 1e-10, atol_seeded, 1000, false, |_, _| {});
        let env2 = envelope(&os2);
        // linearity: s(t) of the two seeds differ by the factor 100 through the growth and the first pulse, while the
        // larger is below 1e-3 (the largest |s_1 - 100 s_2| relative to the largest s_1 of that time window; no ratios of
        // small numbers).  The check covers this time window only; the later pulses are not compared.
        let s2v: Vec<f64> = os2.iter().map(off).collect();
        let t_end = pk_t.get(1).cloned().unwrap_or(f64::INFINITY);
        let nlin = s1.iter().zip(&os1).position(|(v, o)| *v >= 1e-3 || o.t > t_end).unwrap_or(s1.len());
        let smax = s1[..nlin].iter().cloned().fold(0.0, f64::max);
        let lin_dev = (0..nlin).map(|i| (s1[i] - 100.0 * s2v[i]).abs()).fold(0.0, f64::max) / smax;
        let npulse = pk_t.iter().filter(|t| **t <= os1[nlin.saturating_sub(1)].t).count();
        rep.add(&format!("P4-T14 {}", tag), &format!("{}: the departure from the polar sector is linear in the seed: until t = {:.2} ({} maxima of |Phi^3|; the growth and the first pulse, while s < 1e-3), s(t) for the seed 1e-11 is 100 times s(t) for the seed 1e-13 (largest |s_1 - 100 s_2| relative to the largest s_1)", tag, os1[nlin.saturating_sub(1)].t, npulse), lin_dev, 1e-3);
        // the growth factor per pulse: the least-squares line through (k, ln e_k) of the seed 1e-13 over the pulses
        // k = 0, 1, ... while e_k < 0.1 (the departure is then still small: its nonlinear corrections are of the relative
        // order e_k^2, because the departure has the charges +-1 of Q_LL and enters its own equation first at the third
        // order); at least two pulses
        let (mut kx, mut ky) = (vec![], vec![]);
        for (k, e) in env2.iter().enumerate() {
            if *e < 0.1 && *e > 0.0 {
                kx.push(k as f64);
                ky.push(e.ln());
            } else if *e >= 0.1 {
                break;
            }
        }
        let mu = if kx.len() >= 2 { signal::line_fit(&kx, &ky).0.exp() } else { f64::NAN };
        let t_mix = os1.iter().zip(&s1).find(|(_, v)| **v > 0.1).map(|(o, _)| o.t).unwrap_or(f64::NAN);
        let amp_rate = mu;
        vals.push((format!("p4_td_mu_{}", tag), mu));
        vals.push((format!("p4_td_mu_pulses_{}", tag), kx.len() as f64));
        vals.push((format!("p4_td_tmix_{}", tag), t_mix));
        let s_end = env2.last().cloned().unwrap_or(f64::NAN);
        vals.push((format!("p4_td_offend_{}", tag), s_end));
        // the maxima of |Phi^3|
        let mut peaks = vec![];
        for i in 1..amp.len() - 1 {
            if amp[i] > amp[i - 1] && amp[i] >= amp[i + 1] && amp[i] > 0.3 * phi0 {
                peaks.push((og[i].t, amp[i]));
            }
        }
        let period = if peaks.len() >= 2 { peaks[1].0 - peaks[0].0 } else { f64::NAN };
        // (c) the amplitude (Higgs) mode: the ground state of the BdG matrix with P = (1 + 1e-3) P_0
        let tk: Vec<f64> = (0..=1600).map(|i| i as f64 * 0.25).collect();
        let ya = tdhfb::state_of(&model, &n0, &p0.scale_r(1.0 + 1e-3));
        let (oa, da) = run_td(&model, &ya, &tk, 1e-10, 1e-13, 200, false, |_, _| {});
        let ts: Vec<f64> = oa.iter().filter(|o| o.t >= 30.0).map(|o| o.t).collect();
        let sa: Vec<f64> = oa.iter().filter(|o| o.t >= 30.0).map(|o| o.phi[2].abs()).collect();
        let w_a_rpa = modes.iter().filter(|r| r.0 == tag && (r.2 == 1 || r.2 == 3)).map(|r| r.1).next().unwrap();
        let w_a = signal::peak_frequency(&ts, &sa, w_a_rpa - 0.05, w_a_rpa + 0.05);
        rep.add(&format!("P4-T7 {}", tag), &format!("{}: after a kick of the amplitude, |Phi^3| oscillates at {:.9} ({}); the amplitude mode of the linear response is at {:.9} (relative)", tag, w_a, da, w_a_rpa), ((w_a - w_a_rpa) / w_a_rpa).abs(), 2e-4);
        // (d) the non-unitary modes: P = P_0 + i 1e-3 Phi_0 (K^1)^dag
        let yn = tdhfb::state_of(&model, &n0, &p0.add(&k_scalar(1).dagger().scale(c(0.0, 1e-3 * phi0))));
        let (on, dn) = run_td(&model, &yn, &tk, 1e-10, 1e-13, 200, false, |_, _| {});
        let sn: Vec<f64> = on.iter().filter(|o| o.t >= 30.0).map(|o| (o.phi[0] * o.phi[2].conj()).im / o.phi[2].abs()).collect();
        let w_n_rpa = modes.iter().filter(|r| r.0 == tag && (r.2 == 2 || r.2 == 3)).map(|r| r.1).next().unwrap();
        let w_n = signal::peak_frequency(&ts, &sn, w_n_rpa - 0.05, w_n_rpa + 0.05);
        rep.add(&format!("P4-T8 {}", tag), &format!("{}: after a non-unitary kick, Im(Phi^1 Phi^3*)/|Phi^3| oscillates at {:.9} ({}); the non-unitary modes of the linear response are at {:.9} (relative)", tag, w_n, dn, w_n_rpa), ((w_n - w_n_rpa) / w_n_rpa).abs(), 2e-4);
        let edk = oa.iter().map(|o| (o.energy - oa[0].energy).abs()).fold(0.0, f64::max).max(on.iter().map(|o| (o.energy - on[0].energy).abs()).fold(0.0, f64::max)) / e0.abs();
        rep.add(&format!("P4-T9 {}", tag), &format!("{}: the energy is conserved in the two runs with kicks to t = 400 (largest change relative to |E0|)", tag), edk, 1e-7);
        println!("{:<20} gamma RPA {:.9} TD {:.9} | max|Phi3| {:.6} (Phi0 {:.6}) period {:.3} | omega_A {:.9} (RPA {:.9}) omega_NU {:.9} (RPA {:.9}) | dE {:.1e} | growth per pulse {:.4} t_mix {:.1} lin {:.1e} ({:.0} s)", m.name(), g_rpa, g_td, phmax, phi0, period, w_a, w_a_rpa, w_n, w_n_rpa, ed, amp_rate, t_mix, lin_dev, t1.elapsed().as_secs_f64());
        for (k, v) in [("gamma_rpa", g_rpa), ("gamma_td", g_td), ("lam0", lam0), ("Phi_max", phmax), ("Phi0", phi0), ("t_first_max", peaks.first().map(|p| p.0).unwrap_or(f64::NAN)), ("period", period), ("omega_A", w_a), ("omega_A_rpa", w_a_rpa), ("omega_NU", w_n), ("omega_NU_rpa", w_n_rpa), ("energy_drift", ed), ("energy_drift_kicks", edk)] {
            vals.push((format!("p4_td_{}_{}", k, tag), v));
        }
        summary.push(vec![tag.clone(), f(g_rpa), f(g_td), f(lam0), f(phi0), f(phmax), f(peaks.first().map(|p| p.0).unwrap_or(f64::NAN)), f(period), f(w_a), f(w_a_rpa), f(w_n), f(w_n_rpa), f(ed), f(edk), f(amp_rate), f(t_mix)]);
        // the time series
        let s2: Vec<f64> = os2.iter().map(off).collect();
        let rows: Vec<Vec<String>> = og.iter().zip(s1.iter().zip(&s2)).map(|(o, (sv, sw))| vec![f(o.t), e9(o.phi[2].re), e9(o.phi[2].im), e9(o.energy), e9(o.e_kin), e9(o.e_int), e9(o.charges[3]), e9(*sv), e9(*sw)]).collect();
        write_csv(&data.join(format!("part4_td_growth_{}.csv", tag)), &["t", "Re_Phi3", "Im_Phi3", "energy", "energy_kinetic", "energy_interaction", "charge_F", "offpolar_seed_1e-11", "offpolar_seed_1e-13"], &rows);
        let rows: Vec<Vec<String>> = oa.iter().zip(&on).map(|(a, b)| vec![f(a.t), e9(a.phi[2].abs()), e9((b.phi[0] * b.phi[2].conj()).im / b.phi[2].abs())]).collect();
        write_csv(&data.join(format!("part4_td_kicks_{}.csv", tag)), &["t", "abs_Phi3_amplitude_kick", "nonunitary_signal"], &rows);
        if is_cjt {
            write_csv(&data.join("part4_td_frames_CJT.csv"), &["t", "q", "pair_amplitude", "upper_occupation"], &frames);
            // the ground state profile of the same quantities
            let yg = tdhfb::state_of(&model, &n0, &p0);
            let mut gp = vec![];
            for ir in 0..model.grid.kr.len() {
                let n = ir * nd + 4;
                let r = tdhfb::node_matrix(&yg, n);
                let (q, _) = model.grid.node(n);
                let eps = (q[2] * q[2] + M_LL * M_LL).sqrt();
                let vac = CMat::identity(8).sub(&h_free(&q, M_LL).scale_r(1.0 / eps)).scale_r(0.5);
                gp.push(vec![f(q[2]), e9(r.sub_block(0, 8, 8, 8).fro() / 8f64.sqrt()), e9(CMat::identity(8).sub(&vac).mul(&r.sub_block(0, 0, 8, 8)).trace().re)]);
            }
            write_csv(&data.join("part4_td_groundprofile_CJT.csv"), &["q", "pair_amplitude", "upper_occupation"], &gp);
            // (e) the stationary states: the ground state, its phase rotation and its isospin rotation do not move
            let mut dy = vec![0.0; yg.len()];
            tdhfb::rhs(&model, threads(), false, &yg, &mut dy);
            let dmax = dy.iter().map(|x| x.abs()).fold(0.0, f64::max);
            rep.add("P4-T10", "CJT: the ground state is a stationary solution of (6.6) (largest entry of dR/dt)", dmax, 1e-11);
            let alpha = 0.3;
            let rot = [
                ("phase", p0.scale(C64::cis(2.0 * alpha))),
                ("isospin", pair_of(&[C64::ZERO, c(phi0 * alpha.sin(), 0.0), c(phi0 * alpha.cos(), 0.0)]).scale(c(field(&p0)[2].re.signum(), 0.0))),
            ];
            for (k, (name, pr)) in rot.iter().enumerate() {
                let yr = tdhfb::state_of(&model, &n0, pr);
                let t50: Vec<f64> = (0..=20).map(|i| i as f64 * 2.5).collect();
                let (orr, _) = run_td(&model, &yr, &t50, 1e-10, 1e-13, 100, false, |_, _| {});
                let dphi = orr.iter().map(|o| (0..3).map(|i| (o.phi[i] - orr[0].phi[i]).abs()).fold(0.0, f64::max)).fold(0.0, f64::max) / phi0;
                rep.add(&format!("P4-T11{}", ["a", "b"][k]), &format!("CJT: the ground state turned by the {} rotation (angle 0.3, a Goldstone direction) is stationary: the largest change of Phi to t = 50 relative to Phi_0", name), dphi, 1e-9);
            }
            // (f) the accuracy of the growth run: the same run with ten times smaller tolerances
            let (og2, _) = run_td(&model, &y0, &tg, 1e-11, 1e-16, 1000, true, |_, _| {});
            let dd = og.iter().zip(&og2).map(|(a, b)| (a.phi[2] - b.phi[2]).abs()).fold(0.0, f64::max) / phi0;
            rep.add("P4-T12", "CJT: the growth run with rtol 1e-11, atol 1e-16 agrees with the run of rtol 1e-10, atol 1e-15 (largest difference of Phi^3 to t = 250 relative to Phi_0)", dd, 1e-5);
            vals.push(("p4_td_tolerance_difference_CJT".into(), dd));
            // (g) the small grid of the Python comparison: 8 radial nodes, the same seed, to t = 60
            let ms = Arc::new(Model::new(M_LL, 0.0, terms(&m, &v2()), grid(8)));
            let ys = tdhfb::state_of(&ms, &CMat::zeros(8), &k_scalar(3).dagger().scale_r(1e-7));
            let tsm: Vec<f64> = (0..=120).map(|i| i as f64 * 0.5).collect();
            let (os, ds) = run_td(&ms, &ys, &tsm, 1e-10, 1e-15, 1000, true, |_, _| {});
            let rows: Vec<Vec<String>> = os.iter().map(|o| vec![f(o.t), format!("{:.15e}", o.phi[2].re), format!("{:.15e}", o.phi[2].im), format!("{:.15e}", o.energy)]).collect();
            write_csv(&data.join("part4_td_small_CJT.csv"), &["t", "Re_Phi3", "Im_Phi3", "energy"], &rows);
            vals.push(("p4_td_small_Phi3_t60_CJT".into(), os.last().unwrap().phi[2].abs()));
            println!("small grid (8 radial nodes): |Phi3(60)| = {:.9e} ({})", os.last().unwrap().phi[2].abs(), ds);
        }
    }
    write_csv(&data.join("part4_td_summary.csv"), &["method", "gamma_RPA", "gamma_TD", "lambda_P", "Phi0", "Phi_max", "t_first_max", "period", "omega_A", "omega_A_RPA", "omega_NU", "omega_NU_RPA", "energy_drift_growth", "energy_drift_kicks", "offpolar_growth_per_pulse", "t_mix"], &summary);
}

/// Section 15.2: the gauge fields of the effective theory L^CJT
fn section15_waves(rep: &mut Report, vals: &mut Vec<(String, f64)>, p3: &HashMap<String, f64>, data: &Path) {
    let mw = p3["p3_MW_CJT"];
    let mz = p3["p3_MZ_CJT"];
    let f2 = p3["p3_fF2_t_CJT"];
    rep.add("P4-W0", &format!("the mass of the Z of L^CJT is M_Z = 2 g_F f_F = {:.12} (f_F^2 = {:.9e}); g_F (chi_FF)^(1/2) of Part III = {:.12} (relative)", 2.0 * GF * f2.sqrt(), f2, mz), ((2.0 * GF * f2.sqrt() - mz) / mz).abs(), 1e-10);
    // ---------------------------------------------------------------- (a) the transverse zeugon A and W with a source
    let g1 = spectral::Grid1::new(511, 2000.0);
    let d1 = g1.dmatrix();
    // the derivative is exact for a Fourier mode inside the grid: D sin(k x) = k cos(k x) with k = 2 pi 37 / L
    {
        let k = 2.0 * PI * 37.0 / g1.len;
        let u: Vec<f64> = g1.x.iter().map(|x| (k * x).sin()).collect();
        let mut du = vec![0.0; g1.n];
        spectral::matvec(&d1, &u, &mut du);
        let e = g1.x.iter().zip(&du).map(|(x, d)| (d - k * (k * x).cos()).abs()).fold(0.0, f64::max) / k;
        let asym = (0..g1.n).flat_map(|i| (0..g1.n).map(move |j| (i, j))).map(|(i, j)| (d1[i * g1.n + j] + d1[j * g1.n + i]).abs()).fold(0.0, f64::max);
        rep.add("P4-W1", &format!("the spectral matrix (15.4) on 511 points differentiates sin(k x), k = 2 pi 37/L, exactly (largest error relative to k) and is antisymmetric (largest |D_jl + D_lj| = {:.1e})", asym), e + asym, 1e-11);
    }
    let n = g1.n;
    let mut d2 = vec![0.0; n * n];
    for i in 0..n {
        for k in 0..n {
            let a = d1[i * n + k];
            if a == 0.0 {
                continue;
            }
            for j in 0..n {
                d2[i * n + j] += a * d1[k * n + j];
            }
        }
    }
    let src = waves::Source { j0: 1e-3, sigma: 8.0, omega: 0.7 * mw, tau: 300.0 };
    let tt: Vec<f64> = (0..=200).map(|i| i as f64 * 4.5).collect();
    let set = CvodeSettings::adams(1e-10, 1e-13, Weights::Real);
    let tr = waves::transverse(&g1, &d2, mw, &src, &tt, &set).expect("CVODE transverse");
    let ue = waves::transverse_exact(&g1, 0.0, &src, &tt);
    let we = waves::transverse_exact(&g1, mw, &src, &tt);
    let eu = waves::max_diff(&tr.u, &ue, n) / waves::max_abs(&ue, n);
    let ew = waves::max_diff(&tr.w, &we, n) / waves::max_abs(&we, n);
    rep.add("P4-W2", &format!("the zeugon A driven by the source (15.2) to t = 900 ({}) equals the exact Duhamel solution of every Fourier mode (largest difference relative to the largest field)", tr.stats.describe()), eu, 1e-6);
    rep.add("P4-W3", "the W (mass M_W of L^CJT) driven by the same source equals its exact solution (largest difference relative to the largest field)", ew, 1e-6);
    // the radiation of the zeugon: amplitude J_0 |s^(omega)|/(2 omega) with s^(omega) = (2 pi)^(1/2) sigma^3 omega^2 e^{-omega^2 sigma^2/2}
    let om = src.omega;
    let shat = (2.0 * PI).sqrt() * src.sigma.powi(3) * om * om * (-0.5 * om * om * src.sigma * src.sigma).exp();
    let a_pred = src.j0 * shat / (2.0 * om);
    let ulast = tr.u.last().unwrap();
    let a_num = g1.x.iter().zip(ulast).filter(|(x, _)| **x > 100.0 && **x < 400.0).map(|(_, u)| u.abs()).fold(0.0, f64::max);
    rep.add("P4-W4", &format!("the zeugon radiates: at t = 900 its amplitude for 100 < x < 400 is {:.6e}, the outgoing wave J_0 |s^(omega_s)|/(2 omega_s) = {:.6e} (relative)", a_num, a_pred), ((a_num - a_pred) / a_pred).abs(), 2e-2);
    // the W stays bound: its amplitude at the source frequency over the last four periods against the evanescent
    // steady state, and its decay length against 1/(M_W^2 - omega_s^2)^(1/2)
    let period = 2.0 * PI / om;
    let tstart = 900.0 - 2.0 * period;
    let mut aw = vec![0.0; n];
    let sel: Vec<usize> = (0..tt.len()).filter(|&i| tt[i] >= tstart).collect();
    for j in 0..n {
        // the projection (2/Delta) int w sin(omega t) dt by the trapezoidal rule on the outputs (4.5 apart)
        let mut s = 0.0;
        for (k, &i) in sel.iter().enumerate() {
            let wgt = if k == 0 || k + 1 == sel.len() { 0.5 } else { 1.0 };
            s += wgt * tr.w[i][j] * (om * tt[i]).sin();
        }
        aw[j] = 2.0 * s * 4.5 / (tt[*sel.last().unwrap()] - tt[sel[0]]);
    }
    let kap = (mw * mw - om * om).sqrt();
    let sj: Vec<f64> = g1.x.iter().map(|&x| src.j0 * src.profile(x)).collect();
    let sjh = g1.dft(&sj);
    let wss = g1.idft(&sjh.iter().zip(&g1.k).map(|(z, &k)| z.scale(1.0 / (k * k + kap * kap))).collect::<Vec<C64>>());
    let (mut xf, mut lf) = (vec![], vec![]);
    for j in 0..n {
        if g1.x[j] > 30.0 && g1.x[j] < 100.0 {
            xf.push(g1.x[j]);
            lf.push(aw[j].abs().ln());
        }
    }
    let (slope, _) = signal::line_fit(&xf, &lf);
    let a0n = aw.iter().map(|x| x.abs()).fold(0.0, f64::max);
    let a0s = wss.iter().map(|x| x.abs()).fold(0.0, f64::max);
    let prof = (0..n).filter(|&j| g1.x[j].abs() < 100.0).map(|j| (aw[j] - wss[j]).abs()).fold(0.0, f64::max) / a0s;
    rep.add("P4-W5", &format!("the W stays a bound cloud: its decay length for 30 < x < 100 is {:.4} against 1/(M_W^2 - omega_s^2)^(1/2) = {:.4}, and its amplitude for |x| < 100 differs from the evanescent steady state by at most {:.2e} of its largest value {:.5e} (sum of the relative difference of the lengths and this profile difference)", -1.0 / slope, 1.0 / kap, prof, a0s), ((-1.0 / slope) * kap - 1.0).abs() + prof, 5e-2);
    for (k, v) in [("MW", mw), ("MZ", mz), ("omega_s", om), ("A_amplitude", a_num), ("A_amplitude_pred", a_pred), ("W_decay_length", -1.0 / slope), ("W_decay_length_pred", 1.0 / kap), ("W_amplitude", a0n), ("W_amplitude_ss", a0s)] {
        vals.push((format!("p4_waves_{}", k), v));
    }
    let mut rows = vec![];
    for (i, &t) in tt.iter().enumerate().step_by(2) {
        for j in 0..n {
            rows.push(vec![f(t), f(g1.x[j]), e9(tr.u[i][j]), e9(tr.w[i][j])]);
        }
    }
    write_csv(&data.join("part4_waves_transverse.csv"), &["t", "x", "A", "W"], &rows);
    let rows: Vec<Vec<String>> = (0..n).map(|j| vec![f(g1.x[j]), e9(aw[j]), e9(wss[j])]).collect();
    write_csv(&data.join("part4_waves_Wcloud.csv"), &["x", "W_amplitude", "W_steady_state"], &rows);
    // ---------------------------------------------------------------- (b) the Goldstone field and the longitudinal Z
    let g2 = spectral::Grid1::new(255, 1024.0);
    let d = g2.dmatrix();
    let n = g2.n;
    let pk = waves::Packet { eps: 1.0, x0: -150.0, sigma: 40.0, k0: 0.08 };
    let tl: Vec<f64> = (0..=140).map(|i| i as f64 * 2.5).collect();
    let setc = CvodeSettings::adams(1e-10, 1e-13, Weights::Real);
    // the free Goldstone field (g = 0)
    let yfree = waves::initial_temporal(&g2, &d, &pk, 0.0, f2);
    let fr = waves::temporal(&g2, &d, &yfree, 0.0, f2, &tl, &setc).expect("CVODE Goldstone");
    let fe = waves::goldstone_exact(&g2, &yfree, &tl);
    let e1 = waves::max_diff(&fr.states, &fe, 2 * n) / waves::max_abs(&fe, 2 * n);
    rep.add("P4-W6", &format!("the free Goldstone field (g = 0) by CVODE to t = 350 ({}) equals its exact solution (largest difference of theta and pi relative to their largest value)", fr.stats.describe()), e1, 1e-7);
    // the coupled field in the temporal gauge
    let ycpl = waves::initial_temporal(&g2, &d, &pk, GF, f2);
    let cp = waves::temporal(&g2, &d, &ycpl, GF, f2, &tl, &setc).expect("CVODE temporal gauge");
    let mut gauss: f64 = 0.0;
    let mut scale: f64 = 0.0;
    let mut de = vec![0.0; n];
    for y in &cp.states {
        spectral::matvec(&d, &y[3 * n..4 * n], &mut de);
        for j in 0..n {
            gauss = gauss.max((de[j] + 2.0 * GF * f2 * y[n + j]).abs());
            scale = scale.max((2.0 * GF * f2 * y[n + j]).abs());
        }
    }
    rep.add("P4-W7", &format!("temporal gauge, g = g_F, by CVODE ({}): Gauss's law D E + 2 g f^2 pi = 0 holds at all outputs (largest residual relative to the largest 2 g f^2 |pi|)", cp.stats.describe()), gauss / scale, 1e-8);
    let en_t: Vec<f64> = cp.states.iter().map(|y| waves::energy_density_temporal(&d, y, n, GF, f2).iter().sum::<f64>()).collect();
    let en_f: Vec<f64> = fr.states.iter().map(|y| waves::energy_density_temporal(&d, y, n, 0.0, f2).iter().sum::<f64>()).collect();
    let dt = en_t.iter().map(|e| (e - en_t[0]).abs()).fold(0.0, f64::max) / en_t[0];
    let dfree = en_f.iter().map(|e| (e - en_f[0]).abs()).fold(0.0, f64::max) / en_f[0];
    rep.add("P4-W8", "the energies of the free Goldstone field and of the temporal gauge are conserved (sum of the largest relative changes)", dt + dfree, 1e-8);
    // the unitary gauge by IDA from the same physical state
    let u0 = waves::to_unitary(&d, &ycpl, n, GF);
    let seti = IdaSettings::new(1e-10, 1e-13);
    let un = waves::unitary(&g2, &d, &u0, mz, &tl, &seti).expect("IDA unitary gauge");
    let ux = waves::unitary_exact(&g2, &d, &u0, mz, &tl);
    let ei = waves::max_diff(&un.states, &ux, 3 * n) / waves::max_abs(&ux, 3 * n);
    rep.add("P4-W9", &format!("unitary gauge by IDA ({}): (a_u, E, phi_u) equal the exact solution of the longitudinal Z (largest difference relative to the largest value)", un.stats.describe()), ei, 1e-6);
    let tu: Vec<Vec<f64>> = cp.states.iter().map(|y| waves::to_unitary(&d, y, n, GF)).collect();
    let eg = waves::max_diff(&un.states, &tu, 3 * n) / waves::max_abs(&tu, 3 * n);
    rep.add("P4-W10", "the two gauges describe the same physics: the gauge-invariant fields a - D theta/(2 g), E and pi/(2 g) of the temporal gauge (CVODE) equal a_u, E, phi_u of the unitary gauge (IDA) (largest difference relative to the largest value)", eg, 1e-6);
    let en_u: Vec<f64> = un.states.iter().map(|y| waves::energy_density_unitary(y, n, mz).iter().sum::<f64>()).collect();
    let du = en_u.iter().map(|e| (e - en_u[0]).abs()).fold(0.0, f64::max) / en_u[0];
    rep.add("P4-W11", &format!("the energy of the unitary gauge, sum (E^2 + M_Z^2 (a_u^2 + phi_u^2))/2, is conserved by IDA and equals that of the temporal gauge ({:.12e}, {:.12e}; sum of the relative change and the relative difference)", en_u[0], en_t[0]), du + ((en_u[0] - en_t[0]) / en_t[0]).abs(), 1e-6);
    // the centres of energy
    let xc_f: Vec<f64> = fr.states.iter().map(|y| waves::centroid(&g2, &waves::energy_density_temporal(&d, y, n, 0.0, f2))).collect();
    let xc_z: Vec<f64> = un.states.iter().map(|y| waves::centroid(&g2, &waves::energy_density_unitary(y, n, mz))).collect();
    let xc_x: Vec<f64> = ux.iter().map(|y| waves::centroid(&g2, &waves::energy_density_unitary(y, n, mz))).collect();
    let (vf, _) = signal::line_fit(&tl, &xc_f);
    let (vz, _) = signal::line_fit(&tl, &xc_z);
    let (vx, _) = signal::line_fit(&tl, &xc_x);
    let vg = pk.k0 / (pk.k0 * pk.k0 + mz * mz).sqrt();
    rep.add("P4-W12", &format!("the energy of the free Goldstone packet moves at the speed of light: slope of its centre {:.12} (difference from 1)", vf), (vf - 1.0).abs(), 1e-6);
    rep.add("P4-W13", &format!("the energy of the eaten Goldstone field, the longitudinal Z, moves at {:.9} (the exact solution: {:.9}; the group velocity k0/(k0^2 + M_Z^2)^(1/2) of the central wave number: {:.6}); difference from the exact", vz, vx, vg), (vz - vx).abs(), 1e-6);
    for (k, v) in [("goldstone_velocity", vf), ("Z_velocity", vz), ("Z_velocity_exact", vx), ("Z_group_velocity_k0", vg), ("energy_temporal", en_t[0]), ("energy_unitary", en_u[0])] {
        vals.push((format!("p4_waves_{}", k), v));
    }
    let mut rows = vec![];
    for (i, &t) in tl.iter().enumerate() {
        let ef = waves::energy_density_temporal(&d, &fr.states[i], n, 0.0, f2);
        let ez = waves::energy_density_unitary(&un.states[i], n, mz);
        for j in 0..n {
            rows.push(vec![f(t), f(g2.x[j]), e9(fr.states[i][j]), e9(ef[j]), e9(cp.states[i][j]), e9(un.states[i][n + j]), e9(ez[j]), e9(un.states[i][j]), e9(un.states[i][2 * n + j])]);
        }
    }
    write_csv(&data.join("part4_waves_longitudinal.csv"), &["t", "x", "theta_free", "energy_free", "theta_temporal", "E", "energy_Z", "a_u", "phi_u"], &rows);
}

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new("sf_part4: checks of part IV of superfluid.md (Revision_HS_CJT_and_Kohn-Sham, PROJECT spacetimes)");
    let mut vals: Vec<(String, f64)> = vec![];
    let p2 = read_values(&data.join("part2_values.csv"));
    let p3 = read_values(&data.join("part3_values.csv"));
    let _ = (d8(&beta()), C64::ZERO);
    integrator_tests(&mut rep);
    println!("integrator tests done ({:.0} s)", t0.elapsed().as_secs_f64());
    section14(&mut rep, &mut vals, &p3, &data);
    println!("section 14 done ({:.0} s)", t0.elapsed().as_secs_f64());
    section15_waves(&mut rep, &mut vals, &p3, &data);
    println!("section 15.2 done ({:.0} s)", t0.elapsed().as_secs_f64());
    section15_tdhfb(&mut rep, &mut vals, &p2, &data);
    println!("section 15.1 done ({:.0} s)", t0.elapsed().as_secs_f64());
    let vrows: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), format!("{:.12e}", v)]).collect();
    write_csv(&data.join("part4_values.csv"), &["name", "value"], &vrows);
    rep.note(&format!("run time {:.1} s", t0.elapsed().as_secs_f64()));
    rep.write(&data.join("hck_part4_checks.txt"));
    let (np, nn) = rep.counts();
    println!("{} of {} checks passed ({:.0} s)", np, nn, t0.elapsed().as_secs_f64());
    std::process::exit(if np == nn { 0 } else { 1 });
}
