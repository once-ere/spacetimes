//! Command `potential`: the effective potential of the superfluid, its minima, and
//! the spontaneous breaking of the gauge group U(2)_U2 -> SU(2)_U2.
//!
//! Units: the momentum cutoff Lambda_U2 = 1; every mass, momentum, energy and
//! chemical potential is in units of Lambda_U2, the couplings G^S_U2, G^P_U2 in units
//! of Lambda_U2^-6, the effective potential in units of Lambda_U2^8.
//!
//! The mean-field (one-loop in the fermions, tree level in the composite fields)
//! effective potential of the Dirac doublet, per unit volume, at zero temperature,
//! with the spin-zero pair field Delta (real; the potential depends on |Delta| only),
//! the effective mass M and the chemical potential mu, is
//!
//!   Omega(M, Delta) = (M - m)^2/(4 G^S) + Delta^2/G^P - 8 a7 Int_0^1 dq q^6 [E^+(q) + E^-(q)],
//!   E^+- = sqrt((eps +- mu)^2 + Delta^2),  eps = sqrt(q^2 + M^2),  a7 = A_7/(2 pi)^7 = 1/(120 pi^4),
//!
//! (regularization R7: a sharp cutoff on the length q of the seven-dimensional spatial
//! momentum).  The program evaluates it relative to the normal state of mass m at the
//! same mu (that subtraction does not change the minima).  The left-handed (chiral)
//! doublet has no mass and half the states: its potential is Delta^2/G^P - 4 a7 Int ...
//!
//! In the vacuum (mu = 0) the potential depends on M^2 + Delta^2 only through the
//! fermion term; regularization R8 (a sharp cutoff on the length of the Euclidean
//! 8-momentum, Lorentz invariant) gives the closed form
//!
//!   Omega_R8(M, Delta) = (M - m)^2/(4 G^S) + Delta^2/G^P - 8 b8 F(M^2 + Delta^2),
//!   F(s) = Int_0^1 du u^3 ln(u + s),  b8 = A_8/(2 pi)^8 = 1/(768 pi^4).
//!
//! Critical couplings (the coefficient of Delta^2 changes sign):
//!   R7: G^P_crit = 3/(4 a7) = 90 pi^4,   R8: G^P_crit = 3/(8 b8) = 288 pi^4,
//! and G^S_crit = G^P_crit/4 in both.  lambda^P = G^P/G^P_crit, lambda^S = G^S/G^S_crit.

use crate::bdg::e_cplus;
use crate::checks::Checks;
use crate::integrate::adaptive;
use spacetimes::roots::brent;

pub const PI: f64 = std::f64::consts::PI;
const TOL: f64 = 1e-15;

/// A_7/(2 pi)^7 with A_7 = 16 pi^3/15 the area of the unit sphere in 7 dimensions.
pub fn a7() -> f64 {
    1.0 / (120.0 * PI.powi(4))
}
/// A_8/(2 pi)^8 with A_8 = pi^4/3 the area of the unit sphere in 8 dimensions.
pub fn b8() -> f64 {
    1.0 / (768.0 * PI.powi(4))
}
/// Critical pair coupling of regularization R7 (units Lambda^-6): 90 pi^4.
pub fn gcrit7() -> f64 {
    3.0 / (4.0 * a7())
}
/// Critical pair coupling of regularization R8: 288 pi^4.
pub fn gcrit8() -> f64 {
    3.0 / (8.0 * b8())
}

/// The parameters of one state: bare mass m, couplings G^S (0: absent) and G^P,
/// chemical potential mu, and the content (Dirac doublet or left-handed doublet).
#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub m: f64,
    pub gs: f64,
    pub gp: f64,
    pub mu: f64,
    pub chiral: bool,
}

impl Model {
    /// The Dirac doublet with pair coupling lambda^P G^P_crit (R7) and no scalar coupling.
    pub fn pairing(m: f64, lp: f64, mu: f64) -> Model {
        Model { m, gs: 0.0, gp: lp * gcrit7(), mu, chiral: false }
    }
    /// Number of states per flavour-pair weight: 8 for the Dirac doublet, 4 for the chiral one.
    pub fn deg(&self) -> f64 {
        if self.chiral {
            4.0
        } else {
            8.0
        }
    }
}

/// Int_0^1 F(q, eps, xi, E^-) dq, eps = sqrt(q^2 + M^2), xi = eps - mu, E^- = sqrt(xi^2 + Delta^2).
///
/// If the Fermi momentum q_F = sqrt(mu^2 - M^2) lies inside (0, 1) and Delta > 0, the
/// window |xi| < w around the Fermi surface is integrated in the variable t with
/// xi = Delta sinh t (then dxi/sqrt(xi^2 + Delta^2) = dt, and q, eps are computed from
/// xi, so that xi itself carries no rounding error): the integrands, which vary on the
/// scale Delta near the Fermi surface, become smooth functions of t for every Delta > 0,
/// however small.  Outside the window, and in the normal state, q is the variable.
pub fn radial<F: Fn(f64, f64, f64, f64) -> f64>(f: &F, mass: f64, mu: f64, d: f64) -> f64 {
    let direct = |q: f64| {
        let eps = (q * q + mass * mass).sqrt();
        let xi = eps - mu;
        f(q, eps, xi, xi.hypot(d))
    };
    let e1 = (1.0 + mass * mass).sqrt();
    if mu <= mass || mu >= e1 {
        return adaptive(&direct, 0.0, 1.0, TOL);
    }
    let qf = (mu * mu - mass * mass).sqrt();
    if d <= 0.0 {
        return adaptive(&direct, 0.0, qf, TOL) + adaptive(&direct, qf, 1.0, TOL);
    }
    let w = 0.5 * (mu - mass).min(e1 - mu);
    let q_lo = ((mu - w).powi(2) - mass * mass).max(0.0).sqrt();
    let q_hi = ((mu + w).powi(2) - mass * mass).sqrt();
    let t0 = (w / d).asinh();
    let g = |t: f64| {
        // E^- = sqrt(xi^2 + Delta^2) = Delta cosh t, computed without squaring Delta
        let xi = d * t.sinh();
        let em = d * t.cosh();
        let eps = mu + xi;
        let q = (eps * eps - mass * mass).sqrt();
        f(q, eps, xi, em) * (eps / q) * em
    };
    adaptive(&direct, 0.0, q_lo, TOL) + adaptive(&g, -t0, 0.0, TOL) + adaptive(&g, 0.0, t0, TOL) + adaptive(&direct, q_hi, 1.0, TOL)
}

