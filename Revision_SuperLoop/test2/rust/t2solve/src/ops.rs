//! The lattice, the gauge fields, the pair field, the sparse lattice operator H_lat of (2.36) and the numerical tools of
//! the solver (the same formulas as test2/python/t2_solve.py).
use crate::input::Cfg;
use std::fs;
use t2core::c64::{c, C64};
use t2core::cmat::CMat;
use t2core::lattice::{h_lat_plane, link, potential, Fields, Lattice, M2};
use t2core::model;
use t2core::rng::Rng;

/// the lattice of a problem: the geometry (t2core's Lattice) and the coordinates of the sites
pub struct Lat {
    pub l: Lattice,
    pub ns: usize,
    pub active: [bool; 3],
    pub nact: usize,
    pub x: Vec<[f64; 3]>,
}

impl Lat {
    pub fn new(cfg: &Cfg) -> Lat {
        let n = cfg.v("sites");
        let n = [n[0] as usize, n[1] as usize, n[2] as usize];
        let b = cfg.w("boundary");
        let l = Lattice {
            n,
            ell: cfg.f("spacing"),
            periodic: [b[0] == "periodic", b[1] == "periodic", b[2] == "periodic"],
            wilson: cfg.f("wilson"),
            mass: cfg.f("mass"),
            mu: cfg.f("mu"),
        };
        let ns = n[0] * n[1] * n[2];
        let active = [n[0] > 1, n[1] > 1, n[2] > 1];
        let nact = active.iter().filter(|&&a| a).count();
        let x = (0..ns)
            .map(|s| {
                let cc = l.coords(s);
                [cc[0] as f64 * l.ell, cc[1] as f64 * l.ell, cc[2] as f64 * l.ell]
            })
            .collect();
        Lat { l, ns, active, nact, x }
    }
}

/// the gauge potentials A[site][p][c] (c = 1, 2, 3, F as 0..3), with the pulse
pub struct Gauge {
    pub gv: f64,
    pub gf: f64,
    pub ell: f64,
    pub a: Vec<[[f64; 4]; 4]>,
    pub pdir: usize,
    pub pamp: [f64; 4],
    pub ptc: f64,
    pub ptw: f64,
    pub pw: f64,
    pub is_static: bool,
    pub desc: String,
}

fn read_numbers(cfg: &Cfg, name: &str) -> Result<Vec<Vec<f64>>, String> {
    let p = if std::path::Path::new(name).is_absolute() { std::path::PathBuf::from(name) } else { cfg.dir.join(name) };
    let text = fs::read_to_string(&p).map_err(|e| format!("cannot read '{}': {}", p.display(), e))?;
    let mut rows = vec![];
    for line in text.lines() {
        let s = line.split('#').next().unwrap_or("").trim();
        if s.is_empty() {
            continue;
        }
        let r: Result<Vec<f64>, _> = s.split_whitespace().map(|x| x.parse::<f64>()).collect();
        rows.push(r.map_err(|_| format!("'{}': a line is not a list of numbers: {}", name, s))?);
    }
    Ok(rows)
}

