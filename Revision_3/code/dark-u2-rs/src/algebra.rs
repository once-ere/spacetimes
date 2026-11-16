//! Command `algebra`: exact proofs of the algebraic facts on which the model rests.
//!
//! Every matrix here has integer (or Gaussian-integer) entries, and every check is
//! an identity between such matrices, computed in exact i64 arithmetic: a check that
//! passes is a proof.  Dimensions of solution spaces are bounded by ranks computed
//! modulo the prime 2^61-1 (`spacetimes::imat::rank`), which are lower bounds of the
//! ranks over the rationals; every such bound is completed by exhibiting the same
//! number of independent solutions, so the dimension is proved exactly.
//!
//! Notation (all on the 7+1 spacetime; x4 is the time):
//!   g[A]     = gamma^A_1671, the real 16x16 Dirac matrices, {g[A], g[C]} = 2 eta1671^{AC};
//!   omega    = g[0] g[1] ... g[7];
//!   beta     = g[4];  alpha^j = g[4] g[j] for the seven space indices j;
//!   cp = C^+ = g[4];  cm = C^- = g[4] omega   (the two charge-conjugation matrices);
//!   G_K      = g[A1] g[A2] ... g[Ar] with A1 < ... < Ar, the ordered product over an index set K;
//!   E        = the real antisymmetric 2x2 matrix [[0,1],[-1,0]] (= i tau^2) acting on
//!              the doublet index of Psi_U2 = (psi^1_U2, psi^2_U2).

use crate::checks::Checks;
use spacetimes::gamma::{clifford_basis, rank_of, Gammas, ETA1671};
use spacetimes::imat::{rank, GMat, IMat};

/// The seven space-like indices of the 7+1 spacetime (x4 is the time).
pub const SPACE: [usize; 7] = [0, 1, 2, 3, 5, 6, 7];

/// The matrices of the 7+1 spacetime used by the model.
pub struct Mats {
    pub g: Vec<IMat>,
    pub omega: IMat,
    pub beta: IMat,
    pub cp: IMat,
    pub cm: IMat,
    /// alpha^j = g[4] g[j] in the order of SPACE.
    pub alpha: Vec<IMat>,
}

/// Build the matrices from the Rust program of the main project.
pub fn mats() -> Mats {
    let gs = Gammas::build();
    let g = gs.gamma1671.clone();
    let refs: Vec<&IMat> = g.iter().collect();
    let omega = IMat::product(&refs);
    let beta = g[4].clone();
    let cp = beta.clone();
    let cm = beta.mul(&omega);
    let alpha = SPACE.iter().map(|&j| beta.mul(&g[j])).collect();
    Mats { g, omega, beta, cp, cm, alpha }
}

/// Kronecker product of two integer matrices.
pub fn kron(a: &IMat, b: &IMat) -> IMat {
    let mut r = IMat::zeros(a.n * b.n, a.m * b.m);
    for i in 0..a.n {
        for j in 0..a.m {
            let x = a.get(i, j);
            if x == 0 {
                continue;
            }
            for k in 0..b.n {
                for l in 0..b.m {
                    r.set(i * b.n + k, j * b.m + l, x * b.get(k, l));
                }
            }
        }
    }
    r
}

/// Kronecker product of two Gaussian-integer matrices.
fn gkron(a: &GMat, b: &GMat) -> GMat {
    GMat {
        re: kron(&a.re, &b.re).sub(&kron(&a.im, &b.im)),
        im: kron(&a.re, &b.im).add(&kron(&a.im, &b.re)),
    }
}

/// Transpose (no complex conjugation) of a Gaussian-integer matrix.
fn gt(m: &GMat) -> GMat {
    GMat { re: m.re.transpose(), im: m.im.transpose() }
}

/// i times a Gaussian-integer matrix.
fn gi(m: &GMat) -> GMat {
    GMat { re: m.im.neg(), im: m.re.clone() }
}

fn gtrace(m: &GMat) -> (i64, i64) {
    (m.re.trace(), m.im.trace())
}

/// The 2x2 matrices of the Kronecker factors and of the doublet index.
pub fn two_by_two() -> (IMat, IMat, IMat, IMat) {
    let one = IMat::identity(2);
    let x = IMat::from_rows(&[vec![0, 1], vec![1, 0]]);
    let z = IMat::from_rows(&[vec![1, 0], vec![0, -1]]);
    let e = IMat::from_rows(&[vec![0, 1], vec![-1, 0]]);
    (one, x, z, e)
}

