//! The linear response of the uniform states (Section 10.4 of superfluid.md): the static stability of the normal vacuum
//! in every channel, and the collective modes (the first excited states at zero total momentum) of the superfluid.
use sfcore::c64::C64;
use sfcore::cmat::CMat;
use sfcore::linalg::{eigvals, eigvec};
use sfcore::rpa::{describe, kernel, Response};
use sfcore::uniform::Model;

/// the eigenvalues of M(omega) = chi(omega) F sorted by decreasing real part, with the descriptions of the eigenvectors
/// of the first `nd` of them
pub fn spectrum(resp: &Response, f: &CMat, omega: f64, nd: usize) -> Vec<(C64, String)> {
    let m = resp.chi(omega).mul(f);
    let mut ev = eigvals(&m);
    ev.sort_by(|a, b| b.re.partial_cmp(&a.re).unwrap());
    ev.iter().enumerate().map(|(i, &l)| {
        let d = if i < nd { let (v, _) = eigvec(&m, l); describe(&v) } else { String::new() };
        (l, d)
    }).collect()
}

/// the static response of a state of the model with the self-energies (N, P), and the kernel of the model
pub fn response(model: &Model, nself: &CMat, pself: &CMat) -> (Response, CMat) {
    (Response::new(model, nself, pself), kernel(model))
}

/// a collective mode: its frequency (the energy of the excitation at zero total momentum), its multiplicity, the
/// change of the count of eigenvalues above 1 (+ when an eigenvalue rises through 1), and the description of its
/// amplitude vector
#[derive(Clone, Debug)]
pub struct Mode {
    pub omega: f64,
    pub mult: i64,
    pub desc: String,
}

/// the number of real eigenvalues of M(omega) larger than 1
pub fn count_above(resp: &Response, f: &CMat, omega: f64) -> (i64, Vec<C64>) {
    let m = resp.chi(omega).mul(f);
    let ev = eigvals(&m);
    let n = ev.iter().filter(|l| l.re > 1.0 && l.im.abs() < 1e-7 * (1.0 + l.re.abs())).count() as i64;
    (n, ev)
}

/// the Goldstone modes (eigenvalues of M(0) equal to 1 within tol) and the collective modes in (0, wmax): the
/// frequencies at which an eigenvalue of M(omega) passes through 1, located by a scan of nscan frequencies and bisection
pub fn modes(resp: &Response, f: &CMat, wmax: f64, nscan: usize) -> (usize, f64, Vec<Mode>) {
    let ev0 = eigvals(&resp.chi(0.0).mul(f));
    let gold: Vec<&C64> = ev0.iter().filter(|l| (**l - C64::ONE).abs() < 1e-6).collect();
    let gdev = gold.iter().fold(0.0f64, |s, l| s.max((**l - C64::ONE).abs()));
    let mut out = vec![];
    let w0 = 1e-4 * wmax;
    let (mut nprev, _) = count_above(resp, f, w0);
    let mut wprev = w0;
    for i in 1..=nscan {
        // denser near the threshold: omega = wmax (1 - (1 - i/nscan)^2)
        let x = i as f64 / nscan as f64;
        let w = wmax * (1.0 - (1.0 - x) * (1.0 - x));
        let (n, _) = count_above(resp, f, w);
        if n != nprev {
            let (mut a, mut b) = (wprev, w);
            for _ in 0..50 {
                let mid = 0.5 * (a + b);
                if count_above(resp, f, mid).0 == nprev {
                    a = mid;
                } else {
                    b = mid;
                }
            }
            let wc = 0.5 * (a + b);
            let m = resp.chi(wc).mul(f);
            let ev = eigvals(&m);
            let lam = ev.iter().fold(C64::ZERO, |s, l| if (*l - C64::ONE).abs() < (s - C64::ONE).abs() { *l } else { s });
            let (v, _) = eigvec(&m, lam);
            out.push(Mode { omega: wc, mult: n - nprev, desc: describe(&v) });
        }
        nprev = n;
        wprev = w;
    }
    (gold.len(), gdev, out)
}

/// the excitations of a superfluid ground state: the gap, the threshold of two quasi-particles, the Goldstone modes
/// and the collective modes below the threshold
pub struct Excitations {
    pub gap: f64,
    pub q_gap: f64,
    pub threshold: f64,
    pub n_goldstone: usize,
    pub goldstone_dev: f64,
    pub modes: Vec<Mode>,
}

pub fn excitations(model: &Model, nself: &CMat, pself: &CMat, nscan: usize) -> (Excitations, Response, CMat) {
    let (gap, q_gap) = model.gap(nself, pself);
    let (resp, f) = response(model, nself, pself);
    // the continuum of two quasi-particles at zero total momentum begins at 2 x gap; the scan stops just below it and
    // below the lowest transition of the grid
    let wmax = resp.threshold.min(2.0 * gap) * (1.0 - 1e-3);
    let (ng, gdev, ms) = modes(&resp, &f, wmax, nscan);
    (Excitations { gap, q_gap, threshold: 2.0 * gap, n_goldstone: ng, goldstone_dev: gdev, modes: ms }, resp, f)
}

/// the eigenvalues of M(omega) with real parts in [lo, hi] at the frequencies of a grid (for the figure)
pub fn curves(resp: &Response, f: &CMat, wmax: f64, nw: usize, lo: f64, hi: f64) -> Vec<(f64, Vec<f64>)> {
    (0..=nw).map(|i| {
        let w = wmax * i as f64 / nw as f64;
        let mut ev: Vec<f64> = eigvals(&resp.chi(w).mul(f)).iter().filter(|l| l.im.abs() < 1e-7 && l.re >= lo && l.re <= hi).map(|l| l.re).collect();
        ev.sort_by(|a, b| b.partial_cmp(a).unwrap());
        (w, ev)
    }).collect()
}
