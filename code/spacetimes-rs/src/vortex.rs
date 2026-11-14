//! Fermions bound to a vortex line of the Vorton-Scalar: the zero modes of the
//! sixteen-component field.
//!
//! WHAT IS COMPUTED, AND WHY.
//! The model contains a complex scalar field, the Vorton-Scalar Phi_VS, with the potential
//! lambda_VS (|Phi_VS|^2 - v_VS^2/2)^2 and the Yukawa coupling to the fermion field.  Besides
//! the uniform vacuum Phi_VS = v_VS/sqrt(2) it has VORTEX LINES: along a line the field
//! vanishes, and on a circle around the line its phase winds n times,
//!
//!     Phi_VS = (v_VS/sqrt 2) f_n(rho) exp(i n phi),      rho = m_s r,
//!
//! where (r, phi) are polar coordinates in the plane (x1, x2) perpendicular to the line,
//! m_s = sqrt(2 lambda_VS) v_VS is the mass of the radial mode, and the integer n is the
//! WINDING NUMBER.  The profile f_n rises from 0 on the line to 1 far away.
//!
//! In the sector of fields that do not depend on the extra coordinates x0, x5, x6, x7 the
//! one-particle Hamiltonian in such a background is (kernel.rs; fermion mass m = g_VS v_VS)
//!
//!     h = alpha^1 p_1 + alpha^2 p_2 + alpha^3 p_3 + m f_n(rho) [ beta cos(n phi) + b5 sin(n phi) ],
//!     b5 = i beta chi,
//!
//! with the five Hermitian 16x16 matrices alpha^1, alpha^2, alpha^3, beta, b5 of either
//! spacetime, which have square one and anticommute with one another.  The question is: how
//! many solutions of h psi = E psi are bound to the line with E = 0 at p_3 = 0, and how do
//! they move along the line?  For ONE four-component Dirac field the answer is |n| (Jackiw
//! and Rossi 1981).  The field of this project has sixteen components; this module derives
//! and verifies that it has 4|n| such solutions, 2|n| moving in each direction along the line.
//!
//! THE DERIVATION (every step is a check of `VortexAlgebra::checks` or a unit test).
//! 1. Three matrices organize the problem:
//!        Sigma = -i alpha^1 alpha^2   (twice the spin along the line),
//!        A     =  i beta alpha^1,
//!        S     =  Sigma chi.
//!    All three are Hermitian with square one.  A anticommutes with Sigma; S commutes with A
//!    and with Sigma; alpha^3 commutes with all three.
//! 2. Rotations.  [Sigma, alpha^1] = 2 i alpha^2, [Sigma, alpha^2] = -2 i alpha^1, and
//!    [chi, beta] = 2 i b5, [chi, b5] = -2 i beta.  Hence, with
//!        U(phi) = exp(-i phi (Sigma + n chi)/2),
//!    alpha^1 cos(phi) + alpha^2 sin(phi) = U alpha^1 U^-1 and
//!    beta cos(n phi) + b5 sin(n phi) = U beta U^-1: a rotation of the plane by phi combined
//!    with a rotation of the phase of the scalar by n phi leaves h unchanged.  Because Sigma
//!    and chi have the eigenvalues +1 and -1, U(2 pi) = (-1)^(n+1).
//! 3. Separation.  Write psi = exp(i l phi) U(phi) eta(r) with a constant l, the ANGULAR
//!    MOMENTUM of the channel: l is an integer for odd n and a half-integer for even n (so
//!    that psi is single-valued).  For p_3 = 0 and E = 0 the equation h psi = 0 becomes the
//!    radial equation (the identity is `reduction_residual`, tested for random values)
//!        d eta/dr = [ (l Sigma - (1 + n S)/2)/r + m f A ] eta .
//! 4. On the eigenspace S = s (s = +1 or -1) choose a basis in which A = diag(+1, -1) and
//!    Sigma exchanges the two components, eta = (u, w).  With a = (1 + n s)/2 and
//!    mu = m/m_s (lengths in units of 1/m_s),
//!        du/drho = (-a/rho + mu f) u + (l/rho) w,
//!        dw/drho = (-a/rho - mu f) w + (l/rho) u .
//!    Far from the line (f -> 1) one solution falls as exp(-mu rho) and the other grows.
//!    Near the line (f -> 0) the combinations u + w and u - w behave as rho^(-a + l) and
//!    rho^(-a - l).  A bound solution must fall at infinity and must not diverge on the line;
//!    counting conditions, there is exactly one for each pair (u, w) if both exponents are
//!    non-negative, and none if one of them is negative (they are integers, so that a
//!    negative exponent is at most -1 and the solution is not even square-integrable).
//! 5. Both exponents are non-negative exactly for s = -sign(n) and |l| <= (|n| - 1)/2: these
//!    are |n| channels.  The eigenspace S = s has dimension 8 and consists of four pairs
//!    (u, w): each channel carries 4 zero modes, so that the vortex line has 4|n| in all.
//!    For l = 0 (odd n) the solution is known in closed form: u = 0 and
//!        w = rho^((|n|-1)/2) exp(-mu Int_0^rho f_n) .
//! 6. Motion along the line.  alpha^3 anticommutes with the part of h that acts in the
//!    plane, so that it maps zero modes to zero modes, and with p_3 = k_3 the energy of a
//!    zero mode is alpha^3 k_3 = +k_3 or -k_3: the modes move along the line at the speed of
//!    light.  The product of the five matrices, Pc = i alpha^1 alpha^2 alpha^3 chi, commutes
//!    with all five (it is, up to its sign, the generator P of the symmetry U(1)_E), and
//!    alpha^3 = -S Pc.  On the zero modes of winding n > 0 (S = -1) therefore alpha^3 = Pc:
//!    half of them (Pc = +1) move in the direction +x3 and half (Pc = -1) in the direction
//!    -x3; for n < 0 the directions are exchanged.
//!
//! THE PROFILE.  With rho = m_s r the field equation of the Vorton-Scalar gives
//!        f'' + f'/rho - n^2 f/rho^2 = f (f^2 - 1)/2,     f(0) = 0, f(infinity) = 1,
//! the equation of a vortex of the Gross-Pitaevskii equation.  Near the line
//! f = a_n rho^|n| (1 - rho^2/(8(|n|+1)) + ...); far away
//! f = 1 - n^2/rho^2 - n^2(n^2+8)/(2 rho^4) - [n^2(n^2+8)^2 - n^6/2]/rho^6 - ... .
//! The energy of the line per unit length inside the radius R is
//!        pi v_VS^2 [ n^2 ln(m_s R) + c_n ],
//!        c_n = lim_{R -> infinity} { Int_0^R rho drho [ f'^2 + n^2 f^2/rho^2 + (f^2-1)^2/4 ] - n^2 ln R } .
//! The profile is computed in two independent ways: by Newton's method on a finite-difference
//! grid with Richardson extrapolation (`profile`), and by shooting from the line
//! (`slope_by_shooting`); the two must agree in the slope a_n.
//!
//! All algebra is exact (matrices over the Gaussian integers, `imat::GMat`).

