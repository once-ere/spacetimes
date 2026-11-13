//! Homogeneous cosmology on a deflating spacetime: expansion, recombination, thermodynamic tables.
//!
//! DO NOT EDIT.  This file is written by code/scripts/vendor_eb.py of the revision.  It is the code of
//! the structures `Background` and `Thermo` of cosmo.rs of the main project (code/spacetimes-rs/src) with the following changes;
//! every other character is the original.  The main project itself is not changed.
//!   * `Background` carries the rates of deflation (`Deflation`) and a table of s = gsum (x4 - x4_today);
//!   * the Hubble rate is the positive root of E^2 - gam E + gprod/3 = exp(s) [sum of the densities];
//!   * the cosmological term is fixed by this equation today;
//!   * new functions `e_of`, `s_of_a`, `s_early`, `age`, `helium_fraction` (Newton's constant at nucleosynthesis);
//!   * `Thermo::new` takes the helium fraction from `Background::helium_fraction`.
//! See the description at the head of vendor_eb.py and the provenance RP8 of the revision.

use spacetimes::cosmo::{neutrino_factor, Params, StepTable, C_LIGHT, HUBBLE_LENGTH, H_PLANCK, K_BOLTZ, MPC, M_ELECTRON, M_HYDROGEN, NOT4, SIGMA_THOMSON, G_NEWTON};
use spacetimes::quad::gauss_legendre;

/// Which Einstein equation of ordinary space the perturbations obey besides the energy constraint
/// (assumption A3; the equations are in `eb_boltzmann`).  With a Newton's constant that depends on
/// the time they cannot all hold with matter alone: the agent that holds the scale factors of the
/// extra coordinates supplies the difference, and the closure says which perturbations it carries.
/// In both closures the agent carries no energy density (the energy constraint holds).
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Closure {
    /// the momentum constraint holds: the agent carries no momentum density
    Momentum,
    /// the trace equation holds: the agent carries no pressure perturbation
    Pressure,
}

/// The deflation, as far as the cosmological calculation needs it.
///
/// * `gam` = gsum = g55 + g66 + g77 in units of H_100 = 100 km/s/Mpc;
/// * `gprod` = g55 g66 + g55 g77 + g66 g77 in units of H_100^2 (gam^2/3 for three equal rates);
/// * `closure`: which Einstein equations of ordinary space the perturbations obey (assumption A3);
/// * `bbn`: whether the helium fraction responds to Newton's constant at nucleosynthesis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Deflation {
    pub gam: f64,
    pub gprod: f64,
    pub closure: Closure,
    pub bbn: bool,
}

impl Deflation {
    /// No deflation: the model of the main project.
    pub fn none() -> Deflation {
        Deflation { gam: 0.0, gprod: 0.0, closure: Closure::Momentum, bbn: true }
    }

    /// Three equal rates with the total rate `gam` (units of H_100).
    pub fn equal_rates(gam: f64) -> Deflation {
        Deflation { gam, gprod: gam * gam / 3.0, closure: Closure::Momentum, bbn: true }
    }
}

// ---- background ----------------------------------------------------------------------

/// The homogeneous background of a parameter set.
#[derive(Clone, Debug)]
pub struct Background {
    pub p: Params,
    /// the deflation: rates and closure (see `Deflation`)
    pub d: Deflation,
    pub omega_gamma: f64,
    pub omega_nu: f64,
    /// dark-fluid density today (as if it were radiation), omega_d = n_ir * (7/8)(4/11)^(4/3) omega_gamma
    pub omega_d: f64,
    /// density of cold collisionless dark matter (zero if the dark matter is a superfluid)
    pub omega_cdm: f64,
    pub omega_lambda: f64,
    pub step: Option<StepTable>,
    /// conformal time today, Mpc
    pub tau0: f64,
    // table of conformal time versus ln a
    tab_lna: Vec<f64>,
    tab_tau: Vec<f64>,
    /// s = gsum (x4 - x4_today) on the same grid, and E = H/H_100 there
    tab_s: Vec<f64>,
    tab_e: Vec<f64>,
}

