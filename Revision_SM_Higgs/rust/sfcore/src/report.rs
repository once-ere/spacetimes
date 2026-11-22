//! The record of the machine checks and the writers of the data files.
//!
//! A check has an identifier (for example "A-3"), a statement, a measured deviation and a tolerance; it passes when the
//! deviation is at most the tolerance (or, for a check of a statement that is true or false, when the statement holds).
//! A report is written as a text file with one line per check and a final verdict line.
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct Check {
    pub id: String,
    pub text: String,
    pub value: f64,
    pub tol: f64,
    pub pass: bool,
    pub detail: String,
}

pub struct Report {
    pub title: String,
    pub checks: Vec<Check>,
    pub notes: Vec<String>,
}

impl Report {
    pub fn new(title: &str) -> Report {
        Report { title: title.to_string(), checks: vec![], notes: vec![] }
    }
    /// a check of a deviation: passes when value <= tol (a NaN never passes)
    pub fn add(&mut self, id: &str, text: &str, value: f64, tol: f64) -> bool {
        let pass = value <= tol;
        println!("{} {:<7} dev {:.3e} (tol {:.1e})  {}", if pass { "PASS" } else { "FAIL" }, id, value, tol, text);
        self.checks.push(Check { id: id.into(), text: text.into(), value, tol, pass, detail: String::new() });
        pass
    }
    /// a check of a statement that is true or false, with a detail (for example the counts that decide it)
    pub fn add_bool(&mut self, id: &str, text: &str, ok: bool, detail: &str) -> bool {
        println!("{} {:<7} {}  [{}]", if ok { "PASS" } else { "FAIL" }, id, text, detail);
        self.checks.push(Check {
            id: id.into(),
            text: text.into(),
            value: if ok { 0.0 } else { 1.0 },
            tol: 0.0,
            pass: ok,
            detail: detail.into(),
        });
        ok
    }
    pub fn note(&mut self, s: &str) {
        println!("NOTE {}", s);
        self.notes.push(s.to_string());
    }
    pub fn counts(&self) -> (usize, usize) {
        (self.checks.iter().filter(|c| c.pass).count(), self.checks.len())
    }
    /// writes the report; the last line is "VERDICT: ALL PASS (n of n)" or "VERDICT: FAIL (k of n passed)"
    pub fn write(&self, path: &Path) {
        let mut s = String::new();
        s.push_str(&format!("{}\n", self.title));
        s.push_str("Each line: PASS or FAIL, the identifier of the check, the measured deviation and its tolerance (or, for a\n");
        s.push_str("statement that is true or false, the facts that decide it), and the statement that is checked.\n\n");
        for c in &self.checks {
            if c.detail.is_empty() {
                s.push_str(&format!(
                    "{} {:<7} deviation {:.3e} (tolerance {:.1e}): {}\n",
                    if c.pass { "PASS" } else { "FAIL" },
                    c.id,
                    c.value,
                    c.tol,
                    c.text
                ));
            } else {
                s.push_str(&format!("{} {:<7} {} [{}]\n", if c.pass { "PASS" } else { "FAIL" }, c.id, c.text, c.detail));
            }
        }
        if !self.notes.is_empty() {
            s.push_str("\nNotes:\n");
            for n in &self.notes {
                s.push_str(&format!("  {}\n", n));
            }
        }
        let (np, n) = self.counts();
        if np == n {
            s.push_str(&format!("\nVERDICT: ALL PASS ({} of {})\n", np, n));
        } else {
            s.push_str(&format!("\nVERDICT: FAIL ({} of {} passed)\n", np, n));
        }
        write_text(path, &s);
    }
}

/// writes a text file, creating its folder
pub fn write_text(path: &Path, s: &str) {
    if let Some(d) = path.parent() {
        fs::create_dir_all(d).expect("cannot create the data folder");
    }
    let mut f = fs::File::create(path).unwrap_or_else(|e| panic!("cannot write {}: {}", path.display(), e));
    f.write_all(s.as_bytes()).unwrap();
}

/// one field of a CSV line: quoted (with inner quotes doubled) when it contains a comma or a quote (RFC 4180)
fn csv_field(x: &str) -> String {
    if x.contains(',') || x.contains('"') {
        format!("\"{}\"", x.replace('"', "\"\""))
    } else {
        x.to_string()
    }
}

/// writes a CSV file: a header line and one line per row (fields with commas are quoted)
pub fn write_csv(path: &Path, header: &[&str], rows: &[Vec<String>]) {
    let mut s = header.iter().map(|h| csv_field(h)).collect::<Vec<_>>().join(",");
    s.push('\n');
    for r in rows {
        s.push_str(&r.iter().map(|x| csv_field(x)).collect::<Vec<_>>().join(","));
        s.push('\n');
    }
    write_text(path, &s);
}

/// the folder data/ of Revision_SM_Higgs, found from the folder of the calling crate
/// (rust/<crate> -> ../../data) unless the environment variable SF_DATA names another folder
pub fn data_dir(manifest_dir: &str) -> PathBuf {
    if let Ok(d) = std::env::var("SF_DATA") {
        return PathBuf::from(d);
    }
    Path::new(manifest_dir).join("..").join("..").join("data")
}

/// a number in the fixed notation used by the data files (17 significant digits, so that it is read back exactly)
pub fn f(x: f64) -> String {
    format!("{:.17e}", x)
}
