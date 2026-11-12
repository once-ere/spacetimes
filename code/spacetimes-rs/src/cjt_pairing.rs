//! The effective potential of Cornwall, Jackiw and Tomboulis (CJT) for the paired
//! state of the two models, in the approximation in which the gap equation of gap.rs
//! was derived: two-loop Gamma_2 (one exchanged boson), static exchange, particles
//! only, normal self-energy absorbed in the mass and the chemical potential.
//! Derivation: supplement S4, Sec. 3.
//!
//! THE POTENTIAL.  For a unitary pairing state (all eight internal states have the
//! same gap function Delta(k)) the CJT effective potential per unit volume, measured
//! from the unpaired state at the same temperature T and chemical potential mu, is
//!
//!   V[Delta] = nu Int d^3k/(2 pi)^3 { (|xi| - E)/2 - T [ln(1 + e^(-E/T)) - ln(1 + e^(-|xi|/T))] + Delta kappa }
//!              - (nu/2) Int Int d^3k d^3k'/(2 pi)^6  kappa(k) W(k, k') kappa(k') ,
//!
//!   E = sqrt(xi^2 + Delta^2),  xi = sqrt(m^2 + k^2) - mu,  kappa = Delta tanh(E/2T)/(2E),  nu = 8.
//!
//! kappa(k) is the pair amplitude <b(-k) b(k)> of the propagator whose anomalous
//! self-energy is Delta(k); W is the s-wave average of the exchange attraction
//! (gap.rs).  The first line is the "Tr ln" and "Tr S0^-1 S" part of the CJT action,
//! the second line is Gamma_2.  Its derivative is
//!
//!   dV/dDelta(k) = nu [d kappa/d Delta] ( Delta(k) - Int d^3k'/(2 pi)^3 W(k, k') kappa(k') ) ,
//!
//! so V is stationary exactly at the solutions of the gap equation, Eq. (5.2) of
//! supplement S1.  On a radial grid, Int d^3k/(2 pi)^3 f = sum_i omega_i f_i with
//! omega_i = w_i k_i^2/(2 pi^2).
//!
//! WHAT IS COMPUTED HERE.
//!  * `Potential::value`, `gradient`, `newton`: the potential, and a solution of the gap
//!    equation by Newton's method, an algorithm different from the fixed-point
//!    iteration of gap.rs (check of the earlier results);
//!  * `stability_eigenvalues`: the signs of the second variation of V (a minimum at
//!    the paired solution, an instability of the unpaired state below T_c), and the
//!    critical temperature from a symmetric eigenvalue problem (check of the T_c of gap.rs);
//!  * `entropy`, `gl_fit`, `pair_bubble_curvature`: thermodynamics and the three
//!    coefficients of the Ginzburg-Landau energy (check of supplement S1, Sec. 6.2);
//!  * `threshold_d`: the linearized gap equation in vacuum is the Schroedinger
//!    equation of two particles; its threshold reproduces the critical coupling of the
//!    Yukawa potential found by dimer.rs in coordinate space;
//!  * `ShellModel`: the potential on the eight-dimensional space of the order
//!    parameter Delta^a (a = 0, 5, 6, 7), built from the Dirac matrices of each
//!    signature: ground-state manifold, flat directions (Goldstone modes), and the
//!    half-paired stationary state.
//!
//! UNITS: hbar = c = k_B = 1; with m = 1 all energies are in units of the fermion mass.

use crate::complex::C64;
use crate::gap::{Kernel, Pairing};
use crate::kernel::{add, dagger, mul, norm2, scale, trace, transpose, zeros, Mat, OneParticle};
use crate::linalg::{eigh, solve_real, CMat};
use crate::quad::{gauss_legendre, log_grid, Grid};

/// Number of internal states of a particle of given momentum.
pub const NU: f64 = 8.0;

/// ln(1 + e^(-x)) for x >= 0, accurate for large x.
fn ln1pexp(x: f64) -> f64 {
    (-x).exp().ln_1p()
}

/// tanh(E/2T), with the value 1 at T = 0.
fn th(e: f64, t: f64) -> f64 {
    if t > 0.0 { (e / (2.0 * t)).tanh() } else { 1.0 }
}

/// The pair amplitude kappa = Delta tanh(E/2T)/(2E) and its derivative with respect to
/// Delta at fixed xi:  d kappa/d Delta = xi^2 tanh(E/2T)/(2E^3) + Delta^2 sech^2(E/2T)/(4 T E^2).
pub fn kappa(xi: f64, d: f64, t: f64) -> (f64, f64) {
    let e = (xi * xi + d * d).sqrt();
    if e < 1e-300 {
        return if t > 0.0 { (0.0, 1.0 / (4.0 * t)) } else { (0.0, 0.0) };
    }
    if t > 0.0 && e < 1e-6 * t {
        // tanh(x)/x = 1 - x^2/3 for x = E/2T -> 0: kappa = Delta/(4T), slope 1/(4T)
        return (d / (4.0 * t), 1.0 / (4.0 * t));
    }
    let tanh_ = th(e, t);
    let sech2 = if t > 0.0 { 1.0 - tanh_ * tanh_ } else { 0.0 };
    let k = d * tanh_ / (2.0 * e);
    let dk = xi * xi * tanh_ / (2.0 * e * e * e) + if t > 0.0 { d * d * sech2 / (4.0 * t * e * e) } else { 0.0 };
    (k, dk)
}

