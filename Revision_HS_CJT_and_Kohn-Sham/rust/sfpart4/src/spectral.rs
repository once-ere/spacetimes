//! The Fourier (spectral) derivative on a periodic grid with an odd number of points, and the discrete Fourier
//! transform (Sections 15.3 and 15.4 of superfluid.md; the numbers in parentheses are those of its equations).
//!
//! THE GRID.  N points x_j = -L/2 + j L/N (j = 0 .. N-1, N odd) on a periodic interval of the length L.  A function on
//! the grid is interpolated by the trigonometric polynomial u(x) = (1/N) sum_m u^_m e^{i k_m x} with the wave numbers
//! k_m = 2 pi m / L, m = -(N-1)/2 .. (N-1)/2, and the discrete Fourier transform u^_m = sum_j u_j e^{-i k_m x_j}.
//!
//! THE DERIVATIVE.  The derivative of the interpolating polynomial at the grid points is u'_j = sum_l D_jl u_l with the
//! matrix (the Fourier differentiation matrix for odd N)
//!     D_jl = (pi/L) (-1)^(j-l) / sin(pi (j-l)/N)   (j != l),      D_jj = 0.                          (15.4)
//! D is real and antisymmetric, its eigenvectors are the Fourier modes e^{i k_m x_j} with the eigenvalues i k_m exactly,
//! so that D D is the exact second derivative of every mode (for odd N there is no unpaired Nyquist mode).  A smooth
//! function whose Fourier content lies well inside |k| < pi N/L is differentiated with an error that falls faster than
//! any power of 1/N.
use sfcore::c64::{c, C64};
use std::f64::consts::PI;

/// the periodic grid
#[derive(Clone, Debug)]
pub struct Grid1 {
    pub n: usize,
    pub len: f64,
    pub x: Vec<f64>,
    /// the wave numbers k_m in the order m = 0, 1, .., (N-1)/2, -(N-1)/2, .., -1
    pub k: Vec<f64>,
}

impl Grid1 {
    pub fn new(n: usize, len: f64) -> Grid1 {
        assert!(n % 2 == 1, "the spectral grid needs an odd number of points");
        let x = (0..n).map(|j| -0.5 * len + j as f64 * len / n as f64).collect();
        let half = (n as i64 - 1) / 2;
        let k = (0..n as i64).map(|i| if i <= half { i } else { i - n as i64 }).map(|m| 2.0 * PI * m as f64 / len).collect();
        Grid1 { n, len, x, k }
    }
    /// the matrix (15.4), stored by rows
    pub fn dmatrix(&self) -> Vec<f64> {
        let n = self.n;
        let mut d = vec![0.0; n * n];
        for j in 0..n {
            for l in 0..n {
                if j != l {
                    let dj = j as i64 - l as i64;
                    let sign = if dj.rem_euclid(2) == 0 { 1.0 } else { -1.0 };
                    d[j * n + l] = PI / self.len * sign / (PI * dj as f64 / n as f64).sin();
                }
            }
        }
        d
    }
    /// the transform u^_m = sum_j u_j e^{-i k_m x_j} (the modes in the order of `k`)
    pub fn dft(&self, u: &[f64]) -> Vec<C64> {
        self.k.iter().map(|&km| u.iter().zip(&self.x).fold(C64::ZERO, |s, (&uj, &xj)| s + c(uj * (km * xj).cos(), -uj * (km * xj).sin()))).collect()
    }
    /// the inverse transform u_j = (1/N) sum_m u^_m e^{i k_m x_j} (real part; the coefficients of a real function)
    pub fn idft(&self, uh: &[C64]) -> Vec<f64> {
        self.x.iter().map(|&xj| uh.iter().zip(&self.k).fold(0.0, |s, (z, &km)| s + z.re * (km * xj).cos() - z.im * (km * xj).sin()) / self.n as f64).collect()
    }
}

/// y = D x for a matrix stored by rows
pub fn matvec(d: &[f64], x: &[f64], y: &mut [f64]) {
    let n = x.len();
    for j in 0..n {
        let row = &d[j * n..(j + 1) * n];
        y[j] = row.iter().zip(x).map(|(a, b)| a * b).sum();
    }
}
