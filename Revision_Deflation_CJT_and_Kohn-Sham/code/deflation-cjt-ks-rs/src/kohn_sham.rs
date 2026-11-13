//! Kohn-Sham theory of uniform matter of the 16-component Dirac field with mass term and
//! self-interaction, on flat spacetime and on the deflating spacetimes.
//!
//! THE KOHN-SHAM ANSATZ (the author's document KOHN_SHAM_GRAVITY, adapted).  The
//! interacting ground state of density rho is represented by non-interacting particles
//! that move in the mean fields which the particles themselves produce.  For the contact
//! self-interaction L_self = -(1/2) sum_r g_r I_r and uniform matter the mean fields are
//! two numbers (module `selfint`, checks ..-S7):
//!
//!     M   = m + G_S rho_S       the mass of the particles in matter ("Dirac mass", "effective mass"),
//!     V_0 = G_V rho             a shift of every single-particle energy,
//!
//! with the scalar density rho_S = <psibar psi> and the density rho = <psi^dagger psi>.
//! In the Hartree approximation G_S = g_0 and G_V = g_1; with the exchange terms
//! (Hartree-Fock) G_S = G_0 and G_V = G_1 are the combinations of all couplings given
//! in `selfint`.  The single-particle energies are  e(k) = sqrt(k^2 + M^2) + V_0.
//!
//! NO-SEA APPROXIMATION: only the particles of the Fermi sea are summed; the filled
//! states of negative energy are omitted (they are treated in `cjt_contact`).
//!
//! THE FERMI SEA.  The field has NU = 8 states of positive energy for every momentum.
//! In D space dimensions with all states below the Fermi momentum k_F filled,
//!
//!     <f> = NU Int_{|k| < k_F} d^D k/(2 pi)^D f(k),
//!     rho = <1>,   rho_S = <M/w>,   e_kin = <w>,   w = sqrt(k^2 + M^2),
//!     energy density   eps = e_kin + (1/2) G_V rho^2 - (M - m)^2/(2 G_S),
//!     pressure along the direction i   p_i = <k_i^2/w> + Pi,   Pi = (1/2) G_V rho^2 + (M - m)^2/(2 G_S).
//!
//! The mass M is the solution of the gap equation M = m + G_S rho_S(M), which is the
//! condition d eps/dM = 0.  (For G_S = 0 the terms with (M - m)^2/G_S are absent and M = m.)
//!
//! ON THE DEFLATING SPACETIMES two sectors are computed.
//!
//!  (a) SECTOR OF ALL MOMENTA, 7+1 only: momenta along the four flat space coordinates
//!      x0, x1, x2, x3 and along the three deflating coordinates x5, x6, x7.  The wave
//!      numbers along the deflating coordinates are conserved (the metric does not depend
//!      on them), so the proper momenta grow: q_t(x4) = s_t q_t(ref), with the stretch
//!      factors s_t = b_t(ref)/b_t(x4) = exp(rate_t (x4 - x4_ref)).  The occupied states,
//!      a ball of radius k_F at the reference time, become an ellipsoid; the density
//!      grows as rho = rho_ref s_5 s_6 s_7 = rho_ref exp(tau_d) with the deflation clock
//!      tau_d = gsum (x4 - x4_ref).  This is collisionless evolution: every particle keeps
//!      its state.  With u the momentum at the reference time (|u| < k_F),
//!
//!          w = sqrt(u_0^2 + u_1^2 + u_2^2 + u_3^2 + s_5^2 u_5^2 + s_6^2 u_6^2 + s_7^2 u_7^2 + M^2),
//!          <f> = NU s_5 s_6 s_7 Int_{|u| < k_F} d^7u/(2 pi)^7 f,
//!          pressures: p_a = <u_a^2/w> + Pi (a = 0..3),   p_t = <s_t^2 u_t^2/w> + Pi (t = 5, 6, 7).
//!
//!      The covariant conservation of energy on the deflating spacetime is
//!          d eps/dx4 = sum_t rate_t (eps + p_t),
//!      and the Kohn-Sham state satisfies it identically (check ks-K4).
//!
//!  (b) SECTOR OF FIELDS THAT DO NOT DEPEND ON x5, x6, x7 (the quantized sector of 4+4;
//!      the low-energy sector of 7+1).  The momenta lie in the D = 4 flat space
//!      dimensions (or D = 3 if the field does not depend on the hidden coordinate x0
//!      either).  The Lagrangian reduced to these fields has the couplings
//!      g_r/(volume of the three deflating coordinates), which grow as exp(tau_d).
//!      Formulas of the Fermi sea with D = 4 or 3 and with G_S, G_V multiplied by exp(tau_d).
//!      The pressure along a deflating coordinate is then Pi alone.
//!
//! THE COVARIANT FORM ON 4+4 (`TwoHalves`): the two halves W = +1 and W = -1 of the field
//! are two kinds of particles with NU/2 = 4 states each, masses M_+- = m + G_S0 rho_S +-
//! G_0 rho_SW and energy shifts G_V0 rho +- G_1 rho_W (check 44c-S7).
//!
//! WITH THE VORTON-SCALAR (`Unified`): the expectation value phi of the scalar field is a
//! second mean field; the mass of the particles is M = m_D + g phi + G_S rho_S, and the
//! potential of the scalar is added to the energy density.
//!
//! UNITS: hbar = c = 1; all quantities in units of the fermion mass m when m = 1.