impl Background {
    pub fn new(p: &Params, d: &Deflation) -> Background {
        let omega_gamma = p.omega_gamma();
        let omega_nu = p.omega_nu();
        let omega_d = p.dark.n_ir * neutrino_factor() * omega_gamma;
        let step = p.dark.step.as_ref().map(StepTable::new);
        // the equation of the expansion today (a = 1, s = 0, E = h):  h^2 - gam h + gprod/3 = sum of the densities
        let omega_lambda = p.h * p.h - d.gam * p.h + d.gprod / 3.0 - p.omega_b - p.omega_c - omega_gamma - omega_nu - omega_d;
        // the code has one slot for a dark fluid: dark radiation OR superfluid dark matter
        assert!(p.superfluid_dm.is_none() || p.dark.n_ir == 0.0, "dark radiation and superfluid dark matter cannot be combined");
        let omega_cdm = if p.superfluid_dm.is_some() { 0.0 } else { p.omega_c };
        let mut bg = Background { p: *p, d: *d, omega_gamma, omega_nu, omega_d, omega_cdm, omega_lambda, step, tau0: 0.0, tab_lna: Vec::new(), tab_tau: Vec::new(), tab_s: Vec::new(), tab_e: Vec::new() };
        // conformal time tau(a) = Int_0^a da / (a^2 H): tabulate on a grid in ln a
        let gl = gauss_legendre(8);
        let n = 2400;
        let (l0, l1) = ((1e-9f64).ln(), 0.0);
        // s(a): integrate ds/dln a = gam/E backwards from today (s = 0) on the grid of the table
        // (classical Runge-Kutta, eight steps per interval); with gam = 0 it is identically zero
        bg.tab_s = vec![0.0; n + 1];
        bg.tab_e = vec![0.0; n + 1];
        {
            let f = |l: f64, s: f64| d.gam / bg.e_of(l.exp(), s);
            let sub = 8;
            let mut s = 0.0;
            let mut l = l1;
            let dl = -(l1 - l0) / (n * sub) as f64;
            let mut tab = vec![0.0; n + 1];
            for i in (0..n).rev() {
                for _ in 0..sub {
                    let k1 = f(l, s);
                    let k2 = f(l + 0.5 * dl, s + 0.5 * dl * k1);
                    let k3 = f(l + 0.5 * dl, s + 0.5 * dl * k2);
                    let k4 = f(l + dl, s + dl * k3);
                    s += dl / 6.0 * (k1 + 2.0 * k2 + 2.0 * k3 + k4);
                    l += dl;
                }
                tab[i] = s;
            }
            bg.tab_s = tab;
        }
        for i in 0..=n {
            let a = (l0 + (l1 - l0) * i as f64 / n as f64).exp();
            bg.tab_e[i] = bg.e_of(a, bg.tab_s[i]);
        }
        // tau(a_min) from the radiation-dominated law tau = a / (a^2 H)|_rad
        let a0 = l0.exp();
        let mut tau = 1.0 / (a0 * bg.hubble(a0));
        bg.tab_lna.push(l0);
        bg.tab_tau.push(tau);
        for i in 0..n {
            let (la, lb) = (l0 + (l1 - l0) * i as f64 / n as f64, l0 + (l1 - l0) * (i + 1) as f64 / n as f64);
            let (hh, cc) = (0.5 * (lb - la), 0.5 * (lb + la));
            for (t, w) in gl.0.iter().zip(&gl.1) {
                let a = (cc + hh * t).exp();
                tau += hh * w / (a * bg.hubble(a)); // d tau = d ln a / (a H)
            }
            bg.tab_lna.push(lb);
            bg.tab_tau.push(tau);
        }
        bg.tau0 = tau;
        bg
    }

    /// Dark-fluid quantities at scale factor a: (omega_d(a) a^4, w, c_s^2).
    pub fn dark(&self, a: f64) -> (f64, f64, f64) {
        if let Some(sf) = &self.p.superfluid_dm {
            // superfluid dark matter: rho a^4 = omega_c a (rho a^3 / rho_0)
            let (ratio, w, cs2) = sf.state(a);
            return (self.p.omega_c * a * ratio, w, cs2);
        }
        match &self.step {
            None => (self.omega_d, 1.0 / 3.0, 1.0 / 3.0),
            Some(t) => {
                let (r, w, cs2) = t.at(a);
                (self.omega_d * r, w, cs2)
            }
        }
    }

    /// Hubble rate H(a) in 1/Mpc.
    pub fn hubble(&self, a: f64) -> f64 {
        self.e_of(a, self.s_of_a(a)) / HUBBLE_LENGTH
    }

    /// E = H/H_100 at the scale factor a and the value s of the clock: the positive root of
    /// E^2 - gam E + gprod/3 = exp(s) [sum of the densities].  With gam = gprod = 0 and s = 0
    /// the arithmetic is that of the main project (the additional terms are exact zeros).
    pub fn e_of(&self, a: f64, s: f64) -> f64 {
        let (od, _, _) = self.dark(a);
        let e2 = (self.omega_gamma + self.omega_nu + od) / a.powi(4) + (self.p.omega_b + self.omega_cdm) / a.powi(3) + self.omega_lambda;
        0.5 * self.d.gam + (0.25 * self.d.gam * self.d.gam - self.d.gprod / 3.0 + s.exp() * e2).sqrt()
    }

    /// s = gsum (x4 - x4_today) at the scale factor a (cubic Hermite interpolation in ln a with
    /// ds/dln a = gam/E; constant before the first point of the table, where the time since the
    /// beginning is negligible).  exp(s) is Newton's constant divided by its value today.
    pub fn s_of_a(&self, a: f64) -> f64 {
        let n = self.tab_lna_len();
        let la = a.ln();
        let (l0, l1) = ((1e-9f64).ln(), 0.0);
        if la <= l0 {
            return self.tab_s[0];
        }
        if la >= l1 {
            return self.tab_s[n - 1] + self.d.gam / self.tab_e[n - 1] * (la - l1);
        }
        let h = (l1 - l0) / (n - 1) as f64;
        let i = (((la - l0) / h) as usize).min(n - 2);
        let t = (la - (l0 + h * i as f64)) / h;
        let (y0, y1) = (self.tab_s[i], self.tab_s[i + 1]);
        let (d0, d1) = (h * self.d.gam / self.tab_e[i], h * self.d.gam / self.tab_e[i + 1]);
        let t2 = t * t;
        let t3 = t2 * t;
        (2.0 * t3 - 3.0 * t2 + 1.0) * y0 + (t3 - 2.0 * t2 + t) * d0 + (-2.0 * t3 + 3.0 * t2) * y1 + (t3 - t2) * d1
    }

