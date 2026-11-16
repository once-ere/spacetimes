<!-- OUTPUT: provenance/P1_symbols_U2.md -->
---
title: "The symbols of Revision_3: registry, collisions resolved, machine checks"
subtitle: "Provenance document `P1` of PROJECT spacetimes, Revision_3"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P1_symbols_U2.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

Hard Rule 5 of the author requires that no symbol of PROJECT spacetimes denote two objects, that a
MASTER provenance of the symbols exist, and that every document use it. This document is the
registry of the symbols of the folder `Revision_3`: it states how the symbols were chosen
(Section 2), lists every collision with a symbol of the rest of the project that was found and how
it was resolved (Section 3), describes the six machine checks and prints their complete output
(Section 4), and gives the complete table of the symbols (Section 5), which every document of the
folder also contains. The machine-readable registry is the file `provenance/symbols_r3.csv`,
written by `code/scripts/write_symbol_registry.py`.

# 2. How the symbols were chosen

1. **Taken symbols.** Thirteen entries are taken from the two master registries of the project
   (the registry of the main project and the MASTER registry of the revision on the deflating
   spacetimes): the coordinates $X=\{x0,\dots,x7\}$, the index letters, the number $D$ of space
   dimensions, the index sets $K$ and ranks $r$, and the matrices of the 7+1 spacetime
   ($\eta_{1671}$, $\gamma^A_{1671}$, $\Sigma^{AB}_{1671}$, $\omega_{1671}$, $\chi_{1671}$,
   $P^L_{1671}$, $P^R_{1671}$, $C^\pm_{1671}$, $\beta_{1671}$, $\alpha^j_{1671}$). Their meaning is
   copied from the master file, word for word, so that it cannot drift; where Revision_3 uses only
   part of an entry, a note is appended (for instance: only the index letters $A$ and $C$ are used,
   because $B_{U2A}$ is a gauge field here).
2. **New symbols carry the suffix U2**, the name of the new gauge group $U(2)_{U2}$, in their
   subscript: $\Delta_{U2}$, $\mu_{U2}$, $\Omega_{U2}$, $G^P_{U2}$, $m_{U2}$, $\sigma_{U2}$, $B_{U2A}$,
   and so on. A new symbol without the suffix is either a mark that no master registry uses
   (sans-serif matrices such as $\mathsf G_K$, $\mathsf E$, $\mathsf K$; the index $\iota$), or the
   real and imaginary parts $\Delta_1$, $\Delta_2$ of the pair field, or the imaginary unit $i$, the
   component indices $l$, $l'$ and the momentum $\vec q$, $q$, $q^j$, which no master registry
   defines with another meaning in the documents of the project that Revision_3 relies on.
3. **The electroweak theory**, quoted as the template, carries the suffix SM.
4. **Names in programs and data files.** The programs write symbols as plain names (for instance
   `Delta` for $\Delta_{U2}$, `mu` for $\mu_{U2}$); every column of every data file is either a
   registered name (last column of the table of Section 5) or a descriptive word. All data files of
   the folder begin with `r3_` and state their meaning in a header line `# meaning:`, so a name can
   only be read as the symbol of Revision_3.

# 3. The collisions that were found, and how each was resolved

The left column names the letter as a plain name (it is the symbol of another part of the project,
not of Revision_3); the right column gives what Revision_3 writes instead.

