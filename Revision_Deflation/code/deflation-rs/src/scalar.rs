//! The two modes of the Vorton-Scalar on the deflating spacetimes.
//!
//! FIELD EQUATION.  On either deflating spacetime the covariant wave operator of a
//! scalar field S(x) is, with a prime for d/dx4,
//!
//!   7+1:  (1/sqrt|g|) d_mu ( sqrt|g| g^{mu nu} d_nu S ) =  S'' - gsum S' - Lap S - sum_t b_t^{-2} d_t^2 S ,
//!   4+4:  the same operator up to an overall sign, with  + sum_t b_t^{-2} d_t^2 S  (x5, x6, x7 are times),
//!
//! where Lap is the Laplacian of the flat coordinates x0..x3, gsum = g55 + g66 + g77 and
//! sqrt|g| = b_5 b_6 b_7 falls as exp(-gsum x4).  The first-derivative term has the sign
//! opposite to the friction term of an expanding universe: the deflation feeds the field.
//!
//! For a field of mass M proportional to exp(i k.x + i kappa_t x^t), the weighted field
//! Shat = sqrt(b_5 b_6 b_7) S = exp(-gsum x4/2) S obeys
//!
//!   Shat'' + [ Omega0^2 + sgn Q(x4)^2 ] Shat = 0 ,     Omega0^2 = k^2 + M^2 - gsum^2/4 ,
//!
//! with sgn = +1 on 7+1 and -1 on 4+4 and Q^2 = sum_t kappa_t^2/b_t^2.  The radial mode of
//! the Vorton-Scalar has M = m_s, the phase mode M = 0.
//!
//! WHAT THIS MODULE COMPUTES.
//!  (a) Fields that do not depend on x5, x6, x7 (Q = 0): S goes as exp(sigma x4) with
//!      sigma = gsum/2 +- sqrt(gsum^2/4 - k^2 - M^2).  For k^2 + M^2 > gsum^2/4 the weighted
//!      field oscillates with the constant frequency Omega0 (the number of quanta is
//!      conserved); for k^2 + M^2 < gsum^2/4 it does not oscillate.  For the phase mode the
//!      band k < gsum/2 always exists.
//!  (b) The field of a static source.  A source at rest that is spread uniformly over the
//!      deflating coordinates has a proper density proportional to 1/sqrt|g|, that is to
//!      exp(+gsum x4).  For S'' - gsum S' + (k^2 + M^2) S = J exp(gsum x4) the particular
//!      solution is S = J exp(gsum x4)/(k^2 + M^2), exactly: the field of the source is the
//!      Yukawa field of the mass M itself, with a strength that grows as exp(gsum x4); the
//!      shift -gsum^2/4 does not enter.  `static_source_response` integrates the equation
//!      from zero initial data and shows that the solution approaches this form.
//!  (c) 7+1, Q != 0: the number of quanta that the deflation creates in a mode that starts
//!      empty.  For one non-zero kappa_t the solutions are Bessel functions of the order
//!      i Omega0/rate_t and the number is exactly 1/(exp(2 pi Omega0/rate_t) - 1), a
//!      Bose-Einstein law with the temperature rate_t/(2 pi).  Computed numerically here.
//!  (d) 4+4, Q != 0: beyond the turning value of x4 (Q = Omega0) one solution grows and
//!      one decays; the bounded solution is the modified Bessel function K_{i nu}(s), s = Q/rate_t,
//!      nu = Omega0/rate_t, a standing wave at early times.
//!
//! METHOD.  The second-order equation is written as a first-order system for (y, y') and
//! integrated with the same commutator-free Magnus method of order four as the Dirac
//! modes (`modes.rs`); the 2x2 exponents square to multiples of the unit matrix.

use crate::modes::{bessel_k, Background};
use spacetimes::complex::C64;

const CF4_C1: f64 = 0.5 - 0.288_675_134_594_812_9;
const CF4_C2: f64 = 0.5 + 0.288_675_134_594_812_9;
const CF4_A1: f64 = 0.25 + 0.288_675_134_594_812_9;
const CF4_A2: f64 = 0.25 - 0.288_675_134_594_812_9;

/// The two exponents sigma of S ~ exp(sigma x4) for a field that does not depend on the
/// deflating coordinates: sigma = gsum/2 +- sqrt(gsum^2/4 - w2), w2 = k^2 + M^2.
/// Returns (real part of both, imaginary part or 0, real square root or 0).
pub fn zero_mode_exponents(w2: f64, gsum: f64) -> (C64, C64) {
    let disc = 0.25 * gsum * gsum - w2;
    if disc >= 0.0 {
        let r = disc.sqrt();
        (C64::new(0.5 * gsum + r, 0.0), C64::new(0.5 * gsum - r, 0.0))
    } else {
        let r = (-disc).sqrt();
        (C64::new(0.5 * gsum, r), C64::new(0.5 * gsum, -r))
    }
}

