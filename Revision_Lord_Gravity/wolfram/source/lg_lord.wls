#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_lord.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23)

   PURPOSE
   The exact part of the reading of E. A. Lord, "General relativity from gauge invariance" (1971),
   Part I of lord_gravity.md: the identities of his Section 1 and the factors and signs of his
   Section 4, decided in exact arithmetic (symbolic matrices, no floating point).  The identifier of a
   check is WL and its number; each entry names, in parentheses, the checks of the Rust program
   lg_lord and of the Python program python/lg_lord.py that test the same statements in floating
   point.

     WL-1  (Rust L1-1 to L1-4, L1-7 to L1-10; Python YL-1) the identities (1.2), (1.3), (1.5), (1.7),
           (1.10) to (1.15) and (1.17) of Lord's matrices, exactly;
     WL-2  (Rust L1-11a, L1-11b; Python YL-3) for a 2x2 matrix S of determinant 1 with symbolic entries:
           epsilon S = (S^T)^{-1} epsilon, and the sigma-bar of S sigma S' is S'^{-1} sigma-bar S^{-1} for
           any second matrix S' of determinant 1 (with S' = S^dag this is (1.23));
     WL-3  (Rust L1-13, L1-14; Python YL-3b) for any two 2x2 matrices A, B (symbolic entries):
           1/2 (A bar(B) + B bar(A)) = N(A, B) 1, N the polarized determinant; hence (1.3) holds for a
           set exactly when N(sigma^a, sigma^b) = eta^{ab};
     WL-4  (Rust L1-5; Python YL-2) (1.8) to first order: with Lambda_mu^nu = delta_mu^nu +
           eta^{nu rho} lambda_{rho mu} and L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu} (six real symbols
           lambda), eta^{mu rho} lambda_{rho nu} sigma^nu + X sigma^mu + sigma^mu X^dag = 0;
     WL-5  (Rust L4-2, L4-3; Python YL-4, YL-4b) the sign of the inhomogeneous term of (4.14), (4.31):
           for a field S(t) of determinant 1, a constant connection in the Lorentz algebra (symbolic) and
           a spinor of four symbolic functions, D = d - Gamma is covariant with Gamma' = S Gamma S^{-1} +
           (dS) S^{-1}, and not with the printed S (d S^{-1});
     WL-6  (Rust L4-6b, L4-6c, L4-10a; Python YL-6, YL-6b, YL-7b) the connection of the vectors in
           Lord's special gauge (constant gamma, (5.2)): for Gamma = 1/2 g a_bc gamma^{bc} with six
           symbolic a_bc, [Gamma, gamma_b] = -2 g a_b^e gamma_e, so (4.19) gives C = 2 g a, twice the
           printed (4.20), (5.2);
     WL-7  (Rust L4-10b, L4-10c; Python YL-7b, YL-7c) the spinor image lambda(W) = 1/4 W_{ab} gamma^{ab}
           of the Lorentz algebra (W_{ab} antisymmetric, symbolic) satisfies [lambda(W), gamma_b] =
           -W_b^e gamma_e and [lambda(W1), lambda(W2)] = lambda([W1, W2]) (a homomorphism of Lie
           algebras), so the spinor curvature is 1/4 R_{ab} gamma^{ab}, half of the printed 1/2 R gamma.

   USAGE
     wolframscript -file lg_lord.wls <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 1, Print["usage: lg_lord.wls <report.txt>"]; Exit[2]];
reportFile = args[[1]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
zeroQ[m_] := AllTrue[Flatten[{Simplify[m]}], TrueQ[# == 0] &];

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

(* ------------------------------------------------------------------ WL-2 *)
sS = {{a1, b1}, {c1, (1 + b1 c1)/a1}};            (* det = 1 *)
sP = {{a2, b2}, {c2, (1 + b2 c2)/a2}};            (* a second matrix of determinant 1 *)
sigGen = {{s11, s12}, {s21, s22}};                (* any 2x2 matrix *)
okEps = zeroQ[eps . sS - Inverse[Transpose[sS]] . eps];
ok123 = zeroQ[bar[sS . sigGen . sP] - Inverse[sP] . bar[sigGen] . Inverse[sS]];
check["WL-2", "(Rust L1-11a, L1-11b; Python YL-3) for S of determinant 1 (symbolic entries): epsilon S = (S^T)^{-1} \
epsilon; and for any matrix sigma and a second S' of determinant 1, bar(S sigma S') = S'^{-1} bar(sigma) S^{-1}; with \
S' = S^dag this is (1.23)", okEps && ok123, "epsilon " <> ToString[okEps] <> ", (1.23) " <> ToString[ok123]];

(* ------------------------------------------------------------------ WL-3 *)
mA = {{p11, p12}, {p21, p22}}; mB = {{q11, q12}, {q21, q22}};
nAB = (Det[mA + mB] - Det[mA] - Det[mB])/2;
ok3 = zeroQ[(mA . bar[mB] + mB . bar[mA])/2 - nAB IdentityMatrix[2]];
okN = And @@ Flatten@Table[(Det[sig[[m]] + sig[[n]]] - Det[sig[[m]]] - Det[sig[[n]]])/2 == eta[[m, n]], {m, 4}, {n, 4}];
check["WL-3", "(Rust L1-13, L1-14; Python YL-3, YL-3b) for any two 2x2 matrices A, B (symbolic entries): 1/2 (A bar(B) + \
B bar(A)) = N(A, B) 1 with N(A, B) = (det(A + B) - det A - det B)/2, and N(sigma^mu, sigma^nu) = eta^{mu nu}; so (1.3) \
holds for a set of four matrices exactly when their products N are eta^{ab}", ok3 && okN,
  "identity " <> ToString[ok3] <> ", N of the standard set " <> ToString[okN]];

(* ------------------------------------------------------------------ WL-4 *)
lam = {{0, l01, l02, l03}, {-l01, 0, l12, l13}, {-l02, -l12, 0, l23}, {-l03, -l13, -l23, 0}};  (* lambda_{mu nu}, real *)
xx = Sum[lam[[a, b]]/4 smn[sig, a, b], {a, 4}, {b, 4}];
xxd = ConjugateTranspose[xx] /. Conjugate[z_] :> z;                   (* the lambda are real *)
ok4 = And @@ Table[zeroQ[xx . sig[[m]] + sig[[m]] . xxd + Sum[eta[[m, m]] lam[[m, n]] sig[[n]], {n, 4}]], {m, 4}];
check["WL-4", "(Rust L1-5; Python YL-2) (1.8) to first order: with Lambda_mu^nu = delta_mu^nu + eta^{nu rho} \
lambda_{rho mu} and L = 1 + 1/4 lambda_{mu nu} sigma^{mu nu} (six real symbols), eta^{mu rho} lambda_{rho nu} sigma^nu + \
X sigma^mu + sigma^mu X^dag = 0 for every mu", ok4, ToString[ok4]];

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

out = StringJoin[
  "lg_lord.wls: the exact identities and factors of Lord (1971), Sections 1 and 4 (Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WL and a number), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic: symbolic matrices, every identity simplified to zero.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
