#!/usr/bin/env python3
"""
write_symbol_registry.py -- the registry of the symbols of PROJECT spacetimes.

The registry is the list REGISTRY below.  This script writes it to
provenance/symbols.csv, the file that code/scripts/check_symbols.py verifies and from
which the tables of the MASTER provenance (provenance/P1_master_symbols) are made.
Edit the list here, then run (from the repository root)

    python3 code/scripts/write_symbol_registry.py
    python3 code/scripts/check_symbols.py

Each entry:  (group, symbol in documents (LaTeX), names in programs and data files
(separated by blanks; empty if the symbol occurs in documents only), meaning, where it
is defined, origin).

  * "defined in" uses the abbreviations DM = the provenance "dirac matrices",
    S1 ... S4 = the supplements, M = the two manuscripts (section numbers of the PDF).
  * "origin" names the original file and the original symbol if the object was taken
    from a notebook (N01 ... N14, see provenance/generated/notebook_symbols.csv) or a
    paper (D01 ... D09, see provenance/P2_pdfs_used); "new" if the project introduced it.

Rule for names in files: a name that carries the tag 1644 or 1671, or the suffix VS,
is a symbol of the project and must appear here; check_symbols.py enforces this.
"""
import csv
import os
import re

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))

G_COORD = "1 Coordinates and indices"
G_44 = "2 Matrices of the 4+4 spacetime (tag 1644)"
G_71 = "3 Matrices of the 7+1 spacetime (tag 1671)"
G_FIELD = "4 Fields and their equations"
G_SYM = "5 Symmetries, pairs and order parameter"
G_VS = "6 The Vorton-Scalar (suffix VS)"
G_PAIR = "7 Pairing, gap equation and CJT effective action"
G_MAT = "8 Matter, temperature and vacuum"
G_COS = "9 Cosmology"

