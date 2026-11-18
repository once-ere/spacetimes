//! t2_sfbs -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, items [7] to [11].
//!
//! The superfluid boson singlet state (SFBS) of the self-interacting spinor field Psi_LL of item [1]: its
//! construction from the spinor field (item [7]), and the machine checks of every statement of the sections of the
//! document of Test 2 that answer these items.  The program also writes the explicit matrices that the document
//! prints (fragments, so that no entry is typed by hand) and the data of the figures.
//!
//! Item [7], the construction of the SFBS (Section 8 of the document):
//!   SB-1  the pairs of two spinor charges at one point that are Lorentz scalars (spin zero): the antisymmetric
//!         8 x 8 pair matrices Kgen with S^T Kgen + Kgen S = 0 for the six Lorentz generators S form a space of complex
//!         dimension 6, spanned by Kch_iota (scalar) and Kch'_iota (pseudoscalar); their blocks between the two Weyl
//!         blocks vanish; the isospin-singlet candidates are symmetric and give no pair;
//!   SB-2  the quantum numbers of the pairs: spin 0 (the spin Casimir vanishes), isospin 1 (the isospin Casimir is 2),
//!         the electric charges +1, 0, -1 of K(+1) = (Kch_1 - i Kch_2)/sqrt2, K(0) = Kch_3, K(-1) = (Kch_1 + i Kch_2)/sqrt2,
//!         the fermion number 2, the parities +1 (Kch_iota) and -1 (Kch'_iota);
//!   SB-3  the super-loop form: the pair Psi^T Kch_iota Psi of the Dirac field made of the two spinor charges
//!         bSm e4 and bSp e4 is -2i times the sum of the e_iota-components of bSm conj(bSm) and bSp conj(bSp) (as
//!         bilinear forms of the anticommuting coefficients), and the super-loop singlet N_LL(b, b) vanishes;
//!   SB-4  the pairs are bosons: on one lattice site (8 fermion modes, a Fock space of 256 states)
//!         [Pi_i, Pi_j] = 0, [n_F, Pi_i] = -2 Pi_i and [Pi_i, Pi_j^dag] = 16 delta_ij - 4 delta_ij n_F + 8 i eps_ijk n_k;
//!   SB-5  the SFBS state: for one pair of momenta (q, -q) (16 modes, a Fock space of 65536 states) the coherent pair
//!         state  prod (1 - t_p B_p^dag) (1 + t_a B_a^dag) |Dirac sea>  is the ground state of the mean-field
//!         Hamiltonian: it is annihilated by all sixteen quasi-particle operators, its energy is the predicted one,
//!         its pair amplitude is the predicted one, and an independent Lanczos computation finds the same state.
//!
//! Item [8], the effective field equations of the SFBS (Section 9 of the document):
//!   SB-6  the gauge-covariant derivative of the SFBS, nablaB_p d = d_p d - 2i g_F AF_LL_p d + g_v Av_LL_p x d: the Leibniz
//!         rule nablaB_p (Psi_a^T Kch_iota Psi_b) = (nabla_LL_p Psi_a)^T Kch_iota Psi_b + Psi_a^T Kch_iota (nabla_LL_p Psi_b) (with a
//!         random real combination of the six Lorentz generators as the Lorentz connection), the covariance under local
//!         gauge transformations, and the commutator
//!         [nablaB_p, nablaB_p'] d = -2i g_F FF_LL d + g_v Fv_LL x d;
//!   SB-7  the pair susceptibility chi_pr(omega, k) of the normal vacuum: the closed integrand against the second-order
//!         change of the sum of the positive Bogoliubov energies (16 x 16 matrices) with a small modulated pair field, at
//!         several frequencies and wave numbers; chi_pr(0, 0) = 1/G_crit; the coefficients Z_t0 and Z_s0 of omega^2 and k^2
//!         against numerical derivatives;
//!   SB-8  the SFBS boson of the normal vacuum (G_bind < G < G_crit): the pole of 1/(chi_pr - 1/G), its limits (2m at
//!         G_bind, 0 at G_crit), the Ginzburg-Landau mass (a_GL/Z_t0)^(1/2) and the dispersion relation;
//!   SB-9  the currents of the SFBS (the sources of the gauge fields) against numerical derivatives of the Lagrangian;
//! Item [8], the effective potential of the SFBS (Section 10 of the document):
//!   SB-11 the structure of the effective potential: the 16 x 16 Bogoliubov matrix of a general pair field d has the
//!         eigenvalues +-E^+(Delta^+), +-E^-(Delta^+), +-E^+(Delta^-), +-E^-(Delta^-), twice each, with the two gaps
//!         (Delta^+-)^2 = |d|^2 (1 +- zeta); hence Omega(d) = |d|^2/G - W_g((Delta^+)^2) - W_g((Delta^-)^2), checked against
//!         the eigenvalues integrated directly, at mu = 0 and with a Fermi sea; the invariance under exp(2i thF) Rv(U); the
//!         eigenvalues do not depend on the direction of q; the explicit matrix Dpair of the isospin part;
//!   SB-12 the closed forms at mu = 0: Xi(M^2) = 2 int (q^2 + M^2)^(1/2) and its two derivatives, W_g = Xi(m^2 + Delta^2) -
//!         Xi(m^2), the concavity of W_g in Delta^2 (also with a Fermi sea), the Ginzburg-Landau coefficients a_GL and
//!         b_GL = -Xi''(m^2) = Z_t0, the order |d|^6 of the remainder of the Ginzburg-Landau form, the massless form with
//!         its logarithm, and the gap equation as the stationarity of Omega.
//! Item [9], the minima, the breaking of the symmetry and SFBSv (Section 11 of the document):
//!   SB-13 the minima and the other stationary points of Omega at mu = 0, G = 22.5: 40 minimizations (BFGS with numerical
//!         gradients) from random fields end on the vacuum manifold |d| = Delta_vac, zeta = 0; the Hessian (6 x 6, in the
//!         real and imaginary parts) at the vacuum has three zero and three equal positive eigenvalues, at the non-unitary
//!         stationary field (Delta_vac/2)(1, i, 0) two negative, three zero and one positive, at d = 0 six equal negative;
//!         the generators of U(1)_F x SU(2)_v that leave the vacuum unchanged: only T^3 (and for the non-unitary field
//!         T^3 - F/2); the discrete element (thF = pi/2, U = i tau_1) of the stabilizer;
//!   SB-14 SFBSv: the parametrization d = exp(2i thG0) Rv(U(thG1, thG2)) ((Delta_vac + h_a) e_3 + i (c_n1 e_1 + c_n2 e_2)),
//!         the exact potential Omega_s(Delta_vac + h_a + rho_n) + Omega_s(Delta_vac + h_a - rho_n) for random fields, its
//!         Taylor coefficients (second, third, fourth derivatives of the sector potential) in closed form against numerical
//!         derivatives, the order of the remainder of the quartic expansion, and the curvatures of Omega at the vacuum from
//!         the Bogoliubov matrices directly (finite differences), independent of the sector formula.
//!   SB-10 the microscopic check of the effective field equation: the time-dependent mean-field (Bogoliubov-de Gennes)
//!         evolution of a small uniform pair field in the normal vacuum (G = 20) oscillates with the frequency M_B of the
//!         pole of the SFBS propagator (the mass in the effective Klein-Gordon equation), and the conserved functional of
//!         the evolution stays constant.
//!
//! Output (folder test2/data): the report t2_sfbs_checks.txt, the fragments t2_sfbs_fragments.md (explicit matrices
//! for the document) and the data files t2_sfbs_*.csv of the figures.
//!
//! Build and run (from the folder Revision_SuperLoop/test2/rust):
//!     cargo build --release && ./target/release/t2_sfbs
//! The program exits with status 1 if a check fails.  Double precision; every check states its tolerance; random
//! numbers come from a fixed seed (the same numbers in every run).
//!
//! Names (plain-text names of the registry of the folder):
//!   m, mu, Delta, G, Lambda      mass, chemical potential, gap, coupling, momentum cutoff of the spinor charges
//!   q, eps, E^+, E^-             momentum, eps = (q^2 + m^2)^(1/2), E^(+-) = ((eps +- mu)^2 + Delta^2)^(1/2)
//!   Kch_iota, Kch'_iota          the scalar and the pseudoscalar pair matrices (8 x 8)
//!   bSm, bSp                     the bi-quaternions of the two spinor charges of the Dirac field
//!   t_p, t_a                     the weights of the particle pairs and of the antiparticle pairs in the SFBS state
//!   n_F_site                     the number of spinor charges on one lattice site
mod fock;
mod quad;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use t2core::c64::{c, C64};
use t2core::cmat::{eigh, CMat};
use t2core::model::{adjoint, alpha, beta, bq_conj, bq_from, bq_mul, cconj, dirac_from_charges, eps2, eps3, gamma, gamma_ch, jgen, kgen, kpair, kpair_ps, lift2, lift4, pair_potential, rand_sl2c, sigma, spin_gen, su2, tiso};
use t2core::rng::Rng;
use t2core::sym::{tex_matrix, Lin, SymMat, GR};

const PI: f64 = std::f64::consts::PI;
/// Lambda_LL, the cutoff of the momenta (the unit of momentum and energy)
const LAMBDA: f64 = 1.0;
/// m_LL, the mass of the spinor charges
const MASS: f64 = 0.2;
/// G_LL of the condensed vacuum (mu_LL = 0); the critical coupling for m_LL = 0.2 is 21.2866
const G_VAC: f64 = 22.5;
/// G_LL and mu_LL of the superfluid with a Fermi sea (the values of Figure 6 of the manuscript)
const G_FS: f64 = 12.0;
const MU_FS: f64 = 0.5;

// ------------------------------------------------------------------ report, fragments, data files

/// the list of checks: identifier, statement, result, numbers found
struct Checks {
    title: String,
    items: Vec<(String, String, bool, String)>,
}

impl Checks {
    fn add(&mut self, id: &str, statement: &str, ok: bool, detail: String) {
        println!("[{}] {}  {}", if ok { "PASS" } else { "FAIL" }, id, detail);
        self.items.push((id.to_string(), statement.to_string(), ok, detail));
    }
    fn write(&self, path: &PathBuf) -> bool {
        let npass = self.items.iter().filter(|x| x.2).count();
        let mut s = format!("{}\n{} of {} checks passed\n\n", self.title, npass, self.items.len());
        for (id, st, ok, det) in &self.items {
            s.push_str(&format!(
                "[{}] {}  {}{}\n",
                if *ok { "PASS" } else { "FAIL" },
                id,
                st,
                if det.is_empty() { String::new() } else { format!("  -- {}", det) }
            ));
        }
        fs::write(path, s).expect("write report");
        npass == self.items.len()
    }
}

/// the explicit matrices and equations for the document: <!-- BEGIN name --> text <!-- END name -->
struct Frags(Vec<(String, String)>);

impl Frags {
    fn add(&mut self, name: &str, text: String) {
        self.0.push((name.to_string(), text));
    }
    fn write(&self, path: &PathBuf) {
        let mut f = fs::File::create(path).expect("fragments");
        writeln!(f, "<!-- generated by t2_sfbs (test2/rust/t2sfbs); do not edit -->").unwrap();
        for (n, t) in &self.0 {
            writeln!(f, "<!-- BEGIN {} -->\n{}\n<!-- END {} -->", n, t.trim_end(), n).unwrap();
        }
    }
}

/// a data file: comment lines "# key: value" (the first one is "# content: ..."), a header line, rows
struct Csv {
    text: String,
}

impl Csv {
    fn new(content: &str, meta: &[(&str, String)], header: &str) -> Csv {
        let mut text = format!("# content: {}\n", content);
        for (k, v) in meta {
            text.push_str(&format!("# {}: {}\n", k, v));
        }
        text.push_str(header);
        text.push('\n');
        Csv { text }
    }
    fn row(&mut self, fields: &[String]) {
        self.text.push_str(&fields.join(","));
        self.text.push('\n');
    }
    fn write(&self, dir: &PathBuf, name: &str) {
        fs::write(dir.join(name), &self.text).expect("write data file");
    }
}

/// the folder test2/data (two levels above the folder of this crate)
fn data_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p.push("data");
    fs::create_dir_all(&p).unwrap();
    p
}

fn sci(x: f64) -> String {
    format!("{:.1e}", x)
}

fn f10(x: f64) -> String {
    format!("{:.10}", x)
}

// ------------------------------------------------------------------ small helpers

/// the Frobenius inner product tr(a^dag b)
fn inner(a: &CMat, b: &CMat) -> C64 {
    let mut s = C64::ZERO;
    for (x, y) in a.a.iter().zip(&b.a) {
        s += x.conj() * *y;
    }
    s
}

/// the action of a generator X of the spinor field on a pair matrix: Psi -> (1 + i e X) Psi changes
/// Psi^T Kgen Psi into Psi^T (Kgen + i e (X^T Kgen + Kgen X)) Psi, so the pair matrix changes by X^T Kgen + Kgen X
fn pair_action(x: &CMat, k: &CMat) -> CMat {
    x.transpose().mul(k).add(&k.mul(x))
}

/// the antisymmetric part (K - K^T)/2: the only part of a pair matrix that the anticommuting components see
fn antisym(k: &CMat) -> CMat {
    k.sub(&k.transpose()).scale_r(0.5)
}

/// the 8 x 8 plane-wave Hamiltonian h(q) = (sum_j alpha_LL^j q^j + beta_LL m) (x) 1_2
fn h8(q: &[f64; 3], m: f64) -> CMat {
    let mut h = beta().scale_r(m);
    for j in 0..3 {
        h = h.add(&alpha(j + 1).scale_r(q[j]));
    }
    lift4(&h)
}

