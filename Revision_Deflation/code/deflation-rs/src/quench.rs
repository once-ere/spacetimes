//! Can the paired state follow the deflation?  Time-dependent mean-field dynamics of
//! the order parameter while the pairing strength grows exponentially.
//!
//! WHY.  On a deflating spacetime the dimensionless pairing strength grows as
//! lambda(x4) = lambda_i exp(gsum (x4 - x4_i)) (module `history`).  The equilibrium gap
//! for the coupling of the moment grows much faster, d ln(gap)/dx4 = gsum/lambda at weak
//! coupling (check def-H3).  A state changes at most at the rate of its own energy
//! scale, the gap; so the paired state can follow only if
//!
//!     Xi(x4) = gsum / ( lambda(x4) gap_eq(x4) )   <<  1 .
//!
//! Early in the deflation the gap is exponentially small and this must fail.  What
//! happens then is computed here.
//!
//! MODEL.  The standard reduced model of a superfluid: pairs (k, -k) with energies xi
//! (measured from the Fermi energy) in a shell -E_sh < xi < E_sh with a constant
//! density of states, and a pairing interaction of the dimensionless strength lambda
//! between all of them.  In the mean-field (time-dependent Bardeen-Cooper-Schrieffer)
//! approximation each pair is described by its anomalous amplitude F(xi, t) (complex)
//! and its occupation n(xi, t); with the order parameter
//!
//!     Delta(t) = lambda(t) Int_{-E_sh}^{E_sh} F(xi, t) dxi
//!
//! the equations of motion are
//!
//!     dF/dt = -i [ 2 xi F - Delta (1 - 2 n) ] ,        dn/dt = 2 Im( Delta^* F ) .
//!
//! They conserve |2F|^2 + (1 - 2n)^2 = 1 for every xi.  In equilibrium
//! F = Delta/(2E), n = (1 - xi/E)/2, E = sqrt(xi^2 + Delta^2), and the gap equation
//! 1 = lambda asinh(E_sh/Delta) gives
//!
//!     Delta_eq(lambda) = E_sh / sinh(1/lambda)      (= 2 E_sh exp(-1/lambda) at weak coupling).
//!
//! The model keeps exactly what matters for the question: a gap that depends
//! exponentially on a coupling that grows exponentially.  It does not keep the momentum
//! dependence of the gap function of the main project; the mapping is
//! lambda = N(0) W(k_F, k_F) of `gap.rs`, and E_sh stands for the energy range of the
//! attraction.
//!
//! NUMBERS REPORTED.  |Delta(t)|, Delta_eq(lambda(t)), and the density of broken pairs
//!     N_ex = Int dxi (1 - S.B)/2,   S = (2 Re F, 2 Im F, 1 - 2n),   B = (Re Delta, Im Delta, xi)/E ,
//! which is the number of pairs (per unit energy of the shell, times E_sh) that are
//! not in the ground state of the order parameter of the moment.
//!
//! METHOD.  The classical Runge-Kutta method with a fixed step; the energies are placed
//! on a grid that is dense at the Fermi energy, xi = E_sh sinh(a u)/sinh(a), u uniform.
//! UNITS: E_sh = 1.

/// The energies of the shell and their quadrature weights.
pub struct Shell {
    pub xi: Vec<f64>,
    pub w: Vec<f64>,
}

impl Shell {
    /// `n` energies, midpoint rule in u on [-1, 1], xi = sinh(a u)/sinh(a).
    pub fn new(n: usize, a: f64) -> Shell {
        let du = 2.0 / n as f64;
        let mut xi = Vec::with_capacity(n);
        let mut w = Vec::with_capacity(n);
        for j in 0..n {
            let u = -1.0 + (j as f64 + 0.5) * du;
            xi.push((a * u).sinh() / a.sinh());
            w.push(a * (a * u).cosh() / a.sinh() * du);
        }
        Shell { xi, w }
    }
}

/// The state of all pairs.
#[derive(Clone)]
pub struct State {
    pub fre: Vec<f64>,
    pub fim: Vec<f64>,
    pub n: Vec<f64>,
}

/// The equilibrium gap of the shell model, E_sh/sinh(1/lambda), E_sh = 1.
pub fn delta_eq(lambda: f64) -> f64 {
    1.0 / (1.0 / lambda).sinh()
}

impl State {
    /// The equilibrium state for the real gap `delta`.
    pub fn equilibrium(sh: &Shell, delta: f64) -> State {
        let m = sh.xi.len();
        let mut s = State { fre: vec![0.0; m], fim: vec![0.0; m], n: vec![0.0; m] };
        for j in 0..m {
            let e = (sh.xi[j] * sh.xi[j] + delta * delta).sqrt();
            s.fre[j] = 0.5 * delta / e;
            s.n[j] = 0.5 * (1.0 - sh.xi[j] / e);
        }
        s
    }

