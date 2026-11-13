//! Linear cosmological perturbations on a deflating spacetime (Einstein-Boltzmann equations, synchronous gauge).
//!
//! DO NOT EDIT.  This file is written by code/scripts/vendor_eb.py of the revision.  It is the code of
//! boltzmann.rs (everything except its tests) of the main project (code/spacetimes-rs/src) with the following changes;
//! every other character is the original.  The main project itself is not changed.
//!   * the table `Fast` carries s = gsum (x4 - x4_today), the rates and the closure; `Fast::hubble_conf` is the conformal Hubble rate of the deflating model;
//!   * in `rhs`: 8 pi G a^2 rho of every species is multiplied by exp(s); the conformal Hubble rate and its derivative are those of the deflating model;
//!   * in `rhs`: the energy constraint (and the equation of the shear) use the rate `hub_e` = Hc - a gsum/2;
//!   * in `rhs`, closure "Pressure" with deflation: eta' follows from the trace equation instead of the momentum constraint;
//!   * in `rhs`, with deflation: alpha' is the right-hand side of the equation of the shear plus the shear gap, which the closure determines; `integrate_mode` adds the derivative of the gap to alpha'';
//!   * the derivative of the shear term contains the time dependence of Newton's constant;
//!   * in `tca_end`: the conformal Hubble rate of the deflating model;
//!   * for the checks: `Aux` also carries the Hubble rates and two background quantities, `ModeOut` the potential eta + alpha', the quantity eta' + alpha'', the three gaps (what the agent supplies) and two numerical checks, and `eval` gives access to `rhs`.
//! See the description at the head of vendor_eb.py and the provenance RP8 of the revision.
//!
//! THE EQUATIONS ON A DEFLATING SPACETIME (assumption A3 of the revision; derived in
//! code/scripts/verify_deflating_independent.py).  With He = Hc - a gsum/2, G = G_N(x4) = G_N(today) exp(s),
//! and unperturbed scale factors of the extra coordinates, the Einstein equations of ordinary space are
//!   (E)  k^2 eta - (1/2) He h' = - 4 pi G a^2 delta rho                         energy constraint
//!   (M)  k^2 eta' = 4 pi G a^2 (rho + P) theta                                  momentum constraint
//!   (S)  alpha' = - 2 He alpha + eta - 12 pi G a^2 (rho + P) sigma / k^2        equation of the shear
//!   (T)  h'' + 2 He h' - 2 k^2 eta = - 24 pi G a^2 delta P                      trace equation
//! with alpha = (h' + 6 eta')/(2 k^2).  The equations of matter and radiation are those of the main project
//! with the conformal Hubble rate Hc of the deflating model.  With matter and radiation alone on the right
//! the four equations are not compatible when G depends on the time; the agent that holds the extra scale
//! factors supplies the difference.  (E) always holds (the agent has no energy density).  Closure "Momentum":
//! (M) holds, as in the program of the main project.  Closure "Pressure": (T) holds; together with the
//! derivative of (E) and the conservation of matter it gives eta' algebraically (see `rhs`).  In neither
//! closure does (S) hold: alpha' is its right-hand side plus a gap that the closure determines (see `rhs`).
//! The description of the main project follows (read "4 pi G" as 4 pi G_N(x4), "H" in the equations of
//! matter and radiation as Hc, and "H" in the constraint h' = (2/H)(...) as He).
//!
//! Linear cosmological perturbations: the Einstein-Boltzmann equations in the
//! synchronous gauge, and the source functions of the line-of-sight integrals.
//!
//! EQUATIONS.  Ma and Bertschinger, Astrophys. J. 455, 7 (1995) [MB95], synchronous
//! gauge, conformal time tau, Fourier mode k.  Species: cold dark matter (delta_c),
//! baryons (delta_b, theta_b), photons (intensity multipoles F_l, polarization
//! multipoles G_l), massless neutrinos (F_nu,l), and the dark fluid of the model
//! (delta_d, theta_d; equation of state w, adiabatic sound speed c_s^2, no shear,
//! because the fluid is self-interacting).  Metric: eta and h' (h' follows from the
//! energy constraint; alpha = (h' + 6 eta')/(2 k^2)).
//!
//!   delta_c'  = -h'/2
//!   delta_b'  = -theta_b - h'/2
//!   theta_b'  = -H theta_b + c_b^2 k^2 delta_b + R kappa' (theta_g - theta_b),  R = 4 rho_g / (3 rho_b)
//!   delta_d'  = -(1+w)(theta_d + h'/2) - 3 H (c_s^2 - w) delta_d
//!   theta_d'  = -H (1 - 3 c_s^2) theta_d + c_s^2 k^2 delta_d / (1+w)
//!   F_0' = -k F_1 - (2/3) h'                         (F_0 = delta_g)
//!   F_1' = (k/3)(F_0 - 2 F_2) + (4/(3k)) kappa' (theta_b - theta_g)     (theta_g = (3/4) k F_1)
//!   F_2' = (k/5)(2 F_1 - 3 F_3) + (4/15) h' + (8/5) eta' - (9/10) kappa' F_2 + (1/10) kappa' (G_0 + G_2)
//!   F_l' = k/(2l+1) [l F_(l-1) - (l+1) F_(l+1)] - kappa' F_l                       (l >= 3)
//!   G_l' = k/(2l+1) [l G_(l-1) - (l+1) G_(l+1)] - kappa' G_l + (kappa'/2) Pi (delta_l0 + delta_l2/5)
//!   Pi = F_2 + G_0 + G_2
//!   neutrinos: the photon equations without scattering
//!   k^2 eta' = 4 pi G a^2 (rho + P) theta,     h' = (2/H) (k^2 eta + 4 pi G a^2 delta rho)
//! (H = a'/a is the conformal Hubble rate.)  Hierarchies are closed with the
//! free-streaming truncation of MB95, Eq. (51).  At early times the photon-baryon
//! fluid is treated in the tight-coupling approximation of MB95, Eqs. (74)-(76).
//!
//! INITIAL CONDITIONS: the adiabatic growing mode on super-horizon scales in the
//! radiation era [MB95 Eq. (96)], normalized to unit curvature perturbation.
//!
//! SOURCES.  The temperature multipoles today are
//!   Theta_l(k) = Int dtau [ S0 j_l(x) + S1 j_l'(x) + S2 (3 j_l''(x) + j_l(x))/2 ],   x = k (tau0 - tau),
//!   S0 = g (delta_g/4 + alpha') + e^-kappa (eta' + alpha''),
//!   S1 = g (theta_b/k + k alpha),
//!   S2 = g Pi / 8,
//! and the E-polarization multipoles are
//!   E_l(k) = sqrt((l+2)!/(l-2)!) Int dtau  (3 g Pi / 16) j_l(x) / x^2,
//! where g = kappa' e^-kappa is the visibility function.  S0 and S1 are the
//! conformal-Newtonian sources, written with synchronous-gauge variables
//! (psi = alpha' + H alpha, phi = eta - H alpha): this avoids the large cancellations
//! of a direct synchronous-gauge line-of-sight integral.

