//! lg_tidal -- stage 7c of task C23 (folder Revision_Lord_Gravity, PROJECT spacetimes): the pair of two quasi-particles
//! bound by the zeugon falls radially into a Schwarzschild black hole; is it torn apart before it reaches the horizon?
//! The time-dependent Schroedinger equation of the pair along the fall, integrated with CVODE (decision D12 (2) of the
//! task record: "the tidal break-up of the pair, refined by the rate of tunnelling along a real fall").
//!
//! THE PAIR (stage 7a, decision D11 and entry E20 of the task record).  Two quasi-particles of mass m (the gap, 0.3664
//! Lambda_LL) and opposite zeugon charges attract each other with the potential -alpha/r, alpha = alpha_Z = 0.009748.  Their
//! relative motion is that of a hydrogen atom with the reduced mass mu = m/2.  This program uses the ATOMIC UNITS OF THE
//! PAIR: lengths in the Bohr radius a_B = 1/(mu alpha) (hbar = c = 1), energies in mu alpha^2, times in 1/(mu alpha^2).
//! Then the ground state has the energy -1/2 and the wave function e^(-r)/pi^(1/2).
//!
//! THE TIDAL FIELD ALONG THE FALL.  The centre of mass of the pair falls freely along a radial geodesic of the
//! Schwarzschild space-time of mass M (M = G M_BH, a length), from rest far away (energy per unit mass 1).  Along this
//! geodesic r(tau)^3 = (9 M/2) (tau_0 - tau)^2 exactly, tau the proper time and tau_0 the time at which r = 0.  In the
//! freely falling frame the space-time curvature acts on the relative coordinate x of the pair as the tidal potential
//!     V_tid = (mu/2) E_ij x^i x^j,   E = diag(-2 M/r^3, M/r^3, M/r^3)   (radial direction = z),
//! that is V_tid = -k (z^2 - rho^2/2) = -k r^2 P_2(cos theta) with k = mu M/r^3: a stretch along the radius and a squeeze
//! across it (the eigenvalues of E are those of the static frame; a radial boost does not change them).  In atomic units
//! of the pair, with the remaining proper time T = (tau_0 - tau) mu alpha^2,
//!     V_tid = -kappa(T) r^2 P_2(cos theta),   kappa(T) = 2/(9 T^2),                                          (T1)
//! the same for every black hole: the mass only decides WHEN the pair crosses the horizon, r = 2M, which happens at
//!     T_h = (4/3) Mt,   Mt = M mu alpha^2 (the gravitational radius in units of the atomic time of the pair).      (T2)
//! (Python and Wolfram check (T1) and (T2) exactly from the Riemann tensor.)  The classical estimate of entry E20: the
//! top of the barrier of -1/r - kappa z^2 on the axis lies at z* = (2 kappa)^(-1/3) with the height -(3/2)(2 kappa)^(1/3);
//! it falls below the level -1/2 at kappa = 1/54, which by (T1) is T = 12^(1/2) = 2 3^(1/2) = 3.4641 and by (T2) the
//! mass Mt = 3 3^(1/2)/2 = 2.598.  The questions of this stage: (i) how much does tunnelling through the barrier break
//! the pair before the barrier is gone; (ii) the field grows so fast near the end (kappa doubles when T falls by the factor
//! 2^(1/2)) that the pair may not have time to fall apart even when the barrier is gone.  Both are answered by the
//! Schroedinger equation along the fall.
//!
//! THE EQUATION.  i d psi/dT_real = H psi, H = -(1/2) Laplacian - 1/r - kappa(t) r^2 P_2(cos theta), with the real time
//! t = T_0 - T.  The wave function is expanded in the Legendre functions of even order (the initial state is the 1s
//! state, which is even under z -> -z and has no angular momentum about the z axis; the tidal potential keeps both):
//!     psi = sum_{l = 0, 2, ..., l_max} (u_l(r, t)/r) Y_l0(theta),
//!     i du_l/dt = [-(1/2) d^2/dr^2 + l(l+1)/(2 r^2) - 1/r] u_l - kappa g(r) sum_l' P_ll' u_l' - i W(r) u_l,       (T3)
//! with the coupling P_ll' = <Y_l0|P_2|Y_l'0> (non-zero for l' = l, l +- 2; closed forms in `coupling`, checked by LT-0
//! against a quadrature and in Python and Wolfram exactly), g(r) = r^2 up to r = R_f (then flattened smoothly, see below),
//! and the absorbing potential W(r) near the edge of the grid.  The radial functions are represented on the grid
//! r_j = j dr (j = 1 .. N), u(0) = u(R_max + dr) = 0, with the three-point second difference: the ground state of the grid
//! has the energy -1/2 + dr^2/8 (dr = 0.05: 3 10^-4 above -1/2).
//!   * The absorbing potential W(r) = eta ((r - R_c)/(R_max - R_c))^2 for r > R_c removes the waves that leave the region of
//!     interest; the probability it removes is counted as escaped.
//!   * Flattening: the tidal potential accelerates the escaping waves without limit (-kappa r^2); beyond the radius rf the
//!     program replaces r^2 by g(r), which joins r^2 with its value and slope at rf and becomes constant at 1.5 rf (its
//!     slope falls linearly to 0), so that the escaping waves stop accelerating.  The radius rf is 20, or less when the
//!     field is strong: rf = min(20, (40/(1.5 kappa))^(1/2)), so that the drop of the potential, 1.5 kappa rf^2, never
//!     exceeds 40 (it shrinks from T = 1.83 on, to 3.3 at T = 0.3, where the ridge of the barrier lies at 0.6).  Without
//!     this cap the escaping waves would reach energies of several hundred near the end, which the grid cannot follow
//!     and which force the integrator to steps of 10^-6.  The flattened region lies far beyond the ridge of the barrier,
//!     where the waves have escaped in the sense below, so the flattening changes nothing that is measured (checked by
//!     moving rf, the cap, the absorber and the edge in LT-5).
//! Time integration: CVODE with Newton's iteration and the banded LU solver (module cvode_band of the crate lgsundials),
//! Adams-Moulton formulas of the orders 1 and 2 (the trapezoidal rule, which conserves the norm of a Hermitian problem
//! exactly), complex error weights.  The unknowns are ordered radius by radius (for each r_j the real and imaginary parts
//! of u_0, u_2, ..., u_lmax), so that the Jacobian matrix of (T3) is banded with the bandwidth 2 n_l + 1.
//!
//! WHEN IS THE PAIR BROKEN?  Three measures, because a pair in a strong and growing field can be no longer bound
//! without being apart yet:
//!   * P_esc, beyond the barrier: in a direction theta with P_2(cos theta) > 0 the potential -1/r - kappa P_2 r^2 has a
//!     ridge; the escape region is the set of points beyond the OUTER TURNING POINT of the energy -1/2 of the ground state
//!     (where -1/r - kappa P_2 r^2 = -1/2 on the outer side of the ridge), or beyond the ridge itself when the ridge is
//!     lower than -1/2 (no barrier left);
//!         P_esc(T) = (probability in the escape region at T) + (probability absorbed by W up to T);               (T4)
//!     the absorbed probability is the integral of the rate 2 <W>, so that an error of the integration (a drift of the
//!     norm) cannot pass for a break-up;
//!   * P_exc = 1 - |<1s|psi>|^2, no longer in the ground state of the free pair (deformed, excited or unbound);
//!   * P_sep, the probability outside r = 10 (the two quasi-particles more than 10 Bohr radii apart), with the absorbed
//!     probability (everything absorbed was beyond r = 30).
//! The program also records the probability outside r = 20 and the mean squares <z^2>, <x^2> of the separation.
//!
//! WHAT THIS PROGRAM DOES:
//!   LT-0  the coupling coefficients P_ll' (closed forms) against a Gauss-Legendre quadrature of P_l P_2 P_l'.
//!   LT-1  without the tidal field the ground state of the grid stays itself: after the time 40 the norm and the
//!         population of the state are 1 within 1e-9 (the trapezoidal rule and the solver conserve the probability).
//!   LT-2  an exactly solvable case: without the Coulomb potential, a Gaussian packet psi = (pi s^2)^(-3/4) e^(-r^2/(2 s^2))
//!         in the tidal field (T1) from T = 10 to T = 1.  The Hamiltonian is quadratic, so z(t) = X1(t) z(0) + X2(t) p(0)
//!         with the classical solutions of z'' = 2 kappa z, which by (T1) are T^(4/3) and T^(-1/3), and x'' = -kappa x
//!         (T^(2/3) and T^(1/3)); hence <z^2> = (s^2/2)(X1^2 + X2^2/s^4), likewise <x^2>.  The program compares <z^2> and
//!         <x^2> computed from the partial waves with these closed forms (this tests the tidal coupling, the angular
//!         expansion and the time integration together).
//!   LT-3  the static field: the tidal field is switched on smoothly to a constant kappa and held; the probability
//!         inside the escape region boundary then decays as e^(-Gamma t).  The rates Gamma for kappa = 0.0125 and 0.0185
//!         are fitted, the decay is checked to be exponential (the rates of the two halves of the window agree), and the
//!         rates are written for the comparison with the independent complex rotation of python/lg_tidal.py.
//!   LT-4  the fall: the ground state at T_0 = 30 (kappa = 2.5e-4), evolved to T = 0.3; the three measures every 0.01 in
//!         T; the values of T_h (and Mt = 3 T_h/4) at which each reaches 0.001, 0.01, 0.1 and 1/2 (if it does).  The
//!         check: the norm is conserved up to the absorbed probability within 1e-5 (the drift of the integration).
//!   LT-5  convergence: the fall again with a finer grid (dr = 0.035), more partial waves (l_max = 56), a larger box
//!         (flattening from 30 with the cap 80, absorber from 45, edge at 65) and an earlier start (T_0 = 40): P_esc and
//!         P_exc change by less than 5e-3 (absolute, LT-5a), P_sep by less than 10 % where it exceeds 1e-5 for T >= 1
//!         (LT-5b; below T = 1 the flattening of the main computation lies inside r = 10, and the spread of P_sep over
//!         the five computations is reported).  The five series are written to lg_tidal_convergence.csv.
//! The quasi-static comparison (the static rates of the complex rotation integrated along the fall) is made by
//! python/lg_tidal.py (YT-7).
//!
//! Output (folder data/): hck_lg_tidal_checks.txt (the report), lg_tidal_fall.csv (the fall, every 0.01 in T),
//! lg_tidal_convergence.csv (the five computations of LT-5, every 0.1 in T), lg_tidal_static.csv (LT-3),
//! lg_tidal_gauss.csv (LT-2), lg_tidal_snapshots.csv (the density of the pair at six moments, for the figure),
//! lg_tidal_values.csv (the numbers of the text).
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_tidal   (about 40 minutes)
use lgcore::report::{data_dir, f, write_csv, Report};
use lgsundials::cvode::{CvodeSettings, Weights};
use lgsundials::cvode_band::{describe_band, integrate_band, BandWriter};
use std::f64::consts::PI;
use std::rc::Rc;
use std::time::Instant;

