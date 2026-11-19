//! The checks GS-17 to GS-20 of t2_fields: the time integrations of check GS-13 taken apart, step by step.
//!
//! Check GS-13 integrates the lattice equations i dw/dx^0 = H_lat w on the lattice of GS-10 with CVODE for three
//! relative tolerances and reports the deviation from the exact solution at the end, 1.2e-6, 1.7e-7 and 1.7e-10.  The
//! notebook 3+1super-loop.nb of the project repeats the check with IDA (the other integrator of SUNDIALS, the one the
//! Wolfram Language contains) and finds 7.1e-6, 9.3e-8 and 4.4e-11.  Neither set of numbers is proportional to the
//! tolerance, and IDA is six times worse than CVODE at the first tolerance and two to four times better at the others.
//! This module explains each of the six numbers by measuring every internal step of every integration:
//!
//!   GS-17  the deviation at the end is the sum of the exact local errors of the steps, each carried to the end by the
//!          exact evolution (an identity, verified to rounding), and so each of the six numbers is the product
//!          (number of steps) x (mean local error) x (coherence);
//!   GS-18  the local errors follow from the formulas: the relative error of one step of an eigenmode of H_lat is the
//!          difference between the factor of the formula and exp(-i E h), computed in the module t2steps::formulas from
//!          the characteristic equation of the formula AS THE INTEGRATOR APPLIES IT (number of iterations of the
//!          implicit equation; for IDA the age of its iteration matrix); summed over the steps and the modes, with the
//!          rounding of the time variable, this reproduces the measured deviations;
//!   GS-19  the step size is not fixed by the tolerance: both integrators leave the value of their local error test
//!          anywhere in a wide band below 1 (IDA between 0.0078 and 0.5, CVODE between about 0.01 and 1), so that the
//!          ratio deviation/tolerance changes by more than a factor of ten between neighbouring tolerances; the scan
//!          of 25 tolerances shows it for both integrators;
//!   GS-20  IDA run in the open (the pure-Rust IDA of SUNDIALS 7.8.0, module t2steps::ida) takes as many steps as the
//!          IDA inside the Wolfram Language, the same steps before the settled ones, and settled steps whose size
//!          differs from it by 7e-14, 3e-8 and 3e-8 of itself (see the last sentences); at rtol = 1e-10 its iteration
//!          matrix is 16 per cent out of date, which doubles the error of every step; at rtol = 1e-12 it is 31 per cent out of
//!          date, beyond the bound 1.305 at which one Newton step per step becomes unstable: an error that alternates
//!          in sign grows by 1.7 per cent per step, keeps the error estimate high and the steps small, and the
//!          deviation at the end is mostly the rounding of the time variable.  With a matrix that is always current
//!          the alternating error is gone, IDA takes steps of twice the size - and the deviation at rtol = 1e-12 is 8
//!          times LARGER, because the local errors of its steps then add up in phase.  The settled step size is the
//!          first step size 1e-4 times fixed factors of IDA (1/4, 0.9, 2) times ONE factor that IDA computes from the
//!          error estimate of one step; this estimate changes by parts in 1e8 when the iteration matrix is formed by
//!          difference quotients (as the notebook does) instead of exactly, and with it the settled step size.  At
//!          rtol = 1e-12 the rounding of the time depends on the last 16 binary digits of the settled step size: over
//!          all 65536 cases the deviation goes from 2.0e-11 to 1.2e-10, and the number of the notebook, 4.4e-11, is
//!          the median.
//!
//! The data of the figures and tables are written to test2/data/t2_steps_*.csv, the tables of the document to the
//! fragments of t2_fields.
use super::{sci, Checks, Frags};
use std::fs;
use std::path::Path;
use t2core::c64::C64;
use t2core::cmat::CMat;
use t2core::cvode::{ComplexRhs, CvodeSettings};
use t2core::lattice::{Fields, Lattice, Mats};
use t2steps::anatomy::{median, Method, Run, Spectrum, StepKind};
use t2steps::dd::{Cdd, Dd};
use t2steps::formulas;
use t2steps::ida::IdaSettings;
use t2steps::runs::{in_parallel, run_cvode, run_ida, CvodeRun, IdaRun};

/// The numbers of check GS-13 of the notebook 3+1super-loop.nb (IDA through NDSolve, Wolfram 15.0.1; the notebook
/// prints them, and its evaluation log 3+1super-loop_provenance/logs/IDA_lattice_probe.txt has them with all digits),
/// for rtol = 1e-8, 1e-10, 1e-12: the numbers of internal steps and of evaluations of the right side, the largest and
/// the smallest difference of two neighbouring step times, and the relative deviation at the time 20.
const NB_STEPS: [i64; 3] = [2705, 7309, 41111];
const NB_EVALS: [i64; 3] = [11182, 18096, 54585];
const NB_HMAX: [f64; 3] = [0.007489190703255488, 0.0032000000000000015, 0.0005832000000000003];
const NB_HMIN: [f64; 3] = [1.4627325592295171e-5, 1.5625e-6, 9.765625e-8];
const NB_DEV: [f64; 3] = [7.125333219739698e-6, 9.308767142501674e-8, 4.378788443188416e-11];

/// The settled steps of the IDA of the notebook, for rtol = 1e-8, 1e-10, 1e-12.  NDSolve returns the times at the ends
/// of the internal steps of IDA (the grid of the solution), not the step sizes.  The probe
/// 3+1super-loop_provenance/tools/gs13_ida_settled_steps.wl of PROJECT spacetimes reads from this grid what is listed
/// here (its output is the file 3+1super-loop_provenance/logs/IDA_settled_steps_probe.txt): the number of the first
/// step of the settled size and the number of settled steps; the grid point before the first and after the last
/// settled step; the smallest of the neighbouring machine numbers h for which repeated machine addition t <- t + h,
/// started at the grid point before the first settled step, reproduces every grid point of the settled steps bit for
/// bit, and how many such neighbours there are; the time defect of the settled steps (the number of settled steps
/// times h minus the advance of the grid, in exact arithmetic) for the smallest and for the largest of them; and the
/// sizes of the steps up to the first settled one (the machine numbers nearest to the exact differences of the grid
/// points).  A machine number that must be exact is written as the pair (k, e) for k 2^e.
const NB_FIRST_SETTLED: [i64; 3] = [42, 44, 41];
const NB_SETTLED: [usize; 3] = [2663, 7265, 41070];
const NB_T_BEFORE: [(f64, i32); 3] = [(7621335736825744.0, -57), (7915076345103650.0, -58), (5135769006344567.0, -60)];
const NB_T_AFTER: [f64; 3] = [19.996598482852086, 19.99879315696496, 19.99996058729621];
const NB_H_LOWEST: [(f64, i32); 3] = [(8634449013884513.0, -60), (6338713941217036.0, -61), (8981056297398640.0, -64)];
const NB_H_NUMBER: [usize; 3] = [4, 4, 8];
const NB_DEFECT: [(f64, f64); 3] = [(2.112815131183332e-13, 2.1821086604312256e-13), (1.8559164621789748e-12, 1.8653685367187034e-12), (7.19187990411152e-12, 7.207464768239913e-12)];
const NB_EARLY_8: [f64; 42] = [
    0.000025, 0.00001805842665715453, 0.000018058426657154535, 0.00001625258399143909, 0.000016252583991439076,
    0.000014627325592295171, 0.000014627325592295171, 0.000014627325592295171, 0.000029254651184590342,
    0.000029254651184590342, 0.000029254651184590342, 0.000058509302369180683, 0.000058509302369180683,
    0.00011701860473836142, 0.00023403720947672284, 0.00023403720947672273, 0.00023403720947672273,
    0.00023403720947672273, 0.0004680744189534457, 0.00046807441895344547, 0.00046807441895344547,
    0.00046807441895344547, 0.0009361488379068914, 0.0009361488379068909, 0.0009361488379068909,
    0.0009361488379068909, 0.0009361488379068909, 0.0018722976758137819, 0.0018722976758137819,
    0.0018722976758137819, 0.0018722976758137819, 0.0037445953516275637, 0.0037445953516275637,
    0.0037445953516275637, 0.0037445953516275637, 0.0037445953516275637, 0.0037445953516275637,
    0.0037445953516275637, 0.0037445953516275637, 0.0037445953516275637, 0.0037445953516275637,
    0.0074891907032551275,
];
const NB_EARLY_10: [f64; 44] = [
    1.5625e-6, 1.5625e-6, 1.5625000000000003e-6, 3.1249999999999997e-6, 3.1249999999999997e-6,
    3.1249999999999997e-6, 3.1250000000000014e-6, 6.2499999999999995e-6, 0.000012499999999999999,
    0.000024999999999999998, 0.00005000000000000001, 0.000049999999999999996, 0.000049999999999999996,
    0.000049999999999999996, 0.00009999999999999999, 0.00009999999999999999, 0.00010000000000000005,
    0.00010000000000000005, 0.00019999999999999998, 0.00019999999999999998, 0.0002000000000000001,
    0.0002000000000000001, 0.00039999999999999996, 0.0004000000000000002, 0.0004000000000000002,
    0.0004000000000000002, 0.0004000000000000002, 0.0007999999999999999, 0.0008000000000000004,
    0.0008000000000000004, 0.0008000000000000004, 0.0016000000000000007, 0.0016000000000000007,
    0.0016000000000000007, 0.0016000000000000007, 0.0016000000000000007, 0.0016000000000000007,
    0.0016000000000000007, 0.0016000000000000007, 0.0016000000000000007, 0.0016000000000000007,
    0.0016000000000000007, 0.0032000000000000015, 0.0027489789703326675,
];
const NB_EARLY_12: [f64; 41] = [
    9.765625e-8, 9.765625e-8, 9.765625000000002e-8, 1.9531249999999998e-7, 1.9531249999999998e-7,
    1.9531249999999998e-7, 3.9062499999999997e-7, 7.812499999999999e-7, 7.812499999999999e-7, 1.5624999999999999e-6,
    3.1250000000000006e-6, 6.250000000000001e-6, 0.000012499999999999999, 0.000012499999999999999,
    0.000012499999999999999, 0.000012499999999999999, 0.000024999999999999998, 0.000024999999999999998,
    0.00002500000000000001, 0.00002500000000000001, 0.000049999999999999996, 0.00004499999999999998,
    0.00004500000000000001, 0.00009000000000000002, 0.00009000000000000002, 0.00018000000000000004,
    0.00016200000000000003, 0.00016199999999999993, 0.00016200000000000003, 0.00016200000000000003,
    0.00016200000000000003, 0.00032400000000000007, 0.00032399999999999985, 0.00029159999999999993,
    0.00029159999999999993, 0.00029159999999999993, 0.00029159999999999993, 0.00029159999999999993,
    0.00029159999999999993, 0.0005832000000000003, 0.0004868640374427113,
];

/// the end of the integrations of GS-13 and its first output time
const T_END: f64 = 20.0;
const T_FIRST: f64 = 0.5;

/// A number as mantissa x 10^exponent for a formula of the documents, with `digits` decimals of the mantissa:
/// 1.185e-6 -> "{1.185\times10^{-6}}".  The braces make the number one unbreakable piece: in a table column that is
/// too narrow for it the number sticks out, which the layout check of the documents reports, instead of being
/// broken after the multiplication sign.
fn tex_sci(x: f64, digits: usize) -> String {
    if x == 0.0 || !x.is_finite() {
        return format!("{}", x);
    }
    let s = format!("{:.*e}", digits, x);
    let (m, e) = s.split_once('e').unwrap();
    format!("{{{}\\times10^{{{}}}}}", m, e)
}

/// A decimal number for a table of the documents, with `decimals` decimals.  A negative number is written as a
/// formula, so that its sign is printed as a minus sign and not as a hyphen; with `plus` a positive number is
/// written as a formula with a plus sign.
fn tex_signed(x: f64, decimals: usize, plus: bool) -> String {
    if x < 0.0 {
        format!("$-{:.*}$", decimals, -x)
    } else if plus {
        format!("$+{:.*}$", decimals, x)
    } else {
        format!("{:.*}", decimals, x)
    }
}

/// A number with the digits that identify it, as mantissa x 10^exponent for a formula of the documents:
/// 1.5625e-6 -> "{1.5625\times10^{-6}}" (the shortest decimal form that gives the machine number back).
fn tex_short(x: f64) -> String {
    let text = format!("{:e}", x);
    let (m, e) = text.split_once('e').expect("an exponent");
    format!("{{{}\\times10^{{{}}}}}", m, e)
}

/// The width, in points, that the columns of a table of the documents share: the text of a page is 498 pt wide
/// (letter paper with margins of 2 cm), and every column after the first costs 12 pt of space between the columns.
fn table_width(columns: usize) -> u32 {
    498 - 12 * (columns as u32 - 1)
}

/// The first two lines of a table of the documents: the line of the headings and the line of dashes below it.
///
/// The converter of the documents gives a column the fraction (dashes of the column)/(dashes of all columns) of the
/// width `table_width`, whatever the cells contain.  The widths are therefore stated here, next to the headings: each
/// column gets its heading and the width it is to have on the printed page, in points (an even number; one dash stands
/// for 2 pt).  The widths of a table must add up to `table_width` of its number of columns, so that every width is
/// the true one; the function stops the program otherwise.  A width is chosen such that no number of the column is
/// broken (the widest number of four digits and a power of ten takes 64 pt at the type size of the tables).
fn table_head(columns: &[(&str, u32)]) -> String {
    let total: u32 = columns.iter().map(|c| c.1).sum();
    assert!(columns.iter().all(|c| c.1 % 2 == 0 && c.1 >= 2), "the width of a table column must be an even number of points");
    assert_eq!(total, table_width(columns.len()), "the widths of the {} columns of a table (first heading: {}) must add up to the shared width", columns.len(), columns[0].0);
    let mut s = String::from("|");
    for c in columns {
        s.push_str(&format!(" {} |", c.0));
    }
    s.push_str("\n|");
    for c in columns {
        s.push_str(&"-".repeat((c.1 / 2) as usize));
        s.push('|');
    }
    s.push('\n');
    s
}

/// the label of a tolerance: 1e-8 -> "1e-8"
fn tol_label(rt: f64) -> String {
    format!("{:e}", rt)
}

/// the right side dw/dx^0 = -i H_lat w of GS-13, applied by the routine of the solvers
fn lattice_rhs(lat: &Lattice, fl: &Fields) -> ComplexRhs {
    let (lat, fl) = (lat.clone(), fl.clone());
    let mats = Mats::new();
    Box::new(move |_t, w, dw| {
        lat.apply(&fl, &mats, w, dw);
        for z in dw.iter_mut() {
            *z = C64 { re: z.im, im: -z.re };
        }
    })
}

/// the numbers of the steps of the main step size and order that made one, two, or more iterations
fn iteration_counts(run: &Run) -> (i64, i64, i64) {
    let (h, order, _) = run.main_size();
    let mut c = (0i64, 0i64, 0i64);
    for (k, n) in &run.kinds {
        if k.h.to_bits() == h.to_bits() && k.order == order {
            match k.iterations {
                1 => c.0 += n,
                2 => c.1 += n,
                _ => c.2 += n,
            }
        }
    }
    c
}

/// "regular": at least 95 per cent of the steps of the main step size and order are of one kind
fn regular(run: &Run) -> bool {
    let (_, _, n) = run.main_size();
    let (_, m) = run.main_kind();
    (m as f64) >= 0.95 * n as f64
}

/// The largest modulus of a root other than the principal one of the recurrence of the integrator with ONE iteration
/// per step, at the step size and order of most steps (and, for IDA, their ratio cj/cjJ), for the eigenvalue of the
/// largest modulus `emax`: above 1 the one-iteration steps of the integration are unstable.
fn other_root_one_iteration(run: &Run, emax: f64) -> f64 {
    let k = run.main_kind().0;
    match k.method {
        Method::Adams => formulas::adams_parasitic_modulus(k.order as usize, emax * k.h, 1),
        Method::Bdf => formulas::bdf_parasitic_modulus(k.order as usize, C64 { re: 0.0, im: -emax * k.h }, k.ratio, 1),
    }
}

/// the fraction of the steps of the main step size and order that made one iteration
fn fraction_one_iteration(run: &Run) -> f64 {
    let (i1, i2, i3) = iteration_counts(run);
    i1 as f64 / (i1 + i2 + i3) as f64
}

/// the band in which the rule for the next step size leaves the value of the local error test of a settled step
/// (constant step size and order): (lower, upper)
fn band(method: Method, order: i32) -> (f64, f64) {
    match method {
        // IDA doubles the step if (2 err + 0.0001)^(-1/(k+1)) >= 2 and shortens it if this number is <= 1
        Method::Bdf => ((0.5f64.powi(order + 1) - 1e-4) / 2.0, (1.0 - 1e-4) / 2.0),
        // CVODE lengthens the step if 1/((6 dsm)^(1/(q+1)) + 1e-6) >= 1.5; a step with dsm > 1 is rejected
        Method::Adams => ((1.0f64 / 1.5 - 1e-6).powi(order + 1) / 6.0, 1.0),
    }
}

/// the summary of one run for the tables and the data file
struct Summary {
    name: String,
    integrator: &'static str,
    matrix: &'static str,
    rtol: f64,
    run: Run,
    evals: i64,
    matrices: i64,
    test_median: f64,
    wrms_median: f64,
    /// the median over the settled steps of the cosine between the local errors of two consecutive steps
    turn_median: f64,
}

impl Summary {
    fn of_cvode(name: &str, r: &CvodeRun) -> Summary {
        let main = r.run.main_records(10);
        Summary {
            name: name.to_string(),
            integrator: "CVODE",
            matrix: "none",
            rtol: r.rtol,
            evals: r.stats.nfevals,
            matrices: 0,
            test_median: median(&main.iter().map(|x| x.test).collect::<Vec<f64>>()),
            wrms_median: median(&main.iter().map(|x| x.tau).collect::<Vec<f64>>()),
            turn_median: median(&main.iter().map(|x| x.turn).filter(|c| c.is_finite()).collect::<Vec<f64>>()),
            run: r.run.clone(),
        }
    }
    fn of_ida(name: &str, matrix: &'static str, r: &IdaRun) -> Summary {
        let main = r.run.main_records(10);
        Summary {
            name: name.to_string(),
            integrator: "IDA",
            matrix,
            rtol: r.rtol,
            evals: r.stats.nrevals,
            matrices: r.stats.njevals,
            test_median: median(&main.iter().map(|x| x.test).collect::<Vec<f64>>()),
            wrms_median: median(&main.iter().map(|x| x.tau).collect::<Vec<f64>>()),
            turn_median: median(&main.iter().map(|x| x.turn).filter(|c| c.is_finite()).collect::<Vec<f64>>()),
            run: r.run.clone(),
        }
    }
    /// The error budget of the integration, (2.40) of the document: the number of steps times the largest local error
    /// that the tolerances allow in one step, for a state of the norm 1 with `n` complex components and the absolute
    /// tolerance rtol/100 of these integrations.  If the error of every real unknown equals its tolerance, the norm of
    /// the error is sqrt(2) (rtol + atol sqrt(n)) with the weights of CVODE (one tolerance for the real and the
    /// imaginary part of a component, from the modulus of the component) and rtol + atol sqrt(2 n) with the weights
    /// of IDA (one tolerance for every real unknown, from its own modulus).
    fn budget(&self, n: usize) -> f64 {
        let atol = self.rtol / 100.0;
        let step = if self.integrator == "CVODE" { 2f64.sqrt() * (self.rtol + atol * (n as f64).sqrt()) } else { self.rtol + atol * (2.0 * n as f64).sqrt() };
        self.run.nsteps as f64 * step
    }
    fn mean_local_over_rtol(&self) -> f64 {
        self.run.sum_local / self.run.nsteps as f64 / self.rtol
    }
}

