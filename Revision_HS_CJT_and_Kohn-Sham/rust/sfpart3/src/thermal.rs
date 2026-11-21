//! The free energy of the five methods at the temperature T (Section 12.4 of superfluid.md, the addendum asked for by
//! the author on 21 Nov 2026): the critical temperature, the order parameter Phi0(T), the constrained free energy
//! F(Phi; T), its Landau form and its accuracy, and the size of the order parameter against the radius of convergence
//! of the series of F in powers of Phi^2.
//!
//! THE FREE ENERGY.  A uniform quasi-free state at the temperature T is the thermal state R = f(H) of the BdG
//! matrices (f the Fermi function of the quasi-particle energies); its free energy per volume is Omega = E - T S, with
//! the energy E of the method's functional (unchanged) and the entropy S = -1/2 sum_n w_n sum_k [f ln f + (1-f) ln(1-f)]
//! of the quasi-free state (sfcore::uniform::states_t).  The stationary points of Omega are the self-consistent thermal
//! states (solve_t).  The free energy of a state with the field Phi is measured from the normal state at the same
//! temperature: F(Phi; T) = Omega(Phi) - Omega(0); at T = 0 it is the effective potential V(Phi).  The constrained
//! free energy is computed as the effective potential: an applied pair field Phi_a (K^3)^dag with a self-consistent
//! normal self-energy (fixed_pair_state_t).
//!
//! THE CRITICAL TEMPERATURE.  The normal state at T has a normal self-energy that is a Dirac mass M_n(T) (a scalar
//! density appears at T > 0 and shifts the mass through the scalar coupling g^S); its pair channel is unstable when
//! lambda^P(T) = g^P I1(M_n(T)^2, T)/pi^2 > 1, with I1(s, T) = int_0^Lambda q^2 tanh(E/2T)/E dq, E = (q^2 + s)^(1/2)
//! (the static pair response of AS3 at finite temperature; at T = 0 this is lambda^P of Part II).  T_c solves
//! lambda^P(T_c) = 1 (bisection; the normal state is computed at every step).
//!
//! THE GAP EQUATION.  A polar superfluid with the Dirac mass M and the pair amplitude Phi0 has the quasi-particle
//! energies (q^2 + M^2 + Phi0^2)^(1/2) (check P3-8), and its pair self-consistency is g^P I1(M^2 + Phi0^2, T)/pi^2 = 1:
//! the pair coupling alone fixes M^2 + Phi0^2 at every temperature (check P3-T2).
//!
//! THE SIZE OF THE ORDER PARAMETER.  The free energy contains sums over the Matsubara frequencies (2n+1) pi T of
//! functions of omega_n^2 + q^2 + M^2 + Phi_a^2; the nearest singularity in Phi_a^2 is at -(M_n^2 + pi^2 T^2), so the
//! series of F in powers of Phi^2 about the normal state converges within a radius of the order of
//! r(T) = (M_n(T)^2 + pi^2 T^2)^(1/2).  The program reports Phi0(T)/r(T), the Taylor coefficients a(T), b(T) of
//! F = a Phi^2 + b Phi^4 + ... at the origin (four small applied fields, exact interpolation in Phi^2), the prediction
//! of Landau's theory Phi0_L(T) = (-a/(2b))^(1/2), and the accuracy of the Landau form fitted to F(Phi; T).
use crate::landau::even_fit;
use sfcore::cmat::CMat;
use sfcore::dirac::{beta, d8, k_scalar};
use sfcore::methods::{effective_couplings, Method, Potential, M_LL};
use sfcore::report::Report;
use sfcore::uniform::{Model, ThermalSolution};

/// the reduced temperatures T/T_c of the table (0: the effective potential of Part III)
pub const TAUS: [f64; 13] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.85, 0.9, 0.95, 0.98];
/// the reduced temperatures of the free-energy curves
pub const CURVE_TAUS: [f64; 6] = [0.0, 0.5, 0.8, 0.9, 0.95, 0.98];

pub struct Point {
    pub tau: f64,
    pub t: f64,
    /// the order parameter (the pair amplitude of the superfluid), its Dirac mass, the gap (M^2 + Phi0^2)^(1/2)
    pub phi0: f64,
    pub mass: f64,
    pub gap: f64,
    /// the Dirac mass of the normal state, the radius r(T) = (M_n^2 + pi^2 T^2)^(1/2), Phi0/r
    pub mass_n: f64,
    pub radius: f64,
    /// the free energy of the superfluid from the normal state, and the entropy difference S - S_n
    pub free: f64,
    pub dentropy: f64,
    /// the residual of the gap equation, g^P I1(M^2 + Phi0^2, T)/pi^2 - 1
    pub gapeq: f64,
    /// the Taylor coefficients of F at the origin and the order parameter of Landau's theory (-a/(2b))^(1/2)
    pub a: f64,
    pub b: f64,
    pub phi0_landau: f64,
    pub iterations: usize,
}