/// The Pauli matrices tau^1, tau^2, tau^3 as Gaussian-integer matrices.
pub fn pauli() -> Vec<GMat> {
    let z2 = IMat::zeros(2, 2);
    vec![
        GMat { re: IMat::from_rows(&[vec![0, 1], vec![1, 0]]), im: z2.clone() },
        GMat { re: z2.clone(), im: IMat::from_rows(&[vec![0, -1], vec![1, 0]]) },
        GMat { re: IMat::from_rows(&[vec![1, 0], vec![0, -1]]), im: z2 },
    ]
}

/// Rows of the linear system "K is an invariant bilinear form": S^T K + K S = 0 for every S in `gens`.
fn invariant_form_rows(gens: &[IMat]) -> Vec<Vec<i64>> {
    let n = gens[0].n;
    let mut rows = Vec::new();
    for s in gens {
        for i in 0..n {
            for j in 0..n {
                let mut r = vec![0i64; n * n];
                for k in 0..n {
                    r[k * n + j] += s.get(k, i); // (S^T K)_ij = sum_k S_ki K_kj
                    r[i * n + k] += s.get(k, j); // (K S)_ij   = sum_k K_ik S_kj
                }
                rows.push(r);
            }
        }
    }
    rows
}

/// Rows of the linear system "X commutes with every matrix in `gens`": X R - R X = 0.
fn commutant_rows(gens: &[IMat]) -> Vec<Vec<i64>> {
    let n = gens[0].n;
    let mut rows = Vec::new();
    for rm in gens {
        for i in 0..n {
            for j in 0..n {
                let mut r = vec![0i64; n * n];
                for k in 0..n {
                    r[i * n + k] += rm.get(k, j); // (X R)_ij = sum_k X_ik R_kj
                    r[k * n + j] -= rm.get(i, k); // (R X)_ij = sum_k R_ik X_kj
                }
                rows.push(r);
            }
        }
    }
    rows
}

