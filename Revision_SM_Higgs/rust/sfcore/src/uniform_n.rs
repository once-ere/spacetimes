//! Uniform quasi-free states of a field of several Dirac fields: the mean-field (Hartree-Fock-Bogoliubov) engine of the
//! methods HS, KS and CJT for the revised hypotheses of the folder Revision_SM_Higgs (the isodoublet Psi_LL, eight
//! components, and the singlet chi_SH, four components: twelve components).
//!
//! This module generalizes the module `uniform` (written for the eight components of Psi_LL alone) to a field made of
//! blocks: each block is a Dirac field of four components (a singlet) or eight components (an isodoublet, component
//! 2 d + s), with its own mass.  The module `uniform` is unchanged; check SF-1 of the program sh_superfluid compares the
//! two engines on the earlier model, with the singlet block decoupled.
//!
//! A uniform quasi-free state is given by its correlation matrices at the momenta of a grid.  For constant
//! (momentum-independent) self-energies N (Hermitian, n x n) and P (antisymmetric, n x n) the Bogoliubov-de Gennes
//! matrix at the momentum q is
//!     H(q) = ((h(q) + N, P), (P^dag, -(h(-q) + N)^T)),   h(q) = diag over the blocks of (alpha.q + beta m_block) - mu,
//! and the ground state is the projector R(q) onto its negative eigenvalues: C^n(q) = R_11, C^p(q) = -R_12.  The local
//! matrices are drho = int d^3q/(2 pi)^3 (C^n(q) - C^vac(q)) (measured from the normal vacuum, C^vac(q) the projector
//! onto the negative eigenvalues of h(q)) and kap = int d^3q/(2 pi)^3 C^p(q).
//!
//! The interaction energy per volume is the sum over contact terms with the Wick contractions of the module `uniform`:
//!   bilinear term nu :(Psi^dag X Psi)(Psi^dag Y Psi): (here Psi is the whole field):
//!       E = nu [ wH tr(X drho) tr(Y drho) - wF tr(X drho Y drho) - wP tr(kap^* X kap Y^T) ]
//!       N = nu [ wH (X tr(Y drho) + Y tr(X drho)) - wF (X drho Y + Y drho X) ],   P = -nu wP (X kap Y^T + Y kap X^T)
//!   pairing term -(G/4) :Pi^dag Pi:, Pi = Psi^T K Psi (K antisymmetric):
//!       E = -(G/4) [ wP |tr(K kap^T)|^2 + 2 wF tr(K^dag drho^T K drho) ]
//!       N = -G wF K^dag drho^T K,   P = wP d K^dag with d = -(G/2) tr(K kap^T)
//! For the Kohn-Sham method the exchange contraction is evaluated with drho replaced by its orthogonal projection onto
//! the span of a list of density matrices (the Kohn-Sham densities), and its self-energy is projected likewise.
use crate::c64::{c, C64};
use crate::cmat::{eigh, CMat};
use crate::dirac::{alpha, beta, h_free};
use crate::uniform::Grid;

/// one Dirac field of the whole field: its first component, its number of components (4: a singlet; 8: an isodoublet,
/// component 2 d + s with the Dirac index d and the isospin index s) and its mass
#[derive(Clone, Copy, Debug)]
pub struct FieldBlock {
    pub off: usize,
    pub ncomp: usize,
    pub mass: f64,
}

/// the free Dirac Hamiltonian of a singlet (4 x 4) on a plane wave of momentum q: alpha.q + beta m
pub fn h_free4(q: &[f64; 3], m: f64) -> CMat {
    let mut h = beta().scale_r(m);
    for j in 0..3 {
        h = h.add(&alpha(j + 1).scale_r(q[j]));
    }
    h
}

/// the free Hamiltonian of the whole field (block-diagonal) and the projector onto its negative eigenvalues
pub fn h_and_vac(blocks: &[FieldBlock], dim: usize, q: &[f64; 3]) -> (CMat, CMat) {
    let mut h = CMat::zeros(dim);
    let mut vac = CMat::zeros(dim);
    let q2 = q[0] * q[0] + q[1] * q[1] + q[2] * q[2];
    for b in blocks {
        let hb = match b.ncomp {
            4 => h_free4(q, b.mass),
            8 => h_free(q, b.mass),
            n => panic!("a block of {} components is not a Dirac field of this engine", n),
        };
        let eps = (q2 + b.mass * b.mass).sqrt();
        let vb = CMat::identity(b.ncomp).sub(&hb.scale_r(1.0 / eps)).scale_r(0.5);
        h.put(b.off, b.off, &hb);
        vac.put(b.off, b.off, &vb);
    }
    (h, vac)
}

