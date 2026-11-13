//! Pairing in the hidden-axis channel with the contact self-interaction and the exchange
//! of the Vorton-Scalar together.
//!
//! THE PAIR.  The superfluid of the project is a condensate of the pair operator along the
//! hidden axis, psi^T C^- gamma^0 psi.  Two particles with momenta k and -k (in ordinary
//! space x1, x2, x3) and masses M scatter into k', -k' through the interaction.  For a
//! bilinear vertex psi^dagger Gamma psi the amplitude in the pair channel X = C^- gamma^0 is
//!
//!     F(k', k) = Tr[ kappa(k')^dagger Gamma kappa(k) Gamma^T ] / sqrt(N(k') N(k)),
//!     kappa(k) = proj_+(k) conj(X) proj_+(-k)^T,     N(k) = Tr[ kappa(k)^dagger kappa(k) ],
//!
//! with the projector proj_+(k) = (1 + h(k)/E)/2 on the states of positive energy of
//! the single-particle Hamiltonian h(k) = alpha.k + beta M, E = sqrt(k^2 + M^2).
//!
//! THE FOUR CONTACT CHANNELS.  The self-interaction L_self = -(1/2) sum_r g_r I_r has four
//! independent couplings for anticommuting fields (module `selfint`): g_0eff (scalar),
//! g_1eff (vector), g_7eff (axial vector), g_8eff (pseudoscalar).  Summed over the index
//! sets of each rank, with the signs eps_K, the amplitudes are (checks pc-P1, pc-P2)
//!
//!     F_0 = (E E' + M^2 - k.k') / (2 E E') ,            F_8 = -(E E' - M^2 - k.k') / (2 E E') ,
//!     F_1 = (3 E E' - 2 M^2 - k.k') / (E E') ,          F_7 = (3 E E' + 2 M^2 - k.k') / (E E') ,
//!
//! and the pairing potential of the contact interaction is
//!
//!     V_contact(k', k) = g_0eff F_0 + g_1eff F_1 + g_7eff F_7 + g_8eff F_8      (attractive where negative).
//!
//! For two particles at rest F = (1, 1, 5, 0): G_pair = g_0eff + g_1eff + 5 g_7eff, the
//! exact value of `selfint` (check ..-S9).  F_0 is the amplitude F_S of the exchange of
//! the radial mode of the Vorton-Scalar, and -F_8 the amplitude F_P of the exchange of
//! its phase mode, which the main project uses in the forms
//! F_S = [M^2 (E+E')^2 + E^2 k'^2 + E'^2 k^2 - 2 E E' k.k']/(4 E^2 E'^2) and
//! F_P = [(M^2 - E E')^2 + 2 (M^2 - E E') k.k' + M^2 q^2 + k^2 k'^2]/(4 E^2 E'^2); they are
//! the same functions.
//!
//! THE VORTON-SCALAR EXCHANGE IS A SELF-INTERACTION OF FINITE RANGE.  The exchange of the
//! radial mode (mass m_s) and of the phase mode (mass m_ph) gives the pairing potential
//!
//!     V_VS(k', k) = -g_VS^2 [ F_S/(q^2 + m_s^2) + F_P/(q^2 + m_ph^2) ],      q = k' - k,
//!
//! which is V_contact with the momentum-dependent couplings
//!
//!     g_0eff(q) = -g_VS^2/(q^2 + m_s^2) ,        g_8eff(q) = +g_VS^2/(q^2 + m_ph^2) .
//!
//! At momenta far below m_s the radial mode is the scalar contact coupling
//! g_0 = -g_VS^2/m_s^2.  In the phase with unbroken symmetry m_ph = m_s and
//! g_0eff(q) + g_8eff(q) = 0, which is the condition of chiral invariance of the contact
//! interaction (check 71s-S10).
//!
//! THE TWO FORMS ON 4+4, AND THE TWO KINDS OF PAIRS.  The closed forms above hold for the
//! bilinears psibar Theta psi of the 7+1 model and of the quantized-sector form Q of the 4+4
//! model, in the hidden-axis channel X = C^- gamma^0 and, by the internal symmetry of
//! these two models, equally in the pairs along the deflating coordinates, X = C^- gamma^t
//! (t = 5, 6, 7).  In the covariant form C of the 4+4 model (the DEFINITION in this
//! folder) the matrix W = i C^- gamma^4 stands in every bilinear, and W commutes with the
//! Hamiltonian of a particle whose momentum lies in x0..x3.  The pair along the hidden
//! axis consists of one particle of each half W = +-1 of the field (W kappa W^T = -kappa),
//! the pairs along x5, x6, x7 of two particles of the same half (W kappa W^T = +kappa).
//! Therefore, at all momenta (checks pc-P5),
//!
//!     form C, pair along a deflating coordinate:   F_r(C) = +F_r      (as in form Q),
//!     form C, pair along the hidden axis:          F_r(C) = -F_r      (opposite sign).
//!
//! The field `ct_sign` of `PairKernel` is this sign.  The exchange of the Vorton-Scalar is
//! not affected: its Yukawa coupling contains psibar psi and psibar i chi psi, in which
//! the two halves enter alike, in both forms.
//!
//! THE RANGE OF THE CONTACT INTERACTION.  A contact interaction in the gap equation needs
//! a regularization.  Here each contact coupling is given the range 1/m_ct of an
//! exchanged heavy particle of mass m_ct, g -> g m_ct^2/(q^2 + m_ct^2), the form that the
//! exchange of the Vorton-Scalar itself has; m_ct is a parameter of the model (the scale
//! above which the contact interaction is resolved; the field `lam` of `PairKernel`).
//!
//! THE GAP EQUATION is that of the main project (routine `spacetimes::gap::solve_gap`),
//!
//!     Delta(k) = (1/(4 pi^2)) Int_0^inf dk' k'^2 W(k, k') Delta(k') tanh(E_q'/(2T)) / E_q' ,
//!     W(k, k') = (1/2) Int_{-1}^{1} dcos(theta) [ -V(k, k') ] ,   E_q = sqrt(xi^2 + Delta^2),  xi = sqrt(k^2 + M^2) - mu ,
//!
//! with the kernel of this module.  It is the stationarity condition of the CJT effective
//! action with respect to the anomalous propagator in the Hartree-Fock-Bogoliubov
//! (double-bubble) truncation, and equally the Bogoliubov-de Gennes form of the Kohn-Sham
//! equations with a pairing field.
//!
//! UNITS: hbar = c = k_B = 1.

