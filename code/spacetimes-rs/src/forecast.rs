//! Mock-data likelihood for the microwave background, Fisher matrices and profile fits.
//!
//! PURPOSE.  The question asked of the cosmological simulations is: if the dark
//! radiation of the model is added to the universe, how far can the Hubble constant
//! be raised before the fit to microwave-background data of a given quality degrades?
//! The "data" are the spectra of the fiducial LambdaCDM model (no noise realization
//! is added, so the best fit of LambdaCDM itself has chi^2 = 0); a model with extra
//! dark radiation is fitted to them by re-adjusting the six LambdaCDM parameters.
//! The same Boltzmann code produces data and model, so its small systematic
//! differences from other codes cancel in chi^2.
//!
//! LIKELIHOOD.  For Gaussian fields observed on a fraction f_sky of the sky,
//!   chi^2 = sum_l (2l+1) f_sky [ Tr(D_l C_l^-1) + ln(det C_l / det D_l) - n ]  +  ((tau - tau_0)/sigma_tau)^2 ,
//! where C_l is the n x n matrix of model spectra plus noise (temperature T and
//! polarization E, n = 2; T only, n = 1, above the highest polarization multipole)
//! and D_l is the same matrix for the data.  A data set (`Config`) is a list of
//! disjoint sky patches, each with its own f_sky, noise and multipole range; their
//! chi^2 add.  The large-scale polarization, which is not simulated, is represented
//! by a Gaussian prior on the optical depth, as in the author's NCSS project.
//!
//! FIT.  The six parameters are (omega_b, omega_c, h, ln(10^10 A_s), n_s, tau).  The
//! derivatives of the spectra with respect to them are computed by central
//! differences (`jacobian`).  A fit then iterates: (1) compute the spectra exactly at
//! the current point; (2) linearize the spectra around it with the stored
//! derivatives; (3) minimize the exact chi^2 of the linearized spectra by Newton's
//! method; until chi^2 no longer decreases.  The value reported is always the chi^2
//! of an exact computation, never of the linearization.  Because the stored
//! derivatives are those of the fiducial point, the end point is the exact minimum
//! only to the extent that the derivatives have not changed; `fit` can be continued
//! with derivatives recomputed at its end point, which the program does to measure
//! the size of this error (it is recorded in the output files).

use crate::cosmo::Params;
use crate::spectra::{compute, Spectra, Workspace};

/// One sky patch of a mock data set.
pub struct Patch {
    pub fsky: f64,
    pub l_min: usize,
    /// highest multipole of the temperature spectrum
    pub l_max_t: usize,
    /// highest multipole of the polarization spectra (<= l_max_t)
    pub l_max_p: usize,
    /// noise spectra as D_l = l(l+1) N_l / (2 pi) in microkelvin^2
    pub noise_t: Vec<f64>,
    pub noise_p: Vec<f64>,
}

/// A mock data set.
pub struct Config {
    pub name: &'static str,
    pub patches: Vec<Patch>,
    /// width of the Gaussian prior on the optical depth
    pub tau_sigma: f64,
}

/// Noise spectra (as D_l) of a set of frequency channels, each given by
/// (beam FWHM in arcmin, temperature noise, polarization noise in microkelvin arcmin):
///     N_l = [ sum_c w_c exp(-l(l+1) theta_c^2 / (8 ln 2)) ]^-1 ,   w_c = 1/(noise_c)^2 .
pub fn channel_noise(channels: &[(f64, f64, f64)], l_max: usize) -> (Vec<f64>, Vec<f64>) {
    let arcmin = std::f64::consts::PI / 180.0 / 60.0;
    let mut nt = vec![0.0; l_max + 1];
    let mut np = vec![0.0; l_max + 1];
    for l in 2..=l_max {
        let ll = (l * (l + 1)) as f64;
        let (mut wt, mut wp) = (0.0, 0.0);
        for &(fwhm, dt, dp) in channels {
            let beam = (-ll * (fwhm * arcmin).powi(2) / (8.0 * std::f64::consts::LN_2)).exp();
            wt += beam / (dt * arcmin).powi(2);
            wp += beam / (dp * arcmin).powi(2);
        }
        nt[l] = ll / (2.0 * std::f64::consts::PI) / wt;
        np[l] = ll / (2.0 * std::f64::consts::PI) / wp;
    }
    (nt, np)
}

