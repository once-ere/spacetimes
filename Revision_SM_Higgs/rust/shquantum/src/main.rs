//! sh_quantum -- the quantum numbers of the fields, of their spin-zero bilinears and of the superfluid under the revised
//! hypotheses H1-H6 of the folder Revision_SM_Higgs of PROJECT spacetimes (provenance document H0, second version: the
//! isodoublet Psi_LL and the two singlets chi_SH and chi'_SH), and the pattern of the breaking of SU(2)_L x U(1)_Y and of
//! the custodial symmetry SU(2)_L x SU(2)_R by the superfluid.
//!
//! What is computed.  The calculation is group theory: no dynamics enters.  Whether the superfluid forms, its gap and
//! the masses of its excitations are computed by the later programs of the folder.  Every statement of H0 that follows
//! from the hypotheses alone is checked here in floating point.  The Python program python/sh_quantum.py repeats the
//! checks with numpy (the identifiers with the prefix Y) and the Wolfram script wolfram/source/sh_quantum.wls repeats
//! them in exact arithmetic (the prefix W).
//!
//! The field.  The sixteen components of the field are numbered 0..15.  The components 0..7 are those of the isodoublet
//! Psi_LL (component 2 d + s, with the Dirac index d = 0..3 and the isospin index s = 0 (up), 1 (down), the convention of
//! the library sfcore), the components 8..11 those of the singlet chi_SH (component 8 + d) and 12..15 those of the
//! singlet chi'_SH (component 12 + d).  The Dirac matrices are those of the chiral basis of sfcore: d = 0, 1 is the block
//! of chirality -1 (left-handed), d = 2, 3 the block of chirality +1 (right-handed).  The first version of the
//! hypotheses (one singlet) is the part of the field without chi'_SH; every statement about Psi_LL and chi_SH holds in it.
//!
//! The bilinears.  A spin-zero bilinear of two fields phi_1, phi_2 (each Psi_LL, chi_SH or chi'_SH) is either a pair of two
//! particles, Pi[M] = phi_1^T M phi_2 (an operator that annihilates two particles; a superfluid is a condensate of such
//! pairs), or a particle-antiparticle bilinear, B[M] = phi_1^dag M phi_2 (the mean fields of the methods HS, KS and CJT
//! are expectation values of such bilinears).  Its coefficient matrix M has n_1 rows and n_2 columns.  Under a
//! transformation phi -> R phi of the fields, Pi[M] becomes Pi[R_1^T M R_2] and B[M] becomes B[R_1^dag M R_2].  For a
//! generator X of the internal group (R = exp(i eps X), X Hermitian) the change of M is i eps L_X(M), with
//!     L_X(M) = X_1^T M + M X_2          (pairs of particles),
//!     L_X(M) = -X_1 M + M X_2           (particle-antiparticle bilinears).
//! In an orthonormal basis M_1, ..., M_k of a channel (orthonormal in the inner product <A, B> = tr(A^dag B)) the
//! components Pi_s = Pi[M_s] transform as Pi -> (1 + i eps G_X) Pi, with the matrix (G_X)_{s s'} = <M_s', L_X(M_s)>.
//! The matrices G_X are the generators of the representation carried by the channel; their eigenvalues are its charges.
//! A field of the Standard Model with the generators G transforms in the same way, Phi -> exp(i eps G) Phi; the Higgs
//! doublet has G_{T^a} = tau^a / 2 and G_Y = 1/2.
//!
//! The checks (report data/hck_sh_quantum_checks.txt):
//!   QN-1   the internal generators on the sixteen components: T^a (SU(2)_L), T^a_R (the custodial SU(2)_R on the pair
//!          (chi, chi')), Y, Q, the particle number F_tot and the relative number R: their algebra, Hermiticity, and
//!          Y = T^3_R + R/4 (equation (4.3) of H0);
//!   QN-2   the six real generators of SL(2,C): their algebra, and that they commute with the internal generators;
//!   QN-3   the free Hamiltonians commute with the gauge generators for any masses, and with SU(2)_R only when the two
//!          singlets have equal masses;
//!   QN-4   the charges of the components: Y = 1/4, 1/4, -3/4; Q = T^3 + Y = 3/4, -1/4 (Psi_LL), 1/4 (chi), -3/4
//!          (chi'); Y = F_tot/4 on Psi_LL and chi_SH (the first version) but not on chi'_SH;
//!   QN-5   no gauge anomaly (traces with the chirality), vector-like fields, an even number of Weyl doublets (Witten);
//!   QN-6   the Lorentz-invariant pairs and particle-antiparticle bilinears of the twelve sectors, and the explicit basis
//!          C gamma^ch (x) e_s, C (x) e_s of the Psi chi and Psi chi' pairs;
//!   QN-7   the parity of the pair matrices C gamma^ch (x) e_s (+1) and C (x) e_s (-1);
//!   QN-8   the Psi chi pairs of positive parity carry exactly the generators of the Higgs doublet (G_{T^a} = tau^a/2,
//!          G_Y = 1/2, G_Q = diag(1, 0)); the Psi chi' pairs those of a doublet with Y = -1/2 (G_Q = diag(0, -1)); both
//!          have F_tot = 2 and R = 0; together they form a bidoublet of SU(2)_L x SU(2)_R;
//!   QN-9   finite transformations (pseudo-random, seeded): the two pair fields transform as doublets with Y = 1/2 and
//!          Y = -1/2, the bidoublet as M -> L M R^T, every invariant bilinear is Lorentz invariant, and the pairing term
//!          (4.2) of H0 is invariant under SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot;
//!   QN-10  the 24 channels of spin-zero bilinears and their charges: the neutral pairs are those of a down particle
//!          with chi and of an up particle with chi'; no pair of two particles of one field and no chi chi' pair is
//!          neutral;
//!   QN-11  the breaking: every doublet is rotated to (0, phi); the vacuum with the neutral components of both pair
//!          fields keeps only Q in SU(2)_L x U(1)_Y; with equal lengths it keeps the custodial SU(2)_V and R among the
//!          eight generators of SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot, so that four are broken: three of SU(2)_L x U(1)_Y
//!          and the particle number (an extra Goldstone boson); a second condensate with a charged component leaves no
//!          generator of SU(2)_L x U(1)_Y unbroken, a neutral one keeps Q;
//!   QN-12  the mass matrix of W^a and B from a canonical kinetic term of the two pair fields: the masses (2.2) of H0,
//!          the massless photon A = sin(theta_W) W^3 + cos(theta_W) B, the charges +1 and -1 of W^+ and W^-, rho = 1, and
//!          the couplings in GeV units from the measured inputs.
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_quantum
//! Input: data/sh_inputs.csv (the measured values of the Review of Particle Physics, 2026 edition).  Output in data/:
//! hck_sh_quantum_checks.txt, sh_quantum_numbers.csv, sh_channels.csv, sh_couplings.csv, sh_mass_matrix.csv,
//! sh_quantum_values.csv.  Run time well below a second.
use sfcore::algebra::{lorentz_generator, random_unit_biq};
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{adj_gen, alpha, beta, cmat_c, d8, gamma, gamma_ch, h_free, levi, s_gen, s_spin, sigma, t_gen};
use sfcore::report::{data_dir, f, write_csv, Report};
use sfcore::rng::Rng;
use std::collections::HashMap;
use std::f64::consts::PI;
use std::path::Path;

/// the numbers of components of Psi_LL, of each singlet (chi_SH, chi'_SH) and of the whole field
const NP: usize = 8;
const NX: usize = 4;
const N: usize = NP + 2 * NX;
/// the tolerance of the checks of exact identities evaluated in floating point
const TOL: f64 = 1e-12;
/// an eigenvalue of a Gram matrix below this value is counted as zero; the others are of order 1 (the report gives the
/// largest eigenvalue counted as zero and the smallest one counted as not zero, so that the gap can be seen)
const ZERO_EIG: f64 = 1e-10;
/// the masses of the check QN-3 in units of the cutoff Lambda_LL: m_LL of the earlier hypotheses and different masses of
/// the singlets, to show that the gauge symmetries do not need equal masses (decision D22) while SU(2)_R does (D31)
const M_LL: f64 = 0.2;
const M_CHI_TEST: f64 = 0.35;
const M_CHI2_TEST: f64 = 0.5;
/// the number of pseudo-random trials of the checks QN-3, QN-9 and QN-11, and the seed of the generator
const TRIALS: usize = 25;
const SEED: u64 = 0x5348_514e_2026;

// ------------------------------------------------------------------------------------------------ fields and generators

/// the three spinor fields of H1 (second version), and the pair of the two singlets (chi_SH, chi'_SH) taken as one
/// field of eight components (component 4 a + d of the pair: a = 0 for chi_SH, a = 1 for chi'_SH, Dirac index d), on
/// which the custodial SU(2)_R acts
#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    Psi,
    Chi,
    Chi2,
    Sing,
}
impl Field {
    /// the first of the sixteen components that belongs to the field
    fn off(self) -> usize {
        match self {
            Field::Psi => 0,
            Field::Chi | Field::Sing => NP,
            Field::Chi2 => NP + NX,
        }
    }
    /// the number of components of the field
    fn n(self) -> usize {
        match self {
            Field::Psi => NP,
            Field::Chi | Field::Chi2 => NX,
            Field::Sing => 2 * NX,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Field::Psi => "Psi",
            Field::Chi => "chi",
            Field::Chi2 => "chi'",
            Field::Sing => "(chi, chi')",
        }
    }
}

/// a Dirac matrix (4 x 4) acting on the components of one field: A (x) 1_2 on Psi_LL, A on a singlet, diag(A, A) on the
/// pair of singlets
fn dirac_on(fl: Field, a4: &CMat) -> CMat {
    match fl {
        Field::Psi => d8(a4),
        Field::Chi | Field::Chi2 => a4.clone(),
        Field::Sing => CMat::block_diag(a4, a4),
    }
}
/// a Dirac matrix acting on all three fields (16 x 16)
fn dirac16(a4: &CMat) -> CMat {
    CMat::block_diag(&CMat::block_diag(&d8(a4), a4), a4)
}
/// the block of a 16 x 16 operator between two fields (the block of one field with itself when fl1 = fl2)
fn on_fields(x: &CMat, fl1: Field, fl2: Field) -> CMat {
    x.sub_block(fl1.off(), fl2.off(), fl1.n(), fl2.n())
}
/// the block of a 16 x 16 operator that acts on one field
fn on_field(x: &CMat, fl: Field) -> CMat {
    on_fields(x, fl, fl)
}

/// the internal generators on the sixteen components (H1, H2 and H4 of H0)
struct Gens {
    /// T^1, T^2, T^3 = diag(1_4 (x) tau^a / 2, 0_8): SU(2)_L acts on the isospin index of Psi_LL only
    t: Vec<CMat>,
    /// T^1_R, T^2_R, T^3_R = diag(0_8, (tau^a / 2) (x) 1_4): the custodial SU(2)_R acts on the pair (chi_SH, chi'_SH) as on
    /// a doublet (chi_SH the upper member), on the index of the field and not on the Dirac index (H4)
    tr: Vec<CMat>,
    /// the hypercharge of the table of H1: 1/4 on Psi_LL and chi_SH, -3/4 on chi'_SH (built from the table, not from
    /// equation (4.3), which the check QN-1e tests)
    y: CMat,
    /// the particle number F_tot: 1 on every component
    f: CMat,
    /// the relative number R: +1 on Psi_LL, -1 on both singlets
    r: CMat,
    /// the electric charge Q = T^3 + Y
    q: CMat,
}
fn internal() -> Gens {
    let t: Vec<CMat> = (1..=3).map(|a| CMat::block_diag(&t_gen(a), &CMat::zeros(2 * NX))).collect();
    let tr: Vec<CMat> = (1..=3)
        .map(|a| CMat::block_diag(&CMat::zeros(NP), &sigma(a).scale_r(0.5).kron(&CMat::identity(NX))))
        .collect();
    let y = CMat::block_diag(&CMat::identity(NP + NX).scale_r(0.25), &CMat::identity(NX).scale_r(-0.75));
    let fnum = CMat::identity(N);
    let r = CMat::block_diag(&CMat::identity(NP), &CMat::identity(2 * NX).scale_r(-1.0));
    let q = t[2].add(&y);
    Gens { t, tr, y, f: fnum, r, q }
}
/// the positions in the list of Gens::list_all of Y, Q, F_tot, R and of T^1_R, T^2_R, T^3_R (T^a has the position a - 1)
const IX_Y: usize = 3;
const IX_Q: usize = 4;
const IX_F: usize = 5;
const IX_R: usize = 6;
const IX_TR: [usize; 3] = [8, 9, 7];
impl Gens {
    /// the generators that do not mix the three fields, with their names, in the order of the data files
    fn list(&self) -> Vec<(&'static str, CMat)> {
        vec![
            ("T1", self.t[0].clone()),
            ("T2", self.t[1].clone()),
            ("T3", self.t[2].clone()),
            ("Y", self.y.clone()),
            ("Q", self.q.clone()),
            ("F", self.f.clone()),
            ("R", self.r.clone()),
            ("T3R", self.tr[2].clone()),
        ]
    }
    /// all ten generators: those of list() and T^1_R, T^2_R, which mix chi_SH and chi'_SH
    fn list_all(&self) -> Vec<(&'static str, CMat)> {
        let mut v = self.list();
        v.push(("T1R", self.tr[0].clone()));
        v.push(("T2R", self.tr[1].clone()));
        v
    }
}

