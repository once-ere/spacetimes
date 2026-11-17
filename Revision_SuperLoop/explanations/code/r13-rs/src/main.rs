//! r13_yukawa -- PROJECT spacetimes, Revision_SuperLoop, folder `explanations`.
//!
//! The conclusions of the manuscript of Revision_SuperLoop state that the Lorentz super-loop
//! "lives on the 7+1 and 3+1 Minkowski spaces inside the bi-octonions" and that "in 3+1
//! dimensions its Yukawa form is renormalizable; on the eight-dimensional manifolds it is not".
//! The document "The 3+1 manifold of the super-loop and the renormalizability of the Yukawa
//! form" of the folder explanations (.md, .tex, .pdf) defines the 3+1 manifold R^(1,3)_LL,
//! works through examples on it, and explains the statement.  This program computes every
//! number and every curve of that document and checks every statement the document makes about
//! them (checks Z-1 to Z-8, report explanations/data/ex_r13_checks.txt).
//!
//! Conventions (those of the program superloop of Revision_SuperLoop):
//!   * a bi-quaternion b = sum_p xi_p e_p (p = 0..3, complex xi_p) has the 2 x 2 matrix
//!     M_LL(b) = xi_0 sigma_0 - i (xi_1 sigma_1 + xi_2 sigma_2 + xi_3 sigma_3); M_LL is multiplicative
//!     and det M_LL(b) = N_LL(b) = sum_p xi_p^2.
//!   * R^(1,3)_LL: the Hermitian bi-quaternions x = x_LL^0 e0 + i (x_LL^1 e1 + x_LL^2 e2 + x_LL^3 e3)
//!     with real coordinates; their matrices are M_LL(x) = x_LL^0 sigma_0 + x_LL^1 sigma_1 + x_LL^2 sigma_2 + x_LL^3 sigma_3,
//!     Hermitian, with det M_LL(x) = N_LL(x) = (x_LL^0)^2 - (x_LL^1)^2 - (x_LL^2)^2 - (x_LL^3)^2 (in the code: xmat).
//!   * a unit bi-quaternion gL (an element of SL(2,C)) acts by x -> gL x gL^dag, i.e.
//!     M_LL(x) -> M_LL(gL) M_LL(x) M_LL(gL)^dag; in the coordinates this is the real 4 x 4 matrix Lmat with
//!     the entries Lmat[p][p'] = Re tr(sigma_p M_LL(gL) sigma_p' M_LL(gL)^dag) / 2 (in the code: lmat).
//!   * rotation about the axis j by the angle theta: M_LL(gL) = exp(-i theta sigma_j / 2) with gL = exp(theta J_LL^j),
//!     J_LL^j = e_j/2; boost along the axis j with the rapidity theta: M_LL(gL) = exp(theta sigma_j / 2)
//!     with gL = exp(theta K_LL^j), K_LL^j = i e_j/2; the 4 x 4 matrix of the infinitesimal action of
//!     uL = J_LL^j or K_LL^j is V_LL(uL) (in the code: gen).
//!   * power counting in D + 1 space-time dimensions (D = number of space dimensions), with the
//!     names of the manuscript: next_F, next_B external fermion and boson lines, nint_F, nint_B
//!     internal ones, nvert_Y and nvert_4 Yukawa and quartic vertices, nvert all vertices,
//!     nloop = nint_F + nint_B - nvert + 1, deg = (D+1) nloop - nint_F - 2 nint_B.
//!   * the one-loop bubble of the Yukawa form (Euclidean, cutoff Lambda on the loop momentum kE):
//!     Ibub(pE) = int_(|kE| < Lambda) d^4kE/(2 pi)^4 [kE.(kE+pE) + m^2] / ((kE^2 + m^2)((kE+pE)^2 + m^2)).
//!
//! Usage (from the folder Revision_SuperLoop/explanations):
//!     cargo run --release --manifest-path code/r13-rs/Cargo.toml
//! Output, in explanations/data: ex_r13_points.csv, ex_r13_orbits.csv, ex_r13_dimensions.csv,
//! ex_r13_power_counting.csv, ex_r13_divergent_counts.csv, ex_r13_bubble.csv and the report
//! ex_r13_checks.txt.  The program exits with status 1 if a check fails.
//! Arithmetic: double precision (the power counting in exact integers); equalities are tested
//! with the tolerance 1e-12 unless the statement of the check in the report names another one.

use spacetimes::complex::C64;
use spacetimes::io::write_csv_str;
use spacetimes::linalg::CMat;
use spacetimes::quad::gauss_legendre;
use std::f64::consts::PI;
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

// ------------------------------------------------------------------ 2 x 2 complex matrices

fn c(re: f64, im: f64) -> C64 {
    C64::new(re, im)
}

/// the 2 x 2 matrix [[a, b], [cc, d]]
fn m2(a: C64, b: C64, cc: C64, d: C64) -> CMat {
    let mut m = CMat::zeros(2);
    m.set(0, 0, a);
    m.set(0, 1, b);
    m.set(1, 0, cc);
    m.set(1, 1, d);
    m
}

/// sigma_0 (unit matrix), sigma_1, sigma_2, sigma_3
fn sig(p: usize) -> CMat {
    let (z, o, i) = (C64::ZERO, C64::ONE, C64::I);
    match p {
        0 => m2(o, z, z, o),
        1 => m2(z, o, o, z),
        2 => m2(z, -i, i, z),
        _ => m2(o, z, z, -o),
    }
}

fn smul(m: &CMat, s: C64) -> CMat {
    let mut r = m.clone();
    for x in r.a.iter_mut() {
        *x = *x * s;
    }
    r
}

fn det2(m: &CMat) -> C64 {
    m.get(0, 0) * m.get(1, 1) - m.get(0, 1) * m.get(1, 0)
}

fn close(a: &CMat, b: &CMat, tol: f64) -> bool {
    a.sub(b).fro() < tol
}

/// M_LL(b) for the bi-quaternion b = sum_p xi_p e_p
fn mq(xi: &[C64; 4]) -> CMat {
    let mut m = smul(&sig(0), xi[0]);
    for j in 1..4 {
        m = m.add(&smul(&sig(j), xi[j] * c(0.0, -1.0)));
    }
    m
}

/// M_LL(x) = x_LL^0 sigma_0 + x_LL^1 sigma_1 + x_LL^2 sigma_2 + x_LL^3 sigma_3, the matrix of the point x of R^(1,3)_LL
fn xmat(x: &[f64; 4]) -> CMat {
    let mut m = CMat::zeros(2);
    for p in 0..4 {
        m = m.add(&smul(&sig(p), c(x[p], 0.0)));
    }
    m
}

/// the coordinates of the point x with the Hermitian 2 x 2 matrix M_LL(x): x_LL^p = tr(sigma_p M_LL(x))/2
fn coords(xm: &CMat) -> [f64; 4] {
    let mut x = [0.0; 4];
    for p in 0..4 {
        x[p] = 0.5 * sig(p).mul(xm).trace().re;
    }
    x
}

/// N_LL on R^(1,3)_LL: (x_LL^0)^2 - (x_LL^1)^2 - (x_LL^2)^2 - (x_LL^3)^2
fn nmink(x: &[f64; 4]) -> f64 {
    x[0] * x[0] - x[1] * x[1] - x[2] * x[2] - x[3] * x[3]
}

type R4 = [[f64; 4]; 4];

