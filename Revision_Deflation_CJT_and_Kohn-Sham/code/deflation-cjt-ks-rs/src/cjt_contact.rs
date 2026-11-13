//! The effective potential of Cornwall, Jackiw and Tomboulis (CJT) for the Dirac field with
//! mass term and contact self-interaction: uniform matter without pairing.
//!
//! THE METHOD.  The CJT effective action is a functional of the full propagator S of the
//! fermions,
//!
//!     Gamma[S] = -i Tr ln S^{-1} - i Tr (S_0^{-1} S - 1) + Gamma_2[S],
//!
//! where S_0 is the free propagator and Gamma_2 the sum of the two-particle-irreducible
//! vacuum diagrams with the full propagator on their lines.  With the contact interaction
//! L_self = -(1/2) sum_r g_r I_r the lowest order of Gamma_2 consists of the two diagrams
//! with two loops joined at one vertex (the "double bubble"): the direct (Hartree) and the
//! exchange (Fock) diagram.  At this order the stationarity condition
//! d Gamma/d S = 0 says that the self-energy S_0^{-1} - S^{-1} does not depend on the
//! momentum; for uniform matter it is a combination of the unit matrix and of beta
//! (module `selfint`, checks ..-S7).  The propagator is then that of free particles of
//! mass M whose energies are shifted by V_0, and the effective action per unit volume and
//! time becomes an ordinary function of the two variational parameters M and V_0, the
//! effective potential.  At zero temperature and chemical potential mu it is the grand
//! potential per volume
//!
//!     Omega(M, V_0; mu) = Omega_1(M, mu - V_0) - (M - m)^2/(2 G_S) - V_0^2/(2 G_V),
//!
//!     Omega_1(M, mu*)  = - NU Int d^Dk/(2 pi)^D (mu* - w_k) theta(mu* - w_k)                  particles of the Fermi sea
//!                        - NU Int_{|k| < Lambda} d^Dk/(2 pi)^D [ w_k(M) - w_k(m) ]            filled states of negative energy
//!
//! with w_k = sqrt(k^2 + M^2) and the Hartree-Fock couplings G_S = G_0, G_V = G_1.
//! The second line (the "Dirac sea") is the one-loop term -i Tr ln S^{-1} of the vacuum;
//! it needs a cutoff Lambda because the contact interaction in eight dimensions is an
//! effective interaction.  Without it (`sea = None`) the potential is that of the no-sea
//! approximation, and its stationary point must coincide with the Kohn-Sham state: the
//! two methods are then two formulations of the same approximation,
//!
//!     d Omega/d V_0 = 0:   V_0 = G_V rho,
//!     d Omega/d M   = 0:   M = m + G_S (rho_S + rho_S^sea),    rho_S^sea = - NU Int_{|k| < Lambda} d^Dk/(2 pi)^D M/w_k ,
//!
//! Kohn-Sham at fixed density (canonical), CJT at fixed chemical potential (grand
//! canonical).  The numerical routes are independent: the integrals here use Simpson's
//! rule, those of `kohn_sham` Gauss-Legendre quadrature, and the stationary point is found
//! here by minimizing Omega, there by solving the gap equation.
//!
//! WHAT THE DIRAC SEA ADDS (results that the Kohn-Sham calculation in the no-sea
//! approximation cannot give).
//!   * The mass of the fermions in vacuum is not m but the minimum of
//!     Omega_vac(M) = -NU Int^Lambda [w(M) - w(m)] - (M - m)^2/(2 G_S).
//!   * For m = 0 the vacuum generates a mass by itself if the attraction exceeds the
//!     critical value   G_c = -1 / ( NU A_D Lambda^(D-1) / ((2 pi)^D (D - 1)) ),
//!     A_D the area of the unit sphere:
//!         D = 7:  G_c = -90 pi^4/Lambda^6 ,     D = 4:  G_c = -3 pi^2/Lambda^3 ,     D = 3:  G_c = -pi^2/(2 Lambda^2) .
//!     (The value for D = 7 is the one of the author's document DIRAC16COMPLEX.)
//!   * On a deflating spacetime, in the sector of fields that do not depend on x5, x6, x7,
//!     G_S grows as exp(tau_d): the vacuum crosses the critical coupling at the clock
//!     reading tau_c = ln(G_c/G_S(ref)).
//!
//! UNITS: hbar = c = 1.

