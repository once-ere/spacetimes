//! The gradient coefficient of the field Phi at the critical temperature and the Ginzburg number of the transition
//! (Section 12.5 of superfluid.md; asked for by the author on 21 November 2026: to find how wide the fluctuation region
//! near T_c is, which needs the gradient coefficient at T_c).
//!
//! THE FORM OF GINZBURG AND LANDAU.  Near the critical temperature T_c of method X the field Phi of the superfluid is
//! small.  For a field that varies slowly in space, the free energy per volume, to second order in Phi and in the
//! derivatives and to fourth order in Phi without derivatives, is
//!     F[Phi] = c sum_{j=1..3} |d_j Phi|^2 + a(T) |Phi|^2 + b |Phi|^4,    a(T) = alpha (T - T_c)/T_c,
//! with |Phi|^2 = sum_iota |Phi^iota|^2 (Phi is a complex isovector: six real components).  The rotations and the
//! isospin rotations allow no other term of second order in Phi; the quartic term along the polar fields (the minimum
//! of Section 13) is b |Phi|^4.
//!
//! THE QUADRATIC COEFFICIENT FROM THE LINEAR RESPONSE.  A pair field with the total momentum k, Phi(x) = Phi e^{i k x},
//! has, at second order in Phi, the free energy a(k) |Phi|^2 with a(k) = a(T) + c k^2 + O(k^4).  The state of the field is
//! the thermal state of the BdG matrices with a small pair self-energy P (8 x 8 antisymmetric) that pairs the momenta
//! q + k/2 and -q + k/2 (q the relative momentum, |q| < Lambda: the cutoff acts on the relative momentum, see below),
//! and with the normal self-energy N_n of the normal state at T (at second order in Phi a change of N changes the free
//! energy only at fourth order, because the free energy is stationary in N).  Its pair matrix is kap = chi(P), linear
//! in P (the static pair response; the thermal state f(H) to first order in the pair block):
//!     chi(P) = - sum_q w_q U_A [ (U_A^dag P U_D) o F ] U_D^dag,    F_ab = (f(e_a) - f(e'_b)) / (e_a - e'_b),
//! with the eigenvalues e_a, eigenvectors U_A of the particle block A = h(q) + N_n + X and e'_b, U_D of the hole block
//! D = -(h(-q) + N_n + X)^T (o: the elementwise product; F_ab -> f'(e) at e_a = e'_b).  The constant matrix X is the
//! twist: X = Q alpha^3 shifts the momenta of all charges by Q (the pair, of fermion number 2, gets the momentum k = 2Q);
//! X = Q alpha^3 (x) tau_1/2 turns the isospin direction of the field (the components of charge +-1 of T^1 get the
//! momenta +-Q); X = Q alpha^3 (x) tau_3/2 twists the unbroken T^3 of the field along e_3, which does not change the
//! field and costs nothing in a gauge-invariant theory (its cost measures the error of the regularization).
//! The free energy of the state, from the normal state at T, is at second order (dE = tr(N d drho) - Re tr(P dkap^dag))
//!     F_2 = (1/2) Re tr( (P - P_m(kap)) kap^dag ),
//! where P_m(kap) is the pair self-energy of the method's functional (Model::self_energy_lin; it contains every pair
//! channel of the method, also those other than K^3).  The field of the state is Phi = tr(K^3 P_m)/8, and the free
//! energy of the field is the smallest F_2 among the states with this Phi: the components of P along (K^1)^dag,
//! (K^2)^dag, (K^3)^dag (the three components of the field) are held at (0, 0, 1) and all other components are made
//! self-consistent, P_perp = Pi_perp P_m(chi(P)) (a linear system of 28 complex unknowns; Pi_perp removes the
//! components along the three (K^iota)^dag).  Then a(k) = F_2/|Phi|^2.  At k = 0 no other channel is
//! induced (|P_perp| is reported) and a(0) = (1 - lambda^P(T))/(lambda^P(T) g^P), with the eigenvalue lambda^P(T) of
//! (12.9) (check P3-G2).
//!
//! THE COEFFICIENTS.  c = [a(k) - a(0)]/k^2 at T_c (Richardson extrapolation of k = 0.01, 0.02); alpha = T_c da/dT at
//! T_c (central differences at T_c (1 +- h), h = 0.002, 0.004, Richardson); b from the Taylor coefficients of the
//! nonlinear free energy at T_c (four small applied fields, as in Section 12.4).
//!
//! THE GINZBURG NUMBER.  For n real components (here n = 6) the Gaussian fluctuations of the field above T_c add to the
//! specific heat per volume C_fl = n alpha^{3/2} / (16 pi c^{3/2} t^{1/2}) (t = (T - T_c)/T_c, classical fluctuations of
//! the static field, weight exp(-F[Phi]/T)), while the mean-field specific heat jumps at T_c by Delta C = alpha^2/(2 b T_c).
//! The two are equal at t = Gi with
//!     Gi = n^2 b^2 T_c^2 / (64 pi^2 alpha c^3):
//! mean-field theory holds for |t| >> Gi, and the critical region has the width of about Gi T_c.
//!
//! THE REGULARIZATION.  With a sharp cutoff on the momenta of the two charges of a pair (|q +- k/2| < Lambda) a moving
//! pair loses the pairs near the cutoff sphere in a shell of thickness of order |k|, so that a(k) - a(0) is of order |k|
//! and a gradient expansion does not exist; the cutoff is therefore put on the relative momentum q (the routing of the
//! momenta of the space components of Section 12.2, where the uniform twist shifts the momenta inside the fixed sphere).
//! The twist of the unbroken T^3 (check P3-G5) and the twisted states with a self-consistent normal self-energy (P3-G7)
//! measure the remaining error of the regularization.
use sfcore::c64::{c, C64};
use sfcore::cmat::{eigh, CMat};
use sfcore::dirac::{alpha, beta, d8, h_free, i8, k_scalar, sigma};
use sfcore::linalg::lu_solve;
use sfcore::methods::M_LL;
use sfcore::uniform::{fermi, Grid, Model};

