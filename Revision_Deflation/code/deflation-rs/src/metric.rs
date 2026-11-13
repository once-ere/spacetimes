//! Geometry of the two deflating spacetimes.
//!
//! THE HYPOTHESIS (author's instructions of November 12, 2026).  Minkowski spacetime is
//! replaced by a pseudo-Riemannian space with a diagonal metric in which
//!
//!     g_55 = -const5^2 exp(-2 g55 x4),   g_66 = -const6^2 exp(-2 g66 x4),   g_77 = -const7^2 exp(-2 g77 x4),
//!
//! with three positive rates g55, g66, g77 and three positive constants const5, const6,
//! const7, which may all differ; g_AA = eta_AA for the other five coordinates x0, x1,
//! x2, x3, x4.  The names g55, g66, g77 and "const" are the author's and are used
//! unchanged: g55 is the rate in the exponent of the metric component g_55 (written
//! with an underscore), not the component itself.
//!
//! NOTATION OF THE PROGRAM.  The scale factors are
//!
//!     b_5(x4) = const5 exp(-g55 x4),   b_6(x4) = const6 exp(-g66 x4),   b_7(x4) = const7 exp(-g77 x4),
//!
//! so that g_tt = eta_tt b_t(x4)^2 for t = 5, 6, 7.  Where a formula holds for all three
//! deflating coordinates it is written with a general index t and with rate_t, which
//! stands for g55, g66, g77 when t = 5, 6, 7 (and const_t for const5, const6, const7);
//! gsum = g55 + g66 + g77.  A prime is the derivative with respect to x4:
//! b_t' = -rate_t b_t.
//!
//! eta is the flat metric of the main project:
//!     4+4:  eta1644 = diag(+1,+1,+1,+1,-1,-1,-1,-1)   (x5, x6, x7 time-like),
//!     7+1:  eta1671 = diag(-1,-1,-1,-1,+1,-1,-1,-1)   (x5, x6, x7 space-like).
//! In both conventions eta_55 = eta_66 = eta_77 = -1.  For 4+4 this gives the formulas of
//! the author's instruction.  For 7+1 the author's convention writes a space-like
//! component with a plus sign; the real matrices of 7+1.nb satisfy the Clifford relations
//! with time-like = +1, so that in the convention of this project the space-like
//! components are negative.  The proper length along xt is b_t(x4) |dxt| in either.
//!
//! WHAT THIS MODULE COMPUTES.  At one point of the spacetime the geometry depends only
//! on the three rates and on the values b_5, b_6, b_7 of the scale factors at that
//! point, because d g_tt / d x4 = -2 rate_t g_tt.  Every routine is therefore a
//! function of (signature, rates, b).  It is generic in the kind of number: with exact
//! rationals (`Rat`) a check is a proof for those values of the parameters; with `f64`
//! the same code runs at any point.
//!
//!   * the metric and its first two derivatives with respect to x4;
//!   * the Christoffel symbols and their derivatives, from the definition;
//!   * the Riemann tensor, from the definition
//!         R^rho_{sigma mu nu} = d_mu Gamma^rho_{nu sigma} - d_nu Gamma^rho_{mu sigma}
//!                             + Gamma^rho_{mu lam} Gamma^lam_{nu sigma} - Gamma^rho_{nu lam} Gamma^lam_{mu sigma};
//!   * Ricci tensor, curvature scalar, Einstein tensor;
//!   * the closed forms of all of these, which the documents quote, for comparison;
//!   * the Killing vectors and the Lie derivative of the metric along them.
//!
//! INDICES.  Coordinates are numbered 0..7 as x0..x7.  The deflating coordinates are
//! t = 5, 6, 7; rate[t - 5] and b[t - 5] belong to xt.

use crate::rat::Field;
use spacetimes::gamma::{ETA1644, ETA1671};

/// The two spacetimes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sig {
    /// 4+4: x0..x3 space-like, x4..x7 time-like; metric convention "space-like = +1".
    S1644,
    /// 7+1: x4 time-like, the other seven space-like; metric convention "time-like = +1".
    S1671,
}