use spacetimes::quad::gauss_legendre;

pub const PI: f64 = std::f64::consts::PI;

/// Number of states of positive energy of the 16-component field for each momentum.
pub const NU: f64 = 8.0;

/// Gamma function at integers and half-integers: gamma_half(n) = Gamma(n/2).
pub fn gamma_half(n: usize) -> f64 {
    assert!(n >= 1);
    if n == 1 {
        PI.sqrt()
    } else if n == 2 {
        1.0
    } else {
        (n as f64 / 2.0 - 1.0) * gamma_half(n - 2)
    }
}

/// Area of the unit sphere S^(d-1) in d dimensions: 2 pi^(d/2)/Gamma(d/2).
pub fn sphere_area(d: usize) -> f64 {
    2.0 * PI.powf(d as f64 / 2.0) / gamma_half(d)
}

/// Volume of the unit ball in d dimensions.
pub fn ball_volume(d: usize) -> f64 {
    sphere_area(d) / d as f64
}

/// Density of a filled Fermi ball: NU V_D k_F^D/(2 pi)^D.
/// D = 7: k_F^7/(105 pi^4);  D = 4: k_F^4/(4 pi^2);  D = 3: 4 k_F^3/(3 pi^2).
pub fn fermi_density(d: usize, nu: f64, kf: f64) -> f64 {
    nu * ball_volume(d) * kf.powi(d as i32) / (2.0 * PI).powi(d as i32)
}

/// Gauss-Legendre nodes and weights on the interval [a, b].
pub fn gl_interval(a: f64, b: f64, n: usize) -> Vec<(f64, f64)> {
    let (x, w) = gauss_legendre(n);
    (0..n).map(|i| (0.5 * (b - a) * x[i] + 0.5 * (b + a), 0.5 * (b - a) * w[i])).collect()
}

/// One node of the momentum integral over the unit ball (k_F = 1):
/// weight (including the measure and the factor NU/(2 pi)^D), the square of the
/// momentum along the flat directions, and the squares of the three components along
/// the deflating coordinates x5, x6, x7.
#[derive(Clone, Copy, Debug)]
struct Node {
    w: f64,
    a: f64,
    b: [f64; 3],
}

/// The moments of the Fermi sea.
#[derive(Clone, Copy, Debug)]
pub struct Moments {
    pub rho: f64,
    pub rho_s: f64,
    pub e_kin: f64,
    /// kinetic pressure along each flat space direction
    pub p_flat: f64,
    /// kinetic pressure along x5, x6, x7
    pub p_defl: [f64; 3],
}

/// The filled states: a ball of radius k_F in the momenta of the reference time.
#[derive(Clone, Debug)]
pub struct Fermi {
    /// number of flat space dimensions with momentum (4, or 3)
    pub ds: usize,
    /// number of deflating dimensions with momentum (0 or 3)
    pub dd: usize,
    pub nu: f64,
    nodes: Vec<Node>,
}

impl Fermi {
    /// Momenta in d flat dimensions only (flat spacetime, or the sector of fields that do
    /// not depend on the deflating coordinates).  `n` radial Gauss-Legendre nodes.
    pub fn isotropic(d: usize, nu: f64, n: usize) -> Fermi {
        let pref = nu * sphere_area(d) / (2.0 * PI).powi(d as i32);
        let nodes = gl_interval(0.0, 1.0, n).iter().map(|&(k, w)| Node { w: pref * w * k.powi(d as i32 - 1), a: k * k, b: [0.0; 3] }).collect();
        Fermi { ds: d, dd: 0, nu, nodes }
    }

