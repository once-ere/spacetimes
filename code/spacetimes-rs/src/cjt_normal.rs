//! The CJT effective potential for the EXPECTATION VALUE of the Vorton-Scalar: the
//! unpaired ("normal") state of the fermions at finite density and at finite
//! temperature.  Derivation: supplement S4, Secs. 5 and 6.
//!
//! In the CJT action the classical field phi = <Phi_VS> sqrt(2) is a variational
//! variable beside the propagators.  Its stationarity condition is the field equation
//! of the Vorton-Scalar with the fermion condensate as source (S1, Eq. (1.8)).  Because
//! the fermion mass IS the field, m* = g_VS phi, this one equation decides how the
//! mass of the fermions changes in a medium.
//!
//! PART 1, zero temperature, density n = nu k_F^3/(6 pi^2), nu = 8 internal states.
//! In units of the vacuum mass m the energy density as a function of x = m*/m is
//!
//!   eps(k_F, x) = (x^2 - 1)^2/(8 C)                 potential of the Vorton-Scalar, tree level
//!               + nu Int_0^{k_F} k^2 dk/(2 pi^2) sqrt(k^2 + x^2)        the Fermi sea of particles
//!               + V_sea(x)                          (optional) the filled negative-energy states
//!
//!   C = g_VS^2 m^2/m_s^2 = g_VS^4/(2 lambda_VS) ,
//!   V_sea(x) = -(nu/(32 pi^2)) [x^4 ln x^2 - (3/2) x^4 + 2 x^2 - 1/2] .
//!
//! V_sea is the one-loop fermion term renormalized with the two counterterms that
//! the symmetry U(1)_VS allows, fixed so that the vacuum value and the mass of the
//! radial mode keep their tree-level values (V_sea = V_sea' = V_sea'' = 0 at x = 1).
//! Stationarity in x is the in-medium field equation; the stable state is the lowest
//! minimum on 0 <= x <= 1.  x = 0 is the state in which the Vorton-Scalar has no
//! expectation value and the fermions are massless ("abnormal" state, Lee and Wick).
//! The exchange (Fock) energy of the two-loop Gamma_2 is evaluated to first order by
//! `fock_energy`.
//!
//! PART 2, finite temperature, no net density (`Thermal`): fermions at one loop and
//! the two boson propagators in the Hartree (double-bubble) truncation of Gamma_2;
//! the temperature at which the expectation value melts and the fermions lose their
//! mass.
//!
//! UNITS: hbar = c = k_B = 1; energies in units of the vacuum fermion mass m.

use crate::gap::angular_average;
use crate::quad::{gauss_legendre, Grid};

/// Number of internal states of a particle of given momentum.
pub const NU: f64 = 8.0;

const PI: f64 = std::f64::consts::PI;

/// Int_0^{k_F} k^2 dk/(2 pi^2) / sqrt(k^2 + x^2) = [k_F E_F - x^2 ln((k_F + E_F)/x)]/(4 pi^2).
pub fn i_scalar(kf: f64, x: f64) -> f64 {
    let ef = (kf * kf + x * x).sqrt();
    let log = if x > 1e-150 { x * x * ((kf + ef) / x).ln() } else { 0.0 };
    (kf * ef - log) / (4.0 * PI * PI)
}

/// Int_0^{k_F} k^2 dk/(2 pi^2) sqrt(k^2 + x^2) = [k_F E_F (2 k_F^2 + x^2) - x^4 ln((k_F + E_F)/x)]/(16 pi^2).
pub fn e_kinetic(kf: f64, x: f64) -> f64 {
    let ef = (kf * kf + x * x).sqrt();
    let log = if x > 1e-75 { x.powi(4) * ((kf + ef) / x).ln() } else { 0.0 };
    (kf * ef * (2.0 * kf * kf + x * x) - log) / (16.0 * PI * PI)
}

/// The renormalized one-loop energy of the filled negative-energy states (see the
/// module documentation); zero with zero first and second derivative at x = 1.
pub fn v_sea(x: f64) -> f64 {
    let x2 = x * x;
    let log = if x > 1e-75 { x2 * x2 * x2.ln() } else { 0.0 };
    -(NU / (32.0 * PI * PI)) * (log - 1.5 * x2 * x2 + 2.0 * x2 - 0.5)
}

