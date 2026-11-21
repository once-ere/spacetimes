---
title: "Remarks on the results, what they mean and why they matter"
subtitle: "Provenance document `P4` of PROJECT spacetimes, folder Revision_HS_CJT_and_Kohn-Sham"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 21, 2026"
---

<!-- This file is generated from docs_src/P4_remarks.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

On 21 November 2026 the author gave a standing instruction (here in my words): every qualifying remark about a result
(that a result is an approximation, a fit, of a stated accuracy, or limited in some way) must be explained clearly,
with what it means, why it holds and why it matters; the answer must be saved, and it must be put into the documents
of the solution and into the Mathematica notebook `superfluid.nb`. This document keeps these answers, one section per
remark. Each section is complete in itself: it defines every symbol and every equation it uses, gives the numbers
(written by programs from the data files; no number of the tables is typed by hand), the cause, the importance, the
corrections of the first answer given to the author, and the provenance (programs, commands, checks, data files with
their SHA-256 checksums). Section 2 keeps remark R1 (the accuracy of the Landau form). Section 3 keeps the addendum A1
that the author asked for on the same day: the free energy of the five methods at finite temperature, and whether the
order parameter is then smaller and more physically reasonable; it is complete in itself in the same way. Section 4
keeps remark R2, made in the answer of Section 3 (the free energies of the methods are mean-field free energies).

# 2. Remark R1: "The Landau form is a fit; its largest residual is 0.27–0.7 % of the depth"

**Where the remark was made.** The author asked to be shown the three effective Lagrangians $\mathcal L^{\rm CJT}$,
$\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$ of the superfluid (21 November 2026). Their potential parts were summarized by a
polynomial, with the remark above. The author then asked (1) to explain the remark clearly and completely and to discuss
why it is important, and (2) to define the "Landau form" and to give examples. Sections 2.1 to 2.7 are the answer.

## 2.1 The objects

**The spinor field and its states.** The spinor charges are described by an isodoublet of Dirac fields with eight
complex components (four Dirac components times two isospin components), of mass $m_{\rm LL}=0.2$, with the momenta
restricted to $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}=1$ (the units of the project). The free Hamiltonian matrix at the
momentum $\vec q_{\rm LL}$ is
$$\mathsf h_{\rm LL}(\vec q_{\rm LL})=\Big(\sum_{j=1}^3\alpha^j_{\rm LL}q_j+\beta_{\rm LL}m_{\rm LL}\Big)\otimes\mathsf 1_2,\qquad\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL},\quad\beta_{\rm LL}=\gamma^0_{\rm LL},$$
with the Dirac matrices $\gamma^p_{\rm LL}$ ($p=0,\dots,3$) of the chiral basis, and $\mathsf 1_2$ the unit matrix of isospin. The
spin-zero pairs of two charges at one point are described by the three antisymmetric $8\times8$ matrices
$$\mathsf K^\iota_{\rm LL}=(\mathsf C_{\rm LL}\gamma^{\rm ch}_{\rm LL})\otimes(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota),\qquad\iota=1,2,3,$$
with the charge-conjugation matrix $\mathsf C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$, the chirality matrix
$\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}$, the antisymmetric matrix $\mathsf E_{\rm LL}=i\tau^{\rm LL}_2$ and the
Pauli matrices $\tau^{\rm LL}_\iota$ of isospin.

**The uniform quasi-free states and the three methods.** A uniform state is described at every momentum by a normal
correlation matrix and a pair correlation matrix; it is the ground state of the matrix of Bogoliubov and de Gennes
$$\mathsf H_{\rm SF}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N_{\rm SF}&\mathsf P_{\rm SF}\\\mathsf P_{\rm SF}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N_{\rm SF})^T\end{pmatrix}$$
with the normal self-energy $\mathsf N_{\rm SF}$ and the pair self-energy $\mathsf P_{\rm SF}$ (both $8\times8$, independent of the
momentum). The energy per volume of a state contains the contact interactions of the potential $V^{(2)}_{\rm SF}$ of the
solution, evaluated by one of three methods $X$: the method of Hubbard and Stratonovich (HS), with the shares
$s^{\rm HS,S}_{\rm SF}$ and $s^{\rm HS,V}_{\rm SF}$ of the scalar and of the vector term that are decoupled in the density channels
(computed for $(s^{\rm HS,S}_{\rm SF},s^{\rm HS,V}_{\rm SF})=(1,1),(0,1),(0,0)$); the method of Kohn and Sham (KS); and the method of
Cornwall, Jackiw and Tomboulis (CJT). The self-energies are the derivatives of the energy per volume with respect to the
correlation matrices.

**The field and the effective potential.** The field of the superfluid boson singlet state is the complex isospin
vector $\vec\Phi^X_{\rm SF}=(\Phi^{X,1}_{\rm SF},\Phi^{X,2}_{\rm SF},\Phi^{X,3}_{\rm SF})$ of the amplitudes of the pair self-energy that
method $X$ assigns to a state,
$$\mathsf P_{\rm SF}=\sum_{\iota=1}^3\Phi^{X,\iota}_{\rm SF}(\mathsf K^\iota_{\rm LL})^\dagger.\tag{R1.1}$$
The effective potential $\mathcal V^X_{\rm SF}(\vec\Phi)$ is the smallest energy per volume of method $X$ among the uniform states
whose field is $\vec\Phi$, measured from the normal vacuum (so $\mathcal V^X_{\rm SF}(0)=0$):
$$\mathcal V^X_{\rm SF}(\vec\Phi)=\min\{\mathcal E^X_{\rm SF}[\text{state}]-\mathcal E^X_{\rm SF}[\text{normal vacuum}]:\ \vec\Phi^X_{\rm SF}(\text{state})=\vec\Phi\}.\tag{R1.2}$$
It is computed point by point: a fixed pair self-energy $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$ (the applied field, of amplitude
$\Phi_{\rm a}$) is put into $\mathsf H_{\rm SF}$, the normal self-energy is made self-consistent, and the ground state of $\mathsf H_{\rm SF}$
gives one point: its own field $\vec\Phi=\Phi\vec e_3$ and its energy per volume. The program computes 61 points per
method, with $\Phi_{\rm a}$ from $0$ to $2\Phi^X_{0,{\rm SF}}$, each self-consistent to $10^{-13}$; the applied field
$\Phi^X_{0,{\rm SF}}$ gives back the ground state of method $X$ to $10^{-13}$. The **polar** fields are
$\vec\Phi=e^{i\vartheta}\Phi\,\vec n$ with a real phase $\vartheta$, a real unit vector $\vec n$ and $\Phi=\lvert\vec\Phi\rvert$; along them the
potential is a function of $\Phi$ alone. Its minimum lies at $\Phi=\Phi^X_{0,{\rm SF}}$ (the vacuum), and the **depth** is
$\lvert\mathcal V^X_{\rm SF}(\Phi^X_{0,{\rm SF}})\rvert$, the energy per volume by which the superfluid lies below the normal vacuum.
The second derivative of the polar curve at its minimum, computed from five states about it, is
$\mathcal V''_{\rm amp}$; it fixes the mass of the amplitude mode of the field,
$$(M^{X,{\rm H}}_{\rm SF})^2=\frac{\mathcal V''_{\rm amp}}{2(Z^X_{\rm SF}+Y^X_{\rm SF})},\tag{R1.6}$$
where $Z^X_{\rm SF}$ and $Y^X_{\rm SF}$ are the two coefficients of the kinetic part of the effective Lagrangian (they are
computed from the susceptibilities of the vacuum and do not depend on the potential).

## 2.2 The Landau form: definition and examples

**Definition.** A *Landau form* of a potential is its expansion in powers of the order parameter (the field that
vanishes in the symmetric state and not in the broken one) under two rules: only polynomial invariants of the symmetry
group occur, and the expansion stops after the lowest orders that bound the potential from below and produce its
minima. L. D. Landau introduced it for phase transitions (1937): near a continuous transition the order parameter is
small, the free energy is assumed to be analytic in it, and the form is its Taylor expansion, whose coefficients (the
Landau coefficients) are derivatives at the symmetric state; the quadratic coefficient changes its sign at the
transition, and the signs of the higher coefficients decide the order of the transition and which broken state is the
minimum.

**Examples.**

* A uniaxial ferromagnet: $F_{\rm mag}=a_{\rm mag}m_{\rm mag}^2+b_{\rm mag}m_{\rm mag}^4$ for the magnetization $m_{\rm mag}$ (the symmetry
  $m_{\rm mag}\to-m_{\rm mag}$), with $a_{\rm mag}$ proportional to the temperature minus the Curie temperature and $b_{\rm mag}>0$;
  below the Curie temperature the minima are $m_{\rm mag}=\pm(-a_{\rm mag}/(2b_{\rm mag}))^{1/2}$.
* The theory of superconductivity of V. L. Ginzburg and L. D. Landau (1950): $\alpha_{\rm GL}\lvert\psi_{\rm GL}\rvert^2+\tfrac12\beta_{\rm GL}\lvert\psi_{\rm GL}\rvert^4$
  for the complex pair wave function $\psi_{\rm GL}$ (the symmetry $U(1)$), to which the gradient terms and the magnetic
  field are added. L. P. Gor'kov (1959) derived it from the theory of Bardeen, Cooper and Schrieffer near the critical
  temperature, where $\lvert\psi_{\rm GL}\rvert$ is small; there the coefficients are true Taylor coefficients.
* The Higgs potential of the Standard Model, $-(\mu^{\rm SM})^2(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}+\lambda^{\rm SM}((\Phi^{\rm SM})^\dagger\Phi^{\rm SM})^2$
  for the Higgs doublet $\Phi^{\rm SM}$: the doublet has the single invariant $(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}$ of degree 2, and this
  Landau form is exact at tree level, not a truncation.
* The Bose-Einstein condensates of spin 1 (T.-L. Ho, 1998; T. Ohmi and K. Machida, 1998), the closest analogue of the
  field $\vec\Phi$: their order parameter is a complex vector $\vec\varpi$, and their energy per volume
  $\tfrac12c_{\rm n}\lvert\vec\varpi\rvert^4+\tfrac12c_{\rm s}(\lvert\vec\varpi\rvert^4-\lvert\vec\varpi\cdot\vec\varpi\rvert^2)$ contains the same two quartic
  invariants as $\vec\Phi$, the second term being the square of the spin. $c_{\rm s}>0$ (sodium-23) favors the polar state,
  $c_{\rm s}<0$ (rubidium-87) the ferromagnetic state $\vec\varpi\propto(1,i,0)$.
* The ferroelectric barium titanate (A. F. Devonshire, 1949): $\alpha_{\rm D}P_{\rm D}^2+\beta_{\rm D}P_{\rm D}^4+\gamma_{\rm D}P_{\rm D}^6$ for the
  polarization $P_{\rm D}$, with $\beta_{\rm D}<0$ and $\gamma_{\rm D}>0$: the sixth power is required, and the transition is of
  first order.
* Superfluid helium-3 (A. J. Leggett, 1975): one quadratic and five quartic invariants of a complex $3\times3$ matrix, whose
  five coefficients decide between the phases A and B.
* The nematic liquid crystals (P. G. de Gennes, 1971): the traceless symmetric tensor of the nematic order admits an
  invariant of the third degree, so the transition is of first order.

The name has nothing to do with the Landau levels of a charge in a magnetic field (the same physicist, another
subject).

**The Landau form of the field $\vec\Phi$.** The symmetry is $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$: the isospin rotations turn
$\vec\Phi$ as a vector, and the fermion number multiplies it by a phase. The independent invariants up to the fourth
degree are $\lvert\vec\Phi\rvert^2$, $\lvert\vec\Phi\rvert^4$ and $\lvert\vec\Phi\cdot\vec\Phi\rvert^2$, and
$\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2=\lvert\vec\Phi\times\vec\Phi^*\rvert^2$ vanishes exactly on the polar fields. The Landau
form used by the solution is
$$\mathcal V^X_{\rm SF}\approx a^X_{\rm SF}\lvert\vec\Phi\rvert^2+b^X_{\rm SF}\lvert\vec\Phi\rvert^4+c^X_{\rm SF}\lvert\vec\Phi\rvert^6+u^X_{\rm SF}\big(\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2\big).\tag{R1.3}$$
Along the polar fields it is the polynomial $\mathcal V_{\rm L}(\Phi)=a^X_{\rm SF}\Phi^2+b^X_{\rm SF}\Phi^4+c^X_{\rm SF}\Phi^6$. The coefficient
$u^X_{\rm SF}$ is not fitted: it is computed from the second derivative of $\mathcal V^X_{\rm SF}$ in a non-unitary direction at the
minimum, and $u^X_{\rm SF}>0$ in every method, so the polar minimum is a minimum also in the non-unitary directions (the
sodium case of the condensates of spin 1).

## 2.3 What the remark says: the fit and its residual

**"The Landau form is a fit."** The coefficients $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$ are not derived from the theory. They
minimize the sum of the squares of the residuals $\mathcal V_{\rm L}(\Phi_i)-\mathcal V^X_{\rm SF}(\Phi_i)$ over the computed points
$\Phi_i\le1.4\Phi^X_{0,{\rm SF}}$ of the polar curve (47 to 51 points per method). (R1.3) is therefore a summary of the computed
function, neither an exact formula nor a derived expansion.

**"Its largest residual is 0.27–0.7 % of the depth."** The largest residual $\max_i\lvert\mathcal V_{\rm L}(\Phi_i)-\mathcal V^X_{\rm SF}(\Phi_i)\rvert$,
divided by the depth, is $0.27$ per cent (HS (0, 0)) to $0.70$ per cent (HS (1, 1)) (row 8 of Table R1). Since the computed points are accurate to about
$10^{-13}$, every deviation of (R1.3) from them is an error of the form, not of the computation. Figure R1 (left) shows
the residual along the fitted interval: it changes its sign $3$ times (four arcs of alternating sign), it is
largest at the upper end ($\Phi\approx1.39\Phi^X_{0,{\rm SF}}$), and at the minimum it is $0.11$ per cent (HS (0, 0)) to $0.31$ per cent (HS (1, 1)) of the depth. Such an
oscillation about the curve is the mark of a polynomial that approximates a function which is not a polynomial; it is
not numerical noise.