impl Sig {
    /// Tag of the deflating spacetime in names: "1644d" or "1671d".
    pub fn tag(self) -> &'static str {
        match self {
            Sig::S1644 => "1644d",
            Sig::S1671 => "1671d",
        }
    }
    /// Short name used in tables: "4+4" or "7+1".
    pub fn name(self) -> &'static str {
        match self {
            Sig::S1644 => "4+4",
            Sig::S1671 => "7+1",
        }
    }
    /// The flat metric eta_AA of the main project.
    pub fn eta(self) -> [i64; 8] {
        match self {
            Sig::S1644 => ETA1644,
            Sig::S1671 => ETA1671,
        }
    }
}

/// A function of x4 at one point, with its first two derivatives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Jet<F: Field> {
    pub v: F,
    pub d1: F,
    pub d2: F,
}

impl<F: Field> Jet<F> {
    pub fn constant(v: F) -> Jet<F> {
        Jet { v, d1: F::zero(), d2: F::zero() }
    }
    pub fn zero() -> Jet<F> {
        Jet::constant(F::zero())
    }
    pub fn add(self, o: Jet<F>) -> Jet<F> {
        Jet { v: self.v + o.v, d1: self.d1 + o.d1, d2: self.d2 + o.d2 }
    }
    pub fn sub(self, o: Jet<F>) -> Jet<F> {
        Jet { v: self.v - o.v, d1: self.d1 - o.d1, d2: self.d2 - o.d2 }
    }
    /// Product rule: (uv)' = u'v + uv', (uv)'' = u''v + 2u'v' + uv''.
    pub fn mul(self, o: Jet<F>) -> Jet<F> {
        let two = F::from_int(2);
        Jet { v: self.v * o.v, d1: self.d1 * o.v + self.v * o.d1, d2: self.d2 * o.v + two * self.d1 * o.d1 + self.v * o.d2 }
    }
    pub fn scale(self, c: F) -> Jet<F> {
        Jet { v: c * self.v, d1: c * self.d1, d2: c * self.d2 }
    }
    /// 1/u:  (1/u)' = -u'/u^2,  (1/u)'' = 2u'^2/u^3 - u''/u^2.
    pub fn recip(self) -> Jet<F> {
        let two = F::from_int(2);
        let u2 = self.v * self.v;
        Jet { v: F::one() / self.v, d1: -self.d1 / u2, d2: two * self.d1 * self.d1 / (u2 * self.v) - self.d2 / u2 }
    }
    /// The derivative with respect to x4 (the second derivative of the result is not
    /// known and is set to zero; it is never used).
    pub fn derivative(self) -> Jet<F> {
        Jet { v: self.d1, d1: self.d2, d2: F::zero() }
    }
}

/// The geometry at one point: signature, rates, and the values of the scale factors there.
#[derive(Clone, Copy, Debug)]
pub struct Point<F: Field> {
    pub sig: Sig,
    /// the three rates g55, g66, g77 of the author's instruction
    pub rate: [F; 3],
    /// b_5, b_6, b_7 at the point
    pub b: [F; 3],
}

/// All components of a tensor with three indices, T[rho][mu][nu].
pub type T3<F> = [[[F; 8]; 8]; 8];

impl<F: Field> Point<F> {
    pub fn eta(&self, a: usize) -> F {
        F::from_int(self.sig.eta()[a])
    }

    /// The sum of the three rates, gsum.
    pub fn rate_sum(&self) -> F {
        self.rate[0] + self.rate[1] + self.rate[2]
    }

    /// The metric components g_AA with their first two derivatives with respect to x4:
    /// g_tt = eta_tt b_t^2, d g_tt/dx4 = -2 rate_t g_tt, d^2 g_tt/dx4^2 = 4 rate_t^2 g_tt.
    pub fn metric(&self) -> [Jet<F>; 8] {
        let mut g = [Jet::zero(); 8];
        for a in 0..8 {
            g[a] = Jet::constant(self.eta(a));
        }
        for i in 0..3 {
            let t = 5 + i;
            let v = self.eta(t) * self.b[i] * self.b[i];
            let k = self.rate[i];
            g[t] = Jet { v, d1: -F::from_int(2) * k * v, d2: F::from_int(4) * k * k * v };
        }
        g
    }

    /// The vielbein factors e^A_A: 1 for x0..x4 and b_t for the deflating coordinates.
    pub fn vielbein(&self) -> [F; 8] {
        let mut e = [F::one(); 8];
        for i in 0..3 {
            e[5 + i] = self.b[i];
        }
        e
    }

