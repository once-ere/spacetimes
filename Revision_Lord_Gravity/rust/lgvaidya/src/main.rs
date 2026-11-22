//! lg_vaidya -- stage 6 of task C23 (folder Revision_Lord_Gravity, PROJECT spacetimes): the brochon in the ingoing Vaidya
//! space-time, the field of a black hole whose mass M(v) changes by absorbing radial null dust (M' > 0), or, in the model
//! of an evaporating black hole, null dust of negative energy (M' < 0).  The brochon is a Dirac particle of mass m (decision
//! D9 of the task record), with no field of the zeugon.
//!
//! Units: G_N = c = hbar = 1 and the mass of the black hole at the start M_0 = 1 (the horizon at r = 2 at the start).
//!
//! THE GEOMETRY.  ds^2 = f dv^2 - 2 dv dr - r^2 (dtheta^2 + sin^2(theta) dphi^2), f = 1 - 2 M(v)/r, with the advanced time
//! v (the ingoing radial light rays are the lines v = const).  The coframe
//!     e^0 = ((1 + f)/2) dv - dr,   e^1 = ((1 - f)/2) dv + dr,   e^2 = r dtheta,   e^3 = r sin(theta) dphi
//! gives this metric and is regular at every r > 0, on both sides of the apparent horizon r = 2 M(v) (where f = 0); its
//! frame vectors are e_0 = d_v - ((1 - f)/2) d_r and e_1 = d_v + ((1 + f)/2) d_r.  For a constant mass it is a tetrad of
//! the Schwarzschild space-time in the coordinates of Eddington and Finkelstein, v = t + r + 2 ln(r/2 - 1).
//!
//! THE SEPARATION (derived by computer algebra; checks LV-1 here, YV-1 in Python and WV-1 in the Wolfram Language).  With
//! the angular functions A(theta), B(theta) of the Schwarzschild problem (A' = (m_j/sin) A - lambda B,
//! B' = -(m_j/sin) B + lambda A, lambda = +-(j + 1/2)) and the spinor
//!     psi = r^(-1) sin(theta)^(-1/2) e^(i m_j phi) S_H (-i Q A, i P B, P A, Q B),     P = P(v, r), Q = Q(v, r),
//! (S_H = diag(H, H), H = i (sigma_1 + sigma_3)/2^(1/2)), the Dirac equation i gamma^mu (d_mu + Omega_mu) psi = m psi of
//! Part III is equivalent to
//!     d_v P = (M/r - 1/2) d_r P - (M/(2 r^2)) P - (i lambda/(2 r) + m/2) Q,                                     (V1)
//!     d_r Q = (i lambda/r - m) P.                                                                                 (V2)
//! (V1) carries P along the outgoing radial light rays, dr/dv = f/2 (outward outside the apparent horizon, inward inside
//! it); (V2) contains no derivative in v: on each slice v = const (an ingoing light ray) Q follows from P by an integral
//! in r, from the outer edge, where Q = 0 when nothing comes in.  M'(v) does not appear.  The charge of the brochon obeys
//!     d_v (2 |P|^2) + d_r (f |P|^2 - |Q|^2) = 0                                                                  (V3)
//! exactly: 2 |P|^2 is its density on the slice v = const and f |P|^2 - |Q|^2 its flux through the sphere of radius r
//! (both divided by the integral of |A|^2 + |B|^2 over the sphere).
//!
//! THE STATIC LIMIT (check LV-2).  For a constant mass, P = e^(-i omega v) p(r), Q = e^(-i omega v) q(r) turn (V1), (V2) into
//!     r (r - 2) p' = 2 i omega r^2 p - p - (i lambda r + m r^2) q,      r q' = (i lambda - m r) p,
//! whose solution that is analytic at the horizon (a power series in x = r - 2, radius of convergence 2) falls in.  The
//! quasi-bound states (the solution that falls in and decays at infinity) must have the frequencies of stage 5, which
//! were computed in another tetrad and other coordinates: the frequencies do not depend on them.
//!
//! THE TIME EVOLUTION (checks LV-3, LV-4).  The method of lines: P on a uniform grid r_in <= r <= r_out with r_in inside
//! the apparent horizon at all times (there the light rays of (V1) move inward, out of the grid, so that no condition is
//! needed: the black hole is excised) and r_out far outside (the rays move outward, out of the grid); d_r P by finite
//! differences of the sixth order (seven points; one-sided at the edges; weights by Fornberg's algorithm), Q by the
//! integral of (V2) from r_out (Q(r_out) = 0) with interval weights of the sixth order, a small dissipation of the eighth
//! difference (Kreiss and Oliger) against the noise of the shortest wavelengths, and the integration in v by CVODE
//! (Adams formulas, the weights of the complex unknowns).  An extra unknown accumulates the net flux of the charge
//! through the two edges, so that (V3) can be checked in the discrete solution.
//!
//! THE CHANGES OF THE MASS (checks LV-5 to LV-8).  A brochon of mass m = 0.2 starts in the state 1S1/2 of the black hole of
//! mass 1 (Mm = 0.2).  From v = 50 the mass changes smoothly (a step with all derivatives continuous) to 1.25 (growth by
//! ingoing null dust, M' > 0) or to 0.8 (ingoing null dust of negative energy, M' < 0, the model of an evaporating black
//! hole near its horizon), slowly (over the time 1000) or fast (over the time 20); then the mass stays constant for the
//! time 300.  The grid r = 1.2 to 900 contains the apparent horizon at all times; the outgoing light rays move with dr/dv =
//! f/2 < 1/2, so that nothing from inside r = 225 reaches the outer edge before the end of the slow runs (v = 1350), and
//! the flux through the outer edge is part of the recorded inflow.  Recorded: the charge, the net inflow, and the projections of the state on
//! the states 1S1/2 and 2S1/2 of the instantaneous mass (the "occupations" |<p_n, P>|^2/(<p_n, p_n> <P, P>), with the charge
//! product <a, b> = int 2 conj(a) b dr); the charge that a state 1S1/2 following the mass adiabatically would keep,
//! exp(int 2 Im omega_1S dv); the overlap that a sudden change would leave; and the decomposition of the state in the
//! states 1S1/2 to 4S1/2 of the final mass, at single times (Gram system) and by a fit over the time after the change.
//! The quasi-bound states lose charge into the black hole, the evolution is not unitary, and the states are not
//! orthogonal under the charge product: the occupations and the charges assigned to the states are therefore not
//! probabilities, and the program reports how far they are determined (the overlap of 1S1/2 and 2S1/2, the remainder of
//! the decompositions, and whether the coefficients evolve with the frequencies of the states).
//!
//! Output (folder data/): hck_lg_vaidya_checks.txt (the report), lg_vaidya_values.csv (the numbers quoted by the record),
//! lg_vaidya_runs.csv (the four runs at every output time).
//! Build and run (from the folder rust/): cargo build --release && ./target/release/lg_vaidya (about 6 minutes)
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
// (the four helpers below are copies of those of the program lg_schw)

/// the constant spinor rotation S_H = diag(H, H), H = i (sigma_1 + sigma_3) / 2^(1/2)
fn s_h() -> CMat {
    let h = sigma(1).add(&sigma(3)).scale(c(0.0, 1.0 / 2f64.sqrt()));
    CMat::block_diag(&h, &h)
}

/// the eight components of the brochon (isospin up) from the four Dirac components
fn to8(psi4: &[D4]) -> Vec<D4> {
    let mut v = vec![D4::ZERO; 8];
    for d in 0..4 {
        v[2 * d] = psi4[d];
    }
    v
}

/// S_H (-i Q A, i P B, P A, Q B) times the prefactor
fn assemble(p: D4, q: D4, aa: D4, bb: D4, prefactor: D4) -> Vec<D4> {
    let chi = [q.scale(-I) * aa, p.scale(I) * bb, p * aa, q * bb];
    let sh = s_h();
    (0..4)
        .map(|i| (0..4).fold(D4::ZERO, |s, j| s + chi[j].scale(sh.get(i, j))) * prefactor)
        .collect()
}

