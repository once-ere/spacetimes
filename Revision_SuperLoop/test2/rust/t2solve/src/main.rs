//! t2_solve -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, item [5].
//!
//! The Rust solver of the gauge-covariant mean-field field equations of the self-interacting spinor field on the
//! lattice of Section 2.11 of the document of Test 2: the same input files (format "t2in 2"), the same four tasks
//! (evolve, spectrum, ground_state, quench), the same methods and the same output files as the Python solver
//! test2/python/t2_solve.py of item [3].  Pure Rust: only the standard library and the library t2core of the workspace
//! (the matrices of the super-loop, the lattice operator, the Hermitian eigensolver, the time integrator CVODE of
//! SUNDIALS 7.8.0 in pure Rust).
//!
//! Build (from the folder test2/rust):   cargo build --release
//! Use:
//!   ./target/release/t2_solve                        guidance, then the program asks for the input file
//!   ./target/release/t2_solve FILE.t2in              solve the problem of FILE.t2in
//!   ./target/release/t2_solve FILE.t2in --out BASE   the same, with the output files BASE_*
//!   ./target/release/t2_solve --guide                the complete guide to the input file
//!   ./target/release/t2_solve --examples             the list of the examples
//!   ./target/release/t2_solve --example NAME [FILE]  write the example NAME as an input file
//!   ./target/release/t2_solve --selftest [REPORT [EXAMPLES_FOLDER]]
//!                                                    check the sparse operator against t2core's Lattice::apply, the
//!                                                    links, plane waves, and the examples against the files of the folder
mod input;
mod ops;

use input::{example_text, guide_text, read_input, Cfg, EXAMPLES};
use ops::*;
use std::fs;
use std::io::{self, BufRead, Write as IoWrite};
use std::path::Path;
use std::time::Instant;
use t2core::c64::{c, C64};
use t2core::cmat::{eigh, CMat};
use t2core::cvode::{self, ComplexRhs, CvodeSettings};
use t2core::lattice::{link, potential, Fields, Lattice, Mats};
use t2core::rng::Rng;

const PROGRAM: &str = "t2_solve";

// ============================================================================ output
/// rewrite the exponents of Rust's "{:e}" format ("1.5e-7", "0.0e0") as Python writes them ("1.5e-07", "0.0e+00"),
/// so that the reports of the two solvers read alike
fn py_exponents(s: &str) -> String {
    let b: Vec<char> = s.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    while i < b.len() {
        if b[i] == 'e' && i > 0 && b[i - 1].is_ascii_digit() {
            let mut j = i + 1;
            let neg = j < b.len() && b[j] == '-';
            if neg {
                j += 1;
            }
            let k0 = j;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            if j > k0 && (j == b.len() || !b[j].is_ascii_alphanumeric()) {
                let digits: String = b[k0..j].iter().collect();
                out += &format!("e{}{:0>2}", if neg { '-' } else { '+' }, digits);
                i = j;
                continue;
            }
        }
        out.push(b[i]);
        i += 1;
    }
    out
}

/// Python's "%.{p}g": p significant digits, fixed or scientific notation, trailing zeros removed
fn fmt_g(x: f64, p: usize) -> String {
    if x == 0.0 {
        return if x.is_sign_negative() { "-0".into() } else { "0".into() };
    }
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf".into() } else { "-inf".into() };
    }
    let s = format!("{:.*e}", p - 1, x);
    let (mant, ex) = s.split_once('e').unwrap();
    let e: i32 = ex.parse().unwrap();
    if e < -4 || e >= p as i32 {
        let mut m = mant.to_string();
        if m.contains('.') {
            m = m.trim_end_matches('0').trim_end_matches('.').to_string();
        }
        format!("{}e{}{:02}", m, if e < 0 { '-' } else { '+' }, e.abs())
    } else {
        let mut f = format!("{:.*}", (p as i32 - 1 - e).max(0) as usize, x);
        if f.contains('.') {
            f = f.trim_end_matches('0').trim_end_matches('.').to_string();
        }
        f
    }
}

fn g12(x: f64) -> String {
    fmt_g(x, 12)
}

fn g6(x: f64) -> String {
    fmt_g(x, 6)
}

enum Cell {
    F(f64),
    I(usize),
    S(String),
}

fn write_csv(path: &str, content: &str, meta: &[(String, String)], header: &[&str], rows: &[Vec<Cell>]) {
    let mut s = format!("# content: {}\n", content);
    for (k, v) in meta {
        s += &format!("# {}: {}\n", k, v);
    }
    s += &header.join(",");
    s.push('\n');
    for r in rows {
        let cells: Vec<String> = r
            .iter()
            .map(|x| match x {
                Cell::F(v) => g12(*v),
                Cell::I(i) => i.to_string(),
                Cell::S(t) => t.clone(),
            })
            .collect();
        s += &cells.join(",");
        s.push('\n');
    }
    fs::write(path, s).unwrap_or_else(|e| panic!("cannot write {}: {}", path, e));
}

/// the summary file: the problem, the checks and the results
struct Report {
    lines: Vec<String>,
    checks: Vec<(bool, String)>,
}

impl Report {
    fn new(cfg: &Cfg, lat: &Lat) -> Report {
        let mut r = Report { lines: vec![], checks: vec![] };
        let title = cfg.s("title");
        r.add(format!("Revision_SuperLoop, folder test2, program `{}` (Test 2, item 5): {}", PROGRAM, if title.is_empty() { "(no title)".into() } else { title }));
        r.add(format!("input file: {}", Path::new(&cfg.path).file_name().unwrap().to_string_lossy()));
        r.add(format!("task: {}", cfg.s("task")));
        let b = cfg.w("boundary");
        r.add(format!(
            "lattice: sites {} {} {}, dx = {}, boundary {}, r_W = {}, active directions D = {}, number of sites {}, dimension of H_lat {}",
            lat.l.n[0], lat.l.n[1], lat.l.n[2], g6(lat.l.ell), b.join(" "), g6(lat.l.wilson), lat.nact, lat.ns, 16 * lat.ns
        ));
        r.add(format!("parameters: m = {}, mu = {}, g_v = {}, g_F = {}", g6(lat.l.mass), g6(lat.l.mu), g6(cfg.f("g_v")), g6(cfg.f("g_F"))));
        r
    }
    fn add(&mut self, s: String) {
        self.lines.push(py_exponents(&s));
    }
    fn check(&mut self, ok: bool, text: String) {
        let text = py_exponents(&text);
        println!("[{}] {}", if ok { "PASS" } else { "FAIL" }, text);
        self.checks.push((ok, text));
    }
    fn write(&self, path: &str) -> bool {
        let mut s = format!("{}\n", self.lines[0]);
        let npass = self.checks.iter().filter(|c| c.0).count();
        s += &format!("{} of {} checks passed\n", npass, self.checks.len());
        for l in &self.lines[1..] {
            s += l;
            s.push('\n');
        }
        s += "checks:\n";
        for (ok, t) in &self.checks {
            s += &format!("[{}] {}\n", if *ok { "PASS" } else { "FAIL" }, t);
        }
        fs::write(path, s).expect("summary");
        npass == self.checks.len()
    }
}

fn meta_lines(cfg: &Cfg, lat: &Lat, gauge: &Gauge) -> Vec<(String, String)> {
    vec![
        ("program".into(), PROGRAM.into()),
        ("task".into(), cfg.s("task")),
        ("sites".into(), format!("{} {} {}", lat.l.n[0], lat.l.n[1], lat.l.n[2])),
        ("dx".into(), g6(lat.l.ell)),
        ("boundary".into(), cfg.w("boundary").join(" ")),
        ("r_W".into(), g6(lat.l.wilson)),
        ("m".into(), g6(lat.l.mass)),
        ("mu".into(), g6(lat.l.mu)),
        ("g_v".into(), g6(cfg.f("g_v"))),
        ("g_F".into(), g6(cfg.f("g_F"))),
        ("gauge".into(), gauge.desc.clone()),
    ]
}

// ============================================================================ small helpers
fn norm2(w: &[C64]) -> f64 {
    w.iter().map(|z| z.norm_sqr()).sum()
}

fn vdot_re(a: &[C64], b: &[C64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| x.re * y.re + x.im * y.im).sum()
}

