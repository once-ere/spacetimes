//! explain_superfluid -- PROJECT spacetimes, Revision_SuperLoop, folder `explanations`.
//!
//! The manuscript of Revision_SuperLoop contains the sentence
//!     "The spin-zero superfluid of its spinor charges condenses in an isospin-triplet, unitary,
//!      electrically neutral state, and the pair that is a singlet of the whole super-loop
//!      requires two species."
//! The document "The spin-zero superfluid of the spinor charges - an explanation" of the folder
//! explanations (.md, .tex, .pdf) explains this sentence word by word.  This program computes
//! every example of that document and checks every statement that the document makes about
//! them (checks X-1 to X-10, report explanations/data/ex_superfluid_checks.txt).
//!
//! Conventions (those of the program superloop of Revision_SuperLoop):
//!   * Pauli matrices sigma_0 (unit), sigma_1, sigma_2, sigma_3; eps2 = i sigma_2 = [[0, 1], [-1, 0]];
//!     tau_1, tau_2, tau_3 are the same matrices acting on the isospin index.
//!   * chiral Dirac matrices gamma^0 = [[0, sigma_0], [sigma_0, 0]], gamma^j = [[0, sigma_j], [-sigma_j, 0]],
//!     gamma_ch = i gamma^0 gamma^1 gamma^2 gamma^3 = diag(-1, -1, 1, 1),
//!     C_LL = i gamma^2 gamma^0 = diag(eps2, -eps2), C_LL gamma_ch = -diag(eps2, eps2).
//!   * the field Psi_LL has 8 components, Dirac (4) (x) isospin (2); isospin index 1 = up
//!     (T_LL^3 = +1/2, electric charge Q_LL = +1/2), index 2 = down (-1/2).  With two species a
//!     species index (2) follows.  A pair operator is Psi_LL^T Kch Psi_LL with an 8 x 8 (16 x 16)
//!     matrix Kch.
//!   * The spin-zero triplet condensate is described by the complex isospin vector dvec (components
//!     d_iota).  The condensed pairs have the isospin wavefunction -sum_iota d_iota tau_iota eps2, a 2 x 2
//!     matrix whose entry (s, s') is the amplitude of a pair in which one particle has the isospin
//!     state s and the other s' (1 = up, 2 = down).  The 2 x 2 gap matrix of the Bogoliubov-de Gennes
//!     equations is sum_iota d_iota (eps2 tau_iota)^dag.
//!
//! Usage (from the folder Revision_SuperLoop/explanations):
//!     cargo run --release --manifest-path code/explain-rs/Cargo.toml
//! Output, in explanations/data: ex_pair_matrices.csv, ex_states.csv, ex_two_particle_counting.csv
//! and the report ex_superfluid_checks.txt.  The program exits with status 1 if a check fails.
//! Arithmetic: double precision; every matrix entry is 0, +-1, +-i or +-1/2 (or a small integer),
//! so all products are exact or rounded at the level 1e-16; equalities are tested with 1e-12.

use spacetimes::complex::C64;
use spacetimes::io::write_csv_str;
use spacetimes::linalg::{eigh, CMat};
use std::f64::consts::{FRAC_1_SQRT_2, PI};
use std::fs;
use std::io::Write;

const TOL: f64 = 1e-12;

// ------------------------------------------------------------------ the report

/// A list of checks with a title; the report has the format of the reports of the program
/// superloop: title, "N of M checks passed", then one line per check.
struct Checks {
    title: String,
    items: Vec<(String, String, bool, String)>,
}

impl Checks {
    fn new(title: &str) -> Checks {
        Checks { title: title.to_string(), items: Vec::new() }
    }
    fn add(&mut self, id: &str, statement: &str, ok: bool, detail: String) {
        let d = if detail.is_empty() { String::new() } else { format!("  -- {}", detail) };
        println!("[{}] {}  {}{}", if ok { "PASS" } else { "FAIL" }, id, statement, d);
        self.items.push((id.to_string(), statement.to_string(), ok, detail));
    }
    fn write(&self, name: &str) -> bool {
        let n = self.items.iter().filter(|x| x.2).count();
        let mut s = format!("{}\n{} of {} checks passed\n\n", self.title, n, self.items.len());
        for (id, st, ok, d) in &self.items {
            let d = if d.is_empty() { String::new() } else { format!("  -- {}", d) };
            s += &format!("[{}] {}  {}{}\n", if *ok { "PASS" } else { "FAIL" }, id, st, d);
        }
        let path = spacetimes::io::data_dir().join(name);
        let mut f = fs::File::create(&path).expect("cannot write the report");
        f.write_all(s.as_bytes()).expect("cannot write the report");
        println!("wrote {} ({} of {} checks passed)", path.display(), n, self.items.len());
        self.items.iter().all(|x| x.2)
    }
}

// ------------------------------------------------------------------ small complex matrices

fn c(re: f64, im: f64) -> C64 {
    C64::new(re, im)
}

/// n x n matrix from its entries, row by row
fn mat(n: usize, e: &[C64]) -> CMat {
    let mut m = CMat::zeros(n);
    for i in 0..n {
        for j in 0..n {
            m.set(i, j, e[i * n + j]);
        }
    }
    m
}

/// Kronecker product a (x) b; the index of b runs fastest
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

/// complex multiple of a matrix
fn smul(m: &CMat, s: C64) -> CMat {
    let mut r = m.clone();
    for z in r.a.iter_mut() {
        *z = *z * s;
    }
    r
}

/// the matrix [[a, b], [cc, d]] of four equal square blocks
fn block2(a: &CMat, b: &CMat, cc: &CMat, d: &CMat) -> CMat {
    let k = a.n;
    let mut m = CMat::zeros(2 * k);
    for i in 0..k {
        for j in 0..k {
            m.set(i, j, a.get(i, j));
            m.set(i, k + j, b.get(i, j));
            m.set(k + i, j, cc.get(i, j));
            m.set(k + i, k + j, d.get(i, j));
        }
    }
    m
}

fn close(a: &CMat, b: &CMat) -> bool {
    a.sub(b).fro() < TOL
}

fn is_zero(a: &CMat) -> bool {
    a.fro() < TOL
}

/// exchange sign of a pair matrix: +1 if Kch^T = Kch (symmetric), -1 if Kch^T = -Kch (antisymmetric), 0 otherwise
fn exchange_sign(m: &CMat) -> i32 {
    let t = m.transpose();
    if close(&t, m) {
        1
    } else if close(&t, &m.scale(-1.0)) {
        -1
    } else {
        0
    }
}

/// The pair Psi^T Kch Psi of ANTICOMMUTING components is the sum over the pairs of different
/// components (first index < second index) of (Kch_(first,second) - Kch_(second,first)) times the
/// product of the two components (the diagonal terms vanish, because the square of an
/// anticommuting component is zero).  This function counts the terms with a non-zero coefficient.
fn pair_terms(k: &CMat) -> usize {
    let mut n = 0;
    for r in 0..k.n {
        for s in (r + 1)..k.n {
            if (k.get(r, s) - k.get(s, r)).abs() > TOL {
                n += 1;
            }
        }
    }
    n
}

/// the change of the pair matrix Kch under a generator t of the field (Psi -> Psi + eps t Psi):
/// Kch -> t^T Kch + Kch t
fn ad(t: &CMat, k: &CMat) -> CMat {
    t.transpose().mul(k).add(&k.mul(t))
}

