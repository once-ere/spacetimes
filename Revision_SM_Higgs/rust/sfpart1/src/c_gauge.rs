//! Checks C: the gauge group GEE = SL(2,C) x SU(2)_v x U(1)^F_LL and its covariant derivatives (Section 4 of
//! superfluid.md).  Every derivative is exact (forward-mode automatic differentiation to second order, module dual).
//!
//! The fields are smooth test functions on M31 with pseudo-random parameters; the gauge transformations are local
//! (functions of the point).  At each of 12 pseudo-random points the program evaluates both sides of each covariance
//! law and records the largest deviation.
use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::dual::{dmatvec, DMat, D4};
use sfcore::report::Report;
use sfcore::rng::Rng;

/// a smooth real test function f(x) = a0 + a1 sin(k1.x + ph1) + a2 sin(k2.x + ph2) with fixed random parameters
#[derive(Clone)]
struct Smooth {
    a: [f64; 3],
    k: [[f64; 4]; 2],
    ph: [f64; 2],
}
impl Smooth {
    fn new(rng: &mut Rng, offset: f64) -> Smooth {
        Smooth {
            a: [offset + 0.5 * rng.unit(), 0.6 * rng.unit(), 0.4 * rng.unit()],
            k: [
                [rng.unit(), rng.unit(), rng.unit(), rng.unit()],
                [1.5 * rng.unit(), 1.5 * rng.unit(), 1.5 * rng.unit(), 1.5 * rng.unit()],
            ],
            ph: [3.0 * rng.unit(), 3.0 * rng.unit()],
        }
    }
    fn eval(&self, xs: &[D4; 4]) -> D4 {
        let mut s = D4::re(self.a[0]);
        for n in 0..2 {
            let mut arg = D4::re(self.ph[n]);
            for p in 0..4 {
                arg = arg + xs[p].scale(c(self.k[n][p], 0.0));
            }
            s = s + arg.sin().scale(c(self.a[n + 1], 0.0));
        }
        s
    }
}

/// the coordinates of a point as D4 numbers
fn coords(x: &[f64; 4]) -> [D4; 4] {
    [D4::coord(x, 0), D4::coord(x, 1), D4::coord(x, 2), D4::coord(x, 3)]
}

fn dm_from_c(m: &CMat) -> DMat {
    DMat::from_cmat(m)
}

/// the largest deviation between the values of two D4 matrices
fn dev_val(a: &DMat, b: &DMat) -> f64 {
    a.value().sub(&b.value()).max_abs()
}

/// the matrix sum_iota f_iota sigma_iota / 2 (an element of su(2) times i, Hermitian when the f are real)
fn su2_field(fs: &[D4; 3]) -> DMat {
    let mut m = DMat::new(2, 2);
    for i in 0..3 {
        m = m.add(&dm_from_c(&sigma(i + 1).scale_r(0.5)).scale(fs[i]));
    }
    m
}

/// commutator of D4 matrices
fn dcomm(a: &DMat, b: &DMat) -> DMat {
    a.mul(b).sub(&b.mul(a))
}

