//! curved -- the geometry of a curved space-time in the tetrad form of Lord's reference vectors, and the brochon in
//! it: Part III of lord_gravity.md (task C23 of PROJECT spacetimes, folder Revision_Lord_Gravity).
//!
//! Conventions (Section 13 of the manuscript):
//! * frame indices a, b, c, d and coordinate indices mu, nu, rho, sigma, la run over 0..3; the metric of the frame is
//!   eta^LL = diag(1, -1, -1, -1);
//! * the coframe e^a_mu (array `co[a][mu]`) and the frame e_a^mu (array `fr[a][mu]`, Lord's reference vectors
//!   h_a^mu) are inverse to each other: sum_mu co[a][mu] fr[b][mu] = delta_ab;
//! * the metric g_{mu nu} = sum_a eta_aa e^a_mu e^a_nu and its inverse g^{mu nu} = sum_a eta_aa e_a^mu e_a^nu;
//! * the Christoffel symbols Gamma^rho_{mu nu} = 1/2 g^{rho la} (d_mu g_{la nu} + d_nu g_{la mu} - d_la g_{mu nu});
//! * the spin connection omega_mu^a_b = sum_nu e^a_nu (d_mu e_b^nu + sum_la Gamma^nu_{mu la} e_b^la), which is the
//!   tetrad postulate nabla_mu e_b^nu = d_mu e_b^nu + Gamma^nu_{mu la} e_b^la - omega_mu^c_b e_c^nu = 0; the
//!   coefficients omega_{mu ab} = eta_aa omega_mu^a_b are antisymmetric in a, b;
//! * the spinor connection Omega_mu = 1/4 sum_{ab} omega_{mu ab} gamma^{ab}_LL, with gamma^{ab}_LL =
//!   1/2 [gamma^a_LL, gamma^b_LL] (frame indices up): it is the spin connection Gamma^LL_mu of GEE and minus Lord's
//!   auxiliary field Gamma_mu; the covariant derivative of a spinor is nabla_mu psi = d_mu psi + Omega_mu psi;
//! * the Riemann tensor in the convention of Landau and Lifshitz, R^rho_{sigma mu nu} = d_mu Gamma^rho_{nu sigma}
//!   - d_nu Gamma^rho_{mu sigma} + Gamma^rho_{mu la} Gamma^la_{nu sigma} - Gamma^rho_{nu la} Gamma^la_{mu sigma}, so
//!   that [nabla_mu, nabla_nu] v^rho = R^rho_{sigma mu nu} v^sigma; the Ricci tensor R_{sigma nu} = R^rho_{sigma rho nu};
//!   the scalar curvature R = g^{sigma nu} R_{sigma nu}; the Einstein tensor G_{mu nu} = R_{mu nu} - 1/2 g_{mu nu} R.
//!
//! The brochon Psi_LL has eight components, numbered 2 d + s (Dirac index d = 0..3, isospin index s = 0 up, 1 down);
//! a Dirac matrix A acts on it as A (x) 1_2, an isospin matrix t as 1_4 (x) t (module `dirac`).  Its covariant
//! derivative in a curved space-time (Section 15 of the manuscript) is
//!     nabla_mu Psi = d_mu Psi + (Omega_mu (x) 1_2) Psi + i g^v (1_4 (x) W_mu) Psi + i g^F B_mu Psi,
//! with W_mu = sum_iota W^iota_mu tau_iota / 2.
//!
//! The fields are numbers of the type D4 (a value with its exact first and second derivatives at one point).  The
//! coframe must carry exact second derivatives; everything built from its first derivatives (Christoffel symbols, spin
//! connection, Omega) is then valid to first order (its value and first derivatives are exact), and the curvature is
//! valid at the value.  The same holds for the gauge fields and their field strengths.
use crate::c64::{c, C64};
use crate::cmat::CMat;
use crate::dirac;
use crate::dual::{DMat, D4};
use crate::lord::eta;

/// a tensor with two indices (4 x 4) of dual numbers
pub type M4 = [[D4; 4]; 4];
/// a tensor with three indices of dual numbers
pub type T3 = [[[D4; 4]; 4]; 4];
/// a tensor with four indices of complex numbers (values only)
pub type V4 = [[[[C64; 4]; 4]; 4]; 4];

/// the zero tensor with two indices
pub fn m4_zero() -> M4 {
    [[D4::ZERO; 4]; 4]
}
/// the zero tensor with three indices
pub fn t3_zero() -> T3 {
    [[[D4::ZERO; 4]; 4]; 4]
}

/// the derivative d/dx^p of a dual number, as a dual number that is valid to first order: its value is the first
/// derivative and its first derivatives are the second derivatives (its second derivatives would need third
/// derivatives and are set to zero)
pub fn shift(x: D4, p: usize) -> D4 {
    let mut r = D4::cst(x.d[p]);
    for q in 0..4 {
        r.d[q] = x.dd[p][q];
    }
    r
}

/// the determinant of the 3 x 3 submatrix of m with the rows r and the columns k
fn det3(m: &M4, r: [usize; 3], k: [usize; 3]) -> D4 {
    let a = |i: usize, j: usize| m[r[i]][k[j]];
    a(0, 0) * (a(1, 1) * a(2, 2) - a(1, 2) * a(2, 1)) - a(0, 1) * (a(1, 0) * a(2, 2) - a(1, 2) * a(2, 0))
        + a(0, 2) * (a(1, 0) * a(2, 1) - a(1, 1) * a(2, 0))
}
/// the indices 0..3 without i
fn others(i: usize) -> [usize; 3] {
    let mut o = [0usize; 3];
    let mut n = 0;
    for j in 0..4 {
        if j != i {
            o[n] = j;
            n += 1;
        }
    }
    o
}
/// the determinant of a 4 x 4 matrix of dual numbers (expansion along the first row)
pub fn det4(m: &M4) -> D4 {
    let mut s = D4::ZERO;
    for j in 0..4 {
        let t = m[0][j] * det3(m, others(0), others(j));
        s = if j % 2 == 0 { s + t } else { s - t };
    }
    s
}
/// the inverse of a 4 x 4 matrix of dual numbers: the adjugate (transposed cofactors) divided by the determinant
pub fn inv4(m: &M4) -> M4 {
    let di = det4(m).inv();
    let mut r = m4_zero();
    for i in 0..4 {
        for j in 0..4 {
            let cof = det3(m, others(i), others(j));
            let cof = if (i + j) % 2 == 0 { cof } else { -cof };
            r[j][i] = cof * di;
        }
    }
    r
}

