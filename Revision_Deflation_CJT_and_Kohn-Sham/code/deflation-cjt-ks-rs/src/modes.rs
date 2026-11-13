//! Solutions of the two Dirac equations on the deflating spacetimes, mode by mode.
//!
//! A mode is a solution proportional to exp(i p_a x^a + i k_t x^t): p_a (a = 0..3)
//! are the wavenumbers along the flat space coordinates x0..x3, k_t (t = 5, 6, 7) the
//! wavenumbers along the deflating coordinates.  Both are constants of the motion,
//! because the metric depends on x4 only.  What changes with x4 is the *physical*
//! wavenumber along a deflating coordinate,
//!
//!     q_t(x4) = k_t / b_t(x4) = (k_t / const_t) exp(+rate_t x4),      k_d(x4)^2 = q_5^2 + q_6^2 + q_7^2,
//!
//! which grows exponentially (rate_t = g55, g66, g77 for t = 5, 6, 7).  The weighted
//! field psihat = sqrt(b_5 b_6 b_7) psi obeys (checks 71d-D2, 44d-D2 of `curved.rs`)
//!
//!   7+1:   i d psihat/dx4 = h(x4) psihat,   h = m beta + p_a alpha^a + sum_t q_t(x4) alpha^t,   h^2 = (E_perp^2 + k_d^2) 1,
//!   4+4:     d psihat/dx4 = K(x4) psihat,   K = K_perp + sum_t q_t(x4) N_t,                         K^2 = (k_d^2 - E_perp^2) 1,
//!          K_perp = -m gamma^4 + i p_a gamma^4 gamma^a  (anti-Hermitian),  N_t = i gamma^4 gamma^t  (Hermitian, N_t^2 = 1),
//!
//! with E_perp^2 = m^2 + p_0^2 + p_1^2 + p_2^2 + p_3^2.
//!
//! 7+1: PAIR CREATION.  h(x4) is Hermitian: the evolution is unitary, but the positive-
//! and negative-energy states of early times are mixed by the time the mode has become
//! energetic.  The expectation number of particles found at late times in a mode that
//! was empty at early times is the square of the Bogoliubov coefficient.  For one
//! non-zero k_t the equation is solved by Bessel functions of the order
//! 1/2 + i E_perp/rate_t and the result is exactly
//!
//!     n = 1 / ( exp(2 pi E_perp / rate_t) + 1 ),
//!
//! a Fermi-Dirac distribution in E_perp with the temperature rate_t/(2 pi).  This module
//! computes n numerically for any mode and any rates, and compares.
//!
//! 4+4: EXTINCTION.  K(x4) is not anti-Hermitian when k_t != 0, and beyond the value
//! of x4 at which k_d = E_perp its eigenvalues are real, +-sqrt(k_d^2 - E_perp^2): of the sixteen
//! solutions eight grow and eight decay faster than any exponential.  The bounded
//! solutions (the decaying ones) are computed here by integrating backwards in x4.
//! For one non-zero k_t they are, exactly,
//!
//!     psihat = sqrt(u_d) K_{1/2 + i E_perp/rate_t}(u_d) w - sqrt(u_d) K_{1/2 - i E_perp/rate_t}(u_d) N_t w,   u_d = q_t/rate_t,
//!
//! with the modified Bessel function K and any w of positive energy (i K_perp w = E_perp w).
//! They are standing waves at early times and fall off as exp(-s) at late times.
//!
//! NUMERICAL METHOD.  The equations are integrated with the commutator-free Magnus
//! method of order four (two exponentials per step, nodes c = 1/2 -+ sqrt(3)/6, weights
//! a1 = 1/4 + sqrt(3)/6, a2 = 1/4 - sqrt(3)/6):
//!
//!     U(t + dt, t) = exp( dt (a2 A1 + a1 A2) ) exp( dt (a1 A1 + a2 A2) ),   A_i = A(t + c_i dt).
//!
//! Every exponent is a real combination of mutually anticommuting matrices whose
//! squares are multiples of the unit matrix, so that its square is a known multiple s of
//! the unit matrix and the exponential is known in closed form
//! (cos and sin, or cosh and sinh, of dt sqrt|s|).  No matrix function is approximated.

use crate::metric::Sig;
use spacetimes::complex::C64;
use spacetimes::gamma::Gammas;
use spacetimes::linalg::{eigh, CMat};

/// The deflating background in floating point.
#[derive(Clone, Copy, Debug)]
pub struct Background {
    /// the rates g55, g66, g77
    pub rate: [f64; 3],
    /// the constants const5, const6, const7
    pub cst: [f64; 3],
}

