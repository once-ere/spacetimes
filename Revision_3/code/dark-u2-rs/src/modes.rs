//! Command `modes`: the collective modes of the superfluid and the masses of the
//! gauge bosons after the breaking U(2)_U2 -> SU(2)_U2.
//!
//! Random-phase approximation (RPA).  The composite fields are x = (Delta_1, Delta_2, M):
//! the real and imaginary parts of the pair field and the effective mass.  The BdG
//! matrix depends on them linearly, H = H_0 + Delta_1 V_1 + Delta_2 V_2 + M V_3 with
//!
//!   V_1 = [[0, K], [K, 0]],  V_2 = i [[0, K], [-K, 0]],  V_3 = [[beta, 0], [0, -beta]]   (K = C^+),
//!
//! and the classical part of Omega is U(x) = (Delta_1^2 + Delta_2^2)/G^P + (M - m)^2/(4 G^S).
//! Small oscillations x(t) = x_0 + dx exp(-i w t) obey [U''+ Pi(w)] dx = 0 with the
//! polarization of the fermions (a = occupied, b = empty eigenvector of H(q), Om = E_b - E_a)
//!
//!   Pi_ij(w) = Int d^7q/(2pi)^7 sum_{a,b} [ <a|V_i|b><b|V_j|a>/(w - Om) - <a|V_j|b><b|V_i|a>/(w + Om) ].
//!
//! At w = 0 this is the second derivative of the fermion energy, so U'' + Pi(0) is the
//! Hessian of Omega: one zero eigenvalue (the phase, Goldstone), the others positive at a
//! minimum.  The amplitude (sigma_U2) mode is the nonzero root of det[U'' + Pi(w)] below
//! the pair-breaking threshold 2 min E^-, or the threshold itself.  The integrals over q
//! use q along x1 (the integrands are rotation invariant) and Gauss-Legendre panels.
//!
//! Phase stiffness.  A condensate Delta exp(i Q.x) costs Omega(Q) - Omega(0) = (1/2) f_s^2 Q^2
//! (the coefficient of (grad theta)^2/2); the coupling of the phase gradient is the vertex
//! V_Q = (Q/2).[[alpha, 0], [0, -alpha]].  Second-order perturbation theory gives
//! the paired part; the normal-state value, which with a cutoff on |q| is a surface term
//! -(2 a7/7) Q^2/sqrt(1 + M^2) (an artefact of the cutoff, zero in a gauge-invariant
//! regularization), is subtracted.  The time-like stiffness f_t^2 = -(1/4) d^2 Omega/d mu^2.
//! The vector boson B_U2 of the broken U(1)_U2 gets the mass m_B = g'_U2 f (charge of the
//! pair 2 y_U2 = 1); the W_U2 bosons stay massless because the pair is an SU(2)_U2 singlet.

use crate::algebra::SPACE;
use crate::bdg::{bdg, dirac, Dirac};
use crate::checks::Checks;
use crate::integrate::{breaks_around, nodes, uniform, Nodes};
use crate::potential::{a7, b8, density, f8pp, fermion7, ft2_fixed, gcrit7, gcrit8, minimize, omega7, solve_delta, solve_delta8, Model};
use crate::rsym::{dot, eigh, RMat};
use spacetimes::io::par_map;

/// The matrix elements between one occupied and one empty eigenvector.
#[derive(Clone, Copy)]
struct Elem {
    om: f64,
    m1: f64,
    r2: f64, // <b|V_2|a> = i r2
    m3: f64,
}

/// Everything the RPA needs from one state.
pub struct Resp {
    items: Vec<(f64, Vec<Elem>)>, // (weight a7 q^6 w_q, elements)
    pub stiff: f64,               // Int a7 q^6 (1/7) sum_j sum_ab |<b|V_j|a>|^2 / Om
    pub occ_ok: bool,             // exactly 16 negative eigenvalues everywhere
}

fn vertices(d: &Dirac) -> (RMat, RMat, RMat, Vec<RMat>) {
    let z = RMat::zeros(16);
    let k = &d.cp;
    let v1 = RMat::block2(&z, k, k, &z);
    let r2 = RMat::block2(&z, k, &k.scale(-1.0), &z);
    let v3 = RMat::block2(&d.beta, &z, &z, &d.beta.scale(-1.0));
    let vj = d.alpha.iter().map(|a| RMat::block2(&a.scale(0.5), &z, &z, &a.scale(-0.5))).collect();
    (v1, r2, v3, vj)
}

