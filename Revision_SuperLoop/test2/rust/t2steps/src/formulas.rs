//! The error of ONE step of a linear multistep formula on an oscillation, computed from the formula itself.
//!
//! THE QUESTION.  A linear equation dy/dt = lambda y with lambda = -i E (E real) has the solution y(t) = exp(lambda t)
//! y(0): in a step of the size h the exact solution is multiplied by exp(z), z = lambda h = -i E h.  A linear multistep
//! formula that is used with a constant step size h, a constant order and the same treatment of its implicit equation
//! in every step multiplies the numerical solution in every step by a constant factor kappa(z) (after the first few
//! steps: the "principal root" of the recurrence of the formula).  This module computes
//!
//!     eps(z) = kappa(z) exp(-z) - 1 ,
//!
//! the RELATIVE ERROR OF ONE STEP: the numerical solution after N such steps is exp(N z) (1 + eps)^N times its start,
//! and as long as N |eps| is small against 1 its relative error is N eps.  The step errors of one oscillation add up
//! with the same phase.  |eps| is of the order |z|^(q+1) for a formula of the order q, so for the tolerances of Test 2
//! it lies between 1e-9 and 1e-17: the difference of two numbers that agree to 9 to 17 digits.  All computations of
//! this module are therefore made with the double-double numbers of the module `dd` (31 digits).
//!
//! THE FORMULAS.  nabla is the backward difference: nabla y_n = y_n - y_(n-1).  On a geometric sequence y_n = kappa^n it
//! multiplies by u = 1 - 1/kappa, so nabla^j y_n = u^j y_n.
//!
//! (1) Adams-Moulton formula of the order q (the formulas of CVODE for nonstiff problems; q = 1 ... 12):
//!         y_n = y_(n-1) + h sum_(j=0..q-1) c_AM_j nabla^j f_n ,       f_n = f(t_n, y_n) ,
//!     with c_AM_0 = 1 and c_AM_j = - sum_(i=0..j-1) c_AM_i / (j - i + 1) (-1/2, -1/12, -1/24, -19/720, ...).
//!     If the implicit equation is solved exactly (f_n = lambda y_n) the factor kappa = 1/(1 - u) is given by
//!         u = z sum_(j=0..q-1) c_AM_j u^j ,
//!     and eps = - c_AM_q z^(q+1) (1 + O(z)).
//!
//! (2) The same formula as CVODE applies it with its fixed-point iteration, m iterations per step ("P(EC)^m").  CVODE
//!     keeps, besides y_n, the derivatives it has stored at the last q step ends; write G_n for h times the stored
//!     derivative of the step n.  One step:
//!         predictor (Adams-Bashforth formula of the order q):  P    = y_(n-1) + sum_(i=1..q) b_AB_i G_(n-i)
//!         the explicit part of the corrector:                   base = y_(n-1) + sum_(i=1..q-1) b_AM_i G_(n-i)
//!         iteration:  y^(0) = P ,   y^(j+1) = base + b_AM_0 h f(y^(j))      (j = 0 ... m-1)
//!         result:     y_n = y^(m) ,   G_n = h f(y^(m-1)) .
//!     (b_AM_i and b_AB_i are the formulas (1) and Adams-Bashforth written in the values f_(n-i) instead of the
//!     differences.)  The stored derivative G_n belongs to the iterate BEFORE the accepted one; for m = 1 it is the
//!     derivative of the predicted value.  With y_n = kappa^n, G_n = G kappa^n and x = 1/kappa this gives two equations
//!     for x and G, which are reduced below to one equation for x.  For m -> infinity it is formula (1).
//!
//! (3) Backward differentiation formula of the order k (the formulas of IDA; k = 1 ... 5):
//!         sum_(j=1..k) (1/j) nabla^j y_n = h f_n .
//!     Solved exactly: B(u) = z with B(u) = sum_(j=1..k) u^j / j, and eps = z^(k+1)/(k+1) (1 + O(z)).
//!
//! (4) The same formula as IDA applies it: m steps of Newton's method from the predicted value, with an iteration
//!     matrix that was made for another step size or order.  IDA predicts y_n by the polynomial through the last k+1
//!     values, P = y_n - nabla^(k+1) y_n, and solves F(y) = y'(y) - lambda y = 0, where y'(y) is linear in y with the
//!     slope cj = H_k / h (H_k = 1 + 1/2 + ... + 1/k).  Its matrix is cjJ - lambda with the cj of the last time the
//!     matrix was computed, cjJ = cj / r; IDA recomputes the matrix only when r leaves the interval (0.6, 5/3), and it
//!     multiplies every Newton correction by 2/(1 + r).  One Newton step therefore moves the fraction
//!         rho_1 = (2/(1 + r)) (H_k - z) / (H_k / r - z)
//!     of the way from the predicted value to the exact solution Y of the formula, m steps the fraction
//!     rho = 1 - (1 - rho_1)^m:   y_n = P + rho (Y - P).  On y_n = kappa^n this gives
//!         B(u) = z - ((1 - rho)/rho) (H_k - z) u^(k+1) ,
//!     and eps = ( 1/(k+1) + ((rho - 1)/rho) H_k ) z^(k+1) (1 + O(z)).  For r = 1 (a current matrix) rho = 1 and this
//!     is formula (3).  The recurrence (4) has k+1 roots; besides the principal one a root near -1 appears, an error
//!     that changes its sign from step to step.  Its modulus grows with r and passes 1 at r = 1.305 for k = 5: beyond
//!     that ratio one Newton step per step is unstable (function `bdf_alternating_root`).
use crate::dd::{Cdd, Dd};
use t2core::c64::C64;

