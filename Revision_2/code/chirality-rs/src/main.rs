//! Program `chirality` of PROJECT spacetimes, folder `Revision_2` (left-handed Dirac spinors).
//!
//! WHAT IT ANSWERS.  The author asked that every instruction, calculation, analysis,
//! document, manuscript and provenance of the folders `Revision_Deflation_CJT_and_Kohn-Sham` and
//! `Revision_Deflation_CJT_and_Kohn-Sham_definition_C` be repeated for LEFT-HANDED Dirac
//! spinors (and, in the folder `Revision_2b`, for right-handed ones).  A left-handed field
//! is a field with P^L psi = psi.  Before anything can be repeated one must know which terms
//! of the model such a field keeps.  This program answers that question in exact integer
//! arithmetic, for both spacetimes and for both chiralities, and writes the answers to
//! `data/` below the folder from which it is run.
//!
//! THE MATRICES.  They come from the library of the main project, which builds them from the
//! definitions in the author's notebooks:
//!   gamma^A, A = 0..7: real 16x16 Dirac matrices with gamma^A gamma^B + gamma^B gamma^A = 2 eta^{AB},
//!       4+4: eta = diag(+1, +1, +1, +1, -1, -1, -1, -1)   (x0..x3 space-like, x4..x7 time-like),
//!       7+1: eta = diag(-1, -1, -1, -1, +1, -1, -1, -1)   (x4 the time, all others space-like);
//!   omega = gamma^0 gamma^1 ... gamma^7;
//!   chi = omega (4+4), chi = i omega (7+1): the chirality matrix (Hermitian, chi^2 = 1);
//!   P^L = (1 - chi)/2, P^R = (1 + chi)/2: the chiral projectors;
//!   beta = -i gamma^4 (4+4), beta = gamma^4 (7+1): psibar = psi^dagger beta;
//!   alpha^j = -gamma^4 gamma^j (4+4), alpha^j = gamma^4 gamma^j (7+1), j different from 4;
//!   C^- = gamma^0 gamma^1 gamma^2 gamma^3 (4+4), C^- = gamma^4 omega (7+1):
//!       the matrix of the pair operators psi^T C^- ... psi, and on 4+4 also of the covariant
//!       adjoint psitilde = psi^dagger C^-;
//!   W = i C^- gamma^4 (4+4 only): the matrix whose eigenvalues +1 and -1 define the two
//!       halves of the 4+4 field;
//!   Theta_K = gamma^{a1} gamma^{a2} ... gamma^{ar} for a set K = {a1 < a2 < ... < ar}
//!       of r axes (Theta_{} = 1); r is the rank.  There are C(8, r) sets of rank r.
//!
//! THE TEST OF A BILINEAR.  For a field with P psi = psi, where P = P^L or P^R (Hermitian),
//! a bilinear psi^dagger X psi equals psi^dagger (P X P) psi.  It vanishes for every such field
//! exactly when P X P = 0.  The program tests (1 + s chi) X (1 + s chi) = 0, which is 4 P X P,
//! with s = -1 for the left-handed and s = +1 for the right-handed field; all entries then
//! stay Gaussian integers, and every test is exact.
//!
//! THE TEST OF A PAIR OPERATOR.  For an anticommuting field, psi^T M psi equals psi^T A psi,
//! where A is the antisymmetric part of M (the symmetric part drops out because
//! psi_a psi_b = -psi_b psi_a).  For a field with P psi = psi, psi^T M psi = psi^T (P^T M P) psi.
//! It vanishes for every such field exactly when the antisymmetric part of P^T M P is zero.
//! The program tests the antisymmetric part of (1 + s chi)^T M (1 + s chi).
//!
//! WHY THE RESULTS ARE WHAT THEY ARE (the program verifies each statement).  chi anticommutes
//! with every gamma^A.  Hence it commutes with every product of an even number of Dirac
//! matrices and anticommutes with every product of an odd number.  If X anticommutes with chi,
//! P X = X P' with P' the other projector, and P X P = X P' P = 0.  If X commutes with chi and
//! is invertible (every product of Dirac matrices is), P X P = X P is not zero.
//!
//! OUTPUT (data/):
//!   chirality_checks.txt            every check with its statement and verdict; "N of N checks passed"
//!   chirality_terms.csv             for each spacetime and chirality, each term of the model: kept or lost
//!   chirality_bilinears_by_rank.csv the bilinears psi^dagger beta Theta_K psi (form Q) and, on 4+4,
//!                                   psi^dagger C^- Theta_K psi (form C) that are kept, by rank
//!   chirality_pairs_by_rank.csv     the pair operators psi^T C^- Theta_K psi that are kept, by rank
//!
//! Run it from the folder `Revision_2`:
//!     cargo run --release --manifest-path code/chirality-rs/Cargo.toml
//! It returns the exit status 1 if any check fails.

