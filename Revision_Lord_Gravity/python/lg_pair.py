"""
lg_pair.py -- stage 7 of task C23 (PROJECT spacetimes, folder Revision_Lord_Gravity), first part: the pair of two
brochon quasi-particles bound by the exchange of the zeugon, computed exactly (sympy) to the order alpha^4.

THE SYSTEM.  By Theorem L2 of Part III the quasi-particle of the condensed vacuum of the earlier hypotheses is, in every
gravitational field and every field of the zeugon, a particle of spin 1/2 and mass E_gap = E^{CJT,gap}_SF that couples to
the zeugon with the charges +1/2 or -1/2 times g^v_LL, its equation of the second order being that of a Dirac particle
(gyromagnetic factor 2).  Two quasi-particles of opposite charges attract: the exchange of the massless zeugon gives the
potential -alpha_Z/r with
    alpha_Z = (g^v_LL/2)^2 / (4 pi) = (g^v_LL)^2 / (16 pi)        (Heaviside-Lorentz units, hbar = c = 1).
The pair is hydrogen-like (reduced mass m/2 for two quasi-particles of mass m).  Its levels to the order alpha^4 follow
from the Breit-Fermi Hamiltonian of two Dirac particles with a massless vector exchange (the Hamiltonian of positronium),
in first-order perturbation theory in the hydrogen-like states |n l s j>:
    H = p^2/(2 mu) - alpha/r                                                   (mu = m1 m2/(m1 + m2))
      - (p^4/8)(1/m1^3 + 1/m2^3)                                               (relativistic kinetic energy)
      + (pi alpha/2)(1/m1^2 + 1/m2^2) delta^3(r)                               (Darwin term)
      - (alpha/(2 m1 m2)) p_i [(delta_ij + n_i n_j)/r] p_j                     (orbit-orbit: magnetic and retarded exchange)
      + (alpha/r^3)[(1/(2 m1^2) + 1/(m1 m2)) L.S1 + (1/(2 m2^2) + 1/(m1 m2)) L.S2]   (spin-orbit, g = 2)
      + (8 pi alpha/(3 m1 m2)) S1.S2 delta^3(r) + (alpha/(m1 m2 r^3))[3 (S1.n)(S2.n) - S1.S2]   (spin-spin)
    and, for a particle and its antiparticle only (positronium), the virtual annihilation into one vector boson,
      + (pi alpha/(2 m^2))(3 + sigma1.sigma2) delta^3(r).
Whether two quasi-particles of opposite charge can annihilate into zeugons is an open question of the condensed vacuum
(task record, E20); the levels are given without and with this term.

CHECKS (identifiers YP; this program is the exact reference of stage 7a, the Rust program lg_pair and the Wolfram script
lg_pair.wls check it):
  YP-1  one particle in a fixed Coulomb field (m2 -> infinity): the operators above give the fine structure of Dirac's
        theory, -(m alpha^4/(2 n^4))(n/(j + 1/2) - 3/4), for n <= 5 and every l, j                (exact)
  YP-2  two equal masses with the annihilation term (positronium): Ferrell's formula, E^(4) = (m alpha^4/2)[11/(32 n^4)
        + (eps_lsj - 1/(2l + 1))/n^3] with eps = 0 for s = 0, 7/6 for 3S1, and for s = 1, l >= 1:
        (3l + 4)/((l + 1)(2l + 3))/(2(2l + 1)), -1/(l(l + 1))/(2(2l + 1)), -(3l - 1)/(l(2l - 1))/(2(2l + 1)) for
        j = l + 1, l, l - 1, for n <= 5 (exact)
  YP-3  the radial integrals of the hydrogen-like states used here (normalization, <1/r>, <1/r^2>, <1/r^3>, |psi(0)|^2)
        equal the closed formulas, for n <= 5 (exact)
Output: data/lg_pair_levels.csv (the coefficients of m alpha^4 for n <= 4, with and without annihilation),
data/hck_lg_pair_checks_python.txt.
Run (from the folder Revision_Lord_Gravity):   python3 python/lg_pair.py
"""
import csv
import os

import sympy as sp

from lgcommon import DATA, Report

r = sp.symbols('r', positive=True)
NMAX = 5


def radial(n, l, a=1):
    """the normalized radial function R_nl(r) of the hydrogen-like problem with the Bohr radius a"""
    rho = 2 * r / (n * a)
    norm = sp.sqrt((2 / (n * a)) ** 3 * sp.factorial(n - l - 1) / (2 * n * sp.factorial(n + l)))
    return sp.simplify(norm * sp.exp(-rho / 2) * rho ** l * sp.assoc_laguerre(n - l - 1, 2 * l + 1, rho))


def integ(e):
    return sp.simplify(sp.integrate(sp.expand(e), (r, 0, sp.oo)))