/// Lmat(gL): the real 4 x 4 matrix of M_LL(x) -> M_LL(gL) M_LL(x) M_LL(gL)^dag in the coordinates
/// x_LL^0..x_LL^3 (the argument of the function is the 2 x 2 matrix M_LL(gL))
fn lmat(m: &CMat) -> R4 {
    let mut l = [[0.0; 4]; 4];
    for p in 0..4 {
        for q in 0..4 {
            let t = sig(p).mul(m).mul(&sig(q)).mul(&m.dagger()).trace();
            l[p][q] = 0.5 * t.re;
        }
    }
    l
}

fn rmul(a: &R4, b: &R4) -> R4 {
    let mut r = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                r[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    r
}

fn rsub(a: &R4, b: &R4) -> R4 {
    let mut r = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            r[i][j] = a[i][j] - b[i][j];
        }
    }
    r
}

fn rtr(a: &R4) -> R4 {
    let mut r = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            r[i][j] = a[j][i];
        }
    }
    r
}

fn rnorm(a: &R4) -> f64 {
    a.iter().flat_map(|r| r.iter()).map(|x| x * x).sum::<f64>().sqrt()
}

fn rdet(a: &R4) -> f64 {
    // Gaussian elimination with partial pivoting
    let mut m = *a;
    let mut d = 1.0;
    for col in 0..4 {
        let mut piv = col;
        for r in col + 1..4 {
            if m[r][col].abs() > m[piv][col].abs() {
                piv = r;
            }
        }
        if m[piv][col].abs() < 1e-300 {
            return 0.0;
        }
        if piv != col {
            m.swap(piv, col);
            d = -d;
        }
        d *= m[col][col];
        for r in col + 1..4 {
            let f = m[r][col] / m[col][col];
            for k in col..4 {
                m[r][k] -= f * m[col][k];
            }
        }
    }
    d
}

fn apply(l: &R4, x: &[f64; 4]) -> [f64; 4] {
    let mut y = [0.0; 4];
    for p in 0..4 {
        for q in 0..4 {
            y[p] += l[p][q] * x[q];
        }
    }
    y
}

const ETA: R4 = [[1.0, 0.0, 0.0, 0.0], [0.0, -1.0, 0.0, 0.0], [0.0, 0.0, -1.0, 0.0], [0.0, 0.0, 0.0, -1.0]];

/// rotation about the axis j (1..3) by theta: exp(-i theta sigma_j / 2)
fn rot(j: usize, th: f64) -> CMat {
    smul(&sig(0), c((0.5 * th).cos(), 0.0)).add(&smul(&sig(j), c(0.0, -(0.5 * th).sin())))
}

/// boost along the axis j (1..3) with the rapidity theta: exp(theta sigma_j / 2)
fn boost(j: usize, th: f64) -> CMat {
    smul(&sig(0), c((0.5 * th).cosh(), 0.0)).add(&smul(&sig(j), c((0.5 * th).sinh(), 0.0)))
}

/// V_LL(uL): the real 4 x 4 matrix of the infinitesimal action M_LL(x) -> M_LL(uL) M_LL(x) +
/// M_LL(x) M_LL(uL)^dag (the argument of the function is the 2 x 2 matrix M_LL(uL))
fn gen(u: &CMat) -> R4 {
    let mut g = [[0.0; 4]; 4];
    for q in 0..4 {
        let img = u.mul(&sig(q)).add(&sig(q).mul(&u.dagger()));
        let x = coords(&img);
        for p in 0..4 {
            g[p][q] = x[p];
        }
    }
    g
}

fn eps3(i: usize, j: usize, k: usize) -> f64 {
    // Levi-Civita symbol eps3 for indices 0, 1, 2
    let v = [(0, 1, 2), (1, 2, 0), (2, 0, 1)];
    let w = [(0, 2, 1), (2, 1, 0), (1, 0, 2)];
    if v.contains(&(i, j, k)) {
        1.0
    } else if w.contains(&(i, j, k)) {
        -1.0
    } else {
        0.0
    }
}

// ------------------------------------------------------------------ random numbers

/// SplitMix64: a small deterministic pseudo-random generator (fixed seeds, reproducible output)
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
    /// uniform in [-1, 1)
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1u64 << 53) as f64 * 2.0 - 1.0
    }
}

fn csqrt(z: C64) -> C64 {
    let r = z.abs().sqrt();
    let th = 0.5 * z.im.atan2(z.re);
    c(r * th.cos(), r * th.sin())
}

/// a random element of SL(2,C): random entries, divided by a square root of the determinant
fn rand_sl2c(rng: &mut Rng) -> CMat {
    loop {
        let m = m2(c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()));
        let d = det2(&m);
        if d.abs() > 0.1 {
            let s = csqrt(d);
            let inv = C64::ONE / s;
            return smul(&m, inv);
        }
    }
}

fn fnum(x: f64) -> String {
    if x == 0.0 {
        return "0".to_string();
    }
    let s = format!("{:.6}", x);
    if s == "-0.000000" {
        "0".to_string()
    } else {
        s
    }
}

fn fsci(x: f64) -> String {
    format!("{:.6e}", x)
}

fn fpt(x: &[f64; 4]) -> String {
    format!("({}, {}, {}, {})", fnum(x[0]), fnum(x[1]), fnum(x[2]), fnum(x[3]))
}

// ------------------------------------------------------------------ power counting

/// can external lines with these numbers conserve the fermion number? (fermion lines carry +-1,
/// lines of the pair field +-2; the total must vanish)
fn f_allowed(nf: i64, nb: i64) -> bool {
    let mut sf = -nf;
    while sf <= nf {
        let mut sb = -2 * nb;
        while sb <= 2 * nb {
            if sf + sb == 0 {
                return true;
            }
            sb += 4;
        }
        sf += 2;
    }
    false
}

// ------------------------------------------------------------------ the bubble

/// angular average over the 3-sphere of the integrand of Ibub, for the lengths |kE| (argument k)
/// and |pE| (argument p) and the mass m
fn ang_avg(k: f64, p: f64, m: f64, gx: &[f64], gw: &[f64]) -> f64 {
    // the average of a function of the angle ang between kE and pE over the 3-sphere is
    // (2/pi) int_0^pi sin^2(ang) (the function) d(ang); Gauss-Legendre in ang
    let mut s = 0.0;
    for j in 0..gx.len() {
        let ang = 0.5 * PI * (gx[j] + 1.0);
        let w = 0.5 * PI * gw[j];
        let cc = ang.cos();
        let num = k * k + k * p * cc + m * m;
        let den = (k * k + m * m) * (k * k + 2.0 * k * p * cc + p * p + m * m);
        s += w * ang.sin().powi(2) * num / den;
    }
    s * 2.0 / PI
}

/// the coefficient of pE^2 in the angular average at small pE: -1/(kE^2 + m^2)^2 + kE^2/(2 (kE^2 + m^2)^3)
/// = -(kE^2 + 2 m^2)/(2 (kE^2 + m^2)^3), with |kE| the argument k
fn c1(k: f64, m: f64) -> f64 {
    let a = k * k + m * m;
    -1.0 / (a * a) + k * k / (2.0 * a * a * a)
}

