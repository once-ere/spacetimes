//! lg_lord -- the checks of the equations of E. A. Lord, "General relativity from gauge invariance", Proc. Camb. Phil.
//! Soc. 69, 423 (1971) (Part I of lord_gravity.md), and of the association of the group SL(2,C) of the project with his
//! gauge group (Part II).  PROJECT spacetimes, folder Revision_Lord_Gravity, task C23.
//!
//! What is checked (identifiers as in the report data/hck_lg_lord_checks.txt):
//! * L1-*: his Section 1, the algebra of the Pauli and Dirac matrices in the Weyl representation, his Lorentz
//!   transformations (1.8), (1.16), and the changes of representation (1.22), (1.23); including the precise reading of
//!   his statement that the four Hermitian matrices "can be any linearly independent set" (L1-14);
//! * L4-*: his Section 4, the local changes of representation: the transformation of the Dirac matrices (4.2) to
//!   (4.5), the covariance of his derivative (4.13) and the sign of the inhomogeneous term of (4.14), (4.31), the
//!   transformation (4.16), (4.32) of the coefficients a_abc, the connection C of the vectors (4.19), (4.20) and its
//!   properties (4.24), (4.25), the curvature of the spinor connection (4.47), (4.48), and its relation to the
//!   curvature of the vector connection (4.46);
//! * P-*: the dictionary between his objects and the project's (Part II): the Pauli, Dirac and charge-conjugation
//!   matrices, the frames of M31 and their Dirac matrices, the spinor matrix S_LL(g) of a unit bi-quaternion, the Lorentz
//!   matrices, the spin connection of the gauge group GEE of the earlier hypotheses, and the changes of representation
//!   that keep the Weyl form of the Dirac matrices (P-9).
//!
//! The local checks (L4, P-8) use fields of the four coordinates built from smooth pseudo-random functions with exact
//! first and second derivatives (forward-mode automatic differentiation, module lgcore::dual): no finite difference is
//! used.  Each check compares a computed deviation with a tolerance; findings about an equation as printed are checks
//! that pass when the printed form FAILS by the stated amount (the text of the check says so).
//!
//! Build and run (from Revision_Lord_Gravity/rust):  cargo run --release --bin lg_lord
//! Output: data/hck_lg_lord_checks.txt (the report), data/lg_lord_findings.csv (the numbers of the findings).
use lgcore::algebra::{biq_of, lorentz_matrix, mll, q_bar, random_unit_biq};
use lgcore::c64::{c, C64};
use lgcore::cmat::CMat;
use lgcore::dirac;
use lgcore::dual::{dmatvec, DMat, D4};
use lgcore::lord::*;
use lgcore::report::{data_dir, f, write_csv, Report};
use lgcore::rng::Rng;

const NPT: usize = 12; // number of random points of the local checks
const G: f64 = 0.7; // a value of Lord's universal coupling g (the checks hold for every nonzero g)

// ------------------------------------------------------------------------------------------------------------------
// small helpers
// ------------------------------------------------------------------------------------------------------------------

/// the four standard matrices sigma^mu
fn sig_set() -> Vec<CMat> {
    (0..4).map(sig).collect()
}

/// lowered index: X_mu = eta_{mu mu} X^mu (eta is diagonal)
fn low(m: usize) -> f64 {
    eta(m, m)
}

/// a pseudo-random matrix of SL(2,C) (exp of a traceless matrix of size about amp)
fn rand_sl2c(rng: &mut Rng, amp: f64) -> CMat {
    let a = c(amp * rng.unit(), amp * rng.unit());
    let b = c(amp * rng.unit(), amp * rng.unit());
    let cc_ = c(amp * rng.unit(), amp * rng.unit());
    CMat::m2(a, b, cc_, -a).expm()
}

/// a pseudo-random traceless 2x2 field with entries that are smooth functions of the coordinates
fn rand_traceless(rng: &mut Rng, x: &[f64; 4], amp: f64) -> DMat {
    let a = rand_fn(rng, x, amp);
    let b = rand_fn(rng, x, amp);
    let cc_ = rand_fn(rng, x, amp);
    let mut m = DMat::new(2, 2);
    m.set(0, 0, a);
    m.set(0, 1, b);
    m.set(1, 0, cc_);
    m.set(1, 1, -a);
    m
}

/// a field of SL(2,C): exp of a pseudo-random traceless field (DMat::exp_traceless2)
fn rand_sl2c_field(rng: &mut Rng, x: &[f64; 4], amp: f64) -> DMat {
    rand_traceless(rng, x, amp).exp_traceless2()
}

/// Lord's 4x4 matrix (4.1) of a 2x2 matrix S of SL(2,C): diag(S, (S^dag)^{-1}), and its inverse diag(S^{-1}, S^dag)
fn s4(s: &DMat) -> DMat {
    DMat::block_diag(s, &s.dagger().inv2())
}
fn s4inv(s: &DMat) -> DMat {
    DMat::block_diag(&s.inv2(), &s.dagger())
}

/// an element of the Lorentz algebra in Lord's Weyl representation: diag(Y, -Y^dag) with a traceless Y
fn lorentz_alg(y: &DMat) -> DMat {
    DMat::block_diag(y, &y.dagger().scale(D4::re(-1.0)))
}

/// a pseudo-random test spinor field (four components)
fn rand_spinor(rng: &mut Rng, x: &[f64; 4], amp: f64) -> Vec<D4> {
    (0..4).map(|_| rand_fn(rng, x, amp)).collect()
}

/// a real smooth function of the coordinates (real amplitudes), for the real vector connection
fn rand_real_fn(rng: &mut Rng, x: &[f64; 4], amp: f64) -> D4 {
    let mut s = D4::ZERO;
    for _ in 0..3 {
        let a = amp * rng.unit();
        let mut arg = D4::re(std::f64::consts::PI * rng.unit());
        for p in 0..4 {
            arg = arg + D4::coord(x, p) * D4::re(rng.unit());
        }
        s = s + arg.sin().scale(c(a, 0.0));
    }
    s
}

/// the largest modulus of the entries of a vector of complex numbers
fn vmax(v: &[C64]) -> f64 {
    v.iter().fold(0.0, |s, z| s.max(z.abs()))
}

/// a random point of the coordinates
fn rand_point(rng: &mut Rng) -> [f64; 4] {
    [rng.unit(), rng.unit(), rng.unit(), rng.unit()]
}

