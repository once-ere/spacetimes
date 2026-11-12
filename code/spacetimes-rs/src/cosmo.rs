//! Homogeneous cosmology: expansion history, the dark radiation of the model
//! (including its "step"), recombination, and the thermodynamic tables that the
//! Einstein-Boltzmann solver (`boltzmann.rs`) needs.
//!
//! UNITS.  Lengths and conformal times in Mpc; wavenumbers in 1/Mpc; the Hubble rate
//! H in 1/Mpc (H0 = 100 h km/s/Mpc = h / 2997.92458 Mpc^-1); temperatures in kelvin.
//! "omega_x" always means the physical density Omega_x h^2.
//!
//! CONTENT OF THE UNIVERSE: photons, massless free-streaming neutrinos (N_ur species),
//! baryons, cold dark matter, a cosmological constant (flat space), and a DARK FLUID,
//! which is the self-interacting dark radiation of the model (the light quanta of the
//! Vorton-Scalar and of the 16-component fermion field in thermal contact).  The dark
//! fluid is described by its energy density rho_d(a), its pressure and its adiabatic
//! sound speed; see `DarkFluid`.
//!
//! RECOMBINATION follows the effective three-level atoms of Peebles (hydrogen) and of
//! Seager, Sasselov and Scott (helium), in the form of the program RECFAST 1.5.2
//! (Seager, Sasselov and Scott 1999, 2000; Wong, Moss and Scott 2008; Scott and Moss
//! 2009): Saha equilibrium while it holds, then the rate equations.  The equations
//! contain the corrections that make the three-level atoms reproduce the detailed
//! multi-level calculations: the factor 1.125 multiplying the hydrogen case-B
//! coefficient, two Gaussian corrections of the Lyman-alpha escape factor K, and for
//! helium the escape probability of the 2^1P-1^1S line with the continuum opacity of
//! neutral hydrogen, and recombination through the triplet states.  Reionization is a
//! tanh step in (1+z)^(3/2) whose redshift is adjusted to give the requested optical
//! depth.

use crate::quad::gauss_legendre;

// ---- physical constants (SI, CODATA 2018 / IAU 2015) --------------------------------
pub const C_LIGHT: f64 = 299_792_458.0; // m/s
pub const G_NEWTON: f64 = 6.674_30e-11; // m^3 kg^-1 s^-2
pub const K_BOLTZ: f64 = 1.380_649e-23; // J/K
pub const H_PLANCK: f64 = 6.626_070_15e-34; // J s
pub const M_ELECTRON: f64 = 9.109_383_7015e-31; // kg
pub const M_HYDROGEN: f64 = 1.673_575e-27; // kg, mass of the hydrogen atom
pub const SIGMA_THOMSON: f64 = 6.652_458_7321e-29; // m^2
pub const MPC: f64 = 3.085_677_581_491_367_3e22; // m
pub const EV: f64 = 1.602_176_634e-19; // J
/// ratio of the helium-4 mass to the hydrogen-atom mass
pub const NOT4: f64 = 3.9715;
/// c / (100 km/s/Mpc) in Mpc
pub const HUBBLE_LENGTH: f64 = 2997.924_58;

/// Energy density of one massless fermion species relative to the photons after
/// electron-positron annihilation: (7/8) (4/11)^(4/3).
pub fn neutrino_factor() -> f64 {
    7.0 / 8.0 * (4.0f64 / 11.0).powf(4.0 / 3.0)
}