fn absd(d: &[C64; 3]) -> f64 {
    d.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}

/// the eigenvalues and eigenvectors of a dense Hermitian matrix (t2core's eigensolver)
fn diagonalize(a: &CMat) -> (Vec<f64>, CMat) {
    eigh(a)
}

/// the columns of an eigenvector matrix with positive eigenvalues, row by row (n rows of mm columns)
fn positive_modes(ev: &[f64], vec: &CMat) -> (Vec<C64>, usize) {
    let n = ev.len();
    let cols: Vec<usize> = (0..n).filter(|&k| ev[k] > 0.0).collect();
    let mm = cols.len();
    let mut w = vec![C64::ZERO; n * mm];
    for r in 0..n {
        for (ci, &k) in cols.iter().enumerate() {
            w[r * mm + ci] = vec.get(r, k);
        }
    }
    (w, mm)
}

fn add_pair_dense(a: &mut CMat, m: &M, d: &[[C64; 3]]) {
    for (s, ds) in d.iter().enumerate() {
        let p = pair_block(m, ds);
        for i in 0..8 {
            for j in 0..8 {
                if p[i][j] != C64::ZERO {
                    a.set(16 * s + i, 16 * s + 8 + j, a.get(16 * s + i, 16 * s + 8 + j) + p[i][j]);
                    a.set(16 * s + 8 + j, 16 * s + i, a.get(16 * s + 8 + j, 16 * s + i) + p[i][j].conj());
                }
            }
        }
    }
}

// ============================================================================ task evolve
fn task_evolve(cfg: &Cfg, lat: &Lat, gauge: &Gauge, d: &[[C64; 3]], m: &M, rep: &mut Report, base: &str) -> Result<(), String> {
    let ns = lat.ns;
    let build = |t: Option<f64>| build_h(lat, &gauge.fields_at(t), m, Some(d));
    let h_static = build(None);
    let herm = h_static.hermiticity();
    rep.check(herm < 1e-13, format!("the lattice operator H_lat is Hermitian (largest deviation {:.1e})", herm));
    // the output grid: the time series at x^0 = k output_step (k = 0 .. outputs), the fields every fields_every outputs;
    // CVODE chooses its own internal steps between the output times
    let ostep = cfg.f("output_step");
    let nout = cfg.i("outputs") as usize;
    let mut every = cfg.i("fields_every") as usize;
    if every == 0 {
        every = (nout / 50).max(1);
    }
    let set = CvodeSettings::adams(cfg.f("rtol"), cfg.f("atol"));
    let hb = norm_bound(lat, gauge.vmax(), d, 1.0);
    rep.add(format!(
        "time integration: {}; output every {} up to x^0 = {} ({} outputs), fields every {} outputs; Hbound = {}",
        set.describe(), g6(ostep), g6(ostep * nout as f64), nout, every, g6(hb)
    ));
    let w: Vec<C64> = match cfg.s("initial").as_str() {
        "packet" => {
            let q = cfg.v("packet_momentum");
            rep.add(format!(
                "initial mode: Gaussian packet, centre {:?}, width {}, momentum {:?}, branch {}, spin {}, isospin {}, {}; eps_lat(q) = {}",
                if cfg.has("packet_center") { cfg.v("packet_center") } else { vec![] },
                g6(cfg.f("packet_width")),
                q,
                cfg.s("packet_branch"),
                cfg.s("packet_spin"),
                cfg.s("packet_isospin"),
                cfg.s("packet_part"),
                fmt_g(eps_lat_q(lat, &[q[0], q[1], q[2]]), 10)
            ));
            packet_mode(cfg, lat)
        }
        "eigenmode" => {
            if 16 * ns > 6000 {
                return Err("initial = eigenmode needs a small lattice (16 x sites <= 6000)".into());
            }
            let (ev, vec) = diagonalize(&h_static.dense());
            let k = cfg.i("eigenmode_index");
            if k < 0 || k as usize >= ev.len() {
                return Err(format!("eigenmode_index must lie in 0 .. {}", ev.len() - 1));
            }
            rep.add(format!("initial mode: eigenvector number {} of H_lat, E_qp = {}", k, g12(ev[k as usize])));
            (0..16 * ns).map(|r| vec.get(r, k as usize)).collect()
        }
        _ => {
            let name = cfg.s("initial_file");
            let p = if Path::new(&name).is_absolute() { Path::new(&name).to_path_buf() } else { cfg.dir.join(&name) };
            let text = fs::read_to_string(&p).map_err(|e| format!("cannot read '{}': {}", p.display(), e))?;
            let vals: Vec<Vec<f64>> = text
                .lines()
                .map(|l| l.split('#').next().unwrap_or("").trim().to_string())
                .filter(|l| !l.is_empty())
                .map(|l| l.split_whitespace().filter_map(|x| x.parse().ok()).collect())
                .collect();
            if vals.len() != 16 * ns || vals.iter().any(|r| r.len() != 2) {
                return Err(format!("the initial file must have {} lines 're im'", 16 * ns));
            }
            let mut w: Vec<C64> = vals.iter().map(|r| c(r[0], r[1])).collect();
            let nr = norm2(&w).sqrt();
            for z in w.iter_mut() {
                *z = z.scale(1.0 / nr);
            }
            rep.add(format!("initial mode: from the file {}", name));
            w
        }
    };
    let n = 16 * ns;
    let mut hw = vec![C64::ZERO; n];
    let mut series: Vec<Vec<Cell>> = vec![];
    let mut fields: Vec<Vec<Cell>> = vec![];
    // the operators at the times of the stages (all equal for static fields)
    let get = |t: f64| -> Csr {
        if gauge.is_static {
            return build(None);
        }
        build(Some(t))
    };
    let energy = |h: &Csr, w: &[C64], hw: &mut Vec<C64>| -> f64 {
        h.matvec(w, hw);
        vdot_re(w, hw)
    };
    let h0 = if gauge.is_static { h_static } else { get(0.0) };
    let e0 = energy(&h0, &w, &mut hw);
    let (mut dn, mut de) = (0.0f64, 0.0f64);
    let mut record = |t: f64, w: &[C64], h: &Csr, with_fields: bool, series: &mut Vec<Vec<Cell>>, fields: &mut Vec<Vec<Cell>>, hw: &mut Vec<C64>| {
        let mut rp = vec![0.0; ns];
        let mut rh = vec![0.0; ns];
        let mut rup = vec![0.0; ns];
        for s in 0..ns {
            for a in 0..8 {
                rp[s] += w[16 * s + a].norm_sqr();
                rh[s] += w[16 * s + 8 + a].norm_sqr();
            }
            for a in (0..8).step_by(2) {
                rup[s] += w[16 * s + a].norm_sqr();
            }
        }
        let nrm: f64 = rp.iter().sum::<f64>() + rh.iter().sum::<f64>();
        let e = energy(h, w, hw);
        let tot: Vec<f64> = (0..ns).map(|s| rp[s] + rh[s]).collect();
        let cm = center_of_mass(lat, &tot);
        series.push(vec![
            Cell::F(t),
            Cell::F(nrm),
            Cell::F(e),
            Cell::F(rh.iter().sum::<f64>() / nrm),
            Cell::F(rup.iter().sum::<f64>() / nrm),
            Cell::F(cm[0]),
            Cell::F(cm[1]),
            Cell::F(cm[2]),
        ]);
        if with_fields {
            for s in 0..ns {
                fields.push(vec![Cell::F(t), Cell::F(lat.x[s][0]), Cell::F(lat.x[s][1]), Cell::F(lat.x[s][2]), Cell::F(rp[s]), Cell::F(rh[s]), Cell::F(rup[s])]);
            }
        }
        dn = dn.max((nrm - 1.0).abs());
        de = de.max((e - e0).abs());
    };
    let t0 = Instant::now();
    // the right side of CVODE, dw/dx^0 = -i H_lat(x^0) w: for static fields the operator is built once, with a pulse it is
    // built at the time of every evaluation; the closure owns copies of what it needs (CVODE keeps it)
    let (lat_c, gauge_c, d_c, m_c, h0_c) = (lat.clone(), gauge.clone(), d.to_vec(), m.clone(), h0.clone());
    let f: ComplexRhs = Box::new(move |t, wv, dw| {
        let h_t;
        let h: &Csr = if gauge_c.is_static {
            &h0_c
        } else {
            h_t = build_h(&lat_c, &gauge_c.fields_at(Some(t)), &m_c, Some(&d_c));
            &h_t
        };
        h.matvec(wv, dw);
        for z in dw.iter_mut() {
            *z = c(z.im, -z.re);
        }
    });
    let touts = cvode::uniform_times(0.0, ostep, nout);
    let st = cvode::integrate(&w, 0.0, &touts, &set, f, |i, t, wv| {
        let hnow = if gauge.is_static { None } else { Some(get(t)) };
        record(t, wv, hnow.as_ref().unwrap_or(&h0), i % every == 0 || i == nout, &mut series, &mut fields, &mut hw);
    })?;
    let el = t0.elapsed().as_secs_f64();
    rep.add(format!("CVODE: {}", st.describe()));
    // the accumulated local error that the error test of CVODE permits, steps sqrt(2) (rtol ||w|| + atol sqrt(n)) with
    // ||w|| = 1 and n complex unknowns (t2core::cvode::CvodeSettings::error_budget).  CVODE estimates the local error of a
    // step from the difference of the predicted and the corrected solution; the true error may exceed the estimate by a
    // modest factor, and the checks allow 10 times the budget: the norm of the mode, which the exact evolution conserves,
    // may change by at most 10 budget, the energy <w_lat|H_lat|w_lat> (static fields) by 2 Hbound times that
    let budget = set.error_budget(st.nsteps, n, 1.0);
    rep.check(dn < 10.0 * budget, format!("the norm of the mode is conserved: largest change {:.2e} (bound: 10 times the accumulated local error permitted by CVODE, 10 steps sqrt(2) (rtol + atol sqrt(n_complex)) = {:.1e})", dn, 10.0 * budget));
    if gauge.is_static {
        let tol = 20.0 * hb * budget;
        rep.check(de < tol, format!("the energy <w_lat|H_lat|w_lat> = {} is conserved (static fields): largest change {:.2e} (bound: 2 Hbound times the bound of the norm, {:.1e})", fmt_g(e0, 10), de, tol));
    }
    let last = series.last().unwrap();
    let val = |k: usize| match last[k] {
        Cell::F(v) => v,
        _ => 0.0,
    };
    rep.add(format!(
        "results: energy <w_lat|H_lat|w_lat> at x^0 = 0: {}; final norm {}; final P_hole = {}; final P_up = {}; final centre of the density ({}, {}, {}); computing time {:.2} s",
        g12(e0), g12(val(1)), g6(val(3)), g6(val(4)), g6(val(5)), g6(val(6)), g6(val(7)), el
    ));
    if cfg.s("initial") == "packet" && cfg.s("pair") == "none" && gauge.is_free() && (0..3).all(|j| !lat.active[j] || lat.l.periodic[j]) {
        let w0 = packet_mode(cfg, lat);
        let vexp = expected_velocity(&w0, cfg.s("packet_part") == "hole", lat);
        let tt = val(0);
        let first = &series[0];
        let f0 = |k: usize| match first[k] {
            Cell::F(v) => v,
            _ => 0.0,
        };
        let mut moved = [0.0; 3];
        for j in 0..3 {
            if lat.active[j] {
                let l = lat.l.n[j] as f64 * lat.l.ell;
                let dx = (val(5 + j) - f0(5 + j) + 0.5 * l).rem_euclid(l) - 0.5 * l;
                moved[j] = dx / tt;
            }
        }
        let q = cfg.v("packet_momentum");
        let g0 = grad_eps(lat, &[q[0], q[1], q[2]]);
        let dv = (0..3).map(|j| (moved[j] - vexp[j]).abs()).fold(0.0, f64::max);
        let vs = vexp.iter().map(|v| v.abs()).fold(0.0, f64::max);
        rep.check(
            dv < 1e-5 * vs,
            format!(
                "the free packet moves with its mean group velocity: expected [{:.8}, {:.8}, {:.8}] (the group velocity at the central momentum is [{:.8}, {:.8}, {:.8}]), measured [{:.8}, {:.8}, {:.8}]; deviation {:.2e} (allowed: 1e-5 of the velocity)",
                vexp[0], vexp[1], vexp[2], g0[0], g0[1], g0[2], moved[0], moved[1], moved[2], dv
            ),
        );
    }
    let meta = meta_lines(cfg, lat, gauge);
    write_csv(
        &format!("{}_series.csv", base),
        &format!("time series of the mode: norm, energy, probability of the hole part, probability of the particle part with isospin up, centre of the density (task evolve, {})", PROGRAM),
        &meta,
        &["x_LL^0", "norm", "E_expect", "P_hole", "P_up", "x_mean_1", "x_mean_2", "x_mean_3"],
        &series,
    );
    write_csv(
        &format!("{}_fields.csv", base),
        &format!("the densities of the particle part, of the hole part and of the particle part with isospin up on the sites at the output times (task evolve, {})", PROGRAM),
        &meta,
        &["x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3", "density_p", "density_h", "density_up"],
        &fields,
    );
    Ok(())
}

