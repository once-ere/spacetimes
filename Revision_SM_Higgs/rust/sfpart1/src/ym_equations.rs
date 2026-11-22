//! Check E-14: the Euler-Lagrange equations of the Yang-Mills Lagrangian with a source (Section 5.2 of superfluid.md,
//! equation (5.10)), verified by automatic differentiation.
//!
//! The Lagrangian density of the gauge field of SU(2)_v with the coupling g to an external current J^(a,p) (upper index,
//! contracted with the lower index of W^a_p) is
//!     L = -(1/4) sum_a sum_pq eta_pp eta_qq (F^a_pq)^2 - g sum_a sum_p W^a_p J^(a,p),
//!     F^a_pq = d_p W^a_q - d_q W^a_p - g sum_bc eps_abc W^b_p W^c_q,
//! a polynomial of degree four in the twelve components W^a_p and their 48 first derivatives u^a_pq = d_p W^a_q.  Its
//! Euler-Lagrange expressions are
//!     E^a_q = dL/dW^a_q - sum_p d_p ( dL/du^a_pq ),
//! and (5.10) states that they equal  sum_p d_p F^(a,pq) - g sum_bcp eps_abc W^b_p F^(c,pq) - g J^(a,q)
//! with F^(a,pq) = eta_pp eta_qq F^a_pq, i.e. that the field equations E^a_q = 0 are the Yang-Mills equations.
//!
//! The method, independent of a symbolic derivation: the partial derivatives of the polynomial L with respect to its 60
//! arguments are taken by the five-point difference in the argument, which is EXACT for a polynomial of degree four
//! (f'(0) = [f(-2h) - 8 f(-h) + 8 f(h) - f(2h)]/(12 h) has no error for such f); the arguments are the values of smooth
//! test fields W^a_p(x) (sums of sines with random coefficients) and of their first derivatives, carried as numbers of
//! the type D4 (value, first and second derivatives in x, by automatic differentiation), so that the total derivative
//! d_p of dL/du^a_pq along x is read off the result.  The claimed right side is computed directly from F and its
//! derivatives.  Both sides are compared at 6 random points for all 12 components.
use sfcore::c64::{c, C64};
use sfcore::dual::D4;
use sfcore::report::Report;
use sfcore::rng::Rng;

const ETA: [f64; 4] = [1.0, -1.0, -1.0, -1.0];

/// a uniform pseudo-random number between lo and hi (rng.unit() is uniform in [-1, 1))
fn uni(rng: &mut Rng, lo: f64, hi: f64) -> f64 {
    lo + (hi - lo) * 0.5 * (rng.unit() + 1.0)
}

fn eps(a: usize, b: usize, cc: usize) -> f64 {
    if a == b || b == cc || a == cc {
        0.0
    } else if (a, b, cc) == (0, 1, 2) || (a, b, cc) == (1, 2, 0) || (a, b, cc) == (2, 0, 1) {
        1.0
    } else {
        -1.0
    }
}

/// the Lagrangian density L(W, u) for the arguments w[a][p] = W^a_p and u[a][p][q] = d_p W^a_q, with the coupling g and
/// the current j[a][p] = J^(a,p) (constants)
fn lagrangian(w: &[[D4; 4]; 3], u: &[[[D4; 4]; 4]; 3], g: f64, j: &[[f64; 4]; 3]) -> D4 {
    let mut l = D4::ZERO;
    for a in 0..3 {
        for p in 0..4 {
            for q in 0..4 {
                let mut f = u[a][p][q] - u[a][q][p];
                for b in 0..3 {
                    for cc in 0..3 {
                        let e = eps(a, b, cc);
                        if e != 0.0 {
                            f = f - (w[b][p] * w[cc][q]).scale(c(g * e, 0.0));
                        }
                    }
                }
                l = l - (f * f).scale(c(0.25 * ETA[p] * ETA[q], 0.0));
            }
        }
        for p in 0..4 {
            l = l - w[a][p].scale(c(g * j[a][p], 0.0));
        }
    }
    l
}

/// the derivative of L with respect to one argument, by the five-point difference (exact for degree four);
/// sel chooses the argument: (0, a, p, 0) for W^a_p, (1, a, p, q) for u^a_pq
fn partial(w: &[[D4; 4]; 3], u: &[[[D4; 4]; 4]; 3], g: f64, j: &[[f64; 4]; 3], sel: (usize, usize, usize, usize)) -> D4 {
    let h = 0.25;
    let mut acc = D4::ZERO;
    for (k, wt) in [(-2.0, 1.0), (-1.0, -8.0), (1.0, 8.0), (2.0, -1.0)] {
        let (mut ww, mut uu) = (*w, *u);
        let shift = D4::re(k * h);
        if sel.0 == 0 {
            ww[sel.1][sel.2] = ww[sel.1][sel.2] + shift;
        } else {
            uu[sel.1][sel.2][sel.3] = uu[sel.1][sel.2][sel.3] + shift;
        }
        acc = acc + lagrangian(&ww, &uu, g, j).scale(c(wt / (12.0 * h), 0.0));
    }
    acc
}

