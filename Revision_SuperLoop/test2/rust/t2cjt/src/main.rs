//! t2_cjt -- PROJECT spacetimes, Revision_SuperLoop, Test 2, stage D.
//!
//! The creation of the superfluid boson singlet state (SFBS, spin 0) from the spinor field, analyzed by the method of
//! J. M. Cornwall, R. Jackiw and E. Tomboulis (Phys. Rev. D 10, 2428 (1974); "CJT"): the effective action of the
//! propagator of the spinor field, in the two-loop approximation.
//!
//! THE MODEL.  The field Psi_LL has 8 components (Dirac index times isospin index; component number 2 (a - 1) + s for
//! the Dirac index a = 1..4 and the isospin index s = 1, 2).  Its Hamiltonian density is
//!     Psi_LL^dag (h(q) - mu_bare) Psi_LL - (G/4) sum_iota Pi_iota^dag Pi_iota ,
//! with h(q) = sum_j alpha^j q^j + beta m_bare (8 x 8, the unit matrix on the isospin index), the momentum cutoff
//! |q| < Lambda, and the pair operators Pi_iota = Psi_LL^T Kch_iota Psi_LL of the three scalar pair matrices
//! Kch_iota = (C gamma_ch) (x) (eps2 tau_iota), which are antisymmetric and unitary.  m_bare and mu_bare are the
//! parameters that stand in the Hamiltonian when the interaction is written in the order Pi^dag Pi ("bare" mass and
//! "bare" chemical potential).
//!
//! THE TWO-LOOP APPROXIMATION.  The CJT effective action is a functional of the propagator.  With its two-loop term
//! (one interaction vertex, all three Wick contractions) it equals, on a state that does not depend on time,
//! minus (duration times volume) times the energy per volume
//!     E_var[C_n, C_p] = int tr(h(q) C_n(q)) - mu_bare tr C_n - (G/4) sum_iota |<Pi_iota>|^2
//!                       - (G/2) sum_iota tr(Kch_iota C_n Kch_iota^dag C_n^T)
//! of the quasi-free state with the correlation matrices C_n[a][b] = <Psi_b^dag Psi_a> and C_p[a][b] = <Psi_a Psi_b>
//! (int = integral over |q| < Lambda of d^3q/(2 pi)^3; C_n, C_p without argument: the matrices integrated over q;
//! <Pi_iota> = sum_ab Kch_iota[a][b] C_p[a][b]).  The stationary states are the ground states of the mean-field
//! (Bogoliubov-de Gennes) matrix of 16 rows
//!     H_var(q) = [[h(q) + N_sc - mu_bare, Ppair], [Ppair^dag, -(h(-q) + N_sc - mu_bare)^T]] ,
//!     N_sc = -G sum_iota Kch_iota^dag C_n^T Kch_iota ,    Ppair = sum_iota d_iota Kch_iota^dag ,   d_iota = -(G/2) <Pi_iota> ,
//! on the Nambu spinor (Psi(q), Psi^dag(-q)^T): C_n(q) and -C_p(q) are the upper left and the upper right block of the
//! projector on the eigenvectors of H_var(q) with negative eigenvalues.
//!
//! WHAT THE PROGRAM DOES.  It checks every statement of the document of stage D by machine (checks CJ-1 to CJ-10,
//! listed in main below), writes the numbers and tables of that document, and the data of its figures.
//! The time integration of check CJ-8 is done by CVODE (SUNDIALS 7.8.0 in pure Rust, Adams-Moulton formulas).
//!
//! OUTPUT (folder test2/data):
//!     t2_cjt_checks.txt       the report of the checks
//!     t2_cjt_fragments.md     the numbers and tables for the document
//!     t2_cjt_channels.csv     the coefficients of the 64 normal channels
//!     t2_cjt_vacuum.csv       the vacuum against the coupling, in the three readings
//!     t2_cjt_boson.csv        the mass of the SFBS boson of the normal vacuum against the coupling
//!     t2_cjt_surface.csv      the energy of the trial states of the vacuum sector
//!     t2_cjt_time.csv         the pair amplitude against the time (integrations with CVODE)
//!     t2_cjt_site.csv         one site: exact ground energy and two-loop energy
//!
//! USE (from the folder test2/rust):   cargo build --release && ./target/release/t2_cjt
//! Exit status 0 if every check passes.

#[allow(dead_code)]
#[path = "../../t2sfbs/src/fock.rs"]
mod fock;
#[allow(dead_code)]
#[path = "../../t2sfbs/src/quad.rs"]
mod quad;

use std::fs;
use std::io::Write;
use std::path::PathBuf;

use t2core::c64::{c, C64};
use t2core::cmat::{eigh, CMat};
use t2core::cvode::{self, ComplexRhs, CvodeSettings};
use t2core::model::{alpha, beta, gamma_ch, kpair, kpair_ps, lift4, sigma};
use t2core::rng::Rng;

const PI: f64 = std::f64::consts::PI;
/// Lambda, the cutoff of the momenta (the unit of momentum and energy)
const LAMBDA: f64 = 1.0;
/// the mass of the spinor charges of the document of Test 2
const M_T2: f64 = 0.2;
/// the coupling of the condensed vacuum of the document of Test 2
const G_T2: f64 = 22.5;
/// a coupling between the binding coupling and the critical coupling (normal vacuum with a bound SFBS boson)
const G_OSC: f64 = 20.0;

// ------------------------------------------------------------------ report, fragments, data files

/// the list of checks: identifier, statement, result, numbers found
struct Checks {
    title: String,
    items: Vec<(String, String, bool, String)>,
}

impl Checks {
    fn add(&mut self, id: &str, statement: &str, ok: bool, detail: String) {
        println!("[{}] {}  {}", if ok { "PASS" } else { "FAIL" }, id, detail);
        self.items.push((id.to_string(), statement.to_string(), ok, detail));
    }
    fn text(&self) -> String {
        let npass = self.items.iter().filter(|x| x.2).count();
        let mut s = format!("{}\n{} of {} checks passed\n\n", self.title, npass, self.items.len());
        for (id, st, ok, det) in &self.items {
            s.push_str(&format!(
                "[{}] {}  {}{}\n",
                if *ok { "PASS" } else { "FAIL" },
                id,
                st,
                if det.is_empty() { String::new() } else { format!("  -- {}", det) }
            ));
        }
        s
    }
    fn all_pass(&self) -> bool {
        self.items.iter().all(|x| x.2)
    }
}

/// the numbers and tables for the document: <!-- BEGIN name --> text <!-- END name -->
struct Frags(Vec<(String, String)>);

impl Frags {
    fn add(&mut self, name: &str, text: String) {
        self.0.push((name.to_string(), text));
    }
    fn write(&self, path: &PathBuf) {
        let mut f = fs::File::create(path).expect("fragments");
        writeln!(f, "<!-- generated by t2_cjt (test2/rust/t2cjt); do not edit -->").unwrap();
        for (n, t) in &self.0 {
            writeln!(f, "<!-- BEGIN {} -->\n{}\n<!-- END {} -->", n, t.trim_end(), n).unwrap();
        }
    }
}

/// a data file: comment lines "# key: value" (the first one is "# content: ..."), a header line, rows
struct Csv {
    text: String,
}

impl Csv {
    fn new(content: &str, meta: &[(&str, String)], header: &str) -> Csv {
        let mut text = format!("# content: {}\n", content);
        for (k, v) in meta {
            text.push_str(&format!("# {}: {}\n", k, v));
        }
        text.push_str(header);
        text.push('\n');
        Csv { text }
    }
    fn row(&mut self, fields: &[String]) {
        self.text.push_str(&fields.join(","));
        self.text.push('\n');
    }
    fn write(&self, dir: &PathBuf, name: &str) {
        fs::write(dir.join(name), &self.text).expect("write data file");
    }
}

/// the folder test2/data (two levels above the folder of this crate)
fn data_dir() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.pop();
    p.push("data");
    fs::create_dir_all(&p).unwrap();
    p
}

fn sci(x: f64) -> String {
    format!("{:.1e}", x)
}

fn f10(x: f64) -> String {
    format!("{:.10}", x)
}

fn f7(x: f64) -> String {
    format!("{:.7}", x)
}

// ------------------------------------------------------------------ small helpers of linear algebra

/// tr(a b) without forming the product
fn tr_prod(a: &CMat, b: &CMat) -> C64 {
    let mut s = C64::ZERO;
    for i in 0..a.rows {
        for j in 0..a.cols {
            s += a.get(i, j) * b.get(j, i);
        }
    }
    s
}

/// a pseudo-random Hermitian n x n matrix with entries of the size `scale`
fn rand_herm(rng: &mut Rng, n: usize, scale: f64) -> CMat {
    let mut m = CMat::zeros(n);
    for i in 0..n {
        for j in 0..n {
            m.set(i, j, c(rng.unit() * scale, rng.unit() * scale));
        }
    }
    m.add(&m.dagger()).scale_r(0.5)
}

/// a pseudo-random antisymmetric complex n x n matrix with entries of the size `scale`
fn rand_antisym(rng: &mut Rng, n: usize, scale: f64) -> CMat {
    let mut m = CMat::zeros(n);
    for i in 0..n {
        for j in 0..n {
            m.set(i, j, c(rng.unit() * scale, rng.unit() * scale));
        }
    }
    m.sub(&m.transpose()).scale_r(0.5)
}

/// the solution of a small real linear system by Gaussian elimination with partial pivoting (None if singular)
fn solve_small(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for col in 0..n {
        let mut piv = col;
        for r in (col + 1)..n {
            if a[r][col].abs() > a[piv][col].abs() {
                piv = r;
            }
        }
        if a[piv][col].abs() < 1e-300 {
            return None;
        }
        a.swap(col, piv);
        b.swap(col, piv);
        for r in (col + 1)..n {
            let f = a[r][col] / a[col][col];
            for k in col..n {
                a[r][k] -= f * a[col][k];
            }
            b[r] -= f * b[col];
        }
    }
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut s = b[i];
        for k in (i + 1)..n {
            s -= a[i][k] * x[k];
        }
        x[i] = s / a[i][i];
    }
    Some(x)
}

/// Anderson acceleration of the fixed-point iteration x -> F(x): the next iterate is the damped step from the
/// combination of the last `depth` iterates whose residuals F(x) - x have the smallest combined norm (least squares,
/// solved through the normal equations).  Returns (x, largest component of the last residual, iterations).
fn anderson(f: &mut dyn FnMut(&[f64]) -> Vec<f64>, x0: Vec<f64>, depth: usize, damp: f64, tol: f64, maxit: usize) -> (Vec<f64>, f64, usize) {
    let n = x0.len();
    let mut xs: Vec<Vec<f64>> = Vec::new();
    let mut rs: Vec<Vec<f64>> = Vec::new();
    let mut x = x0;
    let mut res = f64::INFINITY;
    let mut it = 0;
    while it < maxit {
        let fx = f(&x);
        let r: Vec<f64> = (0..n).map(|i| fx[i] - x[i]).collect();
        res = r.iter().fold(0.0f64, |a, b| a.max(b.abs()));
        if res < tol {
            break;
        }
        let mut xn: Vec<f64> = (0..n).map(|i| x[i] + damp * r[i]).collect();
        let mk = xs.len().min(depth);
        if mk > 0 {
            let off = xs.len() - mk;
            let dr: Vec<Vec<f64>> = (0..mk).map(|j| (0..n).map(|i| r[i] - rs[off + j][i]).collect()).collect();
            let mut a = vec![vec![0.0; mk]; mk];
            let mut b = vec![0.0; mk];
            for i in 0..mk {
                for j in 0..mk {
                    a[i][j] = (0..n).map(|k| dr[i][k] * dr[j][k]).sum();
                }
                b[i] = (0..n).map(|k| dr[i][k] * r[k]).sum();
            }
            let scale = (0..mk).map(|i| a[i][i]).fold(0.0, f64::max);
            for (i, row) in a.iter_mut().enumerate() {
                row[i] += 1e-12 * scale;
            }
            if let Some(gam) = solve_small(a, b) {
                for j in 0..mk {
                    for i in 0..n {
                        xn[i] -= gam[j] * ((x[i] - xs[off + j][i]) + damp * dr[j][i]);
                    }
                }
            }
        }
        xs.push(x);
        rs.push(r);
        if xs.len() > depth {
            xs.remove(0);
            rs.remove(0);
        }
        x = xn;
        it += 1;
    }
    (x, res, it)
}

// ------------------------------------------------------------------ the model

/// the pair matrices of the interaction and the Dirac matrices lifted to 8 x 8
#[derive(Clone)]
struct Model {
    /// G, the coupling
    g: f64,
    /// the pair matrices of the interaction: the three scalar ones Kch_iota; in the axially symmetric completion
    /// (check CJ-10) the three pseudoscalar ones Kch'_iota follow them
    ks: Vec<CMat>,
    /// their Hermitian conjugates
    kds: Vec<CMat>,
    beta8: CMat,
    alpha8: Vec<CMat>,
}