// ============================================================================ task spectrum
fn task_spectrum(cfg: &Cfg, lat: &Lat, gauge: &Gauge, d: &[[C64; 3]], m: &M, rep: &mut Report, base: &str) -> Result<(), String> {
    let ns = lat.ns;
    if 16 * ns > 6000 {
        return Err("the task spectrum diagonalizes a dense matrix: 16 x (number of sites) must not exceed 6000".into());
    }
    let h = build_h(lat, &gauge.fields_at(None), m, Some(d));
    let herm = h.hermiticity();
    rep.check(herm < 1e-13, format!("the lattice operator H_lat is Hermitian (largest deviation {:.1e})", herm));
    let t0 = Instant::now();
    let (ev, vec) = diagonalize(&h.dense());
    let el = t0.elapsed().as_secs_f64();
    let nn = ev.len();
    let phs = (0..nn).map(|k| (ev[k] + ev[nn - 1 - k]).abs()).fold(0.0, f64::max);
    rep.check(phs < 1e-10, format!("the spectrum is symmetric about zero (particle-hole symmetry; largest |E_n + E_(N-1-n)| = {:.1e})", phs));
    let win = cfg.v("spectrum_window");
    let (lo, hi) = (win[0], win[1]);
    let inwin: Vec<usize> = (0..nn).filter(|&k| ev[k] >= lo && ev[k] <= hi).collect();
    let sel: Vec<usize> = inwin.iter().cloned().take(cfg.i("spectrum_max_states") as usize).collect();
    rep.add(format!(
        "results: {} eigenvalues from {} to {}; {} eigenvalues in the window [{}, {}]: {}; computing time {:.2} s",
        nn,
        fmt_g(ev[0], 10),
        fmt_g(ev[nn - 1], 10),
        inwin.len(),
        g6(lo),
        g6(hi),
        inwin.iter().map(|&k| fmt_g(ev[k], 10)).collect::<Vec<_>>().join(" "),
        el
    ));
    let meta = meta_lines(cfg, lat, gauge);
    write_csv(
        &format!("{}_spectrum.csv", base),
        &format!("all eigenvalues of the lattice operator (task spectrum, {})", PROGRAM),
        &meta,
        &["index", "E_qp"],
        &(0..nn).map(|k| vec![Cell::I(k), Cell::F(ev[k])]).collect::<Vec<_>>(),
    );
    // groups of degenerate eigenvalues (equal within 1e-9): the summed densities do not depend on the eigenbasis
    let mut groups: Vec<Vec<usize>> = vec![];
    for &k in &sel {
        if let Some(gr) = groups.last_mut() {
            if (ev[k] - ev[*gr.last().unwrap()]).abs() < 1e-9 {
                gr.push(k);
                continue;
            }
        }
        groups.push(vec![k]);
    }
    let mut rows = vec![];
    for gr in &groups {
        let e = gr.iter().map(|&k| ev[k]).sum::<f64>() / gr.len() as f64;
        for s in 0..ns {
            let (mut dp, mut dh) = (0.0, 0.0);
            for &k in gr {
                for a in 0..8 {
                    dp += vec.get(16 * s + a, k).norm_sqr();
                    dh += vec.get(16 * s + 8 + a, k).norm_sqr();
                }
            }
            rows.push(vec![Cell::I(gr[0]), Cell::F(e), Cell::I(gr.len()), Cell::F(lat.x[s][0]), Cell::F(lat.x[s][1]), Cell::F(lat.x[s][2]), Cell::F(dp), Cell::F(dh)]);
        }
    }
    let mut meta2 = meta.clone();
    meta2.push(("spectrum_window".into(), format!("{} {}", g6(lo), g6(hi))));
    write_csv(
        &format!("{}_fields.csv", base),
        &format!("the densities of the eigenvectors in the energy window, summed over each group of degenerate eigenvalues (equal within 1e-9; the sum does not depend on the basis that the eigensolver chooses in a degenerate eigenspace) (task spectrum, {})", PROGRAM),
        &meta2,
        &["index", "E_qp", "multiplicity", "x_LL^1", "x_LL^2", "x_LL^3", "density_p", "density_h"],
        &rows,
    );
    write_csv(
        &format!("{}_series.csv", base),
        &format!("the pair field on the sites (task spectrum, {})", PROGRAM),
        &meta,
        &["x_LL^1", "x_LL^2", "x_LL^3", "d1_re", "d1_im", "d2_re", "d2_im", "d3_re", "d3_im", "abs_d"],
        &(0..ns)
            .map(|s| {
                vec![
                    Cell::F(lat.x[s][0]),
                    Cell::F(lat.x[s][1]),
                    Cell::F(lat.x[s][2]),
                    Cell::F(d[s][0].re),
                    Cell::F(d[s][0].im),
                    Cell::F(d[s][1].re),
                    Cell::F(d[s][1].im),
                    Cell::F(d[s][2].re),
                    Cell::F(d[s][2].im),
                    Cell::F(absd(&d[s])),
                ]
            })
            .collect::<Vec<_>>(),
    );
    Ok(())
}