/// the charge q of a pair matrix, if ad_(t3)(Kch) = q Kch (the pair operator annihilates two
/// particles whose charges add up to q)
fn eigen_charge(t3: &CMat, k: &CMat) -> Option<i32> {
    let a = ad(t3, k);
    for q in [-2, -1, 0, 1, 2] {
        if close(&a, &k.scale(q as f64)) {
            return Some(q);
        }
    }
    None
}

/// the Casimir sum_k ad_(t_k)(ad_(t_k)(Kch)) of Hermitian generators t_k: equals I(I+1) Kch for
/// a pair of total isospin (or spin) I
fn casimir(ts: &[CMat], k: &CMat) -> CMat {
    let mut acc = CMat::zeros(k.n);
    for t in ts {
        acc = acc.add(&ad(t, &ad(t, k)));
    }
    acc
}

/// the factor x for which Casimir(Kch) = x Kch, if any
fn casimir_value(ts: &[CMat], k: &CMat) -> Option<f64> {
    let cm = casimir(ts, k);
    for x in [0.0, 0.75, 2.0, 6.0] {
        if close(&cm, &k.scale(x)) {
            return Some(x);
        }
    }
    None
}

/// complex square root (principal branch)
fn csqrt(z: C64) -> C64 {
    C64::cis(0.5 * z.arg()) * z.abs().sqrt()
}

/// a number for the reports: integers without decimals, others with up to 6 decimals
fn fnum(x: f64) -> String {
    let x = if x.abs() < 1e-12 { 0.0 } else { x };
    if (x - x.round()).abs() < 1e-12 {
        format!("{}", x.round() as i64)
    } else {
        let s = format!("{:.6}", x);
        s.trim_end_matches('0').to_string()
    }
}

/// a complex number for the reports, e.g. 0.707107, 0.707107 i, 1 + 0.5 i
fn fcplx(z: C64) -> String {
    let (re, im) = (if z.re.abs() < 1e-12 { 0.0 } else { z.re }, if z.im.abs() < 1e-12 { 0.0 } else { z.im });
    if im == 0.0 {
        fnum(re)
    } else if re == 0.0 {
        if (im - 1.0).abs() < 1e-12 {
            "i".to_string()
        } else if (im + 1.0).abs() < 1e-12 {
            "-i".to_string()
        } else {
            format!("{} i", fnum(im))
        }
    } else if im > 0.0 {
        format!("{} + {} i", fnum(re), fnum(im))
    } else {
        format!("{} - {} i", fnum(re), fnum(-im))
    }
}

fn fvec(v: &[C64]) -> String {
    format!("({})", v.iter().map(|z| fcplx(*z)).collect::<Vec<_>>().join(", "))
}

fn freal(v: &[f64]) -> String {
    format!("({})", v.iter().map(|x| fnum(*x)).collect::<Vec<_>>().join(", "))
}

/// deterministic pseudo-random generator (SplitMix64)
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// uniform integer in [-k, k]
    fn int(&mut self, k: i64) -> i64 {
        (self.next() % (2 * k as u64 + 1)) as i64 - k
    }
    /// uniform real in [-1, 1)
    fn unit(&mut self) -> f64 {
        2.0 * ((self.next() >> 11) as f64 / (1u64 << 53) as f64) - 1.0
    }
}

// ------------------------------------------------------------------ quaternions with complex coefficients

/// a bi-quaternion q0 e0 + q1 e1 + q2 e2 + q3 e3 (complex coefficients)
type Q = [C64; 4];

/// the product of the quaternion units: e_j e_j = -e0, e1 e2 = -e2 e1 = e3 and cyclically
fn qmul(p: &Q, q: &Q) -> Q {
    [
        p[0] * q[0] - p[1] * q[1] - p[2] * q[2] - p[3] * q[3],
        p[0] * q[1] + p[1] * q[0] + p[2] * q[3] - p[3] * q[2],
        p[0] * q[2] - p[1] * q[3] + p[2] * q[0] + p[3] * q[1],
        p[0] * q[3] + p[1] * q[2] - p[2] * q[1] + p[3] * q[0],
    ]
}

/// the conjugation bS -> bSbar (changes the signs of the coefficients of e1, e2, e3; complex-linear)
fn qbar(p: &Q) -> Q {
    [p[0], -p[1], -p[2], -p[3]]
}

fn qadd(p: &Q, q: &Q) -> Q {
    [p[0] + q[0], p[1] + q[1], p[2] + q[2], p[3] + q[3]]
}

fn qsub(p: &Q, q: &Q) -> Q {
    [p[0] - q[0], p[1] - q[1], p[2] - q[2], p[3] - q[3]]
}

fn qeq(p: &Q, q: &Q) -> bool {
    (0..4).all(|k| (p[k] - q[k]).abs() < TOL)
}

/// the polarized norm N_LL(p, q) = sum of the products of the coefficients (complex-bilinear, no conjugation)
fn qnorm(p: &Q, q: &Q) -> C64 {
    p[0] * q[0] + p[1] * q[1] + p[2] * q[2] + p[3] * q[3]
}

/// the bi-octonion product (Cayley-Dickson) of x = a + b e4 and y = c + d e4, written as pairs (a, b):
/// (a + b e4)(c + d e4) = (a c - dbar b) + (d a + b cbar) e4
fn cd(x: &(Q, Q), y: &(Q, Q)) -> (Q, Q) {
    let (a, b) = x;
    let (cc, d) = y;
    (qsub(&qmul(a, cc), &qmul(&qbar(d), b)), qadd(&qmul(d, a), &qmul(b, &qbar(cc))))
}

/// the complex 2 x 2 matrix M_LL(q) = q0 sigma_0 - i (q1 sigma_1 + q2 sigma_2 + q3 sigma_3)
fn mq(q: &Q, s: &[CMat; 4]) -> CMat {
    let mut m = smul(&s[0], q[0]);
    for k in 1..4 {
        m = m.add(&smul(&s[k], q[k] * c(0.0, -1.0)));
    }
    m
}

fn rand_q(rng: &mut Rng) -> Q {
    let mut q = [C64::ZERO; 4];
    for z in q.iter_mut() {
        *z = c(rng.int(3) as f64, rng.int(3) as f64);
    }
    q
}

// ------------------------------------------------------------------ main

