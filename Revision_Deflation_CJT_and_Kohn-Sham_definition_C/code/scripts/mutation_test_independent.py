#!/usr/bin/env python3
"""
mutation_test_independent.py (Revision_Deflation_CJT_and_Kohn-Sham_definition_C) -- are the independent
verifications able to fail?

A check that can only pass proves nothing.  This script takes the two scripts of the
independent verification,

    code/scripts/verify_cjt_ks_independent.py        (self-interaction, Kohn-Sham, CJT, pairing, gravity,
                                                      the two forms of the self-interaction on 4+4)
    code/scripts/verify_deflating_independent.py     (its part on the cosmological perturbations),

introduces ONE deliberate error at a time into a copy of the text, runs the copy, and
records which check fails.  An error that no check detects would show a gap in the
verification.

The errors are listed below, each with a description.  They are errors of physics or of
mathematics (a wrong phase of a matrix, Wick's theorem without the exchange term, a wrong
sign in a gap equation, a wrong coefficient of a Lovelock tensor, a wrong factor in an
Einstein equation, ...), not of syntax.  The original files are not changed, and nothing
is written except the report

    data/mutation_test_independent.txt

Run from the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C, after all commands of the
program (whose data files the checks read); about eight minutes:

    python3 code/scripts/mutation_test_independent.py

Exit status 0 if the unchanged scripts pass all their checks and every error is detected.
"""
import os
import re
import sys
import time

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SRC = os.path.join(ROOT, "code", "scripts", "verify_deflating_independent.py")
SRC_NEW = os.path.join(ROOT, "code", "scripts", "verify_cjt_ks_independent.py")

