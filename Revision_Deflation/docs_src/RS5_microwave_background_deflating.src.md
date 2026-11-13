<!-- OUTPUT: supplement/RS5_microwave_background_deflating.md -->
---
title: "RS5 — The microwave background and the growth of structure on the deflating spacetimes: perturbation equations, closures, spectra and forecasts"
subtitle: "PROJECT spacetimes, Revision_Deflation. Supplement to both manuscripts of the revision"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 13, 2026"
---

<!-- This file is GENERATED from docs_src/RS5_microwave_background_deflating.src.md by code/scripts/assemble_docs.py.
     Edit the source, not this file: every table and every program output below is inserted by machine. -->

# 0. Purpose and summary

The supplement S3 of the main project computes the temperature and polarization
spectra of the cosmic microwave background and the power spectrum of matter with an
Einstein–Boltzmann program of its own, and uses them three times: to validate the
program, to forecast how well mock data constrain its families of dark radiation, and
to follow the growth of structure when the dark matter is a superfluid. The task of
the revision is to repeat every calculation of the main project while the coordinates
$x5$, $x6$, $x7$ deflate. This supplement repeats those three.

The supplement RS3 treats the *unperturbed* universe on the deflating spacetimes: if
gravity is eight-dimensional (assumption A2 of RS3), Newton's constant grows at the
total rate of deflation, gravity was weaker in the past, and the Hubble constant
inferred from the sound horizon falls. The spectra need more than that. They are
built from small perturbations of ordinary space, and the equations that govern the
perturbations contain the *perturbed* stresses of whatever keeps the three rates of
deflation constant. The author's hypotheses prescribe the metric of the deflating
coordinates; they do not say what holds it. One further assumption is therefore
unavoidable. It is called A3 here, it is stated in Sec. 1, and its consequences are
kept apart from those of A1 and A2 throughout.

**Summary.**

1. *The equations* (Sec. 2). With the scale factors of the extra coordinates
   unperturbed, the Einstein equations of ordinary space to first order in a
   perturbation are those of four dimensions with Newton's constant of the moment
   and with the conformal Hubble rate $H_{\rm cf}$ replaced by
   $H_e=H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum}/2$ in three of them. This is derived by
   computer algebra for arbitrary rates and both signatures.
2. *The four equations cannot all hold with matter alone* (Sec. 3). In general
   relativity two of the four scalar Einstein equations imply the other two. With a
   Newton's constant that depends on the time this fails by terms proportional to
   $\mathrm{gsum}$: the agent that holds the extra coordinates must carry
   perturbations, and one must say which. Each choice is a *closure*. Two are
   computed ("Momentum", "Pressure"); a third ("Shear") is shown to coincide with
   "Pressure" far inside the horizon.
3. *What does not depend on the closure* (Sec. 7). For the multipoles
   $\ell\ge30$, which carry almost all the information, the spectra of the two
   closures agree to a thousandth of the effect of the deflation: they follow from
   A1, A2 and the first two parts of A3, whatever the closure. At the rate that lunar laser ranging allows, mock
   data of the quality of the Planck satellite cannot tell the spectra from those
   without deflation; at the rate that nucleosynthesis allows they differ by up to
   5.6 per cent.
4. *What does depend on the closure* (Secs. 4, 7.3). The growth of structure and the
   largest angular scales ($\ell<30$) depend on the closure at the first order in
   $\mathrm{gsum}/H_0$. At a fixed Hubble constant the amplitude of clustering falls
   by $0.443\,\mathrm{gsum}/H_0$ in one closure and by $0.207\,\mathrm{gsum}/H_0$ in
   the other. These are not predictions of the hypotheses. An earlier version of the
   revision quoted a single number, from an equation that is neither closure; that
   was an error (provenance RP9, item R-E33).
5. *The Hubble constant* (Secs. 6, 8). With recombination computed on the deflating
   background, the Hubble constant inferred at a fixed angle of the sound horizon
   falls about six per cent faster than RS3 first reported:
   $d\ln H_0/d(\mathrm{gsum}/H_0)=-0.969$. In fits of all six parameters of the
   standard model to mock data it falls faster still, by 84 km/s/Mpc per unit of
   $\mathrm{gsum}/H_0$. Deflation cannot raise the Hubble constant inferred from the
   microwave background.
6. *A new bound, as a forecast* (Sec. 8). Planck-like mock data bound the rate to
   $\mathrm{gsum}<0.0385\,H_0$ at 95 per cent, and to $0.0207\,H_0$ with a deep
   survey added: weaker than lunar laser ranging by a factor of 12, stronger than
   nucleosynthesis by a factor of 1.7.
7. *The dark-radiation families of the main project* (Sec. 9) are fitted again on
   the deflating background. At the rate that lunar laser ranging allows, every
   best-fit Hubble constant of the main project falls by about a quarter of a
   km/s/Mpc, and its conclusions stand. With the rate free, more extra radiation is
   allowed, because extra radiation and weaker gravity act against each other in the
   early universe; but the Hubble constant falls along that compensation as well,
   and the largest Hubble constant that the mock data allow is reached at zero rate
   for every family.
8. *The superfluid dark matter of the main project* (Sec. 10) is followed again on
   the deflating background. The scale below which its pressure suppresses structure
   moves by 0.15 to 0.16 per cent at the rate that lunar laser ranging allows and by
   about 3 per cent at the rate that nucleosynthesis allows, in either closure; the
   limits on the fermion mass and on the sound speed of a condensate move
   accordingly.

## 0.1 How to reproduce

```bash
cd "/Users/nsh/Developer/github/spacetimes/Revision_Deflation"
python3 code/scripts/vendor_eb.py          # writes the four copied modules (eb_*.rs)
cargo build --release --manifest-path code/deflation-rs/Cargo.toml
cargo test --release --manifest-path code/deflation-rs/Cargo.toml
./code/deflation-rs/target/release/deflation cosmology    # RS3 Sec. 2; Sec. 6 here
./code/deflation-rs/target/release/deflation matter       # Sec. 4
./code/deflation-rs/target/release/deflation background   # Sec. 5.2      (seconds)
./code/deflation-rs/target/release/deflation spectrum     # Sec. 7        (1 minute)
./code/deflation-rs/target/release/deflation forecast     # Sec. 8        (6 minutes)
./code/deflation-rs/target/release/deflation families     # Sec. 9        (40 minutes)
./code/deflation-rs/target/release/deflation superfluid   # Sec. 10       (20 minutes)
python3 code/scripts/verify_deflating_independent.py  # independent checks (I44d-, I71d-, Idef-)
python3 code/scripts/mutation_test_independent.py         # can these checks fail? (4 minutes)
python3 code/scripts/make_figures.py spectra closure forecast families sfdm hubble bounds
python3 code/scripts/write_doc_tables.py
python3 code/scripts/assemble_docs.py docs_src/RS5_microwave_background_deflating.src.md
bash code/scripts/build_docs.sh supplement/RS5_microwave_background_deflating.md
```

(The times are for two processor cores.)

## 0.2 Symbols

The symbols of this supplement that are new, each defined in words where it first
appears below:

{{GEN:symbols_D9}}

The symbols of RS3 ($G_N$, $a_{\rm sp}$, $H$, $\theta_*$, $z_*$, $\delta_m$, $f_g$,
$\sigma_8$) and of the main registry ($\omega_b$, $\omega_c$, $h$, $H_0$, $A_s$, $n_s$,
$\tau$, $N$, $z_t$, $\Delta\chi^2$, $r_s$, $c_{s0}$, $R_{TF}$) keep their meanings.
*A warning about letters.* The standard reference for the equations of this
supplement, Ma and Bertschinger (Ref. [1]), writes $h$, $\eta$, $\alpha$, $\theta$,
$\sigma$ and $\tau$ for quantities that have other meanings in this project ($h$ is
the Hubble parameter, $\eta$ the flat metric, $\tau$ the optical depth, and so on).
This supplement therefore writes $h_{\rm sy}$, $\eta_{\rm sy}$, $\alpha_{\rm sy}$ and
$\tau_{\rm cf}$, and uses script letters for the sums over matter and radiation.

---

# 1. What must be assumed

## 1.1 The unperturbed universe, and the agent

RS3 makes two assumptions. **A1**: ordinary space $x1,x2,x3$ expands with the scale
factor $a_{\rm sp}(x4)$ and the Hubble rate $H=d\ln a_{\rm sp}/dx4$, while $x5,x6,x7$
deflate at the constant rates $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$ and the
hidden coordinate $x0$ is static. **A2**: the metric obeys Einstein's equations in
eight dimensions with a constant coupling $G_8$, and the component of those equations
along the time contains matter and radiation only. Two consequences follow (RS3,
Sec. 1). Newton's constant of ordinary space is $G_8$ divided by the proper volume of
the extra coordinates and grows as
$$G_N(x4)=G_N(\text{today})\,e^{\,\mathrm{gsum}\,(x4-x4_{\rm today})},\qquad \mathrm{gsum}=\mathrm{g55}+\mathrm{g66}+\mathrm{g77},\tag{1.1}$$
and the expansion obeys
$$3H^2-3\,\mathrm{gsum}\,H+\mathrm{gprod}=8\pi\,G_N(x4)\,\rho,\qquad \mathrm{gprod}=\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77},\tag{1.2}$$
with the density $\rho$ of matter, radiation and the cosmological term.

