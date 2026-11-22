//! The accuracy of the Landau form (12.2) of the effective potentials (Section 12.1 of superfluid.md: the paragraph
//! "The accuracy of the Landau form", Table 12.2 and Figure 9).
//!
//! WHAT IS MEASURED.  The polar curve (Phi, V^X(Phi)) of a method X is computed point by point by effpot::curve, each
//! point a self-consistent state (accurate to about 1e-13).  Along the polar fields the Landau form (12.2) is
//! V_L(Phi) = a Phi^2 + b Phi^4 + c Phi^6; its three coefficients are a least-squares fit to the computed points with
//! 0 <= Phi <= 1.4 Phi0 (effpot::landau_fit), not a derivation.  This module measures what the fit gets right and wrong:
//!   * the minimum Phi_L, the depth V_L(Phi_L) and the curvature V_L''(Phi_L) of the fitted polynomial, against Phi0,
//!     the ground-state energy E0 and the curvature V''_amp of the computed curve (V''_amp fixes the mass M^H of the
//!     amplitude mode by (13.6), so sqrt(V_L''/V''_amp) - 1 is the relative error of M^H computed from (12.2));
//!   * the largest residual max_i |V_K(Phi_i) - V^X(Phi_i)| of the least-squares fits V_K = sum_{k=1..K} c_k Phi^(2k)
//!     on the same points, K = 2 (the quartic form of the Standard Model) to K = 6; K = 3 is (12.2).  The residual falls
//!     fast with K: the residual of (12.2) is the truncation of a polynomial series, not numerical noise;
//!   * the coefficients of (12.2) fitted on three intervals: [0, Phi0], [0, 1.4 Phi0] and the whole computed curve;
//!     they change with the interval, so they are properties of the fit, not of the theory;
//!   * the number of sign changes of the residual of (12.2) along [0, 1.4 Phi0] (the oscillation of a polynomial
//!     approximation of a function that is not a polynomial);
//!   * the coefficient a^Tay of Phi^2 of the Taylor series of V^X at the origin, computed directly from three states
//!     with small applied fields: f = V/Phi^2 = a^Tay + (coefficient) Phi^2 + O(Phi^4), and the Richardson elimination of
//!     the Phi^2 term from two of the three points (two pairs, whose agreement is check P3-10).
//! THE CAUSE (checks P3-8, P3-9).  The pair field enters the quasi-particle energies like a mass, (10.3): for the applied
//! pair self-energy Phi_a (K^3)^dag and the normal self-energy (M - m) beta (x) 1_2 the eigenvalues of the BdG matrix are
//! +-(|q|^2 + M^2 + Phi_a^2)^(1/2), eight times each (P3-8), and along the polar curve the normal self-energy is such a
//! Dirac mass (P3-9).  The momentum integrals of these energies contain s^2 ln s with s = M^2 + Phi_a^2, which is not
//! analytic at Phi_a^2 = -M^2: V^X is not a polynomial, and its Taylor series at the origin converges only in a disk of
//! a radius of the order of the Dirac mass (about 0.2), smaller than Phi0 (0.31 to 0.40).
//! THE LEAST SQUARES.  y = sum_{k=1..K} d_k t^k with the scaled variable t = (Phi/Phi_max)^2 (Phi_max the largest fitted
//! Phi), by Householder reflections of the m x K design matrix (no normal equations: the fits with K = 5, 6 are
//! ill-conditioned); then c_k = d_k / Phi_max^(2k).
use crate::effpot::{point, Curve};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{beta, d8, h_free, k_scalar};
use sfcore::methods::M_LL;
use sfcore::report::Report;
use sfcore::rng::Rng;
use sfcore::uniform::Model;

/// the results for one method (all energies per volume, in the units of the project)
pub struct Accuracy {
    /// the minimum Phi_L of the fitted polynomial (12.2), its value V_L(Phi_L) and its curvature V_L''(Phi_L)
    pub phi_fit: f64,
    pub depth_fit: f64,
    pub curv_fit: f64,
    /// the number of sign changes of the residual of (12.2) on [0, 1.4 Phi0]
    pub sign_changes: usize,
    /// the largest residual of the fits with the powers Phi^2 .. Phi^(2K), K = 2 .. 6, on [0, 1.4 Phi0]
    pub res_by_k: Vec<(usize, f64)>,
    /// the coefficient c of Phi^6 of the fit with K = 4 (powers to Phi^8) on [0, 1.4 Phi0]
    pub c_k4: f64,
    /// (the upper end of the interval relative to Phi0, a, b, c, the largest residual) of the fits of (12.2) on
    /// [0, Phi0], [0, 1.4 Phi0] and the whole computed curve
    pub by_range: Vec<(f64, f64, f64, f64, f64)>,
    /// the coefficient of Phi^2 of the Taylor series at the origin (Richardson from the two smaller fields) and the same
    /// from the two larger fields (check P3-10)
    pub a_taylor: f64,
    pub a_taylor_check: f64,
    /// the residuals on [0, 1.4 Phi0] for Figure 9: (Phi, residual of K = 2, of K = 3 = (12.2), of K = 4)
    pub residuals: Vec<(f64, f64, f64, f64)>,
}

