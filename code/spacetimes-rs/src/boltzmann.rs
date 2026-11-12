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

use crate::cosmo::{Background, Thermo, HUBBLE_LENGTH};

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
        }
        f
    }

    #[inline]
    pub fn at(&self, tau: f64) -> Point {
        let t = ((tau.ln() - self.lt0) / self.dlt).max(0.0);
        let i = (t as usize).min(self.n - 2);
        let u = (t - i as f64).min(1.0);
        let l = |v: &Vec<f64>| v[i] + u * (v[i + 1] - v[i]);
        Point { a: l(&self.a), dkappa: l(&self.ln_dk).exp(), kappa: l(&self.kap), cb2: l(&self.cb2), od: l(&self.od), wd: l(&self.wd), cd2: l(&self.cd2) }
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
}

/// Right-hand side of the perturbation equations.
/// `tca`: tight-coupling approximation for the photon-baryon fluid;
/// `rad_off`: radiation-streaming approximation (late times, small scales, matter power only).
#[allow(clippy::too_many_arguments)]
fn rhs(f: &Fast, k: f64, tau: f64, y: &[f64], dy: &mut [f64], tca: bool, rad_off: bool) -> Aux {
    let p = f.at(tau);
    let a = p.a;
    let a2 = a * a;
    let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);
    let grho_g = c0 * f.omega_g / a2;
    let grho_n = c0 * f.omega_n / a2;
    let grho_b = c0 * f.omega_b / a;
    let grho_c = c0 * f.omega_c / a;
    let grho_d = c0 * p.od / a2;
    let grho_l = c0 * f.omega_l * a2;
    let hub = ((grho_g + grho_n + grho_b + grho_c + grho_d + grho_l) / 3.0).sqrt();
    let hub_prime = -(2.0 * grho_g + 2.0 * grho_n + grho_b + grho_c + grho_d * (1.0 + 3.0 * p.wd) - 2.0 * grho_l) / 6.0;
    let k2 = k * k;

    let eta = y[I_ETA];
    let (dc, db, tb) = (y[I_DC], y[I_DB], y[I_TB]);
    let (dd, td) = (y[I_DD], y[I_TD]);
    let (dg, tg) = (y[I_FG], 0.75 * k * y[I_FG + 1]);
    let (dn, tn, sn) = (y[I_FN], 0.75 * k * y[I_FN + 1], 0.5 * y[I_FN + 2]);

    // Einstein constraints.
    let rad = if rad_off { 0.0 } else { 1.0 };
    let (hprime, eta_prime, alpha);
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
        let hp_m = 2.0 * (k2 * eta + 0.5 * drho_m) / hub;
        alpha = (hp_m + 3.0 * rho_theta_m / k2) / (2.0 * k2);
        hprime = hp_m + 3.0 * alpha * g1;
        eta_prime = 0.5 * (rho_theta_m - k2 * alpha * g1) / k2;
    } else {
        let drho = grho_c * dc + grho_b * db + grho_g * dg + grho_n * dn + grho_d * dd;
        let rho_theta = grho_b * tb + 4.0 / 3.0 * (grho_g * tg + grho_n * tn) + grho_d * (1.0 + p.wd) * td;
        hprime = 2.0 * (k2 * eta + 0.5 * drho) / hub;
        eta_prime = 0.5 * rho_theta / k2;
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
    let alpha_prime = -2.0 * hub * alpha + eta - 1.5 * shear / k2;
    let dsn = 0.5 * dy[I_FN + 2];
    let dshear = rad * 4.0 / 3.0 * (grho_g * (dsigma_g - 2.0 * hub * sigma_g) + grho_n * (dsn - 2.0 * hub * sn));
    let alpha_pp = -2.0 * hub_prime * alpha - 2.0 * hub * alpha_prime + eta_prime - 1.5 * dshear / k2;

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
    }
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
}