Something must keep the three rates constant while ordinary space expands and matter
dilutes; left to themselves the extra scale factors would obey equations of their
own. This supplement calls that something *the agent*. Nothing is assumed about its
nature. What A2 says about it is that it has no energy density: otherwise its
density would stand on the right of Eq. (1.2). It cannot be without stresses,
however. The components of Einstein's equations along ordinary space read, for the
unperturbed universe,
$$-\Big(2\,\frac{dH}{dx4}+3H^2-2\,\mathrm{gsum}\,H+\mathrm{gsq}+\mathrm{gprod}\Big)=8\pi G_N\,(P+p_{\rm agent}),\qquad\mathrm{gsq}=\mathrm{g55}^2+\mathrm{g66}^2+\mathrm{g77}^2,$$
with the pressure $P$ of matter and radiation and the pressure $p_{\rm agent}$ of the
agent in ordinary space. Matter is conserved in ordinary space, so that
$d(8\pi G_N\rho)/dx4=\mathrm{gsum}\,8\pi G_N\rho-3H\,8\pi G_N(\rho+P)$ by Eq. (1.1); the
derivative of Eq. (1.2) then gives $P$, and what remains is
$$8\pi G_N\,p_{\rm agent}=-\frac{\mathrm{gsum}}{H}\frac{dH}{dx4}-2\,\mathrm{gsum}\,H+2\,\mathrm{gprod}-\frac{\mathrm{gsum}\,\mathrm{gprod}}{3H}\tag{1.3}$$
(check `Idef-P8`). In an era of matter, where $dH/dx4=-\tfrac32H^2$, this is
$-\mathrm{gsum}\,H/2$ to the first order in the rates: **the agent is a medium without
energy density and with a tension** of the relative size $\mathrm{gsum}/H$. (In an era
of radiation, $dH/dx4=-2H^2$, the first two terms cancel and the pressure vanishes to
this order; when the cosmological term dominates it is $-2\,\mathrm{gsum}\,H$.) (It also has stresses along $x0$
and along the deflating coordinates, which do not enter what follows.) When ordinary
space is perturbed, such a medium will in general be perturbed too.

## 1.2 The third assumption

**A3 — the perturbations.**

* *(i) The extra scale factors are not perturbed.* In a universe with small
  inhomogeneities the scale factors $b_5$, $b_6$, $b_7$ of the deflating coordinates
  remain the prescribed functions of the time, and $x0$ remains static. "The time"
  is the proper time of observers who move with the cold matter (the time coordinate
  of the synchronous gauge, Sec. 2.1). This is the plainest reading of the author's
  formula $g_{55}=\pm\,\mathrm{const5}^2\,e^{-2\,\mathrm{g55}\,x4}$ for the component
  $g_{55}$ of the metric in a universe that is not exactly homogeneous.
* *(ii) The agent has no energy density in the perturbed universe either.* This is
  assumption A2 taken to first order.
* *(iii) A closure.* One more statement about the perturbations of the agent is
  needed (Sec. 3). The hypotheses offer no ground for a choice. The program computes
  with two closures, and every result is marked as independent of the closure or as
  depending on it.

Parts (i) and (ii) are assumptions of this supplement, not hypotheses of the author.
A different part (i) — for instance scale factors that respond to the local density —
would give a different theory of the perturbations, which is not examined here.

---

# 2. The Einstein equations of ordinary space to first order

## 2.1 The perturbed metric

The calculation uses the conformal time $\tau_{\rm cf}$ of ordinary space, defined by
$d\tau_{\rm cf}=dx4/a_{\rm sp}$, and writes a prime for the derivative with respect to
it. The conformal Hubble rate is $H_{\rm cf}=a_{\rm sp}'/a_{\rm sp}=a_{\rm sp}H$. A scalar
perturbation of ordinary space with the comoving wavenumber $k$ along $x3$ is
described, in the synchronous gauge of Ref. [1], by two functions
$h_{\rm sy}(\tau_{\rm cf})$ and $\eta_{\rm sy}(\tau_{\rm cf})$:
$$\begin{aligned}ds^2={}&\eta_{00}\,dx0^2+\eta_{11}\,a_{\rm sp}^2\Big[\big(1-2\eta_{\rm sy}\cos kx3\big)\big(dx1^2+dx2^2\big)+\big(1+(h_{\rm sy}+4\eta_{\rm sy})\cos kx3\big)dx3^2\Big]\\&+\eta_{44}\,a_{\rm sp}^2\,d\tau_{\rm cf}^2+\eta_{55}\sum_{t=5}^{7}b_t^2\,dxt^2 ,\end{aligned}\tag{2.1}$$
where $\eta_{00}$, $\eta_{11}$, $\eta_{44}$, $\eta_{55}$ are the signs of the flat metric
of the spacetime at hand: $+1,+1,-1,-1$ on the 4+4 spacetime and $-1,-1,+1,-1$ on the
7+1 spacetime. The results below are the same for both (checks `I44d-P1`, `I71d-P1`).
The components $g_{55}$, $g_{66}$, $g_{77}$ of the metric are $\eta_{55}\,b_t^2$ with
the scale factors $b_t=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$, which by part (i)
of A3 are not perturbed. "Synchronous" means that the time-time and time-space
components of the metric are not perturbed: the coordinate time is the proper time of
freely falling observers who were at rest initially, which are the particles of cold
matter. The combination
$$\alpha_{\rm sy}=\frac{h_{\rm sy}'+6\,\eta_{\rm sy}'}{2k^2}\tag{2.2}$$
appears repeatedly; the sum of the two gravitational potentials that light feels
(the two potentials of the conformal Newtonian gauge) is
$\mathcal U=\eta_{\rm sy}+\alpha_{\rm sy}'$.

## 2.2 The four equations

Matter and radiation enter through four sums over the species, each multiplied by
$8\pi G_N(x4)\,a_{\rm sp}^2$ with Newton's constant of the moment, Eq. (1.1):

* $\mathcal D$: the perturbation of the energy density;
* $\mathcal T$: the divergence of the momentum density (in Ref. [1]:
  $(\rho+P)\theta$);
* $\mathcal P$: the perturbation of the pressure;
* $\mathcal A$: the anisotropic stress (in Ref. [1]: $(\rho+P)\sigma$).

The Einstein tensor of the metric (2.1) to first order, computed by computer algebra
for arbitrary rates (checks `I44d-P1`, `I71d-P1`), gives

$$\begin{aligned}
\text{(E)}\quad& 2k^2\eta_{\rm sy}-H_e\,h_{\rm sy}'=-\mathcal D, &&\text{energy constraint (time-time)}\\
\text{(M)}\quad& 2k^2\eta_{\rm sy}'=\mathcal T, &&\text{momentum constraint (time-space)}\\
\text{(T)}\quad& h_{\rm sy}''+2H_e\,h_{\rm sy}'-2k^2\eta_{\rm sy}=-3\,\mathcal P, &&\text{trace of the space-space part}\\
\text{(S)}\quad& \alpha_{\rm sy}'+2H_e\,\alpha_{\rm sy}-\eta_{\rm sy}=-\tfrac32\,\mathcal A/k^2, &&\text{traceless space-space part (shear)}
\end{aligned}\tag{2.3}$$

with
$$H_e=H_{\rm cf}-\tfrac12\,a_{\rm sp}\,\mathrm{gsum}.\tag{2.4}$$
These are the equations (21) of Ref. [1] with two changes: Newton's constant is that
of the moment, and $H_{\rm cf}$ is replaced by $H_e$ in (E), (T) and (S). The reason
for the second change can be read off Eq. (1.2). The time-time component of the
Einstein tensor is the sum of the products of all pairs of expansion rates. If the
rate of one ordinary direction is perturbed, the perturbation multiplies the sum of
the rates of all *other* directions: $2H$ in four dimensions, $2H-\mathrm{gsum}$ in
eight dimensions with three deflating coordinates. In the same way the friction term
of a shear, which in four dimensions is the sum $3H$ of all rates, becomes
$3H-\mathrm{gsum}$. The momentum constraint contains no expansion rate and is
unchanged.

The Einstein tensor has perturbed components along $x0$ and along the deflating
coordinates as well (same checks). They are the stresses with which the agent holds
those coordinates, and they are not needed below.

## 2.3 Matter and radiation

Matter and radiation that are spread uniformly over the extra coordinates obey the
four-dimensional equations of motion unchanged: their energy and momentum are
conserved in ordinary space, a photon without momentum along the extra coordinates
follows a geodesic of the metric of ordinary spacetime, and Thomson scattering and
recombination are atomic physics, which the model does not alter (RS3, Sec. 3,
item 6). The program therefore keeps every equation of motion of the main project
for photons, neutrinos, baryons, cold dark matter and the dark fluids (S3, Sec. 2.1;
Ref. [1]) and changes only what multiplies Newton's constant and the expansion rate.
Two relations of the unperturbed universe are used throughout. With
$\mathcal E=8\pi G_N a_{\rm sp}^2\rho$ and $\mathcal E_P=8\pi G_N a_{\rm sp}^2P$, summed
over the species, Eq. (1.2) reads
$$H_{\rm cf}^2-a_{\rm sp}\,\mathrm{gsum}\,H_{\rm cf}+\tfrac13a_{\rm sp}^2\,\mathrm{gprod}=\tfrac13\,\mathcal E ,\tag{2.5}$$
and each species with the equation of state $P=w\rho$ obeys
$$\big(8\pi G_Na_{\rm sp}^2\rho\big)'=\big(a_{\rm sp}\,\mathrm{gsum}-(1+3w)H_{\rm cf}\big)\,8\pi G_Na_{\rm sp}^2\rho ,\tag{2.6}$$
in which the term $a_{\rm sp}\,\mathrm{gsum}$ is the growth of Newton's constant. The
derivative of $H_{\rm cf}$ follows from the two (check `Idef-P2`).

---

# 3. Why the four equations cannot all hold: gaps and closures

## 3.1 Two identities

