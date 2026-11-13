#!/usr/bin/env python3
"""write_symbol_registry.py (Revision_Deflation) -- the unique symbols of the revision.

Hard Rule 5 of the project requires unique symbol definitions.  The MASTER registry of
the main project (../provenance/symbols.csv, 116 entries) remains in force for every
symbol that the revision takes over unchanged.  This script holds the list of the
symbols that the revision ADDS, and writes it as

    provenance/symbols_deflation.csv              the registry (machine-readable)
    provenance/generated/symbols_deflation.md     the registry as Markdown tables (fragments)

Edit the list SYMBOLS below, then run (from the folder Revision_Deflation)

    python3 code/scripts/write_symbol_registry.py
    python3 code/scripts/check_symbols.py

Columns of an entry: group; symbol as written in the documents (LaTeX); names in
programs and data files (blank-separated); meaning; where defined (DMD = "dirac matrices
deflating", RS1..RS5 = supplements of the revision, RP0 = hypotheses, M = manuscripts);
origin ("author" = the author's instruction of November 12, 2026; "new" = introduced by
the revision; "main" = a symbol of the main registry whose meaning the revision extends).
"""
import csv
import os

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

G1 = "D1 Hypotheses and geometry"
G2 = "D2 Matrices, fields and their equations"
G3 = "D3 Solutions mode by mode"
G4 = "D4 The deflation clock and the couplings"
G5 = "D5 Dynamics of the paired state"
G6 = "D6 Cosmology and bounds"
G7 = "D7 Symmetries and vortex lines"
G8 = "D8 Tags and identifiers"
G9 = "D9 Perturbations of ordinary space and the microwave background"

