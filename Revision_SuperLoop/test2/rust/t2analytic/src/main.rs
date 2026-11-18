//! t2_analytic -- PROJECT spacetimes, Revision_SuperLoop, folder test2, item [2] of Test 2.
//!
//! Analytic solutions of the gauge-covariant field equations of the self-interacting spinor field
//! Psi_LL on R^(1,3)_LL, in their mean-field (Dirac-Bogoliubov-de Gennes) form (2.31)-(2.32) of item [1],
//! each one checked against the lattice form (2.34)-(2.36) that the solvers of items [3] and [5] integrate.
//! All computations are in the canonical gauge of the frame (Gamma_LL_p = 0); a gauge transformation
//! (2.10) carries every solution to any other frame.
//!
//!   Case A  free spinor charges in constant gauge potentials:
//!           AS-1  the spectrum: the momentum is shifted by the potential of each isospin component
//!                 (minimal coupling q -> q - Acal_up_1) and the energy by its scalar potential;
//!           AS-2  a constant isospin potential along tau_1 rotates the isospin (time evolution, CVODE);
//!   Case B  the uniform polar superfluid dvec = (0, 0, Delta):
//!           AS-3  the Bogoliubov modes (continuum identity and lattice spectrum);
//!           AS-4  the particle-hole (Rabi) oscillation of an injected particle (CVODE);
//!           AS-5  the gap equation (2.33) in closed form for massless spinor charges;
//!           AS-6  the uniform state is an exact self-consistent solution of the lattice equations;
//!   Case C  a domain wall (kink) of the pair field:
//!           AS-7  bound states with the energies +m and -m (continuum);
//!           AS-8  the square of the operator and the complete spectrum of the bound states of a wall;
//!           AS-9  the wall on the lattice: spectrum, densities, Rayleigh quotient and its convergence;
//!   Case D  constant magnetic fields:
//!           AS-10 the Landau levels of the spinor charges.
//!
//! Output (folder test2/data): the report t2_analytic_checks.txt and the data files t2_A_*.csv,
//! t2_B_*.csv, t2_C_*.csv and t2_D_*.csv of the Figures 3 to 6 of the document.
//!
//! Time integration (AS-2, AS-4): CVODE of SUNDIALS 7.8.0 in pure Rust (vendored in the folder
//! vendor/sundials_rs-7.8.0 of PROJECT spacetimes), Adams-Moulton formulas of the orders 1 to 12 with
//! local error control, fixed-point iteration of the corrector, error weights 1/(rtol |w_k| + atol) of the
//! complex components w_k with rtol = 1e-12 and atol = 1e-14 (module t2core::cvode).  CVODE chooses its
//! steps itself and returns the state at the requested output times by interpolation.
//!
//! Build and run (from the folder Revision_SuperLoop/test2/rust):
//!     cargo build --release && ./target/release/t2_analytic
//! The program exits with status 1 if a check fails.  Arithmetic: double precision; every check names
//! its tolerance; random numbers from a fixed seed, so that every run gives the same numbers.
//!
//! Names in the comments and in the output (the plain-text names of the registry of the folder):
//!   m, mu, Delta, dvec, q, eps_lat, dx, r_W         mass, chemical potential, gap, order parameter,
//!                                                   momentum, lattice energy, lattice spacing,
//!                                                   Wilson coefficient
//!   rtol, atol                                      relative and absolute tolerance of CVODE
//!   g_v, g_F, Av_LL^iota_p, AF_LL_p, Acal_p         couplings and gauge potentials of SU(2)_v and U(1)_F,
//!                                                   Acal_p = g_v Av_LL^iota_p T_LL^iota + g_F AF_LL_p
//!   Acal_up_p, Acal_down_p                          the eigenvalues of Acal_p on the isospin-up and
//!                                                   isospin-down components (potentials along tau_3)
//!   h(q)                                            the 8 x 8 plane-wave Hamiltonian sum_j alpha_LL^j q_j + beta_LL m
//!   Kch_3                                           the scalar pair matrix of the isospin direction 3
//!   E^+, E^-                                        ((eps +- mu)^2 + Delta^2)^(1/2)
//!   theta_B, chi_pw                                 Bogoliubov angle, constant spinor of a plane wave
//!   omega_pr, P_up, P_hole                          precession frequency, probabilities of isospin up
//!                                                   and of the hole
//!   Delta_inf, ell_K, phi_K, chi_K, E_R             kink: asymptotic gap, width, profile of the bound
//!                                                   state, its constant spinor, Rayleigh quotient
//!   B_up, B_down, n_L                               magnetic fields felt by the isospin components,
//!                                                   number of a Landau level
use std::fs;
use std::path::PathBuf;

use t2core::c64::{c, C64};
use t2core::cmat::{eigh, CMat};
use t2core::cvode::{self, CvodeSettings};
use t2core::lattice::{self, Fields, Lattice, Mats};
use t2core::model::{self, kpair, lift4};
use t2core::rng::Rng;

const PI: f64 = std::f64::consts::PI;

// ------------------------------------------------------------------ report and data files

/// the list of checks: identifier, statement, result, numbers found
struct Checks {
    title: String,
    items: Vec<(String, String, bool, String)>,
}

impl Checks {
    fn add(&mut self, id: &str, statement: &str, ok: bool, detail: String) {
        println!("[{}] {}  {}", if ok { "PASS" } else { "FAIL" }, id, detail);
        self.items.push((id.to_string(), statement.to_string(), ok, detail));
    }
    fn write(&self, path: &PathBuf) -> bool {
        let npass = self.items.iter().filter(|x| x.2).count();
        let mut s = format!("{}\n{} of {} checks passed\n\n", self.title, npass, self.items.len());
        for (id, st, ok, det) in &self.items {
            s.push_str(&format!(
                "[{}] {}  {}{}\n",
                if *ok { "PASS" } else { "FAIL" },
                id,
                st,
                if det.is_empty() { String::new() } else { format!("  -- {}", det) }
            ));
        }
        fs::write(path, s).expect("write report");
        npass == self.items.len()
    }
}

/// a data file: comment lines "# key: value" (the first one is "# content: ..."), a header line, rows
struct Csv {
    text: String,
}

impl Csv {
    fn new(content: &str, meta: &[(&str, String)], header: &str) -> Csv {
        let mut text = format!("# content: {}\n", content);
        for (k, v) in meta {
            text.push_str(&format!("# {}: {}\n", k, v));
        }
        text.push_str(header);
        text.push('\n');
        Csv { text }
    }
    fn row(&mut self, fields: &[String]) {
        self.text.push_str(&fields.join(","));
        self.text.push('\n');
    }
    fn write(&self, dir: &PathBuf, name: &str) {
        fs::write(dir.join(name), &self.text).expect("write data file");
    }
}

/// the folder test2/data (two levels above the folder of this crate)
fn data_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p.push("data");
    fs::create_dir_all(&p).unwrap();
    p
}

fn sci(x: f64) -> String {
    format!("{:.1e}", x)
}

fn f10(x: f64) -> String {
    format!("{:.10}", x)
}

fn vnorm(v: &[C64]) -> f64 {
    v.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
}

fn vdiff(a: &[C64], b: &[C64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (*x - *y).norm_sqr()).sum::<f64>().sqrt()
}

fn sorted(mut v: Vec<f64>) -> Vec<f64> {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

/// the allowed momentum of the plane wave number kq of a periodic chain of n sites, in (-pi/dx, pi/dx]
fn chain_momentum(kq: usize, n: usize, ell: f64) -> f64 {
    let k = if kq <= n / 2 { kq as f64 } else { kq as f64 - n as f64 };
    2.0 * PI * k / (n as f64 * ell)
}

/// The 16 x 16 matrix of the lattice operator on the plane waves exp(i q x_LL^1) e_b of a periodic chain
/// (b = 0..15 the components of a site).  With fields that are the same on every site, the image of a
/// plane wave is again a plane wave of the same momentum; its 16 components at the site 0 are the column b.
fn plane_matrix(lat: &Lattice, fl: &Fields, mats: &Mats, q: f64) -> CMat {
    let ns = lat.nsites();
    let mut m16 = CMat::zeros(16);
    for b in 0..16 {
        let mut w = vec![C64::ZERO; 16 * ns];
        for s in 0..ns {
            w[16 * s + b] = C64::cis(q * lat.ell * s as f64);
        }
        let mut out = vec![C64::ZERO; 16 * ns];
        lat.apply(fl, mats, &w, &mut out);
        for a in 0..16 {
            m16.set(a, b, out[a]);
        }
    }
    m16
}

/// the rows and columns idx of a square matrix
fn select(m: &CMat, idx: &[usize]) -> CMat {
    let mut r = CMat::zeros(idx.len());
    for (i, &a) in idx.iter().enumerate() {
        for (j, &b) in idx.iter().enumerate() {
            r.set(i, j, m.get(a, b));
        }
    }
    r
}

/// h(q) = sum_j alpha_LL^j q_j + beta_LL m on Dirac (x) isospin (8 x 8), the plane-wave Hamiltonian of the continuum
fn h8(q: &[f64; 3], m: f64) -> CMat {
    let mut h = lift4(&model::beta()).scale_r(m);
    for j in 0..3 {
        h = h.add(&lift4(&model::alpha(j + 1)).scale_r(q[j]));
    }
    h
}

/// Gauss-Legendre nodes and weights on [-1, 1] (Newton iteration on the Legendre polynomial of degree n)
fn gauss_legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut x = vec![0.0; n];
    let mut w = vec![0.0; n];
    for i in 0..n {
        let mut z = (PI * (i as f64 + 0.75) / (n as f64 + 0.5)).cos();
        for _ in 0..100 {
            let (mut p1, mut p2) = (1.0f64, 0.0f64);
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                p1 = ((2.0 * j as f64 + 1.0) * z * p2 - j as f64 * p3) / (j as f64 + 1.0);
            }
            let pp = n as f64 * (z * p1 - p2) / (z * z - 1.0);
            let z1 = z;
            z = z1 - p1 / pp;
            if (z - z1).abs() < 1e-15 {
                break;
            }
        }
        // recompute the derivative at the converged node for the weight
        let (mut p1, mut p2) = (1.0f64, 0.0f64);
        for j in 0..n {
            let p3 = p2;
            p2 = p1;
            p1 = ((2.0 * j as f64 + 1.0) * z * p2 - j as f64 * p3) / (j as f64 + 1.0);
        }
        let pp = n as f64 * (z * p1 - p2) / (z * z - 1.0);
        x[i] = -z;
        w[i] = 2.0 / ((1.0 - z * z) * pp * pp);
    }
    (x, w)
}

