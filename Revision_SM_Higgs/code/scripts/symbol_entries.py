"""
symbol_entries.py (Revision_SM_Higgs) -- the registry of the symbols of the solution, as data.

Every entry is a tuple
    (group, symbols, plain_names, meaning, origin)
  group        the number and title of the group of the table;
  symbols      the symbol or symbols as written in the documents (LaTeX, each between $ signs);
  plain_names  the names under which programs, figures and data files write the symbol (blank-separated; may be empty);
  meaning      the definition, complete in itself (every symbol in it is itself an entry of this registry);
  origin       one of
                 "LL"     the same object as the entry with this symbol text in the registry of the folder
                                        Revision_SuperLoop (provenance/symbols_ll.csv), written with the same symbol;
                 "LL-renamed:<symbol>"  an object of Revision_SuperLoop that the solution writes with another symbol;
                 "author"               a name that the author fixed in his task of 20 November 2026;
                 "author-KS"            a symbol formed after the author's Kohn-Sham ansatz of the project;
                 "SM"                   a symbol of the author's conventions (the Standard Model), used only to state
                                        the conventions and to compare with them;
                 "new"                  introduced by this solution.
The convention of the solution: an object that the solution takes unchanged from the super-loop work keeps its symbol
with the suffix LL; every object that the solution introduces or defines anew carries the suffix SF (for the
superfluid solution), as a subscript, or as a superscript when the subscript position holds an index.  The exceptions
are indices, operations and number sets, the author's own names (M31, GEE, the effective Lagrangians and SFBSv), and
the symbols of the Standard Model in the author's conventions.

This file is read by code/scripts/write_symbol_registry.py (which writes provenance/symbols_sf.csv and the generated
tables of the MASTER provenance) and by code/scripts/check_symbols.py (the machine checks).
"""

G1 = "1 Indices, conventions, number sets, operations and groups"
G2 = "2 The bi-quaternions, the bi-octonions and the super-loop"
G3 = "3 The manifold M31"
G4 = "4 The spinor field and its matrices"
G5 = "5 The gauge group GEE, the gauge fields and the covariant derivatives"
G6 = "6 The Lagrangian, the field equations and their numerical form"
G7 = "7 The analytic solutions"
G8 = "8 The pair channels and the gauge bosons that mediate the pairing"
G9 = "9 The three methods and the recursion of the potential"
G10 = "10 The effective Lagrangians, the breaking of the symmetry and SFBSv"
G11 = "11 The masses of the gauge bosons and the surviving symmetry"
G12 = "12 The dynamics"
G13 = "13 The Standard Model of the author's conventions"
G14 = "14 The revised hypotheses: the singlet field and the Higgs doublet of the superfluid"