impl Background {
    /// Scale factor b_t(x4), t = 5 + i.
    pub fn b(&self, i: usize, x4: f64) -> f64 {
        self.cst[i] * (-self.rate[i] * x4).exp()
    }
    /// The physical wavenumbers q_t = k_t / b_t(x4).
    pub fn q(&self, kappa: &[f64; 3], x4: f64) -> [f64; 3] {
        [kappa[0] / self.b(0, x4), kappa[1] / self.b(1, x4), kappa[2] / self.b(2, x4)]
    }
    /// k_d(x4) = sqrt(q_5^2 + q_6^2 + q_7^2).
    pub fn q_norm(&self, kappa: &[f64; 3], x4: f64) -> f64 {
        let q = self.q(kappa, x4);
        (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt()
    }
    /// The value of x4 at which k_d equals `target` (k_d increases monotonically with x4).
    pub fn x4_where_q_is(&self, kappa: &[f64; 3], target: f64) -> f64 {
        let (mut lo, mut hi) = (-1.0, 1.0);
        while self.q_norm(kappa, lo) > target {
            lo *= 2.0;
        }
        while self.q_norm(kappa, hi) < target {
            hi *= 2.0;
        }
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if self.q_norm(kappa, mid) < target {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        0.5 * (lo + hi)
    }
}

/// One mode: mass, wavenumbers along x0..x3, wavenumbers along x5, x6, x7.
#[derive(Clone, Copy, Debug)]
pub struct Mode {
    pub m: f64,
    pub p: [f64; 4],
    pub kappa: [f64; 3],
}

impl Mode {
    /// E_perp = sqrt(m^2 + |p|^2): the energy of the mode before the deflation acts on it.
    pub fn eps(&self) -> f64 {
        (self.m * self.m + self.p.iter().map(|x| x * x).sum::<f64>()).sqrt()
    }
}

/// A 16 x cols block of complex numbers (a set of `cols` spinors), stored row by row.
#[derive(Clone, Debug)]
pub struct Block {
    pub cols: usize,
    pub a: Vec<C64>,
}

impl Block {
    pub fn zeros(cols: usize) -> Block {
        Block { cols, a: vec![C64::ZERO; 16 * cols] }
    }
    /// The columns j0..j0+cols of the 16x16 matrix v.
    pub fn from_columns(v: &CMat, j0: usize, cols: usize) -> Block {
        let mut b = Block::zeros(cols);
        for i in 0..16 {
            for j in 0..cols {
                b.a[i * cols + j] = v.get(i, j0 + j);
            }
        }
        b
    }
    /// The cols x cols Gram matrix  self^dagger M other  (M = unit matrix if `m` is None).
    pub fn gram(&self, m: Option<&CMat>, other: &Block) -> CMat {
        let mo = match m {
            Some(mm) => mat_block(mm, other),
            None => other.clone(),
        };
        assert_eq!(self.cols, other.cols);
        let n = self.cols;
        let mut g = CMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                let mut s = C64::ZERO;
                for k in 0..16 {
                    s += self.a[k * n + i].conj() * mo.a[k * n + j];
                }
                g.set(i, j, s);
            }
        }
        g
    }
    /// Frobenius norm.
    pub fn norm(&self) -> f64 {
        self.a.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()
    }
}

/// The product M Y of a 16x16 matrix with a block.
pub fn mat_block(m: &CMat, y: &Block) -> Block {
    let n = y.cols;
    let mut out = Block::zeros(n);
    for i in 0..16 {
        for k in 0..16 {
            let x = m.a[i * 16 + k];
            if x.re == 0.0 && x.im == 0.0 {
                continue;
            }
            for j in 0..n {
                out.a[i * n + j] += x * y.a[k * n + j];
            }
        }
    }
    out
}

/// c1 * A + c2 * B for complex 16x16 matrices and real coefficients.
fn lin2(a: &CMat, c1: f64, b: &CMat, c2: f64) -> CMat {
    let mut out = CMat::zeros(16);
    for i in 0..256 {
        out.a[i] = a.a[i] * c1 + b.a[i] * c2;
    }
    out
}

/// A real integer matrix of the main project as a complex matrix.
fn cmat_from(m: &spacetimes::imat::IMat) -> CMat {
    let mut c = CMat::zeros(16);
    for i in 0..16 {
        for j in 0..16 {
            c.set(i, j, C64::new(m.get(i, j) as f64, 0.0));
        }
    }
    c
}

/// Y <- f1 Y + f2 (A Y) with complex factors: the closed-form exponential applied to a block.
fn apply_exponential(a: &CMat, f1: C64, f2: C64, y: &mut Block) {
    let ay = mat_block(a, y);
    for i in 0..y.a.len() {
        y.a[i] = f1 * y.a[i] + f2 * ay.a[i];
    }
}

/// Nodes and weights of the commutator-free Magnus method of order four.
const CF4_C1: f64 = 0.5 - 0.288_675_134_594_812_9; // 1/2 - sqrt(3)/6
const CF4_C2: f64 = 0.5 + 0.288_675_134_594_812_9;
const CF4_A1: f64 = 0.25 + 0.288_675_134_594_812_9; // 1/4 + sqrt(3)/6
const CF4_A2: f64 = 0.25 - 0.288_675_134_594_812_9;

// =====================================================================================
// 7+1
// =====================================================================================