/// the angular functions at theta as dual numbers: values a0, b0 and the derivatives of (A)
fn angular(th: f64, a0: C64, b0: C64, lam: f64, mj: f64) -> (D4, D4) {
    let s = th.sin();
    let mut a = D4::cst(a0);
    a.d[2] = a0 * (mj / s) - b0 * lam;
    let mut b = D4::cst(b0);
    b.d[2] = b0 * (-mj / s) + a0 * lam;
    (a, b)
}

// ================================================================================================ LV-1: the separation

/// the mass function of the check LV-1 (any smooth function of v serves): M(v) = 1 + 0.3 sin(0.7 v + 0.4)
fn m_test(v: D4) -> D4 {
    D4::re(1.0) + (v * D4::re(0.7) + D4::re(0.4)).sin() * D4::re(0.3)
}

/// the coframe of the ingoing Vaidya space-time at x = (v, r, theta, phi), with exact derivatives
fn coframe_vaidya(x: &[f64; 4]) -> M4 {
    let v = D4::coord(x, 0);
    let r = D4::coord(x, 1);
    let th = D4::coord(x, 2);
    let f = D4::re(1.0) - D4::re(2.0) * m_test(v) * r.inv();
    let mut co = [[D4::ZERO; 4]; 4];
    co[0][0] = (D4::re(1.0) + f) * D4::re(0.5);
    co[0][1] = D4::re(-1.0);
    co[1][0] = (D4::re(1.0) - f) * D4::re(0.5);
    co[1][1] = D4::re(1.0);
    co[2][2] = r;
    co[3][3] = r * th.sin();
    co
}

/// The Dirac residual of the separated spinor at x for the values P = p0, Q = q0 and the free derivatives d_r P = pr and
/// d_v Q = qv; d_v P is set by (V1) and d_r Q by (V2).  Relative to the size of the spinor.
#[allow(clippy::too_many_arguments)]
fn resid_vaidya(x: &[f64; 4], m: f64, lam: f64, mj: f64, p0: C64, q0: C64, pr: C64, qv: C64, a0: C64, b0: C64) -> f64 {
    let geo = Geom::new(coframe_vaidya(x));
    let r = x[1];
    let mv = m_test(D4::re(x[0])).v.re;
    let mut p = D4::cst(p0);
    p.d[1] = pr;
    p.d[0] = pr * (mv / r - 0.5) - p0 * (mv / (2.0 * r * r)) - q0 * (I * (lam / (2.0 * r)) + cr(m / 2.0));
    let mut q = D4::cst(q0);
    q.d[0] = qv;
    q.d[1] = p0 * (I * (lam / r) - cr(m));
    let rr = D4::coord(x, 1);
    let th = D4::coord(x, 2);
    let ph = D4::coord(x, 3);
    let pref = rr.inv() * th.sin().sqrt().inv() * ph.scale(c(0.0, mj)).exp();
    let (aa, bb) = angular(x[2], a0, b0, lam, mj);
    let psi = to8(&assemble(p, q, aa, bb, pref));
    let res = dirac_resid(&geo, &Gauge::zero(0.0, 0.0), &psi, m);
    vmax0(&res) / vmax0(&psi).max(1e-300)
}

// ================================================================================================ the static limit

/// the power series at the horizon (M = 1) of the solution of the static equations that is analytic there, q_0 = 1:
///     p_k (2k - 8 i omega + 1) = -(k - 1) p_{k-1} + 8 i omega p_{k-1} + 2 i omega p_{k-2} - (2 i lambda + 4 m) q_k
///                                - (i lambda + 4 m) q_{k-1} - m q_{k-2},
///     q_{k+1} = ((i lambda - 2 m) p_k - m p_{k-1} - k q_k) / (2 (k + 1))
/// (the orders x^k of r (r - 2) p' = 2 i omega r^2 p - p - (i lambda r + m r^2) q and (2 + x) q' = (i lambda - 2m - m x) p)
fn ef_series(om: C64, m: f64, lam: f64, n: usize) -> (Vec<C64>, Vec<C64>) {
    let mut p = vec![cr(0.0); n + 1];
    let mut q = vec![cr(0.0); n + 2];
    q[0] = cr(1.0);
    let at = |v: &Vec<C64>, k: isize| -> C64 {
        if k < 0 {
            cr(0.0)
        } else {
            v[k as usize]
        }
    };
    for k in 0..=n {
        let ki = k as isize;
        let rhs = at(&p, ki - 1) * cr(-(k as f64 - 1.0)) + at(&p, ki - 1) * (I * om * 8.0) + at(&p, ki - 2) * (I * om * 2.0)
            - q[k] * (I * (2.0 * lam) + cr(4.0 * m))
            - at(&q, ki - 1) * (I * lam + cr(4.0 * m))
            - at(&q, ki - 2) * cr(m);
        p[k] = rhs / (cr(2.0 * k as f64 + 1.0) - I * om * 8.0);
        q[k + 1] = (p[k] * (I * lam - cr(2.0 * m)) - at(&p, ki - 1) * cr(m) - q[k] * cr(k as f64)) / cr(2.0 * (k as f64 + 1.0));
    }
    q.truncate(n + 1);
    (p, q)
}

fn series_eval(cf: &[C64], x: f64) -> C64 {
    cf.iter().rev().fold(cr(0.0), |s, &a| s * x + a)
}

/// the static equations (M = 1): (p', q') at r
fn rhs_ef(r: f64, p: C64, q: C64, om: C64, m: f64, lam: f64) -> (C64, C64) {
    let fr = 1.0 - 2.0 / r;
    let dp = (p * (I * om * 2.0 - cr(1.0 / (r * r))) - q * (I * (lam / r) + cr(m))) * (1.0 / fr);
    let dq = p * (I * (lam / r) - cr(m));
    (dp, dq)
}

fn settings(rtol: f64) -> CvodeSettings {
    CvodeSettings::adams(rtol, 1e-300_f64.max(rtol * 1e-14), Weights::Complex)
}

/// integrates the static equations from r0 to the radii `rs` (increasing outward if sgn = 1, decreasing inward if sgn = -1,
/// in the variable s = sgn r); returns (p, q) at `rs`
fn ef_integrate(r0: f64, p0: C64, q0: C64, rs: &[f64], sgn: f64, om: C64, m: f64, lam: f64, rtol: f64) -> Vec<(C64, C64)> {
    let y0 = vec![p0.re, p0.im, q0.re, q0.im];
    let rhs = Box::new(move |s: f64, y: &[f64], dy: &mut [f64]| -> i32 {
        let (dp, dq) = rhs_ef(sgn * s, c(y[0], y[1]), c(y[2], y[3]), om, m, lam);
        dy[0] = sgn * dp.re;
        dy[1] = sgn * dp.im;
        dy[2] = sgn * dq.re;
        dy[3] = sgn * dq.im;
        0
    });
    let ss: Vec<f64> = rs.iter().map(|r| sgn * r).collect();
    let mut out = vec![];
    integrate(&y0, sgn * r0, &ss, &settings(rtol), rhs, |_, _, y| {
        out.push((c(y[0], y[1]), c(y[2], y[3])));
        true
    })
    .expect("CVODE (static equations)");
    out
}

/// the eigenvector of the matrix of the static equations at r whose eigenvalue has the smaller real part (decay outward)
fn ef_decaying(r: f64, om: C64, m: f64, lam: f64) -> [C64; 2] {
    let fr = 1.0 - 2.0 / r;
    let a = [[(I * om * 2.0 - cr(1.0 / (r * r))) * (1.0 / fr), -(I * (lam / r) + cr(m)) * (1.0 / fr)], [I * (lam / r) - cr(m), cr(0.0)]];
    let tr = a[0][0] + a[1][1];
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    let disc = (tr * tr - det * 4.0).sqrt();
    let (l1, l2) = ((tr - disc) * 0.5, (tr + disc) * 0.5);
    let l = if l1.re < l2.re { l1 } else { l2 };
    let v = [a[0][1], l - a[0][0]];
    let n = (v[0].norm_sqr() + v[1].norm_sqr()).sqrt();
    [v[0] * (1.0 / n), v[1] * (1.0 / n)]
}