use crate::kohn_sham::{bisect, golden_min, sphere_area, Contact, PI};

/// Simpson's rule for Int_a^b f with n (even) intervals.
pub fn simpson<F: Fn(f64) -> f64>(f: F, a: f64, b: f64, n: usize) -> f64 {
    let n = n + n % 2;
    let h = (b - a) / n as f64;
    let mut s = f(a) + f(b);
    for i in 1..n {
        s += f(a + h * i as f64) * if i % 2 == 1 { 4.0 } else { 2.0 };
    }
    s * h / 3.0
}

/// k^(d-1) M/sqrt(k^2 + M^2), with the value 0 at k = 0 and M = 0 (d >= 2).
fn scalar_integrand(k: f64, mass: f64, d: i32) -> f64 {
    let w = (k * k + mass * mass).sqrt();
    if w == 0.0 {
        0.0
    } else {
        k.powi(d - 1) * mass / w
    }
}

/// The CJT effective potential of uniform matter in `d` space dimensions.
#[derive(Clone, Copy, Debug)]
pub struct Cjt {
    pub d: usize,
    pub nu: f64,
    pub c: Contact,
    /// None: no-sea approximation.  Some(Lambda): the filled states of negative energy
    /// with momenta below the cutoff Lambda are included.
    pub sea: Option<f64>,
}

/// A stationary point of the effective potential.
#[derive(Clone, Copy, Debug)]
pub struct CjtState {
    pub mu: f64,
    pub mass: f64,
    pub v0: f64,
    pub rho: f64,
    /// scalar density of the particles of the Fermi sea
    pub rho_s: f64,
    /// scalar density of the filled states of negative energy (0 without sea)
    pub rho_s_sea: f64,
    /// the grand potential per volume at the stationary point (minus the pressure)
    pub omega: f64,
    /// energy density Omega + mu rho, measured from the vacuum of mass m
    pub eps: f64,
}

const N_SIMPSON: usize = 4000;

impl Cjt {
    fn pref(&self) -> f64 {
        self.nu * sphere_area(self.d) / (2.0 * PI).powi(self.d as i32)
    }

    /// Fermi momentum of free particles of mass M at the chemical potential mu*.
    pub fn kf(mass: f64, mu_star: f64) -> f64 {
        if mu_star > mass.abs() {
            (mu_star * mu_star - mass * mass).sqrt()
        } else {
            0.0
        }
    }

    /// The density of the free Fermi gas at mass M and chemical potential mu* (closed form).
    pub fn rho_of(&self, mass: f64, mu_star: f64) -> f64 {
        let kf = Cjt::kf(mass, mu_star);
        self.pref() * kf.powi(self.d as i32) / self.d as f64
    }

    /// (rho, rho_S, pressure of the free Fermi gas) at mass M and chemical potential mu*.
    pub fn fermi_sea(&self, mass: f64, mu_star: f64) -> (f64, f64, f64) {
        let kf = Cjt::kf(mass, mu_star);
        if kf == 0.0 {
            return (0.0, 0.0, 0.0);
        }
        let d = self.d as i32;
        let pref = self.pref();
        let rho = pref * kf.powi(d) / self.d as f64;
        let rho_s = pref * simpson(|k| scalar_integrand(k, mass, d), 0.0, kf, N_SIMPSON);
        let p = pref * simpson(|k| k.powi(d - 1) * (mu_star - (k * k + mass * mass).sqrt()), 0.0, kf, N_SIMPSON);
        (rho, rho_s, p)
    }

