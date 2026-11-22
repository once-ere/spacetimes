//! sh_examples -- explicit, checked examples of the objects that the author named on 22 November 2026, 07:26 PST (entry
//! C27 of the task record of the folder Revision_SM_Higgs, PROJECT spacetimes): a Moufang loop, a super-loop and the
//! Lorentz super-loop, a Dirac spinor, an isovector, an isodoublet, an isospin triplet, an isodoublet of Dirac spinors,
//! the eight spinor charges, and the spin-zero superfluid of these eight spinor charges.
//!
//! Purpose.  The author asked that every manuscript give a concrete and explicit example of each of these objects.  The
//! writing of manuscripts is stopped (C27); this program computes the examples and checks every statement about them,
//! so that the rewriting (object O20 of the plan) can print numbers that a program has verified.  It is the first of three
//! independent computations: python/sh_examples.py repeats every exact example in exact rational arithmetic with its own
//! implementation of the algebra, and wolfram/source/sh_examples.wls derives them symbolically.
//!
//! Definitions (those of the author's manuscript of 16 November 2026, "The Lorentz Super-Loop of the Bi-Octonions",
//! folder Revision_SuperLoop/manuscript of the repository; restated here where they are used):
//!   * quaternion units e_0 = 1, e_1, e_2, e_3 with e_j e_j = -e_0 and e_1 e_2 = -e_2 e_1 = e_3 (and cyclically);
//!     bi-quaternions are their combinations with complex coefficients; the imaginary unit i commutes with the units;
//!   * bi-octonions x = a + b e_4 (a, b bi-quaternions) with the Cayley-Dickson product
//!         (a + b e_4)(c + d e_4) = (a c - dbar b) + (d a + b cbar) e_4 ,
//!     units e_4, e_5 = e_1 e_4, e_6 = e_2 e_4, e_7 = e_3 e_4; xbar changes the signs of the coefficients of e_1..e_7;
//!     the complex norm N(x) = sum_p xi_p(x)^2 with x xbar = N(x) e_0 and N(x y) = N(x) N(y);
//!   * a loop: a set with a product, an identity element, and unique solutions x of w x = y and of x w = y; a group is an
//!     associative loop; a Moufang loop obeys the three Moufang identities
//!         z(x(z y)) = ((z x) z) y,    ((y z) x) z = y(z(x z)),    (z x)(y z) = (z(x y)) z ;
//!   * the Lorentz super-loop L_LL: the bi-octonions of norm 1 (the manuscript also calls it "the super-loop"); it
//!     contains SL(2,C), the unit bi-quaternions; its tangent algebra at e_0 is the Malcev algebra of the imaginary
//!     bi-octonions with the commutator: the Lorentz part (imaginary bi-quaternions u; generators J^j = e_j/2 and
//!     K^j = i e_j/2) and the spinor charges b e_4 (eight real dimensions);
//!   * M_LL(b): the 2x2 matrix of a bi-quaternion, M(e_0) = sigma_0, M(e_j) = -i sigma_j (complex-linear and
//!     multiplicative; det M(b) = N(b)); the points of M31 are x = x^0 e_0 + i sum_j x^j e_j, M(x) = sum_p x^p sigma_p,
//!     and g in SL(2,C) acts by x -> g x g^dag (the Lorentz transformation L(g));
//!   * the derivations (14) D(u)(a + b e_4) = [u, a] + (-b u) e_4 (Lorentz) and (15) D^v(t)(a + b e_4) = (t b) e_4
//!     (isospin); the isospin generators T^iota = i D^v(e_iota/2), the electric charge Q_LL = T^3;
//!   * the field of the spinor charges, psi' = E^{-1} M(b)^T with E = i sigma_2 (Eq. (23)): its first column (isospin up)
//!     and second column (isospin down) are Weyl spinors; the Dirac matrices of the chiral basis
//!     gamma^0 = ((0, 1), (1, 0)), gamma^j = ((0, sigma_j), (-sigma_j, 0)), gamma^ch = i gamma^0 gamma^1 gamma^2 gamma^3
//!     = diag(-1, -1, 1, 1), C = i gamma^2 gamma^0 (module sfcore::dirac); the Dirac isodoublet Psi_LL with 8 components,
//!     component number 2 d + s (Dirac index d, isospin index s: 0 up, 1 down);
//!   * the spin-zero pair matrices K^iota = C gamma^ch (x) E tau_iota (Theorem SF1), the Hamiltonian (24), the
//!     Bogoliubov-de Gennes matrix (26), the grand potential (27), the gap equation (29) and the critical coupling (30).
//!
//! Exactness.  Every exact example is computed with integer coefficients scaled by a known denominator (for example
//! 5 g instead of g = (3 e_0 + 4 e_1)/5), so that double precision makes no rounding error (all intermediate values are
//! integers below 2^53, or halves and quarters); a value is printed as the exact fraction p/q, and the check EX-0 counts
//! the printed values that are not exact multiples of their denominator (none may be).  The superfluid needs integrals and
//! roots: those values are floating-point numbers with stated tolerances.
//!
//! The checks (report data/hck_sh_examples_checks.txt):
//!   EX-1  the octonion loop of the 16 elements +-e_0, ..., +-e_7: closed, identity, inverses, unique division, the
//!         Moufang identities on all 4096 triples, the counts of non-associative triples and non-commuting pairs;
//!   EX-2  three unit octonions with rational coefficients: norm 1, the Moufang identities, a non-zero associator;
//!   EX-3  the Lorentz super-loop: a rotation, a boost and an element off SL(2,C); their matrices, the Lorentz
//!         transformations of M31, the division, the associator, the Moufang identities, the brackets of the spinor
//!         charges, the Jacobiator, the conjugation (17) and the parity (18);
//!   EX-4  a Dirac spinor: the plane-wave spinor of momentum (17/8, 0, 0, 15/8) and mass 1, obtained by the boost of EX-3;
//!   EX-5  an isovector: the rotation R(U) of an SU(2) matrix U;
//!   EX-6  an isodoublet: U acting on (1, 0);
//!   EX-7  an isospin triplet: the symmetric states of two isodoublets, and the three symmetric pair matrices E tau_iota;
//!   EX-8  an isodoublet of Dirac spinors;
//!   EX-9  the eight spinor charges: their matrices, their fields psi', their charges, the action of the derivations, the
//!         table of their brackets;
//!   EX-10 the spin-zero superfluid: the pair matrix of the polar state and its symmetries, the critical coupling, a
//!         vacuum superfluid, the test point of the manuscript of 16 November and the spectrum of its BdG matrix.
//!
//! Build and run (from the folder rust/):
//!     cargo build --release
//!     ./target/release/sh_examples
//! Output in data/: hck_sh_examples_checks.txt, sh_examples.csv (name, value), sh_examples_loop16.csv (the product table
//! of EX-1), sh_examples_charges.csv (the eight spinor charges), sh_examples_brackets.csv (their brackets).
use sfcore::algebra::{lorentz_matrix, mll, rotation_of_su2, BiO, BiQ};
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{d8, eps2, gamma, gamma_ch, h_free, i8, k_general, k_scalar, s_spin, sigma, t_gen};
use sfcore::quad::composite;
use sfcore::report::{data_dir, write_csv, Report};
use std::cell::Cell;
use std::f64::consts::PI;

thread_local! {
    /// the number of printed values that were not exact multiples of their stated denominator (check EX-0)
    static INEXACT: Cell<usize> = Cell::new(0);
}

// ================================================================================================ exact printing

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let t = a % b;
        a = b;
        b = t;
    }
    a
}

/// the exact fraction xs/den, where xs is a value that the program computed as an integer (every exact example is
/// computed with integer coefficients scaled by a known denominator den: for g = (3 e0 + 4 e1)/5 the program computes
/// with 5 g and prints its coefficients divided by 5); a value xs that is not an integer is counted (check EX-0)
fn q(xs: f64, den: i64) -> String {
    if xs != xs.round() || xs.abs() > 9.0e15 {
        INEXACT.with(|n| n.set(n.get() + 1));
        return format!("INEXACT({:e}/{})", xs, den);
    }
    let (mut pn, mut d) = (xs as i128, den as i128);
    let g = gcd(pn, d);
    if g > 1 {
        pn /= g;
        d /= g;
    }
    if d == 1 {
        format!("{}", pn)
    } else {
        format!("{}/{}", pn, d)
    }
}

/// a complex number zs/den as "a", "b i" or "a + b i" with exact fractions (zs computed with integer parts)
fn cq(z: C64, den: i64) -> String {
    let re = q(z.re, den);
    let im = q(z.im, den);
    let imterm = |s: &str| -> String {
        match s {
            "1" => "i".into(),
            "-1" => "-i".into(),
            _ => format!("{} i", s),
        }
    };
    match (re.as_str(), im.as_str()) {
        ("0", "0") => "0".into(),
        (_, "0") => re,
        ("0", _) => imterm(&im),
        _ => {
            if let Some(r) = im.strip_prefix('-') {
                format!("{} - {}", re, imterm(r))
            } else {
                format!("{} + {}", re, imterm(&im))
            }
        }
    }
}