fn main() {
    if !std::path::Path::new("code/explain-rs/Cargo.toml").exists() {
        eprintln!("explain_superfluid: run this program from the folder Revision_SuperLoop/explanations");
        std::process::exit(2);
    }
    let mut ck = Checks::new("Revision_SuperLoop, folder explanations, program `explain_superfluid`: the examples of the explanation of the sentence on the spin-zero superfluid of the spinor charges");
    let z = C64::ZERO;
    let one = C64::ONE;
    let ii = C64::I;

    // ---------------- the matrices
    let s: [CMat; 4] = [
        mat(2, &[one, z, z, one]),
        mat(2, &[z, one, one, z]),
        mat(2, &[z, -ii, ii, z]),
        mat(2, &[one, z, z, -one]),
    ];
    let eps2 = mat(2, &[z, one, -one, z]);
    let tau = [s[1].clone(), s[2].clone(), s[3].clone()];
    let z2 = CMat::zeros(2);
    let i2 = CMat::identity(2);
    let i4 = CMat::identity(4);
    let gam: Vec<CMat> = (0..4)
        .map(|k| if k == 0 { block2(&z2, &s[0], &s[0], &z2) } else { block2(&z2, &s[k], &s[k].scale(-1.0), &z2) })
        .collect();
    let gch = smul(&gam[0].mul(&gam[1]).mul(&gam[2]).mul(&gam[3]), ii);
    let cc = smul(&gam[2].mul(&gam[0]), ii);
    let cgch = cc.mul(&gch);
    let basics = close(&gch, &CMat::diag(&[-1.0, -1.0, 1.0, 1.0]))
        && close(&cc, &block2(&eps2, &z2, &z2, &eps2.scale(-1.0)))
        && close(&cgch, &block2(&eps2, &z2, &z2, &eps2).scale(-1.0));

    // the generators of the field: Lorentz (rotations and boosts) on the Dirac index, isospin on the isospin index
    let mut lorentz8: Vec<CMat> = Vec::new();
    for k in 1..4 {
        let rot = smul(&s[k], c(0.0, -0.5));
        lorentz8.push(kron(&block2(&rot, &z2, &z2, &rot), &i2));
        lorentz8.push(kron(&block2(&s[k].scale(-0.5), &z2, &z2, &s[k].scale(0.5)), &i2));
    }
    let iso8: Vec<CMat> = (0..3).map(|k| kron(&i4, &tau[k].scale(0.5))).collect();
    let t3_8 = iso8[2].clone();
    let i8 = CMat::identity(8);
    let lorentz16: Vec<CMat> = lorentz8.iter().map(|g| kron(g, &i2)).collect();
    let iso16: Vec<CMat> = iso8.iter().map(|g| kron(g, &i2)).collect();
    let species16: Vec<CMat> = (0..3).map(|k| kron(&i8, &tau[k].scale(0.5))).collect();
    let t3_16 = iso16[2].clone();

    // the factors of the pair matrices
    let dirac: Vec<(&str, CMat, &str)> = vec![("C_LL gamma_ch", cgch.clone(), "0+"), ("C_LL", cc.clone(), "0-")];
    let isof: Vec<(&str, CMat)> = vec![
        ("eps2 tau_1", eps2.mul(&tau[0])),
        ("eps2 tau_2", eps2.mul(&tau[1])),
        ("eps2 tau_3", eps2.mul(&tau[2])),
        ("eps2", eps2.clone()),
    ];
    let speciesf: Vec<(&str, CMat, &str)> = vec![("eps2", eps2.clone(), "two species, species singlet"), ("sigma_0", s[0].clone(), "two species, each with itself")];

    // ---------------- X-1 exchange signs of the factors and of their products
    let mut x1 = basics;
    let ds: Vec<i32> = dirac.iter().map(|d| exchange_sign(&d.1)).collect();
    let is: Vec<i32> = isof.iter().map(|f| exchange_sign(&f.1)).collect();
    let ss: Vec<i32> = speciesf.iter().map(|f| exchange_sign(&f.1)).collect();
    x1 &= ds == vec![-1, -1] && is == vec![1, 1, 1, -1] && ss == vec![-1, 1];
    let mut nprod = 0;
    for (di, d) in dirac.iter().enumerate() {
        for (ki, f) in isof.iter().enumerate() {
            let k = kron(&d.1, &f.1);
            x1 &= exchange_sign(&k) == ds[di] * is[ki];
            nprod += 1;
            for (si, sp) in speciesf.iter().enumerate() {
                let k2 = kron(&k, &sp.1);
                x1 &= exchange_sign(&k2) == ds[di] * is[ki] * ss[si];
                nprod += 1;
            }
        }
    }
    ck.add("X-1", &format!("exchange signs: the Dirac factors C_LL gamma_ch (0+) and C_LL (0-) of the spin-zero pairs are antisymmetric (in each chirality block they are eps2, the spin singlet up-down minus down-up), the isospin factors eps2 tau_1, eps2 tau_2, eps2 tau_3 are symmetric (isospin 1) and eps2 is antisymmetric (isospin 0), the species factors eps2 (antisymmetric) and sigma_0 (symmetric); gamma_ch = diag(-1,-1,1,1), C_LL = diag(eps2, -eps2), C_LL gamma_ch = -diag(eps2, eps2); and the exchange sign of each of the {} Kronecker products of a Dirac, an isospin and possibly a species factor is the product of the signs of its factors", nprod), x1, format!("signs: Dirac {:?}, isospin {:?}, species {:?}", ds, is, ss));

    // ---------------- X-2 the Pauli principle in numbers (one field)
    let mut rows_pm: Vec<Vec<String>> = Vec::new();
    let mut terms_one: Vec<usize> = Vec::new();
    let mut x2 = true;
    let mut x3 = true;
    for (di, d) in dirac.iter().enumerate() {
        for (ki, f) in isof.iter().enumerate() {
            let k = kron(&d.1, &f.1);
            let nt = pair_terms(&k);
            terms_one.push(nt);
            x2 &= if is[ki] == 1 { nt == 4 } else { nt == 0 };
            // the pair keeps only the antisymmetric part of its matrix
            let anti = k.sub(&k.transpose()).scale(0.5);
            x2 &= pair_terms(&anti) == nt;
            // X-3: Lorentz invariance and isospin
            let lor = lorentz8.iter().all(|g| is_zero(&ad(g, &k)));
            let cas = casimir_value(&iso8, &k);
            x3 &= lor && cas == Some(if is[ki] == 1 { 2.0 } else { 0.0 });
            let q = eigen_charge(&t3_8, &k);
            rows_pm.push(vec![
                format!("{} (x) {}", d.0, f.0),
                d.2.to_string(),
                if cas == Some(2.0) { "1 (triplet)".to_string() } else { "0 (singlet)".to_string() },
                "one field".to_string(),
                format!("{:+}", ds[di]),
                format!("{:+}", is[ki]),
                "-".to_string(),
                format!("{:+}", exchange_sign(&k)),
                nt.to_string(),
                match q {
                    Some(q) => format!("{:+}", q).replace("+0", "0"),
                    None => "mixture of +1 and -1".to_string(),
                },
            ]);
        }
    }
    ck.add("X-2", "the Pauli principle in numbers: for anticommuting components the pair Psi_LL^T Kch Psi_LL keeps only the antisymmetric part (Kch - Kch^T)/2 of its matrix; counting the products of two different components with a non-zero coefficient, each of the six isospin-triplet matrices (C_LL gamma_ch or C_LL times eps2 tau_iota) gives 4 terms, and each of the two isospin-singlet matrices (C_LL gamma_ch or C_LL times eps2) gives 0: the spin-zero isospin-singlet pair of one field vanishes identically", x2, format!("terms (C_LL gamma_ch with eps2 tau_1, eps2 tau_2, eps2 tau_3, eps2; then C_LL with the same): {:?}", terms_one));
    ck.add("X-3", "the eight spin-zero matrices are Lorentz invariant (generator^T Kch + Kch generator = 0 for the three rotations and the three boosts of Psi_LL), and the isospin Casimir sum_iota of the twofold change under T_LL^iota is 2 Kch = 1(1+1) Kch for the six matrices with eps2 tau_iota (isospin 1, triplet) and 0 for the two matrices with eps2 (isospin 0, singlet)", x3, String::new());

    // ---------------- X-4 the electric charges of the channels
    let k1 = kron(&cgch, &isof[0].1);
    let k2 = kron(&cgch, &isof[1].1);
    let k3 = kron(&cgch, &isof[2].1);
    let ksing1 = kron(&cgch, &eps2);
    let kup = k1.sub(&smul(&k2, ii));
    let kdn = k1.add(&smul(&k2, ii));
    let qs = (eigen_charge(&t3_8, &kup), eigen_charge(&t3_8, &k3), eigen_charge(&t3_8, &kdn), eigen_charge(&t3_8, &ksing1));
    // the isospin parts: up-up only, up-down symmetric, down-down only, up-down antisymmetric
    let iup = isof[0].1.sub(&smul(&isof[1].1, ii));
    let idn = isof[0].1.add(&smul(&isof[1].1, ii));
    let shapes = close(&iup, &mat(2, &[c(2.0, 0.0), z, z, z]))
        && close(&idn, &mat(2, &[z, z, z, c(-2.0, 0.0)]))
        && close(&isof[2].1, &mat(2, &[z, -one, -one, z]))
        && close(&eps2, &mat(2, &[z, one, -one, z]));
    let x4 = qs == (Some(1), Some(0), Some(-1), Some(0)) && shapes;
    for (name, k, q) in [("Kch_1 - i Kch_2", &kup, "+1"), ("Kch_3", &k3, "0"), ("Kch_1 + i Kch_2", &kdn, "-1")] {
        rows_pm.push(vec![
            name.to_string(),
            "0+".to_string(),
            "1 (triplet)".to_string(),
            "one field".to_string(),
            "-1".to_string(),
            "+1".to_string(),
            "-".to_string(),
            format!("{:+}", exchange_sign(k)),
            pair_terms(k).to_string(),
            q.to_string(),
        ]);
    }
    ck.add("X-4", "electric charges of the pairs: the pair operator Psi_LL^T Kch Psi_LL annihilates two particles of total charge Q_LL(Kch), the number with generator^T Kch + Kch generator = Q_LL(Kch) Kch for the generator T_LL^3 = Q_LL: Kch_1 - i Kch_2 (isospin part [[2,0],[0,0]], up with up) has Q_LL(Kch) = +1, Kch_3 (isospin part [[0,-1],[-1,0]], up with down, symmetric) has 0, Kch_1 + i Kch_2 (isospin part [[0,0],[0,-2]], down with down) has -1, and the vanishing singlet C_LL gamma_ch (x) eps2 (up with down, antisymmetric) has 0", x4, {
        let f = |q: Option<i32>| match q { Some(0) => "0".to_string(), Some(v) => format!("{:+}", v), None => "none".to_string() };
        format!("Q_LL(Kch) = {}, {}, {}, {}", f(qs.0), f(qs.1), f(qs.2), f(qs.3))
    });

    // ---------------- X-5, X-6, X-7 the example condensates
    let r = FRAC_1_SQRT_2;
    let ph = C64::cis(PI / 4.0);
    let nrm = (1.0f64 + 0.25).sqrt();
    // (name, the definition of dvec in words and numbers, dvec)
    let states: Vec<(&str, &str, [C64; 3])> = vec![
        ("polar along the third axis", "(0, 0, 1)", [z, z, one]),
        ("polar along the first axis", "(1, 0, 0)", [one, z, z]),
        ("polar along (1,2,2)/3", "(1, 2, 2)/3", [c(1.0 / 3.0, 0.0), c(2.0 / 3.0, 0.0), c(2.0 / 3.0, 0.0)]),
        ("unitary with a phase", "exp(i pi/4) (0, 0.6, 0.8)", [z, ph * 0.6, ph * 0.8]),
        ("partly non-unitary", "(1, i/2, 0)/sqrt(5/4)", [c(1.0 / nrm, 0.0), c(0.0, 0.5 / nrm), z]),
        ("charged, up-up pairs", "(1, i, 0)/sqrt2", [c(r, 0.0), c(0.0, r), z]),
        ("charged, down-down pairs", "(1, -i, 0)/sqrt2", [c(r, 0.0), c(0.0, -r), z]),
    ];
    let cross = |a: &[C64; 3], b: &[C64; 3]| -> [C64; 3] { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] };
    let conj3 = |a: &[C64; 3]| -> [C64; 3] { [a[0].conj(), a[1].conj(), a[2].conj()] };
    let norm2 = |a: &[C64; 3]| -> f64 { a.iter().map(|z| z.norm_sqr()).sum() };
    // pair wavefunction -sum_iota d_iota tau_iota eps2 (and the same written as sum_iota d_iota eps2 tau_iota^*)
    let pairwf = |d: &[C64; 3]| -> (CMat, bool) {
        let mut dt = CMat::zeros(2);
        let mut alt = CMat::zeros(2);
        for k in 0..3 {
            dt = dt.add(&smul(&tau[k], d[k]));
            alt = alt.add(&smul(&eps2.mul(&tau[k].conj()), d[k]));
        }
        let b = smul(&dt.mul(&eps2), c(-1.0, 0.0));
        let same = close(&b, &alt);
        (b, same)
    };
    // average isospin of a pair with the wavefunction b: tr(b^dag (tau b + b tau^T)) / (2 tr(b^dag b))
    let pair_isospin = |b: &CMat| -> ([f64; 3], f64) {
        let tot = b.dagger().mul(b).trace().re;
        let mut v = [0.0; 3];
        let mut imag = 0.0f64;
        for k in 0..3 {
            let tb = tau[k].mul(b).add(&b.mul(&tau[k].transpose()));
            let x = b.dagger().mul(&tb).trace();
            v[k] = x.re / (2.0 * tot);
            imag = imag.max(x.im.abs());
        }
        (v, imag)
    };
    // The action of the four generators of SU(2)_v x U(1)_F_LL (T_LL^1, T_LL^2, T_LL^3, F_LL) on dvec,
    // derived from their action on the pair matrices, not typed in: under the symmetry exp(i theta G)
    // the field changes as Psi_LL -> exp(i theta g) Psi_LL, with g = tau_iota/2 on the isospin index for
    // T_LL^iota and g = 1 for F_LL; the pair operator Psi_LL^T Kch_iota Psi_LL then changes by
    // i theta Psi_LL^T (g^T Kch_iota + Kch_iota g) Psi_LL.  Writing g^T Kch_iota + Kch_iota g as
    // sum_iota' c_(iota iota') Kch_iota', the order parameter (proportional to the expectation value of
    // the pair operators) changes by i theta sum_iota' c_(iota iota') d_iota'.  acts[g][iota][iota'] holds
    // i c_(iota iota').
    let kvec = [k1.clone(), k2.clone(), k3.clone()];
    let gens: Vec<CMat> = vec![iso8[0].clone(), iso8[1].clone(), iso8[2].clone(), i8.clone()];
    let mut acts: Vec<[[C64; 3]; 3]> = Vec::new();
    let mut closes = true;
    for g in &gens {
        let mut a = [[z; 3]; 3];
        for (io, kk) in kvec.iter().enumerate() {
            let ch = ad(g, kk);
            let mut rebuilt = CMat::zeros(8);
            for (jo, kj) in kvec.iter().enumerate() {
                let cij = kj.dagger().mul(&ch).trace() / kj.dagger().mul(kj).trace();
                a[io][jo] = cij * ii;
                rebuilt = rebuilt.add(&smul(kj, cij));
            }
            // the change of a triplet pair matrix is again a combination of the three triplet matrices
            closes &= close(&rebuilt, &ch);
        }
        acts.push(a);
    }
    let apply = |a: &[[C64; 3]; 3], d: &[C64; 3]| -> [C64; 3] {
        let mut out = [z; 3];
        for io in 0..3 {
            for jo in 0..3 {
                out[io] = out[io] + a[io][jo] * d[jo];
            }
        }
        out
    };
    // what the derivation gives: T_LL^iota changes dvec by -cross(unit vector iota, dvec) (a rotation of
    // the vector dvec about the axis iota), F_LL multiplies dvec by 2 i
    let mut actions_ok = closes;
    {
        let mut rng = Rng(77);
        for _ in 0..20 {
            let d = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
            for k in 0..3 {
                let mut e = [z; 3];
                e[k] = one;
                let want = cross(&e, &d);
                let got = apply(&acts[k], &d);
                actions_ok &= (0..3).all(|m| (got[m] + want[m]).abs() < 1e-12);
            }
            let got = apply(&acts[3], &d);
            actions_ok &= (0..3).all(|m| (got[m] - d[m] * c(0.0, 2.0)).abs() < 1e-12);
        }
    }
    // the unbroken generators of a state dvec: the real combinations a_1 T_LL^1 + a_2 T_LL^2 + a_3 T_LL^3 + b F_LL
    // that leave dvec unchanged (null space of the real 6 x 4 matrix of the changes)
    let stabilizer = |d: &[C64; 3]| -> (usize, Vec<Vec<f64>>) {
        let vecs: Vec<Vec<f64>> = acts.iter().map(|a| apply(a, d).iter().flat_map(|w| [w.re, w.im]).collect()).collect();
        let mut gm = CMat::zeros(4);
        for i in 0..4 {
            for j in 0..4 {
                let sdot: f64 = vecs[i].iter().zip(vecs[j].iter()).map(|(a, b)| a * b).sum();
                gm.set(i, j, c(sdot, 0.0));
            }
        }
        let (ev, v) = eigh(&gm);
        let mx = ev.iter().cloned().fold(0.0f64, f64::max);
        let mut null = Vec::new();
        for (col, &x) in ev.iter().enumerate() {
            if x <= 1e-10 * mx {
                let mut u: Vec<f64> = (0..4).map(|rw| v.get(rw, col).re).collect();
                // sign: the component of largest modulus positive; scale: largest component 1
                let big = u.iter().cloned().fold(0.0f64, |a, b| if b.abs() > a.abs() { b } else { a });
                for w in u.iter_mut() {
                    *w /= big;
                }
                null.push(u);
            }
        }
        (null.len(), null)
    };
    let mut x5 = true;
    let mut x6 = true;
    let mut x7 = true;
    let mut rows_st: Vec<Vec<String>> = Vec::new();
    let mut detail5 = Vec::new();
    let mut detail6 = Vec::new();
    let mut detail7 = Vec::new();
    for (name, definition, d) in &states {
        let dd = norm2(d);
        let (b, same) = pairwf(d);
        // the isospin part Dpair = sum_iota d_iota (eps2 tau_iota)^dag of the pairing block of the BdG matrix
        let mut gmat = CMat::zeros(2);
        for k in 0..3 {
            gmat = gmat.add(&smul(&eps2.mul(&tau[k]).dagger(), d[k]));
        }
        x5 &= close(&b, &gmat);
        let w_uu = b.get(0, 0).norm_sqr();
        let w_dd = b.get(1, 1).norm_sqr();
        let w_ud = b.get(0, 1).norm_sqr() + b.get(1, 0).norm_sqr();
        let tot = w_uu + w_ud + w_dd;
        let (fp, f0, fm) = (w_uu / tot, w_ud / tot, w_dd / tot);
        let qavg = fp - fm;
        let cr = cross(d, &conj3(d));
        let formula: Vec<f64> = cr.iter().map(|w| (*w * ii).re / dd).collect();
        let formula_imag = cr.iter().map(|w| (*w * ii).im.abs()).fold(0.0f64, f64::max);
        let (iso, imag) = pair_isospin(&b);
        let zeta = cr.iter().map(|w| w.norm_sqr()).sum::<f64>().sqrt() / dd;
        let isolen = (iso[0] * iso[0] + iso[1] * iso[1] + iso[2] * iso[2]).sqrt();
        x5 &= same && (tot - 2.0 * dd).abs() < TOL && (qavg - formula[2]).abs() < TOL && formula_imag < TOL && imag < TOL;
        x5 &= (0..3).all(|k| (iso[k] - formula[k]).abs() < TOL) && (isolen - zeta).abs() < TOL;
        // gaps
        let (ev, vv) = eigh(&gmat.mul(&gmat.dagger()));
        let dm = ev[0].max(0.0).sqrt();
        let dp = ev[1].max(0.0).sqrt();
        x6 &= (ev[0] - dd * (1.0 - zeta)).abs() < 1e-10 && (ev[1] - dd * (1.0 + zeta)).abs() < 1e-10;
        let mut unpaired = String::from("-");
        if (zeta - 1.0).abs() < 1e-12 {
            // the eigenvector of the eigenvalue 0: the isospin direction of the unpaired particles
            let up = vv.get(0, 0).abs();
            let dn = vv.get(1, 0).abs();
            unpaired = if (dn - 1.0).abs() < 1e-12 && up < 1e-12 { "down".to_string() } else if (up - 1.0).abs() < 1e-12 && dn < 1e-12 { "up".to_string() } else { "other".to_string() };
            x6 &= (name.contains("up-up") && unpaired == "down") || (name.contains("down-down") && unpaired == "up");
        }
        // unbroken generators
        let (nstab, null) = stabilizer(d);
        let unitary = zeta < 1e-12;
        let unbroken = if nstab == 0 {
            "none".to_string()
        } else if nstab == 1 {
            let u = &null[0];
            if unitary && u[3].abs() < 1e-12 {
                // the axis as a unit vector (the null vector u is scaled so that its largest component is 1)
                let ulen = (u[0] * u[0] + u[1] * u[1] + u[2] * u[2]).sqrt();
                let axis = freal(&[u[0] / ulen, u[1] / ulen, u[2] / ulen]);
                if axis == "(0, 0, 1)" {
                    format!("isospin rotation about the axis {}: T_LL^3 = Q_LL", axis)
                } else {
                    format!("isospin rotation about the axis {}", axis)
                }
            } else if u[0].abs() < 1e-12 && u[1].abs() < 1e-12 && (u[2] - 1.0).abs() < 1e-12 && (u[3] + 0.5).abs() < 1e-12 {
                // T_LL^3 - F_LL/2, i.e. (up to the sign) F_LL/2 - T_LL^3: 0 on isospin up, 1 on isospin down
                "F_LL/2 - T_LL^3, the number of isospin-down particles".to_string()
            } else if u[0].abs() < 1e-12 && u[1].abs() < 1e-12 && (u[2] - 1.0).abs() < 1e-12 && (u[3] - 0.5).abs() < 1e-12 {
                "F_LL/2 + T_LL^3, the number of isospin-up particles".to_string()
            } else if u[0].abs() < 1e-12 && u[1].abs() < 1e-12 {
                "one combination of T_LL^3 and F_LL".to_string()
            } else {
                format!("the combination {} of the four generators", freal(u))
            }
        } else {
            format!("{} generators", nstab)
        };
        if unitary {
            // the real direction of dvec: dvec = (phase) x (real vector); the unbroken generator is that vector . T_LL
            let kmax = (0..3).max_by(|a, b| d[*a].abs().partial_cmp(&d[*b].abs()).unwrap()).unwrap();
            let phase = C64::cis(-d[kmax].arg());
            let nvec: Vec<f64> = d.iter().map(|w| (*w * phase).re).collect();
            let nbig = nvec[kmax];
            let u = &null[0];
            x7 &= nstab == 1 && (0..3).all(|k| (u[k] - nvec[k] / nbig).abs() < 1e-10) && u[3].abs() < 1e-10;
            x5 &= qavg.abs() < TOL && isolen < TOL;
        }
        if name.starts_with("charged") {
            // up-up pairs: the down particles are unpaired and their number F_LL/2 - T_LL^3 is conserved;
            // down-down pairs: the number of up particles F_LL/2 + T_LL^3
            let want = if name.contains("up-up") { "F_LL/2 - T_LL^3, the number of isospin-down particles" } else { "F_LL/2 + T_LL^3, the number of isospin-up particles" };
            x7 &= nstab == 1 && unbroken == want;
            x5 &= (qavg.abs() - 1.0).abs() < TOL;
        }
        if name.starts_with("partly") {
            x7 &= nstab == 0;
        }
        detail5.push(format!("{}: charges +1/0/-1 {}/{}/{}, average charge {}, zeta {}", name, fnum(fp), fnum(f0), fnum(fm), fnum(qavg), fnum(zeta)));
        detail6.push(format!("{}: Delta_+ {}, Delta_- {}{}", name, fnum(dp), fnum(dm), if unpaired != "-" { format!(" (unpaired: {})", unpaired) } else { String::new() }));
        detail7.push(format!("{}: {}", name, unbroken));
        rows_st.push(vec![
            name.to_string(),
            definition.to_string(),
            fvec(d),
            fnum(fp),
            fnum(f0),
            fnum(fm),
            fnum(qavg),
            freal(&iso),
            fnum(zeta),
            fnum(dp),
            fnum(dm),
            unpaired,
            nstab.to_string(),
            unbroken,
        ]);
    }
    // the same identities for random order parameters
    let mut rng = Rng(20261116);
    for _ in 0..200 {
        let d = [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())];
        let dd = norm2(&d);
        let (b, same) = pairwf(&d);
        let cr = cross(&d, &conj3(&d));
        let formula: Vec<f64> = cr.iter().map(|w| (*w * ii).re / dd).collect();
        let (iso, _) = pair_isospin(&b);
        let zeta = cr.iter().map(|w| w.norm_sqr()).sum::<f64>().sqrt() / dd;
        x5 &= same && (0..3).all(|k| (iso[k] - formula[k]).abs() < 1e-10);
        let mut gmat = CMat::zeros(2);
        for k in 0..3 {
            gmat = gmat.add(&smul(&eps2.mul(&tau[k]).dagger(), d[k]));
        }
        let (ev, _) = eigh(&gmat.mul(&gmat.dagger()));
        x6 &= (ev[0] - dd * (1.0 - zeta)).abs() < 1e-10 && (ev[1] - dd * (1.0 + zeta)).abs() < 1e-10;
    }
    ck.add("X-5", "the pairs of seven example condensates (|dvec| = 1) and of 200 random ones: the condensed pairs have the isospin wavefunction -sum_iota d_iota tau_iota eps2 (= sum_iota d_iota eps2 tau_iota^*), which is the isospin part Dpair = sum_iota d_iota (eps2 tau_iota)^dag of the pairing block of the Bogoliubov-de Gennes matrix; the weights of its entries up-up, up-down, down-down are the fractions of pairs with the charges +1, 0, -1; the average charge of a pair, fraction(+1) - fraction(-1), equals the third component of i cross(dvec, dvec^*)/|dvec|^2, the whole vector is the average isospin of a pair, and its length is zeta: unitary states (zeta = 0) have pairs of average charge 0 and average isospin 0, the charged states (zeta = 1) consist of pairs of charge +1 only or -1 only", x5, detail5.join("; "));
    ck.add("X-6", "gaps of the examples and of 200 random dvec: the eigenvalues of the 2 x 2 gap matrix times its Hermitian conjugate are |dvec|^2 (1 - zeta) and |dvec|^2 (1 + zeta), so unitary states have the single gap Delta_+ = Delta_- = |dvec| for both isospin directions, and the charged states have Delta_+ = sqrt2 |dvec| and Delta_- = 0, the gapless direction being isospin down for the up-up state and isospin up for the down-down state (those particles are not paired)", x6, detail6.join("; "));
    x7 &= actions_ok;
    ck.add("X-7", "unbroken generators of SU(2)_v x U(1)_F_LL: the action of each generator on dvec is derived from its action on the pair matrices (generator^T Kch_iota + Kch_iota generator is again a combination of Kch_1, Kch_2, Kch_3; true for all 12 cases), and it is: T_LL^iota changes dvec by minus the cross product of the unit vector iota with dvec (a rotation of dvec about the axis iota), F_LL multiplies dvec by 2 i (20 random dvec); a generator is unbroken if it leaves dvec unchanged; for every unitary example exactly one generator is unbroken, the isospin rotation about the real direction of dvec (for the polar state along the third axis that is T_LL^3 = Q_LL: the electric charge is unbroken); for the charged state with up-up pairs the only unbroken generator is F_LL/2 - T_LL^3, the number of isospin-down particles (they are not paired), and for the down-down state F_LL/2 + T_LL^3; the partly non-unitary state breaks all four generators", x7, detail7.join("; "));

    // ---------------- X-8 the singlet of the whole super-loop: two determinants
    let mut x8 = true;
    let mut sym_ok = true;
    let mut cd_ok = true;
    let gq: Q = [one, ii, one, z]; // gL = e0 + i e1 + e2, N_LL(gL) = 1 (a Lorentz transformation)
    let vq: Q = [z, z, z, one]; // vI = e3, a unit quaternion (an isospin rotation)
    x8 &= (qnorm(&gq, &gq) - one).abs() < TOL && (qnorm(&vq, &vq) - one).abs() < TOL;
    for _ in 0..200 {
        let b = rand_q(&mut rng);
        let bp = rand_q(&mut rng);
        // M_LL is multiplicative and its determinant is the norm
        let m = mq(&b, &s);
        let mp = mq(&bp, &s);
        x8 &= close(&mq(&qmul(&b, &bp), &s), &m.mul(&mp));
        let det = m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0);
        x8 &= (det - qnorm(&b, &b)).abs() < TOL;
        // N_LL(bS', bS) = (1/2) tr(M(bS)^T eps2 M(bS') eps2^T): one eps2 contracts the two isospin
        // indices (rows), the other the two spinor indices (columns)
        let two = m.transpose().mul(&eps2).mul(&mp).mul(&eps2.transpose()).trace() * 0.5;
        x8 &= (two - qnorm(&bp, &b)).abs() < TOL;
        // the scalar part of bS'bar bS is N_LL(bS', bS); the bi-octonion product of the spinor charges
        // (bS e4)(bS' e4) has the scalar part -N_LL(bS', bS)
        let prod = qmul(&qbar(&bp), &b);
        x8 &= (prod[0] - qnorm(&bp, &b)).abs() < TOL;
        let (p0, p1) = cd(&([z; 4], b), &([z; 4], bp));
        let (r0, r1) = cd(&([z; 4], bp), &([z; 4], b));
        cd_ok &= (p0[0] + qnorm(&bp, &b)).abs() < TOL && p1.iter().all(|w| w.abs() < TOL) && r1.iter().all(|w| w.abs() < TOL);
        // symmetric scalar part, antisymmetric imaginary part
        sym_ok &= (p0[0] - r0[0]).abs() < TOL && (1..4).all(|k| (p0[k] + r0[k]).abs() < TOL);
        // invariant under the automorphisms bS -> vI bS gLbar (Lorentz and isospin)
        let tb = qmul(&qmul(&vq, &b), &qbar(&gq));
        let tbp = qmul(&qmul(&vq, &bp), &qbar(&gq));
        x8 &= (qnorm(&tbp, &tb) - qnorm(&bp, &b)).abs() < TOL;
        // and under a complex unit gL built from a random element
        let w = rand_q(&mut rng);
        let nw = qnorm(&w, &w);
        if nw.abs() > 0.5 {
            let sq = csqrt(nw);
            let gr: Q = [w[0] / sq, w[1] / sq, w[2] / sq, w[3] / sq];
            let tb2 = qmul(&b, &qbar(&gr));
            let tbp2 = qmul(&bp, &qbar(&gr));
            x8 &= (qnorm(&tbp2, &tb2) - qnorm(&bp, &b)).abs() < 1e-9;
        }
    }
    // two explicit spinor charges: e4 e4 = -e0, e4 e5 = e1, e5 e4 = -e1
    let e0q: Q = [one, z, z, z];
    let e1q: Q = [z, one, z, z];
    let (a44, _) = cd(&([z; 4], e0q), &([z; 4], e0q));
    let (a45, _) = cd(&([z; 4], e0q), &([z; 4], e1q));
    let (a54, _) = cd(&([z; 4], e1q), &([z; 4], e0q));
    let ex_ok = qeq(&a44, &[-one, z, z, z]) && qeq(&a45, &[z, one, z, z]) && qeq(&a54, &[z, -one, z, z]);
    // the bi-octonions are not associative: (e1 e2) e4 = e7, but e1 (e2 e4) = -e7
    let e2q: Q = [z, z, one, z];
    let e3q: Q = [z, z, z, one];
    let zq: Q = [z; 4];
    let left = cd(&cd(&(e1q, zq), &(e2q, zq)), &(zq, e0q));
    let right = cd(&(e1q, zq), &cd(&(e2q, zq), &(zq, e0q)));
    let nonassoc_ok = qeq(&left.0, &zq) && qeq(&left.1, &e3q) && qeq(&right.0, &zq) && qeq(&right.1, &[z, z, z, -one]);
    // the norm of the bi-octonion product is the product of the norms, and varphi_(gL,vI):
    // aH + bS e4 -> gL aH gLbar + (vI bS gLbar) e4 preserves the product (vI = e3 and vI = (e0 + e1 + e2 + e3)/2)
    let bnorm = |x: &(Q, Q)| -> C64 { qnorm(&x.0, &x.0) + qnorm(&x.1, &x.1) };
    let vq2: Q = [c(0.5, 0.0), c(0.5, 0.0), c(0.5, 0.0), c(0.5, 0.0)];
    let mut aut_ok = (qnorm(&vq2, &vq2) - one).abs() < TOL;
    let mut norm_ok = true;
    for _ in 0..200 {
        let xx = (rand_q(&mut rng), rand_q(&mut rng));
        let yy = (rand_q(&mut rng), rand_q(&mut rng));
        let xy = cd(&xx, &yy);
        norm_ok &= (bnorm(&xy) - bnorm(&xx) * bnorm(&yy)).abs() < 1e-9;
        for v in [&vq, &vq2] {
            let phi = |x: &(Q, Q)| -> (Q, Q) { (qmul(&qmul(&gq, &x.0), &qbar(&gq)), qmul(&qmul(v, &x.1), &qbar(&gq))) };
            let lhs = phi(&xy);
            let rhs = cd(&phi(&xx), &phi(&yy));
            aut_ok &= (0..4).all(|k| (lhs.0[k] - rhs.0[k]).abs() < 1e-9 && (lhs.1[k] - rhs.1[k]).abs() < 1e-9);
        }
    }
    let x8all = x8 && sym_ok && cd_ok && ex_ok && nonassoc_ok && norm_ok && aut_ok;
    ck.add("X-8", "the singlet of the whole super-loop (200 random pairs of bi-quaternions bS, bS' with Gaussian-integer coefficients): M_LL is multiplicative and det M_LL(bS) = N_LL(bS); N_LL(bS', bS) = (1/2) tr(M_LL(bS)^T eps2 M_LL(bS') eps2^T), a contraction of the two isospin indices with eps2 (isospin 0) and of the two spinor indices with eps2 (spin 0); it is the scalar part of bS'bar bS and minus the scalar part of the bi-octonion product (bS e4)(bS' e4) (Cayley-Dickson product), which is SYMMETRIC under the exchange of the two spinor charges, while the imaginary part is antisymmetric; it is invariant under bS -> vI bS gLbar (vI = e3, gL = 1 + i e1 + e2, and random complex unit gL); examples: e4 e4 = -e0, e4 e5 = e1, e5 e4 = -e1; the product of B_LL is not associative ((e1 e2) e4 = e7 but e1 (e2 e4) = -e7), its norm is multiplicative (N_LL(x y) = N_LL(x) N_LL(y) for 200 random pairs of bi-octonions, so the unit bi-octonions are closed under the product), and varphi_(gL,vI) preserves the product (the same 200 pairs; vI = e3 and vI = (e0 + e1 + e2 + e3)/2)", x8all, format!("determinant and contraction {}, Cayley-Dickson product {}, exchange symmetry {}, examples {}, non-associativity {}, multiplicative norm {}, automorphism {}", x8, cd_ok, sym_ok, ex_ok, nonassoc_ok, norm_ok, aut_ok));

    // ---------------- X-9 the two-species singlet
    let mut x9 = true;
    for (si, sp) in speciesf.iter().enumerate() {
        for (ki, f) in [&isof[3], &isof[2]].iter().enumerate() {
            let k = kron(&kron(&cgch, &f.1), &sp.1);
            let nt = pair_terms(&k);
            let lor = lorentz16.iter().all(|g| is_zero(&ad(g, &k)));
            let cas = casimir_value(&iso16, &k);
            let scas = casimir_value(&species16, &k);
            let q = eigen_charge(&t3_16, &k);
            let iso_sign = if ki == 0 { -1 } else { 1 };
            let total = exchange_sign(&k);
            x9 &= lor && total == -iso_sign * ss[si] && (if total == -1 { nt == 8 } else { nt == 0 });
            // isospin: 0 for eps2, 1 for eps2 tau_3; species: 0 for the species singlet eps2, 1 for sigma_0
            // (sigma_0 = -i eps2 tau_2 is a member of the species triplet); charge 0 in all four cases
            x9 &= cas == Some(if ki == 0 { 0.0 } else { 2.0 }) && scas == Some(if si == 0 { 0.0 } else { 2.0 }) && q == Some(0);
            if si == 0 && ki == 0 {
                // the super-loop singlet Kch_singlet: spin 0, isospin 0, species singlet
                x9 &= scas == Some(0.0) && cas == Some(0.0) && total == -1 && nt == 8;
            }
            rows_pm.push(vec![
                format!("C_LL gamma_ch (x) {} (x) {}{}", f.0, sp.0, if si == 0 && ki == 0 { " = Kch_singlet" } else { "" }),
                "0+".to_string(),
                if ki == 0 { "0 (singlet)".to_string() } else { "1 (triplet)".to_string() },
                sp.2.to_string(),
                "-1".to_string(),
                format!("{:+}", iso_sign),
                format!("{:+}", ss[si]),
                format!("{:+}", total),
                nt.to_string(),
                "0".to_string(),
            ]);
        }
    }
    ck.add("X-9", "two species: the super-loop singlet Kch_singlet = C_LL gamma_ch (x) eps2 (x) eps2 (spin 0, isospin 0, species singlet: species Casimir 0) is antisymmetric (signs -1, -1, -1), Lorentz invariant, of charge 0, and keeps 8 terms; the same Dirac and isospin factors with the species factor sigma_0 (each species paired with itself) give a symmetric matrix and 0 terms, as for one field; the isospin triplet eps2 tau_3 with sigma_0 keeps 8 terms (each species pairs as a triplet) and with the species singlet eps2 vanishes", x9, String::new());

    // ---------------- X-10 the nucleon analogy: two particles of spin 1/2 and isospin 1/2 in a relative s-wave
    let count = |with_species: bool| -> Vec<(i32, i32, i32, usize, usize, usize)> {
        let n1 = if with_species { 8 } else { 4 };
        let onepart = |which: usize, m: &CMat| -> CMat {
            // which = 0: spin, 1: isospin, 2: species
            let mut op = if which == 0 { m.clone() } else { CMat::identity(2) };
            op = kron(&op, if which == 1 { m } else { &i2 });
            if with_species {
                op = kron(&op, if which == 2 { m } else { &i2 });
            }
            op
        };
        let id1 = CMat::identity(n1);
        let n2 = n1 * n1;
        let id2 = CMat::identity(n2);
        let total = |which: usize| -> CMat {
            let mut acc = CMat::zeros(n2);
            for k in 0..3 {
                let o = onepart(which, &tau[k].scale(0.5));
                let tot = kron(&o, &id1).add(&kron(&id1, &o));
                acc = acc.add(&tot.mul(&tot));
            }
            acc
        };
        let s2 = total(0);
        let t2 = total(1);
        let j2 = if with_species { total(2) } else { CMat::zeros(n2) };
        let mut p = CMat::zeros(n2);
        for a in 0..n1 {
            for b in 0..n1 {
                p.set(b * n1 + a, a * n1 + b, one);
            }
        }
        let anti = id2.sub(&p).scale(0.5);
        let symm = id2.add(&p).scale(0.5);
        let proj = |cas: &CMat, which: i32| -> CMat { if which == 0 { id2.sub(&cas.scale(0.5)) } else { cas.scale(0.5) } };
        let mut out = Vec::new();
        let jr: Vec<i32> = if with_species { vec![0, 1] } else { vec![-1] };
        for sp in [0, 1] {
            for ip in [0, 1] {
                for &jp in &jr {
                    let mut pr = proj(&s2, sp).mul(&proj(&t2, ip));
                    if jp >= 0 {
                        pr = pr.mul(&proj(&j2, jp));
                    }
                    let n = pr.trace().re.round() as usize;
                    let na = pr.mul(&anti).trace().re.round() as usize;
                    let ns = pr.mul(&symm).trace().re.round() as usize;
                    out.push((sp, ip, jp, n, na, ns));
                }
            }
        }
        out
    };
    let c0 = count(false);
    let c1 = count(true);
    let exp0 = vec![(0, 0, -1, 1, 0, 1), (0, 1, -1, 3, 3, 0), (1, 0, -1, 3, 3, 0), (1, 1, -1, 9, 0, 9)];
    let exp1 = vec![
        (0, 0, 0, 1, 1, 0),
        (0, 0, 1, 3, 0, 3),
        (0, 1, 0, 3, 0, 3),
        (0, 1, 1, 9, 9, 0),
        (1, 0, 0, 3, 0, 3),
        (1, 0, 1, 9, 9, 0),
        (1, 1, 0, 9, 9, 0),
        (1, 1, 1, 27, 0, 27),
    ];
    let x10 = c0 == exp0 && c1 == exp1;
    let mut rows_ct: Vec<Vec<String>> = Vec::new();
    for (sp, ip, jp, n, na, ns) in c0.iter().chain(c1.iter()) {
        rows_ct.push(vec![
            sp.to_string(),
            ip.to_string(),
            if *jp < 0 { "one kind".to_string() } else if *jp == 0 { "two species, 0 (singlet)".to_string() } else { "two species, 1 (triplet)".to_string() },
            n.to_string(),
            na.to_string(),
            ns.to_string(),
            if *na > 0 { "yes".to_string() } else { "no".to_string() },
        ]);
    }
    ck.add("X-10", "the nucleon analogy (two particles of spin 1/2 and isospin 1/2 in a relative s-wave, which is symmetric under the exchange of the positions, so that the spin-isospin part must be antisymmetric): of the 16 spin-isospin states the antisymmetric ones are the 3 states of spin 0 and isospin 1 and the 3 states of spin 1 and isospin 0; spin 0 with isospin 0 (1 state) and spin 1 with isospin 1 (9 states) are symmetric and forbidden; with two species (64 states) the state of spin 0, isospin 0 and species singlet is antisymmetric and allowed (counted by the traces of the projectors on total spin, total isospin, total species spin and exchange symmetry)", x10, format!("one kind: {:?}; two species: {:?}", c0.iter().map(|t| (t.0, t.1, t.4)).collect::<Vec<_>>(), c1.iter().map(|t| (t.0, t.1, t.2, t.4)).collect::<Vec<_>>()));

    // ---------------- data files
    let _ = write_csv_str(
        "ex_pair_matrices.csv",
        &[
            ("content", "spin-zero pair matrices Kch of the spinor charges (one field, 8 x 8; two species, 16 x 16): spin and parity, isospin, species, exchange signs of the Dirac, isospin and species factors and of the whole matrix, number of terms that survive for anticommuting components (pairs of different components with a non-zero coefficient), electric charge Q_LL of the pair".to_string()),
            ("rule", "a pair of anticommuting components survives only if the exchange sign of the whole matrix is -1".to_string()),
        ],
        &["pair_matrix", "spin_parity", "isospin", "species", "exchange_sign_Dirac", "exchange_sign_isospin", "exchange_sign_species", "exchange_sign_total", "nonzero_pair_terms", "pair_charge"],
        &rows_pm,
    );
    let _ = write_csv_str(
        "ex_states.csv",
        &[
            ("content", "example condensates of the spin-zero isospin-triplet superfluid with |dvec| = 1: fractions of condensed pairs with the electric charges +1, 0, -1, average charge of a pair, average isospin of a pair (= i cross(dvec, dvec^*)/|dvec|^2), zeta, the two gaps in units of |dvec|, the unpaired isospin direction (if a gap is 0), the dimension of the stabilizer in SU(2)_v x U(1)_F_LL and the unbroken generator".to_string()),
        ],
        &["state", "dvec_definition", "dvec", "pairs_charge_plus1", "pairs_charge_0", "pairs_charge_minus1", "average_pair_charge", "average_pair_isospin", "zeta", "Delta_+", "Delta_-", "unpaired", "stabilizer_dim", "unbroken_generator"],
        &rows_st,
    );
    let _ = write_csv_str(
        "ex_two_particle_counting.csv",
        &[("content", "two particles of spin 1/2 and isospin 1/2 (one kind, or two species) in a relative s-wave: number of two-particle states of each total spin, total isospin and total species spin, how many of them are antisymmetric and symmetric under the exchange of the two particles, and whether the combination is allowed for fermions (antisymmetric states exist)".to_string())],
        &["total_spin", "total_isospin", "species_pair", "number_of_states", "antisymmetric_states", "symmetric_states", "allowed_in_s_wave"],
        &rows_ct,
    );
    let ok = ck.write("ex_superfluid_checks.txt");
    if !ok {
        std::process::exit(1);
    }
}