/// the flat Dirac matrix gamma^a_LL of the chiral basis (frame index up)
pub fn gam_ll(a: usize) -> CMat {
    dirac::gamma(a)
}
/// gamma^{ab}_LL = 1/2 [gamma^a_LL, gamma^b_LL] (frame indices up)
pub fn gam_ab(a: usize, b: usize) -> CMat {
    gam_ll(a).mul(&gam_ll(b)).sub(&gam_ll(b).mul(&gam_ll(a))).scale_r(0.5)
}

/// The geometry at one point, computed from the coframe.
pub struct Geom {
    /// the coframe e^a_mu (co[a][mu]), exact to second order
    pub co: M4,
    /// the frame e_a^mu (fr[a][mu]): Lord's reference vectors h_a^mu
    pub fr: M4,
    /// the metric g_{mu nu} and its inverse g^{mu nu}
    pub g: M4,
    pub gi: M4,
    /// e = det(e^a_mu) = sqrt(-det g) (positive for the tetrads used here)
    pub sqrtg: D4,
    /// the Christoffel symbols chr[rho][mu][nu] = Gamma^rho_{mu nu}, valid to first order
    pub chr: T3,
    /// the spin connection om[mu][a][b] = omega_mu^a_b, valid to first order
    pub om: T3,
    /// the Dirac matrices of the curved space-time gamma^mu_LG = e_a^mu gamma^a_LL (4 x 4)
    pub gam: Vec<DMat>,
    /// the spinor connection Omega_mu = 1/4 omega_{mu ab} gamma^{ab}_LL (4 x 4), valid to first order
    pub spc: Vec<DMat>,
}

impl Geom {
    /// the geometry of a coframe that carries exact second derivatives
    pub fn new(co: M4) -> Geom {
        let einv = inv4(&co); // sum_mu co[a][mu] einv[mu][b] = delta_ab
        let mut fr = m4_zero();
        for a in 0..4 {
            for mu in 0..4 {
                fr[a][mu] = einv[mu][a];
            }
        }
        let mut g = m4_zero();
        let mut gi = m4_zero();
        for mu in 0..4 {
            for nu in 0..4 {
                for a in 0..4 {
                    let s = D4::re(eta(a, a));
                    g[mu][nu] = g[mu][nu] + s * co[a][mu] * co[a][nu];
                    gi[mu][nu] = gi[mu][nu] + s * fr[a][mu] * fr[a][nu];
                }
            }
        }
        let sqrtg = det4(&co);
        // the derivatives of the metric, dg[la][mu][nu] = d_la g_{mu nu} (valid to first order)
        let mut dg = t3_zero();
        for la in 0..4 {
            for mu in 0..4 {
                for nu in 0..4 {
                    dg[la][mu][nu] = shift(g[mu][nu], la);
                }
            }
        }
        let mut chr = t3_zero();
        for rho in 0..4 {
            for mu in 0..4 {
                for nu in 0..4 {
                    let mut s = D4::ZERO;
                    for la in 0..4 {
                        s = s + gi[rho][la] * (dg[mu][la][nu] + dg[nu][la][mu] - dg[la][mu][nu]);
                    }
                    chr[rho][mu][nu] = s * D4::re(0.5);
                }
            }
        }
        // the spin connection from the tetrad postulate
        let mut om = t3_zero();
        for mu in 0..4 {
            for a in 0..4 {
                for b in 0..4 {
                    let mut s = D4::ZERO;
                    for nu in 0..4 {
                        let mut t = shift(fr[b][nu], mu);
                        for la in 0..4 {
                            t = t + chr[nu][mu][la] * fr[b][la];
                        }
                        s = s + co[a][nu] * t;
                    }
                    om[mu][a][b] = s;
                }
            }
        }
        let gam: Vec<DMat> = (0..4)
            .map(|mu| {
                let mut m = DMat::new(4, 4);
                for a in 0..4 {
                    m = m.add(&DMat::from_cmat(&gam_ll(a)).scale(fr[a][mu]));
                }
                m
            })
            .collect();
        let spc: Vec<DMat> = (0..4)
            .map(|mu| {
                let mut m = DMat::new(4, 4);
                for a in 0..4 {
                    for b in 0..4 {
                        if a != b {
                            let w = om[mu][a][b] * D4::re(0.25 * eta(a, a));
                            m = m.add(&DMat::from_cmat(&gam_ab(a, b)).scale(w));
                        }
                    }
                }
                m
            })
            .collect();
        Geom { co, fr, g, gi, sqrtg, chr, om, gam, spc }
    }

    /// the coefficients omega_{mu ab} = eta_aa omega_mu^a_b (first frame index lowered)
    pub fn om_low(&self, mu: usize, a: usize, b: usize) -> D4 {
        self.om[mu][a][b] * D4::re(eta(a, a))
    }

