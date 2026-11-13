//! Symmetries on the deflating spacetimes: which transformations of the flat spacetime
//! survive the deflation, what happens to the internal symmetry of the paired state,
//! and the fermion zero modes on a vortex line of the Vorton-Scalar.
//!
//! (1) ISOMETRIES.  The flat eight-dimensional spacetime has 8 translations and 28
//!     rotations and boosts, one for each pair (A, B) of coordinates, generated on spinors
//!     by the scaled commutators Sigma^(AB).  On the deflating spacetime a rotation of
//!     the pair (A, B), neither of which is x4, is the vector field
//!         xi = c_B x^A d/dx^B - c_A x^B d/dx^A .
//!     The Killing equations require c_A and c_B to be constants with g_BB c_B = g_AA c_A; this
//!     can hold at every x4 only if g_AA/g_BB does not depend on x4, that is, if the two
//!     coordinates deflate at the same rate (or not at all).  A boost or rotation that
//!     involves x4 shifts x4 by an amount that depends on position and changes the scale
//!     factors; it is never an isometry.  `rotation_is_isometry` verifies both statements
//!     by exact arithmetic at two points of the spacetime.
//!
//! (2) INTERNAL SYMMETRY.  For fields that do not depend on the extra coordinates the
//!     dynamics is fixed by the five matrices alpha^1, alpha^2, alpha^3, beta, i beta chi,
//!     exactly as on the flat spacetime (the weighted field obeys the flat equations with
//!     couplings that depend on x4).  The commutant of these matrices, and with it the
//!     symmetry SU(2)_A x SU(2)_B x U(1)_F x U(1)_E of the main project, is therefore
//!     unchanged; this module runs the analysis of the main project again.  What the
//!     deflation changes is the geometric meaning: by (1), the rotations of the extra
//!     axes that realize SU(2)_A x SU(2)_B on the flat 7+1 spacetime are no longer
//!     isometries.
//!
//! (3) ZERO MODES ON A VORTEX LINE.  In a static background Phi_VS = (phi1 + i phi2)/sqrt(2)
//!     of the Vorton-Scalar the weighted fermion field obeys i d psihat/dx4 = h psihat,
//!         h = alpha^j p_j + g phi1 beta + g phi2 (i beta chi) .
//!     For a straight line along x3 around which the phase of Phi_VS winds once,
//!     phi1 + i phi2 ~ f(r) exp(i theta) in the plane (x1, x2), the solutions of zero
//!     energy that do not depend on x3 are exp(-m Int f dr) w0 with a constant spinor w0
//!     that satisfies
//!         (i beta alpha^1) w0 = -w0 ,       (alpha^2 beta chi) w0 = -w0 .
//!     The two matrices are commuting Hermitian involutions.  Exact results (both
//!     spacetimes): the space of such w0 has dimension 4; the product of the five
//!     matrices, Pc = i alpha^1 alpha^2 alpha^3 chi, is a central Hermitian traceless
//!     involution (the generator of U(1)_E up to sign); on the four zero modes
//!     alpha^3 = Pc, with the eigenvalues +1, +1, -1, -1.  Since the energy of a zero mode
//!     with wavenumber k_3 along the line is alpha^3 k_3, two modes run in each direction
//!     at the speed of light.  For the opposite winding alpha^3 = -Pc.

use crate::metric::{Point, Sig, VectorAt};
use crate::qmat::QMat;
use crate::rat::{Qc, Rat};
use spacetimes::gamma::Gammas;

/// Is the rotation (or boost) of the coordinate pair (a, b) an isometry of the deflating
/// spacetime with the integer rates `rate`?  The candidate vector field
/// xi^b = c_b x^a, xi^a = -c_a x^b has constant coefficients chosen so that it preserves
/// the metric at a first point (c_b = g_aa, c_a = g_bb there); its Lie derivative is then
/// evaluated exactly at that point and at a second point whose x4 is larger by ln 2
/// (where the scale factors are b_t 2^(-rate_t), rational for integer rates).
pub fn rotation_is_isometry(sig: Sig, rate: [i128; 3], b: [Rat; 3], a: usize, bb: usize) -> bool {
    assert!(a != bb);
    let r = [Rat::int(rate[0]), Rat::int(rate[1]), Rat::int(rate[2])];
    let p1 = Point { sig, rate: r, b };
    let half_pow = |n: i128| Rat::new(1, 1i128 << n);
    let p2 = Point { sig, rate: r, b: [b[0] * half_pow(rate[0]), b[1] * half_pow(rate[1]), b[2] * half_pow(rate[2])] };
    let g1 = p1.metric();
    let (ca, cb) = (g1[bb].v, g1[a].v);
    // a point with non-zero rational coordinates
    let x = [Rat::new(1, 3), Rat::new(-2, 1), Rat::new(5, 7), Rat::new(1, 2), Rat::new(3, 5), Rat::new(3, 4), Rat::new(-1, 5), Rat::new(2, 3)];
    let mut xi = [Rat::ZERO; 8];
    let mut dxi = [[Rat::ZERO; 8]; 8];
    xi[bb] = cb * x[a];
    xi[a] = -ca * x[bb];
    dxi[a][bb] = cb;
    dxi[bb][a] = -ca;
    let v = VectorAt { name: format!("rotation of x{} and x{}", a, bb), xi, dxi };
    let zero = |p: &Point<Rat>| crate::metric::lie_derivative_of_metric(p, &v).iter().all(|row| row.iter().all(|c| c.is_zero()));
    zero(&p1) && zero(&p2)
}