pub fn run(rep: &mut Report) {
    let mut rng = Rng(3_20261120);
    let (gv, gf) = (0.83, 0.47); // couplings g^v_LL and g^F_LL (any real values)
    // the test fields
    let alpha_f: Vec<Smooth> = (0..3).map(|_| Smooth::new(&mut rng, 1.2)).collect(); // isospin gauge parameters
    let theta_f = Smooth::new(&mut rng, 0.3); // fermion-number phase
    let w_f: Vec<Vec<Smooth>> = (0..4).map(|_| (0..3).map(|_| Smooth::new(&mut rng, 0.0)).collect()).collect(); // W^iota_p
    let b_f: Vec<Smooth> = (0..4).map(|_| Smooth::new(&mut rng, 0.0)).collect(); // B_p
    let rot_f: Vec<Smooth> = (0..3).map(|_| Smooth::new(&mut rng, 0.7)).collect(); // Lorentz: rotation angles
    let boo_f: Vec<Smooth> = (0..3).map(|_| Smooth::new(&mut rng, 0.4)).collect(); // Lorentz: rapidities
    let om_f: Vec<Vec<Smooth>> = (0..4).map(|_| (0..6).map(|_| Smooth::new(&mut rng, 0.0)).collect()).collect(); // Lorentz connection
    // test spinor: 8 components, sums of two plane waves exp(i k.x) with complex amplitudes
    let mut psi_k = vec![];
    for _ in 0..2 {
        psi_k.push([rng.unit(), rng.unit(), rng.unit(), rng.unit()]);
    }
    let psi_a: Vec<[C64; 2]> = (0..8).map(|_| [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())]).collect();
    let phi_a: Vec<[C64; 2]> = (0..3).map(|_| [c(rng.unit(), rng.unit()), c(rng.unit(), rng.unit())]).collect();
    let wave = |xs: &[D4; 4], k: &[f64; 4]| -> D4 {
        let mut arg = D4::ZERO;
        for p in 0..4 {
            arg = arg + xs[p].scale(c(0.0, k[p]));
        }
        arg.exp()
    };
    let eta = [1.0, -1.0, -1.0, -1.0];

    let (mut d1, mut d2, mut d3, mut d4, mut d5, mut d6, mut d7, mut d8_, mut d9, mut d10, mut d11) =
        (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for _pt in 0..12 {
        let x = [2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit(), 2.0 * rng.unit()];
        let xs = coords(&x);
        // ---------------------------------------------------------------- the fields at the point
        let al: [D4; 3] = [alpha_f[0].eval(&xs), alpha_f[1].eval(&xs), alpha_f[2].eval(&xs)];
        let th = theta_f.eval(&xs);
        // U = exp(i alpha.tau/2)
        let u = su2_field(&al).scale(D4::cst(C64::I)).exp_traceless2();
        let ud = u.dagger();
        let eith = th.scale(C64::I).exp();
        let wm: Vec<DMat> = (0..4).map(|p| su2_field(&[w_f[p][0].eval(&xs), w_f[p][1].eval(&xs), w_f[p][2].eval(&xs)])).collect();
        let bp: Vec<D4> = (0..4).map(|p| b_f[p].eval(&xs)).collect();
        let psi: Vec<D4> = (0..8).map(|cc| wave(&xs, &psi_k[0]) * D4::cst(psi_a[cc][0]) + wave(&xs, &psi_k[1]) * D4::cst(psi_a[cc][1])).collect();
        let phi: Vec<D4> = (0..3).map(|cc| wave(&xs, &psi_k[0]) * D4::cst(phi_a[cc][0]) + wave(&xs, &psi_k[1]) * D4::cst(phi_a[cc][1])).collect();
        // ---------------------------------------------------------------- transformed gauge fields
        // W'_p = U W_p U^dag + (i/g) (d_p U) U^dag;  B'_p = B_p - d_p theta / g_F
        let wpm: Vec<DMat> = (0..4)
            .map(|p| u.mul(&wm[p]).mul(&ud).add(&u.partial(p).mul(&ud).scale(D4::cst(c(0.0, 1.0 / gv)))))
            .collect();
        let bpp: Vec<D4> = (0..4)
            .map(|p| {
                let mut dth = D4::cst(th.d[p]);
                for q in 0..4 {
                    dth.d[q] = th.dd[p][q];
                }
                bp[p] - dth.scale(c(1.0 / gf, 0.0))
            })
            .collect();
        // C-2: W' is again Hermitian and traceless (real components W'^iota)
        for p in 0..4 {
            let v = wpm[p].value();
            d2 = d2.max(v.sub(&v.dagger()).max_abs()).max(v.trace().abs());
        }
        // ---------------------------------------------------------------- C-1: the spinor, isospin and fermion number
        // nabla_p Psi = d_p Psi + i g (1_4 (x) W_p) Psi + i g_F B_p Psi
        let id4 = dm_from_c(&CMat::identity(4));
        let cov = |w: &Vec<DMat>, b: &Vec<D4>, v: &Vec<D4>, p: usize| -> Vec<D4> {
            let wp8 = id4.kron(&w[p]).scale(D4::cst(c(0.0, gv)));
            let gauge = dmatvec(&wp8, v);
            (0..8)
                .map(|k| {
                    let mut dv = D4::cst(v[k].d[p]);
                    for q in 0..4 {
                        dv.d[q] = v[k].dd[p][q];
                    }
                    dv + gauge[k] + v[k] * b[p].scale(c(0.0, gf))
                })
                .collect()
        };
        let u8 = id4.kron(&u).scale(eith);
        let psip = dmatvec(&u8, &psi);
        for p in 0..4 {
            let lhs = cov(&wpm, &bpp, &psip, p);
            let rhs = dmatvec(&u8, &cov(&wm, &bp, &psi, p));
            for k in 0..8 {
                d1 = d1.max((lhs[k].v - rhs[k].v).abs());
            }
        }
        // ---------------------------------------------------------------- C-3: field strengths
        // W_pq = d_p W_q - d_q W_p + i g [W_p, W_q] -> U W_pq U^dag;  B_pq invariant
        let fs = |w: &Vec<DMat>, p: usize, q: usize| -> DMat {
            w[q].partial(p).sub(&w[p].partial(q)).add(&dcomm(&w[p], &w[q]).scale(D4::cst(c(0.0, gv))))
        };
        let bpq = |b: &Vec<D4>, p: usize, q: usize| -> C64 { b[q].d[p] - b[p].d[q] };
        for p in 0..4 {
            for q in 0..4 {
                let f = fs(&wm, p, q);
                let fp = fs(&wpm, p, q);
                d3 = d3.max(dev_val(&fp, &u.mul(&f).mul(&ud)));
                d3 = d3.max((bpq(&bpp, p, q) - bpq(&bp, p, q)).abs());
                // the component form: W^iota_pq = d_p W^iota_q - d_q W^iota_p - g eps_{iota a b} W^a_p W^b_q
                for iota in 0..3 {
                    let comp = f.value().mul(&sigma(iota + 1)).trace(); // = W^iota_pq (tr(sigma_a sigma_b/2) = delta)
                    let wv = |pp: usize, a: usize| w_f[pp][a].eval(&xs);
                    let mut want = wv(q, iota).d[p] - wv(p, iota).d[q];
                    for a in 0..3 {
                        for b in 0..3 {
                            want = want - (wv(p, a).v * wv(q, b).v).scale(gv * levi(iota, a, b));
                        }
                    }
                    d3 = d3.max((comp - want).abs());
                }
            }
        }
        // ---------------------------------------------------------------- C-4: the isotriplet Phi of fermion number 2
        // nabla_p Phi = d_p Phi - g W_p x Phi + 2 i g_F B_p Phi;  Phi' = e^{2 i theta} R(U) Phi
        let rmat = |uu: &DMat| -> DMat {
            // R_{a b} = (1/2) tr(sigma_a U sigma_b U^dag)
            let mut r = DMat::new(3, 3);
            let uud = uu.dagger();
            for a in 0..3 {
                for b in 0..3 {
                    let m = dm_from_c(&sigma(a + 1)).mul(uu).mul(&dm_from_c(&sigma(b + 1))).mul(&uud);
                    r.set(a, b, (m.get(0, 0) + m.get(1, 1)).scale(c(0.5, 0.0)));
                }
            }
            r
        };
        let wcomp = |w: &DMat| -> [D4; 3] {
            // W^iota = tr(sigma_iota W) (W = sum W^iota sigma_iota/2)
            let mut out = [D4::ZERO; 3];
            for i in 0..3 {
                let m = dm_from_c(&sigma(i + 1)).mul(w);
                out[i] = m.get(0, 0) + m.get(1, 1);
            }
            out
        };
        let covphi = |w: &Vec<DMat>, b: &Vec<D4>, ph: &Vec<D4>, p: usize| -> Vec<D4> {
            let wc = wcomp(&w[p]);
            (0..3)
                .map(|a| {
                    let mut dv = D4::cst(ph[a].d[p]);
                    for q in 0..4 {
                        dv.d[q] = ph[a].dd[p][q];
                    }
                    let mut cross = D4::ZERO;
                    for i in 0..3 {
                        for j in 0..3 {
                            cross = cross + (wc[i] * ph[j]).scale(c(levi(a, i, j), 0.0));
                        }
                    }
                    dv - cross.scale(c(gv, 0.0)) + ph[a] * b[p].scale(c(0.0, 2.0 * gf))
                })
                .collect()
        };
        let r = rmat(&u);
        let e2ith = th.scale(c(0.0, 2.0)).exp();
        let phip = dmatvec(&r.scale(e2ith), &phi);
        for p in 0..4 {
            let lhs = covphi(&wpm, &bpp, &phip, p);
            let rhs = dmatvec(&r.scale(e2ith), &covphi(&wm, &bp, &phi, p));
            for a in 0..3 {
                d4 = d4.max((lhs[a].v - rhs[a].v).abs());
            }
        }
        // and R(U) is a rotation: R R^T = 1, det R = 1 (values)
        let rv = r.value();
        d4 = d4.max(rv.mul(&rv.transpose()).sub(&CMat::identity(3)).max_abs());
        // ---------------------------------------------------------------- C-5: the Yang-Mills operator
        // D_q F^{q p} = d_q F^{q p} + i g [W_q, F^{q p}] is covariant: D'_q F'^{q p} = U (D_q F^{q p}) U^dag,
        // with F' = U F U^dag (C-3); indices raised with eta
        for p in 0..4 {
            let mut lhs = DMat::new(2, 2);
            let mut rhs = DMat::new(2, 2);
            for q in 0..4 {
                let sgn = D4::re(eta[q] * eta[p]);
                let f = fs(&wm, q, p).scale(sgn);
                let fprime = u.mul(&f).mul(&ud);
                let df = |m: &DMat| -> DMat { m.partial(q) };
                lhs = lhs.add(&df(&fprime).add(&dcomm(&wpm[q], &fprime).scale(D4::cst(c(0.0, gv)))));
                rhs = rhs.add(&df(&f).add(&dcomm(&wm[q], &f).scale(D4::cst(c(0.0, gv)))));
            }
            d5 = d5.max(dev_val(&lhs, &u.mul(&rhs).mul(&ud)));
        }
        // ---------------------------------------------------------------- C-6: local SL(2,C) with the frame
        // M(g(x)) = exp(sum_j (-i rot_j/2 + boost_j/2) sigma_j); S = diag((M^dag)^-1, M); frame f'_p = gtilde f_p gtilde^dag
        let mut xg = DMat::new(2, 2);
        for j in 0..3 {
            let coef = rot_f[j].eval(&xs).scale(c(0.0, -0.5)) + boo_f[j].eval(&xs).scale(c(0.5, 0.0));
            xg = xg.add(&dm_from_c(&sigma(j + 1)).scale(coef));
        }
        let mg = xg.exp_traceless2();
        let mgt = mg.dagger().inv2();
        let s = DMat::block_diag(&mgt, &mg);
        let sinv = DMat::block_diag(&mg.dagger(), &mg.inv2());
        // d6: the frame: gamma^p[f'] = S gamma^p S^-1
        let gam_f: Vec<DMat> = (0..4)
            .map(|p| {
                let mf = mgt.mul(&dm_from_c(&sigma(p))).mul(&mgt.dagger());
                let mfb = mg.mul(&dm_from_c(&sigma_bar(p))).mul(&mg.dagger());
                let mut gmat = DMat::new(4, 4);
                for i in 0..2 {
                    for j in 0..2 {
                        gmat.set(i, 2 + j, mf.get(i, j));
                        gmat.set(2 + i, j, mfb.get(i, j));
                    }
                }
                gmat
            })
            .collect();
        for p in 0..4 {
            d6 = d6.max(dev_val(&gam_f[p], &s.mul(&dm_from_c(&gamma(p))).mul(&sinv)));
        }
        // d7: Gamma'_p = -(d_p S) S^-1 lies in the Lorentz algebra: diag(Y1, Y2), Y1 = -Y2^dag, tr Y2 = 0
        let gam_conn: Vec<DMat> = (0..4).map(|p| s.partial(p).mul(&sinv).scale(D4::re(-1.0))).collect();
        for p in 0..4 {
            let gv4 = gam_conn[p].value();
            let y1 = gv4.sub_block(0, 0, 2, 2);
            let y2 = gv4.sub_block(2, 2, 2, 2);
            d7 = d7.max(gv4.sub_block(0, 2, 2, 2).max_abs()).max(gv4.sub_block(2, 0, 2, 2).max_abs());
            d7 = d7.max(y1.add(&y2.dagger()).max_abs()).max(y2.trace().abs());
        }
        // d8: sum_p gamma^p[f'] (d_p + Gamma'_p)(S psi4) = S sum_p gamma^p d_p psi4 on a four-component test spinor, and
        // the invariance of the value of psi-bar gamma^p nabla_p psi
        let psi4: Vec<D4> = (0..4).map(|k| psi[2 * k]).collect();
        let spsi = dmatvec(&s, &psi4);
        let mut lhs = vec![D4::ZERO; 4];
        let mut rhs = vec![D4::ZERO; 4];
        for p in 0..4 {
            let dsp: Vec<D4> = spsi.iter().map(|z| {
                let mut r = D4::cst(z.d[p]);
                for q in 0..4 {
                    r.d[q] = z.dd[p][q];
                }
                r
            }).collect();
            let conn = dmatvec(&gam_conn[p], &spsi);
            let inner: Vec<D4> = (0..4).map(|k| dsp[k] + conn[k]).collect();
            let t = dmatvec(&gam_f[p], &inner);
            let dpsi: Vec<D4> = psi4.iter().map(|z| D4::cst(z.d[p])).collect();
            let t2 = dmatvec(&dm_from_c(&gamma(p)), &dpsi);
            for k in 0..4 {
                lhs[k] = lhs[k] + t[k];
                rhs[k] = rhs[k] + t2[k];
            }
        }
        let srhs = dmatvec(&s, &rhs);
        for k in 0..4 {
            d8_ = d8_.max((lhs[k].v - srhs[k].v).abs());
        }
        // value of psi-bar (gamma^p nabla_p psi): psi'^dag gamma^0 lhs versus psi^dag gamma^0 rhs
        let g0 = gamma(0);
        let mut l1 = C64::ZERO;
        let mut l2 = C64::ZERO;
        for a in 0..4 {
            for b in 0..4 {
                l1 += spsi[a].v.conj() * g0.get(a, b) * lhs[b].v;
                l2 += psi4[a].v.conj() * g0.get(a, b) * rhs[b].v;
            }
        }
        d8_ = d8_.max((l1 - l2).abs());
        // ---------------------------------------------------------------- C-7: the whole of GEE on the eight components
        // nabla_p = d_p + Gamma_p (x) 1_2 + i g 1_4 (x) W_p + i g_F B_p with a general Lorentz connection
        // Gamma_p = s(u_p), u_p = sum_j (om_pj J^j + om_p,j+3 K^j); transformation Ucal = e^{i theta} S (x) U,
        // Gamma' = S Gamma S^-1 - (dS) S^-1, frame f' = gtilde f gtilde^dag
        let conn_l: Vec<DMat> = (0..4)
            .map(|p| {
                let mut mu = DMat::new(2, 2);
                for j in 0..3 {
                    let coef = om_f[p][j].eval(&xs).scale(c(0.0, -0.5)) + om_f[p][j + 3].eval(&xs).scale(c(0.5, 0.0));
                    mu = mu.add(&dm_from_c(&sigma(j + 1)).scale(coef));
                }
                // s(u) = diag(-M(u)^dag, M(u))
                DMat::block_diag(&mu.dagger().scale(D4::re(-1.0)), &mu)
            })
            .collect();
        let conn_lp: Vec<DMat> = (0..4).map(|p| s.mul(&conn_l[p]).mul(&sinv).add(&gam_conn[p])).collect();
        let id2 = dm_from_c(&CMat::identity(2));
        let ucal = s.kron(&u).scale(eith);
        let cov_full = |gl: &Vec<DMat>, w: &Vec<DMat>, b: &Vec<D4>, v: &Vec<D4>, p: usize| -> Vec<D4> {
            let mut op = gl[p].kron(&id2).add(&id4.kron(&w[p]).scale(D4::cst(c(0.0, gv))));
            for k in 0..8 {
                let z = op.get(k, k) + b[p].scale(c(0.0, gf));
                op.set(k, k, z);
            }
            let g = dmatvec(&op, v);
            (0..8)
                .map(|k| {
                    let mut dv = D4::cst(v[k].d[p]);
                    for q in 0..4 {
                        dv.d[q] = v[k].dd[p][q];
                    }
                    dv + g[k]
                })
                .collect()
        };
        let psi_full = dmatvec(&ucal, &psi);
        let mut lhs = vec![D4::ZERO; 8];
        let mut rhs = vec![D4::ZERO; 8];
        for p in 0..4 {
            let a = dmatvec(&gam_f[p].kron(&id2), &cov_full(&conn_lp, &wpm, &bpp, &psi_full, p));
            let b = dmatvec(&dm_from_c(&d8(&gamma(p))), &cov_full(&conn_l, &wm, &bp, &psi, p));
            for k in 0..8 {
                lhs[k] = lhs[k] + a[k];
                rhs[k] = rhs[k] + b[k];
            }
        }
        let urhs = dmatvec(&ucal, &rhs);
        for k in 0..8 {
            d9 = d9.max((lhs[k].v - urhs[k].v).abs());
        }
        // C-8: the curvature of the Lorentz connection R_pq = d_p Gamma_q - d_q Gamma_p + [Gamma_p, Gamma_q] -> S R S^-1
        for p in 0..4 {
            for q in 0..4 {
                let rr = |gl: &Vec<DMat>| gl[q].partial(p).sub(&gl[p].partial(q)).add(&dcomm(&gl[p], &gl[q]));
                d10 = d10.max(dev_val(&rr(&conn_lp), &s.mul(&rr(&conn_l)).mul(&sinv)));
            }
        }
        // C-9: the Lagrangian density of the Dirac doublet is invariant at the point (its value):
        // Psi-bar' (i gamma^p[f'] nabla'_p - m) Psi' = Psi-bar (i gamma^p nabla_p - m) Psi, Psi-bar = Psi^dag (gamma^0 (x) 1_2)
        let m = 0.2;
        let g08 = d8(&gamma(0));
        let lag = |v: &Vec<D4>, dirac: &Vec<D4>| -> C64 {
            let mut s1 = C64::ZERO;
            for a in 0..8 {
                for b in 0..8 {
                    let ga = g08.get(a, b);
                    if ga.abs() > 0.0 {
                        s1 += v[a].v.conj() * ga * (C64::I * dirac[b].v - v[b].v.scale(m));
                    }
                }
            }
            s1
        };
        d11 = d11.max((lag(&psi_full, &lhs) - lag(&psi, &rhs)).abs());
    }
    rep.add("C-1", "SU(2)_v x U(1)^F: nabla'_p (e^{i theta} U Psi) = e^{i theta} U nabla_p Psi with W' = U W U^dag + (i/g)(dU)U^dag, B' = B - (d theta)/g_F (12 points, 4 directions)", d1, 1e-12);
    rep.add("C-2", "the transformed W'_p is again Hermitian and traceless: real components W'^iota_p", d2, 1e-13);
    rep.add("C-3", "field strengths: W'_pq = U W_pq U^dag, B'_pq = B_pq, and W^iota_pq = dW - dW - g eps W W", d3, 1e-12);
    rep.add("C-4", "isotriplet of fermion number 2: nabla Phi = dPhi - g W x Phi + 2 i g_F B Phi is covariant, Phi' = e^{2 i theta} R(U) Phi; R(U) orthogonal", d4, 1e-12);
    rep.add("C-5", "the Yang-Mills operator D_q W^{q p} is covariant (adjoint): D'_q W'^{q p} = U (D_q W^{q p}) U^dag", d5, 1e-11);
    rep.add("C-6", "local SL(2,C): the Dirac matrices of the transformed frame are S(x) gamma^p S(x)^-1", d6, 1e-12);
    rep.add("C-7", "local SL(2,C): the connection Gamma'_p = -(d_p S) S^-1 lies in the Lorentz algebra (blocks -X^dag, X with tr X = 0)", d7, 1e-12);
    rep.add("C-8", "local SL(2,C): gamma^p[f'] (d_p + Gamma'_p)(S psi) = S gamma^p d_p psi, and psi-bar gamma^p nabla_p psi is invariant", d8_, 1e-11);
    rep.add("C-9", "all of GEE at once on the eight components, with a general Lorentz connection, W and B: the Dirac operator is covariant", d9, 1e-11);
    rep.add("C-10", "the curvature of the Lorentz connection transforms as R'_pq = S R_pq S^-1", d10, 1e-11);
    rep.add("C-11", "the value of the Dirac Lagrangian density Psi-bar (i gamma^p nabla_p - m) Psi is invariant under all of GEE", d11, 1e-11);
}
