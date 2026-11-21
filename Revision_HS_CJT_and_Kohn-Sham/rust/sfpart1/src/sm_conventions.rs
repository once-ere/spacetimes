//! Check SM-1: the conventions of the Standard Model that the author fixed (Section 1 of superfluid.md).
//!
//! With the covariant derivative of the Higgs doublet
//!     D_mu Phi = (d_mu + i g sum_a (tau_a/2) W^a_mu + i g' Y B_mu) Phi,   Y = 1/2,
//! and the vacuum <Phi> = (0, v)/sqrt 2, the kinetic term of the constant vacuum is the quadratic form
//!     (D <Phi>)^dag (D <Phi>) = (1/2) sum_ab M_ab V^a_mu V^(b,mu)
//! of the four fields V = (W^1, W^2, W^3, B), with
//!     M_ab = 2 Re(X_a^dag X_b),   X_a = i g (tau_a/2) <Phi> (a = 1, 2, 3),   X_4 = i g' Y <Phi>.
//! The four orthonormal vectors e_1, e_2 (the fields W^1, W^2), Z = (0, 0, g, -g')/(g^2 + g'^2)^(1/2) and
//! A = (0, 0, g', g)/(g^2 + g'^2)^(1/2) are eigenvectors of M with the eigenvalues M_W^2 = (g v/2)^2 (twice),
//! M_Z^2 = (g^2 + g'^2) v^2/4 and 0: the photon A = sin(theta_W) W^3 + cos(theta_W) B (tan theta_W = g'/g) is
//! massless and Z = cos(theta_W) W^3 - sin(theta_W) B has the mass M_Z.  Since the four vectors span R^4, this fixes
//! all eigenvalues of M.  The check is made for 20 triples (g, g', v) of a fixed grid.
use sfcore::c64::{c, C64};
use sfcore::report::Report;

/// the Pauli matrix tau_a (a = 1, 2, 3) applied to a doublet
fn tau(a: usize, x: [C64; 2]) -> [C64; 2] {
    match a {
        1 => [x[1], x[0]],
        2 => [C64::I.scale(-1.0) * x[1], C64::I * x[0]],
        _ => [x[0], -x[1]],
    }
}

pub fn run(rep: &mut Report) {
    let mut dev: f64 = 0.0;
    for k in 0..20 {
        let g = 0.3 + 0.05 * k as f64;
        let gp = 0.1 + 0.03 * ((7 * k) % 20) as f64;
        let v = 0.5 + 0.1 * ((3 * k) % 20) as f64;
        let vac = [C64::ZERO, c(v / 2f64.sqrt(), 0.0)];
        // the vectors X_a of D_mu <Phi> = sum_a V^a_mu X_a
        let mut xs = [[C64::ZERO; 2]; 4];
        for a in 1..4 {
            let t = tau(a, vac);
            xs[a - 1] = [C64::I * t[0] * c(g / 2.0, 0.0), C64::I * t[1] * c(g / 2.0, 0.0)];
        }
        xs[3] = [C64::I * vac[0] * c(gp / 2.0, 0.0), C64::I * vac[1] * c(gp / 2.0, 0.0)];
        // the mass matrix M_ab = 2 Re(X_a^dag X_b)
        let mut m = [[0.0f64; 4]; 4];
        for a in 0..4 {
            for b in 0..4 {
                let s = xs[a][0].conj() * xs[b][0] + xs[a][1].conj() * xs[b][1];
                m[a][b] = 2.0 * s.re;
            }
        }
        // the predicted eigenvectors and eigenvalues
        let nrm = (g * g + gp * gp).sqrt();
        let (mw2, mz2) = ((g * v / 2.0).powi(2), (g * g + gp * gp) * v * v / 4.0);
        let vecs: [([f64; 4], f64); 4] = [
            ([1.0, 0.0, 0.0, 0.0], mw2),
            ([0.0, 1.0, 0.0, 0.0], mw2),
            ([0.0, 0.0, g / nrm, -gp / nrm], mz2),
            ([0.0, 0.0, gp / nrm, g / nrm], 0.0),
        ];
        for (u, lam) in vecs.iter() {
            for a in 0..4 {
                let mu: f64 = (0..4).map(|b| m[a][b] * u[b]).sum();
                dev = dev.max((mu - lam * u[a]).abs() / mz2);
            }
        }
    }
    rep.add(
        "SM-1",
        "Standard Model with +i g: the mass matrix of (W^1, W^2, W^3, B) from (D <Phi>)^dag (D <Phi>) has the eigenvectors W^1, W^2 (M_W^2 = (g v/2)^2), Z = cos(thetaW) W^3 - sin(thetaW) B (M_Z^2 = (g^2 + g'^2) v^2/4) and the massless photon A = sin(thetaW) W^3 + cos(thetaW) B (20 triples g, g', v; largest residual relative to M_Z^2)",
        dev,
        1e-14,
    );
}
