//! The Dirac matrices, the spin connection and the Dirac operators of the two
//! deflating spacetimes, with exact machine checks.
//!
//! SETTING.  The tangent space of the deflating spacetime at every point is the flat
//! space of the main project, so the eight real 16x16 matrices gamma^A of the notebooks
//! (gamma1644^A for 4+4, gamma1671^A for 7+1) are kept unchanged as the matrices of
//! the orthonormal frame
//!
//!     e^A = dx^A  (A = 0..4),        e^t = b_t(x4) dx^t  (t = 5, 6, 7).
//!
//! The matrices that multiply the partial derivatives in the Dirac equation are the
//! coordinate ("curved") matrices
//!
//!     gamma^mu(x4) = e_A^mu gamma^A :   gamma^t(x4) = gamma^t / b_t(x4),   the other five unchanged,
//!
//! which satisfy gamma^mu gamma^nu + gamma^nu gamma^mu = 2 g^{mu nu}(x4).
//!
//! SPIN CONNECTION.  The frame turns from point to point; the spinor derivative is
//!
//!     nabla_mu = d_mu + Omega_mu,     Omega_mu = (1/4) omega_{mu A B} gamma^A gamma^B,
//!     omega_mu^A_B = e^A_nu ( d_mu e_B^nu + Gamma^nu_{mu lam} e_B^lam )      (tetrad postulate).
//!
//! For the deflating metric the only non-zero coefficients are
//! omega_{t,t4} = -omega_{t,4t} = -eta_tt rate_t b_t, so that
//!
//!     Omega_t = -(1/2) eta_tt rate_t b_t gamma^t gamma^4,     Omega_mu = 0 otherwise,
//!     gamma^mu Omega_mu = -(1/2) gsum gamma^4,          gsum = g55 + g66 + g77.
//!
//! DIRAC EQUATIONS.
//!     4+4 (field dirac1644d):   ( gamma^mu nabla_mu - m ) psi = 0,
//!     7+1 (field dirac1671d):   ( i gamma^mu nabla_mu - m ) psi = 0.
//! With the weighted field psihat = sqrt(b_5 b_6 b_7) psi the spin connection drops out:
//!     4+4:  [ gamma^4 d_4 + gamma^a d_a + sum_t b_t^{-1} gamma^t d_t - m ] psihat = 0,
//!     7+1:  [ i gamma^4 d_4 + i gamma^j d_j + i sum_t b_t^{-1} gamma^t d_t - m ] psihat = 0.
//!
//! EXACTNESS.  At one point all of this is rational in the rates and in the values of
//! the scale factors there; the checks below use rational values and exact arithmetic
//! (`QMat`).  The time derivatives that occur are known in closed form:
//! d b_t/dx4 = -rate_t b_t.

use crate::metric::{Point, Sig};
use crate::qmat::QMat;
use crate::rat::{Qc, Rat};
use spacetimes::gamma::{swap04, Gammas};
use spacetimes::verify::Report;

/// The matrices of one spacetime at one point.
pub struct Curved {
    pub p: Point<Rat>,
    /// the flat (frame) matrices gamma^A of the main project
    pub flat: Vec<QMat>,
    /// the coordinate matrices gamma^mu = e_A^mu gamma^A (contravariant index)
    pub gam: Vec<QMat>,
    /// the spin-connection matrices Omega_mu
    pub omega: Vec<QMat>,
}

fn rat(n: i128) -> Rat {
    Rat::int(n)
}

impl Curved {
    pub fn new(gm: &Gammas, p: Point<Rat>) -> Curved {
        let src = match p.sig {
            Sig::S1644 => &gm.gamma1644,
            Sig::S1671 => &gm.gamma1671,
        };
        let flat: Vec<QMat> = src.iter().map(QMat::from_imat).collect();
        let e = p.vielbein();
        let gam: Vec<QMat> = (0..8).map(|a| flat[a].scale_rat(e[a].recip())).collect();
        let w = Curved::spin_coefficients(&p);
        let mut omega: Vec<QMat> = Vec::new();
        for mu in 0..8 {
            let mut o = QMat::zeros(16, 16);
            for a in 0..8 {
                for b in 0..8 {
                    if !w[mu][a][b].is_zero() {
                        o = o.add(&flat[a].mul(&flat[b]).scale_rat(w[mu][a][b] * Rat::new(1, 4)));
                    }
                }
            }
            omega.push(o);
        }
        Curved { p, flat, gam, omega }
    }