/// The matrices of the 7+1 Hamiltonian.
pub struct Dirac71 {
    pub beta: CMat,
    /// alpha[A] = gamma^4 gamma^A (the entry A = 4 is not used)
    pub alpha: Vec<CMat>,
}

/// Result of the pair-creation calculation for one mode.
#[derive(Clone, Debug)]
pub struct Creation {
    pub eps: f64,
    /// smallest, largest and mean eigenvalue of the 8x8 matrix of created particles
    pub n_min: f64,
    pub n_max: f64,
    pub n_mean: f64,
    /// deviation of the evolved states from orthonormality
    pub unitarity: f64,
    pub steps: usize,
    /// (x4, k_d/E_perp, mean adiabatic particle number) along the way
    pub history: Vec<[f64; 3]>,
}

impl Dirac71 {
    pub fn new(gm: &Gammas) -> Dirac71 {
        let g: Vec<CMat> = gm.gamma1671.iter().map(cmat_from).collect();
        let alpha: Vec<CMat> = (0..8).map(|a| g[4].mul(&g[a])).collect();
        Dirac71 { beta: g[4].clone(), alpha }
    }

    /// h_perp = m beta + sum_a p_a alpha^a (a = 0..3): the Hamiltonian without the deflating momenta.
    pub fn h_perp(&self, mode: &Mode) -> CMat {
        let mut h = self.beta.scale(mode.m);
        for a in 0..4 {
            if mode.p[a] != 0.0 {
                h = h.add(&self.alpha[a].scale(mode.p[a]));
            }
        }
        h
    }

    /// c0 h_perp + sum_t c_t alpha^t.
    fn comb(&self, h_perp: &CMat, c0: f64, c: &[f64; 3]) -> CMat {
        let mut h = h_perp.scale(c0);
        for i in 0..3 {
            if c[i] != 0.0 {
                h = h.add(&self.alpha[5 + i].scale(c[i]));
            }
        }
        h
    }

    /// The Hamiltonian h(x4) of the weighted field.
    pub fn h(&self, mode: &Mode, bg: &Background, x4: f64) -> CMat {
        self.comb(&self.h_perp(mode), 1.0, &bg.q(&mode.kappa, x4))
    }

    /// The projector on the negative-energy states in the first superadiabatic basis.
    ///
    /// With n = h/omega (n^2 = 1) the states that follow the slow change of h to first
    /// order in (rate/omega) are the eigenstates of
    ///     H1 = h - (i/2) (dn/dx4) n ,       H1^2 = (omega^2 + |dn/dx4|^2/4) 1 ,
    /// (for a two-level system h = omega (cos th sigma_3 + sin th sigma_1) this is
    /// h - (dth/dx4)/2 sigma_2, the Hamiltonian in the frame that rotates with h).
    /// Projecting on them instead of on the eigenstates of h removes the oscillation of
    /// relative size (rate/omega) from the particle number, so that its limit is reached
    /// at moderate x4.  Returns (P_negative, |dn/dx4| / (2 omega)).
    pub fn negative_projector(&self, mode: &Mode, bg: &Background, x4: f64) -> (CMat, f64) {
        let eps = mode.eps();
        let q = bg.q(&mode.kappa, x4);
        let qq = q[0] * q[0] + q[1] * q[1] + q[2] * q[2];
        let omega = (eps * eps + qq).sqrt();
        let h_perp = self.h_perp(mode);
        let h = self.comb(&h_perp, 1.0, &q);
        // dh/dx4 = sum rate_t q_t alpha^t,  d omega/dx4 = sum rate_t q_t^2 / omega
        let dq = [bg.rate[0] * q[0], bg.rate[1] * q[1], bg.rate[2] * q[2]];
        let domega = (bg.rate[0] * q[0] * q[0] + bg.rate[1] * q[1] * q[1] + bg.rate[2] * q[2] * q[2]) / omega;
        // dn/dx4 = (dh/dx4)/omega - h (d omega/dx4)/omega^2
        let dn = self.comb(&h_perp, -domega / (omega * omega), &[dq[0] / omega - q[0] * domega / (omega * omega), dq[1] / omega - q[1] * domega / (omega * omega), dq[2] / omega - q[2] * domega / (omega * omega)]);
        let n = h.scale(1.0 / omega);
        // H1 = h - (i/2) dn n
        let dnn = dn.mul(&n);
        let mut h1 = CMat::zeros(16);
        for i in 0..256 {
            h1.a[i] = h.a[i] - C64::new(0.0, 0.5) * dnn.a[i];
        }
        // |dn|^2 from the trace of dn^2
        let dn2 = dn.mul(&dn).trace().re / 16.0;
        let norm = (omega * omega + 0.25 * dn2).sqrt();
        let mut p = CMat::identity(16);
        for i in 0..256 {
            p.a[i] = (p.a[i] - h1.a[i] * (1.0 / norm)) * 0.5;
        }
        (p, 0.5 * dn2.max(0.0).sqrt() / omega)
    }

