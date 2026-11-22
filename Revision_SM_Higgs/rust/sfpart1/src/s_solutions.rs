//! Checks S and data: the analytic solutions of Section 7 of superfluid.md.
//! AS1 plane waves in constant gauge fields (generic, abelian, and the spin-isospin locked non-abelian field);
//! AS2 Landau levels in a constant magnetic field of the direction W^3 (Wilson lattice in x^1);
//! AS3 the uniform superfluid of the spin-zero pairing (mean field: Bogoliubov-de Gennes spectrum, gap equation, energy);
//! AS4 the kink of the pair field and its bound states (exact residuals of the analytic spinors; Wilson lattice).
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::*;
use sfcore::dual::{dmatvec, DMat, D4};
use sfcore::quad::gauss_legendre;
use sfcore::report::{f, write_csv, Report};
use sfcore::rng::Rng;
use sfcore::sturm::BlockTridiag;
use std::f64::consts::PI;
use std::path::Path;

/// the plane-wave Hamiltonian H_pw(q) = beta m + V_0 + sum_j alpha^j (q^j + g W_j + g_F B_j) for constant gauge
/// fields (w[p][iota] = W^iota_p with lower index p, b[p] = B_p)
pub fn h_planewave(q: &[f64; 3], m: f64, gv: f64, gf: f64, w: &[[f64; 3]; 4], b: &[f64; 4]) -> CMat {
    let gauge = |p: usize| -> CMat {
        let mut g = CMat::identity(8).scale_r(gf * b[p]);
        for iota in 1..4 {
            g = g.add(&t_gen(iota).scale_r(gv * w[p][iota - 1]));
        }
        g
    };
    let mut h = d8(&beta()).scale_r(m).add(&gauge(0));
    for j in 1..4 {
        h = h.add(&d8(&alpha(j)).mul(&CMat::identity(8).scale_r(q[j - 1]).add(&gauge(j))));
    }
    h
}

/// eigenvalues of a Hermitian matrix, ascending
fn eigvals(m: &CMat) -> Vec<f64> {
    eigh(m).0
}

fn max_sorted_diff(a: &[f64], b: &[f64]) -> f64 {
    let mut x = a.to_vec();
    let mut y = b.to_vec();
    x.sort_by(|p, q| p.partial_cmp(q).unwrap());
    y.sort_by(|p, q| p.partial_cmp(q).unwrap());
    x.iter().zip(&y).map(|(p, q)| (p - q).abs()).fold(0.0, f64::max)
}

/// runs AS1 to S4; returns the named values that the manuscript prints (written by main to data/part1_values.csv)
pub fn run(rep: &mut Report, data: &Path) -> Vec<(String, f64)> {
    let m = 0.2;
    let mut vals: Vec<(String, f64)> = vec![("m_LL".into(), m), ("Lambda_LL".into(), 1.0)];
    s1(rep, data, m, &mut vals);
    s2(rep, data, m, &mut vals);
    s3(rep, data, m, &mut vals);
    s4(rep, data, m, &mut vals);
    vals
}