/// A state of the normal matter at zero temperature.
#[derive(Clone, Copy, Debug)]
pub struct State {
    pub kf: f64,
    /// m*/m
    pub x: f64,
    /// number density
    pub n: f64,
    /// energy density, vacuum subtracted
    pub eps: f64,
    /// chemical potential d eps/d n = sqrt(k_F^2 + x^2)
    pub mu: f64,
    /// pressure mu n - eps
    pub p: f64,
}

/// The mean-field model of the normal state.
#[derive(Clone, Copy, Debug)]
pub struct Medium {
    /// C = g^2 m^2/m_s^2
    pub c: f64,
    /// include the energy of the filled negative-energy states
    pub sea: bool,
}

impl Medium {
    /// Energy density for given Fermi momentum and trial mass x.
    pub fn energy(&self, kf: f64, x: f64) -> f64 {
        (x * x - 1.0).powi(2) / (8.0 * self.c) + NU * e_kinetic(kf, x) + if self.sea { v_sea(x) } else { 0.0 }
    }

    /// The mass x = m*/m of the stable state: lowest minimum of the energy density on
    /// [0, 1] at fixed density (scan, then golden-section refinement).
    pub fn mass(&self, kf: f64) -> f64 {
        let n = 400;
        let mut best = (f64::MAX, 0usize);
        for i in 0..=n {
            let x = i as f64 / n as f64;
            let e = self.energy(kf, x);
            if e < best.0 {
                best = (e, i);
            }
        }
        let (mut a, mut b) = ((best.1.saturating_sub(1)) as f64 / n as f64, ((best.1 + 1).min(n)) as f64 / n as f64);
        let r = 0.5 * (5f64.sqrt() - 1.0);
        let (mut x1, mut x2) = (b - r * (b - a), a + r * (b - a));
        let (mut f1, mut f2) = (self.energy(kf, x1), self.energy(kf, x2));
        for _ in 0..80 {
            if f1 < f2 {
                b = x2;
                x2 = x1;
                f2 = f1;
                x1 = b - r * (b - a);
                f1 = self.energy(kf, x1);
            } else {
                a = x1;
                x1 = x2;
                f1 = f2;
                x2 = a + r * (b - a);
                f2 = self.energy(kf, x2);
            }
        }
        let mut x = 0.5 * (a + b);
        if x < 1e-6 {
            x = 0.0;
        } else {
            // polish with the field equation F(x) = 0 (the energy locates its minimum
            // only to the square root of the rounding error)
            let (lo, hi) = (x * (1.0 - 1e-4), (x * (1.0 + 1e-4)).min(1.0));
            if self.field_equation(kf, lo) * self.field_equation(kf, hi) < 0.0 {
                if let Ok(r) = crate::roots::brent(|y| self.field_equation(kf, y), lo, hi, 1e-15, 100) {
                    x = r;
                }
            }
        }
        // the end points are candidates too (x = 0 is the abnormal state; it is
        // preferred when the energies are equal to rounding)
        let mut out = (self.energy(kf, x), x);
        for &xe in &[0.0, 1.0] {
            let e = self.energy(kf, xe);
            if e < out.0 || (xe == 0.0 && e <= out.0) {
                out = (e, xe);
            }
        }
        out.1
    }

    /// The in-medium field equation of the Vorton-Scalar divided by x:
    ///   F(x) = (x^2 - 1)/(2 C) + nu I(k_F, x) - [sea] (nu/(8 pi^2)) (x^2 ln x^2 - x^2 + 1) ,
    /// so that d eps/d x = x F(x).  A stationary state with x > 0 has F = 0.
    pub fn field_equation(&self, kf: f64, x: f64) -> f64 {
        let sea = if self.sea {
            let x2 = x * x;
            -(NU / (8.0 * PI * PI)) * (if x > 1e-150 { x2 * x2.ln() } else { 0.0 } - x2 + 1.0)
        } else {
            0.0
        };
        (x * x - 1.0) / (2.0 * self.c) + NU * i_scalar(kf, x) + sea
    }

    /// The stable state at Fermi momentum `kf`.
    pub fn state(&self, kf: f64) -> State {
        let x = self.mass(kf);
        let n = NU * kf.powi(3) / (6.0 * PI * PI);
        let eps = self.energy(kf, x);
        let mu = (kf * kf + x * x).sqrt();
        State { kf, x, n, eps, mu, p: mu * n - eps }
    }