/// the leading term of the relative error of one step of a kind, divided by (E h)^(order+1): |c_AM,q| for the
/// Adams-Moulton formula and 1/(k+1) + ((rho-1)/rho) H_k for the backward differentiation formula with the fraction
/// rho of `iterations` Newton steps at z = 0
fn leading_constant(kind: &StepKind) -> f64 {
    match kind.method {
        Method::Adams => formulas::c_am(kind.order as usize)[kind.order as usize].to_f64().abs(),
        Method::Bdf => {
            let k = kind.order as usize;
            let rho = formulas::bdf_rho(k, Cdd::ZERO, kind.ratio, kind.iterations).to_c64().re;
            1.0 / (k + 1) as f64 + (rho - 1.0) / rho * formulas::harmonic(k).to_f64()
        }
    }
}

/// The value of the local error test that an exactly solved step of a kind gives, from the formula alone.  At a time t
/// the exact solution has the amplitude c exp(-i E t) along the eigenvector of the eigenvalue E; one step of the kind
/// commits on it the error c exp(-i E t) eps_step(E), with the relative error eps_step of the exactly solved formula
/// (for such a step the estimate of the integrator is the leading term of this error).  The function returns the
/// weighted root-mean-square norm of this error vector with the tolerances of the integrator for the exact solution at
/// the time t - CVODE: rtol |w_k| + atol for the real and the imaginary part of the component k; IDA: rtol |y| + atol
/// for every real unknown y - and of these numbers the median over the 81 times t = 0, 0.25, ..., 20 (the weights
/// change with the state, so the norm changes by some 10 per cent along the integration; the median is what the
/// measured median over the settled steps is compared with).
fn formula_test_value(spec: &Spectrum, kind: &StepKind, rtol: f64, atol: f64) -> f64 {
    let m = spec.n();
    let eps: Vec<C64> = (0..m).map(|j| kind.eps_exact(spec.e[j])).collect();
    let mut values = Vec::new();
    for it in 0..=80 {
        let t = 0.25 * it as f64;
        let amplitudes: Vec<C64> = (0..m).map(|j| spec.c0[j] * C64::cis(-spec.e[j] * t)).collect();
        let errors: Vec<C64> = (0..m).map(|j| amplitudes[j] * eps[j]).collect();
        let w = spec.state(&amplitudes);
        let dw = spec.state(&errors);
        let mut sum = 0.0f64;
        for k in 0..m {
            match kind.method {
                Method::Adams => {
                    let tol = rtol * w[k].abs() + atol;
                    sum += (dw[k].re / tol).powi(2) + (dw[k].im / tol).powi(2);
                }
                Method::Bdf => {
                    sum += (dw[k].re / (rtol * w[k].re.abs() + atol)).powi(2) + (dw[k].im / (rtol * w[k].im.abs() + atol)).powi(2);
                }
            }
        }
        values.push((sum / (2 * m) as f64).sqrt());
    }
    median(&values)
}

/// The rule of IDA for a second Newton step, for a step of the order `order` whose matrix is out of date by the ratio
/// `ratio` = cj/cjJ.  After the first Newton step of a step IDA stops if ss * delnrm is at most 0.33, where delnrm is
/// the weighted root-mean-square norm of the first correction and ss is a number that IDA keeps from the last step
/// with two Newton steps: ss = rate/(1 - rate), rate = the norm of the second correction divided by that of the first
/// (ss, delnrm and rate are names of the source text of IDA).  On the linear equations every Newton step with the old
/// matrix multiplies the distance from the exact solution of the formula by 1 - rho_one, rho_one = 2 ratio/(1 + ratio)
/// (for the small step sizes of interest |z_step| is negligible against H_har), so rate = |1 - rho_one|.  The test
/// value of a step is the norm of (accepted minus predicted value)/(order + 1); for the first iterate this is
/// delnrm/(order + 1).  Hence IDA makes a second Newton step exactly when the test value of the first iterate
/// exceeds 0.33/((order + 1) ss).  Returns (rho_one, rate, ss, this threshold).
fn second_newton_rule(ratio: f64, order: i32) -> (f64, f64, f64, f64) {
    let rho_one = 2.0 * ratio / (1.0 + ratio);
    let rate = (1.0 - rho_one).abs();
    let ss = rate / (1.0 - rate);
    (rho_one, rate, ss, 0.33 / ((order + 1) as f64 * ss))
}

/// the machine number k 2^e (exact: k is an integer below 2^53)
fn dyadic(x: (f64, i32)) -> f64 {
    x.0 * 2f64.powi(x.1)
}

/// `count` additions t <- fl(t + h) in double precision, as IDA advances its time (fl: the rounding to the nearest
/// machine number).  Returns the time reached and the sum over the additions of (t + h) - fl(t + h), the part of the
/// time defect that these steps make.  The rounding error of one addition is itself a machine number and is obtained
/// without error by the three subtractions below (the "two-sum" of Knuth); the errors are added as double-double
/// numbers.
fn add_steps(t0: f64, h: f64, count: usize) -> (f64, f64) {
    let mut t = t0;
    let mut defect = Dd::ZERO;
    for _ in 0..count {
        let sum = t + h;
        let part = sum - t;
        let err = (t - (sum - part)) + (h - part);
        defect = defect + Dd::from_f64(err);
        t = sum;
    }
    (t, defect.to_f64())
}

/// The two coefficients with which IDA scales its error estimate in the step number n (1, 2, ...) of the order k,
/// computed from the sizes of the steps: with psi_m = the sum of the sizes of the last m steps, the step n included
/// (before the first step IDA assumes a step of the size of the first),
///     ck    = max( | h/psi_(k+1) + sum_(m=1..k) (h/psi_m - 1/m) |,  h/psi_(k+1) )     (the coefficient of the test),
///     sigma = k! h^k / (psi_2 psi_3 ... psi_(k+1))                                    (the coefficient of the rule
///                                                                                      for the next step size).
/// Both are 1/(k + 1) if the last k + 1 steps have the same size.  (ck, sigma and psi are names of the source text of
/// IDA, routine IDASetCoeffs.)
fn ida_coefficients(sizes: &[f64], n: usize, k: usize) -> (f64, f64) {
    let h = sizes[n - 1];
    let mut psi = Vec::with_capacity(k + 1);
    let mut sum = 0.0f64;
    for m in 1..=k + 1 {
        sum += if n >= m { sizes[n - m] } else { sizes[0] };
        psi.push(sum);
    }
    let mut sigma = 1.0f64;
    let mut test = h / psi[k];
    for m in 1..=k {
        sigma *= m as f64 * h / psi[m];
        test += h / psi[m - 1] - 1.0 / m as f64;
    }
    (test.abs().max(h / psi[k]), sigma)
}

/// How the settled step size of an integration by IDA comes about: the changes of the step size from the first step
/// that IDA tries (of the size h0) to the first step of the settled size.  IDA changes the step size
///   * after an attempt that its local error test rejects: by the factor 0.9 (2 e + 1e-4)^(-1/(k+1)), not above 0.9
///     and not below 1/4, at the first rejection of a step, and by 1/4 at every further one;
///   * after an accepted step: with eta = (2 e + 1e-4)^(-1/(k+1)), by the factor 2 if eta >= 2, by the factor eta, not
///     above 0.9 and not below 0.5, if eta <= 1, and not at all otherwise.
/// Here k is the order of the next step and e the estimate of IDA for it: if the order stays, the test value of the
/// step times sigma/ck (function `ida_coefficients`).
struct History {
    /// the number of the first step of the settled size, and the number of steps of this size
    first: i64,
    settled: usize,
    /// the rejected attempts of the first step (each divided the step size by 4)
    quarters: u32,
    /// the shortenings by the fixed factor 0.9 and the doublings after accepted steps
    tenths: u32,
    doublings: u32,
    /// the changes by a factor that IDA computed from its estimate: (the number of the step after which, the order of
    /// that step, its test value, the estimate e, the factor)
    computed: Vec<(i64, i32, f64, f64, f64)>,
    /// the largest relative difference between such a factor and the rule applied to the recorded numbers
    rule_defect: f64,
    /// the changes of the step size that are none of these
    other: u32,
    /// h0 times the product of all factors in their order, divided by the settled step size, minus 1
    product_defect: f64,
    /// the product of the fixed factors alone: h0 (1/4)^quarters 0.9^tenths 2^doublings
    fixed_product: f64,
    /// the time at the end of the last step before the settled ones, and the time defect up to there
    t_before: f64,
    defect_before: f64,
}

fn history(r: &IdaRun, h0: f64) -> History {
    let st = &r.steps;
    let main_h = r.run.main_size().0;
    let first = st.iter().position(|x| x.h.to_bits() == main_h.to_bits()).expect("a settled step");
    let settled = st.iter().filter(|x| x.h.to_bits() == main_h.to_bits()).count();
    let quarters = ((h0 / st[0].h).log2() / 2.0).round() as u32;
    let mut product = h0 * 0.25f64.powi(quarters as i32);
    let mut out = History { first: st[first].n, settled, quarters, tenths: 0, doublings: 0, computed: Vec::new(), rule_defect: 0.0, other: 0, product_defect: 0.0, fixed_product: 0.0, t_before: 0.0, defect_before: 0.0 };
    if product.to_bits() != st[0].h.to_bits() || st[0].rejected != quarters as i64 {
        out.other += 1;
    }
    let mut fixed = product;
    for j in 0..first {
        let (a, b) = (&st[j], &st[j + 1]);
        if b.h.to_bits() == a.h.to_bits() {
            continue;
        }
        if b.rejected != 0 || a.h_next.to_bits() != b.h.to_bits() {
            out.other += 1;
        } else if b.h.to_bits() == (2.0 * a.h).to_bits() {
            out.doublings += 1;
            product *= 2.0;
            fixed *= 2.0;
        } else if b.h.to_bits() == (a.h * 0.9).to_bits() {
            out.tenths += 1;
            product *= 0.9;
            fixed *= 0.9;
        } else if a.k_next == a.k {
            let estimate = a.err_k();
            let rule = (2.0 * estimate + 1e-4).powf(-1.0 / (a.k as f64 + 1.0));
            let factor = b.h / a.h;
            out.rule_defect = out.rule_defect.max((rule / factor - 1.0).abs());
            out.computed.push((a.n, a.k, a.test, estimate, factor));
            product *= factor;
        } else {
            out.other += 1;
        }
    }
    out.product_defect = product / main_h - 1.0;
    out.fixed_product = fixed;
    if first > 0 {
        out.t_before = st[first - 1].t;
        out.defect_before = r.run.records[first - 1].time_defect;
    }
    out
}