use crate::selfint::{Algebra, Model};
use spacetimes::gap::{angular_average, Kernel};
use spacetimes::complex::C64;
use spacetimes::kernel::{conj, dagger, mul, scale, trace, transpose, Mat, OneParticle};
use spacetimes::gamma::Gammas;

/// The four independent contact channels: ranks 0, 1, 7, 8.
pub const RANKS4: [usize; 4] = [0, 1, 7, 8];

/// The coefficients (a, b, den) of F = (a + b cos(theta))/den for the contact channel
/// number j (0: scalar, 1: vector, 2: axial vector, 3: pseudoscalar), for particles of
/// mass m with momenta of magnitudes k, kp.
pub fn amp_coefficients(j: usize, m: f64, k: f64, kp: f64) -> (f64, f64, f64) {
    let (e, ep) = ((m * m + k * k).sqrt(), (m * m + kp * kp).sqrt());
    match j {
        0 => (e * ep + m * m, -k * kp, 2.0 * e * ep),
        1 => (3.0 * e * ep - 2.0 * m * m, -k * kp, e * ep),
        2 => (3.0 * e * ep + 2.0 * m * m, -k * kp, e * ep),
        3 => (-(e * ep - m * m), k * kp, 2.0 * e * ep),
        _ => panic!("amp_coefficients: channel {}", j),
    }
}

/// The amplitude F_j(k', k) for the angle theta between k and k' (mu = cos theta).
pub fn amp_contact(j: usize, m: f64, k: f64, kp: f64, mu: f64) -> f64 {
    let (a, b, den) = amp_coefficients(j, m, k, kp);
    (a + b * mu) / den
}

/// The pairing kernel of the hidden-axis channel.
#[derive(Clone, Copy, Debug)]
pub struct PairKernel {
    /// mass of the particles (in matter: the Kohn-Sham mass)
    pub m: f64,
    pub k_f: f64,
    /// g_VS^2 of the Vorton-Scalar (0: no exchange)
    pub g2: f64,
    /// mass of the radial mode
    pub m_s: f64,
    /// mass of the phase mode
    pub m_ph: f64,
    /// include the exchange of the phase mode
    pub phase: bool,
    /// the four contact couplings (g_0eff, g_1eff, g_7eff, g_8eff)
    pub contact: [f64; 4],
    /// the range parameter m_ct of the contact couplings (f64::INFINITY: no form factor)
    pub lam: f64,
    /// +1: the contact couplings act with the sign of the closed forms (7+1; form Q of 4+4 in
    /// every channel; form C of 4+4 in a pair of two particles of the same half, such as the
    /// pairs along x5, x6, x7).  -1: form C of 4+4 in a pair of particles of different halves,
    /// such as the pair along the hidden axis.
    pub ct_sign: f64,
}

