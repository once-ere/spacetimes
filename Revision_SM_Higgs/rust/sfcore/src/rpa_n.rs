//! The linear response at a frequency of a uniform quasi-free state of several Dirac fields (module `uniform_n`): the
//! collective modes at zero total momentum in the random-phase approximation of the time-dependent mean field, for the
//! revised hypotheses of the folder Revision_SM_Higgs (stage 4: the amplitude mode of the Higgs superfluid).
//!
//! This module generalizes the module `rpa` (written for the eight components of the earlier work, unchanged) to a
//! field of n components and to a chosen sector of the amplitudes.  A uniform fluctuation with the time factor
//! e^{-i omega t} changes the BdG matrix of `uniform_n` by
//!     dH = ((dN, dP), (dPbar, -dN^T)),
//! where dN (a general complex n x n matrix), dP and dPbar (antisymmetric) are the complex amplitudes of the normal
//! self-energy, of the pair self-energy and of its adjoint (at a real frequency dPbar is not the adjoint of dP: the
//! two amplitudes of e^{-i omega t} and e^{+i omega t} are independent).  Linearizing the equation of motion of the
//! generalized density R(q), i dR/dt = [H(q), R(q)], gives at each momentum, in the eigenbasis (E_k, |k>) of H(q),
//!     dR_kl = dH_kl (f_l - f_k) / (omega - E_k + E_l),        f = 1 for E < 0, else 0,
//! and the local amplitudes drho = sum_q w dR_11, dkap = -sum_q w dR_12, dkbar = -sum_q w dR_21.  Grouping the
//! transitions by their energy Omega = E_k - E_l > 0 (k empty, l occupied) gives
//!     chi(omega) = sum_c [ A_c / (omega - Omega_c) - B_c / (omega + Omega_c) ],
//! A_c from the pairs (k empty, l occupied) and B_c from the pairs (k occupied, l empty).  The contact terms map the
//! density amplitudes to the field amplitudes linearly (the kernel F, with dPbar(dkbar) = [P(dkbar^dag)]^dag), and a
//! collective mode is a frequency at which 1 - chi(omega) F is singular.  At omega = 0 this is the static response of
//! `uniform_n::response_eigs` (eigenvalue 1); a broken symmetry gives a singular 1 - chi(0) F (a Goldstone mode).
//!
//! The sector.  The amplitudes are restricted to a list of normal index pairs (i, j) (entries of dN and drho) and a
//! list of pair index pairs (i, j), i < j (entries of the antisymmetric dP, dkap, dPbar, dkbar).  A sector closed under
//! the symmetries of the state (for example all amplitudes of relative number 0) gives the exact response in it.
//!
//! Added on 22 November 2026 (decision D36): the pair-pair terms of `uniform_n` (the determinant interaction), whose
//! pair self-energy is P = L(kap^*) with a complex-linear map L.  Since dkbar is the amplitude of kap^dag = -kap^* (kap is
//! antisymmetric), the amplitude of kap^* at e^{-i omega t} is -dkbar, and that of kap at e^{+i omega t} is dkbar^dag;
//! so these terms add dP = L(-dkbar) and dPbar = [L(dkap^*)]^dag to the kernel.  At omega = 0, for a Hermitian
//! perturbation (dkbar = dkap^dag), dP = L(dkap^*), the static self-energy.  A model without these terms gives the
//! kernel of before, bit for bit.
use crate::c64::{c, C64};
use crate::cmat::{eigh, CMat};
use crate::uniform_n::ModelN;

/// the sector of the amplitudes: the normal entries (i, j) and the pair entries (i, j) with i < j
#[derive(Clone, Debug)]
pub struct SectorN {
    pub dim: usize,
    pub normal: Vec<(usize, usize)>,
    pub pair: Vec<(usize, usize)>,
}