use crate::gamma::Gammas;
use crate::imat::{GMat, IMat};

// ---------------------------------------------------------------------------------------------
// exact algebra
// ---------------------------------------------------------------------------------------------

fn g_identity(n: usize) -> GMat {
    GMat::from_real(&IMat::identity(n))
}

/// `i` times a Gaussian-integer matrix.
fn times_i(m: &GMat) -> GMat {
    GMat { re: m.im.neg(), im: m.re.clone() }
}

fn neg(m: &GMat) -> GMat {
    m.scale(-1)
}

fn is_hermitian(m: &GMat) -> bool {
    *m == m.dagger()
}

fn comm(a: &GMat, b: &GMat) -> GMat {
    a.mul(b).sub(&b.mul(a))
}

fn anti(a: &GMat, b: &GMat) -> GMat {
    a.mul(b).add(&b.mul(a))
}

/// Trace as (real part, imaginary part).
fn g_trace(m: &GMat) -> (i64, i64) {
    (m.re.trace(), m.im.trace())
}

/// The matrices of the vortex problem of one spacetime, exact.
pub struct VortexAlgebra {
    /// "1671" (7+1) or "1644" (4+4, quantized sector).
    pub tag: &'static str,
    /// alpha^1, alpha^2, alpha^3.
    pub alpha: Vec<GMat>,
    /// beta, the matrix of the mass term.
    pub beta: GMat,
    /// b5 = i beta chi, the matrix that multiplies the imaginary part of the scalar.
    pub b5: GMat,
    /// The chirality matrix.
    pub chi: GMat,
    /// A = i beta alpha^1.
    pub a: GMat,
    /// Sigma = -i alpha^1 alpha^2.
    pub sigma: GMat,
    /// S = Sigma chi.
    pub s: GMat,
    /// Pc = i alpha^1 alpha^2 alpha^3 chi.
    pub pc: GMat,
}

impl VortexAlgebra {
    /// `tag` = "1671" or "1644".  Conventions of kernel.rs:
    /// 7+1: beta = gamma^4, alpha^j = gamma^4 gamma^j, chi = i omega;
    /// 4+4: beta = -i gamma^4, alpha^j = -gamma^4 gamma^j, chi = omega,
    /// where omega is the product of the eight Dirac matrices.
    pub fn new(gm: &Gammas, tag: &'static str) -> VortexAlgebra {
        let (beta, alpha, chi): (GMat, Vec<GMat>, GMat) = match tag {
            "1671" => {
                let g = &gm.gamma1671;
                let refs: Vec<&IMat> = g.iter().collect();
                let om = IMat::product(&refs);
                (GMat::from_real(&g[4]), (1..4).map(|j| GMat::from_real(&g[4].mul(&g[j]))).collect(), GMat::i_times(&om))
            }
            "1644" => {
                let g = &gm.gamma1644;
                (GMat::i_times(&g[4].neg()), (1..4).map(|j| GMat::from_real(&g[4].mul(&g[j]).neg())).collect(), GMat::from_real(&gm.t16_8))
            }
            _ => panic!("VortexAlgebra::new: tag must be \"1671\" or \"1644\""),
        };
        let b5 = times_i(&beta.mul(&chi));
        let a = times_i(&beta.mul(&alpha[0]));
        let sigma = neg(&times_i(&alpha[0].mul(&alpha[1])));
        let s = sigma.mul(&chi);
        let pc = times_i(&alpha[0].mul(&alpha[1]).mul(&alpha[2]).mul(&chi));
        VortexAlgebra { tag, alpha, beta, b5, chi, a, sigma, s, pc }
    }

    /// Four times the projector on the spinors with A = sa and S = ss (sa, ss = +1 or -1):
    /// (1 + sa A)(1 + ss S).  (Four times, so that the entries stay Gaussian integers.)
    pub fn projector4(&self, sa: i64, ss: i64) -> GMat {
        let one = g_identity(16);
        one.add(&self.a.scale(sa)).mul(&one.add(&self.s.scale(ss)))
    }