/// the normalized Wronskian at rmatch of the solution that falls in (series to r = 2.5, then CVODE) and the solution
/// that decays at infinity (from rmax inward)
fn shoot_ef(om: C64, m: f64, lam: f64, rmatch: f64, rmax: f64) -> C64 {
    let (pc, qc) = ef_series(om, m, lam, 90);
    let x0 = 0.5;
    let (ph, qh) = ef_integrate(2.0 + x0, series_eval(&pc, x0), series_eval(&qc, x0), &[rmatch], 1.0, om, m, lam, 1e-13)[0];
    let vd = ef_decaying(rmax, om, m, lam);
    let (pi, qi) = ef_integrate(rmax, vd[0], vd[1], &[rmatch], -1.0, om, m, lam, 1e-13)[0];
    (ph * qi - qh * pi) * (1.0 / ((ph.norm_sqr() + qh.norm_sqr()).sqrt() * (pi.norm_sqr() + qi.norm_sqr()).sqrt()))
}

fn secant_ef(m: f64, lam: f64, rmatch: f64, rmax: f64, w0: C64, w1: C64) -> (C64, usize) {
    let (mut a, mut b) = (w0, w1);
    let mut fa = shoot_ef(a, m, lam, rmatch, rmax);
    let mut fb = shoot_ef(b, m, lam, rmatch, rmax);
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
        fb = shoot_ef(b, m, lam, rmatch, rmax);
        it += 1;
        if (b - a).abs() < 1e-14 * b.abs() {
            break;
        }
    }
    (b, it)
}

/// The quasi-bound state (p, q) at the radii of a grid (r_in > 0, increasing): the series for r <= 2.5 (it converges for
/// |r - 2| < 2), CVODE outward to rmatch, and beyond rmatch the decaying solution from rmax, scaled to fit at rmatch by
/// least squares; normalized by q(2) = 1.
fn qbs_profile(om: C64, m: f64, lam: f64, rs: &[f64], rmatch: f64, rmax: f64) -> Vec<(C64, C64)> {
    let (pc, qc) = ef_series(om, m, lam, 90);
    let x0 = 0.5;
    let mut out = vec![(cr(0.0), cr(0.0)); rs.len()];
    let mut mid: Vec<(usize, f64)> = vec![];
    let mut far: Vec<(usize, f64)> = vec![];
    for (i, &r) in rs.iter().enumerate() {
        if r <= 2.0 + x0 {
            out[i] = (series_eval(&pc, r - 2.0), series_eval(&qc, r - 2.0));
        } else if r <= rmatch {
            mid.push((i, r));
        } else {
            far.push((i, r));
        }
    }
    let mut rad: Vec<f64> = mid.iter().map(|&(_, r)| r).collect();
    rad.push(rmatch);
    let res = ef_integrate(2.0 + x0, series_eval(&pc, x0), series_eval(&qc, x0), &rad, 1.0, om, m, lam, 1e-13);
    for (k, &(i, _)) in mid.iter().enumerate() {
        out[i] = res[k];
    }
    let (ph, qh) = res[res.len() - 1];
    let vd = ef_decaying(rmax, om, m, lam);
    let mut radf: Vec<f64> = far.iter().rev().map(|&(_, r)| r).collect();
    radf.push(rmatch);
    let resf = ef_integrate(rmax, vd[0], vd[1], &radf, -1.0, om, m, lam, 1e-13);
    let (pd, qd) = resf[resf.len() - 1];
    let cfit = (pd.conj() * ph + qd.conj() * qh) / cr(pd.norm_sqr() + qd.norm_sqr());
    for (k, &(i, _)) in far.iter().rev().enumerate() {
        out[i] = (resf[k].0 * cfit, resf[k].1 * cfit);
    }
    out
}

// ================================================================================================ the method of lines

/// The weights of Fornberg's algorithm for the first derivative at z from the nodes xs.
fn fornberg_d1(z: f64, xs: &[f64]) -> Vec<f64> {
    let n = xs.len();
    // c[j][k]: weight of node j for the derivative of order k (k = 0, 1)
    let mut cw = vec![[0.0f64; 2]; n];
    let mut c1 = 1.0;
    let mut c4 = xs[0] - z;
    cw[0][0] = 1.0;
    for i in 1..n {
        let mn = i.min(1);
        let mut c2 = 1.0;
        let c5 = c4;
        c4 = xs[i] - z;
        for j in 0..i {
            let c3 = xs[i] - xs[j];
            c2 *= c3;
            if j == i - 1 {
                for k in (1..=mn).rev() {
                    cw[i][k] = c1 * (k as f64 * cw[i - 1][k - 1] - c5 * cw[i - 1][k]) / c2;
                }
                cw[i][0] = -c1 * c5 * cw[i - 1][0] / c2;
            }
            for k in (1..=mn).rev() {
                cw[j][k] = (c4 * cw[j][k] - k as f64 * cw[j][k - 1]) / c3;
            }
            cw[j][0] = c4 * cw[j][0] / c3;
        }
        c1 = c2;
    }
    cw.iter().map(|w| w[1]).collect()
}

/// the weights w_k of the integral over [0, 1] of the polynomial through the nodes t_k (6 nodes): the moments
/// sum_k w_k t_k^j = 1/(j + 1), j = 0..5, solved by Gaussian elimination with pivoting
fn interval_weights(ts: &[f64]) -> Vec<f64> {
    let n = ts.len();
    let mut a = vec![vec![0.0f64; n + 1]; n];
    for j in 0..n {
        for k in 0..n {
            a[j][k] = ts[k].powi(j as i32);
        }
        a[j][n] = 1.0 / (j as f64 + 1.0);
    }
    for col in 0..n {
        let piv = (col..n).max_by(|&x, &y| a[x][col].abs().partial_cmp(&a[y][col].abs()).unwrap()).unwrap();
        a.swap(col, piv);
        for row in 0..n {
            if row != col {
                let fct = a[row][col] / a[col][col];
                for k in col..=n {
                    a[row][k] -= fct * a[col][k];
                }
            }
        }
    }
    (0..n).map(|k| a[k][n] / a[k][k]).collect()
}

/// the grid and its operators: derivative stencils (seven points), the weights of the integral over each interval [r_i,
/// r_{i+1}] (six points) and the weights of the integral over the whole grid
#[derive(Clone)]
struct Grid {
    r: Vec<f64>,
    h: f64,
    dst: Vec<(usize, [f64; 7])>,
    ist: Vec<(usize, [f64; 6])>,
    qw: Vec<f64>,
}

impl Grid {
    fn new(r_in: f64, r_out: f64, h: f64) -> Grid {
        let n = ((r_out - r_in) / h).round() as usize + 1;
        let h = (r_out - r_in) / (n - 1) as f64;
        let r: Vec<f64> = (0..n).map(|i| r_in + h * i as f64).collect();
        let mut dst = vec![];
        for i in 0..n {
            let s = i.saturating_sub(3).min(n - 7);
            let xs: Vec<f64> = (0..7).map(|k| (s + k) as f64 - i as f64).collect();
            let w = fornberg_d1(0.0, &xs);
            let mut ww = [0.0; 7];
            for k in 0..7 {
                ww[k] = w[k] / h;
            }
            dst.push((s, ww));
        }
        let mut ist = vec![];
        let mut qw = vec![0.0; n];
        for i in 0..n - 1 {
            let s = i.saturating_sub(2).min(n - 6);
            let ts: Vec<f64> = (0..6).map(|k| (s + k) as f64 - i as f64).collect();
            let w = interval_weights(&ts);
            let mut ww = [0.0; 6];
            for k in 0..6 {
                ww[k] = w[k] * h;
                qw[s + k] += w[k] * h;
            }
            ist.push((s, ww));
        }
        Grid { r, h, dst, ist, qw }
    }
    fn n(&self) -> usize {
        self.r.len()
    }
}

/// the parameters of an evolution: the grid, the mass of the brochon, lambda, the strength of the dissipation and the
/// mass function M(v) of the black hole
#[derive(Clone)]
struct Evo {
    g: Grid,
    m: f64,
    lam: f64,
    sigma: f64,
    mass: std::rc::Rc<dyn Fn(f64) -> f64>,
}