/// the number of real components of the field (a complex isovector)
pub const N_COMP: f64 = 6.0;
/// the smaller pair momentum of the extrapolation of the gradient coefficient (the larger is 2 K1)
pub const K1: f64 = 0.01;

/// the 28 index pairs (a, b), a < b, of the basis E_ab - E_ba of the antisymmetric 8 x 8 matrices
fn pairs() -> Vec<(usize, usize)> {
    let mut v = vec![];
    for a in 0..8 {
        for b in (a + 1)..8 {
            v.push((a, b));
        }
    }
    v
}

/// the basis matrix E_ab - E_ba
fn bmat(a: usize, b: usize) -> CMat {
    let mut m = CMat::zeros(8);
    m.set(a, b, c(1.0, 0.0));
    m.set(b, a, c(-1.0, 0.0));
    m
}

/// the coefficients of an antisymmetric matrix in the basis E_ab - E_ba (its entries above the diagonal)
fn coeffs(p: &CMat) -> Vec<C64> {
    pairs().iter().map(|&(a, b)| p.get(a, b)).collect()
}

/// the antisymmetric matrix with the given coefficients
fn from_coeffs(v: &[C64]) -> CMat {
    let mut m = CMat::zeros(8);
    for (j, &(a, b)) in pairs().iter().enumerate() {
        m.set(a, b, v[j]);
        m.set(b, a, c(0.0, 0.0) - v[j]);
    }
    m
}

/// the component of a pair self-energy along (K^3)^dag: tr(K^3 P)/8 (the field Phi of a state is this component of the
/// pair self-energy of the method; tr(K^3 (K^3)^dag) = 8)
fn comp3(p: &CMat) -> C64 {
    k_scalar(3).mul(p).trace().scale(1.0 / 8.0)
}

/// (f(e1) - f(e2))/(e1 - e2), and the derivative f'(e) = -f(1 - f)/t at coinciding arguments
fn dfermi(e1: f64, e2: f64, t: f64) -> f64 {
    if (e1 - e2).abs() < 1e-7 {
        let f = fermi(0.5 * (e1 + e2), t);
        -f * (1.0 - f) / t
    } else {
        (fermi(e1, t) - fermi(e2, t)) / (e1 - e2)
    }
}

/// the twist matrix Q alpha^3 (x) t2 (t2 a 2 x 2 isospin matrix)
pub fn twist_matrix(q: f64, t2: &CMat) -> CMat {
    d8(&alpha(3)).mul(&i8(t2)).scale_r(q)
}