    /// Christoffel symbols Gamma^rho_{mu nu} with their derivative with respect to x4,
    /// from the definition  Gamma^rho_{mu nu} = (1/2) g^{rho rho} (d_mu g_{rho nu} + d_nu g_{rho mu} - d_rho g_{mu nu})
    /// for a diagonal metric that depends on x4 only.
    pub fn christoffel(&self) -> [[[Jet<F>; 8]; 8]; 8] {
        let g = self.metric();
        let half = F::one() / F::from_int(2);
        // d_mu of the component g_{alpha beta}: non-zero only for mu = 4 and alpha = beta
        let d = |mu: usize, alpha: usize, beta: usize| -> Jet<F> {
            if mu == 4 && alpha == beta {
                g[alpha].derivative()
            } else {
                Jet::zero()
            }
        };
        let mut gam = [[[Jet::zero(); 8]; 8]; 8];
        for rho in 0..8 {
            let ginv = g[rho].recip();
            for mu in 0..8 {
                for nu in 0..8 {
                    let s = d(mu, rho, nu).add(d(nu, rho, mu)).sub(d(rho, mu, nu));
                    // only the value and the first derivative of the product are needed
                    gam[rho][mu][nu] = Jet { v: half * ginv.v * s.v, d1: half * (ginv.d1 * s.v + ginv.v * s.d1), d2: F::zero() };
                }
            }
        }
        gam
    }

    /// The closed forms of the Christoffel symbols:
    /// Gamma^t_{4t} = Gamma^t_{t4} = -rate_t,  Gamma^4_{tt} = eta_44 eta_tt rate_t b_t^2;  all others vanish.
    pub fn christoffel_closed(&self) -> T3<F> {
        let mut gam = [[[F::zero(); 8]; 8]; 8];
        for i in 0..3 {
            let t = 5 + i;
            gam[t][4][t] = -self.rate[i];
            gam[t][t][4] = -self.rate[i];
            gam[4][t][t] = self.eta(4) * self.eta(t) * self.rate[i] * self.b[i] * self.b[i];
        }
        gam
    }

    /// The Riemann tensor R^rho_{sigma mu nu} (coordinate components) from the definition.
    pub fn riemann(&self) -> [[[[F; 8]; 8]; 8]; 8] {
        let gam = self.christoffel();
        // d_mu Gamma^rho_{nu sigma} is non-zero only for mu = 4
        let dgam = |mu: usize, rho: usize, nu: usize, sigma: usize| -> F {
            if mu == 4 {
                gam[rho][nu][sigma].d1
            } else {
                F::zero()
            }
        };
        let mut r = [[[[F::zero(); 8]; 8]; 8]; 8];
        for rho in 0..8 {
            for sigma in 0..8 {
                for mu in 0..8 {
                    for nu in 0..8 {
                        let mut x = dgam(mu, rho, nu, sigma) - dgam(nu, rho, mu, sigma);
                        for lam in 0..8 {
                            x = x + gam[rho][mu][lam].v * gam[lam][nu][sigma].v - gam[rho][nu][lam].v * gam[lam][mu][sigma].v;
                        }
                        r[rho][sigma][mu][nu] = x;
                    }
                }
            }
        }
        r
    }

    /// The closed form of the Riemann tensor, in the orthonormal frame e^A = (dx0..dx4, b_t dxt):
    ///
    ///     R^t_{4 t 4} = -rate_t^2,                 R^4_{t 4 t} = -eta_44 eta_tt rate_t^2,
    ///     R^t_{s t s} = -eta_44 eta_ss rate_t rate_s      (t != s both deflating),
    ///
    /// together with the components that follow from the antisymmetry in the last two
    /// indices; all other components vanish.  The coordinate components are obtained
    /// with the vielbein factors: R^rho_{sigma mu nu} = (e_sigma e_mu e_nu / e_rho) R^A_{BCD}.
    pub fn riemann_closed(&self) -> [[[[F; 8]; 8]; 8]; 8] {
        let e = self.vielbein();
        let mut f = [[[[F::zero(); 8]; 8]; 8]; 8]; // frame components
        let e44 = self.eta(4);
        for i in 0..3 {
            let t = 5 + i;
            let k2 = self.rate[i] * self.rate[i];
            f[t][4][t][4] = -k2;
            f[t][4][4][t] = k2;
            f[4][t][4][t] = -e44 * self.eta(t) * k2;
            f[4][t][t][4] = e44 * self.eta(t) * k2;
            for j in 0..3 {
                if j != i {
                    let s = 5 + j;
                    let x = -e44 * self.eta(s) * self.rate[i] * self.rate[j];
                    f[t][s][t][s] = x;
                    f[t][s][s][t] = -x;
                }
            }
        }
        let mut r = [[[[F::zero(); 8]; 8]; 8]; 8];
        for rho in 0..8 {
            for sigma in 0..8 {
                for mu in 0..8 {
                    for nu in 0..8 {
                        r[rho][sigma][mu][nu] = f[rho][sigma][mu][nu] * e[sigma] * e[mu] * e[nu] / e[rho];
                    }
                }
            }
        }
        r
    }

