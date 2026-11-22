//! lg_schw -- stage 5 of task C23 (folder Revision_Lord_Gravity, PROJECT spacetimes): the brochon in the Schwarzschild
//! field.  The brochon is a Dirac particle of mass m (the mass of its quasi-particle in the condensed vacuum or the mass
//! m_LL of the normal vacuum, decision D9 of the task record; the results depend on the mass only through G_N M m).
//!
//! Units: G_N = c = hbar = 1 and the mass of the black hole M_LG = 1, so that the radius is measured in units of
//! G_N M_LG (the horizon is at r = 2), and the mass m and the frequency omega in units of 1/(G_N M_LG); "Mm" below is
//! the number G_N M_LG m.
//!
//! The geometry and the Dirac operator are those of Part III of lord_gravity.md (module lgcore::curved): the coframe
//! e^a_mu, the Dirac matrices gamma^mu = e_a^mu gamma^a_LL of the chiral basis, the spinor connection
//! Omega_mu = (1/4) omega_{mu ab} gamma^{ab}_LL from the tetrad postulate, and the field equation
//! i gamma^mu (d_mu + Omega_mu) psi - m psi = 0.  Two tetrads:
//!   * static (Schwarzschild coordinates t, r, theta, phi): e^0 = f^(1/2) dt, e^1 = f^(-1/2) dr, e^2 = r dtheta,
//!     e^3 = r sin(theta) dphi, with f = 1 - 2/r;
//!   * freely falling (Painleve-Gullstrand coordinates tau, r, theta, phi): e^0 = dtau, e^1 = dr + W dtau, e^2 = r dtheta,
//!     e^3 = r sin(theta) dphi, with W = (2/r)^(1/2); it is regular at r = 2 and is used on both sides of the horizon.
//!
//! THE SEPARATION (found with a computer-algebra derivation and verified here at random points by the Dirac operator of
//! lgcore, checks LS-1 and LS-2).  With the constant spinor rotation S_H = diag(H, H), H = i (sigma_1 + sigma_3)/2^(1/2)
//! (it makes the radial Pauli matrix diagonal), the angular functions A(theta), B(theta) that solve
//!     A' = (m_j / sin theta) A - lambda B,      B' = -(m_j / sin theta) B + lambda A                              (A)
//! (regular at both poles for lambda = +-(j + 1/2), j = 1/2, 3/2, ..., |m_j| <= j), and four radial functions with
//!     a_L = -i b_R,   b_L = i a_R,
//! the spinor
//!     psi = pref(r, theta) e^{-i omega T + i m_j phi} S_H (a_L A, b_L B, a_R A, b_R B)
//! solves the Dirac equation if
//!   * static tetrad (T = t, pref = f^(-1/4) r^(-1) sin(theta)^(-1/2)), a_R = (u - i w)/2, b_R = (u + i w)/2:
//!         f u' = -m f^(1/2) u + (omega + lambda f^(1/2)/r) w,
//!         f w' = -(omega - lambda f^(1/2)/r) u + m f^(1/2) w;                                                     (S)
//!   * freely falling tetrad (T = tau, pref = r^(-1) sin(theta)^(-1/2)), a_R = P, b_R = Q:
//!         P' = [ (i omega - W/(4r)) P - (i lambda/r + m) Q ] / (1 - W),
//!         Q' = [ (i lambda/r - m) P + (-i omega + W/(4r)) Q ] / (1 + W).                                          (PG)
//! The two are the same field (check LS-3): P = (1 - W)^(-1/2) a_R e^{i omega (tau - t)}, Q = (1 + W)^(-1/2) b_R
//! e^{i omega (tau - t)}, with d(tau - t)/dr = W/f.
//!
//! THE HORIZON.  (PG) has a regular singular point at r = 2 (W = 1) in its first equation only.  Its exponents there are
//! 0 (a solution analytic at the horizon: the wave that falls in) and 4 i omega - 1/2 (P ~ (r - 2)^(4 i omega - 1/2):
//! the wave that comes out, singular there).  The analytic solution is a power series in x = r - 2 with the radius of
//! convergence 2 (the distance to r = 0), the same function on both sides of the horizon (checks LS-4, LS-5).  Near
//! r = 0 the solutions behave like r^(1/4) (both P and Q; psi ~ r^(-3/4)), with corrections of the relative order
//! r^(1/2) (check LS-6).  For real omega the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 is constant on both sides
//! of the horizon (check LS-7; outside it is |b_R|^2 - |a_R|^2).
//!
//! QUASI-BOUND STATES (check LS-8): complex omega for which the solution that falls into the horizon decays at
//! infinity.  For a trial omega, the solution that falls in (the horizon series at r = 2.5, integrated outward with
//! CVODE) and the solution that decays at infinity (the decaying local eigenvector of (PG) at r_max = 45/k, k =
//! Re (m^2 - omega^2)^(1/2), integrated inward) are compared at r_match = n^2/(Mm)^2 by their normalized Wronskian (the
//! determinant of the two solution vectors divided by their lengths), which vanishes when they are the same solution;
//! a complex secant iteration finds its zeros, and each state is followed in Mm from an extrapolated start (the
//! comments of LS-8 in main).  For small Mm the roots approach the levels m (1 - (Mm)^2/(2 n^2)) of a hydrogen-like
//! atom, with Im omega < 0 (absorption by the black hole).
//!
//! SCATTERING (check LS-9): for real omega > m the greybody factor Gamma (the fraction of an incoming partial wave that
//! the black hole absorbs) from the flux of the solution that falls in, decomposed at a large radius into the local
//! outgoing and incoming waves of (S).
//!
//! Every integration uses CVODE (SUNDIALS 7.8.0, pure Rust; crate lgsundials), Adams formulas, complex error weights.
//!
//! Output (folder data/): hck_lg_schw_checks.txt (the report), lg_schw_qbs.csv (the quasi-bound states), lg_schw_grey.csv
//! (the greybody factors), lg_schw_interior.csv (a solution on both sides of the horizon, for a figure) and
//! lg_schw_values.csv (single values).
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_schw
use lgcore::c64::{c, C64};
use lgcore::cmat::CMat;
use lgcore::curved::{dirac_resid, vmax0, Gauge, Geom, M4};
use lgcore::dirac::sigma;
use lgcore::dual::D4;
use lgcore::report::{data_dir, f, write_csv, Report};
use lgcore::rng::Rng;
use lgsundials::cvode::{integrate, CvodeSettings, Weights};

