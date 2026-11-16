//! Machine checks.  A check is a numbered statement that is either true (PASS) or
//! false (FAIL).  Every command of the program collects its checks in a `Checks`
//! list and writes them to a report file in the folder data/; the second line of
//! every report reads "N of M checks passed", the form that the script
//! code/scripts/check_all.sh reads.

use std::fs;
use std::io::Write;

/// A list of checks with a title.
pub struct Checks {
    title: String,
    items: Vec<(String, String, bool, String)>,
}

impl Checks {
    /// An empty list.
    pub fn new(title: &str) -> Checks {
        Checks { title: title.to_string(), items: Vec::new() }
    }

    /// Record a check: identifier, statement, result, details (numbers that show the result).
    pub fn add(&mut self, id: &str, statement: &str, ok: bool, detail: String) {
        let d = if detail.is_empty() { String::new() } else { format!("  -- {}", detail) };
        println!("[{}] {}  {}{}", if ok { "PASS" } else { "FAIL" }, id, statement, d);
        self.items.push((id.to_string(), statement.to_string(), ok, detail));
    }

    /// Did every check pass?
    pub fn all_pass(&self) -> bool {
        self.items.iter().all(|x| x.2)
    }

    /// Write the report data/<name>; return true if every check passed.
    pub fn write(&self, name: &str) -> bool {
        let n = self.items.iter().filter(|x| x.2).count();
        let mut s = format!("{}\n{} of {} checks passed\n\n", self.title, n, self.items.len());
        for (id, st, ok, d) in &self.items {
            let d = if d.is_empty() { String::new() } else { format!("  -- {}", d) };
            s += &format!("[{}] {}  {}{}\n", if *ok { "PASS" } else { "FAIL" }, id, st, d);
        }
        let path = spacetimes::io::data_dir().join(name);
        let mut f = fs::File::create(&path).expect("cannot write the report");
        f.write_all(s.as_bytes()).expect("cannot write the report");
        println!("wrote {} ({} of {} checks passed)", path.display(), n, self.items.len());
        self.all_pass()
    }
}
