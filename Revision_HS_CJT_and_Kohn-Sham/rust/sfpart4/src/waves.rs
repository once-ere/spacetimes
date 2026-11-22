//! Sections 15.3 and 15.4 of superfluid.md (the numbers in parentheses are those of its equations): the gauge fields of the effective theory in one space direction (fields of the time t
//! and of x = x^1), integrated with CVODE and IDA on the periodic spectral grid of `spectral`.
//!
//! (a) TRANSVERSE FIELDS (CVODE).  A transverse component (along x^2) of the zeugon A (mass 0; the massless gauge boson of U(1)^Q, not the photon) and of a W (mass M_W)
//! obeys in the unitary gauge, to first order in the fields, the Klein-Gordon equation with a source,
//!     u_tt = u_xx + J(x, t),    w_tt = w_xx - M_W^2 w + J(x, t),                                      (15.2)
//! J(x, t) = J_0 s(x) f(t) a transverse current sheet of the spinor charges with the profile s(x) = (1 - x^2/sigma^2)
//! e^{-x^2/(2 sigma^2)} and f(t) = r(t) sin(omega_s t) (r(t) = sin^2(pi t/(2 tau)) for t < tau, then 1).  Unknowns
//! (u, u_t, w, w_t).  For every Fourier mode k_m the solution from rest is the Duhamel integral
//!     u^_m(t) = J^_m int_0^t sin(omega_m (t - t'))/omega_m f(t') dt',  omega_m = |k_m| (zeugon), (k_m^2 + M_W^2)^(1/2) (W),
//! computed by Gauss-Legendre quadrature; it is the exact solution of the semi-discrete equations.
//!
//! (b) THE LONGITUDINAL FIELD AND THE GOLDSTONE FIELD.  The phase field theta of the condensate, Phi = Phi_0 e^{i theta}
//! e_3, coupled to B = (B_0, B_1) = (phi, -a) (so that a = B^1), with f^2 = 2 (Z + Y) Phi_0^2 and g = g_F, has the
//! Lagrangian (from (12.4) and (5.3))
//!     L = (1/2) E^2 + (1/2) f^2 [ (theta_t + 2 g phi)^2 - (theta_x - 2 g a)^2 ],    E = -phi_x - a_t ,    (15.6)
//! the field equations
//!     d_x E = -2 g f^2 (theta_t + 2 g phi)        (Gauss's law: it contains no time derivative of phi)
//!     d_t E = -2 g f^2 (theta_x - 2 g a)
//!     theta_tt = theta_xx - 2 g phi_t - 2 g a_x ,
//! and the gauge invariance theta -> theta - 2 g lambda, phi -> phi + lambda_t, a -> a - lambda_x.  M_Z = 2 g f.
//!   * TEMPORAL GAUGE phi = 0 (CVODE): unknowns (theta, pi = theta_t, a, E),
//!         theta' = pi, pi' = D D theta - 2 g D a, a' = -E, E' = -2 g f^2 (D theta - 2 g a);
//!     Gauss's law D E + 2 g f^2 pi = 0 is a constraint on the initial values that these equations keep.  g = 0 is the
//!     free Goldstone field.
//!   * UNITARY GAUGE theta = 0 (IDA): unknowns (a_u, E, phi_u), the last algebraic,
//!         F_1 = a_u' + E + D phi_u = 0,   F_2 = E' - M_Z^2 a_u = 0,   F_3 = D E + M_Z^2 phi_u = 0;
//!     a semi-explicit differential-algebraic system of index one (dF_3/dphi_u = M_Z^2 1 is invertible); for M_Z = 0
//!     (the zeugon) Gauss's law no longer determines phi_u: the gauge freedom, and no longitudinal mode.
//!   The two gauges are related by lambda = theta/(2 g): a_u = a - D theta/(2 g), phi_u = pi/(2 g), E unchanged.  Every
//!   Fourier mode of the unitary gauge obeys a_m'' = -(k_m^2 + M_Z^2) a_m, E_m = -M_Z^2 a_m'/(k_m^2 + M_Z^2),
//!   phi_m = -i k_m E_m / M_Z^2: the exact solution.
use crate::spectral::{matvec, Grid1};
use sfcore::c64::{c, C64};
use sfcore::quad::gauss_legendre;
use sfsundials::cvode::{self, CvodeSettings, CvodeStats};
use sfsundials::ida::{self, IdaSettings, IdaStats};
use std::f64::consts::PI;