![](../figures/fig_p4_r1_landau.png){width=100%}

**Figure R1.** The accuracy of the Landau form. Left: the residual $\mathcal V_{\rm L}(\Phi)-\mathcal V^X_{\rm SF}(\Phi)$ of (R1.3) along the
polar fields at the computed points of the fitted interval, relative to the depth (KS and CJT coincide along the polar
fields; the figure shows one curve for both). Right: the largest residual of the least-squares fits
$\sum_{k=1}^Kc_k\Phi^{2k}$ against the highest power $2K$; $2K=4$ is the quartic form of the Standard Model, $2K=6$ is (R1.3).
Data: `data/part3_landau_residuals.csv`, `data/part3_landau.csv`.

## 2.4 Why no polynomial can be exact: the cause

**The pair field enters the quasi-particle energies like a mass.** For the applied pair self-energy
$\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$ and a normal self-energy that is a Dirac mass, $\mathsf N_{\rm SF}=(M^{X,{\rm D}}_{\rm SF}-m_{\rm LL})\beta_{\rm LL}\otimes\mathsf 1_2$,
the eigenvalues of $\mathsf H_{\rm SF}(\vec q_{\rm LL})$ are
$$\pm E(\vec q_{\rm LL}),\qquad E(\vec q_{\rm LL})=\big(\lvert\vec q_{\rm LL}\rvert^2+(M^{X,{\rm D}}_{\rm SF})^2+\Phi_{\rm a}^2\big)^{1/2},\quad\text{each eight times}\tag{R1.4}$$
(*checks P3-8 and YP3-8*, at 20 random momenta, masses and fields), and along the whole polar curve the normal
self-energy is such a Dirac mass (*checks P3-9 and YP3-9*). The pair field $\Phi_{\rm a}$ adds to the square of the mass.

**A logarithm.** The energy per volume is an integral over the momenta $q=\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$ of functions of
$(q^2+s)^{1/2}$ with $s=(M^{X,{\rm D}}_{\rm SF})^2+\Phi_{\rm a}^2$. For example,
$$\int_0^{\Lambda_{\rm LL}}q^2(q^2+s)^{1/2}\,dq=\frac{\Lambda_{\rm LL}(2\Lambda_{\rm LL}^2+s)(\Lambda_{\rm LL}^2+s)^{1/2}}8-\frac{s^2}8\ln\frac{\Lambda_{\rm LL}+(\Lambda_{\rm LL}^2+s)^{1/2}}{s^{1/2}},\tag{R1.5}$$
which contains the term $\tfrac1{16}s^2\ln s$; it is not analytic at $s=0$, that is, at $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$. Hence
$\mathcal V^X_{\rm SF}$ is not a polynomial in $\lvert\vec\Phi\rvert^2$, and no polynomial of finite degree reproduces it exactly.

**The Taylor series converges only near the origin.** A series of powers of $\lvert\vec\Phi\rvert^2$ about the origin can
converge only within a radius of the order of the Dirac mass (the singularity lies at $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$, and
$M^{X,{\rm D}}_{\rm SF}=m_{\rm LL}=0.2$ at the origin), while $\Phi^X_{0,{\rm SF}}=0.31$ to $0.40$ and the fitted interval reaches $0.42$ to
$0.56$. The superfluid is therefore far from the regime of Landau's theory, in which the order parameter is small and
the Landau coefficients are Taylor coefficients. (R1.3) has the Landau form, but its coefficients are least-squares
numbers.

**More powers, smaller residual.** The least-squares fits $\sum_{k=1}^Kc_k\Phi^{2k}$ on the same points reduce the largest
residual to $0.0044$ per cent (HS (0, 0)) to $0.038$ per cent (HS (1, 1)) of the depth with the powers to $\Phi^8$ ($K=4$) and to $4.3\times10^{-5}$ per cent (KS, CJT) to $0.00067$ per cent (HS (1, 1)) with the powers to
$\Phi^{12}$ ($K=6$) (rows 9 to 11 of Table R1, Figure R1 right): the residual of (R1.3) is the truncation of a series. The
quartic form $a\Phi^2+b\Phi^4$, the form of the Higgs potential of the Standard Model, misses by $2.8$ per cent (HS (0, 0)) to $5.9$ per cent (HS (1, 1)).

## 2.5 Why it matters

1. **It enters no other result.** The effective Lagrangians of the solution contain the computed function
   $\mathcal V^X_{\rm SF}$, not (R1.3), and no other result uses $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$: the mass of the amplitude mode
   (R1.6) uses the curvature $\mathcal V''_{\rm amp}$ of the computed curve; $u^X_{\rm SF}$ and the mass of the non-unitary modes use
   the computed curvature in the non-unitary direction; the masses of the gauge bosons and the dynamics use
   $\Phi^X_{0,{\rm SF}}$, $Z^X_{\rm SF}$, $Y^X_{\rm SF}$ and the susceptibilities. This was verified by searching the programs.
2. **It matters to whoever replaces $\mathcal V^X_{\rm SF}$ by (R1.3)**, as in a model of the type of Ginzburg and Landau or of
   Higgs. The errors are those of rows 1 to 4 of Table R1: the minimum $\Phi_{\rm L}$ of the polynomial is
   $0.041$ per cent (KS, CJT) to $0.11$ per cent (HS (1, 1)) too small; its depth is $0.12$ per cent (HS (0, 0)) to $0.31$ per cent (HS (1, 1)) too large; its curvature at its minimum is $0.86$ per cent (HS (0, 0)) to $2.3$ per cent (HS (1, 1))
   too large; and the mass of the amplitude mode by (R1.6) is $0.43$ per cent (HS (0, 0)) to $1.1$ per cent (HS (1, 1)) too large. A derivative enlarges the relative
   error of the function: a residual of a few tenths of a per cent of the depth becomes an error of one to two per cent
   of the curvature. Beyond the fitted interval the error grows: fitted on the whole computed curve (up to
   $\Phi_{\max}=$ $1.52$ to $1.64$ times $\Phi^X_{0,{\rm SF}}$), (R1.3) misses by $1.2$ per cent (HS (0, 0)) to $1.7$ per cent (HS (1, 1)) of the depth (row 17).
3. **The coefficients are not constants of the theory.** $a^X_{\rm SF}$ differs by $0.68$ per cent (KS, CJT) to $2.0$ per cent (HS (1, 1)) from the Taylor coefficient
   $a^{X,{\rm Tay}}_{\rm SF}$ of $\lvert\vec\Phi\rvert^2$ at the origin (rows 5 and 6). $a^{X,{\rm Tay}}_{\rm SF}$ is computed directly from three
   states with the small applied fields $\Phi_{\rm a}=0.004$, $0.008$, $0.016$: the quotient $f=\mathcal V^X_{\rm SF}/\Phi^2$ equals
   $a^{X,{\rm Tay}}_{\rm SF}+f_2\Phi^2+O(\Phi^4)$ with an unknown $f_2$, and two of the points eliminate $f_2$ (Richardson); the two pairs
   agree to $10^{-7}$ (*check P3-10*), and the Python program finds the same value to $10^{-10}$ (*check YP3-10*).
   $b^X_{\rm SF}$ and $c^X_{\rm SF}$ change with the fitted interval and with the number of powers: for CJT, $10^3c^{\rm CJT}_{\rm SF}$ is
   $16.34$, $19.94$ and $23.62$ when (R1.3) is fitted on $[0,\Phi^X_{0,{\rm SF}}]$, on $[0,1.4\Phi^X_{0,{\rm SF}}]$ and on the whole computed curve,
   and $11.61$ when the power $\Phi^8$ is added (rows 13 to 15 and 18). No physical meaning may be given to
   $b^X_{\rm SF}$ or $c^X_{\rm SF}$ alone, and methods may be compared through them only within the accuracy of the fit. The
   physical quantities are those that the computed function defines: $\Phi^X_{0,{\rm SF}}$, the depth, the curvatures at the
   minimum, and $a^{X,{\rm Tay}}_{\rm SF}$.
4. **The qualitative results do not depend on the fit.** That $\mathcal V^X_{\rm SF}$ vanishes at the origin, is negative near it,
   has its minimum at $\Phi^X_{0,{\rm SF}}$, depends only on the two invariants, and has a stable polar minimum
   ($u^X_{\rm SF}>0$) are properties of the computed function, checked on it directly. The sign of $a^X_{\rm SF}$ is that of
   $a^{X,{\rm Tay}}_{\rm SF}$ (negative: the normal vacuum is unstable).
5. **The comparison with the Standard Model.** The field has a Mexican hat with the symmetry structure of the Higgs
   sector and of the polar phase of a condensate of spin 1, but its potential is not the quartic potential of the
   Standard Model: the quartic form misses $\mathcal V^X_{\rm SF}$ by $2.8$ per cent (HS (0, 0)) to $5.9$ per cent (HS (1, 1)) of the depth, about ten times more than
   (R1.3).

## 2.6 The numbers

| quantity | HS (1, 1) | HS (0, 1) | HS (0, 0) | KS | CJT |
|:-----------------------------|-------:|-------:|-------:|-------:|-------:|
| minimum of (R1.3): $\Phi_{\rm L}/\Phi^X_{0,{\rm SF}}-1$ | $-0.11$ | $-0.090$ | $-0.066$ | $-0.041$ | $-0.041$ |
| depth of (R1.3): $\mathcal V_{\rm L}(\Phi_{\rm L})/\mathcal V^X_{\rm SF}(\Phi^X_{0,{\rm SF}})-1$ | $0.31$ | $0.21$ | $0.12$ | $0.12$ | $0.12$ |
| curvature of (R1.3) at its minimum: $\mathcal V_{\rm L}''(\Phi_{\rm L})/\mathcal V''_{\rm amp}-1$ | $2.3$ | $1.5$ | $0.86$ | $0.91$ | $0.91$ |
| $M^{X,{\rm H}}_{\rm SF}$ from (R1.3), relative error | $1.1$ | $0.75$ | $0.43$ | $0.45$ | $0.45$ |
| $10^3a^{X,{\rm Tay}}_{\rm SF}$ (Taylor coefficient at the origin) | $-6.16243$ | $-5.35342$ | $-4.29666$ | $-4.29666$ | $-4.29666$ |
| $a^X_{\rm SF}/a^{X,{\rm Tay}}_{\rm SF}-1$ | $-2.0$ | $-1.4$ | $-0.81$ | $-0.68$ | $-0.68$ |
| largest residual / depth, powers to $\Phi^4$ (quartic) | $5.9$ | $4.4$ | $2.8$ | $5.0$ | $5.0$ |
| the same, powers to $\Phi^6$: (R1.3) | $0.70$ | $0.47$ | $0.27$ | $0.28$ | $0.28$ |
| the same, powers to $\Phi^8$ | $0.038$ | $0.012$ | $0.0044$ | $0.021$ | $0.021$ |
| the same, powers to $\Phi^{10}$ | $0.0099$ | $0.0053$ | $0.0025$ | $0.0015$ | $0.0015$ |
| the same, powers to $\Phi^{12}$ | $0.00067$ | $0.00052$ | $0.00032$ | $4.3\times10^{-5}$ | $4.3\times10^{-5}$ |
| sign changes of the residual of (R1.3) | $3$ | $3$ | $3$ | $3$ | $3$ |
| $10^3c^X_{\rm SF}$, (R1.3) fitted on $[0,\Phi^X_{0,{\rm SF}}]$ | $8.048$ | $8.524$ | $8.450$ | $16.34$ | $16.34$ |
| $10^3c^X_{\rm SF}$, fitted on $[0,1.4\Phi^X_{0,{\rm SF}}]$ | $14.36$ | $14.40$ | $14.00$ | $19.94$ | $19.94$ |
| $10^3c^X_{\rm SF}$, fitted on the whole curve | $17.67$ | $18.42$ | $18.70$ | $23.62$ | $23.62$ |
| end of the whole curve: $\Phi_{\max}/\Phi^X_{0,{\rm SF}}$ | $1.52$ | $1.57$ | $1.64$ | $1.61$ | $1.61$ |
| largest residual / depth on the whole curve | $1.7$ | $1.5$ | $1.2$ | $1.3$ | $1.3$ |
| $10^3c$ of the fit with powers to $\Phi^8$ | $0.6157$ | $1.975$ | $2.736$ | $11.61$ | $11.61$ |

**Table R1.** The accuracy of the Landau form (R1.3), for the five methods. Rows 1 to 4: what the polynomial
$\mathcal V_{\rm L}$ gets wrong at its minimum $\Phi_{\rm L}$ (the position, the depth and the curvature $\mathcal V_{\rm L}''(\Phi_{\rm L})$, compared
with $\Phi^X_{0,{\rm SF}}$, the depth and $\mathcal V''_{\rm amp}$ of the computed curve, and the resulting error of $M^{X,{\rm H}}_{\rm SF}$ by
(R1.6)). Rows 5, 6: the Taylor coefficient of $\lvert\vec\Phi\rvert^2$ at the origin and the deviation of the fitted $a^X_{\rm SF}$
from it. Rows 7 to 11: the largest residual of the least-squares fits with the powers $\Phi^2,\dots,\Phi^{2K}$, $K=2$ to $6$,
relative to the depth. Row 12: the sign changes of the residual of (R1.3). Rows 13 to 17: $c^X_{\rm SF}$ fitted on three
intervals, the end $\Phi_{\max}$ of the computed curve, and the largest residual of the fit on the whole curve. Row 18:
the coefficient of $\Phi^6$ of the fit with the powers to $\Phi^8$. Rows 1 to 4, 6 to 11 and 17 in per cent. The symbols of
the table: $\mathcal V''_{\rm amp}$ (the curvature of the computed polar curve at its minimum, Section 2.1); $a^{X,{\rm Tay}}_{\rm SF}$
(the Taylor coefficient, item 3 of Section 2.5); $\Phi_{\max}$ (the largest field of the computed curve). Data:
`data/part3_landau.csv`, `data/part3_values.csv`.

