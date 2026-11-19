//! The integrations of this crate: CVODE and IDA run step by step on i dw/dx^0 = H w and analysed by `anatomy`.
use crate::anatomy::{Anatomy, Method, Run, Spectrum, StepKind};
use crate::ida::{self, IdaSettings, IdaStats, IdaStep};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use t2core::c64::C64;
use t2core::cmat::CMat;
use t2core::cvode::{self, ComplexRhs, CvodeSettings, CvodeStats, StepReport};

/// the result of one integration by CVODE: the analysis, the reports of the steps, the counters
pub struct CvodeRun {
    pub rtol: f64,
    pub run: Run,
    pub steps: Vec<StepReport>,
    pub stats: CvodeStats,
}

/// the result of one integration by IDA
pub struct IdaRun {
    pub rtol: f64,
    pub run: Run,
    pub steps: Vec<IdaStep>,
    pub stats: IdaStats,
}

/// The number of iterations of the accepted attempt of a step, from the number counted over all attempts of the step:
/// every rejected attempt made at least one iteration.  (Steps with rejected attempts occur only among the first
/// steps of an integration, where the step size is small and the error of a step negligible.)
fn accepted_iterations(total: i64, rejected: i64) -> u32 {
    (total - rejected).max(1) as u32
}

/// CVODE (Adams-Moulton formulas, fixed-point iteration, tolerances rtol |w_k| + atol for the real and the imaginary
/// part of the component k) from the state w0 at the time 0 to `tend`, step by step.  `rhs` makes the right side
/// dw/dx^0 = -i H w; `tfirst` is the first output time of the integration that is reproduced (CVODE uses it once, for
/// its first step size).  `full`: the complete analysis of every step.
pub fn run_cvode(spec: &Spectrum, w0: &[C64], tfirst: f64, tend: f64, settings: &CvodeSettings, rhs: ComplexRhs, full: bool) -> Result<CvodeRun, String> {
    let n = w0.len();
    let mut an = Anatomy::new(spec, full);
    let mut steps: Vec<StepReport> = Vec::new();
    let mut last = w0.to_vec();
    let (mut tol_re, mut tol_im) = (vec![0.0f64; n], vec![0.0f64; n]);
    let stats = cvode::integrate_steps(w0, 0.0, tfirst, tend, settings, rhs, |rep, w, weights| {
        let kind = StepKind { method: Method::Adams, h: rep.h, order: rep.q, iterations: accepted_iterations(rep.iterations, rep.rejected), ratio: 1.0 };
        if full {
            for k in 0..n {
                tol_re[k] = 1.0 / weights[2 * k];
                tol_im[k] = 1.0 / weights[2 * k + 1];
            }
        }
        an.step(rep.n, rep.t, kind, rep.test, w, &tol_re, &tol_im);
        steps.push(rep.clone());
        last.copy_from_slice(w);
    })?;
    Ok(CvodeRun { rtol: settings.rtol, run: an.finish(&last), steps, stats })
}

/// IDA (backward differentiation formulas, Newton's method, tolerances rtol |y_i| + atol of the real unknowns) from
/// the state w0 at the time 0 to `tend`, step by step.  `full`: the complete analysis of every step.
pub fn run_ida(h: &CMat, spec: &Spectrum, w0: &[C64], tend: f64, settings: &IdaSettings, full: bool) -> Result<IdaRun, String> {
    let n = w0.len();
    let mut an = Anatomy::new(spec, full);
    let mut steps: Vec<IdaStep> = Vec::new();
    let mut last = w0.to_vec();
    let stats = ida::integrate_steps(h, w0, tend, settings, |rep, w, tol| {
        let kind = StepKind { method: Method::Bdf, h: rep.h, order: rep.k, iterations: accepted_iterations(rep.newton_steps, rep.rejected), ratio: rep.ratio };
        an.step(rep.n, rep.t, kind, rep.test, w, &tol[..n], &tol[n..]);
        steps.push(rep.clone());
        last.copy_from_slice(w);
    })?;
    Ok(IdaRun { rtol: settings.rtol, run: an.finish(&last), steps, stats })
}

