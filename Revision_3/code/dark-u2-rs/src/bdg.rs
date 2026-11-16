//! Command `bdg`: the mean-field (Bogoliubov-de Gennes, BdG) Hamiltonian of the
//! doublet, its spectrum, and the explicit spinor solutions.
//!
//! Nambu basis.  For a spatial momentum q (seven components q^j, j = 0,1,2,3,5,6,7)
//! the two fields that the pair condensate couples are psi^1_U2 at momentum q and the
//! complex conjugate of psi^2_U2 at momentum -q.  In the basis
//! (psi^1(q), psi^2*(-q)) (16 + 16 components) the grand-canonical mean-field
//! Hamiltonian is the real symmetric 32x32 matrix
//!
//!   H(q) = [[ alpha.q + beta M - mu ,  Delta K               ],
//!           [ Delta K                ,  alpha.q - beta M + mu ]],
//!
//! M = M_U2 the effective mass, mu = mu_U2 the chemical potential, Delta = Delta_U2
//! the pair field (real), K = C^+ = beta for the spin-zero pair Phi^+ (or C^- for the
//! pseudoscalar pair Phi^-).  The lower block is -(h(-q))^T + mu with h(q) = alpha.q
//! + beta M; it equals alpha.q - beta M + mu because alpha^j and beta are real
//! symmetric.  With pair momentum Q (condensate Delta exp(i Q.x)) the upper block
//! carries alpha.(q + Q/2) and the lower block alpha.(q - Q/2).
//!
//! Closed forms (derived in the documents and checked here):
//!   K = C^+ :  E^+-(q) = sqrt((eps -+ ... )): E^+ = sqrt((eps+mu)^2+Delta^2), E^- = sqrt((eps-mu)^2+Delta^2),
//!              eps = sqrt(q^2+M^2); eigenvalues +-E^+ and +-E^-, each 8-fold;
//!   K = C^- :  E^2 = eps^2 + mu^2 + Delta^2 +- 2 sqrt(mu^2 eps^2 + M^2 Delta^2), each sign of E 8-fold;
//!   K = C^+, mu = 0, pair momentum Q:  E^2 = q^2 + Q^2/4 + M^2 + Delta^2 +- sqrt((q.Q)^2 + Delta^2 Q^2), 16-fold each.

use crate::algebra::mats;
use crate::checks::Checks;
use crate::rsym::{dot, eigh, RMat};
use spacetimes::complex::C64;
use spacetimes::linalg::{eigh as ceigh, CMat};

/// The real matrices of the 7+1 Hamiltonian.
pub struct Dirac {
    pub alpha: Vec<RMat>,
    pub beta: RMat,
    pub cp: RMat,
    pub cm: RMat,
    pub omega: RMat,
}

pub fn dirac() -> Dirac {
    let m = mats();
    Dirac {
        alpha: m.alpha.iter().map(RMat::from_imat).collect(),
        beta: RMat::from_imat(&m.beta),
        cp: RMat::from_imat(&m.cp),
        cm: RMat::from_imat(&m.cm),
        omega: RMat::from_imat(&m.omega),
    }
}

/// alpha.q = sum_j q^j alpha^j (q in the order of SPACE).
pub fn alpha_dot(d: &Dirac, q: &[f64; 7]) -> RMat {
    let mut r = RMat::zeros(16);
    for (k, a) in d.alpha.iter().enumerate() {
        if q[k] != 0.0 {
            r = r.add(&a.scale(q[k]));
        }
    }
    r
}

/// The BdG matrix with pair momentum Q (Q = 0: the uniform condensate).
pub fn bdg_q(d: &Dirac, q: &[f64; 7], qq: &[f64; 7], mass: f64, mu: f64, delta: f64, k: &RMat) -> RMat {
    let mut qp = [0.0; 7];
    let mut qm = [0.0; 7];
    for i in 0..7 {
        qp[i] = q[i] + 0.5 * qq[i];
        qm[i] = q[i] - 0.5 * qq[i];
    }
    let id = RMat::identity(16);
    let up = alpha_dot(d, &qp).add(&d.beta.scale(mass)).sub(&id.scale(mu));
    let dn = alpha_dot(d, &qm).sub(&d.beta.scale(mass)).add(&id.scale(mu));
    let off = k.scale(delta);
    RMat::block2(&up, &off, &off, &dn)
}