use crate::eb_cosmo::{Background, Closure, Thermo};
use spacetimes::cosmo::HUBBLE_LENGTH;

/// highest photon multipole (intensity and polarization)
pub const LG: usize = 12;
/// highest neutrino multipole
pub const LN: usize = 17;

const I_ETA: usize = 0;
const I_DC: usize = 1;
const I_DB: usize = 2;
const I_TB: usize = 3;
const I_DD: usize = 4;
const I_TD: usize = 5;
const I_FG: usize = 6;
const I_GG: usize = I_FG + LG + 1;
const I_FN: usize = I_GG + LG + 1;
pub const NVAR: usize = I_FN + LN + 1;

/// Background and thermodynamic quantities on a grid uniform in ln(tau), for fast lookup.
pub struct Fast {
    lt0: f64,
    dlt: f64,
    n: usize,
    a: Vec<f64>,
    ln_dk: Vec<f64>,
    kap: Vec<f64>,
    cb2: Vec<f64>,
    od: Vec<f64>,
    wd: Vec<f64>,
    cd2: Vec<f64>,
    /// s = gsum (x4 - x4_today): exp(s) is Newton's constant over its value today
    s: Vec<f64>,
    /// gsum in 1/Mpc, gprod in 1/Mpc^2, and the closure (true = "Pressure", false = "Momentum")
    pub gam_hat: f64,
    pub gprod_hat: f64,
    pub pressure: bool,
    pub omega_g: f64,
    pub omega_n: f64,
    pub omega_b: f64,
    /// cold collisionless dark matter (zero if the dark matter is a superfluid)
    pub omega_c: f64,
    /// superfluid dark matter today (zero if there is none); it occupies the dark-fluid slot
    pub omega_sf: f64,
    pub omega_l: f64,
    pub tau0: f64,
    /// first time of the table (Mpc)
    pub tau_min: f64,
}

/// Background quantities at one time.
#[derive(Clone, Copy)]
pub struct Point {
    pub a: f64,
    pub dkappa: f64,
    pub kappa: f64,
    pub cb2: f64,
    /// omega_d(a) a^4, equation of state and sound speed of the dark fluid
    pub od: f64,
    pub wd: f64,
    pub cd2: f64,
    /// s = gsum (x4 - x4_today)
    pub s: f64,
}

impl Fast {
    pub fn new(bg: &Background, th: &Thermo) -> Fast {
        Fast::with(bg, th, 1e-3, 24000)
    }

    /// Table that starts at conformal time `tau_min` (Mpc) and has `n` points.
    pub fn with(bg: &Background, th: &Thermo, tau_min: f64, n: usize) -> Fast {
        let (lt0, lt1) = (tau_min.ln(), bg.tau0.ln());
        let dlt = (lt1 - lt0) / (n - 1) as f64;
        let mut f = Fast {
            lt0,
            dlt,
            n,
            a: Vec::with_capacity(n),
            ln_dk: Vec::with_capacity(n),
            kap: Vec::with_capacity(n),
            cb2: Vec::with_capacity(n),
            od: Vec::with_capacity(n),
            wd: Vec::with_capacity(n),
            cd2: Vec::with_capacity(n),
            s: Vec::with_capacity(n),
            gam_hat: bg.d.gam / HUBBLE_LENGTH,
            gprod_hat: bg.d.gprod / (HUBBLE_LENGTH * HUBBLE_LENGTH),
            pressure: bg.d.closure == Closure::Pressure,
            omega_g: bg.omega_gamma,
            omega_n: bg.omega_nu,
            omega_b: bg.p.omega_b,
            omega_c: bg.omega_cdm,
            omega_sf: if bg.p.superfluid_dm.is_some() { bg.p.omega_c } else { 0.0 },
            omega_l: bg.omega_lambda,
            tau0: bg.tau0,
            tau_min,
        };
        for i in 0..n {
            let tau = (lt0 + dlt * i as f64).exp().min(bg.tau0);
            let a = bg.a_of_tau(tau);
            let (dk, kap, cb2) = th.at_tau(tau);
            let (od, wd, cd2) = bg.dark(a);
            f.a.push(a);
            f.ln_dk.push(dk.ln());
            f.kap.push(kap);
            f.cb2.push(cb2);
            f.od.push(od);
            f.wd.push(wd);
            f.cd2.push(cd2);
            f.s.push(bg.s_of_a(a));
        }
        f
    }

    #[inline]
    pub fn at(&self, tau: f64) -> Point {
        let t = ((tau.ln() - self.lt0) / self.dlt).max(0.0);
        let i = (t as usize).min(self.n - 2);
        let u = (t - i as f64).min(1.0);
        let l = |v: &Vec<f64>| v[i] + u * (v[i + 1] - v[i]);
        Point { a: l(&self.a), dkappa: l(&self.ln_dk).exp(), kappa: l(&self.kap), cb2: l(&self.cb2), od: l(&self.od), wd: l(&self.wd), cd2: l(&self.cd2), s: l(&self.s) }
    }