    /// One step of the commutator-free Magnus method for i dY/dx4 = h(x4) Y.
    fn step(&self, h_perp: &CMat, eps: f64, mode: &Mode, bg: &Background, x4: f64, dt: f64, y: &mut Block) {
        let q1 = bg.q(&mode.kappa, x4 + CF4_C1 * dt);
        let q2 = bg.q(&mode.kappa, x4 + CF4_C2 * dt);
        for (wa, wb) in [(CF4_A1, CF4_A2), (CF4_A2, CF4_A1)] {
            // exponent: -i dt A,  A = (wa + wb) h_perp + sum_t (wa q1_t + wb q2_t) alpha^t,  A^2 = s 1
            let c0 = wa + wb;
            let c = [wa * q1[0] + wb * q2[0], wa * q1[1] + wb * q2[1], wa * q1[2] + wb * q2[2]];
            let s = c0 * c0 * eps * eps + c[0] * c[0] + c[1] * c[1] + c[2] * c[2];
            let a = self.comb(h_perp, c0, &c);
            let rt = s.sqrt();
            let th = dt * rt;
            // exp(-i dt A) = cos(th) - i sin(th)/rt A
            let f2 = if rt > 0.0 { C64::new(0.0, -th.sin() / rt) } else { C64::new(0.0, -dt) };
            apply_exponential(&a, C64::new(th.cos(), 0.0), f2, y);
        }
    }

    /// Evolve the block y from x4_a to x4_b (x4_b > x4_a).  The step is chosen so that the
    /// phase omega dt does not exceed `phase` and rate dt does not exceed `slow`.
    /// `sample` is called after every step with (x4, y).  Returns the number of steps.
    pub fn evolve<F: FnMut(f64, &Block)>(&self, mode: &Mode, bg: &Background, x4_a: f64, x4_b: f64, y: &mut Block, phase: f64, slow: f64, mut sample: F) -> usize {
        let h_perp = self.h_perp(mode);
        let eps = mode.eps();
        let rmax = bg.rate.iter().cloned().fold(0.0, f64::max);
        let mut t = x4_a;
        let mut steps = 0;
        while t < x4_b {
            // omega at the end of the step bounds it during the step (omega increases)
            let mut dt = slow / rmax;
            for _ in 0..3 {
                let q = bg.q_norm(&mode.kappa, t + dt);
                let omega = (eps * eps + q * q).sqrt();
                dt = dt.min(phase / omega);
            }
            if t + dt > x4_b {
                dt = x4_b - t;
            }
            self.step(&h_perp, eps, mode, bg, t, dt, y);
            t += dt;
            steps += 1;
            sample(t, y);
            if dt <= 0.0 {
                break;
            }
        }
        steps
    }

    /// The number of particles that the deflation creates in a mode that starts in the
    /// vacuum: the eight positive-energy states at an early time (k_d = `q_start` E_perp) are
    /// evolved to a late time (k_d = `q_end` E_perp) and projected on the negative-energy
    /// states there, in the first superadiabatic basis at both ends.
    pub fn creation(&self, mode: &Mode, bg: &Background, q_start: f64, q_end: f64, phase: f64, slow: f64, with_history: bool) -> Creation {
        let eps = mode.eps();
        let t0 = bg.x4_where_q_is(&mode.kappa, q_start * eps);
        let t1 = bg.x4_where_q_is(&mode.kappa, q_end * eps);
        // initial states: the eight eigenvectors of (1 - P_negative) at t0
        let (pneg0, _) = self.negative_projector(mode, bg, t0);
        let (w, v) = eigh(&pneg0); // eigenvalues 0 (positive energy, 8 times) and 1
        debug_assert!(w[7] < 1e-8 && w[8] > 1.0 - 1e-8);
        let mut y = Block::from_columns(&v, 0, 8);
        let mut history: Vec<[f64; 3]> = Vec::new();
        let mut next = t0;
        let dt_hist = (t1 - t0) / 600.0;
        let steps = self.evolve(mode, bg, t0, t1, &mut y, phase, slow, |t, yy| {
            if with_history && t >= next {
                let (pn, _) = self.negative_projector(mode, bg, t);
                let g = yy.gram(Some(&pn), yy);
                history.push([t, bg.q_norm(&mode.kappa, t) / eps, g.trace().re / 8.0]);
                next = t + dt_hist;
            }
        });
        let (pneg1, _) = self.negative_projector(mode, bg, t1);
        let nmat = y.gram(Some(&pneg1), &y);
        let (nw, _) = eigh(&nmat);
        let gram = y.gram(None, &y);
        let mut unit: f64 = 0.0;
        for i in 0..8 {
            for j in 0..8 {
                let d = gram.get(i, j) - if i == j { C64::ONE } else { C64::ZERO };
                unit = unit.max(d.abs());
            }
        }
        Creation { eps, n_min: nw[0], n_max: nw[7], n_mean: nw.iter().sum::<f64>() / 8.0, unitarity: unit, steps, history }
    }
}