/// The dark fluid: self-interacting dark radiation with an optional step.
///
/// * `n_ir` is the energy density of the fluid today, in units of the density of one
///   massless neutrino species (the usual convention for "N_eff").
/// * Without a step (`step` = None) the fluid is radiation at all times: w = c_s^2 = 1/3.
/// * With a step, the fluid is an ideal gas in equilibrium that consists of massless
///   particles and of particles of mass m_t (see `Step`).  While the dark temperature
///   T_d falls through m_t, the massive particles annihilate into the massless ones at
///   constant comoving entropy, which heats the massless ones: relative to a^(-4)
///   scaling, the energy density of the fluid rises by the factor
///       r_step = [ (g_light + g_heavy,eff) / g_light ]^(1/3),
///   where g_heavy,eff = g_heavy for bosons and (7/8) g_heavy for fermions.
///   `z_t` is defined by  m_t = T_d0 (1 + z_t)  with T_d0 the dark temperature today.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DarkFluid {
    pub n_ir: f64,
    pub step: Option<Step>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Step {
    /// effective number of massless degrees of freedom, counted in bosonic units
    /// (a massless boson counts 1, a massless fermion 7/8)
    pub g_light: f64,
    /// number of degrees of freedom of the particles of mass m_t
    pub g_heavy: f64,
    /// statistics of the massive particles: true = Fermi-Dirac, false = Bose-Einstein
    pub heavy_fermions: bool,
    /// transition redshift, m_t = T_d0 (1 + z_t)
    pub z_t: f64,
}

impl Step {
    /// The step of the project's model: the massless phase mode of the Vorton-Scalar
    /// (one bosonic degree of freedom) and the 16 fermionic degrees of freedom of the
    /// field psi (8 particle and 8 antiparticle states per momentum) of mass m.
    pub fn project(z_t: f64) -> Step {
        Step { g_light: 1.0, g_heavy: 16.0, heavy_fermions: true, z_t }
    }

    /// The step of the Wess-Zumino dark radiation of Aloni, Berlin, Joseph, Schmaltz
    /// and Weiner (2022): a massive complex scalar (2 bosonic degrees of freedom)
    /// annihilates into a massless Weyl fermion (2 degrees of freedom, 7/4 in bosonic
    /// units).  r_step = (15/7)^(1/3).
    pub fn wess_zumino(z_t: f64) -> Step {
        Step { g_light: 1.75, g_heavy: 2.0, heavy_fermions: false, z_t }
    }

    /// The factor by which the fluid density rises relative to a^-4 scaling.
    pub fn r_step(&self) -> f64 {
        let g_eff = if self.heavy_fermions { 0.875 * self.g_heavy } else { self.g_heavy };
        ((self.g_light + g_eff) / self.g_light).powf(1.0 / 3.0)
    }
}

/// Dark matter that is a SUPERFLUID instead of cold collisionless particles.
///
/// If `Params::superfluid_dm` is set, the whole dark-matter density omega_c is carried
/// by a barotropic fluid whose pressure is that of the superfluid ground state.  Its
/// density perturbations propagate as first sound (the Bogoliubov-Anderson mode),
/// with the adiabatic sound speed c_s^2 = dP/d rho.  Two limiting cases:
///
/// * `Degenerate { x0 }`: weak pairing (BCS regime).  The pressure is the degeneracy
///   pressure of the fermions; pairing changes it only by terms of relative order
///   (gap / Fermi energy)^2.  With x = k_F / m = x0 / a (k_F Fermi momentum, m fermion
///   mass, a scale factor) the ideal Fermi gas has
///       rho ~ chi(x) = [ x sqrt(1+x^2) (1+2x^2) - asinh x ] / 8 ,
///       P   ~ phi(x) = [ x sqrt(1+x^2) (2x^2/3 - 1) + asinh x ] / 8 ,
///       c_s^2 = x^2 / (3 (1 + x^2))          (= v_F^2 / 3),
///   both in units of g m^4 / (2 pi^2).
///
/// * `Bose { cs0_sq }`: tightly bound bosons (pairs or larger clusters) that have
///   condensed, with a repulsive contact interaction (BEC regime).  The condensate of
///   a complex scalar field with quartic self-interaction has (Colpi, Shapiro and
///   Wasserman 1986; Li, Rindler-Daller and Shapiro 2014)
///       P = (y - 1)^2 / (18 B),   rho = (y^2 - 1) / (6 B),   y = sqrt(1 + 6 B rho),
///       c_s^2 = (y - 1) / (3 y),   w = (y - 1) / (3 (y + 1)),
///   which is matter-like at low density (c_s^2 = B rho, Bogoliubov sound) and
///   radiation-like at high density.  Energy conservation gives
///       (y - 1) sqrt(2y + 1) = const / a^3.
///   `cs0_sq` is c_s^2 today at the mean cosmic density, in units of c^2.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SuperfluidDm {
    Degenerate { x0: f64 },
    Bose { cs0_sq: f64 },
}

