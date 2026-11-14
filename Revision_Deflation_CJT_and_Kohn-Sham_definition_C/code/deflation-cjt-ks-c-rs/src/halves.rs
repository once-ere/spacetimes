//! Uniform matter of the 4+4 model in its two forms: the two halves of the field.
//!
//! THE TWO HALVES.  On the 4+4 spacetime the Hermitian matrix W = i C^- gamma^4 (W^2 = 1)
//! commutes with the single-particle Hamiltonian h = sum_j alpha^j k_j + beta m of the
//! fields that do not depend on x5, x6, x7 (the quantized sector).  The field therefore
//! consists of two halves, W = +1 and W = -1, each with NU/2 = 4 states of positive energy
//! for every momentum.  The kinetic and mass terms treat the two halves alike.
//!
//! THE TWO FORMS OF THE SELF-INTERACTION (module `selfint`, checks 44c-S7, 44q-S7, 44-S13).
//! For uniform matter whose density matrix is diagonal in W, with the densities
//!
//!     n = n_+ + n_- ,   n_W = n_+ - n_- ,   n_S = s_+ + s_- ,   n_SW = s_+ - s_-
//!
//! (n_+-: number densities of the halves, s_+-: their scalar densities <psibar psi>), the
//! Hartree-Fock energy of the self-interaction is
//!
//!     E_int = (1/2) [ T_V n^2 + T_S n_S^2 + D_V n_W^2 + D_S n_SW^2 ] ,
//!
//!     covariant form C (the DEFINITION in this folder):   T_V = G_Vc, T_S = G_Sc, D_V = G_V^HF, D_S = G_S^HF ,
//!     quantized-sector form Q (the alternative):          T_V = G_V^HF, T_S = G_S^HF, D_V = G_Vc, D_S = G_Sc ,
//!
//! with the four combinations of the couplings of the Lagrangian
//!
//!     G_S^HF = (15 g_0eff - 8 g_1eff - 8 g_7eff - g_8eff)/16 ,     G_V^HF = (-g_0eff + 22 g_1eff - 6 g_7eff + g_8eff)/16 ,
//!     G_Sc   = (-g_0eff - 2 g_1eff + 2 g_7eff + g_8eff)/16 ,       G_Vc   = -(g_0eff + g_8eff)/16 .
//!
//! ("T" for the couplings of the total densities, "D" for those of the differences.)
//! Equivalently, with a = T + D (two particles of the same half) and b = T - D,
//!
//!     E_int = (1/2) a_V (n_+^2 + n_-^2) + b_V n_+ n_- + (1/2) a_S (s_+^2 + s_-^2) + b_S s_+ s_- :
//!
//! a is the same in both forms, and b changes its sign from one form to the other.
//!
//! THE KOHN-SHAM EQUATIONS OF TWO HALVES.  The particles of the half W = +-1 have the mass
//! and the energy shift
//!
//!     M_+- = m + T_S n_S +- D_S n_SW ,        V_+- = T_V n +- D_V n_W ,
//!
//! and fill a ball of the Fermi momentum k_+- in D space dimensions (D = 4: momenta along
//! x0..x3; D = 3: fields that do not depend on the hidden coordinate x0 either).  With the
//! moments of one half, <f>_+- = (NU/2) Int_{|k| < k_+-} d^Dk/(2 pi)^D f,
//!
//!     n_+- = <1>_+- ,   s_+- = <M_+-/w>_+- ,   w = sqrt(k^2 + M_+-^2) ,
//!     eps = sum_+- <w>_+- + (1/2)(T_V n^2 + D_V n_W^2) - (1/2)(T_S n_S^2 + D_S n_SW^2) ,
//!     Pi  = (1/2)(T_V n^2 + D_V n_W^2) + (1/2)(T_S n_S^2 + D_S n_SW^2)      (interaction pressure) ,
//!     p   = sum_+- <k^2/(D w)>_+- + Pi                                      (pressure along x0..x3 or x1..x3) ,
//!     mu_+- = sqrt(k_+-^2 + M_+-^2) + V_+-                                  (chemical potentials) .
//!
//! The two equations for M_+ and M_- are coupled gap equations.  ALL their solutions with
//! M_+- >= 0 are found here (a one-dimensional search, see `Halves::solutions`), and the
//! state for given Fermi momenta is the solution of lowest energy density, as in module
//! `kohn_sham` ("no-sea approximation": only the particles of the two Fermi seas are summed).
//!
//! THE POLARIZATION.  The self-interaction does not conserve the number of particles of
//! each half separately (a bilinear with an odd number of the indices 5, 6, 7 moves a
//! particle from one half to the other), only their sum.  At a given total density n the
//! state of equilibrium is therefore the one of lowest energy with respect to the
//! polarization pol = (n_+ - n_-)/n, where mu_+ = mu_- if both halves are occupied.  The
//! exchange of the two halves is a symmetry of the model (in both forms), so that
//! 0 <= pol <= 1 suffices.  A state with pol different from 0 breaks that symmetry
//! spontaneously.
//!
//! THE EFFECTIVE POTENTIAL (CJT) OF TWO HALVES is the function of the four mean fields
//!
//!     Omega(M_+, M_-, V_+, V_-; mu) = sum_+- Omega_1(M_+-, mu - V_+-)
//!         - muS^2/(2 T_S) - dS^2/(2 D_S) - vT^2/(2 T_V) - vD^2/(2 D_V) ,
//!     muS = (M_+ + M_-)/2 - m ,  dS = (M_+ - M_-)/2 ,  vT = (V_+ + V_-)/2 ,  vD = (V_+ - V_-)/2 ,
//!
//! with the grand potential Omega_1 of free particles of one half (module `cjt_contact`,
//! with NU/2 states); its four stationarity conditions are the four Kohn-Sham equations
//! above.  With the filled states of negative energy (cutoff Lambda) the vacuum itself
//! has the two masses M_+-, which need not be equal: see `CjtHalves::vacuum`.
//!
//! UNITS: hbar = c = 1.

use crate::cjt_contact::Cjt;
use crate::kohn_sham::{golden_min, root, Contact, PI};
use crate::selfint::SelfTables;

/// The form of the self-interaction on 4+4.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Form {
    /// covariant form with the positive kinetic term: the definition adopted in this folder
    C,
    /// quantized-sector form: the alternative
    Q,
}

impl Form {
    pub fn tag(self) -> &'static str {
        match self {
            Form::C => "C",
            Form::Q => "Q",
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Form::C => "covariant form (the definition)",
            Form::Q => "quantized-sector form (the alternative)",
        }
    }
    /// 0 for C, 1 for Q: the number written in the data files.
    pub fn code(self) -> f64 {
        match self {
            Form::C => 0.0,
            Form::Q => 1.0,
        }
    }
    pub fn both() -> [Form; 2] {
        [Form::C, Form::Q]
    }
}

/// The four Hartree-Fock couplings of the 4+4 model (the same four numbers in both forms).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hf4 {
    /// G_S^HF (called G_0 in the checks of `selfint`)
    pub gs_hf: f64,
    /// G_V^HF (G_1)
    pub gv_hf: f64,
    /// G_Sc (G_S0)
    pub gs_c: f64,
    /// G_Vc (G_V0)
    pub gv_c: f64,
}

impl Hf4 {
    /// From the four effective couplings (g_0eff, g_1eff, g_7eff, g_8eff): the closed forms.
    pub fn from_effective(e: &[f64; 4]) -> Hf4 {
        let (g0, g1, g7, g8) = (e[0], e[1], e[2], e[3]);
        Hf4 { gs_hf: (15.0 * g0 - 8.0 * g1 - 8.0 * g7 - g8) / 16.0, gv_hf: (-g0 + 22.0 * g1 - 6.0 * g7 + g8) / 16.0, gs_c: (-g0 - 2.0 * g1 + 2.0 * g7 + g8) / 16.0, gv_c: -(g0 + g8) / 16.0 }
    }
    /// From the exact coefficient tables of module `selfint` (covariant form: the densities
    /// rho, rho_S, rho_W, rho_SW in this order) for the nine couplings g_0 .. g_8.
    pub fn from_tables(tabs: &SelfTables, g: &[f64; 9]) -> Hf4 {
        let t = &tabs.s44c;
        Hf4 { gv_c: t.hf_coupling(0, g), gs_c: t.hf_coupling(1, g), gv_hf: t.hf_coupling(2, g), gs_hf: t.hf_coupling(3, g) }
    }
    pub fn scaled(&self, f: f64) -> Hf4 {
        Hf4 { gs_hf: f * self.gs_hf, gv_hf: f * self.gv_hf, gs_c: f * self.gs_c, gv_c: f * self.gv_c }
    }
}

