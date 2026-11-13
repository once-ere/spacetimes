//! The history of the model on a deflating spacetime: every coupling-dependent result of
//! the main project, computed again as a function of the deflation.
//!
//! WHY THE COUPLINGS DEPEND ON x4.  Consider fields that do not depend on the deflating
//! coordinates x5, x6, x7 (for 4+4 this is the quantized sector; for 7+1 the sector of
//! low energies, because a wavenumber along a deflating coordinate costs an energy that
//! grows as exp(rate_t x4)).  The action is the integral of sqrt|g| L over the eight
//! coordinates, and sqrt|g| = b_5 b_6 b_7 = Vd(x4) falls as exp(-gsum x4),
//! gsum = g55 + g66 + g77.  With the weighted fields psihat = sqrt(Vd) psi and
//! Phihat = sqrt(Vd) Phi_VS, whose kinetic terms have the standard normalization, the
//! Lagrangian of the main project is recovered with the couplings
//!
//!     y_VS -> y_VS / sqrt(Vd),     lambda_VS -> lambda_VS / Vd,     v_VS -> v_VS sqrt(Vd),
//!
//! and with the extra term +(gsum^2/4)|Phihat|^2 for the scalar (module `scalar`).
//! Hence, with the DEFLATION CLOCK
//!
//!     tau = gsum (x4 - x4_ref),        Dd(tau) = Vd(x4_ref)/Vd(x4) = exp(tau),
//!
//!     m = y v/sqrt(2)  and  m_s = sqrt(2 lambda) v      do not change;
//!     g_VS = m/v ~ exp(tau/2);   alpha_VS = g_VS^2/(4 pi),  G_VS,  D_VS = alpha_VS m/m_s,
//!     C = g_VS^2 m^2/m_s^2 = 4 pi alpha_VS (m/m_s)^2          all grow as exp(tau).
//!
//! Only the SUM of the three rates enters, and the constants const5, const6, const7 only
//! through the values of the couplings at the reference time.
//!
//! WHAT THIS MODULE DOES.  It calls the solvers of the main project (gap equation, bound
//! pair, normal matter, vacuum potential, finite temperature, vacuum condensate) along
//! the clock, and converts every threshold of the main project into a value of tau,
//! that is, into a time.  "Adiabatic" throughout: at each tau the equilibrium state for
//! the couplings of that moment is computed.  Whether the state can follow is the
//! subject of the module `quench`.
//!
//! UNITS: hbar = c = k_B = 1 and the fermion mass m = 1.

use spacetimes::cjt_normal::{critical_c, vacuum_instability, Medium, Thermal};
use spacetimes::cjt_vacuum::critical_alpha;
use spacetimes::dimer;
use spacetimes::gap::{critical_temperature, fermi_grid, solve_gap, Pairing};

const PI: f64 = std::f64::consts::PI;

/// The couplings of the model as functions of the deflation clock.
#[derive(Clone, Copy, Debug)]
pub struct Clock {
    /// alpha_VS at the reference time tau = 0
    pub alpha_ref: f64,
    /// m_s/m (constant)
    pub ms: f64,
}

impl Clock {
    pub fn alpha(&self, tau: f64) -> f64 {
        self.alpha_ref * tau.exp()
    }
    pub fn g(&self, tau: f64) -> f64 {
        (4.0 * PI * self.alpha(tau)).sqrt()
    }
    /// D_VS = alpha_VS m/m_s
    pub fn d(&self, tau: f64) -> f64 {
        self.alpha(tau) / self.ms
    }
    /// C = g_VS^2 m^2/m_s^2
    pub fn c(&self, tau: f64) -> f64 {
        4.0 * PI * self.alpha(tau) / (self.ms * self.ms)
    }
    /// The clock reading at which alpha_VS has the value `alpha`.
    pub fn tau_of_alpha(&self, alpha: f64) -> f64 {
        (alpha / self.alpha_ref).ln()
    }
}

/// One row of the gap history.
#[derive(Clone, Copy, Debug)]
pub struct GapRow {
    pub g: f64,
    pub alpha: f64,
    pub lambda_s: f64,
    pub lambda_p: f64,
    /// gaps of the vector channel, of the two-form channel and with the radial mode only, in units of E_F
    pub gap_v: f64,
    pub gap_t: f64,
    pub gap_s: f64,
    /// T_c / gap of the vector channel (NaN if no gap was found)
    pub tc_over_gap: f64,
    pub ef: f64,
}

