//! Two particles of the field in the hidden-axis singlet channel at low density:
//! bound state ("dimer") and low-energy scattering in the attractive Yukawa potential
//! of the exchange of the radial mode of the Vorton-Scalar,
//!
//!     V(r) = - alpha exp(-m_s r) / r ,      alpha = g^2 / (4 pi) ,  g = m / v_VS .
//!
//! (Non-relativistic limit of the exchange amplitude F_S of `kernel.rs`: for momenta
//! much smaller than the fermion mass m, F_S -> 1, and the Fourier transform of
//! -g^2/(q^2 + m_s^2) is the potential above.  The phase mode contributes only at
//! relative order q^2/m^2.)
//!
//! UNITS.  Lengths in units of 1/m_s, so that with rho = m_s r the radial equation of
//! the s wave, for two particles of mass m (reduced mass m/2) and energy E, is
//!
//!     u''(rho) + D exp(-rho)/rho u(rho) = eps u(rho) ,
//!     D = alpha m / m_s ,      eps = - m E / m_s^2 .
//!
//! One dimensionless number, D, decides everything:
//!  * a bound state exists if and only if D > D_c = 1.6798 (computed below);
//!  * its binding energy is  E_b = eps(D) m_s^2 / m ;
//!  * the scattering length is  a = a_hat(D) / m_s , with a_hat = -D for D << 1 (Born
//!    approximation) and a_hat -> +- infinity at D_c.
//!
//! METHOD.  The equation is integrated outwards from rho = 0 with the classical
//! fourth-order Runge-Kutta method on a grid that is uniform in t = ln(1 + rho/rho_0),
//! which resolves the 1/rho singularity of the potential; the regular solution starts
//! as u = rho - D rho^2/2 + ... .

/// Integrate u'' = (eps - D exp(-rho)/rho) u from rho ~ 0 to rho_max.
/// Returns (u, du/drho, number of nodes of u in (0, rho_max)).
fn integrate(d: f64, eps: f64, rho_max: f64, n: usize) -> (f64, f64, usize) {
    let rho0 = 0.05;
    let t_max = (1.0 + rho_max / rho0).ln();
    let h = t_max / n as f64;
    // start slightly off the origin with the series  u = rho - D rho^2/2 + (D^2 + 2D + 2 eps) rho^3/12
    let t_start = 1e-6;
    let r = rho0 * (f64::exp(t_start) - 1.0);
    let mut u = r - 0.5 * d * r * r + (d * d + 2.0 * d + 2.0 * eps) * r * r * r / 12.0;
    let mut v = 1.0 - d * r + (d * d + 2.0 * d + 2.0 * eps) * r * r / 4.0; // du/drho
    let mut t = t_start;
    let mut nodes = 0;
    // with rho = rho0 (e^t - 1):  du/dt = v drho/dt,  dv/dt = (eps - D e^-rho/rho) u drho/dt
    let f = |t: f64, u: f64, v: f64| -> (f64, f64) {
        let rho = rho0 * (t.exp() - 1.0);
        let dr = rho0 * t.exp();
        (v * dr, (eps - d * (-rho).exp() / rho) * u * dr)
    };
    let steps = ((t_max - t_start) / h).ceil() as usize;
    let h = (t_max - t_start) / steps as f64;
    for _ in 0..steps {
        let (k1u, k1v) = f(t, u, v);
        let (k2u, k2v) = f(t + 0.5 * h, u + 0.5 * h * k1u, v + 0.5 * h * k1v);
        let (k3u, k3v) = f(t + 0.5 * h, u + 0.5 * h * k2u, v + 0.5 * h * k2v);
        let (k4u, k4v) = f(t + h, u + h * k3u, v + h * k3v);
        let un = u + h / 6.0 * (k1u + 2.0 * k2u + 2.0 * k3u + k4u);
        let vn = v + h / 6.0 * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);
        if un * u < 0.0 {
            nodes += 1;
        }
        u = un;
        v = vn;
        t += h;
        // keep the numbers in range for large eps
        if u.abs() > 1e200 {
            u *= 1e-200;
            v *= 1e-200;
        }
    }
    (u, v, nodes)
}

