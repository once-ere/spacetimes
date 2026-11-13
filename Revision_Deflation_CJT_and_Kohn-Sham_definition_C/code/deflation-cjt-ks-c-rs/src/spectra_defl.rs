//! The spectra of the microwave background on a deflating spacetime: what is assumed,
//! and the routines that the commands `background`, `spectrum`, `forecast` and `superfluid` share.
//!
//! WHAT IS COMPUTED.  The main project computes the temperature and polarization spectra
//! of the microwave background and the power spectrum of matter with its own
//! Einstein-Boltzmann program.  The revision repeats that calculation while x5, x6, x7
//! deflate.  The modules `eb_cosmo`, `eb_boltzmann`, `eb_spectra` and `eb_fit` are copies
//! of four modules of that program with a short list of changes (they are written by
//! code/scripts/vendor_eb.py and are not edited by hand).
//!
//! ASSUMPTIONS.  (A1) and (A2) are those of `cosmo_defl`: ordinary space expands, and
//! gravity is the eight-dimensional Einstein theory, so that Newton's constant is
//! G_N(x4) = G_N(today) exp(s), s = gsum (x4 - x4_today), and the expansion obeys
//!
//!     3 H^2 - 3 gsum H + gprod = 8 pi G_N(x4) rho .
//!
//! The spectra need one assumption more, because the hypotheses of the revision prescribe
//! the metric and say nothing about the agent that keeps the three rates constant.
//!
//! (A3) THE PERTURBATIONS.  The scale factors of the extra coordinates are not perturbed:
//!      the agent that prescribes them holds them.  The components of the eight-dimensional
//!      Einstein tensor along ordinary spacetime are then, to first order in a perturbation
//!      of ordinary space in the synchronous gauge (conformal time, a prime; Hc = a'/a;
//!      notation of Ma and Bertschinger 1995; derived by computer algebra in
//!      code/scripts/verify_deflating_independent.py), the left-hand sides of
//!
//!        (E)  k^2 eta - (1/2) (Hc - a gsum/2) h'                          = - 4 pi G_N(x4) a^2 delta rho ,
//!        (M)  k^2 eta'                                                    =   4 pi G_N(x4) a^2 (rho + P) theta ,
//!        (S)  (h'' + 6 eta'') + (2 Hc - a gsum)(h' + 6 eta') - 2 k^2 eta  = - 24 pi G_N(x4) a^2 (rho + P) sigma ,
//!        (T)  h'' + (2 Hc - a gsum) h' - 2 k^2 eta                        = - 24 pi G_N(x4) a^2 delta P :
//!
//!      the equations of Ma and Bertschinger with Newton's constant of the moment and with
//!      Hc - a gsum/2 in the place of Hc in three of them.  On the right stand the
//!      perturbations of matter and radiation.  Matter that is spread uniformly over the
//!      extra coordinates obeys the four-dimensional equations of motion unchanged.
//!
//!      Because Newton's constant depends on the time, (E), (M), (S), (T) cannot all hold
//!      with matter and radiation alone on the right: the identities that follow from them
//!      and from the conservation of matter fail by terms proportional to gsum.  The agent
//!      must carry perturbations, and the hypotheses do not say which.  Assumption (A2) says
//!      that the agent has no energy density; taken to first order it says that (E) holds.
//!      One more statement is needed, and it is a choice: the closure.  Each closure is a
//!      consistent linear theory.
//!
//!      "Momentum":  (M) holds: the agent carries no momentum density.  Observers who move
//!                   with the cold matter then see neither an energy density nor a flow of
//!                   energy of the agent, only stresses, which (S) and (T) define.
//!      "Pressure":  (T) holds: the agent carries no pressure perturbation.  Its density
//!                   (zero) and its pressure are then uniform on the surfaces of constant
//!                   synchronous time, like the scale factors that it holds; (M) and (S)
//!                   define its momentum density and its anisotropic stress.
//!      "Shear":     (S) holds: the agent carries no anisotropic stress.  This closure leads
//!                   to stiff equations (the momentum density of the agent relaxes at the
//!                   rate k^2/(3 Hc)); it is not computed by this program.  Far inside the
//!                   horizon it gives the growth equation of the closure "Pressure" (shown
//!                   in code/scripts/verify_deflating_independent.py).
//!
//!      The program computes with "Momentum" and with "Pressure".
//!
//! WHAT DEPENDS ON THE CLOSURE.  The closures differ at the relative order a gsum/Hc.
//! At recombination that ratio is below 1e-5 gsum/H_0: the acoustic peaks, and with them
//! the spectra at the multipoles l >= 30 that the forecast uses, do not depend on the
//! closure.  Today the ratio is gsum/H_0: the growth of structure and the potentials of
//! today (the late integrated Sachs-Wolfe effect, l < 30) depend on the closure at the
//! same order as on the deflation itself.  For pressureless matter the density contrast
//! obeys, exactly and for every wavenumber,
//!
//!     "Pressure":  delta.. + (2 H - gsum/2) delta. = 4 pi G_N(x4) rho_m delta        (cosmic time, a dot) ,
//!     "Momentum":  delta'' + [(2 He' + g)/(2 He)] delta' = [(Hc - a gsum)/(2 He)] g delta ,
//!                  He = Hc - a gsum/2,  g = 8 pi G_N(x4) a^2 rho_m                   (conformal time, a prime) .
//!
//! The program reports every such quantity for both closures.
//!
//! THE SOURCES OF THE TEMPERATURE contain alpha = (h' + 6 eta')/(2 k^2) and its first two
//! derivatives.  In general relativity alpha' is the right-hand side of (S).  With deflation
//! (S) does not hold with matter alone in either closure: alpha' is that right-hand side
//! plus a "gap", which the conservation of matter determines once the closure is chosen
//! (`eb_boltzmann`, function `rhs`).
//!
//! Units: rates in units of H_100 = 100 km/s/Mpc, as in `cosmo_defl`.

