//! Command `malcev`: exact, exhaustive verification of the classification of Lorentz-covariant
//! extensions of the Lorentz and Poincare algebras by spinor charges.
//!
//! The script code/scripts/malcev_groebner.py derives the NECESSARY conditions (Groebner
//! bases of the Jacobi and Malcev constraint ideals); this command checks, with exact integer
//! arithmetic on ALL basis triples (Jacobi) and ALL basis 4-tuples (linearized Malcev identity),
//! that each solution family found there really satisfies the identity (sufficiency), and it
//! exhibits explicit failing 4-tuples for the excluded combinations.
//!
//! The brackets are those of the script (see its docstring); parameters are integers here:
//!   [q, b] = lam (-b q),  [x, y] = a (x ybar - y xbar),  [x, b] = beta (b^* xbar),
//!   [b, b'] = c (bbar b' - b'bar b) + k i (bbar b'^* - b'bar b^*) + Re[d (b b'bar - b' bbar)],
//!   [t, t'] = kT (t t' - t' t),  [t, b] = tau (t b),
//! on so(3,1) (q in C (x) Im H, 6), R^(3,1) (x Hermitian bi-quaternion, 4), su(2)_v (t in Im H,
//! 3) and the spinor charges (b in C (x) H, 8).
//! Data: sl_malcev_families.csv; report sl_malcev_checks.txt.

use crate::bioct::*;
use crate::checks::Checks;
use crate::exact::signature;
use spacetimes::io::write_csv_str;

type Q = Quat<GInt>;

fn g(re: i128, im: i128) -> GInt {
    GInt { re, im }
}

/// Parameters of a family member (complex parameters as Gaussian integers).
#[derive(Clone, Copy, Debug)]
pub struct Par {
    pub lam: GInt,
    pub a: GInt,
    pub beta: GInt,
    pub c: GInt,
    pub k: i128,
    pub d: GInt,
    pub kt: i128,
    pub tau: GInt,
}

impl Par {
    pub fn zero() -> Par {
        Par { lam: g(0, 0), a: g(0, 0), beta: g(0, 0), c: g(0, 0), k: 0, d: g(0, 0), kt: 0, tau: g(0, 0) }
    }
}

/// Which sectors are present: q (6), x (4), t (3), b (8).
#[derive(Clone, Copy)]
pub struct Layout {
    pub has_x: bool,
    pub has_t: bool,
}

impl Layout {
    pub fn dim(&self) -> usize {
        6 + if self.has_x { 4 } else { 0 } + if self.has_t { 3 } else { 0 } + 8
    }
    fn off_x(&self) -> usize {
        6
    }
    fn off_t(&self) -> usize {
        6 + if self.has_x { 4 } else { 0 }
    }
    fn off_b(&self) -> usize {
        self.off_t() + if self.has_t { 3 } else { 0 }
    }
}

struct El {
    q: Q,
    x: Q,
    t: Q,
    b: Q,
}

fn unpack(l: &Layout, v: &[i128]) -> El {
    let q = [g(0, 0), g(v[0], v[3]), g(v[1], v[4]), g(v[2], v[5])];
    let x = if l.has_x {
        let o = l.off_x();
        [g(v[o], 0), g(0, v[o + 1]), g(0, v[o + 2]), g(0, v[o + 3])]
    } else {
        qzero()
    };
    let t = if l.has_t {
        let o = l.off_t();
        [g(0, 0), g(v[o], 0), g(v[o + 1], 0), g(v[o + 2], 0)]
    } else {
        qzero()
    };
    let o = l.off_b();
    let b = [g(v[o], v[o + 4]), g(v[o + 1], v[o + 5]), g(v[o + 2], v[o + 6]), g(v[o + 3], v[o + 7])];
    El { q, x, t, b }
}

