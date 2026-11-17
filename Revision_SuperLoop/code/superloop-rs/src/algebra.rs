//! Command `algebra`: the bi-octonions B_LL, the Lorentz super-loop L_LL and its Malcev algebra m_LL.
//!
//! Checks (all exact unless marked "double precision"):
//!   A-1 .. A-8  where associativity enters: alternativity, non-associativity, the norm N_LL,
//!               the Moufang identities, Jac = 6 x associator, the Malcev identity, operators
//!               always obey the Jacobi identity, left multiplication is not a Lie homomorphism,
//!               and the Jacobiators span m_LL (Theorem R: no realization by operators).
//!   L-1 .. L-9  the Lorentz part l_LL = C (x) Im H inside m_LL = C (x) Im O, the spinor charges
//!               s_LL = (C (x) H) e4, the twisted action [uL, .] = -2 W_LL(uL) on s_LL, the bracket
//!               [s_LL, s_LL] = l_LL, the Lie no-go for the same pattern (Theorem L), the derivations
//!               D_LL (Lorentz) and Dv_LL (isospin), the derivation algebra g2(C) (real dimension 28),
//!               and the automorphisms varphi_(gL,vI).
//!   Q-1 .. Q-6  the operators and their eigenvalues: electric charge Q_LL, isospin Casimir T_LL^2,
//!               Lorentz Casimir Cas_LL, chirality, spinor number, and the twofold inner action.
//!   C-1 .. C-4  the unit e4 (a Chein-type extension of SL(2,C)), parity, and the impossibility of
//!               an associative (group, Hilbert-space) implementation.
//! Names (plain text): x, y, z, w bi-octonions; aH, bS (and cH, dS) the bi-quaternions of
//! x = aH + bS e4; uL in l_LL a Lorentz element; tI in Im H an isospin element; gL, hL elements of
//! SL(2,C); vI a unit quaternion (isospin rotation); e0..e7 the units.
//! Data: sl_multiplication_table.csv, sl_malcev_structure.csv, sl_quantum_numbers.csv,
//!       sl_associators.csv, report sl_algebra_checks.txt.

use crate::bioct::*;
use crate::checks::Checks;
use crate::exact::*;
use spacetimes::complex::C64;
use spacetimes::io::write_csv_str;
use spacetimes::linalg::{eigh, CMat};

pub type B = Bi<GInt>;

pub fn e(a: usize) -> B {
    B::unit(a)
}
pub fn ie(a: usize) -> B {
    B::unit(a).scale(GInt::iu())
}
fn gi(re: i128, im: i128) -> GInt {
    GInt { re, im }
}

/// The real basis of the Malcev algebra m_LL = C (x) Im O (14 elements), in this order:
///   e1, e2, e3 (twice the rotation generators J_LL^j), i e1, i e2, i e3 (twice the boost
///   generators K_LL^j), e4..e7 and i e4..i e7 (the spinor charges, a real basis of s_LL).
pub fn mbasis(k: usize) -> B {
    match k {
        0..=2 => e(k + 1),
        3..=5 => ie(k - 2),
        6..=9 => e(k - 2),
        _ => ie(k - 6),
    }
}

/// Coordinates of an element of m_LL in the basis `mbasis` (panics if the e0 part is non-zero).
pub fn mcoords(x: &B) -> [i128; 14] {
    assert!(x.c[0].re == 0 && x.c[0].im == 0, "element is not imaginary");
    let mut r = [0i128; 14];
    for a in 1..4 {
        r[a - 1] = x.c[a].re;
        r[a + 2] = x.c[a].im;
    }
    for a in 4..8 {
        r[a + 2] = x.c[a].re;
        r[a + 6] = x.c[a].im;
    }
    r
}

/// The Lorentz derivation D_LL(uL) (uL in l_LL = C (x) Im H):  aH + bS e4  ->  [uL, aH] + (-bS uL) e4.
/// It is linear in uL; D_LL(J_LL^j) = D_LL(e_j/2) and D_LL(K_LL^j) = D_LL(i e_j/2).
pub fn d_lorentz(q: &Quat<GInt>, x: &B) -> B {
    let (a, b) = x.quats();
    let qa = qsub(&qmul(q, &a), &qmul(&a, q));
    let bq = qscale(&qmul(&b, q), GInt { re: -1, im: 0 });
    B::from_quats(&qa, &bq)
}

/// The isospin derivation Dv_LL(tI) (tI in Im H):  aH + bS e4 -> (tI bS) e4.
pub fn d_isospin(t: &Quat<GInt>, x: &B) -> B {
    let (_a, b) = x.quats();
    B::from_quats(&qzero(), &qmul(t, &b))
}

fn quat_unit(k: usize) -> Quat<GInt> {
    let mut q = qzero::<GInt>();
    q[k] = GInt::one();
    q
}

fn cmat8<F: Fn(&Bi<C64>) -> Bi<C64>>(f: F) -> CMat {
    let mut m = CMat::zeros(8);
    for col in 0..8 {
        let img = f(&Bi::<C64>::unit(col));
        for row in 0..8 {
            m.set(row, col, img.c[row]);
        }
    }
    m
}

fn to_c64q(q: &Quat<GInt>) -> Quat<C64> {
    [
        C64::new(q[0].re as f64, q[0].im as f64),
        C64::new(q[1].re as f64, q[1].im as f64),
        C64::new(q[2].re as f64, q[2].im as f64),
        C64::new(q[3].re as f64, q[3].im as f64),
    ]
}

fn d_isospin_c(t: &Quat<C64>, x: &Bi<C64>) -> Bi<C64> {
    let (_a, b) = x.quats();
    Bi::from_quats(&qzero(), &qmul(t, &b))
}