impl Evo {
    /// Q from P by (V2), integrated inward from r_out where Q = 0
    fn q_of(&self, p: &[C64]) -> Vec<C64> {
        let n = self.g.n();
        let gfun: Vec<C64> = (0..n).map(|i| p[i] * (I * (self.lam / self.g.r[i]) - cr(self.m))).collect();
        let mut q = vec![cr(0.0); n];
        for i in (0..n - 1).rev() {
            let (s, w) = self.g.ist[i];
            let mut integ = cr(0.0);
            for k in 0..6 {
                integ += gfun[s + k] * w[k];
            }
            q[i] = q[i + 1] - integ;
        }
        q
    }
    /// dP/dv by (V1) with the dissipation; returns the flux f|P|^2 - |Q|^2 at the two edges
    fn rhs(&self, v: f64, p: &[C64], dp: &mut [C64]) -> (f64, f64) {
        let n = self.g.n();
        let mv = (self.mass)(v);
        let q = self.q_of(p);
        for i in 0..n {
            let (s, w) = self.g.dst[i];
            let mut d = cr(0.0);
            for k in 0..7 {
                d += p[s + k] * w[k];
            }
            let r = self.g.r[i];
            dp[i] = d * (mv / r - 0.5) - p[i] * (mv / (2.0 * r * r)) - q[i] * (I * (self.lam / (2.0 * r)) + cr(self.m / 2.0));
        }
        if self.sigma > 0.0 {
            // the dissipation of Kreiss and Oliger: -sigma/(256 h) times the eighth difference, which damps the shortest
            // wavelengths of the grid at the rate sigma/h and changes smooth solutions only at the order h^7
            let cf = [1.0, -8.0, 28.0, -56.0, 70.0, -56.0, 28.0, -8.0, 1.0];
            let fac = -self.sigma / (256.0 * self.g.h);
            for i in 4..n - 4 {
                let mut d8 = cr(0.0);
                for k in 0..9 {
                    d8 += p[i + k - 4] * cf[k];
                }
                dp[i] += d8 * fac;
            }
        }
        let flux = |i: usize| -> f64 {
            let r = self.g.r[i];
            (1.0 - 2.0 * mv / r) * p[i].norm_sqr() - q[i].norm_sqr()
        };
        (flux(0), flux(n - 1))
    }
    /// the charge on the slice: the integral of 2 |P|^2 over the grid
    fn charge(&self, p: &[C64]) -> f64 {
        (0..self.g.n()).map(|i| 2.0 * p[i].norm_sqr() * self.g.qw[i]).sum()
    }
    /// evolves P from v0 to the output times (increasing); returns, at each output time, P, the charge and the
    /// accumulated net inflow of charge through the two edges (the integral in v of F(r_in) - F(r_out)), and the
    /// statistics of CVODE
    fn evolve(&self, p0: &[C64], v0: f64, vouts: &[f64], rtol: f64) -> (Vec<(Vec<C64>, f64, f64)>, String) {
        let n = self.g.n();
        let mut y0 = vec![0.0; 2 * n + 2];
        for i in 0..n {
            y0[2 * i] = p0[i].re;
            y0[2 * i + 1] = p0[i].im;
        }
        let evo = self.clone();
        let rhs = Box::new(move |v: f64, y: &[f64], dy: &mut [f64]| -> i32 {
            let p: Vec<C64> = (0..n).map(|i| c(y[2 * i], y[2 * i + 1])).collect();
            let mut dp = vec![cr(0.0); n];
            let (fin, fout) = evo.rhs(v, &p, &mut dp);
            for i in 0..n {
                dy[2 * i] = dp[i].re;
                dy[2 * i + 1] = dp[i].im;
            }
            dy[2 * n] = fin - fout;
            dy[2 * n + 1] = 0.0; // the partner of the accumulated inflow for the complex error weights
            0
        });
        let mut out = vec![];
        let st = integrate(&y0, v0, vouts, &settings(rtol), rhs, |_, _, y| {
            let p: Vec<C64> = (0..n).map(|i| c(y[2 * i], y[2 * i + 1])).collect();
            let ch = self.charge(&p);
            out.push((p, ch, y[2 * n]));
            true
        })
        .expect("CVODE (evolution)");
        (out, st.describe())
    }
}

// ================================================================================================ the mass changes

/// a smooth step from 0 (x <= 0) to 1 (x >= 1) with all derivatives continuous: e^(-1/x) / (e^(-1/x) + e^(-1/(1-x)))
fn smooth_step(x: f64) -> f64 {
    if x <= 0.0 {
        0.0
    } else if x >= 1.0 {
        1.0
    } else {
        let a = (-1.0 / x).exp();
        let b = (-1.0 / (1.0 - x)).exp();
        a / (a + b)
    }
}

/// The small-mass formula of the level nS1/2 (l = 0, j = 1/2) of stage 5, for x = M m: Re omega / m = 1 - x^2/(2 n^2) +
/// F_n x^4 with F_n = (15/8 - 3n/(2j + 1) - 3n/(2l + 1))/n^4 = (15/8 - 9n/2)/n^4 (F_1 = -2.625, F_2 = -0.4453125).
fn s_level(n: usize, x: f64) -> f64 {
    let nf = n as f64;
    let ff = (15.0 / 8.0 - 4.5 * nf) / nf.powi(4);
    1.0 - x * x / (2.0 * nf * nf) + ff * x.powi(4)
}

/// The quasi-bound state nS1/2 (lambda = 1; n = 1, 2, 3, ...) of a black hole of mass `mass` for a brochon of mass m (both
/// in units of the initial mass of the black hole), followed from the mass `mass_from`, where its frequency is w_from (in
/// the units of the evolution), in steps of at most 1 % of the mass (each a complex secant iteration of the static problem
/// in units of the current mass, started at the ratio omega/m of the previous step).  At the first call w_from = 0 and
/// mass_from = mass: the start is the small-mass formula s_level of the level, with the imaginary part -x^5/n^2, which is
/// close enough at Mm = 0.2.  Every root must lie closer to the formula level of n than half the distance to the formula
/// levels of n - 1 and n + 1 (otherwise the iteration has jumped to another state, and the program stops).  Returns omega
/// in the units of the evolution and the profile p on the grid.
fn qbs_at_mass(m: f64, mass_from: f64, w_from: C64, mass: f64, n: usize, grid: &Grid) -> (C64, Vec<C64>) {
    let nn = (n * n) as f64;
    // the ratio omega/m to start from
    let mut ratio = if w_from.abs() > 0.0 {
        w_from * (1.0 / m)
    } else {
        let x = m * mass;
        c(s_level(n, x), -x.powi(5) / nn)
    };
    let nsteps = (((mass - mass_from).abs() / (0.01 * mass_from.min(mass))).ceil() as usize).max(1);
    let mut wt = cr(0.0);
    let mut rmax = 0.0;
    for k in 1..=nsteps {
        let mk = mass_from + (mass - mass_from) * k as f64 / nsteps as f64;
        let mt = m * mk;
        let w_start = ratio * mt;
        let kap0 = (cr(mt * mt) - w_start * w_start).sqrt().re;
        rmax = (45.0 / kap0).max(grid.r[grid.n() - 1] / mk + 1.0);
        let (w, it) = secant_ef(mt, 1.0, nn / (mt * mt), rmax, w_start, w_start * c(1.0 + 1e-6, 0.0));
        // the guard against a jump to another state: half the distance of the formula levels of the neighbours
        let lev = s_level(n, mt);
        let below = if n > 1 { lev - s_level(n - 1, mt) } else { f64::INFINITY };
        let guard = 0.5 * below.min(s_level(n + 1, mt) - lev);
        assert!(
            it < 60 && w.re / mt > 0.5 && w.re / mt < 0.9999 && (w.re / mt - lev).abs() < guard,
            "the state n = {} at Mm = {} was not found ({} iterations, omega/m = {:?}, formula level {}, guard {})",
            n,
            mt,
            it,
            w * (1.0 / mt),
            lev,
            guard
        );
        ratio = w * (1.0 / mt);
        wt = w;
    }
    let mt = m * mass;
    let rs: Vec<f64> = grid.r.iter().map(|r| r / mass).collect();
    let prof = qbs_profile(wt, mt, 1.0, &rs, nn / (mt * mt), rmax);
    (wt * (1.0 / mass), prof.iter().map(|x| x.0).collect())
}

/// the charge product <a, b> = int 2 conj(a) b dr on the slice (the quadrature weights of the grid)
fn cprod(g: &Grid, a: &[C64], b: &[C64]) -> C64 {
    (0..g.n()).fold(cr(0.0), |s, i| s + a[i].conj() * b[i] * (2.0 * g.qw[i]))
}

