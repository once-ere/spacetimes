//! Machine checks of the geometry of the deflating spacetimes, and their geodesics.
//!
//! The exact checks (G1..G7) evaluate the definitions of differential geometry with
//! rational numbers at one point and compare the results with the closed forms that the
//! documents quote (see `metric.rs`).  Because the deflating spacetime is homogeneous
//! (translations and the scaling isometry carry any point to any other, check G6), one
//! point is as good as any other; the checks are nevertheless repeated for five
//! different sets of rational rates and scale factors.
//!
//! The numerical checks (G8, G9) integrate the geodesic equation
//!
//!     d^2 x^mu / d lam^2 + Gamma^mu_{nu rho} (dx^nu/d lam) (dx^rho/d lam) = 0
//!
//! and compare with the first integrals.  With the conserved covariant momenta
//! P_a = eta_aa dx^a/d lam (a = 0..3), P_t = eta_tt b_t^2 dx^t/d lam (t = 5, 6, 7) and the
//! conserved norm N = g_{mu nu} (dx^mu/d lam)(dx^nu/d lam),
//!
//!     (dx4/d lam)^2 = eta_44 [ N - sum_a eta_aa P_a^2 - sum_t eta_tt P_t^2 / b_t(x4)^2 ].
//!
//!   7+1 (N = M^2 > 0):  (dx4/d lam)^2 = M^2 + |P|^2 + sum_t P_t^2 e^{2 rate_t x4}/const_t^2 :
//!        the energy of a particle with momentum along a deflating coordinate grows without
//!        bound, and x4 -> infinity is reached at a finite value of the affine parameter.
//!   4+4 (N = -M^2 < 0): (dx4/d lam)^2 = M^2 + |P|^2 - sum_t P_t^2 e^{2 rate_t x4}/const_t^2 :
//!        a particle with momentum along a deflating time cannot pass the value of x4 at
//!        which the right-hand side vanishes; it turns back.
//!
//! For one non-zero P_t (E_perp^2 = M^2 + |P|^2, the energy without the motion along xt) the affine
//! parameter that elapses from x4 = x4_0 is, in closed form,
//!     7+1, to x4 = infinity:        arcsinh( E_perp const_t e^{-rate x4_0} / |P_t| ) / (rate E_perp),
//!     4+4, to the turning point:    arccosh( E_perp const_t e^{-rate x4_0} / |P_t| ) / (rate E_perp).

use crate::curved::{parameter_sets, parameter_sets_text, Curved};
use crate::metric::{killing_count, killing_vectors, lie_derivative_of_metric, Point, Sig};
use crate::rat::Rat;
use spacetimes::verify::Report;