/// The right side of the gap equation (2.33) for massless spinor charges (m = 0, eps = q) in closed form:
///   int_{q<Lambda} d^3q/(2 pi)^3 (1/E^+ + 1/E^-),   E^(+-) = ((q +- mu)^2 + Delta^2)^(1/2),
///   = (1/(4 pi^2)) [ (Lambda - 3 mu) E^+(Lambda) + (Lambda + 3 mu) E^-(Lambda)
///                    + (2 mu^2 - Delta^2) (asinh((Lambda + mu)/Delta) + asinh((Lambda - mu)/Delta)) ].
/// Derivation (document, Section 3.3): d^3q/(2 pi)^3 = q^2 dq/(2 pi^2); with y = q +- mu,
/// int y^2/(y^2 + Delta^2)^(1/2) dy = y r/2 - (Delta^2/2) asinh(y/Delta), int y/r dy = r, int 1/r dy = asinh(y/Delta)
/// (r = (y^2 + Delta^2)^(1/2)); the contributions of the lower limit q = 0 cancel between E^+ and E^-.
fn gap_integral_closed(lam: f64, mu: f64, del: f64) -> f64 {
    let ep = ((lam + mu).powi(2) + del * del).sqrt();
    let em = ((lam - mu).powi(2) + del * del).sqrt();
    ((lam - 3.0 * mu) * ep + (lam + 3.0 * mu) * em + (2.0 * mu * mu - del * del) * (((lam + mu) / del).asinh() + ((lam - mu) / del).asinh()))
        / (4.0 * PI * PI)
}

/// the same integral (for a mass m) by composite Gauss-Legendre quadrature in q, with panels refined
/// geometrically towards the Fermi momentum, where 1/E^- has its sharp maximum 1/Delta
fn gap_integral_quad(lam: f64, mu: f64, del: f64, m: f64, gl: &(Vec<f64>, Vec<f64>)) -> f64 {
    let integrand = |q: f64| -> f64 {
        let eps = (q * q + m * m).sqrt();
        let ep = ((eps + mu).powi(2) + del * del).sqrt();
        let em = ((eps - mu).powi(2) + del * del).sqrt();
        q * q * (1.0 / ep + 1.0 / em) / (2.0 * PI * PI)
    };
    let kf = if mu > m { (mu * mu - m * m).sqrt() } else { -1.0 };
    let mut pts = vec![0.0, lam];
    if kf > 0.0 && kf < lam {
        pts.push(kf);
        let mut h = del.max(1e-6) / 4.0;
        while h < lam {
            if kf - h > 0.0 {
                pts.push(kf - h);
            }
            if kf + h < lam {
                pts.push(kf + h);
            }
            h *= 2.0;
        }
    }
    for i in 1..64 {
        pts.push(lam * i as f64 / 64.0);
    }
    let mut pts = sorted(pts);
    pts.dedup_by(|a, b| (*a - *b).abs() < 1e-14);
    let mut tot = 0.0;
    for k in 0..pts.len() - 1 {
        let (a, b) = (pts[k], pts[k + 1]);
        let (xm, xr) = (0.5 * (a + b), 0.5 * (b - a));
        for (x, w) in gl.0.iter().zip(&gl.1) {
            tot += w * xr * integrand(xm + xr * x);
        }
    }
    tot
}