// ------------------------------------------------------------------------------------------------------------------
// the angular coupling
// ------------------------------------------------------------------------------------------------------------------

/// <Y_l0 | P_2(cos theta) | Y_l'0> for l' = l (l(l+1)/((2l-1)(2l+3))) and l' = l + 2
/// ((3/2)(l+1)(l+2)/((2l+3) ((2l+1)(2l+5))^(1/2))); zero otherwise (and symmetric).
fn coupling(l: usize, lp: usize) -> f64 {
    let (a, b) = if l <= lp { (l, lp) } else { (lp, l) };
    let lf = a as f64;
    if a == b {
        lf * (lf + 1.0) / ((2.0 * lf - 1.0) * (2.0 * lf + 3.0))
    } else if b == a + 2 {
        1.5 * (lf + 1.0) * (lf + 2.0) / ((2.0 * lf + 3.0) * ((2.0 * lf + 1.0) * (2.0 * lf + 5.0)).sqrt())
    } else {
        0.0
    }
}

/// the Legendre polynomials P_0(x) .. P_n(x) by their recurrence
fn legendre_all(n: usize, x: f64) -> Vec<f64> {
    let mut p = vec![0.0; n + 1];
    p[0] = 1.0;
    if n >= 1 {
        p[1] = x;
    }
    for k in 1..n {
        let kf = k as f64;
        p[k + 1] = ((2.0 * kf + 1.0) * x * p[k] - kf * p[k - 1]) / (kf + 1.0);
    }
    p
}

/// the nodes and weights of the Gauss-Legendre quadrature of n points on [-1, 1] (Newton's method on P_n)
fn gauss_legendre(n: usize) -> (Vec<f64>, Vec<f64>) {
    let mut x = vec![0.0; n];
    let mut w = vec![0.0; n];
    for i in 0..n {
        let mut z = (PI * (i as f64 + 0.75) / (n as f64 + 0.5)).cos();
        for _ in 0..100 {
            let p = legendre_all(n, z);
            let dp = n as f64 * (z * p[n] - p[n - 1]) / (z * z - 1.0);
            let dz = p[n] / dp;
            z -= dz;
            if dz.abs() < 1e-15 {
                break;
            }
        }
        let p = legendre_all(n, z);
        let dp = n as f64 * (z * p[n] - p[n - 1]) / (z * z - 1.0);
        x[i] = z;
        w[i] = 2.0 / ((1.0 - z * z) * dp * dp);
    }
    (x, w)
}

// ------------------------------------------------------------------------------------------------------------------
// the model: grid, potentials, right side and Jacobian of (T3)
// ------------------------------------------------------------------------------------------------------------------

#[derive(Clone)]
struct Model {
    dr: f64,
    nr: usize,
    nl: usize, // partial waves l = 0, 2, ..., 2 (nl - 1)
    lmax: usize,
    rmax: f64,
    rf0: f64,  // flattening of r^2 starts at rf = min(rf0, (vcap/(1.5 kappa))^(1/2)) ...
    vcap: f64, // ... and ends at 1.5 rf, where kappa g = 1.5 kappa rf^2 <= vcap
    rc: f64,   // the absorbing potential starts here
    eta: f64,  // its strength at rmax
    zc: f64,   // the strength of the Coulomb potential (1, or 0 for the test LT-2)
    r: Vec<f64>,
    w: Vec<f64>,   // the absorbing potential
    dg: Vec<f64>,  // the diagonal of the field-free H: 1/dr^2 + l(l+1)/(2 r^2) - zc/r, index j*nl + k
    off: f64,      // -1/(2 dr^2)
    pd: Vec<f64>,  // P_ll for l = 2k
    po: Vec<f64>,  // P_{l,l+2} for l = 2k
    jmin: Vec<usize>, // the partial wave l = 2k is kept only at the grid points j >= jmin[k] (see ECAP)
}

/// THE CENTRIFUGAL CUT.  Near the origin the centrifugal energy l(l+1)/(2 r^2) of the high partial waves is enormous
/// (3 10^5 for l = 40 at r = 0.05); the true u_l there are of the order r^(l+1), negligible, but on the grid such points
/// carry oscillations so fast that the error test of CVODE forces steps of 10^-5.  The program therefore keeps the
/// partial wave l only where l(l+1)/(2 r^2) <= ECAP (for l = 40 from r = 2.86 on, for l = 2 from r = 0.17); inside, u_l is
/// held at 0.  Every energy of the computation (the bound pair near -1/2, the escaping waves at most about 130 far from
/// the origin) is far below ECAP in that inner region (the local kinetic energy there is about 1/r <= 6), so the cut
/// removes only a classically forbidden region where the wave function is smaller than (r/r_l)^(l+1).
const ECAP: f64 = 100.0;

