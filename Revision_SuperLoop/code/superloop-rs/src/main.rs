//! superloop -- PROJECT spacetimes, Revision_SuperLoop.
//!
//! The no-go theorems that forbid a nontrivial combination of Lorentz and internal symmetries
//! (Coleman-Mandula, McGlinn, O'Raifeartaigh, Haag-Lopuszanski-Sohnius) use associativity in
//! several places: the generators are operators (so their commutators obey the Jacobi
//! identity), symmetries form groups, generators act as representations, and charges add on
//! products of states.  The octonions and the split octonions are not associative.  This
//! program builds the Lorentz super-loop -- the Moufang loop of unit bi-octonions, which
//! contains SL(2,C) = Spin(3,1)^+ and spinor charges that close into the Lorentz algebra --,
//! its operators (electric charge, chirality, spinor number, twist charge), the inversion
//! element, the classification of Lorentz-covariant Malcev extensions (the companion script
//! code/scripts/malcev_groebner.py computes the Groebner bases), the manifolds that carry the
//! super-loop, the spin-zero spinor superfluid of its spinor charges, and power counting.
//!
//! Usage (from the folder Revision_SuperLoop; every output goes to Revision_SuperLoop/data):
//!
//!     cargo run --release --manifest-path code/superloop-rs/Cargo.toml -- all
//!
//! or one command at a time:  algebra | malcev | manifolds | superfluid | renorm
//! Each command writes its data files and a report data/sl_<command>_checks.txt; the
//! program exits with status 1 if any check fails.

mod algebra;
mod bioct;
mod checks;
mod exact;
mod malcev;
mod manifolds;
mod renorm;
mod superfluid;

fn main() {
    if !std::path::Path::new("code/superloop-rs/Cargo.toml").exists() {
        eprintln!("superloop: run this program from the folder Revision_SuperLoop (the folder that contains code/superloop-rs)");
        std::process::exit(2);
    }
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let t0 = std::time::Instant::now();
    let table: Vec<(&str, fn() -> bool)> = vec![("algebra", algebra::run), ("malcev", malcev::run), ("manifolds", manifolds::run), ("superfluid", superfluid::run), ("renorm", renorm::run)];
    let ok = if cmd == "all" {
        let mut ok = true;
        for (name, f) in &table {
            println!("==================== {}", name);
            ok &= f();
        }
        ok
    } else if let Some((_, f)) = table.iter().find(|(n, _)| *n == cmd) {
        f()
    } else {
        println!("usage: superloop algebra | malcev | manifolds | superfluid | renorm | all   (run from the folder Revision_SuperLoop)");
        std::process::exit(2);
    };
    println!("{} finished in {:.1} s: {}", cmd, t0.elapsed().as_secs_f64(), if ok { "ALL CHECKS PASSED" } else { "SOME CHECK FAILED" });
    if !ok {
        std::process::exit(1);
    }
}