use spacetimes::gamma::Gammas;
use spacetimes::imat::{GMat, IMat};
use std::fs;
use std::io::Write;

/// The folder this copy of the program belongs to, and the chirality that the folder studies:
/// -1 for left-handed fields (P^L psi = psi), +1 for right-handed fields (P^R psi = psi).
const FOLDER: &str = "Revision_2";
const OWN: i64 = -1;

fn name_of(s: i64) -> &'static str {
    if s < 0 {
        "left-handed"
    } else {
        "right-handed"
    }
}

fn real(m: &IMat) -> GMat {
    GMat::from_real(m)
}

fn eye() -> GMat {
    GMat::from_real(&IMat::identity(16))
}

fn transpose(m: &GMat) -> GMat {
    GMat { re: m.re.transpose(), im: m.im.transpose() }
}

fn same(a: &GMat, b: &GMat) -> bool {
    a.sub(b).is_zero()
}

/// The matrices of one spacetime.
struct Spacetime {
    /// "4+4" or "7+1".
    name: &'static str,
    gam: Vec<GMat>,
    chi: GMat,
    beta: GMat,
    /// (j, alpha^j) for the seven axes j other than the time x4.
    alpha: Vec<(usize, GMat)>,
    cminus: GMat,
    /// W = i C^- gamma^4, on 4+4 only.
    w: Option<GMat>,
}

fn spacetime(gm: &Gammas, four_four: bool) -> Spacetime {
    let g: Vec<IMat> = if four_four { gm.gamma1644.clone() } else { gm.gamma1671.clone() };
    let refs: Vec<&IMat> = g.iter().collect();
    let omega = IMat::product(&refs);
    let gam: Vec<GMat> = g.iter().map(real).collect();
    let chi = if four_four { real(&omega) } else { GMat::i_times(&omega) };
    let beta = if four_four { GMat::i_times(&g[4].neg()) } else { real(&g[4]) };
    let alpha: Vec<(usize, GMat)> = (0..8)
        .filter(|&j| j != 4)
        .map(|j| (j, if four_four { real(&g[4].mul(&g[j]).neg()) } else { real(&g[4].mul(&g[j])) }))
        .collect();
    let cminus = if four_four { real(&IMat::product(&[&g[0], &g[1], &g[2], &g[3]])) } else { real(&g[4].mul(&omega)) };
    let w = if four_four { Some(GMat::i_times(&cminus.re.mul(&g[4]))) } else { None };
    Spacetime { name: if four_four { "4+4" } else { "7+1" }, gam, chi, beta, alpha, cminus, w }
}

/// Theta_K for the set of axes given by the bits of `mask` (bit a = axis a), in increasing order.
fn theta(st: &Spacetime, mask: u32) -> GMat {
    let mut t = eye();
    for a in 0..8 {
        if mask & (1 << a) != 0 {
            t = t.mul(&st.gam[a]);
        }
    }
    t
}

fn rank_of(mask: u32) -> usize {
    mask.count_ones() as usize
}

fn binomial(n: usize, k: usize) -> usize {
    (0..k).fold(1usize, |acc, i| acc * (n - i) / (i + 1))
}

/// 1 + s chi = 2 P (P = P^L for s = -1, P^R for s = +1).
fn twice_projector(st: &Spacetime, s: i64) -> GMat {
    eye().add(&st.chi.scale(s))
}

/// True if the bilinear psi^dagger X psi survives for a field with P psi = psi.
fn bilinear_survives(st: &Spacetime, s: i64, x: &GMat) -> bool {
    let q = twice_projector(st, s);
    !q.mul(x).mul(&q).is_zero()
}

/// True if the pair operator psi^T M psi of an anticommuting field survives for P psi = psi.
fn pair_survives(st: &Spacetime, s: i64, m: &GMat) -> bool {
    let q = twice_projector(st, s);
    let a = transpose(&q).mul(m).mul(&q);
    !a.sub(&transpose(&a)).is_zero()
}

struct Report {
    lines: Vec<String>,
    passed: usize,
    total: usize,
}

impl Report {
    fn check(&mut self, id: &str, ok: bool, statement: String) {
        self.total += 1;
        if ok {
            self.passed += 1;
        }
        self.lines.push(format!("[{}] {}  {}", if ok { "PASS" } else { "FAIL" }, id, statement));
    }
}

