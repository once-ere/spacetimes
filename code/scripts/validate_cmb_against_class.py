#!/usr/bin/env python3
"""
Validation of the Einstein-Boltzmann solver of PROJECT spacetimes against CLASS.

WHAT IS COMPARED
  * data/cmb_fiducial_cl.csv   spectra of the fiducial LambdaCDM model computed by the
                               Rust program (`spacetimes spectrum`), WITHOUT lensing;
  * data/reference/class_lcdm_lensed_cl.csv
                               spectra of the same model computed with CLASS v3.4.2 WITH
                               lensing (from the author's NCSS project), together with the
                               spectrum of the lensing potential computed by CLASS;
  * data/cmb_fiducial_pk.csv and data/reference/class_lcdm_pk.csv
                               linear matter power spectra today.

HOW
  The Rust program does not compute gravitational lensing.  To compare like with like,
  this script lenses the Rust spectra with the lensing-potential spectrum of the
  reference, using the flat-sky correlation-function method (A. Lewis and A. Challinor,
  Phys. Rep. 429 (2006) 1, Sec. 4.2), to second order in C_gl,2:

      C_gl(r)   = sum_L  w_L J_0(x r),   C_gl,2(r) = sum_L w_L J_2(x r),
      w_L = (1/2pi) x^3 C_L^{phi phi},   x = L + 1/2,       sigma^2(r) = C_gl(0) - C_gl(r),

      xi~(r)   = sum_l (x/2pi) C_l^TT e^{-x^2 sigma^2/2}
                 [ J_0 + (x^2 C_gl,2/2) J_2 + (x^4 C_gl,2^2/16)(J_0 + J_4) ]            (argument x r),
      xi~_+(r) = the same with C^EE (there are no primordial B modes),
      xi~_-(r) = sum_l (x/2pi) C_l^EE e^{-x^2 sigma^2/2}
                 [ J_4 + (x^2 C_gl,2/4)(J_2 + J_6) + (x^4 C_gl,2^2/16)(J_4 + (J_0 + J_8)/2) ],
      xi~_x(r) = sum_l (x/2pi) C_l^TE e^{-x^2 sigma^2/2}
                 [ J_2 + (x^2 C_gl,2/4)(J_0 + J_4) + (x^4 C_gl,2^2/32)(3 J_2 + J_6) ],

      C~_l^TT = 2pi Int r dr xi~(r) J_0(x r),            C~_l^TE = 2pi Int r dr xi~_x(r) J_2(x r),
      C~_l^EE = pi Int r dr [ xi~_+(r) J_0(x r) + xi~_-(r) J_4(x r) ].

  Only the DIFFERENCE between lensed and unlensed correlation functions is transformed
  back, so that the quadrature errors affect the small lensing correction only.

  The lensed spectrum at multipole l receives contributions from unlensed multipoles
  above l.  The Rust spectra end at l = 2500; they are continued to l = 3300 with an
  exponential fitted to 2100 <= l <= 2500 (and the lensing potential with a power law).
  Comparisons above l = 2000 depend on this continuation and are reported separately.

OUTPUT
  data/cmb_validation.csv, data/pk_validation.csv, and a summary on the terminal.
  Exit status 1 if a requirement listed at the end of the script is violated.

RUN
  python3 code/scripts/validate_cmb_against_class.py        (from the repository root)
"""
import io
import math
import os
import sys

import numpy as np
from scipy.interpolate import CubicSpline
from scipy.special import jv

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DATA = os.path.join(ROOT, "data")


def read_csv(path):
    """Read a CSV file with '#' comment lines and a header row; return a structured array."""
    with open(path) as f:
        rows = [line for line in f if not line.startswith("#")]
    return np.genfromtxt(io.StringIO("".join(rows)), delimiter=",", names=True)