/// Scattering length in units of 1/m_s:  u -> const (rho - a_hat) outside the potential.
pub fn scattering_length(d: f64) -> f64 {
    let rho_max = 60.0;
    let (u, v, _) = integrate(d, 0.0, rho_max, 40_000);
    rho_max - u / v
}

/// Number of s-wave bound states = number of nodes of the zero-energy solution in
/// (0, infinity).  Beyond rho_max the potential is negligible and the solution is a
/// straight line, which has one more node if u and u' have opposite signs at rho_max.
pub fn bound_states(d: f64) -> usize {
    let (u, v, nodes) = integrate(d, 0.0, 60.0, 40_000);
    nodes + if u * v < 0.0 { 1 } else { 0 }
}

/// eps = m E_b / m_s^2 of the lowest bound state, or None if there is none.
pub fn binding(d: f64) -> Option<f64> {
    if bound_states(d) == 0 {
        return None;
    }
    // For eps above the ground-state value the outward solution has no node and grows;
    // below it, it has at least one node.  Bisect on the number of nodes.
    let nodes = |eps: f64| -> usize {
        // integrate far enough for the decaying solution exp(-sqrt(eps) rho)
        let rho_max = (30.0 / eps.sqrt()).clamp(20.0, 4000.0);
        integrate(d, eps, rho_max, 60_000).2
    };
    let (mut lo, mut hi) = (1e-12, 0.25 * d * d + 1.0); // eps < D^2/4 (Coulomb limit)
    if nodes(lo) == 0 {
        return Some(0.0); // bound more weakly than the resolution
    }
    for _ in 0..70 {
        let mid = 0.5 * (lo + hi);
        if nodes(mid) == 0 {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    Some(0.5 * (lo + hi))
}

/// The critical value D_c above which the potential binds the pair.
pub fn critical_d() -> f64 {
    let (mut lo, mut hi) = (1.0, 2.5);
    for _ in 0..50 {
        let mid = 0.5 * (lo + hi);
        if bound_states(mid) == 0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn born_limit_of_the_scattering_length() {
        // a_hat = -D (1 + O(D)) : second Born term gives -D - D^2/2 ...; test the leading behaviour
        for &d in &[1e-3, 1e-2] {
            let a = scattering_length(d);
            assert!((a / (-d) - 1.0).abs() < 2.0 * d, "D = {}: a_hat = {}", d, a);
        }
    }

    #[test]
    fn critical_coupling() {
        // literature value for the Yukawa potential: alpha (m/2) / m_s = 0.8399, i.e. D_c = 1.6798
        let dc = critical_d();
        assert!((dc - 1.6798).abs() < 2e-4, "D_c = {}", dc);
        // the scattering length changes sign through infinity at D_c
        assert!(scattering_length(dc - 0.01) < -50.0 && scattering_length(dc + 0.01) > 50.0);
        assert!(binding(1.5).is_none() && binding(2.0).is_some());
    }

    #[test]
    fn coulomb_limit_of_the_binding_energy() {
        // perturbation theory in the screening: eps = D^2/4 - D + 3/2 - 2/D + O(1/D^2)
        for &d in &[20.0, 40.0] {
            let e = binding(d).unwrap();
            let pt = 0.25 * d * d - d + 1.5 - 2.0 / d;
            assert!((e / pt - 1.0).abs() < 2e-3, "D = {}: {} vs {}", d, e, pt);
        }
    }

    #[test]
    fn shallow_bound_state_and_scattering_length() {
        // just above threshold the binding energy is E_b = 1/(m a^2) up to effective-range
        // corrections, i.e. eps = 1/a_hat^2 (1 + O(r_e/a))
        let d = 1.72;
        let e = binding(d).unwrap();
        let a = scattering_length(d);
        assert!(a > 10.0);
        assert!((e * a * a - 1.0).abs() < 0.2, "eps a^2 = {}", e * a * a);
    }
}
