//! Uniform quasi-free states of the spinor field: the mean-field (Hartree-Fock-Bogoliubov) engine of the three methods
//! HS, KS and CJT (Part II of superfluid.md).
//!
//! A uniform quasi-free state is given by its correlation matrices at the momenta of a grid (radial Gauss-Legendre
//! nodes on [0, Lambda] times directions).  For constant (momentum-independent) self-energies N (Hermitian) and P
//! (antisymmetric) the Bogoliubov-de Gennes matrix at the momentum q is
//!     H(q) = ((h(q) + N, P), (P^dag, -(h(-q) + N)^T)),   h(q) = sum_j alpha^j q_j + beta m - mu,
//! and the state is the projector R(q) onto its negative eigenvalues; its blocks are C^n(q) = R_11 and C^p(q) = -R_12
//! (the generalized density ((C^n, -C^p), (-C^p^dag, 1 - C^n(-q)^*))).  The local matrices are
//!     drho = int d^3q/(2pi)^3 (C^n(q) - C^vac(q)),   kap = int d^3q/(2pi)^3 C^p(q),
//! measured from the normal vacuum (C^vac the projector onto the negative eigenvalues of h(q)).
//!
//! The interaction energy per volume is a sum over contact terms, each with the Wick contractions of (6.1), (6.2):
//!   bilinear term nu :(Psi^dag X Psi)(Psi^dag Y Psi):
//!       E = nu [ wH tr(X drho) tr(Y drho) - wF tr(X drho Y drho) - wP tr(kap^* X kap Y^T) ]
//!       N = nu [ wH (X tr(Y drho) + Y tr(X drho)) - wF (X drho Y + Y drho X) ],   P = -nu wP (X kap Y^T + Y kap X^T)
//!   pairing term -(G/4) :Pi^dag Pi:, Pi = Psi^T K Psi:
//!       E = -(G/4) [ wP |tr(K kap^T)|^2 + 2 wF tr(K^dag drho^T K drho) ]
//!       N = -G wF K^dag drho^T K,   P = wP d K^dag with d = -(G/2) tr(K kap^T)
//! where wH, wF, wP are the weights (0 or 1, or a share) with which a method keeps the direct (Hartree), the exchange
//! (Fock) and the pair contraction.  For the Kohn-Sham method the exchange contraction is evaluated with drho replaced
//! by its projection onto the Kohn-Sham densities (span of 1_8 and beta), and its self-energy is projected likewise
//! (the local-density approximation of the exchange).  The first variation is dE = tr(N d drho) - Re tr(P d kap^dag)
//! (checks E-10, E-11 of Part I and P2-1 of Part II).
use crate::c64::{c, C64};
use crate::cmat::{eigh, CMat};
use crate::dirac::{beta, d8, h_free};
use crate::quad::gauss_legendre;
use std::f64::consts::PI;

/// the rule for the directions of the momenta
#[derive(Clone, Copy, Debug)]
pub enum DirRule {
    /// the six directions +-e_1, +-e_2, +-e_3 with equal weights (exact for polynomials of degree at most 3 in the
    /// direction; exact for correlation matrices whose dependence on the direction has the angular momenta 0 and 1)
    Six,
    /// a product rule: Gauss-Legendre in cos(theta) with nt nodes times nphi equally spaced angles phi
    Product(usize, usize),
}

/// the momentum grid of a uniform state
#[derive(Clone, Debug)]
pub struct Grid {
    pub lam: f64,
    pub kr: Vec<f64>,
    pub wr: Vec<f64>,
    pub dirs: Vec<[f64; 3]>,
    pub wdir: Vec<f64>,
}

