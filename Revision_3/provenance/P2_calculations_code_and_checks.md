---
title: "The calculations of Revision_3: formulas, code, commands, and the complete output of every check"
subtitle: "Provenance document `P2` of PROJECT spacetimes, Revision_3"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 16, 2026"
---

<!-- This file is generated from docs_src/P2_calculations_code_and_checks.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the provenance of every computed number, table and figure of the folder
`Revision_3`. For each part of the calculation it states the formulas that the program evaluates,
the method, the command and the data files (Section 3); it says how to run everything (Section 4);
it prints the complete output of every machine check (Section 5); it lists the data files with
their meanings and columns (Section 6), the figures (Section 7), the tables and figures of the
manuscript with their sources (Section 8), the scripts (Section 9), and the source files and the
checksums (Sections 10 and 11). Section 12 is the complete table of the symbols, so that every
symbol used here is defined here. Units: the momentum cutoff $\Lambda_{U2}=1$; $\hbar$ and the
speed of light are 1.

All computations are done by one Rust program, `dark_u2` (folder `code/dark-u2-rs`,
9 source files, 2683 lines), with five commands. It uses the
library crate `code/spacetimes-rs` of the repository, unchanged.

# 2. The setting

The spacetime has the coordinates $X=\{x0,\dots,x7\}$, $x4$ the time, and the metric
$\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$. The indices $A$, $C$ run over $0,\dots,7$, the
index $j$ over the seven space-like values $0,1,2,3,5,6,7$; $\partial_A=\partial/\partial x^A$; a
repeated index is summed. The Dirac matrices $\gamma^A_{1671}$ are real $16\times16$ matrices, each
$\pm$ a Kronecker product of four of the real $2\times2$ matrices
$\mathsf 1_2=\mathrm{diag}(1,1)$, $\mathsf X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$,
$\mathsf Z=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, $\mathsf E=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$:

$$
\begin{aligned}
\gamma^{0}_{1671}&=-\mathsf Z\otimes\mathsf Z\otimes\mathsf Z\otimes\mathsf E\\
\gamma^{1}_{1671}&=-\mathsf Z\otimes\mathsf X\otimes\mathsf E\otimes\mathsf X\\
\gamma^{2}_{1671}&=\mathsf Z\otimes\mathsf X\otimes\mathsf E\otimes\mathsf Z\\
\gamma^{3}_{1671}&=-\mathsf Z\otimes\mathsf X\otimes\mathsf 1_2\otimes\mathsf E\\
\gamma^{4}_{1671}&=\mathsf X\otimes\mathsf 1_2\otimes\mathsf 1_2\otimes\mathsf 1_2\\
\gamma^{5}_{1671}&=\mathsf Z\otimes\mathsf Z\otimes\mathsf E\otimes\mathsf 1_2\\
\gamma^{6}_{1671}&=\mathsf Z\otimes\mathsf Z\otimes\mathsf X\otimes\mathsf E\\
\gamma^{7}_{1671}&=\mathsf Z\otimes\mathsf E\otimes\mathsf 1_2\otimes\mathsf 1_2
\end{aligned}
$$

They obey $\gamma^A_{1671}\gamma^C_{1671}+\gamma^C_{1671}\gamma^A_{1671}=2\eta^{AC}_{1671}\mathsf 1_{16}$.
Further: $\beta_{1671}=\gamma^4_{1671}$, $\alpha^j_{1671}=\gamma^4_{1671}\gamma^j_{1671}$ (real symmetric);
$\omega_{1671}=\gamma^0_{1671}\cdots\gamma^7_{1671}$; $\chi_{1671}=i\omega_{1671}$;
$P^L_{1671}=\tfrac12(\mathsf 1_{16}-\chi_{1671})$, $P^R_{1671}=\tfrac12(\mathsf 1_{16}+\chi_{1671})$;
$C^+_{1671}=\gamma^4_{1671}$, $C^-_{1671}=\gamma^4_{1671}\omega_{1671}$ with
$C^\pm_{1671}\gamma^A_{1671}=\pm(\gamma^A_{1671})^TC^\pm_{1671}$; the spin generators
$\Sigma^{AC}_{1671}=\tfrac14(\gamma^A_{1671}\gamma^C_{1671}-\gamma^C_{1671}\gamma^A_{1671})$; the ordered
products $\mathsf G_K=\gamma^{A_1}_{1671}\cdots\gamma^{A_r}_{1671}$ over index sets $K=(A_1<\dots<A_r)$.

The doublet is $\Psi_{U2}=(\psi^1_{U2},\psi^2_{U2})$ (32 components); the pairs are
$\Phi^\pm_{U2}=\psi^{1T}_{U2}C^\pm_{1671}\psi^2_{U2}=\tfrac12\Psi^T_{U2}(\mathsf E\otimes C^\pm_{1671})\Psi_{U2}$.
The Lagrangian is
$$\begin{aligned}\mathcal L_{U2}={}&\bar\Psi_{U2}(i\gamma^A_{1671}D_{U2A}-m_{U2})\Psi_{U2}+G^S_{U2}(\bar\Psi_{U2}\Psi_{U2})^2+G^P_{U2}\Phi^{+\dagger}_{U2}\Phi^+_{U2}\\&-\tfrac14\boldsymbol W_{U2AC}\cdot\boldsymbol W^{AC}_{U2}-\tfrac14B_{U2AC}B^{AC}_{U2},\end{aligned}$$
with $D_{U2A}=\partial_A+i\mathcal W_{U2A}$,
$\mathcal W_{U2A}=\tfrac12g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+g'_{U2}y_{U2}B_{U2A}\mathsf 1_2$,
$y_{U2}=\tfrac12$.

# 3. The calculations, command by command

## 3.1 Command `algebra` (source `algebra.rs`)

Exact arithmetic: the matrices are integer matrices, $\omega_{1671}$ and the chiral projectors are
handled with Gaussian integers (the factor $\tfrac12$ is carried as an overall denominator).
Statements R3-A1 to R3-A14 and R3-B1, R3-B2 (Section 5): the Clifford relation; reality and
symmetry; $\omega_{1671}$; $C^\pm_{1671}$ symmetric with square $\mathsf 1_{16}$ and the transposition
relations; invariance of $\psi^{1T}_{U2}C^\pm_{1671}\psi^2_{U2}$ under the 28 generators; the space of
invariant bilinear forms $\mathsf K$, solutions of
$(\Sigma^{AC}_{1671})^T\mathsf K+\mathsf K\Sigma^{AC}_{1671}=0$, has dimension exactly 2 (rank of the
$7168\times256$ system modulo the prime $2^{61}-1$ is 254, a lower bound of the rank over the
rationals, and $C^\pm_{1671}$ are two independent solutions); the symmetry of the 512 matrices
$C^\pm_{1671}\mathsf G_K$ by rank (a pair $\psi^{1T}_{U2}\mathsf K\psi^1_{U2}$ of one anticommuting field
keeps only the antisymmetric part of $\mathsf K$); the doublet pairs; the Pauli matrices and
$\Upsilon^T_{U2}\mathsf E\Upsilon_{U2}=\det(\Upsilon_{U2})\mathsf E$; the chiral structure; the rest frame,
$(\mathsf 1_{16}+\beta_{1671})C^+_{1671}(\mathsf 1_{16}+\beta_{1671})=2(\mathsf 1_{16}+\beta_{1671})$,
$(\mathsf 1_{16}\pm\beta_{1671})C^-_{1671}(\mathsf 1_{16}\pm\beta_{1671})=0$; the commutant of the 21
rotation generators (dimension 4) and of the rotations with $\beta_{1671}$ (dimension 2); the
anticommutation relations of the vertices; the Kronecker form of the $\gamma^A_{1671}$; the sum of the three
products of Pauli matrices acting on two doublets (eigenvalues $-3$ and $+1$, i.e.
$\tfrac14\boldsymbol\tau^{(1)}_{U2}\cdot\boldsymbol\tau^{(2)}_{U2}=-\tfrac34$ in the singlet, $+\tfrac14$ in the
triplet); the anomaly coefficients (sums of the fifth, third and first powers of the charges of the
left-handed components: $(\tfrac1{16},\tfrac14,1)$ for a chiral doublet, $(0,0,0)$ for the Dirac
doublet). Data files: `r3_algebra_checks.txt`, `r3_pairs_by_rank.csv`, `r3_gamma_kron.csv`.

## 3.2 Command `gauge` (source `gauge.rs`)