/// The decomposition of P in the states p_1..p_K, which are not orthogonal under the charge product (the quasi-bound
/// states lose charge into the black hole, so their evolution is not unitary): the coefficients c of the best
/// approximation sum_k c_k p_k of P in the charge norm, from the Gram system G c = b, G_jk = <p_j, p_k>, b_j = <p_j, P>
/// (Gaussian elimination with pivoting), and the fraction <R, R>/<P, P> of the charge of the remainder R = P - sum c_k p_k.
fn decompose(g: &Grid, basis: &[Vec<C64>], p: &[C64]) -> (Vec<C64>, f64) {
    let k = basis.len();
    let a: Vec<Vec<C64>> = (0..k)
        .map(|j| {
            let mut row: Vec<C64> = (0..k).map(|l| cprod(g, &basis[j], &basis[l])).collect();
            row.push(cprod(g, &basis[j], p));
            row
        })
        .collect();
    let cs = solve_complex(a);
    let rem: Vec<C64> = (0..g.n()).map(|i| (0..k).fold(p[i], |s, j| s - cs[j] * basis[j][i])).collect();
    let frac = cprod(g, &rem, &rem).re / cprod(g, p, p).re;
    (cs, frac)
}

/// the solution of the complex linear system whose augmented matrix (k rows, k + 1 columns) is a, by Gaussian elimination
/// with pivoting
fn solve_complex(mut a: Vec<Vec<C64>>) -> Vec<C64> {
    let k = a.len();
    for col in 0..k {
        let piv = (col..k).max_by(|&x, &y| a[x][col].abs().partial_cmp(&a[y][col].abs()).unwrap()).unwrap();
        a.swap(col, piv);
        for row in 0..k {
            if row != col {
                let fct = a[row][col] / a[col][col];
                for l in col..=k {
                    let sub = fct * a[col][l];
                    a[row][l] -= sub;
                }
            }
        }
    }
    (0..k).map(|j| a[j][k] / a[j][j]).collect()
}

/// The space-time fit of the state after the change of the mass (constant mass): the coefficients c of
/// sum_n c_n e^(-i omega_n t) p_n that approximate the states P_k at the times t_k (measured from the end of the change)
/// best in the norm sum_k <R_k, R_k>, R_k = P_k - sum_n c_n e^(-i omega_n t_k) p_n (the normal equations
/// sum_n [sum_k conj(e_m(t_k)) e_n(t_k)] <p_m, p_n> c_n = sum_k conj(e_m(t_k)) <p_m, P_k>, e_n(t) = e^(-i omega_n t)), and
/// the fraction sum_k <R_k, R_k> / sum_k <P_k, P_k> of the remainder.  A part of P that does not evolve with the
/// frequencies of the states does not add up coherently over the window, so it affects these coefficients differently
/// from those of the decomposition at a single time; the difference of the two measures how far the charge of P can be
/// assigned to the states at all.
fn fit_space_time(g: &Grid, basis: &[Vec<C64>], w: &[C64], ps: &[&Vec<C64>], ts: &[f64]) -> (Vec<C64>, f64) {
    let k = basis.len();
    let gb: Vec<Vec<C64>> = (0..k).map(|m| (0..k).map(|n| cprod(g, &basis[m], &basis[n])).collect()).collect();
    let e = |n: usize, t: f64| -> C64 { (-(I * w[n]) * t).exp() };
    let mut a = vec![vec![cr(0.0); k + 1]; k];
    for (pk, &t) in ps.iter().zip(ts) {
        for m in 0..k {
            let em = e(m, t).conj();
            for n in 0..k {
                a[m][n] += em * e(n, t) * gb[m][n];
            }
            a[m][k] += em * cprod(g, &basis[m], pk);
        }
    }
    let cs = solve_complex(a);
    let (mut num, mut den) = (0.0, 0.0);
    for (pk, &t) in ps.iter().zip(ts) {
        let rem: Vec<C64> = (0..g.n()).map(|i| (0..k).fold(pk[i], |s, n| s - cs[n] * e(n, t) * basis[n][i])).collect();
        num += cprod(g, &rem, &rem).re;
        den += cprod(g, pk, pk).re;
    }
    (cs, num / den)
}

/// the occupation of the state p1 in P: |<p1, P>|^2 / (<p1, p1> <P, P>) with the charge product <a, b> = int 2 conj(a) b dr
fn occupation(g: &Grid, p1: &[C64], p: &[C64]) -> f64 {
    let mut ov = cr(0.0);
    let (mut n1, mut n2) = (0.0, 0.0);
    for i in 0..g.n() {
        ov += p1[i].conj() * p[i] * (2.0 * g.qw[i]);
        n1 += 2.0 * g.qw[i] * p1[i].norm_sqr();
        n2 += 2.0 * g.qw[i] * p[i].norm_sqr();
    }
    ov.norm_sqr() / (n1 * n2)
}

/// the number of the states nS1/2 (n = 1..NB) in which the state of a run is decomposed
const NB: usize = 4;

/// The decomposition of a run in the states nS1/2 (n = 1..NB) of the final mass: the frequencies of these states (units
/// of the evolution), their charges <p_n, p_n>, the normalized overlap |<p_1, p_2>| / (<p_1, p_1> <p_2, p_2>)^(1/2) of the
/// states 1S1/2 and 2S1/2 (zero for orthogonal states), and the coefficients and the fraction of the remainder (function
/// decompose) for: the initial state in the states of the initial mass (`initial`, which must be (1, 0, ..., 0) with no
/// remainder), the initial state in the states of the final mass (`sudden`: what a sudden change of the mass would
/// leave), the state at the end of the change (v_end = v1 + tc) and the state at the end of the run (v_fin); and the
/// space-time fit over the outputs from v_end to v_fin (`st`, function fit_space_time; its coefficients belong to v_end).
/// After v_end the mass is constant, so the coefficient of a state whose part in the remainder is small must change from
/// v_end to v_fin by the factor e^(-i omega_n (v_fin - v_end)).
struct Decomp {
    w: Vec<C64>,
    norms: Vec<f64>,
    overlap12: f64,
    initial: (Vec<C64>, f64),
    sudden: (Vec<C64>, f64),
    end: (Vec<C64>, f64),
    fin: (Vec<C64>, f64),
    st: (Vec<C64>, f64),
    v_end: f64,
    v_fin: f64,
}