pub struct Curve {
    pub tau: f64,
    /// (applied field, field Phi of the state, F(Phi; T))
    pub rows: Vec<(f64, f64, f64)>,
    /// the largest residual of the Landau form (K = 3) and of the quartic form (K = 2) fitted on [0, 1.4 Phi0(T)],
    /// relative to the depth |F(Phi0; T)|
    pub res6: f64,
    pub res4: f64,
}

pub struct Thermal {
    pub g_s: f64,
    pub g_p: f64,
    pub tc: f64,
    pub lam0: f64,
    pub points: Vec<Point>,
    pub curves: Vec<Curve>,
}

/// I1(s, T) = int_0^Lambda q^2 tanh(E/2T)/E dq with E = (q^2 + s)^(1/2), on the nodes of the grid of the model
/// (each node carries w = w_r q^2/(2 pi^2)/6, so the sum over the nodes times 2 pi^2 is the radial integral)
pub fn i1(model: &Model, s: f64, t: f64) -> f64 {
    let mut out = 0.0;
    for n in 0..model.grid.len() {
        let (q, w) = model.grid.node(n);
        let e = (q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + s).sqrt();
        let th = if t > 0.0 { (e / (2.0 * t)).tanh() } else { 1.0 };
        out += 2.0 * std::f64::consts::PI * std::f64::consts::PI * w * th / e;
    }
    out
}

fn dirac(n_self: &CMat) -> f64 {
    M_LL + d8(&beta()).mul(n_self).trace().re / 8.0
}

fn amplitude(p_self: &CMat) -> f64 {
    k_scalar(3).mul(p_self).trace().scale(1.0 / 8.0).abs()
}

/// the normal state at the temperature t (zero pair self-energy)
fn normal(model: &Model, t: f64) -> ThermalSolution {
    model.solve_t(&CMat::zeros(8), &CMat::zeros(8), 0.6, 1e-14, 200000, t)
}

/// lambda^P(T) = g^P I1(M_n(T)^2, T)/pi^2 of the normal state at T
fn lam_p(model: &Model, g_p: f64, t: f64) -> (f64, f64) {
    let ns = normal(model, t);
    let mn = dirac(&ns.n_self);
    (g_p * i1(model, mn * mn, t) / (std::f64::consts::PI * std::f64::consts::PI), mn)
}

/// the free energy F(Phi; T) of the state with the applied field fa (from the normal state with the free energy f_n)
fn point_f(model: &Model, fa: f64, n0: &CMat, t: f64, f_n: f64) -> (f64, f64, CMat) {
    let s = model.fixed_pair_state_t(&k_scalar(3).dagger().scale_r(fa), n0, 0.7, 1e-14, 200000, t);
    (amplitude(&s.p_self), s.free - f_n, s.n_self)
}