| letter or symbol in the rest of the project | its meaning there | what Revision_3 writes |
|---|---|---|
| `lambda`, `lambda_S`, `lambda_P` | the dimensionless pairing coupling and its parts from the radial and the phase mode | $\lambda^P_{U2}=G^P_{U2}/G^{\rm crit}_{U2}$, $\lambda^S_{U2}=4G^S_{U2}/G^{\rm crit}_{U2}$ |
| `G_S`, `G_V` | the couplings of the Kohn–Sham description of matter | $G^S_{U2}$, $G^P_{U2}$ |
| `Lambda`, `alpha_c` | a momentum cutoff of the ladder equation | $\Lambda_{U2}$ |
| `mu`, `k_F`, `E_F` | chemical potential, Fermi momentum and energy of matter | $\mu_{U2}$, $q^F_{U2}$ |
| `Delta(k)`, `Delta^a`, `Delta_1671^A` | gap function; vector order parameter; vector pair operators | $\Delta_{U2}$, $\Delta_1$, $\Delta_2$ |
| `Omega_mu` | the spin connection of the deflating spacetime | $\Omega_{U2}$, $\Omega^{\rm cl}_{U2}$, $\Omega^{\min}_{U2}$, $\Omega^{\rm sym}_{U2}$ |
| `Theta_K` (with a phase `zeta_r`) | the 256 products of Dirac matrices of the self-interaction | $\mathsf G_K$, the ordered products without a phase |
| `Phi`, `Phi(p)` | a state of a pair, an antisymmetric matrix | $\Phi^\pm_{U2}$, the pair operators |
| `tau`, `tau_d` | optical depth; deflation times | $\tau^1_{U2}$, $\tau^2_{U2}$, $\tau^3_{U2}$, the Pauli matrices |
| index `B`; `B` (bag constant) | an index with the values 0 to 7; an energy density | only $A$ and $C$ as indices; $B_{U2A}$ the $U(1)_{U2}$ gauge field |
| `b_5`, `b_6`, `b_7`; `a_s`, `a_sc` | scale factors; other constants | $a^{(7)}_{U2}$, $a^{(8)}_{U2}$ |
| `m` (with `beta`, `alpha^j`, `chi`, `h`) | the mass of the spacetime at hand | $m_{U2}$, the bare mass of the doublet |
| `M_1671`, `M_s`, `mathsf M` | matrices and masses of other parts | $M_{U2}$, the effective mass |
| `x` | the ratio of masses `m*/m` | only the coordinate names $x0,\dots,x7$, $x^A$, $x^j$ and $\vec x$ |
| `w` | a constant spinor of the exact solutions on the deflating spacetime | $w_{U2}$, a frequency; eigenvectors are $\mathsf e^\pm_{U2}$ |
| `s`, `t` | indices of the coordinates $x5$, $x6$, $x7$ | $s_{U2}=M_{U2}^2+\lvert\Delta_{U2}\rvert^2$; no `t` |
| `a`, `b` | indices of the four extra axes | not used; spinor components are indexed by $l$, $l'$ |
| `l` | the summation index of the closed form of the Fierz matrix | $l$, $l'$ index spinor components; the Fierz matrix does not occur in Revision_3 |
| `K` | an index set (taken) | an index set only; a matrix of a bilinear form is $\mathsf K$ |
| `S`, `S_0` | the propagator of the CJT effective action | "spin zero" is written in words |
| `i` | an index of space-like coordinates | the imaginary unit; space-like indices are $j$, $j'$ |
| `n`, `n_S` | number and scalar density of matter | $n_{U2}$ |
| `N` | the density of dark radiation | $N_{U2}$, the number operator |
| `E_k`, `xi_k` | quasi-particle energy, kinetic energy | $E^\pm_{U2}$, $\xi^\pm_{U2}$ |
| `epsilon(k_F,x)`, `epsilon_kin` | energy densities | $\varepsilon_{U2}$, the single-particle energy |
| `kappa(k)`, `kappa_8`, `kappa_SI` | a pair correlation; constants | $\kappa_{U2}$, $\kappa^{(4)}_{U2}$, the Euclidean momentum |
| `sigma(p)`, `sigma_v`, `sigma_8` | a self-energy; a velocity dispersion; a clustering amplitude | $\sigma_{U2}$, the amplitude mode |
| `theta_s` | an angle of cosmology | $\theta_{U2}$, the phase of the pair field |
| `f_+`, `f_D`, `f_g` | functions and constants | $f^s_{U2}$, $f^t_{U2}$, $f_{U2}$ |
| `g_r`, `g_VS` | couplings of the self-interaction and of the Vorton-Scalar | $g_{U2}$, $g'_{U2}$ |
| `y_VS` | a charge of the Vorton-Scalar | $y_{U2}$ |
| `z_*`, `z_t` | redshifts | $z_{U2}$ |
| `u_d` | a scaled wave number | $u^\pm_{U2}$, $v^\pm_{U2}$ |
| `Pi_1671` | a projector | $\Pi_{U2}$, the polarization |
| `R` | the curvature scalar | $\mathsf R_{U2}$, the matrix of small oscillations |
| `V_d`, `V_0`, `V_s(r)` | a volume; potentials | $V^{\rm C}_{U2}$, $V^{\rm pair}_{U2}$, $\mathsf V^1_{U2}$ to $\mathsf V^3_{U2}$, $\mathrm{Vol}_{U2}$ |
| `varrho_c` | an abbreviation of a ladder equation | $\varrho_{U2}$, a distance |
| `phi_1671^L`, `phi_bos` | chiral parts; a boson field | $\phi^{\rm Y}_{U2}$, $\phi_{\rm SM}$ |
| `psi_1671` | the free field dirac1671 | $\psi^1_{U2}$, $\psi^2_{U2}$, $\Psi_{U2}$ |
| `L`, `R` | chirality labels | kept as labels of $P^L_{1671}$, $P^R_{1671}$; $L_{U2}$ is a function of (R8) |
| `J` | an index set | $J^A_{U2}$, $\boldsymbol J^A_{U2}$ (currents), $\mathcal J^{(\iota)}_{U2}$ (integrals) |
| `c_s` | a sound speed | $c^\sigma_{U2}$ |
| `Q(M)` | a function of a mass | $\vec Q_{U2}$ |
| `Y_He` | the helium fraction | $Y_{\rm SM}$ |

# 4. The machine checks of the symbols

`code/scripts/check_symbols.py` verifies:

* **R3-S1**: no symbol text is registered twice, and no name used in a data file belongs to two
  entries;
* **R3-S2**: every taken entry carries exactly the meaning of the master registry, and no new symbol
  text is a symbol text of either master registry;
* **R3-S3**: every column of every data file is a registered name or a descriptive word, and every
  data file states its meaning;
* **R3-S4**: every document of the folder, and the manuscript, contains the complete table of the
  symbols;
* **R3-S5**: in every formula of the documents, $\lambda^P_{U2}$, $\lambda^S_{U2}$, $G^P_{U2}$,
  $G^S_{U2}$, $G^{\rm crit}_{U2}$, $\mu_{U2}$, $\Lambda_{U2}$, $\Omega_{U2}$ and $\Delta_{U2}$ carry their
  suffix;
* **R3-S6**: in every formula of the documents and of the new registry entries, no letter that a
  master registry defines with another meaning stands without the suffix of its Revision_3 meaning;
  the only letters allowed without a suffix are the taken indices and sets ($A$, $C$, $D$, $K$, $X$,
  $j$, $r$), the new plain symbols ($i$, $l$, $q$) and the differential $d$; the letter x occurs only in the
  coordinate names.

The complete output of the last run:

```
{{FILE:data/r3_symbol_checks.txt}}
```

# 5. The complete table of the symbols

"taken": a symbol of the master registries, used with the same meaning (the master meaning is
copied, followed by a note on its use in Revision_3). "new": introduced in Revision_3.

{{GEN:symbols_r3_table}}