/// the integral from 0 to Lambda of |kE|^3/(8 pi^2) times a function of |kE| (the closure h), over
/// the length |kE|; Gauss-Legendre on geometric panels
fn radial<F: Fn(f64) -> f64>(lam: f64, h: F, gx: &[f64], gw: &[f64]) -> f64 {
    let mut br = vec![0.0];
    let mut x = 0.25;
    while x < lam {
        br.push(x);
        x *= 1.5;
    }
    br.push(lam);
    let mut s = 0.0;
    for w in br.windows(2) {
        let (a, b) = (w[0], w[1]);
        for i in 0..gx.len() {
            let k = 0.5 * (b - a) * gx[i] + 0.5 * (a + b);
            s += 0.5 * (b - a) * gw[i] * k.powi(3) * h(k);
        }
    }
    s / (8.0 * PI * PI)
}

fn main() {
    if !std::path::Path::new("code/r13-rs/Cargo.toml").exists() {
        eprintln!("r13_yukawa: run this program from the folder Revision_SuperLoop/explanations");
        std::process::exit(2);
    }
    let mut ck = Checks::new("Revision_SuperLoop, folder explanations, program `r13_yukawa`: the 3+1 manifold R^(1,3)_LL of the super-loop and the renormalizability of the Yukawa form on it");
    let mut rng = Rng(2026_11_17);

    // ================================================================ Z-1 coordinates
    // the matrices of the basis e0, i e1, i e2, i e3 of R^(1,3)_LL
    let mut z1 = true;
    let basis_xi: [[C64; 4]; 4] = [
        [C64::ONE, C64::ZERO, C64::ZERO, C64::ZERO],
        [C64::ZERO, C64::I, C64::ZERO, C64::ZERO],
        [C64::ZERO, C64::ZERO, C64::I, C64::ZERO],
        [C64::ZERO, C64::ZERO, C64::ZERO, C64::I],
    ];
    for p in 0..4 {
        z1 &= close(&mq(&basis_xi[p]), &sig(p), TOL);
    }
    let mut n_rand = 0;
    for _ in 0..200 {
        let x = [2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit()];
        // the bi-quaternion x = x_LL^0 e0 + i sum_j x_LL^j e_j: coefficients (x_LL^0, i x_LL^1, i x_LL^2, i x_LL^3)
        let xi = [c(x[0], 0.0), c(0.0, x[1]), c(0.0, x[2]), c(0.0, x[3])];
        let xm = mq(&xi);
        z1 &= close(&xm, &xmat(&x), TOL);
        z1 &= close(&xm, &xm.dagger(), TOL);
        let nq: C64 = xi.iter().map(|&w| w * w).fold(C64::ZERO, |a, b| a + b);
        z1 &= (det2(&xm) - nq).abs() < TOL && (nq - c(nmink(&x), 0.0)).abs() < TOL;
        let back = coords(&xm);
        z1 &= (0..4).all(|p| (back[p] - x[p]).abs() < TOL);
        n_rand += 1;
    }
    ck.add("Z-1", "the coordinates of R^(1,3)_LL: the Hermitian bi-quaternion x = x_LL^0 e0 + i (x_LL^1 e1 + x_LL^2 e2 + x_LL^3 e3) with real coordinates has the matrix M_LL(x) = x_LL^0 sigma_0 + x_LL^1 sigma_1 + x_LL^2 sigma_2 + x_LL^3 sigma_3 (M_LL(e0) = sigma_0, M_LL(i e_j) = sigma_j), which is Hermitian; det M_LL(x) = N_LL(x) = (x_LL^0)^2 - (x_LL^1)^2 - (x_LL^2)^2 - (x_LL^3)^2; the coordinates are recovered as x_LL^0 = tr M_LL(x)/2 and x_LL^j = tr(sigma_j M_LL(x))/2", z1, format!("the four basis elements and {} random points", n_rand));

    // ================================================================ Z-2 signature
    // the symmetric matrix of the quadratic form N_LL in the coordinates, by polarization of det M_LL(x)
    let mut gram = [[0.0; 4]; 4];
    for p in 0..4 {
        for q in 0..4 {
            let mut ep = [0.0; 4];
            ep[p] += 1.0;
            let mut eq = [0.0; 4];
            eq[q] += 1.0;
            let mut s = [0.0; 4];
            for r in 0..4 {
                s[r] = ep[r] + eq[r];
            }
            gram[p][q] = 0.5 * (det2(&xmat(&s)).re - det2(&xmat(&ep)).re - det2(&xmat(&eq)).re);
        }
    }
    let z2 = rnorm(&rsub(&gram, &ETA)) < TOL;
    ck.add("Z-2", "R^(1,3)_LL is Minkowski space: the symmetric matrix of the quadratic form N_LL = det M_LL(x) in the coordinates x_LL^0..x_LL^3 (by polarization) is diag(1, -1, -1, -1): signature (1 positive, 3 negative)", z2, format!("matrix diag({}, {}, {}, {})", fnum(gram[0][0]), fnum(gram[1][1]), fnum(gram[2][2]), fnum(gram[3][3])));

    // ================================================================ Z-3 the Lorentz group
    let mut z3 = true;
    let mut worst = 0.0f64;
    for _ in 0..200 {
        let m = rand_sl2c(&mut rng);
        z3 &= (det2(&m) - C64::ONE).abs() < 1e-10;
        let x = [2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit()];
        let xm = xmat(&x);
        let y = m.mul(&xm).mul(&m.dagger());
        z3 &= close(&y, &y.dagger(), 1e-10);
        worst = worst.max((det2(&y).re - det2(&xm).re).abs());
        let l = lmat(&m);
        // Lmat^T diag(1,-1,-1,-1) Lmat = diag(1,-1,-1,-1), det Lmat = 1, Lmat[0][0] >= 1 (proper, orthochronous)
        z3 &= rnorm(&rsub(&rmul(&rtr(&l), &rmul(&ETA, &l)), &ETA)) < 1e-9;
        z3 &= (rdet(&l) - 1.0).abs() < 1e-9 && l[0][0] >= 1.0 - 1e-12;
        // Lmat acts on the coordinates as M_LL(x) -> M_LL(gL) M_LL(x) M_LL(gL)^dag does
        let yx = apply(&l, &x);
        let yc = coords(&y);
        z3 &= (0..4).all(|p| (yx[p] - yc[p]).abs() < 1e-9);
        // gL and -gL give the same Lorentz transformation (double cover)
        let lm = lmat(&smul(&m, c(-1.0, 0.0)));
        z3 &= rnorm(&rsub(&l, &lm)) < 1e-12;
        // composition: Lmat(gL hL) = Lmat(gL) Lmat(hL)
        let m2_ = rand_sl2c(&mut rng);
        z3 &= rnorm(&rsub(&lmat(&m.mul(&m2_)), &rmul(&l, &lmat(&m2_)))) < 1e-9;
    }
    z3 &= worst < 1e-9;
    ck.add("Z-3", "the Lorentz group of R^(1,3)_LL: for 200 random unit bi-quaternions gL (random matrices M_LL(gL) in SL(2,C): four entries with real and imaginary parts uniform in [-1, 1), accepted if the modulus of the determinant exceeds 0.1, divided by a square root of the determinant; tolerances 1e-10 for the 2 x 2 matrices, 1e-9 for Lmat and the coordinates, 1e-12 for Lmat(gL) against Lmat(-gL)), M_LL(x) -> M_LL(gL) M_LL(x) M_LL(gL)^dag keeps the matrix Hermitian and keeps N_LL(x); its matrix Lmat in the coordinates satisfies Lmat^T diag(1,-1,-1,-1) Lmat = diag(1,-1,-1,-1), det Lmat = 1 and Lmat[0][0] >= 1 (a proper orthochronous Lorentz transformation); gL and -gL give the same Lmat (SL(2,C) is the double cover), and Lmat(gL hL) = Lmat(gL) Lmat(hL) (a group homomorphism)", z3, format!("largest change of N_LL: {:.1e}", worst));

    // ================================================================ Z-4 the six generators
    let mut rgen: Vec<R4> = Vec::new();
    let mut bgen: Vec<R4> = Vec::new();
    let mut z4 = true;
    for j in 1..4 {
        // M_LL(J_LL^j) = M_LL(e_j/2) = -i sigma_j / 2, M_LL(K_LL^j) = M_LL(i e_j/2) = sigma_j / 2
        let mut xj = [C64::ZERO; 4];
        xj[j] = c(0.5, 0.0);
        let mut xk = [C64::ZERO; 4];
        xk[j] = c(0.0, 0.5);
        let mj = mq(&xj);
        let mk = mq(&xk);
        z4 &= close(&mj, &smul(&sig(j), c(0.0, -0.5)), TOL) && close(&mk, &smul(&sig(j), c(0.5, 0.0)), TOL);
        let gr = gen(&mj);
        let gb = gen(&mk);
        // the derivatives of the finite transformations at theta = 0 agree with the generators
        let h = 1e-5;
        let dr = rsub(&lmat(&rot(j, h)), &lmat(&rot(j, -h)));
        let db = rsub(&lmat(&boost(j, h)), &lmat(&boost(j, -h)));
        let mut dr2 = [[0.0; 4]; 4];
        let mut db2 = [[0.0; 4]; 4];
        for p in 0..4 {
            for q in 0..4 {
                dr2[p][q] = dr[p][q] / (2.0 * h);
                db2[p][q] = db[p][q] / (2.0 * h);
            }
        }
        z4 &= rnorm(&rsub(&dr2, &gr)) < 1e-8 && rnorm(&rsub(&db2, &gb)) < 1e-8;
        // each generator preserves the metric: V_LL(uL)^T diag(1,-1,-1,-1) + diag(1,-1,-1,-1) V_LL(uL) = 0
        for g in [&gr, &gb] {
            let s = rmul(&rtr(g), &ETA);
            let t = rmul(&ETA, g);
            let mut sum = [[0.0; 4]; 4];
            for p in 0..4 {
                for q in 0..4 {
                    sum[p][q] = s[p][q] + t[p][q];
                }
            }
            z4 &= rnorm(&sum) < TOL;
        }
        rgen.push(gr);
        bgen.push(gb);
    }
    // the commutation relations of so(3,1)
    let comm = |a: &R4, b: &R4| rsub(&rmul(a, b), &rmul(b, a));
    let lin = |coef: &[f64; 3], v: &Vec<R4>| {
        let mut r = [[0.0; 4]; 4];
        for k in 0..3 {
            for p in 0..4 {
                for q in 0..4 {
                    r[p][q] += coef[k] * v[k][p][q];
                }
            }
        }
        r
    };
    let mut relations = 0;
    for i in 0..3 {
        for j in 0..3 {
            let e = [eps3(i, j, 0), eps3(i, j, 1), eps3(i, j, 2)];
            let me = [-e[0], -e[1], -e[2]];
            // with rgen[j] = V_LL(J_LL^j) and bgen[j] = V_LL(K_LL^j):
            // [V_LL(J_LL^j), V_LL(J_LL^j')] = sum eps3 V_LL(J_LL^j''), [V_LL(J_LL^j), V_LL(K_LL^j')] = sum eps3 V_LL(K_LL^j''),
            // [V_LL(K_LL^j), V_LL(K_LL^j')] = -sum eps3 V_LL(J_LL^j'')
            z4 &= rnorm(&rsub(&comm(&rgen[i], &rgen[j]), &lin(&e, &rgen))) < TOL;
            z4 &= rnorm(&rsub(&comm(&rgen[i], &bgen[j]), &lin(&e, &bgen))) < TOL;
            z4 &= rnorm(&rsub(&comm(&bgen[i], &bgen[j]), &lin(&me, &rgen))) < TOL;
            relations += 3;
        }
    }
    // the six generators are linearly independent: the Gram matrix of their 16 entries is invertible
    let all: Vec<&R4> = rgen.iter().chain(bgen.iter()).collect();
    let mut gm = vec![vec![0.0; 6]; 6];
    for a in 0..6 {
        for b in 0..6 {
            gm[a][b] = (0..4).flat_map(|p| (0..4).map(move |q| (p, q))).map(|(p, q)| all[a][p][q] * all[b][p][q]).sum::<f64>();
        }
    }
    let mut rank = 0;
    {
        let mut m = gm.clone();
        let mut row = 0;
        for col in 0..6 {
            let mut piv = row;
            while piv < 6 && m[piv][col].abs() < 1e-9 {
                piv += 1;
            }
            if piv == 6 {
                continue;
            }
            m.swap(piv, row);
            for r in 0..6 {
                if r != row {
                    let f = m[r][col] / m[row][col];
                    for k in 0..6 {
                        m[r][k] -= f * m[row][k];
                    }
                }
            }
            row += 1;
            rank += 1;
            if row == 6 {
                break;
            }
        }
    }
    z4 &= rank == 6;
    ck.add("Z-4", "the six generators: the rotation generators J_LL^j = e_j/2 and the boost generators K_LL^j = i e_j/2 of the super-loop have M_LL(J_LL^j) = -i sigma_j/2 and M_LL(K_LL^j) = sigma_j/2; their action M_LL(x) -> M_LL(uL) M_LL(x) + M_LL(x) M_LL(uL)^dag on R^(1,3)_LL gives six real 4 x 4 matrices V_LL(J_LL^j), V_LL(K_LL^j) (equal, within 1e-8, to the derivatives at theta = 0 of Lmat of the finite rotations exp(-i theta sigma_j/2) and boosts exp(theta sigma_j/2), computed by central differences with the step 1e-5), each preserving the metric (V_LL(uL)^T diag(1,-1,-1,-1) + diag(1,-1,-1,-1) V_LL(uL) = 0), linearly independent (the Gram matrix of their 16 entries has rank 6 in a Gaussian elimination with the pivot threshold 1e-9), with the relations of so(3,1): [V_LL(J_LL^j), V_LL(J_LL^j')] = sum_j'' eps3_jj'j'' V_LL(J_LL^j''), [V_LL(J_LL^j), V_LL(K_LL^j')] = sum_j'' eps3_jj'j'' V_LL(K_LL^j''), [V_LL(K_LL^j), V_LL(K_LL^j')] = -sum_j'' eps3_jj'j'' V_LL(J_LL^j'') (two boosts close into a rotation with the minus sign that distinguishes the Lorentz algebra from so(4))", z4, format!("rank {}, {} commutation relations", rank, relations));

    // ================================================================ Z-5 worked examples
    let mut z5 = true;
    let mut pts: Vec<Vec<String>> = Vec::new();
    let kind = |n: f64| if n.abs() < 1e-12 { "null (on the light cone)" } else if n > 0.0 { "timelike" } else { "spacelike" };
    let push = |name: &str, x: &[f64; 4], pts: &mut Vec<Vec<String>>| {
        let n = nmink(x);
        pts.push(vec![name.to_string(), fnum(x[0]), fnum(x[1]), fnum(x[2]), fnum(x[3]), fnum(n), kind(n).to_string()]);
    };
    // (a) a point and its matrix
    let pp = [2.0, 1.2, 1.5, 1.0];
    let xp = xmat(&pp);
    z5 &= close(&xp, &m2(c(3.0, 0.0), c(1.2, -1.5), c(1.2, 1.5), c(1.0, 0.0)), TOL);
    z5 &= (det2(&xp).re - (-0.69)).abs() < TOL;
    push("the sample point", &pp, &mut pts);
    // (b) the event (1,0,0,0) boosted along x_LL^1 with the rapidity 0.8
    let e1 = [1.0, 0.0, 0.0, 0.0];
    let th = 0.8;
    let b1 = apply(&lmat(&boost(1, th)), &e1);
    z5 &= (b1[0] - th.cosh()).abs() < TOL && (b1[1] - th.sinh()).abs() < TOL && b1[2].abs() < TOL && b1[3].abs() < TOL;
    z5 &= (nmink(&b1) - 1.0).abs() < TOL;
    push("the event (1, 0, 0, 0)", &e1, &mut pts);
    push("the same event after the boost along x_LL^1 with the rapidity 0.8", &b1, &mut pts);
    // (c) the event (0,1,0,0) rotated about x_LL^3 by pi/3
    let e2 = [0.0, 1.0, 0.0, 0.0];
    let r1 = apply(&lmat(&rot(3, PI / 3.0)), &e2);
    z5 &= r1[0].abs() < TOL && (r1[1] - 0.5).abs() < TOL && (r1[2] - (3.0f64).sqrt() / 2.0).abs() < TOL && r1[3].abs() < TOL;
    push("the event (0, 1, 0, 0)", &e2, &mut pts);
    push("the same event after the rotation about x_LL^3 by the angle pi/3", &r1, &mut pts);
    // (d) the rotation by 2 pi: M_LL(gL) = -sigma_0, Lmat = identity
    let m2pi = rot(3, 2.0 * PI);
    z5 &= close(&m2pi, &smul(&sig(0), c(-1.0, 0.0)), 1e-12);
    let mut id4 = [[0.0; 4]; 4];
    for p in 0..4 {
        id4[p][p] = 1.0;
    }
    z5 &= rnorm(&rsub(&lmat(&m2pi), &id4)) < 1e-12;
    // (e) a Weyl spinor chi gives the null point chi chi^dag; boosts move it along the light cone
    let chi = (C64::ONE, C64::ZERO);
    let xchi = m2(chi.0 * chi.0.conj(), chi.0 * chi.1.conj(), chi.1 * chi.0.conj(), chi.1 * chi.1.conj());
    let pchi = coords(&xchi);
    z5 &= (pchi[0] - 0.5).abs() < TOL && (pchi[3] - 0.5).abs() < TOL && pchi[1].abs() < TOL && pchi[2].abs() < TOL;
    let mb = boost(3, th);
    let chib = (mb.get(0, 0) * chi.0 + mb.get(0, 1) * chi.1, mb.get(1, 0) * chi.0 + mb.get(1, 1) * chi.1);
    let xchib = m2(chib.0 * chib.0.conj(), chib.0 * chib.1.conj(), chib.1 * chib.0.conj(), chib.1 * chib.1.conj());
    z5 &= close(&xchib, &mb.mul(&xchi).mul(&mb.dagger()), TOL);
    let pchib = coords(&xchib);
    z5 &= (pchib[0] - 0.5 * th.exp()).abs() < TOL && (pchib[3] - 0.5 * th.exp()).abs() < TOL;
    push("the null point of the spinor chi = (1, 0)", &pchi, &mut pts);
    push("the same after the boost of chi along x_LL^3 with the rapidity 0.8", &pchib, &mut pts);
    // the Lorentz transformation acts on each column of the Weyl isodoublet psi'_LL = eps2^-1 M_LL(bS)^T of a
    // spinor charge bS e4 (bS -> bS gLbar) as chi -> M_LL(gL) chi: eps2^-1 M_LL(gLbar)^T eps2 = M_LL(gL), where
    // M_LL(gLbar) = M_LL(gL)^-1 for a unit bi-quaternion
    let eps2 = m2(C64::ZERO, C64::ONE, -C64::ONE, C64::ZERO);
    let eps2inv = m2(C64::ZERO, -C64::ONE, C64::ONE, C64::ZERO);
    let mut ncol = 0;
    for _ in 0..200 {
        let mg = rand_sl2c(&mut rng);
        // the inverse of a 2 x 2 matrix of determinant 1: exchange the two diagonal entries and change
        // the signs of the two off-diagonal entries
        let mbar = m2(mg.get(1, 1), -mg.get(0, 1), -mg.get(1, 0), mg.get(0, 0));
        z5 &= close(&mg.mul(&mbar), &sig(0), 1e-10);
        let mut mbt = CMat::zeros(2);
        for i in 0..2 {
            for j in 0..2 {
                mbt.set(i, j, mbar.get(j, i));
            }
        }
        z5 &= close(&eps2inv.mul(&mbt).mul(&eps2), &mg, 1e-10);
        ncol += 1;
    }
    let mut nnull = 0;
    for _ in 0..200 {
        let a = c(rng.unit(), rng.unit());
        let b = c(rng.unit(), rng.unit());
        let xm = m2(a * a.conj(), a * b.conj(), b * a.conj(), b * b.conj());
        let x = coords(&xm);
        z5 &= nmink(&x).abs() < 1e-12 && x[0] > 0.0;
        nnull += 1;
    }
    ck.add("Z-5", "worked examples on R^(1,3)_LL: the sample point (2, 1.2, 1.5, 1.0) has M_LL(x) = [[3, 1.2 - 1.5 i], [1.2 + 1.5 i, 1]] and N_LL = -0.69 (spacelike); the boost along x_LL^1 with the rapidity 0.8 takes (1, 0, 0, 0) to (cosh 0.8, sinh 0.8, 0, 0); the rotation about x_LL^3 by pi/3 takes (0, 1, 0, 0) to (0, 1/2, sqrt3/2, 0); the rotation by 2 pi has M_LL(gL) = -sigma_0 and Lmat = identity; a two-component spinor chi gives the null point with M_LL(x) = chi chi^dag (chi = (1, 0): (1/2, 0, 0, 1/2)), and the boost of chi along x_LL^3 with the rapidity 0.8 moves it along the light cone to exp(0.8) (1/2, 0, 0, 1/2); chi chi^dag is a future null point for 200 random chi; the Lorentz transformation gL acts on each column of the Weyl isodoublet psi'_LL = eps2^-1 M_LL(bS)^T of a spinor charge (bS -> bS gLbar) as chi -> M_LL(gL) chi, because eps2^-1 M_LL(gLbar)^T eps2 = M_LL(gL) (200 random gL as in Z-3, tolerance 1e-10)", z5, format!("sample point: N_LL = {}; boost: {}; rotation: {}; null points checked: {}; columns checked: {}", fnum(det2(&xp).re), fpt(&b1), fpt(&r1), nnull, ncol));

    // curves for the figures
    let mut orbits: Vec<Vec<String>> = Vec::new();
    let add_curve = |name: &str, params: &[f64], f: &dyn Fn(f64) -> [f64; 4], out: &mut Vec<Vec<String>>| {
        for &t in params {
            let x = f(t);
            out.push(vec![name.to_string(), fnum(t), fnum(x[0]), fnum(x[1]), fnum(x[2]), fnum(x[3])]);
        }
    };
    let rap: Vec<f64> = (0..=60).map(|k| -1.2 + 2.4 * k as f64 / 60.0).collect();
    for (name, x0) in [
        ("boost orbit of (1, 0, 0, 0) along x_LL^1", [1.0, 0.0, 0.0, 0.0]),
        ("boost orbit of (2, 0, 0, 0) along x_LL^1", [2.0, 0.0, 0.0, 0.0]),
        ("boost orbit of (0, 1, 0, 0) along x_LL^1", [0.0, 1.0, 0.0, 0.0]),
        ("boost orbit of (1, 1, 0, 0) along x_LL^1", [1.0, 1.0, 0.0, 0.0]),
    ] {
        add_curve(name, &rap, &|t| apply(&lmat(&boost(1, t)), &x0), &mut orbits);
    }
    let ang: Vec<f64> = (0..=72).map(|k| 2.0 * PI * k as f64 / 72.0).collect();
    for (name, x0) in [
        ("rotation orbit of (0, 1, 0, 0) about x_LL^3", [0.0, 1.0, 0.0, 0.0]),
        ("rotation orbit of (0, 1.5, 0, 0) about x_LL^3", [0.0, 1.5, 0.0, 0.0]),
    ] {
        add_curve(name, &ang, &|t| apply(&lmat(&rot(3, t)), &x0), &mut orbits);
    }
    // a square grid in the (x_LL^0, x_LL^1) plane and its image under the boost with the rapidity 0.5
    let sp: Vec<f64> = (0..=20).map(|k| -1.5 + 3.0 * k as f64 / 20.0).collect();
    let lb = lmat(&boost(1, 0.5));
    for cval in [-1.0, -0.5, 0.0, 0.5, 1.0] {
        add_curve(&format!("grid line x_LL^1 = {}, original", cval), &sp, &|s| [s, cval, 0.0, 0.0], &mut orbits);
        add_curve(&format!("grid line x_LL^0 = {}, original", cval), &sp, &|s| [cval, s, 0.0, 0.0], &mut orbits);
        add_curve(&format!("grid line x_LL^1 = {}, boosted with the rapidity 0.5", cval), &sp, &|s| apply(&lb, &[s, cval, 0.0, 0.0]), &mut orbits);
        add_curve(&format!("grid line x_LL^0 = {}, boosted with the rapidity 0.5", cval), &sp, &|s| apply(&lb, &[cval, s, 0.0, 0.0]), &mut orbits);
    }
    // the light-cone ray of chi = (1, 0) under boosts along x_LL^3
    add_curve("null points exp(theta) (1/2, 0, 0, 1/2) of the boosted spinor chi", &rap, &|t| {
        let mb = boost(3, t);
        coords(&mb.mul(&xchi).mul(&mb.dagger()))
    }, &mut orbits);

    // ================================================================ Z-6 mass dimensions
    // twice the mass dimensions (integers) from the conditions that every term of the Lagrangian
    // density has the dimension D + 1: kinetic terms 2[Psi_LL] + 1 = D + 1, 2[phi] + 2 = D + 1; contact
    // [G] + 4[Psi_LL] = D + 1; Yukawa [g_Y] + [phi] + 2[Psi_LL] = D + 1; quartic [lam4] + 4[phi] = D + 1.
    let mut dims_rows: Vec<Vec<String>> = Vec::new();
    let mut z6 = true;
    let half = |n2: i64| if n2 % 2 == 0 { format!("{}", n2 / 2) } else { format!("{}/2", n2) };
    let mut doubled = Vec::new();
    for dd in [3i64, 7] {
        let d1 = dd + 1; // the space-time dimension D + 1
        let psi2 = d1 - 1; //            2[Psi_LL]  from 2[Psi_LL] + 1 = D + 1 (the kinetic term of the fermions)
        let phi2 = d1 - 2; //            2[phi]  from 2[phi] + 2 = D + 1 (the kinetic term of the pair field)
        let gc2 = 2 * d1 - 4 * psi2; //  2[G]    from [G] + 4[Psi_LL] = D + 1 (the contact term)
        let gy2 = 2 * d1 - phi2 - 2 * psi2; // 2[g_Y] from [g_Y] + [phi] + 2[Psi_LL] = D + 1 (the Yukawa term)
        let l2 = 2 * d1 - 4 * phi2; //   2[lam4] from [lam4] + 4[phi] = D + 1 (the quartic terms)
        // the closed forms: [Psi_LL] = D/2, [phi] = (D-1)/2, [G] = 1 - D, [g_Y] = (3-D)/2, [lam4] = 3 - D
        z6 &= (psi2, phi2, gc2, gy2, l2) == (dd, dd - 1, 2 * (1 - dd), 3 - dd, 2 * (3 - dd));
        // G = 4 g_Y^2/m_phi^2: [G] = 2[g_Y] - 2
        z6 &= gc2 == 2 * gy2 - 4;
        let yuk = if gy2 >= 0 && l2 >= 0 { "yes" } else { "no" };
        let con = if gc2 >= 0 { "yes" } else { "no" };
        dims_rows.push(vec![format!("{}", dd), format!("{}", d1), half(psi2), half(phi2), half(gc2), half(gy2), half(l2),
                            format!("contact form: {}; Yukawa form: {}", con, yuk)]);
        doubled.push((dd, yuk, con));
    }
    z6 &= doubled == vec![(3, "yes", "no"), (7, "no", "no")];
    ck.add("Z-6", "mass dimensions in D + 1 space-time dimensions (every term of the Lagrangian density has the dimension D + 1): [Psi_LL] = D/2, [phi] = (D-1)/2, the contact coupling [G] = 1 - D, the Yukawa coupling [g_Y] = (3-D)/2, the quartic couplings [lam4] = 3 - D; for D = 3 (the manifold R^(1,3)_LL): 3/2, 1, -2, 0, 0 -- the Yukawa form has only couplings of dimension 0 and is renormalizable by power counting, the contact form is not; for D = 7 (R^(1,7)_LL, and the 4+4 manifold, where power counting sees the total dimension 8): 7/2, 3, -6, -2, -4 -- every coupling has a negative dimension; below the mass m_phi of the pair field its exchange gives the contact coupling G = 4 g_Y^2/m_phi^2, consistent with [G] = 2[g_Y] - 2", z6,
           format!("D = 3: {}, {}, {}, {}, {}; D = 7: {}, {}, {}, {}, {}", dims_rows[0][2], dims_rows[0][3], dims_rows[0][4], dims_rows[0][5], dims_rows[0][6], dims_rows[1][2], dims_rows[1][3], dims_rows[1][4], dims_rows[1][5], dims_rows[1][6]));

    // ================================================================ Z-7 power counting
    let mut pc_rows: Vec<Vec<String>> = Vec::new();
    let mut z7 = true;
    let mut ncompared = 0;
    // divergent, fermion-number-allowed functions (next_F, next_B) with at least one loop, by the
    // largest number of vertices allowed
    let mut counts: Vec<Vec<String>> = Vec::new();
    let mut yuk3_list: Vec<(i64, i64, i64)> = Vec::new();
    for dd in [3i64, 7] {
        let d1 = dd + 1;
        let mut by_vmax_y: Vec<std::collections::BTreeSet<(i64, i64)>> = vec![Default::default(); 7];
        let mut by_vmax_c: Vec<std::collections::BTreeSet<(i64, i64)>> = vec![Default::default(); 7];
        // the Yukawa-quartic theory
        for ny in 0..=6i64 {
            for n4 in 0..=6i64 {
                let nv = ny + n4;
                if nv < 1 || nv > 6 {
                    continue;
                }
                for nf in (0..=2 * ny).step_by(2) {
                    for nb in 0..=(ny + 4 * n4) {
                        if (ny + 4 * n4 - nb) % 2 != 0 {
                            continue;
                        }
                        let nif = (2 * ny - nf) / 2;
                        let nib = (ny + 4 * n4 - nb) / 2;
                        let nloop = nif + nib - nv + 1;
                        if nloop < 1 || nf + nb == 0 {
                            continue;
                        }
                        let deg = d1 * nloop - nif - 2 * nib;
                        // the formula, doubled: 2 deg = 2(D+1) - D nf - (D-1) nb + (D-3)(ny + 2 n4)
                        // (in the code nf = next_F, nb = next_B, ny = nvert_Y, n4 = nvert_4, nif = nint_F, nib = nint_B)
                        let deg2 = 2 * d1 - dd * nf - (dd - 1) * nb + (dd - 3) * (ny + 2 * n4);
                        z7 &= 2 * deg == deg2;
                        ncompared += 1;
                        let allowed = f_allowed(nf, nb);
                        pc_rows.push(vec!["Yukawa form".into(), format!("{}", dd), format!("{}", nf), format!("{}", nb), format!("{}", ny), format!("{}", n4), format!("{}", nv),
                                          format!("{}", nloop), format!("{}", nif), format!("{}", nib), format!("{}", deg), format!("{}", deg2 as f64 / 2.0), (if allowed { "yes" } else { "no" }).into()]);
                        if deg >= 0 && allowed {
                            for vmax in (nv as usize)..=6 {
                                by_vmax_y[vmax].insert((nf, nb));
                            }
                            if dd == 3 && !yuk3_list.iter().any(|t| t.0 == nf && t.1 == nb) {
                                yuk3_list.push((nf, nb, deg));
                            }
                        }
                    }
                }
            }
        }
        // the contact theory
        for nv in 1..=6i64 {
            for nf in (2..=4 * nv).step_by(2) {
                let nif = (4 * nv - nf) / 2;
                let nloop = nif - nv + 1;
                if nloop < 1 {
                    continue;
                }
                let deg = d1 * nloop - nif;
                let deg2 = 2 * d1 - dd * nf + 2 * (dd - 1) * nv;
                z7 &= 2 * deg == deg2;
                ncompared += 1;
                pc_rows.push(vec!["contact form".into(), format!("{}", dd), format!("{}", nf), "0".into(), "0".into(), "0".into(), format!("{}", nv),
                                  format!("{}", nloop), format!("{}", nif), "0".into(), format!("{}", deg), format!("{}", deg2 as f64 / 2.0), "yes".into()]);
                if deg >= 0 {
                    for vmax in (nv as usize)..=6 {
                        by_vmax_c[vmax].insert((nf, 0));
                    }
                }
            }
        }
        for vmax in 1..=6usize {
            counts.push(vec!["Yukawa form".into(), format!("{}", dd), format!("{}", vmax), format!("{}", by_vmax_y[vmax].len())]);
            counts.push(vec!["contact form".into(), format!("{}", dd), format!("{}", vmax), format!("{}", by_vmax_c[vmax].len())]);
        }
        if dd == 3 {
            // the Yukawa form in 3+1: the same four divergent functions for every largest number of vertices >= 2
            let set3: std::collections::BTreeSet<(i64, i64)> = [(0, 2), (0, 4), (2, 0), (2, 1)].into_iter().collect();
            z7 &= (2..=6).all(|v| by_vmax_y[v] == set3);
            // the contact form in 3+1: the number grows with the number of vertices
            z7 &= (1..6).all(|v| by_vmax_c[v + 1].len() >= by_vmax_c[v].len()) && by_vmax_c[6].len() > by_vmax_c[1].len();
        } else {
            z7 &= by_vmax_y[6].len() > by_vmax_y[2].len() && by_vmax_c[6].len() > by_vmax_c[1].len();
        }
    }
    yuk3_list.sort();
    let z7_detail = format!("{} graph classes compared; Yukawa form, D = 3, divergent and allowed: {}", ncompared,
                            yuk3_list.iter().map(|t| format!("(next_F, next_B) = ({}, {}) deg {}", t.0, t.1, t.2)).collect::<Vec<_>>().join(", "));
    z7 &= yuk3_list == vec![(0, 2, 2), (0, 4, 0), (2, 0, 1), (2, 1, 0)];
    ck.add("Z-7", "power counting (all classes of graphs with 1 to 6 vertices and at least one loop; a class is a set of numbers next_F, next_B, nvert_Y, nvert_4 for which the line ends at the vertices pair into internal lines, and deg depends only on the class; topological count nloop = nint_F + nint_B - nvert + 1, deg = (D+1) nloop - nint_F - 2 nint_B, compared with the formulas deg = (D+1) - D next_F/2 - (D-1) next_B/2 + (D-3)(nvert_Y/2 + nvert_4) of the Yukawa form and deg = (D+1) - D next_F/2 + (D-1) nvert of the contact form): in 3+1 dimensions the Yukawa form has, for the graphs with at most nvert vertices and every nvert from 2 to 6, exactly four divergent functions that conserve the fermion number (the pair-field self-energy, deg 2; the fermion self-energy, deg 1; the Yukawa vertex, deg 0; the four-point function of the pair field, deg 0) -- each has a term of the Lagrangian density to absorb it; the functions with one or three pair-field lines and no fermion lines (deg 3 and 1) vanish by fermion-number conservation; the number of divergent functions of the contact form grows with the number of vertices, and so does that of both forms for D = 7", z7, z7_detail);

    // ================================================================ Z-8 the one-loop bubble
    let (gx, gw) = gauss_legendre(48);
    let (p, m) = (1.0, 0.5);
    let target = -p * p / (16.0 * PI * PI);
    let lams = [4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0, 1024.0, 2048.0];
    let mut bub_rows: Vec<Vec<String>> = Vec::new();
    let mut z8 = true;
    let mut iren_vals = Vec::new();
    let mut slopes = Vec::new();
    let mut i0_vals = Vec::new();
    for &lam in &lams {
        let i0 = radial(lam, |k| 1.0 / (k * k + m * m), &gx, &gw);
        let i0c = (lam * lam - m * m * (1.0 + lam * lam / (m * m)).ln()) / (16.0 * PI * PI);
        // check that the angular average of the integrand at pE = 0 is 1/(kE^2 + m^2)
        z8 &= (ang_avg(lam, 0.0, m, &gx, &gw) - 1.0 / (lam * lam + m * m)).abs() < 1e-14;
        // d[Ibub(pE) - Ibub(0)]/d ln Lambda = Lambda^4/(8 pi^2) times (the angular average of the integrand
        // at |kE| = Lambda, minus its value at pE = 0)
        let slope = lam.powi(4) / (8.0 * PI * PI) * (ang_avg(lam, p, m, &gx, &gw) - 1.0 / (lam * lam + m * m));
        // the renormalized bubble: Ibub(pE) - Ibub(0) - pE^2 dIbub/d(pE^2) at 0
        let iren = radial(lam, |k| ang_avg(k, p, m, &gx, &gw) - 1.0 / (k * k + m * m) - p * p * c1(k, m), &gx, &gw);
        z8 &= ((i0 - i0c) / i0c).abs() < 1e-10;
        bub_rows.push(vec![fnum(lam), fsci(i0), fsci(i0c), fsci(slope), fsci(iren)]);
        iren_vals.push(iren);
        slopes.push(slope);
        i0_vals.push(i0);
    }
    let nl = lams.len();
    // quadratic growth: Ibub(0) at 2 Lambda over Ibub(0) at Lambda tends to 4
    let ratio_q = i0_vals[nl - 1] / i0_vals[nl - 2];
    z8 &= (ratio_q - 4.0).abs() < 1e-3;
    // the logarithmic slope tends to -pE^2/(16 pi^2)
    let rel_slope = (slopes[nl - 1] - target) / target;
    z8 &= rel_slope.abs() < 1e-5;
    // the renormalized bubble converges: the changes per doubling of the cutoff, from Lambda = 4 on
    let diffs: Vec<f64> = (1..nl).map(|k| (iren_vals[k] - iren_vals[k - 1]).abs()).collect();
    let ratios: Vec<f64> = (1..4).map(|k| diffs[k - 1] / diffs[k]).collect();
    // power counting guarantees a factor of at least 4 per doubling (a tail falling like Lambda^-2);
    // here the term of order pE^4 of the angular average is pE^4 m^4/(kE^2 + m^2)^5, so the integrand of
    // the tail (with the factor |kE|^3 of the measure) falls like |kE|^-7 and each doubling divides the
    // change by about 2^6 = 64
    z8 &= ratios.iter().all(|r| *r > 16.0 && (r - 64.0).abs() < 8.0);
    let mut p4_ok = true;
    for kk in [1.0f64, 2.0, 4.0] {
        let pp = 0.05;
        let a = kk * kk + m * m;
        let rem = ang_avg(kk, pp, m, &gx, &gw) - 1.0 / a - pp * pp * c1(kk, m);
        p4_ok &= ((rem / pp.powi(4)) / (m.powi(4) / a.powi(5)) - 1.0).abs() < 1e-2;
    }
    z8 &= p4_ok;
    // from Lambda = 32 on, Iren is constant within the rounding errors of double precision
    let late = (4..nl).map(|k| ((iren_vals[k] - iren_vals[nl - 1]) / iren_vals[nl - 1]).abs()).fold(0.0f64, f64::max);
    z8 &= late < 1e-8;
    ck.add("Z-8", "the one-loop bubble of the Yukawa form, Ibub(pE) = int_(|kE| < Lambda) d^4kE/(2 pi)^4 [kE.(kE+pE) + m^2]/((kE^2 + m^2)((kE+pE)^2 + m^2)) (pE = 1, m = 0.5, Lambda = 4 to 2048; the angular average of the integrand at pE = 0 equals 1/(kE^2 + m^2) within 1e-14): Ibub(0) equals the closed form (Lambda^2 - m^2 ln(1 + Lambda^2/m^2))/(16 pi^2) (relative tolerance 1e-10) and grows like Lambda^2 (ratio at 2048/1024 within 1e-3 of 4; absorbed by the mass counterterm of the pair field); the cutoff derivative of the momentum-dependent part, computed on the sphere |kE| = Lambda, tends to -pE^2/(16 pi^2) (relative tolerance 1e-5 at Lambda = 2048; a logarithmic divergence, absorbed by the wave-function counterterm); the renormalized bubble Iren = Ibub(pE) - Ibub(0) - pE^2 dIbub/d(pE^2) at pE = 0 converges as Lambda grows: power counting guarantees a remainder falling at least like Lambda^-2 (a factor 4 per doubling); here the term of order pE^4 of the angular average of the integrand is pE^4 m^4/(kE^2 + m^2)^5 (verified at kE = 1, 2, 4 with pE = 0.05, relative tolerance 1e-2), so each doubling divides the change by about 64 (each of the first three ratios between 56 and 72), and from Lambda = 32 on Iren is constant within the rounding errors (relative changes below 1e-8): after the two counterterms nothing depends on the cutoff", z8,
           format!("Ibub(0) ratio at 2048/1024: {:.6}; slope at Lambda = 2048: {} (target {}); changes of Iren per doubling from Lambda = 4: {}, {}, {}, {} (ratios {:.3}, {:.3}, {:.3}); Iren at 2048: {}; largest relative deviation from it for Lambda >= 32: {:.1e}; pE^4 term verified: {}",
                   ratio_q, fsci(slopes[nl - 1]), fsci(target), fsci(diffs[0]), fsci(diffs[1]), fsci(diffs[2]), fsci(diffs[3]), ratios[0], ratios[1], ratios[2], fsci(iren_vals[nl - 1]), late, p4_ok));

    // ================================================================ output
    let src = "r13_yukawa (explanations/code/r13-rs), PROJECT spacetimes, Revision_SuperLoop".to_string();
    let _ = write_csv_str("ex_r13_points.csv", &[("program", src.clone()), ("content", "worked examples on R^(1,3)_LL: points with their coordinates x_LL^0..x_LL^3, N_LL = det M_LL(x) and their kind (timelike, spacelike, null)".into())],
                          &["example", "x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3", "N_LL", "kind"], &pts);
    let _ = write_csv_str("ex_r13_orbits.csv", &[("program", src.clone()), ("content", "curves on R^(1,3)_LL for the figures: boost orbits along x_LL^1 (parameter = rapidity), rotation orbits about x_LL^3 (parameter = angle), a square grid of the (x_LL^0, x_LL^1) plane and its image under the boost with the rapidity 0.5 (parameter = position along the line), and the null points of the boosted spinor chi (parameter = rapidity)".into())],
                          &["curve", "parameter", "x_LL^0", "x_LL^1", "x_LL^2", "x_LL^3"], &orbits);
    let _ = write_csv_str("ex_r13_dimensions.csv", &[("program", src.clone()), ("content", "mass dimensions of the fields and couplings in D + 1 space-time dimensions, and whether each form is renormalizable by power counting".into())],
                          &["D", "spacetime_dimension", "dim_Psi", "dim_phi", "dim_G", "dim_g_Y", "dim_lam4", "renormalizable"], &dims_rows);
    let _ = write_csv_str("ex_r13_power_counting.csv", &[("program", src.clone()), ("content", "power counting of the Yukawa form and of the contact form for D = 3 and D = 7: every graph class with 1 to 6 vertices and at least one loop, the topological degree deg and the formula".into())],
                          &["theory", "D", "next_F", "next_B", "nvert_Y", "nvert_4", "nvert", "nloop", "nint_F", "nint_B", "deg", "deg_formula", "allowed_by_fermion_number"], &pc_rows);
    let _ = write_csv_str("ex_r13_divergent_counts.csv", &[("program", src.clone()), ("content", "the number of divergent functions (deg >= 0, at least one loop, fermion number conserved) with at most the given number of vertices".into())],
                          &["theory", "D", "max_vertices", "number_of_divergent_functions"], &counts);
    // the change of Iren from the previous cutoff (half the present one)
    for k in 0..nl {
        let ch = if k == 0 { "-".to_string() } else { fsci((iren_vals[k] - iren_vals[k - 1]).abs()) };
        bub_rows[k].push(ch);
    }
    let _ = write_csv_str("ex_r13_bubble.csv", &[("program", src.clone()), ("content", "the one-loop bubble at pE = 1, m = 0.5 versus the cutoff: Ibub(0) numerically and in closed form, the cutoff derivative of Ibub(pE) - Ibub(0), the renormalized bubble Iren, and the absolute change of Iren from the cutoff Lambda/2 to Lambda".into())],
                          &["Lambda", "Ibub_0", "Ibub_0_closed_form", "dIbub_dlnLambda", "Iren", "Iren_change_per_doubling"], &bub_rows);
    if !ck.write("ex_r13_checks.txt") {
        std::process::exit(1);
    }
}
