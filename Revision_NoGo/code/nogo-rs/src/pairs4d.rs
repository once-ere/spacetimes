//! Command `pairs4d`: the spin-zero (S = 0) superfluid of the 16-component Dirac
//! field on the combined Lorentz-plus-internal manifolds M(3,1) x K(4) (7+1) and
//! M(3,1) x K(1,3) (4+4).
//!
//! Model.  Zero modes on the internal fibre (d_a psi = 0, a in INT), single-particle
//! Hamiltonian h(q) = alpha^j q_j + beta m (module sig.rs), chemical potential mu,
//! three-dimensional momentum cutoff |q| <= Lambda = 1 (all quantities in units of
//! Lambda).  Interactions in the pair channels X_a = C(+-) gamma^K_a:
//!
//!   one field:  H_int = -(G/4) sum_a Phi_a^dagger Phi_a,   Phi_a = psi^T X_a psi,
//!               mean field Delta_a = (G/2) <Phi_a>;
//!   doublet:    H_int = -(G_D/4) Phi_D^dagger Phi_D,       Phi_D = 2 psi1^T C+ psi2,
//!               Delta_D = (G_D/2) <Phi_D>   (the internal-singlet channel of the
//!               gauged U(2) model of Revision_3, reduced to M(3,1)).
//!
//! The eight-dimensionally Lorentz-invariant vector-pair interaction
//!     H_V = (G/4) eta_AB V^A^dagger V^B,   V^A = psi^T C- gamma^A psi,
//! reduces on the zero modes to these channels with ONE coupling G: on 7+1 the four
//! internal components V^a (spin zero) and the three space components V^j (spin
//! one) are attractive, the time component V^4 repulsive; on 4+4 the internal
//! components V^5, V^6, V^7 (spin zero) and V^4 are attractive, V^0 and the space
//! components V^1, V^2, V^3 repulsive.
//!
//! Bogoliubov-de Gennes matrices (32 x 32) in the Nambu basis (psi_q, psi_-q^dagger T)
//! (one field) or (psi1_q, psi2_-q^dagger T) (doublet):
//!     BdG = [[h(q) - mu, B], [B^dagger, -(h(-q) - mu)^T]],
//!     one field: B = -sum_a Delta_a X_a^dagger,   doublet: B = Delta_D conj(C+),
//! and the grand potential per unit volume (Delta-independent constants dropped)
//!     Omega(Delta) = |Delta|^2/G - c_N int d^3q/(2 pi)^3 sum_(E_n > 0) E_n(q),
//! c_N = 1/2 (one field: every mode appears twice in the Nambu basis), 1 (doublet).
//!
//! Isotropic channels (internal vector C- gamma^a, internal 2-form C+ gamma^a gamma^b,
//! doublet C+): the spectrum is +-E^+, +-E^- (8 times each),
//!     E^(+-) = sqrt((eps +- mu)^2 + |Delta|^2),  eps = sqrt(q^2 + m^2),
//! so Omega = |Delta|^2/G - nu int (E^+ + E^-), nu = 4 (one field), 8 (doublet), and
//! the gap equation is 1/G = (nu/2) int (1/E^+ + 1/E^-).
//! Non-unitary (chiral) internal-vector states Delta = |Delta|(cos t n + i sin t m)
//! on 7+1: gaps |Delta| sqrt(1 +- s), s = |sin 2t|.
//! Spin-one channel C- gamma^j (direction n = e_j): E_(+-)^2 = eps^2 + mu^2 + |Delta|^2
//! +- 2 sqrt(mu^2 eps^2 + |Delta|^2 (q.n)^2)  (anisotropic; gap |Delta| m/mu along n).

use crate::checks::Checks;
use crate::exact::{null_space, rank, Q, QMat};
use crate::integrate::{adaptive, breaks_around, nodes};
use crate::sig::{to_c, Sig, SigData, INT, SPACE};
use spacetimes::complex::C64;
use spacetimes::io::{par_map, write_csv};
use spacetimes::linalg::{eigh, CMat};
use spacetimes::roots::brent;
use std::f64::consts::PI;

/// nu for one field and for the doublet.
pub const NU1: f64 = 4.0;
pub const NU2: f64 = 8.0;

fn cadd(a: &CMat, b: &CMat) -> CMat {
    a.add(b)
}

/// h(q) = alpha^j q_j + beta m.
pub fn h_of(sd: &SigData, q: [f64; 3], m: f64) -> CMat {
    let mut h = sd.beta().scale(m);
    for (k, &j) in SPACE.iter().enumerate() {
        h = cadd(&h, &sd.alpha(j).scale(q[k]));
    }
    h
}

fn cscale(m: &CMat, z: C64) -> CMat {
    let mut r = m.clone();
    for x in r.a.iter_mut() {
        *x = *x * z;
    }
    r
}

/// The BdG matrix.
pub fn bdg(sd: &SigData, ks: &[CMat], dl: &[C64], q: [f64; 3], mu: f64, m: f64, doublet: bool) -> CMat {
    let n = 16;
    let id = CMat::identity(n);
    let hp = h_of(sd, q, m).sub(&id.scale(mu));
    let hm = h_of(sd, [-q[0], -q[1], -q[2]], m).sub(&id.scale(mu));
    let hole = hm.transpose().scale(-1.0);
    let mut b = CMat::zeros(n);
    for (k, &d) in ks.iter().zip(dl) {
        let term = if doublet { cscale(&k.conj(), d) } else { cscale(&k.dagger(), C64::new(-d.re, -d.im)) };
        b = b.add(&term);
    }
    let bd = b.dagger();
    let mut out = CMat::zeros(2 * n);
    for i in 0..n {
        for j in 0..n {
            out.set(i, j, hp.get(i, j));
            out.set(i, n + j, b.get(i, j));
            out.set(n + i, j, bd.get(i, j));
            out.set(n + i, n + j, hole.get(i, j));
        }
    }
    out
}

/// Eigenvalues (ascending) of a Hermitian matrix.
pub fn spectrum(m: &CMat) -> Vec<f64> {
    eigh(m).0
}

