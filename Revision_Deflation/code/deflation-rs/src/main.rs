//! Command-line program of PROJECT spacetimes, Revision_Deflation.
//!
//! Run it from the folder `Revision_Deflation` (the data files are written to
//! `data/` below the current directory; the notebook matrices of the main project
//! are read from `../data/`):
//!
//!     cargo run --release --manifest-path code/deflation-rs/Cargo.toml -- <command>
//!
//! `deflation help` lists the commands.

use spacetimes_deflation::io::{write_csv, write_report};
use spacetimes_deflation::curved::verify_curved;
use spacetimes_deflation::geometry::{verify_geodesics, verify_geometry};
use spacetimes_deflation::metric::{Point, Sig};

/// Folder of the data files of the main project (the matrices extracted from the notebooks).
fn main_data_dir() -> String {
    std::env::var("SPT_MAIN_DATA_DIR").unwrap_or_else(|_| "../data".to_string())
}

/// `matrices`: (1) repeats every check of the real 16x16 Dirac matrices of the main
/// project (they are the matrices of the tangent space of the deflating spacetimes);
/// (2) checks the geometry of the deflating spacetimes; (3) checks the coordinate
/// matrices, the spin connection and the Dirac operators.
/// Writes data/flat_matrix_checks.{txt,csv}, data/deflating_checks.{txt,csv},
/// data/geodesic_1644d.csv, data/geodesic_1671d.csv, data/curvature.csv.
fn cmd_matrices() -> std::io::Result<bool> {
    std::fs::create_dir_all("data")?;
    // ---- (1) the flat matrices, verified again ---------------------------------------
    let nb44 = format!("{}/notebook_matrices_4+4.txt", main_data_dir());
    let nb71 = format!("{}/notebook_matrices_7+1.txt", main_data_dir());
    let (gm, mut flat) = spacetimes::verify::run_all(&nb44, &nb71);
    spacetimes::dirac::verify_dirac_44(&gm, &mut flat);
    spacetimes::dirac::verify_dirac_71(&gm, &mut flat);
    let pair_lines = spacetimes::pairs::run(&gm, &mut flat)?; // also writes data/pair_classification.csv
    std::fs::write("data/pair_classification.txt", pair_lines.join("\n") + "\n")?;
    let (ok_flat, n_flat) = flat.count();
    let all_flat = flat.all_pass();
    flat.add("ALL", "exact", "every check above passed", all_flat);
    write_report(&flat, "flat_matrix_checks.txt", "flat_matrix_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: the checks of the real 16x16 Dirac matrices of the main project, executed again (these matrices are the matrices of the orthonormal frame of the deflating spacetimes); identifiers 44-.. refer to gamma1644, 71-.. to gamma1671", "deflation matrices")?;
    // the routine of the main project wrote data/pair_classification.csv with its own first line; say who called it
    let pc = std::fs::read_to_string("data/pair_classification.csv")?;
    let mut lines: Vec<String> = pc.lines().map(|l| l.to_string()).collect();
    if !lines.is_empty() {
        lines[0] = format!("# {} through the routine pairs::run of the crate spacetimes; command: deflation matrices", spacetimes_deflation::io::generator_line());
    }
    std::fs::write("data/pair_classification.csv", lines.join("\n") + "\n")?;
    println!("flat matrices (tangent space): {} of {} checks passed", ok_flat, n_flat);

    // ---- (2), (3) the deflating spacetimes -----------------------------------------
    let mut rep = spacetimes::verify::Report::new();
    for sig in [Sig::S1644, Sig::S1671] {
        verify_geometry(sig, &mut rep);
        let geo = verify_geodesics(sig, &mut rep);
        let rows: Vec<Vec<f64>> = geo.path.iter().step_by((geo.path.len() / 2000).max(1)).map(|r| r.to_vec()).collect();
        write_csv(&format!("geodesic_{}.csv", sig.tag()),
            &[("content", format!("a geodesic of the deflating {} spacetime with momentum along the deflating coordinate x5 (check {}): g55 = 1, const5 = 0.8, M = 1, P_1 = 0.5, P_5 = 0.3, start at x4 = -2", sig.name(), if sig == Sig::S1671 { "71d-G8" } else { "44d-G9" })),
              ("columns", "lam: affine parameter (proper time for M = 1); x4; u4 = dx4/dlam".to_string()),
              ("command", "deflation matrices".to_string())],
            &["lam", "x4", "u4"], &rows)?;
        verify_curved(&gm, sig, &mut rep);
    }
    let (ok, n) = rep.count();
    let all = rep.all_pass();
    rep.add("ALL", "exact", "every check above passed", all);
    write_report(&rep, "deflating_checks.txt", "deflating_checks.csv",
        "PROJECT spacetimes, Revision_Deflation: checks of the geometry, of the Dirac matrices, of the spin connection and of the Dirac operators of the deflating spacetimes (44d-..: 4+4 with deflating x5, x6, x7; 71d-..: 7+1 with deflating x5, x6, x7)", "deflation matrices")?;
    for c in &rep.checks {
        println!("[{}] {:<7} ({}) {}", if c.pass { "PASS" } else { "FAIL" }, c.id, c.kind, c.statement);
    }
    println!("\ndeflating spacetimes: {} of {} checks passed", ok, n);

    // ---- curvature table for a few sets of rates ---------------------------------------
    let sets: [[f64; 3]; 4] = [[1.0, 1.0, 1.0], [1.0, 2.0, 3.0], [0.5, 0.75, 5.0 / 3.0], [2.0, 2.0, 5.0]];
    let mut rows: Vec<Vec<f64>> = Vec::new();
    for (s, sig) in [Sig::S1644, Sig::S1671].iter().enumerate() {
        for rate in &sets {
            let p = Point { sig: *sig, rate: *rate, b: [1.0; 3] };
            let (ric, r, ein) = p.curvature_closed();
            rows.push(vec![if s == 0 { 1644.0 } else { 1671.0 }, rate[0], rate[1], rate[2], ric[4], ric[5], ric[6], ric[7], r, ein[0], ein[4], ein[5], ein[6], ein[7]]);
        }
    }
    write_csv("curvature.csv",
        &[("content", "curvature of the deflating spacetimes for several sets of rates, from the closed forms of check G5 (mixed components; they do not depend on x4)".to_string()),
          ("columns", "spacetime: 1644 = 4+4, 1671 = 7+1; g55, g66, g77: rates; Ric44, Ric55, Ric66, Ric77: mixed Ricci tensor R^mu_mu; R: curvature scalar; G00 (= G11 = G22 = G33), G44, G55, G66, G77: mixed Einstein tensor G^mu_mu".to_string()),
          ("command", "deflation matrices".to_string())],
        &["spacetime", "g55", "g66", "g77", "Ric44", "Ric55", "Ric66", "Ric77", "R", "G00", "G44", "G55", "G66", "G77"], &rows)?;
    Ok(all_flat && all)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("help");
    let result = match cmd {
        "matrices" => cmd_matrices(),
        _ => {
            println!("deflation: PROJECT spacetimes, Revision_Deflation (x5, x6, x7 deflate exponentially)\n");
            println!("commands (run from the folder Revision_Deflation):");
            println!("  matrices   checks of the flat Dirac matrices (again), of the geometry of the deflating spacetimes,");
            println!("             of the coordinate matrices, the spin connection and the Dirac operators");
            Ok(true)
        }
    };
    match result {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("deflation {}: at least one check FAILED", cmd);
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("deflation {}: {}", cmd, e);
            std::process::exit(2);
        }
    }
}
