//! Command `manifolds`: the manifolds that carry the Lorentz super-loop L_LL.
//!
//!   N-1  R^(1,7)_LL = Hermitian bi-octonions x_LL^0 + i sum_p x_LL^p e_p (the 7+1 manifold) and R^(1,3)_LL in it
//!   N-2  the split octonions Os = H + (i H) e4 inside B_LL (the 4+4 manifold)
//!   N-3  the space s_LL of spinor charges with Re N_LL and Im N_LL: two quadratic forms of signature (4,4)
//!   N-4  the conjugations x -> gL x gL^dag by loop elements generate so(1,7) on R^(1,7)_LL
//!   N-5  the loop's own Lorentz subgroup acts on R^(1,7)_LL by conjugation with a closure of
//!        dimension 9 = so(3,1) + so(3): conjugation by the loop is not a group action
//!   N-6  the Lie closures of the left multiplications: so(8,C) (56), so(8) (28), so(4,4) (28)
//!   N-7  the loop manifold: the frame fields X_x(w) = w x are tangent to N_LL(w) = 1, and
//!        [X_x, X_y](w) = w [x,y] + 2 [w,x,y]: non-constant structure functions
//!   N-8  the 7-sphere S^7 as Spin(7)/G2: spin(7) = 21, stabilizer of 1 = g2 = 14, orbit 7
//!   N-9  the split real form: Im Os contains sl(2,R) = so(2,1)
//! Data: sl_manifolds.csv; report sl_manifolds_checks.txt.

use crate::algebra::{e, ie};
use crate::bioct::*;
use crate::checks::Checks;
use crate::exact::*;
use spacetimes::io::write_csv_str;

type B = Bi<GInt>;

fn gi(re: i128, im: i128) -> GInt {
    GInt { re, im }
}

/// basis of R^(1,7)_LL: 1 and i e_p (p = 1..7)
fn herm(a: usize) -> B {
    if a == 0 {
        e(0)
    } else {
        ie(a)
    }
}

/// coordinates of a Hermitian bi-octonion in the basis herm(0..7) (panics if not Hermitian)
fn hcoords(x: &B) -> Vec<i128> {
    assert!(x.c[0].im == 0 && (1..8).all(|a| x.c[a].re == 0), "not Hermitian: {:?}", x);
    let mut v = vec![x.c[0].re];
    for a in 1..8 {
        v.push(x.c[a].im);
    }
    v
}

/// polarized complex norm N_LL(x, y) = sum_p (coefficient p of x)(coefficient p of y)
fn npol(x: &B, y: &B) -> GInt {
    let mut s = GInt::zero();
    for k in 0..8 {
        s = s.add(x.c[k].mul(y.c[k]));
    }
    s
}

/// matrix of a linear map on R^(1,7) given in the Hermitian basis
fn hmatrix<F: Fn(&B) -> B>(f: F) -> IM {
    let mut m = zeros(8);
    for j in 0..8 {
        let v = hcoords(&f(&herm(j)));
        for i in 0..8 {
            m[i][j] = v[i];
        }
    }
    m
}

/// is M in so(eta) for diagonal eta: (eta M)^T = -(eta M)
fn in_so(m: &IM, eta: &[i128]) -> bool {
    let n = m.len();
    for i in 0..n {
        for j in 0..n {
            if eta[i] * m[i][j] != -(eta[j] * m[j][i]) {
                return false;
            }
        }
    }
    true
}