    /// the Riemann tensor r[rho][sigma][mu][nu] = R^rho_{sigma mu nu} (values) from the Christoffel symbols
    pub fn riemann(&self) -> V4 {
        let ch = &self.chr;
        let mut r = [[[[C64::ZERO; 4]; 4]; 4]; 4];
        for rho in 0..4 {
            for sg in 0..4 {
                for mu in 0..4 {
                    for nu in 0..4 {
                        let mut s = ch[rho][nu][sg].d[mu] - ch[rho][mu][sg].d[nu];
                        for la in 0..4 {
                            s = s + ch[rho][mu][la].v * ch[la][nu][sg].v - ch[rho][nu][la].v * ch[la][mu][sg].v;
                        }
                        r[rho][sg][mu][nu] = s;
                    }
                }
            }
        }
        r
    }
    /// the curvature of the spin connection rs[a][b][mu][nu] = R^a_{b mu nu} (values) =
    /// d_mu omega_nu^a_b - d_nu omega_mu^a_b + omega_mu^a_c omega_nu^c_b - omega_nu^a_c omega_mu^c_b
    pub fn spin_riemann(&self) -> V4 {
        let w = &self.om;
        let mut r = [[[[C64::ZERO; 4]; 4]; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                for mu in 0..4 {
                    for nu in 0..4 {
                        let mut s = w[nu][a][b].d[mu] - w[mu][a][b].d[nu];
                        for cc in 0..4 {
                            s = s + w[mu][a][cc].v * w[nu][cc][b].v - w[nu][a][cc].v * w[mu][cc][b].v;
                        }
                        r[a][b][mu][nu] = s;
                    }
                }
            }
        }
        r
    }
    /// the Ricci tensor R_{sigma nu} = R^rho_{sigma rho nu} (values)
    pub fn ricci(&self, riem: &V4) -> [[C64; 4]; 4] {
        let mut r = [[C64::ZERO; 4]; 4];
        for sg in 0..4 {
            for nu in 0..4 {
                for rho in 0..4 {
                    r[sg][nu] += riem[rho][sg][rho][nu];
                }
            }
        }
        r
    }
    /// the scalar curvature R = g^{sigma nu} R_{sigma nu} (value)
    pub fn scalar(&self, ric: &[[C64; 4]; 4]) -> C64 {
        let mut s = C64::ZERO;
        for sg in 0..4 {
            for nu in 0..4 {
                s += self.gi[sg][nu].v * ric[sg][nu];
            }
        }
        s
    }
    /// the Dirac matrix gamma_mu = g_{mu nu} gamma^nu with a lower index (4 x 4)
    pub fn gam_low(&self, mu: usize) -> DMat {
        let mut m = DMat::new(4, 4);
        for nu in 0..4 {
            m = m.add(&self.gam[nu].scale(self.g[mu][nu]));
        }
        m
    }
    /// the divergence nabla_mu v^mu = d_mu v^mu + Gamma^mu_{mu la} v^la of a vector field that is valid to first
    /// order (value)
    pub fn div_vec(&self, v: &[D4; 4]) -> C64 {
        let mut s = C64::ZERO;
        for mu in 0..4 {
            s += v[mu].d[mu];
            for la in 0..4 {
                s += self.chr[mu][mu][la].v * v[la].v;
            }
        }
        s
    }
    /// the divergence nabla_mu T^{mu nu} of a tensor with upper indices that is valid to first order (values)
    pub fn div_t2(&self, t: &M4) -> [C64; 4] {
        let mut r = [C64::ZERO; 4];
        for nu in 0..4 {
            let mut s = C64::ZERO;
            for mu in 0..4 {
                s += t[mu][nu].d[mu];
                for la in 0..4 {
                    s += self.chr[mu][mu][la].v * t[la][nu].v + self.chr[nu][mu][la].v * t[mu][la].v;
                }
            }
            r[nu] = s;
        }
        r
    }
    /// raises both indices of a tensor with lower indices: T^{mu nu} = g^{mu al} g^{nu be} T_{al be}
    pub fn raise2(&self, t: &M4) -> M4 {
        let mut r = m4_zero();
        for mu in 0..4 {
            for nu in 0..4 {
                let mut s = D4::ZERO;
                for al in 0..4 {
                    for be in 0..4 {
                        s = s + self.gi[mu][al] * self.gi[nu][be] * t[al][be];
                    }
                }
                r[mu][nu] = s;
            }
        }
        r
    }
}

// ------------------------------------------------------------------------------------------------------------------
// gauge fields of SU(2)_v x U(1)^F_LL
// ------------------------------------------------------------------------------------------------------------------

/// The gauge fields of SU(2)_v (w[iota-1][mu] = W^iota_{SF,mu}, lower index) and of U(1)^F_LL (b[mu] = B_{SF,mu}),
/// with their couplings.
#[derive(Clone)]
pub struct Gauge {
    pub w: [[D4; 4]; 3],
    pub b: [D4; 4],
    pub gv: f64,
    pub gf: f64,
}

impl Gauge {
    /// no gauge fields
    pub fn zero(gv: f64, gf: f64) -> Gauge {
        Gauge { w: [[D4::ZERO; 4]; 3], b: [D4::ZERO; 4], gv, gf }
    }
    /// the matrix W_mu = sum_iota W^iota_mu tau_iota / 2 (2 x 2)
    pub fn w2(&self, mu: usize) -> DMat {
        let mut m = DMat::new(2, 2);
        for i in 0..3 {
            m = m.add(&DMat::from_cmat(&dirac::sigma(i + 1).scale_r(0.5)).scale(self.w[i][mu]));
        }
        m
    }
    /// the internal connection i g^v (1_4 (x) W_mu) + i g^F B_mu on the eight components (8 x 8)
    pub fn conn8(&self, mu: usize) -> DMat {
        let iso = DMat::from_cmat(&CMat::identity(4)).kron(&self.w2(mu)).scale(D4::cst(c(0.0, self.gv)));
        let mut r = iso;
        let bterm = self.b[mu] * D4::cst(c(0.0, self.gf));
        for i in 0..8 {
            let x = r.get(i, i) + bterm;
            r.set(i, i, x);
        }
        r
    }
    /// the field strengths (4.7) of the earlier hypotheses, wf[iota-1][mu][nu] = W^iota_{mu nu} = d_mu W^iota_nu
    /// - d_nu W^iota_mu - g^v eps_{iota i' i''} W^i'_mu W^i''_nu, valid to first order
    pub fn w_fs(&self) -> [M4; 3] {
        let mut r = [m4_zero(); 3];
        for i in 0..3 {
            for mu in 0..4 {
                for nu in 0..4 {
                    let mut s = shift(self.w[i][nu], mu) - shift(self.w[i][mu], nu);
                    for j in 0..3 {
                        for k in 0..3 {
                            let e = dirac::levi(i, j, k);
                            if e != 0.0 {
                                s = s - D4::re(self.gv * e) * self.w[j][mu] * self.w[k][nu];
                            }
                        }
                    }
                    r[i][mu][nu] = s;
                }
            }
        }
        r
    }
    /// the field strength of U(1)^F_LL, B_{mu nu} = d_mu B_nu - d_nu B_mu, valid to first order
    pub fn b_fs(&self) -> M4 {
        let mut r = m4_zero();
        for mu in 0..4 {
            for nu in 0..4 {
                r[mu][nu] = shift(self.b[nu], mu) - shift(self.b[mu], nu);
            }
        }
        r
    }
    /// the matrix of the internal field strengths i g^v (1_4 (x) W_{mu nu}) + i g^F B_{mu nu} on the eight
    /// components (values), with W_{mu nu} = sum_iota W^iota_{mu nu} tau_iota / 2
    pub fn fs8(&self, wf: &[M4; 3], bf: &M4, mu: usize, nu: usize) -> CMat {
        let mut w = CMat::zeros(2);
        for i in 0..3 {
            w = w.add(&dirac::sigma(i + 1).scale(wf[i][mu][nu].v * 0.5));
        }
        let mut r = CMat::identity(4).kron(&w).scale(c(0.0, self.gv));
        for i in 0..8 {
            let x = r.get(i, i) + bf[mu][nu].v * c(0.0, self.gf);
            r.set(i, i, x);
        }
        r
    }
}