// ==================================================================================================== S1
fn s1(rep: &mut Report, data: &Path, m: f64, vals: &mut Vec<(String, f64)>) {
    let mut rng = Rng(11);
    let (gv, gf) = (0.83, 0.47);
    // AS1-1: generic constant fields: every eigenvector chi of H_pw(q) with eigenvalue omega gives the solution
    // Psi = chi exp(-i omega x^0 + i q.x) of i gamma^p nabla_p Psi = m Psi (residual computed with exact derivatives)
    let mut dev: f64 = 0.0;
    for _ in 0..20 {
        let w = [[rng.unit(), rng.unit(), rng.unit()], [rng.unit(), rng.unit(), rng.unit()], [rng.unit(), rng.unit(), rng.unit()], [rng.unit(), rng.unit(), rng.unit()]];
        let b = [rng.unit(), rng.unit(), rng.unit(), rng.unit()];
        let q = [rng.unit(), rng.unit(), rng.unit()];
        let h = h_planewave(&q, m, gv, gf, &w, &b);
        let (ev, vecs) = eigh(&h);
        let x = [rng.unit(), rng.unit(), rng.unit(), rng.unit()];
        let xs = [D4::coord(&x, 0), D4::coord(&x, 1), D4::coord(&x, 2), D4::coord(&x, 3)];
        for n in 0..8 {
            let mut arg = xs[0].scale(c(0.0, -ev[n]));
            for j in 0..3 {
                arg = arg + xs[j + 1].scale(c(0.0, q[j]));
            }
            let ph = arg.exp();
            let psi: Vec<D4> = (0..8).map(|k| ph * D4::cst(vecs.get(k, n))).collect();
            // residual R = sum_p i gamma^p (d_p + i g W_p + i g_F B_p) Psi - m Psi
            let mut r = vec![C64::ZERO; 8];
            for p in 0..4 {
                let mut gp = CMat::identity(8).scale(c(0.0, gf * b[p]));
                for iota in 1..4 {
                    gp = gp.add(&t_gen(iota).scale(c(0.0, gv * w[p][iota - 1])));
                }
                let gam = d8(&gamma(p)).scale(C64::I);
                let dpsi: Vec<C64> = (0..8).map(|k| psi[k].d[p]).collect();
                let gpsi = gp.matvec(&psi.iter().map(|z| z.v).collect::<Vec<_>>());
                let inner: Vec<C64> = (0..8).map(|k| dpsi[k] + gpsi[k]).collect();
                let t = gam.matvec(&inner);
                for k in 0..8 {
                    r[k] += t[k];
                }
            }
            for k in 0..8 {
                dev = dev.max((r[k] - psi[k].v.scale(m)).abs());
            }
        }
    }
    rep.add("AS1-1", "plane waves chi exp(-i omega t + i q.x) with (omega, chi) an eigenpair of H_pw(q) solve the field equation in constant gauge fields (20 random field sets, 8 eigenpairs each)", dev, 1e-13);

    // AS1-2: abelian fields (W^3 and B only): omega = V0_s +- sqrt(m^2 + sum_j (q^j + e_s W^3_j + g_F B_j)^2),
    // e_s = g t_s, t_s = +1/2 (up), -1/2 (down), V0_s = g t_s W^3_0 + g_F B_0
    let mut dev: f64 = 0.0;
    for _ in 0..50 {
        let mut w = [[0.0; 3]; 4];
        let mut b = [0.0; 4];
        for p in 0..4 {
            w[p][2] = rng.unit();
            b[p] = rng.unit();
        }
        let q = [rng.unit(), rng.unit(), rng.unit()];
        let ev = eigvals(&h_planewave(&q, m, gv, gf, &w, &b));
        let mut want = vec![];
        for ts in [0.5, -0.5] {
            let v0 = gv * ts * w[0][2] + gf * b[0];
            let mut p2 = 0.0;
            for j in 0..3 {
                let pj = q[j] + gv * ts * w[j + 1][2] + gf * b[j + 1];
                p2 += pj * pj;
            }
            let e = (m * m + p2).sqrt();
            for _ in 0..2 {
                want.push(v0 + e);
                want.push(v0 - e);
            }
        }
        dev = dev.max(max_sorted_diff(&ev, &want));
    }
    rep.add("AS1-2", "abelian constant fields (W^3, B): omega = V0_s +- [m^2 + (q + e_s W^3 + g_F B)^2]^(1/2), each twice (50 random cases)", dev, 1e-13);

    // AS1-3: the spin-isospin locked field W^iota_j = w delta_iota,j (W_0 = 0, B = 0), q along x^3:
    // omega^2 in { m^2 + (q + g w/2)^2, m^2 + (q - g w/2)^2, m^2 + q^2 + (5/4) g^2 w^2 +- g w (q^2 + g^2 w^2)^(1/2) }
    let wl = 0.9;
    let lock = |wv: f64| -> [[f64; 3]; 4] {
        let mut w = [[0.0; 3]; 4];
        for j in 1..4 {
            w[j][j - 1] = wv;
        }
        w
    };
    let analytic_locked = |q: f64, gw: f64| -> Vec<f64> {
        let w2 = vec![
            m * m + (q + 0.5 * gw).powi(2),
            m * m + (q - 0.5 * gw).powi(2),
            m * m + q * q + 1.25 * gw * gw + gw * (q * q + gw * gw).sqrt(),
            m * m + q * q + 1.25 * gw * gw - gw * (q * q + gw * gw).sqrt(),
        ];
        let mut v = vec![];
        for x in w2 {
            v.push(x.sqrt());
            v.push(-x.sqrt());
        }
        v
    };
    let mut dev: f64 = 0.0;
    let mut rows = vec![];
    let nq = 121;
    for iq in 0..nq {
        let q = -1.5 + 3.0 * iq as f64 / (nq - 1) as f64;
        let ev = eigvals(&h_planewave(&[0.0, 0.0, q], m, gv, 0.0, &lock(wl), &[0.0; 4]));
        let want = analytic_locked(q, gv * wl);
        dev = dev.max(max_sorted_diff(&ev, &want));
        let mut row = vec![f(q)];
        let mut pos: Vec<f64> = want.iter().cloned().filter(|x| *x > 0.0).collect();
        pos.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for x in &pos {
            row.push(f(*x));
        }
        for x in ev.iter().filter(|x| **x > 0.0) {
            row.push(f(*x));
        }
        rows.push(row);
    }
    rep.add("AS1-3", "spin-isospin locked constant field: the eight frequencies at 121 momenta along x^3 equal the four analytic branches with both signs", dev, 1e-12);
    write_csv(
        &data.join("part1_s1_locked_dispersion.csv"),
        &["q3", "branch1", "branch2", "branch3", "branch4", "num1", "num2", "num3", "num4"],
        &rows,
    );
    // AS1-4: at q = 0 the frequencies are (m^2 + g^2 w^2/4)^(1/2) (three times each sign) and (m^2 + 9 g^2 w^2/4)^(1/2)
    let ev = eigvals(&h_planewave(&[0.0; 3], m, gv, 0.0, &lock(wl), &[0.0; 4]));
    let gw = gv * wl;
    vals.push(("AS1_gv".into(), gv));
    vals.push(("AS1_gF".into(), gf));
    vals.push(("AS1_w_locked".into(), wl));
    vals.push(("AS1_q0_J1".into(), (m * m + 0.25 * gw * gw).sqrt()));
    vals.push(("AS1_q0_J0".into(), (m * m + 2.25 * gw * gw).sqrt()));
    let (a, b) = ((m * m + 0.25 * gw * gw).sqrt(), (m * m + 2.25 * gw * gw).sqrt());
    let want = vec![-b, -a, -a, -a, a, a, a, b];
    // AS1-5: the locked field has the constant field strength W^iota_jk = -g w^2 eps_{iota j k} and is not a solution of
    // the source-free Yang-Mills equations: the left side of (5.10) is -2 g^2 w^3 delta_{iota j}, so it is sustained by the
    // constant external current j^{iota,j} = -2 g w^3 delta_{iota j} (computed from the constant fields, all indices)
    let eta = [1.0, -1.0, -1.0, -1.0];
    let wv = |io: usize, p: usize| -> f64 { if p >= 1 && io == p - 1 { wl } else { 0.0 } }; // W^iota_p (lower p)
    let fstr = |io: usize, p: usize, q: usize| -> f64 {
        // W^iota_pq for constant fields: -g sum eps W W
        let mut v = 0.0;
        for a in 0..3 {
            for b in 0..3 {
                v -= gv * levi(io, a, b) * wv(a, p) * wv(b, q);
            }
        }
        v
    };
    let mut dym: f64 = 0.0;
    for io in 0..3 {
        for pp in 0..4 {
            // sum_p [ d_p W^{io,p pp} (= 0) - g sum eps_{io a b} W^a_p W^{b, p pp} ]
            let mut lhs = 0.0;
            for p in 0..4 {
                for a in 0..3 {
                    for b in 0..3 {
                        lhs -= gv * levi(io, a, b) * wv(a, p) * eta[p] * eta[pp] * fstr(b, p, pp);
                    }
                }
            }
            let want = if pp >= 1 && io == pp - 1 { -2.0 * gv * gv * wl.powi(3) } else { 0.0 };
            dym = dym.max((lhs - want).abs());
        }
    }
    for io in 0..3 {
        for j in 1..4 {
            for k in 1..4 {
                let want = -gv * wl * wl * levi(io, j - 1, k - 1);
                dym = dym.max((fstr(io, j, k) - want).abs());
            }
        }
    }
    rep.add("AS1-5", "locked field: W^iota_jk = -g w^2 eps_{iota jk}, and the left side of the Yang-Mills equation (5.10) is -2 g^2 w^3 delta_{iota j} (it needs the external current -2 g w^3 delta_{iota j})", dym, 1e-14);
    rep.add("AS1-4", "locked field at q = 0: frequencies +-(m^2 + g^2 w^2/4)^(1/2) (3 each, total isospin+spin J = 1) and +-(m^2 + 9 g^2 w^2/4)^(1/2) (J = 0)", max_sorted_diff(&ev, &want), 1e-13);
}

