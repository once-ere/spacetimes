#!/usr/bin/env python3
"""
write_symbol_registry.py (Revision_NoGo) -- the registry of every symbol that the documents
of the folder Revision_NoGo use.

Hard Rule 5 of the project requires that every symbol denote ONE object in the whole
project.  This registry has two kinds of entries:

  * TAKEN: symbols of the MASTER registries of the project (the registry of the main
    project, ../provenance/symbols.csv, and the MASTER registry of the revision on the
    deflating spacetimes, ../Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv)
    that Revision_NoGo uses with exactly the same meaning.  Their symbol text and meaning
    are copied from the master file (the one named in the entry), so that they cannot
    drift; a note on their use in Revision_NoGo may be appended;
  * NEW: the symbols that Revision_NoGo introduces.  Every one of them carries the suffix
    NG, or is an element of an algebra written in sans-serif type, a unit matrix, a number
    set, a group, an operation sign, or a plain index or variable that no master registry
    defines with another meaning (checked by code/scripts/check_symbols.py).

The third field of a NEW entry lists the plain-text names under which the programs, the
figures and the data files of Revision_NoGo write the symbol (each name belongs to one
entry; check NG-S1).

Output:
    provenance/symbols_ng.csv                  the registry (machine-readable)
    provenance/generated/symbols_ng.md         the registry as a Markdown table and as
                                               LaTeX table rows (fragments for the documents)

Run from the folder Revision_NoGo:
    python3 code/scripts/write_symbol_registry.py
    python3 code/scripts/check_symbols.py
"""
import csv
import os

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)
MASTER_MAIN = os.path.join(MAIN, "provenance", "symbols.csv")
MASTER_DEFL = os.path.join(MAIN, "Revision_Deflation_CJT_and_Kohn-Sham", "provenance", "symbols_master.csv")

G1 = "1 Coordinates, indices and conventions"
G2 = "2 Matrices and fields of the two spacetimes"
G3 = "3 Octonions, split octonions and triality"
G4 = "4 The no-go theorems and the superalgebras"
G5 = "5 The combined manifolds"
G6 = "6 The spin-zero superfluid"
G7 = "7 Power counting and the Kaluza-Klein tower"
G8 = "8 Number sets, groups, operations and labels"

# (group, symbol text exactly as in the master file, which master file)
TAKEN = [
    (G1, r"$X=\{x0,\dots,x7\}$", "main"),
    (G1, r"$A,B,C,D$", "main"),
    (G1, r"$i,j$", "main"),
    (G1, r"$a,b$", "defl", "INT"),
    (G1, r"$t$, $s$ (indices)", "defl"),
    (G1, r"$D$", "defl"),
    (G1, r"$K$, $J$ (index sets); $r$, $r'$ (ranks)", "defl", "K"),
    (G2, r"$\eta_{1671}$", "main"),
    (G2, r"$\eta_{1644}$", "main"),
    (G2, r"$\gamma_{1671}^A$", "main"),
    (G2, r"$\gamma_{1644}^A$", "main"),
    (G2, r"$\Sigma_{1671}^{AB}$", "main"),
    (G2, r"$\Sigma_{1644}^{AB}$", "main"),
    (G2, r"$\omega_{1671}$", "main"),
    (G2, r"$\omega_{1644}$", "main"),
    (G2, r"$C_{1671}^{+}$, $C_{1671}^{-}$", "main"),
    (G2, r"$C_{1644}^{-}$, $C_{1644}^{+}$", "main"),
    (G2, r"$\beta_{1671}$, $\alpha_{1671}^j$", "main"),
    (G2, r"$\beta_{1644}$, $\alpha_{1644}^j$", "main"),
    (G2, r"$\tau_{1671}^A$", "main"),
    (G2, r"$\tau_{1644}^A$, $\bar\tau_{1644}^A$", "main"),
    (G2, r"$\psi_{1671}$", "main"),
    (G2, r"$\psi_{1644}$", "main"),
    (G2, r"$m_{1671}$", "main"),
    (G2, r"$m_{1644}$", "main"),
    (G6, r"$\Delta_{1644}^{A}$, $\Delta_{1671}^{A}$", "defl", "V^A V^B"),
    (G6, r"$\Delta_{1671}^{AB}$", "defl"),
]

# Notes on the use in Revision_NoGo of a taken symbol (appended to the master meaning).
USE_NOTE = {
    r"$A,B,C,D$": r"In Revision\_NoGo the letters $A$, $B$, $C$ are such indices (also as lower indices, as in $\eta^{\rm NG}_{AB}$, $\mathcal P^{\rm NG}_C$); the letter $D$ is used only with the meaning of the entry $D$ of this table.",
    r"$i,j$": r"In Revision\_NoGo only $j$ is used as an index, with the values 1, 2, 3; the letter $i$ is the imaginary unit (entry $i$).",
    r"$a,b$": r"In Revision\_NoGo $a$ and $b$ always take the values 0, 5, 6, 7 (the fibre of the combined manifolds).",
    r"$D$": r"In Revision\_NoGo $D=7$ on the eight-dimensional spacetimes and $D=3$ in the four-dimensional world of the combined manifolds; the volume and the area of the unit ball do not occur.",
    r"$K$, $J$ (index sets); $r$, $r'$ (ranks)": r"In Revision\_NoGo only $K$ and $r$ are used: $\gamma^K_{\rm NG}$, $\Gamma^K_{71\rm NG}$ and the tensorial charges $\mathcal Z^K_{\rm NG}$ are labelled by $K$; the Fierz matrix and the bit mask do not occur.",
    r"$\beta_{1671}$, $\alpha_{1671}^j$": r"In Revision\_NoGo only $j=1,2,3$ occurs.",
    r"$\beta_{1644}$, $\alpha_{1644}^j$": r"In Revision\_NoGo only $j=1,2,3$ occurs.",
}

