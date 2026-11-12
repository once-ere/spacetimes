//! Spherical Bessel functions j_l(x) for the line-of-sight integrals.
//!
//! `sph_bessel_all(l_max, x)` returns j_0(x) .. j_{l_max}(x), computed with Miller's
//! downward recurrence
//!     j_{l-1}(x) = ((2l+1)/x) j_l(x) - j_{l+1}(x),
//! started far above max(l_max, x), where the wanted (minimal) solution is negligible,
//! and normalized with the closed forms of j_0 and j_1.  The downward direction is
//! numerically stable for all l, in contrast to the upward one.
//!
//! `BesselTable` stores j_l and j_l' for a list of multipoles on a uniform grid in x
//! and interpolates with cubic Hermite polynomials (both the function and its
//! derivative are tabulated, so the interpolation error is of fourth order).

/// j_0(x) .. j_{l_max}(x).
pub fn sph_bessel_all(l_max: usize, x: f64) -> Vec<f64> {
    let mut out = vec![0.0; l_max + 1];
    if x.abs() < 1e-8 {
        out[0] = 1.0;
        return out;
    }
    // starting order: beyond the turning point l = x, where j_l decays faster than
    // exponentially with l
    let start = (l_max.max(x as usize)) + 40 + (12.0 * (x.max(l_max as f64)).powf(1.0 / 3.0)) as usize;
    let (mut fp, mut f) = (0.0f64, 1e-250f64); // f_{l+1}, f_l at l = start
    let mut l = start;
    let mut f0 = 0.0;
    let mut f1 = 0.0;
    while l > 0 {
        let fm = (2.0 * l as f64 + 1.0) / x * f - fp; // f_{l-1}
        fp = f;
        f = fm;
        l -= 1;
        if l <= l_max {
            out[l] = f;
        }
        if l == 1 {
            f1 = f;
        }
        if l == 0 {
            f0 = f;
        }
        if f.abs() > 1e250 {
            // rescale everything computed so far
            let s = 1e-250;
            f *= s;
            fp *= s;
            f1 *= s;
            if l <= l_max {
                for v in out[l..].iter_mut() {
                    *v *= s;
                }
            }
        }
    }
    let (s, c) = x.sin_cos();
    let j0 = s / x;
    let j1 = s / (x * x) - c / x;
    let scale = if l_max >= 1 && j1.abs() > j0.abs() { j1 / f1 } else { j0 / f0 };
    for v in out.iter_mut() {
        *v *= scale;
    }
    out
}

/// Tables of j_l(x) and j_l'(x) for a list of multipoles.
pub struct BesselTable {
    pub ls: Vec<usize>,
    pub dx: f64,
    pub nx: usize,
    /// j[i][n] = j_{ls[i]}(n dx)
    j: Vec<Vec<f64>>,
    /// dj[i][n] = j_{ls[i]}'(n dx)
    dj: Vec<Vec<f64>>,
}