/// the six real generators of SL(2,C) as 2 x 2 matrices M(u): the rotations J^j (M = -i sigma_j / 2) and the boosts
/// K^j (M = sigma_j / 2), j = 1, 2, 3 (the convention of sfcore::algebra::lorentz_generator)
fn lorentz_basis() -> Vec<CMat> {
    let mut v = vec![];
    for j in 0..3 {
        let mut e = [0.0; 3];
        e[j] = 1.0;
        v.push(lorentz_generator(&e, &[0.0; 3]));
    }
    for j in 0..3 {
        let mut e = [0.0; 3];
        e[j] = 1.0;
        v.push(lorentz_generator(&[0.0; 3], &e));
    }
    v
}
/// a Lorentz generator on the Dirac index of one field: s(u) = diag(-M(u)^dag, M(u)) (sfcore::dirac::s_gen)
fn lorentz_on(fl: Field, mu: &CMat) -> CMat {
    dirac_on(fl, &s_gen(mu))
}
/// the free Dirac Hamiltonian of a singlet on a plane wave of momentum q, alpha.q + beta m (4 x 4)
fn h_free4(q: &[f64; 3], m: f64) -> CMat {
    let mut h = beta().scale_r(m);
    for j in 0..3 {
        h = h.add(&alpha(j + 1).scale_r(q[j]));
    }
    h
}

// ------------------------------------------------------------------------------------------------ bilinears

/// the two kinds of bilinears: pairs of two particles and particle-antiparticle bilinears
#[derive(Clone, Copy, PartialEq, Debug)]
enum Kind {
    Pair,
    PartAnti,
}

/// a sector of bilinears: the kind and the two fields
#[derive(Clone, Copy, Debug)]
struct Sector {
    kind: Kind,
    f1: Field,
    f2: Field,
}
impl Sector {
    /// the name used in the data files, for example "Psi chi" (pairs) or "Psi^dag chi"
    fn name(&self) -> String {
        match self.kind {
            Kind::Pair => format!("{} {}", self.f1.label(), self.f2.label()),
            Kind::PartAnti => format!("{}^dag {}", self.f1.label(), self.f2.label()),
        }
    }
    /// a pair of two particles of the same field: only the antisymmetric part of M contributes (Fermi statistics)
    fn same_pair(&self) -> bool {
        self.kind == Kind::Pair && self.f1 == self.f2
    }
}

/// the change of the coefficient matrix M of a bilinear under phi -> (1 + eps A) phi, to first order in eps:
/// A_1^T M + M A_2 (pairs) or A_1^dag M + M A_2 (particle-antiparticle)
fn delta(kind: Kind, a1: &CMat, a2: &CMat, m: &CMat) -> CMat {
    let left = match kind {
        Kind::Pair => a1.transpose(),
        Kind::PartAnti => a1.dagger(),
    };
    left.mul(m).add(&m.mul(a2))
}
/// L_X(M) for an internal generator X (R = exp(i eps X)): the change delta for A = i X, divided by i
fn lmap(sec: &Sector, x: &CMat, m: &CMat) -> CMat {
    let ix1 = on_field(x, sec.f1).scale(C64::I);
    let ix2 = on_field(x, sec.f2).scale(C64::I);
    delta(sec.kind, &ix1, &ix2, m).scale(-C64::I)
}
/// the coefficient matrix after the finite transformation phi -> R phi (R_1, R_2 its blocks on the two fields)
fn transform(kind: Kind, r1: &CMat, r2: &CMat, m: &CMat) -> CMat {
    let left = match kind {
        Kind::Pair => r1.transpose(),
        Kind::PartAnti => r1.dagger(),
    };
    left.mul(m).mul(r2)
}
/// the parity of a bilinear: phi(t, x) -> gamma^0 phi(t, -x) on both fields, with the intrinsic parities +1
fn parity_map(sec: &Sector, m: &CMat) -> CMat {
    transform(sec.kind, &dirac_on(sec.f1, &gamma(0)), &dirac_on(sec.f2, &gamma(0)), m)
}
/// the inner product <A, B> = tr(A^dag B) of two coefficient matrices
fn inner(a: &CMat, b: &CMat) -> C64 {
    a.a.iter().zip(&b.a).fold(C64::ZERO, |s, (x, y)| s + x.conj() * *y)
}

/// a linear map of coefficient matrices
type LinMap<'a> = Box<dyn Fn(&CMat) -> CMat + 'a>;

/// the coefficient matrices M (n1 x n2) annihilated by every map of `maps`: an orthonormal basis, made of the
/// eigenvectors of H = sum_k A_k^dag A_k with eigenvalues below ZERO_EIG (A_k is the matrix of the k-th map on the
/// n1 n2 entries of M); returned with the largest eigenvalue counted as zero and the smallest one counted as not zero
fn null_space(n1: usize, n2: usize, maps: &[LinMap]) -> (Vec<CMat>, f64, f64) {
    let n = n1 * n2;
    let mut h = CMat::zeros(n);
    for mp in maps {
        // the matrix of the map: column k is the image of the k-th unit matrix (entry k of M equal to 1)
        let mut a = CMat::new(n, n);
        for k in 0..n {
            let mut e = CMat::new(n1, n2);
            e.a[k] = C64::ONE;
            let img = mp(&e);
            for i in 0..n {
                a.set(i, k, img.a[i]);
            }
        }
        h = h.add(&a.dagger().mul(&a));
    }
    let (ev, vecs) = eigh(&h);
    let mut basis = vec![];
    let (mut zmax, mut nzmin) = (0.0f64, f64::INFINITY);
    for (j, &l) in ev.iter().enumerate() {
        if l.abs() < ZERO_EIG {
            zmax = zmax.max(l.abs());
            let mut m = CMat::new(n1, n2);
            for i in 0..n {
                m.a[i] = vecs.get(i, j);
            }
            basis.push(m);
        } else {
            nzmin = nzmin.min(l);
        }
    }
    (basis, zmax, nzmin)
}

/// the conditions of Lorentz invariance of the bilinears of a sector (one map for each of the six generators), with
/// the condition of antisymmetry M + M^T = 0 for a pair of particles of the same field when `antisym` is true, or the
/// condition of symmetry M - M^T = 0 when `sym` is true
fn invariance_maps<'a>(sec: &Sector, lb: &'a [CMat], antisym: bool, sym: bool) -> Vec<LinMap<'a>> {
    let mut maps: Vec<LinMap<'a>> = vec![];
    for mu in lb {
        let a1 = lorentz_on(sec.f1, mu);
        let a2 = lorentz_on(sec.f2, mu);
        let kind = sec.kind;
        maps.push(Box::new(move |m: &CMat| delta(kind, &a1, &a2, m)));
    }
    if antisym {
        maps.push(Box::new(|m: &CMat| m.add(&m.transpose())));
    }
    if sym {
        maps.push(Box::new(|m: &CMat| m.sub(&m.transpose())));
    }
    maps
}

/// the matrix G_X of a generator on the span of an orthonormal basis, (G_X)_{s s'} = <M_s', L_X(M_s)>, and the norm of
/// the largest part of an L_X(M_s) outside the span (zero when the span is invariant)
fn rep_matrix(sec: &Sector, x: &CMat, basis: &[CMat]) -> (CMat, f64) {
    let k = basis.len();
    let mut g = CMat::zeros(k);
    let mut res = 0.0f64;
    for s in 0..k {
        let lm = lmap(sec, x, &basis[s]);
        let mut rest = lm.clone();
        for sp in 0..k {
            let cf = inner(&basis[sp], &lm);
            g.set(s, sp, cf);
            rest = rest.sub(&basis[sp].scale(cf));
        }
        res = res.max(rest.fro());
    }
    (g, res)
}

/// the parts of parity +1 and -1 of an invariant space of bilinears (orthonormal bases), and the largest deviation
/// (a part of P(M) outside the space, or an eigenvalue of the parity other than +1 or -1)
fn split_parity(sec: &Sector, basis: &[CMat]) -> (Vec<CMat>, Vec<CMat>, f64) {
    let k = basis.len();
    let mut p = CMat::zeros(k);
    let mut dev = 0.0f64;
    for l in 0..k {
        let pm = parity_map(sec, &basis[l]);
        let mut rest = pm.clone();
        for kk in 0..k {
            let cf = inner(&basis[kk], &pm);
            p.set(kk, l, cf);
            rest = rest.sub(&basis[kk].scale(cf));
        }
        dev = dev.max(rest.fro());
    }
    let (ev, vecs) = eigh(&p);
    let (mut plus, mut minus) = (vec![], vec![]);
    for j in 0..k {
        let mut m = CMat::new(basis[0].rows, basis[0].cols);
        for kk in 0..k {
            m = m.add(&basis[kk].scale(vecs.get(kk, j)));
        }
        if ev[j] > 0.0 {
            dev = dev.max((ev[j] - 1.0).abs());
            plus.push(m);
        } else {
            dev = dev.max((ev[j] + 1.0).abs());
            minus.push(m);
        }
    }
    (plus, minus, dev)
}