REGISTRY = [
    # ------------------------------------------------------------------ coordinates and indices
    (G_COORD, r"$X=\{x0,\dots,x7\}$", "x0 x1 x2 x3 x4 x5 x6 x7",
     "the eight coordinates of both spacetimes, never renamed: $x0$ hidden space-like; $x1,x2,x3$ ordinary space; $x4$ ordinary time; $x5,x6,x7$ additional coordinates (time-like in 4+4, space-like in 7+1)",
     "DM 0.1; M I E", "N01 (text at its head) and N04 `X` (section DEFCHART): the same eight names with the same meaning. N02 uses the same eight names with `x0` as the time: related by swap04"),
    (G_COORD, r"$\mathrm{swap}_{04}$", "swap04",
     "the exchange of the index values 0 and 4; it carries the labels of `7+1.nb` (time at index 0) into those of the project (time at index 4)",
     "DM 0.1, 2.1", "new"),
    (G_COORD, r"$A,B,C,D$", "",
     "index with the values $0,\\dots,7$; always labels the coordinate $xA$", "DM 0.1; M I E", "new"),
    (G_COORD, r"$a,b$", "",
     "in S1 to S4 and in the manuscripts: index of the four extra axes, values 0, 5, 6, 7. In DM Sec. 1.8.4 and in the texts of the checks `44-D6`, `44-D7`: the four space-like axes of 4+4, values 0, 1, 2, 3 (stated there)",
     "M I E; S1 0.1; DM 1.8.4", "new"),
    (G_COORD, r"$t$ (index)", "",
     "index of the three additional coordinates, values 5, 6, 7", "DM 1.8.4; M I E", "new"),
    (G_COORD, r"$i,j$", "",
     "index of space-like coordinates; values 1, 2, 3 (ordinary space) unless a sum is said to run over all space-like coordinates of the spacetime",
     "M I E", "new"),
    (G_COORD, r"$h$ (index)", "",
     "index with the values 1, 2, 3 that labels the $4\\times4$ matrices $s_h$, $t_h$", "DM 1.1; M II A", "N01, N02, N05 `h` in `s4by4[h]`, `t4by4[h]`"),

    # ------------------------------------------------------------------ 4+4 matrices
    (G_44, r"$\eta_{1644}$", "eta1644",
     "metric of the 4+4 spacetime, $\\mathrm{diag}(+1,+1,+1,+1,-1,-1,-1,-1)$: space-like $=+1$", "DM 1.4, Eq. (1.3)", "N01, N02, N05, N10–N12 `η4488`; in N01 also `η8` and `g_AB`; D08 $G_{AB}$ with the index value 8 in the place of the value 0"),
    (G_44, r"$s_h$, $t_h$", "",
     "the three self-dual and the three anti-self-dual real antisymmetric $4\\times4$ matrices", "DM 1.1; M II A (4+4), App. A (7+1)", "N01, N02, N05, N10–N12 `s4by4[h]`, `t4by4[h]`; the letters are those of D06 ($s^h$ self-dual, $t^h$ anti-self-dual)"),
    (G_44, r"$\tau_{1644}^A$, $\bar\tau_{1644}^A$", "tau1644 taubar1644",
     "the real $8\\times8$ blocks of the 4+4 Dirac matrices (reduced generators of the split octonion algebra)", "DM 1.1, App. B; M II A",
     "N01, N10–N12 `τ[A]` and `τ` with an overbar; N02 `τ[A]`, `τbar[A]`; matrices of this kind are the $\\tau^A$, $\\bar\\tau^A$ of D07 and D08, whose index runs from 1 to 8 (the value 8 there is the value 0 here)"),
    (G_44, r"$\sigma_{88}$", "sigma88",
     "the $8\\times8$ matrix with the $4\\times4$ blocks $(0,1_4;\\,1_4,0)$", "DM 1.1", "N01 `σ`, `σ88`; N02 `σ`; N05 `σ88`"),
    (G_44, r"$\gamma_{1644}^A$", "gamma1644",
     "the eight real $16\\times16$ Dirac matrices of the 4+4 spacetime", "DM 1.1, 1.2; M II A", "N01, N10–N12 `T16^A[A]`; N02 `T16[A]`; the $t^A$ of D08 are built in the same way"),
    (G_44, r"$\Sigma_{1644}^{AB}$", "Sigma1644",
     "$\\tfrac14[\\gamma_{1644}^A,\\gamma_{1644}^B]$, the scaled commutators; they span $\\mathfrak{so}(4,4)$", "DM 1.5; M II C", "new name; the same matrices are `SAB` and `sAB[A][B]` of N10–N12; NOT `sAB` of N02, which is a sign"),
    (G_44, r"$\omega_{1644}$", "omega1644",
     "the product $\\gamma_{1644}^0\\cdots\\gamma_{1644}^7$; real chirality matrix of 4+4, $\\mathrm{diag}(-1_8,+1_8)$", "DM 1.6; M II D", "N01 `T16^A[8]`; N02 `T16[8]`"),
    (G_44, r"$P_{1644}^{L}$, $P_{1644}^{R}$", "PL1644 PR1644",
     "the real chiral projectors $\\tfrac12(1\\mp\\omega_{1644})$", "DM 1.6; M II D", "N01 `P_L`, `P_R`; N02 `PL44`, `PR44`"),
    (G_44, r"$C_{1644}^{-}$, $C_{1644}^{+}$", "Cm1644 Cp1644",
     "the products $\\gamma^0\\gamma^1\\gamma^2\\gamma^3$ and $\\gamma^4\\gamma^5\\gamma^6\\gamma^7$ of the 4+4 matrices; $C^{\\mp}\\gamma^A(C^{\\mp})^{-1}=\\mp(\\gamma^A)^T$", "DM 1.7; M II E",
     "N01, N02, N10–N12 `σ16` ($C^{-}$); N02 `S44` ($C^{+}$)"),
    (G_44, r"$W_{1644}$", "W1644",
     "$i\\,C_{1644}^{-}\\gamma_{1644}^4$, the grading: Hermitian, $W^2=1$; density of the covariant current", "DM 1.8.4; M III C", "new"),
    (G_44, r"$K_{1644}$", "K1644",
     "generator of the evolution of dirac1644 in $x4$: $\\partial_4\\psi=K\\psi$", "DM 1.8.4, Eq. (1.15); M III C", "new"),
    (G_44, r"$\beta_{1644}$, $\alpha_{1644}^j$", "beta1644 alpha1644",
     "the Hermitian matrices $-i\\gamma_{1644}^4$ and $-\\gamma_{1644}^4\\gamma_{1644}^j$ ($j=0,1,2,3$) of the quantized 4+4 theory", "S1 2; M III C", "new"),
    (G_44, r"$\mathsf h_{1644}$", "h1644",
     "Hamiltonian matrix of the quantized 4+4 theory, $iK_{1644}$ for fields independent of $x5,x6,x7$", "DM 1.8.4; M III C", "new"),
    (G_44, r"$\Pi_{1644}$", "Pi1644",
     "the projector $(i\\gamma_{1644}^Ak_A+m_{1644})/2m_{1644}$ on the plane-wave solutions of the worked example", "DM 1.8.1; M III A", "new"),

    # ------------------------------------------------------------------ 7+1 matrices
    (G_71, r"$\eta_{1671}$", "eta1671",
     "metric of the 7+1 spacetime, $\\mathrm{diag}(-1,-1,-1,-1,+1,-1,-1,-1)$: time-like $=+1$, time at index 4", "DM 2.4, Eq. (2.3); M II B",
     "N02 `η17` with the index values 0 and 4 exchanged; NOT `η71`"),
    (G_71, r"$\tau_{1671}^A$", "tau1671",
     "the real $8\\times8$ matrices of the 7+1 construction: $\\tau_{1671}^4=1_8$, the other seven represent the imaginary octonion units", "DM 2.1, App. B; M II A",
     "N02 `τO[swap04(A)]`"),
    (G_71, r"$\gamma_{1671}^A$", "gamma1671",
     "the eight real $16\\times16$ Dirac matrices of the 7+1 spacetime (contravariant index)", "DM 2.1, 2.2; M II A",
     "N02 `Γ17up[swap04(A)]`; NOT `Γ71[A]`"),
    (G_71, r"$\Sigma_{1671}^{AB}$", "Sigma1671",
     "$\\tfrac14[\\gamma_{1671}^A,\\gamma_{1671}^B]$, the scaled commutators; they span $\\mathfrak{so}(7,1)$", "DM 2.5; M II C", "N02 `Σ17[A,B]`, built from the `Γ17` with lower index: $\\Sigma_{1671}^{AB}=\\eta_{1671}^{AA}\\eta_{1671}^{BB}\\,$`Σ17[A',B']` with $A'=\\mathrm{swap}_{04}(A)$, $B'=\\mathrm{swap}_{04}(B)$"),
    (G_71, r"$\omega_{1671}$", "omega1671",
     "the product $\\gamma_{1671}^0\\cdots\\gamma_{1671}^7$; real, $\\omega_{1671}^2=-1$", "DM 2.6; M II D", "N02 `Γ17[8]`, `ω17`"),
    (G_71, r"$\chi_{1671}$", "chi1671",
     "$i\\,\\omega_{1671}$, the (complex) chirality matrix of 7+1", "DM 2.6; M II D", "N02 `χ17`; NOT `χ71`"),
    (G_71, r"$P_{1671}^{L}$, $P_{1671}^{R}$", "PL1671 PR1671",
     "the complex chiral projectors $\\tfrac12(1\\mp\\chi_{1671})$", "DM 2.6; M II D", "N02 `PL17`, `PR17`; NOT `PL71`, `PR71`"),
    (G_71, r"$U_{1671}^{c}$", "Uc1671",
     "the unitary matrix $(1-i\\gamma_{1671}^4)/\\sqrt2$ that carries the complex projectors of 7+1 into the real projectors of 4+4", "DM 2.6, Eq. (2.5); M II D", "new"),
    (G_71, r"$C_{1671}^{+}$, $C_{1671}^{-}$", "Cp1671 Cm1671",
     "$\\gamma_{1671}^4$ and $\\gamma_{1671}^4\\omega_{1671}$; $C^{\\pm}\\gamma^A(C^{\\pm})^{-1}=\\pm(\\gamma^A)^T$", "DM 2.7; M II E", "new"),
    (G_71, r"$\beta_{1671}$, $\alpha_{1671}^j$", "beta1671 alpha1671",
     "the real symmetric matrices $\\gamma_{1671}^4$ and $\\gamma_{1671}^4\\gamma_{1671}^j$ (seven space indices $j$) of the Hamiltonian", "DM 2.8.2; M II E",
     "N02 `βD`, `αD[swap04(j)]`"),
    (G_71, r"$\mathsf h_{1671}$", "h1671",
     "Hamiltonian matrix of the 7+1 theory, $\\alpha_{1671}^jp^j+\\beta_{1671}m_{1671}$", "DM 2.8.2; M III B", "N02 `h`"),
    (G_71, r"$\Pi_{1671}$", "Pi1671",
     "the projector $(\\gamma_{1671}^Ak_A+m_{1671})/2m_{1671}$ of the worked example", "DM 2.8.1", "new"),
    (G_71, r"$E_{1671}^{L}$, $E_{1671}^{R}$", "EL1671 ER1671",
     "the chiral frames: $16\\times8$ matrices whose columns span the two eigenspaces of $\\chi_{1671}$", "DM 2.8.3; M III D", "N02 `EL`, `ER`"),
    (G_71, r"$M_{1671}$, $U_{1671}^{+}$, $V_{1671}^{-}$", "M1671 Upos1671 Vneg1671",
     "the $8\\times8$ matrix $m_{1671}+p^j\\tau_{1671}^j$ and the matrices of positive- and negative-energy spinors of the worked example (check `71-D8`)", "DM 2.8.2", "N02 `M` of its Sec. 7.4 (in N02 the letter `M` also denotes other matrices and a mass); the two spinor matrices are new names"),

    # ------------------------------------------------------------------ fields
    (G_FIELD, r"$\psi_{1644}$", "dirac1644",
     "the field dirac1644: 16 complex Grassmann components on the 4+4 spacetime; $(\\gamma_{1644}^A\\partial_A-m_{1644})\\psi_{1644}=0$", "DM 1.8; M III", "new; `Ψ16` of N01 and N10–N12 is a column of 16 real quantities (in N10–N12 functions of `x0`, `x4` called the wave function of the universe), not this field"),
    (G_FIELD, r"$\psi_{1644}^{L}$, $\psi_{1644}^{R}$", "psiL1644 psiR1644",
     "the two 8-component chiral parts of dirac1644 (upper and lower eight components)", "DM 1.8.5; M III D", "new"),
    (G_FIELD, r"$m_{1644}$", "m1644", "mass of the field dirac1644", "DM 1.8.1", "new"),
    (G_FIELD, r"$\tilde\psi$", "",
     "the covariant adjoint $\\psi^\\dagger C_{1644}^{-}$ of the 4+4 theory (invariant under $\\mathrm{Spin}_0(4,4)$)", "DM 1.8.2; M III B (4+4)", "new"),
    (G_FIELD, r"$j_{1644}^A$", "j1644",
     "the conserved current $i\\,\\tilde\\psi\\gamma_{1644}^A\\psi$ of the covariant 4+4 theory (fermion number)", "DM 1.8.2, Eq. (1.13); M III B", "new"),
    (G_FIELD, r"$\psi_{1671}$", "dirac1671",
     "the field dirac1671: 16 complex Grassmann components on the 7+1 spacetime; $(i\\gamma_{1671}^A\\partial_A-m_{1671})\\psi_{1671}=0$", "DM 2.8; M III", "N02 `dirac16complex`"),
    (G_FIELD, r"$\phi_{1671}^{L}$, $\phi_{1671}^{R}$", "phiL1671 phiR1671",
     "the two 8-component chiral parts of dirac1671", "DM 2.8.3; M III D", "N02 `φL`, `φR`"),
    (G_FIELD, r"$m_{1671}$", "m1671", "mass of the field dirac1671", "DM 2.8.1", "N02 `m`, `mD`"),
    (G_FIELD, r"$\bar\psi$", "",
     "the adjoint $\\psi^\\dagger\\beta$ of the quantized theories ($\\beta=\\beta_{1671}$ or $\\beta_{1644}$); in 4+4 it differs from $\\tilde\\psi$", "S1 1.2, 3.2; M V", "new"),
    (G_FIELD, r"$\beta$, $\alpha^j$, $\chi$, $\mathsf h$, $m$", "",
     "in the sections common to both spacetimes: the matrices and the mass of the spacetime at hand, i.e. the tagged symbols with the tag dropped; $\\chi\\equiv\\chi_{1671}$ in 7+1 and $\\chi\\equiv\\omega_{1644}$ in 4+4",
     "S1 2, Eq. (2.1); M V B (4+4), Eq. for $\\chi$", "new"),
    (G_FIELD, r"$U(\vec p)$", "",
     "the $16\\times8$ matrix of orthonormal positive-energy spinors at momentum $\\vec p$", "S1 4.1; M VI A", "new"),

    # ------------------------------------------------------------------ symmetries, pairs, order parameter
    (G_SYM, r"$U(1)_F$", "", "the phase symmetry $\\psi\\to e^{i\\epsilon}\\psi$: fermion number", "S1 1.2; M V", "new"),
    (G_SYM, r"$U(1)_{VS}$", "UVS",
     "the chiral symmetry $\\psi\\to e^{i\\epsilon\\chi}\\psi$, $\\Phi_{VS}\\to e^{-2i\\epsilon}\\Phi_{VS}$, broken by the vacuum value of the Vorton-Scalar", "S1 1.2; M V", "new"),
    (G_SYM, r"$SU(2)_A$, $SU(2)_B$", "",
     "the two factors of the internal symmetry of the fields that do not depend on the four extra coordinates (zero-mode sector), in both spacetimes; $SO(4)=[SU(2)_A\\times SU(2)_B]/Z_2$", "S1 2.1; S4 2.4; M V H", "new"),
    (G_SYM, r"$SU(2)_X$, $SU(2)_Y$", "",
     "the two factors of the rotation group of the four space coordinates $x0,\\dots,x3$ of the 4+4 spacetime (NOT the internal $SU(2)_A\\times SU(2)_B$)", "DM 3.2, App. A", "new"),
    (G_SYM, r"$SO(3)_T$, $SU(2)_T$", "",
     "the rotations of the three additional times $x5,x6,x7$ of 4+4, and its covering group; the diagonal subgroup of $SU(2)_A\\times SU(2)_B$", "S1 6.3; M V H, X", "new"),
    (G_SYM, r"$U(1)_E$, $P$ (generator)", "",
     "the second central phase symmetry of the zero-mode sector and its traceless generator, $P^2=1$: $P=+1$ on the doublets of $SU(2)_A$, $-1$ on those of $SU(2)_B$", "S4 2.4; M V H", "new"),
    (G_SYM, r"$\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$", "DeltaV1644 DeltaV1671",
     "the vector pair operators $\\psi^TC^{-}\\gamma^A\\psi$ of each spacetime", "DM 3; M IV", "new"),
    (G_SYM, r"$\Delta_{1671}^{AB}$", "DeltaT1671",
     "the two-form pair operators $\\psi^TC^{+}\\gamma^A\\gamma^B\\psi$ of the 7+1 spacetime", "DM 3.2", "new"),
    (G_SYM, r"$\Delta^a$", "",
     "order parameter: expectation value of the vector pair operator along the extra axis $a\\in\\{0,5,6,7\\}$; $\\lvert\\Delta\\rvert^2\\equiv\\sum_a\\lvert\\Delta^a\\rvert^2$ (called $S$ in S1 and S4)", "S1 6.1; M VIII A", "new"),
    (G_SYM, r"$\Phi_X(\vec p)$", "",
     "two-particle content $U(\\vec p)^TXU(-\\vec p)$ of the pair operator $\\psi^TX\\psi$", "S1 4.1; M VI A", "new"),
    (G_SYM, r"$\mathsf M$", "",
     "the $8\\times8$ gap matrix on the internal states of a pair", "S1 6.1; M VIII A", "new"),
    (G_SYM, r"$d_\pm$", "",
     "the two fourfold gaps of a general state, $d_\\pm^2=\\lvert\\Delta\\rvert^2\\pm\\sqrt{\\lvert\\Delta\\rvert^4-\\lvert\\eta_{ab}\\Delta^a\\Delta^b\\rvert^2}$", "S4 4.2; M VIII B", "new"),
    (G_SYM, r"$n^a$", "", "real unit vector: direction of the order parameter in the space of the four extra axes", "S1 6.3; M VIII C", "new"),

    # ------------------------------------------------------------------ Vorton-Scalar
    (G_VS, r"$\Phi_{VS}$", "PhiVS", "the Vorton-Scalar: a complex scalar field", "S1 1.2; M V", "new (the name is that of the author's project NCSS, where the scalar is real)"),
    (G_VS, r"$v_{VS}$", "vVS", "vacuum value: $\\langle\\Phi_{VS}\\rangle=v_{VS}/\\sqrt2$", "S1 1.2; M V", "new"),
    (G_VS, r"$\lambda_{VS}$", "lambdaVS lamVS", "self-coupling of the Vorton-Scalar", "S1 1.2; M V", "new"),
    (G_VS, r"$y_{VS}$", "yVS", "Yukawa coupling of the Vorton-Scalar to the fermion field", "S1 1.2; M V", "new"),
    (G_VS, r"$s_{VS}$", "sVS", "the radial mode of the Vorton-Scalar (real field)", "S1 1.4; M V D", "new"),
    (G_VS, r"$m_s$", "msVS", "mass of the radial mode, $\\sqrt{2\\lambda_{VS}}\\,v_{VS}$", "S1 1.4; M V D", "new"),
    (G_VS, r"$\vartheta_{VS}$", "thetaVS thVS", "the phase mode of the Vorton-Scalar (real field, massless)", "S1 1.4; M V D", "new"),
    (G_VS, r"$g_{VS}$", "gVS", "coupling of both modes to the fermions, $m/v_{VS}=y_{VS}/\\sqrt2$", "S1 1.4; M V D", "new"),
    (G_VS, r"$\alpha_{VS}$", "alphaVS aVS", "$g_{VS}^2/4\\pi$", "S1 1.4; M V D", "new"),
    (G_VS, r"$D_{VS}$", "DVS", "$\\alpha_{VS}\\,m/m_s$: two fermions bind if and only if $D_{VS}>D_c$", "S1 5.3; M VII E", "new"),
    (G_VS, r"$J_{VS}^A$", "JVS", "the neutral current of the Vorton-Scalar (bilinear in $\\Phi_{VS}$)", "S1 1.5; M V E", "new"),
    (G_VS, r"$J_5^A$", "J5", "the axial current $\\bar\\psi\\gamma^A\\chi\\psi$ of the fermion field", "S1 1.5; M V E", "new"),
    (G_VS, r"$G_{VS}$", "GVS", "constant of the Fermi type of the neutral-current interaction, $G_{VS}/\\sqrt2=1/(2v_{VS}^2)$", "S1 1.5; M V E", "new"),
    (G_VS, r"$\mathcal L^{VS}_{1644}$, $\mathcal L^{VS}_{1671}$", "LVS",
     "the Lagrangians that define the Vorton-Scalar in the two spacetimes", "S1 1.2, 3.2; M V", "new"),
    (G_VS, r"$C$ (coupling)", "",
     "$g_{VS}^2m^2/m_s^2=g_{VS}^4/2\\lambda_{VS}$, the one number on which the fermion mass in matter depends", "S4 5.1; M IX A", "new"),

    # ------------------------------------------------------------------ pairing, gap, CJT
    (G_PAIR, r"$\nu$", "", "the number of internal states of a particle of given momentum, $\\nu=8$", "S1 5; M VII C", "new"),
    (G_PAIR, r"$F_S$, $F_P$", "F_scalar F_pseudoscalar", "exchange amplitudes of the radial and of the phase mode in a pair channel", "S1 4.1; M VI A", "new"),
    (G_PAIR, r"$V_{\rm NC}$, $V_s(r)$", "", "contact interaction of the neutral current in the hidden-axis channel; Yukawa potential of the radial mode", "S1 4.2; M VI B", "new"),
    (G_PAIR, r"$\sigma_X$", "", "the sign $+1$ (vector pairs along the extra axes) or $-1$ (their two-form pairs) of the phase-mode term of the kernel", "M VII C", "new"),
    (G_PAIR, r"$W(k,k')$", "", "$s$-wave average of the exchange attraction: kernel of the gap equation", "S1 5.1; M VII C", "new"),
    (G_PAIR, r"$\Delta(k)$, $\Delta_F$", "", "gap function; its value at the Fermi momentum", "S1 5.1; M VII C", "new"),
    (G_PAIR, r"$\xi_k$, $E_k$, $\kappa(k)$", "", "$\\sqrt{m^2+k^2}-\\mu$; quasi-particle energy $\\sqrt{\\xi_k^2+\\Delta(k)^2}$; pair correlation $\\langle b(-\\vec k)b(\\vec k)\\rangle$", "S4 3.1; M VII C", "new"),
    (G_PAIR, r"$k_F$, $E_F$, $\mu$", "", "Fermi momentum, $n=\\nu k_F^3/6\\pi^2$; Fermi kinetic energy $\\sqrt{m^2+k_F^2}-m$; chemical potential", "S1 5; M VII C", "new"),
    (G_PAIR, r"$N(0)$", "", "density of states at the Fermi surface per internal state, $k_F\\sqrt{m^2+k_F^2}/2\\pi^2$", "S1 5.2; M VII D", "new"),
    (G_PAIR, r"$\lambda$, $\lambda_S$, $\lambda_P$", "", "dimensionless pairing coupling $N(0)W(k_F,k_F)$ and its parts from the radial and the phase mode", "S1 5.2; M VII D", "new"),
    (G_PAIR, r"$T_c$", "", "critical temperature of the paired state", "S1 5.2; M VII D", "new"),
    (G_PAIR, r"$D_c$", "", "critical value of $D_{VS}$ for a bound pair in the Yukawa potential", "S1 5.3; M VII E", "new"),
    (G_PAIR, r"$a_{\rm sc}$, $a_{BB}$", "", "scattering length of two fermions (written $a$ in S1, S3, S4); scattering length of two bound pairs", "S1 5.3; M VII E, VIII E", "new"),
    (G_PAIR, r"$\Gamma[\phi,G]$, $\Gamma_2$", "", "the effective action for composite operators of Cornwall, Jackiw and Tomboulis; its two-particle-irreducible part", "S4 1; M VII A", "new (method of the cited paper)"),
    (G_PAIR, r"$\mathcal S$, $D_s$, $D_\vartheta$", "", "propagator of the doubled fermion field; propagators of the radial and of the phase mode", "S4 2.1; M VII B", "new"),
    (G_PAIR, r"$V[\Delta]$", "", "the CJT effective potential of the paired state, measured from the unpaired state", "S4 3.1; M VII C", "new"),
    (G_PAIR, r"$\eta(T)$", "", "largest eigenvalue of the linearized gap equation, $\\eta(T_c)=1$", "S4 3.4; M VII F", "new"),
    (G_PAIR, r"$A_2(T)$, $A_4$", "", "coefficients of the expansion of $V$ along a gap function of fixed shape (written $\\alpha$, $\\beta$ in S4)", "S4 3.4; M VII F", "new"),
    (G_PAIR, r"$a(T)$, $b$, $K_{\rm GL}$", "", "Ginzburg-Landau coefficients of the weak-coupling free energy ($K_{\\rm GL}$ is written $K$ in S1 and S4)", "S1 6.2; M VIII B", "new"),
    (G_PAIR, r"$c_s$", "", "speed of first sound of the superfluid, $c_s^2=dP/d\\rho$", "S1 6.4; M VIII E", "new"),

    # ------------------------------------------------------------------ matter, temperature, vacuum
    (G_MAT, r"$\phi$", "", "expectation value of the Vorton-Scalar, $\\langle\\Phi_{VS}\\rangle=\\phi/\\sqrt2$; the fermion mass is $g_{VS}\\phi$", "S4 2.1; M VII B", "new"),
    (G_MAT, r"$m^*$, $x$", "", "fermion mass in the medium, and $x=m^*/m$ (in the manuscripts Sec. XII A, $x$ denotes $k_F/m$, stated there)", "S4 5.1; M IX A", "new"),
    (G_MAT, r"$\varepsilon(k_F,x)$, $V_{\rm sea}(x)$", "", "energy density of the unpaired state in units of $m^4$; renormalized energy of the filled negative-energy states", "S4 5.1; M IX A", "new"),
    (G_MAT, r"$B$", "", "energy density of the state without expectation value of the Vorton-Scalar (bag constant)", "S4 5.2; M IX A", "new"),
    (G_MAT, r"$E_x$", "", "exchange (Fock) energy of the two modes", "S4 5.4; M IX C", "new"),
    (G_MAT, r"$M_s$, $M_\vartheta$, $a_s$, $a_\vartheta$", "", "thermal masses of the two boson modes in the Hartree approximation, and the curvatures $U_{\\rm eff}''$, $U_{\\rm eff}'/\\phi$", "S4 6; M IX D", "new"),
    (G_MAT, r"$P(M)$, $Q(M)$", "", "thermal part of the boson loop and of $\\tfrac12\\mathrm{Tr}\\ln$", "S4 6; M IX D", "new"),
    (G_MAT, r"$T_{\rm high}$, $T_N$, $T_{1/2}$, $T_0$", "", "two closed estimates of the temperature at which the fermion mass vanishes; temperatures at which the expectation value has fallen to half and to zero", "S4 6; M IX D", "new"),
    (G_MAT, r"$\sigma(p)$", "", "self-energy that a pair condensate in the vacuum gives to the fermions: mass splitting $m\\pm\\sigma$", "S4 7.2; M IX E", "new"),
    (G_MAT, r"$\alpha_c$, $\Lambda$ (cutoff)", "", "critical value of $\\alpha_{VS}$ for a pair condensate in the vacuum; momentum cutoff of the ladder equation", "S4 7.2; M IX E", "new"),

    # ------------------------------------------------------------------ cosmology
    (G_COS, r"$N$", "", "density of the dark radiation today in units of the density of one massless neutrino species", "S3 3.2; M XI D", "new"),
    (G_COS, r"$z_t$, $T_d$", "", "transition redshift of the stepped dark fluid, $m=T_{d0}(1+z_t)$; temperature of the dark sector", "S3 3.2; M XI A, D", "new"),
    (G_COS, r"$r_{\rm step}$", "", "factor by which the density of the dark radiation rises when the fermions annihilate, $15^{1/3}$ for this model", "S3 1; M XI A", "new"),
    (G_COS, r"$\omega_b$, $\omega_c$, $h$, $H_0$, $A_s$, $n_s$, $\tau$", "", "the six parameters of the standard cosmological model: densities of baryons and of cold dark matter, Hubble constant $H_0=100\\,h$ km s$^{-1}$ Mpc$^{-1}$, amplitude and tilt of the primordial spectrum, optical depth", "S3 2.1; M XI B", "standard notation"),
    (G_COS, r"$\Delta\chi^2$", "", "worsening of the best fit to the mock data relative to the standard model", "S3 3.3; M XI E", "new"),
    (G_COS, r"$\theta_s$, $r_s$, $D_A$", "", "angular size of the sound horizon; sound horizon; distance to last scattering", "S3 3.1; M XI C", "standard notation"),
    (G_COS, r"$c_{s0}$", "", "sound speed of superfluid dark matter today at the mean cosmic density", "S3 4.1; M XII A", "new"),
    (G_COS, r"$\rho_0$, $R_{TF}$", "", "constant of the equation of state of a condensate of bound pairs; Thomas-Fermi radius of a core supported by its pressure", "S3 4.1; M XII A", "new"),
    (G_COS, r"$m_{\rm deg}$, $m_{B,\max}$", "", "largest fermion mass for degeneracy, and largest boson mass for condensation, at a given density and velocity dispersion", "S3 4.3; M XII C", "new"),
]


