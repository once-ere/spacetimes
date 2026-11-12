//! Machine-checked properties of the two sets of real 16x16 Dirac matrices.
//!
//! Each check has an identifier (quoted in the provenance document
//! `dirac matrices.md`), a statement in words, and a result.  Checks marked
//! "exact" are identities between integer (or Gaussian-integer) matrices and
//! are proved by exact arithmetic; checks marked "numerical" involve
//! transcendental functions and are verified to a stated tolerance.

use crate::fmat::{FMat, Rng};
use crate::gamma::{clifford_basis, swap04, rank_of, Gammas, ETA17_NB, ETA1644, ETA1671};
use crate::imat::{rank, GMat, IMat};

/// One verified statement.
pub struct Check {
    pub id: String,
    pub kind: &'static str, // "exact" or "numerical"
    pub statement: String,
    pub pass: bool,
}

/// The list of all checks, in the order in which they were made.
pub struct Report {
    pub checks: Vec<Check>,
}

impl Report {
    pub fn new() -> Report {
        Report { checks: Vec::new() }
    }
    pub fn add(&mut self, id: &str, kind: &'static str, statement: &str, pass: bool) {
        self.checks.push(Check { id: id.to_string(), kind, statement: statement.to_string(), pass });
    }
    pub fn all_pass(&self) -> bool {
        self.checks.iter().all(|c| c.pass)
    }
    pub fn count(&self) -> (usize, usize) {
        (self.checks.iter().filter(|c| c.pass).count(), self.checks.len())
    }
}

/// Matrices printed by Mathematica and extracted from a notebook by
/// `tools/extract_nb_matrices.py`:  (cell number, index within the cell, matrix).
pub fn read_notebook_matrices(path: &str) -> Vec<(usize, usize, IMat)> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| panic!("cannot read {}: {}", path, e));
    let mut out = Vec::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        if !line.starts_with("MATRIX ") {
            continue;
        }
        // MATRIX <notebook> cell <n> index <k> rows <r> cols <c>
        let w: Vec<&str> = line.split_whitespace().collect();
        let cell: usize = w[3].parse().unwrap();
        let index: usize = w[5].parse().unwrap();
        let rows: usize = w[7].parse().unwrap();
        let cols: usize = w[9].parse().unwrap();
        let mut m = IMat::zeros(rows, cols);
        for i in 0..rows {
            let row: Vec<i64> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            assert_eq!(row.len(), cols);
            for j in 0..cols {
                m.set(i, j, row[j]);
            }
        }
        out.push((cell, index, m));
    }
    out
}

/// The integer matrix [g^A, g^B] = 4 Sigma^{AB}, where Sigma^{AB} = (1/4)[g^A, g^B]
/// is the "scaled commutator" (the generator of Lorentz transformations).
fn m_ab(g: &[IMat], a: usize, b: usize) -> IMat {
    g[a].comm(&g[b])
}

