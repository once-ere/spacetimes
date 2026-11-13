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
//!          G(x4) = G_0 exp( gsum (x4 - x4_today) ) ,        (dG/dx4)/G = gsum > 0 .
//!
//!  (2) Expansion.  The time-time Einstein equation of the metric (A1) is
//!      sum over pairs of the seven spatial expansion rates (H, H, H, 0, -g55, -g66, -g77):
//!
//!          3 H^2 - 3 gsum H + P = 8 pi G(x4) rho ,     P = g55 g66 + g55 g77 + g66 g77 ,
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
//!  gam = gsum/H_100, p = P/H_100^2; omega_L is fixed by E(a = 1) = h.
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
    /// P / H_100^2
    pub p: f64,
    /// true: assumptions (A1), (A2), gravity feels the deflation (G and the expansion equation change);
    /// false: gravity is the ordinary four-dimensional theory with constant G, and the deflation acts
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
    /// the three rates are taken equal, so that P = gsum^2/3.
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

/// Linear growth of the density contrast of matter in ordinary space, far inside the
/// horizon (Newtonian limit of the eight-dimensional theory for matter that is spread
/// uniformly over the extra coordinates):
///
///     d^2 delta/dx4^2 + 2 H d delta/dx4 = 4 pi G(x4) rho_m delta ,
///
/// that is, with a prime for d/d ln a and omega_m(a) = omega_b + omega_c mass(s),
///
///     delta'' + (2 + E'/E) delta' = (3/2) exp(s) omega_m(a) a^-3 delta / E^2 .
///
/// (For `gravity = false` the factor exp(s) is absent.)  The equation is integrated on
/// the grid of `hist` from the first grid point with a >= a_start to a = 1, with the
/// classical Runge-Kutta method over two grid intervals per step; it starts on the
/// growing solution delta = a + 2 a_eq/3, delta' = a of a universe of matter and
/// radiation.  Returns (delta at a = 1, f = d ln delta/d ln a at a = 1).
pub fn growth<M: Fn(f64) -> f64>(m: &Model, hist: &History, a_start: f64, mass: &M) -> (f64, f64) {
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
        (u, -(2.0 + dlne[i]) * u + 1.5 * g * om / (a * a * a * hist.e[i] * hist.e[i]) * d)
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
    fn growth_without_deflation_has_the_known_values() {
        let (z_star, _, _) = main_project_reference();
        let m = Model::fiducial(0.0, 0.6736);
        let (a_min, n) = grid(z_star, 4000);
        let hist = m.history(a_min, n, &|_s| 1.0);
        let (d, f) = growth(&m, &hist, 0.01, &|_s| 1.0);
        // growth rate today: f = Omega_m^0.55 to about one per cent (Omega_m = 0.3153 with the densities of the model)
        let om = (m.omega_b + m.omega_c) / (m.h * m.h);
        assert!((f / om.powf(0.55) - 1.0).abs() < 0.01, "f = {}, Omega_m^0.55 = {}", f, om.powf(0.55));
        // growth factor today relative to the Einstein-de Sitter law delta = a: about 0.78 for Omega_m = 0.315
        assert!(d > 0.76 && d < 0.81, "D(1) = {}", d);
        // matter era: an Einstein-de Sitter universe (no radiation, no cosmological term) keeps delta = a
        let eds = Model { omega_r: 1e-12, omega_gamma: 1e-12, omega_b: 0.05, omega_c: 0.95, h: 1.0, gam: 0.0, p: 0.0, gravity: true };
        let hist = eds.history(a_min, n, &|_s| 1.0);
        let (d, f) = growth(&eds, &hist, 0.01, &|_s| 1.0);
        assert!((d - 1.0).abs() < 1e-5 && (f - 1.0).abs() < 1e-5, "Einstein-de Sitter: D = {}, f = {}", d, f);
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