impl Model {
    fn new(g: f64, axial: bool) -> Model {
        let mut ks: Vec<CMat> = (1..4).map(kpair).collect();
        if axial {
            ks.extend((1..4).map(kpair_ps));
        }
        let kds = ks.iter().map(|k| k.dagger()).collect();
        Model { g, ks, kds, beta8: lift4(&beta()), alpha8: (1..4).map(|j| lift4(&alpha(j))).collect() }
    }
    /// h(q) = sum_j alpha^j q^j + beta m (8 x 8)
    fn h0(&self, q: &[f64; 3], m: f64) -> CMat {
        let mut h = self.beta8.scale_r(m);
        for j in 0..3 {
            if q[j] != 0.0 {
                h = h.add(&self.alpha8[j].scale_r(q[j]));
            }
        }
        h
    }
    /// the pair expectation values <Pi> = sum_ab K[a][b] C_p[a][b] of all pair matrices of the interaction
    fn pairs(&self, cp: &CMat) -> Vec<C64> {
        self.ks
            .iter()
            .map(|k| {
                let mut s = C64::ZERO;
                for a in 0..8 {
                    for b in 0..8 {
                        s += k.get(a, b) * cp.get(a, b);
                    }
                }
                s
            })
            .collect()
    }
    /// the mean fields of a state: N_sc = -G sum K^dag C_n^T K and Ppair = sum d K^dag with d = -(G/2) <Pi>
    fn fields(&self, cn: &CMat, cp: &CMat) -> (CMat, CMat) {
        let cnt = cn.transpose();
        let mut nsc = CMat::zeros(8);
        for (k, kd) in self.ks.iter().zip(&self.kds) {
            nsc = nsc.add(&kd.mul(&cnt).mul(k));
        }
        let nsc = nsc.scale_r(-self.g);
        let mut pp = CMat::zeros(8);
        for (pi, kd) in self.pairs(cp).iter().zip(&self.kds) {
            pp = pp.add(&kd.scale(pi.scale(-0.5 * self.g)));
        }
        (nsc, pp)
    }
    /// the pair term of the energy, -(G/4) sum |<Pi>|^2
    fn e_pair(&self, cp: &CMat) -> f64 {
        -0.25 * self.g * self.pairs(cp).iter().map(|z| z.norm_sqr()).sum::<f64>()
    }
    /// the normal term of the energy, -(G/2) sum tr(K C_n K^dag C_n^T)
    fn e_normal(&self, cn: &CMat) -> f64 {
        let cnt = cn.transpose();
        let mut s = 0.0;
        for (k, kd) in self.ks.iter().zip(&self.kds) {
            s += tr_prod(&k.mul(cn).mul(kd), &cnt).re;
        }
        -0.5 * self.g * s
    }
}

/// the points and weights of the momentum integral int_{|q| < Lambda} d^3q/(2 pi)^3
struct Grid {
    q: Vec<[f64; 3]>,
    w: Vec<f64>,
}

impl Grid {
    /// product rule: Gauss-Legendre in the length q (nr points) and in cos(theta) (nth points), equal weights in
    /// the angle phi (nph points)
    fn product(nr: usize, nth: usize, nph: usize) -> Grid {
        let (xr, wr) = quad::gauss_legendre(nr);
        let (xt, wt) = quad::gauss_legendre(nth);
        let mut q = Vec::new();
        let mut w = Vec::new();
        for i in 0..nr {
            let r = 0.5 * LAMBDA * (xr[i] + 1.0);
            let wri = 0.5 * LAMBDA * wr[i];
            for j in 0..nth {
                let ct = xt[j];
                let st = (1.0 - ct * ct).sqrt();
                for k in 0..nph {
                    let ph = (k as f64 + 0.5) * 2.0 * PI / nph as f64;
                    q.push([r * st * ph.cos(), r * st * ph.sin(), r * ct]);
                    w.push(wri * wt[j] * (2.0 * PI / nph as f64) * r * r / (2.0 * PI).powi(3));
                }
            }
        }
        Grid { q, w }
    }
    /// the two directions +z and -z with half of the solid angle each: exact for the states that are covariant
    /// under rotations, whose matrices C_n(q) and C_p(q) are polynomials of the first degree in the direction of q
    fn axis(nr: usize) -> Grid {
        let (xr, wr) = quad::gauss_legendre(nr);
        let mut q = Vec::new();
        let mut w = Vec::new();
        for i in 0..nr {
            let r = 0.5 * LAMBDA * (xr[i] + 1.0);
            let wri = 0.5 * LAMBDA * wr[i];
            for sgn in [1.0, -1.0] {
                q.push([0.0, 0.0, sgn * r]);
                w.push(wri * 2.0 * PI * r * r / (2.0 * PI).powi(3));
            }
        }
        Grid { q, w }
    }
    /// one site: the single momentum q = 0 with the weight 1 (8 fermion modes)
    fn site() -> Grid {
        Grid { q: vec![[0.0; 3]], w: vec![1.0] }
    }
    /// the number of modes per volume and component, int d^3q/(2 pi)^3 = Lambda^3/(6 pi^2)
    fn nlam(&self) -> f64 {
        self.w.iter().sum()
    }
}

/// a quasi-free state: the correlation matrices integrated over the momentum and the kinetic energy int tr(h C_n(q))
struct State {
    cn: CMat,
    cp: CMat,
    ekin: f64,
}

/// the mean-field matrix H_var(q) of 16 rows
fn bdg(md: &Model, q: &[f64; 3], m: f64, mu: f64, nsc: &CMat, pp: &CMat) -> CMat {
    let shift = CMat::identity(8).scale_r(mu);
    let h = md.h0(q, m).add(nsc).sub(&shift);
    let hm = md.h0(&[-q[0], -q[1], -q[2]], m).add(nsc).sub(&shift);
    CMat::blocks(&h, pp, &pp.dagger(), &hm.transpose().scale_r(-1.0))
}

/// the ground state of the mean-field matrices with the mean fields (N_sc, Ppair): every eigenvector with a negative
/// eigenvalue is occupied
fn ground(md: &Model, gr: &Grid, m: f64, mu: f64, nsc: &CMat, pp: &CMat) -> State {
    let mut an = [[C64::ZERO; 8]; 8];
    let mut ap = [[C64::ZERO; 8]; 8];
    let mut ekin = 0.0;
    for (q, &w) in gr.q.iter().zip(&gr.w) {
        let (e, v) = eigh(&bdg(md, q, m, mu, nsc, pp));
        let h0 = md.h0(q, m);
        let mut cq = [[C64::ZERO; 8]; 8];
        for k in 0..16 {
            if e[k] >= 0.0 {
                continue;
            }
            for a in 0..8 {
                let va = v.get(a, k);
                for b in 0..8 {
                    cq[a][b] += va * v.get(b, k).conj();
                    ap[a][b] -= (va * v.get(8 + b, k).conj()).scale(w);
                }
            }
        }
        for a in 0..8 {
            for b in 0..8 {
                an[a][b] += cq[a][b].scale(w);
                ekin += w * (h0.get(a, b) * cq[b][a]).re;
            }
        }
    }
    let mut cn = CMat::zeros(8);
    let mut cp = CMat::zeros(8);
    for a in 0..8 {
        for b in 0..8 {
            cn.set(a, b, an[a][b]);
            cp.set(a, b, ap[a][b]);
        }
    }
    State { cn, cp, ekin }
}

/// the energy per volume E_var of a state
fn energy(md: &Model, st: &State, mu: f64) -> f64 {
    st.ekin - mu * st.cn.trace().re + md.e_pair(&st.cp) + md.e_normal(&st.cn)
}

fn pack(nsc: &CMat, pp: &CMat) -> Vec<f64> {
    let mut x = Vec::with_capacity(256);
    for m in [nsc, pp] {
        for z in &m.a {
            x.push(z.re);
            x.push(z.im);
        }
    }
    x
}

fn unpack(x: &[f64]) -> (CMat, CMat) {
    let mut nsc = CMat::zeros(8);
    let mut pp = CMat::zeros(8);
    for k in 0..64 {
        nsc.a[k] = c(x[2 * k], x[2 * k + 1]);
        pp.a[k] = c(x[128 + 2 * k], x[128 + 2 * k + 1]);
    }
    (nsc, pp)
}

/// a self-consistent state: its mean fields, the state, its energy, the residual of the self-consistency map
struct Hfb {
    nsc: CMat,
    pp: CMat,
    st: State,
    e: f64,
    res: f64,
    it: usize,
}

/// the self-consistent mean fields: the fixed point of (N_sc, Ppair) -> mean fields of the ground state of H_var,
/// found by the Anderson iteration from the start (nsc0, pp0)
fn hfb_solve(md: &Model, gr: &Grid, m: f64, mu: f64, nsc0: &CMat, pp0: &CMat, tol: f64, maxit: usize) -> Hfb {
    let mut map = |x: &[f64]| -> Vec<f64> {
        let (n, p) = unpack(x);
        let st = ground(md, gr, m, mu, &n, &p);
        let (n2, p2) = md.fields(&st.cn, &st.cp);
        pack(&n2, &p2)
    };
    let (x, _res, it) = anderson(&mut map, pack(nsc0, pp0), 6, 0.6, tol, maxit);
    let (nsc, pp) = unpack(&x);
    let st = ground(md, gr, m, mu, &nsc, &pp);
    let (n2, p2) = md.fields(&st.cn, &st.cp);
    let res = n2.sub(&nsc).max_abs().max(p2.sub(&pp).max_abs());
    let e = energy(md, &st, mu);
    Hfb { nsc, pp, st, e, res, it }
}

/// the order parameter d_iota = tr(Kch_iota Ppair)/8 of a pairing block Ppair = sum d_iota Kch_iota^dag
fn dvec(md: &Model, pp: &CMat) -> [C64; 3] {
    let mut d = [C64::ZERO; 3];
    for i in 0..3 {
        d[i] = tr_prod(&md.ks[i], pp).scale(1.0 / 8.0);
    }
    d
}

/// |d|^2 and the non-unitarity zeta = |d x d^*|/|d|^2
fn d_invariants(d: &[C64; 3]) -> (f64, f64) {
    let n2: f64 = d.iter().map(|z| z.norm_sqr()).sum();
    let cr = [
        d[1] * d[2].conj() - d[2] * d[1].conj(),
        d[2] * d[0].conj() - d[0] * d[2].conj(),
        d[0] * d[1].conj() - d[1] * d[0].conj(),
    ];
    let cn: f64 = cr.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    (n2, if n2 > 0.0 { cn / n2 } else { 0.0 })
}

// ------------------------------------------------------------------ closed forms

/// Xi(x) = 2 int (q^2 + x)^(1/2), closed form
fn xi0(x: f64) -> f64 {
    let l = LAMBDA;
    let ash = if x > 1e-300 { x * x * (l / x.sqrt()).asinh() } else { 0.0 };
    2.0 * (l * (2.0 * l * l + x) * (l * l + x).sqrt() - ash) / 8.0 / (2.0 * PI * PI)
}

/// Xi_1(x) = int (q^2 + x)^(-1/2) = (Lambda sqrt(Lambda^2 + x) - x arsinh(Lambda/sqrt(x)))/(4 pi^2), the derivative of Xi
fn xi1(x: f64) -> f64 {
    let l = LAMBDA;
    let ash = if x > 1e-300 { x * (l / x.sqrt()).asinh() } else { 0.0 };
    (l * (l * l + x).sqrt() - ash) / (4.0 * PI * PI)
}

/// N_Lambda = Lambda^3/(6 pi^2)
fn nlam() -> f64 {
    LAMBDA.powi(3) / (6.0 * PI * PI)
}

/// the energy per volume of the trial state of the vacuum sector with the Dirac mass mv and the gap dv, for the
/// bare mass m and the coupling g at the charge-symmetric chemical potential:
/// V = -2 Xi(x) + 4 (mv - m) s + 4 dv f - 3 g s^2 - 4 g f^2 + 3 g N_Lambda^2, x = mv^2 + dv^2, s = mv Xi_1(x), f = dv Xi_1(x)
fn v_closed(m: f64, g: f64, mv: f64, dv: f64) -> f64 {
    let x = mv * mv + dv * dv;
    let (s, f) = (mv * xi1(x), dv * xi1(x));
    -2.0 * xi0(x) + 4.0 * (mv - m) * s + 4.0 * dv * f - 3.0 * g * s * s - 4.0 * g * f * f + 3.0 * g * nlam() * nlam()
}