/// The CJT potential of the paired state on a momentum grid.
pub struct Potential {
    /// grid momenta
    pub k: Vec<f64>,
    /// integration weights omega_i = w_i k_i^2/(2 pi^2)
    pub omega: Vec<f64>,
    /// xi_i = E(k_i) - mu
    pub xi: Vec<f64>,
    /// the kernel W(k_i, k_j), row-major
    pub w: Vec<f64>,
    pub k_f: f64,
}

impl Potential {
    pub fn new<K: Kernel>(p: &K, grid: &Grid) -> Potential {
        let n = grid.len();
        let pi = std::f64::consts::PI;
        let mut w = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                w[i * n + j] = p.w(grid.x[i], grid.x[j]);
            }
        }
        Potential {
            k: grid.x.clone(),
            omega: (0..n).map(|i| grid.w[i] * grid.x[i] * grid.x[i] / (2.0 * pi * pi)).collect(),
            xi: grid.x.iter().map(|&k| p.xi(k)).collect(),
            w,
            k_f: p.k_f(),
        }
    }

    pub fn n(&self) -> usize {
        self.k.len()
    }

    /// The right-hand side of the gap equation, h_i = sum_j omega_j W_ij kappa_j.
    pub fn field(&self, d: &[f64], t: f64) -> Vec<f64> {
        let n = self.n();
        let kap: Vec<f64> = (0..n).map(|j| self.omega[j] * kappa(self.xi[j], d[j], t).0).collect();
        (0..n).map(|i| (0..n).map(|j| self.w[i * n + j] * kap[j]).sum()).collect()
    }

    /// V[Delta] - V[0] per unit volume (see the module documentation).
    pub fn value(&self, d: &[f64], t: f64) -> f64 {
        let n = self.n();
        let mut v = 0.0;
        let mut kap = vec![0.0; n];
        for i in 0..n {
            let ax = self.xi[i].abs();
            let e = (ax * ax + d[i] * d[i]).sqrt();
            // (|xi| - E)/2 in a form without cancellation
            let qp = if e > 0.0 { -0.5 * d[i] * d[i] / (ax + e) } else { 0.0 };
            let thermal = if t > 0.0 { -t * (ln1pexp(e / t) - ln1pexp(ax / t)) } else { 0.0 };
            kap[i] = kappa(self.xi[i], d[i], t).0;
            v += NU * self.omega[i] * (qp + thermal + d[i] * kap[i]);
        }
        let mut inter = 0.0;
        for i in 0..n {
            let mut s = 0.0;
            for j in 0..n {
                s += self.w[i * n + j] * self.omega[j] * kap[j];
            }
            inter += self.omega[i] * kap[i] * s;
        }
        v - 0.5 * NU * inter
    }

    /// The gradient dV/dDelta_i = nu omega_i kappa'_i (Delta_i - h_i).
    pub fn gradient(&self, d: &[f64], t: f64) -> Vec<f64> {
        let h = self.field(d, t);
        (0..self.n()).map(|i| NU * self.omega[i] * kappa(self.xi[i], d[i], t).1 * (d[i] - h[i])).collect()
    }

    /// Largest violation of the gap equation, max_i |Delta_i - h_i| / max_i |Delta_i|.
    pub fn residual(&self, d: &[f64], t: f64) -> f64 {
        let h = self.field(d, t);
        let scale = d.iter().fold(0.0f64, |a, &b| a.max(b.abs())).max(1e-300);
        d.iter().zip(&h).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max) / scale
    }

    /// Solve the gap equation F(Delta) = Delta - h(Delta) = 0 by Newton's method with
    /// the exact Jacobian J_ij = delta_ij - W_ij omega_j kappa'_j, halving the step
    /// while the residual does not decrease.  Returns (solution, iterations, converged).
    pub fn newton(&self, start: &[f64], t: f64, tol: f64, max_iter: usize) -> (Vec<f64>, usize, bool) {
        let n = self.n();
        let mut d = start.to_vec();
        let norm = |d: &[f64], h: &[f64]| d.iter().zip(h).map(|(a, b)| (a - b) * (a - b)).sum::<f64>().sqrt();
        for it in 0..max_iter {
            let h = self.field(&d, t);
            let r0 = norm(&d, &h);
            let scale_d = d.iter().map(|x| x * x).sum::<f64>().sqrt().max(1e-300);
            if r0 < tol * scale_d {
                return (d, it, true);
            }
            let mut jac = vec![0.0; n * n];
            for j in 0..n {
                let dk = self.omega[j] * kappa(self.xi[j], d[j], t).1;
                for i in 0..n {
                    jac[i * n + j] = -self.w[i * n + j] * dk;
                }
                jac[j * n + j] += 1.0;
            }
            let rhs: Vec<f64> = (0..n).map(|i| h[i] - d[i]).collect();
            let step = match solve_real(&jac, &rhs, n) {
                Some(s) => s,
                None => return (d, it, false),
            };
            let mut fac = 1.0;
            loop {
                let trial: Vec<f64> = (0..n).map(|i| d[i] + fac * step[i]).collect();
                let ht = self.field(&trial, t);
                if norm(&trial, &ht) < r0 || fac < 1e-4 {
                    d = trial;
                    break;
                }
                fac *= 0.5;
            }
        }
        (d, max_iter, false)
    }

    /// Eigenvalues (ascending) of 1 - Ktilde, where
    /// Ktilde_ij = sqrt(omega_i kappa'_i) W_ij sqrt(omega_j kappa'_j).
    /// At a solution of the gap equation the matrix of second derivatives of V is
    /// D^(1/2) (1 - Ktilde) D^(1/2) with the positive diagonal matrix
    /// D = nu omega kappa', so the signs of these eigenvalues are the signs of the
    /// second variation of V (Sylvester's law of inertia).  At Delta = 0 the same
    /// statement holds for the unpaired state, and Ktilde has the eigenvalues of the
    /// linearized gap equation.
    pub fn stability_eigenvalues(&self, d: &[f64], t: f64) -> Vec<f64> {
        let n = self.n();
        let s: Vec<f64> = (0..n).map(|i| (self.omega[i] * kappa(self.xi[i], d[i], t).1).max(0.0).sqrt()).collect();
        let mut m = CMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                let v = if i == j { 1.0 } else { 0.0 } - s[i] * self.w[i * n + j] * s[j];
                m.set(i, j, C64::new(v, 0.0));
            }
        }
        eigh(&m).0
    }

    /// The real symmetric matrix 1 - Ktilde (row-major); see `stability_eigenvalues`.
    fn stability_matrix(&self, d: &[f64], t: f64) -> Vec<f64> {
        let n = self.n();
        let s: Vec<f64> = (0..n).map(|i| (self.omega[i] * kappa(self.xi[i], d[i], t).1).max(0.0).sqrt()).collect();
        let mut m = vec![0.0; n * n];
        for i in 0..n {
            for j in 0..n {
                m[i * n + j] = if i == j { 1.0 } else { 0.0 } - s[i] * self.w[i * n + j] * s[j];
            }
        }
        m
    }

    /// Smallest eigenvalue of 1 - Ktilde if all eigenvalues are positive (the state is
    /// a minimum of V), None otherwise.  Method: Cholesky factorization L L^T, which
    /// exists exactly for positive definite matrices, followed by inverse iteration
    /// (the largest eigenvalue of the inverse).
    pub fn minimum_curvature(&self, d: &[f64], t: f64) -> Option<f64> {
        let n = self.n();
        let mut l = self.stability_matrix(d, t);
        for j in 0..n {
            let mut diag = l[j * n + j];
            for k in 0..j {
                diag -= l[j * n + k] * l[j * n + k];
            }
            if diag <= 0.0 {
                return None;
            }
            let dj = diag.sqrt();
            l[j * n + j] = dj;
            for i in j + 1..n {
                let mut v = l[i * n + j];
                for k in 0..j {
                    v -= l[i * n + k] * l[j * n + k];
                }
                l[i * n + j] = v / dj;
            }
        }
        // inverse iteration: solve L L^T y = x
        let mut x = vec![1.0; n];
        let mut mu = 0.0;
        for _ in 0..500 {
            let mut y = x.clone();
            for i in 0..n {
                for k in 0..i {
                    y[i] -= l[i * n + k] * y[k];
                }
                y[i] /= l[i * n + i];
            }
            for i in (0..n).rev() {
                for k in i + 1..n {
                    y[i] -= l[k * n + i] * y[k];
                }
                y[i] /= l[i * n + i];
            }
            let nrm = y.iter().map(|v| v * v).sum::<f64>().sqrt();
            let done = (nrm - mu).abs() < 1e-12 * nrm;
            mu = nrm;
            x = y.iter().map(|v| v / nrm).collect();
            if done {
                break;
            }
        }
        Some(1.0 / mu)
    }

    /// Sign of the determinant of 1 - Ktilde (Gaussian elimination with partial
    /// pivoting).  Near the critical temperature one eigenvalue changes sign, and
    /// with it the determinant: +1 above T_c, -1 just below.
    pub fn second_variation_sign(&self, d: &[f64], t: f64) -> f64 {
        let n = self.n();
        let mut m = self.stability_matrix(d, t);
        let mut sign = 1.0;
        for col in 0..n {
            let mut piv = col;
            for r in col + 1..n {
                if m[r * n + col].abs() > m[piv * n + col].abs() {
                    piv = r;
                }
            }
            if m[piv * n + col] == 0.0 {
                return 0.0;
            }
            if piv != col {
                for c in 0..n {
                    m.swap(col * n + c, piv * n + c);
                }
                sign = -sign;
            }
            if m[col * n + col] < 0.0 {
                sign = -sign;
            }
            for r in col + 1..n {
                let f = m[r * n + col] / m[col * n + col];
                if f != 0.0 {
                    for c in col + 1..n {
                        m[r * n + c] -= f * m[col * n + c];
                    }
                }
            }
        }
        sign
    }

    /// Critical temperature from the second variation of V at Delta = 0: the
    /// temperature at which the determinant of 1 - Ktilde changes sign (bisection in
    /// ln T between t_lo, where the unpaired state is unstable, and t_hi, where it is
    /// stable; the bracket must contain exactly one sign change).
    pub fn critical_temperature(&self, t_lo: f64, t_hi: f64, steps: usize) -> f64 {
        let zero = vec![0.0; self.n()];
        let (mut lo, mut hi) = (t_lo.ln(), t_hi.ln());
        for _ in 0..steps {
            let mid = 0.5 * (lo + hi);
            if self.second_variation_sign(&zero, mid.exp()) < 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        (0.5 * (lo + hi)).exp()
    }

    /// Entropy density of the quasi-particle gas,
    /// s = nu Int d^3k/(2 pi)^3 [ ln(1 + e^(-E/T)) + (E/T)/(e^(E/T) + 1) ].
    pub fn entropy(&self, d: &[f64], t: f64) -> f64 {
        if t <= 0.0 {
            return 0.0;
        }
        (0..self.n())
            .map(|i| {
                let x = (self.xi[i] * self.xi[i] + d[i] * d[i]).sqrt() / t;
                NU * self.omega[i] * (ln1pexp(x) + x / (x.exp() + 1.0))
            })
            .sum()
    }

    /// The value of a function on the grid at the node nearest to the Fermi momentum.
    pub fn at_fermi(&self, f: &[f64]) -> f64 {
        let mut best = (f64::MAX, 0.0);
        for (i, &k) in self.k.iter().enumerate() {
            let dist = (k - self.k_f).abs();
            if dist < best.0 {
                best = (dist, f[i]);
            }
        }
        best.1
    }

    /// The eigenvector of the linearized gap equation with the largest eigenvalue at
    /// temperature t, normalized to 1 at the Fermi momentum (power iteration).
    pub fn critical_shape(&self, t: f64) -> Vec<f64> {
        let n = self.n();
        let wt: Vec<f64> = (0..n).map(|j| self.omega[j] * kappa(self.xi[j], 0.0, t).1).collect();
        let mut v = vec![1.0; n];
        for _ in 0..500 {
            let nv: Vec<f64> = (0..n).map(|i| (0..n).map(|j| self.w[i * n + j] * wt[j] * v[j]).sum()).collect();
            let s = self.at_fermi(&nv);
            v = nv.iter().map(|x| x / s).collect();
        }
        v
    }

    /// Ginzburg-Landau coefficients from the potential: along the ray
    /// Delta(k) = s phi(k) (phi = `shape`, 1 at the Fermi surface),
    /// V = alpha s^2 + (beta/2) s^4 + O(s^6).  Least-squares fit of V/s^2 to a
    /// polynomial of second degree in s^2 at the amplitudes `s_values`.
    /// Returns (alpha, beta).
    pub fn gl_fit(&self, shape: &[f64], t: f64, s_values: &[f64]) -> (f64, f64) {
        // normal equations for y = c0 + c1 x + c2 x^2 with x = s^2, y = V/s^2
        let mut a = [0.0f64; 9];
        let mut b = [0.0f64; 3];
        for &s in s_values {
            let d: Vec<f64> = shape.iter().map(|p| s * p).collect();
            let x = s * s;
            let y = self.value(&d, t) / x;
            let pw = [1.0, x, x * x];
            for i in 0..3 {
                for j in 0..3 {
                    a[i * 3 + j] += pw[i] * pw[j];
                }
                b[i] += pw[i] * y;
            }
        }
        let c = solve_real(&a, &b, 3).expect("singular fit");
        (c[0], 2.0 * c[1])
    }
}