# ---------------------------------------------------------------------------------------
# errors introduced into verify_cjt_ks_independent.py:
# (description, text of the original, text with the error, letters of the parts to run;
#  F1 .. F8 name a single check of part F, the two forms of the self-interaction on 4+4)
# ---------------------------------------------------------------------------------------
MUTATIONS_NEW = [
    ("Fierz matrix with the sign (-1)^(rs) instead of (-1)^(rs-l)",
     "sum((-1) ** (r * s - l) * math.comb(s, l)", "sum((-1) ** (r * s) * math.comb(s, l)", "S"),
    ("covariant form of 4+4 with the phases of the 7+1 model (i for the ranks 2, 3, 6, 7)",
     "        with_i = {1, 2, 5, 6}\n", "        with_i = {2, 3, 6, 7}\n", "S"),
    ("Wick's theorem without the exchange term",
     "en += 0.5 * eps * (t1 * t1 - np.trace(mk @ n @ mk @ n)).real", "en += 0.5 * eps * (t1 * t1).real", "S"),
    ("interaction of two particles without the transposition of the second vertex",
     "img += g[r] * eps * (mk @ phi @ mk.T)", "img += g[r] * eps * (mk @ phi @ mk)", "S"),
    ("chiral condition of 7+1 with +g_8 instead of -g_8",
     "g_inv[0] = 4 * g[2] + 10 * g[4] + 4 * g[6] - g[8]", "g_inv[0] = 4 * g[2] + 10 * g[4] + 4 * g[6] + g[8]", "S"),
    ("gap equation M = m - G_S rho_S instead of M = m + G_S rho_S",
     "        return mass - m - gs * fermi_integrals(d, kf, mass, nu)[1]\n", "        return mass - m + gs * fermi_integrals(d, kf, mass, nu)[1]\n", "K"),
    ("Kohn-Sham energy density with the scalar term +(M - m)^2/(2 G_S) instead of -(M - m)^2/(2 G_S)",
     '"eps": e_kin + pi - (mass - m) ** 2 / gs, "p": p_kin + pi, "Pi": pi}', '"eps": e_kin + pi, "p": p_kin + pi, "Pi": pi}', "K"),
    ("closed form of the integral over the flat momenta with c^3/3 instead of 2 c^3/3",
     "j_s = wu**3 / 3 - c2 * wu + 2 * cc**3 / 3", "j_s = wu**3 / 3 - c2 * wu + cc**3 / 3", "K"),
    ("density of the deflating matter without the factor s5 s6 s7",
     "front = NU / (2 * math.pi) ** 7 * s[0] * s[1] * s[2] * area(4)", "front = NU / (2 * math.pi) ** 7 * area(4)", "K"),
    ("second gap equation of the two halves with -G_0 rho_SW replaced by +G_0 rho_SW",
     "masses[1] - 1 - gs0 * (rs[0] + rs[1]) + g0c * (rs[0] - rs[1])]", "masses[1] - 1 - gs0 * (rs[0] + rs[1]) - g0c * (rs[0] - rs[1])]", "K"),
    ("CJT potential with +V_0^2/(2 G_V) instead of -V_0^2/(2 G_V)",
     "- (v0**2 / (2 * gv) if gv != 0 else 0.0)", "+ (v0**2 / (2 * gv) if gv != 0 else 0.0)", "J"),
    ("critical coupling from the integral of k^(D-1) instead of k^(D-2)",
     "sp.integrate(k ** (d - 2), (k, 0, lam))", "sp.integrate(k ** (d - 1), (k, 0, lam))", "J"),
    ("potential of the Vorton-Scalar with 1/(4 C) instead of 1/(8 C)",
     "pot = (1 - f_d) ** 2 * (x * x - 1) ** 2 / (8 * c)", "pot = (1 - f_d) ** 2 * (x * x - 1) ** 2 / (4 * c)", "U"),
    ("pair along x1 instead of the hidden axis x0 (7+1)",
     "xpair = cminus @ G71[0]", "xpair = cminus @ G71[1]", "P"),
    ("vector amplitude F_1 with +2 M^2 instead of -2 M^2 in the gap equation",
     "1: (3 * e * ep - 2 * mass**2, -k * kp)", "1: (3 * e * ep + 2 * mass**2, -k * kp)", "P"),
    ("Lovelock tensors with the factor 2^k instead of 2^(k-1)",
     "return (-1) ** (k + 1) * math.factorial(k) * 2 ** (k - 1) * pair_sum(k, dirs, sym)", "return (-1) ** (k + 1) * math.factorial(k) * 2 ** k * pair_sum(k, dirs, sym)", "G"),
    ("Lanczos tensor with the last term doubled",
     "- 2 * term3 + term4) - l_gb / 2))", "- 2 * term3 + 2 * term4) - l_gb / 2))", "G"),
    ("Lovelock density of the reduced action with k! 2^(k-1) instead of k! 2^k",
     "dens = math.factorial(k) * 2**k * pair_sum(", "dens = math.factorial(k) * 2**(k - 1) * pair_sum(", "G"),
    ("pressure of the matter along the first group of coordinates without the interaction part Pi",
     "p_a = np.sum(self.w * ka2 / om) / vol / self.n_a + pi", "p_a = np.sum(self.w * ka2 / om) / vol / self.n_a", "G"),
    ("field equation of the first group of coordinates with +kappa p_a instead of -kappa p_a",
     "[-kappa * p_a - ca[2], -kappa * p_b - cb[2]]", "[kappa * p_a - ca[2], -kappa * p_b - cb[2]]", "G"),
    ('mean-field couplings of the covariant form of 4+4 assigned as in the quantized-sector form (totals and differences not exchanged)',
     'return (gv_c, gs_c, gv_hf, gs_hf) if form == "C" else (gv_hf, gs_hf, gv_c, gs_c)', 'return (gv_hf, gs_hf, gv_c, gs_c)', 'F4'),
    ("Wick's theorem for matter of two halves without the exchange term",
     'val = eps * (t[i] * t[j] - np.trace(md[i] @ md[j])) / 256.0', 'val = eps * (t[i] * t[j]) / 256.0', 'F1'),
    ('internal symmetry: the transformation generated by gamma^0 gamma^5 replaced by a rotation of x5 and x6',
     '"hidden": herm(G44[0] @ G44[5])', '"hidden": herm(G44[5] @ G44[6])', 'F2'),
    ('pairing amplitudes of the covariant form in the hidden-axis pair with the sign of the quantized-sector form',
     '(("44c", 0, -1.0), ("44c", 5, 1.0)', '(("44c", 0, 1.0), ("44c", 5, 1.0)', 'F3'),
    ('effective potential of two halves without the term of the difference of the two masses',
     'for x, gcoup in ((mu_s, ts), (d_s, ds), (v_t, tv), (v_d, dv)):', 'for x, gcoup in ((mu_s, ts), (v_t, tv), (v_d, dv)):', 'F5'),
    ('scalar density of the filled states of negative energy with the sign of that of particles',
     'return -pref(d, nu) * quad(lambda k: k ** (d - 1) * mass / math.sqrt(k * k + mass * mass), 0, lam', 'return pref(d, nu) * quad(lambda k: k ** (d - 1) * mass / math.sqrt(k * k + mass * mass), 0, lam', 'F6'),
    ('gap of the hidden-axis pair of the covariant form with the contact couplings of the sign +1',
     'contact = [gg * gg, 0.0, 0.0, 0.0] if iv == 1 else [0.0, -0.3 * gg * gg, 0.0, 0.0]     # the couplings of the variant times ct_sign = -1', 'contact = [-gg * gg, 0.0, 0.0, 0.0] if iv == 1 else [0.0, 0.3 * gg * gg, 0.0, 0.0]     # the couplings of the variant times ct_sign = -1', 'F7'),
    ('ultraviolet integral of the vector amplitude with 3 pi^2/4 + 1 instead of 3 pi^2/4 - 1',
     'i_s, i_v = math.pi**2 / 8 - 0.5, 3 * math.pi**2 / 4 - 1\n    dev_int', 'i_s, i_v = math.pi**2 / 8 - 0.5, 3 * math.pi**2 / 4 + 1\n    dev_int', 'F8'),
    ('ultraviolet strength with the contact couplings of the opposite sign',
     'return g2 * modes * i_s - sign * lam * lam * (', 'return g2 * modes * i_s + sign * lam * lam * (', 'F8'),
]

