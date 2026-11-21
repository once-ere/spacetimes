//! Dense matrices of fermion operators on the Fock space of n modes (n at most 8: dimension 2^n at most 256).
//!
//! The basis state with the bit pattern b (bit k set: mode k occupied) is
//!     |b> = (c_0^dag)^{b_0} (c_1^dag)^{b_1} ... (c_{n-1}^dag)^{b_{n-1}} |0>,
//! the creation operators in the order of increasing k (the Jordan-Wigner order).  Then c_k acts on a state with bit
//! k set as c_k |b> = (-1)^{(number of occupied modes below k)} |b with bit k cleared>, and c_k^dag on a state with bit
//! k clear with the same sign.  These matrices satisfy the canonical anticommutation relations
//! {c_k, c_l^dag} = delta_kl, {c_k, c_l} = 0 (checked by the program sf_part1, check F-1).
//!
//! The eight modes of one point of the spinor field are the eight components Psi_LL,c (c = 0..7 in the index
//! convention of the module `dirac`); the four modes of a single Dirac field are its four components.
use crate::c64::C64;
use crate::cmat::CMat;

pub struct Fock {
    pub n: usize,
    pub dim: usize,
}

impl Fock {
    pub fn new(n: usize) -> Fock {
        assert!(n <= 10);
        Fock { n, dim: 1 << n }
    }
    /// apply one operator (dag = true: c_k^dag; false: c_k) to the basis state b: Some((sign, new state)) or None
    fn apply1(b: usize, dag: bool, k: usize) -> Option<(f64, usize)> {
        let bit = 1usize << k;
        let occ = b & bit != 0;
        if dag == occ {
            return None;
        }
        let sign = if (b & (bit - 1)).count_ones() % 2 == 0 { 1.0 } else { -1.0 };
        Some((sign, b ^ bit))
    }
    /// adds coef times the product of the operators of `ops` (the first entry is the leftmost factor) to m
    pub fn add_string(&self, ops: &[(bool, usize)], coef: C64, m: &mut CMat) {
        for b0 in 0..self.dim {
            let mut b = b0;
            let mut sign = 1.0;
            let mut alive = true;
            for &(dag, k) in ops.iter().rev() {
                match Fock::apply1(b, dag, k) {
                    Some((s, nb)) => {
                        sign *= s;
                        b = nb;
                    }
                    None => {
                        alive = false;
                        break;
                    }
                }
            }
            if alive {
                let idx = b * self.dim + b0;
                m.a[idx] += coef.scale(sign);
            }
        }
    }
    /// the annihilation operator c_k
    pub fn ann(&self, k: usize) -> CMat {
        let mut m = CMat::zeros(self.dim);
        self.add_string(&[(false, k)], C64::ONE, &mut m);
        m
    }
    /// the creation operator c_k^dag
    pub fn cre(&self, k: usize) -> CMat {
        let mut m = CMat::zeros(self.dim);
        self.add_string(&[(true, k)], C64::ONE, &mut m);
        m
    }
    /// the bilinear sum_{a b} c_a^dag A_ab c_b
    pub fn bilinear(&self, a: &CMat) -> CMat {
        let mut m = CMat::zeros(self.dim);
        for i in 0..a.rows {
            for j in 0..a.cols {
                let x = a.get(i, j);
                if x.abs() > 0.0 {
                    self.add_string(&[(true, i), (false, j)], x, &mut m);
                }
            }
        }
        m
    }
    /// the pair operator sum_{a b} K_ab c_a c_b (it annihilates two fermions)
    pub fn pair_ann(&self, k: &CMat) -> CMat {
        let mut m = CMat::zeros(self.dim);
        for i in 0..k.rows {
            for j in 0..k.cols {
                let x = k.get(i, j);
                if x.abs() > 0.0 {
                    self.add_string(&[(false, i), (false, j)], x, &mut m);
                }
            }
        }
        m
    }
    /// the linear combination sum_a v_a c_a (v_a complex)
    pub fn lin_ann(&self, v: &[C64]) -> CMat {
        let mut m = CMat::zeros(self.dim);
        for (a, &x) in v.iter().enumerate() {
            if x.abs() > 0.0 {
                self.add_string(&[(false, a)], x, &mut m);
            }
        }
        m
    }
    /// the linear combination sum_a v_a c_a^dag
    pub fn lin_cre(&self, v: &[C64]) -> CMat {
        let mut m = CMat::zeros(self.dim);
        for (a, &x) in v.iter().enumerate() {
            if x.abs() > 0.0 {
                self.add_string(&[(true, a)], x, &mut m);
            }
        }
        m
    }
}
