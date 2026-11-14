<!-- OUTPUT: provenance/P2_calculations_code_and_checks.md -->
---
title: "Provenance of the calculations: programs, instructions, data, checks"
subtitle: "PROJECT spacetimes, Revision_Deflation_CJT_and_Kohn-Sham_definition_C — the 16-component Dirac field with mass term and self-interaction on the deflating 4+4 and 7+1 spacetimes"
author: "Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is generated from docs_src/P2_calculations_code_and_checks.src.md by code/scripts/assemble_docs.py; edit the source. -->

# 1. What this document is

This document is the record of how every number, table and figure of the folder
`Revision_Deflation_CJT_and_Kohn-Sham_definition_C` was computed. It gives

- the objects that are computed, with their defining equations (Section 2);
- the instructions for compiling and running every program (Section 3);
- the list of the commands of the program, with their source files (Section 4);
- the complete output of every check of every command, of the two independent
  verification scripts, of the test of those scripts with deliberate errors, of the
  check of the symbols and of the check of the numbers quoted in the documents
  (Section 5);
- every data file, with the command that wrote it, its content and its columns
  (Section 6);
- every figure, with the data files from which it is drawn (Section 7);
- for each table and figure of the two manuscripts, the command and the data files
  behind it (Section 8);
- the scripts and the source files (Sections 9 and 10);
- the complete table of the symbols and of the names used in programs and data files
  (Appendix A).

The statements of the checks are printed by the programs and use the spellings of the
programs: `rho` for the number density $n$, `rho_S` for the scalar density $n_S$, `M`
for the mass $m^*$ of the particles in matter, `eps` for the energy density
$\varepsilon$, `Pi` for the interaction pressure $p_{\rm int}$, `tau_d` for the
deflation clock $\tau_d$, `Lambda` for the cutoff $\Lambda$, `m_ph` for the mass
$m_\vartheta$ of the phase mode, `kappa` for $\kappa_8$, `a`, `b`, `H_a`, `H_b` for
$\mathsf a$, $\mathsf b$, $H_{\mathsf a}$, $H_{\mathsf b}$; for the two halves of the
4+4 field `M_plus`, `M_minus` for the masses $m^*_+$, $m^*_-$, `pol` for the
polarization $\mathrm{pol}_W$, `G_Stot`, `G_Vtot`, `G_Sdif`, `G_Vdif`, `G_Ssame`,
`G_Vsame`, `G_Sopp`, `G_Vopp` for the eight mean-field couplings of Section 2,
`ct_sign` for the sign $s_{\rm ct}$, `uv` for $U_{\rm uv}/4\pi^2$; "the covariant form"
and "the quantized-sector form" for form C and form Q. Appendix A lists every such
name beside its symbol. In those statements "the main project" means the part of
PROJECT spacetimes that treats the flat 4+4 and 7+1 spacetimes: its Rust crate
(`code/spacetimes-rs` at the top of the repository) is used as a library, and its data
files (`data` at the top of the repository) are what several checks compare against.
"The author's document" means his document on Kohn–Sham matter and gravity on the 7+1
spacetime, whose numbers are reproduced.

# 2. The objects that are computed

**Coordinates.** $x0,\dots,x7$: $x0$ a hidden space-like coordinate; $x1,x2,x3$
ordinary space; $x4$ the ordinary time; $x5,x6,x7$ three further coordinates, time-like
on the 4+4 spacetime and space-like on the 7+1 spacetime.