    /// The order parameter Delta = lambda Int F dxi, as (real part, imaginary part).
    pub fn delta(&self, sh: &Shell, lambda: f64) -> (f64, f64) {
        let (mut a, mut b) = (0.0, 0.0);
        for j in 0..sh.xi.len() {
            a += sh.w[j] * self.fre[j];
            b += sh.w[j] * self.fim[j];
        }
        (lambda * a, lambda * b)
    }

    /// The density of broken pairs N_ex with respect to the order parameter of the moment.
    pub fn broken_pairs(&self, sh: &Shell, lambda: f64) -> f64 {
        let (dr, di) = self.delta(sh, lambda);
        let mut s = 0.0;
        for j in 0..sh.xi.len() {
            let e = (sh.xi[j] * sh.xi[j] + dr * dr + di * di).sqrt();
            let dot = (2.0 * self.fre[j] * dr + 2.0 * self.fim[j] * di + (1.0 - 2.0 * self.n[j]) * sh.xi[j]) / e;
            s += sh.w[j] * 0.5 * (1.0 - dot);
        }
        s
    }

    /// The mean-field energy  Int 2 xi n dxi - |Delta|^2/lambda  (conserved when lambda is constant).
    pub fn energy(&self, sh: &Shell, lambda: f64) -> f64 {
        let (dr, di) = self.delta(sh, lambda);
        let mut s = 0.0;
        for j in 0..sh.xi.len() {
            s += sh.w[j] * 2.0 * sh.xi[j] * self.n[j];
        }
        s - (dr * dr + di * di) / lambda
    }

    /// The largest deviation of |2F|^2 + (1 - 2n)^2 from 1 over all pairs.
    pub fn length_deviation(&self) -> f64 {
        let mut d: f64 = 0.0;
        for j in 0..self.n.len() {
            let l = 4.0 * (self.fre[j] * self.fre[j] + self.fim[j] * self.fim[j]) + (1.0 - 2.0 * self.n[j]).powi(2);
            d = d.max((l - 1.0).abs());
        }
        d
    }
}

/// Time derivative of the state for the coupling `lambda`.
fn rhs(sh: &Shell, s: &State, lambda: f64, out: &mut State) {
    let (dr, di) = s.delta(sh, lambda);
    for j in 0..sh.xi.len() {
        let one = 1.0 - 2.0 * s.n[j];
        out.fre[j] = 2.0 * sh.xi[j] * s.fim[j] - di * one;
        out.fim[j] = -2.0 * sh.xi[j] * s.fre[j] + dr * one;
        out.n[j] = 2.0 * (dr * s.fim[j] - di * s.fre[j]);
    }
}

/// One Runge-Kutta step from time t to t + dt with the coupling lambda(t).
pub fn step<L: Fn(f64) -> f64>(sh: &Shell, s: &mut State, lambda: &L, t: f64, dt: f64, work: &mut [State; 5]) {
    let m = sh.xi.len();
    let (k1, rest) = work.split_at_mut(1);
    let (k2, rest) = rest.split_at_mut(1);
    let (k3, rest) = rest.split_at_mut(1);
    let (k4, tmp) = rest.split_at_mut(1);
    let (k1, k2, k3, k4, tmp) = (&mut k1[0], &mut k2[0], &mut k3[0], &mut k4[0], &mut tmp[0]);
    let combine = |base: &State, k: &State, f: f64, out: &mut State| {
        for j in 0..m {
            out.fre[j] = base.fre[j] + f * k.fre[j];
            out.fim[j] = base.fim[j] + f * k.fim[j];
            out.n[j] = base.n[j] + f * k.n[j];
        }
    };
    rhs(sh, s, lambda(t), k1);
    combine(s, k1, 0.5 * dt, tmp);
    rhs(sh, tmp, lambda(t + 0.5 * dt), k2);
    combine(s, k2, 0.5 * dt, tmp);
    rhs(sh, tmp, lambda(t + 0.5 * dt), k3);
    combine(s, k3, dt, tmp);
    rhs(sh, tmp, lambda(t + dt), k4);
    for j in 0..m {
        s.fre[j] += dt / 6.0 * (k1.fre[j] + 2.0 * k2.fre[j] + 2.0 * k3.fre[j] + k4.fre[j]);
        s.fim[j] += dt / 6.0 * (k1.fim[j] + 2.0 * k2.fim[j] + 2.0 * k3.fim[j] + k4.fim[j]);
        s.n[j] += dt / 6.0 * (k1.n[j] + 2.0 * k2.n[j] + 2.0 * k3.n[j] + k4.n[j]);
    }
}

/// Work arrays for `step`.
pub fn workspace(m: usize) -> [State; 5] {
    let z = || State { fre: vec![0.0; m], fim: vec![0.0; m], n: vec![0.0; m] };
    [z(), z(), z(), z(), z()]
}