    /// Momenta in `ds` flat dimensions and in the three deflating dimensions.
    /// Quadrature: radius of the flat part (n_flat nodes), radius of the deflating part
    /// (n_r), and the two angles of the deflating part on one octant (n_ang each; the
    /// integrand depends on the squares of the components only).
    pub fn with_deflating(ds: usize, nu: f64, n_flat: usize, n_r: usize, n_ang: usize) -> Fermi {
        let d = ds + 3;
        let pref = nu * sphere_area(ds) * 8.0 / (2.0 * PI).powi(d as i32);
        let mut nodes: Vec<Node> = Vec::with_capacity(n_flat * n_r * n_ang * n_ang);
        let half_pi = 0.5 * PI;
        let th = gl_interval(0.0, half_pi, n_ang);
        let ph = gl_interval(0.0, half_pi, n_ang);
        for &(r, wr) in &gl_interval(0.0, 1.0, n_r) {
            let umax = (1.0 - r * r).sqrt();
            for &(u, wu) in &gl_interval(0.0, umax, n_flat) {
                let w_ru = pref * wr * r * r * wu * u.powi(ds as i32 - 1);
                for &(t, wt) in &th {
                    let (st, ct) = t.sin_cos();
                    for &(p, wp) in &ph {
                        let (sp, cp) = p.sin_cos();
                        let b = [(r * st * cp).powi(2), (r * st * sp).powi(2), (r * ct).powi(2)];
                        nodes.push(Node { w: w_ru * wt * st * wp, a: u * u, b });
                    }
                }
            }
        }
        Fermi { ds, dd: 3, nu, nodes }
    }

    /// As `with_deflating`, for three EQUAL stretch factors: the integrand then depends on the
    /// momentum along the group of three directions only through its magnitude r, the two
    /// angles are integrated in closed form (factor 4 pi), and each of the three directions
    /// carries one third of r^2.  Must not be used with unequal stretch factors.
    pub fn with_isotropic_three(ds: usize, nu: f64, n_flat: usize, n_r: usize) -> Fermi {
        let d = ds + 3;
        let pref = nu * sphere_area(ds) * 4.0 * PI / (2.0 * PI).powi(d as i32);
        let mut nodes: Vec<Node> = Vec::with_capacity(n_flat * n_r);
        for &(r, wr) in &gl_interval(0.0, 1.0, n_r) {
            let umax = (1.0 - r * r).sqrt();
            for &(u, wu) in &gl_interval(0.0, umax, n_flat) {
                nodes.push(Node { w: pref * wr * r * r * wu * u.powi(ds as i32 - 1), a: u * u, b: [r * r / 3.0; 3] });
            }
        }
        Fermi { ds, dd: 3, nu, nodes }
    }

