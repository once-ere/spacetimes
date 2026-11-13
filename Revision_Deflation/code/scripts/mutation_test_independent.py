#!/usr/bin/env python3
"""
mutation_test_independent.py (Revision_Deflation) -- is the independent verification able to fail?

A check that can only pass proves nothing.  This script takes the part of
code/scripts/verify_deflating_independent.py that verifies the cosmological
perturbations (the checks Idef-C1, Idef-C2, I44d-P1, I71d-P1, Idef-P2 to Idef-P8 of the
supplement RS5), introduces ONE deliberate error at a time into a copy of its text, runs
the copy, and records which check fails.  An error that no check detects would show a
gap in the verification.

The errors are listed below, each with a description.  They are errors of physics (a
wrong factor in an Einstein equation, in an identity, in a growth equation, in the
response of recombination to Newton's constant), not of syntax.  The original file is
not changed, and nothing is written except the report

    data/mutation_test_independent.txt

Run from the folder Revision_Deflation, after the commands `cosmology`, `matter` and
`spectrum` of the program (whose data files the checks read); about four minutes:

    python3 code/scripts/mutation_test_independent.py

Exit status 0 if the unchanged script passes all its checks and every error is detected.
"""
import os
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SRC = os.path.join(ROOT, "code", "scripts", "verify_deflating_independent.py")

# (description, text of the original, text with the error); each original text must occur exactly once
MUTATIONS = [
    ("energy constraint with He = Hc - a gsum instead of Hc - a gsum/2",
     "    he = hc - a * gsum / 2\n", "    he = hc - a * gsum\n"),
    ("trace equation with He h' instead of 2 He h'",
     "- 2 * he * d1(h) - d2(h)) * cosk)", "- he * d1(h) - d2(h)) * cosk)"),
    ("closed form of B with (a gsum)^2/2 instead of (a gsum)^2/4",
     "/ 6 + GA**2 / 4 - A**2 * PH / 3\n", "/ 6 + GA**2 / 2 - A**2 * PH / 3\n"),
    ("pressure of the agent with gprod instead of 2 gprod",
     "- 2 * gsum * Hs + 2 * gprod - gsum * gprod / (3 * Hs)", "- 2 * gsum * Hs + gprod - gsum * gprod / (3 * Hs)"),
    ("first identity between the gaps with (D - DP) instead of (D - 3 DP)",
     "xq = hp * bcl - gg / 2 * (Dsum - 3 * DP)", "xq = hp * bcl - gg / 2 * (Dsum - DP)"),
    ("second identity between the gaps with He instead of 2 He",
     "(dt(x2) + 2 * hef * x2)).subs(shell))", "(dt(x2) + hef * x2)).subs(shell))"),
    ("growth equation of the closure \"Momentum\" with the source not reduced",
     "- (hh - gg) / (2 * hef) * g_m * dlt", "- (hh) / (2 * hef) * g_m * dlt"),
    ("growth equation of the closure \"Pressure\" with the friction Hc instead of He",
     "law_p = sp.Derivative(dlt, (t, 2)) + hef * sp.Derivative(dlt, t)", "law_p = sp.Derivative(dlt, (t, 2)) + hh * sp.Derivative(dlt, t)"),
    ("numerical growth, closure \"Pressure\": friction reduced by gsum instead of gsum/2",
     "return [y[1], -(2 + L - 0.5 * r) * y[1] + q * y[0]]", "return [y[1], -(2 + L - r) * y[1] + q * y[0]]"),
    ("numerical growth, closure \"Momentum\": source not reduced",
     "+ (1 - r) / (1 - 0.5 * r) * q * y[0]]", "+ q * y[0]]"),
    ("system of the closure \"Pressure\" with a gsum instead of a gsum/2",
     "out.append((hpr * b - 0.5 * ga * g * delta) / (2 * kk * kk) / hcv)", "out.append((hpr * b - ga * g * delta) / (2 * kk * kk) / hcv)"),
    ("system of the closure \"Shear\" with Hc instead of He",
     "out.append((-2 * he * alpha + eta_) / hcv)", "out.append((-2 * hcv * alpha + eta_) / hcv)"),
    ("recombination on a background in which Newton's constant does not change",
     "math.exp(s) * (orad / a**4 + (ob + oc) / a**3 + ol))", "1.0 * (orad / a**4 + (ob + oc) / a**3 + ol))"),
    ("response of the helium fraction to Newton's constant doubled",
     "fit(43.0 / 7.0 * (g_early - 1.0))", "fit(2.0 * 43.0 / 7.0 * (g_early - 1.0))"),
    ("Peebles' factor with 4 pi instead of 8 pi (an error of atomic physics)",
     "kk = lam_a**3 / (8.0 * math.pi * hz)", "kk = lam_a**3 / (4.0 * math.pi * hz)"),
    ("equation of the expansion with the term -gsum H doubled (integration at fixed last scattering)",
     "ol = h * h - gam * h + p / 3.0 - orad - ob - oc\n        def e_of(a, s):\n            dens",
     "ol = h * h - 2 * gam * h + p / 3.0 - orad - ob - oc\n        def e_of(a, s):\n            dens"),
]


def run(source):
    """Execute the text of the verification script (without its main program) and run its cosmological parts;
    return the lines of its report.  The output of the checks is suppressed."""
    namespace = {"__file__": SRC, "__name__": "mutant"}
    keep = sys.stdout
    sys.stdout = open(os.devnull, "w")
    try:
        exec(compile(source, SRC, "exec"), namespace)
        for part in ("part_hubble", "part_recombination", "part_perturbations", "part_growth"):
            namespace[part]()
    finally:
        sys.stdout.close()
        sys.stdout = keep
    return list(namespace["LINES"])


def main():
    text = open(SRC, encoding="utf-8").read()
    t0 = time.time()
    base = run(text)
    failing = [l.split()[1] for l in base if not l.startswith("[PASS]")]
    out = ["PROJECT spacetimes, Revision_Deflation: mutation test of the independent verification of the cosmological perturbations",
           "generated by: code/scripts/mutation_test_independent.py",
           "",
           "the unchanged script: %d checks (%s), %s" % (len(base), ", ".join(l.split()[1] for l in base), "all pass" if not failing else "FAILING: " + ", ".join(failing)),
           ""]
    print(out[3], flush=True)
    detected = 0
    for k, (name, old, new) in enumerate(MUTATIONS, 1):
        n = text.count(old)
        if n != 1:
            line = "%2d. %s: THE ERROR COULD NOT BE INTRODUCED (the original text occurs %d times)" % (k, name, n)
        else:
            try:
                lines = run(text.replace(old, new))
                bad = [l.split()[1] for l in lines if not l.startswith("[PASS]")]
            except Exception as err:  # an error that stops the script is detected as well
                bad = ["the script stops (%s)" % type(err).__name__]
            detected += bool(bad)
            line = "%2d. %s: %s" % (k, name, ("detected by " + ", ".join(bad)) if bad else "NOT DETECTED")
        out.append(line)
        print(line, flush=True)
    out += ["", "%d of %d deliberate errors detected" % (detected, len(MUTATIONS))]
    print(out[-1] + "   (%.0f s)" % (time.time() - t0))
    with open(os.path.join(ROOT, "data", "mutation_test_independent.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    return 0 if not failing and detected == len(MUTATIONS) else 1


if __name__ == "__main__":
    sys.exit(main())