/// least squares min |A x - y| for a real m x n matrix A (m >= n, rank n) by Householder reflections; A and y are
/// overwritten (A by R in its upper triangle, y by Q^T y)
fn householder_lsq(a: &mut [Vec<f64>], y: &mut [f64]) -> Vec<f64> {
    let m = a.len();
    let n = a[0].len();
    for k in 0..n {
        // the reflection I - 2 v v^T / (v^T v) that maps the column k (rows k..m) onto alpha e_k
        let norm = (k..m).map(|i| a[i][k] * a[i][k]).sum::<f64>().sqrt();
        let alpha = if a[k][k] > 0.0 { -norm } else { norm };
        let mut v: Vec<f64> = (k..m).map(|i| a[i][k]).collect();
        v[0] -= alpha;
        let vv: f64 = v.iter().map(|x| x * x).sum();
        if vv == 0.0 {
            continue;
        }
        for j in k..n {
            let s = 2.0 * (k..m).map(|i| v[i - k] * a[i][j]).sum::<f64>() / vv;
            for i in k..m {
                a[i][j] -= s * v[i - k];
            }
        }
        let s = 2.0 * (k..m).map(|i| v[i - k] * y[i]).sum::<f64>() / vv;
        for i in k..m {
            y[i] -= s * v[i - k];
        }
    }
    // back substitution R x = (Q^T y)_{0..n}
    let mut x = vec![0.0; n];
    for k in (0..n).rev() {
        let mut s = y[k];
        for j in (k + 1)..n {
            s -= a[k][j] * x[j];
        }
        x[k] = s / a[k][k];
    }
    x
}

/// the value of sum_k c_k Phi^(2k) (k = 1, 2, ...)
pub fn eval_even(c: &[f64], x: f64) -> f64 {
    c.iter().enumerate().map(|(k, ck)| ck * x.powi(2 * (k as i32 + 1))).sum()
}

/// the least-squares fit of y = sum_{k=1..kmax} c_k Phi^(2k) to the points (Phi, y): the coefficients and the largest
/// residual
pub fn even_fit(pts: &[(f64, f64)], kmax: usize) -> (Vec<f64>, f64) {
    let pmax = pts.iter().fold(0.0f64, |s, p| s.max(p.0));
    let mut a: Vec<Vec<f64>> = pts
        .iter()
        .map(|&(x, _)| {
            let t = (x / pmax) * (x / pmax);
            (1..=kmax).map(|k| t.powi(k as i32)).collect()
        })
        .collect();
    let mut y: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let d = householder_lsq(&mut a, &mut y);
    let c: Vec<f64> = (0..kmax).map(|k| d[k] / pmax.powi(2 * (k as i32 + 1))).collect();
    let res = pts.iter().fold(0.0f64, |s, &(x, v)| s.max((eval_even(&c, x) - v).abs()));
    (c, res)
}

/// the stationary point Phi_L > 0 of a Phi^2 + b Phi^4 + c Phi^6 (with s = Phi^2: a + 2 b s + 3 c s^2 = 0, the positive
/// root), the value there and the second derivative d^2/dPhi^2 = 2 a + 12 b Phi^2 + 30 c Phi^4 there
pub fn landau_minimum(a: f64, b: f64, c: f64) -> (f64, f64, f64) {
    let s = if c.abs() < 1e-300 { -a / (2.0 * b) } else { (-2.0 * b + (4.0 * b * b - 12.0 * a * c).sqrt()) / (6.0 * c) };
    (s.sqrt(), a * s + b * s * s + c * s * s * s, 2.0 * a + 12.0 * b * s + 30.0 * c * s * s)
}

/// check P3-8 (once): the quasi-particle energies (10.3) for an applied pair field and a Dirac mass: the eigenvalues of
/// the BdG matrix with N = (M - m) beta (x) 1_2 and P = Phi_a (K^3)^dag are -E (eight times) and +E (eight times),
/// E = (|q|^2 + M^2 + Phi_a^2)^(1/2), at 20 random momenta, masses and fields
pub fn check_qp_energies(rep: &mut Report) {
    let mut rng = Rng(20261121);
    let b8 = d8(&beta());
    let k3d = k_scalar(3).dagger();
    let mut worst = 0.0f64;
    for _ in 0..20 {
        let q = [rng.unit(), rng.unit(), rng.unit()];
        let mass = 0.1 + 0.2 * (rng.unit() + 1.0);
        let fa = 0.3 * (rng.unit() + 1.0);
        let n = b8.scale_r(mass - M_LL);
        let a = h_free(&q, M_LL).add(&n);
        let d = h_free(&[-q[0], -q[1], -q[2]], M_LL).add(&n).transpose().scale_r(-1.0);
        let p = k3d.scale_r(fa);
        let (ev, _) = eigh(&CMat::blocks(&a, &p, &p.dagger(), &d));
        let e = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + mass * mass + fa * fa).sqrt();
        for (k, x) in ev.iter().enumerate() {
            worst = worst.max((x - if k < 8 { -e } else { e }).abs());
        }
    }
    rep.add("P3-8", "the pair field enters the quasi-particle energies like a mass, (10.3): the BdG matrix with N = (M - m) beta (x) 1_2 and P = Phi_a (K^3)^dag has the eigenvalues -+(q^2 + M^2 + Phi_a^2)^(1/2), eight times each (20 random q, M, Phi_a)", worst, 1e-13);
}