/// The exact checks G1..G7 for one spacetime.  Identifiers 44d-G.. and 71d-G...
pub fn verify_geometry(sig: Sig, rep: &mut Report) {
    let pre = match sig {
        Sig::S1644 => "44d",
        Sig::S1671 => "71d",
    };
    let tag = sig.tag();
    let sets = parameter_sets();
    let (mut g1, mut g2, mut g3, mut g4, mut g5, mut g6, mut g7) = (true, true, true, true, true, true, true);
    let mut counts: Vec<usize> = Vec::new();
    for (rate, b) in &sets {
        let p = Point { sig, rate: *rate, b: *b };
        let g = p.metric();
        let e = p.vielbein();
        // G1: g_{mu mu} = eta_mu e_mu^2 and d g_tt/dx4 = -2 rate_t g_tt
        for a in 0..8 {
            g1 &= g[a].v == p.eta(a) * e[a] * e[a];
        }
        for i in 0..3 {
            g1 &= g[5 + i].d1 == -Rat::int(2) * rate[i] * g[5 + i].v && g[5 + i].v == -b[i] * b[i];
        }
        // G2: Christoffel symbols
        let chr = p.christoffel();
        let chr_c = p.christoffel_closed();
        for rho in 0..8 {
            for mu in 0..8 {
                for nu in 0..8 {
                    g2 &= chr[rho][mu][nu].v == chr_c[rho][mu][nu];
                }
            }
        }
        // G3: the two structure equations of Cartan, with the spin-connection coefficients of the tetrad postulate.
        //     omega_mu^A_B = eta^AA omega_{mu A B};  d_4 of every non-zero coefficient is -rate_t times itself.
        let wl = Curved::spin_coefficients(&p);
        let w = |mu: usize, a: usize, bb: usize| -> Rat { p.eta(a) * wl[mu][a][bb] };
        let dw = |lam: usize, mu: usize, a: usize, bb: usize| -> Rat {
            if lam == 4 && mu >= 5 {
                -rate[mu - 5] * w(mu, a, bb)
            } else {
                Rat::ZERO
            }
        };
        // vielbein e^A_nu = delta^A_nu e_A, d_4 e^t_t = -rate_t b_t
        let ev = |a: usize, nu: usize| -> Rat { if a == nu { e[a] } else { Rat::ZERO } };
        let dev = |lam: usize, a: usize, nu: usize| -> Rat {
            if lam == 4 && a == nu && a >= 5 {
                -rate[a - 5] * e[a]
            } else {
                Rat::ZERO
            }
        };
        let riem = p.riemann();
        for a in 0..8 {
            for mu in 0..8 {
                for nu in 0..8 {
                    // torsion T^A_{mu nu} = d_mu e^A_nu - d_nu e^A_mu + omega_mu^A_B e^B_nu - omega_nu^A_B e^B_mu
                    let mut t = dev(mu, a, nu) - dev(nu, a, mu);
                    for bb in 0..8 {
                        t = t + w(mu, a, bb) * ev(bb, nu) - w(nu, a, bb) * ev(bb, mu);
                    }
                    g3 &= t.is_zero();
                    for bb in 0..8 {
                        // curvature 2-form: d_mu omega_nu^A_B - d_nu omega_mu^A_B + omega_mu^A_C omega_nu^C_B - omega_nu^A_C omega_mu^C_B
                        let mut x = dw(mu, nu, a, bb) - dw(nu, mu, a, bb);
                        for c in 0..8 {
                            x = x + w(mu, a, c) * w(nu, c, bb) - w(nu, a, c) * w(mu, c, bb);
                        }
                        // = e^A_rho e_B^sigma R^rho_{sigma mu nu} = (e_A/e_B) R^A_{B mu nu}
                        g3 &= x == e[a] * riem[a][bb][mu][nu] / e[bb];
                    }
                }
            }
        }
        // G4: Riemann tensor from the Christoffel symbols equals the closed form
        g4 &= riem == p.riemann_closed();
        // G5: Ricci tensor, curvature scalar, Einstein tensor, contracted Bianchi identity
        let ric = p.ricci(&riem);
        let (ric_c, r_c, ein_c) = p.curvature_closed();
        let ein = p.einstein_mixed(&ric);
        g5 &= p.scalar(&ric) == r_c;
        for a in 0..8 {
            for bb in 0..8 {
                if a == bb {
                    g5 &= ric[a][a] / g[a].v == ric_c[a] && ein[a][a] == ein_c[a];
                } else {
                    g5 &= ric[a][bb].is_zero() && ein[a][bb].is_zero();
                }
            }
        }
        g5 &= p.divergence_of_diagonal(&ein_c).iter().all(|x| x.is_zero());
        // G6: Killing vectors, at a point with rational coordinates
        let x = [Rat::new(1, 3), Rat::new(-2, 1), Rat::new(5, 7), Rat::new(1, 2), Rat::ZERO, Rat::new(3, 4), Rat::new(-1, 5), Rat::new(2, 3)];
        let ks = killing_vectors(&p, &x);
        g6 &= ks.len() == killing_count(rate);
        counts.push(ks.len());
        for v in &ks {
            g6 &= lie_derivative_of_metric(&p, v).iter().all(|row| row.iter().all(|c| c.is_zero()));
        }
        // G7: constant curvature of the part (x4, x5, x6, x7) if the three rates are equal
        if rate[0] == rate[1] && rate[1] == rate[2] {
            let kk = -p.eta(4) * rate[0] * rate[0];
            for rho in 4..8 {
                for sigma in 4..8 {
                    for mu in 4..8 {
                        for nu in 4..8 {
                            let d = |a: usize, c: usize| if a == c { g[a].v } else { Rat::ZERO };
                            g7 &= g[rho].v * riem[rho][sigma][mu][nu] == kk * (d(rho, mu) * d(sigma, nu) - d(rho, nu) * d(sigma, mu));
                        }
                    }
                }
            }
        }
    }
    let sets_txt = parameter_sets_text();
    let eta = sig.eta();
    rep.add(&format!("{}-G1", pre), "exact", &format!("metric of the deflating {} spacetime: g_{tag} = diag(eta_00, .., eta_44, eta_55 b_5^2, eta_66 b_6^2, eta_77 b_7^2) with eta = diag{:?} and the scale factors b_5 = const5 exp(-g55 x4), b_6 = const6 exp(-g66 x4), b_7 = const7 exp(-g77 x4), that is g_55 = -const5^2 exp(-2 g55 x4) and likewise g_66, g_77 (the author's formulas; g55, g66, g77 > 0 are the three rates, and in statements with a general index t = 5, 6, 7 the rate of xt is written rate_t, gsum = g55 + g66 + g77). Verified: g_tt = -b_t^2 and d g_tt/dx4 = -2 rate_t g_tt; the frame e^A = (dx0, .., dx4, b_5 dx5, b_6 dx6, b_7 dx7) is orthonormal. All exact geometry checks use five rational parameter sets: {}", sig.name(), eta, sets_txt), g1);
    rep.add(&format!("{}-G2", pre), "exact", "the Christoffel symbols computed from their definition are Gamma^t_(4t) = Gamma^t_(t4) = -rate_t and Gamma^4_(tt) = eta_44 eta_tt rate_t b_t^2 (t = 5, 6, 7); all others vanish", g2);
    rep.add(&format!("{}-G3", pre), "exact", "Cartan's structure equations hold with the spin-connection coefficients of the tetrad postulate: the torsion d e^A + omega^A_B ^ e^B vanishes, and d omega^A_B + omega^A_C ^ omega^C_B equals the Riemann tensor computed from the Christoffel symbols (two independent routes to the curvature)", g3);
    rep.add(&format!("{}-G4", pre), "exact", "Riemann tensor in the orthonormal frame: R^t_(4t4) = -rate_t^2, R^4_(t4t) = -eta_44 eta_tt rate_t^2, R^t_(sts) = -eta_44 eta_ss rate_t rate_s (t != s in {5, 6, 7}), and the components that follow from antisymmetry; all other components vanish. No component involves x0, x1, x2 or x3: the space of these four coordinates is flat", g4);
    rep.add(&format!("{}-G5", pre), "exact", "with gsq = g55^2 + g66^2 + g77^2 and gsum = g55 + g66 + g77: Ricci tensor R^4_4 = -eta_44 gsq, R^t_t = -eta_44 rate_t gsum, R^a_a = 0 (a = 0..3); curvature scalar R = -eta_44 (gsq + gsum^2); Einstein tensor G^4_4 = eta_44 (g55 g66 + g55 g77 + g66 g77), G^t_t = eta_44 [(gsq + gsum^2)/2 - rate_t gsum], G^a_a = eta_44 (gsq + gsum^2)/2; its covariant divergence vanishes. The Einstein tensor does not vanish: the deflating spacetime is not a vacuum solution of Einstein's equations", g5);
    rep.add(&format!("{}-G6", pre), "exact", &format!("Killing vectors (the Lie derivative of the metric along each vanishes identically): 7 translations (x0, x1, x2, x3, x5, x6, x7), 6 rotations of (x0, x1, x2, x3), and the scaling d/dx4 + sum_t rate_t xt d/dxt, 14 in all; one more for every pair of equal rates (a rotation of the two coordinates); and 20 in all if the three rates are equal. Numbers found for the five parameter sets: {:?}", counts), g6);
    let cc = match sig {
        Sig::S1644 => "if the three rates are equal, g55 = g66 = g77, the part (x4, x5, x6, x7) of the metric has constant curvature, R_(rho sigma mu nu) = +g55^2 (g_(rho mu) g_(sigma nu) - g_(rho nu) g_(sigma mu)): the four time-like coordinates carry minus the metric of hyperbolic space H^4 of radius 1/g55, written in horospherical coordinates",
        Sig::S1671 => "if the three rates are equal, g55 = g66 = g77, the part (x4, x5, x6, x7) of the metric has constant curvature, R_(rho sigma mu nu) = -g55^2 (g_(rho mu) g_(sigma nu) - g_(rho nu) g_(sigma mu)): it is de Sitter space of radius 1/g55 in the contracting half of its flat slicing",
    };
    rep.add(&format!("{}-G7", pre), "exact", cc, g7);
}