// ------------------------------------------------------------------------------------------------------------------
// the brochon: eight components
// ------------------------------------------------------------------------------------------------------------------

/// the 8 x 8 matrix of a 4 x 4 Dirac matrix of dual numbers acting on the Dirac index: A (x) 1_2
pub fn d8d(a: &DMat) -> DMat {
    a.kron(&DMat::from_cmat(&CMat::identity(2)))
}
/// the product of a matrix of dual numbers and a column of dual numbers
pub fn mv(m: &DMat, v: &[D4]) -> Vec<D4> {
    (0..m.rows).map(|i| (0..m.cols).fold(D4::ZERO, |s, j| s + m.get(i, j) * v[j])).collect()
}
/// the product of a constant matrix and a column of dual numbers
pub fn cmv(m: &CMat, v: &[D4]) -> Vec<D4> {
    (0..m.rows).map(|i| (0..m.cols).fold(D4::ZERO, |s, j| s + v[j].scale(m.get(i, j)))).collect()
}
/// the sum and the difference of two columns
pub fn vadd(a: &[D4], b: &[D4]) -> Vec<D4> {
    a.iter().zip(b).map(|(x, y)| *x + *y).collect()
}
pub fn vsub(a: &[D4], b: &[D4]) -> Vec<D4> {
    a.iter().zip(b).map(|(x, y)| *x - *y).collect()
}
/// a column times a dual number
pub fn vscale(a: &[D4], s: D4) -> Vec<D4> {
    a.iter().map(|x| *x * s).collect()
}
/// the derivative d/dx^p of every component, valid to first order
pub fn vshift(a: &[D4], p: usize) -> Vec<D4> {
    a.iter().map(|x| shift(*x, p)).collect()
}
/// the largest modulus of the values of the components
pub fn vmax0(a: &[D4]) -> f64 {
    a.iter().fold(0.0, |s, x| s.max(x.v.abs()))
}
/// the largest modulus of the values and of the first derivatives of the components
pub fn vmax1(a: &[D4]) -> f64 {
    a.iter().fold(0.0, |s, x| {
        let mut t = s.max(x.v.abs());
        for p in 0..4 {
            t = t.max(x.d[p].abs());
        }
        t
    })
}

/// the covariant derivative of the brochon, nabla_mu Psi = d_mu Psi + (Omega_mu (x) 1_2) Psi + i g^v (1_4 (x) W_mu)
/// Psi + i g^F B_mu Psi (valid to one order less than Psi and the fields)
pub fn nabla8(geo: &Geom, gau: &Gauge, psi: &[D4], mu: usize) -> Vec<D4> {
    let conn = d8d(&geo.spc[mu]).add(&gau.conn8(mu));
    vadd(&vshift(psi, mu), &mv(&conn, psi))
}
/// the Dirac operator of the brochon, sum_mu (gamma^mu_LG (x) 1_2) nabla_mu Psi
pub fn dslash8(geo: &Geom, gau: &Gauge, psi: &[D4]) -> Vec<D4> {
    let mut r = vec![D4::ZERO; 8];
    for mu in 0..4 {
        r = vadd(&r, &mv(&d8d(&geo.gam[mu]), &nabla8(geo, gau, psi, mu)));
    }
    r
}
/// the residual of the field equation of the free brochon in the gauge fields, E = i Dslash Psi - m Psi
pub fn dirac_resid(geo: &Geom, gau: &Gauge, psi: &[D4], m: f64) -> Vec<D4> {
    vsub(&vscale(&dslash8(geo, gau, psi), D4::cst(C64::I)), &vscale(psi, D4::re(m)))
}
/// the Dirac adjoint Psi-bar = Psi^dag (gamma^0_LL (x) 1_2), as a row of dual numbers (the matrix gamma^0 of the frame,
/// which does not depend on the point)
pub fn bar8(psi: &[D4]) -> Vec<D4> {
    let g0 = dirac::d8(&gam_ll(0));
    (0..8).map(|i| (0..8).fold(D4::ZERO, |s, j| s + psi[j].conj().scale(g0.get(j, i)))).collect()
}
/// the bilinear form row . M . column
pub fn bil(row: &[D4], m: &DMat, col: &[D4]) -> D4 {
    let mc = mv(m, col);
    row.iter().zip(&mc).fold(D4::ZERO, |s, (x, y)| s + *x * *y)
}
/// the imaginary part of a dual number (Z - Z*)/(2i), with its derivatives
pub fn im(z: D4) -> D4 {
    (z - z.conj()) * D4::cst(c(0.0, -0.5))
}

