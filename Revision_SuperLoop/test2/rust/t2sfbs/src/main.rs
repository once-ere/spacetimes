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
use t2core::model::{alpha, beta, bq_conj, bq_from, bq_mul, cconj, dirac_from_charges, eps2, gamma, gamma_ch, jgen, kgen, kpair, kpair_ps, lift2, lift4, rand_sl2c, sigma, spin_gen, tiso};
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
    let _ = (lift2, LAMBDA, d_vac_q);
    fr.write(&dir.join("t2_sfbs_fragments.md"));
    let ok = ch.write(&dir.join("t2_sfbs_checks.txt"));
    if !ok {
        std::process::exit(1);
    }
}
