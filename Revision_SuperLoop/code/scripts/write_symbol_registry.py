#!/usr/bin/env python3
"""
write_symbol_registry.py (Revision_SuperLoop) -- the registry of every symbol that the documents,
the programs, the figures and the data files of the folder Revision_SuperLoop use.

Hard Rule 5 of the project requires that every symbol denote ONE object in the whole
project.  This registry has three kinds of entries:

  * TAKEN: symbols of the MASTER registries of the project (the registry of the main
    project, ../provenance/symbols.csv, and the MASTER registry of the revision on the
    deflating spacetimes, ../Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv)
    that Revision_SuperLoop uses with exactly the same meaning.  Their symbol text and meaning
    are copied from the master file (the one named in the entry), so that they cannot
    drift; a note on their use in Revision_SuperLoop may be appended;
  * NEW: the symbols that Revision_SuperLoop introduces.  Every one of them carries the suffix
    LL, or is an element of the bi-octonions written in bold upright type, a number set, a
    group, a Lie algebra, an operation sign, or a plain index that no master registry defines
    with another meaning (checked by code/scripts/check_symbols.py);
  * SHARED: a NEW entry may contain a symbol that the registry of the sibling folder
    Revision_NoGo (../Revision_NoGo/provenance/symbols_ng.csv, not a master registry) also
    contains; this is allowed only when both denote the same object, and every such symbol is
    listed in SHARED below with the object it denotes (check LL-S2 finds every coincidence).

The third field of a NEW entry lists the plain-text names under which the programs, the
figures and the data files of Revision_SuperLoop write the symbol (each name belongs to one
entry; check LL-S1; every name with the suffix _LL in the reports and data files must be one
of them, check LL-S8).

Output:
    provenance/symbols_ll.csv                  the registry (machine-readable)
    provenance/generated/symbols_ll.md         the registry as a Markdown table and as
                                               LaTeX table rows (fragments for the documents)

Run from the folder Revision_SuperLoop:
    python3 code/scripts/write_symbol_registry.py
    python3 code/scripts/check_symbols.py
"""
import csv
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)
MASTER_MAIN = os.path.join(MAIN, "provenance", "symbols.csv")
MASTER_DEFL = os.path.join(MAIN, "Revision_Deflation_CJT_and_Kohn-Sham", "provenance", "symbols_master.csv")

G1 = "1 Indices, conventions and number sets"
G2 = "2 The bi-octonions and the super-loop"
G3 = "3 Operators on the super-loop and their eigenvalues"
G4 = "4 The covariant families and their classification"
G5 = "5 The manifolds"
G6 = "6 The spin-zero superfluid"
G7 = "7 Power counting and renormalizability"
G8 = "8 Groups, Lie algebras, operations and labels"

# (group, symbol text exactly as in the master file, which master file[, plain-text names in Revision_SuperLoop])
TAKEN = [
    (G1, r"$i,j$", "main", "j j' j''"),
    (G1, r"$D$", "defl"),
]

# Notes on the use in Revision_SuperLoop of a taken symbol (appended to the master meaning).
USE_NOTE = {
    r"$i,j$": r"In Revision\_SuperLoop only $j$ is used as an index, with the values 1, 2, 3, and with primes ($j'$, $j''$) where two or three such indices are needed: it labels the three imaginary quaternion units $\mathbf e_j$, which correspond to the three space directions of $\mathbb R^{1,3}_{\rm LL}$, the coordinates $x^j_{\rm LL}$, the momentum components $q^j_{\rm LL}$, the rotation and boost generators $J^j_{\rm LL}$, $K^j_{\rm LL}$, and the Pauli matrices $\sigma^{\rm LL}_j$ and Dirac matrices $\alpha^j_{\rm LL}$, $\gamma^j_{\rm LL}$. The letter $i$ is the imaginary unit (entry $i$); the isospin index is $\iota$ (a different letter).",
    r"$D$": r"In Revision\_SuperLoop $D=3$ for the four-dimensional world of the superfluid and $D=7$ for the eight-dimensional manifolds (on $4+4$ power counting sees the total dimension 8 as well); the value 4, the coordinates $x5,x6,x7$ of the eight-dimensional spacetime of the project (on which such fields do not depend), and the volume and the area of the unit ball do not occur.",
}

# Symbols that the registry of the sibling folder Revision_NoGo also contains, with the object
# that both denote.  Check LL-S2 requires that every symbol of a NEW entry that coincides with a
# symbol of that registry is listed here, and that nothing else is.
SHARED = {
    r"$i$": "the imaginary unit, in both folders",
    r"$\mathbb R$": "the real numbers, in both folders",
    r"$\mathbb C$": "the complex numbers, in both folders",
    r"$p$": "an index over the basis of an algebra, in both folders (here the bi-octonions, there the octonions and split octonions)",
    r"$\mathfrak g_2$": "the compact exceptional Lie algebra of dimension 14, in both folders",
    r"$\mathrm G_2$": "the compact exceptional group of automorphisms of the octonions, in both folders",
    r"$\mathfrak{so}(8)$": "the Lie algebra of the rotations of eight-dimensional Euclidean space, in both folders",
    r"$\mathfrak{so}(4,4)$": "the Lie algebra that preserves a quadratic form of signature (4,4), in both folders",
    r"$\mathfrak{so}(8,\mathbb C)$": "the Lie algebra that preserves a complex quadratic form in eight dimensions, in both folders",
    r"$\mathfrak{so}(3,1)$": "the Lorentz algebra of a world with one time and three space dimensions, in both folders",
    r"$\mathfrak{so}(3)$": "the Lie algebra of the rotations of three-dimensional space, in both folders",
    r"$\mathfrak{su}(2)$": "the Lie algebra of the special unitary group of two dimensions, in both folders",
    r"$SU(2)$": "the special unitary group of two dimensions, in both folders",
    r"$U(1)$": "the group of the phases, in both folders",
    r"$\mathrm{tr}$": "the trace of a matrix, in both folders",
    r"$\det$": "the determinant of a matrix, in both folders",
    r"$\mathrm{Re}$": "the real part, in both folders",
    r"$\mathrm{diag}$": "the diagonal matrix with the listed entries, in both folders",
    r"${}^T$": "the transpose, in both folders",
    r"$\otimes$": "the tensor product, in both folders",
    r"$\langle\ \rangle$": "the expectation value in the ground state, in both folders",
}

