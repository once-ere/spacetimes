//! The spin-zero pair channels at one point and the signs of the interactions in them (Section 8.1 of
//! superfluid.md).  Exact operator algebra on the 256 states of one point.
//!
//! The normalized pair state of the antisymmetric unitary pair matrix K is
//!     |K> = (1/2) sum_cc' K^*_cc' Psi^dag_c Psi^dag_c' |0> / 2
//! (its norm before the division is 2).  For a contact operator O, normal-ordered with respect to the empty state of
//! the point, E_pair[O; K] = <K|O|K> is the interaction energy of the two spinor charges in the channel K: negative
//! means attraction.  The effective coupling of a term in a channel is G^ch = -(1/4) E_pair per unit coupling, so that
//! the pairing term -(G/4) sum Pi^dag Pi has G^ch = G in its own channel.
//!
//! The gauge bosons are treated here as contact interactions (the limit of heavy bosons): the static exchange of the
//! boson b in the Feynman gauge is (g_b^2/2) D_b(0) sum_p eta_pp :(Psi^dag A_p Psi)^2: with A_p = gamma^0 gamma^p (x) t_b;
//! its sign in each channel says whether the boson attracts or repels the two charges of the pair at short distances.
use sfcore::c64::C64;
use sfcore::cmat::CMat;
use sfcore::dirac::*;
use sfcore::fockmat::Fock;
use sfcore::report::{f, write_csv, Report};
use sfcore::uniform::{Term, TermKind};
use std::path::Path;

/// the normal-ordered quartic :(Psi^dag X Psi)(Psi^dag Y Psi): on the Fock space of one point
pub fn quartic(fk: &Fock, a: &CMat, b: &CMat) -> CMat {
    let mut m = CMat::zeros(fk.dim);
    for i in 0..8 {
        for j in 0..8 {
            let x = a.get(i, j);
            if x.abs() == 0.0 {
                continue;
            }
            for k in 0..8 {
                for l in 0..8 {
                    let y = b.get(k, l);
                    if y.abs() == 0.0 {
                        continue;
                    }
                    fk.add_string(&[(true, i), (true, k), (false, l), (false, j)], x * y, &mut m);
                }
            }
        }
    }
    m
}

/// the pair state |K> (normalized) as a vector of the 256 states
pub fn pair_state(fk: &Fock, k: &CMat) -> Vec<C64> {
    let mut m = CMat::zeros(fk.dim);
    for a in 0..8 {
        for b in 0..8 {
            let z = k.get(a, b).conj();
            if z.abs() > 0.0 {
                fk.add_string(&[(true, a), (true, b)], z.scale(0.5), &mut m);
            }
        }
    }
    let mut vac = vec![C64::ZERO; fk.dim];
    vac[0] = C64::ONE;
    let v = m.matvec(&vac);
    let nrm = v.iter().map(|z| z.norm_sqr()).sum::<f64>().sqrt();
    v.iter().map(|z| z.scale(1.0 / nrm)).collect()
}

fn expect(op: &CMat, v: &[C64]) -> C64 {
    let w = op.matvec(v);
    v.iter().zip(&w).fold(C64::ZERO, |s, (a, b)| s + a.conj() * *b)
}

/// the vertex matrices A_p = gamma^0 gamma^p (x) t with the signs eta_pp
pub fn vertices(t2: &CMat) -> Vec<(f64, CMat)> {
    let eta = [1.0, -1.0, -1.0, -1.0];
    (0..4).map(|p| (eta[p], d8(&gamma(0).mul(&gamma(p))).mul(&i8(t2)))).collect()
}