In general relativity the four equations (2.3) are not independent. Because energy
and momentum of matter are conserved, the derivative of (E) is a combination of (M)
and (T), and the derivative of (M) a combination of (T) and (S): two equations
determine the two functions $h_{\rm sy}$, $\eta_{\rm sy}$, and the other two hold
automatically. The same manipulation on the deflating spacetime shows what is
different. Define the three *gaps*, the amounts by which (M), (T) and (S) fail when
only matter and radiation stand on their right-hand sides:
$$\mathcal Y_M=2k^2\eta_{\rm sy}'-\mathcal T,\qquad
\mathcal Y_T=h_{\rm sy}''+2H_e\,h_{\rm sy}'-2k^2\eta_{\rm sy}+3\,\mathcal P,\qquad
\mathcal Y_S=\alpha_{\rm sy}'+2H_e\,\alpha_{\rm sy}-\eta_{\rm sy}+\tfrac32\,\mathcal A/k^2 .\tag{3.1}$$

*First identity.* Differentiate (E). The derivative of $\mathcal D$ follows from
Eq. (2.6) and from the conservation of energy of each species (Ref. [1], Eq. (30)):
$\mathcal D'=(a_{\rm sp}\,\mathrm{gsum}-H_{\rm cf})\,\mathcal D-3H_{\rm cf}\,\mathcal P-\mathcal T-\tfrac12(\mathcal E+\mathcal E_P)\,h_{\rm sy}'$.
Eliminating $h_{\rm sy}''$ with the definition of $\mathcal Y_T$ and $\eta_{\rm sy}$
with (E) leaves
$$\mathcal Y_M=H_e\,\mathcal Y_T+\mathcal Z,\qquad \mathcal Z=h_{\rm sy}'\,\mathcal B-\tfrac12\,a_{\rm sp}\,\mathrm{gsum}\,(\mathcal D-3\,\mathcal P),\tag{3.2}$$
with a quantity of the unperturbed universe,
$$\mathcal B=H_e'-H_e^2+\tfrac12(\mathcal E+\mathcal E_P)
=\frac{a_{\rm sp}\,\mathrm{gsum}}{2H_e}\Big[\tfrac16\textstyle\sum(1-3w)\,8\pi G_Na_{\rm sp}^2\rho+\tfrac14a_{\rm sp}^2\,\mathrm{gsum}^2-\tfrac13a_{\rm sp}^2\,\mathrm{gprod}\Big] .\tag{3.3}$$
The second form follows from Eqs. (2.5), (2.6) (check `Idef-P2`); in an era of matter
$\mathcal B\approx a_{\rm sp}\,\mathrm{gsum}\,H_{\rm cf}/4$.

*Second identity.* Differentiate $\alpha_{\rm sy}$, Eq. (2.2), and use the conservation
of momentum, $\mathcal T'=(a_{\rm sp}\,\mathrm{gsum}-2H_{\rm cf})\,\mathcal T+k^2\,\mathcal P-k^2\mathcal A$:
$$2k^2\,\mathcal Y_S=\mathcal Y_T+\frac{3}{k^2}\big(\mathcal Y_M'+2H_e\,\mathcal Y_M\big).\tag{3.4}$$

Both identities are verified by computer algebra for a mixture of pressureless
matter, radiation with anisotropic stress, a fluid with a variable equation of state
and a cosmological term (check `Idef-P3`), and numerically along the solutions of the
program (Sec. 7.3).

## 3.2 The consequence

Without deflation $\mathcal B=0$ and $\mathcal Z=0$. Then (3.2) and (3.4) say that if
one gap vanishes at all times, all three do: this is general relativity. With
deflation $\mathcal Z\ne0$, and **the three gaps cannot all vanish**. The equations of
ordinary space with matter and radiation alone are inconsistent at the order
$\mathrm{gsum}$. The inconsistency is not a defect of the calculation: it is the
statement that the agent of Sec. 1.1, which has a tension of the relative size
$\mathrm{gsum}/H$, takes part in the perturbations. The gaps are, up to the factor
$8\pi G_Na_{\rm sp}^2$, the perturbations of its momentum density ($\mathcal Y_M$), of
its pressure ($\mathcal Y_T$) and of its anisotropic stress ($\mathcal Y_S$). A gap of
(E) would be a perturbation of its energy density, which part (ii) of A3 excludes.

The identities are two relations between three gaps. One more relation closes the
system. Three natural choices set one gap to zero:

* **Closure "Momentum"**: $\mathcal Y_M=0$. The agent carries no momentum density.
  Together with (ii), observers who move with the cold matter see neither an energy
  density nor a flow of energy of the agent: it is pure stress. Then
  $$\mathcal Y_T=-\frac{\mathcal Z}{H_e},\qquad \mathcal Y_S=-\frac{\mathcal Z}{2k^2H_e}.$$
  (E) and (M) are the two equations of the program of the main project, so that this
  closure changes that program least.
* **Closure "Pressure"**: $\mathcal Y_T=0$. The agent carries no perturbation of its
  pressure: its density (zero) and its pressure are uniform on the surfaces of
  constant synchronous time, like the scale factors that it holds. Then
  $$\mathcal Y_M=\mathcal Z,\qquad \mathcal Y_S=\frac{3}{2k^4}\big(\mathcal Z'+2H_e\,\mathcal Z\big),$$
  and (M) is replaced by $2k^2\eta_{\rm sy}'=\mathcal T+\mathcal Z$.
* **Closure "Shear"**: $\mathcal Y_S=0$. The agent carries no anisotropic stress, as a
  perfect fluid or a scalar field would not. Eliminating $\mathcal Y_T$ from (3.2)
  and (3.4) gives an equation of motion for the momentum density of the agent,
  $$\mathcal Y_M'=-\Big(\frac{k^2}{3H_e}+2H_e\Big)\mathcal Y_M+\frac{k^2}{3H_e}\,\mathcal Z .\tag{3.5}$$
  Far inside the horizon ($k\gg H_{\rm cf}$) the rate $k^2/3H_e$ is enormous, and
  $\mathcal Y_M$ relaxes at once to $\mathcal Z$, its value in the closure "Pressure":
  **inside the horizon the closure "Shear" is the closure "Pressure"**, with
  corrections of the relative order $(H_{\rm cf}/k)^2$. Because of that rate Eq. (3.5)
  is stiff, and the integrator of the main project cannot follow it at an acceptable
  cost. The closure "Shear" is therefore not computed with the Einstein–Boltzmann
  program; it is integrated for pressureless matter with an implicit method in the
  independent script (check `Idef-P6`, Sec. 4).

All gaps are proportional to $a_{\rm sp}\,\mathrm{gsum}/H_{\rm cf}=\mathrm{gsum}/H$.
For a rate of a few hundredths of $H_0$ that ratio is of the order $10^{-6}$ at
recombination and of the order of the rate itself today (Sec. 7.3). The closure
therefore cannot matter for anything that is decided at recombination, and must
matter for the potentials and the growth of structure of the recent universe.

---

# 4. The growth of pressureless matter

The clearest view of the closures is the density contrast $\delta_m$ of pressureless
matter, for which $\mathcal D=\mathcal E_m\,\delta_m$ with
$\mathcal E_m=8\pi G_Na_{\rm sp}^2\rho_m$, $\mathcal P=\mathcal A=0$, and
$h_{\rm sy}'=-2\,\delta_m'$ (matter at rest in the synchronous frame). Each closure
then gives one ordinary differential equation. For pressureless matter and a
cosmological term it holds for every wavenumber; when radiation is present, far
inside the horizon (check `Idef-P4`).

* *Closure "Pressure".* (T) with (E) gives $h_{\rm sy}''+H_e\,h_{\rm sy}'+\mathcal D=0$, that is
  $$\delta_m''+H_e\,\delta_m'=\tfrac12\,\mathcal E_m\,\delta_m
  \qquad\Longleftrightarrow\qquad
  \frac{d^2\delta_m}{dx4^2}+\Big(2H-\frac{\mathrm{gsum}}{2}\Big)\frac{d\delta_m}{dx4}=4\pi\,G_N(x4)\,\rho_m\,\delta_m .\tag{4.1}$$
  Gravity is that of the moment, and the Hubble friction is reduced by
  $\mathrm{gsum}/2$.
* *Closure "Momentum".* The derivative of (E), with (M) for $\eta_{\rm sy}'$, gives
  $$\delta_m''+\frac{2H_e'+\mathcal E_m+\tfrac43\mathcal E_r}{2H_e}\,\delta_m'=\frac{H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum}}{2H_e}\,\mathcal E_m\,\delta_m .\tag{4.2}$$
  Here the *source* is reduced as well, by the factor
  $(H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum})/(H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum}/2)$.
  (The term $\tfrac43\mathcal E_r$, with $\mathcal E_r=8\pi G_Na_{\rm sp}^2\rho_r$ of the
  radiation, is the momentum density of radiation that streams freely: far inside
  the horizon such radiation has the velocity divergence $-h_{\rm sy}'/2$ (Ref. [1]),
  which enters the momentum constraint (M). Without the term Eq. (4.2) would not
  reduce to the familiar equation when the deflation is switched off.)
* *Closure "Shear".* Far inside the horizon Eq. (4.1); in general no single equation.