    /// The conformal Hubble rate Hc = a'/a (1/Mpc) at a point of the table: the positive root of
    /// Hc^2 - a gsum Hc + a^2 gprod/3 = (8 pi G_N(x4) a^2 / 3) (sum of the densities).
    #[inline]
    pub fn hubble_conf(&self, p: &Point) -> f64 {
        let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);
        let a2 = p.a * p.a;
        let gs = p.s.exp();
        let sum = gs * (c0 * ((self.omega_g + self.omega_n + p.od) / a2 + (self.omega_b + self.omega_c) / p.a + self.omega_l * a2));
        let ga = p.a * self.gam_hat;
        0.5 * ga + (0.25 * ga * ga - a2 * self.gprod_hat / 3.0 + sum / 3.0).sqrt()
    }
}

/// Quantities derived from the state at one time, needed for the sources.
#[derive(Clone, Copy, Default)]
pub struct Aux {
    pub hprime: f64,
    pub eta_prime: f64,
    pub alpha: f64,
    pub alpha_prime: f64,
    pub alpha_pp: f64,
    pub pi: f64,
    pub dkappa: f64,
    pub kappa: f64,
    pub delta_g: f64,
    pub theta_b: f64,
    pub delta_m: f64,
    /// for checks: the conformal Hubble rate Hc, its derivative, the rate He = Hc - a gsum/2 and its
    /// derivative, the background quantity B of the closure "Pressure", and (1/2) sum of grho (1 + w)
    pub hub: f64,
    pub hub_prime: f64,
    pub hub_e: f64,
    pub hub_e_prime: f64,
    pub b_trace: f64,
    pub half_rho_plus_p: f64,
    /// three times the pressure perturbation, 3 DP, and the gaps of the momentum constraint, of the trace equation and
    /// of the equation of the shear (what the agent supplies; see `rhs`)
    pub three_dpres: f64,
    pub momentum_gap: f64,
    pub trace_gap: f64,
    pub shear_gap: f64,
}

