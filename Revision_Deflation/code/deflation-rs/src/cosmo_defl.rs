//! Cosmology on a deflating spacetime: the expansion of ordinary space while x5, x6, x7
//! deflate, the bounds on the rate of deflation, and the Hubble constant.
//!
//! ASSUMPTIONS (stated in the documents wherever a result of this module is used).
//!  (A1) Ordinary space x1, x2, x3 expands with a scale factor a(x4), as observed; the
//!       deflating coordinates have the scale factors b_t = const_t exp(-rate_t x4); x0 is
//!       static.  Metric (7+1 signs): dx4^2 - a^2 (dx1^2 + dx2^2 + dx3^2) - dx0^2 - sum_t b_t^2 dxt^2.
//!  (A2) Gravity is the eight-dimensional Einstein theory with a constant gravitational
//!       coupling G_8.  (The author's programme uses Einstein-Lovelock gravity, which
//!       this project does not implement; the Einstein term is its leading part.)
//!
//! CONSEQUENCES.
//!  (1) Newton's constant.  For matter that is spread uniformly over the extra
//!      coordinates, the four-dimensional constant is G_8 divided by the proper volume of
//!      the extra coordinates, which falls as exp(-gsum x4), gsum = g55 + g66 + g77:
//!
//!          G_N(x4) = G_N(today) exp( gsum (x4 - x4_today) ) ,        (dG_N/dx4)/G_N = gsum > 0 .
//!
//!  (2) Expansion.  The time-time Einstein equation of the metric (A1) is
//!      sum over pairs of the seven spatial expansion rates (H, H, H, 0, -g55, -g66, -g77):
//!
//!          3 H^2 - 3 gsum H + gprod = 8 pi G_N(x4) rho ,     gprod = g55 g66 + g55 g77 + g66 g77 ,
//!
//!      with H = (da/dx4)/a and rho the energy density in ordinary space.  The same
//!      equation holds for the 4+4 signs (check G5 of `geometry.rs`: the Einstein tensor
//!      contains the rates only through such products).
//!
//!  Units: rates in units of H_100 = 100 km/s/Mpc; E = H/H_100; s = gsum (x4 - x4_today) <= 0.
//!  With omega_i = Omega_i h^2 the equation reads
//!
//!          E^2 - gam E + p/3 = exp(s) [ omega_r a^-4 + omega_m(a) a^-3 + omega_L ] ,   ds/dln a = gam / E ,
//!
//!  gam = gsum/H_100, p = gprod/H_100^2; omega_L is fixed by E(a = 1) = h.
//!
//! WHAT IS COMPUTED.  The sound horizon r_s at the redshift z_* of last scattering, the
//! comoving distance D_M to it, their ratio theta_* (the best-measured quantity of the
//! microwave background), the age, and G at early times.  Holding theta_* and the
//! densities omega_b, omega_c fixed, the Hubble constant h is solved for as a function
//! of the rate of deflation.  z_* is held at its value in the model without deflation
//! (taken from the recombination calculation of the main project).
//!
//! A second effect is included on request: dark matter made of bound pairs loses rest
//! mass when the coupling grows, omega_c -> omega_c * mass(s) (module `history`).

use spacetimes::cosmo::{Background, Params, Thermo};

/// 1/H_100 in years.
pub const HUBBLE_TIME_100_YR: f64 = 9.777752e9;

/// The deflating cosmological model.
#[derive(Clone, Copy, Debug)]
pub struct Model {
    pub omega_b: f64,
    pub omega_c: f64,
    /// photons plus massless neutrinos
    pub omega_r: f64,
    pub omega_gamma: f64,
    pub h: f64,
    /// gsum / H_100
    pub gam: f64,
    /// gprod / H_100^2
    pub p: f64,
    /// true: assumptions (A1), (A2), gravity feels the deflation (G and the expansion equation change);
    /// false: gravity is the ordinary four-dimensional theory with constant G_N, and the deflation acts
    /// only on the couplings of the dark sector (the clock s still runs at the rate gam)
    pub gravity: bool,
}

