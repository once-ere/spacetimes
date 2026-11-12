//! Which pairs of two identical particles of the 16-component field can exist?
//!
//! A pair of two particles at rest, in a state of zero orbital angular
//! momentum (an "s wave"), is described by an amplitude Phi_{sr}, where s and r
//! label the 8 internal ("spin") states of one particle at rest.  Because the
//! particles are identical fermions and the orbital state is symmetric, the
//! Pauli principle requires Phi_{sr} = -Phi_{rs}: there are 8*7/2 = 28 such
//! pair states.  This module decomposes these 28 states into multiplets of the
//! symmetry group that is left when the particle is at rest:
//!
//! * 7+1:  rotations of the 7 space coordinates, SO(7); and its subgroup
//!   SO(3) x SO(4) (ordinary space x1 x2 x3  times  extra space x0 x5 x6 x7);
//! * 4+4 in the sector of fields that do not depend on the extra times
//!   x5 x6 x7:  SO(4) (space x0 x1 x2 x3) x SO(3)_T (extra times) x {W = +-1}.
//!
//! The decomposition is done numerically: the quadratic Casimir operator of
//! every group factor is a Hermitian 28x28 matrix; the Casimir operators
//! commute, and their joint eigenvalues label the multiplets.  All eigenvalues
//! are small rational numbers (j(j+1) for SU(2)), recovered to 1e-9.
//!
//! Exact (integer) statements about explicit pair matrices are added for 7+1.

use crate::complex::C64;
use crate::gamma::{clifford_basis, swap04, rank_of, Gammas};
use crate::imat::{rank, IMat};
use crate::linalg::{eigh, CMat};
use crate::verify::Report;

/// Complex n x n matrix from an integer matrix.
fn cm(m: &IMat) -> CMat {
    let mut r = CMat::zeros(m.n);
    for i in 0..m.n {
        for j in 0..m.n {
            r.set(i, j, C64::new(m.get(i, j) as f64, 0.0));
        }
    }
    r
}

/// U^dagger G U for a 16x16 matrix G and an orthonormal basis U (8 columns of length 16).
fn restrict(g: &CMat, u: &[Vec<C64>]) -> CMat {
    let k = u.len();
    let mut r = CMat::zeros(k);
    for a in 0..k {
        for b in 0..k {
            let mut s = C64::new(0.0, 0.0);
            for i in 0..16 {
                for j in 0..16 {
                    s += u[a][i].conj() * g.get(i, j) * u[b][j];
                }
            }
            r.set(a, b, s);
        }
    }
    r
}

/// U^T X U (no complex conjugation): the two-particle content of the bilinear psi^T X psi.
fn restrict_bilinear(x: &CMat, u: &[Vec<C64>]) -> CMat {
    let k = u.len();
    let mut r = CMat::zeros(k);
    for a in 0..k {
        for b in 0..k {
            let mut s = C64::new(0.0, 0.0);
            for i in 0..16 {
                for j in 0..16 {
                    s += u[a][i] * x.get(i, j) * u[b][j];
                }
            }
            r.set(a, b, s);
        }
    }
    r
}

/// The index pairs (s, r), s < r, that label the 28 antisymmetric pair states.
fn pair_index(k: usize) -> Vec<(usize, usize)> {
    (0..k).flat_map(|s| ((s + 1)..k).map(move |r| (s, r))).collect()
}

/// The linear map Phi -> G Phi + Phi G^T on antisymmetric k x k matrices, as a
/// matrix in the orthonormal basis (E_sr - E_rs)/sqrt(2).
/// It is the action of the one-particle generator G on a two-particle amplitude.
fn lift(g: &CMat, k: usize) -> CMat {
    let idx = pair_index(k);
    let n = idx.len();
    let mut out = CMat::zeros(n);
    for (b, &(s, r)) in idx.iter().enumerate() {
        // image of the basis element E_sr - E_rs (norm sqrt 2): X = G Phi + Phi G^T
        // X_{ij} = G_{is} delta_{jr} - G_{ir} delta_{js} + delta_{is} G_{jr} - delta_{ir} G_{js}
        for (a, &(i, j)) in idx.iter().enumerate() {
            let mut x = C64::new(0.0, 0.0);
            if j == r { x += g.get(i, s); }
            if j == s { x = x - g.get(i, r); }
            if i == s { x += g.get(j, r); }
            if i == r { x = x - g.get(j, s); }
            out.set(a, b, x); // <a|L|b> with normalized basis: (X_ij - X_ji)/2 = X_ij for antisymmetric X
        }
    }
    out
}

/// The map Phi -> P Phi P^T (action of a discrete symmetry P on a pair amplitude).
fn lift_parity(p: &CMat, k: usize) -> CMat {
    let idx = pair_index(k);
    let n = idx.len();
    let mut out = CMat::zeros(n);
    for (b, &(s, r)) in idx.iter().enumerate() {
        for (a, &(i, j)) in idx.iter().enumerate() {
            out.set(a, b, p.get(i, s) * p.get(j, r) - p.get(i, r) * p.get(j, s));
        }
    }
    out
}