# (group, symbol, plain-text names in programs, figures and data files of Revision_NoGo, meaning)
NEW = [
    # ------------------------------------------------------------------ 1
    (G1, r"$i$", "",
     r"the imaginary unit, $i^2=-1$ (in the plain text of the checks O-14a, O-14c and K-6 and of the figures of the algebras the letter i is the octonion unit $\mathsf e_1$, see the entry $\mathsf e_0,\dots,\mathsf e_7$)"),
    (G1, r"$l$, $l'$", "",
     r"indices of the 16 supercharges $\mathcal Q^{\rm NG}_l$, one for each real spinor component (values $1,\dots,16$); the summation index of the Fierz matrix, which a master registry also calls $l$, does not occur in Revision\_NoGo"),
    (G1, r"$p$", "",
     r"a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions); used only in Sec. III A"),
    (G1, r"$\vec q=(q^1,q^2,q^3)$, $q$", "q q_1",
     r"the momentum of a particle in ordinary space (the three directions $x1,x2,x3$) and its length $q=\lvert\vec q\rvert$"),
    (G1, r"$\gamma^A_{\rm NG}$, $\gamma^K_{\rm NG}$, $\Sigma^{AB}_{\rm NG}$, $\omega_{\rm NG}$, $C^\pm_{\rm NG}$, $\beta_{\rm NG}$, $\alpha^j_{\rm NG}$, $\psi_{\rm NG}$, $m_{\rm NG}$, $\eta_{\rm NG}$",
     "gamma^A gamma^K gamma^a gamma^j Sigma^(AB) Sigma^(ab) omega C+ C- C(+-) beta alpha^j psi m eta eta_AB eta_AA",
     r"in statements that hold on both spacetimes: the object of the spacetime under discussion, i.e.\ $\gamma^A_{1671}$ or $\gamma^A_{1644}$, $\Sigma^{AB}_{1671}$ or $\Sigma^{AB}_{1644}$, $\omega_{1671}$ or $\omega_{1644}$, $C^\pm_{1671}$ or $C^\pm_{1644}$, $\beta_{1671}$ or $\beta_{1644}$, $\alpha^j_{1671}$ or $\alpha^j_{1644}$, $\psi_{1671}$ or $\psi_{1644}$, $m_{1671}$ or $m_{1644}$, $\eta_{1671}$ or $\eta_{1644}$; $\gamma^K_{\rm NG}$ is the ordered product of the $\gamma^A_{\rm NG}$ with $A$ in $K$ (increasing $A$; $\mathsf 1_{16}$ for the empty set); $\eta^{AB}_{1671}$, $\eta^{\rm NG}_{AB}$ and $\eta^{\rm NG}_{AA}=\pm1$ are the entries of the diagonal metric"),
    (G1, r"$\Sigma^{\rm NG}_{AB}$", "",
     r"the spinor generators with lowered indices, $\Sigma^{\rm NG}_{AB}=\eta^{\rm NG}_{AA}\eta^{\rm NG}_{BB}\Sigma^{AB}_{\rm NG}$ (no sum), with which the Lorentz generators $\mathcal J^{AB}_{\rm NG}$ act on the supercharges"),
    (G1, r"$\mathsf 1_8$, $\mathsf 1_{16}$", "",
     r"the unit matrices of size 8 and 16"),
    # ------------------------------------------------------------------ 2
    (G2, r"$u_{\rm NG}(A)$", "",
     r"on $7+1$: the index of the octonion unit attached to the coordinate $xA$, $u_{\rm NG}(4)=0$, $u_{\rm NG}(j)=j$ ($j=1,2,3$), $u_{\rm NG}(0)=4$, $u_{\rm NG}(t)=t$ ($t=5,6,7$)"),
    (G2, r"$\Gamma^A_{71\rm NG}$, $\Gamma^A_{44\rm NG}$; $\Gamma^K_{71\rm NG}$, $\Gamma^K_{44\rm NG}$",
     "Gamma71oct Gamma44oct Gamma_oct",
     r"the octonionic $16\times16$ Dirac matrices: on $7+1$, $\Gamma^4_{71\rm NG}=\begin{pmatrix}0&\mathsf 1_8\\\mathsf 1_8&0\end{pmatrix}$ and $\Gamma^A_{71\rm NG}=\begin{pmatrix}-L_{\rm NG}(\mathsf e_{u_{\rm NG}(A)})&0\\0&L_{\rm NG}(\mathsf e_{u_{\rm NG}(A)})\end{pmatrix}$ for $A\ne4$, with $\Gamma^A_{71\rm NG}\Gamma^C_{71\rm NG}+\Gamma^C_{71\rm NG}\Gamma^A_{71\rm NG}=2\eta^{AC}_{1671}\mathsf 1_{16}$; on $4+4$, $\Gamma^A_{44\rm NG}=\begin{pmatrix}0&L_{\rm NG}(\bar{\mathsf e}_A)\\L_{\rm NG}(\mathsf e_A)&0\end{pmatrix}$ with the Clifford relations of $\eta_{1644}$; $\Gamma^K_{71\rm NG}$, $\Gamma^K_{44\rm NG}$ are their ordered products over an index set $K$"),
    (G2, r"$\mathcal I_{71\rm NG}$, $\mathcal I_{44\rm NG}$; $\mathsf U_{\rm NG}$", "I1671 I1644 I",
     r"the integer intertwiners $\mathcal I_{71\rm NG}=\sum_K\Gamma^K_{71\rm NG}\mathsf U_{\rm NG}(\gamma^K_{1671})^{-1}$ (sum over the 256 index sets), with $\mathcal I_{71\rm NG}\gamma^A_{1671}=\Gamma^A_{71\rm NG}\mathcal I_{71\rm NG}$, and $\mathcal I_{44\rm NG}$, the same sum with $\Gamma^K_{44\rm NG}$ and $\gamma^K_{1644}$; $\mathsf U_{\rm NG}$ is a fixed $16\times16$ matrix with one entry 1 and all others 0"),
    (G2, r"$\omega^{\rm 4D}_{\rm NG}$, $\omega^{\rm int}_{\rm NG}$", "omega4 omega_int",
     r"the products $\gamma^1_{\rm NG}\gamma^2_{\rm NG}\gamma^3_{\rm NG}\gamma^4_{\rm NG}$ of the four matrices of the world and $\gamma^0_{\rm NG}\gamma^5_{\rm NG}\gamma^6_{\rm NG}\gamma^7_{\rm NG}$ of the four matrices of the fibre"),
    (G2, r"$\mathsf A_{\rm NG}$", "A8",
     r"the Dirac conjugation matrix, $\mathsf A_{\rm NG}\Sigma^{AB}_{\rm NG}\mathsf A_{\rm NG}^{-1}=-(\Sigma^{AB}_{\rm NG})^\dagger$ for all 28 generators: $\gamma^4_{1671}$ on $7+1$, $\gamma^4_{1644}\gamma^5_{1644}\gamma^6_{1644}\gamma^7_{1644}$ on $4+4$"),
    # ------------------------------------------------------------------ 3
    (G3, r"$\mathbb H_{\rm NG}$", "H",
     r"the quaternions: real combinations of $\mathsf e_0,\mathsf e_1,\mathsf e_2,\mathsf e_3$ with $\mathsf e_1^2=\mathsf e_2^2=\mathsf e_3^2=-1$, $\mathsf e_1\mathsf e_2=-\mathsf e_2\mathsf e_1=\mathsf e_3$ and cyclic permutations (plain text: Im H its imaginary part, H l the set $\mathbb H_{\rm NG}\mathsf e_4$)"),
    (G3, r"$\mathsf e_0,\dots,\mathsf e_7$; $\mathsf e_p$, $\mathsf e_A$, $\mathsf e_j$, $\mathsf e_{u_{\rm NG}(A)}$", "e0 e1 e2 e3 e4 e5 e6 e7 e_A 1 i j k l il jl kl",
     r"the basis of the octonions and of the split octonions, also called 1, i, j, k, l, il, jl, kl: $\mathsf e_0=1$, $\mathsf e_1,\mathsf e_2,\mathsf e_3$ the imaginary quaternion units (as pairs $(\mathsf e_p,0)$), $\mathsf e_4=(0,1)$, $\mathsf e_5=(0,\mathsf e_1)$, $\mathsf e_6=(0,\mathsf e_2)$, $\mathsf e_7=(0,\mathsf e_3)$"),
    (G3, r"$\mathsf x$, $\mathsf y$, $\mathsf z$, $\mathsf w$; $\mathsf x^p$; $\mathsf x_1$, $\mathsf x_2$, $\mathsf y_1$, $\mathsf y_2$; $\mathsf e$, $\mathsf f$", "x y z",
     r"elements of the algebras (octonions, split octonions or quaternions); $\mathsf x^p$ the real coefficients of $\mathsf x=\sum_p\mathsf x^p\mathsf e_p$; $(\mathsf x_1,\mathsf x_2)=\mathsf x_1+\mathsf x_2\mathsf e_4$ the pair of quaternions that represents an element of the doubled algebra; $\mathsf e$, $\mathsf f$ two imaginary units; in Sec. VI, $\mathsf x$, $\mathsf y$, $\mathsf z$ are generators of a superalgebra"),
    (G3, r"$\bar{\mathsf x}$", "conj(x)",
     r"the conjugate of $\mathsf x$: the signs of $\mathsf e_1,\dots,\mathsf e_7$ are changed; $\overline{(\mathsf x_1,\mathsf x_2)}=(\bar{\mathsf x}_1,-\mathsf x_2)$"),
    (G3, r"$\varsigma_{\rm NG}$", "varsigma",
     r"the sign in the Cayley-Dickson product $(\mathsf x_1,\mathsf x_2)(\mathsf y_1,\mathsf y_2)=(\mathsf x_1\mathsf y_1+\varsigma_{\rm NG}\bar{\mathsf y}_2\mathsf x_2,\ \mathsf y_2\mathsf x_1+\mathsf x_2\bar{\mathsf y}_1)$: $-1$ for the octonions, $+1$ for the split octonions"),
    (G3, r"$\mathbb O_{\rm NG}$, $\mathbb O'_{\rm NG}$; $\mathbb A_{\rm NG}$", "O_NG Os_NG O Os",
     r"the octonions ($\varsigma_{\rm NG}=-1$, $\mathsf e_4^2=-1$) and the split octonions ($\varsigma_{\rm NG}=+1$, $\mathsf e_4^2=+1$); $\mathbb A_{\rm NG}$ denotes either"),
    (G3, r"$N_{\rm NG}$; $\langle\mathsf x,\mathsf y\rangle_{\rm NG}$", "N",
     r"the norm $N_{\rm NG}(\mathsf x)=\mathsf x\bar{\mathsf x}$, $\sum_{p=0}^7(\mathsf x^p)^2$ in $\mathbb O_{\rm NG}$ and $\sum_{p=0}^3(\mathsf x^p)^2-\sum_{p=4}^7(\mathsf x^p)^2$ in $\mathbb O'_{\rm NG}$, and its polarization, the scalar product with $\langle\mathsf x,\mathsf x\rangle_{\rm NG}=N_{\rm NG}(\mathsf x)$ (plain text: $\langle\mathsf x,\mathsf y\rangle_{\rm NG}$ is written <x,y>)"),
    (G3, r"$(\mathsf x,\mathsf y,\mathsf z)$; $[\mathsf x,\mathsf y]$", "",
     r"the associator $(\mathsf x\mathsf y)\mathsf z-\mathsf x(\mathsf y\mathsf z)$ and the commutator $\mathsf x\mathsf y-\mathsf y\mathsf x$ of elements of an algebra"),
    (G3, r"$\mathrm{Jac}_{\rm NG}$", "Jac",
     r"the Jacobiator $\mathrm{Jac}_{\rm NG}(\mathsf x,\mathsf y,\mathsf z)=[[\mathsf x,\mathsf y],\mathsf z]+[[\mathsf y,\mathsf z],\mathsf x]+[[\mathsf z,\mathsf x],\mathsf y]$; it vanishes in every Lie algebra"),
    (G3, r"$\rho_{\rm NG}$", "",
     r"a linear map from the imaginary (split) octonions to the linear operators on a vector space that turns commutators into commutators (Theorem 1)"),
    (G3, r"$L_{\rm NG}(\mathsf x)$", "L_e1 L_e7 L_e_A L_conj(e_A) L_l L",
     r"the left multiplication $\mathsf y\mapsto\mathsf x\mathsf y$, an $8\times8$ matrix"),
    (G3, r"$\mathsf D$; $\mathsf D_1$, $\mathsf D_2$; $\mathrm{Der}(\mathbb A_{\rm NG})$", "delta Der",
     r"derivations: linear maps with $\mathsf D(\mathsf x\mathsf y)=(\mathsf D\mathsf x)\mathsf y+\mathsf x(\mathsf D\mathsf y)$; $\mathrm{Der}(\mathbb A_{\rm NG})$ is the Lie algebra of all derivations (dimension 14)"),
    (G3, r"$B_{\rm NG}$", "",
     r"the Killing form of the derivation algebra, $B_{\rm NG}(\mathsf D_1,\mathsf D_2)=\mathrm{tr}(\mathrm{ad}\,\mathsf D_1\,\mathrm{ad}\,\mathsf D_2)$, with $\mathrm{ad}\,\mathsf D_1$ the matrix of the map $\mathsf D\mapsto[\mathsf D_1,\mathsf D]$"),
    (G3, r"$\mathfrak g_2$, $\mathrm G_2$", "g2 G2 Aut(O)",
     r"the exceptional Lie algebra of dimension 14 (compact for $\mathbb O_{\rm NG}$, split for $\mathbb O'_{\rm NG}$) and the compact group of automorphisms of the octonions"),
    (G3, r"$\mathfrak{so}(N_{\rm NG})$", "so(N)",
     r"the Lie algebra of the linear maps $\mathsf m$ of an algebra with $\langle\mathsf m\mathsf x,\mathsf y\rangle_{\rm NG}+\langle\mathsf x,\mathsf m\mathsf y\rangle_{\rm NG}=0$: $\mathfrak{so}(8)$ for the octonions, $\mathfrak{so}(4,4)$ for the split octonions"),
    (G3, r"$\mathsf m$, $\mathsf m_1$, $\mathsf m_2$, $\mathsf m_3$", "m1 m2 m3 LT",
     r"elements of $\mathfrak{so}(N_{\rm NG})$; local triality: $\mathsf m_1(\mathsf x\mathsf y)=(\mathsf m_2\mathsf x)\mathsf y+\mathsf x(\mathsf m_3\mathsf y)$ for all $\mathsf x$, $\mathsf y$ (plain text LT: this system of equations)"),
    (G3, r"$T^{(2)}_{\rm NG}$, $T^{(3)}_{\rm NG}$", "T2 T3",
     r"the triality maps $\mathsf m_1\mapsto\mathsf m_2$ and $\mathsf m_1\mapsto\mathsf m_3$, outer automorphisms of $\mathfrak{so}(N_{\rm NG})$ of order two"),
    (G3, r"$\mathsf r_{01}$", "r01",
     r"the generator of rotations in the plane of $\mathsf e_0$ and $\mathsf e_1$: $\mathsf r_{01}\mathsf e_1=\mathsf e_0$, $\mathsf r_{01}\mathsf e_0=-\mathsf e_1$, zero on the other six units"),
    (G3, r"$\lambda_{\rm NG}$", "lambda",
     r"the variable of characteristic polynomials, $\det(\lambda_{\rm NG}\mathsf 1_8-\mathsf m)$"),
    (G3, r"$\kappa_{\rm NG}$; $\mathrm{Ad}\,\kappa_{\rm NG}$", "kappa",
     r"the conjugation $\mathsf x\mapsto\bar{\mathsf x}$ (a reflection), and the automorphism $\mathsf m\mapsto\kappa_{\rm NG}\mathsf m\kappa_{\rm NG}$ that it induces"),
    (G3, r"$\vartheta_{\rm NG}$", "vartheta",
     r"the triality automorphism of order three, $\vartheta_{\rm NG}=T^{(2)}_{\rm NG}\circ\mathrm{Ad}\,\kappa_{\rm NG}$"),
    (G3, r"$V_{\rm NG}$, $S^+_{\rm NG}$, $S^-_{\rm NG}$", "S+ S-",
     r"the three eight-dimensional representations of the rotation algebra of eight dimensions: the vectors and the two kinds of half-spinors ($S^\pm_{\rm NG}$ also the two real halves of the spinors of $4+4$)"),
    (G3, r"$\phi_{\rm NG}$, $\phi^{\rm mix}_{\rm NG}$", "phi phi_mixed",
     r"the associative three-form $\phi_{\rm NG}(\mathsf x,\mathsf y,\mathsf z)=\langle\mathsf x,\mathsf y\mathsf z\rangle_{\rm NG}$ on the imaginary octonions, and its part with one unit of ordinary space and two of the fibre"),
    # ------------------------------------------------------------------ 4
    (G4, r"H0, \dots, H8", "",
     r"the labels of the hypotheses of the no-go theorems (Table I)"),
    (G4, r"$\mathcal Q_{\rm NG}$, $\mathcal Q^{\rm NG}_l$; $\mathcal Q^{\rm NG}_{\mathsf v}$, $\mathsf v^l$", "Q Q+ Q-",
     r"supercharges (odd generators); $\mathcal Q^{\rm NG}_l$ the 16 supercharges of Sec. VI; $\mathcal Q^{\rm NG}_{\mathsf v}=\sum_l\mathsf v^l\mathcal Q^{\rm NG}_l$ a real combination with real coefficients $\mathsf v^l$ (plain text: Q+ and Q- are the supercharges of the two chiralities of $4+4$)"),
    (G4, r"$\mathcal J^{AB}_{\rm NG}$", "J",
     r"the 28 Lorentz generators of the super-Poincaré algebra ($A<B$; $\mathcal J^{BA}_{\rm NG}=-\mathcal J^{AB}_{\rm NG}$)"),
    (G4, r"$\mathcal P^{\rm NG}_C$; $\vec{\mathcal P}_{\rm NG}$", "P P_C P_A P_4",
     r"the eight momenta (translation generators); $\lvert\vec{\mathcal P}_{\rm NG}\rvert^2=\sum_{A\ne4}(\mathcal P^{\rm NG}_A)^2$"),
    (G4, r"$\mathcal R_{\rm NG}$", "Rch",
     r"the R-charge, the even generator that commutes with the Lorentz generators and the momenta and acts on the supercharges by $\omega_{\rm NG}$"),
    (G4, r"$\mathsf Y^C_{\rm NG}$", "Y^C",
     r"the symmetric matrices $2C^+_{\rm NG}\gamma^C_{\rm NG}$ of the anticommutator $\{\mathcal Q^{\rm NG}_l,\mathcal Q^{\rm NG}_{l'}\}=\sum_C(\mathsf Y^C_{\rm NG})_{ll'}\mathcal P^{\rm NG}_C$"),
    (G4, r"$\mathcal Z^K_{\rm NG}$", "Z_K",
     r"the 128 tensorial charges of $7+1$ (index sets $K$ with 0, 4, 5 or 8 elements), which commute with the momenta and the supercharges"),
    (G4, r"$\lvert\mathsf x\rvert$", "",
     r"the degree of a generator $\mathsf x$ of a superalgebra: 1 for a supercharge, 0 otherwise"),
    (G4, r"$\varphi_{\rm NG}$", "varphi",
     r"the real parameter of a one-parameter group of transformations: a rotation angle, a rapidity, or the phase of the fermion number"),
    (G4, r"$v_{\rm NG}$", "v",
     r"the component $\mathcal P^{\rm NG}_1$ of the momentum of the figure of the superalgebras, whose component $\mathcal P^{\rm NG}_4$ is 1"),
    # ------------------------------------------------------------------ 5
    (G5, r"$\mathcal M^{3,1}_{\rm NG}$", "M(3,1) M",
     r"the four-dimensional world of the combined manifolds: Minkowski spacetime with the coordinates $x1,x2,x3$ (space) and $x4$ (time)"),
    (G5, r"$\mathcal K_{\rm NG}$; $\mathcal K^4_{\rm NG}$, $\mathcal K^{1,3}_{\rm NG}$", "K(4) K(1,3)",
     r"the fibre with the coordinates $x0,x5,x6,x7$: Euclidean on $7+1$ ($\mathcal K^4_{\rm NG}$), with one direction of each sign class on $4+4$ ($\mathcal K^{1,3}_{\rm NG}$: $\eta_{1644}=+1$ along $x0$, $-1$ along $x5,x6,x7$)"),
    (G5, r"$R_{\rm NG}$", "R",
     r"the radius of a torus fibre (each of its four circles has the circumference $2\pi R_{\rm NG}$)"),
    (G5, r"$n^a_{\rm NG}$; $n^{\max}_{\rm NG}$", "n n_a n0 n5 n6 n7 Nmax",
     r"the integers that label the Kaluza-Klein modes $\exp(i\sum_an^a_{\rm NG}x^a/R_{\rm NG})$ on a torus fibre; $n^{\max}_{\rm NG}$ a bound on $\lvert n^a_{\rm NG}\rvert$"),
    (G5, r"$m^{\rm eff}_{\rm NG}$", "m_eff",
     r"the four-dimensional mass of a Kaluza-Klein mode: $(m^{\rm eff}_{\rm NG})^2=m_{1671}^2+\sum_a(n^a_{\rm NG})^2/R_{\rm NG}^2$ on $7+1$, $m_{1644}^2+[(n^0_{\rm NG})^2-\sum_t(n^t_{\rm NG})^2]/R_{\rm NG}^2$ on $4+4$"),
    (G5, r"$\varepsilon_{\rm NG}$", "eps",
     r"the energy $(q^2+m_{\rm NG}^2)^{1/2}$ of a particle of momentum $\vec q$ in the four-dimensional world (with $m^{\rm eff}_{\rm NG}$ in place of $m_{\rm NG}$ for a Kaluza-Klein mode)"),
    # ------------------------------------------------------------------ 6
    (G6, r"$\mathsf h_{\rm NG}(\vec q)$", "h(q)",
     r"the Hamiltonian matrix $\sum_j\alpha^j_{\rm NG}q^j+\beta_{\rm NG}m_{\rm NG}$ of a plane wave of momentum $\vec q$ in the four-dimensional world, $i\partial_4\psi_{\rm NG}=\mathsf h_{\rm NG}\psi_{\rm NG}$; eigenvalues $\pm\varepsilon_{\rm NG}$, eight times each"),
    (G6, r"$\mathsf K_{\rm NG}$; $\mathsf K^a_{\rm NG}$, $\mathsf K^{ab}_{\rm NG}$", "",
     r"a $16\times16$ matrix of a pair $\psi^T_{\rm NG}\mathsf K_{\rm NG}\psi_{\rm NG}$; the isotropic spin-zero channels $\mathsf K^a_{\rm NG}=C^-_{\rm NG}\gamma^a_{\rm NG}$ (fibre vector) and $\mathsf K^{ab}_{\rm NG}=C^+_{\rm NG}\gamma^a_{\rm NG}\gamma^b_{\rm NG}$ (fibre two-form)"),
    (G6, r"$\Xi^{ab}_{\rm NG}$", "",
     r"the two-form pair operators of $4+4$, $\psi^T_{1644}C^+_{1644}\gamma^a_{1644}\gamma^b_{1644}\psi_{1644}$"),
    (G6, r"$\psi^1_{\rm NG}$, $\psi^2_{\rm NG}$; $\Phi^{\rm D}_{\rm NG}$", "",
     r"two copies of the field $\psi_{1671}$ (the doublet) and their fibre-singlet spin-zero pair $\Phi^{\rm D}_{\rm NG}=2(\psi^1_{\rm NG})^TC^+_{1671}\psi^2_{\rm NG}$"),
    (G6, r"$G_{\rm NG}$, $G^{\rm D}_{\rm NG}$, $G^{\rm crit}_{\rm NG}$", "G G_crit G4f G^crit G^D",
     r"the coupling of the eight-dimensional vector-pair interaction (dimension mass$^{-2}$ in the four-dimensional world), the coupling of the doublet channel, and the critical coupling above which the vacuum condenses (plain text G4f in check R-1: a four-fermion coupling in general)"),
    (G6, r"$\mathcal H^{\rm V}_{\rm NG}$, $\mathcal H^{\rm int}_{\rm NG}$, $\mathcal H^{\rm D}_{\rm NG}$, $\mathcal H^{\rm MF}_{\rm NG}$", "",
     r"Hamiltonian densities: the eight-dimensionally Lorentz-invariant interaction $\tfrac14G_{\rm NG}\sum_{A,B}\eta^{AB}_{1671}(\Delta^A_{1671})^\dagger\Delta^B_{1671}$ of the vector pairs (and the same on $4+4$), its attractive spin-zero part, the interaction $-\tfrac14G^{\rm D}_{\rm NG}(\Phi^{\rm D}_{\rm NG})^\dagger\Phi^{\rm D}_{\rm NG}$ of the doublet, and the mean-field Hamiltonian density"),
    (G6, r"$\Delta^a_{\rm NG}$, $\Delta^{ab}_{\rm NG}$, $\Delta^{\rm D}_{\rm NG}$; $\lvert\Delta_{\rm NG}\rvert$", "Delta Delta^0 Delta^5 Delta_0 Delta_0a",
     r"the pair amplitudes (complex numbers) of the mean-field theory, $\Delta^a_{\rm NG}=\tfrac12G_{\rm NG}\langle\psi^T_{\rm NG}\mathsf K^a_{\rm NG}\psi_{\rm NG}\rangle$ and likewise for the two-form and the doublet; $\lvert\Delta_{\rm NG}\rvert^2$ the sum of the squared moduli of the components; for an isotropic state $\lvert\Delta_{\rm NG}\rvert$ is the gap"),
    (G6, r"$\mu_{\rm NG}$", "mu",
     r"the chemical potential, which fixes the density of fermions"),
    (G6, r"$\mathcal N_{\rm NG}(\vec q)$", "",
     r"the Nambu spinor of 32 components, $(\psi_{\rm NG}(\vec q),\psi_{\rm NG}(-\vec q)^{\dagger T})$, for the doublet $(\psi^1_{\rm NG}(\vec q),\psi^2_{\rm NG}(-\vec q)^{\dagger T})$"),
    (G6, r"$\mathsf H_{\rm NG}(\vec q)$; $\mathsf P_{\rm NG}$", "",
     r"the $32\times32$ Bogoliubov-de Gennes matrix $\begin{pmatrix}\mathsf h_{\rm NG}(\vec q)-\mu_{\rm NG}&\mathsf P_{\rm NG}\\\mathsf P_{\rm NG}^\dagger&-(\mathsf h_{\rm NG}(-\vec q)-\mu_{\rm NG})^T\end{pmatrix}$ and its pairing block, $\mathsf P_{\rm NG}=-\sum_a\Delta^a_{\rm NG}(\mathsf K^a_{\rm NG})^\dagger$ (for the doublet $\Delta^{\rm D}_{\rm NG}(C^+_{1671})^*$)"),
    (G6, r"$\Omega_{\rm NG}$", "Omega",
     r"the grand potential per volume at zero temperature (effective potential of the superfluid), up to terms that do not depend on the pair amplitudes"),
    (G6, r"$\mathcal E_{\rm NG}$", "",
     r"an eigenvalue of $\mathsf H_{\rm NG}(\vec q)$"),
    (G6, r"$E^\pm_{\rm NG}$", "E^+ E^- E^(+-)",
     r"the quasi-particle energies of an isotropic state, $[(\varepsilon_{\rm NG}\pm\mu_{\rm NG})^2+\lvert\Delta_{\rm NG}\rvert^2]^{1/2}$"),
    (G6, r"$E^{(1)\pm}_{\rm NG}$", "E_(+-)",
     r"the quasi-particle energies of the spin-one state, $(E^{(1)\pm}_{\rm NG})^2=\varepsilon_{\rm NG}^2+\mu_{\rm NG}^2+\lvert\Delta_{\rm NG}\rvert^2\pm2[\mu_{\rm NG}^2\varepsilon_{\rm NG}^2+\lvert\Delta_{\rm NG}\rvert^2(q^1)^2]^{1/2}$"),
    (G6, r"$\nu_{\rm NG}$", "nu",
     r"the factor 4 (one field) or 8 (doublet) of the isotropic grand potential $\lvert\Delta_{\rm NG}\rvert^2/G_{\rm NG}-\nu_{\rm NG}\int d^3q\,(2\pi)^{-3}(E^+_{\rm NG}+E^-_{\rm NG})$"),
    (G6, r"$\Lambda_{\rm NG}$", "Lambda",
     r"the momentum cutoff: momentum integrals of the four-dimensional world are cut off at $\lvert\vec q\rvert=\Lambda_{\rm NG}$ (Euclidean four-momenta at $\varkappa_{\rm NG}=\Lambda_{\rm NG}$); $\Lambda_{\rm NG}=1$ sets the units"),
    (G6, r"$\upsilon_{\rm NG}$, $\zeta_{\rm NG}$", "ups zeta",
     r"the angle of the non-unitary vector states $\Delta^0_{\rm NG}=\lvert\Delta_{\rm NG}\rvert\cos\upsilon_{\rm NG}$, $\Delta^5_{\rm NG}=i\lvert\Delta_{\rm NG}\rvert\sin\upsilon_{\rm NG}$, and $\zeta_{\rm NG}=\lvert\sin2\upsilon_{\rm NG}\rvert$"),
    (G6, r"$\hat n_{\rm NG}$", "",
     r"the unit vector in ordinary space along which a spin-one pair points"),
    (G6, r"$q^{\rm F}_{\rm NG}$", "q_F",
     r"the Fermi momentum $(\mu_{\rm NG}^2-m_{\rm NG}^2)^{1/2}$ for $\mu_{\rm NG}>m_{\rm NG}$"),
    (G6, r"$\xi_{\rm NG}$", "",
     r"the energy $\varepsilon_{\rm NG}\pm\mu_{\rm NG}$ of a particle (or hole) measured from the chemical potential"),
    (G6, r"$\delta_{\rm NG}$, $w_{\rm NG}$", "w",
     r"the small real amplitude and the frequency of an oscillation $\lvert\Delta_{\rm NG}\rvert+\delta_{\rm NG}\exp(-iw_{\rm NG}x4)$ of the gap"),
    (G6, r"$\Pi^{\rm A}_{\rm NG}$; $c^\sigma_{\rm NG}$; $m^\sigma_{\rm NG}$", "Pi_A Pi^A c_sigma m_sigma",
     r"the inverse propagator of the amplitude oscillation (random-phase approximation), $\Pi^{\rm A}_{\rm NG}(w_{\rm NG})=(4\lvert\Delta_{\rm NG}\rvert^2-w_{\rm NG}^2)c^\sigma_{\rm NG}(w_{\rm NG})$ with $c^\sigma_{\rm NG}(w_{\rm NG})=\tfrac12\nu_{\rm NG}\int d^3q\,(2\pi)^{-3}\sum_\pm\{E^\pm_{\rm NG}[4(E^\pm_{\rm NG})^2-w_{\rm NG}^2]\}^{-1}$; the mass $m^\sigma_{\rm NG}=2\lvert\Delta_{\rm NG}\rvert$ of the amplitude mode"),
    (G6, r"$\mathcal T^{ab}_{\rm NG}$", "Rot",
     r"the generator of rotations of the fibre in the plane of $xa$ and $xb$ ($a<b$); it changes $\Delta^a_{\rm NG}$ by $-\Delta^b_{\rm NG}$ and $\Delta^b_{\rm NG}$ by $+\Delta^a_{\rm NG}$ per unit parameter (plain text: Rot(0,5) is $\mathcal T^{05}_{\rm NG}$)"),
    (G6, r"$F_{\rm NG}$", "F",
     r"the generator of the fermion number $\psi_{\rm NG}\to\exp(i\varphi_{\rm NG})\psi_{\rm NG}$; it multiplies every pair amplitude by $2i$ (plain text: U(1)\_F is its group)"),
    # ------------------------------------------------------------------ 7
    (G7, r"$\deg_{\rm NG}$", "deg",
     r"the superficial degree of divergence of a one-particle-irreducible graph: the power of the cutoff with which it grows when all its loop momenta are large"),
    (G7, r"$e^{\rm F}_{\rm NG}$, $e^{\rm B}_{\rm NG}$", "",
     r"the numbers of external fermion and boson lines of a graph"),
    (G7, r"$\mathcal V_{\rm NG}$", "nvert",
     r"the number of vertices of a graph"),
    (G7, r"$\ell_{\rm NG}$; $\mathrm r_4$", "ell r4",
     r"the squared length $\sum_a(n^a_{\rm NG})^2$ of the integer vector of a Kaluza-Klein mode on the torus fibre, and the number $\mathrm r_4(\ell_{\rm NG})$ of integer vectors of four components with that squared length (Jacobi: 8 times the sum of the divisors of $\ell_{\rm NG}$ that are not multiples of 4; $\mathrm r_4(0)=1$)"),
    (G7, r"$\varkappa_{\rm NG}$", "",
     r"the Euclidean four-momentum of the world (and its length) in the one-loop bubble"),
    (G7, r"$f_{\rm NG}$", "f",
     r"the one-loop bubble of one four-dimensional field as a function of its squared mass, $f_{\rm NG}((m^{\rm eff}_{\rm NG})^2)=\int_{\varkappa_{\rm NG}<\Lambda_{\rm NG}}d^4\varkappa_{\rm NG}(2\pi)^{-4}(\varkappa^2_{\rm NG}+(m^{\rm eff}_{\rm NG})^2)^{-2}$"),
    (G7, r"$\mathcal B_{\rm NG}$", "B",
     r"the one-loop bubble summed over the Kaluza-Klein tower of a torus fibre up to the cutoff, $\sum_{\ell_{\rm NG}\le(\Lambda_{\rm NG}R_{\rm NG})^2}\mathrm r_4(\ell_{\rm NG})f_{\rm NG}(m_{1671}^2+\ell_{\rm NG}/R_{\rm NG}^2)$"),
    (G7, r"$\mathcal C^{(8)}_{\rm NG}$; $\varpi_{\rm NG}$", "C8 u",
     r"the coefficient of the eight-dimensional growth $\mathcal B_{\rm NG}\to(\Lambda_{\rm NG}R_{\rm NG})^4\mathcal C^{(8)}_{\rm NG}$, $\mathcal C^{(8)}_{\rm NG}=\tfrac18\int_0^1d\varpi_{\rm NG}\,\varpi_{\rm NG}^3[\ln(1+\varpi_{\rm NG}^{-2})-(1+\varpi_{\rm NG}^2)^{-1}]$, with the integration variable $\varpi_{\rm NG}$ (the length of the fibre momentum divided by $\Lambda_{\rm NG}$)"),
    # ------------------------------------------------------------------ 8
    (G8, r"$\mathbb R$, $\mathbb C$", "",
     r"the real and the complex numbers"),
    (G8, r"$\mathfrak{so}(8)$, $\mathfrak{so}(7,1)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{so}(3,1)$, $\mathfrak{so}(1,3)$, $\mathfrak{so}(4)$, $\mathfrak{so}(3)$, $\mathfrak{so}(6,1)$, $\mathfrak{so}(4,3)$, $\mathfrak{su}(2)$",
     "so(8) so(7,1) so(4,4) so(3,1) so(1,3) so(4) so(3) so(6,1) so(4,3) su(2) gl(8)",
     r"Lie algebras of the linear maps that preserve a quadratic form: $\mathfrak{so}(8)$ for a definite form in eight dimensions, $\mathfrak{so}(7,1)$ and $\mathfrak{so}(4,4)$ for the metrics $\eta_{1671}$ and $\eta_{1644}$, $\mathfrak{so}(3,1)$ for the world, $\mathfrak{so}(4)$ and $\mathfrak{so}(1,3)$ for the fibres of $7+1$ and $4+4$, $\mathfrak{so}(3)$ for ordinary space, $\mathfrak{so}(6,1)$ and $\mathfrak{so}(4,3)$ for the directions orthogonal to a fibre vector on $7+1$ and on $4+4$, $\mathfrak{so}(8,\mathbb C)$ for a complex form in eight dimensions; $\mathfrak{su}(2)$ is the Lie algebra of the special unitary group of two dimensions (plain text gl(8): all real $8\times8$ matrices)"),
    (G8, r"$SO(2)$, $SO(3)$, $SO(4)$, $SO(1,1)$, $U(1)$, $SU(2)$", "SO(2) SO(3) SO(4) U(1) SU(2) U(1)_R SO(1,1)_R U(1)_F Sym(3) O(N) Z2",
     r"the rotation groups, the group of phases and the special unitary group of two dimensions (plain text: U(1)\_R and SO(1,1)\_R are the groups of the R-symmetry of $7+1$ and $4+4$, U(1)\_F the group of the fermion number, Sym(3) the group of permutations of three objects, O(N) the group that preserves the norm $N_{\rm NG}$, Z2 the group of two elements; a Z2 grading splits an algebra into an even and an odd part)"),
    (G8, r"$\mathrm{tr}$, $\det$, $\mathrm{diag}$, $\mathrm{Re}$", "tr det diag Re Im span ln sqrt sin cos int sum dim",
     r"trace, determinant, diagonal matrix with the listed entries, real part (plain text: Im is the imaginary part, of an algebra the set of its imaginary elements; span the set of real linear combinations; ln, sqrt, sin, cos the natural logarithm, the square root, sine and cosine; int and sum an integral and a sum; dim a dimension)"),
    (G8, r"${}^T$, ${}^\dagger$, ${}^*$; $[\ ,\ ]$, $\{\ ,\ \}$, $[\ ,\ \}$; $\langle\ \rangle$; $\otimes$; h.c.", "",
     r"transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); commutator, anticommutator and graded bracket (anticommutator of two odd generators, commutator otherwise); expectation value in the ground state; tensor product; the Hermitian conjugate of the preceding terms"),
]