impl SuperfluidDm {
    /// (rho(a) a^3 / rho(1), w, c_s^2) at scale factor a.
    pub fn state(&self, a: f64) -> (f64, f64, f64) {
        match *self {
            SuperfluidDm::Degenerate { x0 } => {
                // chi and phi; series for small x to avoid cancellation
                let chi_phi = |x: f64| -> (f64, f64) {
                    if x < 0.05 {
                        let x3 = x * x * x;
                        let x2 = x * x;
                        (x3 * (1.0 / 3.0 + x2 * (0.1 - x2 / 56.0)), x3 * x2 * (1.0 / 15.0 - x2 / 42.0))
                    } else {
                        let r = (1.0 + x * x).sqrt();
                        let ash = x.asinh();
                        ((x * r * (1.0 + 2.0 * x * x) - ash) / 8.0, (x * r * (2.0 * x * x / 3.0 - 1.0) + ash) / 8.0)
                    }
                };
                let x = x0 / a;
                let (chi, phi) = chi_phi(x);
                let (chi0, _) = chi_phi(x0);
                (chi / chi0 * a * a * a, phi / chi, x * x / (3.0 * (1.0 + x * x)))
            }
            SuperfluidDm::Bose { cs0_sq } => {
                // u = y - 1 solves  u sqrt(2u + 3) = K / a^3  with  K = u0 sqrt(2 u0 + 3);
                // today c_s^2 = u0 / (3 (1 + u0))
                let u0 = 3.0 * cs0_sq / (1.0 - 3.0 * cs0_sq);
                let rhs = u0 * (2.0 * u0 + 3.0).sqrt() / (a * a * a);
                // Newton iteration on f(u) = u sqrt(2u+3) - rhs, started from the asymptotic forms
                let mut u = if rhs < 1.0 { rhs / 3f64.sqrt() } else { (rhs * rhs / 2.0).powf(1.0 / 3.0) };
                for _ in 0..60 {
                    let r = (2.0 * u + 3.0).sqrt();
                    let f = u * r - rhs;
                    let df = r + u / r;
                    let du = f / df;
                    u -= du;
                    if du.abs() < 1e-15 * u.abs() {
                        break;
                    }
                }
                // rho ~ y^2 - 1 = u (u + 2)
                let ratio = u * (u + 2.0) / (u0 * (u0 + 2.0)) * a * a * a;
                (ratio, u / (3.0 * (u + 2.0)), u / (3.0 * (u + 1.0)))
            }
        }
    }
}

/// Cosmological parameters.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Params {
    pub omega_b: f64,
    pub omega_c: f64,
    pub h: f64,
    /// primordial amplitude A_s of the curvature power spectrum at k = 0.05/Mpc
    pub a_s: f64,
    pub n_s: f64,
    /// optical depth of reionization
    pub tau_reio: f64,
    /// number of massless free-streaming neutrino species
    pub n_ur: f64,
    pub t_cmb: f64,
    /// helium mass fraction; None = the standard-BBN value for omega_b and the radiation content
    pub y_he: Option<f64>,
    pub dark: DarkFluid,
    /// if set, the dark matter (density omega_c) is a superfluid instead of cold particles
    pub superfluid_dm: Option<SuperfluidDm>,
}

impl Params {
    /// The fiducial LambdaCDM model of the project (the Planck 2018 best fit that the
    /// author's NCSS project uses, with massless neutrinos).
    pub fn fiducial() -> Params {
        Params {
            omega_b: 0.02237,
            omega_c: 0.1200,
            h: 0.6736,
            a_s: 2.1e-9, // ln(10^10 A_s) = 3.04452
            n_s: 0.9649,
            tau_reio: 0.0544,
            n_ur: 3.044,
            t_cmb: 2.7255,
            y_he: None,
            dark: DarkFluid { n_ir: 0.0, step: None },
            superfluid_dm: None,
        }
    }