/// the gap of the unitary state at mu_LL = 0 from the closed form of the gap equation:
/// 1/G = (Lambda E_Lambda - M^2 arsinh(Lambda/M)) / (2 pi^2), M^2 = m^2 + Delta^2, E_Lambda = (Lambda^2 + M^2)^(1/2);
/// the right side decreases with Delta, so bisection finds the root (zero below the critical coupling)
fn gap_mu0(g: f64, m: f64) -> f64 {
    let rhs = |d: f64| {
        let mm = (m * m + d * d).sqrt();
        (LAMBDA * (LAMBDA * LAMBDA + mm * mm).sqrt() - mm * mm * (LAMBDA / mm).asinh()) / (2.0 * PI * PI)
    };
    if rhs(1e-14) < 1.0 / g {
        return 0.0;
    }
    let (mut lo, mut hi) = (1e-14, 5.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if rhs(mid) > 1.0 / g {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// the gap of the unitary state at any mu_LL from the gap equation by quadrature:
/// 1/G = int_0^Lambda q^2 dq/(2 pi^2) (1/E^+ + 1/E^-), composite Gauss-Legendre rule with a break point at the
/// Fermi momentum (where 1/E^- has its peak of width Delta)
fn gap_quad(g: f64, m: f64, mu: f64) -> f64 {
    let kf = if mu > m { (mu * mu - m * m).sqrt() } else { 0.0 };
    let rhs = |d: f64| {
        let cuts: Vec<f64> = if kf > 0.0 { vec![kf - 4.0 * d, kf, kf + 4.0 * d] } else { vec![] };
        let (xs, ws) = quad::composite(0.0, LAMBDA, &cuts, 40, 20);
        let mut s = 0.0;
        for (q, w) in xs.iter().zip(&ws) {
            let e = (q * q + m * m).sqrt();
            let ep = ((e + mu).powi(2) + d * d).sqrt();
            let em = ((e - mu).powi(2) + d * d).sqrt();
            s += w * q * q * (1.0 / ep + 1.0 / em);
        }
        s / (2.0 * PI * PI)
    };
    let (mut lo, mut hi) = (1e-10, 5.0);
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        if rhs(mid) > 1.0 / g {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

// ================================================================== item [7]

/// SB-1: the Lorentz-invariant (spin-zero) pairs of two spinor charges at one point
fn sb1(ch: &mut Checks, fr: &mut Frags) {
    // an orthonormal basis of the 28 antisymmetric 8 x 8 matrices: (E_kl - E_lk)/sqrt2, k < l
    let s = 0.5f64.sqrt();
    let mut basis = vec![];
    for k in 0..8 {
        for l in (k + 1)..8 {
            let mut b = CMat::zeros(8);
            b.set(k, l, c(s, 0.0));
            b.set(l, k, c(-s, 0.0));
            basis.push(b);
        }
    }
    // the six Lorentz generators on the eight components: rotations J^j and boosts K^j of the super-loop
    let gens: Vec<CMat> = (1..4).map(|j| lift4(&spin_gen(&jgen(j)))).chain((1..4).map(|j| lift4(&spin_gen(&kgen(j))))).collect();
    // the linear map Kgen -> (S^T Kgen + Kgen S) for the six generators; its Gram matrix on the basis
    let images: Vec<Vec<C64>> = basis
        .iter()
        .map(|b| {
            let mut v = vec![];
            for g in &gens {
                v.extend(pair_action(g, b).a.iter().cloned());
            }
            v
        })
        .collect();
    let mut gram = CMat::zeros(28);
    for i in 0..28 {
        for j in 0..28 {
            let mut z = C64::ZERO;
            for (x, y) in images[i].iter().zip(&images[j]) {
                z += x.conj() * *y;
            }
            gram.set(i, j, z);
        }
    }
    let (ev, evec) = eigh(&gram);
    let tol = 1e-12 * ev[27];
    let nullity = ev.iter().filter(|&&x| x < tol).count();
    // the projector onto the null space (coordinates in the orthonormal basis)
    let mut pnull = CMat::zeros(28);
    for n in 0..28 {
        if ev[n] < tol {
            for i in 0..28 {
                for j in 0..28 {
                    let v = pnull.get(i, j) + evec.get(i, n) * evec.get(j, n).conj();
                    pnull.set(i, j, v);
                }
            }
        }
    }
    // the six pair matrices of Theorem SF1 and the projector onto their span (they are mutually orthogonal)
    let six: Vec<CMat> = (1..4).map(kpair).chain((1..4).map(kpair_ps)).collect();
    let mut gram6 = 0.0f64;
    for a in 0..6 {
        for b in 0..6 {
            let z = inner(&six[a], &six[b]);
            let want = if a == b { 8.0 } else { 0.0 };
            gram6 = gram6.max((z - c(want, 0.0)).abs());
        }
    }
    let mut pspan = CMat::zeros(28);
    for k6 in &six {
        let u: Vec<C64> = basis.iter().map(|b| inner(b, k6)).collect();
        let nn: f64 = u.iter().map(|z| z.norm_sqr()).sum();
        for i in 0..28 {
            for j in 0..28 {
                let v = pspan.get(i, j) + (u[i] * u[j].conj()).scale(1.0 / nn);
                pspan.set(i, j, v);
            }
        }
    }
    let proj_diff = pnull.sub(&pspan).max_abs();
    // every invariant pair matrix pairs within the Weyl blocks: the blocks (rows 1..4, columns 5..8) vanish
    let mut mixed = 0.0f64;
    for k6 in &six {
        for i in 0..4 {
            for j in 4..8 {
                mixed = mixed.max(k6.get(i, j).abs()).max(k6.get(j, i).abs());
            }
        }
    }
    // the residuals of the six matrices
    let mut res = 0.0f64;
    for k6 in &six {
        for g in &gens {
            res = res.max(pair_action(g, k6).max_abs());
        }
    }
    // the isospin singlets (C gamma_ch) (x) eps2 and C (x) eps2 are symmetric: no pair
    let s1 = cconj().mul(&gamma_ch()).kron(&eps2());
    let s2 = cconj().kron(&eps2());
    let sym_res = antisym(&s1).max_abs().max(antisym(&s2).max_abs());
    let ok = nullity == 6 && proj_diff < 1e-10 && mixed == 0.0 && res < 1e-14 && gram6 < 1e-14 && sym_res == 0.0;
    ch.add(
        "SB-1",
        "spin-zero pairs: the antisymmetric 8 x 8 matrices Kgen with S^T Kgen + Kgen S = 0 for the six Lorentz generators S form a space of complex dimension 6, which is the span of Kch_1, Kch_2, Kch_3 (scalar) and Kch'_1, Kch'_2, Kch'_3 (pseudoscalar), mutually orthogonal (tr(Kgen^dag Kgen') = 8 or 0); their blocks between the two Weyl blocks vanish; the isospin singlets (C gamma_ch) (x) eps2 and C (x) eps2 are symmetric and give no pair (tol 1e-10)",
        ok,
        format!(
            "dimension of the solution space {} (eigenvalues of the Gram matrix below 1e-12 of the largest; the next is {:.3}), |P_null - P_span| = {}, largest mixed-block entry {}, largest residual {}, Gram deviation {}, antisymmetric part of the singlets {}",
            nullity,
            ev[nullity],
            sci(proj_diff),
            mixed,
            sci(res),
            sci(gram6),
            sym_res
        ),
    );
    // fragments: the pseudoscalar pair matrices, written out
    fr.add(
        "t2s_Kps",
        format!(
            "$$\\begin{{aligned}}\\mathsf K'^1_{{\\rm LL}}&={},\\\\ \\mathsf K'^2_{{\\rm LL}}&={},\\\\ \\mathsf K'^3_{{\\rm LL}}&={}.\\end{{aligned}}$$",
            tex_matrix(&kpair_ps(1)),
            tex_matrix(&kpair_ps(2)),
            tex_matrix(&kpair_ps(3))
        ),
    );
}

/// SB-2: spin, isospin, charges, fermion number and parity of the pairs
fn sb2(ch: &mut Checks, fr: &mut Frags) {
    let six: Vec<CMat> = (1..4).map(kpair).chain((1..4).map(kpair_ps)).collect();
    // the Hermitian spin generators i s_LL(J^j) = diag(sigma_j, sigma_j)/2 on the Dirac index
    let spin: Vec<CMat> = (1..4).map(|j| lift4(&spin_gen(&jgen(j))).scale(C64::I)).collect();
    let mut spin_cas = 0.0f64;
    let mut iso_cas = 0.0f64;
    for k in &six {
        let mut sc = CMat::zeros(8);
        let mut ic = CMat::zeros(8);
        for j in 0..3 {
            sc = sc.add(&pair_action(&spin[j], &pair_action(&spin[j], k)));
            ic = ic.add(&pair_action(&tiso(j + 1), &pair_action(&tiso(j + 1), k)));
        }
        spin_cas = spin_cas.max(sc.max_abs()); // S(S+1) Kgen with S = 0
        iso_cas = iso_cas.max(ic.sub(&k.scale_r(2.0)).max_abs()); // I(I+1) Kgen with I = 1
    }
    // the electric charge Q = T^3: K(+1) = (Kch_1 - i Kch_2)/sqrt2, K(0) = Kch_3, K(-1) = (Kch_1 + i Kch_2)/sqrt2
    let r = 0.5f64.sqrt();
    let kp = kpair(1).sub(&kpair(2).scale(C64::I)).scale_r(r);
    let k0 = kpair(3);
    let km = kpair(1).add(&kpair(2).scale(C64::I)).scale_r(r);
    let q = tiso(3);
    let charge = pair_action(&q, &kp)
        .sub(&kp)
        .max_abs()
        .max(pair_action(&q, &k0).max_abs())
        .max(pair_action(&q, &km).add(&km).max_abs());
    // the fermion number: the generator 1_8 gives 2 Kgen
    let fnum = six.iter().map(|k| pair_action(&CMat::identity(8), k).sub(&k.scale_r(2.0)).max_abs()).fold(0.0, f64::max);
    // parity: Psi -> gamma^0 Psi(x^0, -x): Kgen -> gamma0^T Kgen gamma0
    let g0 = lift4(&gamma(0));
    let mut par = 0.0f64;
    for (i, k) in six.iter().enumerate() {
        let sign = if i < 3 { 1.0 } else { -1.0 };
        par = par.max(g0.transpose().mul(k).mul(&g0).sub(&k.scale_r(sign)).max_abs());
    }
    // the Weyl-block factor eps2 is the spin singlet: sigma_j^T eps2 + eps2 sigma_j = 0
    let singlet = (1..4).map(|j| sigma(j).transpose().mul(&eps2()).add(&eps2().mul(&sigma(j))).max_abs()).fold(0.0, f64::max);
    let ok = spin_cas < 1e-14 && iso_cas < 1e-14 && charge < 1e-14 && fnum < 1e-14 && par < 1e-14 && singlet == 0.0;
    ch.add(
        "SB-2",
        "quantum numbers of the pairs: the spin Casimir sum_j G(S_j)^2 Kgen vanishes (spin 0) and the isospin Casimir sum_iota G(T_iota)^2 Kgen = 2 Kgen (isospin 1) for all six pair matrices, with G(X) Kgen = X^T Kgen + Kgen X; G(T_3) K(q) = q K(q) for K(+1) = (Kch_1 - i Kch_2)/sqrt2, K(0) = Kch_3, K(-1) = (Kch_1 + i Kch_2)/sqrt2; G(1_8) Kgen = 2 Kgen (fermion number 2); gamma0^T Kch_iota gamma0 = Kch_iota and gamma0^T Kch'_iota gamma0 = -Kch'_iota (parity); sigma_j^T eps2 + eps2 sigma_j = 0 (tol 1e-14)",
        ok,
        format!(
            "spin Casimir {}, isospin Casimir - 2: {}, charges {}, fermion number {}, parity {}, singlet {}",
            sci(spin_cas),
            sci(iso_cas),
            sci(charge),
            sci(fnum),
            sci(par),
            singlet
        ),
    );
    fr.add(
        "t2s_Kcharge",
        format!(
            "$$\\begin{{aligned}}\\sqrt2\\,\\mathsf K^{{(+1)}}_{{\\rm LL}}=\\mathsf K^1_{{\\rm LL}}-i\\mathsf K^2_{{\\rm LL}}&={},\\\\ \\sqrt2\\,\\mathsf K^{{(-1)}}_{{\\rm LL}}=\\mathsf K^1_{{\\rm LL}}+i\\mathsf K^2_{{\\rm LL}}&={},\\\\ \\mathsf K^{{(0)}}_{{\\rm LL}}=\\mathsf K^3_{{\\rm LL}}&={}.\\end{{aligned}}$$",
            tex_matrix(&kpair(1).sub(&kpair(2).scale(C64::I))),
            tex_matrix(&kpair(1).add(&kpair(2).scale(C64::I))),
            tex_matrix(&kpair(3))
        ),
    );
}

/// the fragments of Section 8 that are not matrices of a check: the Dirac part of the plane-wave Hamiltonian h(q) and
/// the pair operators of definite charge, written out; every printed expression is evaluated with test values and
/// compared with the matrix it describes (the largest deviation is returned)
fn frags_item7(fr: &mut Frags) -> f64 {
    let mut dev = 0.0f64;
    // h(q) = sum_j alpha^j q^j + beta m (Dirac part; on the eight components it is this matrix (x) sigma_0)
    let qv = [0.31, -0.47, 0.83];
    let mval = 0.27;
    let mut hs = SymMat::zeros(4, 4);
    for j in 0..3 {
        hs.add_scaled(&alpha(j + 1), &Lin::mono(&format!("q^{}_{{\\rm LL}}", j + 1), c(qv[j], 0.0)));
    }
    hs.add_scaled(&beta(), &Lin::mono("m_{\\rm LL}", c(mval, 0.0)));
    let mut hnum = beta().scale_r(mval);
    for j in 0..3 {
        hnum = hnum.add(&alpha(j + 1).scale_r(qv[j]));
    }
    for i in 0..4 {
        for j in 0..4 {
            dev = dev.max((hs.get(i, j).eval() - hnum.get(i, j)).abs());
        }
    }
    fr.add("t2s_h4", format!("$$\\sum_j\\alpha^j_{{\\rm LL}}q^j_{{\\rm LL}}+\\beta_{{\\rm LL}}m_{{\\rm LL}}={}.$$", hs.tex()));
    // the pair operators of definite charge: sqrt2 Pi(+1) = Psi^T (Kch_1 - i Kch_2) Psi, sqrt2 Pi(-1), Pi(0) = Pi_3
    let comp = |k: usize| format!("\\Psi_{{{{\\rm LL}},{}}}", k + 1);
    let prod_val = |a: usize, b: usize| -> C64 { c(0.1 * (a as f64 + 1.0), 0.07 * (b as f64) - 0.2) };
    let mats = [
        ("\\sqrt2\\,\\Pi^{(+1)}_{\\rm LL}", kpair(1).sub(&kpair(2).scale(C64::I))),
        ("\\sqrt2\\,\\Pi^{(-1)}_{\\rm LL}", kpair(1).add(&kpair(2).scale(C64::I))),
        ("\\Pi^{(0)}_{\\rm LL}", kpair(3)),
    ];
    let mut text = String::from("$$\\begin{aligned}\n");
    for (n, (name, km)) in mats.iter().enumerate() {
        let mut l = Lin::zero();
        let mut num = C64::ZERO;
        for a in 0..8 {
            for b in (a + 1)..8 {
                let coef = km.get(a, b) - km.get(b, a);
                if coef.abs() > 0.0 {
                    l.add_term(&format!("{}{}", comp(a), comp(b)), GR::from_c(coef), prod_val(a, b));
                    num += coef * prod_val(a, b);
                }
            }
        }
        dev = dev.max((l.eval() - num).abs());
        text.push_str(&format!("{}&={}{}\n", name, l.tex(), if n < 2 { ",\\\\" } else { "." }));
    }
    text.push_str("\\end{aligned}$$");
    fr.add("t2s_Pi_charge", text);
    dev
}

/// SB-3: the super-loop form of the pairs
fn sb3(ch: &mut Checks) {
    // the linear map from the eight coefficients (xi_p(bSm), xi_p(bSp)) to the eight components of Psi_LL
    let zero = [C64::ZERO; 4];
    let unit = |p: usize| {
        let mut e = [C64::ZERO; 4];
        e[p] = C64::ONE;
        e
    };
    let mut lmap = CMat::zeros(8);
    for p in 0..4 {
        let col_m = dirac_from_charges(&unit(p), &zero);
        let col_p = dirac_from_charges(&zero, &unit(p));
        for k in 0..8 {
            lmap.set(k, p, col_m[k]);
            lmap.set(k, 4 + p, col_p[k]);
        }
    }
    // the bilinear form of Im_iota(b conj(b)): C_pq = (e_iota-coefficient of e_p conj(e_q)); for anticommuting
    // coefficients only its antisymmetric part counts
    let mut worst = 0.0f64;
    let mut scalar_part = 0.0f64;
    for iota in 1..4 {
        let mut cm = CMat::zeros(4);
        let mut nm = CMat::zeros(4);
        for p in 0..4 {
            for qq in 0..4 {
                let prod = bq_mul(&unit(p), &bq_conj(&unit(qq)));
                cm.set(p, qq, prod[iota]);
                nm.set(p, qq, prod[0]);
            }
        }
        let ca = antisym(&cm);
        scalar_part = scalar_part.max(antisym(&nm).max_abs());
        let mut want = CMat::zeros(8);
        want.put(0, 0, &ca);
        want.put(4, 4, &ca);
        let want = want.scale(c(0.0, -2.0));
        let got = antisym(&lmap.transpose().mul(&kpair(iota)).mul(&lmap));
        worst = worst.max(got.sub(&want).max_abs());
    }
    // the transformation law with commuting test values: (v b conj(g)) conj(v b conj(g)) = v (b conj(b)) conj(v) for
    // the spinor charge bSp, and (v b g^dag) conj(v b g^dag) = v (b conj(b)) conj(v) for bSm
    let mut rng = Rng(7177);
    let mut law = 0.0f64;
    for _ in 0..200 {
        let b: [C64; 4] = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
        let g = bq_from(&rand_sl2c(&mut rng));
        let mut vr = [rng.unit(), rng.unit(), rng.unit(), rng.unit()];
        let nv = vr.iter().map(|x| x * x).sum::<f64>().sqrt();
        for x in vr.iter_mut() {
            *x /= nv;
        }
        let v = [c(vr[0], 0.0), c(vr[1], 0.0), c(vr[2], 0.0), c(vr[3], 0.0)];
        let bbar = bq_mul(&b, &bq_conj(&b));
        let rhs = bq_mul(&bq_mul(&v, &bbar), &bq_conj(&v));
        let bp = bq_mul(&bq_mul(&v, &b), &bq_conj(&g));
        let lhs_p = bq_mul(&bp, &bq_conj(&bp));
        let gdag = t2core::model::bq_dag(&g);
        let bm = bq_mul(&bq_mul(&v, &b), &gdag);
        let lhs_m = bq_mul(&bm, &bq_conj(&bm));
        for k in 0..4 {
            law = law.max((lhs_p[k] - rhs[k]).abs()).max((lhs_m[k] - rhs[k]).abs());
        }
    }
    let ok = worst < 1e-14 && scalar_part == 0.0 && law < 1e-11;
    ch.add(
        "SB-3",
        "super-loop form: as bilinear forms of the anticommuting coefficients xi_p of the two spinor charges, the pair Pi_iota (the pair matrix Kch_iota between the transposed field and the field) equals -2i (Im_iota(bSm conj(bSm)) + Im_iota(bSp conj(bSp))) for iota = 1, 2, 3, and the scalar part (the norm form of b with itself) of b conj(b) is symmetric (it vanishes for anticommuting coefficients); for 200 random commuting b, unit bi-quaternions g and unit real quaternions v, (v b conj(g)) conj(v b conj(g)) = v (b conj(b)) conj(v) = (v b g^dag) conj(v b g^dag) (tol 1e-14 and 1e-11)",
        ok,
        format!("largest deviation of the pair forms {}, antisymmetric part of the norm form {}, largest deviation of the transformation law {}", sci(worst), scalar_part, sci(law)),
    );
}

/// SB-4: the commutators of the pair operators of one lattice site (8 fermion modes, 256 states)
fn sb4(ch: &mut Checks, dir: &PathBuf) {
    let dim = 256;
    // Pi_iota = sum_kl Kch_iota[k][l] c_k c_l (c_l acts first)
    let pair_op = |k: CMat| {
        move |v: &[C64]| -> Vec<C64> {
            let mut out = vec![C64::ZERO; v.len()];
            for a in 0..8 {
                for b in 0..8 {
                    let z = k.get(a, b);
                    if z.abs() == 0.0 {
                        continue;
                    }
                    let t = fock::ann(&fock::ann(v, b), a);
                    for (o, x) in out.iter_mut().zip(t) {
                        *o += z * x;
                    }
                }
            }
            out
        }
    };
    let pis: Vec<CMat> = (1..4).map(|i| fock::dense(dim, &pair_op(kpair(i)))).collect();
    // the number of spinor charges n_F and the isospin n_iota = c^dag T_iota c on the site
    let quad_op = |m: CMat| {
        move |v: &[C64]| -> Vec<C64> {
            let mut out = vec![C64::ZERO; v.len()];
            for a in 0..8 {
                for b in 0..8 {
                    let z = m.get(a, b);
                    if z.abs() > 0.0 {
                        fock::hop_add(v, a, b, z, &mut out);
                    }
                }
            }
            out
        }
    };
    let nf = fock::dense(dim, &quad_op(CMat::identity(8)));
    let niso: Vec<CMat> = (1..4).map(|i| fock::dense(dim, &quad_op(tiso(i)))).collect();
    let comm = |a: &CMat, b: &CMat| a.mul(b).sub(&b.mul(a));
    let mut e1 = 0.0f64; // [Pi_i, Pi_j]
    let mut e2 = 0.0f64; // [n_F, Pi_i] + 2 Pi_i
    let mut e3 = 0.0f64; // [Pi_i, Pi_j^dag] - formula
    let mut e4 = 0.0f64; // the general formula 2 tr(K L^dag) - 4 c^dag (K L^dag)^T c
    for i in 0..3 {
        e2 = e2.max(comm(&nf, &pis[i]).add(&pis[i].scale_r(2.0)).max_abs());
        for j in 0..3 {
            e1 = e1.max(comm(&pis[i], &pis[j]).max_abs());
            let lhs = comm(&pis[i], &pis[j].dagger());
            let mut rhs = CMat::zeros(dim);
            if i == j {
                rhs = CMat::identity(dim).scale_r(16.0).sub(&nf.scale_r(4.0));
            }
            for k in 0..3 {
                let e = t2core::model::eps3(i, j, k);
                if e != 0.0 {
                    rhs = rhs.add(&niso[k].scale(c(0.0, 8.0 * e)));
                }
            }
            e3 = e3.max(lhs.sub(&rhs).max_abs());
            let kl = kpair(i + 1).mul(&kpair(j + 1).dagger());
            let gen = CMat::identity(dim).scale(kl.trace().scale(2.0)).sub(&fock::dense(dim, &quad_op(kl.transpose())).scale_r(4.0));
            e4 = e4.max(lhs.sub(&gen).max_abs());
        }
    }
    // the commutator [Pi_3, Pi_3^dag] on the 256 states: it is diagonal, 16 - 4 n_F
    let c33 = comm(&pis[2], &pis[2].dagger());
    let mut off = 0.0f64;
    let mut csv = Csv::new(
        "the commutator of the pair operator Pi_3 of one lattice site with its Hermitian conjugate on the 256 states of the site (8 fermion modes), against the number n_F_site of spinor charges on the site; the line is 16 - 4 n_F_site (Test 2, item 7, Figure 21(c))",
        &[],
        "n_F_site,commutator_33",
    );
    for b in 0..dim {
        for b2 in 0..dim {
            if b != b2 {
                off = off.max(c33.get(b, b2).abs());
            }
        }
        csv.row(&[format!("{}", (b as u32).count_ones()), format!("{:.12}", c33.get(b, b).re)]);
    }
    csv.write(dir, "t2_sfbs_site_commutator.csv");
    let ok = e1 < 1e-12 && e2 < 1e-12 && e3 < 1e-12 && e4 < 1e-12 && off < 1e-12;
    ch.add(
        "SB-4",
        "the pairs are bosons: on one lattice site (8 modes c_site, 256 states, the Jordan-Wigner representation) the site pair operators Pi_site^iota satisfy [Pi_site^iota, Pi_site^iota'] = 0, [n_F_site, Pi_site^iota] = -2 Pi_site^iota, [Pi_site^iota, (Pi_site^iota')^dag] = 2 tr(Kch_iota Kch_iota'^dag) - 4 (c_site^dag (Kch_iota Kch_iota'^dag)^T c_site), which is 16 - 4 n_F_site for iota = iota' and 8 i sum_iota'' eps(iota, iota', iota'') n_iota''_site otherwise, with n_iota_site = c_site^dag T_LL^iota c_site; [Pi_site^3, (Pi_site^3)^dag] is diagonal (tol 1e-12)",
        ok,
        format!("largest deviations {}, {}, {}, {}; off-diagonal part of [Pi_3, Pi_3^dag] {}", sci(e1), sci(e2), sci(e3), sci(e4), sci(off)),
    );
}

/// the many-body mean-field Hamiltonian of one pair of momenta (q, -q): modes 0..8 are the components of Psi(q),
/// modes 8..16 those of Psi(-q);  H = sum Psi(q)^dag (h(q) - mu) Psi(q) + Psi(-q)^dag (h(-q) - mu) Psi(-q)
///                                  + sum_kl P_kl Psi_k(q)^dag Psi_l(-q)^dag + h.c.,  P = Delta Kch_3^dag
fn hq_apply(v: &[C64], hp: &CMat, hm: &CMat, p: &CMat, mu: f64) -> Vec<C64> {
    let mut out = vec![C64::ZERO; v.len()];
    for a in 0..8 {
        for b in 0..8 {
            let mut z = hp.get(a, b);
            if a == b {
                z = z - c(mu, 0.0);
            }
            if z.abs() > 0.0 {
                fock::hop_add(v, a, b, z, &mut out);
            }
            let mut z2 = hm.get(a, b);
            if a == b {
                z2 = z2 - c(mu, 0.0);
            }
            if z2.abs() > 0.0 {
                fock::hop_add(v, 8 + a, 8 + b, z2, &mut out);
            }
            let w = p.get(a, b);
            if w.abs() > 0.0 {
                fock::pair_cre_add(v, a, 8 + b, w, &mut out);
                fock::pair_ann_add(v, a, 8 + b, w.conj(), &mut out);
            }
        }
    }
    out
}

/// one SFBS state: the result of the Fock-space computation for one momentum
struct PairState {
    q: f64,
    t_p: f64,
    t_a: f64,
    residual: f64,
    qp_annihilation: f64,
    e0: f64,
    e0_bdg: f64,
    pair_amp: f64,
    pair_amp_pred: f64,
    lanczos: Option<(f64, f64, f64, f64, f64, f64)>, // (E_L, overlap, t_p from Lanczos, t_a from Lanczos, occupations of particles and antiparticles)
}

/// the SFBS state for the pair of momenta (q n, -q n), n a fixed generic direction
fn sfbs_state(qmag: f64, m: f64, mu: f64, delta: f64, lanczos: bool) -> PairState {
    let dirn = [2.0 / 7.0, 3.0 / 7.0, 6.0 / 7.0];
    let qv = [qmag * dirn[0], qmag * dirn[1], qmag * dirn[2]];
    let qmv = [-qv[0], -qv[1], -qv[2]];
    let hp = h8(&qv, m);
    let hm = h8(&qmv, m);
    let k3 = kpair(3);
    let p = k3.dagger().scale_r(delta);
    let eps = (qmag * qmag + m * m).sqrt();
    let ep = ((eps + mu).powi(2) + delta * delta).sqrt();
    let em = ((eps - mu).powi(2) + delta * delta).sqrt();
    // the eigenvectors of h(q): columns 0..4 belong to -eps, 4..8 to +eps
    let (_, vq) = eigh(&hp);
    let col = |j: usize| -> Vec<C64> { (0..8).map(|i| vq.get(i, j)).collect() };
    let partner = |x: &Vec<C64>| -> Vec<C64> { k3.matvec(x).iter().map(|z| z.conj()).collect() }; // (Kch_3 x)^*
    let dim = 1usize << 16;
    let mut vac = vec![C64::ZERO; dim];
    vac[0] = C64::ONE;
    // the Dirac sea |N>: the negative-energy states of h(q) and of h(-q) filled
    let mut sea = vac;
    for n in 0..4 {
        sea = fock::cre_state(&sea, &col(n), 0);
    }
    for n in 0..4 {
        sea = fock::cre_state(&sea, &partner(&col(n)), 8);
    }
    let t_p = delta / (em + eps - mu);
    let t_a = delta / (ep + eps + mu);
    // |SF> = prod_n (1 - t_p b^dag(chi_n) b^dag(w_n)) prod_n (1 + t_a b(chi'_n) b(w'_n)) |N>
    let mut sf = sea.clone();
    for n in 4..8 {
        let chi = col(n);
        let w = partner(&chi);
        let pairv = fock::cre_state(&fock::cre_state(&sf, &w, 8), &chi, 0);
        sf = fock::axpy(&sf, c(-t_p, 0.0), &pairv);
    }
    for n in 0..4 {
        let chi = col(n);
        let w = partner(&chi);
        let pairv = fock::ann_state(&fock::ann_state(&sf, &w, 8), &chi, 0);
        sf = fock::axpy(&sf, c(t_a, 0.0), &pairv);
    }
    let nsf = fock::norm(&sf);
    for x in sf.iter_mut() {
        *x = x.scale(1.0 / nsf);
    }
    // the predicted ground-state energy and the BdG value: E_0 = tr(A)/2 - (sum of the 16 positive eigenvalues)/2
    let e0 = -8.0 * mu - 4.0 * (ep + em);
    let mut amat = CMat::zeros(16);
    let mut bmat = CMat::zeros(16);
    for a in 0..8 {
        for b in 0..8 {
            amat.set(a, b, hp.get(a, b) - if a == b { c(mu, 0.0) } else { C64::ZERO });
            amat.set(8 + a, 8 + b, hm.get(a, b) - if a == b { c(mu, 0.0) } else { C64::ZERO });
            bmat.set(a, 8 + b, p.get(a, b));
            bmat.set(8 + b, a, -p.get(a, b));
        }
    }
    let mut bdg = CMat::zeros(32);
    bdg.put(0, 0, &amat);
    bdg.put(0, 16, &bmat);
    bdg.put(16, 0, &bmat.conj().scale_r(-1.0));
    bdg.put(16, 16, &amat.conj().scale_r(-1.0));
    let (bev, bvec) = eigh(&bdg);
    let e0_bdg = 0.5 * amat.trace().re - 0.5 * bev[16..].iter().sum::<f64>();
    // the residual of H|SF> = E_0 |SF>
    let hsf = hq_apply(&sf, &hp, &hm, &p, mu);
    let residual = fock::norm(&fock::axpy(&hsf, c(-e0, 0.0), &sf));
    // the sixteen quasi-particle operators gamma_n = sum_k (U_kn^* a_k + V_kn^* a_k^dag) of the positive eigenvalues
    let mut qpa = 0.0f64;
    for n in 16..32 {
        let mut out = vec![C64::ZERO; dim];
        for k in 0..16 {
            let u = bvec.get(k, n);
            let vv = bvec.get(16 + k, n);
            if u.abs() > 1e-15 {
                let t = fock::ann(&sf, k);
                for (o, x) in out.iter_mut().zip(t) {
                    *o += u.conj() * x;
                }
            }
            if vv.abs() > 1e-15 {
                let t = fock::cre(&sf, k);
                for (o, x) in out.iter_mut().zip(t) {
                    *o += vv.conj() * x;
                }
            }
        }
        qpa = qpa.max(fock::norm(&out));
    }
    // the pair amplitude <Pi_3> of the two momenta: Pi_3 = 2 sum_kl Kch_3[k][l] Psi_k(q) Psi_l(-q)
    let mut piv = vec![C64::ZERO; dim];
    for a in 0..8 {
        for b in 0..8 {
            let z = k3.get(a, b);
            if z.abs() > 0.0 {
                let t = fock::ann(&fock::ann(&sf, 8 + b), a);
                for (o, x) in piv.iter_mut().zip(t) {
                    *o += z.scale(2.0) * x;
                }
            }
        }
    }
    let pair_amp = fock::dot(&sf, &piv).re;
    let pair_amp_pred = -4.0 * delta * (1.0 / ep + 1.0 / em);
    // an independent ground state: the Lanczos method on the whole Fock space of the 16 modes
    let lz = if lanczos {
        let op = |v: &[C64]| hq_apply(v, &hp, &hm, &p, mu);
        let (el, gs, _res) = fock::lanczos_ground(&op, dim, 120, 99);
        let overlap = fock::dot(&gs, &sf).abs();
        // the weights from the amplitudes of the states with one pair added to (or removed from) the Dirac sea
        let chi = col(4);
        let p1 = fock::cre_state(&fock::cre_state(&sea, &partner(&chi), 8), &chi, 0);
        let chia = col(0);
        let a1 = fock::ann_state(&fock::ann_state(&sea, &partner(&chia), 8), &chia, 0);
        let base = fock::dot(&sea, &gs);
        let tpl = -(fock::dot(&p1, &gs) / base).re;
        let tal = (fock::dot(&a1, &gs) / base).re;
        // the occupation of one positive-energy state at q and the vacancy of one negative-energy state at q
        let occ_p = fock::norm(&fock::ann_state(&gs, &chi, 0)).powi(2);
        let occ_a = fock::norm(&fock::cre_state(&gs, &chia, 0)).powi(2);
        Some((el, overlap, tpl, tal, occ_p, occ_a))
    } else {
        None
    };
    PairState { q: qmag, t_p, t_a, residual, qp_annihilation: qpa, e0, e0_bdg, pair_amp, pair_amp_pred, lanczos: lz }
}

/// SB-5: the SFBS state is the coherent pair state
fn sb5(ch: &mut Checks, dir: &PathBuf, d_vac: f64, d_fs: f64) {
    let cases = [("vacuum", MASS, 0.0, d_vac), ("Fermi_sea", MASS, MU_FS, d_fs)];
    let qs: [f64; 6] = [0.05, 0.2, 0.35, 0.4583, 0.6, 0.8];
    let lanczos_at: [f64; 3] = [0.05, 0.35, 0.6];
    let mut worst = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut fock_csv = Csv::new(
        "the weights t_p of the particle pairs and t_a of the antiparticle pairs, and the occupation of a positive-energy state and the vacancy of a negative-energy state, of the ground state of the mean-field Hamiltonian of one pair of momenta (q, -q), found by the Lanczos method in the Fock space of the 16 modes (65536 states), next to the predicted values (Test 2, item 7, Figure 21(a), (b), points)",
        &[("m", format!("{}", MASS)), ("Lambda", format!("{}", LAMBDA)), ("direction", "(2, 3, 6)/7".into())],
        "state,mu,Delta,q,t_p,t_a,occupation_particles,occupation_antiparticles,source",
    );
    let mut lines = vec![];
    for (name, m, mu, d) in cases {
        for &q in &qs {
            let lz = lanczos_at.iter().any(|&x| (x - q).abs() < 1e-12);
            let st = sfbs_state(q, m, mu, d, lz);
            worst.0 = worst.0.max(st.residual);
            worst.1 = worst.1.max(st.qp_annihilation);
            worst.2 = worst.2.max((st.e0 - st.e0_bdg).abs());
            worst.3 = worst.3.max((st.pair_amp - st.pair_amp_pred).abs());
            if let Some((el, ov, tpl, tal, op, oa)) = st.lanczos {
                worst.4 = worst.4.max((el - st.e0).abs());
                worst.5 = worst.5.max((1.0 - ov).abs());
                worst.6 = worst.6.max((tpl - st.t_p).abs().max((tal - st.t_a).abs()));
                fock_csv.row(&[name.into(), format!("{}", mu), f10(d), format!("{}", q), format!("{:.12}", tpl), format!("{:.12}", tal), format!("{:.12}", op), format!("{:.12}", oa), "Lanczos".into()]);
            }
            lines.push(format!("{} q = {}: E_0 = {:.10}", name, st.q, st.e0));
        }
    }
    fock_csv.write(dir, "t2_sfbs_pair_weights_fock.csv");
    // the predicted curves of the figure
    let mut curves = Csv::new(
        "the predicted weights t_p = Delta/(E^- + eps - mu) of the particle pairs and t_a = Delta/(E^+ + eps + mu) of the antiparticle pairs in the SFBS state, and the occupation t_p^2/(1 + t_p^2) of a positive-energy state and the vacancy t_a^2/(1 + t_a^2) of a negative-energy state, as functions of the momentum (Test 2, item 7, Figure 21(a), (b), lines)",
        &[("m", format!("{}", MASS)), ("Lambda", format!("{}", LAMBDA))],
        "state,mu,Delta,q,t_p,t_a,occupation_particles,occupation_antiparticles",
    );
    for (name, m, mu, d) in [("vacuum", MASS, 0.0, d_vac), ("Fermi_sea", MASS, MU_FS, d_fs)] {
        for i in 0..=400 {
            let q = LAMBDA * i as f64 / 400.0;
            let eps = (q * q + m * m).sqrt();
            let ep = ((eps + mu).powi(2) + d * d).sqrt();
            let em = ((eps - mu).powi(2) + d * d).sqrt();
            let tp = d / (em + eps - mu);
            let ta = d / (ep + eps + mu);
            curves.row(&[name.into(), format!("{}", mu), f10(d), format!("{:.6}", q), format!("{:.12}", tp), format!("{:.12}", ta), format!("{:.12}", tp * tp / (1.0 + tp * tp)), format!("{:.12}", ta * ta / (1.0 + ta * ta))]);
        }
    }
    curves.write(dir, "t2_sfbs_pair_weights.csv");
    let ok = worst.0 < 1e-11 && worst.1 < 1e-11 && worst.2 < 1e-12 && worst.3 < 1e-12 && worst.4 < 1e-9 && worst.5 < 1e-8 && worst.6 < 1e-6;
    ch.add(
        "SB-5",
        "the SFBS state of one pair of momenta (q, -q) (16 modes, 65536 states; vacuum mu = 0 and Fermi sea mu = 0.5, six momenta each): the coherent pair state prod_n (1 - t_p b^dag(chi_n) b^dag(w_n)) prod_n (1 + t_a b(chi'_n) b(w'_n)) |Dirac sea>, w = (Kch_3 chi)^*, t_p = Delta/(E^- + eps - mu), t_a = Delta/(E^+ + eps + mu), satisfies H|SF> = E_0|SF> with E_0 = -8 mu - 4 (E^+ + E^-), equal to the BdG value; it is annihilated by the sixteen quasi-particle operators of the 32 x 32 BdG matrix; <Pi_3> = -4 Delta (1/E^+ + 1/E^-); the Lanczos ground state (120 steps) has the energy E_0, overlap 1 with |SF> and the weights t_p, t_a (tol 1e-11, 1e-11, 1e-12, 1e-12, 1e-9, 1e-8, 1e-6)",
        ok,
        format!(
            "largest |H SF - E_0 SF| {}, largest |gamma_n SF| {}, |E_0 - E_0(BdG)| {}, |<Pi_3> - prediction| {}, |E_Lanczos - E_0| {}, |1 - overlap| {}, weights from Lanczos {}; gaps Delta = {} (vacuum, G = {}) and {} (Fermi sea, G = {})",
            sci(worst.0),
            sci(worst.1),
            sci(worst.2),
            sci(worst.3),
            sci(worst.4),
            sci(worst.5),
            sci(worst.6),
            f10(d_vac),
            G_VAC,
            f10(d_fs),
            G_FS
        ),
    );
    for l in lines {
        println!("    {}", l);
    }
}

// ================================================================== item [8]

/// the 3 x 3 antisymmetric matrix of an isospin vector a in the vector representation: Avec(a) d = a x d
fn avec(a: &[f64; 3]) -> [[f64; 3]; 3] {
    [[0.0, -a[2], a[1]], [a[2], 0.0, -a[0]], [-a[1], a[0], 0.0]]
}

fn cross_c(a: &[f64; 3], d: &[C64; 3]) -> [C64; 3] {
    [d[2].scale(a[1]) - d[1].scale(a[2]), d[0].scale(a[2]) - d[2].scale(a[0]), d[1].scale(a[0]) - d[0].scale(a[1])]
}

/// the covariant derivative of the SFBS at a point: nablaB d = (derivative of d) - 2i g_F AF d + g_v Av x d
fn nabla_b(dd: &[C64; 3], d: &[C64; 3], af: f64, av: &[f64; 3], gf: f64, gv: f64) -> [C64; 3] {
    let x = cross_c(av, d);
    let mut r = [C64::ZERO; 3];
    for i in 0..3 {
        r[i] = dd[i] + d[i] * c(0.0, -2.0 * gf * af) + x[i].scale(gv);
    }
    r
}

fn rnd_c(rng: &mut Rng) -> C64 {
    c(rng.unit(), rng.unit())
}

/// SB-6: the gauge-covariant derivative of the SFBS
fn sb6(ch: &mut Checks, fr: &mut Frags) {
    let mut rng = Rng(8086);
    let (gv, gf) = (0.83, 0.57);
    // (i) the Leibniz rule with linear test fields phi(x) = phi0 + phi1 x^p, psi(x) = psi0 + psi1 x^p at x = 0, random
    // potentials and a random Lorentz connection Gamma_p = sgen_LL(uLw) (x) sigma_0
    let mut leib = 0.0f64;
    for _ in 0..50 {
        let phi0: Vec<C64> = (0..8).map(|_| rnd_c(&mut rng)).collect();
        let phi1: Vec<C64> = (0..8).map(|_| rnd_c(&mut rng)).collect();
        let psi0: Vec<C64> = (0..8).map(|_| rnd_c(&mut rng)).collect();
        let psi1: Vec<C64> = (0..8).map(|_| rnd_c(&mut rng)).collect();
        let av = [rng.unit(), rng.unit(), rng.unit()];
        let af = rng.unit();
        // a random Lorentz connection: a real combination of the six generators (rotations jgen, boosts kgen)
        let mut gam = CMat::zeros(8);
        for j in 1..4 {
            gam = gam.add(&lift4(&spin_gen(&jgen(j))).scale_r(rng.unit()));
            gam = gam.add(&lift4(&spin_gen(&kgen(j))).scale_r(rng.unit()));
        }
        let mut acal = CMat::identity(8).scale_r(gf * af);
        for i in 0..3 {
            acal = acal.add(&tiso(i + 1).scale_r(gv * av[i]));
        }
        let nab = |f0: &Vec<C64>, f1: &Vec<C64>| -> Vec<C64> {
            let g = gam.matvec(f0);
            let a = acal.matvec(f0);
            (0..8).map(|k| f1[k] + g[k] - C64::I * a[k]).collect()
        };
        let nphi = nab(&phi0, &phi1);
        let npsi = nab(&psi0, &psi1);
        let bil = |x: &Vec<C64>, km: &CMat, y: &Vec<C64>| -> C64 {
            let ky = km.matvec(y);
            x.iter().zip(&ky).fold(C64::ZERO, |s, (a, b)| s + *a * *b)
        };
        let mut f = [C64::ZERO; 3];
        let mut df = [C64::ZERO; 3];
        let mut rhs = [C64::ZERO; 3];
        for i in 0..3 {
            let km = kpair(i + 1);
            f[i] = bil(&phi0, &km, &psi0);
            df[i] = bil(&phi1, &km, &psi0) + bil(&phi0, &km, &psi1);
            rhs[i] = bil(&nphi, &km, &psi0) + bil(&phi0, &km, &npsi);
        }
        let lhs = nabla_b(&df, &f, af, &av, gf, gv);
        for i in 0..3 {
            leib = leib.max((lhs[i] - rhs[i]).abs());
        }
    }
    // (ii) covariance under a local gauge transformation exp(i thF(x)) U(x), along one direction x of length t:
    // thF(t) = a0 + a1 t, U(t) = su2(b0 + b1 t), d(t) = d0 + d1 t + d2 t^2; derivatives by central differences (step h)
    let mut cov = 0.0f64;
    let h = 1e-5;
    for _ in 0..50 {
        let a0 = rng.unit();
        let a1 = rng.unit();
        let b0 = [rng.unit(), rng.unit(), rng.unit()];
        let b1 = [rng.unit(), rng.unit(), rng.unit()];
        let d0 = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
        let d1 = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
        let d2 = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
        let av = [rng.unit(), rng.unit(), rng.unit()];
        let af = rng.unit();
        let dat = |t: f64| -> [C64; 3] { [d0[0] + d1[0].scale(t) + d2[0].scale(t * t), d0[1] + d1[1].scale(t) + d2[1].scale(t * t), d0[2] + d1[2].scale(t) + d2[2].scale(t * t)] };
        let uat = |t: f64| su2(&[b0[0] + b1[0] * t, b0[1] + b1[1] * t, b0[2] + b1[2] * t]);
        let transformed = |t: f64| -> [C64; 3] {
            let r = adjoint(&uat(t));
            let d = dat(t);
            let ph = C64::cis(2.0 * (a0 + a1 * t));
            let mut out = [C64::ZERO; 3];
            for i in 0..3 {
                let mut z = C64::ZERO;
                for k in 0..3 {
                    z += d[k].scale(r[i][k]);
                }
                out[i] = ph * z;
            }
            out
        };
        // the transformed potentials at t = 0: AF' = AF + a1/g_F;  atau' = U atau U^-1 - (i/g_v) (dU) U^-1
        let atau = {
            let mut m = CMat::zeros(2);
            for i in 0..3 {
                m = m.add(&sigma(i + 1).scale_r(0.5 * av[i]));
            }
            m
        };
        let u0 = uat(0.0);
        let du = uat(h).sub(&uat(-h)).scale_r(0.5 / h);
        let ap = u0.mul(&atau).mul(&u0.dagger()).sub(&du.mul(&u0.dagger()).scale(c(0.0, 1.0 / gv)));
        let avp = [sigma(1).mul(&ap).trace().re, sigma(2).mul(&ap).trace().re, sigma(3).mul(&ap).trace().re];
        let afp = af + a1 / gf;
        // left: the covariant derivative of the transformed field with the transformed potentials
        let tp = transformed(h);
        let tm = transformed(-h);
        let dtr: [C64; 3] = [(tp[0] - tm[0]).scale(0.5 / h), (tp[1] - tm[1]).scale(0.5 / h), (tp[2] - tm[2]).scale(0.5 / h)];
        let lhs = nabla_b(&dtr, &transformed(0.0), afp, &avp, gf, gv);
        // right: the transformed covariant derivative
        let nb = nabla_b(&d1, &d0, af, &av, gf, gv);
        let r = adjoint(&u0);
        let ph = C64::cis(2.0 * a0);
        for i in 0..3 {
            let mut z = C64::ZERO;
            for k in 0..3 {
                z += nb[k].scale(r[i][k]);
            }
            cov = cov.max((lhs[i] - ph * z).abs());
        }
    }
    // (iii) the commutator of two covariant derivatives with potentials linear in the coordinates
    let mut commu = 0.0f64;
    for _ in 0..50 {
        // A_p(x) = a_p + sum_p' b[p][p'] x^p' (isospin and fermion number), d(x) quadratic
        let a: Vec<[f64; 3]> = (0..4).map(|_| [rng.unit(), rng.unit(), rng.unit()]).collect();
        let b: Vec<Vec<[f64; 3]>> = (0..4).map(|_| (0..4).map(|_| [rng.unit(), rng.unit(), rng.unit()]).collect()).collect();
        let afa: Vec<f64> = (0..4).map(|_| rng.unit()).collect();
        let afb: Vec<Vec<f64>> = (0..4).map(|_| (0..4).map(|_| rng.unit()).collect()).collect();
        let d0 = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
        let d1: Vec<[C64; 3]> = (0..4).map(|_| [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)]).collect();
        let mut d2 = vec![vec![[C64::ZERO; 3]; 4]; 4];
        for p in 0..4 {
            for q in p..4 {
                let v = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
                d2[p][q] = v;
                d2[q][p] = v;
            }
        }
        // nablaB_q d at 0, and its derivative along p at 0
        let nb_q = |q: usize| nabla_b(&d1[q], &d0, afa[q], &a[q], gf, gv);
        let d_p_nb_q = |p: usize, q: usize| -> [C64; 3] {
            let x1 = cross_c(&b[q][p], &d0);
            let x2 = cross_c(&a[q], &d1[p]);
            let mut r = [C64::ZERO; 3];
            for i in 0..3 {
                r[i] = d2[p][q][i] + (d0[i].scale(afb[q][p]) + d1[p][i].scale(afa[q])) * c(0.0, -2.0 * gf) + (x1[i] + x2[i]).scale(gv);
            }
            r
        };
        let nn = |p: usize, q: usize| -> [C64; 3] { nabla_b(&d_p_nb_q(p, q), &nb_q(q), afa[p], &a[p], gf, gv) };
        for p in 0..4 {
            for q in 0..4 {
                let l1 = nn(p, q);
                let l2 = nn(q, p);
                let ff = afb[q][p] - afb[p][q];
                let mut fv = [0.0; 3];
                for i in 0..3 {
                    fv[i] = b[q][p][i] - b[p][q][i];
                }
                let axa = [a[p][1] * a[q][2] - a[p][2] * a[q][1], a[p][2] * a[q][0] - a[p][0] * a[q][2], a[p][0] * a[q][1] - a[p][1] * a[q][0]];
                for i in 0..3 {
                    fv[i] += gv * axa[i];
                }
                let fx = cross_c(&fv, &d0);
                for i in 0..3 {
                    let want = d0[i] * c(0.0, -2.0 * gf * ff) + fx[i].scale(gv);
                    commu = commu.max((l1[i] - l2[i] - want).abs());
                }
            }
        }
    }
    let ok = leib < 1e-13 && cov < 1e-8 && commu < 1e-13;
    ch.add(
        "SB-6",
        "the gauge-covariant derivative of the SFBS, nablaB_p d = d_p d - 2i g_F AF_LL_p d + g_v (Av_LL_p x d): for 50 random linear test fields Psi_a, Psi_b, potentials and Lorentz connections, nablaB_p (Psi_a^T Kch_iota Psi_b) = (nabla_LL_p Psi_a)^T Kch_iota Psi_b + Psi_a^T Kch_iota (nabla_LL_p Psi_b) (tol 1e-13); for 50 random local transformations exp(i thF) U and quadratic fields, the covariant derivative of the transformed field with the transformed potentials equals exp(2i thF) Rv(U) times the covariant derivative (central differences, tol 1e-8); for 50 random linear potentials, [nablaB_p, nablaB_p'] d = -2i g_F FF_LL_pp' d + g_v Fv_LL_pp' x d for all 16 pairs (p, p') (tol 1e-13)",
        ok,
        format!("largest deviations: Leibniz rule {}, covariance {}, commutator {}", sci(leib), sci(cov), sci(commu)),
    );
    // fragments: the matrix of the isospin potential in the vector representation and the covariant derivative written out
    let mut am = SymMat::zeros(3, 3);
    let avt = [0.37, -0.61, 0.29];
    for i in 0..3 {
        let mut e = [0.0; 3];
        e[i] = 1.0;
        let mm = avec(&e);
        let mut cm = CMat::zeros(3);
        for r in 0..3 {
            for cc in 0..3 {
                cm.set(r, cc, c(mm[r][cc], 0.0));
            }
        }
        am.add_scaled(&cm, &Lin::mono(&format!("A^{}_{{{{\\rm LL}},p}}", i + 1), c(avt[i], 0.0)));
    }
    let mut dev = 0.0f64;
    let want = avec(&avt);
    for r in 0..3 {
        for cc in 0..3 {
            dev = dev.max((am.get(r, cc).eval() - c(want[r][cc], 0.0)).abs());
        }
    }
    assert!(dev < 1e-15, "printed matrix of the isospin potential");
    fr.add("t2s_Avec", format!("$$\\mathcal A^{{\\rm v}}_{{{{\\rm LL}},p}}={}$$", am.tex()));
}

/// the integrand weight: integral over |q| < Lambda of d^3q/(2 pi)^3 of a function of (|q|, cos(angle between q and k)):
/// composite Gauss-Legendre nodes in |q| (panels x points) and in the cosine; returns (q, cos, weight) triples
fn sphere_nodes(panels: usize, nq: usize, nc: usize, cuts: &[f64]) -> Vec<(f64, f64, f64)> {
    let (qx, qw) = quad::composite(0.0, LAMBDA, cuts, panels, nq);
    let (cx, cw) = quad::gauss_legendre(nc);
    let mut v = vec![];
    for (q, wq) in qx.iter().zip(&qw) {
        for (ct, wc) in cx.iter().zip(&cw) {
            v.push((*q, *ct, wq * wc * q * q / (4.0 * PI * PI)));
        }
    }
    v
}

/// the pair susceptibility of the normal vacuum (mu = 0, mass m) at the frequency omega and the wave number k (along x^3):
/// chi_pr = int d^3q/(2 pi)^3 M_pr (eps(+) + eps(-))/((eps(+) + eps(-))^2 - omega^2), M_pr = 2 (1 + (p1.p2 + m^2)/(eps(+) eps(-))),
/// p1 = q + k/2, p2 = q - k/2 (the cutoff acts on the relative momentum q)
fn chi_pr(omega: f64, k: f64, m: f64, nodes: &[(f64, f64, f64)]) -> f64 {
    let mut s = 0.0;
    for &(q, ct, w) in nodes {
        let st = (1.0 - ct * ct).max(0.0).sqrt();
        let p1 = [q * st, 0.0, q * ct + 0.5 * k];
        let p2 = [q * st, 0.0, q * ct - 0.5 * k];
        let e1 = (p1[0] * p1[0] + p1[2] * p1[2] + m * m).sqrt();
        let e2 = (p2[0] * p2[0] + p2[2] * p2[2] + m * m).sqrt();
        let dot = p1[0] * p2[0] + p1[2] * p2[2];
        let mm = 2.0 * (1.0 + (dot + m * m) / (e1 * e2));
        let sum = e1 + e2;
        s += w * mm * sum / (sum * sum - omega * omega);
    }
    s
}

/// the same susceptibility from the Bogoliubov matrices: the pair field d exp(i k x^3) (direction 3, small d) at the
/// chemical potential omega/2 (a pair field that rotates with the frequency omega is gauge-equivalent to a static one at
/// the chemical potential shifted by omega/2); chi = (sum of the positive eigenvalues with d - without d)/(2 d^2),
/// integrated; Richardson extrapolation from d and 2d removes the term of order d^4
fn chi_bdg(omega: f64, k: f64, m: f64, d: f64, nodes: &[(f64, f64, f64)]) -> f64 {
    let mu = 0.5 * omega;
    let k3 = kpair(3);
    let pos_sum = |q: f64, ct: f64, dd: f64| -> f64 {
        let st = (1.0 - ct * ct).max(0.0).sqrt();
        let p1 = [q * st, 0.0, q * ct + 0.5 * k];
        let mp2 = [-q * st, 0.0, -(q * ct - 0.5 * k)];
        let hp = h8(&p1, m).sub(&CMat::identity(8).scale_r(mu));
        let hh = h8(&mp2, m).conj().sub(&CMat::identity(8).scale_r(mu)).scale_r(-1.0);
        let mut big = CMat::zeros(16);
        big.put(0, 0, &hp);
        big.put(0, 8, &k3.dagger().scale_r(dd));
        big.put(8, 0, &k3.scale_r(dd));
        big.put(8, 8, &hh);
        let (ev, _) = eigh(&big);
        ev.iter().filter(|&&e| e > 0.0).sum()
    };
    let mut s1 = 0.0;
    let mut s2 = 0.0;
    for &(q, ct, w) in nodes {
        let base = pos_sum(q, ct, 0.0);
        s1 += w * (pos_sum(q, ct, d) - base);
        s2 += w * (pos_sum(q, ct, 2.0 * d) - base);
    }
    let c1 = s1 / (2.0 * d * d);
    let c2 = s2 / (8.0 * d * d);
    (4.0 * c1 - c2) / 3.0
}

/// 1/G_crit = (Lambda eps_Lambda - m^2 arsinh(Lambda/m))/(2 pi^2) at mu = 0
fn inv_gcrit(m: f64) -> f64 {
    (LAMBDA * (LAMBDA * LAMBDA + m * m).sqrt() - m * m * (LAMBDA / m).asinh()) / (2.0 * PI * PI)
}

/// Z_t(M) = (1/2) int_{q<Lambda} d^3q/(2 pi)^3 (q^2 + M^2)^(-3/2) = (arsinh(Lambda/M) - Lambda/(Lambda^2 + M^2)^(1/2))/(4 pi^2)
fn z_t(mm: f64) -> f64 {
    ((LAMBDA / mm).asinh() - LAMBDA / (LAMBDA * LAMBDA + mm * mm).sqrt()) / (4.0 * PI * PI)
}

/// Z_s0(m) = Z_t(m) + Lambda^3/(24 pi^2 eps_Lambda^3): the coefficient of k^2 of the normal-state susceptibility
fn z_s0(m: f64) -> f64 {
    let el = (LAMBDA * LAMBDA + m * m).sqrt();
    z_t(m) + LAMBDA.powi(3) / (24.0 * PI * PI * el.powi(3))
}

/// 1/G_bind = chi_pr(2m, 0) = (Lambda eps_Lambda + m^2 arsinh(Lambda/m))/(2 pi^2)
fn inv_gbind(m: f64) -> f64 {
    (LAMBDA * (LAMBDA * LAMBDA + m * m).sqrt() + m * m * (LAMBDA / m).asinh()) / (2.0 * PI * PI)
}

/// the pole of the SFBS propagator of the normal vacuum: the frequency in (0, 2m) where chi_pr(omega, k) = 1/G
/// (chi_pr increases with omega there); None if there is none
fn boson_pole(g: f64, k: f64, m: f64, nodes: &[(f64, f64, f64)]) -> Option<f64> {
    let top = 2.0 * (m * m + 0.25 * k * k).sqrt(); // the two-particle threshold at the wave number k
    let f = |w: f64| chi_pr(w, k, m, nodes) - 1.0 / g;
    let (mut lo, mut hi) = (0.0, top * (1.0 - 1e-12));
    if f(lo) > 0.0 || f(hi) < 0.0 {
        return None;
    }
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if f(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(0.5 * (lo + hi))
}

/// SB-7: the pair susceptibility of the normal vacuum
fn sb7(ch: &mut Checks, dir: &PathBuf) {
    let m = MASS;
    let nodes = sphere_nodes(40, 16, 32, &[]);
    // the formula against the Bogoliubov matrices at several (omega, k)
    let pts: [(f64, f64); 6] = [(0.0, 0.0), (0.0, 0.3), (0.0, 0.8), (0.25, 0.0), (0.3, 0.4), (0.38, 0.1)];
    let coarse = sphere_nodes(20, 12, 16, &[]);
    let mut worst = 0.0f64;
    let mut lines = vec![];
    for &(w, k) in &pts {
        let a = chi_pr(w, k, m, &coarse);
        let b = chi_bdg(w, k, m, 1e-3, &coarse);
        worst = worst.max(((a - b) / a).abs());
        lines.push(format!("({}, {}): {:.10} against {:.10}", w, k, a, b));
    }
    // chi_pr(0, 0) = 1/G_crit, and the coefficients of omega^2 and k^2
    let c00 = chi_pr(0.0, 0.0, m, &nodes);
    let crit = ((c00 - inv_gcrit(m)) / inv_gcrit(m)).abs();
    let h = 0.01;
    let zt_num = (4.0 * (chi_pr(h, 0.0, m, &nodes) - c00) / (h * h) - (chi_pr(2.0 * h, 0.0, m, &nodes) - c00) / (4.0 * h * h)) / 3.0;
    let zs_num = -(4.0 * (chi_pr(0.0, h, m, &nodes) - c00) / (h * h) - (chi_pr(0.0, 2.0 * h, m, &nodes) - c00) / (4.0 * h * h)) / 3.0;
    let zt = z_t(m);
    let zs = z_s0(m);
    let ezt = ((zt_num - zt) / zt).abs();
    let ezs = ((zs_num - zs) / zs).abs();
    let ok = worst < 1e-7 && crit < 1e-10 && ezt < 1e-6 && ezs < 1e-6;
    ch.add(
        "SB-7",
        "the pair susceptibility of the normal vacuum (m = 0.2, mu = 0, Lambda = 1), chi_pr(omega, k) = int d^3q/(2 pi)^3 M_pr (eps(+) + eps(-))/((eps(+) + eps(-))^2 - omega^2), M_pr = 2 (1 + (q^2 - k^2/4 + m^2)/(eps(+) eps(-))), eps(+-) = (|q +- k/2|^2 + m^2)^(1/2): against the second-order change of the sum of the positive Bogoliubov energies of the 16 x 16 matrices with the pair field d exp(i k x^3) at the chemical potential omega/2 (d = 1e-3 and 2e-3, Richardson extrapolation) at six points (omega, k) (relative tol 1e-7); chi_pr(0, 0) = 1/G_crit (tol 1e-10); the coefficient of omega^2 is Z_t0 = (arsinh(Lambda/m) - Lambda/eps_Lambda)/(4 pi^2) and minus the coefficient of k^2 is Z_s0 = Z_t0 + Lambda^3/(24 pi^2 eps_Lambda^3) (numerical second derivatives with Richardson extrapolation, relative tol 1e-6)",
        ok,
        format!(
            "largest relative deviation formula/Bogoliubov {}; |chi_pr(0,0) G_crit - 1| {}; Z_t0 = {:.10} (numerical {:.10}), Z_s0 = {:.10} (numerical {:.10}); points: {}",
            sci(worst),
            sci(crit),
            zt,
            zt_num,
            zs,
            zs_num,
            lines.join("; ")
        ),
    );
    // data of Figure 22(a): the susceptibility against omega^2 and against k^2, with its quadratic approximations
    let mut csv = Csv::new(
        "the pair susceptibility chi_pr of the normal vacuum as a function of the frequency (at k = 0) and of the wave number (at omega = 0), the formula and its Ginzburg-Landau approximation 1/G_crit + Z_t0 omega^2 - Z_s0 k^2 (Test 2, item 8, Figure 22(a), lines), and the values from the Bogoliubov matrices (points)",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G_crit", format!("{:.10}", 1.0 / inv_gcrit(m))), ("Z_t0", format!("{:.10}", zt)), ("Z_s0", format!("{:.10}", zs))],
        "variable,omega_B,k_B,chi_pr,chi_pr_GL,source",
    );
    for i in 0..=60 {
        let w = 0.39 * i as f64 / 60.0;
        csv.row(&["omega".into(), format!("{:.6}", w), "0".into(), format!("{:.12}", chi_pr(w, 0.0, m, &nodes)), format!("{:.12}", inv_gcrit(m) + zt * w * w), "formula".into()]);
    }
    for i in 0..=60 {
        let k = 1.0 * i as f64 / 60.0;
        csv.row(&["k".into(), "0".into(), format!("{:.6}", k), format!("{:.12}", chi_pr(0.0, k, m, &nodes)), format!("{:.12}", inv_gcrit(m) - zs * k * k), "formula".into()]);
    }
    for &w in &[0.1, 0.2, 0.3, 0.37] {
        csv.row(&["omega".into(), format!("{}", w), "0".into(), format!("{:.12}", chi_bdg(w, 0.0, m, 1e-3, &coarse)), "".into(), "Bogoliubov".into()]);
    }
    for &k in &[0.25, 0.5, 0.75, 1.0] {
        csv.row(&["k".into(), "0".into(), format!("{}", k), format!("{:.12}", chi_bdg(0.0, k, m, 1e-3, &coarse)), "".into(), "Bogoliubov".into()]);
    }
    csv.write(dir, "t2_sfbs_susceptibility.csv");
}

/// SB-8: the SFBS boson of the normal vacuum
fn sb8(ch: &mut Checks, dir: &PathBuf) {
    let m = MASS;
    let nodes = sphere_nodes(40, 16, 32, &[]);
    let gcrit = 1.0 / inv_gcrit(m);
    let gbind = 1.0 / inv_gbind(m);
    let zt = z_t(m);
    let zs = z_s0(m);
    // chi_pr(2m, 0) by quadrature against the closed form 1/G_bind
    let cb = chi_pr(2.0 * m * (1.0 - 1e-13), 0.0, m, &nodes);
    let ebind = ((cb - inv_gbind(m)) / inv_gbind(m)).abs();
    // the mass against the coupling
    let mut csv = Csv::new(
        "the mass of the SFBS boson of the normal vacuum (the pole of its propagator, chi_pr(M_B, 0) = 1/G) and its Ginzburg-Landau approximation ((1/G - 1/G_crit)/Z_t0)^(1/2), as functions of the coupling between G_bind and G_crit (Test 2, item 8, Figure 22(b))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G_crit", format!("{:.10}", gcrit)), ("G_bind", format!("{:.10}", gbind)), ("Z_t0", format!("{:.10}", zt))],
        "G,M_B,M_B_GL",
    );
    let mut near = 0.0f64;
    let mut prev = f64::INFINITY;
    let mut monotone = true;
    for i in 1..=80 {
        // G = G_crit - (G_crit - G_bind)(1 - u)^2: dense near G_crit, where M_B ~ (G_crit - G)^(1/2)
        let u = i as f64 / 80.0;
        let g = (gcrit - (gcrit - gbind) * (1.0 - u) * (1.0 - u)).min(gcrit * (1.0 - 1e-9));
        let mb = boson_pole(g, 0.0, m, &nodes).unwrap_or(f64::NAN);
        let gl = ((1.0 / g - 1.0 / gcrit) / zt).max(0.0).sqrt();
        if !(mb < prev) {
            monotone = false;
        }
        prev = mb;
        csv.row(&[format!("{:.8}", g), format!("{:.12}", mb), format!("{:.12}", gl)]);
    }
    csv.write(dir, "t2_sfbs_boson_mass.csv");
    // near the critical coupling the Ginzburg-Landau mass is exact to leading order: relative error at small a_GL
    let mut rel = vec![];
    for &eps in &[1e-2, 1e-3, 1e-4] {
        let g = gcrit * (1.0 - eps);
        let mb = boson_pole(g, 0.0, m, &nodes).unwrap();
        let gl = ((1.0 / g - 1.0 / gcrit) / zt).sqrt();
        rel.push(((mb - gl) / gl).abs());
    }
    near = near.max(rel[2]);
    let lim_bind = boson_pole(gbind * (1.0 + 1e-6), 0.0, m, &nodes).unwrap_or(f64::NAN);
    let lim_crit = boson_pole(gcrit * (1.0 - 1e-8), 0.0, m, &nodes).unwrap_or(f64::NAN);
    // the dispersion relation at G = 21 (near the critical coupling) and at G = 20: the pole at the wave number k against
    // the Klein-Gordon form omega^2 = (a_GL + Z_s0 k^2)/Z_t0 of the effective field equation
    let mut csv2 = Csv::new(
        "the dispersion relation of the SFBS boson of the normal vacuum at G = 21 and G = 20: the pole of its propagator at the wave number k (the frequency where chi_pr(omega, k) = 1/G), the Klein-Gordon form ((a_GL + Z_s0 k^2)/Z_t0)^(1/2) of the effective field equation with a_GL = 1/G - 1/G_crit, and the threshold 2 (m^2 + k^2/4)^(1/2) of two free spinor charges (Test 2, item 8, Figure 22(c))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("Z_t0", format!("{:.10}", zt)), ("Z_s0", format!("{:.10}", zs))],
        "G,k_B,omega_B,omega_B_KG,threshold",
    );
    for &gd in &[21.0, 20.0] {
        let a_gl = 1.0 / gd - 1.0 / gcrit;
        for i in 0..=40 {
            let k = 0.8 * i as f64 / 40.0;
            let w = boson_pole(gd, k, m, &nodes).unwrap_or(f64::NAN);
            let kg = ((a_gl + zs * k * k) / zt).sqrt();
            csv2.row(&[format!("{}", gd), format!("{:.6}", k), format!("{:.12}", w), format!("{:.12}", kg), format!("{:.12}", 2.0 * (m * m + 0.25 * k * k).sqrt())]);
        }
    }
    csv2.write(dir, "t2_sfbs_boson_dispersion.csv");
    // the small-k slope of M_B(k)^2 at G = 20 from the poles, against the exact slope -(dchi/dk^2)/(dchi/domega^2) at the
    // pole and against the Ginzburg-Landau value Z_s0/Z_t0
    let g = 20.0;
    let hk = 0.02;
    let w0 = boson_pole(g, 0.0, m, &nodes).unwrap();
    let w1 = boson_pole(g, hk, m, &nodes).unwrap();
    let w2 = boson_pole(g, 2.0 * hk, m, &nodes).unwrap();
    let slope = (4.0 * (w1 * w1 - w0 * w0) / (hk * hk) - (w2 * w2 - w0 * w0) / (4.0 * hk * hk)) / 3.0;
    let c0 = chi_pr(w0, 0.0, m, &nodes);
    let hw = 1e-4;
    let dchi_dw2 = (chi_pr((w0 * w0 + hw).sqrt(), 0.0, m, &nodes) - chi_pr((w0 * w0 - hw).sqrt(), 0.0, m, &nodes)) / (2.0 * hw);
    let hq = 0.01;
    let dchi_dk2 = (4.0 * (chi_pr(w0, hq, m, &nodes) - c0) / (hq * hq) - (chi_pr(w0, 2.0 * hq, m, &nodes) - c0) / (4.0 * hq * hq)) / 3.0;
    let slope_exact = -dchi_dk2 / dchi_dw2;
    // at the pole, the susceptibility from the Bogoliubov matrices also equals 1/G
    let coarse = sphere_nodes(20, 12, 16, &[]);
    let pole_bdg = (chi_bdg(w0, 0.0, m, 1e-3, &coarse) * g - 1.0).abs();
    let c_kg = zs / zt;
    let eslope = ((slope - slope_exact) / slope_exact).abs();
    let ok = ebind < 1e-8 && monotone && (lim_bind - 2.0 * m).abs() < 1e-3 && lim_crit < 1e-2 && rel[0] > rel[1] && rel[1] > rel[2] && near < 2e-3 && pole_bdg < 1e-6 && eslope < 1e-4;
    ch.add(
        "SB-8",
        "the SFBS boson of the normal vacuum (m = 0.2, mu = 0, Lambda = 1): chi_pr(2m, 0) by quadrature equals 1/G_bind = (Lambda eps_Lambda + m^2 arsinh(Lambda/m))/(2 pi^2) (tol 1e-8); for G_bind < G < G_crit the propagator has one pole M_B in (0, 2m), decreasing with G, with M_B -> 2m at G_bind and M_B -> 0 at G_crit; the Ginzburg-Landau mass ((1/G - 1/G_crit)/Z_t0)^(1/2) approaches it as G -> G_crit (relative errors decreasing, below 2e-3 at 1 - G/G_crit = 1e-4); at G = 20 the Bogoliubov matrices give chi(M_B, 0) = 1/G (tol 1e-6), and the small-k slope of M_B(k)^2 from the poles equals -(dchi_pr/dk^2)/(dchi_pr/domega^2) at the pole (relative tol 1e-4; the Ginzburg-Landau value Z_s0/Z_t0 is reported for comparison)",
        ok,
        format!(
            "G_bind = {:.6}, G_crit = {:.6}; |chi_pr(2m,0) G_bind - 1| {}; M_B at G_bind(1 + 1e-6): {:.6}; at G_crit(1 - 1e-8): {:.3e}; relative error of the GL mass at 1 - G/G_crit = 1e-2, 1e-3, 1e-4: {}, {}, {}; at G = 20: M_B = {:.8}, |chi_Bogoliubov(M_B) G - 1| {}, slope of omega^2 against k^2 {:.6} (from the poles) and {:.6} (from the derivatives at the pole), Ginzburg-Landau Z_s0/Z_t0 = {:.6}",
            gbind,
            gcrit,
            sci(ebind),
            lim_bind,
            lim_crit,
            sci(rel[0]),
            sci(rel[1]),
            sci(rel[2]),
            w0,
            sci(pole_bdg),
            slope,
            slope_exact,
            c_kg
        ),
    );
}

/// SB-9: the currents of the SFBS, the sources of the gauge fields: the derivatives of the kinetic Lagrangian
/// L_B = Z_t |nablaB_0 d|^2 - Z_s sum_j |nablaB_j d|^2 with respect to the potentials
fn sb9(ch: &mut Checks) {
    let mut rng = Rng(4242);
    let (gv, gf) = (0.83, 0.57);
    let (zt, zs) = (0.31, 0.37);
    let mut worst = 0.0f64;
    for _ in 0..50 {
        let d = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
        let dd: Vec<[C64; 3]> = (0..4).map(|_| [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)]).collect();
        let av: Vec<[f64; 3]> = (0..4).map(|_| [rng.unit(), rng.unit(), rng.unit()]).collect();
        let af: Vec<f64> = (0..4).map(|_| rng.unit()).collect();
        let lag = |av: &Vec<[f64; 3]>, af: &Vec<f64>| -> f64 {
            let mut l = 0.0;
            for p in 0..4 {
                let nb = nabla_b(&dd[p], &d, af[p], &av[p], gf, gv);
                let sq: f64 = nb.iter().map(|z| z.norm_sqr()).sum();
                l += if p == 0 { zt * sq } else { -zs * sq };
            }
            l
        };
        let h = 1e-6;
        for p in 0..4 {
            let nb = nabla_b(&dd[p], &d, af[p], &av[p], gf, gv);
            let zp = if p == 0 { zt } else { -zs };
            // the formulas: dL/dAv^iota_p = 2 Z g_v Re[(d^* x nablaB_p d)^iota], dL/dAF_p = -4 Z g_F Im(d^* . nablaB_p d) (Z with its sign)
            let dstar = [d[0].conj(), d[1].conj(), d[2].conj()];
            let cr = [
                dstar[1] * nb[2] - dstar[2] * nb[1],
                dstar[2] * nb[0] - dstar[0] * nb[2],
                dstar[0] * nb[1] - dstar[1] * nb[0],
            ];
            let dot = dstar.iter().zip(&nb).fold(C64::ZERO, |s, (a, b)| s + *a * *b);
            for i in 0..3 {
                let mut ap = av.clone();
                let mut am = av.clone();
                ap[p][i] += h;
                am[p][i] -= h;
                let num = (lag(&ap, &af) - lag(&am, &af)) / (2.0 * h);
                let formula = 2.0 * zp * gv * cr[i].re;
                worst = worst.max((num - formula).abs());
            }
            let mut fp = af.clone();
            let mut fm = af.clone();
            fp[p] += h;
            fm[p] -= h;
            let num = (lag(&av, &fp) - lag(&av, &fm)) / (2.0 * h);
            let formula = -4.0 * zp * gf * dot.im;
            worst = worst.max((num - formula).abs());
        }
    }
    let ok = worst < 1e-7;
    ch.add(
        "SB-9",
        "the currents of the SFBS: for 50 random fields, derivatives and potentials, the derivatives of L_B = Z_t |nablaB_0 d|^2 - Z_s sum_j |nablaB_j d|^2 with respect to Av_LL^iota_p and AF_LL_p (central differences) equal 2 Z g_v Re[(d^* x nablaB_p d)^iota] and -4 Z g_F Im(d^* . nablaB_p d), with Z = Z_t for p = 0 and -Z_s for p = 1, 2, 3 (tol 1e-7)",
        ok,
        format!("largest deviation {}", sci(worst)),
    );
    let _ = eps3(0, 1, 2);
}

/// the fragments of item [8]: the components of the covariant derivative of the SFBS and of its effective field
/// equation, generated as linear combinations of symbols and compared numerically with the vector formulas
fn frags_item8(fr: &mut Frags) {
    let mut rng = Rng(1789);
    let (gv, gf) = (0.83, 0.57);
    let d = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
    let dd = [rnd_c(&mut rng), rnd_c(&mut rng), rnd_c(&mut rng)];
    let av = [rng.unit(), rng.unit(), rng.unit()];
    let af = rng.unit();
    let want = nabla_b(&dd, &d, af, &av, gf, gv);
    // (nablaB_p d)^iota = d_p d^iota - 2i g_F AF_p d^iota + g_v sum eps_{iota kappa lambda} A^kappa_p d^lambda
    let mut lines = vec![];
    for i in 0..3 {
        let mut l = Lin::mono(&format!("\\partial_pd^{}_{{\\rm LL}}", i + 1), dd[i]);
        l.add_term(&format!("g^{{\\rm F}}_{{\\rm LL}}A^{{\\rm F}}_{{{{\\rm LL}},p}}d^{}_{{\\rm LL}}", i + 1), GR::imag(-2, 1), d[i].scale(gf * af));
        for k in 0..3 {
            for j in 0..3 {
                let e = eps3(i, k, j);
                if e != 0.0 {
                    l.add_term(&format!("g^{{\\rm v}}_{{\\rm LL}}A^{}_{{{{\\rm LL}},p}}d^{}_{{\\rm LL}}", k + 1, j + 1), GR::int(e as i64), d[j].scale(gv * av[k]));
                }
            }
        }
        assert!((l.eval() - want[i]).abs() < 1e-14, "printed component of the covariant derivative");
        lines.push(format!("(\\nabla^{{\\rm B}}_{{{{\\rm LL}},p}}\\vec d_{{\\rm LL}})^{}&={}", i + 1, l.tex()));
    }
    fr.add("t2s_nablaB", format!("$$\\begin{{aligned}}{}\\end{{aligned}}$$", lines.join("\\\\\n")));
    // the components of the field equation: Z_t nablaB_0 (nablaB_0 d) - Z_s sum_j nablaB_j (nablaB_j d) + dOmega/dd^* = 0,
    // written with the components (nablaB_p d)^iota (symbols) of the first covariant derivative: the second covariant
    // derivative applies the rule of t2s_nablaB to the vector nablaB_p d
    let zt = 0.31;
    let zs = 0.37;
    let mut lhs_lines = vec![];
    for i in 0..3 {
        let mut rng2 = Rng(77 + i as u64);
        let mut parts = vec![];
        let mut total = C64::ZERO;
        let mut formula = C64::ZERO;
        for (p, pidx) in [(0usize, "0"), (1usize, "j")] {
            // numerical test data for the direction p: the vector v = nablaB_p d, its derivative dv, the potentials
            let v = [rnd_c(&mut rng2), rnd_c(&mut rng2), rnd_c(&mut rng2)];
            let dv = [rnd_c(&mut rng2), rnd_c(&mut rng2), rnd_c(&mut rng2)];
            let ap = [rng2.unit(), rng2.unit(), rng2.unit()];
            let afp = rng2.unit();
            let nn = nabla_b(&dv, &v, afp, &ap, gf, gv);
            let vname = |c: usize| format!("(\\nabla^{{\\rm B}}_{{{{\\rm LL}},{}}}\\vec d_{{\\rm LL}})^{}", pidx, c + 1);
            let mut l = Lin::mono(&format!("\\partial_{}{}", pidx, vname(i)), dv[i]);
            l.add_term(&format!("g^{{\\rm F}}_{{\\rm LL}}A^{{\\rm F}}_{{{{\\rm LL}},{}}}{}", pidx, vname(i)), GR::imag(-2, 1), v[i].scale(gf * afp));
            for k in 0..3 {
                for j in 0..3 {
                    let e = eps3(i, k, j);
                    if e != 0.0 {
                        l.add_term(&format!("g^{{\\rm v}}_{{\\rm LL}}A^{}_{{{{\\rm LL}},{}}}{}", k + 1, pidx, vname(j)), GR::int(e as i64), v[j].scale(gv * ap[k]));
                    }
                }
            }
            let zp = if p == 0 { zt } else { -zs };
            total += nn[i].scale(zp);
            formula += l.eval().scale(zp);
            parts.push(l.tex_broken(2));
        }
        assert!((formula - total).abs() < 1e-13, "printed component of the field equation");
        lhs_lines.push(format!(
            "&Z^{{\\rm t}}_{{\\rm LL}}\\big[{}\\big]\\\\\n&\\quad-Z^{{\\rm s}}_{{\\rm LL}}\\sum_j\\big[{}\\big]\\\\\n&\\quad+\\frac{{\\partial\\Omega_{{\\rm LL}}}}{{\\partial(d^{}_{{\\rm LL}})^*}}=0",
            parts[0],
            parts[1],
            i + 1
        ));
    }
    fr.add("t2s_fieldeq", format!("$$\\begin{{aligned}}{}\\end{{aligned}}$$", lhs_lines.join("\\\\[4pt]\n")));
}

/// the uniform time-dependent mean-field (Bogoliubov-de Gennes) evolution of the polar pair field d^3(t) = d(t) without
/// gauge fields: for each momentum shell (node q_i with the weight w_i of int d^3q/(2 pi)^3) and each sign s = +-1 of the
/// energy s eps, the mode (u, v) in the plane of (3.12) obeys i d/dt (u, v) = [[xi, d], [d^*, -xi]] (u, v), xi = s eps - mu,
/// and d(t) = 2G sum_i w_i sum_s u_s v_s^* (the four degenerate modes of each plane included); the start is the
/// positive-energy mode of the matrix with the trial gap delta_trial (real). RK4 with the step dt. Returns the pair field
/// at every step (including t = 0) and the largest relative change of the conserved mean-field energy per volume
/// E = sum_i w_i sum_s 4 xi |v|^2 - |d|^2/G (|v|^2 is the occupation of the state of energy s eps; relative to the scale
/// sum_i w_i sum_s 4 |xi|).
fn uniform_tdbdg(g: f64, m: f64, mu: f64, delta_trial: f64, qn: &[f64], qw: &[f64], dt: f64, nsteps: usize) -> (Vec<C64>, f64) {
    let n = qn.len();
    let xi: Vec<[f64; 2]> = qn.iter().map(|&q| {
        let e = (q * q + m * m).sqrt();
        [e - mu, -e - mu]
    }).collect();
    // state y = [u_+, v_+, u_-, v_-] for each node
    let mut y = vec![C64::ZERO; 4 * n];
    for i in 0..n {
        for s in 0..2 {
            let x = xi[i][s];
            let e = (x * x + delta_trial * delta_trial).sqrt();
            y[4 * i + 2 * s] = c(((1.0 + x / e) / 2.0).sqrt(), 0.0);
            y[4 * i + 2 * s + 1] = c(((1.0 - x / e) / 2.0).sqrt() * delta_trial.signum(), 0.0);
        }
    }
    let pair = |y: &[C64]| -> C64 {
        let mut s = C64::ZERO;
        for i in 0..n {
            s += (y[4 * i] * y[4 * i + 1].conj() + y[4 * i + 2] * y[4 * i + 3].conj()).scale(qw[i]);
        }
        s.scale(2.0 * g)
    };
    let functional = |y: &[C64]| -> (f64, f64) {
        let mut kin = 0.0;
        let mut scale = 0.0;
        for i in 0..n {
            for s in 0..2 {
                kin += 4.0 * qw[i] * xi[i][s] * y[4 * i + 2 * s + 1].norm_sqr();
                scale += 4.0 * qw[i] * xi[i][s].abs();
            }
        }
        let d = pair(y);
        (kin - d.norm_sqr() / g, scale)
    };
    let deriv = |y: &[C64]| -> Vec<C64> {
        let d = pair(y);
        let mut r = vec![C64::ZERO; 4 * n];
        for i in 0..n {
            for s in 0..2 {
                let (u, v) = (y[4 * i + 2 * s], y[4 * i + 2 * s + 1]);
                let x = xi[i][s];
                // -i H (u, v)
                r[4 * i + 2 * s] = (u.scale(x) + d * v) * c(0.0, -1.0);
                r[4 * i + 2 * s + 1] = (u * d.conj() - v.scale(x)) * c(0.0, -1.0);
            }
        }
        r
    };
    let (f0, fscale) = functional(&y);
    let mut out = vec![pair(&y)];
    let mut fdev = 0.0f64;
    for step in 0..nsteps {
        let k1 = deriv(&y);
        let y2: Vec<C64> = y.iter().zip(&k1).map(|(a, b)| *a + b.scale(0.5 * dt)).collect();
        let k2 = deriv(&y2);
        let y3: Vec<C64> = y.iter().zip(&k2).map(|(a, b)| *a + b.scale(0.5 * dt)).collect();
        let k3 = deriv(&y3);
        let y4: Vec<C64> = y.iter().zip(&k3).map(|(a, b)| *a + b.scale(dt)).collect();
        let k4 = deriv(&y4);
        for j in 0..4 * n {
            y[j] += (k1[j] + k2[j].scale(2.0) + k3[j].scale(2.0) + k4[j]).scale(dt / 6.0);
        }
        out.push(pair(&y));
        if step % 100 == 99 || step + 1 == nsteps {
            fdev = fdev.max(((functional(&y).0 - f0) / fscale).abs());
        }
    }
    (out, fdev)
}

/// the frequency of the strongest spectral line of a sampled complex signal in the interval (wlo, whi): the maximum over
/// omega > 0 of |sum_n h_n x_n exp(i omega t_n)|^2 + |sum_n h_n x_n exp(-i omega t_n)|^2 with the Hann window h_n (the mean
/// removed), found by a scan with the step dw and a golden-section refinement
fn line_frequency(x: &[C64], dt: f64, wlo: f64, whi: f64, dw: f64) -> f64 {
    let nn = x.len();
    let mean = x.iter().fold(C64::ZERO, |s, z| s + *z).scale(1.0 / nn as f64);
    let hx: Vec<C64> = x.iter().enumerate().map(|(i, z)| (*z - mean).scale(0.5 - 0.5 * (2.0 * PI * i as f64 / (nn - 1) as f64).cos())).collect();
    let power = |w: f64| -> f64 {
        let mut a = C64::ZERO;
        let mut b = C64::ZERO;
        for (i, z) in hx.iter().enumerate() {
            let ph = C64::cis(w * dt * i as f64);
            a += *z * ph;
            b += *z * ph.conj();
        }
        a.norm_sqr() + b.norm_sqr()
    };
    let mut best = (wlo, -1.0);
    let mut w = wlo;
    while w <= whi {
        let pw = power(w);
        if pw > best.1 {
            best = (w, pw);
        }
        w += dw;
    }
    let (mut a, mut b) = (best.0 - dw, best.0 + dw);
    let gr = (5f64.sqrt() - 1.0) / 2.0;
    let mut x1 = b - gr * (b - a);
    let mut x2 = a + gr * (b - a);
    let (mut f1, mut f2) = (power(x1), power(x2));
    for _ in 0..60 {
        if f1 > f2 {
            b = x2;
            x2 = x1;
            f2 = f1;
            x1 = b - gr * (b - a);
            f1 = power(x1);
        } else {
            a = x1;
            x1 = x2;
            f1 = f2;
            x2 = a + gr * (b - a);
            f2 = power(x2);
        }
    }
    0.5 * (a + b)
}

/// SB-10: the microscopic time evolution of a small uniform pair field in the normal vacuum
fn sb10(ch: &mut Checks, dir: &PathBuf) {
    let m = MASS;
    let g = 20.0;
    let (qn, qw0) = quad::composite(0.0, LAMBDA, &[], 40, 8);
    let qw: Vec<f64> = qn.iter().zip(&qw0).map(|(q, w)| w * q * q / (2.0 * PI * PI)).collect();
    // the pole of the propagator with the same radial nodes: chi_pr(omega, 0) = sum w 8 eps/(4 eps^2 - omega^2)
    let chi_k0 = |w: f64| -> f64 { qn.iter().zip(&qw).map(|(q, wt)| { let e = (q * q + m * m).sqrt(); wt * 8.0 * e / (4.0 * e * e - w * w) }).sum() };
    let (mut lo, mut hi) = (0.0, 2.0 * m * (1.0 - 1e-12));
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        if chi_k0(mid) < 1.0 / g {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let mb_nodes = 0.5 * (lo + hi);
    let nodes3 = sphere_nodes(40, 16, 32, &[]);
    let mb = boson_pole(g, 0.0, m, &nodes3).unwrap();
    let dt = 0.02;
    let nsteps = 75000;
    let delta_trial = 1e-5;
    let (sig, fdev) = uniform_tdbdg(g, m, 0.0, delta_trial, &qn, &qw, dt, nsteps);
    let w_meas = line_frequency(&sig, dt, 0.02, 0.398, 0.0005);
    let rel = ((w_meas - mb_nodes) / mb_nodes).abs();
    let rel_cont = ((mb_nodes - mb) / mb).abs();
    // the amplitude of the bound-state line: least-squares fit of a cos(M_B t) + b sin(M_B t) to Re d(t), Im d(t) on the last
    // two thirds of the run
    let d0 = sig[0];
    let fit = |part: &dyn Fn(C64) -> f64| -> (f64, f64) {
        let (mut scc, mut sss, mut scs, mut syc, mut sys) = (0.0, 0.0, 0.0, 0.0, 0.0);
        for (i, z) in sig.iter().enumerate().skip(nsteps / 3) {
            let t = i as f64 * dt;
            let (cc, ss) = ((mb_nodes * t).cos(), (mb_nodes * t).sin());
            let yv = part(*z) / d0.re;
            scc += cc * cc;
            sss += ss * ss;
            scs += cc * ss;
            syc += yv * cc;
            sys += yv * ss;
        }
        let det = scc * sss - scs * scs;
        ((syc * sss - sys * scs) / det, (sys * scc - syc * scs) / det)
    };
    let (are, bre) = fit(&|z: C64| z.re);
    // the imaginary part stays zero: at mu = 0 a real start gives a real pair field for all times
    let im_max = sig.iter().fold(0.0f64, |a, z| a.max(z.im.abs())) / d0.re;
    let mut csv = Csv::new(
        "the microscopic time evolution of a small uniform pair field in the normal vacuum (time-dependent mean-field equations of Section 2.10 for a uniform polar pair field, G = 20): the pair field d^3(t) divided by d^3(0) (it stays real), and the line of the bound SFBS boson with the frequency M_B of the effective field equation (amplitude and phase fitted on the last two thirds of the run) (Test 2, item 8, Figure 22(d))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", format!("{}", g)), ("M_B", format!("{:.10}", mb_nodes)), ("dt", format!("{}", dt)), ("delta_trial", format!("{}", delta_trial)), ("d0", format!("{:.6e}", d0.re))],
        "x_LL^0,d_over_d0,line_MB",
    );
    for (i, z) in sig.iter().enumerate() {
        if i % 10 != 0 || i as f64 * dt > 400.0 {
            continue;
        }
        let t = i as f64 * dt;
        let (cc, ss) = ((mb_nodes * t).cos(), (mb_nodes * t).sin());
        csv.row(&[format!("{:.2}", t), format!("{:.10}", z.re / d0.re), format!("{:.10}", are * cc + bre * ss)]);
    }
    csv.write(dir, "t2_sfbs_tdbdg_normal.csv");
    let ok = rel < 1e-6 && rel_cont < 1e-9 && fdev < 1e-6 && im_max < 1e-12;
    ch.add(
        "SB-10",
        "the microscopic check of the effective field equation in the normal vacuum (m = 0.2, mu = 0, Lambda = 1, G = 20): the time-dependent mean-field equations of Section 2.10 for a uniform polar pair field (the modes of the planes (3.12) on 320 radial nodes, RK4 with the step 0.02 up to x_LL^0 = 1500), started from the trial gap 1e-5, give a pair field d^3(x_LL^0) whose spectral line below the two-particle threshold 2m lies at the pole M_B of the SFBS propagator, chi_pr(M_B, 0) = 1/G, computed with the same nodes (relative tol 1e-6); that pole equals the one of SB-8 (relative tol 1e-9); the pair field stays real (tol 1e-12 of d^3(0)); the mean-field energy per volume, which the equations conserve, changes by less than 1e-6 of the scale sum 4 |+-eps - mu| (the RK4 truncation error)",
        ok,
        format!(
            "line of d^3(x_LL^0) at {:.9}, pole M_B {:.9} (same nodes; relative deviation {}), M_B of SB-8 {:.9} (relative deviation {}); largest change of the mean-field energy {}; largest imaginary part {}; fitted line amplitude {:.4} of d^3(0)",
            w_meas,
            mb_nodes,
            sci(rel),
            mb,
            sci(rel_cont),
            sci(fdev),
            sci(im_max),
            (are * are + bre * bre).sqrt()
        ),
    );
}

// ================================================================== item [8], the effective potential

/// Xi(M^2) = 2 int_{q<Lambda} d^3q/(2 pi)^3 (q^2 + M^2)^(1/2)
///         = (Lambda (2 Lambda^2 + M^2) (Lambda^2 + M^2)^(1/2) - M^4 arsinh(Lambda/M))/(8 pi^2)
fn xi_cl(m2: f64) -> f64 {
    let el = (LAMBDA * LAMBDA + m2).sqrt();
    let t = if m2 > 0.0 { m2 * m2 * (LAMBDA / m2.sqrt()).asinh() } else { 0.0 };
    (LAMBDA * (2.0 * LAMBDA * LAMBDA + m2) * el - t) / (8.0 * PI * PI)
}

/// dXi/dM^2 = int d^3q/(2 pi)^3 (q^2 + M^2)^(-1/2) = (Lambda (Lambda^2 + M^2)^(1/2) - M^2 arsinh(Lambda/M))/(4 pi^2)
fn xi_cl_d1(m2: f64) -> f64 {
    let el = (LAMBDA * LAMBDA + m2).sqrt();
    let t = if m2 > 0.0 { m2 * (LAMBDA / m2.sqrt()).asinh() } else { 0.0 };
    (LAMBDA * el - t) / (4.0 * PI * PI)
}

/// d^2Xi/d(M^2)^2 = -(1/2) int d^3q/(2 pi)^3 (q^2 + M^2)^(-3/2) = -(arsinh(Lambda/M) - Lambda/(Lambda^2 + M^2)^(1/2))/(4 pi^2)
fn xi_cl_d2(m2: f64) -> f64 {
    let el = (LAMBDA * LAMBDA + m2).sqrt();
    -((LAMBDA / m2.sqrt()).asinh() - LAMBDA / el) / (4.0 * PI * PI)
}

/// radial nodes q_i and weights w_i of int_{q<Lambda} d^3q/(2 pi)^3 f(q) = sum_i w_i f(q_i) (the factor q^2/(2 pi^2) included),
/// composite Gauss-Legendre with break points around the Fermi momentum when mu > m (width: the smallest gap used)
fn radial_nodes(m: f64, mu: f64, width: f64) -> (Vec<f64>, Vec<f64>) {
    let kf = if mu > m { (mu * mu - m * m).sqrt() } else { 0.0 };
    let cuts: Vec<f64> = if kf > 0.0 { vec![kf - 4.0 * width, kf, kf + 4.0 * width] } else { vec![] };
    let (xs, ws) = quad::composite(0.0, LAMBDA, &cuts, 40, 20);
    let w: Vec<f64> = xs.iter().zip(&ws).map(|(q, w)| w * q * q / (2.0 * PI * PI)).collect();
    (xs, w)
}

/// W_g(Delta^2) = int d^3q/(2 pi)^3 (E^+ + E^- - |eps + mu| - |eps - mu|), E^+- = ((eps +- mu)^2 + Delta^2)^(1/2)
fn w_gap(delta2: f64, m: f64, mu: f64, qn: &[f64], qw: &[f64]) -> f64 {
    let mut s = 0.0;
    for (q, w) in qn.iter().zip(qw) {
        let e = (q * q + m * m).sqrt();
        let ep = ((e + mu).powi(2) + delta2).sqrt();
        let em = ((e - mu).powi(2) + delta2).sqrt();
        s += w * (ep + em - (e + mu).abs() - (e - mu).abs());
    }
    s
}

/// the two gaps of a pair field: (Delta^+-)^2 = |d|^2 (1 +- zeta), zeta = |d x d^*|/|d|^2; returns ((Delta^+)^2, (Delta^-)^2)
fn two_gaps2(d: &[C64; 3]) -> (f64, f64) {
    let n2: f64 = d.iter().map(|z| z.norm_sqr()).sum();
    if n2 == 0.0 {
        return (0.0, 0.0);
    }
    let dc = [d[0].conj(), d[1].conj(), d[2].conj()];
    let cr = [d[1] * dc[2] - d[2] * dc[1], d[2] * dc[0] - d[0] * dc[2], d[0] * dc[1] - d[1] * dc[0]];
    let zeta = (cr.iter().map(|z| z.norm_sqr()).sum::<f64>()).sqrt() / n2;
    (n2 * (1.0 + zeta), (n2 * (1.0 - zeta)).max(0.0))
}

/// Omega(d) from the invariants: |d|^2/G - W_g((Delta^+)^2) - W_g((Delta^-)^2)
fn omega_inv(d: &[C64; 3], g: f64, m: f64, mu: f64, qn: &[f64], qw: &[f64]) -> f64 {
    let n2: f64 = d.iter().map(|z| z.norm_sqr()).sum();
    let (dp2, dm2) = two_gaps2(d);
    n2 / g - w_gap(dp2, m, mu, qn, qw) - w_gap(dm2, m, mu, qn, qw)
}

/// the 16 x 16 Bogoliubov matrix H(q) = [[h(q) - mu, P], [P^dag, -(h(-q) - mu)^*]] of a uniform pair field d
fn bdg16(d: &[C64; 3], qv: &[f64; 3], m: f64, mu: f64) -> CMat {
    let p = pair_potential(d);
    let h = h8(qv, m).sub(&CMat::identity(8).scale_r(mu));
    let hm = h8(&[-qv[0], -qv[1], -qv[2]], m).conj().sub(&CMat::identity(8).scale_r(mu));
    let mut big = CMat::zeros(16);
    big.put(0, 0, &h);
    big.put(0, 8, &p);
    big.put(8, 0, &p.dagger());
    big.put(8, 8, &hm.scale_r(-1.0));
    big
}

/// Omega(d) from the Bogoliubov matrices: |d|^2/G - (1/2) int d^3q/(2 pi)^3 [sum of the positive eigenvalues of H(q) - the
/// same at d = 0], with q along the unit direction qdir (the eigenvalues depend only on |q|, check SB-11)
fn omega_bdg(d: &[C64; 3], g: f64, m: f64, mu: f64, qn: &[f64], qw: &[f64], qdir: &[f64; 3]) -> f64 {
    let n2: f64 = d.iter().map(|z| z.norm_sqr()).sum();
    let zero = [C64::ZERO; 3];
    let mut s = 0.0;
    for (q, w) in qn.iter().zip(qw) {
        let qv = [q * qdir[0], q * qdir[1], q * qdir[2]];
        let (e1, _) = eigh(&bdg16(d, &qv, m, mu));
        let (e0, _) = eigh(&bdg16(&zero, &qv, m, mu));
        let p1: f64 = e1.iter().filter(|&&x| x > 0.0).sum();
        let p0: f64 = e0.iter().filter(|&&x| x > 0.0).sum();
        s += w * (p1 - p0);
    }
    n2 / g - 0.5 * s
}

fn rand_d(rng: &mut Rng, scale: f64) -> [C64; 3] {
    [c(scale * rng.unit(), scale * rng.unit()), c(scale * rng.unit(), scale * rng.unit()), c(scale * rng.unit(), scale * rng.unit())]
}

/// SB-11: the structure of the effective potential
fn sb11(ch: &mut Checks, fr: &mut Frags) {
    let m = MASS;
    let mut rng = Rng(31415);
    // (i) the eigenvalues of H(q) for random pair fields, momenta and chemical potentials
    let mut eig_dev = 0.0f64;
    for t in 0..30 {
        let d = rand_d(&mut rng, 0.4);
        let qv = [rng.unit(), rng.unit(), rng.unit()];
        let mu = [0.0, 0.35, 0.6][t % 3];
        let (ev, _) = eigh(&bdg16(&d, &qv, m, mu));
        let e = (qv[0] * qv[0] + qv[1] * qv[1] + qv[2] * qv[2] + m * m).sqrt();
        let (dp2, dm2) = two_gaps2(&d);
        let mut want = vec![];
        for &dd in &[dp2, dm2] {
            for &x in &[((e + mu).powi(2) + dd).sqrt(), ((e - mu).powi(2) + dd).sqrt()] {
                for _ in 0..2 {
                    want.push(x);
                    want.push(-x);
                }
            }
        }
        want.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for (a, b) in ev.iter().zip(&want) {
            eig_dev = eig_dev.max((a - b).abs());
        }
    }
    // (ii) Omega from the Bogoliubov matrices against the invariant form, at mu = 0 and mu = 0.5 (Fermi sea)
    let g = G_VAC;
    let mut om_dev = 0.0f64;
    let mut lines = vec![];
    for &mu in &[0.0, 0.5] {
        let (qn, qw) = radial_nodes(m, mu, 0.05);
        for _ in 0..3 {
            let d = rand_d(&mut rng, 0.25);
            let qdir = {
                let v = [rng.unit(), rng.unit(), rng.unit()];
                let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
                [v[0] / n, v[1] / n, v[2] / n]
            };
            let a = omega_bdg(&d, g, m, mu, &qn, &qw, &qdir);
            let b = omega_inv(&d, g, m, mu, &qn, &qw);
            om_dev = om_dev.max((a - b).abs() / b.abs().max(1e-12));
            lines.push(format!("mu = {}: {:.12} against {:.12}", mu, a, b));
        }
    }
    // (iii) the invariance under exp(2i thF) Rv(U)
    let (qn, qw) = radial_nodes(m, 0.0, 0.05);
    let mut inv_dev = 0.0f64;
    for _ in 0..3 {
        let d = rand_d(&mut rng, 0.25);
        let r = adjoint(&su2(&[rng.unit() * 3.0, rng.unit() * 3.0, rng.unit() * 3.0]));
        let ph = C64::cis(2.0 * rng.unit() * 3.0);
        let mut dt = [C64::ZERO; 3];
        for i in 0..3 {
            let mut z = C64::ZERO;
            for k in 0..3 {
                z += d[k].scale(r[i][k]);
            }
            dt[i] = ph * z;
        }
        let a = omega_bdg(&d, g, m, 0.0, &qn, &qw, &[0.0, 0.0, 1.0]);
        let b = omega_bdg(&dt, g, m, 0.0, &qn, &qw, &[0.0, 0.0, 1.0]);
        inv_dev = inv_dev.max((a - b).abs() / a.abs());
    }
    // (iv) the sum of the positive eigenvalues does not depend on the direction of q
    let mut dir_dev = 0.0f64;
    for _ in 0..10 {
        let d = rand_d(&mut rng, 0.3);
        let q = 0.1 + 0.8 * rng.unit().abs();
        let mut sums = vec![];
        for _ in 0..3 {
            let v = [rng.unit(), rng.unit(), rng.unit()];
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            let qv = [q * v[0] / n, q * v[1] / n, q * v[2] / n];
            let (ev, _) = eigh(&bdg16(&d, &qv, m, 0.3));
            sums.push(ev.iter().filter(|&&x| x > 0.0).sum::<f64>());
        }
        dir_dev = dir_dev.max((sums[0] - sums[1]).abs()).max((sums[0] - sums[2]).abs());
    }
    // (v) the isospin part Dpair = sum d^iota (E tau_iota)^dag of the pairing block, P = (C gamma_ch)^dag (x) Dpair
    let d = rand_d(&mut rng, 0.5);
    let mut dsym = SymMat::zeros(2, 2);
    let mut dnum = CMat::zeros(2);
    for i in 0..3 {
        let m2 = eps2().mul(&sigma(i + 1)).dagger();
        dsym.add_scaled(&m2, &Lin::mono(&format!("d^{}_{{\\rm LL}}", i + 1), d[i]));
        dnum = dnum.add(&m2.scale(d[i]));
    }
    let pfull = pair_potential(&d);
    let pkron = cconj().mul(&gamma_ch()).dagger().kron(&dnum);
    let kron_dev = pfull.sub(&pkron).max_abs();
    let mut sym_dev = 0.0f64;
    for r in 0..2 {
        for cc in 0..2 {
            sym_dev = sym_dev.max((dsym.get(r, cc).eval() - dnum.get(r, cc)).abs());
        }
    }
    assert!(sym_dev < 1e-15 && kron_dev < 1e-15, "printed matrix Dpair");
    fr.add("t2s_Dpair", format!("$$\\mathsf D_{{\\rm LL}}={}$$", dsym.tex()));
    let ok = eig_dev < 1e-12 && om_dev < 1e-10 && inv_dev < 1e-12 && dir_dev < 1e-12;
    ch.add(
        "SB-11",
        "the structure of the effective potential (m = 0.2, Lambda = 1): for 30 random pair fields d, momenta q and mu = 0, 0.35, 0.6, the 16 eigenvalues of the Bogoliubov matrix H_BdG(q) are +-E^+(Delta^+), +-E^-(Delta^+), +-E^+(Delta^-), +-E^-(Delta^-), each twice, with (Delta^+-)^2 = |d|^2 (1 +- zeta) (tol 1e-12); Omega(d) = |d|^2/G - W_g((Delta^+)^2) - W_g((Delta^-)^2) equals |d|^2/G - (1/2) int [sum of the positive eigenvalues of H_BdG(q) - the same at d = 0] for three random d at mu = 0 and at mu = 0.5 (Fermi sea), G = 22.5 (relative tol 1e-10); Omega is unchanged by exp(2i thF) Rv(U) (relative tol 1e-12); the sum of the positive eigenvalues does not depend on the direction of q (tol 1e-12); the printed Dpair satisfies Ppair = (C gamma_ch)^dag (x) Dpair",
        ok,
        format!(
            "largest deviations: eigenvalues {}, Omega {} ({}), invariance {}, direction of q {}, Dpair {}",
            sci(eig_dev),
            sci(om_dev),
            lines.join("; "),
            sci(inv_dev),
            sci(dir_dev),
            sci(kron_dev)
        ),
    );
}

/// the uniform (polar) effective potential at mu = 0 in closed form: Omega_u(Delta^2) = Delta^2/G - 2 (Xi(m^2 + Delta^2) - Xi(m^2))
fn omega_u(delta2: f64, g: f64, m: f64) -> f64 {
    delta2 / g - 2.0 * (xi_cl(m * m + delta2) - xi_cl(m * m))
}

/// the Ginzburg-Landau form a_GL |d|^2 + b_GL (2 |d|^4 - |d.d|^2) at mu = 0, with a_GL = 1/G - 1/G_crit, b_GL = -Xi''(m^2)
fn omega_gl(n2: f64, dd2: f64, g: f64, m: f64) -> f64 {
    let a = 1.0 / g - inv_gcrit(m);
    let b = -xi_cl_d2(m * m);
    a * n2 + b * (2.0 * n2 * n2 - dd2)
}

/// SB-12: the closed forms at mu = 0 and the Ginzburg-Landau expansion
fn sb12(ch: &mut Checks, dir: &PathBuf) {
    let m = MASS;
    let (qn, qw) = radial_nodes(m, 0.0, 0.05);
    // (i) Xi against quadrature
    let mut xi_dev = 0.0f64;
    for &m2 in &[0.04, 0.0839, 0.25, 1.0] {
        let quad: f64 = qn.iter().zip(&qw).map(|(q, w)| 2.0 * w * (q * q + m2).sqrt()).sum();
        xi_dev = xi_dev.max(((quad - xi_cl(m2)) / xi_cl(m2)).abs());
    }
    // (ii) the derivatives against central differences (Richardson)
    let mut der_dev = 0.0f64;
    for &m2 in &[0.04, 0.0839, 0.25] {
        let h = 1e-4;
        let d1 = (4.0 * (xi_cl(m2 + h) - xi_cl(m2 - h)) / (2.0 * h) - (xi_cl(m2 + 2.0 * h) - xi_cl(m2 - 2.0 * h)) / (4.0 * h)) / 3.0;
        let d2 = (4.0 * (xi_cl(m2 + h) - 2.0 * xi_cl(m2) + xi_cl(m2 - h)) / (h * h) - (xi_cl(m2 + 2.0 * h) - 2.0 * xi_cl(m2) + xi_cl(m2 - 2.0 * h)) / (4.0 * h * h)) / 3.0;
        der_dev = der_dev.max(((d1 - xi_cl_d1(m2)) / xi_cl_d1(m2)).abs()).max(((d2 - xi_cl_d2(m2)) / xi_cl_d2(m2)).abs());
    }
    // (iii) W_g at mu = 0 against the closed form
    let mut w_dev = 0.0f64;
    for &dl in &[0.05, 0.2094933, 0.4] {
        let quad = w_gap(dl * dl, m, 0.0, &qn, &qw);
        let cl = xi_cl(m * m + dl * dl) - xi_cl(m * m);
        w_dev = w_dev.max(((quad - cl) / cl).abs());
    }
    // (iv) concavity of W_g in Delta^2: second differences on a grid, at mu = 0 and with a Fermi sea (mu = 0.5)
    let mut concave = true;
    let mut max_second = f64::NEG_INFINITY;
    for &mu in &[0.0, 0.5] {
        let (qn2, qw2) = radial_nodes(m, mu, 0.01);
        let h = 0.002;
        for i in 1..50 {
            let x = 0.01 * i as f64 * 0.01 * i as f64; // Delta^2 from 1e-4 to 0.24
            let sd = w_gap(x + h, m, mu, &qn2, &qw2) - 2.0 * w_gap(x, m, mu, &qn2, &qw2) + w_gap((x - h).max(0.0), m, mu, &qn2, &qw2);
            if x - h < 0.0 {
                continue;
            }
            max_second = max_second.max(sd);
            if sd >= 0.0 {
                concave = false;
            }
        }
    }
    // (v) b_GL = -Xi''(m^2) = Z_t0 and a_GL
    let bgl = -xi_cl_d2(m * m);
    let bz = (bgl - z_t(m)).abs();
    // (vi) the remainder of the Ginzburg-Landau form is of order |d|^6, for a polar and a non-unitary direction
    let mut ratios = vec![];
    let g = G_VAC;
    for &zeta in &[0.0f64, 0.6] {
        let mut rr = vec![];
        for &sc in &[0.04, 0.02, 0.01] {
            // a field with |d| = sc and non-unitarity zeta: (0, i b, a) with a^2 + b^2 = sc^2, 2ab = zeta sc^2
            let (dp, dm) = ((sc * sc * (1.0 + zeta)).sqrt(), (sc * sc * (1.0 - zeta)).sqrt());
            let (a, b) = (0.5 * (dp + dm), 0.5 * (dp - dm));
            let d = [C64::ZERO, c(0.0, b), c(a, 0.0)];
            let n2 = sc * sc;
            let dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
            let full = n2 / g - (xi_cl(m * m + dp * dp) - xi_cl(m * m)) - (xi_cl(m * m + dm * dm) - xi_cl(m * m));
            let gl = omega_gl(n2, dd.norm_sqr(), g, m);
            rr.push((full - gl) / (n2 * n2 * n2));
            let _ = &d;
        }
        ratios.push(rr);
    }
    let conv = ratios.iter().all(|r| (r[2] - r[1]).abs() < 0.3 * (r[1] - r[0]).abs() + 1e-6);
    // (vii) the massless polar potential: Delta^2 (1/G - Lambda^2/(2 pi^2)) + Delta^4 (ln(2 Lambda/Delta) - 1/4)/(4 pi^2) + O(Delta^6)
    let mut mz = vec![];
    for &dl in &[0.04, 0.02, 0.01] {
        let exact = omega_u(dl * dl, g, 0.0);
        let approx = dl * dl * (1.0 / g - LAMBDA * LAMBDA / (2.0 * PI * PI)) + dl.powi(4) * ((2.0 * LAMBDA / dl).ln() - 0.25) / (4.0 * PI * PI);
        mz.push((exact - approx).abs() / dl.powi(4));
    }
    let mz_ok = mz[1] < 0.3 * mz[0] && mz[2] < 0.3 * mz[1];
    // (viii) the gap equation is the stationarity of Omega_u: dOmega_u/dDelta^2 = 1/G - 2 Xi'(m^2 + Delta^2) vanishes at the gap
    let dv = gap_mu0(g, m);
    let stat = (1.0 / g - 2.0 * xi_cl_d1(m * m + dv * dv)).abs();
    let ok = xi_dev < 1e-12 && der_dev < 1e-7 && w_dev < 1e-11 && concave && bz < 1e-15 && conv && mz_ok && stat < 1e-14;
    ch.add(
        "SB-12",
        "the effective potential at mu = 0 (m = 0.2, Lambda = 1): Xi(M^2) = (Lambda (2 Lambda^2 + M^2) (Lambda^2 + M^2)^(1/2) - M^4 arsinh(Lambda/M))/(8 pi^2) equals 2 int d^3q/(2 pi)^3 (q^2 + M^2)^(1/2) by quadrature (relative tol 1e-12); its first and second derivatives in closed form equal numerical derivatives (relative tol 1e-7); W_g(Delta^2) = Xi(m^2 + Delta^2) - Xi(m^2) (relative tol 1e-11); W_g is strictly concave in Delta^2 on a grid of 49 values (second differences negative) at mu = 0 and with a Fermi sea (mu = 0.5); b_GL = -Xi''(m^2) = Z_t0 (tol 1e-15); the remainder of the Ginzburg-Landau form a_GL |d|^2 + b_GL (2 |d|^4 - |d.d|^2) divided by |d|^6 converges as |d| -> 0 (|d| = 0.04, 0.02, 0.01; polar and zeta = 0.6); for m = 0 the remainder of Delta^2 (1/G - Lambda^2/(2 pi^2)) + Delta^4 (ln(2 Lambda/Delta) - 1/4)/(4 pi^2) divided by Delta^4 falls at least by the factor 0.3 when Delta is halved; 1/G - 2 Xi'(m^2 + Delta_vac^2) = 0 at the gap of G = 22.5 (tol 1e-14)",
        ok,
        format!(
            "largest deviations: Xi {}, derivatives {}, W_g {}; largest second difference of W_g {} (negative: concave); b_GL = {:.10} (Z_t0 - b_GL = {}); remainder/|d|^6 at |d| = 0.04, 0.02, 0.01: polar {:.6}, {:.6}, {:.6}, zeta = 0.6 {:.6}, {:.6}, {:.6}; massless remainder/Delta^4 at Delta = 0.04, 0.02, 0.01: {}, {}, {}; stationarity at Delta_vac = {:.10}: {}",
            sci(xi_dev),
            sci(der_dev),
            sci(w_dev),
            sci(max_second),
            bgl,
            sci(bz),
            ratios[0][0],
            ratios[0][1],
            ratios[0][2],
            ratios[1][0],
            ratios[1][1],
            ratios[1][2],
            sci(mz[0]),
            sci(mz[1]),
            sci(mz[2]),
            dv,
            sci(stat)
        ),
    );
    // data of Figure 23
    let gcrit = 1.0 / inv_gcrit(m);
    let dvac = gap_mu0(G_VAC, m);
    let mut csv = Csv::new(
        "the effective potential Omega of the SFBS along the polar direction (|d| = Delta, zeta = 0) at mu = 0 for four couplings, with its Ginzburg-Landau form a_GL Delta^2 + b_GL Delta^4 (Test 2, item 8, Figure 23(a))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G_crit", format!("{:.10}", gcrit)), ("b_GL", format!("{:.10}", bgl))],
        "G,Delta,Omega,Omega_GL",
    );
    for &g in &[18.0, gcrit, 22.5, 24.0] {
        for i in 0..=140 {
            let dl = 0.35 * i as f64 / 140.0;
            csv.row(&[format!("{:.10}", g), format!("{:.6}", dl), format!("{:.12e}", omega_u(dl * dl, g, m)), format!("{:.12e}", omega_gl(dl * dl, dl.powi(4), g, m))]);
        }
    }
    csv.write(dir, "t2_sfbs_potential_polar.csv");
    let mut csv = Csv::new(
        "the effective potential Omega of the SFBS on the complex plane of the polar component d^3 (d^1 = d^2 = 0) at mu = 0, G = 22.5, on a polar grid over three quarters of the plane (41 radii from 0 to 0.31, 55 angles from 0 to 3 pi/2, so that the cut shows the profile; the rows run over the angles for each radius): the Mexican hat (Test 2, item 8, Figure 23(b))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", "22.5".into()), ("Delta_vac", format!("{:.10}", dvac)), ("radii", "41".into()), ("angles", "55".into())],
        "d3_re,d3_im,Omega",
    );
    for i in 0..=40 {
        let r = 0.31 * i as f64 / 40.0;
        for j in 0..=54 {
            let th = 1.5 * PI * j as f64 / 54.0;
            let (x, y) = (r * th.cos(), r * th.sin());
            csv.row(&[format!("{:.8}", x), format!("{:.8}", y), format!("{:.12e}", omega_u(r * r, G_VAC, m))]);
        }
    }
    csv.write(dir, "t2_sfbs_potential_hat.csv");
    let mut csv = Csv::new(
        "the effective potential Omega of the SFBS at fixed |d| as a function of the non-unitarity zeta (the two gaps (Delta^+-)^2 = |d|^2 (1 +- zeta)) at mu = 0, G = 22.5, for |d| = 0.5, 1 and 1.5 times the gap of the vacuum, with the Ginzburg-Landau form (Test 2, item 8, Figure 23(c))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", "22.5".into()), ("Delta_vac", format!("{:.10}", dvac))],
        "abs_d,zeta,Omega,Omega_GL",
    );
    for &f in &[0.5, 1.0, 1.5] {
        let a = f * dvac;
        for i in 0..=50 {
            let z = i as f64 / 50.0;
            let (dp2, dm2) = (a * a * (1.0 + z), a * a * (1.0 - z));
            let om = a * a / G_VAC - (xi_cl(m * m + dp2) - xi_cl(m * m)) - (xi_cl(m * m + dm2) - xi_cl(m * m));
            let dd2 = a.powi(4) * (1.0 - z * z);
            csv.row(&[format!("{:.10}", a), format!("{:.4}", z), format!("{:.12e}", om), format!("{:.12e}", omega_gl(a * a, dd2, G_VAC, m))]);
        }
    }
    csv.write(dir, "t2_sfbs_potential_zeta.csv");
    let mut csv = Csv::new(
        "the gap of the condensed vacuum (the minimum of the effective potential) at mu = 0 against the coupling: the solution of the gap equation 1/G = 2 Xi'(m^2 + Delta^2) and the Ginzburg-Landau value (-a_GL/(2 b_GL))^(1/2), with a_GL and the depth Omega(Delta_vac) of the minimum (Test 2, item 8, Figure 23(d))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G_crit", format!("{:.10}", gcrit)), ("b_GL", format!("{:.10}", bgl))],
        "G,a_GL,Delta_vac,Delta_vac_GL,Omega_min",
    );
    for i in 0..=80 {
        let g = gcrit + (32.0 - gcrit) * (i as f64 / 80.0).powi(2);
        let a = 1.0 / g - inv_gcrit(m);
        let dv = if i == 0 { 0.0 } else { gap_mu0(g, m) };
        let dgl = if a < 0.0 { (-a / (2.0 * bgl)).sqrt() } else { 0.0 };
        csv.row(&[format!("{:.8}", g), format!("{:.12e}", a), format!("{:.12}", dv), format!("{:.12}", dgl), format!("{:.12e}", omega_u(dv * dv, g, m))]);
    }
    csv.write(dir, "t2_sfbs_vacuum_gap.csv");
}