/// The couplings of the two halves: E_int = (1/2)[tv n^2 + ts n_S^2 + dv n_W^2 + ds n_SW^2].
#[derive(Clone, Copy, Debug)]
pub struct HalvesCouplings {
    /// the mass m of the Lagrangian
    pub m: f64,
    pub tv: f64,
    pub ts: f64,
    pub dv: f64,
    pub ds: f64,
}

impl HalvesCouplings {
    pub fn new(form: Form, m: f64, hf: &Hf4) -> HalvesCouplings {
        match form {
            Form::C => HalvesCouplings { m, tv: hf.gv_c, ts: hf.gs_c, dv: hf.gv_hf, ds: hf.gs_hf },
            Form::Q => HalvesCouplings { m, tv: hf.gv_hf, ts: hf.gs_hf, dv: hf.gv_c, ds: hf.gs_c },
        }
    }
    /// scalar coupling between two particles of the same half
    pub fn a_s(&self) -> f64 {
        self.ts + self.ds
    }
    /// scalar coupling between particles of different halves
    pub fn b_s(&self) -> f64 {
        self.ts - self.ds
    }
    pub fn a_v(&self) -> f64 {
        self.tv + self.dv
    }
    pub fn b_v(&self) -> f64 {
        self.tv - self.dv
    }
}

/// The moments of one filled Fermi ball.
#[derive(Clone, Copy, Debug, Default)]
pub struct Mom {
    /// number density
    pub n: f64,
    /// scalar density (it has the sign of the mass)
    pub s: f64,
    /// <w>
    pub e: f64,
    /// <k^2/(D w)>
    pub p: f64,
}

/// The moments of a ball of radius `kf` in d = 3 or 4 dimensions for `nu` internal states
/// and the mass `mass`, in closed form:
///
/// d = 4 (E = sqrt(kf^2 + M^2), De = E - M = kf^2/(E + M)):
///     n = nu kf^4/(32 pi^2),  s = nu M De^2 (E + 2 M)/(24 pi^2),
///     e = nu De^2 (3 E^3 + 6 E^2 M + 4 E M^2 + 2 M^3)/(120 pi^2),  p = nu De^3 (3 E^2 + 9 E M + 8 M^2)/(480 pi^2);
/// d = 3 (L = arsinh(kf/M)):
///     n = nu kf^3/(6 pi^2),  s = nu M (kf E - M^2 L)/(4 pi^2),
///     e = nu [kf E (2 kf^2 + M^2) - M^4 L]/(16 pi^2),  p = nu [kf E (2 kf^2 - 3 M^2) + 3 M^4 L]/(48 pi^2),
/// with the power series in (kf/M)^2 (nine terms) where kf < 0.2 M (the closed forms lose digits there).
/// The forms for d = 4 are factored so that no digits are lost at small kf/M.
pub fn half_moments(d: usize, nu: f64, kf: f64, mass: f64) -> Mom {
    if kf <= 0.0 {
        return Mom::default();
    }
    let m = mass.abs();
    let sign = if mass < 0.0 { -1.0 } else { 1.0 };
    let pi2 = PI * PI;
    match d {
        4 => {
            let e = (kf * kf + m * m).sqrt();
            let de = kf * kf / (e + m);
            Mom {
                n: nu * kf.powi(4) / (32.0 * pi2),
                s: sign * nu * m * de * de * (e + 2.0 * m) / (24.0 * pi2),
                e: nu * de * de * (3.0 * e * e * e + 6.0 * e * e * m + 4.0 * e * m * m + 2.0 * m * m * m) / (120.0 * pi2),
                p: nu * de * de * de * (3.0 * e * e + 9.0 * e * m + 8.0 * m * m) / (480.0 * pi2),
            }
        }
        3 => {
            let n = nu * kf.powi(3) / (6.0 * pi2);
            if m == 0.0 {
                let e = nu * kf.powi(4) / (8.0 * pi2);
                return Mom { n, s: 0.0, e, p: e / 3.0 };
            }
            if kf < 0.2 * m {
                // power series in t = (kf/M)^2: (1 + t)^(-1/2) = sum_j BM[j] t^j, (1 + t)^(1/2) = sum_j BP[j] t^j
                const BM: [f64; 9] = [1.0, -1.0 / 2.0, 3.0 / 8.0, -5.0 / 16.0, 35.0 / 128.0, -63.0 / 256.0, 231.0 / 1024.0, -429.0 / 2048.0, 6435.0 / 32768.0];
                const BP: [f64; 9] = [1.0, 1.0 / 2.0, -1.0 / 8.0, 1.0 / 16.0, -5.0 / 128.0, 7.0 / 256.0, -21.0 / 1024.0, 33.0 / 2048.0, -429.0 / 32768.0];
                let t = (kf / m) * (kf / m);
                let (mut ss, mut se, mut sp) = (0.0, 0.0, 0.0);
                for j in (0..9).rev() {
                    ss = ss * t + BM[j] / (2 * j + 3) as f64;
                    se = se * t + BP[j] / (2 * j + 3) as f64;
                    sp = sp * t + BM[j] / (2 * j + 5) as f64;
                }
                let c = nu / (2.0 * pi2);
                return Mom { n, s: sign * c * kf.powi(3) * ss, e: c * m * kf.powi(3) * se, p: c / 3.0 * kf.powi(5) / m * sp };
            }
            let e = (kf * kf + m * m).sqrt();
            let l = (kf / m).asinh();
            let m4 = m.powi(4);
            Mom {
                n,
                s: sign * nu * m * (kf * e - m * m * l) / (4.0 * pi2),
                e: nu * (kf * e * (2.0 * kf * kf + m * m) - m4 * l) / (16.0 * pi2),
                p: nu * (kf * e * (2.0 * kf * kf - 3.0 * m * m) + 3.0 * m4 * l) / (48.0 * pi2),
            }
        }
        _ => panic!("half_moments: closed forms are implemented for d = 3 and d = 4"),
    }
}

/// The scalar density alone.
pub fn half_scalar(d: usize, nu: f64, kf: f64, mass: f64) -> f64 {
    half_moments(d, nu, kf, mass).s
}

/// A state of the two halves.
#[derive(Clone, Copy, Debug)]
pub struct HState {
    /// Fermi momenta of the halves W = +1, W = -1
    pub kf: [f64; 2],
    /// masses of the particles of the two halves
    pub mass: [f64; 2],
    pub n: [f64; 2],
    pub s: [f64; 2],
    /// energy shifts V_+, V_-
    pub v: [f64; 2],
    /// chemical potentials sqrt(k^2 + M^2) + V of the two halves (for an empty half: the energy |M| + V of its lowest state)
    pub mu: [f64; 2],
    /// energy density
    pub eps: f64,
    /// the interaction pressure Pi
    pub p_int: f64,
    /// pressure along the directions in which the particles have momentum
    pub p_flat: f64,
    pub e_per_particle: f64,
    /// polarization (n_+ - n_-)/n
    pub pol: f64,
    /// number of solutions of the gap equations with non-negative masses at these Fermi momenta
    pub n_solutions: usize,
}

impl HState {
    pub fn n_total(&self) -> f64 {
        self.n[0] + self.n[1]
    }
}

/// The two halves in `d` = 3 or 4 space dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Halves {
    pub d: usize,
    /// number of internal states of ONE half (4 for the 16-component field)
    pub nu_half: f64,
    pub c: HalvesCouplings,
}

/// Roots of f on the sorted grid `xs`: every interval between two neighbouring grid points on
/// which f changes its sign and is finite at both ends is refined with Brent's method.  A
/// point at which f is not defined must return NaN.
fn roots_on_grid<F: FnMut(f64) -> f64>(mut f: F, xs: &[f64]) -> Vec<f64> {
    let mut out: Vec<f64> = Vec::new();
    let mut prev: Option<(f64, f64)> = None;
    for &x in xs {
        let y = f(x);
        if !y.is_finite() {
            prev = None;
            continue;
        }
        if y == 0.0 {
            out.push(x);
            prev = None;
            continue;
        }
        if let Some((xp, yp)) = prev {
            if yp * y < 0.0 {
                // refine; if the function turns out not to be defined inside, the interval is given up
                let mut bad = false;
                let r = root(
                    |t| {
                        let v = f(t);
                        if !v.is_finite() {
                            bad = true;
                            return 0.0;
                        }
                        v
                    },
                    xp,
                    x,
                );
                if !bad {
                    out.push(r);
                }
            }
        }
        prev = Some((x, y));
    }
    out
}

