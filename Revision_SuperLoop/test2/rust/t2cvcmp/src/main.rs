//! t2_cvcmp -- PROJECT spacetimes, Revision_SuperLoop, folder test2/cvode_vs_rk4: the comparison of the time integrator
//! CVODE (SUNDIALS 7.8.0 in pure Rust, module t2core::cvode) with the two fixed-step methods that the first version of
//! Test 2 used and that were replaced on the instruction of the author: the classical Runge-Kutta method of the fourth
//! order (RK4: t2_analytic, t2_fields, the task evolve of t2_solve, t2_sfbs) and the implicit midpoint rule (the task
//! quench of t2_solve).
//!
//! WHAT IS COMPARED
//!   (1) Three problems with an exact solution, computed here with both methods:
//!         AS-2  the isospin precession of a lattice plane wave in a constant isospin potential (check AS-2 of
//!               t2_analytic; periodic chain of 16 sites, 256 complex unknowns; up to x^0 = 16);
//!         AS-4  the particle-hole (Rabi) oscillation in the uniform polar superfluid (check AS-4; 16 sites; up to 20);
//!         LAT   the lattice operator with random links, potentials and pair fields on 3 x 2 x 4 sites (the problem of
//!               check GS-13 of t2_fields, with its own random fields; 384 complex unknowns; up to 20), whose exact
//!               solution is the spectral sum over the eigenpairs of the Hermitian matrix.
//!       RK4 is run with several fixed steps dt (among them the steps of the first version), CVODE with several relative
//!       tolerances rtol (atol = rtol/100).  For each run: the largest relative deviation of the state from the exact
//!       solution at the output times, the number of evaluations of the right side H_lat w (the cost; one RK4 step
//!       needs 4), the number of steps, and the largest change of the norm (which the exact evolution conserves).
//!   (2) The data files of the first version (folder test2/rk4_reference/data, written by the RK4 and implicit-midpoint
//!       programs) against the data files of the present version (folder test2/data, CVODE with rtol = 1e-12) and against
//!       CVODE with rtol = 1e-10 and 1e-13 (folder test2/cvode_vs_rk4/data/conv, written by run_convergence.sh): the
//!       time series of the four time-dependent examples of the solver, the numbers of their summaries, and the time
//!       series and the check reports of t2_sfbs (SB-10, SB-16).  The differences between the three CVODE runs measure
//!       how well CVODE is converged; the difference between the old file and CVODE is then the error of the old method.
//!   (3) Reproduction: the RK4 runs of (1) with the steps of the first version reproduce the columns P_up_rk4 and
//!       P_hole_rk4 of the archived files t2_A_precession.csv and t2_B_rabi.csv to their printed precision, so that the
//!       archive and this program describe the same method.
//!
//! OUTPUT (folder test2/cvode_vs_rk4/data): cmp_report.txt (the checks and the numbers), cmp_tables.md (the tables of
//! the provenance document, generated here so that no number is copied by hand), cmp_work_precision.csv (one row per
//! run of (1)), cmp_lat_deviation.csv (the deviation of the problem LAT against the time, for four runs),
//! cmp_series_differences.csv (one row per compared column of (2)).
//!
//! Build and run (from the folder Revision_SuperLoop/test2/rust, after run_convergence.sh):
//!     cargo build --release && ./target/release/t2_cvcmp
//! The program exits with status 1 if a check fails.
use std::fs;
use std::path::{Path, PathBuf};

use t2core::c64::{c, C64};
use t2core::cmat::eigh;
use t2core::cvode::{self, CvodeSettings};
use t2core::lattice::{self, Fields, Lattice, Mats};
use t2core::model::kpair;
use t2core::rng::Rng;

const PI: f64 = std::f64::consts::PI;

// ============================================================================ the rejected method, for the comparison only
/// One step of the classical Runge-Kutta method of the fourth order (RK4) for i dw/dx^0 = H_lat w with time-independent
/// fields: k1 = -i H w, k2 = -i H (w + dt/2 k1), k3 = -i H (w + dt/2 k2), k4 = -i H (w + dt k3),
/// w <- w + dt/6 (k1 + 2 k2 + 2 k3 + k4).  This is the method of the first version of Test 2 (it was the function
/// t2core::lattice::Lattice::rk4_step), copied here without change so that its numbers can be reproduced and compared;
/// no program of Test 2 uses it any more.
fn rk4_step(lat: &Lattice, fl: &Fields, mats: &Mats, w: &mut Vec<C64>, dt: f64) {
    let n = w.len();
    let mi = C64 { re: 0.0, im: -1.0 };
    let mut tmp = vec![C64::ZERO; n];
    let mut k1 = vec![C64::ZERO; n];
    let mut k2 = vec![C64::ZERO; n];
    let mut k3 = vec![C64::ZERO; n];
    let mut k4 = vec![C64::ZERO; n];
    lat.apply(fl, mats, w, &mut k1);
    for i in 0..n {
        k1[i] = mi * k1[i];
        tmp[i] = w[i] + k1[i].scale(0.5 * dt);
    }
    lat.apply(fl, mats, &tmp, &mut k2);
    for i in 0..n {
        k2[i] = mi * k2[i];
        tmp[i] = w[i] + k2[i].scale(0.5 * dt);
    }
    lat.apply(fl, mats, &tmp, &mut k3);
    for i in 0..n {
        k3[i] = mi * k3[i];
        tmp[i] = w[i] + k3[i].scale(dt);
    }
    lat.apply(fl, mats, &tmp, &mut k4);
    for i in 0..n {
        k4[i] = mi * k4[i];
        w[i] = w[i] + (k1[i] + k2[i].scale(2.0) + k3[i].scale(2.0) + k4[i]).scale(dt / 6.0);
    }
}

