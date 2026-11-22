//! The linear response of a uniform quasi-free state with momentum-independent self-energies: the static stability of
//! a state and its collective modes at zero total momentum (the random-phase approximation of the time-dependent mean
//! field (6.6); Part II of superfluid.md, Section 10.4).
//!
//! A uniform fluctuation of frequency omega (the factor e^{-i omega t}) changes the BdG matrix (6.5) by
//!     dH = ((dN, dP), (dPbar, -dN^T)),
//! where dN, dP and dPbar are the complex amplitudes of the normal self-energy, of the pair self-energy and of its
//! adjoint.  Linearizing (6.6), i omega dR = ... gives at each node, in the eigenbasis (E_k, |k>) of H(q),
//!     dR_kl = dH_kl (f_l - f_k) / (omega - E_k + E_l)          (f = 1 for E < 0, else 0),
//! and the local density amplitudes are drho = sum_q w dR_11, dkap = -sum_q w dR_12, dkbar = -sum_q w dR_21 (the blocks
//! of (6.4a)).  Grouping the transitions by their energy Omega = E_k - E_l > 0 (k empty, l occupied), the map from the
//! field amplitudes (dN, dP, dPbar) to the density amplitudes (drho, dkap, dkbar) is
//!     chi(omega) = sum_c [ A_c / (omega - Omega_c) - B_c / (omega + Omega_c) ],
//! with A_c from the pairs (k empty, l occupied) and B_c from the pairs (k occupied, l empty).  The self-energies of a
//! density fluctuation are a linear map F (the kernel of the contact terms of the method), and the self-consistent
//! fluctuations are the solutions of v = chi(omega) F v: a collective mode is a frequency below the threshold
//! min_c Omega_c at which M(omega) = chi(omega) F has the eigenvalue 1, and a static state is unstable when M(0) has an
//! eigenvalue larger than 1.
//!
//! The amplitudes are vectors of NV = 120 complex numbers: the 64 entries of an 8x8 matrix (drho or dN) and the 28
//! entries (i < j) of each of two antisymmetric 8x8 matrices (dkap and dkbar, or dP and dPbar).
use crate::c64::{c, C64};
use crate::cmat::{eigh, CMat};
use crate::dirac::{cmat_c, dirac_basis16, eps2, sigma};
use crate::uniform::Model;

pub const NV: usize = 120;

/// the 28 index pairs (i, j), i < j, of the antisymmetric 8x8 matrices
pub fn antisym_pairs() -> Vec<(usize, usize)> {
    (0..8).flat_map(|i| ((i + 1)..8).map(move |j| (i, j))).collect()
}

/// the vector of an amplitude triple (x general 8x8; a, b antisymmetric 8x8, of which the antisymmetric parts count)
pub fn to_vec(x: &CMat, a: &CMat, b: &CMat) -> Vec<C64> {
    let mut v = Vec::with_capacity(NV);
    for i in 0..8 {
        for j in 0..8 {
            v.push(x.get(i, j));
        }
    }
    for (i, j) in antisym_pairs() {
        v.push((a.get(i, j) - a.get(j, i)).scale(0.5));
    }
    for (i, j) in antisym_pairs() {
        v.push((b.get(i, j) - b.get(j, i)).scale(0.5));
    }
    v
}

/// the amplitude triple of a vector
pub fn from_vec(v: &[C64]) -> (CMat, CMat, CMat) {
    let mut x = CMat::zeros(8);
    let mut a = CMat::zeros(8);
    let mut b = CMat::zeros(8);
    for i in 0..8 {
        for j in 0..8 {
            x.set(i, j, v[8 * i + j]);
        }
    }
    for (s, (i, j)) in antisym_pairs().into_iter().enumerate() {
        a.set(i, j, v[64 + s]);
        a.set(j, i, -v[64 + s]);
        b.set(i, j, v[92 + s]);
        b.set(j, i, -v[92 + s]);
    }
    (x, a, b)
}

fn unit(b: usize) -> Vec<C64> {
    let mut e = vec![C64::ZERO; NV];
    e[b] = C64::ONE;
    e
}