impl Gauge {
    pub fn new(cfg: &Cfg, lat: &Lat) -> Result<Gauge, String> {
        let ns = lat.ns;
        let mut a = vec![[[0.0f64; 4]; 4]; ns];
        for p in 0..4 {
            let v = cfg.v(&format!("potential_{}", p));
            for s in 0..ns {
                for cc in 0..4 {
                    a[s][p][cc] = v[cc];
                }
            }
        }
        let gfile = cfg.s("gauge_file");
        if !gfile.is_empty() {
            let rows = read_numbers(cfg, &gfile)?;
            if rows.len() != ns || rows.iter().any(|r| r.len() != 16) {
                return Err(format!("the gauge file must have {} lines of 16 numbers", ns));
            }
            for s in 0..ns {
                for p in 0..4 {
                    for cc in 0..4 {
                        a[s][p][cc] += rows[s][4 * p + cc];
                    }
                }
            }
        }
        let mag = cfg.v("magnetic");
        let (ff, f3) = (mag[0], mag[1]);
        if ff != 0.0 || f3 != 0.0 {
            if lat.l.n[0] < 2 || lat.l.n[1] < 2 {
                return Err("a magnetic field needs n_x > 1 and n_y > 1".into());
            }
            if lat.l.periodic[0] {
                return Err("a magnetic field needs 'wall' in direction 1 (the Landau gauge A_2 = F x^1 is not periodic in x^1)".into());
            }
            let xc = 0.5 * (lat.l.n[0] - 1) as f64 * lat.l.ell;
            for s in 0..ns {
                a[s][2][3] += ff * (lat.x[s][0] - xc);
                a[s][2][2] += f3 * (lat.x[s][0] - xc);
            }
        }
        let pamp = cfg.v("pulse_potential");
        let pt = cfg.v("pulse_time");
        let is_static = pamp.iter().all(|&v| v == 0.0);
        let desc = format!(
            "potentials {:?}, magnetic {:?}, gauge file {}, pulse {}",
            (0..4).map(|p| cfg.v(&format!("potential_{}", p))).collect::<Vec<_>>(),
            mag,
            if gfile.is_empty() { "none".to_string() } else { gfile.clone() },
            if is_static {
                "none".to_string()
            } else {
                format!("A_{} amplitudes {:?}, t_c {}, t_w {}, w_p {}", cfg.i("pulse_direction"), pamp, pt[0], pt[1], pt[2])
            }
        );
        Ok(Gauge {
            gv: cfg.f("g_v"),
            gf: cfg.f("g_F"),
            ell: lat.l.ell,
            a,
            pdir: cfg.i("pulse_direction") as usize,
            pamp: [pamp[0], pamp[1], pamp[2], pamp[3]],
            ptc: pt[0],
            ptw: pt[1],
            pw: pt[2],
            is_static,
            desc,
        })
    }

    pub fn envelope(&self, t: f64) -> f64 {
        (-(t - self.ptc).powi(2) / (2.0 * self.ptw * self.ptw)).exp() * (self.pw * (t - self.ptc)).cos()
    }

    /// the links and the potential matrices at the time t (None: without the pulse), as t2core Fields with d = 0
    pub fn fields_at(&self, t: Option<f64>) -> Fields {
        let ns = self.a.len();
        let mut fl = Fields::trivial(ns);
        let env = match t {
            Some(tt) if !self.is_static => self.envelope(tt),
            _ => 0.0,
        };
        for s in 0..ns {
            let mut a = self.a[s];
            if env != 0.0 {
                for cc in 0..4 {
                    a[self.pdir][cc] += self.pamp[cc] * env;
                }
            }
            for j in 0..3 {
                fl.links[s][j] = link(self.gv, self.gf, &[a[j + 1][0], a[j + 1][1], a[j + 1][2]], a[j + 1][3], self.ell);
            }
            fl.v0[s] = potential(self.gv, self.gf, &[a[0][0], a[0][1], a[0][2]], a[0][3]);
        }
        fl
    }

    /// the largest Frobenius norm of V0 over the sites and the times (the triangle inequality for the pulse)
    pub fn vmax(&self) -> f64 {
        let fl = self.fields_at(None);
        let mut v = fl.v0.iter().map(|m| m.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt()).fold(0.0, f64::max);
        if !self.is_static && self.pdir == 0 {
            let vp = potential(self.gv, self.gf, &[self.pamp[0], self.pamp[1], self.pamp[2]], self.pamp[3]);
            v += vp.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        }
        v
    }

    pub fn is_free(&self) -> bool {
        self.is_static && self.a.iter().all(|m| m.iter().all(|r| r.iter().all(|&v| v == 0.0)))
    }
}