// ==================================================================================================== S2
/// the Hamiltonian of one isospin component (four Dirac components, charge e) on a Wilson lattice in x^1 with the
/// field W^3_2 = -B x^1 (lower index; the three-vector potential A^2 = B x^1, magnetic field B along x^3), momenta
/// q2, q3 along x^2, x^3:
///   H = alpha^1 (-i D_c) + alpha^2 (q2 - e B x) + alpha^3 q3 + beta (m + W),
/// D_c the central difference (psi_{s+1} - psi_{s-1})/(2a) and W = (r/(2a)) (2 - T+ - T-) the Wilson term (r = 1),
/// which removes the doubled fermions of the central difference at the edge of the Brillouin zone; open ends.
/// Returned as a block-tridiagonal matrix (4x4 blocks).
fn landau_lattice(n: usize, a: f64, eb: f64, q2: f64, q3: f64, m: f64) -> BlockTridiag {
    let x0 = -0.5 * a * (n as f64 - 1.0);
    let r = 1.0;
    let (a1, a2, a3, b) = (alpha(1), alpha(2), alpha(3), beta());
    let mut diag = vec![];
    let mut upper = vec![];
    for s in 0..n {
        let x = x0 + a * s as f64;
        diag.push(a2.scale_r(q2 - eb * x).add(&a3.scale_r(q3)).add(&b.scale_r(m + r / a)));
        if s + 1 < n {
            // the coupling of site s to site s+1: alpha^1 (-i)/(2a) - beta r/(2a)
            upper.push(a1.scale(c(0.0, -0.5 / a)).sub(&b.scale_r(0.5 * r / a)));
        }
    }
    BlockTridiag { b: 4, diag, upper }
}

