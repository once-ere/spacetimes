//! Fits of the six parameters of the standard model to mock data, for a model on a deflating spacetime.
//!
//! DO NOT EDIT.  This file is written by code/scripts/vendor_eb.py of the revision.  It is the code of
//! the functions `minimize_linearized` and `fit` of forecast.rs of the main project (code/spacetimes-rs/src) with the following changes;
//! every other character is the original.  The main project itself is not changed.
//!   * `fit` takes the deflation (`Deflation`), which is kept fixed in the fit, and computes the spectra with `eb_spectra::compute`.
//! See the description at the head of vendor_eb.py and the provenance RP8 of the revision.

use crate::eb_cosmo::Deflation;
use crate::eb_spectra::compute;
use spacetimes::cosmo::Params;
use spacetimes::forecast::{chi2, from_vec, gradient_and_curvature, solve_spd, Fit, Jacobian, Mock, NPAR};
use spacetimes::spectra::{Spectra, Workspace};

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

/// Fit the six LambdaCDM parameters of `base` (whose other entries, in particular the
/// dark sector, are kept fixed) to the mock data, starting from `start`.
pub fn fit(mock: &Mock, jac: &Jacobian, ws: &Workspace, base: &Params, d: &Deflation, start: &[f64; NPAR], max_iter: usize) -> Fit {
    let mut v = *start;
    let mut p = from_vec(base, &v);
    let mut s = compute(&p, d, ws, 1.0);
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
            let sn = compute(&pn, d, ws, 1.0);
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