    pub fn dim(&self) -> usize {
        self.ds + self.dd
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// The density of the filled ball at the reference time (closed form).
    pub fn density_ref(&self, kf: f64) -> f64 {
        fermi_density(self.dim(), self.nu, kf)
    }

    /// The scalar density alone (used by the root finder).
    pub fn rho_s(&self, kf: f64, mass: f64, s: &[f64; 3]) -> f64 {
        let k2 = kf * kf;
        let (s5, s6, s7) = (s[0] * s[0] * k2, s[1] * s[1] * k2, s[2] * s[2] * k2);
        let m2 = mass * mass;
        let mut x = 0.0;
        for n in &self.nodes {
            x += n.w / (k2 * n.a + s5 * n.b[0] + s6 * n.b[1] + s7 * n.b[2] + m2).sqrt();
        }
        x * mass * kf.powi(self.dim() as i32) * s[0] * s[1] * s[2]
    }

    /// The moments when the proper momenta along the flat group of directions are stretched
    /// as well, by the common factor `s_flat` (used when the flat directions carry a scale
    /// factor a(x4): s_flat = a_ref/a).  The density then carries the factor s_flat^ds.
    pub fn moments_scaled(&self, kf: f64, mass: f64, s_flat: f64, s: &[f64; 3]) -> Moments {
        let k2 = kf * kf;
        let sf = s_flat * s_flat * k2;
        let sq = [s[0] * s[0] * k2, s[1] * s[1] * k2, s[2] * s[2] * k2];
        let m2 = mass * mass;
        let (mut rho, mut rho_s, mut e_kin, mut p_flat) = (0.0, 0.0, 0.0, 0.0);
        let mut p_defl = [0.0; 3];
        for n in &self.nodes {
            let w = (sf * n.a + sq[0] * n.b[0] + sq[1] * n.b[1] + sq[2] * n.b[2] + m2).sqrt();
            rho += n.w;
            rho_s += n.w * mass / w;
            e_kin += n.w * w;
            p_flat += n.w * sf * n.a / w;
            for t in 0..3 {
                p_defl[t] += n.w * sq[t] * n.b[t] / w;
            }
        }
        let f = kf.powi(self.dim() as i32) * s_flat.powi(self.ds as i32) * if self.dd == 3 { s[0] * s[1] * s[2] } else { 1.0 };
        Moments { rho: rho * f, rho_s: rho_s * f, e_kin: e_kin * f, p_flat: p_flat * f / self.ds as f64, p_defl: [p_defl[0] * f, p_defl[1] * f, p_defl[2] * f] }
    }

    /// All moments for the Fermi momentum k_F (at the reference time), the mass M and
    /// the stretch factors s = (s_5, s_6, s_7) (all 1 at the reference time and in the
    /// sectors without momenta along the deflating coordinates).
    pub fn moments(&self, kf: f64, mass: f64, s: &[f64; 3]) -> Moments {
        let k2 = kf * kf;
        let sq = [s[0] * s[0] * k2, s[1] * s[1] * k2, s[2] * s[2] * k2];
        let m2 = mass * mass;
        let (mut rho, mut rho_s, mut e_kin, mut p_flat) = (0.0, 0.0, 0.0, 0.0);
        let mut p_defl = [0.0; 3];
        for n in &self.nodes {
            let w = (k2 * n.a + sq[0] * n.b[0] + sq[1] * n.b[1] + sq[2] * n.b[2] + m2).sqrt();
            rho += n.w;
            rho_s += n.w * mass / w;
            e_kin += n.w * w;
            p_flat += n.w * k2 * n.a / w;
            for t in 0..3 {
                p_defl[t] += n.w * sq[t] * n.b[t] / w;
            }
        }
        let f = kf.powi(self.dim() as i32) * s[0] * s[1] * s[2];
        Moments { rho: rho * f, rho_s: rho_s * f, e_kin: e_kin * f, p_flat: p_flat * f / self.ds as f64, p_defl: [p_defl[0] * f, p_defl[1] * f, p_defl[2] * f] }
    }
}

/// The parameters of the contact model: bare mass and the two mean-field couplings.
#[derive(Clone, Copy, Debug)]
pub struct Contact {
    /// the mass m of the Dirac Lagrangian
    pub m: f64,
    /// scalar mean-field coupling (Hartree: g_0; Hartree-Fock: G_0); attractive if negative
    pub gs: f64,
    /// vector mean-field coupling (Hartree: g_1; Hartree-Fock: G_1); repulsive if positive
    pub gv: f64,
}

/// A Kohn-Sham state of uniform matter.
#[derive(Clone, Copy, Debug)]
pub struct KsState {
    /// the mass M of the particles in matter
    pub mass: f64,
    pub rho: f64,
    pub rho_s: f64,
    pub e_kin: f64,
    /// energy density
    pub eps: f64,
    /// the interaction term of the pressure, Pi
    pub pi: f64,
    /// pressure along each flat space direction
    pub p_flat: f64,
    /// pressure along x5, x6, x7
    pub p_defl: [f64; 3],
    /// energy per particle eps/rho
    pub e_per_particle: f64,
}

/// Bisection for a root of f on [a, b] with f(a) f(b) <= 0, to the last bit.
pub fn bisect<F: FnMut(f64) -> f64>(mut f: F, mut a: f64, mut b: f64) -> f64 {
    let mut fa = f(a);
    if fa == 0.0 {
        return a;
    }
    let fb = f(b);
    if fb == 0.0 {
        return b;
    }
    assert!(fa * fb < 0.0, "bisect: no sign change on [{}, {}]: f = {}, {}", a, b, fa, fb);
    for _ in 0..200 {
        let c = 0.5 * (a + b);
        if c == a || c == b {
            break;
        }
        let fc = f(c);
        if fc == 0.0 {
            return c;
        }
        if fa * fc < 0.0 {
            b = c;
        } else {
            a = c;
            fa = fc;
        }
    }
    0.5 * (a + b)
}

/// A root of f on [a, b] with f(a) f(b) <= 0: Brent's method (routine of the main
/// project), polished by bisection on the last interval if needed.
pub fn root<F: FnMut(f64) -> f64>(mut f: F, a: f64, b: f64) -> f64 {
    let (fa, fb) = (f(a), f(b));
    if fa == 0.0 {
        return a;
    }
    if fb == 0.0 {
        return b;
    }
    match spacetimes::roots::brent(&mut f, a, b, 1e-15 * (a.abs() + b.abs()).max(1e-300), 200) {
        Ok(x) => x,
        Err(_) => bisect(f, a, b),
    }
}

/// The lowest of the local minima of f on [a, b]: scan on n + 1 equidistant points, then
/// golden-section refinement around every grid point that is not higher than its
/// neighbours.  Returns (position, value).
pub fn global_min<F: FnMut(f64) -> f64>(mut f: F, a: f64, b: f64, n: usize, tol: f64) -> (f64, f64) {
    let h = (b - a) / n as f64;
    let v: Vec<f64> = (0..=n).map(|i| f(a + h * i as f64)).collect();
    let mut best = (a, f64::INFINITY);
    for i in 0..=n {
        let left = if i == 0 { f64::INFINITY } else { v[i - 1] };
        let right = if i == n { f64::INFINITY } else { v[i + 1] };
        if v[i] <= left && v[i] <= right {
            let x0 = a + h * i as f64;
            let x = golden_min(&mut f, (x0 - h).max(a), (x0 + h).min(b), tol);
            let fx = f(x);
            let (x, fx) = if fx <= v[i] { (x, fx) } else { (x0, v[i]) };
            if fx < best.1 {
                best = (x, fx);
            }
        }
    }
    best
}

/// Golden-section search for the minimum of f on [a, b].
pub fn golden_min<F: FnMut(f64) -> f64>(mut f: F, mut a: f64, mut b: f64, tol: f64) -> f64 {
    let gr = 0.5 * (5.0_f64.sqrt() - 1.0);
    let mut c = b - gr * (b - a);
    let mut d = a + gr * (b - a);
    let (mut fc, mut fd) = (f(c), f(d));
    while (b - a).abs() > tol {
        if fc < fd {
            b = d;
            d = c;
            fd = fc;
            c = b - gr * (b - a);
            fc = f(c);
        } else {
            a = c;
            c = d;
            fc = fd;
            d = a + gr * (b - a);
            fd = f(d);
        }
    }
    0.5 * (a + b)
}

/// Solve the gap equation M = m + G_S rho_S(M) and return the Kohn-Sham state.
pub fn ks_solve(f: &Fermi, c: &Contact, kf: f64, s: &[f64; 3]) -> KsState {
    let mass = if c.gs == 0.0 {
        c.m
    } else if c.gs < 0.0 {
        if c.m == 0.0 {
            0.0
        } else {
            // F(M) = M - m - G_S rho_S(M) is increasing; F(0) = -m < 0, F(m) > 0
            root(|x| x - c.m - c.gs * f.rho_s(kf, x, s), 0.0, c.m)
        }
    } else {
        let rho = f.density_ref(kf) * s[0] * s[1] * s[2];
        root(|x| x - c.m - c.gs * f.rho_s(kf, x, s), c.m, c.m + c.gs * rho * 1.0000001 + 1e-300)
    };
    ks_state_at_mass(f, c, kf, s, mass)
}

/// The energy density and the pressures for a given (not necessarily self-consistent) mass.
pub fn ks_state_at_mass(f: &Fermi, c: &Contact, kf: f64, s: &[f64; 3], mass: f64) -> KsState {
    let mo = f.moments(kf, mass, s);
    let scalar_term = if c.gs == 0.0 { 0.0 } else { (mass - c.m) * (mass - c.m) / (2.0 * c.gs) };
    let vector_term = 0.5 * c.gv * mo.rho * mo.rho;
    let eps = mo.e_kin + vector_term - scalar_term;
    let pi = vector_term + scalar_term;
    KsState {
        mass,
        rho: mo.rho,
        rho_s: mo.rho_s,
        e_kin: mo.e_kin,
        eps,
        pi,
        p_flat: mo.p_flat + pi,
        p_defl: [mo.p_defl[0] + pi, mo.p_defl[1] + pi, mo.p_defl[2] + pi],
        e_per_particle: eps / mo.rho,
    }
}

/// Chemical potential of an isotropic Kohn-Sham state: mu = sqrt(k_F^2 + M^2) + G_V rho.
pub fn chemical_potential(c: &Contact, kf: f64, st: &KsState) -> f64 {
    (kf * kf + st.mass * st.mass).sqrt() + c.gv * st.rho
}

/// The saturation point of isotropic matter: the Fermi momentum at which the energy per
/// particle is smallest (the pressure vanishes there).  Searches k_F in [lo, hi].
/// Returns (k_F, state).
pub fn saturation(f: &Fermi, c: &Contact, lo: f64, hi: f64) -> (f64, KsState) {
    let one = [1.0; 3];
    let kf = golden_min(|k| ks_solve(f, c, k, &one).e_per_particle, lo, hi, 1e-10);
    // polish with the condition p = 0 if it changes sign near the minimum
    let p = |k: f64| ks_solve(f, c, k, &one).p_flat;
    let (a, b) = (kf * (1.0 - 1e-4), kf * (1.0 + 1e-4));
    let kf = if p(a) * p(b) < 0.0 { bisect(p, a, b) } else { kf };
    (kf, ks_solve(f, c, kf, &one))
}

/// Square of the speed of sound of isotropic matter at zero temperature,
/// c_s^2 = dp/d eps = (rho/mu) d mu/d rho, by a centered difference in k_F.
pub fn sound_speed_sq(f: &Fermi, c: &Contact, kf: f64) -> f64 {
    let one = [1.0; 3];
    let h = 1e-4 * kf;
    let (a, b) = (ks_solve(f, c, kf - h, &one), ks_solve(f, c, kf + h, &one));
    (b.p_flat - a.p_flat) / (b.eps - a.eps)
}

// --------------------------------------------------------------------------------------
// the covariant form of the interaction on 4+4: two halves of the field
// --------------------------------------------------------------------------------------

/// Mean-field couplings of the covariant form on 4+4 (check 44c-S7).
#[derive(Clone, Copy, Debug)]
pub struct CovariantCouplings {
    pub m: f64,
    /// coupling of rho^2
    pub gv0: f64,
    /// coupling of rho_S^2
    pub gs0: f64,
    /// coupling of rho_W^2 (rho_W = rho_+ - rho_-)
    pub g1: f64,
    /// coupling of rho_SW^2 (rho_SW = rho_S+ - rho_S-)
    pub g0: f64,
}

/// A state of the two halves W = +1 and W = -1.
#[derive(Clone, Copy, Debug)]
pub struct HalvesState {
    /// Fermi momenta of the halves W = +1, W = -1
    pub kf: [f64; 2],
    /// masses of the particles of the two halves
    pub mass: [f64; 2],
    pub rho: [f64; 2],
    pub rho_s: [f64; 2],
    pub eps: f64,
    pub e_per_particle: f64,
    /// whether the iteration of the gap equations converged
    pub converged: bool,
}

/// The two halves in `d` flat dimensions; `fermi` must be an isotropic Fermi sea with
/// NU/2 states (one half).
pub struct TwoHalves<'a> {
    pub fermi: &'a Fermi,
    pub c: CovariantCouplings,
}