fn s2(rep: &mut Report, data: &Path, m: f64, vals: &mut Vec<(String, f64)>) {
    let gv: f64 = 1.0;
    let bmag: f64 = 1.0;
    let e: f64 = 0.5 * gv; // isospin up, charge g/2
    let eb: f64 = e * bmag;
    let (q2, q3) = (0.0, 0.3);
    // analytic levels omega_n = (m^2 + q3^2 + 2 n |e B|)^(1/2): n = 0 once, n >= 1 twice (each sign)
    let level = |n: usize| (m * m + q3 * q3 + 2.0 * n as f64 * eb.abs()).sqrt();
    vals.push(("AS2_gv".into(), gv));
    vals.push(("AS2_B".into(), bmag));
    vals.push(("AS2_q3".into(), q3));
    for n in 0..4 {
        vals.push((format!("AS2_level{}", n), level(n)));
    }
    // AS2-1: residual of the analytic lowest level: Psi = (a up h0, b up h0) in the chirality blocks, h0 = exp(-eB (x - x0)^2 / 2),
    // with [[-q3, m], [m, q3]] (a, b) = omega (a, b): (H - omega) Psi = 0 with exact derivatives
    let om = (m * m + q3 * q3).sqrt();
    let (av, bv) = (m, om + q3); // eigenvector of [[-q3, m],[m, q3]] with eigenvalue +omega: (-q3 - om) a + m b = 0
    let nrm = (av * av + bv * bv).sqrt();
    let mut dev: f64 = 0.0;
    let x0 = q2 / eb;
    for i in 0..41 {
        let x1 = -4.0 + 0.2 * i as f64;
        let xs = D4::coord(&[0.0, x1, 0.0, 0.0], 1);
        let d = xs - D4::re(x0);
        let h0 = (d * d).scale(c(-0.5 * eb, 0.0)).exp();
        // components: index 0 = (chirality -, spin up), 1 = (-, down), 2 = (+, up), 3 = (+, down)
        let psi = [h0.scale(c(av / nrm, 0.0)), D4::ZERO, h0.scale(c(bv / nrm, 0.0)), D4::ZERO];
        let hx = |k: usize| -> C64 {
            // (H Psi)_k = sum_l [alpha^1_kl (-i d_1) + alpha^2_kl (q2 - eB x) + alpha^3_kl q3 + beta_kl m] Psi_l
            let mut s = C64::ZERO;
            for l in 0..4 {
                s += alpha(1).get(k, l) * psi[l].d[1] * c(0.0, -1.0);
                s += alpha(2).get(k, l) * psi[l].v.scale(q2 - eb * x1);
                s += alpha(3).get(k, l) * psi[l].v.scale(q3);
                s += beta().get(k, l) * psi[l].v.scale(m);
            }
            s
        };
        for k in 0..4 {
            dev = dev.max((hx(k) - psi[k].v.scale(om)).abs());
        }
    }
    rep.add("AS2-1", "Landau level n = 0: the analytic spinor (spin up in both chirality blocks) times exp(-eB(x-x0)^2/2) solves H Psi = (m^2+q3^2)^(1/2) Psi exactly (41 points)", dev, 1e-14);
    // AS2-0: the counting-and-bisection eigenvalues of a block-tridiagonal matrix equal those of the dense matrix
    let small = landau_lattice(40, 0.25, eb, q2, q3, m);
    let dense_ev = eigvals(&small.dense());
    let bis_ev = small.eigenvalues_in(dense_ev[0] - 1.0, dense_ev[dense_ev.len() - 1] + 1.0, 1e-13);
    let d0 = if bis_ev.len() == dense_ev.len() { max_sorted_diff(&bis_ev, &dense_ev) } else { 1.0 };
    rep.add("AS2-0", "block-tridiagonal eigenvalues by inertia counting and bisection = dense eigenvalues (40 sites, 160 eigenvalues)", d0, 1e-11);
    // AS2-2: Wilson lattice, three spacings: the lowest positive levels converge to the analytic ones
    let mut rows = vec![];
    let mut errs = vec![];
    let mut levels_by_a: Vec<Vec<f64>> = vec![];
    for &(n, a) in &[(200usize, 0.1f64), (400usize, 0.05f64), (800usize, 0.025f64)] {
        let lat = landau_lattice(n, a, eb, q2, q3, m);
        let top = 0.5 * (level(3) + level(4));
        let pos = lat.eigenvalues_in(0.0, top, 1e-12);
        // analytic multiset of the first levels: n=0 once, n=1..3 twice
        let want = vec![level(0), level(1), level(1), level(2), level(2), level(3), level(3)];
        if pos.len() != 7 {
            rep.note(&format!("AS2: lattice {} sites: {} eigenvalues in (0, {:.6}) instead of 7: {:?}", n, pos.len(), top, pos));
        }
        let mut got: Vec<f64> = pos.clone();
        got.resize(7, 0.0);
        levels_by_a.push(got.clone());
        let err = got.iter().zip(&want).map(|(g, w)| ((g - w) / w).abs()).fold(0.0, f64::max);
        errs.push(err);
        for (k, (g, w)) in got.iter().zip(&want).enumerate() {
            rows.push(vec![format!("{}", n), f(a), format!("{}", k), f(*g), f(*w)]);
        }
    }
    rep.add("AS2-2", "Wilson lattice in x^1 (spacing 0.025, 800 sites): the seven lowest positive frequencies agree with the Landau levels, multiplicities included (largest relative deviation)", errs[2], 1e-2);
    rep.add_bool(
        "AS2-3",
        "the lattice deviation decreases each time the spacing is halved (0.1 -> 0.05 -> 0.025): the discretization converges",
        errs[1] < 0.75 * errs[0] && errs[2] < 0.75 * errs[1],
        &format!("relative deviations {:.3e} (a = 0.1), {:.3e} (a = 0.05), {:.3e} (a = 0.025)", errs[0], errs[1], errs[2]),
    );
    // AS2-4: the leading error of Wilson fermions is linear in the spacing a (the halving ratios of AS2-3 are close to 2),
    // so the Richardson extrapolation 2 E(a/2) - E(a) removes it; the remaining error is of order a^2
    let want = vec![level(0), level(1), level(1), level(2), level(2), level(3), level(3)];
    let mut dext: f64 = 0.0;
    for k in 0..7 {
        let ext = 2.0 * levels_by_a[2][k] - levels_by_a[1][k];
        dext = dext.max(((ext - want[k]) / want[k]).abs());
    }
    vals.push(("AS2_err_a0.1".into(), errs[0]));
    vals.push(("AS2_err_a0.05".into(), errs[1]));
    vals.push(("AS2_err_a0.025".into(), errs[2]));
    vals.push(("AS2_err_extrap".into(), dext));
    rep.add("AS2-4", "the Richardson extrapolation 2 E(0.025) - E(0.05) of the seven lattice levels agrees with the Landau levels (largest relative deviation; tolerance of order a^2)", dext, 1e-3);
    write_csv(&data.join("part1_s2_landau.csv"), &["sites", "spacing", "k", "lattice", "analytic"], &rows);
}

// ==================================================================================================== S3
/// the 16x16 Bogoliubov-de Gennes matrix of the uniform state with the pair field d (three complex components):
/// ((h - mu, P), (P^dag, -(h(-q) - mu)^T)), P = sum_iota d^iota K^iota^dag
pub fn bdg(q: &[f64; 3], m: f64, mu: f64, d: &[C64; 3]) -> CMat {
    let h = h_free(q, m).sub(&CMat::identity(8).scale_r(mu));
    let hm = h_free(&[-q[0], -q[1], -q[2]], m).sub(&CMat::identity(8).scale_r(mu));
    let mut p = CMat::zeros(8);
    for iota in 0..3 {
        p = p.add(&k_scalar(iota + 1).dagger().scale(d[iota]));
    }
    CMat::blocks(&h, &p, &p.dagger(), &hm.transpose().scale_r(-1.0))
}