// ================================================================== item [9]: minima, symmetry breaking, SFBSv

/// d^3Xi/d(M^2)^3 = (3/4) int d^3q/(2 pi)^3 (q^2 + M^2)^(-5/2) = Lambda^3/(8 pi^2 M^2 (Lambda^2 + M^2)^(3/2))
fn xi_cl_d3(m2: f64) -> f64 {
    LAMBDA.powi(3) / (8.0 * PI * PI * m2 * (LAMBDA * LAMBDA + m2).powf(1.5))
}

/// d^4Xi/d(M^2)^4 = -(15/8) int d^3q/(2 pi)^3 (q^2 + M^2)^(-7/2), with
/// int_0^Lambda q^2 (q^2 + M^2)^(-7/2) dq = Lambda^3 (2 Lambda^2 + 5 M^2)/(15 M^4 (Lambda^2 + M^2)^(5/2))
fn xi_cl_d4(m2: f64) -> f64 {
    let el2 = LAMBDA * LAMBDA + m2;
    -(15.0 / 8.0) * LAMBDA.powi(3) * (2.0 * LAMBDA * LAMBDA + 5.0 * m2) / (30.0 * PI * PI * m2 * m2 * el2.powf(2.5))
}

/// the potential of one sector as a function of its gap y at mu = 0: Omega_s(y) = y^2/(2G) - (Xi(m^2 + y^2) - Xi(m^2));
/// the effective potential is Omega_s(Delta^+) + Omega_s(Delta^-)
fn omega_s(y: f64, g: f64, m: f64) -> f64 {
    y * y / (2.0 * g) - (xi_cl(m * m + y * y) - xi_cl(m * m))
}