impl Grid {
    pub fn new(nr: usize, lam: f64, rule: DirRule) -> Grid {
        let (x, w) = gauss_legendre(nr);
        let kr: Vec<f64> = x.iter().map(|t| 0.5 * lam * (t + 1.0)).collect();
        let wr: Vec<f64> = w.iter().map(|t| 0.5 * lam * t).collect();
        let (dirs, wdir) = match rule {
            DirRule::Six => {
                let d = vec![[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, -1.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, -1.0]];
                (d, vec![1.0 / 6.0; 6])
            }
            DirRule::Product(nt, nphi) => {
                let (ct, wt) = gauss_legendre(nt);
                let mut d = vec![];
                let mut wd = vec![];
                for (cth, wth) in ct.iter().zip(&wt) {
                    let sth = (1.0 - cth * cth).max(0.0).sqrt();
                    for k in 0..nphi {
                        let ph = 2.0 * PI * (k as f64 + 0.5) / nphi as f64;
                        d.push([sth * ph.cos(), sth * ph.sin(), *cth]);
                        wd.push(0.5 * wth / nphi as f64);
                    }
                }
                (d, wd)
            }
        };
        Grid { lam, kr, wr, dirs, wdir }
    }
    /// the number of momentum nodes
    pub fn len(&self) -> usize {
        self.kr.len() * self.dirs.len()
    }
    /// the momentum vector and the weight of the node n (radial index n / ndir, direction index n % ndir); the weights
    /// integrate int d^3q/(2pi)^3
    pub fn node(&self, n: usize) -> ([f64; 3], f64) {
        let (ir, id) = (n / self.dirs.len(), n % self.dirs.len());
        let q = self.kr[ir];
        let d = self.dirs[id];
        ([q * d[0], q * d[1], q * d[2]], self.wr[ir] * q * q / (2.0 * PI * PI) * self.wdir[id])
    }
}

/// a contact term of the Hamiltonian density
#[derive(Clone, Debug)]
pub enum TermKind {
    /// nu :(Psi^dag X Psi)(Psi^dag Y Psi):
    Bilinear { nu: f64, x: CMat, y: CMat },
    /// -(G/4) :Pi^dag Pi: with Pi = Psi^T K Psi
    Pairing { g: f64, k: CMat },
}

/// a contact term with the weights of its three contractions in a method
#[derive(Clone, Debug)]
pub struct Term {
    pub kind: TermKind,
    pub w_h: f64,
    pub w_f: f64,
    pub w_p: f64,
    /// Kohn-Sham: the exchange contraction in the local-density approximation of the Kohn-Sham densities
    pub ks_projected: bool,
    pub label: String,
}

/// the projection of an 8x8 matrix onto the span of 1_8 and beta (the Kohn-Sham densities n and n_S)
pub fn ks_project(m: &CMat) -> CMat {
    let b = d8(&beta());
    let one = CMat::identity(8);
    let a0 = m.trace().scale(1.0 / 8.0);
    let a1 = b.mul(m).trace().scale(1.0 / 8.0);
    one.scale(a0).add(&b.scale(a1))
}

impl Term {
    /// the energy per volume of the term and its self-energies (N, P) for the local matrices drho, kap
    pub fn eval(&self, drho: &CMat, kap: &CMat) -> (f64, CMat, CMat) {
        match &self.kind {
            TermKind::Bilinear { nu, x, y } => {
                let nu = *nu;
                let rf = if self.ks_projected { ks_project(drho) } else { drho.clone() };
                let (tx, ty) = (x.mul(drho).trace(), y.mul(drho).trace());
                let e_h = (tx * ty).re;
                let e_f = x.mul(&rf).mul(y).mul(&rf).trace().re;
                let e_p = kap.conj().mul(x).mul(kap).mul(&y.transpose()).trace().re;
                let energy = nu * (self.w_h * e_h - self.w_f * e_f - self.w_p * e_p);
                let n_h = x.scale(ty).add(&y.scale(tx));
                let mut n_f = x.mul(&rf).mul(y).add(&y.mul(&rf).mul(x));
                if self.ks_projected {
                    n_f = ks_project(&n_f);
                }
                let nmat = n_h.scale_r(nu * self.w_h).sub(&n_f.scale_r(nu * self.w_f));
                let pmat = x.mul(kap).mul(&y.transpose()).add(&y.mul(kap).mul(&x.transpose())).scale_r(-nu * self.w_p);
                (energy, nmat, pmat)
            }
            TermKind::Pairing { g, k } => {
                let g = *g;
                let rf = if self.ks_projected { ks_project(drho) } else { drho.clone() };
                let t = k.mul(&kap.transpose()).trace();
                let e_p = t.norm_sqr();
                let e_n = k.dagger().mul(&rf.transpose()).mul(k).mul(&rf).trace().re;
                let energy = -0.25 * g * (self.w_p * e_p + 2.0 * self.w_f * e_n);
                let mut n_n = k.dagger().mul(&rf.transpose()).mul(k).scale_r(-g * self.w_f);
                if self.ks_projected {
                    n_n = ks_project(&n_n);
                }
                let d = t.scale(-0.5 * g);
                let pmat = k.dagger().scale(d).scale_r(self.w_p);
                (energy, n_n, pmat)
            }
        }
    }
}

