//! A time integration of i dw/dx^0 = H w taken apart: what every internal step contributes to the final deviation.
//!
//! THE EXACT SOLUTION.  H is a constant Hermitian matrix with the eigenvalues E_j and the orthonormal eigenvectors
//! v_j.  A state is the sum of its components along the eigenvectors, w = sum_j a_j v_j with a_j = v_j^dagger w (the
//! "mode amplitudes"), and the exact evolution over a time t multiplies a_j by exp(-i E_j t).  With the amplitudes c_j
//! of the initial state the exact solution is w_exact(t) = sum_j c_j exp(-i E_j t) v_j.
//!
//! WHAT IS MEASURED AT EVERY INTERNAL STEP n (from the time t_(n-1) to t_n = t_(n-1) + h_n, state w_n):
//!   * the exact local error  delta_n = w_n - U(h_n) w_(n-1), in mode amplitudes a_j(n) - exp(-i E_j h_n) a_j(n-1):
//!     the difference between the computed state and the exact solution that starts from the computed state of the
//!     step before.  Its norm is `local`.
//!   * the norm `tau` of delta_n in the measure of the integrator: the weighted root-mean-square norm
//!     sqrt( (1/2n) sum over the 2n real unknowns of (error / tolerance)^2 ), with the tolerances the step used.
//!     The integrator accepted the step because the same norm of ITS ESTIMATE of the local error (`test`) was at
//!     most 1; comparing the two shows how good the estimate is.
//!   * the deviation  d_n = w_n - w_exact(t_n), with the norm `deviation`.
//!   * the cosine `turn` of the angle between delta_n and delta_(n-1), both carried to the same time: +1 means that the
//!     local errors of consecutive steps add up, -1 that they cancel.
//!
//! THE IDENTITY.  d_n = U(h_n) d_(n-1) + delta_n, hence after N steps
//!     d_N = sum_(n=1..N) U(t_N - t_n) delta_n :
//! the deviation is the sum of the local errors, each carried to the end by the exact evolution.  U changes no norm,
//! so ||d_N|| <= sum_n ||delta_n||, with equality when all carried local errors point in the same direction.  The
//! ratio ||d_N|| / sum_n ||delta_n|| is called the coherence.  The program evaluates the sum mode by mode
//! (sum_n exp(+i E_j t_n) delta_(n,j) is carried to any time by one phase factor) and compares it with d_N.
//!
//! THE PREDICTION.  Every step is of a "kind": method, step size h, order, number of iterations of its implicit
//! equation, and for IDA the ratio of the leading coefficient cj to the cj of its matrix.  The module `formulas` gives
//! for a kind and a mode the relative error eps of one step.  The predicted relative error of the mode j at the end
//! is the sum of eps over all steps, and the predicted deviation is
//!     D = sqrt( sum_j |c_j|^2 |A_j|^2 ),    A_j = sum_n eps(-i E_j h_n; kind of step n) - i E_j dt .
//! The last term is the rounding of the time variable: the integrator adds h_n to its time in every step and rounds
//! the sum to 53 bits; the time t_N that it reports differs from the exact sum of its steps by dt = (sum_n h_n) - t_N
//! (measured here with double-double numbers).  The state belongs to the exact sum, the comparison is made at the
//! reported time, so every mode gets the phase error -i E_j dt.
use crate::dd::{Cdd, Dd};
use crate::formulas;
use t2core::c64::C64;
use t2core::cmat::{eigh, CMat};

/// the eigenvalues and eigenvectors of H and the mode amplitudes of the initial state
pub struct Spectrum {
    /// the eigenvalues E_j in increasing order
    pub e: Vec<f64>,
    /// the eigenvectors (columns)
    pub v: CMat,
    /// the conjugate transpose of v
    pub vdag: CMat,
    /// c_j = v_j^dagger w(0)
    pub c0: Vec<C64>,
}