impl PairKernel {
    pub fn energy(&self, k: f64) -> f64 {
        (self.m * self.m + k * k).sqrt()
    }
    pub fn mu(&self) -> f64 {
        self.energy(self.k_f)
    }
    /// The part of W that comes from the exchange of the Vorton-Scalar.
    pub fn w_exchange(&self, k: f64, kp: f64) -> f64 {
        if self.g2 == 0.0 {
            return 0.0;
        }
        let d = 2.0 * k * kp;
        let (a, b, den) = amp_coefficients(0, self.m, k, kp);
        let mut w = angular_average(a, b, k * k + kp * kp + self.m_s * self.m_s, d) / den;
        if self.phase {
            // F_P = -F_8
            let (a, b, den) = amp_coefficients(3, self.m, k, kp);
            w += angular_average(-a, -b, k * k + kp * kp + self.m_ph * self.m_ph, d) / den;
        }
        self.g2 * w
    }
    /// The part of W that comes from the contact couplings.
    pub fn w_contact(&self, k: f64, kp: f64) -> f64 {
        let mut w = 0.0;
        for j in 0..4 {
            if self.contact[j] == 0.0 {
                continue;
            }
            let (a, b, den) = amp_coefficients(j, self.m, k, kp);
            let avg = if self.lam.is_infinite() { a } else { self.lam * self.lam * angular_average(a, b, k * k + kp * kp + self.lam * self.lam, 2.0 * k * kp) };
            w -= self.ct_sign * self.contact[j] * avg / den;
        }
        w
    }
    /// Density of states at the Fermi surface per pair of internal states, N(0) = k_F E_F/(2 pi^2).
    pub fn n0(&self) -> f64 {
        self.k_f * self.mu() / (2.0 * std::f64::consts::PI * std::f64::consts::PI)
    }
    /// The dimensionless coupling lambda = N(0) W(k_F, k_F).
    pub fn lambda(&self) -> f64 {
        self.n0() * Kernel::w(self, self.k_f, self.k_f)
    }
}

impl Kernel for PairKernel {
    fn w(&self, k: f64, kp: f64) -> f64 {
        self.w_exchange(k, kp) + self.w_contact(k, kp)
    }
    fn xi(&self, k: f64) -> f64 {
        self.energy(k) - self.mu()
    }
    fn k_f(&self) -> f64 {
        self.k_f
    }
}

// --------------------------------------------------------------------------------------
// direct evaluation with the 16x16 matrices (for the checks)
// --------------------------------------------------------------------------------------

fn g16_to_mat(g: &crate::selfint::G16) -> Mat {
    (0..16).map(|i| (0..16).map(|j| C64::new(g.re[i * 16 + j] as f64, g.im[i * 16 + j] as f64)).collect()).collect()
}

/// The matrices needed to evaluate the amplitudes of all nine ranks directly.
pub struct DirectAmplitudes {
    op: OneParticle,
    /// (rank, eps_K, M_K) for the 256 channels
    chan: Vec<(usize, f64, Mat)>,
    /// the pair matrix X = C^- gamma^axis (real)
    x: Mat,
}

impl DirectAmplitudes {
    /// `model`: `S71`, `S44Q` or `S44C`; `axis`: the pair operator is psi^T C^- gamma^axis psi
    /// (0: the hidden axis; 5, 6, 7: a deflating coordinate).
    pub fn new(gm: &Gammas, model: Model, axis: usize) -> DirectAmplitudes {
        let op = match model {
            Model::S71 => OneParticle::new_1671(gm),
            _ => OneParticle::new_1644(gm),
        };
        let al = Algebra::new(gm, model);
        let chan = al.chan.iter().map(|c| (c.rank, c.eps as f64, g16_to_mat(&c.m))).collect();
        let x = op.pair_vector(axis);
        DirectAmplitudes { op, chan, x }
    }

    /// The projector on the states of positive energy, (1 + h(p)/E)/2.
    fn projector(&self, p: &[f64; 8], m: f64) -> Mat {
        let h = self.op.hamiltonian(p, m);
        let e = (m * m + p.iter().map(|x| x * x).sum::<f64>()).sqrt();
        let mut out = scale(&h, C64::new(0.5 / e, 0.0));
        for i in 0..16 {
            out[i][i] = out[i][i] + C64::new(0.5, 0.0);
        }
        out
    }

    /// kappa(p) = proj_+(p) conj(X) proj_+(-p)^T.
    fn kappa(&self, p: &[f64; 8], m: f64) -> Mat {
        let mut pm = *p;
        for x in pm.iter_mut() {
            *x = -*x;
        }
        mul(&mul(&self.projector(p, m), &conj(&self.x)), &transpose(&self.projector(&pm, m)))
    }

