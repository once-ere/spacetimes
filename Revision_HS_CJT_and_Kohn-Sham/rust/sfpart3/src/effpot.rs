//! The effective potentials V^X of the three methods (Section 12 of superfluid.md): the constrained effective potential
//! as a function of the field Phi^X of the superfluid boson singlet state (the pair self-energy amplitude of a state),
//! its symmetries, its minimum, and its curvatures along the amplitude and along the non-unitary directions.
use sfcore::c64::{c, C64};
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::methods::M_LL;
use sfcore::report::Report;
use sfcore::uniform::Model;

/// the field Phi^iota of a state: the components of the pair self-energy of the method's functional,
/// P = sum_iota Phi^iota (K^iota)^dag
pub fn phi_of(model: &Model, drho: &CMat, kap: &CMat) -> Vec<C64> {
    let (_, _, p) = model.interaction(drho, kap);
    (1..4).map(|i| k_scalar(i).mul(&p).trace().scale(1.0 / 8.0)).collect()
}

/// one point of the constrained effective potential: the state of the applied pair field P (with a self-consistent
/// normal self-energy); returns (Phi^1, Phi^2, Phi^3, energy, Dirac mass, normal self-energy)
pub fn point(model: &Model, p_applied: &CMat, n0: &CMat) -> (Vec<C64>, f64, f64, CMat) {
    let (n, drho, kap, e, _, _) = model.fixed_pair_state(p_applied, n0, 0.7, 1e-13, 40000);
    let ph = phi_of(model, &drho, &kap);
    let md = M_LL + d8(&beta()).mul(&n).trace().re / 8.0;
    (ph, e, md, n)
}

pub struct Curve {
    /// (applied amplitude, Phi^3, energy per volume, Dirac mass)
    pub rows: Vec<(f64, f64, f64, f64)>,
    pub v2_amp: f64,
    pub v2_nu: f64,
    pub slope_at_min: f64,
    /// the Landau fit V = a Phi^2 + b Phi^4 + c Phi^6 on [0, 1.4 Phi0] and its largest residual
    pub landau: (f64, f64, f64, f64),
    /// the largest deviation of the normal self-energies of the curve from a multiple of beta (x) 1_2 (a Dirac mass)
    pub ndev: f64,
}

/// the least-squares fit of y = a x^2 + b x^4 + c x^6 (normal equations, 3x3)
fn landau_fit(pts: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let mut m = [[0.0; 3]; 3];
    let mut r = [0.0; 3];
    for &(x, y) in pts {
        let b = [x * x, x.powi(4), x.powi(6)];
        for i in 0..3 {
            for j in 0..3 {
                m[i][j] += b[i] * b[j];
            }
            r[i] += b[i] * y;
        }
    }
    // Gaussian elimination
    for k in 0..3 {
        for i in (k + 1)..3 {
            let f = m[i][k] / m[k][k];
            for j in k..3 {
                m[i][j] -= f * m[k][j];
            }
            r[i] -= f * r[k];
        }
    }
    let mut x = [0.0; 3];
    for k in (0..3).rev() {
        let mut s = r[k];
        for j in (k + 1)..3 {
            s -= m[k][j] * x[j];
        }
        x[k] = s / m[k][k];
    }
    let res = pts.iter().fold(0.0f64, |s, &(p, y)| s.max((x[0] * p * p + x[1] * p.powi(4) + x[2] * p.powi(6) - y).abs()));
    (x[0], x[1], x[2], res)
}