/// the static pair response chi(B_j) of the 28 basis matrices at the temperature t, on the nodes of the grid, for the
/// normal self-energy nself and the twist x
pub fn pair_response(model: &Model, grid: &Grid, nself: &CMat, x: &CMat, t: f64) -> Vec<CMat> {
    let nodes: Vec<([f64; 3], f64)> = (0..grid.len()).map(|n| grid.node(n)).collect();
    pair_response_nodes(model, &nodes, nself, x, t)
}

/// the static pair response on a list of nodes (relative momentum q, weight of int d^3q/(2 pi)^3)
pub fn pair_response_nodes(model: &Model, nodes: &[([f64; 3], f64)], nself: &CMat, x: &CMat, t: f64) -> Vec<CMat> {
    let pr = pairs();
    let one = CMat::identity(8);
    let mut out = vec![CMat::zeros(8); pr.len()];
    for &(q, w) in nodes {
        let a = h_free(&q, model.m).sub(&one.scale_r(model.mu)).add(nself).add(x);
        let d = h_free(&[-q[0], -q[1], -q[2]], model.m).sub(&one.scale_r(model.mu)).add(nself).add(x).transpose().scale_r(-1.0);
        let (ea, ua) = eigh(&a);
        let (ed, ud) = eigh(&d);
        let (uad, udd) = (ua.dagger(), ud.dagger());
        let mut fm = vec![0.0; 64];
        for i in 0..8 {
            for j in 0..8 {
                fm[i * 8 + j] = dfermi(ea[i], ed[j], t);
            }
        }
        for (k, &(pa, pb)) in pr.iter().enumerate() {
            let mut m = uad.mul(&bmat(pa, pb)).mul(&ud);
            for i in 0..64 {
                m.a[i] = m.a[i] * fm[i];
            }
            out[k] = out[k].add(&ua.mul(&m).mul(&udd).scale_r(-w));
        }
    }
    out
}

/// the nodes of a region of relative momenta with the radius r(n) in the direction n (a product rule: Gauss-Legendre in
/// cos(theta) on [-1, 0] and on [0, 1] with nt nodes each, nphi equal steps in phi, nr Gauss-Legendre nodes on [0, r(n)])
pub fn region_nodes(nr: usize, nt: usize, nphi: usize, radius: &dyn Fn(&[f64; 3]) -> f64) -> Vec<([f64; 3], f64)> {
    use sfcore::quad::gauss_legendre;
    let (xr, wr) = gauss_legendre(nr);
    let (xt, wt) = gauss_legendre(nt);
    let mut out = vec![];
    for half in [-1.0f64, 1.0] {
        for (ct0, wt0) in xt.iter().zip(&wt) {
            // cos(theta) on [-1, 0] or [0, 1]; the weights of the unit sphere add up to 1
            let ct = 0.5 * (ct0 + half);
            let wct = 0.25 * wt0;
            let st = (1.0 - ct * ct).max(0.0).sqrt();
            for k in 0..nphi {
                let ph = 2.0 * std::f64::consts::PI * (k as f64 + 0.5) / nphi as f64;
                let n = [st * ph.cos(), st * ph.sin(), ct];
                let r = radius(&n);
                for (x, w) in xr.iter().zip(&wr) {
                    let q = 0.5 * r * (x + 1.0);
                    let wq = 0.5 * r * w * q * q / (2.0 * std::f64::consts::PI * std::f64::consts::PI) * wct / nphi as f64;
                    out.push(([q * n[0], q * n[1], q * n[2]], wq));
                }
            }
        }
    }
    out
}

/// the quadratic coefficient a(k) of the phase twist with the pair momentum k along e_3 for two regularizations: the
/// cutoff on the relative momentum (|q| < Lambda, the sphere) and the cutoff on the momenta of the two charges
/// (|q +- k/2| < Lambda, the lens); returns (a_sphere, a_lens)
pub fn a_regions(model: &Model, nself: &CMat, t: f64, k: f64) -> (f64, f64) {
    let lam = model.grid.lam;
    let x = twist_matrix(0.5 * k, &CMat::identity(2));
    let sphere = region_nodes(24, 12, 6, &|_n| lam);
    let lens = region_nodes(24, 12, 6, &|n| -0.5 * k * n[2].abs() + (lam * lam - 0.25 * k * k * (1.0 - n[2] * n[2])).sqrt());
    let a_s = quad(model, &pair_response_nodes(model, &sphere, nself, &x, t), true).a;
    let a_l = quad(model, &pair_response_nodes(model, &lens, nself, &x, t), true).a;
    (a_s, a_l)
}