impl SectorN {
    /// the sector of all normal entries within the index sets of `blocks_n` (each set with itself) and all pair entries
    /// between the index sets of `pairs` (each a pair of sets)
    pub fn new(dim: usize, blocks_n: &[Vec<usize>], pairs: &[(Vec<usize>, Vec<usize>)]) -> SectorN {
        let mut normal = vec![];
        for set in blocks_n {
            for &i in set {
                for &j in set {
                    normal.push((i, j));
                }
            }
        }
        let mut pair = vec![];
        for (a, b) in pairs {
            for &i in a {
                for &j in b {
                    if i == j {
                        continue;
                    }
                    let p = if i < j { (i, j) } else { (j, i) };
                    if !pair.contains(&p) {
                        pair.push(p);
                    }
                }
            }
        }
        SectorN { dim, normal, pair }
    }
    /// the number of complex amplitudes: the normal entries and twice the pair entries (dP and dPbar)
    pub fn nv(&self) -> usize {
        self.normal.len() + 2 * self.pair.len()
    }
    /// the vector of an amplitude triple (x general; a, b antisymmetric, of which the antisymmetric parts count)
    pub fn to_vec(&self, x: &CMat, a: &CMat, b: &CMat) -> Vec<C64> {
        let mut v = Vec::with_capacity(self.nv());
        for &(i, j) in &self.normal {
            v.push(x.get(i, j));
        }
        for &(i, j) in &self.pair {
            v.push((a.get(i, j) - a.get(j, i)).scale(0.5));
        }
        for &(i, j) in &self.pair {
            v.push((b.get(i, j) - b.get(j, i)).scale(0.5));
        }
        v
    }
    /// the amplitude triple of a vector
    pub fn from_vec(&self, v: &[C64]) -> (CMat, CMat, CMat) {
        let n = self.dim;
        let (mut x, mut a, mut b) = (CMat::zeros(n), CMat::zeros(n), CMat::zeros(n));
        let nn = self.normal.len();
        let np = self.pair.len();
        for (s, &(i, j)) in self.normal.iter().enumerate() {
            x.set(i, j, v[s]);
        }
        for (s, &(i, j)) in self.pair.iter().enumerate() {
            a.set(i, j, v[nn + s]);
            a.set(j, i, -v[nn + s]);
            b.set(i, j, v[nn + np + s]);
            b.set(j, i, -v[nn + np + s]);
        }
        (x, a, b)
    }
    fn unit(&self, b: usize) -> Vec<C64> {
        let mut e = vec![C64::ZERO; self.nv()];
        e[b] = C64::ONE;
        e
    }
    /// the BdG perturbation ((dN, dP), (dPbar, -dN^T)) of an amplitude vector
    pub fn field_matrix(&self, v: &[C64]) -> CMat {
        let (dn, dp, dpb) = self.from_vec(v);
        CMat::blocks(&dn, &dp, &dpb, &dn.transpose().scale_r(-1.0))
    }
    /// the density amplitudes (drho, dkap, dkbar) of a change dR of the generalized density, as a vector
    fn extract(&self, r: &CMat) -> Vec<C64> {
        let n = self.dim;
        let r11 = r.sub_block(0, 0, n, n);
        let r12 = r.sub_block(0, n, n, n).scale_r(-1.0);
        let r21 = r.sub_block(n, 0, n, n).scale_r(-1.0);
        self.to_vec(&r11, &r12, &r21)
    }
}

/// the kernel F of the contact terms of a model in a sector: the field amplitudes (dN, dP, dPbar) produced by the
/// density amplitudes (drho, dkap, dkbar), with dPbar(dkbar) = [P(dkbar^dag)]^dag
pub fn kernel_n(model: &ModelN, sec: &SectorN) -> CMat {
    let nv = sec.nv();
    let z = CMat::zeros(sec.dim);
    let mut f = CMat::new(nv, nv);
    for b in 0..nv {
        let (drho, dk, dkb) = sec.from_vec(&sec.unit(b));
        let (n, _) = model.self_energy_lin_terms(&drho, &z);
        let (_, mut p) = model.self_energy_lin_terms(&z, &dk);
        let (_, pq) = model.self_energy_lin_terms(&z, &dkb.dagger());
        let mut pbar = pq.dagger();
        if !model.pair_pair.is_empty() {
            // the pair-pair terms: dP = L(-dkbar), dPbar = [L(dkap^*)]^dag
            p = p.add(&model.pair_conj_lin(&dkb.scale_r(-1.0)));
            pbar = pbar.add(&model.pair_conj_lin(&dk.conj()).dagger());
        }
        let col = sec.to_vec(&n, &p, &pbar);
        for a in 0..nv {
            f.set(a, b, col[a]);
        }
    }
    f
}

/// one group of transitions of equal energy Omega, with its matrices A and B (weighted with the grid weights)
pub struct BucketN {
    pub omega: f64,
    pub a: CMat,
    pub b: CMat,
}

/// the response map chi(omega) of a uniform state in a sector, stored by the transition energies
pub struct ResponseN {
    pub buckets: Vec<BucketN>,
    /// the lowest transition energy that contributes to the sector (the threshold of two quasi-particles)
    pub threshold: f64,
}

fn merge(list: &mut Vec<BucketN>, bk: BucketN) {
    if let Some(e) = list.iter_mut().find(|e| (e.omega - bk.omega).abs() < 1e-9 * (1.0 + bk.omega)) {
        e.a = e.a.add(&bk.a);
        e.b = e.b.add(&bk.b);
    } else {
        list.push(bk);
    }
}