const I: C64 = C64::I;

fn cr(x: f64) -> C64 {
    c(x, 0.0)
}

// ================================================================================================ the spinor ansatz

/// the constant spinor rotation S_H = diag(H, H), H = i (sigma_1 + sigma_3) / 2^(1/2)
fn s_h() -> CMat {
    let h = sigma(1).add(&sigma(3)).scale(c(0.0, 1.0 / 2f64.sqrt()));
    CMat::block_diag(&h, &h)
}

/// a function of one coordinate as a dual number at the point: its value and its first derivative along the coordinate
/// `p` (the Dirac residual needs only the value and the first derivatives of the spinor)
fn d1(v: C64, p: usize, dv: C64) -> D4 {
    let mut x = D4::cst(v);
    x.d[p] = dv;
    x
}

/// the eight components of the brochon (isospin up) from the four Dirac components psi4 (component 2 d + s, s = 0)
fn to8(psi4: &[D4]) -> Vec<D4> {
    let mut v = vec![D4::ZERO; 8];
    for d in 0..4 {
        v[2 * d] = psi4[d];
    }
    v
}

/// S_H (a_L A, b_L B, a_R A, b_R B) times the prefactor and the phase
fn assemble(a_r: D4, b_r: D4, aa: D4, bb: D4, prefactor: D4) -> Vec<D4> {
    let a_l = b_r.scale(-I);
    let b_l = a_r.scale(I);
    let chi = [a_l * aa, b_l * bb, a_r * aa, b_r * bb];
    let sh = s_h();
    (0..4)
        .map(|i| (0..4).fold(D4::ZERO, |s, j| s + chi[j].scale(sh.get(i, j))) * prefactor)
        .collect()
}

/// the angular functions at theta as dual numbers: values a0, b0 and the derivatives of (A)
fn angular(th: f64, a0: C64, b0: C64, lam: f64, mj: f64) -> (D4, D4) {
    let s = th.sin();
    let da = a0 * (mj / s) - b0 * lam;
    let db = b0 * (-mj / s) + a0 * lam;
    (d1(a0, 2, da), d1(b0, 2, db))
}

/// the right side of (S): (u', w') at r
fn rhs_static(r: f64, u: C64, w: C64, om: C64, m: f64, lam: f64) -> (C64, C64) {
    let fr = 1.0 - 2.0 / r;
    let sf = fr.sqrt();
    let du = (u * (-m * sf) + w * (om + cr(lam * sf / r))) * (1.0 / fr);
    let dw = (u * (-(om - cr(lam * sf / r))) + w * (m * sf)) * (1.0 / fr);
    (du, dw)
}

/// the right side of (PG): (P', Q') at r (r != 2)
fn rhs_pg(r: f64, p: C64, q: C64, om: C64, m: f64, lam: f64) -> (C64, C64) {
    let w = (2.0 / r).sqrt();
    let dp = (p * (I * om - cr(w / (4.0 * r))) - q * (I * (lam / r) + cr(m))) * (1.0 / (1.0 - w));
    let dq = (p * (I * (lam / r) - cr(m)) + q * (-(I * om) + cr(w / (4.0 * r)))) * (1.0 / (1.0 + w));
    (dp, dq)
}

/// the matrix A(r) of (PG) written as (P, Q)' = A (P, Q)
fn mat_pg(r: f64, om: C64, m: f64, lam: f64) -> [[C64; 2]; 2] {
    let w = (2.0 / r).sqrt();
    let a = 1.0 / (1.0 - w);
    let b = 1.0 / (1.0 + w);
    [
        [(I * om - cr(w / (4.0 * r))) * a, -(I * (lam / r) + cr(m)) * a],
        [(I * (lam / r) - cr(m)) * b, (-(I * om) + cr(w / (4.0 * r))) * b],
    ]
}

/// the static coframe at the point x = (t, r, theta, phi), with exact second derivatives
fn coframe_static(x: &[f64; 4]) -> M4 {
    let r = D4::coord(x, 1);
    let th = D4::coord(x, 2);
    let fr = D4::re(1.0) - D4::re(2.0) * r.inv();
    let mut co = [[D4::ZERO; 4]; 4];
    co[0][0] = fr.sqrt();
    co[1][1] = fr.sqrt().inv();
    co[2][2] = r;
    co[3][3] = r * th.sin();
    co
}

/// the freely falling coframe at the point x = (tau, r, theta, phi)
fn coframe_pg(x: &[f64; 4]) -> M4 {
    let r = D4::coord(x, 1);
    let th = D4::coord(x, 2);
    let w = (D4::re(2.0) * r.inv()).sqrt();
    let mut co = [[D4::ZERO; 4]; 4];
    co[0][0] = D4::re(1.0);
    co[1][0] = w;
    co[1][1] = D4::re(1.0);
    co[2][2] = r;
    co[3][3] = r * th.sin();
    co
}

/// the Dirac residual of the separated ansatz in the static tetrad at the point x, for the radial values (u0, w0) and
/// the angular values (a0, b0), relative to the size of the spinor
fn resid_static(x: &[f64; 4], om: C64, m: f64, lam: f64, mj: f64, u0: C64, w0: C64, a0: C64, b0: C64) -> f64 {
    let geo = Geom::new(coframe_static(x));
    let t = D4::coord(x, 0);
    let r = D4::coord(x, 1);
    let th = D4::coord(x, 2);
    let ph = D4::coord(x, 3);
    let fr = D4::re(1.0) - D4::re(2.0) * r.inv();
    let pref = fr.sqrt().sqrt().inv() * r.inv() * th.sin().sqrt().inv();
    let phase = (t.scale(c(om.im, -om.re)) + ph.scale(c(0.0, mj))).exp();
    let (du, dw) = rhs_static(x[1], u0, w0, om, m, lam);
    let u = d1(u0, 1, du);
    let w = d1(w0, 1, dw);
    let a_r = (u - w.scale(I)).scale(cr(0.5));
    let b_r = (u + w.scale(I)).scale(cr(0.5));
    let (aa, bb) = angular(x[2], a0, b0, lam, mj);
    let psi = to8(&assemble(a_r, b_r, aa, bb, pref * phase));
    let res = dirac_resid(&geo, &Gauge::zero(0.0, 0.0), &psi, m);
    vmax0(&res) / vmax0(&psi).max(1e-300)
}