/// the prescribed pair field or the initial guess (the key 'pair')
pub fn pair_profile(cfg: &Cfg, lat: &Lat) -> Result<Vec<[C64; 3]>, String> {
    let ns = lat.ns;
    let mut d = vec![[C64::ZERO; 3]; ns];
    let kind = cfg.s("pair");
    match kind.as_str() {
        "none" => return Ok(d),
        "file" => {
            let rows = read_numbers(cfg, &cfg.s("pair_file"))?;
            if rows.len() != ns || rows.iter().any(|r| r.len() != 6) {
                return Err(format!("the pair file must have {} lines of 6 numbers", ns));
            }
            for s in 0..ns {
                for i in 0..3 {
                    d[s][i] = c(rows[s][2 * i], rows[s][2 * i + 1]);
                }
            }
            return Ok(d);
        }
        "random" => {
            // SplitMix64 of t2core: for each site and isospin direction the real, then the imaginary part
            let mut rng = Rng(cfg.i("pair_seed") as u64);
            let amp = cfg.f("pair_amplitude");
            for s in 0..ns {
                for i in 0..3 {
                    let re = rng.unit();
                    let im = rng.unit();
                    d[s][i] = c(amp * re, amp * im);
                }
            }
            return Ok(d);
        }
        _ => {}
    }
    let i0 = cfg.i("pair_direction") as usize - 1;
    let amp = C64::cis(cfg.f("pair_phase")).scale(cfg.f("pair_amplitude"));
    let l1 = lat.l.n[0] as f64 * lat.l.ell;
    let w = cfg.f("pair_width");
    let pos = cfg.v("pair_position");
    let centre = 0.5 * (lat.l.n[0] - 1) as f64 * lat.l.ell;
    for s in 0..ns {
        let x1 = lat.x[s][0];
        let prof = match kind.as_str() {
            "uniform" => 1.0,
            "kink" => ((x1 - if pos.is_empty() { centre } else { pos[0] }) / w).tanh(),
            "kink_antikink" => {
                let (xa, xb) = if pos.len() == 2 { (pos[0], pos[1]) } else { (0.25 * l1, 0.75 * l1) };
                ((x1 - xa) / w).tanh() * ((xb - x1) / w).tanh()
            }
            _ => 0.5 * (1.0 + ((x1 - if pos.is_empty() { centre } else { pos[0] }) / w).tanh()),
        };
        d[s][i0] = amp.scale(prof);
    }
    Ok(d)
}

/// the fixed 4 x 4 and 8 x 8 matrices: alpha^j, beta, Kch_iota and Kch_iota^dag (from t2core::model)
pub struct M {
    pub alpha: Vec<CMat>,
    pub beta: CMat,
    pub k: Vec<CMat>,
    pub kdag: Vec<CMat>,
}

impl M {
    pub fn new() -> M {
        let k: Vec<CMat> = (1..=3).map(model::kpair).collect();
        let kdag = k.iter().map(|m| m.dagger()).collect();
        M { alpha: (1..=3).map(model::alpha).collect(), beta: model::beta(), k, kdag }
    }
}

/// a sparse matrix in compressed rows
pub struct Csr {
    pub n: usize,
    pub rowptr: Vec<usize>,
    pub col: Vec<usize>,
    pub val: Vec<C64>,
}

fn kron4x2(a: &CMat, u: &M2) -> [[C64; 8]; 8] {
    let mut o = [[C64::ZERO; 8]; 8];
    for p in 0..4 {
        for q in 0..4 {
            let x = a.get(p, q);
            for i in 0..2 {
                for k in 0..2 {
                    o[2 * p + i][2 * q + k] = x * u[2 * i + k];
                }
            }
        }
    }
    o
}

fn m2_dagger(u: &M2) -> M2 {
    [u[0].conj(), u[2].conj(), u[1].conj(), u[3].conj()]
}

/// the 8 x 8 pair block P = sum_iota d^iota Kch_iota^dag of one site
pub fn pair_block(m: &M, d: &[C64; 3]) -> [[C64; 8]; 8] {
    let mut p = [[C64::ZERO; 8]; 8];
    for i in 0..3 {
        if d[i] == C64::ZERO {
            continue;
        }
        for a in 0..8 {
            for b in 0..8 {
                p[a][b] += d[i] * m.kdag[i].get(a, b);
            }
        }
    }
    p
}