/// The pair matrices of the channels.
pub struct Channels {
    pub vector: Vec<CMat>,
    pub twoform: Vec<CMat>,
    pub twoform_pairs: Vec<(usize, usize)>,
    pub spin1: Vec<CMat>,
    pub doublet: CMat,
}

pub fn channels(sd: &SigData) -> Channels {
    let vector = INT.iter().map(|&a| to_c(&sd.cm.mul(&sd.g[a]))).collect();
    let tp = crate::kaluza::pairs(&INT);
    let twoform = tp.iter().map(|&(a, b)| to_c(&sd.cp.mul(&sd.g[a]).mul(&sd.g[b]))).collect();
    let spin1 = SPACE.iter().map(|&j| to_c(&sd.cm.mul(&sd.g[j]))).collect();
    Channels { vector, twoform, twoform_pairs: tp, spin1, doublet: to_c(&sd.cp) }
}

/// Kinematic quantities at the radial momentum q, computed without cancellation:
/// eps = sqrt(q^2 + m^2), xp = eps + mu, xm = eps - mu (accurate even where
/// eps is close to mu, through eps - mu = (q^2 - qF^2)/(eps + mu)), and
/// s2 = |eps^2 - mu^2| = |q^2 + m^2 - mu^2|.
#[derive(Clone, Copy, Debug)]
pub struct Kin {
    pub q: f64,
    pub eps: f64,
    pub xp: f64,
    pub xm: f64,
    pub s2: f64,
}

/// E^(+-) = sqrt((eps +- mu)^2 + d2).
fn epm(q: f64, mu: f64, m: f64, d2: f64) -> (f64, f64) {
    let e = (q * q + m * m).sqrt();
    (((e + mu) * (e + mu) + d2).sqrt(), ((e - mu) * (e - mu) + d2).sqrt())
}

fn fermi(mu: f64, m: f64) -> Option<f64> {
    if mu > m {
        Some(((mu - m) * (mu + m)).sqrt())
    } else {
        None
    }
}

/// Kinematics from the shifted variable t: q = qF + t (with a Fermi surface) or
/// q = t (without).
fn kin(t: f64, mu: f64, m: f64) -> Kin {
    match fermi(mu, m) {
        Some(qf) => {
            let q = qf + t;
            let eps = (q * q + m * m).sqrt();
            let d = t * (2.0 * qf + t); // q^2 - qF^2 = eps^2 - mu^2
            Kin { q, eps, xp: eps + mu, xm: d / (eps + mu), s2: d.abs() }
        }
        None => {
            let q = t;
            let eps = (q * q + m * m).sqrt();
            let d = q * q + (m - mu) * (m + mu); // eps^2 - mu^2 >= 0
            Kin { q, eps, xp: eps + mu, xm: d / (eps + mu), s2: d }
        }
    }
}

/// The interval of t and the break points (clustered around the Fermi surface t = 0).
fn t_breaks(mu: f64, m: f64, width: f64) -> Vec<f64> {
    match fermi(mu, m) {
        Some(qf) => breaks_around(-qf, 1.0 - qf, Some(0.0), width.max(1e-13)),
        None => vec![0.0, 1.0],
    }
}

/// int_0^1 dq f(q) for an integrand given in terms of the kinematics.
fn radial<F: Fn(&Kin) -> f64>(f: &F, mu: f64, m: f64, width: f64) -> f64 {
    let g = |t: f64| f(&kin(t, mu, m));
    t_breaks(mu, m, width).windows(2).map(|w| adaptive(&g, w[0], w[1], 1e-15)).sum()
}

/// sqrt(x^2 + d2) - |x| without cancellation.
fn excess(x: f64, d2: f64) -> f64 {
    d2 / ((x * x + d2).sqrt() + x.abs())
}

/// Omega(d) - Omega(0) of an isotropic channel (closed form), nu = 4 or 8.
pub fn omega_iso(d: f64, mu: f64, m: f64, g: f64, nu: f64) -> f64 {
    omega_nonunitary(d, 0.0, mu, m, g, nu)
}

/// Omega(d) - Omega(0) of a state with gaps d sqrt(1 +- s) (s = 0: unitary).
pub fn omega_nonunitary(d: f64, s: f64, mu: f64, m: f64, g: f64, nu: f64) -> f64 {
    let f = |k: &Kin| {
        let mut t = 0.0;
        for sg in [1.0, -1.0] {
            let d2 = d * d * (1.0 + sg * s);
            t += 0.5 * (excess(k.xp, d2) + excess(k.xm, d2));
        }
        k.q * k.q / (2.0 * PI * PI) * t
    };
    d * d / g - nu * radial(&f, mu, m, d.max(1e-9))
}

/// (E_+ + E_-) - (|eps + mu| + |eps - mu|) of the spin-one channel, without cancellation
/// (qn = component of q along the direction n of the pair).
fn spin1_excess(k: &Kin, mu: f64, d: f64, qn: f64) -> f64 {
    let d2 = d * d;
    let a = k.eps * k.eps + mu * mu + d2;
    // P = E_+ E_- = sqrt((eps^2 - mu^2)^2 + 2 d2 (eps^2 + mu^2) + d2^2 - 4 d2 qn^2)
    let num = 2.0 * d2 * (k.eps * k.eps + mu * mu) + d2 * d2 - 4.0 * d2 * qn * qn;
    let p = (k.s2 * k.s2 + num).max(0.0).sqrt();
    let p_minus = num / (p + k.s2); // P - |eps^2 - mu^2|
    let sum = (2.0 * a + 2.0 * p).sqrt(); // E_+ + E_-
    let r = 2.0 * k.eps.max(mu); // |eps + mu| + |eps - mu|
    (2.0 * d2 + 2.0 * p_minus) / (sum + r)
}