    /// omega_gamma = Omega_gamma h^2 from the temperature of the microwave background.
    pub fn omega_gamma(&self) -> f64 {
        // rho_gamma = a_R T^4, a_R = 8 pi^5 k^4 / (15 h^3 c^3); rho_crit = 3 H^2 / (8 pi G)
        let a_rad = 8.0 * std::f64::consts::PI.powi(5) * K_BOLTZ.powi(4) / (15.0 * H_PLANCK.powi(3) * C_LIGHT.powi(3));
        let rho_gamma = a_rad * self.t_cmb.powi(4) / (C_LIGHT * C_LIGHT); // kg/m^3
        let h100 = 1e5 / MPC; // 100 km/s/Mpc in 1/s
        let rho_crit_h2 = 3.0 * h100 * h100 / (8.0 * std::f64::consts::PI * G_NEWTON);
        rho_gamma / rho_crit_h2
    }

    pub fn omega_nu(&self) -> f64 {
        self.n_ur * neutrino_factor() * self.omega_gamma()
    }

    /// The helium mass fraction: either the given value or standard BBN.
    ///
    /// The dependence on omega_b and on extra radiation is the fit to the code
    /// PArthENoPE quoted by the Planck Collaboration (2016, A&A 594, A13, Eq. 67); it is
    /// shifted by a constant so that the fiducial model has Y_P = 0.2454, the value
    /// of the Planck 2018 analysis.  `delta_n` is the radiation density DURING BBN in
    /// excess of 3.044 neutrino species.
    pub fn helium_fraction(&self) -> f64 {
        if let Some(y) = self.y_he {
            return y;
        }
        let fit = |ob: f64, dn: f64| {
            0.2311 + 0.9502 * ob - 11.27 * ob * ob + dn * (0.01356 + 0.008581 * ob - 0.1810 * ob * ob) + dn * dn * (-0.0009795 - 0.001370 * ob + 0.01746 * ob * ob)
        };
        let dn = self.n_ur - 3.044 + self.dark_n_uv();
        0.2454 + fit(self.omega_b, dn) - fit(0.02237, 0.0)
    }

    /// Density of the dark fluid at early times (before a step), in neutrino units.
    pub fn dark_n_uv(&self) -> f64 {
        match self.dark.step {
            None => self.dark.n_ir,
            Some(s) => self.dark.n_ir / s.r_step(),
        }
    }
}

// ---- thermodynamics of the stepped dark fluid ---------------------------------------

/// Dimensionless energy density and pressure of the equilibrium mixture of massless
/// particles (`g_light` in bosonic units) and of `g_heavy` degrees of freedom of mass m
/// at temperature T, as functions of x = m/T, in units of T^4:
///     rho/T^4 = (pi^2/30) g_light + g_heavy I_rho(x),   P/T^4 = (pi^2/90) g_light + g_heavy I_P(x),
///     I_rho = (1/2pi^2) Int_0^inf q^2 e f(e) dq,   I_P = (1/6pi^2) Int_0^inf (q^4/e) f(e) dq,
///     e = sqrt(q^2 + x^2),   f(e) = 1/(exp(e) + 1) for fermions, 1/(exp(e) - 1) for bosons.
/// Also returned: d(rho/T^4)/dx and d(P/T^4)/dx, needed for the sound speed.
fn step_thermo(st: &Step, x: f64, gl: &(Vec<f64>, Vec<f64>)) -> (f64, f64, f64, f64) {
    let pi2 = std::f64::consts::PI * std::f64::consts::PI;
    let sgn = if st.heavy_fermions { 1.0 } else { -1.0 };
    // integrate in q over [0, q_max] (the integrand is < e^-40 beyond); the panels are
    // graded towards q = 0, where the Bose-Einstein function varies rapidly if x is small
    let panels = 16;
    let q_max = 40.0 + 2.0 * x.min(40.0);
    let (mut i_rho, mut i_p, mut di_rho, mut di_p) = (0.0, 0.0, 0.0, 0.0);
    for p in 0..panels {
        let edge = |i: usize| q_max * (i as f64 / panels as f64).powi(2);
        let (a, b) = (edge(p), edge(p + 1));
        let (hh, cc) = (0.5 * (b - a), 0.5 * (b + a));
        for (t, w) in gl.0.iter().zip(&gl.1) {
            let q = cc + hh * t;
            let e = (q * q + x * x).sqrt();
            if e > 700.0 {
                continue;
            }
            let f = 1.0 / (e.exp() + sgn);
            let dfde = -f * (1.0 - sgn * f); // derivative of the distribution function
            let wq = hh * w;
            i_rho += wq * q * q * e * f;
            i_p += wq * q.powi(4) / e * f;
            // d/dx at fixed q: de/dx = x/e
            di_rho += wq * q * q * (f + e * dfde) * x / e;
            di_p += wq * q.powi(4) * (-f / (e * e) + dfde / e) * x / e;
        }
    }
    let rho = pi2 / 30.0 * st.g_light + st.g_heavy * i_rho / (2.0 * pi2);
    let p = pi2 / 90.0 * st.g_light + st.g_heavy * i_p / (6.0 * pi2);
    (rho, p, st.g_heavy * di_rho / (2.0 * pi2), st.g_heavy * di_p / (6.0 * pi2))
}