/// the sparse lattice operator (2.36): A on the particle components, -conj(A) on the hole components, and the pair
/// blocks P, P^dag (if d is given).  Blocks between the same pair of sites are added.
pub fn build_h(lat: &Lat, fl: &Fields, m: &M, d: Option<&[[C64; 3]]>) -> Csr {
    let ns = lat.ns;
    let ell = lat.l.ell;
    let r = lat.l.wilson;
    let n = 16 * ns;
    let mut rows: Vec<Vec<(usize, C64)>> = vec![Vec::new(); n];
    let push = |rows: &mut Vec<Vec<(usize, C64)>>, s: usize, t: usize, b: &[[C64; 8]; 8], hole: bool| {
        for a in 0..8 {
            for bb in 0..8 {
                let v = b[a][bb];
                if v == C64::ZERO {
                    continue;
                }
                if hole {
                    rows[16 * s + 8 + a].push((16 * t + 8 + bb, -v.conj()));
                } else {
                    rows[16 * s + a].push((16 * t + bb, v));
                }
            }
        }
    };
    let id2: M2 = [C64::ONE, C64::ZERO, C64::ZERO, C64::ONE];
    for s in 0..ns {
        // on the site: (m + r_W D/dx) beta (x) 1_2 - 1_4 (x) (V0 + mu)
        let mut b = kron4x2(&m.beta.scale_r(lat.l.mass + r * lat.nact as f64 / ell), &id2);
        let v0 = fl.v0[s];
        for p in 0..4 {
            for i in 0..2 {
                for k in 0..2 {
                    let mu = if i == k { c(lat.l.mu, 0.0) } else { C64::ZERO };
                    b[2 * p + i][2 * p + k] -= v0[2 * i + k] + mu;
                }
            }
        }
        push(&mut rows, s, s, &b, false);
        push(&mut rows, s, s, &b, true);
        for j in 0..3 {
            if !lat.active[j] {
                continue;
            }
            let df = m.alpha[j].scale(c(0.0, -1.0)).sub(&m.beta.scale_r(r)).scale_r(0.5 / ell);
            let db = m.alpha[j].scale(c(0.0, 1.0)).sub(&m.beta.scale_r(r)).scale_r(0.5 / ell);
            if let Some(sp) = lat.l.neighbour(s, j, true) {
                let blk = kron4x2(&df, &fl.links[s][j]);
                push(&mut rows, s, sp, &blk, false);
                push(&mut rows, s, sp, &blk, true);
            }
            if let Some(sm) = lat.l.neighbour(s, j, false) {
                let blk = kron4x2(&db, &m2_dagger(&fl.links[sm][j]));
                push(&mut rows, s, sm, &blk, false);
                push(&mut rows, s, sm, &blk, true);
            }
        }
        if let Some(dd) = d {
            let p = pair_block(m, &dd[s]);
            for a in 0..8 {
                for bb in 0..8 {
                    if p[a][bb] != C64::ZERO {
                        rows[16 * s + a].push((16 * s + 8 + bb, p[a][bb]));
                        rows[16 * s + 8 + bb].push((16 * s + a, p[a][bb].conj()));
                    }
                }
            }
        }
    }
    // merge the entries of each row (sorted by column; equal columns added)
    let mut rowptr = vec![0usize; n + 1];
    let mut col = Vec::new();
    let mut val = Vec::new();
    for (i, row) in rows.iter_mut().enumerate() {
        row.sort_by_key(|e| e.0);
        let mut k = 0;
        while k < row.len() {
            let cc = row[k].0;
            let mut v = C64::ZERO;
            while k < row.len() && row[k].0 == cc {
                v += row[k].1;
                k += 1;
            }
            col.push(cc);
            val.push(v);
        }
        rowptr[i + 1] = col.len();
    }
    Csr { n, rowptr, col, val }
}

impl Csr {
    pub fn matvec(&self, x: &[C64], y: &mut [C64]) {
        for i in 0..self.n {
            let mut acc = C64::ZERO;
            for k in self.rowptr[i]..self.rowptr[i + 1] {
                acc += self.val[k] * x[self.col[k]];
            }
            y[i] = acc;
        }
    }
    /// Y = H X for the modes X stored row by row (n rows of mm columns)
    pub fn matmat(&self, x: &[C64], y: &mut [C64], mm: usize) {
        for i in 0..self.n {
            let yi = &mut y[i * mm..(i + 1) * mm];
            for v in yi.iter_mut() {
                *v = C64::ZERO;
            }
            for k in self.rowptr[i]..self.rowptr[i + 1] {
                let a = self.val[k];
                let xr = &x[self.col[k] * mm..(self.col[k] + 1) * mm];
                for (yv, xv) in yi.iter_mut().zip(xr) {
                    yv.re += a.re * xv.re - a.im * xv.im;
                    yv.im += a.re * xv.im + a.im * xv.re;
                }
            }
        }
    }
    /// the largest |H_ij - conj(H_ji)| (0 for a Hermitian matrix)
    pub fn hermiticity(&self) -> f64 {
        let mut dev = 0.0f64;
        for i in 0..self.n {
            for k in self.rowptr[i]..self.rowptr[i + 1] {
                let j = self.col[k];
                let other = match self.col[self.rowptr[j]..self.rowptr[j + 1]].binary_search(&i) {
                    Ok(p) => self.val[self.rowptr[j] + p],
                    Err(_) => C64::ZERO,
                };
                dev = dev.max((self.val[k] - other.conj()).abs());
            }
        }
        dev
    }
    pub fn dense(&self) -> CMat {
        let mut a = CMat::zeros(self.n);
        for i in 0..self.n {
            for k in self.rowptr[i]..self.rowptr[i + 1] {
                a.set(i, self.col[k], a.get(i, self.col[k]) + self.val[k]);
            }
        }
        a
    }
}

