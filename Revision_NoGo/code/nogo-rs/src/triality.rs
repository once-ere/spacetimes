//! Command `triality`: Cartan's triality from the octonion product, for the
//! octonions O_NG (the compact algebra so(8)) and the split octonions Os_NG (the
//! split real form so(4,4)), and the proof that it does NOT exist for the 7+1
//! Lorentz algebra so(7,1).
//!
//! Local triality.  For every a in so(N) (N the norm form of the algebra) there is
//! exactly one pair (b, c) in so(N) x so(N) with
//!
//!     a(x y) = (b x) y + x (c y)        for all x, y.                           (LT)
//!
//! The maps T_b : a -> b and T_c : a -> c are Lie-algebra automorphisms of so(N);
//! they change the spectrum of a simple rotation from {+-i, 0 (6 times)} to
//! {+-i/2 (4 times each)} (the spectrum of a half-spinor generator), so they are
//! OUTER automorphisms; together with Ad(kappa), kappa = conjugation x -> conj(x),
//! they generate the group Sym(3) (the permutation group of three objects, which is the outer
//! automorphism group of so(8, C)), and theta = T_b o Ad(kappa) has order 3:
//! it permutes cyclically the vector representation V and the two half-spinor
//! representations S+ and S-.
//!
//! The 7+1 obstruction.  The 7+1 metric is eta(x, x) = Re(x^2) (command
//! `octonions`, check O-14c); it is not the norm of any composition algebra.
//! (LT) has NO solution (b, c), not even in gl(8) x gl(8), for the boost
//! a = E_01 + E_10 of so(7,1); and on the real 16-dimensional spinor space the
//! commutant of so(7,1) is span{1, omega} with omega^2 = -1 (complex type: there
//! are no real 8-dimensional half-spinor representations), while the vector
//! representation is of real type.  Hence no automorphism of so(7,1) can map the
//! vector representation onto a half-spinor representation: triality is a property
//! of so(8) and so(4,4) (and of so(8, C)), not of so(7,1).

use crate::checks::Checks;
use crate::exact::{char_poly, poly_string, qmul, rank, solve, Q, QMat};
use crate::oct::{cayley_dickson, Alg};
use spacetimes::gamma::Gammas;
use spacetimes::imat::IMat;
use spacetimes::io::write_csv_str;

/// The basis a_(ij) = N (E_ij - E_ji), i < j, of so(N) (N = diag(norm), N^-1 = N).
pub fn so_basis(norm: &[i64; 8]) -> Vec<QMat> {
    let mut v = Vec::new();
    for i in 0..8 {
        for j in (i + 1)..8 {
            let mut m = vec![vec![Q::ZERO; 8]; 8];
            m[i][j] = Q::int(norm[i]);
            m[j][i] = Q::int(-norm[j]);
            v.push(m);
        }
    }
    v
}

/// Coordinates of X in so(N) with respect to `so_basis`: x_(ij) = (N X)_ij, i < j.
pub fn so_coords(x: &QMat, norm: &[i64; 8]) -> Vec<Q> {
    let mut c = Vec::new();
    for i in 0..8 {
        for j in (i + 1)..8 {
            c.push(Q::int(norm[i]).mul(x[i][j]));
        }
    }
    c
}

fn from_coords(c: &[Q], basis: &[QMat]) -> QMat {
    let mut m = vec![vec![Q::ZERO; 8]; 8];
    for (k, b) in basis.iter().enumerate() {
        if c[k].is_zero() {
            continue;
        }
        for i in 0..8 {
            for j in 0..8 {
                m[i][j] = m[i][j].add(c[k].mul(b[i][j]));
            }
        }
    }
    m
}

fn mat_comm(a: &QMat, b: &QMat) -> QMat {
    let ab = qmul(a, b);
    let ba = qmul(b, a);
    ab.iter().zip(&ba).map(|(r, s)| r.iter().zip(s).map(|(x, y)| x.sub(*y)).collect()).collect()
}