/// the kernel F of the contact terms of the model: the self-energy amplitudes (dN, dP, dPbar) of the density amplitudes
/// (drho, dkap, dkbar), with dPbar(dkbar) = [P(dkbar^dag)]^dag (the adjoint of the pair self-energy)
pub fn kernel(model: &Model) -> CMat {
    let mut f = CMat::new(NV, NV);
    let z = CMat::zeros(8);
    for b in 0..NV {
        let (drho, dk, dkb) = from_vec(&unit(b));
        let (n, _) = model.self_energy_lin(&drho, &z);
        let (_, p) = model.self_energy_lin(&z, &dk);
        let (_, pq) = model.self_energy_lin(&z, &dkb.dagger());
        let col = to_vec(&n, &p, &pq.dagger());
        for a in 0..NV {
            f.set(a, b, col[a]);
        }
    }
    f
}

/// one group of transitions: the energy Omega and the matrices A, B (already weighted with the grid weights)
pub struct Bucket {
    pub omega: f64,
    pub a: CMat,
    pub b: CMat,
}

/// the response map chi(omega) of a uniform state, stored by transition energies
pub struct Response {
    pub buckets: Vec<Bucket>,
    /// the lowest transition energy (the threshold of two quasi-particles at zero total momentum)
    pub threshold: f64,
}

fn extract(r: &CMat) -> Vec<C64> {
    let r11 = r.sub_block(0, 0, 8, 8);
    let r12 = r.sub_block(0, 8, 8, 8).scale_r(-1.0);
    let r21 = r.sub_block(8, 0, 8, 8).scale_r(-1.0);
    to_vec(&r11, &r12, &r21)
}

fn field_matrix(v: &[C64]) -> CMat {
    let (dn, dp, dpb) = from_vec(v);
    CMat::blocks(&dn, &dp, &dpb, &dn.transpose().scale_r(-1.0))
}

fn node_buckets(model: &Model, nodes: &[usize], nself: &CMat, pself: &CMat, fields: &[CMat]) -> Vec<Bucket> {
    let mut out: Vec<Bucket> = vec![];
    for &n in nodes {
        let (ev, v) = eigh(&model.bdg(n, nself, pself));
        let vd = v.dagger();
        let (_, w) = model.grid.node(n);
        let occ: Vec<usize> = (0..16).filter(|&k| ev[k] < 0.0).collect();
        let emp: Vec<usize> = (0..16).filter(|&k| ev[k] >= 0.0).collect();
        let mut clusters: Vec<(f64, Vec<(usize, usize)>)> = vec![];
        for &k in &emp {
            for &l in &occ {
                let om = ev[k] - ev[l];
                match clusters.iter_mut().find(|cl| (cl.0 - om).abs() < 1e-9 * (1.0 + om)) {
                    Some(cl) => cl.1.push((k, l)),
                    None => clusters.push((om, vec![(k, l)])),
                }
            }
        }
        let xs: Vec<CMat> = fields.iter().map(|dh| vd.mul(dh).mul(&v)).collect();
        for (om, pairs) in clusters {
            let mut a = CMat::new(NV, NV);
            let mut b = CMat::new(NV, NV);
            for (ib, x) in xs.iter().enumerate() {
                let mut ya = CMat::zeros(16);
                let mut yb = CMat::zeros(16);
                for &(k, l) in &pairs {
                    ya.set(k, l, x.get(k, l));
                    yb.set(l, k, x.get(l, k));
                }
                let ca = extract(&v.mul(&ya).mul(&vd));
                let cb = extract(&v.mul(&yb).mul(&vd));
                for i in 0..NV {
                    a.set(i, ib, ca[i].scale(w));
                    b.set(i, ib, cb[i].scale(w));
                }
            }
            merge(&mut out, Bucket { omega: om, a, b });
        }
    }
    out
}

fn merge(list: &mut Vec<Bucket>, bk: Bucket) {
    if let Some(e) = list.iter_mut().find(|e| (e.omega - bk.omega).abs() < 1e-9 * (1.0 + bk.omega)) {
        e.a = e.a.add(&bk.a);
        e.b = e.b.add(&bk.b);
    } else {
        list.push(bk);
    }
}