/// a channel: the bilinears of one sector and one parity, an orthonormal basis and the generators on it
struct Channel {
    sec: Sector,
    parity: i32,
    basis: Vec<CMat>,
    /// G_X for the generators X of the list given to Channel::new (Gens::list or Gens::list_all), in its order
    gens: Vec<CMat>,
    /// the largest part of an L_X(M_s) outside the channel (zero: the channel is a representation)
    closure: f64,
}
impl Channel {
    fn new(sec: Sector, parity: i32, basis: Vec<CMat>, gl: &[(&'static str, CMat)]) -> Channel {
        let mut gens = vec![];
        let mut closure = 0.0f64;
        for (_, x) in gl {
            let (g, res) = rep_matrix(&sec, x, &basis);
            gens.push(g);
            closure = closure.max(res);
        }
        Channel { sec, parity, basis, gens, closure }
    }
    fn dim(&self) -> usize {
        self.basis.len()
    }
    /// the eigenvalues of G_X (index into the generator list), descending
    fn charges(&self, ix: usize) -> Vec<f64> {
        let mut v = eigh(&self.gens[ix]).0;
        v.reverse();
        v
    }
    /// the Casimir operator sum_a G_{T^a}^2 of SU(2)_L
    fn cas_l(&self) -> CMat {
        let mut cas = CMat::zeros(self.dim());
        for a in 0..3 {
            cas = cas.add(&self.gens[a].mul(&self.gens[a]));
        }
        cas
    }
    /// the Casimir operator sum_a G_{T^a_R}^2 of SU(2)_R (for a channel built with Gens::list_all)
    fn cas_r(&self) -> CMat {
        let mut cas = CMat::zeros(self.dim());
        for a in 0..3 {
            cas = cas.add(&self.gens[IX_TR[a]].mul(&self.gens[IX_TR[a]]));
        }
        cas
    }
    /// the eigenvalues of the Casimir operator of SU(2)_L, ascending
    fn casimir(&self) -> Vec<f64> {
        eigh(&self.cas_l()).0
    }
    fn label(&self) -> String {
        format!("{} ({})", self.sec.name(), if self.parity > 0 { "+" } else { "-" })
    }
}

/// the SU(2) multiplets of a representation from the eigenvalues of its Casimir operator, j(j+1): a multiplet of
/// dimension 2j+1 accounts for 2j+1 equal eigenvalues; for example "1 + 3"; the flag is false when a count does not fit
fn su2_content(cas: &[f64]) -> (String, bool) {
    let mut vals: Vec<f64> = cas.to_vec();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let (mut dims, mut ok, mut i) = (vec![], true, 0);
    while i < vals.len() {
        let cv = vals[i];
        let j = ((1.0 + 4.0 * cv).sqrt() - 1.0) / 2.0;
        let d = (2.0 * j + 1.0).round() as usize;
        let cnt = vals.iter().filter(|&&x| (x - cv).abs() < 1e-9).count();
        if d == 0 || cnt % d != 0 || ((2.0 * j) - (2.0 * j).round()).abs() > 1e-9 {
            ok = false;
        } else {
            for _ in 0..(cnt / d) {
                dims.push(d);
            }
        }
        i += cnt;
    }
    (dims.iter().map(|d| d.to_string()).collect::<Vec<_>>().join(" + "), ok)
}

/// the multiplets of SU(2)_L x SU(2)_R of a representation from its two Casimir operators, which commute: an eigenvalue
/// of C_L + sqrt2 C_R is j_L (j_L + 1) + sqrt2 j_R (j_R + 1) for one pair (j_L, j_R) only, because sqrt2 is irrational,
/// and a multiplet (2 j_L + 1, 2 j_R + 1) accounts for (2 j_L + 1)(2 j_R + 1) equal eigenvalues; for example "(2,2)" or
/// "(1,1) + (1,3)"; the flag is false when the operators do not commute or a value or a count does not fit
fn su2x2_content(cl: &CMat, cr: &CMat) -> (String, bool) {
    let mut ok = cl.comm(cr).max_abs() < 1e-9;
    let mut ev = eigh(&cl.add(&cr.scale_r(2.0f64.sqrt()))).0;
    ev.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let js = [0.0, 0.5, 1.0, 1.5, 2.0];
    let mut mult: Vec<(usize, usize)> = vec![];
    let mut i = 0;
    while i < ev.len() {
        let cv = ev[i];
        let cnt = ev.iter().filter(|&&x| (x - cv).abs() < 1e-8).count();
        let mut hit = None;
        for &jl in &js {
            for &jr in &js {
                if (jl * (jl + 1.0) + 2.0f64.sqrt() * jr * (jr + 1.0) - cv).abs() < 1e-8 {
                    hit = Some(((2.0 * jl + 1.0).round() as usize, (2.0 * jr + 1.0).round() as usize));
                }
            }
        }
        match hit {
            Some((dl, dr)) if cnt % (dl * dr) == 0 => {
                for _ in 0..(cnt / (dl * dr)) {
                    mult.push((dl, dr));
                }
            }
            _ => ok = false,
        }
        i += cnt;
    }
    mult.sort();
    (mult.iter().map(|(a, b)| format!("({},{})", a, b)).collect::<Vec<_>>().join(" + "), ok)
}

/// a number as a reduced fraction p/q with q <= 8 when it is one to 1e-9, otherwise in decimal notation
fn frac(x: f64) -> String {
    for q in 1..=8i64 {
        let p = (x * q as f64).round();
        if (x - p / q as f64).abs() < 1e-9 {
            let p = p as i64;
            return if q == 1 { format!("{}", p) } else { format!("{}/{}", p, q) };
        }
    }
    format!("{:.6}", x)
}
/// a list of numbers as fractions, separated by semicolons
fn fracs(v: &[f64]) -> String {
    v.iter().map(|x| frac(*x)).collect::<Vec<_>>().join("; ")
}
/// the largest difference between two lists of numbers of equal length (infinity for unequal lengths)
fn list_dev(a: &[f64], b: &[f64]) -> f64 {
    if a.len() != b.len() {
        return f64::INFINITY;
    }
    a.iter().zip(b).fold(0.0f64, |m, (x, y)| m.max((x - y).abs()))
}

/// the isospin unit vector e_s (2 x 1), s = 0 (up) or 1 (down)
fn e_iso(s: usize) -> CMat {
    let mut e = CMat::new(2, 1);
    e.a[s] = C64::ONE;
    e
}

/// the real dimension of the stabilizer: the real coefficients c (one for each generator of the list `gens_of`) with
/// (sum_A c_A G^(k)_A) w_k = 0 for every pair (G^(k), w_k), where G^(k) are the generators of a channel and w_k a vector
/// of its components; returned with a basis of the stabilizer (unit vectors of coefficients) and the gap of the
/// eigenvalues (the largest counted as zero, the smallest counted as not zero)
fn stabilizer(conds: &[(&[CMat], &[C64])]) -> (Vec<Vec<f64>>, f64, f64) {
    let na = conds[0].0.len();
    // the real matrix of the map c -> (G^(k)(c) w_k)_k, rows: real and imaginary parts of the components
    let mut rows: Vec<Vec<f64>> = vec![];
    for (g, w) in conds {
        let k = w.len();
        for i in 0..k {
            let mut re = vec![0.0; na];
            let mut im = vec![0.0; na];
            for (a, ga) in g.iter().enumerate() {
                let mut s = C64::ZERO;
                for j in 0..k {
                    s += ga.get(i, j) * w[j];
                }
                re[a] = s.re;
                im[a] = s.im;
            }
            rows.push(re);
            rows.push(im);
        }
    }
    let mut ata = CMat::zeros(na);
    for a in 0..na {
        for b in 0..na {
            let s: f64 = rows.iter().map(|r| r[a] * r[b]).sum();
            ata.set(a, b, c(s, 0.0));
        }
    }
    let (ev, vecs) = eigh(&ata);
    let (mut basis, mut zmax, mut nzmin) = (vec![], 0.0f64, f64::INFINITY);
    for (j, &l) in ev.iter().enumerate() {
        if l.abs() < ZERO_EIG {
            zmax = zmax.max(l.abs());
            // the eigenvectors of a real symmetric matrix may carry a common phase: make the largest entry real
            let col: Vec<C64> = (0..na).map(|a| vecs.get(a, j)).collect();
            let big = col.iter().fold(C64::ZERO, |m, z| if z.abs() > m.abs() { *z } else { m });
            let ph = big.conj().scale(1.0 / big.abs());
            basis.push(col.iter().map(|z| (*z * ph).re).collect());
        } else {
            nzmin = nzmin.min(l);
        }
    }
    (basis, zmax, nzmin)
}

/// the largest entry of the difference of the orthogonal projectors onto the spans of two sets of orthonormal real
/// vectors (zero when the two sets span the same space; infinity when their numbers differ)
fn span_dev(a: &[Vec<f64>], b: &[Vec<f64>]) -> f64 {
    if a.len() != b.len() || a.is_empty() {
        return if a.len() == b.len() { 0.0 } else { f64::INFINITY };
    }
    let n = a[0].len();
    let proj = |v: &[Vec<f64>], i: usize, j: usize| v.iter().map(|x| x[i] * x[j]).sum::<f64>();
    let mut d = 0.0f64;
    for i in 0..n {
        for j in 0..n {
            d = d.max((proj(a, i, j) - proj(b, i, j)).abs());
        }
    }
    d
}
/// the number of linearly independent real vectors of a list (the rank of their Gram matrix, eigenvalues above 1e-10)
fn real_rank(v: &[Vec<f64>]) -> usize {
    let k = v.len();
    let mut gm = CMat::zeros(k);
    for i in 0..k {
        for j in 0..k {
            gm.set(i, j, c(v[i].iter().zip(&v[j]).map(|(x, y)| x * y).sum(), 0.0));
        }
    }
    eigh(&gm).0.iter().filter(|&&x| x > 1e-10).count()
}
/// the matrix C of a finite transformation on the span of an orthonormal basis M_1, ..., M_k of coefficient matrices:
/// the bilinear with M_s becomes the one with transform(M_s) = sum_s' C_{s s'} M_s', so that the components (and the
/// condensates) transform as Pi -> C Pi; returned with the largest part of a transformed matrix outside the span
fn trans_matrix(kind: Kind, r1: &CMat, r2: &CMat, basis: &[CMat]) -> (CMat, f64) {
    let k = basis.len();
    let mut cm = CMat::zeros(k);
    let mut res = 0.0f64;
    for s in 0..k {
        let tm = transform(kind, r1, r2, &basis[s]);
        let mut rest = tm.clone();
        for sp in 0..k {
            let cf = inner(&basis[sp], &tm);
            cm.set(s, sp, cf);
            rest = rest.sub(&basis[sp].scale(cf));
        }
        res = res.max(rest.max_abs());
    }
    (cm, res)
}
/// a key for the file of values from the name of a sector: blanks to underscores, without ^ and with p for '
fn key(name: &str) -> String {
    name.replace(' ', "_").replace('^', "").replace('\'', "p").replace(['(', ')', ','], "")
}

/// a pseudo-random complex vector of unit length with k components
fn random_vec(rng: &mut Rng, k: usize) -> Vec<C64> {
    let v: Vec<C64> = (0..k).map(|_| c(rng.unit(), rng.unit())).collect();
    let nrm = v.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    v.iter().map(|z| z.scale(1.0 / nrm)).collect()
}

// ------------------------------------------------------------------------------------------------ the measured inputs

/// splits one line of a CSV file into its fields (fields in double quotes may contain commas; RFC 4180)
fn csv_split(line: &str) -> Vec<String> {
    let (mut out, mut cur, mut inq) = (vec![], String::new(), false);
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if inq {
            if ch == '"' {
                if chars.peek() == Some(&'"') {
                    cur.push('"');
                    chars.next();
                } else {
                    inq = false;
                }
            } else {
                cur.push(ch);
            }
        } else if ch == '"' {
            inq = true;
        } else if ch == ',' {
            out.push(std::mem::take(&mut cur));
        } else {
            cur.push(ch);
        }
    }
    out.push(cur);
    out
}
/// the measured inputs: name -> (value, uncertainty)
fn read_inputs(p: &Path) -> HashMap<String, (f64, f64)> {
    let s = std::fs::read_to_string(p).unwrap_or_else(|_| panic!("missing {}", p.display()));
    let mut m = HashMap::new();
    for line in s.lines().skip(1) {
        let v = csv_split(line);
        if v.len() >= 3 {
            let val: f64 = v[1].parse().unwrap_or_else(|_| panic!("bad value in {}: {}", p.display(), line));
            let unc: f64 = v[2].parse().unwrap_or_else(|_| panic!("bad uncertainty in {}: {}", p.display(), line));
            m.insert(v[0].clone(), (val, unc));
        }
    }
    m
}

// ------------------------------------------------------------------------------------------------ main

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sh_quantum: checks of the quantum numbers and of the breaking under the revised hypotheses, second version \
         (Revision_SM_Higgs, PROJECT spacetimes)",
    );
    let mut vals: Vec<(String, f64)> = vec![];
    let mut rng = Rng(SEED);
    let g = internal();
    let gl = g.list();
    let gla = g.list_all();
    let lb = lorentz_basis();
    let lorentz16: Vec<CMat> = lb.iter().map(|mu| dirac16(&s_gen(mu))).collect();
    let i2 = CMat::identity(2);

    // ------------------------------------------------------------------ QN-1: the internal generators
    // the largest deviation from the algebra [X^a, X^b] = i eps_abc X^c of three generators
    let su2_dev = |x: &[CMat]| -> f64 {
        let mut d = 0.0f64;
        for a in 0..3 {
            for b in 0..3 {
                let mut rhs = CMat::zeros(N);
                for cc in 0..3 {
                    rhs = rhs.add(&x[cc].scale(c(0.0, levi(a, b, cc))));
                }
                d = d.max(x[a].comm(&x[b]).sub(&rhs).max_abs());
            }
        }
        d
    };
    rep.add("QN-1a", "[T^a, T^b] = i eps_abc T^c on the sixteen components (the algebra of SU(2)_L)", su2_dev(&g.t), TOL);
    let mut d_lr = su2_dev(&g.tr);
    for a in 0..3 {
        for b in 0..3 {
            d_lr = d_lr.max(g.t[a].comm(&g.tr[b]).max_abs());
        }
    }
    rep.add(
        "QN-1b",
        "[T^a_R, T^b_R] = i eps_abc T^c_R (the algebra of the custodial SU(2)_R) and [T^a, T^b_R] = 0",
        d_lr,
        TOL,
    );
    let mut d_comm = 0.0f64;
    let abelian = [&g.y, &g.f, &g.r, &g.tr[2]];
    for x in &abelian {
        for a in 0..3 {
            d_comm = d_comm.max(g.t[a].comm(x).max_abs());
        }
        for z in &abelian {
            d_comm = d_comm.max(x.comm(z).max_abs());
        }
    }
    for x in [&g.f, &g.r] {
        for a in 0..3 {
            d_comm = d_comm.max(g.tr[a].comm(x).max_abs());
        }
    }
    rep.add(
        "QN-1c",
        "Y, F_tot, R and T^3_R commute with T^1, T^2, T^3 and with each other, and F_tot and R commute with T^a_R (the \
         global group SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot of H4)",
        d_comm,
        TOL,
    );
    let d_herm = gla.iter().fold(0.0f64, |m, (_, x)| m.max(x.sub(&x.dagger()).max_abs()));
    rep.add("QN-1d", "T^a, T^a_R, Y, Q, F_tot, R are Hermitian (a unitary representation)", d_herm, TOL);
    let d_43 = g.y.sub(&g.tr[2].add(&g.r.scale_r(0.25))).max_abs();
    let d_q3 = g.q.sub(&g.t[2].add(&g.tr[2]).add(&g.r.scale_r(0.25))).max_abs();
    let y_tr1 = g.y.comm(&g.tr[0]).max_abs();
    rep.add_bool(
        "QN-1e",
        "Y = T^3_R + R/4 (equation (4.3)) and Q = T^3 + T^3_R + R/4; Y does not commute with T^1_R (the coupling of B to \
         Y breaks SU(2)_R to the U(1) of T^3_R)",
        d_43 <= TOL && d_q3 <= TOL && y_tr1 > 0.1,
        &format!("deviations {:.1e} and {:.1e}; largest entry of [Y, T^1_R] {}", d_43, d_q3, frac(y_tr1)),
    );

    // ------------------------------------------------------------------ QN-2: the Lorentz generators
    let mut d_lor = 0.0f64;
    for k in 0..6 {
        for l in 0..6 {
            let br = lb[k].comm(&lb[l]);
            let lhs = dirac16(&s_gen(&br));
            d_lor = d_lor.max(lhs.sub(&lorentz16[k].comm(&lorentz16[l])).max_abs());
        }
    }
    // the six generators are independent over the real numbers: the Gram matrix Re tr(S_k^dag S_l) is not singular
    let mut gram = CMat::zeros(6);
    for k in 0..6 {
        for l in 0..6 {
            gram.set(k, l, c(inner(&lorentz16[k], &lorentz16[l]).re, 0.0));
        }
    }
    let gram_min = eigh(&gram).0[0];
    rep.add_bool(
        "QN-2a",
        "the six real generators S(u) = diag(-M(u)^dag, M(u)) on each field satisfy S([u, u']) = [S(u), S(u')] and are \
         independent over the real numbers",
        d_lor <= TOL && gram_min > 0.1,
        &format!("largest deviation {:.1e}; smallest eigenvalue of the real Gram matrix {:.4}", d_lor, gram_min),
    );
    let mut d_li = 0.0f64;
    for s in &lorentz16 {
        for (_, x) in &gla {
            d_li = d_li.max(s.comm(x).max_abs());
        }
    }
    rep.add(
        "QN-2b",
        "the Lorentz generators commute with T^a, T^a_R, Y, Q, F_tot, R (space-time and internal symmetries)",
        d_li,
        TOL,
    );

    // ------------------------------------------------------------------ QN-3: the free Hamiltonians
    // the free Hamiltonian of the three fields with the masses m_LL, m^chi_SH and m^chi'_SH
    let h16 = |q: &[f64; 3], mchi: f64, mchi2: f64| -> CMat {
        CMat::block_diag(&CMat::block_diag(&h_free(q, M_LL), &h_free4(q, mchi)), &h_free4(q, mchi2))
    };
    let (mut d_free, mut d_eq, mut d_neq) = (0.0f64, 0.0f64, 0.0f64);
    for trial in 0..TRIALS {
        let q = [rng.unit(), rng.unit(), rng.unit()];
        // three different masses, or three equal ones (those of the later stages)
        let (m1, m2) = if trial % 2 == 0 { (M_CHI_TEST, M_CHI2_TEST) } else { (M_LL, M_LL) };
        let h = h16(&q, m1, m2);
        for (_, x) in &gl {
            d_free = d_free.max(h.comm(x).max_abs());
        }
        // equal masses of the two singlets, different from m_LL
        let he = h16(&q, M_CHI_TEST, M_CHI_TEST);
        for x in &g.tr {
            d_eq = d_eq.max(he.comm(x).max_abs());
        }
        // unequal masses of the singlets: [h, T^1_R] = (1/2) [[0, h_chi - h_chi'], [h_chi' - h_chi, 0]] with
        // h_chi - h_chi' = beta (m^chi - m^chi'), whose largest entry is |m^chi - m^chi'| / 2 (beta has the entries 0, 1)
        let hn = h16(&q, M_CHI_TEST, M_CHI2_TEST);
        d_neq = d_neq.max((hn.comm(&g.tr[0]).max_abs() - (M_CHI2_TEST - M_CHI_TEST).abs() / 2.0).abs());
    }
    rep.add(
        "QN-3a",
        &format!(
            "the free Hamiltonian diag(h_Psi(q), h_chi(q), h_chi'(q)) commutes with T^a, Y, Q, F_tot, R, T^3_R ({} random \
             momenta; the masses m_LL = {}, m^chi_SH = {}, m^chi'_SH = {}, or all three {})",
            TRIALS, M_LL, M_CHI_TEST, M_CHI2_TEST, M_LL
        ),
        d_free,
        TOL,
    );
    rep.add(
        "QN-3b",
        &format!(
            "with equal masses of the two singlets (m^chi_SH = m^chi'_SH = {}, m_LL = {}) it commutes also with T^1_R, \
             T^2_R, T^3_R: SU(2)_R is a symmetry of the free Lagrangian",
            M_CHI_TEST, M_LL
        ),
        d_eq,
        TOL,
    );
    rep.add(
        "QN-3c",
        &format!(
            "with unequal masses of the singlets ({} and {}) the largest entry of [h, T^1_R] is |m^chi_SH - m^chi'_SH|/2: \
             the difference of the masses breaks SU(2)_R (deviation from that value)",
            M_CHI_TEST, M_CHI2_TEST
        ),
        d_neq,
        TOL,
    );

    // ------------------------------------------------------------------ QN-4: the charges of the components
    // the table of the components (T^1, T^2, T^1_R and T^2_R are not diagonal and are not listed)
    let chir16 = dirac16(&gamma_ch());
    let field_of = |i: usize| -> (&'static str, &'static str, usize) {
        if i < NP {
            ("Psi_LL", if i % 2 == 0 { "up" } else { "down" }, i / 2)
        } else if i < NP + NX {
            ("chi_SH", "singlet", i - NP)
        } else {
            ("chi'_SH", "singlet", i - NP - NX)
        }
    };
    let mut qrows: Vec<Vec<String>> = vec![];
    for i in 0..N {
        let (field, iso, dix) = field_of(i);
        qrows.push(vec![
            i.to_string(),
            field.to_string(),
            iso.to_string(),
            dix.to_string(),
            frac(chir16.get(i, i).re),
            frac(g.t[2].get(i, i).re),
            frac(g.tr[2].get(i, i).re),
            frac(g.y.get(i, i).re),
            frac(g.f.get(i, i).re),
            frac(g.q.get(i, i).re),
            frac(g.r.get(i, i).re),
        ]);
    }
    write_csv(
        &data.join("sh_quantum_numbers.csv"),
        &["component", "field", "isospin", "Dirac_index", "chirality", "T3", "T3R", "Y", "F", "Q", "R"],
        &qrows,
    );
    // 4 Y = F_tot on Psi_LL and chi_SH (equation (4.1)), 4 Y = -3 F_tot on chi'_SH
    let (mut d_fll, mut d_y) = (0.0f64, 0.0f64);
    for i in 0..N {
        let ratio = if i < NP + NX { 1.0 } else { -3.0 };
        d_fll = d_fll.max((4.0 * g.y.get(i, i).re - ratio * g.f.get(i, i).re).abs());
        let want = if i < NP + NX { 0.25 } else { -0.75 };
        d_y = d_y.max((g.y.get(i, i).re - want).abs());
    }
    rep.add(
        "QN-4a",
        "Y = 1/4 on Psi_LL and chi_SH and -3/4 on chi'_SH; F_LL = 4 Y equals F_tot on Psi_LL and chi_SH (equation (4.1)) \
         and -3 F_tot on chi'_SH",
        d_y.max(d_fll),
        TOL,
    );
    let ev_q = eigh(&g.q).0;
    let mut q_expect = vec![-0.75; 4];
    q_expect.extend(vec![-0.25; 4]);
    q_expect.extend(vec![0.25; 4]);
    q_expect.extend(vec![0.75; 4]);
    let mut d_qc = list_dev(&ev_q, &q_expect);
    for i in 0..N {
        let want = if i >= NP + NX {
            -0.75
        } else if i >= NP {
            0.25
        } else if i % 2 == 0 {
            0.75
        } else {
            -0.25
        };
        d_qc = d_qc.max((g.q.get(i, i).re - want).abs());
    }
    rep.add(
        "QN-4b",
        "Q = T^3 + Y is 3/4 on the up components of Psi_LL, -1/4 on the down components, 1/4 on chi_SH and -3/4 on \
         chi'_SH",
        d_qc,
        TOL,
    );
    let mut cas_l16 = CMat::zeros(N);
    let mut cas_r16 = CMat::zeros(N);
    for a in 0..3 {
        cas_l16 = cas_l16.add(&g.t[a].mul(&g.t[a]));
        cas_r16 = cas_r16.add(&g.tr[a].mul(&g.tr[a]));
    }
    let mut d_cas = 0.0f64;
    for i in 0..N {
        let (wl, wr) = if i < NP { (0.75, 0.0) } else { (0.0, 0.75) };
        d_cas = d_cas.max((cas_l16.get(i, i).re - wl).abs()).max((cas_r16.get(i, i).re - wr).abs());
    }
    let diag16 = |m: &CMat| -> CMat {
        let mut d = CMat::zeros(N);
        for i in 0..N {
            d.set(i, i, m.get(i, i));
        }
        d
    };
    d_cas = d_cas.max(cas_l16.sub(&diag16(&cas_l16)).max_abs()).max(cas_r16.sub(&diag16(&cas_r16)).max_abs());
    rep.add(
        "QN-4c",
        "the Casimir operators: sum_a (T^a)^2 = 3/4 on Psi_LL and 0 on the singlets, sum_a (T^a_R)^2 = 0 on Psi_LL and 3/4 \
         on chi_SH and chi'_SH (the pair of singlets is a doublet of SU(2)_R)",
        d_cas,
        TOL,
    );

    // ------------------------------------------------------------------ QN-5: anomalies
    let mut traces: Vec<C64> = vec![];
    let gm = &chir16;
    traces.push(gm.mul(&g.y).mul(&g.y).mul(&g.y).trace()); // U(1)_Y^3
    traces.push(gm.mul(&g.y).trace()); // mixed gravitational
    for a in 0..3 {
        for b in 0..3 {
            traces.push(gm.mul(&g.t[a].acomm(&g.t[b])).mul(&g.y).trace()); // SU(2)^2 U(1)
            traces.push(gm.mul(&g.y).mul(&g.y).mul(&g.t[a]).trace());
            for cc in 0..3 {
                traces.push(gm.mul(&g.t[a]).mul(&g.t[b].acomm(&g.t[cc])).trace()); // SU(2)^3
            }
        }
    }
    let d_anom = traces.iter().fold(0.0f64, |m, z| m.max(z.abs()));
    rep.add(
        "QN-5a",
        &format!(
            "the anomaly traces tr(gamma^ch Y^3), tr(gamma^ch Y), tr(gamma^ch {{T^a, T^b}} Y), tr(gamma^ch Y^2 T^a), \
             tr(gamma^ch T^a {{T^b, T^c}}) vanish ({} traces)",
            traces.len()
        ),
        d_anom,
        TOL,
    );
    let par16 = dirac16(&gamma(0));
    let d_vec = gla.iter().fold(0.0f64, |m, (_, x)| m.max(par16.comm(x).max_abs()));
    rep.add(
        "QN-5b",
        "the fields are vector-like: gamma^0, which exchanges the two chiralities, commutes with every internal generator",
        d_vec,
        TOL,
    );
    let n_doublet_comp = eigh(&cas_l16).0.iter().filter(|x| (*x - 0.75).abs() < 1e-9).count();
    // a Weyl doublet has two spin components and two isospin components
    let n_weyl_doublets = n_doublet_comp / 4;
    rep.add_bool(
        "QN-5c",
        "the number of Weyl doublets of SU(2)_L is even (no global anomaly; the right-handed doublet counts as the \
         conjugate of a left-handed one)",
        n_doublet_comp % 4 == 0 && n_weyl_doublets % 2 == 0,
        &format!("{} components with Casimir 3/4, {} Weyl doublets", n_doublet_comp, n_weyl_doublets),
    );
    vals.push(("weyl_doublets".into(), n_weyl_doublets as f64));

    // ------------------------------------------------------------------ QN-6: the Lorentz-invariant bilinears
    let sec = |kind: Kind, f1: Field, f2: Field| Sector { kind, f1, f2 };
    let (pr, pa) = (Kind::Pair, Kind::PartAnti);
    let (fp, fc, fc2, fs) = (Field::Psi, Field::Chi, Field::Chi2, Field::Sing);
    let sectors = [
        sec(pr, fp, fp),
        sec(pr, fp, fc),
        sec(pr, fp, fc2),
        sec(pr, fc, fc),
        sec(pr, fc, fc2),
        sec(pr, fc2, fc2),
        sec(pa, fp, fp),
        sec(pa, fp, fc),
        sec(pa, fp, fc2),
        sec(pa, fc, fc),
        sec(pa, fc, fc2),
        sec(pa, fc2, fc2),
    ];
    let want_dim = [6usize, 4, 4, 2, 2, 2, 8, 4, 4, 2, 2, 2];
    let mut spaces: Vec<Vec<CMat>> = vec![];
    let mut gaps: Vec<(f64, f64)> = vec![];
    let (mut ok_pair, mut ok_pa) = (true, true);
    let (mut det_pair, mut det_pa) = (vec![], vec![]);
    for (k, sc) in sectors.iter().enumerate() {
        let maps = invariance_maps(sc, &lb, sc.same_pair(), false);
        let (basis, zmax, nzmin) = null_space(sc.f1.n(), sc.f2.n(), &maps);
        let ok = basis.len() == want_dim[k] && zmax < 1e-12 && nzmin > 1e-2;
        let d = format!("{} {}", sc.name(), basis.len());
        if sc.kind == Kind::Pair {
            ok_pair &= ok;
            det_pair.push(d);
        } else {
            ok_pa &= ok;
            det_pa.push(d);
        }
        vals.push((format!("dim_{}", key(&sc.name())), basis.len() as f64));
        gaps.push((zmax, nzmin));
        spaces.push(basis);
    }
    let gz = |r: std::ops::Range<usize>| gaps[r.clone()].iter().fold(0.0f64, |m, g| m.max(g.0));
    let gn = |r: std::ops::Range<usize>| gaps[r].iter().fold(f64::INFINITY, |m, g| m.min(g.1));
    rep.add_bool(
        "QN-6a",
        "the Lorentz-invariant pairs of the six sectors: dimensions 6 (Psi Psi, antisymmetric by Fermi statistics), 4 \
         (Psi chi), 4 (Psi chi'), 2 (chi chi, antisymmetric), 2 (chi chi'), 2 (chi' chi', antisymmetric)",
        ok_pair,
        &format!(
            "{}; eigenvalues counted as zero at most {:.1e}, the others at least {:.3}",
            det_pair.join(", "),
            gz(0..6),
            gn(0..6)
        ),
    );
    rep.add_bool(
        "QN-6b",
        "the Lorentz-invariant particle-antiparticle bilinears of the six sectors: dimensions 8 (Psi^dag Psi), 4 \
         (Psi^dag chi), 4 (Psi^dag chi'), 2 (chi^dag chi), 2 (chi^dag chi'), 2 (chi'^dag chi')",
        ok_pa,
        &format!(
            "{}; eigenvalues counted as zero at most {:.1e}, the others at least {:.3}",
            det_pa.join(", "),
            gz(6..12),
            gn(6..12)
        ),
    );
    // the symmetric Lorentz-invariant pairs of two particles of one field (absent by Fermi statistics)
    let sec_pp = sectors[0];
    let (sym_basis, sym_z, sym_nz) = null_space(NP, NP, &invariance_maps(&sec_pp, &lb, false, true));
    let sym_c = null_space(NX, NX, &invariance_maps(&sectors[3], &lb, false, true)).0.len();
    let sym_c2 = null_space(NX, NX, &invariance_maps(&sectors[5], &lb, false, true)).0.len();
    rep.add_bool(
        "QN-6c",
        "the symmetric Lorentz-invariant matrices of two particles of one field, which give no operator (Fermi \
         statistics): dimension 2 for Psi Psi, 0 for chi chi and chi' chi'",
        sym_basis.len() == 2 && sym_z < 1e-12 && sym_nz > 1e-2 && sym_c == 0 && sym_c2 == 0,
        &format!(
            "dimensions {}, {}, {}; gap {:.1e} / {:.3}",
            sym_basis.len(),
            sym_c,
            sym_c2,
            sym_z,
            sym_nz
        ),
    );
    // the explicit basis of the doublet-singlet pairs: C gamma^ch (x) e_s and C (x) e_s, the same for both singlets
    let cg = cmat_c().mul(&gamma_ch());
    let cc4 = cmat_c();
    let kplus: Vec<CMat> = (0..2).map(|s| cg.kron(&e_iso(s))).collect();
    let kminus: Vec<CMat> = (0..2).map(|s| cc4.kron(&e_iso(s))).collect();
    let (sec_pc, sec_pc2) = (sectors[1], sectors[2]);
    let mut d_expl = 0.0f64;
    let explicit: Vec<&CMat> = kplus.iter().chain(kminus.iter()).collect();
    for (ks, sc) in [(1usize, sec_pc), (2usize, sec_pc2)] {
        for (i, kk) in explicit.iter().enumerate() {
            for mu in &lb {
                let dm = delta(Kind::Pair, &lorentz_on(Field::Psi, mu), &lorentz_on(sc.f2, mu), kk);
                d_expl = d_expl.max(dm.max_abs());
            }
            let mut proj = CMat::new(NP, NX);
            for b in &spaces[ks] {
                proj = proj.add(&b.scale(inner(b, kk)));
            }
            d_expl = d_expl.max(proj.sub(kk).max_abs());
            for (j, kj) in explicit.iter().enumerate() {
                if i != j {
                    d_expl = d_expl.max(inner(kk, kj).abs());
                }
            }
        }
    }
    rep.add(
        "QN-6d",
        "the four matrices C gamma^ch (x) e_s and C (x) e_s (s = up, down) are Lorentz invariant and orthogonal, and they \
         span the pairs Psi chi and the pairs Psi chi'",
        d_expl,
        TOL,
    );
    // the sectors of the custodial symmetry: the pair of singlets (chi, chi') taken as one field
    let csectors = [sec(pr, fp, fs), sec(pr, fs, fs), sec(pa, fp, fs), sec(pa, fs, fs)];
    let want_cdim = [8usize, 6, 8, 8];
    let mut cspaces: Vec<Vec<CMat>> = vec![];
    let mut ok_c = true;
    let mut det_c = vec![];
    for (k, sc) in csectors.iter().enumerate() {
        let (basis, zmax, nzmin) = null_space(sc.f1.n(), sc.f2.n(), &invariance_maps(sc, &lb, sc.same_pair(), false));
        ok_c &= basis.len() == want_cdim[k] && zmax < 1e-12 && nzmin > 1e-2;
        det_c.push(format!("{} {}", sc.name(), basis.len()));
        cspaces.push(basis);
    }
    rep.add_bool(
        "QN-6e",
        "with the two singlets taken as one field (chi, chi'), the Lorentz-invariant bilinears have the dimensions 8 \
         (pairs with Psi = 4 + 4), 6 (antisymmetric pairs of two singlets = 2 + 2 + 2), 8 (Psi^dag (chi, chi') = 4 + 4) \
         and 8 ((chi, chi')^dag (chi, chi'), the four sectors chi^dag chi, chi^dag chi', chi'^dag chi, chi'^dag chi')",
        ok_c,
        &det_c.join(", "),
    );

    // ------------------------------------------------------------------ QN-7: parity of the pair matrices
    let mut d_par = 0.0f64;
    for sc in [&sec_pc, &sec_pc2] {
        for kk in &kplus {
            d_par = d_par.max(parity_map(sc, kk).sub(kk).max_abs());
        }
        for kk in &kminus {
            d_par = d_par.max(parity_map(sc, kk).add(kk).max_abs());
        }
    }
    rep.add(
        "QN-7",
        "parity (phi -> gamma^0 phi, intrinsic parities +1): gamma^0 K gamma^0 = +K for C gamma^ch (x) e_s (spin 0, \
         positive parity) and -K for C (x) e_s, with either singlet",
        d_par,
        TOL,
    );

    // ------------------------------------------------------------------ QN-8: the two pair fields and the bidoublet
    let nrm = kplus[0].fro();
    let hbasis: Vec<CMat> = kplus.iter().map(|k| k.scale_r(1.0 / nrm)).collect();
    let phi_ch = Channel::new(sec_pc, 1, hbasis.clone(), &gl);
    let phi2_ch = Channel::new(sec_pc2, 1, hbasis.clone(), &gl);
    let tau_half = |a: usize| sigma(a + 1).scale_r(0.5);
    let mut d_gen = 0.0f64;
    let mut d_gen2 = 0.0f64;
    for a in 0..3 {
        d_gen = d_gen.max(phi_ch.gens[a].sub(&tau_half(a)).max_abs());
        d_gen2 = d_gen2.max(phi2_ch.gens[a].sub(&tau_half(a)).max_abs());
    }
    d_gen = d_gen.max(phi_ch.gens[IX_Y].sub(&i2.scale_r(0.5)).max_abs());
    d_gen2 = d_gen2.max(phi2_ch.gens[IX_Y].add(&i2.scale_r(0.5)).max_abs());
    rep.add(
        "QN-8a",
        "on Phi_SH = (Pi[C gamma^ch (x) e_up], Pi[C gamma^ch (x) e_down]) / norm(C gamma^ch), the pairs with chi_SH: \
         G_{T^a} = tau^a/2 and G_Y = 1/2, the generators of the Higgs doublet Phi^SM",
        d_gen,
        TOL,
    );
    rep.add(
        "QN-8b",
        "on Phi'_SH, the same pairs with chi'_SH: G_{T^a} = tau^a/2 and G_Y = -1/2, a doublet with the opposite \
         hypercharge",
        d_gen2,
        TOL,
    );
    rep.add(
        "QN-8c",
        "the two pair fields span invariant spaces (closure of L_X for T^a, Y, Q, F_tot, R, T^3_R)",
        phi_ch.closure.max(phi2_ch.closure),
        TOL,
    );
    let qd1 = CMat::m2(C64::ONE, C64::ZERO, C64::ZERO, C64::ZERO);
    let qd2 = CMat::m2(C64::ZERO, C64::ZERO, C64::ZERO, -C64::ONE);
    let d_qfr = phi_ch.gens[IX_Q]
        .sub(&qd1)
        .max_abs()
        .max(phi2_ch.gens[IX_Q].sub(&qd2).max_abs())
        .max(phi_ch.gens[IX_F].sub(&i2.scale_r(2.0)).max_abs())
        .max(phi2_ch.gens[IX_F].sub(&i2.scale_r(2.0)).max_abs())
        .max(phi_ch.gens[IX_R].max_abs())
        .max(phi2_ch.gens[IX_R].max_abs())
        .max(phi_ch.gens[IX_TR[2]].sub(&i2.scale_r(0.5)).max_abs())
        .max(phi2_ch.gens[IX_TR[2]].add(&i2.scale_r(0.5)).max_abs());
    rep.add(
        "QN-8d",
        "G_Q = diag(1, 0) on Phi_SH and diag(0, -1) on Phi'_SH (the neutral components are Phi^d_SH and Phi'^u_SH); \
         G_F = 2 and G_R = 0 on both; G_{T^3_R} = 1/2 on Phi_SH and -1/2 on Phi'_SH",
        d_qfr,
        TOL,
    );
    rep.add(
        "QN-8e",
        "the Casimir sum_a G_{T^a}^2 = 3/4 on the components of both pair fields (two doublets)",
        list_dev(&phi_ch.casimir(), &[0.75, 0.75]).max(list_dev(&phi2_ch.casimir(), &[0.75, 0.75])),
        TOL,
    );
    // the conjugate doublet i tau^2 (Phi'_SH)^*: if Phi' -> exp(i eps G) Phi', then i tau^2 Phi'^* -> exp(i eps G~)
    // i tau^2 Phi'^* with G~ = -(i tau^2) G^* (i tau^2)^T
    let eps2 = CMat::m2(C64::ZERO, C64::ONE, -C64::ONE, C64::ZERO);
    let conj_gen = |gm: &CMat| -> CMat { eps2.mul(&gm.conj()).mul(&eps2.transpose()).scale_r(-1.0) };
    let mut d_conj = 0.0f64;
    for a in 0..3 {
        d_conj = d_conj.max(conj_gen(&phi2_ch.gens[a]).sub(&tau_half(a)).max_abs());
    }
    d_conj = d_conj.max(conj_gen(&phi2_ch.gens[IX_Y]).sub(&i2.scale_r(0.5)).max_abs());
    rep.add(
        "QN-8f",
        "the conjugate i tau^2 (Phi'_SH)^* transforms with G~ = -(i tau^2) G^* (i tau^2)^T: tau^a/2 and Y = 1/2, the \
         generators of the Higgs doublet (a second field with its quantum numbers)",
        d_conj,
        TOL,
    );
    // the bidoublet: the pairs of Psi_LL with the pair of singlets, the components M_{s a} (s = up, down; a = chi,
    // chi'), in the order (up chi, up chi', down chi, down chi'); M_{s a} has C gamma^ch (x) e_s / norm in the block of
    // the columns of the singlet a
    let sec_bd = csectors[0];
    let bbasis: Vec<CMat> = (0..4)
        .map(|k| {
            let (s, a) = (k / 2, k % 2);
            let mut m = CMat::new(NP, 2 * NX);
            m.put(0, NX * a, &hbasis[s]);
            m
        })
        .collect();
    let bd = Channel::new(sec_bd, 1, bbasis.clone(), &gla);
    let mut d_bd = 0.0f64;
    for a in 0..3 {
        d_bd = d_bd.max(bd.gens[a].sub(&tau_half(a).kron(&i2)).max_abs());
        d_bd = d_bd.max(bd.gens[IX_TR[a]].sub(&i2.kron(&tau_half(a))).max_abs());
    }
    d_bd = d_bd
        .max(bd.gens[IX_Y].sub(&i2.kron(&tau_half(2))).max_abs())
        .max(bd.gens[IX_F].sub(&CMat::identity(4).scale_r(2.0)).max_abs())
        .max(bd.gens[IX_R].max_abs())
        .max(bd.closure);
    let (bd_content, bd_ok) = su2x2_content(&bd.cas_l(), &bd.cas_r());
    rep.add_bool(
        "QN-8g",
        "the bidoublet M_SH = (Phi_SH, Phi'_SH): on its components (up chi, up chi', down chi, down chi') G_{T^a} = \
         (tau^a/2) (x) 1_2 and G_{T^a_R} = 1_2 (x) (tau^a/2), so that M_SH -> L M_SH R^T; G_Y = 1_2 (x) tau^3/2, G_F = 2, \
         G_R = 0; the space is invariant under all ten generators, and it is the multiplet (2,2) of SU(2)_L x SU(2)_R",
        d_bd <= TOL && bd_ok && bd_content == "(2,2)",
        &format!("largest deviation {:.1e}; multiplets {}", d_bd, bd_content),
    );

    // ------------------------------------------------------------------ QN-9: finite transformations
    let (mut d_fin, mut d_bdf, mut d_unit, mut d_det, mut d_lor_fin) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut det_moved = 0.0f64;
    for _ in 0..TRIALS {
        // an element of SU(2)_L x U(1)_Y: U = exp(i (alpha.T + beta Y)) on the sixteen components; on the doublets
        // exp(i (alpha.tau/2 + beta/2)) (Phi_SH) and exp(i (alpha.tau/2 - beta/2)) (Phi'_SH)
        let al = [PI * rng.unit(), PI * rng.unit(), PI * rng.unit()];
        let be = 2.0 * PI * rng.unit();
        let mut x16 = g.y.scale_r(be);
        let mut x2 = i2.scale_r(0.5 * be);
        let mut x2p = i2.scale_r(-0.5 * be);
        for a in 0..3 {
            x16 = x16.add(&g.t[a].scale_r(al[a]));
            x2 = x2.add(&tau_half(a).scale_r(al[a]));
            x2p = x2p.add(&tau_half(a).scale_r(al[a]));
        }
        let u16 = x16.scale(C64::I).expm();
        let (usm, usm2) = (x2.scale(C64::I).expm(), x2p.scale(C64::I).expm());
        let (cm, r1) = trans_matrix(Kind::Pair, &on_field(&u16, fp), &on_field(&u16, fc), &hbasis);
        let (cm2, r2) = trans_matrix(Kind::Pair, &on_field(&u16, fp), &on_field(&u16, fc2), &hbasis);
        d_fin = d_fin.max(r1).max(r2).max(cm.sub(&usm).max_abs()).max(cm2.sub(&usm2).max_abs());
        // an element of SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot: exp(i (alpha.T + gamma.T_R + theta_R R + theta_F F))
        let al2 = [PI * rng.unit(), PI * rng.unit(), PI * rng.unit()];
        let ga = [PI * rng.unit(), PI * rng.unit(), PI * rng.unit()];
        let (th_r, th_f) = (2.0 * PI * rng.unit(), 2.0 * PI * rng.unit());
        let mut xc = g.r.scale_r(th_r).add(&g.f.scale_r(th_f));
        let (mut xl, mut xr) = (CMat::zeros(2), CMat::zeros(2));
        for a in 0..3 {
            xc = xc.add(&g.t[a].scale_r(al2[a])).add(&g.tr[a].scale_r(ga[a]));
            xl = xl.add(&tau_half(a).scale_r(al2[a]));
            xr = xr.add(&tau_half(a).scale_r(ga[a]));
        }
        let uc = xc.scale(C64::I).expm();
        let (lm, rm) = (xl.scale(C64::I).expm(), xr.scale(C64::I).expm());
        let (cb, rb) = trans_matrix(Kind::Pair, &on_field(&uc, fp), &on_field(&uc, fs), &bbasis);
        // M -> e^{2 i theta_F} L M R^T is, on the components ordered row by row, the matrix e^{2 i theta_F} L (x) R
        let want = lm.kron(&rm).scale(C64::cis(2.0 * th_f));
        d_bdf = d_bdf.max(rb).max(cb.sub(&want).max_abs());
        for m in [&cm, &cm2] {
            d_unit = d_unit.max(m.dagger().mul(m).sub(&i2).max_abs());
        }
        d_unit = d_unit.max(cb.dagger().mul(&cb).sub(&CMat::identity(4)).max_abs());
        // the determinant term eps_st Pi^s Pi'^t = det M (M = [[Pi^u, Pi'^u], [Pi^d, Pi'^d]]) is multiplied by
        // e^{4 i theta_F}
        let w = random_vec(&mut rng, 4);
        let wt = cb.matvec(&w);
        let det = |v: &[C64]| v[0] * v[3] - v[1] * v[2];
        let ph4 = C64::cis(4.0 * th_f);
        d_det = d_det.max((det(&wt) - ph4 * det(&w)).abs());
        det_moved = det_moved.max((det(&wt) - det(&w)).abs());
        // a Lorentz transformation: every invariant bilinear of every sector is unchanged
        let mg = random_unit_biq(&mut rng, 1.0);
        let s4 = s_spin(&mg);
        for (k, sc) in sectors.iter().enumerate() {
            let (q1, q2) = (dirac_on(sc.f1, &s4), dirac_on(sc.f2, &s4));
            for b in &spaces[k] {
                d_lor_fin = d_lor_fin.max(transform(sc.kind, &q1, &q2, b).sub(b).max_abs());
            }
        }
        for (k, sc) in csectors.iter().enumerate() {
            let (q1, q2) = (dirac_on(sc.f1, &s4), dirac_on(sc.f2, &s4));
            for b in &cspaces[k] {
                d_lor_fin = d_lor_fin.max(transform(sc.kind, &q1, &q2, b).sub(b).max_abs());
            }
        }
    }
    rep.add(
        "QN-9a",
        &format!(
            "for {} random elements of SU(2)_L x U(1)_Y: Phi_SH -> exp(i alpha.tau/2 + i beta/2) Phi_SH, the \
             transformation of Phi^SM, and Phi'_SH -> exp(i alpha.tau/2 - i beta/2) Phi'_SH",
            TRIALS
        ),
        d_fin,
        1e-11,
    );
    rep.add(
        "QN-9b",
        &format!(
            "for {} random elements exp(i (alpha.T + gamma.T_R + theta_R R + theta_F F_tot)) of SU(2)_L x SU(2)_R x U(1)_R \
             x U(1)_F_tot: M_SH -> e^(2 i theta_F) L M_SH R^T with L = exp(i alpha.tau/2) and R = exp(i gamma.tau/2)",
            TRIALS
        ),
        d_bdf,
        1e-11,
    );
    rep.add(
        "QN-9c",
        "the matrices of these transformations are unitary: the pairing term (4.2), minus G_SH times the sum of the \
         squared lengths of the two pair fields (= tr(M_SH^dag M_SH)), is invariant under both groups",
        d_unit,
        1e-11,
    );
    rep.add_bool(
        "QN-9d",
        "the determinant term eps_st Pi^s_SH Pi'^t_SH = det M_SH of consequence 3(c) is multiplied by e^(4 i theta_F): it \
         is invariant under SU(2)_L x SU(2)_R x U(1)_R, hence under SU(2)_L x U(1)_Y, and it breaks U(1)_F_tot",
        d_det <= 1e-11 && det_moved > 1e-3,
        &format!(
            "largest deviation from e^(4 i theta_F) det {:.1e}; largest change of det itself {:.3}",
            d_det, det_moved
        ),
    );
    rep.add(
        "QN-9e",
        &format!(
            "for {} random elements g of SL(2,C) every Lorentz-invariant bilinear of QN-6 (the twelve sectors and the four \
             custodial ones) is unchanged by phi -> S(g) phi",
            TRIALS
        ),
        d_lor_fin,
        1e-10,
    );

    // ------------------------------------------------------------------ QN-10: the twenty-four channels
    let mut channels: Vec<Channel> = vec![];
    let mut d_split = 0.0f64;
    for (k, sc) in sectors.iter().enumerate() {
        let (plus, minus, dev) = split_parity(sc, &spaces[k]);
        d_split = d_split.max(dev);
        channels.push(Channel::new(*sc, 1, plus, &gl));
        channels.push(Channel::new(*sc, -1, minus, &gl));
    }
    let d_clos = channels.iter().fold(0.0f64, |m, ch| m.max(ch.closure));
    rep.add(
        "QN-10a",
        "each of the twelve spaces of QN-6 is the sum of a part of parity +1 and one of parity -1, and each of the 24 \
         parts is invariant under T^a, Y, Q, F_tot, R, T^3_R (a representation)",
        d_split.max(d_clos),
        TOL,
    );
    let mut crow: Vec<Vec<String>> = vec![];
    let mut content: HashMap<String, (String, Vec<f64>, Vec<f64>, Vec<f64>)> = HashMap::new();
    let mut d_content_fit = true;
    for ch in &channels {
        let (su2, ok) = su2_content(&ch.casimir());
        d_content_fit &= ok;
        let (y, q, fq, r, t3r) =
            (ch.charges(IX_Y), ch.charges(IX_Q), ch.charges(IX_F), ch.charges(IX_R), ch.charges(IX_TR[2]));
        let neutral = q.iter().filter(|x| x.abs() < 1e-9).count();
        crow.push(vec![
            ch.sec.name(),
            match ch.sec.kind {
                Kind::Pair => "pair".into(),
                Kind::PartAnti => "particle-antiparticle".into(),
            },
            if ch.parity > 0 { "+1".into() } else { "-1".into() },
            ch.dim().to_string(),
            su2.clone(),
            fracs(&y),
            fracs(&q),
            neutral.to_string(),
            fracs(&fq),
            fracs(&r),
            fracs(&t3r),
        ]);
        content.insert(ch.label(), (su2, y, q, t3r));
    }
    write_csv(
        &data.join("sh_channels.csv"),
        &["sector", "kind", "parity", "dimension", "SU2_multiplets", "Y", "Q", "neutral_components", "F", "R", "T3R"],
        &crow,
    );
    rep.add_bool(
        "QN-10b",
        "the eigenvalues of every Casimir operator are those of whole SU(2) multiplets",
        d_content_fit,
        &format!("{} channels", channels.len()),
    );
    // the expectations, sector by sector (both parities): (sector, multiplets, Y, Q (descending), T^3_R)
    let expect: Vec<(&str, &str, Vec<f64>, Vec<f64>, Vec<f64>)> = vec![
        ("Psi Psi", "3", vec![0.5; 3], vec![1.5, 0.5, -0.5], vec![0.0; 3]),
        ("Psi chi", "2", vec![0.5; 2], vec![1.0, 0.0], vec![0.5; 2]),
        ("Psi chi'", "2", vec![-0.5; 2], vec![0.0, -1.0], vec![-0.5; 2]),
        ("chi chi", "1", vec![0.5], vec![0.5], vec![1.0]),
        ("chi chi'", "1", vec![-0.5], vec![-0.5], vec![0.0]),
        ("chi' chi'", "1", vec![-1.5], vec![-1.5], vec![-1.0]),
        ("Psi^dag Psi", "1 + 3", vec![0.0; 4], vec![1.0, 0.0, 0.0, -1.0], vec![0.0; 4]),
        ("Psi^dag chi", "2", vec![0.0; 2], vec![0.5, -0.5], vec![0.5; 2]),
        ("Psi^dag chi'", "2", vec![-1.0; 2], vec![-0.5, -1.5], vec![-0.5; 2]),
        ("chi^dag chi", "1", vec![0.0], vec![0.0], vec![0.0]),
        ("chi^dag chi'", "1", vec![-1.0], vec![-1.0], vec![-1.0]),
        ("chi'^dag chi'", "1", vec![0.0], vec![0.0], vec![0.0]),
    ];
    let mut bad: Vec<String> = vec![];
    let mut d_ch = 0.0f64;
    for (sname, su2, y, q, t3r) in &expect {
        for par in ["+", "-"] {
            let lab = format!("{} ({})", sname, par);
            match content.get(&lab) {
                Some((s, yy, qq, tt)) => {
                    if s != su2 {
                        bad.push(format!("{}: {} instead of {}", lab, s, su2));
                    }
                    d_ch = d_ch.max(list_dev(yy, y)).max(list_dev(qq, q)).max(list_dev(tt, t3r));
                }
                None => bad.push(format!("{} missing", lab)),
            }
        }
    }
    rep.add_bool(
        "QN-10c",
        "the channels (each for both parities): Psi Psi = triplet, Y = 1/2, Q = 3/2, 1/2, -1/2 (equation (3.1): the \
         isospin singlet is symmetric and absent); Psi chi = doublet, Y = 1/2, Q = 1, 0; Psi chi' = doublet, Y = -1/2, \
         Q = 0, -1; chi chi, chi chi', chi' chi' = singlets with Y = Q = 1/2, -1/2, -3/2; Psi^dag Psi = 1 + 3, Y = 0; \
         Psi^dag chi = doublet, Y = 0, Q = 1/2, -1/2; Psi^dag chi' = doublet, Y = -1, Q = -1/2, -3/2; chi^dag chi and \
         chi'^dag chi' = neutral singlets; chi^dag chi' = singlet, Y = Q = -1; and the values of T^3_R",
        bad.is_empty() && d_ch <= TOL,
        &if bad.is_empty() { format!("largest deviation of a charge {:.1e}", d_ch) } else { bad.join("; ") },
    );
    // the symmetric Psi Psi matrices (QN-6c) are the isospin singlets
    let sym_ch = Channel::new(sec_pp, 0, sym_basis.clone(), &gl);
    let (sym_su2, _) = su2_content(&sym_ch.casimir());
    rep.add_bool(
        "QN-10d",
        "the symmetric Lorentz-invariant Psi Psi matrices of QN-6c form isospin singlets (2 (x) 2 = 1 + 3: the singlet \
         is the part that Fermi statistics removes)",
        sym_su2 == "1 + 1" && sym_ch.closure <= TOL,
        &format!("multiplets {}", sym_su2),
    );
    let no_neutral = ["Psi Psi", "chi chi", "chi' chi'", "chi chi'"];
    let min_q_pp = channels
        .iter()
        .filter(|ch| no_neutral.contains(&ch.sec.name().as_str()))
        .flat_map(|ch| ch.charges(IX_Q))
        .fold(f64::INFINITY, |m, x| m.min(x.abs()));
    rep.add_bool(
        "QN-10e",
        "no pair of two particles of one field and no pair of chi_SH with chi'_SH is neutral: a condensate of such pairs \
         would carry electric charge",
        min_q_pp > 0.49,
        &format!("smallest absolute value of Q in these channels {}", frac(min_q_pp)),
    );
    vals.push(("min_abs_Q_pairs_without_Psi_chi".into(), min_q_pp));
    // the neutral components of the pairs: one in each channel Psi chi and Psi chi', none elsewhere
    let mut neutral_pairs: Vec<String> = vec![];
    let mut ok_np = true;
    for ch in channels.iter().filter(|ch| ch.sec.kind == Kind::Pair) {
        let nn = ch.charges(IX_Q).iter().filter(|x| x.abs() < 1e-9).count();
        let want = if ch.sec.f1 == Field::Psi && ch.sec.f2 != Field::Psi { 1 } else { 0 };
        ok_np &= nn == want;
        if nn > 0 {
            neutral_pairs.push(format!("{}: {}", ch.label(), nn));
        }
    }
    rep.add_bool(
        "QN-10f",
        "the neutral pairs: exactly one neutral component in each of the channels Psi chi and Psi chi' (the down \
         particle with chi_SH and the up particle with chi'_SH, by QN-8d) and none in the other pair channels",
        ok_np,
        &neutral_pairs.join("; "),
    );
    // the custodial content of the four sectors of the pair of singlets
    let mut cchannels: Vec<Channel> = vec![];
    let mut d_csplit = 0.0f64;
    for (k, sc) in csectors.iter().enumerate() {
        let (plus, minus, dev) = split_parity(sc, &cspaces[k]);
        d_csplit = d_csplit.max(dev);
        cchannels.push(Channel::new(*sc, 1, plus, &gla));
        cchannels.push(Channel::new(*sc, -1, minus, &gla));
    }
    let want_c = ["(2,2)", "(2,2)", "(1,3)", "(1,3)", "(2,2)", "(2,2)", "(1,1) + (1,3)", "(1,1) + (1,3)"];
    let mut ccrow: Vec<Vec<String>> = vec![];
    let mut ok_cc = true;
    let mut det_cc: Vec<String> = vec![];
    for (k, ch) in cchannels.iter().enumerate() {
        let (cont, ok) = su2x2_content(&ch.cas_l(), &ch.cas_r());
        ok_cc &= ok && cont == want_c[k] && ch.closure <= TOL;
        det_cc.push(format!("{} {}", ch.label(), cont));
        ccrow.push(vec![
            ch.sec.name(),
            match ch.sec.kind {
                Kind::Pair => "pair".into(),
                Kind::PartAnti => "particle-antiparticle".into(),
            },
            if ch.parity > 0 { "+1".into() } else { "-1".into() },
            ch.dim().to_string(),
            cont,
            fracs(&ch.charges(IX_F)),
            fracs(&ch.charges(IX_R)),
        ]);
    }
    write_csv(
        &data.join("sh_custodial_channels.csv"),
        &["sector", "kind", "parity", "dimension", "SU2L_x_SU2R_multiplets", "F", "R"],
        &ccrow,
    );
    rep.add_bool(
        "QN-10g",
        "the custodial multiplets (both parities, each part invariant under all ten generators): Psi (chi, chi') pairs \
         (2,2); pairs of two singlets (1,3) (chi chi, chi chi', chi' chi' form a triplet of SU(2)_R); Psi^dag (chi, chi') \
         (2,2); (chi, chi')^dag (chi, chi') (1,1) + (1,3)",
        ok_cc && d_csplit <= TOL,
        &det_cc.join("; "),
    );

    // ------------------------------------------------------------------ QN-11: the breaking
    let mut d_rot = 0.0f64;
    for _ in 0..TRIALS {
        let w = random_vec(&mut rng, 2);
        let len = (w[0].norm_sqr() + w[1].norm_sqr()).sqrt();
        // U = ((phi_d, -phi_u), (phi_u^*, phi_d^*)) / |Phi| is in SU(2) and U Phi = (0, |Phi|)
        let u = CMat::m2(w[1], -w[0], w[0].conj(), w[1].conj()).scale_r(1.0 / len);
        let uw0 = u.get(0, 0) * w[0] + u.get(0, 1) * w[1];
        let uw1 = u.get(1, 0) * w[0] + u.get(1, 1) * w[1];
        d_rot = d_rot
            .max(uw0.abs())
            .max((uw1 - c(len, 0.0)).abs())
            .max(u.dagger().mul(&u).sub(&i2).max_abs())
            .max((u.det2() - C64::ONE).abs());
    }
    rep.add(
        "QN-11a",
        &format!(
            "every doublet is brought to the form (0, phi) with real phi > 0 by an element of SU(2)_L ({} random doublets)",
            TRIALS
        ),
        d_rot,
        TOL,
    );
    let sq = 0.5f64.sqrt();
    // the generators T^1, T^2, T^3, Y on the two pair fields
    let hg: Vec<CMat> = phi_ch.gens[0..4].to_vec();
    let hg2: Vec<CMat> = phi2_ch.gens[0..4].to_vec();
    // the unit vector of the stabilizer in the coordinates (T^1, T^2, T^3, Y), oriented, compared with Q = T^3 + Y
    let along_q = |st: &[Vec<f64>]| -> f64 {
        if st.len() != 1 {
            return f64::INFINITY;
        }
        let s = &st[0];
        let sgn = if s[3] < 0.0 { -1.0 } else { 1.0 };
        list_dev(&s.iter().map(|x| sgn * x).collect::<Vec<_>>(), &[0.0, 0.0, sq, sq])
    };
    let phi0 = vec![C64::ZERO, C64::ONE];
    let phi0p = vec![C64::ONE, C64::ZERO];
    let (st1, sz1, snz1) = stabilizer(&[(&hg, &phi0)]);
    // the neutral vacuum of H5 with different, random lengths and phases of the two condensates
    let mut d_st2 = 0.0f64;
    for _ in 0..TRIALS {
        let z1 = c(rng.unit(), rng.unit());
        let z2 = c(rng.unit(), rng.unit()).scale(2.0);
        let (st, _, _) = stabilizer(&[(&hg, &[C64::ZERO, z1]), (&hg2, &[z2, C64::ZERO])]);
        d_st2 = d_st2.max(along_q(&st));
    }
    rep.add_bool(
        "QN-11b",
        &format!(
            "the generators of SU(2)_L x U(1)_Y that leave Phi_SH = (0, phi) unchanged span the direction of T^3 + Y = Q; \
             so do those that leave the neutral vacuum Phi_SH = (0, phi), Phi'_SH = (phi', 0) of H5 unchanged ({} random \
             phi, phi' of different lengths): SU(2)_L x U(1)_Y -> U(1)_Q",
            TRIALS
        ),
        along_q(&st1) <= 1e-12 && d_st2 <= 1e-9,
        &format!(
            "deviation of the unit vector from (0, 0, 1, 1)/sqrt2: {:.1e} (Phi_SH alone), {:.1e} (both); gap {:.1e} / {:.3}",
            along_q(&st1),
            d_st2,
            sz1,
            snz1
        ),
    );
    // the rotation of a neutral vacuum with equal lengths to phi i tau^2 by T^3 and F_tot: M = [[0, a], [b, 0]] with
    // |a| = |b| = phi; exp(i theta_3 T^3) multiplies the row of up by e^{i theta_3/2} and that of down by
    // e^{-i theta_3/2}, exp(i theta_F F_tot) both by e^{2 i theta_F}; a -> phi and b -> -phi need
    // theta_3/2 + 2 theta_F = -arg a and -theta_3/2 + 2 theta_F = pi - arg b
    let mut d_itau = 0.0f64;
    for _ in 0..TRIALS {
        let phase_a = PI * rng.unit();
        let phase_b = PI * rng.unit();
        let len = 0.5 + 0.5 * (rng.unit() + 1.0);
        let (a, b) = (C64::cis(phase_a).scale(len), C64::cis(phase_b).scale(len));
        let th_f = (PI - phase_a - phase_b) / 4.0;
        let th3 = phase_b - phase_a - PI;
        let u = g.t[2].scale_r(th3).add(&g.f.scale_r(th_f)).scale(C64::I).expm();
        let (cb, _) = trans_matrix(Kind::Pair, &on_field(&u, fp), &on_field(&u, fs), &bbasis);
        let wt = cb.matvec(&[C64::ZERO, a, b, C64::ZERO]);
        let want = [C64::ZERO, c(len, 0.0), c(-len, 0.0), C64::ZERO];
        d_itau = d_itau.max((0..4).fold(0.0f64, |m, i| m.max((wt[i] - want[i]).abs())));
    }
    rep.add(
        "QN-11c",
        &format!(
            "a neutral vacuum with equal lengths, M_SH = [[0, a], [b, 0]] with |a| = |b| = phi, is brought to phi i tau^2 \
             by exp(i theta_3 T^3 + i theta_F F_tot) ({} random phases and lengths); the product a b, which SU(2)_L x \
             U(1)_Y does not change (QN-9d), needs F_tot",
            TRIALS
        ),
        d_itau,
        1e-12,
    );
    // the stabilizer of phi i tau^2 in the eight generators (T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot)
    let order8 = [0usize, 1, 2, IX_TR[0], IX_TR[1], IX_TR[2], IX_R, IX_F];
    let g8: Vec<CMat> = order8.iter().map(|&i| bd.gens[i].clone()).collect();
    let w_it = vec![C64::ZERO, c(sq, 0.0), c(-sq, 0.0), C64::ZERO];
    let (st8, sz8, snz8) = stabilizer(&[(&g8, &w_it)]);
    let unit8 = |idx: &[usize], cf: &[f64]| -> Vec<f64> {
        let mut v = vec![0.0; 8];
        for (k, &i) in idx.iter().enumerate() {
            v[i] = cf[k];
        }
        let n = v.iter().map(|x| x * x).sum::<f64>().sqrt();
        v.iter().map(|x| x / n).collect()
    };
    let st8_want = vec![
        unit8(&[0, 3], &[1.0, 1.0]),
        unit8(&[1, 4], &[1.0, 1.0]),
        unit8(&[2, 5], &[1.0, 1.0]),
        unit8(&[6], &[1.0]),
    ];
    let d_st8 = span_dev(&st8, &st8_want);
    // Q = T^3 + T^3_R + R/4 lies in the stabilizer
    let qv8 = unit8(&[2, 5, 6], &[1.0, 1.0, 0.25]);
    let q_in = (0..8).fold(0.0f64, |m, i| {
        let pq: f64 = st8.iter().map(|s| s[i] * s.iter().zip(&qv8).map(|(x, y)| x * y).sum::<f64>()).sum();
        m.max((pq - qv8[i]).abs())
    });
    // the gauged directions T^1, T^2, T^3 and Y = T^3_R + R/4, their intersection with the stabilizer, and the rank of
    // the stabilizer together with the gauged directions and F_tot
    let gauge8 = vec![unit8(&[0], &[1.0]), unit8(&[1], &[1.0]), unit8(&[2], &[1.0]), unit8(&[5, 6], &[1.0, 0.25])];
    let gy: Vec<CMat> = vec![bd.gens[0].clone(), bd.gens[1].clone(), bd.gens[2].clone(), bd.gens[IX_Y].clone()];
    let (st_gauge, _, _) = stabilizer(&[(&gy, &w_it)]);
    let mut all_dirs: Vec<Vec<f64>> = st8.clone();
    all_dirs.extend(gauge8.iter().cloned());
    all_dirs.push(unit8(&[7], &[1.0]));
    let rank_all = real_rank(&all_dirs);
    let rank_sg = {
        let mut v = st8.clone();
        v.extend(gauge8.iter().cloned());
        real_rank(&v)
    };
    let n_broken = 8 - st8.len();
    let n_broken_gauge = 4 - st_gauge.len();
    rep.add_bool(
        "QN-11d",
        "the vacuum phi i tau^2 keeps four of the eight generators of SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot: T^a + T^a_R \
         (the custodial SU(2)_V) and R; Q = T^3 + T^3_R + R/4 lies among them; four are broken, three of them in \
         SU(2)_L x U(1)_Y (whose stabilizer is the direction of Q) and the fourth F_tot, which is not a combination of \
         the kept and the gauged generators: three Goldstone modes for W and Z and one physical",
        st8.len() == 4
            && d_st8 <= 1e-9
            && q_in <= 1e-9
            && along_q(&st_gauge) <= 1e-9
            && n_broken_gauge == 3
            && rank_sg == 7
            && rank_all == 8,
        &format!(
            "stabilizer of dimension {} (deviation of its projector {:.1e}; gap {:.1e} / {:.3}); Q in it to {:.1e}; broken \
             {} of 8, {} of the 4 gauged; rank of kept + gauged {}, with F_tot {}",
            st8.len(),
            d_st8,
            sz8,
            snz8,
            q_in,
            n_broken,
            n_broken_gauge,
            rank_sg,
            rank_all
        ),
    );
    vals.push(("broken_generators".into(), n_broken as f64));
    vals.push(("broken_gauge_generators".into(), n_broken_gauge as f64));
    // equal lengths with arbitrary phases: a stabilizer of dimension 4 (conjugate to SU(2)_V and R); unequal lengths:
    // dimension 2, spanned by T^3 + T^3_R and R (SU(2)_V broken)
    let (mut ok_eq, mut d_uneq) = (true, 0.0f64);
    let st2_want = vec![unit8(&[2, 5], &[1.0, 1.0]), unit8(&[6], &[1.0])];
    for _ in 0..TRIALS {
        let (pa_, pb_) = (PI * rng.unit(), PI * rng.unit());
        let a = C64::cis(pa_);
        let b = C64::cis(pb_);
        let (st_e, _, _) = stabilizer(&[(&g8, &[C64::ZERO, a, b, C64::ZERO])]);
        ok_eq &= st_e.len() == 4;
        let r = 1.5 + 0.4 * rng.unit();
        let (st_u, _, _) = stabilizer(&[(&g8, &[C64::ZERO, a.scale(r), b, C64::ZERO])]);
        d_uneq = d_uneq.max(span_dev(&st_u, &st2_want));
    }
    rep.add_bool(
        "QN-11e",
        &format!(
            "a neutral vacuum with equal lengths and any phases keeps four generators (a conjugate of SU(2)_V, and R); \
             with unequal lengths it keeps only T^3 + T^3_R and R: the custodial SU(2)_V needs equal lengths ({} random \
             vacua of each kind)",
            TRIALS
        ),
        ok_eq && d_uneq <= 1e-9,
        &format!("equal lengths: dimension 4 in all trials: {}; unequal: deviation of the projector {:.1e}", ok_eq, d_uneq),
    );
    // a second condensate in one of the 24 channels together with the neutral vacuum (equal lengths)
    let mut lines: Vec<String> = vec![];
    let mut ok_generic = true;
    let mut ok_neutral = true;
    for ch in &channels {
        let gch: Vec<CMat> = ch.gens[0..4].to_vec();
        // a generic vector of the channel
        let w = random_vec(&mut rng, ch.dim());
        let (st, _, _) = stabilizer(&[(&hg, &phi0), (&hg2, &phi0p), (&gch, &w)]);
        let all_neutral = ch.charges(IX_Q).iter().all(|x| x.abs() < 1e-9);
        let want = if all_neutral { 1 } else { 0 };
        ok_generic &= st.len() == want;
        // a vector of the neutral part of the channel (the eigenvectors of G_Q with eigenvalue 0), when there is one
        let (qe, qv) = eigh(&ch.gens[IX_Q]);
        let neutral_cols: Vec<usize> = (0..ch.dim()).filter(|&j| qe[j].abs() < 1e-9).collect();
        let mut dn = String::from("-");
        if !neutral_cols.is_empty() {
            let cf = random_vec(&mut rng, neutral_cols.len());
            let mut wn = vec![C64::ZERO; ch.dim()];
            for (k, &j) in neutral_cols.iter().enumerate() {
                for i in 0..ch.dim() {
                    wn[i] += qv.get(i, j) * cf[k];
                }
            }
            let (stn, _, _) = stabilizer(&[(&hg, &phi0), (&hg2, &phi0p), (&gch, &wn)]);
            let aq = along_q(&stn) < 1e-9;
            ok_neutral &= aq;
            dn = format!("{}{}", stn.len(), if aq { " (Q)" } else { "" });
        }
        lines.push(format!("{}: generic {}, neutral {}", ch.label(), st.len(), dn));
    }
    rep.add_bool(
        "QN-11f",
        "a generic second condensate in any channel with a charged component, added to the neutral vacuum of H5, leaves \
         no generator of SU(2)_L x U(1)_Y unbroken (the photon would become massive); in the neutral channels \
         chi^dag chi and chi'^dag chi' it leaves Q",
        ok_generic,
        &lines.join("; "),
    );
    rep.add_bool(
        "QN-11g",
        "a second condensate in the neutral part of any channel (Q = 0) keeps exactly Q unbroken: the mean fields \
         compatible with U(1)_Q are the neutral ones",
        ok_neutral,
        "dimension of the stabilizer 1 and along Q in every channel with a neutral part",
    );

    // ------------------------------------------------------------------ QN-12: the gauge-boson masses and the couplings
    let inp = read_inputs(&data.join("sh_inputs.csv"));
    let get = |k: &str| inp.get(k).unwrap_or_else(|| panic!("input {} missing in sh_inputs.csv", k)).0;
    let (gf, mw, mz, mh, s2hat, ainv) = (get("G_F"), get("M_W"), get("M_Z"), get("m_H"), get("s2_hat_Z"), get("alpha_inverse"));
    // the couplings for which the relations (2.2) hold with the measured masses: v = (sqrt2 G_F)^(-1/2),
    // g = 2 M_W / v, cos(theta_W) = M_W / M_Z, g' = g tan(theta_W), e = g sin(theta_W)
    let v = 1.0 / (2.0f64.sqrt() * gf).sqrt();
    let gw = 2.0 * mw / v;
    let cw = mw / mz;
    let sw = (1.0 - cw * cw).sqrt();
    let gp = gw * sw / cw;
    let e = gw * sw;
    let gf_ll = gp / 4.0;
    rep.add_bool(
        "QN-12a",
        "v = (sqrt2 G_F)^(-1/2) = 246.22 GeV to the five digits stated in H0",
        (v - 246.22).abs() < 0.005,
        &format!("v = {:.4} GeV", v),
    );
    // the mass matrix M^2_AB = g_A g_B [Phi^dag {G_A, G_B} Phi + Phi'^dag {G'_A, G'_B} Phi'] of the canonical kinetic
    // term of the two pair fields, with Phi = (0, phi), Phi' = (phi', 0), phi = v_1/sqrt2, phi' = v_2/sqrt2 (so that
    // v_SH^2 = v_1^2 + v_2^2 = 2 (phi^2 + phi'^2)) and the generators of QN-8
    let ga = [gw, gw, gw, gp];
    let mass_matrix = |v1: f64, v2: f64| -> CMat {
        let phi = [C64::ZERO, c(v1 / 2.0f64.sqrt(), 0.0)];
        let phip = [c(v2 / 2.0f64.sqrt(), 0.0), C64::ZERO];
        let mut m2 = CMat::zeros(4);
        for a in 0..4 {
            for b in 0..4 {
                let (ac, ac2) = (hg[a].acomm(&hg[b]), hg2[a].acomm(&hg2[b]));
                let mut s = C64::ZERO;
                for i in 0..2 {
                    for j in 0..2 {
                        s += phi[i].conj() * ac.get(i, j) * phi[j] + phip[i].conj() * ac2.get(i, j) * phip[j];
                    }
                }
                m2.set(a, b, c(ga[a] * ga[b] * s.re, 0.0));
            }
        }
        m2
    };
    let want_l = [0.0, (gw * v / 2.0).powi(2), (gw * v / 2.0).powi(2), (gw * gw + gp * gp) * v * v / 4.0];
    // the vacuum of equal lengths, v_1 = v_2 = v_SH / sqrt2 (phi = phi' = v_SH / 2)
    let m2 = mass_matrix(v * sq, v * sq);
    let (lam, vecs) = eigh(&m2);
    let d_lam = list_dev(&lam, &want_l) / want_l[3];
    rep.add(
        "QN-12b",
        "for the vacuum Phi_SH = (0, phi), Phi'_SH = (phi', 0) of equal lengths, phi = phi' = v_SH/2, the eigenvalues of the \
         mass matrix are 0, M_W^2, M_W^2, M_Z^2 with M_W = g v_SH/2 and M_Z = sqrt(g^2 + g'^2) v_SH/2, where \
         v_SH^2 = 2 (phi^2 + phi'^2) (relative deviation)",
        d_lam,
        1e-13,
    );
    let d_meas = ((lam[1].sqrt() - mw).abs() / mw).max((lam[3].sqrt() - mz).abs() / mz);
    rep.add(
        "QN-12c",
        "with v_SH = v (H6) and these couplings the two masses are the measured M_W and M_Z (relative deviation)",
        d_meas,
        1e-13,
    );
    // the massless vector: (0, 0, sin, cos) in the basis (W^1, W^2, W^3, B); the Z: (0, 0, cos, -sin)
    let col = |j: usize| -> Vec<f64> {
        let vv: Vec<f64> = (0..4).map(|a| vecs.get(a, j).re).collect();
        let big = vv.iter().fold(0.0f64, |m, x| if x.abs() > m.abs() { *x } else { m });
        let ph = vecs.get(vv.iter().position(|x| *x == big).unwrap(), j);
        let ph = ph.conj().scale(1.0 / ph.abs());
        (0..4).map(|a| (vecs.get(a, j) * ph).re * if big < 0.0 { -1.0 } else { 1.0 }).collect()
    };
    let a_vec = col(0);
    let z_vec = col(3);
    let sgn_a = if a_vec[3] < 0.0 { -1.0 } else { 1.0 };
    let sgn_z = if z_vec[2] < 0.0 { -1.0 } else { 1.0 };
    let d_a = list_dev(&a_vec.iter().map(|x| sgn_a * x).collect::<Vec<_>>(), &[0.0, 0.0, sw, cw]);
    let d_z = list_dev(&z_vec.iter().map(|x| sgn_z * x).collect::<Vec<_>>(), &[0.0, 0.0, cw, -sw]);
    rep.add(
        "QN-12d",
        "the massless field is A = sin(theta_W) W^3 + cos(theta_W) B and the field of mass M_Z is \
         Z = cos(theta_W) W^3 - sin(theta_W) B, with tan(theta_W) = g'/g (the conventions (10.3) of the Review)",
        d_a.max(d_z),
        1e-12,
    );
    // the charges of the gauge fields under the unbroken U(1)_Q: the adjoint action of Q = T^3 + Y on (W^1, W^2, W^3, B)
    let mut adq = CMat::zeros(4);
    adq.put(0, 0, &adj_gen(3));
    let (qe, qv) = eigh(&adq.transpose());
    let mut d_wq = list_dev(&qe, &[-1.0, 0.0, 0.0, 1.0]);
    // the eigenvector of charge +1 of the transpose (the coefficients of the field) is (1, -i, 0, 0)/sqrt2: W^+
    let jp = qe.iter().position(|x| (x - 1.0).abs() < 1e-9).unwrap_or(0);
    let wp: Vec<C64> = (0..4).map(|a| qv.get(a, jp)).collect();
    let ph = wp[0].conj().scale(1.0 / wp[0].abs().max(1e-300));
    let wp: Vec<C64> = wp.iter().map(|z| *z * ph).collect();
    d_wq = d_wq
        .max((wp[0] - c(sq, 0.0)).abs())
        .max((wp[1] - c(0.0, -sq)).abs())
        .max(wp[2].abs())
        .max(wp[3].abs());
    // and W^+ lies in the eigenspace of M_W^2 (the plane of W^1 and W^2)
    let mut mwp = vec![C64::ZERO; 4];
    for a in 0..4 {
        for b in 0..4 {
            mwp[a] += m2.get(a, b) * wp[b];
        }
    }
    let d_wm = (0..4).fold(0.0f64, |m, a| m.max((mwp[a] - wp[a].scale(lam[1])).abs())) / lam[1];
    rep.add(
        "QN-12e",
        "under the unbroken U(1)_Q the field (W^1 - i W^2)/sqrt2 = W^+ has the charge +1 and the mass M_W; W^3 and B are \
         neutral (the eigenvalues of the adjoint action of Q are -1, 0, 0, 1)",
        d_wq.max(d_wm),
        1e-12,
    );
    // rho for the vacuum of equal lengths and for random ratios of the two lengths (both doublets have T = |Y| = 1/2)
    let rho = lam[1] / (lam[3] * cw * cw);
    let mut d_rho = (rho - 1.0).abs();
    for _ in 0..TRIALS {
        let ang = 0.5 * PI * 0.5 * (rng.unit() + 1.0);
        let (l2, _) = eigh(&mass_matrix(v * ang.cos(), v * ang.sin()));
        d_rho = d_rho.max(list_dev(&l2, &want_l) / want_l[3]).max((l2[1] / (l2[3] * cw * cw) - 1.0).abs());
    }
    rep.add(
        "QN-12f",
        &format!(
            "rho = M_W^2 / (M_Z^2 cos^2 theta_W) = 1 for a canonical kinetic term of the two pair fields, for the vacuum of \
             equal lengths and for {} random ratios of the lengths, with the same eigenvalues (the value of the dynamics \
             is computed in stage 4)",
            TRIALS
        ),
        d_rho,
        1e-13,
    );
    rep.add(
        "QN-12g",
        "e = g g' / sqrt(g^2 + g'^2) = g sin(theta_W), and g^F_LL = g'/4 reproduces g' Y = g^F_LL F_LL with F_LL = 4 Y on \
         all three fields (Y = 1/4, 1/4, -3/4)",
        ((gw * gp / (gw * gw + gp * gp).sqrt() - e).abs() / e)
            .max([0.25f64, -0.75].iter().fold(0.0f64, |m, yv| m.max((gf_ll * 4.0 * yv - gp * yv).abs()))),
        1e-14,
    );
    rep.note(&format!(
        "couplings of the set used here (tree-level relations exact with the measured M_W and M_Z): g = {:.6}, \
         g' = {:.6}, e = {:.6}, sin^2(theta_W) = 1 - M_W^2/M_Z^2 = {:.5}, g^F_LL = g'/4 = {:.6}",
        gw,
        gp,
        e,
        sw * sw,
        gf_ll
    ));
    rep.note(&format!(
        "e^2/(4 pi) = 1/{:.3} for these couplings; the measured 1/alpha = {:.6} at zero momentum transfer (about 128 at \
         M_Z); the MS-bar sin^2(theta-hat(M_Z)) = {} differs from 1 - M_W^2/M_Z^2 by a radiative correction of the \
         Standard Model",
        4.0 * PI / (e * e),
        ainv,
        s2hat
    ));
    rep.note(&format!(
        "the measured masses with the MS-bar angle give M_W^2/(M_Z^2 (1 - sin^2(theta-hat(M_Z)))) = {:.5}; in the \
         Standard Model this deviation from 1 comes from radiative corrections, mainly those of the top quark. A test of \
         the rho of the superfluid (stage 4) needs an angle measured independently of M_W/M_Z, and these corrections",
        mw * mw / (mz * mz * (1.0 - s2hat))
    ));
    rep.note(&format!(
        "m_H = {} GeV (input; compared with the amplitude mode in stage 4): m_H / v = {:.5}",
        mh,
        mh / v
    ));

    // ------------------------------------------------------------------ data files
    let names = ["W1", "W2", "W3", "B"];
    let mut mrows: Vec<Vec<String>> = vec![];
    for a in 0..4 {
        let mut r = vec![names[a].to_string()];
        for b in 0..4 {
            r.push(f(m2.get(a, b).re));
        }
        mrows.push(r);
    }
    write_csv(&data.join("sh_mass_matrix.csv"), &["field", "W1", "W2", "W3", "B"], &mrows);
    let coup: Vec<(&str, &str, f64, &str, &str)> = vec![
        ("v", "v = (sqrt2 G_F)^(-1/2)", v, "GeV", "from G_F"),
        ("g", "g = 2 M_W / v", gw, "1", "from M_W and G_F"),
        ("cos_theta_W", "cos(theta_W) = M_W / M_Z", cw, "1", "from M_W and M_Z"),
        ("sin2_theta_W", "sin^2(theta_W) = 1 - M_W^2/M_Z^2", sw * sw, "1", "from M_W and M_Z"),
        ("g_prime", "g' = g tan(theta_W)", gp, "1", "from M_W, M_Z and G_F"),
        ("e", "e = g sin(theta_W)", e, "1", "from M_W, M_Z and G_F"),
        ("g_v_LL", "g^v_LL = g (H2)", gw, "1", "the coupling of SU(2)_v of the earlier hypotheses"),
        ("g_F_LL", "g^F_LL = g'/4 (H2)", gf_ll, "1", "the coupling of U(1)^F_LL of the earlier hypotheses"),
        ("M_W_tree", "g v / 2", lam[1].sqrt(), "GeV", "eigenvalue of the mass matrix"),
        ("M_Z_tree", "sqrt(g^2 + g'^2) v / 2", lam[3].sqrt(), "GeV", "eigenvalue of the mass matrix"),
        ("m_H_over_v", "m_H / v", mh / v, "1", "measured m_H and v"),
        ("alpha_inverse_tree", "4 pi / e^2", 4.0 * PI / (e * e), "1", "for these couplings"),
        ("rho_with_s2_hat", "M_W^2 / (M_Z^2 (1 - s2_hat_Z))", mw * mw / (mz * mz * (1.0 - s2hat)), "1", "MS-bar angle"),
    ];
    let crows: Vec<Vec<String>> =
        coup.iter().map(|(n, s, x, u, w)| vec![n.to_string(), s.to_string(), f(*x), u.to_string(), w.to_string()]).collect();
    write_csv(&data.join("sh_couplings.csv"), &["name", "definition", "value", "unit", "from"], &crows);
    for (n, _, x, _, _) in &coup {
        vals.push((n.to_string(), *x));
    }
    vals.push(("gap_zero_max".into(), gaps.iter().fold(0.0f64, |m, g| m.max(g.0))));
    vals.push(("gap_nonzero_min".into(), gaps.iter().fold(f64::INFINITY, |m, g| m.min(g.1))));
    let vrows: Vec<Vec<String>> = vals.iter().map(|(k, x)| vec![k.clone(), format!("{:.12e}", x)]).collect();
    write_csv(&data.join("sh_quantum_values.csv"), &["name", "value"], &vrows);
    rep.write(&data.join("hck_sh_quantum_checks.txt"));
    let (np, nn) = rep.counts();
    println!("{} of {} checks passed", np, nn);
    std::process::exit(if np == nn { 0 } else { 1 });
}
