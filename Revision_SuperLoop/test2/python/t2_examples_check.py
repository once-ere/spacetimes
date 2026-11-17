#!/usr/bin/env python3
"""t2_examples_check.py -- PROJECT spacetimes, Revision_SuperLoop, folder test2, Test 2, items [3] and [5].

Checks the numerical solutions of the seven example problems (test2/examples/*.t2in) against predictions that the
solvers do not compute themselves: the group velocity of the lattice dispersion (2.37), total Andreev reflection
below the gap, the bound states of a kink and an antikink (3.24), (3.29), the lattice gap equation (3.21), the
Wilson shift of the bound states of the self-consistent kink, and the frequency of the amplitude oscillation of the
pair field after a quench from the linearized self-consistent equations.  It also compares the outputs of the Python
solver (item [3]) and of the Rust solver (item [5]) with each other.

Usage:
  python3 t2_examples_check.py PREFIX [OTHER_PREFIX]
PREFIX is the beginning of the output files, for example test2/data/t2_py (the files test2/data/t2_py_<example>_*);
with a second prefix the two sets of outputs are compared.  The report is written to PREFIX_examples_checks.txt.
Requirements: Python 3, numpy, scipy (brentq).
"""

import csv
import math
import os
import sys

import numpy as np
from scipy.optimize import brentq

sys.dont_write_bytecode = True  # no __pycache__ folder next to the scripts
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import t2_solve as T  # noqa: E402  (the lattice dispersion and the input reader)

EXDIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "examples")
NAMES = ["packet_free", "cyclotron", "andreev", "kink_spectrum", "ground_state", "kink_ground_state", "quench"]


def load(path):
    """A CSV file of the solvers: the comment lines (# key: value), the header and the rows (strings)."""
    meta, rows = {}, []
    with open(path, encoding="utf-8") as f:
        for line in f:
            if line.startswith("#"):
                k, _, v = line[1:].partition(":")
                meta[k.strip()] = v.strip()
            else:
                rows.append(line.rstrip("\n").split(","))
    return meta, rows[0], rows[1:]


def col(header, rows, name, conv=float):
    i = header.index(name)
    return np.array([conv(r[i]) for r in rows])


class Rep:
    def __init__(self, title):
        self.title = title
        self.lines = []
        self.checks = []

    def check(self, ok, text):
        self.checks.append((bool(ok), text))
        print("[%s] %s" % ("PASS" if ok else "FAIL", text))

    def add(self, text):
        self.lines.append(text)
        print(text)

    def write(self, path):
        with open(path, "w", encoding="utf-8") as f:
            f.write(self.title + "\n")
            f.write("%d of %d checks passed\n" % (sum(c[0] for c in self.checks), len(self.checks)))
            for line in self.lines:
                f.write(line + "\n")
            f.write("checks:\n")
            for ok, text in self.checks:
                f.write("[%s] %s\n" % ("PASS" if ok else "FAIL", text))
        return all(c[0] for c in self.checks)


def lattice_of(name):
    cfg = T.read_input(os.path.join(EXDIR, name + ".t2in"))
    return cfg, T.Lattice(cfg)