/// The expansion history on a grid in ln a.
pub struct History {
    pub lna: Vec<f64>,
    /// E = H/H_100
    pub e: Vec<f64>,
    /// s = gsum (x4 - x4_today)
    pub s: Vec<f64>,
}

impl Model {
    /// The fiducial densities of the main project with the rate `gam` (units of H_100);
    /// the three rates are taken equal, so that gprod = gsum^2/3.
    pub fn fiducial(gam: f64, h: f64) -> Model {
        let pf = Params::fiducial();
        let og = pf.omega_gamma();
        Model { omega_b: pf.omega_b, omega_c: pf.omega_c, omega_r: og + pf.omega_nu(), omega_gamma: og, h, gam, p: gam * gam / 3.0, gravity: true }
    }

    /// omega_L from the Friedmann equation today (a = 1, s = 0, E = h).
    pub fn omega_lambda<M: Fn(f64) -> f64>(&self, mass: &M) -> f64 {
        let kin = if self.gravity { -self.gam * self.h + self.p / 3.0 } else { 0.0 };
        self.h * self.h + kin - self.omega_r - self.omega_b - self.omega_c * mass(0.0)
    }

    /// E(a, s): the positive root of E^2 - gam E + p/3 = exp(s) [ ... ].
    pub fn e_of<M: Fn(f64) -> f64>(&self, a: f64, s: f64, ol: f64, mass: &M) -> f64 {
        let dens = self.omega_r / a.powi(4) + (self.omega_b + self.omega_c * mass(s)) / a.powi(3) + ol;
        if !self.gravity {
            return dens.max(0.0).sqrt();
        }
        let rhs = s.exp() * dens;
        let disc = 0.25 * self.gam * self.gam - self.p / 3.0 + rhs;
        0.5 * self.gam + disc.max(0.0).sqrt()
    }

    /// Integrate ds/dln a = gam/E backwards from a = 1 to a = a_min (classical Runge-Kutta,
    /// `n` steps).  `mass(s)` is the rest mass of the dark matter relative to today's.
    pub fn history<M: Fn(f64) -> f64>(&self, a_min: f64, n: usize, mass: &M) -> History {
        let ol = self.omega_lambda(mass);
        let l_min = a_min.ln();
        let dl = l_min / n as f64; // negative
        let f = |l: f64, s: f64| self.gam / self.e_of(l.exp(), s, ol, mass);
        let (mut l, mut s) = (0.0_f64, 0.0_f64);
        let mut out = History { lna: vec![l], e: vec![self.e_of(1.0, 0.0, ol, mass)], s: vec![s] };
        for _ in 0..n {
            let k1 = f(l, s);
            let k2 = f(l + 0.5 * dl, s + 0.5 * dl * k1);
            let k3 = f(l + 0.5 * dl, s + 0.5 * dl * k2);
            let k4 = f(l + dl, s + dl * k3);
            s += dl / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
            l += dl;
            out.lna.push(l);
            out.e.push(self.e_of(l.exp(), s, ol, mass));
            out.s.push(s);
        }
        out
    }
}

/// Derived quantities of a history.
#[derive(Clone, Copy, Debug)]
pub struct Derived {
    /// sound horizon at z_* and comoving distance to z_*, in units of c/H_100
    pub rs: f64,
    pub dm: f64,
    /// 100 theta_* = 100 rs/dm
    pub theta100: f64,
    /// age in units of 1/H_100
    pub age: f64,
    /// G at the earliest time of the grid over G today: exp(s_min)
    pub g_early: f64,
    /// G at last scattering over G today
    pub g_star: f64,
}

/// Simpson's rule on a uniform grid (odd number of points).
fn simpson(y: &[f64], h: f64) -> f64 {
    let n = y.len();
    assert!(n >= 3 && n % 2 == 1, "simpson: need an odd number of points");
    let mut s = y[0] + y[n - 1];
    for i in 1..n - 1 {
        s += if i % 2 == 1 { 4.0 * y[i] } else { 2.0 * y[i] };
    }
    s * h / 3.0
}