/// the Dirac mass M_N of the normal stationary state: the root of M = m + (3/2) g M Xi_1(M^2), by bisection
fn mass_normal(m: f64, g: f64) -> f64 {
    let f = |x: f64| x - m - 1.5 * g * x * xi1(x * x);
    let (mut lo, mut hi) = (m, m + 1.5 * g * nlam() + 1e-9);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if f(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// the gap of a condensed state with the Dirac mass md: the root of 1 = 2 g Xi_1(md^2 + Delta^2) (None below the
/// critical coupling 1/(2 Xi_1(md^2)))
fn gap_of(md: f64, g: f64) -> Option<f64> {
    if 2.0 * g * xi1(md * md) <= 1.0 {
        return None;
    }
    let (mut lo, mut hi) = (0.0, 2.0 * g * nlam() + 1.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if 2.0 * g * xi1(md * md + mid * mid) > 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(0.5 * (lo + hi))
}

/// the critical coupling of a vacuum with the Dirac mass md
fn g_crit(md: f64) -> f64 {
    1.0 / (2.0 * xi1(md * md))
}

/// the bare mass that the normal ordering of the interaction with respect to the vacuum implies, for the physical
/// mass m: m (1 - 3 g/(4 g_crit(m))) in the normal vacuum, m/4 in the condensed vacuum
fn bare_mass(m: f64, g: f64) -> f64 {
    let gc = g_crit(m);
    if g <= gc {
        m * (1.0 - 0.75 * g / gc)
    } else {
        0.25 * m
    }
}

/// the pair susceptibility of a normal vacuum with the Dirac mass md at the squared frequency w2 (negative for an
/// imaginary frequency): chi_pr = int 8 eps/(4 eps^2 - w2), eps = (q^2 + md^2)^(1/2), in closed form (w2 <= 4 md^2)
fn chi_pr(w2: f64, md: f64) -> f64 {
    let l = LAMBDA;
    let el = (l * l + md * md).sqrt();
    let ash = (l / md).asinh();
    let a2 = (md * md - 0.25 * w2).max(0.0);
    let a = a2.sqrt();
    let base = 0.5 * (l * el + md * md * ash) - a2 * ash;
    let extra = if a == 0.0 {
        0.0
    } else if w2 >= 0.0 {
        let b = 0.5 * w2.sqrt();
        -a * b * (l * b / (a * el)).atan()
    } else {
        let b = 0.5 * (-w2).sqrt();
        a * b * (l * b / (a * el)).atanh()
    };
    (base + extra) / (PI * PI)
}

/// the same susceptibility by a composite Gauss-Legendre rule in the length of the momentum
fn chi_pr_quad(w2: f64, md: f64) -> f64 {
    let (xs, ws) = quad::composite(0.0, LAMBDA, &[0.001, 0.01, 0.1], 12, 20);
    let mut s = 0.0;
    for (q, w) in xs.iter().zip(&ws) {
        let e = (q * q + md * md).sqrt();
        s += w * q * q / (2.0 * PI * PI) * 8.0 * e / (4.0 * e * e - w2);
    }
    s
}

/// the mass of the SFBS boson of a normal vacuum with the Dirac mass md: the root of g chi_pr(w^2) = 1 below the
/// threshold 2 md (None if the vacuum is unstable, g chi_pr(0) >= 1, or if there is no bound state)
fn boson_mass(g: f64, md: f64) -> Option<f64> {
    if g * chi_pr(0.0, md) >= 1.0 || g * chi_pr(4.0 * md * md, md) <= 1.0 {
        return None;
    }
    let (mut lo, mut hi) = (0.0, 2.0 * md);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if g * chi_pr(mid * mid, md) < 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(0.5 * (lo + hi))
}

/// the growth rate of the pair field on an unstable normal state with the Dirac mass md: the root of
/// g chi_pr(-gamma^2) = 1 (None if the state is stable)
fn growth_rate(g: f64, md: f64) -> Option<f64> {
    if g * chi_pr(0.0, md) <= 1.0 {
        return None;
    }
    let (mut lo, mut hi) = (0.0, 1.0);
    while g * chi_pr(-hi * hi, md) > 1.0 {
        hi *= 2.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if g * chi_pr(-mid * mid, md) > 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    Some(0.5 * (lo + hi))
}

/// the binding coupling of the normal vacuum with the Dirac mass md: 1/chi_pr at the threshold
fn g_bind(md: f64) -> f64 {
    1.0 / chi_pr(4.0 * md * md, md)
}

// ------------------------------------------------------------------ the Dirac matrices and the normal channels

/// the 16 Hermitian Dirac matrices in 8 classes: (name of the class, number of the class, matrix)
fn dirac_basis() -> Vec<(&'static str, usize, CMat)> {
    let (b, g5) = (beta(), gamma_ch());
    let mut v = vec![
        ("1", 0, CMat::identity(4)),
        ("beta", 1, b.clone()),
        ("i beta gamma_ch", 2, b.mul(&g5).scale(C64::I)),
        ("gamma_ch", 3, g5.clone()),
    ];
    for j in 1..4 {
        v.push(("alpha^j", 4, alpha(j)));
    }
    for j in 1..4 {
        v.push(("gamma_ch alpha^j", 5, g5.mul(&alpha(j))));
    }
    for j in 1..4 {
        v.push(("i beta alpha^j", 6, b.mul(&alpha(j)).scale(C64::I)));
    }
    for j in 1..4 {
        v.push(("beta gamma_ch alpha^j", 7, b.mul(&g5).mul(&alpha(j))));
    }
    v
}

/// the 64 channel matrices T = D (x) tau_b (b = 0: unit matrix): (number of the Dirac class, b, matrix)
fn channels() -> Vec<(usize, usize, CMat)> {
    let mut v = Vec::new();
    for (_, cl, d) in dirac_basis() {
        for b in 0..4 {
            v.push((cl, b, d.kron(&sigma(b))));
        }
    }
    v
}

/// the coefficient c of sum_k K T K^dag = c T^T for a set of pair matrices, and the largest entry of the difference
fn channel_coefficient(ks: &[CMat], t: &CMat) -> (f64, f64) {
    let mut s = CMat::zeros(8);
    for k in ks {
        s = s.add(&k.mul(t).mul(&k.dagger()));
    }
    let tt = t.transpose();
    let cc = tr_prod(&s, &t.conj()).scale(1.0 / 8.0);
    let err = s.sub(&tt.scale(cc)).max_abs() + cc.im.abs();
    (cc.re, err)
}

// ------------------------------------------------------------------ the Fock space of one site (8 modes, 256 states)

/// the dense matrix of sum_ab K[a][b] c_a c_b
fn fock_pair(k: &CMat) -> CMat {
    let k = k.clone();
    fock::dense(256, &move |v: &[C64]| -> Vec<C64> {
        let mut out = vec![C64::ZERO; v.len()];
        for a in 0..8 {
            for b in 0..8 {
                let z = k.get(a, b);
                if z.abs() == 0.0 {
                    continue;
                }
                let t = fock::ann(&fock::ann(v, b), a);
                for (o, x) in out.iter_mut().zip(t) {
                    *o += z * x;
                }
            }
        }
        out
    })
}

/// the dense matrix of c^dag h c + (1/2) (sum_ab P[a][b] c_a^dag c_b^dag + Hermitian conjugate)
fn fock_quadratic(h: &CMat, p: &CMat) -> CMat {
    let (h, p) = (h.clone(), p.clone());
    fock::dense(256, &move |v: &[C64]| -> Vec<C64> {
        let mut out = vec![C64::ZERO; v.len()];
        for a in 0..8 {
            for b in 0..8 {
                let z = h.get(a, b);
                if z.abs() > 0.0 {
                    fock::hop_add(v, a, b, z, &mut out);
                }
                let w = p.get(a, b).scale(0.5);
                if w.abs() > 0.0 {
                    fock::pair_cre_add(v, a, b, w, &mut out);
                    fock::pair_ann_add(v, a, b, w.conj(), &mut out);
                }
            }
        }
        out
    })
}

/// the Hamiltonian of one site, c^dag (m beta - mu) c - (G/4) sum Pi^dag Pi, as a dense matrix
fn fock_site_hamiltonian(md: &Model, m: f64, mu: f64, pis: &[CMat]) -> CMat {
    let h = md.beta8.scale_r(m).sub(&CMat::identity(8).scale_r(mu));
    let mut ham = fock_quadratic(&h, &CMat::zeros(8));
    for p in pis {
        ham = ham.sub(&p.dagger().mul(p).scale_r(0.25 * md.g));
    }
    ham
}

/// the correlation matrices C_n[a][b] = <c_b^dag c_a> and C_p[a][b] = <c_a c_b> of a state of the Fock space
fn fock_correlations(gs: &[C64]) -> (CMat, CMat) {
    let mut cn = CMat::zeros(8);
    let mut cp = CMat::zeros(8);
    for a in 0..8 {
        for b in 0..8 {
            let mut out = vec![C64::ZERO; gs.len()];
            fock::hop_add(gs, b, a, C64::ONE, &mut out);
            cn.set(a, b, fock::dot(gs, &out));
            cp.set(a, b, fock::dot(gs, &fock::ann(&fock::ann(gs, b), a)));
        }
    }
    (cn, cp)
}

/// the eigenvector of the lowest eigenvalue of a dense Hermitian matrix, and the eigenvalues
fn lowest(m: &CMat) -> (Vec<f64>, Vec<C64>) {
    let (e, v) = eigh(m);
    let gs: Vec<C64> = (0..m.rows).map(|i| v.get(i, 0)).collect();
    (e, gs)
}

// ------------------------------------------------------------------ CJ-1: the Wick decomposition

fn cj1(ch: &mut Checks) {
    let mut rng = Rng(20261119);
    let md = Model::new(1.0, false);
    let pis: Vec<CMat> = md.ks.iter().map(fock_pair).collect();
    let (mut e_wick, mut e_cn, mut e_cp, mut gapmin) = (0.0f64, 0.0f64, 0.0f64, f64::INFINITY);
    let ntest = 4;
    for _ in 0..ntest {
        // a random quadratic Hamiltonian with pairing, its ground state in the Fock space
        let h = rand_herm(&mut rng, 8, 1.0);
        let p = rand_antisym(&mut rng, 8, 0.7);
        let (ev, gs) = lowest(&fock_quadratic(&h, &p));
        gapmin = gapmin.min(ev[1] - ev[0]);
        let (cn, cp) = fock_correlations(&gs);
        // the same correlation matrices from the mean-field matrix of 16 rows
        let hb = CMat::blocks(&h, &p, &p.dagger(), &h.transpose().scale_r(-1.0));
        let (e, v) = eigh(&hb);
        let mut cn2 = CMat::zeros(8);
        let mut cp2 = CMat::zeros(8);
        for k in 0..16 {
            if e[k] >= 0.0 {
                continue;
            }
            for a in 0..8 {
                for b in 0..8 {
                    cn2.set(a, b, cn2.get(a, b) + v.get(a, k) * v.get(b, k).conj());
                    cp2.set(a, b, cp2.get(a, b) - v.get(a, k) * v.get(8 + b, k).conj());
                }
            }
        }
        e_cn = e_cn.max(cn.sub(&cn2).max_abs());
        e_cp = e_cp.max(cp.sub(&cp2).max_abs());
        // <sum Pi^dag Pi> directly, and by the three contractions
        let direct: f64 = pis.iter().map(|pm| fock::norm(&pm.matvec(&gs)).powi(2)).sum();
        let wick = -4.0 * (md.e_pair(&cp) + md.e_normal(&cn));
        e_wick = e_wick.max((direct - wick).abs());
    }
    ch.add(
        "CJ-1",
        "Wick's theorem for the interaction, in the Fock space of one site (8 fermion modes, 256 states): for the ground state of a random quadratic Hamiltonian c^dag h c + (1/2)(c^dag P c^dag^T + Hermitian conjugate) (h Hermitian, P antisymmetric, both random), <sum_iota Pi_iota^dag Pi_iota> computed directly in the Fock space equals sum_iota |<Pi_iota>|^2 + 2 sum_iota tr(Kch_iota C_n Kch_iota^dag C_n^T); and the correlation matrices C_n[a][b] = <c_b^dag c_a>, C_p[a][b] = <c_a c_b> of that ground state are the upper left block and minus the upper right block of the projector on the eigenvectors with negative eigenvalues of the matrix [[h, P], [P^dag, -h^T]] of 16 rows",
        e_wick < 1e-10 && e_cn < 1e-10 && e_cp < 1e-10 && gapmin > 1e-6,
        format!(
            "{} random Hamiltonians: largest |direct - Wick| = {}; largest difference of C_n {}, of C_p {} between the Fock space and the matrix of 16 rows; smallest gap above the ground state {:.3}",
            ntest,
            sci(e_wick),
            sci(e_cn),
            sci(e_cp),
            gapmin
        ),
    );
}

// ------------------------------------------------------------------ CJ-2: the normal channels

/// the coefficients of the 8 Dirac classes: [class][0: tau_0, 1: tau_iota] for the scalar and for the pseudoscalar pairs
fn cj2(ch: &mut Checks, fr: &mut Frags, dir: &PathBuf) -> ([[f64; 2]; 8], [[f64; 2]; 8]) {
    let ks: Vec<CMat> = (1..4).map(kpair).collect();
    let kps: Vec<CMat> = (1..4).map(kpair_ps).collect();
    let basis = dirac_basis();
    let names: Vec<&str> = (0..8).map(|cl| basis.iter().find(|x| x.1 == cl).unwrap().0).collect();
    let counts: Vec<usize> = (0..8).map(|cl| basis.iter().filter(|x| x.1 == cl).count()).collect();
    // the expected values: sign of the Dirac class times 3 (tau_0) or 1 (tau_iota)
    let sign_s = [1.0, 1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0];
    let sign_p = [1.0, -1.0, 1.0, 1.0, -1.0, -1.0, -1.0, 1.0];
    let (mut cs, mut cps) = ([[0.0; 2]; 8], [[0.0; 2]; 8]);
    let (mut err, mut dev, mut orth) = (0.0f64, 0.0f64, 0.0f64);
    let chs = channels();
    for (i, (cl, b, t)) in chs.iter().enumerate() {
        let (c1, e1) = channel_coefficient(&ks, t);
        let (c2, e2) = channel_coefficient(&kps, t);
        err = err.max(e1).max(e2);
        let k = if *b == 0 { 0 } else { 1 };
        let w = if *b == 0 { 3.0 } else { 1.0 };
        dev = dev.max((c1 - sign_s[*cl] * w).abs()).max((c2 - sign_p[*cl] * w).abs());
        cs[*cl][k] = c1;
        cps[*cl][k] = c2;
        // the channel matrices are Hermitian and orthogonal: tr(T_A T_B) = 8 delta_AB
        orth = orth.max(t.sub(&t.dagger()).max_abs());
        for (j, (_, _, t2)) in chs.iter().enumerate() {
            let x = tr_prod(t, t2);
            orth = orth.max((x - c(if i == j { 8.0 } else { 0.0 }, 0.0)).abs());
        }
    }
    // the mean field and the normal energy of a random Hermitian C_n through the coefficients
    let mut rng = Rng(77);
    let md = Model::new(1.7, false);
    let cn = rand_herm(&mut rng, 8, 0.4);
    let (nsc, _) = md.fields(&cn, &CMat::zeros(8));
    let mut nsc2 = CMat::zeros(8);
    let mut en2 = 0.0;
    for (cl, b, t) in &chs {
        let n_a = tr_prod(t, &cn).re;
        let cc = cs[*cl][if *b == 0 { 0 } else { 1 }];
        nsc2 = nsc2.add(&t.scale_r(-md.g / 8.0 * cc * n_a));
        en2 += -md.g / 16.0 * cc * n_a * n_a;
    }
    let e_n = nsc.sub(&nsc2).max_abs();
    let e_e = (md.e_normal(&cn) - en2).abs();
    ch.add(
        "CJ-2",
        "the 64 normal channels: for every matrix T = D (x) tau_b (D one of the 16 Hermitian Dirac matrices 1, beta, i beta gamma_ch, gamma_ch, alpha^j, gamma_ch alpha^j, i beta alpha^j, beta gamma_ch alpha^j; tau_0 the unit matrix), sum_iota Kch_iota T Kch_iota^dag = c_ch T^T with c_ch = sign_D times 3 for tau_0 and sign_D times 1 for tau_iota, sign_D = +1 for 1, beta, gamma_ch, i beta alpha^j and -1 for the other four classes; the same for the pseudoscalar pair matrices Kch'_iota with the signs +1 for 1, i beta gamma_ch, gamma_ch, beta gamma_ch alpha^j and -1 for the other four; the 64 matrices are Hermitian with tr(T_ch T_ch') = 8 or 0; and for a random Hermitian C_n the mean field N_sc = -G sum_iota Kch_iota^dag C_n^T Kch_iota equals -(G/8) sum_ch c_ch n_ch T_ch and the normal term of the energy equals -(G/16) sum_ch c_ch n_ch^2, with n_ch = tr(T_ch C_n)",
        err < 1e-12 && dev < 1e-12 && orth < 1e-12 && e_n < 1e-12 && e_e < 1e-12,
        format!(
            "largest entry of sum K T K^dag - c_ch T^T {}; largest |c_ch - expected| {}; orthogonality {}; mean field {}; normal energy {}",
            sci(err),
            sci(dev),
            sci(orth),
            sci(e_n),
            sci(e_e)
        ),
    );
    let mut csv = Csv::new(
        "the coefficients c_ch of the normal channels T = D (x) tau_b in sum_iota K T K^dag = c_ch T^T, for the scalar pair matrices, for the pseudoscalar pair matrices, and their sum (the coefficient of the axially symmetric completion of the interaction)",
        &[("program", "t2_cjt (test2/rust/t2cjt), check CJ-2".to_string())],
        "dirac_matrix,isospin_matrix,number_of_channels,coefficient_scalar,coefficient_pseudoscalar,coefficient_sum",
    );
    let tex = [
        r"$\mathsf 1_4$",
        r"$\beta_{\rm LL}$",
        r"$i\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}$",
        r"$\gamma^{\rm ch}_{\rm LL}$",
        r"$\alpha^j_{\rm LL}$",
        r"$\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$",
        r"$i\beta_{\rm LL}\alpha^j_{\rm LL}$",
        r"$\beta_{\rm LL}\gamma^{\rm ch}_{\rm LL}\alpha^j_{\rm LL}$",
    ];
    let what = [
        "density of the spinor charges",
        "scalar density (the Dirac mass)",
        "pseudoscalar density",
        "axial charge density",
        "current",
        "spin density",
        "electric-type tensor",
        "magnetic-type tensor",
    ];
    let mut table = String::from(
        "| Dirac matrix | its density is the | number | $\\varsigma^{\\rm D}_{\\rm LL}$ | $c^A_{\\rm LL}$ with $\\mathsf 1_2$, with $\\tau^{\\rm LL}_\\iota$ | $c'^A_{\\rm LL}$ with $\\mathsf 1_2$, with $\\tau^{\\rm LL}_\\iota$ | sum with $\\mathsf 1_2$, with $\\tau^{\\rm LL}_\\iota$ |\n|----------------|--------------------------|-------|------|-------------|-------------|-------------|\n",
    );
    for cl in 0..8 {
        for k in 0..2 {
            csv.row(&[
                names[cl].to_string(),
                if k == 0 { "tau_0".to_string() } else { "tau_iota".to_string() },
                format!("{}", counts[cl] * if k == 0 { 1 } else { 3 }),
                format!("{:.0}", cs[cl][k]),
                format!("{:.0}", cps[cl][k]),
                format!("{:.0}", cs[cl][k] + cps[cl][k]),
            ]);
        }
        table.push_str(&format!(
            "| {} | {} | {} | ${:+.0}$ | ${:+.0}$, ${:+.0}$ | ${:+.0}$, ${:+.0}$ | ${:.0}$, ${:.0}$ |\n",
            tex[cl],
            what[cl],
            counts[cl],
            sign_s[cl],
            cs[cl][0],
            cs[cl][1],
            cps[cl][0],
            cps[cl][1],
            cs[cl][0] + cps[cl][0],
            cs[cl][1] + cps[cl][1]
        ));
    }
    csv.write(dir, "t2_cjt_channels.csv");
    fr.add("cjt_table_channels", table);
    (cs, cps)
}

// ------------------------------------------------------------------ CJ-3: the commutator and the charge conjugation

fn cj3(ch: &mut Checks, fr: &mut Frags) {
    let md = Model::new(1.0, true);
    let pis: Vec<CMat> = md.ks.iter().map(fock_pair).collect();
    let nf = fock_quadratic(&CMat::identity(8), &CMat::zeros(8));
    let target = CMat::identity(256).scale_r(48.0).sub(&nf.scale_r(12.0));
    let comm = |set: &[CMat]| -> CMat {
        let mut s = CMat::zeros(256);
        for p in set {
            let pd = p.dagger();
            s = s.add(&p.mul(&pd)).sub(&pd.mul(p));
        }
        s
    };
    let e_s = comm(&pis[0..3]).sub(&target).max_abs();
    let e_p = comm(&pis[3..6]).sub(&target).max_abs();
    // the matrix of the charge conjugation: c_a -> sum_b U[a][b] c_b^dag with U^dag alpha^j U = (alpha^j)^T,
    // U^dag beta U = -beta^T, and -(U^T Kch_iota U)^* a unitary combination of the Kch_iota
    let mut found: Option<(String, f64)> = None;
    for (idx, (name, cl, d)) in dirac_basis().into_iter().enumerate() {
        // the number j of the matrix within its class (the classes 4 to 7 have three matrices each)
        let name = if cl >= 4 { name.replace("^j", &format!("^{}", (idx - 4) % 3 + 1)) } else { name.to_string() };
        for b in 0..4 {
            let u = d.kron(&sigma(b));
            let ud = u.dagger();
            let mut e = ud.mul(&md.beta8).mul(&u).add(&md.beta8.transpose()).max_abs();
            for j in 0..3 {
                e = e.max(ud.mul(&md.alpha8[j]).mul(&u).sub(&md.alpha8[j].transpose()).max_abs());
            }
            let mut o = CMat::zeros(3);
            for i in 0..3 {
                let k2 = u.transpose().mul(&md.ks[i]).mul(&u).conj().scale_r(-1.0);
                let mut rec = CMat::zeros(8);
                for k in 0..3 {
                    let x = tr_prod(&md.kds[k], &k2).scale(1.0 / 8.0);
                    o.set(i, k, x);
                    rec = rec.add(&md.ks[k].scale(x));
                }
                e = e.max(rec.sub(&k2).max_abs());
            }
            e = e.max(o.dagger().mul(&o).sub(&CMat::identity(3)).max_abs());
            if e < 1e-12 && found.is_none() {
                found = Some((format!("({}) (x) tau_{}", name, b), e));
            }
        }
    }
    // the spectra of the Hamiltonian of one site at mu and at -mu - 3 G differ by the constant -8 mu - 12 G
    let mut rng = Rng(5);
    let mut e_spec = 0.0f64;
    let mut pars = Vec::new();
    for _ in 0..3 {
        let (m, mu, g) = (0.1 + 0.4 * rng.unit().abs(), rng.unit(), 0.5 + 2.0 * rng.unit().abs());
        let mdl = Model::new(g, false);
        let (e1, _) = lowest(&fock_site_hamiltonian(&mdl, m, mu, &pis[0..3]));
        let (e2, _) = lowest(&fock_site_hamiltonian(&mdl, m, -mu - 3.0 * g, &pis[0..3]));
        for k in 0..256 {
            e_spec = e_spec.max((e1[k] - (e2[k] - 8.0 * mu - 12.0 * g)).abs());
        }
        pars.push(format!("(m, mu, G) = ({:.3}, {:.3}, {:.3})", m, mu, g));
    }
    let (uname, uerr) = found.clone().unwrap_or(("none".to_string(), f64::NAN));
    ch.add(
        "CJ-3",
        "the order of the factors of the interaction and the charge conjugation, on one site (256 states): sum_iota [Pi_iota, Pi_iota^dag] = 48 - 12 n_F_site for the scalar pair operators and for the pseudoscalar ones (n_F_site: the number of spinor charges on the site); there is a matrix U_C = D (x) tau_b with U_C^dag alpha^j U_C = (alpha^j)^T, U_C^dag beta U_C = -beta^T, for which -(U_C^T Kch_iota U_C)^* is a unitary combination of the Kch_iota (the exchange c_a -> sum_b U_C[a][b] c_b^dag of particles and holes keeps the kinetic term and turns sum Pi^dag Pi into sum Pi Pi^dag); and the 256 eigenvalues of the Hamiltonian of one site with the chemical potential mu equal those with the chemical potential -mu - 3 G, shifted by -8 mu - 12 G",
        e_s < 1e-12 && e_p < 1e-12 && found.is_some() && e_spec < 1e-10,
        format!(
            "commutator: largest entry of the difference {} (scalar), {} (pseudoscalar); U_C = {} (largest violation {}); spectra: largest difference {} for {}",
            sci(e_s),
            sci(e_p),
            uname,
            sci(uerr),
            sci(e_spec),
            pars.join(", ")
        ),
    );
    fr.add("cjt_uc_name", uname);
}

// ------------------------------------------------------------------ CJ-4: the three terms of the action; trial states

fn cj4(ch: &mut Checks) {
    // (a) the frequency integral of the trace-log term: for two Hermitian matrices of 16 rows with the spectra e and e',
    // (1/4) int dw/(2 pi) sum_k ln((w^2 + e_k^2)/(w^2 + e'_k^2)) = (1/4) sum_k (|e_k| - |e'_k|)
    let mut rng = Rng(31);
    let h1 = rand_herm(&mut rng, 8, 0.6);
    let p1 = rand_antisym(&mut rng, 8, 0.5);
    let h2 = rand_herm(&mut rng, 8, 0.6);
    let (e1, v1) = eigh(&CMat::blocks(&h1, &p1, &p1.dagger(), &h1.transpose().scale_r(-1.0)));
    let (e2, _) = eigh(&CMat::blocks(&h2, &CMat::zeros(8), &CMat::zeros(8), &h2.transpose().scale_r(-1.0)));
    // w = tan(theta): dw = dtheta/cos^2(theta)
    let (xs, ws) = quad::composite(-0.5 * PI, 0.5 * PI, &[-1.5, -1.0, 0.0, 1.0, 1.5], 40, 20);
    let mut integral = 0.0;
    for (th, wt) in xs.iter().zip(&ws) {
        let w = th.tan();
        let mut s = 0.0;
        for k in 0..16 {
            s += ((w * w + e1[k] * e1[k]) / (w * w + e2[k] * e2[k])).ln();
        }
        integral += wt / (th.cos() * th.cos()) * 0.25 * s / (2.0 * PI);
    }
    let exact: f64 = 0.25 * (0..16).map(|k| e1[k].abs() - e2[k].abs()).sum::<f64>();
    let e_log = (integral - exact).abs();
    // the symmetric equal-time limit of the propagator: -int dw/(2 pi) sum_k v_k v_k^dag e_k/(w^2 + e_k^2) = R - 1/2,
    // R the projector on the negative eigenvalues; and the spectrum is symmetric (trace zero)
    let mut e_eq = 0.0f64;
    let mut e_sym = 0.0f64;
    for k in 0..16 {
        e_sym = e_sym.max((e1[k] + e1[15 - k]).abs());
    }
    for a in 0..16 {
        for b in 0..16 {
            let mut lhs = C64::ZERO;
            let mut rhs = if a == b { c(-0.5, 0.0) } else { C64::ZERO };
            for k in 0..16 {
                let vv = v1.get(a, k) * v1.get(b, k).conj();
                // int dw/(2 pi) e/(w^2 + e^2) = sign(e)/2
                lhs += vv.scale(-0.5 * e1[k].signum());
                if e1[k] < 0.0 {
                    rhs += vv;
                }
            }
            e_eq = e_eq.max((lhs - rhs).abs());
        }
    }
    // (b) the energy of the trial states of the vacuum sector: the grid against the closed form, at the
    // charge-symmetric chemical potential
    let gr = Grid::product(64, 4, 6);
    let nl = gr.nlam();
    let (m, g) = (0.05, G_T2);
    let mdl = Model::new(g, false);
    let mu = -1.5 * g * nl;
    let (mut e_v, mut e_cn, mut e_pi, mut e_grad) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let trials = [(0.2, 0.2094933), (0.31, 0.0), (0.12, 0.33), (0.227347, 0.07)];
    for &(mv, dv) in &trials {
        let nsc = mdl.beta8.scale_r(mv - m).add(&CMat::identity(8).scale_r(mu));
        let pp = mdl.kds[2].scale_r(dv);
        let st = ground(&mdl, &gr, m, mu, &nsc, &pp);
        e_v = e_v.max((energy(&mdl, &st, mu) - v_closed(m, g, mv, dv)).abs());
        let x = mv * mv + dv * dv;
        let (s, f) = (mv * xi1(x), dv * xi1(x));
        let cn_exp = CMat::identity(8).scale_r(0.5 * nl).sub(&mdl.beta8.scale_r(0.5 * s));
        e_cn = e_cn.max(st.cn.sub(&cn_exp).max_abs());
        let pi = mdl.pairs(&st.cp);
        e_pi = e_pi.max((pi[2] - c(-4.0 * f, 0.0)).abs()).max(pi[0].abs()).max(pi[1].abs());
        // the gradient of V: the closed formula against central differences
        let h = 1e-5;
        let dvm = (v_closed(m, g, mv + h, dv) - v_closed(m, g, mv - h, dv)) / (2.0 * h);
        let dvd = (v_closed(m, g, mv, dv + h) - v_closed(m, g, mv, dv - h)) / (2.0 * h);
        // d s/d mv = Xi_1 + 2 mv^2 Xi_2, d f/d mv = d s/d dv = 2 mv dv Xi_2, d f/d dv = Xi_1 + 2 dv^2 Xi_2, with
        // Xi_2 = d Xi_1/dx by a central difference of the closed form
        let x2 = (xi1(x + 1e-6) - xi1(x - 1e-6)) / 2e-6;
        let (a1, a2) = (4.0 * (mv - m) - 6.0 * g * s, 4.0 * dv - 8.0 * g * f);
        let gm = a1 * (xi1(x) + 2.0 * mv * mv * x2) + a2 * (2.0 * mv * dv * x2);
        let gd = a1 * (2.0 * mv * dv * x2) + a2 * (xi1(x) + 2.0 * dv * dv * x2);
        e_grad = e_grad.max((dvm - gm).abs()).max((dvd - gd).abs());
    }
    ch.add(
        "CJ-4",
        "the terms of the effective action on the trial family: (a) for the spectra e_k, e'_k of two random particle-hole symmetric Hermitian matrices of 16 rows, (1/4) int dw/(2 pi) sum_k ln((w^2 + e_k^2)/(w^2 + e'_k^2)) = (1/4) sum_k (|e_k| - |e'_k|) (the trace-log term is minus the ground energy of the trial Hamiltonian), the spectrum is symmetric about zero, and -(1/2) sum_k sign(e_k) v_k v_k^dag = R_Nb - 1/2 with R_Nb the projector on the negative eigenvalues (the symmetric equal-time limit of the propagator); (b) for trial states of the vacuum sector with the Dirac mass M_trial and the gap Delta_trial (bare mass 0.05, G = 22.5, charge-symmetric chemical potential), the energy E_var from the matrices of 16 rows on the momentum grid equals the closed form V_CJT = -2 Xi + 4 (M_trial - m_bare) s_c + 4 Delta_trial f_c - 3 G s_c^2 - 4 G f_c^2 + 3 G N_Lambda^2, with C_n = (N_Lambda/2) - (s_c/2) beta and <Pi_3> = -4 f_c, <Pi_1> = <Pi_2> = 0; and the gradient of V_CJT is (4 (M_trial - m_bare) - 6 G s_c) grad s_c + (4 Delta_trial - 8 G f_c) grad f_c",
        e_log < 1e-9 && e_eq < 1e-12 && e_sym < 1e-12 && e_v < 1e-12 && e_cn < 1e-12 && e_pi < 1e-12 && e_grad < 1e-7,
        format!(
            "(a) frequency integral {} against {} (difference {}); symmetry of the spectrum {}; equal-time limit {}; (b) {} trial states: largest |E_var - V_CJT| {}, C_n {}, <Pi> {}, gradient against central differences {}",
            f10(integral),
            f10(exact),
            sci(e_log),
            sci(e_sym),
            sci(e_eq),
            trials.len(),
            sci(e_v),
            sci(e_cn),
            sci(e_pi),
            sci(e_grad)
        ),
    );
}

// ------------------------------------------------------------------ CJ-5: the vacuum sector with bare parameters

/// the mean fields of the stationary state of the vacuum sector with the Dirac mass mv and the gap dv
fn vacuum_fields(md: &Model, m: f64, mu: f64, mv: f64, dv: f64) -> (CMat, CMat) {
    (md.beta8.scale_r(mv - m).add(&CMat::identity(8).scale_r(mu)), md.kds[2].scale_r(dv))
}

fn cj5(ch: &mut Checks, fr: &mut Frags) {
    let gr = Grid::product(64, 4, 6);
    let nl = gr.nlam();
    let mut lines = Vec::new();
    let (mut e_fix, mut e_n, mut e_4m, mut e_en) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    let mut ok_order = true;
    // (bare mass, coupling)
    let cases = [(0.05, 18.0), (0.05, G_T2), (0.05, 30.0), (0.2, 15.0), (0.2, G_T2), (0.2, 40.0)];
    for &(m, g) in &cases {
        let md = Model::new(g, false);
        let mu = -1.5 * g * nl;
        let mn = mass_normal(m, g);
        // the normal stationary state
        let (nsc, pp) = vacuum_fields(&md, m, mu, mn, 0.0);
        let st = ground(&md, &gr, m, mu, &nsc, &pp);
        let (n2, p2) = md.fields(&st.cn, &st.cp);
        e_fix = e_fix.max(n2.sub(&nsc).max_abs()).max(p2.sub(&pp).max_abs());
        e_n = e_n.max((st.cn.trace().re - 4.0 * nl).abs());
        let en = energy(&md, &st, mu);
        e_en = e_en.max((en - v_closed(m, g, mn, 0.0)).abs());
        let mut line = format!("(m_bare, G) = ({}, {}): normal M_N = {}", m, g, f7(mn));
        // the condensed stationary state, if the coupling exceeds the critical one
        if let Some(dv) = gap_of(4.0 * m, g) {
            let (nsc, pp) = vacuum_fields(&md, m, mu, 4.0 * m, dv);
            let st = ground(&md, &gr, m, mu, &nsc, &pp);
            let (n2, p2) = md.fields(&st.cn, &st.cp);
            e_fix = e_fix.max(n2.sub(&nsc).max_abs()).max(p2.sub(&pp).max_abs());
            e_n = e_n.max((st.cn.trace().re - 4.0 * nl).abs());
            e_4m = e_4m.max((m + tr_prod(&md.beta8, &n2).re / 8.0 - 4.0 * m).abs());
            let ec = energy(&md, &st, mu);
            e_en = e_en.max((ec - v_closed(m, g, 4.0 * m, dv)).abs());
            ok_order &= ec < en && mn > 4.0 * m;
            line.push_str(&format!(", condensed M_D = 4 m_bare = {}, gap {}, E_c - E_n = {:.4e}", 4.0 * m, f7(dv), ec - en));
        } else {
            ok_order &= mn < 4.0 * m;
            line.push_str(", no condensed state");
        }
        lines.push(line);
    }
    // continuity at the critical coupling: M_N = 4 m_bare there
    let mut e_cont = 0.0f64;
    for &m in &[0.05, 0.2] {
        let gc = g_crit(4.0 * m);
        e_cont = e_cont.max((mass_normal(m, gc) - 4.0 * m).abs());
    }
    ch.add(
        "CJ-5",
        "the stationary states of the vacuum sector with bare parameters, at the charge-symmetric chemical potential mu_bare = -(3/2) G N_Lambda: the normal state with the Dirac mass M_N, the root of M_N = m_bare + (3/2) G M_N Xi_1(M_N^2), and, above the critical coupling G_crit_bare = 1/(2 Xi_1(16 m_bare^2)), the condensed state with the Dirac mass M_D = 4 m_bare exactly and the gap from 1 = 2 G Xi_1(16 m_bare^2 + Delta^2), are fixed points of the general self-consistency map of the matrices of 16 rows (mean fields reproduced); both have the density 4 N_Lambda (half filling); their energies equal the closed form V_CJT; the condensed state lies below the normal one where it exists, M_N > 4 m_bare there and M_N < 4 m_bare below the critical coupling; and M_N = 4 m_bare at the critical coupling (the transition is continuous)",
        e_fix < 1e-11 && e_n < 1e-12 && e_4m < 1e-11 && e_en < 1e-12 && ok_order && e_cont < 1e-12,
        format!(
            "largest residual of the mean fields {}; density - 4 N_Lambda {}; |M_D - 4 m_bare| {}; |E_var - V_CJT| {}; |M_N(G_crit_bare) - 4 m_bare| {}; {}",
            sci(e_fix),
            sci(e_n),
            sci(e_4m),
            sci(e_en),
            sci(e_cont),
            lines.join("; ")
        ),
    );
    fr.add("cjt_gcrit_bare", format!("{:.4}", g_crit(4.0 * M_T2)));
    fr.add("cjt_mn_bare_t2", format!("{:.6}", mass_normal(M_T2, G_T2)));
}

// ------------------------------------------------------------------ CJ-6: the normal ordering

fn cj6(ch: &mut Checks, fr: &mut Frags, dir: &PathBuf) {
    let gr = Grid::product(48, 2, 4);
    let nl = gr.nlam();
    let (m, g) = (M_T2, G_T2);
    let gc = g_crit(m);
    let mb = bare_mass(m, g);
    let dvac = gap_of(m, g).expect("condensed vacuum");
    let md = Model::new(g, false);
    let mu_b = -1.5 * g * nl;
    // (a) the vacuum of the normal-ordered theory is the stationary state of the bare functional with m_bare, mu_bare
    let (nsc, pp) = vacuum_fields(&md, mb, mu_b, m, dvac);
    let vac = ground(&md, &gr, mb, mu_b, &nsc, &pp);
    let (n2, p2) = md.fields(&vac.cn, &vac.cp);
    let e_fix = n2.sub(&nsc).max_abs().max(p2.sub(&pp).max_abs());
    let s_vac = m * xi1(m * m + dvac * dvac);
    let e_mb = (mb - (m - 1.5 * g * s_vac)).abs();
    // (b) the normal-ordered functional and the bare functional differ by a constant: random states
    let mut rng = Rng(424242);
    let mut e_const = 0.0f64;
    let const_expected = md.e_normal(&vac.cn);
    for _ in 0..4 {
        let nr = rand_herm(&mut rng, 8, 0.3).add(&CMat::identity(8).scale_r(mu_b));
        let pr = md.kds[0].scale(c(rng.unit() * 0.3, rng.unit() * 0.3)).add(&md.kds[1].scale(c(rng.unit() * 0.3, rng.unit() * 0.3))).add(&md.kds[2].scale(c(rng.unit() * 0.3, rng.unit() * 0.3)));
        let st = ground(&md, &gr, mb, mu_b, &nr, &pr);
        let e_bare = energy(&md, &st, mu_b);
        // E_no = int tr(h(q; m) C_n) - mu tr C_n (mu = 0) - (G/4) sum |<Pi>|^2 - (G/2) sum tr(K (C_n - C_vac) K^dag (C_n - C_vac)^T)
        let ekin_m = st.ekin + (m - mb) * tr_prod(&md.beta8, &st.cn).re;
        let dc = st.cn.sub(&vac.cn);
        let e_no = ekin_m + md.e_pair(&st.cp) + md.e_normal(&dc);
        e_const = e_const.max((e_no - e_bare - const_expected).abs());
    }
    // (c) the numbers of the document of Test 2 are reproduced
    let depth_po = dvac * dvac / g - 2.0 * (xi0(m * m + dvac * dvac) - xi0(m * m));
    let mn = mass_normal(mb, g);
    let depth_cjt = v_closed(mb, g, m, dvac) - v_closed(mb, g, mn, 0.0);
    // (d) the reading without normal ordering: the Lagrangian mass 0.2 as bare mass
    let mn_bare = mass_normal(m, g);
    let gc_bare = g_crit(4.0 * m);
    // (e) the vacuum is the minimum of the energy of the trial states: scan of a grid of (M_trial, Delta_trial)
    let mut vmin = f64::INFINITY;
    let mut csv = Csv::new(
        "the energy per volume of the trial states of the vacuum sector, V_CJT(M_trial, Delta_trial) minus its value at the vacuum, for the physical mass 0.2 with the interaction normal-ordered with respect to the vacuum, at two couplings",
        &[("program", "t2_cjt (test2/rust/t2cjt), check CJ-6".to_string()), ("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA))],
        "G,mass_trial,gap_trial,energy_above_vacuum",
    );
    for &gg in &[G_OSC, g] {
        let mbb = bare_mass(m, gg);
        let d0 = gap_of(m, gg).unwrap_or(0.0);
        let vv = v_closed(mbb, gg, m, d0);
        for i in 0..=80 {
            for j in 0..=80 {
                let (mv, dv) = (0.04 + 0.42 * i as f64 / 80.0, 0.4 * j as f64 / 80.0);
                let v = v_closed(mbb, gg, mv, dv) - vv;
                if gg == g {
                    vmin = vmin.min(v);
                }
                csv.row(&[format!("{}", gg), format!("{:.5}", mv), format!("{:.5}", dv), format!("{:.6e}", v)]);
            }
        }
    }
    csv.write(dir, "t2_cjt_surface.csv");
    ch.add(
        "CJ-6",
        "the normal ordering of the interaction with respect to the vacuum, at the parameters of the document of Test 2 (m = 0.2, G = 22.5, Lambda = 1): (a) the state with the Dirac mass M_D = m and the gap of the pairing-only gap equation 1 = 2 G Xi_1(m^2 + Delta^2) is a stationary state of the functional with the bare parameters m_bare = m - (3/2) G s_vac = m/4 and mu_bare = -(3/2) G N_Lambda; (b) for random states the normal-ordered functional E_no, in which C_n - C_vac replaces C_n in the normal term and m, mu = 0 replace m_bare, mu_bare, differs from the bare functional E_var by the same constant, the normal term of the vacuum; (c) the critical coupling 1/(2 Xi_1(m^2)) = 21.28661 and the gap Delta_vac = 0.2094933 of the pairing-only theory are reproduced; (d) without the normal ordering (0.2 as bare mass) the vacuum at G = 22.5 is normal, with the Dirac mass M_N = 0.539206, and the critical coupling is 1/(2 Xi_1(16 m^2)) = 32.35; (e) on a grid of 81 x 81 trial states (M_trial from 0.04 to 0.46, Delta_trial from 0 to 0.4) no energy lies below that of the vacuum",
        e_fix < 1e-11 && e_mb < 1e-13 && e_const < 1e-12 && (gc - 21.28661).abs() < 5e-6 && (dvac - 0.2094933).abs() < 5e-8 && (mn_bare - 0.539206).abs() < 5e-7 && (gc_bare - 32.35).abs() < 5e-3 && vmin > -1e-14,
        format!(
            "(a) residual of the mean fields {}; |m_bare - (m - (3/2) G s_vac)| {}; m_bare = {}; (b) largest deviation from the constant {} (constant {}); (c) G_crit = {}, Delta_vac = {}; (d) M_N = {}, G_crit_bare = {}; (e) smallest V_CJT - V_CJT(vacuum) on the grid {}",
            sci(e_fix),
            sci(e_mb),
            f7(mb),
            sci(e_const),
            f10(const_expected),
            f7(gc),
            f7(dvac),
            f7(mn_bare),
            f7(gc_bare),
            sci(vmin)
        ),
    );
    fr.add("cjt_gcrit", format!("{:.5}", gc));
    fr.add("cjt_gbind", format!("{:.4}", g_bind(m)));
    fr.add("cjt_dvac", format!("{:.7}", dvac));
    fr.add("cjt_mq", format!("{:.7}", (m * m + dvac * dvac).sqrt()));
    fr.add("cjt_mbare_t2", format!("{:.4}", mb));
    fr.add("cjt_mbare_osc", format!("{:.7}", bare_mass(m, G_OSC)));
    fr.add("cjt_svac", format!("{:.7}", s_vac));
    fr.add("cjt_mn_no_t2", format!("{:.6}", mn));
    fr.add("cjt_depth_po", format!("{:.4e}", depth_po));
    fr.add("cjt_depth_po_abs", format!("{:.4e}", -depth_po));
    fr.add("cjt_depth_cjt_abs", format!("{:.4e}", -depth_cjt));
    fr.add("cjt_depth_cjt", format!("{:.4e}", depth_cjt));
    fr.add("cjt_nlam", format!("{:.7}", nlam()));
    fr.add("cjt_mub_t2", format!("{:.5}", -1.5 * g * nlam()));
    // the table and the data of the vacuum against the coupling, in the three readings
    let mut csv = Csv::new(
        "the vacuum against the coupling G for the mass 0.2: the Dirac mass and the gap of the two-loop vacuum when 0.2 is the bare mass (interaction not ordered), when 0.2 is the physical mass (interaction normal-ordered with respect to the vacuum; then also the bare mass), and the gap of the pairing-only theory; the depth of the condensed vacuum below the normal stationary state",
        &[("program", "t2_cjt (test2/rust/t2cjt), check CJ-6".to_string()), ("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA))],
        "G,mass_unordered,gap_unordered,bare_mass_ordered,mass_ordered,gap_ordered,gap_pairing_only,normal_mass_ordered,depth_ordered,depth_pairing_only",
    );
    let row_of = |gg: f64| -> Vec<String> {
        let mn_u = mass_normal(m, gg);
        let (m_u, d_u) = match gap_of(4.0 * m, gg) {
            Some(d) => (4.0 * m, d),
            None => (mn_u, 0.0),
        };
        let mbb = bare_mass(m, gg);
        let d_o = gap_of(m, gg).unwrap_or(0.0);
        let mn_o = mass_normal(mbb, gg);
        let dep_o = v_closed(mbb, gg, m, d_o) - v_closed(mbb, gg, mn_o, 0.0);
        let dep_p = d_o * d_o / gg - 2.0 * (xi0(m * m + d_o * d_o) - xi0(m * m));
        vec![format!("{}", gg), f7(m_u), f7(d_u), f7(mbb), f7(m), f7(d_o), f7(d_o), f7(mn_o), format!("{:.6e}", dep_o), format!("{:.6e}", dep_p)]
    };
    let mut gg = 10.0;
    while gg <= 40.0 + 1e-9 {
        csv.row(&row_of(gg));
        gg += 0.125;
    }
    csv.write(dir, "t2_cjt_vacuum.csv");
    let mut table = String::from(
        "| $G_{\\rm LL}$ | not ordered: $M^{\\rm D}_{\\rm LL}$ | not ordered: $\\Delta_{\\rm LL}$ | normal-ordered: $m^{\\rm b}_{\\rm LL}$ | normal-ordered: $M^{\\rm D}_{\\rm LL}$ | normal-ordered: $\\Delta_{\\rm LL}$ | pairing only: $\\Delta_{\\rm LL}$ |\n|-------|----------|----------|----------|----------|----------|----------|\n",
    );
    for &gg in &[12.0, 15.0, 18.0, 20.0, 21.0, 22.5, 25.0, 30.0, 33.0, 36.0, 40.0] {
        let r = row_of(gg);
        table.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} |\n", r[0], &r[1][..6], &r[2][..6], &r[3][..6], &r[4][..6], &r[5][..6], &r[6][..6]));
    }
    fr.add("cjt_table_vacuum", table);
}

// ------------------------------------------------------------------ CJ-7: the SFBS boson of the normal vacuum

fn cj7(ch: &mut Checks, fr: &mut Frags, dir: &PathBuf) {
    let gr = Grid::axis(64);
    let nl = gr.nlam();
    let (m, g) = (M_T2, G_OSC);
    let mb = bare_mass(m, g);
    let md = Model::new(g, false);
    let mu_b = -1.5 * g * nl;
    let (nsc, pp) = vacuum_fields(&md, mb, mu_b, m, 0.0);
    // the operators as matrices of 16 rows: O_R = (Pi + Pi^dag)/2 -> [[0, K^dag], [K, 0]], O_I = (Pi - Pi^dag)/(2 i) ->
    // [[0, i K^dag], [-i K, 0]], Psi^dag T Psi -> [[T, 0], [0, -T^T]]
    let z8 = CMat::zeros(8);
    let mut ops: Vec<CMat> = Vec::new();
    for i in 0..3 {
        ops.push(CMat::blocks(&z8, &md.kds[i], &md.ks[i], &z8));
    }
    for i in 0..3 {
        ops.push(CMat::blocks(&z8, &md.kds[i].scale(C64::I), &md.ks[i].scale(c(0.0, -1.0)), &z8));
    }
    for (_, _, t) in channels() {
        ops.push(CMat::blocks(&t, &z8, &z8, &t.transpose().scale_r(-1.0)));
    }
    let nop = ops.len();
    // chi[A][B](w) = (1/2) int sum_{n: e_n < 0} sum_{p: e_p > 0} ( A_np B_pn/(w - (e_p - e_n)) - B_np A_pn/(w + (e_p - e_n)) )
    let freqs = [0.0, 0.25, 0.39];
    let mut chi = vec![vec![vec![C64::ZERO; nop]; 6]; freqs.len()];
    for (q, &w) in gr.q.iter().zip(&gr.w) {
        let (e, v) = eigh(&bdg(&md, q, mb, mu_b, &nsc, &pp));
        let vd = v.dagger();
        let mats: Vec<CMat> = ops.iter().map(|o| vd.mul(o).mul(&v)).collect();
        for n in 0..16 {
            if e[n] >= 0.0 {
                continue;
            }
            for p in 0..16 {
                if e[p] <= 0.0 {
                    continue;
                }
                let de = e[p] - e[n];
                for a in 0..6 {
                    let (a_np, a_pn) = (mats[a].get(n, p), mats[a].get(p, n));
                    for b in 0..nop {
                        let (b_np, b_pn) = (mats[b].get(n, p), mats[b].get(p, n));
                        for (k, &om) in freqs.iter().enumerate() {
                            chi[k][a][b] += (a_np * b_pn).scale(0.5 * w / (om - de)) - (b_np * a_pn).scale(0.5 * w / (om + de));
                        }
                    }
                }
            }
        }
    }
    let (mut e_diag, mut e_off, mut e_mix, mut e_quad) = (0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for (k, &om) in freqs.iter().enumerate() {
        let x = chi_pr(om * om, m);
        e_quad = e_quad.max((x - chi_pr_quad(om * om, m)).abs());
        for a in 0..6 {
            for b in 0..nop {
                let z = chi[k][a][b];
                if a == b {
                    e_diag = e_diag.max((z - c(-2.0 * x, 0.0)).abs());
                } else if b < 6 {
                    e_off = e_off.max(z.abs());
                } else {
                    e_mix = e_mix.max(z.abs());
                }
            }
        }
    }
    e_quad = e_quad.max((chi_pr(-0.09, m) - chi_pr_quad(-0.09, m)).abs());
    // the pole: 1 - U chi = 0 with U = -G/2, that is G chi_pr(M_B^2) = 1; limits
    let mbos = boson_mass(g, m).expect("bound state");
    let gb = g_bind(m);
    let gc = g_crit(m);
    let e_pole = (g * chi_pr(mbos * mbos, m) - 1.0).abs();
    let e_lim0 = (chi_pr(0.0, m) - 2.0 * xi1(m * m)).abs();
    let near_bd = boson_mass(gb * (1.0 + 1e-9), m).unwrap_or(f64::NAN);
    let near_cr = boson_mass(gc * (1.0 - 1e-9), m).unwrap_or(f64::NAN);
    ch.add(
        "CJ-7",
        "the SFBS boson of the normal vacuum (m = 0.2, G = 20, interaction normal-ordered; momentum grid along the axis): the response of the pair operators (Pi_iota + Pi_iota^dag)/2 and (Pi_iota - Pi_iota^dag)/(2 i) to fields coupled to the same six operators, computed from the eigenvectors of the matrices of 16 rows at the frequencies 0, 0.25 and 0.39, is -2 chi_pr times the unit matrix, with chi_pr = int 8 eps/(4 eps^2 - omega_B^2) in its closed form; the response of the 64 normal operators Psi_LL^dag T_ch Psi_LL to these fields vanishes (no mixing in the normal vacuum); the closed form of chi_pr agrees with a quadrature, also at an imaginary frequency; chi_pr(0) = 2 Xi_1(m^2) = 1/G_crit; the pole condition G chi_pr(M_B^2) = 1 has a root below the threshold 2 m, which tends to 2 m at the binding coupling G_bind and to 0 at the critical coupling",
        e_diag < 1e-11 && e_off < 1e-11 && e_mix < 1e-11 && e_quad < 1e-11 && e_pole < 1e-12 && e_lim0 < 1e-14 && (near_bd - 2.0 * m).abs() < 1e-3 && near_cr < 1e-3,
        format!(
            "largest deviation of the diagonal from -2 chi_pr {}; other pair entries {}; mixing with the normal channels {}; closed form against quadrature {}; M_B(G = 20) = {} with |G chi_pr - 1| = {}; G_bind = {}, G_crit = {}; M_B just above G_bind {}, just below G_crit {}",
            sci(e_diag),
            sci(e_off),
            sci(e_mix),
            sci(e_quad),
            f7(mbos),
            sci(e_pole),
            f7(gb),
            f7(gc),
            f7(near_bd),
            sci(near_cr)
        ),
    );
    fr.add("cjt_mb_osc", format!("{:.6}", mbos));
    // the data and the table of the boson mass, for the two readings
    let mut csv = Csv::new(
        "the mass of the SFBS boson of the normal vacuum against the coupling G for the mass 0.2: with the interaction normal-ordered (the Dirac mass of the vacuum is 0.2; identical to the pairing-only theory) and not ordered (0.2 is the bare mass and the Dirac mass of the vacuum depends on G); rows exist only where the boson is bound",
        &[("program", "t2_cjt (test2/rust/t2cjt), check CJ-7".to_string()), ("m", format!("{}", m)), ("Lambda", format!("{}", LAMBDA))],
        "reading,G,dirac_mass,pair_threshold,boson_mass",
    );
    let mut gg = 12.0;
    while gg <= 34.0 {
        if let Some(x) = boson_mass(gg, m) {
            csv.row(&["normal-ordered".to_string(), format!("{:.3}", gg), f7(m), f7(2.0 * m), f7(x)]);
        }
        gg += 0.025;
    }
    // not ordered: the Dirac mass of the normal vacuum is M_N(G); binding coupling: the root of G chi_pr((2 M_N)^2; M_N) = 1
    let (mut lo, mut hi) = (5.0, g_crit(4.0 * m));
    for _ in 0..100 {
        let mid = 0.5 * (lo + hi);
        let mn = mass_normal(m, mid);
        if mid * chi_pr(4.0 * mn * mn, mn) < 1.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let gb_u = 0.5 * (lo + hi);
    let mut gg = 12.0;
    while gg <= 34.0 {
        let mn = mass_normal(m, gg);
        if let Some(x) = boson_mass(gg, mn) {
            csv.row(&["not ordered".to_string(), format!("{:.3}", gg), f7(mn), f7(2.0 * mn), f7(x)]);
        }
        gg += 0.025;
    }
    csv.write(dir, "t2_cjt_boson.csv");
    fr.add("cjt_gbind_bare", format!("{:.3}", gb_u));
    // the boson of the interaction that is not ordered, at the coupling of run 1
    let mn_osc = mass_normal(m, g);
    fr.add("cjt_mn_bare_osc", format!("{:.4}", mn_osc));
    fr.add("cjt_mb_bare_osc", format!("{:.6}", boson_mass(g, mn_osc).unwrap_or(f64::NAN)));
    let mut table = String::from("| $G_{\\rm LL}$ | $M^{\\rm B}_{\\rm LL}$ | $2m_{\\rm LL}-M^{\\rm B}_{\\rm LL}$ |\n|-------|----------|----------|\n");
    for &gg in &[18.0, 19.0, 20.0, 20.5, 21.0, 21.2] {
        if let Some(x) = boson_mass(gg, m) {
            table.push_str(&format!("| {} | {:.6} | {:.6} |\n", gg, x, 2.0 * m - x));
        }
    }
    fr.add("cjt_table_boson", table);
}

// ------------------------------------------------------------------ CJ-8: the creation in real time (CVODE)

struct TdOut {
    t: Vec<f64>,
    pi3: Vec<C64>,
    mass: Vec<f64>,
    edev: f64,
    nsteps: i64,
    nfe: i64,
}

/// the correlation matrices of the occupied vectors: y holds, for every momentum, 8 vectors of 16 components
fn td_moments(y: &[C64], w: &[f64]) -> (CMat, CMat) {
    let mut an = [[C64::ZERO; 8]; 8];
    let mut ap = [[C64::ZERO; 8]; 8];
    for (i, &wi) in w.iter().enumerate() {
        for k in 0..8 {
            let v = &y[(i * 8 + k) * 16..(i * 8 + k + 1) * 16];
            for a in 0..8 {
                let va = v[a].scale(wi);
                for b in 0..8 {
                    an[a][b] += va * v[b].conj();
                    ap[a][b] -= va * v[8 + b].conj();
                }
            }
        }
    }
    let mut cn = CMat::zeros(8);
    let mut cp = CMat::zeros(8);
    for a in 0..8 {
        for b in 0..8 {
            cn.set(a, b, an[a][b]);
            cp.set(a, b, ap[a][b]);
        }
    }
    (cn, cp)
}

/// The time-dependent mean-field equations i dv/dt = H_var(q)[C_n, C_p] v for the occupied vectors v of every
/// momentum, integrated by CVODE from the ground state of the mean fields (nsc0, pp0).
fn evolve(md: &Model, gr: &Grid, m: f64, mu: f64, nsc0: &CMat, pp0: &CMat, t_end: f64, nout: usize) -> TdOut {
    let nq = gr.w.len();
    // the matrices of 16 rows without mean fields, and the kinetic matrices
    let z8 = CMat::zeros(8);
    let hfr: Vec<Vec<C64>> = gr.q.iter().map(|q| bdg(md, q, m, mu, &z8, &z8).a).collect();
    let h0s: Vec<CMat> = gr.q.iter().map(|q| md.h0(q, m)).collect();
    let mut y0 = vec![C64::ZERO; nq * 8 * 16];
    for (i, q) in gr.q.iter().enumerate() {
        let (e, v) = eigh(&bdg(md, q, m, mu, nsc0, pp0));
        let neg: Vec<usize> = (0..16).filter(|&k| e[k] < 0.0).collect();
        assert_eq!(neg.len(), 8, "eight occupied vectors per momentum");
        for (k, &col) in neg.iter().enumerate() {
            for r in 0..16 {
                y0[(i * 8 + k) * 16 + r] = v.get(r, col);
            }
        }
    }
    let (md_c, w_c, hfr_c) = (md.clone(), gr.w.clone(), hfr.clone());
    let f: ComplexRhs = Box::new(move |_t, y, dy| {
        let (cn, cp) = td_moments(y, &w_c);
        let (nsc, pp) = md_c.fields(&cn, &cp);
        let sig = CMat::blocks(&nsc, &pp, &pp.dagger(), &nsc.transpose().scale_r(-1.0));
        for i in 0..w_c.len() {
            let hf = &hfr_c[i];
            for k in 0..8 {
                let off = (i * 8 + k) * 16;
                for r in 0..16 {
                    let mut s = C64::ZERO;
                    for cc in 0..16 {
                        s += (hf[r * 16 + cc] + sig.a[r * 16 + cc]) * y[off + cc];
                    }
                    dy[off + r] = s * c(0.0, -1.0);
                }
            }
        }
    });
    let en = |y: &[C64]| -> (f64, C64, f64) {
        let (cn, cp) = td_moments(y, &gr.w);
        let mut ekin = 0.0;
        for i in 0..nq {
            for k in 0..8 {
                let v = &y[(i * 8 + k) * 16..(i * 8 + k) * 16 + 8];
                for a in 0..8 {
                    for b in 0..8 {
                        ekin += gr.w[i] * (v[a].conj() * h0s[i].get(a, b) * v[b]).re;
                    }
                }
            }
        }
        let e = ekin - mu * cn.trace().re + md.e_pair(&cp) + md.e_normal(&cn);
        let (nsc, _) = md.fields(&cn, &cp);
        (e, md.pairs(&cp)[2], m + tr_prod(&md.beta8, &nsc).re / 8.0)
    };
    let (e0, _, _) = en(&y0);
    let touts = cvode::uniform_times(0.0, t_end / nout as f64, nout);
    let mut out = TdOut { t: Vec::new(), pi3: Vec::new(), mass: Vec::new(), edev: 0.0, nsteps: 0, nfe: 0 };
    let st = cvode::integrate(&y0, 0.0, &touts, &CvodeSettings::adams(1e-13, 1e-15), f, |_k, t, y| {
        let (e, p3, ms) = en(y);
        out.t.push(t);
        out.pi3.push(p3);
        out.mass.push(ms);
        out.edev = out.edev.max((e - e0).abs());
    })
    .expect("CVODE integration of the time-dependent mean-field equations");
    out.nsteps = st.nsteps;
    out.nfe = st.nfevals;
    out
}

/// the frequency at which the Hann-windowed Fourier sum of a real series is largest, between lo and hi
fn peak_frequency(t: &[f64], x: &[f64], lo: f64, hi: f64) -> f64 {
    let n = t.len();
    let mean = x.iter().sum::<f64>() / n as f64;
    let (t0, tt) = (t[0], t[n - 1] - t[0]);
    let power = |om: f64| -> f64 {
        let (mut re, mut im) = (0.0, 0.0);
        for k in 0..n {
            let win = 0.5 * (1.0 - (2.0 * PI * (t[k] - t0) / tt).cos());
            re += win * (x[k] - mean) * (om * t[k]).cos();
            im += win * (x[k] - mean) * (om * t[k]).sin();
        }
        re * re + im * im
    };
    let ns = 2000;
    let step = (hi - lo) / ns as f64;
    let mut best = (0.0, lo);
    for s in 0..=ns {
        let om = lo + step * s as f64;
        let p = power(om);
        if p > best.0 {
            best = (p, om);
        }
    }
    // golden-section search of the maximum in the bracket of the best scan point
    let gr = 0.5 * (5.0f64.sqrt() - 1.0);
    let (mut a, mut b) = (best.1 - step, best.1 + step);
    let (mut x1, mut x2) = (b - gr * (b - a), a + gr * (b - a));
    let (mut p1, mut p2) = (power(x1), power(x2));
    for _ in 0..60 {
        if p1 > p2 {
            b = x2;
            x2 = x1;
            p2 = p1;
            x1 = b - gr * (b - a);
            p1 = power(x1);
        } else {
            a = x1;
            x1 = x2;
            p1 = p2;
            x2 = a + gr * (b - a);
            p2 = power(x2);
        }
    }
    0.5 * (a + b)
}

fn cj8(ch: &mut Checks, fr: &mut Frags, dir: &PathBuf) {
    let gr = Grid::axis(64);
    let nl = gr.nlam();
    let m = M_T2;
    // run 1: the normal vacuum at G = 20 with a small pair seed: the pair amplitude oscillates with the frequency M_B
    let g1 = G_OSC;
    let md1 = Model::new(g1, false);
    let (mb1, mu1) = (bare_mass(m, g1), -1.5 * g1 * nl);
    let seed1 = 1e-4;
    let (nsc, _) = vacuum_fields(&md1, mb1, mu1, m, 0.0);
    let run1 = evolve(&md1, &gr, mb1, mu1, &nsc, &md1.kds[2].scale_r(seed1), 600.0, 2400);
    let x1: Vec<f64> = run1.pi3.iter().map(|z| z.re).collect();
    let mbos = boson_mass(g1, m).expect("bound state");
    let om1 = peak_frequency(&run1.t, &x1, 0.6 * mbos, 1.4 * mbos);
    let im1 = run1.pi3.iter().fold(0.0f64, |a, z| a.max(z.im.abs()));
    let dm1 = run1.mass.iter().fold(0.0f64, |a, x| a.max((x - m).abs()));
    // run 2: the normal stationary state at G = 22.5 (above the critical coupling) with a smaller seed: the pair
    // amplitude grows exponentially with the rate gamma_gr, then saturates
    let g2 = G_T2;
    let md2 = Model::new(g2, false);
    let (mb2, mu2) = (bare_mass(m, g2), -1.5 * g2 * nl);
    let mn2 = mass_normal(mb2, g2);
    let seed2 = 1e-8;
    let (nsc, _) = vacuum_fields(&md2, mb2, mu2, mn2, 0.0);
    let run2 = evolve(&md2, &gr, mb2, mu2, &nsc, &md2.kds[2].scale_r(seed2), 240.0, 1920);
    let gam = growth_rate(g2, mn2).expect("unstable normal state");
    let dvac = gap_of(m, g2).unwrap();
    let pi_sat = 2.0 * dvac / g2;
    let a0 = run2.pi3[0].abs();
    // least-squares slope of ln |<Pi_3>| where 300 times the initial value < |<Pi_3>| < 0.005 times the vacuum value
    let (mut sx, mut sy, mut sxx, mut sxy, mut cnt) = (0.0, 0.0, 0.0, 0.0, 0.0);
    for (t, z) in run2.t.iter().zip(&run2.pi3) {
        let a = z.abs();
        if a > 300.0 * a0 && a < 0.005 * pi_sat {
            let l = a.ln();
            sx += t;
            sy += l;
            sxx += t * t;
            sxy += t * l;
            cnt += 1.0;
        } else if a >= 0.005 * pi_sat {
            break;
        }
    }
    let gam_fit = (cnt * sxy - sx * sy) / (cnt * sxx - sx * sx);
    let pmax = run2.pi3.iter().fold(0.0f64, |a, z| a.max(z.abs()));
    // the mean of |<Pi_3>| over the last third of the run, compared with the vacuum value
    let n3 = run2.t.len() / 3;
    let late: f64 = run2.pi3[2 * n3..].iter().map(|z| z.abs()).sum::<f64>() / (run2.pi3.len() - 2 * n3) as f64;
    ch.add(
        "CJ-8",
        "the creation of the SFBS in real time: the time-dependent mean-field equations of the two-loop action, i dv/dt = H_var(q) v for the occupied vectors of every momentum with the mean fields of the current state, integrated by CVODE (Adams-Moulton formulas, rtol 1e-13, atol 1e-15; m = 0.2, interaction normal-ordered, 128 momenta along the axis). Run 1, G = 20 (between G_bind and G_crit), start: the normal vacuum with the pair seed 1e-4 Kch_3^dag in its mean-field matrix: <Pi_3> stays real and oscillates, and the frequency of the largest line of its Hann-windowed Fourier sum equals the boson mass M_B of the pole condition; the Dirac mass stays at m to the second order in the seed. Run 2, G = 22.5 (above G_crit), start: the normal stationary state with the seed 1e-8: |<Pi_3>| grows exponentially with the rate gamma_gr, the root of G chi_pr(-gamma_gr^2) = 1 for the Dirac mass of that state; the growth ends in oscillations about the condensed vacuum, in which |<Pi_3>| reaches between one half and twice the value 2 Delta_vac/G of the condensed vacuum (the equations conserve the energy, so that the state cannot come to rest in the vacuum). Both integrations conserve the energy E_var to better than 1e-9",
        ((om1 - mbos) / mbos).abs() < 1e-3 && im1 < 1e-9 && dm1 < 1e-6 && ((gam_fit - gam) / gam).abs() < 5e-3 && cnt >= 20.0 && pmax > 0.5 * pi_sat && pmax < 2.0 * pi_sat && run1.edev < 1e-9 && run2.edev < 1e-9,
        format!(
            "run 1: frequency {} against M_B = {} (relative difference {}); largest |Im <Pi_3>| {}; largest |Dirac mass - m| {}; change of the energy {}; {} steps, {} evaluations of the right side. Run 2: fitted rate {} over {} output times against gamma_gr = {} (relative difference {}); largest |<Pi_3>| {} and mean over the last third {} against 2 Delta_vac/G = {}; change of the energy {}; {} steps, {} evaluations",
            f7(om1),
            f7(mbos),
            sci(((om1 - mbos) / mbos).abs()),
            sci(im1),
            sci(dm1),
            sci(run1.edev),
            run1.nsteps,
            run1.nfe,
            f7(gam_fit),
            cnt,
            f7(gam),
            sci(((gam_fit - gam) / gam).abs()),
            f7(pmax),
            f7(late),
            f7(pi_sat),
            sci(run2.edev),
            run2.nsteps,
            run2.nfe
        ),
    );
    let mut csv = Csv::new(
        "the pair amplitude <Pi_3> and the Dirac mass against the time, from the integration of the time-dependent mean-field equations of the two-loop action by CVODE: run 'oscillation' (G = 20, normal vacuum with a pair seed) and run 'growth' (G = 22.5, normal stationary state with a pair seed)",
        &[
            ("program", "t2_cjt (test2/rust/t2cjt), check CJ-8".to_string()),
            ("m", format!("{}", m)),
            ("Lambda", format!("{}", LAMBDA)),
            ("G_oscillation", format!("{}", g1)),
            ("seed_oscillation", format!("{:e}", seed1)),
            ("boson_mass_predicted", f10(mbos)),
            ("frequency_measured", f10(om1)),
            ("G_growth", format!("{}", g2)),
            ("seed_growth", format!("{:e}", seed2)),
            ("growth_rate_predicted", f10(gam)),
            ("growth_rate_fitted", f10(gam_fit)),
            ("pair_amplitude_of_the_vacuum", f10(pi_sat)),
            ("normal_mass_growth", f10(mn2)),
        ],
        "time_integration,time,pair_re,pair_im,dirac_mass",
    );
    for (name, run) in [("oscillation", &run1), ("growth", &run2)] {
        for k in 0..run.t.len() {
            csv.row(&[name.to_string(), format!("{:.4}", run.t[k]), format!("{:.12e}", run.pi3[k].re), format!("{:.12e}", run.pi3[k].im), format!("{:.10}", run.mass[k])]);
        }
    }
    csv.write(dir, "t2_cjt_time.csv");
    fr.add("cjt_om_measured", format!("{:.6}", om1));
    fr.add("cjt_om_reldev", sci(((om1 - mbos) / mbos).abs()));
    fr.add("cjt_gamma", format!("{:.6}", gam));
    fr.add("cjt_gamma_fit", format!("{:.6}", gam_fit));
    fr.add("cjt_gamma_reldev", sci(((gam_fit - gam) / gam).abs()));
    fr.add("cjt_pi_sat", format!("{:.6}", pi_sat));
    fr.add("cjt_pi_max", format!("{:.6}", pmax));
    fr.add("cjt_pi_late", format!("{:.6}", late));
    fr.add("cjt_td_steps", format!("{} and {}", run1.nsteps, run2.nsteps));
}

// ------------------------------------------------------------------ CJ-9: general solutions; one site

fn cj9(ch: &mut Checks, fr: &mut Frags, dir: &PathBuf) {
    // (a) general solutions of the self-consistency equations from random starts, without any symmetry imposed
    let gr = Grid::product(48, 2, 4);
    let nl = gr.nlam();
    let (m, g) = (M_T2, G_T2);
    let mb = bare_mass(m, g);
    let md = Model::new(g, false);
    let mu_b = -1.5 * g * nl;
    let dvac = gap_of(m, g).unwrap();
    let e_vac = v_closed(mb, g, m, dvac);
    let mut rng = Rng(99);
    let mut rows = Vec::new();
    let (mut all_conv, mut none_below, mut n_vac, mut inv_ok) = (true, true, 0, true);
    let mut table = String::from(
        "| start | iterations | residual | $\\mathcal E^{\\rm var}_{\\rm LL}$ minus vacuum | $\\lvert\\vec d_{\\rm LL}\\rvert$ | $\\zeta_{\\rm LL}$ | $M^{\\rm D}_{\\rm LL}$ | reached |\n|-----|---------|--------|-----------|---------|--------|---------|--------------|\n",
    );
    for s in 0..4 {
        let nsc0 = md.beta8.scale_r(0.3 * rng.unit()).add(&rand_herm(&mut rng, 8, 0.03)).add(&CMat::identity(8).scale_r(mu_b));
        let mut pp0 = CMat::zeros(8);
        for i in 0..3 {
            pp0 = pp0.add(&md.kds[i].scale(c(0.25 * rng.unit(), 0.25 * rng.unit())));
        }
        let sol = hfb_solve(&md, &gr, mb, mu_b, &nsc0, &pp0, 1e-12, 1500);
        let d = dvec(&md, &sol.pp);
        let (d2, zeta) = d_invariants(&d);
        let mdir = mb + tr_prod(&md.beta8, &sol.nsc).re / 8.0;
        let de = sol.e - e_vac;
        all_conv &= sol.res < 1e-9;
        none_below &= de > -1e-10;
        let is_vac = de.abs() < 1e-9;
        if is_vac {
            n_vac += 1;
            inv_ok &= (d2.sqrt() - dvac).abs() < 1e-6 && zeta < 1e-5 && (mdir - m).abs() < 1e-6;
        }
        let reached = if is_vac { "the condensed vacuum" } else { "another stationary state" };
        rows.push(format!("start {}: {} iterations, residual {}, E - E_vacuum = {:.3e}, |d| = {}, zeta = {}, M_D = {} ({})", s + 1, sol.it, sci(sol.res), de, f7(d2.sqrt()), sci(zeta), f7(mdir), reached));
        table.push_str(&format!("| {} | {} | {} | {} | {} | {} | {} | {} |\n", s + 1, sol.it, sci(sol.res), sci(de), f7(d2.sqrt()), sci(zeta), f7(mdir), reached));
    }
    ch.add(
        "CJ-9",
        "general solutions of the two-loop equations without imposed symmetry (m = 0.2, G = 22.5, interaction normal-ordered; 384 momenta in a product grid): from four random starts (random Hermitian N_sc, random complex order parameter) the Anderson iteration of the self-consistency map of the full matrices of 16 rows converges; no solution has an energy below that of the vacuum of the closed form; and the solutions that reach this energy have |dvec| = Delta_vac, zeta = 0 and the Dirac mass m (they are the vacuum (0, 0, Delta_vac) turned by a symmetry transformation)",
        all_conv && none_below && n_vac >= 1 && inv_ok,
        rows.join("; "),
    );
    fr.add("cjt_table_starts", table);

    // (b) one site: the functional is the expectation value of the Hamiltonian in the mean-field state, and it lies
    // above the exact ground energy
    let site = Grid::site();
    let mdl1 = Model::new(1.0, false);
    let pis: Vec<CMat> = mdl1.ks.iter().map(fock_pair).collect();
    let mut csv = Csv::new(
        "one site (8 fermion modes, 256 states), mass 0.2, chemical potential -(3/2) G: the exact ground energy of the Hamiltonian c^dag (m beta - mu) c - (G/4) sum Pi^dag Pi and the lowest two-loop energy E_var found (the energy of the best quasi-free state)",
        &[("program", "t2_cjt (test2/rust/t2cjt), check CJ-9".to_string()), ("m", format!("{}", M_T2))],
        "G,energy_exact,energy_two_loop,energy_difference,order_parameter_two_loop",
    );
    let (mut e_expect, mut worst, mut res_max) = (0.0f64, f64::INFINITY, 0.0f64);
    let mut table = String::from("| $G_{\\rm LL}$ | exact ground energy $E^{\\rm ex}_{\\rm LL}$ | two-loop energy $\\mathcal E^{\\rm var}_{\\rm LL}$ | difference | $\\lvert\\vec d_{\\rm LL}\\rvert$ of the two-loop state |\n|------|-----------|-----------|-----------|-----------|\n");
    let mut rng = Rng(7);
    for &gs in &[0.1, 0.2, 0.3, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0] {
        let mds = Model::new(gs, false);
        let mu = -1.5 * gs;
        let ham = fock_site_hamiltonian(&mds, M_T2, mu, &pis);
        let (ev, _) = lowest(&ham);
        // the best of three starts: normal, paired, random
        let mut best: Option<Hfb> = None;
        for s in 0..3 {
            let nsc0 = CMat::identity(8).scale_r(mu).add(&if s == 2 { rand_herm(&mut rng, 8, 0.1) } else { CMat::zeros(8) });
            let pp0 = match s {
                0 => CMat::zeros(8),
                1 => mds.kds[2].scale_r(0.5 * gs),
                _ => mds.kds[2].scale(c(0.3 * gs, 0.2 * gs)).add(&mds.kds[0].scale_r(0.1 * gs)),
            };
            let sol = hfb_solve(&mds, &site, M_T2, mu, &nsc0, &pp0, 1e-13, 4000);
            if sol.res < 1e-9 && best.as_ref().map_or(true, |b| sol.e < b.e) {
                best = Some(sol);
            }
        }
        let sol = best.expect("a converged solution on one site");
        res_max = res_max.max(sol.res);
        // the mean-field state in the Fock space, and the expectation value of the Hamiltonian in it
        let hq = mds.beta8.scale_r(M_T2).add(&sol.nsc).sub(&CMat::identity(8).scale_r(mu));
        let (_, phi) = lowest(&fock_quadratic(&hq, &sol.pp));
        let expect = fock::dot(&phi, &ham.matvec(&phi)).re;
        e_expect = e_expect.max((expect - sol.e).abs());
        worst = worst.min(sol.e - ev[0]);
        let (d2, _) = d_invariants(&dvec(&mds, &sol.pp));
        csv.row(&[format!("{}", gs), f10(ev[0]), f10(sol.e), f10(sol.e - ev[0]), f10(d2.sqrt())]);
        table.push_str(&format!("| {} | {:.6} | {:.6} | {:.6} | {:.6} |\n", gs, ev[0], sol.e, sol.e - ev[0], d2.sqrt()));
    }
    csv.write(dir, "t2_cjt_site.csv");
    ch.add(
        "CJ-10",
        "the two-loop action is a variational energy, tested on one site (8 fermion modes, 256 states; m = 0.2, chemical potential -(3/2) G, ten couplings from 0.1 to 4): the energy E_var of the self-consistent mean-field state equals the expectation value of the Hamiltonian c^dag (m beta - mu) c - (G/4) sum_iota Pi_iota^dag Pi_iota in the ground state of the mean-field Hamiltonian, constructed in the Fock space; and it is never below the exact ground energy (the lowest eigenvalue of the 256 x 256 matrix)",
        e_expect < 1e-10 && worst > -1e-10 && res_max < 1e-9,
        format!("largest |E_var - expectation value| {}; smallest E_var - exact ground energy {:.6}; largest residual of the mean fields {}", sci(e_expect), worst, sci(res_max)),
    );
    fr.add("cjt_table_site", table);
}

// ------------------------------------------------------------------ CJ-11: the axially symmetric completion

fn cj11(ch: &mut Checks, cs: &[[f64; 2]; 8], cps: &[[f64; 2]; 8]) {
    // the classes that flip the chirality: beta, i beta gamma_ch, i beta alpha^j, beta gamma_ch alpha^j
    let flip = [1usize, 2, 6, 7];
    let mut e_sum = 0.0f64;
    for &cl in &flip {
        for k in 0..2 {
            e_sum = e_sum.max((cs[cl][k] + cps[cl][k]).abs());
        }
    }
    let keep_ok = [0usize, 3, 4, 5].iter().all(|&cl| (cs[cl][0] + cps[cl][0]).abs() == 6.0 && (cs[cl][1] + cps[cl][1]).abs() == 2.0);
    // no chirality-flipping component of the mean field of a random state
    let (m, g) = (M_T2, G_T2);
    let md = Model::new(g, true);
    let mut rng = Rng(2026);
    let cn = rand_herm(&mut rng, 8, 0.4);
    let (nsc, _) = md.fields(&cn, &CMat::zeros(8));
    let mut e_flip = 0.0f64;
    for (cl, _, t) in channels() {
        if flip.contains(&cl) {
            e_flip = e_flip.max(tr_prod(&t, &nsc).abs() / 8.0);
        }
    }
    // the condensed vacuum with the Dirac mass m (no dressing) and the pairing-only gap is a stationary state, at
    // the chemical potential -3 G N_Lambda
    let gr = Grid::product(64, 4, 6);
    let nl = gr.nlam();
    let mu = -3.0 * g * nl;
    let dvac = gap_of(m, g).unwrap();
    let (nsc0, pp0) = vacuum_fields(&md, m, mu, m, dvac);
    let st = ground(&md, &gr, m, mu, &nsc0, &pp0);
    let (n2, p2) = md.fields(&st.cn, &st.cp);
    let e_fix = n2.sub(&nsc0).max_abs().max(p2.sub(&pp0).max_abs());
    let pi = md.pairs(&st.cp);
    let e_ps = pi[3].abs().max(pi[4].abs()).max(pi[5].abs());
    // a general solution from a start with scalar and pseudoscalar pair fields
    let grs = Grid::product(48, 2, 4);
    let mus = -3.0 * g * grs.nlam();
    let nsc1 = rand_herm(&mut rng, 8, 0.03).add(&CMat::identity(8).scale_r(mus));
    let mut pp1 = CMat::zeros(8);
    for i in 0..6 {
        pp1 = pp1.add(&md.kds[i].scale(c(0.2 * rng.unit(), 0.2 * rng.unit())));
    }
    let sol = hfb_solve(&md, &grs, m, mus, &nsc1, &pp1, 1e-12, 1500);
    let stv = ground(&md, &grs, m, mus, &vacuum_fields(&md, m, mus, m, dvac).0, &vacuum_fields(&md, m, mus, m, dvac).1);
    let e_vac = energy(&md, &stv, mus);
    let pis = md.pairs(&sol.st.cp);
    let ps_sol = (pis[3].norm_sqr() + pis[4].norm_sqr() + pis[5].norm_sqr()).sqrt();
    let sc_sol = (pis[0].norm_sqr() + pis[1].norm_sqr() + pis[2].norm_sqr()).sqrt() * 0.5 * g;
    let mdir = m + tr_prod(&md.beta8, &sol.nsc).re / 8.0;
    ch.add(
        "CJ-11",
        "the alternative without mass dressing, the axially symmetric completion -(G/4) sum_iota (Pi_iota^dag Pi_iota + Pi'_iota^dag Pi'_iota) of the interaction (pseudoscalar pair operators Pi'_iota of the matrices Kch'_iota): the sums of the coefficients of the scalar and of the pseudoscalar pairs vanish on the four classes of channels that flip the chirality (beta, i beta gamma_ch, i beta alpha^j, beta gamma_ch alpha^j) and equal 6 (tau_0) or 2 (tau_iota) in modulus on the other four; the mean field N_sc of a random state has no component on the flipping channels, so that no state changes the Dirac mass; at the chemical potential -3 G N_Lambda the state with the Dirac mass m = 0.2 and the gap of the pairing-only gap equation (G = 22.5) is a stationary state with vanishing pseudoscalar pair amplitudes; and a general solution from a random start with scalar and pseudoscalar pair fields reaches the energy of this state, with |dvec| = Delta_vac, the Dirac mass m and no pseudoscalar pair amplitude",
        e_sum == 0.0 && keep_ok && e_flip < 1e-13 && e_fix < 1e-11 && e_ps < 1e-12 && sol.res < 1e-9 && (sol.e - e_vac).abs() < 1e-9 && ps_sol < 1e-6 && (sc_sol - dvac).abs() < 1e-6 && (mdir - m).abs() < 1e-9,
        format!(
            "largest sum on the flipping classes {}; largest flipping component of N_sc {}; residual of the mean fields of the vacuum {}; pseudoscalar amplitudes of the vacuum {}; general solution: {} iterations, residual {}, E - E_vacuum = {:.2e}, |dvec| = {}, pseudoscalar amplitude {}, Dirac mass {}",
            sci(e_sum),
            sci(e_flip),
            sci(e_fix),
            sci(e_ps),
            sol.it,
            sci(sol.res),
            sol.e - e_vac,
            f7(sc_sol),
            sci(ps_sol),
            f7(mdir)
        ),
    );
}

// ------------------------------------------------------------------ main

fn main() {
    let dir = data_dir();
    let mut ch = Checks {
        title: "Revision_SuperLoop, folder test2, program `t2_cjt`: the creation of the superfluid boson singlet state (SFBS) from the spinor field Psi_LL by the method of Cornwall, Jackiw and Tomboulis, in the two-loop approximation of the effective action of the propagator (Test 2, stage D)".to_string(),
        items: Vec::new(),
    };
    let mut fr = Frags(Vec::new());
    cj1(&mut ch);
    let (cs, cps) = cj2(&mut ch, &mut fr, &dir);
    cj3(&mut ch, &mut fr);
    cj4(&mut ch);
    cj5(&mut ch, &mut fr);
    cj6(&mut ch, &mut fr, &dir);
    cj7(&mut ch, &mut fr, &dir);
    cj8(&mut ch, &mut fr, &dir);
    cj9(&mut ch, &mut fr, &dir);
    cj11(&mut ch, &cs, &cps);
    let text = ch.text();
    fs::write(dir.join("t2_cjt_checks.txt"), &text).expect("write report");
    fr.add("cjt_report", format!("```text\n{}```", text));
    fr.write(&dir.join("t2_cjt_fragments.md"));
    println!("{}", text.lines().nth(1).unwrap_or(""));
    if !ch.all_pass() {
        std::process::exit(1);
    }
}
