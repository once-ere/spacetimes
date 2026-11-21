//! Can the superfluid play the role of the Higgs particle?  The dimensionless numbers of the five methods that a Higgs
//! sector is judged by, computed from the results of Parts II and III (Section 14.6 of superfluid.md; asked for by the
//! author on 21 November 2026).
//!
//! THE SCALE OF THE BREAKING.  In the Standard Model M_W = g v/2 with v = 246.22 GeV.  Here M_W = g^v_LL f_I with the
//! isospin decay constant f_I = (chi^11)^(1/2) = (2 Z)^(1/2) Phi0 (Sections 12.2 and 13.3), so the analogue of v is
//!     v_SF = 2 f_I,
//! which does not depend on the gauge coupling.  The phase (fermion-number) decay constant is f_F = (chi^FF/4)^(1/2)
//! = (2 (Z + Y))^(1/2) Phi0.
//!
//! THE NUMBERS.  M_H is the mass of the amplitude (Higgs) mode of the linear response (Section 10.5, Table 13.1), M_NU
//! that of the non-unitary doublet (two scalars of charge +-1), E_gap the gap of the spinor quasi-particles at T = 0:
//!     M_H / v_SF  (Standard Model: 125.20 GeV / 246.22 GeV = 0.5085),
//!     lambda_SF = M_H^2 / (2 v_SF^2)  (the quartic coupling in the convention m_H^2 = 2 lambda v^2; Standard Model
//!                 0.129; the tree-level bound of perturbative unitarity is lambda < 8 pi / 3 = 8.38),
//!     M_H / M_W at g^v_LL = 0.7 (Standard Model 1.558), M_H / (2 E_gap) (the binding of the two-body state),
//!     M_NU / M_H (the Standard Model has no such scalars), f_F / f_I (the Standard Model has one decay constant for
//!     its three Goldstone fields, the custodial symmetry, rho = 1).
//!
//! THE WIDTH.  The kinetic part (12.4), written with the fields of (13.4), contains the couplings of the amplitude field
//! h to the Goldstone fields: (1/f_F) h (d pi_k)^2 for the three canonical Goldstone fields pi_0 = f_F theta_0,
//! pi_k = f_I theta_k (k = 1, 2) (the coefficient of h (d theta_0)^2 is f_F, that of h (d theta_k)^2 is f_I^2/f_F; check
//! P3-H2 differentiates (12.4) numerically).  A coupling (g/2) h (d pi)^2 gives the amplitude g M_H^2/2 for the decay
//! into two massless Goldstone fields and the width |amplitude|^2/(32 pi M_H) = g^2 M_H^3/(128 pi) (identical particles),
//! that is M_H^3/(32 pi f_F^2) for g = 2/f_F in each channel, and the three channels give
//!     Gamma(h -> Goldstone pairs) = 3 M_H^3 / (32 pi f_F^2).
//! With the gauge fields the Goldstone fields are the longitudinal W^+-, Z, whose masses are far below M_H/2 here, so
//! the same width is that of h -> W^+ W^-, Z Z (the equivalence theorem; for the Standard Model with f_F = v this is the
//! width 3 m_H^3/(32 pi v^2) of a heavy Higgs boson).  The effective Lagrangian is used at the energy M_H, where it is
//! not accurate (its M_H differs from the linear response by 7 to 17 per cent, Table 13.1): the width is an estimate of
//! the order of magnitude.  The mean-field response of the methods has no such decays (it contains no pairs of
//! collective modes); the width is the first correction beyond the methods.
use sfcore::c64::c;

/// the Standard Model values (Particle Data Group 2024): m_H, v = (sqrt(2) G_F)^(-1/2), M_W, M_Z in GeV
pub const SM_MH: f64 = 125.20;
pub const SM_V: f64 = 246.22;
pub const SM_MW: f64 = 80.3692;
pub const SM_MZ: f64 = 91.1880;

/// the ratios of the Standard Model that have an analogue here: M_H/v, lambda = M_H^2/(2 v^2), M_H/M_W, M_W/M_Z
pub fn standard_model() -> [f64; 4] {
    [SM_MH / SM_V, SM_MH * SM_MH / (2.0 * SM_V * SM_V), SM_MH / SM_MW, SM_MW / SM_MZ]
}

