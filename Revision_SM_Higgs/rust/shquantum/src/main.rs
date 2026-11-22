//! sh_quantum -- the quantum numbers of the fields, of their spin-zero bilinears and of the superfluid under the revised
//! hypotheses H1-H5 of the folder Revision_SM_Higgs of PROJECT spacetimes (provenance document H0), and the pattern of
//! the breaking of SU(2)_L x U(1)_Y by the superfluid.
//!
//! What is computed.  The calculation is group theory: no dynamics enters.  Whether the superfluid forms, its gap and
//! the masses of its excitations are computed by the later programs of the folder.  Every statement of H0 that follows
//! from the hypotheses alone is checked here in floating point.  The Python program python/sh_quantum.py repeats the
//! checks with numpy (the identifiers with the prefix Y) and the Wolfram script wolfram/source/sh_quantum.wls repeats
//! them in exact arithmetic (the prefix W).
//!
//! The field.  The twelve components of the field are numbered 0..11.  The components 0..7 are those of the isodoublet
//! Psi_LL (component 2 d + s, with the Dirac index d = 0..3 and the isospin index s = 0 (up), 1 (down), the convention of
//! the library sfcore), the components 8..11 those of the singlet chi_SH (component 8 + d).  The Dirac matrices are
//! those of the chiral basis of sfcore: d = 0, 1 is the block of chirality -1 (left-handed), d = 2, 3 the block of
//! chirality +1 (right-handed).
//!
//! The bilinears.  A spin-zero bilinear of two fields phi_1, phi_2 (each Psi_LL or chi_SH) is either a pair of two
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
//!   QN-1   the internal generators T^a, Y, F, R, Q on the twelve components: the algebra of SU(2)_L x U(1)_Y and
//!          Hermiticity;
//!   QN-2   the six real generators of SL(2,C): their algebra, and that they commute with the internal generators;
//!   QN-3   the free Hamiltonians of the two fields commute with the internal generators, for unequal masses as well;
//!   QN-4   the charges of the components: Y = F/4 = 1/4 on all; Q = T^3 + Y = 3/4, -1/4, 1/4;
//!   QN-5   no gauge anomaly (traces with the chirality), vector-like fields, an even number of Weyl doublets (Witten);
//!   QN-6   the Lorentz-invariant pairs: the dimensions 4 (Psi chi), 6 (Psi Psi), 2 (chi chi), and the explicit basis
//!          C gamma^ch (x) e_s, C (x) e_s of the Psi chi pairs; the particle-antiparticle bilinears (8, 4, 2);
//!   QN-7   the parity of the pair matrices C gamma^ch (x) e_s (+1) and C (x) e_s (-1);
//!   QN-8   the doublet-singlet pairs of positive parity carry exactly the generators of the Higgs doublet:
//!          G_{T^a} = tau^a / 2, G_Y = 1/2, G_Q = diag(1, 0), G_F = 2, G_R = 0, Casimir 3/4;
//!   QN-9   finite transformations (pseudo-random, seeded): the pair field transforms as the Higgs doublet, it is
//!          Lorentz invariant, and the pairing term (4.2) of H0 is invariant;
//!   QN-10  the twelve channels of spin-zero bilinears and their charges: no pair of two Psi_LL particles or of two
//!          chi_SH particles is neutral;
//!   QN-11  the breaking: every doublet is rotated to (0, phi); the generator that leaves (0, phi) unchanged is
//!          Q = T^3 + Y; a condensate in a charged channel added to it leaves no generator unbroken, a neutral one
//!          keeps Q;
//!   QN-12  the mass matrix of W^a and B from a canonical kinetic term of the pair field: the masses (2.2) of H0, the
//!          massless photon A = sin(theta_W) W^3 + cos(theta_W) B, the charges +1 and -1 of W^+ and W^-, rho = 1, and
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

/// the numbers of components of Psi_LL, of chi_SH and of the whole field
const NP: usize = 8;
const NX: usize = 4;
const N: usize = NP + NX;
/// the tolerance of the checks of exact identities evaluated in floating point
const TOL: f64 = 1e-12;
/// an eigenvalue of a Gram matrix below this value is counted as zero; the others are of order 1 (the report gives the
/// largest eigenvalue counted as zero and the smallest one counted as not zero, so that the gap can be seen)
const ZERO_EIG: f64 = 1e-10;
/// the masses of the check QN-3 in units of the cutoff Lambda_LL: m_LL of the earlier hypotheses and a different mass of
/// chi_SH, to show that the symmetries do not need the choice m^chi_SH = m_LL (decision D22)
const M_LL: f64 = 0.2;
const M_CHI_TEST: f64 = 0.35;
/// the number of pseudo-random trials of the checks QN-3, QN-9 and QN-11, and the seed of the generator
const TRIALS: usize = 25;
const SEED: u64 = 0x5348_514e_2026;