/// The three Planck HFI channels used for cosmology, 100, 143 and 217 GHz
/// (Planck 2018 results I, Table 4).
pub const PLANCK_CHANNELS: [(f64, f64, f64); 3] = [(9.66, 77.4, 117.6), (7.22, 33.0, 70.2), (4.90, 46.8, 105.0)];
/// A deep ground-based survey with the sensitivity of the sixth data release of the
/// Atacama Cosmology Telescope: 1.4 arcmin beam, 10 (temperature) and 14
/// (polarization) microkelvin arcmin.
pub const DEEP_CHANNELS: [(f64, f64, f64); 1] = [(1.4, 10.0, 14.0)];
pub const TAU_PRIOR_SIGMA: f64 = 0.0073;

impl Config {
    /// "Planck-like": 60 per cent of the sky, 30 <= l <= 2500 (T), 30 <= l <= 2000 (E).
    pub fn planck_like() -> Config {
        let (nt, np) = channel_noise(&PLANCK_CHANNELS, 2500);
        Config { name: "planck", patches: vec![Patch { fsky: 0.6, l_min: 30, l_max_t: 2500, l_max_p: 2000, noise_t: nt, noise_p: np }], tau_sigma: TAU_PRIOR_SIGMA }
    }

    /// "Planck-like plus deep survey": as above, but 25 per cent of the sky (inside the
    /// 60 per cent) is also observed by the deep survey.  On that part the two maps
    /// are combined with inverse-noise weights, 1/N = 1/N_Planck + 1/N_deep, and the
    /// polarization is used up to l = 2500.
    pub fn planck_plus_deep() -> Config {
        let (nt, np) = channel_noise(&PLANCK_CHANNELS, 2500);
        let (dt, dp) = channel_noise(&DEEP_CHANNELS, 2500);
        let comb = |a: &Vec<f64>, b: &Vec<f64>| -> Vec<f64> { a.iter().zip(b).map(|(x, y)| if *x > 0.0 && *y > 0.0 { 1.0 / (1.0 / x + 1.0 / y) } else { 0.0 }).collect() };
        Config {
            name: "planck+deep",
            patches: vec![
                Patch { fsky: 0.35, l_min: 30, l_max_t: 2500, l_max_p: 2000, noise_t: nt.clone(), noise_p: np.clone() },
                Patch { fsky: 0.25, l_min: 30, l_max_t: 2500, l_max_p: 2500, noise_t: comb(&nt, &dt), noise_p: comb(&np, &dp) },
            ],
            tau_sigma: TAU_PRIOR_SIGMA,
        }
    }
}

/// A mock data set together with the "observed" spectra.
pub struct Mock {
    pub cfg: Config,
    pub data: Spectra,
    pub tau0: f64,
}