For random smooth fields (quadratic polynomials and exponentials in the coordinates) and random
gauge transformations
$\Upsilon_{U2}=\exp(\tfrac i2\boldsymbol\Theta_{U2}\cdot\boldsymbol\tau_{U2})\exp(iy_{U2}\Theta^0_{U2})$ at a
point, with the transformed connection
$\mathcal W'_{U2A}=\Upsilon_{U2}\mathcal W_{U2A}\Upsilon^{-1}_{U2}+i(\partial_A\Upsilon_{U2})\Upsilon^{-1}_{U2}$,
the program verifies: $D'_{U2A}\Psi'_{U2}=\Upsilon_{U2}D_{U2A}\Psi_{U2}$ (R3-G1); $\mathcal W'_{U2A}$ Hermitian
with $U(1)_{U2}$ part $B_{U2A}-\partial_A\Theta^0_{U2}/g'_{U2}$ (R3-G2);
$\mathcal F_{U2AC}=\partial_A\mathcal W_{U2C}-\partial_C\mathcal W_{U2A}+i[\mathcal W_{U2A},\mathcal W_{U2C}]
=\tfrac12g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2AC}+g'_{U2}y_{U2}B_{U2AC}\mathsf 1_2$ with
$\boldsymbol W_{U2AC}=\partial_A\boldsymbol W_{U2C}-\partial_C\boldsymbol W_{U2A}-g_{U2}\boldsymbol W_{U2A}\times\boldsymbol W_{U2C}$
(R3-G3); $[D_{U2A},D_{U2C}]\Psi_{U2}=i\mathcal F_{U2AC}\Psi_{U2}$ (R3-G4);
$\mathcal F'_{U2AC}=\Upsilon_{U2}\mathcal F_{U2AC}\Upsilon^{-1}_{U2}$ (R3-G5); the pair is multiplied by
$\det\Upsilon_{U2}=\exp(i\Theta^0_{U2})$ (R3-G6). Derivatives are fourth-order central differences
with step $10^{-3}$. Static potentials (R3-G7): the potential of a unit point charge in seven space
dimensions, $V^{\rm C}_{U2}(\varrho_{U2})=\Gamma_{\rm E}(5/2)/(4\pi^{7/2}\varrho^5_{U2})$, and the
factor $\phi^{\rm Y}_{U2}(m_{BU2}\varrho_{U2})=\exp(-m_{BU2}\varrho_{U2})[(m_{BU2}\varrho_{U2})^2+3m_{BU2}\varrho_{U2}+3]/3$
of a vector boson of mass $m_{BU2}$, which solves the radial equation of the massive field in seven
dimensions (checked with five-point differences); the potential of two heavy members of the doublet,
$$\begin{aligned}V^{\rm pair}_{U2}={}&\big[g^2_{U2}\tfrac14\boldsymbol\tau^{(1)}_{U2}\cdot\boldsymbol\tau^{(2)}_{U2}+(g'_{U2}y_{U2})^2\phi^{\rm Y}_{U2}(m_{BU2}\varrho_{U2})\big]\\&\times V^{\rm C}_{U2}(\varrho_{U2}),\end{aligned}$$
tabulated for $(g'_{U2}/g_{U2})^2=1$. Data files: `r3_gauge_checks.txt`, `r3_static_potential.csv`.

## 3.3 Command `bdg` (source `bdg.rs`)

The mean-field (Bogoliubov–de Gennes) matrix of the Nambu spinor
$\mathcal N_{U2}(\vec q)=(\psi^1_{U2}(\vec q),\psi^2_{U2}(-\vec q)^{\dagger T})$ is, for real $\Delta_{U2}$,
$$\mathsf H_{U2}(\vec q)=\begin{pmatrix}\mathsf h_{U2}-\mu_{U2}&\Delta_{U2}C^+_{1671}\\ \Delta_{U2}C^+_{1671}&\mu_{U2}-\beta_{1671}\mathsf h_{U2}\beta_{1671}\end{pmatrix},$$
with $\mathsf h_{U2}=\alpha^j_{1671}q^j+\beta_{1671}M_{U2}$ ($\mu_{U2}$ in a block means $\mu_{U2}\mathsf 1_{16}$). The program diagonalizes it (cyclic Jacobi
method, `rsym.rs`) and compares with the closed forms: eigenvalues $\pm E^\pm_{U2}$, each eightfold,
$E^\pm_{U2}=[(\varepsilon_{U2}\pm\mu_{U2})^2+\Delta_{U2}^2]^{1/2}$, $\varepsilon_{U2}=(q^2+M^2_{U2})^{1/2}$ (R3-C1);
with $C^-_{1671}$ in place of $C^+_{1671}$:
$\pm[\varepsilon^2_{U2}+\mu^2_{U2}+\Delta^2_{U2}\pm2(\mu^2_{U2}\varepsilon^2_{U2}+M^2_{U2}\Delta^2_{U2})^{1/2}]^{1/2}$
(R3-C2); the eigenvectors $(u^\pm_{U2}\mathsf e^\pm_{U2},v^\pm_{U2}\beta_{1671}\mathsf e^\pm_{U2})$ for
$+E^\mp_{U2}$ and $(-v^\pm_{U2}\mathsf e^\pm_{U2},u^\pm_{U2}\beta_{1671}\mathsf e^\pm_{U2})$ for $-E^\mp_{U2}$,
with $\mathsf h_{U2}\mathsf e^\pm_{U2}=\pm\varepsilon_{U2}\mathsf e^\pm_{U2}$,
$\xi^\pm_{U2}=\pm\varepsilon_{U2}-\mu_{U2}$, $u^\pm_{U2}=[(1+\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$,
$v^\pm_{U2}=[(1-\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$ (R3-C3); for a condensate
$\Delta_{U2}\exp(i\vec Q_{U2}\cdot\vec x)$ at $\mu_{U2}=0$ the eigenvalues
$\pm[q^2+\tfrac14Q^2_{U2}+M^2_{U2}+\Delta^2_{U2}\pm((\vec q\cdot\vec Q_{U2})^2+\Delta^2_{U2}Q^2_{U2})^{1/2}]^{1/2}$
(R3-C4); for the left-handed doublet $\pm[(q\mp\mu_{U2})^2+\Delta^2_{U2}]^{1/2}$, each fourfold (R3-C5);
independence of the phase of $\Delta_{U2}$ (R3-U1); the smallest energy at a Fermi surface,
$\Delta_{U2}$ for $C^+_{1671}$ and $\Delta_{U2}(1-M^2_{U2}/\mu^2_{U2})^{1/2}$ for $C^-_{1671}$ (R3-C6). Data
files: `r3_bdg_checks.txt`, `r3_fermi_surface_gap.csv`.

## 3.4 Command `potential` (source `potential.rs`)

**The effective potential** (mean field, zero temperature, per unit volume):
$$\Omega_{U2}=\frac{(M_{U2}-m_{U2})^2}{4G^S_{U2}}+\frac{\lvert\Delta_{U2}\rvert^2}{G^P_{U2}}-8\int\frac{d^7q}{(2\pi)^7}\big(E^+_{U2}+E^-_{U2}\big),$$
the sum of the classical part $\Omega^{\rm cl}_{U2}$ and the energy of the 16 occupied states of
$\mathsf H_{U2}$ for each $\vec q$ (charge-symmetric order, no constant because the trace of
$\mathsf H_{U2}$ vanishes). (R7): $\int d^7q/(2\pi)^7(\dots)=a^{(7)}_{U2}\int_0^1q^6(\dots)dq$,
$a^{(7)}_{U2}=1/(120\pi^4)$. The program evaluates $\Omega_{U2}$ relative to the normal state of mass
$m_{U2}$ at the same $\mu_{U2}$. (R8), vacuum only:
$\Omega^{\rm (R8)}_{U2}=\Omega^{\rm cl}_{U2}-8a^{(8)}_{U2}L_{U2}(s_{U2})$, $s_{U2}=M^2_{U2}+\lvert\Delta_{U2}\rvert^2$,
$a^{(8)}_{U2}=1/(768\pi^4)$,
$$\begin{aligned}L_{U2}(s_{U2})&=2\int_0^1\kappa^7_{U2}\ln(\kappa^2_{U2}+s_{U2})\,d\kappa_{U2}\\&=\tfrac14(1-s^4_{U2})\ln(1+s_{U2})+\tfrac14s^4_{U2}\ln s_{U2}-\tfrac1{16}+\tfrac1{12}s_{U2}-\tfrac18s^2_{U2}+\tfrac14s^3_{U2},\end{aligned}$$
$L'_{U2}(s_{U2})=\tfrac13-\tfrac12s_{U2}+s^2_{U2}-s^3_{U2}\ln[(1+s_{U2})/s_{U2}]$ (R3-P2). The (R7)
vacuum integrals in closed form (R3-P1):
$\mathcal J^{(\iota)}_{U2}(s_{U2})=\int_0^1q^\iota(q^2+s_{U2})^{-1/2}dq$,
$\mathcal J^{(0)}_{U2}=\ln[(1+(1+s_{U2})^{1/2})/s^{1/2}_{U2}]$,
$\iota\mathcal J^{(\iota)}_{U2}=(1+s_{U2})^{1/2}-(\iota-1)s_{U2}\mathcal J^{(\iota-2)}_{U2}$,
$\int_0^1q^6(q^2+s_{U2})^{1/2}dq=\mathcal J^{(8)}_{U2}+s_{U2}\mathcal J^{(6)}_{U2}$.

**Stationarity.** $\partial\Omega_{U2}/\partial\Delta^*_{U2}=0$ and $\partial\Omega_{U2}/\partial M_{U2}=0$
are the gap and the mass equations,
$$\begin{aligned}\frac1{G^P_{U2}}&=4\int\frac{d^7q}{(2\pi)^7}\Big(\frac1{E^+_{U2}}+\frac1{E^-_{U2}}\Big),\\
M_{U2}-m_{U2}&=16G^S_{U2}M_{U2}\int\frac{d^7q}{(2\pi)^7}\frac{1}{\varepsilon_{U2}}\Big(\frac{\varepsilon_{U2}+\mu_{U2}}{E^+_{U2}}+\frac{\varepsilon_{U2}-\mu_{U2}}{E^-_{U2}}\Big).\end{aligned}$$
**Critical couplings** (R3-P3): the right side of the vacuum gap equation at $s_{U2}\to0$ gives
$G^{\rm crit}_{U2}=3/(4a^{(7)}_{U2})=90\pi^4$ (R7) and $3/(8a^{(8)}_{U2})=288\pi^4$ (R8);
$\lambda^P_{U2}=G^P_{U2}/G^{\rm crit}_{U2}$, $\lambda^S_{U2}=4G^S_{U2}/G^{\rm crit}_{U2}$. **Landau forms**
(R3-P6): $\Omega_{U2}(\Delta_{U2})-\Omega_{U2}(0)=\tfrac43a^{(7)}_{U2}(1/\lambda^P_{U2}-1)\lvert\Delta_{U2}\rvert^2+\tfrac12a^{(7)}_{U2}\lvert\Delta_{U2}\rvert^4$
(R7), hence $\lvert\Delta_{U2}\rvert^2=\tfrac43(1-1/\lambda^P_{U2})$ (R7) and $\tfrac23(1-1/\lambda^P_{U2})$ (R8).
**Competition** (R3-P7): in the vacuum with $m_{U2}=0$ the fermion term depends on $s_{U2}$ only, so
the minimum is the pure pair state if $\lambda^P_{U2}>\lambda^S_{U2}$, the pure mass state if
$\lambda^S_{U2}>\lambda^P_{U2}$, a circle if they are equal. **Cooper logarithm** (R3-P8a):
$4\int d^7q/(2\pi)^7/E^-_{U2}\simeq8a^{(7)}_{U2}(q^F_{U2})^5\mu_{U2}\ln(1/\Delta_{U2})$ as
$\Delta_{U2}\to0$, $q^F_{U2}=(\mu^2_{U2}-M^2_{U2})^{1/2}$; hence a gap for every $G^P_{U2}>0$ (R3-P8) and
the weak-coupling law $\ln\Delta_{U2}=\text{const}-1/(8a^{(7)}_{U2}(q^F_{U2})^5\mu_{U2}G^P_{U2})$ (R3-P9).
**Density** (R3-P11):
$n_{U2}=-\partial\Omega_{U2}/\partial\mu_{U2}=8\int d^7q/(2\pi)^7[(\varepsilon_{U2}+\mu_{U2})/E^+_{U2}-(\varepsilon_{U2}-\mu_{U2})/E^-_{U2}]$.
**Transition** (R3-P10): $\lambda^S_{U2}=1.1$, $\lambda^P_{U2}=1.05$, $m_{U2}=0$, scan of $\mu_{U2}$.
**Chiral doublet** (R3-P12): potential $\lvert\Delta_{U2}\rvert^2/G^P_{U2}-4\int d^7q/(2\pi)^7(E^+_{U2}+E^-_{U2})$
with $M_{U2}=0$.

**Methods.** Adaptive Gauss–Legendre quadrature (`integrate.rs`, rules with 20 and 40 nodes,
relative tolerance $10^{-15}$, recursion depth at most 40). Near a Fermi surface the variable is
$\mathrm{arsinh}(\xi^+_{U2}/\Delta_{U2})$, and $E^-_{U2}$ is computed as $\Delta_{U2}$ times the hyperbolic
cosine of that variable, so that the integrands are smooth and free of rounding for every gap down to
$10^{-300}$. The gap equation is solved by Brent's method in $\ln\Delta_{U2}$. The global minimum is
found on the profile $\min_{M_{U2}}\Omega_{U2}$, with the gap solved at each $M_{U2}$, by golden-section
search and a final solution of the mass equation. Data files: `r3_potential_checks.txt`,
`r3_vacuum_gap.csv`, `r3_hat_radial.csv`, `r3_hat_2d.csv`, `r3_competition.csv`,
`r3_density_scan.csv`, `r3_bcs_slope.csv`, `r3_transition.csv`, `r3_phase_map.csv`,
`r3_spectrum.csv`, `r3_summary_potential.csv`.

## 3.5 Command `modes` (source `modes.rs`)

**Random-phase approximation.** The composite fields $(\Delta_1,\Delta_2,M_{U2})$ (real and imaginary
part of $\Delta_{U2}$, mass) enter $\mathsf H_{U2}$ through the vertices
$\mathsf V^1_{U2}=\begin{pmatrix}0&C^+_{1671}\\C^+_{1671}&0\end{pmatrix}$,
$\mathsf V^2_{U2}=i\begin{pmatrix}0&C^+_{1671}\\-C^+_{1671}&0\end{pmatrix}$,
$\mathsf V^3_{U2}=\begin{pmatrix}\beta_{1671}&0\\0&-\beta_{1671}\end{pmatrix}$. Small oscillations with
the time dependence $\exp(-iw_{U2}x4)$ exist where $\det\mathsf R_{U2}(w_{U2})=0$,
$\mathsf R_{U2}(w_{U2})=\mathrm{diag}(2/G^P_{U2},2/G^P_{U2},1/(2G^S_{U2}))+\Pi_{U2}(w_{U2})$ (only the first
two rows and columns when $G^S_{U2}=0$), with
$$\begin{aligned}[\Pi_{U2}(w_{U2})]_{11}={}&\int\frac{d^7q}{(2\pi)^7}\sum_{\rm o,e}\lvert{}_{U2}\langle\mathrm e\rvert\mathsf V^1_{U2}\lvert\mathrm o\rangle_{U2}\rvert^2\\&\times\Big[\frac1{w_{U2}-\mathcal E^{\rm e}_{U2}+\mathcal E^{\rm o}_{U2}}-\frac1{w_{U2}+\mathcal E^{\rm e}_{U2}-\mathcal E^{\rm o}_{U2}}\Big]\end{aligned}$$
over the occupied ($\mathcal E^{\rm o}_{U2}<0$) and the empty ($\mathcal E^{\rm e}_{U2}\ge0$) eigenvectors of
$\mathsf H_{U2}(\vec q)$; the other entries with the vertices of their row and column. The integral uses
Gauss–Legendre panels of 12 nodes (24 panels, or panels refined around the Fermi momentum), with
$\vec q$ along $x1$. Checks: Goldstone, $[\mathsf R_{U2}(0)]_{22}=0$ at every minimum (R3-M2, R3-M8,
R3-M9); $[\mathsf R_{U2}(0)]_{11}$ equals the second derivative of the closed-form potential (R3-M3);
in the vacuum without scalar coupling
$[\mathsf R_{U2}(w_{U2})]_{11}=c^\sigma_{U2}(w_{U2})(4\lvert\Delta_{U2}\rvert^2-w^2_{U2})$, so
$m_{\sigma U2}=2\lvert\Delta_{U2}\rvert$, at the threshold $2(m^2_{U2}+\lvert\Delta_{U2}\rvert^2)^{1/2}$ for
$m_{U2}=0$ and below it for $m_{U2}>0$ (R3-M4, R3-M12); the pair interaction carried by $\sigma_{U2}$,
$\mathcal V^\sigma_{U2}(0)=-[\partial^2\Omega_{U2}/\partial\Delta_1^2]^{-1}$, is negative (R3-M11).

**Stiffness.** Spatial: the second-order response to a phase gradient, vertex
$\tfrac12Q^j_{U2}\,\mathrm{diag}(\alpha^j_{1671},-\alpha^j_{1671})$, minus its normal-state value, which with
the cutoff on $q$ is the surface term $(2a^{(7)}_{U2}/7)(1+M^2_{U2})^{-1/2}$ per $Q^2_{U2}$ (R3-M1); for
$m_{U2}=0$,
$(f^s_{U2})^2=4a^{(7)}_{U2}\int_0^1q^6\tfrac67q^2[q^{-3}-(q^2+\Delta^2_{U2})^{-3/2}]dq$ (R3-M5).
Time-like: $(f^t_{U2})^2=-\tfrac14\partial^2\Omega_{U2}/\partial\mu^2_{U2}$ at fixed $\Delta_{U2}$
$=2a^{(7)}_{U2}\int_0^1q^6\Delta^2_{U2}[(E^+_{U2})^{-3}+(E^-_{U2})^{-3}]dq$ (R3-M6). (R8):
$f^2_{U2}=16\Delta^2_{U2}\int_{\kappa_{U2}<1}d^8\kappa_{U2}/(2\pi)^8\,(\kappa^2_{U2}+s_{U2})^{-2}
=16a^{(8)}_{U2}\Delta^2_{U2}\int_0^1\kappa^7_{U2}(\kappa^2_{U2}+s_{U2})^{-2}d\kappa_{U2}$, the same for
space-like and time-like gradients (R3-M7; the closed form
$\int_0^1\kappa^7_{U2}(\kappa^2_{U2}+s_{U2})^{-2}d\kappa_{U2}=\tfrac12[\tfrac12-2s_{U2}+3s^2_{U2}\ln((1+s_{U2})/s_{U2})+s^3_{U2}/(1+s_{U2})-s^2_{U2}]$
is R3-P2). Mass of $B_{U2A}$: $m_{BU2}=g'_{U2}f_{U2}$. At finite density (R3-M10):
$(f^s_{U2})^2=n_{U2}/(4\mu_{U2})$ within 1 per cent for $\Delta_{U2}<0.02$, and
$(v^G_{U2})^2=(f^s_{U2})^2/(f^t_{U2})^2=1/7$ within 1 per cent where $z_{U2}<0.1$, with the time-like
stiffness including the change of $\Delta_{U2}$ with $\mu_{U2}$. Data files: `r3_modes_checks.txt`,
`r3_modes_vacuum.csv`, `r3_modes_mass.csv`, `r3_modes_density.csv`, `r3_summary_modes.csv`.

# 4. How to run everything

From the folder `Revision_3` (a Rust toolchain for the edition 2021; Python 3 with NumPy and
Matplotlib; for the documents TeX Live with REVTeX 4.2, XeLaTeX and latexmk, and pandoc):

```
cargo run --release --manifest-path code/dark-u2-rs/Cargo.toml -- all
python3 code/scripts/make_figures.py
bash code/scripts/check_all.sh
```

The first command runs the five commands `algebra`, `gauge`, `bdg`, `potential`, `modes` (each can
be run alone) and writes all data files of `data/` in a few seconds; it exits with a nonzero status if
any check fails. The second draws the nine figures of `figures/`. The third runs both, then
`write_symbol_registry.py`, `check_symbols.py`, `check_self_contained.py`, `write_doc_tables.py`,
`assemble_docs.py`, `build_manuscript.py` and `build_docs.sh`, and reports every failure.

# 5. The complete output of the checks

## 5.1 Command `algebra`

```
PROJECT spacetimes, Revision_3, command `algebra`: exact checks in integer and Gaussian-integer arithmetic (the 7+1 spacetime, x4 = time)
16 of 16 checks passed

[PASS] R3-A1  the 8 real 16x16 matrices gamma^A_1671 satisfy gamma^A gamma^C + gamma^C gamma^A = 2 eta1671^{AC} 1 with eta1671 = diag(-1,-1,-1,-1,+1,-1,-1,-1) (64 identities)
[PASS] R3-A2  the matrices are real (integer); gamma^4 is symmetric and the seven gamma^j (j = 0,1,2,3,5,6,7) are antisymmetric; hence beta = gamma^4 and alpha^j = gamma^4 gamma^j are real symmetric and the generators gamma^A gamma^C of the spin group are real: the spinor representation is real, and psi^* transforms like psi
[PASS] R3-A3  omega_1671 = gamma^0 ... gamma^7 satisfies omega^2 = -1, omega^T = -omega, and anticommutes with every gamma^A
[PASS] R3-A4  C^+ = gamma^4 and C^- = gamma^4 omega are real SYMMETRIC, (C^+)^2 = (C^-)^2 = 1, and C^+ gamma^A = +(gamma^A)^T C^+, C^- gamma^A = -(gamma^A)^T C^- for every A
[PASS] R3-A5  for each of the 28 generators S = gamma^A gamma^C (A < C) of Spin(7,1): S^T C^+ + C^+ S = 0 and S^T C^- + C^- S = 0, so psi^T C^+ psi' and psi^T C^- psi' are Lorentz scalars
[PASS] R3-A6  the bilinear forms K on the 16 spinor components that are invariant under Spin(7,1) (S^T K + K S = 0 for all 28 generators) form a space of dimension exactly 2, spanned by C^+ and C^-: the rank of the linear system (7168 equations, 256 unknowns) is 254, and C^+, C^- are two independent solutions  -- rank modulo 2^61-1 = 254, independent solutions = 2
[PASS] R3-A7  pairs psi^T X psi of ONE anticommuting field are nonzero only for antisymmetric X; of the 512 matrices C^+ G_K and C^- G_K (G_K = gamma^A1 ... gamma^Ar, A1 < ... < Ar, the 256 ordered products over the index sets K) each is either symmetric or antisymmetric, and the four Lorentz-scalar ones (rank 0 and rank 8: C^+, C^-, C^+ omega, C^- omega) are all symmetric: a single Dirac field on the 7+1 spacetime has NO spin-zero (Lorentz-scalar) pair  -- antisymmetric C^+ G_K by rank 0..8: [0, 0, 28, 56, 0, 0, 28, 8, 0]; antisymmetric C^- G_K by rank 0..8: [0, 8, 28, 0, 0, 56, 28, 0, 0]
[PASS] R3-A8  on the doublet Psi_U2 = (psi^1_U2, psi^2_U2) (32 components, doublet index outside) the matrices E (x) C^+ and E (x) C^- are antisymmetric and nonzero, while S (x) C^+- is symmetric for the three symmetric 2x2 matrices S = 1, tau^1, tau^3: the spin-zero pairs Phi^+-_U2 = psi^1T C^+- psi^2 = (1/2) Psi^T (E (x) C^+-) Psi exist, and they are antisymmetric in the doublet index
[PASS] R3-A9  (tau^c)^T E + E tau^c = 0 for c = 1, 2, 3 (so U^T E U = det(U) E for every 2x2 matrix U, and E is invariant under SU(2)), E = i tau^2, and [tau^a, tau^b] = 2 i tau^c for (a,b,c) cyclic: the pair is an SU(2)_U2 singlet, and it carries the U(1)_U2 charge 2 y_U2 = 1
[PASS] R3-A10  with 2P^L = 1 - i omega, 2P^R = 1 + i omega: (P^X)^T C^+- P^Y = 0 exactly when X differs from Y, so the spin-zero pairs join a left-handed with a left-handed and a right-handed with a right-handed field; the mass term psi-bar psi joins L with R; (P^L)^T C^+ P^L is symmetric, so a single chiral field has no spin-zero pair either, while a chiral DOUBLET has one  -- C+:LL=1 C+:LR=0 C+:RL=0 C+:RR=1 C-:LL=1 C-:LR=0 C-:RL=0 C-:RR=1
[PASS] R3-A11  in the rest frame: (1+beta) C^+ (1+beta) = 2(1+beta) and (1-beta) C^+ (1-beta) = -2(1-beta), while (1+-beta) C^- (1+-beta) = 0: C^+ pairs two upper (positive-energy, rest-frame) components with each other (s-wave), C^- never does (it pairs upper with lower components, which vanish in the non-relativistic limit)
[PASS] R3-A12  the 21 rotation generators gamma^i gamma^j (i < j space-like) are real antisymmetric and commute with beta; their commutant has dimension exactly 4 (spanned by 1, beta, omega, beta omega) and the commutant of the rotations together with beta has dimension exactly 2 (1, beta): each 8-dimensional eigenspace of beta is an absolutely irreducible real representation of Spin(7), so the rotation-invariant bilinear form on it is unique and there is exactly ONE spin-zero state of two rest-frame fermions of different flavour  -- ranks modulo 2^61-1: 252 (rotations), 254 (rotations and beta); 4 resp. 2 independent solutions exhibited
[PASS] R3-A13  Hamiltonian algebra: alpha^j, beta real symmetric with {alpha^i, alpha^j} = 2 delta^{ij}, {alpha^j, beta} = 0, beta^2 = 1; C^+ anticommutes with every alpha^j and COMMUTES with beta, C^- anticommutes with every alpha^j and with beta; in the 32-dimensional Nambu space the mass vertex diag(beta,-beta) ANTICOMMUTES with the C^+ pairing vertex [[0,C^+],[C^+,0]] (mass and gap add in quadrature) but COMMUTES with the C^- pairing vertex (mass and gap add or subtract); diag(alpha^j, alpha^j) anticommutes with all three
[PASS] R3-A14  each of the 8 matrices gamma^A_1671 is +- a Kronecker product f1 (x) f2 (x) f3 (x) f4 of four of the real 2x2 matrices 1, X, Z, E (exact equality of all 256 entries), so the matrices can be written down completely in eight short formulas (data/r3_gamma_kron.csv)  -- 0:-ZZZE 1:-ZXEX 2:+ZXEZ 3:-ZX1E 4:+X111 5:+ZZE1 6:+ZZXE 7:+ZE11
[PASS] R3-B1  sum_c tau^c (x) tau^c = 2 SWAP - 1 on two doublets: on the antisymmetric (singlet) state it equals -3, on the three symmetric (triplet) states +1; hence T_1 . T_2 = -3/4 in the singlet and +1/4 in the triplet, and the exchange of the SU(2)_U2 bosons attracts in the singlet channel of the pair
[PASS] R3-B2  the symmetrized traces of 3 and of 5 Pauli matrices vanish for all 27 resp. 243 index choices (SU(2)_U2 alone has no perturbative anomaly in 8 dimensions); for the left-handed Weyl content the U(1)_U2 anomaly coefficients sum y^5, sum y^3, sum y are (1/16, 1/4, 1) for a CHIRAL doublet (anomalous: U(1)_U2 cannot be gauged) and (0, 0, 0) for the Dirac doublet, whose right-handed members are left-handed fields of charge -y  -- chiral: sum Y^5 = 2, sum Y^3 = 2, sum Y = 2 (y = Y/2); Dirac: 0, 0, 0; 243 five-fold traces checked
```

## 5.2 Command `gauge`

```
PROJECT spacetimes, Revision_3, command `gauge`: gauge covariance of the covariant derivative D_U2 and the static potentials of the pair channel
7 of 7 checks passed

[PASS] R3-G1  covariance of the covariant derivative: D'_A Psi' = Ups D_A Psi for 4 random field configurations, 8 directions each, at a point X0  -- largest relative deviation 3.02e-12
[PASS] R3-G2  the transformed connection calW'_A is Hermitian (it lies in the Lie algebra u(2)) and its U(1)_U2 part is B'_A = B_A - (d_A Theta^0)/g'_U2  -- Hermiticity 8.81e-13, U(1) part 2.75e-12
[PASS] R3-G3  the matrix field strength d_A calW_C - d_C calW_A + i[calW_A, calW_C] equals g (tau.W_AC)/2 + g' y B_AC with W_AC = d_A W_C - d_C W_A - g W_A x W_C and B_AC = d_A B_C - d_C B_A  -- largest relative deviation 1.11e-13
[PASS] R3-G4  [D_A, D_C] Psi = i calF_AC Psi (second derivatives by nested differences)  -- largest relative deviation 5.44e-13
[PASS] R3-G5  the field strength transforms covariantly: calF'_AC = Ups calF_AC Ups^-1  -- largest relative deviation 3.16e-12
[PASS] R3-G6  for 6 random gauge transformations Ups: Ups^T E Ups = det(Ups) E and det(Ups) = exp(2 i y_U2 Theta^0): the pair Phi^+_U2 is invariant under SU(2)_U2 and is multiplied by exp(i Theta^0) under U(1)_U2 (charge 2 y_U2 = 1)  -- 0.0e0, 2.3e-16
[PASS] R3-G7  the Yukawa potential of a vector boson of mass m in 7 space dimensions, G7(r) phi(m r) with phi(x) = exp(-x)(x^2+3x+3)/3, solves the radial equation Y'' + (6/r) Y' - m^2 Y = 0, and phi(0) = 1 (massless limit: the 7-dimensional Coulomb potential G7(r) = Gamma(5/2)/(4 pi^(7/2) r^5))  -- relative residual 1.0e-8
```

## 5.3 Command `bdg`

```
PROJECT spacetimes, Revision_3, command `bdg`: the mean-field (BdG) Hamiltonian of the doublet, its spectrum and its spinor solutions
7 of 7 checks passed

[PASS] R3-C1  spin-zero pair K = C^+: for 25 random (q, M, mu, Delta) the 32 eigenvalues of the real symmetric BdG matrix are +-E^+ and +-E^-, each 8-fold, with E^+- = sqrt((eps +- mu)^2 + Delta^2), eps = sqrt(q^2 + M^2)  -- largest deviation 1.29e-14
[PASS] R3-C2  pseudoscalar pair K = C^-: the eigenvalues are +-sqrt(eps^2 + mu^2 + Delta^2 +- 2 sqrt(mu^2 eps^2 + M^2 Delta^2)), each 8-fold  -- largest deviation 1.38e-14
[PASS] R3-C3  explicit spinor solutions: with w an eigenvector of h(q) = alpha.q + beta M of eigenvalue s eps (s = +-1, 8 each), xi = s eps - mu, E = sqrt(xi^2 + Delta^2), u = sqrt((1 + xi/E)/2), v = sqrt((1 - xi/E)/2), the 32 vectors (u w, v beta w) and (-v w, u beta w) are orthonormal eigenvectors of the BdG matrix with eigenvalues +E and -E  -- largest residual 1.78e-15, largest deviation from orthonormality 4.44e-16
[PASS] R3-C4  condensate with pair momentum Q (phase gradient), K = C^+, mu = 0: the eigenvalues are +-sqrt(q^2 + Q^2/4 + M^2 + Delta^2 +- sqrt((q.Q)^2 + Delta^2 Q^2)), each 8-fold  -- largest deviation 1.53e-14
[PASS] R3-C5  chiral (left-handed) doublet: psi^1_L and the conjugate psi^2*_L, which is right-handed, span 16 states; with K = C^+ and no mass the 16 eigenvalues are +-sqrt((q -+ mu)^2 + Delta^2), each 4-fold: the chiral doublet pairs with half the states of the Dirac doublet  -- largest deviation 9.99e-15
[PASS] R3-U1  the spectrum does not depend on the phase of the pair field: Delta -> Delta exp(i phi) (upper-right block times exp(i phi), lower-left block times exp(-i phi)) leaves all 32 eigenvalues unchanged (6 random phases); the effective potential depends on |Delta| only  -- largest deviation 4.00e-15
[PASS] R3-C6  the gap at the Fermi surface: for the spin-zero pair (C^+) the smallest quasi-particle energy is Delta itself, reached at eps = mu; for the pseudoscalar pair (C^-) it is Delta sqrt(1 - M^2/mu^2) = Delta q_F/mu, which vanishes in the non-relativistic limit (M = 0.3, mu = 0.5, Delta = 0.08)  -- C^+: 0.0800000000 (BdG matrix 0.0800000000); C^-: 0.0640000005 (BdG matrix 0.0640000000, predicted 0.0640000000)
```

## 5.4 Command `potential`

```
PROJECT spacetimes, Revision_3, command `potential`: the effective potential of the superfluid, its minima and the breaking U(2)_U2 -> SU(2)_U2
13 of 13 checks passed

[PASS] R3-P1  R7 vacuum integrals: Int_0^1 q^6/sqrt(q^2+r^2) dq = J_6(r) and Int_0^1 q^6 sqrt(q^2+r^2) dq = J_8(r) + r^2 J_6(r) with the reduction formula J_n = sqrt(1+r^2)/n - (n-1) r^2 J_{n-2}/n, J_0 = ln((1+sqrt(1+r^2))/r), agree with adaptive quadrature  -- largest difference 3.6e-16
[PASS] R3-P2  R8 closed forms: F(s) = Int_0^1 u^3 ln(u+s) du = (1-s^4)/4 ln(1+s) + s^4/4 ln s - 1/16 + s/12 - s^2/8 + s^3/4, F'(s) = 1/3 - s/2 + s^2 - s^3 ln((1+s)/s), Int_0^1 u^3/(u+s)^2 du = 1/2 - 2s + 3 s^2 ln((1+s)/s) + s^3/(1+s) - s^2 agree with quadrature  -- largest difference 7.8e-16
[PASS] R3-P3  critical couplings of the vacuum: R7: G^P_crit Lambda^6 = 3/(4 a7) = 90 pi^4; R8: 3/(8 b8) = 288 pi^4; the left-handed doublet needs twice the R7 value, 180 pi^4  -- R7 8766.818193 = 90 pi^4 = 8766.818193; R8 28053.818218 = 288 pi^4 = 28053.818218; chiral 17533.636386
[PASS] R3-P4  vacuum, lambda^P > 1: the gap equation 1/G^P = 4 a7 Int q^6 (1/E^+ + 1/E^-) has a root Delta > 0 (residual below 1e-12 in units of 1/G^P), Omega(Delta) < Omega(0) and Delta is a minimum along the radial direction; R8 also has Delta > 0 (102 values of lambda in (1, 3])  -- largest residual 2.1e-15
[PASS] R3-P5  vacuum, lambda^P < 1: Delta = 0 is the global minimum in both regularizations (Omega > 0 on a scan of 40 values of Delta up to 1): no breaking below the critical coupling
[PASS] R3-P6  near the critical coupling the gap follows the Landau form of the Mexican hat: Delta^2 = (4/3)(1 - 1/lambda) (R7) and (2/3)(1 - 1/lambda) (R8), from Omega = c2 Delta^2 + c4 Delta^4 with c2 = (4/3) a7 (1/lambda - 1), c4 = a7/2 (R7); at lambda = 1.0005 the ratios of the computed gap to these forms are 1 within 1 per cent  -- R7 ratio 1.00050, R8 ratio 1.00033
[PASS] R3-P7  competition in the vacuum (m = 0): the fermion term depends on M^2 + Delta^2 only, so the minimum is the pure pair state (M, Delta) = (0, Delta(lambda^P)) if lambda^P > lambda^S, the pure mass state (M(lambda^S), 0) - which does NOT break the gauge group - if lambda^S > lambda^P, and a whole circle M^2 + Delta^2 = r^2 of degenerate minima if lambda^S = lambda^P (G^P = 4 G^S)  -- (1.05,1.1): M = 0.00e0, Delta = 0.3796012728; (1.1,1.05): M = 0.3796012728, Delta = 0.0e0; spread of Omega on the circle 8.0e-20 (Omega = -7.059686e-7)
[PASS] R3-P8a  the logarithm behind the Cooper instability: for mu > 0 the integral 4 a7 Int q^6 (1/E^+ + 1/E^-) grows like 8 a7 q_F^5 mu ln(1/Delta) as Delta -> 0 (computed at Delta = 1e-100 and 1e-200 with the variable xi = Delta sinh t), so the gap equation 1/G^P = ... has a root for EVERY G^P > 0  -- largest relative deviation of the slope 1.7e-15
[PASS] R3-P8  Cooper instability: at every mu > 0 of the scan (60 values up to 0.6) and every lambda^P in {0.5, 0.8, 0.95, 1.05}, also below the critical coupling of the vacuum, the gap equation has a root Delta > 0; where Delta < 1e-300 (weak coupling at small mu) it is not representable in double precision, and its logarithm follows from R3-P8a  -- 185 points with Delta > 1e-300, 55 points with 0 < Delta < 1e-300
[PASS] R3-P9  weak-coupling (BCS) law: ln Delta = const - 1/(8 a7 q_F^5 mu G^P) at a Fermi surface; at mu = 0.4 the slope of ln Delta against 1/G^P at the two smallest couplings agrees with -1/(8 a7 q_F^5 mu) within 2 per cent  -- slope -3.567227e5, predicted -3.567227e5, ratio 1.00000
[PASS] R3-P10  density-driven breaking of the gauge group: for lambda^S = 1.1 > lambda^P = 1.05 the vacuum is the massive, U(2)_U2-symmetric state (Delta = 0); as mu grows the global minimum jumps (first-order transition) to the superfluid with Delta > 0, which breaks U(2)_U2 -> SU(2)_U2  -- Delta(mu=0) = 0, Delta(mu=0.6) = 0.560606, transition between mu = 0.190 and 0.200
[PASS] R3-P11  thermodynamic consistency: the density n = 8 a7 Int q^6 [(eps+mu)/E^+ - (eps-mu)/E^-] of the superfluid minus the density 16 a7 mu^7/7 of the normal massless gas equals -d(Omega_min - Omega_normal)/dmu (five-point differences, lambda^P = 0.8, mu = 0.2, 0.35, 0.5); the derivative of Omega_min at fixed parameters needs no derivative of Delta because Delta is stationary  -- largest relative difference 4.8e-7
[PASS] R3-P12  the left-handed (chiral) doublet: its potential at coupling G^P equals that of the Dirac doublet at G^P/2 (half the states), so in the vacuum it pairs only for G^P > 2 G^P_crit; at finite density it pairs for any attraction (lambda^P = 0.8, mu = 0.4: Delta > 0)  -- Delta(chiral, lambda^P = 0.8, mu = 0.4) = 1.950940e-22
```

## 5.5 Command `modes`

```
PROJECT spacetimes, Revision_3, command `modes`: collective modes (Goldstone, amplitude sigma_U2), phase stiffness and gauge-boson masses
12 of 12 checks passed

[PASS] R3-M1  normal vacuum (Delta = 0, mu = 0, M = 0 and M = 0.3): the second-order response of the BdG states to a phase gradient equals the surface term (2 a7/7)/sqrt(1 + M^2) of the cutoff, Int_0^1 q^6 (6 q^2/7 + M^2)/eps^3 dq = 1/(7 sqrt(1+M^2)): the subtraction of the normal state removes exactly the artefact of the cutoff  -- largest relative difference 3.0e-15
[PASS] R3-M2  Goldstone theorem: at every vacuum minimum (lambda^P = 1.005 ... 1.2) the phase entry 2/G^P + Pi_22(0) of the Hessian vanishes (relative to 2/G^P) to the accuracy of the quadrature, and every q has exactly 16 occupied BdG states  -- largest |2/G^P + Pi_22(0)| G^P/2 = 7.5e-15
[PASS] R3-M3  the amplitude entry 2/G^P + Pi_11(0) equals d^2 Omega/d Delta^2 computed from the closed-form potential by central differences, and is positive (a true minimum)  -- largest relative difference 2.9e-6
[PASS] R3-M4  the amplitude (sigma_U2) mode of the massless vacuum sits exactly at the pair-breaking threshold, m_sigma = 2 Delta: det[U'' + Pi(w)] has no zero in (0, 2 Delta), and 2/G^P + Pi_11(w) vanishes linearly as w -> 2 Delta (its ratio to 1 - w/2Delta is the same at 1e-4 and 1e-6 within 1 per cent)  -- ratios ["0.99995", "0.99995", "0.99994", "0.99992", "0.99990", "0.99984"]
[PASS] R3-M5  vacuum stiffness: the subtracted second-order response 2(S_n - S_sf) equals the closed form f_s^2 = 4 a7 Int q^6 (6q^2/7)(1/q^3 - 1/E^3) dq (m = 0), and is positive (the condensate resists phase gradients: Meissner effect for B_U2)  -- largest relative difference 2.8e-13
[PASS] R3-M6  time-like stiffness: f_t^2 = 2 a7 Int q^6 Delta^2 (1/E+^3 + 1/E-^3) equals -(1/4) d^2/dmu^2 of the fermion energy at fixed Delta (central differences)  -- largest relative difference 1.8e-6
[PASS] R3-M7  Lorentz-invariant regularization R8: f^2 = 16 Delta^2 Int d^8p/(2pi)^8 (p^2 + M^2 + Delta^2)^-2 = 8 b8 Delta^2 Int_0^1 u^3/(u+s)^2 du (s = M^2 + Delta^2), one and the same for space-like and time-like gradients  -- largest relative difference 1.3e-15
[PASS] R3-M8  Goldstone theorem with a bare mass m = 0 ... 0.25 (lambda^P = 1.05): the phase entry of the Hessian vanishes
[PASS] R3-M12  the amplitude mode in the vacuum with a bare mass m (mu = 0, no scalar coupling): m_sigma = 2 Delta exactly, as follows from 2/G^P + Pi_11(w) = c(w) (4 Delta^2 - w^2) (the radial and the angular directions of the plane (M, Delta) contribute (Delta/r)^2 c (4 r^2 - w^2) and -(M/r)^2 c w^2, r^2 = m^2 + Delta^2); for m > 0 it lies below the pair-breaking threshold 2 sqrt(m^2 + Delta^2): sigma_U2 is a stable particle  -- largest |m_sigma/(2 Delta) - 1| = 3.7e-13; m=0.00: m_sigma/threshold=1.000000, m=0.05: m_sigma/threshold=0.981859, m=0.10: m_sigma/threshold=0.925304, m=0.15: m_sigma/threshold=0.822450, m=0.20: m_sigma/threshold=0.651731, m=0.25: m_sigma/threshold=0.318082
[PASS] R3-M9  Goldstone theorem at finite density (mu = 0.49 ... 0.60, lambda^P = 0.5): the phase entry of the Hessian vanishes and every q has 16 occupied BdG states
[PASS] R3-M10  relativistic superfluid at weak coupling: (a) for Delta < 0.02 the spatial stiffness equals n/(4 mu) within 1 per cent (the whole density is superfluid at zero temperature); (b) where the condensation energy varies slowly with mu (Delta d ln Delta/d mu < 0.1) the Goldstone velocity squared f_s^2/f_t^2(relaxed) equals 1/7 = 1/(number of space dimensions), the sound speed of a scale-free gas in 7 space dimensions, within 1 per cent; at larger Delta d ln Delta/d mu the steep growth of the gap with mu (d ln Delta/d mu = 6/(8 a7 mu^7 G^P) in the BCS regime) adds to the compressibility and lowers the velocity (the deviation grows roughly like (Delta d ln Delta/d mu)^2)  -- largest deviations 0.0082, 0.0030
[PASS] R3-M11  the static pair interaction carried by the amplitude field, T_sigma(0) = -[d^2 Omega/d Delta^2]^-1, is negative (attractive) at every minimum tested; it is the zero-frequency value of the sigma_U2 propagator -[U'' + Pi(w)]^-1_11, whose pole is the sigma_U2 mass  -- (m=0,lambda=1.05,mu=0): T_sigma(0)/(-G^P/2) = 10.8977; (m=0,lambda=0.5,mu=0.56): T_sigma(0)/(-G^P/2) = 9.3417; (m=0.1,lambda=1.05,mu=0): T_sigma(0)/(-G^P/2) = 12.7281
```

## 5.6 The symbols

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

## 5.7 Self-containedness

```
PROJECT spacetimes, Revision_3: does any document send its reader to another document? (Hard Rule 7)
1 of 1 checks passed

[PASS] R3-H1  none of the 22 documents and document sources of Revision_3 refers its reader to another document for an equation, a symbol or a definition

Allowed exceptions:
  README: a provenance document named as a place to read (the README is the list of contents of the folder: naming its documents is its purpose)
```

## 5.8 Summary

| report (data/) | command | checks passed |
|---|---|---|
| `r3_algebra_checks.txt` | `dark_u2 algebra` | 16 of 16 |
| `r3_gauge_checks.txt` | `dark_u2 gauge` | 7 of 7 |
| `r3_bdg_checks.txt` | `dark_u2 bdg` | 7 of 7 |
| `r3_potential_checks.txt` | `dark_u2 potential` | 13 of 13 |
| `r3_modes_checks.txt` | `dark_u2 modes` | 12 of 12 |
| `r3_symbol_checks.txt` | `check_symbols.py` | 6 of 6 |
| `r3_self_contained_checks.txt` | `check_self_contained.py` | 1 of 1 |

# 6. The data files

Every data file begins with header lines `# generated by:`, `# meaning:` and, where numbers carry
units, `# units:`. The columns are registered names (Section 12) or descriptive words.

| data file | meaning (header line of the file) | columns |
|---|---|---|
| `r3_bcs_slope.csv` | weak-coupling (BCS) law at mu = 0.4, m = 0: ln Delta versus 1/G^P; predicted slope -1/(8 a7 q_F^5 mu) with q_F = mu | `lambda` `inv_G` `Delta` `lnDelta` |
| `r3_competition.csv` | vacuum Omega_R7(M, Delta) on a grid for (lambda^S, lambda^P) = (1.05, 1.1) [case 0], (1.1, 1.1) [case 1], (1.1, 1.05) [case 2], m = 0 | `case` `M` `Delta` `Omega` |
| `r3_density_scan.csv` | pair field, its log10 (below 1e-300 from the Cooper logarithm), number density and Omega_min - Omega_normal versus mu (m = 0, no scalar coupling) for lambda^P = 0.5, 0.8, 0.95, 1.05 (R7) | `mu` `D050` `lgD050` `n050` `O050` `D080` `lgD080` `n080` `O080` `D095` `lgD095` `n095` `O095` `D105` `lgD105` `n105` `O105` |
| `r3_fermi_surface_gap.csv` | quasi-particle energies near the Fermi surface for M = 0.3, mu = 0.5, Delta = 0.08 (units of the cutoff): normal \|eps - mu\|, spin-zero pair (C^+) E^-, pseudoscalar pair (C^-) smaller branch | `q` `normal` `E_Cplus` `E_Cminus` |
| `r3_gamma_kron.csv` | gamma^A_1671 = sign * f1 (x) f2 (x) f3 (x) f4 with the real 2x2 matrices 1, X = [[0,1],[1,0]], Z = [[1,0],[0,-1]], E = [[0,1],[-1,0]] | `A` `sign` `f1` `f2` `f3` `f4` |
| `r3_hat_2d.csv` | vacuum Omega_R7 as a function of the real and imaginary parts (Delta_1, Delta_2) of the pair field, lambda^P = 1.05, m = 0 | `Delta1` `Delta2` `Omega` |
| `r3_hat_radial.csv` | vacuum Omega_R7(Delta) - Omega_R7(0) (units Lambda^8) for lambda^P = 0.95, 1.0, 1.02, 1.05, 1.1 (m = 0, no scalar coupling) | `Delta` `l095` `l100` `l102` `l105` `l110` |
| `r3_modes_density.csv` | finite density (m = 0, lambda^P = 0.5, no scalar coupling, R7): Delta, threshold 2 Delta, sigma mass and kind, f_s^2, f_t^2 at fixed Delta, f_t^2 relaxed = (1/4) dn/dmu, n, n/(4 mu), f_s^2/(n/(4 mu)), Goldstone velocity squared f_s^2/f_t^2(relaxed), d ln Delta/d mu along the minimum, and Delta d ln Delta/d mu | `mu` `Delta` `threshold` `msigma` `kind` `fs2` `ft2_fixed` `ft2_relaxed` `n` `n_over_4mu` `fs2_over_n4mu` `vG2` `dlnDelta_dmu` `Delta_dlnDelta_dmu` |
| `r3_modes_mass.csv` | vacuum with a bare mass m (lambda^P = 1.05, mu = 0, no scalar coupling): pair field, threshold 2 sqrt(m^2 + Delta^2), sigma mass, kind (0 = bound state below threshold, 1 = at threshold) | `m` `Delta` `threshold` `msigma` `kind` |
| `r3_modes_vacuum.csv` | vacuum (mu = 0, m = 0): pair field, sigma mass in units of 2 Delta, spatial and time-like stiffness f_s^2, f_t^2 (units Lambda^6) in R7 and their ratio (the Goldstone velocity if U(1)_U2 were global), and the R8 pair field and its single (Lorentz-invariant) stiffness f^2 | `lambda` `Delta` `msigma_over_2Delta` `fs2` `ft2` `vG_R7` `Delta_R8` `f2_R8` |
| `r3_pairs_by_rank.csv` | number of ordered products G_K = gamma^A1 ... gamma^Ar (A1 < ... < Ar) of rank r for which C^+ G_K and C^- G_K are antisymmetric (a pair of one field exists) or symmetric (it vanishes) | `rank` `products` `antisym_Cplus` `antisym_Cminus` `sym_Cplus` `sym_Cminus` |
| `r3_phase_map.csv` | pair field at the minimum on the plane of lambda^P and mu (m = 0, no scalar coupling, R7) | `lambda` `mu` `Delta` |
| `r3_spectrum.csv` | quasi-particle energies (m = 0): vacuum normal q and vacuum superfluid sqrt(q^2+Delta^2) at lambda^P = 1.05; mu = 0.4, lambda^P = 0.8: normal \|q-mu\|, q+mu and superfluid E^-, E^+ (self-consistent Delta) | `q` `vac_normal` `vac_sf` `den_normal_minus` `den_normal_plus` `den_sf_minus` `den_sf_plus` |
| `r3_static_potential.csv` | static potential between two heavy members of the doublet, in units of g_U2^2 G7(r), G7(r) = Gamma(5/2)/(4 pi^(7/2) r^5), for (g'_U2/g_U2)^2 = 1, y_U2 = 1/2; x = m_B r | `x` `phi` `singlet_after` `triplet_after` `singlet_before` `triplet_before` |
| `r3_summary_modes.csv` | key numbers of the command `modes` (units Lambda_U2 = 1) | `name` `value` `meaning` |
| `r3_summary_potential.csv` | key numbers of the command `potential` (units Lambda_U2 = 1) | `name` `value` `meaning` |
| `r3_transition.csv` | density-driven breaking: lambda^S = 1.1, lambda^P = 1.05, m = 0 (R7): global minimum (M, Delta, Omega), the U(2)-symmetric branch (Delta = 0: M, Omega) and the density at the minimum, versus mu | `mu` `M` `Delta` `Omega` `M_sym` `Omega_sym` `n` |
| `r3_vacuum_gap.csv` | vacuum (mu = 0, m = 0, no scalar coupling): pair field at the minimum versus lambda^P = G^P/G^P_crit in R7 and R8, the Landau (small-gap) forms sqrt(4/3 (1-1/lambda)) and sqrt(2/3 (1-1/lambda)), the left-handed doublet (R7, same G^P_crit of R7 in lambda), and the minimum value of Omega | `lambda` `Delta_R7` `Delta_R8` `Landau_R7` `Landau_R8` `Delta_chiral_R7` `Omega_R7` `Omega_R8` |

The check reports `r3_*_checks.txt` are printed in full in Section 5.

# 7. The figures

Drawn by `code/scripts/make_figures.py` (one function per figure) from the data files; each as
`.pdf` and `.png` in `figures/`.

| figure | function | data files | what it shows |
|---|---|---|---|
| `fig_r3_hat` | `fig_hat` | `r3_hat_radial.csv`, `r3_hat_2d.csv` | the vacuum effective potential: radial cuts for five couplings; the Mexican hat at $\lambda^P_{U2}=1.05$ |
| `fig_r3_gap` | `fig_gap` | `r3_vacuum_gap.csv` | the vacuum pair field against $\lambda^P_{U2}$ in (R7) and (R8), with the Landau forms |
| `fig_r3_competition` | `fig_competition` | `r3_competition.csv` | the vacuum potential on the plane of $M_{U2}$ and $\Delta_{U2}$ for three pairs of couplings |
| `fig_r3_density` | `fig_density` | `r3_density_scan.csv`, `r3_bcs_slope.csv` | the gap against $\mu_{U2}$ for four couplings; the weak-coupling law |
| `fig_r3_transition` | `fig_transition` | `r3_transition.csv` | the first-order transition driven by density |
| `fig_r3_phase` | `fig_phase` | `r3_phase_map.csv` | the gap on the plane of $\lambda^P_{U2}$ and $\mu_{U2}$ |
| `fig_r3_spectrum` | `fig_spectrum` | `r3_spectrum.csv`, `r3_fermi_surface_gap.csv` | quasi-particle energies in the vacuum, at finite density, and at a Fermi surface for both pairs |
| `fig_r3_modes` | `fig_modes` | `r3_modes_mass.csv`, `r3_modes_vacuum.csv`, `r3_modes_density.csv` | the mass of $\sigma_{U2}$; the stiffness; Leggett's theorem and the Goldstone velocity |
| `fig_r3_static` | `fig_static` | `r3_static_potential.csv` | the static potential of two members of the doublet, singlet and triplet, before and after the breaking |

# 8. The tables and figures of the manuscript, and their sources

| in the manuscript | source |
|---|---|
| the Kronecker form of the Dirac matrices (Sec. II A) | `r3_gamma_kron.csv` (command `algebra`) |
| Table I, pairs of one field | `r3_pairs_by_rank.csv` (`algebra`) |
| Table II, vacuum minimum | `r3_vacuum_gap.csv` (`potential`) |
| Table III, amplitude mode with a bare mass | `r3_modes_mass.csv` (`modes`) |
| Table IV, vacuum modes and stiffness | `r3_modes_vacuum.csv` (`modes`) |
| Table V, finite density | `r3_modes_density.csv` (`modes`) |
| Table VI, symbols | `provenance/symbols_r3.csv` (`write_symbol_registry.py`) |
| Table VII, checks | the reports of Section 5 |
| Figs. 1 to 9 | Section 7 |
| numbers in the text | `r3_summary_potential.csv`, `r3_summary_modes.csv`, `r3_modes_checks.txt` (R3-M11), `r3_transition.csv` |

Every number is inserted into the documents by `code/scripts/write_doc_tables.py` (fragments in
`provenance/generated/`), never typed by hand.

# 9. The scripts

| script | purpose |
|---|---|
| `make_figures.py` | the nine figures |
| `write_symbol_registry.py` | the registry `provenance/symbols_r3.csv` and its fragments (the tables of the symbols) |
| `check_symbols.py` | the checks R3-S1 to R3-S6 |
| `check_self_contained.py` | the check R3-H1: no document sends its reader to another document |
| `write_doc_tables.py` | the fragments of numbers, tables, check lists and checksums |
| `assemble_docs.py` | the Markdown documents from their sources in `docs_src/` |
| `build_manuscript.py`, `tex2md.py` | the manuscript (.tex, .pdf) from `docs_src/manuscript/`, and its Markdown version |
| `build_docs.sh`, `doc_header.tex`, `check_layout.py` | the .tex and .pdf of the Markdown documents, with the layout check |
| `check_all.sh` | everything above, in order |

# 10. The source files and their checksums

```
ebce0cae8681e0f7f1924267829e07bf8a06a8cc5e0301f9b5e507c90fec8aa7  code/dark-u2-rs/src/algebra.rs
644562f6722a71eee2f0416e9c8038e5576b2cd84fa0a29bbc0c4708b0ec0313  code/dark-u2-rs/src/bdg.rs
343f72da16ef057dbf19bdf1c9738f99ca31eaa27abfa83ecfde22b3a100ba75  code/dark-u2-rs/src/checks.rs
c97b952760ba341e764cf30f4cde279fe3854bbae21cf2bf5e73223924708da3  code/dark-u2-rs/src/gauge.rs
651efab2b48a6f53e4eca1e3a1e5d4731a19d1d91bd9feeceb12362382406f1a  code/dark-u2-rs/src/integrate.rs
1e88fc6952360824bd6a491f78816d4aa1434b174a9752ac6fcb83ba6f02c3bd  code/dark-u2-rs/src/main.rs
ab6a4bad6c5140de37d7ab0e0b6557a436a4de0816198c7bbb2a99200edb50b8  code/dark-u2-rs/src/modes.rs
384aec14273a35c63398ff89bc1850abff3ce04af2d8c3b23903811e13d025f6  code/dark-u2-rs/src/potential.rs
9f19d5afaeb8cff6454da5c3a098fc74c8957498f51b0bfa2e503f31bbfb9191  code/dark-u2-rs/src/rsym.rs
0a6bee389a7858f831b41a2f67601aec7c8445aac568dafabee4e8394a89e15c  code/scripts/assemble_docs.py
27004018376dccf914fe48f18f8af84f8f57cad412370580a6d78d4be07005ea  code/scripts/build_docs.sh
cd4544027125a16d32f5dcd86183fe070f1738b786a6a346c143a6ba237b9293  code/scripts/build_manuscript.py
f927120ebaf6390160ac371fcaf68e9f059012df5a6700a071521adb13bdafbe  code/scripts/check_all.sh
465063731c88f51a1d8af013d40f1cae7613007e63d3256e5ab26b364bd3b455  code/scripts/check_layout.py
fd4648f2688fbf6b31e875c2b0828b6ee157ad605b75fe1956e87274489bdffa  code/scripts/check_self_contained.py
0ecce2f9c6b06bb02f7c1e0e132f5b6eb7fec9f98cc55300c91ec37bcec6e6b7  code/scripts/check_symbols.py
ac40294235d8cff5d08bd6145121bbd12f4f584f22999841a699ec0a2a988c08  code/scripts/make_figures.py
60ded4a97e357119ec32d67e04526dc59ec228677c1ca4a1980cae92a4b93073  code/scripts/tex2md.py
75fd9d766f0e6e24c4afd4f2877d3450ca03c68c66df6a58d7b2730c8a28cf97  code/scripts/write_doc_tables.py
c826a214995e506e647843e68e1d31df9d475aea278ca1a411953eaa08c74169  code/scripts/write_symbol_registry.py
```

# 11. Checksums of the data files and of the figures

```
af005f79587417c66507d8d6561bfe58322cb49107bdfff8ae15bc50d25f4db4  r3_algebra_checks.txt
8a84d1618db38ca3d3a36b75cc5108b41666505637475a6878b7d78e8cbf48ea  r3_bcs_slope.csv
7e543021c9cdeff80fb5a2b8fb68f1c244447adf64ddcdd84d73b057ab65c30d  r3_bdg_checks.txt
d01f084921f28bb10c1f3b0343b30a6b3656280a4a01abb0dba5fe27cc81c115  r3_competition.csv
6ee22e4473033582d03f4ea2d9e2476965aa31a6d7df6c7b9ae3a881417943d5  r3_density_scan.csv
4e3e21945ac9dd7856d93ee1367949e06420106738ed55985bda59dff4eab901  r3_fermi_surface_gap.csv
1bb9a53bafc9a05a956ed8a388bc6c3d88adad40dc54fbdc06acf7cd1544ccc1  r3_gamma_kron.csv
a6860c3679eef34d9e6b234b781ce231342ce904633023057f2ce8fb25e19f7c  r3_gauge_checks.txt
7dd499a9cb0e52c3ac59f4c0106cf926a5a41f149dadf00b0bf89f740d285e27  r3_hat_2d.csv
fdb482ebb92bac5ff055398fefca4ab609d01b31301922f36a30a7657c2b5ab0  r3_hat_radial.csv
52be8940d91a5681ebce32561355edd1ba5f9ee2c2ec724dc1277ea78b277653  r3_modes_checks.txt
db72768d2f9c8319e7b5788a5ccb9602373543213cb2676eab52b31d80442385  r3_modes_density.csv
a2531b0a350807ad0f60ce475c05dc23e0b802e5d67b162dbb0afa42827d549b  r3_modes_mass.csv
ef7a37878a48b5e2f6a6c8242e2d658547c1822170b5941327980168f690970f  r3_modes_vacuum.csv
40a2be87ab67e07130b35bea5ac80e90a675b8806cb09984fd1da6804b04d3b9  r3_pairs_by_rank.csv
04a8faa22a6476110e3843387fd8deb68e9e28c53d9191bb0a4fa0ed6abefc9f  r3_phase_map.csv
5d863e8866113e7dc5263f13c73197fe20c01c4498b090acd3d4b18b78251b7a  r3_potential_checks.txt
693e64475de2ef4d38c27c5229fee9901daa8d9aeacaa7f3c6675615ae5c7003  r3_self_contained_checks.txt
038f5aea5d4e3299c6187da715525215629f59f6efa25a7977bd98b76d9ee827  r3_spectrum.csv
59869edf7548a25f05aadab58dbba0ea0784cba1e24d6efc90a5d0afc91f3add  r3_static_potential.csv
16947430fdd4e3d49fa8e56e03afd80001742c2f6d38868038f3eb11049d5b60  r3_summary_modes.csv
6303920b9f5e2b4eb8bc6d395d764f00f2351a32059c44b8baf95fcbfa9f3165  r3_summary_potential.csv
a521ce986f27ccf4d345093f146a7e14f7c2df301751074f4054edae0ad396d9  r3_symbol_checks.txt
ef8e1845cc4c15a7739100ec0ae14bc31e3eb6e686cb086c901d641e895edf2c  r3_transition.csv
a9fbf1142cd50b9cd0c64bf2187971a8f9024007b6423014f6679d0ffa3dc74e  r3_vacuum_gap.csv
2e1335c1a027d789818c8e65628e08e447c12d23cf92815d90726ee0f20b32f9  fig_r3_competition.pdf
39c7ddbd4422f5df4a9387ca7947f529aa7851605dcdbd3ae2fc0c805063551c  fig_r3_density.pdf
855b133ba987423d91b40ae38f9ab39722fe113c2a35aaffa782953fe8e3c57c  fig_r3_gap.pdf
b23679da4e6797d4e73e18111735969383a88bdb952db4c4600868c1a3155c1d  fig_r3_hat.pdf
3b3ba2ee52aab7f762e190830eeb43d7eacdbc1bceed0130a9716a554da120cc  fig_r3_modes.pdf
3660c4c44a93b18186913f5fd1d5e450beb10b39ff215b3ef0bab75e08d89504  fig_r3_phase.pdf
c21ddef72f72185ec05c7763ae897f656d41a2305523b812c39676ae672e31fc  fig_r3_spectrum.pdf
8b1fbe0ff043d430a87f41f938b0cada8fd5beb3f6ec2c438a108589d36a46fe  fig_r3_static.pdf
1242352556da0901d24387415d172ea4055653ce6446dcc1104cf4efb46b5e2a  fig_r3_transition.pdf
```

# 12. The symbols used in this document

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
