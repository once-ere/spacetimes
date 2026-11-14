#!/usr/bin/env python3
"""write_symbol_registry.py (Revision_Deflation_CJT_and_Kohn-Sham_definition_C) -- the MASTER registry of the
symbols of PROJECT spacetimes.

Hard Rule 5 of the project requires that every symbol of the project denote ONE object.
This script builds the single list of all symbols of the project and writes it as

    provenance/symbols_master.csv                 the registry (machine-readable)
    provenance/generated/symbols_master.md        the registry as Markdown tables (fragments),

from three sources:

  (1) the registry of the flat-space part of the project, ../provenance/symbols.csv;
  (2) the registry of the first revision on the deflating spacetimes,
      ../Revision_Deflation/provenance/symbols_deflation.csv;
  (3) the list NEW below: the symbols that this sub-project adds (the metric in the
      author's sign convention, the mass term and the self-interaction, the Kohn-Sham
      and CJT calculations, the pairing with contact couplings, the gravity with
      Kohn-Sham matter).

The entries of (1) and (2) are taken over with their symbols and their names in programs
unchanged; the list REWORD replaces the meanings that pointed to another document or to
"the main registry" by complete definitions, MERGE joins the entries of (1) and (2) that
describe the same symbol, and DROP removes what this sub-project replaces (the metric
symbols of the earlier sign convention, the tags of the earlier documents).  Every
definition of the output is therefore complete in itself: the registry does not send the
reader to any other file.

The script stops if two entries carry the same symbol text or the same name in programs.

Run from the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C:

    python3 code/scripts/write_symbol_registry.py
    python3 code/scripts/check_symbols.py

Columns of the output: group; symbol as written in the documents (LaTeX); names in
programs and data files (blank-separated); meaning; origin (which of the author's
notebooks or papers the symbol comes from -- the key of the codes N01.., D01.., K1.. is
in the fragment `symbols_origin_key` -- or "author" for a name that an instruction of
the author fixes, or "new").
"""
import csv
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)


def read_registry(path):
    with open(path, encoding="utf-8") as f:
        rows = list(csv.reader(line for line in f if not line.startswith("#")))
    return rows[1:]


# ---------------------------------------------------------------------------------------
# groups of the new entries
# ---------------------------------------------------------------------------------------
G10 = "10 The metric of the deflating spacetimes in the author's sign convention"
G11 = "11 Mass term and self-interaction of the fermion field"
G12 = "12 Kohn-Sham and CJT calculations with contact interaction"
G13 = "13 Pairing with contact couplings"
G14 = "14 Einstein-Lovelock gravity with Kohn-Sham matter"
G15 = "15 Tags and identifiers of the calculations with mass term and self-interaction"

# names of the groups of source (2), which carried the prefix D
GROUP_NAMES = {
    "D1 Hypotheses and geometry": "D1 Deflating spacetimes: hypotheses and geometry",
    "D2 Matrices, fields and their equations": "D2 Deflating spacetimes: matrices, fields and their equations",
    "D3 Solutions mode by mode": "D3 Deflating spacetimes: solutions mode by mode",
    "D4 The deflation clock and the couplings": "D4 The deflation clock and the couplings",
    "D5 Dynamics of the paired state": "D5 Dynamics of the paired state",
    "D6 Cosmology and bounds": "D6 Cosmology and bounds on the deflating spacetimes",
    "D7 Symmetries and vortex lines": "D7 Symmetries and vortex lines on the deflating spacetimes",
    "D8 Tags and identifiers": "D8 Tags and identifiers of the deflating spacetimes",
    "D9 Perturbations of ordinary space and the microwave background": "D9 Perturbations of ordinary space and the microwave background",
}

# ---------------------------------------------------------------------------------------
# entries of the sources that are removed: (source, symbol text) -> reason
# ---------------------------------------------------------------------------------------
DROP = {
    ("D", r"$g_{1644d}$, $g_{1671d}$; $g_{\mu\nu}$"): "replaced by the metric symbols of group 10",
    ("D", r"$a,b$; $j$ (indices)"): "merged into the entries of the indices in group 1",
    ("M", r"$t$ (index)"): "merged into the entry of the indices t, s in group D1",
    ("D", r"$\mathsf M$ (main registry)"): "merged into the entry of the gap matrix in group 5",
    ("D", r"$\tau$ (main registry)"): "merged into the entry of the cosmological parameters in group 9",
    ("D", r"RP0 to RP9, RS1 to RS5, DMD"): "tags of the documents of the earlier revision; not symbols",
    ("D", r"`44d-..`, `71d-..`, `def-..`"): "replaced by the entry of the check identifiers in group 15",
}