/// The exact number of created particles for one non-zero k_t: 1/(exp(2 pi E_perp/rate) + 1).
pub fn fermi_dirac_creation(eps: f64, rate: f64) -> f64 {
    1.0 / ((2.0 * std::f64::consts::PI * eps / rate).exp() + 1.0)
}

// =====================================================================================
// 4+4
// =====================================================================================

/// The matrices of the 4+4 evolution generator.
pub struct Dirac44 {
    pub g4: CMat,
    /// g4g[A] = gamma^4 gamma^A
    pub g4g: Vec<CMat>,
    /// N_t = i gamma^4 gamma^t (t = 5, 6, 7), Hermitian with square one
    pub n: Vec<CMat>,
    /// W = i C^- gamma^4: the matrix of the covariant charge
    pub w: CMat,
}

/// Result of the calculation of the bounded solutions of one mode.
#[derive(Clone, Debug)]
pub struct Extinction {
    pub eps: f64,
    /// x4 at which k_d = E_perp (the turning value) and x4 at which the backward integration started
    pub x4_turn: f64,
    pub x4_late: f64,
    /// exponent S = integral of sqrt(k_d^2 - E_perp^2) dx4 from the turning value to x4_late (quadrature)
    pub s_wkb: f64,
    /// smallest and largest logarithm of the amplification of the eight bounded solutions between x4_late and the early time
    pub log_amp_min: f64,
    pub log_amp_max: f64,
    /// relative deviation of the Gram matrices of the positive- and negative-energy parts at early times (0 = standing waves)
    pub standing_wave_dev: f64,
    /// |psihat^dagger W psihat| / |psihat^dagger psihat| of the bounded solutions at early times (0 = no covariant charge)
    pub charge_ratio: f64,
    /// (x4, k_d/E_perp, |psihat|^2 normalized to its maximum) of one bounded solution
    pub profile: Vec<[f64; 3]>,
    pub steps: usize,
}

impl Dirac44 {
    pub fn new(gm: &Gammas) -> Dirac44 {
        let g: Vec<CMat> = gm.gamma1644.iter().map(cmat_from).collect();
        let g4g: Vec<CMat> = (0..8).map(|a| g[4].mul(&g[a])).collect();
        let mut n: Vec<CMat> = Vec::new();
        for t in 5..8 {
            let mut x = CMat::zeros(16);
            for i in 0..256 {
                x.a[i] = C64::I * g4g[t].a[i];
            }
            n.push(x);
        }
        let cm = cmat_from(&gm.sigma16);
        let cg = cm.mul(&g[4]);
        let mut w = CMat::zeros(16);
        for i in 0..256 {
            w.a[i] = C64::I * cg.a[i];
        }
        Dirac44 { g4: g[4].clone(), g4g, n, w }
    }

    /// K_perp = -m gamma^4 + i sum_a p_a gamma^4 gamma^a: anti-Hermitian, K_perp^2 = -E_perp^2.
    pub fn k_perp(&self, mode: &Mode) -> CMat {
        let mut k = self.g4.scale(-mode.m);
        for a in 0..4 {
            if mode.p[a] != 0.0 {
                for i in 0..256 {
                    k.a[i] += C64::new(0.0, mode.p[a]) * self.g4g[a].a[i];
                }
            }
        }
        k
    }

    /// c0 K_perp + sum_t c_t N_t.
    fn comb(&self, k_perp: &CMat, c0: f64, c: &[f64; 3]) -> CMat {
        let mut k = k_perp.scale(c0);
        for i in 0..3 {
            if c[i] != 0.0 {
                k = lin2(&k, 1.0, &self.n[i], c[i]);
            }
        }
        k
    }

    /// The generator K(x4) of the weighted field.
    pub fn k(&self, mode: &Mode, bg: &Background, x4: f64) -> CMat {
        self.comb(&self.k_perp(mode), 1.0, &bg.q(&mode.kappa, x4))
    }

    /// One step (dt may be negative) of the commutator-free Magnus method for dY/dx4 = K(x4) Y.
    fn step(&self, k_perp: &CMat, eps: f64, mode: &Mode, bg: &Background, x4: f64, dt: f64, y: &mut Block) {
        let q1 = bg.q(&mode.kappa, x4 + CF4_C1 * dt);
        let q2 = bg.q(&mode.kappa, x4 + CF4_C2 * dt);
        for (wa, wb) in [(CF4_A1, CF4_A2), (CF4_A2, CF4_A1)] {
            // exponent: dt A,  A = (wa + wb) K_perp + sum_t (wa q1_t + wb q2_t) N_t,  A^2 = s 1,  s = -c0^2 E_perp^2 + sum c_t^2
            let c0 = wa + wb;
            let c = [wa * q1[0] + wb * q2[0], wa * q1[1] + wb * q2[1], wa * q1[2] + wb * q2[2]];
            let s = -c0 * c0 * eps * eps + c[0] * c[0] + c[1] * c[1] + c[2] * c[2];
            let a = self.comb(k_perp, c0, &c);
            let rt = s.abs().sqrt();
            let th = dt * rt;
            let (f1, f2) = if rt < 1e-300 {
                (1.0, dt)
            } else if s > 0.0 {
                (th.cosh(), th.sinh() / rt)
            } else {
                (th.cos(), th.sin() / rt)
            };
            apply_exponential(&a, C64::new(f1, 0.0), C64::new(f2, 0.0), y);
        }
    }