Without deflation both reduce to the familiar
$d^2\delta_m/dx4^2+2H\,d\delta_m/dx4=4\pi G_N\rho_m\delta_m$. *With* deflation that
familiar equation, with $G_N(x4)$ inserted, is **neither** (4.1) nor (4.2). An earlier
version of RS3 and of the manuscripts called it "the Newtonian limit of the
eight-dimensional theory" and quoted its results; that was an error (RP9, item
R-E33). For matter alone and three equal rates the three equations can be solved to
the first order in $\mathrm{gsum}/H$: the growth rate $d\ln\delta_m/d\ln a_{\rm sp}$ is
$$\begin{aligned}
&1-\tfrac{9}{16}\,\frac{\mathrm{gsum}}{H}\quad(\text{closure Pressure}),\qquad
1-\tfrac{13}{16}\,\frac{\mathrm{gsum}}{H}\quad(\text{closure Momentum}),\\
&1-\tfrac{11}{16}\,\frac{\mathrm{gsum}}{H}\quad(\text{the four-dimensional equation})
\end{aligned}$$
(check `Idef-P4`; a unit test of the program reproduces the three coefficients to
$10^{-4}$).

For the model of RS3 (three equal rates, the Hubble constant that keeps the angle of
the sound horizon, growth from $a_{\rm sp}=0.01$ to today, relative to no deflation;
command `matter`, check `def-D4`):

{{GEN:tbl_growth}}

(M, P: the closures "Momentum" and "Pressure". Columns 3 to 6: ratios to the model
without deflation at the Hubble constant of column 2, in km/s/Mpc, which keeps the
angle. Columns 7 to 9: the ratio of the amplitudes at the fixed Hubble constant
67.36 km/s/Mpc, for the two closures and for the four-dimensional equation.)

1. *At a fixed Hubble constant* the amplitude of the density contrast today (the
   amplitude $\sigma_8$ for a given primordial amplitude) falls in both closures:
   $d\ln\sigma_8/d(\mathrm{gsum}/H_0)=-0.443$ ("Momentum") and $-0.207$ ("Pressure").
   The four-dimensional equation gives $-0.317$, between the two.
2. *At a fixed angle of the sound horizon* the Hubble constant is lower (Sec. 6), the
   cosmological term takes over later, and matter clusters for longer. The net change
   is $+0.003$ ("Momentum") and $+0.239$ ("Pressure") per unit of
   $\mathrm{gsum}/H_0$: $+0.001$ and $+0.077$ per cent at the rate that lunar laser
   ranging allows, $+0.01$ and $+1.60$ per cent at the rate that nucleosynthesis
   allows.
3. *The growth rate today* $f_g$ rises in both closures: by the factors 1.0021 and
   1.0035 (lunar laser ranging), 1.0463 and 1.0773 (nucleosynthesis).

Common to both closures is that, at a fixed angle, the amplitude does not fall and
the growth rate rises. How large the changes are, and whether the amplitude changes
at all, depends on the closure: **the growth of structure on the deflating spacetimes
is not predicted by the hypotheses until the perturbations of the agent are
specified.**

*Verification.* (a) An independent integration of the three equations reproduces the
numbers of the program to $6.3\times10^{-7}$ (check `Idef-P5`). (b) Integrated as
systems for $\delta_m$ and the metric, with an implicit method and without using
Eqs. (4.1), (4.2), the closures "Momentum" and "Pressure" give growth factors that do
not depend on the wavenumber (to $7\times10^{-14}$), and the closure "Shear" differs
from "Pressure" by 0.30 $(H_0/k)^2$: $1.5\times10^{-6}$ at $k=0.1$ Mpc$^{-1}$ and
$1.5\times10^{-3}$ at $k=0.003$ Mpc$^{-1}$ (check `Idef-P6`). (c) The complete
Einstein–Boltzmann calculation, with photons, neutrinos and baryons, reproduces the
growth of Eqs. (4.1) and (4.2) to $1.3\times10^{-4}$ (Sec. 7.5).

---

# 5. The program

## 5.1 Copies of four modules, with a listed set of changes

The Einstein–Boltzmann program of the main project (S3, Sec. 2; provenance P8) has
Newton's constant built into its units and the ordinary equation of the expansion
built into its routines. The main project is paused and its program must not be
changed. The revision therefore uses *copies* of the four modules that must change —
the background and recombination, the perturbations, the spectra, the fits — and, so
that a copy cannot drift from its original unnoticed, the copies are not edited by
hand: the script `code/scripts/vendor_eb.py` reads the original files and applies a
list of replacements, each of which must match the original text exactly once.
Whatever is not on the list is the original code, character for character. The list
is short:

1. *Expansion.* The Hubble rate is the positive root of Eq. (1.2); a table holds the
   clock $s=\mathrm{gsum}\,(x4-x4_{\rm today})$, which obeys
   $ds/d\ln a_{\rm sp}=\mathrm{gsum}/H$, so that $G_N=G_N(\text{today})\,e^{s}$.
2. *Helium.* At nucleosynthesis Newton's constant was $e^{s}$ times today's value,
   with the value of $s$ in the early universe. At a given temperature the square of
   the expansion rate is proportional to Newton's constant times the density of the
   radiation, which then consists of photons, electrons and positrons, and three
   neutrino species: 10.75 units of which one neutrino species is 7/4. It is
   therefore proportional to $e^{s}\,(\tfrac{43}{7}+\Delta N)$, where $\Delta N$ counts
   any radiation beyond the standard content that is present during nucleosynthesis,
   in units of one neutrino species (for the stepped fluids of the main project it is
   smaller than their density $N$ after the step; S3, Sec. 3.4). For
   nucleosynthesis the smaller Newton's constant is thus equivalent to
   $(\tfrac{43}{7}+\Delta N)(e^{s}-1)$ additional neutrino species, a negative number,
   and the helium mass fraction $Y_{\rm He}$ is taken, with that number, from the same
   fit to the code PArthENoPE that the main project uses (Ref. [3]). This is the
   usual correspondence between the expansion rate and the number of species.
3. *Perturbations.* Every $8\pi G_Na_{\rm sp}^2\rho$ is multiplied by $e^{s}$; (E) uses
   $H_e$; the closure "Pressure" replaces (M) by $2k^2\eta_{\rm sy}'=\mathcal T+\mathcal Z$.
4. *Sources of the temperature.* They contain $\alpha_{\rm sy}'$ and $\alpha_{\rm sy}''$
   (S3, Sec. 2.1). The main project takes $\alpha_{\rm sy}'$ from (S). With deflation
   (S) has a gap in either closure; the program adds the gap $\mathcal Y_S$ of the
   closure, an algebraic expression by Sec. 3.2, and for $\alpha_{\rm sy}''$ the
   derivative of the gap along the solution.

**Without deflation the copies are the original program.** Every changed expression
reduces to the original one operation for operation, and the spectra agree with those
of the main project *to the last bit*: for the fiducial model, for superfluid dark
matter and for a stepped dark fluid (unit tests of `spectra_defl.rs`; checks
`def-E1`, `def-B1`, `def-F1`, `def-U1`). A second unit test runs the program with a
rate of $10^{-30}$ in units of 100 km/s/Mpc, which is too small to change any number
but makes the program take every branch of the deflating model; the potentials,
sources and spectra are again those of the main project to the last bit.

## 5.2 Recombination on the deflating background

Recombination is computed as in the main project (the three-level atom with the
corrections of the code RECFAST; S3, Sec. 2.1), on the expansion history of Eq. (1.2)
and with the helium fraction of item 2. For the three models of the command
`background` of the main project (check `def-B2`):

{{GEN:tbl_eb_background}}

(The models with extra radiation are the fluid without step and the stepped fluid of
the model with $z_t=2\times10^4$, each with $N=0.5$. Column "held": the row keeps
either the Hubble constant of the main project or the angle $\theta_*$ of the same
model without deflation. $H_0$ in km/s/Mpc; $r_s$: sound horizon at last scattering;
"$G_N$ ratio": Newton's constant at last scattering divided by its value today. The
data file contains in addition the drag epoch, its sound horizon, the redshift of
reionization and the age.) Weaker gravity in the past
means slower expansion at a given density. The sound horizon is larger, the angle
$\theta_*$ larger, and the helium fraction lower. Last scattering is slightly later:
the redshift $z_*$ falls, because in a universe that expands more slowly a photon
spends more time, and meets more electrons, in each interval of redshift, so that the
optical depth reaches one at a lower redshift.
The two models with dark radiation respond exactly as the fiducial model does.

*Independent verification.* A separate calculation of recombination with Peebles'
three-level atom, written in Python and sharing no code with the program (check
`Idef-C2`), gives $z_*=1090.84$ without deflation against 1089.92, as expected of the
simpler atomic model, and reproduces the *shifts* caused by the deflation: $-0.551$
against $-0.550$ for $\mathrm{gsum}=0.03\,H_0$ and $-1.882$ against $-1.877$ for
$0.1\,H_0$.

---

# 6. The Hubble constant at a fixed angle

RS3, Sec. 2, infers the Hubble constant by keeping the angle $\theta_*$ of the sound
horizon at last scattering. Its first version held the redshift of last scattering
and the helium fraction at their values without deflation. With the thermal history
of Sec. 5.2 neither is held (command `cosmology`, check `def-C2`; table in RS3,
Sec. 2). The slope at small rates is
$$\frac{d\ln H_0}{d(\mathrm{gsum}/H_0)}=-0.969\qquad(\text{with } z_* \text{ and } Y_{\rm He}\text{ held fixed: }-0.911),$$
and $H_0=66.70$, $65.36$, $60.37$ km/s/Mpc for $\mathrm{gsum}=0.01$, $0.03$,
$0.1\,H_0$ (held fixed: 66.74, 65.48, 60.77). **Holding $z_*$ and $Y_{\rm He}$ fixed
underestimates the fall of the Hubble constant by 5.8 to 6.0 per cent of itself.**
Most of the difference is the shift of last scattering: at a later last scattering
the sound horizon is larger still. The helium fraction accounts for 0.045 km/s/Mpc of
it at the rate that nucleosynthesis allows (check `def-E7`), less than a fifth. The independent calculation of Sec. 5.2 confirms
both sets of numbers: 65.365 and 60.367 km/s/Mpc in the full calculation (program:
65.365, 60.368), 65.483 and 60.773 with last scattering and helium held fixed
(65.483, 60.773). At the bounds of RS3, Sec. 3, the inferred Hubble constant changes
by $-0.211$ km/s/Mpc (lunar laser ranging) and $-4.38$ km/s/Mpc (nucleosynthesis).
The approximation is recorded as item R-E34 of the provenance RP9.