def expectations(n, l, mu, al):
    """the expectation values needed, for reduced mass mu and coupling al (Bohr radius a = 1/(mu al))"""
    a = 1 / (mu * al)
    R = radial(n, l, a)
    ev = {"norm": integ(R ** 2 * r ** 2), "inv_r": integ(R ** 2 * r), "inv_r2": integ(R ** 2)}
    ev["inv_r3"] = integ(R ** 2 / r) if l > 0 else None
    ev["psi0_sq"] = (sp.limit(R, r, 0) ** 2 / (4 * sp.pi)) if l == 0 else sp.Integer(0)
    # the orbit-orbit operator p_i [(delta_ij + n_i n_j)/r] p_j: int (2 r R'^2 + l(l+1) R^2/r) dr
    Rp = sp.diff(R, r)
    ev["oo"] = integ(2 * r * Rp ** 2 + (l * (l + 1) * R ** 2 / r if l > 0 else 0))
    En = -mu * al ** 2 / (2 * n ** 2)
    # <p^4> = (2 mu)^2 <(E_n + al/r)^2>
    ev["p4"] = sp.simplify(4 * mu ** 2 * (En ** 2 + 2 * En * al * ev["inv_r"] + al ** 2 * ev["inv_r2"]))
    return ev


def tensor_avg(l, j):
    """<3(S1.n)(S2.n) - S1.S2> in the state l, s = 1, j: one quarter of <S12>, S12 = 3(sigma1.n)(sigma2.n) - sigma1.sigma2"""
    if l == 0:
        return sp.Integer(0)
    if j == l + 1:
        s12 = sp.Rational(-2 * l, 2 * l + 3)
    elif j == l:
        s12 = sp.Integer(2)
    else:
        s12 = sp.Rational(-2 * (l + 1), 2 * l - 1)
    return s12 / 4


def pair_levels(n, l, s, j, m, al, annihilation):
    """the energy of the order alpha^4 of the state n l s j of two particles of the same mass m (exact)"""
    mu = m / 2
    ev = expectations(n, l, mu, al)
    e = -ev["p4"] / 8 * (2 / m ** 3)
    e += sp.pi * al / 2 * (2 / m ** 2) * ev["psi0_sq"]
    e += -al / (2 * m * m) * ev["oo"]
    s1s2 = (sp.Rational(s * (s + 1)) - sp.Rational(3, 2)) / 2
    if l > 0:
        ls = (sp.Rational(j * (j + 1)) - l * (l + 1) - s * (s + 1)) / 2
        e += al * ev["inv_r3"] * (sp.Rational(1, 2) + 1) / m ** 2 * ls
        if s == 1:
            e += al / (m * m) * ev["inv_r3"] * tensor_avg(l, j)
    e += 8 * sp.pi * al / (3 * m * m) * s1s2 * ev["psi0_sq"]
    if annihilation:
        sig = 4 * s1s2
        e += sp.pi * al / (2 * m ** 2) * (3 + sig) * ev["psi0_sq"]
    return sp.simplify(e)


def ferrell(n, l, s, j, m, al):
    """Ferrell's formula for positronium (with the annihilation term)"""
    if s == 0:
        eps = 0
    elif l == 0:
        eps = sp.Rational(7, 6)
    else:
        if j == l + 1:
            f = sp.Rational(3 * l + 4, (l + 1) * (2 * l + 3))
        elif j == l:
            f = sp.Rational(-1, l * (l + 1))
        else:
            f = sp.Rational(-(3 * l - 1), l * (2 * l - 1))
        eps = f / (2 * (2 * l + 1))
    return m * al ** 4 / 2 * (sp.Rational(11, 32) / n ** 4 + (eps - sp.Rational(1, 2 * l + 1)) / n ** 3)


def states(nmax):
    for n in range(1, nmax + 1):
        for l in range(n):
            for s in (0, 1):
                js = [l] if s == 0 else ([1] if l == 0 else [l + 1, l, l - 1])
                for j in js:
                    yield n, l, s, j


def label(n, l, s, j):
    return "%d%s%s%s" % (n, "^%d" % (2 * s + 1), "SPDFGH"[l], j)