SYMBOLS = [
    # ---- D1 ------------------------------------------------------------------------------------
    (G1, r"$\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$", "g55 g66 g77",
     r"the three rates of deflation of $x5$, $x6$, $x7$ (positive constants, dimension 1/length). These are the author's names; they are written upright and never with a subscript. They are *not* components of the metric: the components are $g_{55}$, $g_{66}$, $g_{77}$",
     "RP0 1; DMD 0.1; M I", "author"),
    (G1, r"$\mathrm{rate}_t$", "rate_t rate",
     r"the rate of the coordinate $xt$ in a statement that holds for $t=5,6,7$ alike: $\mathrm{rate}_5=\mathrm{g55}$, $\mathrm{rate}_6=\mathrm{g66}$, $\mathrm{rate}_7=\mathrm{g77}$",
     "DMD 0.1; M I", "new"),
    (G1, r"$\mathrm{gsum}$", "gsum gam",
     r"$\mathrm{g55}+\mathrm{g66}+\mathrm{g77}$, the total rate: the proper volume of the three deflating coordinates falls as $e^{-\mathrm{gsum}\,x4}$ (`gam`: gsum in units of 100 km/s/Mpc)",
     "DMD 1.1; M II", "new"),
    (G1, r"$\mathrm{gsq}$, $\mathrm{gprod}$", "gsq gprod",
     r"$\mathrm{g55}^2+\mathrm{g66}^2+\mathrm{g77}^2$ and $\mathrm{g55}\,\mathrm{g66}+\mathrm{g55}\,\mathrm{g77}+\mathrm{g66}\,\mathrm{g77}$; $\mathrm{gsum}^2=\mathrm{gsq}+2\,\mathrm{gprod}$",
     "DMD 1.3; M II", "new"),
    (G1, r"$\mathrm{const5}$, $\mathrm{const6}$, $\mathrm{const7}$; $\mathrm{const}_t$", "const5 const6 const7 const_t cst",
     r"the three positive constants of the author's metric (his word: const): the values of the scale factors at $x4=0$",
     "RP0 1; DMD 0.1; M I", "author"),
    (G1, r"$b_5$, $b_6$, $b_7$; $b_t$", "b_5 b_6 b_7 b_t",
     r"the scale factors of the deflating coordinates, $b_t(x4)=\mathrm{const}_t\,e^{-\mathrm{rate}_t\,x4}$: a coordinate distance $dxt$ is the proper length $b_t\,\lvert dxt\rvert$. Always written with the index 5, 6, 7 or $t$ (the letter $b$ without index is a Ginzburg-Landau coefficient of the main registry)",
     "DMD 1.1; M II", "new"),
    (G1, r"$V_d$", "Vd",
     r"$b_5b_6b_7=\sqrt{\lvert\det g\rvert}$, the proper volume of a unit coordinate cell of the deflating coordinates; $V_d\propto e^{-\mathrm{gsum}\,x4}$",
     "DMD 1.1; RS1 1; M II", "new"),
    (G1, r"$g_{1644d}$, $g_{1671d}$; $g_{\mu\nu}$", "g_1644d g_1671d",
     r"the metrics of the deflating 4+4 and 7+1 spacetimes, $\mathrm{diag}(\eta_{00},\dots,\eta_{44},\eta_{55}b_5^2,\eta_{66}b_6^2,\eta_{77}b_7^2)$ with $\eta=\eta_{1644}$ or $\eta_{1671}$; in both, $g_{tt}=-\mathrm{const}_t^2e^{-2\,\mathrm{rate}_t\,x4}$",
     "DMD 1.1; M II", "author"),
    (G1, r"$\mu,\nu,\rho,\sigma$ (indices)", "mu nu rho sigma lam",
     r"coordinate indices with the values $0,\dots,7$ on the deflating spacetimes (only as sub- or superscripts; as stand-alone letters $\mu$, $\nu$, $\lambda$ keep the meanings of the main registry). The capital indices $A,B,C,D$ of the main registry, also with a prime, label the orthonormal frame: the frame vector $A$ points along the coordinate $xA$ (as a stand-alone letter $C$ is the coupling of the main registry)",
     "DMD 1.1; M II", "new"),
    (G1, r"$t$, $s$ (indices)", "",
     r"indices of the three deflating coordinates, values 5, 6, 7: $t$ as in the main registry, and $s$ as a second such index where two are needed ($s_h$ with the index $h$ are matrices of the main registry)",
     "DMD 0.1, 1.3; M I, II", "main"),
    (G1, r"$a,b$; $j$ (indices)", "",
     r"$a,b$ as in the main registry, with the range stated at each place: in RS1 to RS4 and in the manuscripts the index of the four extra axes, values 0, 5, 6, 7 (on the deflating spacetimes a frame index); in DMD and in the statements of the checks of the programs the index of the four flat coordinates other than the time, values 0, 1, 2, 3. In the manuscripts of the revision, where both sets of coordinates occur, the flat coordinates other than the time carry the index $j$ with the range $0,1,2,3$ stated there; a sum over $j$ with other limits (the ordinary space $1,2,3$ of the main registry; in DMD Eq. (3.12) the seven space-like directions) states them",
     "DMD 0.1; RS1 5.1; M I", "main"),
    (G1, r"$e_A{}^\mu$, $e^A{}_\mu$", "e_A^mu",
     r"the orthonormal frame and its inverse: $e^A{}_\mu=\mathrm{diag}(1,1,1,1,1,b_5,b_6,b_7)$",
     "DMD 1.1; M II", "new"),
    (G1, r"$\Gamma^\rho{}_{\mu\nu}$", "Gamma christoffel",
     r"the Christoffel symbols of the deflating metric (three indices; the letter $\Gamma$ with square brackets is the effective action of the main registry)",
     "DMD 1.2; M II", "new"),
    (G1, r"$R^\rho{}_{\sigma\mu\nu}$, $R^\mu{}_\nu$, $R$", "riemann ricci Ric44 Ric55 Ric66 Ric77 Rscalar",
     r"Riemann tensor, Ricci tensor and curvature scalar of the deflating metric",
     "DMD 1.3; M II", "new"),
    (G1, r"$\mathcal G^\mu{}_\nu$", "Ein00 Ein44 Ein55 Ein66 Ein77 einstein_mixed",
     r"the Einstein tensor $R^\mu{}_\nu-\tfrac12R\,\delta^\mu_\nu$ (script letter, so that it cannot be taken for Newton's constant or for a constant of the Fermi type)",
     "DMD 1.4; M II", "new"),
    (G1, r"$\mathcal X$; $\mathcal X_{\rm scale}$, $\mathcal X_{(s)}$", "killing_vectors killing_count",
     r"a Killing vector field of the deflating metric (script letter; the plain letter $X$ is the set of coordinates of the main registry); the scaling vector $\partial_4+\sum_t\mathrm{rate}_t\,xt\,\partial_t$; the three special Killing vectors of the case of equal rates, $s=5,6,7$",
     "DMD 1.5, 1.6; M II, App. B", "new"),
    (G1, r"$\bar x4$; $\bar x5$, $\bar x6$, $\bar x7$", "",
     r"coordinates of the case of equal rates in which the line element of $x4,\dots,x7$ is a multiple of a flat one: $\bar x4=e^{\mathrm{g55}\,x4}/\mathrm{g55}$, $\bar xt=\mathrm{const}_t\,xt$",
     "DMD 1.6; M App. B", "new"),
    (G1, r"$\mathcal H^\mu{}_\nu$; $\alpha_{GB}$, $\Lambda_8$", "Lanczos alpha_GB Lambda_8",
     r"the Lanczos tensor of the Gauss-Bonnet term (script letter; $H$ is the Hubble rate); the coupling of the Gauss-Bonnet term and the cosmological constant of Einstein-Gauss-Bonnet gravity in eight dimensions (with the index 8; $\Lambda$ without index is the momentum cutoff of the main registry)",
     "DMD 1.4a; M II", "new"),
    (G1, r"$K_{AB}$", "ksec",
     r"the sectional curvature of the plane of the frame directions $A$ and $B$, in the convention in which the space-like components of the metric are positive: $K_{4t}=\mathrm{rate}_t^2$, $K_{ts}=\mathrm{rate}_t\,\mathrm{rate}_s$ for $t\neq s$, zero if one direction is flat. Always with two subscripts: the letter $K$ without subscript is the generator of the main registry, and $K$ with an order as subscript and with an argument is a modified Bessel function",
     "DMD 1.4a; M App. B", "new"),
    (G1, r"$\varsigma$", "affine",
     r"affine parameter along a geodesic; for a massive particle on the 7+1 spacetime, the proper time divided by the mass",
     "DMD 1.7; M II", "new"),
    (G1, r"$P_\mu$", "P_1 P_5",
     r"the conserved covariant momenta of a geodesic (with an index; $P$ without index is the generator of the main registry)",
     "DMD 1.7; M II", "new"),
    (G1, r"$x4_{\rm turn}$", "x4_turn",
     r"the value of the ordinary time at which a classical particle with momentum along a deflating time turns back, and beyond which a mode that depends on a deflating time is extinguished (4+4): there $k_d=E_\perp$",
     "DMD 1.7, 4.3; M II (4+4)", "new"),
    # ---- D2 ------------------------------------------------------------------------------------
    (G2, r"$\gamma_{1644d}^\mu$, $\gamma_{1671d}^\mu$", "gamma_1644d gamma_1671d",
     r"the Dirac matrices of the deflating spacetimes in coordinate components, $\gamma^\mu=e_A{}^\mu\gamma^A$: $\gamma^t=\gamma^{A=t}/b_t$ for $t=5,6,7$, the other five are the constant matrices $\gamma_{1644}^A$, $\gamma_{1671}^A$ of the main project",
     "DMD 2.2; M III", "new"),
    (G2, r"$\omega_{\mu AB}$", "omega_(mu A B)",
     r"the coefficients of the spin connection (three indices; $\omega_{1644}$, $\omega_{1671}$ with a tag are the volume elements of the main registry)",
     "DMD 2.3; M III", "new"),
    (G2, r"$\Omega_\mu$", "Omega_mu omega",
     r"the spin connection as a matrix, $\tfrac14\omega_{\mu AB}\gamma^A\gamma^B$; the covariant derivative of a spinor is $\nabla_\mu\psi=(\partial_\mu+\Omega_\mu)\psi$",
     "DMD 2.3; M III", "new"),
    (G2, r"$\psi_{1644d}$, $\psi_{1671d}$", "dirac1644d dirac1671d",
     r"the fields dirac1644d and dirac1671d: the 16-component fields of the main project on the deflating spacetimes; $(\gamma^\mu\nabla_\mu-m)\psi_{1644d}=0$ and $(i\gamma^\mu\nabla_\mu-m)\psi_{1671d}=0$",
     "DMD 3.1, 3.2; M III", "new"),
    (G2, r"$\hat\psi$", "psihat",
     r"the weighted field $\sqrt{V_d}\,\psi$, which obeys the equation without spin connection",
     "DMD 3.3; M III", "new"),
    (G2, r"$K(x4)$, $\mathsf h(x4)$", "K h",
     r"the generator $K_{1644}$ and the Hamiltonian matrix $\mathsf h_{1671}$ of the main registry with the wavenumbers along the deflating coordinates divided by the scale factors: $\partial_4\hat\psi=K\hat\psi$ (4+4), $i\partial_4\hat\psi=\mathsf h\hat\psi$ (7+1)",
     "DMD 3.4; M III", "main"),
    # ---- D3 ------------------------------------------------------------------------------------
    (G3, r"$k_5$, $k_6$, $k_7$; $k_t$", "k_5 k_6 k_7 k_t",
     r"the conserved wavenumbers of a mode along the deflating coordinates (covariant components of the wave vector $k_A$ of the main project)",
     "DMD 3.4; M III", "new"),
    (G3, r"$E_\perp$", "E_perp",
     r"the energy of a mode, or of a classical particle, without its motion along the deflating coordinates: $\sqrt{m^2+k_0^2+k_1^2+k_2^2+k_3^2}$ (with the wavenumbers or momenta along $x0,\dots,x3$)",
     "DMD 1.7, 3.4; M II, III", "new"),
    (G3, r"$k_d$", "k_d",
     r"the physical wavenumber along the deflating coordinates, $k_d(x4)=\big[\sum_tk_t^2/b_t(x4)^2\big]^{1/2}$; it grows as the coordinates deflate",
     "DMD 3.4; M III", "new"),
    (G3, r"$\mathsf h_\perp$, $K_\perp$", "h_perp K_perp",
     r"the parts of $\mathsf h$ and of $K$ that do not contain the scale factors: the operators of the modes that do not depend on the deflating coordinates, $\mathsf h_\perp=m\beta+\sum_ap_a\alpha^a$ with $\mathsf h_\perp^2=E_\perp^2$, and $K_\perp=-m\gamma^4+i\sum_ak_a\gamma^4\gamma^a$ with $K_\perp^2=-E_\perp^2$",
     "DMD 4.1; M III", "new"),
    (G3, r"$\gamma^d$, $\alpha^d$", "",
     r"the frame matrix along the direction of the wave vector in the deflating coordinates, $\gamma^d=k_d^{-1}\sum_t(k_t/b_t)\gamma^t$, and $\alpha^d=\gamma^4\gamma^d$; constant in time when one rate governs",
     "DMD 4.1; M III", "new"),
    (G3, r"$w$; $f_+$, $f_-$; $Z_+$, $Z_-$", "",
     r"auxiliary objects of the exact solutions (DMD 4.2 and 4.3; $w$, $Z_+$, $Z_-$ also in Sec. IV of the manuscripts): a constant eigenvector of $\mathsf h_\perp$ or of $K_\perp$; the two coefficient functions of the ansatz; two cylinder functions whose orders differ by one",
     "DMD 4.2, 4.3; M IV", "new"),
    (G3, r"$u_d$", "u_d",
     r"$k_d/\mathrm{rate}_t$ for a mode that depends on one deflating coordinate: the argument of the Bessel functions of the exact solutions",
     "DMD 4.1; M IV", "new"),
    (G3, r"$n_{\rm cr}$", "n_cr n_min n_mean n_max n_exact fd5 fd6",
     r"the number of particles per internal state that the deflation creates in a mode that starts empty (7+1)",
     "DMD 4.2; M IV (7+1)", "new"),
    (G3, r"$T_{\rm defl}$", "",
     r"the temperature of the thermal spectrum of the created quanta, $\mathrm{rate}_t/2\pi$ in units with $\hbar=k_B=1$ (7+1)",
     "DMD 4.2; M IV (7+1)", "new"),
    (G3, r"$S_{\rm WKB}$", "S_wkb",
     r"$\int\sqrt{k_d^2-E_\perp^2}\,dx4$ from the turning value on: exponent of the extinction of a bounded solution (4+4)",
     "DMD 4.3; M IV (4+4)", "new"),
    (G3, r"$\varphi_{\rm bos}$, $\hat\varphi_{\rm bos}$, $m_{\rm bos}$", "phi_bos Shat Shat2 m_bos",
     r"a mode of the Vorton-Scalar (the radial mode, $m_{\rm bos}=m_s$, or the phase mode, $m_{\rm bos}=0$), its weighted form $\sqrt{V_d}\,\varphi_{\rm bos}$, and its mass",
     "RS1 2; M V", "new"),
    (G3, r"$\varpi_b$", "varpi_b",
     r"$\sqrt{k^2+m_{\rm bos}^2-\mathrm{gsum}^2/4}$: the frequency of the weighted boson mode that does not depend on the deflating coordinates",
     "RS1 2; M V", "new"),
    # ---- D4 ------------------------------------------------------------------------------------
    (G4, r"$\tau_d$", "tau_d",
     r"the deflation clock, $\tau_d=\mathrm{gsum}\,(x4-x4_{\rm ref})$: the number of e-foldings by which the volume $V_d$ has shrunk since the reference time (the letter $\tau$ without the index $d$ is the optical depth of the main registry)",
     "RS1 1; M V", "new"),
    (G4, r"$\tau_{\rm pair}$", "tau_pair",
     r"the clock measured from the moment at which two fermions become bound: $\ln(D_{VS}/D_c)$",
     "RS1 4; RS4 1; M VII", "new"),
    (G4, r"$\hat\Phi_{VS}$", "Phihat",
     r"the weighted Vorton-Scalar $\sqrt{V_d}\,\Phi_{VS}$",
     "RS1 1; M V", "new"),
    # ---- D5 ------------------------------------------------------------------------------------
    (G5, r"$\Xi_d$", "Xi_d",
     r"$\mathrm{gsum}/(\lambda\,\Delta_{\rm eq})$: decides whether the paired state can follow the deflation ($\Xi_d\ll1$: yes)",
     "RS1 7; M VIII", "new"),
    (G5, r"$n_k$", "",
     r"occupation of the pair of states of the single-particle energy $\xi_k$ in the shell model of the paired state (with the index $k$; the letter $n$ without index is a density)",
     "RS1 7; M VIII", "new"),
    (G5, r"$\Delta_{\rm eq}$", "gap_eq",
     r"the equilibrium gap for the pairing strength of the moment",
     "RS1 7; M VIII", "new"),
    (G5, r"$E_{\rm sh}$", "E_sh",
     r"half-width in energy of the shell of states around the Fermi surface in the model of the pair dynamics; unit of energy and of inverse time there",
     "RS1 7; M VIII", "new"),
    (G5, r"$n_{\rm ex}$", "n_ex n_ex_final",
     r"density of broken pairs (in units of $E_{\rm sh}$ times the density of states)",
     "RS1 7; M VIII", "new"),
    # ---- D6 ------------------------------------------------------------------------------------
    (G6, r"$G_N$, $G_8$", "G_N GN_early GN_star G_8",
     r"Newton's constant of ordinary space, and the gravitational coupling of the eight-dimensional Einstein theory (assumption A2): $G_N=G_8/(\text{proper volume of the extra coordinates})\propto e^{\mathrm{gsum}\,x4}$",
     "RS3 1; M XI", "new"),
    (G6, r"$a_{\rm sp}$, $H$", "",
     r"scale factor of ordinary space (assumption A1) and its rate $H=(da_{\rm sp}/dx4)/a_{\rm sp}$; $H_0$, $h$ as in the main registry. (The author's Einstein-Lovelock notebooks use the letter $H$ for the rate in the exponent of their metric; that rate corresponds here to $\mathrm{g55}$, $\mathrm{g66}$, $\mathrm{g77}$)",
     "RS3 1; M XI", "new"),
    (G6, r"$t_0$", "age age_Gyr",
     r"the age of the universe in the model at hand",
     "RS3 2; M XI", "new"),
    (G6, r"$\theta_*$, $z_*$", "theta_star z_star",
     r"the angle under which the sound horizon at last scattering is seen, $r_s(z_*)/D_A(z_*)$ with the comoving distance $D_A$, and the redshift $z_*$ of last scattering at which the optical depth without reionization is 1 (the $100\,\theta_*$ of the Planck papers). The angle $\theta_s$ of the main registry is the same ratio at the maximum of the visibility function; for the model without deflation $100\,\theta_*=1.03965$ and $100\,\theta_s=1.040358$",
     "RS3 1, 2; M XI", "new"),
    (G6, r"$\delta_m$, $f_g$, $\sigma_8$", "growth f_growth growth_ratio f_growth_ratio f_sigma8_ratio",
     r"density contrast of matter in ordinary space; its growth rate $d\ln\delta_m/d\ln a_{\rm sp}$ today; the amplitude of clustering (only ratios of $\sigma_8$ are used)",
     "RS3 5; M XI", "new"),
    (G6, r"$v_{\rm EW}$ (main project), $n_{\rm EW}$", "v_EW n_EW",
     r"the electroweak scale of the supplement S2 of the main project, and the exponent with which it would follow the coupling under the hypothesis H2: $n_{\rm EW}=d\ln v_{\rm EW}/d\ln\alpha_{VS}$",
     "RS2 5; M XII", "new"),
    (G6, r"$\varrho_c$", "rho_c",
     r"abbreviation in the formula for $n_{\rm EW}$ of the scale-invariant ladder equation: $\varrho_c=\tfrac12\sqrt{\alpha_{VS}/\alpha_c-1}$ (the letter $\rho$ without index is a density)",
     "RS2 5; M XII", "new"),
    (G6, r"$m_p/m_e$", "mu_pe",
     r"the ratio of the proton mass to the electron mass (always written as this ratio in the documents; `mu_pe` in the statements of the programs)",
     "RS2 5; RS3 3; M XI", "new"),
    (G6, r"$G_{\rm Fermi}$, $\alpha_{\rm em}$", "G_Fermi",
     r"the Fermi constant of the weak interaction and the fine-structure constant (they enter only in the discussion of bounds)",
     "RS3 1, 3; M XI", "new"),
    # ---- D7 ------------------------------------------------------------------------------------
    (G7, r"$P_c$", "Pc",
     r"$i\alpha^1\alpha^2\alpha^3\chi$, the product of the five matrices of the sector that does not depend on the extra coordinates; equal, up to sign, to the generator $P$ of $U(1)_E$ of the main registry",
     "RS1 6; M X", "new"),
    (G7, r"$\theta_{12}$", "",
     r"the angle around a vortex line that lies along $x3$, in the plane of $x1$ and $x2$ (with the index 12; $\theta$ with other indices and the scattering angle keep the meanings of the main project)",
     "RS1 6; M X", "new"),
    (G7, r"$\mathsf M$ (main registry)", "M_gap",
     r"the $8\times8$ gap matrix of the main registry; `M_gap` is its spelling in the statements of the programs of the revision",
     "RS1 5; M X", "main"),
    (G7, r"$w_z$", "w_z",
     r"a constant spinor of a fermion zero mode on a vortex line",
     "RS1 6; M X", "new"),
    # ---- D8 ------------------------------------------------------------------------------------
    (G8, r"1644d, 1671d", "1644d 1671d",
     r"tags of the deflating 4+4 and 7+1 spacetimes and of every object defined on them (the main project's tags 1644, 1671 with the letter d)",
     "DMD 0.1; M I", "new"),
    (G8, r"`44d-..`, `71d-..`, `def-..`", "",
     r"identifiers of the machine checks of the revision: on the deflating 4+4 spacetime, on the deflating 7+1 spacetime, and for results that hold on both (`44-..`, `71-..` without d are the checks of the main project, executed again)",
     "DMD Appendix A", "new"),
    (G8, r"RP0 to RP9, RS1 to RS5, DMD", "",
     r"the provenances and supplements of the revision and the document \"dirac matrices deflating\" (P0 to P11, S1 to S4, DM are the documents of the main project)",
     "README", "new"),
    (G8, "A1, A2, A3, H2, HV", "",
     r"A1, A2: the two assumptions of the cosmological part (ordinary space expands; gravity is the eight-dimensional Einstein theory). A3: the assumption about the perturbations of ordinary space (the scale factors of the extra coordinates are not perturbed, and the agent that holds them has no energy density), with its closures. H2: the author's hypothesis of the main project that the superfluid state is the Higgs field. HV: the different assumption that the vacuum value of the Vorton-Scalar sets the electroweak scale",
     "RP0 3; RS2 1; RS5 1; M XI, XII", "new"),
    (G8, r"\"Momentum\", \"Pressure\", \"Shear\" (closures)", "Momentum Pressure Shear closure",
     r"the three closures of the perturbation equations under assumption A3, named after the Einstein equation of ordinary space that holds with matter and radiation alone besides the energy constraint: the momentum constraint, the trace (pressure) equation, or the equation of the shear. In data files the suffixes `_M` and `_P` mark the first two; `_N4` marks the four-dimensional growth equation, which is none of them",
     "RS5 3; M XI", "new"),
    # ---- D9 ------------------------------------------------------------------------------------
    (G9, r"$\tau_{\rm cf}$", "tau_cf tau0_Mpc",
     r"the conformal time of ordinary space, $d\tau_{\rm cf}=dx4/a_{\rm sp}$, in Mpc (`tau0_Mpc`: its value today). In the equations of the perturbations a prime is the derivative with respect to $\tau_{\rm cf}$. (The letter $\tau$ without index is the optical depth of the main registry, and $\tau_d$ is the deflation clock)",
     "RS5 2; M XI", "new"),
    (G9, r"$H_{\rm cf}$, $H_e$", "Hc He hub hub_e a_gsum_over_Hc",
     r"the conformal Hubble rate $H_{\rm cf}=a_{\rm sp}H$, and $H_e=H_{\rm cf}-a_{\rm sp}\,\mathrm{gsum}/2$, which takes its place in three of the Einstein equations of ordinary space when the extra coordinates deflate",
     "RS5 2; M XI", "new"),
    (G9, r"$k$ (perturbations), $\ell$, $D_\ell^{TT}$, $D_\ell^{TE}$, $D_\ell^{EE}$", "",
     r"the comoving wavenumber of a perturbation of ordinary space (in Mpc$^{-1}$; the physical wavenumber is $k/a_{\rm sp}$); the multipole; and the spectra of the temperature, of the polarization and their cross spectrum, $D_\ell=\ell(\ell+1)C_\ell/2\pi$ in $\mu$K$^2$, as in the supplement S3 of the main project",
     "RS5 2, 7; M XI", "new"),
    (G9, r"$h_{\rm sy}$, $\eta_{\rm sy}$, $\alpha_{\rm sy}$", "",
     r"the two functions that describe a scalar perturbation of the metric of ordinary space in the synchronous gauge (the $h$ and $\eta$ of Ma and Bertschinger; here with the index sy, because $h$ is the Hubble parameter and $\eta$ the flat metric), and the combination $\alpha_{\rm sy}=(h_{\rm sy}'+6\eta_{\rm sy}')/2k^2$",
     "RS5 2; M XI", "new"),
    (G9, r"$\mathcal U$", "potential",
     r"$\eta_{\rm sy}+\alpha_{\rm sy}'$, the sum of the two gravitational potentials of the conformal Newtonian gauge (script letter): the potential that light feels",
     "RS5 2, 7", "new"),
    (G9, r"$\mathcal D$, $\mathcal T$, $\mathcal P$, $\mathcal A$", "d_sum three_dpres",
     r"$8\pi G_N a_{\rm sp}^2$ times the perturbation of the density, of the divergence of the momentum density, of the pressure, and of the anisotropic stress of matter and radiation together (script letters; in the notation of Ma and Bertschinger: $\delta\rho$, $(\rho+P)\theta$, $\delta P$, $(\rho+P)\sigma$)",
     "RS5 2", "new"),
    (G9, r"$\mathcal E$, $\mathcal E_P$, $\mathcal E_m$, $\mathcal E_r$, $\mathcal B$", "grho_sum half_rho_plus_p b_trace n_bg",
     r"$8\pi G_N a_{\rm sp}^2$ times the density and the pressure of the unperturbed universe, summed over the species ($\mathcal E_m$, $\mathcal E_r$: the parts of $\mathcal E$ from pressureless matter and from radiation); and $\mathcal B=H_e'-H_e^2+\tfrac12(\mathcal E+\mathcal E_P)$, which vanishes without deflation (script letters)",
     "RS5 2, 3, 4", "new"),
    (G9, r"$p_{\rm agent}$", "",
     r"the pressure in ordinary space of the agent, that is, of whatever keeps the three rates of deflation constant, in the unperturbed universe (under assumption A2 the agent has no energy density)",
     "RS5 1", "new"),
    (G9, r"$\mathcal Y_M$, $\mathcal Y_T$, $\mathcal Y_S$; $\mathcal Z$", "momentum_gap trace_gap shear_gap x_gap shear_check trace_check",
     r"the gaps: what the momentum constraint, the trace equation and the equation of the shear lack when only matter and radiation stand on their right-hand sides; the agent that holds the extra scale factors supplies them. $\mathcal Z=h_{\rm sy}'\,\mathcal B-\tfrac12a_{\rm sp}\,\mathrm{gsum}\,(\mathcal D-3\mathcal P)$ (script letters)",
     "RS5 3", "new"),
    (G9, r"$Y_{\rm He}$", "",
     r"the mass fraction of helium after nucleosynthesis (with the index He; `Y_He` in the data files, as in the main project)",
     "RS5 5; M XI", "new"),
    (G9, r"$\tau$ (main registry)", "tau_reio",
     r"the optical depth of reionization of the main registry; `tau_reio` is its name in the data files that the revision writes (the bare name `tau` is not used there)",
     "RS5 8", "main"),
]