// ============================================================================ helpers
fn vnorm(v: &[C64]) -> f64 {
    v.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}

fn vdiff(a: &[C64], b: &[C64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (*x - *y).norm_sqr()).sum::<f64>().sqrt()
}

fn sci(x: f64) -> String {
    format!("{:.2e}", x)
}

fn chain_momentum(kq: usize, n: usize, ell: f64) -> f64 {
    let k = if kq <= n / 2 { kq as f64 } else { kq as f64 - n as f64 };
    2.0 * PI * k / (n as f64 * ell)
}

fn folder(rel: &str) -> PathBuf {
    // the crate is test2/rust/t2cvcmp; rel is relative to test2
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p.push(rel);
    p
}

/// a problem with an exact solution: the lattice, the fields, the initial state, the exact solution at a time, the
/// interval of the outputs and their number
struct Problem {
    name: &'static str,
    text: &'static str,
    lat: Lattice,
    fl: Fields,
    w0: Vec<C64>,
    exact: Box<dyn Fn(f64) -> Vec<C64>>,
    dt_out: f64,
    nout: usize,
}

/// the result of one run
#[derive(Clone)]
struct Run {
    problem: &'static str,
    method: String,
    param: f64,
    evals: i64,
    steps: i64,
    maxdev: f64,
    enddev: f64,
    maxnorm: f64,
    devs: Vec<f64>,
    // the state at the output times (kept for the reproduction of the archived columns)
    states: Vec<Vec<C64>>,
}

fn run_rk4(p: &Problem, dt: f64) -> Run {
    let mats = Mats::new();
    let per = (p.dt_out / dt).round() as usize;
    assert!((per as f64 * dt - p.dt_out).abs() < 1e-12, "dt must divide the output interval");
    let nsteps = per * p.nout;
    let mut w = p.w0.clone();
    let (mut maxdev, mut enddev, mut maxnorm) = (0.0f64, 0.0f64, 0.0f64);
    let mut devs = vec![];
    let mut states = vec![];
    for step in 0..=nsteps {
        if step % per == 0 {
            let t = step as f64 * dt;
            let ex = (p.exact)(t);
            let d = vdiff(&w, &ex) / vnorm(&ex);
            maxdev = maxdev.max(d);
            enddev = d;
            maxnorm = maxnorm.max((vnorm(&w) / vnorm(&p.w0) - 1.0).abs());
            devs.push(d);
            states.push(w.clone());
        }
        if step < nsteps {
            rk4_step(&p.lat, &p.fl, &mats, &mut w, dt);
        }
    }
    Run { problem: p.name, method: "RK4".into(), param: dt, evals: 4 * nsteps as i64, steps: nsteps as i64, maxdev, enddev, maxnorm, devs, states }
}

fn run_cvode(p: &Problem, rtol: f64) -> Run {
    let set = CvodeSettings::adams(rtol, rtol / 100.0);
    let touts = cvode::uniform_times(0.0, p.dt_out, p.nout);
    let (mut maxdev, mut enddev, mut maxnorm) = (0.0f64, 0.0f64, 0.0f64);
    let mut devs = vec![];
    let mut states = vec![];
    let n0 = vnorm(&p.w0);
    let st = cvode::evolve_lattice(&p.lat, &p.fl, &p.w0, &touts, &set, |_i, t, w| {
        let ex = (p.exact)(t);
        let d = vdiff(w, &ex) / vnorm(&ex);
        maxdev = maxdev.max(d);
        enddev = d;
        maxnorm = maxnorm.max((vnorm(w) / n0 - 1.0).abs());
        devs.push(d);
        states.push(w.to_vec());
    })
    .expect("CVODE");
    Run { problem: p.name, method: "CVODE".into(), param: rtol, evals: st.nfevals, steps: st.nsteps, maxdev, enddev, maxnorm, devs, states }
}

/// a CSV file of the programs of Test 2: the comment lines, the header and the rows
struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
}

fn read_table(path: &Path) -> Table {
    let text = fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {}", path.display(), e));
    let mut lines = text.lines().filter(|l| !l.starts_with('#'));
    let header: Vec<String> = lines.next().expect("header").split(',').map(|s| s.to_string()).collect();
    let rows = lines.filter(|l| !l.is_empty()).map(|l| l.split(',').map(|s| s.to_string()).collect()).collect();
    Table { header, rows }
}

impl Table {
    fn col(&self, name: &str) -> Vec<f64> {
        let i = self.header.iter().position(|h| h == name).unwrap_or_else(|| panic!("no column {}", name));
        self.rows.iter().map(|r| r[i].parse::<f64>().unwrap_or(f64::NAN)).collect()
    }
}

fn maxdiff(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "the compared columns have different lengths");
    a.iter().zip(b).fold(0.0f64, |m, (x, y)| m.max((x - y).abs()))
}