/// a sum of terms "coefficient name", with "+ -" folded into "-"
fn join_terms(terms: &[String]) -> String {
    if terms.is_empty() {
        return "0".into();
    }
    let mut out = terms[0].clone();
    for t in &terms[1..] {
        if let Some(r) = t.strip_prefix('-') {
            out.push_str(" - ");
            out.push_str(r);
        } else {
            out.push_str(" + ");
            out.push_str(t);
        }
    }
    out
}

/// one term "coefficient name" of a sum
fn term(z: C64, den: i64, name: &str) -> Option<String> {
    let s = cq(z, den);
    if s == "0" {
        return None;
    }
    let t = if z.im == 0.0 || z.re == 0.0 {
        match s.as_str() {
            "1" => name.to_string(),
            "-1" => format!("-{}", name),
            _ => format!("{} {}", s, name),
        }
    } else {
        format!("({}) {}", s, name)
    };
    Some(t)
}

/// a bi-octonion divided by den, written in the units e0..e7
fn bio_str(x: &BiO, den: i64) -> String {
    let cf = x.coeffs();
    let terms: Vec<String> = (0..8).filter_map(|p| term(cf[p], den, &format!("e{}", p))).collect();
    join_terms(&terms)
}

/// a complex matrix divided by den, as ((row), (row), ...)
fn mat_str(m: &CMat, den: i64) -> String {
    let rows: Vec<String> = (0..m.rows)
        .map(|i| format!("({})", (0..m.cols).map(|j| cq(m.get(i, j), den)).collect::<Vec<_>>().join(", ")))
        .collect();
    format!("({})", rows.join(", "))
}

/// a real matrix divided by den
fn rmat_str(m: &[Vec<f64>], den: i64) -> String {
    let rows: Vec<String> =
        m.iter().map(|r| format!("({})", r.iter().map(|x| q(*x, den)).collect::<Vec<_>>().join(", "))).collect();
    format!("({})", rows.join(", "))
}

/// a complex vector divided by den
fn vec_str(v: &[C64], den: i64) -> String {
    format!("({})", v.iter().map(|z| cq(*z, den)).collect::<Vec<_>>().join(", "))
}

// ================================================================================================ algebra helpers

/// the bi-octonion with the integer coefficients re[p] + i im[p] of e_p
fn bio(re: [i64; 8], im: [i64; 8]) -> BiO {
    let mut cf = [C64::ZERO; 8];
    for p in 0..8 {
        cf[p] = c(re[p] as f64, im[p] as f64);
    }
    BiO::from_coeffs(&cf)
}

/// the commutator [x, y] = x y - y x
fn comm(x: &BiO, y: &BiO) -> BiO {
    x.mul(y).sub(&y.mul(x))
}

/// the Jacobiator [[x, y], z] + [[y, z], x] + [[z, x], y]
fn jac(x: &BiO, y: &BiO, z: &BiO) -> BiO {
    comm(&comm(x, y), z).add(&comm(&comm(y, z), x)).add(&comm(&comm(z, x), y))
}

/// the largest coefficient of the differences of the two sides of the three Moufang identities
fn moufang_dev(x: &BiO, y: &BiO, z: &BiO) -> f64 {
    let m1 = z.mul(&x.mul(&z.mul(y))).sub(&z.mul(x).mul(z).mul(y)).max_abs();
    let m2 = y.mul(z).mul(x).mul(z).sub(&y.mul(&z.mul(&x.mul(z)))).max_abs();
    let m3 = z.mul(x).mul(&y.mul(z)).sub(&z.mul(&x.mul(y)).mul(z)).max_abs();
    m1.max(m2).max(m3)
}

/// the element s e_p (s = +-1) equal to x, if x is one
fn as_signed_unit(x: &BiO) -> Option<(i64, usize)> {
    let cf = x.coeffs();
    let nz: Vec<usize> = (0..8).filter(|&p| cf[p].re != 0.0 || cf[p].im != 0.0).collect();
    if nz.len() != 1 || cf[nz[0]].im != 0.0 {
        return None;
    }
    match cf[nz[0]].re {
        v if v == 1.0 => Some((1, nz[0])),
        v if v == -1.0 => Some((-1, nz[0])),
        _ => None,
    }
}

fn su_str(s: i64, p: usize) -> String {
    if s > 0 {
        format!("e{}", p)
    } else {
        format!("-e{}", p)
    }
}

/// a bi-quaternion of the Lorentz part written in the generators J^j = e_j/2 and K^j = i e_j/2:
/// xi_j e_j = 2 Re(xi_j) J^j + 2 Im(xi_j) K^j (den divides the coefficients)
fn lorentz_str(a: &BiQ, den: i64) -> String {
    let mut terms = vec![];
    for j in 1..4 {
        if let Some(t) = term(c(2.0 * a[j].re, 0.0), den, &format!("J{}", j)) {
            terms.push(t);
        }
    }
    for j in 1..4 {
        if let Some(t) = term(c(2.0 * a[j].im, 0.0), den, &format!("K{}", j)) {
            terms.push(t);
        }
    }
    join_terms(&terms)
}

/// the largest absolute entry of a complex vector
fn vmax(v: &[C64]) -> f64 {
    v.iter().fold(0.0f64, |m, z| m.max(z.abs()))
}

/// v - w
fn vsub(v: &[C64], w: &[C64]) -> Vec<C64> {
    v.iter().zip(w).map(|(a, b)| *a - *b).collect()
}

/// s v
fn vscale(v: &[C64], s: C64) -> Vec<C64> {
    v.iter().map(|a| *a * s).collect()
}

/// the 2x2 matrix psi' = E^{-1} M(b)^T of a spinor charge b e_4 (Eq. (23)); E^{-1} = -E
fn psi_prime(b: &BiQ) -> CMat {
    eps2().scale_r(-1.0).mul(&mll(b).transpose())
}

// ================================================================================================ superfluid helpers

/// I1(M) = int_0^1 q^2 dq / (q^2 + M^2)^(1/2) = ((1 + M^2)^(1/2) - M^2 arsinh(1/M)) / 2 (the cutoff is 1)
fn i1(mq: f64) -> f64 {
    0.5 * ((1.0 + mq * mq).sqrt() - mq * mq * (1.0 / mq).asinh())
}

/// J(M) = int_0^1 q^2 (q^2 + M^2)^(1/2) dq = ((2 + M^2)(1 + M^2)^(1/2) - M^4 arsinh(1/M)) / 8
fn jint(mq: f64) -> f64 {
    ((2.0 + mq * mq) * (1.0 + mq * mq).sqrt() - mq.powi(4) * (1.0 / mq).asinh()) / 8.0
}

/// the grand potential (27) of a state with the gaps gaps[0], gaps[1] (each counted with the weight 1/2 of the two
/// gaps of Theorem SF2), measured from the normal state: |d|^2/G - (1/(2 pi^2)) sum_gaps int q^2 [E+ + E- - (e + mu)
/// - |e - mu|] dq, with E+- = ((e +- mu)^2 + gap^2)^(1/2) and e = (q^2 + m^2)^(1/2); nodes and weights on [0, 1]
fn omega(d2: f64, gaps: &[f64; 2], g: f64, mu: f64, m: f64, xs: &[f64], ws: &[f64]) -> f64 {
    let mut s = 0.0;
    for (x, w) in xs.iter().zip(ws) {
        let e = (x * x + m * m).sqrt();
        for gp in gaps {
            let ep = ((e + mu).powi(2) + gp * gp).sqrt();
            let em = ((e - mu).powi(2) + gp * gp).sqrt();
            s += w * x * x * (ep + em - (e + mu) - (e - mu).abs());
        }
    }
    d2 / g - s / (2.0 * PI * PI)
}

/// the right side of the gap equation (29) of a unitary state: (1/(2 pi^2)) int q^2 (1/E+ + 1/E-) dq
fn gap_rhs(gap: f64, mu: f64, m: f64, xs: &[f64], ws: &[f64]) -> f64 {
    let mut s = 0.0;
    for (x, w) in xs.iter().zip(ws) {
        let e = (x * x + m * m).sqrt();
        let ep = ((e + mu).powi(2) + gap * gap).sqrt();
        let em = ((e - mu).powi(2) + gap * gap).sqrt();
        s += w * x * x * (1.0 / ep + 1.0 / em);
    }
    s / (2.0 * PI * PI)
}

/// the root of a decreasing function by bisection on [lo, hi] (f(lo) > 0 > f(hi)), to the width tol
fn bisect<F: Fn(f64) -> f64>(f: F, mut lo: f64, mut hi: f64, tol: f64) -> f64 {
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(mid) > 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
        if hi - lo < tol {
            break;
        }
    }
    0.5 * (lo + hi)
}

/// the minimum of a function on [lo, hi] by golden sections (the function has one minimum there)
fn golden<F: Fn(f64) -> f64>(f: F, mut a: f64, mut b: f64) -> (f64, f64) {
    let gr = 0.5 * (5f64.sqrt() - 1.0);
    let (mut x1, mut x2) = (b - gr * (b - a), a + gr * (b - a));
    let (mut f1, mut f2) = (f(x1), f(x2));
    for _ in 0..200 {
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
        if b - a < 1e-13 {
            break;
        }
    }
    let x = 0.5 * (a + b);
    (x, f(x))
}