    /// The bounded solutions of a mode with k_t != 0.
    ///
    /// The eight decaying solutions are identified at a late x4 (where the WKB exponent
    /// counted from the turning value equals `s_late`) as the eigenvectors of K with the
    /// eigenvalue -sqrt(k_d^2 - E_perp^2), and are integrated backwards to an early time
    /// (k_d = `q_early` E_perp).  Backwards they grow, so the admixture of growing solutions
    /// introduced by rounding dies out: the backward integration is stable.
    pub fn bounded(&self, mode: &Mode, bg: &Background, s_late: f64, q_early: f64, phase: f64, slow: f64) -> Extinction {
        let eps = mode.eps();
        let k_perp = self.k_perp(mode);
        let x4_turn = bg.x4_where_q_is(&mode.kappa, eps);
        let rmax = bg.rate.iter().cloned().fold(0.0, f64::max);
        // find x4_late: integrate lambda = sqrt(k_d^2 - E_perp^2) by the midpoint rule until the exponent is s_late
        let mut t = x4_turn;
        let mut s_acc = 0.0;
        let dts = 1e-4 / rmax;
        while s_acc < s_late {
            let q = bg.q_norm(&mode.kappa, t + 0.5 * dts);
            s_acc += dts * (q * q - eps * eps).max(0.0).sqrt();
            t += dts;
        }
        let x4_late = t;
        let x4_early = bg.x4_where_q_is(&mode.kappa, q_early * eps);
        // decaying subspace at x4_late: range of the projector (1 - K/lambda)/2
        let kl = self.k(mode, bg, x4_late);
        let ql = bg.q_norm(&mode.kappa, x4_late);
        let lam = (ql * ql - eps * eps).sqrt();
        let mut pdec = CMat::identity(16);
        for i in 0..256 {
            pdec.a[i] = (pdec.a[i] - kl.a[i] * (1.0 / lam)) * 0.5;
        }
        let gmat = pdec.mul(&pdec.dagger());
        let (gw, gv) = eigh(&gmat);
        debug_assert!(gw[7] < 1e-8 * gw[15] && gw[8] > 1e-3);
        let mut y = Block::from_columns(&gv, 8, 8); // orthonormal basis of the decaying subspace
        // backward integration with samples of the first column
        let mut profile: Vec<[f64; 3]> = Vec::new();
        let col_norm2 = |yy: &Block| -> f64 { (0..16).map(|i| yy.a[i * 8].norm_sqr()).sum::<f64>() };
        profile.push([x4_late, ql / eps, col_norm2(&y)]);
        let mut t = x4_late;
        let mut steps = 0;
        let dt_prof = (x4_late - x4_early) / 4000.0;
        let mut next = x4_late - dt_prof;
        while t > x4_early {
            let q = bg.q_norm(&mode.kappa, t);
            let scale = ((q * q - eps * eps).abs().sqrt()).max(eps);
            let mut dt = (phase / scale).min(slow / rmax);
            if t - dt < x4_early {
                dt = t - x4_early;
            }
            self.step(&k_perp, eps, mode, bg, t, -dt, &mut y);
            t -= dt;
            steps += 1;
            if t <= next {
                profile.push([t, bg.q_norm(&mode.kappa, t) / eps, col_norm2(&y)]);
                next = t - dt_prof;
            }
        }
        // amplification: singular values of the block at the early time (it was orthonormal at x4_late)
        let gram = y.gram(None, &y);
        let (aw, _) = eigh(&gram);
        let log_amp_min = 0.5 * aw[0].ln();
        let log_amp_max = 0.5 * aw[7].ln();
        // decomposition into the positive- and negative-energy states of h_perp = i K_perp
        let mut h_perp = CMat::zeros(16);
        for i in 0..256 {
            h_perp.a[i] = C64::I * k_perp.a[i];
        }
        let (_, hv) = eigh(&h_perp);
        let vneg = Block::from_columns(&hv, 0, 8);
        let vpos = Block::from_columns(&hv, 8, 8);
        // coefficients: U+ = V+^dagger Y, U- = V-^dagger Y (8x8 each); compare U+^dagger U+ with U-^dagger U-
        let coef = |v: &Block| -> CMat {
            let mut c = CMat::zeros(8);
            for i in 0..8 {
                for j in 0..8 {
                    let mut s = C64::ZERO;
                    for k in 0..16 {
                        s += v.a[k * 8 + i].conj() * y.a[k * 8 + j];
                    }
                    c.set(i, j, s);
                }
            }
            c
        };
        let (up, um) = (coef(&vpos), coef(&vneg));
        let gp = up.dagger().mul(&up);
        let gm_ = um.dagger().mul(&um);
        let standing_wave_dev = gp.sub(&gm_).fro() / gp.fro();
        let charge = y.gram(Some(&self.w), &y);
        let charge_ratio = charge.fro() / gram.fro();
        // normalize the profile to its maximum and put it in increasing order of x4
        let pmax = profile.iter().map(|r| r[2]).fold(0.0, f64::max);
        for r in profile.iter_mut() {
            r[2] /= pmax;
        }
        profile.reverse();
        Extinction { eps, x4_turn, x4_late, s_wkb: s_acc, log_amp_min, log_amp_max, standing_wave_dev, charge_ratio, profile, steps }
    }
}