impl Response {
    /// the response of the uniform state of the model with the (momentum-independent) self-energies N, P; the work is
    /// shared by two threads
    pub fn new(model: &Model, nself: &CMat, pself: &CMat) -> Response {
        let fields: Vec<CMat> = (0..NV).map(|b| field_matrix(&unit(b))).collect();
        let nn = model.grid.len();
        let all: Vec<usize> = (0..nn).collect();
        let (h1, h2) = all.split_at(nn / 2);
        let (r1, r2) = std::thread::scope(|s| {
            let t1 = s.spawn(|| node_buckets(model, h1, nself, pself, &fields));
            let t2 = s.spawn(|| node_buckets(model, h2, nself, pself, &fields));
            (t1.join().unwrap(), t2.join().unwrap())
        });
        let mut buckets = r1;
        for bk in r2 {
            merge(&mut buckets, bk);
        }
        buckets.sort_by(|x, y| x.omega.partial_cmp(&y.omega).unwrap());
        let threshold = buckets.first().map(|b| b.omega).unwrap_or(f64::INFINITY);
        Response { buckets, threshold }
    }
    /// chi(omega) (omega real, below the threshold, or complex-free use above it away from the poles)
    pub fn chi(&self, omega: f64) -> CMat {
        let mut m = CMat::new(NV, NV);
        for bk in &self.buckets {
            m = m.add(&bk.a.scale_r(1.0 / (omega - bk.omega))).sub(&bk.b.scale_r(1.0 / (omega + bk.omega)));
        }
        m
    }
    /// chi(omega) at a complex frequency (added for Part IV: at omega = i gamma, gamma > 0, a solution of v = chi F v is
    /// a fluctuation that grows as e^{gamma t}, the unstable mode of a state that is not a minimum)
    pub fn chi_complex(&self, omega: C64) -> CMat {
        let mut m = CMat::new(NV, NV);
        for bk in &self.buckets {
            let dp = C64::ONE / (omega - c(bk.omega, 0.0));
            let dm = C64::ONE / (omega + c(bk.omega, 0.0));
            m = m.add(&bk.a.scale(dp)).sub(&bk.b.scale(dm));
        }
        m
    }
    /// the products chi_c F of every bucket with a kernel, for fast evaluation of M(omega) = chi(omega) F
    pub fn with_kernel(&self, f: &CMat) -> Response {
        let buckets = self.buckets.iter().map(|bk| Bucket { omega: bk.omega, a: bk.a.mul(f), b: bk.b.mul(f) }).collect();
        Response { buckets, threshold: self.threshold }
    }
}

/// named orthogonal bases of the three blocks of an amplitude vector, for the description of modes: the 64 matrices
/// Gamma_A (x) tau_a of the normal block, and the 28 antisymmetric pair matrices (C Gamma_A) (x) (E tau_a)
pub fn named_bases() -> (Vec<(String, CMat)>, Vec<(String, CMat)>) {
    let iso: Vec<(String, CMat)> = (0..4).map(|a| (if a == 0 { "1".to_string() } else { format!("tau{}", a) }, sigma(a))).collect();
    let mut normal = vec![];
    let mut pair = vec![];
    for (na, ga) in dirac_basis16() {
        for (nt, t) in &iso {
            normal.push((format!("{} x {}", na, nt), ga.kron(t)));
            let k = cmat_c().mul(&ga).kron(&eps2().mul(t));
            if k.add(&k.transpose()).max_abs() < 1e-12 {
                pair.push((format!("C {} x E {}", na, nt), k));
            }
        }
    }
    (normal, pair)
}

/// the three largest components of an amplitude vector in the named bases (squared moduli of the normalized
/// projections, with the block: n for drho, k for dkap, kb for dkbar)
pub fn describe(v: &[C64]) -> String {
    let (normal, pair) = named_bases();
    let (x, a, b) = from_vec(v);
    let mut comps: Vec<(f64, String)> = vec![];
    for (name, g) in &normal {
        let p = g.dagger().mul(&x).trace().scale(1.0 / 8.0);
        comps.push((p.norm_sqr(), format!("n[{}]", name)));
    }
    for (name, k) in &pair {
        let p = k.dagger().mul(&a).trace().scale(1.0 / 8.0);
        comps.push((p.norm_sqr(), format!("k[{}]", name)));
        let q = k.mul(&b).trace().scale(1.0 / 8.0);
        comps.push((q.norm_sqr(), format!("kb[{}]", name)));
    }
    let tot: f64 = comps.iter().map(|t| t.0).sum::<f64>().max(1e-300);
    comps.sort_by(|p, q| q.0.partial_cmp(&p.0).unwrap());
    comps.iter().take(4).map(|(w, s)| format!("{} {:.3}", s, w / tot)).collect::<Vec<_>>().join("; ")
}

/// a small helper for the callers: a real number as a complex one
pub fn cr(x: f64) -> C64 {
    c(x, 0.0)
}
