// Provenance: this module is taken from the author's NCSS project (github.com/once-ere/NCSS,
// Revision_1/code/ncss-rs/src/quad.rs), where it is validated by that project's test suite.
// Changes for PROJECT spacetimes, if any, are marked "[spacetimes]".
//
//! Gauss-Legendre quadrature and composite quadrature grids.
//!
//! The nodes/weights of the n-point Gauss-Legendre rule on [-1, 1] are found
//! by Newton iteration on the Legendre polynomial P_n, starting from the
//! Tricomi asymptotic estimate cos(pi (i - 1/4) / (n + 1/2)).  The rule is
//! exact for polynomials of degree <= 2n - 1 (tested below).

/// n-point Gauss-Legendre nodes and weights on [-1, 1].
pub fn gauss_legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    assert!(n >= 1);
    let mut x = vec![0.0; n];
    let mut w = vec![0.0; n];
    let m = (n + 1) / 2;
    for i in 0..m {
        // initial guess for the i-th root (largest first)
        let mut z = (std::f64::consts::PI * (i as f64 + 0.75) / (n as f64 + 0.5)).cos();
        let mut pp = 0.0;
        for _ in 0..100 {
            // recurrence for P_n(z) and P_{n-1}(z)
            let mut p1 = 1.0;
            let mut p2 = 0.0;
            for j in 0..n {
                let p3 = p2;
                p2 = p1;
                p1 = ((2 * j + 1) as f64 * z * p2 - j as f64 * p3) / (j + 1) as f64;
            }
            // derivative P_n'(z)
            pp = n as f64 * (z * p1 - p2) / (z * z - 1.0);
            let z1 = z;
            z = z1 - p1 / pp;
            if (z - z1).abs() < 1e-15 {
                break;
            }
        }
        x[i] = -z;
        x[n - 1 - i] = z;
        w[i] = 2.0 / ((1.0 - z * z) * pp * pp);
        w[n - 1 - i] = w[i];
    }
    (x, w)
}

/// A one-dimensional quadrature grid: nodes and weights such that
/// integral f(x) dx ~= sum_i w_i f(x_i).
#[derive(Clone, Debug, Default)]
pub struct Grid {
    pub x: Vec<f64>,
    pub w: Vec<f64>,
}

impl Grid {
    /// Empty grid.
    pub fn new() -> Self {
        Grid { x: Vec::new(), w: Vec::new() }
    }
    /// Number of nodes.
    pub fn len(&self) -> usize {
        self.x.len()
    }
    /// True if the grid has no nodes.
    pub fn is_empty(&self) -> bool {
        self.x.is_empty()
    }
    /// Append an `n`-point Gauss-Legendre panel on [a, b].
    pub fn push_panel(&mut self, a: f64, b: f64, gl: &(Vec<f64>, Vec<f64>)) {
        let h = 0.5 * (b - a);
        let c = 0.5 * (b + a);
        for (t, wt) in gl.0.iter().zip(gl.1.iter()) {
            self.x.push(c + h * t);
            self.w.push(h * wt);
        }
    }
    /// Append a panel in a mapped variable s in [s0, s1] with x = map(s) and
    /// dx/ds = dmap(s): integral f(x) dx = integral f(map(s)) dmap(s) ds.
    pub fn push_mapped_panel<F, G>(&mut self, s0: f64, s1: f64, gl: &(Vec<f64>, Vec<f64>), map: F, dmap: G)
    where
        F: Fn(f64) -> f64,
        G: Fn(f64) -> f64,
    {
        let h = 0.5 * (s1 - s0);
        let c = 0.5 * (s1 + s0);
        for (t, wt) in gl.0.iter().zip(gl.1.iter()) {
            let s = c + h * t;
            self.x.push(map(s));
            self.w.push(h * wt * dmap(s));
        }
    }
    /// Integrate a function sampled on the grid.
    pub fn integrate(&self, f: &[f64]) -> f64 {
        self.w.iter().zip(f.iter()).map(|(w, f)| w * f).sum()
    }
}

/// Composite Gauss-Legendre grid on [a, b] with `panels` equal panels.
pub fn uniform_grid(a: f64, b: f64, panels: usize, order: usize) -> Grid {
    let gl = gauss_legendre(order);
    let mut g = Grid::new();
    let h = (b - a) / panels as f64;
    for p in 0..panels {
        g.push_panel(a + p as f64 * h, a + (p + 1) as f64 * h, &gl);
    }
    g
}

/// Composite grid on [a, b] (0 < a < b) with panels equally spaced in log x.
pub fn log_grid(a: f64, b: f64, panels: usize, order: usize) -> Grid {
    assert!(a > 0.0 && b > a);
    let gl = gauss_legendre(order);
    let mut g = Grid::new();
    let la = a.ln();
    let lb = b.ln();
    let h = (lb - la) / panels as f64;
    for p in 0..panels {
        let s0 = la + p as f64 * h;
        let s1 = s0 + h;
        g.push_mapped_panel(s0, s1, &gl, |s| s.exp(), |s| s.exp());
    }
    g
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gl_exact_for_polynomials() {
        for n in 1..=20 {
            let (x, w) = gauss_legendre(n);
            for deg in 0..(2 * n) {
                let num: f64 = x.iter().zip(w.iter()).map(|(x, w)| w * x.powi(deg as i32)).sum();
                let exact = if deg % 2 == 1 { 0.0 } else { 2.0 / (deg as f64 + 1.0) };
                assert!((num - exact).abs() < 1e-13, "n={n} deg={deg}: {num} vs {exact}");
            }
        }
    }
    #[test]
    fn log_grid_integrates_one_over_x() {
        let g = log_grid(1e-3, 1e3, 12, 8);
        let f: Vec<f64> = g.x.iter().map(|x| 1.0 / x).collect();
        let exact = (1e6_f64).ln();
        assert!((g.integrate(&f) - exact).abs() < 1e-10);
    }
}
