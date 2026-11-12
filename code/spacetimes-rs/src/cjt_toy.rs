//! The method of Cornwall, Jackiw and Tomboulis (CJT) in the simplest possible setting:
//! a "field theory" in zero space-time dimensions, where every quantity is an ordinary
//! integral and the method can be compared with the exact answer.
//!
//! THE MODEL.  One real variable phi with the Euclidean action
//!
//!     S(phi) = phi^2 / 2 + lambda phi^4 / 24            (unit "mass": m^2 = 1),
//!
//! the "partition function"  Z = Int dphi/sqrt(2 pi) exp(-S)  (so that Z = 1 for
//! lambda = 0) and the "full propagator"  G = <phi^2> = Int phi^2 exp(-S) / Int exp(-S).
//!
//! THE CJT EFFECTIVE ACTION for this model is the function of ONE variable G
//!
//!     Gamma(G) = (1/2) ln(1/G) + (1/2) (G - 1) + Gamma_2(G),
//!
//! in which the first two terms are the zero-dimensional forms of
//! (1/2) Tr ln G^-1 + (1/2) Tr (D^-1 G - 1)  (free propagator D = 1), and Gamma_2 is
//! minus the sum of the two-particle-irreducible (2PI) vacuum diagrams built with the
//! FULL propagator G on every line:
//!
//!     Gamma_2(G) =  lambda G^2 / 8        (two loops: the "double bubble")
//!                 - lambda^2 G^4 / 48     (three loops: the "basketball")
//!                 + lambda^3 G^6 / 48     (four loops)
//!                 - 5 lambda^4 G^8 / 128  (five loops)
//!                 + O(lambda^5).
//!
//! The physical G makes Gamma stationary:  dGamma/dG = 0  is the gap equation
//!
//!     1/G = 1 + Sigma(G),
//!     Sigma = 2 dGamma_2/dG = lambda G/2 - lambda^2 G^3/6 + lambda^3 G^5/4 - 5 lambda^4 G^7/8 + ... ,
//!
//! and the value of Gamma at the stationary point is  -ln Z.
//!
//! WHAT THE MODULE DOES.  It computes Z and G exactly by quadrature, solves the gap
//! equation with Gamma_2 truncated at two to five loops, and compares; the unit tests
//! verify the four coefficients 1/8, 1/48, 1/48, 5/128 numerically against the exact
//! integral.  (The coefficients were obtained by inverting the perturbation series
//! G = 1 - lambda/2 + 2 lambda^2/3 - 11 lambda^3/8 + 34 lambda^4/9 - ... , which follows
//! from the Gaussian moments <phi^(2k)> = (2k-1)!!; see supplement S4, Sec. 1.7.)  This is the test bed for the statement "a truncation of Gamma_2
//! is a self-consistent resummation of perturbation theory" on which the applications
//! of the method to the two models rest (cjt_pairing.rs, cjt_normal.rs, cjt_vacuum.rs).

use crate::quad::uniform_grid;
use crate::roots::brent;

/// Exact (Z, G) by Gauss-Legendre quadrature on [-L, L]; the integrand at |phi| = L = 12
/// is below exp(-72).
pub fn exact(lambda: f64) -> (f64, f64) {
    let grid = uniform_grid(-12.0, 12.0, 240, 8);
    let w: Vec<f64> = grid.x.iter().map(|&x| (-0.5 * x * x - lambda * x.powi(4) / 24.0).exp()).collect();
    let w2: Vec<f64> = grid.x.iter().zip(&w).map(|(&x, &e)| x * x * e).collect();
    let z = grid.integrate(&w);
    (z / (2.0 * std::f64::consts::PI).sqrt(), grid.integrate(&w2) / z)
}

/// Gamma_2 truncated: `loops` = 2, 3, 4 or 5 is the largest number of loops kept.
pub fn gamma2(loops: usize, lambda: f64, g: f64) -> f64 {
    let mut s = lambda * g * g / 8.0;
    if loops >= 3 {
        s -= lambda * lambda * g.powi(4) / 48.0;
    }
    if loops >= 4 {
        s += lambda.powi(3) * g.powi(6) / 48.0;
    }
    if loops >= 5 {
        s -= 5.0 * lambda.powi(4) * g.powi(8) / 128.0;
    }
    s
}

/// The self-energy Sigma = 2 dGamma_2/dG of the same truncation.
pub fn sigma(loops: usize, lambda: f64, g: f64) -> f64 {
    let mut s = lambda * g / 2.0;
    if loops >= 3 {
        s -= lambda * lambda * g.powi(3) / 6.0;
    }
    if loops >= 4 {
        s += lambda.powi(3) * g.powi(5) / 4.0;
    }
    if loops >= 5 {
        s -= 5.0 * lambda.powi(4) * g.powi(7) / 8.0;
    }
    s
}

/// The CJT effective action of the truncation.
pub fn effective_action(loops: usize, lambda: f64, g: f64) -> f64 {
    0.5 * (1.0 / g).ln() + 0.5 * (g - 1.0) + gamma2(loops, lambda, g)
}