/// Product of the algebra applied to rational vectors.
fn amul(alg: &Alg, x: &[Q], y: &[Q]) -> Vec<Q> {
    let mut r = vec![Q::ZERO; 8];
    for a in 0..8 {
        if x[a].is_zero() {
            continue;
        }
        for b in 0..8 {
            if y[b].is_zero() {
                continue;
            }
            let xy = x[a].mul(y[b]);
            for t in 0..8 {
                if alg.c[a][b][t] != 0 {
                    r[t] = r[t].add(xy.mul(Q::int(alg.c[a][b][t])));
                }
            }
        }
    }
    r
}

fn col(m: &QMat, p: usize) -> Vec<Q> {
    (0..8).map(|s| m[s][p]).collect()
}

fn e(p: usize) -> Vec<Q> {
    let mut v = vec![Q::ZERO; 8];
    v[p] = Q::ONE;
    v
}

/// Solve (LT) for given a.  `unknowns` is the list of candidate matrices for b
/// (and the same list for c); the unknowns are the coefficients of b and c in
/// that list.  Returns (b, c, kernel dimension) or None if (LT) is inconsistent.
pub fn local_triality(alg: &Alg, a: &QMat, unknowns: &[QMat]) -> Option<(QMat, QMat, usize)> {
    let nu = unknowns.len();
    let mut rows: QMat = Vec::new();
    let mut rhs: Vec<Q> = Vec::new();
    for p in 0..8 {
        for r in 0..8 {
            // left-hand side a(e_p e_r)
            let epr = amul(alg, &e(p), &e(r));
            let lhs: Vec<Q> = (0..8).map(|s| (0..8).fold(Q::ZERO, |acc, t| acc.add(a[s][t].mul(epr[t])))).collect();
            // columns of the unknowns
            let bcols: Vec<Vec<Q>> = unknowns.iter().map(|u| amul(alg, &col(u, p), &e(r))).collect();
            let ccols: Vec<Vec<Q>> = unknowns.iter().map(|u| amul(alg, &e(p), &col(u, r))).collect();
            for t in 0..8 {
                let mut row = Vec::with_capacity(2 * nu);
                for k in 0..nu {
                    row.push(bcols[k][t]);
                }
                for k in 0..nu {
                    row.push(ccols[k][t]);
                }
                rows.push(row);
                rhs.push(lhs[t]);
            }
        }
    }
    let (x, kdim) = solve(&rows, &rhs)?;
    let b = from_coords(&x[..nu], unknowns);
    let c = from_coords(&x[nu..], unknowns);
    Some((b, c, kdim))
}

/// The 64 matrix units E_st (a basis of gl(8)).
fn gl_basis() -> Vec<QMat> {
    let mut v = Vec::new();
    for s in 0..8 {
        for t in 0..8 {
            let mut m = vec![vec![Q::ZERO; 8]; 8];
            m[s][t] = Q::ONE;
            v.push(m);
        }
    }
    v
}

/// A linear map of so(N) as a 28 x 28 rational matrix (column k = image of basis k).
type Lin = QMat;

fn lin_apply(t: &Lin, x: &[Q]) -> Vec<Q> {
    (0..t.len()).map(|i| (0..x.len()).fold(Q::ZERO, |acc, k| acc.add(t[i][k].mul(x[k])))).collect()
}

fn is_identity(t: &Lin) -> bool {
    (0..t.len()).all(|i| (0..t.len()).all(|j| t[i][j] == if i == j { Q::ONE } else { Q::ZERO }))
}

fn qm_from_imat(m: &IMat) -> QMat {
    (0..m.n).map(|i| (0..m.m).map(|j| Q::int(m.get(i, j))).collect()).collect()
}

