#!/usr/bin/env python3
"""t2_formula_check.py -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, item [1], checks GS-18 and GS-20.

An INDEPENDENT computation of the errors of the linear multistep formulas that the Rust library t2steps (module
formulas) computes with double-double numbers.  This script uses the multiple-precision arithmetic of the Python
library mpmath (40 decimal digits) and its own root finders, shares no code with the Rust program, and compares:

  FC-1  the coefficients of the formulas with the series that define them: the Adams-Moulton coefficients are the
        coefficients of the powers of u in -u/ln(1-u), the Adams-Bashforth coefficients those of -u/((1-u) ln(1-u));
        both are compared with the known fractions, and their difference with the identity c_AM(n) = c_AB(n) - c_AB(n-1);
  FC-2  every number of the file test2/data/t2_steps_formulas.csv, the relative error eps_step of one step of a formula
        on the oscillation exp(z n): the Adams-Moulton formulas of the orders 1 to 12 with the implicit equation solved
        exactly and with 1, 2 and 3 fixed-point iterations, the backward differentiation formulas of the orders 1 to 5
        with 1 and 2 Newton steps and five ratios cj/cjJ;
  FC-3  the same errors by a second route that uses no equation for a root at all: the recurrence of the formula is
        applied step by step to the sequence of the values, and the factor of one step is read off after many steps;
  FC-4  the leading terms: eps_step = -c_AM(q) z^(q+1) for the Adams-Moulton formula of the order q solved exactly,
        z^(k+1)/(k+1) for the backward differentiation formula of the order k, and
        (1/(k+1) + ((rho-1)/rho) H_k) z^(k+1) for the latter with one Newton step and an old matrix;
  FC-5  the numbers of the file test2/data/t2_steps_roots.csv: the largest modulus of a root other than the principal
        one, for the backward differentiation formulas with one Newton step and an old matrix (here: the roots of the
        characteristic polynomial) and for the Adams-Moulton formulas with 1 and 2 fixed-point iterations (here by
        another route than in the Rust library: the eigenvalues of the matrix that advances the values y_n, G_n, ...,
        G_(n-q+1) by one step), and the alternating root of the former;
  FC-6  the numbers of the file test2/data/t2_steps_limits.csv: the limits of stability on the imaginary axis of the
        Adams-Moulton formulas with 1 iteration (orders 1 to 12) and with 2 iterations (orders 5 to 8), and the interval of the ratio cj/cjJ in which the backward
        differentiation formulas with one Newton step are stable; for the orders 4 and 5 the upper ends are 25/17 and
        137/105 exactly, the ratios at which the alternating root is -1.

Usage (from the folder Revision_SuperLoop):   python3 test2/python/t2_formula_check.py
Output: test2/data/t2_py_formula_checks.txt; exit status 1 if a check fails.
Requirements: Python 3 with mpmath (it is installed with sympy, which the folder already uses).

THE FORMULAS (z = lambda h for the equation y' = lambda y; kappa the factor of one step; u = 1 - 1/kappa; a backward
difference multiplies a geometric sequence by u):
  Adams-Moulton, order q, solved exactly:      u = z (c_AM(0) + c_AM(1) u + ... + c_AM(q-1) u^(q-1))
  backward differentiation, order k, exactly:  u + u^2/2 + ... + u^k/k = z
  The exact solution has u = 1 - exp(-z), that is -ln(1-u) = z.  Both formulas are truncations of this relation:
  -ln(1-u) = u + u^2/2 + ... and z = -ln(1-u) = u / (-u/ln(1-u)) = u / (c_AM(0) + c_AM(1) u + ...).
  CVODE with m fixed-point iterations (values y_n, stored derivatives G_n = h times the derivative):
      P    = y_(n-1) + sum_(i=1..q) b_AB(i) G_(n-i)                (Adams-Bashforth predictor)
      base = y_(n-1) + sum_(i=1..q-1) b_AM(i) G_(n-i)
      y^(0) = P,  y^(j+1) = base + b_AM(0) z y^(j),  y_n = y^(m),  G_n = z y^(m-1)
  IDA with m Newton steps and a matrix made for cjJ = cj/r (cj = H_k/h):
      P = the polynomial through the last k+1 values, at the new time;  Y = the exact solution of the formula;
      y_n = P + rho (Y - P),  rho = 1 - (1 - rho_1)^m,  rho_1 = (2/(1+r)) (H_k - z)/(H_k/r - z).
"""
import csv
import os
import sys