/// One step of the Magnus method for y'' + w2(t) y = 0 (w2 may be negative), state (y, y') complex.
fn step<F: Fn(f64) -> f64>(w2: &F, t: f64, dt: f64, y: &mut [C64; 2]) {
    let (w1, w2v) = (w2(t + CF4_C1 * dt), w2(t + CF4_C2 * dt));
    for (wa, wb) in [(CF4_A1, CF4_A2), (CF4_A2, CF4_A1)] {
        // exponent dt A, A = [[0, c0], [-cw, 0]], c0 = wa + wb, cw = wa w1 + wb w2;  A^2 = -c0 cw
        let c0 = wa + wb;
        let cw = wa * w1 + wb * w2v;
        let s = -c0 * cw;
        let rt = s.abs().sqrt();
        let th = dt * rt;
        let (f1, f2) = if rt < 1e-300 {
            (1.0, dt)
        } else if s > 0.0 {
            (th.cosh(), th.sinh() / rt)
        } else {
            (th.cos(), th.sin() / rt)
        };
        let (a, b) = (y[0], y[1]);
        y[0] = a * f1 + b * (f2 * c0);
        y[1] = b * f1 - a * (f2 * cw);
    }
}

/// Result of the calculation of boson creation in one mode (7+1).
#[derive(Clone, Copy, Debug)]
pub struct BosonCreation {
    pub omega0: f64,
    /// number of created quanta (first-order adiabatic number at the late time)
    pub n: f64,
    /// |alpha|^2 - |beta|^2 - 1: deviation from the Bogoliubov normalization
    pub norm_dev: f64,
    pub steps: usize,
}

/// 7+1: number of quanta created in the mode with the squared frequency
/// w2(x4) = omega0^2 + Q(x4)^2, starting empty at Q = q_start omega0 and ending at
/// Q = q_end omega0.  Positive- and negative-frequency parts are separated with the WKB
/// modes of first adiabatic order, f = exp(-i Int omega)/sqrt(2 omega), f' = (-i omega - omega'/(2 omega)) f.
pub fn boson_creation(omega0: f64, kappa: &[f64; 3], bg: &Background, q_start: f64, q_end: f64, phase: f64, slow: f64) -> BosonCreation {
    let t0 = bg.x4_where_q_is(kappa, q_start * omega0);
    let t1 = bg.x4_where_q_is(kappa, q_end * omega0);
    let w2 = |t: f64| -> f64 {
        let q = bg.q_norm(kappa, t);
        omega0 * omega0 + q * q
    };
    // omega and d omega/dx4
    let om = |t: f64| -> (f64, f64) {
        let q = bg.q(kappa, t);
        let o = (omega0 * omega0 + q[0] * q[0] + q[1] * q[1] + q[2] * q[2]).sqrt();
        let d = (bg.rate[0] * q[0] * q[0] + bg.rate[1] * q[1] * q[1] + bg.rate[2] * q[2] * q[2]) / o;
        (o, d)
    };
    // in-mode: y = f, y' = f' at t0
    let (o0, d0) = om(t0);
    let amp = 1.0 / (2.0 * o0).sqrt();
    let mut y = [C64::new(amp, 0.0), C64::new(-d0 / (2.0 * o0) * amp, -o0 * amp)];
    let rmax = bg.rate.iter().cloned().fold(0.0, f64::max);
    let mut t = t0;
    let mut steps = 0;
    while t < t1 {
        let mut dt = slow / rmax;
        for _ in 0..3 {
            dt = dt.min(phase / w2(t + dt).sqrt());
        }
        if t + dt > t1 {
            dt = t1 - t;
        }
        step(&w2, t, dt, &mut y);
        t += dt;
        steps += 1;
    }
    // decomposition at t1:  y = alpha f + beta f^*,  with f f^*' - f' f^* = i:
    //   beta = i (f y' - f' y) ,  alpha = -i (f^* y' - f^*' y)   (up to phases that do not matter)
    let (o1, d1) = om(t1);
    let f = C64::new(1.0 / (2.0 * o1).sqrt(), 0.0);
    let fp = C64::new(-d1 / (2.0 * o1), -o1) * f;
    let beta = f * y[1] - fp * y[0];
    let alpha = f.conj() * y[1] - fp.conj() * y[0];
    let (b2, a2) = (beta.norm_sqr(), alpha.norm_sqr());
    BosonCreation { omega0, n: b2, norm_dev: a2 - b2 - 1.0, steps }
}

