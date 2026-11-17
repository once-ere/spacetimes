//! Gauss-Legendre quadrature: the n nodes x_i in (-1, 1) and weights w_i for which sum_i w_i f(x_i) equals the
//! integral of f over (-1, 1) exactly for every polynomial f of degree at most 2n - 1.
//!
//! The nodes are the zeros of the Legendre polynomial P_n, found by Newton's method from the starting values
//! cos(pi (i + 3/4)/(n + 1/2)); P_n and its derivative come from the three-term recursion
//! (k + 1) P_{k+1}(x) = (2k + 1) x P_k(x) - k P_{k-1}(x); the weights are 2 / ((1 - x^2) P_n'(x)^2).
//!
//! Composite rules: an interval is cut into panels (with chosen break points, for example at a Fermi momentum where
//! an integrand has a sharp peak) and each panel gets the n-point rule.
use std::f64::consts::PI;

/// nodes and weights of the n-point rule on (-1, 1)
pub fn gauss_legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut x = vec![0.0; n];
    let mut w = vec![0.0; n];
    for i in 0..n {
        let mut z = (PI * (i as f64 + 0.75) / (n as f64 + 0.5)).cos();
        for _ in 0..100 {
            let (mut p0, mut p1) = (1.0, z);
            for k in 1..n {
                let p2 = ((2 * k + 1) as f64 * z * p1 - k as f64 * p0) / (k + 1) as f64;
                p0 = p1;
                p1 = p2;
            }
            // p1 = P_n(z), p0 = P_{n-1}(z); P_n'(z) = n (z P_n - P_{n-1}) / (z^2 - 1)
            let dp = n as f64 * (z * p1 - p0) / (z * z - 1.0);
            let dz = p1 / dp;
            z -= dz;
            if dz.abs() < 1e-16 {
                let dpf = {
                    let (mut q0, mut q1) = (1.0, z);
                    for k in 1..n {
                        let q2 = ((2 * k + 1) as f64 * z * q1 - k as f64 * q0) / (k + 1) as f64;
                        q0 = q1;
                        q1 = q2;
                    }
                    n as f64 * (z * q1 - q0) / (z * z - 1.0)
                };
                x[i] = z;
                w[i] = 2.0 / ((1.0 - z * z) * dpf * dpf);
                break;
            }
        }
    }
    (x, w)
}

/// A composite rule on [a, b]: the break points `cuts` (inside (a, b), any order) divide the interval, each piece is
/// divided into `panels` equal panels, and each panel gets the n-point Gauss-Legendre rule.  Returns (nodes, weights).
pub fn composite(a: f64, b: f64, cuts: &[f64], panels: usize, n: usize) -> (Vec<f64>, Vec<f64>) {
    let (gx, gw) = gauss_legendre(n);
    let mut pts = vec![a];
    let mut cs: Vec<f64> = cuts.iter().cloned().filter(|&c| c > a && c < b).collect();
    cs.sort_by(|x, y| x.partial_cmp(y).unwrap());
    pts.extend(cs);
    pts.push(b);
    let (mut xs, mut ws) = (vec![], vec![]);
    for seg in pts.windows(2) {
        let h = (seg[1] - seg[0]) / panels as f64;
        for p in 0..panels {
            let lo = seg[0] + p as f64 * h;
            for (xi, wi) in gx.iter().zip(&gw) {
                xs.push(lo + 0.5 * h * (xi + 1.0));
                ws.push(0.5 * h * wi);
            }
        }
    }
    (xs, ws)
}