/// Omega(d) - Omega(0) of the spin-one channel (closed form, 2D integral), one field.
pub fn omega_spin1(d: f64, mu: f64, m: f64, g: f64) -> f64 {
    let tn = nodes(&t_breaks(mu, m, d.max(1e-6)), 24);
    let cn = nodes(&crate::integrate::uniform(-1.0, 1.0, 8), 16);
    let mut tot = 0.0;
    for (&t, &wt) in tn.x.iter().zip(&tn.w) {
        let k = kin(t, mu, m);
        let mut ang = 0.0;
        for (&c, &wc) in cn.x.iter().zip(&cn.w) {
            ang += wc * spin1_excess(&k, mu, d, k.q * c);
        }
        // int d^3q/(2 pi)^3 = int q^2 dq / (4 pi^2) int_{-1}^{1} dc
        tot += wt * k.q * k.q / (4.0 * PI * PI) * ang;
    }
    d * d / g - 4.0 * tot
}

/// Right-hand side of the gap equation, (nu/2) int (1/E^+ + 1/E^-).
pub fn gap_rhs(d: f64, mu: f64, m: f64, nu: f64) -> f64 {
    let f = |k: &Kin| k.q * k.q / (2.0 * PI * PI) * (1.0 / (k.xp * k.xp + d * d).sqrt() + 1.0 / (k.xm * k.xm + d * d).sqrt());
    0.5 * nu * radial(&f, mu, m, d.max(1e-12))
}

/// Solution d > 0 of 1/G = gap_rhs(d), or 0 if there is none above 1e-12.
pub fn solve_gap(mu: f64, m: f64, g: f64, nu: f64) -> f64 {
    let target = 1.0 / g;
    let lo = 1e-12f64;
    let hi = 20.0f64;
    if gap_rhs(lo, mu, m, nu) < target {
        return 0.0;
    }
    match brent(|x: f64| gap_rhs(x.exp(), mu, m, nu) - target, lo.ln(), hi.ln(), 1e-12, 300) {
        Ok(x) => x.exp(),
        Err(_) => 0.0,
    }
}

/// Critical coupling of the vacuum (mu = 0): 1/G_crit = nu int q^2 dq/(2 pi^2) / eps.
pub fn g_crit_vacuum(m: f64, nu: f64) -> f64 {
    let f = |q: f64| q * q / (2.0 * PI * PI) / (q * q + m * m).sqrt();
    1.0 / (nu * adaptive(&f, 0.0, 1.0, 1e-15))
}

/// c_sigma(w) = (nu/2) int sum_(+-) 1/(E (4E^2 - w^2)); Pi_A(w) = (4d^2 - w^2) c_sigma(w).
pub fn c_sigma(w: f64, d: f64, mu: f64, m: f64, nu: f64) -> f64 {
    let f = |k: &Kin| {
        let mut t = 0.0;
        for x in [k.xp, k.xm] {
            let e2 = x * x + d * d;
            // 4 E^2 - w^2 = 4 x^2 + (4 d^2 - w^2), exact when w = 2d
            t += 1.0 / (e2.sqrt() * (4.0 * x * x + (2.0 * d - w) * (2.0 * d + w)));
        }
        k.q * k.q / (2.0 * PI * PI) * t
    };
    0.5 * nu * radial(&f, mu, m, d.max(1e-9))
}

/// Omega via the numerical BdG spectrum, radial integral with q along x3 (isotropic
/// states only), minus its value at Delta = 0.
pub fn omega_bdg_radial(sd: &SigData, ks: &[CMat], dl: &[C64], mu: f64, m: f64, g: f64, doublet: bool) -> f64 {
    let d2: f64 = dl.iter().map(|z| z.norm_sqr()).sum();
    let zero: Vec<C64> = vec![C64::ZERO; dl.len()];
    let qn = nodes(&breaks_around(0.0, 1.0, fermi(mu, m), d2.sqrt().max(1e-6)), 20);
    let pts: Vec<(f64, f64)> = qn.x.iter().cloned().zip(qn.w.iter().cloned()).collect();
    let cn = if doublet { 1.0 } else { 0.5 };
    let vals = par_map(&pts, |&(q, w)| {
        let e1: f64 = spectrum(&bdg(sd, ks, dl, [0.0, 0.0, q], mu, m, doublet)).iter().filter(|&&x| x > 0.0).sum();
        let e0: f64 = spectrum(&bdg(sd, ks, &zero, [0.0, 0.0, q], mu, m, doublet)).iter().filter(|&&x| x > 0.0).sum();
        w * q * q / (2.0 * PI * PI) * (e1 - e0)
    });
    d2 / g - cn * vals.iter().sum::<f64>()
}