/// The coefficient K of the gradient term of the Ginzburg-Landau energy,
/// F = ... + K Tr[grad M . grad M^dagger], from the pair susceptibility at finite pair
/// momentum q:
///     Pi(q) = Int d^3k/(2 pi)^3 [tanh(xi_+/2T) + tanh(xi_-/2T)] / (2 (xi_+ + xi_-)),   xi_+- = xi(|k +- q/2|),
///     K = lim_{q -> 0} [Pi(0) - Pi(q)] / (2 q^2).
/// Evaluated for two small q and extrapolated linearly in q^2.  `grid`: a momentum
/// grid that resolves the Fermi surface.
pub fn pair_bubble_curvature(p: &Pairing, grid: &Grid, t: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let gl = gauss_legendre(48);
    let v_f = p.k_f / p.mu();
    let bubble = |q: f64| -> f64 {
        let mut s = 0.0;
        for (i, &k) in grid.x.iter().enumerate() {
            let mut ang = 0.0;
            for (x, wx) in gl.0.iter().zip(&gl.1) {
                let kp = (k * k + 0.25 * q * q + k * q * x).sqrt();
                let km = (k * k + 0.25 * q * q - k * q * x).sqrt();
                let (a, b) = ((p.energy(kp) - p.mu()) / (2.0 * t), (p.energy(km) - p.mu()) / (2.0 * t));
                // [tanh a + tanh b]/(a + b), finite for a + b -> 0
                let f = if (a + b).abs() < 1e-7 { 1.0 / (0.5 * (a - b)).cosh().powi(2) } else { (a.tanh() + b.tanh()) / (a + b) };
                // difference from q = 0, where the integrand is tanh(a0)/a0
                let a0 = (p.energy(k) - p.mu()) / (2.0 * t);
                let f0 = if a0.abs() < 1e-7 { 1.0 } else { a0.tanh() / a0 };
                ang += 0.5 * wx * (f0 - f);
            }
            s += grid.w[i] * k * k / (2.0 * pi * pi) * ang / (4.0 * t);
        }
        s
    };
    // q such that v_F q/(2 pi T) = 0.05 and 0.1
    let q1 = 0.05 * 2.0 * pi * t / v_f;
    let q2 = 2.0 * q1;
    let (k1, k2) = (bubble(q1) / (2.0 * q1 * q1), bubble(q2) / (2.0 * q2 * q2));
    // K(q) = K0 + c q^2:  K0 = (4 K(q1) - K(q2))/3
    (4.0 * k1 - k2) / 3.0
}