def manuscript_sections_only(defined_in):
    """The two manuscripts number their subsections differently (the 4+4 manuscript has
    more of them), so a reference to the manuscripts names the section only: in the part
    of the field that follows "M ", the letter of a subsection is removed."""
    parts = defined_in.split(";")
    out = []
    for part in parts:
        if part.strip().startswith("M "):
            part = re.sub(r"\b([IVX]+) [A-K]\b(?!\.)", r"\1", part)
            # "M II, II" and the like after the removal
            nums = []
            for tok in [t.strip() for t in part.strip()[2:].split(",")]:
                if tok and tok not in nums:
                    nums.append(tok)
            part = " M " + ", ".join(nums)
        out.append(part)
    return ";".join(out).strip()


def main():
    path = os.path.join(ROOT, "provenance", "symbols.csv")
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write("# PROJECT spacetimes: registry of the symbols of the project (the unique symbol definitions that Hard Rule 5 requires)\n")
        f.write("# written by: code/scripts/write_symbol_registry.py (edit the list there); verified by: code/scripts/check_symbols.py\n")
        f.write("# columns: group; symbol as written in the documents (LaTeX); names in programs and data files (blank-separated); meaning; where defined "
                "(DM = dirac matrices, S1..S4 = supplements, M = manuscripts); origin (N.. = notebook, D.. = paper, or new)\n")
        w = csv.writer(f)
        w.writerow(["group", "symbol", "names_in_files", "meaning", "defined_in", "origin"])
        for g, sym, names, meaning, defined, origin in REGISTRY:
            w.writerow([g, sym, names, meaning, manuscript_sections_only(defined), origin])
    print("wrote provenance/symbols.csv (%d entries)" % len(REGISTRY))


if __name__ == "__main__":
    main()