import mpmath as mp

sys.dont_write_bytecode = True
mp.mp.dps = 40
HERE = os.path.dirname(os.path.abspath(__file__))
DATA = os.path.join(HERE, "..", "data")


def harmonic(k):
    return sum(mp.mpf(1) / j for j in range(1, k + 1))


def series_coefficients(f, n):
    """The coefficients of u^0 ... u^n of the function f(u) at u = 0 (mpmath.taylor differentiates numerically)."""
    return mp.taylor(f, 0, n)


def c_am(n):
    """c_AM(0..n): the coefficients of -u/ln(1-u); at u = 0 the function has the limit 1."""
    f = lambda u: mp.mpf(1) if u == 0 else -u / mp.log(1 - u)
    with mp.workdps(80):
        return [+x for x in series_coefficients(f, n)]


def c_ab(n):
    """c_AB(0..n): the coefficients of -u/((1-u) ln(1-u))."""
    f = lambda u: mp.mpf(1) if u == 0 else -u / ((1 - u) * mp.log(1 - u))
    with mp.workdps(80):
        return [+x for x in series_coefficients(f, n)]


def weights(q, cam, cab):
    """b_AM(0..q-1) and b_AB(1..q) (index 0 unused): the formulas of the order q written in the values of the derivative.
    A sum of backward differences sum_j c_j nabla^j f_n is sum_i [(-1)^i sum_(j>=i) binomial(j, i) c_j] f_(n-i)."""
    bam = [(-1) ** i * sum(mp.binomial(j, i) * cam[j] for j in range(i, q)) for i in range(q)]
    bab = [None] + [(-1) ** (i - 1) * sum(mp.binomial(j, i - 1) * cab[j] for j in range(i - 1, q)) for i in range(1, q + 1)]
    return bam, bab


CAM = c_am(13)
CAB = c_ab(13)


def am_eps(q, z, m):
    """Adams-Moulton, order q; m = 0: solved exactly; m >= 1: m fixed-point iterations as in CVODE."""
    if m == 0:
        u = mp.findroot(lambda u: u - z * sum(CAM[j] * u ** j for j in range(q)), z)
        return mp.exp(-z) / (1 - u) - 1
    bam, bab = weights(q, CAM, CAB)

    def equations(x, g):
        sab = sum(bab[i] * x ** i for i in range(1, q + 1))
        sam = sum(bam[i] * x ** i for i in range(1, q))
        p = x + g * sab
        yc = (x + g * sam) / (1 - bam[0] * z)
        return (yc + (bam[0] * z) ** m * (p - yc) - 1, z * (yc + (bam[0] * z) ** (m - 1) * (p - yc)) - g)

    x, g = mp.findroot(equations, (mp.exp(-z), z))
    return mp.exp(-z) / x - 1


def bdf_rho(k, z, r, m):
    h = harmonic(k)
    rho1 = (mp.mpf(2) / (1 + r)) * (h - z) / (h / r - z)
    return 1 - (1 - rho1) ** m


def bdf_eps(k, z, r, m):
    h = harmonic(k)
    rho = bdf_rho(k, z, r, m)
    c = (1 - rho) / rho * (h - z)
    u = mp.findroot(lambda u: sum(u ** j / j for j in range(1, k + 1)) - z + c * u ** (k + 1), z)
    return mp.exp(-z) / (1 - u) - 1