/// Checks that hold, with the appropriate metric `eta`, for BOTH sets of matrices.
/// `tag` is "44" or "71"; `name` is the name of the set used in the statements.
pub fn verify_common(tag: &str, name: &str, g: &[IMat], eta: &[i64; 8], rep: &mut Report) {
    let id16 = IMat::identity(16);
    let p = |s: &str| format!("{}-{}", tag, s);
    // tag of the PROJECT symbols in the statements: 1644 (4+4) or 1671 (7+1)
    let t = format!("16{}", tag);

    // ---- R: reality -----------------------------------------------------------
    rep.add(&p("R1"), "exact", &format!("every entry of {}[A], A = 0..7, is one of the integers -1, 0, +1; hence the eight matrices are real", name),
        g.iter().all(|m| m.n == 16 && m.m == 16 && m.entries_in_unit_set()));
    // each row and each column has exactly one non-zero entry: signed permutation matrices
    let signed_perm = g.iter().all(|m| {
        (0..16).all(|i| (0..16).filter(|&j| m.get(i, j) != 0).count() == 1) && (0..16).all(|j| (0..16).filter(|&i| m.get(i, j) != 0).count() == 1)
    });
    rep.add(&p("R2"), "exact", &format!("every {}[A] is a signed permutation matrix (one entry +1 or -1 in each row and in each column)", name), signed_perm);

    // ---- C: Clifford (anticommutation) relations ----------------------------------
    let mut ok = true;
    for a in 0..8 {
        for b in 0..8 {
            let want = if a == b { 2 * eta[a] } else { 0 };
            ok &= g[a].anti(&g[b]) == id16.scale(want);
        }
    }
    rep.add(&p("C1"), "exact", &format!("{0}[A].{0}[B] + {0}[B].{0}[A] = 2 eta{t}^(AB) 1 for all 64 pairs (A, B), with eta{t} = diag{1:?}", name, eta), ok);
    rep.add(&p("C2"), "exact", &format!("transpose({0}[A]) = eta{t}^(AA) {0}[A]: matrices with square +1 are symmetric, matrices with square -1 are antisymmetric; hence transpose({0}[A]) = inverse({0}[A])", name),
        (0..8).all(|a| g[a].transpose() == g[a].scale(eta[a]) && g[a].mul(&g[a].transpose()) == id16));
    let basis = clifford_basis(g);
    let rows: Vec<Vec<i64>> = basis.iter().map(|(_, m)| m.flatten()).collect();
    rep.add(&p("C3"), "exact", &format!("the 256 ordered products {0}[A1]...{0}[Ak] (A1 < ... < Ak, k = 0..8) are linearly independent: they span all real 16x16 matrices, so the representation is irreducible over the real numbers", name),
        rank(&rows) == 256);
    rep.add(&p("C4"), "exact", "every one of the 255 non-empty products has trace 0, and the square of every product is +1 or -1",
        basis.iter().all(|(mask, m)| (*mask == 0 || m.trace() == 0) && { let s = m.mul(m); s == id16 || s == id16.neg() }));

    // ---- L: the Lie algebra generated by the scaled commutators ---------------------
    // Sigma^{AB} = (1/4)[g^A, g^B]; to stay in the integers we use M^{AB} = 4 Sigma^{AB}.
    let mut ok = true;
    for a in 0..8 {
        for b in 0..8 {
            // M^{AB} = 2 g^A g^B for A != B
            if a != b {
                ok &= m_ab(g, a, b) == g[a].mul(&g[b]).scale(2);
            }
        }
    }
    rep.add(&p("L1"), "exact", &format!("Sigma{t}^(AB) := (1/4)[{0}[A], {0}[B]] equals (1/2) {0}[A].{0}[B] for A != B, and Sigma{t}^(AB) = -Sigma{t}^(BA)", name), ok);
    let pairs: Vec<(usize, usize)> = (0..8).flat_map(|a| ((a + 1)..8).map(move |b| (a, b))).collect();
    let gens: Vec<IMat> = pairs.iter().map(|&(a, b)| g[a].mul(&g[b])).collect(); // N^{AB} = 2 Sigma^{AB}
    let rows: Vec<Vec<i64>> = gens.iter().map(|m| m.flatten()).collect();
    rep.add(&p("L2"), "exact", &format!("the 28 matrices Sigma{t}^(AB), A < B, are linearly independent"), rank(&rows) == 28);
    let mut ok = true;
    for a in 0..8 {
        for b in 0..8 {
            for c in 0..8 {
                // [Sigma^{AB}, g^C] = eta^{BC} g^A - eta^{AC} g^B   <=>   [M^{AB}, g^C] = 4(...)
                let lhs = m_ab(g, a, b).comm(&g[c]);
                let mut rhs = IMat::zeros(16, 16);
                if b == c {
                    rhs = rhs.add(&g[a].scale(4 * eta[b]));
                }
                if a == c {
                    rhs = rhs.sub(&g[b].scale(4 * eta[a]));
                }
                ok &= lhs == rhs;
            }
        }
    }
    rep.add(&p("L3"), "exact", &format!("[Sigma{t}^(AB), {0}[C]] = eta{t}^(BC) {0}[A] - eta{t}^(AC) {0}[B] for all A, B, C: the Sigma{t}^(AB) rotate the eight matrices as the components of a vector", name), ok);
    let mut ok = true;
    for a in 0..8 {
        for b in 0..8 {
            for c in 0..8 {
                for d in 0..8 {
                    // [M^{AB}, M^{CD}] = 4 (eta^{BC} M^{AD} - eta^{AC} M^{BD} - eta^{BD} M^{AC} + eta^{AD} M^{BC})
                    let lhs = m_ab(g, a, b).comm(&m_ab(g, c, d));
                    let mut rhs = IMat::zeros(16, 16);
                    if b == c {
                        rhs = rhs.add(&m_ab(g, a, d).scale(4 * eta[b]));
                    }
                    if a == c {
                        rhs = rhs.sub(&m_ab(g, b, d).scale(4 * eta[a]));
                    }
                    if b == d {
                        rhs = rhs.sub(&m_ab(g, a, c).scale(4 * eta[b]));
                    }
                    if a == d {
                        rhs = rhs.add(&m_ab(g, b, c).scale(4 * eta[a]));
                    }
                    ok &= lhs == rhs;
                }
            }
        }
    }
    rep.add(&p("L4"), "exact", &format!("[Sigma{t}^(AB), Sigma{t}^(CD)] = eta{t}^(BC) Sigma{t}^(AD) - eta{t}^(AC) Sigma{t}^(BD) - eta{t}^(BD) Sigma{t}^(AC) + eta{t}^(AD) Sigma{t}^(BC) for all 4096 index combinations: the commutation relations of the Lie algebra so(p,q) of the metric eta{t}"), ok);

    // Killing form from the structure constants, computed from the matrices alone.
    // With N_I = 2 Sigma_I:  f_IJ^K = tr([N_I, N_J] N_K) / (2 tr(N_K N_K)).
    let ng = gens.len();
    let trsq: Vec<i64> = gens.iter().map(|m| m.mul(m).trace()).collect();
    let mut f = vec![0i64; ng * ng * ng];
    let mut integral = true;
    for i in 0..ng {
        for j in 0..ng {
            let c = gens[i].comm(&gens[j]);
            if c.is_zero() {
                continue;
            }
            for k in 0..ng {
                let t = c.mul(&gens[k]).trace();
                let den = 2 * trsq[k];
                if t % den != 0 {
                    integral = false;
                }
                f[(i * ng + j) * ng + k] = t / den;
            }
        }
    }
    let mut killing = IMat::zeros(ng, ng);
    for i in 0..ng {
        for j in 0..ng {
            let mut s = 0i64;
            for k in 0..ng {
                for l in 0..ng {
                    s += f[(i * ng + k) * ng + l] * f[(j * ng + l) * ng + k];
                }
            }
            killing.set(i, j, s);
        }
    }
    let diag_ok = (0..ng).all(|i| (0..ng).all(|j| i == j || killing.get(i, j) == 0));
    let expect_ok = (0..ng).all(|i| killing.get(i, i) == -12 * eta[pairs[i].0] * eta[pairs[i].1]);
    let npos = (0..ng).filter(|&i| killing.get(i, i) > 0).count();
    let nneg = (0..ng).filter(|&i| killing.get(i, i) < 0).count();
    let nspace = eta.iter().filter(|&&x| x == eta[0]).count(); // number of indices with the sign of eta[0]
    let (pp, qq) = (nspace, 8 - nspace);
    let want_noncompact = pp * qq;
    let want_compact = pp * (pp - 1) / 2 + qq * (qq - 1) / 2;
    rep.add(&p("L5"), "exact", &format!("the structure constants are integers and the Killing form is diagonal in the basis Sigma{t}^(AB): B(Sigma{t}^(AB), Sigma{t}^(AB)) = -12 eta{t}^(AA) eta{t}^(BB); it has {} positive and {} negative eigenvalues (no zero eigenvalue: the algebra is semisimple)", npos, nneg),
        integral && diag_ok && expect_ok && npos + nneg == 28 && npos == want_noncompact && nneg == want_compact);
    let nsym = gens.iter().filter(|m| m.is_symmetric()).count();
    let nanti = gens.iter().filter(|m| m.is_antisymmetric()).count();
    rep.add(&p("L6"), "exact", &format!("{} generators are antisymmetric matrices (compact: they generate rotations) and {} are symmetric (non-compact: they generate boosts); these are the numbers of the real form so({},{}), which has {} compact and {} non-compact generators", nanti, nsym, pp.max(qq), pp.min(qq), want_compact, want_noncompact),
        nanti == want_compact && nsym == want_noncompact && nanti == nneg && nsym == npos);

    // ---- G: group elements (Pin) -----------------------------------------------------
    let mut ok = true;
    for a in 0..8 {
        let ginv = g[a].scale(eta[a]); // inverse of g^A, since g^A g^A = eta^{AA}
        ok &= g[a].mul(&ginv) == id16;
        for b in 0..8 {
            // twisted adjoint action  v -> - g^A v (g^A)^{-1}:  reflection in the hyperplane orthogonal to e_A
            let r = g[a].mul(&g[b]).mul(&ginv).neg();
            ok &= if a == b { r == g[b].neg() } else { r == g[b] };
        }
    }
    rep.add(&p("G1"), "exact", &format!("each {0}[A] is invertible with inverse eta{t}^(AA) {0}[A], and -{0}[A].{0}[B].inverse({0}[A]) = {0}[B] for B != A, = -{0}[A] for B = A: {0}[A] acts on the vector index as the reflection that reverses the axis xA", name), ok);

    // numerical: exp(theta Sigma^{AB}) in closed form, and its action on the vector index
    let gf: Vec<FMat> = g.iter().map(FMat::from_imat).collect();
    let mut rng = Rng(0x9E3779B97F4A7C15);
    let mut worst_closed = 0.0f64;
    for &(a, b) in &pairs {
        let th = rng.range(-2.0, 2.0);
        let sig = gf[a].mul(&gf[b]).scale(0.5); // Sigma^{AB}
        let s = sig.scale(th).exp();
        // (g^A g^B)^2 = -eta^{AA} eta^{BB}:  rotation if -1 (cos, sin), boost if +1 (cosh, sinh)
        let sq = -eta[a] * eta[b];
        let (c, sn) = if sq < 0 { ((th / 2.0).cos(), (th / 2.0).sin()) } else { ((th / 2.0).cosh(), (th / 2.0).sinh()) };
        let closed = FMat::identity(16).scale(c).add(&gf[a].mul(&gf[b]).scale(sn));
        worst_closed = worst_closed.max(s.sub(&closed).max_abs());
    }
    rep.add(&p("G2"), "numerical", &format!("exp(theta Sigma{t}^(AB)) = cos(theta/2) 1 + sin(theta/2) {0}[A].{0}[B] when ({0}[A].{0}[B])^2 = -1 (rotation), and = cosh(theta/2) 1 + sinh(theta/2) {0}[A].{0}[B] when ({0}[A].{0}[B])^2 = +1 (boost), for the 28 planes and random theta (largest deviation {1:.1e})", name, worst_closed),
        worst_closed < 1e-12);
    let mut worst_res = 0.0f64;
    let mut worst_orth = 0.0f64;
    let mut worst_det = 0.0f64;
    for _trial in 0..20 {
        // a general element S = exp( sum_{A<B} omega_AB Sigma^{AB} )
        let mut x = FMat::zeros(16);
        for &(a, b) in &pairs {
            x = x.add(&gf[a].mul(&gf[b]).scale(0.5 * rng.range(-0.8, 0.8)));
        }
        let s = x.exp();
        let sinv = s.inverse().unwrap();
        // S g^C S^{-1} = sum_D Lambda[C][D] g^D,  Lambda[C][D] = tr(S g^C S^{-1} g^D) / (16 eta^{DD})
        let mut lam = FMat::zeros(8);
        for c in 0..8 {
            let t = s.mul(&gf[c]).mul(&sinv);
            let mut recon = FMat::zeros(16);
            for d in 0..8 {
                let l = t.mul(&gf[d]).trace() / (16.0 * eta[d] as f64);
                lam.set(c, d, l);
                recon = recon.add(&gf[d].scale(l));
            }
            worst_res = worst_res.max(t.sub(&recon).max_abs());
        }
        // Lambda eta Lambda^T = eta
        let mut e = FMat::zeros(8);
        for i in 0..8 {
            e.set(i, i, eta[i] as f64);
        }
        worst_orth = worst_orth.max(lam.mul(&e).mul(&lam.transpose()).sub(&e).max_abs());
        // determinant of Lambda by LU (via the inverse routine's pivots is overkill): use the 8x8 exponential identity
        // det(Lambda) = +1 is checked through det = exp(tr(log)) = 1 for a matrix connected to the identity:
        // here we verify it directly by Gaussian elimination.
        worst_det = worst_det.max((det(&lam) - 1.0).abs());
    }
    rep.add(&p("G3"), "numerical", &format!("for 20 random elements S = exp(sum_(A<B) theta_AB Sigma{t}^(AB)) (theta_AB random real numbers):  S.{0}[C].inverse(S) = sum_D Lambda^C_D {0}[D] with a real 8x8 matrix Lambda that preserves eta{t} (Lambda.eta{t}.transpose(Lambda) = eta{t}) and has determinant +1 (largest deviations {1:.1e}, {2:.1e}, {3:.1e})", name, worst_res, worst_orth, worst_det),
        worst_res < 1e-10 && worst_orth < 1e-10 && worst_det < 1e-10);
}