impl<'a> TwoHalves<'a> {
    /// Energy density for given Fermi momenta and (trial) masses: the Hartree-Fock energy
    ///   sum over the halves of [ <w> - (M - m) rho_S ]            (the expectation value of the kinetic and mass terms:
    ///                                                              <alpha.k + beta m> = w - (M - m) M/w for a particle of mass M)
    /// + (1/2) [ G_V0 rho^2 + G_S0 rho_S^2 + G_1 rho_W^2 + G_0 rho_SW^2 ] ,
    /// which is stationary at the solution of the two gap equations.
    pub fn energy(&self, kf: &[f64; 2], mass: &[f64; 2]) -> (f64, [f64; 2], [f64; 2]) {
        let one = [1.0; 3];
        let (a, b) = (self.fermi.moments(kf[0], mass[0], &one), self.fermi.moments(kf[1], mass[1], &one));
        let (rho, rho_w) = (a.rho + b.rho, a.rho - b.rho);
        let (rs, rsw) = (a.rho_s + b.rho_s, a.rho_s - b.rho_s);
        let kinetic = a.e_kin - (mass[0] - self.c.m) * a.rho_s + b.e_kin - (mass[1] - self.c.m) * b.rho_s;
        let e = kinetic + 0.5 * (self.c.gv0 * rho * rho + self.c.gs0 * rs * rs + self.c.g1 * rho_w * rho_w + self.c.g0 * rsw * rsw);
        (e, [a.rho, b.rho], [a.rho_s, b.rho_s])
    }