impl Spectrum {
    pub fn new(h: &CMat, w0: &[C64]) -> Spectrum {
        let (e, v) = eigh(h);
        Spectrum::from_parts(e, v, w0)
    }
    /// from eigenpairs computed elsewhere (the columns of v are the eigenvectors)
    pub fn from_parts(e: Vec<f64>, v: CMat, w0: &[C64]) -> Spectrum {
        let vdag = v.dagger();
        let c0 = vdag.matvec(w0);
        Spectrum { e, v, vdag, c0 }
    }
    pub fn n(&self) -> usize {
        self.e.len()
    }
    /// the largest modulus of an eigenvalue
    pub fn emax(&self) -> f64 {
        self.e.iter().fold(0.0f64, |m, x| m.max(x.abs()))
    }
    /// the mode amplitudes v^dagger w of a state
    pub fn amplitudes(&self, w: &[C64]) -> Vec<C64> {
        self.vdag.matvec(w)
    }
    /// the state with the mode amplitudes a
    pub fn state(&self, a: &[C64]) -> Vec<C64> {
        self.v.matvec(a)
    }
    /// the exact solution at the time t
    pub fn exact(&self, t: f64) -> Vec<C64> {
        let a: Vec<C64> = (0..self.n()).map(|j| self.c0[j] * C64::cis(-self.e[j] * t)).collect();
        self.state(&a)
    }
    /// the moment sqrt( sum_j |c_j|^2 |E_j|^(2p) ) of the initial state: the norm of H^p w(0)
    pub fn moment(&self, p: i32) -> f64 {
        (0..self.n()).map(|j| self.c0[j].norm_sqr() * self.e[j].abs().powi(2 * p)).sum::<f64>().sqrt()
    }
}

/// the family of formulas of an integrator
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Method {
    /// Adams-Moulton formulas with fixed-point iteration (CVODE)
    Adams,
    /// backward differentiation formulas with Newton's method (IDA)
    Bdf,
}

/// what distinguishes steps with different errors
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StepKind {
    pub method: Method,
    /// the step size
    pub h: f64,
    /// the order of the formula
    pub order: i32,
    /// the number of iterations of the implicit equation (fixed-point iterations of CVODE, Newton steps of IDA)
    pub iterations: u32,
    /// IDA: cj/cjJ, the leading coefficient of the step divided by that of its matrix; CVODE: 1
    pub ratio: f64,
}

impl StepKind {
    /// the relative error eps of one step of this kind for the eigenvalue e (z = -i e h)
    pub fn eps(&self, e: f64) -> C64 {
        let z = Cdd { re: Dd::ZERO, im: Dd::from_f64(-e) * Dd::from_f64(self.h) };
        match self.method {
            Method::Adams => formulas::adams_eps(self.order as usize, z, Some(self.iterations)).to_c64(),
            Method::Bdf => formulas::bdf_eps(self.order as usize, z, self.ratio, self.iterations).to_c64(),
        }
    }
    /// the same with the implicit equation solved exactly (many fixed-point iterations; a current matrix)
    pub fn eps_exact(&self, e: f64) -> C64 {
        let z = Cdd { re: Dd::ZERO, im: Dd::from_f64(-e) * Dd::from_f64(self.h) };
        match self.method {
            Method::Adams => formulas::adams_eps(self.order as usize, z, None).to_c64(),
            Method::Bdf => formulas::bdf_eps(self.order as usize, z, 1.0, 1).to_c64(),
        }
    }
    fn same(&self, o: &StepKind) -> bool {
        self.method == o.method && self.h.to_bits() == o.h.to_bits() && self.order == o.order && self.iterations == o.iterations && (self.ratio - o.ratio).abs() <= 1e-12 * self.ratio.abs()
    }
}

/// the measurements of one internal step
#[derive(Clone, Debug)]
pub struct StepRecord {
    pub n: i64,
    /// the time at the end of the step, as the integrator reports it
    pub t: f64,
    pub kind: StepKind,
    /// the value of the local error test of the integrator (the norm of its estimate; at most 1)
    pub test: f64,
    /// the norm of the exact local error
    pub local: f64,
    /// the weighted root-mean-square norm of the exact local error with the tolerances of the step
    pub tau: f64,
    /// the norm of the deviation from the exact solution at the end of the step
    pub deviation: f64,
    /// the rounding of the time variable up to the end of the step: the exact sum of the sizes of the steps so far
    /// minus the reported time
    pub time_defect: f64,
    /// the cosine of the angle between the exact local error of this step and that of the step before, both carried
    /// to the same time by the exact evolution (the real part of their scalar product divided by the product of their
    /// norms): +1 if the two errors point in the same direction and add up, -1 if they are opposite and cancel.  NaN
    /// for the first step and in a light analysis.
    pub turn: f64,
}