// ------------------------------------------------------------------------------------------------------------------
// Section 1 of Lord: the algebra of the Weyl representation
// ------------------------------------------------------------------------------------------------------------------
fn section1(rep: &mut Report, rng: &mut Rng, find: &mut Vec<Vec<String>>) {
    let s = sig_set();
    // (1.2): sigma-bar = -eps sigma^T eps is (1, -sigma_j)
    let mut dev = 0.0f64;
    for m in 0..4 {
        let expect = if m == 0 { sig(0) } else { sig(m).scale_r(-1.0) };
        dev = dev.max(bar(&s[m]).sub(&expect).max_abs());
    }
    rep.add("L1-1", "(1.2): sigma-bar^mu = -epsilon (sigma^mu)^T epsilon is (1, -sigma_1, -sigma_2, -sigma_3) for the standard set (1.1)", dev, 1e-15);
    // (1.3)
    let mut dev = 0.0f64;
    for m in 0..4 {
        for n in 0..4 {
            let lhs = s[m].mul(&bar(&s[n])).add(&s[n].mul(&bar(&s[m]))).scale_r(0.5);
            dev = dev.max(lhs.sub(&CMat::identity(2).scale_r(eta(m, n))).max_abs());
        }
    }
    rep.add("L1-2", "(1.3): 1/2 (sigma^mu sigma-bar^nu + sigma^nu sigma-bar^mu) = eta^{mu nu} 1 (16 pairs)", dev, 1e-15);
    // (1.5)
    let mut dev = 0.0f64;
    for m in 0..4 {
        for n in 0..4 {
            for r in 0..4 {
                for t in 0..4 {
                    let lhs = sig_mn(&s, m, n).comm(&sig_mn(&s, r, t));
                    let rhs = sig_mn(&s, m, t)
                        .scale_r(eta(n, r))
                        .sub(&sig_mn(&s, n, t).scale_r(eta(m, r)))
                        .add(&sig_mn(&s, n, r).scale_r(eta(m, t)))
                        .sub(&sig_mn(&s, m, r).scale_r(eta(n, t)))
                        .scale_r(2.0);
                    dev = dev.max(lhs.sub(&rhs).max_abs());
                }
            }
        }
    }
    rep.add("L1-3", "(1.5): [sigma^{mu nu}, sigma^{rho sigma}] = 2 (eta^{nu rho} sigma^{mu sigma} - eta^{mu rho} sigma^{nu sigma} + eta^{mu sigma} sigma^{nu rho} - eta^{nu sigma} sigma^{mu rho}) (256 index combinations)", dev, 1e-14);
    // (1.6), (1.7): self-duality sigma_{mu nu} = sigma_{mu-bar nu-bar} = 1/2 i eps_{mu nu rho sigma} sigma^{rho sigma}
    let mut dev = 0.0f64;
    for m in 0..4 {
        for n in 0..4 {
            let lower = sig_mn(&s, m, n).scale_r(low(m) * low(n));
            let mut dual = CMat::zeros(2);
            for r in 0..4 {
                for t in 0..4 {
                    dual = dual.add(&sig_mn(&s, r, t).scale(c(0.0, 0.5 * levi4(m, n, r, t))));
                }
            }
            dev = dev.max(lower.sub(&dual).max_abs());
        }
    }
    rep.add("L1-4", "(1.6), (1.7): sigma_{mu nu} is self-dual, sigma_{mu nu} = 1/2 i eps_{mu nu rho sigma} sigma^{rho sigma} with eps_0123 = +1", dev, 1e-15);
    // (1.8), infinitesimal: Lambda_nu^mu = delta + eta^{mu rho} lambda_{rho nu} with L = 1 + 1/4 lambda_{ab} sigma^{ab}
    let mut dev = 0.0f64;
    for _ in 0..20 {
        let mut lam = [[0.0; 4]; 4]; // lambda_{ab}, lower indices, antisymmetric
        for a in 0..4 {
            for b in a + 1..4 {
                let v = rng.unit();
                lam[a][b] = v;
                lam[b][a] = -v;
            }
        }
        let mut xx = CMat::zeros(2);
        for a in 0..4 {
            for b in 0..4 {
                xx = xx.add(&sig_mn(&s, a, b).scale_r(0.25 * lam[a][b]));
            }
        }
        for m in 0..4 {
            // eta^{mu rho} lambda_{rho nu} sigma^nu + X sigma^mu + sigma^mu X^dag = 0
            let mut t = xx.mul(&s[m]).add(&s[m].mul(&xx.dagger()));
            for nn in 0..4 {
                t = t.add(&s[nn].scale_r(eta(m, m) * lam[m][nn]));
            }
            dev = dev.max(t.max_abs());
        }
    }
    rep.add("L1-5", "(1.8), infinitesimal: with Lambda_mu^nu = delta_mu^nu + eta^{nu rho} lambda_{rho mu}, L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu} satisfies (1.8) to first order (20 random lambda): eta^{mu rho} lambda_{rho nu} sigma^nu + X sigma^mu + sigma^mu X^dag = 0", dev, 1e-15);
    // (1.8), finite: the Lambda of a random L is a proper orthochronous Lorentz matrix; (1.16)
    let (mut dl, mut d16, mut ok) = (0.0f64, 0.0f64, true);
    for _ in 0..50 {
        let l = rand_sl2c(rng, 0.8);
        let lam = lorentz_of(&l);
        for a in 0..4 {
            for b in 0..4 {
                let mut x = 0.0;
                for r in 0..4 {
                    x += lam[a][r] * lam[b][r] * eta(r, r);
                }
                dl = dl.max((x - eta(a, b)).abs());
            }
        }
        let det = det4(&lam);
        ok &= (det - 1.0).abs() < 1e-10 && lam[0][0] >= 1.0 - 1e-12;
        // (1.16): Lambda_nu^mu L4 gamma^nu L4^{-1} = gamma^mu with L4 = diag(L, (L^dag)^{-1})
        let l4 = CMat::block_diag(&l, &l.dagger().inv2());
        let l4i = l4_inv(&l);
        for mu in 0..4 {
            let mut lhs = CMat::zeros(4);
            for nu in 0..4 {
                lhs = lhs.add(&l4.mul(&gam(&s[nu])).mul(&l4i).scale_r(lam[nu][mu]));
            }
            d16 = d16.max(lhs.sub(&gam(&s[mu])).max_abs());
        }
    }
    rep.add("L1-6a", "(1.8): for 50 random L of SL(2,C), the Lambda defined by Lambda_nu^mu L sigma^nu L^dag = sigma^mu satisfies Lambda eta Lambda^T = eta", dl, 1e-12);
    rep.add_bool("L1-6b", "(1.8): ... and has det Lambda = 1 and Lambda_0^0 >= 1 (proper orthochronous): L -> Lambda is the covering map of the proper orthochronous Lorentz group", ok, "50 random L");
    rep.add("L1-6c", "(1.16): Lambda_nu^mu L4 gamma^nu L4^{-1} = gamma^mu with L4 = diag(L, (L^dag)^{-1}), the same Lambda (50 random L)", d16, 1e-12);
    // (1.10) to (1.14)
    let gm: Vec<CMat> = s.iter().map(gam).collect();
    let g5 = gam5();
    let (mut d10, mut d11) = (0.0f64, 0.0f64);
    for m in 0..4 {
        for n in 0..4 {
            d10 = d10.max(gm[m].acomm(&gm[n]).scale_r(0.5).sub(&CMat::identity(4).scale_r(eta(m, n))).max_abs());
        }
        d11 = d11.max(gm[m].acomm(&g5).max_abs());
    }
    rep.add("L1-7a", "(1.10): 1/2 (gamma^mu gamma^nu + gamma^nu gamma^mu) = eta^{mu nu} for the Weyl representation (1.9)", d10, 1e-15);
    rep.add("L1-7b", "(1.11): gamma^mu gamma^5 + gamma^5 gamma^mu = 0", d11, 1e-15);
    rep.add("L1-7c", "(1.12): (gamma^5)^2 = 1", g5.mul(&g5).sub(&CMat::identity(4)).max_abs(), 1e-15);
    let prod = gm[0].mul(&gm[1]).mul(&gm[2]).mul(&gm[3]).scale(c(0.0, -1.0));
    let mut eps_sum = CMat::zeros(4);
    for a in 0..4 {
        for b in 0..4 {
            for cc_ in 0..4 {
                for d in 0..4 {
                    let e = levi4(a, b, cc_, d);
                    if e != 0.0 {
                        eps_sum = eps_sum.add(&gm[a].mul(&gm[b]).mul(&gm[cc_]).mul(&gm[d]).scale_r(e));
                    }
                }
            }
        }
    }
    let form2 = eps_sum.scale(c(0.0, -1.0 / 24.0));
    rep.add("L1-7d", "(1.13): gamma^5 = -i gamma^0 gamma^1 gamma^2 gamma^3 = -(1/24) i eps_{mu nu rho sigma} gamma^mu gamma^nu gamma^rho gamma^sigma (both forms)", g5.sub(&prod).max_abs().max(g5.sub(&form2).max_abs()), 1e-14);
    let cm = cc();
    let cinv = cm.dagger(); // C is real orthogonal: C^{-1} = C^T = C^dag
    let mut d14 = cm.mul(&cinv).sub(&CMat::identity(4)).max_abs();
    for m in 0..4 {
        d14 = d14.max(cm.mul(&gm[m]).mul(&cinv).add(&gm[m].transpose()).max_abs());
    }
    rep.add("L1-8", "(1.14): C gamma^mu C^{-1} = -(gamma^mu)^T with C = diag(epsilon, -epsilon)", d14, 1e-15);
    // (1.15) block form of gamma^{mu nu}
    let mut d15 = 0.0f64;
    for m in 0..4 {
        for n in 0..4 {
            let blk = CMat::block_diag(&sig_mn(&s, m, n), &sig_mn(&s, m, n).dagger().scale_r(-1.0));
            d15 = d15.max(gam_mn(&gm, m, n).sub(&blk).max_abs());
        }
    }
    rep.add("L1-9", "(1.15): gamma^{mu nu} = diag(sigma^{mu nu}, -(sigma^{mu nu})^dag) in the Weyl representation", d15, 1e-15);
    // (1.17)
    let mut d17 = 0.0f64;
    for m in 0..4 {
        for n in 0..4 {
            let lower = gam_mn(&gm, m, n).scale_r(low(m) * low(n));
            let mut dual = CMat::zeros(4);
            for r in 0..4 {
                for t in 0..4 {
                    dual = dual.add(&gam_mn(&gm, r, t).scale(c(0.0, 0.5 * levi4(m, n, r, t))));
                }
            }
            d17 = d17.max(g5.mul(&lower).sub(&dual).max_abs()).max(lower.mul(&g5).sub(&dual).max_abs());
        }
    }
    rep.add("L1-10", "(1.17): gamma_5 gamma_{mu nu} = gamma_{mu nu} gamma_5 = gamma_{mu-bar nu-bar} (the dual (1.6))", d17, 1e-15);
    // (1.22), (1.23)
    let (mut d22, mut deps) = (0.0f64, 0.0f64);
    for _ in 0..50 {
        let sm = rand_sl2c(rng, 0.8);
        deps = deps.max(eps().mul(&sm).sub(&sm.transpose().inv2().mul(&eps())).max_abs());
        for m in 0..4 {
            let sp = sm.mul(&s[m]).mul(&sm.dagger());
            let rhs = sm.dagger().inv2().mul(&bar(&s[m])).mul(&sm.inv2());
            d22 = d22.max(bar(&sp).sub(&rhs).max_abs());
        }
    }
    rep.add("L1-11a", "property of epsilon used for (1.23): epsilon S = (S^T)^{-1} epsilon for every unimodular S (50 random S)", deps, 1e-13);
    rep.add("L1-11b", "(1.22), (1.23): for sigma^mu -> S sigma^mu S^dag, the sigma-bar defined by (1.2) becomes (S^dag)^{-1} sigma-bar^mu S^{-1} (50 random S)", d22, 1e-12);
    // "All the properties ... are unaltered": the S-transformed sets satisfy (1.3), (1.5), (1.7), (1.10)
    let mut dinv = 0.0f64;
    for _ in 0..20 {
        let sm = rand_sl2c(rng, 0.8);
        let st: Vec<CMat> = s.iter().map(|x| sm.mul(x).mul(&sm.dagger())).collect();
        let gt: Vec<CMat> = st.iter().map(gam).collect();
        for m in 0..4 {
            for n in 0..4 {
                let l3 = st[m].mul(&bar(&st[n])).add(&st[n].mul(&bar(&st[m]))).scale_r(0.5);
                dinv = dinv.max(l3.sub(&CMat::identity(2).scale_r(eta(m, n))).max_abs());
                dinv = dinv.max(gt[m].acomm(&gt[n]).scale_r(0.5).sub(&CMat::identity(4).scale_r(eta(m, n))).max_abs());
                let lower = sig_mn(&st, m, n).scale_r(low(m) * low(n));
                let mut dual = CMat::zeros(2);
                for r in 0..4 {
                    for t in 0..4 {
                        dual = dual.add(&sig_mn(&st, r, t).scale(c(0.0, 0.5 * levi4(m, n, r, t))));
                    }
                }
                dinv = dinv.max(lower.sub(&dual).max_abs());
            }
        }
    }
    rep.add("L1-12", "Section 1, 'all the properties ... are unaltered': every set S sigma^mu S^dag (S unimodular) satisfies (1.3), (1.7) and (1.10) (20 random S)", dinv, 1e-12);
    // L1-13, L1-14: a generic linearly independent Hermitian set satisfies (1.3) only with its own metric N(sigma^a, sigma^b)
    let (mut dgen, mut dmin_eta) = (0.0f64, f64::MAX);
    for _ in 0..20 {
        let set: Vec<CMat> = (0..4)
            .map(|_| {
                let a = rng.unit();
                let d = rng.unit();
                let b = c(rng.unit(), rng.unit());
                CMat::m2(c(a, 0.0), b, b.conj(), c(d, 0.0))
            })
            .collect();
        let mut dev_eta = 0.0f64;
        for m in 0..4 {
            for n in 0..4 {
                let l3 = set[m].mul(&bar(&set[n])).add(&set[n].mul(&bar(&set[m]))).scale_r(0.5);
                let nd = ndet(&set[m], &set[n]);
                dgen = dgen.max(l3.sub(&CMat::identity(2).scale(nd)).max_abs());
                dev_eta = dev_eta.max(l3.sub(&CMat::identity(2).scale_r(eta(m, n))).max_abs());
            }
        }
        dmin_eta = dmin_eta.min(dev_eta);
    }
    rep.add("L1-13", "for ANY four Hermitian matrices, 1/2 (sigma^a sigma-bar^b + sigma^b sigma-bar^a) = N(sigma^a, sigma^b) 1, with N the polarized determinant (the Minkowski product of the matrices read as points of space-time) (20 random sets)", dgen, 1e-14);
    rep.add_bool("L1-14", "hence (1.3) and (1.10) hold for a set exactly when N(sigma^a, sigma^b) = eta^{ab}: a generic linearly independent Hermitian set violates them, and Lord's 'any linearly independent set' must be read as 'any set S sigma^mu S^dag'", dmin_eta > 0.1, &format!("smallest deviation from eta over 20 random sets: {:.3}", dmin_eta));
    find.push(vec!["L1-14".into(), "(1.3) for a generic Hermitian set: smallest deviation from eta over 20 sets".into(), f(dmin_eta)]);
}