// ============================================================================ the self-consistent ground state
struct Gs {
    d: Vec<[C64; 3]>,
    ev: Vec<f64>,
    vec: CMat,
    its: Vec<(usize, f64, f64, f64)>,
}

fn ground_state(cfg: &Cfg, lat: &Lat, gauge: &Gauge, d0: &[[C64; 3]], m: &M, rep: &mut Report, g: f64) -> Result<Gs, String> {
    let ns = lat.ns;
    if 16 * ns > 6000 {
        return Err("the self-consistent ground state diagonalizes a dense matrix: 16 x (number of sites) must not exceed 6000".into());
    }
    let h0 = build_h(lat, &gauge.fields_at(None), m, None).dense();
    let norm = lat.l.ell.powi(lat.nact as i32);
    let (evn, _) = diagonalize(&h0);
    let e_norm = -0.5 * evn.iter().filter(|&&e| e > 0.0).sum::<f64>();
    let line = cfg.s("pair_constraint") == "line";
    let i0 = cfg.i("pair_direction") as usize - 1;
    let ph = C64::cis(cfg.f("pair_phase"));
    let to_d = |x: &[f64]| -> Vec<[C64; 3]> {
        (0..ns)
            .map(|s| {
                let mut o = [C64::ZERO; 3];
                if line {
                    o[i0] = ph.scale(x[s]);
                } else {
                    for i in 0..3 {
                        o[i] = c(x[3 * s + i], x[3 * ns + 3 * s + i]);
                    }
                }
                o
            })
            .collect()
    };
    let to_x = |d: &[[C64; 3]]| -> Vec<f64> {
        if line {
            (0..ns).map(|s| (ph.conj() * d[s][i0]).re).collect()
        } else {
            let mut x = vec![0.0; 6 * ns];
            for s in 0..ns {
                for i in 0..3 {
                    x[3 * s + i] = d[s][i].re;
                    x[3 * ns + 3 * s + i] = d[s][i].im;
                }
            }
            x
        }
    };
    // the self-consistency map and the energy (4.5) measured from the normal state
    let full_map = |d: &[[C64; 3]]| -> (Vec<[C64; 3]>, f64, Vec<f64>, CMat) {
        let mut a = h0.clone();
        add_pair_dense(&mut a, m, d);
        let (ev, vec) = diagonalize(&a);
        let (w, mm) = positive_modes(&ev, &vec);
        let pi = anomalous(m, ns, &w, mm);
        let dn: Vec<[C64; 3]> = pi.iter().map(|p| [p[0].scale(-0.5 * g / norm), p[1].scale(-0.5 * g / norm), p[2].scale(-0.5 * g / norm)]).collect();
        let egs = -0.5 * ev.iter().filter(|&&e| e > 0.0).sum::<f64>() + norm / g * d.iter().map(|x| x.iter().map(|z| z.norm_sqr()).sum::<f64>()).sum::<f64>() - e_norm;
        (dn, egs, ev, vec)
    };
    // Anderson acceleration (memory 6, damping beta), as in t2_solve.py
    let beta = cfg.f("mixing");
    let tol = cfg.f("tolerance");
    let maxit = cfg.i("max_iterations") as usize;
    let mut xs: Vec<Vec<f64>> = vec![];
    let mut fs_: Vec<Vec<f64>> = vec![];
    let mut x = to_x(d0);
    let mut its = vec![];
    let mut converged = false;
    let mut result = x.clone();
    let t0 = Instant::now();
    for k in 1..=maxit {
        let dk = to_d(&x);
        let (dn, egs, _, _) = full_map(&dk);
        let gx = to_x(&dn);
        let f: Vec<f64> = gx.iter().zip(&x).map(|(a, b)| a - b).collect();
        let ch = f.iter().map(|v| v.abs()).fold(0.0, f64::max);
        let mean_d = dk.iter().map(absd).sum::<f64>() / ns as f64;
        its.push((k, ch, mean_d, egs / ns as f64));
        if ch < tol {
            converged = true;
            result = gx;
            break;
        }
        xs.push(x.clone());
        fs_.push(f.clone());
        if xs.len() > 7 {
            xs.remove(0);
            fs_.remove(0);
        }
        if fs_.len() >= 2 {
            let mcols = fs_.len() - 1;
            let df: Vec<Vec<f64>> = (0..mcols).map(|i| fs_[i + 1].iter().zip(&fs_[i]).map(|(a, b)| a - b).collect()).collect();
            let dx: Vec<Vec<f64>> = (0..mcols).map(|i| xs[i + 1].iter().zip(&xs[i]).map(|(a, b)| a - b).collect()).collect();
            let mut a = vec![vec![0.0; mcols]; mcols];
            let mut rhs = vec![0.0; mcols];
            for p in 0..mcols {
                rhs[p] = df[p].iter().zip(&f).map(|(u, v)| u * v).sum();
                for q in 0..mcols {
                    a[p][q] = df[p].iter().zip(&df[q]).map(|(u, v)| u * v).sum();
                }
            }
            let tr: f64 = (0..mcols).map(|p| a[p][p]).sum();
            let lam = 1e-12 * tr.max(1e-300);
            for p in 0..mcols {
                a[p][p] += lam;
            }
            let gam = solve_small(a, rhs);
            for i in 0..x.len() {
                let mut corr = 0.0;
                for p in 0..mcols {
                    corr += (dx[p][i] + beta * df[p][i]) * gam[p];
                }
                x[i] += beta * f[i] - corr;
            }
        } else {
            for i in 0..x.len() {
                x[i] += beta * f[i];
            }
        }
        result = x.clone();
    }
    if !converged {
        // the map applied to the last iterate (as t2_solve.py)
        let (dn, _, _, _) = full_map(&to_d(&x));
        result = to_x(&dn);
    }
    let el = t0.elapsed().as_secs_f64();
    let d = to_d(&result);
    let (dn, egs, ev, vec) = full_map(&d);
    let last = its.last().unwrap().1;
    rep.check(
        converged,
        format!(
            "the self-consistent (Anderson) iteration converged: {} iterations, last change of d^iota {:.2e} (tolerance {}, damping {}{})",
            its.len(),
            last,
            g6(tol),
            g6(beta),
            if line { ", constrained to the line of pair_direction" } else { "" }
        ),
    );
    let res = (0..ns).map(|s| (0..3).map(|i| (dn[s][i] - d[s][i]).abs()).fold(0.0, f64::max)).fold(0.0, f64::max);
    rep.check(
        res < 10.0 * tol,
        format!("the result satisfies the full self-consistency condition d = -(G/2) Pi/dx^D (all three isospin components, real and imaginary parts): largest residual {:.2e}", res),
    );
    let nzero = ev.iter().filter(|e| e.abs() < 1e-9).count();
    rep.add(format!(
        "ground state at G = {}: E_GS per site (measured from the normal state) = {}; number of eigenvalues with |E_qp| < 1e-9: {}; computing time {:.2} s",
        g6(g), g12(egs / ns as f64), nzero, el
    ));
    Ok(Gs { d, ev, vec, its })
}