/// the orthogonal projection of a matrix onto the span of the (mutually orthogonal, nonzero) matrices of a basis, in the
/// inner product tr(A^dag B)
pub fn project_onto(m: &CMat, basis: &[CMat]) -> CMat {
    let mut out = CMat::zeros(m.rows);
    for x in basis {
        let nrm = x.dagger().mul(x).trace().re;
        let cf = x.dagger().mul(m).trace().scale(1.0 / nrm);
        out = out.add(&x.scale(cf));
    }
    out
}

/// a contact term of the Hamiltonian density
#[derive(Clone, Debug)]
pub enum TermKindN {
    /// nu :(Psi^dag X Psi)(Psi^dag Y Psi):
    Bilinear { nu: f64, x: CMat, y: CMat },
    /// -(G/4) :Pi^dag Pi: with Pi = Psi^T K Psi
    Pairing { g: f64, k: CMat },
}

/// a contact term with the weights of its three contractions in a method, and for KS the basis of the Kohn-Sham
/// densities onto which the exchange contraction is projected
#[derive(Clone, Debug)]
pub struct TermN {
    pub kind: TermKindN,
    pub w_h: f64,
    pub w_f: f64,
    pub w_p: f64,
    pub ks_basis: Option<Vec<CMat>>,
    pub label: String,
}

impl TermN {
    fn proj(&self, m: &CMat) -> CMat {
        match &self.ks_basis {
            Some(b) => project_onto(m, b),
            None => m.clone(),
        }
    }
    /// the energy per volume of the term and its self-energies (N, P) for the local matrices drho, kap
    pub fn eval(&self, drho: &CMat, kap: &CMat) -> (f64, CMat, CMat) {
        match &self.kind {
            TermKindN::Bilinear { nu, x, y } => {
                let nu = *nu;
                let rf = self.proj(drho);
                let (tx, ty) = (x.mul(drho).trace(), y.mul(drho).trace());
                let e_h = (tx * ty).re;
                let e_f = x.mul(&rf).mul(y).mul(&rf).trace().re;
                let e_p = kap.conj().mul(x).mul(kap).mul(&y.transpose()).trace().re;
                let energy = nu * (self.w_h * e_h - self.w_f * e_f - self.w_p * e_p);
                let n_h = x.scale(ty).add(&y.scale(tx));
                let n_f = self.proj(&x.mul(&rf).mul(y).add(&y.mul(&rf).mul(x)));
                let nmat = n_h.scale_r(nu * self.w_h).sub(&n_f.scale_r(nu * self.w_f));
                let pmat = x.mul(kap).mul(&y.transpose()).add(&y.mul(kap).mul(&x.transpose())).scale_r(-nu * self.w_p);
                (energy, nmat, pmat)
            }
            TermKindN::Pairing { g, k } => {
                let g = *g;
                let rf = self.proj(drho);
                let t = k.mul(&kap.transpose()).trace();
                let e_p = t.norm_sqr();
                let e_n = k.dagger().mul(&rf.transpose()).mul(k).mul(&rf).trace().re;
                let energy = -0.25 * g * (self.w_p * e_p + 2.0 * self.w_f * e_n);
                let n_n = self.proj(&k.dagger().mul(&rf.transpose()).mul(k)).scale_r(-g * self.w_f);
                let d = t.scale(-0.5 * g);
                let pmat = k.dagger().scale(d).scale_r(self.w_p);
                (energy, n_n, pmat)
            }
        }
    }
}

/// the result of a self-consistent solution
#[derive(Clone, Debug)]
pub struct SolutionN {
    pub n_self: CMat,
    pub p_self: CMat,
    pub drho: CMat,
    pub kap: CMat,
    /// the energy per volume measured from the normal vacuum (kinetic plus interaction, with the external terms)
    pub energy: f64,
    pub e_kin: f64,
    pub e_int: f64,
    pub iterations: usize,
    pub residual: f64,
}