/// the currents of the brochon: the fermion-number current j^{F,mu} = Psi-bar gamma^mu Psi and the isospin currents
/// j^{iota,mu} = Psi-bar gamma^mu T^iota Psi (upper indices; valid to the order of Psi)
pub fn currents(geo: &Geom, psi: &[D4]) -> ([D4; 4], [[D4; 4]; 3]) {
    let pb = bar8(psi);
    let mut jf = [D4::ZERO; 4];
    let mut ji = [[D4::ZERO; 4]; 3];
    for mu in 0..4 {
        jf[mu] = bil(&pb, &d8d(&geo.gam[mu]), psi);
        for i in 0..3 {
            let m = geo.gam[mu].kron(&DMat::from_cmat(&dirac::sigma(i + 1).scale_r(0.5)));
            ji[i][mu] = bil(&pb, &m, psi);
        }
    }
    (jf, ji)
}

/// the Lagrangian density of the brochon (divided by e), in the symmetric form L = i/2 (Psi-bar gamma^mu nabla_mu Psi
/// - (nabla_mu Psi)-bar gamma^mu Psi) - m Psi-bar Psi = -Im(Psi-bar gamma^mu nabla_mu Psi) - m Psi-bar Psi
pub fn lagr8(geo: &Geom, gau: &Gauge, psi: &[D4], m: f64) -> D4 {
    let pb = bar8(psi);
    let mut y = D4::ZERO;
    for mu in 0..4 {
        y = y + bil(&pb, &d8d(&geo.gam[mu]), &nabla8(geo, gau, psi, mu));
    }
    let id8 = DMat::from_cmat(&CMat::identity(8));
    -im(y) - bil(&pb, &id8, psi) * D4::re(m)
}

/// the symmetric energy-momentum tensor of the brochon with lower indices (Section 16 of the manuscript):
/// T_{mu nu} = i/4 (Psi-bar gamma_mu nabla_nu Psi + Psi-bar gamma_nu nabla_mu Psi - (nabla_mu Psi)-bar gamma_nu Psi
/// - (nabla_nu Psi)-bar gamma_mu Psi) - g_{mu nu} L = -1/2 Im(X_{mu nu} + X_{nu mu}) - g_{mu nu} L, with
/// X_{mu nu} = Psi-bar gamma_mu nabla_nu Psi; valid to one order less than Psi and the fields
pub fn stress8(geo: &Geom, gau: &Gauge, psi: &[D4], m: f64) -> M4 {
    let pb = bar8(psi);
    let nab: Vec<Vec<D4>> = (0..4).map(|nu| nabla8(geo, gau, psi, nu)).collect();
    let mut x = m4_zero();
    for mu in 0..4 {
        let gl = d8d(&geo.gam_low(mu));
        for nu in 0..4 {
            x[mu][nu] = bil(&pb, &gl, &nab[nu]);
        }
    }
    let l = lagr8(geo, gau, psi, m);
    let mut t = m4_zero();
    for mu in 0..4 {
        for nu in 0..4 {
            t[mu][nu] = im(x[mu][nu] + x[nu][mu]) * D4::re(-0.5) - geo.g[mu][nu] * l;
        }
    }
    t
}

/// the generalized Lorentz force f^nu = g^v W^iota nu_la j^{iota la} + g^F B^nu_la j^{F la} of the gauge fields on the
/// currents of the brochon (values), with W^{iota nu}_la = g^{nu al} W^iota_{al la}
pub fn lorentz_force(geo: &Geom, gau: &Gauge, psi: &[D4]) -> [C64; 4] {
    let (jf, ji) = currents(geo, psi);
    let wf = gau.w_fs();
    let bf = gau.b_fs();
    let mut f = [C64::ZERO; 4];
    for nu in 0..4 {
        let mut s = C64::ZERO;
        for al in 0..4 {
            for la in 0..4 {
                let gnal = geo.gi[nu][al].v;
                for i in 0..3 {
                    s += gnal * wf[i][al][la].v * ji[i][la].v * gau.gv;
                }
                s += gnal * bf[al][la].v * jf[la].v * gau.gf;
            }
        }
        f[nu] = s;
    }
    f
}

/// the energy-momentum tensor of the Yang-Mills fields (5.3) with lower indices, T_{mu nu} = sum_iota
/// (-W^iota_{mu la} W^iota_nu^la + 1/4 g_{mu nu} W^iota_{al be} W^{iota al be}) + the same for B (valid to first order)
pub fn stress_ym(geo: &Geom, wf: &[M4; 3], bf: &M4) -> M4 {
    let mut fields: Vec<&M4> = wf.iter().collect();
    fields.push(bf);
    let mut t = m4_zero();
    for f in fields {
        // F^{al be} with both indices raised, and F_nu^la with the second raised
        let mut fup = m4_zero();
        let mut fmix = m4_zero();
        for al in 0..4 {
            for be in 0..4 {
                let mut s = D4::ZERO;
                let mut sm = D4::ZERO;
                for ka in 0..4 {
                    sm = sm + f[al][ka] * geo.gi[ka][be];
                    for la in 0..4 {
                        s = s + geo.gi[al][ka] * geo.gi[be][la] * f[ka][la];
                    }
                }
                fup[al][be] = s;
                fmix[al][be] = sm;
            }
        }
        let mut f2 = D4::ZERO;
        for al in 0..4 {
            for be in 0..4 {
                f2 = f2 + f[al][be] * fup[al][be];
            }
        }
        for mu in 0..4 {
            for nu in 0..4 {
                let mut s = D4::ZERO;
                for la in 0..4 {
                    s = s + f[mu][la] * fmix[nu][la];
                }
                t[mu][nu] = t[mu][nu] - s + geo.g[mu][nu] * f2 * D4::re(0.25);
            }
        }
    }
    t
}