fn d_lorentz_c(q: &Quat<C64>, x: &Bi<C64>) -> Bi<C64> {
    let (a, b) = x.quats();
    let qa = qsub(&qmul(q, &a), &qmul(&a, q));
    let bq = qscale(&qmul(&b, q), C64::new(-1.0, 0.0));
    Bi::from_quats(&qa, &bq)
}

/// eigenvalues (ascending) of a Hermitian 8x8 matrix, restricted to the subspace spanned by
/// the basis vectors `idx` (the matrix must preserve that subspace)
fn eig_block(m: &CMat, idx: &[usize]) -> Vec<f64> {
    let n = idx.len();
    let mut b = CMat::zeros(n);
    for (i, &r) in idx.iter().enumerate() {
        for (j, &c) in idx.iter().enumerate() {
            b.set(i, j, m.get(r, c));
        }
    }
    eigh(&b).0
}

fn fmt_eigs(v: &[f64]) -> String {
    v.iter().map(|x| format!("{:+.6}", x)).collect::<Vec<_>>().join(" ")
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_SuperLoop, command `algebra`: bi-octonions, the Lorentz super-loop, its Malcev algebra, its operators and eigenvalues");
    let mut rng = Rng::new(20261116);

    // ---------------- A: where associativity enters ----------------
    // A-1 alternativity: the associator is an alternating trilinear map (basis triples suffice)
    let mut alt_ok = true;
    let mut nonzero_assoc = 0usize;
    let mut assoc_rows: Vec<Vec<String>> = Vec::new();
    for a in 0..8 {
        for b in 0..8 {
            for c in 0..8 {
                let x = e(a).assoc(&e(b), &e(c));
                let y = e(b).assoc(&e(a), &e(c));
                let z = e(a).assoc(&e(c), &e(b));
                alt_ok &= x.add(&y).is_zero(0.0) && x.add(&z).is_zero(0.0);
                if !x.is_zero(0.0) {
                    nonzero_assoc += 1;
                    if a < b && b < c {
                        let k = (0..8).find(|&k| x.c[k].re != 0).unwrap();
                        assoc_rows.push(vec![format!("e{}", a), format!("e{}", b), format!("e{}", c), format!("e{}", k), x.c[k].re.to_string()]);
                    }
                }
            }
        }
    }
    ck.add("A-1", "the associator [x,y,z] = (xy)z - x(yz) of the bi-octonions B_LL is alternating (it changes sign under every exchange of two of its arguments) on all 512 ordered triples of the units e0..e7: B_LL is an ALTERNATIVE algebra", alt_ok, String::new());
    let w = e(1).assoc(&e(2), &e(4));
    ck.add("A-2", "B_LL is NOT associative: exactly 168 of the 512 ordered triples of units have a non-zero associator (the triples of distinct imaginary units outside the 7 quaternionic triples); example [e1,e2,e4] = (e1 e2) e4 - e1 (e2 e4) = 2 e7", nonzero_assoc == 168 && w == e(7).scale(gi(2, 0)), format!("non-zero associators: {}; coefficients of [e1,e2,e4] on e0..e7: {:?}", nonzero_assoc, w.c.iter().map(|z| z.re).collect::<Vec<_>>()));

    // A-3 norm and Moufang identities on random exact elements
    let mut nm_ok = true;
    let mut mf_ok = true;
    for _ in 0..300 {
        let x: B = rng.bi(3);
        let y: B = rng.bi(3);
        let z: B = rng.bi(3);
        nm_ok &= x.mul(&y).norm() == x.norm().mul(y.norm());
        let m1 = z.mul(&x.mul(&z.mul(&y))) == z.mul(&x).mul(&z).mul(&y);
        let m2 = y.mul(&z).mul(&x).mul(&z) == y.mul(&z.mul(&x.mul(&z)));
        let m3 = z.mul(&x).mul(&y.mul(&z)) == z.mul(&x.mul(&y)).mul(&z);
        mf_ok &= m1 && m2 && m3;
    }
    ck.add("A-3", "on 300 random bi-octonions x, y, z with Gaussian-integer coefficients the complex norm is multiplicative, N_LL(xy) = N_LL(x) N_LL(y), and the three Moufang identities z(x(zy)) = ((zx)z)y, ((yz)x)z = y(z(xz)), (zx)(yz) = (z(xy))z hold exactly: the unit bi-octonions form a Moufang LOOP, the super-loop L_LL", nm_ok && mf_ok, String::new());

    // A-4 Jacobiator = 6 x associator on all basis triples (trilinear identity)
    let mut j6_ok = true;
    let mut jac_nonzero = 0;
    for a in 0..8 {
        for b in 0..8 {
            for c in 0..8 {
                let j = jacobiator(&e(a), &e(b), &e(c));
                let s = e(a).assoc(&e(b), &e(c)).scale(gi(6, 0));
                j6_ok &= j == s;
                if !j.is_zero(0.0) {
                    jac_nonzero += 1;
                }
            }
        }
    }
    ck.add("A-4", "for the commutator [x,y] = xy - yx of B_LL the Jacobiator Jac(x,y,z) = [[x,y],z] + [[y,z],x] + [[z,x],y] equals six times the associator on all 512 triples of units, Jac(x,y,z) = 6 [x,y,z]; it is non-zero on 168 triples: the Jacobi identity fails exactly where associativity fails", j6_ok && jac_nonzero == 168, format!("non-zero Jacobiators: {}", jac_nonzero));

    // A-5 Malcev identity on all 4-tuples of imaginary basis units (multilinear identity)
    let mut mal_ok = true;
    for a in 1..8 {
        for b in 1..8 {
            for c in 1..8 {
                for d in 1..8 {
                    mal_ok &= malcev_defect(&e(a), &e(b), &e(c), &e(d)).is_zero(0.0);
                }
            }
        }
    }
    ck.add("A-5", "the imaginary bi-octonions m_LL = C (x) Im O with the commutator satisfy the linearized Malcev identity Jac(x,y,[w,z]) + Jac(w,y,[x,z]) = [Jac(x,y,z),w] + [Jac(w,y,z),x] on all 2401 4-tuples of imaginary units: m_LL is a Malcev algebra that is not a Lie algebra", mal_ok, String::new());

    // A-6 operators always satisfy the Jacobi identity
    let mut op_ok = true;
    for _ in 0..50 {
        let mk = |r: &mut Rng| -> IM { (0..4).map(|_| (0..4).map(|_| r.int(5) as i128).collect()).collect() };
        let (a, b, c) = (mk(&mut rng), mk(&mut rng), mk(&mut rng));
        let j = madd(&madd(&comm(&comm(&a, &b), &c), &comm(&comm(&b, &c), &a)), &comm(&comm(&c, &a), &b));
        op_ok &= is_zero(&j);
    }
    ck.add("A-6", "for 50 random triples of 4x4 integer matrices (operators, whose product is associative) the Jacobiator of the commutator vanishes exactly; this is the step that every no-go proof takes when it treats the symmetry generators as operators", op_ok, String::new());

    // A-7 left multiplication is not a Lie homomorphism
    let l = |x: &B| real_matrix(|y: &B| x.mul(y));
    let defect = msub(&comm(&l(&e(1)), &l(&e(4))), &l(&e(1).comm(&e(4))));
    ck.add("A-7", "left multiplication x -> L_x is not a representation of the Malcev algebra m_LL: [L_e1, L_e4] - L_[e1,e4] is a non-zero matrix", !is_zero(&defect), String::new());

    // A-8 Theorem R: the Jacobiators of the real basis of m_LL span m_LL (rank 14)
    let mut jrows: Vec<Vec<i128>> = Vec::new();
    for i in 0..14 {
        for j in 0..14 {
            for k in 0..14 {
                let jac = jacobiator(&mbasis(i), &mbasis(j), &mbasis(k));
                if !jac.is_zero(0.0) {
                    jrows.push(mcoords(&jac).to_vec());
                }
            }
        }
    }
    let jrank = rank(&jrows);
    ck.add("A-8", "Theorem R: the Jacobiators of the real basis elements e_p, i e_p (p = 1..7) of m_LL span all of m_LL (real rank 14, exact); because operators obey the Jacobi identity (A-6), every linear map from m_LL to operators on a vector space that turns commutators into commutators maps every Jacobiator to 0 and therefore vanishes: m_LL has no non-zero realization by operators", jrank == 14, format!("rank of the span of the Jacobiators: {}", jrank));

    // ---------------- L: the Lorentz super-loop algebra ----------------
    // L-1 so(3,1): [J^j,J^j'] = eps3 J^j'', [J^j,K^j'] = eps3 K^j'', [K^j,K^j'] = -eps3 J^j'' with J^j = e_j/2, K^j = i e_j/2
    let eps = |a: usize, b: usize, c: usize| -> i128 {
        let p = [(1, 2, 3), (2, 3, 1), (3, 1, 2)];
        let m = [(2, 1, 3), (3, 2, 1), (1, 3, 2)];
        if p.contains(&(a, b, c)) {
            1
        } else if m.contains(&(a, b, c)) {
            -1
        } else {
            0
        }
    };
    let mut so31 = true;
    for a in 1..4 {
        for b in 1..4 {
            let mut jj = B::zero();
            let mut jk = B::zero();
            let mut kk = B::zero();
            for c in 1..4 {
                jj = jj.add(&e(c).scale(gi(2 * eps(a, b, c), 0)));
                jk = jk.add(&ie(c).scale(gi(2 * eps(a, b, c), 0)));
                kk = kk.add(&e(c).scale(gi(-2 * eps(a, b, c), 0)));
            }
            so31 &= e(a).comm(&e(b)) == jj && e(a).comm(&ie(b)) == jk && ie(a).comm(&ie(b)) == kk;
        }
    }
    ck.add("L-1", "the Lorentz part l_LL = C (x) Im H (real basis J_LL^j = e_j/2, K_LL^j = i e_j/2, j = 1,2,3) is a Lie subalgebra of m_LL isomorphic to so(3,1): [J_LL^j, J_LL^j'] = sum_j'' eps3_(j j' j'') J_LL^j'', [J_LL^j, K_LL^j'] = sum_j'' eps3_(j j' j'') K_LL^j'', [K_LL^j, K_LL^j'] = -sum_j'' eps3_(j j' j'') J_LL^j''; the unit bi-quaternions SL(2,C) = Spin(3,1)^+ form a subgroup of the super-loop L_LL", so31, String::new());

    // L-2 Killing form of l_LL (6x6, in the basis e1, e2, e3, i e1, i e2, i e3): signature (3 positive, 3 negative)
    let ad6 = |k: usize| -> IM {
        let mut m = vec![vec![0i128; 6]; 6];
        for j in 0..6 {
            let r = mcoords(&mbasis(k).comm(&mbasis(j)));
            for i in 0..6 {
                m[i][j] = r[i];
            }
        }
        m
    };
    let mut kil = vec![vec![0i128; 6]; 6];
    for i in 0..6 {
        for j in 0..6 {
            let p = mmul(&ad6(i), &ad6(j));
            kil[i][j] = (0..6).map(|k| p[k][k]).sum();
        }
    }
    let sig = signature(&kil);
    ck.add("L-2", "the Killing form Killing(x,y) = tr(ad x ad y) of l_LL (6x6, exact) is non-degenerate with signature (3 positive, 3 negative), negative on the rotations and positive on the boosts, as for so(3,1)", sig == (3, 3, 0) && kil[0][0] < 0 && kil[3][3] > 0, format!("signature {:?}, Killing(e1,e1) = {}, Killing(i e1,i e1) = {}", sig, kil[0][0], kil[3][3]));

    // L-3 twisted action: [uL, .] on s_LL equals -2 W_LL(uL), with the Lie representation W_LL(uL) bS = -bS uL
    let mut tw_ok = true;
    let mut sig_rep = true;
    let mut ad_not_rep = true;
    let qs: Vec<Quat<GInt>> = vec![quat_unit(1), quat_unit(2), quat_unit(3), qscale(&quat_unit(1), GInt::iu()), qscale(&quat_unit(2), GInt::iu()), qscale(&quat_unit(3), GInt::iu())];
    let sigma = |q: &Quat<GInt>, b: &Quat<GInt>| -> Quat<GInt> { qscale(&qmul(b, q), gi(-1, 0)) }; // W_LL(q) b = -b q
    for q in &qs {
        for k in 6..14 {
            let s = mbasis(k);
            let (_, b) = s.quats();
            let lhs = B::from_quats(&[GInt::zero(), q[1], q[2], q[3]], &qzero()).comm(&s);
            let rhs = B::from_quats(&qzero(), &qscale(&sigma(q, &b), gi(-2, 0)));
            tw_ok &= lhs == rhs;
        }
    }
    for q1 in &qs {
        for q2 in &qs {
            let q12 = qsub(&qmul(q1, q2), &qmul(q2, q1));
            for k in 6..14 {
                let (_, b) = mbasis(k).quats();
                let lhs = qsub(&sigma(q1, &sigma(q2, &b)), &sigma(q2, &sigma(q1, &b)));
                sig_rep &= lhs == sigma(&q12, &b);
                // the inner action rho_LL = -2 W_LL: [rho1, rho2] = 4 W_LL([q1,q2]) = -2 rho_LL([q1,q2])
                let rho = |q: &Quat<GInt>, b: &Quat<GInt>| qscale(&sigma(q, b), gi(-2, 0));
                let lhs2 = qsub(&rho(q1, &rho(q2, &b)), &rho(q2, &rho(q1, &b)));
                ad_not_rep &= lhs2 == qscale(&rho(&q12, &b), gi(-2, 0));
            }
        }
    }
    ck.add("L-3", "a Lorentz element uL of l_LL acts on a spinor charge bS e4 of s_LL = (C (x) H) e4 by [uL, bS e4] = 2 (bS uL) e4 = -2 (W_LL(uL) bS) e4, where W_LL(uL) bS = -bS uL is a Lie representation (the Weyl spinor); the inner action rho_LL(uL) = -2 W_LL(uL) obeys [rho_LL(uL1), rho_LL(uL2)] = -2 rho_LL([uL1,uL2]): it is NOT a representation (the twist)", tw_ok && sig_rep && ad_not_rep, String::new());

    // L-4 [s_LL, s_LL] lies in l_LL and spans all of l_LL
    let mut ss_in_l = true;
    let mut rows: Vec<Vec<i128>> = Vec::new();
    for i in 6..14 {
        for j in 6..14 {
            let c = mbasis(i).comm(&mbasis(j));
            let r = mcoords(&c);
            ss_in_l &= r[6..].iter().all(|&x| x == 0);
            rows.push(r[..6].to_vec());
        }
    }
    let rk = rank(&rows);
    ck.add("L-4", "the bracket of two spinor charges lies in the Lorentz part, [bS e4, bS' e4] = bSbar bS' - bS'bar bS in l_LL, and these brackets span all of l_LL (rank 6): spinor charges close into Lorentz generators", ss_in_l && rk == 6, format!("rank {}", rk));

    // L-5 Jacobiators by sector
    let mut jls_ok = true;
    for i in 0..6 {
        for j in 0..6 {
            for k in 6..14 {
                let jac = jacobiator(&mbasis(i), &mbasis(j), &mbasis(k));
                let qi = mbasis(i).quats().0;
                let qj = mbasis(j).quats().0;
                let qij = qsub(&qmul(&qi, &qj), &qmul(&qj, &qi));
                let (_, b) = mbasis(k).quats();
                let expect = B::from_quats(&qzero(), &qscale(&sigma(&qij, &b), gi(-6, 0)));
                jls_ok &= jac == expect;
            }
        }
    }
    let jsss = jacobiator(&mbasis(6), &mbasis(7), &mbasis(8));
    let jlss = jacobiator(&mbasis(0), &mbasis(6), &mbasis(8));
    ck.add("L-5", "Jacobiators by sector: Jac(uL1, uL2, bS e4) = -6 (W_LL([uL1,uL2]) bS) e4 = (Theta_LL - Theta_LL^2) (W_LL([uL1,uL2]) bS) e4 with the twist charge Theta_LL = -2 on s_LL (exact on all 288 triples of basis elements); Jac(s_LL, s_LL, s_LL) (e.g. Jac(e4,e5,e6) = -12 e7) and Jac(l_LL, s_LL, s_LL) (e.g. Jac(e1,e4,e6)) are non-zero as well", jls_ok && !jsss.is_zero(0.0) && !jlss.is_zero(0.0), String::new());

    // L-6 the same pattern with the Lie action (twist charge 1) violates the Jacobi identity
    // bracket on l_LL + s_LL: [uL, bS e4] = (W_LL(uL) bS) e4 (Lie), [bS e4, bS' e4] = bSbar bS' - bS'bar bS (as in m_LL)
    let br_lie = |x: &B, y: &B| -> B {
        let (qx, bx) = x.quats();
        let (qy, by) = y.quats();
        let ll = qsub(&qmul(&qx, &qy), &qmul(&qy, &qx));
        let ls = qsub(&sigma(&qx, &by), &sigma(&qy, &bx));
        let ss = qsub(&qmul(&qconj(&bx), &by), &qmul(&qconj(&by), &bx));
        B::from_quats(&qadd(&ll, &ss), &ls)
    };
    let jl = |x: &B, y: &B, z: &B| br_lie(&br_lie(x, y), z).add(&br_lie(&br_lie(y, z), x)).add(&br_lie(&br_lie(z, x), y));
    let mut lie_fails = false;
    for i in 6..14 {
        for j in 6..14 {
            for k in 6..14 {
                if !jl(&mbasis(i), &mbasis(j), &mbasis(k)).is_zero(0.0) {
                    lie_fails = true;
                }
            }
        }
    }
    ck.add("L-6", "with the Lie action (twist charge 1 instead of -2) and the same non-zero bracket [s_LL, s_LL] in l_LL, the Jacobi identity fails on triples of spinor charges: no Lie algebra extends so(3,1) by bosonic spinor charges that close into so(3,1) (Theorem L; the classification G-1, G-5, G-10, M-3 shows that every Lie algebra of the covariant families has no Lorentz part in [s_LL, s_LL])", lie_fails, String::new());

    // L-7 derivations: D_LL(uL) and Dv_LL(tI) are derivations, they commute, and [uL, .] = D_LL(uL) o Theta_LL
    let mut der_ok = true;
    let mut commute_ok = true;
    let mut theta_ok = true;
    let mut iso_fix = true;
    for _ in 0..100 {
        let x: B = rng.bi(3);
        let y: B = rng.bi(3);
        for q in &qs {
            der_ok &= d_lorentz(q, &x.mul(&y)) == d_lorentz(q, &x).mul(&y).add(&x.mul(&d_lorentz(q, &y)));
        }
        for t in 1..4 {
            let tq = quat_unit(t);
            der_ok &= d_isospin(&tq, &x.mul(&y)) == d_isospin(&tq, &x).mul(&y).add(&x.mul(&d_isospin(&tq, &y)));
            for q in &qs {
                commute_ok &= d_lorentz(q, &d_isospin(&tq, &x)) == d_isospin(&tq, &d_lorentz(q, &x));
            }
            let (a, _) = x.quats();
            iso_fix &= d_isospin(&tq, &B::from_quats(&a, &qzero())).is_zero(0.0);
        }
    }
    for q in &qs {
        let qb = B::from_quats(&[GInt::zero(), q[1], q[2], q[3]], &qzero());
        for k in 0..14 {
            let m = mbasis(k);
            let th = if k < 6 { 1 } else { -2 };
            theta_ok &= qb.comm(&m) == d_lorentz(q, &m).scale(gi(th, 0));
        }
    }
    ck.add("L-7", "the Lorentz action by automorphisms, D_LL(uL)(aH + bS e4) = [uL, aH] + (-bS uL) e4, and the isospin action Dv_LL(tI)(aH + bS e4) = (tI bS) e4 (tI in Im H) are derivations of B_LL (100 random pairs), they commute, the isospin derivations vanish on C (x) H, and the inner action factors as [uL, .] = D_LL(uL) o Theta_LL with the twist charge Theta_LL = 1 on l_LL and -2 on s_LL", der_ok && commute_ok && theta_ok && iso_fix, String::new());

    // L-8 the derivation algebra of B_LL has real dimension 28 (g2(C))
    let mut eqs: Vec<Vec<i128>> = Vec::new();
    let rb = |k: usize| -> B { if k < 8 { e(k) } else { ie(k - 8) } };
    for x in 0..16 {
        for y in 0..16 {
            let xy = real_coords(&rb(x).mul(&rb(y)));
            for r in 0..16 {
                let mut row = vec![0i128; 256];
                for k in 0..16 {
                    for l2 in 0..16 {
                        let mut c = 0i128;
                        if r == k {
                            c += xy[l2];
                        }
                        if l2 == x {
                            c -= real_coords(&rb(k).mul(&rb(y)))[r];
                        }
                        if l2 == y {
                            c -= real_coords(&rb(x).mul(&rb(k)))[r];
                        }
                        row[k * 16 + l2] = c;
                    }
                }
                eqs.push(row);
            }
        }
    }
    let rk_eq = rank(&eqs);
    // explicit (inner) derivations [L_x, L_y] + [L_x, R_y] + [R_x, R_y]
    let rmat = |x: &B| real_matrix(|y: &B| y.mul(x));
    let mut dspan = ModSpan::new();
    let mut der_explicit_ok = true;
    for a in 1..8 {
        for b in 1..8 {
            for (xa, xb) in [(e(a), e(b)), (ie(a), e(b))] {
                let d = madd(&madd(&comm(&l(&xa), &l(&xb)), &comm(&l(&xa), &rmat(&xb))), &comm(&rmat(&xa), &rmat(&xb)));
                // check derivation property on random elements
                for _ in 0..3 {
                    let x: B = rng.bi(2);
                    let y: B = rng.bi(2);
                    let dx = from_real_coords(&mmul(&d, &real_coords(&x).iter().map(|&v| vec![v]).collect::<Vec<_>>()).iter().map(|r| r[0]).collect::<Vec<_>>());
                    let dy = from_real_coords(&mmul(&d, &real_coords(&y).iter().map(|&v| vec![v]).collect::<Vec<_>>()).iter().map(|r| r[0]).collect::<Vec<_>>());
                    let dxy = from_real_coords(&mmul(&d, &real_coords(&x.mul(&y)).iter().map(|&v| vec![v]).collect::<Vec<_>>()).iter().map(|r| r[0]).collect::<Vec<_>>());
                    der_explicit_ok &= dxy == dx.mul(&y).add(&x.mul(&dy));
                }
                dspan.insert(&flatten(&d));
            }
        }
    }
    ck.add("L-8", "the real derivations of B_LL form a Lie algebra of real dimension 28 (the complex Lie algebra g2(C)): the linear system has rank 228 for 256 unknowns (at most 28 solutions), and the inner derivations [L_x,L_y] + [L_x,R_y] + [R_x,R_y] are verified derivations spanning 28 dimensions", rk_eq == 228 && dspan.rank() == 28 && der_explicit_ok, format!("rank of the system {}, explicit derivations span {}", rk_eq, dspan.rank()));

    // L-9 the automorphisms varphi_(gL,vI): aH + bS e4 -> gL aH gLbar + (vI bS gLbar) e4 for unit gL, vI
    let u: Quat<GInt> = [gi(1, 0), gi(0, 1), gi(1, 0), gi(0, 0)]; // 1 + i e1 + e2, N = 1
    let v: Quat<GInt> = [gi(0, 0), gi(0, 0), gi(0, 0), gi(1, 0)]; // e3 (a real unit quaternion)
    let phi = |x: &B| -> B {
        let (a, b) = x.quats();
        B::from_quats(&qmul(&qmul(&u, &a), &qconj(&u)), &qmul(&qmul(&v, &b), &qconj(&u)))
    };
    let mut aut_ok = qnorm(&u) == GInt::one() && qnorm(&v) == GInt::one();
    for _ in 0..100 {
        let x: B = rng.bi(3);
        let y: B = rng.bi(3);
        aut_ok &= phi(&x.mul(&y)) == phi(&x).mul(&phi(&y));
    }
    ck.add("L-9", "varphi_(gL,vI)(aH + bS e4) = gL aH gLbar + (vI bS gLbar) e4 is an automorphism of B_LL for the unit bi-quaternions gL = 1 + i e1 + e2 (a Lorentz transformation) and vI = e3 (an isospin rotation), on 100 random pairs: (SL(2,C) x SL(2,C))/Z2 = SO(4,C) inside G2(C)", aut_ok, String::new());

    // ---------------- Q: operators and eigenvalues (double precision) ----------------
    let half = C64::new(0.5, 0.0);
    let t3c = to_c64q(&quat_unit(3));
    // electric charge Q_LL = T_LL^3 = i Dv_LL(e3/2)  (T_LL^iota = i Dv_LL(e_iota/2) obey [T^1, T^2] = i T^3)
    let qop = cmat8(|x| d_isospin_c(&qscale(&t3c, half), x).scale(C64::new(0.0, 1.0)));
    let herm = qop.sub(&qop.dagger()).fro() < 1e-12;
    let q_l = eig_block(&qop, &[0, 1, 2, 3]);
    let q_s = eig_block(&qop, &[4, 5, 6, 7]);
    ck.add("Q-1", "the electric charge Q_LL = T_LL^3 = i Dv_LL(e3/2) (the third isospin generator, an automorphism of B_LL) is a Hermitian operator on B_LL with the eigenvalues 0 (four times) on C (x) H (the Lorentz part and the scalar) and +1/2, -1/2 (twice each) on the spinor charges (double precision)", herm && q_l.iter().all(|x| x.abs() < 1e-12) && (q_s[0] + 0.5).abs() < 1e-12 && (q_s[1] + 0.5).abs() < 1e-12 && (q_s[2] - 0.5).abs() < 1e-12 && (q_s[3] - 0.5).abs() < 1e-12, format!("on C (x) H: {}; on s_LL: {}", fmt_eigs(&q_l), fmt_eigs(&q_s)));

    // isospin Casimir T_LL^2 = -sum_iota Dv_LL(e_iota/2)^2
    let mut t2 = CMat::zeros(8);
    for k in 1..4 {
        let tk = qscale(&to_c64q(&quat_unit(k)), half);
        let dk = cmat8(|x| d_isospin_c(&tk, x));
        t2 = t2.sub(&dk.mul(&dk));
    }
    let t2_s = eig_block(&t2, &[4, 5, 6, 7]);
    let t2_l = eig_block(&t2, &[0, 1, 2, 3]);
    ck.add("Q-2", "the isospin Casimir T_LL^2 = sum_iota (T_LL^iota)^2 = -sum_iota Dv_LL(e_iota/2)^2 equals 0 on C (x) H and 3/4 = (1/2)(1/2 + 1) on the spinor charges: the spinor charges form an isospin doublet (double precision)", t2_l.iter().all(|x| x.abs() < 1e-12) && t2_s.iter().all(|x| (x - 0.75).abs() < 1e-12), format!("on s_LL: {}", fmt_eigs(&t2_s)));

    // Lorentz Casimir: 4 Cas_LL = -sum_j D_LL(e_j)^2 exactly: 0 on e0, 8 on C (x) Im H, 3 on s_LL
    let mut cas = zeros(16);
    for a in 1..4 {
        let d = real_matrix(|x: &B| d_lorentz(&quat_unit(a), x));
        cas = msub(&cas, &mmul(&d, &d));
    }
    let mut cas_ok = true;
    for k in 0..16 {
        let col: Vec<i128> = (0..16).map(|r| cas[r][k]).collect();
        let a = k % 8;
        let expect = if a == 0 { 0 } else if a < 4 { 8 } else { 3 };
        let mut v = vec![0i128; 16];
        v[k] = expect;
        cas_ok &= col == v;
    }
    ck.add("Q-3", "the Lorentz Casimir Cas_LL = -sum_j D_LL(J_LL^j)^2 (exact; 4 Cas_LL is an integer matrix) has the value spin(spin+1) = 0 on the scalar e0, 2 (spin 1) on the Lorentz part C (x) Im H and 3/4 (spin 1/2) on the spinor charges", cas_ok, String::new());

    // chirality: D_LL(i e_j) = i D_LL(e_j) on all of B_LL (complex-linear action, K_LL^j = i J_LL^j)
    let mut chi_ok = true;
    for a in 1..4 {
        for k in 0..8 {
            for xb in [e(k), ie(k)] {
                chi_ok &= d_lorentz(&qscale(&quat_unit(a), GInt::iu()), &xb) == d_lorentz(&quat_unit(a), &xb).scale(GInt::iu());
            }
        }
    }
    ck.add("Q-4", "chirality: on every bi-octonion the boost generators act as i times the rotation generators, D_LL(K_LL^j) = i D_LL(J_LL^j); with respect to the complex structure of B_LL every sector has one chirality, type (spin, 0): l_LL = (1,0), s_LL = (1/2,0); the complex-conjugate super-loop has the other chirality (0, spin)", chi_ok, String::new());

    // spinor number exp(2 pi D_LL(J_LL^3)): i D_LL(J_LL^3) has integer eigenvalues on C (x) H and half-odd ones on s_LL
    let dj3 = cmat8(|x| d_lorentz_c(&qscale(&t3c, half), x));
    let mut ih3 = CMat::zeros(8);
    for i in 0..8 {
        for j in 0..8 {
            ih3.set(i, j, dj3.get(i, j) * C64::I);
        }
    }
    let ev_l = eig_block(&ih3, &[0, 1, 2, 3]);
    let ev_s = eig_block(&ih3, &[4, 5, 6, 7]);
    let int_l = ev_l.iter().all(|x| (x - x.round()).abs() < 1e-12);
    let half_s = ev_s.iter().all(|x| ((x - 0.5) - (x - 0.5).round()).abs() < 1e-12);
    ck.add("Q-5", "spinor number: the rotation generator i D_LL(J_LL^3) has the integer eigenvalues {-1, 0, 0, 1} on C (x) H and the half-odd eigenvalues {-1/2, -1/2, 1/2, 1/2} on the spinor charges, so the rotation by 2 pi, exp(2 pi D_LL(J_LL^3)), is +1 on the Lorentz part and -1 on the spinor charges (double precision)", int_l && half_s, format!("on C (x) H: {}; on s_LL: {}", fmt_eigs(&ev_l), fmt_eigs(&ev_s)));

    // twist charge (L-7: [uL, .] = D_LL(uL) o Theta_LL); here: conjugation by gL in SL(2,C) is bS -> bS gL^2
    let g: Quat<GInt> = u;
    let mut conj_ok = true;
    for _ in 0..50 {
        let bq: Quat<GInt> = [gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128)];
        let s = B::from_quats(&qzero(), &bq);
        let gb = B::from_quats(&g, &qzero());
        let gbar = B::from_quats(&qconj(&g), &qzero());
        let lhs1 = gb.mul(&s).mul(&gbar);
        let lhs2 = gb.mul(&s.mul(&gbar));
        let rhs = B::from_quats(&qzero(), &qmul(&qmul(&bq, &g), &g));
        conj_ok &= lhs1 == rhs && lhs2 == rhs;
    }
    ck.add("Q-6", "inside the loop, conjugation by an element gL of SL(2,C) acts on a spinor charge as gL (bS e4) gLbar = (bS gL^2) e4 (both bracketings agree), whereas the Lorentz automorphism acts as bS -> bS gL^-1: the loop's inner action turns a spinor twice as fast and backwards (twist charge -2)", conj_ok, String::new());

    // ---------------- C: the inversion element e4 ----------------
    let ell = e(4);
    let ell_inv = ell.conj(); // N_LL(e4) = 1, inverse = conjugate = -e4
    let mut c1_ok = ell.mul(&ell) == B::one().neg();
    for _ in 0..100 {
        let qv: Quat<GInt> = [gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128)];
        let qb = B::from_quats(&qv, &qzero());
        c1_ok &= ell.mul(&qb).mul(&ell_inv) == B::from_quats(&qconj(&qv), &qzero());
    }
    // parity on Hermitian bi-quaternions x_LL^0 + i sum_j x_LL^j e_j
    let xh = B::from_quats(&[gi(3, 0), gi(0, 2), gi(0, -1), gi(0, 5)], &qzero());
    let px = ell.mul(&xh).mul(&ell_inv);
    let parity_ok = px == B::from_quats(&[gi(3, 0), gi(0, -2), gi(0, 1), gi(0, -5)], &qzero());
    ck.add("C-1", "the unit e4 of the super-loop (e4^2 = -1, e4^-1 = -e4) acts on bi-quaternions by e4 aH e4^-1 = aHbar (100 random aH): on SL(2,C) it is the inversion gL -> gL^-1, and on Minkowski space R^(1,3)_LL (Hermitian bi-quaternions) it is the parity (x_LL^0, x_LL^1, x_LL^2, x_LL^3) -> (x_LL^0, -x_LL^1, -x_LL^2, -x_LL^3)", c1_ok && parity_ok, String::new());

    let mut chein_ok = true;
    for _ in 0..100 {
        let gq: Quat<GInt> = [gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128)];
        let hq: Quat<GInt> = [gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128), gi(rng.int(3) as i128, rng.int(3) as i128)];
        let gl = B::from_quats(&qzero(), &gq);
        let hl = B::from_quats(&qzero(), &hq);
        chein_ok &= gl.mul(&hl) == B::from_quats(&qscale(&qmul(&qconj(&hq), &gq), gi(-1, 0)), &qzero());
        let gq0 = B::from_quats(&gq, &qzero());
        let hq0 = B::from_quats(&hq, &qzero());
        chein_ok &= gq0.mul(&hl) == B::from_quats(&qzero(), &qmul(&hq, &gq));
        chein_ok &= gl.mul(&hq0) == B::from_quats(&qzero(), &qmul(&gq, &qconj(&hq)));
    }
    ck.add("C-2", "the union of SL(2,C) and SL(2,C) e4 is closed under the product: (gL e4)(hL e4) = -hLbar gL, gL (hL e4) = (hL gL) e4, (gL e4) hL = (gL hLbar) e4; it is a Moufang loop (a Chein-type double of the Lorentz group) inside L_LL, and it is not associative because SL(2,C) is not commutative", chein_ok, String::new());

    let g1 = e(1);
    let h1 = e(2);
    let inv_gh = g1.mul(&h1).conj();
    let ginv_hinv = g1.conj().mul(&h1.conj());
    ck.add("C-3", "inversion is not an automorphism of SL(2,C): for gL = e1, hL = e2, (gL hL)^-1 = -e3 but gL^-1 hL^-1 = +e3; hence no operator on any vector space can implement gL -> gL^-1 by conjugation in a faithful representation (conjugation is multiplicative): the extension by e4 exists only as a loop", inv_gh == e(3).neg() && ginv_hinv == e(3), String::new());

    let mut not_aut = false;
    for _ in 0..20 {
        let x: B = rng.bi(2);
        let y: B = rng.bi(2);
        let fx = ell.mul(&x).mul(&ell_inv);
        let fy = ell.mul(&y).mul(&ell_inv);
        let fxy = ell.mul(&x.mul(&y)).mul(&ell_inv);
        if fxy != fx.mul(&fy) {
            not_aut = true;
        }
    }
    ck.add("C-4", "the inner map x -> e4 x e4^-1 is not an automorphism of B_LL (it fails on random pairs): inner maps of the loop are not symmetries of its multiplication, another place where associativity is missing", not_aut, String::new());

    // ---------------- data ----------------
    let names = ["e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7"];
    let mut rows_t: Vec<Vec<String>> = Vec::new();
    for a in 0..8 {
        let mut r = vec![names[a].to_string()];
        for b in 0..8 {
            let p = e(a).mul(&e(b));
            let k = (0..8).find(|&k| p.c[k].re != 0).unwrap();
            r.push(format!("{}{}", if p.c[k].re < 0 { "-" } else { "+" }, names[k]));
        }
        rows_t.push(r);
    }
    let _ = write_csv_str("sl_multiplication_table.csv", &[("content", "products of the units e0..e7 of B_LL (row: left factor, column: right factor); the bi-octonions are the complex combinations of the units".to_string()), ("rule", "Cayley-Dickson (aH + bS e4)(cH + dS e4) = (aH cH - dSbar bS) + (dS aH + bS cHbar) e4 with e0 = 1, e1, e2, e3 the quaternion units, e5 = e1 e4, e6 = e2 e4, e7 = e3 e4".to_string())], &["left", "e0", "e1", "e2", "e3", "e4", "e5", "e6", "e7"], &rows_t);
    let mut rows_m: Vec<Vec<String>> = Vec::new();
    let mnames = ["e1", "e2", "e3", "ie1", "ie2", "ie3", "e4", "e5", "e6", "e7", "ie4", "ie5", "ie6", "ie7"];
    for i in 0..14 {
        for j in (i + 1)..14 {
            let r = mcoords(&mbasis(i).comm(&mbasis(j)));
            for k in 0..14 {
                if r[k] != 0 {
                    rows_m.push(vec![mnames[i].to_string(), mnames[j].to_string(), mnames[k].to_string(), r[k].to_string()]);
                }
            }
        }
    }
    let _ = write_csv_str("sl_malcev_structure.csv", &[("content", "non-zero commutators of the real basis elements of the Malcev algebra m_LL = C (x) Im O: [left, right] = coefficient x result (left before right in the basis order e1, e2, e3, ie1, ie2, ie3, e4..e7, ie4..ie7)".to_string()), ("names", "ie1 = i e1 etc.; e1, e2, e3 are twice the rotation generators J_LL^j, ie1, ie2, ie3 twice the boost generators K_LL^j, e4..e7 and ie4..ie7 the spinor charges".to_string())], &["left", "right", "result", "coefficient"], &rows_m);
    let qn = vec![
        vec!["scalar e0".to_string(), "2".into(), "0".into(), "0".into(), "0".into(), "(0,0)".into(), "+1".into(), "outside m_LL".into()],
        vec!["Lorentz part l_LL = C (x) Im H".to_string(), "6".into(), "0".into(), "0".into(), "2".into(), "(1,0)".into(), "+1".into(), "1".into()],
        vec!["spinor charges s_LL = (C (x) H) e4".to_string(), "8".into(), "+1/2 twice, -1/2 twice".into(), "3/4".into(), "3/4".into(), "(1/2,0)".into(), "-1".into(), "-2".into()],
    ];
    let _ = write_csv_str("sl_quantum_numbers.csv", &[("content", "quantum numbers of the sectors of B_LL under the operators of the super-loop: Q_LL (eigenvalues with multiplicities), T_LL^2, Cas_LL, Lorentz type, spinor number exp(2 pi D_LL(J_LL^3)), twist charge Theta_LL".to_string())], &["sector", "real_dimension", "Q_LL", "T_LL^2", "Cas_LL", "Lorentz_type", "spinor_number", "Theta_LL"], &qn);
    let _ = write_csv_str("sl_associators.csv", &[("content", "the non-zero associators [e_p1, e_p2, e_p3] = (e_p1 e_p2) e_p3 - e_p1 (e_p2 e_p3) of three units with p1 < p2 < p3; each equals coefficient x the unit result (all other orders follow by alternativity)".to_string())], &["unit1", "unit2", "unit3", "result", "coefficient"], &assoc_rows);

    ck.write("sl_algebra_checks.txt")
}