# ---------------------------------------------------------------------------------------
# errors introduced into the cosmological part of verify_deflating_independent.py:
# (description, text of the original, text with the error); each original text must occur exactly once
# ---------------------------------------------------------------------------------------
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


def run_new(source, parts):
    """Execute the text of verify_cjt_ks_independent.py (without its main program) and run the parts whose
    letters are given; return the lines of its report."""
    namespace = {"__file__": SRC_NEW, "__name__": "mutant"}
    keep = sys.stdout
    sys.stdout = open(os.devnull, "w")
    try:
        exec(compile(source, SRC_NEW, "exec"), namespace)
        if "S" in parts:
            models, red = namespace["part_selfinteraction"]()
        else:
            models = {t: namespace["build_model"](t) for t in ("71s", "44q", "44c")}
            table = namespace["reduction_table"](namespace["fierz_closed"]())
            red = namespace["np"].array([[float(x) for x in table[r]] for r in range(9)])
        if "K" in parts:
            namespace["part_kohn_sham"](models, red)
        if "J" in parts:
            namespace["part_cjt"]()
        if "U" in parts:
            namespace["part_unified"]()
        if "P" in parts:
            namespace["part_pairing"](models, red)
        if "G" in parts:
            namespace["part_gravity"]()
        single = {int(x) for x in re.findall(r"F(\d)", parts)}
        if single:
            namespace["part_forms"](models, single)
        elif "F" in parts:
            namespace["part_forms"](models)
    finally:
        sys.stdout.close()
        sys.stdout = keep
    return list(namespace["LINES"])


def main():
    t0 = time.time()
    out = ["PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C: mutation test of the independent verifications",
           "generated by: code/scripts/mutation_test_independent.py", ""]
    all_ok = True
    total = detected = 0
    # ---- the new verification ----
    text = open(SRC_NEW, encoding="utf-8").read()
    base = run_new(text, "SKJUPGF")
    failing = [l.split()[1] for l in base if not l.startswith("[PASS]")]
    all_ok &= not failing
    out += ["A. code/scripts/verify_cjt_ks_independent.py", "",
            "the unchanged script: %d checks, %s" % (len(base), "all pass" if not failing else "FAILING: " + ", ".join(failing)), ""]
    print(out[-2], flush=True)
    for k, (name, old, new, parts) in enumerate(MUTATIONS_NEW, 1):
        total += 1
        n = text.count(old)
        if n != 1:
            line = "A%02d. %s: THE ERROR COULD NOT BE INTRODUCED (the original text occurs %d times)" % (k, name, n)
        else:
            try:
                lines = run_new(text.replace(old, new), parts)
                bad = [l.split()[1] for l in lines if not l.startswith("[PASS]")]
            except Exception as err:  # an error that stops the script is detected as well
                bad = ["the script stops (%s)" % type(err).__name__]
            detected += bool(bad)
            line = "A%02d. %s: %s" % (k, name, ("detected by " + ", ".join(bad)) if bad else "NOT DETECTED")
        out.append(line)
        print(line, flush=True)
    # ---- the cosmological part of the verification of the deflating spacetimes ----
    text = open(SRC, encoding="utf-8").read()
    base = run(text)
    failing = [l.split()[1] for l in base if not l.startswith("[PASS]")]
    all_ok &= not failing
    out += ["", "B. code/scripts/verify_deflating_independent.py, cosmological perturbations", "",
            "the unchanged script: %d checks (%s), %s" % (len(base), ", ".join(l.split()[1] for l in base), "all pass" if not failing else "FAILING: " + ", ".join(failing)),
            ""]
    print(out[-2], flush=True)
    for k, (name, old, new) in enumerate(MUTATIONS, 1):
        total += 1
        n = text.count(old)
        if n != 1:
            line = "B%02d. %s: THE ERROR COULD NOT BE INTRODUCED (the original text occurs %d times)" % (k, name, n)
        else:
            try:
                lines = run(text.replace(old, new))
                bad = [l.split()[1] for l in lines if not l.startswith("[PASS]")]
            except Exception as err:  # an error that stops the script is detected as well
                bad = ["the script stops (%s)" % type(err).__name__]
            detected += bool(bad)
            line = "B%02d. %s: %s" % (k, name, ("detected by " + ", ".join(bad)) if bad else "NOT DETECTED")
        out.append(line)
        print(line, flush=True)
    out += ["", "%d of %d deliberate errors detected" % (detected, total)]
    print(out[-1] + "   (%.0f s)" % (time.time() - t0))
    with open(os.path.join(ROOT, "data", "mutation_test_independent.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(out) + "\n")
    return 0 if all_ok and detected == total else 1


if __name__ == "__main__":
    sys.exit(main())