fn d_rows(lat: &Lat, d: &[[C64; 3]], tag: Option<&str>, t: Option<f64>) -> Vec<Vec<Cell>> {
    (0..lat.ns)
        .map(|s| {
            let mut r = vec![];
            if let Some(tg) = tag {
                r.push(Cell::S(tg.to_string()));
            }
            if let Some(tt) = t {
                r.push(Cell::F(tt));
            }
            r.extend([
                Cell::F(lat.x[s][0]),
                Cell::F(lat.x[s][1]),
                Cell::F(lat.x[s][2]),
                Cell::F(d[s][0].re),
                Cell::F(d[s][0].im),
                Cell::F(d[s][1].re),
                Cell::F(d[s][1].im),
                Cell::F(d[s][2].re),
                Cell::F(d[s][2].im),
                Cell::F(absd(&d[s])),
            ]);
            r
        })
        .collect()
}

fn its_rows(its: &[(usize, f64, f64, f64)]) -> Vec<Vec<Cell>> {
    its.iter().map(|&(k, ch, md, e)| vec![Cell::I(k), Cell::F(ch), Cell::F(md), Cell::F(e)]).collect()
}

fn task_ground_state(cfg: &Cfg, lat: &Lat, gauge: &Gauge, d0: &[[C64; 3]], m: &M, rep: &mut Report, base: &str) -> Result<(), String> {
    let g = cfg.f("coupling");
    let herm = build_h(lat, &gauge.fields_at(None), m, Some(d0)).hermiticity();
    rep.check(herm < 1e-13, format!("the lattice operator H_lat is Hermitian (largest deviation {:.1e})", herm));
    rep.add(format!(
        "self-consistency: G = {}, initial pair field '{}' (direction {}, amplitude {})",
        g6(g),
        cfg.s("pair"),
        cfg.i("pair_direction"),
        g6(cfg.f("pair_amplitude"))
    ));
    let gs = ground_state(cfg, lat, gauge, d0, m, rep, g)?;
    let nn = gs.ev.len();
    let phs = (0..nn).map(|k| (gs.ev[k] + gs.ev[nn - 1 - k]).abs()).fold(0.0, f64::max);
    rep.check(phs < 1e-10, format!("the spectrum is symmetric about zero (particle-hole symmetry; largest deviation {:.1e})", phs));
    let ad: Vec<f64> = gs.d.iter().map(absd).collect();
    if cfg.s("pair") == "uniform" && lat.nact == 1 && lat.active[0] && lat.l.periodic[0] && gauge.is_free() {
        let root = lattice_gap_root(lat, g);
        let dev = ad.iter().map(|a| (a - root).abs()).fold(0.0, f64::max);
        rep.check(
            dev < 1e-7,
            format!("the converged pair field is uniform and equals the root {} of the lattice gap equation (3.21), 1/G = (1/(n_x dx)) sum_q (1/E^+ + 1/E^-) (largest deviation {:.1e})", g12(root), dev),
        );
    }
    let emin = gs.ev.iter().filter(|&&e| e > 0.0).fold(f64::INFINITY, |a, &b| a.min(b));
    rep.add(format!(
        "results: |d| from {} to {}, mean {}; lowest positive E_qp = {}",
        fmt_g(ad.iter().cloned().fold(f64::INFINITY, f64::min), 10),
        fmt_g(ad.iter().cloned().fold(0.0, f64::max), 10),
        fmt_g(ad.iter().sum::<f64>() / ad.len() as f64, 10),
        fmt_g(emin, 10)
    ));
    let mut meta = meta_lines(cfg, lat, gauge);
    meta.push(("G".into(), g6(g)));
    let itr = its_rows(&gs.its);
    write_csv(
        &format!("{}_iterations.csv", base),
        &format!("the self-consistent iteration: the largest change of the pair field, the mean of |d| and the energy per site E_GS measured from the normal state (task ground_state, {})", PROGRAM),
        &meta,
        &["iteration", "change", "abs_d_mean", "E_GS"],
        &itr,
    );
    write_csv(
        &format!("{}_spectrum.csv", base),
        &format!("the eigenvalues of the lattice operator in the self-consistent ground state ({})", PROGRAM),
        &meta,
        &["index", "E_qp"],
        &(0..nn).map(|k| vec![Cell::I(k), Cell::F(gs.ev[k])]).collect::<Vec<_>>(),
    );
    let mut rows = d_rows(lat, d0, Some("initial"), None);
    rows.extend(d_rows(lat, &gs.d, Some("final"), None));
    write_csv(
        &format!("{}_fields.csv", base),
        &format!("the pair field on the sites: the initial guess and the self-consistent result (task ground_state, {})", PROGRAM),
        &meta,
        &["stage", "x_LL^1", "x_LL^2", "x_LL^3", "d1_re", "d1_im", "d2_re", "d2_im", "d3_re", "d3_im", "abs_d"],
        &rows,
    );
    write_csv(
        &format!("{}_series.csv", base),
        &format!("the self-consistent iteration (task ground_state, {})", PROGRAM),
        &meta,
        &["iteration", "change", "abs_d_mean", "E_GS"],
        &its_rows(&gs.its),
    );
    let _ = gs.vec;
    Ok(())
}

