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
PROJECT spacetimes, Revision_3: checks of the symbols (registry provenance/symbols_r3.csv)
6 of 6 checks passed

[PASS] R3-S1  every symbol text is registered once (90 entries) and every name used in the data files of Revision_3 belongs to one entry (88 names)
[PASS] R3-S2  the 13 taken entries carry exactly the meaning of the MASTER registry (113 entries of the main project, 229 of the revision on the deflating spacetimes compared), and none of the 77 new symbols is a symbol of either master registry
[PASS] R3-S3  every column of the 18 data files data/r3_*.csv is a registered name or a descriptive word (88 different columns), and every file states its meaning; 9 column names also occur as names in a master registry or as symbols of the author's notebooks (A, M, m, mu, n, name, q, rank, x) - they cannot be confused, because every data file of Revision_3 has the prefix r3_ and a line '# meaning:', and the name means in it the registered symbol of Revision_3
[PASS] R3-S4  each of the 4 documents of Revision_3 (and the manuscript) contains the complete table of the 90 registered symbols, so that every symbol it uses is defined in the document itself
[PASS] R3-S5  in the 1554 formulas of the documents, lambda^P, lambda^S, G^P, G^S, G^crit, mu, Lambda, Omega and Delta always carry the suffix U2 (Delta also the index 1 or 2 of its real and imaginary parts)
[PASS] R3-S6  in the 1910 formulas of the documents and of the new registry entries, no letter that a master registry defines with another meaning stands bare: every letter except the taken indices and sets (A, C, D, K, X, j, r), the new plain symbols (i, l, q) and the differential d carries the suffix of its Revision_3 meaning, x occurs only in the coordinate names, and the Greek letters that are master symbols carry their suffix
```

# 5. The complete table of the symbols

"taken": a symbol of the master registries, used with the same meaning (the master meaning is
copied, followed by a note on its use in Revision_3). "new": introduced in Revision_3.

**1 Spacetime, coordinates and indices**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $X=\{x0,\dots,x7\}$ | the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1) | taken | `x0` `x1` `x2` `x3` `x4` `x5` `x6` `x7` |
| $A,B,C,D$ | index with the values $0,\dots,7$; always labels the coordinate $xA$. In Revision\_3 only the letters $A$ and $C$ are used as indices; $B_{U2A}$ is the $U(1)_{U2}$ gauge field. | taken |  |
| $i,j$ | index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime. In Revision\_3 only $j$ (and a second such index $j'$) is used as an index, and sums over $j$ run over all seven space-like coordinates $0,1,2,3,5,6,7$; the letter $i$ is the imaginary unit. | taken |  |
| $D$ | the number of space dimensions in which the particles of matter have momentum: 7 (all space coordinates of 7+1), 4 (fields that do not depend on $x5,x6,x7$) or 3 (fields that depend on ordinary space only). $V_D=\pi^{D/2}/\Gamma_{\rm E}(D/2+1)$ is the volume and $A_D=D\,V_D$ the area of the unit ball in $D$ dimensions ($\Gamma_{\rm E}$: Euler's gamma function). In Revision\_3 $D=7$ ($A_7=16\pi^3/15$), and $A_8=\pi^4/3$ is the area of the unit sphere of the Euclidean 8-momentum. | taken | `D` |
| $K$, $J$ (index sets); $r$, $r'$ (ranks) | $K$, $J$: a strictly increasing list $A_1<\dots<A_r$ of indices out of $0,\dots,7$ (there are 256); $r=\lvert K\rvert$ is its rank, $0\le r\le8$; $r'$ is a second rank and $l$ the summation index in the closed form of the Fierz matrix. In the data files an index set is given by its bit mask (`mask`: the sum of $2^A$ over its indices). In Revision\_3 only $K$ and $r$ are used, for the ordered products $\mathsf G_K$; the Fierz matrix does not occur. | taken | `mask` `rank` `J` |
| $\vec x$ | the seven space-like coordinates $(x0,x1,x2,x3,x5,x6,x7)$ as one vector; $\vec q\cdot\vec x=\sum_jq^jx^j$ | new |  |
| $l$, $l'$ | indices of the components of a spinor (values $1,\dots,16$) or of the doublet $\Psi_{U2}$ (values $1,\dots,32$); the summation index of the closed form of the Fierz matrix, which the master registry also calls $l$, does not occur in Revision\_3 | new |  |
| $\delta^{jj'}$ | the Kronecker symbol: 1 if $j=j'$, 0 otherwise | new |  |
| $\varrho_{U2}$ | the distance between two static sources in the seven-dimensional space | new |  |

**2 Matrices of the 7+1 spacetime**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\eta_{1671}$ | metric of the 7+1 spacetime, $\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$: time-like $=+1$, time at index 4 | taken | `eta1671` |
| $\gamma_{1671}^A$ | the eight real $16\times16$ Dirac matrices of the 7+1 spacetime (contravariant index) | taken | `gamma1671` |
| $\Sigma_{1671}^{AB}$ | $\tfrac14[\gamma_{1671}^A,\gamma_{1671}^B]$, the scaled commutators; they span $\mathfrak{so}(7,1)$ | taken | `Sigma1671` |
| $\omega_{1671}$ | the product $\gamma_{1671}^0\cdots\gamma_{1671}^7$; real, $\omega_{1671}^2=-1$ | taken | `omega1671` |
| $\chi_{1671}$ | $i\,\omega_{1671}$, the (complex) chirality matrix of 7+1 | taken | `chi1671` |
| $P_{1671}^{L}$, $P_{1671}^{R}$ | the complex chiral projectors $\tfrac12(1\mp\chi_{1671})$ | taken | `PL1671` `PR1671` |
| $C_{1671}^{+}$, $C_{1671}^{-}$ | $\gamma_{1671}^4$ and $\gamma_{1671}^4\omega_{1671}$; $C^{\pm}\gamma^A(C^{\pm})^{-1}=\pm(\gamma^A)^T$ | taken | `Cp1671` `Cm1671` |
| $\beta_{1671}$, $\alpha_{1671}^j$ | the real symmetric matrices $\gamma_{1671}^4$ and $\gamma_{1671}^4\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian | taken | `beta1671` `alpha1671` |
| $\mathsf G_K$ | the ordered product $\gamma^{A_1}_{1671}\gamma^{A_2}_{1671}\cdots\gamma^{A_r}_{1671}$ over an index set $K=(A_1<\dots<A_r)$ ($\mathsf G_K=\mathsf 1_{16}$ for the empty set); the 256 matrices $\mathsf G_K$ are a basis of the real $16\times16$ matrices | new |  |
| $\mathsf 1_2$, $\mathsf X$, $\mathsf Z$, $\mathsf E$ | the real $2\times2$ matrices $\mathsf 1_2=\mathrm{diag}(1,1)$, $\mathsf X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\mathsf Z=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, $\mathsf E=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$; each $\gamma^A_{1671}$ is $\pm$ a Kronecker product of four of them, and on the doublet index $\mathsf E=i\tau^2_{U2}$ is the antisymmetric matrix of the spin-zero pair (the columns f1 to f4 of r3\_gamma\_kron.csv hold the letters 1, X, Z, E) | new | `1` `X` `Z` `E` |
| $\mathsf 1_{16}$, $\mathsf 1_{32}$ | the unit matrices of the 16 spinor components and of the 32 components of the doublet (or of the Nambu spinor) | new |  |
| $\mathsf K$ | a real $16\times16$ matrix that defines the bilinear form $\psi^{1T}_{U2}\mathsf K\psi^2_{U2}$ of two spinors (Sec.\ on the pairs) | new |  |

**3 The new gauge sector: fields, group, covariant derivative**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\Psi_{U2}$; $\psi^1_{U2}$, $\psi^2_{U2}$ | the doublet of the new gauge theory: two quantized 16-component Dirac fields $\psi^1_{U2}$, $\psi^2_{U2}$ on the 7+1 spacetime whose components are complex Grassmann operators; $\Psi_{U2}=(\psi^1_{U2},\psi^2_{U2})$, 32 components, doublet index outside | new |  |
| $\bar\Psi_{U2}$; $\bar\psi^1_{U2}$, $\bar\psi^2_{U2}$ | the Dirac adjoint taken member by member, $\bar\Psi_{U2}=(\bar\psi^1_{U2},\bar\psi^2_{U2})=(\psi^{1\dagger}_{U2}\beta_{1671},\ \psi^{2\dagger}_{U2}\beta_{1671})$ | new |  |
| $\tilde\Psi_{U2}$ | the conjugate doublet $\mathsf E\,\Psi^*_{U2}=(\psi^{1*}_{U2},\psi^{2*}_{U2})$ multiplied by $\mathsf E$, i.e.\ $(\psi^{2*}_{U2},-\psi^{1*}_{U2})$; since the $\gamma^A_{1671}$ are real, $\Psi^*_{U2}$ transforms under Spin(7,1) like $\Psi_{U2}$, and $\tilde\Psi_{U2}$ transforms under $SU(2)_{U2}$ like $\Psi_{U2}$, with $U(1)_{U2}$ charge $-y_{U2}$ | new |  |
| $\tau^1_{U2}$, $\tau^2_{U2}$, $\tau^3_{U2}$; $\boldsymbol\tau_{U2}$ | the Pauli matrices $\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\begin{pmatrix}1&0\\0&-1\end{pmatrix}$ acting on the doublet index; $\boldsymbol\tau_{U2}=(\tau^1_{U2},\tau^2_{U2},\tau^3_{U2})$; the three matrices $\tfrac12\tau^1_{U2}$, $\tfrac12\tau^2_{U2}$, $\tfrac12\tau^3_{U2}$ generate $SU(2)_{U2}$; $\boldsymbol\tau^{(1)}_{U2}$, $\boldsymbol\tau^{(2)}_{U2}$: the same matrices acting on the first and on the second of two static sources | new |  |
| $U(2)_{U2}$; $SU(2)_{U2}$, $U(1)_{U2}$ | the new gauge group $U(2)_{U2}=(SU(2)_{U2}\times U(1)_{U2})/\mathbb Z_2$ acting on the doublet index: $SU(2)_{U2}$ by unitary $2\times2$ matrices of determinant 1, $U(1)_{U2}$ by the phases $\exp(i y_{U2}\Theta^0_{U2})$; it commutes with Spin(7,1) | new |  |
| $y_{U2}$ | the $U(1)_{U2}$ charge of $\Psi_{U2}$, $y_{U2}=\tfrac12$; the pairs $\Phi^\pm_{U2}$ carry $2y_{U2}=1$ | new | `Y` |
| $g_{U2}$, $g'_{U2}$ | the gauge couplings of $SU(2)_{U2}$ and $U(1)_{U2}$; on the 7+1 spacetime of dimension mass$^{-2}$ | new | `G` `GP` |
| $\boldsymbol W_{U2A}=(W^1_{U2A},W^2_{U2A},W^3_{U2A})$ | the $SU(2)_{U2}$ gauge field, a triple of real vector fields | new | `w` |
| $B_{U2A}$ | the $U(1)_{U2}$ gauge field, a real vector field | new | `b` |
| $\boldsymbol W_{U2AC}$, $B_{U2AC}$ | the field strengths $\boldsymbol W_{U2AC}=\partial_A\boldsymbol W_{U2C}-\partial_C\boldsymbol W_{U2A}-g_{U2}\,\boldsymbol W_{U2A}\times\boldsymbol W_{U2C}$ ($\times$ the cross product of triples) and $B_{U2AC}=\partial_AB_{U2C}-\partial_CB_{U2A}$ | new |  |
| $D_{U2A}$ | the new covariant derivative $D_{U2A}\Psi_{U2}=(\partial_A+\tfrac i2 g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+i g'_{U2}y_{U2}B_{U2A})\Psi_{U2}$; it acts on the doublet index, on each of the 16 spinor components alike | new |  |
| $\mathcal W_{U2A}$, $\mathcal F_{U2AC}$ | the connection as a Hermitian $2\times2$ matrix, $\mathcal W_{U2A}=\tfrac12g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+g'_{U2}y_{U2}B_{U2A}\mathsf 1_2$ ($D_{U2A}=\partial_A+i\mathcal W_{U2A}$), and its field strength $\mathcal F_{U2AC}=\partial_A\mathcal W_{U2C}-\partial_C\mathcal W_{U2A}+i[\mathcal W_{U2A},\mathcal W_{U2C}]$ | new | `cal_w` |
| $\Upsilon_{U2}$; $\boldsymbol\Theta_{U2}$, $\Theta^0_{U2}$ | a gauge transformation $\Upsilon_{U2}=\exp(\tfrac i2\boldsymbol\Theta_{U2}\cdot\boldsymbol\tau_{U2})\exp(iy_{U2}\Theta^0_{U2})$, with a triple $\boldsymbol\Theta_{U2}(X)$ and a function $\Theta^0_{U2}(X)$ of the coordinates | new | `ups` `theta` `theta0` |
| $J^A_{U2}$, $\boldsymbol J^A_{U2}$ | the $U(1)_{U2}$ current $\bar\Psi_{U2}\gamma^A_{1671}\Psi_{U2}$ and the $SU(2)_{U2}$ currents $\bar\Psi_{U2}\gamma^A_{1671}\tfrac12\boldsymbol\tau_{U2}\Psi_{U2}$ | new |  |

**4 Pairs, couplings, masses and Lagrangian**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\Phi^{+}_{U2}$, $\Phi^{-}_{U2}$ | the spin-zero pairs $\psi^{1T}_{U2}C^\pm_{1671}\psi^2_{U2}=\tfrac12\Psi^T_{U2}(\mathsf E\otimes C^\pm_{1671})\Psi_{U2}$; $\Phi^+_{U2}$ is a Lorentz scalar whose members have spin zero and orbital angular momentum zero in the rest frame (the singlet of total spin zero of the task); $\Phi^-_{U2}$, its pseudoscalar partner, vanishes in the non-relativistic limit; both are $SU(2)_{U2}$ singlets of $U(1)_{U2}$ charge 1 | new |  |
| $m_{U2}$ | the bare Dirac mass of each member of the doublet | new | `m` |
| $G^S_{U2}$, $G^P_{U2}$ | the quartic couplings of $(\bar\Psi_{U2}\Psi_{U2})^2$ and of $\Phi^{+\dagger}_{U2}\Phi^+_{U2}$ in the Lagrangian; positive values attract; dimension mass$^{-6}$ | new | `gs` `gp` |
| $\mathcal L_{U2}$ | the Lagrangian density of the model | new |  |
| $H_{U2}$, $N_{U2}$ | the Hamiltonian and the fermion-number operator (charge-symmetric ordering); the ground state minimizes $H_{U2}-\mu_{U2}N_{U2}$ | new |  |
| $\mathcal L^{\rm MF}_{U2}$, $H^{\rm MF}_{U2}$ | the Lagrangian and the Hamiltonian of the mean-field approximation, in which the squares of the fluctuations of $\bar\Psi_{U2}\Psi_{U2}$ and $\Phi^+_{U2}$ are dropped | new |  |
| $\mathrm{Vol}_{U2}$ | the volume of the seven-dimensional space (a large box with periodic boundary conditions; at the end $\mathrm{Vol}_{U2}\to\infty$) | new |  |

**5 Mean-field theory and spinor solutions**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\mu_{U2}$ | the chemical potential conjugate to $N_{U2}$ | new | `mu` |
| $M_{U2}$ | the effective mass $m_{U2}-2G^S_{U2}\langle\bar\Psi_{U2}\Psi_{U2}\rangle$ (M\_sym: its value on the lowest state with $\Delta_{U2}=0$) | new | `M` `mass` `M_sym` |
| $\Delta_{U2}$; $\Delta_1$, $\Delta_2$ | the pair field $\Delta_{U2}=G^P_{U2}\langle\Phi^+_{U2}\rangle$, complex, the order parameter of the breaking of $U(2)_{U2}$; $\Delta_1$, $\Delta_2$ its real and imaginary parts (in the data files: its value at the minimum; lg: $\log_{10}$, ln: natural logarithm, Landau: the small-gap form) | new | `Delta` `Delta_R7` `Delta_R8` `Delta_chiral_R7` `Landau_R7` `Landau_R8` `Delta1` `Delta2` `D050` `D080` `D095` `D105` `lgD050` `lgD080` `lgD095` `lgD105` `lnDelta` |
| $s_{U2}$ | the sum $M_{U2}^2+\lvert\Delta_{U2}\rvert^2$, on which the vacuum spectrum depends | new | `s` |
| $\vec q$, $q$, $q^j$ | the spatial momentum of a fermion mode (seven components $q^j$, $j=0,1,2,3,5,6,7$) and its length | new | `q` |
| $\psi^1_{U2}(\vec q)$, $\psi^2_{U2}(\vec q)$ | the Fourier components $\mathrm{Vol}_{U2}^{-1/2}\int d^7x\,\exp(-i\vec q\cdot\vec x)\,\psi^{1}_{U2}$ (and the same for $\psi^2_{U2}$) at a fixed time $x4$ | new |  |
| $q^F_{U2}$ | the Fermi momentum $\sqrt{\mu_{U2}^2-M_{U2}^2}$ | new |  |
| $\vec Q_{U2}$ | the pair momentum of a condensate $\Delta_{U2}\exp(i\vec Q_{U2}\cdot\vec x)$ | new |  |
| $\varepsilon_{U2}$ | the single-particle energy $\sqrt{q^2+M_{U2}^2}$ | new |  |
| $\mathsf h_{U2}$ | the $16\times16$ Hamiltonian matrix $\alpha^j_{1671}q^j+\beta_{1671}M_{U2}$ (sum over the seven space-like $j$), eigenvalues $\pm\varepsilon_{U2}$, each 8-fold | new |  |
| $\mathsf e^{+}_{U2}$, $\mathsf e^{-}_{U2}$ | real orthonormal eigenvectors of $\mathsf h_{U2}(\vec q)$ with the eigenvalues $+\varepsilon_{U2}$ and $-\varepsilon_{U2}$ (eight of each sign) | new |  |
| $\mathcal N_{U2}(\vec q)$ | the Nambu spinor $(\psi^1_{U2}(\vec q),\ \psi^{2}_{U2}(-\vec q)^{\dagger T})$: 32 components, the annihilators of $\psi^1_{U2}$ at $\vec q$ and the creators of $\psi^2_{U2}$ at $-\vec q$ | new |  |
| $\mathsf H_{U2}$ | the real symmetric $32\times32$ mean-field (Bogoliubov-de Gennes) matrix of $\mathcal N_{U2}(\vec q)$: diagonal blocks $\mathsf h_{U2}-\mu_{U2}$ and $\alpha^j_{1671}q^j-\beta_{1671}M_{U2}+\mu_{U2}$, off-diagonal blocks $\Delta_{U2}C^+_{1671}$ (for real $\Delta_{U2}$) | new |  |
| $\mathsf Y_{U2}$, $\mathsf Y'_{U2}$ | the $32\times32$ matrices of the cross terms in the square of $\mathsf H_{U2}$, for the spin-zero pair ($\mathsf Y_{U2}$, square $\mu^2_{U2}\varepsilon^2_{U2}\mathsf 1_{32}$) and for the pseudoscalar pair ($\mathsf Y'_{U2}$, square $(\mu^2_{U2}\varepsilon^2_{U2}+M^2_{U2}\lvert\Delta_{U2}\rvert^2)\mathsf 1_{32}$) | new |  |
| $E^{+}_{U2}$, $E^{-}_{U2}$ | the quasi-particle energies $\sqrt{(\varepsilon_{U2}\pm\mu_{U2})^2+\lvert\Delta_{U2}\rvert^2}$ ($E^+_{U2}$: the branch of the antiparticles, $E^-_{U2}$: the branch of the particles; E\_Cminus: the smaller energy of the pseudoscalar pairing) | new | `E_Cplus` `E_Cminus` |
| $\xi^{\pm}_{U2}$, $u^{\pm}_{U2}$, $v^{\pm}_{U2}$ | $\xi^\pm_{U2}=\pm\varepsilon_{U2}-\mu_{U2}$ and the Bogoliubov coefficients $u^\pm_{U2}=[(1+\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$, $v^\pm_{U2}=[(1-\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$ (note $E^\mp_{U2}=[(\xi^\pm_{U2})^2+\lvert\Delta_{U2}\rvert^2]^{1/2}$) | new |  |
| $a_{U2}$, $d_{U2}$ | for an eigenvector $\mathsf e^\pm_{U2}$ of $\mathsf h_{U2}(\vec q)$: the mode annihilators $a_{U2}=(\mathsf e^\pm_{U2})^T\psi^1_{U2}(\vec q)$ and $d_{U2}=(\beta_{1671}\mathsf e^\pm_{U2})^T\psi^2_{U2}(-\vec q)$ | new |  |
| $\lvert\mathrm{SF}_{U2}\rangle$, $b^\dagger_{U2}$, $\lvert\varnothing_{U2}\rangle$ | the superfluid ground state; the operator $b^\dagger_{U2}=a^\dagger_{U2}d^\dagger_{U2}$ that creates one spin-zero pair of members of opposite momenta; the state without any fermion in any mode (annihilated by every $a_{U2}$ and $d_{U2}$); the ground state is a product of factors $u^\pm_{U2}-v^\pm_{U2}b^\dagger_{U2}$ applied to it | new |  |

**6 Effective potential and its minima**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\Omega_{U2}$; $\Omega^{\rm cl}_{U2}$ | the effective potential of the superfluid: the grand potential per unit volume at zero temperature as a function of $M_{U2}$ and $\Delta_{U2}$ (data files: relative to the normal state of mass $m_{U2}$ at the same $\mu_{U2}$; lNNN: at $\lambda^P_{U2}=$ N.NN); $\Omega^{\rm cl}_{U2}=(M_{U2}-m_{U2})^2/(4G^S_{U2})+\lvert\Delta_{U2}\rvert^2/G^P_{U2}$ its classical part | new | `Omega` `Omega_R7` `Omega_R8` `O050` `O080` `O095` `O105` `l095` `l100` `l102` `l105` `l110` |
| $\Omega^{\min}_{U2}$, $\Omega^{\rm sym}_{U2}$ | the value at the global minimum, and on the lowest state with $\Delta_{U2}=0$ ($U(2)_{U2}$ unbroken) | new | `Omega_sym` |
| $\Lambda_{U2}$ | the momentum cutoff of the model; every number of Revision\_3 is in units of it | new |  |
| $G^{\rm crit}_{U2}$ | the critical pair coupling of the vacuum: $90\pi^4\Lambda_{U2}^{-6}$ (R7), $288\pi^4\Lambda_{U2}^{-6}$ (R8); twice the value for the left-handed doublet | new | `Gcrit_R7` `Gcrit_R8` `Gcrit_chiral_R7` |
| $\lambda^P_{U2}$, $\lambda^S_{U2}$ | the dimensionless couplings $G^P_{U2}/G^{\rm crit}_{U2}$ and $4G^S_{U2}/G^{\rm crit}_{U2}$ (inv\_G: $1/G^P_{U2}$) | new | `lambda` `inv_G` |
| $a^{(7)}_{U2}$, $a^{(8)}_{U2}$ | $A_7/(2\pi)^7=1/(120\pi^4)$ and $A_8/(2\pi)^8=1/(768\pi^4)$: for an integrand that depends on the length $q$ only, $\int d^7q/(2\pi)^7(\dots)=a^{(7)}_{U2}\int_0^{\Lambda_{U2}}q^6(\dots)\,dq$, and the same in 8 dimensions with $\kappa_{U2}$ | new | `A7` |
| $\kappa_{U2}$, $\kappa^{(4)}_{U2}$ | the length of the Euclidean 8-momentum, whose components are the seven components $q^j$ and the imaginary-time frequency $\kappa^{(4)}_{U2}$, in the Lorentz-invariant regularization (R8); $\kappa^2_{U2}=q^2+(\kappa^{(4)}_{U2})^2$ | new |  |
| $L_{U2}$ | the function $L_{U2}(s_{U2})=2\int_0^1\kappa_{U2}^7\ln(\kappa_{U2}^2+s_{U2})\,d\kappa_{U2}$ of the vacuum potential of regularization (R8) (units $\Lambda_{U2}=1$) | new |  |
| $\mathcal J^{(\iota)}_{U2}$; $\iota$ | the radial integrals $\mathcal J^{(\iota)}_{U2}(s_{U2})=\int_0^1q^\iota(q^2+s_{U2})^{-1/2}dq$ of the vacuum potential of regularization (R7), for the even exponents $\iota=0,2,4,6,8$ | new |  |
| $n_{U2}$ | the number density $-\partial\Omega_{U2}/\partial\mu_{U2}$ of the fermions (charge-symmetric: zero in the vacuum) | new | `n` `n050` `n080` `n095` `n105` `n_over_4mu` |
| $z_{U2}$ | $\Delta_{U2}\,d\ln\Delta_{U2}/d\mu_{U2}$ along the minimum: how fast the condensation energy changes with $\mu_{U2}$ | new | `dlnDelta_dmu` `Delta_dlnDelta_dmu` |

**7 Collective modes, stiffness and boson masses**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\sigma_{U2}$, $m_{\sigma U2}$ | the amplitude mode of the pair field about the minimum: a real scalar boson, neutral under every gauge generator, that exists only after the breaking; its mass (threshold: twice the smallest quasi-particle energy; kind 0: below it, 1: at it) | new | `msigma` `msigma_over_2Delta` `threshold` `kind` |
| $\theta_{U2}$ | the phase of the pair field; its fluctuation is the Goldstone mode, which $B_{U2A}$ absorbs | new |  |
| $f^s_{U2}$, $f^t_{U2}$, $f_{U2}$ | the stiffness of the phase: the coefficients of $\tfrac12\sum_j(\partial_j\theta_{U2})^2$ and $\tfrac12(\partial_4\theta_{U2})^2$ in the effective Lagrangian; in the Lorentz-invariant regularization (R8) both equal $f_{U2}$ (data files: their squares) | new | `fs2` `ft2` `ft2_fixed` `ft2_relaxed` `f2_R8` `fs2_over_n4mu` |
| $\mathcal L^{\rm eff}_{U2}$ | the effective Lagrangian of the phase $\theta_{U2}$ and of $B_{U2A}$ at long wavelengths | new |  |
| $m_{BU2}$ | the mass $g'_{U2}f_{U2}$ of $B_{U2A}$ after the breaking (the pair has the charge 1) | new |  |
| $v^G_{U2}$ | the velocity $f^s_{U2}/f^t_{U2}$ of the Goldstone mode if $U(1)_{U2}$ were not gauged (vG2: its square) | new | `vG_R7` `vG2` |
| $w_{U2}$ | the frequency of a small oscillation of the composite fields (time dependence $\exp(-iw_{U2}x4)$) | new |  |
| $\mathsf V^1_{U2}$, $\mathsf V^2_{U2}$, $\mathsf V^3_{U2}$ | the $32\times32$ vertices of $\Delta_1$, $\Delta_2$ and $M_{U2}$ in $\mathsf H_{U2}$: $\mathsf V^1_{U2}=\begin{pmatrix}0&C^+_{1671}\\C^+_{1671}&0\end{pmatrix}$, $\mathsf V^2_{U2}=i\begin{pmatrix}0&C^+_{1671}\\-C^+_{1671}&0\end{pmatrix}$, $\mathsf V^3_{U2}=\begin{pmatrix}\beta_{1671}&0\\0&-\beta_{1671}\end{pmatrix}$ | new |  |
| $\lvert\mathrm o\rangle_{U2}$, $\lvert\mathrm e\rangle_{U2}$; $\mathcal E^{\rm o}_{U2}$, $\mathcal E^{\rm e}_{U2}$ | an occupied and an empty eigenvector of $\mathsf H_{U2}(\vec q)$ and their eigenvalues, $\mathcal E^{\rm o}_{U2}<0\le\mathcal E^{\rm e}_{U2}$ | new |  |
| $\Pi_{U2}$, $\mathsf R_{U2}$ | the $3\times3$ polarization matrix of the fermions for the composite fields $(\Delta_1,\Delta_2,M_{U2})$ (entries numbered 1, 2, 3 in this order) and the matrix $\mathsf R_{U2}(w_{U2})$ of the small oscillations, the second derivatives of $\Omega^{\rm cl}_{U2}$ plus $\Pi_{U2}(w_{U2})$ (random-phase approximation) | new |  |
| $c^\sigma_{U2}$ | the positive function $c^\sigma_{U2}(w_{U2})=16\int\frac{d^7q}{(2\pi)^7}(q^2+s_{U2})^{-1/2}\big[4(q^2+s_{U2})-w_{U2}^2\big]^{-1}$, with $[\mathsf R_{U2}(w_{U2})]_{11}=c^\sigma_{U2}(w_{U2})(4\lvert\Delta_{U2}\rvert^2-w_{U2}^2)$ in the vacuum without scalar coupling | new |  |
| $\mathcal V^{\sigma}_{U2}$ | the pair interaction carried by $\sigma_{U2}$, $-[\mathsf R_{U2}(w_{U2})^{-1}]_{11}$; at $w_{U2}=0$ it is $-[\partial^2\Omega_{U2}/\partial\Delta_1^2]^{-1}$ | new |  |
| $V^{\rm C}_{U2}$, $\phi^{\rm Y}_{U2}$, $V^{\rm pair}_{U2}$ | the potential $V^{\rm C}_{U2}(\varrho_{U2})=\Gamma_{\rm E}(5/2)/(4\pi^{7/2}\varrho_{U2}^5)$ of a massless vector boson between two unit charges in seven space dimensions; the factor $\phi^{\rm Y}_{U2}(m_{BU2}\varrho_{U2})=\exp(-m_{BU2}\varrho_{U2})$ times $[(m_{BU2}\varrho_{U2})^2+3m_{BU2}\varrho_{U2}+3]/3$ by which the mass of $B_{U2A}$ multiplies it; the static potential of two heavy members of the doublet (data files: x is $m_{BU2}\varrho_{U2}$) | new | `x` `phi` `singlet_after` `triplet_after` `singlet_before` `triplet_before` |

**8 Numbers, regularizations and labels**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| (R7), (R8) | the two regularizations: (R7) a sharp cutoff on the length of the seven-dimensional spatial momentum; (R8) a sharp cutoff on the length of the Euclidean 8-momentum (Lorentz invariant; used in the vacuum) | new | `R7` `R8` |
| $\Gamma_{\rm E}$ | Euler's Gamma function; $\Gamma_{\rm E}(5/2)=3\sqrt\pi/4$ | new |  |
| $i$ | the imaginary unit | new |  |
| ${}^T$, ${}^\dagger$, ${}^*$; $\otimes$; $[\ ,\ ]$, $\{\ ,\ \}$; $\langle\ \rangle$ | transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; commutator and anticommutator; expectation value in the ground state | new |  |

**9 The electroweak theory as the template (suffix SM)**

| symbol | meaning | origin | names in programs and data files |
|-----------|-------------------------|----|----------|
| $\phi_{\rm SM}$, $Y_{\rm SM}$ | the Higgs doublet of the electroweak theory and its hypercharge $Y_{\rm SM}=\tfrac12$ (quoted as the template of the new covariant derivative) | new |  |
| $\mathrm D^{\rm SM}$, $g_{\rm SM}$, $g'_{\rm SM}$, $\boldsymbol\tau_{\rm SM}$, $\boldsymbol W_{\rm SM}$, $B_{\rm SM}$ | the covariant exterior derivative of the electroweak theory, its two couplings, the Pauli matrices and the gauge fields (one-forms), $\mathrm D^{\rm SM}\phi_{\rm SM}=(\mathrm d+\tfrac i2g_{\rm SM}\boldsymbol\tau_{\rm SM}\cdot\boldsymbol W_{\rm SM}+ig'_{\rm SM}Y_{\rm SM}B_{\rm SM})\phi_{\rm SM}$; $\mathrm d$ is the exterior derivative | new |  |