ENTRIES = [
    # ------------------------------------------------------------------ group 1
    (G1, "$i$", "", "the imaginary unit, $i^2=-1$; it commutes and associates with every bi-quaternion and bi-octonion",
     "LL"),
    (G1, "$\\mathbb R$, $\\mathbb C$", "R C", "the real and the complex numbers", "LL"),
    (G1, "$p$, $p'$", "p p'", "an index over the four coordinates of M31, values 0 (the time), 1, 2, 3; also an index over the four "
     "units $\\mathbf e_0,\\dots,\\mathbf e_3$ of the quaternions", "LL"),
    (G1, "$j$, $j'$, $j''$", "j", "an index over the three space coordinates and over the three imaginary quaternion units, "
     "values 1, 2, 3 (the letter $i$ is never an index)", "LL-as:$i,j$"),
    (G1, "$\\iota$, $\\iota'$, $\\iota''$", "iota", "the isospin index, values 1, 2, 3: it numbers the generators of "
     "$SU(2)_{\\rm v}$, the Pauli matrices $\\tau^{\\rm LL}_\\iota$, the pair matrices, the pair operators and the components "
     "of isospin vectors; an internal direction, not a direction in space", "LL"),
    (G1, "$c$, $c'$", "c", "an index over the eight components of the spinor field, values 1 to 8 (the component number "
     "is $2\\times(\\text{Dirac index}-1)+(\\text{isospin index})$, isospin index 1 for up and 2 for down)", "new"),
    (G1, "$\\alpha$, $\\alpha'$", "", "an index over the two components of a two-component (Weyl) spinor, values 1, 2",
     "new"),
    (G1, "$f$, $f'$", "", "an index over the two members of an $SU(2)_{\\rm v}$ doublet of Weyl spinors, values 1, 2",
     "new"),
    (G1, "$\\epsilon^{\\rm LL}_{jj'j''}$", "eps3", "the Levi-Civita symbol: $+1$ for an even permutation of $(1,2,3)$, "
     "$-1$ for an odd one, $0$ otherwise; also with isospin indices, $\\epsilon^{\\rm LL}_{\\iota\\iota'\\iota''}$",
     "LL"),
    (G1, "$\\sigma^{\\rm LL}_0$, $\\sigma^{\\rm LL}_j$", "sigma_0 sigma_1 sigma_2 sigma_3", "the $2\\times2$ unit matrix and "
     "the Pauli matrices $\\sigma^{\\rm LL}_1=\\begin{pmatrix}0&1\\\\1&0\\end{pmatrix}$, "
     "$\\sigma^{\\rm LL}_2=\\begin{pmatrix}0&-i\\\\i&0\\end{pmatrix}$, $\\sigma^{\\rm LL}_3=\\begin{pmatrix}1&0\\\\0&-1\\end{pmatrix}$, "
     "acting on the two components of a Weyl spinor", "LL"),
    (G1, "$\\tau^{\\rm LL}_\\iota$", "tau_1 tau_2 tau_3", "the same three Pauli matrices acting on the isospin index",
     "LL"),
    (G1, "$\\mathsf E_{\\rm LL}$", "eps2", "the real antisymmetric $2\\times2$ matrix $i\\sigma^{\\rm LL}_2=\\begin{pmatrix}0&1\\\\-1&0\\end{pmatrix}$, "
     "$\\mathsf E_{\\rm LL}^{-1}=\\mathsf E_{\\rm LL}^T=-\\mathsf E_{\\rm LL}$", "LL"),
    (G1, "$\\mathsf 1_2$, $\\mathsf 1_4$, $\\mathsf 1_8$, $\\mathsf 1_{16}$", "", "the unit matrices of 2, 4, 8 and 16 rows",
     "LL"),
    (G1, "$\\eta^{\\rm LL}_{pp'}$", "eta", "the Minkowski metric of M31: $\\eta^{\\rm LL}_{00}=1$, "
     "$\\eta^{\\rm LL}_{11}=\\eta^{\\rm LL}_{22}=\\eta^{\\rm LL}_{33}=-1$, $0$ for $p\\neq p'$; it raises and lowers the indices "
     "$p$ (an upper index is written $p$ up, as in $\\gamma^p_{\\rm LL}$, a lower one down)", "LL"),
    (G1, "$\\mathrm{tr}$, $\\det$, $\\mathrm{diag}$, $\\mathrm{Re}$, $\\mathrm{Im}$", "tr det diag Re Im",
     "trace, determinant, the diagonal matrix with the listed entries, real part, imaginary part",
     "LL"),
    (G1, "${}^T$; $\\otimes$, $\\oplus$; $\\circ$; $\\langle\\ \\rangle$; h.c.", "(x) ^T",
     "transpose; tensor product and direct sum; composition; expectation value in the state named in the text "
     "(the ground state unless said otherwise); the Hermitian conjugate of the preceding terms",
     "LL"),
    (G1, "${}^\\dagger$, ${}^*$", "dag", "the Hermitian conjugate (of a matrix: the complex-conjugate transpose; of an "
     "operator: its adjoint) and the complex conjugate (of a column of operators: the column of their adjoints)", "new"),
    (G1, "$:\\ :$", "", "normal ordering with respect to the normal vacuum: the product of field operators between the "
     "colons is written so that its expectation value in the normal vacuum is zero, and every contraction is measured "
     "from its value in the normal vacuum", "new"),
    (G1, "$\\square$", "", "the end of a proof", "new"),
    (G1, "$SL(2,\\mathbb C)$, $SU(2)$, $U(1)$, $SO(3)$, $O(2)$", "SL(2,C) SU(2) U(1) SO(3) O(2)",
     "groups: the complex $2\\times2$ matrices of determinant 1 (the unit bi-quaternions, the double cover of the Lorentz "
     "group); the unitary $2\\times2$ matrices of determinant 1; the phases; the rotations of three dimensions; the "
     "rotations and reflections of a plane", "LL"),
    (G1, "$\\mathfrak{so}(3,1)$, $\\mathfrak{su}(2)$", "so(3,1) su(2)", "the Lie algebras of the Lorentz group and of "
     "$SU(2)$", "LL"),
    (G1, "$SU(2)_{\\rm v}$, $U(1)^{\\rm F}_{\\rm LL}$; $F_{\\rm LL}$", "SU(2)_v U(1)_F_LL F_LL",
     "the isospin group, which acts on the isospin index of the spinor field (and on isospin vectors by rotations), and "
     "the group of the phases of the fermion number, $\\Psi_{\\rm LL}\\mapsto e^{i\\theta^{\\rm F}_{\\rm LL}}\\Psi_{\\rm LL}$; "
     "$F_{\\rm LL}$ is its generator, the fermion number (1 on $\\Psi_{\\rm LL}$, 2 on a pair of two spinor charges); "
     "the author writes $U(1)^{\\rm F}_{\\rm LL}$ as U(1)FLL",
     "LL"),
    (G1, "$U(2)$", "U(2)", "the group of the unitary $2\\times2$ matrices", "new"),
    (G1, "$S$", "S", "the total spin of a state of two spinor charges; $S=0$, the spin singlet (the author's notation)",
     "author"),
    (G1, "GEE", "GEE", "the gauge group of the problem, $SL(2,\\mathbb C)\\times SU(2)_{\\rm v}\\times U(1)^{\\rm F}_{\\rm LL}$ "
     "(the author's name)", "author"),
    (G1, "$U(1)^{\\rm Q}_{\\rm LL}$", "U(1)^Q", "the group of the charge $Q_{\\rm LL}$ (a charge of $SU(2)_{\\rm v}$, not the "
     "electric charge): the phases $\\exp(i\\theta Q_{\\rm LL})$, $Q_{\\rm LL}=T^3_{\\rm LL}$, the rotations about the third "
     "isospin axis", "LL"),
    # ------------------------------------------------------------------ group 2
    (G2, "$\\mathbb H$, $\\mathbb O$", "H O", "the quaternions and the octonions: the real combinations of the units "
     "$\\mathbf e_0,\\dots,\\mathbf e_3$ and of $\\mathbf e_0,\\dots,\\mathbf e_7$", "LL"),
    (G2, "$\\mathbb B_{\\rm LL}$", "biocts", "the bi-octonions: the complex combinations of the eight units "
     "$\\mathbf e_0,\\dots,\\mathbf e_7$, with the product of the units extended bilinearly (real dimension 16)",
     "LL"),
    (G2, "$\\mathbf e_0,\\dots,\\mathbf e_7$; $\\mathbf e_p$, $\\mathbf e_j$", "e0 e1 e2 e3 e4 e5 e6 e7",
     "the units: $\\mathbf e_0=1$; $\\mathbf e_1,\\mathbf e_2,\\mathbf e_3$ the quaternion units, $\\mathbf e_j^2=-1$, "
     "$\\mathbf e_1\\mathbf e_2=-\\mathbf e_2\\mathbf e_1=\\mathbf e_3$ and cyclically; $\\mathbf e_4$ the unit of the "
     "Cayley-Dickson doubling, $\\mathbf e_4^2=-1$; $\\mathbf e_{4+j}=\\mathbf e_j\\mathbf e_4$",
     "LL"),
    (G2, "$\\xi^{\\rm LL}_p(\\mathbf x)$", "xi_p", "the complex coefficient of the unit $\\mathbf e_p$ in the bi-quaternion "
     "or bi-octonion $\\mathbf x$", "LL"),
    (G2, "$\\mathbf x$, $\\mathbf y$, $\\mathbf z$, $\\mathbf w$", "x y z w", "bi-octonions (or bi-quaternions, as said "
     "in the text); a Hermitian bi-quaternion $\\mathbf x$ is also a point of M31", "LL"),
    (G2, "$\\mathbf a$, $\\mathbf b$, $\\mathbf c$, $\\mathbf d$", "aH bS cH dS", "bi-quaternions: complex combinations "
     "of $\\mathbf e_0,\\dots,\\mathbf e_3$; every bi-octonion is $\\mathbf a+\\mathbf b\\,\\mathbf e_4$ (Cayley-Dickson form), "
     "and $\\mathbf b\\,\\mathbf e_4$ is a spinor charge", "LL"),
    (G2, "$\\bar{\\mathbf x}$, $\\mathbf x^*$, $\\mathbf x^\\dagger$", "xbar x^* x^dag", "the three conjugations: "
     "$\\bar{\\mathbf x}$ changes the signs of the coefficients of $\\mathbf e_1,\\dots,\\mathbf e_7$; $\\mathbf x^*$ "
     "conjugates every complex coefficient; $\\mathbf x^\\dagger=(\\bar{\\mathbf x})^*$",
     "LL"),
    (G2, "$N_{\\rm LL}(\\mathbf x)$, $N_{\\rm LL}(\\mathbf x,\\mathbf y)$", "N_LL", "the complex norm "
     "$N_{\\rm LL}(\\mathbf x)=\\sum_p\\xi^{\\rm LL}_p(\\mathbf x)^2$, with $\\mathbf x\\bar{\\mathbf x}=N_{\\rm LL}(\\mathbf x)\\,\\mathbf e_0$, "
     "and its polarization $N_{\\rm LL}(\\mathbf x,\\mathbf y)=\\sum_p\\xi^{\\rm LL}_p(\\mathbf x)\\xi^{\\rm LL}_p(\\mathbf y)$",
     "LL"),
    (G2, "$\\mathbb L_{\\rm LL}$", "L_LL", "the Lorentz super-loop: the unit bi-octonions, $N_{\\rm LL}(\\mathbf w)=1$, with "
     "the product of $\\mathbb B_{\\rm LL}$; a Moufang loop (not a group), a manifold of real dimension 14",
     "LL"),
    (G2, "$\\mathbf g$", "gL", "a unit bi-quaternion, $N_{\\rm LL}(\\mathbf g)=1$: an element of $SL(2,\\mathbb C)$",
     "LL"),
    (G2, "$\\mathbf v$", "vI", "a unit real quaternion, which rotates the isospin of the spinor charges from the left",
     "LL"),
    (G2, "$J^j_{\\rm LL}$, $K^j_{\\rm LL}$", "J_LL^j K_LL^j", "the rotation and boost generators of the Lorentz algebra of "
     "the super-loop, $J^j_{\\rm LL}=\\mathbf e_j/2$, $K^j_{\\rm LL}=i\\,\\mathbf e_j/2$", "LL"),
    (G2, "$\\mathbf u$", "uL", "a Lorentz generator: an imaginary bi-quaternion (a complex combination of "
     "$\\mathbf e_1,\\mathbf e_2,\\mathbf e_3$)", "LL"),
    (G2, "$M_{\\rm LL}(\\mathbf b)$", "M_LL", "the complex $2\\times2$ matrix of a bi-quaternion, "
     "$M_{\\rm LL}(\\mathbf e_0)=\\sigma^{\\rm LL}_0$, $M_{\\rm LL}(\\mathbf e_j)=-i\\sigma^{\\rm LL}_j$, extended complex-linearly; "
     "$M_{\\rm LL}(\\mathbf b\\mathbf b')=M_{\\rm LL}(\\mathbf b)M_{\\rm LL}(\\mathbf b')$ and $\\det M_{\\rm LL}(\\mathbf b)=N_{\\rm LL}(\\mathbf b)$",
     "LL"),
    (G2, "$R^{\\rm LL}_{\\mathbf x}$", "R_x", "the right multiplication $\\mathbf y\\mapsto\\mathbf y\\mathbf x$ by a bi-octonion",
     "LL"),
    (G2, "$\\psi'_{\\rm LL}$", "psi'_LL", "the Weyl isodoublet of a spinor charge $\\mathbf b\\mathbf e_4$: the $2\\times2$ "
     "matrix $\\psi'_{\\rm LL}(\\mathbf b)=\\mathsf E_{\\rm LL}^{-1}M_{\\rm LL}(\\mathbf b)^T$, whose rows are the two components "
     "of a Weyl spinor and whose columns are isospin up and down", "LL"),
    # ------------------------------------------------------------------ group 3
    (G3, "M31, $\\mathbb R^{1,3}_{\\rm LL}$", "M31 R^(1,3)_LL", "the $3+1$ manifold of the Lorentz super-loop: the "
     "Hermitian bi-quaternions $\\mathbf x=x^0_{\\rm LL}\\mathbf e_0+i\\sum_jx^j_{\\rm LL}\\mathbf e_j$ with four real "
     "coordinates, whose norm $N_{\\rm LL}(\\mathbf x)=(x^0_{\\rm LL})^2-\\sum_j(x^j_{\\rm LL})^2$ is the Minkowski interval "
     "(M31 is the author's name)", "LL"),
    (G3, "$x^p_{\\rm LL}$; $x_{\\rm LL}$, $\\vec x_{\\rm LL}$; $\\partial_p$", "x_LL^p x_LL xvec d_p",
     "the coordinates of a point of M31 ($x^0_{\\rm LL}$ the time), the point, its space part, and the partial derivative "
     "with respect to $x^p_{\\rm LL}$", "LL"),
    (G3, "$\\mathbf f_p$", "f_p", "a frame of M31: four Hermitian bi-quaternions with "
     "$N_{\\rm LL}(\\mathbf f_p,\\mathbf f_{p'})=\\eta^{\\rm LL}_{pp'}$; the canonical frame is $\\mathbf f_0=\\mathbf e_0$, "
     "$\\mathbf f_j=i\\mathbf e_j$, with $M_{\\rm LL}(\\mathbf f_p)=\\sigma^{\\rm LL}_p$", "LL"),
    (G3, "$\\gamma^p_{\\rm LL}[\\mathbf f]$", "gamma_LL^p[f]", "the Dirac matrices of a frame, the $4\\times4$ matrices of "
     "$2\\times2$ blocks $\\begin{pmatrix}0&M_{\\rm LL}(\\mathbf f_p)\\\\M_{\\rm LL}(\\bar{\\mathbf f}_p)&0\\end{pmatrix}$",
     "LL"),
    (G3, "$\\tilde{\\mathbf g}$", "gLt", "for a unit bi-quaternion: $\\tilde{\\mathbf g}=(\\mathbf g^\\dagger)^{-1}=\\mathbf g^*$, "
     "$M_{\\rm LL}(\\tilde{\\mathbf g})=(M_{\\rm LL}(\\mathbf g)^\\dagger)^{-1}$", "LL"),
    (G3, "$\\mathsf L_{\\rm LL}(\\mathbf g)$", "Lmat", "the real $4\\times4$ matrix of the Lorentz transformation "
     "$\\mathbf x\\mapsto\\mathbf g\\mathbf x\\mathbf g^\\dagger$ of M31: its entry in row $p$ and column $p'$ is "
     "$\\tfrac12\\mathrm{Re}\\,\\mathrm{tr}(\\sigma^{\\rm LL}_pM\\sigma^{\\rm LL}_{p'}M^\\dagger)$ with "
     "$M=M_{\\rm LL}(\\mathbf g)$", "LL"),
    # ------------------------------------------------------------------ group 4
    (G4, "$\\Psi_{\\rm LL}$", "Psi_LL", "the spinor field, the doublet field: an isospin doublet of four-component Dirac "
     "fields, eight complex Grassmann (anticommuting) operator components $\\Psi_{{\\rm LL},c}$, made of two spinor charges of the "
     "super-loop. Under the hypotheses of this folder $SU(2)_{\\rm v}$ is $SU(2)_L$ and the field has the hypercharge "
     "$Y^{\\rm SM}=1/4$ and the electric charges $3/4$ (isospin up) and $-1/4$ (isospin down); under the earlier hypotheses, which "
     "the inherited documents still state, it was the DiracLike spinor with the electric charge 0",
     "LL"),
    (G4, "$\\Psi_{{\\rm LL},1},\\dots,\\Psi_{{\\rm LL},8}$; $\\Psi^\\pm_{\\rm LL}$; $\\bar\\Psi_{\\rm LL}$",
     "Psi_LL^+ Psi_LL^-", "the eight components of $\\Psi_{\\rm LL}$ (components 1, 2: first Dirac component, isospin up "
     "and down; 3, 4 the second; 5, 6 the third; 7, 8 the fourth); the chirality blocks $\\Psi^-_{\\rm LL}$ (components 1 to "
     "4) and $\\Psi^+_{\\rm LL}$ (5 to 8); the Dirac adjoint $\\bar\\Psi_{\\rm LL}=\\Psi^\\dagger_{\\rm LL}(\\gamma^0_{\\rm LL}\\otimes\\sigma^{\\rm LL}_0)$",
     "LL"),
    (G4, "$\\psi_{\\rm SF}$, $\\psi^\\pm_{\\rm SF}$", "psi1 psi1_pm", "a single DiracLike field with four complex "
     "Grassmann components (no isospin index), and its chirality blocks $\\psi^-_{\\rm SF}$ (first two components, "
     "chirality $-1$) and $\\psi^+_{\\rm SF}$ (last two, chirality $+1$); used in the theorem that rules out a "
     "GEE-invariant mass term for such a field", "new"),
    (G4, "$\\xi_{{\\rm SF},f}$, $\\xi_{{\\rm SF},f\\alpha}$", "xi_W", "the two left-handed Weyl spinors of a "
     "single DiracLike field, $\\xi_{{\\rm SF},1}=\\psi^-_{\\rm SF}$ and $\\xi_{{\\rm SF},2}=\\mathsf E_{\\rm LL}(\\psi^+_{\\rm SF})^*$, "
     "and their components", "new"),
    (G4, "$I^{\\rm W}_{{\\rm SF},ff'}$", "I_W", "the Lorentz-invariant bilinears "
     "$\\xi_{{\\rm SF},f}^T\\mathsf E_{\\rm LL}\\xi_{{\\rm SF},f'}$ of two left-handed Weyl spinors", "new"),
    (G4, "$\\mathbf b^+_{\\rm LL}$, $\\mathbf b^-_{\\rm LL}$", "bSp bSm", "the two spinor charges "
     "$\\mathbf b^\\pm_{\\rm LL}\\mathbf e_4$ of which $\\Psi_{\\rm LL}$ is made: $\\Psi^-_{\\rm LL}=\\psi'_{\\rm LL}(\\mathbf b^-_{\\rm LL})$, "
     "$\\Psi^+_{\\rm LL}=\\psi'_{\\rm LL}(\\mathbf b^+_{\\rm LL})$, the rows of the matrices read one after the other",
     "LL"),
    (G4, "$\\gamma^p_{\\rm LL}$, $\\gamma^{\\rm ch}_{\\rm LL}$, $C_{\\rm LL}$, $\\alpha^j_{\\rm LL}$, $\\beta_{\\rm LL}$",
     "gamma_LL^p gamma_ch C_LL alpha_LL^j beta_LL", "the Dirac matrices of the canonical frame in the chiral basis, "
     "$\\gamma^0_{\\rm LL}=\\begin{pmatrix}0&\\sigma^{\\rm LL}_0\\\\\\sigma^{\\rm LL}_0&0\\end{pmatrix}$, "
     "$\\gamma^j_{\\rm LL}=\\begin{pmatrix}0&\\sigma^{\\rm LL}_j\\\\-\\sigma^{\\rm LL}_j&0\\end{pmatrix}$; the chirality "
     "matrix $\\gamma^{\\rm ch}_{\\rm LL}=i\\gamma^0_{\\rm LL}\\gamma^1_{\\rm LL}\\gamma^2_{\\rm LL}\\gamma^3_{\\rm LL}=\\mathrm{diag}(-1,-1,1,1)$; "
     "the charge-conjugation matrix $C_{\\rm LL}=i\\gamma^2_{\\rm LL}\\gamma^0_{\\rm LL}$; "
     "$\\alpha^j_{\\rm LL}=\\gamma^0_{\\rm LL}\\gamma^j_{\\rm LL}$, $\\beta_{\\rm LL}=\\gamma^0_{\\rm LL}$ (on the eight "
     "components each acts as its tensor product with $\\sigma^{\\rm LL}_0$)",
     "LL"),
    (G4, "$\\mathsf S_{\\rm LL}(\\mathbf g)$", "Sspin", "the spinor matrix of a unit bi-quaternion, "
     "$\\mathrm{diag}(M_{\\rm LL}(\\tilde{\\mathbf g}),M_{\\rm LL}(\\mathbf g))$ (blocks of $2\\times2$)",
     "LL"),
    (G4, "$\\mathsf s_{\\rm LL}(\\mathbf u)$", "sgen", "the action of a Lorentz generator on the Dirac index, "
     "$\\mathrm{diag}(-M_{\\rm LL}(\\mathbf u)^\\dagger,M_{\\rm LL}(\\mathbf u))$", "LL"),
    (G4, "$\\mathsf K^\\iota_{\\rm LL}$, $\\mathsf K'^\\iota_{\\rm LL}$", "Kch_iota Kch'_iota", "the $8\\times8$ pair "
     "matrices of the spin-zero isotriplet channels: scalar $\\mathsf K^\\iota_{\\rm LL}=C_{\\rm LL}\\gamma^{\\rm ch}_{\\rm LL}\\otimes\\mathsf E_{\\rm LL}\\tau^{\\rm LL}_\\iota$ "
     "and pseudoscalar $\\mathsf K'^\\iota_{\\rm LL}=C_{\\rm LL}\\otimes\\mathsf E_{\\rm LL}\\tau^{\\rm LL}_\\iota$; antisymmetric "
     "and unitary", "LL"),
    (G4, "$\\Pi^\\iota_{\\rm LL}(x_{\\rm LL})$", "Pi_iota(x)", "the local pair operator "
     "$\\Psi_{\\rm LL}(x_{\\rm LL})^T\\mathsf K^\\iota_{\\rm LL}\\Psi_{\\rm LL}(x_{\\rm LL})$, which annihilates two spinor "
     "charges at one point", "LL"),
    (G4, "$\\Pi^\\iota_{\\rm LL}$", "Pi_iota", "the pair operators of the momenta: the sum over $\\vec q_{\\rm LL}$ of "
     "$\\Psi_{\\rm LL}(\\vec q_{\\rm LL})^T\\mathsf K^\\iota_{\\rm LL}\\Psi_{\\rm LL}(-\\vec q_{\\rm LL})$; they annihilate "
     "two spinor charges of opposite momenta", "LL"),
    (G4, "$\\Pi'^\\iota_{\\rm LL}$", "Pi'_iota", "the pseudoscalar pair operators, with $\\mathsf K'^\\iota_{\\rm LL}$ in "
     "the place of $\\mathsf K^\\iota_{\\rm LL}$", "LL"),
    (G4, "$\\vec q_{\\rm LL}$, $q_{\\rm LL}$, $q^j_{\\rm LL}$", "q", "the momentum of a plane wave of the spinor field, "
     "its length and its components", "LL"),
    (G4, "$m_{\\rm LL}$, $\\mu_{\\rm LL}$, $\\Lambda_{\\rm LL}$", "m mu Lambda", "the mass of the spinor charges, the "
     "chemical potential, and the momentum cutoff ($q_{\\rm LL}<\\Lambda_{\\rm LL}$; $\\Lambda_{\\rm LL}=1$ sets the units)",
     "LL"),
    (G4, "$\\mathsf h_{\\rm LL}(\\vec q_{\\rm LL})$", "h(q)", "the $8\\times8$ matrix of the free Hamiltonian on a plane "
     "wave, $\\sum_j\\alpha^j_{\\rm LL}q^j_{\\rm LL}+\\beta_{\\rm LL}m_{\\rm LL}$; eigenvalues $\\pm\\varepsilon_{\\rm LL}$",
     "LL"),
    (G4, "$\\varepsilon_{\\rm LL}$, $E^\\pm_{\\rm LL}$", "eps E^+ E^-", "the free energy "
     "$\\varepsilon_{\\rm LL}=(q_{\\rm LL}^2+m_{\\rm LL}^2)^{1/2}$ and the quasi-particle energies of the scalar pair channel "
     "$E^\\pm_{\\rm LL}=[(\\varepsilon_{\\rm LL}\\pm\\mu_{\\rm LL})^2+\\Delta_{\\rm LL}^2]^{1/2}$",
     "LL"),
    (G4, "$N^\\Lambda_{\\rm LL}$", "N_Lambda", "the number of modes per volume and per component below the cutoff, "
     "$\\Lambda_{\\rm LL}^3/(6\\pi^2)$", "LL"),
    # ------------------------------------------------------------------ group 5
    (G5, "$T^\\iota_{\\rm LL}$", "T_LL^iota", "the generators of $SU(2)_{\\rm v}$ on the spinor field, "
     "$\\mathsf 1_4\\otimes\\tau^{\\rm LL}_\\iota/2$; $[T^1_{\\rm LL},T^2_{\\rm LL}]=iT^3_{\\rm LL}$ and cyclically",
     "LL"),
    (G5, "$Q_{\\rm LL}$", "Q_LL", "the charge of the rotations about the third isospin axis, the generator "
     "$Q_{\\rm LL}=T^3_{\\rm LL}$ of the group $U(1)^{\\rm Q}_{\\rm LL}$; $\\pm\\tfrac12$ on isospin up and down; a charge of "
     "$SU(2)_{\\rm v}$, not the electric charge (the DiracLike spinor has electric charge 0)", "LL"),
    (G5, "$\\theta^{\\rm F}_{\\rm LL}$", "thF", "the phase of a local transformation of the fermion number, a real function "
     "on M31", "LL"),
    (G5, "$\\mathsf U_{\\rm LL}$, $\\mathsf R^{\\rm v}_{\\rm LL}(\\mathsf U_{\\rm LL})$", "U_LL Rv", "a local isospin "
     "rotation, a matrix of $SU(2)$ that is a function on M31 and acts on the isospin index ($\\mathsf U_{\\rm LL}=M_{\\rm LL}(\\mathbf v)$); "
     "$\\mathsf R^{\\rm v}_{\\rm LL}(\\mathsf U_{\\rm LL})$, the real orthogonal $3\\times3$ matrix with "
     "$\\mathsf U_{\\rm LL}\\tau^{\\rm LL}_\\iota\\mathsf U_{\\rm LL}^\\dagger=\\sum_{\\iota'}[\\mathsf R^{\\rm v}_{\\rm LL}]_{\\iota'\\iota}\\tau^{\\rm LL}_{\\iota'}$",
     "LL"),
    (G5, "$\\mathcal U_{\\rm LL}$", "Ucal", "a local gauge transformation of the spinor field by GEE, "
     "$\\mathcal U_{\\rm LL}=\\exp(i\\theta^{\\rm F}_{\\rm LL})\\,\\mathsf S_{\\rm LL}(\\mathbf g)\\otimes\\mathsf U_{\\rm LL}$",
     "LL"),
    (G5, "$\\mathbf u^\\omega_{{\\rm LL},p}$; $\\Gamma^{\\rm LL}_p$", "uLw Gamma_LL", "the Lorentz connection, for each "
     "direction $p$ a Lorentz generator, and the spin connection $\\Gamma^{\\rm LL}_p=\\mathsf s_{\\rm LL}(\\mathbf u^\\omega_{{\\rm LL},p})\\otimes\\sigma^{\\rm LL}_0$ "
     "(on flat M31 a pure gauge, zero in the canonical frame)", "LL"),
    (G5, "$g^{\\rm v}_{\\rm LL}$, $g^{\\rm F}_{\\rm LL}$", "g_v g_F", "the gauge couplings of $SU(2)_{\\rm v}$ and of "
     "$U(1)^{\\rm F}_{\\rm LL}$ (real numbers)", "LL"),
    (G5, "$W^\\iota_{{\\rm SF},p}$", "W_p W^iota_p", "the gauge field of $SU(2)_{\\rm v}$: three real vector fields on M31 "
     "(they enter the covariant derivative with $+ig^{\\rm v}_{\\rm LL}$)", "LL-renamed:$A^\\iota_{{\\rm LL},p}$"),
    (G5, "$B_{{\\rm SF},p}$", "B_p", "the gauge field of $U(1)^{\\rm F}_{\\rm LL}$: a real vector field on M31 (it enters the "
     "covariant derivative with $+ig^{\\rm F}_{\\rm LL}F_{\\rm LL}$)", "LL-renamed:$A^{\\rm F}_{{\\rm LL},p}$"),
    (G5, "$\\nabla^{\\rm GEE}_p$", "nabla_GEE", "the GEE gauge-covariant derivative; on the spinor field "
     "$\\partial_p+\\Gamma^{\\rm LL}_p+ig^{\\rm v}_{\\rm LL}\\sum_\\iota W^\\iota_{{\\rm SF},p}T^\\iota_{\\rm LL}+ig^{\\rm F}_{\\rm LL}B_{{\\rm SF},p}$, "
     "on a field of fermion number $F_{\\rm LL}$ and isospin generators $\\mathsf t$ the same with these in place of "
     "1 and $T^\\iota_{\\rm LL}$, and without $\\Gamma^{\\rm LL}_p$ on a Lorentz scalar", "new"),
    (G5, "$\\mathsf t^\\iota_{\\rm SF}$", "t_adj", "the generators of $SU(2)_{\\rm v}$ on an isospin vector (the adjoint "
     "representation), $(\\mathsf t^\\iota_{\\rm SF})_{\\iota'\\iota''}=-i\\epsilon^{\\rm LL}_{\\iota\\iota'\\iota''}$", "new"),
    (G5, "$W^\\iota_{{\\rm SF},pp'}$, $B_{{\\rm SF},pp'}$", "W_pq B_pq", "the field strengths "
     "$\\partial_pW^\\iota_{{\\rm SF},p'}-\\partial_{p'}W^\\iota_{{\\rm SF},p}-g^{\\rm v}_{\\rm LL}\\sum\\epsilon^{\\rm LL}_{\\iota\\iota'\\iota''}W^{\\iota'}_{{\\rm SF},p}W^{\\iota''}_{{\\rm SF},p'}$ "
     "and $\\partial_pB_{{\\rm SF},p'}-\\partial_{p'}B_{{\\rm SF},p}$", "new"),
    (G5, "$\\mathsf W_{{\\rm SF},p}$", "Wmat_p", "the gauge field of $SU(2)_{\\rm v}$ as a Hermitian traceless $2\\times2$ matrix, "
     "$\\sum_\\iota W^\\iota_{{\\rm SF},p}\\tau^{\\rm LL}_\\iota/2$; with two indices, $\\mathsf W_{{\\rm SF},pp'}$, its field strength "
     "$\\partial_p\\mathsf W_{{\\rm SF},p'}-\\partial_{p'}\\mathsf W_{{\\rm SF},p}+ig^{\\rm v}_{\\rm LL}[\\mathsf W_{{\\rm SF},p},\\mathsf W_{{\\rm SF},p'}]$", "new"),
    (G5, "$\\Omega^{\\rm L}_{{\\rm SF},pp'}$", "Omega_L", "the curvature of the Lorentz connection, "
     "$\\partial_p\\Gamma^{\\rm LL}_{p'}-\\partial_{p'}\\Gamma^{\\rm LL}_p+[\\Gamma^{\\rm LL}_p,\\Gamma^{\\rm LL}_{p'}]$ (zero on flat M31)", "new"),
    (G5, "$j^{\\iota,p}_{\\rm SF}$, $j^{{\\rm F},p}_{\\rm SF}$", "j_iota j_F", "the isospin current "
     "$\\bar\\Psi_{\\rm LL}\\gamma^p_{\\rm LL}T^\\iota_{\\rm LL}\\Psi_{\\rm LL}$ and the fermion-number current "
     "$\\bar\\Psi_{\\rm LL}\\gamma^p_{\\rm LL}\\Psi_{\\rm LL}$ of the spinor field", "new"),
    (G5, "$W^\\pm_{{\\rm SF},p}$, $Z_{{\\rm SF},p}$, $A_{{\\rm SF},p}$", "Wplus Wminus Z_p A_p",
     "the gauge fields of definite mass after the breaking of the symmetry of $SU(2)_{\\rm v}\\times U(1)^{\\rm F}_{\\rm LL}$, "
     "named after the Standard Model (the names only: the group is a new one): the fields "
     "$(W^1_{{\\rm SF},p}\\mp iW^2_{{\\rm SF},p})/\\sqrt2$ of the charges $Q_{\\rm LL}=\\pm1$, the massive field of charge 0 and "
     "the massless gauge boson of $U(1)^{\\rm Q}_{\\rm LL}$, which is not the photon and is named the zeugon (their expressions in "
     "$W^3_{{\\rm SF},p}$ and $B_{{\\rm SF},p}$ are derived in the solution)", "new"),
    (G5, "$\\theta^{\\rm W}_{\\rm SF}$, $g^{\\rm Q}_{\\rm SF}$", "theta_W g_Q", "the mixing angle: the angle between the "
     "massless gauge boson $A_{{\\rm SF},p}$ and the axis $W^3_{{\\rm SF},p}$ in the plane of $W^3_{{\\rm SF},p}$ and "
     "$B_{{\\rm SF},p}$; the coupling of $A_{{\\rm SF},p}$ to the charge $Q_{\\rm LL}$", "new"),
    # ------------------------------------------------------------------ group 6
    (G6, "$\\mathcal L_{\\rm SF}$, $\\mathcal L^\\Psi_{\\rm SF}$, $\\mathcal L^{\\rm self}_{\\rm SF}$, $\\mathcal L^{\\rm YM}_{\\rm SF}$",
     "Lagr Lagr_Psi Lagr_self Lagr_YM", "the Lagrangian density of the solution and its parts: the Dirac part with the "
     "covariant derivative and the mass term, the self-interaction (four field components at one point) and the "
     "Yang-Mills part of the gauge fields", "new"),
    (G6, "$G^{\\rm S}_{\\rm SF}$, $G^{\\rm V}_{\\rm SF}$", "G_S G_V", "the couplings of the scalar and of the vector "
     "contact terms, $\\tfrac12G^{\\rm S}_{\\rm SF}:(\\bar\\Psi_{\\rm LL}\\Psi_{\\rm LL})^2:$ and "
     "$\\tfrac12G^{\\rm V}_{\\rm SF}\\sum_p\\eta^{\\rm LL}_{pp}:(j^{{\\rm F},p}_{\\rm SF})^2:$ "
     "of the Hamiltonian density, formed after the author's Kohn-Sham ansatz of the project (attraction in the scalar "
     "density for $G^{\\rm S}_{\\rm SF}<0$, repulsion of the densities for $G^{\\rm V}_{\\rm SF}>0$)", "author-KS"),
    (G6, "$G_S$, $G_V$", "", "the author's scalar and vector couplings of his Kohn-Sham ansatz of uniform matter in the "
     "earlier work of the project (fields of 16 components on the eight-dimensional deflating spacetimes): the "
     "interaction energy density $\\tfrac12G_Vn^2+\\tfrac12G_Sn_S^2$, attraction for $G_S<0$, repulsion for $G_V>0$; "
     "quoted only where that ansatz is described", "DCKS"),
    (G6, "$n$, $n_S$", "", "in the author's Kohn-Sham ansatz: the number density and the scalar density of uniform "
     "matter; quoted only where that ansatz is described", "DCKS"),
    (G6, "$G_{\\rm LL}$, $G^{\\rm crit}_{\\rm LL}$", "G G_crit", "the coupling of the attractive spin-zero contact "
     "interaction $-\\tfrac14G_{\\rm LL}\\sum_\\iota:(\\Pi^\\iota_{\\rm LL})^\\dagger\\Pi^\\iota_{\\rm LL}:$ of the Hamiltonian "
     "density, and its critical value for the condensation of the vacuum with this interaction alone",
     "LL"),
    (G6, "$V^{(0)}_{\\rm SF}$, $V^{(1)}_{\\rm SF}$, $V^{(n)}_{\\rm SF}$", "V0 V1 Vn", "the self-interaction of the "
     "step $n$ of the recursion of the potential: the set of contact terms (with their couplings) and of gauge-boson "
     "exchanges examined at that step", "new"),
    (G6, "$\\mathsf X_{\\rm SF}$, $\\mathsf Y_{\\rm SF}$", "Xmat Ymat", "two arbitrary $8\\times8$ matrices in the general statements "
     "of Wick's theorem and of the self-energies of a contact term $\\nu{:}(\\Psi^\\dagger\\mathsf X_{\\rm SF}\\Psi)(\\Psi^\\dagger\\mathsf Y_{\\rm SF}\\Psi){:}$", "new"),
    (G6, "$\\mathsf R_{\\rm SF}(\\vec q_{\\rm LL})$", "Rgen", "the generalized density of a uniform quasi-free state at the momentum "
     "$\\vec q_{\\rm LL}$: the $16\\times16$ Hermitian matrix of the expectation values $\\langle\\mathcal N_{{\\rm LL},b}^\\dagger\\mathcal N_{{\\rm LL},a}\\rangle$ "
     "of the Nambu spinor, with the blocks $\\mathsf C^{\\rm n}_{\\rm LL}(\\vec q_{\\rm LL})$, $-\\mathsf C^{\\rm p}_{\\rm LL}(\\vec q_{\\rm LL})$, "
     "$-\\mathsf C^{\\rm p}_{\\rm LL}(\\vec q_{\\rm LL})^\\dagger$, $\\mathsf 1_8-\\mathsf C^{\\rm n}_{\\rm LL}(-\\vec q_{\\rm LL})^*$", "new"),
    (G6, "$\\mathcal H_{\\rm SF}$", "Hdens", "the Hamiltonian density of the spinor field, minus the Lagrangian density "
     "without the time derivative terms", "new"),
    # ------------------------------------------------------------------ group 7
    (G7, "$\\vec d_{\\rm LL}$, $d^\\iota_{\\rm LL}$; $\\zeta_{\\rm LL}$", "dvec d_iota zeta", "the order parameter, the "
     "isospin vector $d^\\iota_{\\rm LL}=-\\tfrac12G_{\\rm LL}\\langle\\Pi^\\iota_{\\rm LL}\\rangle$ in $\\mathbb C^3$, and its "
     "non-unitarity $\\zeta_{\\rm LL}=\\lvert\\vec d_{\\rm LL}\\times\\vec d^{\\,*}_{\\rm LL}\\rvert/\\lvert\\vec d_{\\rm LL}\\rvert^2$",
     "LL"),
    (G7, "$\\Delta_{\\rm LL}$, $\\Delta^\\pm_{\\rm LL}$", "Delta Delta_+ Delta_-", "the gap "
     "$\\lvert\\vec d_{\\rm LL}\\rvert$ of a unitary state ($\\zeta_{\\rm LL}=0$), and the two gaps "
     "$(\\Delta^\\pm_{\\rm LL})^2=\\lvert\\vec d_{\\rm LL}\\rvert^2(1\\pm\\zeta_{\\rm LL})$ of a general state",
     "LL"),
    (G7, "$\\mathsf P_{\\rm LL}$; $\\mathsf H_{\\rm LL}(\\vec q_{\\rm LL})$; $\\mathcal N_{\\rm LL}(\\vec q_{\\rm LL})$",
     "Ppair H_BdG", "the pairing block $\\sum_\\iota d^\\iota_{\\rm LL}(\\mathsf K^\\iota_{\\rm LL})^\\dagger$; the "
     "$16\\times16$ Bogoliubov-de Gennes matrix of $8\\times8$ blocks: upper left "
     "$\\mathsf h_{\\rm LL}(\\vec q_{\\rm LL})-\\mu_{\\rm LL}$, upper right $\\mathsf P_{\\rm LL}$, lower left "
     "$\\mathsf P_{\\rm LL}^\\dagger$, lower right $-(\\mathsf h_{\\rm LL}(-\\vec q_{\\rm LL})-\\mu_{\\rm LL})^T$; and the "
     "Nambu spinor $(\\Psi_{\\rm LL}(\\vec q_{\\rm LL}),\\Psi^*_{\\rm LL}(-\\vec q_{\\rm LL}))$ on which it acts",
     "LL"),
    (G7, "$M^{\\rm q}_{\\rm LL}$", "M_q", "the quasi-particle mass of a uniform polar state at $\\mu_{\\rm LL}=0$, "
     "$(m_{\\rm LL}^2+\\Delta_{\\rm LL}^2)^{1/2}$", "LL"),
    (G7, "$\\omega^{\\rm pw}_{\\rm SF}$, $\\chi^{\\rm pw}_{\\rm SF}$", "omega_pw chi_pw", "the frequency and the constant "
     "eight-component spinor of a plane-wave solution of the field equation in constant gauge fields", "new"),
    (G7, "$\\chi^{\\rm ch}_{\\rm SF}(\\omega)$, $M^{\\rm B}_{\\rm SF}$", "chi_ch M_B_SF", "the pair susceptibility of the "
     "normal vacuum in a pair channel at the frequency $\\omega$ and zero momentum, and the mass of the bound state of two "
     "spinor charges in that channel (the solution of $G^{\\rm ch}_{\\rm SF}\\chi^{\\rm ch}_{\\rm SF}(M^{\\rm B}_{\\rm SF})=1$ below "
     "$2m_{\\rm LL}$)", "new"),
    (G7, "$\\mathsf H^{\\rm pw}_{\\rm SF}(\\vec q_{\\rm LL})$, $\\mathsf k_{{\\rm SF},j}$", "H_pw k_j", "the Hermitian $8\\times8$ matrix whose "
     "eigenvalues are the frequencies of the plane waves of momentum $\\vec q_{\\rm LL}$ in constant gauge fields, "
     "$\\beta_{\\rm LL}m_{\\rm LL}+g^{\\rm v}_{\\rm LL}\\mathsf 1_4\\otimes\\mathsf W_{{\\rm SF},0}+g^{\\rm F}_{\\rm LL}B_{{\\rm SF},0}+\\sum_j\\alpha^j_{\\rm LL}\\mathsf k_{{\\rm SF},j}$, "
     "and the kinetic momenta $\\mathsf k_{{\\rm SF},j}=q^j_{\\rm LL}+g^{\\rm v}_{\\rm LL}\\mathsf 1_4\\otimes\\mathsf W_{{\\rm SF},j}+g^{\\rm F}_{\\rm LL}B_{{\\rm SF},j}$", "new"),
    (G7, "$w^{\\rm lk}_{\\rm SF}$", "w_lk", "the amplitude of the spin-isospin locked constant gauge field "
     "$W^\\iota_{{\\rm SF},j}=w^{\\rm lk}_{\\rm SF}\\delta_{\\iota j}$", "new"),
    (G7, "$\\Sigma^j_{\\rm SF}$", "Sigma_j", "the spin matrices $\\mathrm{diag}(\\sigma^{\\rm LL}_j,\\sigma^{\\rm LL}_j)$ on the Dirac index "
     "(twice the spin)", "new"),
    (G7, "$g^{\\rm c}_{\\rm SF}$", "g_c", "the strength with which an isospin component couples to an abelian gauge field: "
     "$g^{\\rm v}_{\\rm LL}Q_{\\rm LL}$ for the field $W^3_{{\\rm SF},p}$, $g^{\\rm F}_{\\rm LL}$ for $B_{{\\rm SF},p}$", "new"),
    (G7, "$\\mathcal E^{\\rm MF}_{\\rm SF}(\\Delta_{\\rm LL})$", "E_MF", "the energy per volume of the uniform polar state of the "
     "pairing interaction in the mean-field (pairing-only) approximation at $\\mu_{\\rm LL}=0$, as a function of the gap, "
     "measured from the normal vacuum: the ground-state energy of the mean-field Hamiltonian plus "
     "$\\Delta_{\\rm LL}^2/G_{\\rm LL}$", "new"),
    (G7, "$I^{(1)}_{\\rm SF}$, $I^{(3)}_{\\rm SF}$", "I1 I3", "the integrals $\\int_0^{\\Lambda_{\\rm LL}}q^2(q^2+M^2)^{\\mp1/2}dq$ "
     "(upper sign $I^{(1)}_{\\rm SF}$, lower $I^{(3)}_{\\rm SF}$) as functions of the mass $M$ in their argument; at the "
     "temperature $T$, $I^{(1)}_{\\rm SF}(M,T)=\\int_0^{\\Lambda_{\\rm LL}}q^2\\tanh(E/2T)E^{-1}dq$ with "
     "$E=(q^2+M^2)^{1/2}$, which equals $I^{(1)}_{\\rm SF}(M)$ at $T=0$", "new"),
    (G7, "$\\mathsf A^{\\rm K}_{\\rm SF}$, $\\mathsf P^{\\rm K}_{\\rm SF}$, $\\mathsf B^{\\rm K}_{\\rm SF}$, $\\mathsf G^{\\rm K}_{\\rm SF}$",
     "A_K P_K B_K G_K", "the $16\\times16$ matrices of the kink problem: $\\mathrm{diag}(\\alpha^1_{\\rm LL},(\\alpha^1_{\\rm LL})^*)$, the "
     "pairing matrix with the blocks $(\\mathsf K^3_{\\rm LL})^\\dagger$ (upper right) and $\\mathsf K^3_{\\rm LL}$ (lower left), "
     "$\\mathrm{diag}(\\beta_{\\rm LL},-\\beta_{\\rm LL})$, and $-i\\mathsf A^{\\rm K}_{\\rm SF}\\mathsf P^{\\rm K}_{\\rm SF}$", "new"),
    (G7, "$\\mathsf H^{\\rm K}_{\\rm SF}$, $\\mathsf H^{{\\rm K},0}_{\\rm SF}$", "H_K H_K0", "the Bogoliubov-de Gennes operator of the "
     "quasi-particles in the kink of the pair field, $-i\\mathsf A^{\\rm K}_{\\rm SF}\\partial_1+\\Delta_{\\rm LL}(x)\\mathsf P^{\\rm K}_{\\rm SF}+m_{\\rm LL}\\mathsf B^{\\rm K}_{\\rm SF}$, "
     "and the same without the mass term", "new"),
    (G7, "$E^{\\rm K}_{{\\rm SF},n}$, $s^{\\rm K}_{\\rm SF}$", "E_Kn s_K", "the energies of the bound states of the kink problem "
     "without the mass, $(E^{\\rm K}_{{\\rm SF},n})^2=(\\Delta^\\infty_{\\rm LL})^2-(\\Delta^\\infty_{\\rm LL}-n/\\ell^{\\rm K}_{\\rm LL})^2$, "
     "and the index $s^{\\rm K}_{\\rm SF}=\\Delta^\\infty_{\\rm LL}\\ell^{\\rm K}_{\\rm LL}$ of the Poeschl-Teller potential", "new"),
    (G7, "$\\Delta^\\infty_{\\rm LL}$, $\\ell^{\\rm K}_{\\rm LL}$", "Delta_inf ell_K", "the gap far from a domain wall "
     "(kink) of the pair field, and the width of the wall", "LL"),
    (G7, "$\\varphi^{\\rm K}_{\\rm SF}$, $\\varphi^{\\rm K}_{{\\rm SF},n}$, $\\chi^{\\rm K}_{\\rm SF}$", "phi_K chi_K", "the profile of a "
     "bound state of a kink of the pair field (with the number $n$ of the state: $n=0$, $1$, ...), and the constant "
     "16-component spinor of that state", "new"),
    (G7, "$\\mathcal B_{\\rm SF}$, $n^{\\rm L}_{\\rm SF}$, $\\ell^{\\rm B}_{\\rm SF}$", "Bmag n_L l_B", "the strength of a "
     "constant magnetic field (of the gauge field $W^3_{{\\rm SF},p}$ or of $B_{{\\rm SF},p}$, as said in the text), the number of a Landau level "
     "(0, 1, 2, ...), and the magnetic length", "new"),
    # ------------------------------------------------------------------ group 8
    (G8, "$\\mathsf K_{\\rm SF}[\\Gamma,\\mathsf t]$", "Kpair", "the general pair matrix "
     "$C_{\\rm LL}\\Gamma\\otimes\\mathsf E_{\\rm LL}\\mathsf t$ for a $4\\times4$ Dirac matrix $\\Gamma$ and a $2\\times2$ "
     "isospin matrix $\\mathsf t$ (antisymmetric $8\\times8$ when $C_{\\rm LL}\\Gamma$ and $\\mathsf E_{\\rm LL}\\mathsf t$ have "
     "opposite symmetry)", "new"),
    (G8, "$\\Gamma$", "Gamma_D", "one of the sixteen Dirac matrices $\\mathsf 1_4$, $\\gamma^p_{\\rm LL}$, "
     "$\\sigma^{pp'}_{\\rm SF}=\\tfrac i2[\\gamma^p_{\\rm LL},\\gamma^{p'}_{\\rm LL}]$, $\\gamma^p_{\\rm LL}\\gamma^{\\rm ch}_{\\rm LL}$, "
     "$\\gamma^{\\rm ch}_{\\rm LL}$ (without scripts and in square brackets: a placeholder for any of them)", "new"),
    (G8, "$\\sigma^{pp'}_{\\rm SF}$", "sigma_pq", "the six antisymmetric Dirac matrices "
     "$\\tfrac i2(\\gamma^p_{\\rm LL}\\gamma^{p'}_{\\rm LL}-\\gamma^{p'}_{\\rm LL}\\gamma^p_{\\rm LL})$", "new"),
    (G8, "$\\lvert\\mathsf K\\rangle$", "pairstate", "the normalized state of two spinor charges at one point with the "
     "antisymmetric pair matrix $\\mathsf K$: $\\tfrac12\\sum\\mathsf K^*_{cc'}\\Psi^\\dagger_{{\\rm LL},c}\\Psi^\\dagger_{{\\rm LL},c'}$ "
     "applied to the empty state, divided by its norm", "new"),
    (G8, "$E^{\\rm pair}_{\\rm SF}[\\mathcal O;\\mathsf K]$", "E_pair", "the energy of a normal-ordered contact term "
     "$\\mathcal O$ (an operator built from four field components at one point) in the pair state $\\lvert\\mathsf K\\rangle$, "
     "per unit coupling: negative means attraction", "new"),
    (G8, "$G^{\\rm ch}_{\\rm SF}$", "G_ch", "the effective coupling of a self-interaction in a pair channel, normalized "
     "like $G_{\\rm LL}$: $-\\tfrac14$ times the pair energy of the channel", "new"),
    (G8, "$D^{\\rm b}_{\\rm SF}(\\vec k)$, $M^{\\rm b}_{\\rm SF}$", "D_b M_b", "the static kernel "
     "$1/(\\lvert\\vec k\\rvert^2+(M^{\\rm b}_{\\rm SF})^2)$ of the exchange of the gauge boson b (b = $A$, $W$, $Z$) with the "
     "momentum transfer $\\vec k$, and the mass of that boson", "new"),
    (G8, "$\\mathsf t^{\\rm b}_{\\rm SF}$, $\\mathsf A^{\\rm b}_{{\\rm SF},p}$", "t_b A_b", "the isospin matrix of the "
     "gauge boson b ($\\tfrac12\\tau^{\\rm LL}_\\iota$ for $W^\\iota_{{\\rm SF},p}$, $\\mathsf 1_2$ for $B_{{\\rm SF},p}$) and "
     "its vertex matrices $(\\gamma^0_{\\rm LL}\\gamma^p_{\\rm LL})\\otimes\\mathsf t^{\\rm b}_{\\rm SF}$ (the current of b is "
     "$\\Psi^\\dagger_{\\rm LL}\\mathsf A^{\\rm b}_{{\\rm SF},p}\\Psi_{\\rm LL}$)", "new"),
    (G8, "$\\lambda^{\\rm T}_{\\rm SF}$, $g^{\\rm crit}_{\\rm SF}$", "lambda_T g_crit", "the largest eigenvalue of the "
     "linearized gap equation of the normal vacuum in the channel $\\mathsf K^3_{\\rm LL}$ with the exchange of gauge bosons "
     "(the normal vacuum is unstable against pairing when it exceeds 1), and the coupling at which $W^3_{\\rm SF}$ alone would "
     "make it 1", "new"),
    (G8, "$\\mathcal E^{\\rm ex,b}_{\\rm SF}$", "E_ex_b", "the energy per volume of the static exchange of the gauge boson b "
     "in a uniform state, measured from the normal vacuum", "new"),
    # ------------------------------------------------------------------ group 9
    (G9, "HS, CJT, KS; $X$", "HS CJT KS X", "the labels of the three methods: Hubbard-Stratonovich, "
     "Cornwall-Jackiw-Tomboulis, Kohn-Sham; $X$ stands for any one of them", "author"),
    (G9, "$\\sigma^{\\rm HS}_{\\rm SF}$, $\\omega^{{\\rm HS},p}_{\\rm SF}$, $\\vec\\Phi^{\\rm HS}_{\\rm SF}$",
     "sigma_HS omega_HS Phi_HS", "the auxiliary (Hubbard-Stratonovich) fields of the scalar density, of the vector current "
     "and of the pair channel", "new"),
    (G9, "$s^{\\rm HS,S}_{\\rm SF}$, $s^{\\rm HS,V}_{\\rm SF}$", "s_S s_V", "the HS parameters: the shares (between 0 "
     "and 1) of the scalar and of the vector contact term that are decoupled in the density channels (their direct "
     "contraction); the rest of each term is decoupled in the pair channels (its pair contraction)", "new"),
    (G9, "$w^{\\rm H}_{\\rm SF}$, $w^{\\rm F}_{\\rm SF}$, $w^{\\rm P}_{\\rm SF}$", "w_h w_f w_p", "the weights with "
     "which a method keeps the direct (Hartree), the exchange (Fock) and the pair (Bogoliubov) contraction of a contact "
     "term in its energy per volume", "new"),
    (G9, "$\\mathcal P^{\\rm KS}_{\\rm SF}$", "ks_project", "the projection of an $8\\times8$ matrix $\\mathsf X$ onto the "
     "span of $\\mathsf 1_8$ and $\\beta_{\\rm LL}\\otimes\\mathsf 1_2$: "
     "$\\mathcal P^{\\rm KS}_{\\rm SF}(\\mathsf X)=\\tfrac18\\mathrm{tr}(\\mathsf X)\\,\\mathsf 1_8+\\tfrac18\\mathrm{tr}(\\beta_{\\rm LL}\\mathsf X)\\,\\beta_{\\rm LL}$ "
     "(the Kohn-Sham densities are the number density and the scalar density)", "new"),
    (G9, "$g^{X,{\\rm S}}_{\\rm SF}$, $g^{X,{\\rm P}}_{\\rm SF}$", "g_S g_P", "the effective couplings of method $X$ in "
     "the scalar-density channel (a uniform scalar density $n^{\\rm S}_{\\rm SF}$ shifts the Dirac mass by "
     "$g^{X,{\\rm S}}_{\\rm SF}n^{\\rm S}_{\\rm SF}$) and in the pair channel $\\mathsf K^3_{\\rm LL}$ (normalized so that "
     "the pairing term alone has $g^{X,{\\rm P}}_{\\rm SF}=G_{\\rm LL}$)", "new"),
    (G9, "$\\chi^{\\rm S}_{\\rm SF}$", "chi_S", "the scalar susceptibility of the normal vacuum, "
     "$-\\partial n^{\\rm S}_{\\rm SF}/\\partial M=(2/\\pi^2)\\int_0^{\\Lambda_{\\rm LL}}q^4(q^2+m_{\\rm LL}^2)^{-3/2}dq$",
     "new"),
    (G9, "$D^X_{\\rm SF}$", "D_X", "the pair amplitude of a polar state of method $X$: its pair self-energy is "
     "$D^X_{\\rm SF}(\\mathsf K^3_{\\rm LL})^\\dagger$", "new"),
    (G9, "$\\boldsymbol\\chi_{\\rm SF}(\\omega)$, $\\mathsf F^X_{\\rm SF}$, $\\mathsf Q^X_{\\rm SF}(\\omega)$",
     "chi_resp F_kern Q_resp", "the linear response of a uniform state at the frequency $\\omega$ (the map from the "
     "amplitudes of the self-energies to the amplitudes of the local correlation matrices, a $120\\times120$ matrix), the "
     "kernel of method $X$ (the map from the amplitudes of the correlation matrices to those of the self-energies), and "
     "their product $\\boldsymbol\\chi_{\\rm SF}(\\omega)\\mathsf F^X_{\\rm SF}$", "new"),
    (G9, "$\\lambda^{\\rm S}_{\\rm SF}$, $\\lambda^{\\rm P}_{\\rm SF}$", "lambda_S lambda_P", "the eigenvalues of "
     "$\\mathsf Q^X_{\\rm SF}(0)$ of the normal vacuum in the scalar-density channel and in the pair channel (an eigenvalue "
     "larger than 1 means that the normal vacuum is unstable in that channel); $\\lambda^{\\rm P}_{\\rm SF}(T)$ is the "
     "eigenvalue of the pair channel of the normal state at the temperature $T$", "new"),
    (G9, "$\\Omega_{\\rm SF}$", "Omega", "the energy of a transition between an occupied and an empty eigenvector of "
     "$\\mathsf H_{\\rm SF}(\\vec q_{\\rm LL})$, the difference of their eigenvalues", "new"),
    (G9, "$\\Gamma^{\\rm CJT}_{\\rm LL}$; $\\mathcal G_{\\rm LL}$", "Gamma_CJT G_prop", "the effective action of the "
     "propagator in the method of Cornwall, Jackiw and Tomboulis, and the propagator of the Nambu spinor of 16 components",
     "LL"),
    (G9, "$\\mathsf C^{\\rm n}_{\\rm LL}$, $\\mathsf C^{\\rm p}_{\\rm LL}$; $\\mathsf C^{\\rm vac}_{\\rm LL}$", "C_n C_p C_vac",
     "the correlation matrices of a uniform state: $(\\mathsf C^{\\rm n}_{\\rm LL})_{cc'}=\\langle\\Psi^\\dagger_{{\\rm LL},c'}\\Psi_{{\\rm LL},c}\\rangle$ "
     "(normal, Hermitian) and $(\\mathsf C^{\\rm p}_{\\rm LL})_{cc'}=\\langle\\Psi_{{\\rm LL},c}\\Psi_{{\\rm LL},c'}\\rangle$ (pair, "
     "antisymmetric), both fields at one point, and the normal correlation matrix of the normal vacuum; with the argument "
     "$\\vec q_{\\rm LL}$, their parts of one momentum", "LL"),
    (G9, "$\\mathsf N_{\\rm SF}(\\vec q_{\\rm LL})$, $\\mathsf P_{\\rm SF}(\\vec q_{\\rm LL})$", "N_self P_self",
     "the normal and the pair self-energy of a method (Hermitian and antisymmetric $8\\times8$ matrices), which may depend "
     "on the momentum", "new"),
    (G9, "$\\mathsf H_{\\rm SF}(\\vec q_{\\rm LL})$", "H_SF", "the Bogoliubov-de Gennes matrix with the self-energies, of "
     "$8\\times8$ blocks: upper left $\\mathsf h_{\\rm LL}(\\vec q_{\\rm LL})+\\mathsf N_{\\rm SF}(\\vec q_{\\rm LL})$, upper right "
     "$\\mathsf P_{\\rm SF}(\\vec q_{\\rm LL})$, lower left its Hermitian conjugate, lower right minus the transpose of "
     "$\\mathsf h_{\\rm LL}(-\\vec q_{\\rm LL})+\\mathsf N_{\\rm SF}(-\\vec q_{\\rm LL})$", "new"),
    (G9, "$\\mathcal E^X_{\\rm SF}$", "E_X", "the energy per volume of a uniform state in the approximation of method $X$, "
     "measured from the normal vacuum", "new"),
    (G9, "$n_{\\rm SF}$, $n^{\\rm S}_{\\rm SF}$", "n_SF nS_SF", "the number density "
     "$\\langle\\Psi^\\dagger_{\\rm LL}\\Psi_{\\rm LL}\\rangle$ and the scalar density "
     "$\\langle\\bar\\Psi_{\\rm LL}\\Psi_{\\rm LL}\\rangle$ of a uniform state, each measured from its value in the normal "
     "vacuum", "new"),
    (G9, "$m^{\\rm KS}_{\\rm SF}$, $V^{\\rm KS}_{0,{\\rm SF}}$, $\\vec\\Phi^{\\rm KS}_{\\rm SF}$", "m_KS V0_KS Phi_KS",
     "the Kohn-Sham potentials: the Dirac mass, the vector potential and the pair potential of the auxiliary system of "
     "independent spinor charges", "author-KS"),
    (G9, "$G^{\\rm KS,S}_{\\rm SF}$, $G^{\\rm KS,V}_{\\rm SF}$, $G^{\\rm KS,P}_{\\rm SF}$", "GKS_S GKS_V GKS_P",
     "the couplings of the Kohn-Sham functional in the scalar density, in the number density and in the pair channel",
     "author-KS"),
    (G9, "$T^{\\rm s}_{\\rm SF}$, $E^{\\rm xc}_{\\rm SF}$", "T_s E_xc", "the energy of the independent spinor charges of "
     "the Kohn-Sham system, and the exchange-correlation part of the Kohn-Sham functional", "new"),
    (G9, "$E^{X,{\\rm gap}}_{\\rm SF}$, $M^{X,{\\rm D}}_{\\rm SF}$", "E_gap M_D", "the gap of method $X$ (the lowest "
     "energy of one quasi-particle) and the Dirac mass of its ground state", "new"),
    # ------------------------------------------------------------------ group 10
    (G10, "$\\vec\\Phi^X_{\\rm SF}$, $\\Phi^{X,\\iota}_{\\rm SF}$", "Phi_X", "the field of the superfluid boson singlet "
     "state in method $X$, a complex isospin vector of fermion number 2 and spin 0: the amplitudes of the pair self-energy "
     "of a uniform state, $\\mathsf P_{\\rm SF}=\\sum_\\iota\\Phi^{X,\\iota}_{\\rm SF}(\\mathsf K^\\iota_{\\rm LL})^\\dagger$ (the value of the "
     "auxiliary pair field in HS, the anomalous self-energy in CJT, the Kohn-Sham pair potential in KS)", "new"),
    (G10, "$\\Phi^X_{0,{\\rm SF}}$", "Phi0", "the modulus of the field $\\vec\\Phi^X_{\\rm SF}$ in the vacuum of method $X$ "
     "(the minimum of $\\mathcal V^X_{\\rm SF}$); $\\Phi^X_{0,{\\rm SF}}(T)$ is the modulus in the superfluid at the "
     "temperature $T$ (the minimum of $\\mathcal F^X_{\\rm SF}$), with $\\Phi^X_{0,{\\rm SF}}(0)=\\Phi^X_{0,{\\rm SF}}$", "new"),
    (G10, "$Z^X_{\\rm SF}$, $Y^X_{\\rm SF}$", "Z_X Y_X", "the two coefficients of the kinetic part of the effective "
     "Lagrangian of method $X$: of $(\\nabla\\vec\\Phi)^\\dagger\\cdot\\nabla\\vec\\Phi$ and of "
     "$\\lvert\\vec\\Phi^\\dagger\\cdot\\nabla\\vec\\Phi\\rvert^2/\\lvert\\vec\\Phi\\rvert^2$ (each contracted over the coordinates "
     "with $\\eta^{\\rm LL}_{pp}$)", "new"),
    (G10, "$f^{X,{\\rm F}}_{\\rm SF}$, $f^{X,{\\rm I}}_{\\rm SF}$; $\\tilde f^{X,{\\rm F}}_{\\rm SF}$, $\\tilde f^{X,{\\rm I}}_{\\rm SF}$, $\\tilde f^{X,3}_{\\rm SF}$",
     "fF fI fF_s fI_s f3_s", "the decay constants of the Goldstone field of the phase (F) and of the isospin rotations (I) "
     "in method $X$, from the time components (the static susceptibilities); the same from the space components (states "
     "with a uniform twist), and the space-component stiffness of the unbroken generator $T^3_{\\rm LL}$, which vanishes in "
     "a gauge-invariant regularization", "new"),
    (G10, "$\\nu^X_{{\\rm SF},1}$, $\\nu^X_{{\\rm SF},2}$", "nu_1 nu_2", "the two non-unitary fields of "
     "$\\mathrm{SFBSv}^X$ (the imaginary parts of the components 1 and 2 of the field relative to its phase)", "new"),
    (G10, "$a^X_{\\rm SF}$, $b^X_{\\rm SF}$, $c^X_{\\rm SF}$, $u^X_{\\rm SF}$", "landau_a landau_b landau_c u_X landau_res landau_phifit landau_depthfit landau_curvfit landau_signs landau_resK landau_cK4 landau_rng",
     "the coefficients of the Landau form $a\\lvert\\vec\\Phi\\rvert^2+b\\lvert\\vec\\Phi\\rvert^4+c\\lvert\\vec\\Phi\\rvert^6"
     "+u(\\lvert\\vec\\Phi\\rvert^4-\\lvert\\vec\\Phi\\cdot\\vec\\Phi\\rvert^2)$ of $\\mathcal V^X_{\\rm SF}$ (the first three fitted "
     "along the polar states, $u$ from the curvature along the non-unitary directions)", "new"),
    (G10, "$T^{X,{\\rm c}}_{\\rm SF}$", "Tc", "the critical temperature of method $X$: the temperature at which the "
     "eigenvalue $\\lambda^{\\rm P}_{\\rm SF}(T)$ of the static pair response of the normal state reaches 1 (with $k_{\\rm B}=1$, "
     "in units of $\\Lambda_{\\rm LL}$)", "new"),
    (G10, "$\\mathcal S_{\\rm SF}$", "entropy S_minus_S_normal", "the entropy per volume of a uniform quasi-free state: minus one half of the integral "
     "$\\int d^3q/(2\\pi)^3$ over the momenta of $\\sum_k[f_k\\ln f_k+(1-f_k)\\ln(1-f_k)]$, the sum over the 16 eigenvalues "
     "of the matrix of Bogoliubov and de Gennes at the momentum, with their occupations $f_k$", "new"),
    (G10, "$\\mathcal F^X_{\\rm SF}$", "F_min free", "the free energy of the field $\\vec\\Phi$ in method $X$ at the temperature $T$: "
     "the smallest $\\mathcal E^X_{\\rm SF}-T\\mathcal S_{\\rm SF}$ among the thermal states with the field $\\vec\\Phi$, measured from "
     "the normal state at the same temperature; at $T=0$ the effective potential", "new"),
    (G10, "$M^{X,{\\rm n}}_{\\rm SF}$", "Dirac_mass_normal mass_n", "the Dirac mass of the normal state of method $X$ at the "
     "temperature $T$ (the mass plus the scalar part of its normal self-energy)", "new"),
    (G10, "$r^X_{\\rm SF}$", "radius", "the radius of convergence, up to a factor of the order of 1, of the series of the free "
     "energy of method $X$ in powers of $\\lvert\\vec\\Phi\\rvert^2$ about the normal state: "
     "$(M^{X,{\\rm n}}_{\\rm SF}(T)^2+\\pi^2T^2)^{1/2}$", "new"),
    (G10, "$b^{X,{\\rm Tay}}_{\\rm SF}$", "b_Taylor", "the coefficient of $\\lvert\\vec\\Phi\\rvert^4$ of the Taylor series of the "
     "free energy of method $X$ at $\\vec\\Phi=0$ (at the temperature $T$)", "new"),
    (G10, "$a^{X,{\\rm Tay}}_{\\rm SF}$", "a_taylor a_Taylor", "the coefficient of $\\lvert\\vec\\Phi\\rvert^2$ of the Taylor series of "
     "the effective potential $\\mathcal V^X_{\\rm SF}$ at $\\vec\\Phi=0$ (one half of its second derivative along the polar "
     "fields there), computed from states with small applied pair fields; the coefficient $a^X_{\\rm SF}$ of the fitted Landau "
     "form differs from it; $a^{X,{\\rm Tay}}_{\\rm SF}(T)$ is the same coefficient of the free energy "
     "$\\mathcal F^X_{\\rm SF}$ at the temperature $T$, and $a^{X,{\\rm Tay}}_{\\rm SF}(q^{\\rm pair}_{\\rm SF};T)$ that of a pair field "
     "moving with the momentum $q^{\\rm pair}_{\\rm SF}$ ($a^{X,{\\rm Tay}}_{\\rm SF}(0;T)=a^{X,{\\rm Tay}}_{\\rm SF}(T)$)", "new"),
    (G10, "$\\mathcal F^{X,{\\rm GL}}_{\\rm SF}$", "F_GL", "the free energy of Ginzburg and Landau of method $X$ near its "
     "critical temperature: the functional of a static field $\\vec\\Phi(\\vec x)$ that varies slowly in space, to second order "
     "in the field and in its derivatives and to fourth order in the field", "new"),
    (G10, "$\\mathsf c^X_{\\rm SF}$, $\\mathsf c^{X,{\\rm T3}}_{\\rm SF}$, $\\mathsf c^{X,{\\rm tw}}_{\\rm SF}$",
     "gz_c gz_c_fixed gz_c_T3 gz_c_twisted c_isospin_twist c_fixed_channels c_T3_twist c_twisted_states c_k001 c_k002",
     "the gradient coefficient of method $X$ at its critical temperature: the coefficient of "
     "$\\sum_j(\\partial_j\\vec\\Phi)^\\dagger\\cdot\\partial_j\\vec\\Phi$ in the free energy of a static field, from the static pair "
     "response of a moving pair with the cutoff on the relative momentum; the spurious coefficient of the twist of the "
     "unbroken generator $T^3_{\\rm LL}$, which vanishes in a gauge-invariant regularization; and the coefficient from the "
     "twisted states of the nonlinear engine with a self-consistent normal self-energy", "new"),
    (G10, "$\\alpha^{X,{\\rm GL}}_{\\rm SF}$", "gz_alpha", "the slope of the coefficient of $\\lvert\\vec\\Phi\\rvert^2$ at the "
     "critical temperature of method $X$: $T^{X,{\\rm c}}_{\\rm SF}$ times the derivative of $a^{X,{\\rm Tay}}_{\\rm SF}(T)$ with "
     "respect to the temperature", "new"),
    (G10, "$t^{\\rm red}_{\\rm SF}$", "", "the reduced temperature $(T-T^{X,{\\rm c}}_{\\rm SF})/T^{X,{\\rm c}}_{\\rm SF}$", "new"),
    (G10, "$n^\\Phi_{\\rm SF}$", "N_COMP", "the number of real components of the field $\\vec\\Phi^X_{\\rm SF}$: 6 (the real and "
     "imaginary parts of its three isospin components)", "new"),
    (G10, "${\\rm Gi}^{X}_{\\rm SF}$", "gz_Gi Gi Gi_Tc gz_width", "the Ginzburg number of method $X$: the reduced temperature at "
     "which the specific heat of the Gaussian fluctuations of the field equals the jump of the specific heat of mean-field "
     "theory: $(n^\\Phi_{\\rm SF}b^{X,{\\rm Tay}}_{\\rm SF}T^{X,{\\rm c}}_{\\rm SF})^2$ divided by "
     "$64\\pi^2\\alpha^{X,{\\rm GL}}_{\\rm SF}(\\mathsf c^X_{\\rm SF})^3$; "
     "the critical region has the width ${\\rm Gi}^{X}_{\\rm SF}T^{X,{\\rm c}}_{\\rm SF}$ in temperature", "new"),
    (G10, "$C^{X,{\\rm fl}}_{\\rm SF}$, $\\Delta C^X_{\\rm SF}$", "", "the specific heat per volume of the Gaussian fluctuations of "
     "the field above the critical temperature of method $X$, and the jump of the specific heat of mean-field theory at "
     "the critical temperature", "new"),
    (G10, "$\\xi^{X,{\\rm GL}}_{\\rm SF}$", "gz_xi0 xi0", "the amplitude $(\\mathsf c^X_{\\rm SF}/\\alpha^{X,{\\rm GL}}_{\\rm SF})^{1/2}$ "
     "of the correlation length of mean-field theory near the critical temperature of method $X$ (the correlation length "
     "is $\\xi^{X,{\\rm GL}}_{\\rm SF}\\lvert t^{\\rm red}_{\\rm SF}\\rvert^{-1/2}$)", "new"),
    (G10, "$q^{\\rm pair}_{\\rm SF}$", "K1 k_pair", "the momentum of a moving pair field $\\vec\\Phi e^{iq^{\\rm pair}_{\\rm SF}x^3}$, "
     "along $\\vec e_3$", "new"),
    (G10, "$\\mathsf T^{\\rm tw}_{\\rm SF}$", "twist_matrix", "the twist: a constant matrix $Q\\,\\alpha^3_{\\rm LL}\\otimes\\mathsf t$ "
     "added to the particle and hole blocks of the BdG matrix, which moves the charges of the isospin matrix $\\mathsf t$ with "
     "the momentum $Q$ along $\\vec e_3$ (for a moving pair: $Q=q^{\\rm pair}_{\\rm SF}/2$, $\\mathsf t=\\mathsf 1_2$)", "new"),
    (G10, "$\\mathsf P^{\\rm a}_{\\rm SF}$", "", "the applied pair block (8 x 8, antisymmetric) of the BdG matrix of a state with a "
     "small moving pair field", "new"),
    (G10, "$\\mathsf C^{\\rm p,tot}_{\\rm SF}$", "kap_tot", "the pair matrix of a uniform state summed over the momenta, "
     "$\\int\\frac{d^3q}{(2\\pi)^3}\\mathsf C^{\\rm p}_{\\rm LL}(\\vec q_{\\rm LL})$", "new"),
    (G10, "$\\mathcal X^{\\rm P}_{\\rm SF}$", "pair_response pair_response_nodes pair_response_py", "the static pair response: "
     "the linear map from the applied pair block $\\mathsf P^{\\rm a}_{\\rm SF}$ to the summed pair matrix of the thermal state", "new"),
    (G10, "$\\varepsilon^{\\rm A}_{{\\rm SF},a}$, $\\varepsilon^{\\rm D}_{{\\rm SF},b}$, $\\mathsf u_{{\\rm SF},a}$, $\\mathsf w_{{\\rm SF},b}$", "",
     "the eigenvalues and the normalized eigenvectors of the particle block and of the hole block of the BdG matrix of a "
     "moving pair at the relative momentum $\\vec q_{\\rm LL}$", "new"),
    (G10, "$\\mathcal F^X_{2,{\\rm SF}}$", "", "the free energy of a state with a small applied pair block, to second order in it, "
     "measured from the normal state at the same temperature", "new"),
    (G10, "$v^X_{\\rm SF}$", "vSF v_SF", "the scale of the breaking in method $X$, $2f^{X,{\\rm I}}_{\\rm SF}$: the analogue of the vacuum "
     "value $v$ of the Standard Model by the same convention ($M^X_{W,{\\rm SF}}=g^{\\rm v}_{\\rm LL}v^X_{\\rm SF}/2$); a scale "
     "of the new group, not the electroweak scale", "new"),
    (G10, "$\\lambda^{X,{\\rm H}}_{\\rm SF}$", "lambda_SF", "the quartic coupling of the amplitude mode of method $X$ in the "
     "convention of the Standard Model, $(M^{X,{\\rm H}}_{\\rm SF})^2/(2(v^X_{\\rm SF})^2)$", "new"),
    (G10, "$\\Gamma^{X,{\\rm H}}_{\\rm SF}$", "width_MH Gamma_over_M_H", "the estimate of the width of the amplitude mode of "
     "method $X$ from its decays into two Goldstone fields or two longitudinal gauge bosons, "
     "$3(M^{X,{\\rm H}}_{\\rm SF})^3/(32\\pi(f^{X,{\\rm F}}_{\\rm SF})^2)$", "new"),
    (G10, "$\\mathcal L^{\\rm CJT}$, $\\mathcal L^{\\rm HS}$, $\\mathcal L^{\\rm KS}$; $\\mathcal L^X_{\\rm kinetic}$, $\\mathcal L^X_{\\rm potential}$",
     "L_CJT L_HS L_KS L_kin L_pot", "the effective Lagrangian of the field $\\vec\\Phi^X_{\\rm SF}$ in method $X$, and its "
     "kinetic part (the terms with derivatives, to second order) and potential part (the author's names)", "author"),
    (G10, "$\\mathcal V^X_{\\rm SF}$", "V_eff_X", "the effective potential of method $X$, $-\\mathcal L^X_{\\rm potential}$: "
     "the energy per volume of the uniform state with the field $\\vec\\Phi^X_{\\rm SF}$, measured from the normal vacuum",
     "new"),
    (G10, "$\\kappa^{X,{\\rm t}}_{\\rm SF}$, $\\kappa^{X,{\\rm s}}_{\\rm SF}$, $\\kappa^{X,{\\rm h}}_{\\rm SF}$", "kappa_t kappa_s kappa_h",
     "the stiffness coefficients of $\\mathcal L^X_{\\rm kinetic}$: of the time derivatives and of the space derivatives "
     "of the directions in which the field turns (phase and isospin), and of the derivatives of its modulus",
     "LL-renamed:$Z^{\\rm t}_{\\rm LL}$, $Z^{\\rm s}_{\\rm LL}$"),
    (G10, "$\\Delta^X_{\\rm SF}$", "Delta_X", "the vacuum value of method $X$: the modulus of $\\vec\\Phi^X_{\\rm SF}$ at the "
     "minima of $\\mathcal V^X_{\\rm SF}$", "new"),
    (G10, "$\\mathrm{SFBSv}^X$", "SFBSv", "the superfluid solution of method $X$ expanded about its vacuum (the author's "
     "name)", "author"),
    (G10, "$h^X_{\\rm SF}$; $\\vartheta^X_{{\\rm SF},0}$, $\\vartheta^X_{{\\rm SF},1}$, $\\vartheta^X_{{\\rm SF},2}$",
     "h_X th0 th1 th2", "the amplitude (Higgs) field and the three Goldstone fields of $\\mathrm{SFBSv}^X$: the phase of the "
     "fermion number and the angles of the isospin rotations about the first two axes", "new"),
    (G10, "$\\mathcal M^{X,{\\rm vac}}_{\\rm SF}$, $H^{\\rm vac}_{\\rm SF}$", "M_vac H_vac", "the manifold of vacua of method "
     "$X$ (the minima of $\\mathcal V^X_{\\rm SF}$) and the stabilizer of a chosen vacuum in GEE", "new"),
    (G10, "$M^{X,{\\rm H}}_{\\rm SF}$", "M_H", "the mass of the amplitude (Higgs) mode of the superfluid in method $X$",
     "new"),
    (G10, "$M^{X,{\\rm NU}}_{\\rm SF}$", "M_NU", "the mass of the two non-unitary modes of the superfluid in method $X$ "
     "(the oscillations of the imaginary parts of the components 1 and 2 of the order parameter of a polar state along the "
     "third isospin axis)", "new"),
    # ------------------------------------------------------------------ group 11
    (G11, "$\\mathsf M^X_{\\rm SF}$", "Mmat", "the real symmetric $4\\times4$ mass matrix of the gauge fields "
     "$W^1_{{\\rm SF},p}$, $W^2_{{\\rm SF},p}$, $W^3_{{\\rm SF},p}$, $B_{{\\rm SF},p}$ in the vacuum of method $X$", "new"),
    (G11, "$\\mathsf q^a_{\\rm SF}$, $\\mu^a_{\\rm SF}$, $\\chi^{ab}_{\\rm SF}$", "q_a mu_a chi_ab", "the charge matrices "
     "of the four gauge fields ($a=1,2,3$: $\\mathsf 1_4\\otimes\\tfrac12\\tau^{\\rm LL}_a$; $a={\\rm F}$: $\\mathsf 1_8$), the "
     "chemical potentials of the charges $\\Psi^\\dagger_{\\rm LL}\\mathsf q^a_{\\rm SF}\\Psi_{\\rm LL}$, and the static "
     "susceptibilities $\\chi^{ab}_{\\rm SF}=\\partial\\langle\\Psi^\\dagger_{\\rm LL}\\mathsf q^a_{\\rm SF}\\Psi_{\\rm LL}\\rangle/\\partial\\mu^b_{\\rm SF}$ "
     "at $\\mu^b_{\\rm SF}=0$", "new"),
    (G11, "$s^{\\rm g}_{\\rm SF}$", "s_g", "a common factor of the two gauge couplings, $g^{\\rm v}_{\\rm LL}=0.7\\,s^{\\rm g}_{\\rm SF}$ "
     "and $g^{\\rm F}_{\\rm LL}=0.45\\,s^{\\rm g}_{\\rm SF}$", "new"),
    (G11, "$M^X_{W,{\\rm SF}}$, $M^X_{Z,{\\rm SF}}$, $M^X_{A,{\\rm SF}}$", "M_W M_Z M_A", "the masses of $W^\\pm_{{\\rm SF},p}$, "
     "$Z_{{\\rm SF},p}$ and $A_{{\\rm SF},p}$ in method $X$", "new"),
    # ------------------------------------------------------------------ group 12
    (G12, "$\\ell^{\\rm lat}_{\\rm SF}$, $N^{\\rm lat}_{\\rm SF}$", "dx_lat N_lat", "the spacing and the number of sites of "
     "a lattice in one space direction", "new"),
    (G12, "$\\gamma^X_{\\rm SF}$", "gamma_RPA gamma_TD", "the growth rate of the unstable pair mode of the normal vacuum in "
     "method $X$: the number $\\gamma>0$ at which an eigenvalue of $\\mathsf Q^X_{\\rm SF}(i\\gamma)$ equals 1; a fluctuation of the "
     "normal vacuum in the pair channel grows as $e^{\\gamma^X_{\\rm SF}t}$", "new"),
    (G12, "$\\mathsf D^{\\rm sp}_{\\rm SF}$", "dmatrix", "the Fourier differentiation matrix of a periodic grid of an odd number "
     "$N^{\\rm lat}_{\\rm SF}$ of points with the spacing $\\ell^{\\rm lat}_{\\rm SF}$: the derivative, at the points, of the "
     "trigonometric polynomial through the values of a function at the points", "new"),
    (G12, "$r^{\\rm W}_{\\rm SF}$", "r_W", "the Wilson parameter of the lattice: the coefficient of the Wilson term "
     "$\\beta_{\\rm LL}(r^{\\rm W}_{\\rm SF}/2\\ell^{\\rm lat}_{\\rm SF})[2\\Psi(x)-\\Psi(x+\\ell^{\\rm lat}_{\\rm SF})-\\Psi(x-\\ell^{\\rm lat}_{\\rm SF})]$ "
     "(value 1)", "new"),
    # ------------------------------------------------------------------ group 13
    (G13, "$D^{\\rm SM}_\\mu$; $\\Phi^{\\rm SM}$, $\\phi^+$, $\\phi^0$", "", "the covariant derivative of the Higgs doublet "
     "of the Standard Model, the doublet and its two components", "SM"),
    (G13, "$W^a_\\mu$, $B_\\mu$; $g$, $g'$; $\\tau^a$, $T^{a,{\\rm SM}}$; $Y^{\\rm SM}$, $Q^{\\rm SM}$", "",
     "the gauge fields of $SU(2)_L$ and $U(1)_Y$ of the Standard Model, their couplings, the Pauli matrices and generators, "
     "the hypercharge and the electric charge ($a=1,2,3$; $\\mu=0,\\dots,3$)", "SM"),
    (G13, "$W^\\pm_\\mu$, $Z_\\mu$, $A_\\mu$, $\\theta_W$, $e$, $v$", "", "the mass eigenstates of the Standard Model, the "
     "weak mixing angle, the elementary charge and the vacuum value of the Higgs field", "SM"),
    (G13, "$\\mu^{\\rm SM}$, $\\lambda^{\\rm SM}$", "", "the two coefficients of the Higgs potential "
     "$-(\\mu^{\\rm SM})^2(\\Phi^{\\rm SM})^\\dagger\\Phi^{\\rm SM}+\\lambda^{\\rm SM}((\\Phi^{\\rm SM})^\\dagger\\Phi^{\\rm SM})^2$ of the "
     "Standard Model", "SM"),
    (G13, "$M_W$, $M_Z$", "", "the masses of $W^\\pm_\\mu$ and $Z_\\mu$ in the Standard Model, $gv/2$ and $\\sqrt{g^2+g'^2}\\,v/2$", "SM"),
    (G13, "$m_H$, $\\Gamma_H$, $\\lambda_H$, $G_F$", "SM_MH SM_V SM_MW SM_MZ", "the mass and the total width of the Higgs boson of the "
     "Standard Model, its quartic coupling $m_H^2/(2v^2)$, and the Fermi constant ($v=(\\sqrt2G_F)^{-1/2}$)", "SM"),
    (G13, "$SU(2)_L$, $U(1)_Y$", "", "the electroweak gauge group of the Standard Model", "SM"),
    (G13, "$\\hat\\theta_W$, $\\hat\\alpha(M_Z)$", "SM_s2w SM_alpha", "the weak mixing angle and the fine-structure constant "
     "in the $\\overline{\\rm MS}$ scheme at the scale $M_Z$, the inputs of the couplings $g=e/\\sin\\hat\\theta_W$ and "
     "$g'=e/\\cos\\hat\\theta_W$ with $e^2=4\\pi\\hat\\alpha(M_Z)$", "SM"),
    # ------------------------------------------------------------------ group 14 (Revision_SM_Higgs)
    (G14, "$\\chi_{\\rm SH}$", "chi_SH", "the singlet field of the revised hypotheses: a four-component Dirac field of "
     "$SL(2,\\mathbb C)$, a singlet of $SU(2)_L$ and of colour, with the hypercharge $Y^{\\rm SM}=1/4$, the fermion number 1 and "
     "the electric charge $1/4$; four complex Grassmann (anticommuting) operator components", "new-SH"),
    (G14, "$m^\\chi_{\\rm SH}$", "m_chi", "the mass of the singlet field $\\chi_{\\rm SH}$", "new-SH"),
    (G14, "$\\Pi^{\\rm u}_{\\rm SH}$, $\\Pi^{\\rm d}_{\\rm SH}$", "Pi_u Pi_d", "the pair operators of the doublet-singlet channel: "
     "$\\Pi^{\\rm u}_{\\rm SH}$ annihilates an isospin-up particle of $\\Psi_{\\rm LL}$ and a particle of $\\chi_{\\rm SH}$ in the "
     "spin-zero state built with the matrix $C_{\\rm LL}\\gamma^{\\rm ch}_{\\rm LL}$ (a Lorentz scalar of positive parity), "
     "$\\Pi^{\\rm d}_{\\rm SH}$ the same with an isospin-down particle; together an $SU(2)_L$ doublet with $Y^{\\rm SM}=1/2$",
     "new-SH"),
    (G14, "$G_{\\rm SH}$", "G_SH", "the coupling of the attractive pairing term "
     "$-G_{\\rm SH}[(\\Pi^{\\rm u}_{\\rm SH})^\\dagger\\Pi^{\\rm u}_{\\rm SH}+(\\Pi^{\\rm d}_{\\rm SH})^\\dagger\\Pi^{\\rm d}_{\\rm SH}]$ "
     "of the revised hypotheses, the only pairing term of the interaction", "new-SH"),
    (G14, "$\\Phi_{\\rm SH}$; $\\Phi^{\\rm u}_{\\rm SH}$, $\\Phi^{\\rm d}_{\\rm SH}$", "Phi_SH Phi_u Phi_d", "the pair field of the "
     "superfluid of the revised hypotheses and its two components, the expectation values of the pair operators "
     "$\\Pi^{\\rm u}_{\\rm SH}$, $\\Pi^{\\rm d}_{\\rm SH}$ in the state: a Lorentz scalar, a colour singlet, an $SU(2)_L$ doublet with "
     "$Y^{\\rm SM}=+1/2$, whose components have the electric charges $+1$ and $0$; the quantum numbers of the Higgs doublet "
     "$\\Phi^{\\rm SM}$", "new-SH"),
    (G14, "$v_{\\rm SH}$", "v_SH", "the scale of the breaking of $SU(2)_L\\times U(1)_Y$ by the superfluid, defined by the mass "
     "$M_W=g\\,v_{\\rm SH}/2$ that the superfluid gives the $W$ boson (the definition that the Standard Model gives $v$); identified with "
     "$v$ to fix the scale of the model in GeV", "new-SH"),
]