/// Threshold of the two-particle bound state from the linearized gap equation.
///
/// In vacuum (no Fermi sea, T = 0, mu = m - E_b/2) the linearized gap equation
///     Delta(k) = Int d^3k'/(2 pi)^3 W(k, k') Delta(k') / (2 (E' - mu))
/// is, for psi = Delta/(2 (E - mu)), the Schroedinger equation of the relative motion
/// of two particles with the potential -W: (k^2/m + E_b) psi = Int W psi (for k << m).
/// A bound state appears (E_b = 0, mu = m) when the largest eigenvalue of the kernel
/// reaches 1.  The function returns the critical value of D = alpha m / m_s, with
/// alpha = g^2/(4 pi), for the exchange of the radial mode of mass `m_s` (fermion mass
/// 1), including the relativistic form of the kernel and of the kinetic energy.  For
/// m_s -> 0 it must approach the critical coupling 1.6798 of the Yukawa potential.
pub fn threshold_d(m_s: f64) -> f64 {
    let pi = std::f64::consts::PI;
    let p = Pairing { m: 1.0, g: 1.0, m_s, m_ph: 0.0, scalar: true, phase: false, phase_sign: 1.0, k_f: 0.0 };
    // momenta from far below to far above the inverse range m_s of the potential
    let gl = gauss_legendre(8);
    let mut grid = Grid::new();
    grid.push_panel(0.0, 1e-3 * m_s, &gl);
    let lg = log_grid(1e-3 * m_s, 3e3 * m_s, 96, 8);
    grid.x.extend_from_slice(&lg.x);
    grid.w.extend_from_slice(&lg.w);
    let n = grid.len();
    // A_ij = omega_j W_ij / (2 (E_j - m)),  E - m = k^2/(E + m)
    let mut a = vec![0.0; n * n];
    for j in 0..n {
        let k = grid.x[j];
        let e = p.energy(k);
        // omega_j / (2 (E_j - m)) = [w_j k_j^2/(2 pi^2)] (E_j + m)/(2 k_j^2)
        let wt = grid.w[j] * (e + 1.0) / (4.0 * pi * pi);
        for i in 0..n {
            a[i * n + j] = p.w(grid.x[i], k) * wt;
        }
    }
    // largest eigenvalue for g = 1 by power iteration (the kernel is positive)
    let mut v = vec![1.0; n];
    let mut lam = 0.0;
    for _ in 0..2000 {
        let nv: Vec<f64> = (0..n).map(|i| (0..n).map(|j| a[i * n + j] * v[j]).sum()).collect();
        let nrm = nv.iter().fold(0.0f64, |x, &y| x.max(y.abs()));
        let done = (nrm - lam).abs() < 1e-13 * nrm;
        lam = nrm;
        v = nv.iter().map(|x| x / nrm).collect();
        if done {
            break;
        }
    }
    // eigenvalue = g^2 lam = 1  ->  g_c^2 = 1/lam;  D = g^2/(4 pi m_s)
    1.0 / (lam * 4.0 * pi * m_s)
}

