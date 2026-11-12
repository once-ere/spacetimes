//! Pairing kernels of the Vorton-Scalar exchange, channel by channel.
//!
//! PHYSICS.  The Vorton-Scalar field couples to the fermion field through two
//! Hermitian 16x16 "vertex" matrices,
//!
//! * the SCALAR vertex        Gamma_S = beta                 (exchange of the radial mode),
//! * the PSEUDOSCALAR vertex  Gamma_P = i beta chi           (exchange of the phase mode),
//!
//! where beta is the matrix of the mass term of the one-particle Hamiltonian
//! h(p) = alpha.p + beta m and chi is the chirality matrix (chi beta = -beta chi,
//! chi alpha = alpha chi).  For the two spacetimes of the project (MASTER provenance):
//!
//! * 7+1 (tag 1671):  beta = gamma1671[4],  alpha^j = gamma1671[4].gamma1671[j],
//!                    chi = chi1671 = i omega1671;
//! * 4+4 (tag 1644), fields independent of x5 x6 x7 (the unitary sector):
//!                    beta = -i gamma1644[4],  alpha^a = -gamma1644[4].gamma1644[a]
//!                    (a = 0..3),  chi = omega1644.
//!
//! One boson exchange scatters a pair of particles with momenta (p, -p) and internal
//! states (a, b) into (p', -p') and (a', b') with an amplitude proportional to
//!     V1_{a'a} V2_{b'b},   V1 = U(p')^dagger Gamma U(p),   V2 = U(-p')^dagger Gamma U(-p),
//! where the columns of the 16x8 matrix U(p) are the orthonormal positive-energy
//! spinors.  A pair operator dirac^T X dirac contains the product b_a(p) b_b(-p) of
//! annihilation operators with the coefficient Phi_X(p)_{ab} = (U(p)^T X U(-p))_{ab};
//! the pair STATE it creates has the wave function conj(Phi_X).  The exchange
//! amplitude in the channel X is therefore
//!
//!     F_X(p', p) = Tr[ Phi_X(p')^T V1 conj(Phi_X(p)) V2^T ] / sqrt(N(p') N(p)),
//!     N(p) = sum_ab |Phi_X(p)_ab|^2.
//!
//! F_X > 0 means ATTRACTION for the exchange of a boson (the potential is
//! -g^2 F_X / (q^2 + M^2), q = p' - p).  The expression is independent of the choice
//! of the bases U(p) (tested below).
//!
//! RESULT (verified here against the matrices, for both signatures).  For the pair
//! operator along the hidden axis, X = C^- gamma^0, and momenta in ordinary space,
//!
//!   scalar exchange:        F_S = [m^2 (E+E')^2 + E^2 p'^2 + E'^2 p^2 - 2 E E' p.p'] / (4 E^2 E'^2)
//!   pseudoscalar exchange:  F_P = [(m^2 - E E')^2 + 2 (m^2 - E E') p.p' + m^2 q^2 + p^2 p'^2] / (4 E^2 E'^2)
//!
//! which on the energy shell (|p| = |p'|) reduce to (4 m^2 + q^2)/(4 E^2) and q^2/(4 E^2).
//! Both are positive: BOTH modes of the Vorton-Scalar attract in the hidden-axis channel.

use crate::complex::C64;
use crate::gamma::Gammas;
use crate::imat::IMat;
use crate::linalg::{eigh, CMat};

/// A rectangular complex matrix stored by rows.
pub type Mat = Vec<Vec<C64>>;

fn c(re: f64, im: f64) -> C64 {
    C64::new(re, im)
}

pub fn zeros(n: usize, m: usize) -> Mat {
    vec![vec![c(0.0, 0.0); m]; n]
}

pub fn from_imat(a: &IMat) -> Mat {
    (0..a.n).map(|i| (0..a.m).map(|j| c(a.get(i, j) as f64, 0.0)).collect()).collect()
}