/// Time at which tight coupling ends for the mode k: max(k, H) / kappa' = eps.
fn tca_end(f: &Fast, k: f64, tau_ini: f64, eps: f64) -> f64 {
    let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);
    let mut tau = tau_ini;
    // geometric scan, then bisection
    let cond = |tau: f64| {
        let p = f.at(tau);
        let a2 = p.a * p.a;
        let hub = (c0 * ((f.omega_g + f.omega_n + p.od) / a2 + (f.omega_b + f.omega_c) / p.a + f.omega_l * a2) / 3.0).sqrt();
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
    let mut out = ModeOut { k, s0: vec![0.0; n_out], s1: vec![0.0; n_out], s2: vec![0.0; n_out], se: vec![0.0; n_out], delta_m0: 0.0 };
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
        let aux = rhs(f, k, tj, &y, &mut dy, tca, rad_off);
        let ek = (-aux.kappa).exp();
        let g = aux.dkappa * ek;
        out.s0[j] = g * (0.25 * aux.delta_g + aux.alpha_prime) + ek * (aux.eta_prime + aux.alpha_pp);
        out.s1[j] = g * (aux.theta_b / k + k * aux.alpha);
        out.s2[j] = g * aux.pi / 8.0;
        out.se[j] = 3.0 * g * aux.pi / 16.0;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmo::Params;

    /// Super-horizon evolution: for k tau << 1 the curvature variable eta stays at its
    /// initial value, and deep in the matter era the potential phi = eta - H alpha
    /// tends to 3/5 of the curvature perturbation for a mode that enters late.
    #[test]
    fn superhorizon_conservation() {
        let bg = Background::new(&Params::fiducial());
        let th = Thermo::new(&bg);
        let f = Fast::new(&bg, &th);
        let k = 2e-5; // enters the horizon only today
        let times = vec![50.0, 300.0, 2000.0];
        let mut y = vec![0.0; NVAR];
        let mut dy = vec![0.0; NVAR];
        let out = integrate_mode(&f, k, &times, 1e-6, false);
        assert!(out.delta_m0.is_finite());
        // re-integrate by hand to look at phi in the matter era (tau = 2000 Mpc, z ~ 40)
        let tau_ini: f64 = 0.3;
        let p0 = f.at(tau_ini);
        let r_nu = f.omega_n / (f.omega_g + f.omega_n + p0.od);
        let kt2 = (k * tau_ini).powi(2);
        y[I_ETA] = 1.0 - (5.0 + 4.0 * r_nu) / (12.0 * (15.0 + 4.0 * r_nu)) * kt2;
        let dg = -kt2 / 3.0;
        let tg = -k * kt2 * k * tau_ini / 36.0;
        y[I_DC] = 0.75 * dg;
        y[I_DB] = 0.75 * dg;
        y[I_TB] = tg;
        y[I_FG] = dg;
        y[I_FG + 1] = 4.0 / (3.0 * k) * tg;
        y[I_FN] = dg;
        y[I_FN + 1] = 4.0 / (3.0 * k) * tg * (23.0 + 4.0 * r_nu) / (15.0 + 4.0 * r_nu);
        y[I_FN + 2] = 2.0 * (2.0 * kt2 / (3.0 * (15.0 + 4.0 * r_nu)));
        let mut h = 0.03;
        integrate(|t, yy, dd| { rhs(&f, k, t, yy, dd, true, false); }, &mut y, tau_ini, 150.0, &mut h, 1e-7, I_FG + 1);
        let aux = rhs(&f, k, 150.0, &y, &mut dy, true, false);
        let sigma = aux.pi / 5.0;
        y[I_FG + 2] = 2.0 * sigma;
        y[I_GG] = 2.5 * sigma;
        y[I_GG + 2] = 0.5 * sigma;
        integrate(|t, yy, dd| { rhs(&f, k, t, yy, dd, false, false); }, &mut y, 150.0, 2000.0, &mut h, 1e-7, I_FG + 1);
        let aux = rhs(&f, k, 2000.0, &y, &mut dy, false, false);
        let p = f.at(2000.0);
        let hub = {
            let c0 = 3.0 / (HUBBLE_LENGTH * HUBBLE_LENGTH);
            let a2 = p.a * p.a;
            (c0 * ((f.omega_g + f.omega_n) / a2 + (f.omega_b + f.omega_c) / p.a + f.omega_l * a2) / 3.0).sqrt()
        };
        let phi = y[I_ETA] - hub * aux.alpha;
        // 3/5 in pure matter domination; radiation at z ~ 40 lowers the value slightly
        assert!((phi - 0.6).abs() < 0.01, "phi = {}", phi);
    }
}

#[cfg(test)]
mod source_tests {
    use super::*;
    use crate::cosmo::Params;