/// the derivatives of Omega_s with respect to y (orders 1 to 4), from the derivatives of Xi by the chain rule
fn omega_s_derivs(y: f64, g: f64, m: f64) -> [f64; 4] {
    let a = m * m + y * y;
    let (x1, x2, x3, x4) = (xi_cl_d1(a), xi_cl_d2(a), xi_cl_d3(a), xi_cl_d4(a));
    let d1 = y / g - 2.0 * y * x1;
    let d2 = 1.0 / g - 2.0 * x1 - 4.0 * y * y * x2;
    let d3 = -12.0 * y * x2 - 8.0 * y.powi(3) * x3;
    let d4 = -12.0 * x2 - 48.0 * y * y * x3 - 16.0 * y.powi(4) * x4;
    [d1, d2, d3, d4]
}

/// the effective potential at mu = 0 from the invariants (closed form): Omega_s(Delta^+) + Omega_s(Delta^-)
fn omega_cl(d: &[C64; 3], g: f64, m: f64) -> f64 {
    let (dp2, dm2) = two_gaps2(d);
    omega_s(dp2.sqrt(), g, m) + omega_s(dm2.sqrt(), g, m)
}

fn to_r6(d: &[C64; 3]) -> [f64; 6] {
    [d[0].re, d[0].im, d[1].re, d[1].im, d[2].re, d[2].im]
}

