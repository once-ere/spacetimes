(* ::Package:: *)

(* ::Title:: *)
(*Lord's gauge theory of gravitation and the brochon: the notebook of the checks*)

(* ::Subtitle:: *)
(*PROJECT spacetimes, folder Revision_Lord_Gravity. The notebook lord_gravity.nb, prepared for Patrick Lee Nash, Ph.D.*)

(* ::Text:: *)
(***What this notebook is.** The folder `Revision_Lord_Gravity` holds a manuscript, `lord_gravity.md`, that reads E. A. Lord's paper "General relativity from gauge invariance" (Proceedings of the Cambridge Philosophical Society 69, 423 (1971)), relates his gauge group to the group $SL(2,\mathbb C)$ of the project, couples the spinor field of the project, the brochon $\Psi_{\rm LL}$, to gravitation, and solves its equations around black holes. Its statements are checked by programs in three languages: Rust, Python and the Wolfram Language. The nine scripts in the Wolfram Language make 46 checks, most of them in exact arithmetic. This notebook runs all of them, in the order of the manuscript, and explains before each check what it decides and why it matters. A student who works through it sees every algebraic identity of the manuscript being verified, and can change the inputs and watch what happens.*)

(* ::Text:: *)
(***How to use it.** Open the notebook in Wolfram (version 15 or later; it was evaluated with Wolfram 15.0.1) and choose Evaluation > Evaluate Notebook. The cells are evaluated in order; the whole notebook takes about a minute. Nothing outside the notebook is read: the three data files that some checks compare with are written into closed cells (a closed cell shows only a thin bracket; double-click the bracket to open it). Every check prints one line that begins with PASS or FAIL, and the last chapter lists all checks.*)

(* ::Text:: *)
(***How to read it.** A text cell explains, in words and formulas, what the next input cell computes. The formulas in the texts use the symbols of the manuscript; the code uses short names of its own, which the comments in the code and the texts name (for example `sig` for the four matrices $\sigma^{\rm LL}_\mu$). A symbol with the suffix LL is an object of the Lorentz super-loop and of its fields, SF an object of the brochon model and of its superfluid ground state, LG an object that this folder introduces (the gravitational field, the black holes and their problems), and Lord an object of Lord's paper in his own notation.*)

(* ::Text:: *)
(***The checks and their counterparts.** Each check has an identifier: WL for Lord's paper, WG for the curved space-time, WS for the Schwarzschild field, WV for the Vaidya field, WP, WC, WT and WA for the four computations of the pair and the black holes, WW for the worked examples. The line that a check prints names, in parentheses, the checks of the Rust program and of the Python program of the folder that test the same statement in floating point, so that a reader without a Wolfram license can follow every step with those programs (folders `rust/` and `python/` of the folder). The notebook runs the code of the scripts in `wolfram/source/` unchanged; each chapter runs in a context of its own (for example ``LGNB`Lord` ``), so that the chapters, which were written as separate programs, cannot change each other's symbols.*)

(* ::Text:: *)
(***What the notebook does not repeat.** The long numerical computations of the manuscript (the evolutions in time of Parts V and VI, which take minutes to forty minutes) are done by the Rust programs with the integrator CVODE of SUNDIALS; they are not repeated here. The notebook repeats only those numbers that a Wolfram script recomputes independently: three quasi-bound states and three greybody factors of Part IV (checks WS-7 and WS-8), found again in arithmetic of 32 digits.*)

(* ::Chapter:: *)
(*0. The register of the checks*)

(* ::Text:: *)
(*The next cell defines the functions that record and print a check. `check[id, text, ok, detail]` records a check that is decided by facts, such as an exact identity: it passes when `ok` is `True`, and `detail` states the facts. `checkDev[id, text, dev, tol]` records a numerical check: it passes when the measured deviation `dev` is at most the tolerance `tol`. Both append an association to the list ``LGNB`Test`$Results``, the register of all checks, and print the same line that the scripts print into their reports (the files `data/hck_lg_*_checks_wolfram.txt` of the folder). The function `fmt` writes a number with four significant digits in the form `2.857e-8`.*)

(* ::Input:: *)
(* the register of the checks of this notebook: one association per check, in the order of evaluation *)
LGNB`Test`$Results = {};
(* fmt[x, nd]: a real number written d.ddde-x with nd significant digits, as the reports of the scripts write it (the
   function fmt of the script lg_schw.wls, with other names for its variables, so that no common name such as x is
   created in the context Global`) *)
fmt[lgX_, lgNd_ : 4] := Module[{lgE, lgMm, lgY = N[lgX]},
  If[lgY == 0, "0", lgE = Floor[Log10[Abs[lgY]]]; lgMm = Round[Abs[lgY] 10^(lgNd - 1 - lgE)];
   If[lgMm >= 10^lgNd, lgMm = Round[lgMm/10]; lgE = lgE + 1];
   StringJoin[If[lgY < 0, "-", ""], ToString[Quotient[lgMm, 10^(lgNd - 1)]], ".",
    IntegerString[Mod[lgMm, 10^(lgNd - 1)], 10, lgNd - 1], "e", ToString[lgE]]]];
(* checkHook[]: called after every check; it does nothing here (the dry run of the code without the front end uses it
   to print the computing time of each check) *)
checkHook[] := Null;
(* check[id, text, ok, detail]: a check decided by facts; it passes when ok is True *)
check[lgId_String, lgText_String, lgOk_, lgDetail_String] := Module[{lgPass = TrueQ[lgOk], lgLine},
  lgLine = StringJoin[If[lgPass, "PASS ", "FAIL "], StringPadRight[lgId, 9], lgText, " [", lgDetail, "]"];
  AppendTo[LGNB`Test`$Results, <|"id" -> lgId, "pass" -> lgPass, "deviation" -> Missing["NotNumerical"],
     "tolerance" -> Missing["NotNumerical"], "line" -> lgLine|>];
  Print[lgLine]; checkHook[]; lgPass];
(* checkDev[id, text, dev, tol]: a numerical check; it passes when the deviation dev is at most the tolerance tol *)
checkDev[lgId_String, lgText_String, lgDev_, lgTol_] := Module[{lgPass = TrueQ[lgDev <= lgTol], lgLine},
  lgLine = StringJoin[If[lgPass, "PASS ", "FAIL "], StringPadRight[lgId, 9], "deviation ", fmt[lgDev], " (tolerance ",
    fmt[lgTol, 2], "): ", lgText];
  AppendTo[LGNB`Test`$Results, <|"id" -> lgId, "pass" -> lgPass, "deviation" -> N[lgDev], "tolerance" -> N[lgTol],
     "line" -> lgLine|>];
  Print[lgLine]; checkHook[]; lgPass];

(* ::Chapter:: *)
(*1. Lord's paper and the association (Parts I and II of the manuscript)*)

(* ::Text:: *)
(*Lord writes a four-vector $v^\mu$ as the $2\times2$ Hermitian matrix $v^\mu\sigma^{\rm LL}_\mu$, where $\sigma^{\rm LL}_0$ is the unit matrix and $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3$ are the Pauli matrices. Its determinant is the Minkowski square $v_\mu v^\mu$. His central observation is that the four matrices need not be the Pauli matrices: any four Hermitian matrices whose products obey the same rules will do, and changing from one such set to another is a change of representation. Requiring that the physics does not depend on this choice, at every point separately, is a gauge principle, and its gauge field turns out to be gravitation. This chapter checks, in exact arithmetic, the identities of his Section 1 (the algebra of the matrices) and the signs and factors of his Section 4 (the gauge transformation), where several printed formulas do not follow from his definitions; three of them are checked here.*)

(* ::Text:: *)
(*The **bar** of a $2\times2$ matrix is $\bar X=-\mathsf E_{\rm LL}X^T\mathsf E_{\rm LL}$, with $\mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ (Lord's $\epsilon$); for an invertible matrix it is $\det X$ times $X^{-1}$. The products $\sigma^{\mu\nu}_{\rm LG}=\tfrac12(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu-\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu)$ generate the Lorentz transformations on two-component spinors, and the Dirac matrices are the $4\times4$ matrices $\gamma^\mu=\begin{pmatrix}0&\sigma^{\rm LL}_\mu\\\bar\sigma^{\rm LL}_\mu&0\end{pmatrix}$ (the Weyl form). In the code: `sig` for the four $\sigma^{\rm LL}_\mu$, `eps` for $\mathsf E_{\rm LL}$, `bar` for the bar, `smn` for $\sigma^{\mu\nu}_{\rm LG}$, `gam` and `gm` for the Dirac matrices, `g5` for Lord's $\gamma^5$, `cc` for his charge-conjugation matrix $C$. The indices 1 to 4 of the code are the indices 0 to 3 of the texts.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Lord` (the code is that of the script wolfram/source/lg_lord.wls) *)
Begin["LGNB`Lord`"];

(* ::Text:: *)
(*The helper function `zeroQ[m]` decides whether every entry of a matrix (or a list of matrices) simplifies to zero.*)

(* ::Input:: *)
(* the helper functions of the script lg_lord.wls *)
zeroQ[m_] := AllTrue[Flatten[{Simplify[m]}], TrueQ[# == 0] &];

(* ::Text:: *)
(*Lord's matrices, exactly as he defines them.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ Lord's matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
eps = {{0, 1}, {-1, 0}};
eta = DiagonalMatrix[{1, -1, -1, -1}];
bar[s_] := -eps . Transpose[s] . eps;
smn[s_, m_, n_] := (s[[m]] . bar[s[[n]]] - s[[n]] . bar[s[[m]]])/2;
gam[s_] := ArrayFlatten[{{0, s}, {bar[s], 0}}];
g5 = DiagonalMatrix[{1, 1, -1, -1}];
cc = ArrayFlatten[{{eps, 0}, {0, -eps}}];
gmn[g_, m_, n_] := (g[[m]] . g[[n]] - g[[n]] . g[[m]])/2;
levi[a_, b_, c_, d_] := Signature[{a, b, c, d}];   (* eps_0123 = +1 *)
gm = gam /@ sig;

(* ::Section:: *)
(*WL-1: the identities of Lord's Section 1*)

(* ::Text:: *)
(*Lord's Section 1 lists the rules that his matrices obey: the bar of $\sigma^{\rm LL}_\mu$ (1.2); the relation $\tfrac12(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu+\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu)=\eta^{\rm LL}_{\mu\nu}\mathsf 1_2$ (1.3), which says that the matrices carry the Minkowski metric; the commutators of the generators $\sigma^{\mu\nu}_{\rm LG}$ (1.5), which are those of the Lorentz algebra; their self-duality (1.7); the Clifford relation $\tfrac12\{\gamma^\mu,\gamma^\nu\}=\eta^{\rm LL}_{\mu\nu}\mathsf 1_4$ (1.10); the properties of $\gamma^5$ and of the charge-conjugation matrix (1.11) to (1.14); and the block form of $\gamma^{\mu\nu}$ (1.15) and its duality (1.17). The cell evaluates every one of them for all index combinations (for (1.5) these are $4^4=256$ combinations), in exact arithmetic. These are the rules that every later computation uses, so a single wrong sign here would spoil everything after it.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-1 *)
r12 = And @@ Table[bar[sig[[m]]] == If[m == 1, sig[[1]], -sig[[m]]], {m, 4}];
r13 = And @@ Flatten@Table[(sig[[m]] . bar[sig[[n]]] + sig[[n]] . bar[sig[[m]]])/2 == eta[[m, n]] IdentityMatrix[2], {m, 4}, {n, 4}];
r15 = And @@ Flatten@Table[
     smn[sig, m, n] . smn[sig, r, t] - smn[sig, r, t] . smn[sig, m, n] ==
      2 (eta[[n, r]] smn[sig, m, t] - eta[[m, r]] smn[sig, n, t] + eta[[m, t]] smn[sig, n, r] - eta[[n, t]] smn[sig, m, r]),
     {m, 4}, {n, 4}, {r, 4}, {t, 4}];
r17 = And @@ Flatten@Table[eta[[m, m]] eta[[n, n]] smn[sig, m, n] == Sum[I/2 levi[m, n, r, t] smn[sig, r, t], {r, 4}, {t, 4}], {m, 4}, {n, 4}];
r110 = And @@ Flatten@Table[(gm[[m]] . gm[[n]] + gm[[n]] . gm[[m]])/2 == eta[[m, n]] IdentityMatrix[4], {m, 4}, {n, 4}];
r111 = And @@ Table[gm[[m]] . g5 + g5 . gm[[m]] == ConstantArray[0, {4, 4}], {m, 4}];
r112 = g5 . g5 == IdentityMatrix[4];
r113 = g5 == -I gm[[1]] . gm[[2]] . gm[[3]] . gm[[4]] &&
   g5 == -(I/24) Sum[levi[a, b, c, d] gm[[a]] . gm[[b]] . gm[[c]] . gm[[d]], {a, 4}, {b, 4}, {c, 4}, {d, 4}];
r114 = And @@ Table[cc . gm[[m]] . Inverse[cc] == -Transpose[gm[[m]]], {m, 4}];
r115 = And @@ Flatten@Table[gmn[gm, m, n] == ArrayFlatten[{{smn[sig, m, n], 0}, {0, -ConjugateTranspose[smn[sig, m, n]]}}], {m, 4}, {n, 4}];
r117 = And @@ Flatten@Table[
     With[{lo = eta[[m, m]] eta[[n, n]] gmn[gm, m, n], du = Sum[I/2 levi[m, n, r, t] gmn[gm, r, t], {r, 4}, {t, 4}]},
      g5 . lo == du && lo . g5 == du], {m, 4}, {n, 4}];
check["WL-1", "(Rust L1-1 to L1-4, L1-7 to L1-10; Python YL-1) Lord's identities (1.2), (1.3), (1.5) (256 index \
combinations), (1.7), (1.10), (1.11), (1.12), (1.13) (both forms), (1.14), (1.15) and (1.17) hold exactly for his matrices",
  r12 && r13 && r15 && r17 && r110 && r111 && r112 && r113 && r114 && r115 && r117,
  "(1.2) " <> ToString[r12] <> ", (1.3) " <> ToString[r13] <> ", (1.5) " <> ToString[r15] <> ", (1.7) " <> ToString[r17] <>
   ", (1.10)-(1.14) " <> ToString[r110 && r111 && r112 && r113 && r114] <> ", (1.15) " <> ToString[r115] <> ", (1.17) " <> ToString[r117]];

(* ::Section:: *)
(*WL-2 and WL-3: matrices of determinant 1 and the polarized determinant*)

(* ::Text:: *)
(*A change of representation is a $2\times2$ matrix $S_{\rm Lord}$ of determinant 1. WL-2 checks, for a matrix of determinant 1 with symbolic entries, the two rules that make the bar behave well under such a change: $\mathsf E_{\rm LL}S=(S^T)^{-1}\mathsf E_{\rm LL}$, and $\overline{S\sigma S'}=S'^{-1}\bar\sigma S^{-1}$ for any matrix $\sigma$ and a second matrix $S'$ of determinant 1; with $S'=S^\dagger$ this is Lord's (1.23).*)

(* ::Text:: *)
(*WL-3 checks the identity behind the word "Minkowski product of two matrices": for any two $2\times2$ matrices $A$ and $B$, $\tfrac12(A\bar B+B\bar A)=N_{\rm LG}(A,B)\,\mathsf 1_2$, where $N_{\rm LG}(A,B)=\tfrac12(\det(A+B)-\det A-\det B)$ is the polarized determinant. So a set of four Hermitian matrices satisfies Lord's (1.3) exactly when their polarized determinants are $\eta^{\rm LL}_{\mu\nu}$; this is how the manuscript recognizes a valid set (an orthonormal frame) by a simple computation.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-2 *)
sS = {{a1, b1}, {c1, (1 + b1 c1)/a1}};            (* det = 1 *)
sP = {{a2, b2}, {c2, (1 + b2 c2)/a2}};            (* a second matrix of determinant 1 *)
sigGen = {{s11, s12}, {s21, s22}};                (* any 2x2 matrix *)
okEps = zeroQ[eps . sS - Inverse[Transpose[sS]] . eps];
ok123 = zeroQ[bar[sS . sigGen . sP] - Inverse[sP] . bar[sigGen] . Inverse[sS]];
check["WL-2", "(Rust L1-11a, L1-11b; Python YL-3) for S of determinant 1 (symbolic entries): epsilon S = (S^T)^{-1} \
epsilon; and for any matrix sigma and a second S' of determinant 1, bar(S sigma S') = S'^{-1} bar(sigma) S^{-1}; with \
S' = S^dag this is (1.23)", okEps && ok123, "epsilon " <> ToString[okEps] <> ", (1.23) " <> ToString[ok123]];

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-3 *)
mA = {{p11, p12}, {p21, p22}}; mB = {{q11, q12}, {q21, q22}};
nAB = (Det[mA + mB] - Det[mA] - Det[mB])/2;
ok3 = zeroQ[(mA . bar[mB] + mB . bar[mA])/2 - nAB IdentityMatrix[2]];
okN = And @@ Flatten@Table[(Det[sig[[m]] + sig[[n]]] - Det[sig[[m]]] - Det[sig[[n]]])/2 == eta[[m, n]], {m, 4}, {n, 4}];
check["WL-3", "(Rust L1-13, L1-14; Python YL-3, YL-3b) for any two 2x2 matrices A, B (symbolic entries): 1/2 (A bar(B) + \
B bar(A)) = N(A, B) 1 with N(A, B) = (det(A + B) - det A - det B)/2, and N(sigma^mu, sigma^nu) = eta^{mu nu}; so (1.3) \
holds for a set of four matrices exactly when their products N are eta^{ab}", ok3 && okN,
  "identity " <> ToString[ok3] <> ", N of the standard set " <> ToString[okN]];

(* ::Section:: *)
(*WL-4: an infinitesimal Lorentz transformation*)

(* ::Text:: *)
(*Lord's (1.8) says that the spinor matrix $L=\mathsf 1_2+\tfrac14\lambda_{\mu\nu}\sigma^{\mu\nu}_{\rm LG}$ and the Lorentz matrix $\Lambda_\mu{}^\nu=\delta_\mu^\nu+\eta^{\nu\rho}\lambda_{\rho\mu}$ belong together: transforming the matrices $\sigma^{\rm LL}_\mu$ by $L$ is the same as transforming their index by $\Lambda$. The check takes the six parameters $\lambda_{\mu\nu}$ as real symbols and verifies the relation to first order in them, where it is a linear identity.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-4 *)
lam = {{0, l01, l02, l03}, {-l01, 0, l12, l13}, {-l02, -l12, 0, l23}, {-l03, -l13, -l23, 0}};  (* lambda_{mu nu}, real *)
xx = Sum[lam[[a, b]]/4 smn[sig, a, b], {a, 4}, {b, 4}];
xxd = ConjugateTranspose[xx] /. Conjugate[z_] :> z;                   (* the lambda are real *)
ok4 = And @@ Table[zeroQ[xx . sig[[m]] + sig[[m]] . xxd + Sum[eta[[m, m]] lam[[m, n]] sig[[n]], {n, 4}]], {m, 4}];
check["WL-4", "(Rust L1-5; Python YL-2) (1.8) to first order: with Lambda_mu^nu = delta_mu^nu + eta^{nu rho} \
lambda_{rho mu} and L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu} (six real symbols), eta^{mu rho} lambda_{rho nu} sigma^nu + \
X sigma^mu + sigma^mu X^dag = 0 for every mu", ok4, ToString[ok4]];

(* ::Section:: *)
(*WL-5: the sign of the gauge transformation (Lord's Section 4)*)

(* ::Text:: *)
(*When the change of representation $S(x)$ depends on the point, the derivative of a spinor is no longer covariant, and Lord adds an auxiliary field $\Gamma$, as in electrodynamics: $D=\partial-\Gamma$. For $D\psi$ to transform like $\psi$, the field must transform as $\Gamma'=S\Gamma S^{-1}+(\partial S)S^{-1}$. Lord prints the inhomogeneous term as $S\,\partial(S^{-1})$, which is its negative. The check takes an explicit $S(t)$ of determinant 1, a constant $\Gamma$ in the Lorentz algebra with symbolic entries, and a spinor of four symbolic functions, and shows that the corrected law makes $D\psi$ covariant and the printed one does not. Section 7 of the manuscript records this correction and those of the next two checks.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-5 *)
sT = {{1 + t^3, t}, {t^2, 1}};                                         (* det = 1 for every real t *)
s4[m_] := ArrayFlatten[{{m, 0}, {0, Inverse[Transpose[m]]}}];          (* diag(S, (S^dag)^{-1}); S real for real t *)
yY = {{y1, y2}, {y3, -y1}}; yYd = {{yc1, yc3}, {yc2, -yc1}};          (* Y and Y^dag with independent symbols *)
gC = ArrayFlatten[{{yY, 0}, {0, -yYd}}];                               (* a constant element of the Lorentz algebra *)
psiT = {f1[t], f2[t], f3[t], f4[t]};
tt = s4[sT]; tti = Inverse[tt];
rhs5 = tt . (D[psiT, t] - gC . psiT);
gOK = tt . gC . tti + D[tt, t] . tti;
gPR = tt . gC . tti + tt . D[tti, t];
okCov = zeroQ[D[tt . psiT, t] - gOK . (tt . psiT) - rhs5];
badPR = Not[zeroQ[D[tt . psiT, t] - gPR . (tt . psiT) - rhs5]];
check["WL-5", "(Rust L4-2, L4-3; Python YL-4, YL-4b) for S(t) = ((1 + t^3, t), (t^2, 1)) (determinant 1), a constant \
Gamma = diag(Y, -Y^dag) of the Lorentz algebra and a spinor of four symbolic functions: D = d - Gamma is covariant with \
Gamma' = S Gamma S^{-1} + (dS) S^{-1}, and not with the printed inhomogeneous term S (d S^{-1})", okCov && badPR,
  "corrected law covariant " <> ToString[okCov] <> ", printed law not covariant " <> ToString[badPR]];

(* ::Section:: *)
(*WL-6: the factor 2 of the connection of the vectors*)

(* ::Text:: *)
(*A spinor connection $\Gamma$ induces a connection $C$ on vectors through the commutator with the Dirac matrices: $[\Gamma,\gamma_b]$ is a combination of the $\gamma_e$, and its coefficients are $C$. In Lord's special gauge (constant Dirac matrices) with $\Gamma=\tfrac12g\,a_{bc}\gamma^{bc}$, the check computes the coefficients for six symbolic $a_{bc}$ and finds $C=2g\,a$, twice the value that Lord prints in (4.20) and (5.2).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-6 *)
aa = {{0, a01, a02, a03}, {-a01, 0, a12, a13}, {-a02, -a12, 0, a23}, {-a03, -a13, -a23, 0}};  (* a_{bc} of one a *)
gUp = gm;                                                               (* constant gamma^a: the special gauge *)
gLo = Table[eta[[b, b]] gUp[[b]], {b, 4}];
gConn = Sum[gg/2 aa[[b, c]] gmn[gUp, b, c], {b, 4}, {c, 4}];
cSol = Table[Simplify[Tr[(-(gConn . gLo[[b]] - gLo[[b]] . gConn)) . gUp[[e]]]/4], {b, 4}, {e, 4}];   (* C_b^e of (4.19), d gamma = 0 *)
ok6 = zeroQ[cSol - Table[2 gg aa[[b, e]] eta[[e, e]], {b, 4}, {e, 4}]];
okSpan = And @@ Table[zeroQ[-(gConn . gLo[[b]] - gLo[[b]] . gConn) - Sum[cSol[[b, e]] gLo[[e]], {e, 4}]], {b, 4}];
check["WL-6", "(Rust L4-6b, L4-6c; Python YL-6, YL-6b) in Lord's special gauge (constant gamma, (5.2)): for Gamma = 1/2 g \
a_bc gamma^{bc} with six symbolic a_bc, [Gamma, gamma_b] = -2 g a_b^e gamma_e, so (4.19) gives C_b^e = 2 g a_b^e: \
twice the printed (4.20) and (5.2), C = g a", ok6 && okSpan, "C = 2 g a: " <> ToString[ok6] <> ", in the span of gamma_e: " <> ToString[okSpan]];

(* ::Section:: *)
(*WL-7: the curvature of the spinor connection*)

(* ::Text:: *)
(*The map $\lambda(W)=\tfrac14W_{ab}\gamma^{ab}$ takes an element of the Lorentz algebra (an antisymmetric $W_{ab}$) to a $4\times4$ matrix that acts on Dirac spinors. The check shows, for symbolic $W$, that it acts on the Dirac matrices as the Lorentz algebra acts on vectors, and that it preserves commutators: $[\lambda(W_1),\lambda(W_2)]=\lambda([W_1,W_2])$, a homomorphism of Lie algebras. It follows that the curvature of the spinor connection is $\tfrac14R_{ab\mu\nu}\gamma^{ab}$, where $R$ is the Riemann tensor; Lord prints $\tfrac12R\gamma$, twice this. His two factors 2 (this one and that of WL-6) compensate each other only to first order in the fields.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WL-7 *)
wA = {{0, u01, u02, u03}, {-u01, 0, u12, u13}, {-u02, -u12, 0, u23}, {-u03, -u13, -u23, 0}};   (* W_{ab} *)
wB = {{0, v01, v02, v03}, {-v01, 0, v12, v13}, {-v02, -v12, 0, v23}, {-v03, -v13, -v23, 0}};
lambdaOf[w_] := Sum[w[[a, b]]/4 gmn[gUp, a, b], {a, 4}, {b, 4}];
mixed[w_] := w . eta;                                                   (* (W)_b^e = W_{b f} eta^{f e} *)
okAct = And @@ Table[zeroQ[(lambdaOf[wA] . gLo[[b]] - gLo[[b]] . lambdaOf[wA]) + Sum[mixed[wA][[b, e]] gLo[[e]], {e, 4}]], {b, 4}];
comW = mixed[wA] . mixed[wB] - mixed[wB] . mixed[wA];                   (* the commutator, as a mixed matrix *)
comWlow = comW . eta;                                                   (* back to lower indices: (.)_{b f} = (.)_b^e eta_{e f} *)
okHom = zeroQ[lambdaOf[wA] . lambdaOf[wB] - lambdaOf[wB] . lambdaOf[wA] - lambdaOf[comWlow]];
check["WL-7", "(Rust L4-10a, L4-10b, L4-10c; Python YL-7b, YL-7c) lambda(W) = 1/4 W_{ab} gamma^{ab} (W antisymmetric, \
symbolic) satisfies [lambda(W), gamma_b] = -W_b^e gamma_e and [lambda(W1), lambda(W2)] = lambda([W1, W2]): a homomorphism \
of Lie algebras, so the curvature of the spinor connection lambda(W_mu) is lambda(R_{mu nu}) = 1/4 R_{mu nu ab} gamma^{ab} \
exactly, and Lord's printed 1/2 R gamma is twice it", okAct && okHom,
  "action on gamma " <> ToString[okAct] <> ", homomorphism " <> ToString[okHom]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*2. The brochon in a curved space-time (Part III)*)

(* ::Text:: *)
(*Part III turns Lord's reference vectors into the tetrad of a curved space-time: four orthonormal vectors $e^{\rm LG}_a{}^\mu$ at each point, from which the metric $g^{\rm LG}_{\mu\nu}=e^{{\rm LG}\,a}{}_\mu\eta^{\rm LL}_{ab}e^{{\rm LG}\,b}{}_\nu$ follows. The spin connection $\omega^{\rm LG}_{\mu\,ab}$ is fixed by requiring that the tetrad be covariantly constant (the tetrad postulate), and the spinor connection is $\Gamma^{\rm LL}_\mu=\tfrac14\omega^{\rm LG}_{\mu\,ab}\gamma^{ab}_{\rm LL}$. With it the brochon obeys the Dirac equation $i\gamma^\mu_{\rm LG}(\partial_\mu+\Gamma^{\rm LL}_\mu)\Psi_{\rm LL}=m_{\rm LL}\Psi_{\rm LL}$ in any gravitational field. This chapter checks the geometry for the space-times that the later parts use, the square of the Dirac operator, the energy-momentum tensor of the brochon, and a property of the pairing term of the superfluid.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Curved` (the code is that of the script wolfram/source/lg_curved.wls) *)
Begin["LGNB`Curved`"];

(* ::Text:: *)
(*Four helper functions: `zeroQ` (every entry simplifies to zero), `finiteQ` (no infinity in an expression), `zeroE` (every entry expands to zero, which decides identities between polynomials in symbolic functions quickly), and `witnessQ` (an expression is not identically zero, shown by its value for explicit functions at one rational point; it is used for the controls, the statements that a wrong formula fails).*)

(* ::Input:: *)
(* the helper functions of the script lg_curved.wls *)
zeroQ[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[#] == 0] &];
finiteQ[m_] := FreeQ[Flatten[{m}], ComplexInfinity | Indeterminate | DirectedInfinity[_] | Infinity];
(* zeroE: the expanded form is 0 (for polynomials in independent functions and their derivatives, with exponential
   factors, the expanded form is canonical, so this decides the identity exactly and quickly); witnessQ: an expression
   is not identically zero, shown by its value for explicit functions at a rational point *)
zeroE[m_] := AllTrue[Flatten[{m}], Expand[#] === 0 &];
witnessQ[m_, rules_, pt_] := AnyTrue[Flatten[{m}], Abs[N[(# /. rules) /. pt, 30]] > 10^-20 &];

(* ::Text:: *)
(*The Dirac matrices of the chiral basis (the Weyl form), `gam`, the metric `eta` and the products `gab` $=\gamma^{ab}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ the matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
gam = Table[ArrayFlatten[{{0, sig[[a]]}, {sigbar[[a]], 0}}], {a, 4}];   (* gamma^a of the chiral basis *)
eta = DiagonalMatrix[{1, -1, -1, -1}];
gab[a_, b_] := (gam[[a]] . gam[[b]] - gam[[b]] . gam[[a]])/2;
id4 = IdentityMatrix[4];

(* ::Text:: *)
(*The function `geometry[E, X, simp]` takes a coframe, the matrix `E` with entries $e^{{\rm LG}\,a}{}_\mu$ in the coordinates `X`, and returns the tetrad (`F`, the inverse transpose), the metric, the Christoffel symbols, the spin connection of the tetrad postulate, the Dirac matrices $\gamma^\mu_{\rm LG}=e^{\rm LG}_a{}^\mu\gamma^a$ and the spinor connection $\tfrac14\omega_{\mu\,ab}\gamma^{ab}$. The functions `riemann`, `ricci` and `christoffel` compute the curvature in the conventions of Landau and Lifshitz; `nablaGamma` is the covariant derivative of the Dirac matrices, which must vanish, and `spinCurvDev` is the difference between the curvature of the spinor connection and $\tfrac14R_{ab\mu\nu}\gamma^{ab}$, which must vanish too. Then the assumptions $r>2M$, $M>0$, $0<\theta<\pi$ are set for `Simplify` (they hold in this chapter only).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ the geometry of a coframe *)
(* E[[a, mu]] = e^a_mu; F[[a, mu]] = e_a^mu (Lord's reference vectors); Christoffel symbols; the spin
   connection of the tetrad postulate om[[mu, a, b]] = omega_mu^a_b; the Dirac matrices gamma^mu; the
   spinor connection Omega_mu = 1/4 omega_{mu ab} gamma^{ab} *)
geometry[E_, X_, simp_] := Module[{F, g, gi, chr, om, gU, spc},
  F = simp[Transpose[Inverse[E]]];
  g = simp[Transpose[E] . eta . E];
  gi = simp[Transpose[F] . eta . F];
  chr = simp[Table[1/2 Sum[gi[[rr, l]] (D[g[[l, n]], X[[m]]] + D[g[[l, m]], X[[n]]] - D[g[[m, n]], X[[l]]]),
     {l, 4}], {rr, 4}, {m, 4}, {n, 4}]];
  om = simp[Table[Sum[E[[a, n]] (D[F[[b, n]], X[[m]]] + Sum[chr[[n, m, l]] F[[b, l]], {l, 4}]), {n, 4}],
     {m, 4}, {a, 4}, {b, 4}]];
  gU = Table[Sum[F[[a, m]] gam[[a]], {a, 4}], {m, 4}];
  spc = simp[Table[1/4 Sum[eta[[a, a]] om[[m, a, b]] gab[a, b], {a, 4}, {b, 4}], {m, 4}]];
  <|"E" -> E, "F" -> F, "g" -> g, "gi" -> gi, "chr" -> chr, "om" -> om, "gam" -> gU, "spc" -> spc, "X" -> X|>];
(* the Riemann tensor R^rho_{sigma mu nu} (Landau-Lifshitz) of Christoffel symbols *)
riemann[chr_, X_, simp_] := simp[Table[D[chr[[rr, n, s]], X[[m]]] - D[chr[[rr, m, s]], X[[n]]] +
     Sum[chr[[rr, m, l]] chr[[l, n, s]] - chr[[rr, n, l]] chr[[l, m, s]], {l, 4}], {rr, 4}, {s, 4}, {m, 4}, {n, 4}]];
ricci[riem_] := Table[Sum[riem[[rr, s, rr, n]], {rr, 4}], {s, 4}, {n, 4}];
christoffel[g_, X_, simp_] := Module[{gi = simp[Inverse[g]]},
  simp[Table[1/2 Sum[gi[[rr, l]] (D[g[[l, n]], X[[m]]] + D[g[[l, m]], X[[n]]] - D[g[[m, n]], X[[l]]]), {l, 4}],
    {rr, 4}, {m, 4}, {n, 4}]]];
(* the covariant constancy of the Dirac matrices and the spinor curvature *)
nablaGamma[geo_] := Table[D[geo["gam"][[n]], geo["X"][[m]]] + Sum[geo["chr"][[n, m, l]] geo["gam"][[l]], {l, 4}] +
    geo["spc"][[m]] . geo["gam"][[n]] - geo["gam"][[n]] . geo["spc"][[m]], {m, 4}, {n, 4}];
spinCurvDev[geo_, riem_] := Table[D[geo["spc"][[n]], geo["X"][[m]]] - D[geo["spc"][[m]], geo["X"][[n]]] +
    geo["spc"][[m]] . geo["spc"][[n]] - geo["spc"][[n]] . geo["spc"][[m]] -
    1/4 Sum[eta[[a, a]] Sum[geo["E"][[a, rr]] geo["F"][[b, s]] riem[[rr, s, m, n]], {rr, 4}, {s, 4}] gab[a, b],
      {a, 4}, {b, 4}], {m, 4}, {n, 4}];

$Assumptions = {r > 2 M, M > 0, 0 < th < Pi};
simpA = Simplify;

(* ::Section:: *)
(*WG-1: the Schwarzschild space-time in the static tetrad*)

(* ::Text:: *)
(*The Schwarzschild black hole of the mass $M_{\rm LG}$ has the metric $f_{\rm LG}\,dt^2-f_{\rm LG}^{-1}dr^2-r^2d\Omega^2$ with $f_{\rm LG}=1-2\mathcal M_{\rm LG}/r$, where $\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ is the mass as a length (units $c=1$; in the code $G_{\rm N}=1$, the mass `M` and `fS` for $f_{\rm LG}$). The static tetrad is the diagonal one, $e^0=f_{\rm LG}^{1/2}dt$, $e^1=f_{\rm LG}^{-1/2}dr$, $e^2=r\,d\theta$, $e^3=r\sin\theta\,d\varphi$. The check computes from it: the Ricci tensor (zero: the space-time is empty), the Kretschmann invariant $R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}=48\mathcal M_{\rm LG}^2/r^6$ (finite at the horizon $r=2\mathcal M_{\rm LG}$, infinite at $r=0$: the horizon is not a singularity of the geometry, the centre is), the spinor connection $\Gamma^{\rm LL}_\mu$ in closed form, the covariant constancy of the Dirac matrices, and the curvature of the spinor connection, $\tfrac14R^{\rm LG}_{ab\mu\nu}\gamma^{ab}_{\rm LL}$. The last is checked for every static, spherically symmetric tetrad with an arbitrary function $h_{\rm LG}(r)$ in place of $f_{\rm LG}^{1/2}$, which avoids square roots that `Simplify` does not always remove.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WG-1: Schwarzschild, diagonal tetrad *)
fS = 1 - 2 M/r;
XS = {t, r, th, ph};
geoS = geometry[DiagonalMatrix[{Sqrt[fS], 1/Sqrt[fS], r, r Sin[th]}], XS, simpA];
riemS = riemann[geoS["chr"], XS, simpA];
okRicS = zeroQ[ricci[riemS]];
rlowS = Simplify[Table[Sum[geoS["g"][[a, k]] riemS[[k, b, c, d]], {k, 4}], {a, 4}, {b, 4}, {c, 4}, {d, 4}]];
rupS = Simplify[Table[Sum[geoS["gi"][[b, q]] geoS["gi"][[c, s]] geoS["gi"][[d, u]] riemS[[a, q, s, u]], {q, 4}, {s, 4},
     {u, 4}], {a, 4}, {b, 4}, {c, 4}, {d, 4}]];
kretS = Simplify[Total[Flatten[rlowS rupS]]];
okKretS = zeroQ[kretS - 48 M^2/r^6];
omegaS = {M/(2 r^2) gam[[1]] . gam[[2]], 0 id4, Sqrt[fS]/2 gam[[2]] . gam[[3]],
   (Sqrt[fS] Sin[th] gam[[2]] . gam[[4]] + Cos[th] gam[[3]] . gam[[4]])/2};
okOmS = zeroQ[geoS["spc"] - omegaS];
okNgS = zeroQ[nablaGamma[geoS]];
(* the spinor curvature for the general static, spherically symmetric tetrad diag(h(r), 1/h(r), r, r sin th) with a
   symbolic function h, which contains Schwarzschild (h = f^{1/2}) and avoids the square roots that Simplify does not always
   remove *)
geoSh = geometry[DiagonalMatrix[{hS[r], 1/hS[r], r, r Sin[th]}], XS, Simplify];
riemSh = riemann[geoSh["chr"], XS, Simplify];
okCurvS = zeroQ[spinCurvDev[geoSh, riemSh]];
check["WG-1", "(Rust G-4 to G-9; Python YG-2, YG-3) Schwarzschild, f = 1 - 2M/r, coframe e^0 = f^{1/2} dt, e^1 = \
f^{-1/2} dr, e^2 = r dth, e^3 = r sin(th) dph: Ricci tensor zero; Kretschmann invariant 48 M^2/r^6; Omega_t = M/(2 r^2) \
gamma^0 gamma^1, Omega_r = 0, Omega_th = 1/2 f^{1/2} gamma^1 gamma^2, Omega_ph = 1/2 (f^{1/2} sin(th) gamma^1 gamma^3 + \
cos(th) gamma^2 gamma^3); d_mu gamma^nu + Gamma^nu_{mu la} gamma^la + [Omega_mu, gamma^nu] = 0; spinor curvature = 1/4 \
R_{ab mu nu} gamma^{ab} (for every static, spherically symmetric tetrad diag(h(r), 1/h(r), r, r sin th) with a symbolic \
function h)", okRicS && okKretS && okOmS && okNgS && okCurvS,
  "Ricci 0 " <> ToString[okRicS] <> ", Kretschmann " <> ToString[okKretS] <> ", Omega " <> ToString[okOmS] <>
   ", nabla gamma " <> ToString[okNgS] <> ", curvature " <> ToString[okCurvS]];

(* ::Section:: *)
(*WG-2: the freely falling observers (Painleve and Gullstrand)*)

(* ::Text:: *)
(*The static tetrad is singular at the horizon, because a static observer cannot exist there. The observers who fall freely from rest at infinity have a tetrad that is regular across the horizon: with their proper time $\tau_{\rm LG}$ as the time coordinate, $e^0=d\tau_{\rm LG}$, $e^1=dr+w_{\rm LG}\,d\tau_{\rm LG}$ with $w_{\rm LG}=(2\mathcal M_{\rm LG}/r)^{1/2}$, their speed (in the code `tT` and `vPG`). The check shows that this is the same space-time (the metric is Schwarzschild's after the change $\tau_{\rm LG}=t+\int w_{\rm LG}/f_{\rm LG}\,dr$), computes the spinor connection in closed form, and shows that the tetrad, the Christoffel symbols and the spinor connection are finite at $r=2\mathcal M_{\rm LG}$. Part IV integrates the radial equations of the brochon in this tetrad across the horizon.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WG-2: Painleve-Gullstrand *)
vPG = Sqrt[2 M/r];
XP = {tT, r, th, ph};
geoP = geometry[{{1, 0, 0, 0}, {vPG, 1, 0, 0}, {0, 0, r, 0}, {0, 0, 0, r Sin[th]}}, XP, simpA];
(* the metric pulled back to Schwarzschild coordinates by T = t + int v/f dr *)
jacP = {{1, vPG/fS, 0, 0}, {0, 1, 0, 0}, {0, 0, 1, 0}, {0, 0, 0, 1}};
okPull = zeroQ[Transpose[jacP] . geoP["g"] . jacP - DiagonalMatrix[{fS, -1/fS, -r^2, -r^2 Sin[th]^2}]];
riemP = riemann[geoP["chr"], XP, simpA];
okRicP = zeroQ[ricci[riemP]];
omegaP = {M/(2 r^2) gam[[1]] . gam[[2]], vPG/(4 r) gam[[1]] . gam[[2]],
   (-vPG gam[[1]] . gam[[3]] + gam[[2]] . gam[[3]])/2,
   (-vPG Sin[th] gam[[1]] . gam[[4]] + Sin[th] gam[[2]] . gam[[4]] + Cos[th] gam[[3]] . gam[[4]])/2};
okOmP = zeroQ[geoP["spc"] - omegaP];
okNgP = zeroQ[nablaGamma[geoP]];
okCurvP = zeroQ[spinCurvDev[geoP, riemP]];
okRegP = finiteQ[{geoP["F"], geoP["chr"], geoP["spc"], geoP["g"]} /. r -> 2 M] &&
   zeroQ[(geoP["g"] /. r -> 2 M)[[1, 1]]];
check["WG-2", "(Rust G-4 to G-9; Python YG-2, YG-3) Painleve-Gullstrand, v = (2M/r)^{1/2}, coframe e^0 = dT, e^1 = dr \
+ v dT, e^2 = r dth, e^3 = r sin(th) dph (the frame of the observers falling freely from rest at infinity): the metric \
is Schwarzschild's under T = t + int v/f dr; Ricci zero; Omega_T = M/(2 r^2) gamma^0 gamma^1, Omega_r = v/(4 r) gamma^0 \
gamma^1, Omega_th = 1/2 (-v gamma^0 gamma^2 + gamma^1 gamma^2), Omega_ph = 1/2 (-v sin(th) gamma^0 gamma^3 + sin(th) \
gamma^1 gamma^3 + cos(th) gamma^2 gamma^3); nabla gamma = 0; spinor curvature = 1/4 R gamma; the frame, the Christoffel \
symbols and Omega are finite at r = 2M, where g_TT = 0",
  okPull && okRicP && okOmP && okNgP && okCurvP && okRegP,
  "pull-back " <> ToString[okPull] <> ", Ricci 0 " <> ToString[okRicP] <> ", Omega " <> ToString[okOmP] <>
   ", nabla gamma " <> ToString[okNgP] <> ", curvature " <> ToString[okCurvP] <> ", regular at 2M " <> ToString[okRegP]];

(* ::Section:: *)
(*WG-3: the Vaidya space-times (a black hole whose mass changes)*)

(* ::Text:: *)
(*If radiation falls into a black hole, its mass grows; if it radiates, its mass shrinks. The Vaidya metric describes this with a mass that depends on the advanced time $v_{\rm LG}$ (for infalling radiation) or on the retarded time $u_{\rm LG}$ (for outgoing radiation); in the code they are `vv` and `uu`, and the mass functions `mIn` and `mOut`. The check computes Einstein's tensor and finds that it is $8\pi$ times the energy-momentum tensor of a pressureless stream of radiation moving at the speed of light (null dust), with the energy density $\rho_{\rm LG}=\mathcal M_{\rm LG}'(v_{\rm LG})/(4\pi r^2)$, positive when the mass grows, or $-\mathcal M_{\rm LG}'(u_{\rm LG})/(4\pi r^2)$, positive when the mass shrinks; the scalar curvature vanishes. Part V uses the ingoing form.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WG-3: Vaidya *)
einstein[g_, X_] := Module[{chr, riem, ric, rs},
  chr = christoffel[g, X, Simplify];
  riem = riemann[chr, X, Simplify];
  ric = Simplify[ricci[riem]];
  rs = Simplify[Sum[Inverse[g][[a, b]] ric[[a, b]], {a, 4}, {b, 4}]];
  {Simplify[ric - g rs/2], rs}];
gIn = {{1 - 2 mIn[vv]/r, -1, 0, 0}, {-1, 0, 0, 0}, {0, 0, -r^2, 0}, {0, 0, 0, -r^2 Sin[th]^2}};
{gtIn, rsIn} = einstein[gIn, {vv, r, th, ph}];
tIn = 8 Pi (mIn'[vv]/(4 Pi r^2)) Outer[Times, {1, 0, 0, 0}, {1, 0, 0, 0}];
gOut = {{1 - 2 mOut[uu]/r, 1, 0, 0}, {1, 0, 0, 0}, {0, 0, -r^2, 0}, {0, 0, 0, -r^2 Sin[th]^2}};
{gtOut, rsOut} = einstein[gOut, {uu, r, th, ph}];
tOut = 8 Pi (-mOut'[uu]/(4 Pi r^2)) Outer[Times, {1, 0, 0, 0}, {1, 0, 0, 0}];
okIn = zeroQ[gtIn - tIn] && zeroQ[rsIn];
okOut = zeroQ[gtOut - tOut] && zeroQ[rsOut];
check["WG-3", "Vaidya, ingoing: ds^2 = (1 - 2M(v)/r) dv^2 - 2 dv dr - r^2 dOmega^2: G_{mu nu} = 8 pi rho l_mu l_nu with \
l = dv and rho = M'(v)/(4 pi r^2) (null dust falling in), R = 0; outgoing: ds^2 = (1 - 2M(u)/r) du^2 + 2 du dr - r^2 \
dOmega^2: G_{mu nu} = 8 pi rho k_mu k_nu with k = du and rho = -M'(u)/(4 pi r^2) (null dust radiated, positive for M' < 0), \
R = 0", okIn && okOut, "ingoing " <> ToString[okIn] <> ", outgoing " <> ToString[okOut]];

(* ::Section:: *)
(*WG-4: conformal invariance and the square of the Dirac operator*)

(* ::Text:: *)
(*In a space-time whose metric is a multiple of the flat one, $g=\Omega_{\rm LG}^2\eta^{\rm LL}$ with $\Omega_{\rm LG}=e^{\phi(x)}$ and $\phi$ an arbitrary symbolic function, the check verifies two statements for a spinor of four symbolic functions. First, the massless Dirac operator $\gamma^\mu_{\rm LG}\nabla_\mu$ (with $\nabla_\mu=\partial_\mu+\Gamma^{\rm LL}_\mu$) is conformally invariant: $\gamma^\mu_{\rm LG}\nabla_\mu(\Omega_{\rm LG}^{-3/2}\psi)=\Omega_{\rm LG}^{-5/2}\gamma^a\partial_a\psi$, so the massless brochon does not feel such a gravitational field except through a rescaling. Second, the square of the Dirac operator is $(\gamma^\mu_{\rm LG}\nabla_\mu)^2\psi=g^{\mu\nu}_{\rm LG}\nabla_\mu\nabla_\nu\psi-\tfrac14R\psi$ (the formula of Lichnerowicz), with the coefficient $\tfrac14$ of the scalar curvature $R$, not Lord's $\tfrac12$ (the control shows that $\tfrac12$ fails, and that $R$ is not zero here, so the control means something). This coefficient is remark R1 of the manuscript.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WG-4: conformally flat *)
XC = {t, x, y, z};
geoC = geometry[Exp[phi @@ XC] id4, XC, Simplify];
psi = Table[ps[i] @@ XC, {i, 4}];
dslash[geo_, f_] := Sum[geo["gam"][[m]] . (D[f, geo["X"][[m]]] + geo["spc"][[m]] . f), {m, 4}];
flatSlash[f_] := Sum[gam[[a]] . D[f, XC[[a]]], {a, 4}];
okConf = zeroE[dslash[geoC, Exp[-3/2 phi @@ XC] psi] - Exp[-5/2 phi @@ XC] flatSlash[psi]];
riemC = riemann[geoC["chr"], XC, Simplify];
rsC = Simplify[Sum[geoC["gi"][[a, b]] ricci[riemC][[a, b]], {a, 4}, {b, 4}]];
nabC = Table[D[psi, XC[[n]]] + geoC["spc"][[n]] . psi, {n, 4}];
boxC = Sum[geoC["gi"][[m, n]] (D[nabC[[n]], XC[[m]]] + geoC["spc"][[m]] . nabC[[n]] -
      Sum[geoC["chr"][[l, m, n]] nabC[[l]], {l, 4}]), {m, 4}, {n, 4}];
okLich = zeroE[dslash[geoC, dslash[geoC, psi]] - boxC + rsC/4 psi];
witRules = Join[{phi -> Function[{t, x, y, z}, (t + 2 x - y z)/3 + t^2/5]},
   Table[ps[i] -> Function[{t, x, y, z}, Evaluate[{1 + t x, y - 2 z^2, t z + 3, x y t}[[i]]]], {i, 4}]];
witPt = {t -> 1/2, x -> 1/3, y -> 1/5, z -> 1/7};
okLichLord = witnessQ[dslash[geoC, dslash[geoC, psi]] - boxC + rsC/2 psi, witRules, witPt];
okRnz = witnessQ[rsC, witRules, witPt];
check["WG-4", "(Rust G-10, G-15; Python YG-4, YG-7) g = Omega^2 eta with Omega = exp(phi(x)), phi a symbolic function, \
coframe Omega delta, spinor of four symbolic functions: Dslash (Omega^{-3/2} psi) = Omega^{-5/2} dslash psi (the massless \
brochon is conformally invariant); Dslash Dslash psi = Box psi - R/4 psi (Lichnerowicz), not Box psi - R/2 psi (Lord's \
printed coefficient); R is not zero", okConf && okLich && okLichLord && okRnz,
  "conformal " <> ToString[okConf] <> ", -R/4 " <> ToString[okLich] <> ", -R/2 fails " <> ToString[okLichLord] <>
   ", R != 0 " <> ToString[okRnz]];

(* ::Section:: *)
(*WG-5: the energy-momentum tensor from the variation of the tetrad*)

(* ::Text:: *)
(*In general relativity the source of gravitation is the energy-momentum tensor, defined as the response of the action to a change of the metric. For a spinor the action depends on the tetrad, not only on the metric, and the response to the tetrad has a symmetric and an antisymmetric part. The check perturbs the flat tetrad by sixteen symbolic functions, $e_a{}^\mu=\delta+\varepsilon h_{a\mu}$, varies the Dirac action to first order in $\varepsilon$ with the package `VariationalMethods`, and shows: the symmetric part is exactly the tensor $T_{\mu\nu}$ of the manuscript (16.2), for any field; the antisymmetric part is not zero for an arbitrary field, but vanishes for a solution of the Dirac equation (a superposition of two plane waves of mass 3). So on solutions the tensor is symmetric, as Einstein's equations require.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WG-5: the tetrad variation *)
Needs["VariationalMethods`"];
XV = {t, x, y, z};
hH = Table[hh[a, m] @@ XV, {a, 4}, {m, 4}];
fV = id4 + eps hH;                                 (* e_a^mu = delta + eps h_{a mu} *)
eV = id4 - eps Transpose[hH];                       (* e^a_mu, its inverse to first order *)
lin[ex_] := Expand[(ex /. eps -> 0) + eps (D[ex, eps] /. eps -> 0)];   (* keep the first order in eps *)
gV = lin[Transpose[eV] . eta . eV];
giV = lin[Transpose[fV] . eta . fV];
chrV = lin[Table[1/2 Sum[giV[[rr, l]] (D[gV[[l, n]], XV[[m]]] + D[gV[[l, m]], XV[[n]]] - D[gV[[m, n]], XV[[l]]]),
     {l, 4}], {rr, 4}, {m, 4}, {n, 4}]];
omV = lin[Table[Sum[eV[[a, n]] (D[fV[[b, n]], XV[[m]]] + Sum[chrV[[n, m, l]] fV[[b, l]], {l, 4}]), {n, 4}],
    {m, 4}, {a, 4}, {b, 4}]];
gUV = Table[Sum[fV[[a, m]] gam[[a]], {a, 4}], {m, 4}];
spcV = lin[Table[1/4 Sum[eta[[a, a]] omV[[m, a, b]] gab[a, b], {a, 4}, {b, 4}], {m, 4}]];
detV = lin[Det[eV]];
pV = Table[pp[i] @@ XV, {i, 4}];                   (* the spinor *)
qV = Table[qq[i] @@ XV, {i, 4}];                   (* its Dirac adjoint, an independent field *)
lagV = detV (I/2 Sum[qV . gUV[[m]] . (D[pV, XV[[m]]] + spcV[[m]] . pV) -
        (D[qV, XV[[m]]] - qV . spcV[[m]]) . gUV[[m]] . pV, {m, 4}] - mm qV . pV);
lag1 = Expand[D[lagV, eps] /. eps -> 0];
varV = Table[VariationalD[lag1, hh[a, m] @@ XV, XV], {a, 4}, {m, 4}];
thetaV = Table[varV[[n, m]] eta[[n, n]], {m, 4}, {n, 4}];   (* Theta_{mu nu} = (dS/de_a^mu) eta_{a nu} *)
gl = Table[eta[[m, m]] gam[[m]], {m, 4}];
lag0 = I/2 Sum[qV . gam[[m]] . D[pV, XV[[m]]] - D[qV, XV[[m]]] . gam[[m]] . pV, {m, 4}] - mm qV . pV;
tForm = Table[I/4 (qV . gl[[m]] . D[pV, XV[[n]]] + qV . gl[[n]] . D[pV, XV[[m]]] - D[qV, XV[[m]]] . gl[[n]] . pV -
       D[qV, XV[[n]]] . gl[[m]] . pV) - eta[[m, n]] lag0, {m, 4}, {n, 4}];
okSym = zeroE[(thetaV + Transpose[thetaV])/2 - tForm];
(* on shell: a superposition of two plane waves of mass 3 with rational momenta and spinors *)
slashV[k_] := Sum[eta[[a, a]] k[[a]] gam[[a]], {a, 4}];
k1 = {5, 4, 0, 0}; k2 = {5, 0, 0, 4};
u1 = (slashV[k1] + 3 id4) . {1, 2, 0, 1}; u2 = (slashV[k2] + 3 id4) . {0, 1, 1, 3};
dotX[k_] := k[[1]] t - k[[2]] x - k[[3]] y - k[[4]] z;
solP = u1 Exp[-I dotX[k1]] + u2 Exp[-I dotX[k2]];
solQ = (Conjugate[u1] . gam[[1]]) Exp[I dotX[k1]] + (Conjugate[u2] . gam[[1]]) Exp[I dotX[k2]];
subs = Join[Table[pp[i] -> Function[{t, x, y, z}, Evaluate[solP[[i]]]], {i, 4}],
   Table[qq[i] -> Function[{t, x, y, z}, Evaluate[solQ[[i]]]], {i, 4}], {mm -> 3}];
okShellEq = zeroE[Sum[I gam[[m]] . D[solP, XV[[m]]], {m, 4}] - 3 solP];
okAnti = zeroE[((thetaV - Transpose[thetaV])/2) /. subs];
offRules = Join[Table[pp[i] -> Function[{t, x, y, z}, Evaluate[{1 + t x, y - 2 z^2, t z + 3, x y t}[[i]]]], {i, 4}],
   Table[qq[i] -> Function[{t, x, y, z}, Evaluate[{t - y, 2 + x z, y^2, t + z}[[i]]]], {i, 4}], {mm -> 3}];
okAntiOff = witnessQ[(thetaV - Transpose[thetaV])/2, offRules, {t -> 1/2, x -> 1/3, y -> 1/5, z -> 1/7}];
check["WG-5", "(Rust G-13; Python YG-6) the energy-momentum tensor of the brochon from the variation of the action e \
[i/2 (psi-bar gamma^mu nabla_mu psi - (nabla_mu psi)-bar gamma^mu psi) - m psi-bar psi] with respect to the tetrad, at \
first order about flat space-time (e_a^mu = delta + eps h_{a mu}, sixteen symbolic functions h; symbolic psi and psi-bar): \
the symmetric part of Theta_{mu nu} = (dS/de_a^mu) eta_{a nu} is exactly T_{mu nu} = i/4 (psi-bar gamma_mu d_nu psi + \
psi-bar gamma_nu d_mu psi - d_mu psi-bar gamma_nu psi - d_nu psi-bar gamma_mu psi) - eta_{mu nu} L, off shell; the \
antisymmetric part is not zero off shell and vanishes on a solution (two plane waves of mass 3)",
  okSym && okShellEq && okAnti && okAntiOff,
  "symmetric part = T " <> ToString[okSym] <> ", solution " <> ToString[okShellEq] <> ", antisymmetric part 0 on shell " <>
   ToString[okAnti] <> ", and not off shell " <> ToString[okAntiOff]];

(* ::Section:: *)
(*WG-6: the pairing term and the zeugon*)

(* ::Text:: *)
(*In the superfluid ground state of the brochon model the field equation contains a pairing term, which mixes the brochon with its complex conjugate: $\mathcal C_{\rm LG}\Psi=\mathsf P_{\rm LG}\Psi^{*}$ with the matrix $\mathsf P_{\rm LG}=(\gamma^0_{\rm LL}\otimes\mathsf 1_2)(\mathsf K^3_{\rm LL})^\dagger$. For the quasi-particles to move like Dirac particles in a gravitational field (Theorem L2), this map must commute with the Dirac matrices and with every Lorentz transformation, and with the charge of the zeugon. The check shows exactly that: $\mathsf P\,\overline{\gamma^a}=\gamma^a\mathsf P$, $\mathsf P\,\overline{\mathsf P}=-1$, $\mathsf P\,\overline{X}=X\mathsf P$ for every element $X=\tfrac14w^{\rm LG}_{ab}\gamma^{ab}$ of the Lorentz algebra, and $\mathsf P\,\overline{iT^3}=iT^3\mathsf P$ for the generator $T^3_{\rm LL}$ of the zeugon, but not for $T^1$, $T^2$ or the fermion number (the bars here are complex conjugates). This is the algebraic reason why the zeugon, the gauge boson of $T^3_{\rm LL}$, stays massless in the superfluid while the other gauge bosons do not, and why the quasi-particles carry its charge (Theorem L2).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WG-6: the pairing term *)
eL = {{0, 1}, {-1, 0}};
gch = I gam[[1]] . gam[[2]] . gam[[3]] . gam[[4]];
cLL = ArrayFlatten[{{eL, 0}, {0, -eL}}];
k3 = KroneckerProduct[cLL . gch, eL . sig[[4]]];
d8m[m_] := KroneckerProduct[m, IdentityMatrix[2]];
i8m[m_] := KroneckerProduct[id4, m];
pM = d8m[gam[[1]]] . ConjugateTranspose[k3];
okGam = And @@ Table[zeroQ[pM . Conjugate[d8m[gam[[a]]]] - d8m[gam[[a]]] . pM], {a, 4}];
okSq = zeroQ[pM . Conjugate[pM] + IdentityMatrix[8]];
wS = {{0, w01, w02, w03}, {-w01, 0, w12, w13}, {-w02, -w12, 0, w23}, {-w03, -w13, -w23, 0}};
omL = d8m[1/4 Sum[wS[[a, b]] gab[a, b], {a, 4}, {b, 4}]];
okLor = zeroQ[pM . ComplexExpand[Conjugate[omL]] - omL . pM];
genOK[tg_] := zeroQ[pM . Conjugate[I tg] - I tg . pM];
okT3 = genOK[i8m[sig[[4]]/2]];
notT1 = ! genOK[i8m[sig[[2]]/2]];
notT2 = ! genOK[i8m[sig[[3]]/2]];
notF = ! genOK[IdentityMatrix[8]];
check["WG-6", "(Rust G-17; Python YG-8) the pairing term of the polar superfluid, C psi = P conj(psi) with P = (gamma^0 \
(x) 1_2)(K^3_LL)^dag, K^3_LL = C_LL gamma^ch_LL (x) E_LL tau_3: P conj(gamma^a) = gamma^a P (so C anticommutes with i \
gamma^mu, real tetrad), P conj(P) = -1 (C^2 = -1), P conj(Omega) = Omega P for every Omega = 1/4 w_{ab} gamma^{ab} of the \
Lorentz algebra (w real symbols), and P conj(i t) = i t P for the generator t = T^3 of the zeugon but not for T^1, T^2 or \
the fermion number", okGam && okSq && okLor && okT3 && notT1 && notT2 && notF,
  "gamma " <> ToString[okGam] <> ", C^2 = -1 " <> ToString[okSq] <> ", Lorentz " <> ToString[okLor] <> ", T^3 " <>
   ToString[okT3] <> ", not T^1 " <> ToString[notT1] <> ", not T^2 " <> ToString[notT2] <> ", not F " <> ToString[notF]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*3. The brochon in the Schwarzschild field (Part IV)*)

(* ::Text:: *)
(*Part IV solves the Dirac equation of a particle of the mass $m_{\rm LL}$ (one isospin component of the brochon) around a Schwarzschild black hole of the mass $\mathcal M_{\rm LG}=1$ (the horizon at $r=2$; units $G_{\rm N}=c=\hbar=1$), with $f_{\rm LG}=1-2/r$ and $w_{\rm LG}=(2/r)^{1/2}$. Because the black hole is spherical, the equation separates. The angular dependence is that of the hydrogen atom: the angular functions $\alpha_{\rm LG}(\theta)$, $\beta_{\rm LG}(\theta)$ obey (18.4), with the total angular momentum $j_{\rm LG}$, its component $m_{j,{\rm LG}}$ and the separation constant $\lambda_{\rm LG}=\pm(j_{\rm LG}+\tfrac12)$. Two radial functions remain. In the static tetrad they are $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$ and obey (18.7); in the freely falling tetrad they are $\mathcal P_{\rm LG}$, $\mathcal Q_{\rm LG}$ and obey (18.5), which is regular at the horizon:*)

(* ::DisplayFormula:: *)
(*P'=\frac{(i\omega-W/(4r))P-(i\lambda/r+m)Q}{1-W},\qquad Q'=\frac{(i\lambda/r-m)P+(-i\omega+W/(4r))Q}{1+W}.*)

(* ::Text:: *)
(*In the code: `mB` is the mass $m_{\rm LL}$, `om` the frequency $\omega^{\rm S}_{\rm LG}$, `lam` the separation constant $\lambda_{\rm LG}$, `mj` the component $m_{j,{\rm LG}}$, `aA`, `bB` the angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$, `u`, `w` the radial functions $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$ of the static tetrad (the system called (S) in the code), `pP`, `qQ` the radial functions $\mathcal P_{\rm LG}$, $\mathcal Q_{\rm LG}$ of the freely falling tetrad (the system (PG)), `fS` the function $f_{\rm LG}$, and `wPG` the speed $w_{\rm LG}$ (in WS-3 `sW`). The matrices `sH` and `hH` are the constant matrices $\mathsf S^{\rm H}_{\rm LG}$ and $\mathsf H_{\rm LG}$ of (18.3), which bring the separated spinor into its simplest form.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Schw` (the code is that of the script wolfram/source/lg_schw.wls) *)
Begin["LGNB`Schw`"];

(* ::Text:: *)
(*The helper functions: `zeroQ`, `num` (a number written in the CSV files, such as `1.5e-3`, read exactly), and `readTable` (the header and the rows of a CSV file; in the notebook it reads the text of the file from a closed cell of this chapter).*)

(* ::Input:: *)
(* the helper functions of the script lg_schw.wls *)
notes = {};   (* the notes of the computations of this chapter *)
zeroQ[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[#] == 0] &];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
(* in the notebook: readTable reads the text of a CSV file, which a closed data cell of this chapter holds *)
readTable[csv_String] := Module[{tb = ImportString[csv, "CSV", "Numeric" -> False] /. _Missing -> "NaN"}, {First[tb], Rest[tb]}];

(* ::Input:: *)
(* ------------------------------------------------------------------ the matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
gam = Table[ArrayFlatten[{{0, sig[[a]]}, {sigbar[[a]], 0}}], {a, 4}];   (* gamma^a of the chiral basis, (1.2) *)
eta = DiagonalMatrix[{1, -1, -1, -1}];
gab[a_, b_] := (gam[[a]] . gam[[b]] - gam[[b]] . gam[[a]])/2;
hH = I (sig[[2]] + sig[[4]])/Sqrt[2];
sH = ArrayFlatten[{{hH, 0}, {0, hH}}];

(* ::Text:: *)
(*The function `geometry` of the previous chapter, written again (this chapter has its own context), and the Dirac operator `dirac[geo, psi, mass]` $=i\gamma^k_{\rm LG}(\partial_k+\Gamma^{\rm LL}_k)\psi-m\psi$; `angRules` are the angular equations (18.4), $\alpha_{\rm LG}'=(m_{j,{\rm LG}}/\sin\theta)\alpha_{\rm LG}-\lambda_{\rm LG}\beta_{\rm LG}$ and $\beta_{\rm LG}'=-(m_{j,{\rm LG}}/\sin\theta)\beta_{\rm LG}+\lambda_{\rm LG}\alpha_{\rm LG}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ the geometry of a coframe *)
(* E[[a, k]] = e^a_k; F[[a, k]] = e_a^k; the Christoffel symbols; the spin connection of the tetrad postulate
   om[[k, a, b]] = omega_k^a_b; gamma^k = e_a^k gamma^a; the spinor connection Omega_k = 1/4 omega_{k ab} gamma^{ab}
   (the function geometry of lg_curved.wls) *)
geometry[E_, X_, simp_] := Module[{F, g, gi, chr, om1, gU, spc},
  F = simp[Transpose[Inverse[E]]];
  g = simp[Transpose[E] . eta . E];
  gi = simp[Transpose[F] . eta . F];
  chr = simp[Table[1/2 Sum[gi[[rr, l]] (D[g[[l, n]], X[[k]]] + D[g[[l, k]], X[[n]]] - D[g[[k, n]], X[[l]]]),
     {l, 4}], {rr, 4}, {k, 4}, {n, 4}]];
  om1 = simp[Table[Sum[E[[a, n]] (D[F[[b, n]], X[[k]]] + Sum[chr[[n, k, l]] F[[b, l]], {l, 4}]), {n, 4}],
     {k, 4}, {a, 4}, {b, 4}]];
  gU = Table[Sum[F[[a, k]] gam[[a]], {a, 4}], {k, 4}];
  spc = simp[Table[1/4 Sum[eta[[a, a]] om1[[k, a, b]] gab[a, b], {a, 4}, {b, 4}], {k, 4}]];
  <|"gam" -> gU, "spc" -> spc, "X" -> X|>];
(* the Dirac operator i gamma^k (d_k + Omega_k) psi - m psi *)
dirac[geo_, psi_, mass_] := Sum[I geo["gam"][[k]] . (D[psi, geo["X"][[k]]] + geo["spc"][[k]] . psi), {k, 4}] - mass psi;
angRules = {Derivative[1][aA][th] -> mj/Sin[th] aA[th] - lam bB[th], Derivative[1][bB][th] -> -mj/Sin[th] bB[th] + lam aA[th]};

(* ::Section:: *)
(*WS-1 and WS-2: the separation in the two tetrads*)

(* ::Text:: *)
(*These checks prove that the separation is exact. The spinor is written with the angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$ and two radial functions, it is put into the Dirac equation with the spin connection computed from the tetrad, and the angular and radial equations are used to eliminate the derivatives; the result must vanish identically. WS-1 does this in the static tetrad (outside the horizon), WS-2 in the freely falling tetrad, where the identity holds for every $r>0$: on both sides of the horizon, which is what Part IV needs to follow the field into the black hole.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-1: the static tetrad *)
fS = 1 - 2/r;
geoS = geometry[DiagonalMatrix[{Sqrt[fS], 1/Sqrt[fS], r, r Sin[th]}], {t, r, th, ph},
   Simplify[#, r > 2 && 0 < th < Pi] &];
aR = (u[r] - I w[r])/2; bR = (u[r] + I w[r])/2;
radS = {Derivative[1][u][r] -> (-mB Sqrt[fS] u[r] + (om + lam Sqrt[fS]/r) w[r])/fS,
   Derivative[1][w][r] -> (-(om - lam Sqrt[fS]/r) u[r] + mB Sqrt[fS] w[r])/fS};
prefS = fS^(-1/4) r^-1 Sin[th]^(-1/2) Exp[-I om t + I mj ph];
psiS = prefS sH . {-I bR aA[th], I aR bB[th], aR aA[th], bR bB[th]};
resS = Simplify[(dirac[geoS, psiS, mB] /. angRules /. radS)/prefS, r > 2 && 0 < th < Pi];
check["WS-1", "(Rust LS-1; Python YS-1) the separation in the static tetrad: with the spin connection computed from the \
tetrad, the spinor f^(-1/4) r^(-1) sin(th)^(-1/2) e^(-i omega t + i m_j ph) S_H (-i b_R A, i a_R B, a_R A, b_R B), a_R = \
(u - i w)/2, b_R = (u + i w)/2, solves i gamma^k (d_k + Omega_k) psi = m psi exactly when A, B solve (A) and u, w solve \
(S) (omega, m, lambda, m_j symbolic; r > 2)", zeroQ[resS], "residual " <> ToString[InputForm[resS]]];

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-2: the freely falling tetrad *)
wPG = Sqrt[2/r];
geoP = geometry[{{1, 0, 0, 0}, {wPG, 1, 0, 0}, {0, 0, r, 0}, {0, 0, 0, r Sin[th]}}, {tau, r, th, ph},
   Simplify[#, r > 0 && 0 < th < Pi] &];
radP = {Derivative[1][pP][r] -> ((I om - wPG/(4 r)) pP[r] - (I lam/r + mB) qQ[r])/(1 - wPG),
   Derivative[1][qQ][r] -> ((I lam/r - mB) pP[r] + (-I om + wPG/(4 r)) qQ[r])/(1 + wPG)};
prefP = r^-1 Sin[th]^(-1/2) Exp[-I om tau + I mj ph];
psiP = prefP sH . {-I qQ[r] aA[th], I pP[r] bB[th], pP[r] aA[th], qQ[r] bB[th]};
resP = Simplify[(dirac[geoP, psiP, mB] /. angRules /. radP)/prefP, r > 0 && 0 < th < Pi];
check["WS-2", "(Rust LS-2; Python YS-2) the separation in the freely falling tetrad: the spinor r^(-1) sin(th)^(-1/2) \
e^(-i omega tau + i m_j ph) S_H (-i Q A, i P B, P A, Q B) solves the Dirac equation exactly when A, B solve (A) and P, Q \
solve (PG), for every r > 0 (the identity holds on both sides of the horizon)", zeroQ[resP],
  "residual " <> ToString[InputForm[resP]]];

(* ::Section:: *)
(*WS-3: the two descriptions are the same field*)

(* ::Text:: *)
(*Outside the horizon both systems describe the brochon, so they must be related by the change of tetrad and of time coordinate. The check shows that if $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$ solve (18.7), then $\mathcal P_{\rm LG}=(1-w_{\rm LG})^{-1/2}a\,e^{i\omega^{\rm S}_{\rm LG}(\tau_{\rm LG}-t)}$ and $\mathcal Q_{\rm LG}=(1+w_{\rm LG})^{-1/2}b\,e^{i\omega^{\rm S}_{\rm LG}(\tau_{\rm LG}-t)}$, with $a=(\mathcal U_{\rm LG}-i\mathcal V_{\rm LG})/2$, $b=(\mathcal U_{\rm LG}+i\mathcal V_{\rm LG})/2$ and $d(\tau_{\rm LG}-t)/dr=w_{\rm LG}/f_{\rm LG}$, solve (18.5). The square roots are handled by writing everything in terms of the two positive numbers $(1-w_{\rm LG})^{1/2}$ and $(1+w_{\rm LG})^{1/2}$ (in the code `aa` and `bb`), whose squares add up to 2, and reducing the numerators modulo that relation.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-3: the relation of (S) and (PG) *)
(* Outside the horizon (0 < W < 1) every square root is a rational function of the two positive numbers
   a = (1 - W)^(1/2) and b = (1 + W)^(1/2), which satisfy a^2 + b^2 = 2: W = 1 - a^2, r = 2/W^2, f^(1/2) = a b.  The
   functions u, w are written U, V, and the common factor e^{i omega T} is divided out.  By the chain rule, with
   dW/dr = -W/(2r), da/dW = -1/(2a), db/dW = 1/(2b): d(1/a)/dr = (dW/dr)/(2 a^3), d(1/b)/dr = -(dW/dr)/(2 b^3).  The
   numerators of the two residuals are reduced modulo b^2 - (2 - a^2); the remainders must vanish identically. *)
sW = 1 - aa^2; rW = 2/sW^2; fW = aa^2 bb^2; sfW = aa bb; dWdr = -sW/(2 rW);
uDot = (-mB sfW uU + (om + lam sfW/rW) vV)/fW;                                  (* du/dr from (S) *)
vDot = (-(om - lam sfW/rW) uU + mB sfW vV)/fW;                                  (* dw/dr from (S) *)
tDot = sW/fW;                                                                   (* d(tau - t)/dr = W/f *)
aRv = (uU - I vV)/2; bRv = (uU + I vV)/2;
pV = aRv/aa; qV = bRv/bb;
dpV = dWdr/(2 aa^3) aRv + (uDot - I vDot)/(2 aa) + I om tDot pV;
dqV = -dWdr/(2 bb^3) bRv + (uDot + I vDot)/(2 bb) + I om tDot qV;
rel1 = dpV - ((I om - sW/(4 rW)) pV - (I lam/rW + mB) qV)/aa^2;                (* 1 - W = a^2 *)
rel2 = dqV - ((I lam/rW - mB) pV + (-I om + sW/(4 rW)) qV)/bb^2;               (* 1 + W = b^2 *)
relRem = Table[Expand[PolynomialReduce[Expand[Numerator[Together[e]]], {bb^2 - 2 + aa^2}, {bb, aa}][[2]]],
   {e, {rel1, rel2}}];
check["WS-3", "(Rust LS-3; Python YS-3) the two separations describe the same field: if (u, w) solve (S), then P = \
(1 - W)^(-1/2) a_R e^{i omega (tau - t)} and Q = (1 + W)^(-1/2) b_R e^{i omega (tau - t)}, with d(tau - t)/dr = W/f, solve \
(PG) exactly outside the horizon (the identity written as rational functions of a = (1 - W)^(1/2), b = (1 + W)^(1/2), the \
numerators reduced modulo a^2 + b^2 = 2)", relRem === {0, 0}, "remainders " <> ToString[InputForm[relRem]]];

(* ::Section:: *)
(*WS-4: how the field behaves at the horizon and at the centre*)

(* ::Text:: *)
(*Near a point where the coefficients of a linear system blow up like $1/(r-r_0)$, the solutions behave like powers $(r-r_0)^s$, and the exponents $s$ are the eigenvalues of the residue matrix (the coefficient of $1/(r-r_0)$). At the horizon the exponents of (18.5) are $0$, the solution that is smooth there and falls in, and $4i\omega^{\rm S}_{\rm LG}-\tfrac12$, a solution that oscillates infinitely often and would come out of the black hole; the physical boundary condition keeps the first. At $r=0$ both exponents are $\tfrac14$, so both solutions behave as $r^{1/4}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-4: the exponents *)
aPG = {{(I om - wPG/(4 r))/(1 - wPG), -(I lam/r + mB)/(1 - wPG)}, {(I lam/r - mB)/(1 + wPG), (-I om + wPG/(4 r))/(1 + wPG)}};
resH = Simplify[Limit[(r - 2) aPG, r -> 2]];
res0 = Simplify[Limit[r aPG, r -> 0, Direction -> "FromAbove"]];
evH = Simplify[Eigenvalues[resH]];
okH = Length[evH] == 2 && AnyTrue[evH, TrueQ[Simplify[#] == 0] &] &&
   AnyTrue[evH, TrueQ[Simplify[# - (4 I om - 1/2)] == 0] &];
ok0 = zeroQ[res0 - IdentityMatrix[2]/4];
check["WS-4", "(Rust LS-4, LS-4b, LS-6; Python YS-4) the exponents: the residue matrix lim (r - 2) A(r) of (PG) at the \
horizon has the eigenvalues 0 (the solution analytic at r = 2, which falls in) and 4 i omega - 1/2 (the solution that comes \
out); lim r A(r) at r = 0 is 1/4 times the unit matrix (both solutions behave as r^(1/4))", okH && ok0,
  "residue at r = 2: " <> ToString[InputForm[resH]] <> ", eigenvalues " <> ToString[InputForm[evH]] <> "; at r = 0: " <>
   ToString[InputForm[res0]]];

(* ::Section:: *)
(*WS-5 and WS-9: the radial current*)

(* ::Text:: *)
(*The conserved charge of the Dirac field gives the radial current (19.2), $J_{\rm LG}=(1+w_{\rm LG})|\mathcal Q_{\rm LG}|^2-(1-w_{\rm LG})|\mathcal P_{\rm LG}|^2$, the flux into a sphere. For a real frequency it does not depend on $r$ (WS-5): what flows in at infinity flows into the horizon. For a complex frequency $\omega^{\rm S}_{\rm LG}=a+ib$ it changes as $dJ_{\rm LG}/dr=2b(|\mathcal P_{\rm LG}|^2+|\mathcal Q_{\rm LG}|^2)$ (WS-9). A quasi-bound state decays at infinity and has $J_{\rm LG}=2$ at the horizon (with the normalization $\mathcal Q_{\rm LG}(2)=1$), so integrating gives $\mathrm{Im}\,\omega^{\rm S}_{\rm LG}=-1/\mathcal N^{\rm S}_{\rm LG}$, with $\mathcal N^{\rm S}_{\rm LG}$ the integral of $|\mathcal P_{\rm LG}|^2+|\mathcal Q_{\rm LG}|^2$ outside the horizon (20.3): the decay rate of a state is fixed by how much of it lies outside the black hole. In the code the complex conjugates are independent functions `pB`, `qB` that obey the conjugate system, and `aw`, `bw` are the real and the imaginary part of the frequency.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-5: the radial current *)
radPb = {Derivative[1][pB][r] -> ((-I om - wPG/(4 r)) pB[r] - (-I lam/r + mB) qB[r])/(1 - wPG),
   Derivative[1][qB][r] -> ((-I lam/r - mB) pB[r] + (I om + wPG/(4 r)) qB[r])/(1 + wPG)};
jJ = (1 + wPG) qQ[r] qB[r] - (1 - wPG) pP[r] pB[r];
dJ = Simplify[D[jJ, r] /. radP /. radPb, r > 0];
check["WS-5", "(Rust LS-7; Python YS-5) the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 of a real frequency is \
constant: dJ/dr = 0 exactly for every r > 0 (the conjugates of P, Q are independent functions that solve the conjugate \
equations; omega, m, lambda real)", zeroQ[dJ], "dJ/dr = " <> ToString[InputForm[dJ]]];

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-9: the current for a complex frequency *)
radPc = {Derivative[1][pP][r] -> ((I (aw + I bw) - wPG/(4 r)) pP[r] - (I lam/r + mB) qQ[r])/(1 - wPG),
   Derivative[1][qQ][r] -> ((I lam/r - mB) pP[r] + (-I (aw + I bw) + wPG/(4 r)) qQ[r])/(1 + wPG)};
radPbc = {Derivative[1][pB][r] -> ((-I (aw - I bw) - wPG/(4 r)) pB[r] - (-I lam/r + mB) qB[r])/(1 - wPG),
   Derivative[1][qB][r] -> ((-I lam/r - mB) pB[r] + (I (aw - I bw) + wPG/(4 r)) qB[r])/(1 + wPG)};
dJc = Simplify[(D[jJ, r] /. radPc /. radPbc) - 2 bw (pP[r] pB[r] + qQ[r] qB[r]), r > 0];
check["WS-9", "(Rust LS-10 to LS-13; Python YS-10) for a complex frequency omega = a + i b (a, b, m, lambda real) the radial \
current obeys dJ/dr = 2 b (|P|^2 + |Q|^2) exactly for every r > 0; for a quasi-bound state, which decays at infinity, with \
J = 2 at the horizon (q_0 = 1), this gives Im omega = -1/N, N the integral of |P|^2 + |Q|^2 from the horizon to infinity",
  zeroQ[dJc], "dJ/dr - 2 b (|P|^2 + |Q|^2) = " <> ToString[InputForm[dJc]]];

(* ::Section:: *)
(*WS-6: the angular functions of j = 1/2*)

(* ::Text:: *)
(*For $j_{\rm LG}=\tfrac12$ the angular functions have a closed form, for example $\alpha_{\rm LG}=\sin^{1/2}\tfrac\theta2\cos^{3/2}\tfrac\theta2$ and $\beta_{\rm LG}=\lambda_{\rm LG}\sin^{3/2}\tfrac\theta2\cos^{1/2}\tfrac\theta2$ for $m_{j,{\rm LG}}=\tfrac12$ (18.6). The check verifies the four cases $\lambda_{\rm LG}=\pm1$, $m_{j,{\rm LG}}=\pm\tfrac12$ exactly, and that $\alpha_{\rm LG}$ and $\beta_{\rm LG}$ divided by $(\sin\theta)^{1/2}$ are bounded at both poles (they become $\cos\tfrac\theta2/\sqrt2$ and $\sin\tfrac\theta2/\sqrt2$ up to sign). In the code `sS` and `cC` are the sine and the cosine of $\theta/2$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-6: the angular functions of j = 1/2 *)
(* On 0 < th < Pi both S = sin(th/2) and C = cos(th/2) are positive, so the half-integer powers may be expanded
   (PowerExpand); with dS/dth = C/2, dC/dth = -S/2 and sin(th) = 2 S C, each residual of (A), multiplied by S^(1/2)
   C^(1/2), is a polynomial in S, C, and it is reduced modulo S^2 + C^2 - 1. *)
dTh[e_] := D[e /. {sS -> sS[th], cC -> cC[th]}, th] /. {Derivative[1][sS][th] -> cC/2, Derivative[1][cC][th] -> -sS/2,
     sS[th] -> sS, cC[th] -> cC};
redSC[e_] := Expand[PolynomialReduce[Expand[PowerExpand[e Sqrt[sS] Sqrt[cC]]], {sS^2 + cC^2 - 1}, {sS, cC}][[2]]];
(* {lambda, m_j, A, B, the expected A/sin(th)^(1/2), B/sin(th)^(1/2)} *)
angCases = Join[
   Table[{l1, 1/2, Sqrt[sS] cC^(3/2), l1 sS^(3/2) Sqrt[cC], cC/Sqrt[2], l1 sS/Sqrt[2]}, {l1, {1, -1}}],
   Table[{l1, -1/2, sS^(3/2) Sqrt[cC], -l1 Sqrt[sS] cC^(3/2), sS/Sqrt[2], -l1 cC/Sqrt[2]}, {l1, {1, -1}}]];
angOk = Table[Module[{l1 = cs[[1]], m1 = cs[[2]], fa = cs[[3]], fb = cs[[4]], e1, e2, bnd},
    e1 = redSC[dTh[fa] - (m1/(2 sS cC) fa - l1 fb)];
    e2 = redSC[dTh[fb] - (-m1/(2 sS cC) fb + l1 fa)];
    bnd = Simplify[PowerExpand[{fa, fb}/Sqrt[2 sS cC] - cs[[5 ;; 6]]]];
    {e1, e2, bnd} === {0, 0, {0, 0}}], {cs, angCases}];
check["WS-6", "(Rust: the closed forms for j = 1/2) for lambda = +-1: A = sin(th/2)^(1/2) cos(th/2)^(3/2), B = lambda \
sin(th/2)^(3/2) cos(th/2)^(1/2) solve (A) with m_j = 1/2, and A = sin(th/2)^(3/2) cos(th/2)^(1/2), B = -lambda sin(th/2)^(1/2) \
cos(th/2)^(3/2) with m_j = -1/2; A/sin(th)^(1/2) and B/sin(th)^(1/2) are cos(th/2)/2^(1/2) and sin(th/2)/2^(1/2) up to sign, \
bounded at both poles", And @@ angOk, "cases (lambda, m_j) = (1, 1/2), (-1, 1/2), (1, -1/2), (-1, -1/2): " <>
   ToString[angOk]];

(* ::Section:: *)
(*The numerical part: a series at the horizon and NDSolve*)

(* ::Text:: *)
(*The next two checks find quasi-bound states and greybody factors again, independently of the Rust program. The solution that is smooth at the horizon is started from its Taylor series of order 20 at $r=2+\tfrac1{20}$ (the coefficients come from a linear system that the cell builds once), and integrated with `NDSolve` in arithmetic of 32 significant digits. This is an integration in the radius, not in time: `NDSolve` uses its automatic method, which controls its error to the requested 26 digits.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ the numerical part: Taylor start, NDSolve *)
nT = 20; x0T = 1/20; wpN = 32;
pc = Array[pT, nT + 1, 0]; qc = Array[qT, nT, 1];
pSer = Sum[pT[k] xx^k, {k, 0, nT}]; qSer = 1 + Sum[qT[k] xx^k, {k, 1, nT}];
eqT1 = 4 (2 + xx) (1 - Sqrt[2/(2 + xx)]) D[pSer, xx] - ((4 I om (2 + xx) - Sqrt[2/(2 + xx)]) pSer -
     (4 I lam + 4 mB (2 + xx)) qSer);
eqT2 = 4 (2 + xx) (1 + Sqrt[2/(2 + xx)]) D[qSer, xx] - ((4 I lam - 4 mB (2 + xx)) pSer +
     (Sqrt[2/(2 + xx)] - 4 I om (2 + xx)) qSer);
serT1 = Normal[Series[eqT1, {xx, 0, nT}]]; serT2 = Normal[Series[eqT2, {xx, 0, nT - 1}]];
eqsT = Join[Table[Coefficient[serT1, xx, k], {k, 0, nT}], Table[Coefficient[serT2, xx, k], {k, 0, nT - 1}]];
{bT, aT} = Normal[CoefficientArrays[eqsT, Join[pc, qc]]];
(* (P, Q) at r = 2 + x of the solution analytic at the horizon, Q(2) = 1, for numeric omega, m, lambda *)
taylorStart[w1_, m1_, l1_, x_] := Module[{z},
  z = LinearSolve[aT /. {om -> w1, mB -> m1, lam -> l1}, -(bT /. {om -> w1, mB -> m1, lam -> l1})];
  {Sum[z[[k + 1]] x^k, {k, 0, nT}], 1 + Sum[z[[nT + 1 + k]] x^k, {k, 1, nT}]}];
pgMat[w1_, m1_, l1_, r1_] := aPG /. {om -> w1, mB -> m1, lam -> l1, r -> r1};
(* integrate (PG) with NDSolve from r1 (values y1) to r2, arithmetic of wpN digits *)
pgSolve[w1_, m1_, l1_, r1_, y1_, r2_] := Module[{sol, ww = Sqrt[2/rv]},
  sol = NDSolve[{pp'[rv] == ((I w1 - ww/(4 rv)) pp[rv] - (I l1/rv + m1) qq[rv])/(1 - ww),
     qq'[rv] == ((I l1/rv - m1) pp[rv] + (-I w1 + ww/(4 rv)) qq[rv])/(1 + ww), pp[r1] == y1[[1]], qq[r1] == y1[[2]]},
    {pp, qq}, {rv, Min[r1, r2], Max[r1, r2]}, WorkingPrecision -> wpN, PrecisionGoal -> 26, AccuracyGoal -> 26,
    MaxSteps -> 10^7];
  {pp[r2], qq[r2]} /. First[sol]];

(* ::Text:: *)
(*The data of the Rust program that the next two checks compare with: the files `data/lg_schw_qbs.csv` (the quasi-bound states) and `data/lg_schw_grey.csv` (the greybody factors), in closed cells.*)

(* ::Input::Closed:: *)
(* the text of the file data/lg_schw_qbs.csv of the folder Revision_Lord_Gravity (SHA-256 3ca31fe2258bb8bead57f50fce82789b7337bc4f77882047c3e2c89e65fd3552), written into the notebook so that it reads no file *)
lgQbsCSV = "state,Mm,lambda,n,Re_omega_over_m,Im_omega_over_m,hydrogen_like,iterations,residual
1S1/2,5.00000000000000028e-2,1,1,9.98732807243026044e-1,-4.27255946984223579e-7,9.98750000000000027e-1,5,6.01537279968899811e-14
1S1/2,1.00000000000000006e-1,1,1,9.94688277460113457e-1,-2.78702814458410717e-5,9.94999999999999996e-1,5,6.13523218807867565e-14
1S1/2,1.49999999999999994e-1,1,1,9.86904166384580006e-1,-4.85735437457439139e-4,9.88750000000000018e-1,6,4.72686536583172741e-14
1S1/2,2.00000000000000011e-1,1,1,9.74172573469842207e-1,-3.75539694517528378e-3,9.79999999999999982e-1,8,6.72412495961190078e-14
1S1/2,2.50000000000000000e-1,1,1,9.58837701795108188e-1,-1.35865033351687814e-2,9.68750000000000000e-1,10,5.33978524663532428e-14
1S1/2,2.99999999999999989e-1,1,1,9.45463937508782215e-1,-2.96484805870455143e-2,9.54999999999999960e-1,16,2.75820337652039402e-15
1S1/2,3.49999999999999978e-1,1,1,9.35910774209327445e-1,-4.91734815334124581e-2,9.38749999999999973e-1,20,7.85160991913108846e-15
1S1/2,4.00000000000000022e-1,1,1,9.30060715983927033e-1,-7.00578818117484625e-2,9.19999999999999929e-1,27,2.61073072310311823e-14
2S1/2,5.00000000000000028e-2,1,2,9.99684607191841490e-1,-5.34303780089311724e-8,9.99687499999999951e-1,5,1.33857805455926093e-13
2S1/2,1.00000000000000006e-1,1,2,9.98698511271807754e-1,-3.49926565521407882e-6,9.98750000000000027e-1,5,1.30279318612196807e-14
2S1/2,1.49999999999999994e-1,1,2,9.96886278473413601e-1,-6.19108310968224609e-5,9.97187500000000004e-1,6,1.89206110209873947e-13
2S1/2,2.00000000000000011e-1,1,2,9.94010603619051025e-1,-4.97902156961842440e-4,9.94999999999999996e-1,7,2.86955765242529083e-14
2S1/2,2.50000000000000000e-1,1,2,9.90251277819439224e-1,-1.92028132729105973e-3,9.92187500000000000e-1,8,1.01770720749468462e-13
2S1/2,2.99999999999999989e-1,1,2,9.86197332389511794e-1,-4.53876495541347125e-3,9.88750000000000018e-1,11,3.92332624284914913e-15
2S1/2,3.49999999999999978e-1,1,2,9.82269834820340249e-1,-8.19627083327609772e-3,9.84687500000000049e-1,17,1.48821210475095772e-13
2S1/2,4.00000000000000022e-1,1,2,9.78689254918276630e-1,-1.26731103486975409e-2,9.79999999999999982e-1,19,1.76429331523425177e-13
2P1/2,5.00000000000000028e-2,-1,2,9.99686267781544680e-1,-2.39611093521503880e-11,9.99687499999999951e-1,5,8.69235878391624392e-14
2P1/2,1.00000000000000006e-1,-1,2,9.98729693601610236e-1,-5.80363820573760749e-9,9.98750000000000027e-1,4,2.69686696294333090e-13
2P1/2,1.49999999999999994e-1,-1,2,9.97078954499133752e-1,-2.24569905156421605e-7,9.97187500000000004e-1,4,1.09997600918017846e-14
2P1/2,2.00000000000000011e-1,-1,2,9.94625071709697828e-1,-4.13243737693075080e-6,9.94999999999999996e-1,6,3.42279500368372139e-13
2P1/2,2.50000000000000000e-1,-1,2,9.91139279648179938e-1,-4.95275373511254102e-5,9.92187500000000000e-1,6,2.97055492684295842e-13
2P1/2,2.99999999999999989e-1,-1,2,9.86183492456925381e-1,-4.08444298954018816e-4,9.88750000000000018e-1,8,6.95409652781667477e-16
2P1/2,3.49999999999999978e-1,-1,2,9.79487410271767889e-1,-2.04567417896259034e-3,9.84687500000000049e-1,8,1.78574296750884554e-14
2P1/2,4.00000000000000022e-1,-1,2,9.71930219074616852e-1,-6.03213054461888658e-3,9.79999999999999982e-1,8,2.08509379594141421e-15
2P3/2,5.00000000000000028e-2,2,2,9.99686862372169527e-1,-6.33466665874605575e-15,9.99687499999999951e-1,5,5.49936475551947585e-14
2P3/2,1.00000000000000006e-1,2,2,9.98739656785738283e-1,-6.03203056624687495e-12,9.98750000000000027e-1,4,2.51364888364715184e-14
2P3/2,1.49999999999999994e-1,2,2,9.97133876696285837e-1,-4.94230413800284171e-10,9.97187500000000004e-1,4,1.67487624895882950e-14
2P3/2,2.00000000000000011e-1,2,2,9.94824404808472229e-1,-1.50712788228980759e-8,9.94999999999999996e-1,4,5.02484388724360142e-14
2P3/2,2.50000000000000000e-1,2,2,9.91736725301363919e-1,-2.65886000185670419e-7,9.92187500000000000e-1,4,6.26061701241779093e-15
2P3/2,2.99999999999999989e-1,2,2,9.87746803741051616e-1,-3.29846326644492460e-6,9.88750000000000018e-1,6,8.67997425676098719e-14
2P3/2,3.49999999999999978e-1,2,2,9.82631899795033181e-1,-3.16484424739257641e-5,9.84687500000000049e-1,5,2.64185185525073528e-14
2P3/2,4.00000000000000022e-1,2,2,9.75973394613392609e-1,-2.39646921804252168e-4,9.79999999999999982e-1,7,6.12423157048144709e-14
3P3/2,5.00000000000000028e-2,2,3,9.99860849530191609e-1,-2.07552201896180016e-15,9.99861111111111089e-1,6,8.35638458772048776e-14
3P3/2,1.00000000000000006e-1,2,3,9.99440201700413366e-1,-2.13420409776828355e-12,9.99444444444444469e-1,4,1.25604975374269638e-14
3P3/2,1.49999999999999994e-1,2,3,9.98728007516503591e-1,-1.77126741124305354e-10,9.98750000000000027e-1,4,4.20719336539792872e-14
3P3/2,2.00000000000000011e-1,2,3,9.97705773573116295e-1,-5.50999033701762323e-9,9.97777777777777763e-1,5,1.80161073476045991e-13
3P3/2,2.50000000000000000e-1,2,3,9.96342947641910892e-1,-1.00157046135850105e-7,9.96527777777777790e-1,5,2.77290211933909976e-13
3P3/2,2.99999999999999989e-1,2,3,9.94588500479461346e-1,-1.29729126192299424e-6,9.94999999999999996e-1,6,1.49452554804473804e-13
3P3/2,3.49999999999999978e-1,2,3,9.92349577695163187e-1,-1.32140098302237703e-5,9.93194444444444491e-1,6,2.99221229580246337e-15
3P3/2,4.00000000000000022e-1,2,3,9.89448515945533180e-1,-1.07301569764640993e-4,9.91111111111111054e-1,6,1.77082365855567171e-14
3D3/2,5.00000000000000028e-2,-2,3,9.99860942859685631e-1,4.68155507067076383e-17,9.99861111111111089e-1,5,4.42107681237833350e-14
3D3/2,1.00000000000000006e-1,-2,3,9.99441731609666317e-1,4.98229383110348807e-16,9.99444444444444469e-1,4,1.11862953391404061e-13
3D3/2,1.49999999999999994e-1,-2,3,9.98736085411457641e-1,-1.26638025030360012e-13,9.98750000000000027e-1,4,1.31976292780908540e-15
3D3/2,2.00000000000000011e-1,-2,3,9.97732963027037956e-1,-6.84160049941559178e-12,9.97777777777777763e-1,4,2.52706830941351782e-14
3D3/2,2.50000000000000000e-1,-2,3,9.96415562339828309e-1,-1.85650463823658544e-10,9.96527777777777790e-1,4,1.44620063738165335e-13
3D3/2,2.99999999999999989e-1,-2,3,9.94759560096785012e-1,-3.29045925898214665e-9,9.94999999999999996e-1,4,2.91278991923398780e-13
3D3/2,3.49999999999999978e-1,-2,3,9.92729960063286399e-1,-4.34695572630651723e-8,9.93194444444444491e-1,5,2.92144575060030594e-13
3D3/2,4.00000000000000022e-1,-2,3,9.90274966236967091e-1,-4.62372966806976696e-7,9.91111111111111054e-1,5,5.17004797434472130e-13
";

(* ::Input::Closed:: *)
(* the text of the file data/lg_schw_grey.csv of the folder Revision_Lord_Gravity (SHA-256 cb76c0e2a97dd62f81bdde22ff81997aca6318eafb657b476b1e9cb25810d066), written into the notebook so that it reads no file *)
lgGreyCSV = "Mm,lambda,M_omega,Gamma,R2,Gamma_change_rmax
0.00000000000000000e0,1,2.50000000000000000e-1,8.73387004115425691e-1,1.26612995798451172e-1,-3.71314895719621063e-7
0.00000000000000000e0,1,4.00000000000000022e-1,9.98219019033555477e-1,1.78098071601728034e-3,-1.36367300784812073e-8
0.00000000000000000e0,1,5.99999999999999978e-1,9.99992857554069636e-1,7.14278323642155072e-6,9.95006743664816895e-10
0.00000000000000000e0,-1,2.50000000000000000e-1,8.73387004115425913e-1,1.26612995798451172e-1,-3.71314896052687970e-7
0.00000000000000000e0,-1,4.00000000000000022e-1,9.98219019033555477e-1,1.78098071601728576e-3,-1.36367298564366024e-8
0.00000000000000000e0,-1,5.99999999999999978e-1,9.99992857554069858e-1,7.14278323642160493e-6,9.95006965709421820e-10
0.00000000000000000e0,2,2.50000000000000000e-1,9.52848358991271148e-3,9.90471516607249192e-1,-2.97971961670667751e-8
0.00000000000000000e0,2,4.00000000000000022e-1,6.32114486252872920e-1,3.67885513438330536e-1,4.75772008456942785e-7
0.00000000000000000e0,2,5.99999999999999978e-1,9.98753018816733285e-1,1.24698117161822623e-3,-2.37170476591330726e-8
2.00000000000000011e-1,1,2.50000000000000000e-1,9.01650974776639358e-1,9.83490252191970976e-2,1.03281402197286098e-6
2.00000000000000011e-1,1,4.00000000000000022e-1,9.98713424158741758e-1,1.28657503854944693e-3,-4.09976743664941523e-8
2.00000000000000011e-1,1,5.99999999999999978e-1,9.99994615084725424e-1,5.38355611341281585e-6,3.96922716916492391e-10
2.00000000000000011e-1,-1,2.50000000000000000e-1,4.76300721883461953e-1,5.23699278092228271e-1,7.78158591230315722e-7
2.00000000000000011e-1,-1,4.00000000000000022e-1,9.95596462719434805e-1,4.40353832743393871e-3,-6.09653326710812848e-8
2.00000000000000011e-1,-1,5.99999999999999978e-1,9.99987679454224843e-1,1.23191799474963130e-5,7.98140997737561975e-10
2.00000000000000011e-1,2,2.50000000000000000e-1,4.49797620099991972e-3,9.95502023664895486e-1,5.26608285545890342e-8
2.00000000000000011e-1,2,4.00000000000000022e-1,6.32467833166901361e-1,3.67532166962495743e-1,8.48050384916199107e-7
2.00000000000000011e-1,2,5.99999999999999978e-1,9.98902160344904533e-1,1.09784170996789091e-3,-2.05959699295377163e-8
";

(* ::Section:: *)
(*WS-7: three quasi-bound states*)

(* ::Text:: *)
(*A quasi-bound state is a solution that is smooth at the horizon and decays at infinity; like the levels of a hydrogen atom it exists only for discrete frequencies, here complex, because the state slowly falls into the black hole. For the coupling $y_{\rm LG}=\mathcal M_{\rm LG}m_{\rm LL}=\tfrac3{10}$ (in the code `mmQ`) the check takes the states $1S_{1/2}$, $2P_{1/2}$ and $2P_{3/2}$: it integrates outward from the horizon and inward from a large radius (where it starts from the decaying solution), measures the mismatch of the two at a middle radius, and lets `FindRoot` move the frequency until the mismatch vanishes, starting $10^{-4}$ away from the Rust value. The largest difference from the Rust values, divided by $m_{\rm LL}$, must be below $10^{-11}$ (the Rust values have the accuracy of double precision).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-7: quasi-bound states *)
{qHead, qRows} = readTable[lgQbsCSV];
col[name_] := First[Flatten[Position[qHead, name]]];
qbsRust[state_, mmv_] := Module[{row = Select[qRows, #[[col["state"]]] === state && Abs[num[#[[col["Mm"]]]] - mmv] < 10^-9 &]},
  If[row === {}, Missing[], num[row[[1, col["Re_omega_over_m"]]]] + I num[row[[1, col["Im_omega_over_m"]]]]]];
(* the frequency is raised to 40 digits first, so that every number in NDSolve has more digits than its working
   precision *)
matching[w0_?NumericQ, m1_, l1_, rmatch_, rmax_] := Module[{w1 = SetPrecision[w0, 40], st, yh, es, vd, yi},
  st = taylorStart[w1, m1, l1, x0T];
  yh = pgSolve[w1, m1, l1, 2 + x0T, st, rmatch];
  es = Eigensystem[pgMat[w1, m1, l1, rmax]];
  vd = es[[2, First[Ordering[Re[es[[1]]], 1]]]];
  yi = pgSolve[w1, m1, l1, rmax, vd, rmatch];
  (yh[[1]] yi[[2]] - yh[[2]] yi[[1]])/(Norm[yh] Norm[yi])];
mmQ = 3/10;
qbsCases = {{"1S1/2", 1, 1}, {"2P1/2", -1, 2}, {"2P3/2", 2, 2}};
dev7 = 0; det7 = {};
Do[Module[{st = cs[[1]], l1 = cs[[2]], n1 = cs[[3]], wr, kap, rmatch, rmax, sol, wq},
   wr = SetPrecision[qbsRust[st, N[mmQ]] mmQ, 40];
   kap = Re[Sqrt[mmQ^2 - wr^2]];
   rmatch = n1^2/mmQ^2; rmax = SetPrecision[45/kap, 40];
   sol = FindRoot[matching[ww, mmQ, l1, rmatch, rmax], {ww, wr (1 + 10^-4), wr (1 - 10^-4)},
     WorkingPrecision -> 30, PrecisionGoal -> 20, AccuracyGoal -> 22, MaxIterations -> 40];
   wq = ww /. sol;
   dev7 = Max[dev7, Abs[wq - wr]/mmQ];
   AppendTo[det7, st <> ": omega/m = " <> fmt[Re[wq/mmQ], 14] <> " " <> fmt[Im[wq/mmQ], 10] <> " i (Rust " <>
     fmt[Re[wr/mmQ], 14] <> " " <> fmt[Im[wr/mmQ], 10] <> " i)"];
   Print["  WS-7 ", Last[det7]]], {cs, qbsCases}];
checkDev["WS-7", "(Rust LS-8; Python YS-6) the quasi-bound states 1S1/2, 2P1/2 and 2P3/2 at Mm = 3/10, found again with \
NDSolve in arithmetic of 32 digits from the Taylor start of order 20 at r = 2 + 1/20, the decaying local eigenvector at \
r = 45/kappa, matching at r = n^2/(Mm)^2, and FindRoot from starts 1e-4 away from data/lg_schw_qbs.csv: the largest \
|omega_Wolfram - omega_Rust|/m (the Rust values have the accuracy of double precision)", dev7, 10^-11];
AppendTo[notes, "WS-7: " <> StringRiffle[det7, "; "]];

(* ::Section:: *)
(*WS-8: three greybody factors*)

(* ::Text:: *)
(*A wave sent towards the black hole is partly absorbed and partly scattered back; the absorbed fraction is the greybody factor. The check integrates from the horizon to $r=4000$, splits the solution there into an incoming and an outgoing wave, and divides the current into the horizon by the incoming current. The three cases are compared with the Rust values, which carry the error $3\times10^{-8}$ of the integrator CVODE; the tolerance is $10^{-7}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WS-8: greybody factors at r = 4000 *)
{gHead, gRows} = readTable[lgGreyCSV];
gcol[name_] := First[Flatten[Position[gHead, name]]];
greyRust[m1_, l1_, w1_] := Module[{row = Select[gRows, Abs[num[#[[gcol["Mm"]]]] - m1] < 10^-9 &&
      num[#[[gcol["lambda"]]]] == l1 && Abs[num[#[[gcol["M_omega"]]]] - w1] < 10^-9 &]},
  If[row === {}, Missing[], num[row[[1, gcol["Gamma"]]]]]];
greyCases = {{0, 1, 1/4}, {1/5, -1, 1/4}, {0, 2, 2/5}};
dev8 = 0; det8 = {};
rFar = 4000;
Do[Module[{m1 = cs[[1]], l1 = cs[[2]], w1 = cs[[3]], st, y, wf, ar, br, uu, wv, sf, aa, bb, cc, kk, ab, gam1, gr},
   st = taylorStart[w1, m1, l1, x0T];
   y = pgSolve[w1, m1, l1, 2 + x0T, st, rFar];
   wf = Sqrt[2/rFar];
   ar = y[[1]] Sqrt[1 - wf]; br = y[[2]] Sqrt[1 + wf];
   uu = ar + br; wv = I (ar - br);
   sf = Sqrt[1 - 2/rFar];
   aa = m1 sf; bb = w1 + l1 sf/rFar; cc = w1 - l1 sf/rFar;
   kk = Sqrt[bb cc - aa^2];
   (* (u, w) = alpha (b, a + i k) + beta (b, a - i k); the incoming wave beta carries the current b k |beta|^2, the
      current into the horizon is 2 |Q(2)|^2 = 2 *)
   ab = LinearSolve[{{bb, bb}, {aa + I kk, aa - I kk}}, {uu, wv}];
   gam1 = 2/(bb kk Abs[ab[[2]]]^2);
   gr = greyRust[N[m1], l1, N[w1]];
   dev8 = Max[dev8, Abs[gam1 - gr]];
   AppendTo[det8, "Mm = " <> ToString[m1, InputForm] <> ", lambda = " <> ToString[l1] <> ", M omega = " <>
     ToString[w1, InputForm] <> ": Gamma = " <> fmt[gam1, 13] <> " (Rust " <> fmt[gr, 13] <> ")"];
   Print["  WS-8 ", Last[det8]]], {cs, greyCases}];
checkDev["WS-8", "(Rust LS-9; Python YS-7) the greybody factors (Mm, lambda, M omega) = (0, 1, 1/4), (1/5, -1, 1/4), \
(0, 2, 2/5) at r = 4000, found again with NDSolve in arithmetic of 32 digits from the Taylor start and the decomposition \
into the local outgoing and incoming waves: the largest difference of Gamma from data/lg_schw_grey.csv (the Rust values \
carry the global error 3e-8 of CVODE, check LS-9a)", dev8, 10^-7];
AppendTo[notes, "WS-8: " <> StringRiffle[det8, "; "]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*4. The brochon around a black hole whose mass changes (Part V)*)

(* ::Text:: *)
(*Part V puts the brochon into the ingoing Vaidya space-time, $ds^2=f_{\rm LG}\,dv^2-2\,dv\,dr-r^2d\Omega^2$ with $f_{\rm LG}=1-2\mathcal M_{\rm LG}(v_{\rm LG})/r$, the field of a black hole that grows or shrinks (Chapter 2, WG-3). It uses a coframe that is regular across the horizon, $e^0=\tfrac12(1+f_{\rm LG})\,dv-dr$, $e^1=\tfrac12(1-f_{\rm LG})\,dv+dr$, $e^2=r\,d\theta$, $e^3=r\sin\theta\,d\varphi$. The Dirac equation then reduces to two equations for two functions $\mathcal P^{\rm V}_{\rm LG}(v_{\rm LG},r)$ and $\mathcal Q^{\rm V}_{\rm LG}(v_{\rm LG},r)$: an evolution equation in the advanced time (24.2) and a constraint in the radius (24.3),*)

(* ::DisplayFormula:: *)
(*\partial_vP=\Big(\frac Mr-\frac12\Big)\partial_rP-\frac M{2r^2}P-\Big(\frac{i\lambda}{2r}+\frac m2\Big)Q,\qquad\partial_rQ=\Big(\frac{i\lambda}r-m\Big)P.*)

(* ::Text:: *)
(*In the code the advanced time is `v`, the mass function of the black hole `bhM[v]`, the mass of the brochon `mB`, and the radial functions `pP`, `qQ`.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Vaidya` (the code is that of the script wolfram/source/lg_vaidya.wls) *)
Begin["LGNB`Vaidya`"];

(* ::Input:: *)
(* the helper functions of the script lg_vaidya.wls *)
zeroQ[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[#] == 0] &];

(* ::Input:: *)
(* ------------------------------------------------------------------ the matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
gam = Table[ArrayFlatten[{{0, sig[[a]]}, {sigbar[[a]], 0}}], {a, 4}];   (* gamma^a of the chiral basis, (1.2) *)
eta = DiagonalMatrix[{1, -1, -1, -1}];
gab[a_, b_] := (gam[[a]] . gam[[b]] - gam[[b]] . gam[[a]])/2;
hH = I (sig[[2]] + sig[[4]])/Sqrt[2];
sH = ArrayFlatten[{{hH, 0}, {0, hH}}];

(* ::Input:: *)
(* ------------------------------------------------------------------ the geometry of a coframe *)
(* E[[a, k]] = e^a_k; F[[a, k]] = e_a^k; the Christoffel symbols; the spin connection of the tetrad postulate
   om[[k, a, b]] = omega_k^a_b; gamma^k = e_a^k gamma^a; the spinor connection Omega_k = 1/4 omega_{k ab} gamma^{ab}
   (the function geometry of lg_curved.wls and lg_schw.wls) *)
geometry[E_, X_, simp_] := Module[{F, g, gi, chr, om1, gU, spc},
  F = simp[Transpose[Inverse[E]]];
  g = simp[Transpose[E] . eta . E];
  gi = simp[Transpose[F] . eta . F];
  chr = simp[Table[1/2 Sum[gi[[rr, l]] (D[g[[l, n]], X[[k]]] + D[g[[l, k]], X[[n]]] - D[g[[k, n]], X[[l]]]),
     {l, 4}], {rr, 4}, {k, 4}, {n, 4}]];
  om1 = simp[Table[Sum[E[[a, n]] (D[F[[b, n]], X[[k]]] + Sum[chr[[n, k, l]] F[[b, l]], {l, 4}]), {n, 4}],
     {k, 4}, {a, 4}, {b, 4}]];
  gU = Table[Sum[F[[a, k]] gam[[a]], {a, 4}], {k, 4}];
  spc = simp[Table[1/4 Sum[eta[[a, a]] om1[[k, a, b]] gab[a, b], {a, 4}, {b, 4}], {k, 4}]];
  <|"gam" -> gU, "spc" -> spc, "X" -> X, "g" -> g|>];
(* the Dirac operator i gamma^k (d_k + Omega_k) psi - m psi *)
dirac[geo_, psi_, mass_] := Sum[I geo["gam"][[k]] . (D[psi, geo["X"][[k]]] + geo["spc"][[k]] . psi), {k, 4}] - mass psi;
angRules = {Derivative[1][aA][th] -> mj/Sin[th] aA[th] - lam bB[th], Derivative[1][bB][th] -> -mj/Sin[th] bB[th] + lam aA[th]};

(* ::Section:: *)
(*WV-1: the separation for an arbitrary mass function*)

(* ::Text:: *)
(*With the spin connection computed from the coframe for an arbitrary function $\mathcal M_{\rm LG}(v_{\rm LG})$, the check shows that the separated spinor (24.1) solves the Dirac equation exactly when (18.4), (24.2) and (24.3) hold, for every $r>0$. As a control it changes one coefficient of (24.2) and shows that the identity then fails, so the check is able to fail.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WV-1: the separation *)
fV = 1 - 2 bhM[v]/r;
coframe = {{(1 + fV)/2, -1, 0, 0}, {(1 - fV)/2, 1, 0, 0}, {0, 0, r, 0}, {0, 0, 0, r Sin[th]}};
geoV = geometry[coframe, {v, r, th, ph}, Simplify[#, r > 0 && 0 < th < Pi] &];
prefV = r^-1 Sin[th]^(-1/2) Exp[I mj ph];
psiV = prefV sH . {-I qQ[v, r] aA[th], I pP[v, r] bB[th], pP[v, r] aA[th], qQ[v, r] bB[th]};
(* (V1) with the coefficient c of the term in P: c = 1/2 is the system of the Rust program; c = 1 is the control *)
radV[c_] := {Derivative[1, 0][pP][v, r] -> (bhM[v]/r - 1/2) Derivative[0, 1][pP][v, r] - c bhM[v]/r^2 pP[v, r] -
     (I lam/(2 r) + mB/2) qQ[v, r],
   Derivative[0, 1][qQ][v, r] -> (I lam/r - mB) pP[v, r]};
dirV = dirac[geoV, psiV, mB];
resV = Simplify[(dirV /. angRules /. radV[1/2])/prefV, r > 0 && 0 < th < Pi];
resC = Simplify[(dirV /. angRules /. radV[1])/prefV, r > 0 && 0 < th < Pi];
(* the metric of the coframe must be the Vaidya metric *)
gV = {{fV, -1, 0, 0}, {-1, 0, 0, 0}, {0, 0, -r^2, 0}, {0, 0, 0, -r^2 Sin[th]^2}};
check["WV-1", "(Rust LV-1; Python YV-1) the separation in the ingoing Vaidya space-time: the coframe e^0 = ((1+f)/2) dv - \
dr, e^1 = ((1-f)/2) dv + dr, e^2 = r dth, e^3 = r sin(th) dph gives the metric f dv^2 - 2 dv dr - r^2 dOmega^2 with f = \
1 - 2 M(v)/r; with the spin connection computed from it for an arbitrary function M(v), the spinor r^(-1) sin(th)^(-1/2) \
e^(i m_j ph) S_H (-i Q A, i P B, P A, Q B) solves i gamma^k (d_k + Omega_k) psi = m psi exactly when A, B solve (A) and P, Q \
solve (V1), (V2), for every r > 0 and whatever d_v Q is; with the coefficient M/r^2 instead of M/(2 r^2) in (V1) the \
residual is not zero (the control)",
  zeroQ[geoV["g"] - gV] && zeroQ[resV] && ! zeroQ[resC],
  "metric difference " <> ToString[InputForm[Simplify[geoV["g"] - gV]]] <> "; residual " <> ToString[InputForm[resV]] <>
   "; control residual (abbreviated) " <> ToString[Short[InputForm[resC], 2]]];

(* ::Section:: *)
(*WV-2: the conservation of the charge*)

(* ::Text:: *)
(*The charge of the brochon on a slice of constant advanced time is $\mathcal N^{\rm V}_{\rm LG}(v_{\rm LG})$, the integral of $2|\mathcal P^{\rm V}_{\rm LG}|^2$ over $r$ (24.5). It changes only by the flux through the boundaries: $\partial_v(2|\mathcal P^{\rm V}_{\rm LG}|^2)+\partial_r(f_{\rm LG}|\mathcal P^{\rm V}_{\rm LG}|^2-|\mathcal Q^{\rm V}_{\rm LG}|^2)=0$. Part V uses this to follow how much of the field stays outside the black hole while its mass changes, and the Rust program checks it in its evolutions (checks LV-4 and LV-5).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WV-2: the charge *)
(* the conjugates pB, qB of P, Q are independent functions that obey the conjugate equations (M(v), m, lambda real) *)
radVb = {Derivative[1, 0][pB][v, r] -> (bhM[v]/r - 1/2) Derivative[0, 1][pB][v, r] - bhM[v]/(2 r^2) pB[v, r] -
     (-I lam/(2 r) + mB/2) qB[v, r],
   Derivative[0, 1][qB][v, r] -> (-I lam/r - mB) pB[v, r]};
cont = Simplify[(2 D[pP[v, r] pB[v, r], v] + D[fV pP[v, r] pB[v, r] - qQ[v, r] qB[v, r], r]) /. radV[1/2] /. radVb];
check["WV-2", "(Rust LV-4, LV-5; Python YV-2) the conservation of the charge: d_v (2 |P|^2) + d_r (f |P|^2 - |Q|^2) = 0 \
exactly for every solution of (V1), (V2), for an arbitrary function M(v) (the conjugates of P, Q are independent functions \
that solve the conjugate equations; M, m, lambda real)", zeroQ[cont], "residual " <> ToString[InputForm[cont]]];

(* ::Section:: *)
(*WV-3 to WV-4b: the link to Part IV*)

(* ::Text:: *)
(*For a constant mass the equations must reduce to those of Part IV. With $\mathcal P^{\rm V}_{\rm LG}=e^{-i\omega^{\rm S}_{\rm LG}v}p^{\rm V}_{\rm LG}(r)$ and $\mathcal Q^{\rm V}_{\rm LG}=e^{-i\omega^{\rm S}_{\rm LG}v}q^{\rm V}_{\rm LG}(r)$ they become static equations whose exponents at the horizon are again $0$ and $4i\omega^{\rm S}_{\rm LG}-\tfrac12$ (WV-3). Their solutions are the freely falling solutions of Part IV: $p^{\rm V}_{\rm LG}=(1+w_{\rm LG})^{-1/2}e^{i\omega^{\rm S}_{\rm LG}h}\mathcal P_{\rm LG}$ and $q^{\rm V}_{\rm LG}=(1+w_{\rm LG})^{1/2}e^{i\omega^{\rm S}_{\rm LG}h}\mathcal Q_{\rm LG}$ with $h'=1/(1+w_{\rm LG})$ (so that $h$ is the function $\Delta_{\rm LG}=v_{\rm LG}-\tau_{\rm LG}$ of (24.7)); the opposite exponents fail, a control (WV-4). The reason (WV-4b): the coframe of Part V is the freely falling coframe boosted in the radial direction with the rapidity $\vartheta^{\rm V}_{\rm LG}=-\ln(1+w_{\rm LG})$, and a boost multiplies the two radial functions by $e^{\pm\vartheta/2}$. WV-5 shows that the radial light rays are the lines of constant $v_{\rm LG}$ (falling in) and the lines along which (24.2) differentiates $\mathcal P^{\rm V}_{\rm LG}$ (going out); the Rust program uses this to cut the inside of the black hole off its grid.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WV-3: the exponents of the static equations *)
aEF = {{(2 I om - 1/r^2)/(1 - 2/r), -(I lam/r + mB)/(1 - 2/r)}, {I lam/r - mB, 0}};
resEF = Simplify[Limit[(r - 2) aEF, r -> 2]];
evEF = Simplify[Eigenvalues[resEF]];
okEF = Length[evEF] == 2 && AnyTrue[evEF, TrueQ[Simplify[#] == 0] &] &&
   AnyTrue[evEF, TrueQ[Simplify[# - (4 I om - 1/2)] == 0] &];
(* the static equations from (V1), (V2) with P = e^(-i omega v) p(r), Q = e^(-i omega v) q(r), M = 1: the residuals r1,
   r2 of (V1), (V2), multiplied by 2 r^2 e^(i omega v) and r e^(i omega v), must equal the residuals st1, st2 of the static
   equations *)
subStat = {pP -> Function[{vv, rr}, Exp[-I om vv] pp[rr]], qQ -> Function[{vv, rr}, Exp[-I om vv] qq[rr]]};
r1V = Derivative[1, 0][pP][v, r] - ((1/r - 1/2) Derivative[0, 1][pP][v, r] - 1/(2 r^2) pP[v, r] -
     (I lam/(2 r) + mB/2) qQ[v, r]);
r2V = Derivative[0, 1][qQ][v, r] - (I lam/r - mB) pP[v, r];
st1 = r (r - 2) pp'[r] - (2 I om r^2 pp[r] - pp[r] - (I lam r + mB r^2) qq[r]);
st2 = r qq'[r] - (I lam - mB r) pp[r];
statV = Simplify[({2 r^2 Exp[I om v] r1V, r Exp[I om v] r2V} /. subStat) - {st1, st2}];
check["WV-3", "(Rust LV-2; Python YV-3) for a constant mass (M = 1) the substitution P = e^(-i omega v) p(r), Q = e^(-i \
omega v) q(r) turns (V1), (V2) into r (r - 2) p' = 2 i omega r^2 p - p - (i lambda r + m r^2) q, r q' = (i lambda - m r) p; \
the residue matrix lim (r - 2) A(r) of this system at the horizon has the eigenvalues 0 (the solution analytic at r = 2, \
which falls in) and 4 i omega - 1/2, as in stage 5", zeroQ[statV] && okEF,
  "the static reduction " <> ToString[InputForm[statV]] <> "; residue " <> ToString[InputForm[resEF]] <> ", eigenvalues " <>
   ToString[InputForm[evEF]]];

(* ::Input:: *)
(* ------------------------------------------------------------------ WV-4: the relation to stage 5 *)
(* Outside the horizon every square root is a rational function of b = (1 + W)^(1/2): W = b^2 - 1, r = 2/W^2,
   dW/dr = -W^3/4, db/dr = (dW/dr)/(2 b).  P, Q are the values of a solution of the system of stage 5 (freely falling
   tetrad) and dP, dQ its derivatives; p = e^(i omega h) b^s P, q = e^(i omega h) b^(-s) Q with h' = 1/(1 + W) = 1/b^2, so
   that the factor e^(i omega h) contributes (i omega/b^2) p to p'.  For s = -1 both static equations must hold
   identically in b, P, Q; for s = +1 they must fail. *)
wB = bb^2 - 1; rB = 2/wB^2; dWdr = -wB^3/4; dbdr = dWdr/(2 bb);
dP5 = ((I om - wB/(4 rB)) pV - (I lam/rB + mB) qV)/(1 - wB);
dQ5 = ((I lam/rB - mB) pV + (-I om + wB/(4 rB)) qV)/(1 + wB);
relEF[s_] := Module[{p, q, dp, dq},
  p = bb^s pV; q = bb^(-s) qV;
  dp = I om/bb^2 p + s bb^(s - 1) dbdr pV + bb^s dP5;
  dq = I om/bb^2 q - s bb^(-s - 1) dbdr qV + bb^(-s) dQ5;
  Together[{rB (rB - 2) dp - (2 I om rB^2 p - p - (I lam rB + mB rB^2) q), rB dq - (I lam - mB rB) p}]];
rel4 = relEF[-1]; rel4c = relEF[1];
check["WV-4", "(Rust LV-2; Python YV-4) the static solutions of stage 6 and the stationary solutions of stage 5 are the \
same field: if (P, Q) solve the system of stage 5 in the freely falling tetrad (W = (2/r)^(1/2)), then p = (1 + W)^(-1/2) \
e^(i omega h) P, q = (1 + W)^(1/2) e^(i omega h) Q with h' = 1/(1 + W) solve the static equations of stage 6 identically \
(written as rational functions of b = (1 + W)^(1/2)); with the opposite exponents, (1 + W)^(1/2) and (1 + W)^(-1/2), they do \
not (the control).  So the coframe of stage 6 is the freely falling coframe boosted with the rapidity -ln(1 + W), v = tau + \
h(r), and the two problems have the same quasi-bound frequencies", zeroQ[rel4] && ! zeroQ[rel4c],
  "residuals " <> ToString[InputForm[Simplify[rel4]]] <> "; control (abbreviated) " <>
   ToString[Short[InputForm[Simplify[rel4c]], 2]]];
(* the boost itself: in the coordinates (tau, r) of stage 5, with dv = dtau + dr/(1 + W), the coframe of stage 6 is
   Lambda . (freely falling coframe) with Lambda the boost of rapidity chi = -Log[1 + W] in the plane (0, 1) *)
wR = Sqrt[2/r]; fR = 1 - wR^2;
cfPG = {{1, 0}, {wR, 1}};                                                (* e^0 = dtau, e^1 = dr + W dtau, on (dtau, dr) *)
cfEF = {{(1 + fR)/2, (1 + fR)/2/(1 + wR) - 1}, {(1 - fR)/2, (1 - fR)/2/(1 + wR) + 1}};    (* stage 6, dv = dtau + dr/(1+W) *)
chi = -Log[1 + wR];
boost = {{Cosh[chi], Sinh[chi]}, {Sinh[chi], Cosh[chi]}};
boostDiff = Simplify[TrigToExp[cfEF - boost . cfPG], r > 0];
check["WV-4b", "(the explanation of WV-4) with v = tau + h(r), h' = 1/(1 + W), the two radial covectors of the coframe of \
stage 6 are the boost of rapidity -ln(1 + W) of those of the freely falling coframe e^0 = dtau, e^1 = dr + W dtau, at every \
r > 0", zeroQ[boostDiff], "difference " <> ToString[InputForm[boostDiff]]];

(* ::Input:: *)
(* ------------------------------------------------------------------ WV-5: the light rays *)
(* radial vectors (dv, dr): the length g(X, X) = f dv^2 - 2 dv dr; the radial null directions are dv = 0 and dr = f dv/2,
   since f dv^2 - 2 dv dr = dv (f dv - 2 dr) *)
nullLen[x_] := Simplify[x . gV[[1 ;; 2, 1 ;; 2]] . x];
factorRays = Simplify[fV dvv^2 - 2 dvv drr - dvv (fV dvv - 2 drr)];
transport = Simplify[-(bhM[v]/r - 1/2) - fV/2];          (* -(coefficient of d_r P in (V1)) minus f/2 *)
okRays = zeroQ[{nullLen[{0, 1}], nullLen[{1, fV/2}], factorRays, transport}];
check["WV-5", "(Rust: the excision of the black hole) the radial light rays: f dv^2 - 2 dv dr = dv (f dv - 2 dr), so the \
radial null directions are dv = 0 (the ingoing rays, which are the slices v = const) and dr/dv = f/2 (the outgoing rays); \
(V1) is d_v P + (f/2) d_r P = -(M/(2 r^2)) P - (i lambda/(2 r) + m/2) Q, a transport of P along the outgoing rays, outward \
where f > 0 and inward inside the apparent horizon r = 2 M(v); hence an inner edge of the grid inside the apparent horizon \
needs no boundary condition", okRays,
  "lengths of (0, 1) and (1, f/2): " <> ToString[InputForm[{nullLen[{0, 1}], nullLen[{1, fV/2}]}]] <>
   "; factorization " <> ToString[InputForm[factorRays]] <> "; transport " <> ToString[InputForm[transport]]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*5. The pair bound by the zeugon (Part VI, Section 29)*)

(* ::Text:: *)
(*Two quasi-particles of the superfluid with opposite zeugon charges $\pm\tfrac12$ attract each other through the massless zeugon, as an electron and a positron attract each other through the photon. Their bound states form an atom, the pair, with the potential $-\alpha^{\rm Z}_{\rm LG}/r$ and the strength $\alpha^{\rm Z}_{\rm LG}=(g^{\rm v}_{\rm LL})^2/(16\pi)$. To leading order its levels are those of hydrogen with the reduced mass $\mu^{\rm pair}_{\rm LG}$; at the next order, $(\alpha^{\rm Z}_{\rm LG})^4$, the motion of the particles (relativistic corrections), their spins (fine and hyperfine structure) and, for a particle with its own antiparticle, a term of virtual annihilation shift the levels. These corrections are computed from the Breit-Fermi Hamiltonian, as for positronium. In the code the units are those of the relative motion: reduced mass 1 and strength 1, so each particle has the mass `m = 2`; `rr` is the radius.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Pair` (the code is that of the script wolfram/source/lg_pair.wls) *)
Begin["LGNB`Pair`"];

(* ::Input::Closed:: *)
(* the text of the file data/lg_pair_levels.csv of the folder Revision_Lord_Gravity (SHA-256 6ac83ac23af11ef1ada6caa1596a2e3db9243cd340c633afb5d74d099d6fcbcb), written into the notebook so that it reads no file *)
lgPairLevelsCSV = "state,n,l,s,j,E2_over_m_alpha2,E4_over_m_alpha4_without_annihilation,E4_without_annihilation_float,E4_over_m_alpha4_with_annihilation,E4_with_annihilation_float
1^1S0,1,0,0,0,-1/4,-21/64,-3.28125000000000000e-01,-21/64,-3.28125000000000000e-01
1^3S1,1,0,1,1,-1/4,1/192,5.20833333333333304e-03,49/192,2.55208333333333315e-01
2^1S0,2,0,0,0,-1/16,-53/1024,-5.17578125000000000e-02,-53/1024,-5.17578125000000000e-02
2^3S1,2,0,1,1,-1/16,-31/3072,-1.00911458333333339e-02,65/3072,2.11588541666666678e-02
2^1P1,2,1,0,1,-1/16,-31/3072,-1.00911458333333339e-02,-31/3072,-1.00911458333333339e-02
2^3P2,2,1,1,2,-1/16,-43/15360,-2.79947916666666670e-03,-43/15360,-2.79947916666666670e-03
2^3P1,2,1,1,1,-1/16,-47/3072,-1.52994791666666661e-02,-47/3072,-1.52994791666666661e-02
2^3P0,2,1,1,0,-1/16,-95/3072,-3.09244791666666678e-02,-95/3072,-3.09244791666666678e-02
3^1S0,3,0,0,0,-1/36,-85/5184,-1.63966049382716049e-02,-85/5184,-1.63966049382716049e-02
3^3S1,3,0,1,1,-1/36,-7/1728,-4.05092592592592570e-03,1/192,5.20833333333333304e-03
3^1P1,3,1,0,1,-1/36,-7/1728,-4.05092592592592570e-03,-7/1728,-4.05092592592592570e-03
3^3P2,3,1,1,2,-1/36,-49/25920,-1.89043209876543208e-03,-49/25920,-1.89043209876543208e-03
3^3P1,3,1,1,1,-1/36,-29/5184,-5.59413580246913549e-03,-29/5184,-5.59413580246913549e-03
3^3P0,3,1,1,0,-1/36,-53/5184,-1.02237654320987657e-02,-53/5184,-1.02237654320987657e-02
3^1D2,3,2,0,2,-1/36,-41/25920,-1.58179012345679017e-03,-41/25920,-1.58179012345679017e-03
3^3D3,3,2,1,3,-1/36,-127/181440,-6.99955908289241605e-04,-127/181440,-6.99955908289241605e-04
3^3D2,3,2,1,2,-1/36,-49/25920,-1.89043209876543208e-03,-49/25920,-1.89043209876543208e-03
3^3D1,3,2,1,1,-1/36,-1/320,-3.12500000000000017e-03,-1/320,-3.12500000000000017e-03
4^1S0,4,0,0,0,-1/64,-117/16384,-7.14111328125000000e-03,-117/16384,-7.14111328125000000e-03
4^3S1,4,0,1,1,-1/64,-95/49152,-1.93277994791666674e-03,97/49152,1.97347005208333348e-03
4^1P1,4,1,0,1,-1/64,-95/49152,-1.93277994791666674e-03,-95/49152,-1.93277994791666674e-03
4^3P2,4,1,1,2,-1/64,-251/245760,-1.02132161458333339e-03,-251/245760,-1.02132161458333339e-03
4^3P1,4,1,1,1,-1/64,-127/49152,-2.58382161458333348e-03,-127/49152,-2.58382161458333348e-03
4^3P0,4,1,1,0,-1/64,-223/49152,-4.53694661458333304e-03,-223/49152,-4.53694661458333304e-03
4^1D2,4,2,0,2,-1/64,-73/81920,-8.91113281249999978e-04,-73/81920,-8.91113281249999978e-04
4^3D3,4,2,1,3,-1/64,-893/1720320,-5.19089471726190475e-04,-893/1720320,-5.19089471726190475e-04
4^3D2,4,2,1,2,-1/64,-251/245760,-1.02132161458333339e-03,-251/245760,-1.02132161458333339e-03
4^3D1,4,2,1,1,-1/64,-379/245760,-1.54215494791666661e-03,-379/245760,-1.54215494791666661e-03
4^1F3,4,3,0,3,-1/64,-51/114688,-4.44684709821428564e-04,-51/114688,-4.44684709821428564e-04
4^3F4,4,3,1,4,-1/64,-251/1032192,-2.43171812996031749e-04,-251/1032192,-2.43171812996031749e-04
4^3F3,4,3,1,3,-1/64,-169/344064,-4.91187686011904752e-04,-169/344064,-4.91187686011904752e-04
4^3F2,4,3,1,2,-1/64,-1277/1720320,-7.42303757440476155e-04,-1277/1720320,-7.42303757440476155e-04
";

(* ::Section:: *)
(*WP-1: the levels of the pair to the order alpha^4*)

(* ::Text:: *)
(*For every state with principal number $n\le3$ the check computes the coefficient of $m(\alpha^{\rm Z}_{\rm LG})^4$ of the shift, without and with the annihilation term, from the radial functions of hydrogen (`LaguerreL`) by exact integration, and compares it with the exact rational numbers that the Python program wrote into the file `data/lg_pair_levels.csv` (in the closed cell above). The function `ints` computes the integrals of the radial function that the operators need, `level` adds the terms of the Hamiltonian, and `s12` is the expectation value of the tensor force between the two spins.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WP-1 *)
(* the normalized radial function of the hydrogen-like state n l with the Bohr radius 1 *)
rad[n_, l_] := Sqrt[(2/n)^3 (n - l - 1)!/(2 n (n + l)!)] Exp[-rr/n] (2 rr/n)^l LaguerreL[n - l - 1, 2 l + 1, 2 rr/n];
ints[n_, l_] := Module[{R = rad[n, l], p4, en = -1/(2 n^2)},
   <|"ir" -> Integrate[R^2 rr, {rr, 0, Infinity}], "ir2" -> Integrate[R^2, {rr, 0, Infinity}],
    "ir3" -> If[l > 0, Integrate[R^2/rr, {rr, 0, Infinity}], 0],
    "p0" -> If[l == 0, Limit[R, rr -> 0]^2/(4 Pi), 0],
    "oo" -> Integrate[2 rr D[R, rr]^2 + If[l > 0, l (l + 1) R^2/rr, 0], {rr, 0, Infinity}]|>];
s12[l_, j_] := Which[l == 0, 0, j == l + 1, -2 l/(2 l + 3), j == l, 2, True, -2 (l + 1)/(2 l - 1)];
level[n_, l_, s_, j_, ann_] := Module[{iv = ints[n, l], m = 2, en = -1/(2 n^2), p4, e, s1s2},
   p4 = 4 (en^2 + 2 en iv["ir"] + iv["ir2"]);
   s1s2 = (s (s + 1) - 3/2)/2;
   e = -p4/8 (2/m^3) + Pi/2 (2/m^2) iv["p0"] - 1/(2 m^2) iv["oo"] +
     If[l > 0, 3/(2 m^2) iv["ir3"] (j (j + 1) - l (l + 1) - s (s + 1))/2 + If[s == 1, 1/m^2 iv["ir3"] s12[l, j]/4, 0], 0] +
     8 Pi/(3 m^2) s1s2 iv["p0"] + If[ann, Pi/(2 m^2) (3 + 4 s1s2) iv["p0"], 0];
   Simplify[e/2]];
(* in the notebook: the text of data/lg_pair_levels.csv, from the closed data cell of this chapter *)
tab = Rest[ImportString[lgPairLevelsCSV, "CSV", "Numeric" -> False]];
rows3 = Select[tab, ToExpression[#[[2]]] <= 3 &];
cmp = Table[Module[{n, l, s, j, e0, e1, w0, w1},
    {n, l, s, j} = ToExpression /@ rw[[2 ;; 5]];
    w0 = ToExpression[rw[[7]]]; w1 = ToExpression[rw[[9]]];
    e0 = level[n, l, s, j, False]; e1 = level[n, l, s, j, True];
    {rw[[1]], Simplify[e0 - w0] === 0 && Simplify[e1 - w1] === 0, e0, e1}], {rw, rows3}];
check["WP-1", "(Python YP-2; Rust LP-1) the coefficients of m alpha^4 of all " <> ToString[Length[cmp]] <> " states with n <= 3, \
without and with the annihilation term, computed with the radial functions of the Wolfram Language and exact integration, \
equal the rational numbers of data/lg_pair_levels.csv", AllTrue[cmp, #[[2]] &],
  StringRiffle[(#[[1]] <> " " <> ToString[InputForm[#[[3]]]] <> " / " <> ToString[InputForm[#[[4]]]]) & /@ cmp, "; "]];

(* ::Section:: *)
(*WP-2: the barrier of the pair in a tidal field*)

(* ::Text:: *)
(*Near a black hole the tidal field adds $-k^{\rm tid}_{\rm LG}z^2$ to the potential along the radial direction, with $k^{\rm tid}_{\rm LG}=\mu^{\rm pair}_{\rm LG}\mathcal M_{\rm LG}/r^3$ (in the code `k`, and `rg` for $\mathcal M_{\rm LG}$, `al` for $\alpha^{\rm Z}_{\rm LG}$, `mu` for $\mu^{\rm pair}_{\rm LG}$). The sum $-\alpha^{\rm Z}_{\rm LG}/z-k^{\rm tid}_{\rm LG}z^2$ has a maximum, a barrier: the check finds its top at $z_*=(\alpha^{\rm Z}_{\rm LG}/(2k^{\rm tid}_{\rm LG}))^{1/3}$ with the height $-\tfrac32(\alpha^{\rm Z}_{\rm LG})^{2/3}(2k^{\rm tid}_{\rm LG})^{1/3}$, and shows that the barrier sinks to the level $-\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2/(2n^2)$ at the radius given by $r^3=54n^6\mathcal M_{\rm LG}/((\mu^{\rm pair}_{\rm LG})^2(\alpha^{\rm Z}_{\rm LG})^4)$. This is the classical estimate of the break-up of the pair; Part VI shows (Section 32, remark R13) that along a real fall the pair breaks up much later.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WP-2 *)
(* The top lies at z* = (alpha/(2k))^(1/3), where dV/dz = 0 and d^2V/dz^2 = -2 alpha/z^3 - 2k < 0; all quantities are
   positive, so the powers may be expanded (PowerExpand).  (A first version found z* and r with Solve, whose results are
   Root objects that Simplify did not reduce to the closed forms; request 343, recorded in the task record.) *)
pos = al > 0 && k > 0 && mu > 0 && rg > 0 && n > 0 && r > 0;
zs = (al/(2 k))^(1/3);
dV = Simplify[PowerExpand[D[-al/z - k z^2, z] /. z -> zs], pos];
d2V = Simplify[PowerExpand[D[-al/z - k z^2, {z, 2}] /. z -> zs], pos];
top = Simplify[PowerExpand[(-al/z - k z^2) /. z -> zs], pos];
okTop = dV === 0 && Simplify[d2V < 0, pos] && Simplify[PowerExpand[top + 3/2 al^(2/3) (2 k)^(1/3)], pos] === 0;
lev = PowerExpand[top /. k -> mu rg/r^3 /. r -> (54 n^6 rg/(mu^2 al^4))^(1/3)];
okR = Simplify[lev + mu al^2/(2 n^2), pos] === 0;
check["WP-2", "(Rust LP-2) the barrier of -alpha/z - k z^2 has its top at z* = (alpha/(2k))^(1/3) (dV/dz = 0, d^2V/dz^2 < 0), of \
height -(3/2) alpha^(2/3) (2k)^(1/3), and with k = mu r_g/r^3 it equals the level -mu alpha^2/(2 n^2) at r^3 = 54 n^6 r_g/(mu^2 \
alpha^4)", okTop && okR,
  "dV/dz " <> ToString[InputForm[dV]] <> ", d2V/dz2 " <> ToString[InputForm[d2V]] <> ", top " <> ToString[InputForm[top]] <>
   ", top - level at r_ion " <> ToString[InputForm[Simplify[lev + mu al^2/(2 n^2), pos]]]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*6. A black hole that carries the charge of the zeugon (Part VI, Section 30)*)

(* ::Text:: *)
(*A black hole can swallow quasi-particles of one zeugon charge more than of the other and so acquire a charge $Q^{\rm Z}_{\rm LG}$. Its field is that of Reissner and Nordstrom, $f^{\rm RN}_{\rm LG}=1-2\mathcal M_{\rm LG}/r+(\mathcal Q^{\rm Z}_{\rm LG})^2/r^2$, with the charge measured as a length $\mathcal Q^{\rm Z}_{\rm LG}=(G_{\rm N}/4\pi)^{1/2}Q^{\rm Z}_{\rm LG}$ in the normalization of the zeugon field of the manuscript (30.1). This chapter checks the geometry, the limit that selective accretion sets to the charge, and the size of the shadow of such a black hole.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Charge` (the code is that of the script wolfram/source/lg_charge.wls) *)
Begin["LGNB`Charge`"];

(* ::Section:: *)
(*WC-1: the geometry*)

(* ::Text:: *)
(*With the zeugon field $A_{{\rm SF},\mu}dx^\mu=(Q^{\rm Z}_{\rm LG}/(4\pi r))\,dt$ the metric with $f^{\rm RN}_{\rm LG}=1-2G_{\rm N}M_{\rm LG}/r+G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/(4\pi r^2)$ solves Einstein's equations with the energy-momentum tensor of the zeugon exactly; with $G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/r^2$ in place of $G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/(4\pi r^2)$ (the normalization of another convention) it does not, which shows that the factor $4\pi$ matters and that the check can fail. In the code `gN` is $G_{\rm N}$, `mm` the mass and `qq` the charge.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WC-1: the geometry *)
xs = {tt, rr, th, ph};
einsteinMinusSource[fm_] := Module[{g, gi, gam, riem, ric, rs, ein, A, F, Fup, F2, Fmix, T},
  g = DiagonalMatrix[{fm, -1/fm, -rr^2, -rr^2 Sin[th]^2}];
  gi = Simplify[Inverse[g]];
  gam = Table[Simplify[Sum[gi[[a, d]] (D[g[[d, b]], xs[[c]]] + D[g[[d, c]], xs[[b]]] - D[g[[b, c]], xs[[d]]]), {d, 4}]/2],
    {a, 4}, {b, 4}, {c, 4}];
  (* R^a_{s m n} = d_m Gamma^a_{n s} - d_n Gamma^a_{m s} + Gamma^a_{m l} Gamma^l_{n s} - Gamma^a_{n l} Gamma^l_{m s} *)
  riem[a_, s_, m_, n_] := D[gam[[a, n, s]], xs[[m]]] - D[gam[[a, m, s]], xs[[n]]] +
    Sum[gam[[a, m, l]] gam[[l, n, s]] - gam[[a, n, l]] gam[[l, m, s]], {l, 4}];
  ric = Table[Simplify[Sum[riem[a, s, a, n], {a, 4}]], {s, 4}, {n, 4}];
  rs = Simplify[Sum[gi[[s, n]] ric[[s, n]], {s, 4}, {n, 4}]];
  ein = ric - g rs/2;
  A = {qq/(4 Pi rr), 0, 0, 0};
  F = Table[D[A[[n]], xs[[m]]] - D[A[[m]], xs[[n]]], {m, 4}, {n, 4}];
  Fup = gi . F . gi;
  F2 = Sum[F[[m, n]] Fup[[m, n]], {m, 4}, {n, 4}];
  Fmix = F . gi;
  T = Table[-Sum[F[[m, l]] Fmix[[n, l]], {l, 4}] + g[[m, n]] F2/4, {m, 4}, {n, 4}];
  Simplify[ein - 8 Pi gN T]];
res1 = einsteinMinusSource[1 - 2 gN mm/rr + gN qq^2/(4 Pi rr^2)];
ctrl1 = einsteinMinusSource[1 - 2 gN mm/rr + gN qq^2/rr^2];
check["WC-1", "(Python YC-1) with the zeugon field A = (Q/(4 pi r)) dt the metric with f = 1 - 2 G M/r + G Q^2/(4 pi r^2) \
solves G_{mu nu} = 8 pi G T_{mu nu} (the curvature of Landau and Lifshitz, the tensor (16.5) of the zeugon) exactly; with \
G Q^2/r^2 it does not (the control)", res1 === ConstantArray[0, {4, 4}] && ctrl1 =!= ConstantArray[0, {4, 4}],
  "residual " <> ToString[InputForm[Flatten[res1]]] <> "; control tt " <> ToString[InputForm[Factor[ctrl1[[1, 1]]]]]];

(* ::Section:: *)
(*WC-2: the limit of the charge by accretion*)

(* ::Text:: *)
(*A charged black hole repels particles of the same sign. A quasi-particle that starts at rest far away, with the ratio of its charge to its mass written as the length $x^{\rm acc}_{\rm LG}$ (30.2), reaches the horizon only if its radial velocity squared, $(1-x^{\rm acc}_{\rm LG}/r)^2-f^{\rm RN}_{\rm LG}$, stays positive all the way down. In units $\mathcal M_{\rm LG}=1$ (in the code `x` for $x^{\rm acc}_{\rm LG}$, `q2` for $(\mathcal Q^{\rm Z}_{\rm LG})^2$ and `W` for the velocity squared) the check shows the identities that make this simple, $r^2(dr/d\tau^{\rm qp}_{\rm LG})^2=2(1-x^{\rm acc}_{\rm LG})r+(x^{\rm acc}_{\rm LG})^2-(\mathcal Q^{\rm Z}_{\rm LG})^2$ (with $\tau^{\rm qp}_{\rm LG}$ the proper time of the quasi-particle) and $(r_{+,{\rm LG}}-x^{\rm acc}_{\rm LG})^2$ at the outer horizon $r_{+,{\rm LG}}$, and lets `Reduce` decide for six charges that the velocity squared is positive at every $r\ge r_{+,{\rm LG}}$ exactly when $0<x^{\rm acc}_{\rm LG}\le1$. Particles whose charge-to-mass length exceeds the mass of the black hole can no longer fall in, which limits the charge that accretion can build up.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WC-2: the accretion limit *)
fq = 1 - 2/r + q2/r^2;    (* M = 1, q2 = Qg^2 *)
W = (1 - x/r)^2 - fq;
rp = 1 + Sqrt[1 - q2];
idA = Simplify[Expand[W r^2 - (2 (1 - x) r + x^2 - q2)]];
idB = Simplify[Expand[(W r^2 /. r -> rp) - (rp - x)^2]];
(* the decision, for six charges Qg^2 = 0, 1/4, 1/2, 9/16, 3/4, 99/100: Reduce finds the x > 0 for which W r^2 >= 0 at
   every r >= r_+, and the answer must be 0 < x <= 1 *)
decs = Table[Reduce[ForAll[r, r >= (rp /. q2 -> c), (W r^2 /. q2 -> c) >= 0] && x > 0, x, Reals], {c, {0, 1/4, 1/2, 9/16, 3/4, 99/100}}];
okDec = AllTrue[decs, Simplify[Equivalent[#, 0 < x <= 1]] === True &];
check["WC-2", "(Rust LC-1; Python YC-2) W r^2 = 2 (1 - x) r + x^2 - Qg^2 and W(r_+) r_+^2 = (r_+ - x)^2 (M = 1); and for \
Qg^2 = 0, 1/4, 1/2, 9/16, 3/4, 99/100 Reduce decides that W >= 0 for every r >= r_+ exactly when 0 < x <= M",
  idA === 0 && idB === 0 && okDec,
  "identities " <> ToString[InputForm[{idA, idB}]] <> "; Reduce: " <> ToString[InputForm[decs]]];

(* ::Section:: *)
(*WC-3: the shadow*)

(* ::Text:: *)
(*Light that passes close to a black hole can circle it on the photon sphere, where $d(r^2/f^{\rm RN}_{\rm LG})/dr=0$, which here is $r^2-3r+2(\mathcal Q^{\rm Z}_{\rm LG})^2=0$ (units $\mathcal M_{\rm LG}=1$). Light that comes closer falls in, so the black hole casts a shadow whose radius, as seen from far away, is $b^{\rm sh}_{\rm LG}=(r^2/f^{\rm RN}_{\rm LG})^{1/2}$ at the photon sphere $r^{\rm ph}_{\rm LG}$. The check derives $b^{\rm sh}_{\rm LG}=3\sqrt3\,(1-(\mathcal Q^{\rm Z}_{\rm LG})^2/6-7(\mathcal Q^{\rm Z}_{\rm LG})^4/216+\dots)$ for a small charge and $b^{\rm sh}_{\rm LG}=4$ for the largest charge, $\mathcal Q^{\rm Z}_{\rm LG}=\mathcal M_{\rm LG}$; the charge shrinks the shadow, which is how such a charge could be detected (Section 34).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WC-3: the shadow *)
phi = r^2/fq;
crit = Factor[Together[D[phi, r]]];
rph = (3 + Sqrt[9 - 8 q2])/2;
onSphere = Simplify[(r^2 - 3 r + 2 q2) /. r -> rph];
bc = Sqrt[phi /. r -> rph] /. q2 -> eps^2;
ser = Normal[Series[bc, {eps, 0, 4}]];
okSer = Simplify[ser - 3 Sqrt[3] (1 - eps^2/6 - 7 eps^4/216)] === 0;
ext = Simplify[bc /. eps -> 1];
check["WC-3", "(Rust LC-2, LC-3; Python YC-3) d(r^2/f)/dr = 0 is r^2 - 3 r + 2 Qg^2 = 0, solved by r_ph = (3 + (9 - 8 \
Qg^2)^(1/2))/2; b_c = 3 3^(1/2) (1 - Qg^2/6 - 7 Qg^4/216 + ...) and b_c = 4 for the extremal charge (M = 1)",
  onSphere === 0 && okSer && ext === 4,
  "d/dr (r^2/f) = " <> ToString[InputForm[crit]] <> "; series " <> ToString[InputForm[ser]] <> "; extremal " <>
   ToString[InputForm[ext]]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*7. The pair falls into a black hole (Part VI, Section 32)*)

(* ::Text:: *)
(*A pair that falls freely into a black hole is stretched along the radius and squeezed across it by the tidal field. In the atomic units of the pair (lengths in its Bohr radius $a^{\rm pair}_{\rm LG}$, energies in $\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2$) its relative motion is that of a hydrogen atom in the potential $-1/r-\kappa^{\rm tid}_{\rm LG}r^2P_2(\cos\theta)$, where $P_2(x)=\tfrac12(3x^2-1)$ is the Legendre polynomial of degree 2 and $\kappa^{\rm tid}_{\rm LG}$ the tidal strength (32.2). This chapter checks the pieces of that computation that can be done exactly. In the code `kap` (WT-2) and `kk` (WT-3, WT-4) are $\kappa^{\rm tid}_{\rm LG}$, `T` is the time $T^{\rm fall}_{\rm LG}$, `mm` the mass $\mathcal M_{\rm LG}$, and `F` the strength $F^{\rm u}_{\rm LG}$ of a uniform field.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Tidal` (the code is that of the script wolfram/source/lg_tidal.wls) *)
Begin["LGNB`Tidal`"];

(* ::Section:: *)
(*WT-1: how the tidal field couples the angular momenta*)

(* ::Text:: *)
(*The tidal potential $r^2P_2(\cos\theta)$ couples a state of the orbital angular momentum $\ell$ to those of $\ell$ and $\ell\pm2$. The strength of the coupling is the integral of $Y_{\ell0}P_2Y_{\ell'0}$ over the sphere, a product of two 3-j symbols. The check compares the formula with 3-j symbols (`ThreeJSymbol`) with the closed forms that the Rust program uses, $\ell(\ell+1)/((2\ell-1)(2\ell+3))$ for $\ell'=\ell$ and the one for $\ell'=\ell+2$, for $\ell=0,2,\dots,30$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WT-1: the coupling *)
closed[l_, lp_] := Which[l == lp, l (l + 1)/((2 l - 1) (2 l + 3)),
   lp == l + 2, 3/2 (l + 1) (l + 2)/((2 l + 3) Sqrt[(2 l + 1) (2 l + 5)]), True, 0];
(* a 3-j symbol whose three angular momenta violate the triangle condition |l - l'| <= 2 <= l + l' is 0; it is set to 0
   here without calling ThreeJSymbol, which returns 0 as well but prints the message ClebschGordan::tri (for l = l' = 0;
   the first version of this script printed it, request 349) *)
threej[l_, lp_] := If[Abs[l - lp] <= 2 <= l + lp, Sqrt[(2 l + 1) (2 lp + 1)] ThreeJSymbol[{l, 0}, {2, 0}, {lp, 0}]^2, 0];
bad1 = Select[Flatten[Table[{l, lp, FullSimplify[threej[l, lp] - closed[l, lp]]}, {l, 0, 30, 2}, {lp, {l, l + 2}}], 1],
   #[[3]] =!= 0 &];
check["WT-1", "(Rust LT-0; Python YT-1) <Y_l0|P_2|Y_l'0> = ((2l+1)(2l'+1))^(1/2) (l 2 l'; 0 0 0)^2 equals the closed forms \
for l = 0, 2, ..., 30 and l' = l, l + 2", Length[bad1] == 0, ToString[Length[bad1]] <> " of 32 differ"];

(* ::Section:: *)
(*WT-2: the tidal field along the fall*)

(* ::Text:: *)
(*The observers of the Painleve-Gullstrand coordinates fall freely from rest far away. The check verifies that they are geodesic and that their proper time is the coordinate time; that the tidal tensor (the geodesic deviation) in their frame is $\mathrm{diag}(2\mathcal M_{\rm LG}/r^3,-\mathcal M_{\rm LG}/r^3,-\mathcal M_{\rm LG}/r^3)$, stretching along the radius and squeezing across; and that along their path $r^3=\tfrac92\mathcal M_{\rm LG}(\tau^{\rm end}_{\rm LG}-\tau_{\rm LG})^2$. In the atomic units of the pair the tidal strength is then $\kappa^{\rm tid}_{\rm LG}=2/(9(T^{\rm fall}_{\rm LG})^2)$, where $T^{\rm fall}_{\rm LG}$ is the proper time that is left before the centre would be reached: the same for every black hole. The mass of the black hole decides only when the horizon is crossed, at $T^{\rm fall}_{\rm LG}=\tfrac43\tilde{\mathcal M}_{\rm LG}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WT-2: the tidal field along the fall *)
xs = {tt, rr, th, ph};
vv = Sqrt[2 mm/rr];
g = {{1 - vv^2, -vv, 0, 0}, {-vv, -1, 0, 0}, {0, 0, -rr^2, 0}, {0, 0, 0, -rr^2 Sin[th]^2}};
gi = Simplify[Inverse[g]];
gam = Table[Simplify[Sum[gi[[a, d]] (D[g[[d, b]], xs[[c]]] + D[g[[d, c]], xs[[b]]] - D[g[[b, c]], xs[[d]]]), {d, 4}]/2],
   {a, 4}, {b, 4}, {c, 4}];
(* R^a_{b c d} = d_c Gamma^a_{d b} - d_d Gamma^a_{c b} + Gamma^a_{c l} Gamma^l_{d b} - Gamma^a_{d l} Gamma^l_{c b} *)
riem[a_, b_, c_, d_] := D[gam[[a, d, b]], xs[[c]]] - D[gam[[a, c, b]], xs[[d]]] +
   Sum[gam[[a, c, l]] gam[[l, d, b]] - gam[[a, d, l]] gam[[l, c, b]], {l, 4}];
u = {1, -vv, 0, 0};
normU = Simplify[u . g . u];
geo = Simplify[Table[Sum[u[[b]] D[u[[a]], xs[[b]]], {b, 4}] + Sum[gam[[a, b, c]] u[[b]] u[[c]], {b, 4}, {c, 4}], {a, 4}],
   Assumptions -> {mm > 0, rr > 0}];
acc = Table[Simplify[-Sum[riem[a, b, c, d] u[[b]] u[[d]], {b, 4}, {d, 4}], Assumptions -> {mm > 0, rr > 0}], {a, 4}, {c, 4}];
aRR = Simplify[acc[[2, 2]] + vv acc[[1, 2]], Assumptions -> {mm > 0, rr > 0}];
aTT = Simplify[acc[[3, 3]]]; aPP = Simplify[acc[[4, 4]]];
okFrame = normU === 1 && geo === {0, 0, 0, 0} && Simplify[aRR - 2 mm/rr^3] === 0 && Simplify[aTT + mm/rr^3] === 0 &&
   Simplify[aPP + mm/rr^3] === 0;
rOf[s_] := (9/2)^(1/3) mm^(1/3) s^(2/3);   (* s = tau0 - tau > 0 *)
okR = Simplify[D[rOf[s], s]^2 - 2 mm/rOf[s], Assumptions -> {s > 0, mm > 0}] === 0;
kap = Simplify[(mu mm/rOf[s]^3)/(mu^3 al^4) /. s -> T/(mu al^2), Assumptions -> {T > 0, mu > 0, al > 0, mm > 0}];
okK = Simplify[kap - 2/(9 T^2)] === 0;
(* r = 2M at the remaining proper time s = 4M/3 (r^3 = (9M/2) s^2 is increasing in s), i.e. at T_h = (4/3) M mu alpha^2 *)
okH = Simplify[rOf[4 mm/3]^3 - (2 mm)^3, Assumptions -> mm > 0] === 0;
check["WT-2", "(Python YT-2) the Painleve-Gullstrand observers u = d_t - (2M/r)^(1/2) d_r are unit and geodesic; the \
geodesic deviation -R^a_{b c d} u^b u^d in their frame is diag(2M/r^3, -M/r^3, -M/r^3); r = ((9M/2)(tau0 - tau)^2)^(1/3) \
solves (dr/dtau)^2 = 2M/r; kappa = (mu M/r^3)/(mu^3 alpha^4) = 2/(9 T^2) with T = (tau0 - tau) mu alpha^2; r = 2M at \
T_h = (4/3) M mu alpha^2", okFrame && okR && okK && okH,
  "frame " <> ToString[okFrame] <> " (A_rr = " <> ToString[InputForm[aRR]] <> ", A_thth = " <> ToString[InputForm[aTT]] <>
   "), radius " <> ToString[okR] <> ", kappa " <> ToString[InputForm[kap]] <> ", horizon " <> ToString[okH]];

(* ::Section:: *)
(*WT-3: the energy in a weak static tidal field*)

(* ::Text:: *)
(*In a weak static field the energy of the ground state shifts by second-order perturbation theory, which the method of Dalgarno and Lewis turns into a differential equation for one function. The check verifies that $\kappa^{\rm tid}_{\rm LG}(r^3/3+r^2/2)$ solves it, and that the energy is $E^{\rm tid}_{\rm LG}=-\tfrac12-\tfrac{15}2(\kappa^{\rm tid}_{\rm LG})^2-\tfrac{153}2(\kappa^{\rm tid}_{\rm LG})^3+\dots$ (32.5); the Python program compares these coefficients with the eigenvalues of its matrices.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WT-3: Dalgarno-Lewis *)
fdl = kk (r^3/3 + r^2/2);
okDL = Simplify[-1/2 (D[fdl, {r, 2}] + 2 D[fdl, r]/r - 6 fdl/r^2 - 2 D[fdl, r]) - kk r^2] === 0;
p2[x_] := LegendreP[2, x];
ang2 = 2 Pi Integrate[p2[x]^2, {x, -1, 1}]; ang3 = 2 Pi Integrate[p2[x]^3, {x, -1, 1}];
rho0 = Exp[-2 r]/Pi;
e2 = Simplify[-kk ang2 Integrate[r^2 fdl rho0 r^2, {r, 0, Infinity}]/kk^2];
e3 = Simplify[-kk ang3 Integrate[r^2 fdl^2 rho0 r^2, {r, 0, Infinity}]/kk^3];
check["WT-3", "(Python YT-3) f = kappa (r^3/3 + r^2/2) solves the equation of Dalgarno and Lewis for psi_1 = f P_2 psi_0, \
and E = -1/2 - (15/2) kappa^2 - (153/2) kappa^3 + O(kappa^4) (E_2 = <psi_0|V|psi_1>, E_3 = <psi_1|V|psi_1>)",
  okDL && e2 === -15/2 && e3 === -153/2, "solves: " <> ToString[okDL] <> "; E_2/kappa^2 = " <> ToString[InputForm[e2]] <>
   "; E_3/kappa^3 = " <> ToString[InputForm[e3]]];

(* ::Section:: *)
(*WT-4: the rate of tunnelling in a weak field*)

(* ::Text:: *)
(*In a static tidal field the pair can tunnel through its barrier and fly apart, at the rate $\Gamma^{\rm tid}_{\rm LG}$. For a weak field the rate follows from the solution near the nucleus and the semiclassical solution under the barrier. The check first applies the method to a uniform field of the strength $F^{\rm u}_{\rm LG}$, where it must give Landau's rate $(4/F^{\rm u}_{\rm LG})\,e^{-2/(3F^{\rm u}_{\rm LG})}$ (it does), and then to the tidal field, where it gives $8(\kappa^{\rm tid}_{\rm LG})^{-1/2}\exp(-\pi/(2(2\kappa^{\rm tid}_{\rm LG})^{1/2}))/\sinh(\pi/\sqrt2)$, the formula that the manuscript compares with the computed rates.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WT-4: the weak-field rate *)
(* uniform field F: L = 1/(2F); the integral of (1 - z/L)^(1/2) is 2L/3; the Coulomb logarithm log(4L/z_m); the
   Euclidean time to the turning point 2L, Y = t, S2 = 1/t *)
LL = 1/(2 F);
j0u = Integrate[Sqrt[1 - s], {s, 0, 1}] LL;
antiU = Log[(1 - Sqrt[1 - s])/(1 + Sqrt[1 - s])];
okAU = Simplify[D[antiU, s] - 1/(s Sqrt[1 - s]), Assumptions -> 0 < s < 1] === 0;
limU = Limit[(antiU /. s -> 1) - (antiU /. s -> um) - Log[4/um], um -> 0, Direction -> "FromAbove"];
t2u = Integrate[1/Sqrt[1 - s], {s, 0, 1}] LL;
gamU = Simplify[Exp[-2 zm] zm^2 Exp[-2 (j0u - zm - Log[4 LL/zm])]/t2u^2/Pi Pi t2u, Assumptions -> {F > 0, zm > 0}];
okU = okAU && limU === 0 && Simplify[gamU - 4/F Exp[-2/(3 F)], Assumptions -> F > 0] === 0;
(* tidal field: l = (2 kappa)^(-1/2); integral pi l/4; logarithm log(2 l/z_m); time pi l/2; Y = sinh(kappa^(1/2) t)/kappa^(1/2) *)
ell = 1/Sqrt[2 kk];
j0t = Integrate[Sqrt[1 - s^2], {s, 0, 1}] ell;
antiT = -Log[(1 + Sqrt[1 - s^2])/s];
okAT = Simplify[D[antiT, s] - 1/(s Sqrt[1 - s^2]), Assumptions -> 0 < s < 1] === 0;
limT = Limit[(antiT /. s -> 1) - (antiT /. s -> um) - Log[2/um], um -> 0, Direction -> "FromAbove"];
t2t = Integrate[1/Sqrt[1 - s^2], {s, 0, 1}] ell;
yy[t_] := Sinh[Sqrt[kk] t]/Sqrt[kk];
okY = Simplify[yy''[t] - kk yy[t]] === 0 && Limit[yy[t]/t, t -> 0] === 1;
gamT = 2 Exp[-2 zm] zm^2 Exp[-2 (j0t - zm - Log[2 ell/zm])]/yy[t2t]^2/Pi Pi/(yy'[t2t]/yy[t2t]);
targetT = 8/Sqrt[kk] Exp[-Pi/(2 Sqrt[2 kk])]/Sinh[Pi/Sqrt[2]];
okT = okAT && limT === 0 && okY && Simplify[FullSimplify[gamT/targetT, Assumptions -> {kk > 0, zm > 0}] - 1] === 0;
check["WT-4", "(Python YT-5a, YT-5b) the method of the weak-field rate gives Landau's (4/F) exp(-2/(3F)) for a uniform field \
F and 8 kappa^(-1/2) exp(-pi/(2 (2 kappa)^(1/2)))/sinh(pi/2^(1/2)) for the tidal field (both exits)", okU && okT,
  "uniform: " <> ToString[InputForm[gamU]] <> " (antiderivative " <> ToString[okAU] <> ", logarithm " <> ToString[limU] <>
   "); tidal: ratio to the formula " <> ToString[InputForm[FullSimplify[gamT/targetT, Assumptions -> {kk > 0, zm > 0}]]] <>
   " (antiderivative " <> ToString[okAT] <> ", logarithm " <> ToString[limT] <> ", Jacobi " <> ToString[okY] <> ")"];

(* ::Section:: *)
(*WT-5: classical motion in the growing field*)

(* ::Text:: *)
(*Along the fall the tidal strength grows as $\kappa^{\rm tid}_{\rm LG}=2/(9(T^{\rm fall}_{\rm LG})^2)$. The classical equations of the relative motion along and across the radius, $z''=2\kappa^{\rm tid}_{\rm LG}z$ and $x''=-\kappa^{\rm tid}_{\rm LG}x$, then have power-law solutions: $T^{4/3}$ and $T^{-1/3}$ along, $T^{2/3}$ and $T^{1/3}$ across (with $T=T^{\rm fall}_{\rm LG}$). The check lets `DSolve` find them and compares the solutions with given initial values with the closed forms of the Rust program, which uses them to test its integration.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WT-5: the Gaussian packet *)
solZ = DSolve[zf''[T] == 2 (2/(9 T^2)) zf[T], zf[T], T];
solX = DSolve[xf''[T] == -(2/(9 T^2)) xf[T], xf[T], T];
q = T/T0;
x1 = 1/5 q^(4/3) + 4/5 q^(-1/3); x2 = 3/5 T0 (q^(-1/3) - q^(4/3));
y1 = -q^(2/3) + 2 q^(1/3); y2 = 3 T0 (q^(1/3) - q^(2/3));
okODE = And @@ (Simplify[D[#1, {T, 2}] - #2 2/(9 T^2) #1, Assumptions -> {T > 0, T0 > 0}] === 0 & @@@
     {{x1, 2}, {x2, 2}, {y1, -1}, {y2, -1}});
ini = Simplify[{x1, D[x1, T], x2, D[x2, T], y1, D[y1, T], y2, D[y2, T]} /. T -> T0, Assumptions -> T0 > 0];
okIni = ini === {1, 0, 0, -1, 1, 0, 0, -1};
(* the general solutions of DSolve span the same powers *)
spanZ = Simplify[zf[T] /. solZ[[1]], Assumptions -> T > 0];
spanX = Simplify[xf[T] /. solX[[1]], Assumptions -> T > 0];
okSpan = Simplify[D[spanZ, {T, 2}] - 4/(9 T^2) spanZ] === 0 && Simplify[D[spanX, {T, 2}] + 2/(9 T^2) spanX] === 0;
check["WT-5", "(Rust LT-2; Python YT-6) in the field kappa = 2/(9 T^2) the closed forms X1, X2 (z'' = 2 kappa z: powers \
T^(4/3), T^(-1/3)) and Y1, Y2 (x'' = -kappa x: powers T^(2/3), T^(1/3)) solve their equations and have the values (1, 0) \
and (0, -1) of (value, d/dT) at T0; DSolve gives the same powers", okODE && okIni && okSpan,
  "equations " <> ToString[okODE] <> ", initial values " <> ToString[InputForm[ini]] <> "; DSolve: z = " <>
   ToString[InputForm[spanZ]] <> ", x = " <> ToString[InputForm[spanX]]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*8. The quasi-particles are Dirac particles: annihilation (Part VI, Section 33)*)

(* ::Text:: *)
(*The pairing in the superfluid mixes particles and antiparticles of the brochon, so its quasi-particles are mixtures too, and it is not obvious which of them can annihilate. Theorem L3 of the manuscript answers this: the quasi-particle field is $\Psi_{\rm LL}=\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG}\chi_{\rm LG}$ with $b^{\rm qp}_{\rm LG}=-D/(E+M)$, where $\chi_{\rm LG}$ is an ordinary Dirac field of the mass $E=(M^2+|D|^2)^{1/2}$; here, as in the theorem, $M$, $D$ and $E$ are short for the Dirac mass $M^{\rm CJT,D}_{\rm SF}$, the pair amplitude $D^{\rm CJT}_{\rm SF}$ and the gap $E^{\rm CJT,gap}_{\rm SF}$ of the superfluid. So the quasi-particles are the particles and antiparticles of a Dirac field, and the zeugon conserves their number $N^{\rm qp}_{\rm LG}$. This chapter checks the theorem exactly, including in the Fock space of the quantized field.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Annih` (the code is that of the script wolfram/source/lg_annih.wls) *)
Begin["LGNB`Annih`"];

(* ::Text:: *)
(*The matrices of the chiral basis and of the pairing term, on the eight components of the brochon (four Dirac components times two isospin components): `g8` the Dirac matrices, `pm` the matrix $\mathsf P_{\rm LG}$, `t3` the generator $T^3_{\rm LL}$, `fl` the reflection $\mathsf F^{\rm v}_{\rm LG}=\mathsf 1_4\otimes\tau^{\rm LL}_1$ that exchanges the two isospin components.*)

(* ::Input:: *)
(* the helper functions of the script lg_annih.wls *)
(* the matrices of the chiral basis of lord_gravity.md and of the pairing term *)
s0 = IdentityMatrix[2]; s1 = {{0, 1}, {1, 0}}; s2 = {{0, -I}, {I, 0}}; s3 = {{1, 0}, {0, -1}};
z2 = ConstantArray[0, {2, 2}];
blk[a_, b_, c_, d_] := ArrayFlatten[{{a, b}, {c, d}}];
g4 = {blk[z2, s0, s0, z2], blk[z2, s1, -s1, z2], blk[z2, s2, -s2, z2], blk[z2, s3, -s3, z2]};
gch = I g4[[1]] . g4[[2]] . g4[[3]] . g4[[4]];
el = {{0, 1}, {-1, 0}};
cll = ArrayFlatten[{{el, z2}, {z2, -el}}];
k3 = KroneckerProduct[cll . gch, el . s3];
pm = KroneckerProduct[g4[[1]], s0] . ConjugateTranspose[k3];
g8 = KroneckerProduct[#, s0] & /@ g4;
t3 = KroneckerProduct[IdentityMatrix[4], s3/2];
fl = KroneckerProduct[IdentityMatrix[4], s1];
zero8 = ConstantArray[0, {8, 8}];

(* ::Section:: *)
(*WA-1: the algebra of the pairing term*)

(* ::Text:: *)
(*The matrix $\mathsf P_{\rm LG}$ is real, antisymmetric and unitary; it commutes with the Dirac matrices and with $iT^3_{\rm LL}$ in the sense of WG-6; and the reflection $\mathsf F^{\rm v}_{\rm LG}$ commutes with the pairing map and turns $T^3_{\rm LL}$ into $-T^3_{\rm LL}$. The reflection is the element of the unbroken group $O(2)$ that the selection rules of annihilation use.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WA-1 *)
ok1 = AllTrue[Flatten[pm], Im[#] === 0 &] && pm + Transpose[pm] === zero8 && pm . ConjugateTranspose[pm] === IdentityMatrix[8] &&
   AllTrue[g8, pm . Conjugate[#] - # . pm === zero8 &] && pm . Conjugate[I t3] - I t3 . pm === zero8 &&
   pm . Conjugate[fl] - fl . pm === zero8 && fl . t3 . fl + t3 === zero8 && AllTrue[g8, fl . # - # . fl === zero8 &];
check["WA-1", "(Rust AN-1; Python YA-1) P is real, antisymmetric and unitary; P (gamma^a)^* = gamma^a P; P (i T^3)^* = i T^3 P; \
F = 1_4 (x) tau_1 commutes with C and with the gamma^a, and F T^3 F = -T^3 (exact)", ok1, "all identities: " <> ToString[ok1]];

(* ::Section:: *)
(*WA-2: Theorem L3 for a plane wave*)

(* ::Text:: *)
(*The core of Theorem L3 is the identity $[i\gamma\cdot\partial-M-D\,\mathcal C_{\rm LG}](1+b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG})=(1-b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG})[i\gamma\cdot\partial-E]$: the operator of the superfluid, applied to $\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG}\chi_{\rm LG}$, gives the same as the Dirac operator of the mass $E$ applied to $\chi_{\rm LG}$, up to the factor on the left, which can be inverted. The check verifies it on a plane wave with a symbolic spinor (16 real symbols) and a symbolic four-momentum that need not satisfy any mass condition, with symbolic $M$, $|D|$ and phase of $D$. As a control, the wrong sign of $b^{\rm qp}_{\rm LG}$ fails.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WA-2 *)
(* a field A e^(-i p.x) + B e^(+i p.x) as the pair {A, B}: i gamma d -> {g.p A, -g.p B}; C -> {P B^*, P A^*} *)
uvec = Table[x[k] + I y[k], {k, 8}];
pv = {p0, p1, p2, p3};
gp = g8[[1]] p0 - g8[[2]] p1 - g8[[3]] p2 - g8[[4]] p3;
dd = d Exp[I del]; ee = Sqrt[mm^2 + d^2];
conjv[v_] := ComplexExpand[Conjugate[v]];
dsl[{a_, b_}] := {gp . a, -gp . b};
cm[{a_, b_}] := {pm . conjv[b], pm . conjv[a]};
res[bb_] := Module[{chi = {uvec, ConstantArray[0, 8]}, psi, lhs, w, rhs},
   psi = chi + bb cm[chi];
   lhs = dsl[psi] - mm psi - dd cm[psi];
   w = dsl[chi] - ee chi;
   rhs = w - bb cm[w];
   Simplify[ComplexExpand[Flatten[lhs - rhs]], Assumptions -> {mm > 0, d > 0, Element[del, Reals]}]];
r2 = res[-dd/(ee + mm)];
c2 = res[dd/(ee + mm)];
ok2 = AllTrue[r2, # === 0 &] && ! AllTrue[c2, # === 0 &];
check["WA-2", "(Rust AN-2; Python YA-2) [i gamma d - M - D C](1 + b C) = (1 - b C)[i gamma d - E], b = -D/(E + M), on a plane \
wave with a symbolic spinor (16 real symbols) and four-momentum (off the mass shell), symbolic M, |D| and phase of D; with -b it \
fails (control)", ok2, "16 components vanish: " <> ToString[AllTrue[r2, # === 0 &]] <> "; control fails: " <>
   ToString[! AllTrue[c2, # === 0 &]]];

(* ::Section:: *)
(*WA-3: the quantized field and its zeugon current*)

(* ::Text:: *)
(*The quantized field of eight modes lives in a Fock space of dimension $2^8=256$, built with the construction of Jordan and Wigner, in exact rational arithmetic with $b^{\rm qp}_{\rm LG}=\tfrac37+\tfrac25i$. The check shows that $\Psi=\chi+b^{\rm qp}_{\rm LG}\,\mathsf P_{\rm LG}\chi^\dagger$ has the anticommutators of a free field up to the factor $1+|b^{\rm qp}_{\rm LG}|^2$, and that its zeugon current is exactly $1+|b^{\rm qp}_{\rm LG}|^2$ times that of $\chi$, up to a constant: the current contains no term that creates or destroys two quasi-particles, so the zeugon cannot make two quasi-particles of opposite charge annihilate unless they are a particle and its own antiparticle.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WA-3 *)
gmu = Table[g8[[1]] . g8[[m]] . t3, {m, 4}];
sym = AllTrue[gmu, (# . pm) - Transpose[# . pm] === zero8 &];
nrm = AllTrue[gmu, ConjugateTranspose[pm] . # . pm + Transpose[#] === zero8 &];
(* the Fock space: c_i = Z (x) ... (x) Z (x) a (x) 1 (x) ... (x) 1 *)
aop = SparseArray[{{1, 2} -> 1}, {2, 2}]; zop = SparseArray[{{1, 1} -> 1, {2, 2} -> -1}]; one = SparseArray[IdentityMatrix[2]];
cop[i_] := KroneckerProduct @@ Table[Which[k < i, zop, k == i, aop, True, one], {k, 1, 8}];
cs = Table[cop[i], {i, 8}];
cds = ConjugateTranspose /@ cs;
bq = 3/7 + 2 I/5; nb = 1 + Abs[bq]^2;
psi = Table[cs[[i]] + bq Sum[pm[[i, j]] cds[[j]], {j, 8}], {i, 8}];
psid = ConjugateTranspose /@ psi;
id256 = SparseArray[IdentityMatrix[256]];
car1 = AllTrue[Flatten[Table[Normal[psi[[i]] . psid[[j]] + psid[[j]] . psi[[i]] - If[i == j, nb, 0] id256] === ConstantArray[0, {256, 256}],
      {i, 8}, {j, 8}]], TrueQ];
car2 = AllTrue[Flatten[Table[Normal[psi[[i]] . psi[[j]] + psi[[j]] . psi[[i]]] === ConstantArray[0, {256, 256}], {i, 8}, {j, 8}]], TrueQ];
curr = AllTrue[Table[Module[{G = gmu[[m]], jp, jc},
      jp = Sum[psid[[i]] . Sum[G[[i, j]] psi[[j]], {j, 8}], {i, 8}];
      jc = Sum[cds[[i]] . Sum[G[[i, j]] cs[[j]], {j, 8}], {i, 8}];
      Normal[jp - nb jc - (Tr[jp - nb jc]/256) id256] === ConstantArray[0, {256, 256}]], {m, 4}], TrueQ];
check["WA-3", "(Rust AN-3; Python YA-3) gamma^0 gamma^mu T^3 P is symmetric and P^dag (gamma^0 gamma^mu T^3) P = \
-(gamma^0 gamma^mu T^3)^T; in the Fock space of eight modes (exact, b = 3/7 + 2i/5) {Psi_i, Psi_j^dag} = (1 + |b|^2) delta_ij, \
{Psi_i, Psi_j} = 0 and J[Psi] = (1 + |b|^2) J[chi] up to a multiple of the identity", sym && nrm && car1 && car2 && curr,
  "symmetric " <> ToString[sym] <> ", normal part " <> ToString[nrm] <> ", anticommutators " <> ToString[car1 && car2] <>
   ", currents " <> ToString[curr]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*9. The worked examples (Parts II and III, Appendices A and B)*)

(* ::Text:: *)
(*The manuscript illustrates its objects with explicit examples, chosen so that every number is a simple fraction. This chapter checks them exactly. In the code: `mll[xi]` is the $2\times2$ matrix $M_{\rm LL}(\mathbf b)$ of a bi-quaternion with the coefficients `xi`, `ndet` the polarized determinant $N_{\rm LG}$, `dirac` Lord's Dirac matrices of a set of four $2\times2$ matrices, `lorentzOf` the Lorentz matrix $\mathsf L_{\rm LL}(\mathbf g)$ of a unit bi-quaternion.*)

(* ::Input:: *)
(* the symbols that the code of this chapter creates live in the context LGNB`Worked` (the code is that of the script wolfram/source/lg_worked.wls) *)
Begin["LGNB`Worked`"];

(* ::Input:: *)
(* the helper functions of the script lg_worked.wls *)
zeroQ[m_] := AllTrue[Flatten[{Simplify[m]}], # === 0 &];

(* the matrices of the manuscript, written again *)
sig = {IdentityMatrix[2], PauliMatrix[1], PauliMatrix[2], PauliMatrix[3]};
eps = {{0, 1}, {-1, 0}};
bar[x_] := -eps . Transpose[x] . eps;                                    (* Lord's (1.2), the adjugate *)
mll[xi_] := xi[[1]] sig[[1]] - I (xi[[2]] sig[[2]] + xi[[3]] sig[[3]] + xi[[4]] sig[[4]]);
ndet[x_, y_] := Simplify[(Det[x + y] - Det[x] - Det[y])/2];
dirac[s_] := ArrayFlatten[{{0, s}, {bar[s], 0}}];
eta = DiagonalMatrix[{1, -1, -1, -1}];
gam = dirac /@ sig;
herm[m_] := ConjugateTranspose[m];
lorentzOf[mg_] := Table[Re[Tr[sig[[p]] . mg . sig[[q]] . herm[mg]]/2], {p, 4}, {q, 4}];

(* ::Section:: *)
(*WW-1: a change of representation by a boost*)

(* ::Text:: *)
(*The unit bi-quaternion $\mathbf g=\tfrac54\mathbf e_0+\tfrac34i\,\mathbf e_3$ is a boost along the third axis with the velocity $\tfrac{15}{17}$ (its Lorentz matrix has the entries $\tfrac{17}8$ and $\tfrac{15}8$, and $(\tfrac{17}8)^2-(\tfrac{15}8)^2=1$). Its matrix $S=\mathrm{diag}(\tfrac12,2)$ changes Lord's matrices into $\mathrm{diag}(\tfrac14,4)$, $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$ and $\mathrm{diag}(\tfrac14,-4)$. The check verifies, exactly, that the new set is Hermitian with the right polarized determinants, that its Dirac matrices are those of the old set conjugated by the spinor matrix of the boost, that they equal the old ones combined with the inverse Lorentz matrix, and that the new set is the frame $(\tfrac{17}8,0,0,-\tfrac{15}8)$, $(0,1,0,0)$, $(0,0,1,0)$, $(-\tfrac{15}8,0,0,\tfrac{17}8)$ of M31, whose time vector moves with the velocity $-\tfrac{15}{17}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WW-1 *)
g = {5/4, 0, 0, 3/4 I};
mg = mll[g];
s = mll[Conjugate[g]];
sp = Table[s . sig[[m]] . herm[s], {m, 4}];
expect = {DiagonalMatrix[{1/4, 4}], sig[[2]], sig[[3]], DiagonalMatrix[{1/4, -4}]};
s4 = ArrayFlatten[{{s, 0}, {0, Inverse[herm[s]]}}];
s4ll = ArrayFlatten[{{Inverse[herm[mg]], 0}, {0, mg}}];
gp = dirac /@ sp;
lm = lorentzOf[mg];
linv = Inverse[lm];
pts = Table[{Re[Tr[sp[[a]]]/2], Sequence @@ Table[Im[I Tr[sig[[j + 1]] . sp[[a]]]/2], {j, 3}]}, {a, 4}];
ok1 = zeroQ[s - Inverse[herm[mg]]] && zeroQ[s - DiagonalMatrix[{1/2, 2}]] && zeroQ[sp - expect] &&
  zeroQ[sp - (herm /@ sp)] && zeroQ[Table[bar[sp[[m]]] - Inverse[herm[s]] . bar[sig[[m]]] . Inverse[s], {m, 4}]] &&
  zeroQ[Table[ndet[sp[[a]], sp[[b]]] - eta[[a, b]], {a, 4}, {b, 4}]] && zeroQ[s4 - s4ll] &&
  zeroQ[Table[gp[[a]] - s4 . gam[[a]] . Inverse[s4], {a, 4}]] &&
  zeroQ[Table[(gp[[a]] . gp[[b]] + gp[[b]] . gp[[a]])/2 - eta[[a, b]] IdentityMatrix[4], {a, 4}, {b, 4}]] &&
  zeroQ[lm - {{17/8, 0, 0, 15/8}, {0, 1, 0, 0}, {0, 0, 1, 0}, {15/8, 0, 0, 17/8}}] &&
  zeroQ[Table[gp[[a]] - Sum[linv[[a, b]] gam[[b]], {b, 4}], {a, 4}]] &&
  zeroQ[Table[Inverse[s4ll] . gam[[p]] . s4ll - Sum[lm[[p, q]] gam[[q]], {q, 4}], {p, 4}]] &&
  zeroQ[pts - {{17/8, 0, 0, -15/8}, {0, 1, 0, 0}, {0, 0, 1, 0}, {-15/8, 0, 0, 17/8}}] &&
  pts[[1, 4]]/pts[[1, 1]] === -15/17;
check["WW-1", "(LW-1 to LW-3) the change of representation by S = M_LL(gtilde) of the boost (5/4) e_0 + (3/4) i e_3: \
S = diag(1/2, 2), the matrices sigma'_mu, their bars and polarized determinants, Lord's Dirac matrices = S4 gamma^a S4^-1 \
with S4 = S_LL(g), the Clifford relation, the Lorentz matrix L of the boost, gamma'^a = (L^-1)_ab gamma^b, (9.3), and the \
frame of M31 with the velocity -15/17 (exact)", ok1,
  "sigma'_0 = " <> ToString[InputForm[sp[[1]]]] <> ", sigma'_3 = " <> ToString[InputForm[sp[[4]]]] <> ", L^-1 = " <>
   ToString[InputForm[linv]] <> ", f'_0 = " <> ToString[InputForm[pts[[1]]]]];

(* ::Section:: *)
(*WW-2: a set that is not a representation*)

(* ::Text:: *)
(*Lord writes that any linearly independent set of four Hermitian matrices will do; the manuscript shows that this is too much. The set $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3+\tfrac12\sigma^{\rm LL}_0$ is Hermitian and linearly independent, but the polarized determinant of the first and the last is $\tfrac12$ instead of 0, and their Dirac matrices do not anticommute. So a valid set must also satisfy (1.3).*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WW-2 *)
s2 = {sig[[1]], sig[[2]], sig[[3]], sig[[4]] + sig[[1]]/2};
coef = Table[Tr[s2[[k]] . sig[[r]]]/2, {k, 4}, {r, 4}];
g2 = dirac /@ s2;
ok2 = zeroQ[s2 - (herm /@ s2)] && FreeQ[coef, Complex] && Det[coef] === 1 && ndet[s2[[1]], s2[[4]]] === 1/2 &&
  zeroQ[(g2[[1]] . g2[[4]] + g2[[4]] . g2[[1]])/2 - IdentityMatrix[4]/2];
check["WW-2", "(LW-4) the set (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2): Hermitian, linearly independent (the \
determinant of the coefficients is 1), N_LG(sigma''_0, sigma''_3) = 1/2, (1/2){gamma''^0, gamma''^3} = (1/2) 1_4 (exact)", ok2,
  "det " <> ToString[Det[coef]] <> ", N03 " <> ToString[InputForm[ndet[s2[[1]], s2[[4]]]]]];

(* ::Section:: *)
(*WW-3: a plane wave of the brochon and its energy and momentum*)

(* ::Text:: *)
(*The plane wave with the spinor $u_{\rm LL}=(\tfrac12,0,2,0)$, the momentum $P_{\rm LG}=(\tfrac{17}8,0,0,\tfrac{15}8)$ and the mass 1 (the boosted rest spinor of Appendix A) is checked with symbolic coordinates: it solves the free Dirac equation, its current is twice its momentum, its Lagrangian density vanishes (as it must on a solution), and its energy-momentum tensor (16.2) is twice the product of the momentum with itself, with the trace 2: the tensor of a stream of particles of the momentum $P_{\rm LG}$.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WW-3 *)
xs = {x0, x1, x2, x3};
pUp = {17/8, 0, 0, 15/8}; pLow = eta . pUp; mm = 1;
u = s4ll . {1, 0, 1, 0};
w8 = Flatten[Table[{u[[k]], 0}, {k, 4}]];                                (* u (x) (1, 0) *)
g8 = KroneckerProduct[#, IdentityMatrix[2]] & /@ gam;
psi = w8 Exp[-I pLow . xs];
(* the complex conjugate of an expression whose symbols (the coordinates) are real: every I becomes -I *)
conj[e_] := e /. Complex[a_, b_] :> Complex[a, -b];
psiBar[v_] := conj[v] . g8[[1]];
dpsi = Table[D[psi, xs[[k]]], {k, 4}];
eqn = Sum[I g8[[k]] . dpsi[[k]], {k, 4}] - mm psi;
j = Table[Simplify[psiBar[psi] . g8[[k]] . psi, Element[xs, Reals]], {k, 4}];
lag = Simplify[I/2 Sum[psiBar[psi] . g8[[k]] . dpsi[[k]] - psiBar[dpsi[[k]]] . g8[[k]] . psi, {k, 4}] -
    mm psiBar[psi] . psi, Element[xs, Reals]];
glow = Table[Sum[eta[[mu, nu]] g8[[nu]], {nu, 4}], {mu, 4}];
tmn = Table[Simplify[I/4 (psiBar[psi] . glow[[mu]] . dpsi[[nu]] + psiBar[psi] . glow[[nu]] . dpsi[[mu]] -
        psiBar[dpsi[[mu]]] . glow[[nu]] . psi - psiBar[dpsi[[nu]]] . glow[[mu]] . psi) - eta[[mu, nu]] lag,
     Element[xs, Reals]], {mu, 4}, {nu, 4}];
ok3 = u === {1/2, 0, 2, 0} && zeroQ[Simplify[eqn Exp[I pLow . xs]]] && zeroQ[j - 2 pUp] && lag === 0 &&
  zeroQ[tmn - 2 Outer[Times, pLow, pLow]] && Simplify[Sum[eta[[a, a]] tmn[[a, a]], {a, 4}]] === 2 &&
  Simplify[psiBar[psi] . psi, Element[xs, Reals]] === 2;
check["WW-3", "(LW-5) the plane wave (u (x) (1, 0)) Exp[-i p.x], u = (1/2, 0, 2, 0), p = (17/8, 0, 0, 15/8), m = 1: the \
free field equation, the current 2 p^mu, the Lagrangian density 0, the energy-momentum tensor (16.2) 2 p_mu p_nu (lower \
indices) with the trace m Psi-bar Psi = 2 (symbolic coordinates)", ok3,
  "j = " <> ToString[InputForm[j]] <> ", T_00 = " <> ToString[InputForm[tmn[[1, 1]]]] <> ", T_03 = " <>
   ToString[InputForm[tmn[[1, 4]]]] <> ", T_33 = " <> ToString[InputForm[tmn[[4, 4]]]]];

(* ::Section:: *)
(*WW-4: the quasi-particle energies of the superfluid*)

(* ::Text:: *)
(*In the polar ground state of the brochon model the Bogoliubov-de Gennes matrix (a $16\times16$ matrix that couples the brochon at the momentum $\vec q$ with its conjugate at $-\vec q$) has, for symbolic real momentum, Dirac mass $M$ and pair amplitude $D$ (in the code `mM` and `dd`), a square that is $(|\vec q|^2+M^2+D^2)$ times the unit matrix, and the trace 0. Hence its eigenvalues are $\pm E(\vec q)$, eight times each, with $E(\vec q)=(|\vec q|^2+M^2+D^2)^{1/2}$ (B.18): the energy of a quasi-particle, whose smallest value, at $\vec q=0$, is the gap.*)

(* ::Input:: *)
(* ------------------------------------------------------------------ WW-4 *)
cc = I gam[[3]] . gam[[1]];                                              (* C = i gamma^2 gamma^0 *)
gch = I gam[[1]] . gam[[2]] . gam[[3]] . gam[[4]];
k3 = KroneckerProduct[cc . gch, eps . sig[[4]]];
alpha = Table[gam[[1]] . gam[[k + 1]], {k, 3}];
h8[q_, mass_] := KroneckerProduct[Sum[alpha[[k]] q[[k]], {k, 3}] + gam[[1]] mass, IdentityMatrix[2]];
qv = {q1, q2, q3};
pm = dd ConjugateTranspose[k3];                                          (* D (K^3)^dag, D real *)
pmDag = dd k3;                                                           (* its Hermitian conjugate D K^3 *)
hb = ArrayFlatten[{{h8[qv, mM], pm}, {pmDag, -Transpose[h8[-qv, mM]]}}];
assum = Element[{q1, q2, q3, mM, dd}, Reals];
hsq = Simplify[ConjugateTranspose[hb] - hb, assum];
sq = Simplify[hb . hb - (q1^2 + q2^2 + q3^2 + mM^2 + dd^2) IdentityMatrix[16], assum];
ok4 = zeroQ[hsq] && zeroQ[sq] && Simplify[Tr[hb]] === 0;
check["WW-4", "(LW-6) the Bogoliubov-de Gennes matrix with the normal self-energy (M - m_LL) beta (x) 1_2 and the pair \
self-energy D (K^3)^dag, for symbolic real q, M, D: Hermitian, its square is (|q|^2 + M^2 + D^2) 1_16 and its trace is 0, \
so its eigenvalues are -E(q) and +E(q) eight times each (exact)", ok4,
  "H^2 - E^2 1 = 0: " <> ToString[zeroQ[sq]] <> ", tr H = " <> ToString[Simplify[Tr[hb]]]];

(* ::Section:: *)
(*WW-5: the gap equation*)

(* ::Text:: *)
(*The size of the gap is fixed by a gap equation that contains the integral $I^{(1)}_{\rm SF}(M)=\int_0^1q^2(q^2+M^2)^{-1/2}dq$ (A.19, with the cutoff $\Lambda_{\rm LL}=1$). The check computes the integral with `Integrate`, compares it with the closed form of (A.19) symbolically and as numbers of 40 digits, and then puts the printed numbers of the manuscript into the gap equation of the method CJT, $g^{{\rm CJT},{\rm P}}_{\rm SF}\,I^{(1)}_{\rm SF}(E^{\rm CJT,gap}_{\rm SF})/\pi^2=1$ with $g^{{\rm CJT},{\rm P}}_{\rm SF}=23.7$, and into $\pi^2/I^{(1)}_{\rm SF}(\tfrac15)=G^{\rm crit}_{\rm LL}$: both hold to the ten printed digits. The printed numbers come from the fragment file `lg_examples.md`, in the closed cell below.*)

(* ::Input::Closed:: *)
(* the text of the file provenance/generated/lg_examples.md of the folder Revision_Lord_Gravity (SHA-256 a76065fb266da44b4b3590bbf8b46ab36f826957dd599f4e6396fbc589dd3ecc), written into the notebook so that it reads no file *)
lgExamplesText = "<!-- generated by code/scripts/write_lg_examples_fragments.py; copied unchanged from Revision_SM_Higgs/provenance/generated/sh_examples.md, Revision_SM_Higgs/provenance/generated/part2.md; do not edit -->
<!-- BEGIN ex10_gcrit_m0 -->
$19.739209$
<!-- END ex10_gcrit_m0 -->
<!-- BEGIN ex10_gcrit_m0_2 -->
$21.286610$
<!-- END ex10_gcrit_m0_2 -->
<!-- BEGIN ex10_k3_entries -->
$(\\mathsf K^3_{\\rm LL})_{03}=(\\mathsf K^3_{\\rm LL})_{12}=(\\mathsf K^3_{\\rm LL})_{47}=(\\mathsf K^3_{\\rm LL})_{56}=1$ and $(\\mathsf K^3_{\\rm LL})_{21}=(\\mathsf K^3_{\\rm LL})_{30}=(\\mathsf K^3_{\\rm LL})_{65}=(\\mathsf K^3_{\\rm LL})_{74}=-1$
<!-- END ex10_k3_entries -->
<!-- BEGIN ex10_test_d_zeta1 -->
$0.0639341652$
<!-- END ex10_test_d_zeta1 -->
<!-- BEGIN ex10_test_e_minus_q0_3 -->
$0.1661927450$
<!-- END ex10_test_e_minus_q0_3 -->
<!-- BEGIN ex10_test_e_plus_q0_3 -->
$0.8652920221$
<!-- END ex10_test_e_plus_q0_3 -->
<!-- BEGIN ex10_test_gap -->
$0.0904165695$
<!-- END ex10_test_gap -->
<!-- BEGIN ex10_test_omega_unitary -->
$-9.7206051525\\times10^{-5}$
<!-- END ex10_test_omega_unitary -->
<!-- BEGIN ex10_test_omega_zeta1 -->
$-4.8603025763\\times10^{-5}$
<!-- END ex10_test_omega_zeta1 -->
<!-- BEGIN ex10_test_qf -->
$0.4582575695$
<!-- END ex10_test_qf -->
<!-- BEGIN ex10_vacuum_g30_gap -->
$0.6641405875$
<!-- END ex10_vacuum_g30_gap -->
<!-- BEGIN ex10_vacuum_g30_omega_min -->
$-2.3656704935\\times10^{-3}$
<!-- END ex10_vacuum_g30_omega_min -->
<!-- BEGIN ex1_assoc_e1_e2_e4 -->
$2\\mathbf e_{7}$
<!-- END ex1_assoc_e1_e2_e4 -->
<!-- BEGIN ex1_e1_e2e4 -->
$-\\mathbf e_{7}$
<!-- END ex1_e1_e2e4 -->
<!-- BEGIN ex1_e1e2 -->
$\\mathbf e_{3}$
<!-- END ex1_e1e2 -->
<!-- BEGIN ex1_e1e2_e4 -->
$\\mathbf e_{7}$
<!-- END ex1_e1e2_e4 -->
<!-- BEGIN ex1_e2e1 -->
$-\\mathbf e_{3}$
<!-- END ex1_e2e1 -->
<!-- BEGIN ex1_moufang_failures -->
$0$
<!-- END ex1_moufang_failures -->
<!-- BEGIN ex1_moufang_total -->
4096
<!-- END ex1_moufang_total -->
<!-- BEGIN ex1_nonassoc_triples -->
1344 of 4096
<!-- END ex1_nonassoc_triples -->
<!-- BEGIN ex1_noncommuting_pairs -->
168 of 256
<!-- END ex1_noncommuting_pairs -->
<!-- BEGIN ex1_order -->
$16$
<!-- END ex1_order -->
<!-- BEGIN ex2_assoc_x_y_z -->
$-\\tfrac{64}{75}\\mathbf e_{3}+\\tfrac{16}{25}\\mathbf e_{5}+\\tfrac{64}{75}\\mathbf e_{6}$
<!-- END ex2_assoc_x_y_z -->
<!-- BEGIN ex2_x -->
$\\tfrac{3}{5}\\mathbf e_{0}+\\tfrac{4}{5}\\mathbf e_{1}$
<!-- END ex2_x -->
<!-- BEGIN ex2_x_yz -->
$-\\tfrac{8}{25}\\mathbf e_{0}+\\tfrac{6}{25}\\mathbf e_{1}+\\tfrac{11}{25}\\mathbf e_{2}+\\tfrac{44}{75}\\mathbf e_{3}-\\tfrac{2}{25}\\mathbf e_{4}-\\tfrac{8}{75}\\mathbf e_{5}-\\tfrac{32}{75}\\mathbf e_{6}-\\tfrac{8}{25}\\mathbf e_{7}$
<!-- END ex2_x_yz -->
<!-- BEGIN ex2_xbar -->
$\\tfrac{3}{5}\\mathbf e_{0}-\\tfrac{4}{5}\\mathbf e_{1}$
<!-- END ex2_xbar -->
<!-- BEGIN ex2_xy -->
$\\tfrac{9}{25}\\mathbf e_{2}+\\tfrac{12}{25}\\mathbf e_{3}+\\tfrac{12}{25}\\mathbf e_{4}+\\tfrac{16}{25}\\mathbf e_{5}$
<!-- END ex2_xy -->
<!-- BEGIN ex2_xy_z -->
$-\\tfrac{8}{25}\\mathbf e_{0}+\\tfrac{6}{25}\\mathbf e_{1}+\\tfrac{11}{25}\\mathbf e_{2}-\\tfrac{4}{15}\\mathbf e_{3}-\\tfrac{2}{25}\\mathbf e_{4}+\\tfrac{8}{15}\\mathbf e_{5}+\\tfrac{32}{75}\\mathbf e_{6}-\\tfrac{8}{25}\\mathbf e_{7}$
<!-- END ex2_xy_z -->
<!-- BEGIN ex2_y -->
$\\tfrac{3}{5}\\mathbf e_{2}+\\tfrac{4}{5}\\mathbf e_{4}$
<!-- END ex2_y -->
<!-- BEGIN ex2_z -->
$\\tfrac{1}{3}\\mathbf e_{0}+\\tfrac{2}{3}\\mathbf e_{3}+\\tfrac{2}{3}\\mathbf e_{6}$
<!-- END ex2_z -->
<!-- BEGIN ex3_assoc_grot_gboost_s -->
$-\\tfrac{24}{25}i\\mathbf e_{6}$
<!-- END ex3_assoc_grot_gboost_s -->
<!-- BEGIN ex3_boost_of_rest -->
$(\\tfrac{17}{8},0,0,\\tfrac{15}{8})$
<!-- END ex3_boost_of_rest -->
<!-- BEGIN ex3_conj_g_e4_gbar -->
$\\tfrac{17}{8}\\mathbf e_{4}+\\tfrac{15}{8}i\\mathbf e_{7}$
<!-- END ex3_conj_g_e4_gbar -->
<!-- BEGIN ex3_e4_e5 -->
$2\\mathbf e_{1}=4J^1_{\\rm LL}$
<!-- END ex3_e4_e5 -->
<!-- BEGIN ex3_e4_i_e5 -->
$2i\\mathbf e_{1}=4K^1_{\\rm LL}$
<!-- END ex3_e4_i_e5 -->
<!-- BEGIN ex3_g_boost -->
$\\tfrac{5}{4}\\mathbf e_{0}+\\tfrac{3}{4}i\\mathbf e_{3}$
<!-- END ex3_g_boost -->
<!-- BEGIN ex3_g_rot -->
$\\tfrac{3}{5}\\mathbf e_{0}+\\tfrac{4}{5}\\mathbf e_{1}$
<!-- END ex3_g_rot -->
<!-- BEGIN ex3_j1_k2 -->
$K^3_{\\rm LL}$
<!-- END ex3_j1_k2 -->
<!-- BEGIN ex3_jac_e4_e5_e6 -->
$-12\\mathbf e_{7}$
<!-- END ex3_jac_e4_e5_e6 -->
<!-- BEGIN ex3_l_g_boost -->
$\\begin{pmatrix}\\tfrac{17}{8}&0&0&\\tfrac{15}{8}\\\\0&1&0&0\\\\0&0&1&0\\\\\\tfrac{15}{8}&0&0&\\tfrac{17}{8}\\end{pmatrix}$
<!-- END ex3_l_g_boost -->
<!-- BEGIN ex3_l_g_rot -->
$\\begin{pmatrix}1&0&0&0\\\\0&1&0&0\\\\0&0&-\\tfrac{7}{25}&-\\tfrac{24}{25}\\\\0&0&\\tfrac{24}{25}&-\\tfrac{7}{25}\\end{pmatrix}$
<!-- END ex3_l_g_rot -->
<!-- BEGIN ex3_m_g_boost -->
$\\begin{pmatrix}2&0\\\\0&\\tfrac{1}{2}\\end{pmatrix}$
<!-- END ex3_m_g_boost -->
<!-- BEGIN ex3_m_g_rot -->
$\\begin{pmatrix}\\tfrac{3}{5}&-\\tfrac{4}{5}i\\\\-\\tfrac{4}{5}i&\\tfrac{3}{5}\\end{pmatrix}$
<!-- END ex3_m_g_rot -->
<!-- BEGIN ex3_parity_of_2_1_3_5 -->
$2\\mathbf e_{0}-i\\mathbf e_{1}-3i\\mathbf e_{2}-5i\\mathbf e_{3}$
<!-- END ex3_parity_of_2_1_3_5 -->
<!-- BEGIN ex3_s -->
$\\tfrac{3}{5}\\mathbf e_{0}+\\tfrac{4}{5}\\mathbf e_{4}$
<!-- END ex3_s -->
<!-- BEGIN ex3_w -->
$\\tfrac{3}{4}\\mathbf e_{0}+\\tfrac{9}{20}i\\mathbf e_{3}+\\mathbf e_{4}+\\tfrac{3}{5}i\\mathbf e_{7}$
<!-- END ex3_w -->
<!-- BEGIN ex3_wbar -->
$\\tfrac{3}{4}\\mathbf e_{0}-\\tfrac{9}{20}i\\mathbf e_{3}-\\mathbf e_{4}-\\tfrac{3}{5}i\\mathbf e_{7}$
<!-- END ex3_wbar -->
<!-- BEGIN ex3_x_solving_wx_e5 -->
$-\\mathbf e_{1}+\\tfrac{3}{5}i\\mathbf e_{2}+\\tfrac{3}{4}\\mathbf e_{5}+\\tfrac{9}{20}i\\mathbf e_{6}$
<!-- END ex3_x_solving_wx_e5 -->
<!-- BEGIN ex4_gamma_ch_u_up -->
$(-\\tfrac{1}{2},0,2,0)$
<!-- END ex4_gamma_ch_u_up -->
<!-- BEGIN ex4_mass -->
$1$
<!-- END ex4_mass -->
<!-- BEGIN ex4_momentum -->
$(\\tfrac{17}{8},0,0,\\tfrac{15}{8})$
<!-- END ex4_momentum -->
<!-- BEGIN ex4_u_down -->
$(0,2,0,\\tfrac{1}{2})$
<!-- END ex4_u_down -->
<!-- BEGIN ex4_u_rest_up -->
$(1,0,1,0)$
<!-- END ex4_u_rest_up -->
<!-- BEGIN ex4_u_up -->
$(\\tfrac{1}{2},0,2,0)$
<!-- END ex4_u_up -->
<!-- BEGIN ex4_ubar_u -->
$2$
<!-- END ex4_ubar_u -->
<!-- BEGIN ex4_udag_u -->
$\\tfrac{17}{4}$
<!-- END ex4_udag_u -->
<!-- BEGIN ex5_r_0_0_1 -->
$(0,-\\tfrac{24}{25},-\\tfrac{7}{25})$
<!-- END ex5_r_0_0_1 -->
<!-- BEGIN ex5_r_of_u -->
$\\begin{pmatrix}1&0&0\\\\0&-\\tfrac{7}{25}&-\\tfrac{24}{25}\\\\0&\\tfrac{24}{25}&-\\tfrac{7}{25}\\end{pmatrix}$
<!-- END ex5_r_of_u -->
<!-- BEGIN ex5_u -->
$\\begin{pmatrix}\\tfrac{3}{5}&-\\tfrac{4}{5}i\\\\-\\tfrac{4}{5}i&\\tfrac{3}{5}\\end{pmatrix}$
<!-- END ex5_u -->
<!-- BEGIN ex6_n_psi -->
$(0,0,1)$
<!-- END ex6_n_psi -->
<!-- BEGIN ex6_n_u_psi -->
$(0,-\\tfrac{24}{25},-\\tfrac{7}{25})$
<!-- END ex6_n_u_psi -->
<!-- BEGIN ex6_psi -->
$(1,0)$
<!-- END ex6_psi -->
<!-- BEGIN ex6_u_psi -->
$(\\tfrac{3}{5},-\\tfrac{4}{5}i)$
<!-- END ex6_u_psi -->
<!-- BEGIN ex7_e -->
$\\begin{pmatrix}0&1\\\\-1&0\\end{pmatrix}$
<!-- END ex7_e -->
<!-- BEGIN ex7_e_tau1 -->
$\\begin{pmatrix}1&0\\\\0&-1\\end{pmatrix}$
<!-- END ex7_e_tau1 -->
<!-- BEGIN ex7_e_tau1_minus_i_e_tau2 -->
$\\begin{pmatrix}2&0\\\\0&0\\end{pmatrix}$
<!-- END ex7_e_tau1_minus_i_e_tau2 -->
<!-- BEGIN ex7_e_tau1_plus_i_e_tau2 -->
$\\begin{pmatrix}0&0\\\\0&-2\\end{pmatrix}$
<!-- END ex7_e_tau1_plus_i_e_tau2 -->
<!-- BEGIN ex7_e_tau2 -->
$\\begin{pmatrix}i&0\\\\0&i\\end{pmatrix}$
<!-- END ex7_e_tau2 -->
<!-- BEGIN ex7_e_tau3 -->
$\\begin{pmatrix}0&-1\\\\-1&0\\end{pmatrix}$
<!-- END ex7_e_tau3 -->
<!-- BEGIN ex7_t2_eigenvalues -->
$0, 2, 2, 2$
<!-- END ex7_t2_eigenvalues -->
<!-- BEGIN ex7_x_of_u -->
$\\begin{pmatrix}1&0&0\\\\0&-\\tfrac{7}{25}&-\\tfrac{24}{25}\\\\0&\\tfrac{24}{25}&-\\tfrac{7}{25}\\end{pmatrix}$
<!-- END ex7_x_of_u -->
<!-- BEGIN ex8_1xu_psi_up -->
$(\\tfrac{3}{10},-\\tfrac{2}{5}i,0,0,\\tfrac{6}{5},-\\tfrac{8}{5}i,0,0)$
<!-- END ex8_1xu_psi_up -->
<!-- BEGIN ex8_psi_down -->
$(0,\\tfrac{1}{2},0,0,0,2,0,0)$
<!-- END ex8_psi_down -->
<!-- BEGIN ex8_psi_up -->
$(\\tfrac{1}{2},0,0,0,2,0,0,0)$
<!-- END ex8_psi_up -->
<!-- BEGIN ex9_bracket_span_rank -->
$6$
<!-- END ex9_bracket_span_rank -->
<!-- BEGIN ex9_psi_prime_e4 -->
$\\begin{pmatrix}0&-1\\\\1&0\\end{pmatrix}$
<!-- END ex9_psi_prime_e4 -->
<!-- BEGIN ex9_q_eigenvalues -->
$-0.5, -0.5, 0.5, 0.5$
<!-- END ex9_q_eigenvalues -->
<!-- BEGIN ex_table_charges -->
| spinor charge $\\mathbf b\\,\\mathbf e_4$ | $M_{\\rm LL}(\\mathbf b)$ | $\\psi'_{\\rm LL}$ | up column | down column |
|:---|:---|:---|:---|:---|
| $\\mathbf e_{4}$ | $\\begin{pmatrix}1&0\\\\0&1\\end{pmatrix}$ | $\\begin{pmatrix}0&-1\\\\1&0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\1\\end{pmatrix}$ | $\\begin{pmatrix}-1\\\\0\\end{pmatrix}$ |
| $\\mathbf e_{5}$ | $\\begin{pmatrix}0&-i\\\\-i&0\\end{pmatrix}$ | $\\begin{pmatrix}i&0\\\\0&-i\\end{pmatrix}$ | $\\begin{pmatrix}i\\\\0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\-i\\end{pmatrix}$ |
| $\\mathbf e_{6}$ | $\\begin{pmatrix}0&-1\\\\1&0\\end{pmatrix}$ | $\\begin{pmatrix}1&0\\\\0&1\\end{pmatrix}$ | $\\begin{pmatrix}1\\\\0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\1\\end{pmatrix}$ |
| $\\mathbf e_{7}$ | $\\begin{pmatrix}-i&0\\\\0&i\\end{pmatrix}$ | $\\begin{pmatrix}0&-i\\\\-i&0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\-i\\end{pmatrix}$ | $\\begin{pmatrix}-i\\\\0\\end{pmatrix}$ |
| $i\\mathbf e_{4}$ | $\\begin{pmatrix}i&0\\\\0&i\\end{pmatrix}$ | $\\begin{pmatrix}0&-i\\\\i&0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\i\\end{pmatrix}$ | $\\begin{pmatrix}-i\\\\0\\end{pmatrix}$ |
| $i\\mathbf e_{5}$ | $\\begin{pmatrix}0&1\\\\1&0\\end{pmatrix}$ | $\\begin{pmatrix}-1&0\\\\0&1\\end{pmatrix}$ | $\\begin{pmatrix}-1\\\\0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\1\\end{pmatrix}$ |
| $i\\mathbf e_{6}$ | $\\begin{pmatrix}0&-i\\\\i&0\\end{pmatrix}$ | $\\begin{pmatrix}i&0\\\\0&i\\end{pmatrix}$ | $\\begin{pmatrix}i\\\\0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\i\\end{pmatrix}$ |
| $i\\mathbf e_{7}$ | $\\begin{pmatrix}1&0\\\\0&-1\\end{pmatrix}$ | $\\begin{pmatrix}0&1\\\\1&0\\end{pmatrix}$ | $\\begin{pmatrix}0\\\\1\\end{pmatrix}$ | $\\begin{pmatrix}1\\\\0\\end{pmatrix}$ |
<!-- END ex_table_charges -->
<!-- BEGIN ex_verdict_python -->
ALL PASS (9 of 9)
<!-- END ex_verdict_python -->
<!-- BEGIN ex_verdict_rust -->
ALL PASS (27 of 27)
<!-- END ex_verdict_rust -->
<!-- BEGIN ex_verdict_wolfram -->
ALL PASS (6 of 6)
<!-- END ex_verdict_wolfram -->
<!-- BEGIN lgx_sources -->
```
6fa50ef3d438c1ea00b3112233251bab062d0ccd3d35d0ab163e6ba968a99226  Revision_SM_Higgs/provenance/generated/sh_examples.md
bbd0d3193d1e10c43c433216196ae80e18e166013256c0857e7f2fa9923535cf  Revision_SM_Higgs/provenance/generated/part2.md
6a713150001dc13b57374c8555855122d3191b4b4b75f35f83e293b50bf2ea86  Revision_SM_Higgs/data/sh_examples.csv
69fa48b8d706d087fd8a4d1f794885f97e8ee627b1c41ad8a8ab4e82072c9ea4  Revision_SM_Higgs/data/sh_examples_charges.csv
67ca395e508bdbea0af515666f6a6a5ea74871ff7357bfbec97a88f8e1450d47  Revision_SM_Higgs/data/part2_states.csv
23076ace01f0f57bae6acb1599e05574fbb54789fdbea4db2522252c0e6ec4d0  Revision_SM_Higgs/data/part2_effective_couplings.csv
31c590ef811ce049767659f43148914031cc6f89505100e33322a0bd3224494b  Revision_SM_Higgs/data/part2_modes.csv
aaed88f55d3ef88d54cb1d2f4149a22fd9c242c8eab4d28462b78247d135d2d4  Revision_SM_Higgs/data/part2_chi.csv
a9924278268950ee150f24343e5dbf03f2c56890230fbb17874926cc5a30ed7d  Revision_SM_Higgs/data/part2_thouless.csv
533c1840af82f7056dc4b08c891c4c7f8123b39f54747df846ea7a15c37ad170  Revision_SM_Higgs/data/part2_gauge_cjt.csv
0af252aca8349d4fcad4a29a50c56d9fab1dbbb46f586bf5ae8d899a070db855  Revision_SM_Higgs/data/hck_part2_checks.txt
762edbe64fb56153be102cbe89af467630ccf876686f96a65042dac09d48b7ac  Revision_SM_Higgs/data/hck_part2_checks_python.txt
```
<!-- END lgx_sources -->
<!-- BEGIN p2_D_V2_CJT -->
$0.3284861084$
<!-- END p2_D_V2_CJT -->
<!-- BEGIN p2_D_V2_HS_1_1 -->
$0.4030641406$
<!-- END p2_D_V2_HS_1_1 -->
<!-- BEGIN p2_E_V2_CJT -->
$-0.0002422384966$
<!-- END p2_E_V2_CJT -->
<!-- BEGIN p2_E_V2_HS_1_1 -->
$-0.0005186344202$
<!-- END p2_E_V2_HS_1_1 -->
<!-- BEGIN p2_Gcrit -->
$21.28660976$
<!-- END p2_Gcrit -->
<!-- BEGIN p2_M_V2_CJT -->
$0.1622929325$
<!-- END p2_M_V2_CJT -->
<!-- BEGIN p2_M_V2_HS_1_1 -->
$0.2066851932$
<!-- END p2_M_V2_HS_1_1 -->
<!-- BEGIN p2_V2_G -->
$25.2$
<!-- END p2_V2_G -->
<!-- BEGIN p2_V2_GS -->
$2.8$
<!-- END p2_V2_GS -->
<!-- BEGIN p2_chi11_CJT_exchange -->
$0.003777541783$
<!-- END p2_chi11_CJT_exchange -->
<!-- BEGIN p2_chiFF_CJT_exchange -->
$0.01635108248$
<!-- END p2_chiFF_CJT_exchange -->
<!-- BEGIN p2_chiS -->
$0.0831601473$
<!-- END p2_chiS -->
<!-- BEGIN p2_cjtex_D -->
$0.2761755673$
<!-- END p2_cjtex_D -->
<!-- BEGIN p2_cjtex_Dirac_mass -->
$0.1569681212$
<!-- END p2_cjtex_Dirac_mass -->
<!-- BEGIN p2_cjtex_M_A -->
$1.999946729\\times10^{-8}$
<!-- END p2_cjtex_M_A -->
<!-- BEGIN p2_cjtex_M_W -->
$0.04302319693$
<!-- END p2_cjtex_M_W -->
<!-- BEGIN p2_cjtex_M_Z -->
$0.05754210808$
<!-- END p2_cjtex_M_Z -->
<!-- BEGIN p2_cjtex_energy -->
$-0.0001324020178$
<!-- END p2_cjtex_energy -->
<!-- BEGIN p2_cjtex_exchange_energy -->
$9.646786667\\times10^{-5}$
<!-- END p2_cjtex_exchange_energy -->
<!-- BEGIN p2_cjtex_exchange_energy_B -->
$6.455007785\\times10^{-5}$
<!-- END p2_cjtex_exchange_energy_B -->
<!-- BEGIN p2_cjtex_exchange_energy_W1 -->
$4.013221351\\times10^{-5}$
<!-- END p2_cjtex_exchange_energy_W1 -->
<!-- BEGIN p2_cjtex_exchange_energy_W3 -->
$-4.834663821\\times10^{-5}$
<!-- END p2_cjtex_exchange_energy_W3 -->
<!-- BEGIN p2_cjtex_gap -->
$0.3176663895$
<!-- END p2_cjtex_gap -->
<!-- BEGIN p2_gP_V0_CJT -->
$1.2$
<!-- END p2_gP_V0_CJT -->
<!-- BEGIN p2_gS_V0_CJT -->
$-7.4$
<!-- END p2_gS_V0_CJT -->
<!-- BEGIN p2_gap_V2_CJT -->
$0.3663906649$
<!-- END p2_gap_V2_CJT -->
<!-- BEGIN p2_gap_V2_HS_1_1 -->
$0.4529674056$
<!-- END p2_gap_V2_HS_1_1 -->
<!-- BEGIN p2_lamP_V0_CJT -->
$0.05637346735$
<!-- END p2_lamP_V0_CJT -->
<!-- BEGIN p2_lamP_V2_CJT -->
$1.11337598$
<!-- END p2_lamP_V2_CJT -->
<!-- BEGIN p2_lamS_V0_CJT -->
$0.61538509$
<!-- END p2_lamS_V0_CJT -->
<!-- BEGIN p2_lamS_V2_CJT -->
$0.61538509$
<!-- END p2_lamS_V2_CJT -->
<!-- BEGIN p2_mode1_CJT -->
$0.6034759669$
<!-- END p2_mode1_CJT -->
<!-- BEGIN p2_mode2_CJT -->
$0.6440071546$
<!-- END p2_mode2_CJT -->
<!-- BEGIN p2_t_Gcrit_M0_V1 -->
$35.39$
<!-- END p2_t_Gcrit_M0_V1 -->
<!-- BEGIN p2_t_M0_V1_CJT -->
$-0.9308$
<!-- END p2_t_M0_V1_CJT -->
<!-- BEGIN p2_t_gS_V1_CJT -->
$-15.84$
<!-- END p2_t_gS_V1_CJT -->
<!-- BEGIN p2_t_lamP_V2 -->
$1.113$ to $1.184$
<!-- END p2_t_lamP_V2 -->
<!-- BEGIN p2_t_lamS_V1_CJT -->
$1.317$
<!-- END p2_t_lamS_V1_CJT -->
<!-- BEGIN p2_thouless3 -->
$0.03573178316$
<!-- END p2_thouless3 -->
<!-- BEGIN p2_thouless6 -->
$-0.04645131811$
<!-- END p2_thouless6 -->
<!-- BEGIN p2_thouless7 -->
$5.290206869$
<!-- END p2_thouless7 -->
<!-- BEGIN p2_thr_CJT -->
$0.7327813298$
<!-- END p2_thr_CJT -->
<!-- BEGIN p2_verdict_python -->
VERDICT: ALL PASS (23 of 23)
<!-- END p2_verdict_python -->
<!-- BEGIN p2_verdict_rust -->
VERDICT: ALL PASS (44 of 44)
<!-- END p2_verdict_rust -->
";

(* ::Input:: *)
(* ------------------------------------------------------------------ WW-5 *)
i1closed[mx_] := (Sqrt[1 + mx^2] - mx^2 Log[(1 + Sqrt[1 + mx^2])/mx])/2;
i1int = Integrate[q^2/Sqrt[q^2 + mx^2], {q, 0, 1}, Assumptions -> mx > 0];
(* Integrate writes the result with ArcCoth[Sqrt[1 + M^2]].  The closed form of (A.19) has Log[(1 + Sqrt[1 + M^2])/M]
   instead; the two agree because the hyperbolic cotangent of that logarithm is Sqrt[1 + M^2] (shown symbolically below,
   after TrigToExp) and the logarithm is positive, ArcCoth being the inverse of Coth on the positive numbers.  (A first
   version compared the two forms with Simplify, which does not reduce ArcCoth to the logarithm: request 346.)  The two
   forms are also compared as numbers of 40 digits at two masses, each side evaluated separately, so that N never
   evaluates an exact zero (an earlier version did, with a precision warning: request 345). *)
okSym = Simplify[i1int - (Sqrt[1 + mx^2] - mx^2 ArcCoth[Sqrt[1 + mx^2]])/2, mx > 0] === 0 &&
   Simplify[TrigToExp[Coth[Log[(1 + Sqrt[1 + mx^2])/mx]]] - Sqrt[1 + mx^2], mx > 0] === 0 &&
   Simplify[(1 + Sqrt[1 + mx^2])/mx > 1, mx > 0];
okNum = AllTrue[{3/10, 1/5}, Abs[N[i1int /. mx -> #, 40] - N[i1closed[#], 40]] < 10^-35 &];
okInt = okSym && okNum;
(* in the notebook: the text of provenance/generated/lg_examples.md, from the closed data cell of this chapter *)
frag = lgExamplesText;
fragNum[key_] := ToExpression[StringTrim[StringReplace[First[StringCases[frag,
       "<!-- BEGIN " <> key <> " -->\n" ~~ v : Shortest[__] ~~ "\n<!-- END " <> key <> " -->" :> v]], "$" -> ""]]];
gapPr = fragNum["p2_gap_V2_CJT"]; gcritPr = fragNum["p2_Gcrit"];
lhs = N[237/10 i1closed[SetPrecision[gapPr, 30]]/Pi^2, 25];
gcr = N[Pi^2/i1closed[1/5], 25];
ok5 = okInt && Abs[lhs - 1] < 10^-8 && Abs[gcr - gcritPr]/gcr < 10^-9;
check["WW-5", "(LW-7) the integral I^(1)(M) of (A.19), computed by Integrate for M > 0, equals its closed form \
symbolically and as numbers of 40 digits at M = 3/10 and 1/5; with the printed numbers the gap equation 23.7 \
I^(1)(E_gap)/Pi^2 = 1 and Pi^2/I^(1)(1/5) = G^crit_LL hold to the ten printed digits", ok5,
  "Integrate: " <> ToString[InputForm[i1int]] <> "; symbolic equality: " <> ToString[okSym] <> ", numerical equality: " <>
   ToString[okNum] <> ", 23.7 I1(E_gap)/Pi^2 = " <> ToString[NumberForm[lhs, 15]] <>
   ", Pi^2/I1(1/5) = " <> ToString[NumberForm[gcr, 15]]];

(* ::Input:: *)
(* the end of the chapter: the context of the symbols back to Global`, and the assumptions of Simplify back to their default *)
$Assumptions = True;
End[];

(* ::Chapter:: *)
(*10. The remarks of the manuscript*)

(* ::Text:: *)
(*The manuscript qualifies several of its results: it says where a result is an approximation, a model or restricted in some way, and its remarks document explains each such remark (what it means, why it holds, why it matters). Here they are in brief, with the chapter of this notebook that bears on each.*)

(* ::Item:: *)
(*R1, the coupling to the scalar curvature: the square of the Dirac operator contains $-\tfrac14R$, not Lord's $\tfrac12$ (Chapter 2, WG-4).*)

(* ::Item:: *)
(*R2, a constant Dirac mass and pair amplitude: Theorem L2 assumes that the Dirac mass and the pair amplitude of the quasi-particle are the same everywhere; the algebra that the theorem rests on is checked in Chapter 2 (WG-6).*)

(* ::Item:: *)
(*R3, the energy of the condensed vacuum: the condensation lowers the energy of the vacuum, and gravitation responds to that as to a cosmological constant (computed by the Rust and the Python programs; not in this notebook).*)

(* ::Item:: *)
(*R4, operators, normal order and semiclassical gravitation: the interaction and the cutoff of the brochon are defined in flat space-time and have no unique form in a curved one; the source of Einstein's equations is the expectation value of the energy-momentum tensor of Chapter 2 (WG-5), which is called semiclassical gravitation.*)

(* ::Item:: *)
(*R5, the massive gauge fields: in the superfluid $W^1$, $W^2$ and $B$ are massive, and their energy-momentum tensors add to the source of gravitation; the manuscript needs only the tensor of the zeugon, which stays massless (Chapter 2, WG-6).*)

(* ::Item:: *)
(*R6, the small-mass formulas of the gravitational atom: the formulas of Dolan and Dempsey hold for small coupling only; the exact states are those of Chapter 3 (WS-7).*)

(* ::Item:: *)
(*R7, the shrinking black hole is a model: a black hole that shrinks by evaporation is described by the ingoing Vaidya space-time with an inflow of negative energy (the model of W. A. Hiscock); the radiation that leaves is not modelled (Chapter 2, WG-3; Chapter 4).*)

(* ::Item:: *)
(*R8, projections are not probabilities: the quasi-bound states of a black hole are not orthogonal, so the projections of the field on them do not add up to one (computed by the Rust program; not in this notebook).*)

(* ::Item:: *)
(*R9, the polarization of the condensate is not computed: the potential of the pair is that of the empty vacuum; the superfluid could weaken it at long distances (Chapter 5 uses the unweakened potential).*)

(* ::Item:: *)
(*R10, the estimate of the charge by the creation of pairs: Schwinger's rate of pair creation is used as an estimate of the largest charge (Chapter 6 checks the geometry and the accretion limit).*)

(* ::Item:: *)
(*R11, the ringing in the limit of large angular number: the relation between the shadow and the ringing holds for large angular number only (Chapter 6, WC-3).*)

(* ::Item:: *)
(*R12, the quadratic tidal potential and the slow relative motion: the tidal potential is the first term of an expansion in the size of the pair divided by the radius (Chapter 7, WT-2).*)

(* ::Item:: *)
(*R13, the classical estimate of the break-up is false for a real fall: the barrier of Chapter 5 (WP-2) sinks much earlier than the pair actually separates.*)

(* ::Item:: *)
(*R14, the probability of separation late in the fall depends on a numerical device: below the remaining time 1, the probability that the two quasi-particles are more than 10 Bohr radii apart depends on how the tidal potential is flattened far out (computed by the Rust program; not in this notebook).*)

(* ::Item:: *)
(*R15, the uncertainties of the cosmological parameters enter the comparison of the vacuum energy with observation (not in this notebook).*)

(* ::Item:: *)
(*R16, the conservation of the number of quasi-particles holds in the mean-field approximation (Chapter 8, WA-3).*)

(* ::Chapter:: *)
(*The list of all checks*)

(* ::Text:: *)
(*The last cells list every check of this notebook with its result, and count the checks that passed.*)

(* ::Input:: *)
(* the table of all checks: identifier, result, and for the numerical checks the deviation and the tolerance *)
Grid[Prepend[Map[{#["id"], If[#["pass"], "PASS", "FAIL"], If[NumericQ[#["deviation"]], fmt[#["deviation"]], "--"],
      If[NumericQ[#["tolerance"]], fmt[#["tolerance"], 2], "--"]} &, LGNB`Test`$Results],
   Style[#, Bold] & /@ {"check", "result", "deviation", "tolerance"}],
  Frame -> All, Alignment -> Left, BaseStyle -> {FontSize -> 9}]

(* ::Input:: *)
(* the count of the checks that passed *)
Print["checks of this notebook: ", Length[LGNB`Test`$Results], "; passed: ", Count[LGNB`Test`$Results, lgR_ /; lgR["pass"]],
  "; failed: ", Select[LGNB`Test`$Results, ! #["pass"] &][[All, "id"]]]