// ================================================================================================ main

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sh_examples: explicit, checked examples of the objects named by the author on 22 November 2026 (task record C27; \
         Revision_SM_Higgs, PROJECT spacetimes)",
    );
    let mut vals: Vec<Vec<String>> = vec![];
    let mut put = |name: &str, v: String| {
        println!("  {:<28} {}", name, v);
        vals.push(vec![name.to_string(), v]);
    };
    let z8 = [0i64; 8];

    // ------------------------------------------------------------------ EX-1: the octonion loop of 16 elements
    // The 16 elements s e_p (s = +1, -1; p = 0..7) with the bi-octonion product restricted to real units.  Index of
    // s e_p in the list: p for s = +1, 8 + p for s = -1.
    let elems: Vec<(i64, usize)> = [1i64, -1].iter().flat_map(|&s| (0..8).map(move |p| (s, p))).collect();
    let el = |s: i64, p: usize| BiO::unit(p).scale(c(s as f64, 0.0));
    let idx = |s: i64, p: usize| if s > 0 { p } else { 8 + p };
    let mut prod = vec![vec![0usize; 16]; 16];
    let mut closed = true;
    let mut loop_rows = vec![];
    for (i, &(s1, p1)) in elems.iter().enumerate() {
        for (j, &(s2, p2)) in elems.iter().enumerate() {
            let z = el(s1, p1).mul(&el(s2, p2));
            match as_signed_unit(&z) {
                Some((s, p)) => {
                    prod[i][j] = idx(s, p);
                    loop_rows.push(vec![su_str(s1, p1), su_str(s2, p2), su_str(s, p)]);
                }
                None => closed = false,
            }
        }
    }
    // identity: e_0 x = x e_0 = x for every x
    let ident = (0..16).all(|i| prod[0][i] == i && prod[i][0] == i);
    // inverses: for every x exactly one y with x y = e_0, the same y gives y x = e_0, and y = xbar
    let mut inv_ok = true;
    for i in 0..16 {
        let ys: Vec<usize> = (0..16).filter(|&j| prod[i][j] == 0).collect();
        let (s, p) = elems[i];
        let bar = if p == 0 { (s, 0) } else { (-s, p) };
        if ys.len() != 1 || prod[ys[0]][i] != 0 || ys[0] != idx(bar.0, bar.1) {
            inv_ok = false;
        }
    }
    // unique division: for every w and y exactly one x with w x = y and exactly one with x w = y
    let mut div_ok = true;
    for w in 0..16 {
        for y in 0..16 {
            let left = (0..16).filter(|&x| prod[w][x] == y).count();
            let right = (0..16).filter(|&x| prod[x][w] == y).count();
            if left != 1 || right != 1 {
                div_ok = false;
            }
        }
    }
    // the Moufang identities on all 16^3 triples, the non-associative triples and the non-commuting pairs
    let (mut mouf_fail, mut nonassoc, mut noncomm) = (0usize, 0usize, 0usize);
    for x in 0..16 {
        for y in 0..16 {
            if prod[x][y] != prod[y][x] {
                noncomm += 1;
            }
            for z in 0..16 {
                let m1 = prod[z][prod[x][prod[z][y]]] == prod[prod[prod[z][x]][z]][y];
                let m2 = prod[prod[prod[y][z]][x]][z] == prod[y][prod[z][prod[x][z]]];
                let m3 = prod[prod[z][x]][prod[y][z]] == prod[prod[z][prod[x][y]]][z];
                if !(m1 && m2 && m3) {
                    mouf_fail += 1;
                }
                if prod[prod[x][y]][z] != prod[x][prod[y][z]] {
                    nonassoc += 1;
                }
            }
        }
    }
    let name = |i: usize| su_str(elems[i].0, elems[i].1);
    let e12 = prod[1][2];
    let e12_4 = prod[e12][4];
    let e1_24 = prod[1][prod[2][4]];
    put("ex1_order", "16".into());
    put("ex1_e1e2", name(e12));
    put("ex1_e2e1", name(prod[2][1]));
    put("ex1_(e1e2)e4", name(e12_4));
    put("ex1_e1(e2e4)", name(e1_24));
    put("ex1_assoc_e1_e2_e4", bio_str(&el(1, 1).mul(&el(1, 2)).mul(&el(1, 4)).sub(&el(1, 1).mul(&el(1, 2).mul(&el(1, 4)))), 1));
    put("ex1_nonassoc_triples", format!("{} of 4096", nonassoc));
    put("ex1_noncommuting_pairs", format!("{} of 256", noncomm));
    put("ex1_moufang_failures", format!("{}", mouf_fail));
    rep.add_bool(
        "EX-1a",
        "the 16 elements +-e_0, ..., +-e_7 form a loop under the bi-octonion product: every product of two of them is one \
         of them, e_0 is the identity, every element x has exactly one inverse, xbar (x xbar = xbar x = e_0), and for every \
         w and y the equations w x = y and x w = y have exactly one solution x among the 16",
        closed && ident && inv_ok && div_ok,
        &format!("closed {}, identity {}, inverses {}, unique division {}", closed, ident, inv_ok, div_ok),
    );
    rep.add_bool(
        "EX-1b",
        "this loop is a Moufang loop: the three Moufang identities hold on all 4096 triples (x, y, z) of its elements",
        mouf_fail == 0,
        &format!("{} failures", mouf_fail),
    );
    rep.add_bool(
        "EX-1c",
        "it is not a group: 1344 of the 4096 ordered triples are not associative (8 sign choices for each of the 168 \
         ordered triples of units with a non-zero associator), for example (e1 e2) e4 = e7 but e1 (e2 e4) = -e7; and 168 of \
         the 256 ordered pairs do not commute (e1 e2 = e3, e2 e1 = -e3)",
        nonassoc == 1344 && noncomm == 168 && name(e12_4) == "e7" && name(e1_24) == "-e7" && name(e12) == "e3",
        &format!("non-associative {}, non-commuting {}, (e1e2)e4 = {}, e1(e2e4) = {}", nonassoc, noncomm, name(e12_4), name(e1_24)),
    );

    // ------------------------------------------------------------------ EX-2: three unit octonions
    // x = (3 e0 + 4 e1)/5, y = (3 e2 + 4 e4)/5, z = (e0 + 2 e3 + 2 e6)/3, computed as 5x, 5y, 3z with integers.
    let x5 = bio([3, 4, 0, 0, 0, 0, 0, 0], z8);
    let y5 = bio([0, 0, 3, 0, 4, 0, 0, 0], z8);
    let z3 = bio([1, 0, 0, 2, 0, 0, 2, 0], z8);
    let nx = x5.norm().re / 25.0;
    let ny = y5.norm().re / 25.0;
    let nz = z3.norm().re / 9.0;
    let xy = x5.mul(&y5);
    let xy_z = xy.mul(&z3);
    let x_yz = x5.mul(&y5.mul(&z3));
    let as2 = xy_z.sub(&x_yz);
    let nxy = xy.norm().re / 625.0;
    let mut mdev2 = 0.0f64;
    let tri = [x5, y5, z3];
    for a in 0..3 {
        for b in 0..3 {
            for cc in 0..3 {
                mdev2 = mdev2.max(moufang_dev(&tri[a], &tri[b], &tri[cc]));
            }
        }
    }
    let xinv = x5.bar().mul(&x5);
    put("ex2_x", bio_str(&x5, 5));
    put("ex2_y", bio_str(&y5, 5));
    put("ex2_z", bio_str(&z3, 3));
    put("ex2_xy", bio_str(&xy, 25));
    put("ex2_(xy)z", bio_str(&xy_z, 75));
    put("ex2_x(yz)", bio_str(&x_yz, 75));
    put("ex2_assoc_x_y_z", bio_str(&as2, 75));
    put("ex2_xbar", bio_str(&x5.bar(), 5));
    rep.add_bool(
        "EX-2a",
        "the real octonions of norm 1 (the seven-sphere) are a Moufang loop with non-zero associators; the example: x = (3 e0 \
         + 4 e1)/5, y = (3 e2 + 4 e4)/5, z = (e0 + 2 e3 + 2 e6)/3 have norm 1, x y has norm 1, xbar x = e0, the Moufang \
         identities hold exactly for all 27 assignments of x, y, z to their three places, and (x y) z - x (y z) is not zero",
        nx == 1.0 && ny == 1.0 && nz == 1.0 && nxy == 1.0 && mdev2 == 0.0 && as2.max_abs() > 0.0
            && xinv.sub(&BiO::unit(0).scale(c(25.0, 0.0))).max_abs() == 0.0,
        &format!("norms {} {} {} {}, Moufang deviation {}, associator {}", nx, ny, nz, nxy, mdev2, bio_str(&as2, 75)),
    );

    // ------------------------------------------------------------------ EX-3: the Lorentz super-loop
    // g_rot = (3 e0 + 4 e1)/5 (a rotation), g_boost = (5 e0 + 3 i e3)/4 (a boost), s = (3 e0 + 4 e4)/5 (off SL(2,C)).
    let gr5 = bio([3, 4, 0, 0, 0, 0, 0, 0], z8);
    let gb4 = bio([5, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 3, 0, 0, 0, 0]);
    let s5 = bio([3, 0, 0, 0, 4, 0, 0, 0], z8);
    let (ngr, ngb, ns) = (gr5.norm(), gb4.norm(), s5.norm());
    let m_rot5 = mll(&gr5.a);
    let m_boost4 = mll(&gb4.a);
    let l_boost16 = lorentz_matrix(&m_boost4); // 16 L(g_boost)
    let l_rot25 = lorentz_matrix(&m_rot5); // 25 L(g_rot)
    let to_vec = |l: [[f64; 4]; 4]| -> Vec<Vec<f64>> { l.iter().map(|r| r.to_vec()).collect() };
    let w20 = gb4.mul(&s5); // 20 w, w = g_boost s
    let nw = w20.norm(); // 400 N(w)
    let wbar_w = w20.bar().mul(&w20); // 400 e0
    let y_target = BiO::unit(5);
    let x_sol = w20.bar().mul(&y_target); // 20 x, x = wbar e5
    let check_sol = w20.mul(&x_sol).sub(&y_target.scale(c(400.0, 0.0))).max_abs();
    let as3 = gr5.mul(&gb4).mul(&s5).sub(&gr5.mul(&gb4.mul(&s5))); // 100 [g_rot, g_boost, s]
    let tri3 = [gr5, gb4, s5];
    let mut mdev3 = 0.0f64;
    for a in 0..3 {
        for b in 0..3 {
            for cc in 0..3 {
                mdev3 = mdev3.max(moufang_dev(&tri3[a], &tri3[b], &tri3[cc]));
            }
        }
    }
    put("ex3_g_rot", bio_str(&gr5, 5));
    put("ex3_g_boost", bio_str(&gb4, 4));
    put("ex3_s", bio_str(&s5, 5));
    put("ex3_M_g_rot", mat_str(&m_rot5, 5));
    put("ex3_M_g_boost", mat_str(&m_boost4, 4));
    put("ex3_L_g_boost", rmat_str(&to_vec(l_boost16), 16));
    put("ex3_L_g_rot", rmat_str(&to_vec(l_rot25), 25));
    put("ex3_boost_of_rest", format!("({}, {}, {}, {})", q(l_boost16[0][0], 16), q(l_boost16[1][0], 16), q(l_boost16[2][0], 16), q(l_boost16[3][0], 16)));
    put("ex3_w", bio_str(&w20, 20));
    put("ex3_wbar", bio_str(&w20.bar(), 20));
    put("ex3_x_solving_wx_e5", bio_str(&x_sol, 20));
    put("ex3_assoc_grot_gboost_s", bio_str(&as3, 100));
    rep.add_bool(
        "EX-3a",
        "the Lorentz super-loop: g_rot = (3 e0 + 4 e1)/5, g_boost = (5/4) e0 + (3/4) i e3 and s = (3 e0 + 4 e4)/5 have the \
         complex norm 1; M(g_rot) and M(g_boost) have the determinant 1 (elements of SL(2,C)); s has a component along the \
         spinor charge e4 and is not in SL(2,C); w = g_boost s has norm 1, wbar w = e0, and x = wbar e5 solves w x = e5",
        ngr == c(25.0, 0.0) && ngb == c(16.0, 0.0) && ns == c(25.0, 0.0) && m_rot5.det2() == c(25.0, 0.0)
            && m_boost4.det2() == c(16.0, 0.0) && nw == c(400.0, 0.0) && wbar_w.sub(&BiO::unit(0).scale(c(400.0, 0.0))).max_abs() == 0.0 && check_sol == 0.0,
        &format!("N(w) = {}, w x - e5 deviation {}", cq(nw, 400), check_sol),
    );
    // the Lorentz transformations of M31: the boost maps the rest point (1, 0, 0, 0) to (17/8, 0, 0, 15/8); the rotation
    // keeps the 1-axis and turns the 2-3 plane by the angle with cosine -7/25
    let boost_ok = l_boost16[0][0] == 34.0 && l_boost16[3][0] == 30.0 && l_boost16[1][0] == 0.0 && l_boost16[2][0] == 0.0
        && l_boost16[0][3] == 30.0 && l_boost16[3][3] == 34.0 && l_boost16[1][1] == 16.0 && l_boost16[2][2] == 16.0;
    let rot_ok = l_rot25[0][0] == 25.0 && l_rot25[1][1] == 25.0 && l_rot25[2][2] == -7.0 && l_rot25[3][3] == -7.0
        && (l_rot25[2][3].abs() == 24.0) && (l_rot25[3][2] == -l_rot25[2][3]);
    rep.add_bool(
        "EX-3b",
        "the Lorentz transformations x -> g x g^dag of M31: g_boost is the boost along the 3-axis with gamma = 17/8 and \
         gamma v = 15/8 (velocity 15/17), mapping the rest point (1, 0, 0, 0) to (17/8, 0, 0, 15/8); g_rot is the rotation \
         about the 1-axis by the angle with cosine -7/25 and sine 24/25 in magnitude",
        boost_ok && rot_ok,
        &format!("L(g_boost) = {}; L(g_rot) = {}", rmat_str(&to_vec(l_boost16), 16), rmat_str(&to_vec(l_rot25), 25)),
    );
    rep.add_bool(
        "EX-3c",
        "the super-loop is not associative but Moufang: (g_rot g_boost) s - g_rot (g_boost s) is not zero, and the three \
         Moufang identities hold exactly for all 27 assignments of g_rot, g_boost, s to their places",
        as3.max_abs() > 0.0 && mdev3 == 0.0,
        &format!("associator {}; Moufang deviation {}", bio_str(&as3, 100), mdev3),
    );
    // the brackets: spinor charges close into Lorentz generators, the Lorentz algebra, the Jacobiator
    let e = |p: usize| BiO::unit(p);
    let ie = |p: usize| BiO::unit(p).scale(C64::I);
    let b45 = comm(&e(4), &e(5));
    let b4i5 = comm(&e(4), &ie(5));
    let j456 = jac(&e(4), &e(5), &e(6));
    let jk = comm(&e(1), &ie(2)); // [2 J1, 2 K2] = 4 [J1, K2]
    put("ex3_[e4,e5]", format!("{} = {}", bio_str(&b45, 1), lorentz_str(&b45.a, 1)));
    put("ex3_[e4,i e5]", format!("{} = {}", bio_str(&b4i5, 1), lorentz_str(&b4i5.a, 1)));
    put("ex3_[J1,K2]", lorentz_str(&jk.a, 4));
    put("ex3_Jac(e4,e5,e6)", bio_str(&j456, 1));
    rep.add_bool(
        "EX-3d",
        "spinor charges close into Lorentz generators: [e4, e5] = 2 e1 = 4 J1 (a rotation), [e4, i e5] = 2 i e1 = 4 K1 (a \
         boost); the Lorentz part obeys [J1, K2] = K3; and the Jacobi identity fails: Jac(e4, e5, e6) = -12 e7",
        bio_str(&b45, 1) == "2 e1" && bio_str(&b4i5, 1) == "2 i e1" && lorentz_str(&jk.a, 4) == "K3" && bio_str(&j456, 1) == "-12 e7",
        &format!("[e4,e5] = {}, [e4,i e5] = {}, [J1,K2] = {}, Jac = {}", bio_str(&b45, 1), bio_str(&b4i5, 1), lorentz_str(&jk.a, 4), bio_str(&j456, 1)),
    );
    // Eq. (17): g (b e4) gbar = (b g^2) e4 for g = g_boost, b = e0; Eq. (18): e4 a e4^{-1} = abar for a point a of M31
    let lhs17 = gb4.mul(&e(4)).mul(&gb4.bar()); // 16 x
    let g2 = BiO { a: gb4.mul(&gb4).a, b: [C64::ZERO; 4] };
    let rhs17 = BiO { a: [C64::ZERO; 4], b: g2.a }; // (e0 g^2) e4, 16 x
    let pt = bio([2, 0, 0, 0, 0, 0, 0, 0], [0, 1, 3, 5, 0, 0, 0, 0]); // the point (2, 1, 3, 5) of M31
    let par = e(4).mul(&pt).mul(&e(4).scale(c(-1.0, 0.0)));
    put("ex3_conj_g_e4_gbar", bio_str(&lhs17, 16));
    put("ex3_parity_of_(2,1,3,5)", bio_str(&par, 1));
    rep.add_bool(
        "EX-3e",
        "the twist charge -2 in the loop (Eq. (17)): g_boost e4 g_boostbar = (g_boost^2) e4, a boost of the spinor charge \
         by twice the rapidity; and the inversion element e4 (Eq. (18)): e4 x e4^{-1} = xbar, the parity, which maps the \
         point (2, 1, 3, 5) of M31 to (2, -1, -3, -5)",
        lhs17.sub(&rhs17).max_abs() == 0.0 && par.sub(&pt.bar()).max_abs() == 0.0,
        &format!("g e4 gbar = {}; e4 x e4^-1 = {}", bio_str(&lhs17, 16), bio_str(&par, 1)),
    );

    // ------------------------------------------------------------------ EX-4: a Dirac spinor
    // mass m = 1; the rest spinors u(0) = (1, 0, 1, 0) (spin up) and (0, 1, 0, 1) (spin down) in the chiral basis, boosted
    // by S(g_boost) = diag(M(g)^dag^{-1}, M(g)) = diag(1/2, 2, 2, 1/2), to the momentum p = L(g_boost) (1, 0, 0, 0).
    let mg = m_boost4.scale_r(0.25); // M(g_boost) = diag(2, 1/2): halves and twos, exact
    let sg = s_spin(&mg);
    let u0u = vec![C64::ONE, C64::ZERO, C64::ONE, C64::ZERO];
    let u0d = vec![C64::ZERO, C64::ONE, C64::ZERO, C64::ONE];
    let uu = sg.matvec(&u0u);
    let ud = sg.matvec(&u0d);
    let p = [17.0 / 8.0, 0.0, 0.0, 15.0 / 8.0];
    let mut slash = gamma(0).scale_r(p[0]);
    for j in 1..4 {
        slash = slash.sub(&gamma(j).scale_r(p[j]));
    }
    let dirac = slash.sub(&CMat::identity(4)); // gamma^mu p_mu - m with m = 1
    let res_u = vmax(&dirac.matvec(&uu));
    let res_d = vmax(&dirac.matvec(&ud));
    let pp = p[0] * p[0] - p[1] * p[1] - p[2] * p[2] - p[3] * p[3];
    let ubar_u = uu.iter().zip(gamma(0).matvec(&uu)).fold(C64::ZERO, |s, (a, b)| s + a.conj() * b);
    let udag_u = uu.iter().fold(C64::ZERO, |s, a| s + a.conj() * *a);
    // covariance: S^{-1} gamma^mu S = sum_nu L^mu_nu gamma^nu, with L = L(g_boost) (exact: halves, twos, eighths)
    let sinv = s_spin(&mg.inv2());
    let l_boost = lorentz_matrix(&mg);
    let mut cov = 0.0f64;
    for mu in 0..4 {
        let lhs = sinv.mul(&gamma(mu)).mul(&sg);
        let mut rhs = CMat::zeros(4);
        for nu in 0..4 {
            rhs = rhs.add(&gamma(nu).scale_r(l_boost[mu][nu]));
        }
        cov = cov.max(lhs.sub(&rhs).max_abs());
    }
    let chir = gamma_ch().matvec(&uu);
    put("ex4_mass", "1".into());
    put("ex4_momentum", "(17/8, 0, 0, 15/8)".into());
    put("ex4_u_rest_up", vec_str(&u0u, 1));
    put("ex4_u_up", vec_str(&vscale(&uu, c(2.0, 0.0)), 2));
    put("ex4_u_down", vec_str(&vscale(&ud, c(2.0, 0.0)), 2));
    put("ex4_ubar_u", cq(ubar_u, 1));
    put("ex4_udag_u", cq(udag_u * 4.0, 4));
    put("ex4_gamma_ch_u_up", vec_str(&vscale(&chir, c(2.0, 0.0)), 2));
    rep.add_bool(
        "EX-4a",
        "a Dirac spinor: u = (1/2, 0, 2, 0) (spin up along the 3-axis) and (0, 2, 0, 1/2) (spin down) solve the Dirac \
         equation (gamma^mu p_mu - m) u = 0 exactly for m = 1 and p = (17/8, 0, 0, 15/8), with p.p = m^2; they are the \
         boosts S(g_boost) u(0) of the rest spinors (1, 0, 1, 0) and (0, 1, 0, 1); ubar u = 2 m and u^dag u = 2 E = 17/4; the \
         plane wave u e^{-i p.x} solves the Dirac equation (i gamma^mu d_mu - m) psi = 0",
        res_u == 0.0 && res_d == 0.0 && pp == 1.0 && ubar_u == c(2.0, 0.0) && udag_u == c(17.0 / 4.0, 0.0),
        &format!("residuals {} {}, p.p = {}, ubar u = {}, u^dag u = {}", res_u, res_d, pp, cq(ubar_u, 1), cq(udag_u * 4.0, 4)),
    );
    rep.add_bool(
        "EX-4b",
        "the spinor transforms with the Lorentz transformation of its momentum: S^{-1} gamma^mu S = L^mu_nu gamma^nu for \
         S = S(g_boost) and L = L(g_boost), exactly; its chirality components: gamma^ch u = (-1/2, 0, 2, 0), the left-handed \
         part (1/2, 0) and the right-handed part (2, 0)",
        cov == 0.0 && vsub(&chir, &[c(-0.5, 0.0), C64::ZERO, c(2.0, 0.0), C64::ZERO]).iter().all(|z| z.abs() == 0.0),
        &format!("largest deviation {}", cov),
    );

    // ------------------------------------------------------------------ EX-5, EX-6: an isovector and an isodoublet
    // U = M(v) for the unit quaternion v = (3 e0 + 4 e1)/5: U = ((3/5, -4i/5), (-4i/5, 3/5)), computed as 5 U.
    let u5 = mll(&[c(3.0, 0.0), c(4.0, 0.0), C64::ZERO, C64::ZERO]);
    let r25 = rotation_of_su2(&u5); // 25 R(U)
    let r25v: Vec<Vec<f64>> = r25.iter().map(|r| r.to_vec()).collect();
    // R^T R = 1 and det R = 1 (exact with the factors 625 and 25^3)
    let mut orth = 0.0f64;
    for a in 0..3 {
        for b in 0..3 {
            let s: f64 = (0..3).map(|k| r25[k][a] * r25[k][b]).sum();
            orth = orth.max((s - if a == b { 625.0 } else { 0.0 }).abs());
        }
    }
    let det_r = r25[0][0] * (r25[1][1] * r25[2][2] - r25[1][2] * r25[2][1]) - r25[0][1] * (r25[1][0] * r25[2][2] - r25[1][2] * r25[2][0])
        + r25[0][2] * (r25[1][0] * r25[2][1] - r25[1][1] * r25[2][0]);
    // U tau_a U^dag = sum_b R_ba tau_b (exact with 25)
    let mut adj = 0.0f64;
    for a in 0..3 {
        let lhs = u5.mul(&sigma(a + 1)).mul(&u5.dagger());
        let mut rhs = CMat::zeros(2);
        for b in 0..3 {
            rhs = rhs.add(&sigma(b + 1).scale_r(r25[b][a]));
        }
        adj = adj.max(lhs.sub(&rhs).max_abs());
    }
    let v_rot: Vec<f64> = (0..3).map(|k| r25[k][2]).collect(); // 25 R (0, 0, 1)
    put("ex5_U", mat_str(&u5, 5));
    put("ex5_R_of_U", rmat_str(&r25v, 25));
    put("ex5_R_(0,0,1)", format!("({}, {}, {})", q(v_rot[0], 25), q(v_rot[1], 25), q(v_rot[2], 25)));
    rep.add_bool(
        "EX-5",
        "an isovector: three real components V^a that transform with the rotation R(U), defined by U tau_a U^dag = sum_b \
         R_ba tau_b; for U = ((3/5, -4i/5), (-4i/5, 3/5)) R is the rotation about the 1-axis with cosine -7/25, R^T R = 1, \
         det R = 1, and V = (0, 0, 1) becomes R V",
        orth == 0.0 && det_r == 15625.0 && adj == 0.0,
        &format!("R = {}, R (0,0,1) = ({}, {}, {})", rmat_str(&r25v, 25), q(v_rot[0], 25), q(v_rot[1], 25), q(v_rot[2], 25)),
    );
    let up = vec![C64::ONE, C64::ZERO];
    let down = vec![C64::ZERO, C64::ONE];
    let up_rot = u5.matvec(&up); // 5 U (1, 0)
    let t3 = sigma(3).scale_r(0.5);
    let t3_up = vmax(&vsub(&t3.matvec(&up), &vscale(&up, c(0.5, 0.0))));
    let t3_down = vmax(&vsub(&t3.matvec(&down), &vscale(&down, c(-0.5, 0.0))));
    let nvec = |v: &[C64]| -> Vec<f64> {
        (1..4).map(|a| v.iter().zip(sigma(a).matvec(v)).fold(C64::ZERO, |s, (x, y)| s + x.conj() * y).re).collect()
    };
    let n_up = nvec(&up);
    let n_rot = nvec(&up_rot); // 25 n(U psi)
    let n_dev = (0..3).map(|k| (n_rot[k] - r25[k][2]).abs()).fold(0.0f64, f64::max);
    let norm_rot = up_rot.iter().fold(0.0, |s, z| s + z.norm_sqr());
    put("ex6_psi", vec_str(&up, 1));
    put("ex6_U_psi", vec_str(&up_rot, 5));
    put("ex6_n_psi", format!("({}, {}, {})", q(n_up[0], 1), q(n_up[1], 1), q(n_up[2], 1)));
    put("ex6_n_U_psi", format!("({}, {}, {})", q(n_rot[0], 25), q(n_rot[1], 25), q(n_rot[2], 25)));
    rep.add_bool(
        "EX-6",
        "an isodoublet: two complex components that transform with U itself; psi = (1, 0) (isospin up, T3 = +1/2; (0, 1) \
         has T3 = -1/2) becomes U psi = (3/5, -4i/5), of the same length; the bilinear n^a = psi^dag tau_a psi of a doublet \
         is an isovector: n(psi) = (0, 0, 1) becomes n(U psi) = R(U) n(psi)",
        t3_up == 0.0 && t3_down == 0.0 && norm_rot == 25.0 && n_dev == 0.0,
        &format!("U psi = {}, n(U psi) = ({}, {}, {})", vec_str(&up_rot, 5), q(n_rot[0], 25), q(n_rot[1], 25), q(n_rot[2], 25)),
    );

    // ------------------------------------------------------------------ EX-7: an isospin triplet
    // two isodoublets: the basis |s1 s2> (s = 0 up, 1 down), index 2 s1 + s2; 2 T^a_tot = tau_a (x) 1 + 1 (x) tau_a
    let tt: Vec<CMat> = (1..4).map(|a| sigma(a).kron(&sigma(0)).add(&sigma(0).kron(&sigma(a)))).collect();
    let t2x4 = tt.iter().fold(CMat::zeros(4), |s, t| s.add(&t.mul(t))); // 4 T^2
    let st = |v: [f64; 4]| -> Vec<C64> { v.iter().map(|x| c(*x, 0.0)).collect() };
    let trip = [st([1.0, 0.0, 0.0, 0.0]), st([0.0, 1.0, 1.0, 0.0]), st([0.0, 0.0, 0.0, 1.0])];
    let sing = st([0.0, 1.0, -1.0, 0.0]);
    let mut tdev = 0.0f64;
    for (k, v) in trip.iter().enumerate() {
        tdev = tdev.max(vmax(&vsub(&t2x4.matvec(v), &vscale(v, c(8.0, 0.0)))));
        let t3v = tt[2].matvec(v); // 2 T3_tot v
        tdev = tdev.max(vmax(&vsub(&t3v, &vscale(v, c(2.0 - 2.0 * k as f64, 0.0)))));
    }
    let sdev = vmax(&t2x4.matvec(&sing));
    let (evs, _) = eigh(&t2x4.scale_r(0.25));
    // the pair matrices of two doublets: psi^T A chi; A = E tau_a (symmetric) and A = E (antisymmetric)
    let et: Vec<CMat> = (1..4).map(|a| eps2().mul(&sigma(a))).collect();
    let sym = et.iter().map(|m| m.sub(&m.transpose()).max_abs()).fold(0.0f64, f64::max);
    let anti = eps2().add(&eps2().transpose()).max_abs();
    let plus = et[0].sub(&et[1].scale(C64::I)); // E tau1 - i E tau2
    let minus = et[0].add(&et[1].scale(C64::I)); // E tau1 + i E tau2
    // U^T (E tau_a) U = sum_b X_ab (E tau_b): X_ab = (1/2) tr((E tau_b)^dag U^T (E tau_a) U), computed with 5 U (factor 25)
    let mut xmat = vec![vec![0.0; 3]; 3];
    let mut xdev = 0.0f64;
    for a in 0..3 {
        let lhs = u5.transpose().mul(&et[a]).mul(&u5);
        let mut rhs = CMat::zeros(2);
        for b in 0..3 {
            xmat[a][b] = 0.5 * et[b].dagger().mul(&lhs).trace().re;
            rhs = rhs.add(&et[b].scale_r(xmat[a][b]));
        }
        xdev = xdev.max(lhs.sub(&rhs).max_abs());
    }
    // U^T E = E U^{-1} for U in SU(2), so U^T (E tau_a) U = E U^dag tau_a U = sum_b R(U)_ab (E tau_b): X = R(U)
    let x_is_r = (0..3).all(|a| (0..3).all(|b| xmat[a][b] == r25[a][b]));
    put("ex7_triplet_states", "|u u>, |u d> + |d u>, |d d>  (T^2 = 2, T3 = 1, 0, -1)".into());
    put("ex7_singlet_state", "|u d> - |d u>  (T^2 = 0)".into());
    put("ex7_T2_eigenvalues", format!("{:.0}, {:.0}, {:.0}, {:.0}", evs[0], evs[1], evs[2], evs[3]));
    put("ex7_E_tau1", mat_str(&et[0], 1));
    put("ex7_E_tau2", mat_str(&et[1], 1));
    put("ex7_E_tau3", mat_str(&et[2], 1));
    put("ex7_E", mat_str(&eps2(), 1));
    put("ex7_E_tau1_minus_i_E_tau2", mat_str(&plus, 1));
    put("ex7_E_tau1_plus_i_E_tau2", mat_str(&minus, 1));
    put("ex7_X_of_U", rmat_str(&xmat, 25));
    rep.add_bool(
        "EX-7a",
        "an isospin triplet: of the four states of two isodoublets, the three symmetric ones |u u>, |u d> + |d u>, |d d> have \
         the isospin Casimir T^2 = 2 = t (t + 1) with t = 1 and T3 = 1, 0, -1; the antisymmetric |u d> - |d u> has T^2 = 0 \
         (the singlet); the eigenvalues of T^2 are 0, 2, 2, 2",
        tdev == 0.0 && sdev == 0.0 && (evs[0]).abs() < 1e-14 && (1..4).all(|k| (evs[k] - 2.0).abs() < 1e-14),
        &format!("eigenvalues {:.3e} {:.15} {:.15} {:.15}", evs[0], evs[1], evs[2], evs[3]),
    );
    rep.add_bool(
        "EX-7b",
        "the pair of two doublets psi^T A chi: the three matrices E tau_1 = ((1, 0), (0, -1)), E tau_2 = ((i, 0), (0, i)), \
         E tau_3 = ((0, -1), (-1, 0)) are symmetric (the triplet) and E is antisymmetric (the singlet); E tau_1 - i E tau_2 = \
         ((2, 0), (0, 0)) joins two up components (T3 = +1), E tau_1 + i E tau_2 = ((0, 0), (0, -2)) two down ones (T3 = \
         -1), E tau_3 an up with a down one (T3 = 0); under U, U^T (E tau_a) U = sum_b R(U)_ab (E tau_b), so the three pair \
         amplitudes psi^T (E tau_a) chi are multiplied by R(U), like the components of an isovector",
        sym == 0.0 && anti == 0.0 && xdev == 0.0 && x_is_r
            && plus.sub(&CMat::m2(c(2.0, 0.0), C64::ZERO, C64::ZERO, C64::ZERO)).max_abs() == 0.0
            && minus.sub(&CMat::m2(C64::ZERO, C64::ZERO, C64::ZERO, c(-2.0, 0.0))).max_abs() == 0.0,
        &format!("X = {} (= R(U))", rmat_str(&xmat, 25)),
    );

    // ------------------------------------------------------------------ EX-8: an isodoublet of Dirac spinors
    // Psi = u (x) chi with the spinor u of EX-4 and the isospin vector chi; component 2 d + s
    let kron_v = |a: &[C64], b: &[C64]| -> Vec<C64> { a.iter().flat_map(|x| b.iter().map(move |y| *x * *y)).collect() };
    let psi_up = kron_v(&uu, &up);
    let psi_dn = kron_v(&uu, &down);
    let dirac8 = d8(&dirac);
    let r_up = vmax(&dirac8.matvec(&psi_up));
    let r_dn = vmax(&dirac8.matvec(&psi_dn));
    let t3_8 = t_gen(3);
    let q_up = vmax(&vsub(&t3_8.matvec(&psi_up), &vscale(&psi_up, c(0.5, 0.0))));
    let q_dn = vmax(&vsub(&t3_8.matvec(&psi_dn), &vscale(&psi_dn, c(-0.5, 0.0))));
    let psi_rot = i8(&u5).matvec(&psi_up); // 5 (1 (x) U) Psi
    let r_rot = vmax(&dirac8.matvec(&psi_rot));
    let commut = d8(&sg).mul(&i8(&u5)).sub(&i8(&u5).mul(&d8(&sg))).max_abs();
    put("ex8_Psi_up", vec_str(&vscale(&psi_up, c(2.0, 0.0)), 2));
    put("ex8_Psi_down", vec_str(&vscale(&psi_dn, c(2.0, 0.0)), 2));
    put("ex8_(1xU)Psi_up", vec_str(&vscale(&psi_rot, c(2.0, 0.0)), 10));
    rep.add_bool(
        "EX-8",
        "an isodoublet of Dirac spinors (8 components, Dirac index d and isospin index s, component 2 d + s): Psi = u (x) (1, 0) \
         = (1/2, 0, 0, 0, 2, 0, 0, 0) and u (x) (0, 1) solve the Dirac equation ((gamma.p - m) (x) 1) Psi = 0; T3 = 1 (x) \
         tau_3/2 gives +1/2 and -1/2, the electric charges Q_LL = T3 of the spinor charges; the isospin rotation (1 (x) U) \
         Psi = u (x) (3/5, -4i/5) solves the same equation, and Lorentz transformations commute with isospin rotations: \
         [S(g_boost) (x) 1, 1 (x) U] = 0",
        r_up == 0.0 && r_dn == 0.0 && q_up == 0.0 && q_dn == 0.0 && r_rot == 0.0 && commut == 0.0,
        &format!("residuals {} {} {}, charges {} {}, commutator {}", r_up, r_dn, r_rot, q_up, q_dn, commut),
    );

    // ------------------------------------------------------------------ EX-9: the eight spinor charges
    // basis: s_k = b_k e4 with b_k = e0, e1, e2, e3, i e0, i e1, i e2, i e3 (the units e4, e5, e6, e7 and i e4, ..., i e7)
    let names = ["e4", "e5", "e6", "e7", "i e4", "i e5", "i e6", "i e7"];
    let bk: Vec<BiQ> = (0..8)
        .map(|k| {
            let mut b = [C64::ZERO; 4];
            b[k % 4] = if k < 4 { C64::ONE } else { C64::I };
            b
        })
        .collect();
    let sk: Vec<BiO> = bk.iter().map(|b| BiO { a: [C64::ZERO; 4], b: *b }).collect();
    let mut charge_rows = vec![];
    for k in 0..8 {
        let m = mll(&bk[k]);
        let pp_ = psi_prime(&bk[k]);
        let col = |cc: usize| vec_str(&[pp_.get(0, cc), pp_.get(1, cc)], 1);
        charge_rows.push(vec![names[k].to_string(), mat_str(&m, 1), mat_str(&pp_, 1), col(0), col(1)]);
    }
    // the Lorentz derivation D(u) b e4 = (-b u) e4 acts on psi' as D(J^j) = -(i/2) sigma_j and D(K^j) = (1/2) sigma_j from
    // the left (on each column); the isospin generator T^iota = i D^v(e_iota/2), b e4 -> i (e_iota b / 2) e4, acts on the
    // column index as tau_iota/2: psi' -> psi' tau_iota^T / 2 (both checked with the factor 2 on all 8 charges)
    let mut der = 0.0f64;
    for k in 0..8 {
        let pk = psi_prime(&bk[k]);
        for j in 1..4 {
            let mut ej = [C64::ZERO; 4];
            ej[j] = C64::ONE;
            let mut iej = [C64::ZERO; 4];
            iej[j] = C64::I;
            // 2 D(J^j): b -> -b e_j ; 2 D(K^j): b -> -b (i e_j)
            let bj = sfcore::algebra::q_scale(&sfcore::algebra::q_mul(&bk[k], &ej), c(-1.0, 0.0));
            let bkk = sfcore::algebra::q_scale(&sfcore::algebra::q_mul(&bk[k], &iej), c(-1.0, 0.0));
            der = der.max(psi_prime(&bj).sub(&sigma(j).mul(&pk).scale(c(0.0, -1.0))).max_abs());
            der = der.max(psi_prime(&bkk).sub(&sigma(j).mul(&pk)).max_abs());
            // 2 T^j: b -> i e_j b
            let bt = sfcore::algebra::q_scale(&sfcore::algebra::q_mul(&ej, &bk[k]), C64::I);
            der = der.max(psi_prime(&bt).sub(&pk.mul(&sigma(j).transpose())).max_abs());
        }
    }
    // the electric charge Q_LL = T^3 as a complex 4x4 matrix on the coefficients of e4..e7 (complex-linear): eigenvalues
    // +1/2, +1/2, -1/2, -1/2; the isospin Casimir sum_iota (T^iota)^2 = 3/4 (the doublet)
    let tmat = |iota: usize| -> CMat {
        let mut m = CMat::zeros(4);
        let mut t = [C64::ZERO; 4];
        t[iota] = C64::ONE;
        for col in 0..4 {
            let mut b = [C64::ZERO; 4];
            b[col] = C64::ONE;
            let r = sfcore::algebra::q_scale(&sfcore::algebra::q_mul(&t, &b), c(0.0, 0.5));
            for row in 0..4 {
                m.set(row, col, r[row]);
            }
        }
        m
    };
    let tq: Vec<CMat> = (1..4).map(tmat).collect();
    let (qev, _) = eigh(&tq[2]);
    let cas = tq.iter().fold(CMat::zeros(4), |s, t| s.add(&t.mul(t)));
    let cas_dev = cas.sub(&CMat::identity(4).scale_r(0.75)).max_abs();
    let herm = tq.iter().map(|t| t.sub(&t.dagger()).max_abs()).fold(0.0f64, f64::max);
    // the bracket table and the span of the brackets (rank 6 = all of the Lorentz part)
    let mut br_rows = vec![];
    let mut in_l = true;
    let mut gram = CMat::zeros(6);
    for k in 0..8 {
        for l in 0..8 {
            let b = comm(&sk[k], &sk[l]);
            if b.b.iter().any(|z| z.abs() != 0.0) || b.a[0].abs() != 0.0 {
                in_l = false;
            }
            br_rows.push(vec![names[k].to_string(), names[l].to_string(), bio_str(&b, 1), lorentz_str(&b.a, 1)]);
            let v = [b.a[1].re, b.a[2].re, b.a[3].re, b.a[1].im, b.a[2].im, b.a[3].im];
            for r in 0..6 {
                for cc in 0..6 {
                    gram.set(r, cc, gram.get(r, cc) + c(v[r] * v[cc], 0.0));
                }
            }
        }
    }
    let (gev, _) = eigh(&gram);
    let rank = gev.iter().filter(|x| **x > 1e-9 * gev[5].abs().max(1.0)).count();
    put("ex9_psi_prime_e4", mat_str(&psi_prime(&bk[0]), 1));
    put("ex9_Q_eigenvalues", format!("{:.1}, {:.1}, {:.1}, {:.1}", qev[0], qev[1], qev[2], qev[3]));
    put("ex9_bracket_span_rank", format!("{}", rank));
    rep.add_bool(
        "EX-9a",
        "the eight spinor charges e4, e5, e6, e7, i e4, i e5, i e6, i e7 (b e4 with b = e0, e1, e2, e3, i e0, ...): the field \
         psi' = E^{-1} M(b)^T of each is a 2x2 matrix whose first column (isospin up) and second column (isospin down) are \
         Weyl spinors: on all eight, the Lorentz derivation of J^j acts on each column as -(i/2) sigma_j and that of K^j as \
         +(1/2) sigma_j, and the isospin generator T^iota acts on the column index as tau_iota/2",
        der == 0.0,
        &format!("largest deviation {}", der),
    );
    rep.add_bool(
        "EX-9b",
        "their charges: the electric charge Q_LL = T^3 is Hermitian with the eigenvalues +1/2, +1/2, -1/2, -1/2 on the \
         complex span of e4..e7, and the isospin Casimir is 3/4 = (1/2)(1/2 + 1): the spinor charges are an isodoublet \
         with the electric charges +-1/2",
        herm == 0.0 && cas_dev == 0.0 && (qev[0] + 0.5).abs() < 1e-15 && (qev[1] + 0.5).abs() < 1e-15 && (qev[2] - 0.5).abs() < 1e-15
            && (qev[3] - 0.5).abs() < 1e-15,
        &format!("eigenvalues {:.3} {:.3} {:.3} {:.3}, Casimir deviation {}", qev[0], qev[1], qev[2], qev[3], cas_dev),
    );
    rep.add_bool(
        "EX-9c",
        "the brackets of the eight spinor charges (the 64 entries of data/sh_examples_brackets.csv) are all imaginary \
         bi-quaternions, i.e. Lorentz generators, and they span the whole Lorentz part (rank 6)",
        in_l && rank == 6,
        &format!("in the Lorentz part {}, rank {}", in_l, rank),
    );

    // ------------------------------------------------------------------ EX-10: the spin-zero superfluid
    // (a) the pair matrix of the polar state, K^3 = C gamma^ch (x) E tau_3, and its symmetries
    let k3 = k_scalar(3);
    let ks: Vec<CMat> = (1..4).map(k_scalar).collect();
    let k_anti = k3.add(&k3.transpose()).max_abs();
    let s8b = d8(&sg); // S(g_boost) on the 8 components: halves and twos
    let lor_b = s8b.transpose().mul(&k3).mul(&s8b).sub(&k3).max_abs();
    let m_rot = m_rot5.clone(); // 5 M(g_rot); M(g_rot) is unitary, so S(g_rot) = diag(M, M)
    let s8r5 = d8(&CMat::block_diag(&m_rot, &m_rot)); // 5 S(g_rot)
    let lor_r = s8r5.transpose().mul(&k3).mul(&s8r5).sub(&k3.scale_r(25.0)).max_abs();
    // isospin: (1 (x) U)^T K^a (1 (x) U) = sum_b X_ab K^b with the matrix X = R(U) of EX-7
    let mut iso = 0.0f64;
    for a in 0..3 {
        let lhs = i8(&u5).transpose().mul(&ks[a]).mul(&i8(&u5));
        let mut rhs = CMat::zeros(8);
        for b in 0..3 {
            rhs = rhs.add(&ks[b].scale_r(xmat[a][b]));
        }
        iso = iso.max(lhs.sub(&rhs).max_abs());
    }
    // charges of the pairs: T3^T K + K T3 = q K
    let chg = |k: &CMat, qv: f64| t3_8.transpose().mul(k).add(&k.mul(&t3_8)).sub(&k.scale_r(qv)).max_abs();
    let kplus = ks[0].sub(&ks[1].scale(C64::I));
    let kminus = ks[0].add(&ks[1].scale(C64::I));
    let ch = chg(&k3, 0.0).max(chg(&kplus, 1.0)).max(chg(&kminus, -1.0));
    // the isosinglet C gamma^ch (x) E is symmetric (no pair of one field); with a species index (x) E it is antisymmetric
    let ksing = k_general(&gamma_ch(), &sigma(0));
    let sing_sym = ksing.sub(&ksing.transpose()).max_abs();
    let ksing2 = ksing.kron(&eps2());
    let sing2_anti = ksing2.add(&ksing2.transpose()).max_abs();
    put("ex10_K3_nonzero_entries", {
        let mut s = vec![];
        for i in 0..8 {
            for j in 0..8 {
                let z = k3.get(i, j);
                if z.abs() != 0.0 {
                    s.push(format!("({},{}) {}", i, j, cq(z, 1)));
                }
            }
        }
        s.join("; ")
    });
    rep.add_bool(
        "EX-10a",
        "the pair of the polar state, K^3 = C gamma^ch (x) E tau_3 (data/sh_examples.csv lists its 8 non-zero entries): it is \
         antisymmetric; it is Lorentz invariant, S^T K^3 S = K^3 for the boost S(g_boost) and the rotation S(g_rot) (spin \
         0); under isospin the three K^iota transform as an isovector (an isospin triplet); and T3^T K + K T3 = q K with \
         q = 0 for K^3 (it joins an up and a down charge) and q = +1, -1 for K^1 -+ i K^2 (two up, two down)",
        k_anti == 0.0 && lor_b == 0.0 && lor_r == 0.0 && iso == 0.0 && ch == 0.0,
        &format!("antisymmetry {}, Lorentz {} {}, isospin {}, charges {}", k_anti, lor_b, lor_r, iso, ch),
    );
    rep.add_bool(
        "EX-10b",
        "no isosinglet pair of one field: C gamma^ch (x) E is symmetric, so Psi^T K Psi = 0 for anticommuting components; with \
         two species, C gamma^ch (x) E (x) E (the last factor on the species index) is antisymmetric (the super-loop singlet \
         of Theorem SF4)",
        sing_sym == 0.0 && sing2_anti == 0.0,
        &format!("symmetry {}, antisymmetry with two species {}", sing_sym, sing2_anti),
    );
    // (b) the critical coupling (30) and a superfluid of the vacuum (mu = 0) at G = 30 > G^crit, m = 0.2
    let m_ll = 0.2;
    let gcrit = PI * PI / i1(m_ll);
    let gcrit0 = 2.0 * PI * PI;
    let g_vac = 30.0;
    let mq = bisect(|mm| i1(mm) / (PI * PI) - 1.0 / g_vac, m_ll, 5.0, 1e-15);
    let gap_vac = (mq * mq - m_ll * m_ll).sqrt();
    let om_vac = |dd: f64| -> f64 {
        let mm = (m_ll * m_ll + dd * dd).sqrt();
        dd * dd / g_vac - 2.0 / (PI * PI) * (jint(mm) - jint(m_ll))
    };
    let (dmin, omin_vac) = golden(om_vac, 0.01, 2.0);
    put("ex10_Gcrit_m0.2", format!("{:.6}", gcrit));
    put("ex10_Gcrit_m0", format!("{:.6}", gcrit0));
    put("ex10_vacuum_G30_gap", format!("{:.10}", gap_vac));
    put("ex10_vacuum_G30_Omega_min", format!("{:.10e}", omin_vac));
    rep.add(
        "EX-10c",
        "the critical coupling (30) of the vacuum: 21.286610 for m = 0.2 and 2 pi^2 = 19.739209 for m = 0 (the values printed \
         in the manuscript of 16 November; the largest difference)",
        (gcrit - 21.286610).abs().max((gcrit0 - 19.739209).abs()),
        5.0e-7,
    );
    rep.add(
        "EX-10d",
        "a superfluid of the vacuum (mu = 0, m = 0.2, cutoff 1) at G = 30 > G^crit: the gap from the gap equation (29) in \
         closed form agrees with the minimum of the grand potential (27) in closed form (relative difference of the two gaps)",
        (dmin - gap_vac).abs() / gap_vac,
        1e-6,
    );
    // (c) the test point of the manuscript: G = 12, mu = 0.5, m = 0.2 (a Fermi surface at q_F = (mu^2 - m^2)^(1/2))
    let (g_t, mu_t) = (12.0, 0.5);
    let qf = (mu_t * mu_t - m_ll * m_ll).sqrt();
    let (xs, ws) = composite(0.0, 1.0, &[qf], 40, 24);
    let gap_t = bisect(|dd| gap_rhs(dd, mu_t, m_ll, &xs, &ws) - 1.0 / g_t, 1e-4, 1.0, 1e-14);
    let (d_u, om_u) = golden(|dd| omega(dd * dd, &[dd, dd], g_t, mu_t, m_ll, &xs, &ws), 1e-3, 0.5);
    let (d_n, om_n) = golden(|dd| omega(dd * dd, &[2f64.sqrt() * dd, 0.0], g_t, mu_t, m_ll, &xs, &ws), 1e-3, 0.5);
    put("ex10_test_gap", format!("{:.10}", gap_t));
    put("ex10_test_Omega_unitary", format!("{:.10e}", om_u));
    put("ex10_test_Omega_zeta1", format!("{:.10e}", om_n));
    put("ex10_test_d_zeta1", format!("{:.10}", d_n));
    put("ex10_test_qF", format!("{:.10}", qf));
    rep.add(
        "EX-10e",
        "the test point of the manuscript of 16 November (G = 12, mu = 0.5, m = 0.2): the gap 0.090417 and the minimum of the \
         grand potential -9.720605e-5 of the unitary polar state, and -4.860303e-5 for the non-unitary state zeta = 1, \
         recomputed by the gap equation (29) and the minimum of (27) (largest difference from the printed values, in units \
         of their last printed digit)",
        ((gap_t - 0.090417).abs() / 1e-6).max((om_u + 9.720605e-5).abs() / 1e-11).max((om_n + 4.860303e-5).abs() / 1e-11),
        0.5,
    );
    rep.add(
        "EX-10f",
        "at the test point the gap equation and the minimum of the grand potential give the same gap (relative difference), and \
         the non-unitary state has half the condensation energy (|Omega(zeta = 1)/Omega(unitary) - 1/2|)",
        ((d_u - gap_t).abs() / gap_t).max((om_n / om_u - 0.5).abs()),
        1e-6,
    );
    // (d) the Bogoliubov-de Gennes matrix (26) of the polar state at q = (0, 0, 0.3): its eight positive eigenvalues are
    // E+ and E- of Theorem SF2, four times each
    let qv = [0.0, 0.0, 0.3];
    let qm = [0.0, 0.0, -0.3];
    let hq = h_free(&qv, m_ll).sub(&CMat::identity(8).scale_r(mu_t));
    let hmq = h_free(&qm, m_ll).sub(&CMat::identity(8).scale_r(mu_t));
    let pmat = k3.dagger().scale_r(gap_t);
    let bdg = CMat::blocks(&hq, &pmat, &pmat.dagger(), &hmq.transpose().scale_r(-1.0));
    let (bev, _) = eigh(&bdg);
    let eps_q = (0.09f64 + m_ll * m_ll).sqrt();
    let e_plus = ((eps_q + mu_t).powi(2) + gap_t * gap_t).sqrt();
    let e_minus = ((eps_q - mu_t).powi(2) + gap_t * gap_t).sqrt();
    let mut want: Vec<f64> = vec![e_minus; 4];
    want.extend(vec![e_plus; 4]);
    want.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let pos: Vec<f64> = bev.iter().cloned().filter(|x| *x > 0.0).collect();
    let bdg_dev = if pos.len() == 8 { pos.iter().zip(&want).map(|(a, b)| (a - b).abs()).fold(0.0f64, f64::max) } else { 1.0 };
    put("ex10_test_E_minus_q0.3", format!("{:.10}", e_minus));
    put("ex10_test_E_plus_q0.3", format!("{:.10}", e_plus));
    rep.add(
        "EX-10g",
        "the spectrum of the polar state at the test point and the momentum (0, 0, 0.3): the 16 eigenvalues of the BdG \
         matrix (26) with P = gap (K^3)^dag are +-E- and +-E+ of Theorem SF2, four times each (largest difference)",
        bdg_dev,
        1e-13,
    );

    // ------------------------------------------------------------------ EX-0 and the output
    let inexact = INEXACT.with(|n| n.get());
    rep.add_bool(
        "EX-0",
        "every value printed as a fraction is an exact multiple of its stated denominator (the exact examples are computed \
         with integers scaled by known denominators)",
        inexact == 0,
        &format!("{} inexact values", inexact),
    );
    write_csv(&data.join("sh_examples.csv"), &["name", "value"], &vals);
    write_csv(&data.join("sh_examples_loop16.csv"), &["x", "y", "x_times_y"], &loop_rows);
    write_csv(&data.join("sh_examples_charges.csv"), &["charge", "M_LL_of_b", "psi_prime", "isospin_up_column", "isospin_down_column"], &charge_rows);
    write_csv(&data.join("sh_examples_brackets.csv"), &["s_k", "s_l", "bracket", "in_J_and_K"], &br_rows);
    rep.write(&data.join("hck_sh_examples_checks.txt"));
    let (np, nt) = rep.counts();
    println!("sh_examples: {} of {} checks pass", np, nt);
}
