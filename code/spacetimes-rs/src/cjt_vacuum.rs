//! The CJT gap equation in the VACUUM: can the pair condensate form without a medium?
//! Derivation: supplement S4, Sec. 7.
//!
//! THE QUESTION.  Supplement S2 asks whether the pair condensate could be the Higgs
//! field; a necessary condition is that it forms in empty space.  Without a Fermi
//! surface there is no Cooper instability, and the negative-energy states cannot be
//! neglected.  The stationarity condition of the CJT action for the fermion
//! propagator is then the Schwinger-Dyson equation in four-dimensional (Euclidean)
//! momentum space.  With Gamma_2 = one exchanged boson ("rainbow" or "ladder"
//! approximation, the one used in the paper of Cornwall, Jackiw and Tomboulis for
//! dynamical symmetry breaking) it reads, for the self-energy sigma(p) that the
//! condensate of the vector pair channel generates,
//!
//!   2 sigma(p) = g^2 Int d^4k/(2 pi)^4 [ f(m + sigma) - f(m - sigma) ] [ D_s(p - k) + D_th(p - k) ],
//!   f(M) = M/(k^2 + M^2),   D_s = 1/(q^2 + m_s^2),   D_th = 1/q^2 .
//!
//! (In the zero-mode sector the 16-component field is equivalent to four ordinary
//! Dirac fields, two of each sign of the central generator P of symmetry.rs; the pair
//! condensate splits the masses of two combinations of them into m + sigma and
//! m - sigma.  Both modes of the Vorton-Scalar act with the same sign in this channel,
//! as in the medium.)  Linearized in sigma, and after the angular integration in four
//! dimensions,
//!
//!   sigma(p) = (g^2/(8 pi^2)) Int_0^Lambda dk k^3 (k^2 - m^2)/(k^2 + m^2)^2 [A(p, k; m_s) + A(p, k; 0)] sigma(k),
//!   A(p, k; M) = 2 / [ p^2 + k^2 + M^2 + sqrt((p^2 + k^2 + M^2)^2 - 4 p^2 k^2) ] ,
//!
//! with a cutoff Lambda where the four-dimensional description ends.  A condensate
//! forms if the largest eigenvalue of the kernel exceeds 1.  This module computes the
//! critical coupling alpha_c = g_c^2/(4 pi) as a function of Lambda/m and m_s/m.
//!
//! APPROXIMATIONS (stated in S4): one-boson exchange with bare vertices; no
//! wave-function renormalization; linearization (the threshold only).  For two
//! massless exchanged bosons and Lambda -> infinity the equation is scale invariant
//! and has the critical coupling alpha_c = pi/2 exactly; the unit test checks the
//! program against the closed solution with cutoffs.

use crate::quad::{gauss_legendre, log_grid, Grid};

const PI: f64 = std::f64::consts::PI;

/// Average of 1/((p - k)^2 + M^2) over the directions of k in four Euclidean dimensions.
pub fn avg4(p2: f64, k2: f64, m2: f64) -> f64 {
    let s = p2 + k2 + m2;
    2.0 / (s + (s * s - 4.0 * p2 * k2).max(0.0).sqrt())
}

/// Largest eigenvalue of the linearized kernel for g = 1.
/// `m_f`: fermion mass (0 for the scale-invariant test case); `bosons`: masses of the
/// exchanged bosons; momenta from `ir` to `cutoff`.  Returns (eigenvalue, relative
/// residual |A v - lambda v|/|v| of the power iteration).
pub fn ladder_eigenvalue(m_f: f64, bosons: &[f64], ir: f64, cutoff: f64) -> (f64, f64) {
    let decades = (cutoff / ir).log10();
    let grid: Grid = {
        let mut g = Grid::new();
        if m_f > 0.0 {
            // the region below `ir` (negligible weight k^3)
            g.push_panel(0.0, ir, &gauss_legendre(6));
        }
        let lg = log_grid(ir, cutoff, (60.0 * decades).ceil() as usize, 4);
        g.x.extend_from_slice(&lg.x);
        g.w.extend_from_slice(&lg.w);
        g
    };
    let n = grid.len();
    let mut a = vec![0.0; n * n];
    for j in 0..n {
        let k = grid.x[j];
        let k2 = k * k;
        let h = (k2 - m_f * m_f) / (k2 + m_f * m_f).powi(2);
        let wt = grid.w[j] * k * k2 * h / (8.0 * PI * PI);
        for i in 0..n {
            let p2 = grid.x[i] * grid.x[i];
            a[i * n + j] = wt * bosons.iter().map(|mb| avg4(p2, k2, mb * mb)).sum::<f64>();
        }
    }
    // power iteration; the Rayleigh quotient keeps the sign of the eigenvalue
    let mut v = vec![1.0; n];
    let mut lam = 0.0;
    let mut resid = f64::MAX;
    for _ in 0..20000 {
        let av: Vec<f64> = (0..n).map(|i| (0..n).map(|j| a[i * n + j] * v[j]).sum()).collect();
        let vv: f64 = v.iter().map(|x| x * x).sum();
        let new = av.iter().zip(&v).map(|(x, y)| x * y).sum::<f64>() / vv;
        resid = (av.iter().zip(&v).map(|(x, y)| (x - new * y).powi(2)).sum::<f64>() / vv).sqrt() / new.abs();
        let nrm = av.iter().map(|x| x * x).sum::<f64>().sqrt();
        v = av.iter().map(|x| x / nrm).collect();
        let done = (new - lam).abs() < 1e-13 * new.abs() && resid < 1e-9;
        lam = new;
        if done {
            break;
        }
    }
    (lam, resid)
}