/// Determinant of a small real matrix by Gaussian elimination with partial pivoting.
pub fn det(m: &FMat) -> f64 {
    let n = m.n;
    let mut a = m.a.clone();
    let mut d = 1.0;
    for c in 0..n {
        let mut p = c;
        for r in (c + 1)..n {
            if a[r * n + c].abs() > a[p * n + c].abs() {
                p = r;
            }
        }
        if a[p * n + c] == 0.0 {
            return 0.0;
        }
        if p != c {
            for j in 0..n {
                a.swap(c * n + j, p * n + j);
            }
            d = -d;
        }
        d *= a[c * n + c];
        for r in (c + 1)..n {
            let f = a[r * n + c] / a[c * n + c];
            for j in c..n {
                a[r * n + j] -= f * a[c * n + j];
            }
        }
    }
    d
}

/// Checks that are specific to the 4+4 matrices of `4+4.nb`.
pub fn verify_44(gm: &Gammas, nb: &[(usize, usize, IMat)], rep: &mut Report) {
    let g = &gm.gamma1644;
    let id16 = IMat::identity(16);
    let id8 = IMat::identity(8);
    let z8 = IMat::zeros(8, 8);
    let z16 = IMat::zeros(16, 16);

    // ---- N: identity with the matrices printed in the notebook ---------------------
    let t16_nb: Vec<&IMat> = nb.iter().filter(|x| x.0 == 314).map(|x| &x.2).collect();
    rep.add("44-N1", "exact", "the eight matrices rebuilt from the definitions equal, entry by entry, the eight matrices T16[0..7] printed by Mathematica in 4+4.nb (cell 314, Out[211])",
        t16_nb.len() == 8 && (0..8).all(|a| *t16_nb[a] == g[a]));
    let get = |cell: usize| nb.iter().find(|x| x.0 == cell).map(|x| x.2.clone());
    rep.add("44-N2", "exact", "the rebuilt volume element equals T16[8] printed in 4+4.nb (cell 136, Out[99])", get(136) == Some(gm.t16_8.clone()));
    rep.add("44-N3", "exact", "the metric printed in 4+4.nb (cell 13, Out[9]) is eta4488 = diag(1,1,1,1,-1,-1,-1,-1) (PROJECT name: eta1644)", get(13) == Some(IMat::diag(&ETA1644)));

    // ---- definitions of the 8x8 blocks (checks made in 4+4.nb, repeated) ------------
    let (tau, tb, sg) = (&gm.tau, &gm.taubar, &gm.sigma);
    rep.add("44-B1", "exact", "the 8x8 matrix sigma (= sigma88) of 4+4.nb, PROJECT name sigma88: sigma88 = tau[1].tau[2].tau[3] = tau[4].tau[5].tau[6].tau[7], sigma88.sigma88 = 1, tau[1].tau[2]...tau[7] = 1, and tau[7] = diag(-1_4, +1_4) (4+4.nb Out[72], Out[89], Out[87], Out[63])",
        tau[7] == IMat::diag(&[-1, -1, -1, -1, 1, 1, 1, 1]) && *sg == IMat::product(&[&tau[1], &tau[2], &tau[3]]) && *sg == IMat::product(&[&tau[4], &tau[5], &tau[6], &tau[7]]) && sg.mul(sg) == id8
            && IMat::product(&[&tau[1], &tau[2], &tau[3], &tau[4], &tau[5], &tau[6], &tau[7]]) == id8);
    let mut ok = true;
    for a in 0..8 {
        for b in 0..8 {
            let want = if a == b { 2 * ETA1644[a] } else { 0 };
            ok &= tau[a].mul(&tb[b]).add(&tau[b].mul(&tb[a])) == id8.scale(want);
            ok &= tb[a].mul(&tau[b]).add(&tb[b].mul(&tau[a])) == id8.scale(want);
        }
    }
    rep.add("44-B2", "exact", "tau[A].taubar[B] + tau[B].taubar[A] = 2 eta1644^(AB) 1 and taubar[A].tau[B] + taubar[B].tau[A] = 2 eta1644^(AB) 1 (8x8), A, B = 0..7 (4+4.nb Out[77])", ok);
    rep.add("44-B3", "exact", "sigma88.taubar[A] = transpose(sigma88.tau[A]) for A = 0..7 (4+4.nb Out[76])",
        (0..8).all(|a| sg.mul(&tb[a]) == sg.mul(&tau[a]).transpose()));
    rep.add("44-B4", "exact", "T16[A] = [[0, taubar[A]], [tau[A], 0]] in 8x8 blocks, with tau[0] = taubar[0] = 1.  PROJECT names of these objects of 4+4.nb: gamma1644[A] := T16[A], tau1644[A] := tau[A], taubar1644[A] := taubar[A]",
        (0..8).all(|a| g[a] == IMat::block2(&z8, &tb[a], &tau[a], &z8)) && tau[0] == id8 && tb[0] == id8);

    // ---- V: volume element and chirality -----------------------------------------------
    let om = &gm.t16_8;
    rep.add("44-V1", "exact", "omega1644 := gamma1644[0]...gamma1644[7] (= T16[8]) = diag(-1_8, +1_8): real, diagonal, symmetric, with square +1",
        *om == IMat::block2(&id8.neg(), &z8, &z8, &id8) && om.mul(om) == id16);
    rep.add("44-V2", "exact", "omega1644 anticommutes with every gamma1644[A] and commutes with every Sigma1644^(AB)",
        (0..8).all(|a| om.anti(&g[a]).is_zero()) && (0..8).all(|a| (0..8).all(|b| om.comm(&g[a].mul(&g[b])).is_zero())));
    // projectors: 2 PL1644 = 1 - omega1644, 2 PR1644 = 1 + omega1644  must have even entries
    let two_pl = id16.sub(om);
    let two_pr = id16.add(om);
    let even = two_pl.a.iter().chain(two_pr.a.iter()).all(|x| x % 2 == 0);
    let pl = IMat { n: 16, m: 16, a: two_pl.a.iter().map(|x| x / 2).collect() };
    let pr = IMat { n: 16, m: 16, a: two_pr.a.iter().map(|x| x / 2).collect() };
    rep.add("44-P1", "exact", "PL1644 := (1 - omega1644)/2 = diag(1_8, 0_8) and PR1644 := (1 + omega1644)/2 = diag(0_8, 1_8) are real matrices with integer entries",
        even && pl == IMat::block2(&id8, &z8, &z8, &z8) && pr == IMat::block2(&z8, &z8, &z8, &id8));
    rep.add("44-P2", "exact", "PL1644 + PR1644 = 1, PL1644.PL1644 = PL1644, PR1644.PR1644 = PR1644, PL1644.PR1644 = PR1644.PL1644 = 0, trace PL1644 = trace PR1644 = 8 (4+4.nb Out[114])",
        pl.add(&pr) == id16 && pl.mul(&pl) == pl && pr.mul(&pr) == pr && pl.mul(&pr) == z16 && pr.mul(&pl) == z16 && pl.trace() == 8 && pr.trace() == 8);
    rep.add("44-P3", "exact", "the rebuilt projectors PL1644 and PR1644 equal the matrices P_L and P_R printed in 4+4.nb (cells 163 and 165, Out[112] and Out[113])",
        get(163) == Some(pl.clone()) && get(165) == Some(pr.clone()));
    rep.add("44-P4", "exact", "gamma1644[A].PL1644 = PR1644.gamma1644[A] and gamma1644[A].PR1644 = PL1644.gamma1644[A]: every Dirac matrix maps left-handed spinors to right-handed ones and conversely",
        (0..8).all(|a| g[a].mul(&pl) == pr.mul(&g[a]) && g[a].mul(&pr) == pl.mul(&g[a])));
    rep.add("44-P5", "exact", "PL1644 and PR1644 commute with every Sigma1644^(AB): chirality is invariant under Spin(4,4)",
        (0..8).all(|a| (0..8).all(|b| pl.comm(&g[a].mul(&g[b])).is_zero())));

    // ---- other important 16x16 matrices of 4+4.nb ------------------------------------
    rep.add("44-S1", "exact", "Cm1644 := gamma1644[0].gamma1644[1].gamma1644[2].gamma1644[3] (the matrix sigma16 of 4+4.nb) = diag(-sigma88, sigma88), symmetric, Cm1644.Cm1644 = 1 (4+4.nb Out[106])",
        gm.sigma16 == IMat::block2(&gm.sigma.neg(), &z8, &z8, &gm.sigma) && gm.sigma16.is_symmetric() && gm.sigma16.mul(&gm.sigma16) == id16);
    rep.add("44-S2", "exact", "Cp1644 := gamma1644[4].gamma1644[5].gamma1644[6].gamma1644[7] (the matrix S44 of 7+1.nb) = diag(sigma88, sigma88), symmetric, Cp1644.Cp1644 = 1, and omega1644 = Cm1644.Cp1644",
        gm.s44 == IMat::block2(&gm.sigma, &z8, &z8, &gm.sigma) && gm.s44.is_symmetric() && gm.s44.mul(&gm.s44) == id16 && gm.sigma16.mul(&gm.s44) == *om);
    rep.add("44-S3", "exact", "Cm1644.gamma1644[A].Cm1644 = -transpose(gamma1644[A]) and Cp1644.gamma1644[A].Cp1644 = +transpose(gamma1644[A]) for A = 0..7",
        (0..8).all(|a| gm.sigma16.mul(&g[a]).mul(&gm.sigma16) == g[a].transpose().neg() && gm.s44.mul(&g[a]).mul(&gm.s44) == g[a].transpose()));
    rep.add("44-S4", "exact", "Cm1644.gamma1644[A] is antisymmetric for A = 0..7 (4+4.nb Out[105]) and Cp1644.gamma1644[A] is symmetric",
        (0..8).all(|a| gm.sigma16.mul(&g[a]).is_antisymmetric() && gm.s44.mul(&g[a]).is_symmetric()));
    rep.add("44-S5", "exact", "gamma1644[4] (the matrix JT4 = T16[4] of 4+4.nb) is a complex structure: gamma1644[4].gamma1644[4] = -1, and transpose(gamma1644[4]).Cm1644.gamma1644[4] = Cm1644",
        g[4].mul(&g[4]) == id16.neg() && g[4].transpose().mul(&gm.sigma16).mul(&g[4]) == gm.sigma16);
}