// ------------------------------------------------------------------------------------------------ fields and generators

/// the two spinor fields of H1
#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    Psi,
    Chi,
}
impl Field {
    /// the first of the twelve components that belongs to the field
    fn off(self) -> usize {
        match self {
            Field::Psi => 0,
            Field::Chi => NP,
        }
    }
    /// the number of components of the field
    fn n(self) -> usize {
        match self {
            Field::Psi => NP,
            Field::Chi => NX,
        }
    }
    fn label(self) -> &'static str {
        match self {
            Field::Psi => "Psi",
            Field::Chi => "chi",
        }
    }
}

/// a Dirac matrix (4 x 4) acting on the components of one field: A (x) 1_2 on Psi_LL, A on chi_SH
fn dirac_on(fl: Field, a4: &CMat) -> CMat {
    match fl {
        Field::Psi => d8(a4),
        Field::Chi => a4.clone(),
    }
}
/// a Dirac matrix acting on both fields (12 x 12)
fn dirac12(a4: &CMat) -> CMat {
    CMat::block_diag(&d8(a4), a4)
}
/// the block of a 12 x 12 operator that acts on one field (the operators used here do not mix the two fields)
fn on_field(x: &CMat, fl: Field) -> CMat {
    x.sub_block(fl.off(), fl.off(), fl.n(), fl.n())
}