def ex_packet_free(prefix, rep):
    """EX-1: the centre of the free packet moves with the group velocity of (2.37), averaged over the Gaussian
    distribution of momenta exp(-2 sigma^2 (k - q)^2) on the allowed momenta of the ring."""
    cfg, lat = lattice_of("packet_free")
    _, h, r = load(prefix + "_packet_free_series.csv")
    t, x = col(h, r, "x_LL^0"), col(h, r, "x_mean_1")
    L = lat.n[0] * lat.ell
    xu = x[0] + np.concatenate([[0.0], np.cumsum((np.diff(x) + 0.5 * L) % L - 0.5 * L)])
    slope, icpt = np.polyfit(t, xu, 1)
    q, sig, n = cfg["packet_momentum"][0], cfg["packet_width"], lat.n[0]
    ks = [2 * math.pi * (k if k <= n // 2 else k - n) / (n * lat.ell) for k in range(n)]
    wts = np.array([math.exp(-2 * sig ** 2 * (k - q) ** 2) for k in ks])
    vg = np.array([T.grad_eps_lat([k, 0, 0], lat)[0] for k in ks])
    vbar = float(np.sum(wts * vg) / np.sum(wts))
    lin = float(np.max(np.abs(xu - (slope * t + icpt))))
    rep.add("EX-1 packet_free: fitted velocity %.8f; group velocity of (2.37) averaged over the Gaussian distribution of "
            "momenta %.8f; at the central momentum %.8f; largest deviation of the centre from the straight line %.2e"
            % (slope, vbar, T.grad_eps_lat([q, 0, 0], lat)[0], lin))
    rep.check(abs(slope - vbar) < 1e-3 * vbar and lin < 1e-3,
              "EX-1 the free packet moves uniformly with the averaged group velocity of the lattice dispersion (2.37): "
              "relative deviation %.1e (allowed 1e-3: the spinor of the packet varies slightly across its momenta)"
              % (abs(slope - vbar) / vbar))


def ex_cyclotron(prefix, rep):
    """EX-2: the packet in the magnetic field stays on a closed region, and its energy decomposes into the Landau
    levels (3.32); the return time of the centre is compared with the spacings of the levels it occupies."""
    cfg, lat = lattice_of("cyclotron")
    _, h, r = load(prefix + "_cyclotron_series.csv")
    t, x, y, e = col(h, r, "x_LL^0"), col(h, r, "x_mean_1"), col(h, r, "x_mean_2"), col(h, r, "E_expect")
    nrm = col(h, r, "norm")
    B = cfg["g_F"] * cfg["magnetic"][0] + 0.5 * cfg["g_v"] * cfg["magnetic"][1]
    m = cfg["mass"]
    nbar = (e[0] ** 2 - m ** 2) / (2 * abs(B))
    d = np.hypot(x - x[0], y - y[0])
    sel = t > 0.25 * t[-1]
    i = int(np.argmin(np.where(sel, d, np.inf)))
    levels = [math.sqrt(m * m + 2 * k * abs(B)) for k in range(6)]
    rep.add("EX-2 cyclotron: B_up = %.4f, magnetic length %.4f, energy %.6f, mean Landau index (E^2 - m^2)/(2 B_up) = %.3f; "
            "Landau levels (3.32) %s; the centre returns closest to its start at x^0 = %.2f (distance %.3f); the spacings "
            "2 pi/(E_(n+1) - E_n) of the neighbouring levels are %s; the centre stays within x^1 in [%.2f, %.2f], x^2 in "
            "[%.2f, %.2f]" % (B, 1 / math.sqrt(abs(B)), e[0], nbar, ["%.4f" % v for v in levels], t[i], d[i],
                              ["%.2f" % (2 * math.pi / (levels[k + 1] - levels[k])) for k in range(3)],
                              x.min(), x.max(), y.min(), y.max()))
    rep.check(np.max(np.abs(nrm - 1)) < 1e-5 and np.max(np.abs(e - e[0])) < 1e-5,
              "EX-2 norm and energy of the packet in the magnetic field are conserved (largest changes %.1e, %.1e)"
              % (np.max(np.abs(nrm - 1)), np.max(np.abs(e - e[0]))))
    L = (lat.n[0] - 1) * lat.ell
    rep.check(x.min() > 2 and x.max() < L - 2 and y.min() > 2 and y.max() < L - 2,
              "EX-2 the orbit is bounded by the magnetic field and stays away from the walls (more than 2 from each wall)")


def ex_andreev(prefix, rep):
    """EX-3: a particle with an energy inside the gap is reflected completely at the superfluid region; most of it as a
    hole (Andreev reflection), which retraces the path of the particle."""
    cfg, lat = lattice_of("andreev")
    _, h, r = load(prefix + "_andreev_series.csv")
    t, ph, xm, e = col(h, r, "x_LL^0"), col(h, r, "P_hole"), col(h, r, "x_mean_1"), col(h, r, "E_expect")
    _, hf, rf = load(prefix + "_andreev_fields.csv")
    tf = col(hf, rf, "x_LL^0")
    x1 = col(hf, rf, "x_LL^1")
    rho = col(hf, rf, "density_p") + col(hf, rf, "density_h")
    last = tf == tf.max()
    xs = cfg["pair_position"][0]
    delta = cfg["pair_amplitude"]
    vF = 1.0
    xi = vF / delta
    beyond = float(np.sum(rho[last & (x1 > xs + 4 * xi)]))
    imax = int(np.argmax(xm))
    vback = (xm[-1] - xm[imax]) / (t[-1] - t[imax])
    rep.add("EX-3 andreev: energy of the packet %.6f, gap %.3f; the centre turns at x^0 = %.2f (x^1 = %.3f) and then moves "
            "with %.4f; final hole probability (Andreev reflection) %.6f, particle probability (normal reflection) %.6f; "
            "probability beyond x_s + 4 xi at the end %.2e" % (e[0], delta, t[imax], xm[imax], vback, ph[-1], 1 - ph[-1], beyond))
    rep.check(abs(e[0]) < delta and beyond < 1e-4 and vback < 0,
              "EX-3 below the gap (|E| = %.4f < %.2f) the packet is reflected: it moves back (velocity %.4f), and only %.1e of "
              "it is found inside the superfluid region (allowed 1e-4: the components of the packet with energies above the gap, "
              "more than four standard deviations of its energy distribution away, pass the interface)"
              % (abs(e[0]), delta, vback, beyond))
    rep.check(ph[-1] > 0.9, "EX-3 the reflection is mainly Andreev reflection: the reflected packet is a hole with probability "
                            "%.4f" % ph[-1])


def ex_kink_spectrum(prefix, rep):
    """EX-4: the spectrum of the ring with a kink and an antikink: exactly 16 eigenvalues inside the gap of the
    continuum, eight near +E_R and eight near -E_R, with E_R the Rayleigh quotient (3.29) of the analytic bound state,
    within the Kato-Temple bound of check AS-9."""
    cfg, lat = lattice_of("kink_spectrum")
    _, h, r = load(prefix + "_kink_spectrum_spectrum.csv")
    E = col(h, r, "E_qp")
    m, ell, rW = cfg["mass"], lat.ell, lat.wilson
    cont = math.sqrt(m * m + cfg["pair_amplitude"] ** 2)
    ingap = np.sort(E[np.abs(E) < 0.9 * cont])
    x = lat.x[:, 0]
    xa, xb = cfg["pair_position"]
    L = lat.n[0] * ell
    # the profile of the two walls: cosh^-1 of the distance to the nearer wall (Delta_inf ell_K = 1)
    dist = np.minimum(np.abs((x - xa + 0.5 * L) % L - 0.5 * L), np.abs((x - xb + 0.5 * L) % L - 0.5 * L))
    phi = 1 / np.cosh(dist)
    ER = m + rW / (2 * ell) * float(np.sum((np.roll(phi, -1) - phi) ** 2)) / float(np.sum(phi ** 2))
    dev = float(np.max(np.abs(np.abs(ingap) - ER)))
    rep.add("EX-4 kink_spectrum: %d eigenvalues with |E| < 0.9 (m^2 + Delta_inf^2)^(1/2) = %.4f: %s; the Rayleigh quotient "
            "(3.29) of the analytic bound state on this lattice is E_R = %.6f" % (len(ingap), 0.9 * cont,
                                                                               " ".join("%.5f" % v for v in ingap), ER))
    rep.check(len(ingap) == 16 and np.sum(ingap > 0) == 8 and dev < 0.023,
              "EX-4 exactly 16 bound states (8 at positive, 8 at negative energy), within %.4f of +-E_R (the Kato-Temple bound "
              "0.023 of check AS-9)" % dev)


def gap_root(lat, G):
    """the root of the lattice gap equation (3.21), by bisection (independent of the solvers)"""
    n = lat.n[0]
    eps = [T.eps_lat([2 * math.pi * (k if k <= n // 2 else k - n) / (n * lat.ell), 0, 0], lat) for k in range(n)]

    def f(D):
        return sum(1 / math.hypot(e - lat.mu, D) + 1 / math.hypot(e + lat.mu, D) for e in eps) / (n * lat.ell) - 1 / G
    return brentq(f, 1e-9, 50.0, xtol=1e-15)


def ex_ground_state(prefix, rep):
    """EX-5: the self-consistent uniform superfluid is the root of the lattice gap equation (3.21)."""
    cfg, lat = lattice_of("ground_state")
    _, h, r = load(prefix + "_ground_state_fields.csv")
    st = col(h, r, "stage", str)
    ad = col(h, r, "abs_d")[st == "final"]
    root = gap_root(lat, cfg["coupling"])
    _, hs, rs = load(prefix + "_ground_state_spectrum.csv")
    E = col(hs, rs, "E_qp")
    emin = float(np.min(np.abs(E)))
    n = lat.n[0]
    eps = [T.eps_lat([2 * math.pi * (k if k <= n // 2 else k - n) / (n * lat.ell), 0, 0], lat) for k in range(n)]
    emin_th = min(math.hypot(e - lat.mu, root) for e in eps)
    rep.add("EX-5 ground_state: |d| = %.12f on all sites (spread %.1e); root of (3.21) %.12f; smallest quasi-particle energy "
            "%.10f, predicted min_q E^-(q) = %.10f" % (ad.mean(), ad.max() - ad.min(), root, emin, emin_th))
    rep.check(np.max(np.abs(ad - root)) < 1e-9 and abs(emin - emin_th) < 1e-9,
              "EX-5 the self-consistent pair field is the root of the lattice gap equation (3.21) (deviation %.1e), and the "
              "gap of the spectrum is min_q E^-(q) (deviation %.1e)" % (np.max(np.abs(ad - root)), abs(emin - emin_th)))


def ex_kink_ground_state(prefix, rep):
    """EX-6: the self-consistent kink: its bound states lie at the first-order Wilson shift +-(r_W/(2 dx)) sum (phi(s+1)
    - phi(s))^2 / sum phi(s)^2 of (3.29) with m = 0, phi solving d phi/dx = -Delta phi for the computed profile; the
    profile is compared with the continuum kink Delta_0 tanh(Delta_0 (x - x_c)) of the Gross-Neveu model."""
    cfg, lat = lattice_of("kink_ground_state")
    _, h, r = load(prefix + "_kink_ground_state_fields.csv")
    st = col(h, r, "stage", str)
    x = col(h, r, "x_LL^1")[st == "final"]
    d3 = col(h, r, "d3_re")[st == "final"]
    _, hs, rs = load(prefix + "_kink_ground_state_spectrum.csv")
    E = np.sort(np.abs(col(hs, rs, "E_qp")))
    ell, rW = lat.ell, lat.wilson
    # phi on the sites: log phi decreases by Delta dx between neighbours (trapezoidal rule), phi = 1 at the centre
    lphi = np.concatenate([[0.0], -np.cumsum(0.5 * (d3[1:] + d3[:-1]) * ell)])
    phi = np.exp(lphi - lphi.max())
    ER = rW / (2 * ell) * float(np.sum(np.diff(phi) ** 2)) / float(np.sum(phi ** 2))
    D0 = gap_root(lat, cfg["coupling"])
    xc = 0.5 * (lat.n[0] - 1) * ell
    dhn = D0 * np.tanh(D0 * (x - xc))
    inner = np.abs(x - xc) < 0.5 * xc
    dev = float(np.max(np.abs(d3 - dhn)[inner]))
    rel = abs(E[0] - ER) / ER
    Es = np.sort(col(hs, rs, "E_qp"))
    nb = int(np.sum(np.abs(Es) < 0.5 * D0))
    rep.add("EX-6 kink_ground_state: the 12 smallest |E| are %s; the first-order Wilson shift of (3.29) for the computed "
            "profile is %.6f; the uniform gap of the periodic chain at the same G is Delta_0 = %.6f; largest deviation of the "
            "profile from Delta_0 tanh(Delta_0 (x - x_c)) in the inner half %.4f" % (" ".join("%.5f" % v for v in E[:12]),
                                                                                    ER, D0, dev))
    rep.check(nb == 8 and np.sum(Es[np.abs(Es) < 0.5 * D0] > 0) == 4 and np.max(np.abs(E[:8] - E[0])) < 1e-9 and rel < 0.25,
              "EX-6 the self-consistent kink carries 8 bound states (four at each sign, as (3.28) gives for one wall) at the "
              "energy %.5f, within %.1f per cent of the first-order Wilson shift %.5f (the next order is of relative size "
              "r_W dx Delta_0 = %.2f; allowed 25 per cent)" % (E[0], 100 * rel, ER, rW * ell * D0))


def rpa_frequency(lat, D):
    """The amplitude-mode frequency of the uniform state of a periodic chain with m = mu = 0 from the linearized
    self-consistent equations: (w^2/4 - D^2) sum_(q != 0) 1/(E_q (E_q^2 - w^2/4)) = 1/D, E_q = (eps_lat(q)^2 + D^2)^(1/2),
    the root between 2 D and 2 E_1 (E_1 the smallest E_q with q != 0)."""
    n = lat.n[0]
    Es = [math.hypot(T.eps_lat([2 * math.pi * (k if k <= n // 2 else k - n) / (n * lat.ell), 0, 0], lat), D)
          for k in range(1, n)]
    E1 = min(Es)

    def f(w):
        return (w * w / 4 - D * D) * sum(1 / (E * (E * E - w * w / 4)) for E in Es) - 1 / D
    return brentq(f, 2 * D * (1 + 1e-12), 2 * E1 * (1 - 1e-12), xtol=1e-14), E1


def ex_quench(prefix, rep):
    """EX-7: after a small quench the amplitude of the pair field oscillates with the frequency of the amplitude mode of
    the linearized self-consistent equations around the time-averaged gap; the fermion number is conserved."""
    cfg, lat = lattice_of("quench")
    _, h, r = load(prefix + "_quench_series.csv")
    t, a, dnf = col(h, r, "x_LL^0"), col(h, r, "abs_d_mean"), col(h, r, "N_F_change")
    wfit, amp = T.fit_frequency(t[1:], a[1:], 0.2, 6.0, 5801)
    Dbar = float(a[1:].mean())
    w_rpa, E1 = rpa_frequency(lat, Dbar)
    Di, Df = gap_root(lat, cfg["coupling"]), gap_root(lat, cfg["quench_coupling"])
    rep.add("EX-7 quench: Delta before %.6f (root of (3.21) at G = %g), Delta of the ground state at the new coupling %.6f; "
            "|d| jumps to %.6f at x^0 = 0 (the factor G'/G) and oscillates about the mean %.6f with the amplitude %.5f and "
            "the fitted angular frequency %.5f; amplitude mode of the linearized equations at the mean: %.5f (between 2 Delta "
            "= %.5f and 2 E_1 = %.5f)" % (Di, cfg["coupling"], Df, a[0], Dbar, amp, wfit, w_rpa, 2 * Dbar, 2 * E1))
    rep.check(abs(wfit - w_rpa) < 0.01 * w_rpa, "EX-7 the pair field oscillates with the frequency of the amplitude (Higgs) "
              "mode of the linearized equations: relative deviation %.2e (allowed 1e-2: terms of second order in the "
              "amplitude and the weaker modes)" % (abs(wfit - w_rpa) / w_rpa))
    rep.check(np.max(np.abs(dnf)) < 1e-9, "EX-7 the fermion number is constant along the whole evolution (largest change "
                                          "%.1e)" % np.max(np.abs(dnf)))


def compare(p1, p2, rep):
    """EX-8: the outputs of the two solvers agree."""
    worst = []
    for name in NAMES:
        for suffix in ("_series.csv", "_fields.csv", "_spectrum.csv"):
            f1, f2 = p1 + "_" + name + suffix, p2 + "_" + name + suffix
            if not os.path.exists(f1):
                continue
            _, h1, r1 = load(f1)
            _, h2, r2 = load(f2)
            if h1 != h2 or len(r1) != len(r2):
                worst.append((name + suffix, float("inf"), "different columns or rows"))
                continue
            dmax, cmax = 0.0, ""
            for j, c in enumerate(h1):
                try:
                    a = np.array([float(r[j]) for r in r1])
                    b = np.array([float(r[j]) for r in r2])
                except ValueError:
                    if [r[j] for r in r1] != [r[j] for r in r2]:
                        dmax, cmax = float("inf"), c
                    continue
                dd = float(np.max(np.abs(a - b))) if len(a) else 0.0
                if dd > dmax:
                    dmax, cmax = dd, c
            worst.append((name + suffix, dmax, cmax))
    for f, dd, c in worst:
        rep.add("EX-8 %s: largest difference %.2e (column %s)" % (f, dd, c))
    big = max(dd for _, dd, _ in worst)
    rep.check(big < 1e-7, "EX-8 the two solvers give the same numbers in all %d output files (largest difference %.1e; the "
                          "differences are rounding errors amplified by the time stepping and the iterations)" % (len(worst), big))


def main(argv):
    if len(argv) < 2:
        print(__doc__)
        return 2
    prefix = argv[1]
    which = "Python" if os.path.basename(prefix).startswith("t2_py") else "Rust"
    rep = Rep("Revision_SuperLoop, folder test2, program `t2_examples_check.py` (Test 2, items 3 and 5): checks of the "
              "solutions of the seven examples by the %s solver against independent predictions" % which)
    for f in (ex_packet_free, ex_cyclotron, ex_andreev, ex_kink_spectrum, ex_ground_state, ex_kink_ground_state, ex_quench):
        f(prefix, rep)
    if len(argv) > 2:
        compare(prefix, argv[2], rep)
    ok = rep.write(prefix + "_examples_checks.txt")
    print("ALL PASS" if ok else "SOME CHECKS FAILED")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