/// Checks that are specific to the 7+1 matrices of `7+1.nb` and to their PROJECT relabelling.
pub fn verify_71(gm: &Gammas, nb: &[(usize, usize, IMat)], rep: &mut Report) {
    let g = &gm.gamma1671;
    let gnb = &gm.gamma17;
    let id16 = IMat::identity(16);
    let id8 = IMat::identity(8);
    let z8 = IMat::zeros(8, 8);

    // ---- N: identity with the matrices printed in the notebook ---------------------
    let nbm: Vec<&IMat> = nb.iter().filter(|x| x.0 == 204).map(|x| &x.2).collect();
    rep.add("71-N1", "exact", "the eight matrices rebuilt from the definitions equal, entry by entry, the eight matrices Gamma17[0..7] printed by Mathematica in 7+1.nb (cell 204, Out[178])",
        nbm.len() == 8 && (0..8).all(|a| *nbm[a] == gnb[a]));
    rep.add("71-N2", "exact", "PROJECT matrices: gamma1671[A] = Gamma17up[swap04(A)] with swap04 = (0 4) and Gamma17up[A] = eta17^(AA) Gamma17[A] (7+1.nb section 5.3); explicitly gamma1671[4] = Gamma17[0] (time-like), gamma1671[0] = -Gamma17[4], gamma1671[j] = -Gamma17[j] for j = 1, 2, 3, 5, 6, 7.  (gamma1671 is NOT the complex set Gamma71 of 7+1.nb section 4.3, and eta1671 is NOT the matrix eta71 of 7+1.nb)",
        (0..8).all(|a| g[a] == gm.gamma17up[swap04(a)]) && g[4] == gnb[0] && g[0] == gnb[4].neg() && [1usize, 2, 3, 5, 6, 7].iter().all(|&j| g[j] == gnb[j].neg()));
    let mut ok = true;
    for a in 0..8 {
        for b in 0..8 {
            let want = if a == b { 2 * ETA17_NB[a] } else { 0 };
            ok &= gnb[a].anti(&gnb[b]) == id16.scale(want);
        }
    }
    rep.add("71-N3", "exact", "in the labelling of 7+1.nb: Gamma17[A].Gamma17[B] + Gamma17[B].Gamma17[A] = 2 eta17^(AB) 1 with eta17 = diag(+1,-1,-1,-1,-1,-1,-1,-1) (7+1.nb Out[173])", ok);

    // ---- construction from the 4+4 matrices ---------------------------------------------
    let (t16, s44) = (&gm.t16, &gm.s44);
    rep.add("71-B1", "exact", "tauO[A] = tau[A] (A = 1, 2, 3), tauO[A] = tau[A].sigma (A = 4..7): real, antisymmetric, tauO[A].tauO[A] = -1, mutually anticommuting, tauO[1]...tauO[7] = 1 (7+1.nb section 3.1)",
        (1..8).all(|a| gm.tau_o[a].is_antisymmetric() && gm.tau_o[a].mul(&gm.tau_o[a]) == id8.neg())
            && (1..8).all(|a| (1..8).all(|b| a == b || gm.tau_o[a].anti(&gm.tau_o[b]).is_zero()))
            && IMat::product(&gm.tau_o[1..8].iter().collect::<Vec<_>>()) == id8);
    rep.add("71-B2", "exact", "block form: Gamma17[0] = [[0, 1], [1, 0]] and Gamma17[i] = [[tauO[i], 0], [0, tauObar[i]]] with tauObar[i] = transpose(tauO[i]) = -tauO[i], i = 1..7 (7+1.nb Out[175])",
        gnb[0] == IMat::block2(&z8, &id8, &id8, &z8) && (1..8).all(|i| gnb[i] == IMat::block2(&gm.tau_o[i], &z8, &z8, &gm.tau_obar[i])));
    rep.add("71-B3", "exact", "built from the 4+4 matrices only: Gamma17[0] = T16[0], Gamma17[i] = T16[0].T16[i] (i = 1, 2, 3), Gamma17[k] = T16[0].T16[k].S44 (k = 4..7) (7+1.nb Out[176])",
        gnb[0] == t16[0] && (1..=3).all(|i| gnb[i] == t16[0].mul(&t16[i])) && (4..8).all(|k| gnb[k] == t16[0].mul(&t16[k]).mul(s44)));
    // The same relation in the labels of this project (gamma1644[A] = T16[A]; gamma1671[A]
    // as in check 71-N2), with every product reduced to distinct factors.
    let p4 = |i: [usize; 4]| t16[i[0]].mul(&t16[i[1]]).mul(&t16[i[2]]).mul(&t16[i[3]]);
    rep.add("71-B4", "exact", "in the labels of this project: gamma1671[4] = gamma1644[0]; gamma1671[j] = -gamma1644[0].gamma1644[j] (j = 1, 2, 3); gamma1671[0] = +gamma1644[0].gamma1644[5].gamma1644[6].gamma1644[7]; gamma1671[5] = -gamma1644[0].gamma1644[4].gamma1644[6].gamma1644[7]; gamma1671[6] = +gamma1644[0].gamma1644[4].gamma1644[5].gamma1644[7]; gamma1671[7] = -gamma1644[0].gamma1644[4].gamma1644[5].gamma1644[6].  The time-like matrix of 7+1 IS the matrix of the hidden axis of 4+4; the other seven are products of two or of four of the 4+4 matrices and therefore not linear combinations of them",
        g[4] == t16[0] && (1..=3).all(|j| g[j] == t16[0].mul(&t16[j]).neg()) && g[0] == p4([0, 5, 6, 7]) && g[5] == p4([0, 4, 6, 7]).neg()
            && g[6] == p4([0, 4, 5, 7]) && g[7] == p4([0, 4, 5, 6]).neg());

    // ---- V: volume element; no real chirality -----------------------------------------
    let refs: Vec<&IMat> = g.iter().collect();
    let om = IMat::product(&refs); // omega1671 = gamma1671[0] ... gamma1671[7]
    rep.add("71-V1", "exact", "omega1671 := gamma1671[0]...gamma1671[7] = Gamma17[8] = [[0, -1], [1, 0]] in 8x8 blocks (the omega17 of 7+1.nb: the seven sign changes of Gamma17up and the odd permutation swap04 compensate): real, antisymmetric, with square -1",
        om == gm.gamma17_8 && om == IMat::block2(&z8, &id8.neg(), &id8, &z8) && om.is_antisymmetric() && om.mul(&om) == id16.neg());
    rep.add("71-V2", "exact", "omega1671 anticommutes with every gamma1671[A] and commutes with every Sigma1671^(AB)",
        (0..8).all(|a| om.anti(&g[a]).is_zero()) && (0..8).all(|a| (0..8).all(|b| om.comm(&g[a].mul(&g[b])).is_zero())));
    // every real matrix that anticommutes with all eight gammas is a multiple of omega1671:
    // solve X g^A + g^A X = 0 as a linear system for the 256 entries of X
    let mut sys: Vec<Vec<i64>> = Vec::new();
    for a in 0..8 {
        for i in 0..16 {
            for j in 0..16 {
                // (X g + g X)_{ij} = sum_k X_{ik} g_{kj} + g_{ik} X_{kj}
                let mut row = vec![0i64; 256];
                for k in 0..16 {
                    row[i * 16 + k] += g[a].get(k, j);
                    row[k * 16 + j] += g[a].get(i, k);
                }
                sys.push(row);
            }
        }
    }
    rep.add("71-V3", "exact", "the linear system X.gamma1671[A] + gamma1671[A].X = 0 (A = 0..7) for a real 16x16 matrix X has rank 255, and omega1671 solves it (71-V2): the real matrices that anticommute with all eight gamma1671[A] are exactly the multiples of omega1671; since omega1671.omega1671 = -1, no real matrix with square +1 anticommutes with all of them: there is NO real chirality matrix in 7+1",
        rank(&sys) == 255 && (0..8).all(|a| om.anti(&g[a]).is_zero()));

    // ---- P: complex chirality and projectors --------------------------------------------
    // chi1671 := i omega1671 = i Gamma17[8]  (= chi17 of 7+1.nb);  2 PL1671 = 1 - chi1671, 2 PR1671 = 1 + chi1671
    let chi = GMat::i_times(&om);
    let one = GMat::from_real(&id16);
    let two_pl = one.sub(&chi);
    let two_pr = one.add(&chi);
    rep.add("71-P1", "exact", "chi1671 := i omega1671 = i Gamma17[8] (= chi17 of 7+1.nb) is Hermitian, chi1671.chi1671 = 1, anticommutes with every gamma1671[A]",
        chi == GMat::i_times(&gm.gamma17_8) && chi.dagger() == chi && chi.mul(&chi) == one
            && (0..8).all(|a| { let ga = GMat::from_real(&g[a]); chi.mul(&ga).add(&ga.mul(&chi)).is_zero() }));
    rep.add("71-P2", "exact", "PL1671 := (1 - chi1671)/2 and PR1671 := (1 + chi1671)/2: PL1671 + PR1671 = 1, PL1671.PL1671 = PL1671, PR1671.PR1671 = PR1671, PL1671.PR1671 = 0, Hermitian, trace 8 each; they are complex (not real) matrices",
        two_pl.add(&two_pr) == one.scale(2) && two_pl.mul(&two_pl) == two_pl.scale(2) && two_pr.mul(&two_pr) == two_pr.scale(2)
            && two_pl.mul(&two_pr).is_zero() && two_pl.dagger() == two_pl && two_pr.dagger() == two_pr
            && two_pl.re.trace() == 16 && two_pr.re.trace() == 16 && two_pl.im.trace() == 0 && !two_pl.is_real() && !two_pr.is_real());
    rep.add("71-P3", "exact", "gamma1671[A].PL1671 = PR1671.gamma1671[A] for A = 0..7, and PL1671, PR1671 commute with every Sigma1671^(AB)",
        (0..8).all(|a| { let ga = GMat::from_real(&g[a]); ga.mul(&two_pl) == two_pr.mul(&ga) })
            && (0..8).all(|a| (0..8).all(|b| { let s = GMat::from_real(&g[a].mul(&g[b])); s.mul(&two_pl) == two_pl.mul(&s) })));
    // block form of the projectors: chi1671 = [[0, -i], [i, 0]], 2 PL1671 = [[1, i], [-i, 1]], 2 PR1671 = [[1, -i], [i, 1]]
    rep.add("71-P4", "exact", "in 8x8 blocks chi1671 = [[0, -i], [i, 0]], 2 PL1671 = [[1, i], [-i, 1]] and 2 PR1671 = [[1, -i], [i, 1]]; these are chi17, PL17 and PR17 of 7+1.nb (sections 4.6 and 7.10)",
        chi.re.is_zero() && chi.im == IMat::block2(&z8, &id8.neg(), &id8, &z8)
            && two_pl.re == id16 && two_pl.im == IMat::block2(&z8, &id8, &id8.neg(), &z8) && two_pr.im == IMat::block2(&z8, &id8.neg(), &id8, &z8));

    // relation to the real projectors of 4+4.nb.  7+1.nb (section 4.3) maps the real matrices to the
    // complex set Gamma71 = S.(i Gamma17).S^(-1) with the unitary S = (1 - i Gamma17[0])/sqrt(2); then
    // S.Gamma17[8].S^(-1) = Gamma71[8] and, with the notebook's chi1671 := -i Gamma71[8] = T16[8],
    // S.chi17.S^(-1) = S.(i Gamma17[8]).S^(-1) = -T16[8].  Checked as  s.chi = -T16[8].s  with s = 1 - i Gamma17[0].
    let smat = GMat { re: id16.clone(), im: gnb[0].neg() };
    let t8 = GMat::from_real(&gm.t16_8);
    rep.add("71-P5", "exact", "with s := 1 - i Gamma17[0] (s.s^dagger = 2, so Uc1671 := s/sqrt(2) is unitary): s.chi1671 = -T16[8].s, i.e. Uc1671.chi1671.inverse(Uc1671) = -omega1644.  Hence Uc1671.PL1671.inverse(Uc1671) = PR1644 and Uc1671.PR1671.inverse(Uc1671) = PL1644: the complex chiral projectors of 7+1 are unitarily equivalent to the real projectors of 4+4.nb, with the names L and R exchanged.  (7+1.nb defines chi17 = +i Gamma17[8] for its real matrices and chi71 = -i Gamma71[8] for its complex matrices Gamma71; the PROJECT matrix chi1671 equals chi17, NOT chi71)",
        smat.mul(&smat.dagger()) == one.scale(2) && smat.mul(&chi) == t8.mul(&smat).scale(-1));

    // ---- adjoint, charge conjugation ------------------------------------------------------
    rep.add("71-S1", "exact", "Dirac adjoint: transpose(gamma1671[A]) = gamma1671[4].gamma1671[A].gamma1671[4] for A = 0..7 (gamma1671[4] is the time-like matrix; it is symmetric and squares to +1)",
        g[4].is_symmetric() && g[4].mul(&g[4]) == id16 && (0..8).all(|a| g[a].transpose() == g[4].mul(&g[a]).mul(&g[4])));
}