# ---------------------------------------------------------------------------------------
# new meanings (and, where given, new names in programs): (source, symbol text) -> (meaning, names or None)
# ---------------------------------------------------------------------------------------
REWORD = {
    ("M", r"$a,b$"): (
        r"indices whose range is stated at each place of use: the four extra axes, values 0, 5, 6, 7 (in everything that concerns pairs and the order parameter), the four flat coordinates other than the time, values 0, 1, 2, 3 (in the statements of the machine checks), or the sixteen components of the field, values 1 to 16 (in the density matrix $\mathsf n_{ab}$). The letters $\mathsf a$, $\mathsf b$ in sans-serif type are the scale factors of the Einstein-Lovelock evolution; $a_{\rm sp}$, $a_{\rm sc}$, $a(T)$, $b_t$ and $b$ have their own entries", None),
    ("M", r"$i,j$"): (
        r"index of space-like coordinates; values 1, 2, 3 (ordinary space) unless the range is stated: $j=0,1,2,3$ for the four flat coordinates other than the time where the deflating coordinates occur as well, and all seven space-like directions in the Hamiltonian of 7+1. In the Einstein-Lovelock equations $i,j$ label the seven space directions", None),
    ("M", r"$\eta_{1671}$"): (
        r"the matrix of the squares of the Dirac matrices of 7+1, $\gamma_{1671}^A\gamma_{1671}^B+\gamma_{1671}^B\gamma_{1671}^A=2\,\eta_{1671}^{AB}$: $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$, time at index 4. The flat-space part of the project uses it as the metric (time-like $=+1$). It is minus the flat metric $\eta^{(7+1)}$ of the author's sign convention for the deflating spacetimes (space-like $=+1$)", None),
    ("M", r"$\eta_{1644}$"): (
        r"metric of the flat 4+4 spacetime and matrix of the squares of its Dirac matrices, $\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$: space-like $=+1$. It equals the flat metric $\eta^{(4+4)}$ of the author's sign convention for the deflating spacetimes", None),
    ("M", r"$\Delta^a$"): (
        r"order parameter: expectation value of the vector pair operator along the extra axis $a\in\{0,5,6,7\}$; $\lvert\Delta\rvert^2\equiv\sum_a\lvert\Delta^a\rvert^2$", None),
    ("M", r"$a_{\rm sc}$, $a_{BB}$"): (r"scattering length of two fermions; scattering length of two bound pairs", None),
    ("M", r"$A_2(T)$, $A_4$"): (r"coefficients of the expansion of the CJT effective potential $V$ along a gap function of fixed shape", None),
    ("M", r"$a(T)$, $b$, $K_{\rm GL}$"): (r"Ginzburg-Landau coefficients of the weak-coupling free energy (quadratic, quartic and gradient term)", None),
    ("M", r"$m^*$, $x$"): (
        r"$m^*$: the mass of the fermions in matter; $x=\phi/v_{VS}$: the expectation value of the Vorton-Scalar in units of its vacuum value. With a contact interaction $m^*=m+G_S\,n_S$ (gap equation of the Kohn-Sham and CJT calculations); with the Vorton-Scalar and a bare Dirac mass $m^*=m_D+m_Y\,x+G_S\,n_S$, so that $x=m^*/m$ if the whole mass comes from the Vorton-Scalar and no contact coupling acts. $m^*_+$, $m^*_-$: the masses of the particles of the two halves $W_{1644}=\pm1$ of the 4+4 field, $m^*_\pm=m+G_S^{\rm tot}\,n_S\pm G_S^{\rm dif}\,n_{SW}$ in both forms of the self-interaction (with the filled states of negative energy: the masses of the two halves in vacuum). The data files and the statements of the programs write $m^*$ as `M` and $m^*_\pm$ as `M_plus`, `M_minus` (`Mp_..`, `Mm_..` with a suffix that names the method or the strength; `M_large`, `M_small`: the larger and the smaller of the two); the author's document KOHN_SHAM_GRAVITY writes $M$",
        "M M_ks M_cjt M_cjt_global M_plus M_minus M_relaxed x Mp_ks Mp_cjt Mm_ks Mm_cjt Mp_below Mm_below Mp_above Mm_above M_large M_small"),
    ("M", r"$g_{VS}$"): (
        r"coupling of both modes of the Vorton-Scalar to the fermions, $g_{VS}=y_{VS}/\sqrt2=m_Y/v_{VS}$, where $m_Y$ is the part of the fermion mass that the Vorton-Scalar gives (equal to $m/v_{VS}$ when there is no bare Dirac mass)", None),
    ("M", r"$m_{\rm deg}$, $m_{B,\max}$"): (
        r"largest fermion mass for degeneracy, $m_{\rm deg}=[6\pi^2\hbar^3\rho/(\nu\sigma_v^3)]^{1/4}$, and largest mass $m_B$ of a boson for condensation, $m_{B,\max}=[(2\pi)^{3/2}\hbar^3\rho/(2.612\,\sigma_v^3)]^{1/4}$, in a halo of the mass density $\rho$ and the velocity dispersion $\sigma_v$", None),
    ("M", r"$\mathsf M$"): (r"the $8\times8$ gap matrix on the internal states of a pair (sans-serif; `M_gap` in the statements of the programs)", "M_gap"),
    ("M", r"$\omega_b$, $\omega_c$, $h$, $H_0$, $A_s$, $n_s$, $\tau$"): (
        r"the six parameters of the standard cosmological model: densities of baryons and of cold dark matter ($\Omega h^2$), Hubble constant $H_0=100\,h$ km s$^{-1}$ Mpc$^{-1}$, amplitude and spectral index of the primordial fluctuations, optical depth of reionization (`tau_reio` in the data files; the letter $\tau$ with the index $d$ is the deflation clock)",
        "tau_reio"),
    ("M", r"$k_F$, $E_F$, $\mu$"): (
        r"Fermi momentum; Fermi kinetic energy $\sqrt{m^2+k_F^2}-m$; chemical potential. The number density is $n=\nu V_Dk_F^D/(2\pi)^D$ in $D$ space dimensions ($n=\nu k_F^3/6\pi^2$ for $D=3$)", "kF mu"),
    ("M", r"$W(k,k')$"): (
        r"kernel of the gap equation: the $s$-wave average of the pairing attraction, $W(k,k')=\tfrac12\int_{-1}^{1}d\cos\theta\,\big[-V_{VS}-V_{\rm ct}\big]$ for the momenta $k$, $k'$ of the pair before and after the scattering ($\theta$: the angle between them); without contact couplings the exchange attraction alone. Always written with its two arguments ($W_{1644}$ is a matrix)", None),
    ("M", r"$\alpha_c$, $\Lambda$ (cutoff)"): (
        r"critical value of $\alpha_{VS}$ for a pair condensate in the vacuum; $\Lambda$: a momentum cutoff (of the ladder equation in the vacuum, and of the filled states of negative energy in the calculations with contact interaction). $\Lambda_8$ and $\bar\Lambda$ are cosmological terms and have their own entries",
        "Lambda"),
    ("M", r"$\varepsilon(k_F,x)$, $V_{\rm sea}(x)$"): (
        r"energy density of matter as a function of the Fermi momentum and of $x=m^*/m$ (in units of $m^4$ in ordinary space); renormalized energy of the filled negative-energy states. $\varepsilon$ without arguments is the energy density of matter in the units of the calculation at hand", None),
    ("D", r"$b_5$, $b_6$, $b_7$; $b_t$"): (
        r"the scale factors of the deflating coordinates, $b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$: a coordinate distance $dxt$ is the proper length $b_t\,\lvert dxt\rvert$. Always written with the index 5, 6, 7 or $t$ (the letter $b$ without index is a Ginzburg-Landau coefficient, and $\mathsf b$ in sans-serif type is the common scale factor of the three deflating coordinates in the Einstein-Lovelock evolution)", None),
    ("D", r"$\mu,\nu,\rho,\sigma$ (indices)"): (
        r"coordinate indices with the values $0,\dots,7$ on the deflating spacetimes (only as sub- or superscripts; as stand-alone letters $\mu$ is the chemical potential, $\nu$ the number of internal states and $\lambda$ the pairing coupling). The capital indices $A,B,C,D$, also with a prime, label the orthonormal frame: the frame vector $A$ points along the coordinate $xA$ (as a stand-alone letter $C$ is the coupling $g_{VS}^2m^2/m_s^2$, and $D$ the number of space dimensions with momentum). In the statements of the programs the indices are spelled out inside tensor expressions, as in `g^(mu nu)`", "="),
    ("D", r"$t$, $s$ (indices)"): (
        r"indices of the three additional coordinates $x5,x6,x7$, values 5, 6, 7; $s$ is a second such index where two are needed (the matrices $s_h$ carry the index $h$)", None),
    ("D", r"$\Gamma^\rho{}_{\mu\nu}$"): (
        r"the Christoffel symbols of the deflating metric (three indices; $\Gamma[\phi,G]$ with square brackets is the CJT effective action). The author's document KOHN_SHAM_GRAVITY writes the Christoffel symbols with braces and uses $\Gamma^A$ for the Dirac matrices, which are $\gamma^A$ here", None),
    ("D", r"$\mathcal X$; $\mathcal X_{\rm scale}$, $\mathcal X_{(s)}$"): (
        r"a Killing vector field of the deflating metric (script letter; the plain letter $X$ is the set of the eight coordinates); the scaling vector $\partial_4+\sum_t\mathrm{rate}_t\,xt\,\partial_t$; the three special Killing vectors of the case of equal rates, $s=5,6,7$", None),
    ("D", r"$\mathcal H^\mu{}_\nu$; $\alpha_{GB}$, $\Lambda_8$"): (
        r"the Lanczos tensor of the Gauss-Bonnet term, $\mathcal H^\mu{}_\nu=2\big(R\,R^\mu{}_\nu-2R^\mu{}_\alpha R^\alpha{}_\nu-2R^{\mu\alpha}{}_{\nu\beta}R^\beta{}_\alpha+R^{\mu\alpha}{}_{\beta\gamma}R^{\beta\gamma}{}_{\nu\alpha}\big)-\tfrac12\delta^\mu_\nu\big(R^2-4R^\alpha{}_\beta R^\beta{}_\alpha+R^{\alpha\beta}{}_{\gamma\delta}R^{\gamma\delta}{}_{\alpha\beta}\big)$ (script letter; $H$ is the Hubble rate); the coupling of the Gauss-Bonnet term and the cosmological constant of Einstein-Gauss-Bonnet gravity in eight dimensions, whose equations without matter are $\mathcal G^\mu{}_\nu+\Lambda_8\,\delta^\mu_\nu+\alpha_{GB}\,\mathcal H^\mu{}_\nu=0$ (metric with positive space-like components). In the author's Einstein-Lovelock notation $E^{(1)}=-\mathcal G$, $E^{(2)}=\mathcal H$, $\alpha_2=-\alpha_{GB}$, $\bar\Lambda=\Lambda_8$", None),
    ("D", r"$K_{AB}$"): (
        r"the sectional curvature of the plane of the frame directions $A$ and $B$ (metric with positive space-like components): on the deflating spacetimes $K_{4t}=\mathrm{rate}_t^2$, $K_{ts}=\mathrm{rate}_t\,\mathrm{rate}_s$ for $t\neq s$, zero if one direction is flat; for scale factors that depend on $x4$ in any way, $K_{4i}=H_i'+H_i^2$ and $K_{ij}=H_iH_j$ with the rate $H_i$ of the direction $i$ (the pair curvatures of the author's Einstein-Lovelock equations). Always with two subscripts: the letter $K$ without subscript is the generator of the evolution of the 4+4 field in $x4$, $K$ as an index is an index set of the self-interaction, and $K$ with an order as subscript and with an argument is a modified Bessel function", None),
    ("D", r"$P_\mu$"): (r"the conserved covariant momenta of a geodesic (with an index; $P$ without index is the generator of $U(1)_E$)", None),
    ("D", r"$\omega_{\mu AB}$"): (
        r"the coefficients of the spin connection, $\omega_{\mu AB}=\eta_{AC}\,e^C{}_\nu\big(\partial_\mu e_B{}^\nu+\Gamma^\nu{}_{\mu\lambda}e_B{}^\lambda\big)$ with the flat metric $\eta=\eta^{(4+4)}$ or $\eta^{(7+1)}$ (three indices; $\omega_{1644}$, $\omega_{1671}$ with a tag are the products of all eight Dirac matrices)", None),
    ("D", r"$\Omega_\mu$"): (
        r"the spin connection as a matrix, $\Omega_\mu=\tfrac14\,c_\gamma\,\omega_{\mu AB}\gamma^A\gamma^B$; the covariant derivative of a spinor is $\nabla_\mu\psi=(\partial_\mu+\Omega_\mu)\psi$. On both deflating spacetimes $\Omega_t=\tfrac12\,\mathrm{rate}_t\,b_t\,\gamma^t\gamma^4$ (frame matrices) for $t=5,6,7$ and $\Omega_\mu=0$ otherwise. ($\Omega_{\rm CJT}$ is an effective potential)", None),
    ("D", r"$\psi_{1644d}$, $\psi_{1671d}$"): (
        r"the fields dirac1644d and dirac1671d: the 16-component fields on the deflating spacetimes. With mass term and self-interaction they obey $(\gamma^\mu\nabla_\mu-m)\psi_{1644d}=\sum_rg_r\sum_{\lvert K\rvert=r}(\tilde\psi\Theta_K\psi)\Theta^K\psi$ (covariant form of 4+4) and $(i\gamma^\mu\nabla_\mu-m)\psi_{1671d}=\sum_rg_r\sum_{\lvert K\rvert=r}(\bar\psi\Theta_K\psi)\Theta^K\psi$ (7+1)", None),
    ("D", r"$K(x4)$, $\mathsf h(x4)$"): (
        r"the generator $K_{1644}$ and the Hamiltonian matrix $\mathsf h_{1671}$ with the wavenumbers along the deflating coordinates divided by the scale factors: without self-interaction $\partial_4\hat\psi=K\hat\psi$ (4+4), $i\partial_4\hat\psi=\mathsf h\hat\psi$ (7+1)", None),
    ("D", r"$w$; $f_+$, $f_-$; $Z_+$, $Z_-$"): (
        r"auxiliary objects of the exact solutions of the equations without self-interaction: a constant eigenvector of $\mathsf h_\perp$ or of $K_\perp$; the two coefficient functions of the ansatz; two cylinder functions whose orders differ by one", None),
    ("D", r"$\tau_d$"): (
        r"the deflation clock, $\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$: the number of e-foldings by which the volume $V_d$ has shrunk since the reference time; in an evolution in which the rates change, $\tau_d=-\ln(V_d/V_d^{\rm ref})$ (the letter $\tau$ without the index $d$ is the optical depth of reionization)", None),
    ("D", r"$a_{\rm sp}$, $H$"): (
        r"scale factor of ordinary space (assumption A1) and its rate $H=(da_{\rm sp}/dx4)/a_{\rm sp}$; $H_0$ is its value today. (The rates of the Einstein-Lovelock evolution are $H_{\mathsf a}$, $H_{\mathsf b}$; the author's Einstein-Lovelock notebooks use the letter $H$ for the rate in the exponent of their metric, which corresponds here to $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$)", None),
    ("D", r"$\theta_*$, $z_*$"): (
        r"the angle under which the sound horizon at last scattering is seen, $r_s(z_*)/D_A(z_*)$ with the comoving distance $D_A$, and the redshift $z_*$ of last scattering at which the optical depth without reionization is 1 (the $100\,\theta_*$ of the Planck papers). The angle $\theta_s$ is the same ratio at the maximum of the visibility function; for the model without deflation $100\,\theta_*=1.03965$ and $100\,\theta_s=1.040358$", None),
    ("D", r"$v_{\rm EW}$ (main project), $n_{\rm EW}$"): (
        r"the electroweak scale (the vacuum value of the Higgs field of the Standard Model, 246 GeV), and the exponent with which it would follow the coupling under the hypothesis H2: $n_{\rm EW}=d\ln v_{\rm EW}/d\ln\alpha_{VS}$", None),
    ("D", r"$P_c$"): (r"$i\alpha^1\alpha^2\alpha^3\chi$, the product of the five matrices of the sector that does not depend on the extra coordinates; equal, up to sign, to the generator $P$ of $U(1)_E$", None),
    ("D", r"1644d, 1671d"): (r"tags of the deflating 4+4 and 7+1 spacetimes and of every object defined on them (the tags 1644, 1671 of the flat spacetimes with the letter d)", None),
    ("D", r"A1, A2, A3, H2, HV"): (
        r"A1, A2: the two assumptions of the cosmological part (ordinary space expands; gravity is the eight-dimensional Einstein theory). A3: the assumption about the perturbations of ordinary space (the scale factors of the extra coordinates are not perturbed, and the agent that holds them has no energy density), with its closures. H2: the author's hypothesis that the superfluid state is the Higgs field. HV: the different assumption that the vacuum value of the Vorton-Scalar sets the electroweak scale", None),
    ("D", r"$\tau_{\rm cf}$"): (
        r"the conformal time of ordinary space, $d\tau_{\rm cf}=dx4/a_{\rm sp}$, in Mpc (`tau0_Mpc`: its value today). In the equations of the perturbations a prime is the derivative with respect to $\tau_{\rm cf}$", None),
    ("D", r"$k$ (perturbations), $\ell$, $D_\ell^{TT}$, $D_\ell^{TE}$, $D_\ell^{EE}$"): (
        r"the comoving wavenumber of a perturbation of ordinary space (in Mpc$^{-1}$; the physical wavenumber is $k/a_{\rm sp}$); the multipole; and the spectra of the temperature, of the polarization and their cross spectrum, $D_\ell=\ell(\ell+1)C_\ell/2\pi$ in $\mu$K$^2$", None),
}