    /// The exact statements of the derivation in the head of this module.
    /// Returns (identifier, statement, true/false).
    pub fn checks(&self) -> Vec<(&'static str, String, bool)> {
        let one = g_identity(16);
        let two_i = |m: &GMat| times_i(m).scale(2);
        let (a1, a2, a3) = (&self.alpha[0], &self.alpha[1], &self.alpha[2]);
        let five = [a1, a2, a3, &self.beta, &self.b5];
        let mut out: Vec<(&'static str, String, bool)> = Vec::new();

        let mut ok = true;
        for i in 0..5 {
            ok &= is_hermitian(five[i]) && five[i].mul(five[i]) == one;
            for j in (i + 1)..5 {
                ok &= anti(five[i], five[j]).is_zero();
            }
        }
        out.push(("V1", "alpha^1, alpha^2, alpha^3, beta and b5 = i beta chi are Hermitian, have square one and anticommute with one another".into(), ok));

        let ok = [&self.a, &self.sigma, &self.s].iter().all(|m| is_hermitian(m) && m.mul(m) == one)
            && anti(&self.a, &self.sigma).is_zero()
            && comm(&self.s, &self.a).is_zero()
            && comm(&self.s, &self.sigma).is_zero()
            && [&self.a, &self.sigma, &self.s, &self.chi].iter().all(|m| comm(a3, m).is_zero());
        out.push(("V2", "Sigma = -i alpha^1 alpha^2, A = i beta alpha^1 and S = Sigma chi are Hermitian with square one; A anticommutes with Sigma; S commutes with A and with Sigma; alpha^3 commutes with A, Sigma, S and chi".into(), ok));

        let ok = comm(&self.sigma, a1) == two_i(a2)
            && comm(&self.sigma, a2) == neg(&two_i(a1))
            && comm(&self.chi, &self.beta) == two_i(&self.b5)
            && comm(&self.chi, &self.b5) == neg(&two_i(&self.beta))
            && comm(&self.sigma, &self.beta).is_zero()
            && comm(&self.sigma, &self.b5).is_zero()
            && comm(&self.chi, a1).is_zero()
            && comm(&self.chi, a2).is_zero()
            && comm(&self.sigma, &self.chi).is_zero();
        out.push(("V3", "rotations: [Sigma, alpha^1] = 2i alpha^2, [Sigma, alpha^2] = -2i alpha^1, [chi, beta] = 2i b5, [chi, b5] = -2i beta; Sigma commutes with beta, b5 and chi; chi commutes with alpha^1 and alpha^2. Hence exp(-i phi (Sigma + n chi)/2) rotates (alpha^1, alpha^2) by phi and (beta, b5) by n phi".into(), ok));

        let ok = times_i(&a1.mul(&self.beta)) == neg(&self.a) && a1.mul(a2) == times_i(&self.sigma);
        out.push(("V4", "the two products that enter the radial equation: i alpha^1 beta = -A and alpha^1 alpha^2 = i Sigma".into(), ok));

        let prod = a1.mul(a2).mul(a3).mul(&self.beta).mul(&self.b5);
        let mut ok = prod == self.pc && is_hermitian(&self.pc) && self.pc.mul(&self.pc) == one && g_trace(&self.pc) == (0, 0);
        for m in five.iter() {
            ok &= comm(&self.pc, m).is_zero();
        }
        ok &= *a3 == neg(&self.s.mul(&self.pc));
        out.push(("V5", "Pc = i alpha^1 alpha^2 alpha^3 chi equals the product of the five matrices, commutes with each of them, is Hermitian and traceless and has square one; and alpha^3 = -S Pc".into(), ok));

        let mut ok = true;
        for sa in [1, -1] {
            for ss in [1, -1] {
                let p4 = self.projector4(sa, ss);
                // (4 P)^2 = 4 (4 P), trace(4 P) = 16: P is a projector of rank 4
                ok &= p4.mul(&p4) == p4.scale(4) && g_trace(&p4) == (16, 0);
                // alpha^3 on it: trace zero and square one, that is, eigenvalues +1, +1, -1, -1
                let pa = p4.mul(a3);
                ok &= g_trace(&pa) == (0, 0) && pa.mul(&pa) == p4.scale(4);
            }
        }
        out.push(("V6", "each of the four joint eigenspaces of A and S has dimension 4, and alpha^3 has the eigenvalues +1, +1, -1, -1 on it".into(), ok));

        // winding +1, l = 0: the constant spinors with A = -1, S = -1
        let pm = self.projector4(-1, -1);
        let b = a2.mul(&self.beta).mul(&self.chi); // the matrix alpha^2 beta chi of the earlier formulation
        let ok = times_i(a1).add(&self.beta).mul(&pm).is_zero()
            && times_i(a2).add(&self.b5).mul(&pm).is_zero()
            && b == neg(&self.a.mul(&self.s))
            && a3.sub(&self.pc).mul(&pm).is_zero();
        out.push(("V7", "winding +1: every constant spinor with A = -1 and S = -1 solves (i alpha^1 + beta) w = 0 and (i alpha^2 + b5) w = 0, the zero-energy equations far from the line along the two axes; alpha^2 beta chi = -A S, so that these are the spinors with (i beta alpha^1) w = -w and (alpha^2 beta chi) w = -w; on them alpha^3 = Pc".into(), ok));

        let pn = self.projector4(-1, 1);
        let ok = times_i(a1).add(&self.beta).mul(&pn).is_zero() && times_i(a2).sub(&self.b5).mul(&pn).is_zero() && a3.add(&self.pc).mul(&pn).is_zero();
        out.push(("V8", "winding -1: the constant spinors with A = -1 and S = +1 solve (i alpha^1 + beta) w = 0 and (i alpha^2 - b5) w = 0, and on them alpha^3 = -Pc: the two directions of motion are exchanged".into(), ok));
        out
    }
}

// ---------------------------------------------------------------------------------------------
// the reduction to the radial equation, as an identity between matrices (double precision)
// ---------------------------------------------------------------------------------------------

/// The identity behind step 3 of the derivation, for arbitrary values.  For constant
/// 16-component vectors `eta` and `deta` (standing for eta(r) and its derivative), a radius
/// `r`, an angle `phi`, a value `f` of the profile, a mass `m`, a winding number `n` and an
/// angular momentum `l`, the function evaluates both sides of
///
///     h_perp [ exp(i l phi) U(phi) eta ] = exp(i l phi) U(phi) (-i alpha^1) { deta - [ (l Sigma - (1 + n S)/2)/r + m f A ] eta },
///
/// where h_perp = -i alpha^1 d/dx1 - i alpha^2 d/dx2 + m f [beta cos(n phi) + b5 sin(n phi)]
/// and U(phi) = exp(-i phi (Sigma + n chi)/2), and returns the largest absolute difference of
/// their components divided by the largest component.  The left-hand side is formed from the
/// definition of h_perp in polar coordinates,
/// h_perp psi = -i alpha_r d psi/dr - i alpha_phi (1/r) d psi/dphi + m f beta_r psi.
#[allow(clippy::too_many_arguments)]
pub fn reduction_residual(v: &VortexAlgebra, eta: &[(f64, f64)], deta: &[(f64, f64)], r: f64, phi: f64, f: f64, m: f64, n: i32, l: f64) -> f64 {
    use crate::complex::C64;
    use crate::kernel::{add, mul, scale, Mat};
    let c = |re: f64, im: f64| C64::new(re, im);
    let to = |g: &GMat| -> Mat { (0..16).map(|i| (0..16).map(|j| c(g.re.get(i, j) as f64, g.im.get(i, j) as f64)).collect()).collect() };
    let col = |x: &[(f64, f64)]| -> Mat { x.iter().map(|z| vec![c(z.0, z.1)]).collect() };
    let one: Mat = to(&g_identity(16));
    let (a1, a2, beta, b5, chi, sigma, s, a) = (to(&v.alpha[0]), to(&v.alpha[1]), to(&v.beta), to(&v.b5), to(&v.chi), to(&v.sigma), to(&v.s), to(&v.a));
    let nf = n as f64;
    // U = [cos(phi/2) - i Sigma sin(phi/2)] [cos(n phi/2) - i chi sin(n phi/2)]   (Sigma and chi commute)
    let rot = |g: &Mat, ang: f64| add(&scale(&one, c((ang / 2.0).cos(), 0.0)), &scale(g, c(0.0, -(ang / 2.0).sin())));
    let u = mul(&rot(&sigma, phi), &rot(&chi, nf * phi));
    let phase = C64::cis(l * phi);
    let eta_c = col(eta);
    let deta_c = col(deta);
    // psi, d psi/dr, d psi/dphi
    let psi = scale(&mul(&u, &eta_c), phase);
    let dpsi_dr = scale(&mul(&u, &deta_c), phase);
    // d/dphi [e^{i l phi} U eta] = e^{i l phi} U [i l - i (Sigma + n chi)/2] eta
    let gen = add(&scale(&one, c(0.0, l)), &scale(&add(&sigma, &scale(&chi, c(nf, 0.0))), c(0.0, -0.5)));
    let dpsi_dphi = scale(&mul(&u, &mul(&gen, &eta_c)), phase);
    let alpha_r = add(&scale(&a1, c(phi.cos(), 0.0)), &scale(&a2, c(phi.sin(), 0.0)));
    let alpha_phi = add(&scale(&a1, c(-phi.sin(), 0.0)), &scale(&a2, c(phi.cos(), 0.0)));
    let beta_r = add(&scale(&beta, c((nf * phi).cos(), 0.0)), &scale(&b5, c((nf * phi).sin(), 0.0)));
    let lhs = add(
        &add(&scale(&mul(&alpha_r, &dpsi_dr), c(0.0, -1.0)), &scale(&mul(&alpha_phi, &dpsi_dphi), c(0.0, -1.0 / r))),
        &scale(&mul(&beta_r, &psi), c(m * f, 0.0)),
    );
    // right-hand side
    let bracket = add(
        &scale(&add(&scale(&sigma, c(l, 0.0)), &scale(&add(&one, &scale(&s, c(nf, 0.0))), c(-0.5, 0.0))), c(1.0 / r, 0.0)),
        &scale(&a, c(m * f, 0.0)),
    );
    let inner = add(&deta_c, &scale(&mul(&bracket, &eta_c), c(-1.0, 0.0)));
    let rhs = scale(&mul(&u, &mul(&scale(&a1, c(0.0, -1.0)), &inner)), phase);
    let mut big = 0.0f64;
    let mut dev = 0.0f64;
    for i in 0..16 {
        big = big.max(lhs[i][0].abs()).max(rhs[i][0].abs());
        dev = dev.max((lhs[i][0] - rhs[i][0]).abs());
    }
    dev / big.max(1e-300)
}

// ---------------------------------------------------------------------------------------------
// the profile of the vortex
// ---------------------------------------------------------------------------------------------

/// The profile far from the line: 1 - n^2/rho^2 - n^2(n^2+8)/(2 rho^4) - [n^2(n^2+8)^2 - n^6/2]/rho^6.
pub fn asymptotic_f(n: u32, rho: f64) -> f64 {
    let n2 = (n * n) as f64;
    let x = 1.0 / (rho * rho);
    1.0 - n2 * x - 0.5 * n2 * (n2 + 8.0) * x * x - (n2 * (n2 + 8.0) * (n2 + 8.0) - 0.5 * n2 * n2 * n2) * x * x * x
}

/// The profile f_n on the uniform grid rho_i = i h, i = 0 .. N, with its derivative.
#[derive(Clone, Debug)]
pub struct Profile {
    /// Absolute value of the winding number.
    pub n: u32,
    /// Spacing of the grid.
    pub h: f64,
    pub rho: Vec<f64>,
    pub f: Vec<f64>,
    /// df/drho.
    pub df: Vec<f64>,
    /// The coefficient a_n of f = a_n rho^n near the line.
    pub slope: f64,
    /// Largest correction of the last Newton step (coarser of the two grids).
    pub last_newton_step: f64,
    /// Number of Newton steps (coarser of the two grids).
    pub newton_steps: usize,
}

/// Newton's method for the finite-difference equations
///   (f_{i+1} - 2 f_i + f_{i-1})/h^2 + (f_{i+1} - f_{i-1})/(2 h rho_i) - n^2 f_i/rho_i^2 - f_i (f_i^2 - 1)/2 = 0,
/// i = 1 .. N-1, with f_0 = 0 and f_N = asymptotic_f(rho_max).  The Jacobian is tridiagonal
/// and is solved by elimination.  Returns (f, last Newton step, number of steps).
fn profile_fd(n: u32, h: f64, points: usize) -> (Vec<f64>, f64, usize) {
    let nn = points; // index of the last point
    let n2 = (n * n) as f64;
    let rho = |i: usize| i as f64 * h;
    let mut f: Vec<f64> = (0..=nn).map(|i| (rho(i) / 2.0).tanh().powi(n as i32)).collect();
    f[0] = 0.0;
    f[nn] = asymptotic_f(n, rho(nn));
    let (mut last, mut steps) = (f64::INFINITY, 0usize);
    for _ in 0..60 {
        // residual and tridiagonal Jacobian for the unknowns f_1 .. f_{N-1}
        let m = nn - 1;
        let (mut lo, mut di, mut up, mut rhs) = (vec![0.0; m], vec![0.0; m], vec![0.0; m], vec![0.0; m]);
        for i in 1..nn {
            let r = rho(i);
            let res = (f[i + 1] - 2.0 * f[i] + f[i - 1]) / (h * h) + (f[i + 1] - f[i - 1]) / (2.0 * h * r) - n2 * f[i] / (r * r) - 0.5 * f[i] * (f[i] * f[i] - 1.0);
            lo[i - 1] = 1.0 / (h * h) - 1.0 / (2.0 * h * r);
            di[i - 1] = -2.0 / (h * h) - n2 / (r * r) - 0.5 * (3.0 * f[i] * f[i] - 1.0);
            up[i - 1] = 1.0 / (h * h) + 1.0 / (2.0 * h * r);
            rhs[i - 1] = -res;
        }
        // elimination (the boundary values are fixed, so that lo[0] and up[m-1] multiply zero corrections)
        for k in 1..m {
            let w = lo[k] / di[k - 1];
            di[k] -= w * up[k - 1];
            rhs[k] -= w * rhs[k - 1];
        }
        let mut delta = vec![0.0; m];
        delta[m - 1] = rhs[m - 1] / di[m - 1];
        for k in (0..m - 1).rev() {
            delta[k] = (rhs[k] - up[k] * delta[k + 1]) / di[k];
        }
        last = 0.0;
        for i in 1..nn {
            f[i] += delta[i - 1];
            last = last.max(delta[i - 1].abs());
        }
        steps += 1;
        if last < 1e-13 {
            break;
        }
    }
    (f, last, steps)
}

/// The profile of winding number n on the grid of spacing `h` from 0 to `rho_max` (an integer
/// multiple of h).  The finite-difference solution has an error of the order h^2; the function
/// solves on the grids h and h/2 and combines them (Richardson extrapolation,
/// (4 f_{h/2} - f_h)/3), which leaves an error of the order h^4.
pub fn profile(n: u32, h: f64, rho_max: f64) -> Profile {
    assert!(n >= 1, "profile: the winding number must be at least 1");
    let points = (rho_max / h).round() as usize;
    let (fc, last, steps) = profile_fd(n, h, points);
    let (ff, _, _) = profile_fd(n, h / 2.0, 2 * points);
    let rho: Vec<f64> = (0..=points).map(|i| i as f64 * h).collect();
    let f: Vec<f64> = (0..=points).map(|i| (4.0 * ff[2 * i] - fc[i]) / 3.0).collect();
    // derivative: central differences on both grids, combined in the same way
    let mut df = vec![0.0; points + 1];
    for i in 1..points {
        let dc = (fc[i + 1] - fc[i - 1]) / (2.0 * h);
        let dfine = (ff[2 * i + 1] - ff[2 * i - 1]) / h;
        df[i] = (4.0 * dfine - dc) / 3.0;
    }
    // slope a_n from the series f = a rho^n [1 - rho^2/(8(n+1)) + c4 rho^4 + O(rho^6)], with
    // c4 = a^2/48 + 1/768 for n = 1 and c4 = 1/(128 (n+1)(n+2)) for n >= 2 (insert the series
    // into the differential equation).  The values of f at the two radii 0.2 and 0.4 are
    // used, where the extrapolated finite-difference solution is accurate, and the two are
    // combined so that the term of the order rho^6 cancels.
    let nf = n as f64;
    let (i1, i2) = ((0.2 / h).round() as usize, (0.4 / h).round() as usize);
    let mut slope = f[i1] / rho[i1].powi(n as i32);
    for _ in 0..4 {
        let c4 = if n == 1 { slope * slope / 48.0 + 1.0 / 768.0 } else { 1.0 / (128.0 * (nf + 1.0) * (nf + 2.0)) };
        let a_at = |i: usize| f[i] / (rho[i].powi(n as i32) * (1.0 - rho[i] * rho[i] / (8.0 * (nf + 1.0)) + c4 * rho[i].powi(4)));
        let (r1, r2) = (rho[i1], rho[i2]);
        slope = (a_at(i1) * r2.powi(6) - a_at(i2) * r1.powi(6)) / (r2.powi(6) - r1.powi(6));
    }
    df[0] = if n == 1 { slope } else { 0.0 };
    // far end: derivative of the asymptotic form
    let e = 1e-4 * rho_max;
    df[points] = (asymptotic_f(n, rho_max + e) - asymptotic_f(n, rho_max - e)) / (2.0 * e);
    Profile { n, h, rho, f, df, slope, last_newton_step: last, newton_steps: steps }
}

/// The slope a_n by shooting: the solution that starts on the line as a rho^n either
/// exceeds 1 (a too large) or turns down (a too small); bisection on a.  The integration
/// uses the Runge-Kutta method of fourth order with the step `h`.  It starts at rho = 0.05
/// with the first three terms of the series, a rho^n [1 - rho^2/(8(n+1)) + c4 rho^4] (c4 as
/// in `profile`); it must not start at a radius of the order of the step, because close to
/// the line the terms 1/rho and 1/rho^2 of the equation make a step of the size of rho
/// inaccurate, however small the step is.
pub fn slope_by_shooting(n: u32, h: f64) -> f64 {
    let nf = n as f64;
    let n2 = nf * nf;
    let rhs = |r: f64, y: [f64; 2]| -> [f64; 2] { [y[1], -y[1] / r + n2 * y[0] / (r * r) + 0.5 * y[0] * (y[0] * y[0] - 1.0)] };
    let classify = |a: f64| -> i32 {
        let r0: f64 = 0.05;
        let b = -a / (8.0 * (nf + 1.0));
        let c = a * if n == 1 { a * a / 48.0 + 1.0 / 768.0 } else { 1.0 / (128.0 * (nf + 1.0) * (nf + 2.0)) };
        let mut y = [
            a * r0.powf(nf) + b * r0.powf(nf + 2.0) + c * r0.powf(nf + 4.0),
            nf * a * r0.powf(nf - 1.0) + (nf + 2.0) * b * r0.powf(nf + 1.0) + (nf + 4.0) * c * r0.powf(nf + 3.0),
        ];
        let mut r = r0;
        while r < 30.0 {
            let k1 = rhs(r, y);
            let k2 = rhs(r + h / 2.0, [y[0] + h / 2.0 * k1[0], y[1] + h / 2.0 * k1[1]]);
            let k3 = rhs(r + h / 2.0, [y[0] + h / 2.0 * k2[0], y[1] + h / 2.0 * k2[1]]);
            let k4 = rhs(r + h, [y[0] + h * k3[0], y[1] + h * k3[1]]);
            y = [y[0] + h / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]), y[1] + h / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1])];
            r += h;
            if y[0] > 1.0 {
                return 1;
            }
            if y[1] < 0.0 {
                return -1;
            }
        }
        0
    };
    let (mut lo, mut hi) = (0.0f64, 2.0f64);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if mid == lo || mid == hi {
            break;
        }
        match classify(mid) {
            1 => hi = mid,
            -1 => lo = mid,
            _ => break,
        }
    }
    0.5 * (lo + hi)
}