// =====================================================================================
// modified Bessel function of complex order
// =====================================================================================

/// The modified Bessel function K_nu(s) of complex order nu and real argument s > 0, from
/// the integral representation
///
///     K_nu(s) = Int_0^inf exp(-s cosh t) cosh(nu t) dt ,
///
/// evaluated by Gauss-Legendre quadrature on panels of width 1/8 up to the value of t
/// beyond which the integrand is below exp(-50) of its size.
pub fn bessel_k(nu: C64, s: f64) -> C64 {
    assert!(s > 0.0, "bessel_k: the argument must be positive");
    let gl = spacetimes::quad::gauss_legendre(16);
    // upper limit: s cosh T - |Re nu| T > 50
    let mut tmax = 1.0;
    while s * (tmax as f64).cosh() - nu.re.abs() * tmax < 50.0 {
        tmax += 0.125;
    }
    let panels = (tmax / 0.125).ceil() as usize;
    let h = tmax / panels as f64;
    let mut sum = C64::ZERO;
    for p in 0..panels {
        let (a, b) = (p as f64 * h, (p + 1) as f64 * h);
        for (x, w) in gl.0.iter().zip(gl.1.iter()) {
            let t = 0.5 * (a + b) + 0.5 * (b - a) * x;
            // cosh((a + i b) t) = cosh(a t) cos(b t) + i sinh(a t) sin(b t)
            let ch = C64::new((nu.re * t).cosh() * (nu.im * t).cos(), (nu.re * t).sinh() * (nu.im * t).sin());
            sum += ch * ((-s * t.cosh()).exp() * w * 0.5 * (b - a));
        }
    }
    sum
}

/// |psihat|^2 of the exact bounded solution for one non-zero k_t, up to a constant
/// factor:  2 u_d |K_{1/2 + i E_perp/rate_t}(u_d)|^2  with u_d = q_t/rate_t (arguments: u_d, and the ratio E_perp/rate_t).
pub fn bounded_profile_exact(s: f64, mu: f64) -> f64 {
    2.0 * s * bessel_k(C64::new(0.5, mu), s).norm_sqr()
}