/// the source of (15.2)
#[derive(Clone, Copy, Debug)]
pub struct Source {
    pub j0: f64,
    pub sigma: f64,
    pub omega: f64,
    pub tau: f64,
}

impl Source {
    pub fn profile(&self, x: f64) -> f64 {
        let u = x / self.sigma;
        (1.0 - u * u) * (-0.5 * u * u).exp()
    }
    pub fn time(&self, t: f64) -> f64 {
        let r = if t < self.tau { (0.5 * PI * t / self.tau).sin().powi(2) } else { 1.0 };
        r * (self.omega * t).sin()
    }
}

/// the solution of (15.2) at the output times: u and w (each N values per time) and the counters of CVODE
pub struct Transverse {
    pub u: Vec<Vec<f64>>,
    pub w: Vec<Vec<f64>>,
    pub stats: CvodeStats,
}

pub fn transverse(g: &Grid1, d2: &[f64], mw: f64, src: &Source, touts: &[f64], set: &CvodeSettings) -> Result<Transverse, String> {
    let n = g.n;
    let s: Vec<f64> = g.x.iter().map(|&x| src.j0 * src.profile(x)).collect();
    let d2c = d2.to_vec();
    let srcc = *src;
    let mut tmp = vec![0.0; n];
    let f: cvode::Rhs = Box::new(move |t, y, dy| {
        let ft = srcc.time(t);
        // u
        dy[..n].copy_from_slice(&y[n..2 * n]);
        matvec(&d2c, &y[..n], &mut tmp);
        for j in 0..n {
            dy[n + j] = tmp[j] + s[j] * ft;
        }
        // w
        dy[2 * n..3 * n].copy_from_slice(&y[3 * n..4 * n]);
        matvec(&d2c, &y[2 * n..3 * n], &mut tmp);
        for j in 0..n {
            dy[3 * n + j] = tmp[j] - mw * mw * y[2 * n + j] + s[j] * ft;
        }
        0
    });
    let mut us = vec![];
    let mut ws = vec![];
    let stats = cvode::integrate(&vec![0.0; 4 * n], 0.0, touts, set, f, |_, _, y| {
        us.push(y[..n].to_vec());
        ws.push(y[2 * n..3 * n].to_vec());
        true
    })?;
    Ok(Transverse { u: us, w: ws, stats })
}

/// the exact solution of (15.2) from rest for the mass m at the output times (the Duhamel integral of every Fourier mode,
/// accumulated over the intervals between the output times with 12-point Gauss-Legendre panels of length <= 1)
pub fn transverse_exact(g: &Grid1, m: f64, src: &Source, touts: &[f64]) -> Vec<Vec<f64>> {
    let s: Vec<f64> = g.x.iter().map(|&x| src.j0 * src.profile(x)).collect();
    let sh = g.dft(&s);
    let (xg, wg) = gauss_legendre(12);
    let nm = g.n;
    // for every mode: C = int cos(w t') f dt', S = int sin(w t') f dt' (for w = 0: C = int f, S = int t' f)
    let om: Vec<f64> = g.k.iter().map(|&k| (k * k + m * m).sqrt()).collect();
    let mut cc = vec![0.0; nm];
    let mut ss = vec![0.0; nm];
    let mut out = vec![];
    let mut tprev = 0.0;
    for &t in touts {
        let npan = ((t - tprev) / 1.0).ceil().max(1.0) as usize;
        let h = (t - tprev) / npan as f64;
        for p in 0..npan {
            let a = tprev + p as f64 * h;
            for (xi, wi) in xg.iter().zip(&wg) {
                let tt = a + 0.5 * h * (xi + 1.0);
                let ft = src.time(tt) * 0.5 * h * wi;
                for mm in 0..nm {
                    if om[mm] > 0.0 {
                        cc[mm] += (om[mm] * tt).cos() * ft;
                        ss[mm] += (om[mm] * tt).sin() * ft;
                    } else {
                        cc[mm] += ft;
                        ss[mm] += tt * ft;
                    }
                }
            }
        }
        tprev = t;
        let uh: Vec<C64> = (0..nm)
            .map(|mm| {
                let amp = if om[mm] > 0.0 { ((om[mm] * t).sin() * cc[mm] - (om[mm] * t).cos() * ss[mm]) / om[mm] } else { t * cc[mm] - ss[mm] };
                sh[mm].scale(amp)
            })
            .collect();
        out.push(g.idft(&uh));
    }
    out
}

