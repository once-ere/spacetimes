// Provenance: this module is taken from the author's NCSS project (github.com/once-ere/NCSS,
// Revision_1/code/ncss-rs/src/roots.rs), where it is validated by that project's test suite.
// Changes for PROJECT spacetimes, if any, are marked "[spacetimes]".
//
//! Scalar root finding: Brent's method (bracketing, superlinear) and plain
//! bisection.  Both require a sign change of f on [a, b].

/// Error returned when the initial interval does not bracket a root.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoBracket {
    pub fa: f64,
    pub fb: f64,
}

/// Brent's method (Brent 1973, "Algorithms for Minimization Without
/// Derivatives", ch. 4).  Returns x with |f(x)| ~ 0 to absolute tolerance
/// `xtol` in x, or an error if f(a), f(b) have the same sign.
pub fn brent<F: FnMut(f64) -> f64>(mut f: F, mut a: f64, mut b: f64, xtol: f64, maxit: usize) -> Result<f64, NoBracket> {
    let mut fa = f(a);
    let mut fb = f(b);
    if fa == 0.0 {
        return Ok(a);
    }
    if fb == 0.0 {
        return Ok(b);
    }
    if fa.signum() == fb.signum() {
        return Err(NoBracket { fa, fb });
    }
    let mut c = a;
    let mut fc = fa;
    let mut d = b - a;
    let mut e = d;
    for _ in 0..maxit {
        if fb.signum() == fc.signum() {
            c = a;
            fc = fa;
            d = b - a;
            e = d;
        }
        if fc.abs() < fb.abs() {
            a = b;
            b = c;
            c = a;
            fa = fb;
            fb = fc;
            fc = fa;
        }
        let tol1 = 2.0 * f64::EPSILON * b.abs() + 0.5 * xtol;
        let xm = 0.5 * (c - b);
        if xm.abs() <= tol1 || fb == 0.0 {
            return Ok(b);
        }
        if e.abs() >= tol1 && fa.abs() > fb.abs() {
            // attempt inverse quadratic interpolation / secant
            let s = fb / fa;
            let (mut p, mut q);
            if a == c {
                p = 2.0 * xm * s;
                q = 1.0 - s;
            } else {
                let qq = fa / fc;
                let r = fb / fc;
                p = s * (2.0 * xm * qq * (qq - r) - (b - a) * (r - 1.0));
                q = (qq - 1.0) * (r - 1.0) * (s - 1.0);
            }
            if p > 0.0 {
                q = -q;
            }
            p = p.abs();
            let min1 = 3.0 * xm * q - (tol1 * q).abs();
            let min2 = (e * q).abs();
            if 2.0 * p < min1.min(min2) {
                e = d;
                d = p / q;
            } else {
                d = xm;
                e = d;
            }
        } else {
            d = xm;
            e = d;
        }
        a = b;
        fa = fb;
        if d.abs() > tol1 {
            b += d;
        } else {
            b += tol1 * xm.signum();
        }
        fb = f(b);
    }
    Ok(b)
}

/// Bisection on [a, b] for a predicate that is `false` at a and `true` at b
/// (monotone).  Returns the transition point to absolute precision `xtol`.
pub fn bisect_predicate<P: FnMut(f64) -> bool>(mut pred: P, mut a: f64, mut b: f64, xtol: f64, maxit: usize) -> f64 {
    for _ in 0..maxit {
        if (b - a).abs() < xtol {
            break;
        }
        let m = 0.5 * (a + b);
        if pred(m) {
            b = m;
        } else {
            a = m;
        }
    }
    0.5 * (a + b)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn brent_finds_cubic_root() {
        let r = brent(|x| x * x * x - 2.0 * x - 5.0, 2.0, 3.0, 1e-14, 100).unwrap();
        assert!((r - 2.094_551_481_542_327).abs() < 1e-12);
    }
}
