//! A small symbolic algebra for writing the matrices and the component equations in LaTeX.
//!
//! A `Lin` is a linear combination of monomials (LaTeX strings such as "\partial_3\psi^{\rm p}_{{\rm LL},1}")
//! with Gaussian-rational coefficients (a + b i with rational a, b).  Numerical matrices whose entries
//! are Gaussian rationals with small denominators (0, +-1, +-i, +-1/2, ...) act on vectors of `Lin`.
//! Every printed formula is also checked numerically by the programs (the monomials are evaluated
//! for random numbers and compared with the matrix computation).
use crate::c64::{c, C64};
use crate::cmat::CMat;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// a rational number n/d, d > 0, in lowest terms
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Q {
    pub n: i64,
    pub d: i64,
}

impl Q {
    pub fn new(n: i64, d: i64) -> Q {
        if n == 0 {
            return Q { n: 0, d: 1 };
        }
        let g = gcd(n, d);
        let s = if d < 0 { -1 } else { 1 };
        Q { n: s * n / g, d: s * d / g }
    }
    pub fn zero() -> Q {
        Q { n: 0, d: 1 }
    }
    pub fn add(self, o: Q) -> Q {
        Q::new(self.n * o.d + o.n * self.d, self.d * o.d)
    }
    pub fn mul(self, o: Q) -> Q {
        Q::new(self.n * o.n, self.d * o.d)
    }
    pub fn neg(self) -> Q {
        Q { n: -self.n, d: self.d }
    }
    pub fn f(self) -> f64 {
        self.n as f64 / self.d as f64
    }
    /// the nearest rational with denominator dividing 8; panics if x is not such a rational
    pub fn from_f64(x: f64) -> Q {
        let k = (x * 8.0).round();
        assert!((x * 8.0 - k).abs() < 1e-9, "entry {} is not a multiple of 1/8", x);
        Q::new(k as i64, 8)
    }
}

/// a Gaussian rational re + i im
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GR {
    pub re: Q,
    pub im: Q,
}

impl GR {
    pub fn new(re: Q, im: Q) -> GR {
        GR { re, im }
    }
    pub fn int(n: i64) -> GR {
        GR { re: Q::new(n, 1), im: Q::zero() }
    }
    pub fn rat(n: i64, d: i64) -> GR {
        GR { re: Q::new(n, d), im: Q::zero() }
    }
    pub fn imag(n: i64, d: i64) -> GR {
        GR { re: Q::zero(), im: Q::new(n, d) }
    }
    pub fn from_c(z: C64) -> GR {
        GR { re: Q::from_f64(z.re), im: Q::from_f64(z.im) }
    }
    pub fn is_zero(&self) -> bool {
        self.re.n == 0 && self.im.n == 0
    }
    pub fn add(self, o: GR) -> GR {
        GR { re: self.re.add(o.re), im: self.im.add(o.im) }
    }
    pub fn mul(self, o: GR) -> GR {
        GR { re: self.re.mul(o.re).add(self.im.mul(o.im).neg()), im: self.re.mul(o.im).add(self.im.mul(o.re)) }
    }
    pub fn neg(self) -> GR {
        GR { re: self.re.neg(), im: self.im.neg() }
    }
    pub fn conj(self) -> GR {
        GR { re: self.re, im: self.im.neg() }
    }
    pub fn to_c(self) -> C64 {
        c(self.re.f(), self.im.f())
    }
}

fn frac_tex(q: Q, with_i: bool) -> String {
    // q > 0 here
    let num = if with_i {
        if q.n == 1 {
            "i".to_string()
        } else {
            format!("{}i", q.n)
        }
    } else {
        format!("{}", q.n)
    };
    if q.d == 1 {
        num
    } else {
        format!("\\tfrac{{{}}}{{{}}}", num, q.d)
    }
}

/// (sign, magnitude text) of a coefficient; magnitude text is empty for 1
fn coef_tex(g: GR, alone: bool) -> (bool, String) {
    let (re, im) = (g.re, g.im);
    if im.n == 0 {
        let neg = re.n < 0;
        let a = Q { n: re.n.abs(), d: re.d };
        if a.n == a.d && !alone {
            return (neg, String::new());
        }
        return (neg, frac_tex(a, false));
    }
    if re.n == 0 {
        let neg = im.n < 0;
        let a = Q { n: im.n.abs(), d: im.d };
        if a.n == a.d && !alone {
            return (neg, "i".to_string());
        }
        return (neg, frac_tex(a, true));
    }
    // both parts: (a + b i) or -(a + b i)
    let neg = re.n < 0;
    let (r2, i2) = if neg { (re.neg(), im.neg()) } else { (re, im) };
    let rs = frac_tex(Q { n: r2.n.abs(), d: r2.d }, false);
    let is = frac_tex(Q { n: i2.n.abs(), d: i2.d }, true);
    let sign = if i2.n < 0 { "-" } else { "+" };
    (neg, format!("({}{}{})", rs, sign, is))
}

/// a linear combination of monomials with Gaussian-rational coefficients.  Every monomial also
/// carries its numerical value for one assignment of numbers to the symbols, so that every printed
/// expression can be compared numerically with the matrix computation it describes.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Lin {
    pub terms: Vec<(String, GR, C64)>,
}

