//! Gravity driven by Kohn-Sham matter: the author's Einstein-Lovelock equations for a
//! spacetime with two scale factors, on his cosmological spacetime (for validation) and on
//! the deflating spacetimes.
//!
//! THE GEOMETRY.  One coordinate is the time x4 (proper time; lapse 1).  Of the seven other
//! coordinates, n_a carry the scale factor a(x4) and n_b the scale factor b(x4):
//!
//!     ds^2 = -dx4^2 + a(x4)^2 (sum over the n_a coordinates) + s_b b(x4)^2 (sum over the n_b coordinates),
//!
//! with s_b = +1 if the n_b coordinates are space-like (7+1) and s_b = -1 if they are
//! time-like (4+4).  The rates are H_a = a'/a and H_b = b'/b (a prime is d/dx4).
//! On the deflating spacetimes the n_a = 4 coordinates are x0, x1, x2, x3 (a = 1 in the
//! author's hypotheses) and the n_b = 3 coordinates are x5, x6, x7 with equal rates,
//! b = const exp(-g55 x4), H_b = -g55.  In the author's document KOHN_SHAM_GRAVITY the
//! n_a = 3 coordinates are ordinary space and the n_b = 4 an auxiliary space (and the time
//! is called x0 there).
//!
//! PAIR CURVATURES.  For such a metric the Riemann tensor with two upper and two lower
//! indices is diagonal on pairs of directions, R^{ij}_{ij} = K_ij, with
//!
//!     K_{4 i} = H_i' + H_i^2        (the time and a space direction i),
//!     K_{i j} = H_i H_j             (two different directions i, j other than the time),
//!
//! for space-like and for time-like directions i, j alike (program `matrices`, checks
//! 44d-G4, 71d-G4): the equations below are therefore the same on the 4+4 and on the 7+1
//! spacetime.
//!
//! THE FIELD EQUATIONS (the author's Einstein-Lovelock equations, his document, section 16.3):
//!
//!     sum_{k=1}^{3} alpha_k E^(k)mu_nu - LambdaBar delta^mu_nu = kappa T^mu_nu ,     alpha_1 = 1 ,
//!
//! written here for the three independent components (time; one of the n_a directions; one
//! of the n_b directions).  With the pair curvatures,
//!
//!     E^(k)m_m = (-1)^(k+1) k! 2^(k-1)  sum over the sets of 2k directions other than m
//!                of the sum over the ways to divide the set into k pairs of the product of the k pair curvatures:
//!
//!     E^(1)m_m = + sum over pairs {i, j} of K_ij                              (Einstein tensor),
//!     E^(2)m_m = -4 sum over sets {i, j, k, l} of (K_ij K_kl + K_ik K_jl + K_il K_jk)    (Gauss-Bonnet),
//!     E^(3)m_m = +24 sum over sets of six directions of the 15 products of three pair curvatures.
//!
//! (Signs as in the author's document, where the time component of the Einstein tensor is
//! positive: E^(1)4_4 = sum_{i<j} H_i H_j.)  The sources are
//!
//!     time component:  kappa eps ;      direction i:  -kappa p_i ,
//!
//! with the energy density eps and the pressures p_a, p_b of the Kohn-Sham matter.
//! The time component contains no H' and is a CONSTRAINT on the initial data; the two other
//! components are linear in (H_a', H_b') and are solved for them at every step.
//!
//! THE DEFLATING SPACETIME AS A VACUUM SOLUTION.  For n_a = 4, n_b = 3, H_a = 0, H_b = -g
//! (three equal rates g = g55 = g66 = g77) all three equations hold without matter if
//!
//!     LambdaBar = 3 g^2 ,       alpha_2 = 1/(4 g^2)        (alpha_3 arbitrary: E^(3) vanishes),
//!
//! which is the result of the checks 44d-G10 and 71d-G10 in the author's notation.
//!
//! UNITS: hbar = c = 1; kappa in units in which the energy density is in units of m^8.

use crate::kohn_sham::{root, Contact, Fermi};

/// A linear form c0 + ca H_a' + cb H_b'.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lin {
    pub c0: f64,
    pub ca: f64,
    pub cb: f64,
}

impl Lin {
    pub const ZERO: Lin = Lin { c0: 0.0, ca: 0.0, cb: 0.0 };
    fn add(self, o: Lin) -> Lin {
        Lin { c0: self.c0 + o.c0, ca: self.ca + o.ca, cb: self.cb + o.cb }
    }
    fn scale(self, x: f64) -> Lin {
        Lin { c0: self.c0 * x, ca: self.ca * x, cb: self.cb * x }
    }
}