/// One run: the state 1S1/2 of the brochon of mass m around the black hole of mass 1 as the initial data on v = 0; the mass
/// M(v) = 1 + delta smooth_step((v - v1)/tc) changes between v1 and v1 + tc; the evolution to v1 + tc + tail on the grid
/// of spacing h.  Returns the rows (at v = 0 and at the output times, every dv): v, the mass, the charge, the accumulated
/// inflow, the occupations of the states 1S1/2 and 2S1/2 of the instantaneous mass, the complex frequency of its state 1S1/2,
/// and P at r = 20; the sudden overlap: the occupation of the state 1S1/2 of the final mass in the state 1S1/2 of the
/// initial mass, which a sudden change of the mass would leave; and the decomposition in the states nS1/2 of the final mass
/// (struct Decomp).
#[allow(clippy::too_many_arguments)]
fn run(label: &str, m: f64, delta: f64, v1: f64, tc: f64, tail: f64, dv: f64, h: f64, r_out: f64) -> (Vec<Vec<f64>>, f64, Decomp) {
    let grid = Grid::new(1.2, r_out, h);
    let massf = move |v: f64| -> f64 { 1.0 + delta * smooth_step((v - v1) / tc) };
    // the states nS1/2 of the initial and of the final mass (the latter followed from the former in steps of 1 %)
    let basis0: Vec<(C64, Vec<C64>)> = (1..=NB).map(|n| qbs_at_mass(m, 1.0, cr(0.0), 1.0, n, &grid)).collect();
    let basisf: Vec<(C64, Vec<C64>)> = (1..=NB).map(|n| qbs_at_mass(m, 1.0, basis0[n - 1].0, 1.0 + delta, n, &grid)).collect();
    let (w0, p0) = (basis0[0].0, basis0[0].1.clone());
    let (w20, p20) = (basis0[1].0, basis0[1].1.clone());
    let p1_final = basisf[0].1.clone();
    let sudden = occupation(&grid, &p1_final, &p0);
    let b0: Vec<Vec<C64>> = basis0.iter().map(|x| x.1.clone()).collect();
    let bf: Vec<Vec<C64>> = basisf.iter().map(|x| x.1.clone()).collect();
    let evo = Evo { g: grid.clone(), m, lam: 1.0, sigma: 0.02, mass: std::rc::Rc::new(massf) };
    let v_end = v1 + tc + tail;
    let nout = (v_end / dv).round() as usize;
    let vouts: Vec<f64> = (1..=nout).map(|k| k as f64 * dv).collect();
    let (out, st) = evo.evolve(&p0, 0.0, &vouts, 1e-11);
    let ch0 = evo.charge(&p0);
    let i20 = ((20.0 - 1.2) / grid.h).round() as usize;
    let mut rows = vec![vec![0.0, 1.0, ch0, 0.0, 1.0, occupation(&grid, &p20, &p0), w0.re, w0.im, p0[i20].re, p0[i20].im]];
    // the instantaneous states, followed in the mass from one output to the next
    let (mut wt1, mut wt2, mut mprev) = (w0, w20, 1.0);
    for (k, (p, ch, acc)) in out.iter().enumerate() {
        let v = vouts[k];
        let mv = massf(v);
        let (w1, p1) = qbs_at_mass(m, mprev, wt1, mv, 1, &grid);
        let (w2, p2) = qbs_at_mass(m, mprev, wt2, mv, 2, &grid);
        wt1 = w1;
        wt2 = w2;
        mprev = mv;
        rows.push(vec![v, mv, *ch, *acc, occupation(&grid, &p1, p), occupation(&grid, &p2, p), w1.re, w1.im, p[i20].re, p[i20].im]);
    }
    println!("  run {}: {} points, {}", label, grid.n(), st);
    // the decomposition in the states of the final mass
    let v_end = v1 + tc;
    let k_end = vouts.iter().position(|&v| (v - v_end).abs() < 1e-9).expect("the end of the change is an output time");
    let wf: Vec<C64> = basisf.iter().map(|x| x.0).collect();
    let norms: Vec<f64> = bf.iter().map(|b| cprod(&grid, b, b).re).collect();
    let window: Vec<&Vec<C64>> = out[k_end..].iter().map(|x| &x.0).collect();
    let ts: Vec<f64> = vouts[k_end..].iter().map(|v| v - v_end).collect();
    let dec = Decomp {
        overlap12: cprod(&grid, &bf[0], &bf[1]).abs() / (norms[0] * norms[1]).sqrt(),
        initial: decompose(&grid, &b0, &p0),
        sudden: decompose(&grid, &bf, &p0),
        end: decompose(&grid, &bf, &out[k_end].0),
        fin: decompose(&grid, &bf, &out[out.len() - 1].0),
        st: fit_space_time(&grid, &bf, &wf, &window, &ts),
        w: wf,
        norms,
        v_end,
        v_fin: vouts[vouts.len() - 1],
    };
    (rows, sudden, dec)
}

// ================================================================================================ main