/// Hbound (2.38): an upper bound of the eigenvalue magnitudes of H_lat
pub fn norm_bound(lat: &Lat, vmax: f64, d: &[[C64; 3]], dscale: f64) -> f64 {
    let dmax = d.iter().map(|x| x.iter().map(|z| z.abs()).sum::<f64>()).fold(0.0, f64::max) * dscale;
    lat.nact as f64 * (1.0 + 2.0 * lat.l.wilson) / lat.l.ell + lat.l.mass.abs() + lat.l.mu.abs() + vmax + dmax
}

/// Pi^iota[site] = sum over the modes of w_p^T Kch_iota conj(w_h); the modes are the columns of w (n rows of mm columns)
pub fn anomalous(m: &M, ns: usize, w: &[C64], mm: usize) -> Vec<[C64; 3]> {
    let mut pi = vec![[C64::ZERO; 3]; ns];
    let mut kk = [[[C64::ZERO; 8]; 8]; 3];
    for i in 0..3 {
        for a in 0..8 {
            for b in 0..8 {
                kk[i][a][b] = m.k[i].get(a, b);
            }
        }
    }
    for s in 0..ns {
        for a in 0..8 {
            for b in 0..8 {
                let ks = [kk[0][a][b], kk[1][a][b], kk[2][a][b]];
                if ks.iter().all(|z| *z == C64::ZERO) {
                    continue;
                }
                let ur = &w[(16 * s + a) * mm..(16 * s + a + 1) * mm];
                let vr = &w[(16 * s + 8 + b) * mm..(16 * s + 8 + b + 1) * mm];
                let mut acc = C64::ZERO;
                for (u, v) in ur.iter().zip(vr) {
                    acc.re += u.re * v.re + u.im * v.im;
                    acc.im += u.im * v.re - u.re * v.im;
                }
                for i in 0..3 {
                    if ks[i] != C64::ZERO {
                        pi[s][i] += ks[i] * acc;
                    }
                }
            }
        }
    }
    pi
}

/// out += the pair terms of the field d applied to the modes w (n rows of mm columns)
pub fn apply_pair(m: &M, d: &[[C64; 3]], w: &[C64], out: &mut [C64], mm: usize) {
    for (s, ds) in d.iter().enumerate() {
        let p = pair_block(m, ds);
        for a in 0..8 {
            for b in 0..8 {
                let pab = p[a][b];
                if pab == C64::ZERO {
                    continue;
                }
                let pc = pab.conj();
                for cc in 0..mm {
                    let wh = w[(16 * s + 8 + b) * mm + cc];
                    let wp = w[(16 * s + a) * mm + cc];
                    out[(16 * s + a) * mm + cc] += pab * wh;
                    out[(16 * s + 8 + b) * mm + cc] += pc * wp;
                }
            }
        }
    }
}

/// eps_lat(q) of (2.37) and its gradient (the group velocity)
pub fn eps_lat_q(lat: &Lat, q: &[f64; 3]) -> f64 {
    t2core::lattice::eps_lat(q, lat.l.ell, lat.l.mass, lat.l.wilson, &lat.active)
}

pub fn grad_eps(lat: &Lat, q: &[f64; 3]) -> [f64; 3] {
    let ell = lat.l.ell;
    let e = eps_lat_q(lat, q);
    let meff = lat.l.mass + lat.l.wilson / ell * (0..3).filter(|&j| lat.active[j]).map(|j| 1.0 - (q[j] * ell).cos()).sum::<f64>();
    let mut g = [0.0; 3];
    for j in 0..3 {
        if lat.active[j] {
            let (s, cc) = ((q[j] * ell).sin(), (q[j] * ell).cos());
            g[j] = (s * cc / ell + meff * lat.l.wilson * s) / e;
        }
    }
    g
}