/// H_k = 1 + 1/2 + ... + 1/k
pub fn harmonic(k: usize) -> Dd {
    let mut s = Dd::ZERO;
    for j in 1..=k {
        s = s + Dd::ONE / Dd::from_f64(j as f64);
    }
    s
}

/// c_AM_0 ... c_AM_n, the coefficients of the Adams-Moulton formulas in backward differences
pub fn c_am(n: usize) -> Vec<Dd> {
    let mut g = vec![Dd::ONE];
    for j in 1..=n {
        let mut s = Dd::ZERO;
        for i in 0..j {
            s = s + g[i] / Dd::from_f64((j - i + 1) as f64);
        }
        g.push(-s);
    }
    g
}

/// c_AB_0 ... c_AB_n, the coefficients of the Adams-Bashforth formulas in backward differences
/// (1, 1/2, 5/12, 3/8, 251/720, ...): c_AB_j = 1 - sum_(i=0..j-1) c_AB_i / (j - i + 1)
pub fn c_ab(n: usize) -> Vec<Dd> {
    let mut g = vec![Dd::ONE];
    for j in 1..=n {
        let mut s = Dd::ZERO;
        for i in 0..j {
            s = s + g[i] / Dd::from_f64((j - i + 1) as f64);
        }
        g.push(Dd::ONE - s);
    }
    g
}

/// the binomial coefficient, as a double-double number (exact for the small arguments used here)
fn binomial(n: usize, k: usize) -> Dd {
    let mut b = 1.0f64;
    for i in 0..k {
        b = b * ((n - i) as f64) / ((i + 1) as f64);
    }
    Dd::from_f64(b.round())
}

/// b_AM_0 ... b_AM_(q-1): the Adams-Moulton formula of the order q written in the values,
/// y_n = y_(n-1) + h sum_i b_AM_i f_(n-i);  b_AM_i = (-1)^i sum_(j=i..q-1) binomial(j, i) c_AM_j
pub fn b_am(q: usize) -> Vec<Dd> {
    let gs = c_am(q);
    (0..q)
        .map(|i| {
            let mut s = Dd::ZERO;
            for j in i..q {
                s = s + binomial(j, i) * gs[j];
            }
            if i % 2 == 1 {
                -s
            } else {
                s
            }
        })
        .collect()
}

/// b_AB_1 ... b_AB_q (returned with the index shifted by one: element i-1 is b_AB_i): the Adams-Bashforth formula of the order q
/// written in the values, y_n = y_(n-1) + h sum_(i=1..q) b_AB_i f_(n-i);  b_AB_i = (-1)^(i-1) sum_(j=i-1..q-1) binomial(j, i-1) c_AB_j
pub fn b_ab(q: usize) -> Vec<Dd> {
    let g = c_ab(q);
    (1..=q)
        .map(|i| {
            let mut s = Dd::ZERO;
            for j in (i - 1)..q {
                s = s + binomial(j, i - 1) * g[j];
            }
            if (i - 1) % 2 == 1 {
                -s
            } else {
                s
            }
        })
        .collect()
}

/// the polynomial sum_i c_i x^(i + shift)
fn poly(c: &[Dd], shift: usize, x: Cdd) -> Cdd {
    let mut s = Cdd::ZERO;
    for &ci in c.iter().rev() {
        s = s * x + Cdd::real(ci);
    }
    for _ in 0..shift {
        s = s * x;
    }
    s
}

fn powi(x: Cdd, n: u32) -> Cdd {
    let mut p = Cdd::ONE;
    for _ in 0..n {
        p = p * x;
    }
    p
}