/// The couplings of the field equations.
#[derive(Clone, Copy, Debug)]
pub struct Lovelock {
    /// alpha_1 (= 1), alpha_2, alpha_3
    pub alpha: [f64; 3],
    pub lambda_bar: f64,
    pub kappa: f64,
}

/// The numbers of directions that carry the scale factors a and b.
#[derive(Clone, Copy, Debug)]
pub struct Geometry {
    pub n_a: usize,
    pub n_b: usize,
}

/// The sum over all ways to divide the directions `set` into pairs of the product of the
/// pair curvatures.  `kind[i]`: 0 = time, 1 = a-direction, 2 = b-direction.
fn matchings(set: &[usize], kind: &[u8], h: &[f64; 2]) -> Lin {
    if set.is_empty() {
        return Lin { c0: 1.0, ca: 0.0, cb: 0.0 };
    }
    let first = set[0];
    let mut total = Lin::ZERO;
    for idx in 1..set.len() {
        let partner = set[idx];
        let rest: Vec<usize> = set.iter().enumerate().filter(|(i, _)| *i != 0 && *i != idx).map(|(_, &x)| x).collect();
        let sub = matchings(&rest, kind, h);
        let (k1, k2) = (kind[first], kind[partner]);
        // pair curvature of (first, partner)
        let pair = if k1 == 0 || k2 == 0 {
            let s = if k1 == 0 { k2 } else { k1 };
            let hh = h[(s - 1) as usize];
            if s == 1 {
                Lin { c0: hh * hh, ca: 1.0, cb: 0.0 }
            } else {
                Lin { c0: hh * hh, ca: 0.0, cb: 1.0 }
            }
        } else {
            Lin { c0: h[(k1 - 1) as usize] * h[(k2 - 1) as usize], ca: 0.0, cb: 0.0 }
        };
        // the time appears in at most one pair, so one of the two factors is a pure number
        let prod = if pair.ca == 0.0 && pair.cb == 0.0 { sub.scale(pair.c0) } else { pair.scale(sub.c0) };
        debug_assert!(pair.ca == 0.0 && pair.cb == 0.0 || (sub.ca == 0.0 && sub.cb == 0.0));
        total = total.add(prod);
    }
    total
}

/// All subsets of `items` with `k` elements.
fn subsets(items: &[usize], k: usize) -> Vec<Vec<usize>> {
    if k == 0 {
        return vec![vec![]];
    }
    if items.len() < k {
        return vec![];
    }
    let mut out = Vec::new();
    for (i, &x) in items.iter().enumerate() {
        for mut rest in subsets(&items[i + 1..], k - 1) {
            let mut v = vec![x];
            v.append(&mut rest);
            out.push(v);
        }
    }
    out
}

/// The left-hand sides sum_k alpha_k E^(k)m_m - LambdaBar for m = time, an a-direction, a
/// b-direction, as linear forms in (H_a', H_b').
pub fn field_equations(geo: &Geometry, love: &Lovelock, h: &[f64; 2]) -> [Lin; 3] {
    // directions: 0 = time, then n_a a-directions, then n_b b-directions
    let mut kind: Vec<u8> = vec![0];
    kind.extend(std::iter::repeat(1u8).take(geo.n_a));
    kind.extend(std::iter::repeat(2u8).take(geo.n_b));
    let n = kind.len();
    let coeff = [1.0, -4.0, 24.0];
    let mut out = [Lin::ZERO; 3];
    for (which, m) in [0usize, 1, 1 + geo.n_a].iter().enumerate() {
        if (which == 1 && geo.n_a == 0) || (which == 2 && geo.n_b == 0) {
            continue;
        }
        let others: Vec<usize> = (0..n).filter(|i| i != m).collect();
        let mut e = Lin { c0: -love.lambda_bar, ca: 0.0, cb: 0.0 };
        for k in 1..=3 {
            if love.alpha[k - 1] == 0.0 {
                continue;
            }
            let mut s = Lin::ZERO;
            for set in subsets(&others, 2 * k) {
                s = s.add(matchings(&set, &kind, h));
            }
            e = e.add(s.scale(love.alpha[k - 1] * coeff[k - 1]));
        }
        out[which] = e;
    }
    out
}