/// the free-lattice matrix of the particle part (h_lat(q)) or of the hole part (-conj(h_lat(-q)))
pub fn branch_matrix(lat: &Lat, q: &[f64; 3], hole: bool) -> CMat {
    if hole {
        h_lat_plane(&[-q[0], -q[1], -q[2]], lat.l.ell, lat.l.mass, lat.l.wilson, &lat.active).conj().scale_r(-1.0)
    } else {
        h_lat_plane(q, lat.l.ell, lat.l.mass, lat.l.wilson, &lat.active)
    }
}

/// the discrete Fourier transform (sign -1: forward as numpy.fft, +1: inverse without the factor 1/n) of the field
/// f[site][8] along every active direction
pub fn dft(lat: &Lat, f: &mut Vec<[C64; 8]>, sign: f64) {
    for j in 0..3 {
        if !lat.active[j] {
            continue;
        }
        let n = lat.l.n[j];
        let tw: Vec<C64> = (0..n * n).map(|k| C64::cis(sign * 2.0 * std::f64::consts::PI * ((k / n) * (k % n)) as f64 / n as f64)).collect();
        let mut out = f.clone();
        for s in 0..lat.ns {
            let cc = lat.l.coords(s);
            if cc[j] != 0 {
                continue;
            }
            // the line through s along j
            let line: Vec<usize> = (0..n)
                .map(|t| {
                    let mut x = cc;
                    x[j] = t;
                    lat.l.site(x[0], x[1], x[2])
                })
                .collect();
            for (kk, &sk) in line.iter().enumerate() {
                let mut acc = [C64::ZERO; 8];
                for (t, &st) in line.iter().enumerate() {
                    let ph = tw[kk * n + t];
                    for a in 0..8 {
                        acc[a] += ph * f[st][a];
                    }
                }
                out[sk] = acc;
            }
        }
        *f = out;
    }
}

/// the momenta of the sites in the order of the discrete Fourier transform
pub fn momentum(lat: &Lat, s: usize) -> [f64; 3] {
    let cc = lat.l.coords(s);
    let mut q = [0.0; 3];
    for j in 0..3 {
        if lat.active[j] {
            let n = lat.l.n[j] as i64;
            let k = cc[j] as i64;
            let kk = if k <= n / 2 { k } else { k - n };
            q[j] = 2.0 * std::f64::consts::PI * kk as f64 / (n as f64 * lat.l.ell);
        }
    }
    q
}

/// the projector (1 + sgn M(q)/eps_lat(q))/2 on a branch, applied to the 8 components (4 Dirac x 2 isospin)
pub fn project(lat: &Lat, q: &[f64; 3], hole: bool, sgn: f64, v: &[C64; 8]) -> [C64; 8] {
    let mq = branch_matrix(lat, q, hole);
    let e = eps_lat_q(lat, q);
    let mut o = [C64::ZERO; 8];
    for a in 0..4 {
        for b in 0..4 {
            let pab = mq.get(a, b).scale(0.5 * sgn / e) + if a == b { c(0.5, 0.0) } else { C64::ZERO };
            for i in 0..2 {
                o[2 * a + i] += pab * v[2 * b + i];
            }
        }
    }
    o
}