/// State of a geodesic: position and velocity with respect to the affine parameter.
#[derive(Clone, Copy, Debug)]
pub struct GeoState {
    pub x: [f64; 8],
    pub u: [f64; 8],
}

/// Parameters of a geodesic calculation in floating point.
#[derive(Clone, Copy, Debug)]
pub struct GeoSpace {
    pub sig: Sig,
    pub rate: [f64; 3],
    pub b0: [f64; 3],
}

impl GeoSpace {
    pub fn eta(&self, a: usize) -> f64 {
        self.sig.eta()[a] as f64
    }
    pub fn b(&self, i: usize, x4: f64) -> f64 {
        self.b0[i] * (-self.rate[i] * x4).exp()
    }
    /// Right-hand side of the geodesic equation with the closed-form Christoffel symbols:
    /// du^4 = -sum_t Gamma^4_tt (u^t)^2,  du^t = -2 Gamma^t_4t u^4 u^t = 2 rate_t u^4 u^t,  du^a = 0.
    fn rhs(&self, s: &GeoState) -> GeoState {
        let mut dx = [0.0; 8];
        let mut du = [0.0; 8];
        for a in 0..8 {
            dx[a] = s.u[a];
        }
        for i in 0..3 {
            let t = 5 + i;
            let b = self.b(i, s.x[4]);
            du[4] -= self.eta(4) * self.eta(t) * self.rate[i] * b * b * s.u[t] * s.u[t];
            du[t] = 2.0 * self.rate[i] * s.u[4] * s.u[t];
        }
        GeoState { x: dx, u: du }
    }
    /// One classical Runge-Kutta step of size h in the affine parameter.
    pub fn step(&self, s: &GeoState, h: f64) -> GeoState {
        let add = |a: &GeoState, k: &GeoState, f: f64| -> GeoState {
            let mut r = *a;
            for i in 0..8 {
                r.x[i] += f * k.x[i];
                r.u[i] += f * k.u[i];
            }
            r
        };
        let k1 = self.rhs(s);
        let k2 = self.rhs(&add(s, &k1, 0.5 * h));
        let k3 = self.rhs(&add(s, &k2, 0.5 * h));
        let k4 = self.rhs(&add(s, &k3, h));
        let mut r = *s;
        for i in 0..8 {
            r.x[i] += h / 6.0 * (k1.x[i] + 2.0 * k2.x[i] + 2.0 * k3.x[i] + k4.x[i]);
            r.u[i] += h / 6.0 * (k1.u[i] + 2.0 * k2.u[i] + 2.0 * k3.u[i] + k4.u[i]);
        }
        r
    }
    /// The conserved covariant momenta P_mu = g_{mu mu} u^mu for mu != 4, and the norm N.
    pub fn invariants(&self, s: &GeoState) -> ([f64; 8], f64) {
        let mut p = [0.0; 8];
        let mut n = 0.0;
        for a in 0..8 {
            let g = if a >= 5 { self.eta(a) * self.b(a - 5, s.x[4]).powi(2) } else { self.eta(a) };
            p[a] = g * s.u[a];
            n += g * s.u[a] * s.u[a];
        }
        (p, n)
    }
}