    /// Square of the sound speed, dP/d eps = (n/mu) d mu/d n, by a central difference.
    pub fn sound_speed_sq(&self, kf: f64) -> f64 {
        let h = 1e-4 * kf;
        let (a, b) = (self.state(kf - h), self.state(kf + h));
        let s = self.state(kf);
        (s.n / s.mu) * (b.mu - a.mu) / (b.n - a.n)
    }

    /// Lowest energy per particle of matter at any density, in units of m:
    /// returns (E/N at the minimum, k_F there, x there).  If the result is below 1,
    /// matter is self-bound: N fermions together weigh less than N free fermions.
    pub fn saturation(&self) -> (f64, f64, f64) {
        // scan log k_F, then refine
        let e_per_n = |kf: f64| {
            let s = self.state(kf);
            s.eps / s.n
        };
        let n = 600;
        let (lo, hi) = (0.02f64.ln(), 20f64.ln());
        let mut best = (f64::MAX, lo);
        for i in 0..=n {
            let l = lo + (hi - lo) * i as f64 / n as f64;
            let v = e_per_n(l.exp());
            if v < best.0 {
                best = (v, l);
            }
        }
        let step = (hi - lo) / n as f64;
        let (mut a, mut b) = (best.1 - step, best.1 + step);
        let r = 0.5 * (5f64.sqrt() - 1.0);
        for _ in 0..60 {
            let (x1, x2) = (b - r * (b - a), a + r * (b - a));
            if e_per_n(x1.exp()) < e_per_n(x2.exp()) {
                b = x2;
            } else {
                a = x1;
            }
        }
        let kf = (0.5 * (a + b)).exp();
        let s = self.state(kf);
        (s.eps / s.n, kf, s.x)
    }
}