// ============================================================================ task quench
fn task_quench(cfg: &Cfg, lat: &Lat, gauge: &Gauge, d0: &[[C64; 3]], m: &M, rep: &mut Report, base: &str) -> Result<(), String> {
    let (g, g2) = (cfg.f("coupling"), cfg.f("quench_coupling"));
    let ns = lat.ns;
    let norm = lat.l.ell.powi(lat.nact as i32);
    let herm = build_h(lat, &gauge.fields_at(None), m, Some(d0)).hermiticity();
    rep.check(herm < 1e-13, format!("the lattice operator H_lat is Hermitian (largest deviation {:.1e})", herm));
    rep.add(format!("quench: the ground state at G = {} is evolved with G = {} after x^0 = 0", g6(g), g6(g2)));
    let gs = ground_state(cfg, lat, gauge, d0, m, rep, g)?;
    let (w, mm) = positive_modes(&gs.ev, &gs.vec);
    let n = 16 * ns;
    // the output grid: the time series at x^0 = k output_step (k = 0 .. outputs), the fields every fields_every outputs;
    // CVODE chooses its own internal steps between the output times
    let ostep = cfg.f("output_step");
    let nout = cfg.i("outputs") as usize;
    let mut every = cfg.i("fields_every") as usize;
    if every == 0 {
        every = (nout / 50).max(1);
    }
    let set = CvodeSettings::adams(cfg.f("rtol"), cfg.f("atol"));
    rep.add(format!(
        "time integration of the {} modes (the self-consistent mean-field equations, {} complex unknowns): {}; output every {} up to x^0 = {} ({} outputs), fields every {} outputs",
        mm, n * mm, set.describe(), g6(ostep), g6(ostep * nout as f64), nout, every
    ));
    let h0_at = |t: f64| -> Csr { build_h(lat, &gauge.fields_at(if gauge.is_static { None } else { Some(t) }), m, None) };
    let h_static = h0_at(0.0);
    let pair_of = |w: &[C64]| -> Vec<[C64; 3]> {
        anomalous(m, ns, w, mm).iter().map(|p| [p[0].scale(-0.5 * g2 / norm), p[1].scale(-0.5 * g2 / norm), p[2].scale(-0.5 * g2 / norm)]).collect()
    };
    let mut hw = vec![C64::ZERO; n * mm];
    let diagnostics = |w: &[C64], h0: &Csr, hw: &mut Vec<C64>| -> (Vec<[C64; 3]>, f64, f64, f64) {
        let dd = pair_of(w);
        let mut nf = 0.0;
        for s in 0..ns {
            for a in 8..16 {
                nf += norm2(&w[(16 * s + a) * mm..(16 * s + a + 1) * mm]);
            }
        }
        h0.matmat(w, hw, mm);
        let emf = vdot_re(w, hw) + 2.0 * norm / g2 * dd.iter().map(|x| x.iter().map(|z| z.norm_sqr()).sum::<f64>()).sum::<f64>();
        // the largest |W_modes^dag W_modes - 1| (upper triangle and diagonal)
        let mut orth = 0.0f64;
        for p in 0..mm {
            for q in p..mm {
                let mut acc = C64::ZERO;
                for r in 0..n {
                    let a = w[r * mm + p];
                    let b = w[r * mm + q];
                    acc.re += a.re * b.re + a.im * b.im;
                    acc.im += a.re * b.im - a.im * b.re;
                }
                if p == q {
                    acc.re -= 1.0;
                }
                orth = orth.max(acc.abs());
            }
        }
        (dd, nf, emf, orth)
    };
    let (_, nf0, emf0, orth0) = diagnostics(&w, &h_static, &mut hw);
    let mut series: Vec<Vec<Cell>> = vec![];
    let mut fields: Vec<Vec<Cell>> = vec![];
    let (mut mnf, mut memf, mut morth) = (0.0f64, 0.0f64, orth0);
    let mut record = |t: f64, w: &[C64], h0: &Csr, with_fields: bool, series: &mut Vec<Vec<Cell>>, fields: &mut Vec<Vec<Cell>>, hw: &mut Vec<C64>| {
        let (dd, nf, emf, orth) = diagnostics(w, h0, hw);
        let ad: Vec<f64> = dd.iter().map(absd).collect();
        series.push(vec![Cell::F(t), Cell::F(ad.iter().sum::<f64>() / ns as f64), Cell::F(ad[0]), Cell::F(nf), Cell::F(nf - nf0), Cell::F(emf), Cell::F(orth)]);
        if with_fields {
            fields.extend(d_rows(lat, &dd, None, Some(t)));
        }
        mnf = mnf.max((nf - nf0).abs());
        memf = memf.max((emf - emf0).abs());
        morth = morth.max(orth);
    };
    let t0 = Instant::now();
    // the right side of CVODE for all modes at once, dW/dx^0 = -i H_lat(d(W)) W, with the pair field d(W) computed from
    // the modes at every evaluation (the self-consistency of the mean field); the closure owns copies of what it needs
    let (lat_c, gauge_c, m_c, hs_c) = (lat.clone(), gauge.clone(), m.clone(), h_static.clone());
    let f: ComplexRhs = Box::new(move |t, wv, out| {
        let h_t;
        let h0: &Csr = if gauge_c.is_static {
            &hs_c
        } else {
            h_t = build_h(&lat_c, &gauge_c.fields_at(Some(t)), &m_c, None);
            &h_t
        };
        h0.matmat(wv, out, mm);
        let dd: Vec<[C64; 3]> = anomalous(&m_c, ns, wv, mm)
            .iter()
            .map(|p| [p[0].scale(-0.5 * g2 / norm), p[1].scale(-0.5 * g2 / norm), p[2].scale(-0.5 * g2 / norm)])
            .collect();
        apply_pair(&m_c, &dd, wv, out, mm);
        for v in out.iter_mut() {
            *v = c(v.im, -v.re);
        }
    });
    let touts = cvode::uniform_times(0.0, ostep, nout);
    let st = cvode::integrate(&w, 0.0, &touts, &set, f, |i, t, wv| {
        let hnow_owned;
        let hnow: &Csr = if gauge.is_static {
            &h_static
        } else {
            hnow_owned = h0_at(t);
            &hnow_owned
        };
        record(t, wv, hnow, i % every == 0 || i == nout, &mut series, &mut fields, &mut hw);
    })?;
    let el = t0.elapsed().as_secs_f64();
    rep.add(format!("CVODE: {}", st.describe()));
    // the exact equations conserve the fermion number, the orthonormality of the modes and (for static gauge fields) the
    // functional E_MF; CVODE keeps them up to its accumulated local error, budget = steps sqrt(2) (rtol ||W|| + atol
    // sqrt(n mm)) with ||W|| = sqrt(mm) for mm orthonormal modes (t2core::cvode::CvodeSettings::error_budget); as in the
    // task evolve, the checks allow 10 times the budget for the change dW of the modes.  A change dW changes N_F (a sum
    // of squared moduli) by at most 2 ||W|| ||dW|| and an entry of W^dag W by at most 2 ||dW||.
    let budget = set.error_budget(st.nsteps, n * mm, (mm as f64).sqrt());
    let nf_tol = 20.0 * (mm as f64).sqrt() * budget;
    rep.check(mnf < nf_tol, format!("the fermion number N_F = {} is conserved by the self-consistent evolution: largest change {:.2e} (bound: 2 ||W_modes|| times 10 times the accumulated local error permitted by CVODE, {:.1e})", g12(nf0), mnf, nf_tol));
    rep.check(morth < 20.0 * budget, format!("the modes stay orthonormal (the Pauli principle): largest |W_modes^dag W_modes - 1| = {:.2e} (bound: 2 times 10 times the accumulated local error permitted by CVODE, {:.1e})", morth, 20.0 * budget));
    if gauge.is_static {
        rep.add(format!(
            "the conserved mean-field functional E_MF = {} changes by at most {:.3e} (CVODE keeps it up to its accumulated local errors)",
            g12(emf0),
            memf
        ));
    }
    let tt: Vec<f64> = series.iter().map(|r| if let Cell::F(v) = r[0] { v } else { 0.0 }).collect();
    let ad: Vec<f64> = series.iter().map(|r| if let Cell::F(v) = r[1] { v } else { 0.0 }).collect();
    let after = &ad[1..];
    let mean_after = after.iter().sum::<f64>() / after.len() as f64;
    rep.add(format!(
        "results: |d| at x^0 = 0: {}; mean of |d| over the run after the quench {}; smallest and largest |d| after the quench {} and {}; computing time {:.2} s",
        fmt_g(ad[0], 10),
        fmt_g(mean_after, 10),
        fmt_g(after.iter().cloned().fold(f64::INFINITY, f64::min), 10),
        fmt_g(after.iter().cloned().fold(0.0, f64::max), 10),
        el
    ));
    let (wfit, afit) = fit_frequency(&tt[1..], after, 0.2, 6.0, 5801);
    rep.add(format!(
        "the amplitude of the pair field oscillates with the angular frequency {:.6} (least-squares fit of a constant plus a cosine and a sine of that angular frequency of x^0 to |d| for x^0 > 0, the angular frequency scanned from 0.2 to 6 in steps of 0.001 and refined by a parabola; fitted amplitude {}); twice the mean of |d| after the quench is {:.6}",
        wfit,
        g6(afit),
        2.0 * mean_after
    ));
    let mut meta = meta_lines(cfg, lat, gauge);
    meta.push(("G".into(), g6(g)));
    meta.push(("quench_coupling".into(), g6(g2)));
    write_csv(
        &format!("{}_iterations.csv", base),
        &format!("the self-consistent iteration of the initial ground state (task quench, {})", PROGRAM),
        &meta,
        &["iteration", "change", "abs_d_mean", "E_GS"],
        &its_rows(&gs.its),
    );
    write_csv(
        &format!("{}_spectrum.csv", base),
        &format!("the eigenvalues of the lattice operator in the initial ground state (task quench, {})", PROGRAM),
        &meta,
        &["index", "E_qp"],
        &(0..gs.ev.len()).map(|k| vec![Cell::I(k), Cell::F(gs.ev[k])]).collect::<Vec<_>>(),
    );
    write_csv(
        &format!("{}_series.csv", base),
        &format!("time series after the quench: the mean of |d| over the sites, |d| at the first site, the fermion number and its change since x^0 = 0, the conserved functional E_MF and the largest deviation from orthonormality (task quench, {})", PROGRAM),
        &meta,
        &["x_LL^0", "abs_d_mean", "abs_d_site0", "N_F", "N_F_change", "E_MF", "ortho_error"],
        &series,
    );
    write_csv(
        &format!("{}_fields.csv", base),
        &format!("the pair field on the sites at the output times (task quench, {})", PROGRAM),
        &meta,
        &["x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3", "d1_re", "d1_im", "d2_re", "d2_im", "d3_re", "d3_im", "abs_d"],
        &fields,
    );
    Ok(())
}