/// Quadratic Casimir operator  C = - sum_k L_k L_k  of a set of anti-Hermitian generators.
fn casimir(gens: &[CMat]) -> CMat {
    let n = gens[0].n;
    let mut c = CMat::zeros(n);
    for g in gens {
        c = c.sub(&g.mul(g));
    }
    c
}

/// One multiplet type found in a decomposition: the eigenvalue of each operator, and how many states.
pub struct Multiplet {
    pub values: Vec<f64>,
    pub count: usize,
}

/// Joint eigenvalues of commuting Hermitian operators, with multiplicities.
/// Returns the multiplets (sorted) and the largest residual |O v - <O> v|.
fn joint_spectrum(ops: &[CMat]) -> (Vec<Multiplet>, f64, Vec<Vec<C64>>, Vec<Vec<f64>>) {
    let n = ops[0].n;
    // a generic real combination separates all joint eigenspaces
    let weights = [1.0, 1.3717, 2.2361, 3.9051, 5.3113, 7.0711, 9.1104];
    let mut m = CMat::zeros(n);
    for (i, o) in ops.iter().enumerate() {
        m = m.add(&o.scale(weights[i]));
    }
    let (_w, v) = eigh(&m);
    let mut worst = 0.0f64;
    let mut tuples: Vec<Vec<f64>> = Vec::new();
    let mut vecs: Vec<Vec<C64>> = Vec::new();
    for col in 0..n {
        let vec: Vec<C64> = (0..n).map(|i| v.get(i, col)).collect();
        let mut tup = Vec::new();
        for o in ops {
            // expectation value and residual
            let mut ov = vec![C64::new(0.0, 0.0); n];
            for i in 0..n {
                for j in 0..n {
                    ov[i] += o.get(i, j) * vec[j];
                }
            }
            let ev: f64 = (0..n).map(|i| (vec[i].conj() * ov[i]).re).sum();
            let res: f64 = (0..n).map(|i| (ov[i] - vec[i] * ev).norm_sqr()).sum::<f64>().sqrt();
            worst = worst.max(res);
            tup.push((ev * 4.0).round() / 4.0); // Casimir values are multiples of 1/4
        }
        tuples.push(tup);
        vecs.push(vec);
    }
    let mut sorted = tuples.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mut out: Vec<Multiplet> = Vec::new();
    for t in sorted {
        if let Some(last) = out.last_mut() {
            if last.values == t {
                last.count += 1;
                continue;
            }
        }
        out.push(Multiplet { values: t, count: 1 });
    }
    (out, worst, vecs, tuples)
}

/// Label of an SU(2) Casimir value j(j+1): the dimension 2j+1 of the multiplet.
pub fn su2_dim(c: f64) -> usize {
    ((1.0 + 4.0 * c).sqrt()).round() as usize
}

fn fmt_multiplets(ms: &[Multiplet], names: &[&str]) -> String {
    let mut s = String::new();
    for m in ms {
        // "+ 0.0" turns a floating-point "-0" into "0" in the printed label
        let lab: Vec<String> = names.iter().zip(&m.values).map(|(n, v)| format!("{}={}", n, v + 0.0)).collect();
        s.push_str(&format!("[{} : {} states] ", lab.join(", "), m.count));
    }
    s
}

/// Result of the pair analysis, for the documents.
pub struct PairAnalysis {
    pub lines: Vec<String>,
    pub csv_rows: Vec<Vec<String>>,
}