/// The relative error of one step, eps = kappa exp(-z) - 1, of the Adams-Moulton formula of the order q.
/// `iterations = None`: the implicit equation is solved exactly (formula (1) of the module text);
/// `iterations = Some(m)`: m fixed-point iterations from the Adams-Bashforth predictor, the stored derivative is that
/// of the iterate before the last (formula (2)).
pub fn adams_eps(q: usize, z: Cdd, iterations: Option<u32>) -> Cdd {
    assert!(q >= 1);
    let expm = (-z).exp();
    match iterations {
        None => {
            // Newton's method for g(u) = u - z sum_j c_AM_j u^j, from u = z
            let gs = c_am(q);
            let mut u = z;
            for _ in 0..60 {
                let (mut s, mut ds) = (Cdd::ZERO, Cdd::ZERO);
                for j in (0..q).rev() {
                    ds = ds * u + s;
                    s = s * u + Cdd::real(gs[j]);
                }
                let g = u - z * s;
                let dg = Cdd::ONE - z * ds;
                let step = g / dg;
                u = u - step;
                if step.abs_f64() < 1e-33 * u.abs_f64().max(1e-300) {
                    break;
                }
            }
            expm / (Cdd::ONE - u) - Cdd::ONE
        }
        Some(m) => {
            assert!(m >= 1);
            let bam = b_am(q);
            let b = b_ab(q);
            let b0z = z.scale(bam[0]);
            let kp = powi(b0z, m - 1); // (b_AM_0 z)^(m-1)
            let k = kp * b0z; // (b_AM_0 z)^m
            let d = Cdd::ONE - b0z;
            // the residual of the single equation for x = 1/kappa (derivation in the document; the stored derivative
            // G is eliminated with the first equation, which is linear in G)
            let f = |x: Cdd| -> Cdd {
                let sab = poly(&b, 1, x);
                let sam = if q > 1 { poly(&bam[1..], 1, x) } else { Cdd::ZERO };
                let one_k = Cdd::ONE - k;
                let g = (Cdd::ONE - x * (one_k / d + k)) / (one_k * sam / d + k * sab);
                let yc = (x + g * sam) / d;
                let p = x + g * sab;
                z * (yc * (Cdd::ONE - kp) + kp * p) - g
            };
            // the secant method from x = exp(-z)
            let mut x0 = expm;
            let mut x1 = expm * Cdd::from_c64(C64 { re: 1.0 + 1e-9, im: 1e-9 });
            let mut f0 = f(x0);
            for _ in 0..80 {
                let f1 = f(x1);
                let denom = f1 - f0;
                if denom.abs_f64() == 0.0 {
                    break;
                }
                let x2 = x1 - f1 * (x1 - x0) / denom;
                let change = (x2 - x1).abs_f64();
                x0 = x1;
                f0 = f1;
                x1 = x2;
                if change < 1e-32 {
                    break;
                }
            }
            expm / x1 - Cdd::ONE
        }
    }
}

/// All roots kappa of the recurrence (2) (the Adams-Moulton formula of the order q with m fixed-point iterations from
/// the Adams-Bashforth predictor, as CVODE applies it), the largest modulus first (f64 precision).  The recurrence
/// acts on the q + 1 numbers (y_n, G_n, ..., G_(n-q+1)), so it has q + 1 roots: the principal one, close to exp(z),
/// and q others.  With x = 1/kappa and the polynomials A(x) = sum_(i=1..q) b_AB(i) x^i, M(x) = sum_(i=1..q-1)
/// b_AM(i) x^i, the two equations of the recurrence on y_n = kappa^n, G_n = G kappa^n,
///     1 = Yc + (b0 z)^m (P - Yc) ,   G = z (Yc + (b0 z)^(m-1) (P - Yc)) ,   P = x + G A ,   Yc = (x + G M)/(1 - b0 z) ,
/// are linear in G; eliminating G gives the polynomial equation of the degree q + 1
///     (1 - c1 x)(1 - z T2(x)) - c2 z x T1(x) = 0 ,
///     T1 = (1 - k) M/D + k A ,  T2 = (1 - k') M/D + k' A ,  c1 = (1 - k)/D + k ,  c2 = (1 - k')/D + k' ,
/// with k = (b0 z)^m, k' = (b0 z)^(m-1), D = 1 - b0 z and b0 = b_AM(0).
pub fn adams_roots(q: usize, z: C64, m: u32) -> Vec<C64> {
    assert!(q >= 1 && m >= 1);
    let bam: Vec<f64> = b_am(q).iter().map(|x| x.to_f64()).collect();
    let b: Vec<f64> = b_ab(q).iter().map(|x| x.to_f64()).collect();
    let one = C64 { re: 1.0, im: 0.0 };
    let b0z = z.scale(bam[0]);
    let mut kp = one;
    for _ in 0..(m - 1) {
        kp = kp * b0z;
    }
    let k = kp * b0z;
    let d = one - b0z;
    let c1 = (one - k) / d + k;
    let c2 = (one - kp) / d + kp;
    // the coefficients of T1 and T2 (powers x^0 ... x^q)
    let mut t1 = vec![C64::ZERO; q + 1];
    let mut t2 = vec![C64::ZERO; q + 1];
    for i in 1..=q {
        let a = C64 { re: b[i - 1], im: 0.0 };
        let mm = if i < q { C64 { re: bam[i], im: 0.0 } } else { C64::ZERO };
        t1[i] = (one - k) / d * mm + k * a;
        t2[i] = (one - kp) / d * mm + kp * a;
    }
    // (1 - c1 x)(1 - z T2) - c2 z x T1, powers x^0 ... x^(q+1)
    let mut co = vec![C64::ZERO; q + 2];
    for i in 0..=q {
        let f = if i == 0 { one } else { C64::ZERO } - z * t2[i];
        co[i] += f;
        co[i + 1] = co[i + 1] - c1 * f;
        co[i + 1] = co[i + 1] - c2 * z * t1[i];
    }
    let mut kappa: Vec<C64> = polynomial_roots(&co).iter().map(|x| one / *x).collect();
    kappa.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
    kappa
}