# ---------------------------------------------------------------------------------------
# the symbols that this sub-project adds: (group, symbol, names in programs, meaning, origin)
# ---------------------------------------------------------------------------------------
NEW = [
    # ---- 10 ----
    (G10, r"$g^{(4+4)}_{\mu\nu}$, $g^{(7+1)}_{\mu\nu}$", "gs",
     r"the metrics of the deflating spacetimes in the author's sign convention of November 13, 2026 (space-like components positive, time-like components negative): $g^{(4+4)}=\mathrm{diag}(+1,+1,+1,+1,-1,-b_5^2,-b_6^2,-b_7^2)$ and $g^{(7+1)}=\mathrm{diag}(+1,+1,+1,+1,-1,+b_5^2,+b_6^2,+b_7^2)$, that is $g^{(4+4)}_{tt}=-\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$ and $g^{(7+1)}_{tt}=+\mathrm{const}_t^2\,e^{-2\,\mathrm{rate}_t\,x4}$ for $t=5,6,7$. A component of the metric carries the superscript $(4+4)$ or $(7+1)$ and two coordinate indices; it is not a rate ($\mathrm{g55}$) and not a coupling ($g_r$, $g_{VS}$). The first revision on the deflating spacetimes wrote $g_{1644d}=g^{(4+4)}$ and $g_{1671d}=-g^{(7+1)}$ (time-like component positive on 7+1); these two symbols are retired",
     "author"),
    (G10, r"$\eta^{(4+4)}$, $\eta^{(7+1)}$", "",
     r"the flat metrics of the author's sign convention, with which frame indices are raised and lowered on the deflating spacetimes: $\eta^{(4+4)}=\eta_{1644}=\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$ and $\eta^{(7+1)}=-\eta_{1671}=\mathrm{diag}(+1,+1,+1,+1,-1,+1,+1,+1)$; $g_{\mu\nu}\,e_A{}^\mu e_B{}^\nu=\eta_{AB}$",
     "author"),
    (G10, r"$c_\gamma$", "clifford_sign",
     r"the sign that relates the Dirac matrices to the metric of the author's sign convention, $\gamma^A\gamma^B+\gamma^B\gamma^A=2\,c_\gamma\,\eta^{AB}$ with $\eta=\eta^{(4+4)}$ or $\eta^{(7+1)}$: $c_\gamma=+1$ on 4+4 and $c_\gamma=-1$ on 7+1. The Dirac matrices themselves are those of the flat-space part of the project, unchanged",
     "new"),
    (G10, r"$b_t^{\rm ref}$", "s5 s6 s7",
     r"the value of the scale factor $b_t$ at the reference time $x4_{\rm ref}$. A particle that keeps its wavenumber $k_t$ has the proper momentum $k_t/b_t$, larger than at the reference time by the stretch factor $b_t^{\rm ref}/b_t=e^{\mathrm{rate}_t(x4-x4_{\rm ref})}$ (columns `s5`, `s6`, `s7`)",
     "new"),
    (G10, r"$\tau_d^{0}$", "tau_ref tau_ref_end",
     r"the reading that the deflation clock would have if the rates had kept their initial value, $\tau_d^0=3\,\mathrm{g55}\,x4$ in the evolutions with gravity (equal rates), where the clock actually reached is $\tau_d=-3\ln\mathsf b$",
     "new"),
    # ---- 11 ----
    (G11, r"$\mathcal L_{\rm self}$; $\mathcal L_{\rm self}^{\rm C}$, $\mathcal L_{\rm self}^{\rm Q}$", "",
     r"the self-interaction of the fermion field (the author's definition, document KOHN_SHAM_GRAVITY): $\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r$, a sum of nine quartic invariants. On the 4+4 spacetime the invariants can be formed in two ways that are not equivalent: $\mathcal L_{\rm self}^{\rm C}=-\tfrac12\sum_rg_r\,I_r^{\rm C}$ (form C, the definition of the 4+4 model) and $\mathcal L_{\rm self}^{\rm Q}=-\tfrac12\sum_rg_r\,I_r^{\rm Q}$ (form Q, the alternative)",
     "K1; the two forms on 4+4 are new"),
    (G11, r"$g_r$; $g_0,\dots,g_8$; $g_S$, $g_V$, $g_A$, $g_P$", "g_0 g_1 g_2 g_3 g_4 g_5 g_6 g_7 g_8",
     r"the nine couplings of the self-interaction, one for each rank $r$ (the author's symbols; dimension length$^{D-1}$ in $D$ space dimensions); $g_0=g_S$ (scalar), $g_1=g_V$ (vector), $g_7=-g_A$ (axial vector), $g_8=-g_P$ (pseudoscalar). Such a coupling carries one digit as subscript; it is not a rate of deflation ($\mathrm{g55}$: two digits, upright, no subscript), not a component of the metric (superscript $(4+4)$ or $(7+1)$, two indices) and not the coupling $g_{VS}$ of the Vorton-Scalar",
     "K1"),
    (G11, r"$K$, $J$ (index sets); $r$, $r'$ (ranks)", "mask rank J",
     r"$K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices)",
     "K1"),
    (G11, r"$\Theta^K$, $\Theta_K$; $\zeta_r$; $\mathrm{sgn}_K$", "eps_K phase eta_K",
     r"the 256 matrices of the self-interaction (the author's symbols $\Theta^K$, $\Theta_K$): $\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$, with the phase $\zeta_r=1$ or $i$ chosen so that the bilinear formed with $\Theta^K$ is real, and $\Theta_K=(\Theta^K)^{-1}=\mathrm{sgn}_K\,\Theta^K$ with the sign $\mathrm{sgn}_K=\pm1$, so that $(\Theta^K)^2=\mathrm{sgn}_K$. On 7+1: $\zeta_r=i$ for $r=2,3,6,7$ (the author's $i^{\sigma_r}$ with $\sigma_r=r(r-1)/2$ mod 2), and $\mathrm{sgn}_K$ is the product of the squares of the Dirac matrices in $K$ (the author's $\eta_K$; column `eta_K`). On 4+4 (form C, the definition): $\zeta_r=i$ for $r=1,2,5,6$, and $\mathrm{sgn}_K$ is $(-1)^r$ times that product. (The sign is written with the letters sgn, because $\eta$ and $\varepsilon$ have other meanings; columns `eps_K`, `phase`.)",
     "K1; form C of 4+4 is new"),
    (G11, r"$\gamma_{44q}^A$; $\Theta_Q^K$, $\zeta_r^Q$, $\mathrm{sgn}_K^Q$", "gamma44q",
     r"the matrices with which form Q (the quantized-sector form) of the self-interaction on 4+4 is built: $\gamma_{44q}^A=-i\,\gamma_{1644}^A$ for $A=0,\dots,4$ and $\gamma_{44q}^A=\gamma_{1644}^A$ for $A=5,6,7$; they have the squares of the 7+1 matrices, and $\gamma_{44q}^4=\beta_{1644}$. $\Theta_Q^K=\zeta_r^Q\,\gamma_{44q}^{A_1}\cdots\gamma_{44q}^{A_r}$ with $\zeta_r^Q=i$ for $r=2,3,6,7$ and $\zeta_r^Q=1$ for the other ranks; $(\Theta_Q^K)^2=\mathrm{sgn}_K^Q$, the product of the squares of the matrices $\gamma_{44q}^A$ in $K$",
     "new"),
    (G11, r"$I_r$; $I_r^{\rm C}$, $I_r^{\rm Q}$", "",
     r"the invariant of rank $r$ (the author's symbol): $I_r=\sum_{\lvert K\rvert=r}(\psi^\dagger\,\mathrm{Adj}\,\Theta_K\psi)(\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi)$ with the adjoint matrix $\mathrm{Adj}$ of the model. On 7+1: $I_r=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K(\bar\psi\,\Theta^K\psi)^2$. On 4+4 there are two forms: $I_r^{\rm C}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K(\tilde\psi\,\Theta^K\psi)^2$ (form C, the definition) and $I_r^{\rm Q}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K^Q(\bar\psi\,\Theta_Q^K\psi)^2$ (form Q, the alternative); exactly, $I_r^{\rm C}=\sum_{\lvert K\rvert=r}\mathrm{sgn}_K^Q(\psi^\dagger W_{1644}\,\beta\,\Theta_Q^K\psi)^2$. $I_r$ without superscript is the invariant of the model at hand; in a statement that holds in both forms of 4+4, of either form. $I_0$ is the square of the scalar density, $I_1$ the square of the current",
     "K1; the two forms on 4+4 are new"),
    (G11, r"$\mathsf F$, $\mathsf F_{rr'}$", "fierz F_r0 F_r1 F_r2 F_r3 F_r4 F_r5 F_r6 F_r7 F_r8",
     r"the Fierz matrix ($9\times9$): $\mathsf F_{rr'}=\tfrac1{16}\sum_l(-1)^{rr'-l}\binom{r'}{l}\binom{8-r'}{r-l}$, with $\mathsf F^2=1$. For anticommuting fields the nine invariants obey $(1+\mathsf F)\,I=0$. (Sans-serif type; $F_S$, $F_P$, $F_0$, $F_1$, $F_7$, $F_8$ in italic type are pairing amplitudes. The author's document writes $F_{rs}$.)",
     "K1"),
    (G11, r"$g_0^{\rm eff}$, $g_1^{\rm eff}$, $g_7^{\rm eff}$, $g_8^{\rm eff}$", "g0eff g1eff g7eff g8eff g_0eff g_1eff g_7eff g_8eff",
     r"the four independent couplings that remain for anticommuting fields, when $I_2,\dots,I_6$ are expressed by $I_0$, $I_1$, $I_7$, $I_8$: $g_0^{\rm eff}=g_0-3g_2-4g_3-5g_4-4g_5-g_6$, $g_1^{\rm eff}=g_1-g_2-g_5+g_6$, $g_7^{\rm eff}=g_7-g_2-g_3+g_6$, $g_8^{\rm eff}=g_8-g_2+4g_3-5g_4+4g_5-3g_6$; then $\mathcal L_{\rm self}=-\tfrac12\big(g_0^{\rm eff}I_0+g_1^{\rm eff}I_1+g_7^{\rm eff}I_7+g_8^{\rm eff}I_8\big)$",
     "new"),
    (G11, r"$\mathsf n$", "",
     r"the one-body density matrix of matter ($16\times16$), $\mathsf n_{ab}=\langle\psi_b^\dagger\psi_a\rangle$, with $\mathrm{Tr}\,\mathsf n=n$ and $\mathrm{Tr}(\beta\,\mathsf n)=n_S$; for uniform matter at rest $\mathsf n=(n+n_S\,\beta)/16$. (The author's document writes $R$ for the matrix $\mathsf n\,\beta$.)",
     "new"),
    (G11, r"$G_S^{\rm HF}$, $G_V^{\rm HF}$; $G_S^{\rm c}$, $G_V^{\rm c}$", "G_0 G_1 G_S0 G_V0 hf_rho hf_rhoS hf_rhoW hf_rhoSW",
     r"the four combinations of the couplings that the Hartree-Fock approximation gives for uniform matter: $G_S^{\rm HF}=(15g_0^{\rm eff}-8g_1^{\rm eff}-8g_7^{\rm eff}-g_8^{\rm eff})/16$, $G_V^{\rm HF}=(-g_0^{\rm eff}+22g_1^{\rm eff}-6g_7^{\rm eff}+g_8^{\rm eff})/16$, $G_S^{\rm c}=(-g_0^{\rm eff}-2g_1^{\rm eff}+2g_7^{\rm eff}+g_8^{\rm eff})/16$ and $G_V^{\rm c}=-(g_0^{\rm eff}+g_8^{\rm eff})/16$. On 7+1 the interaction energy density is $\tfrac12(G_V^{\rm HF}n^2+G_S^{\rm HF}n_S^2)$: there $G_S^{\rm HF}$, $G_V^{\rm HF}$ are the mean-field couplings $G_S$, $G_V$. On 4+4 it is $\tfrac12(G_V^{\rm tot}n^2+G_S^{\rm tot}n_S^2+G_V^{\rm dif}n_W^2+G_S^{\rm dif}n_{SW}^2)$ with the same four combinations: $G^{\rm tot}=G^{\rm c}$, $G^{\rm dif}=G^{\rm HF}$ in form C and $G^{\rm tot}=G^{\rm HF}$, $G^{\rm dif}=G^{\rm c}$ in form Q. (The superscript c is upright; $G_c$ with the subscript $c$ is the critical coupling of the vacuum. The author's document and the data files write $G_0$, $G_1$ for $G_S^{\rm HF}$, $G_V^{\rm HF}$; the data files write `G_S0`, `G_V0` for $G_S^{\rm c}$, $G_V^{\rm c}$.)",
     "K1; the combinations of 4+4 are new"),
    (G11, r"$G_{\rm pair}$", "G_pair",
     r"the contact coupling of a pair of two particles at rest: the interaction energy of the pair times the volume; attractive if negative. For the seven vector pairs on 7+1 and in form Q of 4+4: $G_{\rm pair}=g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff}$. In form C of 4+4: $-(g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff})$ for the four vector pairs along $x0,x1,x2,x3$ (one particle of each half of the field) and $+(g_0^{\rm eff}+g_1^{\rm eff}+5\,g_7^{\rm eff})$ for the three vector pairs along $x5,x6,x7$ (two particles of the same half)",
     "new"),
    (G11, r"$n_+$, $n_-$, $n_{S+}$, $n_{S-}$; $n_W$, $n_{SW}$; $\mathrm{pol}_W$", "rho_W rho_SW pol_W pol",
     r"number densities $n_\pm=\langle\psi_\pm^\dagger\psi_\pm\rangle$ and scalar densities $n_{S\pm}=\langle\psi_\pm^\dagger\beta\,\psi_\pm\rangle$ of the particles in the two halves $W_{1644}=\pm1$ of the 4+4 field; $n=n_++n_-$, $n_S=n_{S+}+n_{S-}$, $n_W=n_+-n_-$, $n_{SW}=n_{S+}-n_{S-}$; the polarization $\mathrm{pol}_W=(n_+-n_-)/n$ (columns `pol_W`, `pol`)",
     "new"),
    (G11, r"$\mathrm{Adj}$", "",
     r"the adjoint matrix of a form of the self-interaction: its bilinears are $\psi^\dagger\,\mathrm{Adj}\,\Theta^K\psi$. $\mathrm{Adj}=\beta$ on 7+1 and in form Q of 4+4, where $\psi^\dagger\,\mathrm{Adj}=\bar\psi$; $\mathrm{Adj}=C^-_{1644}$ in form C of 4+4, where $\psi^\dagger\,\mathrm{Adj}=\tilde\psi$. (Upright letters; used where a statement holds for every form.)",
     "new"),
    (G11, r"$\psi_+$, $\psi_-$", "",
     r"the two halves of the 4+4 field: $\psi=\psi_++\psi_-$ with $W_{1644}\,\psi_\pm=\pm\psi_\pm$, eight components each. The Hamiltonian of the quantized sector acts inside each half, and a particle has four internal states in each",
     "new"),
    (G11, r"$\mathcal L_{\rm cov}$", "",
     r"the covariant Lagrangian of the free field on 4+4, $\mathcal L_{\rm cov}=\tilde\psi\,(\gamma^A\partial_A-m)\,\psi$. For fields of the quantized sector $\mathcal L_{\rm cov}=-\psi^\dagger W_{1644}\,(i\partial_4-\mathsf h)\,\psi$: it is $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}$ on the half $W_{1644}=-1$ and minus that on the half $W_{1644}=+1$",
     "new"),
    (G11, r"$z_K$", "",
     r"the phase in the exact relation between the matrices of the two forms on 4+4, $C^-_{1644}\,\Theta^K=z_K\,W_{1644}\,\beta\,\Theta_Q^K$: $z_K=\pm1$ if the index set $K$ contains an even number of the indices 5, 6, 7, and $z_K=\pm i$ if it contains an odd number",
     "new"),
    (G11, r"$\mathcal J_K^{++}$, $\mathcal J_K^{--}$, $\mathcal J_K^{+-}$, $\mathcal J_K^{-+}$", "",
     r"the bilinears of the two halves of the 4+4 field for the index set $K$: $\mathcal J_K^{++}=\psi_+^\dagger\beta\,\Theta_Q^K\psi_+$, $\mathcal J_K^{--}=\psi_-^\dagger\beta\,\Theta_Q^K\psi_-$, $\mathcal J_K^{+-}=\psi_+^\dagger\beta\,\Theta_Q^K\psi_-$ and $\mathcal J_K^{-+}=\psi_-^\dagger\beta\,\Theta_Q^K\psi_+=(\mathcal J_K^{+-})^\dagger$. If $K$ contains an even number of the indices 5, 6, 7 only the first two differ from zero, otherwise only the last two. (Script letter with an index set and two signs; $J_{VS}^A$, $J_5^A$ are currents and $J$ is an index set.)",
     "new"),
    (G11, r"$G_S^{\rm tot}$, $G_V^{\rm tot}$; $G_S^{\rm dif}$, $G_V^{\rm dif}$", "G_Stot G_Vtot G_Sdif G_Vdif",
     r"the mean-field couplings of the two halves of the 4+4 field: the interaction energy density of uniform matter is $\tfrac12\big(G_V^{\rm tot}n^2+G_S^{\rm tot}n_S^2+G_V^{\rm dif}n_W^2+G_S^{\rm dif}n_{SW}^2\big)$. $G^{\rm tot}$ multiply the squares of the total densities, $G^{\rm dif}$ the squares of the differences between the densities of the two halves. Form C: $G_S^{\rm tot}=G_S^{\rm c}$, $G_V^{\rm tot}=G_V^{\rm c}$, $G_S^{\rm dif}=G_S^{\rm HF}$, $G_V^{\rm dif}=G_V^{\rm HF}$. Form Q: $G_S^{\rm tot}=G_S^{\rm HF}$, $G_V^{\rm tot}=G_V^{\rm HF}$, $G_S^{\rm dif}=G_S^{\rm c}$, $G_V^{\rm dif}=G_V^{\rm c}$. (In the data files and in the statements of the programs: `G_Stot`, `G_Vtot`, `G_Sdif`, `G_Vdif`.)",
     "new"),
    (G11, r"$G_S^{\rm same}$, $G_V^{\rm same}$; $G_S^{\rm opp}$, $G_V^{\rm opp}$", "G_Ssame G_Vsame G_Sopp G_Vopp",
     r"the mean-field couplings between two particles of the same half and between particles of opposite halves of the 4+4 field: $G^{\rm same}=G^{\rm tot}+G^{\rm dif}$ and $G^{\rm opp}=G^{\rm tot}-G^{\rm dif}$, so that the interaction energy density is $\tfrac12G_V^{\rm same}(n_+^2+n_-^2)+G_V^{\rm opp}\,n_+n_-+\tfrac12G_S^{\rm same}(n_{S+}^2+n_{S-}^2)+G_S^{\rm opp}\,n_{S+}n_{S-}$. $G^{\rm same}=G^{\rm HF}+G^{\rm c}$ is the same in both forms: $G_S^{\rm same}=(7g_0^{\rm eff}-5g_1^{\rm eff}-3g_7^{\rm eff})/8$ and $G_V^{\rm same}=(-g_0^{\rm eff}+11g_1^{\rm eff}-3g_7^{\rm eff})/8$. $G^{\rm opp}$ has opposite signs in the two forms: $G^{\rm opp}=G^{\rm HF}-G^{\rm c}$ in form Q and $G^{\rm opp}=-(G^{\rm HF}-G^{\rm c})$ in form C, with $G_S^{\rm HF}-G_S^{\rm c}=(8g_0^{\rm eff}-3g_1^{\rm eff}-5g_7^{\rm eff}-g_8^{\rm eff})/8$ and $G_V^{\rm HF}-G_V^{\rm c}=(11g_1^{\rm eff}-3g_7^{\rm eff}+g_8^{\rm eff})/8$. (In the data files and in the statements of the programs: `G_Ssame`, `G_Vsame`, `G_Sopp`, `G_Vopp`.)",
     "new"),
    (G11, r"$g$ (strength)", "",
     r"the common strength of the couplings of the Lagrangian in the sets A and B of the numerical examples on 4+4: $g=\lvert g_0\rvert>0$, of dimension length$^{D-1}$ (column `g` of the files `forms_..`, in units $m=1$; in the files of the gaps the column `g` is the coupling $g_{VS}$). On the deflating spacetime $g$ grows as $e^{\tau_d}$",
     "new"),
    # ---- 12 ----
    (G12, r"$G_S$, $G_V$", "G_S G_V GS_abs",
     r"the scalar and the vector coupling of the Kohn-Sham (mean-field) description of uniform matter (the author's symbols): interaction energy density $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$; attraction for $G_S<0$, repulsion for $G_V>0$; dimension length$^{D-1}$. The author's example: $G_S=-10^5$, $G_V=+10^4$ in units $m=1$, $D=7$",
     "K1"),
    (G12, r"$n$, $n_S$", "rho rho_S rho_ks rho_cjt n_ks n_cjt",
     r"number density $\langle\psi^\dagger\psi\rangle$ and scalar density $\langle\bar\psi\psi\rangle$ of the fermions of matter (the filled states of negative energy not counted). The data files and the author's document write $\rho$ (`rho`) and $\rho_S$ (`rho_S`); the files of the two halves of the 4+4 field write `n`",
     "K1 (there $\\rho$, $\\rho_S$)"),
    (G12, r"$D$", "D",
     r"the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function)",
     "new"),
    (G12, r"$E^*(k)$; $\langle f\rangle$", "",
     r"$E^*(k)=\sqrt{k^2+m^{*2}}$, the energy of a particle of matter without the vector term; $\langle f\rangle=\nu\int d^Dk\,f/(2\pi)^D$ over the occupied states: $n=\langle1\rangle$, $n_S=\langle m^*/E^*\rangle$. For the two halves of the 4+4 field: $E^*_\pm(k)=\sqrt{k^2+m_\pm^{*2}}$",
     "K1 (there $\\omega$ for $E^*$)"),
    (G12, r"$\varepsilon_{\rm kin}$; $p_j$; $p_{\rm int}$", "e_kin eps eps_ks eps_cjt eps_rel p p_ks p_flat p5 p6 p7 p_a p_b pa_rel pb_rel Pi",
     r"$\varepsilon_{\rm kin}=\langle E^*\rangle$: the kinetic part of the energy density $\varepsilon=\varepsilon_{\rm kin}+\tfrac12G_Vn^2-(m^*-m)^2/(2G_S)$; $p_j$: the pressure of matter along the coordinate $xj$, $\langle k_j^2/E^*\rangle+p_{\rm int}$; $p_{\rm int}=\tfrac12G_Vn^2+(m^*-m)^2/(2G_S)$: the interaction part of the pressure, which is the whole pressure along a coordinate on which the fields do not depend (`Pi` in the data files, $\Pi$ in the author's document)",
     "K1"),
    (G12, r"$V_0$", "V_0",
     r"the vector mean field $V_0=G_V\,n$ (the author's symbol); it shifts the chemical potential, $\mu\to\mu-V_0$. For the two halves of the 4+4 field: $V_{0\pm}=G_V^{\rm tot}\,n\pm G_V^{\rm dif}\,n_W$",
     "K1"),
    (G12, r"$\Omega_{\rm CJT}$; $\Omega_{\rm half}$", "minus_Omega",
     r"the CJT effective potential of uniform matter with contact interaction per unit volume, a function of the mass $m^*$ and of the vector field $V_0$ at the chemical potential $\mu$: $\Omega_{\rm CJT}=-\nu\int\frac{d^Dk}{(2\pi)^D}\,(\mu-V_0-E^*)\,\theta(\mu-V_0-E^*)-\frac{(m^*-m)^2}{2G_S}-\frac{V_0^2}{2G_V}$; at its stationary point $-\Omega_{\rm CJT}$ is the pressure. For the two halves of the 4+4 field it is a function of two masses and two vector fields: $\Omega_{\rm CJT}=\Omega_{\rm half}(m^*_+,\mu-V_{0+})+\Omega_{\rm half}(m^*_-,\mu-V_{0-})$ $-[\tfrac12(m^*_++m^*_-)-m]^2/(2G_S^{\rm tot})$ $-[\tfrac12(m^*_+-m^*_-)]^2/(2G_S^{\rm dif})$ $-[\tfrac12(V_{0+}+V_{0-})]^2/(2G_V^{\rm tot})$ $-[\tfrac12(V_{0+}-V_{0-})]^2/(2G_V^{\rm dif})$, with the grand potential of free particles of one half (four internal states), $\Omega_{\rm half}(m^*,\mu)=-4\int\frac{d^Dk}{(2\pi)^D}\,(\mu-E^*)\,\theta(\mu-E^*)$",
     "new"),
    (G12, r"$G_c$", "Gc",
     r"the critical coupling of the vacuum: with the filled states of negative energy below the cutoff $\Lambda$ a massless field acquires a mass when $G_S<G_c=-(2\pi)^D(D-1)/(\nu A_D\Lambda^{D-1})$; $G_c\Lambda^6=-90\pi^4$ for $D=7$",
     "K2 ($D=7$)"),
    (G12, r"$m_{\rm vac}$, $m_{\rm bare}$; $\langle\bar\psi\psi\rangle_{\rm vac}$", "M_vac m_bare condensate",
     r"with the filled states of negative energy: the mass of the fermions in vacuum, the mass parameter of the Lagrangian (bare mass) that produces it, and the scalar density of the filled states, $\langle\bar\psi\psi\rangle_{\rm vac}=-\nu\int^\Lambda\frac{d^Dk}{(2\pi)^D}\frac{m_{\rm vac}}{\sqrt{k^2+m_{\rm vac}^2}}$ (negative); $m_{\rm vac}=m_{\rm bare}+G_S\langle\bar\psi\psi\rangle_{\rm vac}$",
     "new"),
    (G12, r"$n_{S+}^{\rm vac}$, $n_{S-}^{\rm vac}$; $\Omega_{\rm vac}$", "Omega",
     r"for the two halves of the 4+4 field with the filled states of negative energy below the cutoff $\Lambda$: the scalar densities of the filled states, $n_{S\pm}^{\rm vac}=-4\int_{\lvert\vec k\rvert<\Lambda}\frac{d^Dk}{(2\pi)^D}\,\frac{m^*_\pm}{E^*_\pm}$, which enter the gap equations of the vacuum $\tfrac12(m^*_++m^*_-)-m=G_S^{\rm tot}(n_{S+}^{\rm vac}+n_{S-}^{\rm vac})$, $\tfrac12(m^*_+-m^*_-)=G_S^{\rm dif}(n_{S+}^{\rm vac}-n_{S-}^{\rm vac})$; and the energy density of the vacuum relative to the state in which both halves have the mass $m$ of the Lagrangian, $\Omega_{\rm vac}=-4\sum_\pm\int_{\lvert\vec k\rvert<\Lambda}\frac{d^Dk}{(2\pi)^D}\,\big[E^*_\pm-\sqrt{k^2+m^2}\big]$ $-[\tfrac12(m^*_++m^*_-)-m]^2/(2G_S^{\rm tot})$ $-[\tfrac12(m^*_+-m^*_-)]^2/(2G_S^{\rm dif})$; among several solutions of the gap equations the one of lowest $\Omega_{\rm vac}$ is the vacuum (column `Omega`)",
     "new"),
    (G12, r"$g_{\rm crit}$; $\Lambda_{\max}$", "g_crit cutoff_max",
     r"$g_{\rm crit}$: the critical strength of a set of couplings, the strength $g$ at which the more attractive of the scalar couplings $G_S^{\rm tot}$, $G_S^{\rm dif}$ reaches the critical coupling $G_c$ of the vacuum of a massless field. $\Lambda_{\max}$: at a given strength, the largest cutoff $\Lambda$ for which that coupling is still weaker than $G_c$, so that the vacuum of the massless field is stable (column `cutoff_max`, in units of $m$)",
     "new"),
    (G12, r"$k_{F+}$, $k_{F-}$, $k_{F,\rm tot}$; $\mu_+$, $\mu_-$", "kF_total mu_plus mu_minus",
     r"for the two halves of the 4+4 field: the Fermi momenta of the two halves, $n_\pm=4V_Dk_{F\pm}^D/(2\pi)^D$ (four internal states each); the Fermi momentum $k_{F,\rm tot}$ that the total density would have with equal halves, $n=8V_Dk_{F,\rm tot}^D/(2\pi)^D$; and the chemical potentials $\mu_\pm=\sqrt{k_{F\pm}^2+m_\pm^{*2}}+V_{0\pm}$ of the two halves (for an empty half: the energy of its lowest state)",
     "new"),
    (G12, r"$\mathsf R_{\rm gap}$", "response",
     r"the response matrix of the two gap equations $m^*_\pm=m+G_S^{\rm same}n_{S\pm}+G_S^{\rm opp}n_{S\mp}$ of the halves of the 4+4 field: the $2\times2$ matrix with the elements $(\mathsf R_{\rm gap})_{++}=G_S^{\rm same}\,\partial n_{S+}/\partial m^*_+$, $(\mathsf R_{\rm gap})_{+-}=G_S^{\rm opp}\,\partial n_{S-}/\partial m^*_-$, $(\mathsf R_{\rm gap})_{-+}=G_S^{\rm opp}\,\partial n_{S+}/\partial m^*_+$, $(\mathsf R_{\rm gap})_{--}=G_S^{\rm same}\,\partial n_{S-}/\partial m^*_-$ (derivatives at fixed Fermi momenta). While its eigenvalues are below 1 the solution is the regular continuation of the dilute gas; at the eigenvalue 1 solutions with other masses branch off. Column `response`: the largest eigenvalue. (Sans-serif with the suffix gap; $R$ in italic type is a curvature.)",
     "new"),
    (G12, r"$-g_0/C$, $g_1/C$", "kappa_0 kappa_1",
     r"the couplings $g_0$ and $g_1$ of the Lagrangian in units of the coupling $C$ of the Vorton-Scalar (units $m=1$), in the calculations with bare mass, self-interaction and Vorton-Scalar together on 4+4; they are written as these ratios, without a letter of their own (columns `kappa_0`, `kappa_1`)",
     "new"),
    (G12, r"$m_D$, $m_Y$, $f_D$", "f_D",
     r"when a bare Dirac mass and the Vorton-Scalar are both present: the bare Dirac mass $m_D$, the part $m_Y=g_{VS}v_{VS}$ of the mass that the Vorton-Scalar gives, and the fraction $f_D=m_D/m$ of the vacuum mass $m=m_D+m_Y$",
     "new"),
    (G12, r"$m_\vartheta$", "mph2_over_C_ms2",
     r"the mass of the phase mode $\vartheta_{VS}$ of the Vorton-Scalar: zero while the chiral symmetry $U(1)_{VS}$ is exact; a bare Dirac mass breaks the symmetry and gives $m_\vartheta^2v_{VS}^2=\big(\lvert\langle\bar\psi\psi\rangle_{\rm vac}\rvert-n_S\big)m_Dm_Y/m$",
     "new"),
    (G12, r"$-G_S/C$, $G_V/C$", "kappa_S kappa_V",
     r"the contact couplings in units of the coupling $C=g_{VS}^2m^2/m_s^2$ of the Vorton-Scalar (units $m=1$); they are written as these ratios, without a letter of their own (columns `kappa_S`, `kappa_V`)",
     "new"),
    # ---- 13 ----
    (G13, r"$F_0$, $F_1$, $F_7$, $F_8$", "F0 F1 F7 F8 F0c F1c F7c F8c",
     r"the pairing amplitudes of the four independent contact channels in the pair channel along the hidden axis, for particles of mass $m^*$ that scatter from the momenta $\vec p,-\vec p$ to $\vec p',-\vec p'$ in ordinary space ($E=\sqrt{m^{*2}+p^2}$, $E'$ likewise): $F_0=(EE'+m^{*2}-\vec p\cdot\vec p')/(2EE')$, $F_1=(3EE'-2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_7=(3EE'+2m^{*2}-\vec p\cdot\vec p')/(EE')$, $F_8=-(EE'-m^{*2}-\vec p\cdot\vec p')/(2EE')$. $F_0=F_S$ and $F_8=-F_P$. The same closed forms hold in the pair channels along $x5,x6,x7$; the sign with which they enter the pairing potential is $s_{\rm ct}$",
     "new"),
    (G13, r"$m_{\rm ct}$", "m_ct",
     r"the range parameter of the contact couplings in the gap equation: each effective coupling $g_r^{\rm eff}$ is replaced by $g_r^{\rm eff}\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, the form that the exchange of a boson of mass $m_{\rm ct}$ gives ($q$: momentum transfer)",
     "new"),
    (G13, r"$V_{\rm ct}$, $V_{VS}$", "",
     r"the pairing potentials in the pair channel along the hidden axis and in those along $x5,x6,x7$: of the contact interaction, $V_{\rm ct}=s_{\rm ct}\sum_{r=0,1,7,8}g_r^{\rm eff}F_r\,m_{\rm ct}^2/(q^2+m_{\rm ct}^2)$, and of the exchange of the two modes of the Vorton-Scalar, $V_{VS}=-g_{VS}^2\big[F_S/(q^2+m_s^2)+F_P/(q^2+m_\vartheta^2)\big]$; attractive where negative",
     "new"),
    (G13, r"$s_{\rm ct}$", "ct_sign",
     r"the sign with which the contact couplings enter the pairing potential $V_{\rm ct}$ of a vector pair whose axis is perpendicular to ordinary space: $s_{\rm ct}=+1$ on 7+1, in form Q of 4+4 (the pair along the hidden axis $x0$ and the pairs along $x5,x6,x7$) and in form C for the pairs along $x5,x6,x7$; $s_{\rm ct}=-1$ in form C for the pair along the hidden axis (column `ct_sign`)",
     "new"),
    (G13, r"$k_{\max}$", "K_MAX K_MAX_EARLIER",
     r"the upper limit of the momentum integral of the gap equation. In the calculations of this folder $k_{\max}=2\times10^4\max(m,k_F)$ (constant `K_MAX` of the program); the earlier calculations of the project used $60\max(m,k_F)$ (`K_MAX_EARLIER`)",
     "new"),
    (G13, r"$w_\infty(t)$; $s_\Delta$; $U_{\rm uv}$; $u_S$, $u_V$", "uv UV_I_S UV_I_V UV_CRITICAL",
     r"the quantities of the ultraviolet criterion of the gap equation. $w_\infty(t)$: the kernel at momenta far above all masses, where $W(k,k')\to w_\infty(k'/k)/k^2$. $s_\Delta$: the exponent of a power-law solution $\Delta(k)\propto k^{-s_\Delta}$ at large momenta, a root of $\int_0^\infty dt\,t^{1-s_\Delta}\,w_\infty(t)=4\pi^2$. $U_{\rm uv}=\int_0^\infty dt\,w_\infty(t)$: the ultraviolet strength of the kernel; the gap equation has a solution that does not depend on $k_{\max}$ if and only if $U_{\rm uv}<4\pi^2$. With both modes of the Vorton-Scalar, $U_{\rm uv}=2u_S\,g_{VS}^2-s_{\rm ct}\,m_{\rm ct}^2\big[(g_0^{\rm eff}-g_8^{\rm eff})\,u_S+(g_1^{\rm eff}+g_7^{\rm eff})\,u_V\big]$, with the two integrals $u_S=\pi^2/8-\tfrac12$ and $u_V=3\pi^2/4-1$ (constants `UV_I_S`, `UV_I_V`; `UV_CRITICAL` is $4\pi^2$). Column `uv`: $U_{\rm uv}/4\pi^2$",
     "new"),
    # ---- 14 ----
    (G14, r"$E^{(k)\mu}{}_\nu$", "",
     r"the Lovelock tensor of order $k=1,2,3$ in the author's notation and sign: $E^{(1)}=-\mathcal G$ (minus the Einstein tensor), $E^{(2)}=\mathcal H$ (the Lanczos tensor). For a metric whose scale factors depend only on $x4$: $E^{(k)m}{}_m=(-1)^{k+1}k!\,2^{k-1}$ times the sum, over the sets of $2k$ frame directions other than $m$ and over their divisions into pairs, of the products of the sectional curvatures $K_{AB}$ of the pairs. (With the order in parentheses as a superscript; $E_F$, $E_k$, $E_\perp$, $E^*$ are energies.)",
     "K1"),
    (G14, r"$\alpha_1$, $\alpha_2$, $\alpha_3$; $\bar\Lambda$; $\kappa_8$", "LambdaBar kappa kappa_eps",
     r"the couplings of the Lovelock terms ($\alpha_1=1$), the cosmological term and the gravitational coupling of the author's Einstein-Lovelock equations: $\sum_k\alpha_kE^{(k)4}{}_4-\bar\Lambda=\kappa_8\,\varepsilon$ for the time component and $\sum_k\alpha_kE^{(k)m}{}_m-\bar\Lambda=-\kappa_8\,p_m$ for the space direction $m$. $\alpha_2=-\alpha_{GB}$ and $\bar\Lambda=\Lambda_8$. The author's document writes $\kappa$ for $\kappa_8=8\pi G_8$ (the letter $\kappa$ with an argument, $\kappa(k)$, is a pair correlation). A Lovelock coupling carries a digit as subscript; $\alpha^j$ with a superscript are Dirac matrices",
     "K1"),
    (G14, r"$\mathsf a$, $\mathsf b$; $H_{\mathsf a}$, $H_{\mathsf b}$; $n_{\mathsf a}$, $n_{\mathsf b}$", "a b H_a H_b Ha_over_g Hb_over_g Hb0_over_g a_end b_end Ha_end_over_g Hb_end_over_g",
     r"the two scale factors of a spacetime whose space coordinates form two groups of $n_{\mathsf a}$ and $n_{\mathsf b}$ coordinates, and their rates $H_{\mathsf a}=\mathsf a'/\mathsf a$, $H_{\mathsf b}=\mathsf b'/\mathsf b$ (a prime: derivative with respect to the time $x4$). In the author's document: $\mathsf a$ for ordinary space ($n_{\mathsf a}=3$) and $\mathsf b$ for the four auxiliary coordinates. On the deflating spacetimes with equal rates: $\mathsf a$ for $x0,\dots,x3$ ($n_{\mathsf a}=4$) and $\mathsf b=b_5=b_6=b_7$ ($n_{\mathsf b}=3$); the undisturbed deflation is $H_{\mathsf a}=0$, $H_{\mathsf b}=-\mathrm{g55}$",
     "K1 (there $a$, $b$, $H_a$, $H_b$)"),
    (G14, r"loading", "loading",
     r"$\kappa_8\,\varepsilon(0)/(3\,\mathrm{g55}^2)$: the energy density of the matter at the start of an evolution, in units of the energy density that would make three coordinates deflate at the rate $\mathrm{g55}$ in Einstein gravity",
     "new"),
    (G14, r"$T^\mu{}_\nu$; $p_{\mathsf a}$, $p_{\mathsf b}$; $\delta H_{\mathsf b}$", "",
     r"the energy-momentum tensor of uniform matter in mixed components, $T^4{}_4=\varepsilon$, $T^m{}_m=-p_m$; its pressures along the coordinates that carry the scale factors $\mathsf a$ and $\mathsf b$; the deviation $H_{\mathsf b}+\mathrm{g55}$ of the rate of the deflating coordinates from the undisturbed deflation",
     "new"),
    (G12, r"$\mathsf h_{\rm KS}$", "",
     r"the Hamiltonian matrix of a particle of matter in the Kohn-Sham description, $\mathsf h_{\rm KS}=\sum_j\alpha^jk_j+\beta\,m^*+V_0$",
     "K1"),
    (G12, r"$\Gamma[S]$; $S$, $S_0$", "",
     r"the CJT effective action as a functional of the propagator of the fermions, $\Gamma[S]=-i\,\mathrm{Tr}\ln S^{-1}-i\,\mathrm{Tr}(S_0^{-1}S-1)+\Gamma_2[S]$, with the full propagator $S$ and the propagator $S_0$ of free particles of the mass $m$ (the argument in brackets tells it from the action $\Gamma[\phi,G]$ of the model with the Vorton-Scalar)",
     "new"),
    (G12, r"$n_{\rm ref}$", "",
     r"the number density of matter at the reference time $x4_{\rm ref}$; matter that keeps its particles has $n=n_{\rm ref}\,e^{\tau_d}$",
     "new"),
    (G11, r"$\mathcal L_{\rm kin}$, $\mathcal L_{\rm mass}$", "",
     r"the kinetic term and the mass term of the Lagrangian of the fermion field, $\mathcal L=V_d\,[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}]$. On 4+4, for fields of the quantized sector, $\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\psi^\dagger(i\partial_4-\mathsf h)\psi$ (the positive Lagrangian, with the mass term $-m\,\bar\psi\psi$)",
     "new"),
    (G13, r"$\Phi$, $\Phi(\vec p)$", "",
     r"a state of a pair of two particles, an antisymmetric $16\times16$ matrix; $\Phi(\vec p)$: the state of the pair along the hidden axis with the momenta $\vec p$, $-\vec p$, the matrix $(C^-\gamma^0)^*$ between the projectors on the states of positive energy. On 4+4: $W_{1644}\,\Phi\,W_{1644}^T=-\Phi$ for a pair of one particle of each half of the field and $+\Phi$ for a pair of two particles of the same half. (Without suffix; the Vorton-Scalar always carries the suffix VS.)",
     "new"),
    ("9 Cosmology", r"$\sigma_v$; $\kappa_{\rm SI}$", "",
     r"the velocity dispersion of the dark matter of a halo (with the index $v$; $\sigma_8$ is the amplitude of clustering and $\sigma(p)$ a self-energy); the bound on the cross-section per mass of dark matter from colliding clusters, about $1~\mathrm{cm^2/g}$ (with the index SI; $\kappa_8$ is the gravitational coupling)",
     "new"),
    # ---- 15 ----
    (G15, r"71s, 44q, 44c", "71s 44q 44c",
     r"tags of the three models of the self-interaction in the programs and the data files: 71s on 7+1; 44c form C on 4+4, the definition (bilinears with $\tilde\psi=\psi^\dagger C^-_{1644}$); 44q form Q on 4+4, the alternative (bilinears with $\bar\psi=\psi^\dagger\beta_{1644}$ and the matrices $\gamma_{44q}^A$)",
     "new"),
    (G15, r"form C, form Q", "",
     r"the two forms of the self-interaction on the 4+4 spacetime. Form C, the covariant form, is the definition of the 4+4 model (instruction of the author of November 13, 2026): the invariants $I_r^{\rm C}$, built with the adjoint $\tilde\psi$, which are invariant under all rotations and boosts of the 4+4 frame, together with the positive kinetic and mass terms. Form Q, the quantized-sector form, is the alternative: the invariants $I_r^{\rm Q}$, built with the adjoint $\bar\psi$ of the positive kinetic term. In the data files the column `form` holds C or Q, or 0 for C and 1 for Q. (Upright capitals; $C$ in italic type is the coupling of the Vorton-Scalar, and $C^\pm$ with a sign are charge-conjugation matrices.)",
     "new"),
    (G15, r"set A, set B", "",
     r"the two sets of couplings of the Lagrangian for which the numbers of the two forms on 4+4 are given, each with one strength $g$: set A, a scalar attraction alone, $g_0=-g$; set B, $g_0=-g$ and $g_1=+0.1\,g$ (the ratio of the author's example); all other couplings zero. In the data files of the two forms the column `set` holds A or B, or 0 for A and 1 for B (in the files of the gaps the column `set` numbers the pairs of values of $m_s$ and $k_F$)",
     "new"),
    (G15, r"identifiers of the machine checks", "",
     r"`44d-..`, `71d-..`, `def-..`: checks on the deflating 4+4 spacetime, on the deflating 7+1 spacetime, and of results that hold on both; `44-..`, `71-..`: checks of the flat spacetimes, executed again; `71s-S..`, `44q-S..`, `44c-S..`, `self-X..`: the algebra of the self-interaction; `ks-K..`: Kohn-Sham calculations; `cjt-J..`: CJT effective potential with contact interaction; `uni-U..`: contact interaction, bare mass and Vorton-Scalar together; `pc-P..`: pairing with contact couplings; `gr-G..`: gravity with Kohn-Sham matter; `fm-F..`: the two forms of the self-interaction on 4+4 (matter of two halves, vacuum, thresholds). A leading capital I (`Icks-..`, `Idef-..`, `I44d-..`, `I71d-..`) marks a check of the independent verification in Python",
     "new"),
]