pub fn run() -> bool {
    let mut ck = Checks::new(
        "PROJECT spacetimes, Revision_3, command `algebra`: exact checks in integer and Gaussian-integer arithmetic (the 7+1 spacetime, x4 = time)",
    );
    let mm = mats();
    let g = &mm.g;
    let id16 = IMat::identity(16);
    let z16 = IMat::zeros(16, 16);

    // ------------------------------------------------------------------ A1 Clifford algebra
    let mut ok = true;
    for a in 0..8 {
        for c in 0..8 {
            let lhs = g[a].mul(&g[c]).add(&g[c].mul(&g[a]));
            let rhs = if a == c { id16.scale(2 * ETA1671[a]) } else { z16.clone() };
            ok &= lhs == rhs;
        }
    }
    ck.add("R3-A1", "the 8 real 16x16 matrices gamma^A_1671 satisfy gamma^A gamma^C + gamma^C gamma^A = 2 eta1671^{AC} 1 with eta1671 = diag(-1,-1,-1,-1,+1,-1,-1,-1) (64 identities)", ok, String::new());

    // ------------------------------------------------------------------ A2 reality and symmetry
    let mut ok = g[4].is_symmetric();
    for &j in SPACE.iter() {
        ok &= g[j].is_antisymmetric();
    }
    ck.add("R3-A2", "the matrices are real (integer); gamma^4 is symmetric and the seven gamma^j (j = 0,1,2,3,5,6,7) are antisymmetric; hence beta = gamma^4 and alpha^j = gamma^4 gamma^j are real symmetric and the generators gamma^A gamma^C of the spin group are real: the spinor representation is real, and psi^* transforms like psi", ok, String::new());

    // ------------------------------------------------------------------ A3 omega
    let om = &mm.omega;
    let mut ok = om.mul(om) == id16.neg() && om.transpose() == om.neg();
    for a in 0..8 {
        ok &= om.mul(&g[a]).add(&g[a].mul(om)).is_zero();
    }
    ck.add("R3-A3", "omega_1671 = gamma^0 ... gamma^7 satisfies omega^2 = -1, omega^T = -omega, and anticommutes with every gamma^A", ok, String::new());

    // ------------------------------------------------------------------ A4 charge conjugation matrices
    let (cp, cm) = (&mm.cp, &mm.cm);
    let mut ok = cp.is_symmetric() && cm.is_symmetric() && cp.mul(cp) == id16 && cm.mul(cm) == id16;
    for a in 0..8 {
        ok &= cp.mul(&g[a]) == g[a].transpose().mul(cp);
        ok &= cm.mul(&g[a]) == g[a].transpose().mul(cm).neg();
    }
    ck.add("R3-A4", "C^+ = gamma^4 and C^- = gamma^4 omega are real SYMMETRIC, (C^+)^2 = (C^-)^2 = 1, and C^+ gamma^A = +(gamma^A)^T C^+, C^- gamma^A = -(gamma^A)^T C^- for every A", ok, String::new());

    // ------------------------------------------------------------------ A5 Lorentz invariance of the pair forms
    let mut gens = Vec::new();
    for a in 0..8 {
        for c in (a + 1)..8 {
            gens.push(g[a].mul(&g[c]));
        }
    }
    let mut ok = true;
    for s in &gens {
        for cc in [cp, cm] {
            ok &= s.transpose().mul(cc).add(&cc.mul(s)).is_zero();
        }
    }
    ck.add("R3-A5", "for each of the 28 generators S = gamma^A gamma^C (A < C) of Spin(7,1): S^T C^+ + C^+ S = 0 and S^T C^- + C^- S = 0, so psi^T C^+ psi' and psi^T C^- psi' are Lorentz scalars", ok, String::new());

    // ------------------------------------------------------------------ A6 dimension of the space of invariant forms
    let rows = invariant_form_rows(&gens);
    let r = rank(&rows);
    // the two solutions C^+, C^- are linearly independent (C^- is not a multiple of C^+)
    let indep = rank(&[cp.flatten(), cm.flatten()]) == 2;
    ck.add(
        "R3-A6",
        "the bilinear forms K on the 16 spinor components that are invariant under Spin(7,1) (S^T K + K S = 0 for all 28 generators) form a space of dimension exactly 2, spanned by C^+ and C^-: the rank of the linear system (7168 equations, 256 unknowns) is 254, and C^+, C^- are two independent solutions",
        r == 254 && indep,
        format!("rank modulo 2^61-1 = {}, independent solutions = {}", r, if indep { 2 } else { 0 }),
    );

    // ------------------------------------------------------------------ A7 pairs of ONE field
    let basis = clifford_basis(g);
    let mut by_rank = vec![[0usize; 5]; 9]; // [count, antisym C+, antisym C-, sym C+, sym C-]
    let mut ok_kind = true;
    for (mask, th) in &basis {
        let r = rank_of(*mask);
        by_rank[r][0] += 1;
        for (k, cc) in [cp, cm].iter().enumerate() {
            let x = cc.mul(th);
            let anti = x.is_antisymmetric();
            let sym = x.is_symmetric();
            ok_kind &= anti != sym;
            if anti {
                by_rank[r][1 + k] += 1;
            } else {
                by_rank[r][3 + k] += 1;
            }
        }
    }
    let scalar_pairs_one_field = by_rank[0][1] + by_rank[0][2] + by_rank[8][1] + by_rank[8][2];
    ck.add(
        "R3-A7",
        "pairs psi^T X psi of ONE anticommuting field are nonzero only for antisymmetric X; of the 512 matrices C^+ G_K and C^- G_K (G_K = gamma^A1 ... gamma^Ar, A1 < ... < Ar, the 256 ordered products over the index sets K) each is either symmetric or antisymmetric, and the four Lorentz-scalar ones (rank 0 and rank 8: C^+, C^-, C^+ omega, C^- omega) are all symmetric: a single Dirac field on the 7+1 spacetime has NO spin-zero (Lorentz-scalar) pair",
        ok_kind && scalar_pairs_one_field == 0,
        format!(
            "antisymmetric C^+ G_K by rank 0..8: {:?}; antisymmetric C^- G_K by rank 0..8: {:?}",
            (0..9).map(|r| by_rank[r][1]).collect::<Vec<_>>(),
            (0..9).map(|r| by_rank[r][2]).collect::<Vec<_>>()
        ),
    );
    let rows_csv: Vec<Vec<String>> = (0..9)
        .map(|r| vec![r.to_string(), by_rank[r][0].to_string(), by_rank[r][1].to_string(), by_rank[r][2].to_string(), by_rank[r][3].to_string(), by_rank[r][4].to_string()])
        .collect();
    spacetimes::io::write_csv_str(
        "r3_pairs_by_rank.csv",
        &[("meaning", "number of ordered products G_K = gamma^A1 ... gamma^Ar (A1 < ... < Ar) of rank r for which C^+ G_K and C^- G_K are antisymmetric (a pair of one field exists) or symmetric (it vanishes)".to_string())],
        &["rank", "products", "antisym_Cplus", "antisym_Cminus", "sym_Cplus", "sym_Cminus"],
        &rows_csv,
    )
    .expect("csv");

    // ------------------------------------------------------------------ A8 pairs of the doublet
    let (one2, x2, z2, e2) = two_by_two();
    let mut ok = true;
    for cc in [cp, cm] {
        let a = kron(&e2, cc);
        ok &= a.is_antisymmetric() && !a.is_zero();
        for s in [&one2, &x2, &z2] {
            ok &= kron(s, cc).is_symmetric();
        }
    }
    ck.add(
        "R3-A8",
        "on the doublet Psi_U2 = (psi^1_U2, psi^2_U2) (32 components, doublet index outside) the matrices E (x) C^+ and E (x) C^- are antisymmetric and nonzero, while S (x) C^+- is symmetric for the three symmetric 2x2 matrices S = 1, tau^1, tau^3: the spin-zero pairs Phi^+-_U2 = psi^1T C^+- psi^2 = (1/2) Psi^T (E (x) C^+-) Psi exist, and they are antisymmetric in the doublet index",
        ok,
        String::new(),
    );

    // ------------------------------------------------------------------ A9 SU(2) invariance of the pair
    let tau = pauli();
    let eg = GMat::from_real(&e2);
    let mut ok = true;
    for t in &tau {
        ok &= gt(t).mul(&eg).add(&eg.mul(t)).is_zero();
    }
    // Pauli algebra and E = i tau^2
    ok &= gi(&tau[1]) == eg;
    for (a, b, c) in [(0usize, 1usize, 2usize), (1, 2, 0), (2, 0, 1)] {
        let comm = tau[a].mul(&tau[b]).sub(&tau[b].mul(&tau[a]));
        ok &= comm == gi(&tau[c]).scale(2);
    }
    ck.add(
        "R3-A9",
        "(tau^c)^T E + E tau^c = 0 for c = 1, 2, 3 (so U^T E U = det(U) E for every 2x2 matrix U, and E is invariant under SU(2)), E = i tau^2, and [tau^a, tau^b] = 2 i tau^c for (a,b,c) cyclic: the pair is an SU(2)_U2 singlet, and it carries the U(1)_U2 charge 2 y_U2 = 1",
        ok,
        String::new(),
    );

    // ------------------------------------------------------------------ A10 chirality structure
    let two_pl = GMat { re: id16.clone(), im: om.neg() }; // 2 P^L = 1 - i omega
    let two_pr = GMat { re: id16.clone(), im: om.clone() }; // 2 P^R = 1 + i omega
    let gb = GMat::from_real(&mm.beta);
    let mut ok = true;
    let mut pattern = Vec::new();
    for (nc, cc) in [("C+", cp), ("C-", cm)] {
        let c = GMat::from_real(cc);
        for (nx, px) in [("L", &two_pl), ("R", &two_pr)] {
            for (ny, py) in [("L", &two_pl), ("R", &two_pr)] {
                let x = gt(px).mul(&c).mul(py);
                let zero = x.is_zero();
                ok &= zero == (nx != ny);
                pattern.push(format!("{}:{}{}={}", nc, nx, ny, if zero { 0 } else { 1 }));
            }
        }
    }
    for (nx, px) in [("L", &two_pl), ("R", &two_pr)] {
        for (ny, py) in [("L", &two_pl), ("R", &two_pr)] {
            let x = px.dagger().mul(&gb).mul(py);
            ok &= x.is_zero() == (nx == ny);
        }
    }
    // one chiral field alone: (P^L)^T C^+ P^L is symmetric, so psi_L^T C^+ psi_L = 0
    let xl = gt(&two_pl).mul(&GMat::from_real(cp)).mul(&two_pl);
    ok &= gt(&xl) == xl;
    ck.add(
        "R3-A10",
        "with 2P^L = 1 - i omega, 2P^R = 1 + i omega: (P^X)^T C^+- P^Y = 0 exactly when X differs from Y, so the spin-zero pairs join a left-handed with a left-handed and a right-handed with a right-handed field; the mass term psi-bar psi joins L with R; (P^L)^T C^+ P^L is symmetric, so a single chiral field has no spin-zero pair either, while a chiral DOUBLET has one",
        ok,
        pattern.join(" "),
    );

    // ------------------------------------------------------------------ A11 rest frame: spin content
    let opb = id16.add(&mm.beta); // 1 + beta = 2 Pi^+
    let omb = id16.sub(&mm.beta); // 1 - beta = 2 Pi^-
    let ok = opb.mul(cp).mul(&opb) == opb.scale(2) && omb.mul(cp).mul(&omb) == omb.scale(-2) && opb.mul(cm).mul(&opb).is_zero() && omb.mul(cm).mul(&omb).is_zero();
    ck.add(
        "R3-A11",
        "in the rest frame: (1+beta) C^+ (1+beta) = 2(1+beta) and (1-beta) C^+ (1-beta) = -2(1-beta), while (1+-beta) C^- (1+-beta) = 0: C^+ pairs two upper (positive-energy, rest-frame) components with each other (s-wave), C^- never does (it pairs upper with lower components, which vanish in the non-relativistic limit)",
        ok,
        String::new(),
    );

    // ------------------------------------------------------------------ A12 Spin(7) irreducibility
    let mut rot = Vec::new();
    for (ia, &i) in SPACE.iter().enumerate() {
        for &j in SPACE.iter().skip(ia + 1) {
            rot.push(g[i].mul(&g[j]));
        }
    }
    let mut ok_comm_beta = true;
    for r in &rot {
        ok_comm_beta &= r.mul(&mm.beta) == mm.beta.mul(r);
        ok_comm_beta &= r.is_antisymmetric();
    }
    let r_no_beta = rank(&commutant_rows(&rot));
    let mut with_beta = rot.clone();
    with_beta.push(mm.beta.clone());
    let r_beta = rank(&commutant_rows(&with_beta));
    let sols4 = [id16.clone(), mm.beta.clone(), om.clone(), mm.beta.mul(om)];
    let mut sols_ok = rank(&sols4.iter().map(|x| x.flatten()).collect::<Vec<_>>()) == 4;
    for s in &sols4 {
        for r in &rot {
            sols_ok &= s.mul(r) == r.mul(s);
        }
    }
    sols_ok &= id16.mul(&mm.beta) == mm.beta.mul(&id16);
    ck.add(
        "R3-A12",
        "the 21 rotation generators gamma^i gamma^j (i < j space-like) are real antisymmetric and commute with beta; their commutant has dimension exactly 4 (spanned by 1, beta, omega, beta omega) and the commutant of the rotations together with beta has dimension exactly 2 (1, beta): each 8-dimensional eigenspace of beta is an absolutely irreducible real representation of Spin(7), so the rotation-invariant bilinear form on it is unique and there is exactly ONE spin-zero state of two rest-frame fermions of different flavour",
        ok_comm_beta && r_no_beta == 252 && r_beta == 254 && sols_ok,
        format!("ranks modulo 2^61-1: {} (rotations), {} (rotations and beta); 4 resp. 2 independent solutions exhibited", r_no_beta, r_beta),
    );

    // ------------------------------------------------------------------ A13 BdG algebra
    let mut ok = cp.mul(&mm.beta) == mm.beta.mul(cp) && cm.mul(&mm.beta) == mm.beta.mul(cm).neg();
    for (ia, a) in mm.alpha.iter().enumerate() {
        ok &= a.is_symmetric() && a.mul(a) == id16;
        ok &= a.mul(&mm.beta).add(&mm.beta.mul(a)).is_zero();
        ok &= cp.mul(a).add(&a.mul(cp)).is_zero() && cm.mul(a).add(&a.mul(cm)).is_zero();
        for (ib, b) in mm.alpha.iter().enumerate() {
            if ib != ia {
                ok &= a.mul(b).add(&b.mul(a)).is_zero();
            }
        }
    }
    ok &= mm.beta.mul(&mm.beta) == id16 && mm.beta.is_symmetric();
    // Nambu vertices: V3 = diag(beta, -beta), V1+- = [[0, C+-], [C+-, 0]], A = diag(alpha^j, alpha^j)
    let v3 = block(&mm.beta, &z16, &z16, &mm.beta.neg());
    let v1p = block(&z16, cp, cp, &z16);
    let v1m = block(&z16, cm, cm, &z16);
    let anti_p = v3.mul(&v1p).add(&v1p.mul(&v3)).is_zero();
    let comm_m = v3.mul(&v1m) == v1m.mul(&v3);
    let mut a_ok = true;
    for a in &mm.alpha {
        let aa = block(a, &z16, &z16, a);
        a_ok &= aa.mul(&v3).add(&v3.mul(&aa)).is_zero() && aa.mul(&v1p).add(&v1p.mul(&aa)).is_zero() && aa.mul(&v1m).add(&v1m.mul(&aa)).is_zero();
    }
    ck.add(
        "R3-A13",
        "Hamiltonian algebra: alpha^j, beta real symmetric with {alpha^i, alpha^j} = 2 delta^{ij}, {alpha^j, beta} = 0, beta^2 = 1; C^+ anticommutes with every alpha^j and COMMUTES with beta, C^- anticommutes with every alpha^j and with beta; in the 32-dimensional Nambu space the mass vertex diag(beta,-beta) ANTICOMMUTES with the C^+ pairing vertex [[0,C^+],[C^+,0]] (mass and gap add in quadrature) but COMMUTES with the C^- pairing vertex (mass and gap add or subtract); diag(alpha^j, alpha^j) anticommutes with all three",
        ok && anti_p && comm_m && a_ok,
        String::new(),
    );

    // ------------------------------------------------------------------ A14 Kronecker form of the matrices
    let (one, x, z, e) = two_by_two();
    let fac = [("1", &one), ("X", &x), ("Z", &z), ("E", &e)];
    let mut kron_rows = Vec::new();
    let mut ok = true;
    for a in 0..8 {
        let mut found: Option<(i64, String)> = None;
        for f1 in &fac {
            for f2 in &fac {
                for f3 in &fac {
                    for f4 in &fac {
                        let k = kron(f1.1, &kron(f2.1, &kron(f3.1, f4.1)));
                        for s in [1i64, -1] {
                            if k.scale(s) == g[a] && found.is_none() {
                                found = Some((s, format!("{}{}{}{}", f1.0, f2.0, f3.0, f4.0)));
                            }
                        }
                    }
                }
            }
        }
        match found {
            Some((s, f)) => kron_rows.push(vec![a.to_string(), s.to_string(), f[0..1].to_string(), f[1..2].to_string(), f[2..3].to_string(), f[3..4].to_string()]),
            None => ok = false,
        }
    }
    spacetimes::io::write_csv_str(
        "r3_gamma_kron.csv",
        &[("meaning", "gamma^A_1671 = sign * f1 (x) f2 (x) f3 (x) f4 with the real 2x2 matrices 1, X = [[0,1],[1,0]], Z = [[1,0],[0,-1]], E = [[0,1],[-1,0]]".to_string())],
        &["A", "sign", "f1", "f2", "f3", "f4"],
        &kron_rows,
    )
    .expect("csv");
    ck.add(
        "R3-A14",
        "each of the 8 matrices gamma^A_1671 is +- a Kronecker product f1 (x) f2 (x) f3 (x) f4 of four of the real 2x2 matrices 1, X, Z, E (exact equality of all 256 entries), so the matrices can be written down completely in eight short formulas (data/r3_gamma_kron.csv)",
        ok,
        kron_rows.iter().map(|r| format!("{}:{}{}{}{}{}", r[0], if r[1] == "1" { "+" } else { "-" }, r[2], r[3], r[4], r[5])).collect::<Vec<_>>().join(" "),
    );

    // ------------------------------------------------------------------ B1 Casimir of SU(2) on two doublets
    let mut sum = GMat::from_real(&IMat::zeros(4, 4));
    for t in &tau {
        sum = sum.add(&gkron(t, t));
    }
    let mut swap = IMat::zeros(4, 4);
    for u in 0..2 {
        for v in 0..2 {
            swap.set(2 * u + v, 2 * v + u, 1);
        }
    }
    let target = GMat::from_real(&swap.scale(2).sub(&IMat::identity(4)));
    ck.add(
        "R3-B1",
        "sum_c tau^c (x) tau^c = 2 SWAP - 1 on two doublets: on the antisymmetric (singlet) state it equals -3, on the three symmetric (triplet) states +1; hence T_1 . T_2 = -3/4 in the singlet and +1/4 in the triplet, and the exchange of the SU(2)_U2 bosons attracts in the singlet channel of the pair",
        sum == target,
        String::new(),
    );

    // ------------------------------------------------------------------ B2 SU(2) has no odd symmetric invariants; anomaly sums
    let mut ok = true;
    let perms3 = permutations(3);
    let perms5 = permutations(5);
    for i0 in 0..3 {
        for i1 in 0..3 {
            for i2 in 0..3 {
                let idx = [i0, i1, i2];
                let mut s = (0i64, 0i64);
                for p in &perms3 {
                    let m = tau[idx[p[0]]].mul(&tau[idx[p[1]]]).mul(&tau[idx[p[2]]]);
                    let t = gtrace(&m);
                    s = (s.0 + t.0, s.1 + t.1);
                }
                ok &= s == (0, 0);
            }
        }
    }
    let mut n5 = 0;
    for code in 0..243usize {
        let idx = [code % 3, (code / 3) % 3, (code / 9) % 3, (code / 27) % 3, (code / 81) % 3];
        let mut s = (0i64, 0i64);
        for p in &perms5 {
            let mut m = tau[idx[p[0]]].clone();
            for k in 1..5 {
                m = m.mul(&tau[idx[p[k]]]);
            }
            let t = gtrace(&m);
            s = (s.0 + t.0, s.1 + t.1);
        }
        ok &= s == (0, 0);
        n5 += 1;
    }
    // U(1)_U2 charges y = Y/2 of the left-handed Weyl components (each component an 8-component Weyl field)
    let chiral: [i64; 2] = [1, 1]; // psi^1_L, psi^2_L with y = +1/2
    let vector: [i64; 4] = [1, 1, -1, -1]; // psi_L (y=+1/2) and (psi_R)^c (y=-1/2) for both members
    let sums = |ys: &[i64]| -> (i64, i64, i64) { (ys.iter().map(|y| y.pow(5)).sum(), ys.iter().map(|y| y.pow(3)).sum(), ys.iter().sum()) };
    let (c5, c3, c1) = sums(&chiral);
    let (v5, v3s, v1) = sums(&vector);
    ok &= (v5, v3s, v1) == (0, 0, 0) && c5 != 0 && c3 != 0 && c1 != 0;
    ck.add(
        "R3-B2",
        "the symmetrized traces of 3 and of 5 Pauli matrices vanish for all 27 resp. 243 index choices (SU(2)_U2 alone has no perturbative anomaly in 8 dimensions); for the left-handed Weyl content the U(1)_U2 anomaly coefficients sum y^5, sum y^3, sum y are (1/16, 1/4, 1) for a CHIRAL doublet (anomalous: U(1)_U2 cannot be gauged) and (0, 0, 0) for the Dirac doublet, whose right-handed members are left-handed fields of charge -y",
        ok,
        format!("chiral: sum Y^5 = {}, sum Y^3 = {}, sum Y = {} (y = Y/2); Dirac: {}, {}, {}; {} five-fold traces checked", c5, c3, c1, v5, v3s, v1, n5),
    );

    ck.write("r3_algebra_checks.txt")
}

/// The 2n x 2n block matrix [[a, b], [c, d]].
pub fn block(a: &IMat, b: &IMat, c: &IMat, d: &IMat) -> IMat {
    let n = a.n;
    let mut r = IMat::zeros(2 * n, 2 * n);
    for i in 0..n {
        for j in 0..n {
            r.set(i, j, a.get(i, j));
            r.set(i, n + j, b.get(i, j));
            r.set(n + i, j, c.get(i, j));
            r.set(n + i, n + j, d.get(i, j));
        }
    }
    r
}

/// All permutations of 0..n.
fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 0 {
        return vec![vec![]];
    }
    let mut out = Vec::new();
    for p in permutations(n - 1) {
        for pos in 0..=p.len() {
            let mut q = p.clone();
            q.insert(pos, n - 1);
            out.push(q);
        }
    }
    out
}
