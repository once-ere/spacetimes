#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_schw.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23, stage 5)

   PURPOSE
   The exact part, and a computation in arithmetic of 32 significant digits, of the brochon (a Dirac
   particle of mass m) in the Schwarzschild field.  Units G_N = c = hbar = 1, the mass of the black
   hole 1 (the horizon at r = 2), f = 1 - 2/r, W = (2/r)^(1/2).  The identifier of a check is WS and
   its number; each entry names, in parentheses, the checks of the Rust program lg_schw and of the
   Python program python/lg_schw.py that test the same statements.

     WS-1  (Rust LS-1; Python YS-1) the separation in the static tetrad e^0 = f^(1/2) dt, e^1 =
           f^(-1/2) dr, e^2 = r dth, e^3 = r sin(th) dph: the spin connection is computed here from the
           tetrad (the tetrad postulate, as in lg_curved.wls), and the separated spinor with the angular
           equations (A) and the radial system (S) solves the Dirac equation exactly;
     WS-2  (Rust LS-2; Python YS-2) the same for the freely falling tetrad e^0 = dtau, e^1 = dr + W dtau
           with the system (PG), for every r > 0 (both sides of the horizon);
     WS-3  (Rust LS-3; Python YS-3) the relation of (S) and (PG): P = (1 - W)^(-1/2) a_R e^{i omega T},
           Q = (1 + W)^(-1/2) b_R e^{i omega T}, T = tau - t, T' = W/f;
     WS-4  (Rust LS-4, LS-4b, LS-6; Python YS-4) the residue matrices of (PG) at r = 2 and r = 0 and
           their eigenvalues: the exponents 0 and 4 i omega - 1/2 at the horizon, 1/4 (twice) at r = 0;
     WS-5  (Rust LS-7; Python YS-5) the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 is conserved for
           real omega, m, lambda;
     WS-6  (Rust: the closed forms of the angular functions for j = 1/2) for lambda = +-1 and m_j = +-1/2
           the functions A = sin(th/2)^(1/2) cos(th/2)^(3/2), B = lambda sin(th/2)^(3/2) cos(th/2)^(1/2)
           (m_j = 1/2) and A = sin(th/2)^(3/2) cos(th/2)^(1/2), B = -lambda sin(th/2)^(1/2)
           cos(th/2)^(3/2) (m_j = -1/2) solve (A), and A, B divided by sin(th)^(1/2) are bounded at both
           poles;
     WS-7  (Rust LS-8; Python YS-6) three quasi-bound states at Mm = 3/10 (1S1/2, 2P1/2, 2P3/2), computed
           with NDSolve in arithmetic of 32 digits from a Taylor start of order 20 at r = 2 + 1/20, and
           FindRoot, compared with data/lg_schw_qbs.csv;
     WS-8  (Rust LS-9; Python YS-7) three greybody factors at r = 4000, computed the same way, compared
           with data/lg_schw_grey.csv.

   THE SYSTEMS (as in the Rust program; m_j, lambda the angular quantum numbers, lambda = +-(j + 1/2))
     (A)   A' = (m_j / sin th) A - lambda B,   B' = -(m_j / sin th) B + lambda A
     (S)   f u' = -m f^(1/2) u + (omega + lambda f^(1/2)/r) w,   f w' = -(omega - lambda f^(1/2)/r) u + m f^(1/2) w
     (PG)  P' = [(i omega - W/(4r)) P - (i lambda/r + m) Q]/(1 - W),
           Q' = [(i lambda/r - m) P + (-i omega + W/(4r)) Q]/(1 + W)
     spinor: pref e^{-i omega T + i m_j ph} S_H (-i b_R A, i a_R B, a_R A, b_R B), S_H = diag(H, H),
             H = i (sigma_1 + sigma_3)/2^(1/2); static: T = t, pref = f^(-1/4) r^(-1) sin(th)^(-1/2),
             a_R = (u - i w)/2, b_R = (u + i w)/2; freely falling: T = tau, pref = r^(-1) sin(th)^(-1/2),
             a_R = P, b_R = Q.

   NOTE ON NAMES
   The coordinate symbols (t, tau, r, th, ph) and the parameters (mB, om, lam, mj) are never used as
   iterators.  The mass of the brochon is mB.

   USAGE
     wolframscript -file lg_schw.wls <lg_schw_qbs.csv> <lg_schw_grey.csv> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: lg_schw.wls <lg_schw_qbs.csv> <lg_schw_grey.csv> <report.txt>"]; Exit[2]];
{qbsFile, greyFile, reportFile} = args[[1 ;; 3]];
t0 = AbsoluteTime[];
lines = {}; notes = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
(* fmt: a real number in the form d.ddde-x with nd significant digits (no 2D output) *)
fmt[x_, nd_ : 4] := Module[{e, mm, y = N[x]},
  If[y == 0, "0", e = Floor[Log10[Abs[y]]]; mm = Round[Abs[y] 10^(nd - 1 - e)];
   If[mm >= 10^nd, mm = Round[mm/10]; e = e + 1];
   StringJoin[If[y < 0, "-", ""], ToString[Quotient[mm, 10^(nd - 1)]], ".",
    IntegerString[Mod[mm, 10^(nd - 1)], 10, nd - 1], "e", ToString[e]]]];