impl Model {
    fn new(dr: f64, rmax: f64, lmax: usize, rf0: f64, vcap: f64, rc: f64, eta: f64, zc: f64) -> Model {
        let nr = (rmax / dr).round() as usize;
        let nl = lmax / 2 + 1;
        let r: Vec<f64> = (1..=nr).map(|j| j as f64 * dr).collect();
        let w: Vec<f64> = r
            .iter()
            .map(|&x| if x > rc { eta * ((x - rc) / (rmax - rc)).powi(2) } else { 0.0 })
            .collect();
        let mut dg = vec![0.0; nr * nl];
        for j in 0..nr {
            for k in 0..nl {
                let l = (2 * k) as f64;
                dg[j * nl + k] = 1.0 / (dr * dr) + l * (l + 1.0) / (2.0 * r[j] * r[j]) - zc / r[j];
            }
        }
        let pd = (0..nl).map(|k| coupling(2 * k, 2 * k)).collect();
        let po = (0..nl).map(|k| coupling(2 * k, 2 * k + 2)).collect();
        let jmin = (0..nl)
            .map(|k| {
                let l = (2 * k) as f64;
                (0..nr).find(|&j| l * (l + 1.0) / (2.0 * r[j] * r[j]) <= ECAP).unwrap_or(nr)
            })
            .collect();
        Model { dr, nr, nl, lmax, rmax, rf0, vcap, rc, eta, zc, r, w, dg, off: -0.5 / (dr * dr), pd, po, jmin }
    }
    /// the flattening radius at the tidal strength kappa: rf = min(rf0, (vcap/(1.5 kappa))^(1/2))
    fn rflat(&self, kappa: f64) -> f64 {
        if kappa > 0.0 {
            self.rf0.min((self.vcap / (1.5 * kappa)).sqrt())
        } else {
            self.rf0
        }
    }
    /// g(r): r^2 up to rf, then r^2 joined with its value and slope at rf, its slope falling linearly to 0 at 1.5 rf
    /// (g = rf^2 + 2 rf d - 2 d^2, d = r - rf), then the constant 1.5 rf^2
    #[inline]
    fn gval(r: f64, rf: f64) -> f64 {
        if r <= rf {
            r * r
        } else if r <= 1.5 * rf {
            let d = r - rf;
            rf * rf + 2.0 * rf * d - 2.0 * d * d
        } else {
            1.5 * rf * rf
        }
    }
    fn n(&self) -> usize {
        2 * self.nr * self.nl
    }
    fn band(&self) -> usize {
        2 * self.nl + 1
    }
    fn describe(&self) -> String {
        format!(
            "grid dr = {}, R_max = {}, l = 0, 2, ..., {} ({} partial waves), r^2 flattened from rf = min({}, ({}/(1.5 kappa))^(1/2)) to 1.5 rf, absorber from {} (strength {}), Coulomb strength {}",
            self.dr,
            self.rmax,
            self.lmax,
            self.nl,
            self.rf0,
            self.vcap,
            self.rc,
            self.eta,
            self.zc
        )
    }
    /// (H u) for the complex vector u (re, im interleaved), with the tidal strength kappa; written into hu
    fn apply_h(&self, kappa: f64, u: &[f64], hu: &mut [f64]) {
        let nl = self.nl;
        let rf = self.rflat(kappa);
        for j in 0..self.nr {
            let kg = kappa * Model::gval(self.r[j], rf);
            for k in 0..nl {
                let i = j * nl + k;
                if j < self.jmin[k] {
                    hu[2 * i] = 0.0;
                    hu[2 * i + 1] = 0.0;
                    continue;
                }
                for c in 0..2 {
                    let mut s = (self.dg[i] - kg * self.pd[k]) * u[2 * i + c];
                    if j > 0 {
                        s += self.off * u[2 * (i - nl) + c];
                    }
                    if j + 1 < self.nr {
                        s += self.off * u[2 * (i + nl) + c];
                    }
                    if k > 0 {
                        s -= kg * self.po[k - 1] * u[2 * (i - 1) + c];
                    }
                    if k + 1 < nl {
                        s -= kg * self.po[k] * u[2 * (i + 1) + c];
                    }
                    hu[2 * i + c] = s;
                }
            }
        }
    }
    /// the right side of (T3): du/dt = -i H u - W u, i.e. da/dt = H b - W a, db/dt = -H a - W b (u = a + i b)
    fn rhs(&self, kappa: f64, y: &[f64], dy: &mut [f64], hu: &mut [f64]) {
        self.apply_h(kappa, y, hu);
        let nl = self.nl;
        for j in 0..self.nr {
            let wj = self.w[j];
            for k in 0..nl {
                let i = j * nl + k;
                if j < self.jmin[k] {
                    dy[2 * i] = 0.0;
                    dy[2 * i + 1] = 0.0;
                    continue;
                }
                dy[2 * i] = hu[2 * i + 1] - wj * y[2 * i];
                dy[2 * i + 1] = -hu[2 * i] - wj * y[2 * i + 1];
            }
        }
    }
    /// the Jacobian matrix of the right side (it does not depend on y)
    fn jacobian(&self, kappa: f64, wr: &mut BandWriter) {
        let nl = self.nl;
        let rf = self.rflat(kappa);
        for j in 0..self.nr {
            let kg = kappa * Model::gval(self.r[j], rf);
            let wj = self.w[j];
            for k in 0..nl {
                let i = j * nl + k;
                if j < self.jmin[k] {
                    continue; // the row of a held point is zero: its update is zero
                }
                let (ia, ib) = (2 * i, 2 * i + 1);
                let hd = self.dg[i] - kg * self.pd[k];
                wr.set(ia, ia, -wj);
                wr.set(ib, ib, -wj);
                wr.set(ia, ib, hd);
                wr.set(ib, ia, -hd);
                if j > 0 {
                    let i2 = i - nl;
                    wr.set(ia, 2 * i2 + 1, self.off);
                    wr.set(ib, 2 * i2, -self.off);
                }
                if j + 1 < self.nr {
                    let i2 = i + nl;
                    wr.set(ia, 2 * i2 + 1, self.off);
                    wr.set(ib, 2 * i2, -self.off);
                }
                if k > 0 {
                    let c = -kg * self.po[k - 1];
                    wr.set(ia, 2 * (i - 1) + 1, c);
                    wr.set(ib, 2 * (i - 1), -c);
                }
                if k + 1 < nl {
                    let c = -kg * self.po[k];
                    wr.set(ia, 2 * (i + 1) + 1, c);
                    wr.set(ib, 2 * (i + 1), -c);
                }
            }
        }
    }
}

/// the ground state of the s wave on the grid (u(r) of the lowest eigenvalue of the field-free radial Hamiltonian,
/// l = 0), by inverse iteration with the shift -0.6 (tridiagonal solves); returns (u normalized with dr sum u^2 = 1, E)
fn ground_state(m: &Model) -> (Vec<f64>, f64) {
    let n = m.nr;
    let shift = -0.6;
    let diag: Vec<f64> = (0..n).map(|j| m.dg[j * m.nl] - shift).collect();
    let off = m.off;
    let mut u: Vec<f64> = m.r.iter().map(|&x| x * (-x).exp()).collect();
    let mut e = 0.0;
    for _ in 0..200 {
        // solve (H - shift) v = u by the Thomas algorithm
        let mut c = vec![0.0; n];
        let mut d = vec![0.0; n];
        c[0] = off / diag[0];
        d[0] = u[0] / diag[0];
        for j in 1..n {
            let den = diag[j] - off * c[j - 1];
            c[j] = off / den;
            d[j] = (u[j] - off * d[j - 1]) / den;
        }
        let mut v = vec![0.0; n];
        v[n - 1] = d[n - 1];
        for j in (0..n - 1).rev() {
            v[j] = d[j] - c[j] * v[j + 1];
        }
        let nrm = (m.dr * v.iter().map(|x| x * x).sum::<f64>()).sqrt();
        for x in v.iter_mut() {
            *x /= nrm;
        }
        // the Rayleigh quotient of the field-free Hamiltonian
        let mut num = 0.0;
        for j in 0..n {
            let mut hv = m.dg[j * m.nl] * v[j];
            if j > 0 {
                hv += off * v[j - 1];
            }
            if j + 1 < n {
                hv += off * v[j + 1];
            }
            num += v[j] * hv;
        }
        let enew = num * m.dr;
        let done = (enew - e).abs() < 1e-15;
        e = enew;
        u = v;
        if done {
            break;
        }
    }
    (u, e)
}