/// The gap at the Fermi surface for the coupling g (fermion mass 1, radial mass m_s,
/// Fermi momentum k_f), in the three variants of the kernel of the main project.
pub fn gap_point(g: f64, m_s: f64, k_f: f64, with_tc: bool) -> GapRow {
    let m = 1.0;
    let base = Pairing { m, g, m_s, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f };
    let ef = base.mu() - m;
    let grid = fermi_grid(k_f, 60.0 * m.max(k_f), 11, 8);
    let lam_s = Pairing { phase: false, ..base }.lambda();
    let lam_p = Pairing { scalar: false, ..base }.lambda();
    let variants = [Pairing { ..base }, Pairing { phase_sign: -1.0, ..base }, Pairing { phase: false, ..base }];
    let mut gaps = [0.0; 3];
    let mut tc_over_gap = f64::NAN;
    for (iv, p) in variants.iter().enumerate() {
        let lam = p.lambda();
        let start = (2.0 * ef * (-1.0 / lam.max(1e-3)).exp()).max(1e-11 * ef).min(ef);
        let sol = solve_gap(p, &grid, 0.0, start, 1e-9, 40000);
        let gap = if sol.converged && sol.delta_f > 1e-7 * ef { sol.delta_f } else { 0.0 };
        gaps[iv] = gap;
        if iv == 0 && gap > 0.0 && with_tc {
            if let Some(tc) = critical_temperature(p, &grid, 1e-11 * ef, 5.0 * (ef + m)) {
                tc_over_gap = tc / gap;
            }
        }
    }
    GapRow { g, alpha: g * g / (4.0 * PI), lambda_s: lam_s, lambda_p: lam_p, gap_v: gaps[0] / ef, gap_t: gaps[1] / ef, gap_s: gaps[2] / ef, tc_over_gap, ef }
}

/// The gap along the clock: `n` values of tau between tau_a and tau_b, with g = g_ref exp(tau/2).
pub fn gap_history(g_ref: f64, m_s: f64, k_f: f64, tau_a: f64, tau_b: f64, n: usize, with_tc: bool) -> Vec<(f64, GapRow)> {
    (0..n)
        .map(|i| {
            let tau = tau_a + (tau_b - tau_a) * i as f64 / (n - 1) as f64;
            (tau, gap_point(g_ref * (0.5 * tau).exp(), m_s, k_f, with_tc))
        })
        .collect()
}

/// One row of the history of the bound pair: D, scattering length times m_s, number of
/// bound states, binding energy times m/m_s^2 (0 if unbound).
pub fn dimer_point(d: f64) -> [f64; 4] {
    let a = dimer::scattering_length(d);
    let nb = dimer::bound_states(d);
    let e = dimer::binding(d).unwrap_or(0.0);
    [d, a, nb as f64, e]
}

