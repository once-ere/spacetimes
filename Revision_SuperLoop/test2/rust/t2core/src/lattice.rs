//! The gauge-covariant lattice Dirac-Bogoliubov-de Gennes operator: the discretized mean-field
//! field equations of the self-interacting spinor field of Test 2.
//!
//! The lattice has n[0] x n[1] x n[2] sites with spacing `ell` (the same in the three directions);
//! site s = ix + n0 (iy + n1 iz).  On every site the Nambu mode function has 16 complex components:
//! the particle part psi_p (8 components, the ordering of Psi_LL) and the hole part psi_h (8).
//!
//! The particle row of the operator H_DB (with the covariant central difference and the Wilson term):
//!   (H w)_p(s) = sum_j [ -i alpha^j (U_j(s) psi_p(s+j) - U_j(s-j)^dag psi_p(s-j)) / (2 ell)
//!                        - (r_W / (2 ell)) beta (U_j(s) psi_p(s+j) + U_j(s-j)^dag psi_p(s-j) - 2 psi_p(s)) ]
//!                + m beta psi_p(s) - (V0(s) + mu) psi_p(s) + P(s) psi_h(s)
//! and the hole row:
//!   (H w)_h(s) = P(s)^dag psi_p(s) - conj( [h_lat - V0 - mu] conj(psi_h) )(s)
//! with the links U_j(s) = exp(-i ell (g_v A^iota_j tau_iota/2 + g_F A^F_j)) on the link from s to
//! s + j (2 x 2 isospin matrices; the Dirac index is untouched), the potential matrix
//! V0(s) = g_v A^iota_0 tau_iota/2 + g_F A^F_0 (2 x 2) and the pair potential P(s) = sum_iota d^iota(s) Kch_iota^dag.
//! Directions without a neighbour (Dirichlet boundary) contribute zero.
use crate::c64::{c, C64};
use crate::cmat::CMat;
use crate::model;

/// a 2 x 2 complex matrix [[m[0], m[1]], [m[2], m[3]]]
pub type M2 = [C64; 4];

pub fn m2_mul(a: &M2, b: &M2) -> M2 {
    [a[0] * b[0] + a[1] * b[2], a[0] * b[1] + a[1] * b[3], a[2] * b[0] + a[3] * b[2], a[2] * b[1] + a[3] * b[3]]
}
pub fn m2_dag(a: &M2) -> M2 {
    [a[0].conj(), a[2].conj(), a[1].conj(), a[3].conj()]
}
pub fn m2_id() -> M2 {
    [C64::ONE, C64::ZERO, C64::ZERO, C64::ONE]
}
pub fn m2_from(m: &CMat) -> M2 {
    [m.get(0, 0), m.get(0, 1), m.get(1, 0), m.get(1, 1)]
}