/// The exact number of created quanta for one non-zero kappa_t: 1/(exp(2 pi omega0/rate) - 1).
pub fn bose_einstein_creation(omega0: f64, rate: f64) -> f64 {
    1.0 / ((2.0 * std::f64::consts::PI * omega0 / rate).exp() - 1.0)
}

/// 4+4: the bounded solution of Shat'' + [omega0^2 - Q(x4)^2] Shat = 0, integrated
/// backwards from the x4 at which the WKB exponent beyond the turning value is `s_late`
/// down to Q = q_early omega0.  Returns the samples (x4, Q/omega0, Shat^2 normalized to
/// its maximum) and the ratio of the amplitudes of the two travelling waves
/// exp(-+ i omega0 x4) at the early time, |c_-/c_+| (1 for a standing wave).
pub fn boson_bounded(omega0: f64, kappa: &[f64; 3], bg: &Background, s_late: f64, q_early: f64, phase: f64, slow: f64) -> (Vec<[f64; 3]>, f64) {
    let rmax = bg.rate.iter().cloned().fold(0.0, f64::max);
    let x4_turn = bg.x4_where_q_is(kappa, omega0);
    let mut t = x4_turn;
    let mut s_acc = 0.0;
    let dts = 1e-4 / rmax;
    while s_acc < s_late {
        let q = bg.q_norm(kappa, t + 0.5 * dts);
        s_acc += dts * (q * q - omega0 * omega0).max(0.0).sqrt();
        t += dts;
    }
    let x4_late = t;
    let x4_early = bg.x4_where_q_is(kappa, q_early * omega0);
    let w2 = |t: f64| -> f64 {
        let q = bg.q_norm(kappa, t);
        omega0 * omega0 - q * q
    };
    // decaying solution at x4_late: y' = -lambda y
    let lam = (-w2(x4_late)).sqrt();
    let mut y = [C64::ONE, C64::new(-lam, 0.0)];
    let mut prof: Vec<[f64; 3]> = vec![[x4_late, bg.q_norm(kappa, x4_late) / omega0, 1.0]];
    let dt_prof = (x4_late - x4_early) / 4000.0;
    let mut next = x4_late - dt_prof;
    while t > x4_early {
        let scale = w2(t).abs().sqrt().max(omega0);
        let mut dt = (phase / scale).min(slow / rmax);
        if t - dt < x4_early {
            dt = t - x4_early;
        }
        step(&w2, t, -dt, &mut y);
        t -= dt;
        if t <= next {
            prof.push([t, bg.q_norm(kappa, t) / omega0, y[0].norm_sqr()]);
            next = t - dt_prof;
        }
    }
    // travelling-wave amplitudes at the early time: y = c_+ e^{-i w t} + c_- e^{+i w t}
    //   c_+ = (y + i y'/w)/2 e^{+i w t},  c_- = (y - i y'/w)/2 e^{-i w t}
    let cp = (y[0] + C64::I * y[1] * (1.0 / omega0)) * 0.5;
    let cm = (y[0] - C64::I * y[1] * (1.0 / omega0)) * 0.5;
    let ratio = cm.abs() / cp.abs();
    let pmax = prof.iter().map(|r| r[2]).fold(0.0, f64::max);
    for r in prof.iter_mut() {
        r[2] /= pmax;
    }
    prof.reverse();
    (prof, ratio)
}

/// Shat^2 of the exact bounded solution for one non-zero kappa_t, up to a constant:
/// K_{i nu}(s)^2 with s = Q/rate_t and nu = omega0/rate_t (K of imaginary order is real).
pub fn boson_bounded_exact(s: f64, nu: f64) -> f64 {
    bessel_k(C64::new(0.0, nu), s).norm_sqr()
}