/// the internal generators on the twelve components (H1 and H2 of H0)
struct Gens {
    /// T^1, T^2, T^3 = diag(1_4 (x) tau^a / 2, 0_4): SU(2)_L acts on the isospin index of Psi_LL only
    t: Vec<CMat>,
    /// the hypercharge Y = F / 4 (equation (4.1) of H0): 1/4 on every component
    y: CMat,
    /// the fermion number F: 1 on every component (both fields have F = 1)
    f: CMat,
    /// the relative number: +1 on Psi_LL, -1 on chi_SH (a global symmetry of the whole Lagrangian, H4)
    r: CMat,
    /// the electric charge Q = T^3 + Y
    q: CMat,
}
fn internal() -> Gens {
    let t: Vec<CMat> = (1..=3).map(|a| CMat::block_diag(&t_gen(a), &CMat::zeros(NX))).collect();
    let fnum = CMat::identity(N);
    let y = fnum.scale_r(0.25);
    let r = CMat::block_diag(&CMat::identity(NP), &CMat::identity(NX).scale_r(-1.0));
    let q = t[2].add(&y);
    Gens { t, y, f: fnum, r, q }
}
impl Gens {
    /// the generators with their names, in the order of the data files
    fn list(&self) -> Vec<(&'static str, CMat)> {
        vec![
            ("T1", self.t[0].clone()),
            ("T2", self.t[1].clone()),
            ("T3", self.t[2].clone()),
            ("Y", self.y.clone()),
            ("Q", self.q.clone()),
            ("F", self.f.clone()),
            ("R", self.r.clone()),
        ]
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
/// the free Dirac Hamiltonian of chi_SH on a plane wave of momentum q, alpha.q + beta m (4 x 4)
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
    /// G_X for X = T1, T2, T3, Y, Q, F, R (the order of Gens::list)
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
    /// the eigenvalues of the Casimir operator sum_a G_{T^a}^2, ascending
    fn casimir(&self) -> Vec<f64> {
        let k = self.dim();
        let mut cas = CMat::zeros(k);
        for a in 0..3 {
            cas = cas.add(&self.gens[a].mul(&self.gens[a]));
        }
        eigh(&cas).0
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
        "sh_quantum: checks of the quantum numbers and of the breaking under the revised hypotheses (Revision_SM_Higgs, \
         PROJECT spacetimes)",
    );
    let mut vals: Vec<(String, f64)> = vec![];
    let mut rng = Rng(SEED);
    let g = internal();
    let gl = g.list();
    let lb = lorentz_basis();
    let lorentz12: Vec<CMat> = lb.iter().map(|mu| dirac12(&s_gen(mu))).collect();

    // ------------------------------------------------------------------ QN-1: the internal generators
    let mut d_alg = 0.0f64;
    for a in 0..3 {
        for b in 0..3 {
            let mut rhs = CMat::zeros(N);
            for cc in 0..3 {
                rhs = rhs.add(&g.t[cc].scale(c(0.0, levi(a, b, cc))));
            }
            d_alg = d_alg.max(g.t[a].comm(&g.t[b]).sub(&rhs).max_abs());
        }
    }
    rep.add("QN-1a", "[T^a, T^b] = i eps_abc T^c on the twelve components (the algebra of SU(2)_L)", d_alg, TOL);
    let mut d_comm = 0.0f64;
    let abelian = [&g.y, &g.f, &g.r];
    for x in &abelian {
        for a in 0..3 {
            d_comm = d_comm.max(g.t[a].comm(x).max_abs());
        }
        for z in &abelian {
            d_comm = d_comm.max(x.comm(z).max_abs());
        }
    }
    rep.add("QN-1b", "Y, F and R commute with T^1, T^2, T^3 and with each other", d_comm, TOL);
    let d_herm = gl.iter().fold(0.0f64, |m, (_, x)| m.max(x.sub(&x.dagger()).max_abs()));
    rep.add("QN-1c", "T^1, T^2, T^3, Y, Q, F, R are Hermitian (a unitary representation)", d_herm, TOL);

    // ------------------------------------------------------------------ QN-2: the Lorentz generators
    let mut d_lor = 0.0f64;
    for k in 0..6 {
        for l in 0..6 {
            let br = lb[k].comm(&lb[l]);
            let lhs = dirac12(&s_gen(&br));
            d_lor = d_lor.max(lhs.sub(&lorentz12[k].comm(&lorentz12[l])).max_abs());
        }
    }
    // the six generators are independent over the real numbers: the Gram matrix Re tr(S_k^dag S_l) is not singular
    let mut gram = CMat::zeros(6);
    for k in 0..6 {
        for l in 0..6 {
            gram.set(k, l, c(inner(&lorentz12[k], &lorentz12[l]).re, 0.0));
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
    for s in &lorentz12 {
        for (_, x) in &gl {
            d_li = d_li.max(s.comm(x).max_abs());
        }
    }
    rep.add("QN-2b", "the Lorentz generators commute with T^a, Y, Q, F, R (space-time and internal symmetries)", d_li, TOL);

    // ------------------------------------------------------------------ QN-3: the free Hamiltonians
    let mut d_free = 0.0f64;
    for trial in 0..TRIALS {
        let q = [rng.unit(), rng.unit(), rng.unit()];
        let mchi = if trial % 2 == 0 { M_CHI_TEST } else { M_LL };
        let h = CMat::block_diag(&h_free(&q, M_LL), &h_free4(&q, mchi));
        for (_, x) in &gl {
            d_free = d_free.max(h.comm(x).max_abs());
        }
    }
    rep.add(
        "QN-3",
        &format!(
            "the free Hamiltonian diag(h_Psi(q), h_chi(q)) commutes with T^a, Y, Q, F, R ({} random momenta; m_LL = {}, \
             m^chi_SH = {} and {})",
            TRIALS, M_LL, M_CHI_TEST, M_LL
        ),
        d_free,
        TOL,
    );

    // ------------------------------------------------------------------ QN-4: the charges of the components
    // the table of the components (T^1 and T^2 are not diagonal and are not listed)
    let mut qrows: Vec<Vec<String>> = vec![];
    let chir12 = dirac12(&gamma_ch());
    for i in 0..N {
        let (field, iso, dix) = if i < NP {
            ("Psi_LL", if i % 2 == 0 { "up" } else { "down" }, i / 2)
        } else {
            ("chi_SH", "singlet", i - NP)
        };
        let row = vec![
            i.to_string(),
            field.to_string(),
            iso.to_string(),
            dix.to_string(),
            frac(chir12.get(i, i).re),
            frac(g.t[2].get(i, i).re),
            frac(g.y.get(i, i).re),
            frac(g.f.get(i, i).re),
            frac(g.q.get(i, i).re),
            frac(g.r.get(i, i).re),
        ];
        qrows.push(row);
    }
    write_csv(
        &data.join("sh_quantum_numbers.csv"),
        &["component", "field", "isospin", "Dirac_index", "chirality", "T3", "Y", "F", "Q", "R"],
        &qrows,
    );
    let ev_y = eigh(&g.y).0;
    let ev_q = eigh(&g.q).0;
    let d_yf = g.y.sub(&g.f.scale_r(0.25)).max_abs();
    let d_y = ev_y.iter().fold(0.0f64, |m, x| m.max((x - 0.25).abs()));
    rep.add("QN-4a", "Y = F/4 (equation (4.1)) and Y = 1/4 on all twelve components", d_y.max(d_yf), TOL);
    let mut q_expect = vec![-0.25; 4];
    q_expect.extend(vec![0.25; 4]);
    q_expect.extend(vec![0.75; 4]);
    let mut d_qc = list_dev(&ev_q, &q_expect);
    for i in 0..N {
        let want = if i >= NP {
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
        "Q = T^3 + Y is 3/4 on the up components of Psi_LL, -1/4 on the down components and 1/4 on chi_SH",
        d_qc,
        TOL,
    );

    // ------------------------------------------------------------------ QN-5: anomalies
    let mut traces: Vec<C64> = vec![];
    let gm = &chir12;
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
    let par12 = dirac12(&gamma(0));
    let d_vec = gl.iter().fold(0.0f64, |m, (_, x)| m.max(par12.comm(x).max_abs()));
    rep.add(
        "QN-5b",
        "the fields are vector-like: gamma^0, which exchanges the two chiralities, commutes with every internal generator",
        d_vec,
        TOL,
    );
    let mut cas12 = CMat::zeros(N);
    for a in 0..3 {
        cas12 = cas12.add(&g.t[a].mul(&g.t[a]));
    }
    let n_doublet_comp = eigh(&cas12).0.iter().filter(|x| (*x - 0.75).abs() < 1e-9).count();
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
    let sectors = [
        Sector { kind: Kind::Pair, f1: Field::Psi, f2: Field::Psi },
        Sector { kind: Kind::Pair, f1: Field::Psi, f2: Field::Chi },
        Sector { kind: Kind::Pair, f1: Field::Chi, f2: Field::Chi },
        Sector { kind: Kind::PartAnti, f1: Field::Psi, f2: Field::Psi },
        Sector { kind: Kind::PartAnti, f1: Field::Psi, f2: Field::Chi },
        Sector { kind: Kind::PartAnti, f1: Field::Chi, f2: Field::Chi },
    ];
    let want_dim = [6usize, 4, 2, 8, 4, 2];
    let mut spaces: Vec<Vec<CMat>> = vec![];
    let mut gaps: Vec<(f64, f64)> = vec![];
    for (k, sec) in sectors.iter().enumerate() {
        let maps = invariance_maps(sec, &lb, sec.same_pair(), false);
        let (basis, zmax, nzmin) = null_space(sec.f1.n(), sec.f2.n(), &maps);
        let id = ["QN-6a", "QN-6b", "QN-6c", "QN-6d", "QN-6e", "QN-6f"][k];
        let what = match sec.kind {
            Kind::Pair if sec.same_pair() => "Lorentz-invariant pairs, antisymmetric (Fermi statistics)",
            Kind::Pair => "Lorentz-invariant pairs",
            Kind::PartAnti => "Lorentz-invariant particle-antiparticle bilinears",
        };
        rep.add_bool(
            id,
            &format!("{} of the sector {}: dimension {}", what, sec.name(), want_dim[k]),
            basis.len() == want_dim[k] && zmax < 1e-12 && nzmin > 1e-2,
            &format!(
                "dimension {}; largest eigenvalue counted as zero {:.1e}, smallest not zero {:.3}",
                basis.len(),
                zmax,
                nzmin
            ),
        );
        vals.push((format!("dim_{}", sec.name().replace(' ', "_").replace('^', "")), basis.len() as f64));
        gaps.push((zmax, nzmin));
        spaces.push(basis);
    }
    // the symmetric Lorentz-invariant pairs of two Psi_LL particles (absent by Fermi statistics; see QN-10c)
    let sec_pp = sectors[0];
    let (sym_basis, sym_z, sym_nz) = null_space(NP, NP, &invariance_maps(&sec_pp, &lb, false, true));
    rep.add_bool(
        "QN-6g",
        "the symmetric Lorentz-invariant matrices of the sector Psi Psi, which give no operator (Fermi statistics): \
         dimension 2",
        sym_basis.len() == 2 && sym_z < 1e-12 && sym_nz > 1e-2,
        &format!("dimension {}; gap {:.1e} / {:.3}", sym_basis.len(), sym_z, sym_nz),
    );
    // the explicit basis of the Psi chi pairs: K^{0+}_s = C gamma^ch (x) e_s and K^{0-}_s = C (x) e_s
    let cg = cmat_c().mul(&gamma_ch());
    let cc4 = cmat_c();
    let kplus: Vec<CMat> = (0..2).map(|s| cg.kron(&e_iso(s))).collect();
    let kminus: Vec<CMat> = (0..2).map(|s| cc4.kron(&e_iso(s))).collect();
    let sec_pc = sectors[1];
    let mut d_inv = 0.0f64;
    let mut d_span = 0.0f64;
    let mut d_orth = 0.0f64;
    let explicit: Vec<&CMat> = kplus.iter().chain(kminus.iter()).collect();
    for (i, kk) in explicit.iter().enumerate() {
        for mu in &lb {
            let dm = delta(Kind::Pair, &lorentz_on(Field::Psi, mu), &lorentz_on(Field::Chi, mu), kk);
            d_inv = d_inv.max(dm.max_abs());
        }
        let mut proj = CMat::new(NP, NX);
        for b in &spaces[1] {
            proj = proj.add(&b.scale(inner(b, kk)));
        }
        d_span = d_span.max(proj.sub(kk).max_abs());
        for (j, kj) in explicit.iter().enumerate() {
            if i != j {
                d_orth = d_orth.max(inner(kk, kj).abs());
            }
        }
    }
    rep.add(
        "QN-6h",
        "the four matrices C gamma^ch (x) e_s and C (x) e_s (s = up, down) are Lorentz invariant, orthogonal and lie in \
         the space of QN-6b, which they therefore span",
        d_inv.max(d_span).max(d_orth),
        TOL,
    );

    // ------------------------------------------------------------------ QN-7: parity of the Psi chi pair matrices
    let mut d_par = 0.0f64;
    for kk in &kplus {
        d_par = d_par.max(parity_map(&sec_pc, kk).sub(kk).max_abs());
    }
    for kk in &kminus {
        d_par = d_par.max(parity_map(&sec_pc, kk).add(kk).max_abs());
    }
    rep.add(
        "QN-7",
        "parity (phi -> gamma^0 phi, intrinsic parities +1): gamma^0 K gamma^0 = +K for C gamma^ch (x) e_s (spin 0, \
         positive parity) and -K for C (x) e_s",
        d_par,
        TOL,
    );

    // ------------------------------------------------------------------ QN-8: the Higgs channel
    let nrm = kplus[0].fro();
    let hbasis: Vec<CMat> = kplus.iter().map(|k| k.scale_r(1.0 / nrm)).collect();
    let higgs = Channel::new(sec_pc, 1, hbasis.clone(), &gl);
    let mut d_gen = 0.0f64;
    for a in 0..3 {
        d_gen = d_gen.max(higgs.gens[a].sub(&sigma(a + 1).scale_r(0.5)).max_abs());
    }
    d_gen = d_gen.max(higgs.gens[3].sub(&CMat::identity(2).scale_r(0.5)).max_abs());
    rep.add(
        "QN-8a",
        "on Phi_SH = (Pi[C gamma^ch (x) e_up], Pi[C gamma^ch (x) e_down]) / norm(C gamma^ch): G_{T^a} = tau^a / 2 and \
         G_Y = 1/2, the generators of the Higgs doublet Phi^SM",
        d_gen,
        TOL,
    );
    rep.add("QN-8b", "the two pair components span an invariant space (closure of L_X for all seven generators)", higgs.closure, TOL);
    let qdiag = CMat::m2(C64::ONE, C64::ZERO, C64::ZERO, C64::ZERO);
    let d_qfr = higgs.gens[4]
        .sub(&qdiag)
        .max_abs()
        .max(higgs.gens[5].sub(&CMat::identity(2).scale_r(2.0)).max_abs())
        .max(higgs.gens[6].max_abs());
    rep.add(
        "QN-8c",
        "G_Q = diag(1, 0) (the charges +1 and 0 of Phi^u_SH and Phi^d_SH), G_F = 2 and G_R = 0 (the relative number is \
         not changed)",
        d_qfr,
        TOL,
    );
    let cas_h = higgs.casimir();
    rep.add("QN-8d", "the Casimir sum_a G_{T^a}^2 = 3/4 on both components (a doublet)", list_dev(&cas_h, &[0.75, 0.75]), TOL);

    // ------------------------------------------------------------------ QN-9: finite transformations
    let mut d_fin = 0.0f64;
    let mut d_unit = 0.0f64;
    let mut d_lor_fin = 0.0f64;
    for _ in 0..TRIALS {
        let al = [PI * rng.unit(), PI * rng.unit(), PI * rng.unit()];
        let be = 2.0 * PI * rng.unit();
        // U = exp(i (alpha.T + beta Y)) on the twelve components, U_SM = exp(i (alpha.tau/2 + beta/2)) on the doublet
        let mut x12 = g.y.scale_r(be);
        let mut x2 = CMat::identity(2).scale_r(0.5 * be);
        for a in 0..3 {
            x12 = x12.add(&g.t[a].scale_r(al[a]));
            x2 = x2.add(&sigma(a + 1).scale_r(0.5 * al[a]));
        }
        let u12 = x12.scale(C64::I).expm();
        let usm = x2.scale(C64::I).expm();
        let (u_psi, u_chi) = (on_field(&u12, Field::Psi), on_field(&u12, Field::Chi));
        // Pi'_s = Pi[U_Psi^T M_s U_chi] = sum_s' C_{s s'} Pi_{s'} with C_{s s'} = <M_s', U_Psi^T M_s U_chi>
        let mut cm = CMat::zeros(2);
        for s in 0..2 {
            let tm = transform(Kind::Pair, &u_psi, &u_chi, &hbasis[s]);
            let mut rest = tm.clone();
            for sp in 0..2 {
                let cf = inner(&hbasis[sp], &tm);
                cm.set(s, sp, cf);
                rest = rest.sub(&hbasis[sp].scale(cf));
            }
            d_fin = d_fin.max(rest.max_abs());
        }
        d_fin = d_fin.max(cm.sub(&usm).max_abs());
        d_unit = d_unit.max(cm.dagger().mul(&cm).sub(&CMat::identity(2)).max_abs());
        // a Lorentz transformation: every invariant bilinear of every sector is unchanged
        let mg = random_unit_biq(&mut rng, 1.0);
        let s4 = s_spin(&mg);
        for (k, sec) in sectors.iter().enumerate() {
            let (r1, r2) = (dirac_on(sec.f1, &s4), dirac_on(sec.f2, &s4));
            for b in &spaces[k] {
                d_lor_fin = d_lor_fin.max(transform(sec.kind, &r1, &r2, b).sub(b).max_abs());
            }
        }
    }
    rep.add(
        "QN-9a",
        &format!(
            "for {} random elements of SU(2)_L x U(1)_Y the pair field transforms as Phi_SH -> U_SM Phi_SH with \
             U_SM = exp(i alpha.tau/2 + i beta/2), the transformation of Phi^SM",
            TRIALS
        ),
        d_fin,
        1e-11,
    );
    rep.add(
        "QN-9b",
        "the matrix of the transformation of Phi_SH is unitary: the pairing term (4.2), minus G_SH times the sum of \
         Pi_s^dag Pi_s, is invariant",
        d_unit,
        1e-11,
    );
    rep.add(
        "QN-9c",
        &format!(
            "for {} random elements g of SL(2,C) every Lorentz-invariant bilinear of QN-6 (all six sectors) is unchanged \
             by phi -> S(g) phi",
            TRIALS
        ),
        d_lor_fin,
        1e-10,
    );

    // ------------------------------------------------------------------ QN-10: the twelve channels
    let mut channels: Vec<Channel> = vec![];
    let mut d_split = 0.0f64;
    for (k, sec) in sectors.iter().enumerate() {
        let (plus, minus, dev) = split_parity(sec, &spaces[k]);
        d_split = d_split.max(dev);
        channels.push(Channel::new(*sec, 1, plus, &gl));
        channels.push(Channel::new(*sec, -1, minus, &gl));
    }
    let d_clos = channels.iter().fold(0.0f64, |m, ch| m.max(ch.closure));
    rep.add(
        "QN-10a",
        "each of the six spaces of QN-6 is the sum of a part of parity +1 and one of parity -1, and each of the twelve \
         parts is invariant under T^a, Y, Q, F, R (a representation)",
        d_split.max(d_clos),
        TOL,
    );
    // the table of the channels
    let mut crow: Vec<Vec<String>> = vec![];
    let mut content: HashMap<String, (String, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>)> = HashMap::new();
    let mut d_content_fit = true;
    for ch in &channels {
        let (su2, ok) = su2_content(&ch.casimir());
        d_content_fit &= ok;
        let (y, q, fq, r) = (ch.charges(3), ch.charges(4), ch.charges(5), ch.charges(6));
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
        ]);
        content.insert(ch.label(), (su2, y, q, fq, r));
    }
    write_csv(
        &data.join("sh_channels.csv"),
        &["sector", "kind", "parity", "dimension", "SU2_multiplets", "Y", "Q", "neutral_components", "F", "R"],
        &crow,
    );
    rep.add_bool(
        "QN-10b",
        "the eigenvalues of every Casimir operator are those of whole SU(2) multiplets",
        d_content_fit,
        &format!("{} channels", channels.len()),
    );
    // expectations, channel by channel: (label, multiplets, Y, Q (descending))
    let expect: Vec<(&str, &str, Vec<f64>, Vec<f64>)> = vec![
        ("Psi Psi (+)", "3", vec![0.5; 3], vec![1.5, 0.5, -0.5]),
        ("Psi Psi (-)", "3", vec![0.5; 3], vec![1.5, 0.5, -0.5]),
        ("Psi chi (+)", "2", vec![0.5; 2], vec![1.0, 0.0]),
        ("Psi chi (-)", "2", vec![0.5; 2], vec![1.0, 0.0]),
        ("chi chi (+)", "1", vec![0.5], vec![0.5]),
        ("chi chi (-)", "1", vec![0.5], vec![0.5]),
        ("Psi^dag Psi (+)", "1 + 3", vec![0.0; 4], vec![1.0, 0.0, 0.0, -1.0]),
        ("Psi^dag Psi (-)", "1 + 3", vec![0.0; 4], vec![1.0, 0.0, 0.0, -1.0]),
        ("Psi^dag chi (+)", "2", vec![0.0; 2], vec![0.5, -0.5]),
        ("Psi^dag chi (-)", "2", vec![0.0; 2], vec![0.5, -0.5]),
        ("chi^dag chi (+)", "1", vec![0.0], vec![0.0]),
        ("chi^dag chi (-)", "1", vec![0.0], vec![0.0]),
    ];
    let mut bad: Vec<String> = vec![];
    let mut d_ch = 0.0f64;
    for (lab, su2, y, q) in &expect {
        match content.get(*lab) {
            Some((s, yy, qq, _, _)) => {
                if s != su2 {
                    bad.push(format!("{}: {} instead of {}", lab, s, su2));
                }
                d_ch = d_ch.max(list_dev(yy, y)).max(list_dev(qq, q));
            }
            None => bad.push(format!("{} missing", lab)),
        }
    }
    rep.add_bool(
        "QN-10c",
        "the channels: Psi Psi = triplet with Y = 1/2 and Q = 3/2, 1/2, -1/2 (equation (3.1): the isospin singlet is \
         symmetric and absent); Psi chi = doublet, Y = 1/2, Q = 1, 0; chi chi = singlet, Y = Q = 1/2; Psi^dag Psi = \
         1 + 3, Y = 0; Psi^dag chi = doublet, Y = 0, Q = 1/2, -1/2; chi^dag chi = singlet, Y = Q = 0 (each for both \
         parities)",
        bad.is_empty() && d_ch <= TOL,
        &if bad.is_empty() { format!("largest deviation of a charge {:.1e}", d_ch) } else { bad.join("; ") },
    );
    // the symmetric Psi Psi matrices (QN-6g) are the isospin singlet
    let sym_ch = Channel::new(sec_pp, 0, sym_basis.clone(), &gl);
    let (sym_su2, _) = su2_content(&sym_ch.casimir());
    rep.add_bool(
        "QN-10d",
        "the symmetric Lorentz-invariant Psi Psi matrices of QN-6g form isospin singlets (2 (x) 2 = 1 + 3: the singlet \
         is the part that Fermi statistics removes)",
        sym_su2 == "1 + 1" && sym_ch.closure <= TOL,
        &format!("multiplets {}", sym_su2),
    );
    let min_q_pp = channels
        .iter()
        .filter(|ch| ch.sec.same_pair())
        .flat_map(|ch| ch.charges(4))
        .fold(f64::INFINITY, |m, x| m.min(x.abs()));
    rep.add_bool(
        "QN-10e",
        "no pair of two Psi_LL particles and no pair of two chi_SH particles is neutral: a condensate of such pairs \
         would carry electric charge",
        min_q_pp > 0.49,
        &format!("smallest absolute value of Q in these channels {}", frac(min_q_pp)),
    );
    vals.push(("min_abs_Q_same_field_pairs".into(), min_q_pp));

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
            .max(u.dagger().mul(&u).sub(&CMat::identity(2)).max_abs())
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
    let phi0 = vec![C64::ZERO, C64::ONE];
    let hg: Vec<CMat> = higgs.gens[0..4].to_vec();
    let (stab, sz, snz) = stabilizer(&[(&hg, &phi0)]);
    let sq = 0.5f64.sqrt();
    let d_stab = if stab.len() == 1 {
        let s = &stab[0];
        let sgn = if s[3] < 0.0 { -1.0 } else { 1.0 };
        list_dev(&s.iter().map(|x| sgn * x).collect::<Vec<_>>(), &[0.0, 0.0, sq, sq])
    } else {
        f64::INFINITY
    };
    rep.add_bool(
        "QN-11b",
        "the generators of SU(2)_L x U(1)_Y that leave Phi_SH = (0, phi) unchanged form a space of dimension 1, spanned by \
         T^3 + Y = Q: the breaking SU(2)_L x U(1)_Y -> U(1)_Q",
        stab.len() == 1 && d_stab <= 1e-12,
        &format!(
            "dimension {}; deviation of the unit vector from (0, 0, 1, 1)/sqrt2 {:.1e}; gap {:.1e} / {:.3}",
            stab.len(),
            d_stab,
            sz,
            snz
        ),
    );
    // a second condensate in one of the twelve channels together with (0, phi) in the Higgs channel
    let mut lines: Vec<String> = vec![];
    let mut ok_generic = true;
    let mut ok_neutral = true;
    for ch in &channels {
        let gch: Vec<CMat> = ch.gens[0..4].to_vec();
        // a generic vector of the channel
        let w = random_vec(&mut rng, ch.dim());
        let (st, _, _) = stabilizer(&[(&hg, &phi0), (&gch, &w)]);
        let all_neutral = ch.charges(4).iter().all(|x| x.abs() < 1e-9);
        let want = if all_neutral { 1 } else { 0 };
        ok_generic &= st.len() == want;
        // a vector of the neutral part of the channel (the eigenvectors of G_Q with eigenvalue 0), when there is one
        let (qe, qv) = eigh(&ch.gens[4]);
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
            let (stn, _, _) = stabilizer(&[(&hg, &phi0), (&gch, &wn)]);
            let along_q = stn.len() == 1 && {
                let s = &stn[0];
                let sgn = if s[3] < 0.0 { -1.0 } else { 1.0 };
                list_dev(&s.iter().map(|x| sgn * x).collect::<Vec<_>>(), &[0.0, 0.0, sq, sq]) < 1e-9
            };
            ok_neutral &= along_q;
            dn = format!("{}{}", stn.len(), if along_q { " (Q)" } else { "" });
        }
        lines.push(format!("{}: generic {}, neutral {}", ch.label(), st.len(), dn));
    }
    rep.add_bool(
        "QN-11c",
        "a generic second condensate in any channel with a charged component, added to (0, phi), leaves no generator \
         unbroken (the photon would become massive); in the neutral channels chi^dag chi it leaves Q",
        ok_generic,
        &lines.join("; "),
    );
    rep.add_bool(
        "QN-11d",
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
    // the mass matrix M^2_AB = g_A g_B Phi^dag {G_A, G_B} Phi with Phi = (0, v/sqrt2) and the generators of the pair
    // field computed in QN-8 (not those of the Standard Model)
    let ga = [gw, gw, gw, gp];
    let phi = [C64::ZERO, c(v / 2.0f64.sqrt(), 0.0)];
    let mut m2 = CMat::zeros(4);
    for a in 0..4 {
        for b in 0..4 {
            let ac = hg[a].acomm(&hg[b]);
            let mut s = C64::ZERO;
            for i in 0..2 {
                for j in 0..2 {
                    s += phi[i].conj() * ac.get(i, j) * phi[j];
                }
            }
            m2.set(a, b, c(ga[a] * ga[b] * s.re, 0.0));
        }
    }
    let (lam, vecs) = eigh(&m2);
    let want_l = [0.0, (gw * v / 2.0).powi(2), (gw * v / 2.0).powi(2), (gw * gw + gp * gp) * v * v / 4.0];
    let d_lam = list_dev(&lam, &want_l) / want_l[3];
    rep.add(
        "QN-12b",
        "the eigenvalues of the mass matrix are 0, M_W^2, M_W^2, M_Z^2 with M_W = g v_SH/2 and M_Z = sqrt(g^2 + g'^2) \
         v_SH/2 (relative deviation)",
        d_lam,
        1e-13,
    );
    let d_meas = ((lam[1].sqrt() - mw).abs() / mw).max((lam[3].sqrt() - mz).abs() / mz);
    rep.add(
        "QN-12c",
        "with v_SH = v (H5) and these couplings the two masses are the measured M_W and M_Z (relative deviation)",
        d_meas,
        1e-13,
    );
    // the massless vector: (0, 0, sin, cos) in the basis (W^1, W^2, W^3, B); the Z: (0, 0, cos, -sin)
    let col = |j: usize| -> Vec<f64> {
        let v: Vec<f64> = (0..4).map(|a| vecs.get(a, j).re).collect();
        let big = v.iter().fold(0.0f64, |m, x| if x.abs() > m.abs() { *x } else { m });
        let ph = vecs.get(v.iter().position(|x| *x == big).unwrap(), j);
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
    let rho = lam[1] / (lam[3] * cw * cw);
    rep.add(
        "QN-12f",
        "rho = M_W^2 / (M_Z^2 cos^2 theta_W) = 1 for a canonical kinetic term of the doublet (the value of the dynamics \
         is computed in stage 5)",
        (rho - 1.0).abs(),
        1e-13,
    );
    rep.add(
        "QN-12g",
        "e = g g' / sqrt(g^2 + g'^2) = g sin(theta_W), and g^F_LL = g'/4 reproduces g' Y = g^F_LL F on both fields",
        ((gw * gp / (gw * gw + gp * gp).sqrt() - e).abs() / e).max((gf_ll * 1.0 - gp * 0.25).abs()),
        1e-14,
    );
    rep.note(&format!(
        "couplings of the set used here (tree-level relations exact with the measured M_W and M_Z): g = {:.6}, \
         g' = {:.6}, e = {:.6}, sin^2(theta_W) = 1 - M_W^2/M_Z^2 = {:.5}, g^F_LL = g'/4 = {:.6}",
        gw, gp, e, sw * sw, gf_ll
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
         the rho of the superfluid (stage 5) needs an angle measured independently of M_W/M_Z, and these corrections",
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