fn main() {
    let gm = Gammas::build();
    let sts = [spacetime(&gm, true), spacetime(&gm, false)];
    let mut rep = Report { lines: Vec::new(), passed: 0, total: 0 };
    let mut terms: Vec<String> = Vec::new();
    let mut bil: Vec<String> = Vec::new();
    let mut pairs: Vec<String> = Vec::new();
    // the counts for the two chiralities, for the comparison of check X1
    let mut tables: Vec<(String, Vec<String>)> = Vec::new();

    for st in &sts {
        let tag = if st.name == "4+4" { "44" } else { "71" };
        // ---------------------------------------------------------------- the chirality matrix
        let one = eye();
        let herm = same(&st.chi, &st.chi.dagger());
        let square = same(&st.chi.mul(&st.chi), &one);
        let anti = st.gam.iter().all(|g| st.chi.mul(g).add(&g.mul(&st.chi)).is_zero());
        let trace0 = st.chi.re.trace() == 0 && st.chi.im.trace() == 0;
        rep.check(&format!("{}-K1", tag), herm && square && anti && trace0,
            format!("{}: chi is Hermitian, chi^2 = 1, chi anticommutes with each of the eight Dirac matrices, and Tr chi = 0, so that P^L and P^R each project onto eight of the sixteen components", st.name));
        let beta_anti = st.chi.mul(&st.beta).add(&st.beta.mul(&st.chi)).is_zero();
        let alpha_comm = st.alpha.iter().all(|(_, a)| same(&st.chi.mul(a), &a.mul(&st.chi)));
        let cm_comm = same(&st.chi.mul(&st.cminus), &st.cminus.mul(&st.chi));
        let cm_anti = st.chi.mul(&st.cminus).add(&st.cminus.mul(&st.chi)).is_zero();
        rep.check(&format!("{}-K2", tag), beta_anti && alpha_comm && (if st.name == "4+4" { cm_comm } else { cm_anti }),
            format!("{}: beta anticommutes with chi; the seven alpha^j commute with chi; C^- {} with chi", st.name, if st.name == "4+4" { "commutes (it is a product of four Dirac matrices)" } else { "anticommutes (it is a product of nine Dirac matrices)" }));

        for s in [OWN, -OWN] {
            let ch = name_of(s);
            let mut row = |term: &str, matrix: &str, keeps: bool, reason: &str| {
                terms.push(format!("{},{},{},{},{},{}", st.name, ch, term, matrix, if keeps { 1 } else { 0 }, reason));
            };
            let pr = eye().scale(1).add(&st.chi);
            let pl = eye().sub(&st.chi);
            // ------------------------------------------------------------ mass and Yukawa coupling
            let mass = bilinear_survives(st, s, &st.beta);
            let yuk_r = bilinear_survives(st, s, &st.beta.mul(&pr));
            let yuk_l = bilinear_survives(st, s, &st.beta.mul(&pl));
            row("mass term psibar psi", "beta", mass, "beta anticommutes with chi");
            row("Yukawa coupling psibar P^R psi", "beta P^R", yuk_r, "beta anticommutes with chi");
            row("Yukawa coupling psibar P^L psi", "beta P^L", yuk_l, "beta anticommutes with chi");
            rep.check(&format!("{}-M1-{}", tag, &ch[..1]), !mass && !yuk_r && !yuk_l,
                format!("{}, {} field: the mass term psibar psi = psi^dagger beta psi and the two bilinears psibar P^R psi, psibar P^L psi of the Yukawa coupling to the Vorton-Scalar vanish identically: the field is massless and does not couple to the Vorton-Scalar", st.name, ch));
            // ------------------------------------------------------------ kinetic terms
            let kin4 = bilinear_survives(st, s, &one);
            let kin_all = st.alpha.iter().all(|(_, a)| bilinear_survives(st, s, a));
            row("kinetic term psi^dagger i d_4 psi", "1", kin4, "1 commutes with chi");
            for (j, a) in &st.alpha {
                row(&format!("kinetic term psi^dagger i alpha^{} d_{} psi", j, j), &format!("alpha^{}", j), bilinear_survives(st, s, a), "alpha^j commutes with chi");
            }
            rep.check(&format!("{}-M2-{}", tag, &ch[..1]), kin4 && kin_all,
                format!("{}, {} field: the kinetic term of the positive Lagrangian, psi^dagger (i d_4 + i alpha^j d_j) psi, is kept, with all seven alpha^j", st.name, ch));
            if let Some(w) = &st.w {
                let cov_kin = (0..8).all(|a| !bilinear_survives(st, s, &st.cminus.mul(&st.gam[a])));
                let cov_mass = bilinear_survives(st, s, &st.cminus);
                for a in 0..8 {
                    row(&format!("covariant kinetic term psitilde gamma^{} d_{} psi", a, a), &format!("C^- gamma^{}", a), bilinear_survives(st, s, &st.cminus.mul(&st.gam[a])), "C^- gamma^A anticommutes with chi");
                }
                row("covariant bilinear psitilde psi", "C^-", cov_mass, "C^- commutes with chi");
                rep.check(&format!("{}-M3-{}", tag, &ch[..1]), cov_kin && cov_mass,
                    format!("{}, {} field: the covariant kinetic term psitilde gamma^A d_A psi (psitilde = psi^dagger C^-) vanishes identically for every A; the covariant bilinear psitilde psi is kept (it is not the mass term of the model, which is written with beta)", st.name, ch));
                // the halves W = +-1
                let q = twice_projector(st, s);
                let qo = twice_projector(st, -s);
                let maps = same(&q.mul(w), &w.mul(&qo));
                let w_anti = st.chi.mul(w).add(&w.mul(&st.chi)).is_zero();
                row("bilinear psi^dagger W psi of the two halves", "W", bilinear_survives(st, s, w), "W anticommutes with chi");
                rep.check(&format!("{}-M4-{}", tag, &ch[..1]), w_anti && maps && !bilinear_survives(st, s, w),
                    format!("{}, {} field: W = i C^- gamma^4 anticommutes with chi, so that W maps every {} spinor onto a {} one: a field of one chirality lies in neither half W = +1 nor W = -1, and psi^dagger W psi vanishes identically", st.name, ch, ch, name_of(-s)));
            }
            // ------------------------------------------------------------ bilinears by rank
            let forms: Vec<(&str, &GMat)> = if st.name == "4+4" { vec![("Q", &st.beta), ("C", &st.cminus)] } else { vec![("Q", &st.beta)] };
            let mut table: Vec<String> = Vec::new();
            for (form, left) in &forms {
                let mut counts = vec![0usize; 9];
                for mask in 0u32..256 {
                    if bilinear_survives(st, s, &left.mul(&theta(st, mask))) {
                        counts[rank_of(mask)] += 1;
                    }
                }
                for r in 0..9 {
                    bil.push(format!("{},{},{},{},{},{}", st.name, ch, form, r, binomial(8, r), counts[r]));
                }
                // expected: form Q keeps the odd ranks, form C (4+4) the even ranks, all of them
                let keep_odd = *form == "Q";
                let ok = (0..9).all(|r| counts[r] == if (r % 2 == 1) == keep_odd { binomial(8, r) } else { 0 });
                let kept: Vec<String> = (0..9).filter(|&r| counts[r] > 0).map(|r| format!("I_{}", r)).collect();
                rep.check(&format!("{}-S{}-{}", tag, form, &ch[..1]), ok,
                    format!("{}, {} field, bilinears with {} (form {}): of the C(8,r) bilinears of rank r, all are kept for {} r and none for {} r ({}); hence the invariants {} of the self-interaction can be non-zero and the other five vanish identically",
                        st.name, ch, if *form == "Q" { "psibar = psi^dagger beta" } else { "psitilde = psi^dagger C^-" }, form, if keep_odd { "odd" } else { "even" }, if keep_odd { "even" } else { "odd" },
                        (0..9).map(|r| format!("{}/{}", counts[r], binomial(8, r))).collect::<Vec<_>>().join(" "), kept.join(", ")));
                table.push(format!("{}:{:?}", form, counts));
            }
            // ------------------------------------------------------------ pair operators by rank
            let mut pc = vec![0usize; 9];
            for mask in 0u32..256 {
                if pair_survives(st, s, &st.cminus.mul(&theta(st, mask))) {
                    pc[rank_of(mask)] += 1;
                }
            }
            for r in 0..9 {
                pairs.push(format!("{},{},{},{},{}", st.name, ch, r, binomial(8, r), pc[r]));
            }
            let scalar = pair_survives(st, s, &st.cminus);
            let vectors: Vec<usize> = (0..8).filter(|&a| pair_survives(st, s, &st.cminus.mul(&st.gam[a]))).collect();
            let hidden = vectors.contains(&0);
            row("scalar pair operator psi^T C^- psi", "C^-", scalar, "see the pair test");
            for a in 0..8 {
                row(&format!("vector pair operator psi^T C^- gamma^{} psi", a), &format!("C^- gamma^{}", a), vectors.contains(&a), "see the pair test");
            }
            rep.check(&format!("{}-P1-{}", tag, &ch[..1]), !scalar && vectors.is_empty() && !hidden && pc[2] == 28,
                format!("{}, {} field: the scalar pair operator psi^T C^- psi and all eight vector pair operators psi^T C^- gamma^A psi vanish identically, among them the pair along the hidden axis x0 (A = 0), the order parameter of the singlet superfluid of the two folders; all 28 two-form pair operators psi^T C^- gamma^A gamma^B psi are kept. Kept pair operators by rank 0..8: {}",
                    st.name, ch, (0..9).map(|r| format!("{}/{}", pc[r], binomial(8, r))).collect::<Vec<_>>().join(" ")));
            table.push(format!("pairs:{:?}", pc));
            tables.push((format!("{} {}", st.name, ch), table));
        }
        // ---------------------------------------------------------------- the reflection x1 -> -x1
        let q_l = twice_projector(st, -1);
        let q_r = twice_projector(st, 1);
        let refl = same(&st.gam[1].mul(&q_l), &q_r.mul(&st.gam[1]));
        rep.check(&format!("{}-X0", tag), refl,
            format!("{}: gamma^1 P^L = P^R gamma^1, so that the matrix gamma^1, which represents the reflection x1 -> -x1 on spinors, maps every left-handed spinor onto a right-handed one", st.name));
    }
    // ---------------------------------------------------------------- left and right: the same counts
    let mut same_counts = true;
    for st in ["4+4", "7+1"] {
        let l = tables.iter().find(|t| t.0 == format!("{} left-handed", st)).unwrap();
        let r = tables.iter().find(|t| t.0 == format!("{} right-handed", st)).unwrap();
        same_counts &= l.1 == r.1;
    }
    let terms_same = {
        let strip = |c: &str| -> Vec<String> {
            terms.iter().filter(|t| t.contains(&format!(",{},", c))).map(|t| t.replace(&format!(",{},", c), ",")).collect()
        };
        strip("left-handed") == strip("right-handed")
    };
    rep.check("X1", same_counts && terms_same,
        "for both spacetimes the left-handed and the right-handed field keep exactly the same terms, the same bilinears of every rank and the same pair operators of every rank".to_string());

    // ---------------------------------------------------------------- write the files
    fs::create_dir_all("data").expect("cannot create data/");
    let head = |what: &str| -> String {
        format!("# PROJECT spacetimes, folder {}: {}\n# generated by: cargo run --release --manifest-path code/chirality-rs/Cargo.toml (program chirality)\n# exact integer arithmetic; chi = gamma^0...gamma^7 (4+4), i gamma^0...gamma^7 (7+1); P^L = (1 - chi)/2, P^R = (1 + chi)/2\n", FOLDER, what)
    };
    let mut f = fs::File::create("data/chirality_terms.csv").unwrap();
    write!(f, "{}spacetime,chirality,term,matrix,kept,reason\n{}\n", head("the terms of the model that a field of one chirality keeps (kept = 1) or loses (kept = 0)"), terms.join("\n")).unwrap();
    let mut f = fs::File::create("data/chirality_bilinears_by_rank.csv").unwrap();
    write!(f, "{}spacetime,chirality,form,rank,number,kept\n{}\n", head("the bilinears psi^dagger beta Theta_K psi (form Q) and, on 4+4, psi^dagger C^- Theta_K psi (form C) that a field of one chirality keeps, by rank r = |K|; number = C(8, r)"), bil.join("\n")).unwrap();
    let mut f = fs::File::create("data/chirality_pairs_by_rank.csv").unwrap();
    write!(f, "{}spacetime,chirality,rank,number,kept\n{}\n", head("the pair operators psi^T C^- Theta_K psi of an anticommuting field of one chirality that do not vanish identically, by rank r = |K|; number = C(8, r)"), pairs.join("\n")).unwrap();
    let mut f = fs::File::create("data/chirality_checks.txt").unwrap();
    write!(f, "{}# the chirality studied in this folder: {}\n{}\n{} of {} checks passed\n", head("which terms of the model a field of one chirality keeps: all checks"), name_of(OWN), rep.lines.join("\n"), rep.passed, rep.total).unwrap();
    for l in &rep.lines {
        println!("{}", l);
    }
    println!("chirality: {} of {} checks passed", rep.passed, rep.total);
    if rep.passed != rep.total {
        std::process::exit(1);
    }
}