/// the number that follows the first occurrence of `key` in `text` (digits, sign, point and exponent)
fn num_after(text: &str, key: &str) -> f64 {
    let i = text.find(key).unwrap_or_else(|| panic!("'{}' not found", key)) + key.len();
    let s: String = text[i..].chars().take_while(|ch| ch.is_ascii_digit() || *ch == '.' || *ch == '-' || *ch == '+' || *ch == 'e').collect();
    let s = s.trim_end_matches(|ch| ch == '.' || ch == 'e');
    s.parse::<f64>().unwrap_or_else(|_| panic!("cannot read the number after '{}': '{}'", key, s))
}

/// the details (the text after "  -- ") of the line of a check report that starts with "[PASS] NAME " or "[FAIL] NAME "
fn check_line(text: &str, name: &str) -> String {
    let line = text
        .lines()
        .find(|l| (l.starts_with("[PASS]") || l.starts_with("[FAIL]")) && l[6..].trim_start().starts_with(&format!("{} ", name)))
        .unwrap_or_else(|| panic!("no line {}", name));
    match line.find("  -- ") {
        Some(i) => line[i + 5..].to_string(),
        None => line.to_string(),
    }
}

struct Report {
    lines: Vec<String>,
    ok: bool,
}

impl Report {
    fn add(&mut self, s: String) {
        println!("{}", s);
        self.lines.push(s);
    }
    fn check(&mut self, ok: bool, s: String) {
        let l = format!("[{}] {}", if ok { "PASS" } else { "FAIL" }, s);
        println!("{}", l);
        self.lines.push(l);
        self.ok &= ok;
    }
}