/// chi^2 of the model spectra (tt, te, ee as D_l) and of the optical depth.
pub fn chi2(mock: &Mock, tt: &[f64], te: &[f64], ee: &[f64], tau: f64) -> f64 {
    let mut c2 = 0.0;
    for pa in &mock.cfg.patches {
        for l in pa.l_min..=pa.l_max_t {
            let w = (2 * l + 1) as f64 * pa.fsky;
            let (dt, mt) = (mock.data.tt[l] + pa.noise_t[l], tt[l] + pa.noise_t[l]);
            if mt <= 0.0 {
                return f64::INFINITY;
            }
            if l <= pa.l_max_p {
                let (de, me) = (mock.data.ee[l] + pa.noise_p[l], ee[l] + pa.noise_p[l]);
                let (dx, mx) = (mock.data.te[l], te[l]);
                let det_m = mt * me - mx * mx;
                let det_d = dt * de - dx * dx;
                if det_m <= 0.0 {
                    return f64::INFINITY;
                }
                // Tr(D M^-1) = (dt me + de mt - 2 dx mx) / det M
                let tr = (dt * me + de * mt - 2.0 * dx * mx) / det_m;
                c2 += w * (tr + (det_m / det_d).ln() - 2.0);
            } else {
                c2 += w * (dt / mt + (mt / dt).ln() - 1.0);
            }
        }
    }
    c2 + ((tau - mock.tau0) / mock.cfg.tau_sigma).powi(2)
}

/// The six fitted parameters.
pub const NPAR: usize = 6;
pub const PAR_NAMES: [&str; NPAR] = ["omega_b", "omega_c", "h", "ln10A", "n_s", "tau"];

pub fn to_vec(p: &Params) -> [f64; NPAR] {
    [p.omega_b, p.omega_c, p.h, (1e10 * p.a_s).ln(), p.n_s, p.tau_reio]
}

pub fn from_vec(base: &Params, v: &[f64; NPAR]) -> Params {
    Params { omega_b: v[0], omega_c: v[1], h: v[2], a_s: v[3].exp() * 1e-10, n_s: v[4], tau_reio: v[5], ..*base }
}

/// Derivatives of the spectra with respect to parameters: d[i] = (dTT/dp_i, dTE/dp_i, dEE/dp_i).
#[derive(Clone)]
pub struct Jacobian {
    pub d: Vec<[Vec<f64>; 3]>,
}

fn diff(a: &Spectra, b: &Spectra, den: f64) -> [Vec<f64>; 3] {
    let f = |x: &Vec<f64>, y: &Vec<f64>| -> Vec<f64> { x.iter().zip(y).map(|(p, q)| (p - q) / den).collect() };
    [f(&a.tt, &b.tt), f(&a.te, &b.te), f(&a.ee, &b.ee)]
}

/// Central-difference derivatives with respect to the six parameters at `base`.
/// The steps are about one third of the Planck uncertainties of the parameters times
/// ten, large enough to be far above the numerical noise of the spectra and small
/// enough for the second-order error of the central difference to be negligible.
pub fn jacobian(base: &Params, ws: &Workspace) -> Jacobian {
    let v0 = to_vec(base);
    let steps = [0.01 * v0[0], 0.01 * v0[1], 0.004 * v0[2], 0.02, 0.005, 0.004];
    let mut d = Vec::new();
    for i in 0..NPAR {
        let (mut vp, mut vm) = (v0, v0);
        vp[i] += steps[i];
        vm[i] -= steps[i];
        let sp = compute(&from_vec(base, &vp), ws, 1.0);
        let sm = compute(&from_vec(base, &vm), ws, 1.0);
        d.push(diff(&sp, &sm, 2.0 * steps[i]));
    }
    Jacobian { d }
}

/// Derivative of the spectra along a one-parameter family of models `family(x)` at x = x0.
pub fn family_derivative(family: &dyn Fn(f64) -> Params, x0: f64, step: f64, one_sided: bool, ws: &Workspace) -> [Vec<f64>; 3] {
    if one_sided {
        // second-order one-sided formula: f'(x0) = [-3 f(x0) + 4 f(x0+h) - f(x0+2h)] / (2h)
        let s0 = compute(&family(x0), ws, 1.0);
        let s1 = compute(&family(x0 + step), ws, 1.0);
        let s2 = compute(&family(x0 + 2.0 * step), ws, 1.0);
        let f = |a: &Vec<f64>, b: &Vec<f64>, c: &Vec<f64>| -> Vec<f64> { (0..a.len()).map(|l| (-3.0 * a[l] + 4.0 * b[l] - c[l]) / (2.0 * step)).collect() };
        [f(&s0.tt, &s1.tt, &s2.tt), f(&s0.te, &s1.te, &s2.te), f(&s0.ee, &s1.ee, &s2.ee)]
    } else {
        let sp = compute(&family(x0 + step), ws, 1.0);
        let sm = compute(&family(x0 - step), ws, 1.0);
        diff(&sp, &sm, 2.0 * step)
    }
}

