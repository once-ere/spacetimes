//! Angular power spectra of the microwave background (TT, TE, EE; unlensed) and the
//! matter power spectrum, from the sources of `boltzmann.rs`.
//!
//!   C_l^XY = 4 pi Int dk/k  P_R(k)  X_l(k) Y_l(k),      P_R(k) = A_s (k / 0.05 Mpc^-1)^(n_s - 1),
//!   P_m(k) = (2 pi^2 / k^3) P_R(k) delta_m(k)^2          (delta_m for unit curvature perturbation),
//!   sigma_8^2 = Int dk/k  P_R(k) delta_m(k)^2 W(k R)^2,  R = 8 Mpc/h,  W(x) = 3 (sin x - x cos x)/x^3.
//!
//! The spectra are computed for a list of sample multipoles and interpolated with a
//! cubic spline; they are returned as D_l = l (l+1) C_l / (2 pi) in microkelvin^2.
//! Gravitational lensing of the microwave background is NOT included.

use crate::bessel::BesselTable;
use crate::boltzmann::{integrate_mode, source_times, Fast, ModeOut};
use crate::cosmo::{Background, Params, Thermo};
use crate::io::par_map;

/// Things that do not depend on the cosmological model and are computed once.
pub struct Workspace {
    pub l_max: usize,
    pub ls: Vec<usize>,
    pub bessel: BesselTable,
}

impl Workspace {
    pub fn new(l_max: usize) -> Workspace {
        // sample multipoles: dense at low l, then steps of 25 and 50
        let mut ls: Vec<usize> = vec![2, 3, 4, 5, 6, 7, 8, 10, 12, 15, 20, 25, 30, 40, 50, 60, 70, 80, 90, 100];
        let mut l = 100;
        while l < l_max {
            l += if l < 800 { 25 } else { 50 };
            ls.push(l.min(l_max));
        }
        ls.dedup();
        // the conformal age is at most ~15,000 Mpc for the models considered; k_max = 2.45 l_max / tau0
        let x_max = 2.45 * l_max as f64 * 1.08 + 50.0;
        let bessel = BesselTable::new(&ls, x_max, 0.2);
        Workspace { l_max, ls, bessel }
    }
}

/// Output of a model computation.
#[derive(Clone, Debug)]
pub struct Spectra {
    /// D_l^TT, D_l^TE, D_l^EE in microkelvin^2 for l = 0..=l_max (entries 0 and 1 are zero)
    pub tt: Vec<f64>,
    pub te: Vec<f64>,
    pub ee: Vec<f64>,
    /// matter power spectrum today: wavenumbers (1/Mpc) and P(k) (Mpc^3)
    pub k_pk: Vec<f64>,
    pub pk: Vec<f64>,
    pub sigma8: f64,
    pub thermo: Derived,
}

/// Derived parameters of the background and the thermal history.
#[derive(Clone, Copy, Debug, Default)]
pub struct Derived {
    pub h: f64,
    pub tau0: f64,
    pub z_star: f64,
    pub z_rec: f64,
    pub z_drag: f64,
    pub rs_star: f64,
    pub rs_drag: f64,
    pub theta_s_100: f64,
    pub theta_star_100: f64,
    pub z_reio: f64,
    pub y_he: f64,
    pub z_eq: f64,
    pub omega_m: f64,
}

/// Natural cubic spline through (x_i, y_i); returns the second derivatives.
pub fn spline(x: &[f64], y: &[f64]) -> Vec<f64> {
    let n = x.len();
    let mut y2 = vec![0.0; n];
    let mut u = vec![0.0; n];
    for i in 1..n - 1 {
        let sig = (x[i] - x[i - 1]) / (x[i + 1] - x[i - 1]);
        let p = sig * y2[i - 1] + 2.0;
        y2[i] = (sig - 1.0) / p;
        u[i] = (y[i + 1] - y[i]) / (x[i + 1] - x[i]) - (y[i] - y[i - 1]) / (x[i] - x[i - 1]);
        u[i] = (6.0 * u[i] / (x[i + 1] - x[i - 1]) - sig * u[i - 1]) / p;
    }
    for i in (0..n - 1).rev() {
        y2[i] = y2[i] * y2[i + 1] + u[i];
    }
    y2
}