**The hypothesis of deflation** (the author's). The coordinates $x5,x6,x7$ deflate
exponentially. With three positive rates $\mathrm{g55}$, $\mathrm{g66}$,
$\mathrm{g77}$, three positive constants and the scale factors
$b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$ ($t=5,6,7$;
$\mathrm{rate}_5=\mathrm{g55}$ and so on), the metrics are, with space-like components
positive and time-like components negative,

$$\begin{aligned}
g^{(4+4)}&=\mathrm{diag}(+1,+1,+1,+1,-1,-b_5^2,-b_6^2,-b_7^2),\\
g^{(7+1)}&=\mathrm{diag}(+1,+1,+1,+1,-1,+b_5^2,+b_6^2,+b_7^2).
\end{aligned}$$

The volume factor is $V_d=b_5b_6b_7$, the total rate
$\mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$, and the deflation clock
$\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$.

**The fermion field.** A column $\psi$ of sixteen complex Grassmann components with the
real $16\times16$ Dirac matrices $\gamma^A$ of the spacetime at hand. Its Lagrangian has
a kinetic term, a mass term and a self-interaction,

$$\mathcal L=V_d\,\big[\mathcal L_{\rm kin}+\mathcal L_{\rm mass}+\mathcal L_{\rm self}\big],
\qquad\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r,\qquad
I_r=\sum_{|K|=r}(\bar\psi\,\Theta_K\psi)(\bar\psi\,\Theta^K\psi).$$

Here $K$ runs over the strictly increasing lists of $r$ indices out of $0,\dots,7$,
$\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$ with a phase $\zeta_r$ (1 or $i$) that
makes the bilinear real, $\Theta_K=\mathrm{sgn}_K\,\Theta^K$ is its inverse
($(\Theta^K)^2=\mathrm{sgn}_K=\pm1$), and $g_0,\dots,g_8$ are nine couplings. On 7+1,
$\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\bar\psi\,(i\gamma^\mu\nabla_\mu-m)\psi$ with
$\bar\psi=\psi^\dagger\beta$, $\beta=\gamma^4$, and the covariant derivative $\nabla_\mu$
of a spinor; the tag 71s denotes its self-interaction.

**The 4+4 spacetime: the quantized sector and the two forms.** On 4+4 the covariant
Lagrangian is $\mathcal L_{\rm cov}=\tilde\psi\,(\gamma^\mu\nabla_\mu-m)\psi$ plus the
self-interaction, with the covariant adjoint $\tilde\psi=\psi^\dagger C^-$,
$C^-=\gamma^0\gamma^1\gamma^2\gamma^3$, and with $I_r$ built from $\tilde\psi$ in place
of $\bar\psi$. Only fields that do not depend on $x5,x6,x7$ have a unitary evolution
(the *quantized sector*). For them, with $\beta=-i\gamma^4$,
$\alpha^j=-\gamma^4\gamma^j$ ($j=0,1,2,3$),
$\mathsf h=\sum_j\alpha^j(-i\partial_j)+\beta m$ and the Hermitian matrix
$W_{1644}=i\,C^-\gamma^4$ ($W_{1644}^2=1$, $C^-=-W_{1644}\beta$),

$$\tilde\psi\,(\gamma^A\partial_A-m)\,\psi=-\psi^\dagger\,W_{1644}\,(i\partial_4-\mathsf h)\,\psi .$$

The field has two *halves*, $\psi=\psi_++\psi_-$ with $W_{1644}\psi_\pm=\pm\psi_\pm$,
eight components each; the covariant kinetic and mass terms are the ordinary ones on
the half $W_{1644}=-1$ and minus the ordinary ones on the half $W_{1644}=+1$. The
quantum theory is defined by the positive Lagrangian
$\mathcal L_{\rm kin}+\mathcal L_{\rm mass}=\psi^\dagger(i\partial_4-\mathsf h)\psi$ for all
sixteen components. The self-interaction is then taken in one of two forms:

* **Form C, the covariant form: the definition in this folder** (tag 44c). The
  invariants of the covariant Lagrangian,
  $$I_r^{\rm C}=\sum_{|K|=r}\mathrm{sgn}_K\,\big(\tilde\psi\,\Theta^K\psi\big)^2,\qquad
  \mathcal L_{\rm self}^{\rm C}=-\tfrac12\sum_rg_r\,I_r^{\rm C},$$
  with the field equation
  $i\partial_4\psi=\mathsf h\psi+\sum_rg_r\sum_{|K|=r}(\tilde\psi\,\Theta_K\psi)\,C^-\Theta^K\psi$.
* **Form Q, the quantized-sector form: the alternative** (tag 44q). With the matrices
  $\gamma_{44q}^A=-i\gamma^A$ ($A=0,\dots,4$), $\gamma_{44q}^A=\gamma^A$ ($A=5,6,7$),
  the products $\Theta_Q^K=\zeta_r^Q\,\gamma_{44q}^{A_1}\cdots\gamma_{44q}^{A_r}$ (phase
  $\zeta_r^Q=i$ for $r=2,3,6,7$ and 1 otherwise) and the signs
  $\mathrm{sgn}_K^Q=(\Theta_Q^K)^2$,
  $$I_r^{\rm Q}=\sum_{|K|=r}\mathrm{sgn}_K^Q\,\big(\bar\psi\,\Theta_Q^K\psi\big)^2,\qquad
  \mathcal L_{\rm self}^{\rm Q}=-\tfrac12\sum_rg_r\,I_r^{\rm Q},\qquad\bar\psi=\psi^\dagger\beta,$$
  with the field equation
  $i\partial_4\psi=\mathsf h\psi+\sum_rg_r\sum_{|K|=r}\mathrm{sgn}_K^Q\,(\bar\psi\,\Theta_Q^K\psi)\,\beta\,\Theta_Q^K\psi$,
  which is term by term that of the 7+1 spacetime for fields that do not depend on
  $x5,x6,x7$.

The two are related exactly: $C^-\Theta^K=z_K\,W_{1644}\,\beta\,\Theta_Q^K$ with a
phase $z_K$ ($\pm1$ or $\pm i$) for each of the 256 lists, so that
$I_r^{\rm C}=\sum_{|K|=r}\mathrm{sgn}_K^Q\,(\psi^\dagger\,W_{1644}\,\beta\,\Theta_Q^K\psi)^2$:
form C is form Q with $W_{1644}$ inserted in every bilinear. Two particles of the same
half interact identically in the two forms; a particle of each half with opposite
signs. In formulas that hold for both, the matrix of the adjoint is written
$\mathrm{Adj}$ ($C^-$ in form C; $\beta$ in form Q and on 7+1).

**Four independent couplings.** Because the components of $\psi$ anticommute, the nine
invariants obey $(1+\mathsf F)\,I=0$ with the Fierz matrix $\mathsf F$; only $I_0$,
$I_1$, $I_7$, $I_8$ are independent, with the effective couplings $g_0^{\rm eff}$
(scalar), $g_1^{\rm eff}$ (vector), $g_7^{\rm eff}$ (axial vector), $g_8^{\rm eff}$
(pseudoscalar).

**The Vorton-Scalar.** A complex scalar field $\Phi_{VS}$ with the potential
$\lambda_{VS}(|\Phi_{VS}|^2-v_{VS}^2/2)^2$ and the Yukawa coupling
$y_{VS}\,\bar\psi(\Phi_{VS}P^R+\Phi_{VS}^*P^L)\psi$. In its vacuum the fermions have the
mass $m=m_D+m_Y$ (bare Dirac mass plus $m_Y=y_{VS}v_{VS}/\sqrt2$), the radial mode has
the mass $m_s=\sqrt{2\lambda_{VS}}\,v_{VS}$, and both modes couple with
$g_{VS}=m_Y/v_{VS}$; $\alpha_{VS}=g_{VS}^2/4\pi$, $D_{VS}=\alpha_{VS}m/m_s$,
$C=g_{VS}^2m^2/m_s^2$.

**Fields that do not depend on $x5,x6,x7$.** The weighted field
$\hat\psi=\sqrt{V_d}\,\psi$ obeys the flat equations with the couplings
$g_r/V_d$, $y_{VS}/\sqrt{V_d}$, $\lambda_{VS}/V_d$ and the vacuum value
$v_{VS}\sqrt{V_d}$: the masses do not change, and $g_r$, $g_{VS}^2$, $C$, $D_{VS}$ grow
as $e^{\tau_d}$.

**The Kohn–Sham method, for one kind of particle.** Uniform matter with the number
density $n$ and the scalar density $n_S$ is described by particles of the mass
$m^*=m+G_S\,n_S$ in the vector field $V_0=G_V\,n$, with the mean-field couplings $G_S$,
$G_V$. With $E^*(k)=\sqrt{k^2+m^{*2}}$ and the average
$\langle f\rangle=\nu\int d^Dk\,f/(2\pi)^D$ over the occupied states $|\vec k|<k_F$
($\nu$ internal states: 8 for a particle of the 7+1 model and for the two equally
filled halves of the 4+4 field, 4 for one half alone; $D$ space dimensions with
momentum),

$$\begin{aligned}
n_S&=\Big\langle\frac{m^*}{E^*}\Big\rangle,&
\varepsilon&=\langle E^*\rangle+\tfrac12G_Vn^2-\frac{(m^*-m)^2}{2G_S},\\
p_j&=\Big\langle\frac{k_j^2}{E^*}\Big\rangle+p_{\rm int},&
p_{\rm int}&=\tfrac12G_Vn^2+\frac{(m^*-m)^2}{2G_S}.
\end{aligned}$$

**The mean-field couplings.** With Wick's theorem (direct minus exchange term) the
energy density of the self-interaction in uniform matter is
$\varepsilon_{\rm int}=\tfrac12G_V^{\rm HF}n^2+\tfrac12G_S^{\rm HF}n_S^2$ on 7+1, and on
4+4, when the two halves are filled to the Fermi momenta $k_{F+}$, $k_{F-}$ (densities
$n_\pm$, scalar densities $n_{S\pm}$; $n=n_++n_-$, $n_W=n_+-n_-$, $n_S=n_{S+}+n_{S-}$,
$n_{SW}=n_{S+}-n_{S-}$),

$$\begin{aligned}
\varepsilon_{\rm int}&=\tfrac12\big(G_V^{\rm tot}n^2+G_S^{\rm tot}n_S^2+G_V^{\rm dif}n_W^2+G_S^{\rm dif}n_{SW}^2\big)\\
&=\tfrac12G_V^{\rm same}(n_+^2+n_-^2)+\tfrac12G_S^{\rm same}(n_{S+}^2+n_{S-}^2)+G_V^{\rm opp}n_+n_-+G_S^{\rm opp}n_{S+}n_{S-},
\end{aligned}$$

with $G^{\rm same}=G^{\rm tot}+G^{\rm dif}$, $G^{\rm opp}=G^{\rm tot}-G^{\rm dif}$ and the four
combinations of the effective couplings

$$\begin{aligned}
G_S^{\rm HF}&=\tfrac1{16}(15g_0^{\rm eff}-8g_1^{\rm eff}-8g_7^{\rm eff}-g_8^{\rm eff}),&
G_V^{\rm HF}&=\tfrac1{16}(-g_0^{\rm eff}+22g_1^{\rm eff}-6g_7^{\rm eff}+g_8^{\rm eff}),\\
G_S^{\rm c}&=\tfrac1{16}(-g_0^{\rm eff}-2g_1^{\rm eff}+2g_7^{\rm eff}+g_8^{\rm eff}),&
G_V^{\rm c}&=-\tfrac1{16}(g_0^{\rm eff}+g_8^{\rm eff}).
\end{aligned}$$

In form C, $(G^{\rm tot},G^{\rm dif})=(G^{\rm c},G^{\rm HF})$; in form Q,
$(G^{\rm tot},G^{\rm dif})=(G^{\rm HF},G^{\rm c})$, for the scalar and for the vector
couplings alike. The numbers are given for two sets of couplings of the Lagrangian
with one strength $g>0$: **set A**, $g_0=-g$; **set B**, $g_0=-g$ and $g_1=+0.1\,g$.

**The Kohn–Sham method for the two halves.** The particles of the half
$W_{1644}=\pm1$ have the mass and the energy shift

$$m^*_\pm=m+G_S^{\rm tot}\,n_S\pm G_S^{\rm dif}\,n_{SW},\qquad V_{0\pm}=G_V^{\rm tot}\,n\pm G_V^{\rm dif}\,n_W,$$

and fill four states for each momentum below $k_{F\pm}$; with
$E^*_\pm(k)=\sqrt{k^2+m^{*2}_\pm}$,

$$\begin{aligned}
n_\pm&=4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D},\qquad
n_{S\pm}=4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D}\,\frac{m^*_\pm}{E^*_\pm},\\
\varepsilon&=\sum_\pm4\!\int_{|\vec k|<k_{F\pm}}\!\frac{d^Dk}{(2\pi)^D}\,E^*_\pm
+\tfrac12\big(G_V^{\rm tot}n^2+G_V^{\rm dif}n_W^2\big)-\tfrac12\big(G_S^{\rm tot}n_S^2+G_S^{\rm dif}n_{SW}^2\big).
\end{aligned}$$