/// the gap Delta that solves 1/G = gap_integral_closed(Lambda, mu, Delta), by bisection (the right side
/// decreases with Delta); 0 if there is no solution above 1e-12
fn solve_gap(g: f64, lam: f64, mu: f64) -> f64 {
    let target = 1.0 / g;
    let (mut lo, mut hi) = (1e-12, 20.0);
    if gap_integral_closed(lam, mu, lo) < target {
        return 0.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if gap_integral_closed(lam, mu, mid) > target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// The number of eigenvalues below sigma of the real symmetric tridiagonal matrix with the diagonal `a` and
/// the constant off-diagonal `b` (Sturm sequence: the number of negative pivots of the LDL^T factorization
/// of the matrix minus sigma).
fn sturm_count(a: &[f64], b: f64, sigma: f64) -> usize {
    let mut cnt = 0usize;
    let mut d = 1.0f64;
    for (i, &ai) in a.iter().enumerate() {
        d = (ai - sigma) - if i == 0 { 0.0 } else { b * b / d };
        if d == 0.0 {
            d = -1e-300;
        }
        if d < 0.0 {
            cnt += 1;
        }
    }
    cnt
}

/// the eigenvalues number 0, 1, ..., k-1 (ascending) below `upper` of the tridiagonal matrix, by bisection
fn sturm_lowest(a: &[f64], b: f64, lower: f64, upper: f64) -> Vec<f64> {
    let n_below = sturm_count(a, b, upper);
    let mut out = vec![];
    for k in 0..n_below {
        let (mut lo, mut hi) = (lower, upper);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if sturm_count(a, b, mid) > k {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        out.push(0.5 * (lo + hi));
    }
    out
}

/// The bound-state eigenvalues (below the continuum threshold Delta_inf^2) of the scalar operator
/// -d^2/dx^2 + Delta^2 + sgn Delta' with Delta = Delta_inf tanh(x/ell_K), by second-order finite differences
/// on (-xmax, xmax) with Dirichlet ends and n interior points, extrapolated (Richardson) from n and 2n+1
/// points (the error of the extrapolation is of fourth order in the spacing).
fn pt_bound_states(dinf: f64, ellk: f64, sgn: f64, xmax: f64, n: usize) -> Vec<f64> {
    let solve = |np: usize| -> Vec<f64> {
        let h = 2.0 * xmax / (np as f64 + 1.0);
        let a: Vec<f64> = (1..=np)
            .map(|i| {
                let x = -xmax + i as f64 * h;
                let del = dinf * (x / ellk).tanh();
                let ddel = dinf / ellk / (x / ellk).cosh().powi(2);
                2.0 / (h * h) + del * del + sgn * ddel
            })
            .collect();
        let vmin = a.iter().cloned().fold(f64::INFINITY, f64::min) - 4.0 / (h * h);
        sturm_lowest(&a, -1.0 / (h * h), vmin, dinf * dinf)
    };
    let e1 = solve(n);
    let e2 = solve(2 * n + 1);
    e1.iter().zip(&e2).map(|(x1, x2)| (4.0 * x2 - x1) / 3.0).collect()
}

fn main() {
    let dd = data_dir();
    let mut ck = Checks {
        title: "Revision_SuperLoop, folder test2, program `t2_analytic`: analytic solutions of the gauge-covariant field equations of the spinor field Psi_LL on R^(1,3)_LL, checked against their lattice form (Test 2, item 2)".into(),
        items: vec![],
    };
    let mut rng = Rng(2026_11_17_08);
    let (gv, gf) = (0.7, 0.45);
    let mass = 0.4;
    let mats = Mats::new();
    let act = [true, false, false];
    let up_idx = [0usize, 2, 4, 6];
    let down_idx = [1usize, 3, 5, 7];

    // ================================================================ Case A, AS-1: constant potentials
    // Isospin potentials along tau_3 and the U(1)_F potential, constant in space and time:
    // Av_LL^3_1 = a3, AF_LL_1 = af (in the links), Av_LL^3_0 = b3, AF_LL_0 = bf (in the potential matrix).
    // The isospin component with Q_LL = T_LL^3 = +1/2 (up) or -1/2 (down) feels Acal_p = g_F AF_LL_p + Q_LL g_v Av_LL^3_p.
    let (a3, af, b3, bf, mu_a) = (0.8, 0.6, 0.5, 0.3, 0.2);
    let (na, ella) = (32usize, 0.5);
    let lat_a = Lattice { n: [na, 1, 1], ell: ella, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_a };
    let mut fl_a = Fields::trivial(na);
    for s in 0..na {
        fl_a.links[s][0] = lattice::link(gv, gf, &[0.0, 0.0, a3], af, ella);
        fl_a.v0[s] = lattice::potential(gv, gf, &[0.0, 0.0, b3], bf);
    }
    let acal1 = |qq: f64| gf * af + qq * gv * a3; // Acal_up_1 (qq = +1/2), Acal_down_1 (qq = -1/2)
    let acal0 = |qq: f64| gf * bf + qq * gv * b3; // Acal_up_0, Acal_down_0
    let mut dev_a1 = 0.0f64;
    let mut pts_a = Csv::new(
        "the eigenvalues of the particle block of the lattice operator on the plane waves of a periodic chain of 32 sites with constant gauge potentials, separately for the isospin components Q_LL = +1/2 (up) and -1/2 (down) (Test 2, item 2, Figure 3(a), points)",
        &[("m", format!("{}", mass)), ("dx", format!("{}", ella)), ("r_W", "1".into()), ("mu", format!("{}", mu_a)), ("g_v", format!("{}", gv)), ("g_F", format!("{}", gf)),
          ("Av_LL^3_1", format!("{}", a3)), ("AF_LL_1", format!("{}", af)), ("Av_LL^3_0", format!("{}", b3)), ("AF_LL_0", format!("{}", bf))],
        "q,Q_LL,E_qp",
    );
    for kq in 0..na {
        let q = chain_momentum(kq, na, ella);
        let m16 = plane_matrix(&lat_a, &fl_a, &mats, q);
        // all 16 eigenvalues: particles +-eps_lat(q - Acal_1) - Acal_0 - mu, holes -(+-eps_lat(q + Acal_1) - Acal_0 - mu)
        let (ev, _) = eigh(&m16);
        let mut expect = vec![];
        for &qq in &[0.5, -0.5] {
            let ep = lattice::eps_lat(&[q - acal1(qq), 0.0, 0.0], ella, mass, 1.0, &act);
            let eh = lattice::eps_lat(&[q + acal1(qq), 0.0, 0.0], ella, mass, 1.0, &act);
            for _ in 0..2 {
                expect.extend_from_slice(&[ep - acal0(qq) - mu_a, -ep - acal0(qq) - mu_a, eh + acal0(qq) + mu_a, -eh + acal0(qq) + mu_a]);
            }
        }
        let expect = sorted(expect);
        for i in 0..16 {
            dev_a1 = dev_a1.max((ev[i] - expect[i]).abs());
        }
        // the particle block (rows and columns 0..7), restricted to each isospin component
        let pblock = m16.sub_block(0, 0, 8, 8);
        for (&qq, idx) in [0.5f64, -0.5].iter().zip([&up_idx, &down_idx]) {
            let (evq, _) = eigh(&select(&pblock, idx));
            let ep = lattice::eps_lat(&[q - acal1(qq), 0.0, 0.0], ella, mass, 1.0, &act);
            let ex = sorted(vec![-ep - acal0(qq) - mu_a, -ep - acal0(qq) - mu_a, ep - acal0(qq) - mu_a, ep - acal0(qq) - mu_a]);
            for i in 0..4 {
                dev_a1 = dev_a1.max((evq[i] - ex[i]).abs());
                pts_a.row(&[f10(q), format!("{}", qq), f10(evq[i])]);
            }
        }
        // the particle block does not mix the two isospin components (the potentials point along tau_3)
        for &a in &up_idx {
            for &b in &down_idx {
                dev_a1 = dev_a1.max(pblock.get(a, b).abs());
            }
        }
    }
    pts_a.write(&dd, "t2_A_dispersion_lattice.csv");
    let mut cur_a = Csv::new(
        "the energies of the particle block for plane waves in constant gauge potentials, from the formula E_qp = +-eps_lat(q - Acal_1) - Acal_0 - mu with Acal_p = Acal_up_p (Q_LL = +1/2) or Acal_down_p (Q_LL = -1/2) (Test 2, item 2, Figure 3(a), curves)",
        &[("m", format!("{}", mass)), ("dx", format!("{}", ella)), ("r_W", "1".into()), ("mu", format!("{}", mu_a)),
          ("Acal_up_1", f10(acal1(0.5))), ("Acal_down_1", f10(acal1(-0.5))), ("Acal_up_0", f10(acal0(0.5))), ("Acal_down_0", f10(acal0(-0.5)))],
        "q,Q_LL,branch,E_qp",
    );
    for i in 0..=400 {
        let q = -PI / ella + 2.0 * PI / ella * i as f64 / 400.0;
        for &qq in &[0.5, -0.5] {
            let ep = lattice::eps_lat(&[q - acal1(qq), 0.0, 0.0], ella, mass, 1.0, &act);
            cur_a.row(&[f10(q), format!("{}", qq), "positive".into(), f10(ep - acal0(qq) - mu_a)]);
            cur_a.row(&[f10(q), format!("{}", qq), "negative".into(), f10(-ep - acal0(qq) - mu_a)]);
        }
    }
    cur_a.write(&dd, "t2_A_dispersion_curves.csv");
    ck.add(
        "AS-1",
        "free spinor charges in constant gauge potentials (case A): on a periodic chain of 32 sites along x_LL^1 (dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, g_v = 0.7, g_F = 0.45) with the constant potentials Av_LL^3_1 = 0.8 and AF_LL_1 = 0.6 (in the links) and Av_LL^3_0 = 0.5 and AF_LL_0 = 0.3 (in the potential matrix), and no pair field, the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum q are, for each isospin component (Q_LL = +1/2 with Acal_up_p = g_F AF_LL_p + g_v Av_LL^3_p/2, Q_LL = -1/2 with Acal_down_p = g_F AF_LL_p - g_v Av_LL^3_p/2), +-eps_lat(q - Acal_1) - Acal_0 - mu (particles) and -(+-eps_lat(q + Acal_1) - Acal_0 - mu) (holes), each twice: the potential of the space direction shifts the momentum (minimal coupling q -> q - Acal_1) and the potential of the time direction shifts the energy; the particle block does not mix the two isospin components, and its restriction to each component has the eigenvalues +-eps_lat(q - Acal_1) - Acal_0 - mu (tolerance 1e-12)",
        dev_a1 < 1e-12,
        format!("largest deviation {}; Acal_up_1 = {:.4}, Acal_down_1 = {:.4}, Acal_up_0 = {:.4}, Acal_down_0 = {:.4}", sci(dev_a1), acal1(0.5), acal1(-0.5), acal0(0.5), acal0(-0.5)),
    );

    // ================================================================ Case A, AS-2: isospin precession
    // A constant isospin potential along tau_1, Av_LL^1_0 = apre; no links, no pair field.  The initial state
    // is the positive-energy lattice plane wave chi_pw (x) (isospin up).  Exact solution of i d_0 psi_p = (h - Acal_0 - mu) psi_p:
    //   psi_p(x^0) = exp(-i (eps_lat - mu) x^0) chi_pw (x) (cos(omega_pr x^0/2), i sin(omega_pr x^0/2)) exp(i q x^1),
    // omega_pr = g_v Av_LL^1_0, because exp(+i x^0 Acal_0) = exp(i (omega_pr x^0/2) tau_1) rotates the isospin.
    let (np, ellp, mu_p, apre) = (16usize, 0.5, 0.2, 1.5);
    let lat_p = Lattice { n: [np, 1, 1], ell: ellp, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_p };
    let mut fl_p = Fields::trivial(np);
    for s in 0..np {
        fl_p.v0[s] = lattice::potential(gv, gf, &[apre, 0.0, 0.0], 0.0);
    }
    let qp = chain_momentum(2, np, ellp);
    let h4p = lattice::h_lat_plane(&[qp, 0.0, 0.0], ellp, mass, 1.0, &act);
    let (e4p, v4p) = eigh(&h4p);
    let eps_p = e4p[3];
    let chi4p: Vec<C64> = (0..4).map(|a| v4p.get(a, 3)).collect();
    let omega_pr = gv * apre;
    let exact_a2 = |t: f64| -> Vec<C64> {
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
    // CVODE settings of every time evolution of this program (AS-2 and AS-4)
    let cv = CvodeSettings::adams(1e-12, 1e-14);
    let w0_a2 = exact_a2(0.0);
    // the 101 output times x^0 = 0, 0.16, ..., 16
    let touts_a2 = cvode::uniform_times(0.0, 0.16, 100);
    let mut dev_a2 = 0.0f64;
    let mut prec = Csv::new(
        "the probability P_up of isospin up of a plane wave in a constant isospin potential along tau_1, from the CVODE integration of the lattice equations and from the exact solution cos^2(omega_pr x_LL^0/2), omega_pr = g_v Av_LL^1_0 (Test 2, item 2, Figure 3(b))",
        &[("m", format!("{}", mass)), ("dx", format!("{}", ellp)), ("integrator", "CVODE (SUNDIALS 7.8.0) Adams-Moulton".into()), ("rtol", format!("{:e}", cv.rtol)), ("atol", format!("{:e}", cv.atol)), ("r_W", "1".into()), ("mu", format!("{}", mu_p)), ("Av_LL^1_0", format!("{}", apre)), ("g_v", format!("{}", gv)), ("q", f10(qp)), ("omega_pr", f10(omega_pr))],
        "x_LL^0,P_up_cvode,P_up_exact",
    );
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
    let st_a2 = cvode::evolve_lattice(&lat_p, &fl_p, &w0_a2, &touts_a2, &cv, |_i, t, w| {
        let ex = exact_a2(t);
        dev_a2 = dev_a2.max(vdiff(w, &ex) / vnorm(&ex));
        prec.row(&[format!("{:.4}", t), f10(p_up(w)), f10((0.5 * omega_pr * t).cos().powi(2))]);
    })
    .expect("CVODE integration of AS-2");
    prec.write(&dd, "t2_A_precession.csv");
    ck.add(
        "AS-2",
        "isospin precession in a constant isospin potential (case A): in the potential Av_LL^1_0 = 1.5 (along tau_1, no other field) the positive-energy lattice plane wave chi_pw (x) (isospin up) evolves as exp(-i (eps_lat - mu) x_LL^0) chi_pw (x) (cos(omega_pr x_LL^0/2), i sin(omega_pr x_LL^0/2)) with omega_pr = g_v Av_LL^1_0, so that P_up = cos^2(omega_pr x_LL^0/2); the CVODE integration of the lattice equations (Adams-Moulton formulas with local error control, rtol = 1e-12, atol = 1e-14; periodic chain of 16 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, momentum 2 pi 2/(16 dx)) agrees with this exact solution at the 101 output times x_LL^0 = 0, 0.16, ..., 16 (relative deviation of the whole state, tolerance 1e-9; the numbers of steps and evaluations of CVODE are stated)",
        dev_a2 < 1e-9,
        format!("largest relative deviation {}; CVODE: {}; omega_pr = {:.4}, eps_lat = {:.6}", sci(dev_a2), st_a2.describe(), omega_pr, eps_p),
    );

    // ================================================================ Case B, AS-3: Bogoliubov modes
    let k3 = kpair(3);
    let mut dev_b1 = 0.0f64;
    for _ in 0..50 {
        let q = [rng.unit() * 2.0, rng.unit() * 2.0, rng.unit() * 2.0];
        let mu = rng.unit().abs();
        let del = 0.1 + 0.9 * rng.unit().abs();
        let hq = h8(&q, mass);
        let hmq_c = h8(&[-q[0], -q[1], -q[2]], mass).conj();
        // the identity Kch_3 h(q) = conj(h(-q)) Kch_3
        dev_b1 = dev_b1.max(k3.mul(&hq).sub(&hmq_c.mul(&k3)).fro());
        // H_BdG(q) = [[h(q) - mu, Ppair], [Ppair^dag, -(conj(h(-q)) - mu)]], Ppair = Delta Kch_3^dag
        let p = model::pair_potential(&[C64::ZERO, C64::ZERO, c(del, 0.0)]);
        let id8 = CMat::identity(8);
        let hbdg = CMat::blocks(&hq.sub(&id8.scale_r(mu)), &p, &p.dagger(), &hmq_c.sub(&id8.scale_r(mu)).scale_r(-1.0));
        let (ev, vecs) = eigh(&hq);
        let eps = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + mass * mass).sqrt();
        for (col, sign) in [(7usize, 1.0f64), (0usize, -1.0f64)] {
            dev_b1 = dev_b1.max((ev[col] - sign * eps).abs());
            let chi: Vec<C64> = (0..8).map(|a| vecs.get(a, col)).collect();
            let kchi = k3.matvec(&chi);
            let xe = sign * eps - mu; // +eps - mu or -eps - mu
            let e = (xe * xe + del * del).sqrt();
            let cth = (0.5 * (1.0 + xe / e)).sqrt(); // cos theta_B
            let sth = (0.5 * (1.0 - xe / e)).sqrt(); // sin theta_B
            let mut v16 = vec![C64::ZERO; 16];
            for a in 0..8 {
                v16[a] = chi[a].scale(cth);
                v16[8 + a] = kchi[a].scale(sth);
            }
            let hv = hbdg.matvec(&v16);
            let res: Vec<C64> = hv.iter().zip(&v16).map(|(x, y)| *x - y.scale(e)).collect();
            dev_b1 = dev_b1.max(vnorm(&res));
            let target = if sign > 0.0 { ((eps - mu).powi(2) + del * del).sqrt() } else { ((eps + mu).powi(2) + del * del).sqrt() };
            dev_b1 = dev_b1.max((e - target).abs());
        }
    }
    // the same on the lattice: plane waves of a periodic chain with the uniform pair field dvec = (0, 0, Delta)
    let (nbg, ellb, mu_b, del_b) = (32usize, 0.5, 0.6, 0.3);
    let lat_bg = Lattice { n: [nbg, 1, 1], ell: ellb, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_b };
    let mut fl_bg = Fields::trivial(nbg);
    for s in 0..nbg {
        fl_bg.d[s] = [C64::ZERO, C64::ZERO, c(del_b, 0.0)];
    }
    let mut dev_b1l = 0.0f64;
    let mut bog_pts = Csv::new(
        "the positive eigenvalues of the lattice operator on the plane waves of a periodic chain of 32 sites with the uniform pair field dvec = (0, 0, Delta) (Test 2, item 2, Figure 4(a), points)",
        &[("m", format!("{}", mass)), ("dx", format!("{}", ellb)), ("r_W", "1".into()), ("mu", format!("{}", mu_b)), ("Delta", format!("{}", del_b))],
        "q,E_qp",
    );
    for kq in 0..nbg {
        let q = chain_momentum(kq, nbg, ellb);
        let (ev, _) = eigh(&plane_matrix(&lat_bg, &fl_bg, &mats, q));
        let el = lattice::eps_lat(&[q, 0.0, 0.0], ellb, mass, 1.0, &act);
        let em = ((el - mu_b).powi(2) + del_b * del_b).sqrt();
        let ep = ((el + mu_b).powi(2) + del_b * del_b).sqrt();
        let mut ex = vec![];
        for _ in 0..4 {
            ex.extend_from_slice(&[em, -em, ep, -ep]);
        }
        let ex = sorted(ex);
        for i in 0..16 {
            dev_b1l = dev_b1l.max((ev[i] - ex[i]).abs());
            if ev[i] > 0.0 {
                bog_pts.row(&[f10(q), f10(ev[i])]);
            }
        }
    }
    bog_pts.write(&dd, "t2_B_bogoliubov_lattice.csv");
    let mut bog_cur = Csv::new(
        "the lattice energy eps_lat and the quasi-particle energies E^- = ((eps_lat - mu)^2 + Delta^2)^(1/2) and E^+ = ((eps_lat + mu)^2 + Delta^2)^(1/2) of the uniform polar superfluid on the lattice (Test 2, item 2, Figure 4(a), curves)",
        &[("m", format!("{}", mass)), ("dx", format!("{}", ellb)), ("r_W", "1".into()), ("mu", format!("{}", mu_b)), ("Delta", format!("{}", del_b))],
        "q,eps_lat,E_minus_polar,E_plus_polar",
    );
    for i in 0..=400 {
        let q = -PI / ellb + 2.0 * PI / ellb * i as f64 / 400.0;
        let el = lattice::eps_lat(&[q, 0.0, 0.0], ellb, mass, 1.0, &act);
        bog_cur.row(&[f10(q), f10(el), f10(((el - mu_b).powi(2) + del_b * del_b).sqrt()), f10(((el + mu_b).powi(2) + del_b * del_b).sqrt())]);
    }
    bog_cur.write(&dd, "t2_B_bogoliubov_curves.csv");
    ck.add(
        "AS-3",
        "the Bogoliubov modes of the uniform polar superfluid dvec = (0, 0, Delta) (case B): Kch_3 h(q) = conj(h(-q)) Kch_3, so that for an eigenvector chi_pw of h(q) with the energy +eps or -eps the hole vector Kch_3 chi_pw is an eigenvector of conj(h(-q)) with the same energy; Ppair = Delta Kch_3^dag couples (chi_pw, 0) only to (0, Kch_3 chi_pw), and (cos theta_B chi_pw, sin theta_B Kch_3 chi_pw) with cos^2 theta_B = (1 + (+-eps - mu)/E)/2, sin^2 theta_B = (1 - (+-eps - mu)/E)/2, E = ((+-eps - mu)^2 + Delta^2)^(1/2), is an eigenvector of H_BdG(q) with the energy E, which is E^- for the positive branch +eps and E^+ for the negative branch -eps (50 random momenta, chemical potentials and gaps, m = 0.4; tolerance 1e-12); on the lattice (periodic chain of 32 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.6, Delta = 0.3) the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum are +-E^- and +-E^+ with eps_lat in place of eps, each four times (tolerance 1e-12)",
        dev_b1 < 1e-12 && dev_b1l < 1e-12,
        format!("largest deviations {} (continuum), {} (lattice)", sci(dev_b1), sci(dev_b1l)),
    );

    // ================================================================ Case B, AS-4: particle-hole (Rabi) oscillation
    // In the two-dimensional space spanned by (chi_pw, 0) and (0, Kch_3 chi_pw) the operator is
    // [[eps_lat - mu, Delta], [Delta, -(eps_lat - mu)]], with the eigenvalues +-E^-; a pure particle evolves as
    //   (cos(E x^0) - i ((eps_lat - mu)/E) sin(E x^0)) (chi_pw, 0) - i (Delta/E) sin(E x^0) (0, Kch_3 chi_pw).
    let nb = 16usize;
    let lat_b = Lattice { n: [nb, 1, 1], ell: ellb, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_b };
    let mut fl_b = Fields::trivial(nb);
    for s in 0..nb {
        fl_b.d[s] = [C64::ZERO, C64::ZERO, c(del_b, 0.0)];
    }
    let qb = chain_momentum(1, nb, ellb);
    let h4b = lattice::h_lat_plane(&[qb, 0.0, 0.0], ellb, mass, 1.0, &act);
    let (e4b, v4b) = eigh(&h4b);
    let eps_b = e4b[3];
    let mut chi8b = vec![C64::ZERO; 8];
    for a in 0..4 {
        chi8b[2 * a] = v4b.get(a, 3);
    }
    let kchi8b = k3.matvec(&chi8b);
    let xb = eps_b - mu_b;
    let e_b = (xb * xb + del_b * del_b).sqrt();
    let exact_b2 = |t: f64| -> Vec<C64> {
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
    let w0_b2 = exact_b2(0.0);
    // the 101 output times x^0 = 0, 0.2, ..., 20
    let touts_b2 = cvode::uniform_times(0.0, 0.2, 100);
    let mut dev_b2 = 0.0f64;
    let mut rabi = Csv::new(
        "the probability P_hole of the hole part of a particle injected into the uniform polar superfluid, from the CVODE integration of the lattice equations and from the exact solution (Delta/E^-)^2 sin^2(E^- x_LL^0) (Test 2, item 2, Figure 4(b))",
        &[("m", format!("{}", mass)), ("dx", format!("{}", ellb)), ("integrator", "CVODE (SUNDIALS 7.8.0) Adams-Moulton".into()), ("rtol", format!("{:e}", cv.rtol)), ("atol", format!("{:e}", cv.atol)), ("r_W", "1".into()), ("mu", format!("{}", mu_b)), ("Delta", format!("{}", del_b)), ("q", f10(qb)), ("eps_lat", f10(eps_b)), ("E^-", f10(e_b))],
        "x_LL^0,P_hole_cvode,P_hole_exact",
    );
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
    let st_b2 = cvode::evolve_lattice(&lat_b, &fl_b, &w0_b2, &touts_b2, &cv, |_i, t, w| {
        let ex = exact_b2(t);
        dev_b2 = dev_b2.max(vdiff(w, &ex) / vnorm(&ex));
        rabi.row(&[format!("{:.4}", t), f10(p_hole(w)), f10((del_b / e_b).powi(2) * (e_b * t).sin().powi(2))]);
    })
    .expect("CVODE integration of AS-4");
    rabi.write(&dd, "t2_B_rabi.csv");
    ck.add(
        "AS-4",
        "the particle-hole (Rabi) oscillation of a particle injected into the uniform polar superfluid (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with the pair field dvec = (0, 0, 0.3), the positive-energy lattice plane wave chi_pw of isospin up at the momentum 2 pi/(16 dx) evolves as (cos(E^- x_LL^0) - i ((eps_lat - mu)/E^-) sin(E^- x_LL^0)) (chi_pw, 0) - i (Delta/E^-) sin(E^- x_LL^0) (0, Kch_3 chi_pw), with E^- = ((eps_lat - mu)^2 + Delta^2)^(1/2): the probability of the hole is P_hole = (Delta/E^-)^2 sin^2(E^- x_LL^0); the CVODE integration of the lattice equations (Adams-Moulton formulas with local error control, rtol = 1e-12, atol = 1e-14) agrees with this exact solution at the 101 output times x_LL^0 = 0, 0.2, ..., 20 (relative deviation of the whole state, tolerance 1e-9; the numbers of steps and evaluations of CVODE are stated)",
        dev_b2 < 1e-9,
        format!("largest relative deviation {}; CVODE: {}; eps_lat = {:.6}, E^- = {:.6}, largest P_hole = (Delta/E^-)^2 = {:.6}", sci(dev_b2), st_b2.describe(), eps_b, e_b, (del_b / e_b).powi(2)),
    );

    // ================================================================ Case B, AS-5: the gap equation in closed form
    let gl = gauss_legendre(32);
    let lam = 1.0;
    let mut dev_b3 = 0.0f64;
    for &(mu, del) in &[(0.0, 0.3), (0.5, 0.2), (0.8, 0.05), (0.3, 1.0), (0.95, 0.01), (0.0, 2.0)] {
        let a = gap_integral_closed(lam, mu, del);
        let b = gap_integral_quad(lam, mu, del, 0.0, &gl);
        dev_b3 = dev_b3.max((a - b).abs() / b.abs());
    }
    // the critical coupling of massless charges at mu = 0: 1/G_crit = Lambda^2/(2 pi^2)
    let gcrit = 2.0 * PI * PI / (lam * lam);
    let gcrit_dev = (gap_integral_closed(lam, 0.0, 1e-9) * gcrit - 1.0).abs();
    let mut gapc = Csv::new(
        "the gap Delta of massless spinor charges as a function of the coupling, from the closed form of the gap equation by bisection (source: closed form) and checked by quadrature at chosen couplings (source: quadrature) (Test 2, item 2, Figure 4(c))",
        &[("Lambda", format!("{}", lam)), ("m", "0".into()), ("G_crit", f10(gcrit))],
        "mu,G_over_Gcrit_m0,Delta,source",
    );
    let mut dev_root = 0.0f64;
    // the number of the cases (mu, G/G_crit) of the second list that have a gap: 11 of the 12 (at mu = 0 and
    // G = 0.6 G_crit there is none)
    let mut nroots = 0usize;
    for &mu in &[0.0, 0.3, 0.6] {
        for i in 0..=280 {
            let r = 0.2 + 2.8 * i as f64 / 280.0;
            let del = solve_gap(r * gcrit, lam, mu);
            gapc.row(&[format!("{}", mu), format!("{:.5}", r), f10(del), "closed form".into()]);
        }
        for &r in &[0.6, 1.2, 1.8, 2.4] {
            let del = solve_gap(r * gcrit, lam, mu);
            if del > 0.0 {
                nroots += 1;
                let ig = gap_integral_quad(lam, mu, del, 0.0, &gl);
                dev_root = dev_root.max((ig * r * gcrit - 1.0).abs());
                gapc.row(&[format!("{}", mu), format!("{:.5}", r), f10(del), "quadrature".into()]);
            }
        }
    }
    gapc.write(&dd, "t2_B_gap.csv");
    let d_mu0 = solve_gap(1.5 * gcrit, lam, 0.0);
    // the weak-coupling (BCS) limit for mu > 0: Delta = 2 (Lambda^2 - mu^2)^(1/2) exp(-3/2) exp(-(pi^2/mu^2)(1/G - 1/G_crit)),
    // from the expansion of the closed form for Delta -> 0; the leading correction is of relative size
    // (Delta^2/(2 mu^2)) ln(1/Delta), and the check requires the deviation to stay below twice that size
    let mut bcs_ok = true;
    let mut bcs_txt = vec![];
    for &r in &[0.3, 0.4, 0.5] {
        let mu = 0.3;
        let g = r * gcrit;
        let del = solve_gap(g, lam, mu);
        let dbcs = 2.0 * (lam * lam - mu * mu).sqrt() * (-1.5f64).exp() * (-(PI * PI / (mu * mu)) * (1.0 / g - 1.0 / gcrit)).exp();
        let rel = del / dbcs - 1.0;
        let scale = del * del / (2.0 * mu * mu) * (1.0 / del).ln();
        bcs_ok &= rel.abs() < 2.0 * scale;
        bcs_txt.push(format!("{:.3e} ({:+.1e}, scale {:.1e})", del, rel, scale));
    }
    ck.add(
        "AS-5",
        "the gap equation (2.33) in closed form for massless spinor charges (case B): int_(q<Lambda) d^3q/(2 pi)^3 (1/E^+ + 1/E^-) with E^(+-) = ((q +- mu)^2 + Delta^2)^(1/2) equals (1/(4 pi^2)) [(Lambda - 3 mu) E^+(Lambda) + (Lambda + 3 mu) E^-(Lambda) + (2 mu^2 - Delta^2) (asinh((Lambda + mu)/Delta) + asinh((Lambda - mu)/Delta))] for six pairs (mu, Delta) (relative deviation from composite Gauss-Legendre quadrature with 32 nodes per panel, tolerance 1e-10); its limit Delta -> 0 at mu = 0 is Lambda^2/(2 pi^2), so that G_crit = 2 pi^2/Lambda^2 (tolerance 1e-8); the gaps Delta(G) found from the closed form by bisection satisfy G int(...) = 1 with the integral computed by quadrature (the 11 of the 12 cases mu = 0, 0.3, 0.6 with G/G_crit = 0.6, 1.2, 1.8, 2.4 that have a gap, tolerance 1e-9); and for mu > 0 and weak coupling (G < G_crit) the gap approaches the BCS form Delta = 2 (Lambda^2 - mu^2)^(1/2) exp(-3/2) exp(-(pi^2/mu^2) (1/G - 1/G_crit)), the relative deviation being smaller than twice the size (Delta^2/(2 mu^2)) ln(1/Delta) of the leading correction (mu = 0.3, G/G_crit = 0.3, 0.4, 0.5); Lambda = 1",
        dev_b3 < 1e-10 && gcrit_dev < 1e-8 && dev_root < 1e-9 && nroots == 11 && bcs_ok,
        format!("largest deviations {}, {}, {} ({} gaps); Delta at G = 1.5 G_crit, mu = 0: {:.6}; weak coupling at mu = 0.3, G/G_crit = 0.3, 0.4, 0.5: Delta = {}", sci(dev_b3), sci(gcrit_dev), sci(dev_root), nroots, d_mu0, bcs_txt.join(", ")),
    );

    // ================================================================ Case B, AS-6: the exact self-consistent uniform state on the lattice
    // Self-consistency (2.32) on a lattice with D directions of more than one site: the continuum mode
    // psi(x) = w(site)/dx^(D/2) for a unit vector w, so Pi_iota(x) = dx^(-D) sum_(E_qp > 0) psi_p^T Kch_iota conj(psi_h),
    // and d_iota = -(G/2) Pi_iota.  For the uniform polar state the mode sum is -2 Delta (1/N) sum_q (1/E^+ + 1/E^-)
    // (eps_lat in place of eps) at every site, so d = -(G/2) Pi reproduces Delta exactly when
    //   1/G = (1/(N dx)) sum_q (1/E^+ + 1/E^-)   (the lattice gap equation of a chain, D = 1).
    let nsc = 16usize;
    let lat_sc = Lattice { n: [nsc, 1, 1], ell: ellb, periodic: [true, false, false], wilson: 1.0, mass, mu: mu_b };
    let mut fl_sc = Fields::trivial(nsc);
    for s in 0..nsc {
        fl_sc.d[s] = [C64::ZERO, C64::ZERO, c(del_b, 0.0)];
    }
    let hsc = lat_sc.dense(&fl_sc, &mats);
    let (evsc, vsc) = eigh(&hsc);
    let mut modes = vec![];
    for i in 0..evsc.len() {
        if evsc[i] > 0.0 {
            modes.push((0..16 * nsc).map(|r| vsc.get(r, i)).collect::<Vec<C64>>());
        }
    }
    let kk = [kpair(1), kpair(2), kpair(3)];
    let pis = lattice::anomalous(nsc, &modes, &kk);
    let mut msum = 0.0;
    for kq in 0..nsc {
        let q = chain_momentum(kq, nsc, ellb);
        let el = lattice::eps_lat(&[q, 0.0, 0.0], ellb, mass, 1.0, &act);
        msum += 1.0 / ((el - mu_b).powi(2) + del_b * del_b).sqrt() + 1.0 / ((el + mu_b).powi(2) + del_b * del_b).sqrt();
    }
    let pi_expect = -2.0 * del_b * msum / nsc as f64;
    let g_lat = 1.0 / (msum / (nsc as f64 * ellb));
    let mut dev_sc = 0.0f64;
    let mut dev_dnew = 0.0f64;
    for s in 0..nsc {
        dev_sc = dev_sc.max(pis[s][0].abs()).max(pis[s][1].abs()).max((pis[s][2] - c(pi_expect, 0.0)).abs());
        let dnew = pis[s][2].scale(-0.5 * g_lat / ellb);
        dev_dnew = dev_dnew.max((dnew - c(del_b, 0.0)).abs());
    }
    ck.add(
        "AS-6",
        "the uniform polar state is an exact self-consistent solution of the lattice mean-field equations (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with dvec = (0, 0, 0.3), the sum over the 128 positive-energy eigenvectors of the lattice operator (unit vectors, with the particle part psi_p and the hole part psi_h) of psi_p^T Kch_iota conj(psi_h), computed at every site by the routine of the solvers, is 0 for iota = 1, 2 and -2 Delta (1/16) sum_q (1/E^+ + 1/E^-) (eps_lat in place of eps, the 16 allowed momenta) for iota = 3 (tolerance 1e-12); with Pi_iota = dx^(-1) times this sum (the continuum normalization of a chain) and the coupling G given by the lattice gap equation 1/G = (1/(16 dx)) sum_q (1/E^+ + 1/E^-), the self-consistency condition d_iota = -(G/2) Pi_iota gives back dvec = (0, 0, Delta) on every site (tolerance 1e-12): a fixed point that the self-consistent solvers must reproduce",
        dev_sc < 1e-12 && dev_dnew < 1e-12 && modes.len() == 8 * nsc,
        format!("{} positive modes; largest deviations {} (mode sum), {} (new dvec); G = {:.6}", modes.len(), sci(dev_sc), sci(dev_dnew), g_lat),
    );

    // ================================================================ Case C, AS-7: the domain wall (kink) of the pair field, continuum
    // With mu = 0 and dvec = (0, 0, Delta(x^1)) real: Ppair = Delta Kch_3^dag.  The matrix Kch_3^dag satisfies
    // (Kch_3^dag)^2 = -1, (Kch_3^dag)^dag = -Kch_3^dag, alpha^1 Kch_3^dag alpha^1 = -Kch_3^dag and
    // beta Kch_3^dag alpha^1 = -Kch_3^dag alpha^1 beta; alpha^1 and beta are real.  Then the mode
    //   psi_p = phi_K chi_K,  psi_h = i Kch_3^dag alpha^1 phi_K chi_K,  phi_K' = -Delta phi_K,  beta chi_K = +-chi_K,
    // solves the equations with the energy +-m.
    let km = k3.dagger();
    let a1 = lift4(&model::alpha(1));
    let bt = lift4(&model::beta());
    let id8 = CMat::identity(8);
    let id_dev = km.mul(&km).add(&id8).fro()
        + km.dagger().add(&km).fro()
        + a1.mul(&km).mul(&a1).add(&km).fro()
        + bt.mul(&km).mul(&a1).add(&km.mul(&a1).mul(&bt)).fro()
        + a1.sub(&a1.conj()).fro()
        + bt.sub(&bt.conj()).fro();
    let (dinf, ellk) = (1.0, 1.0);
    let mut dev_c1 = 0.0f64;
    for trial in 0..50 {
        let x = 4.0 * rng.unit();
        let del = dinf * (x / ellk).tanh();
        let phi = (x / ellk).cosh().powf(-dinf * ellk);
        let dphi = -del * phi; // phi_K' = -Delta phi_K
        // chi_K: a random vector projected on the eigenspace beta = +1 (even trials) or -1 (odd trials)
        let y: Vec<C64> = (0..8).map(|_| c(rng.unit(), rng.unit())).collect();
        let by = bt.matvec(&y);
        let sg = if trial % 2 == 0 { 1.0 } else { -1.0 };
        let chi: Vec<C64> = y.iter().zip(&by).map(|(a, b)| (*a + b.scale(sg)).scale(0.5)).collect();
        let eta = km.mul(&a1).scale(C64::I).matvec(&chi); // i Kch_3^dag alpha^1 chi_K
        let u: Vec<C64> = chi.iter().map(|z| z.scale(phi)).collect();
        let du: Vec<C64> = chi.iter().map(|z| z.scale(dphi)).collect();
        let v: Vec<C64> = eta.iter().map(|z| z.scale(phi)).collect();
        let dv: Vec<C64> = eta.iter().map(|z| z.scale(dphi)).collect();
        // particle row: -i alpha^1 psi_p' + m beta psi_p + Delta Kch_3^dag psi_h
        let t1 = a1.scale(c(0.0, -1.0)).matvec(&du);
        let t2 = bt.scale_r(mass).matvec(&u);
        let t3 = km.scale_r(del).matvec(&v);
        // hole row: Ppair^dag psi_p - conj(-i alpha^1 d/dx + m beta) psi_h = Delta Kch_3 psi_p - (i conj(alpha^1) psi_h' + m conj(beta) psi_h)
        let t4 = km.dagger().scale_r(del).matvec(&u);
        let t5 = a1.conj().scale(C64::I).matvec(&dv);
        let t6 = bt.conj().scale_r(mass).matvec(&v);
        for k in 0..8 {
            let rp = t1[k] + t2[k] + t3[k] - u[k].scale(sg * mass);
            let rh = t4[k] - t5[k] - t6[k] - v[k].scale(sg * mass);
            dev_c1 = dev_c1.max(rp.abs()).max(rh.abs());
        }
    }
    ck.add(
        "AS-7",
        "the bound states of a domain wall (kink) of the pair field (case C), in the continuum: the 8 x 8 matrices (Dirac (x) isospin) satisfy (Kch_3^dag)^2 = -1, (Kch_3^dag)^dag = -Kch_3^dag, alpha_LL^1 Kch_3^dag alpha_LL^1 = -Kch_3^dag and beta_LL Kch_3^dag alpha_LL^1 = -Kch_3^dag alpha_LL^1 beta_LL, and alpha_LL^1 and beta_LL are real (tolerance 1e-13); for the kink dvec = (0, 0, Delta(x_LL^1)) with Delta(x_LL^1) = Delta_inf tanh(x_LL^1/ell_K) (Delta_inf = 1, ell_K = 1) and mu = 0, the mode psi_p = phi_K chi_K, psi_h = i Kch_3^dag alpha_LL^1 phi_K chi_K with phi_K = cosh(x_LL^1/ell_K)^(-Delta_inf ell_K) (so that d phi_K/d x_LL^1 = -Delta phi_K) and a constant chi_K with beta_LL chi_K = +chi_K or -chi_K satisfies the sixteen component equations with the energy +m or -m at 50 random points (m = 0.4, random chi_K; tolerance 1e-12)",
        id_dev < 1e-13 && dev_c1 < 1e-12,
        format!("deviations of the identities {}, of the equations {}", sci(id_dev), sci(dev_c1)),
    );

    // ================================================================ Case C, AS-8: the square of the operator
    // With mu = 0, no gauge fields, dvec = (0, 0, Delta(x^1)) real, and the plane wave exp(i (q^2 x^2 + q^3 x^3)) in
    // the directions along the wall, the operator is H = [[h, Delta Kch_3^dag], [Delta Kch_3, -h~]] with
    // h = -i alpha^1 d/dx^1 + alpha^2 q^2 + alpha^3 q^3 + beta m and the hole operator h~ = conj(-i alpha^1 d/dx^1 - alpha^2 q^2 - alpha^3 q^3 + beta m).
    // Its square is H^2 = (-d^2/dx^2 + (q^2)^2 + (q^3)^2 + m^2 + Delta^2) 1_16 + Delta' Qmat_K with the constant Hermitian
    // matrix Qmat_K = [[0, -i alpha^1 Kch_3^dag], [-i alpha^1 Kch_3, 0]], Qmat_K^2 = 1_16, trace 0.  The check applies
    // H twice to a Gaussian times a random 16-component vector (exact derivatives) at random points.
    let a2 = lift4(&model::alpha(2));
    let a3 = lift4(&model::alpha(3));
    let k3m = k3.clone();
    let mut qk = CMat::zeros(16);
    qk.put(0, 8, &a1.mul(&km).scale(c(0.0, -1.0)));
    qk.put(8, 0, &a1.mul(&k3m).scale(c(0.0, -1.0)));
    let qk_dev = qk.sub(&qk.dagger()).fro() + qk.mul(&qk).sub(&CMat::identity(16)).fro() + qk.trace().abs();
    // the relation alpha^j Kch_3^dag = -Kch_3^dag conj(alpha^j) (j = 1, 2, 3), which removes the cross terms
    let mut rel_dev = 0.0f64;
    for j in 1..=3 {
        let aj = lift4(&model::alpha(j));
        rel_dev = rel_dev.max(aj.mul(&km).add(&km.mul(&aj.conj())).fro());
    }
    let mut dev_sq = 0.0f64;
    for _ in 0..50 {
        let dinf_r = 0.5 + 1.5 * rng.unit().abs();
        let ell_r = 0.5 + 1.5 * rng.unit().abs();
        let (q2, q3) = (rng.unit(), rng.unit());
        let x = 3.0 * rng.unit();
        let (x0, sw) = (rng.unit(), 0.5 + rng.unit().abs());
        let g = (-(x - x0).powi(2) / (2.0 * sw * sw)).exp();
        let g1 = -(x - x0) / (sw * sw) * g;
        let g2 = ((x - x0).powi(2) / sw.powi(4) - 1.0 / (sw * sw)) * g;
        let del = dinf_r * (x / ell_r).tanh();
        let ddel = dinf_r / ell_r / (x / ell_r).cosh().powi(2);
        let wv: Vec<C64> = (0..16).map(|_| c(rng.unit(), rng.unit())).collect();
        let wp: Vec<C64> = wv[0..8].to_vec();
        let wh: Vec<C64> = wv[8..16].to_vec();
        // the x-independent parts of the particle and hole operators
        let hp0 = a2.scale_r(q2).add(&a3.scale_r(q3)).add(&bt.scale_r(mass)); // alpha^2 q^2 + alpha^3 q^3 + beta m
        let hh0 = a2.conj().scale_r(-q2).add(&a3.conj().scale_r(-q3)).add(&bt.conj().scale_r(mass)); // conj(-alpha^2 q^2 - alpha^3 q^3 + beta m)
        let ma1 = a1.scale(c(0.0, -1.0)); // -i alpha^1
        let pa1 = a1.conj().scale(c(0.0, 1.0)); // conj(-i alpha^1) = i conj(alpha^1)
        // v = g w; (H v) and its derivative, from g, g', g'', Delta, Delta'
        let lin = |terms: &[(&CMat, &Vec<C64>, f64)]| -> Vec<C64> {
            let mut o = vec![C64::ZERO; 8];
            for (m, v, f) in terms {
                let mv = m.matvec(v);
                for k in 0..8 {
                    o[k] += mv[k].scale(*f);
                }
            }
            o
        };
        let hvp = lin(&[(&ma1, &wp, g1), (&hp0, &wp, g), (&km, &wh, del * g)]);
        let hvp1 = lin(&[(&ma1, &wp, g2), (&hp0, &wp, g1), (&km, &wh, ddel * g + del * g1)]);
        let k3d = k3m.clone();
        let hvh = lin(&[(&k3d, &wp, del * g), (&pa1, &wh, -g1), (&hh0, &wh, -g)]);
        let hvh1 = lin(&[(&k3d, &wp, ddel * g + del * g1), (&pa1, &wh, -g2), (&hh0, &wh, -g1)]);
        // H (H v)
        let h2p = lin(&[(&ma1, &hvp1, 1.0), (&hp0, &hvp, 1.0), (&km, &hvh, del)]);
        let h2h = lin(&[(&k3d, &hvp, del), (&pa1, &hvh1, -1.0), (&hh0, &hvh, -1.0)]);
        // the formula
        let scal = -g2 + (q2 * q2 + q3 * q3 + mass * mass + del * del) * g;
        let qw = qk.matvec(&wv);
        for k in 0..16 {
            let lhs = if k < 8 { h2p[k] } else { h2h[k - 8] };
            let rhs = wv[k].scale(scal) + qw[k].scale(ddel * g);
            dev_sq = dev_sq.max((lhs - rhs).abs());
        }
    }
    // the bound states of the two scalar operators -d^2/dx^2 + Delta^2 - Delta' (Qmat_K = -1) and
    // -d^2/dx^2 + Delta^2 + Delta' (Qmat_K = +1) for the kink: Poeschl-Teller wells with nu = lambda and
    // nu = lambda - 1 (lambda = Delta_inf ell_K), eigenvalues Delta_inf^2 - (nu - k)^2/ell_K^2 for k = 0, 1, ... < nu;
    // in the numbering k_K of the document: Delta_inf^2 - (Delta_inf - k_K/ell_K)^2, 0 <= k_K < lambda for Qmat_K = -1
    // and 1 <= k_K < lambda for Qmat_K = +1
    let mut dev_pt = 0.0f64;
    let mut count_pt = true;
    let mut pt_txt = vec![];
    for &(dinf_t, ell_t) in &[(1.0f64, 1.0f64), (2.5, 1.0)] {
        let lamk = dinf_t * ell_t;
        for &(sgn, nu) in &[(-1.0f64, lamk), (1.0f64, lamk - 1.0)] {
            let found = pt_bound_states(dinf_t, ell_t, sgn, 20.0, 4000);
            let mut expect = vec![];
            let mut k = 0.0;
            while k < nu - 1e-12 {
                expect.push(dinf_t * dinf_t - (nu - k).powi(2) / (ell_t * ell_t));
                k += 1.0;
            }
            if found.len() != expect.len() {
                count_pt = false;
            }
            for (a, b) in found.iter().zip(&expect) {
                dev_pt = dev_pt.max((a - b).abs());
            }
            pt_txt.push(format!("Delta_inf ell_K = {}, Qmat_K = {:+}: {} bound state{}", lamk, sgn, found.len(), if found.len() == 1 { "" } else { "s" }));
        }
    }
    ck.add(
        "AS-8",
        "the square of the operator and the complete spectrum of the bound states of a wall (case C): with mu = 0, no gauge fields, dvec = (0, 0, Delta(x_LL^1)) real and the plane wave exp(i (q^2 x_LL^2 + q^3 x_LL^3)) along the wall, H_DB^2 = (-d^2/d(x_LL^1)^2 + (q^2)^2 + (q^3)^2 + m^2 + Delta^2) 1_16 + (d Delta/d x_LL^1) Qmat_K with the constant matrix Qmat_K = [[0, -i alpha_LL^1 Kch_3^dag], [-i alpha_LL^1 Kch_3, 0]], which is Hermitian, squares to the unit matrix and has the trace 0 (tolerance 1e-13), because alpha_LL^j Kch_3^dag = -Kch_3^dag conj(alpha_LL^j) for j = 1, 2, 3 (tolerance 1e-13); the identity is checked by applying H_DB twice to a Gaussian times a random 16-component vector, with exact derivatives, for 50 random kinks Delta = Delta_inf tanh(x_LL^1/ell_K), points and momenta (m = 0.4; tolerance 1e-11); the bound states of the two scalar operators -d^2/d(x_LL^1)^2 + Delta^2 - d Delta/d x_LL^1 (the eigenvalue -1 of Qmat_K) and -d^2/d(x_LL^1)^2 + Delta^2 + d Delta/d x_LL^1 (the eigenvalue +1) are those of Poeschl-Teller wells, Delta_inf^2 - (Delta_inf - k_K/ell_K)^2 for the integers k_K with 0 <= k_K < Delta_inf ell_K (eigenvalue -1) and 1 <= k_K < Delta_inf ell_K (eigenvalue +1): finite differences with 4000 and 8001 points on (-20, 20), extrapolated, give exactly these bound states, and no others below the threshold Delta_inf^2, for Delta_inf ell_K = 1 and 2.5 (ell_K = 1; tolerance 1e-6); hence the bound states of the wall have the energies +-(m^2 + (q^2)^2 + (q^3)^2 + 2 k_K Delta_inf/ell_K - k_K^2/ell_K^2)^(1/2)",
        qk_dev < 1e-13 && rel_dev < 1e-13 && dev_sq < 1e-11 && dev_pt < 1e-6 && count_pt,
        format!("deviations: Qmat_K {}, relation {}, square {}, bound states {}; {}", sci(qk_dev), sci(rel_dev), sci(dev_sq), sci(dev_pt), pt_txt.join("; ")),
    );

    // ================================================================ Case C, AS-9: the kink on the lattice
    // A ring of length 24 with a kink at x^1 = 6 and an antikink at x^1 = 18: Delta(x) = tanh(x - 6) tanh(18 - x).
    let lring = 24.0;
    let (xk1, xk2) = (6.0, 18.0);
    let kink_profile = |x: f64| -> f64 { dinf * ((x - xk1) / ellk).tanh() * ((xk2 - x) / ellk).tanh() };
    // the distance of two points of the ring
    let dper = |x: f64, x0: f64| -> f64 {
        let d = (x - x0).rem_euclid(lring);
        d.min(lring - d)
    };
    let phik = |x: f64, x0: f64| -> f64 { (dper(x, x0) / ellk).cosh().powf(-dinf * ellk) };
    // (1) the dense spectrum at dx = 0.4 (60 sites, 960 x 960)
    let nk = 60usize;
    let ellx = lring / nk as f64;
    let lat_k = Lattice { n: [nk, 1, 1], ell: ellx, periodic: [true, false, false], wilson: 1.0, mass, mu: 0.0 };
    let mut fl_k = Fields::trivial(nk);
    for s in 0..nk {
        fl_k.d[s] = [C64::ZERO, C64::ZERO, c(kink_profile(s as f64 * ellx), 0.0)];
    }
    let hk = lat_k.dense(&fl_k, &mats);
    let (evk, veck) = eigh(&hk);
    let threshold = (mass * mass + dinf * dinf).sqrt();
    let inside: Vec<usize> = (0..evk.len()).filter(|&i| evk[i].abs() < 0.8 * threshold).collect();
    let nbound = inside.len();
    let nplus = inside.iter().filter(|&&i| evk[i] > 0.0).count();
    let mut spec = Csv::new(
        "the eigenvalues of the lattice operator of a ring of 60 sites (length 24) with a kink and an antikink of the pair field, Delta(x_LL^1) = tanh(x_LL^1 - 6) tanh(18 - x_LL^1) (Test 2, item 2, Figure 5(c))",
        &[("m", format!("{}", mass)), ("dx", f10(ellx)), ("mu", "0".into()), ("Delta_inf", format!("{}", dinf)), ("ell_K", format!("{}", ellk)), ("r_W", "1".into())],
        "index,E_qp",
    );
    for (i, e) in evk.iter().enumerate() {
        spec.row(&[format!("{}", i), f10(*e)]);
    }
    spec.write(&dd, "t2_C_kink_spectrum.csv");
    // the summed density of the eight eigenvectors near +m against the analytic density of the bound states:
    // four states per wall with the density 2 phi_K^2 |chi_K|^2 each, so the summed density is proportional
    // to phi_K(kink)^2 + phi_K(antikink)^2
    let plus: Vec<usize> = inside.iter().cloned().filter(|&i| evk[i] > 0.0).collect();
    let mut dens = vec![0.0; nk];
    for &i in &plus {
        for s in 0..nk {
            for k in 0..16 {
                dens[s] += veck.get(16 * s + k, i).norm_sqr();
            }
        }
    }
    let dens_an = |x: f64| -> f64 { phik(x, xk1).powi(2) + phik(x, xk2).powi(2) };
    let norm_an: f64 = (0..nk).map(|s| dens_an(s as f64 * ellx)).sum();
    let norm_lat: f64 = dens.iter().sum();
    let mut prof = Csv::new(
        "the pair-field profile of the ring with a kink and an antikink, the summed density of the eight lattice eigenvectors with the energies near +m, and the analytic density of the bound states, both normalized to the integral 1 (Test 2, item 2, Figure 5(a))",
        &[("m", format!("{}", mass)), ("dx", f10(ellx)), ("Delta_inf", format!("{}", dinf)), ("ell_K", format!("{}", ellk))],
        "x_LL^1,Delta,density_lattice,density_exact",
    );
    let mut bc = 0.0;
    for s in 0..nk {
        let x = s as f64 * ellx;
        let a = dens[s] / norm_lat;
        let b = dens_an(x) / norm_an;
        bc += (a * b).sqrt();
        prof.row(&[f10(x), f10(kink_profile(x)), f10(a / ellx), f10(b / ellx)]);
    }
    prof.write(&dd, "t2_C_kink_profile.csv");
    // (2) the Rayleigh quotient of the analytic bound state of the kink at x^1 = 6 (chi_K = (e_0 + e_4)/sqrt 2,
    // beta chi_K = chi_K, energy +m) on rings with dx = 0.4, 0.2, 0.1, 0.05.  Exact lattice result (document,
    // Section 3.4): E_R = m + (r_W/(2 dx)) sum_s (phi_K(s+1) - phi_K(s))^2 / sum_s phi_K(s)^2, which tends to
    // m + r_W dx/6 for phi_K = 1/cosh (the first-order effect of the Wilson term).
    let mut chi = vec![C64::ZERO; 8];
    chi[0] = c(1.0 / 2f64.sqrt(), 0.0);
    chi[4] = c(1.0 / 2f64.sqrt(), 0.0);
    let beta_dev = vnorm(&bt.matvec(&chi).iter().zip(&chi).map(|(a, b)| *a - *b).collect::<Vec<C64>>());
    let eta = km.mul(&a1).scale(C64::I).matvec(&chi);
    let mut conv = Csv::new(
        "the Rayleigh quotient E_R of the analytic bound state (energy +m) of the kink on rings of length 24 with the lattice spacings 0.4, 0.2, 0.1, 0.05, and the relative residual of the state (Test 2, item 2, Figure 5(d))",
        &[("m", format!("{}", mass)), ("Delta_inf", format!("{}", dinf)), ("ell_K", format!("{}", ellk)), ("r_W", "1".into())],
        "dx,E_R,relative_residual",
    );
    let mut eq = vec![];
    let mut dev_formula = 0.0f64;
    // the norm of the part of the normalized analytic state of the ring of 60 sites outside the space of the eight
    // eigenvectors near +m (the square root of 1 minus the sum of its squared scalar products with them)
    let mut outside60 = f64::NAN;
    for &ns in &[60usize, 120, 240, 480] {
        let ell = lring / ns as f64;
        let lat = Lattice { n: [ns, 1, 1], ell, periodic: [true, false, false], wilson: 1.0, mass, mu: 0.0 };
        let mut fl = Fields::trivial(ns);
        for s in 0..ns {
            fl.d[s] = [C64::ZERO, C64::ZERO, c(kink_profile(s as f64 * ell), 0.0)];
        }
        let phis: Vec<f64> = (0..ns).map(|s| phik(s as f64 * ell, xk1)).collect();
        let mut w = vec![C64::ZERO; 16 * ns];
        for s in 0..ns {
            for k in 0..8 {
                w[16 * s + k] = chi[k].scale(phis[s]);
                w[16 * s + 8 + k] = eta[k].scale(phis[s]);
            }
        }
        let mut hw = vec![C64::ZERO; 16 * ns];
        lat.apply(&fl, &mats, &w, &mut hw);
        let num: C64 = w.iter().zip(&hw).fold(C64::ZERO, |acc, (a, b)| acc + a.conj() * *b);
        let den: f64 = w.iter().map(|z| z.norm_sqr()).sum();
        let er = num.re / den;
        let res: Vec<C64> = hw.iter().zip(&w).map(|(h, x)| *h - x.scale(er)).collect();
        let rres = vnorm(&res) / den.sqrt();
        if ns == nk {
            let inside_part: f64 = plus
                .iter()
                .map(|&i| (0..16 * ns).fold(C64::ZERO, |acc, row| acc + veck.get(row, i).conj() * w[row]).norm_sqr())
                .sum::<f64>()
                / den;
            outside60 = (1.0 - inside_part).max(0.0).sqrt();
        }
        // the exact lattice formula
        let s1: f64 = (0..ns).map(|s| (phis[(s + 1) % ns] - phis[s]).powi(2)).sum();
        let s0: f64 = phis.iter().map(|p| p * p).sum();
        let er_formula = mass + 1.0 / (2.0 * ell) * s1 / s0;
        dev_formula = dev_formula.max((er - er_formula).abs());
        eq.push((ell, er, rres));
        conv.row(&[f10(ell), f10(er), format!("{:.6e}", rres)]);
    }
    conv.write(&dd, "t2_C_kink_convergence.csv");
    // the shift E_R - m divided by r_W dx tends to 1/6, with an error of second order in dx
    let ratios: Vec<f64> = eq.iter().map(|x| (x.1 - mass) / x.0).collect();
    let errs: Vec<f64> = ratios.iter().map(|r| 1.0 / 6.0 - r).collect();
    let first_order = (ratios[3] - 1.0 / 6.0).abs() < 0.01 / 6.0 && errs.windows(2).all(|p| p[0] / p[1] > 3.5 && p[0] / p[1] < 4.5);
    let res_dec = eq.windows(2).all(|p| p[1].2 < 0.6 * p[0].2);
    // The bound states of the dense spectrum (dx = 0.4) and the analytic state are compared at second order in the
    // relative residual r of the analytic state.  Let delta be the distance from its Rayleigh quotient E_R to the
    // nearest eigenvalue outside the cluster of the eight positive bound states, and let E_R lie between the nearest
    // eigenvalue below the cluster and the nearest one above it.  Two theorems hold:
    //   (Kato and Temple)  one eigenvalue of the cluster lies at most r^2/delta above E_R, and one at most
    //                      r^2/delta below.  All 16 moduli |E| of the bound states therefore lie within
    //                      r^2/delta + spread of E_R, where spread is the largest minus the smallest of the 16 moduli.
    //   (Davis and Kahan)  the part of the normalized analytic state outside the space of the eigenvectors of the
    //                      cluster has at most the norm r/delta (on that part the operator minus E_R is at least
    //                      delta in magnitude, and the whole image of the state has the norm r).
    // The check verifies both.  For the summed density of the eight eigenvectors it REQUIRES the agreement at second
    // order, 1 - (Bhattacharyya coefficient) < (1/2) (r/delta)^2, the leading term of 1 - cos of the angle whose
    // sine is r/delta; this is a requirement on the scale of the second theorem, not a theorem (the analytic state
    // of the check is one state of one wall).
    let er04 = eq[0].1;
    let r04 = eq[0].2;
    let worst_bound = inside.iter().map(|&i| (evk[i].abs() - er04).abs()).fold(0.0, f64::max);
    let delta04 = (0..evk.len()).filter(|i| !plus.contains(i)).map(|i| (evk[i] - er04).abs()).fold(f64::INFINITY, f64::min);
    // the spread of the 16 moduli, and the position of E_R between the neighbours of the cluster (the eigenvalues
    // are in ascending order, and the cluster is a run of them)
    let moduli: Vec<f64> = inside.iter().map(|&i| evk[i].abs()).collect();
    let spread = moduli.iter().cloned().fold(0.0, f64::max) - moduli.iter().cloned().fold(f64::INFINITY, f64::min);
    let (first, last) = (plus[0], plus[plus.len() - 1]);
    let between = last - first + 1 == plus.len() && first > 0 && last + 1 < evk.len() && evk[first - 1] < er04 && er04 < evk[last + 1];
    let tol_e = r04 * r04 / delta04 + spread;
    let tol_out = r04 / delta04;
    let tol_bc = 0.5 * (r04 / delta04).powi(2);
    ck.add(
        "AS-9",
        "the bound states of the domain wall on the lattice (case C): on a ring of length 24 (60 sites, dx = 0.4, r_W = 1, m = 0.4, mu = 0) with the kink-antikink profile Delta(x_LL^1) = tanh(x_LL^1 - 6) tanh(18 - x_LL^1), the lattice operator has exactly 16 eigenvalues inside 0.8 times the continuum threshold (m^2 + Delta_inf^2)^(1/2), eight positive and eight negative (four per wall and sign); with the relative residual r of the analytic bound state at dx = 0.4 and the distance delta from its Rayleigh quotient E_R to the nearest eigenvalue outside the cluster of the eight positive bound states, which lies between the neighbours of the cluster: (1) the moduli of the 16 eigenvalues lie within r^2/delta + spread of E_R, where spread is the largest minus the smallest of the 16 moduli (the theorem of Kato and Temple: one eigenvalue of the cluster lies at most r^2/delta above E_R and one at most r^2/delta below); (2) the part of the normalized analytic state outside the space of the eight eigenvectors near +m has at most the norm r/delta (the theorem of Davis and Kahan); (3) the summed density of the eight states near +m agrees with the analytic density of the bound states of the two walls at second order in r, 1 - (Bhattacharyya coefficient of the two normalized densities) < (1/2) (r/delta)^2 (a requirement of the check on the scale of (2), not a theorem; the Bhattacharyya coefficient is the sum over the sites of the square root of the product of the two densities, each normalized to the sum 1); for the analytic bound state of the kink (chi_K with the components 1 and 5 equal to 1/sqrt 2 and the others zero, beta_LL chi_K = chi_K) on rings with dx = 0.4, 0.2, 0.1, 0.05 the Rayleigh quotient E_R equals the exact lattice formula m + (r_W/(2 dx)) sum (phi_K(s+1) - phi_K(s))^2 / sum phi_K(s)^2 (tolerance 1e-12), (E_R - m)/(r_W dx) tends to 1/6 (within 1 per cent at dx = 0.05, the difference decreasing by a factor between 3.5 and 4.5 at each halving of dx: the Wilson term shifts the bound state at first order in dx, by r_W dx/6, with a correction of order dx^3), and the relative residual decreases at least by the factor 0.6 at each halving of dx",
        nbound == 16 && nplus == 8 && between && worst_bound <= tol_e && outside60 <= tol_out && 1.0 - bc < tol_bc && dev_formula < 1e-12 && first_order && res_dec && beta_dev < 1e-15,
        format!(
            "{} bound states ({} positive); largest | |E| - E_R | = {:.5} (bound r^2/delta + spread = {:.5}, delta = {:.4}, spread = {}); part of the analytic state outside the eight eigenvectors {:.4} (bound r/delta = {:.4}); 1 - Bhattacharyya coefficient = {:.2e} (required below {:.2e}); E_R = {:.6}, {:.6}, {:.6}, {:.6}; (E_R - m)/dx = {:.5}, {:.5}, {:.5}, {:.5}; residuals {:.2e}, {:.2e}, {:.2e}, {:.2e}; deviation from the lattice formula {}",
            nbound, nplus, worst_bound, tol_e, delta04, sci(spread), outside60, tol_out, 1.0 - bc, tol_bc, eq[0].1, eq[1].1, eq[2].1, eq[3].1, ratios[0], ratios[1], ratios[2], ratios[3], eq[0].2, eq[1].2, eq[2].2, eq[3].2, sci(dev_formula)
        ),
    );

    // ================================================================ Case D, AS-10: Landau levels
    // One isospin component (Q_LL = +1/2 or -1/2) in the constant magnetic fields FF_LL_12 (U(1)_F) and Fv_LL^3_12
    // (third isospin direction), Landau gauge AF_LL_2 = FF_LL_12 x^1, Av_LL^3_2 = Fv_LL^3_12 x^1: the component
    // feels Acal_2 = B x^1 with B = B_up = g_F FF_LL_12 + g_v Fv_LL^3_12/2 or B = B_down = g_F FF_LL_12 - g_v Fv_LL^3_12/2.
    // At p_2 = p_3 = 0: h = alpha^1 p_1 + alpha^2 (-B x^1) + beta m.  Oscillator basis of the frequency |B|:
    // x^1 = (a + a^dag)/sqrt(2|B|), p_1 = i sqrt(|B|/2) (a^dag - a).  Exact energies: +m and -m once each,
    // +-(m^2 + 2 n_L |B|)^(1/2) twice each, n_L = 1, 2, ...  The exact eigenstates of the level n_L contain only the
    // oscillator states n_L - 1 and n_L, so the basis of the states 0 to 119 represents the levels 0 to 119 exactly.
    // The oscillator state 119 with the antiparallel spin has lost its partner (the state 120): the truncated matrix
    // has, instead of the level 120, two states with the energies +m and -m that live on the oscillator state 119.
    // These two states are artefacts of the truncation.  The check discards every eigenvector with a weight above
    // 1e-8 on the ten highest oscillator states: for each isospin component the two artefacts and the 40 states of
    // the levels 110 to 119, which it does not use; it counts both kinds.
    let (ff12, fv12) = (1.2, 1.0);
    let nosc = 120usize;
    let alpha1 = model::alpha(1);
    let alpha2 = model::alpha(2);
    let beta4 = model::beta();
    let mut landau = Csv::new(
        "the Landau levels of one isospin component in the constant magnetic fields FF_LL_12 of U(1)_F and Fv_LL^3_12 of the third isospin direction, from the formula E_qp = +-(m^2 + 2 n_L |B_up|)^(1/2) for Q_LL = +1/2 and +-(m^2 + 2 n_L |B_down|)^(1/2) for Q_LL = -1/2 (source: formula) and from the diagonalization in an oscillator basis of 120 states (source: numerical) (Test 2, item 2, Figure 6)",
        &[("m", format!("{}", mass)), ("g_v", format!("{}", gv)), ("g_F", format!("{}", gf)), ("FF_LL_12", format!("{}", ff12)), ("Fv_LL^3_12", format!("{}", fv12)),
          ("B_up", f10(gf * ff12 + 0.5 * gv * fv12)), ("B_down", f10(gf * ff12 - 0.5 * gv * fv12))],
        "Q_LL,n_L,E_qp,source",
    );
    let mut dev_d = 0.0f64;
    let mut count_ok = true;
    let mut nspur = 0usize;
    // of the discarded states: those at a level 110 to 119, and the artefacts with the energy +m or -m
    let mut nspur_levels = 0usize;
    let mut nspur_mass = 0usize;
    for &qq in &[0.5f64, -0.5] {
        let b = gf * ff12 + qq * gv * fv12;
        let ab = b.abs();
        let mut xm = CMat::zeros(nosc);
        let mut pm = CMat::zeros(nosc);
        for n in 0..nosc - 1 {
            let s = ((n + 1) as f64).sqrt();
            // (a)_(n, n+1) = sqrt(n+1), (a^dag)_(n+1, n) = sqrt(n+1)
            xm.set(n, n + 1, c(s / (2.0 * ab).sqrt(), 0.0));
            xm.set(n + 1, n, c(s / (2.0 * ab).sqrt(), 0.0));
            pm.set(n + 1, n, c(0.0, (ab / 2.0).sqrt() * s));
            pm.set(n, n + 1, c(0.0, -(ab / 2.0).sqrt() * s));
        }
        let h = alpha1.kron(&pm).add(&alpha2.kron(&xm.scale_r(-b))).add(&beta4.kron(&CMat::identity(nosc)).scale_r(mass));
        let (ev, vecs) = eigh(&h);
        let mut kept: Vec<f64> = vec![];
        for i in 0..ev.len() {
            let mut wtop = 0.0;
            for d4 in 0..4 {
                for n in nosc - 10..nosc {
                    wtop += vecs.get(d4 * nosc + n, i).norm_sqr();
                }
            }
            if wtop < 1e-8 {
                kept.push(ev[i]);
            } else {
                nspur += 1;
                let e = ev[i].abs();
                if (nosc - 10..nosc).any(|n| (e - (mass * mass + 2.0 * n as f64 * ab).sqrt()).abs() < 1e-8) {
                    nspur_levels += 1;
                }
                if (e - mass).abs() < 1e-8 {
                    nspur_mass += 1;
                }
            }
        }
        for sign in [1.0f64, -1.0] {
            let mine = sorted(kept.iter().cloned().filter(|e| e * sign > 0.0).map(|e| e.abs()).collect());
            let mut expect: Vec<f64> = vec![mass];
            for n in 1..=10 {
                let e = (mass * mass + 2.0 * n as f64 * ab).sqrt();
                expect.push(e);
                expect.push(e);
            }
            if mine.len() <= expect.len() {
                count_ok = false;
                continue;
            }
            for i in 0..expect.len() {
                dev_d = dev_d.max((mine[i] - expect[i]).abs());
            }
            // the next kept energy lies above the level 10
            if mine[expect.len()] < expect[expect.len() - 1] + 1e-9 {
                count_ok = false;
            }
            for (i, e) in mine.iter().take(expect.len()).enumerate() {
                let level = if i == 0 { 0 } else { (i + 1) / 2 };
                landau.row(&[format!("{}", qq), format!("{}", level), f10(sign * e), "numerical".into()]);
            }
            for n in 0..=10 {
                let e = (mass * mass + 2.0 * n as f64 * ab).sqrt();
                landau.row(&[format!("{}", qq), format!("{}", n), f10(sign * e), "formula".into()]);
            }
        }
    }
    landau.write(&dd, "t2_D_landau.csv");
    ck.add(
        "AS-10",
        "Landau levels (case D): for one isospin component (Q_LL = +1/2 or -1/2) in the constant magnetic fields FF_LL_12 = 1.2 of U(1)_F and Fv_LL^3_12 = 1.0 of the third isospin direction (Landau gauge AF_LL_2 = FF_LL_12 x_LL^1, Av_LL^3_2 = Fv_LL^3_12 x_LL^1; g_v = 0.7, g_F = 0.45), the component feels the magnetic field B_up = g_F FF_LL_12 + g_v Fv_LL^3_12/2 or B_down = g_F FF_LL_12 - g_v Fv_LL^3_12/2; the Dirac Hamiltonian alpha_LL^1 p_1 + alpha_LL^2 (p_2 - B_up x_LL^1) + beta_LL m at p_2 = p_3 = 0 (and the same with B_down), with the momentum operators p_j = -i d_j, diagonalized in a basis of 120 oscillator states of the frequency |B_up| (or |B_down|) (the states with a weight above 1e-8 on the ten highest oscillator states are discarded: for each isospin component they are the 40 states of the levels 110 to 119, which the check does not use, and the two artefacts of the truncation, which have the energies +m and -m and live on the highest oscillator state), has the energies +m and -m once each and +-(m^2 + 2 n_L |B_up|)^(1/2) (or with |B_down|) twice each for n_L = 1 to 10, and no other energy below the level 10 (m = 0.4; tolerance 1e-10)",
        dev_d < 1e-10 && count_ok && nspur == 84 && nspur_levels == 80 && nspur_mass == 4,
        format!("largest deviation {}; B_up = {:.3}, B_down = {:.3}; {} discarded states, of them {} at the levels 110 to 119 and {} artefacts at +-m", sci(dev_d), gf * ff12 + 0.5 * gv * fv12, gf * ff12 - 0.5 * gv * fv12, nspur, nspur_levels, nspur_mass),
    );

    // ================================================================ output
    let ok = ck.write(&dd.join("t2_analytic_checks.txt"));
    println!("wrote test2/data/t2_analytic_checks.txt and the data files t2_A_*, t2_B_*, t2_C_*, t2_D_*");
    if !ok {
        std::process::exit(1);
    }
}