    /// The energy of the filled states of negative energy relative to its value at M = m:
    /// -NU Int_{|k| < Lambda} d^Dk/(2 pi)^D [w_k(M) - w_k(m)]  (0 without sea).
    pub fn sea_energy(&self, mass: f64) -> f64 {
        match self.sea {
            None => 0.0,
            Some(lam) => {
                let d = self.d as i32;
                let m = self.c.m;
                -self.pref() * simpson(|k| k.powi(d - 1) * ((k * k + mass * mass).sqrt() - (k * k + m * m).sqrt()), 0.0, lam, N_SIMPSON)
            }
        }
    }

    /// The scalar density of the filled states of negative energy, -NU Int^Lambda M/w.
    pub fn sea_rho_s(&self, mass: f64) -> f64 {
        match self.sea {
            None => 0.0,
            Some(lam) => {
                let d = self.d as i32;
                -self.pref() * simpson(|k| scalar_integrand(k, mass, d), 0.0, lam, N_SIMPSON)
            }
        }
    }

    /// The effective potential Omega(M, V_0; mu).
    pub fn omega(&self, mass: f64, v0: f64, mu: f64) -> f64 {
        let (_, _, p) = self.fermi_sea(mass, mu - v0);
        let mut om = -p + self.sea_energy(mass);
        if self.c.gs != 0.0 {
            om -= (mass - self.c.m).powi(2) / (2.0 * self.c.gs);
        }
        if self.c.gv != 0.0 {
            om -= v0 * v0 / (2.0 * self.c.gv);
        }
        om
    }

    /// The value of V_0 at which Omega is stationary for given M: V_0 = G_V rho(M, mu - V_0).
    /// Requires G_V >= 0 (then the solution is unique: the stationary point is a maximum in V_0).
    pub fn v0_of(&self, mass: f64, mu: f64) -> f64 {
        assert!(self.c.gv >= 0.0, "Cjt::v0_of: the nested solution needs G_V >= 0");
        if self.c.gv == 0.0 || mu <= mass.abs() {
            return 0.0;
        }
        let hi = mu - mass.abs();
        bisect(|v| v - self.c.gv * self.rho_of(mass, mu - v), 0.0, hi)
    }

    /// Omega as a function of M alone, V_0 eliminated.
    pub fn omega_reduced(&self, mass: f64, mu: f64) -> f64 {
        self.omega(mass, self.v0_of(mass, mu), mu)
    }

    /// An upper bound for the mass at the minimum.
    fn mass_bound(&self, mu: f64) -> f64 {
        let mut b = self.c.m.max(mu);
        if let Some(lam) = self.sea {
            // |G_S rho_S^sea| <= |G_S| NU V_D Lambda^D/(2 pi)^D
            b += self.c.gs.abs() * self.pref() * lam.powi(self.d as i32) / self.d as f64;
        }
        1.05 * b + 1e-9
    }

    /// d Omega/d M at stationary V_0 (envelope theorem):
    /// rho_S(M, mu - V_0(M)) + rho_S^sea(M) - (M - m)/G_S.   Requires G_S != 0.
    pub fn gap_residual(&self, mass: f64, mu: f64) -> f64 {
        let v0 = self.v0_of(mass, mu);
        self.fermi_sea(mass, mu - v0).1 + self.sea_rho_s(mass) - (mass - self.c.m) / self.c.gs
    }

    fn state_at(&self, mass: f64, mu: f64) -> CjtState {
        let v0 = self.v0_of(mass, mu);
        let (rho, rho_s, _) = self.fermi_sea(mass, mu - v0);
        let omega = self.omega(mass, v0, mu);
        CjtState { mu, mass, v0, rho, rho_s, rho_s_sea: self.sea_rho_s(mass), omega, eps: omega + mu * rho }
    }