def am_eps_by_steps(q, z, m, nsteps=400):
    """The same error from the recurrence applied step by step (m >= 1), starting from the exact solution."""
    bam, bab = weights(q, CAM, CAB)
    y = mp.mpc(1)
    g = [z * mp.exp(-i * z) for i in range(q)]
    ratio = None
    for _ in range(nsteps):
        p = y + sum(bab[i] * g[i - 1] for i in range(1, q + 1))
        base = y + sum(bam[i] * g[i - 1] for i in range(1, q))
        yj = prev = p
        for _j in range(m):
            prev = yj
            yj = base + bam[0] * z * prev
        ratio = yj / y
        y = yj
        g = [z * prev] + g[:-1]
    return ratio * mp.exp(-z) - 1


def bdf_eps_by_steps(k, z, r, m, nsteps=400):
    """The same error from the recurrence of IDA applied step by step, starting from the exact solution."""
    h = harmonic(k)
    rho = bdf_rho(k, z, r, m)
    # the backward differentiation formula in the values: sum_(j=0..k) a_j y_(n-j) = z y_n, a_0 = H_k
    a = [(-1) ** j * sum(mp.binomial(i, j) / i for i in range(max(j, 1), k + 1)) for j in range(k + 1)]
    ys = [mp.exp(-i * z) for i in range(k + 1)]          # y_(n-1), ..., y_(n-1-k)
    ratio = None
    for _ in range(nsteps):
        pred = sum((-1) ** (i + 1) * mp.binomial(k + 1, i) * ys[i - 1] for i in range(1, k + 2))
        exact = -sum(a[j] * ys[j - 1] for j in range(1, k + 1)) / (a[0] - z)
        new = pred + rho * (exact - pred)
        ratio = new / ys[0]
        ys = [new] + ys[:-1]
    return ratio * mp.exp(-z) - 1


def bdf_all_roots(k, r):
    """All roots kappa of the recurrence of IDA with one Newton step at z = 0."""
    h = harmonic(k)
    rho = bdf_rho(k, mp.mpf(0), r, 1)
    c = (1 - rho) / rho * h
    coefficients = ([c] if abs(c) > mp.mpf(10) ** -30 else []) + [mp.mpf(1) / j for j in range(k, 0, -1)] + [mp.mpf(0)]   # highest power first
    return [1 / (1 - u) for u in mp.polyroots(coefficients, maxsteps=400, extraprec=400)]


def alternating_root(k, r):
    """The root kappa on the negative real axis of the recurrence of IDA with one Newton step, z = 0 (0 if none)."""
    best = mp.mpf(0)
    for kappa in bdf_all_roots(k, r):
        if abs(mp.im(kappa)) < mp.mpf(10) ** -20 and mp.re(kappa) < 0 and abs(kappa) > abs(best):
            best = mp.re(kappa)
    return best


def bdf_other_root(k, r):
    """The largest modulus of a root other than the principal one (which is 1 at z = 0)."""
    roots = bdf_all_roots(k, r)
    principal = min(range(len(roots)), key=lambda i: abs(roots[i] - 1))
    return max(abs(x) for i, x in enumerate(roots) if i != principal)


def adams_step_matrix(q, z, m):
    """The matrix that advances the column (y_n, G_n, G_(n-1), ..., G_(n-q+1)) by one step of CVODE with m iterations."""
    bam, bab = weights(q, CAM, CAB)
    size = q + 1
    out = mp.matrix(size, size)
    for col in range(size):
        y = mp.mpf(1) if col == 0 else mp.mpf(0)
        g = [mp.mpf(1) if col == i + 1 else mp.mpf(0) for i in range(q)]
        p = y + sum(bab[i] * g[i - 1] for i in range(1, q + 1))
        base = y + sum(bam[i] * g[i - 1] for i in range(1, q))
        yj = prev = p
        for _j in range(m):
            prev = yj
            yj = base + bam[0] * z * prev
        new = [yj, z * prev] + g[:-1]
        for row in range(size):
            out[row, col] = new[row]
    return out


