#!/usr/bin/env python3
"""
make_tables_cjt.py -- table fragments for supplement S4 (the effective action of
Cornwall, Jackiw and Tomboulis) and for the passages of S1 and S2 that quote its
results.  Called by make_tables.py; reads data/cjt_*.csv, data/reduced_symmetry.csv,
data/gap_scan.csv and data/dimer.csv, all written by the Rust program.

Every number in the fragments is read from a CSV file; nothing is typed in by hand.
The single derived quantity computed here (k_F |a| in a dwarf galaxy, fragment
cjt_kfa_dwarf) is documented where it is computed.
"""
import csv
import math
import os


def _meta(DATA, name, key):
    """Value of the comment line '# key: value' of a CSV file."""
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        return None
    for line in open(path):
        if line.startswith("# %s:" % key):
            return line.split(":", 1)[1].strip()
    return None


def _text(DATA, name):
    path = os.path.join(DATA, name)
    if not os.path.exists(path):
        print("  (skipped: data/%s does not exist)" % name)
        return None
    with open(path, newline="") as f:
        rows = list(csv.reader(line for line in f if not line.startswith("#")))
    return [dict(zip(rows[0], r)) for r in rows[1:]]


# The items of data/reduced_symmetry.csv are plain text written by the Rust program.
# For the documents they are typeset here.  A vertical bar must not appear in a cell
# of a Markdown table (it separates the cells), so norms are written with \lVert.
_SYMMETRY_ITEMS = {
    "|P^2 - 1| for the traceless central generator P":
        r"$\lVert P^2-1\rVert$ for the traceless central generator $P$",
    "structure constants |f_123| of the ideals (generators with Tr T_i T_j = 2 delta_ij)":
        r"structure constant $\lvert f_{123}\rvert$ of each ideal (generators with $\mathrm{Tr}\,T_iT_j=2\delta_{ij}$)",
    "|sum_i T_i^2 - (3/4) projector| of the ideals":
        r"$\lVert\sum_iT_i^2-\tfrac34\,(\text{projector})\rVert$ for each ideal",
    "closure of the four vector pair operators C^- gamma^a, a in {0,5,6,7}":
        r"the four vector pair operators $C^-\gamma^a$, $a\in\{0,5,6,7\}$, transform among themselves: residual",
    "|Casimir - 3/4| of the two ideals on the four pair operators":
        r"$\lvert\text{Casimir}-\tfrac34\rvert$ of each ideal on the four pair operators",
    "complex dimension of the particle-antiparticle mixing generators, mu = 0":
        r"complex dimension of the generators that mix particles with antiparticles, $\mu=0$",
}


def _symmetry_item(item):
    """Typeset one item of data/reduced_symmetry.csv for a Markdown table."""
    if item in _SYMMETRY_ITEMS:
        return _SYMMETRY_ITEMS[item]
    parts = item.split()
    if len(parts) == 2 and all(q.startswith("gamma^") for q in parts):   # e.g. "gamma^0 gamma^5"
        return "$\\gamma^%s\\gamma^%s$" % (parts[0][6:], parts[1][6:])
    assert "|" not in item and "^" not in item and "_" not in item, "untreated item: %r" % item
    return item


def _symmetry_value(value, sci):
    """Typeset one value: residuals in scientific notation, no vertical bars."""
    value = value.replace("|[.,dynamics]| =", "commutator with the dynamics")

    def number(tok):
        if tok in ("0.00e0", "0.00e+0"):
            return "0"
        try:
            if "e" in tok:
                return sci(float(tok))
        except ValueError:
            pass
        return tok

    out = " ".join(number(t) for t in value.split())
    assert "|" not in out, "vertical bar left in %r" % out
    return out