/// the result of the quadratic free energy of a state with the field along e_3
#[derive(Clone, Copy, Debug)]
pub struct Quad {
    /// a(k) = F_2 / |Phi|^2
    pub a: f64,
    /// the norm of the self-consistent components of P other than (K^3)^dag (zero if relax = false)
    pub perp: f64,
}

/// the coefficient a(k) of |Phi|^2 for the response chi (the 28 matrices of pair_response); relax: the components of P
/// other than (K^3)^dag self-consistent (the free energy of the field), otherwise held at zero
pub fn quad(model: &Model, chi: &[CMat], relax: bool) -> Quad {
    let nb = chi.len();
    let z8 = CMat::zeros(8);
    // the linear map P -> P_m(chi(P)) on the coefficients: column j is the image of the basis matrix j
    let cols: Vec<Vec<C64>> = chi.iter().map(|k| coeffs(&model.self_energy_lin(&z8, k).1)).collect();
    let e3 = coeffs(&k_scalar(3).dagger());
    // the components along (K^1)^dag, (K^2)^dag, (K^3)^dag are the three components of the field itself (held at
    // (0, 0, 1)); the other 25 directions are the other pair channels (tr(K^i (K^j)^dag) = 8 delta_ij)
    let kd: Vec<Vec<C64>> = (1..4).map(|i| coeffs(&k_scalar(i).dagger())).collect();
    let proj_perp = |v: &[C64]| -> Vec<C64> {
        let mut out = v.to_vec();
        for (i, kv) in kd.iter().enumerate() {
            let ci = k_scalar(i + 1).mul(&from_coeffs(v)).trace().scale(1.0 / 8.0);
            for (o, &y) in out.iter_mut().zip(kv) {
                *o -= y * ci;
            }
        }
        out
    };
    let apply = |v: &[C64]| -> Vec<C64> {
        let mut out = vec![c(0.0, 0.0); nb];
        for (j, &vj) in v.iter().enumerate() {
            for i in 0..nb {
                out[i] += cols[j][i] * vj;
            }
        }
        out
    };
    let mut p = e3.clone();
    let mut perp = 0.0;
    if relax {
        // (1 - Pi_perp M) p_perp = Pi_perp M e3
        let mut mat = CMat::new(nb, nb);
        for j in 0..nb {
            let mut ej = vec![c(0.0, 0.0); nb];
            ej[j] = c(1.0, 0.0);
            let col = proj_perp(&apply(&ej));
            for i in 0..nb {
                mat.set(i, j, ej[i] - col[i]);
            }
        }
        let rhs = proj_perp(&apply(&e3));
        let pp = lu_solve(&mat, &rhs).expect("the system of the other pair channels is singular");
        perp = pp.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
        p = e3.iter().zip(&pp).map(|(&x, &y)| x + y).collect();
    }
    let pm = from_coeffs(&p);
    let mut kap = CMat::zeros(8);
    for (j, k) in chi.iter().enumerate() {
        kap = kap.add(&k.scale(p[j]));
    }
    let pmeth = model.self_energy_lin(&z8, &kap).1;
    let phi = comp3(&pmeth);
    let f2 = 0.5 * pm.sub(&pmeth).mul(&kap.dagger()).trace().re;
    Quad { a: f2 / phi.norm_sqr(), perp }
}

/// the normal state at the temperature t (zero pair self-energy) and its normal self-energy
pub fn normal_self_energy(model: &Model, t: f64) -> (CMat, f64) {
    let ns = model.solve_t(&CMat::zeros(8), &CMat::zeros(8), 0.6, 1e-14, 200000, t);
    (ns.n_self, ns.free)
}

/// the Dirac mass of a normal self-energy
pub fn dirac_mass(nself: &CMat) -> f64 {
    M_LL + d8(&beta()).mul(nself).trace().re / 8.0
}