use crate::eb_boltzmann::{integrate_mode, Fast};
use crate::eb_cosmo::{Background, Deflation, Thermo};
use crate::eb_spectra::compute;
use spacetimes::cosmo::Params;
use spacetimes::forecast::{from_vec, to_vec, Jacobian, NPAR};
use spacetimes::spectra::{Spectra, Workspace};

/// 100 theta_* of the model: the angle under which the sound horizon at last scattering is
/// seen, from the full recombination calculation on the deflating background.
pub fn theta_star_100(p: &Params, d: &Deflation) -> f64 {
    let bg = Background::new(p, d);
    Thermo::new(&bg).theta_star_100
}

/// The Hubble parameter h for which the model with the deflation `d` and the other
/// parameters of `base` has 100 theta_* = `target`.  Bisection: theta_* grows with h.
/// (The angle is a property of the background and does not depend on the closure.)
pub fn h_at_fixed_theta(base: &Params, d: &Deflation, target: f64) -> f64 {
    let f = |h: f64| theta_star_100(&Params { h, ..*base }, d) - target;
    let (mut lo, mut hi) = (0.35, 0.95);
    let (flo, fhi) = (f(lo), f(hi));
    assert!(flo < 0.0 && fhi > 0.0, "h_at_fixed_theta: no bracket ({} at h = 0.35, {} at h = 0.95)", flo, fhi);
    for _ in 0..44 {
        let mid = 0.5 * (lo + hi);
        if f(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// The sum of the two gravitational potentials of a mode and its derivative, at one time.
pub struct Potentials {
    /// eta + alpha': the sum of the two potentials of the conformal Newtonian gauge
    pub potential: f64,
    /// its derivative by a central difference along the solution
    pub difference: f64,
    /// the expression eta' + alpha'' that the source of the temperature uses for that derivative
    pub expression: f64,
    /// THE GAPS, what the agent supplies (all zero without deflation):
    /// alpha' minus the right-hand side of the equation of the shear (S) with matter and
    /// radiation alone (k^2 times it is the anisotropic stress of the agent);
    pub shear_gap: f64,
    /// the left-hand side minus the right-hand side of the trace equation (T) with matter and
    /// radiation alone, h'' + 2 He h' - 2 k^2 eta + 24 pi G a^2 delta P (zero for the closure
    /// "Pressure"), and the size of the terms of that equation;
    pub trace_gap: f64,
    pub trace_scale: f64,
    /// 2 k^2 eta' - 8 pi G a^2 (rho + P) theta, what the momentum constraint (M) lacks (zero for
    /// the closure "Momentum"), and the size of the terms of that equation.
    pub momentum_gap: f64,
    pub momentum_scale: f64,
    /// NUMERICAL CHECKS of the two identities from which the program computes the gaps (zero
    /// up to the error of a numerical derivative): alpha' by a difference along the solution
    /// minus the alpha' of the program; and (T) with h'' by a difference minus the trace gap.
    pub shear_check: f64,
    pub trace_check: f64,
    /// a gsum and the conformal Hubble rate, in 1/Mpc
    pub a_gsum: f64,
    pub hub: f64,
}

/// The potentials of the mode k (1/Mpc) at the conformal times `taus` (Mpc).  The
/// derivative is taken over +-dt with dt = max(0.5 Mpc, 0.004 tau): small against the time
/// over which the potentials change, and large enough for the error of the integration
/// not to matter.  Each closure is a consistent theory, so that the quantity that the source
/// uses equals the derivative to the accuracy of the integration, with and without deflation.
pub fn potentials(p: &Params, d: &Deflation, k: f64, taus: &[f64], rtol: f64) -> Vec<Potentials> {
    let bg = Background::new(p, d);
    let th = Thermo::new(&bg);
    let fast = Fast::new(&bg, &th);
    let step = |t: f64| (0.004 * t).max(0.5);
    let mut times = Vec::new();
    for &t in taus {
        times.push(t - step(t));
        times.push(t);
        times.push(t + step(t));
    }
    let out = integrate_mode(&fast, k, &times, rtol, false);
    (0..taus.len())
        .map(|i| {
            let pt = fast.at(taus[i]);
            let hub = fast.hubble_conf(&pt);
            let pot = out.pot[3 * i + 1];
            Potentials {
                potential: pot,
                difference: (out.pot[3 * i + 2] - out.pot[3 * i]) / (2.0 * step(taus[i])),
                expression: out.dpot[3 * i + 1],
                shear_gap: out.shear_gap[3 * i + 1],
                trace_gap: out.trace_gap[3 * i + 1],
                // the terms of (T) are of the order k^2 times the potential and Hc^2 times the potential
                trace_scale: (k * k + hub * hub) * pot.abs(),
                momentum_gap: out.momentum_gap[3 * i + 1],
                // the terms of (M) are of the order k^2 Hc times the potential
                momentum_scale: k * k * hub * pot.abs(),
                shear_check: out.shear_check[3 * i + 1],
                trace_check: out.trace_check[3 * i + 1],
                a_gsum: pt.a * fast.gam_hat,
                hub,
            }
        })
        .collect()
}

/// The growth of the density contrast of matter (cold dark matter and baryons) of the mode
/// k (1/Mpc) from the scale factor `a_start` to today, from the Einstein-Boltzmann equations.
pub fn growth_factor(p: &Params, d: &Deflation, k: f64, a_start: f64) -> f64 {
    let bg = Background::new(p, d);
    let th = Thermo::new(&bg);
    let fast = Fast::new(&bg, &th);
    let early = integrate_mode(&fast, k, &[bg.tau_of_a(a_start)], 1e-6, false).delta_m0;
    let late = integrate_mode(&fast, k, &[bg.tau0], 1e-6, false).delta_m0;
    late / early
}

/// Central-difference derivatives of the spectra with respect to the six parameters at
/// `base`, on the deflating background `d` (the routine `jacobian` of the main project with
/// the same steps; without deflation the two give the same numbers).
pub fn jacobian_defl(base: &Params, d: &Deflation, ws: &Workspace) -> Jacobian {
    let v0 = to_vec(base);
    let steps = [0.01 * v0[0], 0.01 * v0[1], 0.004 * v0[2], 0.02, 0.005, 0.004];
    let mut out = Vec::new();
    for i in 0..NPAR {
        let (mut vp, mut vm) = (v0, v0);
        vp[i] += steps[i];
        vm[i] -= steps[i];
        let sp = compute(&from_vec(base, &vp), d, ws, 1.0);
        let sm = compute(&from_vec(base, &vm), d, ws, 1.0);
        out.push(difference(&sp, &sm, 2.0 * steps[i]));
    }
    Jacobian { d: out }
}

fn difference(a: &Spectra, b: &Spectra, den: f64) -> [Vec<f64>; 3] {
    let f = |x: &Vec<f64>, y: &Vec<f64>| -> Vec<f64> { x.iter().zip(y).map(|(p, q)| (p - q) / den).collect() };
    [f(&a.tt, &b.tt), f(&a.te, &b.te), f(&a.ee, &b.ee)]
}

/// The derivative of the spectra with respect to the sum of the rates, gam = gsum/H_100, at
/// gam = `gam0` >= 0 for equal rates, the other parameters fixed.  The rates are positive by
/// hypothesis, so that at gam0 = 0 the derivative is one-sided: the formula of second order
/// [-3 f(0) + 4 f(e) - f(2e)]/(2e).  At gam0 >= e a central difference is used.
pub fn rate_derivative(p: &Params, d0: &Deflation, gam0: f64, e: f64, ws: &Workspace) -> [Vec<f64>; 3] {
    let at = |gam: f64| compute(p, &Deflation { closure: d0.closure, bbn: d0.bbn, ..Deflation::equal_rates(gam) }, ws, 1.0);
    if gam0 >= e {
        difference(&at(gam0 + e), &at(gam0 - e), 2.0 * e)
    } else {
        let (s0, s1, s2) = (at(gam0), at(gam0 + e), at(gam0 + 2.0 * e));
        let f = |a: &Vec<f64>, b: &Vec<f64>, c: &Vec<f64>| -> Vec<f64> { (0..a.len()).map(|l| (-3.0 * a[l] + 4.0 * b[l] - c[l]) / (2.0 * e)).collect() };
        [f(&s0.tt, &s1.tt, &s2.tt), f(&s0.te, &s1.te, &s2.te), f(&s0.ee, &s1.ee, &s2.ee)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cosmo_defl::{theta_star, Model};
    use crate::eb_boltzmann::{eval, source_times, NVAR};
    use crate::eb_cosmo::Closure;
    use crate::eb_spectra::matter_power;
    use spacetimes::cosmo::{DarkFluid, Step, SuperfluidDm, HUBBLE_LENGTH};

    /// Without deflation the copies must be the program of the main project: the same
    /// numbers to the last bit, for the background, the thermal history, the sources of
    /// single modes, the spectra and the matter power.
    #[test]
    fn without_deflation_the_copies_reproduce_the_program_of_the_main_project_bit_for_bit() {
        let p = Params::fiducial();
        let d = Deflation::none();
        // background
        let (bg, bg0) = (Background::new(&p, &d), spacetimes::cosmo::Background::new(&p));
        assert_eq!(bg.tau0.to_bits(), bg0.tau0.to_bits());
        assert_eq!(bg.omega_lambda.to_bits(), bg0.omega_lambda.to_bits());
        for &a in &[1e-10, 1e-7, 3e-4, 9.2e-4, 0.1, 0.5, 1.0] {
            assert_eq!(bg.hubble(a).to_bits(), bg0.hubble(a).to_bits(), "hubble at a = {}", a);
            assert_eq!(bg.tau_of_a(a).to_bits(), bg0.tau_of_a(a).to_bits(), "tau at a = {}", a);
        }
        assert_eq!(bg.sound_horizon(1.0 / 1090.0).to_bits(), bg0.sound_horizon(1.0 / 1090.0).to_bits());
        assert_eq!(bg.s_early(), 0.0);
        // thermal history
        let (th, th0) = (Thermo::new(&bg), spacetimes::cosmo::Thermo::new(&bg0));
        assert_eq!(th.y_he.to_bits(), th0.y_he.to_bits());
        assert_eq!(th.z_star.to_bits(), th0.z_star.to_bits());
        assert_eq!(th.theta_star_100.to_bits(), th0.theta_star_100.to_bits());
        assert_eq!(th.theta_s_100.to_bits(), th0.theta_s_100.to_bits());
        assert_eq!(th.xe.len(), th0.xe.len());
        for i in 0..th.xe.len() {
            assert_eq!(th.xe[i].to_bits(), th0.xe[i].to_bits());
            assert_eq!(th.dkappa[i].to_bits(), th0.dkappa[i].to_bits());
        }
        // sources of single modes
        let (fast, fast0) = (Fast::new(&bg, &th), spacetimes::boltzmann::Fast::new(&bg0, &th0));
        let times = source_times(&bg);
        assert_eq!(times, spacetimes::boltzmann::source_times(&bg0));
        for &k in &[0.004, 0.05, 0.2] {
            let m = integrate_mode(&fast, k, &times, 1e-5, false);
            let m0 = spacetimes::boltzmann::integrate_mode(&fast0, k, &times, 1e-5, false);
            for j in 0..times.len() {
                assert_eq!(m.s0[j].to_bits(), m0.s0[j].to_bits(), "S0, k = {}, time {}", k, j);
                assert_eq!(m.s1[j].to_bits(), m0.s1[j].to_bits());
                assert_eq!(m.s2[j].to_bits(), m0.s2[j].to_bits());
                assert_eq!(m.se[j].to_bits(), m0.se[j].to_bits());
            }
            assert_eq!(m.delta_m0.to_bits(), m0.delta_m0.to_bits());
        }
        // spectra (a low highest multipole keeps the test short; the command `spectrum`
        // repeats the comparison at the full resolution) and matter power
        let ws = Workspace::new(120);
        let (s, s0) = (compute(&p, &d, &ws, 1.0), spacetimes::spectra::compute(&p, &ws, 1.0));
        for l in 2..=120 {
            assert_eq!(s.tt[l].to_bits(), s0.tt[l].to_bits(), "TT at l = {}", l);
            assert_eq!(s.te[l].to_bits(), s0.te[l].to_bits());
            assert_eq!(s.ee[l].to_bits(), s0.ee[l].to_bits());
        }
        assert_eq!(s.sigma8.to_bits(), s0.sigma8.to_bits());
        let ks = [0.02, 0.3, 4.0];
        let (pk, _) = matter_power(&p, &d, &ks, 1e-5);
        let (pk0, _) = spacetimes::spectra::matter_power(&p, &ks, 1e-5);
        for i in 0..ks.len() {
            assert_eq!(pk[i].to_bits(), pk0[i].to_bits());
        }
        // superfluid dark matter and a stepped dark fluid, as the main project uses them
        let variants = [
            Params { superfluid_dm: Some(SuperfluidDm::Bose { cs0_sq: 1e-18 }), ..p },
            Params { dark: DarkFluid { n_ir: 0.5, step: Some(Step::project(2.0e4)) }, ..p },
        ];
        for q in &variants {
            let (pk, _) = matter_power(q, &d, &ks, 1e-5);
            let (pk0, _) = spacetimes::spectra::matter_power(q, &ks, 1e-5);
            for i in 0..ks.len() {
                assert_eq!(pk[i].to_bits(), pk0[i].to_bits());
            }
        }
        let (s, s0) = (compute(&variants[1], &d, &ws, 1.0), spacetimes::spectra::compute(&variants[1], &ws, 1.0));
        for l in 2..=120 {
            assert_eq!(s.tt[l].to_bits(), s0.tt[l].to_bits(), "stepped fluid, TT at l = {}", l);
            assert_eq!(s.ee[l].to_bits(), s0.ee[l].to_bits());
        }
    }

    /// Without deflation the closure "Pressure" is the same program: the closures differ
    /// only by terms proportional to the rate, and the copy takes the path of the main
    /// project when the rate is zero.
    #[test]
    fn without_deflation_the_closure_pressure_is_the_same_program() {
        let p = Params::fiducial();
        let (dm, dp) = (Deflation::none(), Deflation { closure: Closure::Pressure, ..Deflation::none() });
        let ws = Workspace::new(120);
        let (s, sp) = (compute(&p, &dm, &ws, 1.0), compute(&p, &dp, &ws, 1.0));
        for l in 2..=120 {
            assert_eq!(sp.tt[l].to_bits(), s.tt[l].to_bits(), "TT at l = {}", l);
            assert_eq!(sp.te[l].to_bits(), s.te[l].to_bits());
            assert_eq!(sp.ee[l].to_bits(), s.ee[l].to_bits());
        }
        assert_eq!(sp.sigma8.to_bits(), s.sigma8.to_bits());
        let ks = [0.02, 0.3, 4.0];
        let (pk, _) = matter_power(&p, &dm, &ks, 1e-5);
        let (pkp, _) = matter_power(&p, &dp, &ks, 1e-5);
        for i in 0..ks.len() {
            assert_eq!(pkp[i].to_bits(), pk[i].to_bits());
        }
    }

    /// The clock s(a), the Hubble rate, the age and the angle theta_* of the copy of the
    /// background agree with the module `cosmo_defl`, which was written independently of
    /// the Einstein-Boltzmann program (and is itself checked by the Python script).
    #[test]
    fn the_background_of_the_copy_agrees_with_the_background_module() {
        let p = Params::fiducial();
        for &gam in &[0.02, 0.2] {
            let d = Deflation::equal_rates(gam);
            let bg = Background::new(&p, &d);
            let th = Thermo::new(&bg);
            let m = Model::fiducial(gam, p.h);
            let der = theta_star(&m, th.z_star, &|_| 1.0);
            // Newton's constant in the early universe
            assert!((bg.s_early().exp() / der.g_early - 1.0).abs() < 1e-7, "G early: {} vs {}", bg.s_early().exp(), der.g_early);
            // age (units of 1/H_100), sound horizon, distance and angle
            assert!((bg.age() / der.age - 1.0).abs() < 2e-6, "age: {} vs {}", bg.age(), der.age);
            let rs = th.rs_star / HUBBLE_LENGTH;
            assert!((rs / der.rs - 1.0).abs() < 2e-6, "r_s: {} vs {}", rs, der.rs);
            assert!((th.theta_star_100 / der.theta100 - 1.0).abs() < 2e-6, "theta_*: {} vs {}", th.theta_star_100, der.theta100);
            // the equation of the expansion at several times
            for &a in &[1e-6, 1e-3, 0.3, 1.0] {
                let s = bg.s_of_a(a);
                let e = bg.hubble(a) * HUBBLE_LENGTH;
                let dens = (bg.omega_gamma + bg.omega_nu) / a.powi(4) + (p.omega_b + p.omega_c) / a.powi(3) + bg.omega_lambda;
                let res = e * e - gam * e + gam * gam / 9.0 - s.exp() * dens;
                assert!(res.abs() < 1e-11 * e * e, "expansion equation at a = {}: residual {}", a, res);
            }
            // ds/dln a = gam/E (central difference of the interpolated table)
            for &a in &[1e-4, 0.01, 0.5] {
                let (ap, am) = (a * 1.001, a / 1.001);
                let ds = (bg.s_of_a(ap) - bg.s_of_a(am)) / (ap / am).ln();
                let want = gam / (bg.hubble(a) * HUBBLE_LENGTH);
                assert!((ds / want - 1.0).abs() < 1e-5, "ds/dln a at a = {}: {} vs {}", a, ds, want);
            }
            assert!(bg.s_early() < 0.0 && bg.s_of_a(1.0) == 0.0);
        }
    }

    /// The derivative of the conformal Hubble rate that the perturbation equations use is
    /// the derivative of the conformal Hubble rate (with deflation it contains the time
    /// dependence of Newton's constant), the rate of the energy constraint is Hc - a gsum/2,
    /// and the background quantity of the closure "Pressure" has the closed form that the
    /// program uses.
    #[test]
    fn the_hubble_rate_of_the_perturbation_equations_and_its_derivative() {
        let p = Params::fiducial();
        for &gam in &[0.0, 0.3] {
            let d = Deflation::equal_rates(gam);
            let bg = Background::new(&p, &d);
            let th = Thermo::new(&bg);
            let fast = Fast::new(&bg, &th);
            let y = vec![0.0; NVAR];
            for &tau in &[5.0, 280.0, 2000.0, 9000.0, 13000.0] {
                let (_, aux) = eval(&fast, 0.01, tau, &y, false);
                let pt = fast.at(tau);
                // Hc from the background at the same scale factor
                let hc = |a: f64| a * bg.hubble(a);
                assert!((aux.hub / hc(pt.a) - 1.0).abs() < 1e-7, "Hc at tau = {}: {} vs {}", tau, aux.hub, hc(pt.a));
                // Hc' = a Hc dHc/da
                let (ap, am) = (pt.a * 1.0005, pt.a / 1.0005);
                let dhc = pt.a * hc(pt.a) * (hc(ap) - hc(am)) / (ap - am);
                assert!((aux.hub_prime - dhc).abs() < 3e-6 * (dhc.abs() + aux.hub * aux.hub), "Hc' at tau = {} (gam = {}): {} vs {}", tau, gam, aux.hub_prime, dhc);
                let want = aux.hub - 0.5 * pt.a * gam / HUBBLE_LENGTH;
                assert!((aux.hub_e / want - 1.0).abs() < 1e-14);
                // the background quantity B of the closure "Pressure": the closed form against its definition
                let b_def = aux.hub_e_prime - aux.hub_e * aux.hub_e + aux.half_rho_plus_p;
                assert!((aux.b_trace - b_def).abs() < 1e-11 * aux.hub * aux.hub, "B at tau = {} (gam = {}): {} vs {}", tau, gam, aux.b_trace, b_def);
                if gam > 0.0 {
                    assert!(aux.b_trace > 0.0);
                } else {
                    assert_eq!(aux.b_trace, 0.0);
                }
            }
        }
        // the same for a dark fluid whose equation of state changes with the time
        let q = Params { dark: DarkFluid { n_ir: 0.5, step: Some(Step::project(2.0e4)) }, ..p };
        let bg = Background::new(&q, &Deflation::equal_rates(0.3));
        let th = Thermo::new(&bg);
        let fast = Fast::new(&bg, &th);
        for &tau in &[2.0, 5.0, 12.0, 280.0, 9000.0] {
            let (_, aux) = eval(&fast, 0.01, tau, &vec![0.0; NVAR], false);
            let b_def = aux.hub_e_prime - aux.hub_e * aux.hub_e + aux.half_rho_plus_p;
            assert!((aux.b_trace - b_def).abs() < 1e-11 * aux.hub * aux.hub, "stepped fluid, B at tau = {}: {} vs {}", tau, aux.b_trace, b_def);
        }
    }

    /// A smaller Newton's constant at nucleosynthesis lowers the helium fraction, by the
    /// amount that the equivalent number of missing neutrino species would.
    #[test]
    fn newtons_constant_at_nucleosynthesis_lowers_the_helium_fraction() {
        let p = Params::fiducial();
        let y0 = Background::new(&p, &Deflation::none()).helium_fraction();
        assert_eq!(y0.to_bits(), p.helium_fraction().to_bits());
        let d = Deflation::equal_rates(0.05);
        let bg = Background::new(&p, &d);
        let y = bg.helium_fraction();
        let dn = 43.0 / 7.0 * (bg.s_early().exp() - 1.0);
        assert!(dn < 0.0 && y < y0);
        // the same shift from the main program's own fit with fewer neutrino species
        let y_main = Params { n_ur: p.n_ur + dn, ..p }.helium_fraction();
        assert!((y - y_main).abs() < 1e-12, "{} vs {}", y, y_main);
        // switched off on request
        let off = Background::new(&p, &Deflation { bbn: false, ..d });
        assert_eq!(off.helium_fraction().to_bits(), y0.to_bits());
        // with extra radiation dn0 the square of the expansion rate is proportional to Newton's constant times
        // (43/7 + dn0): the equivalent number of species is dn0 + (43/7 + dn0)(exp(s_early) - 1)
        let dn0 = 0.5;
        let q = Params { n_ur: p.n_ur + dn0, ..p };
        let bq = Background::new(&q, &d);
        let g = bq.s_early().exp();
        let y_q = bq.helium_fraction();
        let y_equiv = Params { n_ur: p.n_ur + dn0 + (43.0 / 7.0 + dn0) * (g - 1.0), ..p }.helium_fraction();
        assert!((y_q - y_equiv).abs() < 1e-12, "{} vs {}", y_q, y_equiv);
        // check of the correspondence itself: G (43/7 + dn_equivalent) = G_today exp(s) (43/7 + dn0)
        let dn_equiv = dn0 + (43.0 / 7.0 + dn0) * (g - 1.0);
        assert!(((43.0 / 7.0 + dn_equiv) / (g * (43.0 / 7.0 + dn0)) - 1.0).abs() < 1e-14);
        // and without deflation the model with extra radiation is that of the main program, bit for bit
        assert_eq!(Background::new(&q, &Deflation::none()).helium_fraction().to_bits(), q.helium_fraction().to_bits());
    }

    /// The gaps are proportional to the rate of deflation.  With a rate too small to change
    /// any number (1e-30 H_100) the program takes the paths of the deflating model, and must
    /// give the numbers of the program without deflation: potentials, sources and spectra,
    /// to the last bit.  The two numerical checks then test that the equation of the shear
    /// and the trace equation of general relativity hold along the solution.  Both closures;
    /// the fiducial model, superfluid dark matter (a fluid whose equation of state changes
    /// with the time) and a stepped dark fluid.
    #[test]
    fn with_a_negligible_rate_both_closures_reproduce_general_relativity() {
        let fid = Params::fiducial();
        let taus = [240.0, 280.0, 320.0, 800.0, 3000.0, 9000.0, 13000.0];
        let variants = [
            ("fiducial", fid),
            ("superfluid dark matter", Params { superfluid_dm: Some(SuperfluidDm::Bose { cs0_sq: 1e-18 }), ..fid }),
            ("stepped dark fluid", Params { dark: DarkFluid { n_ir: 0.5, step: Some(Step::project(2.0e4)) }, ..fid }),
        ];
        for closure in [Closure::Momentum, Closure::Pressure] {
            let tiny = Deflation { closure, ..Deflation::equal_rates(1e-30) };
            for (name, p) in &variants {
                let mut worst = [0.0f64; 2];
                for &k in &[0.005, 0.02, 0.08, 0.2] {
                    let r = potentials(p, &tiny, k, &taus, 1e-8);
                    let r0 = potentials(p, &Deflation::none(), k, &taus, 1e-8);
                    for i in 0..taus.len() {
                        let scale = r[i].potential.abs() + 1e-3;
                        // the gaps are of the order of the rate: nothing
                        let g = [r[i].shear_gap.abs() / scale, r[i].trace_gap.abs() / (r[i].trace_scale + 1e-3 * k * k), r[i].momentum_gap.abs() / (r[i].momentum_scale + 1e-3 * k * k * r[i].hub)];
                        assert!(g.iter().all(|x| *x < 1e-25), "{:?}, {}, k = {}, tau = {}: gaps {:?}", closure, name, k, taus[i], g);
                        // the numerical checks: the equation of the shear and the trace equation of general relativity
                        let c = [r[i].shear_check.abs() / scale, r[i].trace_check.abs() / (r[i].trace_scale + 1e-3 * k * k)];
                        worst[0] = worst[0].max(c[0]);
                        worst[1] = worst[1].max(c[1]);
                        assert!(c[0] < 2e-3, "{:?}, {}, k = {}, tau = {}: alpha' by a difference deviates by {} (potential {})", closure, name, k, taus[i], r[i].shear_check, r[i].potential);
                        assert!(c[1] < 5e-3, "{:?}, {}, k = {}, tau = {}: the trace equation fails by {} (scale {})", closure, name, k, taus[i], r[i].trace_check, r[i].trace_scale);
                        // potentials and the quantity of the source against the program without deflation: the same bits
                        assert!(r[i].potential.to_bits() == r0[i].potential.to_bits() && r[i].expression.to_bits() == r0[i].expression.to_bits(), "{:?}, {}, k = {}, tau = {}: potential {} against {}, eta' + alpha'' = {} against {}", closure, name, k, taus[i], r[i].potential, r0[i].potential, r[i].expression, r0[i].expression);
                    }
                }
                println!("{:?}, {}: numerical checks of general relativity (relative): equation of the shear {:.1e}, trace equation {:.1e}", closure, name, worst[0], worst[1]);
            }
            // the sources of single modes, at the times of the line-of-sight integration
            let (bg0, bgt) = (Background::new(&fid, &Deflation::none()), Background::new(&fid, &tiny));
            let (th0, tht) = (Thermo::new(&bg0), Thermo::new(&bgt));
            let (f0, ft) = (Fast::new(&bg0, &th0), Fast::new(&bgt, &tht));
            let times = source_times(&bg0);
            for &k in &[0.0005, 0.005, 0.05, 0.2] {
                let (m0, mt) = (integrate_mode(&f0, k, &times, 1e-5, false), integrate_mode(&ft, k, &times, 1e-5, false));
                for j in 0..times.len() {
                    assert_eq!(mt.s0[j].to_bits(), m0.s0[j].to_bits(), "{:?}, S0, k = {}, time {}", closure, k, j);
                    assert_eq!(mt.s1[j].to_bits(), m0.s1[j].to_bits());
                    assert_eq!(mt.s2[j].to_bits(), m0.s2[j].to_bits());
                    assert_eq!(mt.se[j].to_bits(), m0.se[j].to_bits());
                }
            }
            let ws = Workspace::new(120);
            let (s, s0) = (compute(&fid, &tiny, &ws, 1.0), compute(&fid, &Deflation::none(), &ws, 1.0));
            for l in 2..=120 {
                assert_eq!(s.tt[l].to_bits(), s0.tt[l].to_bits(), "{:?}, TT at l = {}", closure, l);
                assert_eq!(s.te[l].to_bits(), s0.te[l].to_bits());
                assert_eq!(s.ee[l].to_bits(), s0.ee[l].to_bits());
            }
        }
    }

    /// With deflation each closure is a consistent theory.  (1) The quantity that the source
    /// of the temperature uses for the derivative of the two potentials is that derivative.
    /// (2) The two identities from which the program computes the gaps hold along the
    /// solution: alpha' by a difference agrees with the alpha' of the program, and the trace
    /// equation with h'' by a difference fails by exactly the trace gap (zero for the closure
    /// "Pressure").  (3) The gaps are of the relative order a gsum/Hc, and so is the
    /// difference between the potentials of the two closures at late times; the shear gap
    /// of the closure "Pressure" is smaller by the factor (Hc/k)^2 inside the horizon.
    #[test]
    fn with_deflation_each_closure_is_consistent_and_the_two_differ_at_first_order() {
        let p = Params::fiducial();
        let taus = [400.0, 800.0, 3000.0, 9000.0, 13000.0];
        let mut worst = [0.0f64; 2];
        for &k in &[0.005, 0.02, 0.08] {
            let dm = Deflation::equal_rates(0.2);
            let dp = Deflation { closure: Closure::Pressure, ..dm };
            let (rm, rp) = (potentials(&p, &dm, k, &taus, 1e-8), potentials(&p, &dp, k, &taus, 1e-8));
            for i in 0..taus.len() {
                for (name, x) in [("Momentum", &rm[i]), ("Pressure", &rp[i])] {
                    // (1)
                    let tol = 2e-3 * (x.difference.abs() + x.expression.abs()) + 1e-9;
                    assert!((x.difference - x.expression).abs() < tol, "closure {}, k = {}, tau = {}: derivative {} against the quantity of the source {}", name, k, taus[i], x.difference, x.expression);
                    // (2)
                    let c = [x.shear_check.abs() / (x.potential.abs() + 1e-3), x.trace_check.abs() / x.trace_scale];
                    worst[0] = worst[0].max(c[0]);
                    worst[1] = worst[1].max(c[1]);
                    assert!(c[0] < 2e-3 && c[1] < 5e-3, "closure {}, k = {}, tau = {}: numerical checks {:?}", name, k, taus[i], c);
                }
                assert_eq!(rm[i].momentum_gap, 0.0);
                assert_eq!(rp[i].trace_gap, 0.0);
                // (3) the gaps in units of (a gsum/Hc) times the size of the terms of their equations
                let ratio = rm[i].a_gsum / rm[i].hub;
                let (sm, sp) = (rm[i].shear_gap / (ratio * rm[i].potential.abs()), rp[i].shear_gap / (ratio * rp[i].potential.abs()));
                let tm = rm[i].trace_gap / (ratio * rm[i].trace_scale);
                let mp = rp[i].momentum_gap / (ratio * rp[i].momentum_scale);
                println!("k = {}, tau = {}, a gsum/Hc = {:.2e}: gaps in units of (a gsum/Hc) x (terms of the equation): Momentum: shear {:+.3}, trace {:+.3}; Pressure: shear {:+.3}, momentum {:+.3}; potentials {:.6} (Momentum), {:.6} (Pressure)", k, taus[i], ratio, sm, tm, sp, mp, rm[i].potential, rp[i].potential);
                // (in the closure "Pressure" the shear gap is smaller by the factor (Hc/k)^2 inside the horizon:
                // there this closure and the closure "Shear", in which the gap is zero, coincide)
                let inside = 1.0 + (k / rm[i].hub).powi(2);
                if taus[i] >= 800.0 {
                    for (name, g) in [("shear, Momentum", sm), ("trace, Momentum", tm), ("shear, Pressure, times 1 + (k/Hc)^2", sp * inside), ("momentum, Pressure", mp)] {
                        assert!(g.abs() > 0.02 && g.abs() < 20.0, "k = {}, tau = {}: gap ({}) = {} in units of a gsum/Hc", k, taus[i], name, g);
                    }
                }
                // the potentials of the two closures: equal at early times, different by the order a gsum/Hc later
                let rel = (rm[i].potential / rp[i].potential - 1.0).abs();
                assert!(rel < 3.0 * ratio + 1e-4, "k = {}, tau = {}: potentials differ by {}", k, taus[i], rel);
            }
            assert!((rm[4].potential / rp[4].potential - 1.0).abs() > 1e-3);
        }
        println!("numerical checks with deflation, largest values (relative): equation of the shear {:.1e}, trace equation {:.1e}", worst[0], worst[1]);
    }

    /// The Hubble constant at a fixed angle theta_*: the routine returns the fiducial value
    /// without deflation, and a smaller one with deflation.
    #[test]
    fn the_hubble_constant_at_fixed_angle_falls_with_the_rate() {
        let p = Params::fiducial();
        let target = theta_star_100(&p, &Deflation::none());
        let h0 = h_at_fixed_theta(&p, &Deflation::none(), target);
        assert!((h0 - p.h).abs() < 1e-9, "{}", h0);
        let h1 = h_at_fixed_theta(&p, &Deflation::equal_rates(0.02), target);
        assert!(h1 < h0 && h1 > 0.6, "{}", h1);
    }

    /// The derivatives of the spectra: without deflation the routine of this module gives
    /// the numbers of the routine of the main project.
    #[test]
    fn the_derivatives_of_the_spectra_agree_with_those_of_the_main_project() {
        let p = Params::fiducial();
        let ws = Workspace::new(60);
        let (j, j0) = (jacobian_defl(&p, &Deflation::none(), &ws), spacetimes::forecast::jacobian(&p, &ws));
        for i in 0..NPAR {
            for q in 0..3 {
                for l in 2..=60 {
                    assert_eq!(j.d[i][q][l].to_bits(), j0.d[i][q][l].to_bits(), "parameter {}, spectrum {}, l = {}", i, q, l);
                }
            }
        }
    }
}