ORIGIN_KEY = [
    ("N01", "the author's notebook `4+4.nb` (Dirac matrices of 4+4; split octonions)"),
    ("N02", "the author's notebook `7+1.nb` (octonions; Dirac matrices and Dirac equation of 7+1)"),
    ("N04", "the author's notebook `Dirac_Einstein-Lovelock_4+4_Nash.nb` (meaning of the coordinates)"),
    ("N05", "the author's notebook `Dirac_Nash_Identity.nb`"),
    ("N10", "the author's notebook `Pair_Creation_of_Universes_WaveFunctionOfUniverse-4+4-Einstein-Lovelock-Nash.nb`"),
    ("N12", "the author's notebook `WaveFunctionUniverse-4+4-Einstein-Lovelock.nb`"),
    ("D06", "P. L. Nash, \"Identities satisfied by the generators of the Dirac algebra\", J. Math. Phys. 25, 204 (1984)"),
    ("D07", "P. L. Nash, \"On the structure of the split octonion algebra\", Nuovo Cimento B 105, 31 (1990)"),
    ("D08", "P. L. Nash, \"Second gravity\", J. Math. Phys. 51, 042501 (2010)"),
    ("K1", "the author's document `KOHN_SHAM_GRAVITY` (folder 7+1; sections 15 and 16 of the notebook `7+1.nb`): nine-channel self-coupling, Fierz matrix, Kohn-Sham theory, Einstein-Lovelock evolution"),
    ("K2", "the author's document `DIRAC16COMPLEX` (folder 7+1): the critical coupling of the vacuum in seven space dimensions"),
    ("author", "a name that an instruction of the author fixes (the rates $\\mathrm{g55}$, $\\mathrm{g66}$, $\\mathrm{g77}$; the constants; the signs of the metric)"),
    ("new", "introduced by this project"),
]


