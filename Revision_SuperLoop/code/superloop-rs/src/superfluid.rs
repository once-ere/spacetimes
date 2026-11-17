//! Command `superfluid`: the spin-zero spinor superfluid of the spinor charges of the super-loop.
//!
//! Field content.  The spinor charges s_LL = (C (x) H) e4 of the super-loop are a Weyl spinor that
//! is an isospin doublet; with the complex-conjugate super-loop (the other chirality) they form a
//! Dirac isodoublet Psi_LL (8 components: Dirac (x) isospin, chiral basis), with the electric
//! charges Q_LL = T_LL^3 = +1/2, -1/2.  Check S-0 derives the Dirac matrices, the Lorentz and the
//! isospin generators of this field from the bi-octonion product.
//!
//! Hamiltonian (units: momentum cutoff Lambda = 1 unless stated):
//!   H = sum_q Psi_LL^dag (h(q) - mu) Psi_LL - (G/4) sum_iota Pi_iota^dag Pi_iota,
//!   h(q) = sum_j alpha_LL^j q^j + beta_LL m (x) 1_iso,   Pi_iota = Psi_LL^T Kch_iota Psi_LL,
//! with the spin-zero (Lorentz-scalar) pair channels Kch_iota = C_LL gamma_ch (x) eps2 tau_iota
//! (scalar, 0+) and Kch'_iota = C_LL (x) eps2 tau_iota (pseudoscalar, 0-), iota = 1,2,3 (an isospin
//! triplet: Theorem S1); eps2 = i sigma_2 = [[0,1],[-1,0]] on the isospin index.
//! Mean field: d_iota = -(G/2) <Pi_iota>, Ppair = sum_iota d_iota Kch_iota^dag, Nambu spinor
//! (Psi_LL(q), Psi_LL^*(-q)),
//!   H_BdG = [[h(q) - mu, Ppair], [Ppair^dag, -(h(-q) - mu)^T]],
//!   Omega = sum_iota |d_iota|^2 / G - (1/2) int d^3q/(2 pi)^3 sum_(positive eigenvalues) (|q| < Lambda).
//! The order parameter is the isospin vector dvec = (d_1, d_2, d_3) in C^3.
//! Data: sl_sf_dispersion.csv, sl_sf_gap_vs_coupling.csv, sl_sf_omega_zeta.csv,
//!       sl_sf_states.csv, sl_sf_channels.csv; report sl_superfluid_checks.txt.

use crate::checks::Checks;
use spacetimes::complex::C64;
use spacetimes::io::{write_csv, write_csv_str};
use spacetimes::linalg::{eigh, CMat};
use spacetimes::quad::gauss_legendre;
use spacetimes::roots::brent;
use std::f64::consts::PI;

fn c(re: f64, im: f64) -> C64 {
    C64::new(re, im)
}

/// small dense complex matrix helpers on CMat
fn kron(a: &CMat, b: &CMat) -> CMat {
    let n = a.n * b.n;
    let mut m = CMat::zeros(n);
    for i in 0..a.n {
        for j in 0..a.n {
            let x = a.get(i, j);
            if x.abs() == 0.0 {
                continue;
            }
            for k in 0..b.n {
                for l in 0..b.n {
                    m.set(i * b.n + k, j * b.n + l, x * b.get(k, l));
                }
            }
        }
    }
    m
}

fn from_rows(rows: &[&[C64]]) -> CMat {
    let n = rows.len();
    let mut m = CMat::zeros(n);
    for i in 0..n {
        for j in 0..n {
            m.set(i, j, rows[i][j]);
        }
    }
    m
}

fn scal(m: &CMat, s: C64) -> CMat {
    let mut r = m.clone();
    for x in r.a.iter_mut() {
        *x = *x * s;
    }
    r
}

fn block2(a: &CMat, b: &CMat, cc: &CMat, d: &CMat) -> CMat {
    let n = a.n;
    let mut m = CMat::zeros(2 * n);
    for i in 0..n {
        for j in 0..n {
            m.set(i, j, a.get(i, j));
            m.set(i, j + n, b.get(i, j));
            m.set(i + n, j, cc.get(i, j));
            m.set(i + n, j + n, d.get(i, j));
        }
    }
    m
}

fn close(a: &CMat, b: &CMat, tol: f64) -> bool {
    a.sub(b).fro() < tol
}

pub struct Dirac {
    pub sig: [CMat; 4], // 1, sigma_1, sigma_2, sigma_3 (Pauli matrices)
    pub g: [CMat; 4],   // gamma_LL^0..gamma_LL^3 (chiral basis)
    pub g5: CMat,       // gamma_ch = i gamma_LL^0 gamma_LL^1 gamma_LL^2 gamma_LL^3
    pub cc: CMat,       // charge conjugation C_LL = i gamma_LL^2 gamma_LL^0
    pub eps: CMat,      // eps2 = i sigma_2
}

impl Dirac {
    pub fn new() -> Dirac {
        let z = C64::ZERO;
        let o = C64::ONE;
        let i = C64::I;
        let s0 = from_rows(&[&[o, z], &[z, o]]);
        let s1 = from_rows(&[&[z, o], &[o, z]]);
        let s2 = from_rows(&[&[z, -i], &[i, z]]);
        let s3 = from_rows(&[&[o, z], &[z, -o]]);
        let zero2 = CMat::zeros(2);
        let g0 = block2(&zero2, &s0, &s0, &zero2);
        let gk = |s: &CMat| block2(&zero2, s, &scal(s, c(-1.0, 0.0)), &zero2);
        let g1 = gk(&s1);
        let g2 = gk(&s2);
        let g3 = gk(&s3);
        let g5 = block2(&scal(&s0, c(-1.0, 0.0)), &zero2, &zero2, &s0);
        let cc = scal(&g2.mul(&g0), i);
        let eps = scal(&s2, i);
        Dirac { sig: [s0, s1, s2, s3], g: [g0, g1, g2, g3], g5, cc, eps }
    }
}

/// The pair matrices on Dirac (x) isospin (8x8): scalar Kch_iota = C_LL gamma_ch (x) eps2 tau_iota, pseudoscalar Kch'_iota = C_LL (x) eps2 tau_iota.
pub fn channels(d: &Dirac) -> (Vec<CMat>, Vec<CMat>) {
    let cg5 = d.cc.mul(&d.g5);
    let mut ks = Vec::new();
    let mut kp = Vec::new();
    for k in 1..4 {
        let s = d.eps.mul(&d.sig[k]);
        ks.push(kron(&cg5, &s));
        kp.push(kron(&d.cc, &s));
    }
    (ks, kp)
}