/// A grid on [0, hi]: `n_lin` equidistant points and `n_log` points in geometric progression
/// from 1e-9 hi, merged and sorted.
fn mass_grid(hi: f64, n_lin: usize, n_log: usize) -> Vec<f64> {
    let mut xs: Vec<f64> = (0..=n_lin).map(|i| hi * i as f64 / n_lin as f64).collect();
    for i in 0..n_log {
        xs.push(hi * 10f64.powf(-9.0 * (1.0 - i as f64 / n_log as f64)));
    }
    xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    xs.dedup();
    xs
}

/// All solutions (x, y) of the two coupled equations
///
///     x = m + a S1(x) + b S2(y),        y = m + a S2(y) + b S1(x)        (b not zero),
///
/// for x between x_lo and x_hi.  S2 must be strictly monotonic with the open range
/// (r_lo, r_hi) of values and the inverse `inv2` (which returns None outside).
///
/// Method.  The unknown is the shift u = x - m (at weak coupling or low density the shifts
/// are many orders of magnitude smaller than m, and a formulation in x itself would lose
/// all their digits).  The first equation gives S2(y) = phi(u)/b =: R(u) with
/// phi(u) = u - a S1(m + u), and y = inv2(R) follows uniquely.  The second equation is then
/// one equation F(u) = [y(u) - m] - a R(u) - b S1(m + u) = 0 for one unknown.  R(u) lies in
/// the range of S2 only on certain intervals ("windows") of u, which can be extremely
/// narrow (their width is of the order |b| times the density).  The windows are therefore
/// determined first: their ends are the roots of phi(u) = b r_lo and phi(u) = b r_hi, found
/// on a grid that is geometric around u = 0.  Inside each window F is evaluated at 49 points
/// that crowd toward the two ends (where y(u) can vary steeply), and every change of sign is
/// refined with Brent's method.
fn coupled_solutions(m: f64, a: f64, b: f64, s1: &dyn Fn(f64) -> f64, s2: &dyn Fn(f64) -> f64, inv2: &dyn Fn(f64) -> Option<f64>, range2: (f64, f64), x_lo: f64, x_hi: f64) -> Vec<[f64; 2]> {
    let (u_lo, u_hi) = (x_lo - m, x_hi - m);
    // grid in u: equidistant points, and points in geometric progression on both sides of u = 0
    let mut grid: Vec<f64> = (0..=240).map(|i| u_lo + (u_hi - u_lo) * i as f64 / 240.0).collect();
    let span = u_hi.abs().max(u_lo.abs());
    for j in 0..=120 {
        let v = span * 10f64.powf(-0.25 * j as f64);
        for w in [v, -v] {
            if w > u_lo && w < u_hi {
                grid.push(w);
            }
        }
    }
    if u_lo < 0.0 && u_hi > 0.0 {
        grid.push(0.0);
    }
    grid.sort_by(|p, q| p.partial_cmp(q).unwrap());
    grid.dedup();
    let phi = |u: f64| u - a * s1(m + u);
    let mut crit: Vec<f64> = vec![u_lo, u_hi];
    crit.extend(roots_on_grid(|u| phi(u) - b * range2.0, &grid));
    crit.extend(roots_on_grid(|u| phi(u) - b * range2.1, &grid));
    crit.sort_by(|p, q| p.partial_cmp(q).unwrap());
    let f = |u: f64| -> f64 {
        let sx = s1(m + u);
        let r = (u - a * sx) / b;
        match inv2(r) {
            Some(y) => (y - m) - a * r - b * sx,
            None => f64::NAN,
        }
    };
    let mut out: Vec<[f64; 2]> = Vec::new();
    let push = |u: f64, out: &mut Vec<[f64; 2]>| {
        let sx = s1(m + u);
        if let Some(y) = inv2((u - a * sx) / b) {
            let x = m + u;
            // y comes from the inversion of S2 and carries its tolerance; one step of the second
            // equation, y = m + a S2(y) + b S1(x), removes it where that step is a contraction
            let res = |y: f64| y - m - a * s2(y) - b * sx;
            let y2 = m + a * s2(y) + b * sx;
            let y = if res(y2).abs() < res(y).abs() { y2 } else { y };
            if !out.iter().any(|o| o[0] == x && o[1] == y) {
                out.push([x, y]);
            }
        }
    };
    for w in crit.windows(2) {
        let (lo, hi) = (w[0], w[1]);
        if !(hi > lo) {
            continue;
        }
        let r_mid = phi(0.5 * (lo + hi)) / b;
        if !(r_mid > range2.0 && r_mid < range2.1) {
            continue;
        }
        // 49 points between the ends, crowded toward the ends
        let np = 48usize;
        let local: Vec<f64> = (0..=np)
            .map(|j| {
                let t = 0.5 * (1.0 - (PI * j as f64 / np as f64).cos());
                let t = t.clamp(1e-11, 1.0 - 1e-11);
                lo + (hi - lo) * t
            })
            .collect();
        // a root exactly at an end of the window (for example the solution x = y = 0 of a massless field)
        for u in [lo, hi] {
            if f(u) == 0.0 {
                push(u, &mut out);
            }
        }
        for u in roots_on_grid(&f, &local) {
            push(u, &mut out);
        }
    }
    out
}

impl Halves {
    fn scalar(&self, kf: f64, mass: f64) -> f64 {
        half_scalar(self.d, self.nu_half, kf, mass)
    }

    /// Density of one half filled up to the Fermi momentum 1.
    pub fn unit_density(&self) -> f64 {
        half_moments(self.d, self.nu_half, 1.0, 0.0).n
    }

    /// Fermi momentum of one half with the density `n_half`.
    pub fn kf_of(&self, n_half: f64) -> f64 {
        if n_half <= 0.0 {
            0.0
        } else {
            (n_half / self.unit_density()).powf(1.0 / self.d as f64)
        }
    }

    /// The mass M >= 0 at which a ball of radius kf has the scalar density s_target
    /// (0 <= s_target < density of the ball); the scalar density grows monotonically with M.
    fn mass_of_scalar(&self, kf: f64, s_target: f64) -> Option<f64> {
        let n = self.unit_density() * kf.powi(self.d as i32);
        if !(s_target >= 0.0 && s_target < n * (1.0 - 1e-13)) {
            return None;
        }
        if s_target == 0.0 {
            return Some(0.0);
        }
        // upper bracket: s -> n as M -> infinity
        let mut hi = kf.max(1e-3);
        let mut it = 0;
        while self.scalar(kf, hi) < s_target {
            hi *= 4.0;
            it += 1;
            if it > 60 {
                return None;
            }
        }
        Some(root(|x| self.scalar(kf, x) - s_target, 0.0, hi))
    }