At a given total density the state is the one of lowest energy with respect to the
polarization $\mathrm{pol}_W=(n_+-n_-)/n$; $k_{F,\rm tot}$ is the Fermi momentum that
the total density would have with equal halves. The matrix $\mathsf R_{\rm gap}$ is
the matrix of the derivatives of the right-hand sides of the two gap equations for
$m^*_+$, $m^*_-$ with respect to the two masses; a solution is a regular function of
the density and the couplings while its eigenvalues are below 1.

**The CJT effective potential.** For matter of one kind of particle, at the chemical potential $\mu$,

$$\Omega_{\rm CJT}(m^*,V_0;\mu)=-\nu\!\int\!\frac{d^Dk}{(2\pi)^D}\,(\mu-V_0-E^*)\,\theta(\mu-V_0-E^*)
-\frac{(m^*-m)^2}{2G_S}-\frac{V_0^2}{2G_V},$$

to which the filled states of negative energy below a cutoff $\Lambda$ add
$-\nu\int_{|\vec k|<\Lambda}d^Dk\,[\sqrt{k^2+m^{*2}}-\sqrt{k^2+m^2}]/(2\pi)^D$. Its
stationary points are the Kohn–Sham states. With the filled states a massless field
acquires a mass when $G_S$ is below the critical coupling
$G_c=-(2\pi)^D(D-1)/(\nu A_D\Lambda^{D-1})$, where $A_D$ is the area of the unit sphere
in $D$ dimensions. For the two halves the potential is a function of four numbers,