/// Random numbers for the spectral checks (deterministic).
struct Rng(u64);
impl Rng {
    fn uni(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
    fn normal(&mut self) -> f64 {
        let u1 = self.uni().max(1e-300);
        let u2 = self.uni();
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}

/// Maximal deviation of the sorted spectrum from a list of expected eigenvalues.
fn dev(spec: &[f64], mut want: Vec<f64>) -> f64 {
    want.sort_by(|a, b| a.partial_cmp(b).unwrap());
    spec.iter().zip(&want).map(|(a, b)| (a - b).abs()).fold(0.0, f64::max)
}

fn iso_list(q: [f64; 3], mu: f64, m: f64, d2: f64, mult: usize) -> Vec<f64> {
    let qq = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
    let (ep, em) = epm(qq, mu, m, d2);
    let mut v = Vec::new();
    for x in [ep, em] {
        for _ in 0..mult {
            v.push(x);
            v.push(-x);
        }
    }
    v
}

/// Dimension of the stabiliser of a complex vector Delta (rational real and
/// imaginary parts) under a list of real generators acting on R^(2n) = C^n:
/// returns (dimension of the stabiliser, basis vectors of coefficients).
fn stabiliser(gens: &[Vec<Vec<Q>>], dre: &[Q], dim: &[Q]) -> Vec<Vec<Q>> {
    // gens act on the real 2n-vector (Re, Im)
    let n2 = dre.len() + dim.len();
    let v: Vec<Q> = dre.iter().chain(dim.iter()).cloned().collect();
    let mut rows: QMat = vec![vec![Q::ZERO; gens.len()]; n2];
    for (k, g) in gens.iter().enumerate() {
        for i in 0..n2 {
            let mut t = Q::ZERO;
            for j in 0..n2 {
                t = t.add(g[i][j].mul(v[j]));
            }
            rows[i][k] = t;
        }
    }
    null_space(&rows, gens.len())
}

/// Real 2n x 2n generator of U(1) number (charge 2 of a pair): Delta -> 2 i Delta.
fn gen_number(n: usize) -> Vec<Vec<Q>> {
    let mut m = vec![vec![Q::ZERO; 2 * n]; 2 * n];
    for i in 0..n {
        m[i][n + i] = Q::int(-2); // d Re = -2 Im
        m[n + i][i] = Q::int(2); //  d Im = +2 Re
    }
    m
}

/// Real 2n x 2n generator of a real n x n matrix r acting on Re and Im alike.
fn gen_real(r: &[Vec<i64>]) -> Vec<Vec<Q>> {
    let n = r.len();
    let mut m = vec![vec![Q::ZERO; 2 * n]; 2 * n];
    for i in 0..n {
        for j in 0..n {
            m[i][j] = Q::int(r[i][j]);
            m[n + i][n + j] = Q::int(r[i][j]);
        }
    }
    m
}

/// so(4) generators in the vector representation (4 x 4) for the internal axes
/// (index order 0, 5, 6, 7 -> 0, 1, 2, 3); `which` selects the planes.
fn so4_vec(planes: &[(usize, usize)]) -> Vec<Vec<Vec<i64>>> {
    planes
        .iter()
        .map(|&(i, j)| {
            let mut r = vec![vec![0i64; 4]; 4];
            r[i][j] = -1;
            r[j][i] = 1;
            r
        })
        .collect()
}

/// so(4) acting on the six 2-form components (index pairs of 0..3) by the adjoint:
/// (L.T)_{ab} = L_{ac} T_{cb} + L_{bc} T_{ac}, i.e. T -> L T - T L for antisymmetric T.
fn so4_on_2forms(l: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let pr = crate::kaluza::pairs(&[0, 1, 2, 3]);
    let mut out = vec![vec![0i64; 6]; 6];
    for (c, &(a, b)) in pr.iter().enumerate() {
        // basis 2-form E_ab - E_ba
        let mut t = vec![vec![0i64; 4]; 4];
        t[a][b] = 1;
        t[b][a] = -1;
        // L T - T L
        let mut u = vec![vec![0i64; 4]; 4];
        for i in 0..4 {
            for j in 0..4 {
                let mut s = 0;
                for k in 0..4 {
                    s += l[i][k] * t[k][j] - t[i][k] * l[k][j];
                }
                u[i][j] = s;
            }
        }
        for (r, &(p, qq)) in pr.iter().enumerate() {
            out[r][c] = u[p][qq];
        }
    }
    out
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_NoGo, command `pairs4d`: the spin-zero superfluid on the combined manifolds M(3,1) x K");
    let s71 = SigData::new(Sig::S71);
    let s44 = SigData::new(Sig::S44);
    let ch71 = channels(&s71);
    let ch44 = channels(&s44);
    let mut rng = Rng(20261116);

    // ---------------------------------------------------------------- P-1
    for sd in [&s71, &s44] {
        let mut ok = true;
        let b = sd.beta();
        ok &= b.sub(&b.dagger()).fro() < 1e-14 && b.mul(&b).sub(&CMat::identity(16)).fro() < 1e-14;
        for &j in &SPACE {
            let a = sd.alpha(j);
            ok &= a.sub(&a.dagger()).fro() < 1e-14 && a.mul(&a).sub(&CMat::identity(16)).fro() < 1e-14;
            ok &= a.mul(&b).add(&b.mul(&a)).fro() < 1e-14;
            for &k in &SPACE {
                if k > j {
                    let c = sd.alpha(k);
                    ok &= a.mul(&c).add(&c.mul(&a)).fro() < 1e-14;
                }
            }
        }
        ck.add(&format!("P-1 [{}]", sd.tag), "alpha^1, alpha^2, alpha^3, beta are Hermitian, square to 1 and anticommute pairwise: the spectrum of h(q) is +-sqrt(q^2 + m^2) (8 times each)", ok, String::new());
    }

    // ---------------------------------------------------------------- P-2 isotropic spectra
    for (sd, ch) in [(&s71, &ch71), (&s44, &ch44)] {
        let mut worst = 0.0f64;
        for _ in 0..40 {
            let q = [rng.normal(), rng.normal(), rng.normal()];
            let mu = 2.0 * rng.uni();
            let m = rng.uni();
            // vector channel, real (polar) direction along a random internal axis
            let ax = (rng.uni() * 4.0) as usize % 4;
            let d = 0.1 + rng.uni();
            let mut dl = vec![C64::ZERO; 4];
            dl[ax] = C64::new(d, 0.0);
            worst = worst.max(dev(&spectrum(&bdg(sd, &ch.vector, &dl, q, mu, m, false)), iso_list(q, mu, m, d * d, 8)));
            // 2-form channel along a random plane
            let pl = (rng.uni() * 6.0) as usize % 6;
            let mut dt = vec![C64::ZERO; 6];
            dt[pl] = C64::new(0.0, d);
            worst = worst.max(dev(&spectrum(&bdg(sd, &ch.twoform, &dt, q, mu, m, false)), iso_list(q, mu, m, d * d, 8)));
            // doublet channel
            worst = worst.max(dev(&spectrum(&bdg(sd, &[ch.doublet.clone()], &[C64::new(d, 0.0)], q, mu, m, true)), iso_list(q, mu, m, d * d, 8)));
        }
        ck.add(&format!("P-2 [{}]", sd.tag), "isotropic channels (internal vector C- gamma^a, internal 2-form C+ gamma^a gamma^b, doublet C+): the 32 BdG eigenvalues are +-E^+, +-E^- (8 times each), E^(+-) = sqrt((eps +- mu)^2 + |Delta|^2), for every momentum direction (40 random samples each)", worst < 1e-12, format!("max deviation {:.2e}", worst));
    }

    // ---------------------------------------------------------------- P-3 non-unitary states and the Wick map
    let mut worst = 0.0f64;
    let mut worst_iso = 0.0f64;
    for _ in 0..40 {
        let q = [rng.normal(), rng.normal(), rng.normal()];
        let mu = 2.0 * rng.uni();
        let m = rng.uni();
        let d = 0.1 + rng.uni();
        let t = PI * rng.uni();
        let dl = vec![C64::new(d * t.cos(), 0.0), C64::new(0.0, d * t.sin()), C64::ZERO, C64::ZERO];
        let s = (2.0 * t).sin().abs();
        let qq = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
        let mut want = Vec::new();
        for sg in [1.0, -1.0] {
            let (ep, em) = epm(qq, mu, m, d * d * (1.0 + sg * s));
            for x in [ep, em] {
                for _ in 0..4 {
                    want.push(x);
                    want.push(-x);
                }
            }
        }
        let sp = spectrum(&bdg(&s71, &ch71.vector, &dl, q, mu, m, false));
        worst = worst.max(dev(&sp, want));
        // isotropy: same spectrum for the momentum rotated to the x3 axis
        let sp2 = spectrum(&bdg(&s71, &ch71.vector, &dl, [0.0, 0.0, qq], mu, m, false));
        worst_iso = worst_iso.max(dev(&sp, sp2));
    }
    ck.add("P-3a [7+1]", "non-unitary internal-vector states Delta = |Delta| (cos t e0 + i sin t e5): the BdG spectrum is +-sqrt((eps +- mu)^2 + |Delta|^2 (1 +- |sin 2t|)) (4 times each) and independent of the momentum direction; the chiral state (t = pi/4) has gaps sqrt(2)|Delta| and 0", worst < 1e-12 && worst_iso < 1e-12, format!("max deviation {:.2e}, isotropy {:.2e}", worst, worst_iso));
    let mut wv = 0.0f64;
    let mut wt = 0.0f64;
    for _ in 0..40 {
        let q = [rng.normal(), rng.normal(), rng.normal()];
        let mu = 2.0 * rng.uni();
        let m = rng.uni();
        let dl: Vec<C64> = (0..4).map(|_| C64::new(rng.normal(), rng.normal()).scale(0.4)).collect();
        let mut dw = dl.clone();
        dw[0] = dl[0] * C64::I;
        wv = wv.max(dev(&spectrum(&bdg(&s44, &ch44.vector, &dl, q, mu, m, false)), spectrum(&bdg(&s71, &ch71.vector, &dw, q, mu, m, false))));
        let dt: Vec<C64> = (0..6).map(|_| C64::new(rng.normal(), rng.normal()).scale(0.4)).collect();
        let dtw: Vec<C64> = dt.iter().zip(&ch71.twoform_pairs).map(|(&z, &(a, _))| if a == 0 { z * C64::I } else { z }).collect();
        wt = wt.max(dev(&spectrum(&bdg(&s44, &ch44.twoform, &dt, q, mu, m, false)), spectrum(&bdg(&s71, &ch71.twoform, &dtw, q, mu, m, false))));
    }
    ck.add("P-3b [4+4]", "Wick map: for arbitrary complex pair fields the 4+4 BdG spectrum equals the 7+1 spectrum after Delta_0 -> i Delta_0 (vector channel) and Delta_0a -> i Delta_0a (2-form channel): the positive-metric 4+4 superfluid is the 7+1 superfluid with the internal axis x0 Wick-rotated (40 random samples each)", wv < 1e-12 && wt < 1e-12, format!("max deviations {:.2e}, {:.2e}", wv, wt));

    // ---------------------------------------------------------------- P-4 spin-one closed form
    let mut w1 = 0.0f64;
    let mut aniso = 0.0f64;
    for _ in 0..40 {
        let q = [rng.normal(), rng.normal(), rng.normal()];
        let mu = 0.2 + 1.5 * rng.uni();
        let m = rng.uni();
        let d = 0.1 + 0.7 * rng.uni();
        let e2 = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + m * m;
        let r = (mu * mu * e2 + d * d * q[0] * q[0]).sqrt();
        let mut want = Vec::new();
        for sg in [1.0, -1.0] {
            let x = (e2 + mu * mu + d * d + sg * 2.0 * r).max(0.0).sqrt();
            for _ in 0..8 {
                want.push(x);
                want.push(-x);
            }
        }
        for sd_ch in [(&s71, &ch71), (&s44, &ch44)] {
            let sp = spectrum(&bdg(sd_ch.0, &[sd_ch.1.spin1[0].clone()], &[C64::new(d, 0.0)], q, mu, m, false));
            w1 = w1.max(dev(&sp, want.clone()));
        }
        // anisotropy: compare with the isotropic formula
        aniso = aniso.max(dev(&spectrum(&bdg(&s71, &[ch71.spin1[0].clone()], &[C64::new(d, 0.0)], q, mu, m, false)), iso_list(q, mu, m, d * d, 8)));
    }
    ck.add("P-4", "spin-one channel C- gamma^1 (both spacetimes): E_(+-)^2 = eps^2 + mu^2 + |Delta|^2 +- 2 sqrt(mu^2 eps^2 + |Delta|^2 q_1^2) (8 times each) -- ANISOTROPIC: it differs from the isotropic formula unless q_1 = 0", w1 < 1e-12 && aniso > 1e-3, format!("max deviation {:.2e}; anisotropy {:.3}", w1, aniso));

    // ---------------------------------------------------------------- P-5 normalisation of Omega
    let (mu, m, g) = (0.5, 0.2, 6.0);
    let d = 0.08;
    let o_cf = omega_iso(d, mu, m, g, NU1);
    let o_bd = omega_bdg_radial(&s71, &ch71.vector, &[C64::new(d, 0.0), C64::ZERO, C64::ZERO, C64::ZERO], mu, m, g, false);
    let o_cf2 = omega_iso(d, mu, m, g, NU2);
    let o_bd2 = omega_bdg_radial(&s71, &[ch71.doublet.clone()], &[C64::new(d, 0.0)], mu, m, g, true);
    let o_cf44 = omega_iso(d, mu, m, g, NU1);
    let o_bd44 = omega_bdg_radial(&s44, &ch44.vector, &[C64::ZERO, C64::new(d, 0.0), C64::ZERO, C64::ZERO], mu, m, g, false);
    let rel = |a: f64, b: f64| (a - b).abs() / a.abs().max(1e-300);
    ck.add(
        "P-5",
        "normalisation: Omega from the numerical BdG spectrum (c_N = 1/2 one field, 1 doublet) equals the closed forms Omega = |Delta|^2/G - nu int (E^+ + E^-) with nu = 4 (one field, 7+1 and 4+4) and nu = 8 (doublet)",
        rel(o_cf, o_bd) < 1e-7 && rel(o_cf2, o_bd2) < 1e-7 && rel(o_cf44, o_bd44) < 1e-7,
        format!("{:.10e} vs {:.10e}; {:.10e} vs {:.10e}; {:.10e} vs {:.10e}", o_cf, o_bd, o_cf2, o_bd2, o_cf44, o_bd44),
    );

    // ---------------------------------------------------------------- P-6 vacuum: critical couplings and gap curves
    let gc1 = g_crit_vacuum(0.0, NU1);
    let gc2 = g_crit_vacuum(0.0, NU2);
    let gc1m = g_crit_vacuum(0.2, NU1);
    let mut rows_g: Vec<Vec<f64>> = Vec::new();
    let mut ok_thr = true;
    for k in 0..=64 {
        let x = 0.4 + 1.6 * k as f64 / 64.0; // G / G_crit(m = 0, one field)
        let g = x * gc1;
        let d1 = solve_gap(0.0, 0.0, g, NU1);
        let d2 = solve_gap(0.0, 0.0, g, NU2);
        let d1m = solve_gap(0.0, 0.2, g, NU1);
        // below the respective thresholds: no gap
        if g < gc1 * 0.999 {
            ok_thr &= d1 == 0.0;
        }
        if g > gc1 * 1.001 {
            ok_thr &= d1 > 0.0;
        }
        if g < gc2 * 0.999 {
            ok_thr &= d2 == 0.0;
        }
        rows_g.push(vec![x, g, d1, d2, d1m]);
    }
    ck.add(
        "P-6",
        "vacuum (mu = 0): the spin-zero gap appears only above the critical coupling G_crit = 4 pi^2/(nu Lambda^2) for m = 0: pi^2 (one field, internal vector or 2-form) and pi^2/2 (doublet); for m = 0.2 Lambda the one-field threshold is higher",
        (gc1 - PI * PI).abs() < 1e-10 && (gc2 - PI * PI / 2.0).abs() < 1e-10 && gc1m > gc1 && ok_thr,
        format!("G_crit = {:.12} (pi^2 = {:.12}), {:.12}, m = 0.2: {:.6}", gc1, PI * PI, gc2, gc1m),
    );
    write_csv(
        "ng_gap_vs_coupling.csv",
        &[
            ("content", "vacuum (mu = 0) gap |Delta| (units of Lambda) versus coupling G; x = G/G_crit with G_crit = pi^2/Lambda^2 (one field, m = 0)".to_string()),
            ("G_crit", format!("one field m=0: {:.12}; doublet m=0: {:.12}; one field m=0.2: {:.12}", gc1, gc2, gc1m)),
        ],
        &["x", "G", "Delta_onefield_m0", "Delta_doublet_m0", "Delta_onefield_m0.2"],
        &rows_g,
    )
    .expect("csv");

    // ---------------------------------------------------------------- P-7 finite density: Cooper instability and BCS law
    let (mu, m): (f64, f64) = (0.5, 0.2);
    let qf = (mu * mu - m * m).sqrt();
    let mut rows_mu: Vec<Vec<f64>> = Vec::new();
    let g_sub = 0.6 * gc1m; // subcritical in vacuum
    let mut ok_cooper = solve_gap(0.0, m, g_sub, NU1) == 0.0;
    for k in 0..=50 {
        let muk = 0.05 + 0.75 * k as f64 / 50.0;
        let dk = solve_gap(muk, m, g_sub, NU1);
        if muk >= 0.3 {
            ok_cooper &= dk > 0.0;
        }
        if muk < m * 0.95 {
            ok_cooper &= dk == 0.0;
        }
        rows_mu.push(vec![muk, dk]);
    }
    // BCS law: ln Delta versus 1/G, slope -2 pi^2/(nu q_F mu)
    let mut rows_bcs: Vec<Vec<f64>> = Vec::new();
    let mut pts = Vec::new();
    for k in 0..12 {
        let invg = 0.9 / gc1m + 0.05 * k as f64 / gc1m * 3.0;
        let dk = solve_gap(mu, m, 1.0 / invg, NU1);
        pts.push((invg, dk.ln()));
        rows_bcs.push(vec![invg, dk, dk.ln()]);
    }
    let n = pts.len();
    let slope = (pts[n - 1].1 - pts[n - 2].1) / (pts[n - 1].0 - pts[n - 2].0);
    let pred = -2.0 * PI * PI / (NU1 * qf * mu);
    ck.add(
        "P-7",
        "finite density (m = 0.2): for a coupling that is subcritical in vacuum there is no gap for mu < m and a gap for every mu >= 0.3 on the grid (Cooper instability of the Fermi surface; just above mu = m the gap is exponentially small, below the solver floor 1e-12), and at weak coupling ln|Delta| falls linearly in 1/G with the BCS slope -2 pi^2/(nu q_F mu)",
        ok_cooper && (slope / pred - 1.0).abs() < 0.03,
        format!("slope {:.5} vs prediction {:.5} (ratio {:.4})", slope, pred, slope / pred),
    );
    write_csv("ng_gap_vs_mu.csv", &[("content", format!("one-field spin-zero gap |Delta| versus mu at m = 0.2, G = 0.6 G_crit(m=0.2) = {:.10} (subcritical in vacuum); a gap below 1e-12 is reported as 0", g_sub))], &["mu", "Delta"], &rows_mu).expect("csv");
    write_csv("ng_bcs_law.csv", &[("content", format!("weak-coupling law at m = 0.2, mu = 0.5: 1/G, |Delta|, ln|Delta|; predicted slope d ln Delta / d(1/G) = {:.10}", pred))], &["invG", "Delta", "lnDelta"], &rows_bcs).expect("csv");

    // ---------------------------------------------------------------- P-8 which state wins at one 8D coupling
    let (mu, m): (f64, f64) = (0.5, 0.2);
    let g = 0.9 * gc1m;
    let dpol = solve_gap(mu, m, g, NU1);
    let mut rows_om: Vec<Vec<f64>> = Vec::new();
    let mut ok_order = true;
    let mut min_pol = 0.0f64;
    let mut min_chi = 0.0f64;
    let mut min_s1 = 0.0f64;
    let grid: Vec<f64> = (1..=60).map(|k| 2.2 * dpol * k as f64 / 60.0).collect();
    let om_s1: Vec<f64> = par_map(&grid, |&d| omega_spin1(d, mu, m, g));
    for (i, &d) in grid.iter().enumerate() {
        let op = omega_iso(d, mu, m, g, NU1);
        let oc = omega_nonunitary(d, 1.0, mu, m, g, NU1);
        let os1 = om_s1[i];
        ok_order &= oc > op && os1 > op;
        min_pol = min_pol.min(op);
        min_chi = min_chi.min(oc);
        min_s1 = min_s1.min(os1);
        rows_om.push(vec![d, op, oc, os1]);
    }
    // check of the spin-one 2D integral against the BdG spectrum at one point
    ck.add(
        "P-8",
        "at the SAME eight-dimensional vector coupling (7+1, m = 0.2, mu = 0.5, G = 0.9 G_crit(m)): Omega_polar < Omega_chiral and Omega_polar < Omega_spin-one for every |Delta| on the grid; the minimum is the polar spin-zero state (an internal vector condensate)",
        ok_order && min_pol < min_s1 && min_pol < min_chi,
        format!("min Omega: polar {:.6e}, chiral {:.6e}, spin one {:.6e}; polar gap {:.6}", min_pol, min_chi, min_s1, dpol),
    );
    write_csv(
        "ng_omega_states.csv",
        &[("content", format!("Omega(|Delta|) - Omega(0) at m = 0.2, mu = 0.5, G = {:.10} (one field, 7+1): polar spin-zero, chiral spin-zero (s = 1), spin-one polar", g))],
        &["Delta", "Omega_polar", "Omega_chiral", "Omega_spin1"],
        &rows_om,
    )
    .expect("csv");
    // dispersion curves
    let mut rows_disp: Vec<Vec<f64>> = Vec::new();
    for k in 0..=200 {
        let q = k as f64 / 200.0;
        let e = (q * q + m * m).sqrt();
        let (_, em) = epm(q, mu, m, dpol * dpol);
        let (_, emc) = epm(q, mu, m, 2.0 * dpol * dpol);
        let em0 = (e - mu).abs();
        // spin one, q along n (c = 1) and perpendicular (c = 0)
        let s1 = |c: f64| {
            let r = (mu * mu * e * e + dpol * dpol * (q * c) * (q * c)).sqrt();
            (e * e + mu * mu + dpol * dpol - 2.0 * r).max(0.0).sqrt()
        };
        rows_disp.push(vec![q, em, emc, em0, s1(1.0), s1(0.0)]);
    }
    write_csv("ng_dispersion.csv", &[("content", format!("lowest BdG branches at m = 0.2, mu = 0.5, |Delta| = {:.10}: polar E^-, chiral branches sqrt((eps-mu)^2 + 2 Delta^2) and |eps - mu|, spin one along n and perpendicular to n", dpol))], &["q", "polar", "chiral_gapped", "chiral_gapless", "spin1_parallel", "spin1_perp"], &rows_disp).expect("csv");

    // ---------------------------------------------------------------- P-9 amplitude (Higgs) mode
    let gv = 1.5 * gc1; // vacuum, m = 0, supercritical
    let dv = solve_gap(0.0, 0.0, gv, NU1);
    let mut rows_amp: Vec<Vec<f64>> = Vec::new();
    let mut ok_amp = true;
    for k in 0..=100 {
        let w = 2.0 * dv * k as f64 / 100.0;
        let cs = c_sigma(w, dv, 0.0, 0.0, NU1);
        let pa = (4.0 * dv * dv - w * w) * cs;
        ok_amp &= cs.is_finite() && cs > 0.0;
        rows_amp.push(vec![w / (2.0 * dv), cs, pa]);
    }
    let pa0 = 4.0 * dv * dv * c_sigma(0.0, dv, 0.0, 0.0, NU1);
    let h = 1e-3 * dv;
    let om = |d: f64| omega_iso(d, 0.0, 0.0, gv, NU1);
    let second = (-om(dv + 2.0 * h) + 16.0 * om(dv + h) - 30.0 * om(dv) + 16.0 * om(dv - h) - om(dv - 2.0 * h)) / (12.0 * h * h);
    let pa_end = rows_amp.last().unwrap()[2];
    ck.add(
        "P-9",
        "amplitude mode (vacuum, m = 0, G = 1.5 G_crit): Pi_A(w) = (4|Delta|^2 - w^2) c_sigma(w) with c_sigma finite and positive on [0, 2|Delta|], so Pi_A vanishes at w = 2|Delta|: m_sigma = 2|Delta|; static check Pi_A(0) = (1/2) d^2 Omega/d|Delta|^2 at the minimum",
        ok_amp && pa_end.abs() < 1e-12 && (0.5 * second / pa0 - 1.0).abs() < 1e-5,
        format!("|Delta| = {:.10}, Pi_A(0) = {:.8e}, Omega''/2 = {:.8e}", dv, pa0, 0.5 * second),
    );
    write_csv("ng_amplitude_mode.csv", &[("content", format!("amplitude-mode kernel in vacuum, m = 0, G = 1.5 pi^2, |Delta| = {:.10}: w/(2|Delta|), c_sigma(w), Pi_A(w)", dv))], &["w_over_2Delta", "c_sigma", "Pi_A"], &rows_amp).expect("csv");

    // ---------------------------------------------------------------- P-10 symmetry breaking and Goldstone counting
    // internal vector channel (7+1): G = U(1)_N x SO(4); Delta in C^4 (axes 0,5,6,7 -> 0..3)
    let planes = crate::kaluza::pairs(&[0, 1, 2, 3]);
    let so4: Vec<Vec<Vec<i64>>> = so4_vec(&planes);
    let mut gens: Vec<Vec<Vec<Q>>> = vec![gen_number(4)];
    for r in &so4 {
        gens.push(gen_real(r));
    }
    let z = Q::ZERO;
    let one = Q::ONE;
    let polar = stabiliser(&gens, &[one, z, z, z], &[z, z, z, z]);
    let chiral = stabiliser(&gens, &[one, z, z, z], &[z, one, z, z]);
    // the locked generator: a stabiliser element with nonzero N and nonzero L_(01) components
    let locked = chiral.iter().any(|v| !v[0].is_zero() && !v[1].is_zero());
    // 2-form channel: Delta in C^6, so(4) acts by the adjoint on 2-forms; self-dual direction T_01 + T_23
    let mut gens2: Vec<Vec<Vec<Q>>> = vec![gen_number(6)];
    for r in &so4 {
        gens2.push(gen_real(&so4_on_2forms(r)));
    }
    let pr = crate::kaluza::pairs(&[0, 1, 2, 3]);
    let mut sd_re = vec![z; 6];
    sd_re[pr.iter().position(|&p| p == (0, 1)).unwrap()] = one;
    sd_re[pr.iter().position(|&p| p == (2, 3)).unwrap()] = one;
    let tw = stabiliser(&gens2, &sd_re, &[z; 6]);
    // 4+4 with the eta-weighted interaction: symmetry U(1)_N x SO(3) (rotations of 5,6,7)
    let so3: Vec<Vec<Vec<i64>>> = so4_vec(&[(1, 2), (1, 3), (2, 3)]);
    let mut gens44: Vec<Vec<Vec<Q>>> = vec![gen_number(4)];
    for r in &so3 {
        gens44.push(gen_real(r));
    }
    let trip = stabiliser(&gens44, &[z, one, z, z], &[z; 4]);
    // doublet channel: internal singlet, SO(4) acts trivially -> only U(1) broken
    let dim_g = |gs: &Vec<Vec<Vec<Q>>>| -> usize { rank(&gs.iter().map(|m| m.iter().flatten().cloned().collect()).collect::<QMat>()) };
    let ok = polar.len() == 3 && chiral.len() == 2 && locked && tw.len() == 4 && trip.len() == 1 && dim_g(&gens) == 7 && dim_g(&gens2) == 7 && dim_g(&gens44) == 4;
    ck.add(
        "P-10",
        "symmetry breaking (exact stabilisers): internal vector, polar: U(1)_N x SO(4) (7) -> SO(3) (3), 4 Goldstone modes; chiral: -> 2 generators, one of them LOCKS the internal rotation L_(0 5) (an 8D Lorentz rotation) to the fermion number N, 5 Goldstone modes; internal 2-form, self-dual polar: 7 -> U(1) x SU(2) (4), 3 Goldstone modes; 4+4 with the eta-weighted interaction: U(1)_N x SO(3) (4) -> SO(2) (1), 3 Goldstone modes; the four-dimensional Lorentz group is unbroken in every case (spin-zero condensates)",
        ok,
        format!("stabiliser dims: polar {}, chiral {} (locked {}), 2-form {}, 4+4 triplet {}", polar.len(), chiral.len(), locked, tw.len(), trip.len()),
    );
    let gold_rows: Vec<Vec<f64>> = vec![
        vec![71.0, 1.0, 7.0, polar.len() as f64, (7 - polar.len()) as f64],
        vec![71.0, 2.0, 7.0, chiral.len() as f64, (7 - chiral.len()) as f64],
        vec![71.0, 3.0, 7.0, tw.len() as f64, (7 - tw.len()) as f64],
        vec![44.0, 4.0, 4.0, trip.len() as f64, (4 - trip.len()) as f64],
    ];
    write_csv("ng_goldstone.csv", &[("content", "symmetry breaking patterns: phase 1 = internal vector polar, 2 = internal vector chiral, 3 = internal 2-form self-dual polar (7+1), 4 = internal triplet polar with the eta-weighted interaction (4+4); dim G, dim H, number of broken generators".to_string())], &["signature", "phase", "dimG", "dimH", "broken"], &gold_rows).expect("csv");

    // ---------------------------------------------------------------- P-11 signs of the eight-dimensional vector interaction
    let att71: Vec<usize> = (0..8).filter(|&a| s71.eta[a] < 0).collect();
    let att44: Vec<usize> = (0..8).filter(|&a| s44.eta[a] < 0).collect();
    ck.add(
        "P-11",
        "the 8D-invariant interaction H_V = (G/4) eta_AB V^A^dagger V^B (G > 0) attracts the components with eta_AA = -1: on 7+1 these are the internal 0,5,6,7 (spin zero, isotropic) and the space 1,2,3 (spin one, anisotropic) -- the time component 4 is repelled; on 4+4 they are 4,5,6,7: the internal 5,6,7 (spin zero, isotropic) and the time component 4 (not s-wave) -- the internal 0 and the space 1,2,3 are repelled",
        att71 == vec![0, 1, 2, 3, 5, 6, 7] && att44 == vec![4, 5, 6, 7],
        format!("7+1 attractive {:?}; 4+4 attractive {:?}", att71, att44),
    );
    ck.write("ng_pairs4d_checks.txt")
}

