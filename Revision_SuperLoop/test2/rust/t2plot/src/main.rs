//! t2_plot -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, item [6].
//!
//! A pure Rust program that plots the solutions written by the Rust solver t2_solve of item [5] (and by the Python solver,
//! which writes the same files).  For a solution with the base name BASE it reads BASE_summary.txt, BASE_series.csv,
//! BASE_fields.csv and, if present, BASE_spectrum.csv and BASE_iterations.csv, chooses the panels by the task (the line
//! "# task:" of the data files), and writes one figure as Scalable Vector Graphics, FIG.svg (any web browser shows it; the
//! document of Test 2 converts it to PDF and PNG with the program rsvg-convert).  For the seven examples of the folder
//! test2/examples it also draws the predictions of the checks EX-1 to EX-7, computed here in Rust from the formulas of the
//! document (the mean group velocity (4.7), the Rayleigh quotient (3.29), the gap equation (3.21), the continuum kink and the
//! Wilson shift, and the frequency (4.11) of the amplitude mode).
//!
//! Build (from the folder test2/rust):  cargo build --release
//! Use (from the folder Revision_SuperLoop):
//!   ./test2/rust/target/release/t2_plot                          asks for the base name of a solution
//!   ./test2/rust/target/release/t2_plot BASE                     writes BASE_plot.svg
//!   ./test2/rust/target/release/t2_plot BASE --out FIG           writes FIG.svg
//!   ./test2/rust/target/release/t2_plot BASE --no-predictions    without the predictions of the examples
//!
//! The predictions are drawn only when the base name ends with the name of one of the seven examples (for instance
//! test2/data/t2_rs_andreev); for any other solution the figure shows the data alone.  The program never changes a data file.
//!
//! The symbols in the comments are those of the document of Test 2: x^0_LL time, x^1_LL, x^2_LL, x^3_LL position, l^x_LL (here
//! `ell`) the lattice spacing, n^x_LL (here `n`) the number of sites in direction 1, m_LL the mass, mu_LL the chemical
//! potential, r^W_LL the Wilson parameter, G_LL the coupling, d^i_LL (i = 1, 2, 3) the isotriplet pair field, Delta_LL its
//! uniform magnitude, E^qp_LL the quasiparticle energies, eps^lat_LL the lattice dispersion (2.37).
mod svg;

use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, Write};
use svg::*;
use t2core::lattice::eps_lat;

const EXAMPLES: [&str; 7] = ["packet_free", "cyclotron", "andreev", "kink_spectrum", "ground_state", "kink_ground_state", "quench"];

/// one data file of a solution: the header lines "# key: value" (`meta`) and the columns of the comma-separated table
/// below them, kept as text and indexed by the names in the first table line (`cols`)
struct Table {
    meta: HashMap<String, String>,
    cols: HashMap<String, Vec<String>>,
}

impl Table {
    /// reads a file; None if the file does not exist (the files _spectrum.csv and _iterations.csv are optional)
    fn read(path: &str) -> Option<Table> {
        let text = fs::read_to_string(path).ok()?;
        let mut meta = HashMap::new();
        let mut rows: Vec<Vec<String>> = vec![];
        for line in text.lines() {
            if let Some(rest) = line.strip_prefix('#') {
                if let Some((k, v)) = rest.split_once(':') {
                    meta.insert(k.trim().to_string(), v.trim().to_string());
                }
            } else if !line.is_empty() {
                rows.push(line.split(',').map(|x| x.to_string()).collect());
            }
        }
        // the first table line holds the column names; the following lines hold the values
        let header = rows.first()?.clone();
        let mut cols = HashMap::new();
        for (i, h) in header.iter().enumerate() {
            cols.insert(h.clone(), rows[1..].iter().map(|r| r.get(i).cloned().unwrap_or_default()).collect());
        }
        Some(Table { meta, cols })
    }
    /// a column as numbers (NaN for an entry that is not a number; an empty vector for a missing column)
    fn f(&self, name: &str) -> Vec<f64> {
        self.cols.get(name).map(|v| v.iter().map(|x| x.parse::<f64>().unwrap_or(f64::NAN)).collect()).unwrap_or_default()
    }
    /// a column as text (for the column `stage`, whose entries are "initial" and "final")
    fn s(&self, name: &str) -> Vec<String> {
        self.cols.get(name).cloned().unwrap_or_default()
    }
    /// the first number of the header line "# key: value" (NaN if the key is missing)
    fn m(&self, key: &str) -> f64 {
        self.meta.get(key).and_then(|v| v.split_whitespace().next()?.parse().ok()).unwrap_or(f64::NAN)
    }
}

/// all files of a solution with the base name BASE: BASE_series.csv (one line per output step; required),
/// BASE_fields.csv (fields on the sites at the output steps), BASE_spectrum.csv (eigenvalues), BASE_iterations.csv (the
/// iteration history of the self-consistent tasks) and BASE_summary.txt (the readable report of the solver)
struct Data {
    series: Table,
    fields: Option<Table>,
    spectrum: Option<Table>,
    iterations: Option<Table>,
    summary: Vec<String>,
}

impl Data {
    /// reads all files of the solution BASE; an error (with an explanation) if BASE_series.csv does not exist
    fn read(base: &str) -> Result<Data, String> {
        let series = Table::read(&format!("{}_series.csv", base)).ok_or_else(|| format!("there is no file {}_series.csv: is '{}' the base name of a solution?", base, base))?;
        Ok(Data {
            series,
            fields: Table::read(&format!("{}_fields.csv", base)),
            spectrum: Table::read(&format!("{}_spectrum.csv", base)),
            iterations: Table::read(&format!("{}_iterations.csv", base)),
            summary: fs::read_to_string(format!("{}_summary.txt", base)).map(|t| t.lines().map(|l| l.to_string()).collect()).unwrap_or_default(),
        })
    }
    /// the title of the figure: the first line of the summary after "): " (the title of the run)
    fn title(&self) -> String {
        self.summary.first().map(|l| l.split_once("): ").map(|x| x.1.to_string()).unwrap_or_else(|| l.clone())).unwrap_or_default()
    }
    /// the rest of the first summary line that begins with `prefix` (for instance "pair field:")
    fn summary_value(&self, prefix: &str) -> String {
        self.summary.iter().find(|l| l.starts_with(prefix)).map(|l| l[prefix.len()..].trim().to_string()).unwrap_or_default()
    }
    /// the numbers of sites in the three directions (the header line "# sites: n1 n2 n3")
    fn sites(&self) -> Vec<usize> {
        self.series.meta.get("sites").map(|s| s.split_whitespace().filter_map(|x| x.parse().ok()).collect()).unwrap_or_default()
    }
}