/// The constant c_n of the energy per unit length, pi v_VS^2 [n^2 ln(m_s R) + c_n].
/// The integral over the grid is taken with Simpson's rule (the integrand at rho = 0 is its
/// limit: 2 a_1^2 + 1/4 times rho for n = 1, which vanishes, as it does for every n); the
/// part outside the grid is -n^4/(2 R^2) to the order kept.
pub fn energy_constant(p: &Profile) -> f64 {
    let n2 = (p.n * p.n) as f64;
    let np = p.rho.len() - 1;
    let np = if np % 2 == 0 { np } else { np - 1 };
    let g = |i: usize| -> f64 {
        if i == 0 {
            return 0.0;
        }
        let (r, f, d) = (p.rho[i], p.f[i], p.df[i]);
        r * (d * d + n2 * f * f / (r * r) + 0.25 * (f * f - 1.0) * (f * f - 1.0))
    };
    let mut sum = g(0) + g(np);
    for i in 1..np {
        sum += if i % 2 == 1 { 4.0 * g(i) } else { 2.0 * g(i) };
    }
    let integral = sum * p.h / 3.0;
    let big_r = p.rho[np];
    integral - n2 * big_r.ln() - n2 * n2 / (2.0 * big_r * big_r)
}

// ---------------------------------------------------------------------------------------------
// the radial functions of the fermion
// ---------------------------------------------------------------------------------------------