$$\begin{aligned}
\Omega_{\rm CJT}={}&\sum_\pm\Omega_{\rm half}(m^*_\pm,\mu-V_{0\pm})
-\frac{[\tfrac12(m^*_++m^*_-)-m]^2}{2G_S^{\rm tot}}-\frac{[\tfrac12(m^*_+-m^*_-)]^2}{2G_S^{\rm dif}}\\
&-\frac{[\tfrac12(V_{0+}+V_{0-})]^2}{2G_V^{\rm tot}}-\frac{[\tfrac12(V_{0+}-V_{0-})]^2}{2G_V^{\rm dif}},
\end{aligned}$$

with the grand potential
$\Omega_{\rm half}(m^*,\mu)=-4\int d^Dk\,(\mu-E^*)\,\theta(\mu-E^*)/(2\pi)^D$ of free
particles of one half. The vacuum of the two halves, with the scalar densities
$n_{S\pm}^{\rm vac}=-4\int_{|\vec k|<\Lambda}d^Dk\,(m^*_\pm/E^*_\pm)/(2\pi)^D$ of the filled
states and a bare mass $m_{\rm bare}$, obeys
$\tfrac12(m^*_++m^*_-)-m_{\rm bare}=G_S^{\rm tot}(n_{S+}^{\rm vac}+n_{S-}^{\rm vac})$ and
$\tfrac12(m^*_+-m^*_-)=G_S^{\rm dif}(n_{S+}^{\rm vac}-n_{S-}^{\rm vac})$; among several
solutions it is the one of lowest energy density