/// a uniform model of several Dirac fields: blocks, chemical potential, contact terms, grid, optional external terms
pub struct ModelN {
    pub dim: usize,
    pub blocks: Vec<FieldBlock>,
    pub mu: f64,
    pub terms: Vec<TermN>,
    pub grid: Grid,
    /// an external pair source lambda (Pi + Pi^dag) with Pi = Psi^T K Psi (used for a fixed pair amplitude)
    pub ext_pair: Option<(CMat, f64)>,
    /// an external one-body term Psi^dag X Psi (Hermitian X): a uniform gauge potential or minus a chemical potential
    /// times a charge (used for the susceptibilities)
    pub ext_n: Option<CMat>,
    vac: Vec<CMat>,
    hq: Vec<CMat>,
    hmq: Vec<CMat>,
}

impl ModelN {
    pub fn new(blocks: Vec<FieldBlock>, mu: f64, terms: Vec<TermN>, grid: Grid) -> ModelN {
        let dim = blocks.iter().map(|b| b.off + b.ncomp).max().unwrap_or(0);
        let mut vac = vec![];
        let mut hq = vec![];
        let mut hmq = vec![];
        let one = CMat::identity(dim);
        for n in 0..grid.len() {
            let (q, _) = grid.node(n);
            let (h, v) = h_and_vac(&blocks, dim, &q);
            let (hm, _) = h_and_vac(&blocks, dim, &[-q[0], -q[1], -q[2]]);
            vac.push(v);
            hq.push(h.sub(&one.scale_r(mu)));
            hmq.push(hm.sub(&one.scale_r(mu)));
        }
        ModelN { dim, blocks, mu, terms, grid, ext_pair: None, ext_n: None, vac, hq, hmq }
    }
    /// the BdG matrix at the node n for the self-energies (N, P)
    pub fn bdg(&self, n: usize, nself: &CMat, pself: &CMat) -> CMat {
        let a = self.hq[n].add(nself);
        let d = self.hmq[n].add(nself).transpose().scale_r(-1.0);
        CMat::blocks(&a, pself, &pself.dagger(), &d)
    }
    /// the correlation matrices C^n(q), C^p(q) of the ground state of the BdG matrices at every node
    pub fn states(&self, nself: &CMat, pself: &CMat) -> (Vec<CMat>, Vec<CMat>) {
        let nd = self.dim;
        let m2 = 2 * nd;
        let mut cn = vec![];
        let mut cp = vec![];
        for n in 0..self.grid.len() {
            let (ev, v) = eigh(&self.bdg(n, nself, pself));
            let mut r = CMat::zeros(m2);
            for k in 0..m2 {
                if ev[k] < 0.0 {
                    for a in 0..m2 {
                        let va = v.get(a, k);
                        for b in 0..m2 {
                            r.a[a * m2 + b] += va * v.get(b, k).conj();
                        }
                    }
                }
            }
            cn.push(r.sub_block(0, 0, nd, nd));
            cp.push(r.sub_block(0, nd, nd, nd).scale_r(-1.0));
        }
        (cn, cp)
    }
    /// the local matrices drho (from the normal vacuum) and kap, and the kinetic energy per volume from the normal vacuum
    pub fn local(&self, cn: &[CMat], cp: &[CMat]) -> (CMat, CMat, f64) {
        let mut drho = CMat::zeros(self.dim);
        let mut kap = CMat::zeros(self.dim);
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
    /// the interaction energy and the self-energies of the local matrices (with the external terms, if any)
    pub fn interaction(&self, drho: &CMat, kap: &CMat) -> (f64, CMat, CMat) {
        let mut e = 0.0;
        let mut nself = CMat::zeros(self.dim);
        let mut pself = CMat::zeros(self.dim);
        for t in &self.terms {
            let (et, nt, pt) = t.eval(drho, kap);
            e += et;
            nself = nself.add(&nt);
            pself = pself.add(&pt);
        }
        if let Some((k, lam)) = &self.ext_pair {
            let t = k.mul(&kap.transpose()).trace();
            e += 2.0 * lam * t.re;
            pself = pself.add(&k.dagger().scale_r(2.0 * lam));
        }
        if let Some(x) = &self.ext_n {
            e += x.mul(drho).trace().re;
            nself = nself.add(x);
        }
        (e, nself, pself)
    }
    /// the self-energies of the contact terms alone, as linear functions of drho and kap (used by the static response)
    pub fn self_energy_lin(&self, drho: &CMat, kap: &CMat) -> (CMat, CMat) {
        let mut nself = CMat::zeros(self.dim);
        let mut pself = CMat::zeros(self.dim);
        for t in &self.terms {
            let (_, nt, pt) = t.eval(drho, kap);
            nself = nself.add(&nt);
            pself = pself.add(&pt);
        }
        (nself, pself)
    }
    /// the self-consistent solution from the initial self-energies, by linear mixing of the self-energies
    pub fn solve(&self, n0: &CMat, p0: &CMat, mix: f64, tol: f64, maxit: usize) -> SolutionN {
        let mut nself = n0.clone();
        let mut pself = p0.clone();
        let mut it = 0;
        loop {
            let (cn, cp) = self.states(&nself, &pself);
            let (drho, kap, ek) = self.local(&cn, &cp);
            let (ei, n2, p2) = self.interaction(&drho, &kap);
            let res = n2.sub(&nself).max_abs().max(p2.sub(&pself).max_abs());
            if res < tol || it >= maxit {
                return SolutionN { n_self: n2, p_self: p2, drho, kap, energy: ek + ei, e_kin: ek, e_int: ei, iterations: it, residual: res };
            }
            nself = n2.scale_r(mix).add(&nself.scale_r(1.0 - mix));
            pself = p2.scale_r(mix).add(&pself.scale_r(1.0 - mix));
            // keep the symmetries of the self-energies against rounding: N Hermitian, P antisymmetric
            nself = nself.add(&nself.dagger()).scale_r(0.5);
            pself = pself.sub(&pself.transpose()).scale_r(0.5);
            it += 1;
        }
    }
    /// the energy per volume of the ground state of given self-energies (not necessarily self-consistent)
    pub fn energy_of(&self, nself: &CMat, pself: &CMat) -> (f64, CMat, CMat) {
        let (cn, cp) = self.states(nself, pself);
        let (drho, kap, ek) = self.local(&cn, &cp);
        let (ei, _, _) = self.interaction(&drho, &kap);
        (ek + ei, drho, kap)
    }
    /// the quasi-particle energies (the non-negative eigenvalues of the BdG matrix, ascending) at the momentum q
    pub fn qp_energies(&self, q: &[f64; 3], nself: &CMat, pself: &CMat) -> Vec<f64> {
        let one = CMat::identity(self.dim);
        let (h, _) = h_and_vac(&self.blocks, self.dim, q);
        let (hm, _) = h_and_vac(&self.blocks, self.dim, &[-q[0], -q[1], -q[2]]);
        let a = h.sub(&one.scale_r(self.mu)).add(nself);
        let d = hm.sub(&one.scale_r(self.mu)).add(nself).transpose().scale_r(-1.0);
        let (ev, _) = eigh(&CMat::blocks(&a, pself, &pself.dagger(), &d));
        let mut out: Vec<f64> = ev.iter().filter(|e| **e >= 0.0).cloned().collect();
        out.sort_by(|a, b| a.partial_cmp(b).unwrap());
        out
    }
    /// the minimum over 0 <= |q| <= Lambda (along e_3; the uniform states are isotropic) of a function of the momentum:
    /// a scan of 201 momenta refined by a golden-section search; returns the minimum and the momentum of the minimum
    pub fn min_over_q<F: Fn(f64) -> f64>(&self, f: F) -> (f64, f64) {
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
    /// the gap of the state: the lowest quasi-particle energy over the momenta 0 <= |q| <= Lambda
    pub fn gap(&self, nself: &CMat, pself: &CMat) -> (f64, f64) {
        self.min_over_q(|x| self.qp_energies(&[0.0, 0.0, x], nself, pself)[0])
    }
}

// ================================================================================================ the static response

/// a real direction in the space of the self-energies: an element (i, j) of N (Hermitian: the pair of elements (i, j)
/// and (j, i)) or of P (antisymmetric), with a real or an imaginary unit coefficient
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Dir {
    /// N_ij = N_ji^* = 1 (i < j), or N_ii = 1 (i = j)
    NRe(usize, usize),
    /// N_ij = i, N_ji = -i (i < j)
    NIm(usize, usize),
    /// P_ij = -P_ji = 1 (i < j)
    PRe(usize, usize),
    /// P_ij = -P_ji = i (i < j)
    PIm(usize, usize),
}

impl Dir {
    /// the perturbation (dN, dP) of the direction, scaled by h
    pub fn mats(&self, dim: usize, h: f64) -> (CMat, CMat) {
        let mut dn = CMat::zeros(dim);
        let mut dp = CMat::zeros(dim);
        match *self {
            Dir::NRe(i, j) => {
                dn.set(i, j, c(h, 0.0));
                dn.set(j, i, c(h, 0.0));
            }
            Dir::NIm(i, j) => {
                dn.set(i, j, c(0.0, h));
                dn.set(j, i, c(0.0, -h));
            }
            Dir::PRe(i, j) => {
                dp.set(i, j, c(h, 0.0));
                dp.set(j, i, c(-h, 0.0));
            }
            Dir::PIm(i, j) => {
                dp.set(i, j, c(0.0, h));
                dp.set(j, i, c(0.0, -h));
            }
        }
        (dn, dp)
    }
    /// the coordinate of a self-energy pair (N, P) along the direction (the dual of `mats` with h = 1)
    pub fn coord(&self, n: &CMat, p: &CMat) -> f64 {
        match *self {
            Dir::NRe(i, j) => {
                if i == j {
                    n.get(i, i).re
                } else {
                    n.get(i, j).re
                }
            }
            Dir::NIm(i, j) => n.get(i, j).im,
            Dir::PRe(i, j) => p.get(i, j).re,
            Dir::PIm(i, j) => p.get(i, j).im,
        }
    }
}

/// the directions of the normal self-energy between the components of the index sets a and b (all elements (i, j) with
/// i in a, j in b, taken once), and of the pair self-energy likewise
pub fn n_dirs(a: &[usize], b: &[usize]) -> Vec<Dir> {
    let mut out = vec![];
    for &i in a {
        for &j in b {
            let (x, y) = if i <= j { (i, j) } else { (j, i) };
            if x == y {
                if !out.contains(&Dir::NRe(x, x)) {
                    out.push(Dir::NRe(x, x));
                }
            } else if !out.contains(&Dir::NRe(x, y)) {
                out.push(Dir::NRe(x, y));
                out.push(Dir::NIm(x, y));
            }
        }
    }
    out
}
pub fn p_dirs(a: &[usize], b: &[usize]) -> Vec<Dir> {
    let mut out = vec![];
    for &i in a {
        for &j in b {
            if i == j {
                continue;
            }
            let (x, y) = if i < j { (i, j) } else { (j, i) };
            if !out.contains(&Dir::PRe(x, y)) {
                out.push(Dir::PRe(x, y));
                out.push(Dir::PIm(x, y));
            }
        }
    }
    out
}

/// the static response of a stationary state with the self-energies (N, P), restricted to a list of real directions:
/// the real matrix M with M_kl = the coordinate k of the self-energy that the contact terms produce from the change of
/// the local matrices caused by a unit change of the self-energies along the direction l (central differences with the
/// step h, so the error is of order h^2).  At a stationary state an eigenvalue of M above 1 is an instability (for the
/// normal vacuum: the Thouless criterion); a symmetry that the state breaks gives the eigenvalue 1 (a Goldstone mode).
pub fn static_response(model: &ModelN, nself: &CMat, pself: &CMat, dirs: &[Dir], h: f64) -> CMat {
    let k = dirs.len();
    let mut m = CMat::zeros(k);
    for (l, d) in dirs.iter().enumerate() {
        let (dn, dp) = d.mats(model.dim, h);
        let (_, rp, kp) = model.energy_of(&nself.add(&dn), &pself.add(&dp));
        let (_, rm, km) = model.energy_of(&nself.sub(&dn), &pself.sub(&dp));
        let drho = rp.sub(&rm).scale_r(0.5 / h);
        let dkap = kp.sub(&km).scale_r(0.5 / h);
        let (n2, p2) = model.self_energy_lin(&drho, &dkap);
        for (kk, dk) in dirs.iter().enumerate() {
            m.set(kk, l, c(dk.coord(&n2, &p2), 0.0));
        }
    }
    m
}

/// the eigenvalues of the static response of a stationary state, restricted to a list of real directions, computed
/// from two symmetric matrices so that only Hermitian eigenproblems are solved.  In an orthonormal basis e_k of the
/// directions (in the real inner product Re tr(A^dag B)), with the "density" conjugate to N being drho and that
/// conjugate to P being -kap (the first variation of the energy is tr(N d drho) - Re tr(P d kap^dag)):
///   chi_kl = the coordinate k of the change of (drho, -kap) caused by the self-energy h e_l (central differences),
///            the second derivative of the energy of the ground state of the BdG matrices (negative semidefinite);
///   K_kl   = the coordinate k of the self-energy that the contact terms produce from the density e_l (exact, linear),
///            the second derivative of the interaction energy.
/// The response matrix K chi has the eigenvalues of the symmetric matrix -B K B with B = (-chi)^(1/2).  Returns the
/// eigenvalues (descending), the largest asymmetry of chi and of K (zero up to rounding), and the smallest eigenvalue
/// of -chi (non-negative up to rounding).
pub fn response_eigs(model: &ModelN, nself: &CMat, pself: &CMat, dirs: &[Dir], h: f64) -> (Vec<f64>, f64, f64, f64) {
    let k = dirs.len();
    let dim = model.dim;
    // the orthonormal basis matrices and whether each is a pair direction
    let basis: Vec<(bool, CMat)> = dirs
        .iter()
        .map(|d| {
            let (n, p) = d.mats(dim, 1.0);
            let is_p = p.max_abs() > 0.0;
            let m = if is_p { p } else { n };
            let nrm = m.dagger().mul(&m).trace().re.sqrt();
            (is_p, m.scale_r(1.0 / nrm))
        })
        .collect();
    let coord = |x: &CMat, e: &CMat| -> f64 { e.dagger().mul(x).trace().re };
    let mut chi = CMat::zeros(k);
    let z = CMat::zeros(dim);
    for (l, (is_p, e)) in basis.iter().enumerate() {
        let (dn, dp) = if *is_p { (z.clone(), e.scale_r(h)) } else { (e.scale_r(h), z.clone()) };
        let (_, rp, kp) = model.energy_of(&nself.add(&dn), &pself.add(&dp));
        let (_, rm, km) = model.energy_of(&nself.sub(&dn), &pself.sub(&dp));
        let drho = rp.sub(&rm).scale_r(0.5 / h);
        let mkap = km.sub(&kp).scale_r(0.5 / h);
        for (kk, (isp_k, ek)) in basis.iter().enumerate() {
            let v = if *isp_k { coord(&mkap, ek) } else { coord(&drho, ek) };
            chi.set(kk, l, c(v, 0.0));
        }
    }
    let mut kmat = CMat::zeros(k);
    for (l, (is_p, e)) in basis.iter().enumerate() {
        let (r, kap) = if *is_p { (z.clone(), e.scale_r(-1.0)) } else { (e.clone(), z.clone()) };
        let (n2, p2) = model.self_energy_lin(&r, &kap);
        for (kk, (isp_k, ek)) in basis.iter().enumerate() {
            let v = if *isp_k { coord(&p2, ek) } else { coord(&n2, ek) };
            kmat.set(kk, l, c(v, 0.0));
        }
    }
    let asym = |m: &CMat| -> f64 { m.sub(&m.transpose()).max_abs() };
    let (a_chi, a_k) = (asym(&chi), asym(&kmat));
    let chi_s = chi.add(&chi.transpose()).scale_r(0.5);
    let k_s = kmat.add(&kmat.transpose()).scale_r(0.5);
    // B = (-chi)^(1/2)
    let (av, au) = eigh(&chi_s.scale_r(-1.0));
    let amin = av.iter().cloned().fold(f64::INFINITY, f64::min);
    let mut bsq = CMat::zeros(k);
    for i in 0..k {
        for j in 0..k {
            let mut s = C64::ZERO;
            for m in 0..k {
                s += au.get(i, m) * au.get(j, m).conj() * av[m].max(0.0).sqrt();
            }
            bsq.set(i, j, s);
        }
    }
    let sm = bsq.mul(&k_s).mul(&bsq).scale_r(-1.0);
    let sm = sm.add(&sm.dagger()).scale_r(0.5);
    let (mut ev, _) = eigh(&sm);
    ev.reverse();
    (ev, a_chi, a_k, amin)
}

/// a small complex helper
pub fn cr(x: f64) -> C64 {
    c(x, 0.0)
}
