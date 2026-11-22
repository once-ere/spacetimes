//! sf_part1 -- the machine checks and the analytic solutions of part I of the manuscript superfluid.md
//! (folder Revision_SM_Higgs of PROJECT spacetimes): the bi-octonions, the super-loop and the manifold
//! M31 (checks A), the spinor field on M31 and theorem D1 (checks F), the gauge group GEE and its covariant
//! derivatives (checks C), the field equations as Heisenberg equations (checks E), the analytic solutions
//! AS1 to AS4 (checks S), and the conventions of the Standard Model (check SM-1).
//!
//! Build and run (from the folder rust/ of Revision_SM_Higgs):
//!     cargo build --release
//!     ./target/release/sf_part1
//! Output (folder data/ of Revision_SM_Higgs, or the folder named by the environment variable SF_DATA):
//!     hck_part1_checks.txt              the report of the checks, one line per check, and the verdict
//!     part1_m31_examples.csv            worked examples of M31
//!     part1_s1_locked_dispersion.csv    the frequencies of AS1 in the spin-isospin locked field
//!     part1_s2_landau.csv               the Landau levels of AS2: lattice and analytic
//!     part1_s3_energy.csv, part1_s3_gap.csv   the energy and the gap of the uniform superfluid S3
//!     part1_s4_kink_lattice.csv, part1_s4_kink_profiles.csv   the kink S4
//!     part1_values.csv                  the named values printed in the manuscript
//! The exit status is 0 when every check passes and 1 otherwise.
mod a_algebra;
mod c_gauge;
mod e_fieldeq;
mod f_spinor;
mod s_solutions;
mod sm_conventions;
mod ym_equations;

use sfcore::report::{data_dir, Report};
use std::time::Instant;

fn main() {
    let t0 = Instant::now();
    let data = data_dir(env!("CARGO_MANIFEST_DIR"));
    let mut rep = Report::new(
        "sf_part1: checks of part I of superfluid.md (Revision_SM_Higgs, PROJECT spacetimes)",
    );
    println!("== A: bi-octonions, super-loop, M31");
    a_algebra::run(&mut rep, &data);
    println!("== F: the spinor field and theorem D1");
    f_spinor::run(&mut rep);
    println!("== C: GEE and the covariant derivatives");
    c_gauge::run(&mut rep);
    println!("== E: the field equations as Heisenberg equations");
    e_fieldeq::run(&mut rep);
    ym_equations::run(&mut rep);
    println!("== S: the analytic solutions");
    let vals = s_solutions::run(&mut rep, &data);
    println!("== SM: the conventions of the Standard Model");
    sm_conventions::run(&mut rep);
    let rows: Vec<Vec<String>> = vals.iter().map(|(k, v)| vec![k.clone(), sfcore::report::f(*v)]).collect();
    sfcore::report::write_csv(&data.join("part1_values.csv"), &["name", "value"], &rows);
    rep.note(&format!("run time {:.1} s", t0.elapsed().as_secs_f64()));
    rep.write(&data.join("hck_part1_checks.txt"));
    let (np, n) = rep.counts();
    println!("{} of {} checks passed", np, n);
    std::process::exit(if np == n { 0 } else { 1 });
}