fn node_buckets(model: &ModelN, nodes: &[usize], nself: &CMat, pself: &CMat, sec: &SectorN, fields: &[CMat]) -> Vec<BucketN> {
    let m2 = 2 * model.dim;
    let nv = sec.nv();
    let mut out: Vec<BucketN> = vec![];
    for &n in nodes {
        let (ev, v) = eigh(&model.bdg(n, nself, pself));
        let vd = v.dagger();
        let (_, w) = model.grid.node(n);
        let occ: Vec<usize> = (0..m2).filter(|&k| ev[k] < 0.0).collect();
        let emp: Vec<usize> = (0..m2).filter(|&k| ev[k] >= 0.0).collect();
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
            let mut a = CMat::new(nv, nv);
            let mut b = CMat::new(nv, nv);
            let mut any = false;
            for (ib, x) in xs.iter().enumerate() {
                let mut ya = CMat::zeros(m2);
                let mut yb = CMat::zeros(m2);
                let mut nz = false;
                for &(k, l) in &pairs {
                    let (xa, xb) = (x.get(k, l), x.get(l, k));
                    if xa.norm_sqr() + xb.norm_sqr() > 1e-30 {
                        nz = true;
                    }
                    ya.set(k, l, xa);
                    yb.set(l, k, xb);
                }
                if !nz {
                    continue;
                }
                any = true;
                let ca = sec.extract(&v.mul(&ya).mul(&vd));
                let cb = sec.extract(&v.mul(&yb).mul(&vd));
                for i in 0..nv {
                    a.set(i, ib, ca[i].scale(w));
                    b.set(i, ib, cb[i].scale(w));
                }
            }
            if any {
                merge(&mut out, BucketN { omega: om, a, b });
            }
        }
    }
    out
}

impl ResponseN {
    /// the response of the uniform state of a model with the (momentum-independent) self-energies N, P in a sector; the
    /// nodes of the grid are shared by `threads` threads
    pub fn new(model: &ModelN, nself: &CMat, pself: &CMat, sec: &SectorN, threads: usize) -> ResponseN {
        let fields: Vec<CMat> = (0..sec.nv()).map(|b| sec.field_matrix(&sec.unit(b))).collect();
        let nn = model.grid.len();
        let all: Vec<usize> = (0..nn).collect();
        let nt = threads.max(1);
        let chunk = (nn + nt - 1) / nt;
        let parts: Vec<Vec<BucketN>> = std::thread::scope(|s| {
            let hs: Vec<_> = all
                .chunks(chunk)
                .map(|ch| {
                    let f = &fields;
                    s.spawn(move || node_buckets(model, ch, nself, pself, sec, f))
                })
                .collect();
            hs.into_iter().map(|h| h.join().unwrap()).collect()
        });
        let mut buckets: Vec<BucketN> = vec![];
        for part in parts {
            for bk in part {
                merge(&mut buckets, bk);
            }
        }
        buckets.sort_by(|x, y| x.omega.partial_cmp(&y.omega).unwrap());
        let threshold = buckets.first().map(|b| b.omega).unwrap_or(f64::INFINITY);
        ResponseN { buckets, threshold }
    }
    /// chi(omega) at a real frequency away from the transition energies
    pub fn chi(&self, omega: f64) -> CMat {
        let nv = self.buckets.first().map(|b| b.a.rows).unwrap_or(0);
        let mut m = CMat::new(nv, nv);
        for bk in &self.buckets {
            m = m.add(&bk.a.scale_r(1.0 / (omega - bk.omega))).sub(&bk.b.scale_r(1.0 / (omega + bk.omega)));
        }
        m
    }
    /// the products A_c F and B_c F of every bucket with a kernel, so that chi(omega) of the result is chi(omega) F
    pub fn with_kernel(&self, f: &CMat) -> ResponseN {
        let buckets = self.buckets.iter().map(|bk| BucketN { omega: bk.omega, a: bk.a.mul(f), b: bk.b.mul(f) }).collect();
        ResponseN { buckets, threshold: self.threshold }
    }
    /// the smallest singular value of 1 - chi(omega) F (the response must have been combined with F by
    /// `with_kernel`) and its right singular vector: zero at a collective mode
    pub fn smallest_singular(&self, omega: f64) -> (f64, Vec<C64>) {
        let m = self.chi(omega);
        let nv = m.rows;
        let a = CMat::identity(nv).sub(&m);
        let (ev, v) = eigh(&a.dagger().mul(&a));
        let k = (0..nv).fold(0, |b, i| if ev[i] < ev[b] { i } else { b });
        ((ev[k].max(0.0)).sqrt(), (0..nv).map(|i| v.get(i, k)).collect())
    }
    /// the number of singular values of 1 - chi(omega) F below a bound (used at omega = 0 to count the Goldstone modes)
    pub fn count_small_singular(&self, omega: f64, bound: f64) -> (usize, Vec<f64>) {
        let m = self.chi(omega);
        let nv = m.rows;
        let a = CMat::identity(nv).sub(&m);
        let (ev, _) = eigh(&a.dagger().mul(&a));
        let mut s: Vec<f64> = ev.iter().map(|x| x.max(0.0).sqrt()).collect();
        s.sort_by(|p, q| p.partial_cmp(q).unwrap());
        (s.iter().filter(|x| **x < bound).count(), s)
    }
}

/// a small helper: a real number as a complex one
pub fn cr(x: f64) -> C64 {
    c(x, 0.0)
}