/// the roots of the polynomial with the coefficients co[0] + co[1] x + ... (Durand-Kerner iteration, f64); leading
/// coefficients that vanish (to 1e-14 of the largest coefficient) are dropped
fn polynomial_roots(co: &[C64]) -> Vec<C64> {
    let scale = co.iter().map(|c| c.abs()).fold(0.0f64, f64::max);
    let mut deg = co.len() - 1;
    while deg > 0 && co[deg].abs() <= 1e-14 * scale {
        deg -= 1;
    }
    if deg == 0 {
        return Vec::new();
    }
    let one = C64 { re: 1.0, im: 0.0 };
    let lead = co[deg];
    let eval = |x: C64| -> C64 {
        let mut s = C64::ZERO;
        for a in co[..=deg].iter().rev() {
            s = s * x + *a;
        }
        s / lead
    };
    // a bound of the moduli of the roots (Cauchy) gives the radius of the starting circle
    let radius = 1.0 + co[..deg].iter().map(|c| (*c / lead).abs()).fold(0.0f64, f64::max);
    let mut roots: Vec<C64> = (0..deg).map(|i| C64::cis(0.4 + 2.0 * std::f64::consts::PI * i as f64 / deg as f64).scale(radius.min(4.0))).collect();
    for _ in 0..5000 {
        let mut change = 0.0f64;
        for i in 0..deg {
            let mut den = one;
            for j in 0..deg {
                if j != i {
                    den = den * (roots[i] - roots[j]);
                }
            }
            let delta = eval(roots[i]) / den;
            roots[i] = roots[i] - delta;
            change = change.max(delta.abs() / roots[i].abs().max(1e-300));
        }
        if change < 1e-15 {
            break;
        }
    }
    roots
}

/// The largest modulus of the roots of the recurrence (2) other than the principal root, for z = -i `z_abs`: the
/// factor per step of the slowest-decaying (or fastest-growing) error that is not the solution itself.  The scheme
/// is unstable for this z if the number exceeds 1.
pub fn adams_parasitic_modulus(q: usize, z_abs: f64, m: u32) -> f64 {
    let z = C64 { re: 0.0, im: -z_abs };
    let roots = adams_roots(q, z, m);
    let principal = z.exp();
    // the principal root is the root nearest to exp(z)
    let ip = (0..roots.len()).min_by(|&a, &b| (roots[a] - principal).abs().partial_cmp(&(roots[b] - principal).abs()).unwrap()).expect("roots");
    roots.iter().enumerate().filter(|(i, _)| *i != ip).map(|(_, r)| r.abs()).fold(0.0f64, f64::max)
}

/// The stability limit of the recurrence (2) on the imaginary axis: the smallest |z| (searched in steps of 0.002 up
/// to 1, then refined by bisection) at which a root other than the principal one reaches the modulus 1; None if
/// there is none up to |z| = 1.  For z -> 0 all other roots tend to zero, so the recurrence is stable for small
/// enough |z|; for the high orders with one iteration the limit lies below the first point of the search and is
/// found by bisection between 1e-6 and that point.
pub fn adams_stability_limit(q: usize, m: u32) -> Option<f64> {
    let step = 0.002;
    let mut lo = step;
    if adams_parasitic_modulus(q, lo, m) >= 1.0 {
        let (mut stable, mut bad) = (1e-6f64, lo);
        if adams_parasitic_modulus(q, stable, m) >= 1.0 {
            return Some(0.0);
        }
        for _ in 0..50 {
            let mid = 0.5 * (stable + bad);
            if adams_parasitic_modulus(q, mid, m) >= 1.0 {
                bad = mid;
            } else {
                stable = mid;
            }
        }
        return Some(0.5 * (stable + bad));
    }
    let mut hi = lo + step;
    while hi <= 1.0 + 1e-12 {
        if adams_parasitic_modulus(q, hi, m) >= 1.0 {
            for _ in 0..50 {
                let mid = 0.5 * (lo + hi);
                if adams_parasitic_modulus(q, mid, m) >= 1.0 {
                    hi = mid;
                } else {
                    lo = mid;
                }
            }
            return Some(0.5 * (lo + hi));
        }
        lo = hi;
        hi += step;
    }
    None
}

/// the fraction rho of the way from the predicted value to the exact solution of the backward differentiation formula
/// that `newton_steps` steps of Newton's method cover when the matrix was made for cjJ = cj / ratio
pub fn bdf_rho(k: usize, z: Cdd, ratio: f64, newton_steps: u32) -> Cdd {
    let hk = Cdd::real(harmonic(k));
    let r = Dd::from_f64(ratio);
    let scale = Dd::from_f64(2.0) / (Dd::ONE + r);
    let rho1 = ((hk - z) / (hk.scale(Dd::ONE / r) - z)).scale(scale);
    Cdd::ONE - powi(Cdd::ONE - rho1, newton_steps)
}