/// the result of a self-consistent solution
#[derive(Clone, Debug)]
pub struct Solution {
    pub n_self: CMat,
    pub p_self: CMat,
    pub drho: CMat,
    pub kap: CMat,
    /// the energy per volume measured from the normal vacuum (kinetic, interaction, and the external source if any)
    pub energy: f64,
    pub e_kin: f64,
    pub e_int: f64,
    pub iterations: usize,
    pub residual: f64,
    /// the gap: the lowest quasi-particle energy over all momenta below the cutoff (Model::gap)
    pub gap: f64,
}

/// a uniform model: mass, chemical potential, contact terms, grid, and an optional external pair source
/// (the term lambda (Pi^3 + Pi^3^dag) of the Hamiltonian density, used to compute the effective potential at a fixed
/// pair amplitude)
pub struct Model {
    pub m: f64,
    pub mu: f64,
    pub terms: Vec<Term>,
    pub grid: Grid,
    pub ext_pair: Option<(CMat, f64)>,
    /// an external one-body term Psi^dag X Psi of the Hamiltonian density (8x8 Hermitian X): a uniform gauge potential,
    /// or minus a chemical potential times a charge matrix (used for the susceptibilities and the gauge-boson masses)
    pub ext_n: Option<CMat>,
    vac: Vec<CMat>,
    hq: Vec<CMat>,
    hmq: Vec<CMat>,
}