pub fn run(rep: &mut Report) {
    let mut rng = Rng(20261121);
    let g = 0.83;
    // the test fields W^a_p(x) = c0 + c1 sin(k1.x + ph1) + c2 sin(k2.x + ph2), as functions of the point x
    let mut coef = vec![];
    for _ in 0..12 {
        let cs = [uni(&mut rng, -0.5, 0.5), uni(&mut rng, -0.6, 0.6), uni(&mut rng, -0.4, 0.4)];
        let k1 = [uni(&mut rng, -1.0, 1.0), uni(&mut rng, -1.0, 1.0), uni(&mut rng, -1.0, 1.0), uni(&mut rng, -1.0, 1.0)];
        let k2 = [uni(&mut rng, -1.0, 1.0), uni(&mut rng, -1.0, 1.0), uni(&mut rng, -1.0, 1.0), uni(&mut rng, -1.0, 1.0)];
        let ph = [uni(&mut rng, -3.0, 3.0), uni(&mut rng, -3.0, 3.0)];
        coef.push((cs, k1, k2, ph));
    }
    let mut j = [[0.0; 4]; 3];
    for a in 0..3 {
        for p in 0..4 {
            j[a][p] = uni(&mut rng, -1.0, 1.0);
        }
    }
    let mut dev: f64 = 0.0;
    let mut scale: f64 = 0.0;
    for _ in 0..6 {
        let x = [uni(&mut rng, -2.0, 2.0), uni(&mut rng, -2.0, 2.0), uni(&mut rng, -2.0, 2.0), uni(&mut rng, -2.0, 2.0)];
        let xd: Vec<D4> = (0..4).map(|p| D4::coord(&x, p)).collect();
        // the fields as D4 (value, first and second derivatives at x)
        let mut w = [[D4::ZERO; 4]; 3];
        for a in 0..3 {
            for p in 0..4 {
                let (cs, k1, k2, ph) = coef[a * 4 + p];
                let mut a1 = D4::re(ph[0]);
                let mut a2 = D4::re(ph[1]);
                for r in 0..4 {
                    a1 = a1 + xd[r].scale(c(k1[r], 0.0));
                    a2 = a2 + xd[r].scale(c(k2[r], 0.0));
                }
                w[a][p] = D4::re(cs[0]) + a1.sin().scale(c(cs[1], 0.0)) + a2.sin().scale(c(cs[2], 0.0));
            }
        }
        // the derivatives u^a_pq = d_p W^a_q as D4: value d_p W, first derivatives d_r d_p W (the second derivatives
        // of W); their own second derivatives are not needed
        let mut u = [[[D4::ZERO; 4]; 4]; 3];
        for a in 0..3 {
            for p in 0..4 {
                for q in 0..4 {
                    let mut d = D4::cst(w[a][q].d[p]);
                    for r in 0..4 {
                        d.d[r] = w[a][q].dd[r][p];
                    }
                    u[a][p][q] = d;
                }
            }
        }
        for a in 0..3 {
            for q in 0..4 {
                // the Euler-Lagrange expression E^a_q = dL/dW^a_q - sum_p d_p (dL/du^a_pq)
                let mut el = partial(&w, &u, g, &j, (0, a, q, 0)).v;
                for p in 0..4 {
                    el = el - partial(&w, &u, g, &j, (1, a, p, q)).d[p];
                }
                // the claimed form: sum_p d_p F^(a,pq) - g sum eps_abc W^b_p F^(c,pq) - g J^(a,q)
                let fup = |cc: usize, p: usize, qq: usize| -> D4 {
                    let mut f = u[cc][p][qq] - u[cc][qq][p];
                    for b in 0..3 {
                        for d in 0..3 {
                            let e = eps(cc, b, d);
                            if e != 0.0 {
                                f = f - (w[b][p] * w[d][qq]).scale(c(g * e, 0.0));
                            }
                        }
                    }
                    f.scale(c(ETA[p] * ETA[qq], 0.0))
                };
                let mut claim = C64::ZERO;
                for p in 0..4 {
                    claim = claim + fup(a, p, q).d[p];
                    for b in 0..3 {
                        for cc in 0..3 {
                            let e = eps(a, b, cc);
                            if e != 0.0 {
                                claim = claim - (w[b][p].v * fup(cc, p, q).v) * (g * e);
                            }
                        }
                    }
                }
                claim = claim - c(g * j[a][q], 0.0);
                dev = dev.max((el - claim).abs());
                scale = scale.max(el.abs());
            }
        }
    }
    rep.add(
        "E-14",
        &format!("the Euler-Lagrange equations of L = -(1/4) F^a_pq F^(a,pq) - g W^a_p J^(a,p) are the Yang-Mills equations (5.10): d_p F^(a,pq) - g eps_abc W^b_p F^(c,pq) = g J^(a,q), all 12 components at 6 random points (exact derivatives of the polynomial L by five-point differences in its arguments, automatic differentiation in x; largest residual, the largest |E| being {:.3e})", scale),
        dev,
        1e-11,
    );
}