/// Result of the numerical geodesic checks of one spacetime.
pub struct GeoResult {
    /// largest relative change of a conserved quantity along the path
    pub drift: f64,
    /// (affine parameter elapsed, closed form) for the characteristic event:
    /// 7+1: reaching the largest x4 of the run (compared with the closed form to that x4);
    /// 4+4: reaching the turning point
    pub lam: f64,
    pub lam_closed: f64,
    /// 4+4: the largest x4 reached, and the predicted turning value
    pub x4_max: f64,
    pub x4_turn: f64,
    /// the sampled path (lam, x4, proper energy-like quantity |u^4|)
    pub path: Vec<[f64; 3]>,
}

/// Integrate one geodesic with momentum along x1 and along the deflating coordinate x5.
pub fn geodesic_run(sig: Sig, rate: f64, b0: f64, m: f64, p1: f64, p5: f64, x4_start: f64) -> GeoResult {
    let sp = GeoSpace { sig, rate: [rate, 0.7 * rate, 1.3 * rate], b0: [b0, 1.0, 1.0] };
    let e0 = (m * m + p1 * p1).sqrt();
    let q = p5.abs() / b0;
    // initial velocity from the first integrals: u^1 = P_1/eta_11, u^5 = P_5/(eta_55 b^2), u^4 from the norm
    let b = sp.b(0, x4_start);
    let mut s = GeoState { x: [0.0; 8], u: [0.0; 8] };
    s.x[4] = x4_start;
    s.u[1] = p1 / sp.eta(1);
    s.u[5] = p5 / (sp.eta(5) * b * b);
    let u4sq = match sig {
        Sig::S1671 => e0 * e0 + q * q * (2.0 * rate * x4_start).exp(),
        Sig::S1644 => e0 * e0 - q * q * (2.0 * rate * x4_start).exp(),
    };
    assert!(u4sq > 0.0, "geodesic_run: the starting point lies beyond the turning point");
    s.u[4] = u4sq.sqrt();
    let (p_start, n_start) = sp.invariants(&s);
    let mut drift: f64 = 0.0;
    let mut path: Vec<[f64; 3]> = vec![[0.0, s.x[4], s.u[4]]];
    let mut lam = 0.0;
    let mut x4_max = s.x[4];
    let mut lam_event = f64::NAN;
    let x4_turn = (e0 / q).ln() / rate;
    match sig {
        Sig::S1671 => {
            // step in lam with h proportional to 1/u^4, so that x4 advances by about 1e-3 per step, up to x4_start + 6/rate
            let x4_end = x4_start + 6.0 / rate;
            while s.x[4] < x4_end {
                let h = (1e-3 / rate / s.u[4]).min((x4_end - s.x[4]) / s.u[4] * 1.0000001 + 1e-300);
                s = sp.step(&s, h);
                lam += h;
                let (pp, n) = sp.invariants(&s);
                drift = drift.max(((pp[1] - p_start[1]) / p_start[1]).abs()).max(((pp[5] - p_start[5]) / p_start[5]).abs()).max(((n - n_start) / n_start).abs());
                if path.len() < 20000 {
                    path.push([lam, s.x[4], s.u[4]]);
                }
            }
            lam_event = lam;
            x4_max = s.x[4];
            // closed form from x4_start to x4_max: [asinh(E_perp e^{-rate x4_start}/q) - asinh(E_perp e^{-rate x4_max}/q)] / (rate E_perp), q = |P_t|/const_t
            let f = |x4: f64| (e0 * (-rate * x4).exp() / q).asinh() / (rate * e0);
            let lam_closed = f(x4_start) - f(x4_max);
            GeoResult { drift, lam: lam_event, lam_closed, x4_max, x4_turn: f64::INFINITY, path }
        }
        Sig::S1644 => {
            // fixed small steps in lam; follow the path through the turning point and back to x4_start
            let lam_turn = (e0 * (-rate * x4_start).exp() / q).acosh() / (rate * e0);
            let h = lam_turn / 20000.0;
            let mut passed = false;
            for _ in 0..40000 {
                let prev_u4 = s.u[4];
                s = sp.step(&s, h);
                lam += h;
                let (pp, n) = sp.invariants(&s);
                drift = drift.max(((pp[1] - p_start[1]) / p_start[1]).abs()).max(((pp[5] - p_start[5]) / p_start[5]).abs()).max(((n - n_start) / n_start).abs());
                x4_max = x4_max.max(s.x[4]);
                if !passed && prev_u4 > 0.0 && s.u[4] <= 0.0 {
                    // linear interpolation of the zero of u^4
                    lam_event = lam - h * (-s.u[4]) / (prev_u4 - s.u[4]);
                    passed = true;
                }
                if path.len() < 20000 || passed {
                    path.push([lam, s.x[4], s.u[4]]);
                }
            }
            GeoResult { drift, lam: lam_event, lam_closed: lam_turn, x4_max, x4_turn, path }
        }
    }
}