/// the same for the freely falling tetrad with the radial values (P0, Q0)
fn resid_pg(x: &[f64; 4], om: C64, m: f64, lam: f64, mj: f64, p0: C64, q0: C64, a0: C64, b0: C64) -> f64 {
    let geo = Geom::new(coframe_pg(x));
    let t = D4::coord(x, 0);
    let r = D4::coord(x, 1);
    let th = D4::coord(x, 2);
    let ph = D4::coord(x, 3);
    let pref = r.inv() * th.sin().sqrt().inv();
    let phase = (t.scale(c(om.im, -om.re)) + ph.scale(c(0.0, mj))).exp();
    let (dp, dq) = rhs_pg(x[1], p0, q0, om, m, lam);
    let (aa, bb) = angular(x[2], a0, b0, lam, mj);
    let psi = to8(&assemble(d1(p0, 1, dp), d1(q0, 1, dq), aa, bb, pref * phase));
    let res = dirac_resid(&geo, &Gauge::zero(0.0, 0.0), &psi, m);
    vmax0(&res) / vmax0(&psi).max(1e-300)
}

// ================================================================================================ the horizon series

/// The power series of the solution of (PG) that is analytic at the horizon, in x = r - 2: P = sum p_k x^k,
/// Q = sum q_k x^k with q_0 = 1 (k = 0..n).  With W(x) = (1 + x/2)^(-1/2) = sum w_j x^j, (PG) reads
///     a(x) P' = b(x) P - c(x) Q,    d(x) Q' = e(x) P + g(x) Q,
/// a = 4 r (1 - W), b = 4 i omega r - W, c = 4 i lambda + 4 m r, d = 4 r (1 + W), e = 4 i lambda - 4 m r, g = W - 4 i
/// omega r, r = 2 + x; a(0) = 0 and a'(0) = 2, so the order k of the first equation gives
///     p_k (2 k - b_0) = sum_{j>=1} b_j p_{k-j} - sum_j c_j q_{k-j} - sum_{j>=2} a_j (k - j + 1) p_{k-j+1},
/// with b_0 = 8 i omega - 1, and the order k of the second q_{k+1} (d_0 = 16).
fn horizon_series(om: C64, m: f64, lam: f64, n: usize) -> (Vec<C64>, Vec<C64>) {
    let nn = n + 2;
    // w_j: the binomial series of (1 + x/2)^(-1/2)
    let mut wj = vec![cr(0.0); nn + 1];
    wj[0] = cr(1.0);
    for j in 1..=nn {
        wj[j] = wj[j - 1] * (-(2.0 * j as f64 - 1.0) / (2.0 * j as f64) / 2.0);
    }
    let rj = |j: usize| -> f64 {
        match j {
            0 => 2.0,
            1 => 1.0,
            _ => 0.0,
        }
    };
    // s = 1 - W, t = 1 + W
    let s: Vec<C64> = (0..=nn).map(|j| if j == 0 { cr(1.0) - wj[0] } else { -wj[j] }).collect();
    let tt: Vec<C64> = (0..=nn).map(|j| if j == 0 { cr(1.0) + wj[0] } else { wj[j] }).collect();
    let a: Vec<C64> = (0..=nn).map(|j| (s[j] * 2.0 + if j > 0 { s[j - 1] } else { cr(0.0) }) * 4.0).collect();
    let d: Vec<C64> = (0..=nn).map(|j| (tt[j] * 2.0 + if j > 0 { tt[j - 1] } else { cr(0.0) }) * 4.0).collect();
    let b: Vec<C64> = (0..=nn).map(|j| I * om * (4.0 * rj(j)) - wj[j]).collect();
    let cc: Vec<C64> = (0..=nn).map(|j| I * (4.0 * lam) * if j == 0 { 1.0 } else { 0.0 } + cr(4.0 * m * rj(j))).collect();
    let e: Vec<C64> = (0..=nn).map(|j| I * (4.0 * lam) * if j == 0 { 1.0 } else { 0.0 } - cr(4.0 * m * rj(j))).collect();
    let g: Vec<C64> = (0..=nn).map(|j| wj[j] - I * om * (4.0 * rj(j))).collect();
    let mut p = vec![cr(0.0); n + 1];
    let mut q = vec![cr(0.0); n + 2];
    q[0] = cr(1.0);
    for k in 0..=n {
        let mut rhs = cr(0.0);
        for j in 1..=k {
            rhs += b[j] * p[k - j];
        }
        for j in 0..=k {
            rhs -= cc[j] * q[k - j];
        }
        for j in 2..=k {
            rhs -= a[j] * ((k - j + 1) as f64) * p[k - j + 1];
        }
        p[k] = rhs / (a[1] * (k as f64) - b[0]);
        // q_{k+1}
        let mut rq = cr(0.0);
        for j in 0..=k {
            rq += e[j] * p[k - j] + g[j] * q[k - j];
        }
        for j in 1..=k {
            rq -= d[j] * ((k - j + 1) as f64) * q[k - j + 1];
        }
        q[k + 1] = rq / (d[0] * ((k + 1) as f64));
    }
    q.truncate(n + 1);
    (p, q)
}

/// the value and the derivative of a power series at x
fn series_eval(cf: &[C64], x: f64) -> (C64, C64) {
    let (mut v, mut dv) = (cr(0.0), cr(0.0));
    for k in (0..cf.len()).rev() {
        v = v * x + cf[k];
        if k > 0 {
            dv = dv * x + cf[k] * (k as f64);
        }
    }
    (v, dv)
}

// ================================================================================================ CVODE integrations

fn settings(rtol: f64) -> CvodeSettings {
    CvodeSettings::adams(rtol, 1e-300_f64.max(rtol * 1e-14), Weights::Complex)
}

/// integrates (PG) outward from r0 (values p0, q0) and returns (P, Q) at the radii `rs` (increasing, >= r0, all > 2)
fn pg_outward(r0: f64, p0: C64, q0: C64, rs: &[f64], om: C64, m: f64, lam: f64, rtol: f64) -> Vec<(C64, C64)> {
    let y0 = vec![p0.re, p0.im, q0.re, q0.im];
    let rhs = Box::new(move |r: f64, y: &[f64], dy: &mut [f64]| -> i32 {
        let (dp, dq) = rhs_pg(r, c(y[0], y[1]), c(y[2], y[3]), om, m, lam);
        dy[0] = dp.re;
        dy[1] = dp.im;
        dy[2] = dq.re;
        dy[3] = dq.im;
        0
    });
    let mut out = vec![];
    integrate(&y0, r0, rs, &settings(rtol), rhs, |_, _, y| {
        out.push((c(y[0], y[1]), c(y[2], y[3])));
        true
    })
    .expect("CVODE outward");
    out
}