/// The smallest C for which matter is self-bound (bisection on `saturation`).
pub fn critical_c(sea: bool) -> f64 {
    let bound = |c: f64| Medium { c, sea }.saturation().0 < 1.0 - 1e-9;
    let (mut lo, mut hi) = (0.5, 40.0);
    assert!(!bound(lo) && bound(hi));
    for _ in 0..50 {
        let mid = 0.5 * (lo + hi);
        if bound(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    0.5 * (lo + hi)
}

/// Exchange (Fock) energy density of the two modes of the Vorton-Scalar for a Fermi
/// sea of particles with mass parameter x, to first order (static exchange):
///
///   E_x = (g^2/2) (nu/2) Int Int_{k, k' < k_F} d^3k d^3k'/(2 pi)^6
///            (E E' +- x^2 - k.k') / (E E')  /  (|k - k'|^2 + M^2) ,
///
/// upper sign and M = m_s for the radial mode, lower sign and M = 0 for the phase
/// mode.  The factor (nu/2)(E E' +- x^2 - k.k')/(E E') is the trace
/// Tr[Lambda_+(k') Gamma Lambda_+(k) Gamma] over the internal states, with Lambda_+ the
/// projector on the particle states and Gamma = beta or i beta chi.  Both energies
/// are positive: exchange reduces the attraction of the mean field.
/// Returns (radial, phase).
pub fn fock_energy(kf: f64, x: f64, g2: f64, m_s: f64) -> (f64, f64) {
    // composite Gauss-Legendre grid on [0, k_F]
    let gl = gauss_legendre(12);
    let mut grid = Grid::new();
    for p in 0..8 {
        grid.push_panel(kf * p as f64 / 8.0, kf * (p + 1) as f64 / 8.0, &gl);
    }
    let (mut es, mut ep) = (0.0, 0.0);
    for (i, &k) in grid.x.iter().enumerate() {
        let e = (k * k + x * x).sqrt();
        for (j, &kp) in grid.x.iter().enumerate() {
            let e2 = (kp * kp + x * x).sqrt();
            let meas = grid.w[i] * k * k / (2.0 * PI * PI) * grid.w[j] * kp * kp / (2.0 * PI * PI);
            // angular averages of (a + b cos)/(c - d cos)
            let s = angular_average(e * e2 + x * x, -k * kp, k * k + kp * kp + m_s * m_s, 2.0 * k * kp);
            let p = angular_average(e * e2 - x * x, -k * kp, k * k + kp * kp, 2.0 * k * kp);
            es += meas * s / (e * e2);
            ep += meas * p / (e * e2);
        }
    }
    (0.5 * g2 * 0.5 * NU * es, 0.5 * g2 * 0.5 * NU * ep)
}

// ------------------------------------------------------------------------------------
// Finite temperature
// ------------------------------------------------------------------------------------

/// Composite Gauss-Legendre grid in y = k/T on [0, 60].
fn thermal_grid() -> Grid {
    let gl = gauss_legendre(24);
    let mut g = Grid::new();
    for w in [0.0, 0.05, 0.3, 1.0, 3.0, 8.0, 20.0, 60.0].windows(2) {
        g.push_panel(w[0], w[1], &gl);
    }
    g
}

/// Thermal integrals in units of the temperature, as functions of a = M/T:
/// (P_B/T^2, Q_B/T^4, P_F/T^2, Q_F/T^4) with
///   P_B = Int d^3k/(2 pi)^3 n_B(w)/w ,      Q_B = T Int d^3k/(2 pi)^3 ln(1 - e^(-w/T)) ,
///   P_F = Int d^3k/(2 pi)^3 n_F(w)/w ,      Q_F = T Int d^3k/(2 pi)^3 ln(1 + e^(-w/T)) ,
/// w = sqrt(k^2 + M^2).  Limits at M = 0: 1/12, -pi^2/90, 1/24, 7 pi^2/720.
pub fn thermal_integrals(a: f64) -> (f64, f64, f64, f64) {
    let g = thermal_grid();
    let (mut pb, mut qb, mut pf, mut qf) = (0.0, 0.0, 0.0, 0.0);
    for (y, wy) in g.x.iter().zip(&g.w) {
        let w = (y * y + a * a).sqrt();
        let meas = wy * y * y / (2.0 * PI * PI);
        let em = (-w).exp();
        pb += meas * em / (w * (1.0 - em));
        qb += meas * (-em).ln_1p();
        pf += meas * em / (w * (1.0 + em));
        qf += meas * em.ln_1p();
    }
    (pb, qb, pf, qf)
}

/// Table of the four thermal integrals at a = 0, h, 2h, ... (h = 0.002, a <= 50),
/// built once on first use; `thermal_fast` interpolates linearly (relative error
/// below 1e-6) and returns zeros beyond the table, where the integrals are below e^-50.
struct ThermalTable {
    h: f64,
    v: Vec<[f64; 4]>,
}

fn thermal_table() -> &'static ThermalTable {
    static TABLE: std::sync::OnceLock<ThermalTable> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let h = 0.002;
        let n = 25001;
        let v = (0..n)
            .map(|i| {
                let (a, b, c, d) = thermal_integrals(i as f64 * h);
                [a, b, c, d]
            })
            .collect();
        ThermalTable { h, v }
    })
}

/// The thermal integrals by interpolation in the table (see `thermal_integrals`).
pub fn thermal_fast(a: f64) -> (f64, f64, f64, f64) {
    let t = thermal_table();
    let x = a.abs() / t.h;
    let i = x.floor() as usize;
    if i + 1 >= t.v.len() {
        return (0.0, 0.0, 0.0, 0.0);
    }
    let f = x - i as f64;
    let (p, q) = (&t.v[i], &t.v[i + 1]);
    (p[0] + f * (q[0] - p[0]), p[1] + f * (q[1] - p[1]), p[2] + f * (q[2] - p[2]), p[3] + f * (q[3] - p[3]))
}

/// Fermions at one loop and the radial and phase modes in the Hartree approximation of
/// the CJT potential, at temperature T and zero net density.
///
/// Variables: phi (the expectation value; the fermion mass is g phi), and the masses
/// M_s, M_th of the two boson propagators.  With the tree-level potential
/// U = (lambda/4)(phi^2 - v^2)^2, the renormalized vacuum loop of the fermions
/// V_sea(g phi) (`v_sea`) and their thermal potential V_F = -2 nu Q_F(g phi),
/// U_eff = U + V_sea + V_F, the potential is
///
///   V = U_eff + Q_B(M_s) + Q_B(M_th) + (1/2)(a_s - M_s^2) P_s + (1/2)(a_th - M_th^2) P_th
///       + (lambda/4)(3 P_s^2 + 3 P_th^2 + 2 P_s P_th),
///   a_s = U_eff''(phi),  a_th = U_eff'(phi)/phi,  P_i = P_B(M_i).
///
/// The last term is the double-bubble diagram of Gamma_2.  Stationarity in the masses
/// gives the gap equations  M_s^2 = a_s + 3 lambda P_s + lambda P_th,
/// M_th^2 = a_th + lambda P_s + 3 lambda P_th.  For the BOSON loops only the thermal
/// parts are kept.  For the FERMION loop the vacuum part must be kept together with
/// the thermal part: each contains a term (g phi)^4 ln(g phi) that is not analytic
/// at phi = 0, and only their sum is analytic (error E27 of the error log records
/// what happens otherwise).
#[derive(Clone, Copy, Debug)]
pub struct Thermal {
    /// Yukawa coupling g = m/v
    pub g: f64,
    /// quartic coupling lambda = m_s^2/(2 v^2)
    pub lambda: f64,
}