/// The matter: Kohn-Sham matter of fixed comoving occupation.
pub struct Matter<'a> {
    /// the Fermi sea: its "flat" group of `ds` directions and its group of three directions
    pub fermi: &'a Fermi,
    /// true if the flat group of the Fermi sea carries the scale factor a (and the group of three
    /// the scale factor b); false if the flat group carries b and the group of three a
    pub flat_is_a: bool,
    pub c: Contact,
    /// Fermi momentum at a = b = 1
    pub kf: f64,
    /// true: the particles have no momentum along the group of three directions (fields that do
    /// not depend on them); `fermi` is then an isotropic Fermi sea in the flat directions, and
    /// the densities are divided by b^3 (if flat_is_a) or a^3
    pub zero_mode: bool,
}

/// Energy density, pressures along the a- and b-directions, and the mass, at the scale factors (a, b).
#[derive(Clone, Copy, Debug)]
pub struct Source {
    pub eps: f64,
    pub p_a: f64,
    pub p_b: f64,
    pub mass: f64,
    pub rho: f64,
}

impl<'a> Matter<'a> {
    pub fn source(&self, a: f64, b: f64) -> Source {
        if !(a > 0.0 && b > 0.0 && a.is_finite() && b.is_finite()) {
            // outside the domain (a trial stage of the integrator beyond a singularity): no source
            return Source { eps: f64::NAN, p_a: f64::NAN, p_b: f64::NAN, mass: f64::NAN, rho: f64::NAN };
        }
        let (s_flat, s3) = if self.flat_is_a { (1.0 / a, 1.0 / b) } else { (1.0 / b, 1.0 / a) };
        // the moments as functions of the mass
        let mo = |mass: f64| {
            let mut m = self.fermi.moments_scaled(self.kf, mass, s_flat, &[s3; 3]);
            if self.zero_mode {
                // no momenta along the three directions: the density per proper volume carries 1/(scale factor)^3
                let f = s3 * s3 * s3;
                m.rho *= f;
                m.rho_s *= f;
                m.e_kin *= f;
                m.p_flat *= f;
                m.p_defl = [0.0; 3];
            }
            m
        };
        let c = &self.c;
        let mass = if c.gs == 0.0 {
            c.m
        } else if c.gs < 0.0 {
            root(|x| x - c.m - c.gs * mo(x).rho_s, 0.0, c.m)
        } else {
            let rho = mo(c.m).rho;
            root(|x| x - c.m - c.gs * mo(x).rho_s, c.m, c.m + c.gs * rho * 1.0000001 + 1e-300)
        };
        let m = mo(mass);
        let scalar = if c.gs == 0.0 { 0.0 } else { (mass - c.m).powi(2) / (2.0 * c.gs) };
        let vector = 0.5 * c.gv * m.rho * m.rho;
        let eps = m.e_kin + vector - scalar;
        let pi = vector + scalar;
        let (p_flat, p3) = (m.p_flat + pi, m.p_defl[0] + pi);
        let (p_a, p_b) = if self.flat_is_a { (p_flat, p3) } else { (p3, p_flat) };
        Source { eps, p_a, p_b, mass, rho: m.rho }
    }
}

/// The state of the evolution.
#[derive(Clone, Copy, Debug)]
pub struct State {
    pub x4: f64,
    pub a: f64,
    pub b: f64,
    pub h_a: f64,
    pub h_b: f64,
}

/// The evolution problem.
pub struct Evolution<'a> {
    pub geo: Geometry,
    pub love: Lovelock,
    pub matter: Option<Matter<'a>>,
}

impl<'a> Evolution<'a> {
    pub fn source(&self, a: f64, b: f64) -> Source {
        match &self.matter {
            Some(m) => m.source(a, b),
            None => Source { eps: 0.0, p_a: 0.0, p_b: 0.0, mass: 0.0, rho: 0.0 },
        }
    }

    /// The residual of the constraint, sum_k alpha_k E^(k)4_4 - LambdaBar - kappa eps, and the
    /// scale against which it is to be judged (the sum of the absolute values of its terms).
    pub fn constraint(&self, s: &State) -> (f64, f64) {
        let h = [s.h_a, s.h_b];
        let src = self.source(s.a, s.b);
        let e0 = field_equations(&self.geo, &self.love, &h)[0].c0;
        // scale: Einstein term alone plus the other terms
        let ein = field_equations(&self.geo, &Lovelock { alpha: [1.0, 0.0, 0.0], lambda_bar: 0.0, kappa: 0.0 }, &h)[0].c0;
        let scale = ein.abs() + (e0 - ein + self.love.lambda_bar).abs() + self.love.lambda_bar.abs() + (self.love.kappa * src.eps).abs();
        (e0 - self.love.kappa * src.eps, scale)
    }