/// Right-hand side of the perturbation equations.
/// `tca`: tight-coupling approximation for the photon-baryon fluid;
/// `rad_off`: radiation-streaming approximation (late times, small scales, matter power only).
#[allow(clippy::too_many_arguments)]
fn rhs(f: &Fast, k: f64, tau: f64, y: &[f64], dy: &mut [f64], tca: bool, rad_off: bool) -> Aux {
    let p = f.at(tau);
    let a = p.a;
    let a2 = a * a;
    // 8 pi G_N(x4) a^2 rho of each species: Newton's constant of the moment, exp(s) times today's
    let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);
    let gs = p.s.exp();
    let grho_g = gs * (c0 * f.omega_g / a2);
    let grho_n = gs * (c0 * f.omega_n / a2);
    let grho_b = gs * (c0 * f.omega_b / a);
    let grho_c = gs * (c0 * f.omega_c / a);
    let grho_d = gs * (c0 * p.od / a2);
    let grho_l = gs * (c0 * f.omega_l * a2);
    // conformal Hubble rate: Hc^2 - a gsum Hc + a^2 gprod/3 = (sum of the grho)/3
    let ga = a * f.gam_hat;
    let grho_sum = grho_g + grho_n + grho_b + grho_c + grho_d + grho_l;
    let hub = 0.5 * ga + (0.25 * ga * ga - a2 * f.gprod_hat / 3.0 + grho_sum / 3.0).sqrt();
    // its derivative: the expression of the main project plus the terms of the deflation
    // (the derivative of exp(s) is a gsum exp(s), that of a is a Hc)
    let hub_prime_0 = -(2.0 * grho_g + 2.0 * grho_n + grho_b + grho_c + grho_d * (1.0 + 3.0 * p.wd) - 2.0 * grho_l) / 6.0;
    let hub_prime = hub_prime_0 + (ga * (hub_prime_0 + grho_sum / 3.0 + hub * hub) - 2.0 / 3.0 * f.gprod_hat * a2 * hub) / (2.0 * hub - ga);
    // the Hubble rate of the energy constraint, of the equation of the shear and of the trace equation in
    // eight dimensions with unperturbed scale factors of the extra coordinates: He = Hc - a gsum/2
    let hub_e = hub - 0.5 * ga;
    let hub_e_prime = hub_prime - 0.5 * ga * hub;
    // the background quantity of the closure "Pressure": He' - He^2 + (1/2) sum of grho (1 + w), which is
    // a gsum [ (1/6) sum of grho (1 - 3w) + (a gsum)^2/4 - a^2 gprod/3 ] / (2 He) by the equation of the expansion
    // (zero without deflation)
    let n_bg = (grho_b + grho_c + (1.0 - 3.0 * p.wd) * grho_d + 4.0 * grho_l) / 6.0 + 0.25 * ga * ga - a2 * f.gprod_hat / 3.0;
    let b_trace = ga * n_bg / (2.0 * hub_e);
    let pressure_closure = f.pressure && ga != 0.0;
    let k2 = k * k;

    let eta = y[I_ETA];
    let (dc, db, tb) = (y[I_DC], y[I_DB], y[I_TB]);
    let (dd, td) = (y[I_DD], y[I_TD]);
    let (dg, tg) = (y[I_FG], 0.75 * k * y[I_FG + 1]);
    let (dn, tn, sn) = (y[I_FN], 0.75 * k * y[I_FN + 1], 0.5 * y[I_FN + 2]);

    // Einstein constraints.
    let rad = if rad_off { 0.0 } else { 1.0 };
    let (hprime, eta_prime, alpha);
    // D = sum of grho delta, three times the pressure perturbation 3 DP = 3 (sum of grho c_s^2 delta), and
    // X = h' B - (a gsum/2)(D - 3 DP)  (see the gaps below); they stay zero with streaming radiation
    let (mut d_sum, mut three_dpres, mut x_gap) = (0.0, 0.0, 0.0);
    if rad_off {
        // RADIATION-STREAMING APPROXIMATION (late times, wavelengths far inside the
        // horizon).  After decoupling the radiation streams freely and is, on average,
        // unperturbed and at rest in the conformal Newtonian frame, up to terms of the
        // order of the gravitational potentials, which are smaller than the matter
        // density contrast by the factor (H/k)^2.  The synchronous frame moves with the
        // cold dark matter, which falls into the potential wells.  Seen from it the
        // radiation therefore has the bulk velocity and the density contrast
        //     theta_r = - k^2 alpha ,     delta_r = 3 H (1 + w_r) alpha ,
        // (gauge transformation with the time shift alpha).  These terms must be kept:
        // the momentum density of the radiation is NOT small compared with that of the
        // baryons, which move with the dark matter.  Inserting them into the constraints
        // gives alpha from the matter terms alone, and the corrections below.
        // (a superfluid in the dark-fluid slot is matter and is kept; dark radiation streams)
        let sf = f.omega_sf > 0.0;
        let g1 = 4.0 / 3.0 * (grho_g + grho_n) + if sf { 0.0 } else { (1.0 + p.wd) * grho_d };
        let drho_m = grho_c * dc + grho_b * db + if sf { grho_d * dd } else { 0.0 };
        let rho_theta_m = grho_b * tb + if sf { grho_d * (1.0 + p.wd) * td } else { 0.0 };
        let hp_m = 2.0 * (k2 * eta + 0.5 * drho_m) / hub_e;
        // (the density contrast of the streaming radiation, 3 Hc (1 + w) alpha, contains the true rate Hc)
        let hr = hub / hub_e;
        if pressure_closure {
            // closure "Pressure" (see below) with streaming radiation: h' = hp_m + c_h alpha and
            // 2 k^2 eta' = a_0 + a_1 alpha, so that alpha = (h' + 6 eta')/(2 k^2) solves a linear equation.
            // The pressure perturbation of the streaming radiation is one third of its density perturbation
            // (c_s^2 times it for a dark fluid that streams).
            let gp1 = 4.0 / 9.0 * (grho_g + grho_n) + if sf { 0.0 } else { p.cd2 * (1.0 + p.wd) * grho_d };
            let dpres_m = if sf { p.cd2 * grho_d * dd } else { 0.0 };
            let c_h = 3.0 * g1 * hr;
            let a_0 = rho_theta_m + hp_m * b_trace - 0.5 * ga * (drho_m - 3.0 * dpres_m);
            let a_1 = -g1 * k2 + c_h * b_trace - 1.5 * ga * hub * (g1 - 3.0 * gp1);
            alpha = (hp_m + 3.0 * a_0 / k2) / (2.0 * k2 - c_h - 3.0 * a_1 / k2);
            hprime = hp_m + c_h * alpha;
            eta_prime = 0.5 * (a_0 + a_1 * alpha) / k2;
        } else {
            alpha = (hp_m + 3.0 * rho_theta_m / k2) / (2.0 * k2 - 3.0 * g1 * (hr - 1.0));
            hprime = hp_m + 3.0 * alpha * g1 * hr;
            eta_prime = 0.5 * (rho_theta_m - k2 * alpha * g1) / k2;
        }
    } else {
        let drho = grho_c * dc + grho_b * db + grho_g * dg + grho_n * dn + grho_d * dd;
        let rho_theta = grho_b * tb + 4.0 / 3.0 * (grho_g * tg + grho_n * tn) + grho_d * (1.0 + p.wd) * td;
        hprime = 2.0 * (k2 * eta + 0.5 * drho) / hub_e;
        // (the pressure of the baryons is neglected in DP, as in their equation of continuity)
        d_sum = drho;
        three_dpres = grho_g * dg + grho_n * dn + 3.0 * p.cd2 * grho_d * dd;
        x_gap = hprime * b_trace - 0.5 * ga * (drho - three_dpres);
        if pressure_closure {
            // closure "Pressure": the trace equation, h'' + 2 He h' - 2 k^2 eta = -3 DP, holds in the place of
            // the momentum constraint.  The derivative of the energy constraint, the conservation of matter
            // and the trace equation give  2 k^2 eta' = T + X,  with T = sum of grho (1 + w) theta.
            eta_prime = 0.5 * (rho_theta + x_gap) / k2;
        } else {
            // closure "Momentum" (and general relativity): the momentum constraint
            eta_prime = 0.5 * rho_theta / k2;
        }
        alpha = (hprime + 6.0 * eta_prime) / (2.0 * k2);
    }

    for v in dy.iter_mut() {
        *v = 0.0;
    }
    dy[I_ETA] = eta_prime;
    dy[I_DC] = -0.5 * hprime;
    dy[I_DB] = -tb - 0.5 * hprime;

    let dkappa = p.dkappa;
    let r = 4.0 * grho_g / (3.0 * grho_b);
    let sigma_g;
    let pi;
    let mut dsigma_g = 0.0;
    if rad_off {
        // baryons: pressureless, decoupled
        dy[I_TB] = -hub * tb + p.cb2 * k2 * db;
        if f.omega_sf > 0.0 {
            // superfluid dark matter: the fluid equations stay on
            dy[I_DD] = -(1.0 + p.wd) * (td + 0.5 * hprime) - 3.0 * hub * (p.cd2 - p.wd) * dd;
            dy[I_TD] = -hub * (1.0 - 3.0 * p.cd2) * td + p.cd2 * k2 * dd / (1.0 + p.wd);
        }
        sigma_g = 0.0;
        pi = 0.0;
    } else {
        // dark fluid
        dy[I_DD] = -(1.0 + p.wd) * (td + 0.5 * hprime) - 3.0 * hub * (p.cd2 - p.wd) * dd;
        dy[I_TD] = -hub * (1.0 - 3.0 * p.cd2) * td + p.cd2 * k2 * dd / (1.0 + p.wd);
        // neutrinos
        dy[I_FN] = -k * y[I_FN + 1] - 2.0 / 3.0 * hprime;
        dy[I_FN + 1] = k / 3.0 * (y[I_FN] - 2.0 * y[I_FN + 2]);
        dy[I_FN + 2] = k / 5.0 * (2.0 * y[I_FN + 1] - 3.0 * y[I_FN + 3]) + 4.0 / 15.0 * hprime + 8.0 / 5.0 * eta_prime;
        for l in 3..LN {
            dy[I_FN + l] = k / (2.0 * l as f64 + 1.0) * (l as f64 * y[I_FN + l - 1] - (l as f64 + 1.0) * y[I_FN + l + 1]);
        }
        dy[I_FN + LN] = k * y[I_FN + LN - 1] - (LN as f64 + 1.0) / tau * y[I_FN + LN];

        if tca {
            let tauc = 1.0 / dkappa;
            sigma_g = 16.0 / 45.0 * tauc * (tg + k2 * alpha);
            pi = 5.0 * sigma_g;
            let ddg = -4.0 / 3.0 * tg - 2.0 / 3.0 * hprime;
            let ddb = dy[I_DB];
            let app_over_a = hub * hub + hub_prime;
            let slip = 2.0 * r / (1.0 + r) * hub * (tb - tg) + tauc / (1.0 + r) * (-app_over_a * tb - hub * k2 * dg / 2.0 + k2 * (p.cb2 * ddb - ddg / 4.0));
            let dtb = (-hub * tb + p.cb2 * k2 * db + r * k2 * (dg / 4.0 - sigma_g) + r * slip) / (1.0 + r);
            let dtg = -(dtb + hub * tb - p.cb2 * k2 * db) / r + k2 * (dg / 4.0 - sigma_g);
            dy[I_TB] = dtb;
            dy[I_FG] = ddg;
            dy[I_FG + 1] = 4.0 / (3.0 * k) * dtg;
        } else {
            sigma_g = 0.5 * y[I_FG + 2];
            pi = y[I_FG + 2] + y[I_GG] + y[I_GG + 2];
            dy[I_TB] = -hub * tb + p.cb2 * k2 * db + r * dkappa * (tg - tb);
            dy[I_FG] = -k * y[I_FG + 1] - 2.0 / 3.0 * hprime;
            dy[I_FG + 1] = k / 3.0 * (y[I_FG] - 2.0 * y[I_FG + 2]) + 4.0 / (3.0 * k) * dkappa * (tb - tg);
            dy[I_FG + 2] = k / 5.0 * (2.0 * y[I_FG + 1] - 3.0 * y[I_FG + 3]) + 4.0 / 15.0 * hprime + 8.0 / 5.0 * eta_prime - 0.9 * dkappa * y[I_FG + 2] + 0.1 * dkappa * (y[I_GG] + y[I_GG + 2]);
            for l in 3..LG {
                dy[I_FG + l] = k / (2.0 * l as f64 + 1.0) * (l as f64 * y[I_FG + l - 1] - (l as f64 + 1.0) * y[I_FG + l + 1]) - dkappa * y[I_FG + l];
            }
            dy[I_FG + LG] = k * y[I_FG + LG - 1] - ((LG as f64 + 1.0) / tau + dkappa) * y[I_FG + LG];
            // polarization
            dy[I_GG] = -k * y[I_GG + 1] - dkappa * y[I_GG] + 0.5 * dkappa * pi;
            dy[I_GG + 1] = k / 3.0 * (y[I_GG] - 2.0 * y[I_GG + 2]) - dkappa * y[I_GG + 1];
            dy[I_GG + 2] = k / 5.0 * (2.0 * y[I_GG + 1] - 3.0 * y[I_GG + 3]) - dkappa * y[I_GG + 2] + 0.1 * dkappa * pi;
            for l in 3..LG {
                dy[I_GG + l] = k / (2.0 * l as f64 + 1.0) * (l as f64 * y[I_GG + l - 1] - (l as f64 + 1.0) * y[I_GG + l + 1]) - dkappa * y[I_GG + l];
            }
            dy[I_GG + LG] = k * y[I_GG + LG - 1] - ((LG as f64 + 1.0) / tau + dkappa) * y[I_GG + LG];
            dsigma_g = 0.5 * dy[I_FG + 2];
        }
    }

    // alpha' and alpha'' (for the sources)
    let shear = rad * 4.0 / 3.0 * (grho_g * sigma_g + grho_n * sn);
    // THE GAPS: what the agent supplies (all zero without deflation).  Define
    //     momentum gap = 2 k^2 eta' - T ,     trace gap = h'' + 2 He h' - 2 k^2 eta + 3 DP ,
    //     shear gap = alpha' - (right-hand side of the equation of the shear) ,
    // and X = h' B - (a gsum/2)(D - 3 DP).  The energy constraint and the conservation of matter give two identities,
    //     momentum gap = He (trace gap) + X ,
    //     2 k^2 (shear gap) = (trace gap) + (3/k^2) [ (momentum gap)' + 2 He (momentum gap) ] .
    // Closure "Momentum": momentum gap = 0, trace gap = -X/He, shear gap = -X/(2 k^2 He).
    // Closure "Pressure": trace gap = 0, momentum gap = X, shear gap = (3/(2 k^4)) (X' + 2 He X), where X' follows
    // from the equations of motion: (grho)' = (a gsum - (1 + 3w) Hc) grho for each species, h'' from the trace
    // equation, and B' from B = a gsum N/(2 He).
    let (mut momentum_gap, mut trace_gap, mut shear_gap) = (0.0, 0.0, 0.0);
    if ga != 0.0 && !rad_off {
        if pressure_closure {
            momentum_gap = x_gap;
            let (c_m, c_r, c_d) = (ga - hub, ga - 2.0 * hub, ga - (1.0 + 3.0 * p.wd) * hub);
            let d_prime = c_m * (grho_c * dc + grho_b * db) + c_r * (grho_g * dg + grho_n * dn) + c_d * grho_d * dd
                + grho_c * dy[I_DC] + grho_b * dy[I_DB] + grho_g * dy[I_FG] + grho_n * dy[I_FN] + grho_d * dy[I_DD];
            // (the derivative of the sound speed of the dark fluid is taken from the table)
            let cd2_prime = (f.at(tau * (1.0 + 1e-4)).cd2 - f.at(tau * (1.0 - 1e-4)).cd2) / (2e-4 * tau);
            let three_dpres_prime = c_r * (grho_g * dg + grho_n * dn) + grho_g * dy[I_FG] + grho_n * dy[I_FN]
                + 3.0 * (cd2_prime * grho_d * dd + p.cd2 * (c_d * grho_d * dd + grho_d * dy[I_DD]));
            // (the derivative of the equation of state of the dark fluid is 3 Hc (1 + w)(w - c_s^2))
            let wd_prime = 3.0 * hub * (1.0 + p.wd) * (p.wd - p.cd2);
            let n_prime = (c_m * (grho_b + grho_c) + (1.0 - 3.0 * p.wd) * c_d * grho_d - 3.0 * wd_prime * grho_d + 4.0 * (ga + 2.0 * hub) * grho_l) / 6.0
                + 0.5 * ga * ga * hub - 2.0 / 3.0 * a2 * f.gprod_hat * hub;
            let b_prime = ga * (hub * n_bg + n_prime) / (2.0 * hub_e) - b_trace * hub_e_prime / hub_e;
            let hpp = -hub_e * hprime - d_sum - three_dpres;
            let x_prime = hpp * b_trace + hprime * b_prime - 0.5 * ga * hub * (d_sum - three_dpres) - 0.5 * ga * (d_prime - three_dpres_prime);
            shear_gap = 1.5 * (x_prime + 2.0 * hub_e * x_gap) / (k2 * k2);
        } else {
            trace_gap = -x_gap / hub_e;
            shear_gap = 0.5 * trace_gap / k2;
        }
    }
    // alpha' = (right-hand side of the equation of the shear) + (shear gap); alpha'' is its derivative without
    // the derivative of the gap, which `integrate_mode` adds
    let alpha_prime_s = -2.0 * hub_e * alpha + eta - 1.5 * shear / k2;
    let alpha_prime = if ga != 0.0 { alpha_prime_s + shear_gap } else { alpha_prime_s };
    let dsn = 0.5 * dy[I_FN + 2];
    // (the derivative of the shear term contains the time dependence of Newton's constant: a gsum times the term)
    let dshear = rad * 4.0 / 3.0 * (grho_g * (dsigma_g - 2.0 * hub * sigma_g) + grho_n * (dsn - 2.0 * hub * sn)) + ga * shear;
    let alpha_pp = -2.0 * hub_e_prime * alpha - 2.0 * hub_e * alpha_prime + eta_prime - 1.5 * dshear / k2;

    Aux {
        hprime,
        eta_prime,
        alpha,
        alpha_prime,
        alpha_pp,
        pi,
        dkappa,
        kappa: p.kappa,
        delta_g: dg,
        theta_b: tb,
        delta_m: (f.omega_c * dc + f.omega_sf * dd + f.omega_b * db) / (f.omega_c + f.omega_sf + f.omega_b),
        hub,
        hub_prime,
        hub_e,
        hub_e_prime,
        b_trace,
        half_rho_plus_p: 0.5 * (4.0 / 3.0 * (grho_g + grho_n) + grho_b + grho_c + (1.0 + p.wd) * grho_d),
        three_dpres,
        momentum_gap,
        trace_gap,
        shear_gap,
    }
}