impl BesselTable {
    /// Tabulate for the multipoles `ls` (ascending) on 0 <= x <= x_max.
    pub fn new(ls: &[usize], x_max: f64, dx: f64) -> BesselTable {
        let nx = (x_max / dx).ceil() as usize + 2;
        let l_max = *ls.last().unwrap();
        let mut j = vec![vec![0.0; nx]; ls.len()];
        let mut dj = vec![vec![0.0; nx]; ls.len()];
        // split the x range over the available threads
        let nthreads = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).min(8);
        let chunk = (nx + nthreads - 1) / nthreads;
        let results: Vec<Vec<(usize, Vec<f64>, Vec<f64>)>> = std::thread::scope(|sc| {
            let handles: Vec<_> = (0..nthreads)
                .map(|t| {
                    let ls = ls.to_vec();
                    sc.spawn(move || {
                        let mut part = Vec::new();
                        for n in (t * chunk)..((t + 1) * chunk).min(nx) {
                            let x = n as f64 * dx;
                            let all = sph_bessel_all(l_max, x);
                            let mut jv = Vec::with_capacity(ls.len());
                            let mut dv = Vec::with_capacity(ls.len());
                            for &l in &ls {
                                jv.push(all[l]);
                                // j_l' = j_{l-1} - (l+1)/x j_l ;  at x = 0 only j_1' = 1/3 is non-zero
                                let d = if x < 1e-8 {
                                    if l == 1 { 1.0 / 3.0 } else { 0.0 }
                                } else if l == 0 {
                                    -all.get(1).copied().unwrap_or_else(|| (x.sin() / (x * x)) - x.cos() / x)
                                } else {
                                    all[l - 1] - (l as f64 + 1.0) / x * all[l]
                                };
                                dv.push(d);
                            }
                            part.push((n, jv, dv));
                        }
                        part
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for part in results {
            for (n, jv, dv) in part {
                for i in 0..ls.len() {
                    j[i][n] = jv[i];
                    dj[i][n] = dv[i];
                }
            }
        }
        BesselTable { ls: ls.to_vec(), dx, nx, j, dj }
    }

    /// (j_l(x), j_l'(x)) for the multipole with index `il` in `ls`.
    #[inline]
    pub fn eval(&self, il: usize, x: f64) -> (f64, f64) {
        let t = x / self.dx;
        let n = t as usize;
        if n + 1 >= self.nx {
            return (0.0, 0.0);
        }
        let u = t - n as f64;
        let (y0, y1) = (self.j[il][n], self.j[il][n + 1]);
        let (d0, d1) = (self.dj[il][n] * self.dx, self.dj[il][n + 1] * self.dx);
        let u2 = u * u;
        let u3 = u2 * u;
        let val = (2.0 * u3 - 3.0 * u2 + 1.0) * y0 + (u3 - 2.0 * u2 + u) * d0 + (-2.0 * u3 + 3.0 * u2) * y1 + (u3 - u2) * d1;
        // derivative of the Hermite polynomial with respect to x
        let der = ((6.0 * u2 - 6.0 * u) * y0 + (3.0 * u2 - 4.0 * u + 1.0) * d0 + (-6.0 * u2 + 6.0 * u) * y1 + (3.0 * u2 - 2.0 * u) * d1) / self.dx;
        (val, der)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closed_forms_and_sum_rule() {
        for &x in &[0.3, 2.0, 17.5, 250.0, 3000.0] {
            let j = sph_bessel_all(60, x);
            let (s, c) = (x as f64).sin_cos();
            assert!((j[0] - s / x).abs() < 1e-13);
            assert!((j[1] - (s / (x * x) - c / x)).abs() < 1e-13);
            assert!((j[2] - ((3.0 / (x * x) - 1.0) * s / x - 3.0 * c / (x * x))).abs() < 1e-11 * (1.0 + 1.0 / (x * x * x)));
        }
        // sum rule: sum_l (2l+1) j_l(x)^2 = 1
        for &x in &[1.0, 40.0, 700.0] {
            let j = sph_bessel_all(2000, x);
            let s: f64 = j.iter().enumerate().map(|(l, v)| (2.0 * l as f64 + 1.0) * v * v).sum();
            assert!((s - 1.0).abs() < 1e-10, "x = {}: {}", x, s);
        }
    }

    #[test]
    fn table_interpolation_error_bounds() {
        // Cubic Hermite interpolation with exact end-point values and derivatives on a
        // grid of spacing h has the error bounds
        //     |value error|      <= (h^4 / 384)          max |d^4 f / dx^4| ,
        //     |derivative error| <= (sqrt(3) h^3 / 216)  max |d^4 f / dx^4| .
        // For f = j_l the fourth derivative is bounded by 1/x beyond the turning point
        // (amplitude 1/x times the fourth power of a local wavenumber <= 1) and is much
        // smaller below it.  The table must respect these bounds (a factor 1.2 of
        // margin); the reference values are direct evaluations of the recurrence.
        let ls = [2usize, 30, 400, 1500];
        let h: f64 = 0.2;
        let tab = BesselTable::new(&ls, 4000.0, h);
        let mut worst_v: f64 = 0.0;
        let mut worst_d: f64 = 0.0;
        for (il, &l) in ls.iter().enumerate() {
            for &x in &[5.3f64, 77.7, 911.1, 3456.7] {
                let all = sph_bessel_all(l, x);
                let exact = all[l];
                let (v, d) = tab.eval(il, x);
                let bound_v = 1.2 * h.powi(4) / 384.0 / x + 1e-12;
                assert!((v - exact).abs() < bound_v, "l = {}, x = {}: {} vs {} (bound {:e})", l, x, v, exact, bound_v);
                worst_v = worst_v.max((v - exact).abs() * x);
                // exact derivative from the recurrence j_l' = j_{l-1} - (l+1) j_l / x
                let exact_d = all[l - 1] - (l as f64 + 1.0) / x * all[l];
                let bound_d = 1.2 * 3f64.sqrt() * h.powi(3) / 216.0 / x + 1e-12;
                assert!((d - exact_d).abs() < bound_d, "l = {}, x = {}: {} vs {} (bound {:e})", l, x, d, exact_d, bound_d);
                worst_d = worst_d.max((d - exact_d).abs() * x);
            }
        }
        // errors relative to the amplitude 1/x: a few 1e-6 (values), below 1e-4 (derivatives)
        assert!(worst_v < 5e-6 && worst_d < 8e-5, "{:e} {:e}", worst_v, worst_d);
    }

    #[test]
    fn differential_equation() {
        // x^2 j'' + 2 x j' + (x^2 - l(l+1)) j = 0, with j' from the recurrence that the
        // table uses and j'' from a central difference of j'
        for &l in &[2usize, 30, 400, 1500] {
            for &x in &[5.3f64, 77.7, 911.1, 3456.7] {
                let dj = |x: f64| -> f64 {
                    let a = sph_bessel_all(l, x);
                    a[l - 1] - (l as f64 + 1.0) / x * a[l]
                };
                let j = sph_bessel_all(l, x)[l];
                let e = 1e-3;
                let ddj = (dj(x + e) - dj(x - e)) / (2.0 * e);
                let res = x * x * ddj + 2.0 * x * dj(x) + (x * x - (l * (l + 1)) as f64) * j;
                // scale of the individual terms: x^2 |j| <= x
                assert!(res.abs() < 1e-5 * x, "l = {}, x = {}: residual {:e}", l, x, res);
            }
        }
    }
}