    /// ALL solutions (M_+, M_-) with non-negative masses of the two gap equations
    ///     M_+ = m + a_S s_+(M_+) + b_S s_-(M_-),     M_- = m + a_S s_-(M_-) + b_S s_+(M_+)
    /// (a_S = T_S + D_S, b_S = T_S - D_S) for the Fermi momenta kf = (k_+, k_-).
    ///
    /// Method.  If both halves are occupied and b_S is not zero, the first equation gives
    /// s_-(M_-) = [M_+ - m - a_S s_+(M_+)]/b_S =: R(M_+), and M_- follows uniquely because s_-
    /// grows monotonically with M_- (no solution if R lies outside [0, n_-)).  The second
    /// equation is then one equation F(M_+) = M_-(M_+) - m - a_S R - b_S s_+(M_+) = 0 for one
    /// unknown, whose roots are searched between 0 and the
    /// bound m + |a_S| n_+ + |b_S| n_- (see `coupled_solutions`, which treats correctly the
    /// narrow intervals of M_+ on which R lies in the range of s_-).  If one half is empty or
    /// b_S = 0 the equations decouple.
    pub fn solutions(&self, kf: &[f64; 2]) -> Vec<[f64; 2]> {
        let (a, b, m) = (self.c.a_s(), self.c.b_s(), self.c.m);
        let unit = self.unit_density();
        let n = [unit * kf[0].powi(self.d as i32), unit * kf[1].powi(self.d as i32)];
        let hi = m + a.abs() * n[0].max(n[1]) + b.abs() * n[0].max(n[1]) + 1e-12;
        let grid = mass_grid(hi, 240, 120);
        // roots of M = m0 + a s(k, M) for one half, M >= 0
        let single = |k: f64, m0: f64| -> Vec<f64> {
            if k == 0.0 {
                return if m0 >= 0.0 { vec![m0] } else { vec![] };
            }
            roots_on_grid(|x| x - m0 - a * self.scalar(k, x), &grid)
        };
        let mut out: Vec<[f64; 2]> = Vec::new();
        if kf[0] == 0.0 && kf[1] == 0.0 {
            out.push([m, m]);
        } else if kf[1] == 0.0 {
            // the half W = -1 is empty: s_- = 0
            for mp in single(kf[0], m) {
                out.push([mp, m + b * self.scalar(kf[0], mp)]);
            }
        } else if kf[0] == 0.0 {
            for mm in single(kf[1], m) {
                out.push([m + b * self.scalar(kf[1], mm), mm]);
            }
        } else if b == 0.0 {
            let (r0, r1) = (single(kf[0], m), single(kf[1], m));
            for &x in &r0 {
                for &y in &r1 {
                    out.push([x, y]);
                }
            }
        } else {
            // the scalar density of the half W = -1 takes the values [0, n_-) for masses in [0, infinity)
            let s1 = |x: f64| self.scalar(kf[0], x);
            let s2 = |x: f64| self.scalar(kf[1], x);
            let inv2 = |r: f64| self.mass_of_scalar(kf[1], r);
            out = coupled_solutions(m, a, b, &s1, &s2, &inv2, (-1e-300, n[1] * (1.0 - 1e-13)), 0.0, hi);
        }
        out
    }

    /// The state for given Fermi momenta and masses (the masses need not solve the gap
    /// equations: the energy shifts and the energy are computed from the densities).
    pub fn state_at(&self, kf: &[f64; 2], mass: &[f64; 2], n_solutions: usize) -> HState {
        let c = &self.c;
        let (a, b) = (half_moments(self.d, self.nu_half, kf[0], mass[0]), half_moments(self.d, self.nu_half, kf[1], mass[1]));
        let (n, nw, ns, nsw) = (a.n + b.n, a.n - b.n, a.s + b.s, a.s - b.s);
        let vector = 0.5 * (c.tv * n * n + c.dv * nw * nw);
        let scalar = 0.5 * (c.ts * ns * ns + c.ds * nsw * nsw);
        let eps = a.e + b.e + vector - scalar;
        let p_int = vector + scalar;
        let v = [c.tv * n + c.dv * nw, c.tv * n - c.dv * nw];
        let mu = [(kf[0] * kf[0] + mass[0] * mass[0]).sqrt() + v[0], (kf[1] * kf[1] + mass[1] * mass[1]).sqrt() + v[1]];
        HState { kf: *kf, mass: *mass, n: [a.n, b.n], s: [a.s, b.s], v, mu, eps, p_int, p_flat: a.p + b.p + p_int, e_per_particle: if n > 0.0 { eps / n } else { c.m }, pol: if n > 0.0 { nw / n } else { 0.0 }, n_solutions }
    }

    /// The solution of the gap equations of lowest energy density for the Fermi momenta kf
    /// (None if the gap equations have no solution with non-negative masses of the occupied halves).
    pub fn solve(&self, kf: &[f64; 2]) -> Option<HState> {
        let sols = self.solutions(kf);
        let ns = sols.len();
        let mut best: Option<HState> = None;
        for mass in sols {
            // the mass of an empty half does not enter the energy; a negative value is the mass -|M| of a Dirac particle of mass |M|
            let st = self.state_at(kf, &mass, ns);
            if best.as_ref().map_or(true, |b| st.eps < b.eps) {
                best = Some(st);
            }
        }
        best
    }

    /// Total density that corresponds to the Fermi momentum `kf_total` when both halves are filled equally.
    pub fn density_of_kf_total(&self, kf_total: f64) -> f64 {
        2.0 * self.unit_density() * kf_total.powi(self.d as i32)
    }

    /// The largest eigenvalue of the response matrix of the two gap equations at a state.
    ///
    /// The gap equations are M_+ = m + a_S s_+ + b_S s_-, M_- = m + b_S s_+ + a_S s_-, with the
    /// scalar densities s_+-(k_+-, M_+-) of the two halves.  A small change of the masses changes
    /// the right-hand sides by the matrix  R = [[a_S s_+', b_S s_-'], [b_S s_+', a_S s_-']],
    /// s' = ds/dM at fixed Fermi momentum (positive for particles of positive energy).  In
    /// dilute matter the eigenvalues of R are small.  The solution that is connected to the
    /// dilute gas is a regular function of the density and of the couplings as long as both
    /// eigenvalues stay below 1; where one reaches 1 the solution branches, and beyond that
    /// point the state is unstable against the corresponding change of the two masses.  For
    /// equal halves the eigenvalues are 2 T_S s' (the two masses change together) and
    /// 2 D_S s' (they split); for one filled half they are a_S s' and 0.
    pub fn response_eigenvalue(&self, st: &HState) -> f64 {
        let sp = |kf: f64, mass: f64| -> f64 {
            if kf <= 0.0 {
                return 0.0;
            }
            let h = 1e-5 * (1.0 + mass.abs());
            (half_moments(self.d, self.nu_half, kf, mass + h).s - half_moments(self.d, self.nu_half, kf, mass - h).s) / (2.0 * h)
        };
        let (s1, s2) = (sp(st.kf[0], st.mass[0]), sp(st.kf[1], st.mass[1]));
        let (a, b) = (self.c.a_s(), self.c.b_s());
        let tr = a * (s1 + s2);
        let disc = (a * a * (s1 - s2).powi(2) + 4.0 * b * b * s1 * s2).sqrt();
        0.5 * (tr + disc)
    }

    /// The state of total density `n` and polarization `pol` = (n_+ - n_-)/n in [0, 1].
    pub fn at_polarization(&self, n: f64, pol: f64) -> Option<HState> {
        let pol = pol.clamp(0.0, 1.0);
        let kf = [self.kf_of(0.5 * n * (1.0 + pol)), if pol >= 1.0 { 0.0 } else { self.kf_of(0.5 * n * (1.0 - pol)) }];
        self.solve(&kf)
    }

    /// The state of lowest energy at the total density `n`: minimum over the polarization
    /// (21 equidistant values, then golden-section refinement around the best one; the ends
    /// pol = 0 and pol = 1 are always included).
    pub fn lowest_at_density(&self, n: f64) -> Option<HState> {
        let np = 20usize;
        let mut vals: Vec<Option<HState>> = Vec::with_capacity(np + 1);
        let mut best: Option<(usize, HState)> = None;
        for i in 0..=np {
            let st = self.at_polarization(n, i as f64 / np as f64);
            if let Some(s) = st {
                if best.as_ref().map_or(true, |b| s.eps < b.1.eps) {
                    best = Some((i, s));
                }
            }
            vals.push(st);
        }
        let (i, mut st) = best?;
        let (lo, hi) = ((i as f64 - 1.0).max(0.0) / np as f64, (i as f64 + 1.0).min(np as f64) / np as f64);
        if hi > lo {
            let f = |p: f64| self.at_polarization(n, p).map_or(f64::INFINITY, |s| s.eps);
            let p = golden_min(f, lo, hi, 1e-7);
            if let Some(s) = self.at_polarization(n, p) {
                // the refined value replaces the grid value only if it is lower by more than rounding errors,
                // so that the exact ends pol = 0 and pol = 1 are not replaced by values next to them
                if s.eps < st.eps - 1e-13 * st.eps.abs() {
                    st = s;
                }
            }
        }
        // an interior minimum is the point at which the two chemical potentials are equal: this condition
        // locates it more accurately than the comparison of energies
        if st.pol > 1e-6 && st.pol < 1.0 - 1e-6 {
            let dmu = |p: f64| self.at_polarization(n, p).map_or(f64::NAN, |s| s.mu[0] - s.mu[1]);
            let (a, b) = ((st.pol - 1e-3).max(1e-9), (st.pol + 1e-3).min(1.0 - 1e-9));
            let (fa, fb) = (dmu(a), dmu(b));
            if fa.is_finite() && fb.is_finite() && fa * fb < 0.0 {
                let mut bad = false;
                let p = root(
                    |p| {
                        let v = dmu(p);
                        if v.is_finite() {
                            v
                        } else {
                            bad = true;
                            0.0
                        }
                    },
                    a,
                    b,
                );
                if !bad {
                    if let Some(s) = self.at_polarization(n, p) {
                        if s.eps <= st.eps + 1e-12 * st.eps.abs() {
                            st = s;
                        }
                    }
                }
            }
        }
        Some(st)
    }