    fn tab_lna_len(&self) -> usize {
        self.tab_s.len()
    }

    /// s at the earliest time of the table: ln of Newton's constant in the early universe over today's.
    pub fn s_early(&self) -> f64 {
        self.tab_s[0]
    }

    /// The age of the universe in units of 1/H_100:  Int_0^1 da / (a E).
    pub fn age(&self) -> f64 {
        let gl = gauss_legendre(8);
        let n = 2400;
        let (l0, l1) = ((1e-9f64).ln(), 0.0);
        let a0 = l0.exp();
        // before the table: radiation era, t = 1/(2H)
        let mut t = 0.5 / (self.hubble(a0) * HUBBLE_LENGTH);
        for i in 0..n {
            let (la, lb) = (l0 + (l1 - l0) * i as f64 / n as f64, l0 + (l1 - l0) * (i + 1) as f64 / n as f64);
            let (hh, cc) = (0.5 * (lb - la), 0.5 * (lb + la));
            for (x, w) in gl.0.iter().zip(&gl.1) {
                let a = (cc + hh * x).exp();
                t += hh * w / (self.hubble(a) * HUBBLE_LENGTH);
            }
        }
        t
    }

    /// The helium mass fraction.  Newton's constant at nucleosynthesis was exp(s_early) times
    /// today's.  The square of the expansion rate at a given temperature is proportional to
    /// Newton's constant times (43/7 + dn), where dn is the radiation in excess of the standard
    /// content in units of one neutrino species; the smaller Newton's constant is therefore
    /// equivalent to (43/7 + dn)(exp(s_early) - 1) additional species (a negative number).  The
    /// difference is added to the value of the main project with the same fit; with gam = 0 it
    /// is an exact zero.
    pub fn helium_fraction(&self) -> f64 {
        let y = self.p.helium_fraction();
        if self.p.y_he.is_some() || !self.d.bbn {
            return y;
        }
        let fit = |ob: f64, dn: f64| {
            0.2311 + 0.9502 * ob - 11.27 * ob * ob + dn * (0.01356 + 0.008581 * ob - 0.1810 * ob * ob) + dn * dn * (-0.0009795 - 0.001370 * ob + 0.01746 * ob * ob)
        };
        let dn = self.p.n_ur - 3.044 + self.p.dark_n_uv();
        let dn_g = (43.0 / 7.0 + dn) * (self.s_early().exp() - 1.0);
        y + (fit(self.p.omega_b, dn + dn_g) - fit(self.p.omega_b, dn))
    }

    /// Conformal Hubble rate a H = a'/a in 1/Mpc.
    pub fn conformal_hubble(&self, a: f64) -> f64 {
        a * self.hubble(a)
    }

    /// Conformal time tau(a) in Mpc (cubic interpolation in ln a using d tau/d ln a = 1/(aH)).
    pub fn tau_of_a(&self, a: f64) -> f64 {
        let la = a.ln();
        let n = self.tab_lna.len();
        if la <= self.tab_lna[0] {
            return 1.0 / (a * self.hubble(a));
        }
        let h = (self.tab_lna[n - 1] - self.tab_lna[0]) / (n - 1) as f64;
        let i = (((la - self.tab_lna[0]) / h) as usize).min(n - 2);
        let t = (la - self.tab_lna[i]) / h;
        let (a0, a1) = (self.tab_lna[i].exp(), self.tab_lna[i + 1].exp());
        let (d0, d1) = (h / (a0 * self.hubble(a0)), h / (a1 * self.hubble(a1)));
        let (y0, y1) = (self.tab_tau[i], self.tab_tau[i + 1]);
        // cubic Hermite
        let t2 = t * t;
        let t3 = t2 * t;
        (2.0 * t3 - 3.0 * t2 + 1.0) * y0 + (t3 - 2.0 * t2 + t) * d0 + (-2.0 * t3 + 3.0 * t2) * y1 + (t3 - t2) * d1
    }