/// The checks GS-17 to GS-20.  `gs13_at_20` are the relative deviations that check GS-13 found at its last output time,
/// 20, for the three tolerances (the solution of CVODE interpolated to that time).
#[allow(clippy::too_many_arguments)]
pub fn run(ck: &mut Checks, fr: &mut Frags, dd: &Path, lat: &Lattice, fl: &Fields, hl: &CMat, w0: &[C64], e: &[f64], v: &CMat, gs13_at_20: &[f64; 3]) {
    let spec = Spectrum::from_parts(e.to_vec(), v.clone(), w0);
    let n = spec.n();
    let emax = spec.emax();
    let rtols = [1e-8, 1e-10, 1e-12];

    // ------------------------------------------------------------------ the integrations
    // the three integrations of GS-13 by CVODE and the same three by IDA set up as NDSolve sets it up, each with the
    // complete analysis of every step; IDA with an iteration matrix that is computed anew whenever cj changes;
    let cv: Vec<CvodeRun> = rtols.iter().map(|&rt| run_cvode(&spec, w0, T_FIRST, T_END, &CvodeSettings::adams(rt, rt / 100.0), lattice_rhs(lat, fl), true).expect("CVODE step by step")).collect();
    // and IDA with its default rule, but with the matrix that it forms itself by difference quotients
    let jobs: Vec<(f64, f64, bool)> = rtols.iter().flat_map(|&rt| [(rt, 0.25, false), (rt, 1e-9, false), (rt, 0.25, true)]).collect();
    let mut ida_all: Vec<IdaRun> = in_parallel(jobs.len(), |i| {
        let mut set = IdaSettings::like_ndsolve(jobs[i].0, jobs[i].0 / 100.0);
        set.delta_cj = jobs[i].1;
        set.dq_matrix = jobs[i].2;
        run_ida(hl, &spec, w0, T_END, &set, true).expect("IDA step by step")
    });
    let mut ida: Vec<IdaRun> = Vec::new();
    let mut idc: Vec<IdaRun> = Vec::new();
    let mut idq: Vec<IdaRun> = Vec::new();
    for (i, r) in ida_all.drain(..).enumerate() {
        match i % 3 {
            0 => ida.push(r),
            1 => idc.push(r),
            _ => idq.push(r),
        }
    }
    let mut sums: Vec<Summary> = Vec::new();
    for (i, &rt) in rtols.iter().enumerate() {
        sums.push(Summary::of_cvode(&format!("cvode_{}", tol_label(rt)), &cv[i]));
    }
    for (i, &rt) in rtols.iter().enumerate() {
        sums.push(Summary::of_ida(&format!("ida_{}", tol_label(rt)), "default", &ida[i]));
    }
    for (i, &rt) in rtols.iter().enumerate() {
        sums.push(Summary::of_ida(&format!("ida_current_{}", tol_label(rt)), "current", &idc[i]));
    }

    // ------------------------------------------------------------------ the scan of the tolerance
    // 25 tolerances from 1e-6 to 1e-12, four per decade; the three tolerances of GS-13 are among them and are not
    // integrated again
    let mantissa = [1.0, 0.5623413251903491, 0.31622776601683794, 0.17782794100389229];
    let scan_rtols: Vec<f64> = (0..25).map(|i| mantissa[i % 4] * format!("1e-{}", 6 + i / 4).parse::<f64>().unwrap()).collect();
    let is_table = |rt: f64| rtols.iter().position(|&x| x == rt);
    let scan_cv: Vec<Run> = in_parallel(scan_rtols.len(), |i| {
        let rt = scan_rtols[i];
        match is_table(rt) {
            Some(k) => cv[k].run.clone(),
            None => run_cvode(&spec, w0, T_FIRST, T_END, &CvodeSettings::adams(rt, rt / 100.0), lattice_rhs(lat, fl), false).expect("CVODE scan").run,
        }
    });
    let scan_ida: Vec<Run> = in_parallel(scan_rtols.len(), |i| {
        let rt = scan_rtols[i];
        match is_table(rt) {
            Some(k) => ida[k].run.clone(),
            None => run_ida(hl, &spec, w0, T_END, &IdaSettings::like_ndsolve(rt, rt / 100.0), false).expect("IDA scan").run,
        }
    });

    // ------------------------------------------------------------------ data files
    let mut s = String::from("# content: the time integrations of check GS-13 taken apart (checks GS-17 to GS-20): one line for each integration of the lattice equations on the 3 x 2 x 4 lattice of check GS-10 from the time 0 to 20 - CVODE (integration cvode_*), IDA set up like NDSolve of the Wolfram Language (integration ida_*: first step 1e-4, matrix recomputed when cj/cjJ leaves the interval 0.6 to 5/3) and IDA with a matrix that is recomputed whenever cj changes (integration ida_current_*), for rtol = 1e-8, 1e-10, 1e-12 (atol = rtol/100) - with the number of internal steps, the step size and the order of most steps and the number of these steps, how many of them made one and two iterations of the implicit equation, cj/cjJ of most steps, the deviation from the exact solution at the end, the sum of the norms of the exact local errors of all steps, the coherence (their ratio), the defect of the identity deviation = sum of the carried local errors, the rounding of the time variable (exact sum of the step sizes minus the reported time), the medians over the settled steps of the value of the local error test, of the same norm of the exact local error and of the cosine of the angle between the exact local errors of two consecutive steps (both carried to the same time by the exact evolution), the predicted deviation and its parts (from the formulas as applied; from the formulas with every implicit equation solved exactly; from the rounding of the time), the norm of the difference between the measured and the predicted errors of the modes, and the numbers of evaluations of the right side and of the matrix; the parameter second_newton_threshold is the value of the local error test of a first Newton iterate above which IDA makes a second Newton step in the settled steps of its integration at rtol = 1e-12 with the default rule\n");
    {
        // the test value above which IDA makes a second Newton step in the settled steps of its integration at
        // rtol = 1e-12 (Figure 2C(a) of the document)
        let k = ida[2].run.main_kind().0;
        s.push_str(&format!("# second_newton_threshold: {:.6}\n", second_newton_rule(k.ratio, k.order).3));
    }
    s.push_str("integration,integrator,matrix_rule,rtol,atol,N_steps,h_step,order,main_steps,one_iteration,two_iterations,cj_ratio,max_deviation,sum_local_error,coherence,identity_defect,time_defect,test_value_median,local_wrms_median,local_cosine_median,predicted_deviation,predicted_formula,predicted_exact_solve,predicted_time,rest,N_evals,N_matrix\n");
    for x in &sums {
        let (h, order, nmain) = x.run.main_size();
        let (i1, i2, _) = iteration_counts(&x.run);
        s.push_str(&format!(
            "{},{},{},{:e},{:e},{},{:.17e},{},{},{},{},{:.12},{:.9e},{:.9e},{:.6},{:.3e},{:.6e},{:.6},{:.6},{:.6},{:.9e},{:.9e},{:.9e},{:.9e},{:.6e},{},{}\n",
            x.name, x.integrator, x.matrix, x.rtol, x.rtol / 100.0, x.run.nsteps, h, order, nmain, i1, i2, x.run.main_kind().0.ratio, x.run.deviation, x.run.sum_local, x.run.coherence(),
            x.run.identity_defect, x.run.time_defect, x.test_median, x.wrms_median, x.turn_median, x.run.predicted, x.run.predicted_formula, x.run.predicted_exact, x.run.predicted_time, x.run.rest, x.evals, x.matrices
        ));
    }
    fs::write(dd.join("t2_steps_runs.csv"), s).expect("write t2_steps_runs.csv");

    let mut s = String::from("# content: the sequence of the internal steps of the nine integrations of the file t2_steps_runs.csv, run-length encoded (checks GS-17 to GS-20): each line is a group of consecutive steps of the same kind - the same step size, order, number of iterations of the implicit equation (fixed-point iterations of CVODE, Newton steps of IDA) and, for IDA, the same ratio cj/cjJ of the leading coefficient of the step to that of its iteration matrix (1 for CVODE) - with the number of steps of the group; the step sizes are printed with 17 significant digits, which determine the double-precision number\n");
    s.push_str("integration,h_step,order,iterations,cj_ratio,count\n");
    for x in &sums {
        for (k, c) in &x.run.segments {
            s.push_str(&format!("{},{:.17e},{},{},{:.15},{}\n", x.name, k.h, k.order, k.iterations, k.ratio, c));
        }
    }
    fs::write(dd.join("t2_steps_segments.csv"), s).expect("write t2_steps_segments.csv");

    let mut s = String::from("# content: the internal steps of the nine integrations of the file t2_steps_runs.csv (checks GS-17 to GS-20): the number of the step, the time at its end as the integrator reports it, its size, order, number of iterations and cj/cjJ, the value of the local error test of the integrator (the weighted root-mean-square norm of its estimate of the local error), the same norm of the exact local error, the norm of the exact local error, the cosine of the angle between the exact local error of the step and that of the step before (both carried to the same time by the exact evolution; 0 for the first step), the norm of the deviation from the exact solution at the end of the step, and the rounding of the time variable up to the end of the step (the exact sum of the step sizes so far minus the reported time); all steps of an integration with at most 3000 steps, otherwise the first 1200 steps, then every 25th step, and the last 100 steps\n");
    s.push_str("integration,step,t_step,h_step,order,iterations,cj_ratio,test_value,local_wrms,local_error,local_cosine,deviation,time_defect\n");
    for x in &sums {
        let total = x.run.records.len();
        for (i, r) in x.run.records.iter().enumerate() {
            if total <= 3000 || i < 1200 || i + 100 >= total || (i + 1) % 25 == 0 {
                s.push_str(&format!("{},{},{:.15e},{:.9e},{},{},{:.9},{:.6e},{:.6e},{:.6e},{:.5},{:.6e},{:.4e}\n", x.name, r.n, r.t, r.kind.h, r.kind.order, r.kind.iterations, r.kind.ratio, r.test, r.tau, r.local, if r.turn.is_finite() { r.turn } else { 0.0 }, r.deviation, r.time_defect));
            }
        }
    }
    fs::write(dd.join("t2_steps_trace.csv"), s).expect("write t2_steps_trace.csv");

    let mut s = String::from("# content: the relative error of every eigenmode of the lattice operator at the end of the last internal step (the time 20 for IDA; less than one step size after it for CVODE) of the nine integrations of the file t2_steps_runs.csv (checks GS-17 to GS-20): the eigenvalue of the mode, the modulus of the amplitude of the initial state along its eigenvector, the measured relative error of the amplitude (computed amplitude divided by the exact one, minus 1; real and imaginary part), the error predicted from the formulas as the integrator applies them (the sum over the steps of the relative error of one step), the same with every implicit equation solved exactly, and the imaginary part of the error from the rounding of the time variable (minus the eigenvalue times the time defect)\n");
    s.push_str("integration,E_qp,weight,measured_re,measured_im,formula_re,formula_im,exact_solve_re,exact_solve_im,time_im\n");
    for x in &sums {
        for m in &x.run.modes {
            s.push_str(&format!("{},{:.15e},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e},{:.9e}\n", x.name, m.e, m.weight, m.measured.re, m.measured.im, m.formula.re, m.formula.im, m.formula_exact.re, m.formula_exact.im, m.time.im));
        }
    }
    fs::write(dd.join("t2_steps_modes.csv"), s).expect("write t2_steps_modes.csv");

    let mut s = String::from("# content: the scan of the relative tolerance (check GS-19): the integration of check GS-13 (lattice of GS-10, time 0 to 20, atol = rtol/100) by CVODE and by IDA set up like NDSolve, for 25 relative tolerances from 1e-6 to 1e-12 (four per decade) - the number of internal steps, the step size and the order of most steps and the number of these steps, how many of them made one and two iterations, cj/cjJ of most steps (1 for CVODE), the deviation from the exact solution at the end, the predicted deviation and two of its parts, the norm of the difference between the measured and the predicted errors of the modes, the median of the value of the local error test over the settled steps, the rounding of the time variable, whether at least 95 per cent of the settled steps are of one kind (regular = 1), and the largest modulus of a root other than the principal one of the recurrence of the integrator with one iteration per step at the settled step size and order for the eigenvalue of the largest modulus (above 1: the steps with one iteration are unstable)\n");
    s.push_str("integrator,rtol,N_steps,h_step,order,main_steps,one_iteration,two_iterations,cj_ratio,max_deviation,predicted_deviation,predicted_exact_solve,predicted_time,rest,test_value_median,time_defect,regular,other_root\n");
    for (i, &rt) in scan_rtols.iter().enumerate() {
        for (name, r) in [("CVODE", &scan_cv[i]), ("IDA", &scan_ida[i])] {
            let (h, order, nmain) = r.main_size();
            let (i1, i2, _) = iteration_counts(r);
            let tm = median(&r.main_records(10).iter().map(|x| x.test).collect::<Vec<f64>>());
            s.push_str(&format!(
                "{},{:.6e},{},{:.9e},{},{},{},{},{:.9},{:.9e},{:.9e},{:.9e},{:.9e},{:.6e},{:.6},{:.6e},{},{:.6}\n",
                name, rt, r.nsteps, h, order, nmain, i1, i2, r.main_kind().0.ratio, r.deviation, r.predicted, r.predicted_exact, r.predicted_time, r.rest, tm, r.time_defect, if regular(r) { 1 } else { 0 }, other_root_one_iteration(r, emax)
            ));
        }
    }
    fs::write(dd.join("t2_steps_scan.csv"), s).expect("write t2_steps_scan.csv");

    // the relative error of one step of the formulas on a grid of |z| (for the figure and for the independent check
    // with multiple-precision arithmetic in Python); every number as the two parts hi and lo of its double-double form
    let mut s = String::from("# content: the relative error of one step of a linear multistep formula on the oscillation exp(z n) with z = -i |z| (checks GS-18 and GS-20), eps = kappa exp(-z) - 1 with the factor kappa of the formula, computed with double-double numbers: for the Adams-Moulton formulas of the orders 1 to 12 with the implicit equation solved exactly (iterations = 0) and with 1, 2 and 3 fixed-point iterations, and for the backward differentiation formulas of the orders 1 to 5 with one and two Newton steps and the ratios cj/cjJ = 1, 10/9, 1.164068524711, 1.312865875279 and 0.6088888888888889 of the leading coefficient to that of the iteration matrix; real and imaginary part, each as the sum of the two columns hi and lo\n");
    s.push_str("formula,order,iterations,cj_ratio,z_abs,eps_re_hi,eps_re_lo,eps_im_hi,eps_im_lo\n");
    let zgrid = [0.005, 0.01, 0.02, 0.04, 0.08, 0.12];
    let mut formula_rows = 0usize;
    for q in 1..=12usize {
        for m in [0u32, 1, 2, 3] {
            for &za in &zgrid {
                let z = Cdd { re: Dd::ZERO, im: Dd::from_f64(-za) };
                let eps = formulas::adams_eps(q, z, if m == 0 { None } else { Some(m) });
                s.push_str(&format!("AM,{},{},1,{},{:.17e},{:.17e},{:.17e},{:.17e}\n", q, m, za, eps.re.hi, eps.re.lo, eps.im.hi, eps.im.lo));
                formula_rows += 1;
            }
        }
    }
    let ratios = [1.0, 10.0 / 9.0, 1.164068524711, 1.312865875279, 0.6088888888888889];
    for k in 1..=5usize {
        for m in [1u32, 2] {
            for &r in &ratios {
                for &za in &zgrid {
                    let z = Cdd { re: Dd::ZERO, im: Dd::from_f64(-za) };
                    let eps = formulas::bdf_eps(k, z, r, m);
                    s.push_str(&format!("BDF,{},{},{:.16},{},{:.17e},{:.17e},{:.17e},{:.17e}\n", k, m, r, za, eps.re.hi, eps.re.lo, eps.im.hi, eps.im.lo));
                    formula_rows += 1;
                }
            }
        }
    }
    fs::write(dd.join("t2_steps_formulas.csv"), s).expect("write t2_steps_formulas.csv");

    // the stability of the schemes with few iterations: the largest modulus of a root other than the principal one
    let mut s = String::from("# content: the stability of the formulas as the integrators apply them with few iterations (checks GS-18 to GS-20): the largest modulus of a root other than the principal one of the recurrence (above 1: an error that is not the solution grows from step to step) - for the Adams-Moulton formulas of the orders 1 to 12 with 1 and 2 fixed-point iterations (CVODE) as a function of |z| from 0.005 to 0.2 on the imaginary axis (cj_ratio = 1), and for the backward differentiation formulas of the orders 2 to 5 with 1 Newton step (IDA) at z = 0 as a function of the ratio cj/cjJ from 0.6 to 1.66; for the latter also the root on the negative real axis (the alternating root; 0 where there is none)\n");
    s.push_str("formula,order,iterations,cj_ratio,z_abs,other_root,alternating_root\n");
    for q in 1..=12usize {
        for m in [1u32, 2] {
            for i in 1..=40 {
                let za = 0.005 * i as f64;
                s.push_str(&format!("AM,{},{},1,{:.3},{:.9},0\n", q, m, za, formulas::adams_parasitic_modulus(q, za, m)));
            }
        }
    }
    for k in 2..=5usize {
        for i in 0..=106 {
            let r = 0.60 + 0.01 * i as f64;
            s.push_str(&format!("BDF,{},1,{:.2},0,{:.9},{:.9}\n", k, r, formulas::bdf_parasitic_modulus(k, C64::ZERO, r, 1), formulas::bdf_alternating_root(k, r).unwrap_or(0.0)));
        }
    }
    fs::write(dd.join("t2_steps_roots.csv"), s).expect("write t2_steps_roots.csv");

    // the limits of stability
    let mut s = String::from("# content: the limits of stability of the formulas as the integrators apply them (checks GS-18 to GS-20): for the Adams-Moulton formulas of the orders 1 to 12 with 1, 2 and 3 fixed-point iterations (CVODE) the smallest |z| on the imaginary axis at which a root other than the principal one reaches the modulus 1 (limit_low = 0, limit_high = that |z|; 1 if there is none up to |z| = 1), and for the backward differentiation formulas of the orders 1 to 5 with one Newton step (IDA) the interval of the ratio cj/cjJ, inside the interval 0.6 to 5/3 that IDA tolerates, in which all other roots are inside the unit circle at z = 0\n");
    s.push_str("formula,order,iterations,limit_low,limit_high\n");
    for q in 1..=12usize {
        for m in [1u32, 2, 3] {
            s.push_str(&format!("AM,{},{},0,{:.9}\n", q, m, formulas::adams_stability_limit(q, m).unwrap_or(1.0)));
        }
    }
    for k in 1..=5usize {
        let (lo, hi) = formulas::bdf_stability_window(k);
        s.push_str(&format!("BDF,{},1,{:.9},{:.9}\n", k, lo, hi));
    }
    fs::write(dd.join("t2_steps_limits.csv"), s).expect("write t2_steps_limits.csv");

    // ------------------------------------------------------------------ GS-17: the identity and the decomposition
    let six: Vec<&Summary> = sums.iter().take(6).collect();
    let identity_ok = sums.iter().all(|x| x.run.identity_defect <= 1e-16 * x.run.nsteps as f64);
    let largest_identity = sums.iter().map(|x| x.run.identity_defect / (1e-16 * x.run.nsteps as f64)).fold(0.0f64, f64::max);
    // the product (number of steps) x (mean local error) x (coherence) is the deviation by the definitions; it is
    // evaluated from the three printed factors to show that the rounded factors of the table reproduce the deviation
    let product_dev = six.iter().map(|x| ((x.run.nsteps as f64 * (x.run.sum_local / x.run.nsteps as f64) * x.run.coherence()) / x.run.deviation - 1.0).abs()).fold(0.0f64, f64::max);
    let coherent = |x: &Summary| x.run.coherence() > 0.99;
    let n_coherent = six.iter().filter(|x| coherent(x)).count();
    // GS-13 itself: the deviations at the end of the three integrations by CVODE, as GS-13 reports them (1.2e-6,
    // 1.7e-7, 1.7e-10), and its numbers of steps
    // why the coherence is what it is: the direction of the local errors of consecutive steps.  Where the coherence is
    // near 1 consecutive local errors are parallel; where it is small they are turned against each other.
    let cosines_ok = sums.iter().all(|x| (x.run.coherence() <= 0.95 || x.turn_median > 0.99) && (x.run.coherence() >= 0.5 || x.turn_median < 0.0)) && sums[5].turn_median < -0.9;
    // where an integration ends.  IDA is told to stop at the time 20 and ends its last step there.  CVODE, run as in
    // GS-13, takes its steps past an output time and interpolates back, so that its last internal step ends less than
    // one step size after the time 20; the deviation measured there is compared with the one that GS-13 reports for
    // the interpolated solution at the time 20
    let end_times: Vec<f64> = sums.iter().map(|x| x.run.records.last().expect("a step").t).collect();
    let ends_ok = (0..3).all(|i| end_times[i] > T_END && end_times[i] - T_END < cv[i].run.main_size().0) && (3..9).all(|i| end_times[i] == T_END);
    let gs13_rel: Vec<f64> = (0..3).map(|i| cv[i].run.deviation / gs13_at_20[i] - 1.0).collect();
    let gs13_same = ends_ok && gs13_rel.iter().all(|x| x.abs() < 2e-3);
    // the six deviations against their error budgets
    let budget_shares: Vec<f64> = six.iter().map(|x| x.run.deviation / x.budget(n)).collect();
    let (share_low, share_high) = (budget_shares.iter().cloned().fold(f64::INFINITY, f64::min), budget_shares.iter().cloned().fold(0.0f64, f64::max));
    let within_budget = share_high < 1.0;
    // the settled steps (the steps of the most frequent size and order) form one unbroken run: the number of steps
    // before the first of them, and the number of steps of another size or order after it (IDA shortens its last
    // step, so that it ends at the time 20)
    let settled_run = |x: &Summary| -> (usize, usize) {
        let (h, order, nmain) = x.run.main_size();
        let first = x.run.records.iter().position(|r| r.kind.h.to_bits() == h.to_bits() && r.kind.order == order).expect("a settled step");
        (first, x.run.records.len() - first - nmain as usize)
    };
    let settled_runs: Vec<(usize, usize)> = six.iter().map(|x| settled_run(x)).collect();
    let settled_ok = six.iter().zip(&settled_runs).all(|(x, r)| r.0 <= 50 && r.1 <= if x.integrator == "IDA" { 1 } else { 0 });
    ck.add(
        "GS-17",
        "the time integrations of GS-13 step by step, the identity: the three integrations of GS-13 by CVODE and the same three by IDA (SUNDIALS 7.8.0 in pure Rust, set up as NDSolve of the Wolfram Language sets it up, and a second time with an iteration matrix that is always current) are run one internal step at a time; at the end of every step the exact local error w_local (the computed state minus the exact solution that starts from the computed state of the step before) and the deviation w_deviation from the exact solution w_exact are measured in the eigenvectors of H_lat. In each of the nine integrations the deviation at the end of the last internal step equals the sum of the local errors of all steps, each carried to that time by the exact evolution, within N_steps times 1e-16 (one rounding error per step); therefore the relative deviation at the end is the product of N_steps, the mean of the norms local_error of the steps, and the coherence (the norm of the sum divided by the sum of the norms, at most 1), and the six numbers of GS-13 and of the notebook follow from these three factors; the coherence follows the direction of the local errors of consecutive steps: the median over the settled steps of local_cosine (the cosine of the angle between the local error of a step and that of the step before, both carried to the same time by the exact evolution) is above 0.99 in every integration with a coherence above 0.95 and negative in every integration with a coherence below 0.5, and in IDA at rtol = 1e-12 it is below -0.9 (consecutive local errors are nearly opposite and cancel); IDA is told to stop at the time 20 and ends its last step exactly there; CVODE is run through the same routine as in GS-13, which takes its steps past an output time and interpolates back: its last internal step ends less than one step size after the time 20, and the deviation there agrees within 0.2 per cent with the deviation that GS-13 reports for the interpolated solution at the time 20; each of the six deviations is smaller than its budget, which is N_steps times budget_step, the largest local error that the tolerances allow in one step; in each of the six integrations the settled steps (the steps of the most frequent size and order) follow one another without interruption, from one of the first 50 steps to the end (IDA shortens its last step, which ends at the time 20)",
        identity_ok && product_dev < 1e-12 && gs13_same && cosines_ok && within_budget && settled_ok,
        format!(
            "largest defect of the identity {:.2} of N_steps x 1e-16; N_steps, mean local_error/rtol, coherence, median of local_cosine and deviation: {}; coherence above 0.99 in {} of the 6 integrations of the table; IDA with a current matrix: coherence {:.3}, {:.3}, {:.3} and median of local_cosine {:.3}, {:.3}, {:.3} for rtol = 1e-8, 1e-10, 1e-12; deviation divided by budget: {}; the budget exceeds the deviation by factors from {:.2} to {:.0}; steps before the first settled step: {}; steps of another size or order after it: {}; the last internal step of CVODE ends at the time {}; its deviation there divided by that of GS-13 at the time 20, minus 1: {}",
            largest_identity,
            six.iter().map(|x| format!("{} {}: {}, {:.3}, {:.3}, {:.3}, {}", x.integrator, tol_label(x.rtol), x.run.nsteps, x.mean_local_over_rtol(), x.run.coherence(), x.turn_median, sci(x.run.deviation))).collect::<Vec<_>>().join("; "),
            n_coherent,
            sums[6].run.coherence(), sums[7].run.coherence(), sums[8].run.coherence(), sums[6].turn_median, sums[7].turn_median, sums[8].turn_median,
            budget_shares.iter().map(|x| format!("{:.5}", x)).collect::<Vec<_>>().join(", "), 1.0 / share_high, 1.0 / share_low,
            settled_runs.iter().map(|r| r.0.to_string()).collect::<Vec<_>>().join(", "), settled_runs.iter().map(|r| r.1.to_string()).collect::<Vec<_>>().join(", "),
            end_times[..3].iter().map(|t| format!("{:.4}", t)).collect::<Vec<_>>().join(", "), gs13_rel.iter().map(|x| sci(*x)).collect::<Vec<_>>().join(", ")
        ),
    );

    // ------------------------------------------------------------------ GS-18: the formulas predict the deviations
    // (a) integrations whose settled steps are all of one kind and whose iteration is stable: IDA 1e-8 and 1e-10,
    //     CVODE 1e-10, and the three integrations of IDA with a current matrix
    // "stable": the steps of the integration with one iteration are stable, or it has (almost) none of them
    let stable_run = |r: &Run| other_root_one_iteration(r, emax) < 1.0 || fraction_one_iteration(r) < 0.05;
    let reg: Vec<&Summary> = sums.iter().filter(|x| regular(&x.run) && stable_run(&x.run)).collect();
    let reg_worst = reg.iter().map(|x| (x.run.predicted / x.run.deviation - 1.0).abs()).fold(0.0f64, f64::max);
    // the norm of the difference between the measured and the predicted errors of the modes, in units of the
    // tolerance: the formulas describe settled steps; the first steps of an integration (low orders, changing step
    // sizes) commit local errors of the size that the tolerance allows, which the prediction does not contain
    let reg_rest = reg.iter().map(|x| x.run.rest / x.rtol).fold(0.0f64, f64::max);
    // (b) the two integrations by CVODE whose settled steps make one iteration in some steps and two in others: the
    //     measured deviation lies between the prediction with every corrector solved exactly and the prediction that
    //     adds the errors of the two kinds of steps
    let mixed: Vec<&Summary> = six.iter().copied().filter(|x| !regular(&x.run)).collect();
    let mixed_ok = mixed.iter().all(|x| x.run.predicted_exact <= x.run.deviation && x.run.deviation <= x.run.predicted);
    //     why they are mixed: with one iteration their formula is unstable for the fastest modes (another root of the
    //     recurrence lies outside the unit circle), with two iterations it is stable
    let mixed_roots: Vec<(f64, f64, f64)> = mixed.iter().map(|x| {
        let k = x.run.main_kind().0;
        (emax * k.h, formulas::adams_parasitic_modulus(k.order as usize, emax * k.h, 1), formulas::adams_parasitic_modulus(k.order as usize, emax * k.h, 2))
    }).collect();
    let mixed_unstable = mixed_roots.iter().all(|(_, one, two)| *one > 1.0 && *two < 1.0);
    let limits: Vec<f64> = (5..=7).map(|q| formulas::adams_stability_limit(q, 1).unwrap_or(1.0)).collect();
    //     the two kinds of steps err differently: for the eigenvalue of the largest modulus, the relative error of a
    //     settled step with one iteration and with two, each divided by the leading term |c_AM| |z_step|^(order+1),
    //     and the angle between the two errors in the complex plane
    let mixed_kinds: Vec<(f64, f64, f64)> = mixed.iter().map(|x| {
        let k = x.run.main_kind().0;
        let q = k.order as usize;
        let z = Cdd { re: Dd::ZERO, im: Dd::from_f64(-emax) * Dd::from_f64(k.h) };
        let one = formulas::adams_eps(q, z, Some(1)).to_c64();
        let two = formulas::adams_eps(q, z, Some(2)).to_c64();
        let lead = formulas::c_am(q)[q].to_f64().abs() * (emax * k.h).powi(k.order + 1);
        let angle = ((one.re * two.re + one.im * two.im) / (one.abs() * two.abs())).clamp(-1.0, 1.0).acos().to_degrees();
        (one.abs() / lead, two.abs() / lead, angle)
    }).collect();
    let mixed_kinds_ok = mixed_kinds.iter().all(|(one, two, angle)| one / two > 1.4 && *angle > 45.0);
    //     why CVODE begins with one iteration in these two integrations and not at rtol = 1e-10: the test value that an
    //     exactly solved step of the settled size and order gives, against the threshold 0.1 of the second iteration
    let cv_formula_tests: Vec<f64> = cv.iter().map(|c| formula_test_value(&spec, &c.run.main_kind().0, c.rtol, c.rtol / 100.0)).collect();
    let cv_tests_ok = cv_formula_tests[0] < 0.1 && cv_formula_tests[2] < 0.1 && cv_formula_tests[1] > 0.1 && (cv_formula_tests[1] / six[1].test_median - 1.0).abs() < 0.05;
    //     the same for all six integrations: where the settled steps are of one kind and stable the formula gives the
    //     measured test value; where the steps with one iteration are unstable the measured value is larger
    let formula_tests: Vec<f64> = six.iter().map(|x| formula_test_value(&spec, &x.run.main_kind().0, x.rtol, x.rtol / 100.0)).collect();
    let formula_tests_ok = six.iter().zip(&formula_tests).all(|(x, f)| {
        if other_root_one_iteration(&x.run, emax) < 1.0 { (x.test_median / f - 1.0).abs() < 0.05 } else { x.test_median / f > 1.4 }
    });
    // (c) the estimate of the integrator against the exact local error, in the norm of the integrator, for the regular
    //     integrations: the ratio of the medians is the ratio of the error of the formula as applied to the error that
    //     the estimate assumes (1 for a current matrix or an exactly solved corrector)
    let ratio_pred = |x: &Summary| -> f64 {
        let k = x.run.main_kind().0;
        match k.method {
            Method::Adams => 1.0,
            Method::Bdf => leading_constant(&k) * (k.order + 1) as f64,
        }
    };
    let est: Vec<(String, f64, f64)> = reg.iter().map(|x| (format!("{} {} ({})", x.integrator, tol_label(x.rtol), x.matrix), x.wrms_median / x.test_median, ratio_pred(x))).collect();
    let est_worst = est.iter().map(|(_, a, b)| (a / b - 1.0).abs()).fold(0.0f64, f64::max);
    ck.add(
        "GS-18",
        "the time integrations of GS-13 step by step, the formulas: the relative error eps_step of one step of an eigenmode (the factor kappa_step of the formula times exp(-z_step), minus 1, with z_step = -i E_qp h_step) is computed with double-double numbers from the characteristic equation of the formula as the integrator applies it - the Adams-Moulton formula of the order q_CV with n_fp fixed-point iterations from the Adams-Bashforth predictor (CVODE), the backward differentiation formula of the order k_IDA with n_Newton Newton steps and an iteration matrix made for cj/cj_ratio (IDA); the predicted error err_mode of a mode at the end is the sum of eps_step over the steps minus i E_qp time_defect (the rounding of the time variable), and predicted_deviation is the norm of these errors weighted with the amplitudes of the initial state. (a) For every integration whose settled steps are at least 95 per cent of one kind and stable (IDA at rtol = 1e-8 and 1e-10 and CVODE at 1e-10, and IDA with a current matrix at the three tolerances) the measured and the predicted errors of the 384 modes differ by a vector (rest) of a norm of at most 3 rtol, the size of the errors of the first steps of an integration, which the formulas of settled steps do not describe, while the deviations are 115 to 1700 times rtol: predicted_deviation agrees with the measured deviation within 1 per cent; (b) for the two integrations by CVODE whose settled steps make one iteration in some steps and two in others (rtol = 1e-8, 1e-12) the measured deviation lies between the prediction with every implicit equation solved exactly and the prediction that adds the errors of the two kinds of steps; these two integrations are the ones in which the formula with one iteration is unstable: at their step size and order (6 and 7) and the largest eigenvalue E_max a root of the recurrence other than the principal one lies outside the unit circle (the limits of |z_step| for one iteration are 0.111, 0.057, 0.029 for the orders 5, 6, 7), while with two iterations all other roots are inside; and the two kinds of steps err differently: for the eigenvalue of the largest modulus the relative error of a settled step with one iteration is more than 1.4 times that of a step with two iterations, and the two errors differ in their direction in the complex plane by more than 45 degrees, so that the local errors of the two kinds of steps do not add up in phase; the value of the local error test that an exactly solved step of the settled size and order gives (computed from the relative errors of the formula for the exact solution at 81 times, median) is below 0.1 in these two integrations, so that CVODE, which makes a second iteration only above 0.1, begins with one iteration per step, and it is above 0.1 at rtol = 1e-10, where it agrees with the measured median of the test value within 5 per cent and CVODE makes two iterations in every settled step; over all six integrations the measured median of the test value is the value of the formula within 5 per cent where the steps with one iteration are stable, and more than 1.4 times this value where they are unstable; (c) for the integrations of (a) the median of the exact local error in the norm of the integrator, divided by the median of the test value (the same norm of the estimate of the integrator), is the ratio that the formulas give - 1 for an exactly solved step, 1 + (k_IDA + 1)((rho_Newton - 1)/rho_Newton) H_har for IDA with an old matrix - within 3 per cent",
        reg.len() == 6 && reg_worst < 0.01 && reg_rest < 3.0 && mixed.len() == 2 && mixed_ok && mixed_unstable && mixed_kinds_ok && cv_tests_ok && formula_tests_ok
            && (limits[0] - 0.111).abs() < 1e-3 && (limits[1] - 0.057).abs() < 1e-3 && (limits[2] - 0.029).abs() < 1e-3 && est_worst < 0.03,
        format!(
            "(a) {} integrations, largest rest/rtol = {:.2}, largest |predicted/measured - 1| = {}: {}; (b) exact solution <= measured <= sum of the kinds: {}; largest |z_step| and the largest modulus of another root with one and with two iterations: {}; limits {:.4}, {:.4}, {:.4}; relative error of a step of the fastest mode divided by its leading term, with one and with two iterations, and the angle between the two errors: {}; test value of an exactly solved settled step for rtol = 1e-8, 1e-10, 1e-12: {:.4}, {:.4}, {:.4} (measured medians {:.4}, {:.4}, {:.4}); measured median divided by the value of the formula, for the six integrations of the table: {}; (c) measured and predicted ratio exact/estimated: {}",
            reg.len(),
            reg_rest,
            sci(reg_worst),
            reg.iter().map(|x| format!("{} {} ({}) {} against {}", x.integrator, tol_label(x.rtol), x.matrix, sci3(x.run.predicted), sci3(x.run.deviation))).collect::<Vec<_>>().join(", "),
            mixed.iter().map(|x| format!("{} {}: {} <= {} <= {}", x.integrator, tol_label(x.rtol), sci3(x.run.predicted_exact), sci3(x.run.deviation), sci3(x.run.predicted))).collect::<Vec<_>>().join(", "),
            mixed_roots.iter().map(|(z, one, two)| format!("{:.4}: {:.3} and {:.3}", z, one, two)).collect::<Vec<_>>().join("; "),
            limits[0], limits[1], limits[2],
            mixed_kinds.iter().map(|(one, two, angle)| format!("{:.2}, {:.2}, {:.0} degrees", one, two, angle)).collect::<Vec<_>>().join("; "),
            cv_formula_tests[0], cv_formula_tests[1], cv_formula_tests[2], six[0].test_median, six[1].test_median, six[2].test_median,
            six.iter().zip(&formula_tests).map(|(x, f)| format!("{} {}: {:.3}", x.integrator, tol_label(x.rtol), x.test_median / f)).collect::<Vec<_>>().join(", "),
            est.iter().map(|(name, a, b)| format!("{} {:.3} and {:.3}", name, a, b)).collect::<Vec<_>>().join(", ")
        ),
    );

    // ------------------------------------------------------------------ GS-19: the bands of the controllers and the scan
    let in_band = |r: &Run, test: f64| -> bool {
        let (_, order, _) = r.main_size();
        let (lo, hi) = band(r.main_kind().0.method, order);
        test > lo && test <= hi
    };
    let scan_tests = |r: &Run| median(&r.main_records(10).iter().map(|x| x.test).collect::<Vec<f64>>());
    let bands_ok = scan_cv.iter().chain(scan_ida.iter()).all(|r| in_band(r, scan_tests(r)));
    let ratio_cv: Vec<f64> = scan_cv.iter().zip(&scan_rtols).map(|(r, rt)| r.deviation / rt).collect();
    let ratio_ida: Vec<f64> = scan_ida.iter().zip(&scan_rtols).map(|(r, rt)| r.deviation / rt).collect();
    let span = |v: &[f64]| v.iter().cloned().fold(0.0f64, f64::max) / v.iter().cloned().fold(f64::INFINITY, f64::min);
    let quotient: Vec<f64> = scan_ida.iter().zip(&scan_cv).map(|(a, b)| a.deviation / b.deviation).collect();
    let crossings = quotient.windows(2).filter(|w| (w[0] > 1.0) != (w[1] > 1.0)).count();
    let jump = |v: &[f64]| v.windows(2).map(|w| (w[1] / w[0]).max(w[0] / w[1])).fold(0.0f64, f64::max);
    let scan_reg: Vec<&Run> = scan_cv.iter().chain(scan_ida.iter()).filter(|r| regular(r) && stable_run(r)).collect();
    // (e) where the one-iteration steps are stable the settled steps are of one kind; every integration with settled
    //     steps of both kinds is unstable in its one-iteration steps
    let all_scan: Vec<&Run> = scan_cv.iter().chain(scan_ida.iter()).collect();
    let n_mixed = all_scan.iter().filter(|r| !regular(r)).count();
    let mixed_all_unstable = all_scan.iter().filter(|r| !regular(r)).all(|r| other_root_one_iteration(r, emax) > 1.0);
    let n_stable_one = all_scan.iter().filter(|r| other_root_one_iteration(r, emax) < 1.0).count();
    let stable_all_regular = all_scan.iter().filter(|r| other_root_one_iteration(r, emax) < 1.0).all(|r| regular(r));
    // the remaining integrations are unstable in their one-iteration steps and still at least 95 per cent of one
    // kind: those that make two iterations in (almost) every step, and the weakly unstable ones, in which fewer than
    // 5 per cent of the steps need a second iteration
    let n_two_only = all_scan.iter().filter(|r| regular(r) && other_root_one_iteration(r, emax) >= 1.0 && fraction_one_iteration(r) < 0.05).count();
    let n_weak = all_scan.iter().filter(|r| regular(r) && other_root_one_iteration(r, emax) >= 1.0 && fraction_one_iteration(r) >= 0.05).count();
    let scan_reg_worst = scan_reg.iter().map(|r| (r.predicted / r.deviation - 1.0).abs()).fold(0.0f64, f64::max);
    let scan_rest = |r: &Run| -> f64 {
        // the tolerance of a run of the scan: its position in the two lists
        let i = scan_cv.iter().position(|x| std::ptr::eq(x, r)).or_else(|| scan_ida.iter().position(|x| std::ptr::eq(x, r))).expect("a run of the scan");
        r.rest / scan_rtols[i]
    };
    let scan_reg_rest = scan_reg.iter().map(|r| scan_rest(r)).fold(0.0f64, f64::max);
    // the rule of CVODE for a second iteration, on all settled steps of the three integrations by CVODE
    let one_iter_ok = cv.iter().all(|c| c.run.main_records(10).iter().all(|x| x.kind.iterations != 1 || x.test <= 0.1 * (1.0 + 1e-9)));
    // the other direction.  The second iterate minus the predicted value is (1 + b0 z_step) times the first iterate
    // minus the predicted value, mode by mode (b0: the coefficient b_AM of the new step end), so the test value that
    // is recorded for a step with two iterations is at least 1 - b0 |z_step| (largest eigenvalue) times that of the
    // first iterate, which exceeded 0.1
    let two_slack = |c: &CvodeRun| -> f64 {
        let k = c.run.main_kind().0;
        formulas::b_am(k.order as usize)[0].to_f64() * emax * k.h
    };
    let two_iter_ok = cv.iter().all(|c| c.run.main_records(10).iter().all(|x| x.kind.iterations != 2 || x.test >= 0.1 * (1.0 - two_slack(c))));
    let one_largest = cv.iter().flat_map(|c| c.run.main_records(10).into_iter().filter(|x| x.kind.iterations == 1).map(|x| x.test).collect::<Vec<f64>>()).fold(0.0f64, f64::max);
    let two_smallest = cv.iter().flat_map(|c| c.run.main_records(10).into_iter().filter(|x| x.kind.iterations == 2).map(|x| x.test).collect::<Vec<f64>>()).fold(f64::INFINITY, f64::min);
    ck.add(
        "GS-19",
        "the time integrations of GS-13 step by step, the step size: the integration of GS-13 is repeated by CVODE and by IDA for 25 relative tolerances from 1e-6 to 1e-12, four per decade (atol = rtol/100). (a) In every one of the 50 integrations the median of the test value over the settled steps (constant step size and order) lies in the band in which the rule of the integrator for the next step leaves it: IDA doubles a step only if (2 test_value + 0.0001)^(-1/(k_IDA+1)) is at least 2 and shortens it if this number is at most 1, the band from (2^-(k_IDA+1) - 0.0001)/2 to 0.49995 (0.0078 to 0.5 for k_IDA = 5); CVODE lengthens a step only if 1/((6 test_value)^(1/(q_CV+1)) + 1e-6) is at least 1.5, the band from (1/1.5 - 1e-6)^(q_CV+1)/6 to 1 (0.015, 0.0098 and 0.0065 to 1 for q_CV = 5, 6, 7). (b) The ratio deviation/rtol is therefore not a property of the integrator: over the scan it varies by more than a factor 20 for each integrator, jumps by more than a factor 10 between two neighbouring tolerances, and the quotient of the deviations of IDA and of CVODE passes the value 1 at least 8 times. (c) CVODE makes a second fixed-point iteration in a step exactly when the test value of the first iterate exceeds 0.1: every settled step of the three integrations of GS-13 with one iteration has a test value of at most 0.1, and every settled step with two iterations has a test value of at least 0.1 (1 - b0 |z_step|), where b0 is the coefficient b_AM of the new step end and |z_step| belongs to the eigenvalue of the largest modulus (the test value of a step with two iterations is that of the second iterate, which differs from that of the first by at most the factor 1 - b0 |z_step| or 1 + b0 |z_step|). (d) For every integration of the scan whose settled steps are at least 95 per cent of one kind and stable, the measured and the predicted errors of the modes differ by a vector of a norm of at most 3 rtol (the errors of the first steps), so that predicted_deviation agrees with the measured deviation within 3 rtol, that is within 3.6 per cent where the deviation is 85 rtol and within 0.2 per cent where it is 2000 rtol. (e) The settled steps of an integration are of both kinds (less than 95 per cent of one kind) only where the steps with one iteration are unstable: in every such integration of the scan the recurrence of the integrator with one iteration, at the settled step size and order (for IDA also the settled cj_ratio) and the largest eigenvalue, has a root other than the principal one outside the unit circle; and every integration in which all other roots are inside is of one kind",
        bands_ok && span(&ratio_cv) > 20.0 && span(&ratio_ida) > 20.0 && jump(&ratio_cv) > 10.0 && jump(&ratio_ida) > 10.0 && crossings >= 8 && one_iter_ok && two_iter_ok && scan_reg_rest < 3.0 && scan_reg.len() >= 25
            && n_mixed >= 10 && mixed_all_unstable && n_stable_one >= 15 && stable_all_regular && n_mixed + n_stable_one + n_two_only + n_weak == 50,
        format!(
            "(a) all 50 medians in their bands; (b) deviation/rtol from {:.0} to {:.0} (CVODE) and from {:.0} to {:.0} (IDA), largest jump between neighbouring tolerances a factor {:.1} (CVODE) and {:.1} (IDA), deviation of IDA / deviation of CVODE from {:.3} to {:.1}, passing 1 {} times; at rtol = 1e-8, 1e-10, 1e-12 it is {:.2}, {:.2}, {:.2}; (c) largest test value of a settled step with one iteration {:.5}, smallest of a settled step with two iterations {:.5} (lower bounds 0.1 (1 - b0 |z_step|) = {:.4}, {:.4}, {:.4} for rtol = 1e-8, 1e-10, 1e-12); (d) {} of the 50 integrations, largest rest/rtol = {:.2}, largest |predicted/measured - 1| = {}; (e) {} integrations with steps of both kinds, all unstable with one iteration; {} integrations stable with one iteration, all of one kind; of the other {}, which are unstable with one iteration and of one kind, {} make two iterations in at least 95 per cent of their steps and {} are unstable so weakly that fewer than 5 per cent of their steps need a second iteration",
            ratio_cv.iter().cloned().fold(f64::INFINITY, f64::min),
            ratio_cv.iter().cloned().fold(0.0f64, f64::max),
            ratio_ida.iter().cloned().fold(f64::INFINITY, f64::min),
            ratio_ida.iter().cloned().fold(0.0f64, f64::max),
            jump(&ratio_cv),
            jump(&ratio_ida),
            quotient.iter().cloned().fold(f64::INFINITY, f64::min),
            quotient.iter().cloned().fold(0.0f64, f64::max),
            crossings,
            ida[0].run.deviation / cv[0].run.deviation,
            ida[1].run.deviation / cv[1].run.deviation,
            ida[2].run.deviation / cv[2].run.deviation,
            one_largest,
            two_smallest,
            0.1 * (1.0 - two_slack(&cv[0])),
            0.1 * (1.0 - two_slack(&cv[1])),
            0.1 * (1.0 - two_slack(&cv[2])),
            scan_reg.len(),
            scan_reg_rest,
            sci(scan_reg_worst),
            n_mixed,
            n_stable_one,
            50 - n_mixed - n_stable_one,
            n_two_only,
            n_weak
        ),
    );

    // ------------------------------------------------------------------ GS-20: IDA in the open
    // (a) the same steps as the IDA of the notebook
    let mut same_steps = true;
    let mut same_evals = true;
    let mut dev_rel = [0.0f64; 3];
    for i in 0..3 {
        let hs: Vec<f64> = ida[i].steps.iter().map(|s| s.h).collect();
        let hmax = hs.iter().cloned().fold(0.0f64, f64::max);
        let hmin = hs.iter().cloned().fold(f64::INFINITY, f64::min);
        same_steps &= ida[i].run.nsteps == NB_STEPS[i] && (hmax / NB_HMAX[i] - 1.0).abs() < 1e-10 && (hmin / NB_HMIN[i] - 1.0).abs() < 1e-10;
        same_evals &= ida[i].stats.nrevals + (2 * n as i64 + 1) * ida[i].stats.njevals + 1 == NB_EVALS[i];
        dev_rel[i] = ida[i].run.deviation / NB_DEV[i] - 1.0;
    }
    // (b) the ratio cj/cjJ of the settled steps and its consequences
    let r10 = ida[1].run.main_kind().0.ratio;
    let r12 = ida[2].run.main_kind().0.ratio;
    let bound = formulas::bdf_stability_bound(5);
    let (window_lo, window_hi) = formulas::bdf_stability_window(5);
    let root10 = formulas::bdf_alternating_root(5, r10).unwrap_or(0.0);
    let root12 = formulas::bdf_alternating_root(5, r12).unwrap_or(0.0);
    let factor10 = leading_constant(&ida[1].run.main_kind().0) * 6.0;
    //     where the ratio of the settled steps comes from: the last step before them whose matrix was made for its own
    //     leading coefficient (ratio 1) has the step size h_J and the order k_J, the settled steps have h and k, and
    //     the leading coefficient of a step is cj = H_har/h_step, so that cj_ratio = (H_k/h)/(H_(k_J)/h_J)
    let origin = |r: &IdaRun| -> (i64, f64, i32, f64) {
        let main = r.run.main_kind().0;
        let first = r.steps.iter().position(|x| x.h.to_bits() == main.h.to_bits() && x.k == main.order && (x.ratio - main.ratio).abs() < 1e-12).expect("a settled step");
        let j = (0..=first).rev().find(|&j| (r.steps[j].ratio - 1.0).abs() < 1e-12).expect("a step with a current matrix");
        let from_sizes = (formulas::harmonic(main.order as usize).to_f64() / main.h) / (formulas::harmonic(r.steps[j].k as usize).to_f64() / r.steps[j].h);
        (r.steps[j].n, r.steps[j].h, r.steps[j].k, from_sizes)
    };
    let origins: Vec<(i64, f64, i32, f64)> = ida.iter().map(origin).collect();
    let origins_ok = ida.iter().zip(&origins).all(|(r, o)| (o.3 / r.run.main_kind().0.ratio - 1.0).abs() < 1e-12);
    // the measured growth of the test value per step at rtol = 1e-12: the median of the ratios of the test values of
    // two consecutive settled steps with one Newton step each, where the test value is at least 0.05
    let rec12 = ida[2].run.main_records(10);
    let growth: Vec<f64> = rec12.windows(2).filter(|w| w[0].kind.iterations == 1 && w[1].kind.iterations == 1 && w[1].n == w[0].n + 1 && w[0].test >= 0.05).map(|w| w[1].test / w[0].test).collect();
    let growth_median = median(&growth);
    let (_, two12, _) = iteration_counts(&ida[2].run);
    //     the rule of IDA for a second Newton step (function second_newton_rule)
    let (rho_one, newton_rate, newton_ss, second_threshold) = second_newton_rule(r12, 5);
    //     in a step with two Newton steps the accepted value lies the fraction 1 - (1 - rho_one)^2 of the way from the
    //     predicted value to the exact solution of the formula, the first iterate the fraction rho_one: the recorded
    //     test value times rho_one/(1 - (1 - rho_one)^2) is the test value of the first iterate
    let first_iterate_factor = rho_one / (1.0 - (1.0 - rho_one) * (1.0 - rho_one));
    let first_two: Vec<f64> = rec12.iter().filter(|x| x.kind.iterations == 2).map(|x| x.test * first_iterate_factor).collect();
    let largest_one = rec12.iter().filter(|x| x.kind.iterations == 1).map(|x| x.test).fold(0.0f64, f64::max);
    let two_low = first_two.iter().cloned().fold(f64::INFINITY, f64::min);
    let two_high = first_two.iter().cloned().fold(0.0f64, f64::max);
    //     the number of steps from one step with two Newton steps to the next
    let two_numbers: Vec<f64> = rec12.iter().filter(|x| x.kind.iterations == 2).map(|x| x.n as f64).collect();
    let period_median = median(&two_numbers.windows(2).map(|w| w[1] - w[0]).collect::<Vec<f64>>());
    let second_ok = !first_two.is_empty() && (two_low / second_threshold - 1.0).abs() < 0.03 && (two_high / second_threshold - 1.0).abs() < 0.03 && largest_one < 1.005 * second_threshold;
    //     the saw-tooth of the test value: its range over the settled steps (the smallest and the largest value, and
    //     the values below which 5 and 95 per cent of the settled steps lie), and its period.  The second Newton step
    //     multiplies the distance from the exact solution of the formula by `rate`; if the growing error were all of
    //     that distance, the test value would need ln(1/rate)/ln|root| steps to grow back to the threshold
    let mut tests12: Vec<f64> = rec12.iter().map(|x| x.test).collect();
    tests12.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let quantile = |q: f64| tests12[((tests12.len() - 1) as f64 * q).round() as usize];
    let (test_low, test_05, test_95, test_high) = (tests12[0], quantile(0.05), quantile(0.95), tests12[tests12.len() - 1]);
    let period_predicted = (1.0 / newton_rate).ln() / root12.abs().ln();
    let saw_ok = test_low > band(Method::Bdf, 5).0 && test_high < 1.005 * second_threshold && test_05 > 0.05 && test_95 < 0.33 && (period_predicted / period_median - 1.0).abs() < 0.1;
    // (c) the deviation at rtol = 1e-12 is mostly the rounding of the time variable
    let time_share = ida[2].run.predicted_time / ida[2].run.deviation;
    let pred12 = ida[2].run.predicted / ida[2].run.deviation - 1.0;
    //     the time that IDA reports is the rounded sum t_(s-1) + h_s in every step but the last, which ends at the stop
    //     time; the rounding error of one such addition of the settled step size is the same for all times between two
    //     powers of 2 (there the times are multiples of one unit in the last place)
    let (settled_h, _, _) = ida[2].run.main_size();
    let mut t_before = 0.0f64;
    let mut rounded_sums = 0usize;
    for st in &ida[2].steps {
        if t_before + st.h == st.t {
            rounded_sums += 1;
        }
        t_before = st.t;
    }
    let ulp_error = |t0: f64| -> f64 {
        let exact = Dd::from_f64(t0) + Dd::from_f64(settled_h);
        let unit = f64::EPSILON * 2f64.powi(t0.log2().floor() as i32);
        (Dd::from_f64(t0 + settled_h) - exact).to_f64() / unit
    };
    let ulp_errors = [ulp_error(5.0), ulp_error(10.0), ulp_error(18.0)];
    //     the defect predicted from these three numbers and the numbers of settled steps that start in the three ranges
    let count_in = |lo: f64, hi: f64| ida[2].run.records.iter().filter(|r| r.kind.h.to_bits() == settled_h.to_bits() && r.t - r.kind.h >= lo && r.t - r.kind.h < hi).count() as f64;
    let defect_from_units = -(count_in(4.0, 8.0) * ulp_errors[0] * f64::EPSILON * 4.0 + count_in(8.0, 16.0) * ulp_errors[1] * f64::EPSILON * 8.0 + count_in(16.0, 32.0) * ulp_errors[2] * f64::EPSILON * 16.0);
    let defect_4_to_20 = ida[2].run.time_defect - ida[2].run.records.iter().filter(|r| r.t - r.kind.h < 4.0).last().map(|r| r.time_defect).unwrap_or(0.0);
    // (d) with a current matrix
    let cur = &idc[2];
    let (hc, _, _) = cur.run.main_size();
    let (hd, _, _) = ida[2].run.main_size();
    let cur_ok = idc.iter().all(|r| r.steps.iter().all(|s| (s.ratio - 1.0).abs() < 1e-6)) && cur.run.nsteps * 2 < ida[2].run.nsteps + 2000 && hc > 2.0 * hd && cur.run.coherence() > 0.95 && cur.run.deviation > 5.0 * ida[2].run.deviation;
    let half10 = idc[1].run.deviation / ida[1].run.deviation;
    //     the test value that the formula alone would give at the step size of the default integration: the estimate of
    //     a settled step is proportional to h^(k+1), so it is (h_default/h_current)^6 times the median test value of
    //     the integration with a current matrix; it lies below the band of IDA, that is, without the growing error
    //     IDA would have doubled the step
    let test_formula_alone = (hd / hc).powi(6) * sums[8].test_median;
    //     the same number computed directly from the formula (function formula_test_value)
    let test_formula_direct = formula_test_value(&spec, &ida[2].run.main_kind().0, 1e-12, 1e-14);
    let would_double = test_formula_alone < band(Method::Bdf, 5).0 && test_formula_direct < band(Method::Bdf, 5).0 && (test_formula_direct / test_formula_alone - 1.0).abs() < 0.2;
    //     and the quotient of the two deviations at rtol = 1e-10 follows from the step sizes, the numbers of steps and
    //     the factor of the outdated matrix: (h_current/h_default)^6 (N_current/N_default) / factor
    let (hc10, _, _) = idc[1].run.main_size();
    let (hd10, _, _) = ida[1].run.main_size();
    let half10_predicted = (hc10 / hd10).powi(6) * (idc[1].run.nsteps as f64 / ida[1].run.nsteps as f64) / factor10;
    // (e) how the settled step size comes about: the history of the step size of the three integrations (and of the
    //     same three with the matrix by difference quotients), and the two coefficients of IDA computed from the step
    //     sizes for every step of all nine integrations by IDA
    let h_first = 1e-4;
    let hist: Vec<History> = ida.iter().map(|r| history(r, h_first)).collect();
    let hist_q: Vec<History> = idq.iter().map(|r| history(r, h_first)).collect();
    let mut coefficient_defect = 0.0f64;
    for r in ida.iter().chain(idq.iter()).chain(idc.iter()) {
        let sizes: Vec<f64> = r.steps.iter().map(|x| x.h).collect();
        for x in &r.steps {
            let (c_test, c_next) = ida_coefficients(&sizes, x.n as usize, x.k as usize);
            coefficient_defect = coefficient_defect.max((c_test / x.ck - 1.0).abs()).max((c_next / x.ida_sigma - 1.0).abs());
        }
    }
    let history_counts: Vec<(u32, u32, u32, i64)> = hist.iter().map(|x| (x.quarters, x.tenths, x.doublings, x.computed.first().map(|c| c.0).unwrap_or(0))).collect();
    let rule_defect = hist.iter().chain(hist_q.iter()).map(|x| x.rule_defect).fold(0.0f64, f64::max);
    let product_defect = hist.iter().chain(hist_q.iter()).map(|x| x.product_defect.abs()).fold(0.0f64, f64::max);
    let history_ok = hist.iter().chain(hist_q.iter()).all(|x| x.other == 0 && x.computed.len() == 1)
        && rule_defect < 1e-13
        && product_defect < 1e-14
        && coefficient_defect < 1e-12
        && history_counts == vec![(1, 2, 9, 1), (3, 0, 11, 43), (5, 3, 13, 40)]
        && hist.iter().zip(&hist_q).all(|(a, b)| a.quarters == b.quarters && a.tenths == b.tenths && a.doublings == b.doublings && a.computed[0].0 == b.computed[0].0 && a.first == b.first && a.settled == b.settled);
    // (f) the matrix by difference quotients, and the IDA of the notebook
    let mut same_counts = true;
    let mut first_other = [0i64; 3];
    let mut factor_rel = [0.0f64; 3];
    let mut estimate_rel = [0.0f64; 3];
    let mut sensitivity = [0.0f64; 3];
    let mut settled_rel_q = [0.0f64; 3];
    for i in 0..3 {
        same_counts &= idq[i].stats.nsteps == ida[i].stats.nsteps && idq[i].stats.nrevals == ida[i].stats.nrevals && idq[i].stats.njevals == ida[i].stats.njevals;
        same_counts &= ida[i].stats.nre_dq == 0 && idq[i].stats.nre_dq == 2 * n as i64 * idq[i].stats.njevals;
        let (a, b) = (&ida[i].steps, &idq[i].steps);
        first_other[i] = (0..a.len().min(b.len())).find(|&j| a[j].h.to_bits() != b[j].h.to_bits() || a[j].k != b[j].k).map(|j| a[j].n).unwrap_or(0);
        let (ca, cb) = (hist[i].computed[0], hist_q[i].computed[0]);
        factor_rel[i] = cb.4 / ca.4 - 1.0;
        estimate_rel[i] = cb.3 / ca.3 - 1.0;
        // the factor is (2 e + 1e-4)^(-1/(k+1)): a relative change x of e changes it by -x (2 e/(2 e + 1e-4))/(k + 1)
        sensitivity[i] = -(2.0 * ca.3 / (2.0 * ca.3 + 1e-4)) / (ca.1 as f64 + 1.0);
        settled_rel_q[i] = idq[i].run.main_size().0 / ida[i].run.main_size().0 - 1.0;
    }
    //     the errors of the formulas do not notice the change of the settled step size
    let formula_rel_q = (0..3).map(|i| (idq[i].run.predicted_formula / ida[i].run.predicted_formula - 1.0).abs()).fold(0.0f64, f64::max);
    let dq_ok = same_counts
        && formula_rel_q < 1e-6
        && (0..3).all(|i| first_other[i] == hist[i].computed[0].0 + 1)
        && factor_rel[0].abs() < 1e-14
        && (1..3).all(|i| factor_rel[i].abs() > 2e-9 && factor_rel[i].abs() < 2e-8 && (factor_rel[i] / (sensitivity[i] * estimate_rel[i]) - 1.0).abs() < 1e-3 && (settled_rel_q[i] / factor_rel[i] - 1.0).abs() < 1e-6);
    //     the IDA of the notebook: repeated addition of its settled step size reproduces its grid, and its steps
    //     before the settled ones are those of the program
    let nb_early: [&[f64]; 3] = [&NB_EARLY_8, &NB_EARLY_10, &NB_EARLY_12];
    let mut nb_reproduced = true;
    let mut nb_defect_ok = true;
    let mut nb_counts_ok = true;
    let mut nb_same_start = [false; 3];
    let mut nb_early_rel = [0.0f64; 3];
    let mut nb_settled_rel = [0.0f64; 3];
    for i in 0..3 {
        let t0 = dyadic(NB_T_BEFORE[i]);
        for j in 0..NB_H_NUMBER[i] {
            let (t1, d) = add_steps(t0, dyadic((NB_H_LOWEST[i].0 + j as f64, NB_H_LOWEST[i].1)), NB_SETTLED[i]);
            nb_reproduced &= t1.to_bits() == NB_T_AFTER[i].to_bits();
            if j == 0 {
                nb_defect_ok &= (d / NB_DEFECT[i].0 - 1.0).abs() < 1e-9;
            }
            if j + 1 == NB_H_NUMBER[i] {
                nb_defect_ok &= (d / NB_DEFECT[i].1 - 1.0).abs() < 1e-9;
            }
        }
        nb_counts_ok &= NB_FIRST_SETTLED[i] == hist[i].first && NB_SETTLED[i] == hist[i].settled && nb_early[i].len() == hist[i].first as usize;
        for st in 0..(hist[i].first as usize - 1).min(nb_early[i].len()) {
            nb_early_rel[i] = nb_early_rel[i].max((nb_early[i][st] / ida[i].steps[st].h - 1.0).abs());
        }
        nb_same_start[i] = hist[i].t_before.to_bits() == t0.to_bits();
        nb_settled_rel[i] = dyadic(NB_H_LOWEST[i]) / ida[i].run.main_size().0 - 1.0;
    }
    let nb_ok = nb_reproduced && nb_defect_ok && nb_counts_ok && nb_same_start == [false, true, true]
        && nb_early_rel.iter().all(|&x| x < 1e-12) && nb_settled_rel[0].abs() < 1e-12
        && (1..3).all(|i| nb_settled_rel[i] < -2e-8 && nb_settled_rel[i] > -4e-8);
    // (g) the consequences.  The time defect of an integration with the settled step size h: the defect of the steps
    //     before the settled ones, the additions of the settled steps, and the last step, which IDA makes
    //     (20 - t)(1 - 4 eps) long (eps = 2^-52, the rounding unit) and ends at the stop time 20
    let last_step = |t1: f64| -> f64 { (T_END - t1) * (1.0 - 4.0 * f64::EPSILON) };
    let defect_of = |x: &History, h: f64| -> (f64, f64) {
        let (t1, d) = add_steps(x.t_before, h, x.settled);
        (t1, x.defect_before + d + (last_step(t1) - (T_END - t1)))
    };
    //     the predicted deviation of the integration number i for another time defect: the errors of the formulas of
    //     the integration with the exact matrix and the phase error -i E dt of the rounded time, for every eigenvector
    let predicted_with = |i: usize, dt: f64| -> f64 { ida[i].run.modes.iter().map(|m| m.weight * m.weight * (m.formula + C64 { re: 0.0, im: -m.e * dt }).norm_sqr()).sum::<f64>().sqrt() };
    let mut sim_ok = true;
    for i in 0..3 {
        for (r, x) in [(&ida[i], &hist[i]), (&idq[i], &hist_q[i])] {
            let (t1, d) = defect_of(x, r.run.main_size().0);
            let last = &r.steps[r.steps.len() - 1];
            sim_ok &= t1.to_bits() == r.steps[r.steps.len() - 2].t.to_bits() && last.h.to_bits() == last_step(t1).to_bits() && last.t == T_END && (d / r.run.time_defect - 1.0).abs() < 1e-12;
        }
        sim_ok &= (predicted_with(i, ida[i].run.time_defect) / ida[i].run.predicted - 1.0).abs() < 1e-12;
    }
    //     the differences between the deviations of the three integrations of one tolerance, measured and predicted
    //     from their time defects (for the notebook the middle of the time defects of its candidates)
    let nb_defect_mid = |i: usize| 0.5 * (NB_DEFECT[i].0 + NB_DEFECT[i].1);
    let diff_q: Vec<(f64, f64)> = (0..3).map(|i| (idq[i].run.deviation / ida[i].run.deviation - 1.0, predicted_with(i, idq[i].run.time_defect) / ida[i].run.predicted - 1.0)).collect();
    let diff_nb: Vec<(f64, f64)> = (0..3).map(|i| (NB_DEV[i] / ida[i].run.deviation - 1.0, predicted_with(i, nb_defect_mid(i)) / ida[i].run.predicted - 1.0)).collect();
    let pred12q = idq[2].run.predicted / idq[2].run.deviation - 1.0;
    let pred12nb = predicted_with(2, nb_defect_mid(2)) / NB_DEV[2] - 1.0;
    //     rtol = 1e-12: every machine number h of one period.  The times from 16 to 32 are multiples of the unit
    //     u16 = 2^-48, the times of every range below are multiples of a power of 2 that divides u16; the rounding of
    //     an addition t + h therefore depends on h only through its remainder after division by u16, and the settled
    //     step size and the 65535 machine numbers after the largest multiple of u16 below it are all cases there are.
    let h12 = ida[2].run.main_size().0;
    let u16 = f64::EPSILON * 16.0;
    let ulp_h = f64::EPSILON * 2f64.powi(h12.log2().floor() as i32);
    let period = (u16 / ulp_h).round() as usize;
    let h_base = (h12 / u16).floor() * u16;
    let phase_of = |h: f64| -> usize { ((h - (h / u16).floor() * u16) / ulp_h).round() as usize };
    let chunks = 64usize;
    let per_chunk = period / chunks;
    let last_digits: Vec<(f64, f64)> = in_parallel(chunks, |c| {
        (0..per_chunk)
            .map(|j| {
                let (_, dt) = defect_of(&hist[2], h_base + (c * per_chunk + j) as f64 * ulp_h);
                (dt, predicted_with(2, dt))
            })
            .collect::<Vec<(f64, f64)>>()
    })
    .into_iter()
    .flatten()
    .collect();
    //     the same time defect from the closed formula: for every unit u of the last place, the number of settled
    //     steps that end at a time with this unit, times the distance of h from the nearest multiple of u.  If h lies
    //     exactly half way between two multiples of u, the machine addition rounds to the sum whose last binary digit
    //     is 0; from the second step on the time before has this property too, so that the multiple of u added is the
    //     even one: `round_ties_even`
    let mut units: Vec<(f64, usize)> = Vec::new();
    {
        let mut t = hist[2].t_before;
        for _ in 0..hist[2].settled {
            t += h12;
            let u = f64::EPSILON * 2f64.powi(t.log2().floor() as i32);
            match units.iter_mut().find(|x| x.0 == u) {
                Some(x) => x.1 += 1,
                None => units.push((u, 1)),
            }
        }
    }
    let saw_formula = |h: f64| -> f64 { hist[2].defect_before + units.iter().map(|&(u, count)| count as f64 * (h - (h / u).round_ties_even() * u)).sum::<f64>() };
    let defect_bound: f64 = units.iter().map(|&(u, count)| 0.5 * count as f64 * u).sum();
    let unit_sum: f64 = units.iter().map(|&(u, _)| u).sum();
    let saw_defect = (0..period).map(|j| (last_digits[j].0 - saw_formula(h_base + j as f64 * ulp_h)).abs()).fold(0.0f64, f64::max);
    let mut ld_pred: Vec<f64> = last_digits.iter().map(|x| x.1).collect();
    ld_pred.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let ld_q = |q: f64| ld_pred[((ld_pred.len() - 1) as f64 * q).round() as usize];
    let ld_below = |v: f64| ld_pred.iter().filter(|&&x| x < v).count() as f64 / ld_pred.len() as f64;
    let ld_defect_max = last_digits.iter().map(|x| x.0.abs()).fold(0.0f64, f64::max);
    let formula_alone = predicted_with(2, 0.0);
    //     the three integrations at rtol = 1e-12 are three points of this one function
    let phase12 = phase_of(h12);
    let phase12q = phase_of(idq[2].run.main_size().0);
    let phase12nb = phase_of(dyadic(NB_H_LOWEST[2]));
    let on_curve = (last_digits[phase12].0 / ida[2].run.time_defect - 1.0).abs() < 1e-12
        && (last_digits[phase12q].0 - idq[2].run.time_defect).abs() < unit_sum
        && (last_digits[phase12nb].0 - NB_DEFECT[2].0).abs() < unit_sum
        && phase12nb + NB_H_NUMBER[2] <= period;
    let below = [ld_below(ida[2].run.deviation), ld_below(idq[2].run.deviation), ld_below(NB_DEV[2])];
    //     what the closed formula leaves out: the steps before the settled ones and the shortening of the last step
    let before_largest = hist.iter().chain(hist_q.iter()).map(|x| x.defect_before.abs()).fold(0.0f64, f64::max);
    let before_12 = hist[2].defect_before.abs().max(hist_q[2].defect_before.abs());
    let last_shortening = {
        let t1 = ida[2].steps[ida[2].steps.len() - 2].t;
        last_step(t1) - (T_END - t1)
    };
    let last_digits_ok = period == 65536 && sim_ok && on_curve && saw_defect < unit_sum && ld_defect_max <= defect_bound
        && before_largest < 1e-16 && before_12 < 1e-18 && last_shortening < -3.0e-20 && last_shortening > -4.0e-20 && units.len() == 13
        && units.contains(&(f64::EPSILON * 4.0, 8216)) && units.contains(&(f64::EPSILON * 8.0, 16432)) && units.contains(&(f64::EPSILON * 16.0, 8216))
        && ld_pred[0] > 0.99 * formula_alone && ld_pred[0] < 1.001 * formula_alone
        && ld_q(0.5) > 4.0e-11 && ld_q(0.5) < 5.0e-11 && ld_pred[period - 1] > 1.0e-10 && ld_pred[period - 1] < 1.4e-10
        && below.iter().all(|&x| x > 0.35 && x < 0.65)
        && pred12q.abs() < 0.05 && pred12nb.abs() < 0.01
        && (0..2).all(|i| (diff_q[i].0 / diff_q[i].1 - 1.0).abs() < 0.1 && (diff_nb[i].0 / diff_nb[i].1 - 1.0).abs() < 0.1)
        && (diff_q[2].0 - diff_q[2].1).abs() < 0.04 && (diff_nb[2].0 - diff_nb[2].1).abs() < 0.01;
    ck.add(
        "GS-20",
        "the time integrations of GS-13 step by step, IDA in the open: (a) IDA of SUNDIALS 7.8.0 in pure Rust, with the tolerance rtol |y| + atol of every real unknown, a first step of 1e-4, the orders 1 to 5 and the dense matrix cj 1 - M, reproduces the IDA inside NDSolve of the Wolfram Language as far as the rounding of one number allows (parts (e) to (g)): for rtol = 1e-8, 1e-10, 1e-12 the numbers of internal steps are those of the notebook (2705, 7309, 41111), the largest and the smallest step agree with the notebook within 1e-10, and the number of evaluations of the right side that the notebook reports is the number of residual evaluations here plus 769 for every matrix plus 1 (11182, 18096, 54585: the notebook computes its matrix by difference quotients, one evaluation for each of the 768 unknowns and one more); the deviation at the time 20 agrees with the notebook within 1e-4 for rtol = 1e-8 and 1e-10 and within 20 per cent for rtol = 1e-12, where it is a sum of rounding errors. (b) IDA computes its matrix anew only when cj_ratio leaves the interval 0.6 to 5/3; in each of the three integrations the cj_ratio of the settled steps is the quotient of cj = H_har/h_step of the settled steps and of the last step before them that had a current matrix (within 1e-12): the matrix of the settled steps was made for an earlier, larger step size (and at rtol = 1e-12 also for the order 4) and is used unchanged to the end; with one Newton step per step the formula of the order 5 is stable only for cj_ratio between 0.772 and 137/105 = 1.3048. At rtol = 1e-10 the settled steps have cj_ratio = 1.164: one Newton step then overshoots, the error of a step is 1 + 6 ((rho_Newton - 1)/rho_Newton) H_har = 1.97 times that of the formula, and the alternating root is -0.65 (stable). At rtol = 1e-12 they have cj_ratio = 1.313, above the bound 137/105 at which the alternating root passes -1: an error that changes its sign from step to step grows by the factor 1.017 per step, the measured test value grows by this factor per step (within 0.3 per cent). The growth is cut back by the rule of IDA for the convergence of its iteration: IDA makes a second Newton step in a step when the test value of the first iterate exceeds 0.33/((k_IDA + 1) ss), where ss = rate/(1 - rate) and rate = |1 - rho_one| is the factor by which one Newton step with the old matrix multiplies the distance from the exact solution of the formula, rho_one = 2 cj_ratio/(1 + cj_ratio) (ss and rate are names of the source text of IDA); this threshold is 0.352, the test value of the first iterate of every settled step with two Newton steps lies within 3 per cent of it, no settled step with one Newton step has a larger test value, and a step with two Newton steps occurs about every 110 steps (the second Newton step multiplies the distance from the exact solution of the formula by rate, and ln(1/rate)/ln|alternating root| steps of growth lead back to the threshold: within 10 per cent of the measured number); the test value of every settled step lies inside the band of IDA, above 0.0078 and not above the threshold, and nine tenths of the settled steps have a test value between 0.05 and 0.33. (c) At rtol = 1e-12 the rounding of the time variable alone (time_defect = 8.1e-12 after 41111 additions) gives more than 80 per cent of the deviation, and predicted_deviation agrees with the measured deviation within 5 per cent. The time that IDA reports is the rounded sum of the reported time before and the step size in every step but the last; one addition of the settled step size is rounded by the same amount for all times between two powers of 2 (the rounded sum exceeds the exact one by +0.48, -0.26 and -0.13 units in the last place of the time between 4 and 8, 8 and 16, 16 and 20), and these three numbers times the numbers of the steps in the three ranges give the growth of time_defect from the time 4 to the time 20 within 1 per cent. (d) With a matrix that is always current (cj_ratio = 1 in every step) IDA at rtol = 1e-12 takes less than half as many steps of more than twice the size, the local errors add up in phase (coherence above 0.95), and the deviation is more than 5 times LARGER than with the default: the growing error of the default integration keeps its test value high and its steps small - the test value that the formula alone would give at the step size of the default integration, (ratio of the two step sizes)^6 times the median test value of the integration with a current matrix, lies below 0.0078, where IDA doubles a step (the same number computed directly from the formula, for the exact solution at 81 times, agrees within 20 per cent); at rtol = 1e-10 the deviation with a current matrix is 0.64 times the default one, which is (ratio of the step sizes)^6 times the ratio of the numbers of steps divided by the factor 1.97 of the outdated matrix (within 1 per cent). (e) How the settled step size comes about. IDA changes the step size by fixed factors - 1/4 after every rejected attempt of the first step (1, 3, 5 attempts for the three tolerances), 0.9 or 2 after an accepted step (2, 0, 3 times 0.9 and 9, 11, 13 doublings before the first settled step) - with one exception in each integration: after the step number 1, 43, 40 it multiplies the step size by the factor (2 e + 1e-4)^(-1/(k_IDA + 1)) that it computes from its estimate e for the next step, which is the test value of that step times the quotient of two coefficients of IDA; both coefficients follow from the sizes of the last steps, and the formulas of the program give the coefficients that IDA reports in every step of the nine integrations by IDA within 1e-12. The rule gives the factor within 1e-13, and the first step size 1e-4 times the product of all factors is the settled step size within 1e-14. (f) The same three integrations with the matrix that IDA forms itself by difference quotients (768 further evaluations of the right side for every matrix) have the same numbers of steps, of residual evaluations and of matrices, and bit for bit the same step sizes up to that one computed factor. The factor is another number because the test value of that one step is another number (the relative difference of the factor is -1/(k_IDA + 1) times that of the estimate, within 0.1 per cent at rtol = 1e-10 and 1e-12): the settled step sizes differ by less than 1e-14 at rtol = 1e-8 and by 2e-9 to 2e-8 at 1e-10 and 1e-12, and the deviation predicted from the formulas alone changes by less than 1e-6 of itself. The settled step size of the IDA of the notebook is read from the grid of its solution: 4, 4 and 8 neighbouring machine numbers reproduce the grid points of its 2663, 7265 and 41070 settled steps bit for bit by repeated addition (the same additions here end at the same grid point and give the same time defect within 1e-9). It differs from that of this program with the exact matrix by less than 1e-12 at rtol = 1e-8 and by 2e-8 to 4e-8 at 1e-10 and 1e-12; the steps of the notebook before the settled ones are those of this program within 1e-12, and at rtol = 1e-10 and 1e-12 the grid point before the first settled step is the same machine number. (g) The consequence. The time defect of an integration follows from its settled step size: the defect of the steps before, repeated machine addition of the settled step size, and the last step, which IDA makes (20 - t)(1 - 4 eps) long (eps = 2^-52), give the measured time defect of the six integrations (exact matrix and difference quotients) within 1e-12 and their last grid points bit for bit. The deviations of the three integrations of one tolerance (exact matrix, difference quotients, notebook) differ as their time defects say: the relative differences from the integration with the exact matrix, measured and predicted (the errors of the formulas of the integration with the exact matrix and the other time defect), agree within 10 per cent at rtol = 1e-8 and 1e-10 and within 0.04 at rtol = 1e-12. At rtol = 1e-12 the time defect depends on the settled step size only through its last 16 binary digits, its remainder after division by 2^-48 (the unit in the last place of the times from 16 to 32): for all 65536 machine numbers of one such period it is computed by repeated addition; it agrees with the closed formula - the sum over the units u in the last place of the time of (number of settled steps that end at a time with the unit u) x (step size minus the nearest multiple of u) - within the sum of the units, and its modulus stays below half the sum of (number of steps) x (unit). The predicted deviation over the period goes from the value of the formulas alone (within 1 per cent) to more than 1e-10, with a median between 4e-11 and 5e-11; the time defects of the three integrations are the values of this one function at their step sizes, their predicted deviations agree with the measured ones within 5 per cent (difference quotients) and 1 per cent (notebook), and their measured deviations lie between the 35th and the 65th of 100 equal parts of the period ordered by the predicted deviation",
        same_steps && same_evals && dev_rel[0].abs() < 1e-4 && dev_rel[1].abs() < 1e-4 && dev_rel[2].abs() < 0.2
            && (r10 - 1.164).abs() < 1e-3 && (factor10 - 1.97).abs() < 0.01 && (root10 + 0.65).abs() < 0.01
            && (window_lo - 0.772).abs() < 1e-3 && (window_hi - 137.0 / 105.0).abs() < 1e-8 && (bound - window_hi).abs() < 1e-8
            && (r12 - 1.313).abs() < 1e-3 && r12 > bound && (bound - 1.305).abs() < 1e-3 && root12 < -1.0 && (growth_median / root12.abs() - 1.0).abs() < 0.003 && two12 > 300 && two12 < 450
            && second_ok && saw_ok && (second_threshold - 0.352).abs() < 1e-3 && origins_ok
            && time_share > 0.8 && pred12.abs() < 0.05 && rounded_sums + 1 >= ida[2].steps.len() && (defect_from_units / defect_4_to_20 - 1.0).abs() < 0.01
            && (ulp_errors[0] - 0.48).abs() < 0.01 && (ulp_errors[1] + 0.26).abs() < 0.01 && (ulp_errors[2] + 0.13).abs() < 0.01
            && cur_ok && (half10 - 0.64).abs() < 0.02 && would_double && (half10_predicted / half10 - 1.0).abs() < 0.01
            && history_ok && dq_ok && nb_ok && last_digits_ok,
        format!(
            "(a) steps {}, {}, {}; residual evaluations + 769 matrices + 1 = {}, {}, {}; deviation here / notebook - 1 = {}, {}, {}; (b) matrix of the settled steps made in the step number {}, {}, {} for the step size {}, {}, {} and the order {}, {}, {}; stable for cj_ratio from {:.4} to {:.4}; rtol = 1e-10: cj_ratio {:.4}, factor {:.3}, alternating root {:.4}; rtol = 1e-12: cj_ratio {:.4}, bound {:.4}, alternating root {:.4}, measured growth of the test value per step {:.4} ({} pairs of steps), {} steps with two Newton steps; rho_one {:.4}, rate {:.4}, ss {:.4}, threshold of the second Newton step {:.4}, test value of the first iterate in the steps with two Newton steps {:.4} to {:.4}, largest test value of a step with one Newton step {:.4}, median number of steps from one step with two Newton steps to the next {:.0} (from rate and root: {:.0}), test value of the settled steps from {:.4} to {:.4}, nine tenths of them between {:.4} and {:.4}; (c) time_defect {}, its share of the deviation {:.3}, predicted/measured - 1 = {}; {} of {} reported times are the rounded sums; rounding of one addition {:.3}, {:.3}, {:.3} units in the last place; growth of time_defect from the time 4 to 20: {} from these numbers, {} measured; (d) current matrix at rtol = 1e-12: {} steps of the size {} (default: {} of {}), coherence {:.3}, deviation {} (default {}), test value of the formula alone at the default step size {} (directly from the formula: {}); at 1e-10: deviation {} = {:.3} of the default, predicted {:.3}",
            ida[0].run.nsteps, ida[1].run.nsteps, ida[2].run.nsteps,
            ida[0].stats.nrevals + 769 * ida[0].stats.njevals + 1, ida[1].stats.nrevals + 769 * ida[1].stats.njevals + 1, ida[2].stats.nrevals + 769 * ida[2].stats.njevals + 1,
            sci(dev_rel[0]), sci(dev_rel[1]), sci(dev_rel[2]),
            origins[0].0, origins[1].0, origins[2].0, sci3(origins[0].1), sci3(origins[1].1), sci3(origins[2].1), origins[0].2, origins[1].2, origins[2].2,
            window_lo, window_hi, r10, factor10, root10, r12, bound, root12, growth_median, growth.len(), two12,
            rho_one, newton_rate, newton_ss, second_threshold, two_low, two_high, largest_one, period_median, period_predicted, test_low, test_high, test_05, test_95,
            sci(ida[2].run.time_defect), time_share, sci(pred12), rounded_sums, ida[2].steps.len(), ulp_errors[0], ulp_errors[1], ulp_errors[2], sci3(defect_from_units), sci3(defect_4_to_20),
            cur.run.nsteps, sci3(hc), ida[2].run.nsteps, sci3(hd), cur.run.coherence(), sci3(cur.run.deviation), sci3(ida[2].run.deviation), sci(test_formula_alone), sci(test_formula_direct), sci3(idc[1].run.deviation), half10, half10_predicted
        ) + &format!(
            "; (e) rejected attempts of the first step {}, {}, {}; shortenings by 0.9: {}, {}, {}; doublings {}, {}, {}; computed factor after the step {}, {}, {} of the order {}, {}, {}: test value {:.6}, {:.6}, {:.6}, estimate for the next step {:.6}, {:.6}, {:.6}, factor {:.12}, {:.12}, {:.12}; rule/factor - 1 at most {}, product of the factors/settled step size - 1 at most {}, coefficients from the step sizes/reported - 1 at most {}; first settled step {}, {}, {}; settled steps {}, {}, {}; (f) difference quotients: evaluations for the matrices {}, {}, {} = 768 x {}, {}, {}; first step of another size {}, {}, {}; factor/factor of the exact matrix - 1 = {}, {}, {}; estimate/estimate - 1 = {}, {}, {}; -1/(k_IDA + 1) with the term 1e-4 of the rule: {:.5}, {:.5}, {:.5}; deviation predicted from the formulas alone/that of the exact matrix - 1 at most {}; notebook: settled step size/here - 1 = {}, {}, {}; largest relative difference of a step before the settled ones {}, {}, {}; grid point before the first settled step the same machine number: {}, {}, {}; (g) time defect: exact matrix {}, {}, {}; difference quotients {}, {}, {}; notebook {} to {}, {} to {}, {} to {}; deviation relative to the exact matrix - 1, measured and predicted: difference quotients {} and {}, {} and {}, {:.4} and {:.4}; notebook {} and {}, {} and {}, {:.4} and {:.4}; rtol = 1e-12: {} machine numbers in the period, phases {} (exact matrix), {} (difference quotients), {} (notebook, smallest candidate); largest |addition - closed formula| {} (sum of the {} units {}; settled steps that end at a time with the unit 2^-50, 2^-49, 2^-48: 8216, 16432, 8216); time defect of the steps before the settled ones at most {} in the six integrations and {} in the two at rtol = 1e-12; the last step is shorter than 20 - t by {}; largest |time defect| {} (bound {}); predicted deviation over the period: smallest {}, 5 per cent below {}, a quarter below {}, median {}, three quarters below {}, 95 per cent below {}, largest {} (formulas alone {}); part of the period with a smaller predicted deviation than the measured one: {:.3} (exact matrix), {:.3} (difference quotients), {:.3} (notebook); predicted/measured - 1: {} (difference quotients), {} (notebook)",
            hist[0].quarters, hist[1].quarters, hist[2].quarters, hist[0].tenths, hist[1].tenths, hist[2].tenths, hist[0].doublings, hist[1].doublings, hist[2].doublings,
            hist[0].computed[0].0, hist[1].computed[0].0, hist[2].computed[0].0, hist[0].computed[0].1, hist[1].computed[0].1, hist[2].computed[0].1,
            hist[0].computed[0].2, hist[1].computed[0].2, hist[2].computed[0].2, hist[0].computed[0].3, hist[1].computed[0].3, hist[2].computed[0].3,
            hist[0].computed[0].4, hist[1].computed[0].4, hist[2].computed[0].4,
            sci(rule_defect), sci(product_defect), sci(coefficient_defect), hist[0].first, hist[1].first, hist[2].first, hist[0].settled, hist[1].settled, hist[2].settled,
            idq[0].stats.nre_dq, idq[1].stats.nre_dq, idq[2].stats.nre_dq, idq[0].stats.njevals, idq[1].stats.njevals, idq[2].stats.njevals,
            first_other[0], first_other[1], first_other[2], sci(factor_rel[0]), sci3(factor_rel[1]), sci3(factor_rel[2]), sci(estimate_rel[0]), sci3(estimate_rel[1]), sci3(estimate_rel[2]),
            sensitivity[0], sensitivity[1], sensitivity[2], sci(formula_rel_q),
            sci(nb_settled_rel[0]), sci3(nb_settled_rel[1]), sci3(nb_settled_rel[2]), sci(nb_early_rel[0]), sci(nb_early_rel[1]), sci(nb_early_rel[2]), nb_same_start[0], nb_same_start[1], nb_same_start[2],
            sci3(ida[0].run.time_defect), sci3(ida[1].run.time_defect), sci3(ida[2].run.time_defect), sci3(idq[0].run.time_defect), sci3(idq[1].run.time_defect), sci3(idq[2].run.time_defect),
            sci3(NB_DEFECT[0].0), sci3(NB_DEFECT[0].1), sci3(NB_DEFECT[1].0), sci3(NB_DEFECT[1].1), sci3(NB_DEFECT[2].0), sci3(NB_DEFECT[2].1),
            sci3(diff_q[0].0), sci3(diff_q[0].1), sci3(diff_q[1].0), sci3(diff_q[1].1), diff_q[2].0, diff_q[2].1,
            sci3(diff_nb[0].0), sci3(diff_nb[0].1), sci3(diff_nb[1].0), sci3(diff_nb[1].1), diff_nb[2].0, diff_nb[2].1,
            period, phase12, phase12q, phase12nb, sci(saw_defect), units.len(), sci(unit_sum), sci(before_largest), sci(before_12), sci(-last_shortening), sci3(ld_defect_max), sci3(defect_bound),
            sci3(ld_pred[0]), sci3(ld_q(0.05)), sci3(ld_q(0.25)), sci3(ld_q(0.5)), sci3(ld_q(0.75)), sci3(ld_q(0.95)), sci3(ld_pred[period - 1]), sci3(formula_alone),
            below[0], below[1], below[2], sci(pred12q), sci(pred12nb)
        ),
    );

    // the data of the parts (e) to (g): the history of the settled step size and the three integrations of every
    // tolerance; and the time defect over one period of the last digits of the step size
    let mut s = String::from("# content: how the settled step size of IDA comes about, and three integrations of every tolerance (check GS-20, parts (e) to (g)): for the relative tolerances 1e-8, 1e-10 and 1e-12 the integration of check GS-13 by IDA set up like NDSolve, with the exact iteration matrix (integration ida_*, the one of the file t2_steps_runs.csv), the same with the matrix that IDA forms itself by difference quotients (ida_dq_*), and the IDA of the notebook 3+1super-loop.nb (notebook_*; its numbers are read from the grid of its solution) - the number of rejected attempts of the first step (each divides the step size by 4), of shortenings by the fixed factor 0.9 and of doublings before the first settled step; the number and the order of the step after which IDA multiplied the step size by a factor that it computed from its estimate, the test value of that step, the estimate for the next step and the factor (nan where the notebook does not show a number; the factor of the notebook is its settled step size divided by fixed_product); fixed_product, the first step size 1e-4 times the fixed factors; the settled step size with 17 digits (for the notebook the smallest of the h_candidates neighbouring machine numbers that reproduce its grid); the number of the first settled step and the number of settled steps; the phase of the settled step size at rtol = 1e-12, its remainder after division by 2^-48 in units of 2^-64 (-1 for the other tolerances); the time defect (for the notebook that of its settled steps for the smallest candidate) and time_defect_spread, the time defect for the largest candidate minus that for the smallest; the number of internal steps; the deviation at the end and the predicted deviation (for the notebook from the errors of the formulas of ida_* and its own time defect)\n");
    s.push_str("integration,matrix_source,rtol,quarter_cuts,tenth_cuts,doublings,factor_step,factor_order,test_value,next_estimate,computed_factor,fixed_product,h_step,h_candidates,first_settled,settled_steps,phase,time_defect,time_defect_spread,N_steps,max_deviation,predicted_deviation\n");
    for i in 0..3 {
        for (name, source, r, x) in [("ida", "exact", &ida[i], &hist[i]), ("ida_dq", "difference_quotients", &idq[i], &hist_q[i])] {
            let c = x.computed[0];
            let h = r.run.main_size().0;
            s.push_str(&format!(
                "{}_{},{},{:e},{},{},{},{},{},{:.15},{:.15},{:.17},{:.17e},{:.17e},1,{},{},{},{:.9e},0,{},{:.9e},{:.9e}\n",
                name, tol_label(rtols[i]), source, rtols[i], x.quarters, x.tenths, x.doublings, c.0, c.1, c.2, c.3, c.4, x.fixed_product, h, x.first, x.settled,
                if i == 2 { phase_of(h) as i64 } else { -1 }, r.run.time_defect, r.run.nsteps, r.run.deviation, r.run.predicted
            ));
        }
        let h = dyadic(NB_H_LOWEST[i]);
        s.push_str(&format!(
            "notebook_{},notebook,{:e},{},{},{},{},{},nan,nan,{:.17},{:.17e},{:.17e},{},{},{},{},{:.9e},{:.9e},{},{:.9e},{:.9e}\n",
            tol_label(rtols[i]), rtols[i], hist[i].quarters, hist[i].tenths, hist[i].doublings, hist[i].computed[0].0, hist[i].computed[0].1, h / hist[i].fixed_product, hist[i].fixed_product, h, NB_H_NUMBER[i],
            NB_FIRST_SETTLED[i], NB_SETTLED[i], if i == 2 { phase_of(h) as i64 } else { -1 }, NB_DEFECT[i].0, NB_DEFECT[i].1 - NB_DEFECT[i].0, NB_STEPS[i], NB_DEV[i], predicted_with(i, NB_DEFECT[i].0)
        ));
    }
    fs::write(dd.join("t2_steps_settled.csv"), s).expect("write t2_steps_settled.csv");

    let mut s = String::from("# content: the time defect and the predicted deviation of the integration of check GS-13 by IDA at rtol = 1e-12 as functions of the last 16 binary digits of the settled step size (check GS-20, part (g)): the settled step size is replaced by h_base + phase 2^-64 for every 32nd of the 65536 values of the phase (h_base: the largest multiple of 2^-48 that is not larger than the settled step size of the integration ida_1e-12; the phases are all machine numbers of one period); the time defect is computed by repeated machine addition as IDA advances its time (column time_defect) and from the closed formula, the sum over the units in the last place of the time of (number of settled steps that end at a time with this unit) x (step size minus the nearest multiple of the unit) (column time_defect_formula); the predicted deviation from the errors of the formulas of ida_1e-12 and this time defect\n");
    s.push_str(&format!("# period: {}\n# h_base: {:.17e}\n", period, h_base));
    s.push_str("phase,h_step,time_defect,time_defect_formula,predicted_deviation\n");
    for j in (0..period).step_by(32) {
        let h = h_base + j as f64 * ulp_h;
        s.push_str(&format!("{},{:.17e},{:.9e},{:.9e},{:.9e}\n", j, h, last_digits[j].0, saw_formula(h), last_digits[j].1));
    }
    fs::write(dd.join("t2_steps_last_digits.csv"), s).expect("write t2_steps_last_digits.csv");

    // ------------------------------------------------------------------ the tables of the document
    // Every table begins with `table_head`, which states the headings and the width of every column on the printed
    // page; numbers with a power of ten are unbreakable (`tex_sci`), so that a column that is too narrow for its
    // numbers is reported by the layout check of the documents.
    //
    // Table: the six numbers of the question (and the same integration by the IDA of this program)
    let mut t = table_head(&[
        ("$r^{\\rm tol}_{\\rm LL}$", 34),
        ("IDA, notebook", 62),
        ("IDA, this program", 62),
        ("CVODE", 56),
        ("IDA (notebook) divided by $r^{\\rm tol}_{\\rm LL}$", 78),
        ("CVODE divided by $r^{\\rm tol}_{\\rm LL}$", 62),
        ("IDA (notebook) divided by CVODE", 72),
    ]);
    for i in 0..3 {
        t.push_str(&format!(
            "| $10^{{{}}}$ | ${}$ | ${}$ | ${}$ | {:.0} | {:.0} | {:.2} |\n",
            rtols[i].log10().round() as i32, tex_sci(NB_DEV[i], 1), tex_sci(ida[i].run.deviation, 1), tex_sci(cv[i].run.deviation, 1), NB_DEV[i] / rtols[i], cv[i].run.deviation / rtols[i], NB_DEV[i] / cv[i].run.deviation
        ));
    }
    fr.add("t2_steps_table_six", t);
    // for the caption of this table: where the last internal step of CVODE ends, and what GS-13 reports at the time 20
    fr.add("t2_steps_cvode_end_times", format!("{:.4}, {:.4} and {:.4}", end_times[0], end_times[1], end_times[2]));
    fr.add("t2_steps_gs13_at_20", format!("${}$, ${}$ and ${}$", tex_sci(gs13_at_20[0], 3), tex_sci(gs13_at_20[1], 3), tex_sci(gs13_at_20[2], 3)));
    fr.add("t2_steps_gs13_at_20_tolerances", format!("{:.0}, {:.0} and {:.0}", gs13_at_20[0] / rtols[0], gs13_at_20[1] / rtols[1], gs13_at_20[2] / rtols[2]));

    // Table: the six deviations against the error budgets (2.40) (function `Summary::budget`; check GS-17)
    let mut t = table_head(&[
        ("integrator", 62),
        ("$r^{\\rm tol}_{\\rm LL}$", 46),
        ("$N^{\\rm st}_{\\rm LL}$", 46),
        ("$B^{\\rm loc}_{\\rm LL}/r^{\\rm tol}_{\\rm LL}$", 58),
        ("$B^{\\rm err}_{\\rm LL}$", 74),
        ("$\\delta^{\\rm rel}_{\\rm LL}$ at the end", 82),
        ("$\\delta^{\\rm rel}_{\\rm LL}/B^{\\rm err}_{\\rm LL}$", 58),
    ]);
    for x in &six {
        let budget = x.budget(n);
        t.push_str(&format!("| {} | $10^{{{}}}$ | {} | {:.3} | ${}$ | ${}$ | {:.5} |\n", x.integrator, x.rtol.log10().round() as i32, x.run.nsteps, budget / (x.run.nsteps as f64 * x.rtol), tex_sci(budget, 2), tex_sci(x.run.deviation, 3), x.run.deviation / budget));
    }
    fr.add("t2_steps_table_budget", t);

    // Table: the decomposition of the six deviations
    let mut t = table_head(&[
        ("integrator", 46),
        ("$r^{\\rm tol}_{\\rm LL}$", 30),
        ("$N^{\\rm st}_{\\rm LL}$", 30),
        ("mean of $\\delta^{\\rm loc}_{\\rm LL}$, divided by $r^{\\rm tol}_{\\rm LL}$", 72),
        ("$\\Xi^{\\rm coh}_{\\rm LL}$", 32),
        ("product: $\\delta^{\\rm rel}_{\\rm LL}$ at the end", 68),
        ("$\\delta^{\\rm rel}_{\\rm LL}/r^{\\rm tol}_{\\rm LL}$", 36),
        ("median cosine between the local errors of consecutive steps", 100),
    ]);
    for x in &six {
        t.push_str(&format!("| {} | $10^{{{}}}$ | {} | {:.4} | {:.4} | ${}$ | {:.0} | {} |\n", x.integrator, x.rtol.log10().round() as i32, x.run.nsteps, x.mean_local_over_rtol(), x.run.coherence(), tex_sci(x.run.deviation, 3), x.run.deviation / x.rtol, tex_signed(x.turn_median, 3, false)));
    }
    fr.add("t2_steps_table_decomposition", t);

    // Table: the settled steps and the local error test.  The upper end of the band is 1 for CVODE and (1 - 1e-4)/2
    // for IDA, printed with one digit.
    let mut t = table_head(&[
        ("integrator", 44),
        ("$r^{\\rm tol}_{\\rm LL}$", 26),
        ("settled steps", 30),
        ("$h^{\\rm st}_{\\rm LL}$", 66),
        ("order", 24),
        ("with 1, with 2 iterations", 48),
        ("$r^{\\rm cj}_{\\rm LL}$", 30),
        ("median of $\\delta^{\\rm est}_{\\rm LL}$", 32),
        ("its band", 58),
        ("median of $\\delta^{\\rm wrms}_{\\rm LL}$", 32),
    ]);
    for x in &six {
        let (h, order, nmain) = x.run.main_size();
        let (i1, i2, _) = iteration_counts(&x.run);
        let (lo, hi) = band(x.run.main_kind().0.method, order);
        let ratio = if x.integrator == "IDA" { format!("{:.4}", x.run.main_kind().0.ratio) } else { "(none)".to_string() };
        t.push_str(&format!("| {} | $10^{{{}}}$ | {} | ${}$ | {} | {}, {} | {} | {:.4} | {:.4} to {:.1} | {:.4} |\n", x.integrator, x.rtol.log10().round() as i32, nmain, tex_sci(h, 4), order, i1, i2, ratio, x.test_median, lo, hi, x.wrms_median));
    }
    fr.add("t2_steps_table_settled", t);

    // Table: the value of the local error test from the formula alone and as measured
    let mut t = table_head(&[
        ("integrator", 42),
        ("$r^{\\rm tol}_{\\rm LL}$", 26),
        ("order", 24),
        ("$h^{\\rm st}_{\\rm LL}$", 66),
        ("$\\delta^{\\rm est}_{\\rm LL}$ of an exactly solved step, from the formula", 80),
        ("median of the measured $\\delta^{\\rm est}_{\\rm LL}$", 56),
        ("measured divided by formula", 56),
        ("largest modulus of another root with one iteration", 64),
    ]);
    for (x, f) in six.iter().zip(&formula_tests) {
        let (h, order, _) = x.run.main_size();
        // a small value of the formula is printed as mantissa and power of ten, a large quotient without decimals
        let shown = if *f >= 0.01 { format!("{:.4}", f) } else { format!("${}$", tex_sci(*f, 2)) };
        let quotient = x.test_median / f;
        let quotient_shown = if quotient < 10.0 { format!("{:.3}", quotient) } else { format!("{:.0}", quotient) };
        t.push_str(&format!("| {} | $10^{{{}}}$ | {} | ${}$ | {} | {:.4} | {} | {:.4} |\n", x.integrator, x.rtol.log10().round() as i32, order, tex_sci(h, 4), shown, x.test_median, quotient_shown, other_root_one_iteration(&x.run, emax)));
    }
    fr.add("t2_steps_table_tests", t);

    // Two tables of the predictions.  First: the deviation at the end from the formulas alone, in three approximations
    // (the leading term; every implicit equation solved exactly; the formulas as the integrator applies them).
    let mut t = table_head(&[
        ("integrator", 46),
        ("$r^{\\rm tol}_{\\rm LL}$", 30),
        ("leading term", 84),
        ("with every step solved exactly", 96),
        ("from the formulas as applied", 96),
        ("measured $\\delta^{\\rm rel}_{\\rm LL}$", 86),
    ]);
    for x in &six {
        let (h, order, _) = x.run.main_size();
        // the leading term: the sum over the kinds of the main step size and order of (number of steps) x (constant)
        // x h^(order+1) x ||H^(order+1) w(0)||
        let p = order + 1;
        let lead: f64 = x.run.kinds.iter().filter(|(k, _)| k.h.to_bits() == h.to_bits() && k.order == order).map(|(k, c)| *c as f64 * leading_constant(k)).sum::<f64>() * h.powi(p) * spec.moment(p);
        t.push_str(&format!(
            "| {} | $10^{{{}}}$ | ${}$ | ${}$ | ${}$ | ${}$ |\n",
            x.integrator, x.rtol.log10().round() as i32, tex_sci(lead, 3), tex_sci(x.run.predicted_exact, 3), tex_sci(x.run.predicted_formula, 3), tex_sci(x.run.deviation, 3)
        ));
    }
    fr.add("t2_steps_table_formulas", t);

    // Second: the predicted deviation, the formulas as applied together with the rounding of the time, against the
    // measured one
    let mut t = table_head(&[
        ("integrator", 46),
        ("$r^{\\rm tol}_{\\rm LL}$", 30),
        ("from the formulas as applied", 74),
        ("from the rounding of the time", 74),
        ("both: $\\delta^{\\rm pre}_{\\rm LL}$", 70),
        ("measured $\\delta^{\\rm rel}_{\\rm LL}$", 70),
        ("$\\delta^{\\rm pre}_{\\rm LL}$/measured", 62),
    ]);
    for x in &six {
        t.push_str(&format!(
            "| {} | $10^{{{}}}$ | ${}$ | ${}$ | ${}$ | ${}$ | {:.4} |\n",
            x.integrator, x.rtol.log10().round() as i32, tex_sci(x.run.predicted_formula, 3), tex_sci(x.run.predicted_time, 2), tex_sci(x.run.predicted, 3), tex_sci(x.run.deviation, 3), x.run.predicted / x.run.deviation
        ));
    }
    fr.add("t2_steps_table_prediction", t);

    // Table: the constants of the leading terms and the moments of the initial state
    let gs = formulas::c_am(8);
    let mut t = table_head(&[
        ("order", 50),
        ("Adams-Moulton, $\\lvert c^{\\rm AM}_{{\\rm LL},q}\\rvert$", 116),
        ("backward differentiation, $1/(k^{\\rm IDA}_{\\rm LL}+1)$", 164),
        ("$\\lVert(\\mathcal H^{\\rm lat}_{\\rm LL})^{\\rm order+1}w^{\\rm lat}_{\\rm LL}(0)\\rVert$", 132),
    ]);
    for order in 1..=7i32 {
        t.push_str(&format!("| {} | {:.6} | {} | ${}$ |\n", order, gs[order as usize].to_f64().abs(), if order <= 5 { format!("{:.6}", 1.0 / (order + 1) as f64) } else { "(none)".to_string() }, tex_sci(spec.moment(order + 1), 4)));
    }
    fr.add("t2_steps_table_constants", t);

    // Table: IDA here and in the notebook, quantity by quantity
    let mut t = table_head(&[
        ("$r^{\\rm tol}_{\\rm LL}$", 36),
        ("quantity", 140),
        ("IDA of the notebook", 126),
        ("IDA of the program `t2_fields`", 160),
    ]);
    for i in 0..3 {
        let hs: Vec<f64> = ida[i].steps.iter().map(|s| s.h).collect();
        let tol = format!("$10^{{{}}}$", rtols[i].log10().round() as i32);
        t.push_str(&format!("| {} | internal steps, $N^{{\\rm st}}_{{\\rm LL}}$ | {} | {} |\n", tol, NB_STEPS[i], ida[i].run.nsteps));
        t.push_str(&format!("| {} | largest internal step | ${}$ | ${}$ |\n", tol, tex_sci(NB_HMAX[i], 9), tex_sci(hs.iter().cloned().fold(0.0f64, f64::max), 9)));
        t.push_str(&format!("| {} | smallest internal step | ${}$ | ${}$ |\n", tol, tex_sci(NB_HMIN[i], 9), tex_sci(hs.iter().cloned().fold(f64::INFINITY, f64::min), 9)));
        t.push_str(&format!("| {} | settled internal step, 16 digits | ${}$ | ${}$ |\n", tol, tex_sci(dyadic(NB_H_LOWEST[i]), 15), tex_sci(ida[i].run.main_size().0, 15)));
        t.push_str(&format!(
            "| {} | evaluations of the right side, $N^{{\\rm ev}}_{{\\rm LL}}$ | {} | ${}+769\\times{}+1={}$ |\n",
            tol, NB_EVALS[i], ida[i].stats.nrevals, ida[i].stats.njevals, ida[i].stats.nrevals + 769 * ida[i].stats.njevals + 1
        ));
        t.push_str(&format!("| {} | $\\delta^{{\\rm rel}}_{{\\rm LL}}$ at $x^0_{{\\rm LL}}=20$ | ${}$ | ${}$ |\n", tol, tex_sci(NB_DEV[i], 6), tex_sci(ida[i].run.deviation, 6)));
    }
    fr.add("t2_steps_table_notebook", t);

    // Table: how the settled step size of IDA comes about (one column per tolerance)
    let mut t = table_head(&[("quantity", 126), ("$r^{\\rm tol}_{\\rm LL}=10^{-8}$", 112), ("$10^{-10}$", 112), ("$10^{-12}$", 112)]);
    {
        let row = |name: &str, f: &dyn Fn(usize) -> String| -> String { format!("| {} | {} | {} | {} |\n", name, f(0), f(1), f(2)) };
        t.push_str(&row("first step size that IDA tries", &|_| "$10^{-4}$".to_string()));
        t.push_str(&row("attempts of the first step that the test (2.44) rejects; each divides the step size by 4", &|i| format!("{}", hist[i].quarters)));
        t.push_str(&row("size of the first accepted step", &|i| format!("${}$", tex_short(ida[i].steps[0].h))));
        t.push_str(&row("shortenings by the fixed factor 0.9 after an accepted step", &|i| format!("{}", hist[i].tenths)));
        t.push_str(&row("doublings after an accepted step", &|i| format!("{}", hist[i].doublings)));
        t.push_str(&row("step after which IDA applies a computed factor", &|i| format!("number {}", hist[i].computed[0].0)));
        t.push_str(&row("its order $k^{\\rm IDA}_{\\rm LL}$", &|i| format!("{}", hist[i].computed[0].1)));
        t.push_str(&row("its test value $\\delta^{\\rm est}_{\\rm LL}$", &|i| format!("{:.6}", hist[i].computed[0].2)));
        t.push_str(&row("quotient $\\sigma^{\\rm IDA}_{\\rm LL}/c^{\\rm est}_{\\rm LL}$ of its two coefficients (2.66)", &|i| format!("{:.6}", hist[i].computed[0].3 / hist[i].computed[0].2)));
        t.push_str(&row("estimate for the next step, $\\delta^{\\rm nxt}_{\\rm LL}$", &|i| format!("{:.6}", hist[i].computed[0].3)));
        t.push_str(&row("computed factor $\\eta^{\\rm cmp}_{\\rm LL}$ of (2.65)", &|i| format!("{:.12}", hist[i].computed[0].4)));
        t.push_str(&row("settled step size: $10^{-4}$ times all factors", &|i| format!("${}$", tex_sci(ida[i].run.main_size().0, 12))));
        t.push_str(&row("number of the first step of this size", &|i| format!("{}", hist[i].first)));
        t.push_str(&row("number of steps of this size", &|i| format!("{}", hist[i].settled)));
    }
    fr.add("t2_steps_table_history", t);

    // Table: three integrations of every tolerance
    let mut t = table_head(&[
        ("$r^{\\rm tol}_{\\rm LL}$", 28),
        ("integration", 80),
        ("settled step size / that of the first line, minus 1", 64),
        ("$\\delta^{\\rm t}_{\\rm LL}$", 66),
        ("$\\delta^{\\rm rel}_{\\rm LL}$ at the end", 68),
        ("$\\delta^{\\rm rel}_{\\rm LL}$ / that of the first line, minus 1", 60),
        ("the same, predicted from $\\delta^{\\rm t}_{\\rm LL}$", 60),
    ]);
    for i in 0..3 {
        let tol = format!("$10^{{{}}}$", rtols[i].log10().round() as i32);
        let small = |x: f64| -> String {
            if x.abs() >= 1e-3 {
                tex_signed(x, 4, false)
            } else {
                format!("${}$", tex_sci(x, 2))
            }
        };
        t.push_str(&format!("| {} | program, exact matrix | 0 | ${}$ | ${}$ | 0 | 0 |\n", tol, tex_sci(ida[i].run.time_defect, 2), tex_sci(ida[i].run.deviation, 4)));
        t.push_str(&format!(
            "| {} | program, difference quotients | {} | ${}$ | ${}$ | {} | {} |\n",
            tol, if settled_rel_q[i].abs() < 1e-12 { format!("${}$", tex_sci(settled_rel_q[i], 1)) } else { format!("${}$", tex_sci(settled_rel_q[i], 2)) }, tex_sci(idq[i].run.time_defect, 2), tex_sci(idq[i].run.deviation, 4), small(diff_q[i].0), small(diff_q[i].1)
        ));
        t.push_str(&format!(
            "| {} | notebook | {} | ${}$ | ${}$ | {} | {} |\n",
            tol, if nb_settled_rel[i].abs() < 1e-12 { format!("${}$", tex_sci(nb_settled_rel[i], 1)) } else { format!("${}$", tex_sci(nb_settled_rel[i], 2)) }, tex_sci(nb_defect_mid(i), 2), tex_sci(NB_DEV[i], 4), small(diff_nb[i].0), small(diff_nb[i].1)
        ));
    }
    fr.add("t2_steps_table_realizations", t);

    // Table: the predicted deviation over one period of the last digits, and the three integrations
    let mut t = table_head(&[("line", 200), ("deviation at the end", 90), ("divided by $r^{\\rm tol}_{\\rm LL}$", 72), ("part of the period with a smaller $\\delta^{\\rm pre}_{\\rm LL}$", 100)]);
    for (name, q) in [("$\\delta^{\\rm pre}_{\\rm LL}$: smallest value of the period", 0.0), ("5 per cent of the period below", 0.05), ("a quarter below", 0.25), ("half below (median)", 0.5), ("three quarters below", 0.75), ("95 per cent below", 0.95), ("largest value", 1.0)] {
        let v = if q == 0.0 { ld_pred[0] } else if q == 1.0 { ld_pred[period - 1] } else { ld_q(q) };
        t.push_str(&format!("| {} | ${}$ | {:.1} | {:.2} |\n", name, tex_sci(v, 3), v / 1e-12, q));
    }
    t.push_str(&format!("| $\\delta^{{\\rm pre}}_{{\\rm LL}}$ of the formulas alone ($\\delta^{{\\rm t}}_{{\\rm LL}}=0$) | ${}$ | {:.1} | {:.2} |\n", tex_sci(formula_alone, 3), formula_alone / 1e-12, ld_below(formula_alone)));
    for (name, v, b) in [("measured $\\delta^{\\rm rel}_{\\rm LL}$: program, exact matrix", ida[2].run.deviation, below[0]), ("measured: program, difference quotients", idq[2].run.deviation, below[1]), ("measured: notebook", NB_DEV[2], below[2])] {
        t.push_str(&format!("| {} | ${}$ | {:.1} | {:.2} |\n", name, tex_sci(v, 3), v / 1e-12, b));
    }
    fr.add("t2_steps_table_last_digits", t);
    fr.add("t2_steps_defect_bound", format!("${}$", tex_sci(defect_bound, 2)));
    fr.add("t2_steps_defect_largest", format!("${}$", tex_sci(ld_defect_max, 2)));
    fr.add("t2_steps_units_12", format!("{}", units.len()));
    fr.add("t2_steps_pred12nb", format!("${}$", tex_sci(predicted_with(2, nb_defect_mid(2)), 3)));
    fr.add("t2_steps_unit_sum", format!("${}$", tex_sci(unit_sum, 1)));

    // Table: where the ratio cj/cjJ of the settled steps of IDA comes from
    let mut t = table_head(&[
        ("$r^{\\rm tol}_{\\rm LL}$", 28),
        ("last step with a current matrix", 50),
        ("its $h^{\\rm IDA}_{\\rm LL}$", 66),
        ("its $k^{\\rm IDA}_{\\rm LL}$", 22),
        ("settled steps: $h^{\\rm IDA}_{\\rm LL}$", 66),
        ("$k^{\\rm IDA}_{\\rm LL}$", 22),
        ("quotient of the two $c^{\\rm ld}_{\\rm LL}=H^{\\rm har}_{\\rm LL}/h^{\\rm IDA}_{\\rm LL}$", 72),
        ("$r^{\\rm cj}_{\\rm LL}$ that IDA reports", 40),
        ("steps taken with this matrix", 36),
    ]);
    for (i, r) in ida.iter().enumerate() {
        let main = r.run.main_kind().0;
        let o = &origins[i];
        t.push_str(&format!("| $10^{{{}}}$ | number {} | ${}$ | {} | ${}$ | {} | {:.6} | {:.6} | {} |\n", rtols[i].log10().round() as i32, o.0, tex_sci(o.1, 4), o.2, tex_sci(main.h, 4), main.order, o.3, main.ratio, r.run.nsteps - o.0));
    }
    fr.add("t2_steps_table_origin", t);

    // Table: the limits of stability of the Adams-Moulton formulas as CVODE applies them, on the imaginary axis
    let mut t = table_head(&[
        ("$q^{\\rm CV}_{\\rm LL}$", 30),
        ("limit of $\\lvert z^{\\rm st}_{\\rm LL}\\rvert$ with $n^{\\rm fp}_{\\rm LL}=1$", 110),
        ("with $n^{\\rm fp}_{\\rm LL}=2$", 62),
        ("with $n^{\\rm fp}_{\\rm LL}=3$", 62),
        ("gain of the predictor", 74),
        ("limit with $n^{\\rm fp}_{\\rm LL}=1$ times the gain", 100),
    ]);
    for q in 1..=12usize {
        let lim = |m: u32| -> String {
            match formulas::adams_stability_limit(q, m) {
                Some(x) => format!("{:.4}", x),
                None => "above 1".to_string(),
            }
        };
        let gain = formulas::adams_predictor_gain(q);
        let l1 = formulas::adams_stability_limit(q, 1).unwrap_or(f64::NAN);
        t.push_str(&format!("| {} | {} | {} | {} | {:.2} | {:.3} |\n", q, lim(1), lim(2), lim(3), gain, l1 * gain));
    }
    fr.add("t2_steps_table_limits_adams", t);

    // Table: the interval of stability of the backward differentiation formulas with one Newton step (IDA)
    let mut t = table_head(&[
        ("$k^{\\rm IDA}_{\\rm LL}$", 40),
        ("stable for $r^{\\rm cj}_{\\rm LL}$ from", 126),
        ("to", 126),
        ("the alternating root reaches $-1$ at $r^{\\rm cj}_{\\rm LL}=$", 170),
    ]);
    for k in 1..=5usize {
        let (lo, hi) = formulas::bdf_stability_window(k);
        let exact = formulas::bdf_alternating_bound_exact(k);
        // the closed form 1/(1 - 2 c) is a fraction; the fractions printed here are compared with it
        let (num, den) = [(0.0f64, 0.0f64), (3.0, 1.0), (11.0, 6.0), (25.0, 17.0), (137.0, 105.0)][k - 1];
        let fraction = if den == 0.0 {
            assert!(exact.is_infinite());
            "(never)".to_string()
        } else {
            assert!((exact - num / den).abs() < 1e-12, "the bound of the order {}", k);
            if den == 1.0 { format!("{:.0}", num) } else { format!("{:.0}/{:.0} = {:.4}", num, den, num / den) }
        };
        let shown = |x: f64, limit: f64| if (x - limit).abs() < 1e-9 { format!("{:.4} (the limit of IDA)", x) } else { format!("{:.4}", x) };
        t.push_str(&format!("| {} | {} | {} | {} |\n", k, shown(lo, 0.6), shown(hi, 5.0 / 3.0), fraction));
    }
    fr.add("t2_steps_table_limits_bdf", t);

    // Table: the six integrations against these limits
    let mut t = table_head(&[
        ("integrator", 46),
        ("$r^{\\rm tol}_{\\rm LL}$", 30),
        ("order", 28),
        ("$E^{\\rm max}_{\\rm LL}h^{\\rm st}_{\\rm LL}$", 44),
        ("$r^{\\rm cj}_{\\rm LL}$", 36),
        ("largest modulus of another root with one iteration", 88),
        ("with two iterations", 56),
        ("settled steps with one, with two iterations", 86),
    ]);
    for x in &six {
        let k = x.run.main_kind().0;
        let (one, two) = match k.method {
            Method::Adams => (formulas::adams_parasitic_modulus(k.order as usize, emax * k.h, 1), formulas::adams_parasitic_modulus(k.order as usize, emax * k.h, 2)),
            Method::Bdf => (
                formulas::bdf_parasitic_modulus(k.order as usize, C64 { re: 0.0, im: -emax * k.h }, k.ratio, 1),
                formulas::bdf_parasitic_modulus(k.order as usize, C64 { re: 0.0, im: -emax * k.h }, k.ratio, 2),
            ),
        };
        let (i1, i2, _) = iteration_counts(&x.run);
        let ratio = if x.integrator == "IDA" { format!("{:.4}", k.ratio) } else { "(none)".to_string() };
        t.push_str(&format!("| {} | $10^{{{}}}$ | {} | {:.4} | {} | {:.4} | {:.4} | {}, {} |\n", x.integrator, x.rtol.log10().round() as i32, k.order, emax * k.h, ratio, one, two, i1, i2));
    }
    fr.add("t2_steps_table_roots", t);

    // Table: the rounding of the time variable of IDA at rtol = 1e-12
    let mut t = table_head(&[
        ("range of the time", 150),
        ("one unit in the last place", 70),
        ("rounding of one addition, in units", 76),
        ("settled steps that start in the range", 76),
        ("their sum: change of $\\delta^{\\rm t}_{\\rm LL}$", 78),
    ]);
    let ranges = [(4.0f64, 8.0f64, "4 to 8"), (8.0, 16.0, "8 to 16"), (16.0, 32.0, "16 to 20")];
    let mut total = 0.0f64;
    for (i, (lo, hi, name)) in ranges.iter().enumerate() {
        let unit = f64::EPSILON * lo;
        let count = count_in(*lo, *hi);
        let change = -count * ulp_errors[i] * unit;
        total += change;
        t.push_str(&format!("| {} | ${}$ | {} | {:.0} | ${}$ |\n", name, tex_sci(unit, 3), tex_signed(ulp_errors[i], 4, true), count, tex_sci(change, 3)));
    }
    t.push_str(&format!("| 4 to 20, sum of the three lines | | | | ${}$ |\n| 4 to 20, measured | | | | ${}$ |\n| 0 to 20, measured ($\\delta^{{\\rm t}}_{{\\rm LL}}$ at the end) | | | | ${}$ |\n", tex_sci(total, 3), tex_sci(defect_4_to_20, 3), tex_sci(ida[2].run.time_defect, 3)));
    fr.add("t2_steps_table_rounding", t);

    // Table: the experiment with a matrix that is always current (the numbers of matrices that IDA computed are in
    // the column `matrices` of t2_steps_runs.csv)
    let mut t = table_head(&[
        ("$r^{\\rm tol}_{\\rm LL}$", 26),
        ("matrix", 62),
        ("$N^{\\rm st}_{\\rm LL}$", 26),
        ("$h^{\\rm st}_{\\rm LL}$ of the settled steps", 66),
        ("$r^{\\rm cj}_{\\rm LL}$", 30),
        ("median of $\\delta^{\\rm est}_{\\rm LL}$", 32),
        ("$\\Xi^{\\rm coh}_{\\rm LL}$", 30),
        ("$\\delta^{\\rm rel}_{\\rm LL}$ at the end", 64),
        ("$\\delta^{\\rm pre}_{\\rm LL}$", 66),
    ]);
    for i in 0..3 {
        for x in [&sums[3 + i], &sums[6 + i]] {
            let (h, _, _) = x.run.main_size();
            t.push_str(&format!("| $10^{{{}}}$ | {} | {} | ${}$ | {:.4} | {:.4} | {:.4} | ${}$ | ${}$ |\n", x.rtol.log10().round() as i32, if x.matrix == "default" { "default of IDA" } else { "always current" }, x.run.nsteps, tex_sci(h, 4), x.run.main_kind().0.ratio, x.test_median, x.run.coherence(), tex_sci(x.run.deviation, 3), tex_sci(x.run.predicted, 3)));
        }
    }
    fr.add("t2_steps_table_matrix", t);

    // Table: the scan
    let mut t = table_head(&[
        ("$r^{\\rm tol}_{\\rm LL}$", 60),
        ("CVODE: $N^{\\rm st}_{\\rm LL}$", 34),
        ("$h^{\\rm CV}_{\\rm LL}$", 60),
        ("$q^{\\rm CV}_{\\rm LL}$", 18),
        ("$\\delta^{\\rm rel}_{\\rm LL}/r^{\\rm tol}_{\\rm LL}$", 34),
        ("IDA: $N^{\\rm st}_{\\rm LL}$", 28),
        ("$h^{\\rm IDA}_{\\rm LL}$", 60),
        ("$r^{\\rm cj}_{\\rm LL}$", 26),
        ("$\\delta^{\\rm rel}_{\\rm LL}/r^{\\rm tol}_{\\rm LL}$", 34),
        ("$\\delta^{\\rm rel}_{\\rm LL}$ of IDA / of CVODE", 36),
    ]);
    for (i, &rt) in scan_rtols.iter().enumerate() {
        let (hc, qc, _) = scan_cv[i].main_size();
        let (hi, _, _) = scan_ida[i].main_size();
        let mark = |r: &Run| if regular(r) { "" } else { " (m)" };
        t.push_str(&format!(
            "| ${}$ | {} | ${}$ | {} | {:.0}{} | {} | ${}$ | {:.3} | {:.0}{} | {:.3} |\n",
            tex_sci(rt, 2), scan_cv[i].nsteps, tex_sci(hc, 3), qc, scan_cv[i].deviation / rt, mark(&scan_cv[i]), scan_ida[i].nsteps, tex_sci(hi, 3), scan_ida[i].main_kind().0.ratio, scan_ida[i].deviation / rt, mark(&scan_ida[i]), scan_ida[i].deviation / scan_cv[i].deviation
        ));
    }
    fr.add("t2_steps_table_scan", t);
    fr.add("t2_steps_formula_rows", format!("{}", formula_rows));
}

/// a number with three decimals of the mantissa
fn sci3(x: f64) -> String {
    format!("{:.3e}", x)
}