/// h(q) - mu on Dirac (x) isospin
fn hdirac(d: &Dirac, q: &[f64; 3], m: f64, mu: f64) -> CMat {
    let mut h = scal(&d.g[0], c(m, 0.0));
    for k in 0..3 {
        // alpha_LL^j = gamma_LL^0 gamma_LL^j
        h = h.add(&scal(&d.g[0].mul(&d.g[k + 1]), c(q[k], 0.0)));
    }
    h = h.sub(&scal(&CMat::identity(4), c(mu, 0.0)));
    kron(&h, &CMat::identity(2))
}

/// BdG matrix (16x16) for the pairing block Ppair (8x8)
fn bdg(d: &Dirac, q: &[f64; 3], m: f64, mu: f64, p: &CMat) -> CMat {
    let h = hdirac(d, q, m, mu);
    let mq = [-q[0], -q[1], -q[2]];
    let hm = hdirac(d, &mq, m, mu).transpose();
    block2(&h, p, &p.dagger(), &scal(&hm, c(-1.0, 0.0)))
}

fn pairing(ks: &[CMat], dvec: &[C64; 3]) -> CMat {
    let mut p = CMat::zeros(8);
    for k in 0..3 {
        p = p.add(&scal(&ks[k].dagger(), dvec[k]));
    }
    p
}

/// |dvec|^2 and zeta = |cross(dvec, dvec^*)| / |dvec|^2
fn dnorm(dv: &[C64; 3]) -> (f64, f64) {
    let n2: f64 = dv.iter().map(|z| z.norm_sqr()).sum();
    // cross(dvec, dvec^*)  (a purely imaginary vector)
    let dc = [dv[0].conj(), dv[1].conj(), dv[2].conj()];
    let w = [dv[1] * dc[2] - dv[2] * dc[1], dv[2] * dc[0] - dv[0] * dc[2], dv[0] * dc[1] - dv[1] * dc[0]];
    let wn = (w.iter().map(|z| z.norm_sqr()).sum::<f64>()).sqrt();
    (n2, if n2 > 0.0 { wn / n2 } else { 0.0 })
}

/// scalar (0+) channel spectrum: E^(+-) = sqrt((eps +- mu)^2 + Delta^2), eps = sqrt(q^2 + m^2)
fn e_scalar(qn: f64, m: f64, mu: f64, gap2: f64) -> (f64, f64) {
    let eps = (qn * qn + m * m).sqrt();
    (((eps + mu).powi(2) + gap2).sqrt(), ((eps - mu).powi(2) + gap2).sqrt())
}

/// pseudoscalar (0-) channel spectrum:
/// Etilde^(+-)^2 = q^2 + m^2 + mu^2 + Delta^2 +- 2 sqrt(mu^2 (q^2 + m^2) + m^2 Delta^2)
fn e_pseudo(qn: f64, m: f64, mu: f64, gap2: f64) -> (f64, f64) {
    let a = qn * qn + m * m + mu * mu + gap2;
    let b = 2.0 * (mu * mu * (qn * qn + m * m) + m * m * gap2).sqrt();
    ((a + b).sqrt(), (a - b).max(0.0).sqrt())
}

/// radial quadrature on [0, Lambda] with breaks around the Fermi momentum kF
struct Radial {
    x: Vec<f64>,
    w: Vec<f64>,
}

impl Radial {
    fn new(lam: f64, kf: f64, n: usize) -> Radial {
        let (gx, gw) = gauss_legendre(n);
        let mut x = Vec::new();
        let mut w = Vec::new();
        let mut breaks = vec![0.0];
        if kf > 0.0 && kf < lam {
            for f in [0.5, 0.9, 0.99, 1.0, 1.01, 1.1, 1.5] {
                let b = kf * f;
                if b > 0.0 && b < lam {
                    breaks.push(b);
                }
            }
        }
        breaks.push(lam);
        breaks.sort_by(|a, b| a.partial_cmp(b).unwrap());
        breaks.dedup_by(|a, b| (*a - *b).abs() < 1e-14);
        for s in breaks.windows(2) {
            let (a, b) = (s[0], s[1]);
            for k in 0..gx.len() {
                x.push(0.5 * (b - a) * gx[k] + 0.5 * (a + b));
                w.push(0.5 * (b - a) * gw[k]);
            }
        }
        Radial { x, w }
    }
    /// int d^3q/(2 pi)^3 f(|q|)
    fn int3<F: Fn(f64) -> f64>(&self, f: F) -> f64 {
        let mut s = 0.0;
        for (q, w) in self.x.iter().zip(self.w.iter()) {
            s += w * q * q * f(*q);
        }
        s / (2.0 * PI * PI)
    }
}

/// Omega for a state with the squared gaps Delta_+^2 = |dvec|^2 (1 + zeta), Delta_-^2 = |dvec|^2 (1 - zeta)
/// in the scalar channel (each energy twice): Omega = |dvec|^2/G - int [E^+(Delta_+) + E^-(Delta_+) + E^+(Delta_-) + E^-(Delta_-)]
/// (measured from the normal state)
fn omega_scalar(rad: &Radial, g: f64, m: f64, mu: f64, d2: f64, zeta: f64) -> f64 {
    let g1 = d2 * (1.0 + zeta);
    let g2 = d2 * (1.0 - zeta);
    let i = rad.int3(|q| {
        let (a1, b1) = e_scalar(q, m, mu, g1);
        let (a2, b2) = e_scalar(q, m, mu, g2);
        let (a0, b0) = e_scalar(q, m, mu, 0.0);
        (a1 + b1 + a2 + b2) - 2.0 * (a0 + b0)
    });
    d2 / g - i
}

fn omega_pseudo(rad: &Radial, g: f64, m: f64, mu: f64, d2: f64) -> f64 {
    let i = rad.int3(|q| {
        let (a1, b1) = e_pseudo(q, m, mu, d2);
        let (a0, b0) = e_pseudo(q, m, mu, 0.0);
        2.0 * ((a1 + b1) - (a0 + b0))
    });
    d2 / g - i
}