fn pack(l: &Layout, q: &Q, x: &Q, t: &Q, b: &Q) -> Vec<i128> {
    let mut v = vec![0i128; l.dim()];
    assert!(q[0].re == 0 && q[0].im == 0);
    for k in 0..3 {
        v[k] = q[k + 1].re;
        v[k + 3] = q[k + 1].im;
    }
    if l.has_x {
        let o = l.off_x();
        assert!(x[0].im == 0 && x[1].re == 0 && x[2].re == 0 && x[3].re == 0, "not Hermitian");
        v[o] = x[0].re;
        for k in 0..3 {
            v[o + 1 + k] = x[k + 1].im;
        }
    } else {
        assert!(x.iter().all(|z| z.re == 0 && z.im == 0));
    }
    if l.has_t {
        let o = l.off_t();
        assert!(t[0].re == 0 && t[0].im == 0 && t.iter().all(|z| z.im == 0), "not real imaginary");
        for k in 0..3 {
            v[o + k] = t[k + 1].re;
        }
    } else {
        assert!(t.iter().all(|z| z.re == 0 && z.im == 0));
    }
    let o = l.off_b();
    for k in 0..4 {
        v[o + k] = b[k].re;
        v[o + 4 + k] = b[k].im;
    }
    v
}

fn qdag(p: &Q) -> Q {
    qcc(&qconj(p))
}

/// The bracket of two elements given by coordinates.
fn bracket(l: &Layout, p: &Par, u: &[i128], w: &[i128]) -> Vec<i128> {
    let a = unpack(l, u);
    let c = unpack(l, w);
    // Lorentz
    let mut q = qsub(&qmul(&a.q, &c.q), &qmul(&c.q, &a.q));
    let mut x = qzero::<GInt>();
    let mut t = qzero::<GInt>();
    // twist: [q1, b2] - [q2, b1] with [q, b] = lam (-b q)
    let s = qadd(&qscale(&qmul(&c.b, &a.q), g(-1, 0)), &qmul(&a.b, &c.q));
    let mut b = qscale(&s, p.lam);
    // [b, b'] -> Lorentz
    let m = qsub(&qmul(&qconj(&a.b), &c.b), &qmul(&qconj(&c.b), &a.b));
    q = qadd(&q, &qscale(&m, p.c));
    if l.has_x {
        let vx = qsub(&qadd(&qmul(&a.q, &c.x), &qmul(&c.x, &qdag(&a.q))), &qadd(&qmul(&c.q, &a.x), &qmul(&a.x, &qdag(&c.q))));
        x = qadd(&x, &vx);
        let mm = qsub(&qmul(&a.x, &qconj(&c.x)), &qmul(&c.x, &qconj(&a.x)));
        q = qadd(&q, &qscale(&mm, p.a));
        let tb = qsub(&qmul(&qcc(&c.b), &qconj(&a.x)), &qmul(&qcc(&a.b), &qconj(&c.x)));
        b = qadd(&b, &qscale(&tb, p.beta));
        let h = qscale(&qsub(&qmul(&qconj(&a.b), &qcc(&c.b)), &qmul(&qconj(&c.b), &qcc(&a.b))), g(0, 1));
        x = qadd(&x, &qscale(&h, g(p.k, 0)));
    }
    if l.has_t {
        t = qadd(&t, &qscale(&qsub(&qmul(&a.t, &c.t), &qmul(&c.t, &a.t)), g(p.kt, 0)));
        let r = qsub(&qmul(&a.t, &c.b), &qmul(&c.t, &a.b));
        b = qadd(&b, &qscale(&r, p.tau));
        let wv = qsub(&qmul(&a.b, &qconj(&c.b)), &qmul(&c.b, &qconj(&a.b)));
        let dw = qscale(&wv, p.d);
        let re = [g(0, 0), g(dw[1].re, 0), g(dw[2].re, 0), g(dw[3].re, 0)];
        t = qadd(&t, &re);
    }
    pack(l, &q, &x, &t, &b)
}

/// Structure constants f[i][j] = [e_i, e_j] (vectors).
struct Algebra {
    n: usize,
    f: Vec<Vec<Vec<i128>>>,
}