pub fn mul(a: &Mat, b: &Mat) -> Mat {
    let (n, k, m) = (a.len(), b.len(), b[0].len());
    assert_eq!(a[0].len(), k);
    let mut out = zeros(n, m);
    for i in 0..n {
        for l in 0..k {
            let x = a[i][l];
            if x.re == 0.0 && x.im == 0.0 {
                continue;
            }
            for j in 0..m {
                out[i][j] += x * b[l][j];
            }
        }
    }
    out
}

pub fn scale(a: &Mat, s: C64) -> Mat {
    a.iter().map(|r| r.iter().map(|x| *x * s).collect()).collect()
}

pub fn add(a: &Mat, b: &Mat) -> Mat {
    a.iter().zip(b).map(|(r, s)| r.iter().zip(s).map(|(x, y)| *x + *y).collect()).collect()
}

pub fn transpose(a: &Mat) -> Mat {
    (0..a[0].len()).map(|j| (0..a.len()).map(|i| a[i][j]).collect()).collect()
}

pub fn conj(a: &Mat) -> Mat {
    a.iter().map(|r| r.iter().map(|x| x.conj()).collect()).collect()
}

pub fn dagger(a: &Mat) -> Mat {
    conj(&transpose(a))
}

pub fn trace(a: &Mat) -> C64 {
    let mut t = c(0.0, 0.0);
    for i in 0..a.len() {
        t += a[i][i];
    }
    t
}

pub fn norm2(a: &Mat) -> f64 {
    a.iter().map(|r| r.iter().map(|x| x.norm_sqr()).sum::<f64>()).sum()
}

/// The one-particle problem of one of the two spacetimes.
pub struct OneParticle {
    /// "1671" (7+1) or "1644" (4+4, fields independent of x5 x6 x7).
    pub tag: &'static str,
    /// The eight Dirac matrices.
    pub gam: Vec<Mat>,
    /// Hermitian matrix of the mass term, beta.beta = 1.
    pub beta: Mat,
    /// alpha[A] for the coordinates along which a particle can move (empty otherwise).
    pub alpha: Vec<Option<Mat>>,
    /// Chirality matrix (Hermitian, square 1).
    pub chi: Mat,
    /// The transposition matrices C^- and C^+.
    pub cminus: Mat,
    pub cplus: Mat,
}

impl OneParticle {
    /// 7+1: beta = gamma1671[4], alpha^j = gamma1671[4].gamma1671[j], chi1671 = i omega1671.
    pub fn new_1671(gm: &Gammas) -> OneParticle {
        let g: Vec<Mat> = gm.gamma1671.iter().map(from_imat).collect();
        let refs: Vec<&IMat> = gm.gamma1671.iter().collect();
        let om = IMat::product(&refs);
        let beta = g[4].clone();
        let alpha = (0..8).map(|j| if j == 4 { None } else { Some(mul(&g[4], &g[j])) }).collect();
        OneParticle {
            tag: "1671",
            beta,
            alpha,
            chi: scale(&from_imat(&om), c(0.0, 1.0)),
            cplus: g[4].clone(),
            cminus: from_imat(&gm.gamma1671[4].mul(&om)),
            gam: g,
        }
    }

    /// 4+4, unitary sector: h1644 = -i m gamma^4 - k_a gamma^4 gamma^a (a = 0..3), so
    /// beta = -i gamma1644[4], alpha^a = -gamma1644[4].gamma1644[a]; chi = omega1644.
    pub fn new_1644(gm: &Gammas) -> OneParticle {
        let g: Vec<Mat> = gm.gamma1644.iter().map(from_imat).collect();
        let beta = scale(&g[4], c(0.0, -1.0));
        let alpha = (0..8).map(|a| if a <= 3 { Some(scale(&mul(&g[4], &g[a]), c(-1.0, 0.0))) } else { None }).collect();
        OneParticle {
            tag: "1644",
            beta,
            alpha,
            chi: from_imat(&gm.t16_8),
            cminus: from_imat(&gm.sigma16),
            cplus: from_imat(&gm.s44),
            gam: g,
        }
    }