/// Quadrature nodes for a state: refined around the Fermi momentum if there is one.
pub fn state_nodes(mass: f64, mu: f64, delta: f64) -> Nodes {
    if mu > mass {
        let qf = (mu * mu - mass * mass).sqrt();
        nodes(&breaks_around(0.0, 1.0, Some(qf), 1e-3 * delta.max(1e-12)), 12)
    } else {
        nodes(&uniform(0.0, 1.0, 24), 12)
    }
}

pub fn response(d: &Dirac, mass: f64, mu: f64, delta: f64, nd: &Nodes) -> Resp {
    let (v1, r2, v3, vj) = vertices(d);
    let a = a7();
    let pts: Vec<(f64, f64)> = nd.x.iter().cloned().zip(nd.w.iter().cloned()).collect();
    let per = par_map(&pts, |&(qn, w)| {
        let mut q = [0.0; 7];
        q[1] = qn; // along x1 (SPACE[1] = 1)
        let h = bdg(d, &q, mass, mu, delta, &d.cp);
        let (ev, vecs) = eigh(&h);
        let occ: Vec<usize> = (0..32).filter(|&i| ev[i] < 0.0).collect();
        let emp: Vec<usize> = (0..32).filter(|&i| ev[i] >= 0.0).collect();
        let cols: Vec<Vec<f64>> = (0..32).map(|i| vecs.col(i)).collect();
        let mut elems = Vec::with_capacity(occ.len() * emp.len());
        let mut s = 0.0;
        for &ia in &occ {
            let va = &cols[ia];
            let y1 = v1.matvec(va);
            let y2 = r2.matvec(va);
            let y3 = v3.matvec(va);
            let yj: Vec<Vec<f64>> = vj.iter().map(|m| m.matvec(va)).collect();
            for &ib in &emp {
                let vb = &cols[ib];
                let om = ev[ib] - ev[ia];
                elems.push(Elem { om, m1: dot(vb, &y1), r2: dot(vb, &y2), m3: dot(vb, &y3) });
                let sj: f64 = yj.iter().map(|y| dot(vb, y).powi(2)).sum::<f64>() / 7.0;
                s += sj / om;
            }
        }
        (w * a * qn.powi(6), elems, s, occ.len() == 16)
    });
    let mut items = Vec::new();
    let mut stiff = 0.0;
    let mut occ_ok = true;
    for (wt, el, s, ok) in per {
        stiff += wt * s;
        occ_ok &= ok;
        items.push((wt, el));
    }
    let _ = SPACE;
    Resp { items, stiff, occ_ok }
}

/// The Hermitian matrix U'' + Pi(w), stored as its real diagonal (A11, A22, A33), the
/// real (1,3) entry and the imaginary parts p (entry (1,2) = i p) and t (entry (2,3) = i t).
#[derive(Clone, Copy, Debug)]
pub struct ModeMat {
    pub a11: f64,
    pub a22: f64,
    pub a33: f64,
    pub a13: f64,
    pub p: f64,
    pub t: f64,
}

pub fn mode_matrix(resp: &Resp, md: &Model, w: f64) -> ModeMat {
    let (mut c11, mut c22, mut c33, mut c13, mut p, mut t) = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    for (wt, el) in &resp.items {
        for e in el {
            let den = w * w - e.om * e.om;
            let f = 2.0 * e.om / den * wt;
            let g = 2.0 * w / den * wt;
            c11 += e.m1 * e.m1 * f;
            c22 += e.r2 * e.r2 * f;
            c33 += e.m3 * e.m3 * f;
            c13 += e.m1 * e.m3 * f;
            p += e.m1 * e.r2 * g;
            t -= e.r2 * e.m3 * g;
        }
    }
    let u33 = if md.gs > 0.0 { 1.0 / (2.0 * md.gs) } else { f64::INFINITY };
    ModeMat { a11: 2.0 / md.gp + c11, a22: 2.0 / md.gp + c22, a33: u33 + c33, a13: c13, p, t }
}