/// Table of the stepped fluid as a function of ln a: density relative to the
/// late-time a^-4 law, equation of state w, and adiabatic sound speed c_s^2.
#[derive(Clone, Debug)]
pub struct StepTable {
    ln_a: Vec<f64>,
    /// rho_d(a) a^4 / (rho_d a^4 today)
    ratio: Vec<f64>,
    w: Vec<f64>,
    cs2: Vec<f64>,
}

impl StepTable {
    pub fn new(s: &Step) -> StepTable {
        let gl = gauss_legendre(24);
        // Parametrize by x = m_t / T_d.  Entropy conservation: s a^3 = const with
        // s = (rho + P)/T, so  a(x) = a_t x [ sigma(inf) / sigma(x) ]^(1/3),
        // sigma = (rho + P)/T^4, where a_t = 1/(1+z_t) is the scale factor at which
        // the late-time temperature law T_d = T_d0 / a would give x = 1.
        let (rho_inf, p_inf, _, _) = step_thermo(s, 600.0, &gl);
        let sig_inf = rho_inf + p_inf;
        let a_t = 1.0 / (1.0 + s.z_t);
        let n = 400;
        let (lx0, lx1) = ((1e-4f64).ln(), (200.0f64).ln());
        let mut t = StepTable { ln_a: Vec::new(), ratio: Vec::new(), w: Vec::new(), cs2: Vec::new() };
        for i in 0..n {
            let x = (lx0 + (lx1 - lx0) * i as f64 / (n - 1) as f64).exp();
            let (rho, p, drho, dp) = step_thermo(s, x, &gl);
            let a = a_t * x * (sig_inf / (rho + p)).powf(1.0 / 3.0);
            // rho_d = rho T^4 = rho (m/x)^4; today rho_inf (m/x_0)^4 with a = a_t x_0 (late times):
            //   rho_d a^4 / (rho_d a^4)_today = (rho/rho_inf) (a/(a_t x))^4
            let ratio = rho / rho_inf * (a / (a_t * x)).powi(4);
            // adiabatic sound speed c_s^2 = dP/drho along the equilibrium curve, with
            // rho_phys = rho(x) m^4/x^4:  d rho_phys/dx = m^4 (rho'(x)/x^4 - 4 rho/x^5)
            let cs2 = (dp - 4.0 * p / x) / (drho - 4.0 * rho / x);
            t.ln_a.push(a.ln());
            t.ratio.push(ratio);
            t.w.push(p / rho);
            t.cs2.push(cs2);
        }
        t
    }

