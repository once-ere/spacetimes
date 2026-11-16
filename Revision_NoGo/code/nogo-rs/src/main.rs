//! nogo -- PROJECT spacetimes, Revision_NoGo.
//!
//! The no-go theorems that forbid a nontrivial combination of Lorentz (space-time)
//! and internal symmetries (Coleman-Mandula, McGlinn, O'Raifeartaigh, and the
//! graded extension of Haag, Lopuszanski and Sohnius), the places where their
//! hypotheses fail in eight dimensions -- on the 7+1 octonionic manifold and on
//! the 4+4 split-octonionic manifold -- and the spin-zero superfluid of the
//! 16-component Dirac field on the combined Lorentz-plus-internal manifolds
//! M(3,1) x K(4) defined by those constructions; power counting decides whether
//! the combined manifolds cure the non-renormalizability of the 8D theory.
//!
//! Usage (from the folder Revision_NoGo; every output goes to Revision_NoGo/data):
//!
//!     cargo run --release --manifest-path code/nogo-rs/Cargo.toml -- all
//!
//! or one command at a time:
//!     octonions | triality | kaluza | superalgebra | pairs4d | renorm
//! Each command writes its data files and a report data/ng_<command>_checks.txt;
//! the program exits with status 1 if any check fails.

mod checks;
mod exact;
mod integrate;
mod kaluza;
mod oct;
mod pairs4d;
mod renorm;
mod rsym;
mod sig;
mod superalg;
mod triality;

fn main() {
    // Every output goes to ./data; refuse to run from any folder other than
    // Revision_NoGo (the folder that contains code/nogo-rs/Cargo.toml), so that the
    // data files can never land in another folder of the project.
    if !std::path::Path::new("code/nogo-rs/Cargo.toml").exists() {
        eprintln!("nogo: run this program from the folder Revision_NoGo (the folder that contains code/nogo-rs)");
        std::process::exit(2);
    }
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let t0 = std::time::Instant::now();
    let ok = match cmd {
        "octonions" => oct::run(),
        "triality" => triality::run(),
        "kaluza" => kaluza::run(),
        "superalgebra" => superalg::run(),
        "pairs4d" => pairs4d::run(),
        "renorm" => renorm::run(),
        "all" => {
            let mut ok = true;
            for (name, f) in [("octonions", oct::run as fn() -> bool), ("triality", triality::run), ("kaluza", kaluza::run), ("superalgebra", superalg::run), ("pairs4d", pairs4d::run), ("renorm", renorm::run)] {
                println!("==================== {}", name);
                ok &= f();
            }
            ok
        }
        _ => {
            println!("usage: nogo octonions | triality | kaluza | superalgebra | pairs4d | renorm | all   (run from the folder Revision_NoGo)");
            true
        }
    };
    println!("{} finished in {:.1} s: {}", cmd, t0.elapsed().as_secs_f64(), if ok { "ALL CHECKS PASSED" } else { "SOME CHECK FAILED" });
    std::process::exit(if ok { 0 } else { 1 });
}
