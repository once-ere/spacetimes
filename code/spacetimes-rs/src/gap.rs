//! The gap equation of the hidden-axis singlet superfluid.
//!
//! MODEL (effective theory in ordinary space, both signatures; see supplement S1).
//! A degenerate gas of the fermions of the 16-component field (mass m, eight internal
//! states, Fermi momentum k_F) interacts through the exchange of the two modes of the
//! Vorton-Scalar field: the radial mode (mass m_s, scalar vertex) and the phase mode
//! (mass m_ph, zero for an exact symmetry; pseudoscalar vertex).  Both couple with
//! the strength g = m / v_VS.  In the hidden-axis pair channel, the pair operator
//! DeltaV^0 = dirac^T C^- gamma^0 dirac, the pairing interaction between the pairs
//! (k, -k) and (k', -k') is
//!
//!     V(k, k') = - g^2 [ F_S / (q^2 + m_s^2) + F_P / (q^2 + m_ph^2) ],   q = k' - k,
//!
//! with the amplitudes F_S, F_P of `kernel.rs` (verified there against the Dirac
//! matrices).  The state is "unitary": all eight internal states have the same gap.
//!
//! GAP EQUATION (mean field, particles only; valid for gap and temperature << m):
//!
//!     Delta(k) = (1/(4 pi^2)) Int_0^inf dk' k'^2 W(k, k') Delta(k') tanh(E'/(2T)) / E',
//!     E' = sqrt(xi'^2 + Delta(k')^2),   xi' = sqrt(m^2 + k'^2) - mu,
//!     W(k, k') = (1/2) Int_{-1}^{1} dcos(theta) [-V(k, k')]     (s-wave average),
//!
//! with the chemical potential mu = sqrt(m^2 + k_F^2) (weak-coupling value).
//! The critical temperature is the temperature at which the linearized equation
//! (Delta -> 0 in E') has the eigenvalue 1.
//!
//! UNITS: hbar = c = k_B = 1; all momenta, masses, gaps and temperatures in units of
//! the fermion mass m when the functions are called with m = 1.

use crate::quad::{gauss_legendre, Grid};

/// Parameters of the pairing problem.
#[derive(Clone, Copy, Debug)]
pub struct Pairing {
    /// fermion mass
    pub m: f64,
    /// coupling g = m / v_VS of both modes of the Vorton-Scalar
    pub g: f64,
    /// mass of the radial mode
    pub m_s: f64,
    /// mass of the phase mode (0 for an exact U(1) symmetry)
    pub m_ph: f64,
    /// include the exchange of the radial mode
    pub scalar: bool,
    /// include the exchange of the phase mode
    pub phase: bool,
    /// +1 for the vector pair channel (the hidden-axis pair), in which the phase mode
    /// attracts; -1 for a two-form pair channel with its plane in the extra dimensions,
    /// in which the phase mode repels with the same strength (kernel.rs, `channels`)
    pub phase_sign: f64,
    /// Fermi momentum
    pub k_f: f64,
}

/// (1/2) Int_{-1}^{1} dmu (a + b mu) / (c - d mu), with c > d >= 0.
/// For c - d -> 0 the logarithm diverges; the callers guarantee that a + b -> 0
/// there at least as fast as (c - d), so that the product vanishes.
pub fn angular_average(a: f64, b: f64, c: f64, d: f64) -> f64 {
    if d < 1e-5 * c {
        // series in d/c
        return a / c + (a * d * d + b * c * d) / (3.0 * c * c * c);
    }
    let gap = c - d;
    if gap <= 1e-14 * c {
        // forward-scattering limit of a massless exchange: the numerator vanishes
        return 0.5 * (-2.0 * b / d);
    }
    let l = ((c + d) / gap).ln();
    0.5 * (-2.0 * b / d + (a + b * c / d) * l / d)
}

impl Pairing {
    pub fn energy(&self, k: f64) -> f64 {
        (self.m * self.m + k * k).sqrt()
    }

    pub fn mu(&self) -> f64 {
        self.energy(self.k_f)
    }