/// the initial packet of the Goldstone field: theta_0(x) = eps exp(-(x - x0)^2/(2 sig^2)) cos(k0 (x - x0))
#[derive(Clone, Copy, Debug)]
pub struct Packet {
    pub eps: f64,
    pub x0: f64,
    pub sigma: f64,
    pub k0: f64,
}

impl Packet {
    pub fn theta(&self, x: f64) -> f64 {
        let u = (x - self.x0) / self.sigma;
        self.eps * (-0.5 * u * u).exp() * (self.k0 * (x - self.x0)).cos()
    }
}

/// the temporal gauge (theta, pi, a, E) at the output times; the initial values: theta_0, pi = -D theta_0 (moving to
/// the right at the speed 1 when g = 0), a = 0, E = 2 g f^2 (theta_0 - mean theta_0) (Gauss's law)
pub struct Temporal {
    pub states: Vec<Vec<f64>>,
    pub stats: CvodeStats,
}

pub fn initial_temporal(g: &Grid1, d: &[f64], pk: &Packet, gc: f64, f2: f64) -> Vec<f64> {
    let n = g.n;
    let th: Vec<f64> = g.x.iter().map(|&x| pk.theta(x)).collect();
    let mut dth = vec![0.0; n];
    matvec(d, &th, &mut dth);
    let mean = th.iter().sum::<f64>() / n as f64;
    let mut y = vec![0.0; 4 * n];
    for j in 0..n {
        y[j] = th[j];
        y[n + j] = -dth[j];
        y[2 * n + j] = 0.0;
        y[3 * n + j] = 2.0 * gc * f2 * (th[j] - mean);
    }
    y
}

pub fn temporal(g: &Grid1, d: &[f64], y0: &[f64], gc: f64, f2: f64, touts: &[f64], set: &CvodeSettings) -> Result<Temporal, String> {
    let n = g.n;
    let dc = d.to_vec();
    let (mut dth, mut d2th, mut da) = (vec![0.0; n], vec![0.0; n], vec![0.0; n]);
    let f: cvode::Rhs = Box::new(move |_t, y, dy| {
        // theta' = pi
        dy[..n].copy_from_slice(&y[n..2 * n]);
        // pi' = D D theta - 2 g D a
        matvec(&dc, &y[..n], &mut dth);
        matvec(&dc, &dth, &mut d2th);
        matvec(&dc, &y[2 * n..3 * n], &mut da);
        for j in 0..n {
            dy[n + j] = d2th[j] - 2.0 * gc * da[j];
        }
        // a' = -E
        for j in 0..n {
            dy[2 * n + j] = -y[3 * n + j];
        }
        // E' = -2 g f^2 (D theta - 2 g a)
        for j in 0..n {
            dy[3 * n + j] = -2.0 * gc * f2 * (dth[j] - 2.0 * gc * y[2 * n + j]);
        }
        0
    });
    let mut states = vec![];
    let stats = cvode::integrate(y0, 0.0, touts, set, f, |_, _, y| {
        states.push(y.to_vec());
        true
    })?;
    Ok(Temporal { states, stats })
}