/// determinant of a real 4x4 matrix (Laplace expansion along the first row)
fn det4(m: &[[f64; 4]; 4]) -> f64 {
    let mut d = 0.0;
    for j in 0..4 {
        let mut sub = [[0.0; 3]; 3];
        for i in 1..4 {
            let mut k = 0;
            for jj in 0..4 {
                if jj != j {
                    sub[i - 1][k] = m[i][jj];
                    k += 1;
                }
            }
        }
        let d3 = sub[0][0] * (sub[1][1] * sub[2][2] - sub[1][2] * sub[2][1]) - sub[0][1] * (sub[1][0] * sub[2][2] - sub[1][2] * sub[2][0])
            + sub[0][2] * (sub[1][0] * sub[2][1] - sub[1][1] * sub[2][0]);
        d += if j % 2 == 0 { 1.0 } else { -1.0 } * m[0][j] * d3;
    }
    d
}

/// the inverse of diag(L, (L^dag)^{-1}): diag(L^{-1}, L^dag)
fn l4_inv(l: &CMat) -> CMat {
    CMat::block_diag(&l.inv2(), &l.dagger())
}

// ------------------------------------------------------------------------------------------------------------------
// Section 4 of Lord: the local changes of representation
// ------------------------------------------------------------------------------------------------------------------
fn section4(rep: &mut Report, rng: &mut Rng, find: &mut Vec<Vec<String>>) {
    let s0 = sig_set();
    let g0: Vec<CMat> = s0.iter().map(gam).collect();
    let (mut d41, mut d42, mut d43_rel, mut d44, mut d44_printed_rel) = (0.0f64, 0.0f64, f64::MAX, 0.0f64, f64::MAX);
    let (mut d45a, mut d45b, mut d45_half, mut d45_full_rel, mut d24, mut d25) = (0.0f64, 0.0f64, 0.0f64, f64::MAX, 0.0f64, 0.0f64);
    for _ in 0..NPT {
        let x = rand_point(rng);
        // a field S(x) of SL(2,C) and the Pauli-type matrices sigma^a(x) = S sigma^a S^dag (4.2), their gamma^a(x) (4.5)
        let sf = rand_sl2c_field(rng, &x, 0.6);
        let sig_x: Vec<DMat> = s0.iter().map(|m| sf.mul(&DMat::from_cmat(m)).mul(&sf.dagger())).collect();
        let gam_x: Vec<DMat> = sig_x.iter().map(d_gam).collect();
        let (s4f, s4fi) = (s4(&sf), s4inv(&sf));
        for a in 0..4 {
            let rhs = s4f.mul(&DMat::from_cmat(&g0[a])).mul(&s4fi);
            d41 = d41.max(d_max_abs1(&gam_x[a].sub(&rhs)));
        }
        // a gauge transformation T(x) (Lord's S(x) of (4.30)), a spinor connection Gamma_a in the Lorentz algebra, a spinor
        let tf = rand_sl2c_field(rng, &x, 0.6);
        let (t4, t4i) = (s4(&tf), s4inv(&tf));
        let gamma_conn: Vec<DMat> = (0..4).map(|_| lorentz_alg(&rand_traceless(rng, &x, 0.5))).collect();
        let psi = rand_spinor(rng, &x, 1.0);
        let tpsi = dmatvec(&t4, &psi);
        for a in 0..4 {
            // D_a psi = d_a psi - Gamma_a psi (4.13); transformed: psi' = T psi, Gamma'_a = T Gamma T^{-1} + (d_a T) T^{-1}
            let hom = t4.value().mul(&gamma_conn[a].value()).mul(&t4i.value());
            let gp_ok = hom.add(&t4.deriv(a).mul(&t4i.value()));
            let gp_printed = hom.add(&t4.value().mul(&t4i.deriv(a))); // Lord's "S d_a S^{-1}" read as a product
            let dpsi: Vec<C64> = psi.iter().map(|z| z.d[a]).collect();
            let psiv: Vec<C64> = psi.iter().map(|z| z.v).collect();
            let gpsi = gamma_conn[a].value().matvec(&psiv);
            let dpsi_cov: Vec<C64> = dpsi.iter().zip(&gpsi).map(|(u, w)| *u - *w).collect();
            let rhs = t4.value().matvec(&dpsi_cov);
            let tpv: Vec<C64> = tpsi.iter().map(|z| z.v).collect();
            let tpd: Vec<C64> = tpsi.iter().map(|z| z.d[a]).collect();
            let l_ok: Vec<C64> = tpd.iter().zip(gp_ok.matvec(&tpv)).map(|(u, w)| *u - w).collect();
            let l_pr: Vec<C64> = tpd.iter().zip(gp_printed.matvec(&tpv)).map(|(u, w)| *u - w).collect();
            let e_ok: Vec<C64> = l_ok.iter().zip(&rhs).map(|(u, w)| *u - *w).collect();
            let e_pr: Vec<C64> = l_pr.iter().zip(&rhs).map(|(u, w)| *u - *w).collect();
            d42 = d42.max(vmax(&e_ok) / vmax(&rhs).max(1e-300));
            d43_rel = d43_rel.min(vmax(&e_pr) / vmax(&rhs).max(1e-300));
        }
        // (4.15), (4.16), (4.32): Gamma_a = 1/2 g a_abc gamma^{bc}(x); the coefficients from a_abc = -(1/(4g)) tr(gamma_bc Gamma_a)
        let gam_up: Vec<CMat> = gam_x.iter().map(|m| m.value()).collect();
        let gam_up_t: Vec<CMat> = gam_up.iter().map(|m| t4.value().mul(m).mul(&t4i.value())).collect();
        for a in 0..4 {
            let gp = t4.value().mul(&gamma_conn[a].value()).mul(&t4i.value()).add(&t4.deriv(a).mul(&t4i.value()));
            let gp_pr = t4.value().mul(&gamma_conn[a].value()).mul(&t4i.value()).add(&t4.value().mul(&t4i.deriv(a)));
            let xi = t4.deriv(a).mul(&t4i.value()); // (d_a T) T^{-1}
            let mut recon = CMat::zeros(4);
            for b in 0..4 {
                for cc_ in 0..4 {
                    if b == cc_ {
                        continue;
                    }
                    // gamma_bc with lower indices, of the old and of the transformed representation
                    let gbc = gam_mn(&gam_up, b, cc_).scale_r(low(b) * low(cc_));
                    let gbc_t = gam_mn(&gam_up_t, b, cc_).scale_r(low(b) * low(cc_));
                    let a_old = gbc.mul(&gamma_conn[a].value()).trace().scale(-1.0 / (4.0 * G));
                    let a_new = gbc_t.mul(&gp).trace().scale(-1.0 / (4.0 * G));
                    let a_new_pr = gbc_t.mul(&gp_pr).trace().scale(-1.0 / (4.0 * G));
                    // the corrected (4.32): delta a_abc = -(1/g) tr_N[gamma'_bc (d_a T) T^{-1}], tr_N = 1/4 tr (normalized trace)
                    let corr = gbc_t.mul(&xi).trace().scale(-1.0 / (4.0 * G));
                    d44 = d44.max((a_new - a_old - corr).abs());
                    // the printed form, read with tr_N: the inhomogeneous term with the opposite sign
                    let printed = gbc_t.mul(&t4.value().mul(&t4i.deriv(a))).trace().scale(-1.0 / (4.0 * G));
                    let _ = a_new_pr;
                    if corr.abs() > 1e-6 {
                        d44_printed_rel = d44_printed_rel.min((a_new - a_old - printed).abs() / corr.abs());
                    }
                    // reconstruction of Gamma_a from its coefficients: 1/2 g a_abc gamma^{bc}
                    recon = recon.add(&gam_mn(&gam_up, b, cc_).scale(a_old.scale(0.5 * G)));
                }
            }
            d44 = d44.max(recon.sub(&gamma_conn[a].value()).max_abs());
        }
        // (4.19), (4.20): with Gamma_a = 1/2 g a_abc gamma^{bc}(x) for random constant a_abc (antisymmetric in b, c),
        // solve d_a gamma_b - [Gamma_a, gamma_b] = C_ba^e gamma_e and compare with the closed forms
        let mut acoef = [[[0.0f64; 4]; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                for cc_ in b + 1..4 {
                    let v = 0.5 * rng.unit();
                    acoef[a][b][cc_] = v;
                    acoef[a][cc_][b] = -v;
                }
            }
        }
        let gconn: Vec<CMat> = (0..4)
            .map(|a| {
                let mut m = CMat::zeros(4);
                for b in 0..4 {
                    for cc_ in 0..4 {
                        if b != cc_ {
                            m = m.add(&gam_mn(&gam_up, b, cc_).scale_r(0.5 * G * acoef[a][b][cc_]));
                        }
                    }
                }
                m
            })
            .collect();
        let mut cfull = [[[0.0f64; 4]; 4]; 4]; // C_ba^e stored as cfull[b][a][e]
        // the printed (4.20) with the full trace is compared with C as a whole at this point: the largest difference over
        // all coefficients relative to the largest coefficient (a single coefficient can agree by accident)
        let (mut pf_diff, mut c_max) = (0.0f64, 0.0f64);
        for a in 0..4 {
            for b in 0..4 {
                let gb = gam_x[b].deriv(a).scale_r(low(b)); // d_a gamma_b (lower b)
                let gbv = gam_up[b].scale_r(low(b));
                let m = gb.sub(&gconn[a].comm(&gbv));
                // expansion in gamma_e: C_ba^e = 1/4 tr(M gamma^e) since tr(gamma_e gamma^f) = 4 delta
                let mut resid = m.clone();
                for e in 0..4 {
                    let ce = m.mul(&gam_up[e]).trace().scale(0.25);
                    cfull[b][a][e] = ce.re;
                    d45a = d45a.max(ce.im.abs());
                    resid = resid.sub(&gam_up[e].scale_r(low(e)).scale(ce));
                    // the closed form: C_ba^e = 2 g a_ab^e + tr_N[(d_a gamma_b) gamma^e] (tr_N = 1/4 tr)
                    let a_up = acoef[a][b][e] * low(e);
                    let trn = gb.mul(&gam_up[e]).trace().scale(0.25);
                    let closed = c(2.0 * G * a_up, 0.0) + trn;
                    d45b = d45b.max((ce - closed).abs());
                    // Lord's (4.20) as printed, with tr the normalized trace: g a_ab^e + 1/2 tr_N[...]
                    let printed_n = c(G * a_up, 0.0) + trn.scale(0.5);
                    d45_half = d45_half.max((printed_n - ce.scale(0.5)).abs());
                    // Lord's (4.20) as printed, with tr the full trace: g a_ab^e + 1/2 tr[...]
                    let printed_f = c(G * a_up, 0.0) + trn.scale(2.0);
                    pf_diff = pf_diff.max((printed_f - ce).abs());
                    c_max = c_max.max(ce.abs());
                }
                d45a = d45a.max(resid.max_abs());
            }
        }
        d45_full_rel = d45_full_rel.min(pf_diff / c_max.max(1e-300));
        // (4.24): C_bae = -C_eab (lower e: C_bae = C_ba^f eta_fe)
        for a in 0..4 {
            for b in 0..4 {
                for e in 0..4 {
                    d24 = d24.max((cfull[b][a][e] * low(e) + cfull[e][a][b] * low(b)).abs());
                }
            }
        }
        // (4.25): D_e eps_abcd = C_ae^f eps_fbcd + C_be^f eps_afcd + C_ce^f eps_abfd + C_de^f eps_abcf = 0
        for e in 0..4 {
            for a in 0..4 {
                for b in 0..4 {
                    for cc_ in 0..4 {
                        for d in 0..4 {
                            let mut sum = 0.0;
                            for ff in 0..4 {
                                sum += cfull[a][e][ff] * levi4(ff, b, cc_, d)
                                    + cfull[b][e][ff] * levi4(a, ff, cc_, d)
                                    + cfull[cc_][e][ff] * levi4(a, b, ff, d)
                                    + cfull[d][e][ff] * levi4(a, b, cc_, ff);
                            }
                            d25 = d25.max(sum.abs());
                        }
                    }
                }
            }
        }
    }
    rep.add("L4-1", "(4.2) to (4.5): for a field S(x) of SL(2,C), the Dirac matrices gamma^a(x) built by (4.3), (4.5) from sigma^a(x) = S sigma^a S^dag equal S4 gamma^a S4^{-1}, S4 = diag(S, (S^dag)^{-1}) (4.1): values and first derivatives at 12 random points", d41, 1e-12);
    rep.add("L4-2", "(4.13), (4.31) corrected: D_a = d_a - Gamma_a is covariant, D'_a (T psi) = T D_a psi, when Gamma'_a = T Gamma_a T^{-1} + (d_a T) T^{-1} (relative deviation, 12 points, 4 directions)", d42, 1e-13);
    rep.add_bool("L4-3", "(4.14), (4.31) as printed: with the inhomogeneous term T d_a T^{-1} read as the product T (d_a T^{-1}) = -(d_a T) T^{-1}, D_a = d_a - Gamma_a is NOT covariant; the printed sign belongs to the derivative d_a + Gamma_a", d43_rel > 0.1, &format!("smallest relative deviation of the printed law: {:.3}", d43_rel));
    find.push(vec!["L4-3".into(), "(4.31) as printed: smallest relative violation of covariance".into(), f(d43_rel)]);
    rep.add("L4-4", "(4.15), (4.16), (4.32) corrected: Gamma_a = 1/2 g a_abc gamma^{bc} with a_abc = -(1/(4g)) tr(gamma_bc Gamma_a) (reconstruction), and under the gauge transformation delta a_abc = -(1/g) tr_N[gamma'_bc (d_a T) T^{-1}], with tr_N the trace normalized to tr_N 1 = 1 and gamma' the transformed matrices", d44, 1e-11);
    rep.add_bool("L4-5", "(4.16), (4.32) as printed, with tr = tr_N: the inhomogeneous term -(1/g) tr[gamma_bc T d_a T^{-1}] has the opposite sign of the correct one (it is the law of the derivative d_a + Gamma_a)", d44_printed_rel > 1.9, &format!("smallest |printed - true| / |true| over all coefficients: {:.4} (2 for an opposite sign)", d44_printed_rel));
    find.push(vec!["L4-5".into(), "(4.32) as printed with tr_N: smallest |printed - true|/|true|".into(), f(d44_printed_rel)]);
    rep.add("L4-6a", "(4.19): d_a gamma_b - [Gamma_a, gamma_b] is a linear combination of the gamma_e with real coefficients C_ba^e, as Lord states (residual and imaginary parts)", d45a, 1e-12);
    rep.add("L4-6b", "(4.19) solved: C_ba^e = 2 g a_ab^e + tr_N[(d_a gamma_b) gamma^e] (tr_N = 1/4 tr), for Gamma_a = 1/2 g a_abc gamma^{bc} (4.15)", d45b, 1e-12);
    rep.add("L4-6c", "(4.20) as printed, read with tr = tr_N, g a_ab^e + 1/2 tr_N[(d_a gamma_b) gamma^e], is exactly HALF of the C_ba^e that (4.19) requires", d45_half, 1e-12);
    rep.add_bool("L4-6d", "(4.20) as printed, read with tr the full trace, is not C_ba^e either; since its term g a_ab^e carries half the coefficient 2 g that (4.19) requires, no normalization of tr makes (4.20) equal to C, and only tr_N makes it proportional (L4-6c)", d45_full_rel > 0.05, &format!("smallest, over the 12 points, of the largest deviation of a coefficient relative to the largest coefficient: {:.4}", d45_full_rel));
    find.push(vec!["L4-6d".into(), "(4.20) as printed with the full trace: smallest relative deviation from (4.19)".into(), f(d45_full_rel)]);
    rep.add("L4-7", "(4.24): C_bae = -C_eab, for the C that solves (4.19)", d24, 1e-12);
    rep.add("L4-8", "(4.25): D_e eps_abcd = C_ae^f eps_fbcd + C_be^f eps_afcd + C_ce^f eps_abfd + C_de^f eps_abcf = 0", d25, 1e-11);

    // (4.47), (4.48): the curvature of the spinor connection, and its relation to the curvature (4.46) of the vectors
    let (mut d48, mut d49a, mut d49b, mut d49_lord_half) = (0.0f64, 0.0f64, 0.0f64, f64::MAX);
    let mut lin_ratio = Vec::new();
    for ip in 0..NPT {
        let x = rand_point(rng);
        // a spinor connection Gamma_mu(x) in the Lorentz algebra and a spinor field, with second derivatives
        let gconn: Vec<DMat> = (0..4).map(|_| lorentz_alg(&rand_traceless(rng, &x, 0.5))).collect();
        let psi = rand_spinor(rng, &x, 1.0);
        let psim = col(&psi);
        for mu in 0..4 {
            for nu in 0..4 {
                if mu == nu {
                    continue;
                }
                // D_nu psi = d_nu psi - Gamma_nu psi, as a field with first derivatives
                let dn = psim.partial(nu).sub(&gconn[nu].mul(&psim));
                let dm = psim.partial(mu).sub(&gconn[mu].mul(&psim));
                // D_mu (D_nu psi) at the point
                let dmdn = dn.deriv(mu).sub(&gconn[mu].value().mul(&dn.value()));
                let dndm = dm.deriv(nu).sub(&gconn[nu].value().mul(&dm.value()));
                let comm = dmdn.sub(&dndm);
                let fmn = gconn[mu].deriv(nu).sub(&gconn[nu].deriv(mu)).add(&gconn[mu].value().comm(&gconn[nu].value()));
                let formula = fmn.mul(&psim.value());
                d48 = d48.max(comm.sub(&formula).max_abs());
            }
        }
        // a real vector connection W_mu(x) in so(1,3): (W_mu)_b^e = omega_{mu, b f} eta^{fe}, omega antisymmetric in b, f;
        // the spinor connection Gamma_mu = 1/4 omega_{mu, ab} gamma^{ab} with the constant gamma^a (Lord's special gauge (5.2))
        for (amp_i, amp) in [0.4, 0.04].iter().enumerate() {
            let mut omega: Vec<Vec<Vec<D4>>> = vec![vec![vec![D4::ZERO; 4]; 4]; 4];
            for om in omega.iter_mut() {
                for b in 0..4 {
                    for ff in b + 1..4 {
                        let v = rand_real_fn(rng, &x, *amp);
                        om[b][ff] = v;
                        om[ff][b] = -v;
                    }
                }
            }
            let wmat: Vec<DMat> = omega
                .iter()
                .map(|om| {
                    let mut m = DMat::new(4, 4);
                    for b in 0..4 {
                        for e in 0..4 {
                            m.set(b, e, om[b][e] * D4::re(low(e)));
                        }
                    }
                    m
                })
                .collect();
            let gsp: Vec<DMat> = omega
                .iter()
                .map(|om| {
                    let mut m = DMat::new(4, 4);
                    for a in 0..4 {
                        for b in 0..4 {
                            if a != b {
                                m = m.add(&DMat::from_cmat(&gam_mn(&g0, a, b)).scale(om[a][b] * D4::re(0.25)));
                            }
                        }
                    }
                    m
                })
                .collect();
            for mu in 0..4 {
                // [Gamma_mu, gamma_b] = -(W_mu)_b^e gamma_e: Gamma_mu is the spinor image of W_mu (D_mu gamma_b = 0)
                for b in 0..4 {
                    let gb = g0[b].scale_r(low(b));
                    let lhs = gsp[mu].value().comm(&gb);
                    let mut rhs = CMat::zeros(4);
                    for e in 0..4 {
                        rhs = rhs.sub(&g0[e].scale_r(low(e)).scale(wmat[mu].get(b, e).v));
                    }
                    d49a = d49a.max(lhs.sub(&rhs).max_abs());
                }
            }
            let mut worst_half = 0.0f64;
            let mut worst_lin = 0.0f64;
            for mu in 0..4 {
                for nu in 0..4 {
                    if mu == nu {
                        continue;
                    }
                    // vector curvature (4.46) as a matrix: R_{mu nu} = d_nu W_mu - d_mu W_nu + [W_mu, W_nu] (entries (R)_rho^sigma)
                    let rv = wmat[mu].deriv(nu).sub(&wmat[nu].deriv(mu)).add(&wmat[mu].value().comm(&wmat[nu].value()));
                    // spinor curvature (4.48): F_{mu nu} = d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu]
                    let fs = gsp[mu].deriv(nu).sub(&gsp[nu].deriv(mu)).add(&gsp[mu].value().comm(&gsp[nu].value()));
                    // 1/4 R_{mu nu alpha beta} gamma^{alpha beta}, with R_{mu nu alpha beta} = (R)_alpha^sigma eta_{sigma beta}
                    let mut quarter = CMat::zeros(4);
                    for al in 0..4 {
                        for be in 0..4 {
                            if al != be {
                                quarter = quarter.add(&gam_mn(&g0, al, be).scale(rv.get(al, be).scale(0.25 * low(be))));
                            }
                        }
                    }
                    d49b = d49b.max(fs.sub(&quarter).max_abs());
                    worst_half = worst_half.max(fs.sub(&quarter.scale_r(2.0)).max_abs() / fs.max_abs().max(1e-300));
                    // Lord's printed factors: C = 1/2 W (4.20) and F = 1/2 R(C) gamma (4.48): the curvature of C = W/2
                    let half_w: Vec<CMat> = (0..4).map(|k| wmat[k].value().scale_r(0.5)).collect();
                    let rc = wmat[mu].deriv(nu).scale_r(0.5).sub(&wmat[nu].deriv(mu).scale_r(0.5)).add(&half_w[mu].comm(&half_w[nu]));
                    let mut lord = CMat::zeros(4);
                    for al in 0..4 {
                        for be in 0..4 {
                            if al != be {
                                lord = lord.add(&gam_mn(&g0, al, be).scale(rc.get(al, be).scale(0.5 * low(be))));
                            }
                        }
                    }
                    worst_lin = worst_lin.max(fs.sub(&lord).max_abs());
                }
            }
            if amp_i == 0 {
                d49_lord_half = d49_lord_half.min(worst_half);
            }
            if ip == 0 {
                lin_ratio.push(worst_lin);
            }
        }
    }
    rep.add("L4-9", "(4.47), (4.48): for D_mu = d_mu - Gamma_mu (coordinate derivatives, no torsion), [D_mu, D_nu] psi = (d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu]) psi (12 points, exact second derivatives)", d48, 1e-12);
    rep.add("L4-10a", "the spinor image of a vector connection: Gamma_mu = 1/4 omega_{mu ab} gamma^{ab} satisfies [Gamma_mu, gamma_b] = -(W_mu)_b^e gamma_e, that is D_mu gamma_b = 0 with constant gamma (Lord's special gauge (5.2))", d49a, 1e-13);
    rep.add("L4-10b", "then the spinor curvature is d_nu Gamma_mu - d_mu Gamma_nu + [Gamma_mu, Gamma_nu] = 1/4 R_{mu nu alpha beta} gamma^{alpha beta}, with R the curvature (4.46) of the vector connection: exactly, to all orders", d49b, 1e-12);
    rep.add_bool("L4-10c", "(4.47), (4.48) as printed, 1/2 R_{mu nu alpha beta} gamma^{alpha beta}, with R the curvature of the same (compatible) vector connection: twice the spinor curvature", d49_lord_half > 0.9, &format!("smallest relative deviation of the printed 1/2: {:.4} (1 for a factor 2)", d49_lord_half));
    find.push(vec!["L4-10c".into(), "(4.48) with 1/2 and the compatible C: smallest relative deviation".into(), f(d49_lord_half)]);
    let ratio = if lin_ratio.len() == 2 && lin_ratio[1] > 0.0 { lin_ratio[0] / lin_ratio[1] } else { 0.0 };
    rep.add_bool("L4-10d", "with Lord's printed factors together, C = W/2 from (4.20) and 1/2 R(C) gamma^{ab} in (4.48), the spinor curvature is reproduced only to first order in the connection: the deviation is of second order (it falls by 100 when the connection falls by 10)", ratio > 90.0 && ratio < 110.0, &format!("deviation at amplitude 0.4: {:.3e}; at 0.04: {:.3e}; ratio {:.2}", lin_ratio.first().copied().unwrap_or(0.0), lin_ratio.get(1).copied().unwrap_or(0.0), ratio));
    find.push(vec!["L4-10d".into(), "printed factors (4.20), (4.48): ratio of the deviations for amplitudes 0.4 and 0.04".into(), f(ratio)]);
}