pub fn run(rep: &mut Report, tag: &str, m: &Method, v2: &Potential, model: &Model, gs_n: &CMat, gs_p: &CMat, phi0_0: f64, e0: f64, lam_part2: f64, res_part3: f64) -> Thermal {
    let (g_s, g_p) = effective_couplings(m, v2);
    let pi2 = std::f64::consts::PI * std::f64::consts::PI;
    // ---- the T -> 0 limit of the thermal engine: T = 0.005 (exp(-gap/T) < 1e-25) reproduces the ground state
    let low = model.solve_t(gs_n, gs_p, 0.6, 1e-14, 200000, 0.005);
    let n_low = normal(model, 0.005);
    rep.add(&format!("P3-T1 {}", tag), &format!("{}: the thermal engine at T = 0.005 reproduces the ground state of T = 0 (field and free energy; |change of Phi0| + relative change of the energy)", tag),
            (amplitude(&low.p_self) - phi0_0).abs() + ((low.free - n_low.free) - e0).abs() / e0.abs(), 1e-12);
    // ---- the critical temperature: lambda^P(T_c) = 1, by bisection
    let (lam0, _) = lam_p(model, g_p, 0.0);
    let (mut lo, mut hi) = (0.01, 1.0);
    assert!(lam_p(model, g_p, lo).0 > 1.0 && lam_p(model, g_p, hi).0 < 1.0);
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if lam_p(model, g_p, mid).0 > 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let tc = 0.5 * (lo + hi);
    let (lam_c, _) = lam_p(model, g_p, tc);
    rep.add(&format!("P3-T3 {}", tag), &format!("{}: the critical temperature T_c = {:.10} solves lambda^P(T_c) = 1 (|lambda^P(T_c) - 1|), and lambda^P(0) = {:.10} equals the eigenvalue of the static pair response of the normal vacuum computed by sf_part2 (difference)", tag, tc, lam0),
            (lam_c - 1.0).abs() + (lam0 - lam_part2).abs(), 1e-12);
    // ---- the points of the table: the superfluid at every T, the normal state, the Taylor coefficients
    let mut points = vec![];
    let (mut n_prev, mut p_prev) = (gs_n.clone(), gs_p.clone());
    let mut worst_gapeq = 0.0f64;
    for &tau in TAUS.iter() {
        let t = tau * tc;
        let sf = model.solve_t(&n_prev, &p_prev, 0.6, 1e-13, 400000, t);
        let ns = normal(model, t);
        n_prev = sf.n_self.clone();
        p_prev = sf.p_self.clone();
        let phi0 = amplitude(&sf.p_self);
        let mass = dirac(&sf.n_self);
        let mass_n = dirac(&ns.n_self);
        let gapeq = g_p * i1(model, mass * mass + phi0 * phi0, t) / pi2 - 1.0;
        worst_gapeq = worst_gapeq.max(gapeq.abs());
        // the Taylor coefficients of F(Phi; T) at the origin: F/Phi^2 = a + b x + c x^2 + d x^3 (x = Phi^2) through the
        // states of the applied fields 0.01, 0.02, 0.03, 0.04 (exact interpolation, Gaussian elimination)
        let mut xs = vec![];
        let mut ys = vec![];
        let mut n0 = ns.n_self.clone();
        for fa in [0.01, 0.02, 0.03, 0.04] {
            let (phi, f, nn) = point_f(model, fa, &n0, t, ns.free);
            n0 = nn;
            xs.push(phi * phi);
            ys.push(f / (phi * phi));
        }
        let coef = solve4(&xs, &ys);
        let (a, b) = (coef[0], coef[1]);
        let phi0_landau = if a < 0.0 && b > 0.0 { (-a / (2.0 * b)).sqrt() } else { f64::NAN };
        let radius = (mass_n * mass_n + pi2 * t * t).sqrt();
        points.push(Point { tau, t, phi0, mass, gap: (mass * mass + phi0 * phi0).sqrt(), mass_n, radius, free: sf.free - ns.free,
                            dentropy: sf.entropy - ns.entropy, gapeq, a, b, phi0_landau, iterations: sf.iterations });
    }
    rep.add(&format!("P3-T2 {}", tag), &format!("{}: the gap equation g^P I1(M^2 + Phi0^2, T)/pi^2 = 1 holds for the superfluid at every temperature of the table (largest |residual|; g^P = {:.6})", tag, g_p), worst_gapeq, 1e-9);
    // the Dirac mass of the superfluid does not depend on the temperature below T_c (the gap equation fixes the integral
    // that the scalar density contains, so the scalar self-consistency M = m + g^S n^S(M) has a temperature-independent
    // solution)
    let dm = points.iter().map(|p| (p.mass - points[0].mass).abs()).fold(0.0f64, f64::max);
    rep.add(&format!("P3-T8 {}", tag), &format!("{}: the Dirac mass of the superfluid is the same at every temperature below T_c ({:.10}; largest change)", tag, points[0].mass), dm, 1e-10);
    // Landau's theory near T_c: the order parameter (-a/2b)^(1/2) of the Taylor coefficients at the origin approaches
    // Phi0(T) with an error of relative order 1 - T/T_c
    let gl = points.iter().filter(|p| p.tau >= 0.9).map(|p| (p.phi0_landau / p.phi0 - 1.0).abs() / (1.0 - p.tau)).fold(0.0f64, f64::max);
    rep.add(&format!("P3-T5 {}", tag), &format!("{}: near T_c the order parameter of Landau's theory (-a/2b)^(1/2) approaches Phi0(T) with an error of relative order 1 - T/T_c (largest |(-a/2b)^(1/2)/Phi0 - 1| / (1 - T/T_c) at T/T_c = 0.9, 0.95, 0.98; at T/T_c = 0.98 the error is {:.2e})", tag,
            points.iter().find(|p| (p.tau - 0.98).abs() < 1e-12).map(|p| (p.phi0_landau / p.phi0 - 1.0).abs()).unwrap()), gl, 0.5);
    // the superfluid exists just below T_c and not just above it
    let below = model.solve_t(&n_prev, &p_prev, 0.6, 1e-13, 400000, 0.99 * tc);
    let above = model.solve_t(&CMat::zeros(8), &k_scalar(3).dagger().scale_r(0.05), 0.6, 1e-13, 400000, 1.01 * tc);
    let (pb, pa) = (amplitude(&below.p_self), amplitude(&above.p_self));
    rep.add_bool(&format!("P3-T4 {}", tag), &format!("{}: a polar start ends in a superfluid at 0.99 T_c (Phi0 = {:.3e}) and in the normal state at 1.01 T_c (Phi0 = {:.1e})", tag, pb, pa), pb > 1e-3 && pa < 1e-7, "");
    // ---- the free-energy curves and the accuracy of their Landau forms
    let mut curves = vec![];
    for &tau in CURVE_TAUS.iter() {
        let pt = points.iter().find(|p| (p.tau - tau).abs() < 1e-12).unwrap();
        let t = pt.t;
        let ns = normal(model, t);
        let mut rows = vec![];
        let mut n0 = ns.n_self.clone();
        // the applied fields of the curve of Part III: 61 points from 0 to 2 Phi0(T)
        for i in 0..=60 {
            let fa = 2.0 * pt.phi0 * i as f64 / 60.0;
            let (phi, f, nn) = point_f(model, fa, &n0, t, ns.free);
            n0 = nn;
            rows.push((fa, phi, f));
        }
        let fitted: Vec<(f64, f64)> = rows.iter().filter(|r| r.1 <= 1.4 * pt.phi0).map(|r| (r.1, r.2)).collect();
        let depth = pt.free.abs();
        let (_, r6) = even_fit(&fitted, 3);
        let (_, r4) = even_fit(&fitted, 2);
        curves.push(Curve { tau, rows, res6: r6 / depth, res4: r4 / depth });
    }
    // the curve at T = 0 is the effective potential of Section 12.1: its Landau residual is that of Table 12.1
    let c0 = curves.iter().find(|c| c.tau == 0.0).unwrap();
    rep.add(&format!("P3-T7 {}", tag), &format!("{}: at T = 0 the free energy is the effective potential: the largest residual of its Landau form, {:.6e} of the depth, is that of Table 12.1 (relative difference)", tag, c0.res6), (c0.res6 - res_part3).abs() / res_part3, 1e-8);
    // near T_c the Landau form becomes exact: the residual of the quartic form falls with 1 - T/T_c and that of (12.2) is
    // below 1e-4 of the depth at 0.98 T_c
    let r4: Vec<f64> = [0.9, 0.95, 0.98].iter().map(|&x| curves.iter().find(|c| (c.tau - x).abs() < 1e-12).unwrap().res4).collect();
    let r6_98 = curves.iter().find(|c| (c.tau - 0.98).abs() < 1e-12).unwrap().res6;
    rep.add_bool(&format!("P3-T6 {}", tag), &format!("{}: the Landau form becomes exact as T -> T_c: the residual of the quartic form falls ({:.2e}, {:.2e}, {:.2e} of the depth at 0.9, 0.95, 0.98 T_c) and that of (12.2) is {:.1e} of the depth at 0.98 T_c", tag, r4[0], r4[1], r4[2], r6_98),
                 r4[0] > r4[1] && r4[1] > r4[2] && r6_98 < 1e-4, "");
    Thermal { g_s, g_p, tc, lam0, points, curves }
}

/// the coefficients of the polynomial c0 + c1 x + c2 x^2 + c3 x^3 through four points (Gaussian elimination with
/// partial pivoting on the Vandermonde matrix)
fn solve4(xs: &[f64], ys: &[f64]) -> [f64; 4] {
    let mut a = [[0.0; 5]; 4];
    for i in 0..4 {
        for j in 0..4 {
            a[i][j] = xs[i].powi(j as i32);
        }
        a[i][4] = ys[i];
    }
    for k in 0..4 {
        let p = (k..4).max_by(|&i, &j| a[i][k].abs().partial_cmp(&a[j][k].abs()).unwrap()).unwrap();
        a.swap(k, p);
        for i in (k + 1)..4 {
            let f = a[i][k] / a[k][k];
            for j in k..5 {
                a[i][j] -= f * a[k][j];
            }
        }
    }
    let mut x = [0.0; 4];
    for k in (0..4).rev() {
        let mut s = a[k][4];
        for j in (k + 1)..4 {
            s -= a[k][j] * x[j];
        }
        x[k] = s / a[k][k];
    }
    x
}