// ------------------------------------------------------------------------------------
// The order-parameter space: potential as a function of the four complex numbers Delta^a
// ------------------------------------------------------------------------------------

/// The 8x8 gap matrix M = sum_a Delta^a Phihat_a(p) and the four normalized
/// two-particle contents Phihat_a = Phi_a / sqrt(N_a/8),
/// Phi_a = U(p)^T C^- gamma^a U(-p), a = 0, 5, 6, 7 (so that Tr[Phihat_a^dagger Phihat_b] = 8 delta_ab).
pub fn gap_matrix(op: &OneParticle, delta: &[C64; 4], p: &[f64; 8], m: f64) -> (Mat, Vec<Mat>) {
    let mut pm = *p;
    for x in pm.iter_mut() {
        *x = -*x;
    }
    let (u, _) = op.particles(p, m);
    let (um, _) = op.particles(&pm, m);
    let mut gap = zeros(8, 8);
    let mut phis = Vec::new();
    for (i, &a) in [0usize, 5, 6, 7].iter().enumerate() {
        let phi = mul(&mul(&transpose(&u), &op.pair_vector(a)), &um);
        let n = norm2(&phi);
        let hat = scale(&phi, C64::new((8.0 / n).sqrt(), 0.0));
        gap = add(&gap, &scale(&hat, delta[i]));
        phis.push(hat);
    }
    (gap, phis)
}