/// the Gaussian packet of the input file (4.2), projected on its branch at every momentum on a periodic lattice
pub fn packet_mode(cfg: &Cfg, lat: &Lat) -> Vec<C64> {
    let ns = lat.ns;
    let q = cfg.v("packet_momentum");
    let q = [q[0], q[1], q[2]];
    let cen = if cfg.has("packet_center") {
        cfg.v("packet_center")
    } else {
        (0..3).map(|j| 0.5 * (lat.l.n[j] - 1) as f64 * lat.l.ell).collect()
    };
    let sig = cfg.f("packet_width");
    let hole = cfg.s("packet_part") == "hole";
    let sgn = if cfg.s("packet_branch") == "positive" { 1.0 } else { -1.0 };
    let r2 = 1.0 / 2f64.sqrt();
    let up = [c(r2, 0.0), C64::ZERO, c(r2, 0.0), C64::ZERO];
    let down = [C64::ZERO, c(r2, 0.0), C64::ZERO, c(r2, 0.0)];
    let to8 = |x: &[C64; 4]| -> [C64; 8] {
        let mut o = [C64::ZERO; 8];
        for a in 0..4 {
            o[2 * a] = x[a];
        }
        o
    };
    // the projection of the reference spinor (isospin component 0 for the Dirac spinor; the isospin is set below)
    let refv = if cfg.s("packet_spin") == "up" { up } else { down };
    let mut chi = project(lat, &q, hole, sgn, &to8(&refv));
    if chi.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt() < 1e-8 {
        chi = project(lat, &q, hole, sgn, &to8(if cfg.s("packet_spin") == "up" { &down } else { &up }));
    }
    let nrm = chi.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    let iso = if cfg.s("packet_isospin") == "up" { 0 } else { 1 };
    let mut sp8 = [C64::ZERO; 8];
    for a in 0..4 {
        sp8[2 * a + iso] = chi[2 * a].scale(1.0 / nrm);
    }
    let mut f = vec![[C64::ZERO; 8]; ns];
    for s in 0..ns {
        let mut amp = 1.0;
        let mut ph = 0.0;
        for j in 0..3 {
            if !lat.active[j] {
                continue;
            }
            let mut dx = lat.x[s][j] - cen[j];
            if lat.l.periodic[j] {
                let l = lat.l.n[j] as f64 * lat.l.ell;
                dx = (dx + 0.5 * l).rem_euclid(l) - 0.5 * l;
            }
            amp *= (-dx * dx / (4.0 * sig * sig)).exp();
            ph += q[j] * dx;
        }
        let z = C64::cis(ph).scale(amp);
        for a in 0..8 {
            f[s][a] = z * sp8[a];
        }
    }
    if (0..3).all(|j| !lat.active[j] || lat.l.periodic[j]) {
        dft(lat, &mut f, -1.0);
        for s in 0..ns {
            let qk = momentum(lat, s);
            f[s] = project(lat, &qk, hole, sgn, &f[s]);
        }
        dft(lat, &mut f, 1.0);
    }
    let off = if hole { 8 } else { 0 };
    let mut w = vec![C64::ZERO; 16 * ns];
    for s in 0..ns {
        for a in 0..8 {
            w[16 * s + off + a] = f[s][a];
        }
    }
    let nrm = w.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    w.iter().map(|z| z.scale(1.0 / nrm)).collect()
}

/// the exact mean velocity of a mode of the free lattice operator on a periodic lattice (see t2_solve.py)
pub fn expected_velocity(w: &[C64], hole: bool, lat: &Lat) -> [f64; 3] {
    let off = if hole { 8 } else { 0 };
    let mut f: Vec<[C64; 8]> = (0..lat.ns)
        .map(|s| {
            let mut o = [C64::ZERO; 8];
            o.copy_from_slice(&w[16 * s + off..16 * s + off + 8]);
            o
        })
        .collect();
    dft(lat, &mut f, -1.0);
    let mut num = [0.0; 3];
    let mut den = 0.0;
    for s in 0..lat.ns {
        let q = momentum(lat, s);
        let pp = project(lat, &q, hole, 1.0, &f[s]);
        let wp: f64 = pp.iter().map(|z| z.norm_sqr()).sum();
        let tot: f64 = f[s].iter().map(|z| z.norm_sqr()).sum();
        let wm = tot - wp;
        let g = grad_eps(lat, &q);
        for j in 0..3 {
            num[j] += (wp - wm) * g[j];
        }
        den += tot;
    }
    [num[0] / den, num[1] / den, num[2] / den]
}

/// the centre of a density (as in t2_solve.py: the circular mean, then the mean in the window around it, on a ring)
pub fn center_of_mass(lat: &Lat, rho: &[f64]) -> [f64; 3] {
    let tot: f64 = rho.iter().sum();
    let mut out = [0.0; 3];
    for j in 0..3 {
        if !lat.active[j] || tot == 0.0 {
            continue;
        }
        if lat.l.periodic[j] {
            let l = lat.l.n[j] as f64 * lat.l.ell;
            let (mut sn, mut cs) = (0.0, 0.0);
            for s in 0..lat.ns {
                let th = 2.0 * std::f64::consts::PI * lat.x[s][j] / l;
                sn += rho[s] * th.sin();
                cs += rho[s] * th.cos();
            }
            let ang = sn.atan2(cs).rem_euclid(2.0 * std::f64::consts::PI);
            let cc = ang * l / (2.0 * std::f64::consts::PI);
            let mut m = 0.0;
            for s in 0..lat.ns {
                m += rho[s] * ((lat.x[s][j] - cc + 0.5 * l).rem_euclid(l) - 0.5 * l);
            }
            out[j] = (cc + m / tot).rem_euclid(l);
        } else {
            out[j] = (0..lat.ns).map(|s| rho[s] * lat.x[s][j]).sum::<f64>() / tot;
        }
    }
    out
}