/// Critical coupling alpha_c = g_c^2/(4 pi) of the vacuum pair condensate for fermion
/// mass 1, radial-mode mass `m_s` and cutoff `cutoff` (both in units of the fermion
/// mass), and the residual of the eigenvalue calculation.
pub fn critical_alpha(m_s: f64, cutoff: f64) -> (f64, f64) {
    let (lam, resid) = ladder_eigenvalue(1.0, &[m_s, 0.0], 1e-3, cutoff);
    (1.0 / (4.0 * PI * lam), resid)
}

/// The scale-invariant test problem: massless fermion, ONE massless boson, momenta
/// between mu and Lambda.  With x = p^2 and hat-lambda = g^2/(16 pi^2) the equation is
///   sigma(x) = hat-lambda [ (1/x) Int_a^x sigma dy + Int_x^b sigma dy/y ],
/// equivalent to (x^2 sigma')' = -hat-lambda sigma with sigma'(a) = 0, (x sigma)'(b) = 0.
/// Its solutions are x^(-1/2) cos(rho ln(x/a) + delta) with hat-lambda = 1/4 + rho^2,
/// and the boundary conditions give  rho L = arctan(1/(2 rho)),  L = ln(Lambda/mu),
/// for the smallest critical coupling.  Returns that hat-lambda.
pub fn analytic_critical_lambda_hat(l: f64) -> f64 {
    // fixed-point iteration rho = arctan(1/(2 rho))/L converges for L > 1
    let mut rho = PI / (2.0 * (l + 2.0));
    for _ in 0..200 {
        rho = (1.0 / (2.0 * rho)).atan() / l;
    }
    0.25 + rho * rho
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The angular average against direct quadrature over the polar angle in four
    /// dimensions (measure sin^2(theta) dtheta, normalized).
    #[test]
    fn four_dimensional_angular_average() {
        let gl = gauss_legendre(64);
        for &(p, k, m) in &[(1.0f64, 0.3f64, 0.5f64), (0.2, 2.0, 0.0), (1.5, 1.4, 0.1)] {
            let mut s = 0.0;
            for (t, w) in gl.0.iter().zip(&gl.1) {
                let th = 0.5 * PI * (t + 1.0);
                s += 0.5 * PI * w * th.sin().powi(2) / (p * p + k * k - 2.0 * p * k * th.cos() + m * m);
            }
            s *= 2.0 / PI;
            assert!((avg4(p * p, k * k, m * m) - s).abs() < 1e-9 * s, "{} vs {}", avg4(p * p, k * k, m * m), s);
        }
    }

    /// The program reproduces the closed solution of the scale-invariant problem.
    #[test]
    fn scale_invariant_ladder_equation() {
        for &(mu, lam) in &[(1.0, 1e3), (1.0, 1e5)] {
            let (ev, resid) = ladder_eigenvalue(0.0, &[0.0], mu, lam);
            assert!(resid < 1e-8);
            // eigenvalue for g = 1 is 1/(16 pi^2 hat-lambda_c)
            let numeric = 1.0 / (16.0 * PI * PI * ev);
            let exact = analytic_critical_lambda_hat((lam / mu as f64).ln());
            assert!((numeric / exact - 1.0).abs() < 2e-3, "hat-lambda_c = {} (program) vs {} (closed form)", numeric, exact);
        }
    }

    /// With the fermion mass and both modes, the critical coupling decreases
    /// towards pi/2 as the cutoff grows, and stays above it.
    #[test]
    fn critical_coupling_approaches_pi_over_two() {
        let (a3, r3) = critical_alpha(1.0, 1e3);
        let (a6, r6) = critical_alpha(1.0, 1e6);
        assert!(r3 < 1e-6 && r6 < 1e-6);
        assert!(a3 > a6 && a6 > 0.5 * PI, "alpha_c = {} (cutoff 1e3), {} (cutoff 1e6)", a3, a6);
        assert!(a6 < 0.5 * PI * 1.25);
    }
}