    /// Scale factor at conformal time tau (bisection on the monotonic table, then Newton).
    pub fn a_of_tau(&self, tau: f64) -> f64 {
        let n = self.tab_tau.len();
        if tau <= self.tab_tau[0] {
            // radiation era: tau proportional to a
            return self.tab_lna[0].exp() * tau / self.tab_tau[0];
        }
        if tau >= self.tau0 {
            return 1.0;
        }
        let (mut lo, mut hi) = (0, n - 1);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.tab_tau[mid] <= tau {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let mut la = self.tab_lna[lo] + (self.tab_lna[hi] - self.tab_lna[lo]) * (tau - self.tab_tau[lo]) / (self.tab_tau[hi] - self.tab_tau[lo]);
        for _ in 0..4 {
            let a = la.exp();
            let f = self.tau_of_a(a) - tau;
            la -= f * a * self.hubble(a); // d tau / d ln a = 1/(aH)
        }
        la.exp()
    }

    /// Comoving sound horizon of the photon-baryon fluid at scale factor a, in Mpc:
    /// r_s = Int_0^a c_s da / (a^2 H),  c_s = 1/sqrt(3 (1 + R)),  R = 3 rho_b / (4 rho_gamma).
    pub fn sound_horizon(&self, a: f64) -> f64 {
        let gl = gauss_legendre(8);
        let n = 600;
        let (l0, l1) = ((1e-9f64).ln(), a.ln());
        let cs = |a: f64| 1.0 / (3.0 * (1.0 + 0.75 * self.p.omega_b * a / self.omega_gamma)).sqrt();
        let a0 = l0.exp();
        let mut r = cs(a0) / (a0 * self.hubble(a0));
        for i in 0..n {
            let (la, lb) = (l0 + (l1 - l0) * i as f64 / n as f64, l0 + (l1 - l0) * (i + 1) as f64 / n as f64);
            let (hh, cc) = (0.5 * (lb - la), 0.5 * (lb + la));
            for (t, w) in gl.0.iter().zip(&gl.1) {
                let x = (cc + hh * t).exp();
                r += hh * w * cs(x) / (x * self.hubble(x));
            }
        }
        r
    }

    /// Scale factor of matter-radiation equality (all radiation, including the dark fluid).
    pub fn a_eq(&self) -> f64 {
        let om = self.p.omega_b + self.p.omega_c;
        let mut a = (self.omega_gamma + self.omega_nu + self.omega_d) / om;
        if self.p.superfluid_dm.is_some() {
            return a; // (the superfluid counts as matter here)
        }
        for _ in 0..20 {
            let (od, _, _) = self.dark(a);
            a = (self.omega_gamma + self.omega_nu + od) / om;
        }
        a
    }
}

// ---- recombination and reionization -----------------------------------------------------

/// Thermal history: free-electron fraction, optical depth, visibility, baryon sound speed.
#[derive(Clone, Debug)]
pub struct Thermo {
    /// redshift grid (descending from z_start to 0, uniform spacing)
    pub z: Vec<f64>,
    /// free electrons per hydrogen nucleus
    pub xe: Vec<f64>,
    /// baryon temperature in K
    pub tb: Vec<f64>,
    /// conformal time at each z
    pub tau: Vec<f64>,
    /// scattering rate kappa' = a n_e sigma_T in 1/Mpc
    pub dkappa: Vec<f64>,
    /// optical depth from tau to today
    pub kappa: Vec<f64>,
    /// baryon sound speed squared (units of c^2)
    pub cb2: Vec<f64>,
    pub y_he: f64,
    pub z_reio: f64,
    /// redshift at which the optical depth (without reionization) is 1
    pub z_star: f64,
    /// redshift of the maximum of the visibility function
    pub z_rec: f64,
    /// redshift at which the baryon drag depth is 1
    pub z_drag: f64,
    /// sound horizons at these redshifts (Mpc)
    pub rs_star: f64,
    pub rs_rec: f64,
    pub rs_drag: f64,
    /// 100 * r_s(z_rec) / D_A-comoving(z_rec): the "100*theta_s" of the code CLASS
    pub theta_s_100: f64,
    /// 100 * r_s(z_star) / D_M(z_star): the 100 theta_* of the Planck papers
    pub theta_star_100: f64,
}

/// Number density of hydrogen nuclei today, in 1/m^3.
fn n_hydrogen_today(p: &Params, y_he: f64) -> f64 {
    let h100 = 1e5 / MPC;
    let rho_crit_h2 = 3.0 * h100 * h100 / (8.0 * std::f64::consts::PI * G_NEWTON);
    (1.0 - y_he) * p.omega_b * rho_crit_h2 / M_HYDROGEN
}

impl Thermo {
    pub fn new(bg: &Background) -> Thermo {
        let p = &bg.p;
        let y_he = bg.helium_fraction();
        let f_he = y_he / (NOT4 * (1.0 - y_he)); // helium nuclei per hydrogen nucleus
        let n_h0 = n_hydrogen_today(p, y_he);
        let hubble_si = |z: f64| bg.hubble(1.0 / (1.0 + z)) * C_LIGHT / MPC; // 1/s

        // ---- constants of the atomic model (values of RECFAST 1.5.2) ----
        let lam_alpha: f64 = 1.0 / 8.225_916_453e6; // Lyman-alpha wavelength (m), from L(2s1s) of H
        let l_h_ion = 1.096_787_737e7; // 1/m, ionization level of H
        let l_h_alpha = 8.225_916_453e6; // 1/m
        let l_he1_ion = 1.983_107_72e7;
        let l_he2_ion = 4.389_088_863e7;
        let l_he_2s = 1.662_774_34e7;
        let l_he_2p: f64 = 1.711_348_91e7;
        let lambda_2s_h = 8.224_580_9; // 1/s, two-photon rate of H
        let lambda_2s_he = 51.3; // 1/s, two-photon rate of He
        let hc_k = H_PLANCK * C_LIGHT / K_BOLTZ; // m K
        let cr = 2.0 * std::f64::consts::PI * M_ELECTRON * K_BOLTZ / (H_PLANCK * H_PLANCK); // (2 pi m k / h^2), 1/(m^2 K)
        // hydrogen: fudge factor of the case-B coefficient, and the Gaussians in ln(1+z)
        // that correct the escape factor K (amplitude, centre, width)
        let fudge = 1.125;
        let k_gauss = [(-0.14, 7.28, 0.18), (0.079, 6.73, 0.33)];
        // helium: singlet 2^1P-1^1S and triplet 2^3P-1^1S lines
        let a2p_s = 1.798_287e9; // 1/s, Einstein A of the singlet line
        let a2p_t = 177.58; // 1/s, Einstein A of the triplet (intercombination) line
        let sigma_he_2ps = 1.436_289e-22; // m^2, H ionization cross section at the singlet line
        let sigma_he_2pt = 1.484_872e-22; // m^2, the same at the triplet line
        let l_he_2pt: f64 = 1.690_871_466e7; // 1/m, level 2^3P
        let l_he_2st = 1.598_559_752_6e7; // 1/m, level 2^3S
        let l_he2st_ion = 3.845_469_384_5e6; // 1/m, ionization of 2^3S
        let b_he = 0.86; // fudge exponent of the continuum-opacity correction (singlet)

        // ---- Saha equilibrium ----
        // helium doubly ionized -> singly:  x_e (x_e - 1 - f)/(1 + 2 f - x_e) = rhs
        let saha_he2 = |z: f64| -> f64 {
            let t = p.t_cmb * (1.0 + z);
            let nh = n_h0 * (1.0 + z).powi(3);
            let rhs = (cr * t).powf(1.5) * (-hc_k * l_he2_ion / t).exp() / nh;
            // x_e^2 + (rhs - 1 - f) x_e - rhs (1 + 2 f) = 0
            let b = rhs - 1.0 - f_he;
            0.5 * ((b * b + 4.0 * (1.0 + 2.0 * f_he) * rhs).sqrt() - b)
        };
        // helium singly ionized -> neutral, hydrogen fully ionized: x_e = 1 + x_He
        let saha_he1 = |z: f64| -> f64 {
            let t = p.t_cmb * (1.0 + z);
            let nh = n_h0 * (1.0 + z).powi(3);
            let rhs = 4.0 * (cr * t).powf(1.5) * (-hc_k * l_he1_ion / t).exp() / nh;
            // x_e (x_e - 1) / (1 + f - x_e) = rhs
            let b = rhs - 1.0;
            0.5 * ((b * b + 4.0 * (1.0 + f_he) * rhs).sqrt() - b)
        };
        let saha_h = |z: f64| -> f64 {
            let t = p.t_cmb * (1.0 + z);
            let nh = n_h0 * (1.0 + z).powi(3);
            let rhs = (cr * t).powf(1.5) * (-hc_k * l_h_ion / t).exp() / nh;
            0.5 * ((rhs * rhs + 4.0 * rhs).sqrt() - rhs)
        };

        // ---- rate equations (derivatives with respect to z) ----
        let a_rad = 8.0 * std::f64::consts::PI.powi(5) * K_BOLTZ.powi(4) / (15.0 * H_PLANCK.powi(3) * C_LIGHT.powi(3));
        let ct = 8.0 / 3.0 * SIGMA_THOMSON / (M_ELECTRON * C_LIGHT) * a_rad;
        let derivs = |z: f64, xh: f64, xhe: f64, tm: f64, he_ode: bool, h_ode: bool| -> (f64, f64, f64) {
            let x = xh + xhe;
            let trad = p.t_cmb * (1.0 + z);
            let nh = n_h0 * (1.0 + z).powi(3);
            let hz = hubble_si(z);
            // hydrogen
            let dxh = if h_ode {
                let t4 = tm / 1e4;
                let rdown = fudge * 1e-19 * 4.309 * t4.powf(-0.6166) / (1.0 + 0.6703 * t4.powf(0.5300));
                let rup = rdown * (cr * tm).powf(1.5) * (-hc_k * (l_h_ion - l_h_alpha) / tm).exp();
                let lnz = (1.0 + z).ln();
                let corr: f64 = 1.0 + k_gauss.iter().map(|&(a, c, w)| a * (-((lnz - c) / w).powi(2)).exp()).sum::<f64>();
                let k = lam_alpha.powi(3) / (8.0 * std::f64::consts::PI * hz) * corr;
                let n1s = nh * (1.0 - xh);
                (x * xh * nh * rdown - rup * (1.0 - xh) * (-hc_k * l_h_alpha / tm).exp()) * (1.0 + k * lambda_2s_h * n1s) / (hz * (1.0 + z) * (1.0 + k * (lambda_2s_h + rup) * n1s))
            } else {
                0.0
            };
            // helium: singlet channel with the modified escape probability, and triplet channel
            // (as in RECFAST, helium is not evolved once its ionized fraction is below 1e-15)
            let dxhe = if he_ode && xhe > 1e-15 * f_he {
                let pi = std::f64::consts::PI;
                let sq0 = (tm / 3.0).sqrt();
                let sq1 = (tm / 10f64.powf(5.114)).sqrt();
                let n_he1 = (nh * (f_he - xhe)).max(1e-30); // neutral helium atoms per m^3
                let neutral_h = (1.0 - xh).max(0.0);
                // thermal Doppler width factor sqrt(2 k T / (m_He c^2))
                let dop = (2.0 * K_BOLTZ * tm / (M_HYDROGEN * NOT4 * C_LIGHT * C_LIGHT)).sqrt();
                // -- singlet --
                let rdown = 10f64.powf(-16.744) / (sq0 * (1.0 + sq0).powf(1.0 - 0.711) * (1.0 + sq1).powf(1.0 + 0.711));
                let rup = 4.0 * rdown * (cr * tm).powf(1.5) * (-hc_k * (l_he1_ion - l_he_2s) / tm).exp();
                // Sobolev optical depth and escape probability of the 2^1P-1^1S line
                let tau_s = a2p_s * 3.0 * n_he1 / (8.0 * pi * hz * l_he_2p.powi(3));
                let p_s = if tau_s < 1e-7 { 1.0 - 0.5 * tau_s } else { (1.0 - (-tau_s).exp()) / tau_s };
                // additional escape through absorption by neutral hydrogen (Kholupenko et al.)
                let a_hcon_s = if neutral_h > 1e-7 {
                    let doppler = C_LIGHT * l_he_2p * dop;
                    let gamma = 3.0 * a2p_s * (f_he - xhe).max(1e-30) * C_LIGHT * C_LIGHT / (pi.sqrt() * sigma_he_2ps * 8.0 * pi * doppler * neutral_h) / (C_LIGHT * l_he_2p).powi(2);
                    a2p_s / (1.0 + 0.36 * gamma.powf(b_he))
                } else {
                    0.0
                };
                // K_He times the density of neutral helium
                let kn = 1.0 / (3.0 * (a2p_s * p_s + a_hcon_s));
                // (the exponent is capped, as in RECFAST, to avoid an overflow at low temperature)
                let boltz = (hc_k * (l_he_2p - l_he_2s) / tm).min(680.0).exp();
                let singlet = (x * xhe * nh * rdown - rup * (f_he - xhe) * (-hc_k * l_he_2s / tm).exp()) * (1.0 + kn * lambda_2s_he * boltz)
                    / (hz * (1.0 + z) * (1.0 + kn * (lambda_2s_he + rup) * boltz));
                // -- triplet --
                let rdown_t = 10f64.powf(-16.306) / (sq0 * (1.0 + sq0).powf(1.0 - 0.761) * (1.0 + sq1).powf(1.0 + 0.761));
                let rup_t = 4.0 / 3.0 * rdown_t * (cr * tm).powf(1.5) * (-hc_k * l_he2st_ion / tm).exp();
                let tau_t = a2p_t * 3.0 * n_he1 / (8.0 * pi * hz * l_he_2pt.powi(3));
                let p_t = if tau_t < 1e-7 { 1.0 - 0.5 * tau_t } else { (1.0 - (-tau_t).exp()) / tau_t };
                let a_hcon_t = if neutral_h > 1e-5 {
                    let doppler = C_LIGHT * l_he_2pt * dop;
                    let gamma = 3.0 * a2p_t * (f_he - xhe).max(1e-30) * C_LIGHT * C_LIGHT / (pi.sqrt() * sigma_he_2pt * 8.0 * pi * doppler * neutral_h) / (C_LIGHT * l_he_2pt).powi(2);
                    a2p_t / (1.0 + 0.66 * gamma.powf(0.9)) / 3.0
                } else {
                    0.0
                };
                // branching factor C_t = c / (R_up + c),  c = (A p + A_Hcon) exp(-(E_2P - E_2S)/kT);
                // written with the ratio R_up/c, which cannot produce 0/0 at low temperature
                let ratio = 4.0 / 3.0 * rdown_t * (cr * tm).powf(1.5) / (a2p_t * p_t + a_hcon_t) * (-hc_k * (l_he2st_ion - (l_he_2pt - l_he_2st)) / tm).exp();
                let cf = 1.0 / (1.0 + ratio);
                let triplet = (x * xhe * nh * rdown_t - (f_he - xhe) * 3.0 * rup_t * (-hc_k * l_he_2st / tm).exp()) * cf / (hz * (1.0 + z));
                singlet + triplet
            } else {
                0.0
            };
            // matter temperature
            let dtm = ct * trad.powi(4) * x / (1.0 + x + f_he) * (tm - trad) / (hz * (1.0 + z)) + 2.0 * tm / (1.0 + z);
            (dxh, dxhe, dtm)
        };

        // ---- integrate from z_start downwards ----
        let z_start = 9000.0;
        let dz = 0.5;
        let n = (z_start / dz) as usize + 1;
        let mut zs = Vec::with_capacity(n);
        let mut xe = Vec::with_capacity(n);
        let mut tb = Vec::with_capacity(n);
        let (mut xh, mut xhe) = (1.0, f_he);
        let mut tm = p.t_cmb * (1.0 + z_start);
        let (mut he_ode, mut h_ode) = (false, false);
        for i in 0..n {
            let z = z_start - dz * i as f64;
            let x_now;
            if z > 5000.0 {
                x_now = saha_he2(z);
                tm = p.t_cmb * (1.0 + z);
            } else if !he_ode {
                let xs = saha_he1(z);
                xhe = xs - 1.0;
                tm = p.t_cmb * (1.0 + z);
                x_now = xs;
                if xhe < 0.99 * f_he {
                    he_ode = true;
                }
            } else {
                // one RK4 step from z + dz to z was taken at the end of the previous pass
                x_now = xh + xhe;
            }
            zs.push(z);
            xe.push(x_now);
            tb.push(tm);
            if z <= 0.0 {
                break;
            }
            // advance the rate equations to z - dz
            if he_ode {
                if !h_ode {
                    let xs = saha_h(z - dz);
                    if xs < 0.985 {
                        h_ode = true;
                        xh = saha_h(z);
                    } else {
                        xh = xs;
                    }
                }
                // matter temperature: tightly coupled to the radiation at early times
                let tight = z > 400.0;
                // While the Compton coupling of the gas to the radiation is tight (the
                // temperature equation is then extremely stiff), every Runge-Kutta stage
                // uses the quasi-static solution of the temperature equation (as RECFAST does),
                //     T_m = T_r - eps,   eps = H (1 + x + f_He) / (C_T T_r^3 x),
                // instead of a temperature advanced with the stiff derivative.
                let tm_qs = |zz: f64, x: f64| -> f64 {
                    let trad = p.t_cmb * (1.0 + zz);
                    trad - hubble_si(zz) * (1.0 + x + f_he) / (ct * trad.powi(3) * x)
                };
                let step = |xh: f64, xhe: f64, tm: f64, zz: f64| {
                    let t = if tight { tm_qs(zz, xh + xhe) } else { tm };
                    derivs(zz, xh, xhe, t, true, h_ode)
                };
                let h = -dz;
                let (k1h, k1e, k1t) = step(xh, xhe, tm, z);
                let (k2h, k2e, k2t) = step(xh + 0.5 * h * k1h, xhe + 0.5 * h * k1e, tm + 0.5 * h * k1t, z + 0.5 * h);
                let (k3h, k3e, k3t) = step(xh + 0.5 * h * k2h, xhe + 0.5 * h * k2e, tm + 0.5 * h * k2t, z + 0.5 * h);
                let (k4h, k4e, k4t) = step(xh + h * k3h, xhe + h * k3e, tm + h * k3t, z + h);
                if h_ode {
                    xh += h / 6.0 * (k1h + 2.0 * k2h + 2.0 * k3h + k4h);
                }
                xhe += h / 6.0 * (k1e + 2.0 * k2e + 2.0 * k3e + k4e);
                xhe = xhe.max(0.0);
                assert!(xh.is_finite() && xhe.is_finite(), "recombination: non-finite ionization fraction at z = {}", z);
                if tight {
                    tm = tm_qs(z - dz, xh + xhe);
                } else {
                    tm += h / 6.0 * (k1t + 2.0 * k2t + 2.0 * k3t + k4t);
                }
            }
        }

        // ---- reionization: find z_reio that gives the requested optical depth ----
        let n_z = zs.len();
        let x_rec = xe.clone();
        let reion = |z: f64, z_re: f64| -> f64 {
            // CAMB-style: tanh in y = (1+z)^1.5, width dz = 0.5; helium reionized once with
            // hydrogen, and for the second time at z = 3.5
            let y = (1.0 + z).powf(1.5);
            let y_re = (1.0 + z_re).powf(1.5);
            let dy = 1.5 * (1.0 + z_re).sqrt() * 0.5;
            let f1 = 0.5 * (1.0 + ((y_re - y) / dy).tanh());
            let f2 = 0.5 * (1.0 + ((3.5 - z) / 0.5).tanh());
            (1.0 + f_he) * f1 + f_he * f2
        };
        let sigma_mpc = SIGMA_THOMSON * MPC; // m^3 -> multiply by n (1/m^3) to get 1/Mpc
        let dkappa_dz = |z: f64, x: f64| -> f64 {
            // d kappa / dz = n_e sigma_T c / (H (1+z)) -> with H in 1/Mpc: n_e sigma_T / (H (1+z))
            let nh = n_h0 * (1.0 + z).powi(3);
            x * nh * sigma_mpc / (bg.hubble(1.0 / (1.0 + z)) * (1.0 + z))
        };
        let tau_of = |z_re: f64| -> f64 {
            // optical depth of the reionized gas (the residual from recombination is not counted)
            let mut s = 0.0;
            for i in 0..n_z - 1 {
                let (z1, z2) = (zs[i], zs[i + 1]);
                if z1 > 50.0 {
                    continue;
                }
                let f = |z: f64, xr: f64| dkappa_dz(z, (reion(z, z_re) - xr).max(0.0));
                s += 0.5 * (z1 - z2) * (f(z1, x_rec[i]) + f(z2, x_rec[i + 1]));
            }
            s
        };
        let (mut lo, mut hi) = (2.0, 30.0);
        for _ in 0..50 {
            let mid = 0.5 * (lo + hi);
            if tau_of(mid) < p.tau_reio {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let z_reio = 0.5 * (lo + hi);
        for i in 0..n_z {
            if zs[i] < 50.0 {
                xe[i] = xe[i].max(reion(zs[i], z_reio));
            }
        }

        // ---- tables in conformal time ----
        let tau: Vec<f64> = zs.iter().map(|&z| bg.tau_of_a(1.0 / (1.0 + z))).collect();
        let dkappa: Vec<f64> = zs
            .iter()
            .zip(&xe)
            .map(|(&z, &x)| {
                let a = 1.0 / (1.0 + z);
                x * n_h0 * (1.0 + z).powi(3) * sigma_mpc * a
            })
            .collect();
        // kappa(z) = Int_0^z (d kappa/dz) dz, accumulated from today (the last entry) backwards
        let mut kappa = vec![0.0; n_z];
        let mut kappa_rec = vec![0.0; n_z]; // without reionization
        for i in (0..n_z - 1).rev() {
            let (z1, z2) = (zs[i], zs[i + 1]);
            kappa[i] = kappa[i + 1] + 0.5 * (z1 - z2) * (dkappa_dz(z1, xe[i]) + dkappa_dz(z2, xe[i + 1]));
            kappa_rec[i] = kappa_rec[i + 1] + 0.5 * (z1 - z2) * (dkappa_dz(z1, x_rec[i]) + dkappa_dz(z2, x_rec[i + 1]));
        }
        // baryon sound speed: c_b^2 = (k T_b / (mu m_H c^2)) (1 - (1/3) d ln T_b / d ln a)
        let mut cb2 = vec![0.0; n_z];
        for i in 0..n_z {
            let (i0, i1) = (if i == 0 { 0 } else { i - 1 }, if i == n_z - 1 { n_z - 1 } else { i + 1 });
            let dlnt_dlna = -((tb[i1].ln() - tb[i0].ln()) / ((1.0 + zs[i1]).ln() - (1.0 + zs[i0]).ln()));
            let mu_inv = 1.0 - y_he + y_he / NOT4 + (1.0 - y_he) * xe[i]; // 1/mu: particles per hydrogen-atom mass
            cb2[i] = K_BOLTZ * tb[i] * mu_inv / (M_HYDROGEN * C_LIGHT * C_LIGHT) * (1.0 - dlnt_dlna / 3.0);
        }

        // ---- characteristic redshifts ----
        let interp_z = |f: &dyn Fn(usize) -> f64, target: f64| -> f64 {
            // f decreases with increasing index (decreasing z); find the crossing of `target`
            for i in 0..n_z - 1 {
                let (f1, f2) = (f(i), f(i + 1));
                if f1 >= target && f2 < target {
                    return zs[i] + (zs[i + 1] - zs[i]) * (f1 - target) / (f1 - f2);
                }
            }
            f64::NAN
        };
        let z_star = interp_z(&|i| kappa_rec[i], 1.0);
        // drag depth: d kappa_d = d kappa / R, R = 3 rho_b / (4 rho_gamma)
        let mut kd = vec![0.0; n_z];
        for i in (0..n_z - 1).rev() {
            let (z1, z2) = (zs[i], zs[i + 1]);
            let f = |z: f64, x: f64| dkappa_dz(z, x) / (0.75 * p.omega_b / bg.omega_gamma / (1.0 + z));
            kd[i] = kd[i + 1] + 0.5 * (z1 - z2) * (f(z1, x_rec[i]) + f(z2, x_rec[i + 1]));
        }
        let z_drag = interp_z(&|i| kd[i], 1.0);
        // maximum of the visibility g = kappa' exp(-kappa) (in conformal time), around recombination
        let mut best = (0.0, 0usize);
        for i in 1..n_z - 1 {
            if zs[i] > 500.0 && zs[i] < 2000.0 {
                let g = dkappa[i] * (-kappa[i]).exp();
                if g > best.0 {
                    best = (g, i);
                }
            }
        }
        // refine with a parabola through the three points around the maximum
        let ib = best.1;
        let gv = |i: usize| dkappa[i] * (-kappa[i]).exp();
        let (g0, g1, g2) = (gv(ib - 1), gv(ib), gv(ib + 1));
        let shift = 0.5 * (g0 - g2) / (g0 - 2.0 * g1 + g2);
        let z_rec = zs[ib] + shift * (zs[ib + 1] - zs[ib]);

        let rs = |z: f64| bg.sound_horizon(1.0 / (1.0 + z));
        let (rs_star, rs_rec, rs_drag) = (rs(z_star), rs(z_rec), rs(z_drag));
        let dm = |z: f64| bg.tau0 - bg.tau_of_a(1.0 / (1.0 + z));
        Thermo {
            z: zs,
            xe,
            tb,
            tau,
            dkappa,
            kappa,
            cb2,
            y_he,
            z_reio,
            z_star,
            z_rec,
            z_drag,
            rs_star,
            rs_rec,
            rs_drag,
            theta_s_100: 100.0 * rs_rec / dm(z_rec),
            theta_star_100: 100.0 * rs_star / dm(z_star),
        }
    }

    /// Interpolate (kappa', kappa, c_b^2) at conformal time tau.  The tables are indexed
    /// by redshift (descending), i.e. by ascending tau.
    pub fn at_tau(&self, tau: f64) -> (f64, f64, f64) {
        let n = self.tau.len();
        if tau <= self.tau[0] {
            // before the table: fully ionized, kappa' proportional to a^-2 ~ tau^-2
            let r = (self.tau[0] / tau).powi(2);
            return (self.dkappa[0] * r, self.kappa[0] + self.dkappa[0] * self.tau[0] * (self.tau[0] / tau - 1.0), self.cb2[0] * self.tau[0] / tau);
        }
        if tau >= self.tau[n - 1] {
            return (self.dkappa[n - 1], 0.0, self.cb2[n - 1]);
        }
        let (mut lo, mut hi) = (0, n - 1);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.tau[mid] <= tau {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let f = (tau - self.tau[lo]) / (self.tau[hi] - self.tau[lo]);
        // kappa' varies by orders of magnitude: interpolate its logarithm
        let dk = (self.dkappa[lo].ln() + f * (self.dkappa[hi].ln() - self.dkappa[lo].ln())).exp();
        let k = self.kappa[lo] + f * (self.kappa[hi] - self.kappa[lo]);
        let cb = self.cb2[lo] + f * (self.cb2[hi] - self.cb2[lo]);
        (dk, k, cb)
    }
}