/// the accuracy of the Landau form for one method; the checks P3-9 (the normal self-energy along the curve is a Dirac
/// mass), P3-10 (the Richardson value of the Taylor coefficient is converged) and P3-11 (the fit of (12.2) by
/// Householder reflections equals that of the normal equations of effpot::landau_fit)
pub fn accuracy(rep: &mut Report, tag: &str, model: &Model, cv: &Curve, phi0: f64) -> Accuracy {
    let (a, b, c, _) = cv.landau;
    // P3-9: along the polar curve the normal self-energy of every state is a Dirac mass (a multiple of beta (x) 1_2)
    rep.add(&format!("P3-9 {}", tag), &format!("{}: along the polar curve the normal self-energy of every state is a Dirac mass, so the quasi-particle energies are (10.3) (largest deviation from a multiple of beta (x) 1_2)", tag), cv.ndev, 1e-12);
    // the fitted points of (12.2)
    let pts: Vec<(f64, f64)> = cv.rows.iter().filter(|r| r.1 <= 1.4 * phi0).map(|r| (r.1, r.2)).collect();
    let (phi_fit, depth_fit, curv_fit) = landau_minimum(a, b, c);
    // the residual of (12.2) and its sign changes (points where the residual is below 1e-3 of the largest are skipped)
    let r6: Vec<f64> = pts.iter().map(|&(x, v)| a * x * x + b * x.powi(4) + c * x.powi(6) - v).collect();
    let rmax = r6.iter().fold(0.0f64, |s, r| s.max(r.abs()));
    let signs: Vec<f64> = r6.iter().filter(|r| r.abs() > 1e-3 * rmax).map(|r| r.signum()).collect();
    let sign_changes = signs.windows(2).filter(|w| w[0] != w[1]).count();
    // the fits with K = 2 .. 6 on the same points
    let mut res_by_k = vec![];
    let mut fits = vec![];
    for k in 2..=6 {
        let (cf, res) = even_fit(&pts, k);
        res_by_k.push((k, res));
        fits.push(cf);
    }
    let dqr = (fits[1][0] - a).abs() / a.abs() + (fits[1][1] - b).abs() / b.abs() + (fits[1][2] - c).abs() / c.abs();
    rep.add(&format!("P3-11 {}", tag), &format!("{}: the least squares of (12.2) by Householder reflections give the coefficients of the normal equations (sum of the relative differences of a, b, c)", tag), dqr, 1e-9);
    let c_k4 = fits[2][2];
    // the residuals for Figure 9
    let residuals: Vec<(f64, f64, f64, f64)> = pts
        .iter()
        .map(|&(x, v)| (x, eval_even(&fits[0], x) - v, eval_even(&fits[1], x) - v, eval_even(&fits[2], x) - v))
        .collect();
    // (12.2) on three intervals
    let pmax_all = cv.rows.iter().fold(0.0f64, |s, r| s.max(r.1));
    let mut by_range = vec![];
    for top in [1.0, 1.4, pmax_all / phi0] {
        let p: Vec<(f64, f64)> = cv.rows.iter().filter(|r| r.1 <= top * phi0 * (1.0 + 1e-12)).map(|r| (r.1, r.2)).collect();
        let (cf, res) = even_fit(&p, 3);
        by_range.push((top, cf[0], cf[1], cf[2], res));
    }
    // the Taylor coefficient of Phi^2 at the origin: V/Phi^2 at the applied fields 0.004, 0.008, 0.016 (start: the
    // normal vacuum) and the Richardson elimination of the Phi^2 term
    let k3d = k_scalar(3).dagger();
    let mut f = vec![];
    for fa in [0.004, 0.008, 0.016] {
        let (ph, e, _, _) = point(model, &k3d.scale_r(fa), &CMat::zeros(8));
        let x2 = ph[2].re * ph[2].re;
        f.push((x2, e / x2));
    }
    let rich = |i: usize, j: usize| (f[i].1 * f[j].0 - f[j].1 * f[i].0) / (f[j].0 - f[i].0);
    let (a_taylor, a_taylor_check) = (rich(0, 1), rich(1, 2));
    rep.add(&format!("P3-10 {}", tag), &format!("{}: the coefficient of Phi^2 of the Taylor series at the origin, a^Tay = {:.10e}, by Richardson from the fields 0.004, 0.008 and from 0.008, 0.016 (relative difference; the fit (12.2) has a = {:.10e})", tag, a_taylor, a), (a_taylor - a_taylor_check).abs() / a_taylor.abs(), 1e-6);
    Accuracy { phi_fit, depth_fit, curv_fit, sign_changes, res_by_k, c_k4, by_range, a_taylor, a_taylor_check, residuals }
}