    /// The lowest energy per particle over the total densities that correspond to kf_total in
    /// [lo, hi]: scan of `n_scan` + 1 values of ln kf_total, then golden-section refinement
    /// around EVERY grid point that is not higher than its neighbours (the energy per particle
    /// of a dilute gas tends to m from above, so that the lowest grid value can lie at the
    /// dilute end while a narrow bound minimum lies between two grid points elsewhere).
    /// Returns (kf_total, state).
    pub fn lowest_energy_per_particle(&self, lo: f64, hi: f64, n_scan: usize) -> Option<(f64, HState)> {
        let (a, b) = (lo.ln(), hi.ln());
        let h = (b - a) / n_scan as f64;
        let e = |l: f64| self.lowest_at_density(self.density_of_kf_total(l.exp())).map_or(f64::INFINITY, |s| s.e_per_particle);
        let v: Vec<f64> = (0..=n_scan).map(|i| e(a + h * i as f64)).collect();
        let mut best = (f64::INFINITY, a);
        for i in 0..=n_scan {
            let left = if i == 0 { f64::INFINITY } else { v[i - 1] };
            let right = if i == n_scan { f64::INFINITY } else { v[i + 1] };
            if v[i].is_finite() && v[i] <= left && v[i] <= right {
                let l0 = a + h * i as f64;
                let l = golden_min(e, (l0 - h).max(a), (l0 + h).min(b), 1e-9);
                let el = e(l);
                let (l, el) = if el <= v[i] { (l, el) } else { (l0, v[i]) };
                if el < best.0 {
                    best = (el, l);
                }
            }
        }
        if !best.0.is_finite() {
            return None;
        }
        self.lowest_at_density(self.density_of_kf_total(best.1.exp())).map(|s| (best.1.exp(), s))
    }

    /// The matter of one kind of particle that a fully polarized state is: `nu_half` states,
    /// scalar coupling a_S and vector coupling a_V (for the comparison with module `kohn_sham`).
    pub fn polarized_contact(&self) -> Contact {
        Contact { m: self.c.m, gs: self.c.a_s(), gv: self.c.a_v() }
    }

    /// The matter with equal halves as matter of one kind of particle with 2 `nu_half` states:
    /// scalar coupling T_S and vector coupling T_V.
    pub fn unpolarized_contact(&self) -> Contact {
        Contact { m: self.c.m, gs: self.c.ts, gv: self.c.tv }
    }
}

/// The scan of the total density used for saturation points and binding thresholds:
/// kf_total from 0.05 m to 5 m in 80 steps of ln kf_total.
pub const KF_SCAN: (f64, f64, usize) = (0.05, 5.0, 80);

/// The two halves in d dimensions for the couplings g times `hf_unit` and the mass 1.
pub fn halves_for(d: usize, form: Form, hf_unit: &Hf4, g: f64) -> Halves {
    Halves { d, nu_half: 4.0, c: HalvesCouplings::new(form, 1.0, &hf_unit.scaled(g)) }
}

/// The smallest strength g at which uniform matter with the couplings g times `hf_unit` is
/// self-bound: its lowest energy per particle, over all densities with kf_total in
/// [0.05, 5] (scan of 81 values with refinement around every local minimum) and all
/// polarizations, lies below the mass (bisection in ln g on [0.01, 1e5]).
/// Units m = 1.  Returns NaN if the interval contains no threshold.
pub fn binding_threshold(d: usize, form: Form, hf_unit: &Hf4) -> f64 {
    let bound = |ln_g: f64| halves_for(d, form, hf_unit, ln_g.exp()).lowest_energy_per_particle(KF_SCAN.0, KF_SCAN.1, KF_SCAN.2).map_or(false, |(_, s)| s.e_per_particle - 1.0 < -1e-10);
    let (mut lo, mut hi) = (1e-2f64.ln(), 1e5f64.ln());
    if bound(lo) || !bound(hi) {
        return f64::NAN;
    }
    for _ in 0..48 {
        let mid = 0.5 * (lo + hi);
        if bound(mid) {
            hi = mid;
        } else {
            lo = mid;
        }
    }
    (0.5 * (lo + hi)).exp()
}

// --------------------------------------------------------------------------------------
// the effective potential of two halves
// --------------------------------------------------------------------------------------

/// The CJT effective potential of the two halves (see the head of this file).
#[derive(Clone, Copy, Debug)]
pub struct CjtHalves {
    pub d: usize,
    pub nu_half: f64,
    pub c: HalvesCouplings,
    /// None: no-sea approximation; Some(Lambda): the filled states of negative energy below the cutoff are included
    pub sea: Option<f64>,
}

/// A stationary point of the effective potential of two halves.
#[derive(Clone, Copy, Debug)]
pub struct CjtHalvesState {
    pub mass: [f64; 2],
    pub v: [f64; 2],
    pub n: [f64; 2],
    /// scalar densities of the particles of the two Fermi seas
    pub s: [f64; 2],
    /// scalar densities of the filled states of negative energy (0 without sea)
    pub s_sea: [f64; 2],
    pub omega: f64,
    /// energy density Omega + mu n
    pub eps: f64,
    /// largest residual of the four stationarity conditions
    pub residual: f64,
}