/// The eight eigenvalues d_r^2 of M M^dagger (ascending) and the eigenvectors.
fn gap_spectrum(m: &Mat) -> (Vec<f64>, CMat) {
    let mm = mul(m, &dagger(m));
    let mut h = CMat::zeros(8);
    for i in 0..8 {
        for j in 0..8 {
            h.set(i, j, mm[i][j]);
        }
    }
    eigh(&h)
}

/// Squares of the eight gaps, d_r^2, for the order parameter `delta`.
pub fn gap_squares(op: &OneParticle, delta: &[C64; 4], p: &[f64; 8], m: f64) -> Vec<f64> {
    gap_spectrum(&gap_matrix(op, delta, p, m).0).0
}

/// The CJT potential at T = 0 in the weak-coupling ("shell") form: a density of states
/// N(0) per internal state that is constant for |xi| < omega_c, and the same constant
/// attraction W_0 in each of the four vector pair channels.  For a gap matrix M with
/// the eigenvalues d_r^2 and eigenvectors |r> of M M^dagger,
///
///   V = sum_r { -N(0) [ (omega_c sqrt(omega_c^2 + d_r^2) + d_r^2 asinh(omega_c/d_r))/2 - omega_c^2/2 ] + f_r d_r^2 }
///       - (nu/2) W_0 sum_a |kappa^a|^2 ,
///   f_r = N(0) asinh(omega_c/d_r),   kappa = sum_r f_r |r><r| M,   kappa^a = Tr[Phihat_a^dagger kappa]/8 .
///
/// The first term is the energy of the quasi-particle vacuum relative to the Fermi
/// sea, the second is Re Tr[M^dagger kappa], the third is Gamma_2.  For a unitary M
/// with gap Delta the gap equation is 1 = W_0 N(0) asinh(omega_c/Delta).
#[derive(Clone, Copy, Debug)]
pub struct ShellModel {
    /// density of states per internal state
    pub n0: f64,
    /// half-width of the shell
    pub omega_c: f64,
    /// attraction in each vector channel
    pub w0: f64,
}

impl ShellModel {
    /// The model whose unitary state has the gap `delta0`.
    pub fn with_gap(n0: f64, omega_c: f64, delta0: f64) -> ShellModel {
        ShellModel { n0, omega_c, w0: 1.0 / (n0 * (omega_c / delta0).asinh()) }
    }

    /// The potential for the order parameter `delta` (components along x0, x5, x6, x7).
    pub fn potential(&self, op: &OneParticle, delta: &[C64; 4], p: &[f64; 8], m: f64) -> f64 {
        let (gap, phis) = gap_matrix(op, delta, p, m);
        let (d2, vecs) = gap_spectrum(&gap);
        let wc = self.omega_c;
        let mut v = 0.0;
        // kappa = V diag(f) V^dagger M
        let mut proj = zeros(8, 8);
        for r in 0..8 {
            let d = d2[r].max(0.0).sqrt().max(1e-14 * wc);
            let f = self.n0 * (wc / d).asinh();
            v += -self.n0 * (0.5 * (wc * (wc * wc + d * d).sqrt() + d * d * (wc / d).asinh()) - 0.5 * wc * wc) + f * d * d;
            for i in 0..8 {
                for j in 0..8 {
                    proj[i][j] += vecs.get(i, r) * vecs.get(j, r).conj() * f;
                }
            }
        }
        let kap = mul(&proj, &gap);
        for phi in &phis {
            let ka = trace(&mul(&dagger(phi), &kap)) * (1.0 / 8.0);
            v -= 0.5 * NU * self.w0 * ka.norm_sqr();
        }
        v
    }