---

# 7. The spectra at a fixed angle

## 7.1 The models

Five rates are computed, each with three equal rates of deflation, with the densities
of baryons and of cold dark matter, the primordial amplitude and tilt and the optical
depth of the fiducial model of the main project, and with the Hubble constant that
keeps $100\,\theta_*=1.03965$ (command `spectrum`, check `def-E2`):

{{GEN:tbl_eb_summary}}

(The rates $0.00324\,H_0$ and $0.0644\,H_0$ are the largest that lunar laser ranging
and nucleosynthesis allow, RS3, Sec. 3. $H_0$ in km/s/Mpc. "$G_N$ ratio": Newton's
constant at last scattering divided by its value today. M, P: the closures "Momentum"
and "Pressure". "Largest change": of $D_\ell^{TT}$ relative to the model without
deflation for $30\le\ell\le2500$. "Ideal": the root of the sum over those multipoles
of the squared change divided by the variance $2/(2\ell+1)$ of a single multipole,
that is, the number of standard deviations by which a measurement of the temperature
on the whole sky without noise would tell the model from no deflation if nothing else
were unknown. $\Delta\chi^2$: against the Planck-like mock data of the main project
with all other parameters fixed. The last three columns are for the closure
"Momentum". The redshift of last scattering and the sound horizon of these models
are in the table of RS3, Sec. 2.)

## 7.2 What remains when the angle is kept

Keeping $\theta_*$ keeps the acoustic peaks in place: the first five move by at most
0.32 in $\ell$.

{{GEN:tbl_eb_peaks}}

(Positions and heights, in $\mu$K$^2$, of the first five maxima of $D_\ell^{TT}$.) What
changes is their height, by at most 0.12 per cent at the rate of lunar laser
ranging and 2.5 per cent at the rate of nucleosynthesis, and the damping tail
(check `def-E3`). The reasons are the same as for any change of the expansion rate at
recombination: a slower expansion changes the ratio of the diffusion length to the
sound horizon, the width of the last-scattering surface, and the decay of the
potentials shortly after recombination; and the lower helium fraction leaves more
electrons per baryon.

* *At the rate that lunar laser ranging allows*, the temperature spectrum differs
  from that of the model without deflation by at most $2.7\times10^{-3}$ of itself,
  0.134 of the cosmic variance of a single multipole; $\Delta\chi^2=1.12$, less than
  the 3.84 of a deviation of two standard deviations. Data of the quality of Planck
  cannot tell this rate from no deflation.
* *At the rate that nucleosynthesis allows* Newton's constant at last scattering was
  0.9383 of today's, the spectrum changes by up to 5.6 per cent, and
  $\Delta\chi^2=468$ before any parameter is refitted. The microwave background is
  then a stronger test than nucleosynthesis (Sec. 8).

*The helium fraction* matters at this level. At the rate of nucleosynthesis the
slower expansion of the early universe is equivalent to $-0.379$ neutrino species, and
the helium fraction falls from 0.24540 to 0.24008. Ignoring this would change the
temperature spectrum by up to $8.6\times10^{-3}$ of itself and $\Delta\chi^2$ from
468.3 to 335.3 (check `def-E7`).

![The spectra of the microwave background at a fixed angle of the sound horizon. (a) The temperature spectrum without deflation and at the largest rate that nucleosynthesis allows. (b), (c) The change of the temperature and polarization spectra relative to the model without deflation for four rates (darker: faster), with the cosmic variance of a single multipole as a band. Closure "Momentum"; at these multipoles the closure "Pressure" gives the same curves. Data: `data/cmb_deflation_cl.csv`, `data/cmb_deflation_summary.csv`; program: `deflation spectrum`.](../figures/fig_spectra.png){width=100%}

## 7.3 The closure

{{GEN:tbl_eb_closure}}

("Difference": the largest difference between the temperature spectra of the two
closures in the range of multipoles, in units of the cosmic variance of a single
multipole. "Share of the effect": the root of the sum of squares of that difference
divided by the root of the sum of squares of the effect of the deflation itself, in
the same range. $\sigma_8$ ratio: to the model without deflation; M, P: the closures
"Momentum" and "Pressure".) Check `def-E4`:

* **For $30\le\ell\le2500$ the closure does not matter.** The spectra of the two
  closures differ by a thousandth of the effect of the deflation ($1.4\times10^{-3}$
  and $1.2\times10^{-3}$ of it at the two bounds), and $\Delta\chi^2$ against the mock
  data is the same to five digits. Everything in Secs. 7.2 and 8 follows from A1, A2
  and the first two parts of A3, whatever the closure.
* **For $\ell<30$ it does.** There the potentials of the recent universe act (the
  late integrated Sachs–Wolfe effect), and the closures differ by more than the
  effect of the deflation in either of them (2.1 times). The largest angular scales
  are not a prediction of the hypotheses. Their cosmic variance is large; at the rate
  of nucleosynthesis the closures differ by at most a third of it.
* **The amplitude of matter** $\sigma_8$ is 0.9762 of its value without deflation in
  the closure "Momentum" and 0.9918 in the closure "Pressure" at the rate of
  nucleosynthesis (0.99892 and 0.99969 at the rate of lunar laser ranging).

The table that follows shows, for the wavenumber $k=0.02$ Mpc$^{-1}$ and five times,
what the agent must supply in each closure (M: "Momentum", P: "Pressure"), and the
numerical checks of the identities of Sec. 3.1 (check `def-E5`). The column
$\mathrm{gsum}/H$ is the ratio $a_{\rm sp}\,\mathrm{gsum}/H_{\rm cf}$. The gaps are
divided by that ratio times the size of the terms of their equations —
$k^2H_{\rm cf}\lvert\mathcal U\rvert$ for (M),
$(k^2+H_{\rm cf}^2)\lvert\mathcal U\rvert$ for (T), $\lvert\mathcal U\rvert$ for (S) —
so that a number of order one means "of the first order in the rate". The data file
has the same for $k=0.005$ and $0.08$ Mpc$^{-1}$.

{{GEN:tbl_eb_gaps}}

For the three wavenumbers of the data file together, and after recombination (the
last four times), the closure "Momentum" lacks 0.54 to 0.98 in the trace equation and
0.31 to 0.49 in the equation of the shear, in these units; in the closure
"Pressure" the momentum constraint lacks 0.62 to 0.94, and the equation of the shear
lacks 2.94 to 4.84 times $(H_{\rm cf}/k)^2$ — far inside the horizon ($k>20\,H_{\rm cf}$)
between $2.5\times10^{-5}$ and $7.0\times10^{-3}$: there the closure "Pressure"
satisfies the equation of the shear as well, as Eq. (3.5) says. The two identities
hold along the numerical solutions to $8.6\times10^{-4}$, which is the accuracy of the
numerical derivative with which they are tested. The ratio
$a_{\rm sp}\,\mathrm{gsum}/H_{\rm cf}=\mathrm{gsum}/H$ is $2.7\times10^{-6}$ at
recombination, $5.5\times10^{-2}$ at the last time of the table and
$6.9\times10^{-2}$ today.

![The closure of the perturbation equations at the largest rate that nucleosynthesis allows. (a) The difference between the temperature spectra of the closures "Momentum" and "Pressure", and the effect of the deflation itself, in units of the cosmic variance of a single multipole: the closure matters below $\ell=30$ only. (b) What the agent supplies: the gaps of the momentum constraint, the trace equation and the equation of the shear, in units of $a_{\rm sp}\,\mathrm{gsum}/H_{\rm cf}$ times the terms of each equation. (c) The growth of the density contrast of matter since $a_{\rm sp}=0.01$ relative to no deflation: the growth equations (4.1), (4.2) (lines), the complete calculation at $k=0.1$ Mpc$^{-1}$ (symbols), and the four-dimensional equation, which is neither closure. Data: `data/cmb_deflation_closure.csv`, `data/cmb_deflation_gaps.csv`, `data/cmb_deflation_growth.csv`, `data/growth_deflation.csv`.](../figures/fig_closure.png){width=100%}

## 7.4 Numerical convergence

Halving all tolerances and all wavenumber steps changes the spectra of the deflating
model with $\mathrm{gsum}=0.03\,H_0$, in either closure, by at most $1.5\times10^{-4}$
(TT), $2.0\times10^{-4}$ (TE, in units of the geometric mean of TT and EE) and
$3.6\times10^{-4}$ (EE) for $\ell\ge100$, and by $1.2\times10^{-3}$,
$3.3\times10^{-3}$, $3.5\times10^{-3}$ for $30\le\ell<100$; $\sigma_8$ changes by
$2.5\times10^{-7}$ (check `def-E6`). These are the numbers of the main project for
its fiducial model ($1.4\times10^{-4}$, $1.8\times10^{-4}$, $3.4\times10^{-4}$;
$1.2\times10^{-3}$, $4.1\times10^{-3}$, $5.9\times10^{-3}$), and its requirements are
met.

## 7.5 The growth of structure in the complete calculation

At the fixed Hubble constant 67.36 km/s/Mpc:

{{GEN:tbl_eb_growth}}