## 2.7 Corrections of the first answers

The first answers to the author (21 November 2026) contained five statements that were not exact; the numbers above are
the corrected ones, computed by the programs of Section 2.8.

1. *"The residual changes sign five times."* It changes sign three times (row 12 of Table R1). The number had been read
   from a list of every third residual without counting.
2. *"A fit on $[0,2\Phi_0]$ has residuals of 1.2–1.7 %."* That fit used all 61 computed points; the applied field reaches
   $2\Phi^X_{0,{\rm SF}}$, but the field of the states reaches only $1.52$ to $1.64$ times $\Phi^X_{0,{\rm SF}}$ (row 16). The correct
   statement: a fit on the whole computed curve has residuals of $1.2$ per cent (HS (0, 0)) to $1.7$ per cent (HS (1, 1)) of the depth.
3. *"The field of the states reaches only 1.52–1.61 $\Phi_0$"* (in the first correction of item 2): the range is
   $1.52$ to $1.64$ (HS $(0,0)$ reaches $1.64$).
4. *"The power series converges only for $\lvert\Phi\rvert<M$."* The singularity lies at $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$ in the
   applied field; in the field $\Phi$ of the states the radius of convergence is of the order of the Dirac mass, not
   exactly equal to it.
5. *"The curvature at the minimum is 1.1–2.7 % too large, and $M^{X,{\rm H}}_{\rm SF}$ 0.5–1.3 %."* Those numbers are the curvature of
   the polynomial at the minimum $\Phi^X_{0,{\rm SF}}$ of the computed curve. A user of (R1.3) evaluates the polynomial at its own
   minimum $\Phi_{\rm L}$, where the errors are $0.86$ per cent (HS (0, 0)) to $2.3$ per cent (HS (1, 1)) and $0.43$ per cent (HS (0, 0)) to $1.1$ per cent (HS (1, 1)) (rows 3 and 4); the table gives these.

## 2.8 Provenance

**Programs.** The Rust program `sf_part3` (crate `rust/sfpart3`) computes the polar curves (module `effpot`) and the
accuracy of the Landau form (module `landau`: the least squares by Householder reflections on scaled columns, the
minimum of the polynomial, the fits with $K=2$ to $6$ and on three intervals, the sign changes, the Taylor coefficient
by Richardson, and the checks P3-8 to P3-11). The Python program `python/sf_part3.py` repeats the checks with its own
engine and with LAPACK (YP3-8 to YP3-10) and recomputes every number of Table R1 from the data files with the
singular-value decomposition of numpy (YP3-11). `python/sf_part3_figures.py` draws Figure R1, and
`code/scripts/write_part3_fragments.py` writes Table R1 and every number of this section from the data files. The
notebook `superfluid.nb` repeats the computation of Table R1 in the Wolfram language from the same polar curves.

**Commands** (from the folder `Revision_HS_CJT_and_Kohn-Sham`):

    (cd rust && cargo build --release && ./target/release/sf_part3)
    (cd python && python3 sf_part3.py)
    python3 python/sf_part3_figures.py
    python3 code/scripts/write_part3_fragments.py
    python3 code/scripts/assemble_docs.py docs_src/P4_remarks.src.md
    bash code/scripts/build_docs.sh provenance/P4_remarks.md

**The checks** (from the reports `data/hck_part3_checks.txt` and `data/hck_part3_checks_python.txt`; the deviations of
P3-10 are relative differences, those of YP3-10 and YP3-11 relative differences from the Rust values):

| check | result | deviation | tolerance | statement |
|:---|:---|---:|---:|:-----------|
| P3-8 | PASS | $2.44\times10^{-15}$ | $1\times10^{-13}$ | the pair field enters the quasi-particle energies like a mass, (R1.4): the BdG matrix with N = (M - m) beta (x) 1\_2 and P = Phi\_a (K\^3)\^dag has the eigenvalues -+(q\^2 + M\^2 + Phi\_a\^2)\^(1/2), eight times each (20 random q, M, Phi\_a) |
| P3-9 HS_1_1 | PASS | $2.03\times10^{-17}$ | $1\times10^{-12}$ | HS\_1\_1: along the polar curve the normal self-energy of every state is a Dirac mass, so the quasi-particle energies are (R1.4) (largest deviation from a multiple of beta (x) 1\_2) |
| P3-11 HS_1_1 | PASS | $1.1\times10^{-14}$ | $1\times10^{-9}$ | HS\_1\_1: the least squares of (R1.3) by Householder reflections give the coefficients of the normal equations (sum of the relative differences of a, b, c) |
| P3-10 HS_1_1 | PASS | $8.79\times10^{-9}$ | $1\times10^{-6}$ | HS\_1\_1: the coefficient of Phi\^2 of the Taylor series at the origin, a\^Tay = -6.1624310941e-3, by Richardson from the fields 0.004, 0.008 and from 0.008, 0.016 (relative difference; the fit (R1.3) has a = -6.0408804409e-3) |
| P3-9 HS_0_1 | PASS | $2.27\times10^{-17}$ | $1\times10^{-12}$ | HS\_0\_1: along the polar curve the normal self-energy of every state is a Dirac mass, so the quasi-particle energies are (R1.4) (largest deviation from a multiple of beta (x) 1\_2) |
| P3-11 HS_0_1 | PASS | $5.62\times10^{-13}$ | $1\times10^{-9}$ | HS\_0\_1: the least squares of (R1.3) by Householder reflections give the coefficients of the normal equations (sum of the relative differences of a, b, c) |
| P3-10 HS_0_1 | PASS | $5.42\times10^{-9}$ | $1\times10^{-6}$ | HS\_0\_1: the coefficient of Phi\^2 of the Taylor series at the origin, a\^Tay = -5.3534198133e-3, by Richardson from the fields 0.004, 0.008 and from 0.008, 0.016 (relative difference; the fit (R1.3) has a = -5.2811441242e-3) |
| P3-9 HS_0_0 | PASS | $0$ | $1\times10^{-12}$ | HS\_0\_0: along the polar curve the normal self-energy of every state is a Dirac mass, so the quasi-particle energies are (R1.4) (largest deviation from a multiple of beta (x) 1\_2) |
| P3-11 HS_0_0 | PASS | $8.61\times10^{-13}$ | $1\times10^{-9}$ | HS\_0\_0: the least squares of (R1.3) by Householder reflections give the coefficients of the normal equations (sum of the relative differences of a, b, c) |
| P3-10 HS_0_0 | PASS | $7.21\times10^{-9}$ | $1\times10^{-6}$ | HS\_0\_0: the coefficient of Phi\^2 of the Taylor series at the origin, a\^Tay = -4.2966587316e-3, by Richardson from the fields 0.004, 0.008 and from 0.008, 0.016 (relative difference; the fit (R1.3) has a = -4.2619672631e-3) |
| P3-9 KS | PASS | $2.22\times10^{-16}$ | $1\times10^{-12}$ | KS: along the polar curve the normal self-energy of every state is a Dirac mass, so the quasi-particle energies are (R1.4) (largest deviation from a multiple of beta (x) 1\_2) |
| P3-11 KS | PASS | $7.5\times10^{-13}$ | $1\times10^{-9}$ | KS: the least squares of (R1.3) by Householder reflections give the coefficients of the normal equations (sum of the relative differences of a, b, c) |
| P3-10 KS | PASS | $7.43\times10^{-8}$ | $1\times10^{-6}$ | KS: the coefficient of Phi\^2 of the Taylor series at the origin, a\^Tay = -4.2966587553e-3, by Richardson from the fields 0.004, 0.008 and from 0.008, 0.016 (relative difference; the fit (R1.3) has a = -4.2674033395e-3) |
| P3-9 CJT | PASS | $3.32\times10^{-16}$ | $1\times10^{-12}$ | CJT: along the polar curve the normal self-energy of every state is a Dirac mass, so the quasi-particle energies are (R1.4) (largest deviation from a multiple of beta (x) 1\_2) |
| P3-11 CJT | PASS | $2.94\times10^{-13}$ | $1\times10^{-9}$ | CJT: the least squares of (R1.3) by Householder reflections give the coefficients of the normal equations (sum of the relative differences of a, b, c) |
| P3-10 CJT | PASS | $7.43\times10^{-8}$ | $1\times10^{-6}$ | CJT: the coefficient of Phi\^2 of the Taylor series at the origin, a\^Tay = -4.2966587554e-3, by Richardson from the fields 0.004, 0.008 and from 0.008, 0.016 (relative difference; the fit (R1.3) has a = -4.2674033395e-3) |
| YP3-8 | PASS | $1.33\times10^{-15}$ | $1\times10^{-13}$ | the quasi-particle energies (R1.4) for an applied pair field and a Dirac mass: -+(q\^2 + M\^2 + Phi\_a\^2)\^(1/2), eight times each (LAPACK, 20 random q, M, Phi\_a) |
| YP3-11 HS_1_1 | PASS | $9.62\times10^{-12}$ | $1\times10^{-7}$ | HS\_1\_1: the accuracy of the Landau form recomputed from data/part3\_effpot\_HS\_1\_1.csv with numpy (SVD): the minimum, depth and curvature of (R1.3), the residuals of the fits with Phi\^2 .. Phi\^(2K), K = 2 .. 6, the fits on three intervals and the sign changes = Rust (largest relative difference) |
| YP3-11 HS_0_1 | PASS | $1.48\times10^{-11}$ | $1\times10^{-7}$ | HS\_0\_1: the accuracy of the Landau form recomputed from data/part3\_effpot\_HS\_0\_1.csv with numpy (SVD): the minimum, depth and curvature of (R1.3), the residuals of the fits with Phi\^2 .. Phi\^(2K), K = 2 .. 6, the fits on three intervals and the sign changes = Rust (largest relative difference) |
| YP3-11 HS_0_0 | PASS | $5.7\times10^{-11}$ | $1\times10^{-7}$ | HS\_0\_0: the accuracy of the Landau form recomputed from data/part3\_effpot\_HS\_0\_0.csv with numpy (SVD): the minimum, depth and curvature of (R1.3), the residuals of the fits with Phi\^2 .. Phi\^(2K), K = 2 .. 6, the fits on three intervals and the sign changes = Rust (largest relative difference) |
| YP3-11 KS | PASS | $3.1\times10^{-9}$ | $1\times10^{-7}$ | KS: the accuracy of the Landau form recomputed from data/part3\_effpot\_KS.csv with numpy (SVD): the minimum, depth and curvature of (R1.3), the residuals of the fits with Phi\^2 .. Phi\^(2K), K = 2 .. 6, the fits on three intervals and the sign changes = Rust (largest relative difference) |
| YP3-11 CJT | PASS | $1.22\times10^{-9}$ | $1\times10^{-7}$ | CJT: the accuracy of the Landau form recomputed from data/part3\_effpot\_CJT.csv with numpy (SVD): the minimum, depth and curvature of (R1.3), the residuals of the fits with Phi\^2 .. Phi\^(2K), K = 2 .. 6, the fits on three intervals and the sign changes = Rust (largest relative difference) |
| YP3-9 HS_1_1 | PASS | $8.92\times10^{-18}$ | $1\times10^{-12}$ | HS\_1\_1: the normal self-energy of the states with the applied fields 0.5, 1, 1.5, 2 Phi0 is a Dirac mass (largest deviation from a multiple of beta (x) 1\_2) |
| YP3-10 HS_1_1 | PASS | $3.07\times10^{-11}$ | $1\times10^{-8}$ | HS\_1\_1: the coefficient of Phi\^2 of the Taylor series of the effective potential at the origin a\^Tay = -6.1624310943e-03 = Rust (relative; the Richardson values from 0.004, 0.008 and from 0.008, 0.016 differ by 8.8e-09) |
| YP3-9 CJT | PASS | $8.56\times10^{-17}$ | $1\times10^{-12}$ | CJT: the normal self-energy of the states with the applied fields 0.5, 1, 1.5, 2 Phi0 is a Dirac mass (largest deviation from a multiple of beta (x) 1\_2) |
| YP3-10 CJT | PASS | $2.87\times10^{-12}$ | $1\times10^{-8}$ | CJT: the coefficient of Phi\^2 of the Taylor series of the effective potential at the origin a\^Tay = -4.2966587554e-03 = Rust (relative; the Richardson values from 0.004, 0.008 and from 0.008, 0.016 differ by 7.4e-08) |

**Data files and figure** (SHA-256 checksums):

* `data/part3_effpot_HS_1_1.csv`, `data/part3_effpot_HS_0_1.csv`, `data/part3_effpot_HS_0_0.csv`, `data/part3_effpot_KS.csv`,
  `data/part3_effpot_CJT.csv` (the polar curves):\
  `adec9b41732d6e022c680889612a4f56a4610a52765d36d44a1ae76de2e2dc5c`\
  `d2a263df26a3949391ed3cf24626c1116b9fd794c00a81dcdbfb156a85b8fce7`\
  `b0439dd3ee1bd3a71d6851e4389af27a480fed96707ec6e46c06ae19be0700bc`\
  `96e7485937c714b51f0df37fa06661a407e204cde4156caef7e5274ac29f49d0`\
  `32c3fb53937001c51a23a33b2790112e0970043d8b33f08473d277be1bb9954c`
* `data/part3_values.csv` (all numbers of Part III, among them those of Table R1):\
  `348a825bc2d41da358659c46f13bd651468176b04620de44272dd45f1b753a43`
* `data/part3_landau.csv` (Table R1, Figure R1 right):\
  `f2b56e0925e8494c23b4365ab6b91747ccc34bd3c7f3774d8a3817d4685be089`
* `data/part3_landau_residuals.csv` (Figure R1 left):\
  `11a6001a3ffc86e0d2d0dadbdeb4ebcb223ff15f5634e16121130b67ee3b382f`