impl Algebra {
    fn new(l: &Layout, p: &Par) -> Algebra {
        let n = l.dim();
        let mut f = vec![vec![vec![0i128; n]; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut u = vec![0i128; n];
                let mut w = vec![0i128; n];
                u[i] = 1;
                w[j] = 1;
                f[i][j] = bracket(l, p, &u, &w);
            }
        }
        Algebra { n, f }
    }
    fn br(&self, u: &[i128], w: &[i128]) -> Vec<i128> {
        let mut out = vec![0i128; self.n];
        for i in 0..self.n {
            if u[i] == 0 {
                continue;
            }
            for j in 0..self.n {
                if w[j] == 0 {
                    continue;
                }
                let c = u[i] * w[j];
                for (k, &v) in self.f[i][j].iter().enumerate() {
                    if v != 0 {
                        out[k] += c * v;
                    }
                }
            }
        }
        out
    }
    fn unit(&self, i: usize) -> Vec<i128> {
        let mut v = vec![0i128; self.n];
        v[i] = 1;
        v
    }
    /// Jacobiators of all basis triples: jt[i][j][k]
    fn jac_table(&self) -> Vec<Vec<Vec<Vec<i128>>>> {
        let n = self.n;
        let mut jt = vec![vec![vec![vec![0i128; n]; n]; n]; n];
        for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    let a = self.br(&self.f[i][j], &self.unit(k));
                    let b = self.br(&self.f[j][k], &self.unit(i));
                    let c = self.br(&self.f[k][i], &self.unit(j));
                    jt[i][j][k] = (0..n).map(|r| a[r] + b[r] + c[r]).collect();
                }
            }
        }
        jt
    }
    /// (Jacobi holds, a failing triple)
    fn jacobi(&self, jt: &[Vec<Vec<Vec<i128>>>]) -> (bool, Option<(usize, usize, usize)>) {
        for i in 0..self.n {
            for j in 0..self.n {
                for k in 0..self.n {
                    if jt[i][j][k].iter().any(|&x| x != 0) {
                        return (false, Some((i, j, k)));
                    }
                }
            }
        }
        (true, None)
    }
    /// linearized Malcev identity on all basis 4-tuples; returns (holds, number of failing tuples, first failing tuple)
    fn malcev(&self, jt: &[Vec<Vec<Vec<i128>>>]) -> (bool, usize, Option<(usize, usize, usize, usize)>) {
        let n = self.n;
        let mut fails = 0usize;
        let mut first = None;
        for x in 0..n {
            for y in 0..n {
                for z in 0..n {
                    for w in 0..n {
                        let mut out = vec![0i128; n];
                        // J(x,y,[w,z]) = sum_k f_wz^k J(x,y,e_k)
                        for (k, &c) in self.f[w][z].iter().enumerate() {
                            if c != 0 {
                                for r in 0..n {
                                    out[r] += c * jt[x][y][k][r];
                                }
                            }
                        }
                        for (k, &c) in self.f[x][z].iter().enumerate() {
                            if c != 0 {
                                for r in 0..n {
                                    out[r] += c * jt[w][y][k][r];
                                }
                            }
                        }
                        let a = self.br(&jt[x][y][z], &self.unit(w));
                        let b = self.br(&jt[w][y][z], &self.unit(x));
                        for r in 0..n {
                            out[r] -= a[r] + b[r];
                        }
                        if out.iter().any(|&v| v != 0) {
                            fails += 1;
                            if first.is_none() {
                                first = Some((x, y, z, w));
                            }
                        }
                    }
                }
            }
        }
        (fails == 0, fails, first)
    }
}