    /// The Hamiltonian matrix h(p) = alpha.p + beta m; `p[A]` is the momentum along xA.
    pub fn hamiltonian(&self, p: &[f64; 8], m: f64) -> Mat {
        let mut h = scale(&self.beta, c(m, 0.0));
        for a in 0..8 {
            if p[a] != 0.0 {
                let al = self.alpha[a].as_ref().expect("the particle cannot move along this coordinate");
                h = add(&h, &scale(al, c(p[a], 0.0)));
            }
        }
        h
    }

    /// The 16x8 matrix U(p) of orthonormal positive-energy spinors (columns), and the energy.
    pub fn particles(&self, p: &[f64; 8], m: f64) -> (Mat, f64) {
        let h = self.hamiltonian(p, m);
        let mut hm = CMat::zeros(16);
        for i in 0..16 {
            for j in 0..16 {
                hm.set(i, j, h[i][j]);
            }
        }
        let (w, v) = eigh(&hm);
        // eigenvalues ascend: the last eight are +E
        let e = w[15];
        debug_assert!((w[8] - e).abs() < 1e-9 * e && (w[7] + e).abs() < 1e-9 * e);
        let u: Mat = (0..16).map(|i| (8..16).map(|k| v.get(i, k)).collect()).collect();
        (u, e)
    }

    /// Scalar vertex matrix beta.
    pub fn vertex_scalar(&self) -> Mat {
        self.beta.clone()
    }

    /// Pseudoscalar vertex matrix i beta chi (Hermitian).
    pub fn vertex_pseudoscalar(&self) -> Mat {
        scale(&mul(&self.beta, &self.chi), c(0.0, 1.0))
    }

    /// Pair operator matrix C^- gamma^A (the vector pair operator DeltaV^A).
    pub fn pair_vector(&self, a: usize) -> Mat {
        mul(&self.cminus, &self.gam[a])
    }

    /// Pair operator matrix C^+ gamma^A gamma^B (the two-form pair operator).
    pub fn pair_twoform(&self, a: usize, b: usize) -> Mat {
        mul(&mul(&self.cplus, &self.gam[a]), &self.gam[b])
    }
}

fn neg(p: &[f64; 8]) -> [f64; 8] {
    let mut q = *p;
    for x in q.iter_mut() {
        *x = -*x;
    }
    q
}

/// The channel amplitude F_X(p', p) for the vertex matrix `vertex` and the pair
/// operator matrix `x` (see the module documentation).  Returns (F, N(p), N(p')),
/// where N is the squared norm of the two-particle content of the operator.
pub fn channel_amplitude(op: &OneParticle, vertex: &Mat, x: &Mat, p: &[f64; 8], pp: &[f64; 8], m: f64) -> (f64, f64, f64) {
    let (u_p, _) = op.particles(p, m);
    let (u_mp, _) = op.particles(&neg(p), m);
    let (u_pp, _) = op.particles(pp, m);
    let (u_mpp, _) = op.particles(&neg(pp), m);
    let phi = mul(&mul(&transpose(&u_p), x), &u_mp); // Phi_X(p)
    let phip = mul(&mul(&transpose(&u_pp), x), &u_mpp); // Phi_X(p')
    let v1 = mul(&mul(&dagger(&u_pp), vertex), &u_p);
    let v2 = mul(&mul(&dagger(&u_mpp), vertex), &u_mp);
    let t = trace(&mul(&mul(&mul(&transpose(&phip), &v1), &conj(&phi)), &transpose(&v2)));
    let (n, np) = (norm2(&phi), norm2(&phip));
    if n < 1e-20 || np < 1e-20 {
        return (0.0, n, np);
    }
    debug_assert!(t.im.abs() < 1e-9 * (1.0 + t.re.abs()));
    (t.re / (n * np).sqrt(), n, np)
}