def adams_other_root(q, za, m):
    """The largest modulus of an eigenvalue of the step matrix other than the one nearest to exp(z), z = -i za."""
    z = mp.mpc(0, -za)
    values = mp.eig(adams_step_matrix(q, z, m), left=False, right=False)
    principal = min(range(len(values)), key=lambda i: abs(values[i] - mp.exp(z)))
    return max(abs(x) for i, x in enumerate(values) if i != principal)


def adams_limit(q, m):
    """The smallest |z| at which another root reaches the modulus 1 (steps of 0.002, then bisection); 1 if none below 1.
    For z -> 0 all other roots tend to zero; for the high orders with one iteration the limit lies below the first
    point of the search and is found by bisection between 1e-6 and that point."""
    lo = mp.mpf("0.002")
    if adams_other_root(q, lo, m) >= 1:
        stable, bad = mp.mpf("1e-6"), lo
        if adams_other_root(q, stable, m) >= 1:
            return mp.mpf(0)
        for _ in range(40):
            mid = (stable + bad) / 2
            if adams_other_root(q, mid, m) >= 1:
                bad = mid
            else:
                stable = mid
        return (stable + bad) / 2
    hi = lo + mp.mpf("0.002")
    while hi <= 1:
        if adams_other_root(q, hi, m) >= 1:
            for _ in range(40):
                mid = (lo + hi) / 2
                if adams_other_root(q, mid, m) >= 1:
                    hi = mid
                else:
                    lo = mid
            return (lo + hi) / 2
        lo = hi
        hi += mp.mpf("0.002")
    return mp.mpf(1)


def read_csv(name):
    with open(os.path.join(DATA, name), encoding="utf-8") as f:
        lines = [l for l in f if not l.startswith("#")]
    return list(csv.DictReader(lines))