/// integrates (PG) inward from r0 to the radii `rs` (decreasing, <= r0), in the variable sigma = -ln r, so that
/// d/dsigma = -r d/dr and sigma increases; returns (P, Q) at `rs`
fn pg_inward(r0: f64, p0: C64, q0: C64, rs: &[f64], om: C64, m: f64, lam: f64, rtol: f64) -> Vec<(C64, C64)> {
    let y0 = vec![p0.re, p0.im, q0.re, q0.im];
    let rhs = Box::new(move |s: f64, y: &[f64], dy: &mut [f64]| -> i32 {
        let r = (-s).exp();
        let (dp, dq) = rhs_pg(r, c(y[0], y[1]), c(y[2], y[3]), om, m, lam);
        dy[0] = -r * dp.re;
        dy[1] = -r * dp.im;
        dy[2] = -r * dq.re;
        dy[3] = -r * dq.im;
        0
    });
    let ss: Vec<f64> = rs.iter().map(|r| -r.ln()).collect();
    let mut out = vec![];
    integrate(&y0, -r0.ln(), &ss, &settings(rtol), rhs, |_, _, y| {
        out.push((c(y[0], y[1]), c(y[2], y[3])));
        true
    })
    .expect("CVODE inward");
    out
}

/// integrates (PG) inward from r0 to r1 < r0 in the variable s = -r (s increases); returns (P, Q) at r1
fn pg_inward_lin(r0: f64, p0: C64, q0: C64, r1: f64, om: C64, m: f64, lam: f64, rtol: f64) -> (C64, C64) {
    let y0 = vec![p0.re, p0.im, q0.re, q0.im];
    let rhs = Box::new(move |s: f64, y: &[f64], dy: &mut [f64]| -> i32 {
        let (dp, dq) = rhs_pg(-s, c(y[0], y[1]), c(y[2], y[3]), om, m, lam);
        dy[0] = -dp.re;
        dy[1] = -dp.im;
        dy[2] = -dq.re;
        dy[3] = -dq.im;
        0
    });
    let mut out = (cr(0.0), cr(0.0));
    integrate(&y0, -r0, &[-r1], &settings(rtol), rhs, |_, _, y| {
        out = (c(y[0], y[1]), c(y[2], y[3]));
        true
    })
    .expect("CVODE inward (linear)");
    out
}

/// the eigenvalues and eigenvectors of a complex 2x2 matrix, ordered by the real part of the eigenvalue (ascending)
fn eig2(a: &[[C64; 2]; 2]) -> [(C64, [C64; 2]); 2] {
    let tr = a[0][0] + a[1][1];
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    let disc = (tr * tr - det * 4.0).sqrt();
    let l1 = (tr - disc) * 0.5;
    let l2 = (tr + disc) * 0.5;
    let vec = |l: C64| -> [C64; 2] {
        // (a00 - l) v0 + a01 v1 = 0
        if a[0][1].abs() > 1e-300 {
            [a[0][1], l - a[0][0]]
        } else {
            [l - a[1][1], a[1][0]]
        }
    };
    let mut e = [(l1, vec(l1)), (l2, vec(l2))];
    if e[0].0.re > e[1].0.re {
        e.swap(0, 1);
    }
    e
}

/// the matching function of the quasi-bound states: the solution that is analytic at the horizon (the series at
/// r = 2.5, then CVODE outward to rmatch) and the solution that decays at infinity (the decaying local eigenvector of
/// (PG) at rmax, integrated inward to rmatch, where it grows, so that the integration is stable) are linearly dependent
/// at a quasi-bound state; returns their normalized determinant (P_h Q_inf - Q_h P_inf)/(|h| |inf|)
fn shoot(om: C64, m: f64, lam: f64, rmatch: f64, rmax: f64) -> C64 {
    let (pc, qc) = horizon_series(om, m, lam, 90);
    let x0 = 0.5;
    let (p0, _) = series_eval(&pc, x0);
    let (q0, _) = series_eval(&qc, x0);
    let (ph, qh) = pg_outward(2.0 + x0, p0, q0, &[rmatch], om, m, lam, 1e-13)[0];
    let e = eig2(&mat_pg(rmax, om, m, lam));
    let vd = e[0].1;
    let (pi, qi) = pg_inward_lin(rmax, vd[0], vd[1], rmatch, om, m, lam, 1e-13);
    let nh = (ph.norm_sqr() + qh.norm_sqr()).sqrt();
    let ni = (pi.norm_sqr() + qi.norm_sqr()).sqrt();
    (ph * qi - qh * pi) * (1.0 / (nh * ni))
}

/// the complex secant iteration for a zero of `shoot` from the two starts w0, w1
fn secant(m: f64, lam: f64, rmatch: f64, rmax: f64, w0: C64, w1: C64) -> (C64, usize, f64) {
    let (mut a, mut b) = (w0, w1);
    let mut fa = shoot(a, m, lam, rmatch, rmax);
    let mut fb = shoot(b, m, lam, rmatch, rmax);
    let mut it = 0;
    while it < 60 {
        let den = fb - fa;
        if den.abs() == 0.0 {
            break;
        }
        let nw = b - fb * (b - a) / den;
        a = b;
        fa = fb;
        b = nw;
        fb = shoot(b, m, lam, rmatch, rmax);
        it += 1;
        if (b - a).abs() < 1e-14 * b.abs() {
            break;
        }
    }
    (b, it, fb.abs())
}