/// the relative error of one mode at the end of an integration
#[derive(Clone, Debug)]
pub struct ModeError {
    /// the eigenvalue E_j
    pub e: f64,
    /// |c_j|
    pub weight: f64,
    /// measured: a_j(t_N) / (c_j exp(-i E_j t_N)) - 1
    pub measured: C64,
    /// predicted from the formulas: the sum of eps over the steps
    pub formula: C64,
    /// the same with every implicit equation solved exactly
    pub formula_exact: C64,
    /// the phase error of the rounded time, -i E_j dt
    pub time: C64,
}

/// the result of the analysis of one integration
#[derive(Clone, Debug)]
pub struct Run {
    /// one record per internal step (in a light analysis without the local error, its norm tau and the deviation,
    /// which are then NaN)
    pub records: Vec<StepRecord>,
    /// the kinds of the steps in the order of their first appearance, with the number of steps of each
    pub kinds: Vec<(StepKind, i64)>,
    /// the sequence of the kinds, run-length encoded
    pub segments: Vec<(StepKind, i64)>,
    pub modes: Vec<ModeError>,
    /// the number of internal steps
    pub nsteps: i64,
    /// the time at the end, as reported
    pub t_end: f64,
    /// the norm of the deviation at the end
    pub deviation: f64,
    /// the sum of the norms of the exact local errors (0 for a light analysis)
    pub sum_local: f64,
    /// the norm of the difference between the deviation at the end and the sum of the carried local errors
    pub identity_defect: f64,
    /// dt = (exact sum of the step sizes) - (reported time at the end)
    pub time_defect: f64,
    /// the predicted deviation from the formulas alone
    pub predicted_formula: f64,
    /// the predicted deviation from the formulas with every implicit equation solved exactly
    pub predicted_exact: f64,
    /// the deviation that the rounded time alone gives, |dt| times the norm of H w(0)
    pub predicted_time: f64,
    /// the predicted deviation, formulas and rounded time together
    pub predicted: f64,
    /// the norm of (measured - predicted) over the modes, weighted with |c_j|
    pub rest: f64,
}

impl Run {
    /// the coherence: deviation / sum of the local errors
    pub fn coherence(&self) -> f64 {
        if self.sum_local > 0.0 {
            self.deviation / self.sum_local
        } else {
            f64::NAN
        }
    }
    /// the kind with the most steps and the number of its steps
    pub fn main_kind(&self) -> (StepKind, i64) {
        *self.kinds.iter().max_by_key(|(_, n)| *n).expect("an integration has steps")
    }
    /// the step size and order with the most steps (over all numbers of iterations and ratios) and their number of steps
    pub fn main_size(&self) -> (f64, i32, i64) {
        let (k, _) = self.main_kind();
        let n = self.kinds.iter().filter(|(x, _)| x.h.to_bits() == k.h.to_bits() && x.order == k.order).map(|(_, n)| *n).sum();
        (k.h, k.order, n)
    }
    /// the records of the steps that have the main step size and order, without the first `skip` of them
    pub fn main_records(&self, skip: usize) -> Vec<&StepRecord> {
        let (h, order, _) = self.main_size();
        self.records.iter().filter(|r| r.kind.h.to_bits() == h.to_bits() && r.kind.order == order).skip(skip).collect()
    }
}

/// the median of a list (the mean of the two middle values for an even number); NaN for an empty list
pub fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    let mut v = values.to_vec();
    v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let m = v.len() / 2;
    if v.len() % 2 == 1 {
        v[m]
    } else {
        0.5 * (v[m - 1] + v[m])
    }
}

/// the analysis of one integration, fed step by step
pub struct Anatomy<'a> {
    spec: &'a Spectrum,
    full: bool,
    a_prev: Vec<C64>,
    t_prev: f64,
    time_sum: Dd,
    carried: Vec<C64>,
    /// the local error of the step before, carried back to the time 0, and its norm
    before: Vec<C64>,
    before_norm: f64,
    sum_local: f64,
    records: Vec<StepRecord>,
    kinds: Vec<(StepKind, i64)>,
    segments: Vec<(StepKind, i64)>,
    nsteps: i64,
}