def lens_flat_sky(ell, cl_tt, cl_te, cl_ee, ell_pp, cpp, r_max=0.35, dr=1.0e-4, apod=0.08):
    """Lensing corrections (Delta C_l^TT, Delta C_l^TE, Delta C_l^EE) on the multipoles `ell`.

    ell, cl_*   : multipoles and unlensed C_l (not D_l)
    ell_pp, cpp : multipoles and C_L^{phi phi} of the lensing potential
    r_max, dr   : extent and step of the grid in the separation angle r (radians)
    apod        : width (radians) of the cosine taper applied to the end of the r grid
    """
    r = (np.arange(int(r_max / dr)) + 0.5) * dr
    # deflection correlation functions
    xp = ell_pp + 0.5
    w = xp**3 * cpp / (2.0 * math.pi)
    cgl0 = w.sum()
    cgl = np.zeros_like(r)
    cgl2 = np.zeros_like(r)
    for chunk in np.array_split(np.arange(len(xp)), 20):
        arg = np.outer(r, xp[chunk])
        cgl += jv(0, arg) @ w[chunk]
        cgl2 += jv(2, arg) @ w[chunk]
    sigma2 = cgl0 - cgl
    x = ell + 0.5
    pref = x / (2.0 * math.pi)
    d_xi = np.zeros_like(r)      # temperature
    d_xip = np.zeros_like(r)     # xi_+
    d_xim = np.zeros_like(r)     # xi_-
    d_xix = np.zeros_like(r)     # xi_x
    # loop over blocks of r to limit the memory
    for blk in np.array_split(np.arange(len(r)), 40):
        arg = np.outer(r[blk], x)
        j0, j2, j4, j6, j8 = (jv(n, arg) for n in (0, 2, 4, 6, 8))
        s2 = sigma2[blk][:, None]
        c2 = cgl2[blk][:, None]
        x2 = (x * x)[None, :]
        damp = np.exp(-0.5 * x2 * s2)
        a1 = 0.5 * x2 * c2            # first-order coefficient  x^2 C_gl,2 / 2
        a2 = (x2 * c2) ** 2 / 16.0    # second-order coefficient x^4 C_gl,2^2 / 16
        k_t = damp * (j0 + a1 * j2 + a2 * (j0 + j4)) - j0
        k_m = damp * (j4 + 0.5 * a1 * (j2 + j6) + a2 * (j4 + 0.5 * (j0 + j8))) - j4
        k_x = damp * (j2 + 0.5 * a1 * (j0 + j4) + 0.5 * a2 * (3.0 * j2 + j6)) - j2
        d_xi[blk] = k_t @ (pref * cl_tt)
        d_xip[blk] = k_t @ (pref * cl_ee)
        d_xim[blk] = k_m @ (pref * cl_ee)
        d_xix[blk] = k_x @ (pref * cl_te)
    # taper
    win = np.ones_like(r)
    m = r > r_max - apod
    win[m] = 0.5 * (1.0 + np.cos(math.pi * (r[m] - (r_max - apod)) / apod))
    wr = r * dr * win
    d_tt = np.zeros_like(x)
    d_te = np.zeros_like(x)
    d_ee = np.zeros_like(x)
    for chunk in np.array_split(np.arange(len(x)), 40):
        arg = np.outer(x[chunk], r)
        j0, j2, j4 = jv(0, arg), jv(2, arg), jv(4, arg)
        d_tt[chunk] = 2.0 * math.pi * (j0 @ (wr * d_xi))
        d_te[chunk] = 2.0 * math.pi * (j2 @ (wr * d_xix))
        d_ee[chunk] = math.pi * (j0 @ (wr * d_xip) + j4 @ (wr * d_xim))
    rms_deflection_arcmin = math.sqrt(cgl0) * 180.0 * 60.0 / math.pi
    return d_tt, d_te, d_ee, rms_deflection_arcmin