/// Determinant of U'' + Pi(w); without the scalar coupling the M row and column are absent.
pub fn det(mm: &ModeMat, with_mass: bool) -> f64 {
    if with_mass {
        mm.a11 * mm.a22 * mm.a33 - 2.0 * mm.p * mm.t * mm.a13 - mm.a11 * mm.t * mm.t - mm.a22 * mm.a13 * mm.a13 - mm.a33 * mm.p * mm.p
    } else {
        mm.a11 * mm.a22 - mm.p * mm.p
    }
}

/// The amplitude mode: (mass, threshold, kind) with kind 0 = bound below threshold, 1 = at threshold.
pub fn sigma_mass(resp: &Resp, md: &Model, mass: f64, delta: f64) -> (f64, f64, i32) {
    let thr = 2.0 * ((mass - md.mu).max(0.0).powi(2) + delta * delta).sqrt();
    let wm = md.gs > 0.0;
    let mut ws: Vec<f64> = (1..50).map(|k| thr * k as f64 / 50.0).collect();
    for k in 2..=9 {
        ws.push(thr * (1.0 - 10f64.powi(-k)));
    }
    let s0 = det(&mode_matrix(resp, md, ws[0]), wm).signum();
    let mut prev = ws[0];
    for &w in ws.iter().skip(1) {
        let s = det(&mode_matrix(resp, md, w), wm).signum();
        if s != s0 {
            let r = spacetimes::roots::brent(|x| det(&mode_matrix(resp, md, x), wm), prev, w, 1e-13 * thr, 200).unwrap_or(w);
            return (r, thr, 0);
        }
        prev = w;
    }
    (thr, thr, 1)
}

/// Normal-state surface term of the spatial stiffness (coefficient S_n with Omega_n(Q) - Omega_n(0) = -S_n Q^2).
pub fn surface(mass: f64) -> f64 {
    2.0 * a7() / 7.0 / (1.0 + mass * mass).sqrt()
}

/// f_s^2 = 2 (S_n - S_sf).
pub fn fs2(resp: &Resp, mass: f64) -> f64 {
    2.0 * (surface(mass) - resp.stiff)
}

fn write_rows(name: &str, meaning: &str, header: &[&str], rows: &[Vec<f64>]) {
    spacetimes::io::write_csv(name, &[("meaning", meaning.to_string()), ("units", "Lambda_U2 = 1".to_string())], header, rows).expect("csv");
}

