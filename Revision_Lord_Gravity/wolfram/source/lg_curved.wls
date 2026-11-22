#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_curved.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23)

   PURPOSE
   The exact part of Part III of lord_gravity.md, the gravitational coupling of the brochon by Lord's
   covariant derivative: the geometry of the tetrad (Lord's reference vectors), the spin connection,
   the curvature, the Dirac operator of the brochon and its energy-momentum tensor, decided in exact
   arithmetic for explicit space-times and for symbolic fields.  The identifier of a check is WG and its
   number; each entry names, in parentheses, the checks of the Rust program lg_curved and of the Python
   program python/lg_curved.py that test the same statements in floating point.

     WG-1  (Rust G-4 to G-9; Python YG-2, YG-3) the Schwarzschild space-time with the diagonal tetrad:
           Ricci tensor zero, Kretschmann invariant 48 M^2/r^6, the spinor connection Omega_mu in closed
           form, the Dirac matrices covariantly constant, the spinor curvature 1/4 R_{ab mu nu} gamma^{ab};
     WG-2  (the same) the Painleve-Gullstrand coordinates with the tetrad of the freely falling observers:
           the metric is Schwarzschild's under T = t + int v/f dr, Ricci zero, Omega_mu in closed form,
           the frame, the Christoffel symbols and Omega_mu finite at the horizon r = 2M;
     WG-3  the Vaidya space-times (ingoing, advanced time v; outgoing, retarded time u): Einstein's tensor
           is 8 pi times the energy-momentum tensor of null dust, with the energy density M'(v)/(4 pi r^2)
           and -M'(u)/(4 pi r^2), and the scalar curvature vanishes;
     WG-4  (Rust G-10, G-15; Python YG-4, YG-7) in g = Omega^2 eta with Omega = exp(phi(x)) symbolic:
           Dslash (Omega^{-3/2} psi) = Omega^{-5/2} dslash psi (conformal invariance of the massless
           brochon) and Dslash Dslash psi = Box psi - R/4 psi (Lichnerowicz), for a spinor of four symbolic
           functions;
     WG-5  (Rust G-13; Python YG-6) the energy-momentum tensor from the variation of the action with
           respect to the tetrad: at first order about flat space-time, with sixteen symbolic functions in
           the tetrad and a symbolic spinor, the symmetric part of (1/e) dS/de_a^mu e_{a nu} is exactly
           T_{mu nu} of the manuscript (off shell); its antisymmetric part vanishes on a solution (two plane
           waves);
     WG-6  (Rust G-17; Python YG-8) the pairing term of the polar superfluid, P = (gamma^0 (x) 1)(K^3)^dag:
           P conj(gamma^a) = gamma^a P, P conj(P) = -1, P conj(Omega) = Omega P for every element Omega
           of the Lorentz algebra (symbolic), and P conj(i t) = i t P exactly for the generator t = T^3 of
           the zeugon, not for T^1, T^2 and the fermion number.

   NOTE ON NAMES
   The coordinate symbols (t, r, th, ph, tT, vv, uu, x, y, z) are never used as iterators of Table or
   Sum: an iterator of the same name would replace the coordinate inside the iteration (the iterator of
   the first index of the Christoffel and Riemann tensors is therefore rr).

   USAGE
     wolframscript -file lg_curved.wls <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 1, Print["usage: lg_curved.wls <report.txt>"]; Exit[2]];
reportFile = args[[1]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
zeroQ[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[#] == 0] &];
finiteQ[m_] := FreeQ[Flatten[{m}], ComplexInfinity | Indeterminate | DirectedInfinity[_] | Infinity];
(* zeroE: the expanded form is 0 (for polynomials in independent functions and their derivatives, with exponential
   factors, the expanded form is canonical, so this decides the identity exactly and quickly); witnessQ: an expression
   is not identically zero, shown by its value for explicit functions at a rational point *)
zeroE[m_] := AllTrue[Flatten[{m}], Expand[#] === 0 &];
witnessQ[m_, rules_, pt_] := AnyTrue[Flatten[{m}], Abs[N[(# /. rules) /. pt, 30]] > 10^-20 &];

(* ------------------------------------------------------------------ the matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
gam = Table[ArrayFlatten[{{0, sig[[a]]}, {sigbar[[a]], 0}}], {a, 4}];   (* gamma^a of the chiral basis *)
eta = DiagonalMatrix[{1, -1, -1, -1}];
gab[a_, b_] := (gam[[a]] . gam[[b]] - gam[[b]] . gam[[a]])/2;
id4 = IdentityMatrix[4];

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

out = StringJoin[
  "lg_curved.wls: the exact checks of Part III of lord_gravity.md, the gravitational coupling of the brochon \
(Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WG and a number), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic: symbolic metrics, tetrads and fields, every identity simplified to zero.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