/// Classification for the 7+1 spacetime.
pub fn analyse_71(gm: &Gammas, rep: &mut Report) -> PairAnalysis {
    let g = &gm.gamma1671;
    let mut out = PairAnalysis { lines: Vec::new(), csv_rows: Vec::new() };
    let space: [usize; 7] = [0, 1, 2, 3, 5, 6, 7];
    // one particle at rest: beta u = +u, beta = gamma^4 = [[0,1],[1,0]]:  u_s = (e_s; e_s)/sqrt(2)
    let h = 1.0 / 2f64.sqrt();
    let u: Vec<Vec<C64>> = (0..8).map(|s| (0..16).map(|i| C64::new(if i == s || i == s + 8 { h } else { 0.0 }, 0.0)).collect()).collect();
    let sig = |a: usize, b: usize| -> CMat { restrict(&cm(&g[a].mul(&g[b])).scale(0.5), &u) };
    // rotation generators on the one-particle space
    let jgen = vec![sig(2, 3), sig(3, 1), sig(1, 2)];
    let lgen = vec![sig(6, 7), sig(7, 5), sig(5, 6)];
    let kgen = vec![sig(5, 0), sig(6, 0), sig(7, 0)];
    let agen: Vec<CMat> = (0..3).map(|a| lgen[a].add(&kgen[a]).scale(0.5)).collect();
    let bgen: Vec<CMat> = (0..3).map(|a| lgen[a].sub(&kgen[a]).scale(0.5)).collect();
    let mut all21 = Vec::new();
    for (ii, &i) in space.iter().enumerate() {
        for &j in &space[(ii + 1)..] {
            all21.push(sig(i, j));
        }
    }
    // the two SU(2) factors of SO(4) commute, and each closes into su(2)
    let mut comm_ab = 0.0f64;
    for a in &agen {
        for b in &bgen {
            comm_ab = comm_ab.max(a.mul(b).sub(&b.mul(a)).fro());
        }
    }
    // one-particle content
    let ops1 = vec![casimir(&jgen), casimir(&agen), casimir(&bgen), casimir(&all21)];
    let (m1, res1, _, _) = joint_spectrum(&ops1);
    let s1 = fmt_multiplets(&m1, &["J(J+1)", "A(A+1)", "B(B+1)", "C_SO(7)"]);
    out.lines.push(format!("7+1 one particle at rest (8 states): {}", s1));
    let ok1 = m1.len() == 2 && m1.iter().all(|m| m.count == 4 && m.values[0] == 0.75 && (m.values[3] - 5.25).abs() < 1e-9)
        && m1.iter().any(|m| m.values[1] == 0.75 && m.values[2] == 0.0) && m1.iter().any(|m| m.values[1] == 0.0 && m.values[2] == 0.75);
    rep.add("71-Q1", "numerical", &format!("7+1, one particle at rest: the 8 spin states form the spinor of SO(7) (Casimir 21/4) and split under SO(3) x SU(2)_A x SU(2)_B (ordinary rotations x rotations of the extra space x0 x5 x6 x7) as (2; 2, 1) + (2; 1, 2); [A_a, B_b] = 0 to {:.0e}, residual {:.0e}", comm_ab, res1),
        ok1 && comm_ab < 1e-12 && res1 < 1e-9);

    // two particles: the 28 antisymmetric s-wave pair states
    let lj: Vec<CMat> = jgen.iter().map(|x| lift(x, 8)).collect();
    let la: Vec<CMat> = agen.iter().map(|x| lift(x, 8)).collect();
    let lb: Vec<CMat> = bgen.iter().map(|x| lift(x, 8)).collect();
    let l21: Vec<CMat> = all21.iter().map(|x| lift(x, 8)).collect();
    let ops2 = vec![casimir(&lj), casimir(&la), casimir(&lb), casimir(&l21)];
    let (m2, res2, _, _) = joint_spectrum(&ops2);
    out.lines.push(format!("7+1 two particles, s wave (28 states): {}", fmt_multiplets(&m2, &["J(J+1)", "A(A+1)", "B(B+1)", "C_SO(7)"])));
    for m in &m2 {
        out.csv_rows.push(vec!["7+1".into(), format!("{}", su2_dim(m.values[0])), format!("{}", su2_dim(m.values[1])), format!("{}", su2_dim(m.values[2])),
            format!("{}", if m.values[3] == 6.0 { "7 (vector)" } else if m.values[3] == 10.0 { "21 (adjoint)" } else { "?" }), format!("{}", m.count)]);
    }
    let find = |j: f64, a: f64, b: f64, c: f64| m2.iter().find(|m| m.values == vec![j, a, b, c]).map(|m| m.count).unwrap_or(0);
    let ok2 = find(0.0, 0.75, 0.75, 6.0) == 4 && find(2.0, 0.0, 0.0, 6.0) == 3 && find(0.0, 2.0, 0.0, 10.0) == 3 && find(0.0, 0.0, 2.0, 10.0) == 3
        && find(2.0, 0.0, 0.0, 10.0) == 3 && find(2.0, 0.75, 0.75, 10.0) == 12 && m2.iter().map(|m| m.count).sum::<usize>() == 28;
    rep.add("71-Q2", "numerical", &format!("7+1, two identical particles in an s wave (28 states allowed by the Pauli principle): under SO(7) they form 7 + 21 (Casimir 6 and 10); there is NO SO(7) singlet.  Under SO(3) x SU(2)_A x SU(2)_B:  7 = (1; 2, 2) + (3; 1, 1),  21 = (1; 3, 1) + (1; 1, 3) + (3; 1, 1) + (3; 2, 2).  The pairs of ordinary spin 0 are the 10 states (1; 2, 2) + (1; 3, 1) + (1; 1, 3) (residual {:.0e})", res2),
        ok2 && res2 < 1e-9);

    // ---- exact statements with the integer matrices tau1671^j = tauO[swap04(j)] ------------------
    // On the particle space the generator Sigma^{ij} acts as (1/2) tau1671^i tau1671^j, and a pair amplitude
    // transforms by the commutator.  (gamma^i gamma^j = diag(tau1671^i tau1671^j, tau1671^i tau1671^j) for space indices.)
    let tau1671: Vec<IMat> = (0..8).map(|a| gm.tau_o[swap04(a)].clone()).collect();
    let id8 = IMat::identity(8);
    let z8 = IMat::zeros(8, 8);
    let ok_blocks = space.iter().all(|&i| space.iter().all(|&j| i == j || g[i].mul(&g[j]) == IMat::block2(&tau1671[i].mul(&tau1671[j]), &z8, &z8, &tau1671[i].mul(&tau1671[j]))));
    rep.add("71-Q3", "exact", "7+1: for space indices i != j, gamma1671[i].gamma1671[j] = diag(tau1671^i tau1671^j, tau1671^i tau1671^j) in 8x8 blocks, so on the particles at rest, u = (e; e)/sqrt(2), the rotation generator Sigma1671^(ij) acts as the real antisymmetric 8x8 matrix (1/2) tau1671^i tau1671^j", ok_blocks);
    // the 7 matrices tau1671^j and the 21 matrices tau1671^i tau1671^j: 28 independent antisymmetric matrices
    let mut basis: Vec<IMat> = space.iter().map(|&j| tau1671[j].clone()).collect();
    let mut planes: Vec<IMat> = Vec::new();
    for (ii, &i) in space.iter().enumerate() {
        for &j in &space[(ii + 1)..] {
            planes.push(tau1671[i].mul(&tau1671[j]));
        }
    }
    basis.extend(planes.iter().cloned());
    let rows: Vec<Vec<i64>> = basis.iter().map(|m| m.flatten()).collect();
    rep.add("71-Q4", "exact", "7+1: the 7 matrices tau1671^j and the 21 matrices tau1671^i tau1671^j (i < j; space indices) are antisymmetric and linearly independent: together they span all 28 antisymmetric 8x8 matrices, i.e. all s-wave pair amplitudes",
        basis.iter().all(|m| m.is_antisymmetric()) && rank(&rows) == 28);
    // so(7) Casimir on the two families:  sum_{i<j} [N_ij, [N_ij, Phi]] = -4 C Phi  with N_ij = tau1671^i tau1671^j = 2 Sigma^{ij}
    let cas = |phi: &IMat| -> IMat {
        let mut s = IMat::zeros(8, 8);
        for n in &planes {
            s = s.add(&n.comm(&n.comm(phi)));
        }
        s
    };
    rep.add("71-Q5", "exact", "7+1: the quadratic Casimir operator of SO(7), Cas7(Phi) := -sum_(i<j) [Sigma1671^(ij), [Sigma1671^(ij), Phi]], gives Cas7(Phi) = 6 Phi for every Phi = tau1671^j (the vector, 7) and Cas7(Phi) = 10 Phi for every Phi = tau1671^i tau1671^j (the adjoint, 21)",
        space.iter().all(|&j| cas(&tau1671[j]) == tau1671[j].scale(-24)) && planes.iter().all(|n| cas(n) == n.scale(-40)));
    // no invariant antisymmetric matrix: the only 8x8 matrices commuting with all tau1671^i tau1671^j are multiples of 1
    let mut sys: Vec<Vec<i64>> = Vec::new();
    for n in &planes {
        for i in 0..8 {
            for j in 0..8 {
                let mut row = vec![0i64; 64];
                for k in 0..8 {
                    row[k * 8 + j] += n.get(i, k); // (N X)_{ij}
                    row[i * 8 + k] -= n.get(k, j); // -(X N)_{ij}
                }
                sys.push(row);
            }
        }
    }
    rep.add("71-Q6", "exact", "7+1: the linear system [tau1671^i tau1671^j, X] = 0 (all 21 planes) for an 8x8 matrix X has rank 63 and is solved by X = 1: the only rotation-invariant pair matrix is the unit matrix, which is SYMMETRIC and therefore excluded by the Pauli principle.  Two identical particles of the field cannot form an s-wave pair that is a singlet under all rotations of the seven space dimensions",
        rank(&sys) == 63 && planes.iter().all(|n| n.comm(&id8).is_zero()));
    // the ten pair matrices of ordinary spin 0
    let extra: [usize; 4] = [0, 5, 6, 7];
    let mut ten: Vec<IMat> = extra.iter().map(|&j| tau1671[j].clone()).collect();
    for (ii, &a) in extra.iter().enumerate() {
        for &b in &extra[(ii + 1)..] {
            ten.push(tau1671[a].mul(&tau1671[b]));
        }
    }
    let jplanes = [tau1671[2].mul(&tau1671[3]), tau1671[3].mul(&tau1671[1]), tau1671[1].mul(&tau1671[2])];
    // system: antisymmetric Phi commuting with the three ordinary rotation generators
    let idx = pair_index(8);
    let mut sys: Vec<Vec<i64>> = Vec::new();
    for n in &jplanes {
        for i in 0..8 {
            for j in 0..8 {
                let mut row = vec![0i64; 28];
                for (c, &(s, r)) in idx.iter().enumerate() {
                    // basis element E_sr - E_rs; ([N, Phi])_{ij}
                    let mut x = 0i64;
                    if j == r { x += n.get(i, s); }
                    if j == s { x -= n.get(i, r); }
                    if i == s { x -= n.get(r, j); }
                    if i == r { x += n.get(s, j); }
                    row[c] = x;
                }
                sys.push(row);
            }
        }
    }
    let rows: Vec<Vec<i64>> = ten.iter().map(|m| m.flatten()).collect();
    rep.add("71-Q7", "exact", "7+1: the ten antisymmetric matrices tau1671^0, tau1671^5, tau1671^6, tau1671^7 and tau1671^a tau1671^b (a < b in {0, 5, 6, 7}) are linearly independent and commute with the generators tau1671^2 tau1671^3, tau1671^3 tau1671^1, tau1671^1 tau1671^2 of ordinary rotations; the system [tau1671^i tau1671^j, Phi] = 0 (ordinary rotations) on antisymmetric Phi has rank 18 = 28 - 10: these ten matrices are ALL the s-wave pairs of ordinary spin 0",
        rank(&rows) == 10 && ten.iter().all(|m| m.is_antisymmetric() && jplanes.iter().all(|n| n.comm(m).is_zero())) && rank(&sys) == 18);
    rep.add("71-Q8", "exact", "7+1: each of tau1671^j (any space index j) and tau1671^i tau1671^j (i != j) is an orthogonal matrix: Phi.transpose(Phi) = 1.  A condensate whose pair matrix is proportional to tau1671^j or to tau1671^i tau1671^j therefore gives the SAME gap to all eight spin states (a 'unitary' state)",
        space.iter().all(|&j| tau1671[j].mul(&tau1671[j].transpose()) == id8) && planes.iter().all(|n| n.mul(&n.transpose()) == id8));

    // ---- which field bilinears psi^T X psi contain these pairs? ---------------------------
    // two-particle content of psi^T X psi at zero momentum: u_s^T X u_r = (X11 + X12 + X21 + X22)_{sr} / 2
    let refs: Vec<&IMat> = g.iter().collect();
    let om = IMat::product(&refs);
    let c_plus = g[4].clone();
    let c_minus = g[4].mul(&om);
    let content = |x: &IMat| -> IMat { x.block(0, 0, 8, 8).add(&x.block(0, 8, 8, 8)).add(&x.block(8, 0, 8, 8)).add(&x.block(8, 8, 8, 8)) };
    // u = (e; e)/sqrt(2):  u_s^T X u_r = content(X)_{sr} / 2
    let ok_v = space.iter().all(|&j| content(&c_minus.mul(&g[j])) == tau1671[j].scale(-2)) && content(&c_minus.mul(&g[4])).is_zero();
    let ok_t = space.iter().all(|&i| space.iter().all(|&j| i == j || content(&c_plus.mul(&g[i]).mul(&g[j])) == tau1671[i].mul(&tau1671[j]).scale(2)));
    rep.add("71-Q9", "exact", "7+1: two-particle content of the field bilinears, u_s^T X u_r for particles at rest: X = Cm1671.gamma1671[j] gives -tau1671^j (space index j) and X = Cm1671.gamma1671[4] gives 0; X = Cp1671.gamma1671[i].gamma1671[j] gives tau1671^i tau1671^j.  Hence the vector pair field is DeltaV1671^j := dirac1671^T Cm1671 gamma1671^j dirac1671 (its time component contains no pair of two particles) and the adjoint pair field is DeltaT1671^(ij) := dirac1671^T Cp1671 gamma1671^i gamma1671^j dirac1671",
        ok_v && ok_t);

    // ---- the unique singlet of SO(3) (x1 x2 x3) x SO(3)' (x5 x6 x7) -----------------------
    // In stage 2 of the author's programme the hidden coordinate x0 is singled out, and the
    // rotation symmetry of the extra space x0 x5 x6 x7 is reduced from SO(4) to the rotations
    // SO(3)' of x5 x6 x7.  Which s-wave pairs are invariant under SO(3) x SO(3)'?
    let six = [tau1671[2].mul(&tau1671[3]), tau1671[3].mul(&tau1671[1]), tau1671[1].mul(&tau1671[2]),
               tau1671[6].mul(&tau1671[7]), tau1671[7].mul(&tau1671[5]), tau1671[5].mul(&tau1671[6])];
    let mut sys: Vec<Vec<i64>> = Vec::new();
    for n in &six {
        for i in 0..8 {
            for j in 0..8 {
                let mut row = vec![0i64; 28];
                for (c, &(s, r)) in idx.iter().enumerate() {
                    // basis element E_sr - E_rs; ([N, Phi])_{ij}
                    let mut x = 0i64;
                    if j == r { x += n.get(i, s); }
                    if j == s { x -= n.get(i, r); }
                    if i == s { x -= n.get(r, j); }
                    if i == r { x += n.get(s, j); }
                    row[c] = x;
                }
                sys.push(row);
            }
        }
    }
    rep.add("71-Q10", "exact", "7+1: the system [tau1671^i tau1671^j, Phi] = 0 for the six rotation planes (1,2), (1,3), (2,3) of ordinary space and (5,6), (5,7), (6,7) of the extra space, on antisymmetric Phi, has rank 27 = 28 - 1, and Phi = tau1671^0 solves it: EXACTLY ONE s-wave pair is a singlet both of the ordinary rotations and of the rotations of x5 x6 x7.  Its pair matrix is tau1671^0, the two-particle content of DeltaV1671^0 = dirac1671^T Cm1671 gamma1671^0 dirac1671, the component of the vector pair field along the hidden space dimension x0 (the same result as in 4+4, check 44-Q4)",
        rank(&sys) == 27 && six.iter().all(|n| n.comm(&tau1671[0]).is_zero()) && tau1671[0].is_antisymmetric());
    out
}