def main():
    mine = read_csv(os.path.join(DATA, "cmb_fiducial_cl.csv"))
    ref = read_csv(os.path.join(DATA, "reference", "class_lcdm_lensed_cl.csv"))
    ell = mine["ell"]
    assert np.array_equal(ell, ref["ell"]) and ell[0] == 2 and ell[-1] == 2500
    to_cl = 2.0 * math.pi / (ell * (ell + 1.0))

    # continuation above l = 2500 (see the header)
    l_ext = np.arange(2501, 3301, dtype=float)
    fit_rng = ell >= 2100

    def extend(dl, signed=False):
        if signed:
            # the cross spectrum oscillates about zero: continue its envelope times the last oscillation
            env = np.polyfit(ell[fit_rng], np.log(np.abs(dl[fit_rng]) + 1e-3), 1)
            return np.exp(np.polyval(env, l_ext)) * np.cos(math.pi * (l_ext - 2500.0) / 150.0) * np.sign(dl[-1])
        coef = np.polyfit(ell[fit_rng], np.log(dl[fit_rng]), 1)
        return np.exp(np.polyval(coef, l_ext))

    ell_all = np.concatenate([ell, l_ext])
    to_cl_all = 2.0 * math.pi / (ell_all * (ell_all + 1.0))
    tt_all = np.concatenate([mine["Dl_TT"], extend(mine["Dl_TT"])]) * to_cl_all
    ee_all = np.concatenate([mine["Dl_EE"], extend(mine["Dl_EE"])]) * to_cl_all
    te_all = np.concatenate([mine["Dl_TE"], extend(mine["Dl_TE"], signed=True)]) * to_cl_all
    # lensing potential: C_L^{phi phi} = 2 pi L4Cpp / [L(L+1)]^2, continued with a power law
    cpp = 2.0 * math.pi * ref["L4Cpp"] / (ell * (ell + 1.0)) ** 2
    pw = np.polyfit(np.log(ell[ell >= 1500]), np.log(cpp[ell >= 1500]), 1)
    cpp_all = np.concatenate([cpp, np.exp(np.polyval(pw, np.log(l_ext)))])

    d_tt, d_te, d_ee, defl = lens_flat_sky(ell_all, tt_all, te_all, ee_all, ell_all, cpp_all)
    n = len(ell)
    lensed = {
        "TT": mine["Dl_TT"] + d_tt[:n] / to_cl,
        "TE": mine["Dl_TE"] + d_te[:n] / to_cl,
        "EE": mine["Dl_EE"] + d_ee[:n] / to_cl,
    }
    print(f"rms deflection angle from the reference lensing potential: {defl:.3f} arcmin (expected: about 2.4 arcmin)")

    # relative differences; the cross spectrum is normalized by sqrt(TT EE) because it has zeros
    norm_te = np.sqrt(ref["Dl_TT"] * ref["Dl_EE"])
    diff = {
        "TT": lensed["TT"] / ref["Dl_TT"] - 1.0,
        "EE": lensed["EE"] / ref["Dl_EE"] - 1.0,
        "TE": (lensed["TE"] - ref["Dl_TE"]) / norm_te,
    }
    diff_unlensed = {
        "TT": mine["Dl_TT"] / ref["Dl_TT"] - 1.0,
        "EE": mine["Dl_EE"] / ref["Dl_EE"] - 1.0,
        "TE": (mine["Dl_TE"] - ref["Dl_TE"]) / norm_te,
    }
    # cosmic variance of one multipole for a full sky, in the same units, for orientation
    cv = np.sqrt(2.0 / (2.0 * ell + 1.0))

    ranges = [(2, 29), (30, 99), (100, 499), (500, 999), (1000, 1499), (1500, 2000), (2001, 2500)]
    print("\nlensed Rust spectra against CLASS (maximum and rms of the differences defined above)")
    print(f"{'l range':>12} " + " ".join(f"{q + ' max':>10} {q + ' rms':>10}" for q in ("TT", "TE", "EE")) + "   (without lensing: TT max)")
    summary = {}
    for lo, hi in ranges:
        m = (ell >= lo) & (ell <= hi)
        cells = []
        for q in ("TT", "TE", "EE"):
            mx, rms = np.abs(diff[q][m]).max(), math.sqrt(np.mean(diff[q][m] ** 2))
            summary[(lo, hi, q)] = (mx, rms)
            cells.append(f"{mx:10.2e} {rms:10.2e}")
        print(f"{lo:>5}-{hi:<6} " + " ".join(cells) + f"   {np.abs(diff_unlensed['TT'][m]).max():10.2e}")

    with open(os.path.join(DATA, "cmb_validation.csv"), "w") as f:
        f.write("# generated by: code/scripts/validate_cmb_against_class.py\n")
        f.write("# content: spectra of the fiducial LambdaCDM model: Rust solver without lensing (rust_unlensed), the same lensed with the lensing potential of the reference by the flat-sky correlation-function method (rust_lensed), and CLASS v3.4.2 with lensing (class)\n")
        f.write("# units: D_l = l(l+1) C_l/(2 pi) in microkelvin^2\n")
        f.write("# differences: rel_TT = TT_rust_lensed/TT_class - 1; rel_EE likewise; d_TE = (TE_rust_lensed - TE_class)/sqrt(TT_class EE_class); *_unl: the same for the unlensed Rust spectra\n")
        f.write(f"# rms_deflection_arcmin: {defl:.4f}\n")
        for (lo, hi, q), (mx, rms) in summary.items():
            f.write(f"# summary_{q}_{lo}_{hi}: max {mx:.3e}, rms {rms:.3e}\n")
        f.write("ell,TT_rust_unlensed,TT_rust_lensed,TT_class,TE_rust_unlensed,TE_rust_lensed,TE_class,EE_rust_unlensed,EE_rust_lensed,EE_class,rel_TT,d_TE,rel_EE,rel_TT_unl,d_TE_unl,rel_EE_unl,cosmic_variance\n")
        for i in range(n):
            vals = [ell[i], mine["Dl_TT"][i], lensed["TT"][i], ref["Dl_TT"][i], mine["Dl_TE"][i], lensed["TE"][i], ref["Dl_TE"][i],
                    mine["Dl_EE"][i], lensed["EE"][i], ref["Dl_EE"][i], diff["TT"][i], diff["TE"][i], diff["EE"][i],
                    diff_unlensed["TT"][i], diff_unlensed["TE"][i], diff_unlensed["EE"][i], cv[i]]
            f.write(",".join(f"{v:.10e}" for v in vals) + "\n")

    # ---- matter power spectrum ----
    pk = read_csv(os.path.join(DATA, "cmb_fiducial_pk.csv"))
    pref = read_csv(os.path.join(DATA, "reference", "class_lcdm_pk.csv"))
    h = 0.6736
    k_ref = pref["k_h_Mpc"] * h            # 1/Mpc
    p_ref = pref["Pk"] / h**3              # Mpc^3
    m = (k_ref >= pk["k"][0]) & (k_ref <= pk["k"][-1])
    # cubic spline of ln P against ln k (the Rust wavenumber grid is coarse around the maximum of P)
    p_mine = np.exp(CubicSpline(np.log(pk["k"]), np.log(pk["P"]))(np.log(k_ref[m])))
    rel = p_mine / p_ref[m] - 1.0
    print("\nlinear matter power spectrum today, Rust / CLASS - 1:")
    pk_summary = []
    for lo, hi in [(1e-4, 1e-3), (1e-3, 1e-2), (1e-2, 1e-1), (1e-1, 1.0), (1.0, 3.0)]:
        mm = (k_ref[m] >= lo) & (k_ref[m] < hi)
        if mm.any():
            pk_summary.append((lo, hi, np.abs(rel[mm]).max()))
            print(f"  {lo:8.0e} <= k < {hi:8.0e} 1/Mpc: max |difference| = {np.abs(rel[mm]).max():.2e}")
    with open(os.path.join(DATA, "pk_validation.csv"), "w") as f:
        f.write("# generated by: code/scripts/validate_cmb_against_class.py\n")
        f.write("# content: linear matter power spectrum of the fiducial model today: Rust solver (interpolated to the reference wavenumbers) and CLASS v3.4.2\n")
        f.write("# units: k in 1/Mpc, P in Mpc^3\n")
        for lo, hi, mx in pk_summary:
            f.write(f"# summary_{lo:.0e}_{hi:.0e}: max |P_rust/P_class - 1| = {mx:.3e}\n")
        f.write("k,P_rust,P_class,rel\n")
        for a, b, c, d in zip(k_ref[m], p_mine, p_ref[m], rel):
            f.write(f"{a:.10e},{b:.10e},{c:.10e},{d:.10e}\n")

    # ---- requirements ----
    # 100 <= l <= 2000:  TT within 0.3 per cent, TE within 0.003 sqrt(TT EE), EE within 0.5 per cent;
    #  30 <= l <   100:  TT within 0.3 per cent, TE within 0.01 sqrt(TT EE), EE within 2 per cent
    #                    (the two codes describe reionization slightly differently, and the cosmic
    #                    variance of one multipole is 10 to 20 per cent there);
    # matter power spectrum within 0.5 per cent for 1e-4 <= k <= 3 /Mpc.
    limits = {"TT": (3.0e-3, 3.0e-3), "TE": (1.0e-2, 3.0e-3), "EE": (2.0e-2, 5.0e-3)}
    ok = True
    for (lo, hi, q), (mx, rms) in summary.items():
        if lo == 30:
            ok &= mx < limits[q][0]
        elif 100 <= lo and hi <= 2000:
            ok &= mx < limits[q][1]
    for lo, hi, mx in pk_summary:
        ok &= mx < 5.0e-3
    print("\nrequirements met:", ok)
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