/// the gap of the unitary scalar state: solves 1/G = int (1/E^+ + 1/E^-)
fn gap_scalar(rad: &Radial, g: f64, m: f64, mu: f64) -> Option<f64> {
    let f = |dl: f64| -> f64 {
        1.0 / g - rad.int3(|q| {
            let (a, b) = e_scalar(q, m, mu, dl * dl);
            1.0 / a + 1.0 / b
        })
    };
    if f(1e-9) > 0.0 {
        return None;
    }
    brent(f, 1e-9, 20.0, 1e-13, 300).ok()
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_SuperLoop, command `superfluid`: the spin-zero spinor superfluid of the super-loop's spinor charges");
    let d = Dirac::new();
    let tol = 1e-12;

    // ---------------- S-0: the field content from the bi-octonions ----------------
    // quaternion -> 2x2 matrix M_LL: 1 -> unit matrix, e_j -> -i sigma_j
    let mq = |k: usize| -> CMat { if k == 0 { d.sig[0].clone() } else { scal(&d.sig[k], c(0.0, -1.0)) } };
    // isomorphism check: the quaternion products are reproduced
    let mut iso_ok = close(&mq(1).mul(&mq(2)), &mq(3), tol) && close(&mq(2).mul(&mq(3)), &mq(1), tol) && close(&mq(1).mul(&mq(1)), &scal(&mq(0), c(-1.0, 0.0)), tol);
    // Lorentz derivation on s_LL: bS -> -bS uL; isospin: bS -> tI bS.  psi'_LL = eps2^-1 M_LL(bS)^T:
    // column r of psi'_LL is eps2^-1 (row r of M_LL(bS))^T.  The 4x4 matrices act on the four entries of psi'_LL
    // (position 2 r + s for column r and row s, r, s = 0, 1).
    let eps_inv = d.eps.dagger(); // eps2 is real orthogonal: eps2^-1 = eps2^T = eps2^dag
    let to_psi = |bm: &CMat| -> Vec<C64> {
        let mut v = vec![C64::ZERO; 4];
        for a in 0..2 {
            for al in 0..2 {
                let mut s = C64::ZERO;
                for be in 0..2 {
                    s = s + eps_inv.get(al, be) * bm.get(a, be);
                }
                v[2 * a + al] = s;
            }
        }
        v
    };
    let from_psi = |v: &[C64]| -> CMat {
        // invert: (row r of M_LL(bS))^T = eps2 (column r of psi'_LL)
        let mut bm = CMat::zeros(2);
        for a in 0..2 {
            for be in 0..2 {
                let mut s = C64::ZERO;
                for al in 0..2 {
                    s = s + d.eps.get(be, al) * v[2 * a + al];
                }
                bm.set(a, be, s);
            }
        }
        bm
    };
    let op4 = |f: &dyn Fn(&CMat) -> CMat| -> CMat {
        let mut m = CMat::zeros(4);
        for col in 0..4 {
            let mut v = vec![C64::ZERO; 4];
            v[col] = C64::ONE;
            let img = to_psi(&f(&from_psi(&v)));
            for row in 0..4 {
                m.set(row, col, img[row]);
            }
        }
        m
    };
    // generators: D_LL(J_LL^j): bS -> -bS e_j/2 ; D_LL(K_LL^j): bS -> -bS (i e_j)/2 ; Dv_LL(e_iota/2): bS -> (e_iota/2) bS
    let dj: Vec<CMat> = (1..4).map(|k| op4(&|b: &CMat| scal(&b.mul(&mq(k)), c(-0.5, 0.0)))).collect();
    let dk: Vec<CMat> = (1..4).map(|k| op4(&|b: &CMat| scal(&b.mul(&mq(k)), c(0.0, -0.5)))).collect();
    let dv: Vec<CMat> = (1..4).map(|k| op4(&|b: &CMat| scal(&mq(k).mul(b), c(0.5, 0.0)))).collect();
    // expected on psi'_LL = isospin (x) Weyl:  D_LL(J_LL^j) = 1 (x) (-i sigma_j/2), D_LL(K_LL^j) = 1 (x) (sigma_j/2),
    //           Dv_LL(e_iota/2) = (-i tau_iota/2) (x) 1, i.e. T_LL^iota = i Dv_LL(e_iota/2) = (tau_iota/2) (x) 1
    for k in 0..3 {
        iso_ok &= close(&dj[k], &kron(&d.sig[0], &scal(&d.sig[k + 1], c(0.0, -0.5))), tol);
        iso_ok &= close(&dk[k], &kron(&d.sig[0], &scal(&d.sig[k + 1], c(0.5, 0.0))), tol);
        iso_ok &= close(&dv[k], &kron(&scal(&d.sig[k + 1], c(0.0, -0.5)), &d.sig[0]), tol);
    }
    // Dirac intertwining: chiral blocks upper = conjugate super-loop (D_LL(K_LL^j) = -sigma_j/2), lower = s_LL (+sigma_j/2);
    // the vector action V_LL from the loop's conjugations on the Hermitian bi-quaternions:
    // rotation J_LL^j: x -> [e_j/2, x]; boost K_LL^j: x -> (i/2)(e_j x + x e_j); x = x_LL^0 + i sum_j x_LL^j e_j.
    let wrot = |k: usize| -> [[f64; 4]; 4] {
        let mut w = [[0.0; 4]; 4];
        // d x_LL^i / d theta = eps3_(k j i) x_LL^j  (rotation about the axis k)
        let e = |a: usize, b: usize, cc: usize| -> f64 {
            let p = [(1, 2, 3), (2, 3, 1), (3, 1, 2)];
            let n = [(2, 1, 3), (3, 2, 1), (1, 3, 2)];
            if p.contains(&(a, b, cc)) { 1.0 } else if n.contains(&(a, b, cc)) { -1.0 } else { 0.0 }
        };
        for i in 1..4 {
            for j in 1..4 {
                w[i][j] = e(k, j, i);
            }
        }
        w
    };
    let wboost = |k: usize| -> [[f64; 4]; 4] {
        let mut w = [[0.0; 4]; 4];
        w[0][k] = 1.0;
        w[k][0] = 1.0;
        w
    };
    let zero2 = CMat::zeros(2);
    let mut int_ok = true;
    for k in 1..4 {
        let djl = scal(&d.sig[k], c(0.0, -0.5));
        let dkl_lower = scal(&d.sig[k], c(0.5, 0.0));
        let dkl_upper = scal(&d.sig[k], c(-0.5, 0.0));
        let jdir = block2(&djl, &zero2, &zero2, &djl);
        let kdir = block2(&dkl_upper, &zero2, &zero2, &dkl_lower);
        for (dd, w) in [(jdir, wrot(k)), (kdir, wboost(k))] {
            for mu in 0..4 {
                let lhs = d.g[mu].mul(&dd).sub(&dd.mul(&d.g[mu]));
                let mut rhs = CMat::zeros(4);
                for nu in 0..4 {
                    rhs = rhs.add(&scal(&d.g[nu], c(w[mu][nu], 0.0)));
                }
                int_ok &= close(&lhs, &rhs, tol);
            }
        }
    }
    // the vector action from the conjugations of B_LL reproduces wrot/wboost
    let mut vec_ok = true;
    {
        use crate::bioct::*;
        let gi = |re: i128, im: i128| GInt { re, im };
        let herm = |x: &[i128; 4]| Bi::<GInt>::from_quats(&[gi(x[0], 0), gi(0, x[1]), gi(0, x[2]), gi(0, x[3])], &qzero());
        let coords = |y: &Bi<GInt>| -> [i128; 4] { [y.c[0].re, y.c[1].im, y.c[2].im, y.c[3].im] };
        for k in 1..4 {
            for j in 0..4 {
                let mut x = [0i128; 4];
                x[j] = 1;
                let xb = herm(&x);
                let ek = Bi::<GInt>::unit(k);
                let rot = ek.mul(&xb).sub(&xb.mul(&ek)); // 2 x the rotation generator
                let iek = ek.scale(GInt::iu());
                let bst = iek.mul(&xb).add(&xb.mul(&iek)); // 2 x the boost generator
                let (r, b) = (coords(&rot), coords(&bst));
                let (wr, wb) = (wrot(k), wboost(k));
                for i in 0..4 {
                    vec_ok &= (r[i] as f64 - 2.0 * wr[i][j]).abs() < 1e-14 && (b[i] as f64 - 2.0 * wb[i][j]).abs() < 1e-14;
                }
            }
        }
    }
    ck.add("S-0", "field content from the bi-octonions: with the isomorphism C (x) H = M_2(C), e_j -> -i sigma_j (j = 1,2,3), the spinor charge bS, as the 2x2 matrix M_LL(bS), gives the Weyl isodoublet psi'_LL = eps2^-1 M_LL(bS)^T, whose column 1 (isospin up) and column 2 (isospin down) are Weyl spinors; on it the Lorentz derivations are D_LL(J_LL^j) = -i sigma_j/2, D_LL(K_LL^j) = +sigma_j/2 (on each column), and the isospin generators are T_LL^iota = tau_iota/2 (mixing the columns); the chiral Dirac matrices gamma_LL^p (p = 0..3) intertwine these spinor generators with the vector action V_LL of the loop's conjugations on R^(1,3)_LL (all 24 relations [gamma_LL^p, D] = sum_p' V_LL^p_p' gamma_LL^p'), the spinor charges being the gamma_ch = +1 block: Psi_LL = Dirac (x) isodoublet", iso_ok && int_ok && vec_ok, String::new());

    // ---------------- S-1: the spin-zero channels ----------------
    let (ks, kp) = channels(&d);
    // Lorentz generators on Dirac (x) isospin, and the isospin generators Dv_LL(e_iota/2)
    let mut lor: Vec<CMat> = Vec::new();
    for k in 1..4 {
        let djl = scal(&d.sig[k], c(0.0, -0.5));
        lor.push(kron(&block2(&djl, &zero2, &zero2, &djl), &CMat::identity(2)));
        lor.push(kron(&block2(&scal(&d.sig[k], c(-0.5, 0.0)), &zero2, &zero2, &scal(&d.sig[k], c(0.5, 0.0))), &CMat::identity(2)));
    }
    let isos: Vec<CMat> = (1..4).map(|k| kron(&CMat::identity(4), &scal(&d.sig[k], c(0.0, -0.5)))).collect();
    let invariant = |kk: &CMat, gens: &[CMat]| gens.iter().all(|x| x.transpose().mul(kk).add(&kk.mul(x)).fro() < tol);
    let antisym = |kk: &CMat| kk.add(&kk.transpose()).fro() < tol;
    let unitary = |kk: &CMat| close(&kk.mul(&kk.dagger()), &CMat::identity(8), tol);
    let mut ch_ok = true;
    for kk in ks.iter().chain(kp.iter()) {
        ch_ok &= antisym(kk) && invariant(kk, &lor) && unitary(kk);
    }
    // the space of Lorentz-invariant antisymmetric pair matrices: solve X^T K + K X = 0, K^T = -K (complex 8x8)
    // dimension by real linear algebra (rank of the Gram matrix of the vectorized constraints)
    let mut basis: Vec<Vec<f64>> = Vec::new();
    for i in 0..8 {
        for j in (i + 1)..8 {
            for ph in [c(1.0, 0.0), c(0.0, 1.0)] {
                let mut kk = CMat::zeros(8);
                kk.set(i, j, ph);
                kk.set(j, i, -ph);
                // constraint vector: concatenation of X^T K + K X over generators
                let mut v = Vec::new();
                for x in &lor {
                    let r = x.transpose().mul(&kk).add(&kk.mul(x));
                    for z in &r.a {
                        v.push(z.re);
                        v.push(z.im);
                    }
                }
                basis.push(v);
            }
        }
    }
    // nullity of the linear map K -> constraints on the real 56-dimensional space of antisymmetric K
    let rankf = |rows: &Vec<Vec<f64>>| -> usize {
        // rank of the matrix whose COLUMNS are the vectors: Gram matrix eigenvalues
        let n = rows.len();
        let mut gm = CMat::zeros(n);
        for i in 0..n {
            for j in 0..n {
                let s: f64 = rows[i].iter().zip(rows[j].iter()).map(|(a, b)| a * b).sum();
                gm.set(i, j, c(s, 0.0));
            }
        }
        let (ev, _) = eigh(&gm);
        let mx = ev.iter().cloned().fold(0.0f64, f64::max);
        ev.iter().filter(|&&x| x > 1e-10 * mx).count()
    };
    let nullity = basis.len() - rankf(&basis);
    // isospin content: the isospin Casimir on the channel space is 2 (triplet) for every Kch_iota
    let mut trip = true;
    for kk in ks.iter().chain(kp.iter()) {
        // action of isospin on a pair matrix: K -> X^T K + K X ; Casimir -sum (ad)^2 K = 2 K
        let mut acc = CMat::zeros(8);
        for x in &isos {
            let a1 = x.transpose().mul(kk).add(&kk.mul(x));
            let a2 = x.transpose().mul(&a1).add(&a1.mul(x));
            acc = acc.sub(&a2);
        }
        trip &= close(&acc, &scal(kk, c(2.0, 0.0)), 1e-11);
    }
    ck.add("S-1", "Theorem S1 (spin-zero channels): the antisymmetric, Lorentz-invariant pair matrices on Dirac (x) isospin form a complex space of dimension 6, spanned by Kch_iota = C_LL gamma_ch (x) eps2 tau_iota (scalar 0+) and Kch'_iota = C_LL (x) eps2 tau_iota (pseudoscalar 0-), iota = 1,2,3; each is unitary and an isospin TRIPLET (Casimir 2): a spin-zero s-wave pair of spinor charges carries isospin 1 and electric charge Q_LL = +1, 0, -1", ch_ok && nullity == 12 && trip, format!("real nullity {} (= 6 complex dimensions)", nullity));

    // charges of the channels: Q_LL = T_LL^3 summed over the pair; Kch_3 pairs (up, down): 0; Kch_1 -+ i Kch_2 pair (up,up)/(down,down)
    let t3 = kron(&CMat::identity(4), &scal(&d.sig[3], c(0.5, 0.0)));
    let qcharge = |kk: &CMat| -> Option<f64> {
        // the pair operator Psi_LL^T K Psi_LL annihilates the charge q if T3^T K + K T3 = q K
        let a = t3.transpose().mul(kk).add(&kk.mul(&t3));
        for q in [-1.0, 0.0, 1.0] {
            if close(&a, &scal(kk, c(q, 0.0)), 1e-12) {
                return Some(q);
            }
        }
        None
    };
    let kplus = ks[0].sub(&scal(&ks[1], c(0.0, 1.0)));
    let kminus = ks[0].add(&scal(&ks[1], c(0.0, 1.0)));
    let qs = (qcharge(&ks[2]), qcharge(&kplus), qcharge(&kminus));
    ck.add("S-2", "electric charge of the spin-zero channels (charge of the pair operator Psi_LL^T Kch Psi_LL, which annihilates two spinor charges): Kch_3 pairs up with down, charge 0; Kch_1 - i Kch_2 and Kch_1 + i Kch_2 pair up-up and down-down, charges +1 and -1", qs == (Some(0.0), Some(1.0), Some(-1.0)) || qs == (Some(0.0), Some(-1.0), Some(1.0)), format!("Q_LL(Kch_3) = {:?}, Q_LL(Kch_1 - i Kch_2) = {:?}, Q_LL(Kch_1 + i Kch_2) = {:?}", qs.0, qs.1, qs.2));

    // ---------------- S-3: BdG spectra ----------------
    let mut spec_ok = true;
    let mut worst = 0.0f64;
    let mut rng = crate::bioct::Rng::new(77);
    for trial in 0..60 {
        let q = [rng.unif() - 0.5, rng.unif() - 0.5, rng.unif() - 0.5];
        let qn = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
        let m = if trial % 3 == 0 { 0.0 } else { 0.3 * rng.unif() };
        let mu = 0.8 * rng.unif();
        let dv = [c(rng.unif() - 0.5, rng.unif() - 0.5), c(rng.unif() - 0.5, rng.unif() - 0.5), c(rng.unif() - 0.5, rng.unif() - 0.5)];
        let (n2, z) = dnorm(&dv);
        let (ev, _) = eigh(&bdg(&d, &q, m, mu, &pairing(&ks, &dv)));
        let mut pos: Vec<f64> = ev.iter().cloned().filter(|&x| x > 0.0).collect();
        pos.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut expect = Vec::new();
        for gg in [n2 * (1.0 + z), n2 * (1.0 - z)] {
            let (a, b) = e_scalar(qn, m, mu, gg);
            expect.extend([a, a, b, b]);
        }
        expect.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if pos.len() != 8 {
            spec_ok = false;
            continue;
        }
        for i in 0..8 {
            worst = worst.max((pos[i] - expect[i]).abs());
        }
        // pseudoscalar, unitary d (real up to phase)
        let dr = [c(rng.unif() - 0.5, 0.0), c(rng.unif() - 0.5, 0.0), c(rng.unif() - 0.5, 0.0)];
        let ph = c(0.6, 0.8);
        let du = [dr[0] * ph, dr[1] * ph, dr[2] * ph];
        let (nn, _) = dnorm(&du);
        let (ev2, _) = eigh(&bdg(&d, &q, m, mu, &pairing(&kp, &du)));
        let mut pos2: Vec<f64> = ev2.iter().cloned().filter(|&x| x > 1e-13).collect();
        pos2.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (a, b) = e_pseudo(qn, m, mu, nn);
        let mut ex2 = vec![a, a, a, a, b, b, b, b];
        ex2.sort_by(|a, b| a.partial_cmp(b).unwrap());
        if pos2.len() == 8 {
            for i in 0..8 {
                worst = worst.max((pos2[i] - ex2[i]).abs());
            }
        } else if b > 1e-9 {
            spec_ok = false;
        }
    }
    spec_ok &= worst < 1e-10;
    ck.add("S-3", "Theorem S2 (spectra; 60 random momenta, masses, chemical potentials and order parameters dvec in C^3, double precision): in the scalar channel the 8 positive BdG energies are E^+(Delta_+), E^-(Delta_+), E^+(Delta_-), E^-(Delta_-) (each twice) with E^(+-)(Delta) = [(eps +- mu)^2 + Delta^2]^(1/2), eps = (q^2 + m^2)^(1/2), and the two gaps Delta_(+-)^2 = |dvec|^2 (1 +- zeta), zeta = |cross(dvec, dvec^*)| / |dvec|^2 (unitary states: zeta = 0; non-unitary: two gaps); in the pseudoscalar channel (unitary dvec) Etilde^(+-) = [q^2 + m^2 + mu^2 + Delta^2 +- 2 (mu^2 (q^2 + m^2) + m^2 Delta^2)^(1/2)]^(1/2), each four times", spec_ok, format!("largest deviation {:.2e}", worst));

    // ---------------- S-4: gap equation and critical coupling ----------------
    let rad0 = Radial::new(1.0, 0.0, 40);
    let i0 = rad0.int3(|q| 2.0 / q.max(1e-300));
    let gcrit_num = 1.0 / i0;
    let gcrit = 2.0 * PI * PI;
    let m_ex = 0.2;
    let im = rad0.int3(|q| 2.0 / (q * q + m_ex * m_ex).sqrt());
    let gcrit_m_closed = 2.0 * PI * PI / ((1.0 + m_ex * m_ex).sqrt() - m_ex * m_ex * ((1.0 + (1.0 + m_ex * m_ex).sqrt()) / m_ex).ln());
    ck.add("S-4", "critical coupling of the scalar channel at mu = 0 (gap equation 1/G = int d^3q/(2 pi)^3 (1/E^+ + 1/E^-), momentum cutoff Lambda): G_crit = 2 pi^2/Lambda^2 = 19.7392 for m = 0, and G_crit = 2 pi^2/[Lambda (Lambda^2 + m^2)^(1/2) - m^2 ln((Lambda + (Lambda^2 + m^2)^(1/2))/m)] for m > 0 (m = 0.2: quadrature against the closed form)", (gcrit_num - gcrit).abs() < 1e-10 * gcrit && (1.0 / im - gcrit_m_closed).abs() < 1e-10 * gcrit_m_closed, format!("G_crit at m = 0: {:.6}, at m = 0.2: {:.6}", gcrit_num, 1.0 / im));

    // gap versus coupling at mu = 0.5, m = 0.2 (unitary scalar state) and BCS behaviour
    let (mu0, mm): (f64, f64) = (0.5, 0.2);
    let kf = (mu0 * mu0 - mm * mm).sqrt();
    let rad = Radial::new(1.0, kf, 40);
    let mut rows_g: Vec<Vec<f64>> = Vec::new();
    let mut gaps = Vec::new();
    for i in 0..40 {
        let g = 6.0 + 0.6 * i as f64;
        let dl = gap_scalar(&rad, g, mm, mu0).unwrap_or(0.0);
        let om = omega_scalar(&rad, g, mm, mu0, dl * dl, 0.0);
        rows_g.push(vec![g, g / gcrit, dl, om]);
        gaps.push((g, dl));
    }
    let mono = gaps.windows(2).all(|w| w[1].1 > w[0].1);
    // weak coupling: the slope of ln Delta versus 1/G at the two weakest couplings against the BCS value -1/(2 nuF)
    // nuF = density of states per unit volume at the Fermi surface for ONE species and spin state: kF mu / (2 pi^2)
    let n0 = kf * mu0 / (2.0 * PI * PI);
    let (g1, d1) = gaps[0];
    let (g2, d2) = gaps[1];
    let slope = (d2.ln() - d1.ln()) / (1.0 / g2 - 1.0 / g1);
    let bcs_slope = -1.0 / (2.0 * n0);
    ck.add("S-5", "the gap of the unitary scalar state grows monotonically with the coupling (mu = 0.5, m = 0.2, 40 couplings), and at weak coupling ln Delta is linear in 1/G with the BCS slope -1/(2 nuF) = -pi^2/(kF mu), nuF = kF mu/(2 pi^2) the density of states of one species at the Fermi surface (ratio of the slopes within 5 %): a Fermi surface pairs for every G > 0", mono && (slope / bcs_slope - 1.0).abs() < 0.05, format!("slope {:.4}, BCS value {:.4}, ratio {:.4}", slope, bcs_slope, slope / bcs_slope));

    // ---------------- S-6: unitary vs non-unitary states ----------------
    let gsel = 12.0;
    let dl = gap_scalar(&rad, gsel, mm, mu0).unwrap();
    let mut rows_z: Vec<Vec<f64>> = Vec::new();
    let mut omins = Vec::new();
    for zi in 0..=10 {
        let zeta = zi as f64 / 10.0;
        // minimize Omega over |dvec| at fixed zeta (golden section)
        let f = |x: f64| omega_scalar(&rad, gsel, mm, mu0, x * x, zeta);
        let (mut a, mut b) = (0.0, 3.0 * dl);
        let gr = 0.5 * (5f64.sqrt() - 1.0);
        let mut x1 = b - gr * (b - a);
        let mut x2 = a + gr * (b - a);
        let (mut f1, mut f2) = (f(x1), f(x2));
        for _ in 0..120 {
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
        omins.push(f(xm));
        rows_z.push(vec![zeta, xm, f(xm), omega_scalar(&rad, gsel, mm, mu0, dl * dl, zeta)]);
    }
    let increasing = omins.windows(2).all(|w| w[1] > w[0]);
    ck.add("S-6", "Theorem S3 (the unitary state wins): at G = 12, mu = 0.5, m = 0.2 the minimum over |dvec| of Omega increases strictly with zeta from the unitary (polar) state zeta = 0 to the fully charged state zeta = 1 (concavity of E^(+-) in Delta^2): the ground state is a unitary isospin-triplet condensate", increasing, format!("minimum of Omega at zeta = 0: {:.6e}, at zeta = 1: {:.6e}", omins[0], omins[10]));

    // ---------------- S-10 (placed here): scalar versus pseudoscalar channel ----------------
    let golden = |f: &dyn Fn(f64) -> f64, hi: f64| -> (f64, f64) {
        let (mut a, mut b) = (0.0, hi);
        let gr = 0.5 * (5f64.sqrt() - 1.0);
        let mut x1 = b - gr * (b - a);
        let mut x2 = a + gr * (b - a);
        let (mut f1, mut f2) = (f(x1), f(x2));
        for _ in 0..150 {
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
        let x = 0.5 * (a + b);
        (x, f(x))
    };
    let (_, om_s) = golden(&|x: f64| omega_scalar(&rad, gsel, mm, mu0, x * x, 0.0), 3.0 * dl);
    let (_, om_p) = golden(&|x: f64| omega_pseudo(&rad, gsel, mm, mu0, x * x), 3.0 * dl);
    let rad_m0 = Radial::new(1.0, mu0, 40);
    let dl0 = gap_scalar(&rad_m0, gsel, 0.0, mu0).unwrap();
    let (_, om_s0) = golden(&|x: f64| omega_scalar(&rad_m0, gsel, 0.0, mu0, x * x, 0.0), 3.0 * dl0);
    let (_, om_p0) = golden(&|x: f64| omega_pseudo(&rad_m0, gsel, 0.0, mu0, x * x), 3.0 * dl0);
    ck.add("S-10", "which spin-zero channel condenses: at the same coupling (G = 12, mu = 0.5) the minimum of Omega in the scalar (0+) channel lies below that of the pseudoscalar (0-) channel for m = 0.2, and the two minima coincide for m = 0 (chiral symmetry maps one channel onto the other): for massive spinor charges the scalar isospin-triplet condensate is the ground state", om_s < om_p && (om_s0 - om_p0).abs() < 1e-9 * om_s0.abs(), format!("m = 0.2: {:.6e} (0+) and {:.6e} (0-); m = 0: {:.6e} and {:.6e}", om_s, om_p, om_s0, om_p0));

    // ---------------- S-7: stabilizers and Goldstone modes ----------------
    // group SU(2)_v x U(1)_F_LL acting on dvec: rotations dvec -> cross(e_k, dvec) (real 3x3 on C^3), F_LL: dvec -> 2 i dvec
    let rank_stab = |dv: &[C64; 3]| -> (usize, usize) {
        let mut vecs: Vec<Vec<f64>> = Vec::new();
        for k in 0..3 {
            let mut e = [0.0; 3];
            e[k] = 1.0;
            let cr = [
                c(e[1], 0.0) * dv[2] - c(e[2], 0.0) * dv[1],
                c(e[2], 0.0) * dv[0] - c(e[0], 0.0) * dv[2],
                c(e[0], 0.0) * dv[1] - c(e[1], 0.0) * dv[0],
            ];
            vecs.push(cr.iter().flat_map(|z| [z.re, z.im]).collect());
        }
        vecs.push(dv.iter().flat_map(|z| { let w = *z * c(0.0, 2.0); [w.re, w.im] }).collect());
        let r = rankf(&vecs);
        (r, 4 - r)
    };
    let polar = [c(0.0, 0.0), c(0.0, 0.0), c(1.0, 0.0)];
    let ferro = [c(1.0, 0.0), c(0.0, 1.0), c(0.0, 0.0)];
    let (rp, sp) = rank_stab(&polar);
    let (rf, sf) = rank_stab(&ferro);
    let qpolar = qcharge(&pairing(&ks, &polar).dagger());
    let qferro = qcharge(&pairing(&ks, &[c(1.0, 0.0), c(0.0, -1.0), c(0.0, 0.0)]).dagger());
    ck.add("S-7", "symmetry breaking under SU(2)_v x U(1)_F_LL (isospin and fermion number, 4 generators): the polar state dvec = (0,0,1) keeps the rotations about the 3-axis (stabilizer of dimension 1; the electric charge Q_LL = T_LL^3 stays unbroken: a neutral superfluid), the charged state dvec = (1, i, 0) keeps one combination of T_LL^3 and F_LL (stabilizer 1; the condensate carries charge 1); both break 3 generators", rp == 3 && sp == 1 && rf == 3 && sf == 1 && qpolar == Some(0.0) && qferro.is_some(), format!("polar: broken {}, stabilizer {}, Q_LL = {:?}; charged: broken {}, stabilizer {}, Q_LL = {:?}", rp, sp, qpolar, rf, sf, qferro));

    // ---------------- S-8: the octonionic two-species channel ----------------
    // two isodoublets bS and bS'; product (bS e4)(bS' e4) = -bS'bar bS = scalar part + part in l_LL
    // the scalar part -N_LL(bS', bS) pairs spin 0, isospin 0; the imaginary part lies in l_LL (spin 1, isospin 0)
    {
        use crate::bioct::*;
        // coefficient matrices of the bilinear forms (bS'bar bS)_p = bS'^T M_p bS, p = 0..3, on the complex coefficients
        let mut mats: Vec<CMat> = Vec::new();
        for a in 0..4 {
            let mut m = CMat::zeros(4);
            for i in 0..4 {
                for j in 0..4 {
                    let mut bi = qzero::<C64>();
                    bi[i] = C64::ONE;
                    let mut bj = qzero::<C64>();
                    bj[j] = C64::ONE;
                    let p = qmul(&qconj(&bj), &bi);
                    m.set(j, i, p[a]);
                }
            }
            mats.push(m);
        }
        // Lorentz and isospin generators on bS in the basis of the quaternion coefficients (4x4 complex)
        let gen_l: Vec<CMat> = (1..4)
            .flat_map(|k| {
                let mut out = Vec::new();
                for ph in [c(1.0, 0.0), c(0.0, 1.0)] {
                    let mut m = CMat::zeros(4);
                    for i in 0..4 {
                        let mut bi = qzero::<C64>();
                        bi[i] = C64::ONE;
                        let mut qk = qzero::<C64>();
                        qk[k] = ph * c(0.5, 0.0);
                        let r = qscale(&qmul(&bi, &qk), c(-1.0, 0.0));
                        for j in 0..4 {
                            m.set(j, i, r[j]);
                        }
                    }
                    out.push(m);
                }
                out
            })
            .collect();
        let gen_v: Vec<CMat> = (1..4)
            .map(|k| {
                let mut m = CMat::zeros(4);
                for i in 0..4 {
                    let mut bi = qzero::<C64>();
                    bi[i] = C64::ONE;
                    let mut tk = qzero::<C64>();
                    tk[k] = c(0.5, 0.0);
                    let r = qmul(&tk, &bi);
                    for j in 0..4 {
                        m.set(j, i, r[j]);
                    }
                }
                m
            })
            .collect();
        // scalar part M_0: invariant under both (X^T M + M X = 0); imaginary parts: invariant under isospin, rotated by Lorentz
        let inv0 = gen_l.iter().chain(gen_v.iter()).all(|x| x.transpose().mul(&mats[0]).add(&mats[0].mul(x)).fro() < 1e-12);
        let invi = (1..4).all(|a| gen_v.iter().all(|x| x.transpose().mul(&mats[a]).add(&mats[a].mul(x)).fro() < 1e-12));
        let rot = (1..4).any(|a| gen_l.iter().any(|x| x.transpose().mul(&mats[a]).add(&mats[a].mul(x)).fro() > 1e-6));
        // symmetry: M_0 symmetric (only distinct species pair in it), imaginary parts antisymmetric
        let sym0 = close(&mats[0], &mats[0].transpose(), 1e-14);
        let anti = (1..4).all(|a| close(&mats[a], &scal(&mats[a].transpose(), c(-1.0, 0.0)), 1e-14));
        ck.add("S-8", "the octonionic Cooper pair of two spinor-charge fields: (bS e4)(bS' e4) = -bS'bar bS splits into the scalar part -N_LL(bS', bS) (Lorentz scalar AND isospin singlet: spin 0, isospin 0, charge 0: the super-loop singlet) and the imaginary part, which lies in the Lorentz part l_LL (spin 1, isospin 0: the Malcev bracket); the scalar part is SYMMETRIC in the two fields, so it vanishes for one fermion field and needs two species (Theorem S4)", inv0 && invi && rot && sym0 && anti, String::new());
    }
    // two-species singlet BdG: Psi_LL of two species, pairing matrix Kch_singlet = C_LL gamma_ch (x) eps2 (x) eps2: spectrum E^(+-), each 8 times
    {
        let cg5 = d.cc.mul(&d.g5);
        // one species: the isospin-singlet scalar C_LL gamma_ch (x) eps2 is SYMMETRIC, so Psi_LL^T K Psi_LL = 0 for a fermion field
        let one = kron(&cg5, &d.eps);
        let sym_one = close(&one, &one.transpose(), 1e-14);
        // two species: Kch_singlet = C_LL gamma_ch (x) eps2 (isospin) (x) eps2 (species) is antisymmetric
        let ks0 = kron(&kron(&cg5, &d.eps), &d.eps);
        let anti_s0 = close(&ks0, &scal(&ks0.transpose(), c(-1.0, 0.0)), 1e-14);
        // BdG 32x32 for Psi_LL in Dirac (x) isospin (x) species
        let q = [0.13, -0.21, 0.07];
        let qn = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2]) as f64;
        let qn = qn.sqrt();
        let (m, mu, dl) = (0.2, 0.5, 0.3);
        let hh = |qq: &[f64; 3]| kron(&hdirac(&d, qq, m, mu), &CMat::identity(2));
        let p = scal(&ks0.dagger(), c(dl, 0.0));
        let mq3 = [-q[0], -q[1], -q[2]];
        let big = block2(&hh(&q), &p, &p.dagger(), &scal(&hh(&mq3).transpose(), c(-1.0, 0.0)));
        let (ev, _) = eigh(&big);
        let mut pos: Vec<f64> = ev.into_iter().filter(|&x| x > 0.0).collect();
        pos.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let (a, b) = e_scalar(qn, m, mu, dl * dl);
        let ok = pos.len() == 16 && pos[..8].iter().all(|x| (x - b).abs() < 1e-12) && pos[8..].iter().all(|x| (x - a).abs() < 1e-12);
        ck.add("S-9", "two-species singlet: for one field the isospin-singlet scalar C_LL gamma_ch (x) eps2 is symmetric (the pair vanishes); for two species the pair matrix Kch_singlet = C_LL gamma_ch (x) eps2 (x) eps2 (spin 0, isospin 0, species singlet: the third factor acts on the species index; antisymmetric as fermions require) gives the 16 positive BdG energies E^- (8 times) and E^+ (8 times) with E^(+-) = [(eps +- mu)^2 + Delta^2]^(1/2): a fully gapped super-loop-singlet superfluid that breaks only the fermion number and keeps isospin, electric charge and Lorentz invariance", anti_s0 && ok && sym_one, format!("E^- = {:.6}, E^+ = {:.6}", b, a));
    }

    // ---------------- data ----------------
    let _ = write_csv("sl_sf_gap_vs_coupling.csv", &[("content", "unitary scalar (0+) isospin-triplet state: gap Delta and Omega(Delta) - Omega(0) versus the coupling G (cutoff Lambda = 1, mu = 0.5, m = 0.2); G_over_Gcrit_m0 is G divided by G_crit at mu = 0, m = 0".to_string()), ("G_crit_mu0_m0", format!("{:.10}", gcrit))], &["G", "G_over_Gcrit_m0", "Delta", "Omega_minus_Omega0"], &rows_g);
    let _ = write_csv("sl_sf_omega_zeta.csv", &[("content", "minimum over |dvec| of Omega(dvec) - Omega(0) at fixed zeta = |cross(dvec, dvec^*)|/|dvec|^2 (G = 12, mu = 0.5, m = 0.2), the |dvec| of the minimum, and Omega at |dvec| = the gap of the polar state".to_string())], &["zeta", "abs_dvec_at_min", "Omega_min", "Omega_at_polar_gap"], &rows_z);
    let mut rows_d: Vec<Vec<f64>> = Vec::new();
    let dl12 = gap_scalar(&rad, gsel, mm, mu0).unwrap();
    for i in 0..=200 {
        let q = 1.0 * i as f64 / 200.0;
        let (a, b) = e_scalar(q, mm, mu0, dl12 * dl12);
        let (_a1, b1) = e_scalar(q, mm, mu0, 2.0 * dl12 * dl12);
        let (_, b0) = e_scalar(q, mm, mu0, 0.0);
        let (pa, pb) = e_pseudo(q, mm, mu0, dl12 * dl12);
        rows_d.push(vec![q, b, a, b1, b0, pb, pa]);
    }
    let _ = write_csv("sl_sf_dispersion.csv", &[("content", "quasiparticle energies versus q at G = 12, mu = 0.5, m = 0.2: polar state E^-, E^+; charged state (zeta = 1) gapped branch E^-(sqrt(2) Delta) [its other branch is the normal |eps - mu|]; normal state |eps - mu|; pseudoscalar channel Etilde^-, Etilde^+ at the same Delta".to_string()), ("Delta", format!("{:.10}", dl12))], &["q", "E_minus_polar", "E_plus_polar", "E_minus_charged_gapped", "E_normal", "Etilde_minus", "Etilde_plus"], &rows_d);
    let st = vec![
        vec!["polar (unitary)".to_string(), "(0,0,1) Delta".into(), "0".into(), "1".into(), "3".into(), "0".into(), "Delta (all branches)".into()],
        vec!["charged (zeta = 1)".to_string(), "(1,i,0) Delta/sqrt2 or (1,-i,0) Delta/sqrt2".into(), "1".into(), "1".into(), "3".into(), "+1 or -1".into(), "Delta_+ = sqrt2 Delta and Delta_- = 0".into()],
        vec!["general non-unitary".to_string(), "cross(dvec, dvec^*) != 0".into(), "0 < zeta < 1".into(), "-".into(), "-".into(), "-".into(), "Delta_(+-) = |dvec| (1 +- zeta)^(1/2)".into()],
    ];
    let _ = write_csv_str("sl_sf_states.csv", &[("content", "spin-zero isospin-triplet states of the superfluid of the spinor charges: order parameter dvec, zeta, dimension of the stabilizer in SU(2)_v x U(1)_F_LL, broken generators, charge of the condensed pairs, gaps".to_string())], &["state", "dvec", "zeta", "stabilizer_dim", "broken_generators", "pair_charge", "gaps"], &st);
    let chs = vec![
        vec!["Kch_iota = C_LL gamma_ch (x) eps2 tau_iota".to_string(), "0+".into(), "1 (triplet)".into(), "+1, 0, -1".into(), "one species".into()],
        vec!["Kch'_iota = C_LL (x) eps2 tau_iota".to_string(), "0-".into(), "1 (triplet)".into(), "+1, 0, -1".into(), "one species".into()],
        vec!["Kch_singlet = C_LL gamma_ch (x) eps2 (x) eps2".to_string(), "0+".into(), "0 (singlet)".into(), "0".into(), "two species (scalar part of the bi-octonion product)".into()],
        vec!["imaginary part of (bS e4)(bS' e4)".to_string(), "spin 1 (in l_LL)".into(), "0".into(), "0".into(), "Malcev bracket (a Lorentz tensor)".into()],
    ];
    let _ = write_csv_str("sl_sf_channels.csv", &[("content", "pair channels of the spinor charges: pair matrix, spin and parity, isospin, charge Q_LL of the pair, remark".to_string())], &["pair_matrix", "spin_parity", "isospin", "pair_charge", "remark"], &chs);
    ck.write("sl_superfluid_checks.txt")
}