fn solve_linear(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for i in 0..n {
        let p = (i..n).max_by(|&x, &y| a[x][i].abs().partial_cmp(&a[y][i].abs()).unwrap())?;
        if a[p][i].abs() < 1e-300 {
            return None;
        }
        a.swap(i, p);
        b.swap(i, p);
        for r in (i + 1)..n {
            let f = a[r][i] / a[i][i];
            for c in i..n {
                a[r][c] -= f * a[i][c];
            }
            b[r] -= f * b[i];
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for c in (i + 1)..n {
            s -= a[i][c] * x[c];
        }
        x[i] = s / a[i][i];
    }
    Some(x)
}

impl CjtHalves {
    /// The free gas of one half (module `cjt_contact`, Simpson integrals).
    fn half(&self) -> Cjt {
        Cjt { d: self.d, nu: self.nu_half, c: Contact { m: self.c.m, gs: 0.0, gv: 0.0 }, sea: self.sea }
    }

    /// (n_+-, s_+- of the Fermi seas, s_+- of the filled states of negative energy, sum of the free grand potentials).
    fn free(&self, mass: &[f64; 2], v: &[f64; 2], mu: f64) -> ([f64; 2], [f64; 2], [f64; 2], f64) {
        let h = self.half();
        let (mut n, mut s, mut ss) = ([0.0; 2], [0.0; 2], [0.0; 2]);
        let mut om = 0.0;
        for i in 0..2 {
            let (rho, rho_s, p) = h.fermi_sea(mass[i].abs(), mu - v[i]);
            n[i] = rho;
            s[i] = rho_s * if mass[i] < 0.0 { -1.0 } else { 1.0 };
            ss[i] = h.sea_rho_s(mass[i]);
            om += -p + h.sea_energy(mass[i]);
        }
        (n, s, ss, om)
    }

    /// Omega(M_+, M_-, V_+, V_-; mu).  A term whose coupling is zero is absent (the field
    /// that belongs to it must then be zero).
    pub fn omega(&self, mass: &[f64; 2], v: &[f64; 2], mu: f64) -> f64 {
        let c = &self.c;
        let (_, _, _, mut om) = self.free(mass, v, mu);
        let (mu_s, d_s) = (0.5 * (mass[0] + mass[1]) - c.m, 0.5 * (mass[0] - mass[1]));
        let (v_t, v_d) = (0.5 * (v[0] + v[1]), 0.5 * (v[0] - v[1]));
        for (x, g) in [(mu_s, c.ts), (d_s, c.ds), (v_t, c.tv), (v_d, c.dv)] {
            if g != 0.0 {
                om -= x * x / (2.0 * g);
            }
        }
        om
    }

    /// The four stationarity conditions, written as the differences
    /// (muS - T_S n_S, dS - D_S n_SW, vT - T_V n, vD - D_V n_W), with the scalar densities of
    /// the filled states of negative energy included in n_S and n_SW.
    pub fn residuals(&self, mass: &[f64; 2], v: &[f64; 2], mu: f64) -> [f64; 4] {
        let c = &self.c;
        let (n, s, ss, _) = self.free(mass, v, mu);
        let (sp, sm) = (s[0] + ss[0], s[1] + ss[1]);
        [0.5 * (mass[0] + mass[1]) - c.m - c.ts * (sp + sm), 0.5 * (mass[0] - mass[1]) - c.ds * (sp - sm), 0.5 * (v[0] + v[1]) - c.tv * (n[0] + n[1]), 0.5 * (v[0] - v[1]) - c.dv * (n[0] - n[1])]
    }

    pub fn state(&self, mass: &[f64; 2], v: &[f64; 2], mu: f64) -> CjtHalvesState {
        let (n, s, s_sea, _) = self.free(mass, v, mu);
        let omega = self.omega(mass, v, mu);
        let r = self.residuals(mass, v, mu);
        CjtHalvesState { mass: *mass, v: *v, n, s, s_sea, omega, eps: omega + mu * (n[0] + n[1]), residual: r.iter().fold(0.0, |a: f64, x| a.max(x.abs())) }
    }

    /// The stationary point of Omega near the start values, by Newton's method on the four
    /// stationarity conditions with a Jacobian of central differences.  Returns None if the
    /// iteration does not converge.
    pub fn stationary_near(&self, mu: f64, mass0: &[f64; 2], v0: &[f64; 2]) -> Option<CjtHalvesState> {
        let mut x = [mass0[0], mass0[1], v0[0], v0[1]];
        let res = |x: &[f64; 4]| self.residuals(&[x[0], x[1]], &[x[2], x[3]], mu);
        let scale = self.c.m.abs().max(mu.abs()).max(1e-3);
        for _ in 0..60 {
            let r = res(&x);
            let err = r.iter().fold(0.0, |a: f64, y| a.max(y.abs()));
            if err < 1e-12 * scale {
                return Some(self.state(&[x[0], x[1]], &[x[2], x[3]], mu));
            }
            let h = 1e-6 * scale;
            let mut jac = vec![vec![0.0; 4]; 4];
            for j in 0..4 {
                let (mut xp, mut xm) = (x, x);
                xp[j] += h;
                xm[j] -= h;
                let (rp, rm) = (res(&xp), res(&xm));
                for i in 0..4 {
                    jac[i][j] = (rp[i] - rm[i]) / (2.0 * h);
                }
            }
            let dx = solve_linear(jac, r.to_vec())?;
            for j in 0..4 {
                x[j] -= dx[j];
            }
            if !x.iter().all(|y| y.is_finite()) {
                return None;
            }
        }
        None
    }

    /// The vacuum with the filled states of negative energy: ALL solutions (M_+, M_-) of
    ///     (M_+ + M_-)/2 - m = T_S (s_+ + s_-),   (M_+ - M_-)/2 = D_S (s_+ - s_-),
    ///     s_+- = -(NU/2) Int_{|k| < Lambda} d^Dk/(2 pi)^D M_+-/sqrt(k^2 + M_+-^2),
    /// with masses of either sign, each with its value of Omega_vac = sum of the energies of the
    /// filled states (relative to mass m) - muS^2/(2 T_S) - dS^2/(2 D_S).  Sorted by Omega.
    /// The scalar density of the filled states falls monotonically with the mass, so that the
    /// search is the same one-dimensional search as in `Halves::solutions`.
    pub fn vacuum(&self) -> Vec<([f64; 2], f64)> {
        let lam = match self.sea {
            Some(l) => l,
            None => return vec![([self.c.m, self.c.m], 0.0)],
        };
        let h = self.half();
        let (a, b, m) = (self.c.ts + self.c.ds, self.c.ts - self.c.ds, self.c.m);
        // |s| < bound
        let bound = self.nu_half * crate::kohn_sham::ball_volume(self.d) * lam.powi(self.d as i32) / (2.0 * PI).powi(self.d as i32);
        let hi = m.abs() + (a.abs() + b.abs()) * bound + 1e-9;
        let mut grid: Vec<f64> = mass_grid(hi, 160, 60);
        let neg: Vec<f64> = grid.iter().filter(|x| **x > 0.0).map(|x| -x).collect();
        grid.extend(neg);
        grid.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let s_of = |x: f64| h.sea_rho_s(x);
        // inverse of the monotonically falling function s(M) on (-bound, bound)
        let inv = |s: f64| -> Option<f64> {
            if !(s.abs() < bound * (1.0 - 1e-12)) {
                return None;
            }
            if s == 0.0 {
                return Some(0.0);
            }
            let mut top = lam.max(1e-3);
            let mut it = 0;
            while s_of(top).abs() < s.abs() {
                top *= 4.0;
                it += 1;
                if it > 60 {
                    return None;
                }
            }
            // s > 0 belongs to a negative mass
            Some(if s > 0.0 { root(|x| s_of(x) - s, -top, 0.0) } else { root(|x| s_of(x) - s, 0.0, top) })
        };
        let mut sols: Vec<[f64; 2]> = Vec::new();
        if b == 0.0 {
            let r: Vec<f64> = roots_on_grid(|x| x - m - a * s_of(x), &grid);
            for &x in &r {
                for &y in &r {
                    sols.push([x, y]);
                }
            }
        } else {
            sols = coupled_solutions(m, a, b, &s_of, &s_of, &inv, (-bound * (1.0 - 1e-12), bound * (1.0 - 1e-12)), -hi, hi);
        }
        let mut out: Vec<([f64; 2], f64)> = sols.into_iter().map(|ms| (ms, self.omega(&ms, &[0.0, 0.0], 0.0))).collect();
        out.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        out
    }
}

// --------------------------------------------------------------------------------------
// two halves together with the Vorton-Scalar and a bare Dirac mass
// --------------------------------------------------------------------------------------

/// The mean-field functional with a bare Dirac mass, the self-interaction in one of its two
/// forms and the expectation value of the Vorton-Scalar, for fields in ordinary space
/// (D = 3).  Units: the mass of the fermions in vacuum, m = m_D + m_Y, is 1.
///
///     eps = (1 - f_D)^2 (x^2 - 1)^2/(8 C)                  potential of the scalar, x = phi/v_VS
///         + (energy density of the two halves with the mass f_D + (1 - f_D) x in place of m) ,
///
/// f_D = m_D/m, C = g_VS^2 m^2/m_s^2.  The Yukawa coupling of the Vorton-Scalar contains the
/// bilinear psibar psi = psi^dagger beta psi, in which the two halves enter alike: the
/// scalar field shifts the masses of both halves by the same amount, in both forms.
#[derive(Clone, Copy, Debug)]
pub struct UnifiedHalves {
    pub form: Form,
    pub f_d: f64,
    /// the number C of the Vorton-Scalar
    pub c: f64,
    /// the four Hartree-Fock couplings of the self-interaction (units m = 1)
    pub hf: Hf4,
}

/// A state of the functional of `UnifiedHalves`.
#[derive(Clone, Copy, Debug)]
pub struct UnifiedHalvesState {
    /// phi/v_VS
    pub x: f64,
    pub halves: HState,
    /// energy density including the potential of the scalar
    pub eps: f64,
    pub e_per_particle: f64,
}

impl UnifiedHalves {
    fn potential(&self, x: f64) -> f64 {
        (1.0 - self.f_d).powi(2) * (x * x - 1.0).powi(2) / (8.0 * self.c)
    }

    /// The state of lowest energy of the two halves for the total density n at a given x.
    pub fn at_x(&self, n: f64, x: f64) -> Option<UnifiedHalvesState> {
        let m_x = self.f_d + (1.0 - self.f_d) * x;
        let h = Halves { d: 3, nu_half: 4.0, c: HalvesCouplings::new(self.form, m_x, &self.hf) };
        let st = h.lowest_at_density(n)?;
        let eps = st.eps + self.potential(x);
        Some(UnifiedHalvesState { x, halves: st, eps, e_per_particle: eps / n })
    }

    /// The state of lowest energy for the total density n on 0 <= x <= 1.2: 13 equidistant
    /// values of x, golden-section refinement around the lowest to 1e-4, and one step of
    /// parabolic interpolation through three points at the distance 1e-3 (the energy is a
    /// smooth function of x near an interior minimum; the step is accepted only if it lowers
    /// the energy).
    pub fn state(&self, n: f64) -> Option<UnifiedHalvesState> {
        if self.f_d >= 1.0 {
            return self.at_x(n, 1.0);
        }
        let nx = 12usize;
        let h = 1.2 / nx as f64;
        let mut best: Option<UnifiedHalvesState> = None;
        for i in 0..=nx {
            if let Some(s) = self.at_x(n, h * i as f64) {
                if best.as_ref().map_or(true, |b| s.eps < b.eps) {
                    best = Some(s);
                }
            }
        }
        let mut b = best?;
        let x = golden_min(|x| self.at_x(n, x).map_or(f64::INFINITY, |s| s.eps), (b.x - h).max(0.0), (b.x + h).min(1.2), 1e-4);
        if let Some(s) = self.at_x(n, x) {
            if s.eps < b.eps - 1e-13 * b.eps.abs() {
                b = s;
            }
        }
        // parabolic step
        let dx = 1e-3;
        if b.x > dx && b.x < 1.2 - dx {
            if let (Some(lo), Some(hi)) = (self.at_x(n, b.x - dx), self.at_x(n, b.x + dx)) {
                let curv = hi.eps - 2.0 * b.eps + lo.eps;
                if curv > 0.0 {
                    let step = -0.5 * dx * (hi.eps - lo.eps) / curv;
                    if step.abs() <= dx {
                        if let Some(s) = self.at_x(n, b.x + step) {
                            if s.eps < b.eps {
                                b = s;
                            }
                        }
                    }
                }
            }
        }
        Some(b)
    }

    /// Density of the Fermi momentum kf_total with equal halves (D = 3, eight states): 4 kf^3/(3 pi^2).
    pub fn density_of_kf_total(kf_total: f64) -> f64 {
        4.0 * kf_total.powi(3) / (3.0 * PI * PI)
    }

    /// The lowest energy per particle over kf_total in [lo, hi] (scan of n_scan + 1 values of
    /// ln kf_total, golden-section refinement around every grid point that is not higher
    /// than its neighbours).  Returns (kf_total, state).
    pub fn lowest_energy_per_particle(&self, lo: f64, hi: f64, n_scan: usize) -> Option<(f64, UnifiedHalvesState)> {
        let (a, b) = (lo.ln(), hi.ln());
        let h = (b - a) / n_scan as f64;
        let e = |l: f64| self.state(UnifiedHalves::density_of_kf_total(l.exp())).map_or(f64::INFINITY, |s| s.e_per_particle);
        let v: Vec<f64> = (0..=n_scan).map(|i| e(a + h * i as f64)).collect();
        let mut best = (f64::INFINITY, a);
        for i in 0..=n_scan {
            let left = if i == 0 { f64::INFINITY } else { v[i - 1] };
            let right = if i == n_scan { f64::INFINITY } else { v[i + 1] };
            if v[i].is_finite() && v[i] <= left && v[i] <= right {
                let l0 = a + h * i as f64;
                let l = golden_min(e, (l0 - h).max(a), (l0 + h).min(b), 1e-6);
                let el = e(l);
                let (l, el) = if el <= v[i] { (l, el) } else { (l0, v[i]) };
                if el < best.0 {
                    best = (el, l);
                }
            }
        }
        if !best.0.is_finite() {
            return None;
        }
        self.state(UnifiedHalves::density_of_kf_total(best.1.exp())).map(|s| (best.1.exp(), s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kohn_sham::{ks_solve, Fermi, NU};

    #[test]
    fn closed_forms_of_the_moments_agree_with_quadrature() {
        for d in [3usize, 4] {
            let f = Fermi::isotropic(d, 4.0, 96);
            for (kf, m) in [(0.5, 1.0), (1.7, 0.3), (0.02, 1.0), (0.199, 1.0), (0.201, 1.0), (0.031, 1.0), (3.0, 0.05), (0.8, 2.5), (1.0, 0.0)] {
                let a = half_moments(d, 4.0, kf, m);
                let b = f.moments(kf, m, &[1.0; 3]);
                assert!((a.n / b.rho - 1.0).abs() < 1e-12, "d = {} n", d);
                assert!((a.s - b.rho_s).abs() < 1e-12 * b.rho, "d = {} kf = {} m = {}: s {} against {}", d, kf, m, a.s, b.rho_s);
                assert!((a.e / b.e_kin - 1.0).abs() < 1e-12, "d = {} kf = {} m = {}: e {} against {}", d, kf, m, a.e, b.e_kin);
                assert!((a.p / b.p_flat - 1.0).abs() < 2e-11, "d = {} kf = {} m = {}: p {} against {}", d, kf, m, a.p, b.p_flat);
            }
            // the scalar density has the sign of the mass
            assert_eq!(half_moments(d, 4.0, 0.7, -0.4).s, -half_moments(d, 4.0, 0.7, 0.4).s);
        }
    }

    #[test]
    fn the_closed_forms_of_the_four_couplings_are_those_of_the_exact_tables() {
        let gm = spacetimes::gamma::Gammas::build();
        let tabs = SelfTables::build(&gm);
        let g = [1.0, -0.7, 0.3, 0.2, -0.5, 0.9, 0.4, -1.3, 0.6];
        let e = crate::selfint::reduce_couplings(&g);
        let (a, b) = (Hf4::from_tables(&tabs, &g), Hf4::from_effective(&e));
        for (x, y) in [(a.gs_hf, b.gs_hf), (a.gv_hf, b.gv_hf), (a.gs_c, b.gs_c), (a.gv_c, b.gv_c)] {
            assert!((x - y).abs() < 1e-13, "{} against {}", x, y);
        }
        // quantized-sector form: the couplings of the total densities are the Hartree-Fock couplings of 7+1
        assert!((a.gs_hf - tabs.s44q.hf_coupling(1, &g)).abs() < 1e-13 && (a.gv_hf - tabs.s44q.hf_coupling(0, &g)).abs() < 1e-13);
    }

    #[test]
    fn equal_halves_and_one_half_are_kohn_sham_matter_of_one_kind() {
        let hf = Hf4::from_effective(&[-60.0, 6.0, 0.0, 0.0]);
        for form in Form::both() {
            for d in [3usize, 4] {
                let hf = if d == 3 { hf.scaled(0.2) } else { hf };
                let h = Halves { d, nu_half: 4.0, c: HalvesCouplings::new(form, 1.0, &hf) };
                let (f8, f4) = (Fermi::isotropic(d, NU, 96), Fermi::isotropic(d, NU / 2.0, 96));
                for kf in [0.4, 0.9] {
                    // equal halves, among the solutions the symmetric one
                    let sols = h.solutions(&[kf, kf]);
                    let ks = ks_solve(&f8, &h.unpolarized_contact(), kf, &[1.0; 3]);
                    let sym = sols.iter().find(|s| (s[0] - s[1]).abs() < 1e-9 && (s[0] - ks.mass).abs() < 1e-9);
                    assert!(sym.is_some(), "{:?} d = {} kf = {}: no symmetric solution near {} among {:?}", form, d, kf, ks.mass, sols);
                    let st = h.state_at(&[kf, kf], sym.unwrap(), sols.len());
                    assert!((st.eps / ks.eps - 1.0).abs() < 1e-10 && (st.p_flat - ks.p_flat).abs() < 1e-10 * ks.eps.abs());
                    // one half
                    let one = h.solve(&[kf, 0.0]).unwrap();
                    let ks1 = ks_solve(&f4, &h.polarized_contact(), kf, &[1.0; 3]);
                    assert!((one.mass[0] - ks1.mass).abs() < 1e-9 && (one.eps / ks1.eps - 1.0).abs() < 1e-10, "{:?} d = {}: {} against {}", form, d, one.mass[0], ks1.mass);
                }
            }
        }
    }

    #[test]
    fn the_chemical_potentials_are_the_derivatives_of_the_energy_density() {
        let hf = Hf4::from_effective(&[-12.0, 1.5, 0.4, -0.3]);
        for form in Form::both() {
            let h = Halves { d: 3, nu_half: 4.0, c: HalvesCouplings::new(form, 1.0, &hf) };
            let kf = [0.62, 0.48];
            let st = h.solve(&kf).unwrap();
            let unit = h.unit_density();
            // d eps/d n_+ at fixed n_-, by a central difference in k_+
            let dk = 1e-5;
            let (a, b) = (h.solve(&[kf[0] + dk, kf[1]]).unwrap(), h.solve(&[kf[0] - dk, kf[1]]).unwrap());
            let dn = unit * ((kf[0] + dk).powi(3) - (kf[0] - dk).powi(3));
            assert!(((a.eps - b.eps) / dn / st.mu[0] - 1.0).abs() < 1e-7, "{:?}: {} against {}", form, (a.eps - b.eps) / dn, st.mu[0]);
            // p = mu_+ n_+ + mu_- n_- - eps
            assert!((st.mu[0] * st.n[0] + st.mu[1] * st.n[1] - st.eps - st.p_flat).abs() < 1e-12);
        }
    }

    #[test]
    fn the_effective_potential_is_stationary_at_the_kohn_sham_state() {
        let hf = Hf4::from_effective(&[-14.0, 1.4, 0.0, 0.0]);
        for form in Form::both() {
            let c = HalvesCouplings::new(form, 1.0, &hf);
            let h = Halves { d: 3, nu_half: 4.0, c };
            let st = h.lowest_at_density(h.density_of_kf_total(0.55)).unwrap();
            // equilibrium of the two halves: the chemical potential of the occupied half(s)
            let mu = st.mu[0];
            let cj = CjtHalves { d: 3, nu_half: 4.0, c, sea: None };
            // an empty half stays empty if its lowest state lies above mu
            if st.n[1] == 0.0 {
                assert!(st.mu[1] > mu);
            } else {
                assert!((st.mu[1] / mu - 1.0).abs() < 1e-6, "{:?}: mu = {:?}", form, st.mu);
            }
            let found = cj.stationary_near(mu, &[st.mass[0] * 1.002, st.mass[1] * 0.999], &st.v).unwrap();
            assert!((found.mass[0] - st.mass[0]).abs() < 1e-7 && (found.mass[1] - st.mass[1]).abs() < 1e-6, "{:?}: {:?} against {:?}", form, found.mass, st.mass);
            assert!((found.eps / st.eps - 1.0).abs() < 1e-7 && (-found.omega - st.p_flat).abs() < 1e-7 * st.eps.abs());
        }
    }

    #[test]
    fn the_response_eigenvalue_reaches_one_where_the_solution_branches() {
        // quantized-sector form, D = 3, scalar attraction alone: T_S = -15 g/16, D_S = +g/16.  Equal halves:
        // eigenvalues 2 T_S s' < 0 and 2 D_S s' > 0.  Where 2 D_S s' passes 1, solutions with unequal masses
        // appear at equal densities.
        let hf1 = Hf4::from_effective(&[-1.0, 0.0, 0.0, 0.0]);
        let kf = 0.3;
        let lam = |g: f64| {
            let h = halves_for(3, Form::Q, &hf1, g);
            let st = h.solve(&[kf, kf]).unwrap();
            assert!((st.mass[0] - st.mass[1]).abs() < 1e-9 || h.response_eigenvalue(&st) > 0.9);
            // closed form for equal halves
            let m = st.mass[0];
            let sprime = (half_moments(3, 4.0, kf, m + 1e-5).s - half_moments(3, 4.0, kf, m - 1e-5).s) / 2e-5;
            let expected = (2.0 * h.c.ds * sprime).max(2.0 * h.c.ts * sprime);
            (h.response_eigenvalue(&st), expected, h.solutions(&[kf, kf]).len())
        };
        let (l1, e1, n1) = lam(20.0);
        assert!((l1 - e1).abs() < 1e-7 && l1 < 1.0 && n1 == 1, "{} {} {}", l1, e1, n1);
        // one filled half: eigenvalue a_S s' (negative for an attraction inside the half)
        let h = halves_for(3, Form::C, &hf1, 20.0);
        let st = h.solve(&[0.4, 0.0]).unwrap();
        assert!(h.response_eigenvalue(&st) <= 0.0);
        // strong coupling: find a strength at which the eigenvalue of the equal-halves solution exceeds 1,
        // and verify that solutions with unequal masses then exist at equal densities
        let mut g = 100.0;
        let mut found = false;
        while g < 1.0e5 {
            let h = halves_for(3, Form::Q, &hf1, g);
            let sols = h.solutions(&[kf, kf]);
            let sym: Vec<&[f64; 2]> = sols.iter().filter(|x| (x[0] - x[1]).abs() < 1e-9).collect();
            if let Some(ms) = sym.first() {
                let st = h.state_at(&[kf, kf], ms, sols.len());
                if h.response_eigenvalue(&st) > 1.02 {
                    assert!(sols.iter().any(|x| (x[0] - x[1]).abs() > 1e-6), "g = {}: eigenvalue {} but no solution with unequal masses: {:?}", g, h.response_eigenvalue(&st), sols);
                    found = true;
                    break;
                }
            }
            g *= 1.1;
        }
        assert!(found, "no strength with an eigenvalue above 1 found");
    }

    #[test]
    fn the_vacuum_of_a_massless_field_acquires_the_mass_of_the_attractive_channel() {
        // D = 3, Lambda = 1: G_c = -pi^2/2 for NU = 8.  Scalar attraction alone, g_0eff = -g:
        // G_S^HF = -15 g/16 (attractive), G_Sc = +g/16 (repulsive).
        let gc = Cjt::critical_coupling(3, 8.0, 1.0);
        for (factor, massive) in [(0.97, false), (1.03, true)] {
            let g = factor * gc.abs() * 16.0 / 15.0;
            let hf = Hf4::from_effective(&[-g, 0.0, 0.0, 0.0]);
            for form in Form::both() {
                let cj = CjtHalves { d: 3, nu_half: 4.0, c: HalvesCouplings::new(form, 0.0, &hf), sea: Some(1.0) };
                let v = cj.vacuum();
                let (ms, _) = v[0];
                if !massive {
                    assert!(ms[0].abs() < 1e-6 && ms[1].abs() < 1e-6, "{:?}: {:?}", form, v);
                } else {
                    assert!(ms[0].abs() > 1e-2, "{:?}: {:?}", form, v);
                    match form {
                        // covariant form: the two halves acquire masses of opposite sign (the mass term of the covariant Lagrangian)
                        Form::C => assert!((ms[0] + ms[1]).abs() < 1e-7),
                        // quantized-sector form: equal masses
                        Form::Q => assert!((ms[0] - ms[1]).abs() < 1e-7),
                    }
                }
            }
        }
    }

    #[test]
    fn the_unified_functional_of_two_halves_reduces_to_the_functional_of_one_kind() {
        use crate::kohn_sham::Unified;
        let f = Fermi::isotropic(3, NU, 96);
        // quantized-sector form with a repulsive coupling of the differences: equal halves; the functional is
        // that of module kohn_sham with G_S = G_S^HF, G_V = G_V^HF
        let hf = Hf4 { gs_hf: -2.0, gv_hf: 0.5, gs_c: 0.3, gv_c: 0.2 };
        let u2 = UnifiedHalves { form: Form::Q, f_d: 0.3, c: 5.0, hf };
        let u1 = Unified { f_d: 0.3, c: 5.0, gs: -2.0, gv: 0.5 };
        for kf in [0.4, 0.8] {
            let a = u2.state(UnifiedHalves::density_of_kf_total(kf)).unwrap();
            let b = u1.state(&f, kf);
            assert!(a.halves.pol.abs() < 1e-9, "polarized: {:?}", a.halves);
            assert!((a.x - b.x).abs() < 1e-5 && (a.eps / b.eps - 1.0).abs() < 1e-9, "kf = {}: x {} against {}, eps {} against {}", kf, a.x, b.x, a.eps, b.eps);
        }
        // without self-interaction both forms give the functional of the Vorton-Scalar alone, also for massless particles (x = 0)
        for form in Form::both() {
            let u2 = UnifiedHalves { form, f_d: 0.0, c: 6.0, hf: Hf4 { gs_hf: 0.0, gv_hf: 0.0, gs_c: 0.0, gv_c: 0.0 } };
            let u1 = Unified { f_d: 0.0, c: 6.0, gs: 0.0, gv: 0.0 };
            for kf in [0.5, 1.2] {
                let a = u2.state(UnifiedHalves::density_of_kf_total(kf)).unwrap();
                let b = u1.state(&f, kf);
                assert!((a.eps / b.eps - 1.0).abs() < 1e-9 && (a.x - b.x).abs() < 1e-4, "{:?} kf = {}: {} {} against {} {}", form, kf, a.x, a.eps, b.x, b.eps);
            }
        }
    }
}