/// r_s = Int_0^{a_*} c_s da/(a^2 E), D_M = Int_{a_*}^1 da/(a^2 E), age = Int da/(a E),
/// with c_s = 1/sqrt(3 (1 + R)), R = 3 omega_b a/(4 omega_gamma).
/// The grid of `hist` must contain ln a_* as one of its points with an even index.
pub fn derive(m: &Model, hist: &History, z_star: f64) -> Derived {
    let l_star = -(1.0 + z_star).ln();
    let dl = hist.lna[1] - hist.lna[0]; // negative
    let i_star = ((l_star - hist.lna[0]) / dl).round() as usize;
    assert!((hist.lna[i_star] - l_star).abs() < 1e-9 && i_star % 2 == 0, "derive: ln a_* is not an even grid point");
    // D_M: integral over [l_star, 0] of d ln a / (a E)
    let y: Vec<f64> = (0..=i_star).map(|i| 1.0 / (hist.lna[i].exp() * hist.e[i])).collect();
    let dm = simpson(&y, -dl);
    // r_s: integral over (-inf, l_star] of c_s d ln a/(a E).  The grid ends at a_min; below it the universe is
    // dominated by radiation, the integrand is a/sqrt(3 exp(s) omega_r), and its integral from 0 to a_min,
    // a_min/sqrt(3 exp(s) omega_r), is added.
    let n_tot = hist.lna.len() - 1;
    let last = if (n_tot - i_star) % 2 == 0 { n_tot } else { n_tot - 1 };
    let y: Vec<f64> = (i_star..=last)
        .map(|i| {
            let a = hist.lna[i].exp();
            let r = 0.75 * m.omega_b * a / m.omega_gamma;
            1.0 / ((3.0 * (1.0 + r)).sqrt() * a * hist.e[i])
        })
        .collect();
    let a_min = hist.lna[last].exp();
    let gfac = if m.gravity { hist.s[last].exp() } else { 1.0 };
    let tail = a_min / (3.0 * gfac * m.omega_r).sqrt();
    let rs = simpson(&y, -dl) + tail;
    // age: integral of d ln a / E over the whole grid plus the radiation-era remainder a_min^2/(2 sqrt(exp(s) omega_r))
    let y: Vec<f64> = (0..=last).map(|i| 1.0 / hist.e[i]).collect();
    let yy = if y.len() % 2 == 1 { y } else { y[..y.len() - 1].to_vec() };
    let age = simpson(&yy, -dl) + a_min * a_min / (2.0 * (gfac * m.omega_r).sqrt());
    Derived { rs, dm, theta100: 100.0 * rs / dm, age, g_early: hist.s[last].exp(), g_star: hist.s[i_star].exp() }
}

/// The grid used throughout: ln a from 0 down to ln(1e-9), with ln a_* on an even point.
/// Returns (a_min, number of steps): the step is ln(1 + z_*)/n_star with n_star even.
pub fn grid(z_star: f64, n_star: usize) -> (f64, usize) {
    let dl = (1.0 + z_star).ln() / n_star as f64;
    let n = ((1e9f64).ln() / dl).floor() as usize;
    let n = if n % 2 == 0 { n } else { n - 1 };
    ((-(n as f64) * dl).exp(), n)
}

/// 100 theta_* of the model.
pub fn theta_star<M: Fn(f64) -> f64>(m: &Model, z_star: f64, mass: &M) -> Derived {
    let (a_min, n) = grid(z_star, 4000);
    let hist = m.history(a_min, n, mass);
    derive(m, &hist, z_star)
}