fn from_r6(x: &[f64; 6]) -> [C64; 3] {
    [c(x[0], x[1]), c(x[2], x[3]), c(x[4], x[5])]
}

/// numerical gradient (central differences) of a function on R^6
fn grad6(f: &dyn Fn(&[f64; 6]) -> f64, x: &[f64; 6], h: f64) -> [f64; 6] {
    let mut gv = [0.0; 6];
    for i in 0..6 {
        let mut xp = *x;
        let mut xm = *x;
        xp[i] += h;
        xm[i] -= h;
        gv[i] = (f(&xp) - f(&xm)) / (2.0 * h);
    }
    gv
}

/// numerical Hessian (central differences) of a function on R^6
fn hess6(f: &dyn Fn(&[f64; 6]) -> f64, x: &[f64; 6], h: f64) -> CMat {
    let mut hm = CMat::zeros(6);
    let f0 = f(x);
    for i in 0..6 {
        for j in i..6 {
            let v = if i == j {
                let mut xp = *x;
                let mut xm = *x;
                xp[i] += h;
                xm[i] -= h;
                (f(&xp) - 2.0 * f0 + f(&xm)) / (h * h)
            } else {
                let mut pp = *x;
                let mut pm = *x;
                let mut mp = *x;
                let mut mm = *x;
                pp[i] += h;
                pp[j] += h;
                pm[i] += h;
                pm[j] -= h;
                mp[i] -= h;
                mp[j] += h;
                mm[i] -= h;
                mm[j] -= h;
                (f(&pp) - f(&pm) - f(&mp) + f(&mm)) / (4.0 * h * h)
            };
            hm.set(i, j, c(v, 0.0));
            hm.set(j, i, c(v, 0.0));
        }
    }
    hm
}

