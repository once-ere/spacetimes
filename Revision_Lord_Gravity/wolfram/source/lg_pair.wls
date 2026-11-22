#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_pair.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23, stage 7a)

   PURPOSE
   The exact check, in the Wolfram Language, of the levels of the order alpha^4 of the pair of two
   brochon quasi-particles bound by the zeugon (two particles of spin 1/2, the same mass m, gyromagnetic
   factor 2, opposite charges, a massless vector exchange): the Breit-Fermi Hamiltonian in first-order
   perturbation theory in the hydrogen-like states |n l s j>, and the closed form of the tidal barrier.
   The identifier of a check is WP and its number; the counterparts in brackets.

     WP-1  (Python YP-2; Rust LP-1) the coefficients of m alpha^4 of every state with n <= 3, without and
           with the term of virtual annihilation, computed here with the hydrogen radial functions of
           the Wolfram Language (Hypergeometric form of LaguerreL) and exact integration, equal the exact
           rational numbers of data/lg_pair_levels.csv (written by python/lg_pair.py);
     WP-2  (Rust LP-2) the top of the barrier of V(z) = -alpha/z - k z^2 is -(3/2) alpha^(2/3) (2 k)^(1/3),
           and it equals the level -mu alpha^2/(2 n^2) at r^3 = 54 n^6 r_g/(mu^2 alpha^4) when k = mu r_g/r^3.

   THE HAMILTONIAN (units mu = alpha = 1, so that the Bohr radius is 1 and each mass is m = 2; the results
   are divided by m alpha^4 = 2): -p^4/(8)(2/m^3) + (pi/2)(2/m^2) delta(r) - (1/(2 m^2)) p_i[(delta_ij +
   n_i n_j)/r]p_j + (3/(2 m^2)) L.S/r^3 + (8 pi/(3 m^2)) S1.S2 delta(r) + (1/m^2)[3(S1.n)(S2.n) - S1.S2]/r^3
   [+ (pi/(2 m^2))(3 + sigma1.sigma2) delta(r)].

   USAGE
     wolframscript -file lg_pair.wls <lg_pair_levels.csv> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 2, Print["usage: lg_pair.wls <lg_pair_levels.csv> <report.txt>"]; Exit[2]];
{levelsFile, reportFile} = args[[1 ;; 2]];
t0 = AbsoluteTime[];
lines = {}; notes = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];

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
tab = Rest[Import[levelsFile, "CSV", "Numeric" -> False]];
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

out = StringJoin[
  "lg_pair.wls: the exact checks of stage 7a of task C23, the pair of two brochon quasi-particles bound by the zeugon \
(Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WP and a number), the statement and, in brackets, the\n",
  "facts that decide it.  All checks are exact.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