/// The solution of the radial equations that falls far from the line, for one channel.
#[derive(Clone, Debug)]
pub struct RadialSolution {
    /// Radii (a subset of the grid of the profile: every second point from rho = 8 h on).
    pub rho: Vec<f64>,
    /// The component with A = +1.
    pub u: Vec<f64>,
    /// The component with A = -1.
    pub w: Vec<f64>,
    /// Measured exponent of u + w near the line.
    pub exponent_plus: f64,
    /// Measured exponent of u - w near the line.
    pub exponent_minus: f64,
    /// True if the channel carries a bound solution; `u` and `w` are then normalized to
    /// Int_0^infinity (u^2 + w^2) rho drho = 1.  Otherwise they are scaled to w = 1 at the
    /// far end of the grid, and the integral does not exist.
    pub bound: bool,
}

impl RadialSolution {
    /// The measured exponent of the combination that dominates near the line: u - w for
    /// l > 0, u + w for l < 0 (for l = 0 the two are equal).
    pub fn dominant_exponent(&self, two_l: i32) -> f64 {
        if two_l > 0 {
            self.exponent_minus
        } else {
            self.exponent_plus
        }
    }
}

/// The exponents -a + l and -a - l of u + w and u - w near the line, a = (1 + n s)/2; `two_l` = 2 l.
/// They are integers.  (The smaller of the two is the exponent of the solution itself.  The
/// combination with the larger exponent also contains a part that the other one drives
/// through the term mu f, of the exponent (smaller exponent) + |n| + 1; where 2|l| > |n| + 1
/// that part is the larger one near the line, so that only the smaller exponent can then be
/// read off a numerical solution.)
pub fn expected_exponents(n: i32, s: i32, two_l: i32) -> (f64, f64) {
    let a = 0.5 * (1.0 + (n * s) as f64);
    let l = 0.5 * two_l as f64;
    (-a + l, -a - l)
}