fn describe(l: &Layout, p: &Par) -> String {
    let mut s = Vec::new();
    let c = |z: GInt| if z.im == 0 { format!("{}", z.re) } else { format!("{}{:+}i", z.re, z.im) };
    s.push(format!("lam={}", c(p.lam)));
    if l.has_x {
        s.push(format!("a={}", c(p.a)));
        s.push(format!("beta={}", c(p.beta)));
        s.push(format!("k={}", p.k));
    }
    s.push(format!("c={}", c(p.c)));
    if l.has_t {
        s.push(format!("kT={}", p.kt));
        s.push(format!("tau={}", c(p.tau)));
        s.push(format!("d={}", c(p.d)));
    }
    s.join(" ")
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_SuperLoop, command `malcev`: exhaustive exact verification of the Lie and Malcev classification of Lorentz-covariant extensions by spinor charges");
    let la = Layout { has_x: false, has_t: false };
    let lb = Layout { has_x: true, has_t: false };
    let lc = Layout { has_x: false, has_t: true };
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut eval = |fam: &str, l: &Layout, p: &Par| -> (bool, bool, usize, String) {
        let alg = Algebra::new(l, p);
        let jt = alg.jac_table();
        let (lie, jw) = alg.jacobi(&jt);
        let (mal, nf, mw) = alg.malcev(&jt);
        let w = match (jw, mw) {
            (_, Some(t)) => format!("first failing Malcev 4-tuple (basis indices) {:?}", t),
            (Some(t), None) => format!("first failing Jacobi triple {:?}", t),
            _ => String::new(),
        };
        rows.push(vec![fam.to_string(), describe(l, p), lie.to_string(), mal.to_string(), nf.to_string()]);
        (lie, mal, nf, w)
    };

    // ---- F_A: so(3,1) + s
    let mut p = Par::zero();
    p.lam = g(-2, 0);
    p.c = g(1, 0);
    let (lie, mal, _, _) = eval("F_A", &la, &p);
    // identical to the bi-octonion commutator?
    let alg = Algebra::new(&la, &p);
    let mut same = true;
    for i in 0..14 {
        for j in 0..14 {
            let bi = crate::algebra::mbasis(i).comm(&crate::algebra::mbasis(j));
            same &= alg.f[i][j] == crate::algebra::mcoords(&bi).to_vec();
        }
    }
    ck.add("M-1", "F_A with twist lam = -2 and c = 1 is exactly the commutator algebra of the imaginary bi-octonions m (all 196 basis brackets agree); it satisfies the Malcev identity on all 38416 basis 4-tuples and violates the Jacobi identity", same && mal && !lie, String::new());
    let mut lie_all = true;
    let mut mal_all = true;
    for lam in [0i128, 1, -2] {
        let mut p = Par::zero();
        p.lam = g(lam, 0);
        let (lie, mal, _, _) = eval("F_A", &la, &p);
        mal_all &= mal;
        lie_all &= lie == (lam != -2);
    }
    ck.add("M-2", "F_A with c = 0: the twists lam = 0, 1, -2 give Malcev algebras (exhaustive); lam = 0 and lam = 1 give Lie algebras, lam = -2 does not (Elduque's non-Lie module of sl(2))", lie_all && mal_all, String::new());
    let mut excluded = true;
    let mut wit = String::new();
    for (lam, c) in [(1i128, 1i128), (0, 1), (2, 0), (-1, 0), (3, 0)] {
        let mut p = Par::zero();
        p.lam = g(lam, 0);
        p.c = g(c, 0);
        let (lie, mal, nf, w) = eval("F_A", &la, &p);
        excluded &= !mal && !lie && nf > 0;
        if lam == 1 {
            wit = w;
        }
    }
    ck.add("M-3", "excluded members of F_A fail exhaustively: (lam, c) = (1, 1) [Lie action with spinor charges closing into Lorentz], (0, 1), (2, 0), (-1, 0), (3, 0) violate both identities (in agreement with G-1, G-3, G-4)", excluded, wit);

    // ---- F_B: Poincare / (A)dS + s
    let mut p = Par::zero();
    p.lam = g(-2, 0);
    let (lie, mal, _, w) = eval("F_B", &lb, &p);
    ck.add("M-4", "F_B with twist lam = -2 and all other parameters 0 (the abelian twisted module of the Poincare algebra) satisfies the Malcev identity on all 104976 basis 4-tuples and violates the Jacobi identity: internal spinor charges, commuting with the translations, with the non-Lie Lorentz action", mal && !lie, w);
    let mut lie_sol = true;
    let mut p1 = Par::zero();
    p1.lam = g(1, 0);
    let (l1, _, _, _) = eval("F_B", &lb, &p1);
    let mut p2 = Par::zero();
    p2.lam = g(1, 0);
    p2.k = 1;
    let (l2, _, _, _) = eval("F_B", &lb, &p2);
    let mut p3 = Par::zero();
    p3.lam = g(1, 0);
    p3.a = g(1, 0);
    p3.beta = g(1, 0);
    let (l3, _, _, _) = eval("F_B", &lb, &p3);
    let mut p4 = Par::zero();
    p4.lam = g(1, 0);
    p4.a = g(2, 0);
    p4.beta = g(1, 1);
    let (l4, _, _, _) = eval("F_B", &lb, &p4);
    lie_sol &= l1 && l2 && l3 && l4;
    ck.add("M-5", "the Lie solutions of F_B found by G-6 are Lie algebras (exhaustive): Poincare with abelian spinor charges (lam = 1), Poincare with [s, s] = k P (lam = 1, k = 1: bosonic spinor charges squaring to translations), and curved space-time with a = |beta|^2 (lam = 1, a = 1, beta = 1 and a = 2, beta = 1 + i): spinor charges that do not commute with the translations", lie_sol, String::new());
    // signature of the Killing form of the space-time part for a = 1 (dS or AdS?)
    let alg = Algebra::new(&lb, &p3);
    let ad = |k: usize| -> Vec<Vec<i128>> {
        let mut m = vec![vec![0i128; 10]; 10];
        for j in 0..10 {
            for i in 0..10 {
                m[i][j] = alg.f[k][j][i];
            }
        }
        m
    };
    let mut kil = vec![vec![0i128; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            let (a, b) = (ad(i), ad(j));
            let mut tr = 0i128;
            for r in 0..10 {
                for s in 0..10 {
                    tr += a[r][s] * b[s][r];
                }
            }
            kil[i][j] = tr;
        }
    }
    let sig = signature(&kil);
    ck.add("M-6", "for a > 0 the space-time part (so(3,1) + translations with [P, P] = a M) has a Killing form of signature (4 positive, 6 negative): it is so(4,1), the de Sitter algebra; the spinor charges with [P, s] != 0 form its real 8-dimensional spinor representation", sig == (4, 6, 0), format!("signature {:?}", sig));
    let mut excl_b = true;
    let mut wb = String::new();
    for (lam, c, k, a) in [(-2i128, 1i128, 0i128, 0i128), (1, 1, 0, 0), (1, 1, 1, 0), (-2, 0, 1, 0), (-2, 0, 0, 1)] {
        let mut p = Par::zero();
        p.lam = g(lam, 0);
        p.c = g(c, 0);
        p.k = k;
        p.a = g(a, 0);
        let (lie, mal, _, w) = eval("F_B", &lb, &p);
        excl_b &= !lie && !mal;
        if lam == -2 && c == 1 {
            wb = w;
        }
    }
    ck.add("M-7", "excluded members of F_B fail exhaustively: the bi-octonion bracket together with translations (lam = -2, c = 1), the Lie action with c = 1 (with or without k = 1), and the twist together with k = 1 or with curvature a = 1: with translations present, spinor charges never close into the Lorentz algebra (G-7, G-9)", excl_b, wb);

    // ---- F_C: so(3,1) + su(2) + s
    let mut p = Par::zero();
    p.lam = g(-2, 0);
    p.c = g(1, 0);
    p.kt = 1;
    let (lie, mal, _, _) = eval("F_C", &lc, &p);
    let mut q = Par::zero();
    q.lam = g(1, 0);
    q.tau = g(1, 0);
    q.kt = 1;
    let (lie2, mal2, _, _) = eval("F_C", &lc, &q);
    ck.add("M-8", "F_C: the bi-octonion algebra m with a decoupled isospin algebra su(2) (lam = -2, c = 1, kT = 1, tau = 0, d = 0) is Malcev and not Lie; isospin acting on the spinor charges with the Lie Lorentz action and [s, s] = 0 (lam = 1, tau = 1, kT = 1) is a Lie algebra", mal && !lie && lie2 && mal2, String::new());
    let mut excl_c = true;
    let mut wc = String::new();
    for (tau, d) in [(1i128, 0i128), (0, 1), (1, 1)] {
        let mut p = Par::zero();
        p.lam = g(-2, 0);
        p.c = g(1, 0);
        p.kt = 1;
        p.tau = g(tau, 0);
        p.d = g(d, 0);
        let (_, mal, _, w) = eval("F_C", &lc, &p);
        excl_c &= !mal;
        if tau == 1 && d == 0 {
            wc = w;
        }
    }
    ck.add("M-9", "isospin cannot act on the spinor charges of m inside a Malcev algebra: adding tau = 1 (isospin acts on s), or d = 1 ([s, s] gets an isospin part), or both, to m + su(2) violates the Malcev identity (exhaustive; G-11): electric charge enters the super-loop as an outer operator (an automorphism), not as an element", excl_c, wc);

    let _ = write_csv_str("sl_malcev_families.csv", &[("content", "members of the covariant families verified exhaustively: Jacobi on all basis triples, linearized Malcev on all basis 4-tuples".to_string()), ("families", "F_A = so(3,1) + s (dim 14), F_B = so(3,1) + R^(3,1) + s (dim 18), F_C = so(3,1) + su(2) + s (dim 17)".to_string())], &["family", "parameters", "Lie", "Malcev", "failing_Malcev_4_tuples"], &rows);
    ck.write("sl_malcev_checks.txt")
}