/// The relative error of one step, eps = kappa exp(-z) - 1, of the backward differentiation formula of the order k
/// with `newton_steps` steps of Newton's method per step and a matrix made for cjJ = cj / ratio (formula (4) of the
/// module text; ratio = 1 is formula (3), for any number of Newton steps).
pub fn bdf_eps(k: usize, z: Cdd, ratio: f64, newton_steps: u32) -> Cdd {
    assert!((1..=5).contains(&k) && newton_steps >= 1);
    let hk = Cdd::real(harmonic(k));
    let rho = bdf_rho(k, z, ratio, newton_steps);
    let c = (Cdd::ONE - rho) / rho * (hk - z);
    // Newton's method for g(u) = B(u) - z + c u^(k+1), from u = z
    let mut u = z;
    for _ in 0..60 {
        let (mut bsum, mut dsum) = (Cdd::ZERO, Cdd::ZERO);
        let mut up = Cdd::ONE; // u^(j-1)
        for j in 1..=k {
            dsum = dsum + up;
            up = up * u;
            bsum = bsum + up.scale(Dd::ONE / Dd::from_f64(j as f64));
        }
        // up = u^k
        let g = bsum - z + c * up * u;
        let dg = dsum + c.scale(Dd::from_f64((k + 1) as f64)) * up;
        let step = g / dg;
        u = u - step;
        if step.abs_f64() < 1e-33 * u.abs_f64().max(1e-300) {
            break;
        }
    }
    (-z).exp() / (Cdd::ONE - u) - Cdd::ONE
}

/// All roots kappa of the recurrence (4) (f64 precision): the k+1 factors by which the solutions of the recurrence are
/// multiplied in one step (k roots if rho = 1), the largest modulus first.  The first is the principal root or, beyond
/// the stability bound, the alternating root.
pub fn bdf_roots(k: usize, z: C64, ratio: f64, newton_steps: u32) -> Vec<C64> {
    let zz = Cdd::from_c64(z);
    let hk = harmonic(k).to_f64();
    let rho = bdf_rho(k, zz, ratio, newton_steps).to_c64();
    let one = C64 { re: 1.0, im: 0.0 };
    let c = (one - rho) / rho * (C64 { re: hk, im: 0.0 } - z);
    // the polynomial in u, coefficients from the constant term upwards
    let mut co: Vec<C64> = vec![C64 { re: -z.re, im: -z.im }];
    for j in 1..=k {
        co.push(C64 { re: 1.0 / j as f64, im: 0.0 });
    }
    if c.abs() > 1e-14 {
        co.push(c);
    }
    let roots = polynomial_roots(&co);
    let mut kappa: Vec<C64> = roots.iter().map(|u| one / (one - *u)).collect();
    kappa.sort_by(|a, b| b.abs().partial_cmp(&a.abs()).unwrap());
    kappa
}

/// The largest modulus of the roots of the recurrence (4) other than the principal root: the factor per step of the
/// slowest-decaying (or fastest-growing) error that is not the solution itself.  The scheme is unstable if the number
/// exceeds 1.
pub fn bdf_parasitic_modulus(k: usize, z: C64, ratio: f64, newton_steps: u32) -> f64 {
    let roots = bdf_roots(k, z, ratio, newton_steps);
    let principal = z.exp();
    let ip = (0..roots.len()).min_by(|&a, &b| (roots[a] - principal).abs().partial_cmp(&(roots[b] - principal).abs()).unwrap()).expect("roots");
    roots.iter().enumerate().filter(|(i, _)| *i != ip).map(|(_, r)| r.abs()).fold(0.0f64, f64::max)
}

/// The interval of the ratio r = cj/cjJ, inside the interval (0.6, 5/3) that IDA tolerates before it computes its
/// matrix anew, in which the recurrence (4) of the order k with ONE Newton step per step is stable for z = 0 (all
/// roots other than the principal one inside the unit circle).  Below the lower end a pair of complex roots leaves
/// the unit circle, above the upper end the alternating root does; an end that equals 0.6 or 5/3 means that the
/// scheme is stable up to the limit of IDA.  The ends are found by bisection from r = 1, where the scheme is the
/// exactly solved formula.
pub fn bdf_stability_window(k: usize) -> (f64, f64) {
    let unstable = |r: f64| bdf_parasitic_modulus(k, C64::ZERO, r, 1) >= 1.0;
    let end = |limit: f64| -> f64 {
        if !unstable(limit) {
            return limit;
        }
        let (mut stable, mut bad) = (1.0f64, limit);
        for _ in 0..60 {
            let mid = 0.5 * (stable + bad);
            if unstable(mid) {
                bad = mid;
            } else {
                stable = mid;
            }
        }
        0.5 * (stable + bad)
    };
    (end(0.6), end(5.0 / 3.0))
}

/// The alternating root of the recurrence (4) for z = 0 and one Newton step: the root kappa on the negative real axis
/// (an error that changes its sign from step to step), or None if the recurrence has none (ratio = 1).
pub fn bdf_alternating_root(k: usize, ratio: f64) -> Option<f64> {
    let roots = bdf_roots(k, C64::ZERO, ratio, 1);
    roots.iter().filter(|r| r.re < 0.0 && r.im.abs() < 1e-9 * r.re.abs().max(1e-300)).map(|r| r.re).fold(None, |acc: Option<f64>, x| match acc {
        None => Some(x),
        Some(a) => Some(if x.abs() > a.abs() { x } else { a }),
    })
}