    /// The s-wave average W(k, k') of the attraction (positive = attractive).
    pub fn w(&self, k: f64, kp: f64) -> f64 {
        let m2 = self.m * self.m;
        let (e, ep) = (self.energy(k), self.energy(kp));
        let den = 4.0 * e * e * ep * ep;
        let d = 2.0 * k * kp;
        let mut w = 0.0;
        if self.scalar {
            // F_S = [m^2 (E+E')^2 + E^2 k'^2 + E'^2 k^2 - 2 E E' k k' mu] / (4 E^2 E'^2)
            let a = m2 * (e + ep) * (e + ep) + e * e * kp * kp + ep * ep * k * k;
            let b = -2.0 * e * ep * k * kp;
            w += angular_average(a, b, k * k + kp * kp + self.m_s * self.m_s, d) / den;
        }
        if self.phase {
            // F_P = [(m^2 - E E')^2 + 2 (m^2 - E E') k k' mu + m^2 (k^2 + k'^2 - 2 k k' mu) + k^2 k'^2] / (4 E^2 E'^2)
            let cc = m2 - e * ep;
            let a = cc * cc + m2 * (k * k + kp * kp) + k * k * kp * kp;
            let b = 2.0 * cc * k * kp - 2.0 * m2 * k * kp; // = -2 E E' k k'
            w += self.phase_sign * angular_average(a, b, k * k + kp * kp + self.m_ph * self.m_ph, d) / den;
        }
        self.g * self.g * w
    }

    /// Density of states at the Fermi surface per pair of internal states that form
    /// Cooper pairs: N(0) = k_F E_F / (2 pi^2).
    pub fn n0(&self) -> f64 {
        self.k_f * self.mu() / (2.0 * std::f64::consts::PI * std::f64::consts::PI)
    }

    /// Dimensionless BCS coupling lambda = N(0) W(k_F, k_F).
    pub fn lambda(&self) -> f64 {
        self.n0() * self.w(self.k_f, self.k_f)
    }
}

/// Momentum grid that resolves the Fermi surface: panels whose edges approach k_F
/// geometrically from both sides (down to 10^-`decades` k_F), plus logarithmic
/// panels up to `k_max`.
pub fn fermi_grid(k_f: f64, k_max: f64, decades: usize, order: usize) -> Grid {
    let gl = gauss_legendre(order);
    let mut g = Grid::new();
    // below the Fermi surface: [0, k_F (1 - 10^0.. )]
    let mut edges_lo: Vec<f64> = vec![0.0];
    let mut j = 0usize;
    while j <= 2 * decades {
        edges_lo.push(k_f * (1.0 - 10f64.powf(-0.5 * (j as f64 + 0.6))));
        j += 1;
    }
    edges_lo.push(k_f);
    for w in edges_lo.windows(2) {
        g.push_panel(w[0], w[1], &gl);
    }
    // above the Fermi surface
    let mut edges_hi: Vec<f64> = vec![k_f];
    let mut j = 2 * decades as i64;
    while j >= 0 {
        edges_hi.push(k_f * (1.0 + 10f64.powf(-0.5 * (j as f64 + 0.6))));
        j -= 1;
    }
    let mut top = *edges_hi.last().unwrap();
    while top < k_max {
        top = (top * 1.6).min(k_max);
        edges_hi.push(top);
    }
    for w in edges_hi.windows(2) {
        g.push_panel(w[0], w[1], &gl);
    }
    g
}

/// Result of a gap calculation.
#[derive(Clone, Debug)]
pub struct GapSolution {
    /// the grid momenta
    pub k: Vec<f64>,
    /// the gap function on the grid
    pub delta: Vec<f64>,
    /// the gap at the Fermi momentum (interpolated from the two nearest nodes)
    pub delta_f: f64,
    /// number of iterations used
    pub iterations: usize,
    pub converged: bool,
}

/// A general (not necessarily exchange-generated) pairing kernel, used for tests.
pub trait Kernel: Sync {
    fn w(&self, k: f64, kp: f64) -> f64;
    fn xi(&self, k: f64) -> f64;
    fn k_f(&self) -> f64;
}

impl Kernel for Pairing {
    fn w(&self, k: f64, kp: f64) -> f64 {
        Pairing::w(self, k, kp)
    }
    fn xi(&self, k: f64) -> f64 {
        self.energy(k) - self.mu()
    }
    fn k_f(&self) -> f64 {
        self.k_f
    }
}

fn kernel_matrix<K: Kernel>(p: &K, grid: &Grid) -> Vec<f64> {
    let n = grid.len();
    let mut km = vec![0.0; n * n];
    let pref = 1.0 / (4.0 * std::f64::consts::PI * std::f64::consts::PI);
    for i in 0..n {
        for j in 0..n {
            km[i * n + j] = pref * grid.w[j] * grid.x[j] * grid.x[j] * p.w(grid.x[i], grid.x[j]);
        }
    }
    km
}

fn value_at_fermi(grid: &Grid, f: &[f64], k_f: f64) -> f64 {
    // the two nodes that bracket k_F are extremely close to it (10^-decades k_F)
    let mut best = (f64::MAX, 0.0);
    for (i, &k) in grid.x.iter().enumerate() {
        let d = (k - k_f).abs();
        if d < best.0 {
            best = (d, f[i]);
        }
    }
    best.1
}