pub fn curve(rep: &mut Report, tag: &str, model: &Model, n_gs: &CMat, phi0: f64, e0: f64, npts: usize) -> Curve {
    let k3d = k_scalar(3).dagger();
    let mut rows = vec![];
    let mut n0 = CMat::zeros(8);
    let b8 = d8(&beta());
    let mut ndev = 0.0f64;
    for i in 0..=npts {
        let fa = 2.0 * phi0 * i as f64 / npts as f64;
        let (ph, e, md, n) = point(model, &k3d.scale_r(fa), &n0);
        // the normal self-energy minus its Dirac-mass part (tr((beta (x) 1_2) N)/8) beta (x) 1_2
        ndev = ndev.max(n.sub(&b8.scale(b8.mul(&n).trace().scale(1.0 / 8.0))).max_abs());
        n0 = n;
        rows.push((fa, ph[2].re, e, md));
    }
    // the minimum: the applied field equal to the ground-state amplitude gives the ground state itself
    let (ph, e, _, _) = point(model, &k3d.scale_r(phi0), n_gs);
    rep.add(&format!("P3-1 {}", tag), &format!("{}: the applied field Phi0 = {:.10} gives back Phi^3 = Phi0 and the ground-state energy {:.10e} of part II", tag, phi0, e0), (ph[2].re - phi0).abs().max((e - e0).abs() / e0.abs()), 1e-9);
    // the curvature along the amplitude: a local polynomial of degree 4 in (Phi - Phi0) through 5 points
    let h = 0.02 * phi0;
    let mut loc = vec![];
    for k in -2i32..=2 {
        let (ph, e, _, _) = point(model, &k3d.scale_r(phi0 + k as f64 * h), n_gs);
        loc.push((ph[2].re - phi0, e - e0));
    }
    // fit e = s x + (v2/2) x^2 + t x^3 + u x^4 (5 points, exact interpolation)
    let mut a = CMat::zeros(5);
    let mut bvec = vec![C64::ZERO; 5];
    for (i, &(x, y)) in loc.iter().enumerate() {
        for j in 0..5 {
            a.set(i, j, c(x.powi(j as i32), 0.0));
        }
        bvec[i] = c(y, 0.0);
    }
    let coef = sfcore::linalg::lu_solve(&a, &bvec).unwrap();
    let slope = coef[1].re;
    let v2_amp = 2.0 * coef[2].re;
    rep.add(&format!("P3-2 {}", tag), &format!("{}: the effective potential is stationary at its minimum (slope dV/dPhi, relative to Phi0 V'')", tag), slope.abs() / (v2_amp * phi0), 1e-6);
    // the symmetries: a phase of the applied field, and the isospin direction 1 instead of 3
    let (_, e_ph, _, _) = point(model, &k3d.scale(C64::cis(0.7)).scale_r(0.8 * phi0), n_gs);
    let (_, e_k1, _, _) = point(model, &k_scalar(1).dagger().scale_r(0.8 * phi0), n_gs);
    let (_, e_ref, _, _) = point(model, &k3d.scale_r(0.8 * phi0), n_gs);
    rep.add(&format!("P3-3 {}", tag), &format!("{}: the effective potential depends only on |Phi| (a phase of U(1)F and the isospin direction 1 give the same energy)", tag), (e_ph - e_ref).abs().max((e_k1 - e_ref).abs()) / e0.abs(), 1e-10);
    // the curvature along the non-unitary direction Phi = Phi0 e3 + i nu e1, corrected for the small amplitude change
    let mut vn = vec![];
    for nu in [0.01 * phi0, 0.02 * phi0] {
        let p = k3d.scale_r(phi0).add(&k_scalar(1).dagger().scale(c(0.0, nu)));
        let (ph, e, _, _) = point(model, &p, n_gs);
        let nu_r = ph[0].im;
        let da = ph[2].re - phi0;
        vn.push(2.0 * (e - e0 - 0.5 * v2_amp * da * da) / (nu_r * nu_r));
    }
    let v2_nu = (4.0 * vn[0] - vn[1]) / 3.0;
    rep.add_bool(&format!("P3-4 {}", tag), &format!("{}: the polar minimum is a minimum also in the non-unitary directions (curvature {:.6e} > 0)", tag, v2_nu), v2_nu > 0.0, "");
    // the origin is a maximum (the Mexican hat): V < 0 near the origin, V(0) = 0
    let small = rows[1];
    rep.add_bool(&format!("P3-5 {}", tag), &format!("{}: Mexican hat: V(0) = {:.1e} and V < 0 near the origin (V({:.4}) = {:.6e})", tag, rows[0].2, small.1, small.2), rows[0].2.abs() < 1e-14 && small.2 < 0.0, "");
    let pts: Vec<(f64, f64)> = rows.iter().filter(|r| r.1 <= 1.4 * phi0).map(|r| (r.1, r.2)).collect();
    let landau = landau_fit(&pts);
    Curve { rows, v2_amp, v2_nu, slope_at_min: slope, landau, ndev }
}