# (group, symbol, plain-text names in programs, figures and data files of Revision_SuperLoop, meaning)
NEW = [
    # ------------------------------------------------------------------ 1
    (G1, r"$i$", "",
     r"the imaginary unit, $i^2=-1$; in $\mathbb B_{\rm LL}$ it is the imaginary unit of the complex coefficients: it commutes and associates with every bi-octonion and is not one of the units $\mathbf e_1,\dots,\mathbf e_7$"),
    (G1, r"$\mathbb R$, $\mathbb C$", "R C R^8 C^3 C^8",
     r"the real and the complex numbers (plain text: `R^8`, `C^3`, `C^8` are the real or complex vector spaces of that dimension)"),
    (G1, r"$p$, $p'$", "p p' p1 p2 p3",
     r"a summation index over the basis of an algebra (values $0,\dots,7$, or $0,\dots,3$ for the quaternions): in Revision\_SuperLoop it labels the units $\mathbf e_p$ of $\mathbb B_{\rm LL}$, the coefficients $\xi^{\rm LL}_p(\mathbf x)$, the coordinates $x^p_{\rm LL}$ of $\mathbb R^{1,7}_{\rm LL}$ and $\mathbb R^{1,3}_{\rm LL}$ and the Dirac matrices $\gamma^p_{\rm LL}$ ($p=0,1,2,3$); $p'$ is a second index of the same kind (plain text `p1`, `p2`, `p3`: three such indices)"),
    (G1, r"$\iota$, $\iota'$, $\iota''$", "iota",
     r"the isospin index, values 1, 2, 3: it labels the isospin generators $T^\iota_{\rm LL}$, the Pauli matrices $\tau^{\rm LL}_\iota$ on the isospin index of a field, the pair channels $\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, the pair operators $\Pi^\iota_{\rm LL}$ and the components $d^\iota_{\rm LL}$ of the order parameter; it is an internal direction, not a direction in space (that is $j$)"),
    (G1, r"$\epsilon^{\rm LL}_{jj'j''}$", "eps3",
     r"the Levi-Civita symbol: $+1$ if $(j,j',j'')$ is an even permutation of $(1,2,3)$, $-1$ if it is an odd one, $0$ otherwise; also with the isospin indices, $\epsilon^{\rm LL}_{\iota\iota'\iota''}$"),
    (G1, r"$\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_j$", "sigma_0 sigma_j sigma_1 sigma_2 sigma_3",
     r"the $2\times2$ unit matrix $\sigma^{\rm LL}_0$ and the Pauli matrices $\sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, acting on the two components of a Weyl spinor or on the species index"),
    (G1, r"$\tau^{\rm LL}_\iota$", "tau_iota tau_1 tau_2 tau_3",
     r"the same three Pauli matrices acting on the isospin index of a field (the index of the two components up and down of an isodoublet)"),
    (G1, r"$\mathsf E_{\rm LL}$", "eps2",
     r"the real antisymmetric $2\times2$ matrix $i\sigma^{\rm LL}_2=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$, with $\mathsf E_{\rm LL}^{-1}=\mathsf E_{\rm LL}^T=-\mathsf E_{\rm LL}$; it acts on a Weyl, isospin or species index, as stated at each place"),
    (G1, r"$\mathsf 1_4$", "",
     r"the $4\times4$ unit matrix"),
    # ------------------------------------------------------------------ 2
    (G2, r"$\mathbb H$, $\mathbb O$", "H O",
     r"the quaternions and the octonions: the real combinations of $\mathbf e_0,\dots,\mathbf e_3$ and of $\mathbf e_0,\dots,\mathbf e_7$; ${\rm Im}\,\mathbb H$ and ${\rm Im}\,\mathbb O$ are their imaginary parts, the real combinations of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ and of $\mathbf e_1,\dots,\mathbf e_7$ (plain text `Im H`, `Im O`; `C (x) H` is $\mathbb C\otimes\mathbb H$, the bi-quaternions)"),
    (G2, r"$\mathbb B_{\rm LL}$", "B_LL",
     r"the bi-octonions $\mathbb C\otimes\mathbb O$: the complex combinations $\mathbf x=\sum_{p=0}^7\xi^{\rm LL}_p(\mathbf x)\,\mathbf e_p$ of the eight units, with the product of the units extended bilinearly; real dimension 16"),
    (G2, r"$\mathbf e_0,\dots,\mathbf e_7$; $\mathbf e_p$, $\mathbf e_j$, $\mathbf e_\iota$", "e0 e1 e2 e3 e4 e5 e6 e7 e_p e_p' e_j e_iota e_p1 e_p2 e_p3 ie1 ie2 ie3 ie4 ie5 ie6 ie7",
     r"the units of $\mathbb B_{\rm LL}$, of $\mathbb O$ and of $\mathbb H$: $\mathbf e_0=1$; $\mathbf e_1,\mathbf e_2,\mathbf e_3$ the quaternion units, $\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically; $\mathbf e_4$ the unit of the Cayley-Dickson doubling, $\mathbf e_4^2=-1$; $\mathbf e_5=\mathbf e_1\mathbf e_4$, $\mathbf e_6=\mathbf e_2\mathbf e_4$, $\mathbf e_7=\mathbf e_3\mathbf e_4$ ($\mathbf e_\iota$: a quaternion unit labelled by the isospin index; plain text `ie1`, ..., `ie7`: the bi-octonions $i\mathbf e_1,\dots,i\mathbf e_7$)"),
    (G2, r"$\xi^{\rm LL}_p(\mathbf x)$", "",
     r"the complex coefficient of the unit $\mathbf e_p$ in the bi-octonion $\mathbf x$"),
    (G2, r"$\mathbf x$, $\mathbf y$, $\mathbf z$, $\mathbf w$", "x y z w",
     r"elements of $\mathbb B_{\rm LL}$ (bi-octonions); $\mathbf w$ is also a point of the super-loop manifold $\mathbb L_{\rm LL}$"),
    (G2, r"$\mathbf a$, $\mathbf b$, $\mathbf c$, $\mathbf d$; $\mathbf b'$", "aH bS cH dS bS'",
     r"bi-quaternions: elements of $\mathbb C\otimes\mathbb H$, the complex combinations of $\mathbf e_0,\mathbf e_1,\mathbf e_2,\mathbf e_3$; every bi-octonion is written $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ (Cayley-Dickson form), and $\mathbf b\,\mathbf e_4$, $\mathbf b'\,\mathbf e_4$ are spinor charges"),
    (G2, r"$\bar{\mathbf x}$, $\mathbf x^*$, $\mathbf x^\dagger$", "xbar x^* x^dag aHbar bSbar bS'bar cHbar dSbar gLbar hLbar rPbar rP'bar bS^* bS'^* gL^dag uL^dag",
     r"the three conjugations (also of bi-quaternions): $\bar{\mathbf x}$ changes the signs of the coefficients of $\mathbf e_1,\dots,\mathbf e_7$ (it is complex-linear, $\overline{\mathbf x\mathbf y}=\bar{\mathbf y}\,\bar{\mathbf x}$, and $\overline{\mathbf a+\mathbf b\mathbf e_4}=\bar{\mathbf a}-\mathbf b\mathbf e_4$); $\mathbf x^*$ replaces every complex coefficient by its complex conjugate; $\mathbf x^\dagger=(\bar{\mathbf x})^*$ (plain text: `xbar`, `x^*`, `x^dag`, and likewise `aHbar`, `gL^dag`, ...)"),
    (G2, r"$N_{\rm LL}(\mathbf x)$, $N_{\rm LL}(\mathbf x,\mathbf y)$", "N_LL",
     r"the complex norm $N_{\rm LL}(\mathbf x)=\sum_p\xi^{\rm LL}_p(\mathbf x)^2$, with $\mathbf x\bar{\mathbf x}=\bar{\mathbf x}\mathbf x=N_{\rm LL}(\mathbf x)\,\mathbf e_0$, and its polarization $N_{\rm LL}(\mathbf x,\mathbf y)=\sum_p\xi^{\rm LL}_p(\mathbf x)\,\xi^{\rm LL}_p(\mathbf y)$, $N_{\rm LL}(\mathbf x,\mathbf x)=N_{\rm LL}(\mathbf x)$ (complex-valued; for bi-quaternions the determinant of the $2\times2$ matrix $M_{\rm LL}$)"),
    (G2, r"$[\mathbf x,\mathbf y]$; $[\mathbf x,\mathbf y,\mathbf z]$", "[x,y] [x,y,z]",
     r"the commutator $\mathbf x\mathbf y-\mathbf y\mathbf x$ and the associator $(\mathbf x\mathbf y)\mathbf z-\mathbf x(\mathbf y\mathbf z)$ of bi-octonions (the same brackets of other elements are defined at their place: the bracket of an algebra $\mathcal F^{\rm A}_{\rm LL}$, $\mathcal F^{\rm B}_{\rm LL}$, $\mathcal F^{\rm C}_{\rm LL}$, and the commutator of two matrices or operators, the product in one order minus the product in the other)"),
    (G2, r"$\mathrm{Jac}_{\rm LL}$", "Jac",
     r"the Jacobiator $\mathrm{Jac}_{\rm LL}(\mathbf x,\mathbf y,\mathbf z)=[[\mathbf x,\mathbf y],\mathbf z]+[[\mathbf y,\mathbf z],\mathbf x]+[[\mathbf z,\mathbf x],\mathbf y]$ of a bracket; it vanishes identically exactly in a Lie algebra"),
    (G2, r"$L^{\rm LL}_{\mathbf x}$, $R^{\rm LL}_{\mathbf x}$", "L_x L_y L_e1 L_e4 L_[e1,e4] L_(e_p) L_(e_p') R_x R_y",
     r"the left and right multiplications $\mathbf y\mapsto\mathbf x\mathbf y$ and $\mathbf y\mapsto\mathbf y\mathbf x$ by a bi-octonion, real $16\times16$ matrices (on $\mathbb O$ or $\mathbb O^{\rm s}_{\rm LL}$: $8\times8$)"),
    (G2, r"$\mathbb L_{\rm LL}$", "L_LL",
     r"the Lorentz super-loop: the unit bi-octonions, $\mathbb L_{\rm LL}=\{\mathbf w\in\mathbb B_{\rm LL}:N_{\rm LL}(\mathbf w)=1\}$, with the product of $\mathbb B_{\rm LL}$; a Moufang loop (not a group), and a manifold of real dimension 14"),
    (G2, r"$\mathbf g$, $\mathbf h$", "gL hL",
     r"elements of $SL(2,\mathbb C)$, i.e. unit bi-quaternions ($N_{\rm LL}(\mathbf g)=1$), a subgroup of $\mathbb L_{\rm LL}$"),
    (G2, r"$\mathbf v$", "vI",
     r"a unit (bi-)quaternion that rotates the isospin of the spinor charges (from the left)"),
    (G2, r"$\mathfrak m_{\rm LL}$, $\mathfrak l_{\rm LL}$, $\mathfrak s_{\rm LL}$", "m_LL l_LL s_LL",
     r"the Malcev algebra of the super-loop, $\mathfrak m_{\rm LL}=\mathbb C\otimes{\rm Im}\,\mathbb O$ with the commutator (real dimension 14), its Lorentz part $\mathfrak l_{\rm LL}=\mathbb C\otimes{\rm Im}\,\mathbb H$ (6, a Lie algebra isomorphic to $\mathfrak{so}(3,1)$), and the space of spinor charges $\mathfrak s_{\rm LL}=(\mathbb C\otimes\mathbb H)\,\mathbf e_4$ (8); $\mathfrak m_{\rm LL}=\mathfrak l_{\rm LL}\oplus\mathfrak s_{\rm LL}$"),
    (G2, r"$J^j_{\rm LL}$, $K^j_{\rm LL}$", "J_LL^j K_LL^j J_LL^j' K_LL^j' J_LL^j'' K_LL^j'' J_LL^3",
     r"the rotation and boost generators of $\mathfrak l_{\rm LL}$, $J^j_{\rm LL}=\mathbf e_j/2$ and $K^j_{\rm LL}=i\,\mathbf e_j/2$"),
    (G2, r"$\mathbf u$, $\mathbf u'$, $\mathbf u_1$, $\mathbf u_2$", "uL uL' uL1 uL2",
     r"elements of the Lorentz part $\mathfrak l_{\rm LL}$ (Lorentz generators): imaginary bi-quaternions"),
    (G2, r"$\mathbf t$, $\mathbf t'$", "tI tI'",
     r"elements of ${\rm Im}\,\mathbb H$ (real imaginary quaternions): isospin generators, which act on the spinor charges from the left"),
    (G2, r"$W_{\rm LL}(\mathbf u)$", "W_LL",
     r"the Weyl representation of $\mathfrak l_{\rm LL}$ on bi-quaternions, $W_{\rm LL}(\mathbf u)\mathbf b=-\mathbf b\,\mathbf u$: a Lie representation, $[W_{\rm LL}(\mathbf u_1),W_{\rm LL}(\mathbf u_2)]=W_{\rm LL}([\mathbf u_1,\mathbf u_2])$"),
    (G2, r"$\rho_{\rm LL}(\mathbf u)$", "rho_LL",
     r"the inner action of a Lorentz element on the spinor charges, $[\mathbf u,\mathbf b\,\mathbf e_4]=(\rho_{\rm LL}(\mathbf u)\mathbf b)\,\mathbf e_4$ with $\rho_{\rm LL}=-2W_{\rm LL}$; not a representation"),
    (G2, r"$\Theta_{\rm LL}$", "Theta_LL Theta_LL^2",
     r"the twist charge: the linear map of $\mathfrak m_{\rm LL}$ that is $1$ on $\mathfrak l_{\rm LL}$ and $-2$ on $\mathfrak s_{\rm LL}$; the inner action is $[\mathbf u,\cdot\,]=D_{\rm LL}(\mathbf u)\circ\Theta_{\rm LL}$ (as a number: the eigenvalue $1$ or $-2$, or the value of $c^{\rm tw}_{\rm LL}$ in the families)"),
    (G2, r"$D_{\rm LL}(\mathbf u)$, $D^{\rm v}_{\rm LL}(\mathbf t)$", "D_LL Dv_LL",
     r"the Lorentz and isospin derivations of $\mathbb B_{\rm LL}$: $D_{\rm LL}(\mathbf u)(\mathbf a+\mathbf b\mathbf e_4)=[\mathbf u,\mathbf a]+(-\mathbf b\mathbf u)\mathbf e_4$ and $D^{\rm v}_{\rm LL}(\mathbf t)(\mathbf a+\mathbf b\mathbf e_4)=(\mathbf t\mathbf b)\mathbf e_4$ (both linear in their argument)"),
    (G2, r"$\mathcal D_{\rm LL}$", "",
     r"a derivation of $\mathbb B_{\rm LL}$: a linear map with $\mathcal D_{\rm LL}(\mathbf x\mathbf y)=(\mathcal D_{\rm LL}\mathbf x)\mathbf y+\mathbf x(\mathcal D_{\rm LL}\mathbf y)$, the infinitesimal form of an automorphism"),
    (G2, r"$\varphi^{\rm LL}_{\mathbf g,\mathbf v}$", "varphi",
     r"the automorphism $\mathbf a+\mathbf b\mathbf e_4\mapsto\mathbf g\mathbf a\bar{\mathbf g}+(\mathbf v\mathbf b\bar{\mathbf g})\mathbf e_4$ of $\mathbb B_{\rm LL}$ ($\mathbf g$ a Lorentz transformation, $\mathbf v$ an isospin rotation)"),
    (G2, r"$\varrho_{\rm LL}$; $\mathsf O_1$, $\mathsf O_2$, $\mathsf O_3$", "",
     r"a linear map from $\mathfrak m_{\rm LL}$ to the linear operators (matrices) on a vector space that turns brackets into commutators, $\varrho_{\rm LL}([\mathbf x,\mathbf y])=\varrho_{\rm LL}(\mathbf x)\varrho_{\rm LL}(\mathbf y)-\varrho_{\rm LL}(\mathbf y)\varrho_{\rm LL}(\mathbf x)$ (Theorem R); $\mathsf O_1,\mathsf O_2,\mathsf O_3$ three such operators"),
    (G2, r"$\mathrm{Kil}_{\rm LL}$; $\mathrm{ad}$", "Killing ad",
     r"the Killing form $\mathrm{Kil}_{\rm LL}(\mathbf x,\mathbf y)=\mathrm{tr}(\mathrm{ad}\,\mathbf x\,\mathrm{ad}\,\mathbf y)$ of a Lie algebra, where $\mathrm{ad}\,\mathbf x$ is the matrix of the map $\mathbf y\mapsto[\mathbf x,\mathbf y]$"),
    # ------------------------------------------------------------------ 3
    (G3, r"$Q_{\rm LL}$", "Q_LL",
     r"the electric charge, $Q_{\rm LL}=T^3_{\rm LL}$: eigenvalues $0$ on $\mathbb C\otimes\mathbb H$ and $\pm\tfrac12$ on the spinor charges"),
    (G3, r"$T^\iota_{\rm LL}$, $(T_{\rm LL})^2$", "T_LL^iota T_LL^3 T_LL^2",
     r"the isospin generators $T^\iota_{\rm LL}=i\,D^{\rm v}_{\rm LL}(\mathbf e_\iota/2)$, Hermitian, $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$ and cyclically, and the isospin Casimir $(T_{\rm LL})^2=\sum_\iota(T^\iota_{\rm LL})^2$"),
    (G3, r"$\mathrm{Cas}_{\rm LL}$", "Cas_LL",
     r"the Casimir operator of the rotations, $\mathrm{Cas}_{\rm LL}=-\sum_jD_{\rm LL}(J^j_{\rm LL})^2$; its eigenvalue is 0 for spin 0, 3/4 for spin 1/2 and 2 for spin 1"),
    # ------------------------------------------------------------------ 4
    (G4, r"$\mathcal F^{\rm A}_{\rm LL}$, $\mathcal F^{\rm B}_{\rm LL}$, $\mathcal F^{\rm C}_{\rm LL}$", "F_A F_B F_C",
     r"the three families of covariant brackets: $\mathcal F^{\rm A}_{\rm LL}$ on $\mathfrak l_{\rm LL}\oplus\mathfrak s_{\rm LL}$ (dimension 14), $\mathcal F^{\rm B}_{\rm LL}$ on $\mathfrak l_{\rm LL}\oplus\mathfrak p_{\rm LL}\oplus\mathfrak s_{\rm LL}$ (18), $\mathcal F^{\rm C}_{\rm LL}$ on $\mathfrak l_{\rm LL}\oplus\mathfrak{su}(2)\oplus\mathfrak s_{\rm LL}$ (17)"),
    (G4, r"$\mathfrak p_{\rm LL}$; $\mathbf r$, $\mathbf r'$", "p_LL rP rP'",
     r"the translations of the family $\mathcal F^{\rm B}_{\rm LL}$: the real space of the Hermitian bi-quaternions $\mathbf r=\mathbf r^\dagger$ (dimension 4), on which $\mathfrak l_{\rm LL}$ acts by $[\mathbf u,\mathbf r]=\mathbf u\mathbf r+\mathbf r\mathbf u^\dagger$"),
    (G4, r"$c^{\rm tw}_{\rm LL}$, $c^{\rm sl}_{\rm LL}$, $c^{\rm PP}_{\rm LL}$, $c^{\rm Ps}_{\rm LL}$, $c^{\rm sP}_{\rm LL}$, $c^{\rm TT}_{\rm LL}$, $c^{\rm Ts}_{\rm LL}$, $c^{\rm sT}_{\rm LL}$", "c_tw c_sl c_PP c_Ps c_sP c_TT c_Ts c_sT csP cTT ctw_re ctw_im csl_re csl_im cPP_re cPP_im cPs_re cPs_im cTs_re cTs_im csT_re csT_im",
     r"the coefficients of the families: $[\mathbf u,\mathbf b\mathbf e_4]=c^{\rm tw}_{\rm LL}(W_{\rm LL}(\mathbf u)\mathbf b)\mathbf e_4$ (the twist), $[\mathbf b\mathbf e_4,\mathbf b'\mathbf e_4]\supset c^{\rm sl}_{\rm LL}(\bar{\mathbf b}\mathbf b'-\bar{\mathbf b}'\mathbf b)$, $[\mathbf r,\mathbf r']=c^{\rm PP}_{\rm LL}(\mathbf r\bar{\mathbf r}'-\mathbf r'\bar{\mathbf r})$, $[\mathbf r,\mathbf b\mathbf e_4]=c^{\rm Ps}_{\rm LL}(\mathbf b^*\bar{\mathbf r})\mathbf e_4$, $[\mathbf b\mathbf e_4,\mathbf b'\mathbf e_4]\supset c^{\rm sP}_{\rm LL}\,i(\bar{\mathbf b}\mathbf b'^*-\bar{\mathbf b}'\mathbf b^*)$, $[\mathbf t,\mathbf t']=c^{\rm TT}_{\rm LL}(\mathbf t\mathbf t'-\mathbf t'\mathbf t)$, $[\mathbf t,\mathbf b\mathbf e_4]=c^{\rm Ts}_{\rm LL}(\mathbf t\mathbf b)\mathbf e_4$, $[\mathbf b\mathbf e_4,\mathbf b'\mathbf e_4]\supset\mathrm{Re}[c^{\rm sT}_{\rm LL}(\mathbf b\bar{\mathbf b}'-\mathbf b'\bar{\mathbf b})]$; complex except $c^{\rm sP}_{\rm LL}$, $c^{\rm TT}_{\rm LL}$ (real); plain text `ctw_re`, `ctw_im`: real and imaginary part of $c^{\rm tw}_{\rm LL}$, and likewise"),
    (G4, r"$\lambda^{\rm sc}_{\rm LL}$", "lambda_sc",
     r"a complex number $\neq0$ by which the spinor charges are rescaled, $\mathbf b\mapsto\lambda^{\rm sc}_{\rm LL}\mathbf b$"),
    (G4, r"$\mathcal I^{\rm Jac}_{\rm LL}$, $\mathcal I^{\rm Mal}_{\rm LL}$; $w^{\rm sat}_{\rm LL}$", "w_sat",
     r"the ideals of polynomials in the real coefficients of a family generated by the Jacobi and by the Malcev identity; $w^{\rm sat}_{\rm LL}$ the auxiliary variable of the saturation (the points of the variety at which a given polynomial does not vanish are the points of the variety of the ideal enlarged by that polynomial times $w^{\rm sat}_{\rm LL}$, minus 1)"),
    # ------------------------------------------------------------------ 5
    (G5, r"$\mathbb R^{1,7}_{\rm LL}$, $\mathbb R^{1,3}_{\rm LL}$; $x^p_{\rm LL}$", "R^(1,7)_LL R^(1,3)_LL x_LL^0 x_LL^p x_LL^1 x_LL^2 x_LL^3 x_LL^j",
     r"the Hermitian bi-octonions $\mathbf x=\mathbf x^\dagger=x^0_{\rm LL}\mathbf e_0+i\sum_{p=1}^7x^p_{\rm LL}\mathbf e_p$ (real $x^p_{\rm LL}$): the $7+1$ Minkowski space with the real norm $N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_p(x^p_{\rm LL})^2$; and its subspace of Hermitian bi-quaternions ($p\le3$), Minkowski space of $3+1$ dimensions"),
    (G5, r"$\mathbb O^{\rm s}_{\rm LL}$", "Os R^(4,4)",
     r"the split octonions inside $\mathbb B_{\rm LL}$: the real space $\mathbb H+(i\mathbb H)\mathbf e_4$ (real combinations of $\mathbf e_0,\dots,\mathbf e_3,i\mathbf e_4,\dots,i\mathbf e_7$), closed under the product, on which $N_{\rm LL}$ has signature $(4,4)$: the $4+4$ manifold (plain text `R^(4,4)`)"),
    (G5, r"$S^7_{\rm LL}$, $S^{3,4}_{\rm LL}$", "S^7 S^(3,4)",
     r"the unit elements ($N_{\rm LL}=1$) of $\mathbb O$ (the seven-sphere) and of $\mathbb O^{\rm s}_{\rm LL}$ (the pseudo-sphere): the two real forms of $\mathbb L_{\rm LL}$, both Moufang loops"),
    (G5, r"$X^{\rm LL}_{\mathbf x}$", "X_x X_y",
     r"the frame field $X^{\rm LL}_{\mathbf x}(\mathbf w)=\mathbf w\mathbf x$ on $\mathbb L_{\rm LL}$ for imaginary $\mathbf x$ (a vector field: the velocity of the curve $\mathbf w(\theta_{\rm LL})$ with $d\mathbf w/d\theta_{\rm LL}=\mathbf w\mathbf x$)"),
    (G5, r"$\theta_{\rm LL}$", "theta",
     r"the real parameter of a one-parameter family of transformations or of a curve (an angle, a rapidity)"),
    (G5, r"$\mathcal V_{\rm LL}(\mathbf u)$", "V_LL",
     r"the vector action of a Lorentz element on $\mathbb R^{1,3}_{\rm LL}$, $\mathcal V_{\rm LL}(\mathbf u)\mathbf x=\mathbf u\mathbf x+\mathbf x\mathbf u^\dagger$, a real $4\times4$ matrix with entries $\mathcal V_{\rm LL}(\mathbf u)^p{}_{p'}$ in the coordinates $x^p_{\rm LL}$ (plain text `V_LL^p_p'`)"),
    # ------------------------------------------------------------------ 6
    (G6, r"$M_{\rm LL}(\mathbf b)$", "M_LL M_2(C)",
     r"the complex $2\times2$ matrix of a bi-quaternion, $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$, $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$, extended complex-linearly; $M_{\rm LL}(\mathbf b\mathbf b')=M_{\rm LL}(\mathbf b)M_{\rm LL}(\mathbf b')$, $\det M_{\rm LL}(\mathbf b)=N_{\rm LL}(\mathbf b)$ (plain text `M_2(C)`: all complex $2\times2$ matrices)"),
    (G6, r"$\psi'_{\rm LL}$", "psi'_LL",
     r"the Weyl isodoublet of a spinor charge $\mathbf b\mathbf e_4$: the $2\times2$ matrix $\psi'_{\rm LL}=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf b)^T$, whose first column (isospin up) and second column (isospin down) are Weyl spinors"),
    (G6, r"$\Psi_{\rm LL}$", "Psi_LL Psi_LL^T Psi_LL^dag",
     r"the Dirac isodoublet of the spinor charges: a field of 8 complex Grassmann components (Dirac $\otimes$ isospin), whose $\gamma^{\rm ch}_{\rm LL}=+1$ half is built from $\mathfrak s_{\rm LL}$ and whose other half from the complex-conjugate super-loop"),
    (G6, r"$\gamma^p_{\rm LL}$, $\gamma^{\rm ch}_{\rm LL}$, $C_{\rm LL}$, $\alpha^j_{\rm LL}$, $\beta_{\rm LL}$", "gamma_LL^p gamma_LL^p\' gamma_ch C_LL alpha_LL^j beta_LL",
     r"the $4\times4$ Dirac matrices of $\mathbb R^{1,3}_{\rm LL}$ in the chiral basis, $\gamma^0_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_0\\\sigma^{\rm LL}_0&0\end{pmatrix}$, $\gamma^j_{\rm LL}=\begin{pmatrix}0&\sigma^{\rm LL}_j\\-\sigma^{\rm LL}_j&0\end{pmatrix}$; the chirality matrix $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$; the charge-conjugation matrix $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$; $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$, $\beta_{\rm LL}=\gamma^0_{\rm LL}$ (on Dirac $\otimes$ isospin each acts as $\otimes\sigma^{\rm LL}_0$)"),
    (G6, r"$\mathsf K^\iota_{\rm LL}$, $\mathsf K'^\iota_{\rm LL}$, $\mathsf K^{\rm s}_{\rm LL}$", "Kch Kch_iota Kch_1 Kch_2 Kch_3 Kch'_iota Kch_singlet",
     r"the $8\times8$ pair matrices of the spin-zero channels: scalar $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^+$), pseudoscalar $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ ($0^-$), and the $16\times16$ singlet of two species $\mathsf K^{\rm s}_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\otimes\mathsf E_{\rm LL}$ (the last factor on the species index); the plain-text name Kch alone, and the letter $\mathsf K$ without scripts in Theorem SF1, stand for any $8\times8$ pair matrix, the pair of two spinor charges being $\Psi^T_{\rm LL}\mathsf K\Psi_{\rm LL}$"),
    (G6, r"$\Pi^\iota_{\rm LL}$", "Pi_iota",
     r"the pair operators $\Pi^\iota_{\rm LL}=\sum_{\vec q_{\rm LL}}\Psi_{\rm LL}(\vec q_{\rm LL})^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q_{\rm LL})$, which annihilate two spinor charges of opposite momenta"),
    (G6, r"$\vec q_{\rm LL}$, $q_{\rm LL}$, $q^j_{\rm LL}$", "q",
     r"the momentum of a particle in the space of $\mathbb R^{1,3}_{\rm LL}$, its length and its components"),
    (G6, r"$m_{\rm LL}$, $\mu_{\rm LL}$, $\Lambda_{\rm LL}$", "m mu Lambda",
     r"the mass of the spinor charges, the chemical potential, and the momentum cutoff ($q_{\rm LL}<\Lambda_{\rm LL}$; $\Lambda_{\rm LL}=1$ sets the units, except where it varies)"),
    (G6, r"$\varepsilon_{\rm LL}$, $E^\pm_{\rm LL}$, $\tilde E^\pm_{\rm LL}$", "eps E^+ E^- E^(+-) E_minus_polar E_plus_polar Etilde^+ Etilde^- Etilde^(+-) Etilde_minus Etilde_plus",
     r"the energy $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$, the quasi-particle energies of the scalar channel $E^\pm_{\rm LL}=[(\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2]^{1/2}$, a function of the gap (written $E^\pm_{\rm LL}(\Delta^+_{\rm LL})$ and $E^\pm_{\rm LL}(\Delta^-_{\rm LL})$ when $\Delta_{\rm LL}$ is replaced by one of the two gaps of a general state), and those of the pseudoscalar channel, $(\tilde E^\pm_{\rm LL})^2=\varepsilon_{\rm LL}^2+\mu_{\rm LL}^2+\Delta_{\rm LL}^2\pm2[\mu_{\rm LL}^2\varepsilon_{\rm LL}^2+m_{\rm LL}^2\Delta_{\rm LL}^2]^{1/2}$"),
    (G6, r"$G_{\rm LL}$, $G^{\rm crit}_{\rm LL}$", "G G_crit G_over_Gcrit_m0 G_crit_mu0_m0",
     r"the coupling of the attractive spin-zero contact interaction (dimension mass$^{-2}$) and its critical value at $\mu_{\rm LL}=0$, above which the vacuum condenses"),
    (G6, r"$\mathcal H^{\rm sf}_{\rm LL}$", "Hsf",
     r"the Hamiltonian of the superfluid, $\sum_{\vec q_{\rm LL}}\Psi^\dagger_{\rm LL}(\mathsf h_{\rm LL}-\mu_{\rm LL})\Psi_{\rm LL}-\tfrac14G_{\rm LL}\sum_\iota(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}$"),
    (G6, r"$\mathsf h_{\rm LL}(\vec q_{\rm LL})$", "h(q)",
     r"the $8\times8$ Hamiltonian matrix of a plane wave, $\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ (times $\sigma^{\rm LL}_0$ on the isospin index); eigenvalues $\pm\varepsilon_{\rm LL}$"),
    (G6, r"$\vec d_{\rm LL}$, $d^\iota_{\rm LL}$; $\zeta_{\rm LL}$", "dvec d_iota abs_dvec_at_min zeta",
     r"the order parameter, the isospin vector $\vec d_{\rm LL}=(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})\in\mathbb C^3$ with $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\langle\Pi^\iota_{\rm LL}\rangle$, and its non-unitarity $\zeta_{\rm LL}=\lvert\vec d_{\rm LL}\times\vec d^{\,*}_{\rm LL}\rvert/\lvert\vec d_{\rm LL}\rvert^2\in[0,1]$ (plain text `cross(dvec, dvec^*)`: the cross product)"),
    (G6, r"$\Delta_{\rm LL}$, $\Delta^\pm_{\rm LL}$", "Delta Delta_+ Delta_- Delta_(+-)",
     r"the gap: $\Delta_{\rm LL}=\lvert\vec d_{\rm LL}\rvert$ for a unitary state, and the two gaps $(\Delta^\pm_{\rm LL})^2=\lvert\vec d_{\rm LL}\rvert^2(1\pm\zeta_{\rm LL})$ of a general state"),
    (G6, r"$\mathsf P_{\rm LL}$; $\mathsf H_{\rm LL}(\vec q_{\rm LL})$; $\mathcal N_{\rm LL}(\vec q_{\rm LL})$", "Ppair H_BdG",
     r"the pairing block $\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$, the $16\times16$ Bogoliubov-de Gennes matrix $\mathsf H_{\rm LL}=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}&\mathsf P_{\rm LL}\\\mathsf P_{\rm LL}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})-\mu_{\rm LL})^T\end{pmatrix}$ and the Nambu spinor $\mathcal N_{\rm LL}=(\Psi_{\rm LL}(\vec q_{\rm LL}),\Psi_{\rm LL}^{*}(-\vec q_{\rm LL}))$ on which it acts"),
    (G6, r"$\mathsf D_{\rm LL}$", "",
     r"the isospin part of the pairing block of the scalar channel, the $2\times2$ matrix $\mathsf D_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf E_{\rm LL}\tau^{\rm LL}_\iota)^\dagger$, so that $\mathsf P_{\rm LL}=(C_{\rm LL}\gamma^{\rm ch}_{\rm LL})^\dagger\otimes\mathsf D_{\rm LL}$"),
    (G6, r"$\Omega_{\rm LL}$", "Omega Omega_min Omega_minus_Omega0 Omega_at_polar_gap",
     r"the grand potential per volume at zero temperature, measured from the normal state: $\Omega_{\rm LL}=\lvert\vec d_{\rm LL}\rvert^2/G_{\rm LL}-\tfrac12\int\!d^3q_{\rm LL}(2\pi)^{-3}\sum(\text{positive eigenvalues of }\mathsf H_{\rm LL})$ minus its value at $\vec d_{\rm LL}=0$"),
    (G6, r"$k^{\rm F}_{\rm LL}$, $\nu^{\rm F}_{\rm LL}$", "kF nuF",
     r"the Fermi momentum $(\mu_{\rm LL}^2-m_{\rm LL}^2)^{1/2}$ and the density of states of one species and one spin state at the Fermi surface, $\nu^{\rm F}_{\rm LL}=k^{\rm F}_{\rm LL}\mu_{\rm LL}/(2\pi^2)$"),
    (G6, r"$F_{\rm LL}$; $U(1)^{\rm F}_{\rm LL}$; $SU(2)_{\rm v}$", "F_LL U(1)_F_LL SU(2)_v",
     r"the generator of the fermion number $\Psi_{\rm LL}\to e^{i\theta_{\rm LL}}\Psi_{\rm LL}$ (it multiplies $\vec d_{\rm LL}$ by $2i$) and its group; the isospin group, which rotates the isospin index of $\Psi_{\rm LL}$ and $\vec d_{\rm LL}$ as a vector"),
    (G6, r"$\Psi^{(1)}_{\rm LL}$, $\Psi^{(2)}_{\rm LL}$", "",
     r"two species of the field $\Psi_{\rm LL}$, combined into one field of 16 components with a species index"),
    # ------------------------------------------------------------------ 7
    (G7, r"$[\ \cdot\ ]$ (one argument)", "[Psi_LL] [phi] [G] [g_Y] [lam4]",
     r"the mass dimension of a field or a coupling, in units in which $\hbar$ and the speed of light are 1 (a bracket with two or three arguments is a commutator or an associator)"),
    (G7, r"$\phi_{\rm LL}$, $g^{\rm Y}_{\rm LL}$, $\lambda^{(1)}_{\rm LL}$, $\lambda^{(2)}_{\rm LL}$", "phi g_Y lam4 lam4_1 lam4_2",
     r"the Yukawa completion: a complex isospin-triplet pair field $\phi_{\rm LL}=(\phi^1_{\rm LL},\phi^2_{\rm LL},\phi^3_{\rm LL})$ coupled to $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$ with the Yukawa coupling $g^{\rm Y}_{\rm LL}$, and the couplings $\lambda^{(1)}_{\rm LL}$, $\lambda^{(2)}_{\rm LL}$ of the quartic terms $(\phi^\dagger_{\rm LL}\phi_{\rm LL})^2$ and $\lvert\phi^T_{\rm LL}\phi_{\rm LL}\rvert^2$ (plain text `lam4`: either of them)"),
    (G7, r"$\deg_{\rm LL}$", "deg",
     r"the superficial degree of divergence of a one-particle-irreducible graph: the power of the cutoff with which it grows when all its loop momenta grow together"),
    (G7, r"$n^{\rm loop}_{\rm LL}$, $n^{\rm int,F}_{\rm LL}$, $n^{\rm int,B}_{\rm LL}$, $n^{\rm ext,F}_{\rm LL}$, $n^{\rm ext,B}_{\rm LL}$, $n^{\rm v}_{\rm LL}$, $n^{\rm v,Y}_{\rm LL}$, $n^{\rm v,4}_{\rm LL}$", "nloop nint_F nint_B next_F next_B nvert nvert_Y nvert_4",
     r"the numbers of loops, of internal fermion and boson lines, of external fermion and boson lines, of vertices, of Yukawa vertices and of quartic vertices of a graph"),
    (G7, r"$I^{\rm bub}_{\rm LL}$; $k^{\rm E}_{\rm LL}$, $p^{\rm E}_{\rm LL}$", "Ibub Ibub_0 Ibub_pE Ibub_pE_minus_Ibub_0 kE pE",
     r"the one-loop bubble of the Yukawa completion, $I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})=\int_{k^{\rm E}_{\rm LL}<\Lambda_{\rm LL}}\frac{d^4k^{\rm E}_{\rm LL}}{(2\pi)^4}\frac{k^{\rm E}_{\rm LL}\cdot(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})+m_{\rm LL}^2}{[(k^{\rm E}_{\rm LL})^2+m_{\rm LL}^2][(k^{\rm E}_{\rm LL}+p^{\rm E}_{\rm LL})^2+m_{\rm LL}^2]}$, with the Euclidean loop and external four-momenta $k^{\rm E}_{\rm LL}$, $p^{\rm E}_{\rm LL}$ (and their lengths)"),
    # ------------------------------------------------------------------ 8
    (G8, r"$SL(2,\mathbb C)$, $Spin(3,1)^+$, $SO(4,\mathbb C)$, $\mathrm G_2$, $\mathrm G_2(\mathbb C)$, $SO(1,7)$, $Spin(7)$, $Spin(7,\mathbb C)$, $SL(2,\mathbb R)$, $Spin(2,1)$, $SU(2)$, $U(1)$, $\mathbb Z_2$",
     "SL(2,C) Spin(3,1)^+ SO(4,C) G2 G2(C) SO(1,7) SO(3,1) Spin(7) Spin(7,C) SL(2,R) Spin(2,1) SU(2) U(1) Z2",
     r"groups: the complex $2\times2$ matrices of determinant 1 (the unit bi-quaternions), equal to the connected double cover of the Lorentz group of $3+1$ dimensions; the complex orthogonal group of four dimensions; the compact exceptional group $\mathrm G_2$ (automorphisms of $\mathbb O$) and its complex form (automorphisms of $\mathbb B_{\rm LL}$); the Lorentz group of $7+1$ dimensions; the spin groups of seven dimensions (compact and complex); the real $2\times2$ matrices of determinant 1, the double cover of the Lorentz group of $2+1$ dimensions; the special unitary group of two dimensions; the phases; the group of two elements"),
    (G8, r"$\mathfrak{so}(3,1)$, $\mathfrak{so}(1,7)$, $\mathfrak{so}(4,1)$, $\mathfrak{so}(8)$, $\mathfrak{so}(4,4)$, $\mathfrak{so}(8,\mathbb C)$, $\mathfrak{so}(3)$, $\mathfrak{so}(2,1)$, $\mathfrak{sl}(2,\mathbb R)$, $\mathfrak{sl}(2)$, $\mathfrak{su}(2)$, $\mathfrak{spin}(7)$, $\mathfrak g_2$, $\mathfrak g_2(\mathbb C)$",
     "so(3,1) so(1,7) so(4,1) so(8) so(4,4) so(8,C) so(3) so(2,1) sl(2,R) sl(2) su(2) spin(7) g2 g2(C)",
     r"Lie algebras: of the Lorentz groups of $3+1$ and $7+1$ dimensions; of the de Sitter group (signature of the Killing form: 4 positive, 6 negative); of the linear maps that preserve a definite real, a $(4,4)$ and a complex quadratic form in eight dimensions; of the rotations of three-dimensional space; of the Lorentz group of $2+1$ dimensions, equal to $\mathfrak{sl}(2,\mathbb R)$; $\mathfrak{sl}(2)$ the simple Lie algebra of dimension 3 over any field; $\mathfrak{su}(2)$ isospin; $\mathfrak{spin}(7)$ of dimension 21; the exceptional Lie algebra $\mathfrak g_2$ (compact, dimension 14) and its complex form (real dimension 28)"),
    (G8, r"$\mathrm{tr}$, $\det$, $\mathrm{diag}$, $\mathrm{Re}$, $\mathrm{Im}$", "tr det diag Re Im exp ln sqrt sum int span dim",
     r"trace, determinant, the diagonal matrix with the listed entries, real part, imaginary part (also: the imaginary part of an algebra, the span of its imaginary units); plain text `exp`, `ln`, `sqrt`, `sum`, `int`, `span`, `dim`: the exponential, natural logarithm, square root, a sum, an integral, the set of real linear combinations, a dimension"),
    (G8, r"${}^T$; $\otimes$, $\oplus$; $\circ$; $\langle\ \rangle$; h.c.", "(x) ^T o",
     r"transpose; tensor product and direct sum (plain text `(x)` for $\otimes$); composition of maps; expectation value in the ground state; the Hermitian conjugate of the preceding terms"),
    (G8, r"A-1, ..., R-6; G-1, ..., G-12; LL-S1, ..., LL-H1; Theorems R, L, M, SF1, ..., SF4", "",
     r"the identifiers of the machine checks (letters: A associativity, L Lorentz super-loop, Q operators and eigenvalues, C the inversion element, M Malcev classification, G Groebner bases, N manifolds, S superfluid, R renormalizability, LL-S symbols, LL-H self-containedness) and the labels of the theorems proved in the manuscript"),
    (G8, r"H-grp, H-op, H-Lie, H-gr, H-dyn", "",
     r"the labels of the hypotheses of the no-go theorems: the symmetries form a group (H-grp), act as linear operators on the space of states (H-op), have generators that form a Lie algebra (H-Lie) or a Lie superalgebra (H-gr), and the dynamical hypotheses on the scattering matrix (H-dyn)"),
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
        # the plain-text names: those of the master file, unless Revision_SuperLoop uses others
        # (a fourth field; the master names of this entry occur only in other folders)
        names = entry[3] if len(entry) > 3 else r["names_in_files"]
        out.append({"group": group, "symbol": sym, "names_in_files": names, "meaning": meaning,
                    "origin": "taken from the MASTER registry of the %s" % ("main project" if src == "main" else "revision on the deflating spacetimes")})
    for group, sym, names, meaning in NEW:
        out.append({"group": group, "symbol": sym, "names_in_files": names, "meaning": meaning, "origin": "new (Revision_SuperLoop)"})
    seen = set()
    for r in out:
        if r["symbol"] in seen:
            raise SystemExit("symbol registered twice: %r" % r["symbol"])
        seen.add(r["symbol"])
    out.sort(key=lambda r: r["group"])
    os.makedirs(os.path.join(ROOT, "provenance", "generated"), exist_ok=True)
    with open(os.path.join(ROOT, "provenance", "symbols_ll.csv"), "w", encoding="utf-8", newline="") as f:
        f.write("# PROJECT spacetimes, Revision_SuperLoop: registry of the symbols (written by code/scripts/write_symbol_registry.py)\n")
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
        # a plain-text name inside a meaning is written as Markdown code (`R^8`); in LaTeX it becomes \texttt
        meaning_tex = re.sub(r"`([^`]*)`", lambda m: "\\texttt{%s}" % tex_name(m.group(1)), r["meaning"])
        tex_lines.append("%s & %s & %s\\\\" % (r["symbol"], meaning_tex, names))
    frag = []
    frag.append("<!-- BEGIN symbols_ll_table -->\n%s\n<!-- END symbols_ll_table -->" % "\n".join(md_lines))
    frag.append("<!-- BEGIN symbols_ll_texrows -->\n%s\n<!-- END symbols_ll_texrows -->" % "\n".join(tex_lines))
    frag.append("<!-- BEGIN symbols_ll_count -->\n%d\n<!-- END symbols_ll_count -->" % len(out))
    frag.append("<!-- BEGIN symbols_ll_count_taken -->\n%d\n<!-- END symbols_ll_count_taken -->" % len(TAKEN))
    frag.append("<!-- BEGIN symbols_ll_count_new -->\n%d\n<!-- END symbols_ll_count_new -->" % len(NEW))
    with open(os.path.join(ROOT, "provenance", "generated", "symbols_ll.md"), "w", encoding="utf-8") as f:
        f.write("\n\n".join(frag) + "\n")
    print("wrote provenance/symbols_ll.csv and provenance/generated/symbols_ll.md: %d entries (%d taken, %d new)" % (len(out), len(TAKEN), len(NEW)))


def tex_name(n):
    """A plain-text name for \\texttt{...} in LaTeX."""
    out = []
    for ch in n:
        out.append({"_": "\\_", "^": "\\^{}", "&": "\\&", "%": "\\%", "#": "\\#", "<": "$<$", ">": "$>$", "{": "\\{", "}": "\\}", "~": "\\~{}"}.get(ch, ch))
    return "".join(out)


if __name__ == "__main__":
    main()