/// a(k) at the temperature t for the twist of strength q of the isospin matrix t2 (q = 0: no twist)
pub fn a_twist(model: &Model, grid: &Grid, nself: &CMat, t: f64, q: f64, t2: &CMat, relax: bool) -> Quad {
    let x = twist_matrix(q, t2);
    let chi = pair_response(model, grid, nself, &x, t);
    quad(model, &chi, relax)
}

/// the gradient coefficient from the twist t2: [a(k) - a(0)] / k^2 at the pair momenta k = k1, 2 k1 and the Richardson
/// extrapolation; the twist strength is Q = k/s (s = 2 for the phase twist, whose pair of fermion number 2 gets the
/// momentum 2Q; s = 1 for the isospin twists, whose components of charge +-1 get the momenta +-Q); returns
/// (c, c(k1), c(2 k1))
pub fn gradient(model: &Model, grid: &Grid, nself: &CMat, t: f64, t2: &CMat, s: f64, k1: f64, relax: bool) -> (f64, f64, f64) {
    let a0 = a_twist(model, grid, nself, t, 0.0, t2, relax).a;
    let c1 = (a_twist(model, grid, nself, t, k1 / s, t2, relax).a - a0) / (k1 * k1);
    let c2 = (a_twist(model, grid, nself, t, 2.0 * k1 / s, t2, relax).a - a0) / (4.0 * k1 * k1);
    ((4.0 * c1 - c2) / 3.0, c1, c2)
}

/// the identity, tau_1/2 and tau_3/2 (the isospin matrices of the three twists)
pub fn twist_charges() -> (CMat, CMat, CMat) {
    (CMat::identity(2), sigma(1).scale_r(0.5), sigma(3).scale_r(0.5))
}

/// the Taylor coefficients (a, b) of the nonlinear free energy F(Phi; T) = a Phi^2 + b Phi^4 + ... at the origin, at the
/// temperature t, with an optional twist x (a constant one-body term of the Hamiltonian density): states with the applied
/// fields 0.01 .. 0.04 along (K^3)^dag and a self-consistent normal self-energy (Model::fixed_pair_state_t), from the
/// normal state with the same twist; F/Phi^2 is interpolated by a cubic polynomial in Phi^2 (as in Section 12.4)
pub fn taylor_nonlinear(model: &mut Model, t: f64, x: Option<CMat>) -> (f64, f64) {
    taylor_nonlinear_fields(model, t, x, &[0.01, 0.02, 0.03, 0.04])
}

/// taylor_nonlinear with the four applied fields given (check P3-G12 compares two sets of fields)
pub fn taylor_nonlinear_fields(model: &mut Model, t: f64, x: Option<CMat>, fields: &[f64; 4]) -> (f64, f64) {
    model.ext_n = x.clone();
    let x0 = x.unwrap_or_else(|| CMat::zeros(8));
    let ns = model.solve_t(&x0, &CMat::zeros(8), 0.6, 1e-14, 200000, t);
    let mut n0 = ns.n_self.clone();
    let mut xs = vec![];
    let mut ys = vec![];
    for &fa in fields {
        let s = model.fixed_pair_state_t(&k_scalar(3).dagger().scale_r(fa), &n0, 0.7, 1e-14, 200000, t);
        n0 = s.n_self.clone();
        let phi = comp3(&s.p_self).abs();
        xs.push(phi * phi);
        ys.push((s.free - ns.free) / (phi * phi));
    }
    model.ext_n = None;
    let cf = poly4(&xs, &ys);
    (cf[0], cf[1])
}

/// the coefficients of c0 + c1 x + c2 x^2 + c3 x^3 through four points (Gaussian elimination with partial pivoting)
fn poly4(xs: &[f64], ys: &[f64]) -> [f64; 4] {
    let mut a = [[0.0; 5]; 4];
    for i in 0..4 {
        for j in 0..4 {
            a[i][j] = xs[i].powi(j as i32);
        }
        a[i][4] = ys[i];
    }
    for k in 0..4 {
        let p = (k..4).max_by(|&i, &j| a[i][k].abs().partial_cmp(&a[j][k].abs()).unwrap()).unwrap();
        a.swap(k, p);
        for i in (k + 1)..4 {
            let f = a[i][k] / a[k][k];
            for j in k..5 {
                a[i][j] -= f * a[k][j];
            }
        }
    }
    let mut x = [0.0; 4];
    for k in (0..4).rev() {
        let mut s = a[k][4];
        for j in (k + 1)..4 {
            s -= a[k][j] * x[j];
        }
        x[k] = s / a[k][k];
    }
    x
}