    /// (H_a', H_b') from the two space components; None if the two equations are not
    /// independent (determinant of the linear system below 1e-12 of its scale).
    pub fn accelerations(&self, s: &State) -> Option<(f64, f64)> {
        let h = [s.h_a, s.h_b];
        let src = self.source(s.a, s.b);
        let fe = field_equations(&self.geo, &self.love, &h);
        // fe[1].c0 + ca Ha' + cb Hb' = -kappa p_a ;   fe[2] likewise with p_b
        let (r1, r2) = (-self.love.kappa * src.p_a - fe[1].c0, -self.love.kappa * src.p_b - fe[2].c0);
        if !(src.eps.is_finite() && fe[1].c0.is_finite() && fe[2].c0.is_finite()) {
            return None;
        }
        let det = fe[1].ca * fe[2].cb - fe[1].cb * fe[2].ca;
        let scale = (fe[1].ca.abs() + fe[1].cb.abs()) * (fe[2].ca.abs() + fe[2].cb.abs());
        if det.abs() < 1e-12 * scale.max(1e-300) {
            return None;
        }
        Some(((r1 * fe[2].cb - r2 * fe[1].cb) / det, (fe[1].ca * r2 - fe[2].ca * r1) / det))
    }

    fn rhs(&self, s: &State) -> Option<[f64; 4]> {
        let (da, db) = self.accelerations(s)?;
        Some([s.a * s.h_a, s.b * s.h_b, da, db])
    }

    /// One classical Runge-Kutta step.  None if the equations degenerate on the way.
    pub fn step(&self, s: &State, dt: f64) -> Option<State> {
        let add = |s: &State, k: &[f64; 4], f: f64| State { x4: s.x4 + f, a: s.a + f * k[0], b: s.b + f * k[1], h_a: s.h_a + f * k[2], h_b: s.h_b + f * k[3] };
        let k1 = self.rhs(s)?;
        let k2 = self.rhs(&add(s, &k1, 0.5 * dt))?;
        let k3 = self.rhs(&add(s, &k2, 0.5 * dt))?;
        let k4 = self.rhs(&add(s, &k3, dt))?;
        let mut k = [0.0; 4];
        for i in 0..4 {
            k[i] = (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]) / 6.0;
        }
        Some(add(s, &k, dt))
    }

    /// The value of H_b that satisfies the constraint for given a, b, H_a: the root nearest to
    /// `guess` found by a scan of the interval [lo, hi] for sign changes.
    pub fn solve_constraint_for_hb(&self, a: f64, b: f64, h_a: f64, lo: f64, hi: f64, guess: f64) -> Option<f64> {
        let f = |hb: f64| self.constraint(&State { x4: 0.0, a, b, h_a, h_b: hb }).0;
        scan_root(f, lo, hi, guess)
    }

    /// The value of H_a that satisfies the constraint for given a, b, H_b.
    pub fn solve_constraint_for_ha(&self, a: f64, b: f64, h_b: f64, lo: f64, hi: f64, guess: f64) -> Option<f64> {
        let f = |ha: f64| self.constraint(&State { x4: 0.0, a, b, h_a: ha, h_b }).0;
        scan_root(f, lo, hi, guess)
    }
}