/// the greybody factor of the partial wave (lambda) at the real frequency omega > m: the solution that falls in (the
/// horizon series, normalized so that q_0 = 1), integrated to rmax, converted to the static functions (a_R, b_R) up to a
/// common phase, and decomposed into the local outgoing and incoming eigenvectors of (S); returns (Gamma, |R|^2, the
/// flux at the horizon, the flux at rmax)
fn greybody(om: f64, m: f64, lam: f64, rmax: f64) -> (f64, f64, f64, f64) {
    let omc = cr(om);
    let (pc, qc) = horizon_series(omc, m, lam, 90);
    let x0 = 0.5;
    let (p0, _) = series_eval(&pc, x0);
    let (q0, _) = series_eval(&qc, x0);
    let out = pg_outward(2.0 + x0, p0, q0, &[rmax], omc, m, lam, 1e-13);
    let (p, q) = out[0];
    let w = (2.0 / rmax).sqrt();
    let a_r = p * (1.0 - w).sqrt();
    let b_r = q * (1.0 + w).sqrt();
    let u = a_r + b_r;
    let wv = (a_r - b_r) * I;
    // the static matrix in r*: (u, w)^* = [[-m s, om + lam s/r], [-(om - lam s/r), m s]] (u, w), s = f^(1/2)
    let fr = 1.0 - 2.0 / rmax;
    let s = fr.sqrt();
    let aa = m * s;
    let bb = om + lam * s / rmax;
    let ccc = om - lam * s / rmax;
    let k = (bb * ccc - aa * aa).sqrt();
    // eigenvectors (b, a +- i k) for the eigenvalues +- i k (outgoing: + i k)
    let vout = [cr(bb), c(aa, k)];
    let vin = [cr(bb), c(aa, -k)];
    let det = vout[0] * vin[1] - vout[1] * vin[0];
    let al = (u * vin[1] - wv * vin[0]) / det;
    let be = (vout[0] * wv - vout[1] * u) / det;
    let j_h = 2.0; // (1 + W)|Q|^2 - (1 - W)|P|^2 at the horizon (W = 1, q_0 = 1): 2 |q_0|^2 = 2
    let j_far = (1.0 + w) * q.norm_sqr() - (1.0 - w) * p.norm_sqr();
    let gamma = j_h / (bb * k * be.norm_sqr());
    let refl = al.norm_sqr() / be.norm_sqr();
    (gamma, refl, j_h, j_far)
}