/// The right-hand side and the derived quantities at one time, for a given state (for checks).
pub fn eval(f: &Fast, k: f64, tau: f64, y: &[f64], tca: bool) -> (Vec<f64>, Aux) {
    let mut dy = vec![0.0; NVAR];
    let aux = rhs(f, k, tau, y, &mut dy, tca, false);
    (dy, aux)
}

/// One adaptive Dormand-Prince 5(4) integration from t0 to t1.
/// `h` is the suggested first step and returns the last successful step size.
#[allow(clippy::too_many_arguments)]
fn integrate<F: FnMut(f64, &[f64], &mut [f64])>(mut f: F, y: &mut [f64], t0: f64, t1: f64, h: &mut f64, rtol: f64, nscale: usize) {
    let n = y.len();
    let mut k = vec![vec![0.0; n]; 7];
    let mut yt = vec![0.0; n];
    let mut t = t0;
    let mut step = h.min(t1 - t0);
    if step <= 0.0 {
        return;
    }
    f(t, y, &mut k[0]);
    let mut guard = 0u64;
    loop {
        guard += 1;
        assert!(guard < 50_000_000, "integration did not finish");
        let last = t + step >= t1 * (1.0 - 1e-15);
        if last {
            step = t1 - t;
        }
        // stages
        for i in 0..n {
            yt[i] = y[i] + step * (1.0 / 5.0) * k[0][i];
        }
        f(t + step / 5.0, &yt, &mut k[1]);
        for i in 0..n {
            yt[i] = y[i] + step * (3.0 / 40.0 * k[0][i] + 9.0 / 40.0 * k[1][i]);
        }
        f(t + 0.3 * step, &yt, &mut k[2]);
        for i in 0..n {
            yt[i] = y[i] + step * (44.0 / 45.0 * k[0][i] - 56.0 / 15.0 * k[1][i] + 32.0 / 9.0 * k[2][i]);
        }
        f(t + 0.8 * step, &yt, &mut k[3]);
        for i in 0..n {
            yt[i] = y[i] + step * (19372.0 / 6561.0 * k[0][i] - 25360.0 / 2187.0 * k[1][i] + 64448.0 / 6561.0 * k[2][i] - 212.0 / 729.0 * k[3][i]);
        }
        f(t + 8.0 / 9.0 * step, &yt, &mut k[4]);
        for i in 0..n {
            yt[i] = y[i] + step * (9017.0 / 3168.0 * k[0][i] - 355.0 / 33.0 * k[1][i] + 46732.0 / 5247.0 * k[2][i] + 49.0 / 176.0 * k[3][i] - 5103.0 / 18656.0 * k[4][i]);
        }
        f(t + step, &yt, &mut k[5]);
        for i in 0..n {
            yt[i] = y[i] + step * (35.0 / 384.0 * k[0][i] + 500.0 / 1113.0 * k[2][i] + 125.0 / 192.0 * k[3][i] - 2187.0 / 6784.0 * k[4][i] + 11.0 / 84.0 * k[5][i]);
        }
        {
            let (head, tail) = k.split_at_mut(6);
            let _ = head;
            f(t + step, &yt, &mut tail[0]);
        }
        // error estimate; the tolerance of each variable is relative to its own size
        // plus a fraction of the largest of the first `nscale` (non-hierarchy) variables
        let mut big: f64 = 0.0;
        for i in 0..nscale {
            big = big.max(yt[i].abs());
        }
        let mut err: f64 = 0.0;
        for i in 0..n {
            let e = step * (71.0 / 57600.0 * k[0][i] - 71.0 / 16695.0 * k[2][i] + 71.0 / 1920.0 * k[3][i] - 17253.0 / 339200.0 * k[4][i] + 22.0 / 525.0 * k[5][i] - 1.0 / 40.0 * k[6][i]);
            let sc = rtol * (yt[i].abs().max(y[i].abs()) + 0.01 * big + 1e-300);
            err = err.max((e / sc).abs());
        }
        if err <= 1.0 {
            t += step;
            y.copy_from_slice(&yt);
            // first-same-as-last
            let (head, tail) = k.split_at_mut(6);
            head[0].copy_from_slice(&tail[0]);
            *h = step;
            if last {
                break;
            }
            let grow = (0.9 * err.max(1e-10).powf(-0.2)).min(4.0);
            step *= grow;
        } else {
            step *= (0.9 * err.powf(-0.2)).max(0.2);
        }
    }
}