    /// Solve the two gap equations M_+- = m + G_S0 rho_S +- G_0 rho_SW by damped iteration
    /// from the start values `start`.
    pub fn solve(&self, kf: &[f64; 2], start: &[f64; 2]) -> HalvesState {
        let one = [1.0; 3];
        let mut mass = *start;
        let mut converged = false;
        let mut damp = 0.5;
        let mut last = f64::INFINITY;
        for _ in 0..4000 {
            let rs = [if kf[0] > 0.0 { self.fermi.rho_s(kf[0], mass[0], &one) } else { 0.0 }, if kf[1] > 0.0 { self.fermi.rho_s(kf[1], mass[1], &one) } else { 0.0 }];
            let (sum, dif) = (rs[0] + rs[1], rs[0] - rs[1]);
            let new = [self.c.m + self.c.gs0 * sum + self.c.g0 * dif, self.c.m + self.c.gs0 * sum - self.c.g0 * dif];
            let err = (new[0] - mass[0]).abs().max((new[1] - mass[1]).abs());
            if err < 1e-13 {
                converged = true;
                mass = new;
                break;
            }
            if err > last {
                damp *= 0.7;
            }
            last = err;
            mass = [mass[0] + damp * (new[0] - mass[0]), mass[1] + damp * (new[1] - mass[1])];
        }
        let (eps, rho, rho_s) = self.energy(kf, &mass);
        HalvesState { kf: *kf, mass, rho, rho_s, eps, e_per_particle: eps / (rho[0] + rho[1]), converged }
    }

    /// The state of total density `rho_total` and polarization P = (rho_+ - rho_-)/rho.
    pub fn at_polarization(&self, rho_total: f64, pol: f64) -> HalvesState {
        let d = self.fermi.dim();
        let unit = self.fermi.density_ref(1.0);
        let kf_of = |rho: f64| if rho <= 0.0 { 0.0 } else { (rho / unit).powf(1.0 / d as f64) };
        let kf = [kf_of(0.5 * rho_total * (1.0 + pol)), kf_of(0.5 * rho_total * (1.0 - pol))];
        // several start values; keep the converged solution of lowest energy
        let mut best: Option<HalvesState> = None;
        for start in [[self.c.m, self.c.m], [0.5 * self.c.m, 0.5 * self.c.m], [0.1 * self.c.m, self.c.m], [self.c.m, 0.1 * self.c.m], [0.1 * self.c.m, 0.1 * self.c.m], [1.5 * self.c.m, 0.3 * self.c.m], [0.3 * self.c.m, 1.5 * self.c.m]] {
            let st = self.solve(&kf, &start);
            if st.converged && st.mass[0] >= 0.0 && st.mass[1] >= 0.0 && best.as_ref().map_or(true, |b| st.eps < b.eps) {
                best = Some(st);
            }
        }
        best.unwrap_or_else(|| self.solve(&kf, &[self.c.m, self.c.m]))
    }
}