    /// The amplitudes of the nine ranks, sum_{K of rank r} eps_K Tr[kappa'^dagger M_K kappa M_K^T]/sqrt(N N').
    pub fn amplitudes(&self, p: &[f64; 8], pp: &[f64; 8], m: f64) -> [f64; 9] {
        let (k, kp) = (self.kappa(p, m), self.kappa(pp, m));
        let kpd = dagger(&kp);
        let n = (trace(&mul(&dagger(&k), &k)).re * trace(&mul(&kpd, &kp)).re).sqrt();
        let mut out = [0.0; 9];
        for (r, eps, mk) in &self.chan {
            let t = trace(&mul(&mul(&mul(&kpd, mk), &k), &transpose(mk)));
            out[*r] += eps * t.re / n;
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spacetimes::gap::Pairing;

    #[test]
    fn the_closed_forms_agree_with_those_of_the_main_project() {
        for (m, k, kp, mu) in [(1.0, 0.3, 0.7, 0.2), (0.4, 1.3, 0.2, -0.8), (2.0, 0.1, 0.1, 1.0)] {
            assert!((amp_contact(0, m, k, kp, mu) - spacetimes::kernel::f_scalar_hidden(m, k, kp, mu)).abs() < 1e-14);
            assert!((amp_contact(3, m, k, kp, mu) + spacetimes::kernel::f_pseudo_hidden(m, k, kp, mu)).abs() < 1e-14);
        }
        // at rest: (1, 1, 5, 0)
        let at_rest: Vec<f64> = (0..4).map(|j| amp_contact(j, 1.0, 0.0, 0.0, 0.3)).collect();
        assert_eq!(at_rest, vec![1.0, 1.0, 5.0, 0.0]);
    }

    #[test]
    fn without_contact_terms_the_kernel_is_that_of_the_main_project() {
        let main = Pairing { m: 1.0, g: 2.1, m_s: 1.6, m_ph: 0.0, scalar: true, phase: true, phase_sign: 1.0, k_f: 0.5 };
        let here = PairKernel { m: 1.0, k_f: 0.5, g2: 2.1 * 2.1, m_s: 1.6, m_ph: 0.0, phase: true, contact: [0.0; 4], lam: f64::INFINITY, ct_sign: 1.0 };
        for (k, kp) in [(0.5, 0.5), (0.1, 2.0), (3.0, 0.4), (0.5, 0.500001)] {
            let (a, b) = (Kernel::w(&main, k, kp), Kernel::w(&here, k, kp));
            assert!((a / b - 1.0).abs() < 1e-12, "{} against {}", a, b);
        }
    }

    #[test]
    fn the_radial_mode_is_the_scalar_contact_channel_with_the_range_of_the_scalar() {
        let (g2, m_s) = (3.7, 1.9);
        let exchange = PairKernel { m: 1.0, k_f: 0.6, g2, m_s, m_ph: 0.0, phase: false, contact: [0.0; 4], lam: f64::INFINITY, ct_sign: 1.0 };
        let contact = PairKernel { m: 1.0, k_f: 0.6, g2: 0.0, m_s, m_ph: 0.0, phase: false, contact: [-g2 / (m_s * m_s), 0.0, 0.0, 0.0], lam: m_s, ct_sign: 1.0 };
        for (k, kp) in [(0.6, 0.6), (0.2, 1.5), (4.0, 0.3)] {
            let (a, b) = (Kernel::w(&exchange, k, kp), Kernel::w(&contact, k, kp));
            assert!((a / b - 1.0).abs() < 1e-13, "{} against {}", a, b);
        }
    }

    #[test]
    fn direct_evaluation_with_the_matrices_gives_the_closed_forms() {
        let gm = Gammas::build();
        // (model, axis of the pair, sign of the amplitudes relative to the closed forms)
        for (model, axis, sign) in [(Model::S71, 0, 1.0), (Model::S44Q, 0, 1.0), (Model::S44Q, 5, 1.0), (Model::S71, 6, 1.0), (Model::S44C, 5, 1.0), (Model::S44C, 7, 1.0), (Model::S44C, 0, -1.0)] {
            let d = DirectAmplitudes::new(&gm, model, axis);
            let m = 0.8;
            // momenta in ordinary space (x1, x2, x3)
            let p = [0.0, 0.3, -0.2, 0.5, 0.0, 0.0, 0.0, 0.0];
            let pp = [0.0, -0.6, 0.1, 0.2, 0.0, 0.0, 0.0, 0.0];
            let f = d.amplitudes(&p, &pp, m);
            let (k, kp) = ((0.09f64 + 0.04 + 0.25).sqrt(), (0.36f64 + 0.01 + 0.04).sqrt());
            let mu = (-0.18 - 0.02 + 0.10) / (k * kp);
            for (j, &r) in RANKS4.iter().enumerate() {
                assert!((f[r] - sign * amp_contact(j, m, k, kp, mu)).abs() < 1e-12, "{:?} axis {} rank {}: {} against {}", model, axis, r, f[r], sign * amp_contact(j, m, k, kp, mu));
            }
        }
    }
}