At the Hubble constant that keeps the angle:

{{GEN:tbl_eb_growth_angle}}

(Growth of the density contrast of cold dark matter and baryons at $k=0.1$ Mpc$^{-1}$
from $a_{\rm sp}=0.01$ to today, relative to no deflation. "Program": the complete
Einstein–Boltzmann calculation in the closure "Momentum" (M) or "Pressure" (P); the
columns headed by an equation: the growth equation of that closure. $H_0$ in
km/s/Mpc.) The complete calculation and the growth equation of the same closure
agree to $1.3\times10^{-4}$ for the five rates, at a fixed Hubble constant and at a
fixed angle; with the equation of the *other* closure they would disagree by at
least $0.24\,\mathrm{gsum}/H_0$ (check `def-E8`; independently `Idef-P7`). The
amplitude $\sigma_8$ contains, besides this growth, the change of the scale
$8\,h^{-1}$ Mpc with the Hubble constant and the growth before $a_{\rm sp}=0.01$ under
a weaker gravity.

---

# 8. What mock data say about the rate

## 8.1 The question, and the method

S3, Sec. 3.3, asks how far the Hubble constant can be raised by dark radiation before
the fit to microwave-background data degrades, and answers with mock data: the
spectra of the fiducial model stand in for the measurements (so that the fiducial
model fits with $\Delta\chi^2=0$), with the noise and sky coverage of the Planck
satellite, and of Planck together with a deep survey of a quarter of the sky. The
same question is put here to the deflation: *given such data, how large can
$\mathrm{gsum}$ be, and what happens to the Hubble constant?* The likelihood, the two
data sets, the six parameters ($\omega_b$, $\omega_c$, $h$, the amplitude $A_s$ and
tilt $n_s$ of the primordial spectrum, the optical depth $\tau$) and the fitting
procedure are those of the main project, used from its library unchanged (check
`def-F1`: its Fisher errors are reproduced to $3.1\times10^{-11}$). The rate is the
seventh parameter. Because the rates are positive by hypothesis, the derivative of
the spectra with respect to $\mathrm{gsum}$ at zero rate is one-sided.

## 8.2 The Fisher matrix

{{GEN:tbl_fc_fisher}}

("Error of the rate": the standard error of $\mathrm{gsum}/H_0$ with the six
parameters free. "Bound": 1.96 times that error. "Six held": the error if the six
parameters are held fixed. "Slope": the change of the best-fit Hubble constant in
km/s/Mpc per unit of $\mathrm{gsum}/H_0$. Last two columns: the standard error of the
Hubble constant in km/s/Mpc with the rate held at zero, and with the rate free. M, P:
the closures. Checks `def-F2`, `def-F3`.) With the six parameters free, Planck-like data determine
$\mathrm{gsum}/H_0$ to 0.01963 (one standard deviation), and the upper bound at 95 per
cent — 1.96 standard deviations, for a parameter that cannot be negative and whose
best value is zero — is
$$\begin{aligned}
\mathrm{gsum}&<0.0385\,H_0=2.65\times10^{-12}\ \text{per year}&&(\text{Planck-like}),\\
\mathrm{gsum}&<0.0207\,H_0&&(\text{with the deep survey}).
\end{aligned}$$
With the six parameters *held* the error would be 0.00108: nearly all of the effect
of the deflation on the spectra — above all the shift of the angle $\theta_*$ — can
be absorbed by the other parameters, and only one part in eighteen of the
sensitivity survives. The closure changes the error by half a per cent
(0.01963 against 0.01974).

**The bound lies between those of RS3**: it is 11.9 times weaker than that of lunar
laser ranging ($0.00324\,H_0$) and 1.7 times stronger than that of nucleosynthesis
($0.0644\,H_0$). Unlike lunar laser ranging it tests the rate at early times, and
unlike both it is a forecast for mock data, not a measurement.

**The Hubble constant falls along the degeneracy**: by 84.35 km/s/Mpc per unit of
$\mathrm{gsum}/H_0$ for the Planck-like data and 82.11 with the deep survey, that is
by 3.2 and 1.7 km/s/Mpc at the respective bounds. (At a fixed angle and fixed
densities the slope is 65.24, Sec. 6; in the fit the density of dark matter rises and
the tilt falls as well.) The error of the Hubble constant grows from 0.561 to
1.748 km/s/Mpc when the rate is free — entirely towards smaller values, since the
rate cannot be negative.

## 8.3 Fits at fixed rate

{{GEN:tbl_fc_profile}}

(Check `def-F4`. Each row is a fit of the six parameters at the given rate; $H_0$ in
km/s/Mpc; "$G_N$ ratio": Newton's constant at last scattering divided by its value
today. The data file contains all parameters of each fit.) For
rates up to $0.01\,H_0$ the minimum $\Delta\chi^2$ is 1.00 to 1.01 times the value
$(\mathrm{gsum}/\text{error})^2$ of the Fisher matrix, and the shift of the Hubble
constant 1.00 to 1.01 times its slope: the Fisher matrix describes the fits. The
profile reaches $\Delta\chi^2=3.84$ at $\mathrm{gsum}=0.0376\,H_0$ (Planck-like) and
$0.0205\,H_0$ (with the deep survey), against the bounds 0.0385 and 0.0207 of the
Fisher matrix. At the rate of lunar laser ranging the best fit has
$\Delta\chi^2=0.027$ and $H_0=67.086$ km/s/Mpc. At the rate of nucleosynthesis it has
$\Delta\chi^2=11.6$ (3.4 standard deviations) and $H_0=61.70$ km/s/Mpc for the
Planck-like data, and $\Delta\chi^2=39.8$ (6.3 standard deviations) with the deep
survey: **the largest rate that nucleosynthesis allows would be excluded by data of
the quality of Planck.**

*Accuracy* (check `def-F5`). Recomputing the derivatives of the spectra at the best
fit for $\mathrm{gsum}=0.02\,H_0$ and continuing the fit changes $\Delta\chi^2$ by less
than 0.001 and the Hubble constant by 0.001 km/s/Mpc. Refitting with the closure
"Pressure" changes $\Delta\chi^2$ by at most 0.9 per cent and the Hubble constant by
0.0005 km/s/Mpc.

![Mock data of the microwave background against the rate of deflation. (a) The minimum $\Delta\chi^2$ of fits of the six parameters at fixed rate (symbols) and the parabola of the Fisher matrix (lines), for the two data sets; the rates that lunar laser ranging and nucleosynthesis allow are marked. (b) The best-fit Hubble constant, with the slope of the Fisher matrix and, dotted, the Hubble constant that keeps the angle when nothing else is refitted. (c) The temperature spectrum at $\mathrm{gsum}=0.02\,H_0$ relative to the data: with the angle kept and nothing refitted, and at the best fit, which absorbs most of the difference; the band is the cosmic variance of a single multipole. Data: `data/deflation_profile.csv`, `data/deflation_fisher.csv`, `data/deflation_residuals.csv`, `data/hubble_deflation.csv`; program: `deflation forecast`.](../figures/fig_forecast.png){width=100%}

---

# 9. The dark-radiation families of the main project on the deflating background

S3, Sec. 3, fits seven one-parameter families of extra radiation of the density $N$
(in units of one neutrino species) to the mock data: free-streaming radiation, a
fluid, the stepped fluid of the model at four transition redshifts $z_t$, and the
Wess–Zumino stepped fluid. (The tables below use the names of S3: "model fluid" is
the stepped fluid of the model, "Wess–Zumino fluid" that of the Wess–Zumino model.)
The command `families` repeats the complete calculation —
Fisher matrices, fits at eight values of $N$ for both data sets, the check of the
derivatives, the scan of the transition redshift — with three equal rates of
deflation at the largest total rate that lunar laser ranging allows,
$\mathrm{gsum}=0.00324\,H_0$. The data remain the spectra of the fiducial model
without deflation, so that $\Delta\chi^2$ is counted from a model that has neither
deflation nor extra radiation. A second part, Sec. 9.3, treats the rate and the
density $N$ as free parameters together.

## 9.1 The Fisher matrix at the rate of lunar laser ranging

The deflating model *without* extra radiation fits the Planck-like data with
$\Delta\chi^2=0.027$ at $H_0=67.086$ km/s/Mpc, and the data with the deep survey with
$\Delta\chi^2=0.094$ at $H_0=67.094$ km/s/Mpc (check `def-N1`; the first pair is the
fit of Sec. 8.3). The derivatives of the spectra with respect to the six parameters
and to $N$ are taken at that best fit, on the deflating background.

{{GEN:tbl_fam_fisher}}

("Error of $N$": one standard deviation with the six parameters free. $dH_0/dN$: the
change of the best-fit Hubble constant in km/s/Mpc per unit of $N$. "No deflation":
the main project, file `data/dr_fisher.csv`.) The errors of $N$ agree with those of
the main project to 0.2 per cent for the Planck-like data and to 0.3 per cent with
the deep survey: at this rate the deflation does not change what the data say about
extra radiation (check `def-N2`).

## 9.2 Fits at a fixed density of extra radiation

{{GEN:tbl_fam_profiles}}

(Each row is a fit of the six parameters at the fixed density $N$; $H_0$ in km/s/Mpc;
"shift": deflating minus not deflating. The data file has eight values of $N$ for
each family.) Three things can be read off (check `def-N2`).

1. *Every Hubble constant is lower by about a quarter of a km/s/Mpc.* For all seven
   families and all eight densities the best-fit Hubble constant on the deflating
   background lies 0.274 to 0.279 km/s/Mpc below that of the main project for the
   Planck-like data, and 0.266 to 0.270 km/s/Mpc below it with the deep survey. The
   model without extra radiation is lower by 0.274 and 0.266 km/s/Mpc: this is what
   the slopes of Sec. 8.2 give for this rate. The effects of the deflation and of the
   extra radiation on the Hubble constant add.