fn main() {
    let out = folder("cvode_vs_rk4/data");
    fs::create_dir_all(&out).unwrap();
    let mut rep = Report { lines: vec![], ok: true };
    let mut md = String::new();
    rep.add("Revision_SuperLoop, folder test2/cvode_vs_rk4, program `t2_cvcmp`: CVODE (SUNDIALS 7.8.0, pure Rust) against RK4 and the implicit midpoint rule of the first version of Test 2".into());
    let (gv, gf, mass) = (0.7, 0.45, 0.4);
    let act = [true, false, false];

    // ------------------------------------------------------------------------ problem AS-2
    let (np, ellp, mu_p, apre) = (16usize, 0.5, 0.2, 1.5);
    let lat_p = Lattice { n: [np, 1, 1], ell: ellp, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_p };
    let mut fl_p = Fields::trivial(np);
    for s in 0..np {
        fl_p.v0[s] = lattice::potential(gv, gf, &[apre, 0.0, 0.0], 0.0);
    }
    let qp = chain_momentum(2, np, ellp);
    let (e4p, v4p) = eigh(&lattice::h_lat_plane(&[qp, 0.0, 0.0], ellp, mass, 1.0, &act));
    let eps_p = e4p[3];
    let chi4p: Vec<C64> = (0..4).map(|a| v4p.get(a, 3)).collect();
    let omega_pr = gv * apre;
    let exact_a2 = move |t: f64| -> Vec<C64> {
        let mut w = vec![C64::ZERO; 16 * np];
        let ph_t = C64::cis(-(eps_p - mu_p) * t);
        let (cu, cd) = (c((0.5 * omega_pr * t).cos(), 0.0), c(0.0, (0.5 * omega_pr * t).sin()));
        for s in 0..np {
            let ph = ph_t * C64::cis(qp * ellp * s as f64);
            for a in 0..4 {
                w[16 * s + 2 * a] = chi4p[a] * ph * cu;
                w[16 * s + 2 * a + 1] = chi4p[a] * ph * cd;
            }
        }
        w
    };
    let as2 = Problem {
        name: "AS-2",
        text: "isospin precession of a lattice plane wave in the constant isospin potential Av_LL^1_0 = 1.5 (periodic chain of 16 sites, dx = 0.5, m = 0.4, mu = 0.2; 256 complex unknowns; outputs every 0.16 up to x^0 = 16)",
        lat: lat_p.clone(),
        fl: fl_p.clone(),
        w0: exact_a2(0.0),
        exact: Box::new(exact_a2),
        dt_out: 0.16,
        nout: 100,
    };
    // ------------------------------------------------------------------------ problem AS-4
    let (nb, ellb, mu_b, del_b) = (16usize, 0.5, 0.6, 0.3);
    let lat_b = Lattice { n: [nb, 1, 1], ell: ellb, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_b };
    let mut fl_b = Fields::trivial(nb);
    for s in 0..nb {
        fl_b.d[s] = [C64::ZERO, C64::ZERO, c(del_b, 0.0)];
    }
    let qb = chain_momentum(1, nb, ellb);
    let (e4b, v4b) = eigh(&lattice::h_lat_plane(&[qb, 0.0, 0.0], ellb, mass, 1.0, &act));
    let eps_b = e4b[3];
    let mut chi8b = vec![C64::ZERO; 8];
    for a in 0..4 {
        chi8b[2 * a] = v4b.get(a, 3);
    }
    let kchi8b = kpair(3).matvec(&chi8b);
    let xb = eps_b - mu_b;
    let e_b = (xb * xb + del_b * del_b).sqrt();
    let exact_b2 = move |t: f64| -> Vec<C64> {
        let cp = c((e_b * t).cos(), -(xb / e_b) * (e_b * t).sin());
        let ch = c(0.0, -(del_b / e_b) * (e_b * t).sin());
        let mut w = vec![C64::ZERO; 16 * nb];
        for s in 0..nb {
            let ph = C64::cis(qb * ellb * s as f64);
            for a in 0..8 {
                w[16 * s + a] = chi8b[a] * cp * ph;
                w[16 * s + 8 + a] = kchi8b[a] * ch * ph;
            }
        }
        w
    };
    let as4 = Problem {
        name: "AS-4",
        text: "particle-hole (Rabi) oscillation of a particle injected into the uniform polar superfluid dvec = (0, 0, 0.3) (periodic chain of 16 sites, dx = 0.5, m = 0.4, mu = 0.6; 256 complex unknowns; outputs every 0.2 up to x^0 = 20)",
        lat: lat_b.clone(),
        fl: fl_b.clone(),
        w0: exact_b2(0.0),
        exact: Box::new(exact_b2),
        dt_out: 0.2,
        nout: 100,
    };
    // ------------------------------------------------------------------------ problem LAT (random fields, spectral solution)
    let lat_l = Lattice { n: [3, 2, 4], ell: 0.7, periodic: [true, false, true], wilson: 1.0, mass, mu: 0.3 };
    let ns_l = lat_l.nsites();
    let mut rng = Rng(2026111801);
    let mut fl_l = Fields::trivial(ns_l);
    for s in 0..ns_l {
        for j in 0..3 {
            fl_l.links[s][j] = lattice::link(gv, gf, &[rng.unit(), rng.unit(), rng.unit()], rng.unit(), lat_l.ell);
        }
        fl_l.v0[s] = lattice::potential(gv, gf, &[rng.unit(), rng.unit(), rng.unit()], rng.unit());
        fl_l.d[s] = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
    }
    let n_l = lat_l.dim();
    let mut w0_l: Vec<C64> = (0..n_l).map(|_| c(rng.unit(), rng.unit())).collect();
    let nr = vnorm(&w0_l);
    for z in w0_l.iter_mut() {
        *z = z.scale(1.0 / nr);
    }
    let mats = Mats::new();
    let hl = lat_l.dense(&fl_l, &mats);
    let (e_l, v_l) = eigh(&hl);
    let hbound_l = lat_l.norm_bound(&fl_l);
    let rho_l = e_l.iter().fold(0.0f64, |m, x| m.max(x.abs()));
    let coef_l: Vec<C64> = (0..n_l).map(|k| (0..n_l).fold(C64::ZERO, |acc, r| acc + v_l.get(r, k).conj() * w0_l[r])).collect();
    let exact_l = move |t: f64| -> Vec<C64> {
        let mut w = vec![C64::ZERO; n_l];
        for k in 0..n_l {
            let a = coef_l[k] * C64::cis(-e_l[k] * t);
            for r in 0..n_l {
                w[r] += v_l.get(r, k) * a;
            }
        }
        w
    };
    let latp = Problem {
        name: "LAT",
        text: "the lattice operator with random links, potentials and pair fields on 3 x 2 x 4 sites (dx = 0.7, periodic in x^1 and x^3, walls in x^2, m = 0.4, mu = 0.3; 384 complex unknowns; random normalized initial state; outputs every 0.5 up to x^0 = 20; exact solution: the spectral sum)",
        lat: lat_l.clone(),
        fl: fl_l.clone(),
        w0: w0_l.clone(),
        exact: Box::new(exact_l),
        dt_out: 0.5,
        nout: 40,
    };
    rep.add(format!("problem LAT: spectral radius of H_lat {:.6}, the bound Hbound {:.6}; the time-step rule of the first version, dt Hbound <= 2.5, allowed dt <= {:.6}", rho_l, hbound_l, 2.5 / hbound_l));

    // ------------------------------------------------------------------------ (1) the runs
    let rtols = [1e-6, 1e-8, 1e-10, 1e-12, 1e-13];
    let plans: Vec<(&Problem, Vec<f64>, f64)> = vec![
        (&as2, vec![0.016, 0.008, 0.004, 0.002, 0.001], 0.004),
        (&as4, vec![0.1, 0.05, 0.02, 0.01, 0.005], 0.02),
        (&latp, vec![0.125, 0.1, 0.05, 0.025, 0.0125, 0.00625], 0.125),
    ];
    let mut runs: Vec<Run> = vec![];
    md += "<!-- generated by t2_cvcmp: the table of the runs on the three problems with an exact solution -->\n";
    md += "| problem | method | $\\ell^{\\rm t}_{\\rm LL}$ or $r^{\\rm tol}_{\\rm LL}$ | evaluations $N^{\\rm ev}_{\\rm LL}$ | steps $N^{\\rm st}_{\\rm LL}$ | largest $\\delta^{\\rm rel}_{\\rm LL}$ | $\\delta^{\\rm rel}_{\\rm LL}$ at the end | largest relative change of the norm |\n";
    // the numbers of dashes give the relative widths of the columns in the typeset table
    md += "|:--------|:---------------|-----------------:|----------:|--------:|---------:|---------:|-----------:|\n";
    for (p, dts, dt_old) in &plans {
        rep.add(format!("problem {}: {}", p.name, p.text));
        for &dt in dts {
            let r = run_rk4(p, dt);
            rep.add(format!(
                "  RK4   dt = {:<8} {:>8} evaluations {:>7} steps  largest deviation {}  at the end {}  relative norm change {}{}",
                dt, r.evals, r.steps, sci(r.maxdev), sci(r.enddev), sci(r.maxnorm),
                if dt == *dt_old { "  (the step of the first version)" } else { "" }
            ));
            md += &format!(
                "| {} | RK4{} | $\\ell^{{\\rm t}}_{{\\rm LL}}={}$ | {} | {} | {} | {} | {} |\n",
                p.name, if dt == *dt_old { " (first version)" } else { "" }, dt, r.evals, r.steps, sci(r.maxdev), sci(r.enddev), sci(r.maxnorm)
            );
            runs.push(r);
        }
        for &rt in &rtols {
            let r = run_cvode(p, rt);
            rep.add(format!(
                "  CVODE rtol = {:<6e} {:>8} evaluations {:>7} steps  largest deviation {}  at the end {}  relative norm change {}{}",
                rt, r.evals, r.steps, sci(r.maxdev), sci(r.enddev), sci(r.maxnorm),
                if rt == 1e-12 { "  (the setting of the programs)" } else { "" }
            ));
            md += &format!(
                "| {} | CVODE{} | $r^{{\\rm tol}}_{{\\rm LL}}=10^{{{}}}$ | {} | {} | {} | {} | {} |\n",
                p.name, if rt == 1e-12 { " (programs)" } else { "" }, rt.log10().round() as i32, r.evals, r.steps, sci(r.maxdev), sci(r.enddev), sci(r.maxnorm)
            );
            runs.push(r);
        }
    }
    md += "\n";
    let mut wp = String::from("# content: the runs of RK4 (fixed step dt) and CVODE (relative tolerance rtol, atol = rtol/100) on the three problems with an exact solution: the number of evaluations of the right side H_lat w, the number of steps, the largest relative deviation of the state from the exact solution at the output times, the deviation at the last output time, and the largest relative change of the norm (program t2_cvcmp)\nproblem,method,param,evaluations,steps,max_deviation,end_deviation,max_norm_change\n");
    for r in &runs {
        wp += &format!("{},{},{:e},{},{},{:.6e},{:.6e},{:.6e}\n", r.problem, r.method, r.param, r.evals, r.steps, r.maxdev, r.enddev, r.maxnorm);
    }
    fs::write(out.join("cmp_work_precision.csv"), wp).unwrap();
    // the deviation of LAT against the time for four runs
    let pick = |pr: &str, m: &str, par: f64| runs.iter().find(|r| r.problem == pr && r.method == m && (r.param - par).abs() < 1e-15 * par.max(1.0)).unwrap().clone();
    let l1 = pick("LAT", "RK4", 0.125);
    let l2 = pick("LAT", "RK4", 0.025);
    let l3 = pick("LAT", "CVODE", 1e-8);
    let l4 = pick("LAT", "CVODE", 1e-12);
    let mut ld = String::from("# content: the relative deviation of the state from the exact spectral solution against the time for the problem LAT (random fields on 3 x 2 x 4 sites): RK4 with dt = 0.125 (allowed by the time-step rule of the first version) and dt = 0.025, CVODE with rtol = 1e-8 and 1e-12 (program t2_cvcmp)\nx_LL^0,rk4_dt0.125,rk4_dt0.025,cvode_rtol1e-8,cvode_rtol1e-12\n");
    for k in 0..=latp.nout {
        ld += &format!("{:.4},{:.6e},{:.6e},{:.6e},{:.6e}\n", k as f64 * latp.dt_out, l1.devs[k], l2.devs[k], l3.devs[k], l4.devs[k]);
    }
    fs::write(out.join("cmp_lat_deviation.csv"), ld).unwrap();

    // the cost at equal accuracy: for each problem, the cheapest CVODE run that is at least as accurate as RK4 with the step of
    // the first version
    for (p, _, dt_old) in &plans {
        let r_old = pick(p.name, "RK4", *dt_old);
        let r_prog = pick(p.name, "CVODE", 1e-12);
        rep.add(format!(
            "{}: RK4 with the step of the first version (dt = {}): deviation {} with {} evaluations; CVODE as the programs use it (rtol = 1e-12): deviation {} with {} evaluations",
            p.name, dt_old, sci(r_old.maxdev), r_old.evals, sci(r_prog.maxdev), r_prog.evals
        ));
    }

    // ------------------------------------------------------------------------ (3) reproduction of the archived columns
    let ref_dir = folder("rk4_reference/data");
    let new_dir = folder("data");
    let conv_dir = folder("cvode_vs_rk4/data/conv");
    let p_up = |w: &[C64]| -> f64 {
        let tot: f64 = w.iter().map(|z| z.norm_sqr()).sum();
        let mut u = 0.0;
        for s in 0..np {
            for a in 0..4 {
                u += w[16 * s + 2 * a].norm_sqr();
            }
        }
        u / tot
    };
    let p_hole = |w: &[C64]| -> f64 {
        let tot: f64 = w.iter().map(|z| z.norm_sqr()).sum();
        let mut h = 0.0;
        for s in 0..nb {
            for a in 0..8 {
                h += w[16 * s + 8 + a].norm_sqr();
            }
        }
        h / tot
    };
    let old_prec = read_table(&ref_dir.join("t2_A_precession.csv"));
    let rk_a2 = pick("AS-2", "RK4", 0.004);
    let mine: Vec<f64> = rk_a2.states.iter().map(|w| p_up(w)).collect();
    let d_a2 = maxdiff(&mine, &old_prec.col("P_up_rk4"));
    rep.check(d_a2 < 5.1e-11, format!("the RK4 run of AS-2 with dt = 0.004 reproduces the column P_up_rk4 of the archived file t2_A_precession.csv at its 101 times (largest difference {}, allowed 5.1e-11: half a unit of the 10th decimal)", sci(d_a2)));
    let old_rabi = read_table(&ref_dir.join("t2_B_rabi.csv"));
    let rk_a4 = pick("AS-4", "RK4", 0.02);
    let mine: Vec<f64> = rk_a4.states.iter().map(|w| p_hole(w)).collect();
    let d_a4 = maxdiff(&mine, &old_rabi.col("P_hole_rk4"));
    rep.check(d_a4 < 5.1e-11, format!("the RK4 run of AS-4 with dt = 0.02 reproduces the column P_hole_rk4 of the archived file t2_B_rabi.csv at its 101 times (largest difference {}, allowed 5.1e-11)", sci(d_a4)));
    // the new files against the exact columns and against each other
    let new_prec = read_table(&new_dir.join("t2_A_precession.csv"));
    let new_rabi = read_table(&new_dir.join("t2_B_rabi.csv"));
    let e_old_a2 = maxdiff(&old_prec.col("P_up_rk4"), &old_prec.col("P_up_exact"));
    let e_new_a2 = maxdiff(&new_prec.col("P_up_cvode"), &new_prec.col("P_up_exact"));
    let e_old_a4 = maxdiff(&old_rabi.col("P_hole_rk4"), &old_rabi.col("P_hole_exact"));
    let e_new_a4 = maxdiff(&new_rabi.col("P_hole_cvode"), &new_rabi.col("P_hole_exact"));
    rep.add(format!("the probabilities of Figures 3(b) and 4(b): largest |P_up - exact| RK4 {}, CVODE {}; largest |P_hole - exact| RK4 {}, CVODE {} (both files print 10 decimals)", sci(e_old_a2), sci(e_new_a2), sci(e_old_a4), sci(e_new_a4)));
    md += "<!-- generated by t2_cvcmp: the probabilities of the figures of AS-2 and AS-4 -->\n";
    md += "| quantity (data file) | largest deviation from the exact value, RK4 (first version) | the same, CVODE ($r^{\\rm tol}_{\\rm LL}=10^{-12}$) |\n|:--|--:|--:|\n";
    md += &format!("| $P^\\uparrow_{{\\rm LL}}$ of AS-2 (`t2_A_precession.csv`) | {} | {} |\n", sci(e_old_a2), sci(e_new_a2));
    md += &format!("| $P^{{\\rm h}}_{{\\rm LL}}$ of AS-4 (`t2_B_rabi.csv`) | {} | {} |\n\n", sci(e_old_a4), sci(e_new_a4));

    // ------------------------------------------------------------------------ (2) the solver examples and t2_sfbs
    let mut sd = String::from("# content: the largest differences between the time series of the first version (RK4, implicit midpoint rule; test2/rk4_reference/data) and of CVODE with rtol = 1e-12 (test2/data), and between CVODE with rtol = 1e-10 or 1e-13 (test2/cvode_vs_rk4/data/conv) and CVODE with rtol = 1e-12, column by column (program t2_cvcmp)\nexample,column,old_minus_cvode12,cvode10_minus_cvode12,cvode13_minus_cvode12\n");
    md += "<!-- generated by t2_cvcmp: the time series of the examples of the solver -->\n";
    md += "| example | column | first version minus CVODE ($r^{\\rm tol}_{\\rm LL}=10^{-12}$) | CVODE $10^{-10}$ minus CVODE $10^{-12}$ | CVODE $10^{-13}$ minus CVODE $10^{-12}$ |\n|:--|:--|--:|--:|--:|\n";
    let series_cols: Vec<(&str, Vec<&str>)> = vec![
        ("packet_free", vec!["norm", "E_expect", "P_hole", "P_up", "x_mean_1"]),
        ("cyclotron", vec!["norm", "E_expect", "P_up", "x_mean_1", "x_mean_2"]),
        ("andreev", vec!["norm", "E_expect", "P_hole", "P_up", "x_mean_1"]),
        ("quench", vec!["abs_d_mean", "abs_d_site0", "N_F", "E_MF", "ortho_error"]),
    ];
    let mut converged_ok = true;
    for (ex, cols) in &series_cols {
        let old = read_table(&ref_dir.join(format!("t2_rs_{}_series.csv", ex)));
        let new = read_table(&new_dir.join(format!("t2_rs_{}_series.csv", ex)));
        let c10 = read_table(&conv_dir.join(format!("{}_rtol1e-10_series.csv", ex)));
        let c13 = read_table(&conv_dir.join(format!("{}_rtol1e-13_series.csv", ex)));
        let t_ok = maxdiff(&old.col("x_LL^0"), &new.col("x_LL^0")) < 1e-9 && maxdiff(&c10.col("x_LL^0"), &new.col("x_LL^0")) < 1e-9 && maxdiff(&c13.col("x_LL^0"), &new.col("x_LL^0")) < 1e-9;
        rep.check(t_ok, format!("example {}: the four time series have the same {} output times", ex, new.rows.len()));
        for col in cols {
            let a = maxdiff(&old.col(col), &new.col(col));
            let b = maxdiff(&c10.col(col), &new.col(col));
            let c13d = maxdiff(&c13.col(col), &new.col(col));
            sd += &format!("{},{},{:.6e},{:.6e},{:.6e}\n", ex, col, a, b, c13d);
            md += &format!("| {} | `{}` | {} | {} | {} |\n", ex, col, sci(a), sci(b), sci(c13d));
            rep.add(format!("  {} {}: first version - CVODE {}; CVODE 1e-10 - 1e-12 {}; CVODE 1e-13 - 1e-12 {}", ex, col, sci(a), sci(b), sci(c13d)));
            // CVODE converged: the change from rtol 1e-12 to 1e-13 is smaller than the difference to the first version,
            // except for quantities that the first version also got right to the printed precision
            if c13d > a.max(1e-9) {
                converged_ok = false;
            }
        }
    }
    md += "\n";
    rep.check(converged_ok, "the CVODE results are converged: for every compared column the change between rtol = 1e-12 and rtol = 1e-13 is smaller than the difference from the first version (or below 1e-9)".into());
    // the numbers of the summaries
    let summ = |dir: &Path, f: &str| fs::read_to_string(dir.join(f)).unwrap();
    let qo = summ(&ref_dir, "t2_rs_quench_summary.txt");
    let qn = summ(&new_dir, "t2_rs_quench_summary.txt");
    let q10 = summ(&conv_dir, "quench_rtol1e-10_summary.txt");
    let q13 = summ(&conv_dir, "quench_rtol1e-13_summary.txt");
    let freq = |t: &str| num_after(t, "oscillates with the angular frequency ");
    let meand = |t: &str| num_after(t, "mean of |d| over the run after the quench ");
    let ampl = |t: &str| num_after(t, "fitted amplitude ");
    let demf = |t: &str| num_after(t, " changes by at most ");
    md += "<!-- generated by t2_cvcmp: the quench -->\n";
    md += "| quantity of the example quench | implicit midpoint rule, $\\ell^{\\rm t}_{\\rm LL}=0.04$ (first version) | CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-10}$ | CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-12}$ (programs) | CVODE, $r^{\\rm tol}_{\\rm LL}=10^{-13}$ |\n|:--|--:|--:|--:|--:|\n";
    md += &format!("| angular frequency of the amplitude oscillation | {:.6} | {:.6} | {:.6} | {:.6} |\n", freq(&qo), freq(&q10), freq(&qn), freq(&q13));
    md += &format!("| mean of the gap after the quench | {:.10} | {:.10} | {:.10} | {:.10} |\n", meand(&qo), meand(&q10), meand(&qn), meand(&q13));
    md += &format!("| amplitude of the oscillation (fit) | {:.8} | {:.8} | {:.8} | {:.8} |\n", ampl(&qo), ampl(&q10), ampl(&qn), ampl(&q13));
    md += &format!("| largest change of the conserved functional $E^{{\\rm MF}}_{{\\rm LL}}$ | {} | {} | {} | {} |\n\n", sci(demf(&qo)), sci(demf(&q10)), sci(demf(&qn)), sci(demf(&q13)));
    rep.add(format!(
        "quench: frequency {:.6} (implicit midpoint) / {:.6}, {:.6}, {:.6} (CVODE 1e-10, 1e-12, 1e-13); mean of |d| {:.10} / {:.10}, {:.10}, {:.10}; amplitude {:.8} / {:.8}, {:.8}, {:.8}; largest change of E_MF {} / {}, {}, {}",
        freq(&qo), freq(&q10), freq(&qn), freq(&q13), meand(&qo), meand(&q10), meand(&qn), meand(&q13), ampl(&qo), ampl(&q10), ampl(&qn), ampl(&q13),
        sci(demf(&qo)), sci(demf(&q10)), sci(demf(&qn)), sci(demf(&q13))
    ));
    let conv_q = (freq(&q10) - freq(&qn)).abs() < 1e-6 && (freq(&q13) - freq(&qn)).abs() < 1e-6 && (meand(&q13) - meand(&qn)).abs() < 1e-8;
    rep.check(conv_q, "quench: the frequency agrees to 1e-6 and the mean of |d| to 1e-8 between the three CVODE tolerances (converged), while the implicit midpoint rule differs from them".into());
    // the examples check of the first version compared the frequency of the quench with the amplitude mode of the
    // linearized equations; the predicted value and the deviation of the first version
    let exo = summ(&ref_dir, "t2_rs_examples_checks.txt");
    let pred_old = num_after(&exo, "amplitude mode of the linearized equations at the mean: ");
    rep.add(format!("the examples check of the first version: amplitude mode of the linearized equations at the mean of the first version {:.5}; relative deviation of the implicit midpoint frequency {}; of the CVODE frequency from the same prediction {}", pred_old, sci(((freq(&qo) - pred_old) / pred_old).abs()), sci(((freq(&qn) - pred_old) / pred_old).abs())));
    // the evolve examples: the conservation and the results of the summaries
    md += "<!-- generated by t2_cvcmp: the conservation laws of the evolve examples -->\n";
    md += "| example | largest change of the norm, RK4 (first version) | the same, CVODE | largest change of the energy, RK4 | the same, CVODE | evaluations $N^{\\rm ev}_{\\rm LL}$, RK4 | the same, CVODE |\n|:--|--:|--:|--:|--:|--:|--:|\n";
    for ex in ["packet_free", "cyclotron", "andreev"] {
        let so = summ(&ref_dir, &format!("t2_rs_{}_summary.txt", ex));
        let sn = summ(&new_dir, &format!("t2_rs_{}_summary.txt", ex));
        let nold = num_after(&so, "the norm of the mode is conserved: largest change ");
        let nnew = num_after(&sn, "the norm of the mode is conserved: largest change ");
        let eold = num_after(&so, "is conserved (static fields): largest change ");
        let enew = num_after(&sn, "is conserved (static fields): largest change ");
        let steps_old = num_after(&so, "steps = ");
        // the line "CVODE: N steps, M evaluations of the right side, ..." of the new summary
        let evals_new = {
            let i = sn.find("CVODE: ").expect("the CVODE line of the summary");
            num_after(&sn[i..], "steps, ") as i64
        };
        md += &format!("| {} | {} | {} | {} | {} | {} | {} |\n", ex, sci(nold), sci(nnew), sci(eold), sci(enew), 4 * steps_old as i64, evals_new);
        rep.add(format!("{}: norm change RK4 {} / CVODE {}; energy change RK4 {} / CVODE {}; evaluations RK4 {} / CVODE {}", ex, sci(nold), sci(nnew), sci(eold), sci(enew), 4 * steps_old as i64, evals_new));
    }
    md += "\n";
    // t2_sfbs: SB-10 and SB-16
    let fo = summ(&ref_dir, "t2_sfbs_checks.txt");
    let fnw = summ(&new_dir, "t2_sfbs_checks.txt");
    let s10o = check_line(&fo, "SB-10");
    let s10n = check_line(&fnw, "SB-10");
    let s16o = check_line(&fo, "SB-16");
    let s16n = check_line(&fnw, "SB-16");
    let line10 = |t: &str| num_after(t, "line of d^3(x_LL^0) at ");
    let pole10 = |t: &str| num_after(t, "pole M_B ");
    let en10 = |t: &str| num_after(t, "largest change of the mean-field energy ");
    let l16a = |t: &str| num_after(t, "disp = 0.02: line ");
    let l16b = |t: &str| num_after(t, "disp = 0.01: line ");
    let en16 = |t: &str| num_after(t, "; energy ");
    md += "<!-- generated by t2_cvcmp: t2_sfbs -->\n";
    md += "| check | quantity | RK4 (first version) | CVODE ($r^{\\rm tol}_{\\rm LL}=10^{-12}$) | reference value |\n|:-------|:--------------------------|-------------:|-------------:|-----------:|\n";
    md += &format!("| SB-10 | spectral line of the pair field (pole $M^{{\\rm B}}_{{\\rm LL}}$ with the same nodes) | {:.9} | {:.9} | {:.9} |\n", line10(&s10o), line10(&s10n), pole10(&s10n));
    md += &format!("| SB-10 | relative deviation of the line from the pole | {} | {} | |\n", sci(((line10(&s10o) - pole10(&s10o)) / pole10(&s10o)).abs()), sci(((line10(&s10n) - pole10(&s10n)) / pole10(&s10n)).abs()));
    md += &format!("| SB-10 | largest relative change of the mean-field energy | {} | {} | 0 |\n", sci(en10(&s10o)), sci(en10(&s10n)));
    md += &format!("| SB-16 | line of the gap oscillation, displacement 0.02 | {:.8} | {:.8} | |\n", l16a(&s16o), l16a(&s16n));
    md += &format!("| SB-16 | line of the gap oscillation, displacement 0.01 | {:.8} | {:.8} | |\n", l16b(&s16o), l16b(&s16n));
    md += &format!("| SB-16 | largest relative change of the mean-field energy | {} | {} | 0 |\n\n", sci(en16(&s16o)), sci(en16(&s16n)));
    rep.add(format!("SB-10: line {:.9} (RK4) / {:.9} (CVODE), pole {:.9}; energy {} / {}. SB-16: lines {:.8}, {:.8} (RK4) / {:.8}, {:.8} (CVODE); energy {} / {}", line10(&s10o), line10(&s10n), pole10(&s10n), sci(en10(&s10o)), sci(en10(&s10n)), l16a(&s16o), l16b(&s16o), l16a(&s16n), l16b(&s16n), sci(en16(&s16o)), sci(en16(&s16n))));
    for (f, colname) in [("t2_sfbs_tdbdg_normal.csv", "d_over_d0"), ("t2_sfbs_tdbdg_higgs.csv", "d_over_dvac")] {
        let a = read_table(&ref_dir.join(f));
        let b = read_table(&new_dir.join(f));
        let dmax = maxdiff(&a.col(colname), &b.col(colname));
        let tmax = maxdiff(&a.col("x_LL^0"), &b.col("x_LL^0"));
        rep.add(format!("{}: same {} times (largest difference of the times {}); largest difference of {} between RK4 and CVODE {}", f, b.rows.len(), sci(tmax), colname, sci(dmax)));
        sd += &format!("{},{},{:.6e},,\n", f, colname, dmax);
    }
    fs::write(out.join("cmp_series_differences.csv"), sd).unwrap();
    fs::write(out.join("cmp_tables.md"), md).unwrap();
    let mut text = rep.lines.join("\n");
    text.push('\n');
    fs::write(out.join("cmp_report.txt"), text).unwrap();
    println!("wrote test2/cvode_vs_rk4/data/cmp_report.txt, cmp_tables.md, cmp_work_precision.csv, cmp_lat_deviation.csv, cmp_series_differences.csv");
    if !rep.ok {
        std::process::exit(1);
    }
}