/// Classification for the 4+4 spacetime, in the sector of fields independent of x5, x6, x7.
pub fn analyse_44(gm: &Gammas, rep: &mut Report) -> PairAnalysis {
    let g = &gm.gamma1644;
    let mut out = PairAnalysis { lines: Vec::new(), csv_rows: Vec::new() };
    // one particle at rest: psi = u exp(-i m x4) with gamma^4 u = +i u.
    // gamma^4 is a signed permutation with square -1: gamma^4 e_a = +-e_b.  u = (e_a - i gamma^4 e_a)/sqrt(2).
    let h = 1.0 / 2f64.sqrt();
    let mut used = [false; 16];
    let mut u: Vec<Vec<C64>> = Vec::new();
    for a in 0..16 {
        if used[a] {
            continue;
        }
        // column a of gamma^4: gamma^4 e_a
        let (b, sgn) = (0..16).map(|i| (i, g[4].get(i, a))).find(|x| x.1 != 0).unwrap();
        used[a] = true;
        used[b] = true;
        let mut v = vec![C64::new(0.0, 0.0); 16];
        v[a] = C64::new(h, 0.0);
        v[b] = C64::new(0.0, -h * sgn as f64);
        u.push(v);
    }
    assert_eq!(u.len(), 8);
    let g4 = cm(&g[4]);
    // check gamma^4 u = i u
    let mut dev = 0.0f64;
    for v in &u {
        for i in 0..16 {
            let mut s = C64::new(0.0, 0.0);
            for j in 0..16 {
                s += g4.get(i, j) * v[j];
            }
            dev = dev.max((s - v[i] * C64::new(0.0, 1.0)).abs());
        }
    }
    let sig = |a: usize, b: usize| -> CMat { restrict(&cm(&g[a].mul(&g[b])).scale(0.5), &u) };
    let jgen = vec![sig(2, 3), sig(3, 1), sig(1, 2)]; // ordinary rotations (also the L of SO(4))
    let kgen = vec![sig(1, 0), sig(2, 0), sig(3, 0)]; // rotations into the hidden dimension x0
    let agen: Vec<CMat> = (0..3).map(|a| jgen[a].add(&kgen[a]).scale(0.5)).collect();
    let bgen: Vec<CMat> = (0..3).map(|a| jgen[a].sub(&kgen[a]).scale(0.5)).collect();
    let tgen = vec![sig(6, 7), sig(7, 5), sig(5, 6)]; // rotations of the extra times
    // W1644 = i Cm1644 gamma1644^4 (Cm1644 = sigma16 of 4+4.nb) restricted to the particle space
    let wfull = cm(&gm.sigma16.mul(&g[4]));
    let mut w = restrict(&wfull, &u);
    for i in 0..8 {
        for j in 0..8 {
            let z = w.get(i, j) * C64::new(0.0, 1.0);
            w.set(i, j, z);
        }
    }
    let mut comm_ab = 0.0f64;
    for a in &agen {
        for b in &bgen {
            comm_ab = comm_ab.max(a.mul(b).sub(&b.mul(a)).fro());
        }
    }
    let ops1 = vec![casimir(&agen), casimir(&bgen), casimir(&tgen), casimir(&jgen), w.clone()];
    let (m1, res1, _, _) = joint_spectrum(&ops1);
    out.lines.push(format!("4+4 (fields independent of x5,x6,x7) one particle at rest (8 states): {}", fmt_multiplets(&m1, &["A(A+1)", "B(B+1)", "T(T+1)", "J(J+1)", "W"])));
    let ok1 = m1.len() == 2 && m1.iter().all(|m| m.count == 4 && m.values[2] == 0.75 && m.values[3] == 0.75)
        && m1.iter().any(|m| m.values[0] == 0.75 && m.values[1] == 0.0) && m1.iter().any(|m| m.values[0] == 0.0 && m.values[1] == 0.75)
        && m1[0].values[4] * m1[1].values[4] == -1.0;
    rep.add("44-Q1", "numerical", &format!("4+4, fields independent of x5 x6 x7, one particle at rest (gamma1644[4] u = i u, 8 states; deviation {:.0e}): under SU(2)_A x SU(2)_B (rotations of the space x0 x1 x2 x3) x SU(2)_T (rotations of the extra times) the states are (2, 1; 2) with one sign of W1644 and (1, 2; 2) with the other; every state has ordinary spin 1/2 (residual {:.0e})", dev, res1),
        ok1 && dev < 1e-12 && comm_ab < 1e-12 && res1 < 1e-9);

    let la: Vec<CMat> = agen.iter().map(|x| lift(x, 8)).collect();
    let lb: Vec<CMat> = bgen.iter().map(|x| lift(x, 8)).collect();
    let lt: Vec<CMat> = tgen.iter().map(|x| lift(x, 8)).collect();
    let lj: Vec<CMat> = jgen.iter().map(|x| lift(x, 8)).collect();
    let ww = lift_parity(&w, 8);
    let ops2 = vec![casimir(&la), casimir(&lb), casimir(&lt), casimir(&lj), ww];
    let (m2, res2, vecs, tuples) = joint_spectrum(&ops2);
    out.lines.push(format!("4+4 two particles, s wave (28 states): {}", fmt_multiplets(&m2, &["A(A+1)", "B(B+1)", "T(T+1)", "J(J+1)", "WW"])));
    for m in &m2 {
        out.csv_rows.push(vec!["4+4".into(), format!("{}", su2_dim(m.values[0])), format!("{}", su2_dim(m.values[1])), format!("{}", su2_dim(m.values[2])),
            format!("{}", su2_dim(m.values[3])), format!("{:+}", m.values[4]), format!("{}", m.count)]);
    }
    let cnt = |a: f64, b: f64, t: f64| m2.iter().filter(|m| m.values[0] == a && m.values[1] == b && m.values[2] == t).map(|m| m.count).sum::<usize>();
    let n_so4_singlets = cnt(0.0, 0.0, 2.0) + cnt(0.0, 0.0, 0.0);
    let n_j0: usize = m2.iter().filter(|m| m.values[3] == 0.0).map(|m| m.count).sum();
    let ok2 = cnt(0.0, 0.0, 2.0) == 6 && cnt(0.0, 0.0, 0.0) == 0 && cnt(2.0, 0.0, 0.0) == 3 && cnt(0.0, 2.0, 0.0) == 3 && cnt(0.75, 0.75, 0.0) == 4 && cnt(0.75, 0.75, 2.0) == 12
        && m2.iter().map(|m| m.count).sum::<usize>() == 28;
    rep.add("44-Q2", "numerical", &format!("4+4, fields independent of x5 x6 x7, two identical particles in an s wave (28 states): (1, 1; 3) twice, (3, 1; 1), (1, 3; 1), (2, 2; 1), (2, 2; 3) under SU(2)_A x SU(2)_B x SU(2)_T.  The pairs that are singlets under ALL rotations of the four space dimensions are the {} states (1, 1; 3) + (1, 1; 3): two triplets of the extra-time rotations, none of them a singlet of SU(2)_T.  {} states have ordinary spin 0 (residual {:.0e})", n_so4_singlets, n_j0, res2),
        ok2 && n_so4_singlets == 6 && n_j0 == 10 && res2 < 1e-9);

    // which covariant bilinears dirac1644^T (Cm1644 gamma1644[A1]...gamma1644[Ar]) dirac1644 contain the SO(4)-singlet pairs?
    let basis = clifford_basis(g);
    let idx = pair_index(8);
    let mut kinds: Vec<String> = Vec::new();
    for (mask, m) in &basis {
        let x = gm.sigma16.mul(m);
        if !x.is_antisymmetric() {
            continue;
        }
        let r = restrict_bilinear(&cm(&x), &u);
        // components in the pair basis (E_sr - E_rs)/sqrt(2):  sqrt(2) R_sr
        let comp: Vec<C64> = idx.iter().map(|&(s, t)| r.get(s, t) * 2f64.sqrt()).collect();
        let norm: f64 = comp.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        if norm < 1e-9 {
            continue;
        }
        // weight of the SO(4)-singlet multiplets (A = B = 0) in this bilinear
        let mut wsing = 0.0;
        for (v, t) in vecs.iter().zip(&tuples) {
            if t[0] == 0.0 && t[1] == 0.0 {
                let ov: C64 = v.iter().zip(&comp).fold(C64::new(0.0, 0.0), |acc, (a, b)| acc + a.conj() * *b);
                wsing += ov.norm_sqr();
            }
        }
        if wsing / (norm * norm) > 0.999 {
            let ind: Vec<String> = (0..8).filter(|a| mask & (1 << a) != 0).map(|a| a.to_string()).collect();
            kinds.push(format!("{{{}}}", ind.join(",")));
        }
    }
    out.lines.push(format!("4+4: bilinears dirac1644^T Cm1644 gamma1644[A1]...gamma1644[Ar] dirac1644 whose two-particle content lies entirely in the SO(4)-singlet pairs: index sets {{A1,...,Ar}} = {}", kinds.join(" ")));
    let has = |s: &str| kinds.iter().any(|k| k == s);
    rep.add("44-Q3", "numerical", &format!("4+4: among the 120 Pauli-allowed bilinears dirac1644^T Cm1644 gamma1644[A1]...gamma1644[Ar] dirac1644 (A1 < ... < Ar), those whose two-particle content (particles at rest) lies entirely in the SO(4)-singlet pairs have the index sets {{A1,...,Ar}} = {}; they include the components A = 5, 6, 7 (the extra-time components) of the vector pair field DeltaV1644^A := dirac1644^T Cm1644 gamma1644^A dirac1644", kinds.join(" ")),
        has("{5}") && has("{6}") && has("{7}") && !has("{0}") && !has("{4}"));
    // the unique pair state that is a singlet of ordinary rotations AND of the extra-time rotations
    let mut kinds0: Vec<String> = Vec::new();
    for (mask, m) in &basis {
        let x = gm.sigma16.mul(m);
        if !x.is_antisymmetric() {
            continue;
        }
        let r = restrict_bilinear(&cm(&x), &u);
        let comp: Vec<C64> = idx.iter().map(|&(s, t)| r.get(s, t) * 2f64.sqrt()).collect();
        let norm: f64 = comp.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        if norm < 1e-9 {
            continue;
        }
        let mut w0 = 0.0;
        for (v, t) in vecs.iter().zip(&tuples) {
            if t[2] == 0.0 && t[3] == 0.0 && t[0] == 0.75 && t[1] == 0.75 {
                let ov: C64 = v.iter().zip(&comp).fold(C64::new(0.0, 0.0), |acc, (a, b)| acc + a.conj() * *b);
                w0 += ov.norm_sqr();
            }
        }
        if w0 / (norm * norm) > 0.999 {
            let ind: Vec<String> = (0..8).filter(|a| mask & (1 << a) != 0).map(|a| a.to_string()).collect();
            kinds0.push(format!("{{{}}}", ind.join(",")));
        }
    }
    let n00: usize = m2.iter().filter(|m| m.values[2] == 0.0 && m.values[3] == 0.0).map(|m| m.count).sum();
    out.lines.push(format!("4+4: bilinears dirac1644^T Cm1644 gamma1644[A1]...gamma1644[Ar] dirac1644 whose two-particle content is the unique pair state with ordinary spin 0 and extra-time isospin 0: index sets {{A1,...,Ar}} = {}", kinds0.join(" ")));
    rep.add("44-Q4", "numerical", &format!("4+4: exactly {} of the 28 pair states is a singlet both of the ordinary rotations (x1 x2 x3) and of the extra-time rotations (x5 x6 x7); it belongs to the space vector (2, 2; 1), is made of one particle with W1644 = +1 and one with W1644 = -1, and is the two-particle content of the bilinears with the index sets {{A1,...,Ar}} = {}: in particular of DeltaV1644^0 = dirac1644^T Cm1644 gamma1644^0 dirac1644, the component of the vector pair field along the hidden space dimension x0", n00, kinds0.join(" ")),
        n00 == 1 && kinds0.iter().any(|k| k == "{0}"));
    let _ = rank_of(0);
    out
}