2. *The fits are slightly better than without deflation.* The deflating model
   without extra radiation fits worse than the model that produced the data. Yet for
   every family and every density $N\ge0.1$ the minimum $\Delta\chi^2$ on the
   deflating background is *lower* than in the main project, by up to 3.11 at $N=1$
   for the Planck-like data and by up to 8.06 with the deep survey. The reason is
   given in Sec. 9.3.
3. *The measures of the main project.* S3, Sec. 3.4, summarizes each family by the
   density at which the best fit reaches $\Delta\chi^2=4$, by the Hubble constant
   there, and by the $\Delta\chi^2$ at which the best-fit Hubble constant reaches the
   local measurement of 73.50 km/s/Mpc. On both backgrounds:

{{GEN:tbl_fam_summary_planck}}

{{GEN:tbl_fam_summary_deep}}

(First table: Planck-like mock data; second table: with the deep survey. The
interpolation is that of the main project: between the fitted densities the square
root of $\Delta\chi^2$ and the Hubble constant are linear in $N$; an asterisk marks a
linear extrapolation beyond $N=1$. $H_0$ in km/s/Mpc.) Because the fits are slightly
better, slightly more extra radiation is allowed at $\Delta\chi^2=4$; the Hubble
constant reached there is nevertheless lower than in the main project, and the local
value costs the same or more. For free-streaming radiation and the Planck-like data:
$N=0.31$ instead of 0.29, $H_0=69.62$ instead of 69.74 km/s/Mpc, and
$\Delta\chi^2=26$ instead of 25 at the local value. **At the rate that lunar laser
ranging allows, the deflation changes none of the conclusions of the main project
about its dark radiation, and what it changes goes against a larger Hubble
constant.**

## 9.3 The rate and the extra radiation as free parameters together

Why are the fits with extra radiation better on the deflating background? The answer
is in the Fisher matrix with eight parameters — the six of the standard model, the
density $N$ and the rate — at the fiducial model (check `def-N4`):

{{GEN:tbl_fam_joint_planck}}

{{GEN:tbl_fam_joint_deep}}

(First table: Planck-like mock data; second table: with the deep survey. Errors: one
standard deviation of $N$ and of $\mathrm{gsum}/H_0$, with the other of the two held
at zero or free; the six parameters are free throughout. "Correlation": the
correlation coefficient of the two. "$N$ per unit rate": the best-fitting density at
a fixed rate, divided by $\mathrm{gsum}/H_0$; it is the correlation coefficient times
the error of $N$ divided by the error of the rate, both with both free. "Net slope": the
change of the best-fit Hubble constant, in km/s/Mpc per unit of $\mathrm{gsum}/H_0$,
when $N$ follows its best-fitting value: the slope of Sec. 8.2 plus $dH_0/dN$ times
the preceding column. "Largest rise of $H_0$": the largest increase of the best-fit
Hubble constant, in km/s/Mpc, among all models with $N\ge0$, $\mathrm{gsum}\ge0$ and
$\Delta\chi^2\le3.84$; in every row it is reached at zero rate.)

1. *Consistency.* With the rate held at zero the errors of $N$ are those of the main
   project, to $2.6\times10^{-11}$.
2. *The density of extra radiation and the rate are positively correlated* for every
   family: the correlation coefficient is 0.37 to 0.89 for the Planck-like data and
   0.28 to 0.93 with the deep survey. Extra radiation makes the early universe
   expand faster; the weaker gravity of the deflating spacetime makes it expand more
   slowly; in the spectra the two compensate in part. With both free, the error of
   $\mathrm{gsum}/H_0$ from the Planck-like data grows from 0.0196 to between 0.0211
   and 0.0429, depending on the family, and the error of $N$ grows by the same
   factor.
3. *This is why the fits of Sec. 9.2 are better.* For two correlated parameters
   $\Delta\chi^2$ contains a term proportional to the product of $N$ and the rate,
   with a negative sign. At a fixed positive rate it lowers $\Delta\chi^2$ in
   proportion to $N$, and the best-fitting density is no longer zero: at the rate
   that lunar laser ranging allows it is $N=0.005$ to 0.023, with
   $\Delta\chi^2=0.006$ to 0.023 instead of 0.027 (Planck-like). The fits confirm the
   matrix: at $N=0.1$ and 0.2 the differences between the minimum $\Delta\chi^2$ of
   Sec. 9.2 and that of the main project are 0.99 to 1.01 times the term of the
   Fisher matrix.
4. *The compensation does not help the Hubble constant.* When the rate grows and $N$
   follows its best-fitting value, extra radiation raises the Hubble constant and
   the deflation lowers it. The sum, the column "net slope", is negative for every
   family and both data sets: $-35.3$ km/s/Mpc per unit of $\mathrm{gsum}/H_0$ for
   free-streaming radiation and the Planck-like data, against $-84.35$ without extra
   radiation. The largest Hubble constant with $\Delta\chi^2\le3.84$ is therefore
   reached at *zero* rate in all fourteen cases: a rise of 0.78 to 2.95 km/s/Mpc for
   the Planck-like data. **The deflation lowers the Hubble constant faster than it
   makes room for extra radiation; it cannot help the dark radiation of the model,
   or any of the other families, to raise the Hubble constant.**

## 9.4 Accuracy

Recomputing the derivatives of the spectra at the best fit for $N=0.5$ of each family
and continuing the fit changes $\Delta\chi^2$ by at most 0.054 and the Hubble constant
by at most 0.050 km/s/Mpc. The scan of the transition redshift of the stepped fluid
of the model at $N=0.4$, from $z_t=10^{3}$ to $10^{6}$, has the shape that it has in
the main project, with Hubble constants lower by 0.272 to 0.277 km/s/Mpc (check
`def-N3`; files `data/dr_deflation_chord_check.csv`, `data/dr_deflation_zt_scan.csv`).
The threshold 3.84 is that of a deviation of 1.96 standard deviations in one
parameter, here the Hubble constant; Sec. 9.3 rests on the Fisher matrix, which
Sec. 9.2 confirms for densities up to 0.2 and at the rate of lunar laser ranging
only.

![The dark-radiation families of the main project on the deflating background. (a) The best-fit Hubble constant against the density of extra radiation for four families: main project (lines) and deflating background at the rate that lunar laser ranging allows (symbols). (b) The difference for all seven families; the horizontal line is the shift of the model without extra radiation. (c) The rate and the extra radiation as free parameters together: the regions with $\Delta\chi^2<3.84$ of the Fisher matrix with eight parameters for two families, lines of constant change of the best-fit Hubble constant for free-streaming radiation (dotted), the point of the largest Hubble constant of each region (filled symbols, at zero rate), and the rate that lunar laser ranging allows (dashed). Planck-like mock data. Data: `data/dr_deflation_profiles.csv`, `data/dr_deflation_joint.csv`, and `data/dr_profiles.csv` of the main project; program: `deflation families`.](../figures/fig_families.png){width=100%}

---

# 10. Superfluid dark matter on the deflating background

S3, Secs. 4.1 to 4.3, asks what becomes of the growth of structure if the dark matter
is the superfluid itself, a fluid with first sound, in two limits: a degenerate gas of
fermions of mass $m$ with eight internal states (weak pairing) and a condensate of
bound pairs with the sound speed $c_{s0}$ today. Pressure suppresses structure below
a wavenumber; the *half mode* is the wavenumber at which the power falls to a quarter
of that of cold dark matter, and the bound of the Lyman-$\alpha$ forest on thermal
warm dark matter, translated into a half mode, limits $m$ from below and $c_{s0}$
from above. The command `superfluid` repeats the calculation on four backgrounds: no
deflation; the rate of lunar laser ranging; the rate of nucleosynthesis in each of
the two closures. Each background has the Hubble constant that keeps the angle.

{{GEN:tbl_sfdm_half}}

{{GEN:tbl_sfdm_bounds}}