/// the radial integral of (12.9) and its two derivatives on the radial nodes of the grid (E = (q^2 + M^2)^(1/2)):
///     I1(M, T) = sum_r w_r q^2 tanh(E/2T)/E,
///     dI1/dT  = -(1/(2 T^2)) sum_r w_r q^2 sech^2(E/2T),
///     dI1/dM  = sum_r w_r q^2 (M/E) [sech^2(E/2T)/(2 T E) - tanh(E/2T)/E^2];
/// at T = 0 the factor tanh is 1 and dI1/dT = 0
pub fn i1_derivatives(model: &Model, mm: f64, t: f64) -> (f64, f64, f64) {
    let g = &model.grid;
    let (mut i1, mut i1t, mut i1m) = (0.0, 0.0, 0.0);
    for (q, w) in g.kr.iter().zip(&g.wr) {
        let e = (q * q + mm * mm).sqrt();
        let (th, sech2) = if t > 0.0 {
            let x = e / (2.0 * t);
            (x.tanh(), 1.0 / (x.cosh() * x.cosh()))
        } else {
            (1.0, 0.0)
        };
        i1 += w * q * q * th / e;
        if t > 0.0 {
            i1t -= w * q * q * sech2 / (2.0 * t * t);
            i1m += w * q * q * (mm / e) * (sech2 / (2.0 * t * e) - th / (e * e));
        } else {
            i1m -= w * q * q * mm / (e * e * e);
        }
    }
    (i1, i1t, i1m)
}

/// alpha = T_c d a(0, T)/dT at T_c in closed form (check P3-G10): a(0, T) = (1 - lambda(T))/(lambda(T) g^P) (check P3-G2)
/// with lambda(T) = g^P I1(M_n(T), T)/pi^2 of (12.9), so that alpha = -T_c lambda'(T_c)/(lambda(T_c)^2 g^P); the Dirac
/// mass M_n of the normal state solves its scalar self-consistency M - m = g^S n_S(M, T),
/// n_S = -(2/pi^2) [M I1(M, T) - m I1(m, 0)] (bisection), and dM_n/dT = g^S dn_S/dT / (1 - g^S dn_S/dM) (implicit
/// function theorem); no BdG matrix and no finite difference is used.  Returns (alpha, lambda(T_c), M_n(T_c)).
pub fn alpha_closed(model: &Model, g_s: f64, g_p: f64, tc: f64) -> (f64, f64, f64) {
    let pi2 = std::f64::consts::PI * std::f64::consts::PI;
    let i1_m0 = i1_derivatives(model, M_LL, 0.0).0;
    let gap = |mm: f64| mm - M_LL + g_s * (2.0 / pi2) * (mm * i1_derivatives(model, mm, tc).0 - M_LL * i1_m0);
    let mn = if g_s == 0.0 {
        M_LL
    } else {
        let (mut lo, mut hi) = (0.01, 1.0);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if gap(lo) * gap(mid) <= 0.0 {
                hi = mid;
            } else {
                lo = mid;
            }
        }
        0.5 * (lo + hi)
    };
    let (i1, i1t, i1m) = i1_derivatives(model, mn, tc);
    let dns_t = -(2.0 / pi2) * mn * i1t;
    let dns_m = -(2.0 / pi2) * (i1 + mn * i1m);
    let dm_t = g_s * dns_t / (1.0 - g_s * dns_m);
    let lam = g_p * i1 / pi2;
    let dlam = g_p * (i1t + i1m * dm_t) / pi2;
    (-tc * dlam / (lam * lam * g_p), lam, mn)
}