// ================================================================================================ main

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "lg_schw: the brochon in the Schwarzschild field (stage 5 of task C23, Revision_Lord_Gravity, PROJECT spacetimes); \
         units G_N = c = hbar = 1, mass of the black hole 1 (horizon at r = 2)",
    );
    let mut vals: Vec<Vec<String>> = vec![];
    let mut rng = Rng(20261122);

    // ------------------------------------------------------------------ LS-1, LS-2: the separation
    let lams = [1.0, -1.0, 2.0, -2.0, 3.0, -3.0];
    let mjs = [0.5, -0.5, 1.5, -1.5];
    let mut dev1 = 0.0f64;
    let mut dev2 = 0.0f64;
    let mut dev2in = 0.0f64;
    for k in 0..40 {
        let lam = lams[k % 6];
        let mj = mjs[k % 4];
        let om = c(0.1 + 1.9 * (0.5 + 0.5 * rng.unit()), 0.3 * rng.unit());
        let m = 0.5 + 0.5 * rng.unit();
        let rnd = |rng: &mut Rng| c(rng.unit(), rng.unit());
        let (u0, w0, a0, b0) = (rnd(&mut rng), rnd(&mut rng), rnd(&mut rng), rnd(&mut rng));
        let x = [3.0 * rng.unit(), 2.2 + 17.8 * (0.5 + 0.5 * rng.unit()), 0.2 + 2.7 * (0.5 + 0.5 * rng.unit()), 6.0 * rng.unit()];
        dev1 = dev1.max(resid_static(&x, om, m, lam, mj, u0, w0, a0, b0));
        // PG: outside and inside the horizon (|r - 2| > 0.1)
        let r_out = 2.2 + 17.8 * (0.5 + 0.5 * rng.unit());
        let r_in = 0.3 + 1.6 * (0.5 + 0.5 * rng.unit());
        let xo = [x[0], r_out, x[2], x[3]];
        let xi = [x[0], r_in, x[2], x[3]];
        dev2 = dev2.max(resid_pg(&xo, om, m, lam, mj, u0, w0, a0, b0));
        dev2in = dev2in.max(resid_pg(&xi, om, m, lam, mj, u0, w0, a0, b0));
    }
    rep.add(
        "LS-1",
        "the separation in the static tetrad: the spinor f^(-1/4) r^(-1) sin(theta)^(-1/2) e^(-i omega t + i m_j phi) S_H \
         (a_L A, b_L B, a_R A, b_R B) with (A), (S) and a_L = -i b_R, b_L = i a_R solves the Dirac equation of Part III \
         (module lgcore::curved, the spinor connection from the tetrad postulate) at 40 random points outside the horizon, \
         for lambda = +-1, +-2, +-3, m_j = +-1/2, +-3/2, complex omega and random values of the functions (largest \
         residual relative to the spinor)",
        dev1,
        1e-12,
    );
    rep.add(
        "LS-2",
        "the separation in the freely falling tetrad with (PG): the same at 40 random points outside and 40 inside the \
         horizon (0.3 < r < 1.9); the largest relative residual of both",
        dev2.max(dev2in),
        1e-12,
    );
    vals.push(vec!["ls1_largest_residual".into(), f(dev1)]);
    vals.push(vec!["ls2_largest_residual".into(), f(dev2.max(dev2in))]);

    // ------------------------------------------------------------------ LS-3: the relation of (S) and (PG)
    let mut dev3 = 0.0f64;
    for _ in 0..40 {
        let lam = lams[(rng.next_u64() % 6) as usize];
        let om = c(0.1 + 1.9 * (0.5 + 0.5 * rng.unit()), 0.2 * rng.unit());
        let m = 0.5 + 0.5 * rng.unit();
        let r = 2.2 + 17.8 * (0.5 + 0.5 * rng.unit());
        let u = c(rng.unit(), rng.unit());
        let w = c(rng.unit(), rng.unit());
        // P = (1 - W)^(-1/2) a_R e^{i om T}, Q = (1 + W)^(-1/2) b_R e^{i om T}; the phase is 1 at r (T(r) = 0)
        let wv = (2.0 / r).sqrt();
        let fr = 1.0 - 2.0 / r;
        let dwv = -wv / (2.0 * r);
        let dt = wv / fr;
        let (du, dw) = rhs_static(r, u, w, om, m, lam);
        let a_r = (u - w * I) * 0.5;
        let b_r = (u + w * I) * 0.5;
        let da = (du - dw * I) * 0.5;
        let db = (du + dw * I) * 0.5;
        let p = a_r * (1.0 - wv).powf(-0.5);
        let q = b_r * (1.0 + wv).powf(-0.5);
        let dp = a_r * (0.5 * (1.0 - wv).powf(-1.5) * dwv) + da * (1.0 - wv).powf(-0.5) + p * (I * om * dt);
        let dq = b_r * (-0.5 * (1.0 + wv).powf(-1.5) * dwv) + db * (1.0 + wv).powf(-0.5) + q * (I * om * dt);
        let (ep, eq) = rhs_pg(r, p, q, om, m, lam);
        let sc = dp.abs().max(dq.abs()).max(1e-300);
        dev3 = dev3.max((dp - ep).abs().max((dq - eq).abs()) / sc);
    }
    rep.add(
        "LS-3",
        "the two separations describe the same field: if (u, w) solve (S), then P = (1 - W)^(-1/2) a_R e^{i omega (tau - t)}, \
         Q = (1 + W)^(-1/2) b_R e^{i omega (tau - t)} solve (PG), with d(tau - t)/dr = W/f (40 random cases outside the \
         horizon; largest relative difference of the derivatives)",
        dev3,
        1e-13,
    );

    // ------------------------------------------------------------------ LS-4, LS-5: the horizon
    // the series of the analytic solution satisfies (PG) on both sides of the horizon
    let cases4 = [(c(0.7, 0.0), 0.3, 1.0), (c(1.3, 0.0), 0.6, -1.0), (c(0.4, -0.01), 0.45, 2.0), (c(2.0, 0.0), 0.0, -2.0)];
    let mut dev4 = 0.0f64;
    let mut dev5 = 0.0f64;
    for &(om, m, lam) in &cases4 {
        let (pc, qc) = horizon_series(om, m, lam, 90);
        for &x in &[-0.5, -0.25, 0.25, 0.5] {
            let (p, dp) = series_eval(&pc, x);
            let (q, dq) = series_eval(&qc, x);
            let (ep, eq) = rhs_pg(2.0 + x, p, q, om, m, lam);
            let sc = dp.abs().max(dq.abs()).max(p.abs()).max(q.abs());
            dev4 = dev4.max((dp - ep).abs().max((dq - eq).abs()) / sc);
        }
        // CVODE from the series at x = +0.1 outward to x = +0.5, and from x = -0.1 inward to x = -0.5
        let (p1, _) = series_eval(&pc, 0.1);
        let (q1, _) = series_eval(&qc, 0.1);
        let o = pg_outward(2.1, p1, q1, &[2.5], om, m, lam, 1e-12);
        let (p5, _) = series_eval(&pc, 0.5);
        let (q5, _) = series_eval(&qc, 0.5);
        dev5 = dev5.max((o[0].0 - p5).abs().max((o[0].1 - q5).abs()) / p5.abs().max(q5.abs()));
        let (pm1, _) = series_eval(&pc, -0.1);
        let (qm1, _) = series_eval(&qc, -0.1);
        let i_ = pg_inward(1.9, pm1, qm1, &[1.5], om, m, lam, 1e-12);
        let (pm5, _) = series_eval(&pc, -0.5);
        let (qm5, _) = series_eval(&qc, -0.5);
        dev5 = dev5.max((i_[0].0 - pm5).abs().max((i_[0].1 - qm5).abs()) / pm5.abs().max(qm5.abs()));
    }
    rep.add(
        "LS-4",
        "the horizon: the power series in x = r - 2 (90 terms) of the solution of (PG) that is analytic at r = 2 satisfies \
         (PG) at x = -0.5, -0.25, 0.25, 0.5, on both sides of the horizon, for four cases (real and complex omega, m = 0 \
         to 0.6, lambda = +-1, +-2; largest relative residual)",
        dev4,
        1e-12,
    );
    rep.add(
        "LS-5",
        "the series agrees with CVODE on both sides: from the series at x = 0.1 integrated outward to x = 0.5, and from \
         x = -0.1 inward to x = -0.5, the solution equals the series there (largest relative difference)",
        dev5,
        1e-9,
    );
    // the singular solution: a start that is not the analytic one, integrated toward the horizon from outside, grows like
    // (r - 2)^(-1/2) (the exponent 4 i omega - 1/2 of P; its modulus for real omega)
    let (om_s, m_s, lam_s) = (cr(0.7), 0.3, 1.0);
    let xs_s = [1e-3, 1e-4, 1e-5, 1e-6];
    let y0s = vec![1.0, 0.0, 0.0, 0.0];
    let rhs_s = Box::new(move |s: f64, y: &[f64], dy: &mut [f64]| -> i32 {
        // the variable s = -ln(r - 2) increases toward the horizon: d/ds = -(r - 2) d/dr
        let x = (-s).exp();
        let (dp, dq) = rhs_pg(2.0 + x, c(y[0], y[1]), c(y[2], y[3]), om_s, m_s, lam_s);
        dy[0] = -x * dp.re;
        dy[1] = -x * dp.im;
        dy[2] = -x * dq.re;
        dy[3] = -x * dq.im;
        0
    });
    let ss: Vec<f64> = xs_s.iter().map(|x: &f64| -x.ln()).collect();
    let mut sing = vec![];
    integrate(&y0s, -(1.0f64).ln(), &ss, &settings(1e-12), rhs_s, |_, _, y| {
        sing.push(c(y[0], y[1]).abs());
        true
    })
    .expect("CVODE toward the horizon");
    // the logarithmic slope d ln|P| / d ln x between the last two radii
    let slope = (sing[3] / sing[2]).ln() / (xs_s[3] / xs_s[2]).ln();
    rep.add(
        "LS-4b",
        "the other solution at the horizon: a start that is not analytic, integrated with CVODE toward r = 2 from outside \
         (in the variable -ln(r - 2)), has |P| ~ (r - 2)^(-1/2), the modulus of the exponent 4 i omega - 1/2 (omega = 0.7, \
         m = 0.3, lambda = 1; difference of the logarithmic slope between x = 1e-5 and 1e-6 from -1/2)",
        (slope + 0.5).abs(),
        1e-4,
    );
    vals.push(vec!["ls4b_slope".into(), f(slope)]);

    // ------------------------------------------------------------------ LS-6, LS-7: inside the horizon, and the flux
    let (om6, m6, lam6) = (cr(0.7), 0.3, 1.0);
    let (pc6, qc6) = horizon_series(om6, m6, lam6, 90);
    let (p6, _) = series_eval(&pc6, -0.5);
    let (q6, _) = series_eval(&qc6, -0.5);
    let rs_in: Vec<f64> = (1..=60).map(|k| 1.5 * 10f64.powf(-0.2 * k as f64)).collect();
    let inn = pg_inward(1.5, p6, q6, &rs_in, om6, m6, lam6, 1e-13);
    let rsmall = *rs_in.last().unwrap();
    let (ps, qs) = *inn.last().unwrap();
    let (dps, dqs) = rhs_pg(rsmall, ps, qs, om6, m6, lam6);
    let ep = dps * rsmall / ps;
    let eq = dqs * rsmall / qs;
    let dev6 = (ep - cr(0.25)).abs().max((eq - cr(0.25)).abs());
    rep.add_bool(
        "LS-6",
        "inside the horizon: the solution that falls in, continued from the series at r = 1.5 inward with CVODE to \
         r = 1.5e-12, has the logarithmic derivatives r P'/P and r Q'/Q equal to 1/4 up to corrections of the order \
         (r/2)^(1/2) = 8.7e-7 (so P, Q ~ r^(1/4) and psi ~ r^(-3/4) at the singularity); the difference is at most 3 \
         |lambda| (r/2)^(1/2)",
        dev6 <= 3.0 * lam6.abs() * (rsmall / 2.0).sqrt(),
        &format!("r P'/P - 1/4 = {:.2e}, r Q'/Q - 1/4 = {:.2e}", (ep - cr(0.25)).abs(), (eq - cr(0.25)).abs()),
    );
    // the flux J = (1 + W)|Q|^2 - (1 - W)|P|^2, on both sides
    let jflux = |r: f64, p: C64, q: C64| -> f64 {
        let w = (2.0 / r).sqrt();
        (1.0 + w) * q.norm_sqr() - (1.0 - w) * p.norm_sqr()
    };
    let j_h = 2.0; // at the horizon, q_0 = 1
    let mut dev7 = 0.0f64;
    for (k, &(p, q)) in inn.iter().enumerate() {
        dev7 = dev7.max((jflux(rs_in[k], p, q) - j_h).abs() / j_h);
    }
    let rs_out: Vec<f64> = (1..=40).map(|k| 2.5 * 10f64.powf(0.075 * k as f64)).collect();
    let (p25, _) = series_eval(&pc6, 0.5);
    let (q25, _) = series_eval(&qc6, 0.5);
    let outw = pg_outward(2.5, p25, q25, &rs_out, om6, m6, lam6, 1e-13);
    for (k, &(p, q)) in outw.iter().enumerate() {
        dev7 = dev7.max((jflux(rs_out[k], p, q) - j_h).abs() / j_h);
    }
    rep.add(
        "LS-7",
        "the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 of a real frequency is the same at the horizon (J = 2 for \
         q_0 = 1), at 60 radii inside down to r = 1.5e-12 and at 40 radii outside up to r = 2.5e3 (largest relative \
         difference; CVODE with rtol 1e-13, whose global error over the integration is of the order 1e-8)",
        dev7,
        1e-7,
    );
    // the interior solution for a figure: r, |P|, |Q| on both sides
    let mut int_rows = vec![];
    for (k, &(p, q)) in inn.iter().enumerate().rev() {
        int_rows.push(vec![f(rs_in[k]), f(p.abs()), f(q.abs())]);
    }
    for &x in &[-0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2, 0.3, 0.4] {
        let (p, _) = series_eval(&pc6, x);
        let (q, _) = series_eval(&qc6, x);
        int_rows.push(vec![f(2.0 + x), f(p.abs()), f(q.abs())]);
    }
    for (k, &(p, q)) in outw.iter().enumerate() {
        int_rows.push(vec![f(rs_out[k]), f(p.abs()), f(q.abs())]);
    }

    // ------------------------------------------------------------------ LS-8: quasi-bound states
    // Six states are followed from Mm = 0.05, where the secant iteration starts at the hydrogen-like level n (with a
    // small negative imaginary part), to Mm = 0.4 in steps of 0.05.  Each step starts from the root of the previous one,
    // extrapolated: the difference of Re omega/m from the hydrogen-like level is scaled with (Mm)^4 (the order of the
    // first correction) and Im omega/m with (Mm)^6.  The angular number lambda corresponds to the number kappa of the hydrogen atom by kappa = -lambda (found
    // here: lambda = 1 has the level n = 1, lambda = -1 has none), so the states are named like those of hydrogen.
    // A root is accepted when the iteration has converged (step below 1e-14 of omega, at most 60 iterations) and the
    // normalized determinant of the matching is below 1e-10.  The matching radius is n^2/(Mm)^2 (the radius of the
    // hydrogen-like orbit, in units of G M), the outer radius 45/k with k = Re (m^2 - omega^2)^(1/2) at the start.
    let states: [(f64, usize, &str); 6] =
        [(1.0, 1, "1S1/2"), (1.0, 2, "2S1/2"), (-1.0, 2, "2P1/2"), (2.0, 2, "2P3/2"), (2.0, 3, "3P3/2"), (-2.0, 3, "3D3/2")];
    let mms = [0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4];
    let mut qbs_rows = vec![];
    let mut bad8 = 0usize;
    let mut unresolved = 0usize;
    let mut conv = 0.0f64;
    let mut hyd_dev = 0.0f64;
    let mut lost: Vec<String> = vec![];
    let hyd = |mm: f64, n: usize| 1.0 - mm * mm / (2.0 * (n * n) as f64);
    let mut exps: Vec<String> = vec![];
    for &(lam, n, label) in &states {
        let mut prev: Option<(f64, C64)> = None;
        let mut im_at: Vec<(f64, f64)> = vec![];
        for &mm in &mms {
            let w0 = match prev {
                Some((mp, z)) => {
                    let sc = mm / mp;
                    c(mm * (hyd(mm, n) + (z.re - hyd(mp, n)) * sc.powi(4)), mm * z.im * sc.powi(6))
                }
                None => c(mm * hyd(mm, n), -1e-7 * mm),
            };
            let k0 = (cr(mm * mm) - w0 * w0).sqrt().re.abs().max(1e-6);
            let rmatch = (n * n) as f64 / (mm * mm);
            let rmax = 45.0 / k0;
            let (w, it, res) = secant(mm, lam, rmatch, rmax, w0, w0 * c(1.0 + 1e-5, 0.0));
            if !(it < 60 && res < 1e-10) {
                lost.push(format!("{} at Mm = {}", label, mm));
                qbs_rows.push(vec![label.into(), f(mm), format!("{}", lam), format!("{}", n), "not found".into(), "".into(), "".into(), format!("{}", it), f(res)]);
                println!("  QBS {} Mm = {}: no convergence ({} iterations, residual {:.2e})", label, mm, it, res);
                break;
            }
            let (w2, _, _) = secant(mm, lam, rmatch, rmax * 1.3, w, w * c(1.0 + 1e-9, 0.0));
            let cv = (w2 - w).abs() / w.abs();
            conv = conv.max(cv);
            if !(w.re < mm) {
                bad8 += 1;
            }
            if w.im.abs() < 1e-12 * mm {
                unresolved += 1;
            } else if w.im > 0.0 {
                bad8 += 1;
            }
            if mm < 0.06 {
                hyd_dev = hyd_dev.max((w.re / mm - hyd(mm, n)).abs() / mm.powi(4));
            }
            if w.im.abs() >= 1e-12 * mm {
                im_at.push((mm, (w.im / mm).abs()));
            }
            qbs_rows.push(vec![label.into(), f(mm), format!("{}", lam), format!("{}", n), f(w.re / mm), f(w.im / mm), f(hyd(mm, n)), format!("{}", it), f(res)]);
            println!("  QBS {} Mm = {}: omega/m = {:.12} {:+.6e} i (hydrogen-like {:.10}), {} iterations, change with rmax {:.1e}", label, mm, w.re / mm, w.im / mm, hyd(mm, n), it, cv);
            prev = Some((mm, w * (1.0 / mm)));
        }
        // the local power p of |Im omega|/m ~ (Mm)^p between the two smallest masses of the list at which |Im omega| is
        // resolved (at least 1e-12 m), and at which they are consecutive in the list
        let mut done = false;
        for wpair in im_at.windows(2) {
            let (a, b) = (wpair[0], wpair[1]);
            if (b.0 - a.0 - 0.05).abs() < 1e-9 {
                let pw = (b.1 / a.1).ln() / (b.0 / a.0).ln();
                exps.push(format!("{} {:.2} (Mm = {} to {})", label, pw, a.0, b.0));
                done = true;
                break;
            }
        }
        if !done {
            exps.push(format!("{} unresolved", label));
        }
    }
    rep.add_bool(
        "LS-8a",
        "quasi-bound states (data/lg_schw_qbs.csv): six states followed from Mm = 0.05 to 0.4; every root has Re omega < m, \
         and Im omega < 0 (it decays in time by absorption into the black hole) wherever |Im omega| exceeds the resolution \
         1e-12 m of the computation",
        bad8 == 0 && !qbs_rows.is_empty(),
        &format!("{} roots, {} with |Im omega| below the resolution; lost: {}", qbs_rows.len(), unresolved, if lost.is_empty() { "none".to_string() } else { lost.join(", ") }),
    );
    rep.add(
        "LS-8b",
        "the outer radius: 1.3 times larger changes no quasi-bound frequency by more than 1e-10 (relative)",
        conv,
        1e-10,
    );
    rep.add(
        "LS-8c",
        "the hydrogen-like limit: at Mm = 0.05 every state lies within 10 (Mm)^4 of its level 1 - (Mm)^2/(2 n^2) (the \
         largest difference divided by (Mm)^4)",
        hyd_dev,
        10.0,
    );
    vals.push(vec!["ls8c_largest_dev_over_Mm4".into(), f(hyd_dev)]);
    vals.push(vec!["ls8_local_power_of_Im_omega".into(), exps.join("; ")]);
    println!("  powers of |Im omega|/m: {}", exps.join("; "));

    // ------------------------------------------------------------------ LS-9: greybody factors
    let mut grey_rows = vec![];
    let mut dev9 = 0.0f64;
    let mut flux9 = 0.0f64;
    for &mm in &[0.0, 0.2] {
        for &lam in &[1.0, -1.0, 2.0] {
            for &omw in &[0.25, 0.4, 0.6] {
                if omw <= mm {
                    continue;
                }
                let (g1, r1, jh, jf) = greybody(omw, mm, lam, 2000.0);
                let (g2, r2, _, jf2) = greybody(omw, mm, lam, 4000.0);
                dev9 = dev9.max((g2 - g1).abs());
                flux9 = flux9.max((jf - jh).abs() / jh).max((jf2 - jh).abs() / jh).max((g2 + r2 - 1.0).abs()).max((g1 + r1 - 1.0).abs());
                grey_rows.push(vec![f(mm), format!("{}", lam), f(omw), f(g2), f(r2), f(g2 - g1)]);
                println!("  greybody Mm = {} lambda = {} M omega = {}: Gamma = {:.8} (rmax 2000: {:.8})", mm, lam, omw, g2, g1);
            }
        }
    }
    rep.add(
        "LS-9a",
        "scattering: the greybody factors Gamma of the partial waves lambda = +-1, 2 at M omega = 0.25, 0.4, 0.6 and Mm = 0, \
         0.2 (data/lg_schw_grey.csv) obey flux conservation, Gamma + |R|^2 = 1, and the current at the outer radius equals \
         the current into the horizon (largest deviation; the global error of CVODE with rtol 1e-13 over r = 2.5 to 4000)",
        flux9,
        1e-7,
    );
    rep.add(
        "LS-9b",
        "the outer radius of the decomposition into outgoing and incoming waves: Gamma at r = 4000 and r = 2000 differ by \
         less than 1e-4 (the local eigenvectors leave errors of the order 1/(k r^2) and of the Coulomb phase)",
        dev9,
        1e-4,
    );

    // ------------------------------------------------------------------ the output
    write_csv(&data.join("lg_schw_values.csv"), &["name", "value"], &vals);
    write_csv(
        &data.join("lg_schw_qbs.csv"),
        &["state", "Mm", "lambda", "n", "Re_omega_over_m", "Im_omega_over_m", "hydrogen_like", "iterations", "residual"],
        &qbs_rows,
    );
    write_csv(&data.join("lg_schw_grey.csv"), &["Mm", "lambda", "M_omega", "Gamma", "R2", "Gamma_change_rmax"], &grey_rows);
    write_csv(&data.join("lg_schw_interior.csv"), &["r", "abs_P", "abs_Q"], &int_rows);
    rep.write(&data.join("hck_lg_schw_checks.txt"));
    let (np, nt) = rep.counts();
    println!("lg_schw: {} of {} checks pass", np, nt);
}