/// Transposition symmetry of the 256 matrices C.g[A1]...g[Ar] (A1 < ... < Ar,
/// r = 0..8) for the two candidate "charge conjugation" matrices C of a set g of
/// real Dirac matrices.
///
/// A bilinear psi^T (C g[A1]...g[Ar]) psi of ONE anticommuting field psi vanishes
/// identically when the matrix C g[A1]...g[Ar] is symmetric.  The function returns,
/// for each rank r = 0..8, the sign s(r) = +1 (all matrices of that rank symmetric)
/// or -1 (all antisymmetric), for C = c_minus and for C = c_plus, and reports
/// whether the sign is uniform within every rank.
pub fn pair_symmetry_table(g: &[IMat], c_minus: &IMat, c_plus: &IMat) -> (Vec<i64>, Vec<i64>, bool) {
    let basis = clifford_basis(g);
    let mut sm = vec![0i64; 9];
    let mut sp = vec![0i64; 9];
    let mut uniform = true;
    for (mask, m) in &basis {
        let r = rank_of(*mask);
        for (c, tab) in [(c_minus, &mut sm), (c_plus, &mut sp)] {
            let x = c.mul(m);
            let s = if x.is_symmetric() { 1 } else if x.is_antisymmetric() { -1 } else { 0 };
            if s == 0 || (tab[r] != 0 && tab[r] != s) {
                uniform = false;
            }
            tab[r] = s;
        }
    }
    (sm, sp, uniform)
}