/// the left side of the Yang-Mills equations in the curved space-time, y[iota-1][la] = nabla_mu W^{iota mu la}
/// (with the adjoint term -g^v eps W W) and y[3][la] = nabla_mu B^{mu la} (values); the equations are
/// y = g^v j^iota and y = g^F j^F
pub fn ym_lhs(geo: &Geom, gau: &Gauge, wf: &[M4; 3], bf: &M4) -> [[C64; 4]; 4] {
    // the field strengths with upper indices, valid to first order
    let up = |f: &M4| -> M4 {
        let mut r = m4_zero();
        for mu in 0..4 {
            for la in 0..4 {
                let mut s = D4::ZERO;
                for al in 0..4 {
                    for be in 0..4 {
                        s = s + geo.gi[mu][al] * geo.gi[la][be] * f[al][be];
                    }
                }
                r[mu][la] = s;
            }
        }
        r
    };
    let wup: Vec<M4> = wf.iter().map(up).collect();
    let bup = up(bf);
    let mut y = [[C64::ZERO; 4]; 4];
    for la in 0..4 {
        for i in 0..3 {
            // nabla_mu F^{mu la} of an antisymmetric tensor = (1/e) d_mu (e F^{mu la})
            let mut s = C64::ZERO;
            for mu in 0..4 {
                s += (geo.sqrtg * wup[i][mu][la]).d[mu] / geo.sqrtg.v;
                for j in 0..3 {
                    for k in 0..3 {
                        let e = dirac::levi(i, j, k);
                        if e != 0.0 {
                            s += -(gau.w[j][mu].v * wup[k][mu][la].v) * (gau.gv * e);
                        }
                    }
                }
            }
            y[i][la] = s;
        }
        let mut s = C64::ZERO;
        for mu in 0..4 {
            s += (geo.sqrtg * bup[mu][la]).d[mu] / geo.sqrtg.v;
        }
        y[3][la] = s;
    }
    y
}

// ------------------------------------------------------------------------------------------------------------------
// local Lorentz transformations
// ------------------------------------------------------------------------------------------------------------------

/// the Dirac form diag(S, (S^dag)^{-1}) of a field S of SL(2,C) (4 x 4)
pub fn s4(s: &DMat) -> DMat {
    DMat::block_diag(s, &s.dagger().inv2())
}
/// its inverse diag(S^{-1}, S^dag)
pub fn s4inv(s: &DMat) -> DMat {
    DMat::block_diag(&s.inv2(), &s.dagger())
}
/// the Lorentz matrix of a spinor matrix: Lambda^a_b with S4^{-1} gamma^a S4 = Lambda^a_b gamma^b, computed as
/// Lambda^a_b = 1/4 tr(S4^{-1} gamma^a S4 gamma_b), gamma_b = eta_bb gamma^b
pub fn lorentz_of_s4(s4m: &DMat, s4i: &DMat) -> M4 {
    let mut l = m4_zero();
    for a in 0..4 {
        let t = s4i.mul(&DMat::from_cmat(&gam_ll(a))).mul(s4m);
        for b in 0..4 {
            let p = t.mul(&DMat::from_cmat(&gam_ll(b).scale_r(eta(b, b))));
            let mut tr = D4::ZERO;
            for i in 0..4 {
                tr = tr + p.get(i, i);
            }
            l[a][b] = tr * D4::re(0.25);
        }
    }
    l
}

// ------------------------------------------------------------------------------------------------------------------
// the pairing term of the polar superfluid (Section 15.4 of the manuscript)
// ------------------------------------------------------------------------------------------------------------------

/// the constant 8 x 8 matrix (gamma^0_LL (x) 1_2) (K^3_LL)^dag of the pairing term, K^3_LL = C_LL gamma^ch_LL (x)
/// E_LL tau_3 (the scalar pair matrix along the third isospin direction)
pub fn pair_matrix() -> CMat {
    dirac::d8(&gam_ll(0)).mul(&dirac::k_scalar(3).dagger())
}
/// the antilinear map C Psi = (gamma^0 (x) 1_2) (K^3)^dag Psi^* of the pairing term
pub fn cmap(psi: &[D4]) -> Vec<D4> {
    let p = pair_matrix();
    let conj: Vec<D4> = psi.iter().map(|x| x.conj()).collect();
    cmv(&p, &conj)
}
/// the residual of the quasi-particle (Bogoliubov-de Gennes) equation of the polar superfluid,
/// E = i Dslash Psi - M Psi - D C Psi, with a real Dirac mass M and a complex pair amplitude D
pub fn bdg_resid(geo: &Geom, gau: &Gauge, psi: &[D4], mass: f64, dpair: C64) -> Vec<D4> {
    let e = dirac_resid(geo, gau, psi, mass);
    vsub(&e, &vscale(&cmap(psi), D4::cst(dpair)))
}

// ------------------------------------------------------------------------------------------------------------------
// local solutions
// ------------------------------------------------------------------------------------------------------------------