/// I1(M) = int_0^L q^2 / (q^2 + M^2)^(1/2) dq = (1/2) [L (L^2 + M^2)^(1/2) - M^2 asinh(L/M)]
pub fn i1(mm: f64, lam: f64) -> f64 {
    0.5 * (lam * (lam * lam + mm * mm).sqrt() - mm * mm * (lam / mm).asinh())
}
/// I3(M) = int_0^L q^2 (q^2 + M^2)^(1/2) dq = (1/8) [L (2 L^2 + M^2)(L^2 + M^2)^(1/2) - M^4 asinh(L/M)]
pub fn i3(mm: f64, lam: f64) -> f64 {
    0.125 * (lam * (2.0 * lam * lam + mm * mm) * (lam * lam + mm * mm).sqrt() - mm.powi(4) * (lam / mm).asinh())
}
/// the energy per volume of the polar state at mu = 0 measured from the normal vacuum:
/// E(Delta) = -(2/pi^2) [I3((m^2+Delta^2)^(1/2)) - I3(m)] + Delta^2 / G
pub fn energy_polar(delta: f64, m: f64, g: f64, lam: f64) -> f64 {
    -(2.0 / (PI * PI)) * (i3((m * m + delta * delta).sqrt(), lam) - i3(m, lam)) + delta * delta / g
}
/// the solution of the gap equation 1 = (G/pi^2) I1((m^2 + Delta^2)^(1/2)) by bisection (0 if G <= G_crit)
pub fn gap_polar(m: f64, g: f64, lam: f64) -> f64 {
    let fgap = |d: f64| g / (PI * PI) * i1((m * m + d * d).sqrt(), lam) - 1.0;
    if fgap(0.0) <= 0.0 {
        return 0.0;
    }
    let (mut lo, mut hi) = (0.0, 10.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if fgap(mid) > 0.0 {
            lo = mid
        } else {
            hi = mid
        }
    }
    0.5 * (lo + hi)
}

fn s3(rep: &mut Report, data: &Path, m: f64, vals: &mut Vec<(String, f64)>) {
    let lam = 1.0;
    let mut rng = Rng(33);
    // AS3-1: the BdG spectrum of a unitary state d = Delta e^{i phi} n (n a real unit vector):
    // +-[(eps +- mu)^2 + Delta^2]^(1/2), each four times
    let mut dev: f64 = 0.0;
    for _ in 0..100 {
        let q = [rng.unit(), rng.unit(), rng.unit()];
        let mu = 0.5 * rng.unit();
        let delta = 0.5 * (rng.unit() + 1.0);
        let phi = 3.0 * rng.unit();
        let mut nv = [rng.unit(), rng.unit(), rng.unit()];
        let nn = (nv.iter().map(|x| x * x).sum::<f64>()).sqrt();
        for x in nv.iter_mut() {
            *x /= nn;
        }
        let d = [C64::cis(phi).scale(delta * nv[0]), C64::cis(phi).scale(delta * nv[1]), C64::cis(phi).scale(delta * nv[2])];
        let ev = eigvals(&bdg(&q, m, mu, &d));
        let eps = (q.iter().map(|x| x * x).sum::<f64>() + m * m).sqrt();
        let (ep, em) = (((eps + mu).powi(2) + delta * delta).sqrt(), ((eps - mu).powi(2) + delta * delta).sqrt());
        let mut want = vec![];
        for _ in 0..4 {
            want.extend_from_slice(&[ep, -ep, em, -em]);
        }
        dev = dev.max(max_sorted_diff(&ev, &want));
    }
    rep.add("AS3-1", "uniform unitary pair field: the 16 BdG eigenvalues are +-[(eps +- mu)^2 + Delta^2]^(1/2), each four times (100 random q, mu, Delta, phase, direction)", dev, 1e-12);
    // AS3-2: the energy per volume by quadrature of the BdG eigenvalues equals the closed form (mu = 0, polar state)
    let g = 22.5;
    let (gx, gw) = gauss_legendre(64);
    let energy_quad = |delta: f64| -> f64 {
        let d = [C64::ZERO, C64::ZERO, c(delta, 0.0)];
        let d0 = [C64::ZERO; 3];
        let mut s = 0.0;
        for (xi, wi) in gx.iter().zip(&gw) {
            let q = 0.5 * lam * (xi + 1.0);
            // average over three directions (the polar state is isotropic: the three agree)
            let mut acc = 0.0;
            for dir in 0..3 {
                let mut qv = [0.0; 3];
                qv[dir] = q;
                let e1: f64 = eigvals(&bdg(&qv, m, 0.0, &d)).iter().filter(|x| **x > 0.0).sum();
                let e0: f64 = eigvals(&bdg(&qv, m, 0.0, &d0)).iter().filter(|x| **x > 0.0).sum();
                acc += -0.5 * (e1 - e0) / 3.0;
            }
            s += 0.5 * lam * wi * q * q * acc;
        }
        s / (2.0 * PI * PI) + delta * delta / g
    };
    let mut dev: f64 = 0.0;
    for &delta in &[0.05, 0.1, 0.2, 0.3, 0.5] {
        dev = dev.max((energy_quad(delta) - energy_polar(delta, m, g, lam)).abs());
    }
    rep.add("AS3-2", "energy per volume of the polar state: Gauss-Legendre quadrature (64 nodes) of the BdG eigenvalues = closed form -(2/pi^2)[I3(M) - I3(m)] + Delta^2/G (5 values of Delta)", dev, 1e-12);
    // AS3-3: the gap equation: at its solution dE/dDelta = 0 (central difference) and E < 0; G_crit = pi^2 / I1(m)
    let gcrit = PI * PI / i1(m, lam);
    let delta = gap_polar(m, g, lam);
    let h = 1e-5;
    let deriv = (energy_polar(delta + h, m, g, lam) - energy_polar(delta - h, m, g, lam)) / (2.0 * h);
    rep.add("AS3-3", "the gap equation 1 = (G/pi^2) I1(M_q), M_q = (m^2 + Delta^2)^(1/2), gives the stationary point of E(Delta) (central difference of E)", deriv.abs(), 1e-9);
    rep.add_bool(
        "AS3-4",
        "for G = 22.5 > G_crit the gap is nonzero and the condensed state lies below the normal vacuum",
        delta > 0.0 && energy_polar(delta, m, g, lam) < 0.0 && g > gcrit,
        &format!("G_crit = {:.6}, Delta = {:.6}, E(Delta) = {:.6e}", gcrit, delta, energy_polar(delta, m, g, lam)),
    );
    vals.push(("AS3_G".into(), g));
    vals.push(("AS3_Gcrit".into(), gcrit));
    vals.push(("AS3_I1m".into(), i1(m, lam)));
    vals.push(("AS3_Delta".into(), delta));
    vals.push(("AS3_Mq".into(), (m * m + delta * delta).sqrt()));
    vals.push(("AS3_E".into(), energy_polar(delta, m, g, lam)));
    rep.note(&format!("AS3: G_crit = pi^2/I1(m) = {:.10}; at G = 22.5: Delta = {:.10}, quasiparticle mass (m^2+Delta^2)^(1/2) = {:.10}, E(Delta) = {:.10e}", gcrit, delta, (m * m + delta * delta).sqrt(), energy_polar(delta, m, g, lam)));
    // data: E(Delta) for several G, and Delta(G)
    let mut rows = vec![];
    for &gg in &[20.0, 21.0, gcrit, 22.0, 22.5, 23.0, 24.0] {
        for i in 0..101 {
            let dl = 0.4 * i as f64 / 100.0;
            rows.push(vec![f(gg), f(dl), f(energy_polar(dl, m, gg, lam))]);
        }
    }
    write_csv(&data.join("part1_s3_energy.csv"), &["G", "Delta", "E"], &rows);
    let mut rows = vec![];
    for i in 0..121 {
        let gg = 18.0 + 12.0 * i as f64 / 120.0;
        rows.push(vec![f(gg), f(gap_polar(m, gg, lam))]);
    }
    write_csv(&data.join("part1_s3_gap.csv"), &["G", "Delta"], &rows);
}

