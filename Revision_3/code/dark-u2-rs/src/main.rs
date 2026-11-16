//! dark_u2 -- PROJECT spacetimes, Revision_3.
//!
//! A new gauged symmetry U(2)_U2 = SU(2)_U2 x U(1)_U2 of a doublet Psi_U2 = (psi^1_U2,
//! psi^2_U2) of 16-component Dirac fields on the 7+1 Minkowski spacetime, its
//! self-interacting Dirac equations (quadratic and quartic terms), the effective
//! potential of the spin-zero pair condensate, the spontaneous breaking
//! U(2)_U2 -> SU(2)_U2 at its minimum, and the collective modes.
//!
//! Usage (from the folder Revision_3; every output goes to Revision_3/data):
//!
//!     cargo run --release --manifest-path code/dark-u2-rs/Cargo.toml -- all
//!
//! or one command at a time: algebra | gauge | bdg | potential | modes.
//! Each command writes its data files and a report data/r3_<command>_checks.txt;
//! the program exits with status 1 if any check fails.

mod algebra;
mod bdg;
mod checks;
mod gauge;
mod integrate;
mod modes;
mod potential;
mod rsym;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let t0 = std::time::Instant::now();
    let ok = match cmd {
        "algebra" => algebra::run(),
        "gauge" => gauge::run(),
        "bdg" => bdg::run(),
        "potential" => potential::run(),
        "modes" => modes::run(),
        "all" => {
            let mut ok = true;
            for (name, f) in [("algebra", algebra::run as fn() -> bool), ("gauge", gauge::run), ("bdg", bdg::run), ("potential", potential::run), ("modes", modes::run)] {
                println!("==================== {}", name);
                ok &= f();
            }
            ok
        }
        _ => {
            println!("usage: dark_u2 algebra | gauge | bdg | potential | modes | all   (run from the folder Revision_3)");
            true
        }
    };
    println!("{} finished in {:.1} s: {}", cmd, t0.elapsed().as_secs_f64(), if ok { "ALL CHECKS PASSED" } else { "SOME CHECK FAILED" });
    std::process::exit(if ok { 0 } else { 1 });
}