/// Completes the Taylor data of a spinor field at the point x0 so that a field equation of first order holds there
/// with its first derivatives: E(x0) = 0 and d_la E(x0) = 0 for la = 0..3.  The field equation must contain the time
/// derivative as i gamma^0_LG (x) 1_2 d_0 Psi plus terms that do not contain d_0 Psi (true for the Dirac and the
/// Bogoliubov-de Gennes equations, whose other terms contain at most first derivatives).  The value, the space
/// derivatives d_j Psi and the second space derivatives d_j d_k Psi of `psi` are kept; d_0 Psi, d_0 d_j Psi and
/// d_0 d_0 Psi are determined, in this order, each from a linear equation with the matrix i gamma^0_LG (x) 1_2.
/// Returns the largest residual (values and first derivatives) after the completion.
pub fn complete_local(psi: &mut Vec<D4>, g0: &CMat, resid: &dyn Fn(&[D4]) -> Vec<D4>) -> f64 {
    // (i gamma^0)^{-1} = -i gamma^0 / g^{00}, since (gamma^0)^2 = g^{00}
    let g00 = (g0.mul(g0)).get(0, 0);
    let ig0inv = dirac::d8(&g0.scale(c(0.0, -1.0) / g00));
    let solve = |r: &[C64]| -> Vec<C64> { ig0inv.matvec(r).iter().map(|x| -*x).collect() };
    // step 1: d_0 Psi from E(x0) = 0
    let r = resid(psi);
    let dl = solve(&r.iter().map(|x| x.v).collect::<Vec<_>>());
    for i in 0..psi.len() {
        psi[i].d[0] += dl[i];
    }
    // step 2: d_0 d_j Psi from d_j E(x0) = 0
    let r = resid(psi);
    for j in 1..4 {
        let dl = solve(&r.iter().map(|x| x.d[j]).collect::<Vec<_>>());
        for i in 0..psi.len() {
            psi[i].dd[0][j] += dl[i];
            psi[i].dd[j][0] += dl[i];
        }
    }
    // step 3: d_0 d_0 Psi from d_0 E(x0) = 0
    let r = resid(psi);
    let dl = solve(&r.iter().map(|x| x.d[0]).collect::<Vec<_>>());
    for i in 0..psi.len() {
        psi[i].dd[0][0] += dl[i];
    }
    vmax1(&resid(psi))
}

// ------------------------------------------------------------------------------------------------------------------
// random smooth fields
// ------------------------------------------------------------------------------------------------------------------

