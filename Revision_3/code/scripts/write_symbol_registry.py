#!/usr/bin/env python3
"""
write_symbol_registry.py (Revision_3) -- the registry of every symbol that the documents
of the folder Revision_3 use.

Hard Rule 5 of the project requires that every symbol denote ONE object in the whole
project.  This registry has two kinds of entries:

  * TAKEN: symbols of the MASTER registries of the project (the registry of the main
    project, ../provenance/symbols.csv, and the MASTER registry of the revision on the
    deflating spacetimes, ../Revision_Deflation_CJT_and_Kohn-Sham/provenance/symbols_master.csv)
    that Revision_3 uses with exactly the same meaning.  Their symbol text and meaning are
    copied from the master file, so that they cannot drift;
  * NEW: the symbols that Revision_3 introduces.  Every one of them carries the suffix U2
    (the name of the new gauge group), or is a plain symbol that no master registry
    contains (checked by code/scripts/check_symbols.py).

Output:
    provenance/symbols_r3.csv                  the registry (machine-readable)
    provenance/generated/symbols_r3.md         the registry as a Markdown table and as
                                               LaTeX table rows (fragments for the documents)

Run from the folder Revision_3:
    python3 code/scripts/write_symbol_registry.py
    python3 code/scripts/check_symbols.py
"""
import csv
import os

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MAIN = os.path.dirname(ROOT)
MASTER_MAIN = os.path.join(MAIN, "provenance", "symbols.csv")
MASTER_DEFL = os.path.join(MAIN, "Revision_Deflation_CJT_and_Kohn-Sham", "provenance", "symbols_master.csv")

G1 = "1 Spacetime, coordinates and indices"
G2 = "2 Matrices of the 7+1 spacetime"
G3 = "3 The new gauge sector: fields, group, covariant derivative"
G4 = "4 Pairs, couplings, masses and Lagrangian"
G5 = "5 Mean-field theory and spinor solutions"
G6 = "6 Effective potential and its minima"
G7 = "7 Collective modes, stiffness and boson masses"
G8 = "8 Numbers, regularizations and labels"
G9 = "9 The electroweak theory as the template (suffix SM)"

# (group, symbol text exactly as in the master file, which master file)
TAKEN = [
    (G1, r"$X=\{x0,\dots,x7\}$", "main"),
    (G1, r"$A,B,C,D$", "main"),
    (G1, r"$i,j$", "main"),
    (G1, r"$D$", "defl"),
    (G1, r"$K$, $J$ (index sets); $r$, $r'$ (ranks)", "defl"),
    (G2, r"$\eta_{1671}$", "main"),
    (G2, r"$\gamma_{1671}^A$", "main"),
    (G2, r"$\Sigma_{1671}^{AB}$", "main"),
    (G2, r"$\omega_{1671}$", "main"),
    (G2, r"$\chi_{1671}$", "main"),
    (G2, r"$P_{1671}^{L}$, $P_{1671}^{R}$", "main"),
    (G2, r"$C_{1671}^{+}$, $C_{1671}^{-}$", "main"),
    (G2, r"$\beta_{1671}$, $\alpha_{1671}^j$", "main"),
]

# Notes on the use in Revision_3 of a taken symbol (appended to the master meaning).
USE_NOTE = {
    r"$K$, $J$ (index sets); $r$, $r'$ (ranks)": r"In Revision\_3 only $K$ and $r$ are used, for the ordered products $\mathsf G_K$; the Fierz matrix does not occur.",
    r"$A,B,C,D$": r"In Revision\_3 only the letters $A$ and $C$ are used as indices; $B_{U2A}$ is the $U(1)_{U2}$ gauge field.",
    r"$i,j$": r"In Revision\_3 only $j$ (and a second such index $j'$) is used as an index, and sums over $j$ run over all seven space-like coordinates $0,1,2,3,5,6,7$; the letter $i$ is the imaginary unit.",
    r"$D$": r"In Revision\_3 $D=7$ ($A_7=16\pi^3/15$), and $A_8=\pi^4/3$ is the area of the unit sphere of the Euclidean 8-momentum.",
}

