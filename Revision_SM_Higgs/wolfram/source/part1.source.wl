(* ::Package:: *)

(* ::Chapter:: *)
(*Part I. The manifold M31, the spinor field, GEE, the field equations and four analytic solutions*)

(* ::Text:: *)
(*Part I treats items 1 to 5 of the problem (Sections 1 to 7 of the manuscript): the manifold M31 of the Lorentz super-loop, the spinor fields on it and the DiracLike field, the gauge group GEE and its covariant derivatives, the Lagrangian with its field equations, and analytic solutions of four simple cases. Its checks are those of the Python program `sf_part1.py`, which repeats a part of the checks of the Rust program `sf_part1`; the random test values of this notebook come from the generator of the Wolfram Language with the seed 20261121, so they differ from those of the two programs, as theirs differ from each other.*)

(* ::Section:: *)
(*I.1 Conventions, units and the common objects*)

(* ::Text:: *)
(***Units.** Planck's constant and the speed of light are 1; energies, masses and momenta are measured in units of the momentum cutoff $\Lambda_{\rm LL}=1$, lengths and times in units of $1/\Lambda_{\rm LL}$. The mass of the spinor field is $m_{\rm LL}=0.2$.*)

(* ::Text:: *)
(***Indices.** $p,p'$ run over the four coordinates of M31 (0 is the time), $j,j',j''$ over the three space coordinates, $\iota,\iota',\iota''$ over the three directions of isospin (an internal direction, not a direction in space), and $c,c'$ over the eight components of the spinor field. The letter $i$ is always the imaginary unit. The Minkowski metric is $\eta^{\rm LL}_{pp'}=\mathrm{diag}(1,-1,-1,-1)$.*)

(* ::Text:: *)
(***Matrices.** $\sigma^{\rm LL}_0$ is the $2\times2$ unit matrix and $\sigma^{\rm LL}_1,\sigma^{\rm LL}_2,\sigma^{\rm LL}_3$ are the Pauli matrices; acting on isospin they are written $\tau^{\rm LL}_\iota$. $\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2$. The Dirac matrices are those of the chiral basis, $\gamma^0_{\rm LL}$ with the blocks $\sigma^{\rm LL}_0$ off the diagonal and $\gamma^j_{\rm LL}$ with the blocks $\sigma^{\rm LL}_j$ and $-\sigma^{\rm LL}_j$; $\gamma^{\rm ch}_{\rm LL}=i\gamma^0_{\rm LL}\gamma^1_{\rm LL}\gamma^2_{\rm LL}\gamma^3_{\rm LL}=\mathrm{diag}(-1,-1,1,1)$, $C_{\rm LL}=i\gamma^2_{\rm LL}\gamma^0_{\rm LL}$, $\alpha^j_{\rm LL}=\gamma^0_{\rm LL}\gamma^j_{\rm LL}$ and $\beta_{\rm LL}=\gamma^0_{\rm LL}$. The spinor field $\Psi_{\rm LL}$ has eight components: the Dirac index $d$ (four values) and the isospin index $s$ (up, down); a $4\times4$ Dirac matrix $A$ acts on it as $A\otimes\mathsf 1_2$ and an isospin matrix $t$ as $\mathsf 1_4\otimes t$. The isospin generators are $T^\iota_{\rm LL}=\mathsf 1_4\otimes\tau^{\rm LL}_\iota/2$, the matrices of the three spin-zero pair fields are $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$, and the free Dirac Hamiltonian at the momentum $\vec q_{\rm LL}$ is $\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$.*)

(* ::Text:: *)
(***The author's conventions (the Standard Model).** The covariant derivative of the Higgs doublet is $D^{\rm SM}_\mu\Phi^{\rm SM}=\big(\partial_\mu+ig\sum_{a=1}^3\frac{\tau^a}2W^a_\mu+ig'Y^{\rm SM}B_\mu\big)\Phi^{\rm SM}$ with $Y^{\rm SM}=\tfrac12$ and $Q^{\rm SM}=T^{3,{\rm SM}}+Y^{\rm SM}$; the fields of definite mass are $W^\pm_\mu=(W^1_\mu\mp iW^2_\mu)/\sqrt2$, $Z_\mu=\cos\theta_WW^3_\mu-\sin\theta_WB_\mu$ and the photon $A_\mu=\sin\theta_WW^3_\mu+\cos\theta_WB_\mu$, with $e=g\sin\theta_W$; with $\langle\Phi^{\rm SM}\rangle=(0,v)/\sqrt2$ the masses are $M_W=gv/2$ and $M_Z=\sqrt{g^2+g'^2}\,v/2$, and the photon is massless. All GEE covariant derivatives of the solution use the same sign, $+i$ in front of every coupling. The author gave these conventions to establish terms, symbols and conventions for a new gauge-covariant derivative modelled after this one but not equal or equivalent to it: $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$ is a new gauge group, not the electroweak group, and the spinor field of the solution, the DiracLike spinor, has electric charge 0 and no electroweak quantum numbers. The solution borrows only the names: its charge $Q_{\rm LL}=T^3_{\rm LL}$ is not the electric charge, and its massless gauge boson, the zeugon, is not the photon.*)

(* ::Text:: *)
(*The next cell defines these matrices, the bi-quaternions and bi-octonions of Section I.2, the Lorentz matrices, the spinor matrices and the operators of the Fock space of one point. They are the same objects as those of the Rust library `sfcore` and of the Python module `sfcommon.py`, written here with the functions of the Wolfram Language.*)

(* ::Input:: *)
(* the mass of the spinor field, m_LL, in units of the cutoff Lambda_LL = 1 *)
mLL = 1/5;
(* sig[[p + 1]] is sigma^LL_p (p = 0, 1, 2, 3): sigma_0 is the 2 x 2 unit matrix, sigma_1, sigma_2, sigma_3 the Pauli matrices *)
sig = {IdentityMatrix[2], PauliMatrix[1], PauliMatrix[2], PauliMatrix[3]};
(* sigbar[[p + 1]] is sigma-bar_p: sigma_0 and -sigma_j, the blocks of the Dirac matrices below the diagonal *)
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
(* eLL is E_LL = i sigma_2 = {{0, 1}, {-1, 0}} *)
eLL = {{0, 1}, {-1, 0}};
(* etaLL is the Minkowski metric eta^LL = diag(1, -1, -1, -1) *)
etaLL = DiagonalMatrix[{1, -1, -1, -1}];
(* levi[a, b, c] is the Levi-Civita symbol epsilon^LL of the indices 1, 2, 3 *)
levi[a_, b_, c_] := Signature[{a, b, c}];
(* gam[p] is the Dirac matrix gamma^p_LL of the chiral basis: {{0, sigma_p}, {sigmabar_p, 0}} *)
gam[p_Integer] := ArrayFlatten[{{0, sig[[p + 1]]}, {sigbar[[p + 1]], 0}}];
(* gch is the chirality matrix gamma^ch_LL = i gamma^0 gamma^1 gamma^2 gamma^3 *)
gch = I gam[0] . gam[1] . gam[2] . gam[3];
(* cLL is the charge-conjugation matrix C_LL = i gamma^2 gamma^0 *)
cLL = I gam[2] . gam[0];
(* betaLL = gamma^0_LL and alphaLL[j] = gamma^0 gamma^j, the matrices of the Dirac Hamiltonian *)
betaLL = gam[0];
alphaLL[j_Integer] := gam[0] . gam[j];
(* d8[a] = a (x) 1_2 acts on the Dirac index of the eight components (component 2 d + s, d = 0..3, s = 0 up, 1 down) *)
d8[a_] := KroneckerProduct[a, IdentityMatrix[2]];
(* i8[t] = 1_4 (x) t acts on the isospin index *)
i8[t_] := KroneckerProduct[IdentityMatrix[4], t];
(* tLL[iota] is the isospin generator T^iota_LL = 1_4 (x) tau_iota/2 *)
tLL[iota_Integer] := i8[sig[[iota + 1]]/2];
(* kLL[iota] is the pair matrix K^iota_LL = C_LL gamma^ch_LL (x) E_LL tau_iota of the scalar spin-zero pairs *)
kLL[iota_Integer] := KroneckerProduct[cLL . gch, eLL . sig[[iota + 1]]];
(* hLL[q, m] is the free Dirac Hamiltonian h_LL(q) = sum_j alpha^j q_j + beta m on the eight components *)
hLL[q_List, m_] := d8[betaLL m + Sum[alphaLL[j] q[[j]], {j, 3}]];

(* ---- bi-quaternions: a bi-quaternion b = xi_0 e_0 + xi_1 e_1 + xi_2 e_2 + xi_3 e_3 is the list {xi_0, xi_1, xi_2, xi_3} ---- *)
(* qMul is the product of bi-quaternions by the rules (2.1) of the quaternion units *)
qMul[{a0_, a1_, a2_, a3_}, {b0_, b1_, b2_, b3_}] := {a0 b0 - a1 b1 - a2 b2 - a3 b3, a0 b1 + a1 b0 + a2 b3 - a3 b2,
   a0 b2 - a1 b3 + a2 b0 + a3 b1, a0 b3 + a1 b2 - a2 b1 + a3 b0};
(* qBar is the conjugation (the signs of xi_1, xi_2, xi_3 change); qDag the Hermitian conjugation *)
qBar[{a0_, a1_, a2_, a3_}] := {a0, -a1, -a2, -a3};
qDag[x_List] := Conjugate[qBar[x]];
(* qNorm is the complex norm N_LL(b) = sum_p xi_p^2 *)
qNorm[x_List] := Total[x^2];
(* mLLmat[b] is the matrix M_LL(b) = xi_0 sigma_0 - i sum_j xi_j sigma_j of (2.2), and biqOf its inverse map *)
mLLmat[b_List] := b[[1]] sig[[1]] - I (b[[2]] sig[[2]] + b[[3]] sig[[3]] + b[[4]] sig[[4]]);
biqOf[m_] := Join[{Tr[m]/2}, Table[I/2 Tr[sig[[j + 1]] . m], {j, 3}]];

(* ---- bi-octonions: x = a + b e_4 is the pair {a, b} of bi-quaternions ---- *)
(* oMul is the Cayley-Dickson product (2.3): (a + b e4)(c + d e4) = (a c - dbar b) + (d a + b cbar) e4 *)
oMul[{a_, b_}, {c_, d_}] := {qMul[a, c] - qMul[qBar[d], b], qMul[d, a] + qMul[b, qBar[c]]};
(* oUnit[p] is the unit e_p (p = 0..7; e_{4+j} = e_j e_4) *)
oUnit[p_Integer] := If[p < 4, {UnitVector[4, p + 1], {0, 0, 0, 0}}, {{0, 0, 0, 0}, UnitVector[4, p - 3]}];
(* oBar is the conjugation of bi-octonions, oNorm the complex norm N_LL(x) = N_LL(a) + N_LL(b) *)
oBar[{a_, b_}] := {qBar[a], -b};
oNorm[{a_, b_}] := qNorm[a] + qNorm[b];
(* oMax is the largest modulus of the sixteen coefficients: the size of a difference *)
oMax[x_] := Max[Abs[Flatten[x]]];

(* ---- Lorentz transformations and spinor matrices ---- *)
(* expTraceless[x] is exp(X) of a traceless 2 x 2 matrix: cosh(s) 1 + sinh(s)/s X with s^2 = -det X *)
expTraceless[x_] := Module[{s = Sqrt[-Det[x]]}, If[Abs[s] < 10^-8, IdentityMatrix[2] + x + x . x/2,
    Cosh[s] IdentityMatrix[2] + Sinh[s]/s x]];
(* lorentzGenerator[rot, boost] is the matrix M_LL(u) of the Lorentz generator with the rotation parameters rot and the
   rapidities boost: sum_j (boost_j/2 - i rot_j/2) sigma_j *)
lorentzGenerator[rot_List, boost_List] := Sum[(boost[[j]]/2 - I rot[[j]]/2) sig[[j + 1]], {j, 3}];
(* lorentzMatrix[mg] is L_LL(g)_pp' = (1/2) Re tr(sigma_p M sigma_p' M^dag) of (2.10), mg = M_LL(g) *)
lorentzMatrix[mg_] := Table[Re[Tr[sig[[p + 1]] . mg . sig[[q + 1]] . ConjugateTranspose[mg]]]/2, {p, 0, 3}, {q, 0, 3}];
(* sSpin[mg] is the spinor matrix S_LL(g) = diag((M^dag)^-1, M) of (3.7) *)
sSpin[mg_] := ArrayFlatten[{{Inverse[ConjugateTranspose[mg]], 0}, {0, mg}}];
(* sGen[mu] is s_LL(u) = diag(-M(u)^dag, M(u)), the action of a Lorentz generator on the Dirac index *)
sGen[mu_] := ArrayFlatten[{{-ConjugateTranspose[mu], 0}, {0, mu}}];
(* randUnitBiq[] is the matrix of a random unit bi-quaternion: exp of a random Lorentz generator *)
randUnitBiq[amp_ : 1.5] := expTraceless[lorentzGenerator[amp RandomReal[{-1, 1}, 3], amp RandomReal[{-1, 1}, 3]]];

(* ---- the Fock space of one point: 2^n states, the occupation of mode k is the bit k of the state number ---- *)
(* fockOps[n] are the annihilation operators c_0 .. c_(n-1) as sparse matrices, with the sign (-1)^(number of occupied
   modes below k) of the ordering of Jordan and Wigner *)
fockOps[n_Integer] := Table[SparseArray[Flatten[Table[If[BitGet[b, k] == 1,
       {BitXor[b, 2^k] + 1, b + 1} -> (-1)^DigitCount[BitAnd[b, 2^k - 1], 2, 1], Nothing], {b, 0, 2^n - 1}]], {2^n, 2^n}],
   {k, 0, n - 1}];
(* the seed of the random test values of this notebook *)
SeedRandom[20261121];
Print["the common objects are defined; m_LL = ", mLL];

(* ::Text:: *)
(*The first check of the conventions: the mass matrix of the gauge fields of the Standard Model, computed from the covariant derivative (1.1) with $+ig$, has the eigenvalues $0$ (the photon), $M_W^2$ (twice) and $M_Z^2$, with the eigenvectors $A_\mu$ and $Z_\mu$ of the text. The kinetic term $(D_\mu\langle\Phi^{\rm SM}\rangle)^\dagger D^\mu\langle\Phi^{\rm SM}\rangle$ of the constant vacuum is the quadratic form $\tfrac12\sum_{ab}\mathsf M_{ab}V^a_\mu V^{b,\mu}$ of the four fields $(W^1,W^2,W^3,B)$; the cell computes $\mathsf M$ exactly, with symbolic couplings, and diagonalizes it. The Rust program `sf_part1` checks the same statement with numbers through the four eigenvectors, and the Python program `sf_part1.py` through the eigenvalues computed by LAPACK (check SM-1 of both); the same computation for the superfluid is the mass matrix (14.3) of Part IV.*)

(* ::Input:: *)
(* gSM, gpSM: the couplings g and g' of SU(2)_L and U(1)_Y; vSM: the vacuum value v (all positive symbols) *)
Clear[gSM, gpSM, vSM];
(* phiSM: the vacuum <Phi^SM> = (0, v)/sqrt2 of the Higgs doublet *)
phiSM = {0, vSM/Sqrt[2]};
(* xSM[a]: the vector X_a with D_mu <Phi> = sum_a V^a_mu X_a for the fields V = (W^1, W^2, W^3, B):
   i g tau_a/2 <Phi> for a = 1, 2, 3 and i g' Y <Phi> (Y = 1/2) for B *)
xSM = Join[Table[(I gSM/2) sig[[a + 1]] . phiSM, {a, 3}], {(I gpSM/2) phiSM}];
(* the mass matrix M_ab = 2 Re(X_a^dag X_b): (D <Phi>)^dag (D <Phi>) = (1/2) sum_ab M_ab V^a V^b *)
mSM = Table[2 ComplexExpand[Re[Conjugate[xSM[[a]]] . xSM[[b]]]], {a, 4}, {b, 4}];
(* the predicted eigenvalues: 0, M_W^2 = (g v/2)^2 twice, M_Z^2 = (g^2 + g'^2) v^2/4; they are the eigenvalues exactly when
   the characteristic polynomial det(M - lam 1) equals the product of the factors (lam - eigenvalue) *)
evSMpred = {0, (gSM vSM/2)^2, (gSM vSM/2)^2, (gSM^2 + gpSM^2) vSM^2/4};
charSM = Expand[CharacteristicPolynomial[mSM, lam] - Product[lam - ev, {ev, evSMpred}]];
(* the photon A = sin(thetaW) W^3 + cos(thetaW) B with tan(thetaW) = g'/g is the null vector *)
photonSM = {0, 0, gpSM, gSM}/Sqrt[gSM^2 + gpSM^2];
Print["the mass matrix of (W1, W2, W3, B): ", MatrixForm[mSM]];
checkTrue["WSM-1", "Standard Model with +i g: the eigenvalues of the mass matrix are 0, M_W^2 (twice), M_Z^2, and the photon sin(thetaW) W^3 + cos(thetaW) B is massless",
  charSM === 0 && Simplify[mSM . photonSM] === {0, 0, 0, 0}, "exact, with symbolic g, g', v",
  "SM-1", "SM-1"];

(* ::Section:: *)
(*I.2 The manifold M31 of the Lorentz super-loop (item 1)*)

(* ::Text:: *)
(***Bi-quaternions.** A bi-quaternion is $\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\,\mathbf e_p$ with four complex coefficients and the units $\mathbf e_0=1,\mathbf e_1,\mathbf e_2,\mathbf e_3$, multiplied by the rules of the quaternions, $\mathbf e_j^2=-1$, $\mathbf e_1\mathbf e_2=-\mathbf e_2\mathbf e_1=\mathbf e_3$ and cyclically. The conjugation $\bar{\mathbf b}$ changes the signs of the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$, the complex conjugation $\mathbf b^*$ conjugates every coefficient, and $\mathbf b^\dagger=(\bar{\mathbf b})^*$. The complex norm $N_{\rm LL}(\mathbf b)=\sum_p\xi^{\rm LL}_p(\mathbf b)^2$ satisfies $\mathbf b\bar{\mathbf b}=N_{\rm LL}(\mathbf b)\,\mathbf e_0$ and $N_{\rm LL}(\mathbf b\mathbf b')=N_{\rm LL}(\mathbf b)N_{\rm LL}(\mathbf b')$. The matrix $M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0\sigma^{\rm LL}_0-i\sum_j\xi^{\rm LL}_j\sigma^{\rm LL}_j$ is multiplicative, has the determinant $N_{\rm LL}(\mathbf b)$, satisfies $M_{\rm LL}(\mathbf b^\dagger)=M_{\rm LL}(\mathbf b)^\dagger$, and every complex $2\times2$ matrix is the matrix of exactly one bi-quaternion; the unit bi-quaternions ($N_{\rm LL}=1$) are therefore the group $SL(2,\mathbb C)$.*)

(* ::Text:: *)
(***Bi-octonions and the super-loop.** A bi-octonion is $\mathbf x=\mathbf a+\mathbf b\,\mathbf e_4$ with two bi-quaternions, multiplied by the rule of Cayley and Dickson*)

(* ::DisplayFormula:: *)
(*(\mathbf a+\mathbf b\,\mathbf e_4)(\mathbf c+\mathbf d\,\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\,\mathbf e_4,*)

(* ::Text:: *)
(*with the eight units $\mathbf e_0,\dots,\mathbf e_3,\mathbf e_4$ and $\mathbf e_{4+j}=\mathbf e_j\mathbf e_4$. The norm is multiplicative, $N_{\rm LL}(\mathbf x\mathbf y)=N_{\rm LL}(\mathbf x)N_{\rm LL}(\mathbf y)$, and $\mathbf x\bar{\mathbf x}=N_{\rm LL}(\mathbf x)\mathbf e_0$. The product is not associative ($(\mathbf e_1\mathbf e_2)\mathbf e_4-\mathbf e_1(\mathbf e_2\mathbf e_4)=2\mathbf e_7$), but it is alternative, $(\mathbf x\mathbf x)\mathbf y=\mathbf x(\mathbf x\mathbf y)$, and satisfies the three Moufang identities $(\mathbf z\mathbf x\mathbf z)\mathbf y=\mathbf z(\mathbf x(\mathbf z\mathbf y))$, $\mathbf y(\mathbf z\mathbf x\mathbf z)=((\mathbf y\mathbf z)\mathbf x)\mathbf z$ and $(\mathbf z\mathbf x)(\mathbf y\mathbf z)=(\mathbf z(\mathbf x\mathbf y))\mathbf z$. The unit bi-octonions form the Lorentz super-loop $\mathbb L_{\rm LL}$, a Moufang loop of real dimension 14 that contains $SL(2,\mathbb C)$.*)

(* ::Text:: *)
(*The next cell checks these rules exactly: bi-octonions with small integer (Gaussian integer) coefficients are multiplied in exact arithmetic, so every identity must hold with the deviation exactly 0. Of the 343 ordered triples of imaginary units, 168 have a nonzero associator: a triple with two equal members has none (alternativity), and of the 210 triples of distinct units a triple has none exactly when its third unit is plus or minus the product of the first two, which leaves 210 - 42 = 168.*)

(* ::Input:: *)
(* A-2: the rules of the eight units: e_p^2 = -1, e_p e_q = -e_q e_p (p != q, both >= 1), e_j e_4 = e_(4+j) *)
dA2 = Max[Join[
    Table[oMax[oMul[oUnit[p], oUnit[p]] + oUnit[0]], {p, 1, 7}],
    Flatten[Table[If[p != q, oMax[oMul[oUnit[p], oUnit[q]] + oMul[oUnit[q], oUnit[p]]], 0], {p, 1, 7}, {q, 1, 7}]],
    Table[oMax[oMul[oUnit[j], oUnit[4]] - oUnit[4 + j]], {j, 1, 3}]]];
check["WA-2", "bi-octonion units: e_p^2 = -1, e_p e_q = -e_q e_p, e_j e_4 = e_(4+j) (exact)", dA2, 0, "A-2", "A-2"];
(* rintBio[] is a random bi-octonion with Gaussian-integer coefficients between -3 and 3 *)
rintBio[] := {RandomInteger[{-3, 3}, 4] + I RandomInteger[{-3, 3}, 4], RandomInteger[{-3, 3}, 4] + I RandomInteger[{-3, 3}, 4]};
(* A-3 to A-6 on 200 random triples (x, y, z), in exact arithmetic *)
{dA3, dA4, dA5, dA6} = {0, 0, 0, 0};
Do[Module[{x = rintBio[], y = rintBio[], z = rintBio[], zxz},
   dA3 = Max[dA3, Abs[oNorm[oMul[x, y]] - oNorm[x] oNorm[y]]];
   dA4 = Max[dA4, oMax[oMul[x, oBar[x]] - oNorm[x] oUnit[0]]];
   dA5 = Max[dA5, oMax[oMul[oMul[x, x], y] - oMul[x, oMul[x, y]]], oMax[oMul[oMul[y, x], x] - oMul[y, oMul[x, x]]]];
   zxz = oMul[oMul[z, x], z];
   dA6 = Max[dA6, oMax[oMul[zxz, y] - oMul[z, oMul[x, oMul[z, y]]]], oMax[oMul[y, zxz] - oMul[oMul[oMul[y, z], x], z]],
     oMax[oMul[oMul[z, x], oMul[y, z]] - oMul[oMul[z, oMul[x, y]], z]]]], {200}];
check["WA-3", "N(xy) = N(x) N(y), 200 random Gaussian-integer bi-octonions (exact)", dA3, 0, "A-3", "A-3"];
check["WA-4", "x xbar = N(x) e_0 (exact)", dA4, 0, "A-4", "A-4"];
check["WA-5", "alternativity (xx)y = x(xy), (yx)x = y(xx) (exact)", dA5, 0, "A-5", "A-5"];
check["WA-6", "the three Moufang identities (exact)", dA6, 0, "A-6", "A-6"];
(* A-7: the associators of the 343 ordered triples of imaginary units, and (e1 e2) e4 - e1 (e2 e4) = 2 e7 *)
nonzeroAssociators = Count[Flatten[Table[oMax[oMul[oMul[oUnit[p], oUnit[q]], oUnit[r]] - oMul[oUnit[p], oMul[oUnit[q], oUnit[r]]]],
      {p, 1, 7}, {q, 1, 7}, {r, 1, 7}]], v_ /; v > 0];
a124 = oMul[oMul[oUnit[1], oUnit[2]], oUnit[4]] - oMul[oUnit[1], oMul[oUnit[2], oUnit[4]]];
checkTrue["WA-7", "168 of the 343 triples of imaginary units have a nonzero associator; (e1 e2) e4 - e1 (e2 e4) = 2 e7",
  nonzeroAssociators == 168 && oMax[a124 - 2 oUnit[7]] == 0, ToString[nonzeroAssociators] <> " nonzero", "A-7", "A-7"];

(* ::Text:: *)
(***The manifold M31 and its Lorentz group.** M31 is the real vector space of the Hermitian bi-quaternions $\mathbf x=x^0_{\rm LL}\mathbf e_0+i\sum_jx^j_{\rm LL}\mathbf e_j$; their matrices $M_{\rm LL}(\mathbf x)=\sum_px^p_{\rm LL}\sigma^{\rm LL}_p$ are the Hermitian $2\times2$ matrices, and the norm $N_{\rm LL}(\mathbf x)=(x^0_{\rm LL})^2-\sum_j(x^j_{\rm LL})^2$ is the Minkowski interval. A unit bi-quaternion $\mathbf g$ acts by $\mathbf x\mapsto\mathbf g\mathbf x\mathbf g^\dagger$; this keeps M31 and the interval, so it is a Lorentz transformation, with the real matrix*)

(* ::DisplayFormula:: *)
(*\mathsf L_{\rm LL}(\mathbf g)_{pp'}=\tfrac12\,\mathrm{Re}\,\mathrm{tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\,\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger\big),*)

(* ::Text:: *)
(*which is proper and orthochronous ($\mathsf L^T\eta\mathsf L=\eta$, $\det\mathsf L=1$, $\mathsf L_{00}\ge1$) and multiplicative, with $\mathsf L_{\rm LL}(-\mathbf g)=\mathsf L_{\rm LL}(\mathbf g)$: $SL(2,\mathbb C)$ is the double cover of the proper orthochronous Lorentz group. The next cell checks the properties of $M_{\rm LL}$ exactly (with Gaussian-integer coefficients) and those of $\mathsf L_{\rm LL}$ with 200 random unit bi-quaternions in floating point. A spinor charge of the super-loop is a bi-octonion $\mathbf b\,\mathbf e_4$; by the product rule the super-loop acts on it as $(\mathbf b\mathbf e_4)\mathbf g=(\mathbf b\bar{\mathbf g})\mathbf e_4$ and $\mathbf g(\mathbf b\mathbf e_4)=(\mathbf b\mathbf g)\mathbf e_4$, and the isospin $\mathbf v$ acts as $(\mathbf v((\mathbf b\mathbf e_4)\mathbf e_4))\bar{\mathbf e}_4=(\mathbf v\mathbf b)\mathbf e_4$; the last check of the cell verifies these three rules exactly.*)

(* ::Input:: *)
(* A-8: M(bb') = M(b) M(b'), det M(b) = N(b), M(b^dag) = M(b)^dag, and the inverse map, 200 random Gaussian-integer pairs *)
rintBiq[] := RandomInteger[{-3, 3}, 4] + I RandomInteger[{-3, 3}, 4];
dA8 = Max[Table[Module[{a = rintBiq[], b = rintBiq[]},
     Max[Abs[Flatten[mLLmat[qMul[a, b]] - mLLmat[a] . mLLmat[b]]], Abs[Det[mLLmat[a]] - qNorm[a]],
       Abs[Flatten[mLLmat[qDag[a]] - ConjugateTranspose[mLLmat[a]]]], Abs[biqOf[mLLmat[a]] - a]]], {200}]];
check["WA-8", "M(bb') = M(b)M(b'), det M = N, M(b^dag) = M(b)^dag, inverse map (exact)", dA8, 0, "A-8", "A-8"];
(* A-10b, c, d: the Lorentz matrices of 200 random unit bi-quaternions (floating point) *)
{dA10b, dA10d, l00ok} = {0., 0., True};
Do[Module[{mg = randUnitBiq[], mh = randUnitBiq[], lg, lh},
   lg = lorentzMatrix[mg]; lh = lorentzMatrix[mh];
   dA10b = Max[dA10b, Max[Abs[Transpose[lg] . etaLL . lg - etaLL]], Abs[Det[lg] - 1]];
   dA10d = Max[dA10d, Max[Abs[lorentzMatrix[mg . mh] - lg . lh]], Max[Abs[lorentzMatrix[-mg] - lg]]];
   l00ok = l00ok && lg[[1, 1]] >= 1 - 10^-12], {200}];
check["WA-10b", "L^T eta L = eta and det L = 1 (200 random unit g)", dA10b, 10^-10, "A-10b", "A-10b"];
checkTrue["WA-10c", "L_00 >= 1: orthochronous", l00ok, "200 random unit g", "A-10c", "A-10c"];
check["WA-10d", "L(gh) = L(g) L(h) and L(-g) = L(g) (200 random pairs)", dA10d, 10^-10, "A-10d", "A-10d"];
(* A-12: the spinor charges b e4 under the multiplications of the super-loop (exact): (b e4) g = (b gbar) e4,
   g (b e4) = (b g) e4, and the isospin (g ((b e4) e4)) e4bar = (g b) e4 *)
z4 = {0, 0, 0, 0};
dA12 = Max[Table[Module[{b = rintBiq[], g = rintBiq[], be4, gq, iso},
     be4 = {z4, b}; gq = {g, z4};
     iso = oMul[oMul[gq, oMul[be4, oUnit[4]]], oBar[oUnit[4]]];
     Max[oMax[oMul[be4, gq] - {z4, qMul[b, qBar[g]]}], oMax[oMul[gq, be4] - {z4, qMul[b, g]}], oMax[iso - {z4, qMul[g, b]}]]], {200}]];
check["WA-12", "spinor charges under the super-loop multiplications (exact)", dA12, 0, "A-12", "A-12"];

(* ::Section:: *)
(*I.3 The spinor field and the DiracLike field (item 2); theorem D1*)

(* ::Text:: *)
(*A two-component spinor transforms under the unit bi-quaternion $\mathbf g$ as $\chi\mapsto M_{\rm LL}(\mathbf g)\chi$ (or, for the other kind, with $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$). The Dirac matrices of a frame $\mathbf f_p$ are $\gamma^p_{\rm LL}[\mathbf f]$ with the blocks $M_{\rm LL}(\mathbf f_p)$ and $M_{\rm LL}(\bar{\mathbf f}_p)$ off the diagonal; the spinor matrix is $\mathsf S_{\rm LL}(\mathbf g)=\mathrm{diag}(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g))$, and*)

(* ::DisplayFormula:: *)
(*\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}\mathsf L_{pp'}\gamma^{p'}_{\rm LL},\qquad\mathsf S_{\rm LL}(\mathbf g)^\dagger\gamma^0_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\gamma^0_{\rm LL},\qquad\gamma^p_{\rm LL}[\mathbf f']=\mathsf S_{\rm LL}(\mathbf g)\,\gamma^p_{\rm LL}[\mathbf f]\,\mathsf S_{\rm LL}(\mathbf g)^{-1}*)

(* ::Text:: *)
(*for the transformed frame $\mathbf f'_p=\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$ (equations (3.8) and (3.9) of the manuscript). The spinor field $\Psi_{\rm LL}$ is the DiracLike isodoublet: eight anticommuting components, made of two spinor charges of the super-loop; it transforms as $\Psi_{\rm LL}\mapsto(\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf 1_2)\Psi_{\rm LL}$, its isospin generators are $T^\iota_{\rm LL}$, its fermion number is 1, and its mass term $m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL}$ is invariant under all of GEE.*)

(* ::Text:: *)
(***Theorem D1.** A single DiracLike field with four components, on which $SU(2)_{\rm v}$ acts by a non-trivial linear representation that commutes with $SL(2,\mathbb C)$ and keeps the kinetic term, has no Lorentz-invariant bilinear without derivatives (no Dirac, pseudoscalar or Majorana mass) that is invariant under $SU(2)_{\rm v}$. The reason: the two left-handed Weyl spinors $\psi^-$ and $\mathsf E_{\rm LL}(\psi^+)^*$ of the DiracLike field form an isospin doublet (the group of Pauli and Gürsey), and the Lorentz-invariant bilinears $\xi_f^T\mathsf E_{\rm LL}\xi_{f'}$ are symmetric in the doublet indices, an isospin triplet, which contains no invariant. This is why the spinor field of the solution is an isodoublet of DiracLike fields. The next cell repeats the finite part of the proof as a computation: on the eight operators $(\psi_1,\dots,\psi_4,\psi^\dagger_1,\dots,\psi^\dagger_4)$ of one point, a bilinear $\mathfrak n^T\mathfrak Q\mathfrak n$ with an antisymmetric $8\times8$ matrix $\mathfrak Q$ is invariant under a group exactly when $\mathfrak X^T\mathfrak Q+\mathfrak Q\mathfrak X=0$ for every generator $\mathfrak X$; the dimension of the space of such $\mathfrak Q$ is $28$ minus the rank of these linear conditions. With exact arithmetic: dimension 6 for the Lorentz group, 0 for the Lorentz group together with the Pauli-Gürsey $SU(2)$.*)

(* ::Input:: *)
(* F-2: the Clifford algebra {gamma^p, gamma^q} = 2 eta^pq, C gamma^p C^dag = -(gamma^p)^T, gamma^ch = diag(-1,-1,1,1), C^T = -C (exact) *)
dF2 = Max[Flatten[{Table[Abs[gam[p] . gam[q] + gam[q] . gam[p] - 2 etaLL[[p + 1, q + 1]] IdentityMatrix[4]], {p, 0, 3}, {q, 0, 3}],
     Table[Abs[cLL . gam[p] . ConjugateTranspose[cLL] + Transpose[gam[p]]], {p, 0, 3}],
     Abs[gch - DiagonalMatrix[{-1, -1, 1, 1}]], Abs[cLL + Transpose[cLL]]}]];
check["WF-2", "Clifford algebra, chirality, charge conjugation (exact)", dF2, 0, "F-2", "F-2"];
(* F-3a, F-3b, F-3d: the spinor matrices of 200 random unit bi-quaternions (floating point) *)
{dF3a, dF3b, dF3d} = {0., 0., 0.};
Do[Module[{mg = randUnitBiq[], s, si, lg, mgt, gf},
   s = sSpin[mg]; si = Inverse[s]; lg = lorentzMatrix[mg]; mgt = Inverse[ConjugateTranspose[mg]];
   Do[dF3a = Max[dF3a, Max[Abs[si . gam[p] . s - Sum[lg[[p + 1, q + 1]] gam[q], {q, 0, 3}]]]];
     (* the Dirac matrices of the frame gtilde f gtilde^dag, built from its blocks, against S gamma^p S^-1 *)
     gf = ArrayFlatten[{{0, mgt . sig[[p + 1]] . ConjugateTranspose[mgt]}, {mg . sigbar[[p + 1]] . ConjugateTranspose[mg], 0}}];
     dF3d = Max[dF3d, Max[Abs[gf - s . gam[p] . si]]], {p, 0, 3}];
   dF3b = Max[dF3b, Max[Abs[ConjugateTranspose[s] . gam[0] . s - gam[0]]]]], {200}];
check["WF-3a", "S^-1 gamma^p S = sum_q L_pq gamma^q (200 random g)", dF3a, 10^-10, "F-3a", "F-3a"];
check["WF-3b", "S^dag gamma^0 S = gamma^0 (200 random g)", dF3b, 10^-10, "F-3b", "F-3b"];
check["WF-3d", "the Dirac matrices of the transformed frame are S gamma^p S^-1 (200 random g)", dF3d, 10^-10, "F-3d", "F-3d"];
(* F-5: theorem D1.  lorGens: the six Lorentz generators on (psi, psi^dag): diag(s(u), conj(s(u))) *)
lorGens = Flatten[Table[Module[{r = {0, 0, 0}, b = {0, 0, 0}, sg},
     If[kind == 0, r[[j]] = 1, b[[j]] = 1];
     sg = sGen[lorentzGenerator[r, b]];
     ArrayFlatten[{{sg, 0}, {0, Conjugate[sg]}}]], {j, 3}, {kind, 0, 1}], 1];
(* pmD1: the map from (psi, psi^dag) to the doublet (xi_1, xi_2) = (psi^-, E conj(psi^+)); amatD1 completes it with the conjugates *)
pmD1 = ConstantArray[0, {4, 8}]; pmD1[[1, 1]] = 1; pmD1[[2, 2]] = 1; pmD1[[3 ;; 4, 7 ;; 8]] = eLL;
sxD1 = ArrayFlatten[{{0, IdentityMatrix[4]}, {IdentityMatrix[4], 0}}];
amatD1 = Join[pmD1, Conjugate[pmD1] . sxD1];
(* pgGens: the three Pauli-Guersey generators, the doublet action (i tau/2) (x) 1_2 on (xi_1, xi_2), carried back *)
pgGens = Table[With[{t = KroneckerProduct[I sig[[iota + 1]]/2, IdentityMatrix[2]]},
    Inverse[amatD1] . ArrayFlatten[{{t, 0}, {0, Conjugate[t]}}] . amatD1], {iota, 3}];
dF5a = Max[Flatten[Table[Abs[x . l - l . x], {x, pgGens}, {l, lorGens}]]];
check["WF-5a", "D1: the Pauli-Guersey generators commute with the Lorentz generators (exact)", dF5a, 0, "F-5a", "F-5a"];
(* the 28 antisymmetric 8 x 8 matrices E_ij - E_ji (i < j), a basis of the bilinears *)
basisD1 = Flatten[Table[If[i < j, Normal[SparseArray[{{i, j} -> 1, {j, i} -> -1}, {8, 8}]], Nothing], {i, 8}, {j, 8}], 1];
(* invDim[gens]: the dimension of the space of antisymmetric Q with X^T Q + Q X = 0 for every generator X *)
invDim[gens_] := Length[basisD1] - MatrixRank[Join @@ Table[Transpose[Table[Flatten[Transpose[x] . b + b . x], {b, basisD1}]], {x, gens}]];
{dimLorentz, dimGEE} = {invDim[lorGens], invDim[Join[lorGens, pgGens]]};
checkTrue["WF-5c", "D1: the Lorentz-invariant bilinears of one point form a space of dimension 6; those invariant also under the Pauli-Guersey SU(2), dimension 0",
  dimLorentz == 6 && dimGEE == 0, "dimensions " <> ToString[dimLorentz] <> ", " <> ToString[dimGEE] <> " (exact ranks)", "F-5c", "F-5c"];

(* ::Section:: *)
(*I.4 The gauge group GEE and its covariant derivatives (item 3)*)

(* ::Text:: *)
(*The gauge group is $\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$. A local transformation is a unit bi-quaternion $\mathbf g(x)$, an isospin rotation $\mathsf U_{\rm LL}(x)$ and a phase $\theta^{\rm F}_{\rm LL}(x)$; it acts on the spinor field as $\Psi_{\rm LL}\mapsto e^{i\theta^{\rm F}_{\rm LL}}\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}\,\Psi_{\rm LL}$. The gauge fields are the Lorentz connection with the spin connection $\Gamma^{\rm LL}_p$, the field of $SU(2)_{\rm v}$, $\mathsf W_{{\rm SF},p}=\sum_\iota W^\iota_{{\rm SF},p}\tau^{\rm LL}_\iota/2$, and the field $B_{{\rm SF},p}$ of $U(1)^{\rm F}_{\rm LL}$, with the couplings $g^{\rm v}_{\rm LL}$ and $g^{\rm F}_{\rm LL}$. The covariant derivative of the spinor field is*)

(* ::DisplayFormula:: *)
(*\nabla^{\rm GEE}_p\Psi_{\rm LL}=\Big(\partial_p+\Gamma^{\rm LL}_p+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},p}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\Big)\Psi_{\rm LL},*)

(* ::Text:: *)
(*and it is covariant, $\nabla'^{\rm GEE}_p(\mathcal U_{\rm LL}\Psi_{\rm LL})=\mathcal U_{\rm LL}\nabla^{\rm GEE}_p\Psi_{\rm LL}$, when the gauge fields transform as*)

(* ::DisplayFormula:: *)
(*\Gamma'^{\rm LL}_p=\mathsf S\,\Gamma^{\rm LL}_p\,\mathsf S^{-1}-(\partial_p\mathsf S)\,\mathsf S^{-1},\qquad\mathsf W'_{{\rm SF},p}=\mathsf U_{\rm LL}\mathsf W_{{\rm SF},p}\mathsf U_{\rm LL}^\dagger+\frac i{g^{\rm v}_{\rm LL}}(\partial_p\mathsf U_{\rm LL})\mathsf U_{\rm LL}^\dagger,\qquad B'_{{\rm SF},p}=B_{{\rm SF},p}-\frac1{g^{\rm F}_{\rm LL}}\partial_p\theta^{\rm F}_{\rm LL}.*)

(* ::Text:: *)
(*An isospin vector of fermion number 2 and spin 0, such as the pair field $\vec\Phi$ of the superfluid, has the covariant derivative $\nabla_p\vec\Phi=\partial_p\vec\Phi-g^{\rm v}_{\rm LL}\vec W_{{\rm SF},p}\times\vec\Phi+2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec\Phi$; the field strengths are $\mathsf W_{{\rm SF},pp'}=\partial_p\mathsf W_{{\rm SF},p'}-\partial_{p'}\mathsf W_{{\rm SF},p}+ig^{\rm v}_{\rm LL}[\mathsf W_{{\rm SF},p},\mathsf W_{{\rm SF},p'}]$ and $B_{{\rm SF},pp'}=\partial_pB_{{\rm SF},p'}-\partial_{p'}B_{{\rm SF},p}$, and $\mathsf W'_{{\rm SF},pp'}=\mathsf U_{\rm LL}\mathsf W_{{\rm SF},pp'}\mathsf U_{\rm LL}^\dagger$.*)

(* ::Text:: *)
(*The next cell checks these covariance laws with random smooth fields: the isospin rotation $\mathsf U_{\rm LL}(x)=\exp(\tfrac i2\sum_\iota a^\iota(x)\tau^{\rm LL}_\iota)$, the phase, the gauge fields and the test fields are sums of sines of the four coordinates with random coefficients. The derivatives are computed symbolically by the function `D` of the Wolfram Language and evaluated at four random points: a third method, next to the automatic differentiation of the Rust program and the differences of eighth order of the Python program. The couplings of the test are $g^{\rm v}=0.83$ and $g^{\rm F}=0.47$. The last check of the cell is the local $SL(2,\mathbb C)$ transformation with the frame: with the spinor matrix $\mathsf S(x)$ of a random field of unit bi-quaternions and the connection $\Gamma'_p=-(\partial_p\mathsf S)\mathsf S^{-1}$ of the transformed frame, $\sum_p\gamma^p[\mathbf f'](\partial_p+\Gamma'_p)(\mathsf S\psi)=\mathsf S\sum_p\gamma^p\partial_p\psi$.*)

(* ::Input:: *)
(* the four coordinates x^0 .. x^3 of the test, as symbols *)
Clear[x0, x1, x2, x3];
xs = {x0, x1, x2, x3};
(* gvT, gfT: the couplings of the test *)
{gvT, gfT} = {0.83, 0.47};
(* smooth[off]: a random smooth real function off + a1 sin(k1.x + ph1) + a2 sin(k2.x + ph2) *)
smooth[off_] := Module[{a = {off + 0.5 RandomReal[{-1, 1}], 0.6 RandomReal[{-1, 1}], 0.4 RandomReal[{-1, 1}]},
    k = RandomReal[{-1, 1}, {2, 4}], ph = RandomReal[{-3, 3}, 2]},
   a[[1]] + a[[2]] Sin[k[[1]] . xs + ph[[1]]] + a[[3]] Sin[k[[2]] . xs + ph[[2]]]];
(* the isospin rotation U = exp((i/2) a . tau) = cos(r/2) 1 + i sin(r/2) (a . tau)/r, r = |a|, and its adjoint *)
aT = Table[smooth[1.2], {3}];
rT = Sqrt[aT . aT];
uT = Cos[rT/2] IdentityMatrix[2] + I Sin[rT/2] Sum[aT[[i]] sig[[i + 1]], {i, 3}]/rT;
uTdag = Cos[rT/2] IdentityMatrix[2] - I Sin[rT/2] Sum[aT[[i]] sig[[i + 1]], {i, 3}]/rT;
(* the phase theta^F of U(1)^F *)
thT = smooth[0.3];
(* the gauge fields: wT[p] = sum_i W^i_p tau_i/2 (real components), bT[p] = B_p, p = 0..3 *)
wComp = Table[smooth[0.], {4}, {3}];
bComp = Table[smooth[0.], {4}];
wT[p_] := Sum[wComp[[p + 1, i]] sig[[i + 1]]/2, {i, 3}];
bT[p_] := bComp[[p + 1]];
(* dd[f, p]: the derivative with respect to x^p *)
dd[f_, p_] := D[f, xs[[p + 1]]];
(* the transformed gauge fields (4.5) *)
wTp[p_] := uT . wT[p] . uTdag + I/gvT dd[uT, p] . uTdag;
bTp[p_] := bT[p] - dd[thT, p]/gfT;
(* the test spinor (eight components, two plane waves) and its transform e^(i theta) (1_4 (x) U) psi *)
kT = RandomReal[{-1, 1}, {2, 4}];
ampT = RandomReal[{-1, 1}, {8, 2}] + I RandomReal[{-1, 1}, {8, 2}];
psiT = ampT[[All, 1]] Exp[I kT[[1]] . xs] + ampT[[All, 2]] Exp[I kT[[2]] . xs];
psiTp = Exp[I thT] KroneckerProduct[IdentityMatrix[4], uT] . psiT;
(* covPsi[w, b, v, p]: nabla_p v = d_p v + i g_v (1_4 (x) W_p) v + i g_F B_p v *)
covPsi[w_, b_, v_, p_] := dd[v, p] + I gvT KroneckerProduct[IdentityMatrix[4], w[p]] . v + I gfT b[p] v;
(* four random points of the test *)
ptsT = Table[Thread[xs -> RandomReal[{-2, 2}, 4]], {4}];
(* C-1: nabla'_p (e^(i theta) U psi) = e^(i theta) U nabla_p psi *)
dC1 = Max[Table[Max[Abs[(covPsi[wTp, bTp, psiTp, p] - Exp[I thT] KroneckerProduct[IdentityMatrix[4], uT] . covPsi[wT, bT, psiT, p]) /. pt]],
    {pt, ptsT}, {p, 0, 3}]];
check["WC-1", "SU(2)_v x U(1)^F covariance of nabla on Psi (symbolic derivatives, 4 points)", dC1, 10^-9, "C-1", "C-1"];
(* C-3: the field strength W'_pq = U W_pq U^dag *)
fieldStrength[w_, p_, q_] := dd[w[q], p] - dd[w[p], q] + I gvT (w[p] . w[q] - w[q] . w[p]);
dC3 = Max[Table[Max[Abs[(fieldStrength[wTp, p, q] - uT . fieldStrength[wT, p, q] . uTdag) /. pt]],
    {pt, ptsT[[1 ;; 2]]}, {p, 0, 3}, {q, p + 1, 3}]];
check["WC-3", "field strength W'_pq = U W_pq U^dag (symbolic second derivatives, 2 points)", dC3, 10^-8, "C-3", "C-3"];
(* C-4: the isotriplet of fermion number 2: Phi' = e^(2 i theta) R(U) Phi, nabla Phi = dPhi - g W x Phi + 2 i g_F B Phi *)
phiAmp = RandomReal[{-1, 1}, {3, 2}] + I RandomReal[{-1, 1}, {3, 2}];
phiT = phiAmp[[All, 1]] Exp[I kT[[1]] . xs] + phiAmp[[All, 2]] Exp[I kT[[2]] . xs];
(* rotT: the rotation matrix R_ab = (1/2) tr(tau_a U tau_b U^dag) of the isospin rotation *)
rotT = Table[Tr[sig[[a + 1]] . uT . sig[[b + 1]] . uTdag]/2, {a, 3}, {b, 3}];
phiTp = Exp[2 I thT] rotT . phiT;
(* wVec[w, p]: the three components W^i_p = tr(tau_i W_p) *)
wVec[w_, p_] := Table[Tr[sig[[i + 1]] . w[p]], {i, 3}];
covPhi[w_, b_, v_, p_] := dd[v, p] - gvT Cross[wVec[w, p], v] + 2 I gfT b[p] v;
dC4 = Max[Table[Max[Abs[(covPhi[wTp, bTp, phiTp, p] - Exp[2 I thT] rotT . covPhi[wT, bT, phiT, p]) /. pt]], {pt, ptsT}, {p, 0, 3}]];
check["WC-4", "isotriplet: nabla Phi = dPhi - g W x Phi + 2 i g_F B Phi is covariant (symbolic derivatives)", dC4, 10^-9, "C-4", "C-4"];
(* C-8: local SL(2,C) with the frame.  The field of unit bi-quaternions M(x) = exp(c . sigma) with c_j = -(i/2) r_j + (1/2) b_j
   (r_j, b_j real functions); its adjoint M^dag = exp(cbar . sigma) with the complex-conjugate coefficients *)
rotF = Table[smooth[0.7], {3}]; boostF = Table[smooth[0.4], {3}];
cF = -I/2 rotF + 1/2 boostF; cFbar = I/2 rotF + 1/2 boostF;
(* expSig[c] = exp(c . sigma) = cosh(s) 1 + sinh(s)/s (c . sigma), s^2 = c . c (even in s, so no branch of the root matters) *)
expSig[c_] := With[{s = Sqrt[c . c]}, Cosh[s] IdentityMatrix[2] + Sinh[s]/s Sum[c[[j]] sig[[j + 1]], {j, 3}]];
mF = expSig[cF]; mFdag = expSig[cFbar];
(* inv2[m]: the inverse of a 2 x 2 matrix of determinant 1, the adjugate *)
inv2[m_] := {{m[[2, 2]], -m[[1, 2]]}, {-m[[2, 1]], m[[1, 1]]}};
mFt = inv2[mFdag];          (* M(gtilde) = (M^dag)^-1 *)
mFtdag = inv2[mF];          (* its adjoint, (M^-1)^dag ... written as the adjoint of M^-1: ((M^dag)^-1)^dag = M^-1 *)
sF = ArrayFlatten[{{mFt, 0}, {0, mF}}];
sFinv = ArrayFlatten[{{mFdag, 0}, {0, inv2[mF]}}];
(* the four Dirac components of isospin up of the test spinor *)
psi4 = psiT[[1 ;; 8 ;; 2]];
spsi = sF . psi4;
lhsC8 = Sum[ArrayFlatten[{{0, mFt . sig[[p + 1]] . mFtdag}, {mF . sigbar[[p + 1]] . mFdag, 0}}] .
     (dd[spsi, p] - dd[sF, p] . sFinv . spsi), {p, 0, 3}];
rhsC8 = sF . Sum[gam[p] . dd[psi4, p], {p, 0, 3}];
dC8 = Max[Table[Max[Abs[(lhsC8 - rhsC8) /. pt]], {pt, ptsT}]];
check["WC-8", "local SL(2,C): gamma^p[f'] (d_p + Gamma'_p)(S psi) = S gamma^p d_p psi (symbolic derivatives)", dC8, 10^-9, "C-8", "C-8"];

(* ::Section:: *)
(*I.5 The Lagrangian and the field equations (item 4)*)

(* ::Text:: *)
(*The Lagrangian density of the solution is $\mathcal L_{\rm SF}=\mathcal L^\Psi_{\rm SF}+\mathcal L^{\rm self}_{\rm SF}+\mathcal L^{\rm YM}_{\rm SF}$, with the Dirac part $\mathcal L^\Psi_{\rm SF}=\bar\Psi_{\rm LL}(i\sum_p\gamma^p_{\rm LL}\nabla^{\rm GEE}_p-m_{\rm LL})\Psi_{\rm LL}$, the Yang-Mills part $\mathcal L^{\rm YM}_{\rm SF}=-\tfrac14\sum_\iota\sum_{pp'}W^\iota_{{\rm SF},pp'}W^{\iota,pp'}_{\rm SF}-\tfrac14\sum_{pp'}B_{{\rm SF},pp'}B^{pp'}_{\rm SF}$, and the self-interaction $\mathcal L^{\rm self}_{\rm SF}=-V^{(n)}_{\rm SF}$, a normal-ordered GEE-invariant contact interaction. Its first two steps are*)

(* ::DisplayFormula:: *)
(*V^{(0)}_{\rm SF}=\tfrac12G^{\rm S}_{\rm SF}:(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:+\tfrac12G^{\rm V}_{\rm SF}\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:,\qquad V^{(1)}_{\rm SF}=V^{(0)}_{\rm SF}-\tfrac14G_{\rm LL}\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:,*)

(* ::Text:: *)
(*with the fermion-number current $j^{{\rm F},p}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}$ and the pair operators $\Pi^\iota_{\rm LL}=\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}$ of one point. $V^{(0)}_{\rm SF}$ is formed after the author's Kohn-Sham ansatz, whose interaction energy density of uniform matter is $\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$; $V^{(1)}_{\rm SF}$ adds the attraction in the spin-zero pair channel. Varying the action with respect to $\bar\Psi_{\rm LL}$ gives the field equation of the spinor field,*)

(* ::DisplayFormula:: *)
(*i\sum_p\gamma^p_{\rm LL}\nabla^{\rm GEE}_p\Psi_{\rm LL}-m_{\rm LL}\Psi_{\rm LL}=G^{\rm S}_{\rm SF}(\bar\Psi_{\rm LL}\Psi_{\rm LL})\Psi_{\rm LL}+G^{\rm V}_{\rm SF}\sum_pj^{\rm F}_{{\rm SF},p}\gamma^p_{\rm LL}\Psi_{\rm LL}-\tfrac12G_{\rm LL}\sum_\iota\gamma^0_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger\Psi^*_{\rm LL}\,\Pi^\iota_{\rm LL},*)

(* ::Text:: *)
(*and varying with respect to the gauge fields gives the Yang-Mills equations with the currents of the spinor field as sources, $\sum_p\partial_pW^{\iota,pp'}_{\rm SF}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}\sum_pW^{\iota'}_{{\rm SF},p}W^{\iota'',pp'}_{\rm SF}=g^{\rm v}_{\rm LL}j^{\iota,p'}_{\rm SF}$ and $\sum_p\partial_pB^{pp'}_{\rm SF}=g^{\rm F}_{\rm LL}j^{{\rm F},p'}_{\rm SF}$.*)

(* ::Text:: *)
(*The right side of the field equation of the spinor field is the commutator of the field with the Hamiltonian of the contact terms. The next cell checks this exactly, as an identity of operators on the Fock space of the eight components of one point ($2^8=256$ states), for each contact term: with the Hamiltonians $H_{\rm S}=\tfrac12G^{\rm S}:(\Psi^\dagger\beta\Psi)^2:$, $H_{\rm V}=\tfrac12G^{\rm V}\big(:(\Psi^\dagger\Psi)^2:-\sum_j:(\Psi^\dagger\alpha^j\Psi)^2:\big)$ and $H_{\rm P}=-\tfrac14G\sum_\iota\Pi^{\iota\dagger}\Pi^\iota$, the commutators $[\Psi_e,H]$ equal $G^{\rm S}(\bar\Psi\Psi)(\gamma^0\Psi)_e$, the vector term, and $-\tfrac12G\sum_\iota(\mathsf K^{\iota\dagger}\Psi^*)_e\Pi^\iota$. The couplings of the test are $G^{\rm S}=-8$, $G^{\rm V}=4/5$ and $G=45/2$, exact numbers, so every deviation must be exactly 0. The last check of the cell derives the Yang-Mills equations with a source symbolically, with the function `EulerEquations` of the package `VariationalMethods` of the Wolfram Language, and compares all twelve equations with the form above.*)

(* ::Input:: *)
(* the annihilation operators c_e (e = 1..8) of the eight components and their adjoints, 256 x 256 sparse matrices *)
cOps = fockOps[8];
cdOps = ConjugateTranspose /@ cOps;
(* bilinear[a]: sum_ij a_ij c_i^dag c_j *)
bilinear[a_] := Sum[If[a[[i, j]] =!= 0, a[[i, j]] cdOps[[i]] . cOps[[j]], 0], {i, 8}, {j, 8}];
(* quartic[a, b]: the normal-ordered product sum_ijkl a_ij b_kl c_i^dag c_k^dag c_l c_j *)
quartic[a_, b_] := Sum[If[a[[i, j]] =!= 0 && b[[k, l]] =!= 0, a[[i, j]] b[[k, l]] cdOps[[i]] . cdOps[[k]] . cOps[[l]] . cOps[[j]], 0],
   {i, 8}, {j, 8}, {k, 8}, {l, 8}];
(* gScal, gVec, gPair: the exact couplings of the test *)
{gScal, gVec, gPair} = {-8, 4/5, 45/2};
beta8 = d8[betaLL]; alpha8 = Table[d8[alphaLL[j]], {j, 3}];
hS = gScal/2 quartic[beta8, beta8];
hV = gVec/2 (quartic[IdentityMatrix[8], IdentityMatrix[8]] - Sum[quartic[alpha8[[j]], alpha8[[j]]], {j, 3}]);
(* the pair operators Pi^iota = sum_ij K_ij c_i c_j and the pairing Hamiltonian -(G/4) sum Pi^dag Pi *)
piOps = Table[With[{k = kLL[iota]}, Sum[If[k[[i, j]] =!= 0, k[[i, j]] cOps[[i]] . cOps[[j]], 0], {i, 8}, {j, 8}]], {iota, 3}];
hP = -gPair/4 Sum[ConjugateTranspose[piOps[[k]]] . piOps[[k]], {k, 3}];
nBeta = bilinear[beta8]; nOne = bilinear[IdentityMatrix[8]]; nAlpha = Table[bilinear[alpha8[[j]]], {j, 3}];
(* lin[v] = sum_b v_b c_b *)
lin[v_] := Sum[v[[b]] cOps[[b]], {b, 8}];
{dE2, dE3, dE4} = {0, 0, 0};
Do[
  dE2 = Max[dE2, Max[Abs[Normal[cOps[[e]] . hS - hS . cOps[[e]] - gScal nBeta . lin[beta8[[e]]]]]]];
  dE3 = Max[dE3, Max[Abs[Normal[cOps[[e]] . hV - hV . cOps[[e]] - gVec (nOne . cOps[[e]] - Sum[nAlpha[[j]] . lin[alpha8[[j, e]]], {j, 3}])]]]];
  dE4 = Max[dE4, Max[Abs[Normal[cOps[[e]] . hP - hP . cOps[[e]] + gPair/2 Sum[Sum[Conjugate[kLL[k][[a, e]]] cdOps[[a]], {a, 8}] . piOps[[k]], {k, 3}]]]]],
  {e, 8}];
check["WE-2", "scalar contact term: [Psi_e, H_S] = G^S (Psi-bar Psi)(gamma^0 Psi)_e (exact, 256 states)", dE2, 0, "E-2", "E-2"];
check["WE-3", "vector contact term (exact, 256 states)", dE3, 0, "E-3", "E-3"];
check["WE-4", "pairing term: [Psi_e, H_P] = -(G/2) sum (K^dag Psi^*)_e Pi (exact, 256 states)", dE4, 0, "E-4", "E-4"];
(* E-14: the Euler-Lagrange equations of L = -(1/4) W^a_pq W^(a,pq) - g W^a_p j^(a,p), derived symbolically *)
Needs["VariationalMethods`"];
Clear[gYM];
etaD = {1, -1, -1, -1};
wF = Table[ToExpression["wf" <> ToString[a] <> ToString[p]][x0, x1, x2, x3], {a, 3}, {p, 0, 3}];
jF = Table[ToExpression["jf" <> ToString[a] <> ToString[p]][x0, x1, x2, x3], {a, 3}, {p, 0, 3}];
fYM = Table[D[wF[[a, q]], xs[[p]]] - D[wF[[a, p]], xs[[q]]] - gYM Sum[levi[a, b, c] wF[[b, p]] wF[[c, q]], {b, 3}, {c, 3}],
   {a, 3}, {p, 4}, {q, 4}];
lYM = -1/4 Sum[etaD[[p]] etaD[[q]] fYM[[a, p, q]]^2, {a, 3}, {p, 4}, {q, 4}] - gYM Sum[wF[[a, p]] jF[[a, p]], {a, 3}, {p, 4}];
eqsYM = EulerEquations[lYM, Flatten[wF], xs];
(* the claimed form: sum_p d_p F^(a,p p') - g sum eps_abc W^b_p F^(c,p p') - g J^(a,p') must equal lhs - rhs *)
fUp[c_, p_, pp_] := etaD[[p]] etaD[[pp]] fYM[[c, p, pp]];
claimedYM = Flatten[Table[Sum[D[fUp[a, p, pp], xs[[p]]], {p, 4}] - gYM Sum[levi[a, b, c] wF[[b, p]] fUp[c, p, pp], {b, 3}, {c, 3}, {p, 4}] -
      gYM jF[[a, pp]], {a, 3}, {pp, 4}]];
(* EulerEquations writes each equation as lhs == rhs; lhs - rhs is compared with the claimed form (an equation is the same
   equation when lhs - rhs equals the claimed form or its negative; the sign that occurred is reported) *)
lhsYM = eqsYM[[All, 1]] - eqsYM[[All, 2]];
signYM = Table[Which[Expand[lhsYM[[k]] - claimedYM[[k]]] === 0, 1, Expand[lhsYM[[k]] + claimedYM[[k]]] === 0, -1, True, 0], {k, Length[lhsYM]}];
checkTrue["WE-14", "symbolic Euler-Lagrange equations of the Yang-Mills Lagrangian with the source: d_p W^(a,p p') - g eps_abc W^b_p W^(c,p p') = g j^(a,p') (12 components)",
  Length[eqsYM] == 12 && FreeQ[signYM, 0], ToString[Count[signYM, Except[0]]] <> " of 12 equations identical (EulerEquations; signs " <> ToString[signYM] <> ")",
  "E-14", "E-14"];

(* ::Section:: *)
(*I.6 Four analytic solutions (item 5)*)

(* ::Text:: *)
(*Section 7 of the manuscript solves four simple cases exactly and checks the solutions numerically.*)

(* ::Text:: *)
(***AS1: plane waves in constant gauge fields.** In constant gauge fields the field equation without self-interaction has plane-wave solutions $\chi\,e^{-i\omega t+i\vec q\cdot\vec x}$, where $(\omega,\chi)$ is an eigenpair of the Hermitian matrix $\mathsf h_{\rm pw}(\vec q)=\beta_{\rm LL}m_{\rm LL}+g^{\rm F}B_0+g^{\rm v}\mathsf W_0+\sum_j\alpha^j_{\rm LL}(q_j+g^{\rm F}B_j+g^{\rm v}\mathsf W_j)$. For the spin-isospin locked field $W^\iota_{{\rm SF},j}=w\,\delta_{\iota j}$ the eight frequencies at the momentum $q$ along $x^3$ are $\pm\omega$ with the four branches $\omega^2=m^2+(q\pm gw/2)^2$ and $\omega^2=m^2+q^2+\tfrac54g^2w^2\pm gw(q^2+g^2w^2)^{1/2}$.*)

(* ::Text:: *)
(***AS2: Landau levels.** In a constant magnetic field $B$ along $x^3$ the frequencies are $(m^2+q_3^2+2neB)^{1/2}$, $n=0,1,2,\dots$; the program discretizes $x^1$ on a lattice with the Wilson term (spacings $0.1$, $0.05$, $0.025$) and compares the seven lowest positive frequencies, and their Richardson extrapolation $2E(0.025)-E(0.05)$, with the Landau levels.*)

(* ::Text:: *)
(***AS3: the uniform superfluid of the spin-zero pairing.** For a uniform unitary pair field the 16 eigenvalues of the matrix of Bogoliubov and de Gennes are $\pm[(\varepsilon\pm\mu)^2+\Delta^2]^{1/2}$, each four times; the mean-field energy is stationary where $1=(G/\pi^2)I_1((m^2+\Delta^2)^{1/2})$ with $I_1(M)=\tfrac12\big((1+M^2)^{1/2}-M^2\,\mathrm{arcsinh}(1/M)\big)$, and at $G=22.5$ the gap is $\Delta=0.2094933033$.*)

(* ::Text:: *)
(***AS4: the kink of the pair field.** For the pair field $\Delta(x)=\Delta_\infty\tanh(x/\ell)$ ($\Delta_\infty=0.5$, $\ell=4$) the reduced problem has the bound states $\mathrm{sech}^2(x/\ell)$ (zero mode, energy $\pm m$) and $\mathrm{sech}(x/\ell)\tanh(x/\ell)$ (energy $\pm(m^2+E_1^2)^{1/2}$ with $E_1^2=3/16$); on the Wilson lattice exactly three bound states lie below the continuum $(m^2+\Delta_\infty^2)^{1/2}$.*)

(* ::Text:: *)
(*The next cell computes the four cases. The eigenvalues of the Hermitian matrices are computed by the function `Eigenvalues` of the Wolfram Language (dense matrices of up to 4800 rows); the gap equation is solved by `FindRoot` with 30 digits; the profiles of AS4 are checked symbolically by `FullSimplify`.*)

(* ::Input:: *)
(* hPW[q, m, gv, gf, w, b]: the plane-wave matrix of AS1 for the constant fields w[[p+1, i]] = W^i_p and b[[p+1]] = B_p *)
hPW[q_, m_, gv_, gf_, w_, b_] := Module[{gauge},
   gauge[p_] := gf b[[p + 1]] IdentityMatrix[8] + Sum[gv w[[p + 1, i]] tLL[i], {i, 3}];
   d8[betaLL] m + gauge[0] + Sum[d8[alphaLL[j]] . (q[[j]] IdentityMatrix[8] + gauge[j]), {j, 3}]];
(* AS1-3: the spin-isospin locked field W^i_j = w delta_ij, w = 0.9, g_v = 0.83, at 61 momenta along x^3 *)
Module[{w = 0.9, gv = 0.83, wl, gw, m = N[mLL], d = 0.},
  wl = {{0, 0, 0}, {w, 0, 0}, {0, w, 0}, {0, 0, w}};
  gw = gv w;
  Do[Module[{ev = Sort[Re[Eigenvalues[N[hPW[{0, 0, q}, m, gv, 0, wl, {0, 0, 0, 0}]]]]], w2, want},
     w2 = {m^2 + (q + gw/2)^2, m^2 + (q - gw/2)^2, m^2 + q^2 + 1.25 gw^2 + gw Sqrt[q^2 + gw^2], m^2 + q^2 + 1.25 gw^2 - gw Sqrt[q^2 + gw^2]};
     want = Sort[Join[Sqrt[w2], -Sqrt[w2]]];
     d = Max[d, Max[Abs[ev - want]]]], {q, Subdivide[-1.5, 1.5, 60]}];
  check["WAS1-3", "spin-isospin locked field: four analytic branches (61 momenta)", d, 10^-12, "AS1-3", "AS1-3"]];
(* AS2: Landau levels on the Wilson lattice in x^1: eB = 0.5, q2 = 0, q3 = 0.3 *)
Module[{eb = 0.5, q2 = 0., q3 = 0.3, m = N[mLL], lev, want, vals},
  lev[n_] := Sqrt[m^2 + q3^2 + 2 n eb];
  want = {lev[0], lev[1], lev[1], lev[2], lev[2], lev[3], lev[3]};
  vals = Map[Function[na, Module[{n = na[[1]], a = na[[2]], x, h, ev},
      x = Table[-0.5 a (n - 1) + a s, {s, 0, n - 1}];
      h = SparseArray[{}, {4 n, 4 n}, 0. I];
      Do[h[[4 s + 1 ;; 4 s + 4, 4 s + 1 ;; 4 s + 4]] = alphaLL[2] (q2 - eb x[[s + 1]]) + alphaLL[3] q3 + betaLL (m + 1/a);
        If[s + 1 < n,
          With[{hop = alphaLL[1] (-0.5 I/a) - betaLL (0.5/a)},
            h[[4 s + 1 ;; 4 s + 4, 4 s + 5 ;; 4 s + 8]] = hop;
            h[[4 s + 5 ;; 4 s + 8, 4 s + 1 ;; 4 s + 4]] = ConjugateTranspose[hop]]], {s, 0, n - 1}];
      ev = Re[Eigenvalues[Normal[h]]];
      Sort[Select[ev, 0 < # <= 0.5 (lev[3] + lev[4]) &]]]], {{200, 0.1}, {400, 0.05}, {800, 0.025}}];
  check["WAS2-2", "Landau levels, Wilson lattice spacing 0.025 (largest relative deviation of the seven levels)",
    Max[Abs[vals[[3]] - want]/want], 10^-2, "AS2-2", "AS2-2"];
  check["WAS2-4", "Landau levels, Richardson extrapolation 2 E(0.025) - E(0.05)",
    Max[Abs[2 vals[[3]] - vals[[2]] - want]/want], 10^-3, "AS2-4", "AS2-4"]];
(* AS3-1: the BdG spectrum of 50 random unitary pair fields *)
Module[{d = 0., m = N[mLL]},
  Do[Module[{q = RandomReal[{-1, 1}, 3], mu = 0.5 RandomReal[{-1, 1}], dl = 0.5 (RandomReal[{-1, 1}] + 1), nv, dv, h, hm, pm, bd, eps, ep, em, want},
     nv = Normalize[RandomVariate[NormalDistribution[], 3]];
     dv = Exp[I RandomReal[{-3, 3}]] dl nv;
     h = hLL[q, m] - mu IdentityMatrix[8]; hm = hLL[-q, m] - mu IdentityMatrix[8];
     pm = Sum[dv[[i]] ConjugateTranspose[kLL[i]], {i, 3}];
     bd = ArrayFlatten[{{h, pm}, {ConjugateTranspose[pm], -Transpose[hm]}}];
     eps = Sqrt[q . q + m^2]; ep = Sqrt[(eps + mu)^2 + dl^2]; em = Sqrt[(eps - mu)^2 + dl^2];
     want = Sort[Flatten[ConstantArray[{ep, -ep, em, -em}, 4]]];
     d = Max[d, Max[Abs[Sort[Re[Eigenvalues[bd]]] - want]]]], {50}];
  check["WAS3-1", "BdG spectrum of unitary pair fields (50 random)", d, 10^-12, "AS3-1", "AS3-1"]];
(* AS3-4: the gap equation at G = 22.5, solved with 30 digits *)
i1[mm_] := (Sqrt[1 + mm^2] - mm^2 ArcSinh[1/mm])/2;
gapAS3 = Delta /. FindRoot[45/2/Pi^2 i1[Sqrt[(1/5)^2 + Delta^2]] == 1, {Delta, 2/10}, WorkingPrecision -> 30];
Print["AS3: G_crit = pi^2/I1(m) = ", N[Pi^2/i1[1/5], 12], "; Delta(G = 22.5) = ", N[gapAS3, 12]];
check["WAS3-4", "the gap at G = 22.5 agrees with the Rust value 0.2094933033", Abs[gapAS3 - 0.2094933033], 10^-9, "AS3-4", "AS3-4"];
(* AS4: the kink, reduced lattice problem, three spacings *)
Module[{dinf = 0.5, ell = 4., m = N[mLL], e1sq, thr, res, w1, ext},
  e1sq = dinf^2 - (dinf - 1/ell)^2;
  thr = Sqrt[m^2 + dinf^2];
  res = Map[Function[na, Module[{n = na[[1]], a = na[[2]], x, h, ev, av, em, e1},
      x = Table[-0.5 a (n - 1) + a s, {s, 0, n - 1}];
      h = SparseArray[{}, {2 n, 2 n}, 0. I];
      Do[h[[2 s + 1 ;; 2 s + 2, 2 s + 1 ;; 2 s + 2]] = sig[[3]] dinf Tanh[x[[s + 1]]/ell] + sig[[4]] (m + 1/a);
        If[s + 1 < n,
          With[{hop = sig[[2]] (-0.5 I/a) - sig[[4]] (0.5/a)},
            h[[2 s + 1 ;; 2 s + 2, 2 s + 3 ;; 2 s + 4]] = hop;
            h[[2 s + 3 ;; 2 s + 4, 2 s + 1 ;; 2 s + 2]] = ConjugateTranspose[hop]]], {s, 0, n - 1}];
      ev = Select[Re[Eigenvalues[Normal[h]]], -thr + 10^-3 < # < thr - 10^-3 &];
      av = Abs[ev];
      em = av[[First[Ordering[Abs[av - m], 1]]]];
      w1 = Sqrt[m^2 + e1sq];
      e1 = Mean[Select[av, Abs[# - w1] < 0.05 &]];
      {Length[ev], em, e1}]], {{600, 0.1}, {1200, 0.05}, {2400, 0.025}}];
  ext = Max[Abs[2 res[[3, 2]] - res[[2, 2]] - m]/m, Abs[2 res[[3, 3]] - res[[2, 3]] - Sqrt[m^2 + e1sq]]/Sqrt[m^2 + e1sq]];
  checkTrue["WAS4-5a", "three bound states below the continuum for each spacing", AllTrue[res[[All, 1]], # == 3 &],
    "counts " <> ToString[res[[All, 1]]], "AS4-5a", "AS4-5a"];
  check["WAS4-5b", "kink bound states, Richardson extrapolation", ext, 10^-3, "AS4-5b", "AS4-5b"]];
(* AS4-3: the analytic profiles, symbolically: (d/dx + Delta) sech^2(x/ell) = 0 and
   (-d^2 + Delta^2 - Delta') sech tanh = E1^2 sech tanh with E1^2 = 3/16 (exact Delta_inf = 1/2, ell = 4) *)
Module[{xx, dl, phi0, phi1, r0, r1},
  dl = 1/2 Tanh[xx/4];
  phi0 = Sech[xx/4]^2;
  phi1 = Sech[xx/4] Tanh[xx/4];
  r0 = FullSimplify[D[phi0, xx] + dl phi0];
  r1 = FullSimplify[-D[phi1, {xx, 2}] + (dl^2 - D[dl, xx]) phi1 - 3/16 phi1];
  checkTrue["WAS4-3", "symbolic: (d/dx + Delta) sech^2(x/ell) = 0 and (-d^2 + Delta^2 - Delta') sech tanh = (3/16) sech tanh",
    r0 === 0 && r1 === 0 && (1/2)^2 - (1/2 - 1/4)^2 == 3/16, "residuals " <> ToString[r0] <> ", " <> ToString[r1], "AS4-3", "AS4-3"]];
