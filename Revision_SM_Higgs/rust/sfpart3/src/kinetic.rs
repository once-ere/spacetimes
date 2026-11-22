//! The kinetic part of the effective Lagrangians (Section 12.2 of superfluid.md): the stiffnesses of the phase and of
//! the isospin direction of the field Phi^X, from the static susceptibilities (the time components) and, for comparison,
//! from states with a uniform twist (the space components).
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::uniform::{susceptibility_column, Model};

pub struct Stiffness {
    pub chi11: f64,
    pub chiff: f64,
    pub chi33: f64,
    /// the squared decay constants of the phase (F) and of the isospin rotations (I), from the time components
    pub ff_t: f64,
    pub fi_t: f64,
    /// the same from the twisted states (space components), and the twist of the unbroken T^3 (zero in a gauge-invariant
    /// regularization)
    pub ff_s: f64,
    pub fi_s: f64,
    pub f3_s: f64,
}

/// the energy of a uniform twist Q along the third space axis of the charge t (2x2 isospin matrix) of the superfluid
/// minus that of the normal state with the same twist, divided by Q^2, extrapolated to Q -> 0 (steps 0.01, 0.02)
pub fn twist(model: &mut Model, n0: &CMat, p0: &CMat, e0: f64, t2: &CMat) -> f64 {
    let mut de = vec![];
    for q in [0.01, 0.02] {
        let x = d8(&alpha(3)).mul(&i8(t2)).scale_r(q);
        model.ext_n = Some(x.clone());
        let s = model.solve(&n0.add(&x), p0, 0.6, 1e-14, 40000);
        let nrm = model.solve(&x, &CMat::zeros(8), 0.6, 1e-14, 40000);
        model.ext_n = None;
        de.push((s.energy - nrm.energy - e0) / (q * q));
    }
    (4.0 * de[0] - de[1]) / 3.0
}

pub fn stiffness(model: &mut Model, n0: &CMat, p0: &CMat, e0: f64) -> Stiffness {
    let c1 = susceptibility_column(model, n0, p0, 0, 2e-3);
    let c3 = susceptibility_column(model, n0, p0, 2, 2e-3);
    let cf = susceptibility_column(model, n0, p0, 3, 2e-3);
    // phase twist: the pair phase gradient is 2Q and the energy (f^2/2)(2Q)^2; isospin twist: (f^2/2) Q^2
    let ff_s = twist(model, n0, p0, e0, &CMat::identity(2)) / 2.0;
    let fi_s = 2.0 * twist(model, n0, p0, e0, &sigma(1).scale_r(0.5));
    let f3_s = 2.0 * twist(model, n0, p0, e0, &sigma(3).scale_r(0.5));
    Stiffness { chi11: c1[0], chiff: cf[3], chi33: c3[2], ff_t: cf[3] / 4.0, fi_t: c1[0], ff_s, fi_s, f3_s }
}