# (group, symbol, names in programs and data files of Revision_3, meaning (LaTeX, complete in itself))
NEW = [
    (G1, r"$\vec x$", "",
     r"the seven space-like coordinates $(x0,x1,x2,x3,x5,x6,x7)$ as one vector; $\vec q\cdot\vec x=\sum_jq^jx^j$"),
    (G1, r"$l$, $l'$", "",
     r"indices of the components of a spinor (values $1,\dots,16$) or of the doublet $\Psi_{U2}$ (values $1,\dots,32$); the summation index of the closed form of the Fierz matrix, which the master registry also calls $l$, does not occur in Revision\_3"),
    (G1, r"$\delta^{jj'}$", "",
     r"the Kronecker symbol: 1 if $j=j'$, 0 otherwise"),
    (G1, r"$\varrho_{U2}$", "",
     r"the distance between two static sources in the seven-dimensional space"),
    (G2, r"$\mathsf G_K$", "",
     r"the ordered product $\gamma^{A_1}_{1671}\gamma^{A_2}_{1671}\cdots\gamma^{A_r}_{1671}$ over an index set $K=(A_1<\dots<A_r)$ "
     r"($\mathsf G_K=\mathsf 1_{16}$ for the empty set); the 256 matrices $\mathsf G_K$ are a basis of the real $16\times16$ matrices"),
    (G2, r"$\mathsf 1_2$, $\mathsf X$, $\mathsf Z$, $\mathsf E$", "1 X Z E",
     r"the real $2\times2$ matrices $\mathsf 1_2=\mathrm{diag}(1,1)$, $\mathsf X=\begin{pmatrix}0&1\\1&0\end{pmatrix}$, "
     r"$\mathsf Z=\begin{pmatrix}1&0\\0&-1\end{pmatrix}$, $\mathsf E=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$; each $\gamma^A_{1671}$ is $\pm$ a "
     r"Kronecker product of four of them, and on the doublet index $\mathsf E=i\tau^2_{U2}$ is the antisymmetric matrix of the spin-zero pair "
     r"(the columns f1 to f4 of r3\_gamma\_kron.csv hold the letters 1, X, Z, E)"),
    (G2, r"$\mathsf 1_{16}$, $\mathsf 1_{32}$", "",
     r"the unit matrices of the 16 spinor components and of the 32 components of the doublet (or of the Nambu spinor)"),
    (G2, r"$\mathsf K$", "",
     r"a real $16\times16$ matrix that defines the bilinear form $\psi^{1T}_{U2}\mathsf K\psi^2_{U2}$ of two spinors (Sec.\ on the pairs)"),
    (G3, r"$\Psi_{U2}$; $\psi^1_{U2}$, $\psi^2_{U2}$", "",
     r"the doublet of the new gauge theory: two quantized 16-component Dirac fields $\psi^1_{U2}$, $\psi^2_{U2}$ on the 7+1 spacetime whose "
     r"components are complex Grassmann operators; $\Psi_{U2}=(\psi^1_{U2},\psi^2_{U2})$, 32 components, doublet index outside"),
    (G3, r"$\bar\Psi_{U2}$; $\bar\psi^1_{U2}$, $\bar\psi^2_{U2}$", "",
     r"the Dirac adjoint taken member by member, $\bar\Psi_{U2}=(\bar\psi^1_{U2},\bar\psi^2_{U2})=(\psi^{1\dagger}_{U2}\beta_{1671},\ \psi^{2\dagger}_{U2}\beta_{1671})$"),
    (G3, r"$\tilde\Psi_{U2}$", "",
     r"the conjugate doublet $\mathsf E\,\Psi^*_{U2}=(\psi^{1*}_{U2},\psi^{2*}_{U2})$ multiplied by $\mathsf E$, i.e.\ $(\psi^{2*}_{U2},-\psi^{1*}_{U2})$; since the $\gamma^A_{1671}$ are real, $\Psi^*_{U2}$ "
     r"transforms under Spin(7,1) like $\Psi_{U2}$, and $\tilde\Psi_{U2}$ transforms under $SU(2)_{U2}$ like $\Psi_{U2}$, with $U(1)_{U2}$ charge $-y_{U2}$"),
    (G3, r"$\tau^1_{U2}$, $\tau^2_{U2}$, $\tau^3_{U2}$; $\boldsymbol\tau_{U2}$", "",
     r"the Pauli matrices $\begin{pmatrix}0&1\\1&0\end{pmatrix}$, $\begin{pmatrix}0&-i\\i&0\end{pmatrix}$, $\begin{pmatrix}1&0\\0&-1\end{pmatrix}$ "
     r"acting on the doublet index; $\boldsymbol\tau_{U2}=(\tau^1_{U2},\tau^2_{U2},\tau^3_{U2})$; the three matrices $\tfrac12\tau^1_{U2}$, $\tfrac12\tau^2_{U2}$, $\tfrac12\tau^3_{U2}$ generate $SU(2)_{U2}$; "
     r"$\boldsymbol\tau^{(1)}_{U2}$, $\boldsymbol\tau^{(2)}_{U2}$: the same matrices acting on the first and on the second of two static sources"),
    (G3, r"$U(2)_{U2}$; $SU(2)_{U2}$, $U(1)_{U2}$", "",
     r"the new gauge group $U(2)_{U2}=(SU(2)_{U2}\times U(1)_{U2})/\mathbb Z_2$ acting on the doublet index: $SU(2)_{U2}$ by unitary $2\times2$ "
     r"matrices of determinant 1, $U(1)_{U2}$ by the phases $\exp(i y_{U2}\Theta^0_{U2})$; it commutes with Spin(7,1)"),
    (G3, r"$y_{U2}$", "Y",
     r"the $U(1)_{U2}$ charge of $\Psi_{U2}$, $y_{U2}=\tfrac12$; the pairs $\Phi^\pm_{U2}$ carry $2y_{U2}=1$"),
    (G3, r"$g_{U2}$, $g'_{U2}$", "G GP",
     r"the gauge couplings of $SU(2)_{U2}$ and $U(1)_{U2}$; on the 7+1 spacetime of dimension mass$^{-2}$"),
    (G3, r"$\boldsymbol W_{U2A}=(W^1_{U2A},W^2_{U2A},W^3_{U2A})$", "w",
     r"the $SU(2)_{U2}$ gauge field, a triple of real vector fields"),
    (G3, r"$B_{U2A}$", "b",
     r"the $U(1)_{U2}$ gauge field, a real vector field"),
    (G3, r"$\boldsymbol W_{U2AC}$, $B_{U2AC}$", "",
     r"the field strengths $\boldsymbol W_{U2AC}=\partial_A\boldsymbol W_{U2C}-\partial_C\boldsymbol W_{U2A}-g_{U2}\,\boldsymbol W_{U2A}\times\boldsymbol W_{U2C}$ "
     r"($\times$ the cross product of triples) and $B_{U2AC}=\partial_AB_{U2C}-\partial_CB_{U2A}$"),
    (G3, r"$D_{U2A}$", "",
     r"the new covariant derivative $D_{U2A}\Psi_{U2}=(\partial_A+\tfrac i2 g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+i g'_{U2}y_{U2}B_{U2A})\Psi_{U2}$; "
     r"it acts on the doublet index, on each of the 16 spinor components alike"),
    (G3, r"$\mathcal W_{U2A}$, $\mathcal F_{U2AC}$", "cal_w",
     r"the connection as a Hermitian $2\times2$ matrix, $\mathcal W_{U2A}=\tfrac12g_{U2}\boldsymbol\tau_{U2}\cdot\boldsymbol W_{U2A}+g'_{U2}y_{U2}B_{U2A}\mathsf 1_2$ "
     r"($D_{U2A}=\partial_A+i\mathcal W_{U2A}$), and its field strength $\mathcal F_{U2AC}=\partial_A\mathcal W_{U2C}-\partial_C\mathcal W_{U2A}+i[\mathcal W_{U2A},\mathcal W_{U2C}]$"),
    (G3, r"$\Upsilon_{U2}$; $\boldsymbol\Theta_{U2}$, $\Theta^0_{U2}$", "ups theta theta0",
     r"a gauge transformation $\Upsilon_{U2}=\exp(\tfrac i2\boldsymbol\Theta_{U2}\cdot\boldsymbol\tau_{U2})\exp(iy_{U2}\Theta^0_{U2})$, with a triple "
     r"$\boldsymbol\Theta_{U2}(X)$ and a function $\Theta^0_{U2}(X)$ of the coordinates"),
    (G3, r"$J^A_{U2}$, $\boldsymbol J^A_{U2}$", "",
     r"the $U(1)_{U2}$ current $\bar\Psi_{U2}\gamma^A_{1671}\Psi_{U2}$ and the $SU(2)_{U2}$ currents $\bar\Psi_{U2}\gamma^A_{1671}\tfrac12\boldsymbol\tau_{U2}\Psi_{U2}$"),
    (G4, r"$\Phi^{+}_{U2}$, $\Phi^{-}_{U2}$", "",
     r"the spin-zero pairs $\psi^{1T}_{U2}C^\pm_{1671}\psi^2_{U2}=\tfrac12\Psi^T_{U2}(\mathsf E\otimes C^\pm_{1671})\Psi_{U2}$; $\Phi^+_{U2}$ is a Lorentz scalar "
     r"whose members have spin zero and orbital angular momentum zero in the rest frame (the singlet of total spin zero of the task); $\Phi^-_{U2}$, its "
     r"pseudoscalar partner, vanishes in the non-relativistic limit; both are $SU(2)_{U2}$ singlets of $U(1)_{U2}$ charge 1"),
    (G4, r"$m_{U2}$", "m",
     r"the bare Dirac mass of each member of the doublet"),
    (G4, r"$G^S_{U2}$, $G^P_{U2}$", "gs gp",
     r"the quartic couplings of $(\bar\Psi_{U2}\Psi_{U2})^2$ and of $\Phi^{+\dagger}_{U2}\Phi^+_{U2}$ in the Lagrangian; positive values attract; dimension mass$^{-6}$"),
    (G4, r"$\mathcal L_{U2}$", "",
     r"the Lagrangian density of the model"),
    (G4, r"$H_{U2}$, $N_{U2}$", "",
     r"the Hamiltonian and the fermion-number operator (charge-symmetric ordering); the ground state minimizes $H_{U2}-\mu_{U2}N_{U2}$"),
    (G4, r"$\mathcal L^{\rm MF}_{U2}$, $H^{\rm MF}_{U2}$", "",
     r"the Lagrangian and the Hamiltonian of the mean-field approximation, in which the squares of the fluctuations of $\bar\Psi_{U2}\Psi_{U2}$ and $\Phi^+_{U2}$ are dropped"),
    (G4, r"$\mathrm{Vol}_{U2}$", "",
     r"the volume of the seven-dimensional space (a large box with periodic boundary conditions; at the end $\mathrm{Vol}_{U2}\to\infty$)"),
    (G5, r"$\mu_{U2}$", "mu",
     r"the chemical potential conjugate to $N_{U2}$"),
    (G5, r"$M_{U2}$", "M mass M_sym",
     r"the effective mass $m_{U2}-2G^S_{U2}\langle\bar\Psi_{U2}\Psi_{U2}\rangle$ (M\_sym: its value on the lowest state with $\Delta_{U2}=0$)"),
    (G5, r"$\Delta_{U2}$; $\Delta_1$, $\Delta_2$", "Delta Delta_R7 Delta_R8 Delta_chiral_R7 Landau_R7 Landau_R8 Delta1 Delta2 D050 D080 D095 D105 lgD050 lgD080 lgD095 lgD105 lnDelta",
     r"the pair field $\Delta_{U2}=G^P_{U2}\langle\Phi^+_{U2}\rangle$, complex, the order parameter of the breaking of $U(2)_{U2}$; $\Delta_1$, $\Delta_2$ "
     r"its real and imaginary parts (in the data files: its value at the minimum; lg: $\log_{10}$, ln: natural logarithm, Landau: the small-gap form)"),
    (G5, r"$s_{U2}$", "s",
     r"the sum $M_{U2}^2+\lvert\Delta_{U2}\rvert^2$, on which the vacuum spectrum depends"),
    (G5, r"$\vec q$, $q$, $q^j$", "q",
     r"the spatial momentum of a fermion mode (seven components $q^j$, $j=0,1,2,3,5,6,7$) and its length"),
    (G5, r"$\psi^1_{U2}(\vec q)$, $\psi^2_{U2}(\vec q)$", "",
     r"the Fourier components $\mathrm{Vol}_{U2}^{-1/2}\int d^7x\,\exp(-i\vec q\cdot\vec x)\,\psi^{1}_{U2}$ (and the same for $\psi^2_{U2}$) at a fixed time $x4$"),
    (G5, r"$q^F_{U2}$", "",
     r"the Fermi momentum $\sqrt{\mu_{U2}^2-M_{U2}^2}$"),
    (G5, r"$\vec Q_{U2}$", "",
     r"the pair momentum of a condensate $\Delta_{U2}\exp(i\vec Q_{U2}\cdot\vec x)$"),
    (G5, r"$\varepsilon_{U2}$", "",
     r"the single-particle energy $\sqrt{q^2+M_{U2}^2}$"),
    (G5, r"$\mathsf h_{U2}$", "",
     r"the $16\times16$ Hamiltonian matrix $\alpha^j_{1671}q^j+\beta_{1671}M_{U2}$ (sum over the seven space-like $j$), eigenvalues $\pm\varepsilon_{U2}$, each 8-fold"),
    (G5, r"$\mathsf e^{+}_{U2}$, $\mathsf e^{-}_{U2}$", "",
     r"real orthonormal eigenvectors of $\mathsf h_{U2}(\vec q)$ with the eigenvalues $+\varepsilon_{U2}$ and $-\varepsilon_{U2}$ (eight of each sign)"),
    (G5, r"$\mathcal N_{U2}(\vec q)$", "",
     r"the Nambu spinor $(\psi^1_{U2}(\vec q),\ \psi^{2}_{U2}(-\vec q)^{\dagger T})$: 32 components, the annihilators of $\psi^1_{U2}$ at $\vec q$ and the creators of $\psi^2_{U2}$ at $-\vec q$"),
    (G5, r"$\mathsf H_{U2}$", "",
     r"the real symmetric $32\times32$ mean-field (Bogoliubov-de Gennes) matrix of $\mathcal N_{U2}(\vec q)$: "
     r"diagonal blocks $\mathsf h_{U2}-\mu_{U2}$ and $\alpha^j_{1671}q^j-\beta_{1671}M_{U2}+\mu_{U2}$, off-diagonal blocks $\Delta_{U2}C^+_{1671}$ (for real $\Delta_{U2}$)"),
    (G5, r"$\mathsf Y_{U2}$, $\mathsf Y'_{U2}$", "",
     r"the $32\times32$ matrices of the cross terms in the square of $\mathsf H_{U2}$, for the spin-zero pair ($\mathsf Y_{U2}$, square $\mu^2_{U2}\varepsilon^2_{U2}\mathsf 1_{32}$) and for the pseudoscalar pair ($\mathsf Y'_{U2}$, square $(\mu^2_{U2}\varepsilon^2_{U2}+M^2_{U2}\lvert\Delta_{U2}\rvert^2)\mathsf 1_{32}$)"),
    (G5, r"$E^{+}_{U2}$, $E^{-}_{U2}$", "E_Cplus E_Cminus",
     r"the quasi-particle energies $\sqrt{(\varepsilon_{U2}\pm\mu_{U2})^2+\lvert\Delta_{U2}\rvert^2}$ ($E^+_{U2}$: the branch of the antiparticles, $E^-_{U2}$: the branch of the particles; "
     r"E\_Cminus: the smaller energy of the pseudoscalar pairing)"),
    (G5, r"$\xi^{\pm}_{U2}$, $u^{\pm}_{U2}$, $v^{\pm}_{U2}$", "",
     r"$\xi^\pm_{U2}=\pm\varepsilon_{U2}-\mu_{U2}$ and the Bogoliubov coefficients $u^\pm_{U2}=[(1+\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$, $v^\pm_{U2}=[(1-\xi^\pm_{U2}/E^\mp_{U2})/2]^{1/2}$ "
     r"(note $E^\mp_{U2}=[(\xi^\pm_{U2})^2+\lvert\Delta_{U2}\rvert^2]^{1/2}$)"),
    (G5, r"$a_{U2}$, $d_{U2}$", "",
     r"for an eigenvector $\mathsf e^\pm_{U2}$ of $\mathsf h_{U2}(\vec q)$: the mode annihilators $a_{U2}=(\mathsf e^\pm_{U2})^T\psi^1_{U2}(\vec q)$ and $d_{U2}=(\beta_{1671}\mathsf e^\pm_{U2})^T\psi^2_{U2}(-\vec q)$"),
    (G5, r"$\lvert\mathrm{SF}_{U2}\rangle$, $b^\dagger_{U2}$, $\lvert\varnothing_{U2}\rangle$", "",
     r"the superfluid ground state; the operator $b^\dagger_{U2}=a^\dagger_{U2}d^\dagger_{U2}$ that creates one spin-zero pair of members of opposite momenta; "
     r"the state without any fermion in any mode (annihilated by every $a_{U2}$ and $d_{U2}$); the ground state is a product of factors $u^\pm_{U2}-v^\pm_{U2}b^\dagger_{U2}$ applied to it"),
    (G6, r"$\Omega_{U2}$; $\Omega^{\rm cl}_{U2}$", "Omega Omega_R7 Omega_R8 O050 O080 O095 O105 l095 l100 l102 l105 l110",
     r"the effective potential of the superfluid: the grand potential per unit volume at zero temperature as a function of $M_{U2}$ and "
     r"$\Delta_{U2}$ (data files: relative to the normal state of mass $m_{U2}$ at the same $\mu_{U2}$; lNNN: at $\lambda^P_{U2}=$ N.NN); "
     r"$\Omega^{\rm cl}_{U2}=(M_{U2}-m_{U2})^2/(4G^S_{U2})+\lvert\Delta_{U2}\rvert^2/G^P_{U2}$ its classical part"),
    (G6, r"$\Omega^{\min}_{U2}$, $\Omega^{\rm sym}_{U2}$", "Omega_sym",
     r"the value at the global minimum, and on the lowest state with $\Delta_{U2}=0$ ($U(2)_{U2}$ unbroken)"),
    (G6, r"$\Lambda_{U2}$", "",
     r"the momentum cutoff of the model; every number of Revision\_3 is in units of it"),
    (G6, r"$G^{\rm crit}_{U2}$", "Gcrit_R7 Gcrit_R8 Gcrit_chiral_R7",
     r"the critical pair coupling of the vacuum: $90\pi^4\Lambda_{U2}^{-6}$ (R7), $288\pi^4\Lambda_{U2}^{-6}$ (R8); twice the value for the left-handed doublet"),
    (G6, r"$\lambda^P_{U2}$, $\lambda^S_{U2}$", "lambda inv_G",
     r"the dimensionless couplings $G^P_{U2}/G^{\rm crit}_{U2}$ and $4G^S_{U2}/G^{\rm crit}_{U2}$ (inv\_G: $1/G^P_{U2}$)"),
    (G6, r"$a^{(7)}_{U2}$, $a^{(8)}_{U2}$", "A7",
     r"$A_7/(2\pi)^7=1/(120\pi^4)$ and $A_8/(2\pi)^8=1/(768\pi^4)$: for an integrand that depends on the length $q$ only, $\int d^7q/(2\pi)^7(\dots)=a^{(7)}_{U2}\int_0^{\Lambda_{U2}}q^6(\dots)\,dq$, and the same in 8 dimensions with $\kappa_{U2}$"),
    (G6, r"$\kappa_{U2}$, $\kappa^{(4)}_{U2}$", "",
     r"the length of the Euclidean 8-momentum, whose components are the seven components $q^j$ and the imaginary-time frequency $\kappa^{(4)}_{U2}$, in the Lorentz-invariant regularization (R8); $\kappa^2_{U2}=q^2+(\kappa^{(4)}_{U2})^2$"),
    (G6, r"$L_{U2}$", "",
     r"the function $L_{U2}(s_{U2})=2\int_0^1\kappa_{U2}^7\ln(\kappa_{U2}^2+s_{U2})\,d\kappa_{U2}$ of the vacuum potential of regularization (R8) (units $\Lambda_{U2}=1$)"),
    (G6, r"$\mathcal J^{(\iota)}_{U2}$; $\iota$", "",
     r"the radial integrals $\mathcal J^{(\iota)}_{U2}(s_{U2})=\int_0^1q^\iota(q^2+s_{U2})^{-1/2}dq$ of the vacuum potential of regularization (R7), for the even exponents $\iota=0,2,4,6,8$"),
    (G6, r"$n_{U2}$", "n n050 n080 n095 n105 n_over_4mu",
     r"the number density $-\partial\Omega_{U2}/\partial\mu_{U2}$ of the fermions (charge-symmetric: zero in the vacuum)"),
    (G6, r"$z_{U2}$", "dlnDelta_dmu Delta_dlnDelta_dmu",
     r"$\Delta_{U2}\,d\ln\Delta_{U2}/d\mu_{U2}$ along the minimum: how fast the condensation energy changes with $\mu_{U2}$"),
    (G7, r"$\sigma_{U2}$, $m_{\sigma U2}$", "msigma msigma_over_2Delta threshold kind",
     r"the amplitude mode of the pair field about the minimum: a real scalar boson, neutral under every gauge generator, that exists only after "
     r"the breaking; its mass (threshold: twice the smallest quasi-particle energy; kind 0: below it, 1: at it)"),
    (G7, r"$\theta_{U2}$", "",
     r"the phase of the pair field; its fluctuation is the Goldstone mode, which $B_{U2A}$ absorbs"),
    (G7, r"$f^s_{U2}$, $f^t_{U2}$, $f_{U2}$", "fs2 ft2 ft2_fixed ft2_relaxed f2_R8 fs2_over_n4mu",
     r"the stiffness of the phase: the coefficients of $\tfrac12\sum_j(\partial_j\theta_{U2})^2$ and $\tfrac12(\partial_4\theta_{U2})^2$ in the effective Lagrangian; "
     r"in the Lorentz-invariant regularization (R8) both equal $f_{U2}$ (data files: their squares)"),
    (G7, r"$\mathcal L^{\rm eff}_{U2}$", "",
     r"the effective Lagrangian of the phase $\theta_{U2}$ and of $B_{U2A}$ at long wavelengths"),
    (G7, r"$m_{BU2}$", "",
     r"the mass $g'_{U2}f_{U2}$ of $B_{U2A}$ after the breaking (the pair has the charge 1)"),
    (G7, r"$v^G_{U2}$", "vG_R7 vG2",
     r"the velocity $f^s_{U2}/f^t_{U2}$ of the Goldstone mode if $U(1)_{U2}$ were not gauged (vG2: its square)"),
    (G7, r"$w_{U2}$", "",
     r"the frequency of a small oscillation of the composite fields (time dependence $\exp(-iw_{U2}x4)$)"),
    (G7, r"$\mathsf V^1_{U2}$, $\mathsf V^2_{U2}$, $\mathsf V^3_{U2}$", "",
     r"the $32\times32$ vertices of $\Delta_1$, $\Delta_2$ and $M_{U2}$ in $\mathsf H_{U2}$: $\mathsf V^1_{U2}=\begin{pmatrix}0&C^+_{1671}\\C^+_{1671}&0\end{pmatrix}$, "
     r"$\mathsf V^2_{U2}=i\begin{pmatrix}0&C^+_{1671}\\-C^+_{1671}&0\end{pmatrix}$, $\mathsf V^3_{U2}=\begin{pmatrix}\beta_{1671}&0\\0&-\beta_{1671}\end{pmatrix}$"),
    (G7, r"$\lvert\mathrm o\rangle_{U2}$, $\lvert\mathrm e\rangle_{U2}$; $\mathcal E^{\rm o}_{U2}$, $\mathcal E^{\rm e}_{U2}$", "",
     r"an occupied and an empty eigenvector of $\mathsf H_{U2}(\vec q)$ and their eigenvalues, $\mathcal E^{\rm o}_{U2}<0\le\mathcal E^{\rm e}_{U2}$"),
    (G7, r"$\Pi_{U2}$, $\mathsf R_{U2}$", "",
     r"the $3\times3$ polarization matrix of the fermions for the composite fields $(\Delta_1,\Delta_2,M_{U2})$ (entries numbered 1, 2, 3 in this order) and the matrix "
     r"$\mathsf R_{U2}(w_{U2})$ of the small oscillations, the second derivatives of $\Omega^{\rm cl}_{U2}$ plus $\Pi_{U2}(w_{U2})$ (random-phase approximation)"),
    (G7, r"$c^\sigma_{U2}$", "",
     r"the positive function $c^\sigma_{U2}(w_{U2})=16\int\frac{d^7q}{(2\pi)^7}(q^2+s_{U2})^{-1/2}\big[4(q^2+s_{U2})-w_{U2}^2\big]^{-1}$, with $[\mathsf R_{U2}(w_{U2})]_{11}=c^\sigma_{U2}(w_{U2})(4\lvert\Delta_{U2}\rvert^2-w_{U2}^2)$ in the vacuum without scalar coupling"),
    (G7, r"$\mathcal V^{\sigma}_{U2}$", "",
     r"the pair interaction carried by $\sigma_{U2}$, $-[\mathsf R_{U2}(w_{U2})^{-1}]_{11}$; at $w_{U2}=0$ it is $-[\partial^2\Omega_{U2}/\partial\Delta_1^2]^{-1}$"),
    (G7, r"$V^{\rm C}_{U2}$, $\phi^{\rm Y}_{U2}$, $V^{\rm pair}_{U2}$", "x phi singlet_after triplet_after singlet_before triplet_before",
     r"the potential $V^{\rm C}_{U2}(\varrho_{U2})=\Gamma_{\rm E}(5/2)/(4\pi^{7/2}\varrho_{U2}^5)$ of a massless vector boson between two unit charges in seven space dimensions; "
     r"the factor $\phi^{\rm Y}_{U2}(m_{BU2}\varrho_{U2})=\exp(-m_{BU2}\varrho_{U2})$ times $[(m_{BU2}\varrho_{U2})^2+3m_{BU2}\varrho_{U2}+3]/3$ by which the mass of $B_{U2A}$ multiplies it; "
     r"the static potential of two heavy members of the doublet (data files: x is $m_{BU2}\varrho_{U2}$)"),
    (G8, r"(R7), (R8)", "R7 R8",
     r"the two regularizations: (R7) a sharp cutoff on the length of the seven-dimensional spatial momentum; (R8) a sharp cutoff on the length "
     r"of the Euclidean 8-momentum (Lorentz invariant; used in the vacuum)"),
    (G8, r"$\Gamma_{\rm E}$", "",
     r"Euler's Gamma function; $\Gamma_{\rm E}(5/2)=3\sqrt\pi/4$"),
    (G8, r"$i$", "",
     r"the imaginary unit"),
    (G8, r"${}^T$, ${}^\dagger$, ${}^*$; $\otimes$; $[\ ,\ ]$, $\{\ ,\ \}$; $\langle\ \rangle$", "",
     r"transpose, Hermitian conjugate, complex conjugate (for operators: the Hermitian conjugate of each component); Kronecker product; "
     r"commutator and anticommutator; expectation value in the ground state"),
    (G9, r"$\phi_{\rm SM}$, $Y_{\rm SM}$", "",
     r"the Higgs doublet of the electroweak theory and its hypercharge $Y_{\rm SM}=\tfrac12$ (quoted as the template of the new covariant derivative)"),
    (G9, r"$\mathrm D^{\rm SM}$, $g_{\rm SM}$, $g'_{\rm SM}$, $\boldsymbol\tau_{\rm SM}$, $\boldsymbol W_{\rm SM}$, $B_{\rm SM}$", "",
     r"the covariant exterior derivative of the electroweak theory, its two couplings, the Pauli matrices and the gauge fields (one-forms), "
     r"$\mathrm D^{\rm SM}\phi_{\rm SM}=(\mathrm d+\tfrac i2g_{\rm SM}\boldsymbol\tau_{\rm SM}\cdot\boldsymbol W_{\rm SM}+ig'_{\rm SM}Y_{\rm SM}B_{\rm SM})\phi_{\rm SM}$; "
     r"$\mathrm d$ is the exterior derivative"),
]