/// The number of pairs (A, B) whose rotation is an isometry, and the list of those pairs.
pub fn surviving_rotations(sig: Sig, rate: [i128; 3], b: [Rat; 3]) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for a in 0..8 {
        for bb in (a + 1)..8 {
            if rotation_is_isometry(sig, rate, b, a, bb) {
                out.push((a, bb));
            }
        }
    }
    out
}

/// The matrices of the vortex zero-mode problem of one spacetime, exact.
pub struct VortexAlgebra {
    /// alpha^1, alpha^2, alpha^3, beta, i beta chi
    pub five: Vec<QMat>,
    /// i beta alpha^1 and alpha^2 beta chi (= i (i beta chi) alpha^2)
    pub inv1: QMat,
    pub inv2: QMat,
    /// Pc = i alpha^1 alpha^2 alpha^3 chi
    pub pc: QMat,
}

impl VortexAlgebra {
    pub fn new(gm: &Gammas, sig: Sig) -> VortexAlgebra {
        // conventions of the main project (kernel.rs):
        //   7+1: beta = gamma^4, alpha^j = gamma^4 gamma^j, chi = i omega;
        //   4+4: beta = -i gamma^4, alpha^j = -gamma^4 gamma^j, chi = omega.
        let (g, om): (Vec<QMat>, QMat) = match sig {
            Sig::S1671 => {
                let refs: Vec<&spacetimes::imat::IMat> = gm.gamma1671.iter().collect();
                (gm.gamma1671.iter().map(QMat::from_imat).collect(), QMat::from_imat(&spacetimes::imat::IMat::product(&refs)))
            }
            Sig::S1644 => (gm.gamma1644.iter().map(QMat::from_imat).collect(), QMat::from_imat(&gm.t16_8)),
        };
        let (beta, alpha, chi): (QMat, Vec<QMat>, QMat) = match sig {
            Sig::S1671 => (g[4].clone(), (1..4).map(|j| g[4].mul(&g[j])).collect(), om.times_i()),
            Sig::S1644 => (g[4].times_i().neg(), (1..4).map(|j| g[4].mul(&g[j]).neg()).collect(), om),
        };
        let b5 = beta.mul(&chi).times_i();
        let inv1 = beta.mul(&alpha[0]).times_i();
        let inv2 = b5.mul(&alpha[1]).times_i();
        let pc = alpha[0].mul(&alpha[1]).mul(&alpha[2]).mul(&chi).times_i();
        VortexAlgebra { five: vec![alpha[0].clone(), alpha[1].clone(), alpha[2].clone(), beta, b5], inv1, inv2, pc }
    }

    /// The projector on the spinors w0 with inv1 = s1, inv2 = s2 (s1, s2 = +-1).
    pub fn projector(&self, s1: i128, s2: i128) -> QMat {
        let one = QMat::identity(16);
        let a = one.add(&self.inv1.scale(Qc::int(s1)));
        let b = one.add(&self.inv2.scale(Qc::int(s2)));
        a.mul(&b).scale_rat(Rat::new(1, 4))
    }
}