// ============================================================================ the self-test
fn selftest(report: Option<&str>, exdir: Option<&str>) -> bool {
    let mut lines: Vec<String> = vec![];
    let mut say = |s: String| {
        let s = py_exponents(&s);
        println!("{}", s);
        lines.push(s);
    };
    say(format!("self-test of {}", PROGRAM));
    let mut ok_all = true;
    let mut rng = Rng(2026_11_17_05);
    let m = M::new();
    // 1. the sparse operator against t2core's Lattice::apply (a site-by-site implementation), random fields, 3D
    let l = Lattice { n: [3, 4, 2], ell: 0.7, periodic: [true, false, true], wilson: 0.8, mass: 0.3, mu: 0.25 };
    let ns = 24;
    let lat = Lat {
        x: (0..ns).map(|s| { let cc = l.coords(s); [cc[0] as f64 * 0.7, cc[1] as f64 * 0.7, cc[2] as f64 * 0.7] }).collect(),
        l,
        ns,
        active: [true, true, true],
        nact: 3,
    };
    let mut fl = Fields::trivial(ns);
    for s in 0..ns {
        for j in 0..3 {
            fl.links[s][j] = link(0.7, 0.45, &[rng.unit(), rng.unit(), rng.unit()], rng.unit(), 0.7);
        }
        fl.v0[s] = potential(0.7, 0.45, &[rng.unit(), rng.unit(), rng.unit()], rng.unit());
        fl.d[s] = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
    }
    let h = build_h(&lat, &fl, &m, Some(&fl.d));
    let mats = Mats::new();
    let mut dev = 0.0f64;
    for _ in 0..5 {
        let wv: Vec<C64> = (0..16 * ns).map(|_| c(rng.unit(), rng.unit())).collect();
        let mut a = vec![C64::ZERO; 16 * ns];
        let mut b = vec![C64::ZERO; 16 * ns];
        h.matvec(&wv, &mut a);
        lat.l.apply(&fl, &mats, &wv, &mut b);
        dev = dev.max(a.iter().zip(&b).map(|(x, y)| (*x - *y).abs()).fold(0.0, f64::max));
    }
    let ok = dev < 1e-13;
    ok_all &= ok;
    say(format!("[{}] the sparse lattice operator equals the site-by-site operator Lattice::apply of t2core (3 x 4 x 2 sites, random links, potentials and pair field, periodic and wall directions): largest deviation {:.1e}", if ok { "PASS" } else { "FAIL" }, dev));
    let herm = h.hermiticity();
    let ok = herm < 1e-13;
    ok_all &= ok;
    say(format!("[{}] it is Hermitian: largest deviation {:.1e}", if ok { "PASS" } else { "FAIL" }, herm));
    // 2. the links against the matrix exponential
    let mut dl = 0.0f64;
    for _ in 0..20 {
        let a = [2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit()];
        let b = 2.0 * rng.unit();
        let mut gen = CMat::zeros(2);
        for i in 0..3 {
            gen = gen.add(&t2core::model::sigma(i + 1).scale_r(0.35 * a[i]));
        }
        gen = gen.add(&CMat::identity(2).scale_r(0.45 * b));
        let e = gen.scale(c(0.0, -0.6)).expm();
        let u = link(0.7, 0.45, &a, b, 0.6);
        for i in 0..2 {
            for k in 0..2 {
                dl = dl.max((u[2 * i + k] - e.get(i, k)).abs());
            }
        }
    }
    let ok = dl < 1e-13;
    ok_all &= ok;
    say(format!("[{}] the closed form of the links equals the matrix exponential exp(-i dx (g_v a.tau/2 + g_F b)) (20 random potentials): largest deviation {:.1e}", if ok { "PASS" } else { "FAIL" }, dl));
    // 3. plane waves in constant potentials along tau_3 (check AS-1)
    let l2 = Lattice { n: [16, 1, 1], ell: 0.5, periodic: [true, true, true], wilson: 1.0, mass: 0.4, mu: 0.2 };
    let lat2 = Lat { x: (0..16).map(|s| [s as f64 * 0.5, 0.0, 0.0]).collect(), l: l2, ns: 16, active: [true, false, false], nact: 1 };
    let mut fl2 = Fields::trivial(16);
    for s in 0..16 {
        fl2.links[s][0] = link(0.7, 0.45, &[0.0, 0.0, 0.8], 0.6, 0.5);
        fl2.v0[s] = potential(0.7, 0.45, &[0.0, 0.0, 0.5], 0.3);
    }
    let (ev, _) = diagonalize(&build_h(&lat2, &fl2, &m, None).dense());
    let mut expect = vec![];
    for k in 0..16 {
        let q = 2.0 * std::f64::consts::PI * (if k <= 8 { k as f64 } else { k as f64 - 16.0 }) / 8.0;
        for qq in [0.5, -0.5] {
            let a1 = 0.45 * 0.6 + qq * 0.7 * 0.8;
            let a0 = 0.45 * 0.3 + qq * 0.7 * 0.5;
            let ep = eps_lat_q(&lat2, &[q - a1, 0.0, 0.0]);
            let eh = eps_lat_q(&lat2, &[q + a1, 0.0, 0.0]);
            for _ in 0..2 {
                expect.extend([ep - a0 - 0.2, -ep - a0 - 0.2, eh + a0 + 0.2, -eh + a0 + 0.2]);
            }
        }
    }
    expect.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let dev2 = ev.iter().zip(&expect).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max);
    let ok = dev2 < 1e-12 && ev.len() == expect.len();
    ok_all &= ok;
    say(format!("[{}] plane waves in constant potentials: the 256 eigenvalues of a ring of 16 sites are the energies of check AS-1 (largest deviation {:.1e})", if ok { "PASS" } else { "FAIL" }, dev2));
    // 4. the time integration with CVODE: the eigenvector v of the operator of check 1 that belongs to its largest
    //    eigenvalue E evolves as exp(-i E x^0) v; the largest deviation up to x^0 = 20 is compared with 10 times the
    //    accumulated local error that the error test of CVODE permits
    let (evs, vecs) = diagonalize(&h.dense());
    let kmax = evs.len() - 1;
    let e_max = evs[kmax];
    let v: Vec<C64> = (0..16 * ns).map(|r| vecs.get(r, kmax)).collect();
    let set = CvodeSettings::adams(1e-12, 1e-14);
    let hc = h.clone();
    let f: ComplexRhs = Box::new(move |_t, wv, dw| {
        hc.matvec(wv, dw);
        for z in dw.iter_mut() {
            *z = c(z.im, -z.re);
        }
    });
    let mut worst = 0.0f64;
    let res = cvode::integrate(&v, 0.0, &cvode::uniform_times(0.0, 0.5, 40), &set, f, |_i, t, wv| {
        let ph = C64::cis(-e_max * t);
        let d2: f64 = wv.iter().zip(&v).map(|(a, b)| (*a - ph * *b).norm_sqr()).sum();
        worst = worst.max(d2.sqrt());
    });
    match res {
        Ok(st) => {
            let bound = 10.0 * set.error_budget(st.nsteps, 16 * ns, 1.0);
            let ok = worst < bound;
            ok_all &= ok;
            say(format!("[{}] the time integration with CVODE: the eigenvector of the largest eigenvalue E = {:.6} of the operator of check 1 evolves as exp(-i E x^0) up to x^0 = 20: largest deviation {:.1e} (bound: 10 times the accumulated local error permitted by CVODE, {:.1e}; {} steps)", if ok { "PASS" } else { "FAIL" }, e_max, worst, bound, st.nsteps));
        }
        Err(err) => {
            ok_all = false;
            say(format!("[FAIL] the time integration with CVODE failed: {}", err));
        }
    }
    // 5. the examples are the same as the files written by the Python solver
    if let Some(dir) = exdir {
        let mut same = 0;
        for e in EXAMPLES {
            let p = Path::new(dir).join(format!("{}.t2in", e.0));
            if fs::read_to_string(&p).map(|t| Some(t) == example_text(e.0)).unwrap_or(false) {
                same += 1;
            }
        }
        let ok = same == EXAMPLES.len();
        ok_all &= ok;
        say(format!("[{}] the {} examples of this program are identical to the input files of the folder {} ({} of {})", if ok { "PASS" } else { "FAIL" }, EXAMPLES.len(), dir, same, EXAMPLES.len()));
    }
    say(format!("self-test: {}", if ok_all { "ALL PASS" } else { "FAILED" }));
    if let Some(r) = report {
        let checks: Vec<&String> = lines.iter().filter(|s| s.starts_with('[')).collect();
        let npass = checks.iter().filter(|s| s.starts_with("[PASS]")).count();
        let mut s = format!("Revision_SuperLoop, folder test2, program `{}` (Test 2, item 5): the self-test of the lattice operator, of the time integration and of the examples of the Rust solver\n", PROGRAM);
        s += &format!("{} of {} checks passed\n", npass, checks.len());
        for l in &lines {
            s += l;
            s.push('\n');
        }
        fs::write(r, s).expect("report");
    }
    ok_all
}