/// the number after "key " in a line of the summary ("amplitude 1, width 1, positions [6.0, 18.0]")
fn number_after(line: &str, key: &str) -> Option<f64> {
    let i = line.find(key)? + key.len();
    line[i..].trim_start().split(|c: char| c == ',' || c.is_whitespace()).next()?.parse().ok()
}

/// the numbers of the list that follows `key` up to the next "]" ("positions [6.0, 18.0]" gives 6 and 18)
fn list_after(line: &str, key: &str) -> Vec<f64> {
    match line.find(key) {
        Some(i) => {
            let rest = &line[i + key.len()..];
            let inner = rest.split(']').next().unwrap_or("");
            inner.split(',').filter_map(|x| x.trim().parse().ok()).collect()
        }
        None => vec![],
    }
}

/// the distinct values of v in ascending order (values closer than 1e-12 count as equal): the grid of the coordinates or times
fn unique(v: &[f64]) -> Vec<f64> {
    let mut u = v.to_vec();
    u.sort_by(|a, b| a.partial_cmp(b).unwrap());
    u.dedup_by(|a, b| (*a - *b).abs() < 1e-12);
    u
}

/// the position of the value v in the sorted list u (from `unique`)
fn idx(u: &[f64], v: f64) -> usize {
    u.iter().position(|&x| (x - v).abs() < 1e-9).unwrap_or(0)
}

/// four times spread evenly over the output times
fn pick4(times: &[f64]) -> Vec<f64> {
    (0..4).map(|k| times[((k as f64) * (times.len() - 1) as f64 / 3.0).round() as usize]).collect()
}

// ============================================================================ the predictions (the formulas of the document)
/// a periodic chain of n sites in direction 1 (the other directions have one site each): the data needed for the
/// predictions of the examples
struct Chain {
    n: usize, // n^x_LL, the number of sites
    ell: f64, // l^x_LL, the lattice spacing (header key "dx")
    m: f64,   // m_LL, the mass
    r: f64,   // r^W_LL, the Wilson parameter
}