/// The exact statements of item (3) for one spacetime.  Returns a list of (statement, true/false).
pub fn vortex_checks(gm: &Gammas, sig: Sig) -> Vec<(String, bool)> {
    let v = VortexAlgebra::new(gm, sig);
    let one = QMat::identity(16);
    let mut out: Vec<(String, bool)> = Vec::new();
    // the five matrices: Hermitian, square one, mutually anticommuting
    let mut ok = true;
    for i in 0..5 {
        ok &= v.five[i].is_hermitian() && v.five[i].mul(&v.five[i]) == one;
        for j in (i + 1)..5 {
            ok &= v.five[i].anti(&v.five[j]).is_zero();
        }
    }
    out.push(("alpha^1, alpha^2, alpha^3, beta and i beta chi are Hermitian, have square one and anticommute with one another".into(), ok));
    // the two involutions
    let ok = v.inv1.is_hermitian() && v.inv2.is_hermitian() && v.inv1.mul(&v.inv1) == one && v.inv2.mul(&v.inv2) == one && v.inv1.comm(&v.inv2).is_zero();
    out.push(("i beta alpha^1 and alpha^2 beta chi are commuting Hermitian matrices with square one".into(), ok));
    // dimensions of the four joint eigenspaces
    let mut dims = true;
    for s1 in [1, -1] {
        for s2 in [1, -1] {
            let p = v.projector(s1, s2);
            dims &= p.mul(&p) == p && p.trace() == Qc::int(4);
        }
    }
    out.push(("each of the four joint eigenspaces of the two matrices has dimension 4: a vortex line of unit winding carries four zero modes of the sixteen-component field".into(), dims));
    // Pc central, Hermitian, traceless, involution, equal to the product of the five
    let mut central = v.pc.is_hermitian() && v.pc.mul(&v.pc) == one && v.pc.trace().is_zero();
    for m in &v.five {
        central &= v.pc.comm(m).is_zero();
    }
    let prod = v.five[0].mul(&v.five[1]).mul(&v.five[2]).mul(&v.five[3]).mul(&v.five[4]);
    central &= prod == v.pc;
    out.push(("Pc = i alpha^1 alpha^2 alpha^3 chi equals the product of the five matrices, commutes with each of them, is Hermitian and traceless and has square one: it is, up to sign, the generator P of U(1)_E".into(), central));
    // zero-mode equation for winding +1 and -1, and alpha^3 on the zero modes
    let (a1, a2, a3, beta, b5) = (&v.five[0], &v.five[1], &v.five[2], &v.five[3], &v.five[4]);
    let pm = v.projector(-1, -1);
    // (i alpha^1 + beta) w0 = 0 and (i alpha^2 + i beta chi) w0 = 0
    let eq_plus = a1.times_i().add(beta).mul(&pm).is_zero() && a2.times_i().add(b5).mul(&pm).is_zero();
    let a3_plus = a3.sub(&v.pc).mul(&pm).is_zero();
    let split_plus = pm.mul(a3).trace().is_zero();
    out.push(("winding +1: every w0 with (i beta alpha^1) w0 = -w0, (alpha^2 beta chi) w0 = -w0 solves the zero-energy equations (i alpha^1 + beta) w0 = 0, (i alpha^2 + i beta chi) w0 = 0; on these four spinors alpha^3 = Pc, and its trace vanishes: two zero modes have alpha^3 = +1 (energy +k_3) and two alpha^3 = -1 (energy -k_3)".into(), eq_plus && a3_plus && split_plus));
    let pn = v.projector(-1, 1);
    let eq_minus = a1.times_i().add(beta).mul(&pn).is_zero() && a2.times_i().sub(b5).mul(&pn).is_zero();
    let a3_minus = a3.add(&v.pc).mul(&pn).is_zero();
    out.push(("winding -1: the spinors w0 with (i beta alpha^1) w0 = -w0, (alpha^2 beta chi) w0 = +w0 solve (i alpha^1 + beta) w0 = 0, (i alpha^2 - i beta chi) w0 = 0, and on them alpha^3 = -Pc: the directions of motion are exchanged".into(), eq_minus && a3_minus));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn r(n: i128, d: i128) -> Rat {
        Rat::new(n, d)
    }

    #[test]
    fn only_rotations_of_coordinates_with_equal_rates_survive() {
        let b = [r(1, 2), r(1, 3), r(1, 5)];
        for sig in [Sig::S1644, Sig::S1671] {
            // three different rates: the six rotations of (x0, x1, x2, x3)
            let s = surviving_rotations(sig, [1, 2, 3], b);
            assert_eq!(s, vec![(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]);
            // two equal rates: one more, the rotation of (x5, x6)
            let s = surviving_rotations(sig, [2, 2, 5], b);
            assert_eq!(s.len(), 7);
            assert!(s.contains(&(5, 6)));
            // three equal rates: the three rotations of (x5, x6, x7)
            let s = surviving_rotations(sig, [1, 1, 1], b);
            assert_eq!(s.len(), 9);
            assert!(s.contains(&(5, 6)) && s.contains(&(5, 7)) && s.contains(&(6, 7)));
            // no rotation that involves x4, and none that mixes x0 with a deflating coordinate
            assert!(s.iter().all(|(a, bb)| *a != 4 && *bb != 4 && !(*a == 0 && *bb >= 5)));
        }
    }

    #[test]
    fn a_vortex_line_carries_four_zero_modes_two_in_each_direction() {
        let gm = Gammas::build();
        for sig in [Sig::S1644, Sig::S1671] {
            for (statement, ok) in vortex_checks(&gm, sig) {
                assert!(ok, "{}: {}", sig.name(), statement);
            }
        }
    }
}