* `figures/fig_p4_r1_landau.pdf` (Figure R1):\
  `86e198e622c620ac54509e64dee62ce3d99fab0ea339d29d554ad701651be4c8`

**References.** L. D. Landau, On the theory of phase transitions, Zhurnal Eksperimental'noi i Teoreticheskoi Fiziki 7,
19 (1937); V. L. Ginzburg and L. D. Landau, On the theory of superconductivity, Zhurnal Eksperimental'noi i
Teoreticheskoi Fiziki 20, 1064 (1950); L. P. Gor'kov, Microscopic derivation of the Ginzburg-Landau equations in the
theory of superconductivity, Soviet Physics JETP 9, 1364 (1959); A. F. Devonshire, Theory of barium titanate, Part I,
Philosophical Magazine 40, 1040 (1949); T.-L. Ho, Spinor Bose condensates in optical traps, Physical Review Letters 81,
742 (1998); T. Ohmi and K. Machida, Bose-Einstein condensation with internal degrees of freedom in alkali atom gases,
Journal of the Physical Society of Japan 67, 1822 (1998); A. J. Leggett, A theoretical description of the new phases of
liquid $^3$He, Reviews of Modern Physics 47, 331 (1975); P. G. de Gennes, Short range order effects in the isotropic
phase of nematics and cholesterics, Molecular Crystals and Liquid Crystals 12, 193 (1971); A. S. Householder, Unitary
triangularization of a nonsymmetric matrix, Journal of the ACM 5, 339 (1958); L. F. Richardson, The approximate
arithmetical solution by finite differences of physical problems involving differential equations, Philosophical
Transactions of the Royal Society A 210, 307 (1911).

# 3. Addendum A1: the free energy of the five methods, and the size of the order parameter

**The request.** On 21 November 2026 the author asked (here in my words) to add to the analysis and the solution: the
effective potential of each of the five computed methods $X$ (HS $(1,1)$, HS $(0,1)$, HS $(0,0)$, KS and CJT), the free
energy of each of them, an analysis that determines whether the order parameter is then smaller and more physically
reasonable, and the documentation of this addendum with its provenance. The effective potential (R1.2) of each method
is defined and computed in Section 2.1. This section computes the free energy at the temperature $T$, shows that its
value at $T=0$ is the effective potential, and answers the question (Section 3.6). Sections 3.1 to 3.8 use the objects
of Section 2.1: the spinor field with the mass $m_{\rm LL}=0.2$ and the momentum cutoff $\Lambda_{\rm LL}=1$, the matrix of
Bogoliubov and de Gennes $\mathsf H_{\rm SF}$, the normal and the pair self-energies $\mathsf N_{\rm SF}$ and $\mathsf P_{\rm SF}$, the
energy per volume $\mathcal E^X_{\rm SF}$ of method $X$, the field $\vec\Phi^X_{\rm SF}$ (R1.1) and the effective potential
$\mathcal V^X_{\rm SF}$ (R1.2).

## 3.1 The free energy at the temperature $T$

**Units and conditions.** The temperature $T$ is measured in units of energy (Boltzmann's constant $k_{\rm B}=1$), in the
units of the project ($\Lambda_{\rm LL}=1$). The chemical potential $\mu_{\rm LL}$ of the fermion number is zero, as everywhere
in the solution: the states have as many particles as antiparticles.

**The thermal states.** At the temperature $T$ a uniform quasi-free state is the thermal state of its matrix
$\mathsf H_{\rm SF}$: at every momentum $\vec q_{\rm LL}$ its generalized density (the $16\times16$ matrix of the normal and the pair
correlations) is $f(\mathsf H_{\rm SF}(\vec q_{\rm LL}))$, the Fermi function $f(\varepsilon)=(e^{\varepsilon/T}+1)^{-1}$ applied to the
eigenvalues $\varepsilon_k$ ($k=1,\dots,16$) of $\mathsf H_{\rm SF}(\vec q_{\rm LL})$, with the occupations $f_k=f(\varepsilon_k)$. At $T=0$ the
Fermi function is 1 for the negative and 0 for the positive eigenvalues, and the thermal state is the ground state of
Section 2.1.

**The free energy.** The free energy per volume of method $X$ is
$$\mathcal E^X_{\rm SF}-T\,\mathcal S_{\rm SF},\qquad\mathcal S_{\rm SF}=-\frac12\int\!\frac{d^3q}{(2\pi)^3}\sum_{k=1}^{16}\big[f_k\ln f_k+(1-f_k)\ln(1-f_k)\big],\tag{A1.1}$$
with the energy per volume $\mathcal E^X_{\rm SF}$ of method $X$ (unchanged) and the entropy per volume $\mathcal S_{\rm SF}$ of the
quasi-free state; the integral runs over $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$. Every mode of the doubled space of $\mathsf H_{\rm SF}$
appears twice, at $\pm\varepsilon$ with the occupations $f$ and $1-f$, hence the factor $\tfrac12$. The self-energies are the
derivatives of $\mathcal E^X_{\rm SF}$ with respect to the correlations, as at $T=0$, and the stationary points of (A1.1) are the
self-consistent thermal states. What (A1.1) is in each method:

* **HS:** the free energy of the saddle point of the auxiliary fields of Hubbard and Stratonovich, at the temperature
  $T$;
* **CJT:** the effective action of Cornwall, Jackiw and Tomboulis at the temperature $T$, with the same truncation as at
  $T=0$, evaluated on the thermal states (the self-consistent improvement of the free energy at finite temperature of
  G. Amelino-Camelia and S.-Y. Pi uses the same method);
* **KS:** the functional of N. D. Mermin (the theory of Hohenberg, Kohn and Sham at finite temperature), with the entropy
  of the quasi-free state of Kohn and Sham and the exchange-correlation energy of the method at $T=0$. In the exact
  theory of Mermin the exchange-correlation free energy depends on the temperature; here only the states do. This is an
  approximation of the method, stated here once.

**The free energy of the field.** The free energy of the field $\vec\Phi$ in method $X$ is the smallest free energy among
the thermal states with this field, measured from the normal state at the same temperature:
$$\mathcal F^X_{\rm SF}(\vec\Phi;T)=\min\{\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}:\ \vec\Phi^X_{\rm SF}=\vec\Phi\}-\big(\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}\big)_{\rm normal}.\tag{A1.2}$$
It is computed point by point as in Section 2.1: a fixed pair self-energy $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$ (the applied
field) is put into $\mathsf H_{\rm SF}$, the normal self-energy is made self-consistent at the temperature $T$, and the thermal state
gives one point, its own field $\Phi$ and its free energy. The program computes 61 points per method and temperature,
with $\Phi_{\rm a}$ from $0$ to $2\Phi^X_{0,{\rm SF}}(T)$, at $T/T^{X,{\rm c}}_{\rm SF}=0$, $0.5$, $0.8$, $0.9$, $0.95$, $0.98$ (the critical
temperature $T^{X,{\rm c}}_{\rm SF}$ is defined in Section 3.2). The minimum of $\mathcal F^X_{\rm SF}(\vec\Phi;T)$ lies at
$\lvert\vec\Phi\rvert=\Phi^X_{0,{\rm SF}}(T)$, the **order parameter at the temperature $T$**: the modulus of the field of the
self-consistent superfluid at $T$ (both self-energies self-consistent), computed at the 13 temperatures
$T/T^{X,{\rm c}}_{\rm SF}=0,0.1,\dots,0.8,0.85,0.9,0.95,0.98$. **At $T=0$ the entropy vanishes, and
$\mathcal F^X_{\rm SF}(\vec\Phi;0)=\mathcal V^X_{\rm SF}(\vec\Phi)$: the effective potential is the free energy at zero temperature,** and
$\Phi^X_{0,{\rm SF}}(0)=\Phi^X_{0,{\rm SF}}$. *Checks P3-T1 and YP3-T1:* the thermal states at $T=0.005$ reproduce the ground states
to $10^{-12}$ (the field and the free energy); *check P3-T7:* the Landau form (R1.3) fitted to the free energy at $T=0$
has the largest residual of row 8 of Table R1.

## 3.2 The critical temperature

**The effective couplings.** In every method a uniform scalar density $n^{\rm S}_{\rm SF}$ (the expectation value of
$\bar\psi\psi$ of the spinor field per volume, summed over the isospin components and measured from the normal vacuum at
$T=0$) shifts the Dirac mass by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$, and a pair amplitude in the channel $\mathsf K^3_{\rm LL}$
produces the pair self-energy $\Phi(\mathsf K^3_{\rm LL})^\dagger$ with the strength $g^{X,{\rm P}}_{\rm SF}$: the effective scalar and
pair couplings of method $X$ (rows 1 and 2 of Table A1; the program computes them from the self-energy maps of the
contact terms of the method). For a state with the Dirac mass $M^{X,{\rm D}}_{\rm SF}$ (the normal self-energy
$(M^{X,{\rm D}}_{\rm SF}-m_{\rm LL})\beta_{\rm LL}\otimes\mathsf 1_2$) and the pair field $\Phi$ at the temperature $T$, the quasi-particle energies
are (R1.4) with $\Phi$ in place of $\Phi_{\rm a}$, and the scalar density is
$$n^{\rm S}_{\rm SF}=\int\!\frac{d^3q}{(2\pi)^3}\Big[-\frac{4M^{X,{\rm D}}_{\rm SF}\tanh(E/2T)}E+\frac{4m_{\rm LL}}{(q^2+m_{\rm LL}^2)^{1/2}}\Big]\tag{A1.3}$$
that is,
$$n^{\rm S}_{\rm SF}=-\frac2{\pi^2}\Big[M^{X,{\rm D}}_{\rm SF}I^{(1)}_{\rm SF}(E^{X,{\rm gap}}_{\rm SF},T)-m_{\rm LL}I^{(1)}_{\rm SF}(m_{\rm LL},0)\Big],$$
with $q=\lvert\vec q_{\rm LL}\rvert$, $E=(q^2+(E^{X,{\rm gap}}_{\rm SF})^2)^{1/2}$, the gap $E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+\Phi^2)^{1/2}$ (the
smallest quasi-particle energy), and the integral
$$I^{(1)}_{\rm SF}(M,T)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\tanh(E_M/2T)}{E_M}\,dq,\qquad E_M=(q^2+M^2)^{1/2},$$
whose value at $T=0$ (where $\tanh=1$) is $I^{(1)}_{\rm SF}(M,0)=\int_0^{\Lambda_{\rm LL}}q^2(q^2+M^2)^{-1/2}dq$. The factor $\tanh(E/2T)=1-2f(E)$
is the thermal factor of a pair of quasi-particles. The Dirac mass is self-consistent when
$M^{X,{\rm D}}_{\rm SF}-m_{\rm LL}=g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$.

**The normal state and its instability.** The normal state at $T$ has no pair field and the Dirac mass
$M^{X,{\rm n}}_{\rm SF}(T)$ that solves this condition with $\Phi=0$; at $T=0$ it is $m_{\rm LL}$, at $T>0$ the thermal particles and
antiparticles change the scalar density, which shifts the mass where $g^{X,{\rm S}}_{\rm SF}\ne0$ (in HS $(0,1)$ and $(0,0)$,
where $g^{X,{\rm S}}_{\rm SF}=0$, it stays $m_{\rm LL}$). A small pair field in the normal state is amplified by the factor
$$\lambda^{\rm P}_{\rm SF}(T)=\frac{g^{X,{\rm P}}_{\rm SF}\,I^{(1)}_{\rm SF}\big(M^{X,{\rm n}}_{\rm SF}(T),T\big)}{\pi^2},\tag{A1.4}$$
the eigenvalue of the static response of the normal state in the pair channel: the normal state is unstable towards a
superfluid when $\lambda^{\rm P}_{\rm SF}(T)>1$ (the criterion of Thouless). At $T=0$, $\lambda^{\rm P}_{\rm SF}>1$ in every method (row 3 of
Table A1); the thermal factor $\tanh(E/2T)<1$ lowers it at higher temperatures. The **critical temperature**
$T^{X,{\rm c}}_{\rm SF}$ solves $\lambda^{\rm P}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})=1$. *Check P3-T3:* the
program finds it by bisection, with the normal state computed at every step, to $10^{-12}$, and at $T=0$ (A1.4) gives
the eigenvalue of the full static response of the normal vacuum (row 3 of Table A1); *check YP3-T3:* the Python program
finds the same $T^{X,{\rm c}}_{\rm SF}$ to $10^{-10}$; *checks P3-T9 and YP3-T9:* in every method $\lambda^{\rm P}_{\rm SF}(T)$ does
not increase from $T=0.01$ to $T=0.6$ (60 temperatures with the normal state of the thermal engine; in Python 600
temperatures from $T=0.001$, with the normal state from its scalar self-consistency), so $T^{X,{\rm c}}_{\rm SF}$ is the
only solution there; *check P3-T4:* below $T^{X,{\rm c}}_{\rm SF}$ (at $0.99\,T^{X,{\rm c}}_{\rm SF}$) a polar start
ends in a superfluid, above it (at $1.01\,T^{X,{\rm c}}_{\rm SF}$) in the normal state. The critical temperatures are
$0.214$ (HS (0, 0)) to $0.260$ (HS (1, 1)) (row 4); KS and CJT coincide at every temperature, as they do at $T=0$.

## 3.3 The gap equation, and why the Dirac mass of the superfluid does not depend on the temperature

