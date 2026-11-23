#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_tidal.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23, stage 7c)

   PURPOSE
   The exact checks, in the Wolfram Language, of the pair of two quasi-particles bound by the zeugon
   in the tidal field of a black hole along a radial fall.  Atomic units of the pair: lengths in its
   Bohr radius, energies in mu alpha^2; the relative motion is that of a hydrogen atom,
   H = -(1/2) Laplacian - 1/r - kappa r^2 P_2(cos theta).  The identifier of a check is WT and its
   number; the counterparts in the Rust program lg_tidal (LT) and the Python program lg_tidal.py (YT)
   in brackets.

     WT-1  (Rust LT-0; Python YT-1) the coupling <Y_l0|P_2|Y_l'0> = ((2l+1)(2l'+1))^(1/2) (l 2 l'; 0 0 0)^2
           (three-j symbols) equals the closed forms l(l+1)/((2l-1)(2l+3)) (l' = l) and
           (3/2)(l+1)(l+2)/((2l+3)((2l+1)(2l+5))^(1/2)) (l' = l + 2) for l = 0, 2, ..., 30;
     WT-2  (Python YT-2) the observers of the Painleve-Gullstrand coordinates,
           ds^2 = dt^2 - (dr + (2M/r)^(1/2) dt)^2 - r^2 dOmega^2, fall freely from rest far away with the
           proper time t; the geodesic deviation in their frame is diag(2M/r^3, -M/r^3, -M/r^3); along
           their path r^3 = (9M/2)(tau0 - tau)^2; in the atomic units of the pair the tidal strength is
           kappa = 2/(9 T^2) with T = (tau0 - tau) mu alpha^2, and r = 2M at T_h = (4/3) M mu alpha^2;
     WT-3  (Python YT-3) perturbation theory of Dalgarno and Lewis: f = kappa (r^3/3 + r^2/2) solves
           -(1/2)(f'' + 2 f'/r - 6 f/r^2 - 2 f') = kappa r^2, and the energy of the ground state in the static
           tidal field is E = -1/2 - (15/2) kappa^2 - (153/2) kappa^3 + O(kappa^4);
     WT-4  (Python YT-5a, YT-5b) the pieces of the weak-field rate of tunnelling: for a uniform field F the
           method gives Landau's (4/F) exp(-2/(3F)); for the tidal field it gives
           8 kappa^(-1/2) exp(-pi/(2 (2 kappa)^(1/2)))/sinh(pi/2^(1/2));
     WT-5  (Rust LT-2; Python YT-6) the classical solutions in the field kappa = 2/(9 T^2): DSolve gives
           T^(4/3), T^(-1/3) for z'' = 2 kappa z and T^(2/3), T^(1/3) for x'' = -kappa x, and the solutions
           with the initial values (1, 0) and (0, -1) at T0 (d/dT = -d/dt) are the closed forms of lg_tidal.

   USAGE
     wolframscript -file lg_tidal.wls <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 1, Print["usage: lg_tidal.wls <report.txt>"]; Exit[2]];
reportFile = args[[1]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];

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

out = StringJoin[
  "lg_tidal.wls: the exact checks of stage 7c of task C23, the pair bound by the zeugon in the tidal field of a black hole \
(Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WT and a number), the statement and, in brackets, the\n",
  "facts that decide it.  All checks are exact.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
