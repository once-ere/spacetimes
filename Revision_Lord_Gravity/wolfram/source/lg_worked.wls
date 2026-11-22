#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_worked.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23)

   PURPOSE
   The exact (symbolic) check, in the Wolfram Language, of the worked examples of the manuscript
   lord_gravity: the third computation besides the Rust program lg_worked (checks LW) and the Python
   program python/lg_worked.py (checks YW).  The identifier of a check is WW and its number; the
   counterparts are named in brackets.

     WW-1  (LW-1 to LW-3) Lord's change of representation by S = M_LL(gtilde), g = (5/4) e_0 + (3/4) i e_3:
           S = diag(1/2, 2) = (M_LL(g)^dag)^-1; sigma'_mu = S sigma_mu S^dag = diag(1/4, 4), sigma_1, sigma_2,
           diag(1/4, -4), Hermitian, with the bars (S^dag)^-1 bar(sigma_mu) S^-1 and the polarized determinants
           eta_ab; Lord's Dirac matrices of the new set equal S4 gamma^a S4^-1 with S4 = diag(S, (S^dag)^-1) =
           S_LL(g), obey the Clifford relation, and equal sum_b (L^-1)_ab gamma^b with the Lorentz matrix L of the
           boost; the frame f'_a = M_LL^-1(sigma'_a) is (17/8, 0, 0, -15/8), (0, 1, 0, 0), (0, 0, 1, 0),
           (-15/8, 0, 0, 17/8), with the velocity -15/17 of f'_0;
     WW-2  (LW-4) the set (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2): Hermitian, linearly independent
           (determinant 1), N_LG(sigma''_0, sigma''_3) = 1/2 and (1/2){gamma''^0, gamma''^3} = (1/2) 1_4;
     WW-3  (LW-5) the plane wave Psi = (u (x) (1, 0)) Exp[-i p.x] with u = (1/2, 0, 2, 0), p = (17/8, 0, 0, 15/8),
           m = 1, symbolic coordinates: the free field equation, the current 2 p^mu, the Lagrangian density 0, the
           energy-momentum tensor of (16.2) T_{mu nu} = 2 p_mu p_nu (lower indices) and its trace 2;
     WW-4  (LW-6) the Bogoliubov-de Gennes matrix of the polar ground state with SYMBOLIC real momentum q, Dirac
           mass M and pair amplitude D: its square is (|q|^2 + M^2 + D^2) 1_16 and its trace is 0, so its
           eigenvalues are -E(q) and +E(q) eight times each, E(q) = (|q|^2 + M^2 + D^2)^(1/2);
     WW-5  (LW-7) the integral I^(1)(M) = Integrate[q^2/Sqrt[q^2 + M^2], {q, 0, 1}] equals the closed form of
           (A.19) for M > 0 (symbolic), and with the printed numbers of the fragment file lg_examples.md the gap
           equation 23.7 I^(1)(E_gap)/Pi^2 = 1 and Pi^2/I^(1)(1/5) = G^crit_LL hold to the ten printed digits.

   USAGE
     wolframscript -file lg_worked.wls <provenance/generated/lg_examples.md> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 2, Print["usage: lg_worked.wls <lg_examples.md> <report.txt>"]; Exit[2]];
{fragFile, reportFile} = args[[1 ;; 2]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
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

(* ------------------------------------------------------------------ WW-2 *)
s2 = {sig[[1]], sig[[2]], sig[[3]], sig[[4]] + sig[[1]]/2};
coef = Table[Tr[s2[[k]] . sig[[r]]]/2, {k, 4}, {r, 4}];
g2 = dirac /@ s2;
ok2 = zeroQ[s2 - (herm /@ s2)] && FreeQ[coef, Complex] && Det[coef] === 1 && ndet[s2[[1]], s2[[4]]] === 1/2 &&
  zeroQ[(g2[[1]] . g2[[4]] + g2[[4]] . g2[[1]])/2 - IdentityMatrix[4]/2];
check["WW-2", "(LW-4) the set (sigma_0, sigma_1, sigma_2, sigma_3 + sigma_0/2): Hermitian, linearly independent (the \
determinant of the coefficients is 1), N_LG(sigma''_0, sigma''_3) = 1/2, (1/2){gamma''^0, gamma''^3} = (1/2) 1_4 (exact)", ok2,
  "det " <> ToString[Det[coef]] <> ", N03 " <> ToString[InputForm[ndet[s2[[1]], s2[[4]]]]]];

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
frag = Import[fragFile, "Text"];
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

out = StringJoin[
  "lg_worked.wls: the exact checks of the worked examples of the manuscript lord_gravity (Revision_Lord_Gravity, PROJECT \
spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WW and a number), the statement and, in brackets, the\n",
  "facts that decide it.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