    /// Ricci tensor R_{sigma nu} = R^rho_{sigma rho nu} from a Riemann tensor.
    pub fn ricci(&self, riem: &[[[[F; 8]; 8]; 8]; 8]) -> [[F; 8]; 8] {
        let mut ric = [[F::zero(); 8]; 8];
        for sigma in 0..8 {
            for nu in 0..8 {
                let mut x = F::zero();
                for rho in 0..8 {
                    x = x + riem[rho][sigma][rho][nu];
                }
                ric[sigma][nu] = x;
            }
        }
        ric
    }

    /// Curvature scalar R = g^{sigma nu} R_{sigma nu}.
    pub fn scalar(&self, ric: &[[F; 8]; 8]) -> F {
        let g = self.metric();
        let mut x = F::zero();
        for a in 0..8 {
            x = x + ric[a][a] / g[a].v;
        }
        x
    }

    /// Mixed Einstein tensor G^mu_nu = R^mu_nu - (1/2) delta^mu_nu R.
    pub fn einstein_mixed(&self, ric: &[[F; 8]; 8]) -> [[F; 8]; 8] {
        let g = self.metric();
        let r = self.scalar(ric);
        let half = F::one() / F::from_int(2);
        let mut e = [[F::zero(); 8]; 8];
        for mu in 0..8 {
            for nu in 0..8 {
                e[mu][nu] = ric[mu][nu] / g[mu].v;
                if mu == nu {
                    e[mu][nu] = e[mu][nu] - half * r;
                }
            }
        }
        e
    }

    /// Closed forms (gsq = sum of the squares of the rates, gsum = their sum):
    ///
    ///     R^4_4 = -eta_44 gsq,     R^t_t = -eta_44 rate_t gsum,     R^a_a = 0 (a = 0..3),
    ///     R     = -eta_44 (gsq + gsum^2),
    ///     G^4_4 = (eta_44/2)(gsum^2 - gsq) = eta_44 (g55 g66 + g55 g77 + g66 g77),
    ///     G^t_t = eta_44 [ (gsq + gsum^2)/2 - rate_t gsum ],
    ///     G^a_a = (eta_44/2)(gsq + gsum^2)   (a = 0..3).
    ///
    /// Returns (diagonal of the mixed Ricci tensor, curvature scalar, diagonal of the mixed Einstein tensor).
    pub fn curvature_closed(&self) -> ([F; 8], F, [F; 8]) {
        let e44 = self.eta(4);
        let vs = self.rate_sum();
        let s2 = self.rate[0] * self.rate[0] + self.rate[1] * self.rate[1] + self.rate[2] * self.rate[2];
        let half = F::one() / F::from_int(2);
        let mut ric = [F::zero(); 8];
        ric[4] = -e44 * s2;
        for i in 0..3 {
            ric[5 + i] = -e44 * self.rate[i] * vs;
        }
        let r = -e44 * (s2 + vs * vs);
        let mut ein = [F::zero(); 8];
        for a in 0..8 {
            ein[a] = ric[a] - half * r;
        }
        (ric, r, ein)
    }

    /// The covariant divergence of a diagonal, constant mixed tensor E^mu_nu = diag(e):
    /// (nabla_mu E^mu_nu) = Gamma^mu_{mu lam} E^lam_nu - Gamma^lam_{mu nu} E^mu_lam.
    /// For the Einstein tensor it must vanish (contracted Bianchi identity).
    pub fn divergence_of_diagonal(&self, e: &[F; 8]) -> [F; 8] {
        let gam = self.christoffel();
        let mut out = [F::zero(); 8];
        for nu in 0..8 {
            let mut x = F::zero();
            for mu in 0..8 {
                x = x + gam[mu][mu][nu].v * e[nu];
                for lam in 0..8 {
                    if lam == mu {
                        x = x - gam[lam][mu][nu].v * e[mu];
                    }
                }
            }
            out[nu] = x;
        }
        out
    }
}