/// One sample of a run.
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    /// tau_d = gsum (t - t_i): the deflation clock since the start
    pub tau: f64,
    pub lambda: f64,
    /// |Delta| of the evolving state and the equilibrium gap for the coupling of the moment
    pub delta: f64,
    pub delta_eq: f64,
    /// density of broken pairs
    pub n_ex: f64,
}

/// The paired state under exponential growth of the coupling,
/// lambda(t) = lambda_i exp(gsum t), from lambda_i to lambda_f, starting in equilibrium.
/// `gsum` in units of E_sh; `dt` in units of 1/E_sh; about `samples` samples are returned.
pub fn ramp(sh: &Shell, lambda_i: f64, lambda_f: f64, gsum: f64, dt: f64, samples: usize) -> (Vec<Sample>, f64) {
    let t_end = (lambda_f / lambda_i).ln() / gsum;
    let steps = (t_end / dt).ceil() as usize;
    let dt = t_end / steps as f64;
    let lam = |t: f64| lambda_i * (gsum * t).exp();
    let mut s = State::equilibrium(sh, delta_eq(lambda_i));
    let mut work = workspace(sh.xi.len());
    let every = (steps / samples).max(1);
    let mut out: Vec<Sample> = Vec::new();
    let sample = |s: &State, t: f64| -> Sample {
        let l = lam(t);
        let (dr, di) = s.delta(sh, l);
        Sample { tau: gsum * t, lambda: l, delta: (dr * dr + di * di).sqrt(), delta_eq: delta_eq(l), n_ex: s.broken_pairs(sh, l) }
    };
    out.push(sample(&s, 0.0));
    for i in 0..steps {
        let t = i as f64 * dt;
        step(sh, &mut s, &lam, t, dt, &mut work);
        if (i + 1) % every == 0 || i + 1 == steps {
            out.push(sample(&s, (i + 1) as f64 * dt));
        }
    }
    (out, s.length_deviation())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equilibrium_gap_solves_the_gap_equation_on_the_grid() {
        let sh = Shell::new(4000, 8.0);
        // sum of weights = 2, the length of the shell; the midpoint rule in u has the relative error (a du)^2/24 = 7e-7
        assert!((sh.w.iter().sum::<f64>() - 2.0).abs() < 5e-6);
        for lambda in [0.15, 0.3, 0.5] {
            let d = delta_eq(lambda);
            let s = State::equilibrium(&sh, d);
            let (dr, di) = s.delta(&sh, lambda);
            assert!((dr / d - 1.0).abs() < 1e-5 && di.abs() < 1e-15, "lambda {}: {} vs {}", lambda, dr, d);
            assert!(s.broken_pairs(&sh, lambda).abs() < 1e-5);
        }
    }

    #[test]
    fn equilibrium_is_stationary_and_the_energy_is_conserved_after_a_sudden_change() {
        let sh = Shell::new(2000, 8.0);
        let lambda = 0.3;
        let mut s = State::equilibrium(&sh, delta_eq(lambda));
        let mut work = workspace(2000);
        let lam = |_t: f64| lambda;
        for i in 0..4000 {
            step(&sh, &mut s, &lam, i as f64 * 0.025, 0.025, &mut work);
        }
        let (dr, di) = s.delta(&sh, lambda);
        assert!(((dr * dr + di * di).sqrt() / delta_eq(lambda) - 1.0).abs() < 1e-5);
        // sudden change of the coupling to 0.4: the energy with the new coupling is conserved
        let lam2 = |_t: f64| 0.4;
        let e0 = s.energy(&sh, 0.4);
        for i in 0..8000 {
            step(&sh, &mut s, &lam2, i as f64 * 0.025, 0.025, &mut work);
        }
        let e1 = s.energy(&sh, 0.4);
        assert!((e1 - e0).abs() < 1e-7 * e0.abs().max(1e-3), "energy {} -> {}", e0, e1);
        assert!(s.length_deviation() < 1e-8);
    }

    #[test]
    fn a_slow_ramp_is_followed_and_a_fast_one_is_not() {
        let sh = Shell::new(2000, 8.0);
        // slow: at the start Xi = gsum/(lambda gap) = 3e-4/(0.3 * 0.0715) = 0.014
        let (slow, _) = ramp(&sh, 0.3, 0.5, 3e-4, 0.025, 50);
        let last = slow.last().unwrap();
        assert!((last.delta / last.delta_eq - 1.0).abs() < 0.01, "slow ramp: {} vs {}", last.delta, last.delta_eq);
        assert!(last.n_ex < 1e-3);
        // fast: Xi = 30 at the start; the gap has no time to grow
        let (fast, _) = ramp(&sh, 0.3, 0.5, 0.6, 0.025, 50);
        let last = fast.last().unwrap();
        assert!(last.delta / last.delta_eq < 0.6, "fast ramp: {} vs {}", last.delta, last.delta_eq);
    }
}