    /// (ratio, w, c_s^2) at scale factor a (linear interpolation in ln a; constant outside).
    pub fn at(&self, a: f64) -> (f64, f64, f64) {
        let la = a.ln();
        let n = self.ln_a.len();
        if la <= self.ln_a[0] {
            return (self.ratio[0], self.w[0], self.cs2[0]);
        }
        if la >= self.ln_a[n - 1] {
            return (self.ratio[n - 1], self.w[n - 1], self.cs2[n - 1]);
        }
        // the table is monotonic in ln a: bisection
        let (mut lo, mut hi) = (0, n - 1);
        while hi - lo > 1 {
            let mid = (lo + hi) / 2;
            if self.ln_a[mid] <= la {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let f = (la - self.ln_a[lo]) / (self.ln_a[hi] - self.ln_a[lo]);
        let lerp = |v: &Vec<f64>| v[lo] + f * (v[hi] - v[lo]);
        (lerp(&self.ratio), lerp(&self.w), lerp(&self.cs2))
    }
}

// ---- background ----------------------------------------------------------------------

/// The homogeneous background of a parameter set.
#[derive(Clone, Debug)]
pub struct Background {
    pub p: Params,
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
}

impl Background {
    pub fn new(p: &Params) -> Background {
        let omega_gamma = p.omega_gamma();
        let omega_nu = p.omega_nu();
        let omega_d = p.dark.n_ir * neutrino_factor() * omega_gamma;
        let step = p.dark.step.as_ref().map(StepTable::new);
        let omega_lambda = p.h * p.h - p.omega_b - p.omega_c - omega_gamma - omega_nu - omega_d;
        // the code has one slot for a dark fluid: dark radiation OR superfluid dark matter
        assert!(p.superfluid_dm.is_none() || p.dark.n_ir == 0.0, "dark radiation and superfluid dark matter cannot be combined");
        let omega_cdm = if p.superfluid_dm.is_some() { 0.0 } else { p.omega_c };
        let mut bg = Background { p: *p, omega_gamma, omega_nu, omega_d, omega_cdm, omega_lambda, step, tau0: 0.0, tab_lna: Vec::new(), tab_tau: Vec::new() };
        // conformal time tau(a) = Int_0^a da / (a^2 H): tabulate on a grid in ln a
        let gl = gauss_legendre(8);
        let n = 2400;
        let (l0, l1) = ((1e-9f64).ln(), 0.0);
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
        let (od, _, _) = self.dark(a);
        let e2 = (self.omega_gamma + self.omega_nu + od) / a.powi(4) + (self.p.omega_b + self.omega_cdm) / a.powi(3) + self.omega_lambda;
        e2.sqrt() / HUBBLE_LENGTH
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
        let y_he = p.helium_fraction();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn photon_density() {
        // omega_gamma for T = 2.7255 K is 2.473e-5 (standard value)
        let p = Params::fiducial();
        assert!((p.omega_gamma() / 2.4728e-5 - 1.0).abs() < 2e-4, "{}", p.omega_gamma());
    }

    #[test]
    fn step_limits() {
        // 16 fermionic degrees of freedom and one massless boson: r_step = 15^(1/3)
        let s = Step::project(1.0e4);
        assert!((s.r_step() - 15f64.powf(1.0 / 3.0)).abs() < 1e-12);
        let t = StepTable::new(&s);
        let (r_early, w_early, c_early) = t.at(1e-9);
        let (r_late, w_late, c_late) = t.at(1.0);
        assert!((r_early * s.r_step() - 1.0).abs() < 2e-3, "early ratio {}", r_early);
        assert!((r_late - 1.0).abs() < 1e-3, "late ratio {}", r_late);
        for v in [w_early, c_early, w_late, c_late] {
            assert!((v - 1.0 / 3.0).abs() < 2e-3, "{}", v);
        }
        // in between, pressure support is reduced: w < 1/3 and c_s^2 < 1/3
        let (_, w_mid, c_mid) = t.at(1.0 / (1.0 + 1.0e4) / 3.0);
        assert!(w_mid < 0.33 && c_mid < 0.33 && w_mid > 0.2 && c_mid > 0.15, "{} {}", w_mid, c_mid);
    }

    #[test]
    fn step_thermodynamic_identity_and_wess_zumino_limit() {
        // With P = p(x) T^4 and x = m/T, the identity s = dP/dT = (rho + P)/T reads
        //     rho = 3 p - x dp/dx ,
        // which tests the four integrals of `step_thermo` against each other, for both
        // statistics of the massive particles.
        let gl = gauss_legendre(24);
        for st in [Step::project(1.0e4), Step::wess_zumino(1.0e4)] {
            for &x in &[0.05, 0.7, 3.0, 12.0] {
                let (rho, p, _, dp) = step_thermo(&st, x, &gl);
                assert!((rho - 3.0 * p + x * dp).abs() < 2e-9 * rho, "x = {}: {:e}", x, rho - 3.0 * p + x * dp);
            }
            // massless limit: rho = (pi^2/30) (g_light + g_heavy,eff)
            let (rho0, p0, _, _) = step_thermo(&st, 1e-5, &gl);
            let g_eff = st.g_light + if st.heavy_fermions { 0.875 * st.g_heavy } else { st.g_heavy };
            let exact = std::f64::consts::PI.powi(2) / 30.0 * g_eff;
            assert!((rho0 / exact - 1.0).abs() < 1e-6, "{} {}", rho0, exact);
            assert!((3.0 * p0 / rho0 - 1.0).abs() < 1e-6);
        }
        let wz = Step::wess_zumino(2.0e4);
        assert!((wz.r_step() - (15.0f64 / 7.0).powf(1.0 / 3.0)).abs() < 1e-12);
        let t = StepTable::new(&wz);
        assert!((t.at(1e-9).0 * wz.r_step() - 1.0).abs() < 2e-3);
        assert!((t.at(1.0).0 - 1.0).abs() < 1e-3);
    }

    #[test]
    fn ionization_history() {
        // Checks of the ionization history x_e(z) = n_e / n_H of the fiducial model:
        //  * helium is doubly ionized at z = 6000 (x_e = 1 + 2 f_He), singly at z = 4000
        //    (x_e = 1 + f_He), and recombines GRADUALLY between z = 3000 and z = 1700;
        //  * hydrogen: x_e = 1/2 near z = 1270-1290, and a residual ionization of a few 1e-4.
        // (An earlier version lost all singly ionized helium within one integration step
        // at z = 2812, because the stiff temperature equation produced a NaN; the test
        // would fail for that version.)
        let bg = Background::new(&Params::fiducial());
        let th = Thermo::new(&bg);
        let f_he = th.y_he / (NOT4 * (1.0 - th.y_he));
        let xe = |z: f64| -> f64 {
            let i = th.z.iter().position(|&zz| (zz - z).abs() < 1e-9).unwrap();
            th.xe[i]
        };
        assert!(th.xe.iter().all(|v| v.is_finite()));
        assert!((xe(6000.0) - (1.0 + 2.0 * f_he)).abs() < 0.03, "{}", xe(6000.0));
        assert!((xe(4000.0) - (1.0 + f_he)).abs() < 1e-4);
        let he_left = |z: f64| (xe(z) - 1.0) / f_he; // fraction of helium still ionized
        assert!(he_left(2800.0) > 0.97, "{}", he_left(2800.0));
        assert!(he_left(2400.0) > 0.75 && he_left(2400.0) < 0.92, "{}", he_left(2400.0));
        assert!(he_left(2000.0) > 0.35 && he_left(2000.0) < 0.65, "{}", he_left(2000.0));
        assert!(he_left(1800.0) < 0.10 && he_left(1800.0) > 0.0, "{}", he_left(1800.0));
        // monotonic decrease from z = 3000 to z = 200
        let mut prev = xe(3000.0);
        let mut z = 2999.5;
        while z >= 200.0 {
            let v = xe(z);
            assert!(v <= prev * (1.0 + 1e-9), "x_e increases at z = {}", z);
            prev = v;
            z -= 0.5;
        }
        assert!(xe(1300.0) > 0.5 && xe(1250.0) < 0.5, "{} {}", xe(1300.0), xe(1250.0));
        assert!(xe(200.0) > 2.5e-4 && xe(200.0) < 4.5e-4, "{}", xe(200.0));
    }

    #[test]
    fn superfluid_equations_of_state() {
        // (1) energy conservation d(rho a^3)/d ln a = -3 w (rho a^3), by finite differences;
        // (2) c_s^2 = dP/d rho along the expansion;  (3) the limits.
        for sf in [SuperfluidDm::Degenerate { x0: 3e-3 }, SuperfluidDm::Bose { cs0_sq: 1e-9 }] {
            for &a in &[1e-7f64, 1e-5, 1e-4, 1e-3, 1e-2, 0.3] {
                let e = 1e-4;
                let (r0, w, cs2) = sf.state(a);
                let (rp, wp, _) = sf.state(a * (1.0 + e));
                let (rm, wm, _) = sf.state(a * (1.0 - e));
                let dlnr = (rp - rm) / (2.0 * e * r0);
                assert!((dlnr + 3.0 * w).abs() < 1e-6 + 1e-5 * w, "{:?} a = {}: {} vs {}", sf, a, dlnr, -3.0 * w);
                // rho ~ r / a^3, P = w rho
                let rho = |r: f64, a: f64| r / (a * a * a);
                let (rho_p, rho_m) = (rho(rp, a * (1.0 + e)), rho(rm, a * (1.0 - e)));
                let dp = wp * rho_p - wm * rho_m;
                let drho = rho_p - rho_m;
                assert!((dp / drho - cs2).abs() < 1e-6 + 1e-4 * cs2, "{:?} a = {}: {} vs {}", sf, a, dp / drho, cs2);
            }
            let (r1, _, _) = sf.state(1.0);
            assert!((r1 - 1.0).abs() < 1e-12);
            // radiation-like at early times
            let (_, w_early, c_early) = sf.state(1e-12);
            assert!((w_early - 1.0 / 3.0).abs() < 1e-3 && (c_early - 1.0 / 3.0).abs() < 1e-3, "{} {}", w_early, c_early);
        }
        // today: c_s^2 = x0^2/3 (Fermi), = cs0_sq (Bose); w = x0^2/5 (Fermi), = cs0_sq/2 (Bose)
        let (_, w, c) = SuperfluidDm::Degenerate { x0: 1e-4 }.state(1.0);
        assert!((c / (1e-8 / 3.0) - 1.0).abs() < 1e-6 && (w / (1e-8 / 5.0) - 1.0).abs() < 1e-6);
        let (_, w, c) = SuperfluidDm::Bose { cs0_sq: 1e-10 }.state(1.0);
        assert!((c / 1e-10 - 1.0).abs() < 1e-8 && (w / 0.5e-10 - 1.0).abs() < 1e-8);
    }

    #[test]
    fn background_and_recombination_of_the_fiducial_model() {
        let bg = Background::new(&Params::fiducial());
        // conformal age: about 14.2 Gpc for these parameters
        assert!(bg.tau0 > 14_000.0 && bg.tau0 < 14_400.0, "tau0 = {}", bg.tau0);
        assert!((bg.tau_of_a(bg.a_of_tau(300.0)) - 300.0).abs() < 1e-6);
        let th = Thermo::new(&bg);
        // values of the code CLASS (v3.4.2, recombination code HyRec) for the same parameters,
        // quoted in the author's NCSS project: 100 theta_s = 1.040374, r_drag = 147.1142 Mpc;
        // Planck 2018 quotes z_* = 1089.9 and z_drag = 1059.9 for almost the same model
        assert!((th.z_star - 1089.9).abs() < 0.5, "z_star = {}", th.z_star);
        assert!((th.z_drag - 1059.9).abs() < 0.5, "z_drag = {}", th.z_drag);
        assert!((th.rs_drag / 147.1142 - 1.0).abs() < 2e-4, "r_drag = {}", th.rs_drag);
        assert!((th.theta_s_100 / 1.040374 - 1.0).abs() < 1e-4, "100 theta_s = {}", th.theta_s_100);
        assert!((th.z_reio - 7.7).abs() < 0.6, "z_reio = {}", th.z_reio);
        assert!((th.y_he - 0.2454).abs() < 1e-9);
    }
}