/// Solve the gap equation at temperature `t` by fixed-point iteration, starting from
/// the constant `start`.  Returns the solution; `delta_f` = 0 if only the trivial
/// solution exists (the iteration then decays to zero).
pub fn solve_gap<K: Kernel>(p: &K, grid: &Grid, t: f64, start: f64, tol: f64, max_iter: usize) -> GapSolution {
    let n = grid.len();
    let km = kernel_matrix(p, grid);
    let xi: Vec<f64> = grid.x.iter().map(|&k| p.xi(k)).collect();
    let mut delta = vec![start; n];
    let mut converged = false;
    let mut it = 0;
    while it < max_iter {
        it += 1;
        let weight: Vec<f64> = (0..n)
            .map(|j| {
                let e = (xi[j] * xi[j] + delta[j] * delta[j]).sqrt();
                let th = if t > 0.0 { (e / (2.0 * t)).tanh() } else { 1.0 };
                if e > 0.0 { delta[j] * th / e } else { 0.0 }
            })
            .collect();
        let mut new = vec![0.0; n];
        let mut change: f64 = 0.0;
        let mut scale: f64 = 0.0;
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += km[i * n + j] * weight[j];
            }
            new[i] = s;
        }
        for i in 0..n {
            change = change.max((new[i] - delta[i]).abs());
            scale = scale.max(new[i].abs());
        }
        delta = new;
        if scale < 1e-300 {
            break;
        }
        if change < tol * scale {
            converged = true;
            break;
        }
    }
    let delta_f = value_at_fermi(grid, &delta, p.k_f());
    GapSolution { k: grid.x.clone(), delta, delta_f, iterations: it, converged }
}

/// Largest eigenvalue of the linearized gap equation at temperature `t`
/// (power iteration; the kernel is positive, so the eigenvector is positive).
pub fn linear_eigenvalue<K: Kernel>(p: &K, grid: &Grid, km: &[f64], t: f64) -> f64 {
    let n = grid.len();
    let weight: Vec<f64> = grid
        .x
        .iter()
        .map(|&k| {
            let x = p.xi(k).abs();
            if x < 1e-300 { 1.0 / (2.0 * t) } else { (x / (2.0 * t)).tanh() / x }
        })
        .collect();
    let mut v = vec![1.0; n];
    let mut lam = 0.0;
    for _ in 0..400 {
        let mut nv = vec![0.0; n];
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += km[i * n + j] * weight[j] * v[j];
            }
            nv[i] = s;
        }
        let norm = nv.iter().fold(0.0f64, |a, &b| a.max(b.abs()));
        if norm == 0.0 {
            return 0.0;
        }
        for x in nv.iter_mut() {
            *x /= norm;
        }
        let done = (norm - lam).abs() < 1e-12 * norm;
        lam = norm;
        v = nv;
        if done {
            break;
        }
    }
    lam
}