// ==================================================================================================== S4
fn s4(rep: &mut Report, data: &Path, m: f64, vals: &mut Vec<(String, f64)>) {
    // the matrices of the kink problem (16 x 16): A = diag(alpha^1, alpha^1*), P = ((0, K3^dag), (K3, 0)),
    // Bm = diag(beta, -beta); H = -i A d/dx + Delta(x) P + m Bm
    let a1 = d8(&alpha(1));
    let am = CMat::block_diag(&a1, &a1.conj());
    let k3 = k_scalar(3);
    let pm = CMat::blocks(&CMat::zeros(8), &k3.dagger(), &k3, &CMat::zeros(8));
    let bm = CMat::block_diag(&d8(&beta()), &d8(&beta()).scale_r(-1.0));
    let id = CMat::identity(16);
    // AS4-1: A, P, Bm anticommute pairwise and square to 1 (a representation of the Clifford algebra of three units)
    let mut dev = am.mul(&am).sub(&id).max_abs().max(pm.mul(&pm).sub(&id).max_abs()).max(bm.mul(&bm).sub(&id).max_abs());
    dev = dev.max(am.acomm(&pm).max_abs()).max(am.acomm(&bm).max_abs()).max(pm.acomm(&bm).max_abs());
    rep.add("AS4-1", "kink: the 16x16 matrices A = diag(alpha^1, alpha^1*), P (pairing K^3) and Bm = diag(beta, -beta) anticommute pairwise and square to 1, so H^2 = H0^2 + m^2", dev, 1e-14);
    // Gamma = -i A P: Hermitian, squares to 1, commutes with Bm; multiplicities of its eigenvalues
    let gam = am.mul(&pm).scale(c(0.0, -1.0));
    let (gev, gvec) = eigh(&gam);
    let nminus = gev.iter().filter(|x| **x < 0.0).count();
    rep.add_bool("AS4-2", "Gamma = -i A P has the eigenvalues -1 and +1, eight times each, and commutes with Bm", nminus == 8 && gam.comm(&bm).max_abs() < 1e-14, &format!("{} eigenvalues -1", nminus));
    // AS4-2b: in the eigenspace Gamma = -1 (where the zero modes of the massless problem live), Bm has the eigenvalues +1
    // and -1 four times each: with the mass, the n = 0 states have the energies +m and -m, four times each
    let mut proj = CMat::zeros(16);
    for n in 0..16 {
        if gev[n] < 0.0 {
            for a in 0..16 {
                for b in 0..16 {
                    let v = proj.get(a, b) + gvec.get(a, n) * gvec.get(b, n).conj();
                    proj.set(a, b, v);
                }
            }
        }
    }
    let bres = proj.mul(&bm).mul(&proj);
    let (bev, _) = eigh(&bres);
    let npl = bev.iter().filter(|x| (**x - 1.0).abs() < 1e-10).count();
    let nmi = bev.iter().filter(|x| (**x + 1.0).abs() < 1e-10).count();
    rep.add_bool("AS4-2b", "restricted to the eigenspace Gamma = -1, Bm has the eigenvalues +1 and -1 four times each (n = 0 states at +m and -m, four each)", npl == 4 && nmi == 4, &format!("+1: {}, -1: {}", npl, nmi));
    // parameters: Delta_inf = 0.5, ell = 4 (s = Delta_inf ell = 2): bound states n = 0, 1
    let (dinf, ell): (f64, f64) = (0.5, 4.0);
    let s = dinf * ell;
    let e1sq = dinf * dinf - (dinf - 1.0 / ell).powi(2);
    vals.push(("AS4_Dinf".into(), dinf));
    vals.push(("AS4_ell".into(), ell));
    vals.push(("AS4_s".into(), dinf * ell));
    vals.push(("AS4_E1sq".into(), e1sq));
    vals.push(("AS4_E_n1".into(), (m * m + e1sq).sqrt()));
    vals.push(("AS4_threshold".into(), (m * m + dinf * dinf).sqrt()));
    // a vector of the Gamma = -1 eigenspace that is also an eigenvector of Bm (Bm commutes with Gamma)
    let mut chi = vec![C64::ZERO; 16];
    for k in 0..16 {
        chi[k] = gvec.get(k, 0); // first eigenvector, eigenvalue -1
    }
    let bchi = bm.matvec(&chi);
    let plus: Vec<C64> = chi.iter().zip(&bchi).map(|(a, b)| (*a + *b).scale(0.5)).collect();
    let minus: Vec<C64> = chi.iter().zip(&bchi).map(|(a, b)| (*a - *b).scale(0.5)).collect();
    let (chi_b, sign_b) = if plus.iter().map(|z| z.norm_sqr()).sum::<f64>() > 0.1 { (plus, 1.0) } else { (minus, -1.0) };
    let nrm = chi_b.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    let chi_b: Vec<C64> = chi_b.iter().map(|z| z.scale(1.0 / nrm)).collect();
    // the operators applied to a spinor function of x (D4 entries; only the x^1 derivatives are used)
    let apply = |psi: &Vec<D4>, x: f64, mass: f64| -> Vec<D4> {
        // returns H psi as D4 (values and first derivatives valid)
        let delta = {
            let xs = D4::coord(&[0.0, x, 0.0, 0.0], 1).scale(c(1.0 / ell, 0.0));
            let (ep, em) = (xs.exp(), (-xs).exp());
            (ep - em).div(ep + em).scale(c(dinf, 0.0))
        };
        let dpsi: Vec<D4> = psi
            .iter()
            .map(|z| {
                let mut r = D4::cst(z.d[1]);
                for q in 0..4 {
                    r.d[q] = z.dd[1][q];
                }
                r
            })
            .collect();
        let kin = dmatvec(&DMat::from_cmat(&am.scale(c(0.0, -1.0))), &dpsi);
        let pair = dmatvec(&DMat::from_cmat(&pm), psi);
        let mass_t = dmatvec(&DMat::from_cmat(&bm.scale_r(mass)), psi);
        (0..16).map(|k| kin[k] + pair[k] * delta + mass_t[k]).collect()
    };
    let sech_pow = |x: D4, p: i32| -> D4 {
        let (ep, em) = (x.exp(), (-x).exp());
        let sech = (ep + em).scale(c(0.5, 0.0)).inv();
        let mut r = D4::re(1.0);
        for _ in 0..p {
            r = r * sech;
        }
        r
    };
    // AS4-3: the n = 0 states: Psi = chi_b sech^s(x/ell) solves H Psi = sign_b m Psi
    let mut dev0: f64 = 0.0;
    let mut dev1: f64 = 0.0;
    for i in 0..61 {
        let x = -12.0 + 0.4 * i as f64;
        let xs = D4::coord(&[0.0, x, 0.0, 0.0], 1).scale(c(1.0 / ell, 0.0));
        let phi0 = sech_pow(xs, s as i32);
        let psi: Vec<D4> = chi_b.iter().map(|z| phi0 * D4::cst(*z)).collect();
        let hp = apply(&psi, x, m);
        for k in 0..16 {
            dev0 = dev0.max((hp[k].v - psi[k].v.scale(sign_b * m)).abs());
        }
        // AS4-4: the n = 1 state: u = chi phi1, phi1 = sech^(s-1) tanh (chi in the Gamma = -1 eigenspace);
        // Psi0 = H0 u + E1 u is an eigenvector of H0 (eigenvalue E1); on (Psi0, Bm Psi0) H acts as ((E1, m), (m, -E1)):
        // the eigenvector a Psi0 + b Bm Psi0 with b = (E - E1) a / m has the eigenvalue E = (E1^2 + m^2)^(1/2)
        let (ep, em) = (xs.exp(), (-xs).exp());
        let tanh = (ep - em).div(ep + em);
        let phi1 = sech_pow(xs, s as i32 - 1) * tanh;
        let u: Vec<D4> = chi.iter().map(|z| phi1 * D4::cst(*z)).collect();
        let e1 = e1sq.sqrt();
        let h0u = apply(&u, x, 0.0);
        let psi0: Vec<D4> = (0..16).map(|k| h0u[k] + u[k].scale(c(e1, 0.0))).collect();
        let bpsi0 = dmatvec(&DMat::from_cmat(&bm), &psi0);
        let ee = (e1sq + m * m).sqrt();
        let bcoef = (ee - e1) / m;
        let full: Vec<D4> = (0..16).map(|k| psi0[k] + bpsi0[k].scale(c(bcoef, 0.0))).collect();
        let hfull = apply(&full, x, m);
        for k in 0..16 {
            dev1 = dev1.max((hfull[k].v - full[k].v.scale(ee)).abs());
        }
    }
    rep.add("AS4-3", "kink Delta_inf tanh(x/ell) (Delta_inf = 0.5, ell = 4): the analytic bound state sech^2(x/ell) chi has the energy +-m exactly (61 points)", dev0, 1e-14);
    rep.add("AS4-4", "kink: the analytic n = 1 bound state built from sech(x/ell) tanh(x/ell) has the energy (m^2 + 2 Delta_inf/ell - 1/ell^2)^(1/2) exactly (61 points)", dev1, 1e-13);
    // AS4-5: Wilson lattice of the reduced two-component problem h = sigma1 (-i D_c) + sigma2 Delta(x) + sigma3 (m + W)
    // (the 16 components carry eight copies of the two irreducible representations of the three anticommuting units
    // A, P, Bm): its bound states below the continuum threshold (m^2 + Delta_inf^2)^(1/2) are +-m (one sign per
    // representation) and +-(m^2 + E1^2)^(1/2)
    let thr = (m * m + dinf * dinf).sqrt();
    let mut rows = vec![];
    let mut errs = vec![];
    for &(n, a) in &[(600usize, 0.1f64), (1200usize, 0.05f64), (2400usize, 0.025f64)] {
        let x0 = -0.5 * a * (n as f64 - 1.0);
        let mut diag = vec![];
        let mut upper = vec![];
        for st in 0..n {
            let x = x0 + a * st as f64;
            let dl = dinf * (x / ell).tanh();
            diag.push(sigma(2).scale_r(dl).add(&sigma(3).scale_r(m + 1.0 / a)));
            if st + 1 < n {
                upper.push(sigma(1).scale(c(0.0, -0.5 / a)).sub(&sigma(3).scale_r(0.5 / a)));
            }
        }
        let lat = BlockTridiag { b: 2, diag, upper };
        let ev = lat.eigenvalues_in(-thr + 1e-3, thr - 1e-3, 1e-12);
        // the state near |E| = m (one sign) and the pair of the n = 1 state (both signs; the Wilson term makes their
        // moduli differ at order a, so their mean is used)
        let em = ev.iter().map(|x| x.abs()).fold(f64::INFINITY, |acc, x| if (x - m).abs() < (acc - m).abs() { x } else { acc });
        let want1 = (m * m + e1sq).sqrt();
        let pair: Vec<f64> = ev.iter().map(|x| x.abs()).filter(|x| (x - want1).abs() < 0.05).collect();
        let e1 = if pair.is_empty() { f64::NAN } else { pair.iter().sum::<f64>() / pair.len() as f64 };
        let err = ((em - m) / m).abs().max(((e1 - want1) / want1).abs());
        errs.push((err, ev.clone(), em, e1, pair.len()));
        for (k, b) in ev.iter().enumerate() {
            rows.push(vec![format!("{}", n), f(a), format!("{}", k), f(*b)]);
        }
    }
    let want1 = (m * m + e1sq).sqrt();
    let ext_m = 2.0 * errs[2].2 - errs[1].2;
    let ext_1 = 2.0 * errs[2].3 - errs[1].3;
    let dext = ((ext_m - m) / m).abs().max(((ext_1 - want1) / want1).abs());
    rep.add_bool(
        "AS4-5a",
        "Wilson lattice of the reduced kink problem: below the continuum there are exactly three bound states, one at -m or +m and the pair +-(m^2+E1^2)^(1/2), for each of the three spacings",
        errs.iter().all(|e| e.1.len() == 3 && e.4 == 2),
        &format!("numbers of eigenvalues {}, {}, {}", errs[0].1.len(), errs[1].1.len(), errs[2].1.len()),
    );
    rep.add("AS4-5b", "the Richardson extrapolation 2 E(0.025) - E(0.05) of the lattice bound states agrees with m and (m^2+E1^2)^(1/2) (largest relative deviation; tolerance of order a^2)", dext, 1e-3);
    vals.push(("AS4_err_a0.1".into(), errs[0].0));
    vals.push(("AS4_err_a0.05".into(), errs[1].0));
    vals.push(("AS4_err_a0.025".into(), errs[2].0));
    vals.push(("AS4_err_extrap".into(), dext));
    rep.note(&format!("AS4 raw lattice deviation at the spacing 0.025: {:.3e} (first order in a, as expected for Wilson fermions)", errs[2].0));
    rep.add_bool(
        "AS4-6",
        "the lattice deviation of the bound states decreases each time the spacing is halved",
        errs[1].0 < 0.75 * errs[0].0 && errs[2].0 < 0.75 * errs[1].0,
        &format!("{:.3e}, {:.3e}, {:.3e}", errs[0].0, errs[1].0, errs[2].0),
    );
    rep.note(&format!(
        "AS4 lattice eigenvalues below the continuum threshold {:.10}: a = 0.1: {:?}; a = 0.05: {:?}; a = 0.025: {:?}; analytic |E|: {:.10}, {:.10}",
        thr, errs[0].1, errs[1].1, errs[2].1, m, (m * m + e1sq).sqrt()
    ));
    write_csv(&data.join("part1_s4_kink_lattice.csv"), &["sites", "spacing", "k", "energy"], &rows);
    // the profiles of the bound states for a figure
    let mut rows = vec![];
    for i in 0..241 {
        let x = -12.0 + 0.1 * i as f64;
        let t = (x / ell).tanh();
        let sh = 1.0 / (x / ell).cosh();
        rows.push(vec![f(x), f(dinf * t), f(sh.powf(s)), f(sh.powf(s - 1.0) * t)]);
    }
    write_csv(&data.join("part1_s4_kink_profiles.csv"), &["x", "Delta", "phi0", "phi1"], &rows);
}