/// BFGS minimization of a function on R^6 with numerical gradients and a backtracking line search (Armijo condition);
/// returns the path of the iterates, the start included
fn bfgs6(f: &dyn Fn(&[f64; 6]) -> f64, x0: &[f64; 6], maxit: usize) -> Vec<[f64; 6]> {
    let mut x = *x0;
    let mut path = vec![x];
    let mut hinv = [[0.0f64; 6]; 6];
    for i in 0..6 {
        hinv[i][i] = 100.0; // the inverse of a curvature of about 0.01, the scale of the potential
    }
    let mut gx = grad6(f, &x, 1e-7);
    for _ in 0..maxit {
        let gn = gx.iter().map(|v| v * v).sum::<f64>().sqrt();
        if gn < 1e-10 {
            // the numerical gradient has a noise of about 1e-11: stop when it is below 1e-10
            break;
        }
        let mut p = [0.0; 6];
        for i in 0..6 {
            for j in 0..6 {
                p[i] -= hinv[i][j] * gx[j];
            }
        }
        if p.iter().zip(&gx).map(|(a, b)| a * b).sum::<f64>() >= 0.0 {
            // not a descent direction: restart with the scaled gradient
            for i in 0..6 {
                for j in 0..6 {
                    hinv[i][j] = if i == j { 100.0 } else { 0.0 };
                }
                p[i] = -100.0 * gx[i];
            }
        }
        let slope: f64 = p.iter().zip(&gx).map(|(a, b)| a * b).sum();
        let fx = f(&x);
        let mut t = 1.0;
        let mut xn = x;
        for _ in 0..60 {
            for i in 0..6 {
                xn[i] = x[i] + t * p[i];
            }
            if f(&xn) <= fx + 1e-4 * t * slope {
                break;
            }
            t *= 0.5;
        }
        let gnew = grad6(f, &xn, 1e-7);
        let sv: Vec<f64> = (0..6).map(|i| xn[i] - x[i]).collect();
        let yv: Vec<f64> = (0..6).map(|i| gnew[i] - gx[i]).collect();
        let sy: f64 = sv.iter().zip(&yv).map(|(a, b)| a * b).sum();
        if sy > 1e-30 {
            let mut hy = [0.0; 6];
            for i in 0..6 {
                for j in 0..6 {
                    hy[i] += hinv[i][j] * yv[j];
                }
            }
            let yhy: f64 = yv.iter().zip(&hy).map(|(a, b)| a * b).sum();
            for i in 0..6 {
                for j in 0..6 {
                    hinv[i][j] += (sy + yhy) * sv[i] * sv[j] / (sy * sy) - (hy[i] * sv[j] + sv[i] * hy[j]) / sy;
                }
            }
        }
        let stalled = f(&xn) >= fx - 1e-21;
        x = xn;
        gx = gnew;
        path.push(x);
        if stalled {
            // no further decrease at the precision of the potential (about 1e-21 at its size 5e-5)
            break;
        }
    }
    path
}