/// Solve the symmetric positive-definite system A x = b by Cholesky decomposition.
pub fn solve_spd(a: &[Vec<f64>], b: &[f64]) -> Option<Vec<f64>> {
    let n = b.len();
    let mut l = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in 0..=i {
            let mut s = a[i][j];
            for k in 0..j {
                s -= l[i][k] * l[j][k];
            }
            if i == j {
                if s <= 0.0 {
                    return None;
                }
                l[i][i] = s.sqrt();
            } else {
                l[i][j] = s / l[j][j];
            }
        }
    }
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut s = b[i];
        for k in 0..i {
            s -= l[i][k] * y[k];
        }
        y[i] = s / l[i][i];
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = y[i];
        for k in i + 1..n {
            s -= l[k][i] * x[k];
        }
        x[i] = s / l[i][i];
    }
    Some(x)
}

/// Inverse of a symmetric positive-definite matrix.
pub fn invert_spd(a: &[Vec<f64>]) -> Option<Vec<Vec<f64>>> {
    let n = a.len();
    let mut inv = vec![vec![0.0; n]; n];
    for j in 0..n {
        let mut e = vec![0.0; n];
        e[j] = 1.0;
        let col = solve_spd(a, &e)?;
        for i in 0..n {
            inv[i][j] = col[i];
        }
    }
    Some(inv)
}

/// Gradient g_i = d chi^2 / d p_i and the matrix H_ik = sum_l w Tr(M^-1 J_i M^-1 J_k),
/// which is the second derivative of chi^2 where the model equals the data (and its
/// Gauss-Newton approximation elsewhere), for model spectra (tt, te, ee) and
/// derivatives `d` (any number of parameters).  The prior on the optical depth is
/// NOT included.
pub fn gradient_and_curvature(mock: &Mock, tt: &[f64], te: &[f64], ee: &[f64], d: &[[Vec<f64>; 3]]) -> (Vec<f64>, Vec<Vec<f64>>) {
    let n = d.len();
    let mut g = vec![0.0; n];
    let mut h = vec![vec![0.0; n]; n];
    let mut pm = vec![[0.0f64; 4]; n];
    for pa in &mock.cfg.patches {
        for l in pa.l_min..=pa.l_max_t {
            let w = (2 * l + 1) as f64 * pa.fsky;
            let (dt, mt) = (mock.data.tt[l] + pa.noise_t[l], tt[l] + pa.noise_t[l]);
            if l <= pa.l_max_p {
                let (de, me) = (mock.data.ee[l] + pa.noise_p[l], ee[l] + pa.noise_p[l]);
                let (dx, mx) = (mock.data.te[l], te[l]);
                let det = mt * me - mx * mx;
                // M^-1
                let (i11, i12, i22) = (me / det, -mx / det, mt / det);
                // A = M^-1 D ;  B = M^-1 - A M^-1
                let (a11, a12, a21, a22) = (i11 * dt + i12 * dx, i11 * dx + i12 * de, i12 * dt + i22 * dx, i12 * dx + i22 * de);
                let (b11, b12, b22) = (i11 - (a11 * i11 + a12 * i12), i12 - (a11 * i12 + a12 * i22), i22 - (a21 * i12 + a22 * i22));
                for i in 0..n {
                    let (j11, j12, j22) = (d[i][0][l], d[i][1][l], d[i][2][l]);
                    g[i] += w * (b11 * j11 + 2.0 * b12 * j12 + b22 * j22);
                    // P_i = M^-1 J_i
                    pm[i] = [i11 * j11 + i12 * j12, i11 * j12 + i12 * j22, i12 * j11 + i22 * j12, i12 * j12 + i22 * j22];
                }
                for i in 0..n {
                    for k in 0..=i {
                        // Tr(P_i P_k)
                        h[i][k] += w * (pm[i][0] * pm[k][0] + pm[i][1] * pm[k][2] + pm[i][2] * pm[k][1] + pm[i][3] * pm[k][3]);
                    }
                }
            } else {
                let b = 1.0 / mt - dt / (mt * mt);
                for i in 0..n {
                    let ji = d[i][0][l];
                    g[i] += w * b * ji;
                    for k in 0..=i {
                        h[i][k] += w * ji * d[k][0][l] / (mt * mt);
                    }
                }
            }
        }
    }
    for i in 0..n {
        for k in i + 1..n {
            h[i][k] = h[k][i];
        }
    }
    (g, h)
}