/// E^+ = sqrt((eps + mu)^2 + Delta^2) (hypot: no underflow for tiny Delta).
#[inline]
fn eplus(eps: f64, mu: f64, d: f64) -> f64 {
    (eps + mu).hypot(d)
}

/// Omega (R7) relative to the normal state of mass m at the same mu.
pub fn omega7(md: &Model, mass: f64, d: f64) -> f64 {
    let mass = if md.chiral { 0.0 } else { mass };
    let mref = if md.chiral { 0.0 } else { md.m };
    let u = d * d / md.gp + if md.gs > 0.0 && !md.chiral { (mass - md.m).powi(2) / (4.0 * md.gs) } else { 0.0 };
    let mu = md.mu;
    let f = |q: f64, eps: f64, _xi: f64, em: f64| {
        let ep = eplus(eps, mu, d);
        q.powi(6) * (ep + em)
    };
    let fr = |q: f64, eps: f64, xi: f64, _em: f64| q.powi(6) * ((eps + mu).abs() + xi.abs());
    u - md.deg() * a7() * (radial(&f, mass, mu, d) - radial(&fr, mref, mu, 0.0))
}

/// The fermion part -deg a7 Int q^6 (E^+ + E^-) itself (not subtracted), at chemical potential mu.
pub fn fermion7(md: &Model, mass: f64, d: f64, mu: f64) -> f64 {
    let f = |q: f64, eps: f64, _xi: f64, em: f64| {
        let ep = eplus(eps, mu, d);
        q.powi(6) * (ep + em)
    };
    -md.deg() * a7() * radial(&f, mass, mu, d)
}

/// dOmega/dDelta divided by 2 Delta: 1/G^P - (deg/2) a7 Int q^6 (1/E^+ + 1/E^-).
pub fn gap_delta(md: &Model, mass: f64, d: f64) -> f64 {
    let mass = if md.chiral { 0.0 } else { mass };
    let mu = md.mu;
    let f = |q: f64, eps: f64, _xi: f64, em: f64| {
        let ep = eplus(eps, mu, d);
        q.powi(6) * (1.0 / ep + 1.0 / em)
    };
    1.0 / md.gp - 0.5 * md.deg() * a7() * radial(&f, mass, mu, d)
}

/// dOmega/dM: (M - m)/(2 G^S) - deg a7 Int q^6 (M/eps) [(eps + mu)/E^+ + (eps - mu)/E^-].
pub fn gap_mass(md: &Model, mass: f64, d: f64) -> f64 {
    let mu = md.mu;
    let f = |q: f64, eps: f64, xi: f64, em: f64| {
        let ep = eplus(eps, mu, d);
        q.powi(6) * (mass / eps) * ((eps + mu) / ep + xi / em)
    };
    (mass - md.m) / (2.0 * md.gs) - md.deg() * a7() * radial(&f, mass, mu, d)
}

/// Number density n = -dOmega/dmu = deg a7 Int q^6 [(eps + mu)/E^+ - (eps - mu)/E^-].
pub fn density(md: &Model, mass: f64, d: f64) -> f64 {
    let mass = if md.chiral { 0.0 } else { mass };
    let mu = md.mu;
    let f = |q: f64, eps: f64, xi: f64, em: f64| {
        let ep = eplus(eps, mu, d);
        q.powi(6) * ((eps + mu) / ep - xi / em)
    };
    md.deg() * a7() * radial(&f, mass, mu, d)
}

/// Time-like stiffness at fixed (M, Delta): f_t^2 = 2 a7 Int q^6 Delta^2 (1/E+^3 + 1/E-^3) (Dirac doublet).
pub fn ft2_fixed(mass: f64, mu: f64, d: f64) -> f64 {
    let f = |q: f64, eps: f64, _xi: f64, em: f64| {
        let ep = eplus(eps, mu, d);
        q.powi(6) * d * d * (1.0 / ep.powi(3) + 1.0 / em.powi(3))
    };
    2.0 * a7() * radial(&f, mass, mu, d)
}

/// The pair field that minimizes Omega at fixed M (the unique root of the gap
/// equation, or 0).  The root is found in the variable ln(Delta).
pub fn solve_delta(md: &Model, mass: f64) -> f64 {
    let lo: f64 = 1e-300;
    if gap_delta(md, mass, lo) >= 0.0 {
        return 0.0;
    }
    let mut hi: f64 = 1.0;
    while gap_delta(md, mass, hi) < 0.0 && hi < 1e3 {
        hi *= 2.0;
    }
    match brent(|t: f64| gap_delta(md, mass, t.exp()), lo.ln(), hi.ln(), 1e-14, 300) {
        Ok(t) => t.exp(),
        Err(_) => 0.0,
    }
}

/// Golden-section minimization of a unimodal function on [a, b].
fn golden<F: Fn(f64) -> f64>(f: &F, mut a: f64, mut b: f64, tol: f64) -> (f64, f64) {
    let r = 0.5 * (5f64.sqrt() - 1.0);
    let mut c = b - r * (b - a);
    let mut d = a + r * (b - a);
    let (mut fc, mut fd) = (f(c), f(d));
    while (b - a).abs() > tol {
        if fc < fd {
            b = d;
            d = c;
            fd = fc;
            c = b - r * (b - a);
            fc = f(c);
        } else {
            a = c;
            c = d;
            fc = fd;
            d = a + r * (b - a);
            fd = f(d);
        }
    }
    let x = 0.5 * (a + b);
    (x, f(x))
}