/// a smooth real function of the four coordinates at the point x: a sum of three terms a_k sin(k_k . x + phi_k) with
/// pseudo-random real amplitudes of size `amp`, wave vectors of size about 1 and phases
pub fn rand_real(rng: &mut crate::rng::Rng, x: &[f64; 4], amp: f64) -> D4 {
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
/// a random coframe e^a_mu = delta^a_mu + amp r^a_mu(x) with smooth real functions r (a curved space-time whose
/// curvature is of the order amp)
pub fn rand_coframe(rng: &mut crate::rng::Rng, x: &[f64; 4], amp: f64) -> M4 {
    let mut co = m4_zero();
    for a in 0..4 {
        for mu in 0..4 {
            co[a][mu] = rand_real(rng, x, amp) + D4::re(if a == mu { 1.0 } else { 0.0 });
        }
    }
    co
}
/// random gauge fields W^iota_mu, B_mu (smooth, real, of size amp) with the couplings gv, gf
pub fn rand_gauge(rng: &mut crate::rng::Rng, x: &[f64; 4], amp: f64, gv: f64, gf: f64) -> Gauge {
    let mut g = Gauge::zero(gv, gf);
    for i in 0..3 {
        for mu in 0..4 {
            g.w[i][mu] = rand_real(rng, x, amp);
        }
    }
    for mu in 0..4 {
        g.b[mu] = rand_real(rng, x, amp);
    }
    g
}
/// a smooth random field of SL(2,C): exp of a traceless matrix of smooth complex functions of size amp
pub fn rand_sl2c_field(rng: &mut crate::rng::Rng, x: &[f64; 4], amp: f64) -> DMat {
    let mut m = DMat::new(2, 2);
    let a = crate::lord::rand_fn(rng, x, amp);
    m.set(0, 0, a);
    m.set(0, 1, crate::lord::rand_fn(rng, x, amp));
    m.set(1, 0, crate::lord::rand_fn(rng, x, amp));
    m.set(1, 1, -a);
    m.exp_traceless2()
}
/// a random spinor field of eight smooth complex components of size amp
pub fn rand_spinor8(rng: &mut crate::rng::Rng, x: &[f64; 4], amp: f64) -> Vec<D4> {
    (0..8).map(|_| crate::lord::rand_fn(rng, x, amp)).collect()
}

/// A smooth map X(y) = y + sum_k A_k sin(kappa_k . y + phi_k) of the coordinates (a change of coordinates of flat
/// space-time, invertible near the points used), with its Jacobian dX^b/dy^mu written out, so that both carry exact
/// second derivatives.
pub struct RandMap {
    pub amp: Vec<[f64; 4]>,
    pub kap: Vec<[f64; 4]>,
    pub ph: Vec<f64>,
}
impl RandMap {
    pub fn new(rng: &mut crate::rng::Rng, size: f64, nterms: usize) -> RandMap {
        let mut amp = vec![];
        let mut kap = vec![];
        let mut ph = vec![];
        for _ in 0..nterms {
            amp.push([size * rng.unit(), size * rng.unit(), size * rng.unit(), size * rng.unit()]);
            kap.push([rng.unit(), rng.unit(), rng.unit(), rng.unit()]);
            ph.push(std::f64::consts::PI * rng.unit());
        }
        RandMap { amp, kap, ph }
    }
    fn arg(&self, k: usize, y: &[f64; 4]) -> D4 {
        let mut a = D4::re(self.ph[k]);
        for p in 0..4 {
            a = a + D4::coord(y, p) * D4::re(self.kap[k][p]);
        }
        a
    }
    /// X^b(y)
    pub fn x(&self, y: &[f64; 4]) -> [D4; 4] {
        let mut r = [D4::ZERO; 4];
        for b in 0..4 {
            r[b] = D4::coord(y, b);
        }
        for k in 0..self.ph.len() {
            let s = self.arg(k, y).sin();
            for b in 0..4 {
                r[b] = r[b] + s * D4::re(self.amp[k][b]);
            }
        }
        r
    }
    /// the Jacobian j[b][mu] = dX^b / dy^mu
    pub fn jac(&self, y: &[f64; 4]) -> M4 {
        let mut j = m4_zero();
        for b in 0..4 {
            j[b][b] = D4::re(1.0);
        }
        for k in 0..self.ph.len() {
            let cs = self.arg(k, y).cos();
            for b in 0..4 {
                for mu in 0..4 {
                    j[b][mu] = j[b][mu] + cs * D4::re(self.amp[k][b] * self.kap[k][mu]);
                }
            }
        }
        j
    }
}

// ------------------------------------------------------------------------------------------------------------------
// the Volkov solution: the brochon in a plane wave of the zeugon (flat space-time)
// ------------------------------------------------------------------------------------------------------------------

/// The data of a plane wave of the zeugon A_a(x) = a_a cos(k . x) (k null, k . a = 0; flat components, lower index
/// a_a = eta_aa a^a) and of a brochon in it: for each isospin component s (charge Q = +1/2, -1/2) a momentum p_s with
/// p_s . p_s = m^2 and a spinor u_s with (p-slash_s - m) u_s = 0.
pub struct Volkov {
    pub k: [f64; 4],
    pub a: [f64; 4],
    pub p: [[f64; 4]; 2],
    pub u: [Vec<C64>; 2],
    pub m: f64,
    pub gv: f64,
}

/// the Minkowski product of two vectors with upper indices
pub fn mdot(a: &[f64; 4], b: &[f64; 4]) -> f64 {
    a[0] * b[0] - a[1] * b[1] - a[2] * b[2] - a[3] * b[3]
}
/// the slash of a vector with upper indices, v-slash = gamma^a eta_ab v^b = gamma^0 v^0 - gamma^j v^j (4 x 4)
pub fn slash(v: &[f64; 4]) -> CMat {
    let mut m = CMat::zeros(4);
    for a in 0..4 {
        m = m.add(&gam_ll(a).scale_r(eta(a, a) * v[a]));
    }
    m
}

impl Volkov {
    /// random data: a null wave vector of size about 1, a polarization orthogonal to it, and for each isospin
    /// component a momentum on the mass shell (m may be 0) and a spinor of the kind (p-slash + m) w (or p-slash w for
    /// m = 0), which satisfies (p-slash - m) u = 0 because p-slash p-slash = p . p
    pub fn random(rng: &mut crate::rng::Rng, m: f64, amp: f64, gv: f64) -> Volkov {
        let dir = |rng: &mut crate::rng::Rng| -> [f64; 3] {
            let v = [rng.unit(), rng.unit(), rng.unit()];
            let n = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            [v[0] / n, v[1] / n, v[2] / n]
        };
        let kd = dir(rng);
        let kk = 0.6 + 0.3 * rng.unit().abs();
        let k = [kk, kk * kd[0], kk * kd[1], kk * kd[2]];
        // a polarization: a spatial vector orthogonal to the direction of k (then k . a = 0 and a . a < 0)
        let r = dir(rng);
        let dot = r[0] * kd[0] + r[1] * kd[1] + r[2] * kd[2];
        let mut av = [r[0] - dot * kd[0], r[1] - dot * kd[1], r[2] - dot * kd[2]];
        let n = (av[0] * av[0] + av[1] * av[1] + av[2] * av[2]).sqrt();
        for j in 0..3 {
            av[j] *= amp / n;
        }
        let a = [0.0, av[0], av[1], av[2]];
        let mut p = [[0.0; 4]; 2];
        let mut u: [Vec<C64>; 2] = [vec![], vec![]];
        for s in 0..2 {
            let pd = dir(rng);
            let pm = 0.3 + 0.5 * rng.unit().abs();
            let e = (pm * pm + m * m).sqrt();
            p[s] = [e, pm * pd[0], pm * pd[1], pm * pd[2]];
            let w: Vec<C64> = (0..4).map(|_| c(rng.unit(), rng.unit())).collect();
            let op = slash(&p[s]).add(&CMat::identity(4).scale_r(m));
            u[s] = op.matvec(&w);
        }
        Volkov { k, a, p, u, m, gv }
    }
    /// the potential of the zeugon with a lower flat index, A_a(x) = eta_aa a^a cos(k . x), at the flat point x (dual)
    pub fn potential(&self, x: &[D4; 4]) -> [D4; 4] {
        let phi = self.phase_k(x);
        let cs = phi.cos();
        let mut r = [D4::ZERO; 4];
        for a in 0..4 {
            r[a] = cs * D4::re(eta(a, a) * self.a[a]);
        }
        r
    }
    fn phase_k(&self, x: &[D4; 4]) -> D4 {
        let mut s = D4::ZERO;
        for a in 0..4 {
            s = s + x[a] * D4::re(eta(a, a) * self.k[a]);
        }
        s
    }
    /// the Volkov solution of (i gamma^a (d_a + i e A_a) - m) psi = 0 for each isospin component (charge e = g^v Q),
    /// psi_s = [1 + e/(2 k.p) k-slash a-slash cos(phi)] u_s exp(-i S_s), phi = k . x,
    /// S_s = p.x + e (p.a)/(k.p) sin(phi) - e^2 (a.a)/(2 k.p) (phi/2 + sin(2 phi)/4);
    /// returns the eight components Psi = sum_s psi_s (x) chi_s at the flat point x (dual)
    pub fn psi(&self, x: &[D4; 4]) -> Vec<D4> {
        let phi = self.phase_k(x);
        let (sn, cs) = (phi.sin(), phi.cos());
        let sn2 = (phi * D4::re(2.0)).sin();
        let ka = slash(&self.k).mul(&slash(&self.a));
        let mut out = vec![D4::ZERO; 8];
        for s in 0..2 {
            let q = if s == 0 { 0.5 } else { -0.5 };
            let e = self.gv * q;
            let kp = mdot(&self.k, &self.p[s]);
            let pa = mdot(&self.p[s], &self.a);
            let aa = mdot(&self.a, &self.a);
            let mut px = D4::ZERO;
            for b in 0..4 {
                px = px + x[b] * D4::re(eta(b, b) * self.p[s][b]);
            }
            let sph = px + sn * D4::re(e * pa / kp) - (phi * D4::re(0.5) + sn2 * D4::re(0.25)) * D4::re(e * e * aa / (2.0 * kp));
            let ph = (sph * D4::cst(c(0.0, -1.0))).exp();
            let kau = ka.matvec(&self.u[s]);
            for d in 0..4 {
                let comp = D4::cst(self.u[s][d]) + cs * D4::cst(kau[d].scale(e / (2.0 * kp)));
                out[2 * d + s] = comp * ph;
            }
        }
        out
    }
}
