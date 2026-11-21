//! Checks A: the bi-octonions, the super-loop, the manifold M31 and its Lorentz group (Section 2 of superfluid.md).
use sfcore::algebra::*;
use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::sigma;
use sfcore::report::{f, write_csv, Report};
use sfcore::rng::Rng;
use std::path::Path;

/// the Minkowski metric diag(1, -1, -1, -1)
const ETA: [f64; 4] = [1.0, -1.0, -1.0, -1.0];

pub fn run(rep: &mut Report, data: &Path) {
    let mut rng = Rng(20261120);

    // A-1: the quaternion units: e_j^2 = -1, e_1 e_2 = e_3 = -e_2 e_1 and cyclically
    let mut dev: f64 = 0.0;
    for j in 1..4 {
        let sq = q_mul(&q_unit(j), &q_unit(j));
        dev = dev.max(q_max_abs(&q_add(&sq, &q_unit(0))));
        let (k, l) = (j % 3 + 1, (j + 1) % 3 + 1); // (j, k, l) cyclic: (1,2,3), (2,3,1), (3,1,2)
        dev = dev.max(q_max_abs(&q_sub(&q_mul(&q_unit(j), &q_unit(k)), &q_unit(l))));
        dev = dev.max(q_max_abs(&q_add(&q_mul(&q_unit(k), &q_unit(j)), &q_unit(l))));
    }
    rep.add("A-1", "quaternion units: e_j^2 = -1, e_j e_k = -e_k e_j = e_l for (j,k,l) cyclic", dev, 0.0);

    // A-2: the eight units of the bi-octonions: e_p^2 = -1 and e_p e_q = -e_q e_p for p != q (p, q >= 1);
    // e_4^2 = -1 and e_{4+j} = e_j e_4
    let mut dev: f64 = 0.0;
    for p in 1..8 {
        dev = dev.max(BiO::unit(p).mul(&BiO::unit(p)).add(&BiO::unit(0)).max_abs());
        for q in 1..8 {
            if q != p {
                dev = dev.max(BiO::unit(p).mul(&BiO::unit(q)).add(&BiO::unit(q).mul(&BiO::unit(p))).max_abs());
            }
        }
    }
    for j in 1..4 {
        dev = dev.max(BiO::unit(j).mul(&BiO::unit(4)).sub(&BiO::unit(4 + j)).max_abs());
    }
    rep.add("A-2", "bi-octonion units: e_p^2 = -1, e_p e_q = -e_q e_p (p != q, both >= 1), e_j e_4 = e_{4+j}", dev, 0.0);

    // A-3 .. A-6 on 200 random bi-octonions with integer coefficients (products without rounding)
    let n = 200;
    let (mut d3, mut d4, mut d5, mut d6, mut dna) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..n {
        let x = random_int_bio(&mut rng, 3);
        let y = random_int_bio(&mut rng, 3);
        let z = random_int_bio(&mut rng, 3);
        // A-3: N(x y) = N(x) N(y)
        d3 = d3.max((x.mul(&y).norm() - x.norm() * y.norm()).abs());
        // A-4: x xbar = N(x) e_0
        d4 = d4.max(x.mul(&x.bar()).sub(&BiO::unit(0).scale(x.norm())).max_abs());
        // A-5: alternativity (x x) y = x (x y) and (y x) x = y (x x)
        d5 = d5.max(x.mul(&x).mul(&y).sub(&x.mul(&x.mul(&y))).max_abs());
        d5 = d5.max(y.mul(&x).mul(&x).sub(&y.mul(&x.mul(&x))).max_abs());
        // A-6: the three Moufang identities
        // (z x z) y = z (x (z y)),  y (z x z) = ((y z) x) z,  (z x)(y z) = (z (x y)) z
        let zxz = z.mul(&x).mul(&z);
        d6 = d6.max(zxz.mul(&y).sub(&z.mul(&x.mul(&z.mul(&y)))).max_abs());
        d6 = d6.max(y.mul(&zxz).sub(&y.mul(&z).mul(&x).mul(&z)).max_abs());
        d6 = d6.max(z.mul(&x).mul(&y.mul(&z)).sub(&z.mul(&x.mul(&y)).mul(&z)).max_abs());
        // the associator is not zero in general
        dna = dna.max(x.assoc(&y, &z).max_abs());
    }
    rep.add("A-3", "the complex norm is multiplicative, N(xy) = N(x)N(y), 200 random bi-octonions (exact integers)", d3, 0.0);
    rep.add("A-4", "x xbar = N(x) e_0, 200 random bi-octonions (exact)", d4, 0.0);
    rep.add("A-5", "alternativity (xx)y = x(xy), (yx)x = y(xx), 200 random triples (exact)", d5, 0.0);
    rep.add("A-6", "the three Moufang identities, 200 random triples (exact)", d6, 0.0);

    // A-7: non-associativity: the associators of the units; count the triples with a nonzero associator
    let mut nonzero = 0;
    let mut total = 0;
    for p in 1..8 {
        for q in 1..8 {
            for r in 1..8 {
                total += 1;
                if BiO::unit(p).assoc(&BiO::unit(q), &BiO::unit(r)).max_abs() > 0.0 {
                    nonzero += 1;
                }
            }
        }
    }
    let a124 = BiO::unit(1).assoc(&BiO::unit(2), &BiO::unit(4));
    let a124_is_2e7 = a124.sub(&BiO::unit(7).scale(c(2.0, 0.0))).max_abs() == 0.0;
    rep.add_bool(
        "A-7",
        "the bi-octonion product is not associative: (e1 e2) e4 - e1 (e2 e4) = 2 e7, and of the 343 triples of imaginary units 168 have a nonzero associator; the largest associator of the 200 random triples is nonzero",
        nonzero == 168 && total == 343 && a124_is_2e7 && dna > 0.0,
        &format!("nonzero unit associators {} of {}; (e1e2)e4 - e1(e2e4) = 2 e7: {}; max random associator {}", nonzero, total, a124_is_2e7, dna),
    );

    // A-8: the bi-quaternions: associative; M multiplicative; det M = N; M(b^dag) = M(b)^dag; biq_of inverts M
    let (mut dass, mut dmul, mut ddet, mut ddag, mut dinv) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..n {
        let r = |rng: &mut Rng| -> BiQ { [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())] };
        let (a, b, cq) = (r(&mut rng), r(&mut rng), r(&mut rng));
        dass = dass.max(q_max_abs(&q_sub(&q_mul(&q_mul(&a, &b), &cq), &q_mul(&a, &q_mul(&b, &cq)))));
        dmul = dmul.max(mll(&q_mul(&a, &b)).sub(&mll(&a).mul(&mll(&b))).max_abs());
        ddet = ddet.max((mll(&a).det2() - q_norm(&a)).abs());
        ddag = ddag.max(mll(&q_dag(&a)).sub(&mll(&a).dagger()).max_abs());
        dinv = dinv.max(q_max_abs(&q_sub(&biq_of(&mll(&a)), &a)));
    }
    rep.add("A-8", "bi-quaternions: associative; M(bb') = M(b)M(b'); det M(b) = N(b); M(b^dag) = M(b)^dag; the inverse map recovers b (200 random)", dass.max(dmul).max(ddet).max(ddag).max(dinv), 1e-13);

    // A-9: M31: points are Hermitian, M(x) = sum_p x^p sigma_p, N(x) = (x^0)^2 - |x|^2; the sample point
    let mut dev: f64 = 0.0;
    for _ in 0..n {
        let x = [2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit()];
        let px = point(&x);
        let (back, herm) = coords(&q_dag(&px));
        dev = dev.max(herm);
        for p in 0..4 {
            dev = dev.max((back[p] - x[p]).abs());
        }
        let mut mx = CMat::zeros(2);
        for p in 0..4 {
            mx = mx.add(&sigma(p).scale_r(x[p]));
        }
        dev = dev.max(mll(&px).sub(&mx).max_abs());
        let interval = x[0] * x[0] - x[1] * x[1] - x[2] * x[2] - x[3] * x[3];
        dev = dev.max((q_norm(&px) - c(interval, 0.0)).abs());
    }
    rep.add("A-9", "M31: the points are Hermitian bi-quaternions (x^dag = x), M(x) = sum_p x^p sigma_p, N(x) = Minkowski interval (200 random)", dev, 1e-13);

    // A-10: Lorentz transformations x -> g x g^dag: Hermitian image, N preserved, L^T eta L = eta, det L = 1,
    // L_00 >= 1, L(gh) = L(g)L(h), L(-g) = L(g)
    let (mut dh, mut dn, mut dm, mut dd, mut dhom) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut l00_ok = true;
    for _ in 0..n {
        let mg = random_unit_biq(&mut rng, 1.5);
        let mh = random_unit_biq(&mut rng, 1.5);
        let g = biq_of(&mg);
        let x = [2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit()];
        let gx = q_mul(&q_mul(&g, &point(&x)), &q_dag(&g));
        let (_, herm) = coords(&gx);
        dh = dh.max(herm);
        dn = dn.max((q_norm(&gx) - q_norm(&point(&x))).abs());
        let l = lorentz_matrix(&mg);
        for p in 0..4 {
            for q in 0..4 {
                let s: f64 = (0..4).map(|r| l[r][p] * ETA[r] * l[r][q]).sum();
                let want = if p == q { ETA[p] } else { 0.0 };
                dm = dm.max((s - want).abs());
            }
        }
        dd = dd.max((det4(&l) - 1.0).abs());
        if l[0][0] < 1.0 - 1e-12 {
            l00_ok = false;
        }
        let lgh = lorentz_matrix(&mg.mul(&mh));
        let lh = lorentz_matrix(&mh);
        let lneg = lorentz_matrix(&mg.scale_r(-1.0));
        for p in 0..4 {
            for q in 0..4 {
                let prod: f64 = (0..4).map(|r| l[p][r] * lh[r][q]).sum();
                dhom = dhom.max((lgh[p][q] - prod).abs()).max((lneg[p][q] - l[p][q]).abs());
            }
        }
    }
    rep.add("A-10a", "x -> g x g^dag maps M31 to itself (Hermitian image) and keeps N(x) (200 random unit g)", dh.max(dn), 1e-11);
    rep.add("A-10b", "L(g)^T eta L(g) = eta and det L(g) = 1 (200 random unit g)", dm.max(dd), 1e-10);
    rep.add_bool("A-10c", "L(g)_00 >= 1 (orthochronous) for the 200 random unit g", l00_ok, "all 200");
    rep.add("A-10d", "L(gh) = L(g) L(h) and L(-g) = L(g) (200 random pairs)", dhom, 1e-10);

    // A-11: the Lorentz generators J^j = e_j/2, K^j = i e_j/2: [J,J] = eps J, [J,K] = eps K, [K,K] = -eps J
    let jj = |j: usize| q_scale(&q_unit(j), c(0.5, 0.0));
    let kk = |j: usize| q_scale(&q_unit(j), c(0.0, 0.5));
    let comm = |a: &BiQ, b: &BiQ| q_sub(&q_mul(a, b), &q_mul(b, a));
    let mut dev: f64 = 0.0;
    for j in 1..4 {
        for k in 1..4 {
            let mut want_j = [C64::ZERO; 4];
            let mut want_k = [C64::ZERO; 4];
            for l in 1..4 {
                let e = sfcore::dirac::levi(j - 1, k - 1, l - 1);
                want_j = q_add(&want_j, &q_scale(&jj(l), c(e, 0.0)));
                want_k = q_add(&want_k, &q_scale(&kk(l), c(e, 0.0)));
            }
            dev = dev.max(q_max_abs(&q_sub(&comm(&jj(j), &jj(k)), &want_j)));
            dev = dev.max(q_max_abs(&q_sub(&comm(&jj(j), &kk(k)), &want_k)));
            dev = dev.max(q_max_abs(&q_add(&comm(&kk(j), &kk(k)), &want_j)));
        }
    }
    rep.add("A-11", "the 27 commutators of the generators: [J^j,J^k] = eps_jkl J^l, [J^j,K^k] = eps_jkl K^l, [K^j,K^k] = -eps_jkl J^l", dev, 1e-15);

    // A-12: the spinor charge b e4 under the multiplications of the super-loop: (b e4) g = (b gbar) e4,
    // g (b e4) = (b g) e4, and the isospin rotation (v ((b e4) e4)) e4^{-1} = (v b) e4 (exact integers)
    let mut dev: f64 = 0.0;
    for _ in 0..n {
        let b = random_int_bio(&mut rng, 3).a;
        let g = random_int_bio(&mut rng, 3).a;
        let be4 = BiO { a: [C64::ZERO; 4], b };
        let gq = BiO { a: g, b: [C64::ZERO; 4] };
        let right = be4.mul(&gq);
        let left = gq.mul(&be4);
        dev = dev.max(right.sub(&BiO { a: [C64::ZERO; 4], b: q_mul(&b, &q_bar(&g)) }).max_abs());
        dev = dev.max(left.sub(&BiO { a: [C64::ZERO; 4], b: q_mul(&b, &g) }).max_abs());
        let e4 = BiO::unit(4);
        let e4inv = e4.bar(); // N(e4) = 1, so e4^{-1} = e4bar = -e4
        let iso = gq.mul(&be4.mul(&e4)).mul(&e4inv);
        dev = dev.max(iso.sub(&BiO { a: [C64::ZERO; 4], b: q_mul(&g, &b) }).max_abs());
    }
    rep.add("A-12", "spinor charges: (b e4) g = (b gbar) e4, g (b e4) = (b g) e4, (v ((b e4) e4)) e4^-1 = (v b) e4 (200 random, exact)", dev, 0.0);

    // data: worked examples of M31 for the manuscript (the sample point, a boost and a rotation)
    let mut rows: Vec<Vec<String>> = vec![];
    let sample = [2.0, 1.2, 1.5, 1.0];
    let ps = point(&sample);
    rows.push(vec!["sample point".into(), f(sample[0]), f(sample[1]), f(sample[2]), f(sample[3]), f(q_norm(&ps).re)]);
    let boost = exp_traceless2(&lorentz_generator(&[0.0, 0.0, 0.0], &[0.8, 0.0, 0.0]));
    let l = lorentz_matrix(&boost);
    let img: Vec<f64> = (0..4).map(|p| l[p][0]).collect();
    rows.push(vec!["boost rapidity 0.8 along x1 of (1 0 0 0)".into(), f(img[0]), f(img[1]), f(img[2]), f(img[3]), f(img[0] * img[0] - img[1] * img[1] - img[2] * img[2] - img[3] * img[3])]);
    let rot = exp_traceless2(&lorentz_generator(&[0.0, 0.0, std::f64::consts::PI / 3.0], &[0.0, 0.0, 0.0]));
    let l = lorentz_matrix(&rot);
    let img: Vec<f64> = (0..4).map(|p| l[p][1]).collect();
    rows.push(vec!["rotation pi/3 about x3 of (0 1 0 0)".into(), f(img[0]), f(img[1]), f(img[2]), f(img[3]), f(img[0] * img[0] - img[1] * img[1] - img[2] * img[2] - img[3] * img[3])]);
    write_csv(&data.join("part1_m31_examples.csv"), &["case", "x0", "x1", "x2", "x3", "N"], &rows);
}

/// the determinant of a real 4x4 matrix (Laplace expansion along the first row)
pub fn det4(m: &[[f64; 4]; 4]) -> f64 {
    let mut d = 0.0;
    for j in 0..4 {
        let mut minor = [[0.0; 3]; 3];
        for r in 1..4 {
            let mut cc = 0;
            for k in 0..4 {
                if k != j {
                    minor[r - 1][cc] = m[r][k];
                    cc += 1;
                }
            }
        }
        let d3 = minor[0][0] * (minor[1][1] * minor[2][2] - minor[1][2] * minor[2][1])
            - minor[0][1] * (minor[1][0] * minor[2][2] - minor[1][2] * minor[2][0])
            + minor[0][2] * (minor[1][0] * minor[2][1] - minor[1][1] * minor[2][0]);
        d += if j % 2 == 0 { m[0][j] * d3 } else { -m[0][j] * d3 };
    }
    d
}