    /// The coefficients omega_{mu A B} (frame indices lowered with eta) from the tetrad
    /// postulate, omega_mu^A_B = e^A_nu ( d_mu e_B^nu + Gamma^nu_{mu lam} e_B^lam ),
    /// with the Christoffel symbols computed from the metric by `metric.rs`.
    pub fn spin_coefficients(p: &Point<Rat>) -> [[[Rat; 8]; 8]; 8] {
        let gam = p.christoffel();
        let e = p.vielbein();
        let mut w = [[[Rat::ZERO; 8]; 8]; 8];
        for mu in 0..8 {
            for a in 0..8 {
                for b in 0..8 {
                    // e_B^nu = delta^nu_B / e_B;  d_4 (1/b_t) = rate_t / b_t
                    let mut x = gam[a][mu][b].v / e[b];
                    if mu == 4 && a == b && b >= 5 {
                        x = x + p.rate[b - 5] / e[b];
                    }
                    w[mu][a][b] = p.eta(a) * e[a] * x; // lower the first frame index
                }
            }
        }
        w
    }

    /// d_mu gamma^nu: only d_4 gamma^t = +rate_t gamma^t is non-zero.
    pub fn d_gamma(&self, mu: usize, nu: usize) -> QMat {
        if mu == 4 && nu >= 5 {
            self.gam[nu].scale_rat(self.p.rate[nu - 5])
        } else {
            QMat::zeros(16, 16)
        }
    }

    /// d_mu Omega_nu: only d_4 Omega_t = -rate_t Omega_t is non-zero.
    pub fn d_omega(&self, mu: usize, nu: usize) -> QMat {
        if mu == 4 && nu >= 5 {
            self.omega[nu].scale_rat(-self.p.rate[nu - 5])
        } else {
            QMat::zeros(16, 16)
        }
    }

    /// The matrix gamma^4 gamma^A of the flat matrices.
    pub fn g4g(&self, a: usize) -> QMat {
        self.flat[4].mul(&self.flat[a])
    }

    /// 4+4: the generator K of the evolution of the weighted field in x4, d_4 psihat = K psihat,
    /// for a field proportional to exp(i k_a x^a + i k_t x^t):
    ///     K = -m gamma^4 + i sum_{a=0..3} k_a gamma^4 gamma^a + i sum_t (k_t / b_t) gamma^4 gamma^t .
    pub fn k_1644(&self, m: Rat, k: &[Rat; 4], kappa: &[Rat; 3]) -> QMat {
        let mut out = self.flat[4].scale_rat(-m);
        for a in 0..4 {
            out = out.add(&self.g4g(a).scale(Qc { re: Rat::ZERO, im: k[a] }));
        }
        for i in 0..3 {
            out = out.add(&self.g4g(5 + i).scale(Qc { re: Rat::ZERO, im: kappa[i] / self.p.b[i] }));
        }
        out
    }

    /// 7+1: the Hamiltonian of the weighted field, i d_4 psihat = h psihat, for a field
    /// proportional to exp(i p_j x^j + i k_t x^t)  (j = 0, 1, 2, 3):
    ///     h = m beta + sum_j p_j alpha^j + sum_t (k_t / b_t) alpha^t,   beta = gamma^4, alpha^A = gamma^4 gamma^A .
    pub fn h_1671(&self, m: Rat, p: &[Rat; 4], kappa: &[Rat; 3]) -> QMat {
        let mut out = self.flat[4].scale_rat(m);
        for a in 0..4 {
            out = out.add(&self.g4g(a).scale_rat(p[a]));
        }
        for i in 0..3 {
            out = out.add(&self.g4g(5 + i).scale_rat(kappa[i] / self.p.b[i]));
        }
        out
    }
}

/// The rational parameter sets (rates; values of the scale factors at the point) for
/// which every exact check is carried out.  The first is the simplest isotropic case,
/// the second and third are anisotropic, the fourth has two equal rates, the fifth is
/// isotropic with unequal constants.
pub fn parameter_sets() -> Vec<([Rat; 3], [Rat; 3])> {
    let r = Rat::new;
    vec![
        ([r(1, 1), r(1, 1), r(1, 1)], [r(1, 1), r(1, 1), r(1, 1)]),
        ([r(1, 1), r(2, 1), r(3, 1)], [r(1, 2), r(1, 3), r(1, 5)]),
        ([r(1, 2), r(3, 4), r(5, 3)], [r(2, 1), r(1, 3), r(5, 7)]),
        ([r(2, 1), r(2, 1), r(5, 1)], [r(3, 2), r(3, 2), r(1, 4)]),
        ([r(7, 3), r(7, 3), r(7, 3)], [r(1, 2), r(3, 1), r(5, 4)]),
    ]
}

/// A short description of the parameter sets for the statements of the checks.
pub fn parameter_sets_text() -> String {
    parameter_sets()
        .iter()
        .map(|(k, b)| format!("(g55, g66, g77) = ({}, {}, {}) at a point where (b_5, b_6, b_7) = ({}, {}, {})", k[0], k[1], k[2], b[0], b[1], b[2]))
        .collect::<Vec<String>>()
        .join("; ")
}