def main():
    rep = Report("lg_pair.py: stage 7a of task C23 - the pair of two brochon quasi-particles bound by the zeugon, exact "
                 "levels of the order alpha^4 (Revision_Lord_Gravity, PROJECT spacetimes)",
                 "the identifier of the check (YP and a number)")
    m, al = sp.symbols('m alpha', positive=True)

    # ---- YP-3: the radial integrals
    ok3, det3 = True, []
    for n in range(1, NMAX + 1):
        for l in range(n):
            ev = expectations(n, l, sp.Integer(1), sp.Integer(1))
            want = {"norm": 1, "inv_r": sp.Rational(1, n * n), "inv_r2": sp.Rational(2, n ** 3 * (2 * l + 1))}
            if l > 0:
                want["inv_r3"] = sp.Rational(2, n ** 3 * l * (2 * l + 1) * (l + 1))
            if l == 0:
                want["psi0_sq"] = 1 / (sp.pi * n ** 3)
            for k, w in want.items():
                if sp.simplify(ev[k] - w) != 0:
                    ok3 = False
                    det3.append("%d%d %s: %s vs %s" % (n, l, k, ev[k], w))
    rep.add_bool("YP-3", "the radial functions of the hydrogen-like states (n <= 5, a = 1) are normalized and give <1/r> = "
                 "1/n^2, <1/r^2> = 2/(n^3 (2l+1)), <1/r^3> = 2/(n^3 l (2l+1)(l+1)) (l >= 1) and |psi(0)|^2 = 1/(pi n^3) "
                 "(l = 0) (sympy, exact)", ok3, "; ".join(det3) if det3 else "all equal")

    # ---- YP-1: one particle in a fixed Coulomb field, m2 -> infinity
    ok1, det1 = True, []
    for n in range(1, NMAX + 1):
        for l in range(n):
            for j2 in ([1] if l == 0 else [2 * l + 1, 2 * l - 1]):
                j = sp.Rational(j2, 2)
                ev = expectations(n, l, m, al)
                e = -ev["p4"] / (8 * m ** 3) + sp.pi * al / (2 * m ** 2) * ev["psi0_sq"]
                if l > 0:
                    ls = (j * (j + 1) - l * (l + 1) - sp.Rational(3, 4)) / 2
                    e += al / (2 * m ** 2) * ev["inv_r3"] * ls
                dirac = -m * al ** 4 / (2 * n ** 4) * (n / (j + sp.Rational(1, 2)) - sp.Rational(3, 4))
                if sp.simplify(e - dirac) != 0:
                    ok1 = False
                    det1.append("n %d l %d j %s: %s vs %s" % (n, l, j, sp.simplify(e), dirac))
    rep.add_bool("YP-1", "one particle of spin 1/2 in a fixed Coulomb field (the limit m2 -> infinity of the Hamiltonian): the "
                 "relativistic kinetic energy, the Darwin term and the spin-orbit term give Dirac's fine structure "
                 "-(m alpha^4/(2 n^4))(n/(j + 1/2) - 3/4) for every state with n <= 5 (sympy, exact)",
                 ok1, "; ".join(det1) if det1 else "all equal")

    # ---- YP-2: positronium, Ferrell's formula
    ok2, det2 = True, []
    for st in states(NMAX):
        got = pair_levels(*st, m, al, True)
        want = ferrell(*st, m, al)
        if sp.simplify(got - want) != 0:
            ok2 = False
            det2.append("%s: %s vs %s" % (label(*st), sp.simplify(got / (m * al ** 4)), sp.simplify(want / (m * al ** 4))))
    rep.add_bool("YP-2", "two particles of the same mass with the term of virtual annihilation (positronium): the levels of the "
                 "order alpha^4 of every state with n <= 5 equal Ferrell's formula (sympy, exact); in particular the hyperfine "
                 "splitting of the ground state is (7/12) m alpha^4, of which (1/3) m alpha^4 is the magnetic interaction and "
                 "(1/4) m alpha^4 the annihilation", ok2, "; ".join(det2) if det2 else "all equal")

    # ---- the table of the coefficients for n <= 4 (units m alpha^4), without and with annihilation
    rows = []
    for st in states(4):
        c0 = sp.nsimplify(sp.simplify(pair_levels(*st, m, al, False) / (m * al ** 4)))
        c1 = sp.nsimplify(sp.simplify(pair_levels(*st, m, al, True) / (m * al ** 4)))
        n, l, s, j = st
        rows.append([label(*st), n, l, s, j, str(sp.Rational(-1, 4 * n * n)), str(c0), "%.17e" % float(c0), str(c1), "%.17e" % float(c1)])
    with open(os.path.join(DATA, "lg_pair_levels.csv"), "w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(["state", "n", "l", "s", "j", "E2_over_m_alpha2", "E4_over_m_alpha4_without_annihilation",
                    "E4_without_annihilation_float", "E4_over_m_alpha4_with_annihilation", "E4_with_annihilation_float"])
        w.writerows(rows)
    rep.note("the coefficients of m alpha^4 (without / with annihilation): " +
             "; ".join("%s %s / %s" % (x[0], x[6], x[8]) for x in rows if x[1] <= 2))
    rep.write("hck_lg_pair_checks_python.txt")
    print("lg_pair.py: %d of %d checks pass" % (rep.npass, rep.n))


if __name__ == "__main__":
    main()