/// Run both analyses and write data/pair_classification.csv.
pub fn run(gm: &Gammas, rep: &mut Report) -> std::io::Result<Vec<String>> {
    let a71 = analyse_71(gm, rep);
    let a44 = analyse_44(gm, rep);
    let mut rows: Vec<Vec<String>> = Vec::new();
    for r in a71.csv_rows {
        rows.push(vec![r[0].clone(), r[1].clone(), r[2].clone(), r[3].clone(), "-".into(), r[4].clone(), "-".into(), r[5].clone()]);
    }
    for r in a44.csv_rows {
        rows.push(vec![r[0].clone(), r[4].clone(), r[1].clone(), r[2].clone(), r[3].clone(), "-".into(), r[5].clone(), r[6].clone()]);
    }
    // columns: for 7+1: (dim J, dim A, dim B, SO(7) multiplet); for 4+4: (dim J, dim A, dim B, dim T, W W)
    crate::io::write_csv_str("pair_classification.csv",
        &[("content", "s-wave pair states of two identical particles of the 16-component field at rest, by multiplet".to_string()),
          ("7+1", "J: ordinary rotations (x1 x2 x3); A, B: the two SU(2) factors of the rotations of the extra space (x0 x5 x6 x7); SO(7) multiplet".to_string()),
          ("4+4", "fields independent of x5 x6 x7; A, B: the two SU(2) factors of the rotations of space (x0 x1 x2 x3); T: rotations of the extra times; J: ordinary rotations; WW: product of the W1644 eigenvalues of the two particles".to_string()),
          ("entries", "dimension 2j+1 of each SU(2) multiplet".to_string())],
        &["spacetime", "dim_J", "dim_A", "dim_B", "dim_T", "SO7_multiplet", "WW", "states"], &rows)?;
    let mut lines = a71.lines;
    lines.extend(a44.lines);
    Ok(lines)
}