/// All exact checks of the curved matrices, the spin connection and the Dirac operator
/// of one spacetime.  Identifiers: 44d-.. for 4+4, 71d-.. for 7+1.
pub fn verify_curved(gm: &Gammas, sig: Sig, rep: &mut Report) {
    let pre = match sig {
        Sig::S1644 => "44d",
        Sig::S1671 => "71d",
    };
    let tag = sig.tag();
    let gname = match sig {
        Sig::S1644 => "gamma1644",
        Sig::S1671 => "gamma1671",
    };
    let sets = parameter_sets();
    let one = QMat::identity(16);
    let zero16 = QMat::zeros(16, 16);

    // accumulators: a check passes only if it holds for every parameter set
    let mut c1 = true; // real, Clifford relation with g^{mu nu}
    let mut c2 = true; // spin coefficients
    let mut c3 = true; // Omega closed form
    let mut c4 = true; // covariant constancy of gamma^mu
    let mut c5 = true; // gamma^mu Omega_mu
    let mut c6 = true; // spinor curvature = Riemann
    let mut c7 = true; // (1/2) gamma^mu gamma^nu F_{mu nu} = -(R/4) 1
    let mut c8 = true; // compatibility with the adjoint
    for (rate, b) in &sets {
        let p = Point { sig, rate: *rate, b: *b };
        let cv = Curved::new(gm, p);
        let g = p.metric();
        let e = p.vielbein();
        // C1
        for mu in 0..8 {
            c1 &= cv.gam[mu].is_real();
            for nu in 0..8 {
                let want = if mu == nu { Qc::real(rat(2) / g[mu].v) } else { Qc::ZERO };
                c1 &= cv.gam[mu].anti(&cv.gam[nu]).is_multiple_of_identity(want);
            }
        }
        // C2
        let w = Curved::spin_coefficients(&p);
        for mu in 0..8 {
            for a in 0..8 {
                for bb in 0..8 {
                    c2 &= w[mu][a][bb] == -w[mu][bb][a];
                    let want = if mu >= 5 && a == mu && bb == 4 {
                        -p.eta(mu) * rate[mu - 5] * b[mu - 5]
                    } else if mu >= 5 && a == 4 && bb == mu {
                        p.eta(mu) * rate[mu - 5] * b[mu - 5]
                    } else {
                        Rat::ZERO
                    };
                    c2 &= w[mu][a][bb] == want;
                }
            }
        }
        // C3
        for mu in 0..8 {
            let want = if mu >= 5 {
                cv.flat[mu].mul(&cv.flat[4]).scale_rat(-p.eta(mu) * rate[mu - 5] * b[mu - 5] * Rat::new(1, 2))
            } else {
                zero16.clone()
            };
            c3 &= cv.omega[mu] == want;
        }
        // C4: d_mu gamma^nu + Gamma^nu_{mu lam} gamma^lam + [Omega_mu, gamma^nu] = 0
        let chr = p.christoffel();
        for mu in 0..8 {
            for nu in 0..8 {
                let mut x = cv.d_gamma(mu, nu).add(&cv.omega[mu].comm(&cv.gam[nu]));
                for lam in 0..8 {
                    if !chr[nu][mu][lam].v.is_zero() {
                        x = x.add(&cv.gam[lam].scale_rat(chr[nu][mu][lam].v));
                    }
                }
                c4 &= x.is_zero();
            }
        }
        // C5: gamma^mu Omega_mu = -(1/2) gsum gamma^4
        let mut go = zero16.clone();
        for mu in 0..8 {
            go = go.add(&cv.gam[mu].mul(&cv.omega[mu]));
        }
        c5 &= go == cv.flat[4].scale_rat(-p.rate_sum() * Rat::new(1, 2));
        // C6: F_{mu nu} = d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = (1/4) R_{A B mu nu} gamma^A gamma^B
        let riem = p.riemann();
        let mut lich = zero16.clone();
        for mu in 0..8 {
            for nu in 0..8 {
                let f = cv.d_omega(mu, nu).sub(&cv.d_omega(nu, mu)).add(&cv.omega[mu].comm(&cv.omega[nu]));
                let mut want = zero16.clone();
                for a in 0..8 {
                    for bb in 0..8 {
                        // R_{A B mu nu} = eta_AA e_A (1/e_B) R^A_{B mu nu}  (coordinate components R^rho_{sigma mu nu})
                        let r = p.eta(a) * e[a] * riem[a][bb][mu][nu] / e[bb];
                        if !r.is_zero() {
                            want = want.add(&cv.flat[a].mul(&cv.flat[bb]).scale_rat(r * Rat::new(1, 4)));
                        }
                    }
                }
                c6 &= f == want;
                lich = lich.add(&cv.gam[mu].mul(&cv.gam[nu]).mul(&f).scale_rat(Rat::new(1, 2)));
            }
        }
        // C7
        let rs = p.scalar(&p.ricci(&riem));
        c7 &= lich.is_multiple_of_identity(Qc::real(-rs * Rat::new(1, 4)));
        // C8: (4+4) Omega^T C^- + C^- Omega = 0 with C^- = gamma^0 gamma^1 gamma^2 gamma^3;
        //     (7+1) Omega^T gamma^4 + gamma^4 Omega = 0
        let adj = match sig {
            Sig::S1644 => QMat::from_imat(&gm.sigma16),
            Sig::S1671 => cv.flat[4].clone(),
        };
        for mu in 0..8 {
            c8 &= cv.omega[mu].dagger().mul(&adj).add(&adj.mul(&cv.omega[mu])).is_zero();
        }
    }
    let sets_txt = parameter_sets_text();
    rep.add(&format!("{}-C1", pre), "exact", &format!("the coordinate matrices gamma_{tag}^mu = e_A^mu {gname}[A] (gamma^t = {gname}[t]/b_t for t = 5, 6, 7; the other five unchanged) are real and satisfy gamma^mu.gamma^nu + gamma^nu.gamma^mu = 2 g_{tag}^(mu nu) 1 for all 64 pairs, with g^(tt) = eta^(tt)/b_t^2. Verified for five rational parameter sets: {sets_txt}"), c1);
    rep.add(&format!("{}-C2", pre), "exact", "the spin-connection coefficients computed from the tetrad postulate omega_mu^A_B = e^A_nu (d_mu e_B^nu + Gamma^nu_(mu lam) e_B^lam), with the Christoffel symbols obtained from the metric, are antisymmetric, omega_(mu A B) = -omega_(mu B A), and the only non-zero ones are omega_(t,t4) = -omega_(t,4t) = -eta_tt rate_t b_t (t = 5, 6, 7); same five parameter sets", c2);
    rep.add(&format!("{}-C3", pre), "exact", &format!("Omega_mu := (1/4) omega_(mu A B) {gname}[A].{gname}[B] equals -(1/2) eta_tt rate_t b_t {gname}[t].{gname}[4] for mu = t in {{5, 6, 7}} and vanishes for mu = 0..4: the spin connection is an element of the Lie algebra spanned by the scaled commutators Sigma^(t4)"), c3);
    rep.add(&format!("{}-C4", pre), "exact", "d_mu gamma^nu + Gamma^nu_(mu lam) gamma^lam + [Omega_mu, gamma^nu] = 0 for all 64 pairs (mu, nu), with d_4 gamma^t = +rate_t gamma^t: the coordinate matrices are covariantly constant", c4);
    rep.add(&format!("{}-C5", pre), "exact", &format!("sum_mu gamma^mu.Omega_mu = -(1/2) (g55 + g66 + g77) {gname}[4]: in the Dirac operator the spin connection amounts to the replacement d_4 -> d_4 - gsum/2, which the weighted field psihat = sqrt(b_5 b_6 b_7) psi removes"), c5);
    rep.add(&format!("{}-C6", pre), "exact", &format!("d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu] = (1/4) R_(A B mu nu) {gname}[A].{gname}[B] for all 64 pairs (mu, nu), with the Riemann tensor that metric.rs computes from the metric (d_4 Omega_t = -rate_t Omega_t): the curvature of the spinor derivative is the Riemann tensor"), c6);
    rep.add(&format!("{}-C7", pre), "exact", "(1/2) sum_(mu nu) gamma^mu.gamma^nu.(d_mu Omega_nu - d_nu Omega_mu + [Omega_mu, Omega_nu]) = -(R/4) 1 with the curvature scalar R = -eta_44 (g55^2 + g66^2 + g77^2 + gsum^2): the square of the Dirac operator differs from the wave operator of the spinor derivative by -R/4", c7);
    let adj_txt = match sig {
        Sig::S1644 => "transpose(Omega_mu).Cm1644 + Cm1644.Omega_mu = 0 (Cm1644 = gamma1644[0].gamma1644[1].gamma1644[2].gamma1644[3]): the adjoint field psi^dagger Cm1644 is differentiated with -Omega_mu on its right, as the conserved current requires",
        Sig::S1671 => "transpose(Omega_mu).gamma1671[4] + gamma1671[4].Omega_mu = 0: the adjoint field psi^dagger gamma1671[4] is differentiated with -Omega_mu on its right, as the conserved current requires",
    };
    rep.add(&format!("{}-C8", pre), "exact", adj_txt, c8);

    // ---------------- the Dirac operator in momentum space ----------------------------
    let r = Rat::new;
    // momenta used: several rational choices
    let moms: Vec<(Rat, [Rat; 4], [Rat; 3])> = vec![
        (r(1, 1), [r(0, 1), r(2, 1), r(2, 1), r(4, 1)], [r(0, 1), r(0, 1), r(0, 1)]),
        (r(3, 1), [r(0, 1), r(0, 1), r(0, 1), r(0, 1)], [r(5, 1), r(0, 1), r(0, 1)]),
        (r(2, 3), [r(1, 2), r(-3, 1), r(1, 5), r(2, 1)], [r(1, 1), r(-2, 3), r(4, 1)]),
        (r(5, 2), [r(1, 1), r(1, 1), r(-1, 1), r(2, 1)], [r(0, 1), r(7, 2), r(-1, 3)]),
    ];
    match sig {
        Sig::S1644 => {
            let cm = QMat::from_imat(&gm.sigma16);
            let (mut d1, mut d2, mut d3, mut d4, mut d5, mut d6) = (true, true, true, true, true, true);
            for (rate, b) in &sets {
                let p = Point { sig, rate: *rate, b: *b };
                let cv = Curved::new(gm, p);
                let w = cm.mul(&cv.flat[4]).times_i(); // W = i C^- gamma^4
                d4 &= w.is_hermitian() && w.mul(&w) == one && w.trace().is_zero();
                for (m, k, kap) in &moms {
                    let kk = cv.k_1644(*m, k, kap);
                    let mut eps2 = *m * *m;
                    for a in 0..4 {
                        eps2 = eps2 + k[a] * k[a];
                    }
                    let mut q2 = Rat::ZERO;
                    for i in 0..3 {
                        let x = kap[i] / b[i];
                        q2 = q2 + x * x;
                    }
                    // D1: (gamma^mu i k_mu + m)(gamma^nu i k_nu - m) = -(g^{mu nu} k_mu k_nu + m^2) for an arbitrary k_4
                    let k4 = r(3, 7);
                    let mut dk = cv.gam[4].scale(Qc { re: Rat::ZERO, im: k4 });
                    for a in 0..4 {
                        dk = dk.add(&cv.gam[a].scale(Qc { re: Rat::ZERO, im: k[a] }));
                    }
                    for i in 0..3 {
                        dk = dk.add(&cv.gam[5 + i].scale(Qc { re: Rat::ZERO, im: kap[i] }));
                    }
                    let gkk = eps2 - *m * *m - k4 * k4 - q2; // g^{mu nu} k_mu k_nu = |k|^2 - k_4^2 - sum k_t^2/b^2
                    let mm = one.scale_rat(*m);
                    d1 &= dk.add(&mm).mul(&dk.sub(&mm)).is_multiple_of_identity(Qc::real(-(gkk + *m * *m)));
                    // D2: K^2 = (q^2 - E_perp^2) 1,  K + K^dagger = 2 i sum (k_t/b_t) gamma^4 gamma^t
                    d2 &= kk.mul(&kk).is_multiple_of_identity(Qc::real(q2 - eps2));
                    let mut herm = zero16.clone();
                    let mut kdot = zero16.clone();
                    for i in 0..3 {
                        herm = herm.add(&cv.g4g(5 + i).scale(Qc { re: Rat::ZERO, im: rat(2) * kap[i] / b[i] }));
                        // d_4 (k_t / b_t) = +rate_t k_t / b_t
                        kdot = kdot.add(&cv.g4g(5 + i).scale(Qc { re: Rat::ZERO, im: rate[i] * kap[i] / b[i] }));
                    }
                    d2 &= kk.add(&kk.dagger()) == herm;
                    // D3: dK/dx4 = i sum_t rate_t (k_t/b_t) gamma^4 gamma^t (because d_4 (1/b_t) = rate_t/b_t).
                    //     It is Hermitian (i times a real antisymmetric matrix) and its square is sum_t (rate_t k_t/b_t)^2.
                    //     The second-order equation of the weighted field is d_4^2 psihat = (K^2 + dK/dx4) psihat.
                    d3 &= kdot.is_hermitian() && kdot.mul(&kdot).is_multiple_of_identity(Qc::real({
                        let mut s = Rat::ZERO;
                        for i in 0..3 {
                            let x = rate[i] * kap[i] / b[i];
                            s = s + x * x;
                        }
                        s
                    }));
                    // D4: W K + K^dagger W = 0 (the covariant charge psihat^dagger W psihat is conserved for every mode)
                    d4 &= w.mul(&kk).add(&kk.dagger().mul(&w)).is_zero();
                    // D5: K is anti-Hermitian (unitary evolution) if and only if k_t = 0
                    let anti = kk.is_antihermitian();
                    d5 &= anti == (kap.iter().all(|x| x.is_zero()));
                    // D6: the two 8-component equations.  gamma^A = [[0, taubar^A], [tau^A, 0]]:
                    //     upper-right block of (gamma^mu i k_mu) is i sum taubar^A k_A / e_A, lower-left block i sum tau^A k_A / e_A
                    let e = p.vielbein();
                    let kvec = [k[0], k[1], k[2], k[3], k4, kap[0], kap[1], kap[2]];
                    let mut ur = QMat::zeros(8, 8);
                    let mut ll = QMat::zeros(8, 8);
                    for a in 0..8 {
                        let c = Qc { re: Rat::ZERO, im: kvec[a] / e[a] };
                        ur = ur.add(&QMat::from_imat(&gm.taubar[a]).scale(c));
                        ll = ll.add(&QMat::from_imat(&gm.tau[a]).scale(c));
                    }
                    d6 &= dk.block(0, 8, 8, 8) == ur && dk.block(8, 0, 8, 8) == ll && dk.block(0, 0, 8, 8).is_zero() && dk.block(8, 8, 8, 8).is_zero();
                }
            }
            rep.add("44d-D1", "exact", "in momentum space (d_mu -> i k_mu) the Dirac operator of the weighted field at fixed x4, D = i gamma^mu k_mu with the coordinate matrices, satisfies (D + m)(D - m) = -(g^(mu nu) k_mu k_nu + m^2) 1 with g^(mu nu) k_mu k_nu = k_0^2 + k_1^2 + k_2^2 + k_3^2 - k_4^2 - sum_t k_t^2/b_t^2: the instantaneous mass shell is k_4^2 + sum_t k_t^2/b_t(x4)^2 = |k|^2 + m^2. Verified for five rational parameter sets and four rational momenta each", d1);
            rep.add("44d-D2", "exact", "the generator of the evolution of the weighted field in x4, K(x4) = -m gamma1644[4] + i sum_a k_a gamma1644[4].gamma1644[a] + i sum_t (k_t/b_t(x4)) gamma1644[4].gamma1644[t], satisfies K^2 = (sum_t k_t^2/b_t^2 - m^2 - |k|^2) 1 and K + K^dagger = 2 i sum_t (k_t/b_t) gamma1644[4].gamma1644[t]", d2);
            rep.add("44d-D3", "exact", "dK/dx4 = i sum_t rate_t (k_t/b_t) gamma1644[4].gamma1644[t] is a Hermitian matrix whose square is sum_t (rate_t k_t/b_t)^2 times the unit matrix; the weighted field obeys the second-order equation d_4^2 psihat = (K^2 + dK/dx4) psihat, a Klein-Gordon equation with a term that depends on the spin", d3);
            rep.add("44d-D4", "exact", "W = i Cm1644.gamma1644[4] is Hermitian with W^2 = 1 and trace 0, and W.K + K^dagger.W = 0 for every momentum and all k_5, k_6, k_7: the covariant charge psihat^dagger W psihat of a mode is conserved on the deflating spacetime, and it is not positive", d4);
            rep.add("44d-D5", "exact", "K is anti-Hermitian (the evolution in x4 is unitary and psihat^dagger psihat is conserved) if and only if k_5 = k_6 = k_7 = 0, that is, if the field does not depend on the deflating times", d5);
            rep.add("44d-D6", "exact", "with gamma1644[A] = [[0, taubar1644[A]], [tau1644[A], 0]] the operator gamma^mu d_mu of the weighted field has the blocks [[0, sum_A taubar1644[A] e_A^mu d_mu], [sum_A tau1644[A] e_A^mu d_mu, 0]], e_A^mu d_mu = (d_0, .., d_4, d_5/b_5, d_6/b_6, d_7/b_7): the equation is the pair taubar^A e_A^mu d_mu psihat^R = m psihat^L, tau^A e_A^mu d_mu psihat^L = m psihat^R of real 8-component equations", d6);
            // D7: worked example in the evanescent region: m = 3, k = 0, k_5/b_5 = 5 -> K^2 = +16
            let p = Point { sig, rate: [r(1, 1), r(1, 1), r(1, 1)], b: [r(1, 5), r(1, 1), r(1, 1)] };
            let cv = Curved::new(gm, p);
            let kk = cv.k_1644(r(3, 1), &[Rat::ZERO; 4], &[r(1, 1), Rat::ZERO, Rat::ZERO]);
            let four = one.scale_rat(r(4, 1));
            let grow = kk.add(&four).scale_rat(r(1, 8)); // projector on K = +4
            let decay = four.sub(&kk).scale_rat(r(1, 8)); // projector on K = -4
            let ok = kk.mul(&kk).is_multiple_of_identity(Qc::int(16))
                && grow.mul(&grow) == grow
                && decay.mul(&decay) == decay
                && grow.trace() == Qc::int(8)
                && decay.trace() == Qc::int(8)
                && grow.mul(&decay).is_zero();
            rep.add("44d-D7", "exact", "worked example: m = 3, k = 0, (k_5, k_6, k_7) = (1, 0, 0) at the value of x4 where b_5 = 1/5 (so k_5/b_5 = 5 > m): K^2 = +16 1, and (4 + K)/8 and (4 - K)/8 are complementary projectors of trace 8: at this x4 eight independent solutions grow as exp(+4 x4) and eight decay as exp(-4 x4) to first order. A mode that depends on a deflating time is oscillatory only while k_t/b_t(x4) < sqrt(m^2 + |k|^2)", ok);
        }
        Sig::S1671 => {
            let refs: Vec<&spacetimes::imat::IMat> = gm.gamma1671.iter().collect();
            let om = QMat::from_imat(&spacetimes::imat::IMat::product(&refs));
            let (mut d1, mut d2, mut d3, mut d5, mut d6) = (true, true, true, true, true);
            for (rate, b) in &sets {
                let p = Point { sig, rate: *rate, b: *b };
                let cv = Curved::new(gm, p);
                let e = p.vielbein();
                for (m, pp, kap) in &moms {
                    let h = cv.h_1671(*m, pp, kap);
                    let mut eps2 = *m * *m;
                    for a in 0..4 {
                        eps2 = eps2 + pp[a] * pp[a];
                    }
                    let mut q2 = Rat::ZERO;
                    for i in 0..3 {
                        let x = kap[i] / b[i];
                        q2 = q2 + x * x;
                    }
                    // D1: (gamma^mu k_mu + m)(gamma^nu k_nu - m) = (g^{mu nu} k_mu k_nu - m^2) 1 for arbitrary k_4
                    let k4 = r(3, 7);
                    let kvec = [pp[0], pp[1], pp[2], pp[3], k4, kap[0], kap[1], kap[2]];
                    let mut dk = zero16.clone();
                    for a in 0..8 {
                        dk = dk.add(&cv.gam[a].scale_rat(kvec[a]));
                    }
                    let gkk = k4 * k4 - (eps2 - *m * *m) - q2;
                    let mm = one.scale_rat(*m);
                    d1 &= dk.add(&mm).mul(&dk.sub(&mm)).is_multiple_of_identity(Qc::real(gkk - *m * *m));
                    // D2: h real symmetric, h^2 = (E_perp^2 + q^2) 1
                    d2 &= h.is_real() && h.is_hermitian() && h.mul(&h).is_multiple_of_identity(Qc::real(eps2 + q2));
                    // D3: dh/dx4 = sum_t rate_t (k_t/b_t) alpha^t; it is real symmetric and anticommutes with beta
                    let mut hdot = zero16.clone();
                    for i in 0..3 {
                        hdot = hdot.add(&cv.g4g(5 + i).scale_rat(rate[i] * kap[i] / b[i]));
                    }
                    d3 &= hdot.is_hermitian() && hdot.anti(&cv.flat[4]).is_zero();
                    // D5: charge conjugation: omega commutes with alpha^A and anticommutes with beta
                    for a in 0..8 {
                        if a != 4 {
                            d5 &= om.comm(&cv.g4g(a)).is_zero();
                        }
                    }
                    d5 &= om.anti(&cv.flat[4]).is_zero();
                    // D6: chiral 8-component form.  Etilde^L = (1, -i)^T, Etilde^R = (-i, 1)^T (blocks of 8; sqrt(2) times E^L, E^R):
                    //     Etilde^L^dagger h Etilde^L = -2 i sum_j (p_j/e_j) tau^j,  Etilde^R^dagger h Etilde^R = +2 i (same),
                    //     Etilde^L^dagger h Etilde^R = Etilde^R^dagger h Etilde^L = 2 m
                    let mut el = QMat::zeros(16, 8);
                    let mut er = QMat::zeros(16, 8);
                    for i in 0..8 {
                        el.set(i, i, Qc::ONE);
                        el.set(8 + i, i, -Qc::I);
                        er.set(i, i, -Qc::I);
                        er.set(8 + i, i, Qc::ONE);
                    }
                    let mut ptau = QMat::zeros(8, 8);
                    for a in 0..8 {
                        if a != 4 {
                            ptau = ptau.add(&QMat::from_imat(&gm.tau_o[swap04(a)]).scale_rat(kvec[a] / e[a]));
                        }
                    }
                    let two_m = QMat::identity(8).scale_rat(rat(2) * *m);
                    d6 &= el.dagger().mul(&h).mul(&el) == ptau.scale(Qc { re: Rat::ZERO, im: rat(-2) })
                        && er.dagger().mul(&h).mul(&er) == ptau.scale(Qc { re: Rat::ZERO, im: rat(2) })
                        && el.dagger().mul(&h).mul(&er) == two_m
                        && er.dagger().mul(&h).mul(&el) == two_m;
                }
            }
            rep.add("71d-D1", "exact", "in momentum space (d_mu -> -i k_mu) the Dirac operator of the weighted field at fixed x4 is gamma^mu k_mu - m with the coordinate matrices, and (gamma^mu k_mu + m)(gamma^nu k_nu - m) = (g^(mu nu) k_mu k_nu - m^2) 1 with g^(mu nu) k_mu k_nu = k_4^2 - k_0^2 - k_1^2 - k_2^2 - k_3^2 - sum_t k_t^2/b_t^2: the instantaneous mass shell is k_4^2 = |p|^2 + sum_t k_t^2/b_t(x4)^2 + m^2. Verified for five rational parameter sets and four rational momenta each", d1);
            rep.add("71d-D2", "exact", "the Hamiltonian of the weighted field, h(x4) = m beta1671 + sum_j p_j alpha1671^j + sum_t (k_t/b_t(x4)) alpha1671^t (beta1671 = gamma1671[4], alpha1671^A = gamma1671[4].gamma1671[A]), is real and symmetric for every momentum and every x4, and h^2 = (m^2 + |p|^2 + sum_t k_t^2/b_t^2) 1: the evolution in x4 is unitary and psihat^dagger psihat is conserved, but the energy of a mode with k_t != 0 grows as exp(rate_t x4)", d2);
            rep.add("71d-D3", "exact", "dh/dx4 = sum_t rate_t (k_t/b_t) alpha1671^t is real symmetric and anticommutes with beta1671: the Hamiltonian changes with x4 only through the momenta along the deflating coordinates, which is what creates pairs", d3);
            rep.add("71d-D5", "exact", "omega1671 commutes with every alpha1671^A and anticommutes with beta1671; hence the charge-conjugate field omega1671 psi^* solves the Dirac equation on the deflating spacetime whenever psi does (the scale factors are real)", d5);
            rep.add("71d-D6", "exact", "with the chiral 16x8 matrices E^L, E^R of the main project, (E^L)^dagger h E^L = -i sum_j (p_j/e_j) tau1671^j, (E^R)^dagger h E^R = +i sum_j (p_j/e_j) tau1671^j and (E^L)^dagger h E^R = (E^R)^dagger h E^L = m, where p_j/e_j = (p_0, p_1, p_2, p_3, k_5/b_5, k_6/b_6, k_7/b_7): the equation of the weighted field is the pair (i d_4 + tau^j e_j^mu d_mu) phihat^L = m phihat^R, (i d_4 - tau^j e_j^mu d_mu) phihat^R = m phihat^L of 8-component equations", d6);
            // D7: worked example: m = 3, p = 0, k_5/b_5 = 4 -> E = 5
            let p = Point { sig, rate: [r(1, 1), r(1, 1), r(1, 1)], b: [r(1, 2), r(1, 1), r(1, 1)] };
            let cv = Curved::new(gm, p);
            let h = cv.h_1671(r(3, 1), &[Rat::ZERO; 4], &[r(2, 1), Rat::ZERO, Rat::ZERO]);
            let five = one.scale_rat(r(5, 1));
            let pos = five.add(&h).scale_rat(r(1, 10));
            let neg = five.sub(&h).scale_rat(r(1, 10));
            let ok = h.mul(&h).is_multiple_of_identity(Qc::int(25)) && pos.mul(&pos) == pos && neg.mul(&neg) == neg && pos.trace() == Qc::int(8) && neg.trace() == Qc::int(8);
            rep.add("71d-D7", "exact", "worked example: m = 3, p = 0, (k_5, k_6, k_7) = (2, 0, 0) at the value of x4 where b_5 = 1/2 (so k_5/b_5 = 4): h^2 = 25 1, and (5 + h)/10, (5 - h)/10 are complementary projectors of trace 8: eight states of energy +5 and eight of energy -5 at this x4; one e-folding of the deflation later the energy is sqrt(9 + 16 e^2) = 11.28", ok);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_exact_checks_of_the_curved_matrices_pass() {
        let gm = Gammas::build();
        let mut rep = Report::new();
        verify_curved(&gm, Sig::S1644, &mut rep);
        verify_curved(&gm, Sig::S1671, &mut rep);
        for c in &rep.checks {
            assert!(c.pass, "check {} fails: {}", c.id, c.statement);
        }
        // 8 checks C1..C8 and 7 (4+4) or 6 (7+1) checks of the Dirac operator
        assert_eq!(rep.checks.len(), 8 + 7 + 8 + 6);
    }

    #[test]
    fn spin_connection_of_the_simplest_case() {
        // rates 1, scale factors 1, 7+1: Omega_5 = -(1/2)(-1)(1)(1) gamma^5 gamma^4 = (1/2) gamma^5 gamma^4
        let gm = Gammas::build();
        let one = Rat::ONE;
        let p = Point { sig: Sig::S1671, rate: [one; 3], b: [one; 3] };
        let cv = Curved::new(&gm, p);
        assert_eq!(cv.omega[5], cv.flat[5].mul(&cv.flat[4]).scale_rat(Rat::new(1, 2)));
        assert!(cv.omega[4].is_zero() && cv.omega[0].is_zero());
    }
}