pub fn bdg(d: &Dirac, q: &[f64; 7], mass: f64, mu: f64, delta: f64, k: &RMat) -> RMat {
    bdg_q(d, q, &[0.0; 7], mass, mu, delta, k)
}

/// Closed form for K = C^+: (E^+, E^-).
pub fn e_cplus(qn: f64, mass: f64, mu: f64, delta: f64) -> (f64, f64) {
    let eps = (qn * qn + mass * mass).sqrt();
    (((eps + mu).powi(2) + delta * delta).sqrt(), ((eps - mu).powi(2) + delta * delta).sqrt())
}

/// Closed form for K = C^-: (larger, smaller) positive eigenvalue.
pub fn e_cminus(qn: f64, mass: f64, mu: f64, delta: f64) -> (f64, f64) {
    let e2 = qn * qn + mass * mass;
    let r = (mu * mu * e2 + mass * mass * delta * delta).sqrt();
    let s = e2 + mu * mu + delta * delta;
    ((s + 2.0 * r).sqrt(), (s - 2.0 * r).max(0.0).sqrt())
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn sorted_with(vals: &[(f64, usize)]) -> Vec<f64> {
    let mut v = Vec::new();
    for &(x, m) in vals {
        for _ in 0..m {
            v.push(x);
        }
    }
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    v
}

fn max_dev(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b).map(|(x, y)| (x - y).abs()).fold(0.0, f64::max)
}

/// Orthonormal basis (columns) of the range of a symmetric projector P (Gram-Schmidt on its columns).
fn range_basis(p: &RMat, dim: usize) -> Vec<Vec<f64>> {
    let mut out: Vec<Vec<f64>> = Vec::new();
    for j in 0..p.n {
        let mut v = p.col(j);
        for u in &out {
            let c = dot(u, &v);
            for i in 0..v.len() {
                v[i] -= c * u[i];
            }
        }
        let nrm = dot(&v, &v).sqrt();
        if nrm > 1e-8 {
            out.push(v.iter().map(|x| x / nrm).collect());
        }
        if out.len() == dim {
            break;
        }
    }
    out
}