/// the root of the lattice gap equation (3.21) of a periodic chain by bisection (0 if there is none)
pub fn lattice_gap_root(lat: &Lat, g: f64) -> f64 {
    let n = lat.l.n[0];
    let eps: Vec<f64> = (0..n)
        .map(|k| {
            let kk = if k <= n / 2 { k as f64 } else { k as f64 - n as f64 };
            eps_lat_q(lat, &[2.0 * std::f64::consts::PI * kk / (n as f64 * lat.l.ell), 0.0, 0.0])
        })
        .collect();
    let mu = lat.l.mu;
    let rhs = |dd: f64| -> f64 {
        eps.iter().map(|e| 1.0 / ((e - mu).powi(2) + dd * dd).sqrt() + 1.0 / ((e + mu).powi(2) + dd * dd).sqrt()).sum::<f64>()
            / (n as f64 * lat.l.ell)
    };
    let (mut lo, mut hi) = (1e-12, 50.0);
    if rhs(lo) < 1.0 / g {
        return 0.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if rhs(mid) > 1.0 / g {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// solve the small real linear system a x = b (Gaussian elimination with partial pivoting)
pub fn solve_small(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let n = b.len();
    for k in 0..n {
        let p = (k..n).max_by(|&i, &j| a[i][k].abs().partial_cmp(&a[j][k].abs()).unwrap()).unwrap();
        a.swap(k, p);
        b.swap(k, p);
        for i in k + 1..n {
            let f = a[i][k] / a[k][k];
            for j in k..n {
                a[i][j] -= f * a[k][j];
            }
            b[i] -= f * b[k];
        }
    }
    let mut x = vec![0.0; n];
    for k in (0..n).rev() {
        let mut s = b[k];
        for j in k + 1..n {
            s -= a[k][j] * x[j];
        }
        x[k] = s / a[k][k];
    }
    x
}

/// the angular frequency of the best least-squares fit of a0 + b cos(w t) + c sin(w t) (as t2_solve.py)
pub fn fit_frequency(t: &[f64], a: &[f64], wlo: f64, whi: f64, nw: usize) -> (f64, f64) {
    let resid = |w: f64| -> (f64, Vec<f64>) {
        let mut ata = vec![vec![0.0; 3]; 3];
        let mut atb = vec![0.0; 3];
        for (ti, ai) in t.iter().zip(a) {
            let r = [1.0, (w * ti).cos(), (w * ti).sin()];
            for p in 0..3 {
                atb[p] += r[p] * ai;
                for q in 0..3 {
                    ata[p][q] += r[p] * r[q];
                }
            }
        }
        let coef = solve_small(ata, atb);
        let rr: f64 = t.iter().zip(a).map(|(ti, ai)| (ai - coef[0] - coef[1] * (w * ti).cos() - coef[2] * (w * ti).sin()).powi(2)).sum();
        (rr, coef)
    };
    let ws: Vec<f64> = (0..nw).map(|k| wlo + (whi - wlo) * k as f64 / (nw - 1) as f64).collect();
    let rs: Vec<f64> = ws.iter().map(|&w| resid(w).0).collect();
    let mut k = 0;
    for i in 1..nw {
        if rs[i] < rs[k] {
            k = i;
        }
    }
    let k = k.clamp(1, nw - 2);
    let (r0, r1, r2) = (rs[k - 1], rs[k], rs[k + 1]);
    let h = ws[1] - ws[0];
    let den = r0 - 2.0 * r1 + r2;
    let w = ws[k] + if den > 0.0 { 0.5 * h * (r0 - r2) / den } else { 0.0 };
    let coef = resid(w).1;
    (w, coef[1].hypot(coef[2]))
}