The polar superfluid at the temperature $T$ has the pair field $\Phi^X_{0,{\rm SF}}(T)$, and its pair self-consistency is the
**gap equation**
$$1=\frac{g^{X,{\rm P}}_{\rm SF}}{\pi^2}\,I^{(1)}_{\rm SF}\Big(\big((M^{X,{\rm D}}_{\rm SF})^2+\Phi^X_{0,{\rm SF}}(T)^2\big)^{1/2},T\Big).\tag{A1.5}$$
*Check P3-T2:* the self-consistent superfluids satisfy it to $10^{-12}$ at every temperature of the table; *check
YP3-T4:* the Python program solves (A1.5) alone for HS $(0,0)$ (where $M^{X,{\rm D}}_{\rm SF}=m_{\rm LL}$) and finds the same order
parameter to $10^{-9}$. The pair coupling alone fixes the gap $E^{X,{\rm gap}}_{\rm SF}$ at every temperature. The scalar density
(A1.3) of the superfluid contains the same integral, which (A1.5) fixes to $\pi^2/g^{X,{\rm P}}_{\rm SF}$:
$n^{\rm S}_{\rm SF}=-2M^{X,{\rm D}}_{\rm SF}/g^{X,{\rm P}}_{\rm SF}+2m_{\rm LL}I^{(1)}_{\rm SF}(m_{\rm LL},0)/\pi^2$. The scalar self-consistency
$M^{X,{\rm D}}_{\rm SF}-m_{\rm LL}=g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$ therefore contains no temperature, and it has the same solution at every
temperature below $T^{X,{\rm c}}_{\rm SF}$: **the Dirac mass of the superfluid does not depend on the temperature** (*check
P3-T8*, to $10^{-12}$; row 8 of Table A1), and the order parameter follows from (A1.5) alone. In the normal state the
integral is $I^{(1)}_{\rm SF}(M^{X,{\rm n}}_{\rm SF}(T),T)$, which nothing fixes, and $M^{X,{\rm n}}_{\rm SF}(T)$ changes with the temperature.

## 3.4 The radius of convergence at the temperature $T$

Section 2.4 found that at $T=0$ the series of $\mathcal V^X_{\rm SF}$ in powers of $\lvert\vec\Phi\rvert^2$ converges only within a
radius of the order of the Dirac mass, and that the superfluid lies outside it. At $T>0$ the free energy is built from
integrals over the momenta of functions of the quasi-particle energies $E=(q^2+M^2+\Phi_{\rm a}^2)^{1/2}$ (for a Dirac mass $M$
and an applied pair field $\Phi_{\rm a}$) and of their Fermi functions: the terms $-T\ln(2\cosh(E/2T))$, one for each pair of
eigenvalues $\pm E$ of $\mathsf H_{\rm SF}$ (at the Fermi occupation $f=f(E)$ the energy $E(f-\tfrac12)$ of the pair and its term
$T[f\ln f+(1-f)\ln(1-f)]$ in $-T\mathcal S_{\rm SF}$ add up to $-T\ln(2\cosh(E/2T))$), and the densities with the factor
$\tanh(E/2T)/E$. With the **Matsubara frequencies** $\omega_n=(2n+1)\pi T$ ($n=0,1,2,\dots$) they become sums over $n$,
$$\ln\big(2\cosh(E/2T)\big)=\ln2+\sum_{n=0}^\infty\ln\Big(1+\frac{E^2}{\omega_n^2}\Big),\qquad\frac{\tanh(E/2T)}E=4T\sum_{n=0}^\infty\frac1{\omega_n^2+E^2},$$
of functions of $\omega_n^2+q^2+M^2+\Phi_{\rm a}^2$. Integrated over the momenta, each of them is singular where
$\omega_n^2+M^2+\Phi_{\rm a}^2=0$ (at the end $q=0$ of the integral). The nearest of these points to the origin, for the Dirac
mass of the normal state held fixed, is $\Phi_{\rm a}^2=-(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2)$; at $T=0$ the frequencies become
continuous, and it is the point $\Phi_{\rm a}^2=-m_{\rm LL}^2$ of Section 2.4. The series of $\mathcal F^X_{\rm SF}$ in powers of
$\lvert\vec\Phi\rvert^2$ about the normal state therefore converges within a radius of the order of
$$r^X_{\rm SF}(T)=\big(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2\big)^{1/2},\tag{A1.6}$$
not exactly equal to it: the Dirac mass changes with the applied field where $g^{X,{\rm S}}_{\rm SF}\ne0$, and the field $\Phi$ of the
states is not the applied field $\Phi_{\rm a}$. $r^X_{\rm SF}(0)=m_{\rm LL}$, and $r^X_{\rm SF}$ grows with the temperature to
$0.688$ (HS (0, 0)) to $0.828$ (HS (1, 1)) at $0.98\,T^{X,{\rm c}}_{\rm SF}$. Because (A1.6) is an estimate, the answer of Section 3.6 rests also on two
direct measures that do not use it: the residual of the Landau form, and the error of the order parameter of Landau's
theory.

**Landau's theory and its order parameter.** Landau's theory of a continuous transition expands the free energy in
powers of the order parameter at the symmetric state, $\mathcal F^X_{\rm SF}(\Phi;T)=a^{X,{\rm Tay}}_{\rm SF}(T)\Phi^2+b^{X,{\rm Tay}}_{\rm SF}(T)\Phi^4+\dots$
along the polar fields, and predicts the order parameter $(-a/2b)^{1/2}$, the minimum of the first two terms, with
$a=a^{X,{\rm Tay}}_{\rm SF}(T)$ and $b=b^{X,{\rm Tay}}_{\rm SF}(T)$, the Taylor coefficients of $\Phi^2$ and $\Phi^4$ at $\Phi=0$. The program
computes them from four small applied fields, $\Phi_{\rm a}=0.01$, $0.02$, $0.03$, $0.04$: the quotient $\mathcal F^X_{\rm SF}/\Phi^2$ is
interpolated exactly by a cubic polynomial in $\Phi^2$, whose value and slope at $\Phi^2=0$ are $a$ and $b$. If the order
parameter lies inside the radius of convergence and is small, $(-a/2b)^{1/2}$ approaches it, with an error of relative order
$1-T/T^{X,{\rm c}}_{\rm SF}$ (*check P3-T5*).

## 3.5 The numbers

| quantity | HS (1, 1) | HS (0, 1) | HS (0, 0) | KS | CJT |
|:-----------------------------|-------:|-------:|-------:|-------:|-------:|
| effective pair coupling $g^{X,{\rm P}}_{\rm SF}$ | $25.20$ | $24.50$ | $23.70$ | $23.70$ | $23.70$ |
| effective scalar coupling $g^{X,{\rm S}}_{\rm SF}$ | $2.80$ | $0$ | $0$ | $-7.40$ | $-7.40$ |
| $\lambda^{\rm P}_{\rm SF}(0)$ of the normal vacuum | $1.18384$ | $1.15096$ | $1.11338$ | $1.11338$ | $1.11338$ |
| critical temperature $T^{X,{\rm c}}_{\rm SF}$ | $0.260467$ | $0.240465$ | $0.213799$ | $0.222119$ | $0.222119$ |
| $\Phi^X_{0,{\rm SF}}(0)$ | $0.403064$ | $0.362009$ | $0.306989$ | $0.328486$ | $0.328486$ |
| $\Phi^X_{0,{\rm SF}}(0)/T^{X,{\rm c}}_{\rm SF}$ | $1.547$ | $1.505$ | $1.436$ | $1.479$ | $1.479$ |
| $E^{X,{\rm gap}}_{\rm SF}(0)/T^{X,{\rm c}}_{\rm SF}$ | $1.739$ | $1.720$ | $1.714$ | $1.650$ | $1.650$ |
| Dirac mass of the superfluid (every $T<T^{X,{\rm c}}_{\rm SF}$) | $0.206685$ | $0.200000$ | $0.200000$ | $0.162293$ | $0.162293$ |
| $\Phi^X_{0,{\rm SF}}(T)<r^X_{\rm SF}(T)$ above $T/T^{X,{\rm c}}_{\rm SF}=$ | $0.42$ | $0.40$ | $0.35$ | $0.37$ | $0.37$ |
| $\Phi^X_{0,{\rm SF}}/r^X_{\rm SF}$ at $T/T^{X,{\rm c}}_{\rm SF}=0$ | $2.02$ | $1.81$ | $1.53$ | $1.64$ | $1.64$ |
| $\Phi^X_{0,{\rm SF}}/r^X_{\rm SF}$ at $T/T^{X,{\rm c}}_{\rm SF}=0.5$ | $0.860$ | $0.824$ | $0.765$ | $0.800$ | $0.800$ |
| $\Phi^X_{0,{\rm SF}}/r^X_{\rm SF}$ at $T/T^{X,{\rm c}}_{\rm SF}=0.9$ | $0.295$ | $0.287$ | $0.273$ | $0.284$ | $0.284$ |
| $\Phi^X_{0,{\rm SF}}/r^X_{\rm SF}$ at $T/T^{X,{\rm c}}_{\rm SF}=0.98$ | $0.127$ | $0.124$ | $0.118$ | $0.123$ | $0.123$ |
| residual of (R1.3) / depth at $0$ (per cent) | $0.70$ | $0.47$ | $0.27$ | $0.28$ | $0.28$ |
| residual of (R1.3) / depth at $0.5$ (per cent) | $0.56$ | $0.38$ | $0.19$ | $0.23$ | $0.23$ |
| residual of (R1.3) / depth at $0.9$ (per cent) | $0.040$ | $0.030$ | $0.019$ | $0.024$ | $0.024$ |
| residual of (R1.3) / depth at $0.98$ (per cent) | $0.0017$ | $0.0013$ | $0.00078$ | $0.0010$ | $0.0010$ |
| residual of the quartic form / depth at $0$ (per cent) | $5.9$ | $4.4$ | $2.8$ | $5.0$ | $5.0$ |
| residual of the quartic form / depth at $0.9$ (per cent) | $1.8$ | $1.6$ | $1.3$ | $1.6$ | $1.6$ |
| residual of the quartic form / depth at $0.98$ (per cent) | $0.37$ | $0.32$ | $0.26$ | $0.31$ | $0.31$ |
| $(-a/2b)^{1/2}/\Phi^X_{0,{\rm SF}}-1$ at $0$ (per cent) | $3.8$ | $3.1$ | $1.9$ | $7.0$ | $7.0$ |
| $(-a/2b)^{1/2}/\Phi^X_{0,{\rm SF}}-1$ at $0.9$ (per cent) | $2.7$ | $2.3$ | $1.8$ | $2.3$ | $2.3$ |
| $(-a/2b)^{1/2}/\Phi^X_{0,{\rm SF}}-1$ at $0.98$ (per cent) | $0.56$ | $0.49$ | $0.38$ | $0.47$ | $0.47$ |
| occupation of the states at the cutoff at $0.98\,T^{X,{\rm c}}_{\rm SF}$ (per cent) | $1.8$ | $1.3$ | $0.76$ | $0.94$ | $0.94$ |

**Table A1.** The free energy at finite temperature, for the five methods. Rows 1 to 3: the effective couplings and the
eigenvalue (A1.4) of the normal vacuum at $T=0$; rows 4 to 8: the critical temperature, the order parameter
$\Phi^X_{0,{\rm SF}}(0)=\Phi^X_{0,{\rm SF}}$ at $T=0$, its ratio to $T^{X,{\rm c}}_{\rm SF}$ and that of the gap $E^{X,{\rm gap}}_{\rm SF}$ at $T=0$, and the Dirac
mass of the superfluid; row 9: the reduced temperature $T/T^{X,{\rm c}}_{\rm SF}$ above which the order parameter lies inside the
radius (A1.6) (interpolated linearly in the table of temperatures); rows 10 to 13: $\Phi^X_{0,{\rm SF}}(T)/r^X_{\rm SF}(T)$ at four
temperatures; rows 14 to 20: the largest residual of the Landau form (R1.3) and of the quartic form $a\Phi^2+b\Phi^4$,
fitted by least squares to $\mathcal F^X_{\rm SF}(\Phi;T)$ on $[0,1.4\,\Phi^X_{0,{\rm SF}}(T)]$, relative to the depth
$\lvert\mathcal F^X_{\rm SF}(\Phi^X_{0,{\rm SF}}(T);T)\rvert$; rows 21 to 23: the relative error of the order parameter $(-a/2b)^{1/2}$ of
Landau's theory (Section 3.4); row 24: the occupation $f(E)$ of the states at the cutoff, $E=(\Lambda_{\rm LL}^2+M^{X,{\rm n}}_{\rm SF}(T)^2)^{1/2}$,
at $0.98\,T^{X,{\rm c}}_{\rm SF}$. Data: `data/part3_thermal.csv`,
`data/part3_thermal_landau.csv`, `data/part3_values.csv`.

![](../figures/fig_p4_a1_thermal.png){width=100%}

**Figure A1.** The free energy at finite temperature. (a) The order parameter $\Phi^X_{0,{\rm SF}}(T)$ against
$T/T^{X,{\rm c}}_{\rm SF}$ (KS and CJT coincide; one curve for both), with the radius $r^X_{\rm SF}(T)$ of (A1.6) thin and dashed in the
same colour. (b) The ratio $\Phi^X_{0,{\rm SF}}(T)/r^X_{\rm SF}(T)$: below 1 the order parameter lies inside the radius of
convergence. (c) The free energy of CJT normalized to its minimum,
$\mathcal F^{\rm CJT}_{\rm SF}(\Phi;T)/\lvert\mathcal F^{\rm CJT}_{\rm SF}(\Phi^{\rm CJT}_{0,{\rm SF}}(T);T)\rvert$ against $\Phi/\Phi^{\rm CJT}_{0,{\rm SF}}(T)$, at six
temperatures, with the curve $x^4-2x^2$ of Landau's quartic theory (dashed; $x$ is the field in units of its minimum).
(d) The largest residual of the Landau form (R1.3) (solid) and of the quartic form (dotted), relative to the depth, and
the error of Landau's order parameter $(-a/2b)^{1/2}$ (dashed), in per cent. Data: `data/part3_thermal.csv`,
`data/part3_thermal_curves.csv`, `data/part3_thermal_landau.csv`.

## 3.6 The answer: is the order parameter smaller and more physically reasonable?

Yes, near the critical temperature, and only there.