/// A vector field xi^mu(x) together with its matrix of partial derivatives
/// dxi[mu][lam] = d_mu xi^lam, both evaluated at one point.
pub struct VectorAt<F: Field> {
    pub name: String,
    pub xi: [F; 8],
    pub dxi: [[F; 8]; 8],
}

/// The Lie derivative of the metric along xi at a point,
/// (L_xi g)_{mu nu} = xi^lam d_lam g_{mu nu} + g_{lam nu} d_mu xi^lam + g_{mu lam} d_nu xi^lam.
/// xi is a Killing vector if and only if all 64 components vanish.
pub fn lie_derivative_of_metric<F: Field>(p: &Point<F>, v: &VectorAt<F>) -> [[F; 8]; 8] {
    let g = p.metric();
    let mut l = [[F::zero(); 8]; 8];
    for mu in 0..8 {
        for nu in 0..8 {
            let mut x = F::zero();
            if mu == nu {
                x = x + v.xi[4] * g[mu].d1; // the metric depends on x4 only
            }
            x = x + g[nu].v * v.dxi[mu][nu] + g[mu].v * v.dxi[nu][mu];
            l[mu][nu] = x;
        }
    }
    l
}

/// The Killing vectors of the deflating spacetime, evaluated at the point with the
/// coordinates `x`, at which the scale factors have the values `p.b`.
///
/// Always present (14 vectors):
///   * the seven translations d/dx0..d/dx3, d/dx5, d/dx6, d/dx7;
///   * the six rotations of the flat coordinates x0..x3:  xa d/dxb - xb d/dxa;
///   * the scaling  d/dx4 + sum_t rate_t xt d/dxt  (a shift of x4 combined with a
///     stretching of the deflating coordinates).
///
/// Present only for special rates:
///   * if rate_t = rate_s: the rotation of the pair (xt, xs).  With equal
///     rates the ratio b_t/b_s does not depend on x4, and in the coordinates
///     Y^t = b_t xt, Y^s = b_s xs the rotation is Y^t d/dY^s - Y^s d/dY^t, that is
///     (b_t xt / b_s) d/dxs - (b_s xs / b_t) d/dxt;
///   * if all three rates are equal (rate): three further vectors.
///
/// DERIVATION of the three further vectors.  Shift the origin of x4 to the point, so
/// that b_i(x4) = b_i e^{-rate x4} with the constants b_i = `p.b[i]`, and put
/// Y^i = b_i x^i (i = 5, 6, 7) and z = e^{rate x4}/rate.  The part of the metric
/// that contains x4, x5, x6, x7 becomes
///
///     eta_44 [ dx4^2 + sgn e^{-2 rate x4} dY.dY ] = eta_44 [ dz^2 + sgn dY.dY ] / (rate z)^2,
///     sgn = eta_tt / eta_44 = +1 (4+4),  -1 (7+1).
///
/// For sgn = +1 this is the upper-half-space form of hyperbolic space, for sgn = -1 the
/// conformally flat form of de Sitter space.  In both cases the special conformal
/// transformations of the flat metric dz^2 + sgn dY.dY whose parameter C points along
/// Y are isometries; their generators are
///
///     xi_C = 2 (C.Y) [ z d/dz + Y^i d/dY^i ] - ( Y.Y + sgn z^2 ) C^i d/dY^i ,     z d/dz = (1/rate) d/dx4 .
///
/// At the point itself x4 = 0 and z = 1/rate.  The components and the partial
/// derivatives used below follow by differentiation, with d z^2/dx4 = 2 rate z^2.
pub fn killing_vectors<F: Field>(p: &Point<F>, x: &[F; 8]) -> Vec<VectorAt<F>> {
    let mut out: Vec<VectorAt<F>> = Vec::new();
    let zero = F::zero();
    let one = F::one();
    let two = F::from_int(2);
    // translations
    for a in [0usize, 1, 2, 3, 5, 6, 7] {
        let mut xi = [zero; 8];
        xi[a] = one;
        out.push(VectorAt { name: format!("translation along x{}", a), xi, dxi: [[zero; 8]; 8] });
    }
    // rotations of the flat coordinates x0..x3
    for a in 0..4 {
        for b in (a + 1)..4 {
            let mut xi = [zero; 8];
            let mut dxi = [[zero; 8]; 8];
            xi[b] = x[a];
            xi[a] = -x[b];
            dxi[a][b] = one; // d_a xi^b
            dxi[b][a] = -one; // d_b xi^a
            out.push(VectorAt { name: format!("rotation of x{} and x{}", a, b), xi, dxi });
        }
    }
    // scaling: xi = d/dx4 + sum_t rate_t x^t d/dx^t
    {
        let mut xi = [zero; 8];
        let mut dxi = [[zero; 8]; 8];
        xi[4] = one;
        for i in 0..3 {
            xi[5 + i] = p.rate[i] * x[5 + i];
            dxi[5 + i][5 + i] = p.rate[i];
        }
        out.push(VectorAt { name: "scaling: shift of x4 with stretching of x5, x6, x7".to_string(), xi, dxi });
    }
    // rotations of two deflating coordinates with equal rates
    for i in 0..3 {
        for j in (i + 1)..3 {
            if p.rate[i] == p.rate[j] {
                let (t, s) = (5 + i, 5 + j);
                let mut xi = [zero; 8];
                let mut dxi = [[zero; 8]; 8];
                xi[s] = p.b[i] * x[t] / p.b[j];
                xi[t] = -p.b[j] * x[s] / p.b[i];
                dxi[t][s] = p.b[i] / p.b[j];
                dxi[s][t] = -p.b[j] / p.b[i];
                out.push(VectorAt { name: format!("rotation of x{} and x{} (equal rates)", t, s), xi, dxi });
            }
        }
    }
    // the three special vectors of the maximally symmetric case
    if p.rate[0] == p.rate[1] && p.rate[1] == p.rate[2] {
        let k = p.rate[0];
        let sgn = p.eta(5) / p.eta(4); // +1 for 4+4, -1 for 7+1
        let y: Vec<F> = (0..3).map(|i| p.b[i] * x[5 + i]).collect(); // Y^i = b_i x^i
        let yy = y[0] * y[0] + y[1] * y[1] + y[2] * y[2];
        let z2 = one / (k * k); // z^2 at the point
        for c in 0..3 {
            // C = unit vector number c
            let mut xi = [zero; 8];
            let mut dxi = [[zero; 8]; 8];
            // component along x4:  (2/rate) (C.Y) = (2/rate) Y^c ;  it does not depend on x4
            xi[4] = two * y[c] / k;
            dxi[5 + c][4] = two * p.b[c] / k;
            for i in 0..3 {
                let delta = if i == c { one } else { zero };
                // component along x^i:  [ 2 Y^c Y^i - (Y.Y + sgn z^2) delta_ic ] / b_i
                xi[5 + i] = (two * y[c] * y[i] - (yy + sgn * z2) * delta) / p.b[i];
                // derivative with respect to x^j:  dY^m/dx^j = b_j delta_jm
                for j in 0..3 {
                    let dj_yc = if j == c { p.b[j] } else { zero };
                    let dj_yi = if j == i { p.b[j] } else { zero };
                    let dj_yy = two * y[j] * p.b[j];
                    dxi[5 + j][5 + i] = (two * dj_yc * y[i] + two * y[c] * dj_yi - dj_yy * delta) / p.b[i];
                }
                // derivative with respect to x4: only z^2 depends on x4, d z^2/dx4 = 2 rate z^2 = 2/rate
                dxi[4][5 + i] = -sgn * (two / k) * delta / p.b[i];
            }
            out.push(VectorAt { name: format!("special isometry {} of the maximally symmetric case (all rates equal)", c + 1), xi, dxi });
        }
    }
    out
}