/// a state with u_0 = phi (real) and the other partial waves zero, as the vector of unknowns
fn state_from_s(m: &Model, phi: &[f64]) -> Vec<f64> {
    let mut y = vec![0.0; m.n()];
    for j in 0..m.nr {
        y[2 * (j * m.nl)] = phi[j];
    }
    y
}

// ------------------------------------------------------------------------------------------------------------------
// the measures (T4)
// ------------------------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, Default)]
struct Meas {
    t: f64,
    kappa: f64,
    norm: f64,       // probability left on the grid
    p_region: f64,   // probability in the escape region
    p_esc: f64,      // (T4)
    out10: f64,      // probability outside r = 10 (plus the absorbed probability)
    out20: f64,      // probability outside r = 20 (plus the absorbed probability)
    not1s: f64,      // 1 - |<1s|psi>|^2
    r2: f64,         // <r^2>
    r2p2: f64,       // <r^2 P_2>
    absrate: f64,    // the rate of absorption 2 <W> (d norm/dt = -2 <W> exactly for (T3))
    absorbed: f64,   // its integral from the start (filled in by `run`, trapezoidal rule over the output times)
    drift: f64,      // (1 - norm) - absorbed: the loss of norm that is not absorption (the error of the integration)
}

/// the boundary of the escape region in the direction with P_2(cos theta) = p2 > 0 at the tidal strength kappa: the outer
/// turning point of the energy -1/2 (the outer root of a r^3 - r/2 + 1 = 0, a = kappa p2) when the ridge of the
/// potential is above -1/2 (a < 1/54), else the ridge r* = (2 a)^(-1/3); infinity when there is no ridge
fn escape_boundary(kappa: f64, p2: f64) -> f64 {
    let a = kappa * p2;
    if a <= 0.0 {
        return f64::INFINITY;
    }
    let rstar = (2.0 * a).powf(-1.0 / 3.0);
    if a >= 1.0 / 54.0 {
        return rstar;
    }
    let fun = |r: f64| a * r * r * r - 0.5 * r + 1.0;
    let mut lo = rstar;
    let mut hi = rstar.max(1.0 / (2.0 * a).sqrt()) * 2.0 + 10.0;
    while fun(hi) < 0.0 {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if fun(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo < 1e-13 * hi {
            break;
        }
    }
    0.5 * (lo + hi)
}

struct Angular {
    x: Vec<f64>,       // Gauss-Legendre nodes in cos theta
    wt: Vec<f64>,      // weights times 2 pi
    ylm: Vec<Vec<f64>>, // Y_l0(x_i) for l = 2k
    p2: Vec<f64>,
}

impl Angular {
    fn new(m: &Model) -> Angular {
        let nth = 2 * m.lmax + 8;
        let (x, w) = gauss_legendre(nth);
        let ylm = x
            .iter()
            .map(|&xi| {
                let p = legendre_all(m.lmax + 2, xi);
                (0..m.nl).map(|k| ((4.0 * k as f64 + 1.0) / (4.0 * PI)).sqrt() * p[2 * k]).collect()
            })
            .collect();
        let p2 = x.iter().map(|&xi| 1.5 * xi * xi - 0.5).collect();
        let wt = w.iter().map(|&wi| 2.0 * PI * wi).collect();
        Angular { x, wt, ylm, p2 }
    }
}

fn measures(m: &Model, ang: &Angular, phi0: &[f64], t: f64, kappa: f64, y: &[f64]) -> Meas {
    let nl = m.nl;
    let mut norm = 0.0;
    let mut in10 = 0.0;
    let mut in20 = 0.0;
    let mut r2 = 0.0;
    let mut r2p2 = 0.0;
    let mut absr = 0.0;
    let (mut ovr, mut ovi) = (0.0, 0.0);
    for j in 0..m.nr {
        let mut s = 0.0;
        let mut sp = 0.0;
        for k in 0..nl {
            let i = j * nl + k;
            let (a, b) = (y[2 * i], y[2 * i + 1]);
            s += a * a + b * b;
            sp += m.pd[k] * (a * a + b * b);
            if k + 1 < nl {
                let (a2, b2) = (y[2 * (i + 1)], y[2 * (i + 1) + 1]);
                sp += 2.0 * m.po[k] * (a * a2 + b * b2);
            }
        }
        norm += s;
        absr += m.w[j] * s;
        if m.r[j] < 10.0 {
            in10 += s;
        }
        if m.r[j] < 20.0 {
            in20 += s;
        }
        r2 += m.r[j] * m.r[j] * s;
        r2p2 += m.r[j] * m.r[j] * sp;
        ovr += phi0[j] * y[2 * (j * nl)];
        ovi += phi0[j] * y[2 * (j * nl) + 1];
    }
    let dr = m.dr;
    norm *= dr;
    in10 *= dr;
    in20 *= dr;
    r2 *= dr;
    r2p2 *= dr;
    ovr *= dr;
    ovi *= dr;
    // the escape region, on the angular grid
    let bounds: Vec<f64> = ang.p2.iter().map(|&p| escape_boundary(kappa, p)).collect();
    let mut preg = 0.0;
    for (i, _) in ang.x.iter().enumerate() {
        if !bounds[i].is_finite() {
            continue;
        }
        let jmin = ((bounds[i] / dr).ceil() as usize).max(1) - 1; // first grid point with r_j >= bound
        let yl = &ang.ylm[i];
        let mut acc = 0.0;
        for j in jmin..m.nr {
            let (mut pr, mut pi) = (0.0, 0.0);
            for k in 0..nl {
                let idx = j * nl + k;
                pr += y[2 * idx] * yl[k];
                pi += y[2 * idx + 1] * yl[k];
            }
            acc += pr * pr + pi * pi;
        }
        preg += ang.wt[i] * acc * dr;
    }
    // p_esc, absorbed and drift are completed by `run` once the absorbed probability is integrated
    Meas {
        t,
        kappa,
        norm,
        p_region: preg,
        p_esc: f64::NAN,
        out10: 1.0 - in10,
        out20: 1.0 - in20,
        not1s: 1.0 - (ovr * ovr + ovi * ovi),
        r2,
        r2p2,
        absrate: 2.0 * absr * dr,
        absorbed: f64::NAN,
        drift: f64::NAN,
    }
}

/// the density |psi|^2 at the point (z, rho), by linear interpolation of the u_l in r
fn density(m: &Model, y: &[f64], z: f64, rho: f64) -> f64 {
    let r = (z * z + rho * rho).sqrt();
    if r < m.dr || r >= m.rmax {
        return 0.0;
    }
    let x = z / r;
    let p = legendre_all(m.lmax + 2, x);
    let jf = r / m.dr - 1.0;
    let j0 = jf.floor() as usize;
    let th = jf - j0 as f64;
    let (mut pr, mut pi) = (0.0, 0.0);
    for k in 0..m.nl {
        let yl = ((4.0 * k as f64 + 1.0) / (4.0 * PI)).sqrt() * p[2 * k];
        let i0 = j0 * m.nl + k;
        let i1 = ((j0 + 1).min(m.nr - 1)) * m.nl + k;
        let a = (1.0 - th) * y[2 * i0] + th * y[2 * i1];
        let b = (1.0 - th) * y[2 * i0 + 1] + th * y[2 * i1 + 1];
        pr += a * yl;
        pi += b * yl;
    }
    (pr * pr + pi * pi) / (r * r)
}

// ------------------------------------------------------------------------------------------------------------------
// one integration
// ------------------------------------------------------------------------------------------------------------------

struct RunOut {
    meas: Vec<Meas>,
    snaps: Vec<(f64, Vec<f64>)>, // (output time, state) at the requested snapshot indices
    stats: String,
    method: String,
    seconds: f64,
}

/// integrates (T3) from t = 0 with the tidal strength kappa_of(t), the initial vector y0, and records the measures at
/// the output times touts; the states at the output indices in `snap_at` are kept
fn run(
    m: &Model,
    kappa_of: Rc<dyn Fn(f64) -> f64>,
    y0: &[f64],
    phi0: &[f64],
    touts: &[f64],
    snap_at: &[usize],
    rtol: f64,
    atol: f64,
) -> Result<RunOut, String> {
    let mut set = CvodeSettings::adams(rtol, atol, Weights::Complex);
    set.max_order = 2;
    let mrc = Rc::new(m.clone());
    let (m1, k1) = (mrc.clone(), kappa_of.clone());
    let mut hu = vec![0.0; m.n()];
    let f = Box::new(move |t: f64, y: &[f64], dy: &mut [f64]| -> i32 {
        m1.rhs(k1(t), y, dy, &mut hu);
        0
    });
    let (m2, k2) = (mrc.clone(), kappa_of.clone());
    let jac = Box::new(move |t: f64, _y: &[f64], w: &mut BandWriter| -> i32 {
        m2.jacobian(k2(t), w);
        0
    });
    let ang = Angular::new(m);
    let mut meas = Vec::with_capacity(touts.len());
    let mut snaps = Vec::new();
    let t0 = Instant::now();
    let method = describe_band(&set, m.band(), m.band());
    let st = integrate_band(y0, 0.0, touts, &set, f, jac, m.band(), m.band(), |i, t, y| {
        meas.push(measures(m, &ang, phi0, t, kappa_of(t), y));
        if snap_at.contains(&i) {
            snaps.push((t, y.to_vec()));
        }
        true
    })?;
    // the absorbed probability: the integral of the rate 2 <W> over the output times (trapezoidal rule); the escape
    // probability (T4) counts the absorbed probability, not the whole loss of norm, so that an error of the integration
    // (the drift) cannot pass for a break-up
    let mut acc = 0.0;
    for i in 0..meas.len() {
        if i > 0 {
            acc += 0.5 * (meas[i].absrate + meas[i - 1].absrate) * (meas[i].t - meas[i - 1].t);
        }
        meas[i].absorbed = acc;
        meas[i].p_esc = meas[i].p_region + acc;
        meas[i].drift = (1.0 - meas[i].norm) - acc;
    }
    Ok(RunOut { meas, snaps, stats: st.describe(), method, seconds: t0.elapsed().as_secs_f64() })
}

/// kappa along the fall (T1), as a function of the real time t = T0 - T
fn fall_kappa(t0_fall: f64) -> Rc<dyn Fn(f64) -> f64> {
    Rc::new(move |t: f64| {
        let tt = t0_fall - t;
        2.0 / (9.0 * tt * tt)
    })
}

/// a smooth switching on: kappa_f s(t/tr) with s(x) = 6x^5 - 15x^4 + 10x^3 (s' and s'' vanish at both ends), then constant
fn ramp_kappa(kf: f64, tr: f64) -> Rc<dyn Fn(f64) -> f64> {
    Rc::new(move |t: f64| {
        let x = (t / tr).clamp(0.0, 1.0);
        kf * x * x * x * (10.0 - 15.0 * x + 6.0 * x * x)
    })
}

/// the closed forms of LT-2: <z^2> and <x^2> of the Gaussian of width s released at T0 in the field (T1), at T
fn gauss_exact(s: f64, t0: f64, tt: f64) -> (f64, f64) {
    // z'' = 2 kappa z: z = A T^(4/3) + B T^(-1/3); X1(T0) = 1, X1'(T0) = 0; X2(T0) = 0, dX2/dt = 1 (dX2/dT = -1)
    let q = tt / t0;
    let x1 = 0.2 * q.powf(4.0 / 3.0) + 0.8 * q.powf(-1.0 / 3.0);
    let x2 = 0.6 * t0 * (q.powf(-1.0 / 3.0) - q.powf(4.0 / 3.0));
    // x'' = -kappa x: x = C T^(2/3) + D T^(1/3), dx/dT = (2/3) C T^(-1/3) + (1/3) D T^(-2/3):
    //   Y1: C T0^(2/3) + D T0^(1/3) = 1, (2/3) C T0^(-1/3) + (1/3) D T0^(-2/3) = 0  ->  D = -2 C T0^(1/3), C = -T0^(-2/3)
    //       so Y1 = -q^(2/3) + 2 q^(1/3)
    //   Y2: value 0 and dY2/dT = -1:  C T0^(2/3) + D T0^(1/3) = 0, (2/3) C T0^(-1/3) + (1/3) D T0^(-2/3) = -1
    //       ->  D = -C T0^(1/3), (2/3 - 1/3) C T0^(-1/3) = -1  ->  C = -3 T0^(1/3), D = 3 T0^(2/3)
    //       so Y2 = 3 T0 (q^(1/3) - q^(2/3))
    let y1 = -q.powf(2.0 / 3.0) + 2.0 * q.powf(1.0 / 3.0);
    let y2 = 3.0 * t0 * (q.powf(1.0 / 3.0) - q.powf(2.0 / 3.0));
    let z2 = 0.5 * s * s * (x1 * x1 + x2 * x2 / s.powi(4));
    let xx = 0.5 * s * s * (y1 * y1 + y2 * y2 / s.powi(4));
    (z2, xx)
}

/// the static rate of tunnelling in the limit of a weak field (derived in python/lg_tidal.py, check YT-6):
/// Gamma = 8 kappa^(-1/2) exp(-pi/(2 (2 kappa)^(1/2))) / sinh(pi/2^(1/2)), both exits z > 0 and z < 0 together
fn gamma_asymptotic(kappa: f64) -> f64 {
    8.0 / kappa.sqrt() * (-PI / (2.0 * (2.0 * kappa).sqrt())).exp() / (PI / 2.0f64.sqrt()).sinh()
}

/// the value of T at which the decreasing record crosses the level p (linear interpolation in T; NaN if never)
fn crossing(fall: &[Meas], t0_fall: f64, level: f64, pick: impl Fn(&Meas) -> f64) -> f64 {
    for w in fall.windows(2) {
        let (a, b) = (pick(&w[0]), pick(&w[1]));
        if (a - level) * (b - level) <= 0.0 && a != b {
            let ta = t0_fall - w[0].t;
            let tb = t0_fall - w[1].t;
            return ta + (level - a) / (b - a) * (tb - ta);
        }
    }
    f64::NAN
}

fn main() {
    // an optional argument --parts=0,1,2 runs only those checks (for the development); the default runs all
    let parts: Option<Vec<String>> = std::env::args()
        .find(|a| a.starts_with("--parts="))
        .map(|a| a["--parts=".len()..].split(',').map(|s| s.to_string()).collect());
    let want = |id: &str| parts.as_ref().map_or(true, |p| p.iter().any(|q| q == id));
    let ddir = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "PROJECT spacetimes, Revision_Lord_Gravity, stage 7c (task C23, decision D12): the pair bound by the zeugon in the tidal field of a black hole along a radial fall; checks of the program lg_tidal (Rust, CVODE)",
    );
    let t_all = Instant::now();

    // ---------------- LT-0: the coupling coefficients against a quadrature ----------------
    if want("0") {
        let (x, w) = gauss_legendre(80);
        let mut dev: f64 = 0.0;
        for l in (0..=60).step_by(2) {
            for lp in [l, l + 2] {
                let mut s = 0.0;
                for (xi, wi) in x.iter().zip(w.iter()) {
                    let p = legendre_all(lp + 2, *xi);
                    s += wi * p[l] * (1.5 * xi * xi - 0.5) * p[lp];
                }
                // <Y_l0|P2|Y_l'0> = (1/2) ((2l+1)(2l'+1))^(1/2) int P_l P_2 P_l' dx
                let q = 0.5 * (((2 * l + 1) * (2 * lp + 1)) as f64).sqrt() * s;
                dev = dev.max((q - coupling(l, lp)).abs());
            }
        }
        rep.add(
            "LT-0",
            "the coupling <Y_l0|P_2|Y_l'0> of the closed forms equals (1/2)((2l+1)(2l'+1))^(1/2) times the Gauss-Legendre quadrature (80 points) of P_l P_2 P_l' for l = 0, 2, ..., 60 and l' = l, l + 2: largest difference",
            dev,
            1e-13,
        );
    }

    // the model of the main computation
    let main_model = Model::new(0.05, 50.0, 40, 20.0, 40.0, 30.0, 10.0, 1.0);
    let (phi0, e0) = ground_state(&main_model);
    rep.note(&format!(
        "main model: {}; {} real unknowns, bandwidth {}; ground state of the grid E = {:.10} (-1/2 + dr^2/8 = {:.10})",
        main_model.describe(),
        main_model.n(),
        main_model.band(),
        e0,
        -0.5 + main_model.dr * main_model.dr / 8.0
    ));

    // ---------------- LT-1: no field, the ground state stays itself ----------------
    if want("1") {
        let y0 = state_from_s(&main_model, &phi0);
        let touts: Vec<f64> = (0..=4).map(|i| 10.0 * i as f64).collect();
        let kz: Rc<dyn Fn(f64) -> f64> = Rc::new(|_t: f64| 0.0);
        let out = run(&main_model, kz, &y0, &phi0, &touts, &[], 1e-8, 1e-12).expect("LT-1 integration");
        let last = out.meas.last().unwrap();
        let dev = (1.0 - last.norm).abs().max(last.not1s.abs());
        rep.add(
            "LT-1",
            &format!(
                "without the tidal field the ground state of the grid keeps its norm and stays the ground state up to t = 40: largest of |1 - norm| and 1 - |<1s|psi>|^2 ({}; {} s)",
                out.stats, out.seconds as i64
            ),
            dev,
            1e-9,
        );
    }

    // ---------------- LT-2: the free Gaussian in the field (T1), exact ----------------
    if want("2") {
        // the same packet on two grids: the deviation from the closed forms is the error O(dr^2) of the three-point
        // second difference (its dispersion E(k) = (1 - cos k dr)/dr^2 = k^2/2 - k^4 dr^2/24 slows the spreading by about
        // (dr^2/3) <k^4>/<k^2>, 5 10^-4 of the spreading for dr = 0.05 and s = 2), so it must fall by 4 when dr is halved
        let mut gauss_rows: Vec<Vec<String>> = Vec::new();
        let s = 2.0;
        let t0g = 10.0;
        let tts: Vec<f64> = (0..=90).map(|i| 10.0 - 0.1 * i as f64).collect(); // T = 10 .. 1
        let touts: Vec<f64> = tts.iter().map(|&tt| t0g - tt).collect();
        let mut devs = Vec::new();
        let mut detail = String::new();
        for &drg in &[0.05f64, 0.025] {
            // no flattening (rf = 40 to 60) and the absorber far out (r = 45 to 60), where the packet has no probability
            let mg = Model::new(drg, 60.0, 40, 40.0, 1e9, 45.0, 10.0, 0.0);
            let norm0 = (PI * s * s).powf(-0.75) * (4.0 * PI).sqrt();
            let phi: Vec<f64> = mg.r.iter().map(|&r| norm0 * r * (-r * r / (2.0 * s * s)).exp()).collect();
            let nrm: f64 = (mg.dr * phi.iter().map(|x| x * x).sum::<f64>()).sqrt();
            let y0 = state_from_s(&mg, &phi);
            let out = run(&mg, fall_kappa(t0g), &y0, &phi, &touts, &[], 1e-9, 1e-10).expect("LT-2 integration");
            let mut dev: f64 = 0.0;
            let mut dev_end: f64 = 0.0;
            for (mm, &tt) in out.meas.iter().zip(tts.iter()) {
                let z2 = (mm.r2 + 2.0 * mm.r2p2) / 3.0;
                let x2 = (mm.r2 - mm.r2p2) / 3.0;
                let (z2e, x2e) = gauss_exact(s, t0g, tt);
                let d = ((z2 - z2e) / z2e).abs().max(((x2 - x2e) / x2e).abs());
                if tt >= 1.5 - 1e-9 {
                    dev = dev.max(d);
                } else {
                    dev_end = dev_end.max(d);
                }
                gauss_rows.push(vec![f(drg), f(tt), f(z2), f(z2e), f(x2), f(x2e), f(mm.norm)]);
            }
            devs.push(dev);
            detail.push_str(&format!(
                "dr = {}: largest relative difference {:.3e} for T >= 1.5 ({:.3e} for 1.5 > T >= 1; norm of the discretized packet at the start {:.12}; {}; {} s). ",
                drg, dev, dev_end, nrm, out.stats, out.seconds as i64
            ));
        }
        let (z2e, x2e) = gauss_exact(s, t0g, 1.0);
        rep.add(
            "LT-2a",
            &format!(
                "without the Coulomb potential a Gaussian packet of width s = 2 released at T = 10 in the field (T1) has, from T = 10 to T = 1.5, <z^2> and <x^2> equal to the closed forms (s^2/2)(X1^2 + X2^2/s^4) (at T = 1: <z^2> = {:.4}, <x^2> = {:.4}): largest relative difference on the grid dr = 0.025 (below T = 1.5 the velocity of the packet grows along z in proportion to z, which puts probability at its edges into high radial momenta and high partial waves, and the difference no longer falls as dr^2; the values down to T = 1 are given in brackets). {}",
                z2e, x2e, detail
            ),
            devs[1],
            6e-4,
        );
        let ratio = devs[0] / devs[1];
        rep.add(
            "LT-2b",
            &format!(
                "the difference of LT-2a (T >= 1.5) is the error of second order of the grid: halving dr divides it by {:.3} (expected 4): |ratio/4 - 1|",
                ratio
            ),
            (ratio / 4.0 - 1.0).abs(),
            0.1,
        );
        write_csv(
            &ddir.join("lg_tidal_gauss.csv"),
            &["dr", "T", "z2_computed", "z2_exact", "x2_computed", "x2_exact", "norm"],
            &gauss_rows,
        );
    }

    // ---------------- LT-3: static fields, the rates of tunnelling ----------------
    if want("3") {
        let mut static_rows: Vec<Vec<String>> = Vec::new();
        let ms = Model::new(0.05, 40.0, 32, 15.0, 40.0, 23.0, 10.0, 1.0);
        let (phis, es) = ground_state(&ms);
        rep.note(&format!("static model: {}; ground state E = {:.10}", ms.describe(), es));
        let mut all_ok = true;
        let mut detail = String::new();
        for &(kf, thold) in &[(0.0125f64, 320.0f64), (0.0185, 200.0)] {
            let tr = 40.0;
            let touts: Vec<f64> = (0..=((tr + thold) as usize)).map(|i| i as f64).collect();
            let y0 = state_from_s(&ms, &phis);
            let out = run(&ms, ramp_kappa(kf, tr), &y0, &phis, &touts, &[], 1e-8, 1e-10).expect("LT-3 integration");
            // the probability inside the escape region boundary: norm - p_region
            let ts: Vec<f64> = out.meas.iter().map(|q| q.t).collect();
            let lp: Vec<f64> = out.meas.iter().map(|q| (q.norm - q.p_region).ln()).collect();
            let fit = |a: f64, b: f64| -> f64 {
                let idx: Vec<usize> = (0..ts.len()).filter(|&i| ts[i] >= a && ts[i] <= b).collect();
                let n = idx.len() as f64;
                let mx = idx.iter().map(|&i| ts[i]).sum::<f64>() / n;
                let my = idx.iter().map(|&i| lp[i]).sum::<f64>() / n;
                let sxy: f64 = idx.iter().map(|&i| (ts[i] - mx) * (lp[i] - my)).sum();
                let sxx: f64 = idx.iter().map(|&i| (ts[i] - mx) * (ts[i] - mx)).sum();
                -sxy / sxx
            };
            let ta = tr + 60.0;
            let tb = tr + thold;
            let g_all = fit(ta, tb);
            let g1 = fit(ta, 0.5 * (ta + tb));
            let g2 = fit(0.5 * (ta + tb), tb);
            let rel = ((g1 - g2) / g_all).abs();
            all_ok &= rel <= 0.01;
            detail.push_str(&format!(
                "kappa = {}: Gamma = {:.5e} (halves {:.5e}, {:.5e}; relative difference {:.2e}); the weak-field formula gives {:.5e}; {}; {} s. ",
                kf, g_all, g1, g2, rel, gamma_asymptotic(kf), out.stats, out.seconds as i64
            ));
            static_rows.push(vec![f(kf), f(g_all), f(g1), f(g2), f(gamma_asymptotic(kf)), f(ms.dr), ms.lmax.to_string()]);
        }
        rep.add_bool(
            "LT-3",
            "in a static field (switched on smoothly within the time 40) the probability inside the boundary of the escape region decays exponentially: the rates fitted on the two halves of the window (from 60 after the switching on to its end) agree within 1 %; the rates are compared with the complex rotation in python/lg_tidal.py (YT-4)",
            all_ok,
            &detail,
        );
        write_csv(
            &ddir.join("lg_tidal_static.csv"),
            &["kappa", "Gamma_fit", "Gamma_first_half", "Gamma_second_half", "Gamma_weak_field_formula", "dr", "lmax"],
            &static_rows,
        );
    }
    if !want("4") {
        let (np, n) = rep.counts();
        println!("lg_tidal (parts only, no report written): {} of {} checks pass", np, n);
        return;
    }

    // ---------------- LT-4: the fall ----------------
    let t0_fall: f64 = 30.0;
    let tmin: f64 = 0.3;
    let dtt: f64 = 0.01;
    let n_out = ((t0_fall - tmin) / dtt).round() as usize;
    let tts: Vec<f64> = (0..=n_out).map(|i| t0_fall - dtt * i as f64).collect();
    let touts: Vec<f64> = tts.iter().map(|&tt| t0_fall - tt).collect();
    let snap_ts = [4.0, 2.0, 1.5, 1.0, 0.7, 0.5];
    let snap_idx: Vec<usize> = snap_ts.iter().map(|&s| ((t0_fall - s) / dtt).round() as usize).collect();
    let y0 = state_from_s(&main_model, &phi0);
    let main = run(&main_model, fall_kappa(t0_fall), &y0, &phi0, &touts, &snap_idx, 1e-8, 1e-10).expect("fall");
    rep.note(&format!("the fall (main model): {}; {}; {} s", main.method, main.stats, main.seconds as i64));

    // ---------------- the three measures at the crossing of the horizon, T_h = (4/3) Mt ----------------
    // P_esc (T4): beyond the barrier; P_exc = 1 - |<1s|psi>|^2: no longer in the ground state; P_sep: separated by more
    // than 10 Bohr radii (the probability outside r = 10, which includes the absorbed probability)
    let measures_list: Vec<(&str, fn(&Meas) -> f64)> = vec![
        ("P_esc", |q: &Meas| q.p_esc),
        ("P_exc", |q: &Meas| q.not1s),
        ("P_sep", |q: &Meas| q.out10),
    ];
    let levels = [0.001, 0.01, 0.1, 0.5];
    let mut cross_text = Vec::new();
    let mut cross_vals: Vec<(String, f64)> = Vec::new();
    for (name, pick) in &measures_list {
        for &lv in &levels {
            let th = crossing(&main.meas, t0_fall, lv, pick);
            cross_vals.push((format!("{}_{}", name, lv), th));
            if th.is_finite() {
                cross_text.push(format!("{} = {}: T_h = {:.4}, Mt = {:.4}", name, lv, th, 0.75 * th));
            } else {
                cross_text.push(format!("{} = {}: not reached by T = {}", name, lv, tmin));
            }
        }
    }
    let t_half = crossing(&main.meas, t0_fall, 0.5, |q: &Meas| q.p_esc);
    let drift_max = main.meas.iter().map(|q| q.drift.abs()).fold(0.0, f64::max);
    let last = main.meas.last().unwrap();
    rep.add(
        "LT-4",
        &format!(
            "along the fall from T = {} to T = {} the norm is conserved up to the absorbed probability: largest |drift| = |(1 - norm) - absorbed| (the error of the integration; the probabilities reported are far larger). At T = {}: P_esc = {:.4e}, P_exc = {:.4e}, P_sep = {:.4e} (outside r = 20: {:.3e}; absorbed {:.3e}). Crossings: {}",
            t0_fall,
            tmin,
            tmin,
            last.p_esc,
            last.not1s,
            last.out10,
            last.out20,
            last.absorbed,
            cross_text.join("; ")
        ),
        drift_max,
        1e-5,
    );

    // ---------------- LT-5: convergence ----------------
    let mut vals_extra: Vec<Vec<String>> = Vec::new();
    if want("5") {
        let variants: Vec<(&str, Model, f64)> = vec![
            ("finer grid dr = 0.035", Model::new(0.035, 50.0, 40, 20.0, 40.0, 30.0, 10.0, 1.0), t0_fall),
            ("more partial waves l_max = 56", Model::new(0.05, 50.0, 56, 20.0, 40.0, 30.0, 10.0, 1.0), t0_fall),
            ("larger box and flattening (rf0 = 30, cap 80, absorber from 45, edge at 65)", Model::new(0.05, 65.0, 40, 30.0, 80.0, 45.0, 10.0, 1.0), t0_fall),
            ("earlier start T_0 = 40", main_model.clone(), 40.0),
        ];
        let mut det_a = String::new();
        let mut det_b = String::new();
        let mut worst_a: f64 = 0.0;
        let mut worst_b: f64 = 0.0;
        // the three measures of the main computation and of every variant every 0.1 in T, for the record
        let tgrid: Vec<f64> = (0..=((t0_fall - tmin) / 0.1).round() as usize).map(|i| t0_fall - 0.1 * i as f64).collect();
        let pick_main = |tt: f64| -> &Meas { &main.meas[((t0_fall - tt) / dtt).round() as usize] };
        let mut conv_cols: Vec<Vec<f64>> = vec![
            tgrid.clone(),
            tgrid.iter().map(|&tt| pick_main(tt).p_esc).collect(),
            tgrid.iter().map(|&tt| pick_main(tt).not1s).collect(),
            tgrid.iter().map(|&tt| pick_main(tt).out10).collect(),
        ];
        let mut sep_end: Vec<f64> = vec![main.meas.last().unwrap().out10];
        for (name, mv, t0v) in variants {
            let (phv, _) = ground_state(&mv);
            let yv = state_from_s(&mv, &phv);
            let nv = ((t0v - tmin) / dtt).round() as usize;
            let ttv: Vec<f64> = (0..=nv).map(|i| t0v - dtt * i as f64).collect();
            let tov: Vec<f64> = ttv.iter().map(|&tt| t0v - tt).collect();
            let out = run(&mv, fall_kappa(t0v), &yv, &phv, &tov, &[], 1e-8, 1e-10).expect("variant");
            // compare at the common values of T
            let (mut da, mut db): (f64, f64) = (0.0, 0.0);
            for (q, &tt) in out.meas.iter().zip(ttv.iter()) {
                let im = ((t0_fall - tt) / dtt).round() as isize;
                if im < 0 || im as usize >= main.meas.len() {
                    continue;
                }
                let qm = &main.meas[im as usize];
                da = da.max((q.p_esc - qm.p_esc).abs()).max((q.not1s - qm.not1s).abs());
                // P_sep is compared where the flattening of the main computation lies beyond r = 10 (T >= 1: rf >= 10.9)
                if qm.out10 >= 1e-5 && tt >= 1.0 - 1e-9 {
                    db = db.max(((q.out10 - qm.out10) / qm.out10).abs());
                }
            }
            let pick_v = |tt: f64| -> &Meas { &out.meas[((t0v - tt) / dtt).round() as usize] };
            conv_cols.push(tgrid.iter().map(|&tt| pick_v(tt).p_esc).collect());
            conv_cols.push(tgrid.iter().map(|&tt| pick_v(tt).not1s).collect());
            conv_cols.push(tgrid.iter().map(|&tt| pick_v(tt).out10).collect());
            sep_end.push(out.meas.last().unwrap().out10);
            worst_a = worst_a.max(da);
            worst_b = worst_b.max(db);
            let th_v = crossing(&out.meas, t0v, 0.5, |q: &Meas| q.p_esc);
            det_a.push_str(&format!(
                "{}: {:.2e}, T_h(P_esc = 1/2) = {:.4} (main {:.4}); {}; {} s. ",
                name, da, th_v, t_half, out.stats, out.seconds as i64
            ));
            det_b.push_str(&format!("{}: {:.2e}. ", name, db));
        }
        rep.add(
            "LT-5a",
            &format!("the probabilities P_esc and P_exc along the fall do not depend on the numerical parameters: largest absolute difference between the main computation and each variant, over all common T. {}", det_a),
            worst_a,
            5e-3,
        );
        let sep_max = sep_end.iter().cloned().fold(0.0, f64::max);
        let sep_min = sep_end.iter().cloned().fold(1.0, f64::min);
        rep.add(
            "LT-5b",
            &format!("the probability P_sep of a separation beyond 10 Bohr radii does not depend on the numerical parameters: largest relative difference where P_sep >= 1e-5, for T >= 1 (below T = 1 the flattening of the tidal potential of the main computation lies inside r = 10, so that P_sep there depends on it: at T = {} it lies between {:.2e} and {:.2e} in the five computations). {}", tmin, sep_min, sep_max, det_b),
            worst_b,
            0.1,
        );
        let mut header: Vec<String> = vec!["T_remaining".into(), "main_P_esc".into(), "main_P_exc".into(), "main_P_sep".into()];
        for tag in ["finer_grid", "more_l", "larger_box", "earlier_start"] {
            for m in ["P_esc", "P_exc", "P_sep"] {
                header.push(format!("{}_{}", tag, m));
            }
        }
        let hdr: Vec<&str> = header.iter().map(|s| s.as_str()).collect();
        let crows: Vec<Vec<String>> = (0..tgrid.len()).map(|i| conv_cols.iter().map(|c| f(c[i])).collect()).collect();
        write_csv(&ddir.join("lg_tidal_convergence.csv"), &hdr, &crows);
        vals_extra.push(vec!["P_sep_end_min_over_variants".into(), f(sep_min)]);
        vals_extra.push(vec!["P_sep_end_max_over_variants".into(), f(sep_max)]);
    }

    // ---------------- the classical estimate, for comparison ----------------
    let mt_classical = 1.5 * 3.0f64.sqrt();
    rep.note(&format!(
        "classical estimate of entry E20: the barrier disappears at kappa = 1/54, T = 12^(1/2) = {:.4}, Mt = 3 3^(1/2)/2 = {:.4}; along the real fall P_esc is {:.3e} there, and reaches 1/2 only at T_h = {:.4} (Mt = {:.4}); the quasi-static comparison is made in python/lg_tidal.py (YT-7) and in the fragments",
        12f64.sqrt(),
        mt_classical,
        main.meas[((t0_fall - 12f64.sqrt()) / dtt).round() as usize].p_esc,
        t_half,
        0.75 * t_half
    ));
    let mut fall_rows: Vec<Vec<String>> = Vec::new();
    for q in main.meas.iter() {
        let tt = t0_fall - q.t;
        fall_rows.push(vec![
            f(tt),
            f(0.75 * tt),
            f(q.kappa),
            f(q.p_esc),
            f(q.p_region),
            f(q.absorbed),
            f(q.drift),
            f(q.out10),
            f(q.out20),
            f(q.not1s),
            f((q.r2 + 2.0 * q.r2p2) / 3.0),
            f((q.r2 - q.r2p2) / 3.0),
        ]);
    }
    write_csv(
        &ddir.join("lg_tidal_fall.csv"),
        &[
            "T_remaining",
            "Mt_if_horizon_here",
            "kappa",
            "P_esc",
            "P_escape_region",
            "P_absorbed",
            "norm_drift",
            "P_sep_outside_r10",
            "P_outside_r20",
            "P_exc_not_1s",
            "z2_mean",
            "x2_mean",
        ],
        &fall_rows,
    );

    // snapshots of the density for the figure
    let mut snap_rows: Vec<Vec<String>> = Vec::new();
    for (t, y) in &main.snaps {
        let tt = t0_fall - t;
        for iz in 0..=120 {
            let z = -15.0 + 0.25 * iz as f64;
            for ir in 0..=40 {
                let rho = 0.25 * ir as f64;
                snap_rows.push(vec![format!("{:.2}", tt), format!("{:.2}", z), format!("{:.2}", rho), f(density(&main_model, y, z, rho))]);
            }
        }
    }
    write_csv(&ddir.join("lg_tidal_snapshots.csv"), &["T_remaining", "z", "rho", "density"], &snap_rows);

    // values
    let mut vals: Vec<Vec<String>> = vec![
        vec!["E_grid_ground_state".into(), f(e0)],
        vec!["T_classical".into(), f(12f64.sqrt())],
        vec!["Mt_classical".into(), f(mt_classical)],
        vec!["T0_fall".into(), f(t0_fall)],
        vec!["T_min".into(), f(tmin)],
        vec!["drift_max".into(), f(drift_max)],
    ];
    for (name, th) in &cross_vals {
        vals.push(vec![format!("T_h_{}", name), f(*th)]);
        vals.push(vec![format!("Mt_{}", name), f(0.75 * *th)]);
    }
    for &tt in &[6.0, 5.0, 4.0, 3.4641016151377544, 3.0, 2.0, 1.5, 1.0, 0.7, 0.5, 0.3] {
        let i = ((t0_fall - tt) / dtt).round() as usize;
        if i < main.meas.len() {
            let q = &main.meas[i];
            vals.push(vec![format!("P_esc_at_T{:.4}", tt), f(q.p_esc)]);
            vals.push(vec![format!("P_exc_at_T{:.4}", tt), f(q.not1s)]);
            vals.push(vec![format!("P_sep_at_T{:.4}", tt), f(q.out10)]);
            vals.push(vec![format!("z2_at_T{:.4}", tt), f((q.r2 + 2.0 * q.r2p2) / 3.0)]);
            vals.push(vec![format!("x2_at_T{:.4}", tt), f((q.r2 - q.r2p2) / 3.0)]);
        }
    }
    vals.extend(vals_extra);
    write_csv(&ddir.join("lg_tidal_values.csv"), &["quantity", "value"], &vals);

    rep.note(&format!("total run time {} s", t_all.elapsed().as_secs()));
    if parts.is_none() {
        rep.write(&ddir.join("hck_lg_tidal_checks.txt"));
    }
    let (np, n) = rep.counts();
    println!("lg_tidal: {} of {} checks pass", np, n);
}