def build():
    rows = []   # (group, symbol, names, meaning, origin)
    used = set()
    for src, path in (("M", os.path.join(MAIN, "provenance", "symbols.csv")),
                      ("D", os.path.join(MAIN, "Revision_Deflation", "provenance", "symbols_deflation.csv"))):
        for r in read_registry(path):
            group, sym, names, meaning = r[0], r[1], r[2], r[3]
            origin = r[5]
            key = (src, sym)
            if key in DROP:
                used.add(key)
                continue
            if key in REWORD:
                used.add(key)
                meaning, new_names = REWORD[key]
                if new_names is not None and new_names.startswith("="):
                    names = new_names[1:].strip()          # "=...": the names are replaced
                elif new_names:
                    names = (names + " " + new_names).strip()
            if src == "D":
                group = GROUP_NAMES[group]
                # origin of the first revision: author / new / main (an extension of a symbol of the flat-space part)
                origin = {"author": "author", "new": "new", "main": "new"}[origin]
            meaning = meaning.strip().rstrip(".")
            # the registry is self-contained: the phrases that pointed to the flat-space part of the project are replaced
            for old, new_ in ((r"the constant matrices $\gamma_{1644}^A$, $\gamma_{1671}^A$ of the main project", r"the constant matrices $\gamma_{1644}^A$, $\gamma_{1671}^A$ of the frame"),
                              (r"(covariant components of the wave vector $k_A$ of the main project)", r"(covariant components of the wave vector $k_A$)"),
                              (r"; $\theta$ with other indices and the scattering angle keep the meanings of the main project)", r"; $\theta_*$ is the angle of the sound horizon and $\theta(\cdot)$ the step function)"),
                              (r"`Y_He` in the data files, as in the main project)", r"`Y_He` in the data files)")):
                meaning = meaning.replace(old, new_)
            sym = sym.replace(r"$v_{\rm EW}$ (main project), $n_{\rm EW}$", r"$v_{\rm EW}$, $n_{\rm EW}$")
            # the symbols `g_1644d`, `g_1671d` of the retired sign convention
            if sym == r"$\gamma_{1644d}^\mu$, $\gamma_{1671d}^\mu$":
                meaning = meaning.replace("2g^{\\mu\\nu}", "2c_\\gamma g^{\\mu\\nu}")
            rows.append([group, sym, names, meaning, origin])
    missing = [k for k in list(DROP) + list(REWORD) if k not in used]
    if missing:
        sys.exit("entries of DROP or REWORD that do not exist in the sources: %r" % missing)
    for g, sym, names, meaning, origin in NEW:
        rows.append([g, sym, names, meaning, origin])
    # no definition may point to another document
    bad = [r[1] for r in rows if re.search(r"main registry|main project|\bRS\d\b|\bDMD\b|\bRP\d\b|supplement|\bS[1-4]\b(?! \$)|in the manuscripts|\bDM\b", r[3] + " " + r[1])]
    if bad:
        sys.exit("meanings that still point to another document: %r" % bad)
    # uniqueness of the symbol texts and of the names in programs
    seen_sym, seen_name = {}, {}
    for r in rows:
        if r[1] in seen_sym:
            sys.exit("symbol registered twice: %s" % r[1])
        seen_sym[r[1]] = 1
        for nm in r[2].split():
            if nm in seen_name:
                sys.exit("name in programs registered twice: %s (%s and %s)" % (nm, seen_name[nm], r[1]))
            seen_name[nm] = r[1]
    return rows