/// The global minimum of a function of M on [0, mmax]: a grid of 61 points, then
/// golden-section refinement around the three lowest local minima of the grid.
fn min_over_mass<F: Fn(f64) -> f64 + Sync>(f: &F, mmax: f64) -> (f64, f64) {
    let n = 61;
    let ms: Vec<f64> = (0..n).map(|i| mmax * i as f64 / (n - 1) as f64).collect();
    let vals = spacetimes::io::par_map(&ms, |&m| f(m));
    let mut cands: Vec<usize> = (0..n)
        .filter(|&i| (i == 0 || vals[i] <= vals[i - 1]) && (i == n - 1 || vals[i] <= vals[i + 1]))
        .collect();
    cands.sort_by(|&i, &j| vals[i].partial_cmp(&vals[j]).unwrap());
    let mut best = (ms[cands[0]], vals[cands[0]]);
    for &i in cands.iter().take(3) {
        let lo = if i == 0 { 0.0 } else { ms[i - 1] };
        let hi = if i == n - 1 { ms[i] } else { ms[i + 1] };
        let (x, v) = golden(f, lo, hi, 1e-11);
        if v < best.1 {
            best = (x, v);
        }
        if vals[i] < best.1 {
            best = (ms[i], vals[i]);
        }
    }
    best
}

/// The global minimum of Omega (R7).
#[derive(Clone, Copy, Debug)]
pub struct Min {
    pub mass: f64,
    pub d: f64,
    pub omega: f64,
}

/// For a Dirac doublet with scalar coupling the minimum is found on the profile
/// Omega_prof(M) = min over Delta of Omega(M, Delta) = Omega(M, Delta*(M)), where
/// Delta*(M) is the unique root of the gap equation (or 0); the global minimum over M
/// of the profile is the global minimum of Omega.
pub fn minimize(md: &Model) -> Min {
    if md.chiral || md.gs <= 0.0 {
        let mass = if md.chiral { 0.0 } else { md.m };
        let d = solve_delta(md, mass);
        return Min { mass, d, omega: omega7(md, mass, d) };
    }
    let prof = |m: f64| omega7(md, m, solve_delta(md, m));
    let (m0, _) = min_over_mass(&prof, 1.2);
    let mass = polish(&|m: f64| gap_mass(md, m, solve_delta(md, m)), m0);
    let d = solve_delta(md, mass);
    Min { mass, d, omega: omega7(md, mass, d) }
}

/// A minimum located by function values is accurate only to about 1e-8; it is
/// polished with the root of the derivative g = dOmega/dM (total derivative along the
/// profile, because dOmega/dDelta = 0 there).  At the end point M = 0 the minimum is
/// M = 0 if g(0) >= 0.
fn polish<G: Fn(f64) -> f64>(g: &G, m0: f64) -> f64 {
    if m0 < 1e-5 && g(1e-13) >= 0.0 {
        return 0.0;
    }
    let h = 1e-5f64.max(1e-5 * m0);
    let (a, b) = ((m0 - h).max(1e-13), m0 + h);
    let (ga, gb) = (g(a), g(b));
    if ga < 0.0 && gb > 0.0 {
        brent(|x| g(x), a, b, 1e-15, 300).unwrap_or(m0)
    } else {
        m0
    }
}

/// The lowest state with Delta = 0 (the U(2)_U2-symmetric branch): min over M of Omega(M, 0).
pub fn symmetric_branch(md: &Model) -> Min {
    if md.gs <= 0.0 {
        return Min { mass: md.m, d: 0.0, omega: omega7(md, md.m, 0.0) };
    }
    let f = |m: f64| omega7(md, m, 0.0);
    let (m0, _) = min_over_mass(&f, 1.2);
    let mass = polish(&|m: f64| gap_mass(md, m, 0.0), m0);
    Min { mass, d: 0.0, omega: omega7(md, mass, 0.0) }
}

// ------------------------------------------------------------------------- R8 (covariant)
/// F(s) = Int_0^1 u^3 ln(u + s) du.
pub fn f8(s: f64) -> f64 {
    let sl = if s > 0.0 { s.powi(4) / 4.0 * s.ln() } else { 0.0 };
    (1.0 - s.powi(4)) / 4.0 * (1.0 + s).ln() + sl - 1.0 / 16.0 + s / 12.0 - s * s / 8.0 + s.powi(3) / 4.0
}
/// F'(s) = Int_0^1 u^3/(u + s) du.
pub fn f8p(s: f64) -> f64 {
    let l = if s > 0.0 { s.powi(3) * ((1.0 + s) / s).ln() } else { 0.0 };
    1.0 / 3.0 - s / 2.0 + s * s - l
}
/// Int_0^1 u^3/(u + s)^2 du = -F''(s).
pub fn f8pp(s: f64) -> f64 {
    let l = if s > 0.0 { 3.0 * s * s * ((1.0 + s) / s).ln() } else { 0.0 };
    0.5 - 2.0 * s + l + s.powi(3) / (1.0 + s) - s * s
}
/// Omega_R8 relative to the state (M, Delta) = (m, 0).
pub fn omega8(m: f64, gs: f64, gp: f64, mass: f64, d: f64) -> f64 {
    let u = d * d / gp + if gs > 0.0 { (mass - m).powi(2) / (4.0 * gs) } else { 0.0 };
    u - 8.0 * b8() * (f8(mass * mass + d * d) - f8(m * m))
}
/// Vacuum pair field of R8 at fixed M (root of 1/G^P = 8 b8 F'(M^2 + Delta^2)).
pub fn solve_delta8(gp: f64, mass: f64) -> f64 {
    let g = |d: f64| 1.0 / gp - 8.0 * b8() * f8p(mass * mass + d * d);
    if g(0.0) >= 0.0 {
        return 0.0;
    }
    brent(|t: f64| g(t.exp()), (1e-14f64).ln(), (50.0f64).ln(), 1e-14, 300).map(|t| t.exp()).unwrap_or(0.0)
}