/// exp(-i ell (gv a.tau/2 + gf b)) in closed form: exp(-i ell gf b) (cos(x) - i sin(x) n.tau), x = ell gv |a|/2
pub fn link(gv: f64, gf: f64, a: &[f64; 3], b: f64, ell: f64) -> M2 {
    let na = (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt();
    let x = 0.5 * ell * gv * na;
    let ph = C64::cis(-ell * gf * b);
    let (cx, sx) = (x.cos(), x.sin());
    let (n1, n2, n3) = if na > 0.0 { (a[0] / na, a[1] / na, a[2] / na) } else { (0.0, 0.0, 0.0) };
    // cos x 1 - i sin x (n1 tau1 + n2 tau2 + n3 tau3)
    let m = [c(cx, -sx * n3), c(-sx * n2, -sx * n1), c(sx * n2, -sx * n1), c(cx, sx * n3)];
    [ph * m[0], ph * m[1], ph * m[2], ph * m[3]]
}

/// the 2 x 2 potential matrix gv A0.tau/2 + gf A0F
pub fn potential(gv: f64, gf: f64, a0: &[f64; 3], a0f: f64) -> M2 {
    let h = 0.5 * gv;
    [c(gf * a0f + h * a0[2], 0.0), c(h * a0[0], -h * a0[1]), c(h * a0[0], h * a0[1]), c(gf * a0f - h * a0[2], 0.0)]
}

/// the lattice geometry and the constant parameters
#[derive(Clone, Debug)]
pub struct Lattice {
    pub n: [usize; 3],
    pub ell: f64,
    pub periodic: [bool; 3],
    pub wilson: f64,
    pub mass: f64,
    pub mu: f64,
}

/// the fields on the lattice: links (3 per site), the potential matrix and the order parameter
#[derive(Clone, Debug)]
pub struct Fields {
    pub links: Vec<[M2; 3]>,
    pub v0: Vec<M2>,
    pub d: Vec<[C64; 3]>,
}

impl Fields {
    pub fn trivial(nsites: usize) -> Fields {
        Fields { links: vec![[m2_id(); 3]; nsites], v0: vec![[C64::ZERO; 4]; nsites], d: vec![[C64::ZERO; 3]; nsites] }
    }
}

/// the fixed 4 x 4 Dirac matrices alpha^1..3 and beta and the 8 x 8 matrices Kch_iota^dag
pub struct Mats {
    pub alpha: [[[C64; 4]; 4]; 3],
    pub beta: [[C64; 4]; 4],
    pub kdag: [[[C64; 8]; 8]; 3],
}

impl Mats {
    pub fn new() -> Mats {
        let mut alpha = [[[C64::ZERO; 4]; 4]; 3];
        let mut beta = [[C64::ZERO; 4]; 4];
        let mut kdag = [[[C64::ZERO; 8]; 8]; 3];
        for j in 0..3 {
            let a = model::alpha(j + 1);
            for r in 0..4 {
                for cc in 0..4 {
                    alpha[j][r][cc] = a.get(r, cc);
                }
            }
        }
        let b = model::beta();
        for r in 0..4 {
            for cc in 0..4 {
                beta[r][cc] = b.get(r, cc);
            }
        }
        for i in 0..3 {
            let k = model::kpair(i + 1).dagger();
            for r in 0..8 {
                for cc in 0..8 {
                    kdag[i][r][cc] = k.get(r, cc);
                }
            }
        }
        Mats { alpha, beta, kdag }
    }
}

impl Default for Mats {
    fn default() -> Self {
        Mats::new()
    }
}

#[inline]
fn dirac4(m: &[[C64; 4]; 4], w: &[C64; 8]) -> [C64; 8] {
    let mut o = [C64::ZERO; 8];
    for a in 0..4 {
        for b in 0..4 {
            let x = m[a][b];
            if x.re != 0.0 || x.im != 0.0 {
                o[2 * a] += x * w[2 * b];
                o[2 * a + 1] += x * w[2 * b + 1];
            }
        }
    }
    o
}

#[inline]
fn iso2(m: &M2, w: &[C64; 8]) -> [C64; 8] {
    let mut o = [C64::ZERO; 8];
    for a in 0..4 {
        o[2 * a] = m[0] * w[2 * a] + m[1] * w[2 * a + 1];
        o[2 * a + 1] = m[2] * w[2 * a] + m[3] * w[2 * a + 1];
    }
    o
}

impl Lattice {
    pub fn nsites(&self) -> usize {
        self.n[0] * self.n[1] * self.n[2]
    }
    pub fn dim(&self) -> usize {
        16 * self.nsites()
    }
    pub fn site(&self, ix: usize, iy: usize, iz: usize) -> usize {
        ix + self.n[0] * (iy + self.n[1] * iz)
    }
    pub fn coords(&self, s: usize) -> [usize; 3] {
        [s % self.n[0], (s / self.n[0]) % self.n[1], s / (self.n[0] * self.n[1])]
    }
    /// the neighbour of s in direction j (+1 or -1), or None at a Dirichlet boundary.  A direction
    /// with a single site has no neighbours (the fields do not depend on that coordinate).
    pub fn neighbour(&self, s: usize, j: usize, fwd: bool) -> Option<usize> {
        let mut x = self.coords(s);
        let n = self.n[j];
        if n == 1 {
            return None;
        }
        if fwd {
            if x[j] + 1 == n {
                if !self.periodic[j] {
                    return None;
                }
                x[j] = 0;
            } else {
                x[j] += 1;
            }
        } else if x[j] == 0 {
            if !self.periodic[j] {
                return None;
            }
            x[j] = n - 1;
        } else {
            x[j] -= 1;
        }
        Some(self.site(x[0], x[1], x[2]))
    }

    /// the lattice Dirac Hamiltonian minus (V0 + mu) acting on the 8-component field `f` at site s
    fn h_minus_v(&self, fl: &Fields, mats: &Mats, f: &dyn Fn(usize) -> [C64; 8], s: usize) -> [C64; 8] {
        let ps = f(s);
        let mut out = dirac4(&mats.beta, &ps);
        for k in 0..8 {
            out[k] = out[k].scale(self.mass);
        }
        let inv2l = 0.5 / self.ell;
        let rw = self.wilson * inv2l;
        for j in 0..3 {
            if self.n[j] == 1 {
                continue;
            }
            let wf = match self.neighbour(s, j, true) {
                Some(sp) => iso2(&fl.links[s][j], &f(sp)),
                None => [C64::ZERO; 8],
            };
            let wb = match self.neighbour(s, j, false) {
                Some(sm) => iso2(&m2_dag(&fl.links[sm][j]), &f(sm)),
                None => [C64::ZERO; 8],
            };
            let mut diff = [C64::ZERO; 8];
            let mut wil = [C64::ZERO; 8];
            for k in 0..8 {
                diff[k] = (wf[k] - wb[k]).scale(inv2l);
                wil[k] = wf[k] + wb[k] - ps[k].scale(2.0);
            }
            let ad = dirac4(&mats.alpha[j], &diff);
            let bw = dirac4(&mats.beta, &wil);
            for k in 0..8 {
                // -i alpha^j diff - rw beta wil
                out[k] += c(ad[k].im, -ad[k].re) - bw[k].scale(rw);
            }
        }
        let vp = iso2(&fl.v0[s], &ps);
        for k in 0..8 {
            out[k] -= vp[k] + ps[k].scale(self.mu);
        }
        out
    }

    /// out = H_DB w for a Nambu field w (16 components per site)
    pub fn apply(&self, fl: &Fields, mats: &Mats, w: &[C64], out: &mut [C64]) {
        let ns = self.nsites();
        let part = |s: usize| -> [C64; 8] {
            let mut r = [C64::ZERO; 8];
            r.copy_from_slice(&w[16 * s..16 * s + 8]);
            r
        };
        let hole_conj = |s: usize| -> [C64; 8] {
            let mut r = [C64::ZERO; 8];
            for k in 0..8 {
                r[k] = w[16 * s + 8 + k].conj();
            }
            r
        };
        for s in 0..ns {
            let hp = self.h_minus_v(fl, mats, &part, s);
            let hh = self.h_minus_v(fl, mats, &hole_conj, s);
            let d = fl.d[s];
            let u = part(s);
            let mut vh = [C64::ZERO; 8];
            for k in 0..8 {
                vh[k] = w[16 * s + 8 + k];
            }
            for r in 0..8 {
                // P v and P^dag u, P = sum_i d_i Kdag_i
                let mut pv = C64::ZERO;
                let mut pdu = C64::ZERO;
                for i in 0..3 {
                    for cc in 0..8 {
                        let kd = mats.kdag[i][r][cc];
                        if kd.re != 0.0 || kd.im != 0.0 {
                            pv += d[i] * kd * vh[cc];
                        }
                        // (P^dag)_{r cc} = conj(P_{cc r}) = sum_i conj(d_i) conj(Kdag_i[cc][r])
                        let kd2 = mats.kdag[i][cc][r];
                        if kd2.re != 0.0 || kd2.im != 0.0 {
                            pdu += (d[i] * kd2).conj() * u[cc];
                        }
                    }
                }
                out[16 * s + r] = hp[r] + pv;
                out[16 * s + 8 + r] = pdu - hh[r].conj();
            }
        }
    }

    /// the dense matrix of H_DB (dimension 16 x number of sites), built column by column
    pub fn dense(&self, fl: &Fields, mats: &Mats) -> CMat {
        let n = self.dim();
        let mut m = CMat::zeros(n);
        let mut e = vec![C64::ZERO; n];
        let mut out = vec![C64::ZERO; n];
        for col in 0..n {
            for x in e.iter_mut() {
                *x = C64::ZERO;
            }
            e[col] = C64::ONE;
            self.apply(fl, mats, &e, &mut out);
            for row in 0..n {
                m.set(row, col, out[row]);
            }
        }
        m
    }

    /// an upper bound of the spectral radius of H_DB (used for the stability of the time step):
    /// sum over the active directions of (1 + 2 r_W)/ell, plus m, |mu|, max ||V0|| and max sum_i |d^i|
    pub fn norm_bound(&self, fl: &Fields) -> f64 {
        let ndir = (0..3).filter(|&j| self.n[j] > 1).count() as f64;
        let mut vmax = 0.0f64;
        let mut dmax = 0.0f64;
        for s in 0..self.nsites() {
            let v = &fl.v0[s];
            let fro = (v.iter().map(|z| z.norm_sqr()).sum::<f64>()).sqrt();
            vmax = vmax.max(fro);
            dmax = dmax.max(fl.d[s].iter().map(|z| z.abs()).sum::<f64>());
        }
        ndir * (1.0 + 2.0 * self.wilson) / self.ell + self.mass.abs() + self.mu.abs() + vmax + dmax
    }
}

/// the free lattice dispersion: eps_lat(q)^2 = sum_j sin^2(q_j ell)/ell^2 + (m + (r_W/ell) sum_j (1 - cos q_j ell))^2
pub fn eps_lat(q: &[f64; 3], ell: f64, m: f64, r: f64, active: &[bool; 3]) -> f64 {
    let mut s2 = 0.0;
    let mut w = 0.0;
    for j in 0..3 {
        if active[j] {
            s2 += (q[j] * ell).sin().powi(2) / (ell * ell);
            w += 1.0 - (q[j] * ell).cos();
        }
    }
    (s2 + (m + r / ell * w).powi(2)).sqrt()
}

/// the anomalous density of a set of mode functions: Pi^iota(s) = sum_n psi_p,n(s)^T Kch_iota conj(psi_h,n(s))
/// (the expectation value <Psi^T Kch_iota Psi> in the state in which these modes are the
/// positive-energy quasi-particle modes)
pub fn anomalous(nsites: usize, modes: &[Vec<C64>], k: &[CMat; 3]) -> Vec<[C64; 3]> {
    let mut out = vec![[C64::ZERO; 3]; nsites];
    for w in modes {
        for s in 0..nsites {
            for i in 0..3 {
                let mut acc = C64::ZERO;
                for a in 0..8 {
                    let ua = w[16 * s + a];
                    if ua.re == 0.0 && ua.im == 0.0 {
                        continue;
                    }
                    for b in 0..8 {
                        let kab = k[i].get(a, b);
                        if kab.re != 0.0 || kab.im != 0.0 {
                            acc += ua * kab * w[16 * s + 8 + b].conj();
                        }
                    }
                }
                out[s][i] += acc;
            }
        }
    }
    out
}