/// The Hubble parameter h for which the model has the given 100 theta_*, by bisection.
pub fn solve_h<M: Fn(f64) -> f64>(base: &Model, z_star: f64, theta100: f64, mass: &M) -> f64 {
    let f = |h: f64| theta_star(&Model { h, ..*base }, z_star, mass).theta100 - theta100;
    // theta_* decreases when h increases... determine the bracket from the signs
    let (mut lo, mut hi) = (0.4, 1.0);
    let (flo, fhi) = (f(lo), f(hi));
    assert!(flo * fhi < 0.0, "solve_h: no sign change between h = 0.4 and h = 1.0 ({} and {})", flo, fhi);
    for _ in 0..50 {
        let mid = 0.5 * (lo + hi);
        if f(mid) * flo > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// Which equation the density contrast of matter obeys on the deflating spacetime.
///
/// With a Newton's constant that depends on the time, the Einstein equations of ordinary
/// space cannot all hold with matter alone; which of them hold is the closure of the
/// perturbation equations (assumption A3; module `spectra_defl`).  The growth of structure
/// depends on the closure at the first order in the rate of deflation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GrowthLaw {
    /// closure "Momentum": energy and momentum constraints
    Momentum,
    /// closure "Pressure": energy constraint and trace equation (also the closure "Shear",
    /// energy constraint and equation of the shear, far inside the horizon)
    Pressure,
    /// the equation of four-dimensional Newtonian gravity with Newton's constant of the
    /// moment.  It is NOT a consequence of the eight-dimensional equations with either
    /// closure; an earlier version of this project used it (error R-E33 of the provenance
    /// RP9), and it is kept so that the size of that error can be stated.
    FourDimensional,
}

/// Linear growth of the density contrast delta of pressureless matter in ordinary space,
/// for matter that is spread uniformly over the extra coordinates.  With a prime for
/// d/d ln a, E = H/H_100, r = gam/E, omega_m(a) = omega_b + omega_c mass(s),
///
///     Q = (3/2) exp(s) omega_m(a) a^-3 / E^2 ,    Q_r = 2 exp(s) omega_r a^-4 / E^2 ,    L = E'/E ,
///
/// the three laws are (derived in the provenance RS3 of the revision and, by computer
/// algebra, in code/scripts/verify_deflating_independent.py):
///
///     Pressure:         delta'' + (2 + L - r/2) delta' = Q delta ,
///                       that is  d^2 delta/dx4^2 + (2 H - gsum/2) d delta/dx4 = 4 pi G_N(x4) rho_m delta ;
///     Momentum:         delta'' + { 1 + L + [2 (1 + L) - r + 2 (Q + Q_r)]/(2 - r) } delta' = [(1 - r)/(1 - r/2)] Q delta ;
///     FourDimensional:  delta'' + (2 + L) delta' = Q delta ,
///                       that is  d^2 delta/dx4^2 + 2 H d delta/dx4 = 4 pi G_N(x4) rho_m delta .
///
/// "Pressure" is exact for pressureless matter at every wavenumber; "Momentum" is exact
/// for pressureless matter alone and, with the term Q_r (the momentum of radiation that
/// streams freely), holds far inside the horizon when radiation is present.  Without
/// deflation (r = 0, where L = -Q - Q_r) the three coincide.  For `gravity = false` the
/// factor exp(s) and the terms with r are absent, and the three laws are the same equation.
/// The laws "Momentum" and "Pressure" assume that the rest mass of the matter is constant
/// (`mass` = 1).
///
/// The equation is integrated on the grid of `hist` from the first grid point with
/// a >= a_start to a = 1, with the classical Runge-Kutta method over two grid intervals
/// per step; it starts on the growing solution delta = a + 2 a_eq/3, delta' = a of a
/// universe of matter and radiation.  Returns (delta at a = 1, f = d ln delta/d ln a at a = 1).
pub fn growth<M: Fn(f64) -> f64>(m: &Model, hist: &History, a_start: f64, mass: &M, law: GrowthLaw) -> (f64, f64) {
    let dl = hist.lna[1] - hist.lna[0]; // negative
    let n = hist.lna.len();
    // d ln E/d ln a at every grid point (central differences; one-sided at the ends)
    let dlne: Vec<f64> = (0..n)
        .map(|i| {
            let (a, b) = (if i == 0 { 0 } else { i - 1 }, if i + 1 == n { i } else { i + 1 });
            (hist.e[b].ln() - hist.e[a].ln()) / (hist.lna[b] - hist.lna[a])
        })
        .collect();
    let mut i0 = ((a_start.ln() - hist.lna[0]) / dl).floor() as usize;
    if i0 % 2 == 1 {
        i0 -= 1;
    }
    assert!(i0 >= 2 && i0 < n);
    let rhs = |i: usize, d: f64, u: f64| -> (f64, f64) {
        let a = hist.lna[i].exp();
        let om = m.omega_b + m.omega_c * mass(hist.s[i]);
        let g = if m.gravity { hist.s[i].exp() } else { 1.0 };
        let e2 = hist.e[i] * hist.e[i];
        let q = 1.5 * g * om / (a * a * a * e2);
        let r = if m.gravity { m.gam / hist.e[i] } else { 0.0 };
        let l = dlne[i];
        match law {
            GrowthLaw::FourDimensional => (u, -(2.0 + l) * u + q * d),
            GrowthLaw::Pressure => (u, -(2.0 + l - 0.5 * r) * u + q * d),
            GrowthLaw::Momentum => {
                let q_r = 2.0 * g * m.omega_r / (a * a * a * a * e2);
                let friction = 1.0 + l + (2.0 * (1.0 + l) - r + 2.0 * (q + q_r)) / (2.0 - r);
                (u, -friction * u + (1.0 - r) / (1.0 - 0.5 * r) * q * d)
            }
        }
    };
    let a0 = hist.lna[i0].exp();
    let a_eq = m.omega_r / (m.omega_b + m.omega_c * mass(hist.s[i0]));
    let (mut d, mut u) = (a0 + 2.0 * a_eq / 3.0, a0);
    let h = -2.0 * dl; // positive step in ln a
    let mut i = i0;
    while i >= 2 {
        let (k1d, k1u) = rhs(i, d, u);
        let (k2d, k2u) = rhs(i - 1, d + 0.5 * h * k1d, u + 0.5 * h * k1u);
        let (k3d, k3u) = rhs(i - 1, d + 0.5 * h * k2d, u + 0.5 * h * k2u);
        let (k4d, k4u) = rhs(i - 2, d + h * k3d, u + h * k3u);
        d += h / 6.0 * (k1d + 2.0 * k2d + 2.0 * k3d + k4d);
        u += h / 6.0 * (k1u + 2.0 * k2u + 2.0 * k3u + k4u);
        i -= 2;
    }
    (d, u / d)
}

/// The largest present total rates of deflation that the observations of the cosmological
/// part allow, and the age of the model without deflation.  The commands `cosmology` and
/// `matter` both take them from here, so that the two cannot disagree.  Rates are in units
/// of H_100 = 100 km/s/Mpc (divide by `h` for units of the Hubble constant of the model).
///
/// * Lunar laser ranging: (dG_N/dt)/G_N = (7.1 +- 7.6) e-14 per year (68 per cent; Hofmann
///   and Mueller 2018).  Under assumption A2, (dG_N/dt)/G_N = gsum; the two-sigma upper value
///   is (7.1 + 2 * 7.6) e-14 per year.
/// * Nucleosynthesis and Newton's constant: G_N(nucleosynthesis)/G_N(today) = 0.99 +0.06 -0.05
///   at two sigma (Alvey et al. 2020), hence >= 0.94.  Under A2 and for a constant rate the
///   ratio is exp(-gsum t_0), with the age t_0 of the deflating model itself (Hubble constant
///   held at `h`); the rate at which it equals 0.94 is found by bisection.
/// * Nucleosynthesis and the Fermi constant (assumption HV: the vacuum value of the
///   Vorton-Scalar sets the electroweak scale; NOT the author's hypothesis H2): with all
///   masses fixed, G_Fermi at t = 1 s within 20 (10) per cent of today's value (Scherrer and
///   Spergel 1993, their case (b)); G_Fermi grows at the rate gsum, so that
///   gsum t_0 < ln(1/0.8) (ln(1/0.9)), with the age of the model without deflation (no
///   assumption about gravity is made here).
pub struct RateBounds {
    /// lunar laser ranging, two-sigma upper value of the rate, per year
    pub llr_per_yr: f64,
    /// the same in units of H_100
    pub llr: f64,
    /// nucleosynthesis and Newton's constant, in units of H_100
    pub bbn: f64,
    /// nucleosynthesis and the Fermi constant, 20 per cent, in units of H_100
    pub gfermi_20: f64,
    /// the same for 10 per cent
    pub gfermi_10: f64,
    /// age of the model without deflation, in units of 1/H_100
    pub age: f64,
}

/// The bounds for the fiducial model with the Hubble constant `h` (see `RateBounds`).
pub fn rate_bounds(h: f64) -> RateBounds {
    let (z_star, _, _) = main_project_reference();
    let none = |_s: f64| 1.0;
    let age = theta_star(&Model::fiducial(0.0, h), z_star, &none).age;
    let llr_per_yr = (7.1 + 2.0 * 7.6) * 1e-14;
    // bisection for exp(-gsum t_0) = 0.94
    let (mut lo, mut hi) = (0.0_f64, 1.0_f64);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if theta_star(&Model::fiducial(mid, h), z_star, &none).g_early > 0.94 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    RateBounds {
        llr_per_yr,
        llr: llr_per_yr * HUBBLE_TIME_100_YR,
        bbn: 0.5 * (lo + hi),
        gfermi_20: (1.0_f64 / 0.8).ln() / age,
        gfermi_10: (1.0_f64 / 0.9).ln() / age,
        age,
    }
}

/// z_* and 100 theta_* of the fiducial model of the main project (its recombination
/// calculation and its own distance integrals), for comparison.
pub fn main_project_reference() -> (f64, f64, f64) {
    let p = Params::fiducial();
    let bg = Background::new(&p);
    let th = Thermo::new(&bg);
    (th.z_star, th.theta_star_100, th.rs_star)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn without_deflation_the_integrals_agree_with_the_main_project() {
        let (z_star, theta_main, rs_main) = main_project_reference();
        let m = Model::fiducial(0.0, 0.6736);
        let d = theta_star(&m, z_star, &|_s| 1.0);
        // same physics (massless neutrinos, same densities): agreement to 2e-4
        assert!((d.theta100 / theta_main - 1.0).abs() < 2e-4, "100 theta_*: {} here, {} in the main project", d.theta100, theta_main);
        // sound horizon in Mpc: r_s (c/H_100) = r_s * 2997.92458 Mpc
        assert!((d.rs * 2997.92458 / rs_main - 1.0).abs() < 2e-4, "r_s: {} Mpc here, {} Mpc in the main project", d.rs * 2997.92458, rs_main);
        // age of the fiducial model: 13.8 Gyr
        let age_gyr = d.age * HUBBLE_TIME_100_YR / 1e9;
        assert!((age_gyr - 13.8).abs() < 0.1, "age {} Gyr", age_gyr);
        assert!((d.g_early - 1.0).abs() < 1e-15);
    }

    #[test]
    fn the_bounds_on_the_rate_follow_from_their_definitions() {
        let h = 0.6736;
        let rb = rate_bounds(h);
        // lunar laser ranging: 22.3e-14 per year, in units of H_100
        assert!((rb.llr_per_yr - 22.3e-14).abs() < 1e-18);
        assert!((rb.llr - 22.3e-14 * HUBBLE_TIME_100_YR).abs() < 1e-15);
        // nucleosynthesis: at the bound, Newton's constant in the early universe is 0.94 of today's value
        let (z_star, _, _) = main_project_reference();
        let d = theta_star(&Model::fiducial(rb.bbn, h), z_star, &|_s| 1.0);
        assert!((d.g_early - 0.94).abs() < 1e-9, "G_N ratio at the bound: {}", d.g_early);
        // and exp(-gsum t_0) with the age of that model
        assert!(((-rb.bbn * d.age).exp() - 0.94).abs() < 1e-6);
        // Fermi constant: gsum t_0 = ln(1/0.8) and ln(1/0.9) with the age of the model without deflation
        assert!(((rb.gfermi_20 * rb.age).exp() - 1.25).abs() < 1e-12);
        assert!(((rb.gfermi_10 * rb.age).exp() - 1.0 / 0.9).abs() < 1e-12);
        // order of the bounds, and the age: H_0 t_0 = 0.95 for this model
        assert!(rb.llr < rb.bbn && rb.bbn < rb.gfermi_10 && rb.gfermi_10 < rb.gfermi_20);
        assert!((rb.age * h - 0.9516).abs() < 5e-4, "H_0 t_0 = {}", rb.age * h);
    }

    #[test]
    fn growth_without_deflation_has_the_known_values() {
        let (z_star, _, _) = main_project_reference();
        let m = Model::fiducial(0.0, 0.6736);
        let (a_min, n) = grid(z_star, 4000);
        let hist = m.history(a_min, n, &|_s| 1.0);
        let (d, f) = growth(&m, &hist, 0.01, &|_s| 1.0, GrowthLaw::Pressure);
        // growth rate today: f = Omega_m^0.55 to about one per cent (Omega_m = 0.3153 with the densities of the model)
        let om = (m.omega_b + m.omega_c) / (m.h * m.h);
        assert!((f / om.powf(0.55) - 1.0).abs() < 0.01, "f = {}, Omega_m^0.55 = {}", f, om.powf(0.55));
        // growth factor today relative to the Einstein-de Sitter law delta = a: about 0.78 for Omega_m = 0.315
        assert!(d > 0.76 && d < 0.81, "D(1) = {}", d);
        // without deflation the three laws are the same equation (the law "Momentum" is written with
        // L = E'/E taken from the tabulated history, and agrees to the accuracy of that derivative)
        let (d4, f4) = growth(&m, &hist, 0.01, &|_s| 1.0, GrowthLaw::FourDimensional);
        assert!(d4.to_bits() == d.to_bits() && f4.to_bits() == f.to_bits());
        let (dm, fm) = growth(&m, &hist, 0.01, &|_s| 1.0, GrowthLaw::Momentum);
        assert!((dm / d - 1.0).abs() < 1e-6 && (fm / f - 1.0).abs() < 1e-6, "Momentum without deflation: D {} vs {}, f {} vs {}", dm, d, fm, f);
        // matter era: an Einstein-de Sitter universe (no radiation, no cosmological term) keeps delta = a
        let eds = Model { omega_r: 1e-12, omega_gamma: 1e-12, omega_b: 0.05, omega_c: 0.95, h: 1.0, gam: 0.0, p: 0.0, gravity: true };
        let hist = eds.history(a_min, n, &|_s| 1.0);
        for law in [GrowthLaw::Momentum, GrowthLaw::Pressure, GrowthLaw::FourDimensional] {
            let (d, f) = growth(&eds, &hist, 0.01, &|_s| 1.0, law);
            assert!((d - 1.0).abs() < 1e-5 && (f - 1.0).abs() < 1e-5, "Einstein-de Sitter, {:?}: D = {}, f = {}", law, d, f);
        }
    }

    /// With deflation and pressureless matter alone the three laws can be solved to the first order
    /// in r = gam/E.  For three equal rates the expansion obeys E^2 - gam E + gam^2/9 = exp(s) omega_m a^-3,
    /// so that Q = (3/2)(1 - r), L = E'/E = -3/2 + (5/4) r and r' = (3/2) r to this order; the growing
    /// solution is delta = a (1 + beta r) with 6 beta = S_1 - F_1, where F_1 and S_1 are the coefficients
    /// of r in the friction term and in the source term of the law.  The growth rate is then
    ///     d ln delta/d ln a = 1 + c r ,    c = (S_1 - F_1)/4 = -9/16 ("Pressure"), -13/16 ("Momentum"), -11/16 (four-dimensional)
    /// (derived again by computer algebra in code/scripts/verify_deflating_independent.py).
    #[test]
    fn growth_with_deflation_follows_the_first_order_laws() {
        let (z_star, _, _) = main_project_reference();
        let (a_min, n) = grid(z_star, 4000);
        // matter only (the Hubble constant is the one for which the cosmological term vanishes);
        // a rate that makes r = gam/E about 1e-3 at a = 0.05
        let gam: f64 = 0.1;
        let h = 0.5 * gam + (0.25 * gam * gam - gam * gam / 9.0 + 1.0).sqrt();
        let mdl = Model { omega_r: 1e-14, omega_gamma: 1e-14, omega_b: 0.05, omega_c: 0.95, h, gam, p: gam * gam / 3.0, gravity: true };
        assert!(mdl.omega_lambda(&|_s| 1.0).abs() < 1e-12);
        let hist = mdl.history(a_min, n, &|_s| 1.0);
        for (law, c) in [(GrowthLaw::Pressure, -9.0 / 16.0), (GrowthLaw::Momentum, -13.0 / 16.0), (GrowthLaw::FourDimensional, -11.0 / 16.0)] {
            // integrate from a = 5e-4 to a = 0.05 and read the growth rate there
            let cut = |a_end: f64| -> History {
                // the history from a = a_end downwards, relabelled so that the integration ends there
                let i_end = ((a_end.ln() - hist.lna[0]) / (hist.lna[1] - hist.lna[0])).round() as usize;
                let i_end = if i_end % 2 == 0 { i_end } else { i_end + 1 };
                History { lna: hist.lna[i_end..].to_vec(), e: hist.e[i_end..].to_vec(), s: hist.s[i_end..].to_vec() }
            };
            let h1 = cut(0.05);
            let (_, f) = growth(&mdl, &h1, 5e-4, &|_s| 1.0, law);
            let r = gam / h1.e[0];
            println!("{:?}: (f - 1)/r = {:.5} at r = {:.3e}; first order: {:.5}", law, (f - 1.0) / r, r, c);
            assert!(((f - 1.0) / r - c).abs() < 2e-3, "{:?}: (f - 1)/r = {} at r = {}, expected {}", law, (f - 1.0) / r, r, c);
        }
    }

    #[test]
    fn solving_for_h_returns_the_input() {
        let (z_star, _, _) = main_project_reference();
        let m = Model::fiducial(0.0, 0.6736);
        let d = theta_star(&m, z_star, &|_s| 1.0);
        let h = solve_h(&m, z_star, d.theta100, &|_s| 1.0);
        assert!((h - 0.6736).abs() < 1e-7);
    }

    #[test]
    fn deflation_makes_gravity_weaker_in_the_past_and_lowers_the_inferred_hubble_constant() {
        let (z_star, _, _) = main_project_reference();
        let m0 = Model::fiducial(0.0, 0.6736);
        let theta = theta_star(&m0, z_star, &|_s| 1.0).theta100;
        // gsum = 0.01 H_0
        let gam = 0.01 * 0.6736;
        let m1 = Model::fiducial(gam, 0.6736);
        let d1 = theta_star(&m1, z_star, &|_s| 1.0);
        // G at early times: exp(-gsum t_0) with t_0 close to 0.95/H_0
        assert!(d1.g_early < 1.0 && (d1.g_early.ln() + gam * d1.age).abs() < 1e-6);
        let h1 = solve_h(&m1, z_star, theta, &|_s| 1.0);
        assert!(h1 < 0.6736, "h = {}", h1);
    }
}