/// a column vector of dual numbers as a 4x1 matrix
fn col(v: &[D4]) -> DMat {
    let mut m = DMat::new(v.len(), 1);
    for (i, z) in v.iter().enumerate() {
        m.set(i, 0, *z);
    }
    m
}

// ------------------------------------------------------------------------------------------------------------------
// Part II: the dictionary between Lord's objects and the project's
// ------------------------------------------------------------------------------------------------------------------
fn project(rep: &mut Report, rng: &mut Rng) {
    let s = sig_set();
    // P-1 to P-4: the matrices
    let mut d1 = 0.0f64;
    for p in 0..4 {
        d1 = d1.max(s[p].sub(&dirac::sigma(p)).max_abs()).max(bar(&s[p]).sub(&dirac::sigma_bar(p)).max_abs());
    }
    rep.add("P-1", "Lord's sigma^mu (1.1) and sigma-bar^mu (1.2) are the project's sigma^LL_p and sigma-bar_p (the Pauli matrices of the chiral basis)", d1, 0.0);
    rep.add("P-2", "Lord's epsilon (1.1) is the project's E_LL", eps().sub(&dirac::eps2()).max_abs(), 0.0);
    let mut d3 = cc().sub(&dirac::cmat_c()).max_abs();
    for p in 0..4 {
        d3 = d3.max(gam(&s[p]).sub(&dirac::gamma(p)).max_abs());
    }
    rep.add("P-3", "Lord's gamma^mu of the Weyl representation (1.9) are the project's gamma^p_LL of the chiral basis, and his C (1.14) is C_LL", d3, 0.0);
    rep.add("P-4", "Lord's gamma^5 (1.9) is minus the project's chirality matrix gamma^ch_LL = i gamma^0 gamma^1 gamma^2 gamma^3", gam5().add(&dirac::gamma_ch()).max_abs(), 1e-15);
    // P-5, P-6, P-7: unit bi-quaternions
    let (mut d5, mut d6a, mut d6b, mut d7) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _ in 0..50 {
        let mg = random_unit_biq(rng, 0.8); // M_LL(g) of a unit bi-quaternion g
        let mgt = mg.dagger().inv2(); // M_LL(g-tilde) = (M_LL(g)^dag)^{-1}
        // P-5: Lord's 4x4 S = diag(S, (S^dag)^{-1}) with his S = M_LL(g-tilde) is the project's spinor matrix S_LL(g)
        let lord_s4 = CMat::block_diag(&mgt, &mgt.dagger().inv2());
        d5 = d5.max(lord_s4.sub(&dirac::s_spin(&mg)).max_abs());
        // P-6a: the project's conjugation of a bi-quaternion is Lord's sigma-bar: M_LL(b-bar) = -epsilon M_LL(b)^T epsilon
        let mut cf = [C64::ZERO; 4];
        for z in cf.iter_mut() {
            *z = c(rng.unit(), rng.unit());
        }
        let b = biq_of(&CMat::m2(cf[0], cf[1], cf[2], cf[3]));
        d6a = d6a.max(mll(&q_bar(&b)).sub(&bar(&mll(&b))).max_abs());
        // P-6b: the transformed frame f'_p = g-tilde f_p g-tilde^dag has the Dirac matrices (3.6) of Lord's (4.5) with
        // sigma'^p = S sigma^p S^dag, and they equal S_LL(g) gamma^p S_LL(g)^{-1} (3.9)
        for p in 0..4 {
            let sp = mgt.mul(&s[p]).mul(&mgt.dagger()); // M_LL(f'_p)
            let fbar = mll(&q_bar(&biq_of(&sp))); // M_LL(f'-bar_p)
            let gproj = dirac::gamma_frame(&sp, &fbar);
            let glord = gam(&sp);
            let gconj = dirac::s_spin(&mg).mul(&dirac::gamma(p)).mul(&dirac::s_spin(&mg).inv_any());
            d6b = d6b.max(gproj.sub(&glord).max_abs()).max(gproj.sub(&gconj).max_abs());
        }
        // P-7: Lord's Lambda (1.8) of L = M_LL(g-tilde) is the transpose of the project's L_LL(g) (2.10)
        let lam = lorentz_of(&mgt);
        let ll = lorentz_matrix(&mg);
        for nu in 0..4 {
            for mu in 0..4 {
                d7 = d7.max((lam[nu][mu] - ll[mu][nu]).abs());
            }
        }
    }
    rep.add("P-5", "Lord's 4x4 change of representation (4.1), diag(S, (S^dag)^{-1}) with S = M_LL(g-tilde) = (M_LL(g)^dag)^{-1}, is the project's spinor matrix S_LL(g) = diag(M_LL(g-tilde), M_LL(g)) of the unit bi-quaternion g (50 random g)", d5, 1e-13);
    rep.add("P-6a", "the project's conjugation of bi-quaternions is Lord's sigma-bar: M_LL(b-bar) = -epsilon M_LL(b)^T epsilon for every bi-quaternion b (50 random b)", d6a, 1e-14);
    rep.add("P-6b", "the frame f'_p = g-tilde f_p g-tilde^dag of the project is Lord's change of representation (1.22): its Dirac matrices (3.6) are Lord's gamma (4.5) of sigma'^p = S sigma^p S^dag and equal S_LL(g) gamma^p S_LL(g)^{-1}", d6b, 1e-12);
    rep.add("P-7", "Lord's Lorentz matrix (1.8) of L = M_LL(g-tilde) is the transpose of the project's L_LL(g) (2.10): Lambda_nu^mu = (L_LL(g))_{mu nu}", d7, 1e-12);
    // P-8: the spin connection of GEE: nabla = d + Gamma^LL with Gamma'^LL = S Gamma^LL S^{-1} - (d S) S^{-1} is Lord's
    // derivative with Gamma_a = -Gamma^LL_a and the corrected law (L4-2)
    let mut d8 = 0.0f64;
    for _ in 0..NPT {
        let x = rand_point(rng);
        let gf = rand_sl2c_field(rng, &x, 0.6); // M_LL(g(x)) of a local unit bi-quaternion
        let sll = DMat::block_diag(&gf.dagger().inv2(), &gf); // S_LL(g(x))
        let slli = DMat::block_diag(&gf.dagger(), &gf.inv2());
        let gll: Vec<DMat> = (0..4).map(|_| lorentz_alg(&rand_traceless(rng, &x, 0.5))).collect();
        let psi = rand_spinor(rng, &x, 1.0);
        let spsi = dmatvec(&sll, &psi);
        for a in 0..4 {
            let gp = sll.value().mul(&gll[a].value()).mul(&slli.value()).sub(&sll.deriv(a).mul(&slli.value()));
            let psiv: Vec<C64> = psi.iter().map(|z| z.v).collect();
            let dpsi: Vec<C64> = psi.iter().map(|z| z.d[a]).collect();
            let gp_psi = gll[a].value().matvec(&psiv);
            let cov: Vec<C64> = dpsi.iter().zip(&gp_psi).map(|(u, w)| *u + *w).collect(); // nabla psi = d psi + Gamma^LL psi
            let rhs = sll.value().matvec(&cov);
            let spv: Vec<C64> = spsi.iter().map(|z| z.v).collect();
            let spd: Vec<C64> = spsi.iter().map(|z| z.d[a]).collect();
            let lhs: Vec<C64> = spd.iter().zip(gp.matvec(&spv)).map(|(u, w)| *u + w).collect();
            let e: Vec<C64> = lhs.iter().zip(&rhs).map(|(u, w)| *u - *w).collect();
            d8 = d8.max(vmax(&e) / vmax(&rhs).max(1e-300));
            // Lord's form with Gamma_a = -Gamma^LL_a: d - Gamma = d + Gamma^LL, and his corrected law is the project's law
            let lord_gp = sll.value().mul(&gll[a].value().scale_r(-1.0)).mul(&slli.value()).add(&sll.deriv(a).mul(&slli.value()));
            d8 = d8.max(lord_gp.add(&gp).max_abs());
        }
    }
    // P-9: the changes of representation that keep the Weyl form.  For a matrix A of SL(2,C), the real-linear map
    // C -> (A X_r C - (A X_r C)^dag)_{r = 0..3} (X_r = sigma_r, a real basis of the Hermitian matrices) has a null
    // space of real dimension 1, spanned by A^dag: A X C is Hermitian for every Hermitian X exactly when C = kappa A^dag
    let (mut d9, mut null_max, mut gap_min) = (0.0f64, 0.0f64, f64::MAX);
    for _ in 0..50 {
        let am = rand_sl2c(rng, 0.8);
        // the 16 x 8 real matrix of the map, built column by column from the 8 real basis directions of C
        let mut cols: Vec<Vec<f64>> = Vec::new();
        for k in 0..8 {
            let mut cm = CMat::zeros(2);
            let z = if k % 2 == 0 { C64::ONE } else { C64::I };
            cm.set((k / 2) / 2, (k / 2) % 2, z);
            let mut v = Vec::new();
            for r in 0..4 {
                let m = am.mul(&s[r]).mul(&cm);
                let anti = m.sub(&m.dagger());
                for e in anti.a.iter() {
                    v.push(e.re);
                    v.push(e.im);
                }
            }
            cols.push(v);
        }
        // Gram matrix M^T M (8 x 8, real symmetric) and its eigenvalues
        let mut gram = CMat::zeros(8);
        for i in 0..8 {
            for j in 0..8 {
                let x: f64 = cols[i].iter().zip(&cols[j]).map(|(u, w)| u * w).sum();
                gram.set(i, j, c(x, 0.0));
            }
        }
        let (ev, _) = lgcore::cmat::eigh(&gram);
        let mut evs = ev.clone();
        evs.sort_by(|u, w| u.partial_cmp(w).unwrap());
        let scale = evs[7].max(1e-300);
        // the smallest eigenvalue is zero up to the rounding of the eigensolver (about 1e-16 of the largest)
        null_max = null_max.max(evs[0].abs() / scale);
        gap_min = gap_min.min(evs[1] / scale);
        // A^dag satisfies the equations
        for r in 0..4 {
            let m = am.mul(&s[r]).mul(&am.dagger());
            d9 = d9.max(m.sub(&m.dagger()).max_abs());
        }
    }
    rep.add_bool("P-9", "the changes of representation that keep the Weyl form: for 50 random A of SL(2,C), the matrices C for which A X C is Hermitian for every Hermitian X form a real line spanned by A^dag (the Gram matrix of the map has exactly one zero eigenvalue); with det A = det B this leaves S = diag(A, B), B = +-(A^dag)^{-1}: Lord's group and the reflection by gamma^5", null_max < 1e-13 && gap_min > 1e-6 && d9 < 1e-13, &format!("largest smallest eigenvalue relative to the largest {:.1e} (zero to rounding); smallest second eigenvalue relative to the largest {:.3e}; A sigma A^dag Hermitian to {:.1e}", null_max, gap_min, d9));
    rep.add("P-8", "the Lorentz part of the GEE derivative of the earlier hypotheses, nabla_p = d_p + Gamma^LL_p with Gamma'^LL = S_LL Gamma^LL S_LL^{-1} - (d_p S_LL) S_LL^{-1}, is covariant, and it is Lord's derivative (4.13) with Gamma_a = -Gamma^LL_a and his inhomogeneous term corrected (L4-2) (relative deviation, 12 points)", d8, 1e-13);
}