impl Model {
    pub fn new(m: f64, mu: f64, terms: Vec<Term>, grid: Grid) -> Model {
        let mut vac = vec![];
        let mut hq = vec![];
        let mut hmq = vec![];
        for n in 0..grid.len() {
            let (q, _) = grid.node(n);
            let h = h_free(&q, m);
            let eps = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + m * m).sqrt();
            vac.push(CMat::identity(8).sub(&h.scale_r(1.0 / eps)).scale_r(0.5));
            hq.push(h.sub(&CMat::identity(8).scale_r(mu)));
            hmq.push(h_free(&[-q[0], -q[1], -q[2]], m).sub(&CMat::identity(8).scale_r(mu)));
        }
        Model { m, mu, terms, grid, ext_pair: None, ext_n: None, vac, hq, hmq }
    }
    /// the BdG matrix at the node n for the self-energies (N, P) (momentum-independent)
    pub fn bdg(&self, n: usize, nself: &CMat, pself: &CMat) -> CMat {
        let a = self.hq[n].add(nself);
        let d = self.hmq[n].add(nself).transpose().scale_r(-1.0);
        CMat::blocks(&a, pself, &pself.dagger(), &d)
    }
    /// the correlation matrices C^n(q), C^p(q) of the ground state of the BdG matrices, and the lowest positive eigenvalue
    pub fn states(&self, nself: &CMat, pself: &CMat) -> (Vec<CMat>, Vec<CMat>, f64) {
        let mut cn = vec![];
        let mut cp = vec![];
        let mut gap = f64::INFINITY;
        for n in 0..self.grid.len() {
            let (ev, v) = eigh(&self.bdg(n, nself, pself));
            let mut r = CMat::zeros(16);
            for k in 0..16 {
                if ev[k] < 0.0 {
                    for a in 0..16 {
                        let va = v.get(a, k);
                        for b in 0..16 {
                            let idx = a * 16 + b;
                            r.a[idx] += va * v.get(b, k).conj();
                        }
                    }
                } else {
                    gap = gap.min(ev[k]);
                }
            }
            cn.push(r.sub_block(0, 0, 8, 8));
            cp.push(r.sub_block(0, 8, 8, 8).scale_r(-1.0));
        }
        (cn, cp, gap)
    }
    /// the local matrices drho (from the normal vacuum) and kap, and the kinetic energy per volume from the normal vacuum
    pub fn local(&self, cn: &[CMat], cp: &[CMat]) -> (CMat, CMat, f64) {
        let mut drho = CMat::zeros(8);
        let mut kap = CMat::zeros(8);
        let mut ek = 0.0;
        for n in 0..self.grid.len() {
            let (_, w) = self.grid.node(n);
            let d = cn[n].sub(&self.vac[n]);
            drho = drho.add(&d.scale_r(w));
            kap = kap.add(&cp[n].scale_r(w));
            ek += w * self.hq[n].mul(&d).trace().re;
        }
        (drho, kap, ek)
    }
    /// the interaction energy and the self-energies of the local matrices (with the external pair source, if any)
    pub fn interaction(&self, drho: &CMat, kap: &CMat) -> (f64, CMat, CMat) {
        let mut e = 0.0;
        let mut nself = CMat::zeros(8);
        let mut pself = CMat::zeros(8);
        for t in &self.terms {
            let (et, nt, pt) = t.eval(drho, kap);
            e += et;
            nself = nself.add(&nt);
            pself = pself.add(&pt);
        }
        if let Some((k, lam)) = &self.ext_pair {
            // lambda (Pi + Pi^dag) with <Pi> = tr(K kap^T): energy 2 lambda Re tr(K kap^T); its pair self-energy
            // from dE = -Re tr(P dkap^dag): P = -2 lambda K^* (antisymmetric part), i.e. 2 lambda K^dag
            let t = k.mul(&kap.transpose()).trace();
            e += 2.0 * lam * t.re;
            pself = pself.add(&k.dagger().scale_r(2.0 * lam));
        }
        if let Some(x) = &self.ext_n {
            // the expectation value tr(X drho), measured from the normal vacuum (the normal order of the external term,
            // as for the contact terms), and the constant normal self-energy X
            e += x.mul(drho).trace().re;
            nself = nself.add(x);
        }
        (e, nself, pself)
    }
    /// the self-energies of the contact terms alone (no external terms), as complex-linear functions of drho and kap;
    /// used by the linear response, where drho and kap are the (complex) amplitudes of a fluctuation
    pub fn self_energy_lin(&self, drho: &CMat, kap: &CMat) -> (CMat, CMat) {
        let mut nself = CMat::zeros(8);
        let mut pself = CMat::zeros(8);
        for t in &self.terms {
            let (_, nt, pt) = t.eval(drho, kap);
            nself = nself.add(&nt);
            pself = pself.add(&pt);
        }
        (nself, pself)
    }
    /// the self-consistent solution from the initial self-energies, by linear mixing of the self-energies
    pub fn solve(&self, n0: &CMat, p0: &CMat, mix: f64, tol: f64, maxit: usize) -> Solution {
        let mut nself = n0.clone();
        let mut pself = p0.clone();
        let mut it = 0;
        loop {
            let (cn, cp, gap) = self.states(&nself, &pself);
            let (drho, kap, ek) = self.local(&cn, &cp);
            let (ei, n2, p2) = self.interaction(&drho, &kap);
            let res = n2.sub(&nself).max_abs().max(p2.sub(&pself).max_abs());
            if res < tol || it >= maxit {
                // the state of the final self-energies (consistent within the residual)
                let _ = gap;
                let gap = self.gap(&n2, &p2).0;
                return Solution { n_self: n2, p_self: p2, drho, kap, energy: ek + ei, e_kin: ek, e_int: ei, iterations: it, residual: res, gap };
            }
            nself = n2.scale_r(mix).add(&nself.scale_r(1.0 - mix));
            pself = p2.scale_r(mix).add(&pself.scale_r(1.0 - mix));
            // keep the symmetries of the self-energies against rounding: N Hermitian, P antisymmetric
            nself = nself.add(&nself.dagger()).scale_r(0.5);
            pself = pself.sub(&pself.transpose()).scale_r(0.5);
            it += 1;
        }
    }
    /// the lowest quasi-particle energy at the momentum q (the smallest absolute eigenvalue of the BdG matrix, whose
    /// spectrum is symmetric about zero), for momentum-independent self-energies
    pub fn qp_energy(&self, q: &[f64; 3], nself: &CMat, pself: &CMat) -> f64 {
        let one = CMat::identity(8);
        let a = h_free(q, self.m).sub(&one.scale_r(self.mu)).add(nself);
        let d = h_free(&[-q[0], -q[1], -q[2]], self.m).sub(&one.scale_r(self.mu)).add(nself).transpose().scale_r(-1.0);
        let (ev, _) = eigh(&CMat::blocks(&a, pself, &pself.dagger(), &d));
        ev.iter().fold(f64::INFINITY, |s, e| s.min(e.abs()))
    }
    /// the gap of the state: the minimum of the quasi-particle energy over the momenta 0 <= |q| <= Lambda along e_3
    /// (the uniform states are isotropic): a scan of 201 momenta refined by a golden-section search; returns the gap and
    /// the momentum of the minimum
    pub fn gap(&self, nself: &CMat, pself: &CMat) -> (f64, f64) {
        let f = |x: f64| self.qp_energy(&[0.0, 0.0, x], nself, pself);
        let nscan = 200;
        let lam = self.grid.lam;
        let mut best = (f(0.0), 0usize);
        for i in 1..=nscan {
            let v = f(lam * i as f64 / nscan as f64);
            if v < best.0 {
                best = (v, i);
            }
        }
        let (mut a, mut b) = (lam * (best.1.max(1) - 1) as f64 / nscan as f64, lam * (best.1 + 1).min(nscan) as f64 / nscan as f64);
        if best.1 == 0 {
            a = 0.0;
            b = lam / nscan as f64;
        }
        let gr = 0.5 * (5f64.sqrt() - 1.0);
        let (mut x1, mut x2) = (b - gr * (b - a), a + gr * (b - a));
        let (mut f1, mut f2) = (f(x1), f(x2));
        for _ in 0..80 {
            if f1 < f2 {
                b = x2;
                x2 = x1;
                f2 = f1;
                x1 = b - gr * (b - a);
                f1 = f(x1);
            } else {
                a = x1;
                x1 = x2;
                f1 = f2;
                x2 = a + gr * (b - a);
                f2 = f(x2);
            }
        }
        let xm = 0.5 * (a + b);
        let cands = [(f(0.0), 0.0), (f(xm), xm), (best.0, lam * best.1 as f64 / nscan as f64)];
        cands.iter().fold((f64::INFINITY, 0.0), |s, &(v, x)| if v < s.0 { (v, x) } else { s })
    }
    /// the energy per volume of the ground state of given self-energies (not necessarily self-consistent): the
    /// expectation value of the Hamiltonian in that quasi-free state
    pub fn energy_of(&self, nself: &CMat, pself: &CMat) -> (f64, CMat, CMat) {
        let (cn, cp, _) = self.states(nself, pself);
        let (drho, kap, ek) = self.local(&cn, &cp);
        let (ei, _, _) = self.interaction(&drho, &kap);
        (ek + ei, drho, kap)
    }
}

/// the scalar density n_S = tr(beta drho), the number density n = tr(drho), and the order parameter
/// d^iota = -(G/2) tr(K^iota kap^T) for the given pair matrices
pub fn densities(drho: &CMat, kap: &CMat, g: f64, ks: &[CMat]) -> (f64, f64, Vec<C64>) {
    let ns = d8(&beta()).mul(drho).trace().re;
    let n = drho.trace().re;
    let d = ks.iter().map(|k| k.mul(&kap.transpose()).trace().scale(-0.5 * g)).collect();
    (ns, n, d)
}

/// the Dirac mass of a state: m plus the beta-component of the normal self-energy, tr(beta N)/8
pub fn dirac_mass(m: f64, nself: &CMat) -> f64 {
    m + d8(&beta()).mul(nself).trace().re / 8.0
}

/// a small complex helper
pub fn cr(x: f64) -> C64 {
    c(x, 0.0)
}