    /// Refine a minimum found by comparison of values (accurate to about the square root of
    /// the rounding error) with the gap equation d Omega/d M = 0 (accurate to rounding error).
    fn polish(&self, mass: f64, mu: f64, h: f64) -> f64 {
        let (a, b) = ((mass - h).max(0.0), mass + h);
        let (fa, fb) = (self.gap_residual(a, mu), self.gap_residual(b, mu));
        if fa * fb < 0.0 {
            bisect(|x| self.gap_residual(x, mu), a, b)
        } else {
            mass
        }
    }

    /// The stable state at the chemical potential mu: the minimum of Omega over M (G_S <= 0),
    /// with V_0 at its stationary value.  (At the chemical potential of saturated matter the
    /// vacuum and the matter have the same Omega: the two phases coexist.)
    pub fn stationary(&self, mu: f64) -> CjtState {
        assert!(self.c.gs <= 0.0, "Cjt::stationary: the minimization needs G_S <= 0");
        let mass = if self.c.gs == 0.0 {
            self.c.m
        } else {
            let hi = self.mass_bound(mu);
            let n = 400;
            let mut best = (f64::INFINITY, 0.0);
            for i in 0..=n {
                let x = hi * i as f64 / n as f64;
                let o = self.omega_reduced(x, mu);
                if o < best.0 {
                    best = (o, x);
                }
            }
            let h = hi / n as f64;
            let x = golden_min(|x| self.omega_reduced(x, mu), (best.1 - h).max(0.0), best.1 + h, 1e-9 * hi.max(1.0));
            self.polish(x, mu, 1e-4 * hi.max(1.0))
        };
        self.state_at(mass, mu)
    }

    /// The stationary point of Omega nearest to the mass `guess` (a local minimum, a
    /// maximum or a point of inflection in M: every solution of the gap equation, stable
    /// or not).  Returns None if the gap equation has no solution within `width` of the guess.
    pub fn stationary_near(&self, mu: f64, guess: f64, width: f64) -> Option<CjtState> {
        if self.c.gs == 0.0 {
            return Some(self.state_at(self.c.m, mu));
        }
        let (a, b) = ((guess - width).max(0.0), guess + width);
        let (fa, fb) = (self.gap_residual(a, mu), self.gap_residual(b, mu));
        if fa * fb > 0.0 {
            return None;
        }
        Some(self.state_at(bisect(|x| self.gap_residual(x, mu), a, b), mu))
    }

    /// The residuals of the two stationarity conditions at a state:
    /// (V_0 - G_V rho,  M - m - G_S (rho_S + rho_S^sea)).
    pub fn residuals(&self, st: &CjtState) -> (f64, f64) {
        (st.v0 - self.c.gv * st.rho, st.mass - self.c.m - self.c.gs * (st.rho_s + st.rho_s_sea))
    }

    /// The mass of the fermions in vacuum (mu = 0).  Without sea it is m.
    pub fn vacuum_mass(&self) -> f64 {
        if self.sea.is_none() {
            return self.c.m;
        }
        self.stationary(0.0).mass
    }

    /// The critical coupling of a massless field: for G_S < G_c < 0 the vacuum generates a mass.
    pub fn critical_coupling(d: usize, nu: f64, lam: f64) -> f64 {
        -1.0 / (nu * sphere_area(d) * lam.powi(d as i32 - 1) / ((2.0 * PI).powi(d as i32) * (d as f64 - 1.0)))
    }