/// Solution of the truncated gap equation 1/G = 1 + Sigma(G): the root that is
/// connected to G = 1 at lambda = 0 (the first sign change of 1/G - 1 - Sigma when G
/// decreases from 1 ... or, if there is none, the first one above 1).  None if the
/// truncated equation has no real solution in (0, 2).
pub fn solve_gap(loops: usize, lambda: f64) -> Option<f64> {
    let h = |g: f64| 1.0 / g - 1.0 - sigma(loops, lambda, g);
    // scan downwards from 1 (interactions reduce the propagator), then upwards
    let n = 4000;
    let mut prev = (1.0, h(1.0));
    if prev.1 == 0.0 {
        return Some(1.0);
    }
    for i in 1..n {
        let g = 1.0 - i as f64 / n as f64;
        let v = h(g);
        if v * prev.1 < 0.0 {
            return brent(h, g, prev.0, 1e-15, 200).ok();
        }
        prev = (g, v);
    }
    prev = (1.0, h(1.0));
    for i in 1..n {
        let g = 1.0 + i as f64 / n as f64;
        let v = h(g);
        if v * prev.1 < 0.0 {
            return brent(h, prev.0, g, 1e-15, 200).ok();
        }
        prev = (g, v);
    }
    None
}

/// Ordinary perturbation theory for G to order lambda^(loops-1):
/// G = 1 - lambda/2 + 2 lambda^2/3 - 11 lambda^3/8 + 34 lambda^4/9 - ...
pub fn perturbative(loops: usize, lambda: f64) -> f64 {
    let mut s = 1.0;
    if loops >= 2 {
        s -= lambda / 2.0;
    }
    if loops >= 3 {
        s += 2.0 * lambda * lambda / 3.0;
    }
    if loops >= 4 {
        s -= 11.0 * lambda.powi(3) / 8.0;
    }
    if loops >= 5 {
        s += 34.0 * lambda.powi(4) / 9.0;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The quadrature reproduces the first terms of the perturbation series.
    #[test]
    fn exact_integral_against_perturbation_series() {
        let l = 0.01;
        let (z, g) = exact(l);
        // Z = 1 - l/8 + 35 l^2/384 - 385 l^3/3072 + ...
        assert!((z - (1.0 - l / 8.0 + 35.0 * l * l / 384.0 - 385.0 * l.powi(3) / 3072.0)).abs() < 3e-9);
        assert!((g - perturbative(5, l)).abs() < 3e-9);
    }

    /// The coefficients of Gamma_2.  With the exact G, the exact self-energy is
    /// 1/G - 1.  Subtracting the terms of Sigma up to n loops must leave
    /// c lambda^n G^(2n-1) + O(lambda^(n+1)), where c is the coefficient of the next
    /// term: 1/2, -1/6, 1/4, -5/8 for n = 1, 2, 3, 4.  The limit lambda -> 0 of the
    /// remainder divided by lambda^n is taken by linear extrapolation from two small
    /// couplings.  A wrong coefficient at any order makes the next remainder diverge
    /// like 1/lambda instead of converging.
    #[test]
    fn two_particle_irreducible_coefficients() {
        for (loops, next) in [(1usize, 0.5f64), (2, -1.0 / 6.0), (3, 0.25), (4, -0.625)] {
            let q = |l: f64| {
                let (_, g) = exact(l);
                let kept = if loops == 1 { 0.0 } else { sigma(loops, l, g) };
                (1.0 / g - 1.0 - kept) / l.powi(loops as i32)
            };
            let c = 2.0 * q(0.005) - q(0.01);
            assert!((c - next).abs() < 2e-3 * next.abs().max(0.1), "after {} loops the next coefficient is {} (expected {})", loops, c, next);
        }
    }

    /// At the stationary point the effective action equals -ln Z, with an error that
    /// decreases with the number of loops kept (moderate coupling).
    #[test]
    fn stationary_value_is_minus_log_z() {
        let l = 0.3;
        let (z, g_exact) = exact(l);
        let mut last = f64::MAX;
        for loops in 2..=4 {
            let g = solve_gap(loops, l).unwrap();
            let err = (effective_action(loops, l, g) + z.ln()).abs();
            assert!(err < last, "error must decrease: {} loops, {}", loops, err);
            last = err;
            assert!((g - g_exact).abs() < 0.02);
        }
        assert!(last < 3e-4, "four-loop error {}", last);
    }

    /// The two-loop (Hartree) truncation has the closed solution G = (sqrt(1 + 2 lambda) - 1)/lambda.
    #[test]
    fn hartree_closed_form() {
        for &l in &[0.1, 1.0, 10.0] {
            let g = solve_gap(2, l).unwrap();
            assert!((g - ((1.0 + 2.0 * l).sqrt() - 1.0) / l).abs() < 1e-12);
        }
    }
}