/// The number of Killing vectors that `killing_vectors` returns for given rates:
/// 14 if the three rates differ, 15 if exactly two are equal, 20 if all are equal.
pub fn killing_count<F: Field>(rate: &[F; 3]) -> usize {
    let e01 = rate[0] == rate[1];
    let e02 = rate[0] == rate[2];
    let e12 = rate[1] == rate[2];
    if e01 && e12 {
        20
    } else if e01 || e02 || e12 {
        15
    } else {
        14
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rat::Rat;

    fn r(n: i128, d: i128) -> Rat {
        Rat::new(n, d)
    }

    /// Parameter sets (rates; scale factors at the point), all rational.
    fn sets() -> Vec<([Rat; 3], [Rat; 3])> {
        vec![
            ([r(1, 1), r(1, 1), r(1, 1)], [r(1, 1), r(1, 1), r(1, 1)]),
            ([r(1, 1), r(2, 1), r(3, 1)], [r(1, 1), r(1, 1), r(1, 1)]),
            ([r(1, 2), r(3, 4), r(5, 3)], [r(2, 1), r(1, 3), r(5, 7)]),
            ([r(2, 1), r(2, 1), r(5, 1)], [r(3, 2), r(3, 2), r(1, 4)]),
            ([r(7, 3), r(7, 3), r(7, 3)], [r(1, 2), r(3, 1), r(5, 4)]),
        ]
    }

    #[test]
    fn jets_follow_the_product_and_quotient_rules() {
        // u = x^2 + 1, v = 3x at x = 2: u = (5, 4, 2), v = (6, 3, 0)
        let u = Jet { v: r(5, 1), d1: r(4, 1), d2: r(2, 1) };
        let v = Jet { v: r(6, 1), d1: r(3, 1), d2: r(0, 1) };
        // uv = 3x^3 + 3x: (30, 39, 36)
        assert_eq!(u.mul(v), Jet { v: r(30, 1), d1: r(39, 1), d2: r(36, 1) });
        // 1/u: -(2x)/(x^2+1)^2 = -4/25;  second derivative (6x^2-2)/(x^2+1)^3 = 22/125
        assert_eq!(u.recip(), Jet { v: r(1, 5), d1: r(-4, 25), d2: r(22, 125) });
    }

    #[test]
    fn christoffel_symbols_from_the_definition_equal_the_closed_forms() {
        for sig in [Sig::S1644, Sig::S1671] {
            for (rate, b) in sets() {
                let p = Point { sig, rate, b };
                let g = p.christoffel();
                let c = p.christoffel_closed();
                for rho in 0..8 {
                    for mu in 0..8 {
                        for nu in 0..8 {
                            assert_eq!(g[rho][mu][nu].v, c[rho][mu][nu]);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn riemann_tensor_from_the_definition_equals_the_closed_form() {
        for sig in [Sig::S1644, Sig::S1671] {
            for (rate, b) in sets() {
                let p = Point { sig, rate, b };
                assert!(p.riemann() == p.riemann_closed());
            }
        }
    }

    #[test]
    fn ricci_scalar_and_einstein_tensor_equal_the_closed_forms() {
        for sig in [Sig::S1644, Sig::S1671] {
            for (rate, b) in sets() {
                let p = Point { sig, rate, b };
                let ric = p.ricci(&p.riemann());
                let (ric_c, r_c, ein_c) = p.curvature_closed();
                let g = p.metric();
                let ein = p.einstein_mixed(&ric);
                assert_eq!(p.scalar(&ric), r_c);
                for a in 0..8 {
                    for bb in 0..8 {
                        if a == bb {
                            assert_eq!(ric[a][a] / g[a].v, ric_c[a]);
                            assert_eq!(ein[a][a], ein_c[a]);
                        } else {
                            assert!(ric[a][bb].is_zero() && ein[a][bb].is_zero());
                        }
                    }
                }
                // contracted Bianchi identity
                assert!(p.divergence_of_diagonal(&ein_c).iter().all(|x| x.is_zero()));
            }
        }
    }

    #[test]
    fn equal_rates_give_a_space_of_constant_curvature() {
        // R_{rho sigma mu nu} = K (g_{rho mu} g_{sigma nu} - g_{rho nu} g_{sigma mu}) on the
        // coordinates 4, 5, 6, 7, with K = -eta_44 rate^2
        for sig in [Sig::S1644, Sig::S1671] {
            let k = r(7, 3);
            let p = Point { sig, rate: [k, k, k], b: [r(1, 2), r(3, 1), r(5, 4)] };
            let riem = p.riemann();
            let g = p.metric();
            let kk = -p.eta(4) * k * k;
            for rho in 4..8 {
                for sigma in 4..8 {
                    for mu in 4..8 {
                        for nu in 4..8 {
                            let lower = g[rho].v * riem[rho][sigma][mu][nu];
                            let d = |a: usize, b: usize| if a == b { g[a].v } else { Rat::ZERO };
                            assert_eq!(lower, kk * (d(rho, mu) * d(sigma, nu) - d(rho, nu) * d(sigma, mu)));
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn the_listed_vectors_are_killing_vectors() {
        let x = [r(1, 3), r(-2, 1), r(5, 7), r(1, 2), r(0, 1), r(3, 4), r(-1, 5), r(2, 3)];
        for sig in [Sig::S1644, Sig::S1671] {
            for (rate, b) in sets() {
                let p = Point { sig, rate, b };
                let ks = killing_vectors(&p, &x);
                assert_eq!(ks.len(), killing_count(&rate));
                for v in &ks {
                    let l = lie_derivative_of_metric(&p, v);
                    assert!(l.iter().all(|row| row.iter().all(|c| c.is_zero())), "{} is not a Killing vector", v.name);
                }
            }
        }
    }

    #[test]
    fn a_rotation_of_two_deflating_coordinates_with_different_rates_is_not_an_isometry() {
        // rates 1 and 2: the scale factors of x5 and x6 differ at every x4 but one;
        // at a point where b_5 = 1/2 and b_6 = 1/4 the rotation x5 d/dx6 - x6 d/dx5 changes the metric
        let p = Point { sig: Sig::S1671, rate: [r(1, 1), r(2, 1), r(3, 1)], b: [r(1, 2), r(1, 4), r(1, 8)] };
        let x = [r(0, 1), r(0, 1), r(0, 1), r(0, 1), r(0, 1), r(1, 1), r(2, 1), r(3, 1)];
        let mut xi = [Rat::ZERO; 8];
        let mut dxi = [[Rat::ZERO; 8]; 8];
        xi[6] = x[5];
        xi[5] = -x[6];
        dxi[5][6] = Rat::ONE;
        dxi[6][5] = -Rat::ONE;
        let l = lie_derivative_of_metric(&p, &VectorAt { name: "rotation of x5 and x6".into(), xi, dxi });
        // (L g)_{56} = g_66 - g_55 = -1/16 + 1/4 = 3/16
        assert_eq!(l[5][6], r(3, 16));
        // and the rotation that is adapted to this one value of x4, (b_5 x5/b_6) d/dx6 - (b_6 x6/b_5) d/dx5
        // with coefficients that follow the scale factors, has d_4 xi^t != 0 and fails in the components (4, t)
        let mut xi2 = [Rat::ZERO; 8];
        let mut dxi2 = [[Rat::ZERO; 8]; 8];
        let ratio = p.b[0] / p.b[1]; // b_5/b_6 = 2 here; its derivative is (k6 - k5) ratio
        xi2[6] = ratio * x[5];
        xi2[5] = -x[6] / ratio;
        dxi2[5][6] = ratio;
        dxi2[6][5] = -Rat::ONE / ratio;
        dxi2[4][6] = (p.rate[1] - p.rate[0]) * ratio * x[5];
        dxi2[4][5] = (p.rate[1] - p.rate[0]) * x[6] / ratio;
        let l2 = lie_derivative_of_metric(&p, &VectorAt { name: "adapted rotation".into(), xi: xi2, dxi: dxi2 });
        assert!(l2[5][6].is_zero() && !l2[4][6].is_zero() && !l2[4][5].is_zero());
    }

    #[test]
    fn floating_point_geometry_at_a_general_point_agrees_with_the_closed_forms() {
        let rate = [0.37_f64, 1.9, 0.83];
        let b0 = [1.3_f64, 0.4, 2.2];
        let x4 = 0.731_f64;
        let b = [b0[0] * (-rate[0] * x4).exp(), b0[1] * (-rate[1] * x4).exp(), b0[2] * (-rate[2] * x4).exp()];
        for sig in [Sig::S1644, Sig::S1671] {
            let p = Point { sig, rate, b };
            let (a, c) = (p.riemann(), p.riemann_closed());
            let mut dev = 0.0_f64;
            for rho in 0..8 {
                for sigma in 0..8 {
                    for mu in 0..8 {
                        for nu in 0..8 {
                            dev = dev.max((a[rho][sigma][mu][nu] - c[rho][sigma][mu][nu]).abs());
                        }
                    }
                }
            }
            assert!(dev < 1e-12, "deviation {}", dev);
        }
    }
}