// ============================================================================ main
fn solve(path: &str, out: Option<&str>) -> Result<bool, String> {
    let cfg = read_input(path)?;
    let lat = Lat::new(&cfg);
    let gauge = Gauge::new(&cfg, &lat)?;
    let d = pair_profile(&cfg, &lat)?;
    let m = M::new();
    let base = match out {
        Some(o) => o.to_string(),
        None => {
            let o = cfg.s("output");
            if Path::new(&o).is_absolute() { o } else { cfg.dir.join(&o).to_string_lossy().to_string() }
        }
    };
    if let Some(parent) = Path::new(&base).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("cannot create the folder of {}: {}", base, e))?;
        }
    }
    for suffix in ["_summary.txt", "_series.csv", "_fields.csv", "_spectrum.csv", "_iterations.csv"] {
        let _ = fs::remove_file(format!("{}{}", base, suffix));
    }
    let mut rep = Report::new(&cfg, &lat);
    rep.add(format!(
        "pair field: {}, direction {}, amplitude {}, phase {}, width {}, positions {:?}",
        cfg.s("pair"),
        cfg.i("pair_direction"),
        g6(cfg.f("pair_amplitude")),
        g6(cfg.f("pair_phase")),
        g6(cfg.f("pair_width")),
        cfg.v("pair_position")
    ));
    rep.add(format!("gauge fields: {}", gauge.desc));
    let task = cfg.s("task");
    if !gauge.is_static && (task == "spectrum" || task == "ground_state") {
        rep.add(format!("note: the task {} ignores the pulse", task));
    }
    println!("{}: task {}, {} sites, dimension {}", Path::new(path).file_name().unwrap().to_string_lossy(), task, lat.ns, 16 * lat.ns);
    match task.as_str() {
        "evolve" => task_evolve(&cfg, &lat, &gauge, &d, &m, &mut rep, &base)?,
        "spectrum" => task_spectrum(&cfg, &lat, &gauge, &d, &m, &mut rep, &base)?,
        "ground_state" => task_ground_state(&cfg, &lat, &gauge, &d, &m, &mut rep, &base)?,
        _ => task_quench(&cfg, &lat, &gauge, &d, &m, &mut rep, &base)?,
    }
    let files: Vec<String> = ["_summary.txt", "_series.csv", "_fields.csv", "_spectrum.csv", "_iterations.csv"]
        .iter()
        .map(|s| format!("{}{}", base, s))
        .filter(|f| f.ends_with("_summary.txt") || Path::new(f).exists())
        .collect();
    rep.add(format!("output files: {}", files.iter().map(|f| Path::new(f).file_name().unwrap().to_string_lossy().to_string()).collect::<Vec<_>>().join(", ")));
    let ok = rep.write(&format!("{}_summary.txt", base));
    println!("wrote {}", files.join(", "));
    println!("{}", if ok { "all checks passed".to_string() } else { format!("SOME CHECKS FAILED (see {}_summary.txt)", base) });
    Ok(ok)
}

fn list_examples() {
    println!("Examples (write one with: t2_solve --example NAME [FILE]):");
    for e in EXAMPLES {
        println!("  {:<18} {}", e.0, e.1);
    }
}

fn write_example(name: &str, path: Option<&str>) -> Result<String, String> {
    let text = example_text(name).ok_or_else(|| format!("there is no example '{}'", name))?;
    let p = path.map(|s| s.to_string()).unwrap_or_else(|| format!("{}.t2in", name));
    fs::write(&p, text).map_err(|e| format!("cannot write {}: {}", p, e))?;
    println!("wrote the example input file {}", p);
    Ok(p)
}

fn interactive() -> i32 {
    println!("t2_solve: the Rust solver of the gauge-covariant mean-field field equations of the spinor field of Test 2");
    println!("(item [5]; the same input files and output files as the Python solver t2_solve.py).");
    println!();
    println!("How to define the initial and the boundary conditions, in short:");
    println!("  * boundary conditions: 'sites = n_x n_y n_z' and 'boundary = periodic|wall periodic|wall periodic|wall';");
    println!("  * initial condition of a mode: 'initial = packet' with packet_center, packet_width, packet_momentum,");
    println!("    packet_branch, packet_spin, packet_isospin, packet_part (or 'eigenmode', or 'file');");
    println!("  * fields: potential_0 .. potential_3, gauge_file, magnetic, the pulse, and the pair field 'pair = ...'");
    println!("    (the initial guess of the self-consistent tasks).");
    println!("Type '?' for the complete guide, 'examples' for the list of examples, 'example NAME' to write one, 'q' to quit.");
    let stdin = io::stdin();
    loop {
        print!("\nName of the input file: ");
        io::stdout().flush().ok();
        let mut line = String::new();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 {
            return 1;
        }
        let ans = line.trim();
        if ans.is_empty() || ans == "q" || ans == "quit" {
            return 0;
        }
        if ans == "?" {
            println!("{}", guide_text());
            continue;
        }
        if ans == "examples" {
            list_examples();
            continue;
        }
        if let Some(name) = ans.strip_prefix("example ") {
            match write_example(name.trim(), None) {
                Ok(p) => println!("run it by typing its name: {}", p),
                Err(e) => println!("error: {}", e),
            }
            continue;
        }
        match solve(ans, None) {
            Ok(ok) => return if ok { 0 } else { 1 },
            Err(e) => {
                println!("error in the input: {}", e);
                println!("(type '?' for the guide)");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let code = if args.len() == 1 {
        interactive()
    } else {
        let a = args[1].as_str();
        let r: Result<i32, String> = match a {
            "--guide" => {
                println!("{}", guide_text());
                Ok(0)
            }
            "--examples" => {
                list_examples();
                Ok(0)
            }
            "--example" => {
                if args.len() < 3 {
                    Err("--example needs the name of an example (see --examples)".into())
                } else {
                    write_example(&args[2], args.get(3).map(|s| s.as_str())).map(|_| 0)
                }
            }
            "--selftest" => Ok(if selftest(args.get(2).map(|s| s.as_str()), args.get(3).map(|s| s.as_str())) { 0 } else { 1 }),
            _ => {
                let out = if args.len() > 2 {
                    if args[2] != "--out" || args.len() != 4 {
                        Err("usage: t2_solve FILE.t2in [--out BASE]".to_string())
                    } else {
                        Ok(Some(args[3].as_str()))
                    }
                } else {
                    Ok(None)
                };
                out.and_then(|o| solve(a, o)).map(|ok| if ok { 0 } else { 1 })
            }
        };
        match r {
            Ok(c) => c,
            Err(e) => {
                println!("error in the input: {}", e);
                2
            }
        }
    };
    std::process::exit(code);
}