/// Dimension of the commutant { X : X G = G X for all G in gens } of a set of n x n
/// integer matrices: an upper bound from the rank modulo a prime (exact upper
/// bound, see spacetimes::imat::rank), which the caller confirms by exhibiting
/// that many independent commuting matrices.
fn commutant_dim_upper(gens: &[IMat]) -> usize {
    let n = gens[0].n;
    let mut rows: Vec<Vec<i64>> = Vec::new();
    for g in gens {
        for i in 0..n {
            for j in 0..n {
                // (X G - G X)_ij = sum_l X_il G_lj - sum_k G_ik X_kj
                let mut row = vec![0i64; n * n];
                for l in 0..n {
                    row[i * n + l] += g.get(l, j);
                }
                for k in 0..n {
                    row[k * n + j] -= g.get(i, k);
                }
                rows.push(row);
            }
        }
    }
    n * n - spacetimes::imat::rank(&rows)
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_NoGo, command `triality`: local triality of O_NG (so(8)) and Os_NG (so(4,4)); its absence for so(7,1)");
    let mut out: Vec<Vec<String>> = Vec::new();
    let gm = Gammas::build();
    for alg in [&cayley_dickson(-1), &cayley_dickson(1)] {
        let nm = alg.name;
        let basis = so_basis(&alg.norm);
        // T-1: unique solution for every basis element
        let mut tb: Vec<Vec<Q>> = Vec::new();
        let mut tc: Vec<Vec<Q>> = Vec::new();
        let mut ok = true;
        for a in &basis {
            match local_triality(alg, a, &basis) {
                Some((b, c, kdim)) => {
                    ok &= kdim == 0;
                    tb.push(so_coords(&b, &alg.norm));
                    tc.push(so_coords(&c, &alg.norm));
                }
                None => {
                    ok = false;
                    tb.push(vec![Q::ZERO; 28]);
                    tc.push(vec![Q::ZERO; 28]);
                }
            }
        }
        ck.add(&format!("T-1 [{}]", nm), "local triality (LT): for each of the 28 basis elements m1 of so(N) the linear system for (m2, m3) in so(N) x so(N) is consistent with a unique solution (exact)", ok, String::new());
        // matrices of T_b, T_c (column k = coordinates of the image of basis k)
        let tbm: Lin = (0..28).map(|i| (0..28).map(|k| tb[k][i]).collect()).collect();
        let tcm: Lin = (0..28).map(|i| (0..28).map(|k| tc[k][i]).collect()).collect();
        // T-2 automorphisms
        let mut ok = rank(&tbm) == 28 && rank(&tcm) == 28;
        for i in 0..28 {
            for j in (i + 1)..28 {
                let br = so_coords(&mat_comm(&basis[i], &basis[j]), &alg.norm);
                for t in [&tbm, &tcm] {
                    let lhs = lin_apply(t, &br);
                    let ti = from_coords(&lin_apply(t, &so_coords(&basis[i], &alg.norm)), &basis);
                    let tj = from_coords(&lin_apply(t, &so_coords(&basis[j], &alg.norm)), &basis);
                    let rhs = so_coords(&mat_comm(&ti, &tj), &alg.norm);
                    ok &= lhs == rhs;
                }
            }
        }
        ck.add(&format!("T-2 [{}]", nm), "T2: m1 -> m2 and T3: m1 -> m3 are invertible Lie-algebra automorphisms of so(N): they preserve the commutators of all 378 pairs of basis elements", ok, String::new());
        // T-3 involutions
        let tb2 = qmul(&tbm, &tbm);
        let tc2 = qmul(&tcm, &tcm);
        ck.add(&format!("T-3 [{}]", nm), "T2 and T3 are involutions: T2^2 = T3^2 = 1", is_identity(&tb2) && is_identity(&tc2), String::new());
        // T-4 outer: characteristic polynomials of a_(01) and of its images
        let a01 = &basis[0];
        let b01 = from_coords(&lin_apply(&tbm, &so_coords(a01, &alg.norm)), &basis);
        let c01 = from_coords(&lin_apply(&tcm, &so_coords(a01, &alg.norm)), &basis);
        let pa = char_poly(a01);
        let pb = char_poly(&b01);
        let pc = char_poly(&c01);
        // (lambda^2 + 1/4)^4 = lambda^8 + lambda^6 + 3/8 lambda^4 + 1/16 lambda^2 + 1/256
        let spin = vec![Q::new(1, 256), Q::ZERO, Q::new(1, 16), Q::ZERO, Q::new(3, 8), Q::ZERO, Q::ONE, Q::ZERO, Q::ONE];
        let vecp = vec![Q::ZERO, Q::ZERO, Q::ZERO, Q::ZERO, Q::ZERO, Q::ZERO, Q::ONE, Q::ZERO, Q::ONE];
        ck.add(
            &format!("T-4 [{}]", nm),
            "T2 and T3 are OUTER: the simple rotation r01 has det(lambda - r01) = lambda^6 (lambda^2 + 1) (vector representation), its images have det(lambda - T2 r01) = det(lambda - T3 r01) = (lambda^2 + 1/4)^4 (half-spinor spectrum); conjugation by O(N) preserves characteristic polynomials",
            pa == vecp && pb == spin && pc == spin,
            format!("r01: {}; T2 r01: {}; T3 r01: {}", poly_string(&pa), poly_string(&pb), poly_string(&pc)),
        );
        // T-5 the order-3 automorphism and the permutation group Sym(3)
        let kappa: Vec<i64> = (0..8).map(|i| if i == 0 { 1 } else { -1 }).collect();
        let adk: Lin = {
            let cols: Vec<Vec<Q>> = basis
                .iter()
                .map(|m| {
                    let km: QMat = (0..8).map(|i| (0..8).map(|j| m[i][j].mul(Q::int(kappa[i] * kappa[j]))).collect()).collect();
                    so_coords(&km, &alg.norm)
                })
                .collect();
            (0..28).map(|i| (0..28).map(|k| cols[k][i]).collect()).collect()
        };
        let theta = qmul(&tbm, &adk);
        let theta2 = qmul(&theta, &theta);
        let theta3 = qmul(&theta2, &theta);
        let k2 = qmul(&adk, &adk);
        let dihedral = qmul(&qmul(&adk, &theta), &adk) == theta2; // kappa theta kappa = theta^-1 = theta^2
        ck.add(
            &format!("T-5 [{}]", nm),
            "vartheta = T2 o Ad(kappa) (kappa: x -> conj(x)) has order 3 (vartheta^3 = 1, vartheta != 1), Ad(kappa)^2 = 1 and Ad(kappa) vartheta Ad(kappa) = vartheta^-1: the automorphisms generate Sym(3) (the permutations of three objects), the outer automorphism group of so(8, C)",
            is_identity(&theta3) && !is_identity(&theta) && is_identity(&k2) && dihedral,
            String::new(),
        );
        // T-6 the images are the half-spinor generators of the project matrices
        // (4+4: gamma1644 is block off-diagonal, Sigma^01 = (1/4)[g0, g1] is block
        // diagonal; its 8x8 blocks act on S+ and S-.)
        if alg.g > 0 {
            let s01 = gm.gamma1644[0].mul(&gm.gamma1644[1]); // = 2 Sigma^01 (A != B)
            let up = s01.block(0, 0, 8, 8);
            let dn = s01.block(8, 8, 8, 8);
            // char poly of (1/2) block
            let half = |m: &IMat| -> QMat { (0..8).map(|i| (0..8).map(|j| Q::new(m.get(i, j) as i128, 2)).collect()).collect() };
            let pu = char_poly(&half(&up));
            let pd = char_poly(&half(&dn));
            ck.add("T-6 [Os_NG]", "the triality images have exactly the spectrum of the half-spinor generators of the project: Sigma^01_1644 restricted to S+ and to S- has det(lambda - Sigma^01_1644) = (lambda^2 + 1/4)^4", pu == spin && pd == spin, format!("S+: {}; S-: {}", poly_string(&pu), poly_string(&pd)));
        }
        let theta_a = from_coords(&lin_apply(&theta, &so_coords(a01, &alg.norm)), &basis);
        let theta2_a = from_coords(&lin_apply(&theta2, &so_coords(a01, &alg.norm)), &basis);
        for (lab, p) in [("r01 (vector)", pa.clone()), ("T2(r01)", pb.clone()), ("T3(r01)", pc.clone()), ("vartheta(r01)", char_poly(&theta_a)), ("vartheta^2(r01)", char_poly(&theta2_a))] {
            out.push(vec![nm.to_string(), lab.to_string(), poly_string(&p)]);
        }
    }

    // T-7 the 7+1 obstruction
    let o = cayley_dickson(-1);
    let mut boost = vec![vec![Q::ZERO; 8]; 8];
    boost[0][1] = Q::ONE;
    boost[1][0] = Q::ONE;
    // boost is in so(eta), eta = diag(1, -1, ..., -1) = Re(x^2): boost^T eta + eta boost = 0
    let eta = [1i64, -1, -1, -1, -1, -1, -1, -1];
    let in_so71 = (0..8).all(|i| (0..8).all(|j| boost[j][i].mul(Q::int(eta[j])).add(Q::int(eta[i]).mul(boost[i][j])).is_zero()));
    let sol = local_triality(&o, &boost, &gl_basis());
    ck.add("T-7a", "so(7,1) has no local triality: for the boost m1 with m1 e0 = e1, m1 e1 = e0 and m1 = 0 on e2..e7 (an element of so(7,1) for the metric Re(x^2)) the system (LT) is INCONSISTENT even with m2, m3 arbitrary real 8-by-8 matrices (exact elimination)", in_so71 && sol.is_none(), String::new());
    // commutants on the real 16-dimensional spinor space
    let gens71: Vec<IMat> = (0..7).map(|a| gm.gamma1671[a].mul(&gm.gamma1671[a + 1])).collect();
    let gens44: Vec<IMat> = (0..7).map(|a| gm.gamma1644[a].mul(&gm.gamma1644[a + 1])).collect();
    let om71 = IMat::product(&gm.gamma1671.iter().collect::<Vec<_>>());
    let om44 = IMat::product(&gm.gamma1644.iter().collect::<Vec<_>>());
    let d71 = commutant_dim_upper(&gens71);
    let d44 = commutant_dim_upper(&gens44);
    let comm71 = gens71.iter().all(|g| g.comm(&om71).is_zero());
    let comm44 = gens44.iter().all(|g| g.comm(&om44).is_zero());
    let sq71 = om71.mul(&om71) == IMat::identity(16).neg();
    let sq44 = om44.mul(&om44) == IMat::identity(16);
    ck.add("T-7b", "commutant of so(7,1) on the real 16-dimensional spinor space is span{1, omega1671} with omega1671^2 = -1: the spinor representation is of COMPLEX type (no real 8-dimensional half-spinors; S+ and S- are complex conjugates)", d71 == 2 && comm71 && sq71, format!("dim <= {} (rank modulo a large prime), 1 and omega1671 commute", d71));
    ck.add("T-7c", "commutant of so(4,4) on the real 16-dimensional spinor space is span{1, omega1644} with omega1644^2 = +1: the space is S+ + S- with two REAL 8-dimensional half-spinor representations (Majorana-Weyl)", d44 == 2 && comm44 && sq44, format!("dim <= {} (rank modulo a large prime)", d44));
    // vector representation of so(7,1): generators V_(AB) = E_AB eta_BB - E_BA eta_AA
    let eta71 = spacetimes::gamma::ETA1671;
    let vgens: Vec<IMat> = (0..7)
        .map(|a| {
            let mut m = IMat::zeros(8, 8);
            m.set(a, a + 1, eta71[a + 1]);
            m.set(a + 1, a, -eta71[a]);
            m
        })
        .collect();
    let dv = commutant_dim_upper(&vgens);
    ck.add("T-7d", "commutant of so(7,1) in its vector representation is the real numbers (dimension 1): the vector representation is of REAL type; an automorphism preserves the type, so no automorphism of so(7,1) maps the vector representation onto a half-spinor representation -- there is no triality for 7+1", dv == 1, format!("dim <= {} (rank modulo a large prime)", dv));
    let _ = qm_from_imat; // (helper kept for symmetric use in other commands)
    write_csv_str(
        "ng_triality_charpolys.csv",
        &[("content", "characteristic polynomials det(lambda - matrix) of the simple rotation r01 and of its images under the triality automorphisms T2, T3, vartheta, vartheta^2 (exact rational coefficients)".to_string())],
        &["algebra", "matrix", "char_poly"],
        &out,
    )
    .expect("csv");
    ck.write("ng_triality_checks.txt")
}