/// The spacetime to which a calculation refers (used by the callers for labels).
pub fn label(sig: Sig) -> &'static str {
    sig.name()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bg1() -> Background {
        Background { rate: [1.0, 1.0, 1.0], cst: [1.0, 1.0, 1.0] }
    }

    #[test]
    fn bessel_k_of_order_one_half_and_the_recurrence_for_complex_order() {
        // K_{1/2}(s) = sqrt(pi/(2s)) exp(-s)
        for s in [0.05, 0.3, 1.0, 4.0, 20.0] {
            let k = bessel_k(C64::new(0.5, 0.0), s);
            let exact = (std::f64::consts::PI / (2.0 * s)).sqrt() * (-s).exp();
            assert!((k.re - exact).abs() < 1e-12 * exact && k.im.abs() < 1e-14, "s = {}", s);
        }
        // recurrence K_{nu+1}(s) - K_{nu-1}(s) = (2 nu/s) K_nu(s) for nu = 1/2 + i mu
        for (mu, s) in [(0.7, 0.4), (2.0, 1.5), (3.5, 6.0)] {
            let nu = C64::new(0.5, mu);
            let lhs = bessel_k(C64::new(1.5, mu), s) - bessel_k(C64::new(-0.5, mu), s);
            let rhs = nu * bessel_k(nu, s) * (2.0 / s);
            assert!((lhs - rhs).abs() < 1e-11 * rhs.abs(), "mu = {}, s = {}", mu, s);
        }
    }

    #[test]
    fn creation_in_one_direction_is_a_fermi_dirac_distribution() {
        let gm = Gammas::build();
        let d = Dirac71::new(&gm);
        for (m, p, kappa, rate) in [
            (1.0, [0.0; 4], [1.0, 0.0, 0.0], [1.0, 1.0, 1.0]),
            (1.0, [0.0, 0.3, -0.4, 0.5], [0.7, 0.0, 0.0], [2.0, 1.0, 1.0]),
            (0.3, [0.2, 0.0, 0.0, 0.0], [0.0, 0.0, 2.0], [1.0, 1.0, 0.5]),
        ] {
            let mode = Mode { m, p, kappa };
            let bg = Background { rate, cst: [1.0, 0.8, 1.3] };
            let r = d.creation(&mode, &bg, 1e-6, 200.0, 0.05, 0.01, false);
            let which = kappa.iter().position(|x| *x != 0.0).unwrap();
            let exact = fermi_dirac_creation(mode.eps(), rate[which]);
            assert!(r.unitarity < 1e-9, "unitarity {}", r.unitarity);
            assert!(((r.n_min - exact) / exact).abs() < 2e-5 && ((r.n_max - exact) / exact).abs() < 2e-5, "numerical [{}, {}], exact {}", r.n_min, r.n_max, exact);
        }
    }

    #[test]
    fn equal_rates_give_the_same_distribution_for_any_direction() {
        let gm = Gammas::build();
        let d = Dirac71::new(&gm);
        let mode = Mode { m: 1.0, p: [0.0, 0.1, 0.2, 0.3], kappa: [0.5, -0.8, 0.3] };
        let bg = Background { rate: [1.3, 1.3, 1.3], cst: [1.0, 1.0, 1.0] };
        let r = d.creation(&mode, &bg, 1e-6, 200.0, 0.05, 0.01, false);
        let exact = fermi_dirac_creation(mode.eps(), 1.3);
        assert!(((r.n_min - exact) / exact).abs() < 2e-5 && ((r.n_max - exact) / exact).abs() < 2e-5);
    }

    #[test]
    fn the_integrator_converges_with_order_four() {
        // The result with a coarse step is compared with a reference computed with a fine
        // step over the same interval of x4 (so that the error of stopping at a finite
        // x4 cancels): halving the step must reduce the error by about 2^4 = 16.
        let gm = Gammas::build();
        let d = Dirac71::new(&gm);
        let mode = Mode { m: 1.0, p: [0.0; 4], kappa: [1.0, 0.0, 0.0] };
        let reference = d.creation(&mode, &bg1(), 1e-6, 50.0, 0.0125, 0.0025, false).n_mean;
        let e1 = (d.creation(&mode, &bg1(), 1e-6, 50.0, 0.2, 0.04, false).n_mean - reference).abs();
        let e2 = (d.creation(&mode, &bg1(), 1e-6, 50.0, 0.1, 0.02, false).n_mean - reference).abs();
        // measured: 2.3e-9 and 1.6e-10, ratio 15.0
        assert!(e1 / e2 > 12.0 && e1 / e2 < 20.0, "errors {} and {}: ratio {}", e1, e2, e1 / e2);
    }

    #[test]
    fn bounded_solutions_of_4_plus_4_are_standing_waves_that_die_out() {
        let gm = Gammas::build();
        let d = Dirac44::new(&gm);
        // one deflating time
        let mode = Mode { m: 1.0, p: [0.2, 0.3, -0.4, 0.5], kappa: [0.7, 0.0, 0.0] };
        let bg = Background { rate: [2.0, 1.0, 1.0], cst: [1.0, 1.0, 1.0] };
        let r = d.bounded(&mode, &bg, 25.0, 1e-6, 0.02, 0.005);
        assert!(r.standing_wave_dev < 1e-6, "standing-wave deviation {}", r.standing_wave_dev);
        assert!(r.charge_ratio < 1e-6, "charge ratio {}", r.charge_ratio);
        // all eight solutions are amplified (backwards) by the same factor, close to exp(S)
        assert!((r.log_amp_max - r.log_amp_min).abs() < 1e-6);
        assert!((r.log_amp_min - r.s_wkb).abs() < 1.5, "log amplification {}, S = {}", r.log_amp_min, r.s_wkb);
        // comparison of the profile with the exact solution 2 u_d |K_{1/2 + i E_perp/rate_t}(u_d)|^2 (ratio constant)
        let mu = mode.eps() / 2.0;
        let mut ratios: Vec<f64> = Vec::new();
        for row in r.profile.iter().step_by(40) {
            let s = row[1] * mode.eps() / 2.0; // s = q/rate
            if s > 0.02 && s < 12.0 {
                ratios.push(row[2] / bounded_profile_exact(s, mu));
            }
        }
        let mean = ratios.iter().sum::<f64>() / ratios.len() as f64;
        for x in &ratios {
            assert!((x / mean - 1.0).abs() < 1e-6, "ratio {} vs mean {}", x, mean);
        }
        // three deflating times with different rates
        let mode = Mode { m: 1.0, p: [0.0, 0.1, 0.2, 0.3], kappa: [0.5, -0.8, 0.3] };
        let bg = Background { rate: [1.3, 0.7, 2.0], cst: [1.0, 0.5, 2.0] };
        let r = d.bounded(&mode, &bg, 25.0, 1e-6, 0.02, 0.005);
        assert!(r.standing_wave_dev < 1e-5 && r.charge_ratio < 1e-5);
    }
}