pub fn run() -> bool {
    let mut ck = Checks::new("PROJECT spacetimes, Revision_3, command `bdg`: the mean-field (BdG) Hamiltonian of the doublet, its spectrum and its spinor solutions");
    let d = dirac();
    let mut rng = Lcg(424242);
    let rq = |rng: &mut Lcg, s: f64| -> [f64; 7] {
        let mut q = [0.0; 7];
        for x in q.iter_mut() {
            *x = s * (2.0 * rng.next() - 1.0);
        }
        q
    };

    // ---------------------------------------------------------------- C1, C2: spectra
    let (mut dev_p, mut dev_m, mut asym) = (0.0f64, 0.0f64, 0.0f64);
    for _ in 0..25 {
        let q = rq(&mut rng, 0.6);
        let (mass, mu, delta) = (0.8 * rng.next(), 1.2 * rng.next(), 0.6 * rng.next() + 0.01);
        let qn = dot(&q, &q).sqrt();
        let h = bdg(&d, &q, mass, mu, delta, &d.cp);
        asym = asym.max(h.sub(&h.transpose()).max_abs());
        let (w, _) = eigh(&h);
        let (ep, em) = e_cplus(qn, mass, mu, delta);
        dev_p = dev_p.max(max_dev(&w, &sorted_with(&[(ep, 8), (-ep, 8), (em, 8), (-em, 8)])));
        let h = bdg(&d, &q, mass, mu, delta, &d.cm);
        let (w, _) = eigh(&h);
        let (eb, es) = e_cminus(qn, mass, mu, delta);
        dev_m = dev_m.max(max_dev(&w, &sorted_with(&[(eb, 8), (-eb, 8), (es, 8), (-es, 8)])));
    }
    ck.add("R3-C1", "spin-zero pair K = C^+: for 25 random (q, M, mu, Delta) the 32 eigenvalues of the real symmetric BdG matrix are +-E^+ and +-E^-, each 8-fold, with E^+- = sqrt((eps +- mu)^2 + Delta^2), eps = sqrt(q^2 + M^2)", dev_p < 1e-12 && asym == 0.0, format!("largest deviation {:.2e}", dev_p));
    ck.add("R3-C2", "pseudoscalar pair K = C^-: the eigenvalues are +-sqrt(eps^2 + mu^2 + Delta^2 +- 2 sqrt(mu^2 eps^2 + M^2 Delta^2)), each 8-fold", dev_m < 1e-11, format!("largest deviation {:.2e}", dev_m));

    // ---------------------------------------------------------------- C3: explicit spinor solutions
    let mut worst_res = 0.0f64;
    let mut worst_orth = 0.0f64;
    for _ in 0..8 {
        let q = rq(&mut rng, 0.7);
        let (mass, mu, delta) = (0.6 * rng.next(), 1.0 * rng.next(), 0.5 * rng.next() + 0.02);
        let qn = dot(&q, &q).sqrt();
        let eps = (qn * qn + mass * mass).sqrt();
        let h16 = alpha_dot(&d, &q).add(&d.beta.scale(mass));
        let big = bdg(&d, &q, mass, mu, delta, &d.cp);
        let mut vecs: Vec<(Vec<f64>, f64)> = Vec::new();
        for s in [1.0f64, -1.0] {
            // projector onto h = s eps:  (1 + s h/eps)/2
            let p = RMat::identity(16).add(&h16.scale(s / eps)).scale(0.5);
            let ws = range_basis(&p, 8);
            assert_eq!(ws.len(), 8);
            let xi = s * eps - mu;
            let e = (xi * xi + delta * delta).sqrt();
            let u = (0.5 * (1.0 + xi / e)).sqrt();
            let v = (0.5 * (1.0 - xi / e)).sqrt();
            for w in &ws {
                let bw = d.beta.matvec(w);
                let mut plus = vec![0.0; 32];
                let mut minus = vec![0.0; 32];
                for i in 0..16 {
                    plus[i] = u * w[i];
                    plus[16 + i] = v * bw[i];
                    minus[i] = -v * w[i];
                    minus[16 + i] = u * bw[i];
                }
                vecs.push((plus, e));
                vecs.push((minus, -e));
            }
        }
        for (x, e) in &vecs {
            let hx = big.matvec(x);
            let r: f64 = hx.iter().zip(x).map(|(a, b)| (a - e * b).powi(2)).sum::<f64>().sqrt();
            worst_res = worst_res.max(r);
        }
        for (i, (x, _)) in vecs.iter().enumerate() {
            for (j, (y, _)) in vecs.iter().enumerate() {
                let g = dot(x, y) - if i == j { 1.0 } else { 0.0 };
                worst_orth = worst_orth.max(g.abs());
            }
        }
    }
    ck.add(
        "R3-C3",
        "explicit spinor solutions: with w an eigenvector of h(q) = alpha.q + beta M of eigenvalue s eps (s = +-1, 8 each), xi = s eps - mu, E = sqrt(xi^2 + Delta^2), u = sqrt((1 + xi/E)/2), v = sqrt((1 - xi/E)/2), the 32 vectors (u w, v beta w) and (-v w, u beta w) are orthonormal eigenvectors of the BdG matrix with eigenvalues +E and -E",
        worst_res < 1e-12 && worst_orth < 1e-12,
        format!("largest residual {:.2e}, largest deviation from orthonormality {:.2e}", worst_res, worst_orth),
    );

    // ---------------------------------------------------------------- C4: pair momentum Q (mu = 0)
    let mut dev_q = 0.0f64;
    for _ in 0..12 {
        let q = rq(&mut rng, 0.6);
        let qq = rq(&mut rng, 0.4);
        let (mass, delta) = (0.6 * rng.next(), 0.5 * rng.next() + 0.02);
        let h = bdg_q(&d, &q, &qq, mass, 0.0, delta, &d.cp);
        let (w, _) = eigh(&h);
        let q2 = dot(&q, &q);
        let qq2 = dot(&qq, &qq);
        let qdq = dot(&q, &qq);
        let base = q2 + 0.25 * qq2 + mass * mass + delta * delta;
        let rt = (qdq * qdq + delta * delta * qq2).sqrt();
        let (ea, eb) = ((base + rt).sqrt(), (base - rt).sqrt());
        dev_q = dev_q.max(max_dev(&w, &sorted_with(&[(ea, 8), (-ea, 8), (eb, 8), (-eb, 8)])));
    }
    ck.add("R3-C4", "condensate with pair momentum Q (phase gradient), K = C^+, mu = 0: the eigenvalues are +-sqrt(q^2 + Q^2/4 + M^2 + Delta^2 +- sqrt((q.Q)^2 + Delta^2 Q^2)), each 8-fold", dev_q < 1e-11, format!("largest deviation {:.2e}", dev_q));

    // ---------------------------------------------------------------- C5: chiral doublet; U1: phase of Delta
    // chiral frame: eigenvectors of chi = i omega (Hermitian), eigenvalue -1 = left-handed
    let mut chi = CMat::zeros(16);
    for i in 0..16 {
        for j in 0..16 {
            chi.set(i, j, C64::new(0.0, d.omega.get(i, j)));
        }
    }
    let (cw, cv) = ceigh(&chi);
    let left: Vec<usize> = (0..16).filter(|&k| (cw[k] + 1.0).abs() < 1e-9).collect();
    let mut dev_ch = if left.len() == 8 { 0.0f64 } else { 1.0 };
    for _ in 0..10 {
        let q = rq(&mut rng, 0.6);
        let (mu, delta) = (1.0 * rng.next(), 0.5 * rng.next() + 0.02);
        let qn = dot(&q, &q).sqrt();
        let a = alpha_dot(&d, &q);
        // basis vectors: e_k (left-handed, upper block) and conj(e_k) (lower block, the conjugate field)
        let mut h = CMat::zeros(16);
        let el: Vec<Vec<C64>> = left.iter().map(|&k| (0..16).map(|i| cv.get(i, k)).collect()).collect();
        let er: Vec<Vec<C64>> = el.iter().map(|v| v.iter().map(|z| z.conj()).collect()).collect();
        let apply = |m: &RMat, v: &Vec<C64>| -> Vec<C64> { (0..16).map(|i| (0..16).fold(C64::ZERO, |s, j| s + v[j] * m.get(i, j))).collect() };
        let inner = |x: &Vec<C64>, y: &Vec<C64>| -> C64 { x.iter().zip(y).fold(C64::ZERO, |s, (a, b)| s + a.conj() * *b) };
        for i in 0..8 {
            for j in 0..8 {
                let up = inner(&el[i], &apply(&a, &el[j])) - if i == j { C64::new(mu, 0.0) } else { C64::ZERO };
                let dn = inner(&er[i], &apply(&a, &er[j])) + if i == j { C64::new(mu, 0.0) } else { C64::ZERO };
                let off = inner(&el[i], &apply(&d.cp, &er[j])) * delta;
                h.set(i, j, up);
                h.set(8 + i, 8 + j, dn);
                h.set(i, 8 + j, off);
                h.set(8 + j, i, off.conj());
            }
        }
        let (w, _) = ceigh(&h);
        let (ep, em) = e_cplus(qn, 0.0, mu, delta);
        dev_ch = dev_ch.max(max_dev(&w, &sorted_with(&[(ep, 4), (-ep, 4), (em, 4), (-em, 4)])));
    }
    ck.add("R3-C5", "chiral (left-handed) doublet: psi^1_L and the conjugate psi^2*_L, which is right-handed, span 16 states; with K = C^+ and no mass the 16 eigenvalues are +-sqrt((q -+ mu)^2 + Delta^2), each 4-fold: the chiral doublet pairs with half the states of the Dirac doublet", dev_ch < 1e-11, format!("largest deviation {:.2e}", dev_ch));

    let mut dev_u1 = 0.0f64;
    for _ in 0..6 {
        let q = rq(&mut rng, 0.6);
        let (mass, mu, delta, ph) = (0.6 * rng.next(), 0.9 * rng.next(), 0.4 * rng.next() + 0.02, 6.28 * rng.next());
        let h = bdg(&d, &q, mass, mu, delta, &d.cp);
        let (w0, _) = eigh(&h);
        let mut hc = CMat::zeros(32);
        let z = C64::cis(ph);
        for i in 0..32 {
            for j in 0..32 {
                let mut x = C64::new(h.get(i, j), 0.0);
                if i < 16 && j >= 16 {
                    x = x * z;
                }
                if i >= 16 && j < 16 {
                    x = x * z.conj();
                }
                hc.set(i, j, x);
            }
        }
        let (w1, _) = ceigh(&hc);
        dev_u1 = dev_u1.max(max_dev(&w0, &w1));
    }
    ck.add("R3-U1", "the spectrum does not depend on the phase of the pair field: Delta -> Delta exp(i phi) (upper-right block times exp(i phi), lower-left block times exp(-i phi)) leaves all 32 eigenvalues unchanged (6 random phases); the effective potential depends on |Delta| only", dev_u1 < 1e-11, format!("largest deviation {:.2e}", dev_u1));

    // ---------------------------------------------------------------- C6: gap at the Fermi surface, C^+ versus C^-
    let (mass, mu, delta): (f64, f64, f64) = (0.3, 0.5, 0.08);
    let mut min_p = f64::MAX;
    let mut min_m = f64::MAX;
    let mut rows = Vec::new();
    let qf = (mu * mu - mass * mass).sqrt();
    for k in 0..=4000 {
        let qn = 0.2 + 0.4 * k as f64 / 4000.0;
        let (_, em) = e_cplus(qn, mass, mu, delta);
        let (_, es) = e_cminus(qn, mass, mu, delta);
        min_p = min_p.min(em);
        min_m = min_m.min(es);
        if k % 20 == 0 {
            let eps = (qn * qn + mass * mass).sqrt();
            rows.push(vec![qn, (eps - mu).abs(), em, es]);
        }
    }
    // confirm the closed forms at the Fermi momentum with the matrix itself
    let mut q = [0.0; 7];
    q[1] = qf;
    let (wp, _) = eigh(&bdg(&d, &q, mass, mu, delta, &d.cp));
    let lowest_p = wp.iter().cloned().filter(|x| *x > 0.0).fold(f64::MAX, f64::min);
    let qmin = (mu * mu - mass * mass * delta * delta / (mu * mu) - mass * mass).sqrt();
    q[1] = qmin;
    let (wm, _) = eigh(&bdg(&d, &q, mass, mu, delta, &d.cm));
    let lowest_m = wm.iter().cloned().filter(|x| *x > 0.0).fold(f64::MAX, f64::min);
    let pred_m = delta * (1.0 - mass * mass / (mu * mu)).sqrt();
    ck.add(
        "R3-C6",
        "the gap at the Fermi surface: for the spin-zero pair (C^+) the smallest quasi-particle energy is Delta itself, reached at eps = mu; for the pseudoscalar pair (C^-) it is Delta sqrt(1 - M^2/mu^2) = Delta q_F/mu, which vanishes in the non-relativistic limit (M = 0.3, mu = 0.5, Delta = 0.08)",
        (min_p - delta).abs() < 1e-6 && (min_m - pred_m).abs() < 1e-6 && (lowest_p - delta).abs() < 1e-12 && (lowest_m - pred_m).abs() < 1e-12,
        format!("C^+: {:.10} (BdG matrix {:.10}); C^-: {:.10} (BdG matrix {:.10}, predicted {:.10})", min_p, lowest_p, min_m, lowest_m, pred_m),
    );
    spacetimes::io::write_csv(
        "r3_fermi_surface_gap.csv",
        &[("meaning", "quasi-particle energies near the Fermi surface for M = 0.3, mu = 0.5, Delta = 0.08 (units of the cutoff): normal |eps - mu|, spin-zero pair (C^+) E^-, pseudoscalar pair (C^-) smaller branch".to_string())],
        &["q", "normal", "E_Cplus", "E_Cminus"],
        &rows,
    )
    .expect("csv");
    ck.write("r3_bdg_checks.txt")
}