/// the unitary-gauge values (a_u, E, phi_u) of a temporal-gauge state
pub fn to_unitary(d: &[f64], y: &[f64], n: usize, gc: f64) -> Vec<f64> {
    let mut dth = vec![0.0; n];
    matvec(d, &y[..n], &mut dth);
    let mut u = vec![0.0; 3 * n];
    for j in 0..n {
        u[j] = y[2 * n + j] - dth[j] / (2.0 * gc);
        u[n + j] = y[3 * n + j];
        u[2 * n + j] = y[n + j] / (2.0 * gc);
    }
    u
}

pub struct Unitary {
    pub states: Vec<Vec<f64>>,
    pub stats: IdaStats,
}

/// the unitary gauge by IDA from the consistent values y0 = (a_u, E, phi_u); the derivatives a_u' = -E - D phi_u,
/// E' = M^2 a_u, phi_u' = -D a_u are computed here
pub fn unitary(g: &Grid1, d: &[f64], y0: &[f64], mz: f64, touts: &[f64], set: &IdaSettings) -> Result<Unitary, String> {
    let n = g.n;
    let m2 = mz * mz;
    let mut yp0 = vec![0.0; 3 * n];
    {
        let mut t = vec![0.0; n];
        matvec(d, &y0[2 * n..3 * n], &mut t);
        for j in 0..n {
            yp0[j] = -y0[n + j] - t[j];
            yp0[n + j] = m2 * y0[j];
        }
        matvec(d, &y0[..n], &mut t);
        for j in 0..n {
            yp0[2 * n + j] = -t[j];
        }
    }
    let mut id = vec![1.0; 3 * n];
    for v in id[2 * n..].iter_mut() {
        *v = 0.0;
    }
    let dc = d.to_vec();
    let mut t1 = vec![0.0; n];
    let res: ida::Residual = Box::new(move |_t, y, yp, r| {
        matvec(&dc, &y[2 * n..3 * n], &mut t1);
        for j in 0..n {
            r[j] = yp[j] + y[n + j] + t1[j];
            r[n + j] = yp[n + j] - m2 * y[j];
        }
        matvec(&dc, &y[n..2 * n], &mut t1);
        for j in 0..n {
            r[2 * n + j] = t1[j] + m2 * y[2 * n + j];
        }
        0
    });
    let dj = d.to_vec();
    let nn = 3 * n;
    let jac: ida::DenseJac = Box::new(move |_t, cj, _y, _yp, m| {
        // column-major: m[col * nn + row]
        for j in 0..n {
            // F_1 = a_u' + E + D phi
            m[j * nn + j] += cj; // dF1_j/da_u_j
            m[(n + j) * nn + j] += 1.0; // dF1_j/dE_j
            for l in 0..n {
                m[(2 * n + l) * nn + j] += dj[j * n + l]; // dF1_j/dphi_l
            }
            // F_2 = E' - M^2 a_u
            m[j * nn + n + j] += -m2;
            m[(n + j) * nn + n + j] += cj;
            // F_3 = D E + M^2 phi
            for l in 0..n {
                m[(n + l) * nn + 2 * n + j] += dj[j * n + l];
            }
            m[(2 * n + j) * nn + 2 * n + j] += m2;
        }
        0
    });
    let mut states = vec![];
    let stats = ida::integrate(y0, &yp0, &id, 0.0, touts, set, res, Some(jac), |_, _, y, _| {
        states.push(y.to_vec());
        true
    })?;
    Ok(Unitary { states, stats })
}