/// The smallest C at which normal matter of the Fermi momentum k_f is in the abnormal
/// state (the Vorton-Scalar has lost its expectation value: m* = 0), by bisection.
/// Returns None if the matter is still normal at C = 400.
pub fn abnormal_c(k_f: f64, sea: bool) -> Option<f64> {
    let abnormal = |c: f64| Medium { c, sea }.mass(k_f) < 1e-3;
    if !abnormal(400.0) {
        return None;
    }
    let (mut lo, mut hi) = (1e-3, 400.0);
    if abnormal(lo) {
        return Some(lo);
    }
    for _ in 0..60 {
        let mid = (lo * hi).sqrt();
        if abnormal(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Some((lo * hi).sqrt())
}

/// A threshold of the model, expressed as a value of alpha_VS.
#[derive(Clone, Debug)]
pub struct Event {
    pub name: String,
    /// the quantity that crosses a critical value, and that value
    pub quantity: String,
    pub critical: f64,
    /// the alpha_VS at which it happens (for the given m_s/m and, where relevant, k_F/m and cutoff)
    pub alpha: f64,
}

/// The thresholds of the main project as values of alpha_VS, for the mass ratio
/// `ms` = m_s/m, the Fermi momentum `k_f` (units of m) of the matter considered, and the
/// cutoff `cutoff` (units of m) of the vacuum-condensate calculation.
pub fn events(ms: f64, k_f: f64, cutoff: f64) -> Vec<Event> {
    let mut ev: Vec<Event> = Vec::new();
    // bound pair: D = alpha m/m_s > D_c
    let dc = dimer::critical_d();
    ev.push(Event { name: "two fermions form a bound pair".into(), quantity: "D_VS = alpha_VS m/m_s".into(), critical: dc, alpha: dc * ms });
    // C = 4 pi alpha (m/m_s)^2
    let c_to_alpha = |c: f64| c * ms * ms / (4.0 * PI);
    let (c1, c2) = (critical_c(false), critical_c(true));
    ev.push(Event { name: "matter is self-bound (Fermi sea only)".into(), quantity: "C = g_VS^2 m^2/m_s^2".into(), critical: c1, alpha: c_to_alpha(c1) });
    ev.push(Event { name: "matter is self-bound (with the negative-energy states)".into(), quantity: "C".into(), critical: c2, alpha: c_to_alpha(c2) });
    // the one-loop vacuum potential falls below its vacuum value at a field below 10 v, below 3 v
    for target in [10.0_f64, 3.0] {
        let (mut lo, mut hi) = (0.05_f64, 60.0_f64);
        for _ in 0..60 {
            let mid = (lo * hi).sqrt();
            if vacuum_instability(mid).1 > target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let c = (lo * hi).sqrt();
        ev.push(Event { name: format!("the one-loop potential of the Vorton-Scalar falls below its vacuum value at a field of {} v_VS", target), quantity: "C".into(), critical: c, alpha: c_to_alpha(c) });
    }
    // abnormal matter at the given density
    for sea in [false, true] {
        if let Some(c) = abnormal_c(k_f, sea) {
            ev.push(Event { name: format!("matter of Fermi momentum {} m becomes abnormal (massless fermions){}", k_f, if sea { ", with the negative-energy states" } else { "" }), quantity: "C".into(), critical: c, alpha: c_to_alpha(c) });
        }
    }
    // pair condensate in the vacuum
    let (ac, _) = critical_alpha(ms, cutoff);
    ev.push(Event { name: format!("pair condensate in the vacuum (ladder equation, cutoff {} m)", cutoff), quantity: "alpha_VS".into(), critical: ac, alpha: ac });
    ev.sort_by(|a, b| a.alpha.partial_cmp(&b.alpha).unwrap());
    ev
}

/// The melting temperatures of the expectation value of the Vorton-Scalar for the
/// coupling g and the mass ratio m_s/m: (lambda, C, T_high/m, T_largeN/m, T_half/m, T_zero/m, phi before the drop).
pub fn thermal_point(g: f64, ms: f64) -> [f64; 7] {
    let lambda = 0.5 * g * g * ms * ms;
    let th = Thermal { g, lambda };
    let (t_half, t_zero, phi_before) = th.melting();
    [lambda, g * g / (ms * ms), th.t_high(), th.t_large_n(), t_half, t_zero, phi_before]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_clock_scales_all_dimensionless_couplings_alike() {
        let c = Clock { alpha_ref: 0.3, ms: 0.25 };
        for tau in [-2.0_f64, 0.0, 1.5] {
            let f = tau.exp();
            assert!((c.alpha(tau) - 0.3 * f).abs() < 1e-14);
            assert!((c.d(tau) - 1.2 * f).abs() < 1e-13);
            assert!((c.c(tau) - 4.0 * PI * 0.3 * 16.0 * f).abs() < 1e-11);
            assert!((c.g(tau).powi(2) / (4.0 * PI) - c.alpha(tau)).abs() < 1e-14);
            assert!((c.tau_of_alpha(c.alpha(tau)) - tau).abs() < 1e-13);
        }
    }

    #[test]
    fn thresholds_of_the_main_project_are_reproduced() {
        assert!((dimer::critical_d() - 1.67981).abs() < 1e-4);
        assert!((critical_c(false) - 3.0 * PI * PI / 8.0).abs() < 2e-3);
        assert!((critical_c(true) - 3.0 * PI * PI / 5.0).abs() < 2e-3);
    }

    #[test]
    fn abnormal_matter_sets_in_near_the_weak_coupling_estimate() {
        // k_F = pi m/(2 sqrt(C))  <=>  C = pi^2/(4 k_F^2); for k_F = 1: C = 2.47
        let c = abnormal_c(1.0, false).unwrap();
        assert!(c > 1.5 && c < 3.5, "C = {}", c);
    }

    #[test]
    fn events_are_ordered_and_scale_with_the_mass_ratio() {
        let e1 = events(0.1, 0.5, 100.0);
        for w in e1.windows(2) {
            assert!(w[0].alpha <= w[1].alpha);
        }
        // the bound pair forms at alpha = D_c m_s/m
        let pair = e1.iter().find(|e| e.name.contains("bound pair")).unwrap();
        assert!((pair.alpha - 0.167981).abs() < 1e-4);
    }
}