fn main() {
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "lg_vaidya: the brochon in the ingoing Vaidya space-time (stage 6 of task C23, Revision_Lord_Gravity, PROJECT spacetimes); \
         units G_N = c = hbar = 1, initial mass of the black hole 1",
    );
    let mut vals: Vec<Vec<String>> = vec![];
    let mut rng = Rng(20261122);

    // ------------------------------------------------------------------ LV-1: the separation
    let lams = [1.0, -1.0, 2.0, -2.0, 3.0, -3.0];
    let mjs = [0.5, -0.5, 1.5, -1.5];
    let mut dev1 = 0.0f64;
    let mut n_in = 0usize;
    for k in 0..80 {
        let lam = lams[k % 6];
        let mj = mjs[k % 4];
        let m = 0.2 + 0.6 * (0.5 + 0.5 * rng.unit());
        let rnd = |rng: &mut Rng| c(rng.unit(), rng.unit());
        let (p0, q0, pr, qv, a0, b0) = (rnd(&mut rng), rnd(&mut rng), rnd(&mut rng), rnd(&mut rng), rnd(&mut rng), rnd(&mut rng));
        // half of the points inside the apparent horizon (0.2 < r < 2 M(v) - 0.05), half outside (up to r = 20)
        let v = 10.0 * rng.unit();
        let mv = m_test(D4::re(v)).v.re;
        let r = if k % 2 == 0 { 0.2 + (2.0 * mv - 0.25) * (0.5 + 0.5 * rng.unit()) } else { 2.0 * mv + 0.05 + (20.0 - 2.0 * mv) * (0.5 + 0.5 * rng.unit()) };
        if r < 2.0 * mv {
            n_in += 1;
        }
        let x = [v, r, 0.2 + 2.7 * (0.5 + 0.5 * rng.unit()), 6.0 * rng.unit()];
        dev1 = dev1.max(resid_vaidya(&x, m, lam, mj, p0, q0, pr, qv, a0, b0));
    }
    rep.add(
        "LV-1",
        &format!(
            "the separation in the ingoing Vaidya space-time with M(v) = 1 + 0.3 sin(0.7 v + 0.4): the spinor r^(-1) sin(theta)^(-1/2) \
             e^(i m_j phi) S_H (-i Q A, i P B, P A, Q B), with (A), (V1) and (V2) and free values of P, Q, d_r P and d_v Q, solves the \
             Dirac equation of Part III (module lgcore::curved, the spinor connection from the tetrad postulate) at 80 random points, {} \
             of them inside the apparent horizon, for lambda = +-1, +-2, +-3, m_j = +-1/2, +-3/2 (largest residual relative to the spinor)",
            n_in
        ),
        dev1,
        1e-12,
    );
    vals.push(vec!["lv1_largest_residual".into(), f(dev1)]);

    // ------------------------------------------------------------------ LV-2: the static limit
    // the quasi-bound states of stage 5 (data/lg_schw_qbs.csv: Mm = 0.3) found again from the static equations of this
    // tetrad, from starts 1e-6 away
    let stage5: [(&str, f64, usize, f64, C64); 3] = [
        ("1S1/2", 1.0, 1, 0.3, c(9.45463937508782215e-1, -2.96484805870455143e-2)),
        ("2P1/2", -1.0, 2, 0.3, c(9.86183492456925381e-1, -4.08444298954018816e-4)),
        ("2P3/2", 2.0, 2, 0.3, c(9.87746803741051616e-1, -3.29846326644492460e-6)),
    ];
    let mut dev2 = 0.0f64;
    let mut det2 = vec![];
    for &(label, lam, n, mm, w5) in &stage5 {
        let w5 = w5 * mm;
        let kap = (cr(mm * mm) - w5 * w5).sqrt().re;
        let (w, it) = secant_ef(mm, lam, (n * n) as f64 / (mm * mm), 45.0 / kap, w5 * c(1.0 + 1e-6, 0.0), w5 * c(1.0 - 1e-6, 0.0));
        dev2 = dev2.max((w - w5).abs() / mm);
        det2.push(format!("{} {:.12} {:+.6e} i ({} iterations)", label, w.re / mm, w.im / mm, it));
    }
    rep.add(
        "LV-2",
        "the static limit: the quasi-bound states 1S1/2, 2P1/2 and 2P3/2 at Mm = 0.3 of stage 5 (data/lg_schw_qbs.csv, computed in the \
         freely falling tetrad) are found again from the static equations of the tetrad of Eddington and Finkelstein (series at the \
         horizon, CVODE, matching at n^2/(Mm)^2, complex secant): the largest |omega - omega_stage5|/m",
        dev2,
        1e-11,
    );
    rep.note(&format!("LV-2: {}", det2.join("; ")));

    // ------------------------------------------------------------------ LV-3, LV-4: the evolution of a quasi-bound state
    // The state 1S1/2 at Mm = 0.3 (constant mass 1) as initial data on the slice v = 0; at v = 60 the solution must be
    // e^(-i omega v) times the initial data.  Two grids (h = 0.1 and 0.05) show the order of the error.
    let (mm, lam, n) = (0.3, 1.0, 1usize);
    let w5 = stage5[0].4 * mm;
    let kap = (cr(mm * mm) - w5 * w5).sqrt().re;
    let (w, _) = secant_ef(mm, lam, (n * n) as f64 / (mm * mm), 45.0 / kap, w5 * c(1.0 + 1e-6, 0.0), w5 * c(1.0 - 1e-6, 0.0));
    let (r_in, r_out, v_end) = (1.2, 300.0, 60.0);
    let mass_const: std::rc::Rc<dyn Fn(f64) -> f64> = std::rc::Rc::new(|_v: f64| -> f64 { 1.0 });
    let mut errs = vec![];
    let mut bal = 0.0f64;
    let mut stats = vec![];
    for &h in &[0.1, 0.05] {
        let grid = Grid::new(r_in, r_out, h);
        let prof = qbs_profile(w, mm, lam, &grid.r, (n * n) as f64 / (mm * mm), 45.0 / kap);
        let p0: Vec<C64> = prof.iter().map(|x| x.0).collect();
        let evo = Evo { g: grid.clone(), m: mm, lam, sigma: 0.02, mass: mass_const.clone() };
        let (out, st) = evo.evolve(&p0, 0.0, &[v_end], 1e-11);
        let (pend, ch, acc) = &out[0];
        let ph = (-(I * w) * v_end).exp();
        let pmax = p0.iter().fold(0.0f64, |s, x| s.max(x.abs()));
        let err = (0..grid.n()).fold(0.0f64, |s, i| s.max((pend[i] - p0[i] * ph).abs())) / pmax;
        errs.push(err);
        let ch0 = evo.charge(&p0);
        bal = bal.max((ch - ch0 - acc).abs() / ch0);
        stats.push(format!("h = {}: {} points, {}", h, grid.n(), st));
        println!("  LV-3 h = {}: max |P(60) - e^(-i omega 60) P(0)| / max |P(0)| = {:.3e}; charge {:.10} -> {:.10}, inflow {:.3e}", h, err, ch0, ch, acc);
    }
    rep.add(
        "LV-3",
        &format!(
            "the evolution by the method of lines with CVODE reproduces the quasi-bound state 1S1/2 at Mm = 0.3 of a constant mass: from the \
             state on the slice v = 0 (grid r = 1.2 to 300, inside the horizon to far outside), at v = 60 the solution is e^(-i omega 60) \
             times the initial data, its decay by absorption into the black hole included (largest difference relative to the largest \
             |P|, on the finer grid h = 0.05; on the grid h = 0.1 it is {:.2e}, the ratio {:.1} shows the order of the error)",
            errs[0],
            errs[0] / errs[1]
        ),
        errs[1],
        1e-6,
    );
    rep.add(
        "LV-4",
        "the charge in the discrete evolution: the change of the integral of 2 |P|^2 over the slice from v = 0 to 60 equals the \
         integral in v of the net inflow f |P|^2 - |Q|^2 through the two edges of the grid (largest difference relative to the initial \
         charge, both grids)",
        bal,
        1e-6,
    );
    rep.note(&format!("LV-3: CVODE statistics: {}", stats.join("; ")));
    vals.push(vec!["lv3_error_h01".into(), f(errs[0])]);
    vals.push(vec!["lv3_error_h005".into(), f(errs[1])]);

    // ------------------------------------------------------------------ LV-5 to LV-8: the mass changes
    // The brochon of mass m = 0.2 (Mm = 0.2 at the start) in the state 1S1/2; the black hole grows by 25 % (accretion of null
    // dust, M' > 0) or shrinks by 20 % (null dust of negative energy, M' < 0), slowly (over v = 1000) or fast (over v = 20),
    // beginning at v = 50.  The grid r = 1.2 to 900 (spacing 0.1) contains the apparent horizon r = 2 M(v) at all times.
    let m_b = 0.2;
    let specs: [(&str, f64, f64, f64, f64); 4] = [
        ("grow_slow", 0.25, 1000.0, 300.0, 10.0),
        ("grow_fast", 0.25, 20.0, 300.0, 5.0),
        ("shrink_slow", -0.2, 1000.0, 300.0, 10.0),
        ("shrink_fast", -0.2, 20.0, 300.0, 5.0),
    ];
    let mut run_rows: Vec<Vec<String>> = vec![];
    let mut bal5 = 0.0f64;
    // (label, final occupation of 1S, of 2S, final charge ratio, adiabatic charge ratio, final mass, occupation of 1S at the
    // end of the change, sudden overlap)
    let mut finals: Vec<(String, f64, f64, f64, f64, f64, f64, f64)> = vec![];
    // per run: the decomposition, the initial charge and the adiabatic charge ratio at the end of the change
    let mut decs: Vec<(String, Decomp, f64, f64)> = vec![];
    for &(label, delta, tc, tail, dv) in &specs {
        let (rows, sudden, dec) = run(label, m_b, delta, 50.0, tc, tail, dv, 0.1, 900.0);
        let ch0 = rows[0][2];
        // the charge balance; the adiabatic charge exp(int 2 Im omega_1S dv) by the trapezoidal rule over the outputs
        let mut ad = 0.0f64;
        let mut occ_end_change = 0.0;
        let mut ad_end_change = 0.0;
        for k in 0..rows.len() {
            if k > 0 {
                ad += (rows[k][0] - rows[k - 1][0]) * (rows[k][7] + rows[k - 1][7]);
            }
            if (rows[k][0] - (50.0 + tc)).abs() < 1e-9 {
                occ_end_change = rows[k][4];
                ad_end_change = ad.exp();
            }
            bal5 = bal5.max((rows[k][2] - ch0 - rows[k][3]).abs() / ch0);
            run_rows.push(vec![
                label.to_string(), f(rows[k][0]), f(rows[k][1]), f(rows[k][2]), f(rows[k][3]), f(rows[k][4]), f(rows[k][5]), f(rows[k][6]),
                f(rows[k][7]), f(ch0 * ad.exp()), f(rows[k][8]), f(rows[k][9]),
            ]);
        }
        let last = rows.last().unwrap();
        let bal_run = rows.iter().fold(0.0f64, |b, rw| b.max((rw[2] - ch0 - rw[3]).abs() / ch0));
        println!("  run {}: charge balance {:.3e}", label, bal_run);
        finals.push((label.to_string(), last[4], last[5], last[2] / ch0, ad.exp(), last[1], occ_end_change, sudden));
        println!(
            "  run {}: at the end of the change occupation 1S {:.6}; final: occupation 1S {:.6}, 2S {:.6}, charge / initial {:.6e}, adiabatic {:.6e}; sudden overlap {:.6}",
            label, occ_end_change, last[4], last[5], last[2] / ch0, ad.exp(), sudden
        );
        decs.push((label.to_string(), dec, ch0, ad_end_change));
    }
    rep.add(
        "LV-5",
        "the charge in the four runs with a changing mass: the change of the integral of 2 |P|^2 over the slice equals the integral \
         of the net inflow through the edges at every output time (largest difference relative to the initial charge)",
        bal5,
        1e-6,
    );
    // LV-6: the fast runs again on the grid of twice the spacing (h = 0.2); the final occupation and charge must agree with
    // those of the grid h = 0.1, whose own error is smaller still (the scheme has the order 6)
    // the charge that the state n of the final mass carries alone, |c_n|^2 <p_n, p_n>, relative to the initial charge
    let alone = |d: &Decomp, cs: &[C64], n: usize, ch0: f64| -> f64 { cs[n].norm_sqr() * d.norms[n] / ch0 };
    let mut conv6 = 0.0f64;
    for &(label, delta, tc, tail, dv) in specs.iter().filter(|s| s.0.ends_with("fast")) {
        let (rows_f, _, dec_f) = run(&format!("{}_h02", label), m_b, delta, 50.0, tc, tail, dv, 0.2, 900.0);
        let fin = finals.iter().find(|x| x.0 == label).unwrap();
        let (_, dec, ch0, _) = decs.iter().find(|x| x.0 == label).unwrap();
        let lf = rows_f.last().unwrap();
        conv6 = conv6.max((lf[4] - fin.1).abs()).max((lf[2] / rows_f[0][2] - fin.3).abs());
        for n in 0..2 {
            conv6 = conv6.max((alone(&dec_f, &dec_f.fin.0, n, rows_f[0][2]) - alone(dec, &dec.fin.0, n, *ch0)).abs());
        }
    }
    rep.add(
        "LV-6",
        "the grid: the two fast runs repeated on the grid of twice the spacing (h = 0.2 instead of 0.1) give the same final \
         occupation of the state 1S1/2, the same final charge and the same final charges of the states 1S1/2 and 2S1/2 in the \
         decomposition (largest difference; the error of the grid h = 0.1 is smaller still, by about 2^6 for a scheme of the \
         order 6)",
        conv6,
        1e-6,
    );
    // LV-7: the decomposition of the initial state in the states of the initial mass is (1, 0, 0, 0) with no remainder (the
    // initial state is the first of them).  LV-8: the states 1S1/2 and 2S1/2 of the final mass of the growing runs (Mm = 0.25),
    // found by the continuation in the mass from Mm = 0.2, are those of stage 5 at Mm = 0.25 (data/lg_schw_qbs.csv); the
    // continuation of every state is guarded against jumps by the formula levels (function qbs_at_mass).
    let mut dev7 = 0.0f64;
    for (_, d, _, _) in &decs {
        let (cs, rem) = &d.initial;
        dev7 = dev7.max((cs[0] - cr(1.0)).abs()).max(*rem);
        for n in 1..NB {
            dev7 = dev7.max(cs[n].abs());
        }
    }
    rep.add(
        "LV-7",
        "the decomposition (Gram system in the charge product): the initial state of every run, decomposed in the states 1S1/2 to \
         4S1/2 of the initial mass, has the coefficients (1, 0, 0, 0) and no remainder (largest deviation)",
        dev7,
        1e-10,
    );
    let stage5_025 = [c(9.58837701795108188e-1, -1.35865033351687814e-2), c(9.90251277819439224e-1, -1.92028132729105973e-3)];
    let mut dev8 = 0.0f64;
    for (label, d, _, _) in decs.iter().filter(|x| x.0.starts_with("grow")) {
        for n in 0..2 {
            let dev = (d.w[n] * (1.0 / m_b) - stage5_025[n]).abs();
            dev8 = dev8.max(dev);
            println!("  LV-8 {}: state {}S1/2 omega/m = {:.12} {:+.6e} i, stage 5 difference {:.2e}", label, n + 1, d.w[n].re / m_b, d.w[n].im / m_b, dev);
        }
    }
    rep.add(
        "LV-8",
        "the identification of the states of the final mass: the states 1S1/2 and 2S1/2 of the growing runs (Mm = 0.25), followed \
         from Mm = 0.2 in steps of 1 % of the mass, are those of stage 5 at Mm = 0.25 (data/lg_schw_qbs.csv, computed in the freely \
         falling tetrad): the largest |omega - omega_stage5|/m (the other states are guarded at every step by the formula levels)",
        dev8,
        1e-11,
    );
    // the measurements of the decomposition (notes and values; no tolerances, they are results)
    for (label, d, ch0, ad_end) in &decs {
        let dvv = d.v_fin - d.v_end;
        let fmt_w = |cs: &[C64]| -> String { (0..NB).map(|n| format!("{:.5}", alone(d, cs, n, *ch0))).collect::<Vec<_>>().join(", ") };
        let evol: Vec<String> = (0..NB)
            .map(|n| {
                let pred = d.end.0[n] * (-(I * d.w[n]) * dvv).exp();
                format!("{:.2e}", (d.fin.0[n] / pred - cr(1.0)).abs())
            })
            .collect();
        // the width of the state 1S1/2 relative to the distance of the states 1S1/2 and 2S1/2
        let width_ratio = d.w[0].im.abs() / (d.w[1].re - d.w[0].re);
        rep.note(&format!(
            "decomposition, run {}: the charges |c_n|^2 <p_n, p_n> / (initial charge) of the states 1S..4S of the final mass: sudden \
             limit [{}] (remainder {:.3e}); end of the change v = {}: at that time [{}] (remainder {:.3e}; 1S relative to the \
             adiabatic charge {:.6}), by the space-time fit over v = {} to {} [{}] (remainder {:.3e}); end v = {} [{}] (remainder \
             {:.3e}); |c_n(end)/(c_n(change) e^(-i omega_n dv)) - 1| = [{}]; overlap of 1S and 2S {:.4}; |Im omega_1S| / (Re \
             omega_2S - Re omega_1S) = {:.4}",
            label,
            fmt_w(&d.sudden.0),
            d.sudden.1,
            d.v_end,
            fmt_w(&d.end.0),
            d.end.1,
            alone(d, &d.end.0, 0, *ch0) / ad_end,
            d.v_end,
            d.v_fin,
            fmt_w(&d.st.0),
            d.st.1,
            d.v_fin,
            fmt_w(&d.fin.0),
            d.fin.1,
            evol.join(", "),
            d.overlap12,
            width_ratio
        ));
        for n in 0..NB {
            vals.push(vec![format!("lv_{}_state{}S_omega_re", label, n + 1), f(d.w[n].re)]);
            vals.push(vec![format!("lv_{}_state{}S_omega_im", label, n + 1), f(d.w[n].im)]);
            vals.push(vec![format!("lv_{}_sudden_charge{}S", label, n + 1), f(alone(d, &d.sudden.0, n, *ch0))]);
            vals.push(vec![format!("lv_{}_end_of_change_charge{}S", label, n + 1), f(alone(d, &d.end.0, n, *ch0))]);
            vals.push(vec![format!("lv_{}_space_time_charge{}S", label, n + 1), f(alone(d, &d.st.0, n, *ch0))]);
            vals.push(vec![format!("lv_{}_final_charge{}S", label, n + 1), f(alone(d, &d.fin.0, n, *ch0))]);
            vals.push(vec![format!("lv_{}_end_of_change_c{}S_re", label, n + 1), f(d.end.0[n].re)]);
            vals.push(vec![format!("lv_{}_end_of_change_c{}S_im", label, n + 1), f(d.end.0[n].im)]);
            let pred = d.end.0[n] * (-(I * d.w[n]) * dvv).exp();
            vals.push(vec![format!("lv_{}_evolution_test{}S", label, n + 1), f((d.fin.0[n] / pred - cr(1.0)).abs())]);
        }
        vals.push(vec![format!("lv_{}_sudden_remainder", label), f(d.sudden.1)]);
        vals.push(vec![format!("lv_{}_end_of_change_remainder", label), f(d.end.1)]);
        vals.push(vec![format!("lv_{}_space_time_remainder", label), f(d.st.1)]);
        vals.push(vec![format!("lv_{}_final_remainder", label), f(d.fin.1)]);
        vals.push(vec![format!("lv_{}_end_of_change_adiabatic_charge_ratio", label), f(*ad_end)]);
        vals.push(vec![format!("lv_{}_overlap_1S_2S", label), f(d.overlap12)]);
        vals.push(vec![format!("lv_{}_width_over_spacing", label), f(width_ratio)]);
    }
    for (label, occ, occ2, chr, chad, mf, occe, sud) in &finals {
        vals.push(vec![format!("lv_{}_final_mass", label), f(*mf)]);
        vals.push(vec![format!("lv_{}_occupation_1S_end_of_change", label), f(*occe)]);
        vals.push(vec![format!("lv_{}_final_occupation_1S", label), f(*occ)]);
        vals.push(vec![format!("lv_{}_final_occupation_2S", label), f(*occ2)]);
        vals.push(vec![format!("lv_{}_final_charge_ratio", label), f(*chr)]);
        vals.push(vec![format!("lv_{}_adiabatic_charge_ratio", label), f(*chad)]);
        vals.push(vec![format!("lv_{}_sudden_overlap", label), f(*sud)]);
    }
    write_csv(
        &data.join("lg_vaidya_runs.csv"),
        &["run", "v", "M", "charge", "inflow", "occupation_1S", "occupation_2S", "Re_omega_1S", "Im_omega_1S", "adiabatic_charge", "Re_P_r20", "Im_P_r20"],
        &run_rows,
    );

    // ------------------------------------------------------------------ the output
    write_csv(&data.join("lg_vaidya_values.csv"), &["name", "value"], &vals);
    rep.write(&data.join("hck_lg_vaidya_checks.txt"));
    let (np, nt) = rep.counts();
    println!("lg_vaidya: {} of {} checks pass", np, nt);
}