/// Checks on the bilinear forms that decide which fermion pairs can exist.
pub fn verify_pairs(gm: &Gammas, rep: &mut Report) -> ((Vec<i64>, Vec<i64>), (Vec<i64>, Vec<i64>)) {
    let binom = [1usize, 8, 28, 56, 70, 56, 28, 8, 1];
    // 4+4:  Cm1644 = sigma16 of 4+4.nb (C g C^-1 = -g^T),  Cp1644 = S44 of 7+1.nb (C g C^-1 = +g^T)
    let (sm44, sp44, u44) = pair_symmetry_table(&gm.gamma1644, &gm.sigma16, &gm.s44);
    let n_anti = |s: &Vec<i64>| (0..9).filter(|&r| s[r] < 0).map(|r| binom[r]).sum::<usize>();
    rep.add("44-K1", "exact", &format!("4+4: the matrices Cm1644.gamma1644[A1]...gamma1644[Ar] (A1 < ... < Ar) of rank r = 0..8 are all symmetric (+) or all antisymmetric (-): signs {:?}; the matrices Cp1644.gamma1644[A1]...gamma1644[Ar]: signs {:?}; in both cases exactly 120 of the 256 matrices are antisymmetric", sm44, sp44),
        u44 && n_anti(&sm44) == 120 && n_anti(&sp44) == 120);
    rep.add("44-K2", "exact", "4+4: the two Lorentz-scalar pair matrices Cm1644 (rank 0) and Cm1644.omega1644 (rank 8; equal to Cp1644 up to sign) are SYMMETRIC: the Lorentz-scalar bilinears dirac1644^T Cm1644 dirac1644 and dirac1644^T Cp1644 dirac1644 of one anticommuting 16-component field vanish identically",
        sm44[0] == 1 && sm44[8] == 1 && sp44[0] == 1 && sp44[8] == 1);
    // 7+1:  Cp1671 = gamma1671[4] (the time-like matrix),  Cm1671 = gamma1671[4].omega1671
    let g = &gm.gamma1671;
    let refs: Vec<&IMat> = g.iter().collect();
    let om = IMat::product(&refs);
    let c_plus = g[4].clone();
    let c_minus = g[4].mul(&om);
    let ok_c = (0..8).all(|a| c_plus.mul(&g[a]) == g[a].transpose().mul(&c_plus) && c_minus.mul(&g[a]) == g[a].transpose().neg().mul(&c_minus));
    let (sm71, sp71, u71) = pair_symmetry_table(g, &c_minus, &c_plus);
    rep.add("71-K0", "exact", "7+1: Cp1671 := gamma1671[4] satisfies Cp1671.gamma1671[A] = +transpose(gamma1671[A]).Cp1671 and Cm1671 := gamma1671[4].omega1671 satisfies Cm1671.gamma1671[A] = -transpose(gamma1671[A]).Cm1671; both are symmetric matrices",
        ok_c && c_plus.is_symmetric() && c_minus.is_symmetric());
    rep.add("71-K1", "exact", &format!("7+1: signs (+ symmetric, - antisymmetric) of the matrices Cm1671.gamma1671[A1]...gamma1671[Ar] (A1 < ... < Ar) by rank r = 0..8: {:?}; of Cp1671.gamma1671[A1]...gamma1671[Ar]: {:?}; exactly 120 of the 256 matrices are antisymmetric in both cases", sm71, sp71),
        u71 && n_anti(&sm71) == 120 && n_anti(&sp71) == 120);
    rep.add("71-K2", "exact", "7+1: the two Lorentz-scalar pair matrices Cm1671 (rank 0) and Cm1671.omega1671 = -Cp1671 (rank 8) are SYMMETRIC: the Lorentz-scalar bilinears dirac1671^T Cm1671 dirac1671 and dirac1671^T Cp1671 dirac1671 of one anticommuting 16-component field vanish identically",
        sm71[0] == 1 && sm71[8] == 1 && sp71[0] == 1 && sp71[8] == 1);

    // ---- K3: the hidden-axis component of the vector pair field is a scalar of the
    //      Lorentz group of the seven coordinates other than x0 ---------------------------
    // dirac -> S dirac changes DeltaV^0 = dirac^T (C_minus g[0]) dirac by
    //   dirac^T ( transpose(N) C g[0] + C g[0] N ) dirac  for the generator N = g[A] g[B] = 2 Sigma^(AB).
    let k3 = |g: &[IMat], cm: &IMat| -> (bool, bool) {
        let x = cm.mul(&g[0]);
        let var = |a: usize, b: usize| { let n = g[a].mul(&g[b]); n.transpose().mul(&x).add(&x.mul(&n)) };
        let inv = (1..8).all(|a| (1..8).all(|b| a == b || var(a, b).is_zero()));
        // a rotation or boost that involves the axis x0 turns DeltaV^0 into DeltaV^B: the variation is not zero
        let moved = (1..8).all(|b| !var(0, b).is_zero());
        (inv, moved)
    };
    let (inv44, mov44) = k3(&gm.gamma1644, &gm.sigma16);
    rep.add("44-K3", "exact", "4+4: for all A != B in 1..7, transpose(Sigma1644^(AB)).Cm1644.gamma1644[0] + Cm1644.gamma1644[0].Sigma1644^(AB) = 0, while the same expression with A = 0 is not zero: the bilinear DeltaV1644^0 = dirac1644^T Cm1644 gamma1644^0 dirac1644 is INVARIANT under the 21 generators of the pseudo-rotations of the seven coordinates x1..x7 (the group Spin(3,4), which contains the Lorentz group of x1 x2 x3 x4), and only the 7 generators that involve the hidden axis x0 change it",
        inv44 && mov44);
    let (inv71, mov71) = k3(g, &c_minus);
    rep.add("71-K3", "exact", "7+1: for all A != B in 1..7, transpose(Sigma1671^(AB)).Cm1671.gamma1671[0] + Cm1671.gamma1671[0].Sigma1671^(AB) = 0, while the same expression with A = 0 is not zero: the bilinear DeltaV1671^0 = dirac1671^T Cm1671 gamma1671^0 dirac1671 is INVARIANT under the 21 generators of the Lorentz group of the seven coordinates x1..x7 (the group Spin(6,1), which contains the Lorentz group of x1 x2 x3 x4), and only the 7 generators that involve the hidden axis x0 change it",
        inv71 && mov71);
    ((sm44, sp44), (sm71, sp71))
}

/// Run every matrix check.  `nb44` and `nb71` are the paths of the files written
/// by `tools/extract_nb_matrices.py`.
pub fn run_all(nb44: &str, nb71: &str) -> (Gammas, Report) {
    let gm = Gammas::build();
    let mut rep = Report::new();
    let m44 = read_notebook_matrices(nb44);
    let m71 = read_notebook_matrices(nb71);
    verify_common("44", "gamma1644", &gm.gamma1644, &ETA1644, &mut rep);
    verify_44(&gm, &m44, &mut rep);
    verify_common("71", "gamma1671", &gm.gamma1671, &ETA1671, &mut rep);
    verify_71(&gm, &m71, &mut rep);
    verify_pairs(&gm, &mut rep);
    (gm, rep)
}