    /// (phi + psi)' = eta' + alpha'': compare the analytic expression used in the
    /// source S0 with a finite difference of phi + psi = eta + alpha'.
    #[test]
    fn isw_source_is_the_derivative_of_the_potentials() {
        let bg = Background::new(&Params::fiducial());
        let th = Thermo::new(&bg);
        let f = Fast::new(&bg, &th);
        for &k in &[0.005, 0.02, 0.08] {
            // integrate to a set of closely spaced times after recombination
            let times: Vec<f64> = vec![400.0, 400.5, 401.0, 800.0, 800.5, 801.0, 3000.0, 3001.0, 3002.0];
            let tau_ini = (0.02 / k as f64).min(0.3).max(2e-3);
            let _ = tau_ini;
            // use integrate_mode's machinery through a local copy of its loop
            let mut pots = Vec::new();
            let mut ders = Vec::new();
            {
                let tau_ini = (0.02 / k as f64).min(0.3).max(2e-3);
                let p0 = f.at(tau_ini);
                let r_nu = f.omega_n / (f.omega_g + f.omega_n + p0.od);
                let kt = k * tau_ini;
                let kt2 = kt * kt;
                let mut y = vec![0.0; NVAR];
                let dg = -kt2 / 3.0;
                let tg = -k * kt2 * kt / 36.0;
                y[I_ETA] = 1.0 - (5.0 + 4.0 * r_nu) / (12.0 * (15.0 + 4.0 * r_nu)) * kt2;
                y[I_DC] = 0.75 * dg;
                y[I_DB] = 0.75 * dg;
                y[I_TB] = tg;
                y[I_DD] = dg;
                // velocity of a perfect fluid with equation of state w in the adiabatic mode:
    // theta = theta_gamma 9w/(4 - 3w)  (theta_gamma for w = 1/3, zero for pressureless matter)
    y[I_TD] = tg * 9.0 * p0.wd / (4.0 - 3.0 * p0.wd);
                y[I_FG] = dg;
                y[I_FG + 1] = 4.0 / (3.0 * k) * tg;
                y[I_FN] = dg;
                y[I_FN + 1] = 4.0 / (3.0 * k) * tg * (23.0 + 4.0 * r_nu) / (15.0 + 4.0 * r_nu);
                y[I_FN + 2] = 2.0 * (2.0 * kt2 / (3.0 * (15.0 + 4.0 * r_nu)));
                let tau_sw = tca_end(&f, k, tau_ini, 0.008);
                let mut h = 0.1 * tau_ini;
                let mut dy = vec![0.0; NVAR];
                integrate(|t, yy, dd| { rhs(&f, k, t, yy, dd, true, false); }, &mut y, tau_ini, tau_sw, &mut h, 1e-7, I_FG + 1);
                let aux = rhs(&f, k, tau_sw, &y, &mut dy, true, false);
                let sigma = aux.pi / 5.0;
                y[I_FG + 2] = 2.0 * sigma;
                y[I_GG] = 2.5 * sigma;
                y[I_GG + 2] = 0.5 * sigma;
                h = h.min(0.1 / aux.dkappa);
                let mut t = tau_sw;
                for &tj in &times {
                    integrate(|tt, yy, dd| { rhs(&f, k, tt, yy, dd, false, false); }, &mut y, t, tj, &mut h, 1e-7, I_FG + 1);
                    t = tj;
                    let aux = rhs(&f, k, tj, &y, &mut dy, false, false);
                    pots.push(y[I_ETA] + aux.alpha_prime);
                    ders.push(aux.eta_prime + aux.alpha_pp);
                }
            }
            for base in [0usize, 3, 6] {
                let dt = times[base + 2] - times[base];
                let fd = (pots[base + 2] - pots[base]) / dt;
                let an = ders[base + 1];
                eprintln!("k = {}, tau = {}: phi+psi = {:.5}, finite difference {:.4e}, analytic {:.4e}", k, times[base + 1], pots[base + 1], fd, an);
                assert!((fd - an).abs() < 2e-3 * (fd.abs() + an.abs()) + 1e-9, "k = {}: {} vs {}", k, fd, an);
            }
        }
    }
}

#[cfg(test)]
mod pk_tests {
    use super::*;
    use crate::cosmo::Params;

    /// The fast "matter only" integration used for the matter power spectrum at large
    /// wavenumbers must agree with the full integration.
    #[test]
    fn matter_only_mode_agrees_with_full_integration() {
        let bg = Background::new(&Params::fiducial());
        let th = Thermo::new(&bg);
        let f = Fast::new(&bg, &th);
        let tau_out = source_times(&bg);
        for &k in &[0.2, 0.35, 0.42, 2.0, 5.0] {
            let full = integrate_mode(&f, k, &tau_out, 1e-5, false).delta_m0;
            let fast = integrate_mode(&f, k, &[bg.tau0], 1e-5, true).delta_m0;
            println!("k = {}: full {:.6e}  matter-only {:.6e}  ratio {:.5}", k, full, fast, fast / full);
            assert!((fast / full - 1.0).abs() < 1e-3, "k = {}: {} vs {}", k, fast, full);
        }
    }

    /// A superfluid with a negligible sound speed must reproduce cold dark matter
    /// (it is evolved by the general fluid equations in the dark-fluid slot), and a
    /// sound speed must suppress the growth at large wavenumbers only.
    #[test]
    fn superfluid_with_zero_sound_speed_is_cold_dark_matter() {
        use crate::cosmo::SuperfluidDm;
        let fid = Params::fiducial();
        let run = |p: &Params, k: f64| -> f64 {
            let bg = Background::new(p);
            let th = Thermo::new(&bg);
            let f = Fast::new(&bg, &th);
            integrate_mode(&f, k, &[bg.tau0], 1e-5, true).delta_m0
        };
        let cold = Params { superfluid_dm: Some(SuperfluidDm::Bose { cs0_sq: 1e-30 }), ..fid };
        let warm = Params { superfluid_dm: Some(SuperfluidDm::Bose { cs0_sq: 1e-14 }), ..fid };
        for &k in &[0.01, 0.3, 2.5] {
            let (c, s, w) = (run(&fid, k), run(&cold, k), run(&warm, k));
            println!("k = {}: CDM {:.6e}  superfluid(c_s=0) {:.6e}  ratio {:.6};  c_s0^2 = 1e-14: ratio {:.4}", k, c, s, s / c, w / c);
            assert!((s / c - 1.0).abs() < 2e-4, "k = {}: {} vs {}", k, s, c);
            assert!(w / c < 1.0 + 1e-3);
        }
        assert!((run(&warm, 0.01) / run(&fid, 0.01) - 1.0).abs() < 1e-3);
        assert!(run(&warm, 2.5) / run(&fid, 2.5) < 0.9);
    }
}