/// SB-13: the minima, the other stationary points and the stabilizer
fn sb13(ch: &mut Checks, dir: &PathBuf) {
    let m = MASS;
    let g = G_VAC;
    let dvac = gap_mu0(g, m);
    let om_min = 2.0 * omega_s(dvac, g, m);
    let f = |x: &[f64; 6]| omega_cl(&from_r6(x), g, m);
    // (i) minimizations from 40 random fields
    let mut rng = Rng(271828);
    let mut worst_abs = 0.0f64;
    let mut worst_zeta = 0.0f64;
    let mut worst_om = 0.0f64;
    let mut paths = Csv::new(
        "the paths of 40 minimizations (BFGS) of the effective potential at mu = 0, G = 22.5 from random pair fields, in the plane of the two gaps (Delta^+, Delta^-), with the potential minus its minimum (Test 2, item 9, Figure 24(a), (c))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", format!("{}", g)), ("Delta_vac", format!("{:.10}", dvac)), ("Omega_min", format!("{:.12e}", om_min))],
        "run,iteration,Delta_+,Delta_-,Omega_above_min",
    );
    let mut iters = vec![];
    for run in 0..40 {
        let x0 = [0.3 * rng.unit(), 0.3 * rng.unit(), 0.3 * rng.unit(), 0.3 * rng.unit(), 0.3 * rng.unit(), 0.3 * rng.unit()];
        let path = bfgs6(&f, &x0, 400);
        iters.push(path.len() - 1);
        for (it, x) in path.iter().enumerate() {
            let (dp2, dm2) = two_gaps2(&from_r6(x));
            paths.row(&[format!("{}", run), format!("{}", it), format!("{:.10}", dp2.sqrt()), format!("{:.10}", dm2.sqrt()), format!("{:.6e}", (f(x) - om_min).max(1e-30))]);
        }
        let xf = path.last().unwrap();
        let d = from_r6(xf);
        let n2: f64 = d.iter().map(|z| z.norm_sqr()).sum();
        let (dp2, dm2) = two_gaps2(&d);
        let zeta = (dp2 - dm2) / (dp2 + dm2);
        worst_abs = worst_abs.max((n2.sqrt() - dvac).abs());
        worst_zeta = worst_zeta.max(zeta);
        worst_om = worst_om.max((f(xf) - om_min).abs());
    }
    paths.write(dir, "t2_sfbs_minimization_paths.csv");
    // (ii) the Hessians at the three kinds of stationary points
    let hstep = 1e-4;
    let vac = [0.0, 0.0, 0.0, 0.0, dvac, 0.0];
    let sad = [0.5 * dvac, 0.0, 0.0, 0.5 * dvac, 0.0, 0.0]; // (Delta_vac/2)(1, i, 0)
    let gvac = grad6(&f, &vac, 1e-6);
    let gsad = grad6(&f, &sad, 1e-6);
    let grad_max = gvac.iter().chain(gsad.iter()).fold(0.0f64, |a, v| a.max(v.abs()));
    let (ev_vac, _) = eigh(&hess6(&f, &vac, hstep));
    let (ev_sad, _) = eigh(&hess6(&f, &sad, hstep));
    // at d = 0 the potential is a_GL |d|^2 + b_GL |d|^4 along a polar line (Section 10.3); the radial Richardson second
    // difference (16 Omega(r) - Omega(2r))/(12 r^2) removes the quartic term and gives a_GL
    let a_gl = 1.0 / g - inv_gcrit(m);
    let rad = {
        let h = 1e-3;
        let x1 = [h, 0.0, 0.0, 0.0, 0.0, 0.0];
        let x2 = [2.0 * h, 0.0, 0.0, 0.0, 0.0, 0.0];
        (16.0 * f(&x1) - f(&x2)) / (12.0 * h * h)
    };
    // the predicted curvatures at the vacuum: 0 (three times) and 2 Omega_s^(2)(Delta_vac) (three times)
    let ds = omega_s_derivs(dvac, g, m);
    let v_pred = 2.0 * ds[1];
    let vac_ok = ev_vac[0].abs() < 1e-6 && ev_vac[1].abs() < 1e-6 && ev_vac[2].abs() < 1e-6 && ev_vac[3..].iter().all(|v| ((v - v_pred) / v_pred).abs() < 1e-6);
    let neg_sad = ev_sad.iter().filter(|&&v| v < -1e-6).count();
    let zero_sad = ev_sad.iter().filter(|&&v| v.abs() <= 1e-6).count();
    let pos_sad = ev_sad.iter().filter(|&&v| v > 1e-6).count();
    let om_sad = f(&sad);
    let sad_ok = neg_sad == 2 && zero_sad == 3 && pos_sad == 1 && ((om_sad - 0.5 * om_min) / om_min).abs() < 1e-12;
    let zero_ok = ((rad - a_gl) / a_gl).abs() < 1e-5 && a_gl < 0.0;
    // the data of Figure 24(d): the eigenvalues at the three stationary points
    let mut hcsv = Csv::new(
        "the eigenvalues of the Hessian (second derivatives with respect to the real and imaginary parts of d^1, d^2, d^3) of the effective potential at its three kinds of stationary points, mu = 0, G = 22.5: the vacuum Delta_vac e_3, the non-unitary field (Delta_vac/2)(1, i, 0), and d = 0 (where the Hessian is 2 a_GL times the unit matrix) (Test 2, item 9, Figure 24(d))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", format!("{}", g)), ("Delta_vac", format!("{:.10}", dvac)), ("a_GL", format!("{:.12e}", a_gl))],
        "point,index,eigenvalue",
    );
    for (name, ev) in [("vacuum", ev_vac.clone()), ("non_unitary", ev_sad.clone()), ("origin", vec![2.0 * a_gl; 6])] {
        for (i, v) in ev.iter().enumerate() {
            hcsv.row(&[name.to_string(), format!("{}", i), format!("{:.10e}", v)]);
        }
    }
    hcsv.write(dir, "t2_sfbs_hessian.csv");
    // (iii) the generators that leave a field unchanged: the Gram matrix (4 x 4) of the actions of F (d -> 2i d) and of
    // T^iota (d -> -e_iota x d), seen as vectors of R^6
    let gram = |d: &[C64; 3]| -> CMat {
        let mut cols: Vec<[C64; 3]> = vec![[d[0] * c(0.0, 2.0), d[1] * c(0.0, 2.0), d[2] * c(0.0, 2.0)]];
        for i in 0..3 {
            let mut e = [0.0; 3];
            e[i] = 1.0;
            let x = cross_c(&e, d);
            cols.push([-x[0], -x[1], -x[2]]);
        }
        let mut mtx = CMat::zeros(4);
        for a in 0..4 {
            for b in 0..4 {
                let ra = to_r6(&cols[a]);
                let rb = to_r6(&cols[b]);
                mtx.set(a, b, c(ra.iter().zip(&rb).map(|(x, y)| x * y).sum(), 0.0));
            }
        }
        mtx
    };
    let (evg, vecg) = eigh(&gram(&from_r6(&vac)));
    let (evs, vecs) = eigh(&gram(&from_r6(&sad)));
    // the null vector at the vacuum is T^3 = (0, 0, 0, 1); at the non-unitary field it is proportional to T^3 - F/2
    let nv: Vec<f64> = (0..4).map(|i| vecg.get(i, 0).re).collect();
    let ns: Vec<f64> = (0..4).map(|i| vecs.get(i, 0).re).collect();
    let nv_dev = (nv[3].abs() - 1.0).abs() + nv[0].abs() + nv[1].abs() + nv[2].abs();
    let ns_ratio = ns[0] / ns[3];
    let stab_ok = evg[0].abs() < 1e-14 && evg[1] > 1e-3 && nv_dev < 1e-12 && evs[0].abs() < 1e-14 && evs[1] > 1e-3 && (ns_ratio + 0.5).abs() < 1e-12;
    // (iv) the discrete element of the stabilizer: thF = pi/2, U = i tau_1 maps the vacuum to itself and reverses e_3
    let u = sigma(1).scale(c(0.0, 1.0));
    let r = adjoint(&u);
    let ph = C64::cis(2.0 * PI / 2.0);
    let dv = from_r6(&vac);
    let mut dimg = [C64::ZERO; 3];
    for i in 0..3 {
        let mut z = C64::ZERO;
        for k in 0..3 {
            z += dv[k].scale(r[i][k]);
        }
        dimg[i] = ph * z;
    }
    let disc_dev = (0..3).fold(0.0f64, |a, i| a.max((dimg[i] - dv[i]).abs()));
    let r33 = r[2][2];
    // (v) the plane of the normal form, d = (0, i x2, x1), is kept by the gradient flow (11.4): at 20 random points of the
    // plane the gradient has no component along Re d^1, Im d^1, Re d^2, Im d^3, and its components along Re d^3 and Im d^2
    // are Omega_s'(x1 + x2) + Omega_s'(x1 - x2) and Omega_s'(x1 + x2) - Omega_s'(x1 - x2)
    let mut plane_dev = 0.0f64;
    for _ in 0..20 {
        let x1 = 0.05 + 0.3 * rng.unit().abs();
        let x2 = 0.3 * rng.unit().abs();
        let x = [0.0, 0.0, 0.0, x2, x1, 0.0];
        let gr = grad6(&f, &x, 1e-6);
        let du = omega_s_derivs(x1 + x2, g, m)[0];
        let dw = omega_s_derivs(x1 - x2, g, m)[0];
        plane_dev = plane_dev.max(gr[0].abs()).max(gr[1].abs()).max(gr[2].abs()).max(gr[5].abs());
        plane_dev = plane_dev.max((gr[4] - (du + dw)).abs()).max((gr[3] - (du - dw)).abs());
    }
    // (vi) the typed matrices of (11.6) and (11.8): the first-order change of a random field under the transformation with the
    // angles t thF, t th^1, t th^2, t th^3 (central difference in t, step 1e-5) equals the sum of the four matrices of (11.6)
    // applied to the field; and the two families of (11.8) equal exp(2i thF) Rv(U) computed from the definition, and keep (0, 0, 1)
    let mut gen_dev = 0.0f64;
    let mut stab_dev = 0.0f64;
    for _ in 0..20 {
        let dd = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
        let (tf, t1, t2, t3) = (rng.unit(), rng.unit(), rng.unit(), rng.unit());
        let tr = |tt: f64| -> [C64; 3] {
            let r = adjoint(&su2(&[tt * t1, tt * t2, tt * t3]));
            let ph = C64::cis(2.0 * tt * tf);
            let mut o = [C64::ZERO; 3];
            for i in 0..3 {
                for k in 0..3 {
                    o[i] += ph * dd[k].scale(r[i][k]);
                }
            }
            o
        };
        let hs = 1e-5;
        let (ap, am) = (tr(hs), tr(-hs));
        // the matrices of (11.6): 2i thF 1 + th^1 [[0,0,0],[0,0,1],[0,-1,0]] + th^2 [[0,0,-1],[0,0,0],[1,0,0]] + th^3 [[0,1,0],[-1,0,0],[0,0,0]]
        let pred = [
            c(0.0, 2.0 * tf) * dd[0] - dd[2].scale(t2) + dd[1].scale(t3),
            c(0.0, 2.0 * tf) * dd[1] + dd[2].scale(t1) - dd[0].scale(t3),
            c(0.0, 2.0 * tf) * dd[2] - dd[1].scale(t1) + dd[0].scale(t2),
        ];
        for i in 0..3 {
            gen_dev = gen_dev.max(((ap[i] - am[i]).scale(0.5 / hs) - pred[i]).abs());
        }
        // (11.8)
        let th3 = 3.0 * t3;
        let (co, si) = (th3.cos(), th3.sin());
        let m1 = [[co, si, 0.0], [-si, co, 0.0], [0.0, 0.0, 1.0]];
        let m2 = [[-co, -si, 0.0], [-si, co, 0.0], [0.0, 0.0, 1.0]];
        let r1 = adjoint(&su2(&[0.0, 0.0, th3]));
        let r2 = adjoint(&sigma(1).scale(c(0.0, 1.0)).mul(&su2(&[0.0, 0.0, th3])));
        for i in 0..3 {
            for k in 0..3 {
                stab_dev = stab_dev.max((r1[i][k] - m1[i][k]).abs());
                stab_dev = stab_dev.max((-r2[i][k] - m2[i][k]).abs());
            }
        }
        stab_dev = stab_dev.max((m1[2][2] - 1.0).abs()).max((m2[2][2] - 1.0).abs()).max(m1[0][2].abs()).max(m2[1][2].abs());
    }
    let ok = plane_dev < 1e-11
        && gen_dev < 1e-9
        && stab_dev < 1e-14
        && worst_abs < 1e-6 && worst_zeta < 1e-6 && worst_om < 1e-14 && grad_max < 1e-10 && vac_ok && sad_ok && zero_ok && stab_ok && disc_dev < 1e-15 && (r33 + 1.0).abs() < 1e-15;
    ch.add(
        "SB-13",
        "the stationary points of the effective potential at mu = 0, G = 22.5 (m = 0.2, Lambda = 1; closed form of Section 10.4): 40 minimizations (BFGS, numerical gradients) from random fields in the six real dimensions end at |d| = Delta_vac and zeta = 0 (tol 1e-6) with Omega = Omega_min (tol 1e-14); the gradient vanishes at the vacuum Delta_vac e_3 and at the non-unitary field (Delta_vac/2)(1, i, 0) (tol 1e-10); the Hessian at the vacuum has three zero eigenvalues and three equal to 2 Omega_s^(2)(Delta_vac) (relative tol 1e-6), at the non-unitary field two negative, three zero and one positive eigenvalue, with Omega = Omega_min/2 there (tol 1e-12), and at d = 0 it is 2 a_GL < 0 times the unit matrix (radial Richardson second difference, relative tol 1e-5); the Gram matrix of the actions of F, T^1, T^2, T^3 on the vacuum has rank 3 with the null vector T^3, on the non-unitary field rank 3 with the null vector T^3 - F/2 (tol 1e-12); thF = pi/2, U = i tau_1 leaves the vacuum unchanged and reverses the third isospin axis; at 20 random points of the plane d = (0, i x2, x1) the gradient lies in the plane and equals (Omega_s'(x1 + x2) + Omega_s'(x1 - x2), Omega_s'(x1 + x2) - Omega_s'(x1 - x2)) in the coordinates (x1, x2) (tol 1e-11); the four matrices of the generators (11.6) give the first-order change of 20 random fields under random transformations (central differences, tol 1e-9), and the two families of the stabilizer (11.8) equal exp(2i thF) Rv(U) from its definition for 20 random angles th^3 (tol 1e-14)",
        ok,
        format!(
            "minimizations: largest ||d| - Delta_vac| {}, largest zeta {}, largest |Omega - Omega_min| {}, iterations {} to {}; largest gradient at the two stationary fields {}; Hessian eigenvalues at the vacuum {}, predicted 2 Omega_s^(2) = {:.8e}; at the non-unitary field {}; Omega there {:.8e} = {:.12} Omega_min; at d = 0: 2 a_GL = {:.8e} (radial second difference gives a_GL = {:.8e}); Gram eigenvalues at the vacuum {}, at the non-unitary field {}; null vector at the non-unitary field: F component / T^3 component = {:.12}; discrete element: deviation {}, Rv(i tau_1)_33 = {}; plane of the normal form: largest deviation of the gradient {}; generators (11.6) {}; stabilizer (11.8) {}",
            sci(worst_abs),
            sci(worst_zeta),
            sci(worst_om),
            iters.iter().min().unwrap(),
            iters.iter().max().unwrap(),
            sci(grad_max),
            ev_vac.iter().map(|v| format!("{:.6e}", v)).collect::<Vec<_>>().join(", "),
            v_pred,
            ev_sad.iter().map(|v| format!("{:.6e}", v)).collect::<Vec<_>>().join(", "),
            om_sad,
            om_sad / om_min,
            2.0 * a_gl,
            rad,
            evg.iter().map(|v| format!("{:.3e}", v)).collect::<Vec<_>>().join(", "),
            evs.iter().map(|v| format!("{:.3e}", v)).collect::<Vec<_>>().join(", "),
            ns_ratio,
            sci(disc_dev),
            r33,
            sci(plane_dev),
            sci(gen_dev),
            sci(stab_dev)
        ),
    );
    // the landscape in the plane of the two gaps and the velocity of the gradient flow in that plane (Figure 24(a)):
    // on the plane of the normal form, d = (0, i x2, x1), Omega = Omega_s(x1 + x2) + Omega_s(x1 - x2), and the gradient
    // flow dx/dt = -grad Omega of the six real dimensions stays in the plane and moves the gaps Delta^+- = x1 +- x2 with
    // the velocities -2 Omega_s'(Delta^+) and -2 Omega_s'(Delta^-)
    let mut land = Csv::new(
        "the effective potential at mu = 0, G = 22.5 as a function of the two gaps, Omega_s(Delta^+) + Omega_s(Delta^-), on a grid of the plane (Delta^+, Delta^-) (the fields have Delta^+ >= Delta^-; the function is symmetric), with the two components -2 Omega_s'(Delta^+), -2 Omega_s'(Delta^-) of the velocity of the gradient flow in that plane (Test 2, item 9, Figure 24(a))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", format!("{}", g)), ("Delta_vac", format!("{:.10}", dvac)), ("Omega_min", format!("{:.12e}", om_min))],
        "Delta_+,Delta_-,Omega,flow_+,flow_-",
    );
    for i in 0..=70 {
        for j in 0..=70 {
            let (x, y) = (0.35 * i as f64 / 70.0, 0.35 * j as f64 / 70.0);
            land.row(&[
                format!("{:.6}", x),
                format!("{:.6}", y),
                format!("{:.12e}", omega_s(x, g, m) + omega_s(y, g, m)),
                format!("{:.12e}", -2.0 * omega_s_derivs(x, g, m)[0]),
                format!("{:.12e}", -2.0 * omega_s_derivs(y, g, m)[0]),
            ]);
        }
    }
    land.write(dir, "t2_sfbs_landscape.csv");
}