/// the inverse of a general 4x4 complex matrix, through the solver of linear systems
trait InvAny {
    fn inv_any(&self) -> CMat;
}
impl InvAny for CMat {
    fn inv_any(&self) -> CMat {
        let n = self.rows;
        let mut r = CMat::zeros(n);
        for j in 0..n {
            let mut e = vec![C64::ZERO; n];
            e[j] = C64::ONE;
            let x = lgcore::linalg::lu_solve(self, &e).expect("singular matrix");
            for i in 0..n {
                r.set(i, j, x[i]);
            }
        }
        r
    }
}

fn main() {
    let mut rep = Report::new(
        "PROJECT spacetimes, Revision_Lord_Gravity: lg_lord -- the checks of the equations of E. A. Lord, \
         General relativity from gauge invariance (1971), Part I of lord_gravity.md, and of the association of the \
         SL(2,C) of the project with his gauge group, Part II",
    );
    rep.note("tolerances: exact identities of the matrices 0 or 1e-15; products of random matrices 1e-12 to 1e-14 (rounding); \
              findings about an equation as printed pass when the printed form fails by the stated amount");
    let mut rng = Rng(0x2026_1122_C23);
    let mut find: Vec<Vec<String>> = Vec::new();
    section1(&mut rep, &mut rng, &mut find);
    section4(&mut rep, &mut rng, &mut find);
    project(&mut rep, &mut rng);
    let dd = data_dir(env!("CARGO_MANIFEST_DIR"));
    write_csv(&dd.join("lg_lord_findings.csv"), &["check", "quantity", "value"], &find);
    rep.write(&dd.join("hck_lg_lord_checks.txt"));
    let (np, n) = rep.counts();
    println!("lg_lord: {} of {} checks passed", np, n);
    if np != n {
        std::process::exit(1);
    }
}