def read_master(path):
    with open(path, encoding="utf-8") as f:
        rows = list(csv.DictReader(line for line in f if not line.startswith("#")))
    return {r["symbol"]: r for r in rows}


def main():
    mm = read_master(MASTER_MAIN)
    md = read_master(MASTER_DEFL)
    out = []
    for group, sym, src in TAKEN:
        reg = mm if src == "main" else md
        if sym not in reg:
            raise SystemExit("taken symbol %r not found in the master registry %s" % (sym, src))
        r = reg[sym]
        meaning = r["meaning"]
        if sym in USE_NOTE:
            # the master meaning is a phrase that may end without a full stop
            meaning = meaning.rstrip()
            if not meaning.endswith("."):
                meaning += "."
            meaning = meaning + " " + USE_NOTE[sym]
        out.append({"group": group, "symbol": sym, "names_in_files": r["names_in_files"], "meaning": meaning,
                    "origin": "taken from the MASTER registry of the %s" % ("main project" if src == "main" else "revision on the deflating spacetimes")})
    for group, sym, names, meaning in NEW:
        out.append({"group": group, "symbol": sym, "names_in_files": names, "meaning": meaning, "origin": "new (Revision_3)"})
    # no symbol text twice
    seen = set()
    for r in out:
        if r["symbol"] in seen:
            raise SystemExit("symbol registered twice: %r" % r["symbol"])
        seen.add(r["symbol"])
    out.sort(key=lambda r: r["group"])
    os.makedirs(os.path.join(ROOT, "provenance", "generated"), exist_ok=True)
    with open(os.path.join(ROOT, "provenance", "symbols_r3.csv"), "w", encoding="utf-8", newline="") as f:
        f.write("# PROJECT spacetimes, Revision_3: registry of the symbols (written by code/scripts/write_symbol_registry.py)\n")
        w = csv.DictWriter(f, fieldnames=["group", "symbol", "names_in_files", "meaning", "origin"])
        w.writeheader()
        for r in out:
            w.writerow(r)
    # fragments: Markdown table (by group), LaTeX rows for the manuscript, counts
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
            md_lines.append("| symbol | meaning | origin | names in programs and data files |")
            md_lines.append("|-----------|-------------------------|----|----------|")
            tex_lines.append("\\multicolumn{3}{l}{\\textbf{%s}}\\\\" % group)
        if "|" in r["meaning"] or "|" in r["symbol"]:
            raise SystemExit("a vertical bar in the entry %r would break the Markdown table; write \\lvert, \\rvert" % r["symbol"])
        meaning = r["meaning"]
        md_names = " ".join("`%s`" % n for n in r["names_in_files"].split())
        md_lines.append("| %s | %s | %s | %s |" % (r["symbol"], meaning, "taken" if r["origin"].startswith("taken") else "new", md_names))
        names = " ".join("\\texttt{%s}" % n.replace("_", "\\_") for n in r["names_in_files"].split())
        tex_lines.append("%s & %s & %s\\\\" % (r["symbol"], r["meaning"], names))
    frag = []
    frag.append("<!-- BEGIN symbols_r3_table -->\n%s\n<!-- END symbols_r3_table -->" % "\n".join(md_lines))
    frag.append("<!-- BEGIN symbols_r3_texrows -->\n%s\n<!-- END symbols_r3_texrows -->" % "\n".join(tex_lines))
    frag.append("<!-- BEGIN symbols_r3_count -->\n%d\n<!-- END symbols_r3_count -->" % len(out))
    frag.append("<!-- BEGIN symbols_r3_count_taken -->\n%d\n<!-- END symbols_r3_count_taken -->" % len(TAKEN))
    frag.append("<!-- BEGIN symbols_r3_count_new -->\n%d\n<!-- END symbols_r3_count_new -->" % len(NEW))
    with open(os.path.join(ROOT, "provenance", "generated", "symbols_r3.md"), "w", encoding="utf-8") as f:
        f.write("\n\n".join(frag) + "\n")
    print("wrote provenance/symbols_r3.csv and provenance/generated/symbols_r3.md: %d entries (%d taken, %d new)" % (len(out), len(TAKEN), len(NEW)))


def tex_escape(s):
    """A meaning (plain text with a few formulas written in words) for a LaTeX table cell."""
    s = s.replace("\\", "\\textbackslash{}")
    for a, b in (("&", "\\&"), ("%", "\\%"), ("#", "\\#"), ("_", "\\_"), ("^", "\\^{}"), ("~", "\\~{}")):
        s = s.replace(a, b)
    return s


if __name__ == "__main__":
    main()