// --------------------------------------------------------------------------------------
// contact interaction together with the Vorton-Scalar field
// --------------------------------------------------------------------------------------

/// The mean-field functional with a bare mass, the contact couplings and the
/// expectation value of the Vorton-Scalar.  Units: the mass of the fermions in vacuum,
/// m = m_D + g_VS v_VS, is 1.
///
///     eps(k_F; x, M) = (1 - f_D)^2 (x^2 - 1)^2/(8 C)                       potential of the scalar, x = phi/v_VS
///                    + e_kin(k_F, M) + (1/2) G_V rho^2 - (M - f_D - (1 - f_D) x)^2/(2 G_S) ,
///
/// f_D = m_D/m the fraction of the vacuum mass that is a bare Dirac mass, and
/// C = g_VS^2 m^2/m_s^2 the coupling of the scalar (m_s: mass of its radial mode).
/// Stationarity in M: M = f_D + (1 - f_D) x + G_S rho_S;  stationarity in x: the field
/// equation of the scalar with the scalar density as its source.
#[derive(Clone, Copy, Debug)]
pub struct Unified {
    pub f_d: f64,
    pub c: f64,
    pub gs: f64,
    pub gv: f64,
}

#[derive(Clone, Copy, Debug)]
pub struct UnifiedState {
    /// phi/v_VS
    pub x: f64,
    pub mass: f64,
    pub rho: f64,
    pub rho_s: f64,
    pub eps: f64,
    pub e_per_particle: f64,
}

impl Unified {
    /// Energy density at given x >= 0 (the gap equation in M solved).
    pub fn energy_at(&self, f: &Fermi, kf: f64, x: f64) -> UnifiedState {
        let m_eff = self.f_d + (1.0 - self.f_d) * x;
        let one = [1.0; 3];
        let st = ks_solve(f, &Contact { m: m_eff, gs: self.gs, gv: self.gv }, kf, &one);
        let u = (1.0 - self.f_d).powi(2) * (x * x - 1.0).powi(2) / (8.0 * self.c);
        let eps = st.eps + u;
        UnifiedState { x, mass: st.mass, rho: st.rho, rho_s: st.rho_s, eps, e_per_particle: eps / st.rho }
    }