/// Does the channel (n, s, l) carry a bound solution of zero energy?  It does exactly if both
/// exponents are non-negative.
pub fn channel_is_bound(n: i32, s: i32, two_l: i32) -> bool {
    let (ep, em) = expected_exponents(n, s, two_l);
    ep > -0.5 && em > -0.5
}

/// The channels (s, 2l) of the winding number n that carry zero modes, by the rule of the
/// derivation: s = -sign(n), l = -(|n|-1)/2, ..., (|n|-1)/2.
pub fn zero_mode_channels(n: i32) -> Vec<(i32, i32)> {
    let s = if n > 0 { -1 } else { 1 };
    let k = n.abs() - 1;
    (0..=k).map(|j| (s, -k + 2 * j)).collect()
}

/// Integrates the radial equations
///     du/drho = (-a/rho + mu f) u + (l/rho) w,    dw/drho = (-a/rho - mu f) w + (l/rho) u,
/// a = (1 + n s)/2, INWARD from the far end of the grid, where the solution is started with
/// its asymptotic form (w = 1, u = -l/(2 mu rho)).  Integrated inward, the solution that
/// falls outward is the growing one, so that rounding errors and the error of the start are
/// damped.  Runge-Kutta method of fourth order with the step 2h; the profile is needed at
/// the grid points only.  The integration stops at rho = 8 h (closer to the line the step
/// is not small compared with rho).  `p` must be the profile of |n|.
///
/// The exponents are measured between rho = 16 h and rho = 32 h.  For a channel with a bound
/// solution the functions are normalized; the part of the integral inside rho = 8 h is added
/// with the power laws u + w = c_+ rho^(-a+l), u - w = c_- rho^(-a-l) fitted at rho = 8 h.
pub fn radial_solution(p: &Profile, n: i32, s: i32, two_l: i32, mu: f64) -> RadialSolution {
    assert_eq!(p.n as i32, n.abs(), "radial_solution: the profile must have the winding number |n|");
    let a = 0.5 * (1.0 + (n * s) as f64);
    let l = 0.5 * two_l as f64;
    let np = p.rho.len() - 1;
    let top = if np % 2 == 0 { np } else { np - 1 };
    assert!(top >= 64, "radial_solution: the grid is too short");
    let step = 2.0 * p.h;
    let rhs = |r: f64, f: f64, y: [f64; 2]| -> [f64; 2] { [(-a / r + mu * f) * y[0] + (l / r) * y[1], (-a / r - mu * f) * y[1] + (l / r) * y[0]] };
    let mut y = [-l / (2.0 * mu * p.rho[top]), 1.0];
    let mut rows: Vec<(usize, f64, f64)> = vec![(top, y[0], y[1])];
    let mut i = top;
    while i >= 10 {
        let (r0, r1, r2) = (p.rho[i], p.rho[i - 1], p.rho[i - 2]);
        let (f0, f1, f2) = (p.f[i], p.f[i - 1], p.f[i - 2]);
        let hh = -step;
        let k1 = rhs(r0, f0, y);
        let k2 = rhs(r1, f1, [y[0] + hh / 2.0 * k1[0], y[1] + hh / 2.0 * k1[1]]);
        let k3 = rhs(r1, f1, [y[0] + hh / 2.0 * k2[0], y[1] + hh / 2.0 * k2[1]]);
        let k4 = rhs(r2, f2, [y[0] + hh * k3[0], y[1] + hh * k3[1]]);
        y = [y[0] + hh / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]), y[1] + hh / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1])];
        i -= 2;
        rows.push((i, y[0], y[1]));
        // keep the numbers in range
        let big = y[0].abs().max(y[1].abs());
        if big > 1e200 {
            for row in rows.iter_mut() {
                row.1 /= big;
                row.2 /= big;
            }
            y = [y[0] / big, y[1] / big];
        }
    }
    rows.reverse();
    let rho: Vec<f64> = rows.iter().map(|r| p.rho[r.0]).collect();
    let mut u: Vec<f64> = rows.iter().map(|r| r.1).collect();
    let mut w: Vec<f64> = rows.iter().map(|r| r.2).collect();
    // exponents of u + w and u - w between rho = 16 h and rho = 32 h
    let find = |idx: usize| rows.iter().position(|r| r.0 == idx).expect("grid too short");
    let (j1, j2) = (find(16), find(32));
    let ex = |x1: f64, x2: f64| if x1 == 0.0 || x2 == 0.0 { f64::NAN } else { (x2.abs() / x1.abs()).ln() / (rho[j2] / rho[j1]).ln() };
    let (mut exponent_plus, mut exponent_minus) = (ex(u[j1] + w[j1], u[j2] + w[j2]), ex(u[j1] - w[j1], u[j2] - w[j2]));
    if two_l == 0 {
        // u vanishes identically: both combinations are +-w
        let e = ex(w[j1], w[j2]);
        exponent_plus = e;
        exponent_minus = e;
    }
    let bound = channel_is_bound(n, s, two_l);
    if bound {
        // integral over the grid (trapezoidal rule, spacing 2h) plus the part inside rho_0 = 8 h
        let dens: Vec<f64> = (0..rho.len()).map(|k| (u[k] * u[k] + w[k] * w[k]) * rho[k]).collect();
        let mut integral = 0.0;
        for k in 1..rho.len() {
            integral += 0.5 * (dens[k] + dens[k - 1]) * (rho[k] - rho[k - 1]);
        }
        let (ep, em) = expected_exponents(n, s, two_l);
        let r0 = rho[0];
        let (cp, cm) = (u[0] + w[0], u[0] - w[0]);
        // u^2 + w^2 = [(u+w)^2 + (u-w)^2]/2;  Int_0^r0 c^2 (rho/r0)^(2e) rho drho = c^2 r0^2/(2e + 2)
        integral += 0.5 * (cp * cp * r0 * r0 / (2.0 * ep + 2.0) + cm * cm * r0 * r0 / (2.0 * em + 2.0));
        let scale = 1.0 / integral.sqrt();
        // sign: w positive where it is largest
        let kmax = (0..rho.len()).max_by(|&x, &y2| w[x].abs().partial_cmp(&w[y2].abs()).unwrap()).unwrap();
        let sign = if w[kmax] < 0.0 { -1.0 } else { 1.0 };
        for k in 0..rho.len() {
            u[k] *= scale * sign;
            w[k] *= scale * sign;
        }
    } else {
        let far = w[rho.len() - 1];
        for k in 0..rho.len() {
            u[k] /= far;
            w[k] /= far;
        }
    }
    RadialSolution { rho, u, w, exponent_plus, exponent_minus, bound }
}

