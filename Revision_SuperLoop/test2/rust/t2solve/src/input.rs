//! The input file of the solvers of Test 2 (format "t2in 2"): the table of the keys, the guide, the seven examples, and
//! the reading and checking of a file.  The table and the examples are the same as in test2/python/t2_solve.py (the
//! self-test of this program compares its examples with the files test2/examples/*.t2in written by the Python solver).
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// the seven examples (the same text as in t2_solve.py; the self-test compares them with test2/examples/*.t2in)
pub const EXAMPLES: &[(&str, &str, &str)] = &[
    ("packet_free", "a free Gaussian packet of positive energy on a ring (group velocity)", r#"format = t2in 2
title = a free Gaussian wave packet on a ring
task = evolve
sites = 128 1 1
spacing = 0.5
boundary = periodic periodic periodic
mass = 0.4
mu = 0.0
initial = packet
packet_center = 16 0 0
packet_width = 3.0
packet_momentum = 0.8 0 0
packet_branch = positive
packet_spin = up
packet_isospin = up
packet_part = particle
output_step = 0.05
outputs = 600
fields_every = 20
"#),
    ("cyclotron", "a packet in a magnetic field on a 40 x 40 lattice with walls (cyclotron orbit)", r#"format = t2in 2
title = a wave packet in a constant magnetic field (cyclotron motion)
task = evolve
sites = 40 40 1
spacing = 0.5
boundary = wall wall periodic
mass = 0.4
mu = 0.0
magnetic = 0.4 0.0
initial = packet
packet_center = 9.75 13.3 0
packet_width = 1.5
packet_momentum = 0.6 0 0
packet_branch = positive
packet_spin = up
packet_isospin = up
output_step = 0.05
outputs = 720
fields_every = 80
"#),
    ("andreev", "a particle hitting a superfluid region: Andreev reflection into a hole", r#"format = t2in 2
title = Andreev reflection of a particle at a normal-superfluid interface
task = evolve
sites = 240 1 1
spacing = 0.5
boundary = wall periodic periodic
mass = 0.4
mu = 1.0
pair = step
pair_direction = 3
pair_amplitude = 0.6
pair_width = 1.0
pair_position = 60
initial = packet
packet_center = 30 0 0
packet_width = 4.0
packet_momentum = 0.917 0 0
packet_branch = positive
packet_spin = up
packet_isospin = up
packet_part = particle
output_step = 0.05
outputs = 1200
fields_every = 40
"#),
    ("kink_spectrum", "the spectrum of a ring with a kink and an antikink of the pair field (bound states)", r#"format = t2in 2
title = bound states of a kink and an antikink of the pair field
task = spectrum
sites = 60 1 1
spacing = 0.4
boundary = periodic periodic periodic
mass = 0.4
mu = 0.0
pair = kink_antikink
pair_direction = 3
pair_amplitude = 1.0
pair_width = 1.0
pair_position = 6 18
spectrum_window = -0.8 0.8
spectrum_max_states = 16
"#),
    ("ground_state", "the self-consistent uniform superfluid of a chain (compare with the lattice gap equation)", r#"format = t2in 2
title = the self-consistent uniform polar superfluid of a chain
task = ground_state
sites = 32 1 1
spacing = 0.5
boundary = periodic periodic periodic
mass = 0.4
mu = 0.6
coupling = 0.46
pair = uniform
pair_direction = 3
pair_amplitude = 0.2
mixing = 0.6
tolerance = 1e-11
max_iterations = 400
"#),
    ("kink_ground_state", "a self-consistent kink of the pair field between two walls (massless charges)", r#"format = t2in 2
title = a self-consistent kink of the pair field
task = ground_state
sites = 40 1 1
spacing = 0.5
boundary = wall periodic periodic
mass = 0.0
mu = 0.0
coupling = 0.48
pair = kink
pair_direction = 3
pair_amplitude = 0.6
pair_width = 1.7
pair_constraint = line
mixing = 0.5
tolerance = 1e-10
max_iterations = 400
"#),
    ("quench", "a quench of the coupling: the amplitude (Higgs) oscillation of the pair field", r#"format = t2in 2
title = a quench of the coupling of the uniform superfluid
task = quench
sites = 24 1 1
spacing = 0.5
boundary = periodic periodic periodic
mass = 0.0
mu = 0.0
coupling = 0.45
quench_coupling = 0.455
pair = uniform
pair_direction = 3
pair_amplitude = 0.5
mixing = 0.6
tolerance = 1e-12
max_iterations = 400
output_step = 0.04
outputs = 600
fields_every = 25
"#),
];

// the input keys: (name, kind, default, meaning); the same table as SPEC of t2_solve.py
pub const SPEC: &[(&str, &str, Option<&str>, &str)] = &[
    ("format", "str", None, "the format of the file; must be 't2in 2'"),
    ("title", "str", Some(""), "a free title of the problem"),
    ("task", "choice:evolve,spectrum,ground_state,quench", None, "what to solve (see the guide)"),
    ("output", "str", Some(""), "the base name of the output files, relative to the folder of the input file (default: the name of the input file without its extension)"),
    ("sites", "ints3", None, "the numbers of lattice sites n_x n_y n_z along x^1, x^2, x^3; 1 means that the fields do not depend on that coordinate"),
    ("spacing", "float+", None, "the lattice spacing dx (the same in all directions)"),
    ("boundary", "words3:periodic,wall", Some("periodic periodic periodic"), "the boundary condition of each direction: periodic (a ring) or wall (the field vanishes beyond the last site: Dirichlet)"),
    ("wilson", "float0+", Some("1.0"), "the coefficient r_W of the Wilson term (removes the doublers; 1 is standard)"),
    ("mass", "float", Some("0.4"), "the mass m of the spinor charges"),
    ("mu", "float", Some("0.0"), "the chemical potential mu"),
    ("g_v", "float", Some("0.7"), "the isospin coupling g_v"),
    ("g_F", "float", Some("0.45"), "the coupling g_F of the phase group"),
    ("potential_0", "floats4", Some("0 0 0 0"), "the constant potentials A^1_0 A^2_0 A^3_0 A^F_0 (time direction)"),
    ("potential_1", "floats4", Some("0 0 0 0"), "the constant potentials A^1_1 A^2_1 A^3_1 A^F_1 (direction x^1)"),
    ("potential_2", "floats4", Some("0 0 0 0"), "the constant potentials A^1_2 A^2_2 A^3_2 A^F_2 (direction x^2)"),
    ("potential_3", "floats4", Some("0 0 0 0"), "the constant potentials A^1_3 A^2_3 A^3_3 A^F_3 (direction x^3)"),
    ("magnetic", "floats2", Some("0 0"), "constant magnetic fields along x^3: F^F_12 (phase group) and F^3_12 (third isospin direction), in the Landau gauge A_2 = F (x^1 - centre); needs n_x > 1, n_y > 1 and a wall in direction 1"),
    ("gauge_file", "str", Some(""), "a file with potentials that depend on the site (added to the constant ones): one line per site, in the order of the sites, with 16 numbers A^1_p A^2_p A^3_p A^F_p for p = 0, 1, 2, 3"),
    ("pulse_direction", "int0123", Some("1"), "the index p (0, 1, 2 or 3) of the potential A_p that carries the pulse"),
    ("pulse_potential", "floats4", Some("0 0 0 0"), "the amplitudes A^1 A^2 A^3 A^F of a uniform pulse of the potential A_p (all zero: no pulse)"),
    ("pulse_time", "floats3", Some("5 1 0"), "the centre t_c, the duration t_w and the angular frequency w_p of the pulse: A_p(x^0) = amplitudes exp(-(x^0 - t_c)^2/(2 t_w^2)) cos(w_p (x^0 - t_c))"),
    ("pair", "choice:none,uniform,kink,kink_antikink,step,random,file", Some("none"), "the profile of the pair field d^iota: prescribed (evolve, spectrum) or the initial guess (ground_state, quench)"),
    ("pair_direction", "int123", Some("3"), "the isospin direction iota of the pair field (1, 2 or 3)"),
    ("pair_amplitude", "float", Some("0.0"), "the gap Delta (uniform, step) or Delta_inf (kinks)"),
    ("pair_phase", "float", Some("0.0"), "the phase of the pair field, in radians"),
    ("pair_width", "float+", Some("1.0"), "the width ell_K of a kink or of a step"),
    ("pair_position", "floatlist", Some(""), "the positions along x^1 of the walls (kink: one value, default the centre; kink_antikink: two values, default 1/4 and 3/4 of the length; step: one value, default the centre)"),
    ("pair_seed", "int", Some("1"), "the seed of the random pair field (pair = random)"),
    ("pair_file", "str", Some(""), "a file with the pair field (pair = file): one line per site, in the order of the sites, with d1_re d1_im d2_re d2_im d3_re d3_im"),
    ("output_step", "float+", None, "the interval of the time series: the state is written at the times x^0 = 0, output_step, 2 output_step, ..., outputs x output_step (evolve, quench); CVODE chooses its own internal steps between these times"),
    ("outputs", "int+", None, "the number of output intervals: the evolution ends at x^0 = outputs x output_step (evolve, quench)"),
    ("fields_every", "int+", Some("0"), "write the fields on the sites every this many outputs (0: about 50 times); the time series is written at every output"),
    ("rtol", "float+", Some("1e-12"), "the relative tolerance of CVODE: the error weight of a complex component w_cmp of the state is the inverse of (rtol times the modulus of w_cmp, plus atol) (evolve, quench)"),
    ("atol", "float+", Some("1e-14"), "the absolute tolerance of CVODE: components smaller than this count as zero in the error test (evolve, quench)"),
    ("initial", "choice:packet,eigenmode,file", Some("packet"), "the initial mode of the task evolve"),
    ("packet_center", "floats3", Some(""), "the centre of the Gaussian packet (x^1 x^2 x^3; default the centre of the lattice)"),
    ("packet_width", "float+", Some("2.0"), "the width of the packet (standard deviation of the density)"),
    ("packet_momentum", "floats3", Some("0 0 0"), "the momentum q of the packet"),
    ("packet_branch", "choice:positive,negative", Some("positive"), "the sign of the energy +-eps_lat(q) of the free lattice operator (without mu) of the packet"),
    ("packet_spin", "choice:up,down", Some("up"), "the spin along x^3 of the reference spinor"),
    ("packet_isospin", "choice:up,down", Some("up"), "the isospin component"),
    ("packet_part", "choice:particle,hole", Some("particle"), "particle part or hole part of the mode"),
    ("eigenmode_index", "int", Some("0"), "the number (ascending, from 0) of the eigenvector used as initial mode (initial = eigenmode)"),
    ("initial_file", "str", Some(""), "a file with the initial mode (initial = file): 16 x (number of sites) lines 're im' in the order of the components"),
    ("spectrum_window", "floats2", Some("-1 1"), "the energy window of the eigenvectors whose densities are written (spectrum)"),
    ("spectrum_max_states", "int+", Some("32"), "at most this many eigenvectors of the window are written (spectrum), summed over each group of degenerate eigenvalues"),
    ("coupling", "float+", None, "the coupling G of the self-consistency condition (ground_state, quench)"),
    ("pair_constraint", "choice:none,line", Some("none"), "line: the self-consistent iteration keeps the pair field a real multiple of exp(i pair_phase) along pair_direction (for walls of the pair field, which are unstable without it)"),
    ("mixing", "float01", Some("0.5"), "the damping factor beta of the Anderson iteration (ground_state, quench)"),
    ("tolerance", "float+", Some("1e-10"), "the iteration stops when the largest change of d^iota is below this"),
    ("max_iterations", "int+", Some("400"), "the largest number of iterations"),
    ("quench_coupling", "float+", None, "the coupling after x^0 = 0 (quench)"),
];

pub const TASK_KEYS: &[(&str, &[&str])] = &[
    ("evolve", &["outputs", "output_step"]),
    ("spectrum", &[]),
    ("ground_state", &["coupling"]),
    ("quench", &["coupling", "quench_coupling", "outputs", "output_step"]),
];

pub const FORMAT: &str = "t2in 2";

pub const GUIDE: &str = r#"
THE TEST 2 SOLVER (Rust) -- GUIDE TO THE INPUT FILE (format t2in 2)

What is solved.  The mean-field field equations of the self-interacting spinor field on the lattice:
    i dw/dx^0 = H_lat w,     H_lat = [[h_lat - V0 - mu, P], [P^dag, -conj(h_lat - V0 - mu)]],
for Nambu modes w (8 particle and 8 hole components per site).  h_lat is the covariant lattice Dirac
operator: the links U_j(s) = exp(-i dx (g_v A^iota_j(s) tau_iota/2 + g_F A^F_j(s))) on the bonds from the
site s to its neighbour s + j (the gauge-covariant central difference), the Wilson term with the coefficient
r_W, and the mass term m beta; V0 = g_v A^iota_0 tau_iota/2 + g_F A^F_0 is the potential of the time
direction, and P = sum_iota d^iota Kch_iota^dag the pair field.  With self-consistency (tasks ground_state
and quench) the pair field is
    d^iota(site) = -(G/2) Pi^iota(site)/dx^D,   Pi^iota = sum over the modes of w_p^T Kch_iota conj(w_h),
the modes being unit vectors and D the number of directions with more than one site.

The file.  One entry per line: 'key = value'.  '#' starts a comment.  Unknown keys are errors (so that a
misspelt key is not silently ignored).  Keys may appear in any order.

Boundary conditions.  'sites = n_x n_y n_z' fixes the lattice (sites at x^j = s^j dx, s^j = 0 .. n_j - 1);
a direction with one site is not used (the fields do not depend on that coordinate).  'boundary' gives,
for each direction, 'periodic' (the last site is the neighbour of the first: a ring or a torus; a constant
potential then acts through its holonomy) or 'wall' (the field vanishes beyond the last site: a hard
wall, the Dirichlet condition).  A magnetic field needs a wall in direction 1 (the Landau gauge).

Initial conditions.  For 'task = evolve' the initial mode is
    initial = packet     a Gaussian wave packet: centre, width (standard deviation of the density),
                         momentum q, energy branch (the sign of +-eps_lat(q)), spin and isospin along x^3, and
                         whether it is a particle or a hole; its spinor is the eigenvector of the free
                         lattice operator at q, so that the packet moves with the group velocity;
    initial = eigenmode  an eigenvector of H_lat (by its number in the ascending list; small lattices);
    initial = file       16 x (number of sites) lines 're im'.
For 'ground_state' and 'quench' the initial condition is the guess of the pair field ('pair = ...'), from
which the self-consistent iteration starts; the quench starts from the converged ground state.

Gauge fields.  Constant potentials ('potential_p = A^1_p A^2_p A^3_p A^F_p', p = 0 .. 3), potentials that
depend on the site ('gauge_file'), constant magnetic fields along x^3 ('magnetic = F^F_12 F^3_12', in the
Landau gauge), and a uniform pulse that depends on the time ('pulse_direction', 'pulse_potential',
'pulse_time'): A_p(x^0) = amplitudes exp(-(x^0 - t_c)^2/(2 t_w^2)) cos(w_p (x^0 - t_c)) is added to the
potential A_p (a pulse of A_1, A_2 or A_3 is a pulse of an electric field).  All contributions are added; the
links of a site use the potentials at that site.  The tasks spectrum and ground_state ignore the pulse.

The pair field.  'pair = none | uniform | kink | kink_antikink | step | random | file', with
'pair_direction', 'pair_amplitude', 'pair_phase', 'pair_width', 'pair_position'.  A kink is
Delta_inf tanh((x^1 - x_w)/ell_K); kink_antikink is Delta_inf tanh((x^1 - x_1)/ell_K) tanh((x_2 - x^1)/ell_K);
step is Delta (1 + tanh((x^1 - x_s)/ell_K))/2 (a superfluid on the right, a normal region on the left).

Time integration.  Both tasks that evolve in time (evolve, quench) integrate the equations with CVODE, the
variable-order, variable-step integrator of SUNDIALS 7.8.0 (in pure Rust, vendored in PROJECT spacetimes):
Adams-Moulton formulas of the orders 1 to 12, the corrector solved by fixed-point iteration, and a test of the
estimated local error of every step,
    sqrt( (1/(2 n_complex)) sum over w_cmp of |error of w_cmp|^2 / (rtol |w_cmp| + atol)^2 ) <= 1
(the sum over the n_complex complex components w_cmp of the state, the real and imaginary parts of every
component weighted alike).  CVODE chooses its
step sizes and orders itself; 'output_step' and 'outputs' only give the output times x^0 = 0, output_step,
2 output_step, ..., at which the state is written (by the interpolating polynomial of CVODE), and 'rtol'
(default 1e-12) and 'atol' (default 1e-14) the accuracy.  The quench integrates all modes at once, with the pair field computed from the modes at
every evaluation of the right side.  The exact equations conserve the norm of a mode (evolve), the fermion
number and the orthonormality of the modes (quench); CVODE keeps them up to the accumulated local error that its
error test permits, (number of steps) sqrt(2) (rtol ||w_lat|| + atol sqrt(n_complex)) for a state w_lat of the
norm ||w_lat||, and the program checks the change of the norm against 10 times this amount (the change of the
fermion number against 2 ||W|| times that, of the orthonormality against 2 times that; the true local error may
exceed the estimate of CVODE by a modest factor).  Hbound = (number of active directions)(1 + 2 r_W)/dx + |m| +
|mu| + max|V0| + max sum|d^iota|, an upper bound of the eigenvalue magnitudes of H_lat, is reported.
The format t2in 1 (with the keys time_step, steps and output_every of a fixed-step method) is not accepted.

Output.  'output = NAME' (default: the name of the input file), or '--out NAME' on the command line.  The
program writes NAME_summary.txt (the problem, the checks and the results), NAME_series.csv (quantities
against time or iteration), NAME_fields.csv (fields on the sites), and for some tasks NAME_spectrum.csv and
NAME_iterations.csv.  Plot them with 'python3 t2_plot.py NAME' (item [4]) or the Rust 't2_plot NAME' (item [6]).

The keys:
"#;

/// a checked value of the input file
#[derive(Clone, Debug)]
pub enum Val {
    S(String),
    F(f64),
    I(i64),
    V(Vec<f64>),
    W(Vec<String>),
    Missing,
}

/// the checked contents of an input file, with the defaults filled in
pub struct Cfg {
    pub vals: HashMap<String, Val>,
    pub dir: PathBuf,
    pub path: String,
}

impl Cfg {
    pub fn f(&self, k: &str) -> f64 {
        match &self.vals[k] {
            Val::F(x) => *x,
            Val::I(i) => *i as f64,
            v => panic!("key {} is not a number: {:?}", k, v),
        }
    }
    pub fn i(&self, k: &str) -> i64 {
        match &self.vals[k] {
            Val::I(i) => *i,
            v => panic!("key {} is not an integer: {:?}", k, v),
        }
    }
    pub fn s(&self, k: &str) -> String {
        match &self.vals[k] {
            Val::S(s) => s.clone(),
            v => panic!("key {} is not a text: {:?}", k, v),
        }
    }
    /// a list of numbers (empty if the key has no value)
    pub fn v(&self, k: &str) -> Vec<f64> {
        match &self.vals[k] {
            Val::V(v) => v.clone(),
            Val::Missing => vec![],
            v => panic!("key {} is not a list: {:?}", k, v),
        }
    }
    pub fn w(&self, k: &str) -> Vec<String> {
        match &self.vals[k] {
            Val::W(w) => w.clone(),
            v => panic!("key {} is not a list of words: {:?}", k, v),
        }
    }
    pub fn has(&self, k: &str) -> bool {
        !matches!(self.vals[k], Val::Missing)
    }
}

pub fn kind_of(key: &str) -> &'static str {
    SPEC.iter().find(|e| e.0 == key).map(|e| e.1).unwrap_or("str")
}

/// convert the text of a value; an error message names the key and what is expected
pub fn parse_value(key: &str, kind: &str, text: &str) -> Result<Val, String> {
    let t = text.trim();
    let num = |s: &str| -> Result<f64, String> {
        s.parse::<f64>().map_err(|_| format!("the value of '{}' ('{}') is not of the kind {}", key, t, kind))
    };
    let int = |s: &str| -> Result<i64, String> {
        s.parse::<i64>().map_err(|_| format!("the value of '{}' ('{}') is not of the kind {}", key, t, kind))
    };
    if kind == "str" {
        return Ok(Val::S(t.to_string()));
    }
    if let Some(opts) = kind.strip_prefix("choice:") {
        let o: Vec<&str> = opts.split(',').collect();
        if !o.contains(&t) {
            return Err(format!("'{}' must be one of {}, not '{}'", key, o.join(" | "), t));
        }
        return Ok(Val::S(t.to_string()));
    }
    if let Some(opts) = kind.strip_prefix("words3:") {
        let o: Vec<&str> = opts.split(',').collect();
        let w: Vec<&str> = t.split_whitespace().collect();
        if w.len() != 3 || w.iter().any(|x| !o.contains(x)) {
            return Err(format!("'{}' needs three words, each one of {}", key, o.join(" | ")));
        }
        return Ok(Val::W(w.iter().map(|x| x.to_string()).collect()));
    }
    match kind {
        "float" => Ok(Val::F(num(t)?)),
        "float+" => {
            let v = num(t)?;
            if !(v > 0.0) {
                return Err(format!("'{}' must be positive", key));
            }
            Ok(Val::F(v))
        }
        "float0+" => {
            let v = num(t)?;
            if v < 0.0 {
                return Err(format!("'{}' must not be negative", key));
            }
            Ok(Val::F(v))
        }
        "float01" => {
            let v = num(t)?;
            if !(v > 0.0 && v <= 1.0) {
                return Err(format!("'{}' must lie in (0, 1]", key));
            }
            Ok(Val::F(v))
        }
        "int" => Ok(Val::I(int(t)?)),
        "int+" => {
            let v = int(t)?;
            if v < 0 || (v == 0 && key != "fields_every") {
                return Err(format!("'{}' must be a positive integer", key));
            }
            Ok(Val::I(v))
        }
        "int0123" => {
            let v = int(t)?;
            if !(0..=3).contains(&v) {
                return Err(format!("'{}' must be 0, 1, 2 or 3", key));
            }
            Ok(Val::I(v))
        }
        "int123" => {
            let v = int(t)?;
            if !(1..=3).contains(&v) {
                return Err(format!("'{}' must be 1, 2 or 3", key));
            }
            Ok(Val::I(v))
        }
        "ints3" => {
            let v: Result<Vec<i64>, String> = t.split_whitespace().map(int).collect();
            let v = v?;
            if v.len() != 3 || v.iter().any(|&x| x < 1) {
                return Err(format!("'{}' needs three integers >= 1", key));
            }
            Ok(Val::V(v.iter().map(|&x| x as f64).collect()))
        }
        "floats2" | "floats3" | "floats4" => {
            if t.is_empty() {
                return Ok(Val::Missing);
            }
            let n: usize = kind[6..].parse().unwrap();
            let v: Result<Vec<f64>, String> = t.split_whitespace().map(num).collect();
            let v = v?;
            if v.len() != n {
                return Err(format!("'{}' needs {} numbers", key, n));
            }
            Ok(Val::V(v))
        }
        "floatlist" => {
            let v: Result<Vec<f64>, String> = t.split_whitespace().map(num).collect();
            Ok(Val::V(v?))
        }
        _ => Err(format!("internal: unknown kind {}", kind)),
    }
}

/// all keys with their defaults (Missing where the key is required)
pub fn defaults() -> HashMap<String, Val> {
    let mut m = HashMap::new();
    for (k, kind, default, _) in SPEC {
        let v = match default {
            Some(d) => parse_value(k, kind, d).expect("default"),
            None => Val::Missing,
        };
        m.insert(k.to_string(), v);
    }
    m
}

/// read an input file into a checked configuration
pub fn read_input(path: &str) -> Result<Cfg, String> {
    let p = Path::new(path);
    if !p.is_file() {
        return Err(format!("there is no file '{}'", path));
    }
    let text = fs::read_to_string(p).map_err(|e| format!("cannot read '{}': {}", path, e))?;
    // the old format of a fixed-step method is refused with an explanation
    if text.lines().any(|l| {
        let s = l.split('#').next().unwrap_or("").trim();
        s.split_once('=').map(|(k, v)| k.trim() == "format" && v.trim() == "t2in 1").unwrap_or(false)
    }) {
        return Err("the format 't2in 1' (with the keys time_step, steps and output_every of a fixed-step method) is no longer accepted: the time integration is done by CVODE; write 'format = t2in 2' and replace time_step by output_step, steps by outputs and output_every by fields_every (optional: rtol, atol)".into());
    }
    let mut raw: HashMap<String, String> = HashMap::new();
    for (no, line) in text.lines().enumerate() {
        let s = line.split('#').next().unwrap_or("").trim();
        if s.is_empty() {
            continue;
        }
        let Some((k, v)) = s.split_once('=') else {
            return Err(format!("line {}: expected 'key = value', found '{}'", no + 1, s));
        };
        let k = k.trim();
        if !SPEC.iter().any(|e| e.0 == k) {
            let pre: String = k.chars().take(4).collect();
            let close: Vec<&str> = SPEC.iter().map(|e| e.0).filter(|x| x.starts_with(&pre)).collect();
            let hint = if close.is_empty() { String::new() } else { format!(" (did you mean {}?)", close.join(", ")) };
            return Err(format!("line {}: unknown key '{}'{}", no + 1, k, hint));
        }
        if raw.contains_key(k) {
            return Err(format!("line {}: the key '{}' appears twice", no + 1, k));
        }
        raw.insert(k.to_string(), v.to_string());
    }
    let mut vals = defaults();
    for (k, v) in &raw {
        vals.insert(k.clone(), parse_value(k, kind_of(k), v)?);
    }
    let mut cfg = Cfg { vals, dir: p.parent().map(|d| d.to_path_buf()).unwrap_or_else(|| PathBuf::from(".")), path: path.to_string() };
    if cfg.s("format") != FORMAT {
        return Err(format!("the first entry must be 'format = {}'", FORMAT));
    }
    for k in ["task", "sites", "spacing"] {
        if !cfg.has(k) {
            return Err(format!("the key '{}' is required", k));
        }
    }
    let task = cfg.s("task");
    for (t, keys) in TASK_KEYS {
        if *t == task {
            for k in keys.iter() {
                if !cfg.has(k) {
                    return Err(format!("the task '{}' needs the key '{}'", task, k));
                }
            }
        }
    }
    if cfg.s("output").is_empty() {
        let stem = p.file_stem().map(|x| x.to_string_lossy().to_string()).unwrap_or_else(|| "out".into());
        cfg.vals.insert("output".into(), Val::S(stem));
    }
    if cfg.dir.as_os_str().is_empty() {
        cfg.dir = PathBuf::from(".");
    }
    Ok(cfg)
}

/// the complete guide: the text and every key with its meaning, kind and default
pub fn guide_text() -> String {
    let mut s = String::from(GUIDE);
    for (k, kind, default, expl) in SPEC {
        let d = match default {
            None => "required".to_string(),
            Some("") => "default: (see text)".to_string(),
            Some(d) => format!("default: {}", d),
        };
        s += &format!("  {:<20} {}  [{}; {}]\n", k, expl, kind, d);
    }
    s += "\nTasks and the keys they need: ";
    let parts: Vec<String> = TASK_KEYS
        .iter()
        .map(|(t, v)| format!("{}: {}", t, if v.is_empty() { "-".to_string() } else { v.join(", ") }))
        .collect();
    s += &parts.join("; ");
    s += "\nExamples: t2_solve --examples\n";
    s
}

/// the text of an example input file, as t2_solve.py writes it
pub fn example_text(name: &str) -> Option<String> {
    EXAMPLES.iter().find(|e| e.0 == name).map(|e| {
        format!("# example '{}' of the Test 2 solver: {}\n# every key is explained by: python3 t2_solve.py --guide, or by the Rust solver: t2_solve --guide\n{}", e.0, e.1, e.2)
    })
}