/// Result of the integration of one Fourier mode.
pub struct ModeOut {
    pub k: f64,
    /// sources at the output times: S0, S1, S2 and the polarization source 3 g Pi / 16
    pub s0: Vec<f64>,
    pub s1: Vec<f64>,
    pub s2: Vec<f64>,
    pub se: Vec<f64>,
    /// matter density contrast (cdm + baryons, synchronous gauge) today
    pub delta_m0: f64,
    /// at the output times: the sum of the two gravitational potentials, eta + alpha', and the
    /// quantity eta' + alpha'' that the source S0 uses for its derivative; the three gaps (what the
    /// agent supplies: see `rhs`); and two numerical checks of the identities behind them, which
    /// vanish up to the error of a numerical derivative: alpha' by a difference along the solution
    /// minus alpha', and the trace equation with h'' by a difference minus the trace gap
    pub pot: Vec<f64>,
    pub dpot: Vec<f64>,
    pub shear_gap: Vec<f64>,
    pub trace_gap: Vec<f64>,
    pub momentum_gap: Vec<f64>,
    pub shear_check: Vec<f64>,
    pub trace_check: Vec<f64>,
}

/// Time at which tight coupling ends for the mode k: max(k, H) / kappa' = eps.
fn tca_end(f: &Fast, k: f64, tau_ini: f64, eps: f64) -> f64 {
    let mut tau = tau_ini;
    // geometric scan, then bisection
    let cond = |tau: f64| {
        let p = f.at(tau);
        let hub = f.hubble_conf(&p);
        k.max(hub) / p.dkappa > eps
    };
    if cond(tau) {
        return tau;
    }
    let mut prev = tau;
    while tau < f.tau0 {
        tau *= 1.03;
        if cond(tau) {
            let (mut lo, mut hi) = (prev, tau);
            for _ in 0..30 {
                let mid = 0.5 * (lo + hi);
                if cond(mid) {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            return hi;
        }
        prev = tau;
    }
    f.tau0
}

/// Integrate the mode k and return the sources at the times `tau_out` (ascending,
/// the last one equal to tau0).  With `matter_only` the sources are not computed
/// at late times: after tau = 800 Mpc the hierarchies of the radiation are replaced
/// by the radiation-streaming approximation (see `rhs`), which is adequate (and much
/// faster) for the matter power spectrum at large k.
pub fn integrate_mode(f: &Fast, k: f64, tau_out: &[f64], rtol: f64, matter_only: bool) -> ModeOut {
    let tau_ini = (0.02 / k).min(0.3).max(2.0 * f.tau_min);
    // free-streaming fraction of the radiation
    let p0 = f.at(tau_ini);
    let r_nu = f.omega_n / (f.omega_g + f.omega_n + p0.od);
    let kt = k * tau_ini;
    let kt2 = kt * kt;
    let mut y = vec![0.0; NVAR];
    // MB95 Eq. (96) with C = 1/2 (unit curvature perturbation)
    let dg = -kt2 / 3.0;
    let tg = -k * kt2 * kt / 36.0;
    y[I_ETA] = 1.0 - (5.0 + 4.0 * r_nu) / (12.0 * (15.0 + 4.0 * r_nu)) * kt2;
    y[I_DC] = 0.75 * dg;
    y[I_DB] = 0.75 * dg;
    y[I_TB] = tg;
    y[I_DD] = 0.75 * (1.0 + p0.wd) * dg;
    // velocity of a perfect fluid with equation of state w in the adiabatic mode:
    // theta = theta_gamma 9w/(4 - 3w)  (theta_gamma for w = 1/3, zero for pressureless matter)
    y[I_TD] = tg * 9.0 * p0.wd / (4.0 - 3.0 * p0.wd);
    y[I_FG] = dg;
    y[I_FG + 1] = 4.0 / (3.0 * k) * tg;
    y[I_FN] = dg;
    y[I_FN + 1] = 4.0 / (3.0 * k) * tg * (23.0 + 4.0 * r_nu) / (15.0 + 4.0 * r_nu);
    y[I_FN + 2] = 2.0 * (2.0 * kt2 / (3.0 * (15.0 + 4.0 * r_nu)));

    let tau_sw = tca_end(f, k, tau_ini, 0.008);
    // (for k > 3.5/Mpc, beyond the wavenumbers used for sigma_8, the switch is made earlier:
    // these modes are deep inside the horizon soon after recombination)
    let tau_off = if matter_only { if k > 3.5 { 450.0 } else { 800.0 } } else { f64::INFINITY };
    let n_out = tau_out.len();
    let mut out = ModeOut { k, s0: vec![0.0; n_out], s1: vec![0.0; n_out], s2: vec![0.0; n_out], se: vec![0.0; n_out], delta_m0: 0.0, pot: vec![0.0; n_out], dpot: vec![0.0; n_out], shear_gap: vec![0.0; n_out], trace_gap: vec![0.0; n_out], momentum_gap: vec![0.0; n_out], shear_check: vec![0.0; n_out], trace_check: vec![0.0; n_out] };
    let mut t = tau_ini;
    let mut h = 0.1 * tau_ini;
    let mut tca = true;
    let mut rad_off = false;
    let mut dy = vec![0.0; NVAR];
    for (j, &tj) in tau_out.iter().enumerate() {
        // advance to tj, switching regimes on the way
        while t < tj {
            let mut stop = tj;
            if tca && tau_sw < stop {
                stop = tau_sw.max(t);
            }
            if !rad_off && tau_off < stop {
                stop = tau_off.max(t);
            }
            if stop > t {
                let (tc, ro) = (tca, rad_off);
                integrate(|tt, yy, dd| { rhs(f, k, tt, yy, dd, tc, ro); }, &mut y, t, stop, &mut h, rtol, I_FG + 1);
                t = stop;
            }
            if tca && t >= tau_sw {
                // leave tight coupling: initialize the photon quadrupole and polarization
                let aux = rhs(f, k, t, &y, &mut dy, true, false);
                let sigma = aux.pi / 5.0;
                y[I_FG + 2] = 2.0 * sigma;
                y[I_GG] = 2.5 * sigma;
                y[I_GG + 2] = 0.5 * sigma;
                tca = false;
                h = h.min(0.1 / aux.dkappa);
            }
            if !rad_off && t >= tau_off {
                rad_off = true;
            }
        }
        let mut aux = rhs(f, k, tj, &y, &mut dy, tca, rad_off);
        let (mut shear_check, mut trace_check) = (0.0, 0.0);
        if f.gam_hat != 0.0 && !rad_off {
            // With deflation alpha'' lacks the derivative of the shear gap.  It is taken along the solution: a
            // difference over -eps and +eps with the state moved along its derivative (at the last time, today,
            // where the table ends: a one-sided difference).  The same differences of alpha and of h' check the
            // two identities of `rhs` numerically.
            let eps = 1e-4 * (1.0 / k).min(tj);
            let mut yy = vec![0.0; NVAR];
            let mut scratch = vec![0.0; NVAR];
            let mut at = |m: f64| -> Aux {
                for i in 0..NVAR {
                    yy[i] = y[i] + m * eps * dy[i];
                }
                rhs(f, k, tj + m * eps, &yy, &mut scratch, tca, rad_off)
            };
            let (gap_prime, alpha_num, hpp_num);
            if tj + eps <= f.tau0 {
                let (xp, xm) = (at(1.0), at(-1.0));
                gap_prime = (xp.shear_gap - xm.shear_gap) / (2.0 * eps);
                alpha_num = (xp.alpha - xm.alpha) / (2.0 * eps);
                hpp_num = (xp.hprime - xm.hprime) / (2.0 * eps);
            } else {
                let (x1, x2) = (at(-1.0), at(-2.0));
                gap_prime = (3.0 * aux.shear_gap - 4.0 * x1.shear_gap + x2.shear_gap) / (2.0 * eps);
                alpha_num = (3.0 * aux.alpha - 4.0 * x1.alpha + x2.alpha) / (2.0 * eps);
                hpp_num = (3.0 * aux.hprime - 4.0 * x1.hprime + x2.hprime) / (2.0 * eps);
            }
            aux.alpha_pp += gap_prime;
            shear_check = alpha_num - aux.alpha_prime;
            trace_check = hpp_num + 2.0 * aux.hub_e * aux.hprime - 2.0 * k * k * y[I_ETA] + aux.three_dpres - aux.trace_gap;
        }
        let ek = (-aux.kappa).exp();
        let g = aux.dkappa * ek;
        out.s0[j] = g * (0.25 * aux.delta_g + aux.alpha_prime) + ek * (aux.eta_prime + aux.alpha_pp);
        out.s1[j] = g * (aux.theta_b / k + k * aux.alpha);
        out.s2[j] = g * aux.pi / 8.0;
        out.se[j] = 3.0 * g * aux.pi / 16.0;
        out.pot[j] = y[I_ETA] + aux.alpha_prime;
        out.dpot[j] = aux.eta_prime + aux.alpha_pp;
        out.shear_gap[j] = aux.shear_gap;
        out.trace_gap[j] = aux.trace_gap;
        out.momentum_gap[j] = aux.momentum_gap;
        out.shear_check[j] = shear_check;
        out.trace_check[j] = trace_check;
        if j == n_out - 1 {
            out.delta_m0 = aux.delta_m;
        }
    }
    out
}

/// The time grid of the sources: fine through recombination, then coarser.
pub fn source_times(bg: &Background) -> Vec<f64> {
    let t_start = bg.tau_of_a(1.0 / 2301.0);
    let t_fine_end = bg.tau_of_a(1.0 / 601.0);
    let mut t = vec![t_start];
    let mut cur = t_start;
    while cur < t_fine_end {
        cur += 1.0;
        t.push(cur);
    }
    let mut dt = 1.0;
    while cur < bg.tau0 {
        dt = (dt * 1.04f64).min(25.0);
        cur += dt;
        if cur >= bg.tau0 - 0.5 * dt {
            cur = bg.tau0;
        }
        t.push(cur);
    }
    t
}