/// Fisher matrix F = H/2 of the mock data at the fiducial model for the six
/// parameters followed by any extra parameters whose derivatives are given;
/// includes the prior on the optical depth.  The covariance matrix of the
/// parameters is the inverse of F.
pub fn fisher(mock: &Mock, jac: &Jacobian, extra: &[[Vec<f64>; 3]]) -> Vec<Vec<f64>> {
    let mut d: Vec<[Vec<f64>; 3]> = jac.d.clone();
    d.extend(extra.iter().cloned());
    let (_, h) = gradient_and_curvature(mock, &mock.data.tt, &mock.data.te, &mock.data.ee, &d);
    let mut f: Vec<Vec<f64>> = h.iter().map(|r| r.iter().map(|x| 0.5 * x).collect()).collect();
    f[5][5] += 1.0 / mock.cfg.tau_sigma.powi(2);
    f
}

/// Minimize the chi^2 of the linearized spectra  C(v) = C0 + J (v - v0)  over v by
/// Newton iterations with a line search; returns the minimizing parameter vector.
fn minimize_linearized(mock: &Mock, jac: &Jacobian, s0: &Spectra, v0: &[f64; NPAR]) -> [f64; NPAR] {
    let lin = |v: &[f64; NPAR]| -> (Vec<f64>, Vec<f64>, Vec<f64>) {
        let mut tt = s0.tt.clone();
        let mut te = s0.te.clone();
        let mut ee = s0.ee.clone();
        for i in 0..NPAR {
            let dv = v[i] - v0[i];
            if dv != 0.0 {
                for l in 2..tt.len() {
                    tt[l] += dv * jac.d[i][0][l];
                    te[l] += dv * jac.d[i][1][l];
                    ee[l] += dv * jac.d[i][2][l];
                }
            }
        }
        (tt, te, ee)
    };
    let mut v = *v0;
    let (tt, te, ee) = lin(&v);
    let mut c_now = chi2(mock, &tt, &te, &ee, v[5]);
    for _ in 0..60 {
        let (tt, te, ee) = lin(&v);
        let (mut g, mut h) = gradient_and_curvature(mock, &tt, &te, &ee, &jac.d);
        g[5] += 2.0 * (v[5] - mock.tau0) / mock.cfg.tau_sigma.powi(2);
        h[5][5] += 2.0 / mock.cfg.tau_sigma.powi(2);
        // Newton step: chi^2 ~ c + g.dv + (1/2) dv.H.dv
        let rhs: Vec<f64> = g.iter().map(|x| -x).collect();
        let dv = match solve_spd(&h, &rhs) {
            Some(x) => x,
            None => break,
        };
        let mut lam = 1.0;
        let mut improved = false;
        for _ in 0..14 {
            let mut vn = v;
            for i in 0..NPAR {
                vn[i] += lam * dv[i];
            }
            let (tt, te, ee) = lin(&vn);
            let c_new = chi2(mock, &tt, &te, &ee, vn[5]);
            if c_new < c_now {
                improved = c_now - c_new > 1e-8;
                v = vn;
                c_now = c_new;
                break;
            }
            lam *= 0.5;
        }
        if !improved {
            break;
        }
    }
    v
}