/// Critical temperature: the temperature at which the linearized gap equation has the
/// eigenvalue 1.  Returns None if the eigenvalue is below 1 even at `t_min`.
pub fn critical_temperature<K: Kernel>(p: &K, grid: &Grid, t_min: f64, t_max: f64) -> Option<f64> {
    let km = kernel_matrix(p, grid);
    if linear_eigenvalue(p, grid, &km, t_min) < 1.0 {
        return None;
    }
    if linear_eigenvalue(p, grid, &km, t_max) > 1.0 {
        return Some(t_max);
    }
    // bisection in log T: the eigenvalue decreases monotonically with T
    let (mut lo, mut hi) = (t_min.ln(), t_max.ln());
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if linear_eigenvalue(p, grid, &km, mid.exp()) > 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some((0.5 * (lo + hi)).exp())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The BCS model: a constant attraction U inside a shell |xi| < omega_D around the
    /// Fermi surface.  With a constant density of states,
    ///     Delta = omega_D / sinh(1/lambda),   T_c = (2 e^gamma/pi) omega_D e^{-1/lambda}
    /// (weak coupling), so that T_c/Delta -> e^gamma/pi = 0.5669.
    struct Shell {
        k_f: f64,
        m: f64,
        u: f64,
        omega: f64,
    }
    impl Kernel for Shell {
        fn w(&self, k: f64, kp: f64) -> f64 {
            if self.xi(k).abs() < self.omega && self.xi(kp).abs() < self.omega { self.u } else { 0.0 }
        }
        fn xi(&self, k: f64) -> f64 {
            (k * k - self.k_f * self.k_f) / (2.0 * self.m)
        }
        fn k_f(&self) -> f64 {
            self.k_f
        }
    }

    #[test]
    fn bcs_shell_model() {
        let (k_f, m) = (1.0, 1.0);
        let n0 = m * k_f / (2.0 * std::f64::consts::PI * std::f64::consts::PI);
        let lambda = 0.25;
        let omega = 0.01 * k_f * k_f / (2.0 * m); // narrow shell: the density of states is constant to 1%
        let sh = Shell { k_f, m, u: lambda / n0, omega };
        // grid: the shell edges are at k_F (1 -+ 0.005); resolve them and the Fermi surface
        let gl = gauss_legendre(16);
        let mut grid = Grid::new();
        let lo = (k_f * k_f - 2.0 * m * omega).sqrt();
        let hi = (k_f * k_f + 2.0 * m * omega).sqrt();
        let mut edges = vec![lo];
        for j in (0..24).rev() {
            edges.push(k_f - (k_f - lo) * 10f64.powf(-0.5 * (j as f64 + 0.5)) * 10f64.powf(0.25));
        }
        edges.retain(|&x| x >= lo && x < k_f);
        edges.push(k_f);
        for j in 0..24 {
            edges.push(k_f + (hi - k_f) * 10f64.powf(-0.5 * ((23 - j) as f64 + 0.5)) * 10f64.powf(0.25));
        }
        edges.retain(|&x| x <= hi);
        edges.push(hi);
        edges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        edges.dedup_by(|a, b| (*a - *b).abs() < 1e-15);
        for w in edges.windows(2) {
            grid.push_panel(w[0], w[1], &gl);
        }
        let sol = solve_gap(&sh, &grid, 0.0, 0.1 * omega, 1e-11, 20000);
        assert!(sol.converged);
        let exact = omega / (1.0 / lambda).sinh();
        assert!((sol.delta_f / exact - 1.0).abs() < 0.02, "gap {} vs {}", sol.delta_f, exact);
        let tc = critical_temperature(&sh, &grid, 1e-6 * omega, omega).unwrap();
        let ratio = tc / sol.delta_f;
        assert!((ratio - 0.5669).abs() < 0.02, "T_c/Delta = {}", ratio);
    }

    #[test]
    fn angular_average_matches_quadrature() {
        // composite quadrature with panels that shrink towards mu = 1, where the
        // integrand of a nearly massless exchange is nearly singular
        let gl = gauss_legendre(32);
        let mut grid = Grid::new();
        let edges = [-1.0, 0.0, 0.9, 0.99, 0.999, 0.9999, 0.99999, 1.0];
        for w in edges.windows(2) {
            grid.push_panel(w[0], w[1], &gl);
        }
        for &(a, b, c, d) in &[(1.0, 0.3, 2.0, 1.5), (0.2, -0.1, 1.0, 0.999), (3.0, 1.0, 5.0, 1e-7), (1.0, -1.0, 2.0, 1.2)] {
            let f: Vec<f64> = grid.x.iter().map(|&x| 0.5 * (a + b * x) / (c - d * x)).collect();
            let s = grid.integrate(&f);
            let an = angular_average(a, b, c, d);
            assert!((an - s).abs() < 1e-10 * (1.0 + s.abs()), "{} vs {}", an, s);
        }
    }

    /// The s-wave average of the kernel agrees with a direct quadrature of the closed
    /// forms of kernel.rs.
    #[test]
    fn w_matches_kernel_closed_forms() {
        use crate::kernel::{f_pseudo_hidden, f_scalar_hidden};
        let p = Pairing { m: 1.0, g: 2.0, m_s: 0.7, m_ph: 0.05, scalar: true, phase: true, phase_sign: 1.0, k_f: 0.5 };
        let gl = gauss_legendre(200);
        for &(k, kp) in &[(0.5, 0.5), (0.2, 1.7), (3.0, 0.1), (0.5, 0.52)] {
            let mut s = 0.0;
            for (x, w) in gl.0.iter().zip(&gl.1) {
                let q2 = k * k + kp * kp - 2.0 * k * kp * x;
                s += 0.5 * w * p.g * p.g * (f_scalar_hidden(p.m, k, kp, *x) / (q2 + p.m_s * p.m_s) + f_pseudo_hidden(p.m, k, kp, *x) / (q2 + p.m_ph * p.m_ph));
            }
            assert!((p.w(k, kp) - s).abs() < 1e-8 * s.abs(), "{} vs {}", p.w(k, kp), s);
        }
    }
}