/// Closed form of the scalar-exchange amplitude in the hidden-axis channel
/// (momenta orthogonal to the hidden axis).  `k`, `kp`: |p|, |p'|; `mu`: cos of the
/// angle between p and p'.
pub fn f_scalar_hidden(m: f64, k: f64, kp: f64, mu: f64) -> f64 {
    let (e, ep) = ((m * m + k * k).sqrt(), (m * m + kp * kp).sqrt());
    (m * m * (e + ep) * (e + ep) + e * e * kp * kp + ep * ep * k * k - 2.0 * e * ep * k * kp * mu) / (4.0 * e * e * ep * ep)
}

/// Closed form of the pseudoscalar-exchange amplitude in the hidden-axis channel.
pub fn f_pseudo_hidden(m: f64, k: f64, kp: f64, mu: f64) -> f64 {
    let (e, ep) = ((m * m + k * k).sqrt(), (m * m + kp * kp).sqrt());
    let cc = m * m - e * ep;
    let pdot = k * kp * mu;
    let q2 = k * k + kp * kp - 2.0 * pdot;
    (cc * cc + 2.0 * cc * pdot + m * m * q2 + k * k * kp * kp) / (4.0 * e * e * ep * ep)
}

/// The gap matrix of the four spin-0 vector pairs, and the invariants that enter the
/// Ginzburg-Landau energy.
///
/// For an order parameter with components Delta^a along the hidden axis and the
/// three other extra axes (a = 0, 5, 6, 7) the gap matrix on the eight internal
/// states of the pair (p, -p) is  M = sum_a Delta^a Phi_a(p) / sqrt(N_a/8),  where
/// Phi_a(p) = U(p)^T C^- gamma^a U(-p) is the two-particle content of the vector pair
/// operator along a, and N_a its squared norm (so that each normalized content has
/// Phi Phi^dagger of trace 8).  Returns
///     t2 = Tr[M M^dagger] / 8 ,     t4 = Tr[(M M^dagger)^2] / 8 .
/// The traces do not depend on the choice of the basis U(p).
pub fn gap_matrix_invariants(op: &OneParticle, delta: &[C64; 4], p: &[f64; 8], m: f64) -> (f64, f64) {
    let (u, _) = op.particles(p, m);
    let (um, _) = op.particles(&neg(p), m);
    let axes = [0usize, 5, 6, 7];
    let mut gap = zeros(8, 8);
    for (i, &a) in axes.iter().enumerate() {
        let phi = mul(&mul(&transpose(&u), &op.pair_vector(a)), &um);
        let n = norm2(&phi);
        gap = add(&gap, &scale(&phi, delta[i] * (8.0 / n).sqrt()));
    }
    let mm = mul(&gap, &dagger(&gap));
    (trace(&mm).re / 8.0, trace(&mul(&mm, &mm)).re / 8.0)
}