checkDev[id_String, text_String, dev_, tol_] := Module[{line, ok = TrueQ[dev <= tol]},
  ntot++; If[ok, npass++];
  line = StringJoin[If[ok, "PASS ", "FAIL "], StringPadRight[id, 9], "deviation ", fmt[dev], " (tolerance ",
    fmt[tol, 2], "): ", text];
  Print[line]; AppendTo[lines, line]];
zeroQ[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[#] == 0] &];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
readTable[file_] := Module[{tb = Import[file, "CSV", "Numeric" -> False] /. _Missing -> "NaN"}, {First[tb], Rest[tb]}];

(* ------------------------------------------------------------------ the matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
gam = Table[ArrayFlatten[{{0, sig[[a]]}, {sigbar[[a]], 0}}], {a, 4}];   (* gamma^a of the chiral basis, (1.2) *)
eta = DiagonalMatrix[{1, -1, -1, -1}];
gab[a_, b_] := (gam[[a]] . gam[[b]] - gam[[b]] . gam[[a]])/2;
hH = I (sig[[2]] + sig[[4]])/Sqrt[2];
sH = ArrayFlatten[{{hH, 0}, {0, hH}}];

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

(* ------------------------------------------------------------------ WS-5: the radial current *)
radPb = {Derivative[1][pB][r] -> ((-I om - wPG/(4 r)) pB[r] - (-I lam/r + mB) qB[r])/(1 - wPG),
   Derivative[1][qB][r] -> ((-I lam/r - mB) pB[r] + (I om + wPG/(4 r)) qB[r])/(1 + wPG)};
jJ = (1 + wPG) qQ[r] qB[r] - (1 - wPG) pP[r] pB[r];
dJ = Simplify[D[jJ, r] /. radP /. radPb, r > 0];
check["WS-5", "(Rust LS-7; Python YS-5) the radial current J = (1 + W)|Q|^2 - (1 - W)|P|^2 of a real frequency is \
constant: dJ/dr = 0 exactly for every r > 0 (the conjugates of P, Q are independent functions that solve the conjugate \
equations; omega, m, lambda real)", zeroQ[dJ], "dJ/dr = " <> ToString[InputForm[dJ]]];

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

(* ------------------------------------------------------------------ WS-7: quasi-bound states *)
{qHead, qRows} = readTable[qbsFile];
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

(* ------------------------------------------------------------------ WS-8: greybody factors at r = 4000 *)
{gHead, gRows} = readTable[greyFile];
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

out = StringJoin[
  "lg_schw.wls: the exact checks and a computation in arithmetic of 32 digits of stage 5 of task C23, the brochon in the \
Schwarzschild field (Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WS and a number), the statement and, in brackets, the\n",
  "facts that decide it (or the measured deviation and its tolerance).  WS-1 to WS-6 are exact (symbolic).\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  StringJoin[("  " <> # <> "\n") & /@ notes],
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
