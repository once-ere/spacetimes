//! Quadrature on the radial momentum q (0 <= q <= 1 in units of the cutoff).
//!
//! * `adaptive`: Gauss-Legendre rules of 20 and 40 points are compared on an
//!   interval; if they differ by more than the tolerance, the interval is halved
//!   (recursively).  The difference of the two rules is a reliable error estimate
//!   because the 40-point rule is far more accurate than the 20-point rule.
//! * `breaks_around`: break points that cluster geometrically around a point q_F
//!   (the Fermi momentum), where the integrands of a paired state vary on the
//!   scale of the gap; integrating panel by panel between such break points
//!   resolves that scale whatever its size.
//! * `nodes`: a fixed composite Gauss-Legendre grid (nodes and weights) on given
//!   panels, for the integrals of the collective modes, whose integrands are
//!   expensive (one 32x32 diagonalisation per node).

use spacetimes::quad::gauss_legendre;
use std::sync::OnceLock;

static GL20: OnceLock<(Vec<f64>, Vec<f64>)> = OnceLock::new();
static GL40: OnceLock<(Vec<f64>, Vec<f64>)> = OnceLock::new();

fn rule_on<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64, r: &(Vec<f64>, Vec<f64>)) -> f64 {
    let h = 0.5 * (b - a);
    let c = 0.5 * (b + a);
    r.0.iter().zip(&r.1).map(|(x, w)| w * f(c + h * x)).sum::<f64>() * h
}

fn adapt<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64, abs_tol: f64, rel_tol: f64, depth: usize) -> f64 {
    let g20 = GL20.get_or_init(|| gauss_legendre(20));
    let g40 = GL40.get_or_init(|| gauss_legendre(40));
    let i1 = rule_on(f, a, b, g20);
    let i2 = rule_on(f, a, b, g40);
    // stop when the two rules agree, when the depth limit is reached, or when the
    // interval is so short that rounding of its end points dominates (further halving
    // would only chase rounding noise)
    let tiny = (b - a).abs() <= 1e-12 * a.abs().max(b.abs()).max(1e-300);
    if (i2 - i1).abs() <= abs_tol.max(rel_tol * i2.abs()) || depth >= 40 || tiny {
        return i2;
    }
    let m = 0.5 * (a + b);
    adapt(f, a, m, 0.5 * abs_tol, rel_tol, depth + 1) + adapt(f, m, b, 0.5 * abs_tol, rel_tol, depth + 1)
}

/// Adaptive integral of f over [a, b].
pub fn adaptive<F: Fn(f64) -> f64>(f: &F, a: f64, b: f64, abs_tol: f64) -> f64 {
    if b <= a {
        return 0.0;
    }
    adapt(f, a, b, abs_tol, 1e-14, 0)
}

/// Adaptive integral of f over the panels between consecutive break points.
#[allow(dead_code)]
pub fn over_breaks<F: Fn(f64) -> f64>(f: &F, breaks: &[f64], abs_tol: f64) -> f64 {
    breaks.windows(2).map(|w| adaptive(f, w[0], w[1], abs_tol)).sum()
}

/// Break points in [lo, hi]: lo, hi and, if qf lies strictly inside, qf and the
/// points qf -/+ width * 3^k (k = 0, 1, 2, ...) that lie inside.
pub fn breaks_around(lo: f64, hi: f64, qf: Option<f64>, width: f64) -> Vec<f64> {
    let mut b = vec![lo, hi];
    if let Some(q0) = qf {
        if q0 > lo && q0 < hi {
            b.push(q0);
            let w0 = width.max(1e-14 * (hi - lo));
            let mut d = w0;
            while q0 - d > lo || q0 + d < hi {
                if q0 - d > lo {
                    b.push(q0 - d);
                }
                if q0 + d < hi {
                    b.push(q0 + d);
                }
                d *= 3.0;
            }
        }
    }
    b.sort_by(|x, y| x.partial_cmp(y).unwrap());
    b.dedup_by(|x, y| (*x - *y).abs() < 1e-300);
    b
}

/// A composite Gauss-Legendre grid: nodes and weights.
#[derive(Clone, Debug)]
pub struct Nodes {
    pub x: Vec<f64>,
    pub w: Vec<f64>,
}

/// Gauss-Legendre rule of `order` points on every panel between consecutive break points.
pub fn nodes(breaks: &[f64], order: usize) -> Nodes {
    let (gx, gw) = gauss_legendre(order);
    let mut x = Vec::new();
    let mut w = Vec::new();
    for p in breaks.windows(2) {
        let h = 0.5 * (p[1] - p[0]);
        let c = 0.5 * (p[1] + p[0]);
        for (xi, wi) in gx.iter().zip(&gw) {
            x.push(c + h * xi);
            w.push(h * wi);
        }
    }
    Nodes { x, w }
}

/// Uniform break points: lo, lo + (hi-lo)/n, ..., hi.
pub fn uniform(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    (0..=n).map(|k| lo + (hi - lo) * k as f64 / n as f64).collect()
}