/// returns the table rows (operator, channel, E_pair per unit coupling, G^ch per unit coupling) and records checks
pub fn run(rep: &mut Report, data: &Path) -> Vec<(String, String, f64)> {
    let fk = Fock::new(8);
    let beta8 = d8(&beta());
    let one8 = CMat::identity(8);
    let alph8: Vec<CMat> = (1..4).map(|j| d8(&alpha(j))).collect();
    // the operators per unit coupling
    let mut ops: Vec<(String, CMat)> = vec![];
    ops.push(("scalar (1/2):(Psi-bar Psi)^2:".into(), quartic(&fk, &beta8, &beta8).scale_r(0.5)));
    let mut vecop = quartic(&fk, &one8, &one8);
    for a in &alph8 {
        vecop = vecop.sub(&quartic(&fk, a, a));
    }
    ops.push(("vector (1/2):j_p j^p:".into(), vecop.scale_r(0.5)));
    let ks: Vec<CMat> = (1..4).map(k_scalar).collect();
    let kps: Vec<CMat> = (1..4).map(k_pseudo).collect();
    let mut pairop = CMat::zeros(fk.dim);
    for k in &ks {
        let p = fk.pair_ann(k);
        pairop = pairop.add(&p.dagger().mul(&p));
    }
    ops.push(("pairing -(1/4) sum Pi^dag Pi".into(), pairop.scale_r(-0.25)));
    // gauge bosons as contact interactions: (1/2) sum_p eta_pp :(Psi^dag A_p Psi)^2: per unit g^2 D(0)
    let bos: Vec<(String, CMat)> = vec![
        ("W1".into(), sigma(1).scale_r(0.5)),
        ("W2".into(), sigma(2).scale_r(0.5)),
        ("W3".into(), sigma(3).scale_r(0.5)),
        ("B".into(), CMat::identity(2)),
    ];
    for (name, t) in &bos {
        let mut op = CMat::zeros(fk.dim);
        for (eta, a) in vertices(t) {
            op = op.add(&quartic(&fk, &a, &a).scale_r(0.5 * eta));
        }
        ops.push((format!("boson {} (1/2) sum eta :j^p j_p:", name), op));
    }
    let chans: Vec<(String, CMat)> = (0..3)
        .map(|i| (format!("K^{} (scalar 0+)", i + 1), ks[i].clone()))
        .chain((0..3).map(|i| (format!("K'^{} (pseudoscalar 0-)", i + 1), kps[i].clone())))
        .collect();
    let mut rows = vec![];
    let mut out = vec![];
    let mut imag_max: f64 = 0.0;
    for (oname, op) in &ops {
        for (cname, k) in &chans {
            let v = pair_state(&fk, k);
            let e = expect(op, &v);
            imag_max = imag_max.max(e.im.abs());
            rows.push(vec![oname.clone(), cname.clone(), f(e.re), f(-0.25 * e.re)]);
            out.push((oname.clone(), cname.clone(), e.re));
        }
    }
    write_csv(&data.join("part2_pair_channels.csv"), &["operator", "channel", "E_pair_per_unit_coupling", "G_ch_per_unit_coupling"], &rows);
    rep.add("P2-0a", "the pair-channel energies are real (largest imaginary part)", imag_max, 1e-12);
    // the pairing term in its own channel: E_pair = -4, so G^ch = G
    let e33 = out.iter().find(|r| r.0.starts_with("pairing") && r.1.starts_with("K^3")).unwrap().2;
    rep.add("P2-0b", "the pairing term -(G/4) sum Pi^dag Pi has the pair energy -4 G in the channel K^3 (so G^ch = G)", (e33 + 4.0).abs(), 1e-12);
    // the vector term is the contact limit of the exchange of B (the same operator); the sum over the three W bosons
    // is a quarter of B in every spin-zero channel (the Casimir value T1.T2 = 1/4 of the isospin triplet)
    let get = |o: &str, c: &str| out.iter().find(|r| r.0.starts_with(o) && r.1 == c).unwrap().2;
    let mut dv: f64 = 0.0;
    let mut dw: f64 = 0.0;
    for (cname, _) in &chans {
        dv = dv.max((get("vector", cname) - get("boson B", cname)).abs());
        let sw = get("boson W1", cname) + get("boson W2", cname) + get("boson W3", cname);
        dw = dw.max((sw - 0.25 * get("boson B", cname)).abs());
    }
    rep.add("P2-0c", "the vector term and the contact limit of the exchange of B have equal pair energies in the six channels", dv, 1e-12);
    rep.add("P2-0d", "sum over W^1, W^2, W^3 = 1/4 of B in every channel (isospin Casimir of the triplet pair, +1/4)", dw, 1e-12);
    // the pair contraction of the mean field (6.1), (6.2) against the exact two-body energies: for kap = K^*, the pair
    // contraction energy of every operator is 4 E_pair (|K> has the norm 2 before normalization, tr(K K^dag) = 8)
    let mut worst: f64 = 0.0;
    let opterms: Vec<(String, Vec<Term>)> = {
        let mut v: Vec<(String, Vec<Term>)> = vec![];
        let bt = |nu: f64, x: &CMat| Term { kind: TermKind::Bilinear { nu, x: x.clone(), y: x.clone() }, w_h: 0.0, w_f: 0.0, w_p: 1.0, ks_projected: false, label: String::new() };
        v.push(("scalar".into(), vec![bt(0.5, &beta8)]));
        let mut vt = vec![bt(0.5, &one8)];
        for a in &alph8 {
            vt.push(bt(-0.5, a));
        }
        v.push(("vector".into(), vt));
        v.push(("pairing".into(), ks.iter().map(|k| Term { kind: TermKind::Pairing { g: 1.0, k: k.clone() }, w_h: 0.0, w_f: 0.0, w_p: 1.0, ks_projected: false, label: String::new() }).collect()));
        for (name, t) in &bos {
            v.push((format!("boson {}", name), vertices(t).into_iter().map(|(eta, a)| bt(0.5 * eta, &a)).collect()));
        }
        v
    };
    for (oname, ts) in &opterms {
        for (cname, k) in &chans {
            let kap = k.conj();
            let e: f64 = ts.iter().map(|t| t.eval(&CMat::zeros(8), &kap).0).sum();
            worst = worst.max((e - 4.0 * get(oname, cname)).abs());
        }
    }
    rep.add("P2-0e", "the pair contraction of the mean field = 4 x the exact two-body energy, every operator and channel (42 cases)", worst, 1e-12);
    out
}