1. **It is smaller.** $\Phi^X_{0,{\rm SF}}(T)$ keeps its value of $T=0$ up to about $0.3\,T^{X,{\rm c}}_{\rm SF}$ (the gap at $T=0$,
   $0.366$ (HS (0, 0)) to $0.453$ (HS (1, 1)), is much larger than these temperatures, and thermal quasi-particles are exponentially rare; at
   $0.3\,T^{X,{\rm c}}_{\rm SF}$ the order parameter has changed by $0.12$ per cent (HS (0, 0)) to $0.13$ per cent (KS, CJT)), then falls and vanishes continuously at
   $T^{X,{\rm c}}_{\rm SF}$, like $(1-T/T^{X,{\rm c}}_{\rm SF})^{1/2}$ (Figure A1a): at $0.98\,T^{X,{\rm c}}_{\rm SF}$ it is $0.0812$ (HS (0, 0)) to $0.105$ (HS (1, 1)),
   against $0.307$ (HS (0, 0)) to $0.403$ (HS (1, 1)) at $T=0$.
2. **It lies inside the radius of convergence.** At $T=0$ the order parameter is $1.53$ (HS (0, 0)) to $2.02$ (HS (1, 1)) times the radius
   (A1.6), which is then the mass $m_{\rm LL}$; above $0.35$ (HS (0, 0)) to $0.42$ (HS (1, 1)) times $T^{X,{\rm c}}_{\rm SF}$ it is smaller than the radius (row 9),
   and at $0.98\,T^{X,{\rm c}}_{\rm SF}$ the ratio is $0.118$ (HS (0, 0)) to $0.127$ (HS (1, 1)) (Figure A1b). There the series of the free energy in powers of
   $\lvert\vec\Phi\rvert^2$ converges at the minimum, and the Landau coefficients are the Taylor coefficients of the free energy:
   the regime of Landau's theory, which Section 2.4 found absent at $T=0$.
3. **The Landau form becomes exact.** The largest residual of (R1.3), relative to the depth, is $0.27$ per cent (HS (0, 0)) to $0.70$ per cent (HS (1, 1)) at
   $T=0$ and $0.00078$ per cent (HS (0, 0)) to $0.0017$ per cent (HS (1, 1)) at $0.98\,T^{X,{\rm c}}_{\rm SF}$. That of the quartic form (the form of the Higgs potential of the
   Standard Model and of the theory of Ginzburg and Landau) is $2.8$ per cent (HS (0, 0)) to $5.9$ per cent (HS (1, 1)) at $T=0$ and $0.26$ per cent (HS (0, 0)) to $0.37$ per cent (HS (1, 1)) at
   $0.98\,T^{X,{\rm c}}_{\rm SF}$; near $T^{X,{\rm c}}_{\rm SF}$ it falls in proportion to $1-T/T^{X,{\rm c}}_{\rm SF}$ (*check P3-T6*). The normalized free energy approaches the curve
   $x^4-2x^2$ (Figure A1c).
4. **Landau's theory predicts the order parameter.** $(-a/2b)^{1/2}$ errs by $1.9$ per cent (HS (0, 0)) to $7.0$ per cent (KS, CJT) at $T=0$ and by $0.38$ per cent (HS (0, 0)) to $0.56$ per cent (HS (1, 1)) at
   $0.98\,T^{X,{\rm c}}_{\rm SF}$; near $T^{X,{\rm c}}_{\rm SF}$ the error is about $0.19$ (HS (0, 0)) to $0.28$ (HS (1, 1)) times $1-T/T^{X,{\rm c}}_{\rm SF}$ (*check P3-T5*), as
   Landau's theory requires.