/// Check of the quartic Ginzburg-Landau invariant: for every complex 4-vector Delta
/// (components along x0, x5, x6, x7),
///     t2 = sum_a |Delta^a|^2 ,
///     t4 = 2 (sum_a |Delta^a|^2)^2 - | eta_ab Delta^a Delta^b |^2 ,
/// where eta_ab is the metric of the spacetime restricted to the four axes:
/// (-1, -1, -1, -1) in 7+1 (all four are space-like) and (+1, -1, -1, -1) in 4+4 (x0
/// is space-like, x5, x6, x7 are time-like).  Returns the largest deviation found for
/// a set of test vectors, at rest and at |p| = m.
pub fn quartic_invariant_check(op: &OneParticle) -> f64 {
    let eta: [f64; 4] = if op.tag == "1644" { [1.0, -1.0, -1.0, -1.0] } else { [-1.0; 4] };
    let tests: [[C64; 4]; 5] = [
        [c(1.0, 0.0), c(0.0, 0.0), c(0.0, 0.0), c(0.0, 0.0)],
        [c(0.6, 0.0), c(0.0, 0.0), c(0.8, 0.0), c(0.0, 0.0)],
        [c(1.0, 0.0), c(0.0, 1.0), c(0.0, 0.0), c(0.0, 0.0)],
        [c(0.3, -0.7), c(1.1, 0.2), c(-0.4, 0.9), c(0.5, 0.5)],
        [c(0.0, 0.2), c(-1.3, 0.4), c(0.7, 0.7), c(0.1, -0.6)],
    ];
    let mut worst: f64 = 0.0;
    for p in [[0.0; 8], [0.0, 0.6, 0.0, 0.8, 0.0, 0.0, 0.0, 0.0]] {
        for d in &tests {
            let (t2, t4) = gap_matrix_invariants(op, d, &p, 1.0);
            let dd: f64 = d.iter().map(|z| z.norm_sqr()).sum();
            let mut d2 = c(0.0, 0.0);
            for (z, e) in d.iter().zip(&eta) {
                d2 += *z * *z * *e;
            }
            worst = worst.max((t2 - dd).abs()).max((t4 - (2.0 * dd * dd - d2.norm_sqr())).abs());
        }
    }
    worst
}

/// One line of the channel table: on-shell amplitudes at scattering angle theta.
pub struct ChannelRow {
    pub tag: &'static str,
    pub channel: String,
    pub description: String,
    pub norm: f64,
    pub f_s: Vec<f64>,
    pub f_p: Vec<f64>,
}

