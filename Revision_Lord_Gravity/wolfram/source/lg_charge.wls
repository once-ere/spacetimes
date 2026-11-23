#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_charge.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23, stage 7b)

   PURPOSE
   The exact checks, in the Wolfram Language, of a black hole that carries zeugon charge: the geometry
   of Reissner and Nordstrom with the normalization of the zeugon field of the manuscript, the limit of
   the charge by selective accretion, and the shadow.  The identifier of a check is WC and its number;
   the counterparts in the Rust program lg_charge (LC) and the Python program lg_charge.py (YC) in
   brackets.

     WC-1  (Python YC-1) with the zeugon field A = (Q/(4 pi r)) dt, the metric
           diag(f, -1/f, -r^2, -r^2 sin^2 th), f = 1 - 2 G M/r + G Q^2/(4 pi r^2), solves Einstein's
           equations G_{mu nu} = 8 pi G T_{mu nu} with the curvature of Landau and Lifshitz and the tensor
           T_{mu nu} = -F_{mu la} F_nu^la + (1/4) g_{mu nu} F^2 of the zeugon (manuscript (14.2), (14.3),
           (16.5), (16.8)); with G Q^2/r^2 in place of G Q^2/(4 pi r^2) it does not (the control);
     WC-2  (Rust LC-1; Python YC-2) the radial velocity squared of a particle of charge-to-mass length x
           at rest far away, W = (1 - x/r)^2 - f (M = 1, f = 1 - 2/r + Qg^2/r^2), obeys
           W r^2 = 2 (1 - x) r + x^2 - Qg^2 and W(r_+) r_+^2 = (r_+ - x)^2; and Reduce decides, for six
           values of Qg^2 between 0 and 99/100, that W >= 0 for every r >= r_+ exactly when 0 < x <= 1;
     WC-3  (Rust LC-2, LC-3; Python YC-3) the photon sphere r^2 - 3 r + 2 Qg^2 = 0 and the shadow
           b_c = 3 3^(1/2) (1 - Qg^2/6 - 7 Qg^4/216 + ...), 4 for the extremal charge.

   USAGE
     wolframscript -file lg_charge.wls <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 1, Print["usage: lg_charge.wls <report.txt>"]; Exit[2]];
reportFile = args[[1]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];

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

out = StringJoin[
  "lg_charge.wls: the exact checks of stage 7b of task C23, a black hole with zeugon charge (Revision_Lord_Gravity, PROJECT \
spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WC and a number), the statement and, in brackets, the\n",
  "facts that decide it.  All checks are exact.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