def main():
    out_csv = os.path.join(ROOT, "provenance", "symbols_deflation.csv")
    os.makedirs(os.path.dirname(out_csv), exist_ok=True)
    with open(out_csv, "w", newline="", encoding="utf-8") as f:
        f.write("# PROJECT spacetimes, Revision_Deflation: registry of the symbols that the revision adds to the MASTER registry of the main project (../provenance/symbols.csv)\n")
        f.write("# written by: code/scripts/write_symbol_registry.py (edit the list there); verified by: code/scripts/check_symbols.py\n")
        f.write("# columns: group; symbol as written in the documents (LaTeX); names in programs and data files (blank-separated); meaning; where defined (DMD = dirac matrices deflating, RS1..RS5 = supplements, RP0 = provenance of the hypotheses, M = the two manuscripts of the revision, whose sections carry the same numbers in both; each followed by the number of the section); origin (author = the author's instruction; new = introduced by the revision; main = extends a symbol of the main registry)\n")
        w = csv.writer(f)
        w.writerow(["group", "symbol", "program_names", "meaning", "defined", "origin"])
        for row in SYMBOLS:
            w.writerow(row)
    gen = os.path.join(ROOT, "provenance", "generated")
    os.makedirs(gen, exist_ok=True)
    groups = []
    for row in SYMBOLS:
        if row[0] not in groups:
            groups.append(row[0])
    with open(os.path.join(gen, "symbols_deflation.md"), "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_symbol_registry.py; do not edit -->\n\n")
        # one fragment with all groups, and one per group
        def table(rows):
            lines = ["| symbol | in programs and data files | meaning | defined in | origin |", "|:---|:---|:------------|:---|:---|"]
            for g, sym, prog, meaning, where, origin in rows:
                progs = ", ".join("`%s`" % p for p in prog.split()) if prog else "—"
                lines.append("| %s | %s | %s | %s | %s |" % (sym, progs, meaning, where, origin))
            return "\n".join(lines)
        f.write("<!-- BEGIN symbols_all -->\n")
        for g in groups:
            f.write("**%s**\n\n" % g)
            f.write(table([r for r in SYMBOLS if r[0] == g]) + "\n\n")
        f.write("<!-- END symbols_all -->\n\n")
        for i, g in enumerate(groups, 1):
            f.write("<!-- BEGIN symbols_D%d -->\n%s\n<!-- END symbols_D%d -->\n\n" % (i, table([r for r in SYMBOLS if r[0] == g]), i))
    print("wrote provenance/symbols_deflation.csv (%d entries) and provenance/generated/symbols_deflation.md" % len(SYMBOLS))


if __name__ == "__main__":
    main()