pub fn run() -> bool {
    let mut ck = Checks::new("PROJECT spacetimes, Revision_3, command `modes`: collective modes (Goldstone, amplitude sigma_U2), phase stiffness and gauge-boson masses");
    let d = dirac();
    let a = a7();
    let mut summary: Vec<Vec<String>> = Vec::new();
    let mut put = |k: &str, v: f64, meaning: &str| summary.push(vec![k.to_string(), format!("{:.10e}", v), meaning.to_string()]);

    // ------------------------------------------------------------- M1: normal-state stiffness = surface term (mu = 0)
    let mut worst = 0.0f64;
    for &mass in &[0.0, 0.3] {
        let nd = nodes(&uniform(0.0, 1.0, 40), 16);
        let r = response(&d, mass, 0.0, 0.0, &nd);
        worst = worst.max((r.stiff / surface(mass) - 1.0).abs());
    }
    ck.add("R3-M1", "normal vacuum (Delta = 0, mu = 0, M = 0 and M = 0.3): the second-order response of the BdG states to a phase gradient equals the surface term (2 a7/7)/sqrt(1 + M^2) of the cutoff, Int_0^1 q^6 (6 q^2/7 + M^2)/eps^3 dq = 1/(7 sqrt(1+M^2)): the subtraction of the normal state removes exactly the artefact of the cutoff", worst < 1e-6, format!("largest relative difference {:.1e}", worst));

    // ------------------------------------------------------------- vacuum versus lambda (m = 0)
    let mut rows = Vec::new();
    let (mut ok_g, mut ok_thr, mut ok_fs, mut ok_hess) = (true, true, true, true);
    let mut worst_fs = 0.0f64;
    let mut worst_ft = 0.0f64;
    let mut worst_h = 0.0f64;
    let mut worst_gold = 0.0f64;
    let mut thr_ratio = Vec::new();
    for &lp in &[1.005, 1.01, 1.02, 1.05, 1.1, 1.2] {
        let md = Model::pairing(0.0, lp, 0.0);
        let dl = solve_delta(&md, 0.0);
        let nd = state_nodes(0.0, 0.0, dl);
        let resp = response(&d, 0.0, 0.0, dl, &nd);
        ok_g &= resp.occ_ok;
        let m0 = mode_matrix(&resp, &md, 0.0);
        let gold = m0.a22 / (2.0 / md.gp);
        worst_gold = worst_gold.max(gold.abs());
        // Hessian d^2 Omega/d Delta^2 by central differences
        let h = 1e-3 * dl;
        let hess = (omega7(&md, 0.0, dl + h) - 2.0 * omega7(&md, 0.0, dl) + omega7(&md, 0.0, dl - h)) / (h * h);
        worst_h = worst_h.max((m0.a11 / hess - 1.0).abs());
        ok_hess &= m0.a11 > 0.0;
        // amplitude mode at threshold: A11(w) vanishes linearly at w = 2 Delta
        let (ms, thr, kind) = sigma_mass(&resp, &md, 0.0, dl);
        let r1 = mode_matrix(&resp, &md, thr * (1.0 - 1e-4)).a11 / 1e-4;
        let r2 = mode_matrix(&resp, &md, thr * (1.0 - 1e-6)).a11 / 1e-6;
        ok_thr &= kind == 1 && (ms - 2.0 * dl).abs() < 1e-12 && (r1 / r2 - 1.0).abs() < 1e-2 && r2 > 0.0;
        thr_ratio.push(r1 / r2);
        // stiffness
        let fs = fs2(&resp, 0.0);
        let fs_an = 4.0 * a * crate::integrate::adaptive(&|q: f64| {
            let e = (q * q + dl * dl).sqrt();
            q.powi(6) * (6.0 * q * q / 7.0) * (1.0 / q.powi(3).max(1e-300) - 1.0 / e.powi(3))
        }, 0.0, 1.0, 1e-20);
        worst_fs = worst_fs.max((fs / fs_an - 1.0).abs());
        ok_fs &= fs > 0.0;
        let ft = ft2_fixed(0.0, 0.0, dl);
        // f_t^2 from -1/4 d^2/dmu^2 of the fermion energy at fixed Delta
        let hm = 1e-3;
        let ft_num = -0.25 * (fermion7(&md, 0.0, dl, hm) - 2.0 * fermion7(&md, 0.0, dl, 0.0) + fermion7(&md, 0.0, dl, -hm)) / (hm * hm);
        worst_ft = worst_ft.max((ft / ft_num - 1.0).abs());
        // R8 (Lorentz-invariant) stiffness f^2 = 8 b8 Delta^2 Int u^3/(u+s)^2, at the R8 gap of the same lambda
        let d8 = solve_delta8(lp * gcrit8(), 0.0);
        let f8 = 8.0 * b8() * d8 * d8 * f8pp(d8 * d8);
        rows.push(vec![lp, dl, ms / (2.0 * dl), fs, ft, (fs / ft).sqrt(), d8, f8]);
    }
    write_rows("r3_modes_vacuum.csv", "vacuum (mu = 0, m = 0): pair field, sigma mass in units of 2 Delta, spatial and time-like stiffness f_s^2, f_t^2 (units Lambda^6) in R7 and their ratio (the Goldstone velocity if U(1)_U2 were global), and the R8 pair field and its single (Lorentz-invariant) stiffness f^2", &["lambda", "Delta", "msigma_over_2Delta", "fs2", "ft2", "vG_R7", "Delta_R8", "f2_R8"], &rows);
    ck.add("R3-M2", "Goldstone theorem: at every vacuum minimum (lambda^P = 1.005 ... 1.2) the phase entry 2/G^P + Pi_22(0) of the Hessian vanishes (relative to 2/G^P) to the accuracy of the quadrature, and every q has exactly 16 occupied BdG states", ok_g && worst_gold < 1e-8, format!("largest |2/G^P + Pi_22(0)| G^P/2 = {:.1e}", worst_gold));
    ck.add("R3-M3", "the amplitude entry 2/G^P + Pi_11(0) equals d^2 Omega/d Delta^2 computed from the closed-form potential by central differences, and is positive (a true minimum)", ok_hess && worst_h < 1e-5, format!("largest relative difference {:.1e}", worst_h));
    ck.add("R3-M4", "the amplitude (sigma_U2) mode of the massless vacuum sits exactly at the pair-breaking threshold, m_sigma = 2 Delta: det[U'' + Pi(w)] has no zero in (0, 2 Delta), and 2/G^P + Pi_11(w) vanishes linearly as w -> 2 Delta (its ratio to 1 - w/2Delta is the same at 1e-4 and 1e-6 within 1 per cent)", ok_thr, format!("ratios {:?}", thr_ratio.iter().map(|x| format!("{:.5}", x)).collect::<Vec<_>>()));
    ck.add("R3-M5", "vacuum stiffness: the subtracted second-order response 2(S_n - S_sf) equals the closed form f_s^2 = 4 a7 Int q^6 (6q^2/7)(1/q^3 - 1/E^3) dq (m = 0), and is positive (the condensate resists phase gradients: Meissner effect for B_U2)", ok_fs && worst_fs < 1e-6, format!("largest relative difference {:.1e}", worst_fs));
    ck.add("R3-M6", "time-like stiffness: f_t^2 = 2 a7 Int q^6 Delta^2 (1/E+^3 + 1/E-^3) equals -(1/4) d^2/dmu^2 of the fermion energy at fixed Delta (central differences)", worst_ft < 1e-5, format!("largest relative difference {:.1e}", worst_ft));
    put("vac_l105_Delta", rows[3][1], "vacuum Delta at lambda^P = 1.05 (R7)");
    put("vac_l105_fs2", rows[3][3], "vacuum spatial stiffness f_s^2/Lambda^6 at lambda^P = 1.05 (R7)");
    put("vac_l105_ft2", rows[3][4], "vacuum time-like stiffness f_t^2/Lambda^6 at lambda^P = 1.05 (R7)");
    put("vac_l105_vG", rows[3][5], "sqrt(f_s^2/f_t^2) at lambda^P = 1.05 (R7): the regulator's breaking of Lorentz invariance");
    put("vac_l105_Delta_R8", rows[3][6], "vacuum Delta at lambda^P = 1.05 (R8)");
    put("vac_l105_f2_R8", rows[3][7], "Lorentz-invariant stiffness f^2/Lambda^6 at lambda^P = 1.05 (R8)");
    put("vac_l1005_vG", rows[0][5], "sqrt(f_s^2/f_t^2) at lambda^P = 1.005 (R7)");

    // ------------------------------------------------------------- M7: R8 stiffness closed form versus quadrature
    let mut worst = 0.0f64;
    for &(m, dd) in &[(0.0, 0.3), (0.2, 0.5), (0.0, 0.05)] {
        let s: f64 = m * m + dd * dd;
        let num = 16.0 * dd * dd * b8() * crate::integrate::adaptive(&|p: f64| p.powi(7) / (p * p + s).powi(2), 0.0, 1.0, 1e-18);
        worst = worst.max((num / (8.0 * b8() * dd * dd * f8pp(s)) - 1.0).abs());
    }
    ck.add("R3-M7", "Lorentz-invariant regularization R8: f^2 = 16 Delta^2 Int d^8p/(2pi)^8 (p^2 + M^2 + Delta^2)^-2 = 8 b8 Delta^2 Int_0^1 u^3/(u+s)^2 du (s = M^2 + Delta^2), one and the same for space-like and time-like gradients", worst < 1e-12, format!("largest relative difference {:.1e}", worst));

    // ------------------------------------------------------------- vacuum with a bare mass (lambda^P = 1.5, no scalar coupling)
    let mut rows = Vec::new();
    let mut ok_gold_m = true;
    for &m in &[0.0, 0.05, 0.1, 0.15, 0.2, 0.25] {
        let md = Model::pairing(m, 1.05, 0.0);
        let dl = solve_delta(&md, m);
        if dl <= 0.0 {
            rows.push(vec![m, 0.0, f64::NAN, f64::NAN, f64::NAN]);
            continue;
        }
        let nd = state_nodes(m, 0.0, dl);
        let resp = response(&d, m, 0.0, dl, &nd);
        let m0 = mode_matrix(&resp, &md, 0.0);
        ok_gold_m &= (m0.a22 * md.gp / 2.0).abs() < 1e-8;
        let (ms, thr, kind) = sigma_mass(&resp, &md, m, dl);
        rows.push(vec![m, dl, thr, ms, kind as f64]);
    }
    write_rows("r3_modes_mass.csv", "vacuum with a bare mass m (lambda^P = 1.05, mu = 0, no scalar coupling): pair field, threshold 2 sqrt(m^2 + Delta^2), sigma mass, kind (0 = bound state below threshold, 1 = at threshold)", &["m", "Delta", "threshold", "msigma", "kind"], &rows);
    ck.add("R3-M8", "Goldstone theorem with a bare mass m = 0 ... 0.25 (lambda^P = 1.05): the phase entry of the Hessian vanishes", ok_gold_m, String::new());
    // the amplitude mode with a bare mass: m_sigma = 2 Delta exactly, below the threshold 2 sqrt(m^2 + Delta^2)
    let mut ok_2d = true;
    let mut worst_2d = 0.0f64;
    for r in rows.iter().filter(|r| r[1] > 0.0) {
        worst_2d = worst_2d.max((r[3] / (2.0 * r[1]) - 1.0).abs());
        ok_2d &= if r[0] > 0.0 { r[4] == 0.0 && r[3] < r[2] } else { r[4] == 1.0 };
    }
    ck.add("R3-M12", "the amplitude mode in the vacuum with a bare mass m (mu = 0, no scalar coupling): m_sigma = 2 Delta exactly, as follows from 2/G^P + Pi_11(w) = c(w) (4 Delta^2 - w^2) (the radial and the angular directions of the plane (M, Delta) contribute (Delta/r)^2 c (4 r^2 - w^2) and -(M/r)^2 c w^2, r^2 = m^2 + Delta^2); for m > 0 it lies below the pair-breaking threshold 2 sqrt(m^2 + Delta^2): sigma_U2 is a stable particle", ok_2d && worst_2d < 1e-9, format!("largest |m_sigma/(2 Delta) - 1| = {:.1e}; {}", worst_2d, rows.iter().map(|r| format!("m={:.2}: m_sigma/threshold={:.6}", r[0], r[3] / r[2])).collect::<Vec<_>>().join(", ")));

    // ------------------------------------------------------------- finite density (m = 0, lambda^P = 0.6)
    let mus: Vec<f64> = vec![0.49, 0.50, 0.51, 0.52, 0.53, 0.54, 0.56, 0.58, 0.60];
    let mut rows = Vec::new();
    let mut ok_gd = true;
    let mut worst_lg = 0.0f64;
    let mut worst_vg = 0.0f64;
    for &mu in &mus {
        let md = Model::pairing(0.0, 0.5, mu);
        let mn = minimize(&md);
        let dl = mn.d;
        let nd = state_nodes(0.0, mu, dl);
        let resp = response(&d, 0.0, mu, dl, &nd);
        let m0 = mode_matrix(&resp, &md, 0.0);
        ok_gd &= resp.occ_ok && (m0.a22 * md.gp / 2.0).abs() < 1e-7;
        let (ms, thr, kind) = sigma_mass(&resp, &md, 0.0, dl);
        let fs = fs2(&resp, 0.0);
        let ftf = ft2_fixed(0.0, mu, dl);
        // relaxed time-like stiffness: (1/4) dn/dmu along the minimum
        let hm = 1e-4;
        let nmin = |m: f64| {
            let md2 = Model::pairing(0.0, 0.5, m);
            let mn2 = minimize(&md2);
            density(&md2, 0.0, mn2.d)
        };
        let ftr = 0.25 * (nmin(mu - 2.0 * hm) - 8.0 * nmin(mu - hm) + 8.0 * nmin(mu + hm) - nmin(mu + 2.0 * hm)) / (12.0 * hm);
        let n = density(&md, 0.0, dl);
        let leg = fs / (n / (4.0 * mu));
        let vg2 = fs / ftr;
        // logarithmic derivative of the gap along the minimum, d ln Delta / d mu (five-point differences)
        let dmin = |m: f64| minimize(&Model::pairing(0.0, 0.5, m)).d.ln();
        let dlog = (dmin(mu - 2.0 * hm) - 8.0 * dmin(mu - hm) + 8.0 * dmin(mu + hm) - dmin(mu + 2.0 * hm)) / (12.0 * hm);
        if dl < 0.02 {
            worst_lg = worst_lg.max((leg - 1.0).abs());
        }
        if dl * dlog < 0.1 {
            worst_vg = worst_vg.max((vg2 * 7.0 - 1.0).abs());
        }
        rows.push(vec![mu, dl, thr, ms, kind as f64, fs, ftf, ftr, n, n / (4.0 * mu), leg, vg2, dlog, dl * dlog]);
    }
    write_rows("r3_modes_density.csv", "finite density (m = 0, lambda^P = 0.5, no scalar coupling, R7): Delta, threshold 2 Delta, sigma mass and kind, f_s^2, f_t^2 at fixed Delta, f_t^2 relaxed = (1/4) dn/dmu, n, n/(4 mu), f_s^2/(n/(4 mu)), Goldstone velocity squared f_s^2/f_t^2(relaxed), d ln Delta/d mu along the minimum, and Delta d ln Delta/d mu", &["mu", "Delta", "threshold", "msigma", "kind", "fs2", "ft2_fixed", "ft2_relaxed", "n", "n_over_4mu", "fs2_over_n4mu", "vG2", "dlnDelta_dmu", "Delta_dlnDelta_dmu"], &rows);
    ck.add("R3-M9", "Goldstone theorem at finite density (mu = 0.49 ... 0.60, lambda^P = 0.5): the phase entry of the Hessian vanishes and every q has 16 occupied BdG states", ok_gd, String::new());
    ck.add("R3-M10", "relativistic superfluid at weak coupling: (a) for Delta < 0.02 the spatial stiffness equals n/(4 mu) within 1 per cent (the whole density is superfluid at zero temperature); (b) where the condensation energy varies slowly with mu (Delta d ln Delta/d mu < 0.1) the Goldstone velocity squared f_s^2/f_t^2(relaxed) equals 1/7 = 1/(number of space dimensions), the sound speed of a scale-free gas in 7 space dimensions, within 1 per cent; at larger Delta d ln Delta/d mu the steep growth of the gap with mu (d ln Delta/d mu = 6/(8 a7 mu^7 G^P) in the BCS regime) adds to the compressibility and lowers the velocity (the deviation grows roughly like (Delta d ln Delta/d mu)^2)", worst_lg < 0.01 && worst_vg < 0.01, format!("largest deviations {:.4}, {:.4}", worst_lg, worst_vg));
    let k3 = rows.iter().position(|r| (r[0] - 0.54).abs() < 1e-9).unwrap();
    put("den_mu054_Delta", rows[k3][1], "Delta at mu = 0.54, lambda^P = 0.5 (R7)");
    put("den_mu054_msigma_over_thr", rows[k3][3] / rows[k3][2], "sigma mass over threshold at mu = 0.54, lambda^P = 0.5");
    put("den_mu054_fs2_over_n4mu", rows[k3][10], "f_s^2/(n/(4 mu)) at mu = 0.54, lambda^P = 0.5");
    put("den_mu054_vG2", rows[k3][11], "Goldstone velocity squared at mu = 0.54, lambda^P = 0.5 (1/7 = 0.142857 for a scale-free gas)");

    // ------------------------------------------------------------- M11: static pair interaction mediated by sigma is attractive
    let mut ok = true;
    let mut vals = Vec::new();
    for (m, lp, mu) in [(0.0, 1.05, 0.0), (0.0, 0.5, 0.56), (0.1, 1.05, 0.0)] {
        let md = Model::pairing(m, lp, mu);
        let dl = solve_delta(&md, m);
        let resp = response(&d, m, mu, dl, &state_nodes(m, mu, dl));
        let m0 = mode_matrix(&resp, &md, 0.0);
        // static exchange of sigma in the pair channel: T_sigma(0) = -1/(d^2 Omega/d Delta^2); bare contact -G^P/2
        let t = -1.0 / m0.a11;
        vals.push(format!("(m={},lambda={},mu={}): T_sigma(0)/(-G^P/2) = {:.4}", m, lp, mu, t / (-md.gp / 2.0)));
        ok &= t < 0.0;
    }
    ck.add("R3-M11", "the static pair interaction carried by the amplitude field, T_sigma(0) = -[d^2 Omega/d Delta^2]^-1, is negative (attractive) at every minimum tested; it is the zero-frequency value of the sigma_U2 propagator -[U'' + Pi(w)]^-1_11, whose pole is the sigma_U2 mass", ok, vals.join("; "));

    let _ = gcrit7();
    spacetimes::io::write_csv_str("r3_summary_modes.csv", &[("meaning", "key numbers of the command `modes` (units Lambda_U2 = 1)".to_string())], &["name", "value", "meaning"], &summary).expect("csv");
    ck.write("r3_modes_checks.txt")
}