def group_key(g):
    m = re.match(r"(D?)(\d+)", g)
    return (1 if m.group(1) else (2 if int(m.group(2)) >= 10 else 0), int(m.group(2)))


def main():
    rows = build()
    groups = sorted({r[0] for r in rows}, key=group_key)
    os.makedirs(os.path.join(ROOT, "provenance", "generated"), exist_ok=True)
    with open(os.path.join(ROOT, "provenance", "symbols_master.csv"), "w", encoding="utf-8", newline="") as f:
        f.write("# PROJECT spacetimes: MASTER registry of the symbols of the project (the unique symbol definitions that Hard Rule 5 requires)\n")
        f.write("# written by: code/scripts/write_symbol_registry.py (edit the lists there); verified by: code/scripts/check_symbols.py\n")
        f.write("# columns: group; symbol as written in the documents (LaTeX); names in programs and data files (blank-separated); meaning; origin (N.., D.., K..: a notebook, paper or document of the author; author: a name fixed by an instruction of the author; new)\n")
        w = csv.writer(f)
        w.writerow(["group", "symbol", "names_in_files", "meaning", "origin"])
        for g in groups:
            for r in rows:
                if r[0] == g:
                    w.writerow(r)
    out = []
    n_names = sum(len(r[2].split()) for r in rows)
    out.append("<!-- BEGIN symbols_count -->\n%d<!-- END symbols_count -->".replace("%d<", "%d\n<") % len(rows))
    out.append("<!-- BEGIN symbols_names_count -->\n%d\n<!-- END symbols_names_count -->" % n_names)
    out.append("<!-- BEGIN symbols_groups_count -->\n%d\n<!-- END symbols_groups_count -->" % len(groups))
    body = []
    for g in groups:
        body.append("**%s**\n" % g)
        body.append("| symbol | names in programs and data files | meaning | origin |")
        body.append("|:------------|:---------|:------------------------------------------|:--------------|")
        for r in rows:
            if r[0] == g:
                names = " ".join("`%s`" % x for x in r[2].split()) or "—"
                body.append("| %s | %s | %s | %s |" % (r[1].replace("|", "\\|"), names, r[3].replace("|", "\\|"), r[4].replace("|", "\\|")))
        body.append("")
    out.append("<!-- BEGIN symbols_master_tables -->\n%s\n<!-- END symbols_master_tables -->" % "\n".join(body).rstrip())
    key = ["| code | source |", "|---|---|"] + ["| %s | %s |" % kv for kv in ORIGIN_KEY]
    out.append("<!-- BEGIN symbols_origin_key -->\n%s\n<!-- END symbols_origin_key -->" % "\n".join(key))
    with open(os.path.join(ROOT, "provenance", "generated", "symbols_master.md"), "w", encoding="utf-8") as f:
        f.write("\n\n".join(out) + "\n")
    print("wrote provenance/symbols_master.csv (%d entries in %d groups, %d names in programs) and provenance/generated/symbols_master.md" % (len(rows), len(groups), n_names))


if __name__ == "__main__":
    main()