/// the numbers of one method
#[derive(Clone, Debug)]
pub struct Higgs {
    pub v_sf: f64,
    pub f_f: f64,
    pub f_i: f64,
    pub mh: f64,
    pub mnu: f64,
    pub mw: f64,
    pub mz: f64,
    pub mh_over_v: f64,
    pub lambda: f64,
    pub mh_over_mw: f64,
    pub mh_over_2gap: f64,
    pub mnu_over_mh: f64,
    pub ff_over_fi: f64,
    pub mw_over_mz: f64,
    pub width_over_mh: f64,
    pub tc_over_v: f64,
    /// the scale Lambda_LL in GeV if v_SF is set to 246.22 GeV, and the masses of h and of the spinor quasi-particles
    /// (the gap) in GeV on that scale
    pub lambda_gev: f64,
    pub mh_gev: f64,
    pub gap_gev: f64,
    pub tc_gev: f64,
    /// the couplings of h to (d theta_0)^2 and to (d theta_1)^2 from finite differences of (12.4)
    pub g_phase: f64,
    pub g_iso: f64,
}

/// the kinetic part (12.4) for one derivative direction, Z |dPhi|^2 + Y |Phi^dag dPhi|^2 / |Phi|^2, for the field
/// Phi = (Phi0 + h/N_h) e^{i theta_0} R_1(theta_1) e_3 (R_1 the rotation about e_1, N_h = (2 (Z + Y))^(1/2)) at theta = 0,
/// with the derivatives dh, dth0, dth1
fn kinetic(z: f64, y: f64, phi0: f64, h: f64, dh: f64, dth0: f64, dth1: f64) -> f64 {
    let nh = (2.0 * (z + y)).sqrt();
    let r = phi0 + h / nh;
    // Phi = r e_3 at theta = 0; dPhi = (dh/N_h) e_3 + i r dth0 e_3 + r dth1 (d/dtheta R_1 e_3) = ... + r dth1 e_2
    let phi = [c(0.0, 0.0), c(0.0, 0.0), c(r, 0.0)];
    let dphi = [c(0.0, 0.0), c(r * dth1, 0.0), c(dh / nh, r * dth0)];
    let n2: f64 = phi.iter().map(|x| x.norm_sqr()).sum();
    let d2: f64 = dphi.iter().map(|x| x.norm_sqr()).sum();
    let mut pd = c(0.0, 0.0);
    for i in 0..3 {
        pd += phi[i].conj() * dphi[i];
    }
    z * d2 + y * pd.norm_sqr() / n2
}

/// the coefficient of h (d theta)^2 in (12.4): d/dh of the coefficient of (d theta)^2, by central differences
fn coupling(z: f64, y: f64, phi0: f64, phase: bool) -> f64 {
    let dl = 1e-4;
    let k = |h: f64| if phase { kinetic(z, y, phi0, h, 0.0, 1.0, 0.0) } else { kinetic(z, y, phi0, h, 0.0, 0.0, 1.0) };
    (k(dl) - k(-dl)) / (2.0 * dl)
}

pub fn run(phi0: f64, z: f64, y: f64, mh: f64, mnu: f64, gv: f64, gf: f64, gap0: f64, tc: f64) -> Higgs {
    let f_i = (2.0 * z).sqrt() * phi0;
    let f_f = (2.0 * (z + y)).sqrt() * phi0;
    let v_sf = 2.0 * f_i;
    let mw = gv * f_i;
    let mz = gf * 2.0 * f_f;
    let lambda_gev = SM_V / v_sf;
    Higgs {
        v_sf,
        f_f,
        f_i,
        mh,
        mnu,
        mw,
        mz,
        mh_over_v: mh / v_sf,
        lambda: mh * mh / (2.0 * v_sf * v_sf),
        mh_over_mw: mh / mw,
        mh_over_2gap: mh / (2.0 * gap0),
        mnu_over_mh: mnu / mh,
        ff_over_fi: f_f / f_i,
        mw_over_mz: mw / mz,
        width_over_mh: 3.0 * mh * mh / (32.0 * std::f64::consts::PI * f_f * f_f),
        tc_over_v: tc / v_sf,
        lambda_gev,
        mh_gev: mh * lambda_gev,
        gap_gev: gap0 * lambda_gev,
        tc_gev: tc * lambda_gev,
        g_phase: coupling(z, y, phi0, true),
        g_iso: coupling(z, y, phi0, false),
    }
}