/// The closed form of the zero mode with l = 0 (odd n): u = 0,
/// w = rho^((|n|-1)/2) exp(-mu Int_0^rho f), not normalized, at the grid points of the
/// profile (the integral by the trapezoidal rule with the end correction of the
/// Euler-Maclaurin formula, h^2 (f'(0) - f'(rho))/12).
pub fn closed_form_l0(p: &Profile, mu: f64) -> Vec<f64> {
    let k = 0.5 * (p.n as f64 - 1.0);
    let mut integral = 0.0;
    let mut out = Vec::with_capacity(p.rho.len());
    out.push(if p.n == 1 { 1.0 } else { 0.0 });
    for i in 1..p.rho.len() {
        integral += 0.5 * p.h * (p.f[i] + p.f[i - 1]);
        let corrected = integral + p.h * p.h * (p.df[0] - p.df[i]) / 12.0;
        out.push(p.rho[i].powf(k) * (-mu * corrected).exp());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_exact_statements_hold_on_both_spacetimes() {
        let gm = Gammas::build();
        for tag in ["1671", "1644"] {
            let v = VortexAlgebra::new(&gm, tag);
            for (id, statement, ok) in v.checks() {
                assert!(ok, "{} {}: {}", tag, id, statement);
            }
        }
    }

    #[test]
    fn the_reduction_to_the_radial_equation_is_an_identity() {
        // pseudo-random values from a linear congruential generator (no external crate)
        let mut seed: u64 = 20261113;
        let mut next = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((seed >> 11) as f64) / ((1u64 << 53) as f64) - 0.5
        };
        let gm = Gammas::build();
        for tag in ["1671", "1644"] {
            let v = VortexAlgebra::new(&gm, tag);
            for n in [1, -1, 2, -2, 3] {
                for trial in 0..3 {
                    let eta: Vec<(f64, f64)> = (0..16).map(|_| (next(), next())).collect();
                    let deta: Vec<(f64, f64)> = (0..16).map(|_| (next(), next())).collect();
                    let l = if n % 2 == 0 { 0.5 + trial as f64 } else { trial as f64 - 1.0 };
                    let res = reduction_residual(&v, &eta, &deta, 0.3 + next().abs() * 3.0, 6.0 * next(), 0.2 + next().abs(), 0.5 + next().abs(), n, l);
                    assert!(res < 1e-13, "{} n = {} l = {}: residual {:e}", tag, n, l, res);
                }
            }
        }
    }

    #[test]
    fn the_profile_by_two_methods() {
        let p1 = profile(1, 0.02, 40.0);
        let shoot = slope_by_shooting(1, 0.002);
        // the known slope of the Gross-Pitaevskii vortex, 0.58319 in units of the healing length, divided by sqrt 2
        assert!((shoot - 0.58319 / 2f64.sqrt()).abs() < 2e-6, "shooting {}", shoot);
        assert!((p1.slope - shoot).abs() < 2e-7, "finite differences {} against shooting {}", p1.slope, shoot);
        assert!(p1.last_newton_step < 1e-12);
        // monotonic, between 0 and 1, and the differential equation holds for the extrapolated values
        for i in 1..p1.rho.len() {
            assert!(p1.f[i] > p1.f[i - 1] && p1.f[i] < 1.0);
        }
        let i = 200; // rho = 4
        let d2 = (p1.f[i + 1] - 2.0 * p1.f[i] + p1.f[i - 1]) / (p1.h * p1.h);
        let res = d2 + p1.df[i] / p1.rho[i] - p1.f[i] / (p1.rho[i] * p1.rho[i]) - 0.5 * p1.f[i] * (p1.f[i] * p1.f[i] - 1.0);
        assert!(res.abs() < 1e-4, "residual of the differential equation {:e}", res);
        // the energy constant: ln(1.464/sqrt 2) = 0.0346 in the literature on the Gross-Pitaevskii vortex (1.464 is rounded)
        let c1 = energy_constant(&p1);
        assert!((c1 - 0.0343).abs() < 5e-4, "c_1 = {}", c1);
        // winding number 2
        let p2 = profile(2, 0.02, 40.0);
        let shoot2 = slope_by_shooting(2, 0.002);
        assert!((p2.slope - shoot2).abs() < 2e-6 * shoot2.max(1.0), "n = 2: {} against {}", p2.slope, shoot2);
    }

    #[test]
    fn the_asymptotic_form_solves_the_equation_far_from_the_line() {
        // the residual of the equation for the form truncated after rho^-6 must fall as rho^-8:
        // by the factor 2^8 = 256 when the radius is doubled
        for n in [1u32, 2, 3] {
            let residual = |r: f64| -> f64 {
                let e = 0.05;
                let f = asymptotic_f(n, r);
                let d1 = (asymptotic_f(n, r + e) - asymptotic_f(n, r - e)) / (2.0 * e);
                let d2 = (asymptotic_f(n, r + e) - 2.0 * f + asymptotic_f(n, r - e)) / (e * e);
                d2 + d1 / r - (n * n) as f64 * f / (r * r) - 0.5 * f * (f * f - 1.0)
            };
            let (r1, r2) = (residual(20.0), residual(40.0));
            assert!(r1.abs() < 1e-5, "n = {}: residual {:e} at rho = 20", n, r1);
            let ratio = r1 / r2;
            assert!(ratio > 200.0 && ratio < 320.0, "n = {}: the residual falls by the factor {} from rho = 20 to 40", n, ratio);
        }
    }

    #[test]
    fn the_zero_mode_with_l_equal_zero_is_the_closed_form() {
        let p = profile(1, 0.01, 40.0);
        for mu in [0.5, 1.0, 2.0] {
            let sol = radial_solution(&p, 1, -1, 0, mu);
            let exact = closed_form_l0(&p, mu);
            // compare the shapes between rho = 0.2 and rho = 8/mu
            let k0 = sol.rho.iter().position(|r| *r >= 0.2).unwrap();
            let idx = |k: usize| (sol.rho[k] / p.h).round() as usize;
            let ratio0 = sol.w[k0] / exact[idx(k0)];
            let mut dev = 0.0f64;
            let mut k = k0;
            while sol.rho[k] < 8.0 / mu {
                dev = dev.max((sol.w[k] / exact[idx(k)] / ratio0 - 1.0).abs());
                assert!(sol.u[k].abs() < 1e-9 * sol.w[k].abs().max(1e-300), "u does not vanish");
                k += 1;
            }
            assert!(dev < 1e-6, "mu = {}: deviation from the closed form {:e}", mu, dev);
        }
    }

    #[test]
    fn the_count_of_zero_modes_is_four_times_the_winding_number() {
        for n in [1i32, 2, 3, -1, -2] {
            let p = profile(n.unsigned_abs(), 0.01, 40.0);
            let mut bound_channels = 0;
            for s in [1, -1] {
                // l runs over the integers (odd n) or the half-integers (even n) with |l| <= 3
                let start = if n % 2 == 0 { -5 } else { -6 };
                let mut two_l = start;
                while two_l <= 6 {
                    let (ep, em) = expected_exponents(n, s, two_l);
                    for mu in [0.5, 2.0] {
                        let sol = radial_solution(&p, n, s, two_l, mu);
                        // the exponent of the solution near the line is the smaller of the two expected integers
                        let dom = sol.dominant_exponent(two_l);
                        assert!((dom - ep.min(em)).abs() < 0.1, "n {} s {} 2l {} mu {}: exponent {} expected {}", n, s, two_l, mu, dom, ep.min(em));
                        if channel_is_bound(n, s, two_l) {
                            // in a channel with a bound solution both exponents can be read off
                            assert!((sol.exponent_plus - ep).abs() < 0.1 && (sol.exponent_minus - em).abs() < 0.1, "n {} s {} 2l {} mu {}: exponents {} {} expected {} {}", n, s, two_l, mu, sol.exponent_plus, sol.exponent_minus, ep, em);
                        }
                    }
                    if channel_is_bound(n, s, two_l) {
                        bound_channels += 1;
                        assert!(zero_mode_channels(n).contains(&(s, two_l)));
                    }
                    two_l += 2;
                }
            }
            assert_eq!(bound_channels, n.abs(), "n = {}", n);
            assert_eq!(zero_mode_channels(n).len() as i32, n.abs());
            // four zero modes of the sixteen-component field in each channel: 4|n| in all
            assert_eq!(4 * bound_channels, 4 * n.abs());
        }
    }
}