/// Evaluate the spline of `spline` at xx (x ascending).
pub fn splint(x: &[f64], y: &[f64], y2: &[f64], xx: f64) -> f64 {
    let n = x.len();
    let (mut lo, mut hi) = (0, n - 1);
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if x[mid] > xx {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    let h = x[hi] - x[lo];
    let a = (x[hi] - xx) / h;
    let b = (xx - x[lo]) / h;
    a * y[lo] + b * y[hi] + ((a * a * a - a) * y2[lo] + (b * b * b - b) * y2[hi]) * h * h / 6.0
}

/// Compute the spectra of the model `p`.  `accuracy` scales the integration tolerance
/// and the density of the wavenumber grids (1.0 = default).
pub fn compute(p: &Params, ws: &Workspace, accuracy: f64) -> Spectra {
    let bg = Background::new(p);
    let th = Thermo::new(&bg);
    let fast = Fast::new(&bg, &th);
    let tau_out = source_times(&bg);
    let nt = tau_out.len();
    let tau0 = bg.tau0;
    let l_max = ws.l_max;
    let k_max = 2.45 * l_max as f64 / tau0;
    let rtol = 1e-5 / accuracy;

    // ---- coarse wavenumber grid for the sources ----
    let mut ks: Vec<f64> = Vec::new();
    let n_log = (30.0 * accuracy) as usize;
    let (k0, k1): (f64, f64) = (8e-6, 4e-3);
    for i in 0..n_log {
        ks.push((k0.ln() + (k1.ln() - k0.ln()) * i as f64 / n_log as f64).exp());
    }
    let dk = 0.0025 / accuracy;
    let mut k = k1;
    while k < k_max + dk {
        ks.push(k);
        k += dk;
    }
    let modes: Vec<ModeOut> = par_map(&ks, |&k| integrate_mode(&fast, k, &tau_out, rtol, false));

    // ---- splines of the sources in k, at every output time ----
    // layout: for each time j, four arrays over the coarse k grid and their second derivatives
    let nk = ks.len();
    let col = |get: &dyn Fn(&ModeOut) -> f64| -> Vec<f64> { modes.iter().map(|m| get(m)).collect() };
    struct SrcSpline {
        y: [Vec<f64>; 4],
        y2: [Vec<f64>; 4],
    }
    let splines: Vec<SrcSpline> = (0..nt)
        .map(|j| {
            let y = [col(&|m: &ModeOut| m.s0[j]), col(&|m: &ModeOut| m.s1[j]), col(&|m: &ModeOut| m.s2[j]), col(&|m: &ModeOut| m.se[j])];
            let y2 = [spline(&ks, &y[0]), spline(&ks, &y[1]), spline(&ks, &y[2]), spline(&ks, &y[3])];
            SrcSpline { y, y2 }
        })
        .collect();

    // trapezoidal weights of the time grid
    let mut wt = vec![0.0; nt];
    for j in 0..nt {
        let left = if j > 0 { tau_out[j] - tau_out[j - 1] } else { 0.0 };
        let right = if j + 1 < nt { tau_out[j + 1] - tau_out[j] } else { 0.0 };
        wt[j] = 0.5 * (left + right);
    }
    // index of the last time needed for modes with k > k_late (recombination and early ISW only)
    let j_late = tau_out.iter().position(|&t| t > 1500.0).unwrap_or(nt);
    let k_late = 0.03;

    // ---- fine wavenumber grid for the k integration ----
    let mut kf: Vec<f64> = Vec::new();
    let dk_fine = 4.5e-5 / accuracy;
    let mut k = 0.5 / tau0;
    while k < k_max {
        kf.push(k);
        k += (0.02 * k / accuracy).min(dk_fine);
    }
    let nkf = kf.len();
    let mut wk = vec![0.0; nkf];
    for i in 0..nkf {
        let left = if i > 0 { kf[i] - kf[i - 1] } else { 0.0 };
        let right = if i + 1 < nkf { kf[i + 1] - kf[i] } else { 0.0 };
        wk[i] = 0.5 * (left + right);
    }

    // ---- line-of-sight integrals and k integration, in parallel chunks of the fine grid ----
    let nl = ws.ls.len();
    let chunks: Vec<(usize, usize)> = {
        let nchunk = 16;
        (0..nchunk).map(|c| (c * nkf / nchunk, (c + 1) * nkf / nchunk)).collect()
    };
    let a_s = p.a_s;
    let n_s = p.n_s;
    let partial: Vec<Vec<[f64; 3]>> = par_map(&chunks, |&(i0, i1)| {
        let mut acc = vec![[0.0f64; 3]; nl];
        let mut s = vec![[0.0f64; 4]; nt];
        for i in i0..i1 {
            let k = kf[i];
            let jn = if k > k_late { j_late } else { nt };
            // interpolate the sources at this k: locate the interval once
            let (mut lo, mut hi) = (0, nk - 1);
            while hi - lo > 1 {
                let mid = (lo + hi) / 2;
                if ks[mid] > k {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            let h = ks[hi] - ks[lo];
            let a = (ks[hi] - k) / h;
            let b = (k - ks[lo]) / h;
            let (ca, cb) = ((a * a * a - a) * h * h / 6.0, (b * b * b - b) * h * h / 6.0);
            for j in 0..jn {
                let sp = &splines[j];
                for q in 0..4 {
                    s[j][q] = a * sp.y[q][lo] + b * sp.y[q][hi] + ca * sp.y2[q][lo] + cb * sp.y2[q][hi];
                }
            }
            let prim = a_s * (k / 0.05).powf(n_s - 1.0);
            let weight = 4.0 * std::f64::consts::PI * wk[i] / k * prim;
            for (il, &l) in ws.ls.iter().enumerate() {
                let lf = l as f64;
                // Multipoles that this wavenumber can contribute to: j_l(x) is negligible
                // for x = k (tau0 - tau) well below l.  There is NO upper limit in k: the
                // projection of a wave of wavenumber k > l/tau0 seen at an angle
                // contributes to l with the slowly decaying weight ~ 1/(x sqrt(x^2 - l^2)).
                if lf > 1.02 * k * tau0 + 10.0 {
                    continue;
                }
                let x_min = lf * (1.0 - 2.5 * lf.powf(-2.0 / 3.0));
                let ll1 = lf * (lf + 1.0);
                let (mut th_l, mut e_l) = (0.0, 0.0);
                for j in 0..jn {
                    let x = k * (tau0 - tau_out[j]);
                    if x < x_min || x < 1e-6 {
                        break; // x decreases with j: nothing more to add
                    }
                    let (jl, djl) = ws.bessel.eval(il, x);
                    let ddjl = -2.0 / x * djl - (1.0 - ll1 / (x * x)) * jl;
                    th_l += wt[j] * (s[j][0] * jl + s[j][1] * djl + s[j][2] * 0.5 * (3.0 * ddjl + jl));
                    e_l += wt[j] * s[j][3] * jl / (x * x);
                }
                let e_fac = ((lf + 2.0) * (lf + 1.0) * lf * (lf - 1.0)).sqrt();
                let e_l = e_l * e_fac;
                acc[il][0] += weight * th_l * th_l;
                acc[il][1] += weight * th_l * e_l;
                acc[il][2] += weight * e_l * e_l;
            }
        }
        acc
    });
    let mut cl = vec![[0.0f64; 3]; nl];
    for part in &partial {
        for il in 0..nl {
            for q in 0..3 {
                cl[il][q] += part[il][q];
            }
        }
    }

    // ---- D_l at all multipoles ----
    let t_muk2 = (p.t_cmb * 1e6).powi(2);
    let xs: Vec<f64> = ws.ls.iter().map(|&l| l as f64).collect();
    let mut out = [vec![0.0; l_max + 1], vec![0.0; l_max + 1], vec![0.0; l_max + 1]];
    for q in 0..3 {
        let ys: Vec<f64> = (0..nl).map(|il| xs[il] * (xs[il] + 1.0) / (2.0 * std::f64::consts::PI) * cl[il][q] * t_muk2).collect();
        let y2 = spline(&xs, &ys);
        for l in 2..=l_max {
            out[q][l] = splint(&xs, &ys, &y2, l as f64);
        }
    }

    // ---- matter power spectrum ----
    let mut k_pk: Vec<f64> = ks.iter().copied().filter(|&k| k <= 0.3).collect();
    let mut d_pk: Vec<f64> = modes.iter().filter(|m| m.k <= 0.3).map(|m| m.delta_m0).collect();
    let hi_ks: Vec<f64> = {
        let n = (14.0 * accuracy) as usize;
        let (a, b): (f64, f64) = (0.33, 3.0);
        (0..=n).map(|i| (a.ln() + (b.ln() - a.ln()) * i as f64 / n as f64).exp()).collect()
    };
    let last = [tau0];
    let hi_modes: Vec<f64> = par_map(&hi_ks, |&k| integrate_mode(&fast, k, &last, rtol, true).delta_m0);
    k_pk.extend_from_slice(&hi_ks);
    d_pk.extend_from_slice(&hi_modes);
    let pk: Vec<f64> = k_pk.iter().zip(&d_pk).map(|(&k, &d)| 2.0 * std::f64::consts::PI.powi(2) / k.powi(3) * a_s * (k / 0.05).powf(n_s - 1.0) * d * d).collect();
    // sigma_8: integrate in ln k with a spline of delta_m / k^2 (a smooth function)
    let lnk: Vec<f64> = k_pk.iter().map(|k| k.ln()).collect();
    let tk: Vec<f64> = k_pk.iter().zip(&d_pk).map(|(&k, &d)| d / (k * k)).collect();
    let tk2 = spline(&lnk, &tk);
    let r8 = 8.0 / p.h;
    let n_int = 4000;
    let (l0, l1) = (lnk[0], lnk[lnk.len() - 1]);
    let mut s8 = 0.0;
    for i in 0..=n_int {
        let lk = l0 + (l1 - l0) * i as f64 / n_int as f64;
        let k = lk.exp();
        let d = splint(&lnk, &tk, &tk2, lk) * k * k;
        let x = k * r8;
        let w = if x < 1e-3 { 1.0 - x * x / 10.0 } else { 3.0 * (x.sin() - x * x.cos()) / (x * x * x) };
        let f = a_s * (k / 0.05).powf(n_s - 1.0) * d * d * w * w;
        s8 += if i == 0 || i == n_int { 0.5 * f } else { f };
    }
    let sigma8 = (s8 * (l1 - l0) / n_int as f64).sqrt();

    let [tt, te, ee] = out;
    Spectra {
        tt,
        te,
        ee,
        k_pk,
        pk,
        sigma8,
        thermo: Derived {
            h: p.h,
            tau0,
            z_star: th.z_star,
            z_rec: th.z_rec,
            z_drag: th.z_drag,
            rs_star: th.rs_star,
            rs_drag: th.rs_drag,
            theta_s_100: th.theta_s_100,
            theta_star_100: th.theta_star_100,
            z_reio: th.z_reio,
            y_he: th.y_he,
            z_eq: 1.0 / bg.a_eq() - 1.0,
            omega_m: (p.omega_b + p.omega_c) / (p.h * p.h),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spline_reproduces_a_cubic_segment() {
        let x: Vec<f64> = (0..12).map(|i| i as f64 * 0.5).collect();
        let y: Vec<f64> = x.iter().map(|&t| (0.7 * t).sin()).collect();
        let y2 = spline(&x, &y);
        for &t in &[1.3, 2.75, 4.1] {
            assert!((splint(&x, &y, &y2, t) - (0.7 * t as f64).sin()).abs() < 2e-3);
        }
    }
}

/// Linear matter power spectrum today, P(k) in Mpc^3, at the wavenumbers `ks` (1/Mpc),
/// which may be much larger than those needed for the microwave background.  Uses the
/// fast integration with the radiation-streaming approximation at late times, and a
/// thermodynamic table that starts early enough for the largest wavenumber.
/// Returns (P(k), delta_m(k)); the latter is the density contrast per unit primordial
/// curvature perturbation.
pub fn matter_power(p: &Params, ks: &[f64], rtol: f64) -> (Vec<f64>, Vec<f64>) {
    let bg = Background::new(p);
    let th = Thermo::new(&bg);
    let k_max = ks.iter().cloned().fold(0.0, f64::max);
    // start when k tau = 0.02 for the largest k, but not before tau = 2e-5 Mpc (z ~ 2e10)
    let tau_min = (0.01 / k_max).clamp(1.0e-5, 1.0e-3);
    let fast = Fast::with(&bg, &th, tau_min, 30000);
    let last = [bg.tau0];
    let d: Vec<f64> = par_map(ks, |&k| integrate_mode(&fast, k, &last, rtol, true).delta_m0);
    let pk = ks.iter().zip(&d).map(|(&k, &dm)| 2.0 * std::f64::consts::PI.powi(2) / k.powi(3) * p.a_s * (k / 0.05).powf(p.n_s - 1.0) * dm * dm).collect();
    (pk, d)
}