(First table: the half-mode wavenumber, in Mpc$^{-1}$, of fifteen models on the four
backgrounds; M, P: the closures "Momentum" and "Pressure"; "change": of the column
"nucleosynthesis rate, M" relative to no deflation. Second table: the limits that
follow when the half mode is required not to lie below that of a thermal relic of 3.5
or 5.3 keV (column "relic"), the bounds of the Lyman-$\alpha$ forest that the main
project uses. "Half mode": that of the relic. "Fermion mass": the smallest allowed
mass $m$. The largest allowed sound speed of a condensate today is given in units of
the speed of light and in m/s. $R_{TF}$ is the radius of the core that a condensate
with that sound speed would form, with Newton's constant of today.)

1. *Without deflation* the copy of the program reproduces the two files of the main
   project to $3.4\times10^{-11}$ (check `def-U1`).
2. *The suppression moves to slightly smaller wavenumbers.* The half-mode wavenumber
   falls by 0.15 to 0.16 per cent at the rate that lunar laser ranging allows and by
   3.0 to 3.3 per cent at the rate that nucleosynthesis allows, for all fifteen models
   (check `def-U2`). The reason is the weaker gravity of the time at which the
   suppression is set, long before today. The Jeans wavenumber, below which pressure
   wins over gravity, is proportional to the square root of Newton's constant, and in
   the early universe Newton's constant was 0.9969 and 0.9383 of its present value at
   the two rates: the square roots differ from one by $-0.154$ and $-3.13$ per cent.
3. *The limits move accordingly.* For the benchmark of 5.3 keV the smallest fermion
   mass moves from 1.514 keV without deflation to 1.516 keV at the rate of lunar laser
   ranging and 1.555 keV at the rate of nucleosynthesis; the largest sound speed of a
   condensate from $1.645\times10^{-11}\,c$ to $1.641\times10^{-11}\,c$ and
   $1.565\times10^{-11}\,c$.
4. *The closure does not matter:* the two closures give the same half-mode
   wavenumbers to $1.5\times10^{-5}$. The suppression is set long before the closure
   can act (Sec. 3.2).

*Two cautions.* (a) The benchmark wavenumbers are those of the standard model, in
which the bounds of the Lyman-$\alpha$ forest were derived. At the rate of
nucleosynthesis the expansion rate at the redshifts of the forest differs from that of
the standard model by a few per cent as well, and with it the relation between
wavenumbers and the velocities that the forest measures. The last rows of the second
table therefore show the size of the effect; they are not limits of the same standing
as those of the main project. At the rate that lunar laser ranging allows both
changes are below two tenths of a per cent. (b) The equation of state of the dark
fluid in ordinary space is that of the main project. For the degenerate gas it
follows from the density and the mass of the fermions alone. For the condensate it
contains the strength of a contact interaction. Whether that strength, seen from
ordinary space, would itself follow the volume of the deflating coordinates, as the
couplings of RS1 do, depends on the microscopic model; if it grew at the rate
$\mathrm{gsum}$, the sound speed at a given density would have been smaller in the
past, which acts against the shift found here. The calculation does not decide this.

![Superfluid dark matter on the deflating background. (a) The power spectrum of matter relative to cold dark matter for two fermion masses and two sound speeds of a condensate: no deflation (lines) and the largest rate that nucleosynthesis allows (symbols). (b), (c) The change of the half-mode wavenumber for the fermions and for the condensate on the three deflating backgrounds. Data: `data/sfdm_deflation_transfer.csv`, `data/sfdm_deflation_summary.csv`; program: `deflation superfluid`.](../figures/fig_sfdm.png){width=100%}

---

# 11. What has been established, and what has not

**Established**, each under the assumptions named:

1. the Einstein equations of ordinary space to first order with unperturbed extra
   scale factors, Eqs. (2.3), (2.4), for arbitrary rates and both signatures
   (assumption A3, part (i); computer algebra);
2. that these equations are inconsistent with matter alone at the order
   $\mathrm{gsum}$, the two identities (3.2), (3.4) between the gaps, and the three
   closures; that the closure "Shear" coincides with "Pressure" inside the horizon
   (A1, A2, A3);
3. the growth equations (4.1), (4.2), and that the four-dimensional equation with
   $G_N(x4)$ is neither;
4. that the copies of the Einstein–Boltzmann program reproduce the main project to
   the last bit without deflation;
5. the Hubble constant at a fixed angle with recombination on the deflating
   background, $d\ln H_0/d(\mathrm{gsum}/H_0)=-0.969$ (A1, A2), confirmed by an
   independent calculation of recombination;
6. the spectra for $\ell\ge30$ at five rates, their independence of the closure, and
   the forecast $\mathrm{gsum}<0.0385\,H_0$ (Planck-like), $0.0207\,H_0$ (with a deep
   survey) (A1, A2, parts (i) and (ii) of A3);
7. that the Hubble constant falls in every fit;
8. for the seven dark-radiation families of the main project, at the rate that lunar
   laser ranging allows: best-fit Hubble constants lower by about a quarter of a
   km/s/Mpc, and slightly better fits (A1, A2, parts (i) and (ii) of A3; fits with the
   complete calculation);
9. that the density of extra radiation and the rate of deflation are positively
   correlated, and that the largest Hubble constant with $\Delta\chi^2\le3.84$ is
   reached at zero rate for every family (Fisher matrix, confirmed by the fits at the
   rate of lunar laser ranging);
10. the shift of the scale of suppression of superfluid dark matter, and its
    independence of the closure (A1, A2, A3; with the two cautions of Sec. 10).

**Not established.**

1. *The agent.* Nothing in the hypotheses says what keeps the rates constant, and
   therefore nothing says which closure holds, or whether any of the three does. The
   growth of structure, the amplitude $\sigma_8$ and the spectra below $\ell=30$ are
   computed for two closures and differ at the first order in $\mathrm{gsum}/H_0$.
   They are the quantities that a theory of the agent would have to supply.
2. *Part (i) of A3.* That the extra scale factors do not respond to inhomogeneities is
   an assumption. In a theory in which they are dynamical fields, they would be
   perturbed, and the equations of Sec. 2 would contain additional terms.
3. *The theory of gravity.* Everything rests on A2. If gravity is of the
   Einstein–Gauss–Bonnet kind in which the deflating spacetimes with equal rates are
   vacuum solutions (DMD, Sec. 1.4a), Eq. (1.1) and with it this whole supplement
   would have to be derived anew (RS3, Sec. 9).
4. *Real data.* Sections 8 and 9 are forecasts for mock data without gravitational
   lensing. No measured spectrum has been fitted.
5. *A varying rate.* The rates are constant throughout cosmic history in every
   calculation.
6. *Nonlinear structure.* Only linear perturbations are computed.

---

# Appendix A. Complete output of the programs

## A.1 `deflation background`

```text
{{FILE:data/background_checks.txt}}
```

## A.2 `deflation spectrum`

```text
{{FILE:data/spectrum_checks.txt}}
```

## A.3 `deflation forecast`

```text
{{FILE:data/forecast_checks.txt}}
```

## A.4 `deflation families`

```text
{{FILE:data/families_checks.txt}}
```

## A.5 `deflation superfluid`

```text
{{FILE:data/superfluid_checks.txt}}
```

# Appendix B. Files

| file | content | written by |
|:---|:------------|:---|
| `code/scripts/vendor_eb.py` | writes the four copied modules from the sources of the main project | — |
| `code/deflation-rs/src/eb_cosmo.rs`, `eb_boltzmann.rs`, `eb_spectra.rs`, `eb_fit.rs` | the Einstein–Boltzmann program on the deflating spacetime (copies; not edited by hand) | `vendor_eb.py` |
| `code/deflation-rs/src/spectra_defl.rs` | assumption A3 in words; shared routines; nine unit tests | — |
| `code/deflation-rs/src/cmd_spectra.rs` | the commands `background`, `spectrum`, `forecast`, `families`, `superfluid` | — |
| `code/deflation-rs/src/cosmo_defl.rs` | the growth equations of the closures | — |
| `data/background_deflation.csv` | Sec. 5.2 | `deflation background` |
| `data/hubble_deflation.csv` | Sec. 6 | `deflation cosmology` |
| `data/growth_deflation.csv` | Sec. 4 | `deflation matter` |
| `data/cmb_deflation_summary.csv`, `cmb_deflation_cl.csv`, `cmb_deflation_pk.csv`, `cmb_deflation_peaks.csv` | Secs. 7.1, 7.2 | `deflation spectrum` |
| `data/cmb_deflation_closure.csv`, `cmb_deflation_gaps.csv` | Sec. 7.3 | `deflation spectrum` |
| `data/cmb_deflation_convergence.csv`, `cmb_deflation_growth.csv` | Secs. 7.4, 7.5 | `deflation spectrum` |
| `data/deflation_fisher.csv`, `deflation_profile.csv`, `deflation_chord_check.csv`, `deflation_residuals.csv` | Sec. 8 | `deflation forecast` |
| `data/dr_deflation_fisher.csv`, `dr_deflation_profiles.csv`, `dr_deflation_chord_check.csv`, `dr_deflation_residuals.csv`, `dr_deflation_zt_scan.csv`, `dr_deflation_joint.csv` | Sec. 9 | `deflation families` |
| `data/sfdm_deflation_transfer.csv`, `sfdm_deflation_summary.csv`, `sfdm_deflation_bounds.csv` | Sec. 10 | `deflation superfluid` |
| `data/background_checks.txt`, `spectrum_checks.txt`, `forecast_checks.txt`, `families_checks.txt`, `superfluid_checks.txt` (and `.csv`) | Appendix A | the five commands |
| `data/deflating_checks_independent.txt` | checks `I44d-P1`, `I71d-P1`, `Idef-P2` to `Idef-P8`, `Idef-C2` | `verify_deflating_independent.py` |
| `data/mutation_test_independent.txt` | sixteen deliberate errors in the independent verification, and the check that detects each (provenance RP8, Sec. 4) | `mutation_test_independent.py` |
| `figures/fig_spectra`, `fig_closure`, `fig_forecast`, `fig_families`, `fig_sfdm` (`.pdf`, `.png`) | the five figures | `make_figures.py` |

# References

1. C.-P. Ma and E. Bertschinger, *Cosmological perturbation theory in the synchronous and conformal Newtonian gauges*, Astrophys. J. **455**, 7 (1995).
2. P. J. E. Peebles, *Recombination of the primeval plasma*, Astrophys. J. **153**, 1 (1968); S. Seager, D. D. Sasselov and D. Scott, *A new calculation of the recombination epoch*, Astrophys. J. **523**, L1 (1999).
3. Planck Collaboration, *Planck 2015 results. XIII. Cosmological parameters*, Astron. Astrophys. **594**, A13 (2016), Eq. (67) (fit to the code PArthENoPE).
4. M. Viel, J. Lesgourgues, M. G. Haehnelt, S. Matarrese and A. Riotto, *Constraining warm dark matter candidates including sterile neutrinos and light gravitinos with WMAP and the Lyman-α forest*, Phys. Rev. D **71**, 063534 (2005).
5. The supplement S3 of the main project and its references; the supplement RS3 of the revision and its references (bounds on the variation of Newton's constant).