/// The jobs 0 ... count-1 on as many threads as the machine offers (at most `count`), each job by `job(index)`; the
/// results in the order of the indices.  Every job creates its own integrator, so nothing but read-only data is shared.
pub fn in_parallel<T: Send>(count: usize, job: impl Fn(usize) -> T + Sync) -> Vec<T> {
    let threads = std::thread::available_parallelism().map(|x| x.get()).unwrap_or(1).min(count.max(1));
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<Option<T>>> = Mutex::new((0..count).map(|_| None).collect());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= count {
                    break;
                }
                let r = job(i);
                results.lock().unwrap()[i] = Some(r);
            });
        }
    });
    results.into_inner().unwrap().into_iter().map(|x| x.expect("every job has run")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::anatomy::median;
    use t2core::c64::c;

    fn small_problem() -> (CMat, Vec<C64>) {
        // a Hermitian 4 x 4 matrix with eigenvalues of both signs and a normalized state
        let mut h = CMat::zeros(4);
        let d = [1.3, -0.4, 2.1, -1.7];
        for i in 0..4 {
            h.set(i, i, c(d[i], 0.0));
        }
        let off = [(0usize, 1usize, c(0.3, 0.2)), (1, 2, c(-0.5, 0.1)), (2, 3, c(0.2, -0.6)), (0, 3, c(0.4, 0.3))];
        for &(i, j, z) in &off {
            h.set(i, j, z);
            h.set(j, i, z.conj());
        }
        let w: Vec<C64> = vec![c(0.5, 0.1), c(-0.3, 0.4), c(0.2, -0.5), c(0.4, 0.2)];
        let nrm = w.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        (h, w.iter().map(|z| z.scale(1.0 / nrm)).collect())
    }

    /// IDA with a matrix that is always current, on a small problem: the deviation is the sum of the local errors
    /// carried to the end (identity); in the settled part (constant step size and order) the local errors add up in
    /// one direction (the growth of the deviation over the settled part is the sum of its local errors, up to twice
    /// the deviation at its start), and the estimate of IDA agrees with the exact local error in the norm of IDA
    /// within 20 per cent;
    /// the deviation predicted from the formulas agrees with the measured one within 2 per cent.  (The first steps,
    /// of low order, have larger local errors that point in other directions; over the whole integration the ratio of
    /// the deviation to the sum of all local errors is therefore below 1, here about 0.88.)
    #[test]
    fn ida_on_a_small_problem() {
        let (h, w0) = small_problem();
        let spec = Spectrum::new(&h, &w0);
        let mut set = IdaSettings::like_ndsolve(1e-8, 1e-10);
        set.delta_cj = 1e-9;
        let r = run_ida(&h, &spec, &w0, 20.0, &set, true).unwrap();
        assert_eq!(r.run.nsteps, r.stats.nsteps);
        assert!(r.run.identity_defect < 1e-12, "identity defect {:e}", r.run.identity_defect);
        assert!(r.run.coherence() > 0.8 && r.run.coherence() <= 1.0 + 1e-12, "coherence {}", r.run.coherence());
        let main = r.run.main_records(10);
        assert!(main.len() > 1000, "{} settled steps", main.len());
        // d_end = U d_start + (sum of the carried local errors of the settled part).  If these all point in one
        // direction the norm of their sum is the sum of their norms, and the triangle inequality gives
        //     sum - 2 ||d_start||  <=  ||d_end|| - ||d_start||  <=  sum .
        let start = main[0].deviation;
        let growth = main[main.len() - 1].deviation - start;
        let sum: f64 = main[1..].iter().map(|x| x.local).sum();
        assert!(growth <= sum * (1.0 + 1e-9) && growth >= 0.999 * sum - 2.0 * start, "growth of the deviation {:e}, sum of the local errors {:e}, deviation at the start {:e}", growth, sum, start);
        let ratio = median(&main.iter().map(|x| x.tau / x.test).collect::<Vec<f64>>());
        assert!((ratio - 1.0).abs() < 0.2, "exact local error / estimate = {}", ratio);
        assert!((r.run.predicted / r.run.deviation - 1.0).abs() < 0.02, "predicted {:e} measured {:e}", r.run.predicted, r.run.deviation);
        assert!(r.steps.iter().all(|s| s.test <= 1.0 && (s.ratio - 1.0).abs() < 1e-6));
    }

    /// CVODE on the same problem: the identity, a coherence between 0 and 1, every test value at most 1, and the rule
    /// for the number of fixed-point iterations
    #[test]
    fn cvode_on_a_small_problem() {
        let (h, w0) = small_problem();
        let spec = Spectrum::new(&h, &w0);
        let hh = h.clone();
        let rhs: ComplexRhs = Box::new(move |_t, w, dw| {
            let r = hh.matvec(w);
            for k in 0..w.len() {
                dw[k] = C64 { re: r[k].im, im: -r[k].re };
            }
        });
        let set = CvodeSettings::adams(1e-10, 1e-12);
        let r = run_cvode(&spec, &w0, 0.5, 20.0, &set, rhs, true).unwrap();
        assert_eq!(r.run.nsteps, r.stats.nsteps);
        assert!(r.run.identity_defect < 1e-12, "identity defect {:e}", r.run.identity_defect);
        assert!(r.run.coherence() > 0.0 && r.run.coherence() <= 1.0 + 1e-12);
        assert!(r.steps.iter().all(|s| s.test <= 1.0));
        // the rule of CVODE for the number of iterations: the first iterate is accepted if and only if the value of
        // the test made with it is at most one tenth.  A step with one iteration therefore has a test value of at
        // most 0.1 (a step with two iterations can end with a value below 0.1: the test value is that of the last
        // iterate)
        let main = r.run.main_records(10);
        assert!(main.iter().any(|x| x.kind.iterations == 1) && main.iter().any(|x| x.kind.iterations == 2));
        assert!(main.iter().all(|x| x.kind.iterations != 1 || x.test <= 0.1 * (1.0 + 1e-9)), "a step with one iteration has a test value above 0.1");
    }

    /// the jobs of `in_parallel` come back in order
    #[test]
    fn parallel_jobs_in_order() {
        let v = in_parallel(7, |i| i * i);
        assert_eq!(v, vec![0, 1, 4, 9, 16, 25, 36]);
    }
}