impl Lin {
    pub fn zero() -> Lin {
        Lin { terms: vec![] }
    }
    /// the monomial `m` (with numerical value `val`) with coefficient 1; the monomial "1" is the number 1
    pub fn mono(m: &str, val: C64) -> Lin {
        Lin { terms: vec![(m.to_string(), GR::int(1), val)] }
    }
    pub fn one() -> Lin {
        Lin::mono("1", C64::ONE)
    }
    pub fn num(g: GR) -> Lin {
        if g.is_zero() {
            Lin::zero()
        } else {
            Lin { terms: vec![("1".to_string(), g, C64::ONE)] }
        }
    }
    pub fn add_term(&mut self, m: &str, g: GR, val: C64) {
        if g.is_zero() {
            return;
        }
        for t in self.terms.iter_mut() {
            if t.0 == m {
                t.1 = t.1.add(g);
                return;
            }
        }
        self.terms.push((m.to_string(), g, val));
    }
    pub fn add(&self, o: &Lin) -> Lin {
        let mut r = self.clone();
        for (m, g, v) in &o.terms {
            r.add_term(m, *g, *v);
        }
        r.terms.retain(|t| !t.1.is_zero());
        r
    }
    pub fn scale(&self, g: GR) -> Lin {
        let mut r = Lin { terms: self.terms.iter().map(|(m, x, v)| (m.clone(), x.mul(g), *v)).collect() };
        r.terms.retain(|t| !t.1.is_zero());
        r
    }
    /// every monomial multiplied (as text, on its right) by the factor `f` with the value `fv`
    pub fn times(&self, f: &str, fv: C64) -> Lin {
        Lin {
            terms: self
                .terms
                .iter()
                .map(|(m, g, v)| (if m == "1" { f.to_string() } else { format!("{}{}", m, f) }, *g, *v * fv))
                .collect(),
        }
    }
    pub fn is_zero(&self) -> bool {
        self.terms.iter().all(|t| t.1.is_zero())
    }
    /// the numerical value: sum of coefficient times value of the monomial
    pub fn eval(&self) -> C64 {
        self.terms.iter().fold(C64::ZERO, |s, (_, g, v)| s + g.to_c() * *v)
    }
    /// LaTeX text; terms are separated so that at most `per_line` terms stand on one line of an
    /// aligned environment (0: no breaks)
    pub fn tex_broken(&self, per_line: usize) -> String {
        let mut out = String::new();
        let mut first = true;
        let mut count = 0;
        for (m, g, _) in &self.terms {
            if g.is_zero() {
                continue;
            }
            let alone = m == "1";
            let (neg, mag) = coef_tex(*g, alone);
            if per_line > 0 && count > 0 && count % per_line == 0 {
                out.push_str("\\\\\n&\\quad ");
            }
            if first {
                if neg {
                    out.push('-');
                }
            } else {
                out.push_str(if neg { "-" } else { "+" });
            }
            out.push_str(&mag);
            if !alone {
                if !mag.is_empty() && (mag.ends_with('i') || mag.ends_with(')') || mag.ends_with('}')) {
                    out.push_str("\\,");
                }
                out.push_str(m);
            }
            first = false;
            count += 1;
        }
        if first {
            "0".to_string()
        } else {
            out
        }
    }
    pub fn tex(&self) -> String {
        self.tex_broken(0)
    }
}

/// the Gaussian-rational entries of a numerical matrix
pub fn gr_matrix(m: &CMat) -> Vec<Vec<GR>> {
    (0..m.rows).map(|i| (0..m.cols).map(|j| GR::from_c(m.get(i, j))).collect()).collect()
}

/// numerical matrix (Gaussian-rational entries) times a vector of linear combinations
pub fn mat_lin(m: &CMat, v: &[Lin]) -> Vec<Lin> {
    let g = gr_matrix(m);
    (0..m.rows)
        .map(|i| {
            let mut s = Lin::zero();
            for j in 0..m.cols {
                if !g[i][j].is_zero() {
                    s = s.add(&v[j].scale(g[i][j]));
                }
            }
            s
        })
        .collect()
}

/// a symbolic matrix whose entries are linear combinations
#[derive(Clone, Debug)]
pub struct SymMat {
    pub rows: usize,
    pub cols: usize,
    pub e: Vec<Lin>,
}

impl SymMat {
    pub fn zeros(rows: usize, cols: usize) -> SymMat {
        SymMat { rows, cols, e: vec![Lin::zero(); rows * cols] }
    }
    pub fn get(&self, i: usize, j: usize) -> &Lin {
        &self.e[i * self.cols + j]
    }
    /// adds (numerical matrix) times (linear combination)
    pub fn add_scaled(&mut self, m: &CMat, l: &Lin) {
        let g = gr_matrix(m);
        for i in 0..self.rows {
            for j in 0..self.cols {
                if !g[i][j].is_zero() {
                    let k = i * self.cols + j;
                    self.e[k] = self.e[k].add(&l.scale(g[i][j]));
                }
            }
        }
    }
    /// LaTeX pmatrix
    pub fn tex(&self) -> String {
        let mut s = String::from("\\begin{pmatrix}");
        for i in 0..self.rows {
            let row: Vec<String> = (0..self.cols).map(|j| self.get(i, j).tex()).collect();
            s.push_str(&row.join("&"));
            if i + 1 < self.rows {
                s.push_str("\\\\");
            }
        }
        s.push_str("\\end{pmatrix}");
        s
    }
}

/// LaTeX pmatrix of a numerical matrix with Gaussian-rational entries
pub fn tex_matrix(m: &CMat) -> String {
    let mut sm = SymMat::zeros(m.rows, m.cols);
    sm.add_scaled(m, &Lin::one());
    sm.tex()
}