/// On-shell channel amplitudes for a list of representative pair operators, for
/// momenta of magnitude `k` in the (x1, x2) plane and the scattering angles `thetas`.
pub fn channel_table(op: &OneParticle, m: f64, k: f64, thetas: &[f64]) -> Vec<ChannelRow> {
    let vs = op.vertex_scalar();
    let vp = op.vertex_pseudoscalar();
    let mut ops: Vec<(String, String, Mat)> = Vec::new();
    for a in 0..8 {
        let what = match (op.tag, a) {
            (_, 0) => "vector pair operator along the hidden axis x0",
            (_, 1) | (_, 2) | (_, 3) => "vector pair operator along an ordinary space axis",
            (_, 4) => "vector pair operator along the time axis x4",
            ("1671", _) => "vector pair operator along an extra space axis",
            _ => "vector pair operator along an extra time axis",
        };
        ops.push((format!("Cm{}.gamma{}[{}]", op.tag, op.tag, a), what.to_string(), op.pair_vector(a)));
    }
    for &(a, b, what) in &[
        (0usize, 5usize, "two-form pair operator, plane (x0, x5)"),
        (5, 6, "two-form pair operator, plane (x5, x6)"),
        (1, 2, "two-form pair operator, plane (x1, x2): the plane of the momenta"),
        (2, 3, "two-form pair operator, plane (x2, x3)"),
        (0, 3, "two-form pair operator, plane (x0, x3)"),
        (0, 4, "two-form pair operator, plane (x0, x4)"),
        (3, 5, "two-form pair operator, plane (x3, x5)"),
    ] {
        ops.push((format!("Cp{}.gamma{}[{}].gamma{}[{}]", op.tag, op.tag, a, op.tag, b), what.to_string(), op.pair_twoform(a, b)));
    }
    let mut p = [0.0; 8];
    p[1] = k;
    let mut rows = Vec::new();
    for (name, what, x) in ops {
        let mut f_s = Vec::new();
        let mut f_p = Vec::new();
        let mut norm = 0.0;
        for &th in thetas {
            let mut pp = [0.0; 8];
            pp[1] = k * th.cos();
            pp[2] = k * th.sin();
            let (fs, n, _) = channel_amplitude(op, &vs, &x, &p, &pp, m);
            let (fp, _, _) = channel_amplitude(op, &vp, &x, &p, &pp, m);
            norm = n;
            f_s.push(fs);
            f_p.push(fp);
        }
        rows.push(ChannelRow { tag: op.tag, channel: name, description: what, norm, f_s, f_p });
    }
    rows
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fmat::Rng;

    fn random_momentum(rng: &mut Rng, k: f64) -> [f64; 8] {
        // a random direction in the ordinary space x1 x2 x3
        let (mut x, mut y, mut z);
        loop {
            x = rng.range(-1.0, 1.0);
            y = rng.range(-1.0, 1.0);
            z = rng.range(-1.0, 1.0);
            let r2: f64 = x * x + y * y + z * z;
            if r2 > 0.05 && r2 < 1.0 {
                let r = r2.sqrt();
                x /= r;
                y /= r;
                z /= r;
                break;
            }
        }
        let mut p = [0.0; 8];
        p[1] = k * x;
        p[2] = k * y;
        p[3] = k * z;
        p
    }

    /// The closed forms of the hidden-axis channel hold for both signatures, on and
    /// off the energy shell.
    #[test]
    fn hidden_axis_closed_forms() {
        let gm = Gammas::build();
        let mut rng = Rng(20261112);
        for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
            let x = op.pair_vector(0);
            let (vs, vp) = (op.vertex_scalar(), op.vertex_pseudoscalar());
            for _ in 0..12 {
                let m = rng.range(0.2, 2.0);
                let (k, kp) = (rng.range(0.1, 3.0), rng.range(0.1, 3.0));
                let (p, pp) = (random_momentum(&mut rng, k), random_momentum(&mut rng, kp));
                let mu = (p[1] * pp[1] + p[2] * pp[2] + p[3] * pp[3]) / (k * kp);
                let (fs, n, np) = channel_amplitude(&op, &vs, &x, &p, &pp, m);
                let (fp, _, _) = channel_amplitude(&op, &vp, &x, &p, &pp, m);
                assert!(n > 1e-6 && np > 1e-6, "{}: the hidden-axis operator must contain particle pairs", op.tag);
                assert!((fs - f_scalar_hidden(m, k, kp, mu)).abs() < 1e-10, "{} scalar: {} vs {}", op.tag, fs, f_scalar_hidden(m, k, kp, mu));
                assert!((fp - f_pseudo_hidden(m, k, kp, mu)).abs() < 1e-10, "{} pseudoscalar: {} vs {}", op.tag, fp, f_pseudo_hidden(m, k, kp, mu));
            }
        }
    }

    /// Positive-energy spinors: h U = E U, U^dagger U = 1, for both signatures.
    #[test]
    fn particle_spinors() {
        let gm = Gammas::build();
        let mut rng = Rng(7);
        for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
            let m = 1.3;
            let p = random_momentum(&mut rng, 0.8);
            let (u, e) = op.particles(&p, m);
            assert!((e - (m * m + 0.64f64).sqrt()).abs() < 1e-12);
            let hu = mul(&op.hamiltonian(&p, m), &u);
            let diff = add(&hu, &scale(&u, c(-e, 0.0)));
            assert!(norm2(&diff) < 1e-20);
            let uu = mul(&dagger(&u), &u);
            for i in 0..8 {
                for j in 0..8 {
                    let want = if i == j { 1.0 } else { 0.0 };
                    assert!((uu[i][j].re - want).abs() < 1e-12 && uu[i][j].im.abs() < 1e-12);
                }
            }
        }
    }

    /// Quartic invariant of the Ginzburg-Landau energy, both signatures.
    #[test]
    fn ginzburg_landau_quartic_invariant() {
        let gm = Gammas::build();
        for op in [OneParticle::new_1671(&gm), OneParticle::new_1644(&gm)] {
            let w = quartic_invariant_check(&op);
            assert!(w < 1e-9, "signature {}: deviation {:e}", op.tag, w);
        }
    }
}