    /// The bare mass m for which the mass in vacuum has the value `m_vac` (with sea):
    /// m = m_vac - G_S rho_S^sea(m_vac).
    pub fn bare_mass_for(d: usize, nu: f64, gs: f64, lam: f64, m_vac: f64) -> f64 {
        let probe = Cjt { d, nu, c: Contact { m: 0.0, gs, gv: 0.0 }, sea: Some(lam) };
        m_vac - gs * probe.sea_rho_s(m_vac)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kohn_sham::{chemical_potential, ks_solve, Fermi, NU};

    #[test]
    fn simpson_is_exact_for_cubics() {
        let x = simpson(|t| t * t * t - 2.0 * t + 1.0, 0.0, 2.0, 10);
        assert!((x - (4.0 - 4.0 + 2.0)).abs() < 1e-14);
    }

    #[test]
    fn without_sea_the_cjt_minimum_is_the_kohn_sham_state() {
        // Fermi momenta above saturation, where the pressure is positive and uniform matter is the stable phase
        for (d, gs, gv, kf) in [(7usize, -1.0e5, 1.0e4, 0.85), (4, -120.0, 14.0, 0.9), (3, -9.0, 2.0, 1.1)] {
            let f = Fermi::isotropic(d, NU, 96);
            let c = Contact { m: 1.0, gs, gv };
            let ks = ks_solve(&f, &c, kf, &[1.0; 3]);
            assert!(ks.p_flat > 0.0, "D = {}: choose a Fermi momentum above saturation", d);
            let mu = chemical_potential(&c, kf, &ks);
            let cjt = Cjt { d, nu: NU, c, sea: None };
            let st = cjt.stationary(mu);
            assert!((st.mass - ks.mass).abs() < 1e-9, "D = {}: M = {} against {}", d, st.mass, ks.mass);
            assert!((st.rho / ks.rho - 1.0).abs() < 1e-9 && (st.eps / ks.eps - 1.0).abs() < 1e-9, "D = {}", d);
            // minus Omega is the pressure
            assert!((-st.omega / ks.p_flat - 1.0).abs() < 1e-8, "D = {}: {} against {}", d, -st.omega, ks.p_flat);
        }
    }

    #[test]
    fn at_saturation_matter_and_vacuum_coexist() {
        // the author's example at its saturation point: the stationary point near the Kohn-Sham mass has Omega = 0 = Omega of the vacuum
        let c = Contact { m: 1.0, gs: -1.0e5, gv: 1.0e4 };
        let f = Fermi::isotropic(7, NU, 96);
        let kf = 0.756636150701493;
        let ks = ks_solve(&f, &c, kf, &[1.0; 3]);
        let mu = chemical_potential(&c, kf, &ks);
        let cjt = Cjt { d: 7, nu: NU, c, sea: None };
        let st = cjt.stationary_near(mu, ks.mass, 0.05).unwrap();
        assert!((st.mass - 0.348097340146).abs() < 1e-10, "M = {}", st.mass);
        assert!(st.omega.abs() < 1e-12 * ks.eps, "Omega = {}", st.omega);
        // Hugenholtz-Van Hove: at saturation the chemical potential is the energy per particle
        assert!((mu - ks.e_per_particle).abs() < 1e-10);
    }

    #[test]
    fn critical_couplings_in_closed_form() {
        let lam = 1.7_f64;
        assert!((Cjt::critical_coupling(7, NU, lam) / (-90.0 * PI.powi(4) / lam.powi(6)) - 1.0).abs() < 1e-13);
        assert!((Cjt::critical_coupling(4, NU, lam) / (-3.0 * PI * PI / lam.powi(3)) - 1.0).abs() < 1e-13);
        assert!((Cjt::critical_coupling(3, NU, lam) / (-PI * PI / (2.0 * lam * lam)) - 1.0).abs() < 1e-13);
    }

    #[test]
    fn a_massless_field_acquires_a_mass_beyond_the_critical_coupling() {
        let lam = 2.0;
        let gc = Cjt::critical_coupling(4, NU, lam);
        let below = Cjt { d: 4, nu: NU, c: Contact { m: 0.0, gs: 0.9 * gc, gv: 0.0 }, sea: Some(lam) };
        let above = Cjt { d: 4, nu: NU, c: Contact { m: 0.0, gs: 1.5 * gc, gv: 0.0 }, sea: Some(lam) };
        assert!(below.vacuum_mass() < 1e-5);
        let m = above.vacuum_mass();
        assert!(m > 0.5, "mass {}", m);
        // the gap equation of the vacuum holds: M = -G_S NU Int^Lambda M/w, i.e. M = G_S rho_S^sea(M)
        assert!((m - above.c.gs * above.sea_rho_s(m)).abs() < 1e-10);
    }
}