impl Thermal {
    /// the vacuum expectation value in units of m
    pub fn v(&self) -> f64 {
        1.0 / self.g
    }

    /// dU_eff/dphi.  The vacuum part of the fermion loop is d V_sea/dphi with
    /// d V_sea/dx = -(nu/(8 pi^2)) x (x^2 ln x^2 - x^2 + 1), x = g phi.
    fn du_eff(&self, phi: f64, t: f64) -> f64 {
        let v = self.v();
        let x = self.g * phi;
        let pf = thermal_fast(x / t).2 * t * t;
        let x2 = x * x;
        let dsea = -(NU / (8.0 * PI * PI)) * x * (if x.abs() > 1e-150 { x2 * x2.ln() } else { 0.0 } - x2 + 1.0);
        self.lambda * phi * (phi * phi - v * v) + self.g * dsea + 2.0 * NU * self.g * self.g * phi * pf
    }

    /// U_eff = tree-level potential + fermion loop (vacuum part V_sea and thermal part).
    fn u_eff(&self, phi: f64, t: f64) -> f64 {
        let v = self.v();
        let x = self.g * phi;
        0.25 * self.lambda * (phi * phi - v * v).powi(2) + v_sea(x.abs()) - 2.0 * NU * thermal_fast(x / t).3 * t.powi(4)
    }

    /// The "tree-level" curvatures (a_s, a_th) of the effective potential of the fermions.
    fn curvatures(&self, phi: f64, t: f64) -> (f64, f64) {
        let h = 1e-4 * self.v();
        if phi < 2.0 * h {
            // symmetric point: both equal U_eff''(0) = -lambda v^2 + 2 nu g^2 P_F(0)
            let a = (self.du_eff(h, t) - self.du_eff(-h, t)) / (2.0 * h);
            return (a, a);
        }
        ((self.du_eff(phi + h, t) - self.du_eff(phi - h, t)) / (2.0 * h), self.du_eff(phi, t) / phi)
    }