/// The field of a static source on the deflating background (item (b) of the module
/// documentation).  Integrates S'' - gsum S' + w2 S = exp(gsum x4) from S = S' = 0 at
/// x4 = 0 to x4 = x4_end with the classical Runge-Kutta method and returns the samples
/// (x4, S(x4) w2 exp(-gsum x4)): the ratio of the solution to the exact particular
/// solution exp(gsum x4)/w2.  It tends to 1; the difference falls as exp(-gsum x4/2).
pub fn static_source_response(w2: f64, gsum: f64, x4_end: f64, n: usize) -> Vec<[f64; 2]> {
    // variables u = S exp(-gsum x4) (so that the target is the constant 1/w2):
    //   u'' + gsum u' + w2 u = 1
    let h = x4_end / n as f64;
    let f = |u: f64, v: f64| -> (f64, f64) { (v, 1.0 - gsum * v - w2 * u) };
    let (mut u, mut v) = (0.0_f64, 0.0_f64);
    let mut out: Vec<[f64; 2]> = vec![[0.0, 0.0]];
    for i in 0..n {
        let (k1u, k1v) = f(u, v);
        let (k2u, k2v) = f(u + 0.5 * h * k1u, v + 0.5 * h * k1v);
        let (k3u, k3v) = f(u + 0.5 * h * k2u, v + 0.5 * h * k2v);
        let (k4u, k4v) = f(u + h * k3u, v + h * k3v);
        u += h / 6.0 * (k1u + 2.0 * k2u + 2.0 * k3u + k4u);
        v += h / 6.0 * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);
        out.push([(i + 1) as f64 * h, u * w2]);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exponents_of_fields_that_do_not_depend_on_the_deflating_coordinates() {
        // massless, k = 0: sigma = gsum and 0
        let (a, b) = zero_mode_exponents(0.0, 3.0);
        assert!((a.re - 3.0).abs() < 1e-15 && b.re.abs() < 1e-15 && a.im == 0.0);
        // k^2 + M^2 = 4, gsum = 2: sigma = 1 +- i sqrt(3)
        let (a, b) = zero_mode_exponents(4.0, 2.0);
        assert!((a.re - 1.0).abs() < 1e-15 && (a.im - 3f64.sqrt()).abs() < 1e-15 && (b.im + 3f64.sqrt()).abs() < 1e-15);
    }

    #[test]
    fn boson_creation_in_one_direction_is_a_bose_einstein_distribution() {
        let bg = Background { rate: [1.0, 2.0, 0.5], cst: [1.0, 0.7, 1.3] };
        for (omega0, kappa, rate) in [(1.0, [1.0, 0.0, 0.0], 1.0), (0.4, [0.0, 0.3, 0.0], 2.0), (0.8, [0.0, 0.0, 2.0], 0.5)] {
            let r = boson_creation(omega0, &kappa, &bg, 1e-5, 300.0, 0.05, 0.01);
            let exact = bose_einstein_creation(omega0, rate);
            assert!(r.norm_dev.abs() < 1e-8, "normalization {}", r.norm_dev);
            assert!(((r.n - exact) / exact).abs() < 1e-4, "numerical {}, exact {}", r.n, exact);
        }
    }

    #[test]
    fn bounded_scalar_solution_is_a_standing_wave_with_the_exact_profile() {
        let bg = Background { rate: [1.0, 1.0, 1.0], cst: [1.0, 1.0, 1.0] };
        let (prof, ratio) = boson_bounded(1.5, &[1.0, 0.0, 0.0], &bg, 25.0, 1e-6, 0.02, 0.005);
        assert!((ratio - 1.0).abs() < 1e-6, "ratio of the travelling waves {}", ratio);
        let mut rs: Vec<f64> = Vec::new();
        for row in prof.iter().step_by(40) {
            let s = row[1] * 1.5;
            if s > 0.05 && s < 10.0 {
                let ex = boson_bounded_exact(s, 1.5);
                if ex > 1e-12 * 1.0 && row[2] > 1e-9 {
                    rs.push(row[2] / ex);
                }
            }
        }
        let mean = rs.iter().sum::<f64>() / rs.len() as f64;
        // near the zeros of the standing wave the ratio is ill-conditioned; compare where the wave is not small
        let good = rs.iter().filter(|x| ((*x / mean) - 1.0).abs() < 1e-4).count();
        assert!(good as f64 > 0.8 * rs.len() as f64, "{} of {} samples agree", good, rs.len());
    }

    #[test]
    fn field_of_a_static_source_approaches_the_yukawa_form_with_growing_strength() {
        // gsum = 0.2, k^2 + m_s^2 = 1: after x4 = 80 the transient exp(-0.1 x4) has fallen to exp(-8)
        let r = static_source_response(1.0, 0.2, 80.0, 80000);
        let last = r.last().unwrap()[1];
        assert!((last - 1.0).abs() < 2.0 * (-8.0f64).exp(), "ratio {}", last);
        // and the approach is not faster than the transient allows: at x4 = 20 the deviation is of the order exp(-2)
        let mid = r[20000][1];
        assert!((mid - 1.0).abs() < (-2.0f64).exp() * 1.5);
    }
}