/// SB-14: SFBSv, the expansion about the minimum
fn sb14(ch: &mut Checks, dir: &PathBuf) {
    let m = MASS;
    let g = G_VAC;
    let dvac = gap_mu0(g, m);
    let mut rng = Rng(16180);
    // (i) the parametrization and the invariance: for random SFBSv fields, Omega(d) equals the sector form
    let sfbsv = |t0: f64, t1: f64, t2: f64, h: f64, c1: f64, c2: f64| -> [C64; 3] {
        let base = [c(0.0, c1), c(0.0, c2), c(dvac + h, 0.0)];
        let r = adjoint(&su2(&[t1, t2, 0.0]));
        let ph = C64::cis(2.0 * t0);
        let mut out = [C64::ZERO; 3];
        for i in 0..3 {
            let mut z = C64::ZERO;
            for k in 0..3 {
                z += base[k].scale(r[i][k]);
            }
            out[i] = ph * z;
        }
        out
    };
    let mut par_dev = 0.0f64;
    for _ in 0..50 {
        let (t0, t1, t2) = (3.0 * rng.unit(), 3.0 * rng.unit(), 3.0 * rng.unit());
        let (h, c1, c2) = (0.08 * rng.unit(), 0.08 * rng.unit(), 0.08 * rng.unit());
        let d = sfbsv(t0, t1, t2, h, c1, c2);
        let rho = (c1 * c1 + c2 * c2).sqrt();
        let a = omega_cl(&d, g, m);
        let b = omega_s(dvac + h + rho, g, m) + omega_s(dvac + h - rho, g, m);
        par_dev = par_dev.max((a - b).abs());
    }
    // (i-b) the explicit forms (11.8), (11.9), (11.10) of SFBSv: the 2 x 2 matrix U_G, the 3 x 3 rotation Rv(U_G) and the
    // three components of d, against the matrix exponential, the adjoint map and the product computed by sfbsv
    let mut expl_dev = 0.0f64;
    for _ in 0..50 {
        let (t0, t1, t2) = (3.0 * rng.unit(), 3.0 * rng.unit(), 3.0 * rng.unit());
        let (h, c1, c2) = (0.08 * rng.unit(), 0.08 * rng.unit(), 0.08 * rng.unit());
        let th = (t1 * t1 + t2 * t2).sqrt();
        let (n1, n2) = (t1 / th, t2 / th);
        let (co, si) = (th.cos(), th.sin());
        let (chh, shh) = ((0.5 * th).cos(), (0.5 * th).sin());
        // (11.8): U_G = [[cos(th/2), i sin(th/2) (n1 - i n2)], [i sin(th/2) (n1 + i n2), cos(th/2)]]
        let ug = CMat::m2(c(chh, 0.0), c(0.0, shh) * c(n1, -n2), c(0.0, shh) * c(n1, n2), c(chh, 0.0));
        expl_dev = expl_dev.max(ug.sub(&su2(&[t1, t2, 0.0])).max_abs());
        // (11.9): Rv(U_G), the rotation by the angle th about the axis -(n1, n2, 0)
        let rx = [
            [co + (1.0 - co) * n1 * n1, (1.0 - co) * n1 * n2, -si * n2],
            [(1.0 - co) * n1 * n2, co + (1.0 - co) * n2 * n2, si * n1],
            [si * n2, -si * n1, co],
        ];
        let ra = adjoint(&su2(&[t1, t2, 0.0]));
        for i in 0..3 {
            for k in 0..3 {
                expl_dev = expl_dev.max((rx[i][k] - ra[i][k]).abs());
            }
        }
        // (11.10): the three components of d
        let ph = C64::cis(2.0 * t0);
        let a = dvac + h;
        let dx = [
            ph * c(-si * n2 * a, (co + (1.0 - co) * n1 * n1) * c1 + (1.0 - co) * n1 * n2 * c2),
            ph * c(si * n1 * a, (1.0 - co) * n1 * n2 * c1 + (co + (1.0 - co) * n2 * n2) * c2),
            ph * c(co * a, si * n2 * c1 - si * n1 * c2),
        ];
        let d = sfbsv(t0, t1, t2, h, c1, c2);
        for i in 0..3 {
            expl_dev = expl_dev.max((dx[i] - d[i]).abs());
        }
    }
    // (i-c) the linear order (11.11): the derivatives of the SFBSv field at the vacuum with respect to thG0, thG1, thG2,
    // h_a, c_n1, c_n2 (central differences with the step 1e-6)
    let e6 = 1e-6;
    let pred: [[C64; 3]; 6] = [
        [C64::ZERO, C64::ZERO, c(0.0, 2.0 * dvac)],
        [C64::ZERO, c(dvac, 0.0), C64::ZERO],
        [c(-dvac, 0.0), C64::ZERO, C64::ZERO],
        [C64::ZERO, C64::ZERO, c(1.0, 0.0)],
        [c(0.0, 1.0), C64::ZERO, C64::ZERO],
        [C64::ZERO, c(0.0, 1.0), C64::ZERO],
    ];
    let mut lin_dev = 0.0f64;
    for k in 0..6 {
        let mut p = [0.0; 6];
        p[k] = e6;
        let dp = sfbsv(p[0], p[1], p[2], p[3], p[4], p[5]);
        let dm = sfbsv(-p[0], -p[1], -p[2], -p[3], -p[4], -p[5]);
        for i in 0..3 {
            lin_dev = lin_dev.max(((dp[i] - dm[i]).scale(0.5 / e6) - pred[k][i]).abs());
        }
    }
    // (i-d) the residual symmetry (11.15): the rotation Rv(exp(i al tau_3/2)) of U(1)^Q maps the SFBSv field to the SFBSv
    // field with thG1 - i thG2 and c_n1 - i c_n2 multiplied by exp(i al) (charge +1) and thG0, h_a unchanged; the element
    // thF = pi/2, U = i tau_1 (which acts on d as -Rv(i tau_1)) maps it to the field with thG2 -> -thG2, c_n1 -> -c_n1
    let rot3 = |z: &[C64; 3], r: &[[f64; 3]; 3]| -> [C64; 3] {
        let mut o = [C64::ZERO; 3];
        for i in 0..3 {
            for k in 0..3 {
                o[i] += z[k].scale(r[i][k]);
            }
        }
        o
    };
    let mut chg_dev = 0.0f64;
    let rz = adjoint(&sigma(1).scale(c(0.0, 1.0)));
    for _ in 0..30 {
        let (t0, t1, t2) = (rng.unit(), rng.unit(), rng.unit());
        let (h, c1, c2) = (0.08 * rng.unit(), 0.08 * rng.unit(), 0.08 * rng.unit());
        let al = 3.0 * rng.unit();
        let d = sfbsv(t0, t1, t2, h, c1, c2);
        let lhs = rot3(&d, &adjoint(&su2(&[0.0, 0.0, al])));
        let tq = c(t1, -t2) * C64::cis(al);
        let cq = c(c1, -c2) * C64::cis(al);
        let rhs = sfbsv(t0, tq.re, -tq.im, h, cq.re, -cq.im);
        let lz = rot3(&d, &rz);
        let rhs2 = sfbsv(t0, t1, -t2, h, -c1, c2);
        for i in 0..3 {
            chg_dev = chg_dev.max((lhs[i] - rhs[i]).abs());
            chg_dev = chg_dev.max((-lz[i] - rhs2[i]).abs());
        }
    }
    // (ii) the derivatives of Omega_s at Delta_vac in closed form against numerical derivatives
    let ds = omega_s_derivs(dvac, g, m);
    let f = |y: f64| omega_s(y, g, m);
    let hh = 1e-3;
    let n2 = (f(dvac + hh) - 2.0 * f(dvac) + f(dvac - hh)) / (hh * hh);
    // third and fourth derivatives by central differences with the steps 0.02 and 0.01, Richardson-extrapolated (the
    // error of each difference is proportional to the square of the step)
    let d3 = |h: f64| (f(dvac + 2.0 * h) - 2.0 * f(dvac + h) + 2.0 * f(dvac - h) - f(dvac - 2.0 * h)) / (2.0 * h.powi(3));
    let d4 = |h: f64| (f(dvac + 2.0 * h) - 4.0 * f(dvac + h) + 6.0 * f(dvac) - 4.0 * f(dvac - h) + f(dvac - 2.0 * h)) / h.powi(4);
    let n3 = (4.0 * d3(0.01) - d3(0.02)) / 3.0;
    let n4 = (4.0 * d4(0.01) - d4(0.02)) / 3.0;
    let der_dev = [((n2 - ds[1]) / ds[1]).abs(), ((n3 - ds[2]) / ds[2]).abs(), ((n4 - ds[3]) / ds[3]).abs()];
    // (ii-b) the third and fourth derivatives of Xi in closed form against quadrature, at M_q^2 and at m^2; and the closed
    // form (11.13) of the curvature, Omega_s^(2) = -4 Delta_vac^2 Xi^(2)(m^2 + Delta_vac^2), which follows from the gap equation
    let (qn0, qw0) = radial_nodes(m, 0.0, 0.05);
    let mut xi_dev = 0.0f64;
    for &mm2 in &[m * m + dvac * dvac, m * m] {
        let q3: f64 = qn0.iter().zip(&qw0).map(|(q, w)| 0.75 * w * (q * q + mm2).powf(-2.5)).sum();
        let q4: f64 = qn0.iter().zip(&qw0).map(|(q, w)| -1.875 * w * (q * q + mm2).powf(-3.5)).sum();
        xi_dev = xi_dev.max(((q3 - xi_cl_d3(mm2)) / xi_cl_d3(mm2)).abs());
        xi_dev = xi_dev.max(((q4 - xi_cl_d4(mm2)) / xi_cl_d4(mm2)).abs());
    }
    let mq2 = m * m + dvac * dvac;
    let curv_cf = -4.0 * dvac * dvac * xi_cl_d2(mq2);
    let curv_dev = ((curv_cf - ds[1]) / ds[1]).abs();
    // the potential of the Higgs form with the same minimum and the same curvature along h_a (11.16):
    // lambda_H (|d|^2 - Delta_vac^2)^2 with 4 lambda_H Delta_vac^2 = Omega_s^(2)
    let lam_h = ds[1] / (4.0 * dvac * dvac);
    // (iii) the quartic expansion
    let om_min = 2.0 * omega_s(dvac, g, m);
    let quartic = |h: f64, rho: f64| -> f64 {
        om_min + ds[1] * (h * h + rho * rho) + ds[2] / 3.0 * (h.powi(3) + 3.0 * h * rho * rho) + ds[3] / 12.0 * (h.powi(4) + 6.0 * h * h * rho * rho + rho.powi(4))
    };
    let mut rem = vec![];
    for &s in &[0.02, 0.01, 0.005] {
        let (h, rho) = (0.6 * s, 0.8 * s);
        let exact = omega_s(dvac + h + rho, g, m) + omega_s(dvac + h - rho, g, m);
        rem.push((exact - quartic(h, rho)).abs() / s.powi(5));
    }
    // a remainder of the fifth order: the ratio to s^5 settles (its changes shrink when s is halved)
    let rem_ok = (rem[2] - rem[1]).abs() < 0.6 * (rem[1] - rem[0]).abs() + 0.05 * rem[2];
    // (iv) the curvatures at the vacuum from the Bogoliubov matrices directly, independent of the sector formula
    let (qn, qw) = radial_nodes(m, 0.0, 0.05);
    let ob = |d: &[C64; 3]| omega_bdg(d, g, m, 0.0, &qn, &qw, &[0.0, 0.0, 1.0]);
    let hb = 2e-3;
    let d0 = [C64::ZERO, C64::ZERO, c(dvac, 0.0)];
    let o0 = ob(&d0);
    let second = |dir6: [f64; 6]| -> f64 {
        let mut dp = d0;
        let mut dm = d0;
        let dd = from_r6(&dir6);
        for i in 0..3 {
            dp[i] += dd[i].scale(hb);
            dm[i] -= dd[i].scale(hb);
        }
        (ob(&dp) - 2.0 * o0 + ob(&dm)) / (hb * hb)
    };
    let v_h = second([0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    let v_c = second([0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
    let v_g = second([1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
    let v_ph = second([0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
    let v_pred = 2.0 * ds[1];
    let bdg_ok = ((v_h - v_pred) / v_pred).abs() < 1e-3 && ((v_c - v_pred) / v_pred).abs() < 1e-3 && v_g.abs() < 1e-3 * v_pred && v_ph.abs() < 1e-3 * v_pred;
    let ok = par_dev < 1e-15
        && expl_dev < 1e-14
        && lin_dev < 1e-8
        && chg_dev < 1e-14
        && der_dev.iter().all(|&e| e < 1e-4)
        && xi_dev < 1e-10
        && curv_dev < 1e-10
        && rem_ok
        && bdg_ok
        && ds[0].abs() < 1e-15;
    ch.add(
        "SB-14",
        "SFBSv, the expansion about the vacuum Delta_vac e_3 (mu = 0, G = 22.5, m = 0.2, Lambda = 1): for 50 random fields d = exp(2i thG0) Rv(U(thG1, thG2)) ((Delta_vac + h_a) e_3 + i (c_n1 e_1 + c_n2 e_2)), Omega(d) = Omega_s(Delta_vac + h_a + rho_n) + Omega_s(Delta_vac + h_a - rho_n) with rho_n = (c_n1^2 + c_n2^2)^(1/2), whatever the Goldstone fields thG0, thG1, thG2 (tol 1e-15); the explicit 2 x 2 matrix U(thG1, thG2), the explicit rotation Rv(U(thG1, thG2)) and the explicit three components of d equal the matrix exponential, the adjoint map and the product (50 random fields, tol 1e-14); the derivatives of d at the vacuum with respect to thG0, thG1, thG2, h_a, c_n1, c_n2 are (0, 0, 2i Delta_vac), (0, Delta_vac, 0), (-Delta_vac, 0, 0), (0, 0, 1), (i, 0, 0), (0, i, 0) (central differences, tol 1e-8); the rotation of U(1)^Q by the angle al multiplies thG1 - i thG2 and c_n1 - i c_n2 by exp(i al) and leaves thG0, h_a unchanged, and the element thF = pi/2, U = i tau_1 changes the signs of thG2 and c_n1 (30 random fields and angles, tol 1e-14); the second, third and fourth derivatives of the sector potential Omega_s at Delta_vac in closed form equal numerical derivatives (central differences, Richardson-extrapolated for the third and fourth; relative tol 1e-4), and the first vanishes (tol 1e-15); the remainder of the quartic expansion falls like the fifth power of the fields; the second derivatives of Omega at the vacuum computed from the Bogoliubov matrices directly (finite differences, step 2e-3) are 2 Omega_s^(2) along Re d^3 (h_a) and Im d^1 (c_n1) and zero along Re d^1 and Im d^3 (Goldstone directions) (relative tol 1e-3); the closed forms of Xi^(3) and Xi^(4) equal quadrature at M_q^2 and m^2 (relative tol 1e-10), and Omega_s^(2) = -4 Delta_vac^2 Xi^(2)(m^2 + Delta_vac^2) (relative tol 1e-10)",
        ok,
        format!(
            "parametrization {}; explicit matrices and components {}; linear order {}; charges and discrete element {}; Xi^(3), Xi^(4) against quadrature {}; Omega_s^(2) from (11.13) {:.10e} (deviation {}); the Higgs form with the same curvature: lambda_H = {:.8e}, cubic coefficient 4 lambda_H Delta_vac = {:.8e} against Omega_s^(3)/3 = {:.8e}, quartic coefficient lambda_H against Omega_s^(4)/12 = {:.8e}, depth -lambda_H Delta_vac^4 = {:.8e} against Omega_min = {:.8e}; Omega_s^(1) = {:.3e}, Omega_s^(2) = {:.10e} (numerical {:.10e}), Omega_s^(3) = {:.8e} (numerical {:.8e}), Omega_s^(4) = {:.8e} (numerical {:.8e}); remainder/s^5 at s = 0.02, 0.01, 0.005: {:.6e}, {:.6e}, {:.6e}; from the Bogoliubov matrices: Re d^3 {:.8e}, Im d^1 {:.8e}, Re d^1 {:.3e}, Im d^3 {:.3e} against 2 Omega_s^(2) = {:.8e}",
            sci(par_dev),
            sci(expl_dev),
            sci(lin_dev),
            sci(chg_dev),
            sci(xi_dev),
            curv_cf,
            sci(curv_dev),
            lam_h,
            4.0 * lam_h * dvac,
            ds[2] / 3.0,
            ds[3] / 12.0,
            -lam_h * dvac.powi(4),
            om_min,
            ds[0],
            ds[1],
            n2,
            ds[2],
            n3,
            ds[3],
            n4,
            rem[0],
            rem[1],
            rem[2],
            v_h,
            v_c,
            v_g,
            v_ph,
            v_pred
        ),
    );
    // slices of the SFBSv potential (Figure 24(b))
    let mut sl = Csv::new(
        "the SFBSv potential at mu = 0, G = 22.5 along the amplitude field h_a (rho_n = 0) and along the non-unitary field rho_n (h_a = 0), minus its minimum: exact, quadratic Omega_s^(2) x^2, quartic expansion, and the potential of the Higgs form lambda_H (|d|^2 - Delta_vac^2)^2 with the same curvature along h_a (Test 2, item 9, Figure 24(b))",
        &[("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA)), ("G", format!("{}", g)), ("Delta_vac", format!("{:.10}", dvac)), ("Omega_s2", format!("{:.10e}", ds[1])), ("Omega_s3", format!("{:.10e}", ds[2])), ("Omega_s4", format!("{:.10e}", ds[3])), ("lambda_H", format!("{:.10e}", lam_h))],
        "variable,amplitude,Omega_above_min,Omega_quad,Omega_quartic,Omega_H",
    );
    // h_a from -0.25 to 0.25 (h_a = -Delta_vac is the field d = 0), rho_n from 0 to 0.25; the sector form holds for all
    // values because Omega_s is even
    for var in ["h_a", "rho_n"] {
        let npts = if var == "h_a" { 100 } else { 50 };
        for i in 0..=npts {
            let x = if var == "h_a" { -0.25 + 0.5 * i as f64 / 100.0 } else { 0.25 * i as f64 / 50.0 };
            let (h, rho) = if var == "h_a" { (x, 0.0) } else { (0.0, x) };
            let exact = omega_s(dvac + h + rho, g, m) + omega_s(dvac + h - rho, g, m) - om_min;
            // the Higgs form (11.16): |d|^2 = (Delta_vac + h_a)^2 + rho_n^2
            let dd2 = (dvac + h) * (dvac + h) + rho * rho;
            let higgs = lam_h * (dd2 - dvac * dvac).powi(2);
            sl.row(&[var.to_string(), format!("{:.6}", x), format!("{:.12e}", exact), format!("{:.12e}", ds[1] * x * x), format!("{:.12e}", quartic(h, rho) - om_min), format!("{:.12e}", higgs)]);
        }
    }
    sl.write(dir, "t2_sfbs_sfbsv_slices.csv");
}

fn main() {
    let dir = data_dir();
    let mut ch = Checks { title: "Revision_SuperLoop, folder test2, program `t2_sfbs`: the superfluid boson singlet state (SFBS) of the spinor field Psi_LL, its effective field theory and the Higgs mechanism (Test 2, items 7 to 11)".into(), items: vec![] };
    let mut fr = Frags(vec![]);
    // the gaps of the two states used by the checks
    let d_vac = gap_mu0(G_VAC, MASS);
    let d_fs = gap_quad(G_FS, MASS, MU_FS);
    let d_vac_q = gap_quad(G_VAC, MASS, 0.0);
    println!("gaps: vacuum {} (closed form) {} (quadrature), Fermi sea {}", d_vac, d_vac_q, d_fs);
    // item [7]
    sb1(&mut ch, &mut fr);
    sb2(&mut ch, &mut fr);
    let fdev = frags_item7(&mut fr);
    println!("printed expressions of Section 8 against their matrices: largest deviation {}", sci(fdev));
    assert!(fdev < 1e-14, "a printed expression of Section 8 does not match its matrix");
    sb3(&mut ch);
    sb4(&mut ch, &dir);
    sb5(&mut ch, &dir, d_vac, d_fs);
    // item [8]
    sb6(&mut ch, &mut fr);
    sb7(&mut ch, &dir);
    sb8(&mut ch, &dir);
    sb9(&mut ch);
    frags_item8(&mut fr);
    sb10(&mut ch, &dir);
    // item [8], the effective potential
    sb11(&mut ch, &mut fr);
    sb12(&mut ch, &dir);
    // item [9]
    sb13(&mut ch, &dir);
    sb14(&mut ch, &dir);
    let _ = (lift2, LAMBDA, d_vac_q);
    fr.write(&dir.join("t2_sfbs_fragments.md"));
    let ok = ch.write(&dir.join("t2_sfbs_checks.txt"));
    if !ok {
        std::process::exit(1);
    }
}