    /// The state of lowest energy for the Fermi momentum k_F on 0 <= x <= 1.2 (the range of
    /// the main project: a real expectation value that has not changed its sign).  The
    /// reflection x -> -x is a rotation of the phase of the complex scalar by pi; with a
    /// bare mass the two signs are not equivalent, and that direction is treated with the
    /// phase mode of the scalar, not here.
    pub fn state(&self, f: &Fermi, kf: f64) -> UnifiedState {
        if self.f_d >= 1.0 {
            return self.energy_at(f, kf, 1.0);
        }
        let (x, _) = global_min(|x| self.energy_at(f, kf, x).eps, 0.0, 1.2, 96, 1e-11);
        self.energy_at(f, kf, x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn geometry_of_spheres() {
        assert!((sphere_area(3) - 4.0 * PI).abs() < 1e-14);
        assert!((sphere_area(4) - 2.0 * PI * PI).abs() < 1e-13);
        assert!((sphere_area(7) - 16.0 * PI.powi(3) / 15.0).abs() < 1e-12);
        // densities of the filled ball
        assert!((fermi_density(7, 8.0, 1.0) - 1.0 / (105.0 * PI.powi(4))).abs() < 1e-18);
        assert!((fermi_density(4, 8.0, 1.0) - 1.0 / (4.0 * PI * PI)).abs() < 1e-17);
        assert!((fermi_density(3, 8.0, 1.0) - 4.0 / (3.0 * PI * PI)).abs() < 1e-16);
    }

    #[test]
    fn the_numbers_of_the_author_s_document_are_reproduced() {
        // m = 1, G_S = -1e5, G_V = 1e4, k_F = 0.756636150701493 in seven space dimensions:
        // M = 0.348097340146, rho = 1.388100213e-5, eps = 1.348787233e-5, E/A - m = -0.02832142796
        let f = Fermi::isotropic(7, NU, 96);
        let c = Contact { m: 1.0, gs: -1.0e5, gv: 1.0e4 };
        let kf = 0.756636150701493;
        let st = ks_solve(&f, &c, kf, &[1.0; 3]);
        assert!((st.mass - 0.348097340146).abs() < 2e-12, "M = {}", st.mass);
        assert!((st.rho / 1.388100213e-5 - 1.0).abs() < 1e-9, "rho = {}", st.rho);
        assert!((st.eps / 1.348787233e-5 - 1.0).abs() < 1e-9, "eps = {}", st.eps);
        assert!((st.e_per_particle - 1.0 + 0.02832142796).abs() < 1e-10, "E/A - m = {}", st.e_per_particle - 1.0);
        // it is the saturation point: the pressure vanishes and E/A is smallest
        assert!(st.p_flat.abs() < 1e-14, "p = {}", st.p_flat);
        let (ksat, _) = saturation(&f, &c, 0.4, 1.2);
        assert!((ksat - kf).abs() < 1e-9, "k_F at saturation = {}", ksat);
    }

    #[test]
    fn the_ellipsoid_reduces_to_the_ball_and_conserves_energy() {
        let iso = Fermi::isotropic(7, NU, 96);
        let ani = Fermi::with_deflating(4, NU, 20, 40, 12);
        let c = Contact { m: 1.0, gs: -1.0e5, gv: 1.0e4 };
        let kf = 0.6;
        let a = ks_solve(&iso, &c, kf, &[1.0; 3]);
        let b = ks_solve(&ani, &c, kf, &[1.0; 3]);
        assert!((a.mass - b.mass).abs() < 1e-10 && (a.eps / b.eps - 1.0).abs() < 1e-10 && (a.p_flat / b.p_flat - 1.0).abs() < 1e-8);
        for t in 0..3 {
            assert!((b.p_defl[t] / a.p_flat - 1.0).abs() < 1e-8);
        }
        // d eps/d x4 = sum_t rate_t (eps + p_t) with s_t = exp(rate_t x4)
        let rate = [0.7, 1.0, 1.6];
        let s_of = |x4: f64| [(rate[0] * x4).exp(), (rate[1] * x4).exp(), (rate[2] * x4).exp()];
        let (x4, h) = (0.8, 1e-4);
        let st = ks_solve(&ani, &c, kf, &s_of(x4));
        let d = (ks_solve(&ani, &c, kf, &s_of(x4 + h)).eps - ks_solve(&ani, &c, kf, &s_of(x4 - h)).eps) / (2.0 * h);
        let want: f64 = (0..3).map(|t| rate[t] * (st.eps + st.p_defl[t])).sum();
        assert!((d / want - 1.0).abs() < 1e-7, "{} against {}", d, want);
    }

    #[test]
    fn equal_stretch_factors_allow_the_closed_angular_integral() {
        let full = Fermi::with_deflating(4, NU, 20, 40, 12);
        let fast = Fermi::with_isotropic_three(4, NU, 32, 64);
        let c = Contact { m: 1.0, gs: -1.0e5, gv: 1.0e4 };
        for s in [0.6, 1.0, 2.5] {
            let (a, b) = (ks_solve(&full, &c, 0.7, &[s; 3]), ks_solve(&fast, &c, 0.7, &[s; 3]));
            assert!((a.mass / b.mass - 1.0).abs() < 1e-8 && (a.eps / b.eps - 1.0).abs() < 1e-8 && (a.p_defl[1] / b.p_defl[1] - 1.0).abs() < 1e-7 && (a.p_flat / b.p_flat - 1.0).abs() < 1e-7, "s = {}", s);
        }
    }

    #[test]
    fn the_unified_functional_reduces_to_the_contact_model_and_to_the_scalar_model() {
        let f = Fermi::isotropic(3, NU, 96);
        // (1) no scalar field: f_D = 1 (the whole mass is a bare Dirac mass)
        let u = Unified { f_d: 1.0, c: 1.0, gs: -3.0, gv: 1.0 };
        let a = u.state(&f, 0.8);
        let b = ks_solve(&f, &Contact { m: 1.0, gs: -3.0, gv: 1.0 }, 0.8, &[1.0; 3]);
        assert!((a.mass - b.mass).abs() < 1e-12 && (a.eps - b.eps).abs() < 1e-13);
        // (2) scalar only: the functional of the main project, eps = (x^2 - 1)^2/(8 C) + e_kin(k_F, x)
        let u = Unified { f_d: 0.0, c: 6.0, gs: 0.0, gv: 0.0 };
        let st = u.state(&f, 0.5);
        let med = spacetimes::cjt_normal::Medium { c: 6.0, sea: false };
        let x_main = med.mass(0.5);
        assert!((st.mass - x_main).abs() < 1e-7, "{} against {}", st.mass, x_main);
        assert!((st.eps - med.energy(0.5, x_main)).abs() < 1e-10);
    }
}