/// the results of one method
#[derive(Clone, Debug)]
pub struct Ginzburg {
    pub tc: f64,
    /// a(0) at T_c (zero up to the accuracy of T_c), alpha = T_c da/dT, b at T_c (nonlinear Taylor coefficient)
    pub a_tc: f64,
    pub alpha: f64,
    pub b: f64,
    /// the gradient coefficient at T_c from the phase twist (the result), from the isospin twist (equal by isospin
    /// invariance), without the relaxation of the other pair channels, and the spurious coefficient of the T^3 twist
    pub c: f64,
    pub c_iso: f64,
    pub c_fixed: f64,
    pub c_t3: f64,
    /// the phase twist with a self-consistent normal self-energy (twisted states of the nonlinear engine, P along K^3)
    pub c_nonlinear: f64,
    /// the extrapolation steps c(h), c(2h) of the phase twist
    pub c_h: f64,
    pub c_2h: f64,
    /// the largest norm of the induced other pair channels (at k = 0 and at the twist 2h)
    pub perp0: f64,
    pub perp_k: f64,
    /// the Ginzburg number, the width Gi T_c of the critical region, the correlation-length amplitude (c/alpha)^(1/2) and
    /// the field at the edge of the critical region (alpha Gi/(2 b))^(1/2) of mean-field theory
    pub gi: f64,
    pub width: f64,
    pub xi0: f64,
    pub phi_g: f64,
    /// the Dirac mass of the normal state at T_c
    pub mass_n: f64,
}

/// the Ginzburg number of one method; g_p is the effective pair coupling, tc the critical temperature of Section 12.4
pub fn run(model: &mut Model, g_p: f64, tc: f64) -> Ginzburg {
    let grid = model.grid.clone();
    let (tf, ti, t3) = twist_charges();
    let (n_c, _) = normal_self_energy(model, tc);
    // alpha: central differences of a(0, T) at T_c (1 +- h), the normal state recomputed at every temperature
    let a0 = |model: &Model, t: f64| -> f64 {
        let (nn, _) = normal_self_energy(model, t);
        a_twist(model, &grid, &nn, t, 0.0, &tf, true).a
    };
    let d1 = (a0(model, tc * 1.002) - a0(model, tc * 0.998)) / (2.0 * 0.002 * tc);
    let d2 = (a0(model, tc * 1.004) - a0(model, tc * 0.996)) / (2.0 * 0.004 * tc);
    let alpha = tc * (4.0 * d1 - d2) / 3.0;
    let q0 = a_twist(model, &grid, &n_c, tc, 0.0, &tf, true);
    // the gradient coefficient at T_c
    // the pair momenta k = 0.01, 0.02 for every twist (phase twist Q = k/2, isospin twists Q = k)
    let (c, c_h, c_2h) = gradient(model, &grid, &n_c, tc, &tf, 2.0, K1, true);
    let (c_iso, _, _) = gradient(model, &grid, &n_c, tc, &ti, 1.0, K1, true);
    let (c_fixed, _, _) = gradient(model, &grid, &n_c, tc, &tf, 2.0, K1, false);
    let (c_t3, _, _) = gradient(model, &grid, &n_c, tc, &t3, 1.0, K1, true);
    let perp_k = a_twist(model, &grid, &n_c, tc, K1, &tf, true).perp;
    // the quartic coefficient and the nonlinear twisted states at T_c (normal self-energy self-consistent in the
    // twisted states; the pair self-energy along (K^3)^dag only, as c_fixed)
    let (a_nl0, b) = taylor_nonlinear(model, tc, None);
    let a_nl1 = taylor_nonlinear(model, tc, Some(twist_matrix(0.5 * K1, &tf))).0;
    let a_nl2 = taylor_nonlinear(model, tc, Some(twist_matrix(K1, &tf))).0;
    let c_nonlinear = (4.0 * (a_nl1 - a_nl0) / (K1 * K1) - (a_nl2 - a_nl0) / (4.0 * K1 * K1)) / 3.0;
    let _ = g_p;
    let pi2 = std::f64::consts::PI * std::f64::consts::PI;
    let gi = N_COMP * N_COMP * b * b * tc * tc / (64.0 * pi2 * alpha * c.powi(3));
    Ginzburg {
        tc,
        a_tc: q0.a,
        alpha,
        b,
        c,
        c_iso,
        c_fixed,
        c_t3,
        c_nonlinear,
        c_h,
        c_2h,
        perp0: q0.perp,
        perp_k,
        gi,
        width: gi * tc,
        xi0: (c / alpha).sqrt(),
        phi_g: (alpha * gi / (2.0 * b)).sqrt(),
        mass_n: dirac_mass(&n_c),
    }
}