$$\Omega_{\rm vac}=-4\sum_\pm\!\int_{|\vec k|<\Lambda}\!\frac{d^Dk}{(2\pi)^D}\Big[E^*_\pm-\sqrt{k^2+m_{\rm bare}^2}\Big]
-\frac{[\tfrac12(m^*_++m^*_-)-m_{\rm bare}]^2}{2G_S^{\rm tot}}-\frac{[\tfrac12(m^*_+-m^*_-)]^2}{2G_S^{\rm dif}} .$$

The critical strength of a field without mass term is $g_{\rm crit}$, the strength at
which the more attractive of $G_S^{\rm tot}$, $G_S^{\rm dif}$ equals $G_c$ (with
$\nu=8$); $\Lambda_{\max}$ is the largest cutoff for which the vacuum is stable at a
given strength.

**Pairs.** Two particles at rest have 28 antisymmetric states; the self-interaction
acts on a pair state $\Phi$ (an antisymmetric $16\times16$ matrix) as
$\Phi\mapsto\sum_rg_r\sum_{|K|=r}\mathrm{sgn}_K\,(\mathrm{Adj}\,\Theta^K)\,\Phi\,(\mathrm{Adj}\,\Theta^K)^T$
projected on the states of positive energy (in form Q with $\Theta_Q^K$,
$\mathrm{sgn}_K^Q$), and its eigenvalues are the pair couplings $G_{\rm pair}$. For
two particles of the mass $m^*$ with momenta $\pm\vec p$ scattering into $\pm\vec p'$
in the pair along the hidden axis (energies $E$, $E'$), the amplitudes of the four
independent channels are, on 7+1 and in form Q,

$$\begin{aligned}
F_0&=\frac{EE'+m^{*2}-\vec p\cdot\vec p'}{2EE'},&
F_1&=\frac{3EE'-2m^{*2}-\vec p\cdot\vec p'}{EE'},\\
F_7&=\frac{3EE'+2m^{*2}-\vec p\cdot\vec p'}{EE'},&
F_8&=-\frac{EE'-m^{*2}-\vec p\cdot\vec p'}{2EE'},
\end{aligned}$$

and $F_S=F_0$, $F_P=-F_8$ are the amplitudes of the exchange of the radial and of the
phase mode of the Vorton-Scalar. The pairing potentials are

$$V_{\rm ct}=s_{\rm ct}\,\big(g_0^{\rm eff}F_0+g_1^{\rm eff}F_1+g_7^{\rm eff}F_7+g_8^{\rm eff}F_8\big)\frac{m_{\rm ct}^2}{q^2+m_{\rm ct}^2},\qquad
V_{VS}=-g_{VS}^2\Big[\frac{F_S}{q^2+m_s^2}+\frac{F_P}{q^2+m_\vartheta^2}\Big],$$

with the momentum transfer $q$, the range parameter $m_{\rm ct}$ of the contact
couplings, and the sign $s_{\rm ct}$: $-1$ for the pair along the hidden axis in form
C, $+1$ for the pairs along $x5,x6,x7$ in form C, for all of them in form Q, and on 7+1.
The gap function $\Delta(k)$ obeys

$$\Delta(k)=\frac1{4\pi^2}\int_0^{k_{\max}}dk'\,k'^2\,W(k,k')\,\frac{\Delta(k')}{E_{k'}}\tanh\frac{E_{k'}}{2T},\qquad
W(k,k')=\frac12\int_{-1}^{1}d\cos\theta\,\big[-V_{VS}-V_{\rm ct}\big],$$

with $\xi_k=\sqrt{k^2+m^2}-\mu$, $E_k=\sqrt{\xi_k^2+\Delta(k)^2}$, the temperature $T$,
and the upper limit $k_{\max}$ of the momentum integral ($2\times10^4$ times the larger
of $m$ and $k_F$ in this folder). At momenta above all masses
$W(k,k')\to w_\infty(k'/k)/k^2$, and a solution that does not depend on $k_{\max}$
exists, falling as $k^{-s_\Delta}$, only if the *ultraviolet strength*

$$\begin{aligned}
U_{\rm uv}&=\int_0^\infty dt\,w_\infty(t)=2\,u_S\,g_{VS}^2-s_{\rm ct}\,m_{\rm ct}^2\big[(g_0^{\rm eff}-g_8^{\rm eff})\,u_S+(g_1^{\rm eff}+g_7^{\rm eff})\,u_V\big],\\
u_S&=\frac{\pi^2}{8}-\frac12,\qquad u_V=\frac{3\pi^2}{4}-1,
\end{aligned}$$

is below $4\pi^2$; no gap is given above that criterion.

**Bare mass, self-interaction and Vorton-Scalar together.** Let $\phi$ be the
expectation value of the Vorton-Scalar, $\langle\Phi_{VS}\rangle=\phi/\sqrt2$,
$x=\phi/v_{VS}$, and $f_D=m_D/m$ the fraction of the fermion mass that is a bare Dirac
mass. In units $m=1$ the energy density of matter of one kind of particle with the
Fermi momentum $k_F$, for fields in ordinary space ($D=3$), is the stationary value of

$$\varepsilon=\frac{(1-f_D)^2\,(x^2-1)^2}{8\,C}+\langle E^*\rangle+\tfrac12G_V\,n^2-\frac{\big[m^*-f_D-(1-f_D)\,x\big]^2}{2\,G_S}$$

with respect to $x$ and $m^*$. For the two halves of the 4+4 field the last three
terms are replaced by the energy density of the two halves given above, with
$f_D+(1-f_D)\,x$ in place of $m$. The data files give the smallest $C$ at which matter
is self-bound against the ratios $-g_0/C$ and $g_1/C$ of the contact couplings to $C$
(columns `kappa_0`, `kappa_1`).

**Gravity.** For a metric with two scale factors $\mathsf a(x4)$ (for $n_{\mathsf a}$
coordinates) and $\mathsf b(x4)$ (for $n_{\mathsf b}$ coordinates), rates
$H_{\mathsf a}$, $H_{\mathsf b}$, the Einstein–Lovelock equations are
$\sum_{k=1}^3\alpha_kE^{(k)\mu}{}_\nu-\bar\Lambda\,\delta^\mu_\nu=\kappa_8T^\mu{}_\nu$
with $\alpha_1=1$, $T^4{}_4=\varepsilon$, $T^m{}_m=-p_m$, and
$E^{(k)m}{}_m=(-1)^{k+1}k!\,2^{k-1}$ times the sum, over the sets of $2k$ directions
other than $m$ and over their divisions into pairs, of the products of the pair
curvatures $K_{4i}=H_i'+H_i^2$, $K_{ij}=H_iH_j$.

# 3. How to run everything

**What is needed.** A Rust toolchain (`cargo`); Python 3 with the packages numpy,
scipy, sympy, mpmath and matplotlib; for the documents, pandoc, XeLaTeX with latexmk,
and pdflatex with REVTeX 4.2 and BibTeX. No Rust crate is downloaded: the program
depends only on the library crate of the same repository.

**Where.** All commands are run from the folder
`Revision_Deflation_CJT_and_Kohn-Sham_definition_C` of the repository
`https://github.com/once-ere/spacetimes`. On the author's Mac that is

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation_CJT_and_Kohn-Sham_definition_C"
```

**Step 1: compile the program and run its unit tests.**

```bash
cargo test  --release --manifest-path code/deflation-cjt-ks-c-rs/Cargo.toml
cargo build --release --manifest-path code/deflation-cjt-ks-c-rs/Cargo.toml
```

The first command compiles the program and executes its {{GEN:n_unit_tests}} unit
tests; it must end with `test result: ok`. The second builds the program
`code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c`.

**Step 2: run the commands.** Each command writes its data files and its report of
checks into the folder `data` and ends with a line such as `kohnsham: 13 of 13 checks
passed`.

```bash
B=code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c
$B help
for c in selfcoupling kohnsham pairing gravity matrices modes symmetry channels \
         history quench paired cosmology matter background spectrum; do $B $c; done
$B forms
$B forecast
$B superfluid
$B families
```

The first line abbreviates the path of the program; `help` lists the commands. The
loop runs fifteen commands, each of which takes from a few seconds to about two
minutes on the two-processor machine on which they were executed. The last four take
longer: `forms` about 10 minutes, `forecast` about 13 minutes, `superfluid` about 42
minutes, `families` about 75 minutes (`$B families quick` is a short test of the same
code).

**Step 3: the independent verifications.**

```bash
python3 code/scripts/verify_deflating_independent.py
python3 code/scripts/verify_cjt_ks_independent.py
python3 code/scripts/mutation_test_independent.py
python3 code/scripts/vendor_eb.py --check
```

The first two scripts repeat the central results with computer algebra and
general-purpose numerical tools and write `data/deflating_checks_independent.txt` and
`data/cjt_ks_checks_independent.txt` (the second takes about three minutes). The third
introduces deliberate errors into copies of the two scripts, one at a time, and records
whether the checks notice (about 13 minutes). The fourth verifies that the four modules of the Einstein–Boltzmann
program in this folder are exactly the modules of the library crate with the listed
changes.

**Step 4: figures, tables and documents.**

```bash
python3 code/scripts/make_figures.py
python3 code/scripts/write_symbol_registry.py
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py
python3 code/scripts/build_manuscripts.py
bash    code/scripts/build_docs.sh
python3 code/scripts/check_numbers.py
python3 code/scripts/check_symbols.py
python3 code/scripts/check_self_contained.py
bash    code/scripts/check_all.sh
```

In this order: draw every figure from the data files; write the registry of symbols;
write every table and every computed number of the documents from the data files;
assemble the Markdown documents; assemble and typeset the two manuscripts (REVTeX; the
script fails if a reference is undefined or a box is too wide) and write their
Markdown versions; typeset the other documents; verify that every number of the
running text is a number that a program stated; verify the symbols; verify that no
document sends its reader to another document. The last command runs these three
checks once more and reads the verdict of every report in the folder `data`; it prints
one line for each and ends with `ALL PASS` only if every check of every report passes.
Nothing is to be committed to the repository unless it does.

One of the reports, `data/symbol_checks.txt`, is made from the assembled documents,
and `write_doc_tables.py` refuses to run while a report fails. If a change of a
document makes that report fail, run `python3 code/scripts/write_doc_tables.py
--first-pass` (which does not require that one report to pass), then
`assemble_docs.py` and `check_symbols.py`, and then the commands above in their
order.

# 4. The commands of the program

{{GEN:tbl_commands}}

The program performs {{GEN:n_checks_rust}} checks in all (each report ends with one
further line, `ALL`, which says that every check above it passed). A check marked
`exact` is an identity verified in exact rational or integer arithmetic; a check
marked `numerical` states its largest deviation.

# 5. The complete output of the checks

## 5.1 Mass term and self-interaction: command `selfcoupling`

```
{{FILE:data/selfcoupling_checks.txt}}
```

## 5.2 Kohn–Sham and CJT calculations: command `kohnsham`

```
{{FILE:data/kohnsham_checks.txt}}
```

## 5.3 The two forms of the 4+4 model: command `forms`

```
{{FILE:data/forms_checks.txt}}
```

## 5.4 Pairing with contact couplings, and the ultraviolet criterion: command `pairing`

```
{{FILE:data/pairing_checks.txt}}
```

## 5.5 Gravity with Kohn–Sham matter: command `gravity`

```
{{FILE:data/gravity_checks.txt}}
```

## 5.6 Geometry, matrices, spin connection, Dirac operators: command `matrices`

The Dirac matrices of the frame:

```
{{FILE:data/flat_matrix_checks.txt}}
```

The deflating spacetimes:

```
{{FILE:data/deflating_checks.txt}}
```

## 5.7 Solutions mode by mode: command `modes`

```
{{FILE:data/mode_checks.txt}}
```

## 5.8 Symmetries and exponents of the couplings: command `symmetry`

```
{{FILE:data/symmetry_checks.txt}}
```

## 5.9 Exchange of the Vorton-Scalar: command `channels`

```
{{FILE:data/channel_checks.txt}}
```

## 5.10 The model along the clock: commands `history`, `quench`, `paired`

```
{{FILE:data/history_checks.txt}}
```

```
{{FILE:data/quench_checks.txt}}
```

```
{{FILE:data/paired_checks.txt}}
```

## 5.11 Cosmology: commands `cosmology`, `matter`, `background`, `spectrum`, `forecast`, `families`, `superfluid`

```
{{FILE:data/cosmology_checks.txt}}
```

```
{{FILE:data/matter_checks.txt}}
```

```
{{FILE:data/background_checks.txt}}
```

```
{{FILE:data/spectrum_checks.txt}}
```

```
{{FILE:data/forecast_checks.txt}}
```

```
{{FILE:data/families_checks.txt}}
```

```
{{FILE:data/superfluid_checks.txt}}
```

## 5.12 Independent verification of the geometry, the Dirac field, the modes and the cosmology

```
{{FILE:data/deflating_checks_independent.txt}}
```

## 5.13 Independent verification of the self-interaction, of the two forms, of the Kohn–Sham and CJT calculations, of the pairing and of the gravity

```
{{FILE:data/cjt_ks_checks_independent.txt}}
```

## 5.14 The test of the independent scripts with deliberate errors

```
{{FILE:data/mutation_test_independent.txt}}
```

## 5.15 The symbols

```
{{FILE:data/symbol_checks.txt}}
```

## 5.16 The numbers quoted in the running text of the documents

```
{{FILE:data/number_checks.txt}}
```

## 5.17 No document sends its reader to another document

```
{{FILE:data/self_contained_checks.txt}}
```

# 6. The data files

The folder `data` holds {{GEN:n_data_csv}} tables of numbers (files `.csv`) besides the
reports of Section 5. Each begins with lines that say which program and command wrote
it, what it contains, and what its columns are; the table repeats these lines.

{{GEN:tbl_data_files}}

# 7. The figures

Every figure is drawn by `code/scripts/make_figures.py` from data files; none contains
a number typed by hand. `python3 code/scripts/make_figures.py` draws all of them;
`python3 code/scripts/make_figures.py ks_flat unified` draws the named ones.

{{GEN:tbl_figures_all}}

# 8. The tables and figures of the two manuscripts

The manuscripts are assembled from one source, `docs_src/manuscript/*.tex`, in which
every table of computed numbers is a placeholder that
`code/scripts/build_manuscripts.py` replaces by rows that
`code/scripts/write_doc_tables.py` wrote from the data files. The two tables give, for
each table and figure of each manuscript, its label in the source, the beginning of
its caption (formulas replaced by dots), the commands and the data files.

**Manuscript on the 4+4 deflating spacetime** ({{GEN:n_prov_items_1644}} items):

{{GEN:tbl_prov_plain_1644}}

**Manuscript on the 7+1 deflating spacetime** ({{GEN:n_prov_items_1671}} items):

{{GEN:tbl_prov_plain_1671}}

# 9. The scripts

| script (in `code/scripts`) | what it does |
|:---------------|:-------------------------------------------|
| `make_figures.py` | draws every figure from the data files |
| `write_symbol_registry.py` | writes the registry of symbols `provenance/symbols_master.csv` and its tables |
| `check_symbols.py` | the eight checks of the symbols (Section 5.15) |
| `write_doc_tables.py` | writes every table and every computed number of the documents from the data files, and the tables of provenance of this document, into `provenance/generated/tables_cjt_ks.md` |
| `assemble_docs.py` | builds each Markdown document from its source in `docs_src` by inserting the generated tables and the reports |
| `build_docs.sh` | typesets every Markdown document (pandoc, XeLaTeX) and checks its layout with `check_layout.py` |
| `build_manuscripts.py` | assembles and typesets the two manuscripts and writes their Markdown versions with `tex2md.py` |
| `check_numbers.py` | verifies that every number of the running text of the documents is a number that a program stated (Section 5.16) |
| `check_self_contained.py` | verifies that no document sends its reader to another document (Section 5.17) |
| `check_all.sh` | runs the three checks of the documents again, reads the verdict of every report in `data`, and verifies that every data file was written in this folder; ends with `ALL PASS` or names what fails |
| `build_all_docs.sh` | runs the registry, the generated tables, the assembly, the typesetting and the checks in the order in which each finds current input (twice where a report that a document prints is made from the documents), and ends with the verdict of `check_all.sh` |
| `verify_deflating_independent.py` | independent verification of geometry, Dirac field, modes, cosmology (Section 5.12) |
| `verify_cjt_ks_independent.py` | independent verification of self-interaction, the two forms of the 4+4 model, Kohn–Sham and CJT calculations, pairing with the ultraviolet criterion, gravity (Section 5.13) |
| `mutation_test_independent.py` | the test of the two independent scripts with deliberate errors (Section 5.14) |
| `vendor_eb.py` | writes the four modules of the Einstein–Boltzmann program of this folder from those of the library crate, applying a listed set of changes; with `--check` it verifies that they are up to date |

# 10. The source files of the program

The program consists of {{GEN:n_source_files}} source files with
{{GEN:n_source_lines}} lines, comments included, and {{GEN:n_unit_tests}} unit tests.
Each file begins with a comment that states the equations it implements.

{{GEN:tbl_sources}}

# Appendix A. The symbols, and the names used in programs and data files

Columns: the symbol as it is written in the documents; the names that programs and
data files use for it; its meaning; its origin.

{{GEN:symbols_master_tables}}

Key to the column "origin":

{{GEN:symbols_origin_key}}