    /// Eigenvalues (ascending) of the matrix of second derivatives of the potential
    /// with respect to the eight real coordinates (Re Delta^0, Im Delta^0, Re Delta^5, ...)
    /// at the point `delta`, by central differences with the step `h`; and the
    /// gradient norm at that point.
    pub fn hessian(&self, op: &OneParticle, delta: &[C64; 4], p: &[f64; 8], m: f64, h: f64) -> (Vec<f64>, CMat, f64) {
        let point = |x: &[f64; 8]| -> [C64; 4] { [C64::new(x[0], x[1]), C64::new(x[2], x[3]), C64::new(x[4], x[5]), C64::new(x[6], x[7])] };
        let mut x0 = [0.0f64; 8];
        for a in 0..4 {
            x0[2 * a] = delta[a].re;
            x0[2 * a + 1] = delta[a].im;
        }
        let f = |x: &[f64; 8]| self.potential(op, &point(x), p, m);
        let f0 = f(&x0);
        let mut hess = CMat::zeros(8);
        let mut grad2 = 0.0;
        for i in 0..8 {
            let (mut xp, mut xm) = (x0, x0);
            xp[i] += h;
            xm[i] -= h;
            let (fp, fm) = (f(&xp), f(&xm));
            grad2 += ((fp - fm) / (2.0 * h)).powi(2);
            hess.set(i, i, C64::new((fp - 2.0 * f0 + fm) / (h * h), 0.0));
            for j in 0..i {
                let (mut a, mut b, mut c_, mut d) = (x0, x0, x0, x0);
                a[i] += h;
                a[j] += h;
                b[i] += h;
                b[j] -= h;
                c_[i] -= h;
                c_[j] += h;
                d[i] -= h;
                d[j] -= h;
                let v = (f(&a) - f(&b) - f(&c_) + f(&d)) / (4.0 * h * h);
                hess.set(i, j, C64::new(v, 0.0));
                hess.set(j, i, C64::new(v, 0.0));
            }
        }
        let (w, vecs) = eigh(&hess);
        (w, vecs, grad2.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gamma::Gammas;
    use crate::gap::{critical_temperature, fermi_grid, solve_gap};

    fn test_pairing() -> Pairing {
        // moderately weak coupling: lambda about 0.3
        Pairing { m: 1.0, g: 3.0, m_s: 1.0, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f: 0.5 }
    }

    /// The analytic gradient agrees with finite differences of the potential.
    #[test]
    fn gradient_against_finite_differences() {
        let p = test_pairing();
        let grid = fermi_grid(p.k_f, 12.0, 4, 6);
        let pot = Potential::new(&p, &grid);
        let n = pot.n();
        for &t in &[0.0, 0.01] {
            let d: Vec<f64> = pot.k.iter().map(|&k| 0.03 / (1.0 + k * k)).collect();
            let g = pot.gradient(&d, t);
            for &i in &[3usize, n / 3, n / 2, n - 5] {
                let h = 1e-6;
                let (mut dp, mut dm) = (d.clone(), d.clone());
                dp[i] += h;
                dm[i] -= h;
                let num = (pot.value(&dp, t) - pot.value(&dm, t)) / (2.0 * h);
                assert!((num - g[i]).abs() < 1e-6 * g[i].abs() + 1e-13, "T = {}: {} vs {}", t, num, g[i]);
            }
        }
    }

    /// Newton's method started from a different function reaches the solution of the
    /// fixed-point iteration of gap.rs; the potential is stationary there and smaller
    /// than for rescaled gap functions.
    #[test]
    fn newton_solution_is_the_fixed_point_and_a_minimum() {
        let p = test_pairing();
        let grid = fermi_grid(p.k_f, 12.0, 5, 6);
        let pot = Potential::new(&p, &grid);
        let fp = solve_gap(&p, &grid, 0.0, 0.01, 1e-12, 100000);
        assert!(fp.converged && fp.delta_f > 1e-5);
        let start: Vec<f64> = pot.k.iter().map(|&k| 3.0 * fp.delta_f / (1.0 + 4.0 * (k - p.k_f).powi(2))).collect();
        let (d, _, conv) = pot.newton(&start, 0.0, 1e-12, 200);
        assert!(conv);
        let worst = d.iter().zip(&fp.delta).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max);
        assert!(worst < 1e-7 * fp.delta_f, "Newton and fixed point differ by {}", worst / fp.delta_f);
        let v0 = pot.value(&d, 0.0);
        assert!(v0 < 0.0);
        for s in [0.9, 0.97, 1.03, 1.1] {
            let ds: Vec<f64> = d.iter().map(|x| s * x).collect();
            assert!(pot.value(&ds, 0.0) > v0);
        }
        // all second variations positive at the solution
        assert!(pot.stability_eigenvalues(&d, 0.0)[0] > 0.0);
    }

    /// The temperature at which the second variation of V at Delta = 0 changes sign is
    /// the critical temperature of the linearized gap equation (gap.rs).
    #[test]
    fn critical_temperature_from_the_second_variation() {
        let p = test_pairing();
        let grid = fermi_grid(p.k_f, 12.0, 4, 6);
        let pot = Potential::new(&p, &grid);
        let tc = critical_temperature(&p, &grid, 1e-8, 1.0).unwrap();
        let tc2 = pot.critical_temperature(0.8 * tc, 1.25 * tc, 24);
        assert!((tc2 / tc - 1.0).abs() < 1e-5, "{} vs {}", tc2, tc);
        // the same through the eigenvalues of the symmetric matrix
        let zero = vec![0.0; pot.n()];
        assert!(pot.stability_eigenvalues(&zero, 0.99 * tc)[0] < 0.0 && pot.stability_eigenvalues(&zero, 1.01 * tc)[0] > 0.0);
        assert!(pot.minimum_curvature(&zero, 0.99 * tc).is_none());
        let c = pot.minimum_curvature(&zero, 1.2 * tc).unwrap();
        assert!((c - pot.stability_eigenvalues(&zero, 1.2 * tc)[0]).abs() < 1e-8);
    }

    /// The linearized gap equation in vacuum reproduces the critical coupling of the
    /// Yukawa potential (1.6798) when the range of the force is long compared with
    /// the Compton wavelength of the fermions.
    #[test]
    fn bound_state_threshold_agrees_with_the_schroedinger_equation() {
        let d = threshold_d(0.01);
        assert!((d - 1.6798).abs() < 0.01, "D_c = {}", d);
    }

    /// The order-parameter space, for both signatures: two fourfold gaps
    /// d^2 = S +- sqrt(S^2 - |eta_ab Delta^a Delta^b|^2); a unitary ground state with
    /// four flat directions; a stationary half-paired state with half the energy.
    #[test]
    fn order_parameter_space() {
        let gm = Gammas::build();
        let p = [0.0, 0.3, -0.2, 0.5, 0.0, 0.0, 0.0, 0.0];
        for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
            let eta: [f64; 4] = if op.tag == "1644" { [1.0, -1.0, -1.0, -1.0] } else { [1.0; 4] };
            let c = |re: f64, im: f64| C64::new(re, im);
            let delta = [c(0.3, -0.7), c(1.1, 0.2), c(-0.4, 0.9), c(0.5, 0.5)];
            let s: f64 = delta.iter().map(|z| z.norm_sqr()).sum();
            let mut dd = c(0.0, 0.0);
            for (z, e) in delta.iter().zip(&eta) {
                dd += *z * *z * *e;
            }
            let root = (s * s - dd.norm_sqr()).sqrt();
            let d2 = gap_squares(&op, &delta, &p, 1.0);
            for r in 0..4 {
                assert!((d2[r] - (s - root)).abs() < 1e-9 && (d2[r + 4] - (s + root)).abs() < 1e-9, "{} {:?}", op.tag, d2);
            }
            // the contents are orthonormal: Tr[Phihat_a^dagger Phihat_b] = 8 delta_ab
            let (_, phis) = gap_matrix(&op, &delta, &p, 1.0);
            for a in 0..4 {
                for b in 0..4 {
                    let t = trace(&mul(&dagger(&phis[a]), &phis[b]));
                    assert!((t - c(if a == b { 8.0 } else { 0.0 }, 0.0)).abs() < 1e-9);
                }
            }
            // shell model with gap 1, shell width 200, N(0) = 1
            let sm = ShellModel::with_gap(1.0, 200.0, 1.0);
            let ground = [c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)];
            let v_g = sm.potential(&op, &ground, &p, 1.0);
            assert!((v_g + 2.0).abs() < 1e-3, "condensation energy {}", v_g);
            let (w, _, grad) = sm.hessian(&op, &ground, &p, 1.0, 1e-3);
            assert!(grad < 1e-6);
            for r in 0..4 {
                assert!(w[r].abs() < 1e-4, "{}: flat direction {} has curvature {}", op.tag, r, w[r]);
                assert!(w[r + 4] > 1.0, "{}: massive direction {}", op.tag, w[r + 4]);
            }
            // half-paired state: eta_ab Delta^a Delta^b = 0, larger gap equal to the unitary gap
            let half = if op.tag == "1644" { [c(0.5f64.sqrt(), 0.0), c(0.5f64.sqrt(), 0.0), c(0.0, 0.0), c(0.0, 0.0)] } else { [c(0.5f64.sqrt(), 0.0), c(0.0, 0.5f64.sqrt()), c(0.0, 0.0), c(0.0, 0.0)] };
            let d2h = gap_squares(&op, &half, &p, 1.0);
            assert!(d2h[0].abs() < 1e-9 && (d2h[7] - 2.0).abs() < 1e-9);
            let scale_to_unit_gap: Vec<C64> = half.iter().map(|z| z.scale(0.5f64.sqrt())).collect();
            let half1 = [scale_to_unit_gap[0], scale_to_unit_gap[1], scale_to_unit_gap[2], scale_to_unit_gap[3]];
            let v_h = sm.potential(&op, &half1, &p, 1.0);
            assert!((v_h + 1.0).abs() < 1e-3, "half-paired energy {}", v_h);
            let (wh, _, gradh) = sm.hessian(&op, &half1, &p, 1.0, 1e-3);
            assert!(gradh < 1e-5, "half-paired state is stationary: gradient {}", gradh);
            assert!(wh[0] < -1e-3, "half-paired state is a saddle point: {:?}", wh);
        }
    }
}