/// The ratio r = cj/cjJ at which the alternating root of the order k reaches the modulus 1 (bisection between 1.01
/// and 5/3, the largest ratio that IDA tolerates before it recomputes its matrix).
pub fn bdf_stability_bound(k: usize) -> f64 {
    let modulus = |r: f64| bdf_alternating_root(k, r).map(|x| x.abs()).unwrap_or(0.0);
    let (mut lo, mut hi) = (1.01f64, 5.0 / 3.0);
    if modulus(hi) < 1.0 {
        return f64::INFINITY;
    }
    for _ in 0..60 {
        let mid = 0.5 * (lo + hi);
        if modulus(mid) < 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// The same ratio in closed form.  For z = 0 and one Newton step the recurrence (4) is
///     B(u) = - ((1 - rho)/rho) H_k u^(k+1) ,     rho = 2 r/(1 + r) ,     B(u) = u + u^2/2 + ... + u^k/k .
/// The alternating root kappa = -1 is u = 1 - 1/kappa = 2.  Setting u = 2 gives (rho - 1)/rho = B(2)/(H_k 2^(k+1)); call
/// this number c.  Then rho = 1/(1 - c) and r = rho/(2 - rho) = 1/(1 - 2 c):
///     k = 1: c = 1/2, no bound;   k = 2: c = 1/3, r = 3;   k = 3: c = 5/22, r = 11/6;
///     k = 4: c = 4/25, r = 25/17 = 1.4706;   k = 5: c = 16/137, r = 137/105 = 1.3048.
/// IDA keeps its matrix up to r = 5/3 = 1.6667: for the orders 4 and 5 the bound lies inside this interval.
/// Returns infinity where there is no bound (k = 1).
pub fn bdf_alternating_bound_exact(k: usize) -> f64 {
    let mut b2 = 0.0f64;
    for j in 1..=k {
        b2 += 2f64.powi(j as i32) / j as f64;
    }
    let c = b2 / (harmonic(k).to_f64() * 2f64.powi(k as i32 + 1));
    if 1.0 - 2.0 * c <= 1e-14 {
        f64::INFINITY
    } else {
        1.0 / (1.0 - 2.0 * c)
    }
}

/// The factor by which the Adams-Bashforth predictor of the order q multiplies an error of the stored derivatives
/// that changes its sign from step to step: the backward difference of the order n of such a sequence is 2^n times
/// the sequence, so the predictor sum_(n=0..q-1) c_AB_n nabla^n multiplies it by sum_(n=0..q-1) c_AB_n 2^n
/// (1, 2, 11/3, 20/3, 551/45, ... for q = 1, 2, 3, 4, 5).  It about doubles with every order; this is why the limit of
/// stability of the steps with one iteration, whose stored derivative is that of the predicted value, about halves
/// with every order (`adams_stability_limit`).
pub fn adams_predictor_gain(q: usize) -> f64 {
    let g = c_ab(q);
    (0..q).map(|n| g[n].to_f64() * 2f64.powi(n as i32)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use t2core::c64::c;

    /// the closed form of the bound of the alternating root against the bisection, and its fractions
    #[test]
    fn bound_of_the_alternating_root() {
        assert!((bdf_alternating_bound_exact(5) - 137.0 / 105.0).abs() < 1e-14);
        assert!((bdf_alternating_bound_exact(4) - 25.0 / 17.0).abs() < 1e-14);
        assert!((bdf_alternating_bound_exact(3) - 11.0 / 6.0).abs() < 1e-14);
        assert!((bdf_alternating_bound_exact(2) - 3.0).abs() < 1e-13);
        assert!(bdf_alternating_bound_exact(1).is_infinite());
        for k in [4usize, 5] {
            assert!((bdf_stability_bound(k) - bdf_alternating_bound_exact(k)).abs() < 1e-9, "order {}", k);
        }
        // the gains of the predictor: 1, 2, 11/3, 20/3, 551/45
        let want = [1.0, 2.0, 11.0 / 3.0, 20.0 / 3.0, 551.0 / 45.0];
        for q in 1..=5usize {
            assert!((adams_predictor_gain(q) - want[q - 1]).abs() < 1e-13, "q = {}: {}", q, adams_predictor_gain(q));
        }
    }

    fn zc(x: f64, y: f64) -> Cdd {
        Cdd::from_c64(c(x, y))
    }
    fn rel(a: Cdd, b: Cdd) -> f64 {
        (a - b).abs_f64() / b.abs_f64()
    }

    /// the coefficients: c_AM_j = -1/2, -1/12, -1/24, -19/720, -3/160, -863/60480, -275/24192;
    /// c_AB_j = 1/2, 5/12, 3/8, 251/720; c_AM_j = c_AB_j - c_AB_(j-1); the trapezoidal rule and Adams-Bashforth 2
    #[test]
    fn coefficients() {
        let gs = c_am(7);
        let want = [1.0, -0.5, -1.0 / 12.0, -1.0 / 24.0, -19.0 / 720.0, -3.0 / 160.0, -863.0 / 60480.0, -275.0 / 24192.0];
        for j in 0..=7 {
            assert!((gs[j].to_f64() - want[j]).abs() < 1e-16, "c_AM_{} = {}", j, gs[j].to_f64());
        }
        let g = c_ab(4);
        let want = [1.0, 0.5, 5.0 / 12.0, 3.0 / 8.0, 251.0 / 720.0];
        for j in 0..=4 {
            assert!((g[j].to_f64() - want[j]).abs() < 1e-16);
        }
        for j in 1..=4 {
            assert!(((g[j] - g[j - 1]) - gs[j]).to_f64().abs() < 1e-30);
        }
        let bam = b_am(2);
        assert!((bam[0].to_f64() - 0.5).abs() < 1e-30 && (bam[1].to_f64() - 0.5).abs() < 1e-30);
        let b = b_ab(2);
        assert!((b[0].to_f64() - 1.5).abs() < 1e-30 && (b[1].to_f64() + 0.5).abs() < 1e-30);
        // the weights of a formula of any order add up to 1
        for q in 1..=12 {
            let s = b_am(q).iter().fold(Dd::ZERO, |a, x| a + *x);
            let t = b_ab(q).iter().fold(Dd::ZERO, |a, x| a + *x);
            assert!((s - Dd::ONE).to_f64().abs() < 1e-24 && (t - Dd::ONE).to_f64().abs() < 1e-24, "order {}", q);
        }
    }

    /// formulas with a known factor: backward Euler kappa = 1/(1 - z) (it is the Adams-Moulton formula of the order 1
    /// and the backward differentiation formula of the order 1), the trapezoidal rule kappa = (1 + z/2)/(1 - z/2), and
    /// the backward differentiation formula of the order 2, kappa = (2 + sqrt(1 + 2 z))/(3 - 2 z)
    #[test]
    fn known_factors() {
        for &(x, y) in &[(0.0, -0.05), (0.0, 0.3), (-0.02, 0.11)] {
            let z = zc(x, y);
            let euler = (-z).exp() / (Cdd::ONE - z) - Cdd::ONE;
            assert!(rel(adams_eps(1, z, None), euler) < 1e-26);
            assert!(rel(bdf_eps(1, z, 1.0, 1), euler) < 1e-26);
            let half = z.scale(Dd::from_f64(0.5));
            let trap = (-z).exp() * (Cdd::ONE + half) / (Cdd::ONE - half) - Cdd::ONE;
            assert!(rel(adams_eps(2, z, None), trap) < 1e-25);
            // BDF2 against the closed form evaluated in f64, whose difference kappa exp(-z) - 1 has an absolute
            // rounding error of a few times 1e-16
            let zf = c(x, y);
            let root = (c(1.0, 0.0) + zf.scale(2.0)).sqrt();
            let kappa = (c(2.0, 0.0) + root) / (c(3.0, 0.0) - zf.scale(2.0));
            let want = kappa * (c(0.0, 0.0) - zf).exp() - c(1.0, 0.0);
            let got = bdf_eps(2, z, 1.0, 1).to_c64();
            assert!((got - want).abs() < 2e-15, "BDF2 at {:?}: {:?} against {:?}", zf, got, want);
        }
    }

    /// the leading terms: eps = -c_AM_q z^(q+1) for Adams-Moulton, z^(k+1)/(k+1) for backward differentiation,
    /// (1/(k+1) + ((rho-1)/rho) H_k) z^(k+1) with an old matrix; the relative deviation from them is of the order |z|.
    /// (|z| = 0.01: for the order 12 the error of a step is then 7e-29, still 1e3 times the precision of the numbers.)
    #[test]
    fn leading_terms() {
        let z = zc(0.0, -1e-2);
        let gs = c_am(12);
        for q in 1..=12 {
            let lead = (-Cdd::real(gs[q])) * powi(z, q as u32 + 1);
            assert!(rel(adams_eps(q, z, None), lead) < 0.1, "Adams-Moulton {}: {:e}", q, rel(adams_eps(q, z, None), lead));
            assert!(rel(adams_eps(q, z, Some(4)), lead) < 0.1, "Adams-Moulton {} with 4 iterations: {:e}", q, rel(adams_eps(q, z, Some(4)), lead));
        }
        let z = zc(0.0, -1e-3);
        for k in 1..=5 {
            let lead = powi(z, k as u32 + 1).scale(Dd::ONE / Dd::from_f64((k + 1) as f64));
            assert!(rel(bdf_eps(k, z, 1.0, 1), lead) < 5e-3, "BDF {}", k);
            let r = 1.164;
            let rho = 2.0 * r / (1.0 + r);
            let coefficient = 1.0 / (k + 1) as f64 + (rho - 1.0) / rho * harmonic(k).to_f64();
            let lead = powi(z, k as u32 + 1).scale(Dd::from_f64(coefficient));
            assert!(rel(bdf_eps(k, z, r, 1), lead) < 5e-3, "BDF {} with an old matrix", k);
        }
    }

    /// many fixed-point iterations give the exactly solved formula; two Newton steps with an old matrix are close to
    /// the exactly solved formula ((1 - rho_1)^2 is small); a current matrix gives it for any number of steps
    #[test]
    fn limits() {
        let z = zc(0.0, -0.08);
        for q in [2usize, 5, 6, 7] {
            assert!(rel(adams_eps(q, z, Some(40)), adams_eps(q, z, None)) < 1e-12, "order {}", q);
        }
        for k in 1..=5 {
            assert!(rel(bdf_eps(k, z, 1.0, 3), bdf_eps(k, z, 1.0, 1)) < 1e-20);
            let exact = bdf_eps(k, z, 1.0, 1);
            let two = bdf_eps(k, z, 1.2, 2);
            let one = bdf_eps(k, z, 1.2, 1);
            assert!(rel(two, exact) < 0.2 * rel(one, exact), "order {}", k);
        }
    }

    /// the roots of the recurrence of CVODE: the principal root is the one that `adams_eps` computes; with one
    /// iteration the formulas of the orders 6 and 7 have another root outside the unit circle at the largest |z| of the
    /// integrations of GS-13 (0.1115 and 0.0446), with two iterations all other roots are inside; the two roots of the
    /// order 1 with one iteration against their quadratic equation
    #[test]
    fn roots_of_the_adams_recurrence() {
        for &(q, m, za) in &[(2usize, 1u32, 0.05f64), (5, 2, 0.0463), (6, 1, 0.03), (7, 2, 0.0446)] {
            let z = c(0.0, -za);
            let roots = adams_roots(q, z, m);
            assert_eq!(roots.len(), q + 1);
            let eps = adams_eps(q, Cdd::from_c64(z), Some(m)).to_c64();
            let principal = z.exp() * (c(1.0, 0.0) + eps);
            let nearest = roots.iter().map(|r| (*r - principal).abs()).fold(f64::INFINITY, f64::min);
            assert!(nearest < 1e-10, "order {} with {} iterations: nearest root at a distance {:e}", q, m, nearest);
        }
        assert!(adams_parasitic_modulus(6, 0.1115, 1) > 1.0 && adams_parasitic_modulus(6, 0.1115, 2) < 1.0);
        assert!(adams_parasitic_modulus(7, 0.0446, 1) > 1.0 && adams_parasitic_modulus(7, 0.0446, 2) < 1.0);
        assert!(adams_parasitic_modulus(5, 0.0463, 2) < 1.0);
        // the order 1 with one iteration: y_n = y_(n-1) + G_n, G_n = z (y_(n-1) + G_(n-1)), whose two factors are the
        // roots of kappa^2 - (1 + 2 z) kappa + z = 0
        let z = c(0.0, -0.2);
        let roots = adams_roots(1, z, 1);
        assert_eq!(roots.len(), 2);
        for r in &roots {
            let residual = *r * *r - (c(1.0, 0.0) + z.scale(2.0)) * *r + z;
            assert!(residual.abs() < 1e-13, "{:?}", roots);
        }
        // the limits: the higher the order, the smaller the interval of stability with one iteration
        let l5 = adams_stability_limit(5, 1).unwrap();
        let l6 = adams_stability_limit(6, 1).unwrap();
        let l7 = adams_stability_limit(7, 1).unwrap();
        assert!(l5 > l6 && l6 > l7 && l7 > 0.0, "{} {} {}", l5, l6, l7);
        assert!(l6 < 0.1115 && l7 < 0.0446);
    }

    /// the alternating root of the order 5: modulus 0.65 at the ratio 1.164, above 1 (1.017) at 1.313; the bound is 1.3048
    #[test]
    fn alternating_root() {
        let a = bdf_alternating_root(5, 1.164068524711).unwrap();
        assert!((a + 0.6547).abs() < 2e-3, "{}", a);
        let b = bdf_alternating_root(5, 1.312865875279).unwrap();
        assert!((b + 1.0171).abs() < 1e-3, "{}", b);
        let bound = bdf_stability_bound(5);
        assert!((bound - 1.3048).abs() < 1e-3, "{}", bound);
        // the principal root is among the roots, with modulus 1 at z = 0
        let roots = bdf_roots(5, C64::ZERO, 1.2, 1);
        assert!(roots.iter().any(|r| (r.re - 1.0).abs() < 1e-9 && r.im.abs() < 1e-9));
        // the window of stability of the order 5 with one Newton step: the upper end is the bound of the alternating
        // root, 137/105 exactly (the alternating root is -1 where u = 2: B(2) + ((1-rho)/rho) H_5 2^6 = 0 gives
        // rho = 137/121, and rho = 2r/(1+r) gives r = 137/105); below the lower end a complex pair is outside
        let (lo, hi) = bdf_stability_window(5);
        assert!((hi - 137.0 / 105.0).abs() < 1e-9, "{}", hi);
        assert!(lo > 0.7 && lo < 0.9, "{}", lo);
        assert!(bdf_parasitic_modulus(5, C64::ZERO, 0.6089, 1) > 1.1 && bdf_parasitic_modulus(5, C64::ZERO, 0.6089, 2) < 0.9);
        assert!(bdf_parasitic_modulus(5, C64::ZERO, 1.0, 1) < 0.72);
        // the lower orders tolerate more: the window of the order 1 is all that IDA allows
        let (lo1, hi1) = bdf_stability_window(1);
        assert!((lo1 - 0.6).abs() < 1e-12 && (hi1 - 5.0 / 3.0).abs() < 1e-12, "{} {}", lo1, hi1);
    }
}