// ------------------------------------------------------------------------- R7 closed forms (vacuum)
/// J_n(r) = Int_0^1 q^n / sqrt(q^2 + r^2) dq for n = 0, 2, 4, 6, 8 (reduction formula).
pub fn jn(r: f64) -> [f64; 5] {
    let s = (1.0 + r * r).sqrt();
    let j0 = ((1.0 + s) / r).ln();
    let j2 = 0.5 * s - 0.5 * r * r * j0;
    let j4 = 0.25 * s - 0.75 * r * r * j2;
    let j6 = s / 6.0 - 5.0 / 6.0 * r * r * j4;
    let j8 = s / 8.0 - 7.0 / 8.0 * r * r * j6;
    [j0, j2, j4, j6, j8]
}

fn write_rows(name: &str, meaning: &str, header: &[&str], rows: &[Vec<f64>]) {
    spacetimes::io::write_csv(name, &[("meaning", meaning.to_string()), ("units", "Lambda_U2 = 1".to_string())], header, rows).expect("csv");
}

pub fn run() -> bool {
    let t0 = std::time::Instant::now();
    let mut ck = Checks::new("PROJECT spacetimes, Revision_3, command `potential`: the effective potential of the superfluid, its minima and the breaking U(2)_U2 -> SU(2)_U2");
    let a = a7();
    let mut summary: Vec<Vec<String>> = Vec::new();
    let mut put = |k: &str, v: f64, meaning: &str| summary.push(vec![k.to_string(), format!("{:.10e}", v), meaning.to_string()]);

    // ------------------------------------------------------------- P1, P2: closed forms versus quadrature
    let mut worst = 0.0f64;
    for &r in &[0.05, 0.2, 0.7, 1.3] {
        let j = jn(r);
        let num6 = adaptive(&|q: f64| q.powi(6) / (q * q + r * r).sqrt(), 0.0, 1.0, 1e-16);
        let num_k = adaptive(&|q: f64| q.powi(6) * (q * q + r * r).sqrt(), 0.0, 1.0, 1e-16);
        worst = worst.max((num6 - j[3]).abs()).max((num_k - (j[4] + r * r * j[3])).abs());
    }
    ck.add("R3-P1", "R7 vacuum integrals: Int_0^1 q^6/sqrt(q^2+r^2) dq = J_6(r) and Int_0^1 q^6 sqrt(q^2+r^2) dq = J_8(r) + r^2 J_6(r) with the reduction formula J_n = sqrt(1+r^2)/n - (n-1) r^2 J_{n-2}/n, J_0 = ln((1+sqrt(1+r^2))/r), agree with adaptive quadrature", worst < 1e-14, format!("largest difference {:.1e}", worst));
    let mut worst = 0.0f64;
    for &s in &[0.0, 0.01, 0.3, 1.0, 2.5] {
        let n0 = adaptive(&|u: f64| u.powi(3) * (u + s).ln(), if s == 0.0 { 1e-300 } else { 0.0 }, 1.0, 1e-16);
        let n1 = adaptive(&|u: f64| u.powi(3) / (u + s), 0.0, 1.0, 1e-16);
        let n2 = adaptive(&|u: f64| u.powi(3) / (u + s).powi(2), 0.0, 1.0, 1e-16);
        worst = worst.max((n0 - f8(s)).abs()).max((n1 - f8p(s)).abs()).max((n2 - f8pp(s)).abs());
    }
    ck.add("R3-P2", "R8 closed forms: F(s) = Int_0^1 u^3 ln(u+s) du = (1-s^4)/4 ln(1+s) + s^4/4 ln s - 1/16 + s/12 - s^2/8 + s^3/4, F'(s) = 1/3 - s/2 + s^2 - s^3 ln((1+s)/s), Int_0^1 u^3/(u+s)^2 du = 1/2 - 2s + 3 s^2 ln((1+s)/s) + s^3/(1+s) - s^2 agree with quadrature", worst < 1e-13, format!("largest difference {:.1e}", worst));

    // ------------------------------------------------------------- P3: critical couplings
    let vac = Model::pairing(0.0, 1.0, 0.0);
    let gc7_num = 1.0 / (0.5 * 8.0 * a * adaptive(&|q: f64| q.powi(6) * 2.0 / q, 0.0, 1.0, 1e-16));
    let gc8_num = 1.0 / (8.0 * b8() * f8p(0.0));
    let chiral = Model { chiral: true, ..vac };
    let g_ch = 1.0 / (0.5 * chiral.deg() * a * adaptive(&|q: f64| q.powi(6) * 2.0 / q, 0.0, 1.0, 1e-16));
    ck.add("R3-P3", "critical couplings of the vacuum: R7: G^P_crit Lambda^6 = 3/(4 a7) = 90 pi^4; R8: 3/(8 b8) = 288 pi^4; the left-handed doublet needs twice the R7 value, 180 pi^4", (gc7_num / (90.0 * PI.powi(4)) - 1.0).abs() < 1e-13 && (gc8_num / (288.0 * PI.powi(4)) - 1.0).abs() < 1e-13 && (g_ch / (180.0 * PI.powi(4)) - 1.0).abs() < 1e-13, format!("R7 {:.6} = 90 pi^4 = {:.6}; R8 {:.6} = 288 pi^4 = {:.6}; chiral {:.6}", gc7_num, gcrit7(), gc8_num, gcrit8(), g_ch));
    put("Gcrit_R7", gcrit7(), "critical pair coupling G^P_crit Lambda^6 of R7 (90 pi^4)");
    put("Gcrit_R8", gcrit8(), "critical pair coupling G^P_crit Lambda^6 of R8 (288 pi^4)");
    put("Gcrit_chiral_R7", g_ch, "critical pair coupling of the left-handed doublet (R7), 180 pi^4");

    // ------------------------------------------------------------- vacuum gap versus lambda (R7, R8, chiral)
    let mut rows = Vec::new();
    let (mut ok_gap, mut ok_min, mut ok_zero) = (true, true, true);
    let mut worst_res = 0.0f64;
    for k in 0..=125 {
        let lp = 0.5 + 0.02 * k as f64;
        let md = Model::pairing(0.0, lp, 0.0);
        let d7 = solve_delta(&md, 0.0);
        let d8 = solve_delta8(lp * gcrit8(), 0.0);
        let mc = Model { chiral: true, ..md };
        let dch = solve_delta(&mc, 0.0);
        let land7 = if lp > 1.0 { (4.0 / 3.0 * (1.0 - 1.0 / lp)).sqrt() } else { 0.0 };
        let land8 = if lp > 1.0 { (2.0 / 3.0 * (1.0 - 1.0 / lp)).sqrt() } else { 0.0 };
        let om7 = omega7(&md, 0.0, d7);
        let om8 = omega8(0.0, 0.0, lp * gcrit8(), 0.0, d8);
        if lp > 1.0 + 1e-9 {
            ok_gap &= d7 > 0.0 && d8 > 0.0;
            worst_res = worst_res.max((gap_delta(&md, 0.0, d7) * md.gp).abs());
            // a minimum: lower than Delta = 0 and lower than its neighbours
            let h = 1e-4 * d7;
            ok_min &= om7 < 0.0 && omega7(&md, 0.0, d7 + h) > om7 && omega7(&md, 0.0, d7 - h) > om7;
        } else if lp < 1.0 - 1e-9 {
            ok_zero &= d7 == 0.0 && d8 == 0.0;
            // Delta = 0 is the global minimum: Omega > 0 on a scan
            for j in 1..=40 {
                ok_zero &= omega7(&md, 0.0, 0.025 * j as f64) > 0.0;
            }
        }
        rows.push(vec![lp, d7, d8, land7, land8, dch, om7, om8]);
    }
    write_rows("r3_vacuum_gap.csv", "vacuum (mu = 0, m = 0, no scalar coupling): pair field at the minimum versus lambda^P = G^P/G^P_crit in R7 and R8, the Landau (small-gap) forms sqrt(4/3 (1-1/lambda)) and sqrt(2/3 (1-1/lambda)), the left-handed doublet (R7, same G^P_crit of R7 in lambda), and the minimum value of Omega", &["lambda", "Delta_R7", "Delta_R8", "Landau_R7", "Landau_R8", "Delta_chiral_R7", "Omega_R7", "Omega_R8"], &rows);
    ck.add("R3-P4", "vacuum, lambda^P > 1: the gap equation 1/G^P = 4 a7 Int q^6 (1/E^+ + 1/E^-) has a root Delta > 0 (residual below 1e-12 in units of 1/G^P), Omega(Delta) < Omega(0) and Delta is a minimum along the radial direction; R8 also has Delta > 0 (102 values of lambda in (1, 3])", ok_gap && ok_min && worst_res < 1e-12, format!("largest residual {:.1e}", worst_res));
    ck.add("R3-P5", "vacuum, lambda^P < 1: Delta = 0 is the global minimum in both regularizations (Omega > 0 on a scan of 40 values of Delta up to 1): no breaking below the critical coupling", ok_zero, String::new());
    // Landau limit
    let lp = 1.0005;
    let d7 = solve_delta(&Model::pairing(0.0, lp, 0.0), 0.0);
    let d8 = solve_delta8(lp * gcrit8(), 0.0);
    let r7 = d7 / (4.0 / 3.0 * (1.0 - 1.0 / lp)).sqrt();
    let r8 = d8 / (2.0 / 3.0 * (1.0 - 1.0 / lp)).sqrt();
    ck.add("R3-P6", "near the critical coupling the gap follows the Landau form of the Mexican hat: Delta^2 = (4/3)(1 - 1/lambda) (R7) and (2/3)(1 - 1/lambda) (R8), from Omega = c2 Delta^2 + c4 Delta^4 with c2 = (4/3) a7 (1/lambda - 1), c4 = a7/2 (R7); at lambda = 1.0005 the ratios of the computed gap to these forms are 1 within 1 per cent", (r7 - 1.0).abs() < 0.01 && (r8 - 1.0).abs() < 0.01, format!("R7 ratio {:.5}, R8 ratio {:.5}", r7, r8));
    for (lp, tag) in [(1.02, "102"), (1.05, "105"), (1.1, "110")] {
        let d7 = solve_delta(&Model::pairing(0.0, lp, 0.0), 0.0);
        put(&format!("Delta_vac_R7_l{}", tag), d7, &format!("vacuum pair field Delta/Lambda at lambda^P = {} (R7)", lp));
        put(&format!("Delta_vac_R8_l{}", tag), solve_delta8(lp * gcrit8(), 0.0), &format!("vacuum pair field Delta/Lambda at lambda^P = {} (R8)", lp));
        put(&format!("Omega_vac_R7_l{}", tag), omega7(&Model::pairing(0.0, lp, 0.0), 0.0, d7), &format!("vacuum Omega_min/Lambda^8 at lambda^P = {} (R7)", lp));
    }

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "exican hat");
    // ------------------------------------------------------------- Mexican hat: radial cuts and a 2-D map
    let lams = [0.95, 1.0, 1.02, 1.05, 1.1];
    let mut rows = Vec::new();
    for k in 0..=180 {
        let d = 0.5 * k as f64 / 180.0;
        let mut r = vec![d];
        for &lp in &lams {
            r.push(omega7(&Model::pairing(0.0, lp, 0.0), 0.0, d));
        }
        rows.push(r);
    }
    write_rows("r3_hat_radial.csv", "vacuum Omega_R7(Delta) - Omega_R7(0) (units Lambda^8) for lambda^P = 0.95, 1.0, 1.02, 1.05, 1.1 (m = 0, no scalar coupling)", &["Delta", "l095", "l100", "l102", "l105", "l110"], &rows);
    let md = Model::pairing(0.0, 1.05, 0.0);
    let mut rows = Vec::new();
    let n2 = 61;
    for i in 0..n2 {
        for j in 0..n2 {
            let x = -0.45 + 0.9 * i as f64 / (n2 - 1) as f64;
            let y = -0.45 + 0.9 * j as f64 / (n2 - 1) as f64;
            rows.push(vec![x, y, omega7(&md, 0.0, (x * x + y * y).sqrt())]);
        }
    }
    write_rows("r3_hat_2d.csv", "vacuum Omega_R7 as a function of the real and imaginary parts (Delta_1, Delta_2) of the pair field, lambda^P = 1.05, m = 0", &["Delta1", "Delta2", "Omega"], &rows);

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "7:");
    // ------------------------------------------------------------- P7: competition of mass and pair (vacuum)
    let cases = [(1.05, 1.1), (1.1, 1.1), (1.1, 1.05)];
    let mut rows = Vec::new();
    let mut comp_res = Vec::new();
    for (ci, &(ls, lp)) in cases.iter().enumerate() {
        let md = Model { m: 0.0, gs: ls * gcrit7() / 4.0, gp: lp * gcrit7(), mu: 0.0, chiral: false };
        let nn = 41;
        for i in 0..nn {
            for j in 0..nn {
                let mass = 0.5 * i as f64 / (nn - 1) as f64;
                let d = 0.5 * j as f64 / (nn - 1) as f64;
                rows.push(vec![ci as f64, mass, d, omega7(&md, mass, d)]);
            }
        }
        let mn = minimize(&md);
        comp_res.push((ls, lp, mn));
    }
    write_rows("r3_competition.csv", "vacuum Omega_R7(M, Delta) on a grid for (lambda^S, lambda^P) = (1.05, 1.1) [case 0], (1.1, 1.1) [case 1], (1.1, 1.05) [case 2], m = 0", &["case", "M", "Delta", "Omega"], &rows);
    let d_15 = solve_delta(&Model::pairing(0.0, 1.1, 0.0), 0.0);
    let (_, _, m0) = comp_res[0];
    let (_, _, m2) = comp_res[2];
    let d_125 = solve_delta(&Model::pairing(0.0, 1.05, 0.0), 0.0);
    // case 1: Omega constant on the circle M^2 + Delta^2 = r^2
    let md1 = Model { m: 0.0, gs: 1.1 * gcrit7() / 4.0, gp: 1.1 * gcrit7(), mu: 0.0, chiral: false };
    let mut circ = Vec::new();
    for k in 0..=12 {
        let th = 0.5 * PI * k as f64 / 12.0;
        circ.push(omega7(&md1, d_15 * th.cos(), d_15 * th.sin()));
    }
    let spread = circ.iter().cloned().fold(f64::MIN, f64::max) - circ.iter().cloned().fold(f64::MAX, f64::min);
    let ok = m0.mass.abs() < 1e-9 && (m0.d - d_15).abs() < 1e-9 && m2.d.abs() < 1e-12 && (m2.mass - d_15).abs() < 1e-9 && spread < 1e-9 * circ[0].abs() && d_125 > 0.0;
    ck.add("R3-P7", "competition in the vacuum (m = 0): the fermion term depends on M^2 + Delta^2 only, so the minimum is the pure pair state (M, Delta) = (0, Delta(lambda^P)) if lambda^P > lambda^S, the pure mass state (M(lambda^S), 0) - which does NOT break the gauge group - if lambda^S > lambda^P, and a whole circle M^2 + Delta^2 = r^2 of degenerate minima if lambda^S = lambda^P (G^P = 4 G^S)", ok, format!("(1.05,1.1): M = {:.2e}, Delta = {:.10}; (1.1,1.05): M = {:.10}, Delta = {:.1e}; spread of Omega on the circle {:.1e} (Omega = {:.6e})", m0.mass, m0.d, m2.mass, m2.d, spread, circ[0]));

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "inite density, m = 0");
    // ------------------------------------------------------------- finite density, m = 0
    let lps = [0.5, 0.8, 0.95, 1.05];
    let mut rows = Vec::new();
    let mut ok_cooper = true;
    let (mut n_repr, mut n_tiny) = (0, 0);
    for k in 0..=60 {
        let mu = 0.01 * k as f64;
        let mut r = vec![mu];
        for &lp in &lps {
            let md = Model::pairing(0.0, lp, mu);
            let mn = minimize(&md);
            // log10 of the pair field; below 1e-300 (not representable in double precision) the
            // value follows from the logarithm of check R3-P8a:
            // ln Delta = ln 1e-300 - gap(1e-300)/(8 a7 q_F^5 mu)
            let lg = if mn.d > 0.0 {
                n_repr += (mu > 0.0) as usize;
                mn.d.log10()
            } else if mu > 0.0 {
                n_tiny += 1;
                let g0 = gap_delta(&md, 0.0, 1e-300);
                ok_cooper &= g0 > 0.0;
                (-300.0 * 10f64.ln() - g0 / (8.0 * a * mu.powi(6))) / 10f64.ln()
            } else {
                f64::NEG_INFINITY
            };
            r.push(mn.d);
            r.push(lg);
            r.push(density(&md, 0.0, mn.d));
            r.push(mn.omega);
        }
        rows.push(r);
    }
    write_rows("r3_density_scan.csv", "pair field, its log10 (below 1e-300 from the Cooper logarithm), number density and Omega_min - Omega_normal versus mu (m = 0, no scalar coupling) for lambda^P = 0.5, 0.8, 0.95, 1.05 (R7)", &["mu", "D050", "lgD050", "n050", "O050", "D080", "lgD080", "n080", "O080", "D095", "lgD095", "n095", "O095", "D105", "lgD105", "n105", "O105"], &rows);
    // the logarithm behind the Cooper instability: 1/G^P - gap_delta grows like 8 a7 q_F^5 mu ln(1/Delta)
    let mut worst_log = 0.0f64;
    for &(mu, lp) in &[(0.2, 0.5), (0.4, 0.8), (0.55, 0.3)] {
        let md = Model::pairing(0.0, lp, mu);
        let g1 = gap_delta(&md, 0.0, 1e-100);
        let g2 = gap_delta(&md, 0.0, 1e-200);
        let slope = (g1 - g2) / (100.0 * 10f64.ln());
        worst_log = worst_log.max((slope / (8.0 * a * mu.powi(6)) - 1.0).abs());
    }
    ck.add("R3-P8a", "the logarithm behind the Cooper instability: for mu > 0 the integral 4 a7 Int q^6 (1/E^+ + 1/E^-) grows like 8 a7 q_F^5 mu ln(1/Delta) as Delta -> 0 (computed at Delta = 1e-100 and 1e-200 with the variable xi = Delta sinh t), so the gap equation 1/G^P = ... has a root for EVERY G^P > 0", worst_log < 1e-9, format!("largest relative deviation of the slope {:.1e}", worst_log));
    ck.add("R3-P8", "Cooper instability: at every mu > 0 of the scan (60 values up to 0.6) and every lambda^P in {0.5, 0.8, 0.95, 1.05}, also below the critical coupling of the vacuum, the gap equation has a root Delta > 0; where Delta < 1e-300 (weak coupling at small mu) it is not representable in double precision, and its logarithm follows from R3-P8a", ok_cooper, format!("{} points with Delta > 1e-300, {} points with 0 < Delta < 1e-300", n_repr, n_tiny));

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "9");
    // ------------------------------------------------------------- P9: BCS law
    let mu = 0.4;
    let mut rows = Vec::new();
    for k in 0..=20 {
        let lp = 0.15 + 0.015 * k as f64;
        let md = Model::pairing(0.0, lp, mu);
        let d = solve_delta(&md, 0.0);
        rows.push(vec![lp, 1.0 / md.gp, d, d.ln()]);
    }
    let slope = (rows[1][3] - rows[0][3]) / (rows[1][1] - rows[0][1]);
    let pred = -1.0 / (8.0 * a * mu.powi(5) * mu);
    write_rows("r3_bcs_slope.csv", "weak-coupling (BCS) law at mu = 0.4, m = 0: ln Delta versus 1/G^P; predicted slope -1/(8 a7 q_F^5 mu) with q_F = mu", &["lambda", "inv_G", "Delta", "lnDelta"], &rows);
    ck.add("R3-P9", "weak-coupling (BCS) law: ln Delta = const - 1/(8 a7 q_F^5 mu G^P) at a Fermi surface; at mu = 0.4 the slope of ln Delta against 1/G^P at the two smallest couplings agrees with -1/(8 a7 q_F^5 mu) within 2 per cent", ((slope / pred) - 1.0).abs() < 0.02, format!("slope {:.6e}, predicted {:.6e}, ratio {:.5}", slope, pred, slope / pred));
    put("BCS_slope_fit", slope, "slope d ln Delta / d(1/G^P) at mu = 0.4, two smallest couplings");
    put("BCS_slope_pred", pred, "predicted slope -1/(8 a7 q_F^5 mu), q_F = mu = 0.4");

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "10");
    // ------------------------------------------------------------- P10: density-driven breaking (mass phase -> superfluid)
    let (ls, lp) = (1.1, 1.05);
    let mut rows = Vec::new();
    let mut prev: Option<(f64, f64)> = None;
    let mut mu_c = f64::NAN;
    let mut d_at_0 = -1.0;
    let mut d_at_end = -1.0;
    for k in 0..=60 {
        let mu = 0.01 * k as f64;
        let md = Model { m: 0.0, gs: ls * gcrit7() / 4.0, gp: lp * gcrit7(), mu, chiral: false };
        let mn = minimize(&md);
        let sym = symmetric_branch(&md);
        if k == 0 {
            d_at_0 = mn.d;
        }
        d_at_end = mn.d;
        if let Some((pm, pd)) = prev {
            if pd == 0.0 && mn.d > 0.0 && mu_c.is_nan() {
                mu_c = mu - 0.005;
                let _ = pm;
            }
        }
        prev = Some((mn.mass, mn.d));
        rows.push(vec![mu, mn.mass, mn.d, mn.omega, sym.mass, sym.omega, density(&md, mn.mass, mn.d)]);
    }
    write_rows("r3_transition.csv", "density-driven breaking: lambda^S = 1.1, lambda^P = 1.05, m = 0 (R7): global minimum (M, Delta, Omega), the U(2)-symmetric branch (Delta = 0: M, Omega) and the density at the minimum, versus mu", &["mu", "M", "Delta", "Omega", "M_sym", "Omega_sym", "n"], &rows);
    ck.add("R3-P10", "density-driven breaking of the gauge group: for lambda^S = 1.1 > lambda^P = 1.05 the vacuum is the massive, U(2)_U2-symmetric state (Delta = 0); as mu grows the global minimum jumps (first-order transition) to the superfluid with Delta > 0, which breaks U(2)_U2 -> SU(2)_U2", d_at_0 == 0.0 && d_at_end > 0.0 && !mu_c.is_nan(), format!("Delta(mu=0) = {}, Delta(mu=0.6) = {:.6}, transition between mu = {:.3} and {:.3}", d_at_0, d_at_end, mu_c - 0.005, mu_c + 0.005));
    put("mu_transition", mu_c, "chemical potential (midpoint of the scan step) at which the minimum changes from the massive symmetric state to the superfluid, lambda^S = 1.1, lambda^P = 1.05");

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "hase map");
    // ------------------------------------------------------------- phase map (m = 0, no scalar coupling)
    let mut rows = Vec::new();
    let ml: Vec<f64> = (0..=44).map(|k| 0.1 + 0.025 * k as f64).collect();
    let mm: Vec<f64> = (0..=30).map(|k| 0.02 * k as f64).collect();
    let pairs: Vec<(f64, f64)> = ml.iter().flat_map(|&l| mm.iter().map(move |&m| (l, m))).collect();
    let vals = spacetimes::io::par_map(&pairs, |&(l, mu)| solve_delta(&Model::pairing(0.0, l, mu), 0.0));
    for ((l, mu), d) in pairs.iter().zip(vals) {
        rows.push(vec![*l, *mu, d]);
    }
    write_rows("r3_phase_map.csv", "pair field at the minimum on the plane of lambda^P and mu (m = 0, no scalar coupling, R7)", &["lambda", "mu", "Delta"], &rows);

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "11");
    // ------------------------------------------------------------- P11: thermodynamic consistency n = -dOmega/dmu
    let mut worst = 0.0f64;
    for &mu in &[0.2, 0.35, 0.5] {
        let h = 1e-4;
        let om = |m: f64| {
            let md = Model::pairing(0.0, 0.8, m);
            let mn = minimize(&md);
            mn.omega
        };
        // five-point stencil (truncation error of order h^4; Omega_min varies steeply where the gap opens)
        let num = -(om(mu - 2.0 * h) - 8.0 * om(mu - h) + 8.0 * om(mu + h) - om(mu + 2.0 * h)) / (12.0 * h); // = n_sf - n_normal
        let md = Model::pairing(0.0, 0.8, mu);
        let mn = minimize(&md);
        let n_sf = density(&md, 0.0, mn.d);
        let n_normal = 16.0 * a * mu.powi(7) / 7.0;
        worst = worst.max(((n_sf - n_normal) - num).abs() / n_sf);
    }
    ck.add("R3-P11", "thermodynamic consistency: the density n = 8 a7 Int q^6 [(eps+mu)/E^+ - (eps-mu)/E^-] of the superfluid minus the density 16 a7 mu^7/7 of the normal massless gas equals -d(Omega_min - Omega_normal)/dmu (five-point differences, lambda^P = 0.8, mu = 0.2, 0.35, 0.5); the derivative of Omega_min at fixed parameters needs no derivative of Delta because Delta is stationary", worst < 1e-6, format!("largest relative difference {:.1e}", worst));

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "pectra");
    // ------------------------------------------------------------- spectra for the figure (self-consistent)
    let dvac = solve_delta(&Model::pairing(0.0, 1.05, 0.0), 0.0);
    let dden = solve_delta(&Model::pairing(0.0, 0.8, 0.4), 0.0);
    let mut rows = Vec::new();
    for k in 0..=400 {
        let q = k as f64 / 400.0;
        let (_, emv) = e_cplus(q, 0.0, 0.0, dvac);
        let (epd, emd) = e_cplus(q, 0.0, 0.4, dden);
        rows.push(vec![q, q, emv, (q - 0.4).abs(), q + 0.4, emd, epd]);
    }
    write_rows("r3_spectrum.csv", "quasi-particle energies (m = 0): vacuum normal q and vacuum superfluid sqrt(q^2+Delta^2) at lambda^P = 1.05; mu = 0.4, lambda^P = 0.8: normal |q-mu|, q+mu and superfluid E^-, E^+ (self-consistent Delta)", &["q", "vac_normal", "vac_sf", "den_normal_minus", "den_normal_plus", "den_sf_minus", "den_sf_plus"], &rows);
    put("Delta_spectrum_vac", dvac, "Delta at lambda^P = 1.05, mu = 0 (R7), used in the spectrum figure");
    put("Delta_spectrum_den", dden, "Delta at lambda^P = 0.8, mu = 0.4 (R7), used in the spectrum figure");

    eprintln!("[potential] {:.1} s: {}", t0.elapsed().as_secs_f64(), "12");
    // ------------------------------------------------------------- P12: chiral doublet
    let mut ok = true;
    for &lp in &[1.5, 2.1, 2.2] {
        let mc = Model { chiral: true, ..Model::pairing(0.0, lp, 0.0) };
        let d = solve_delta(&mc, 0.0);
        // the chiral doublet at G^P equals the Dirac doublet at G^P/2
        let dd = solve_delta(&Model::pairing(0.0, lp / 2.0, 0.0), 0.0);
        ok &= (d - dd).abs() < 1e-12 && ((lp < 2.0) == (d == 0.0));
    }
    let mc = Model { chiral: true, ..Model::pairing(0.0, 0.8, 0.4) };
    let dch = solve_delta(&mc, 0.0);
    ok &= dch > 0.0;
    ck.add("R3-P12", "the left-handed (chiral) doublet: its potential at coupling G^P equals that of the Dirac doublet at G^P/2 (half the states), so in the vacuum it pairs only for G^P > 2 G^P_crit; at finite density it pairs for any attraction (lambda^P = 0.8, mu = 0.4: Delta > 0)", ok, format!("Delta(chiral, lambda^P = 0.8, mu = 0.4) = {:.6e}", dch));

    spacetimes::io::write_csv_str("r3_summary_potential.csv", &[("meaning", "key numbers of the command `potential` (units Lambda_U2 = 1)".to_string())], &["name", "value", "meaning"], &summary).expect("csv");
    ck.write("r3_potential_checks.txt")
}