impl Chain {
    /// the chain of a solution, from the header lines of its series file
    fn of(t: &Table, n: usize) -> Chain {
        Chain { n, ell: t.m("dx"), m: t.m("m"), r: t.m("r_W") }
    }
    /// the allowed momenta of the periodic chain: q^1_LL = 2 pi kk / (n l^x_LL) with kk = k for k <= n/2 and kk = k - n
    /// otherwise, so that the n momenta lie in the interval (-pi/l^x_LL, pi/l^x_LL]
    fn q(&self, k: usize) -> f64 {
        let kk = if k <= self.n / 2 { k as f64 } else { k as f64 - self.n as f64 };
        2.0 * std::f64::consts::PI * kk / (self.n as f64 * self.ell)
    }
    /// (2.37) with q^2_LL = q^3_LL = 0: eps^lat_LL(q)^2 = sin^2(q l)/l^2 + (m + (r/l)(1 - cos(q l)))^2, from the library
    /// crate t2core (the same function the solver uses)
    fn eps(&self, q: f64) -> f64 {
        eps_lat(&[q, 0.0, 0.0], self.ell, self.m, self.r, &[true, false, false])
    }
    /// (4.7): the group velocity v^g_LL = d eps^lat_LL / dq; differentiating eps^2 of (2.37) and dividing by 2 eps gives
    /// (sin(q l) cos(q l)/l + r sin(q l)(m + (r/l)(1 - cos(q l)))) / eps
    fn vg(&self, q: f64) -> f64 {
        let (s, c) = ((q * self.ell).sin(), (q * self.ell).cos());
        (s * c / self.ell + self.r * s * (self.m + self.r / self.ell * (1.0 - c))) / self.eps(q)
    }
    /// (3.21): the root Delta_LL of the gap equation of the periodic chain,
    ///     1/G_LL = (n l)^(-1) sum_q (1/E^+ + 1/E^-),  E^(+-) = ((eps^lat_LL(q) -+ mu_LL)^2 + Delta_LL^2)^(1/2),
    /// found by bisection: the right side decreases monotonically with Delta_LL, so the root lies where it crosses 1/G_LL;
    /// 200 halvings of the interval (1e-12, 50) reach the precision of the floating-point numbers
    fn gap_root(&self, g: f64, mu: f64) -> f64 {
        let rhs = |d: f64| (0..self.n).map(|k| { let e = self.eps(self.q(k)); 1.0 / (e - mu).hypot(d) + 1.0 / (e + mu).hypot(d) }).sum::<f64>() / (self.n as f64 * self.ell);
        let (mut lo, mut hi) = (1e-12, 50.0);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if rhs(mid) > 1.0 / g {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }
    /// (4.11): the frequency omega^H_LL of the amplitude mode for m_LL = mu_LL = 0 and the uniform gap d = Delta_LL,
    ///     (omega^2/4 - Delta^2) sum_{q != 0} 1/(E (E^2 - omega^2/4)) = 1/Delta,  E = (eps^lat_LL(q)^2 + Delta^2)^(1/2);
    /// the left side minus 1/Delta is negative just above omega = 2 Delta and tends to +infinity below omega = 2 E_1 (E_1 the
    /// smallest E with q != 0), so bisection between these two values finds the root
    fn amplitude_mode(&self, d: f64) -> f64 {
        let es: Vec<f64> = (1..self.n).map(|k| self.eps(self.q(k)).hypot(d)).collect();
        let e1 = es.iter().cloned().fold(f64::INFINITY, f64::min);
        let f = |w: f64| (w * w / 4.0 - d * d) * es.iter().map(|e| 1.0 / (e * (e * e - w * w / 4.0))).sum::<f64>() - 1.0 / d;
        let (mut lo, mut hi) = (2.0 * d * (1.0 + 1e-12), 2.0 * e1 * (1.0 - 1e-12));
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if f(mid) < 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }
}

/// the least-squares fit a + b cos(w t) + c sin(w t) with a given w: with the basis v = (1, cos(w t), sin(w t)) the
/// coefficients x solve the normal equations M x = r, M = sum_t v v^T, r = sum_t v a(t) (a 3 x 3 linear system)
fn sinusoid(t: &[f64], a: &[f64], w: f64) -> [f64; 3] {
    let mut m = [[0.0; 3]; 3];
    let mut r = [0.0; 3];
    for (ti, ai) in t.iter().zip(a) {
        let v = [1.0, (w * ti).cos(), (w * ti).sin()];
        for p in 0..3 {
            r[p] += v[p] * ai;
            for q in 0..3 {
                m[p][q] += v[p] * v[q];
            }
        }
    }
    // Gaussian elimination (3 x 3; M is symmetric and positive definite, no pivoting needed), then back substitution
    for k in 0..3 {
        for i in k + 1..3 {
            let f = m[i][k] / m[k][k];
            for j in k..3 {
                m[i][j] -= f * m[k][j];
            }
            r[i] -= f * r[k];
        }
    }
    let mut x = [0.0; 3];
    for k in (0..3).rev() {
        x[k] = (r[k] - (k + 1..3).map(|j| m[k][j] * x[j]).sum::<f64>()) / m[k][k];
    }
    x
}

// ============================================================================ the figures
/// the task evolve on a chain (four panels): (a) the density of the mode (particle plus hole part) in space and time, with
/// the path of its centre, the steps of the pair field and, for the example packet_free, the predicted straight path with
/// the mean group velocity; (b) the particle and hole parts at four times; (c) the probabilities of the hole part and of
/// isospin up; (d) the conservation of norm and energy.  A lattice with more than one site in direction 2 or 3 goes to
/// plot_evolve_2d.
fn plot_evolve(d: &Data, pred: Option<&str>) -> Figure {
    let n = d.sites();
    if n.len() == 3 && (n[1] > 1 || n[2] > 1) {
        return plot_evolve_2d(d);
    }
    let f = d.fields.as_ref().expect("fields");
    let t = d.series.f("x_LL^0");
    let tf = f.f("x_LL^0");
    let x = f.f("x_LL^1");
    let (rp, rh) = (f.f("density_p"), f.f("density_h"));
    let times = unique(&tf);
    let xs = unique(&x);
    // mt, mp, mh [time][site]: the total, particle and hole densities at the output times
    let mut mt = vec![vec![0.0; xs.len()]; times.len()];
    let mut mp = mt.clone();
    let mut mh = mt.clone();
    for i in 0..tf.len() {
        let (a, b) = (idx(&times, tf[i]), idx(&xs, x[i]));
        mt[a][b] = rp[i] + rh[i];
        mp[a][b] = rp[i];
        mh[a][b] = rh[i];
    }
    let mut fig = Figure::new(1200.0, 880.0, &d.title());
    // (a) the density in space and time
    let mut a = Axes::new(90.0, 80.0, 400.0, 300.0);
    a.xlim = (xs[0], xs[xs.len() - 1]);
    a.ylim = (times[0], times[times.len() - 1]);
    let zmax = mt.iter().flatten().cloned().fold(0.0, f64::max);
    a.heatmap(&mt, (xs[0], xs[xs.len() - 1], times[0], times[times.len() - 1]), zmax);
    a.grid = false;
    let l = xs.len() as f64 * d.series.m("dx");
    let periodic = d.series.meta.get("boundary").map(|b| b.starts_with("periodic")).unwrap_or(false);
    let mut xm = d.series.f("x_mean_1");
    // on a ring the centre jumps from one end to the other when the packet goes round: break the line there (NaN = pen up)
    if periodic {
        for i in 1..xm.len() {
            if (xm[i] - xm[i - 1]).abs() > 0.5 * l {
                xm[i] = f64::NAN;
            }
        }
    }
    a.line(&xm, &t, ORANGE, 1.6, "");
    a.add_key(Key::Line(ORANGE.into(), 1.6, String::new()), "centre of the density");
    let pairline = d.summary_value("pair field:");
    let pos = list_after(&pairline, "positions [");
    for &p in &pos {
        a.vline(p, RED, 1.0, "2,3");
    }
    if !pos.is_empty() {
        a.add_key(Key::Line(RED.into(), 1.0, "2,3".into()), "position of the step of the pair field");
    }
    if pred == Some("packet_free") {
        // the mean group velocity of EX-1: the squared modulus of the Fourier transform of the envelope of (4.2),
        // exp(-2 (w^pk_LL)^2 (q - q^pk)^2), weighs the group velocity (4.7) over the allowed momenta of the ring
        let init = d.summary_value("initial mode:");
        let w = number_after(&init, "width").unwrap_or(1.0);
        let q0 = list_after(&init, "momentum [").first().cloned().unwrap_or(0.0);
        let ch = Chain::of(&d.series, n[0]);
        let (mut num, mut den) = (0.0, 0.0);
        for k in 0..ch.n {
            let q = ch.q(k);
            let wt = (-2.0 * w * w * (q - q0).powi(2)).exp();
            num += wt * ch.vg(q);
            den += wt;
        }
        let v = num / den;
        // the predicted path x^1 = (centre at the start) + v x^0, wrapped onto the ring
        let tt: Vec<f64> = (0..300).map(|k| times[0] + (times[times.len() - 1] - times[0]) * k as f64 / 299.0).collect();
        let mut xx: Vec<f64> = tt.iter().map(|&s| (xm[0] + v * s).rem_euclid(l)).collect();
        for i in 1..xx.len() {
            if (xx[i] - xx[i - 1]).abs() > 0.5 * l {
                xx[i] = f64::NAN;
            }
        }
        a.line(&xx, &tt, INK, 1.0, "5,3");
        a.add_key(Key::Line(INK.into(), 1.0, "5,3".into()), &format!("prediction: mean group velocity {:.5}", v));
    }
    a.legend_at = (0.42, 0.03);
    a.title = "(a) density of the mode (particle and hole part)".into();
    a.xlabel = "x^{1}_{LL}".into();
    a.ylabel = "x^{0}_{LL}".into();
    fig.colorbar(500.0, 80.0, 14.0, 300.0, zmax, "density of the mode on a site");
    fig.axes.push(a);
    // (b) the particle and hole parts at four times
    let mut b = Axes::new(700.0, 80.0, 460.0, 300.0);
    let p4 = pick4(&times);
    let ymax = mp.iter().chain(mh.iter()).flatten().cloned().fold(0.0, f64::max);
    b.xlim = (xs[0], xs[xs.len() - 1]);
    b.ylim = (-0.03 * ymax, 1.05 * ymax);
    for (k, &tv) in p4.iter().enumerate() {
        let i = idx(&times, tv);
        b.line(&xs, &mp[i], SEQ[k], 1.6, "");
        b.add_key(Key::Line(SEQ[k].into(), 1.6, String::new()), &format!("particle part, x^{{0}}_{{LL}} = {}", tick_label(tv)));
        if mh[i].iter().cloned().fold(0.0, f64::max) > 1e-6 {
            b.line(&xs, &mh[i], SEQ[k], 1.6, "5,3");
            b.add_key(Key::Line(SEQ[k].into(), 1.6, "5,3".into()), &format!("hole part, x^{{0}}_{{LL}} = {}", tick_label(tv)));
        }
    }
    for &p in &pos {
        b.vline(p, RED, 1.0, "2,3");
    }
    b.legend_at = (0.55, 0.35);
    b.title = "(b) the particle and the hole part at four times".into();
    b.xlabel = "x^{1}_{LL}".into();
    b.ylabel = "density on a site".into();
    fig.axes.push(b);
    // (c) the probabilities of the hole part and of isospin up (every output step)
    let mut c = Axes::new(90.0, 500.0, 460.0, 300.0);
    c.xlim = (t[0], t[t.len() - 1]);
    c.ylim = (-0.03, 1.03);
    c.line(&t, &d.series.f("P_hole"), ORANGE, 1.7, "");
    c.line(&t, &d.series.f("P_up"), BLUE, 1.7, "");
    c.add_key(Key::Line(ORANGE.into(), 1.7, String::new()), "P^{h}_{LL}: hole part");
    c.add_key(Key::Line(BLUE.into(), 1.7, String::new()), "P^{↑}_{LL}: isospin up");
    c.legend_at = (0.03, 0.42);
    c.title = "(c) the probabilities of the hole part and of isospin up".into();
    c.xlabel = "x^{0}_{LL}".into();
    c.ylabel = "probability".into();
    fig.axes.push(c);
    fig.axes.push(conservation(d, 700.0, 500.0, 460.0, 300.0, "(d) conservation of the norm and of the energy (CVODE errors)"));
    fig
}

/// a panel with the changes |norm - 1| and |<H^lat_LL> - <H^lat_LL> at the start| over time, on a logarithmic axis (an exact
/// zero is drawn at 1e-17); CVODE conserves both only up to its accumulated local errors, which these curves show
fn conservation(d: &Data, x0: f64, y0: f64, w: f64, h: f64, title: &str) -> Axes {
    let t = d.series.f("x_LL^0");
    let nrm: Vec<f64> = d.series.f("norm").iter().map(|v| (v - 1.0).abs().max(1e-17)).collect();
    let e = d.series.f("E_expect");
    let de: Vec<f64> = e.iter().map(|v| (v - e[0]).abs().max(1e-17)).collect();
    let mut a = Axes::new(x0, y0, w, h);
    a.logy = true;
    let all: Vec<f64> = nrm.iter().chain(de.iter()).cloned().collect();
    a.autoscale(&t, &all);
    a.line(&t, &nrm, BLUE, 1.5, "");
    a.line(&t, &de, ORANGE, 1.5, "");
    a.add_key(Key::Line(BLUE.into(), 1.5, String::new()), "|norm - 1|");
    a.add_key(Key::Line(ORANGE.into(), 1.5, String::new()), "|⟨H^{lat}_{LL}⟩ - ⟨H^{lat}_{LL}⟩ at the start|");
    a.legend_at = (0.35, 0.05);
    a.title = title.into();
    a.xlabel = "x^{0}_{LL}".into();
    a.ylabel = "change since the start".into();
    a
}

/// the task evolve on a plane (six panels): (a) to (d) the density of the mode (summed over x^3_LL) at four times, with the
/// path of its centre up to that time; (e) the whole path of the centre; (f) the conservation of norm and energy
fn plot_evolve_2d(d: &Data) -> Figure {
    let f = d.fields.as_ref().expect("fields");
    let t = d.series.f("x_LL^0");
    let tf = f.f("x_LL^0");
    let (x, y) = (f.f("x_LL^1"), f.f("x_LL^2"));
    let rho: Vec<f64> = f.f("density_p").iter().zip(f.f("density_h")).map(|(a, b)| a + b).collect();
    let (xs, ys, times) = (unique(&x), unique(&y), unique(&tf));
    let (xm, ym) = (d.series.f("x_mean_1"), d.series.f("x_mean_2"));
    let p4 = pick4(&times);
    // frames[k][row][col]: the density at the time p4[k], row = position in x^2, col = position in x^1
    let frames: Vec<Vec<Vec<f64>>> = p4
        .iter()
        .map(|&tv| {
            let mut z = vec![vec![0.0; xs.len()]; ys.len()];
            for i in 0..tf.len() {
                if (tf[i] - tv).abs() < 1e-12 {
                    z[idx(&ys, y[i])][idx(&xs, x[i])] += rho[i]; // summed over x^3
                }
            }
            z
        })
        .collect();
    let zmax = frames.iter().flatten().flatten().cloned().fold(0.0, f64::max);
    let mut fig = Figure::new(1440.0, 900.0, &d.title());
    let places = [(90.0, 80.0), (480.0, 80.0), (90.0, 500.0), (480.0, 500.0)];
    let ext = (xs[0], xs[xs.len() - 1], ys[0], ys[ys.len() - 1]);
    for (k, ((px, py), &tv)) in places.iter().zip(&p4).enumerate() {
        let mut a = Axes::new(*px, *py, 300.0, 300.0);
        a.xlim = (ext.0, ext.1);
        a.ylim = (ext.2, ext.3);
        a.grid = false;
        a.heatmap(&frames[k], ext, zmax);
        let upto: Vec<usize> = (0..t.len()).filter(|&i| t[i] <= tv + 1e-12).collect();
        let (ux, uy): (Vec<f64>, Vec<f64>) = upto.iter().map(|&i| (xm[i], ym[i])).unzip();
        a.line(&ux, &uy, ORANGE, 1.4, "");
        a.dots(&[ux[ux.len() - 1]], &[uy[uy.len() - 1]], ORANGE, 4.0);
        a.title = format!("({}) x^{{0}}_{{LL}} = {}", ["a", "b", "c", "d"][k], tick_label(tv));
        a.xlabel = "x^{1}_{LL}".into();
        a.ylabel = "x^{2}_{LL}".into();
        fig.axes.push(a);
    }
    fig.colorbar(800.0, 80.0, 14.0, 720.0, zmax, "density of the mode on a site (a to d); orange: path of its centre");
    let mut e = Axes::new(1000.0, 80.0, 300.0, 300.0);
    e.xlim = (ext.0, ext.1);
    e.ylim = (ext.2, ext.3);
    e.line(&xm, &ym, ORANGE, 1.6, "");
    e.dots(&[xm[0]], &[ym[0]], INK, 4.0);
    e.add_key(Key::Line(ORANGE.into(), 1.6, String::new()), "centre of the density");
    e.add_key(Key::Dot(INK.into()), "start");
    e.title = "(e) the path of the centre".into();
    e.xlabel = "x^{1}_{LL}".into();
    e.ylabel = "x^{2}_{LL}".into();
    fig.axes.push(e);
    fig.axes.push(conservation(d, 1000.0, 500.0, 380.0, 300.0, "(f) conservation of norm and energy"));
    fig
}

/// the task spectrum (two panels): (a) all eigenvalues in ascending order, those in the window of the bound states
/// highlighted, with the predicted energies +-E^R_LL of (3.29) and the edges of the continuum for the example kink_spectrum;
/// (b) the pair field and the summed density of the positive eigenvectors in the window
fn plot_spectrum(d: &Data, pred: Option<&str>) -> Figure {
    let sp = d.spectrum.as_ref().expect("spectrum");
    let f = d.fields.as_ref().expect("fields");
    let e = sp.f("E_qp");
    let ix: Vec<f64> = (0..e.len()).map(|k| k as f64).collect();
    let win: Vec<f64> = f.meta.get("spectrum_window").map(|s| s.split_whitespace().filter_map(|x| x.parse().ok()).collect()).unwrap_or(vec![-1.0, 1.0]);
    let mut fig = Figure::new(1240.0, 480.0, &d.title());
    let mut a = Axes::new(90.0, 80.0, 470.0, 320.0);
    a.autoscale(&ix, &e);
    a.hspan(win[0], win[1], GRID);
    let (inw, outw): (Vec<usize>, Vec<usize>) = (0..e.len()).partition(|&k| e[k] >= win[0] && e[k] <= win[1]);
    a.dots(&outw.iter().map(|&k| ix[k]).collect::<Vec<_>>(), &outw.iter().map(|&k| e[k]).collect::<Vec<_>>(), GREY, 1.6);
    a.dots(&inw.iter().map(|&k| ix[k]).collect::<Vec<_>>(), &inw.iter().map(|&k| e[k]).collect::<Vec<_>>(), ORANGE, 3.0);
    a.add_key(Key::Dot(GREY.into()), "eigenvalues outside the window");
    a.add_key(Key::Dot(ORANGE.into()), &format!("eigenvalues in the window [{}, {}]", tick_label(win[0]), tick_label(win[1])));
    let pl = d.summary_value("pair field:");
    if pred == Some("kink_spectrum") {
        // (3.29) with the profile cosh(dist/ell_K)^(-Delta_inf ell_K) of each wall, and the edge of the continuum:
        // the pair field of a wall is Delta_inf tanh(x/ell_K), and the zero mode of the continuum solves
        // d phi/dx = -Delta(x) phi, so phi = cosh(x/ell_K)^(-Delta_inf ell_K); `dist` is the distance from the nearest wall
        // on the ring.  The Rayleigh quotient of this profile with the Wilson term gives the energy E^R_LL of the bound
        // states; the continuum begins at (m^2 + Delta_inf^2)^(1/2).
        let amp = number_after(&pl, "amplitude").unwrap_or(1.0);
        let wid = number_after(&pl, "width").unwrap_or(1.0);
        let pos = list_after(&pl, "positions [");
        let xs = d.series.f("x_LL^1");
        let (ell, m, r) = (d.series.m("dx"), d.series.m("m"), d.series.m("r_W"));
        let l = xs.len() as f64 * ell;
        let phi: Vec<f64> = xs
            .iter()
            .map(|&x| {
                let dist = pos.iter().map(|&p| ((x - p + 0.5 * l).rem_euclid(l) - 0.5 * l).abs()).fold(f64::INFINITY, f64::min);
                (dist / wid).cosh().powf(-amp * wid)
            })
            .collect();
        let nn = phi.len();
        let num: f64 = (0..nn).map(|i| (phi[(i + 1) % nn] - phi[i]).powi(2)).sum();
        let den: f64 = phi.iter().map(|p| p * p).sum();
        let er = m + r / (2.0 * ell) * num / den;
        let edge = m.hypot(amp);
        for s in [1.0, -1.0] {
            a.hline(s * er, INK, 1.0, "5,3");
            a.hline(s * edge, RED, 1.0, "2,3");
        }
        a.add_key(Key::Line(INK.into(), 1.0, "5,3".into()), &format!("prediction ±E^{{R}}_{{LL}} = ±{:.5} (3.29)", er));
        a.add_key(Key::Line(RED.into(), 1.0, "2,3".into()), "edge of the continuum ±(m^{2}_{LL} + (Δ^{∞}_{LL})^{2})^{1/2}");
    }
    a.legend_at = (0.02, 0.62);
    a.title = "(a) all eigenvalues of the lattice operator".into();
    a.xlabel = "number of the eigenvalue (ascending)".into();
    a.ylabel = "E^{qp}_{LL,n}".into();
    fig.axes.push(a);
    let mut b = Axes::new(690.0, 80.0, 500.0, 320.0);
    let xs = d.series.f("x_LL^1");
    let d3 = d.series.f("d3_re");
    // the summed density of the positive eigenvectors in the window, normalized to the integral 1.  The fields file holds
    // one density per group of degenerate eigenvectors (summed over the group, which does not depend on the basis chosen in
    // the degenerate subspace) and its multiplicity; nvec counts the eigenvectors: each distinct positive eigenvalue once,
    // with its multiplicity
    let (fx, ek) = (f.f("x_LL^1"), f.f("E_qp"));
    let rho: Vec<f64> = f.f("density_p").iter().zip(f.f("density_h")).map(|(a, b)| a + b).collect();
    let ux = unique(&fx);
    let mut tot = vec![0.0; ux.len()];
    let mut nvec = 0;
    let mult = f.f("multiplicity");
    let mut seen = vec![];
    for i in 0..fx.len() {
        if ek[i] > 0.0 {
            tot[idx(&ux, fx[i])] += rho[i];
            if !seen.iter().any(|&e: &f64| (e - ek[i]).abs() < 1e-12) {
                seen.push(ek[i]);
                nvec += mult[i] as usize;
            }
        }
    }
    let s: f64 = tot.iter().sum::<f64>() * d.series.m("dx");
    let tot: Vec<f64> = tot.iter().map(|v| v / s).collect();
    let all: Vec<f64> = d3.iter().chain(tot.iter()).cloned().collect();
    let allx: Vec<f64> = xs.iter().chain(ux.iter()).cloned().collect();
    b.autoscale(&allx, &all);
    b.line(&xs, &d3, BLUE, 1.7, "");
    b.line(&ux, &tot, ORANGE, 1.7, "");
    b.add_key(Key::Line(BLUE.into(), 1.7, String::new()), "pair field Re d^{3}_{LL}");
    b.add_key(Key::Line(ORANGE.into(), 1.7, String::new()), &format!("summed density of the {} positive eigenvectors in the window", nvec));
    b.legend_at = (0.12, 0.08);
    b.title = "(b) the pair field and the bound states".into();
    b.xlabel = "x^{1}_{LL}".into();
    b.ylabel = "pair field; density".into();
    fig.axes.push(b);
    fig
}

/// the task ground_state (three panels): (a) the pair field, initial guess and self-consistent, with the root of the gap
/// equation (3.21) for the example ground_state, or the continuum kink for the example kink_ground_state; (b) the
/// convergence of the Anderson iteration; (c) the 80 eigenvalues nearest to zero in the self-consistent field, with the
/// predicted Wilson shift of the bound states of the kink
fn plot_ground_state(d: &Data, pred: Option<&str>) -> Figure {
    let f = d.fields.as_ref().expect("fields");
    let st = f.s("stage");
    let x = f.f("x_LL^1");
    let ad = f.f("abs_d");
    let d3 = f.f("d3_re");
    let sel = |tag: &str| -> Vec<usize> { (0..st.len()).filter(|&i| st[i] == tag).collect() };
    let (ini, fin) = (sel("initial"), sel("final"));
    let pick = |v: &[f64], s: &[usize]| -> Vec<f64> { s.iter().map(|&i| v[i]).collect() };
    let mut fig = Figure::new(1480.0, 480.0, &d.title());
    let mut a = Axes::new(90.0, 80.0, 360.0, 320.0);
    let kink = pred == Some("kink_ground_state");
    let mut ally = pick(&ad, &fin);
    ally.extend(pick(&ad, &ini));
    if kink {
        ally.extend(pick(&d3, &fin));
        ally.extend(pick(&d3, &ini));
    }
    a.autoscale(&x, &ally);
    a.line(&pick(&x, &ini), &pick(&ad, &ini), GREY, 1.6, "5,3");
    a.line(&pick(&x, &fin), &pick(&ad, &fin), BLUE, 1.7, "");
    a.add_key(Key::Line(GREY.into(), 1.6, "5,3".into()), "|d_{LL}|, initial guess");
    a.add_key(Key::Line(BLUE.into(), 1.7, String::new()), "|d_{LL}|, self-consistent");
    let n = d.sites();
    let ch = Chain::of(&d.series, n.first().cloned().unwrap_or(1));
    let g = d.series.m("G");
    if pred == Some("ground_state") {
        let root = ch.gap_root(g, d.series.m("mu"));
        a.hline(root, INK, 1.0, "2,3");
        a.add_key(Key::Line(INK.into(), 1.0, "2,3".into()), &format!("prediction: root of the gap equation (3.21), {:.6}", root));
    }
    let mut wilson = f64::NAN;
    if kink {
        a.line(&pick(&x, &ini), &pick(&d3, &ini), GREY, 1.2, "5,3");
        a.line(&pick(&x, &fin), &pick(&d3, &fin), AQUA, 1.6, "");
        a.add_key(Key::Line(AQUA.into(), 1.6, String::new()), "Re d^{3}_{LL}, self-consistent");
        // the self-consistent kink of the continuum (Dashen, Hasslacher and Neveu): Delta_LL tanh(Delta_LL (x - centre)),
        // with Delta_LL the uniform root of (3.21) at mu_LL = 0, centred between the walls
        let d0 = ch.gap_root(g, 0.0);
        let xc = 0.5 * (n[0] - 1) as f64 * ch.ell;
        let xx: Vec<f64> = (0..400).map(|k| (n[0] - 1) as f64 * ch.ell * k as f64 / 399.0).collect();
        let yy: Vec<f64> = xx.iter().map(|&s| d0 * (d0 * (s - xc)).tanh()).collect();
        a.line(&xx, &yy, INK, 1.0, "2,3");
        a.add_key(Key::Line(INK.into(), 1.0, "2,3".into()), "continuum kink Δ_{LL} tanh(Δ_{LL}(x^{1}_{LL} - centre))");
        // the first-order Wilson shift of the bound states: (3.29) with m = 0 and the zero mode of the computed profile:
        // ln phi(x) = -int_0^x Re d^3_LL dx' by the trapezoidal rule; the maximum of ln phi is subtracted before the
        // exponential (no overflow; the quotient does not depend on the normalization); the sums run over the sites, with
        // the neighbour missing at the last site (the chain has walls)
        let p3 = pick(&d3, &fin);
        let mut lphi = vec![0.0];
        for i in 1..p3.len() {
            let prev = lphi[i - 1];
            lphi.push(prev - 0.5 * (p3[i] + p3[i - 1]) * ch.ell);
        }
        let mx = lphi.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let phi: Vec<f64> = lphi.iter().map(|v| (v - mx).exp()).collect();
        let num: f64 = (1..phi.len()).map(|i| (phi[i] - phi[i - 1]).powi(2)).sum();
        wilson = ch.r / (2.0 * ch.ell) * num / phi.iter().map(|p| p * p).sum::<f64>();
    }
    a.legend_at = if kink { (0.5, 0.05) } else { (0.3, 0.05) };
    a.title = "(a) the pair field".into();
    a.xlabel = "x^{1}_{LL}".into();
    a.ylabel = "pair field".into();
    fig.axes.push(a);
    // (b) the largest component of the residual (the change of the pair field) at each iteration
    let it = d.iterations.as_ref().unwrap_or(&d.series);
    let (k, chg) = (it.f("iteration"), it.f("change"));
    let mut b = Axes::new(580.0, 80.0, 360.0, 320.0);
    b.logy = true;
    b.autoscale(&k, &chg);
    b.line(&k, &chg, BLUE, 1.5, "");
    b.dots(&k, &chg, BLUE, 3.0);
    b.add_key(Key::Line(BLUE.into(), 1.5, String::new()), "largest component of the residual");
    b.legend_at = (0.3, 0.82);
    b.title = "(b) convergence of the Anderson iteration".into();
    b.xlabel = "iteration".into();
    b.ylabel = "change of the pair field".into();
    fig.axes.push(b);
    // (c) the 80 eigenvalues around the middle of the ascending list (the eigenvalues nearest to zero)
    let e = d.spectrum.as_ref().expect("spectrum").f("E_qp");
    let k0 = e.len() / 2;
    let (lo, hi) = (k0.saturating_sub(40), (k0 + 40).min(e.len()));
    let ix: Vec<f64> = (lo..hi).map(|i| i as f64).collect();
    let ev: Vec<f64> = (lo..hi).map(|i| e[i]).collect();
    let mut c = Axes::new(1070.0, 80.0, 360.0, 320.0);
    c.autoscale(&ix, &ev);
    c.dots(&ix, &ev, BLUE, 2.6);
    c.add_key(Key::Dot(BLUE.into()), "eigenvalues near zero");
    if wilson.is_finite() {
        c.hline(wilson, INK, 1.0, "5,3");
        c.hline(-wilson, INK, 1.0, "5,3");
        c.add_key(Key::Line(INK.into(), 1.0, "5,3".into()), &format!("prediction: Wilson shift ±{:.5}", wilson));
    }
    c.legend_at = (0.42, 0.04);
    c.title = "(c) the spectrum in the self-consistent field".into();
    c.xlabel = "number of the eigenvalue (ascending)".into();
    c.ylabel = "E^{qp}_{LL,n}".into();
    fig.axes.push(c);
    fig
}

/// the task quench (three panels): (a) |d_LL| averaged over the sites, with its mean after the quench and, for the example
/// quench, the least-squares sinusoid with the frequency (4.11) of the amplitude mode (only its amplitude, phase and offset
/// are fitted; the frequency is predicted); (b) the quantities that the exact equations conserve, as CVODE keeps them; (c) |d_LL| on the sites
/// at four times (the field stays uniform)
fn plot_quench(d: &Data, pred: Option<&str>) -> Figure {
    let t = d.series.f("x_LL^0");
    let a = d.series.f("abs_d_mean");
    // the mean over the run after the quench (the first entry, at x^0_LL = 0, is the ground state before the quench)
    let mean = a[1..].iter().sum::<f64>() / (a.len() - 1) as f64;
    let mut fig = Figure::new(1480.0, 480.0, &d.title());
    let mut p = Axes::new(90.0, 80.0, 360.0, 320.0);
    p.autoscale(&t, &a);
    p.line(&t, &a, BLUE, 1.7, "");
    p.hline(mean, GREY, 1.0, "5,3");
    p.add_key(Key::Line(BLUE.into(), 1.7, String::new()), "|d_{LL}| (mean over the sites)");
    p.add_key(Key::Line(GREY.into(), 1.0, "5,3".into()), "mean after the quench");
    if pred == Some("quench") {
        let ch = Chain::of(&d.series, d.sites()[0]);
        let w = ch.amplitude_mode(mean);
        let cf = sinusoid(&t[1..], &a[1..], w);
        let tt: Vec<f64> = (0..1500).map(|k| t[1] + (t[t.len() - 1] - t[1]) * k as f64 / 1499.0).collect();
        let yy: Vec<f64> = tt.iter().map(|&s| cf[0] + cf[1] * (w * s).cos() + cf[2] * (w * s).sin()).collect();
        p.line(&tt, &yy, ORANGE, 1.1, "5,3");
        p.add_key(Key::Line(ORANGE.into(), 1.1, "5,3".into()), &format!("prediction: frequency ω^{{H}}_{{LL}} = {:.4} of (4.11)", w));
    }
    p.legend_at = (0.25, 0.03);
    p.title = "(a) the amplitude of the pair field".into();
    p.xlabel = "x^{0}_{LL}".into();
    p.ylabel = "pair field".into();
    fig.axes.push(p);
    // (b) |N^F_LL - N^F_LL(0)|, the orthonormality error of the modes and the relative change of E^MF_LL (log axis)
    let dnf: Vec<f64> = d.series.f("N_F_change").iter().map(|v| v.abs().max(1e-17)).collect();
    let orth: Vec<f64> = d.series.f("ortho_error").iter().map(|v| v.max(1e-17)).collect();
    let emf = d.series.f("E_MF");
    let demf: Vec<f64> = emf.iter().map(|v| ((v - emf[0]) / emf[0]).abs().max(1e-17)).collect();
    let mut q = Axes::new(580.0, 80.0, 360.0, 320.0);
    q.logy = true;
    let all: Vec<f64> = dnf.iter().chain(orth.iter()).chain(demf.iter()).cloned().collect();
    q.autoscale(&t, &all);
    q.line(&t, &dnf, BLUE, 1.4, "");
    q.line(&t, &orth, AQUA, 1.4, "");
    q.line(&t, &demf, ORANGE, 1.4, "");
    q.add_key(Key::Line(BLUE.into(), 1.4, String::new()), "|N^{F}_{LL} - N^{F}_{LL}(0)|");
    q.add_key(Key::Line(AQUA.into(), 1.4, String::new()), "orthonormality error of the modes");
    q.add_key(Key::Line(ORANGE.into(), 1.4, String::new()), "relative change of E^{MF}_{LL}");
    q.legend_at = (0.3, 0.06);
    q.title = "(b) the conserved quantities (CVODE errors)".into();
    q.xlabel = "x^{0}_{LL}".into();
    q.ylabel = "change since the start (0 drawn at 10^{-17})".into();
    fig.axes.push(q);
    // (c) |d_LL| on the sites at four output times
    let f = d.fields.as_ref().expect("fields");
    let (tf, xf, ad) = (f.f("x_LL^0"), f.f("x_LL^1"), f.f("abs_d"));
    let times = unique(&tf);
    let mut r = Axes::new(1070.0, 80.0, 360.0, 320.0);
    r.autoscale(&xf, &ad);
    for (k, &tv) in pick4(&times).iter().enumerate() {
        let s: Vec<usize> = (0..tf.len()).filter(|&i| (tf[i] - tv).abs() < 1e-12).collect();
        let (xx, yy): (Vec<f64>, Vec<f64>) = s.iter().map(|&i| (xf[i], ad[i])).unzip();
        r.line(&xx, &yy, SEQ[k], 1.3, "");
        r.dots(&xx, &yy, SEQ[k], 2.5);
        r.add_key(Key::Line(SEQ[k].into(), 1.3, String::new()), &format!("x^{{0}}_{{LL}} = {}", tick_label(tv)));
    }
    r.legend_at = (0.55, 0.35);
    r.title = "(c) the pair field stays uniform".into();
    r.xlabel = "x^{1}_{LL}".into();
    r.ylabel = "|d_{LL}| on the sites".into();
    fig.axes.push(r);
    fig
}

/// plots the solution BASE: reads its files, recognizes an example by the end of the base name (the longest matching
/// name, so that kink_ground_state is not taken for ground_state), chooses the figure by the task and writes FIG.svg
/// (FIG = the --out name, or BASE_plot); creates the folder of FIG if needed
fn plot(base: &str, out: Option<&str>, predictions: bool) -> Result<String, String> {
    let d = Data::read(base)?;
    let name = std::path::Path::new(base).file_name().map(|x| x.to_string_lossy().to_string()).unwrap_or_default();
    let pred = if predictions { EXAMPLES.iter().copied().filter(|e| name.ends_with(e)).max_by_key(|e| e.len()) } else { None };
    let task = d.series.meta.get("task").cloned().unwrap_or_default();
    let fig = match task.as_str() {
        "evolve" => plot_evolve(&d, pred),
        "spectrum" => plot_spectrum(&d, pred),
        "ground_state" => plot_ground_state(&d, pred),
        "quench" => plot_quench(&d, pred),
        _ => return Err(format!("unknown task '{}' in {}_series.csv", task, base)),
    };
    let path = format!("{}.svg", out.map(|s| s.to_string()).unwrap_or_else(|| format!("{}_plot", base)));
    if let Some(dir) = std::path::Path::new(&path).parent() {
        if !dir.as_os_str().is_empty() {
            fs::create_dir_all(dir).map_err(|e| format!("cannot create {}: {}", dir.display(), e))?;
        }
    }
    fs::write(&path, fig.svg()).map_err(|e| format!("cannot write {}: {}", path, e))?;
    println!("wrote {} (task {}{})", path, task, pred.map(|p| format!(", with the predictions of the example {}", p)).unwrap_or_default());
    Ok(path)
}

/// the command line: base names of solutions, --out FIG, --no-predictions; without a base name the program lists the
/// solutions of the current folder and asks for one.  Exit code 0 on success, 2 on an error (with a message).
fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let predictions = !args.iter().any(|a| a == "--no-predictions");
    let mut rest: Vec<String> = args.iter().filter(|a| *a != "--no-predictions").cloned().collect();
    let mut out = None;
    if let Some(i) = rest.iter().position(|a| a == "--out") {
        if i + 1 >= rest.len() {
            println!("--out needs a name for the figure");
            std::process::exit(2);
        }
        out = Some(rest[i + 1].clone());
        rest.drain(i..i + 2);
    }
    if rest.is_empty() {
        println!("t2_plot: plots a solution of the solvers of Test 2 (writes Scalable Vector Graphics, .svg).");
        if let Ok(entries) = fs::read_dir(".") {
            let mut found: Vec<String> = entries.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().to_string()).filter(|n| n.ends_with("_summary.txt")).map(|n| n.trim_end_matches("_summary.txt").to_string()).collect();
            found.sort();
            if !found.is_empty() {
                println!("Solutions in this folder: {}", found.join(", "));
            }
        }
        print!("Base name of the solution (the file names without _summary.txt, _series.csv, ...): ");
        io::stdout().flush().ok();
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).ok();
        if line.trim().is_empty() {
            return;
        }
        rest.push(line.trim().to_string());
    }
    let mut code = 0;
    // a base name may also be given as the name of its summary or series file
    for b in &rest {
        let base = b.trim_end_matches("_summary.txt").trim_end_matches("_series.csv").to_string();
        if let Err(e) = plot(&base, out.as_deref(), predictions) {
            println!("error: {}", e);
            code = 2;
        }
    }
    std::process::exit(code);
}