impl<'a> Anatomy<'a> {
    /// `full`: measure the local error and the deviation at every step (two changes of basis per step); otherwise only
    /// the kinds of the steps are collected and the final state is analysed
    pub fn new(spec: &'a Spectrum, full: bool) -> Anatomy<'a> {
        Anatomy {
            spec,
            full,
            a_prev: spec.c0.clone(),
            t_prev: 0.0,
            time_sum: Dd::ZERO,
            carried: vec![C64::ZERO; spec.n()],
            before: vec![C64::ZERO; spec.n()],
            before_norm: 0.0,
            sum_local: 0.0,
            records: Vec::new(),
            kinds: Vec::new(),
            segments: Vec::new(),
            nsteps: 0,
        }
    }

    /// One internal step: its number n, the reported time t at its end, its kind, the value of the local error test,
    /// the state w at its end, and the tolerances of the real and of the imaginary part of every component that the
    /// step used (`tol_re[k]`, `tol_im[k]`).
    pub fn step(&mut self, n: i64, t: f64, kind: StepKind, test: f64, w: &[C64], tol_re: &[f64], tol_im: &[f64]) {
        self.nsteps += 1;
        self.time_sum = self.time_sum + Dd::from_f64(kind.h);
        match self.kinds.iter_mut().find(|(k, _)| k.same(&kind)) {
            Some((_, count)) => *count += 1,
            None => self.kinds.push((kind, 1)),
        }
        match self.segments.last_mut() {
            Some((k, count)) if k.same(&kind) => *count += 1,
            _ => self.segments.push((kind, 1)),
        }
        let time_defect = (self.time_sum - Dd::from_f64(t)).to_f64();
        if !self.full {
            self.records.push(StepRecord { n, t, kind, test, local: f64::NAN, tau: f64::NAN, deviation: f64::NAN, time_defect, turn: f64::NAN });
            self.t_prev = t;
            return;
        }
        let m = self.spec.n();
        let a = self.spec.amplitudes(w);
        // the step size as the difference of the reported times: the exact solution is compared at the reported times
        let dt = t - self.t_prev;
        let mut delta = vec![C64::ZERO; m];
        let (mut l2, mut d2) = (0.0f64, 0.0f64);
        // the scalar product of the local error of the step before with that of this step, both carried to the time 0
        let mut overlap = 0.0f64;
        for j in 0..m {
            delta[j] = a[j] - self.a_prev[j] * C64::cis(-self.spec.e[j] * dt);
            l2 += delta[j].norm_sqr();
            let at_zero = delta[j] * C64::cis(self.spec.e[j] * t);
            self.carried[j] += at_zero;
            overlap += self.before[j].re * at_zero.re + self.before[j].im * at_zero.im;
            self.before[j] = at_zero;
            d2 += (a[j] - self.spec.c0[j] * C64::cis(-self.spec.e[j] * t)).norm_sqr();
        }
        let turn = if self.before_norm > 0.0 && l2 > 0.0 { overlap / (self.before_norm * l2.sqrt()) } else { f64::NAN };
        self.before_norm = l2.sqrt();
        let dw = self.spec.state(&delta);
        let mut s = 0.0f64;
        for k in 0..m {
            s += (dw[k].re / tol_re[k]).powi(2) + (dw[k].im / tol_im[k]).powi(2);
        }
        let tau = (s / (2 * m) as f64).sqrt();
        self.sum_local += l2.sqrt();
        self.records.push(StepRecord { n, t, kind, test, local: l2.sqrt(), tau, deviation: d2.sqrt(), time_defect, turn });
        self.a_prev = a;
        self.t_prev = t;
    }

    /// The end of the integration: `w` is the state at the last reported time.
    pub fn finish(self, w: &[C64]) -> Run {
        let spec = self.spec;
        let m = spec.n();
        let t = self.t_prev;
        let a = if self.full { self.a_prev.clone() } else { spec.amplitudes(w) };
        let time_defect = (self.time_sum - Dd::from_f64(t)).to_f64();
        // the predicted relative error of every mode: the sum of eps over the kinds
        let mut formula = vec![C64::ZERO; m];
        let mut formula_exact = vec![C64::ZERO; m];
        for (kind, count) in &self.kinds {
            let c = *count as f64;
            for j in 0..m {
                formula[j] += kind.eps(spec.e[j]).scale(c);
                formula_exact[j] += kind.eps_exact(spec.e[j]).scale(c);
            }
        }
        let mut modes = Vec::with_capacity(m);
        let (mut dev2, mut pf2, mut pe2, mut pt2, mut p2, mut rest2, mut id2) = (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
        for j in 0..m {
            let exact = spec.c0[j] * C64::cis(-spec.e[j] * t);
            let measured = a[j] / exact - C64::ONE;
            let time = C64 { re: 0.0, im: -spec.e[j] * time_defect };
            let wgt = spec.c0[j].norm_sqr();
            dev2 += (a[j] - exact).norm_sqr();
            pf2 += wgt * formula[j].norm_sqr();
            pe2 += wgt * formula_exact[j].norm_sqr();
            pt2 += wgt * time.norm_sqr();
            p2 += wgt * (formula[j] + time).norm_sqr();
            rest2 += wgt * (measured - formula[j] - time).norm_sqr();
            if self.full {
                id2 += (a[j] - exact - self.carried[j] * C64::cis(-spec.e[j] * t)).norm_sqr();
            }
            modes.push(ModeError { e: spec.e[j], weight: spec.c0[j].abs(), measured, formula: formula[j], formula_exact: formula_exact[j], time });
        }
        Run {
            records: self.records,
            kinds: self.kinds,
            segments: self.segments,
            modes,
            nsteps: self.nsteps,
            t_end: t,
            deviation: dev2.sqrt(),
            sum_local: self.sum_local,
            identity_defect: id2.sqrt(),
            time_defect,
            predicted_formula: pf2.sqrt(),
            predicted_exact: pe2.sqrt(),
            predicted_time: pt2.sqrt(),
            predicted: p2.sqrt(),
            rest: rest2.sqrt(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use t2core::c64::c;

    /// a 2 x 2 Hermitian matrix, integrated by exact steps with a known relative error per step: the analysis must
    /// return that error, a coherence of 1, an exact identity and the moments of the state
    #[test]
    fn exact_steps_with_a_known_error() {
        let mut h = CMat::zeros(2);
        h.set(0, 0, c(1.0, 0.0));
        h.set(0, 1, c(0.3, -0.4));
        h.set(1, 0, c(0.3, 0.4));
        h.set(1, 1, c(-0.5, 0.0));
        let w0 = vec![c(0.6, 0.0), c(0.0, 0.8)];
        let spec = Spectrum::new(&h, &w0);
        assert!((spec.moment(0) - 1.0).abs() < 1e-14);
        let hw = h.matvec(&w0);
        let norm_hw = (hw[0].norm_sqr() + hw[1].norm_sqr()).sqrt();
        assert!((spec.moment(1) - norm_hw).abs() < 1e-13);
        let kind = StepKind { method: Method::Bdf, h: 0.125, order: 5, iterations: 1, ratio: 1.0 };
        let eps = 1e-7;
        let mut an = Anatomy::new(&spec, true);
        let mut a = spec.c0.clone();
        let mut w = w0.clone();
        let tol = vec![1e-6; 2];
        for n in 1..=80i64 {
            for j in 0..2 {
                a[j] = a[j] * C64::cis(-spec.e[j] * kind.h).scale(1.0 + eps);
            }
            w = spec.state(&a);
            an.step(n, n as f64 * kind.h, kind, 0.5, &w, &tol, &tol);
        }
        let run = an.finish(&w);
        assert_eq!(run.nsteps, 80);
        assert!((run.deviation - ((1.0 + eps).powi(80) - 1.0)).abs() < 1e-13);
        assert!(run.identity_defect < 1e-14, "identity defect {:e}", run.identity_defect);
        assert!((run.coherence() - 1.0).abs() < 1e-5, "coherence {}", run.coherence());
        assert!(run.time_defect.abs() < 1e-15);
        // the local error of the step n is eps (1 + eps)^(n-1); the four real unknowns have the tolerance 1e-6, so the
        // weighted root-mean-square norm is the norm divided by sqrt(4) 1e-6
        for r in &run.records {
            assert!((r.local - eps).abs() < 1e-11 && (r.tau - eps / 2e-6).abs() < 1e-6);
        }
        assert_eq!(run.kinds.len(), 1);
        assert_eq!(run.segments.len(), 1);
        assert_eq!(run.main_size(), (0.125, 5, 80));
        assert!((median(&[3.0, 1.0, 2.0]) - 2.0).abs() < 1e-15 && (median(&[4.0, 1.0, 2.0, 3.0]) - 2.5).abs() < 1e-15);
    }
}