/// the exact unitary-gauge solution (a_u, E, phi_u) at the output times from the consistent initial values y0
pub fn unitary_exact(g: &Grid1, d: &[f64], y0: &[f64], mz: f64, touts: &[f64]) -> Vec<Vec<f64>> {
    let n = g.n;
    let m2 = mz * mz;
    let a0 = g.dft(&y0[..n]);
    // a_u'(0) = -E - D phi_u
    let mut t = vec![0.0; n];
    matvec(d, &y0[2 * n..3 * n], &mut t);
    let ap: Vec<f64> = (0..n).map(|j| -y0[n + j] - t[j]).collect();
    let ap0 = g.dft(&ap);
    touts
        .iter()
        .map(|&tt| {
            let mut ah = vec![C64::ZERO; n];
            let mut eh = vec![C64::ZERO; n];
            let mut ph = vec![C64::ZERO; n];
            for mm in 0..n {
                let km = g.k[mm];
                let om = (km * km + m2).sqrt();
                let (co, si) = ((om * tt).cos(), (om * tt).sin());
                ah[mm] = a0[mm].scale(co) + ap0[mm].scale(si / om);
                let apt = a0[mm].scale(-om * si) + ap0[mm].scale(co);
                eh[mm] = apt.scale(-m2 / (om * om));
                ph[mm] = c(0.0, -km) * eh[mm].scale(1.0 / m2);
            }
            let mut out = g.idft(&ah);
            out.extend(g.idft(&eh));
            out.extend(g.idft(&ph));
            out
        })
        .collect()
}

/// the exact free Goldstone field (g = 0) at the output times from (theta_0, pi_0): theta^_m(t) = theta^_m cos(|k| t) +
/// pi^_m sin(|k| t)/|k| (the mode k = 0: theta^_0 + pi^_0 t)
pub fn goldstone_exact(g: &Grid1, y0: &[f64], touts: &[f64]) -> Vec<Vec<f64>> {
    let n = g.n;
    let th = g.dft(&y0[..n]);
    let pi = g.dft(&y0[n..2 * n]);
    touts
        .iter()
        .map(|&tt| {
            let mut a = vec![C64::ZERO; n];
            let mut b = vec![C64::ZERO; n];
            for mm in 0..n {
                let om = g.k[mm].abs();
                if om > 0.0 {
                    a[mm] = th[mm].scale((om * tt).cos()) + pi[mm].scale((om * tt).sin() / om);
                    b[mm] = th[mm].scale(-om * (om * tt).sin()) + pi[mm].scale((om * tt).cos());
                } else {
                    a[mm] = th[mm] + pi[mm].scale(tt);
                    b[mm] = pi[mm];
                }
            }
            let mut out = g.idft(&a);
            out.extend(g.idft(&b));
            out
        })
        .collect()
}

/// the energy density of the temporal gauge, (1/2) E^2 + (1/2) f^2 pi^2 + (1/2) f^2 (D theta - 2 g a)^2
pub fn energy_density_temporal(d: &[f64], y: &[f64], n: usize, gc: f64, f2: f64) -> Vec<f64> {
    let mut dth = vec![0.0; n];
    matvec(d, &y[..n], &mut dth);
    (0..n).map(|j| 0.5 * y[3 * n + j].powi(2) + 0.5 * f2 * y[n + j].powi(2) + 0.5 * f2 * (dth[j] - 2.0 * gc * y[2 * n + j]).powi(2)).collect()
}

/// the energy density of the unitary gauge, (1/2) E^2 + (1/2) M^2 (a_u^2 + phi_u^2)
pub fn energy_density_unitary(y: &[f64], n: usize, mz: f64) -> Vec<f64> {
    (0..n).map(|j| 0.5 * y[n + j].powi(2) + 0.5 * mz * mz * (y[j].powi(2) + y[2 * n + j].powi(2))).collect()
}

/// the centre of a density on the grid, sum x e / sum e
pub fn centroid(g: &Grid1, e: &[f64]) -> f64 {
    let s: f64 = e.iter().sum();
    g.x.iter().zip(e).map(|(x, v)| x * v).sum::<f64>() / s
}

/// the largest absolute difference of two lists of vectors (the first `len` entries of each)
pub fn max_diff(a: &[Vec<f64>], b: &[Vec<f64>], len: usize) -> f64 {
    a.iter().zip(b).map(|(x, y)| x[..len].iter().zip(&y[..len]).map(|(p, q)| (p - q).abs()).fold(0.0, f64::max)).fold(0.0, f64::max)
}

pub fn max_abs(a: &[Vec<f64>], len: usize) -> f64 {
    a.iter().map(|x| x[..len].iter().map(|p| p.abs()).fold(0.0, f64::max)).fold(0.0, f64::max)
}