/// The numerical checks G8 (7+1) and G9 (4+4) of the geodesics.
pub fn verify_geodesics(sig: Sig, rep: &mut Report) -> GeoResult {
    match sig {
        Sig::S1671 => {
            let r = geodesic_run(sig, 1.0, 0.8, 1.0, 0.5, 0.3, -2.0);
            let ok = r.drift < 1e-9 && ((r.lam - r.lam_closed) / r.lam_closed).abs() < 1e-8;
            let total = (((1.0f64 + 0.25).sqrt() * (2.0f64).exp() / (0.3 / 0.8)).asinh()) / (1.25f64).sqrt();
            rep.add("71d-G8", "numerical", &format!("geodesic of a particle of mass m = 1 with momentum P_1 = 0.5 along x1 and P_5 = 0.3 along the deflating coordinate x5 (g55 = 1, const5 = 0.8), integrated from x4 = -2 to x4 = 4 by the Runge-Kutta method: P_1, P_5 and the norm are conserved to {:.1e}; the affine parameter (the proper time, since m = 1) that elapses is {:.10}, the closed form gives {:.10}; the proper time to x4 = infinity is finite, {:.6}: a particle that moves along a deflating coordinate leaves the coordinate patch after a finite proper time, with unbounded energy", r.drift, r.lam, r.lam_closed, total), ok);
            r
        }
        Sig::S1644 => {
            let r = geodesic_run(sig, 1.0, 0.8, 1.0, 0.5, 0.3, -2.0);
            let ok = r.drift < 1e-9 && ((r.lam - r.lam_closed) / r.lam_closed).abs() < 1e-6 && (r.x4_max - r.x4_turn).abs() < 1e-6;
            rep.add("44d-G9", "numerical", &format!("geodesic with m = 1, momentum P_1 = 0.5 along x1 and P_5 = 0.3 along the deflating time x5 (g55 = 1, const5 = 0.8), started at x4 = -2 with increasing x4: P_1, P_5 and the norm are conserved to {:.1e}; the path reaches the largest value x4 = {:.8} (predicted turning value ln(E_perp const5/|P_5|)/g55 with E_perp = sqrt(m^2 + P_1^2) = {:.8}) after the affine parameter {:.8} (closed form {:.8}) and then returns to smaller x4: a particle with momentum along a deflating time cannot pass the turning value", r.drift, r.x4_max, r.x4_turn, r.lam, r.lam_closed), ok);
            r
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_geometry_checks_pass_for_both_spacetimes() {
        let mut rep = Report::new();
        verify_geometry(Sig::S1644, &mut rep);
        verify_geometry(Sig::S1671, &mut rep);
        assert_eq!(rep.checks.len(), 14);
        for c in &rep.checks {
            assert!(c.pass, "check {} fails", c.id);
        }
    }

    #[test]
    fn geodesics_conserve_their_first_integrals_and_match_the_closed_forms() {
        let mut rep = Report::new();
        let r71 = verify_geodesics(Sig::S1671, &mut rep);
        let r44 = verify_geodesics(Sig::S1644, &mut rep);
        for c in &rep.checks {
            assert!(c.pass, "check {} fails: {}", c.id, c.statement);
        }
        // 7+1: the velocity dx4/dlam grows along the path; 4+4: it changes sign
        assert!(r71.path.last().unwrap()[2] > r71.path[0][2]);
        assert!(r44.path.last().unwrap()[2] < 0.0);
    }
}