/// The root of f in [lo, hi] nearest to `guess` (scan with 4000 intervals, then refinement).
fn scan_root<F: Fn(f64) -> f64>(f: F, lo: f64, hi: f64, guess: f64) -> Option<f64> {
    let n = 4000;
    let mut best: Option<f64> = None;
    let mut prev = (lo, f(lo));
    for i in 1..=n {
        let x = lo + (hi - lo) * i as f64 / n as f64;
        let fx = f(x);
        if prev.1 == 0.0 || prev.1 * fx < 0.0 {
            let r = if prev.1 == 0.0 { prev.0 } else { root(&f, prev.0, x) };
            if best.map_or(true, |b| (r - guess).abs() < (b - guess).abs()) {
                best = Some(r);
            }
        }
        prev = (x, fx);
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_components_of_the_author_s_document_are_reproduced() {
        // n_a = 3, n_b = 4: the printed polynomials of section 16.3 at arbitrary rates
        let geo = Geometry { n_a: 3, n_b: 4 };
        let (ha, hb) = (0.37_f64, -0.21_f64);
        let h = [ha, hb];
        let only = |k: usize| {
            let mut alpha = [0.0; 3];
            alpha[k] = 1.0;
            field_equations(&geo, &Lovelock { alpha, lambda_bar: 0.0, kappa: 0.0 }, &h)
        };
        let close = |x: f64, y: f64| (x - y).abs() < 1e-12 * (1.0 + y.abs());
        // E^(1)
        let e = only(0);
        assert!(close(e[0].c0, 3.0 * (ha * ha + 4.0 * ha * hb + 2.0 * hb * hb)));
        assert!(close(e[1].ca, 2.0) && close(e[1].cb, 4.0) && close(e[1].c0, 3.0 * ha * ha + 8.0 * ha * hb + 10.0 * hb * hb));
        assert!(close(e[2].ca, 3.0) && close(e[2].cb, 3.0) && close(e[2].c0, 6.0 * ha * ha + 9.0 * ha * hb + 6.0 * hb * hb));
        // E^(2)
        let e = only(1);
        assert!(close(e[0].c0, -12.0 * hb * (4.0 * ha.powi(3) + 18.0 * ha * ha * hb + 12.0 * ha * hb * hb + hb.powi(3))));
        assert!(close(e[1].ca, -32.0 * ha * hb - 48.0 * hb * hb) && close(e[1].cb, -16.0 * ha * ha - 96.0 * ha * hb - 48.0 * hb * hb));
        assert!(close(e[1].c0, -32.0 * ha.powi(3) * hb - 136.0 * ha * ha * hb * hb - 192.0 * ha * hb.powi(3) - 60.0 * hb.powi(4)));
        assert!(close(e[2].ca, -12.0 * ha * ha - 72.0 * ha * hb - 36.0 * hb * hb) && close(e[2].cb, -36.0 * ha * ha - 72.0 * ha * hb - 12.0 * hb * hb));
        assert!(close(e[2].c0, -12.0 * ha.powi(4) - 108.0 * ha.powi(3) * hb - 180.0 * ha * ha * hb * hb - 108.0 * ha * hb.powi(3) - 12.0 * hb.powi(4)));
        // E^(3)
        let e = only(2);
        assert!(close(e[0].c0, 360.0 * ha * ha * hb.powi(3) * (4.0 * ha + 3.0 * hb)));
        assert!(close(e[1].ca, 72.0 * hb * hb * (8.0 * ha * hb + 2.0 * hb * hb)) && close(e[1].cb, 72.0 * hb * hb * (12.0 * ha * ha + 8.0 * ha * hb)));
        assert!(close(e[1].c0, 72.0 * hb * hb * (8.0 * ha.powi(3) * hb + 19.0 * ha * ha * hb * hb + 8.0 * ha * hb.powi(3))));
        assert!(close(e[2].ca, 72.0 * ha * hb * (9.0 * ha * hb + 6.0 * hb * hb)) && close(e[2].cb, 72.0 * ha * hb * (6.0 * ha * ha + 9.0 * ha * hb)));
        assert!(close(e[2].c0, 72.0 * ha * hb * (9.0 * ha.powi(3) * hb + 17.0 * ha * ha * hb * hb + 9.0 * ha * hb.powi(3))));
    }

    #[test]
    fn the_deflating_spacetime_is_a_vacuum_solution_for_the_tuned_couplings() {
        let geo = Geometry { n_a: 4, n_b: 3 };
        for g in [0.01, 0.3, 2.0] {
            let love = Lovelock { alpha: [1.0, 1.0 / (4.0 * g * g), 0.7], lambda_bar: 3.0 * g * g, kappa: 1.0 };
            let fe = field_equations(&geo, &love, &[0.0, -g]);
            for e in &fe {
                assert!(e.c0.abs() < 1e-13 * g * g, "{:?}", e);
            }
            // the linear system for (H_a', H_b') is diagonal there: (-6, 0; 0, 2)
            assert!((fe[1].ca + 6.0).abs() < 1e-12 && fe[1].cb.abs() < 1e-12 && fe[2].ca.abs() < 1e-12 && (fe[2].cb - 2.0).abs() < 1e-12);
        }
    }
}
