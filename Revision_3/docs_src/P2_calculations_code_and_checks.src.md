<!-- OUTPUT: provenance/P2_calculations_code_and_checks.md -->
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
{{GEN:n_rust_files}} source files, {{GEN:n_rust_lines}} lines), with five commands. It uses the
library crate `code/spacetimes-rs` of the repository, unchanged.

# 2. The setting

The spacetime has the coordinates $X=\{x0,\dots,x7\}$, $x4$ the time, and the metric
$\eta_{1671}=\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$. The indices $A$, $C$ run over $0,\dots,7$, the
index $j$ over the seven space-like values $0,1,2,3,5,6,7$; $\partial_A=\partial/\partial x^A$; a
repeated index is summed. The Dirac matrices $\gamma^A_{1671}$ are real $16\times16$ matrices, each
$\pm$ a Kronecker product of four of the real $2\times2$ matrices
$\mathsf 1_2=\mathrm{diag}(1,1)$, $\mathsf X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$,
$\mathsf Z=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, $\mathsf E=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$:

{{GEN:gamma_kron_md}}

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
{{FILE:data/r3_algebra_checks.txt}}
```

## 5.2 Command `gauge`

```
{{FILE:data/r3_gauge_checks.txt}}
```

## 5.3 Command `bdg`

```
{{FILE:data/r3_bdg_checks.txt}}
```

## 5.4 Command `potential`

```
{{FILE:data/r3_potential_checks.txt}}
```

## 5.5 Command `modes`

```
{{FILE:data/r3_modes_checks.txt}}
```

## 5.6 The symbols

```
{{FILE:data/r3_symbol_checks.txt}}
```

## 5.7 Self-containedness

```
{{FILE:data/r3_self_contained_checks.txt}}
```

## 5.8 Summary

{{GEN:checks_table}}

# 6. The data files

Every data file begins with header lines `# generated by:`, `# meaning:` and, where numbers carry
units, `# units:`. The columns are registered names (Section 12) or descriptive words.

{{GEN:data_files_table}}

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
{{GEN:code_sha256}}
```

# 11. Checksums of the data files and of the figures

```
{{GEN:data_sha256}}
{{GEN:figure_sha256}}
```

# 12. The symbols used in this document

{{GEN:symbols_r3_table}}