/// Result of a fit.
#[derive(Clone, Debug)]
pub struct Fit {
    pub params: Params,
    pub chi2: f64,
    pub spectra: Spectra,
    /// number of exact computations of the spectra
    pub evaluations: usize,
}

/// Fit the six LambdaCDM parameters of `base` (whose other entries, in particular the
/// dark sector, are kept fixed) to the mock data, starting from `start`.
pub fn fit(mock: &Mock, jac: &Jacobian, ws: &Workspace, base: &Params, start: &[f64; NPAR], max_iter: usize) -> Fit {
    let mut v = *start;
    let mut p = from_vec(base, &v);
    let mut s = compute(&p, ws, 1.0);
    let mut c = chi2(mock, &s.tt, &s.te, &s.ee, v[5]);
    let mut evals = 1;
    for _ in 0..max_iter {
        let target = minimize_linearized(mock, jac, &s, &v);
        // exact evaluation at the proposed point, with step halving if it is not better
        let mut lam = 1.0;
        let mut accepted = false;
        for _ in 0..4 {
            let mut vn = v;
            for i in 0..NPAR {
                vn[i] += lam * (target[i] - v[i]);
            }
            let pn = from_vec(base, &vn);
            let sn = compute(&pn, ws, 1.0);
            evals += 1;
            let cn = chi2(mock, &sn.tt, &sn.te, &sn.ee, vn[5]);
            if cn < c {
                accepted = c - cn > 0.003;
                v = vn;
                p = pn;
                s = sn;
                c = cn;
                break;
            }
            lam *= 0.5;
        }
        if !accepted {
            break;
        }
    }
    Fit { params: p, chi2: c, spectra: s, evaluations: evals }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn noise_levels() {
        let (nt, np) = channel_noise(&PLANCK_CHANNELS, 2500);
        // the best channel has (33 muK arcmin)^2 = 9.2e-5 muK^2 sr of white noise; with
        // its 7.22 arcmin beam the combined D_l noise at l = 1000 is of order 10 muK^2
        assert!(nt[1000] > 3.0 && nt[1000] < 30.0, "{}", nt[1000]);
        assert!(np[1000] > 2.0 * nt[1000]);
        assert!(nt[2500] > nt[1000]);
        // white-noise limit at low l: N = 1/sum_c w_c
        let arcmin = std::f64::consts::PI / 180.0 / 60.0;
        let w: f64 = PLANCK_CHANNELS.iter().map(|c| 1.0 / (c.1 * arcmin).powi(2)).sum();
        let l = 10usize;
        assert!((nt[l] / ((l * (l + 1)) as f64 / (2.0 * std::f64::consts::PI) / w) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn cholesky_solver_and_inverse() {
        let n = 7;
        let mut a = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                a[i][j] = if i == j { 4.0 + i as f64 } else { 1.0 / (1.0 + (i as f64 - j as f64).abs()) };
            }
        }
        let x0 = [1.0, -2.0, 0.5, 3.0, -1.0, 2.0, 0.25];
        let mut b = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                b[i] += a[i][j] * x0[j];
            }
        }
        let x = solve_spd(&a, &b).unwrap();
        for i in 0..n {
            assert!((x[i] - x0[i]).abs() < 1e-12);
        }
        let inv = invert_spd(&a).unwrap();
        for i in 0..n {
            for j in 0..n {
                let s: f64 = (0..n).map(|k| a[i][k] * inv[k][j]).sum();
                assert!((s - if i == j { 1.0 } else { 0.0 }).abs() < 1e-12);
            }
        }
    }

    /// The analytic gradient and curvature agree with finite differences of chi^2
    /// (synthetic smooth spectra; no Boltzmann computation needed).
    #[test]
    fn gradient_and_curvature_against_finite_differences() {
        let lmax = 2500;
        let shape = |amp: f64, tilt: f64| -> (Vec<f64>, Vec<f64>, Vec<f64>) {
            let mut tt = vec![0.0; lmax + 1];
            let mut te = vec![0.0; lmax + 1];
            let mut ee = vec![0.0; lmax + 1];
            for l in 2..=lmax {
                let x = l as f64 / 1000.0;
                let osc = (l as f64 / 48.0).cos();
                tt[l] = amp * 3000.0 * x.powf(tilt) * (-x * x).exp() * (1.0 + 0.4 * osc) + 50.0 * amp;
                ee[l] = amp * 30.0 * x.powf(tilt + 1.0) * (-0.5 * x * x).exp() * (1.0 - 0.4 * osc) + 0.2 * amp;
                te[l] = 0.3 * amp * (tt[l] * ee[l]).sqrt() * (l as f64 / 97.0).sin();
            }
            (tt, te, ee)
        };
        let (tt, te, ee) = shape(1.0, 0.0);
        let data = Spectra { tt, te, ee, k_pk: vec![], pk: vec![], sigma8: 0.0, thermo: Default::default() };
        let mock = Mock { cfg: Config::planck_plus_deep(), data, tau0: 0.05 };
        // model point displaced from the data; two parameters (amp, tilt)
        let (a0, t0) = (1.03, 0.02);
        let c = |a: f64, t: f64| -> f64 {
            let (tt, te, ee) = shape(a, t);
            chi2(&mock, &tt, &te, &ee, 0.05)
        };
        let (e1, e2) = (1e-4, 1e-4);
        let d_of = |da: f64, dt: f64, den: f64| -> [Vec<f64>; 3] {
            let (p, q) = (shape(a0 + da, t0 + dt), shape(a0 - da, t0 - dt));
            let f = |x: &Vec<f64>, y: &Vec<f64>| -> Vec<f64> { x.iter().zip(y).map(|(u, v)| (u - v) / den).collect() };
            [f(&p.0, &q.0), f(&p.1, &q.1), f(&p.2, &q.2)]
        };
        let d = vec![d_of(e1, 0.0, 2.0 * e1), d_of(0.0, e2, 2.0 * e2)];
        let (tt, te, ee) = shape(a0, t0);
        let (g, _) = gradient_and_curvature(&mock, &tt, &te, &ee, &d);
        let g_num = [(c(a0 + e1, t0) - c(a0 - e1, t0)) / (2.0 * e1), (c(a0, t0 + e2) - c(a0, t0 - e2)) / (2.0 * e2)];
        for i in 0..2 {
            assert!((g[i] / g_num[i] - 1.0).abs() < 1e-5, "gradient {}: {} vs {}", i, g[i], g_num[i]);
        }
        // curvature at the data point equals the second derivative of chi^2 there
        let d1 = {
            let (p, q) = (shape(1.0 + e1, 0.0), shape(1.0 - e1, 0.0));
            let f = |x: &Vec<f64>, y: &Vec<f64>| -> Vec<f64> { x.iter().zip(y).map(|(u, v)| (u - v) / (2.0 * e1)).collect() };
            vec![[f(&p.0, &q.0), f(&p.1, &q.1), f(&p.2, &q.2)]]
        };
        let (g0, h0) = gradient_and_curvature(&mock, &mock.data.tt, &mock.data.te, &mock.data.ee, &d1);
        assert!(g0[0].abs() < 1e-6 * h0[0][0]);
        let e = 2e-3;
        let h_num = (c(1.0 + e, 0.0) - 2.0 * c(1.0, 0.0) + c(1.0 - e, 0.0)) / (e * e);
        assert!((h0[0][0] / h_num - 1.0).abs() < 2e-3, "{} vs {}", h0[0][0], h_num);
    }
}