5. **The scales.** Near $T^{X,{\rm c}}_{\rm SF}$ the order parameter is small also against the Dirac mass and the cutoff. The
   critical temperatures themselves, however, are about a fifth to a quarter of the cutoff ($0.214$ (HS (0, 0)) to $0.260$ (HS (1, 1))); the states at
   the cutoff are occupied by $0.76$ per cent (HS (0, 0)) to $1.8$ per cent (HS (1, 1)) at $0.98\,T^{X,{\rm c}}_{\rm SF}$ (row 24), so the thermal physics reaches the cutoff only
   weakly. The ratio $\Phi^X_{0,{\rm SF}}(0)/T^{X,{\rm c}}_{\rm SF}$ is $1.44$ (HS (0, 0)) to $1.55$ (HS (1, 1)), and that of the gap,
   $E^{X,{\rm gap}}_{\rm SF}(0)/T^{X,{\rm c}}_{\rm SF}$, is $1.65$ (KS, CJT) to $1.74$ (HS (1, 1)); the theory of Bardeen, Cooper and Schrieffer gives the ratio
   $\Delta(0)/T_{\rm c}=\pi e^{-\gamma_{\rm E}}=1.764$ of the gap $\Delta(0)$ at $T=0$ to the critical temperature $T_{\rm c}$ for weak pairing
   at a Fermi surface ($\gamma_{\rm E}=0.5772$ is Euler's constant). The ratios are of the same size but not equal: here there
   is no Fermi surface ($\mu_{\rm LL}=0$), the pairing needs a coupling above a critical one, and the gap contains the Dirac
   mass.

**In one sentence.** The order parameter of the mean-field free energy is smaller, and is described exactly by
Landau's theory, near the critical temperature; at low temperatures (below about $0.3\,T^{X,{\rm c}}_{\rm SF}$) it is the order
parameter of $T=0$, outside the radius of convergence. The free energy of these methods is a mean-field free energy; what
that implies near $T^{X,{\rm c}}_{\rm SF}$ is remark R2 (Section 4).

## 3.7 Why it matters

1. **A derived Landau theory.** Near $T^{X,{\rm c}}_{\rm SF}$ the effective Lagrangian of the field may use the quartic potential
   $a\lvert\vec\Phi\rvert^2+b\lvert\vec\Phi\rvert^4$ with the Taylor coefficients of Section 3.4: there it is a derived approximation
   with a known error (items 3 and 4), not a fit as at $T=0$ (Section 2.3). At $T=0$ no truncated series of the potential
   may replace the computed function (Section 2.5).
2. **New predictions of each method.** The critical temperature, the temperature dependence of the order parameter and
   the temperature-independent Dirac mass of the superfluid are results of each method that the effective potential at
   $T=0$ does not contain. The methods differ in $T^{X,{\rm c}}_{\rm SF}$ by about a fifth (row 4), in the same order as their pair
   couplings (row 1), which is the difference that decides their order parameters at $T=0$ as well.
3. **The comparison with the Higgs sector.** The quartic form misses the effective potential at $T=0$ by
   $2.8$ per cent (HS (0, 0)) to $5.9$ per cent (HS (1, 1)) of the depth; near $T^{X,{\rm c}}_{\rm SF}$ the free energy of the superfluid has the quartic form of the Higgs
   potential to $0.26$ per cent (HS (0, 0)) to $0.37$ per cent (HS (1, 1)) at $0.98\,T^{X,{\rm c}}_{\rm SF}$. The resemblance to the Standard Model is close only there.
4. **What it does not change.** The results of the solution at $T=0$ (the ground states, the masses, the effective
   Lagrangians and the dynamics) are unchanged: they belong to $T=0$, where the free energy is the effective potential.
5. **Its limits.** The statements of Section 3.6 concern the mean-field free energy; the fluctuations that it omits
   matter near $T^{X,{\rm c}}_{\rm SF}$ (remark R2, Section 4), and $T^{X,{\rm c}}_{\rm SF}$ is a sizable fraction of the cutoff (item 5 of
   Section 3.6).

## 3.8 Provenance

**Programs.** The Rust program `sf_part3` (crate `rust/sfpart3`, module `thermal`) computes the thermal states, the
normal states, the critical temperatures by bisection of (A1.4), the order parameter at the 13 temperatures, the
free-energy curves (A1.2) at six temperatures, the Taylor coefficients, the least-squares fits of the Landau form, and
the checks P3-T1 to P3-T9; the thermal states are functions of the library `sfcore` (module `uniform`: `fermi`,
`mode_entropy`, `states_t`, `solve_t`, `fixed_pair_state_t`). The Python program `python/sf_part3.py` computes the thermal
states of HS $(1,1)$ and CJT with its own engine (the limit $T\to0$, the critical temperatures, the order parameter and the
free energy and the Dirac mass at $0.5$ and $0.9\,T^{X,{\rm c}}_{\rm SF}$), solves the gap equation (A1.5) of HS $(0,0)$ alone,
computes the Taylor coefficients of the free energy of CJT at $0.9\,T^{\rm CJT,c}_{\rm SF}$ and the order parameter of
Landau's theory, recomputes from the data file of the free-energy curves the residuals of their Landau forms, and shows
for the five methods that $\lambda^{\rm P}_{\rm SF}(T)$ does not increase with the temperature (checks YP3-T1 to YP3-T9;
each has the identifier of the Rust check of the same statement with the prefix Y). The notebook `superfluid.nb`
repeats the computation in its Section III.T with the engine of its Part II (the checks WP3-T1 to WP3-T9). `python/sf_part3_figures.py` draws Figure A1, and `code/scripts/write_part3_fragments.py` writes Table A1, the
table of the checks below and every number of Section 3 from the data files.

**Commands** (from the folder `Revision_HS_CJT_and_Kohn-Sham`; the Rust program takes about fifteen minutes, the Python
program about one and a half):

    (cd rust && cargo build --release && ./target/release/sf_part3)
    (cd python && python3 sf_part3.py)
    python3 python/sf_part3_figures.py
    python3 code/scripts/write_part3_fragments.py
    python3 code/scripts/assemble_docs.py docs_src/P4_remarks.src.md
    bash code/scripts/build_docs.sh provenance/P4_remarks.md

**The checks** (from the reports `data/hck_part3_checks.txt` and `data/hck_part3_checks_python.txt`; a check shown with
"--" is the check of a fact, which passes when the statement holds):

| check | result | deviation | tolerance | statement |
|:---|:---|---:|---:|:-----------|
| P3-T1 HS_1_1 | PASS | $2.36\times10^{-14}$ | $1\times10^{-12}$ | HS\_1\_1: the thermal engine at T = 0.005 reproduces the ground state of T = 0 (field and free energy; /change of Phi0/ + relative change of the energy) |
| P3-T3 HS_1_1 | PASS | $4.41\times10^{-14}$ | $1\times10^{-12}$ | HS\_1\_1: the critical temperature T\_c = 0.2604672394 solves lambda\^P(T\_c) = 1 (/lambda\^P(T\_c) - 1/), and lambda\^P(0) = 1.1838428143 equals the eigenvalue of the static pair response of the normal vacuum computed by sf\_part2 (difference) |
| P3-T9 HS_1_1 | PASS | $0$ | $1\times10^{-12}$ | HS\_1\_1: lambda\^P(T) of the normal state does not increase on 60 temperatures from 0.01 to 0.6 (from 1.183843 to 0.620857; the largest rise), so T\_c is the only solution of lambda\^P(T) = 1 there |
| P3-T2 HS_1_1 | PASS | $9.36\times10^{-13}$ | $1\times10^{-9}$ | HS\_1\_1: the gap equation g\^P I1(M\^2 + Phi0\^2, T)/pi\^2 = 1 holds for the superfluid at every temperature of the table (largest /residual/; g\^P = 25.200000) |
| P3-T8 HS_1_1 | PASS | $3.62\times10^{-14}$ | $1\times10^{-10}$ | HS\_1\_1: the Dirac mass of the superfluid is the same at every temperature below T\_c (0.2066851932; largest change) |
| P3-T5 HS_1_1 | PASS | $0.281$ | $0.5$ | HS\_1\_1: near T\_c the order parameter of Landau's theory (-a/2b)\^(1/2) approaches Phi0(T) with an error of relative order 1 - T/T\_c (largest /(-a/2b)\^(1/2)/Phi0 - 1/ / (1 - T/T\_c) at T/T\_c = 0.9, 0.95, 0.98; at T/T\_c = 0.98 the error is 5.63e-3) |
| P3-T4 HS_1_1 | PASS | -- | -- | HS\_1\_1: a polar start ends in a superfluid at 0.99 T\_c (Phi0 = 7.489e-2) and in the normal state at 1.01 T\_c (Phi0 = 2.8e-11) |
| P3-T7 HS_1_1 | PASS | $5.58\times10^{-12}$ | $1\times10^{-8}$ | HS\_1\_1: at T = 0 the free energy is the effective potential: the largest residual of its Landau form, 7.035495e-3 of the depth, is that of row 8 of Table R1 (relative difference) |
| P3-T6 HS_1_1 | PASS | -- | -- | HS\_1\_1: the Landau form becomes exact as T -> T\_c: the residual of the quartic form falls (1.83e-2, 8.81e-3, 3.73e-3 of the depth at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.7e-5 of the depth at 0.98 T\_c |
| P3-T1 HS_0_1 | PASS | $9.23\times10^{-14}$ | $1\times10^{-12}$ | HS\_0\_1: the thermal engine at T = 0.005 reproduces the ground state of T = 0 (field and free energy; /change of Phi0/ + relative change of the energy) |
| P3-T3 HS_0_1 | PASS | $4.03\times10^{-13}$ | $1\times10^{-12}$ | HS\_0\_1: the critical temperature T\_c = 0.2404654563 solves lambda\^P(T\_c) = 1 (/lambda\^P(T\_c) - 1/), and lambda\^P(0) = 1.1509582917 equals the eigenvalue of the static pair response of the normal vacuum computed by sf\_part2 (difference) |
| P3-T9 HS_0_1 | PASS | $0$ | $1\times10^{-12}$ | HS\_0\_1: lambda\^P(T) of the normal state does not increase on 60 temperatures from 0.01 to 0.6 (from 1.150958 to 0.604690; the largest rise), so T\_c is the only solution of lambda\^P(T) = 1 there |
| P3-T2 HS_0_1 | PASS | $1.03\times10^{-12}$ | $1\times10^{-9}$ | HS\_0\_1: the gap equation g\^P I1(M\^2 + Phi0\^2, T)/pi\^2 = 1 holds for the superfluid at every temperature of the table (largest /residual/; g\^P = 24.500000) |
| P3-T8 HS_0_1 | PASS | $0$ | $1\times10^{-10}$ | HS\_0\_1: the Dirac mass of the superfluid is the same at every temperature below T\_c (0.2000000000; largest change) |
| P3-T5 HS_0_1 | PASS | $0.244$ | $0.5$ | HS\_0\_1: near T\_c the order parameter of Landau's theory (-a/2b)\^(1/2) approaches Phi0(T) with an error of relative order 1 - T/T\_c (largest /(-a/2b)\^(1/2)/Phi0 - 1/ / (1 - T/T\_c) at T/T\_c = 0.9, 0.95, 0.98; at T/T\_c = 0.98 the error is 4.87e-3) |
| P3-T4 HS_0_1 | PASS | -- | -- | HS\_0\_1: a polar start ends in a superfluid at 0.99 T\_c (Phi0 = 6.757e-2) and in the normal state at 1.01 T\_c (Phi0 = 3.2e-11) |
| P3-T7 HS_0_1 | PASS | $5.9\times10^{-13}$ | $1\times10^{-8}$ | HS\_0\_1: at T = 0 the free energy is the effective potential: the largest residual of its Landau form, 4.693236e-3 of the depth, is that of row 8 of Table R1 (relative difference) |
| P3-T6 HS_0_1 | PASS | -- | -- | HS\_0\_1: the Landau form becomes exact as T -> T\_c: the residual of the quartic form falls (1.60e-2, 7.67e-3, 3.24e-3 of the depth at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.3e-5 of the depth at 0.98 T\_c |
| P3-T1 HS_0_0 | PASS | $1.04\times10^{-13}$ | $1\times10^{-12}$ | HS\_0\_0: the thermal engine at T = 0.005 reproduces the ground state of T = 0 (field and free energy; /change of Phi0/ + relative change of the energy) |
| P3-T3 HS_0_0 | PASS | $1.98\times10^{-13}$ | $1\times10^{-12}$ | HS\_0\_0: the critical temperature T\_c = 0.2137990984 solves lambda\^P(T\_c) = 1 (/lambda\^P(T\_c) - 1/), and lambda\^P(0) = 1.1133759801 equals the eigenvalue of the static pair response of the normal vacuum computed by sf\_part2 (difference) |
| P3-T9 HS_0_0 | PASS | $0$ | $1\times10^{-12}$ | HS\_0\_0: lambda\^P(T) of the normal state does not increase on 60 temperatures from 0.01 to 0.6 (from 1.113376 to 0.584946; the largest rise), so T\_c is the only solution of lambda\^P(T) = 1 there |
| P3-T2 HS_0_0 | PASS | $1.21\times10^{-12}$ | $1\times10^{-9}$ | HS\_0\_0: the gap equation g\^P I1(M\^2 + Phi0\^2, T)/pi\^2 = 1 holds for the superfluid at every temperature of the table (largest /residual/; g\^P = 23.700000) |
| P3-T8 HS_0_0 | PASS | $0$ | $1\times10^{-10}$ | HS\_0\_0: the Dirac mass of the superfluid is the same at every temperature below T\_c (0.2000000000; largest change) |
| P3-T5 HS_0_0 | PASS | $0.192$ | $0.5$ | HS\_0\_0: near T\_c the order parameter of Landau's theory (-a/2b)\^(1/2) approaches Phi0(T) with an error of relative order 1 - T/T\_c (largest /(-a/2b)\^(1/2)/Phi0 - 1/ / (1 - T/T\_c) at T/T\_c = 0.9, 0.95, 0.98; at T/T\_c = 0.98 the error is 3.83e-3) |
| P3-T4 HS_0_0 | PASS | -- | -- | HS\_0\_0: a polar start ends in a superfluid at 0.99 T\_c (Phi0 = 5.773e-2) and in the normal state at 1.01 T\_c (Phi0 = 3.9e-11) |
| P3-T7 HS_0_0 | PASS | $1.84\times10^{-11}$ | $1\times10^{-8}$ | HS\_0\_0: at T = 0 the free energy is the effective potential: the largest residual of its Landau form, 2.690453e-3 of the depth, is that of row 8 of Table R1 (relative difference) |
| P3-T6 HS_0_0 | PASS | -- | -- | HS\_0\_0: the Landau form becomes exact as T -> T\_c: the residual of the quartic form falls (1.27e-2, 6.06e-3, 2.56e-3 of the depth at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 7.8e-6 of the depth at 0.98 T\_c |
| P3-T1 KS | PASS | $6.27\times10^{-14}$ | $1\times10^{-12}$ | KS: the thermal engine at T = 0.005 reproduces the ground state of T = 0 (field and free energy; /change of Phi0/ + relative change of the energy) |
| P3-T3 KS | PASS | $1.99\times10^{-13}$ | $1\times10^{-12}$ | KS: the critical temperature T\_c = 0.2221187371 solves lambda\^P(T\_c) = 1 (/lambda\^P(T\_c) - 1/), and lambda\^P(0) = 1.1133759801 equals the eigenvalue of the static pair response of the normal vacuum computed by sf\_part2 (difference) |
| P3-T9 KS | PASS | $0$ | $1\times10^{-12}$ | KS: lambda\^P(T) of the normal state does not increase on 60 temperatures from 0.01 to 0.6 (from 1.113376 to 0.588434; the largest rise), so T\_c is the only solution of lambda\^P(T) = 1 there |
| P3-T2 KS | PASS | $1.14\times10^{-12}$ | $1\times10^{-9}$ | KS: the gap equation g\^P I1(M\^2 + Phi0\^2, T)/pi\^2 = 1 holds for the superfluid at every temperature of the table (largest /residual/; g\^P = 23.700000) |
| P3-T8 KS | PASS | $3.27\times10^{-13}$ | $1\times10^{-10}$ | KS: the Dirac mass of the superfluid is the same at every temperature below T\_c (0.1622929325; largest change) |
| P3-T5 KS | PASS | $0.235$ | $0.5$ | KS: near T\_c the order parameter of Landau's theory (-a/2b)\^(1/2) approaches Phi0(T) with an error of relative order 1 - T/T\_c (largest /(-a/2b)\^(1/2)/Phi0 - 1/ / (1 - T/T\_c) at T/T\_c = 0.9, 0.95, 0.98; at T/T\_c = 0.98 the error is 4.70e-3) |
| P3-T4 KS | PASS | -- | -- | KS: a polar start ends in a superfluid at 0.99 T\_c (Phi0 = 6.128e-2) and in the normal state at 1.01 T\_c (Phi0 = 3.8e-11) |
| P3-T7 KS | PASS | $3.46\times10^{-11}$ | $1\times10^{-8}$ | KS: at T = 0 the free energy is the effective potential: the largest residual of its Landau form, 2.846809e-3 of the depth, is that of row 8 of Table R1 (relative difference) |
| P3-T6 KS | PASS | -- | -- | KS: the Landau form becomes exact as T -> T\_c: the residual of the quartic form falls (1.58e-2, 7.47e-3, 3.13e-3 of the depth at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.0e-5 of the depth at 0.98 T\_c |
| P3-T1 CJT | PASS | $8.08\times10^{-14}$ | $1\times10^{-12}$ | CJT: the thermal engine at T = 0.005 reproduces the ground state of T = 0 (field and free energy; /change of Phi0/ + relative change of the energy) |
| P3-T3 CJT | PASS | $1.98\times10^{-13}$ | $1\times10^{-12}$ | CJT: the critical temperature T\_c = 0.2221187371 solves lambda\^P(T\_c) = 1 (/lambda\^P(T\_c) - 1/), and lambda\^P(0) = 1.1133759801 equals the eigenvalue of the static pair response of the normal vacuum computed by sf\_part2 (difference) |
| P3-T9 CJT | PASS | $0$ | $1\times10^{-12}$ | CJT: lambda\^P(T) of the normal state does not increase on 60 temperatures from 0.01 to 0.6 (from 1.113376 to 0.588434; the largest rise), so T\_c is the only solution of lambda\^P(T) = 1 there |
| P3-T2 CJT | PASS | $1.14\times10^{-12}$ | $1\times10^{-9}$ | CJT: the gap equation g\^P I1(M\^2 + Phi0\^2, T)/pi\^2 = 1 holds for the superfluid at every temperature of the table (largest /residual/; g\^P = 23.700000) |
| P3-T8 CJT | PASS | $3.27\times10^{-13}$ | $1\times10^{-10}$ | CJT: the Dirac mass of the superfluid is the same at every temperature below T\_c (0.1622929325; largest change) |
| P3-T5 CJT | PASS | $0.235$ | $0.5$ | CJT: near T\_c the order parameter of Landau's theory (-a/2b)\^(1/2) approaches Phi0(T) with an error of relative order 1 - T/T\_c (largest /(-a/2b)\^(1/2)/Phi0 - 1/ / (1 - T/T\_c) at T/T\_c = 0.9, 0.95, 0.98; at T/T\_c = 0.98 the error is 4.70e-3) |
| P3-T4 CJT | PASS | -- | -- | CJT: a polar start ends in a superfluid at 0.99 T\_c (Phi0 = 6.128e-2) and in the normal state at 1.01 T\_c (Phi0 = 3.8e-11) |
| P3-T7 CJT | PASS | $4.6\times10^{-11}$ | $1\times10^{-8}$ | CJT: at T = 0 the free energy is the effective potential: the largest residual of its Landau form, 2.846809e-3 of the depth, is that of row 8 of Table R1 (relative difference) |
| P3-T6 CJT | PASS | -- | -- | CJT: the Landau form becomes exact as T -> T\_c: the residual of the quartic form falls (1.58e-2, 7.47e-3, 3.13e-3 of the depth at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.0e-5 of the depth at 0.98 T\_c |
| YP3-T1 HS_1_1 | PASS | $6.16\times10^{-14}$ | $1\times10^{-12}$ | HS\_1\_1: the thermal states at T = 0.005 reproduce the ground state of T = 0 (/change of Phi0/ + relative change of the energy) |
| YP3-T3 HS_1_1 | PASS | $6.76\times10^{-14}$ | $1\times10^{-10}$ | HS\_1\_1: the critical temperature T\_c = 0.2604672394 = Rust (relative; bisection of lambda\^P(T) = 1 with the normal states of this engine) |
| YP3-T8 HS_1_1 | PASS | $1.71\times10^{-14}$ | $1\times10^{-10}$ | HS\_1\_1: the Dirac mass of the superfluid at 0.5 T\_c and 0.9 T\_c equals that at T = 0 (0.2066851932) and the Rust value (largest difference) |
| YP3-T2 HS_1_1 | PASS | $3.31\times10^{-12}$ | $1\times10^{-8}$ | HS\_1\_1: the order parameter and the free energy of the superfluid at 0.5 T\_c and 0.9 T\_c = Rust (largest difference; the free energy relative) |
| YP3-T1 CJT | PASS | $1.05\times10^{-13}$ | $1\times10^{-12}$ | CJT: the thermal states at T = 0.005 reproduce the ground state of T = 0 (/change of Phi0/ + relative change of the energy) |
| YP3-T3 CJT | PASS | $2.14\times10^{-13}$ | $1\times10^{-10}$ | CJT: the critical temperature T\_c = 0.2221187371 = Rust (relative; bisection of lambda\^P(T) = 1 with the normal states of this engine) |
| YP3-T8 CJT | PASS | $1.72\times10^{-13}$ | $1\times10^{-10}$ | CJT: the Dirac mass of the superfluid at 0.5 T\_c and 0.9 T\_c equals that at T = 0 (0.1622929325) and the Rust value (largest difference) |
| YP3-T2 CJT | PASS | $6.21\times10^{-13}$ | $1\times10^{-8}$ | CJT: the order parameter and the free energy of the superfluid at 0.5 T\_c and 0.9 T\_c = Rust (largest difference; the free energy relative) |
| YP3-T4 | PASS | $9.81\times10^{-12}$ | $1\times10^{-9}$ | HS (0, 0): the order parameter of every temperature of the table from the gap equation g\^P I1(m\^2 + Phi0\^2, T)/pi\^2 = 1 alone (without the self-consistent engine) = Rust (largest difference) |
| YP3-T9 HS_1_1 | PASS | $2.42\times10^{-14}$ | $1\times10^{-9}$ | HS\_1\_1: lambda\^P(T) of the normal state does not increase on 600 temperatures from 0.001 to 0.6 (largest rise 0.0e+00; from 1.183843 to 0.620857), so T\_c is the only solution of lambda\^P(T) = 1, and lambda\^P(T\_c of Rust) = 1 (the normal state from its scalar self-consistency alone; the larger of the rise and /lambda\^P(T\_c) - 1/) |
| YP3-T9 HS_0_1 | PASS | $3.46\times10^{-14}$ | $1\times10^{-9}$ | HS\_0\_1: lambda\^P(T) of the normal state does not increase on 600 temperatures from 0.001 to 0.6 (largest rise 0.0e+00; from 1.150958 to 0.604690), so T\_c is the only solution of lambda\^P(T) = 1, and lambda\^P(T\_c of Rust) = 1 (the normal state from its scalar self-consistency alone; the larger of the rise and /lambda\^P(T\_c) - 1/) |
| YP3-T9 HS_0_0 | PASS | $1.69\times10^{-14}$ | $1\times10^{-9}$ | HS\_0\_0: lambda\^P(T) of the normal state does not increase on 600 temperatures from 0.001 to 0.6 (largest rise 0.0e+00; from 1.113376 to 0.584946), so T\_c is the only solution of lambda\^P(T) = 1, and lambda\^P(T\_c of Rust) = 1 (the normal state from its scalar self-consistency alone; the larger of the rise and /lambda\^P(T\_c) - 1/) |
| YP3-T9 KS | PASS | $5.91\times10^{-14}$ | $1\times10^{-9}$ | KS: lambda\^P(T) of the normal state does not increase on 600 temperatures from 0.001 to 0.6 (largest rise 0.0e+00; from 1.113376 to 0.588434), so T\_c is the only solution of lambda\^P(T) = 1, and lambda\^P(T\_c of Rust) = 1 (the normal state from its scalar self-consistency alone; the larger of the rise and /lambda\^P(T\_c) - 1/) |
| YP3-T9 CJT | PASS | $5.91\times10^{-14}$ | $1\times10^{-9}$ | CJT: lambda\^P(T) of the normal state does not increase on 600 temperatures from 0.001 to 0.6 (largest rise 0.0e+00; from 1.113376 to 0.588434), so T\_c is the only solution of lambda\^P(T) = 1, and lambda\^P(T\_c of Rust) = 1 (the normal state from its scalar self-consistency alone; the larger of the rise and /lambda\^P(T\_c) - 1/) |
| YP3-T5 CJT | PASS | $3.08\times10^{-7}$ | $1\times10^{-5}$ | CJT at 0.9 T\_c: the Taylor coefficients a = -1.05102722e-03, b = 1.47550953e-02 of the free energy at the origin (four applied fields, exact cubic interpolation of F/Phi\^2) and the error 2.2771e-02 of the order parameter (-a/2b)\^(1/2) of Landau's theory = Rust (largest relative difference of a and b, difference of the error); the error is 0.228 times 1 - T/T\_c |
| YP3-T6 HS_1_1 | PASS | $7.06\times10^{-11}$ | $1\times10^{-7}$ | HS\_1\_1: the largest residuals of the Landau form (R1.3) and of the quartic form, relative to the depth, of the free-energy curves at T/T\_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed from data/part3\_thermal\_curves.csv with numpy (SVD) = Rust (largest relative difference); the quartic residual falls towards T\_c (1.83e-02, 8.81e-03, 3.73e-03 at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.7e-05 at 0.98 T\_c: yes |
| YP3-T7 HS_1_1 | PASS | $5.64\times10^{-12}$ | $1\times10^{-8}$ | HS\_1\_1: at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0, 7.035495e-03 of the depth, is that of row 8 of Table R1 (relative difference) |
| YP3-T6 HS_0_1 | PASS | $9.27\times10^{-12}$ | $1\times10^{-7}$ | HS\_0\_1: the largest residuals of the Landau form (R1.3) and of the quartic form, relative to the depth, of the free-energy curves at T/T\_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed from data/part3\_thermal\_curves.csv with numpy (SVD) = Rust (largest relative difference); the quartic residual falls towards T\_c (1.60e-02, 7.67e-03, 3.24e-03 at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.3e-05 at 0.98 T\_c: yes |
| YP3-T7 HS_0_1 | PASS | $1.3\times10^{-12}$ | $1\times10^{-8}$ | HS\_0\_1: at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0, 4.693236e-03 of the depth, is that of row 8 of Table R1 (relative difference) |
| YP3-T6 HS_0_0 | PASS | $4.88\times10^{-10}$ | $1\times10^{-7}$ | HS\_0\_0: the largest residuals of the Landau form (R1.3) and of the quartic form, relative to the depth, of the free-energy curves at T/T\_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed from data/part3\_thermal\_curves.csv with numpy (SVD) = Rust (largest relative difference); the quartic residual falls towards T\_c (1.27e-02, 6.06e-03, 2.56e-03 at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 7.8e-06 at 0.98 T\_c: yes |
| YP3-T7 HS_0_0 | PASS | $1.89\times10^{-11}$ | $1\times10^{-8}$ | HS\_0\_0: at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0, 2.690453e-03 of the depth, is that of row 8 of Table R1 (relative difference) |
| YP3-T6 KS | PASS | $1.31\times10^{-10}$ | $1\times10^{-7}$ | KS: the largest residuals of the Landau form (R1.3) and of the quartic form, relative to the depth, of the free-energy curves at T/T\_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed from data/part3\_thermal\_curves.csv with numpy (SVD) = Rust (largest relative difference); the quartic residual falls towards T\_c (1.58e-02, 7.47e-03, 3.13e-03 at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.0e-05 at 0.98 T\_c: yes |
| YP3-T7 KS | PASS | $3.54\times10^{-11}$ | $1\times10^{-8}$ | KS: at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0, 2.846809e-03 of the depth, is that of row 8 of Table R1 (relative difference) |
| YP3-T6 CJT | PASS | $5.8\times10^{-11}$ | $1\times10^{-7}$ | CJT: the largest residuals of the Landau form (R1.3) and of the quartic form, relative to the depth, of the free-energy curves at T/T\_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed from data/part3\_thermal\_curves.csv with numpy (SVD) = Rust (largest relative difference); the quartic residual falls towards T\_c (1.58e-02, 7.47e-03, 3.13e-03 at 0.9, 0.95, 0.98 T\_c) and that of (R1.3) is 1.0e-05 at 0.98 T\_c: yes |
| YP3-T7 CJT | PASS | $4.64\times10^{-11}$ | $1\times10^{-8}$ | CJT: at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0, 2.846809e-03 of the depth, is that of row 8 of Table R1 (relative difference) |

**Data files and figure** (SHA-256 checksums):

* `data/part3_thermal.csv` (Table A1, Figure A1 (a), (b), (d); per method and temperature: the order parameter, the
  Dirac masses of the superfluid and of the normal state, the gap, the radius (A1.6), the free energy at the minimum,
  the entropy difference, the residual of the gap equation, the Taylor coefficients, Landau's order parameter):\
  `c5fee32769ca52cea89527613b67cdd854ce91c1ee113805aab3d2ff02d0450f`
* `data/part3_thermal_landau.csv` (Table A1, Figure A1 (d)):\
  `ff63593fd7adfb0540f2141cad93438d810e0c00d44612df93889d17a1b577f0`
* `data/part3_thermal_curves.csv` (Figure A1 (c); the free-energy curves (A1.2)):\
  `519b410f803a762dcdb4d503197344d25079c751b45388421a0d79cc80e69e9b`
* `data/part3_values.csv` (the critical temperatures, the couplings and the eigenvalues of rows 1 to 4):\
  `348a825bc2d41da358659c46f13bd651468176b04620de44272dd45f1b753a43`
* `figures/fig_p4_a1_thermal.pdf` (Figure A1):\
  `ab6b75e647949def5f440154f7354c61407b24085680809c50561e5f3dd5740e`

**References.** T. Matsubara, A new approach to quantum-statistical mechanics, Progress of Theoretical Physics 14, 351
(1955) (the Matsubara frequencies); N. D. Mermin, Thermal properties of the inhomogeneous electron gas, Physical Review
137, A1441 (1965) (the functional of KS at finite temperature); G. Amelino-Camelia and S.-Y. Pi, Self-consistent
improvement of the finite-temperature effective potential, Physical Review D 47, 2356 (1993) (the CJT method at finite
temperature); D. J. Thouless, Perturbation theory in statistical mechanics and the theory of superconductivity, Annals
of Physics 10, 553 (1960) (the instability of the normal state); J. Bardeen, L. N. Cooper and J. R. Schrieffer, Theory
of superconductivity, Physical Review 108, 1175 (1957) (the ratio $\Delta(0)/T_{\rm c}$); L. D. Landau, On the theory of phase
transitions, Zhurnal Eksperimental'noi i Teoreticheskoi Fiziki 7, 19 (1937) (Landau's theory).

# 4. Remark R2: "All three methods give mean-field free energies; the fluctuations of the order parameter are not included"

**Where the remark was made.** In the answer to the addendum A1 (Section 3.6): the statements about the order
parameter near the critical temperature were qualified by this remark. By the author's standing instruction it is
explained here: what it says, why it holds, and why it matters.

## 4.1 What the remark says

The free energy (A1.1) is evaluated on quasi-free (thermal) states: states fully described by their normal and pair
correlations at each momentum, in which the order parameter has a fixed value $\Phi$. Such a state contains the thermal
quasi-particles of the spinor field (the occupations $f_k$), but not the thermal fluctuations of the order parameter
about its mean value, and not the gauge fields. The fluctuations of the order parameter are its collective modes: the
amplitude mode (the "Higgs" mode of the field), the Goldstone modes of the broken symmetry and the non-unitary modes,
excited thermally at every momentum. In each method:

* **HS:** the integral over the auxiliary fields of Hubbard and Stratonovich is replaced by its saddle point; the
  fluctuations of the auxiliary fields about the saddle point are the fluctuations of the order parameter, and they
  are left out;
* **CJT:** the effective action is truncated at the contractions of the contact interactions in pairs (the Hartree,
  Fock and pairing contractions); the diagrams that describe the exchange of collective modes (chains of bubbles and
  beyond) are left out;
* **KS:** the exchange-correlation energy is that of $T=0$, without a thermal part; the thermal fluctuations of the order
  parameter would belong to that thermal part.

## 4.2 Why it holds

A quasi-free state is the most general state whose correlations of four and more fields are fixed by its correlations
of two fields (Wick's theorem). The three methods choose the best such state for the energy (or the free energy) of the
contact interactions: they are self-consistent methods of independent quasi-particles, and their only correlations are
those of the normal and pair densities. A fluctuation of the order parameter is a correlation of two pairs (four
fields) beyond the product of the pair densities, so no quasi-free state contains it. This is the meaning of "mean
field": each charge moves in the mean pair field of the others, whose value is the same in every member of the thermal
ensemble.

## 4.3 Why it matters

1. **The critical region.** Near a continuous transition the fluctuations of the order parameter grow without bound
   (their correlation length diverges at the transition). Their size relative to the order parameter itself is
   measured by the **Ginzburg number** of the transition (Levanyuk 1959, Ginzburg 1960): the mean-field free energy is
   accurate where $1-T/T^{X,{\rm c}}_{\rm SF}$ is much larger than the Ginzburg number, and it fails within the **critical
   region**, where $1-T/T^{X,{\rm c}}_{\rm SF}$ is smaller. For a free energy of the form of Ginzburg and Landau (Section 2.2) in
   three dimensions, $\gamma_{\rm GL}\lvert\nabla\psi_{\rm GL}\rvert^2+\alpha'_{\rm GL}(T/T_{\rm c}-1)\lvert\psi_{\rm GL}\rvert^2+\tfrac12\beta_{\rm GL}\lvert\psi_{\rm GL}\rvert^4$
   with the critical temperature $T_{\rm c}$, the gradient coefficient $\gamma_{\rm GL}$ and the slope $\alpha'_{\rm GL}$ of the
   quadratic coefficient, the Ginzburg number is $T_{\rm c}^2\beta_{\rm GL}^2/(\alpha'_{\rm GL}\gamma_{\rm GL}^3)$, up to a numerical
   factor that depends on the conventions; it needs the gradient coefficient at the critical temperature. Within the
   critical region:
   * the order parameter vanishes with an exponent different from the $\tfrac12$ of item 1 of Section 3.6 (in three
     dimensions this exponent is near $0.33$ to $0.37$ for the Ising, $XY$ and Heisenberg universality classes);
   * the critical temperature is in general lower than the mean-field one (the fluctuations disorder the field);
   * for an order parameter with two quartic invariants, as here ($\lvert\vec\Phi\rvert^4$ and $\lvert\vec\Phi\cdot\vec\Phi\rvert^2$), the
     fluctuations can also make the transition discontinuous (of first order); the renormalization-group theory of such
     fields is reviewed by Pelissetto and Vicari (2002).
2. **The gauge fields.** The field is coupled to gauge fields (it gives masses to gauge bosons in the solution). The
   fluctuations of a gauge field coupled to an order parameter make the transition of a superconductor weakly first
   order (Halperin, Lubensky and Ma 1974); in the electroweak theory the thermal transition of the Higgs field is of
   first order for small Higgs masses, and the line of these transitions ends at a critical Higgs mass, above which
   there is no transition but a smooth crossover (lattice simulations of Kajantie, Laine, Rummukainen and Shaposhnikov,
   1996). Whether the transition of this superfluid is continuous in the full theory is therefore not decided by the
   free energy of Section 3.
3. **What remains valid.** Outside the critical region the mean-field free energy is the leading approximation, and
   items 1 to 4 of Section 3.6 describe the superfluid there. The size of the critical region is not known yet: the
   gradient coefficient of the field at $T^{X,{\rm c}}_{\rm SF}$ is not computed (the solution computes the coefficients of the
   gradient terms at $T=0$ only). Computing it, from the static pair response of the normal state at
   $T^{X,{\rm c}}_{\rm SF}$ and small momenta, is the next step if the critical region is to be located.
4. **At $T=0$.** The same methods omit the quantum fluctuations of the order parameter at $T=0$; that is a property of
   the methods, not the subject of this remark, which concerns the thermal fluctuations near $T^{X,{\rm c}}_{\rm SF}$.

**Provenance.** The remark is a statement about the definitions (A1.1) and (A1.2), which contain only quasi-free
states; no number of this section is computed. The symbols $\gamma_{\rm GL}$, $\alpha'_{\rm GL}$ and $T_{\rm c}$ are defined in item 1 of
Section 4.3 for this section only; $\beta_{\rm GL}$ and $\psi_{\rm GL}$ are those of the example of Ginzburg and Landau in
Section 2.2. References: A. P. Levanyuk, Zhurnal Eksperimental'noi i
Teoreticheskoi Fiziki 36, 810 (1959) [Soviet Physics JETP 9, 571]; V. L. Ginzburg, Some remarks on phase transitions of
the second kind and the microscopic theory of ferroelectric materials, Fizika Tverdogo Tela 2, 2031 (1960) [Soviet
Physics Solid State 2, 1824 (1961)]; A. Pelissetto and E. Vicari, Critical phenomena and renormalization-group theory,
Physics Reports 368, 549 (2002); B. I. Halperin, T. C. Lubensky and S.-k. Ma, First-order phase transitions in
superconductors and smectic-A liquid crystals, Physical Review Letters 32, 292 (1974); K. Kajantie, M. Laine, K.
Rummukainen and M. Shaposhnikov, Physical Review Letters 77, 2887 (1996).
