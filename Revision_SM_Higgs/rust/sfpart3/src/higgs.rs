//! Can the superfluid play the role of the Higgs particle?  The dimensionless numbers by which the superfluid of each
//! method is judged as the Higgs sector of its own gauge group SU(2)_v x U(1)^F_LL, computed from the results of Parts II
//! and III (Section 14.6 of superfluid.md; asked for by the author on 21 November 2026).  The DiracLike spinor has
//! electric charge 0 and no electroweak quantum numbers, so the superfluid is a singlet of the electroweak group and
//! cannot be the Higgs field of the Standard Model whatever these numbers are; the module therefore compares nothing
//! with the Standard Model and gives no value in GeV (the scale of the new group is not known).  (Corrected on
//! 21 November 2026: the first version compared these numbers with the Higgs boson of the Standard Model and set the
//! scale of the breaking to 246.22 GeV; Section 17.6 of superfluid.md.)
//!
//! THE SCALE OF THE BREAKING.  M_W = g^v_LL f_I with the isospin decay constant f_I = (chi^11)^(1/2) = (2 Z)^(1/2) Phi0
//! (Sections 12.2 and 13.3).  With the convention M_W = g v/2 by which the Standard Model defines its vacuum value, the
//! scale of the breaking is
//!     v_SF = 2 f_I,
//! which does not depend on the gauge coupling.  The phase (fermion-number) decay constant is f_F = (chi^FF/4)^(1/2)
//! = (2 (Z + Y))^(1/2) Phi0.
//!
//! THE NUMBERS.  M_H is the mass of the amplitude (Higgs) mode of the linear response (Section 10.5, Table 13.1), M_NU
//! that of the non-unitary doublet (two scalars of the charges Q_LL = +-1 of the unbroken U(1)^Q, which is not
//! electromagnetism), E_gap the gap of the spinor quasi-particles at T = 0:
//!     M_H / v_SF,  lambda_SF = M_H^2 / (2 v_SF^2)  (the quartic coupling in the convention m_H^2 = 2 lambda v^2),
//!     M_H / M_W at g^v_LL = 0.7,  M_H / (2 E_gap) (the binding of the two-body state),  M_NU / M_H,  f_F / f_I.
//!
//! THE WIDTH.  The kinetic part (12.4), written with the fields of (13.4), contains the couplings of the amplitude field
//! h to the Goldstone fields: (1/f_F) h (d pi_k)^2 for the three canonical Goldstone fields pi_0 = f_F theta_0,
//! pi_k = f_I theta_k (k = 1, 2) (the coefficient of h (d theta_0)^2 is f_F, that of h (d theta_k)^2 is f_I^2/f_F; check
//! P3-H2 differentiates (12.4) numerically).  A coupling (g/2) h (d pi)^2 gives the amplitude g M_H^2/2 for the decay
//! into two massless Goldstone fields and the width |amplitude|^2/(32 pi M_H) = g^2 M_H^3/(128 pi) (identical particles),
//! that is M_H^3/(32 pi f_F^2) for g = 2/f_F in each channel, and the three channels (one for each broken generator)
//! give
//!     Gamma(h -> Goldstone pairs) = 3 M_H^3 / (32 pi f_F^2).
//! With the gauge fields the Goldstone fields are the longitudinal W^+-, Z of the new group, whose masses are far below
//! M_H/2 here, so the same width is that of h -> W^+ W^-, Z Z of the new group (the equivalence theorem; with f_F
//! replaced by the vacuum value v of the Standard Model it is the width 3 m^3/(32 pi v^2) of a heavy Higgs boson, a check
//! of the normalization).  The effective Lagrangian is used at the energy M_H, where it is not accurate (its M_H differs
//! from the linear response by 7 to 17 per cent, Table 13.1): the width is an estimate of the order of magnitude.  The
//! mean-field response of the methods has no such decays (it contains no pairs of collective modes); the width is the
//! first correction beyond the methods.
use sfcore::c64::c;

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
        g_phase: coupling(z, y, phi0, true),
        g_iso: coupling(z, y, phi0, false),
    }
}