def read_master(path):
    with open(path, encoding="utf-8") as f:
        rows = list(csv.DictReader(line for line in f if not line.startswith("#")))
    return {r["symbol"]: r for r in rows}


def main():
    mm = read_master(MASTER_MAIN)
    md = read_master(MASTER_DEFL)
    out = []
    for entry in TAKEN:
        group, sym, src = entry[:3]
        reg = mm if src == "main" else md
        if sym not in reg:
            raise SystemExit("taken symbol %r not found in the master registry %s" % (sym, src))
        r = reg[sym]
        meaning = r["meaning"]
        if sym in USE_NOTE:
            meaning = meaning.rstrip()
            if not meaning.endswith("."):
                meaning += "."
            meaning = meaning + " " + USE_NOTE[sym]
        # the plain-text names: those of the master file, unless Revision_NoGo uses others
        # (a fourth field; the master names of this entry occur only in other folders)
        names = entry[3] if len(entry) > 3 else r["names_in_files"]
        out.append({"group": group, "symbol": sym, "names_in_files": names, "meaning": meaning,
                    "origin": "taken from the MASTER registry of the %s" % ("main project" if src == "main" else "revision on the deflating spacetimes")})
    for group, sym, names, meaning in NEW:
        out.append({"group": group, "symbol": sym, "names_in_files": names, "meaning": meaning, "origin": "new (Revision_NoGo)"})
    seen = set()
    for r in out:
        if r["symbol"] in seen:
            raise SystemExit("symbol registered twice: %r" % r["symbol"])
        seen.add(r["symbol"])
    out.sort(key=lambda r: r["group"])
    os.makedirs(os.path.join(ROOT, "provenance", "generated"), exist_ok=True)
    with open(os.path.join(ROOT, "provenance", "symbols_ng.csv"), "w", encoding="utf-8", newline="") as f:
        f.write("# PROJECT spacetimes, Revision_NoGo: registry of the symbols (written by code/scripts/write_symbol_registry.py)\n")
        w = csv.DictWriter(f, fieldnames=["group", "symbol", "names_in_files", "meaning", "origin"])
        w.writeheader()
        for r in out:
            w.writerow(r)
    md_lines = []
    tex_lines = []
    group = None
    for r in out:
        if r["group"] != group:
            group = r["group"]
            if md_lines:
                md_lines.append("")
            md_lines.append("**%s**" % group)
            md_lines.append("")
            md_lines.append("| symbol | meaning | origin | plain-text names |")
            md_lines.append("|-----------|-------------------------|----|----------|")
            tex_lines.append("\\multicolumn{3}{l}{\\textbf{%s}}\\\\" % group)
        if "|" in r["meaning"] or "|" in r["symbol"]:
            raise SystemExit("a vertical bar in the entry %r would break the Markdown table; write \\lvert, \\rvert" % r["symbol"])
        md_names = " ".join("`%s`" % n for n in r["names_in_files"].split())
        md_lines.append("| %s | %s | %s | %s |" % (r["symbol"], r["meaning"], "taken" if r["origin"].startswith("taken") else "new", md_names))
        names = " ".join("\\texttt{%s}" % tex_name(n) for n in r["names_in_files"].split())
        tex_lines.append("%s & %s & %s\\\\" % (r["symbol"], r["meaning"], names))
    frag = []
    frag.append("<!-- BEGIN symbols_ng_table -->\n%s\n<!-- END symbols_ng_table -->" % "\n".join(md_lines))
    frag.append("<!-- BEGIN symbols_ng_texrows -->\n%s\n<!-- END symbols_ng_texrows -->" % "\n".join(tex_lines))
    frag.append("<!-- BEGIN symbols_ng_count -->\n%d\n<!-- END symbols_ng_count -->" % len(out))
    frag.append("<!-- BEGIN symbols_ng_count_taken -->\n%d\n<!-- END symbols_ng_count_taken -->" % len(TAKEN))
    frag.append("<!-- BEGIN symbols_ng_count_new -->\n%d\n<!-- END symbols_ng_count_new -->" % len(NEW))
    with open(os.path.join(ROOT, "provenance", "generated", "symbols_ng.md"), "w", encoding="utf-8") as f:
        f.write("\n\n".join(frag) + "\n")
    print("wrote provenance/symbols_ng.csv and provenance/generated/symbols_ng.md: %d entries (%d taken, %d new)" % (len(out), len(TAKEN), len(NEW)))


def tex_name(n):
    """A plain-text name for \\texttt{...} in LaTeX."""
    out = []
    for ch in n:
        out.append({"_": "\\_", "^": "\\^{}", "&": "\\&", "%": "\\%", "#": "\\#", "<": "$<$", ">": "$>$", "{": "\\{", "}": "\\}", "~": "\\~{}"}.get(ch, ch))
    return "".join(out)


if __name__ == "__main__":
    main()