pub fn run() -> bool {
    let mut ck = Checks::new("Revision_SuperLoop, command `manifolds`: the manifolds that carry the Lorentz super-loop");
    let mut rng = Rng::new(424242);
    let mut rows: Vec<Vec<String>> = Vec::new();

    // N-1 signatures of the Hermitian slices
    let gram = |basis: &Vec<B>| -> IM {
        let n = basis.len();
        let mut g = zeros(n);
        for i in 0..n {
            for j in 0..n {
                let v = npol(&basis[i], &basis[j]);
                assert!(v.im == 0);
                g[i][j] = v.re;
            }
        }
        g
    };
    let h8: Vec<B> = (0..8).map(herm).collect();
    let h4: Vec<B> = (0..4).map(herm).collect();
    let s8 = signature(&gram(&h8));
    let s4 = signature(&gram(&h4));
    let mut herm_ok = true;
    for _ in 0..50 {
        // Hermitian elements: the coefficient of e0 real, the other coefficients imaginary; N_LL real
        let mut x = B::zero();
        x.c[0] = gi(rng.int(4) as i128, 0);
        for a in 1..8 {
            x.c[a] = gi(0, rng.int(4) as i128);
        }
        herm_ok &= x.dag() == x && x.norm().im == 0;
    }
    ck.add("N-1", "the Hermitian bi-octonions x = x_LL^0 + i sum_p x_LL^p e_p (x^dag = x, real x_LL^0, x_LL^p, p = 1..7) form the 7+1 Minkowski space R^(1,7)_LL: the complex norm N_LL is real on them, with signature (1 positive, 7 negative); the Hermitian bi-quaternions (p = 1,2,3) form R^(1,3)_LL, signature (1,3), on which SL(2,C) acts by x -> gL x gL^dag", s8 == (1, 7, 0) && s4 == (1, 3, 0) && herm_ok, format!("R^(1,7)_LL: {:?}, R^(1,3)_LL: {:?}", s8, s4));
    rows.push(vec!["R^(1,7)_LL = Hermitian bi-octonions".into(), "8".into(), "(1,7)".into(), "the 7+1 manifold; the conjugations by loop elements generate SO(1,7)".into()]);
    rows.push(vec!["R^(1,3)_LL = Hermitian bi-quaternions".into(), "4".into(), "(1,3)".into(), "Minkowski space of the loop's SL(2,C)".into()]);

    // N-2 the split octonions Os = H + (i H) e4
    let os_basis: Vec<B> = (0..8).map(|a| if a < 4 { e(a) } else { ie(a) }).collect();
    let in_os = |x: &B| -> bool { (0..4).all(|a| x.c[a].im == 0) && (4..8).all(|a| x.c[a].re == 0) };
    let mut os_ok = true;
    for a in 0..8 {
        for b in 0..8 {
            let p = os_basis[a].mul(&os_basis[b]);
            os_ok &= in_os(&p);
            // compare with the real split Cayley-Dickson product (sign +1) under (aH, bS) -> aH + (i bS) e4
            let mut ra = Bi::<GInt>::zero();
            let mut rb = Bi::<GInt>::zero();
            let ea = if a < 4 { a } else { a };
            let eb = if b < 4 { b } else { b };
            ra.c[ea] = GInt::one();
            rb.c[eb] = GInt::one();
            let sp = ra.mul_s(&rb, 1);
            // map the split product (real coefficients) to Os: the coefficients of e4..e7 get a factor i
            let mut mapped = B::zero();
            for k in 0..8 {
                mapped.c[k] = if k < 4 { sp.c[k] } else { sp.c[k].mul(GInt::iu()) };
            }
            os_ok &= mapped == p;
        }
    }
    let sos = signature(&gram(&os_basis));
    ck.add("N-2", "the split octonions sit inside B_LL as Os = H + (i H) e4: this real 8-dimensional subspace is closed under the product, the map from the split Cayley-Dickson algebra (pairs (aH, bS) of real quaternions with the sign +1) to B_LL, (aH, bS) -> aH + (i bS) e4, is an isomorphism onto Os, and N_LL restricted to Os has signature (4,4): the 4+4 manifold is the real form Os of the super-loop's algebra; its unit elements form the pseudo-sphere S^(3,4), a Moufang loop", os_ok && sos == (4, 4, 0), format!("signature {:?}", sos));
    rows.push(vec!["R^(4,4) = split octonions Os = H + (i H) e4".into(), "8".into(), "(4,4)".into(), "the 4+4 manifold; its unit loop is S^(3,4)".into()]);

    // N-3 the space s_LL of spinor charges: Re N_LL and Im N_LL
    let sb: Vec<B> = (4..8).map(e).chain((4..8).map(ie)).collect();
    let mut gre = zeros(8);
    let mut gim = zeros(8);
    for i in 0..8 {
        for j in 0..8 {
            let v = npol(&sb[i], &sb[j]);
            gre[i][j] = v.re;
            gim[i][j] = v.im;
        }
    }
    let (sr, si) = (signature(&gre), signature(&gim));
    ck.add("N-3", "the space of spinor charges s_LL (real dimension 8) carries the quadratic forms Re N_LL and Im N_LL (invariant under the Lorentz and isospin automorphisms, because N_LL is multiplicative), both of signature (4,4): the base of the fibration of L_LL over s_LL (aH + bS e4 -> bS, fibre a copy of SL(2,C) wherever N_LL(aH) = 1 - N_LL(bS) is not zero) is a 4+4 space", sr == (4, 4, 0) && si == (4, 4, 0), format!("Re N_LL {:?}, Im N_LL {:?}", sr, si));
    rows.push(vec!["s_LL = spinor charges with Re N_LL, Im N_LL".into(), "8".into(), "(4,4) and (4,4)".into(), "base of the super-loop over its spinor directions".into()]);

    // N-4 conjugations by loop elements on R^(1,7)_LL
    let eta8: Vec<i128> = (0..8).map(|a| if a == 0 { 1 } else { -1 }).collect();
    let mut gens: Vec<IM> = Vec::new();
    let mut in_so8 = true;
    for a in 1..8 {
        let ea = e(a);
        let iea = ie(a);
        let r = hmatrix(|x| ea.mul(x).sub(&x.mul(&ea)));
        let bm = hmatrix(|x| iea.mul(x).add(&x.mul(&iea)));
        in_so8 &= in_so(&r, &eta8) && in_so(&bm, &eta8);
        gens.push(r);
        gens.push(bm);
    }
    let (dim17, _) = lie_closure(&gens);
    ck.add("N-4", "on the 7+1 manifold the one-parameter conjugations x -> gL x gL^dag by the loop elements gL = exp(theta e_p) (rotations) and gL = exp(theta i e_p) (boosts), p = 1..7, have the generators x -> [e_p, x] and x -> i (e_p x + x e_p), which lie in so(1,7), and their Lie closure is all of so(1,7) (dimension 28, exact): the super-loop acts on R^(1,7)_LL through the Lorentz group SO(1,7)", in_so8 && dim17 == 28, format!("closure dimension {}", dim17));

    // N-5 the loop's SL(2,C) acting by conjugation on R^(1,7)_LL: closure 9
    let (dim_l, basis_l) = lie_closure(&gens[0..6].to_vec());
    // restriction to R^(1,3)_LL (first 4 coordinates) has dimension 6
    let mut span4 = ExactSpan::new();
    for m in &basis_l {
        let mut v = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                v.push(m[i][j]);
            }
        }
        span4.insert(&v);
    }
    ck.add("N-5", "conjugation by the loop's own Lorentz subgroup (p = 1,2,3) is not a group action on the 7+1 manifold: its six generators preserve R^(1,3)_LL and act on it as so(3,1), but their Lie closure on R^(1,7)_LL has dimension 9 = so(3,1) + so(3) (exact), the extra so(3) rotating only the four directions i e4..i e7 (there the boosts act trivially and the rotations by the twisted spinor action)", dim_l == 9 && span4.rank() == 6, format!("closure on R^(1,7)_LL: {}, restricted to R^(1,3)_LL: {}", dim_l, span4.rank()));

    // N-6 Lie closures of left multiplications
    let mut lgens: Vec<IM> = Vec::new();
    for a in 1..8 {
        lgens.push(real_matrix(|y: &B| e(a).mul(y)));
        lgens.push(real_matrix(|y: &B| ie(a).mul(y)));
    }
    // each L_x is complex-linear and antisymmetric for Re N_LL (so it preserves the complex bilinear form N_LL)
    let jc = real_matrix(|y: &B| y.scale(GInt::iu()));
    let mut gre16 = zeros(16);
    for i in 0..16 {
        for j in 0..16 {
            let x = from_real_coords(&(0..16).map(|k| if k == i { 1 } else { 0 }).collect::<Vec<_>>());
            let y = from_real_coords(&(0..16).map(|k| if k == j { 1 } else { 0 }).collect::<Vec<_>>());
            gre16[i][j] = npol(&x, &y).re;
        }
    }
    let mut cl_ok = true;
    for m in &lgens {
        cl_ok &= is_zero(&comm(m, &jc));
        let a = mmul(&gre16, m);
        cl_ok &= is_zero(&madd(&a, &transpose(&a)));
    }
    let (d56, _) = lie_closure(&lgens);
    let rgens: Vec<IM> = (1..8).map(|a| {
        // real octonions: 8x8 integer matrices of L_(e_a) on the real span of e_0..e_7
        let mut m = zeros(8);
        for j in 0..8 {
            let p = e(a).mul(&e(j));
            for i in 0..8 {
                m[i][j] = p.c[i].re;
            }
        }
        m
    }).collect();
    let (d28, _) = lie_closure(&rgens);
    let eta_s: Vec<i128> = (0..8).map(|a| if a < 4 { 1 } else { -1 }).collect();
    let sgens: Vec<IM> = (1..8).map(|a| {
        let mut m = zeros(8);
        let u = os_basis[a];
        for j in 0..8 {
            let p = u.mul(&os_basis[j]);
            for i in 0..8 {
                m[i][j] = if i < 4 { p.c[i].re } else { p.c[i].im };
            }
        }
        m
    }).collect();
    let s_in = sgens.iter().all(|m| in_so(m, &eta_s));
    let (d44, _) = lie_closure(&sgens);
    ck.add("N-6", "the left multiplications generate Lie groups (their closures, exact): by imaginary bi-octonions, so(8,C) of real dimension 56 (every L_x is complex-linear and preserves the complex norm N_LL); by imaginary octonions on R^8, so(8) (28); by imaginary split units on Os = R^(4,4), so(4,4) (28; each generator preserves the split norm): any linear action of the loop is an action of these LIE groups", cl_ok && d56 == 56 && d28 == 28 && d44 == 28 && s_in, format!("closures {} / {} / {}", d56, d28, d44));

    // N-7 frame fields on the loop manifold
    let mut frame_ok = true;
    let mut nonconst = false;
    for _ in 0..100 {
        let u: B = rng.bi(3);
        let mut x: B = rng.bi(3);
        let mut y: B = rng.bi(3);
        x.c[0] = GInt::zero();
        y.c[0] = GInt::zero();
        // tangency: N_LL(w, w x) = N_LL(w) (coefficient of e0 of x) = 0  (the point w is called u in the code)
        frame_ok &= npol(&u, &u.mul(&x)) == GInt::zero();
        // [X_x, X_y](w) = (wx)y - (wy)x = w[x,y] + 2 [w,x,y]
        let lhs = u.mul(&x).mul(&y).sub(&u.mul(&y).mul(&x));
        let rhs = u.mul(&x.comm(&y)).add(&u.assoc(&x, &y).scale(gi(2, 0)));
        frame_ok &= lhs == rhs;
        if !u.assoc(&x, &y).is_zero(0.0) {
            nonconst = true;
        }
    }
    let at1 = {
        let x = e(1);
        let y = e(4);
        let u = B::one();
        u.mul(&x).mul(&y).sub(&u.mul(&y).mul(&x)) == x.comm(&y)
    };
    ck.add("N-7", "the super-loop manifold (N_LL(w) = 1, a complex 7-sphere of real dimension 14) is parallelized by the frame fields X_x(w) = w x (x imaginary), which are tangent (N_LL(w, w x) = 0); their Lie brackets are [X_x, X_y](w) = w [x,y] + 2 [w,x,y] (100 random w, x, y): at w = 1 this is the Malcev bracket, elsewhere the associator makes the structure functions non-constant: the manifold is parallelizable but carries no Lie group structure", frame_ok && nonconst && at1, String::new());
    rows.push(vec!["super-loop L_LL: N_LL(w) = 1 in C^8".into(), "14".into(), "complex quadric".into(), "Spin(7,C)/G2(C); real forms S^7 and S^(3,4)".into()]);

    // N-8 S^7 = Spin(7)/G2: spin(7) spanned by L_(e_p) L_(e_p') (p < p') on R^8; the stabilizer of 1 is 14-dimensional
    let mut sp7: Vec<IM> = Vec::new();
    for a in 0..7 {
        for b in (a + 1)..7 {
            sp7.push(mmul(&rgens[a], &rgens[b]));
        }
    }
    let (d21, b21) = lie_closure(&sp7);
    // stabilizer of e0: the combinations of the basis matrices that annihilate e0 (nullity of that linear map)
    let cols: Vec<Vec<i128>> = b21.iter().map(|m| (0..8).map(|i| m[i][0]).collect()).collect();
    // rank of the 8 x 21 matrix whose columns are M_i e0
    let rk = rank(&cols);
    let stab = d21 - rk;
    ck.add("N-8", "the 7-sphere S^7 of unit octonions (the compact real form of the super-loop) is the homogeneous space Spin(7)/G2: the products L_(e_p) L_(e_p') (p < p') span spin(7) (dimension 21, exact) acting on R^8, the stabilizer of 1 has dimension 14 (g2) and the orbit of 1 has dimension 7", d21 == 21 && stab == 14 && rk == 7, format!("spin(7): {}, stabilizer: {}, orbit: {}", d21, stab, rk));

    // N-9 split real form: sl(2,R) = span{e1, i e4, i e5} inside Im Os
    let h1 = e(1);
    let h2 = ie(4);
    let h3 = ie(5);
    let br12 = h1.comm(&h2);
    let br23 = h2.comm(&h3);
    let br31 = h3.comm(&h1);
    // closure: brackets stay in span{e1, i e4, i e5}
    let span_ok = [br12, br23, br31].iter().all(|v| (0..8).all(|k| (k == 1 && v.c[k].im == 0) || (k == 4 && v.c[k].re == 0) || (k == 5 && v.c[k].re == 0) || (v.c[k].re == 0 && v.c[k].im == 0)));
    // Killing form signature of this 3-dim algebra
    let bas = [h1, h2, h3];
    let coord3 = |v: &B| -> Vec<i128> { vec![v.c[1].re, v.c[4].im, v.c[5].im] };
    let ad3 = |k: usize| -> IM {
        let mut m = zeros(3);
        for j in 0..3 {
            let c = coord3(&bas[k].comm(&bas[j]));
            for i in 0..3 {
                m[i][j] = c[i];
            }
        }
        m
    };
    let mut kil = zeros(3);
    for i in 0..3 {
        for j in 0..3 {
            let p = mmul(&ad3(i), &ad3(j));
            kil[i][j] = (0..3).map(|k| p[k][k]).sum();
        }
    }
    let sk = signature(&kil);
    ck.add("N-9", "the split real form: inside the imaginary split octonions the elements e1, i e4, i e5 span a Lie subalgebra whose Killing form has signature (2 positive, 1 negative): sl(2,R) = so(2,1), the Lorentz algebra of 2+1 dimensions; the unit split octonions S^(3,4) contain SL(2,R) = Spin(2,1): on the real 4+4 manifold the super-loop's Lorentz part is the 2+1 Lorentz group, and SO(3,1) requires the complex form", span_ok && sk == (2, 1, 0), format!("signature {:?}", sk));

    let _ = write_csv_str("sl_manifolds.csv", &[("content", "manifolds that carry the Lorentz super-loop".to_string())], &["manifold", "real_dimension", "signature", "role"], &rows);
    ck.write("sl_manifolds_checks.txt")
}