    /// Root M^2 >= 0 of  M^2 = a + coef lambda P_B(M) + other  (the left side minus the
    /// right side increases with M^2, because P_B decreases); None if the root would
    /// be negative.
    fn solve_mass2(&self, a: f64, coef: f64, other: f64, t: f64) -> Option<f64> {
        let f = |m2: f64| m2 - a - coef * self.lambda * thermal_fast(m2.sqrt() / t).0 * t * t - other;
        if f(0.0) > 0.0 {
            return None;
        }
        let mut hi = (a.abs() + other.abs() + coef * self.lambda * t * t).max(1e-12);
        while f(hi) < 0.0 {
            hi *= 2.0;
        }
        let mut lo = 0.0;
        for _ in 0..70 {
            let mid = 0.5 * (lo + hi);
            if f(mid) < 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        Some(0.5 * (lo + hi))
    }

    /// Solve the two gap equations at fixed phi (alternating exact solutions of each
    /// equation).  Returns (M_s^2, M_th^2), or None if they have no solution with
    /// non-negative squared masses.
    pub fn masses(&self, phi: f64, t: f64) -> Option<(f64, f64)> {
        let (a_s, a_t) = self.curvatures(phi, t);
        if phi < 2e-4 * self.v() {
            // symmetric point: one equation M^2 = a + 4 lambda P_B(M)
            let m2 = self.solve_mass2(a_s, 4.0, 0.0, t)?;
            return Some((m2, m2));
        }
        let lam_t2 = self.lambda * t * t;
        let mut mt2 = a_t.max(0.0);
        let mut ms2 = a_s.max(0.0);
        for _ in 0..500 {
            let pt = thermal_fast(mt2.sqrt() / t).0;
            let ns = self.solve_mass2(a_s, 3.0, lam_t2 * pt, t)?;
            let ps = thermal_fast(ns.sqrt() / t).0;
            let nt = self.solve_mass2(a_t, 3.0, lam_t2 * ps, t)?;
            let change = (ns - ms2).abs() + (nt - mt2).abs();
            ms2 = ns;
            mt2 = nt;
            if change < 1e-13 * (ms2 + mt2 + t * t) {
                break;
            }
        }
        Some((ms2, mt2))
    }

    /// The potential at fixed phi with the masses at their stationary values.
    pub fn potential(&self, phi: f64, t: f64) -> Option<f64> {
        let (ms2, mt2) = self.masses(phi, t)?;
        let (a_s, a_t) = self.curvatures(phi, t);
        let (ps, qs, _, _) = thermal_fast(ms2.sqrt() / t);
        let (pt, qt, _, _) = thermal_fast(mt2.sqrt() / t);
        let (ps, pt) = (ps * t * t, pt * t * t);
        Some(self.u_eff(phi, t) + (qs + qt) * t.powi(4) + 0.5 * (a_s - ms2) * ps + 0.5 * (a_t - mt2) * pt
            + 0.25 * self.lambda * (3.0 * ps * ps + 3.0 * pt * pt + 2.0 * ps * pt))
    }

    /// The state of lowest potential at temperature t: (phi, potential, M_s^2, M_th^2).
    ///
    /// The potential is defined only where the gap equations have a solution: for
    /// phi >= phi_b(T), with phi_b > 0 below the temperature `t_high` and phi_b = 0
    /// above it.  The function scans this domain, takes the lowest value and refines
    /// it by golden-section search; phi = 0 is returned exactly if it is the lowest
    /// point of the scan.  (Below `t_high` the minimum of the state with broken
    /// symmetry lies above phi_b, with a small but non-zero mass of the phase mode:
    /// the known defect of the Hartree approximation.)
    pub fn global_minimum(&self, t: f64) -> Option<(f64, f64, f64, f64)> {
        let v = self.v();
        let top = 1.05 * v;
        self.potential(top, t)?;
        let phi_b = if self.potential(0.0, t).is_some() {
            0.0
        } else {
            let (mut lo, mut hi) = (0.0, top);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if self.potential(mid, t).is_some() {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            hi
        };
        let n = 400;
        let x_of = |i: usize| phi_b + (top - phi_b) * i as f64 / n as f64;
        let mut best = (f64::MAX, 0usize);
        for i in 0..=n {
            if let Some(val) = self.potential(x_of(i), t) {
                if val < best.0 {
                    best = (val, i);
                }
            }
        }
        let i = best.1;
        let mut phi = x_of(i);
        if !(i == 0 && phi_b == 0.0) {
            let (mut a, mut b) = (x_of(i.saturating_sub(1)), x_of((i + 1).min(n)));
            let r = 0.5 * (5f64.sqrt() - 1.0);
            for _ in 0..60 {
                let (x1, x2) = (b - r * (b - a), a + r * (b - a));
                let (f1, f2) = (self.potential(x1, t).unwrap_or(f64::MAX), self.potential(x2, t).unwrap_or(f64::MAX));
                if f1 < f2 {
                    b = x2;
                } else {
                    a = x1;
                }
            }
            phi = 0.5 * (a + b);
            if self.potential(phi, t).is_none() {
                phi = x_of(i);
            }
        }
        let (ms2, mt2) = self.masses(phi, t)?;
        Some((phi, self.potential(phi, t)?, ms2, mt2))
    }

    /// The melting of the expectation value in the Hartree approximation: the
    /// temperature T_half at which phi of the state of lowest potential falls below
    /// v/2 (bisection between t_lo and t_hi), and the values of phi/v just below and
    /// just above it (at T_half (1 -+ 0.002)).  A difference of the two values that
    /// is of order one means a discontinuous (first-order) transition.
    pub fn melting(&self) -> (f64, f64, f64) {
        let v = self.v();
        let phi = |t: f64| self.global_minimum(t).map(|s| s.0).unwrap_or(0.0);
        let (mut lo, mut hi) = (0.2 * self.t_high(), 3.0 * self.t_high());
        for _ in 0..36 {
            let mid = 0.5 * (lo + hi);
            if phi(mid) > 0.5 * v {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let t_half = 0.5 * (lo + hi);
        (t_half, phi(0.998 * t_half) / v, phi(1.002 * t_half) / v)
    }

    /// Temperature above which the symmetric state exists in the Hartree approximation
    /// (its mass vanishes there); it is also the critical temperature of the one-loop
    /// high-temperature expansion.  The curvature of the potential at phi = 0 is
    /// -lambda v^2 - nu g^2 m^2/(8 pi^2) (tree level and vacuum fermion loop)
    /// + nu g^2 T^2/12 (thermal fermions) + 4 lambda T^2/12 (thermal bosons), so
    ///   T^2 = 12 [lambda v^2 + nu g^2 m^2/(8 pi^2)] / (4 lambda + nu g^2).
    pub fn t_high(&self) -> f64 {
        (12.0 * (self.lambda * self.v().powi(2) + NU * self.g * self.g / (8.0 * PI * PI)) / (4.0 * self.lambda + NU * self.g * self.g)).sqrt()
    }

    /// The same with the boson loops in their large-N form (2 lambda in place of
    /// 4 lambda), in which the phase mode stays massless below the transition.
    pub fn t_large_n(&self) -> f64 {
        (12.0 * (self.lambda * self.v().powi(2) + NU * self.g * self.g / (8.0 * PI * PI)) / (2.0 * self.lambda + NU * self.g * self.g)).sqrt()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    /// Closed forms of the Fermi-sea integrals against quadrature.
    #[test]
    fn fermi_sea_integrals() {
        let gl = gauss_legendre(40);
        for &(kf, x) in &[(0.7, 1.0), (2.0, 0.3), (0.2, 0.9)] {
            let (mut a, mut b) = (0.0, 0.0);
            for (t, w) in gl.0.iter().zip(&gl.1) {
                let k = 0.5 * kf * (t + 1.0);
                let e = (k * k + x * x as f64).sqrt();
                a += 0.5 * kf * w * k * k / (2.0 * PI * PI) / e;
                b += 0.5 * kf * w * k * k / (2.0 * PI * PI) * e;
            }
            assert!((i_scalar(kf, x) - a).abs() < 1e-12 && (e_kinetic(kf, x) - b).abs() < 1e-12);
        }
        // massless limit
        assert!((i_scalar(1.0, 0.0) - 1.0 / (4.0 * PI * PI)).abs() < 1e-15);
        assert!((e_kinetic(1.0, 0.0) - 1.0 / (8.0 * PI * PI)).abs() < 1e-15);
    }

    /// The sea term and its first two derivatives vanish at the vacuum mass; at x = 0
    /// it equals nu/(64 pi^2).
    #[test]
    fn sea_term_renormalization_conditions() {
        let h = 1e-4;
        assert!(v_sea(1.0).abs() < 1e-15);
        assert!(((v_sea(1.0 + h) - v_sea(1.0 - h)) / (2.0 * h)).abs() < 1e-9);
        assert!(((v_sea(1.0 + h) - 2.0 * v_sea(1.0) + v_sea(1.0 - h)) / (h * h)).abs() < 1e-6);
        assert!((v_sea(0.0) - NU / (64.0 * PI * PI)).abs() < 1e-15);
    }

    /// Low density: the mass shift and the energy follow from the scalar density,
    /// x^2 = 1 - 2 C rho_s with rho_s = n to lowest order, so x = 1 - C n; and the
    /// in-medium field equation is satisfied at the minimum.
    #[test]
    fn low_density_limit_and_field_equation() {
        let md = Medium { c: 2.0, sea: false };
        let kf = 0.05;
        let s = md.state(kf);
        assert!((s.x - (1.0 - md.c * s.n)).abs() < 2e-3 * md.c * s.n, "x = {}, n = {}", s.x, s.n);
        // field equation x^2 = 1 - 2 C nu I(k_F, x) at a moderate density
        let s = md.state(0.6);
        assert!((s.x * s.x - (1.0 - 2.0 * md.c * NU * i_scalar(0.6, s.x))).abs() < 1e-6);
        // F(x) is the derivative of the energy divided by x, with and without the sea term
        for sea in [false, true] {
            let md = Medium { c: 2.0, sea };
            let (kf, x, h) = (0.6, 0.7, 1e-5);
            let num = (md.energy(kf, x + h) - md.energy(kf, x - h)) / (2.0 * h);
            assert!((num - x * md.field_equation(kf, x)).abs() < 1e-9);
        }
        // chemical potential = d eps/d n
        let h = 1e-5;
        let (a, b) = (md.state(0.6 - h), md.state(0.6 + h));
        assert!(((b.eps - a.eps) / (b.n - a.n) - s.mu).abs() < 1e-6);
    }

    /// Thresholds for self-bound matter: the abnormal state has the energy per
    /// particle (24 pi^2 B/nu)^(1/4) with B the energy density of the state without
    /// expectation value; it drops below m at C = 3 pi^2/8 without the sea term.
    /// The numerical threshold must not be above this (a bound state with x > 0 at
    /// smaller C is possible), and must equal it if the bound state at threshold is
    /// the abnormal one.
    #[test]
    fn binding_threshold() {
        let c0 = critical_c(false);
        assert!(c0 <= 3.0 * PI * PI / 8.0 + 1e-6, "threshold {}", c0);
        let (e, kf, x) = Medium { c: 1.02 * c0, sea: false }.saturation();
        assert!(e < 1.0 && kf > 0.1);
        if x == 0.0 {
            assert!((c0 - 3.0 * PI * PI / 8.0).abs() < 1e-4, "threshold {}", c0);
        }
        // weak coupling: no bound matter, and the sound speed is that of a free gas
        let md = Medium { c: 0.01, sea: false };
        assert!(md.saturation().0 >= 1.0 - 1e-9);
        let kf: f64 = 0.3;
        let free = kf * kf / (3.0 * (1.0 + kf * kf));
        assert!((md.sound_speed_sq(kf) / free - 1.0).abs() < 0.02);
    }

    /// Exchange energy in the limit of a heavy radial mode (contact interaction):
    /// E_x = (g^2/(2 m_s^2)) (n^2 + rho_s^2)/(2 nu), the Fierz rearrangement of the
    /// scalar interaction.
    #[test]
    fn fock_energy_contact_limit() {
        let (kf, x, g2, ms) = (0.4, 0.9, 1.0, 200.0);
        let (es, _) = fock_energy(kf, x, g2, ms);
        let n = NU * kf.powi(3) / (6.0 * PI * PI);
        let rho_s = NU * x * i_scalar(kf, x);
        let contact = g2 / (2.0 * ms * ms) * (n * n + rho_s * rho_s) / (2.0 * NU);
        assert!((es / contact - 1.0).abs() < 1e-3, "{} vs {}", es, contact);
    }

    /// Thermal integrals at zero mass, and the temperature at which the symmetric
    /// state becomes unstable.
    #[test]
    fn thermal_limits() {
        let (pb, qb, pf, qf) = thermal_integrals(0.0);
        assert!((pb - 1.0 / 12.0).abs() < 1e-7 && (qb + PI * PI / 90.0).abs() < 1e-8);
        assert!((pf - 1.0 / 24.0).abs() < 1e-9 && (qf - 7.0 * PI * PI / 720.0).abs() < 1e-9);
        let th = Thermal { g: 1.0, lambda: 0.5 };
        let t2 = th.t_high();
        // just above t2 the symmetric state exists with a small mass; just below it does not
        let (m2, _) = th.masses(0.0, 1.02 * t2).unwrap();
        assert!(m2 > 0.0 && m2 < 0.2 * t2 * t2);
        assert!(th.masses(0.0, 0.98 * t2).is_none());
        // at low temperature the state of lowest potential is the vacuum
        let (phi, _, ms2, mt2) = th.global_minimum(0.05).unwrap();
        assert!((phi / th.v() - 1.0).abs() < 1e-2 && (ms2 - 2.0 * th.lambda * th.v().powi(2)).abs() < 2e-2 && mt2 < 1e-2);
        // far above the transition it is the symmetric state
        assert_eq!(th.global_minimum(2.0 * t2).unwrap().0, 0.0);
        // the melting temperature lies within 25% of the two closed estimates
        let (t_half, _, _) = th.melting();
        assert!((t_half / t2 - 1.0).abs() < 0.25 && (t_half / th.t_large_n() - 1.0).abs() < 0.25);
    }
}