def main():
    checks = []

    def check(cid, statement, ok, detail):
        checks.append((cid, statement, bool(ok), detail))
        print("[%s] %s  %s" % ("PASS" if ok else "FAIL", cid, detail))

    # ---------------------------------------------------------------- FC-1
    fractions_am = [(1, 1), (-1, 2), (-1, 12), (-1, 24), (-19, 720), (-3, 160), (-863, 60480), (-275, 24192)]
    fractions_ab = [(1, 1), (1, 2), (5, 12), (3, 8), (251, 720), (95, 288), (19087, 60480), (5257, 17280)]
    dev_am = max(abs(CAM[n] - mp.mpf(a) / b) for n, (a, b) in enumerate(fractions_am))
    dev_ab = max(abs(CAB[n] - mp.mpf(a) / b) for n, (a, b) in enumerate(fractions_ab))
    dev_id = max(abs(CAM[n] - (CAB[n] - CAB[n - 1])) for n in range(1, 13))
    # the weights of every order add up to 1 (the formulas integrate a constant derivative exactly)
    dev_sum = mp.mpf(0)
    for q in range(1, 13):
        bam, bab = weights(q, CAM, CAB)
        dev_sum = max(dev_sum, abs(sum(bam) - 1), abs(sum(bab[1:]) - 1))
    check("FC-1", "the coefficients c_AM(0..7) of the series of -u/ln(1-u) are 1, -1/2, -1/12, -1/24, -19/720, -3/160, -863/60480, -275/24192, the coefficients c_AB(0..7) of -u/((1-u) ln(1-u)) are 1, 1/2, 5/12, 3/8, 251/720, 95/288, 19087/60480, 5257/17280, c_AM(n) = c_AB(n) - c_AB(n-1) for n = 1 to 12, and the weights b_AM and b_AB of every order from 1 to 12 add up to 1 (tolerance 1e-30)",
          dev_am < 1e-30 and dev_ab < 1e-30 and dev_id < 1e-30 and dev_sum < 1e-30,
          "largest deviations %s, %s, %s, %s" % (mp.nstr(dev_am, 2), mp.nstr(dev_ab, 2), mp.nstr(dev_id, 2), mp.nstr(dev_sum, 2)))

    # ---------------------------------------------------------------- FC-2 and FC-3
    rows = read_csv("t2_steps_formulas.csv")
    worst_abs, worst_rel, worst_row = mp.mpf(0), mp.mpf(0), None
    worst_steps = mp.mpf(0)
    n_steps_compared = 0
    for r in rows:
        # the ratio and |z| as the double-precision numbers that the Rust program used (the file prints them with
        # enough digits to determine the double)
        q, m, ratio, za = int(r["order"]), int(r["iterations"]), mp.mpf(float(r["cj_ratio"])), mp.mpf(float(r["z_abs"]))
        z = mp.mpc(0, -za)
        # a double-double number is the exact sum of two double-precision numbers; each is printed with 17 significant
        # digits, which determine the double, so it is read as a double first and then added with 40 digits
        dd = lambda name: mp.mpf(float(r[name + "_hi"])) + mp.mpf(float(r[name + "_lo"]))
        rust = mp.mpc(dd("eps_re"), dd("eps_im"))
        here = am_eps(q, z, m) if r["formula"] == "AM" else bdf_eps(q, z, ratio, m)
        d = abs(rust - here)
        if d > worst_abs:
            worst_abs, worst_row = d, r
        if abs(here) > mp.mpf(10) ** -24:
            worst_rel = max(worst_rel, d / abs(here))
        # the second route, for the cases in which the other roots of the recurrence are small enough to be forgotten
        # after 400 steps (not for the nearly unstable or unstable ratio 1.3129 of the backward differentiation formulas)
        slow = r["z_abs"] in ("0.02", "0.08")
        if slow and ((r["formula"] == "AM" and q <= 8 and (m >= 2 or (m == 1 and q <= 4))) or (r["formula"] == "BDF" and mp.mpf("0.9") < ratio < mp.mpf("1.2"))):
            by_steps = am_eps_by_steps(q, z, m) if r["formula"] == "AM" else bdf_eps_by_steps(q, z, ratio, m)
            if abs(here) > mp.mpf(10) ** -24:
                worst_steps = max(worst_steps, abs(by_steps - here) / abs(here))
                n_steps_compared += 1
    check("FC-2", "the %d relative errors eps_step of the file t2_steps_formulas.csv (double-double numbers of the Rust library t2steps) agree with the values computed here with 40 digits: absolute difference at most 3e-30, and relative difference at most 1e-6 wherever |eps_step| exceeds 1e-24" % len(rows),
          worst_abs < mp.mpf("3e-30") and worst_rel < mp.mpf("1e-6"),
          "largest absolute difference %s (formula %s, order %s, iterations %s, cj_ratio %s, |z| = %s), largest relative difference %s" % (
              mp.nstr(worst_abs, 2), worst_row["formula"], worst_row["order"], worst_row["iterations"], worst_row["cj_ratio"], worst_row["z_abs"], mp.nstr(worst_rel, 2)))
    check("FC-3", "the factor of one step read off after 400 steps of the recurrence itself (CVODE: predictor, fixed-point iterations, stored derivative; IDA: predictor, Newton steps with the old matrix) agrees with the root of the characteristic equation, for |z| = 0.02 and 0.08, wherever the recurrence is stable enough for the other roots to be forgotten after 400 steps: the Adams-Moulton formulas of the orders 1 to 8 with 2 and 3 iterations and of the orders 1 to 4 with 1 iteration, and the backward differentiation formulas of the orders 1 to 5 with the ratios cj/cjJ = 1, 10/9 and 1.164 (relative difference of eps_step at most 1e-9)",
          worst_steps < mp.mpf("1e-9") and n_steps_compared >= 90,
          "%d cases, largest relative difference %s" % (n_steps_compared, mp.nstr(worst_steps, 2)))

    # ---------------------------------------------------------------- FC-4
    z = mp.mpc(0, -mp.mpf("0.001"))
    lead = mp.mpf(0)
    for q in range(1, 9):
        lead = max(lead, abs(am_eps(q, z, 0) / (-CAM[q] * z ** (q + 1)) - 1))
    for k in range(1, 6):
        lead = max(lead, abs(bdf_eps(k, z, 1, 1) / (z ** (k + 1) / (k + 1)) - 1))
        for ratio in (mp.mpf(10) / 9, mp.mpf("1.164068524711"), mp.mpf("1.312865875279")):
            rho = 2 * ratio / (1 + ratio)
            constant = mp.mpf(1) / (k + 1) + (rho - 1) / rho * harmonic(k)
            lead = max(lead, abs(bdf_eps(k, z, ratio, 1) / (constant * z ** (k + 1)) - 1))
    factor = 1 + 6 * ((2 * mp.mpf("1.164068524711") / (1 + mp.mpf("1.164068524711")) - 1) / (2 * mp.mpf("1.164068524711") / (1 + mp.mpf("1.164068524711")))) * harmonic(5)
    check("FC-4", "the leading terms at |z| = 0.001: eps_step = -c_AM(q) z^(q+1) for the Adams-Moulton formulas of the orders 1 to 8 solved exactly, z^(k+1)/(k+1) for the backward differentiation formulas of the orders 1 to 5, and (1/(k+1) + ((rho-1)/rho) H_k) z^(k+1) with rho = 2r/(1+r) for one Newton step and the ratios r = 10/9, 1.164068524711, 1.312865875279 (relative deviation at most 0.01, of the order of |z| times a constant); for the order 5 and r = 1.164068524711 the error is 1 + 6 ((rho-1)/rho) H_5 = 1.965 times that of the formula",
          lead < mp.mpf("0.01") and abs(factor - mp.mpf("1.965")) < mp.mpf("0.001"),
          "largest relative deviation %s; factor %s" % (mp.nstr(lead, 2), mp.nstr(factor, 6)))

    # ---------------------------------------------------------------- FC-5
    rows = read_csv("t2_steps_roots.csv")
    worst_bdf, worst_alt, worst_am, n_bdf, n_am = mp.mpf(0), mp.mpf(0), mp.mpf(0), 0, 0
    for r in rows:
        q, m = int(r["order"]), int(r["iterations"])
        if r["formula"] == "BDF":
            ratio = mp.mpf(r["cj_ratio"])
            worst_bdf = max(worst_bdf, abs(bdf_other_root(q, ratio) - mp.mpf(r["other_root"])))
            worst_alt = max(worst_alt, abs(alternating_root(q, ratio) - mp.mpf(r["alternating_root"])))
            n_bdf += 1
        else:
            # every fifth |z| of the file (0.025, 0.05, ..., 0.2): the eigenvalues of the step matrix
            za = mp.mpf(r["z_abs"])
            if int(round(float(r["z_abs"]) * 1000)) % 25 == 0:
                here = adams_other_root(q, za, m)
                worst_am = max(worst_am, abs(here - mp.mpf(r["other_root"])) / here)
                n_am += 1
    check("FC-5", "the file t2_steps_roots.csv: the largest modulus of a root other than the principal one of the backward differentiation formulas of the orders 2 to 5 with one Newton step (%d ratios cj/cjJ from 0.6 to 1.66, z = 0) and their alternating root agree with the roots of the characteristic polynomial computed here (tolerance 2e-9, the file prints 9 decimals); and for the Adams-Moulton formulas of the orders 1 to 12 with 1 and 2 fixed-point iterations it agrees, at |z| = 0.025, 0.05, ..., 0.2 (%d values), with the eigenvalues of the matrix of one step computed here (relative tolerance 1e-6)" % (n_bdf, n_am),
          worst_bdf < mp.mpf("2e-9") and worst_alt < mp.mpf("2e-9") and worst_am < mp.mpf("1e-6") and n_am == 192,
          "largest differences %s (other root) and %s (alternating root) for the backward differentiation formulas, largest relative difference %s for the Adams-Moulton formulas" % (mp.nstr(worst_bdf, 2), mp.nstr(worst_alt, 2), mp.nstr(worst_am, 2)))

    # ---------------------------------------------------------------- FC-6
    rows = read_csv("t2_steps_limits.csv")
    worst_lim, n_lim = mp.mpf(0), 0
    window = {}
    for r in rows:
        q, m = int(r["order"]), int(r["iterations"])
        if r["formula"] == "AM":
            if m == 1 or m == 2 and 5 <= q <= 8:
                worst_lim = max(worst_lim, abs(adams_limit(q, m) - mp.mpf(r["limit_high"])))
                n_lim += 1
        else:
            window[q] = (mp.mpf(r["limit_low"]), mp.mpf(r["limit_high"]))
    # the ends of the windows of the backward differentiation formulas: bisection with the roots computed here
    def end(k, limit):
        if bdf_other_root(k, limit) < 1:
            return limit
        stable, bad = mp.mpf(1), limit
        for _ in range(45):
            mid = (stable + bad) / 2
            if bdf_other_root(k, mid) >= 1:
                bad = mid
            else:
                stable = mid
        return (stable + bad) / 2
    worst_win = mp.mpf(0)
    for k in range(1, 6):
        lo, hi = end(k, mp.mpf("0.6")), end(k, mp.mpf(5) / 3)
        worst_win = max(worst_win, abs(lo - window[k][0]), abs(hi - window[k][1]))
    at_run = alternating_root(5, mp.mpf("1.312865875279"))
    exact = max(abs(window[4][1] - mp.mpf(25) / 17), abs(window[5][1] - mp.mpf(137) / 105))
    check("FC-6", "the file t2_steps_limits.csv: the limits of stability on the imaginary axis of the Adams-Moulton formulas with one iteration (orders 1 to 12) and with two iterations (orders 5 to 8), %d numbers, agree with the limits found here from the eigenvalues of the step matrix (tolerance 1e-5); the intervals of cj/cjJ in which the backward differentiation formulas of the orders 1 to 5 with one Newton step are stable agree with those found here (tolerance 1e-8); the upper ends for the orders 4 and 5 are 25/17 and 137/105 (tolerance 1e-8); the alternating root of the order 5 at the ratio 1.312865875279 of the integration by IDA at rtol = 1e-12 is -1.0171 (tolerance 1e-4)" % n_lim,
          worst_lim < mp.mpf("1e-5") and worst_win < mp.mpf("1e-8") and exact < mp.mpf("1e-8") and abs(at_run + mp.mpf("1.0171")) < mp.mpf("1e-4"),
          "largest differences %s (Adams-Moulton) and %s (backward differentiation); distance of the upper ends from 25/17 and 137/105: %s; window of the order 5: %s to %s; alternating root %s" % (
              mp.nstr(worst_lim, 2), mp.nstr(worst_win, 2), mp.nstr(exact, 2), mp.nstr(window[5][0], 7), mp.nstr(window[5][1], 9), mp.nstr(at_run, 8)))

    npass = sum(1 for c in checks if c[2])
    out = os.path.join(DATA, "t2_py_formula_checks.txt")
    with open(out, "w", encoding="utf-8") as f:
        f.write("Revision_SuperLoop, folder test2, program `t2_formula_check.py` (Test 2, item 1, checks GS-18 and GS-20): the errors of the linear multistep formulas of CVODE and of IDA, computed independently with 40 digits\n")
        f.write("%d of %d checks passed\n\n" % (npass, len(checks)))
        for cid, statement, ok, detail in checks:
            f.write("[%s] %s  %s  -- %s\n" % ("PASS" if ok else "FAIL", cid, statement, detail))
    print("wrote test2/data/t2_py_formula_checks.txt")
    return 0 if npass == len(checks) else 1


if __name__ == "__main__":
    sys.exit(main())