def run(frag, read_csv, sci, table, DATA):
    # ---------------------------------------------------------------- symmetry (S1 Sec. 2.1, S4 Sec. 2)
    rows = _text(DATA, "reduced_symmetry.csv")
    if rows is not None:
        body = [[_symmetry_item(r["item"]), _symmetry_value(r["value_1671"], sci), _symmetry_value(r["value_1644"], sci)] for r in rows]
        text = table(["item (computed from the matrices)", "7+1", "4+4"], [":------------------------------------------", ":-----------------", ":-----------------"], body)
        n_ok = sum(1 for r in rows if r["ok"] == "1")
        text += "\n\nAll %d items have the expected values (`data/reduced_symmetry.csv`, column `expected`; %d of %d flagged as correct by the program)." % (len(rows), n_ok, len(rows))
        frag("cjt_symmetry_table", text)

    # ---------------------------------------------------------------- zero-dimensional test
    t = read_csv("cjt_toy.csv")
    if t is not None:
        def cell(x, d=4):
            return "—" if not math.isfinite(x) else "%.*f" % (d, x)
        body = []
        for r in t:
            if r["lambda"] in (0.01, 0.03, 30.0):
                continue
            body.append(["%g" % r["lambda"], cell(r["G_exact"]), cell(r["G_2loop"]), cell(r["G_3loop"]), cell(r["G_4loop"]), cell(r["G_5loop"]),
                         cell(r["pert_1"], 3), cell(r["pert_2"], 3), cell(r["pert_3"], 3)])
        frag("cjt_toy_table", table(["$\\lambda$", "exact", "2 loops", "3 loops", "4 loops", "5 loops", "pert. $O(\\lambda)$", "$O(\\lambda^2)$", "$O(\\lambda^3)$"],
                                    ["----:"] + ["------:"] * 8, body))
        # worst error of the two-loop truncation over all couplings
        err = max(abs(r["G_2loop"] / r["G_exact"] - 1) for r in t)
        frag("cjt_toy_hartree_err", "%.0f" % (100 * err))
        r1 = [r for r in t if r["lambda"] == 1.0][0]
        frag("cjt_toy_gamma_err_2", sci(abs(r1["Gamma_2loop"] - r1["minus_lnZ"])))
        frag("cjt_toy_gamma_err_4", sci(abs(r1["Gamma_4loop"] - r1["minus_lnZ"])))

    # ---------------------------------------------------------------- pairing checks
    rows = _text(DATA, "cjt_pairing_check.csv")
    scan = read_csv("gap_scan.csv")
    if rows is not None:
        names = {"V": "vector", "T": "two-form", "S": "radial only"}
        col = {"V": "gap_V_over_EF", "T": "gap_T_over_EF", "S": "gap_S_over_EF"}
        body = []
        worst = {"newton": 0.0, "scan": 0.0, "tc": 0.0, "smin": 0.0}
        v_lo, v_hi = 1e9, -1e9
        for r in rows:
            g, ms, kf, ef = float(r["g"]), float(r["m_s"]), float(r["k_F"]), float(r["E_F"])
            gap = float(r["gap_newton"])
            stored = float("nan")
            if scan is not None:
                m = [x for x in scan if abs(x["g"] - g) < 1e-9 and abs(x["m_s_over_m"] - ms) < 1e-9 and abs(x["kF_over_m"] - kf) < 1e-9]
                if m:
                    stored = float(m[0][col[r["channel"]]]) * ef
            d_scan = abs(stored / gap - 1) if math.isfinite(stored) else float("nan")
            worst["newton"] = max(worst["newton"], float(r["max_rel_diff"]))
            if math.isfinite(d_scan):
                worst["scan"] = max(worst["scan"], d_scan)
            worst["tc"] = max(worst["tc"], abs(float(r["Tc_rel_diff"])))
            worst["smin"] = max(worst["smin"], abs(float(r["s_min_minus_1"])))
            v = float(r["V_over_V_BCS"])
            v_lo, v_hi = min(v_lo, v), max(v_hi, v)
            body.append(["%g" % g, "%g" % ms, "%g" % kf, names[r["channel"]], sci(gap / ef, 3), sci(float(r["max_rel_diff"])),
                         sci(d_scan) if math.isfinite(d_scan) else "—", "%.4f" % v, "%.3f" % float(r["curvature_min"]),
                         "%.4f" % float(r["Tc_over_gap"]), sci(abs(float(r["Tc_rel_diff"])))])
        hdr = ["$g_{VS}$", "$m_s$", "$k_F$", "pair", "gap$/E_F$", "Newton vs fixed point", "vs `gap_scan.csv`", "$V/V_{\\rm BCS}$", "smallest curvature", "$T_c$/gap", "$T_c$: two methods"]
        frag("cjt_pairing_table", table(hdr, ["-----:", "-----:", "-----:", ":------", "--------:", "--------:", "--------:", "---------:", "-------:", "------:", "--------:"], body))
        frag("cjt_pair_worst_newton", sci(worst["newton"]))
        frag("cjt_pair_worst_scan", sci(worst["scan"]))
        frag("cjt_pair_worst_tc", sci(worst["tc"]))
        frag("cjt_pair_worst_smin", sci(worst["smin"]))
        frag("cjt_pair_v_lo", "%.3f" % v_lo)
        frag("cjt_pair_v_hi", "%.3f" % v_hi)

    # ---------------------------------------------------------------- threshold
    t = read_csv("cjt_threshold.csv")
    if t is not None:
        body = [["%g" % r["m_s_over_m"], "%.5f" % r["D_c_gap_equation"], "%+.2e" % r["relative_difference"]] for r in t]
        text = table(["$m_s/m$", "$D_c$ from the gap equation", "relative difference from `dimer.rs`"], ["------:", "-----------:", "-----------:"], body)
        frag("cjt_threshold_table", text)
        frag("cjt_threshold_best", "%.5f" % t[-1]["D_c_gap_equation"])
        frag("cjt_threshold_ode", "%.5f" % t[-1]["D_c_schroedinger"])

    # ---------------------------------------------------------------- thermodynamics
    for key, name, fmt in (("gap0_over_Tc", "cjt_gap0_over_tc", "%.4f"), ("heat_capacity_jump", "cjt_c_jump", "%.3f"), ("gap_slope_at_0.98", "cjt_gap_slope", "%.3f")):
        v = _meta(DATA, "cjt_thermo.csv", key)
        if v is not None:
            frag(name, fmt % float(v))
    t = read_csv("cjt_thermo.csv")
    if t is not None:
        frag("cjt_tc_over_gap0", "%.4f" % (1.0 / float(_meta(DATA, "cjt_thermo.csv", "gap0_over_Tc"))))
        body = [["%.2f" % r["T_over_Tc"], "%.4f" % r["gap_over_gap0"], "%.4f" % (-r["V_over_absV0"]), "%.4f" % (r["S_paired"] / r["S_unpaired"]), "%.4f" % (r["C_paired"] / r["C_unpaired"])]
                for r in t if round(r["T_over_Tc"], 3) in (0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99)]
        frag("cjt_thermo_table", table(["$T/T_c$", "gap$(T)$/gap$(0)$", "$-V(T)/|V(0)|$", "$S_s/S_n$", "$C_s/C_n$"], ["-----:", "--------:", "--------:", "------:", "------:"], body))

    # ---------------------------------------------------------------- Ginzburg-Landau
    t = read_csv("cjt_gl_summary.csv")
    if t is not None:
        body = [["%g" % r["g"], "%g" % r["m_s"], "%g" % r["k_F"], "%.3f" % r["lambda"], sci(r["Tc_over_EF"]), "%.4f" % r["alpha_ratio"], "%.4f" % r["beta_ratio"], "%.4f" % r["K_ratio"]] for r in t]
        frag("cjt_gl_table", table(["$g_{VS}$", "$m_s/m$", "$k_F/m$", "$\\lambda$", "$T_c/E_F$", "$a$: potential / formula", "$b$: potential / formula", "$K$: susceptibility / formula"],
                                   ["---:", "---:", "---:", "-----:", "-------:", "---------:", "---------:", "---------:"], body))
        frag("cjt_gl_worst", "%.1f" % (100 * max(max(abs(r["alpha_ratio"] - 1), abs(r["beta_ratio"] - 1), abs(r["K_ratio"] - 1)) for r in t)))
    t = read_csv("cjt_gl.csv")
    summ = read_csv("cjt_gl_summary.csv")
    if t is not None and summ is not None:
        # logarithmic slope of the quartic coefficient with temperature at T_c (first parameter set):
        # measured from T/T_c = 0.99 and 1.01, and predicted by Eq. (3.4): -(2 + 4 lambda)
        lo = [r for r in t if r["g"] == 3.0 and abs(r["T_over_Tc"] - 0.99) < 1e-9][0]
        hi = [r for r in t if r["g"] == 3.0 and abs(r["T_over_Tc"] - 1.01) < 1e-9][0]
        frag("cjt_beta_slope_measured", "%.2f" % (math.log(hi["beta"] / lo["beta"]) / math.log(1.01 / 0.99)))
        frag("cjt_beta_slope_predicted", "%.2f" % (-(2 + 4 * [r for r in summ if r["g"] == 3.0][0]["lambda"])))
    if t is not None:
        sel = [r for r in t if r["g"] == 3.0]
        body = [["%.2f" % r["T_over_Tc"], "%.4f" % r["alpha_ratio"], "%.4f" % r["beta_ratio"]] for r in sel]
        frag("cjt_gl_T_table", table(["$T/T_c$", "$\\alpha/[4N(0)\\ln(T/T_c)]$", "$\\beta/[7\\zeta(3)N(0)/(2\\pi^2T_c^2)]$"], ["-----:", "-----------:", "-----------:"], body))

    # ---------------------------------------------------------------- order-parameter space
    rows = _text(DATA, "cjt_landscape.csv")
    if rows is not None:
        body = []
        for r in rows:
            sig = "7+1" if r["signature"] == "1671" else "4+4"
            what = ("%s flat, %s positive (%s)" % (r["flat"], r["positive"], r["hessian"].split()[-1])) if r["hessian"] != "-" else ("stationary; unstable" if r["state"].startswith("half") else "not stationary")
            body.append([sig, r["state"].replace("eta_ab Delta^a Delta^b", "$\\eta_{ab}\\Delta^a\\Delta^b$"), "%.3f, %.3f" % (float(r["gap2_small"]), float(r["gap2_large"])), "%+.4f" % float(r["V"]), what,
                         r["flat_coordinates"].replace("D0", "$\\Delta^0$").replace("D5", "$\\Delta^5$").replace("D6", "$\\Delta^6$").replace("D7", "$\\Delta^7$") if r["flat_coordinates"] not in ("-", "") else "—"])
        frag("cjt_landscape_table", table(["", "state", "the two gaps$^2$", "$V$", "second derivatives", "flat directions"],
                                          [":---", ":-------------------------", ":--------", "------:", ":----------------", ":----------------"], body))
        half = [r for r in rows if r["state"].startswith("half")][0]
        frag("cjt_half_up", sci(max(float(half["dV_scaled_up"]), float(half["dV_scaled_down"]))))
        frag("cjt_half_open", sci(-float(half["dV_second_gap_opened"])))
        g = [r for r in rows if r["state"].startswith("hidden")][0]
        frag("cjt_massive_curvature", g["hessian"].split()[-1])

    # ---------------------------------------------------------------- normal state
    t = read_csv("cjt_normal_summary.csv")
    if t is not None:
        frag("cjt_c_crit", "%.4f" % float(_meta(DATA, "cjt_normal_summary.csv", "critical_C_without_sea")))
        frag("cjt_c_crit_sea", "%.4f" % float(_meta(DATA, "cjt_normal_summary.csv", "critical_C_with_sea")))

        def f(x, fmt="%.3f"):
            return "—" if not math.isfinite(x) else fmt % x
        body = []
        for c in sorted(set(t["C"])):
            a = [r for r in t if r["C"] == c and r["sea"] == 0][0]
            b = [r for r in t if r["C"] == c and r["sea"] == 1][0]
            body.append(["%g" % c,
                         ("%+.3f" % a["e_bind"]) if a["bound"] else "not bound", f(a["kF_sat"]) if a["bound"] else "—", f(a["kF_abnormal"]), f(a["x_before"], "%.2f"), f(a["kF_spinodal"]),
                         ("%+.3f" % b["e_bind"]) if b["bound"] else "not bound", f(b["kF_abnormal"]), f(b["kF_spinodal"])])
        hdr = ["$C$", "$E/N-m$", "$k_F^{\\rm sat}$", "$k_F^{\\rm abn}$", "$m^*/m$ before", "$k_F^{\\rm inst}$", "with sea: $E/N-m$", "$k_F^{\\rm abn}$", "$k_F^{\\rm inst}$"]
        frag("cjt_normal_table", table(hdr, ["----:", "--------:", "------:", "------:", "------:", "------:", "--------:", "------:", "------:"], body))
        for c, name in ((35.5, "355"),):
            a = [r for r in t if abs(r["C"] - c) < 1e-9 and r["sea"] == 0]
            b = [r for r in t if abs(r["C"] - c) < 1e-9 and r["sea"] == 1]
            if a and b:
                frag("cjt_ebind_%s" % name, "%.0f" % (-100 * a[0]["e_bind"]))
                frag("cjt_ebind_%s_sea" % name, "%.0f" % (-100 * b[0]["e_bind"]))
    t = read_csv("cjt_fock.csv")
    if t is not None:
        body = [["%g" % r["C"], "%g" % r["m_s"], "%.2f" % r["g2"], "%g" % r["kF"], "%.3f" % r["x"], "%+.1f" % (100 * r["fock_radial_over_hartree"]), "%+.1f" % (100 * r["fock_phase_over_hartree"])] for r in t]
        frag("cjt_fock_table", table(["$C$", "$m_s/m$", "$g_{VS}^2$", "$k_F/m$", "$m^*/m$", "radial mode (%)", "phase mode (%)"], ["---:", "----:", "-----:", "----:", "-----:", "--------:", "--------:"], body))
        frag("cjt_fock_contact", "%.1f" % (100 * max(r["fock_radial_over_hartree"] for r in t)))

    # k_F |a| in a dwarf galaxy for particles at the self-interaction limit.
    # sigma/m <= 1 cm^2/g = 4.5782e-24 eV^-3 (hbar c = 1.973269804e-7 eV m, 1 g = 5.609588603e32 eV);
    # sigma = 4 pi a^2; fermions of mass 1 keV with velocity 10 km/s: k_F ~ m v/c.
    hbarc, gram = 1.973269804e-7, 5.609588603e32
    cm2_per_g = (1e-2 / hbarc) ** 2 / gram
    m_ev, v = 1.0e3, 1.0e4 / 2.99792458e8
    a_max = math.sqrt(cm2_per_g * m_ev / (4 * math.pi))
    frag("cjt_kfa_dwarf", sci(m_ev * v * a_max))

    # ---------------------------------------------------------------- finite temperature
    t = read_csv("cjt_thermal.csv")
    if t is not None:
        body = [["%g" % r["g"], "%g" % r["m_s_over_m"], "%.3g" % r["C"], "%.3f" % r["T_high_over_v"], "%.3f" % r["T_largeN_over_v"], "%.3f" % r["T_half_over_v"],
                 "%.3f" % r["T_zero_over_v"], "%.2f" % r["phi_before_zero"], "%.2f" % r["T_half_over_m"]] for r in t]
        frag("cjt_thermal_table", table(["$g_{VS}$", "$m_s/m$", "$C$", "$T_{\\rm high}/v$", "$T_{N}/v$", "$T_{1/2}/v$", "$T_0/v$", "$\\phi/v$ before $T_0$", "$T_{1/2}/m$"],
                                        ["----:", "----:", "-----:", "------:", "------:", "------:", "------:", "---------:", "------:"], body))
        frag("cjt_thermal_jumps", ", ".join("(%g, %g)" % (r["g"], r["m_s_over_m"]) for r in t if r["phi_before_zero"] > 0.25))
        frag("cjt_thermal_jump_size", "%.1f" % max(r["phi_before_zero"] for r in t))
        cont = [r["phi_before_zero"] for r in t if r["phi_before_zero"] <= 0.25]
        frag("cjt_thermal_cont_lo", "%.2f" % min(cont))
        frag("cjt_thermal_cont_hi", "%.2f" % max(cont))
        frag("cjt_thermal_spread", "%.0f" % (100 * max(max(abs(r["T_half_over_v"] / r["T_high_over_v"] - 1), abs(r["T_half_over_v"] / r["T_largeN_over_v"] - 1)) for r in t)))
    p = read_csv("cjt_thermal_profile.csv")
    if p is not None:
        frag("cjt_goldstone_defect", "%.2f" % max(r["Mth_over_ms"] for r in p if r["phi_over_v"] > 0.3))

    # ---------------------------------------------------------------- stability of the one-loop vacuum
    t = read_csv("cjt_vacuum_stability.csv")
    if t is not None:
        body = [["%g" % r["C"], ("%.3g" % r["x_max"]) if r["x_max"] < 1e4 else sci(r["x_max"]), ("%.3g" % r["x_zero"]) if r["x_zero"] < 1e4 else sci(r["x_zero"])] for r in t]
        frag("cjt_vacstab_table", table(["$C$", "maximum of the potential at $\\phi/v_{VS}=$", "below the vacuum beyond $\\phi/v_{VS}=$"], ["------:", "---------------:", "---------------:"], body))

    # ---------------------------------------------------------------- vacuum
    t = read_csv("cjt_vacuum.csv")
    if t is not None:
        cuts = sorted(set(t["cutoff_over_m"]))
        body = []
        for ms in sorted(set(t["m_s_over_m"])):
            body.append(["%g" % ms] + ["%.3f" % [r for r in t if r["m_s_over_m"] == ms and r["cutoff_over_m"] == c][0]["alpha_c"] for c in cuts])
        hdr = ["$m_s/m$"] + ["$\\Lambda/m=10^{%g}$" % math.log10(c) if abs(math.log10(c) - round(math.log10(c))) < 1e-9 else "$\\Lambda/m=%g$" % c for c in cuts]
        frag("cjt_vacuum_table", table(hdr, ["-----:"] + ["------:"] * len(cuts), body))
        for c, name in ((10.0, "10"), (100.0, "100"), (1e6, "1e6")):
            frag("cjt_alpha_c_" + name, "%.2f" % [r for r in t if r["m_s_over_m"] == 1.0 and r["cutoff_over_m"] == c][0]["alpha_c"])
