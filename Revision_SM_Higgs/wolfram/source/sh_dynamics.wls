#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_dynamics.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The exact part of stage 6 of the revision of the hypotheses, part one: the dynamics in time of
   the Higgs superfluid by the time-dependent mean field (the Rust program sh_dynamics).  The
   statements that the program relies on are derived here in exact arithmetic, and the numbers of
   the program are compared with them.  The identifiers are W, the letter D of this stage and the
   number of the check in this script; each entry names, in parentheses, the checks of the Rust
   program and of the Python program python/sh_dynamics.py that test the same statements in
   floating point.

     WD-1  (Rust D-3a, D-3b, D-3c; Python YD-2a, YD-2b) the time-dependent mean field
           i dR/dt = [H, R] conserves the energy when H is the derivative of the energy:
           d tr(H R)/dt = -i tr(H [H, R]) = 0 (true for matrices of any size by the cyclicity of
           the trace; checked here symbolically for 3 x 3 Hermitian matrices with symbolic
           entries), and the evolution keeps R^2 = R and R^dag = R (it is a unitary similarity);
     WD-2  (Rust D-6a, D-6b; Python YD-5) the Josephson relation: with the particle number F (the unit matrix on the sixteen
           components, diag(1, -1) on the Nambu space) and H - mu F, the state
           exp(i mu F t) R_0 exp(-i mu F t) solves the equation when R_0 is stationary, and its
           pair block carries the factor exp(2 i mu t) (symbolic); the rates that the Rust program
           measured from the phase of d(t) (data/sh_dyn_signals.csv, the Josephson runs, fitted
           here by a straight line in exact arithmetic on the numbers of the file) against 2 mu;
     WD-3  (Rust D-4a; Python YD-3) the amplitude mode of the pairing-only methods lies at d_0 (stage 4): d_0 from the
           gap equation (G_SH / 4 pi^2) I1((m^2 + d_0^2/4)^(1/2)) = 1 to 30 digits, against the
           peaks of the spectra of the Rust program (data/sh_dyn_runs.csv);
     WD-4  (Rust D-4b; no Python check) the Hann window h(t) = sin^2(pi t / T): the modulus of its Fourier transform, normalized
           to 1 at the centre, is |sin(x pi) / (x pi (1 - x^2))| at the offset x 2 pi / T (the exact
           integral against this form at nine offsets, in exact arithmetic), which is exactly 1/2 at x = 1; the half width at half maximum of a pure cosine is therefore
           2 pi / T in the continuum limit, against the widths of the sampled cosines of the Rust
           program (the column hwhm_window);
     WD-5  (no Rust check; Python YD-6) HS (0, 0): the reduced model of Anderson pseudospins (one at +eps_k and one at -eps_k for every
           radial node k, eps_k = (k^2 + m^2)^(1/2), each with half the weight of the node), integrated by
           NDSolve with the method IDA (of SUNDIALS), gives the d_0 of the gap equation and the d(t) of
           the Rust program after the kick 1e-3, 0 <= t <= 50.

   USAGE
     wolframscript -file sh_dynamics.wls <sh_dyn_runs.csv> <sh_dyn_signals.csv> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: sh_dynamics.wls <sh_dyn_runs.csv> <sh_dyn_signals.csv> <report.txt>"]; Exit[2]];
{runsFile, sigFile, reportFile} = args[[1 ;; 3]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
sci[x_] := ToString[CForm[N[x, 2]]];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
num[x_?NumericQ] := x;
readTable[file_] := Module[{t = Import[file, "CSV", "Numeric" -> False]}, {First[t], Rest[t]}];
col[h_, n_] := Position[h, n][[1, 1]];

(* ------------------------------------------------------------------ WD-1: the conservation laws *)
(* a symbolic Hermitian 3 x 3 matrix: real parts s[i, j], imaginary parts u[i, j] above the diagonal *)
herm[s_, u_] := Table[If[i == j, s[i, i], If[i < j, s[i, j] + I u[i, j], s[j, i] - I u[j, i]]], {i, 3}, {j, 3}];
hh = herm[a, ai]; rr = herm[b, bi];
dR = -I (hh . rr - rr . hh);
consOK = Simplify[ComplexExpand[Tr[hh . dR]], Element[_, Reals]] === 0;
(* R^2 = R is kept: d(R^2 - R)/dt = dR R + R dR - dR vanishes at R^2 = R; with R = P a projector of rank 1 *)
pv = {p1, p2, p3}; proj = Outer[Times, pv, Conjugate[pv]];
dP = -I (hh . proj - proj . hh);
projOK2 = Simplify[ComplexExpand[(dP . proj + proj . dP - dP) /. Thread[pv -> {1, 0, 0}]]] === ConstantArray[0, {3, 3}];
hermOK = Simplify[ComplexExpand[ConjugateTranspose[dR] - dR], Element[_, Reals]] === ConstantArray[0, {3, 3}];
check["WD-1", "the time-dependent mean field i dR/dt = [H, R] conserves tr(H R) when H is the derivative of the energy \
(d tr(H R)/dt = -i tr(H [H, R]) = 0), keeps R Hermitian, and keeps a projector a projector (symbolic, 3 x 3 Hermitian matrices \
with symbolic entries; the projector of rank 1 along the first axis)", consOK && hermOK && projOK2,
  "energy " <> ToString[consOK] <> ", hermiticity " <> ToString[hermOK] <> ", projector " <> ToString[projOK2]];

(* ------------------------------------------------------------------ WD-2: the Josephson relation *)
(* the Nambu space of one component: F = diag(1, -1); the BdG matrix {{a, p}, {Conjugate[p], -a}} *)
fN = DiagonalMatrix[{1, -1}];
uu = MatrixExp[I mu fN tt];
(* ComplexExpand takes every symbol as real; the identities are polynomial in the entries r, h, so that they then hold
   for complex entries too *)
rot = ComplexExpand[uu . {{r11, r12}, {r21, r22}} . ConjugateTranspose[uu]];
josOK = Simplify[rot[[1, 2]] - ComplexExpand[Exp[2 I mu tt]] r12] === 0 && Simplify[rot[[1, 1]] - r11] === 0;
(* the equation: i d/dt (U R0 U^dag) = [H0' - mu F, U R0 U^dag] with H0' = U H0 U^dag and [H0, R0] = 0 *)
h0 = {{h11, h12}, {h21, h22}}; r0 = {{q11, q12}, {q21, q22}};
lhs = I D[uu . r0 . ConjugateTranspose[uu], tt];
rhs = (uu . h0 . ConjugateTranspose[uu] - mu fN) . (uu . r0 . ConjugateTranspose[uu]) -
   (uu . r0 . ConjugateTranspose[uu]) . (uu . h0 . ConjugateTranspose[uu] - mu fN);
(* i dR/dt = -mu [F, R(t)] and the right side is U [H0, R0] U^dag - mu [F, R(t)]: they agree when [H0, R0] = 0 *)
eqOK = Simplify[ComplexExpand[(lhs - rhs) + uu . (h0 . r0 - r0 . h0) . ConjugateTranspose[uu]]] === ConstantArray[0, {2, 2}];
{shdr, srows} = readTable[sigFile];
jruns = Select[DeleteDuplicates[srows[[All, col[shdr, "run"]]]], StringStartsQ[#, "josephson"] &];
rates = Table[Module[{rows = Select[srows, #[[col[shdr, "run"]]] == r &], ts, ph, fit},
    ts = num /@ rows[[All, col[shdr, "t"]]];
    ph = Arg /@ (num[#[[col[shdr, "re_d"]]]] + I num[#[[col[shdr, "im_d"]]]] & /@ rows);
    (* unwrap *)
    ph = FoldList[#1 + Mod[#2 - #1 + Pi, 2 Pi] - Pi &, First[ph], Rest[ph]];
    fit = Fit[Transpose[{Rationalize[ts, 0], Rationalize[ph, 0]}], {1, x}, x];
    {r, Coefficient[fit, x]}], {r, jruns}];
dev2 = Max[Abs[Abs[#[[2]]] - 2/100]/(2/100) & /@ rates];
check["WD-2", "the Josephson relation: exp(i mu F t) R_0 exp(-i mu F t) solves i dR/dt = [H - mu F, R] for a stationary R_0 \
and multiplies the pair block by exp(2 i mu t) (symbolic); the phases of d(t) of the Josephson runs of the Rust program grow at \
the rate 2 mu = 0.02 (straight-line fit in exact arithmetic on the numbers of the file; relative)",
  josOK && eqOK && dev2 < 10^-6, "rates " <> StringRiffle[(#[[1]] <> " " <> ToString[NumberForm[N[#[[2]], 12], 12]]) & /@ rates, ", "] <>
   "; deviation " <> sci[dev2]];

(* ------------------------------------------------------------------ WD-3: the amplitude mode at d_0 *)
i1[mm_] := (Sqrt[1 + mm^2] - mm^2 ArcSinh[1/mm])/2;
mLL = 1/5; gSH = 504/5;
mEff = mm /. FindRoot[gSH i1[mm]/(4 Pi^2) == 1, {mm, 9/20}, WorkingPrecision -> 40];
d0 = 2 Sqrt[mEff^2 - mLL^2];
{rhdr, rrows} = readTable[runsFile];
sel = Select[rrows, #[[col[rhdr, "kind"]]] == "amplitude" && MemberQ[{"HS (0, 1)", "HS (0, 0)"}, #[[col[rhdr, "method"]]]] &];
dev3 = Max[Abs[num[#[[col[rhdr, "omega_peak"]]]] - d0]/d0 & /@ sel];
check["WD-3", "the peaks of the amplitude runs of HS (0, 1) and HS (0, 0) of the Rust program (both versions) lie at d_0 = " <>
  ToString[NumberForm[N[d0, 20], 20]] <> " of the gap equation (relative)", Length[sel] == 4 && dev3 < 10^-4, "deviation " <> sci[dev3]];

(* ------------------------------------------------------------------ WD-4: the width of the Hann window *)
wHann[x_] := Sin[x Pi]/(x Pi (1 - x^2));
(* the transform of sin^2(pi t / T) exp(-i w t) over [0, T], normalized at w = 0, at w = x 2 pi / T *)
tw[x_] := Integrate[Sin[Pi s]^2 Exp[I 2 Pi x s], {s, 0, 1}]/Integrate[Sin[Pi s]^2, {s, 0, 1}];
(* the squared modulus of the exact transform against the closed form at nine rational offsets, in exact arithmetic *)
formOK = AllTrue[{1/10, 3/10, 1/2, 7/10, 9/10, 11/10, 3/2, 5/2, 7/2},
   FullSimplify[ComplexExpand[Abs[tw[#]]^2] - wHann[#]^2] === 0 &];
halfOK = Limit[wHann[x], x -> 1] == 1/2;
tLong = Max[num /@ Select[rrows, #[[col[rhdr, "kind"]]] == "amplitude" &][[All, col[rhdr, "t_end"]]]];
ww = 2 Pi/tLong;
hw = num /@ Select[rrows, MemberQ[{"amplitude", "triplet", "large"}, #[[col[rhdr, "kind"]]]] &][[All, col[rhdr, "hwhm_window"]]];
dev4 = Max[Abs[hw - ww]/ww];
check["WD-4", "the Hann window: its normalized transform has the modulus |sin(x pi) / (x pi (1 - x^2))| at the offset x 2 pi / T \
(the squared modulus of the exact integral equals that of the closed form at nine rational offsets, exactly) and the \
value 1/2 at x = 1 (limit), so that the half width at half maximum of a pure cosine is 2 pi / T = " <>
  ToString[NumberForm[N[ww, 8], 8]] <> " for the runs of length " <> ToString[Round[tLong]] <> "; the widths of the sampled cosines of \
the Rust program against it (relative)", formOK && halfOK && dev4 < 10^-2,
  "form " <> ToString[formOK] <> ", half " <> ToString[halfOK] <> ", deviation " <> sci[dev4]];

(* ------------------------------------------------------------------ WD-5: the reduced model of pseudospins *)
(* HS (0, 0): for every radial node k one pseudospin n at +eps_k and one at -eps_k, eps_k = (k^2 + m^2)^(1/2), each
   with half the weight w_k = (Gauss-Legendre weight) k^2 / (2 pi^2) of the node; dn/dt = 2 b x n with
   b = (-Dx, -Dy, eps) and the gap parameter Dx - i Dy = (G/2) sum (w/2) (n^x - i n^y); d(t) = 2 |Dx - i Dy| *)
Needs["NumericalDifferentialEquationAnalysis`"];
gl = GaussianQuadratureWeights[24, 0, 1, 40];
kq = gl[[All, 1]]; wq = gl[[All, 2]] kq^2/(2 Pi^2);
ekq = Sqrt[kq^2 + mLL^2];
eps2 = Join[ekq, -ekq]; w2 = Join[wq, wq]/2; nps = Length[eps2];
dl0 = dd /. FindRoot[gSH/2 Total[w2/Sqrt[eps2^2 + dd^2]] == 1, {dd, 2/5}, WorkingPrecision -> 25];
kick = 1/1000; dli = dl0 (1 + kick);
nx0 = dli/Sqrt[eps2^2 + dli^2]; nz0 = -eps2/Sqrt[eps2^2 + dli^2];
vx = Table[Symbol["px" <> ToString[j]], {j, nps}]; vy = Table[Symbol["py" <> ToString[j]], {j, nps}];
vz = Table[Symbol["pz" <> ToString[j]], {j, nps}];
dxt = gSH/2 Total[w2 Through[vx[tq]]]; dyt = gSH/2 Total[w2 Through[vy[tq]]];
eqs = Flatten[Table[{
     vx[[j]]'[tq] == 2 ((-dyt) vz[[j]][tq] - eps2[[j]] vy[[j]][tq]),
     vy[[j]]'[tq] == 2 (eps2[[j]] vx[[j]][tq] - (-dxt) vz[[j]][tq]),
     vz[[j]]'[tq] == 2 ((-dxt) vy[[j]][tq] - (-dyt) vx[[j]][tq]),
     vx[[j]][0] == N[nx0[[j]]], vy[[j]][0] == 0, vz[[j]][0] == N[nz0[[j]]]}, {j, nps}]];
sol = First[NDSolve[eqs, Join[vx, vy, vz], {tq, 0, 50}, Method -> {"IDA"}, PrecisionGoal -> 11, AccuracyGoal -> 13,
     MaxSteps -> 10^7]];
dps[t_] := 2 Sqrt[(gSH/2 Total[w2 (Through[vx[t]] /. sol)])^2 + (gSH/2 Total[w2 (Through[vy[t]] /. sol)])^2];
amp = Select[srows, #[[col[shdr, "run"]]] == "amplitude_second_HS0_0" && num[#[[col[shdr, "t"]]]] <= 50 &];
dev5 = Max[Table[Abs[dps[num[r[[col[shdr, "t"]]]]] - Abs[num[r[[col[shdr, "re_d"]]]] + I num[r[[col[shdr, "im_d"]]]]]]/(2 dl0), {r, amp}]];
check["WD-5", "HS (0, 0): the reduced model of 48 Anderson pseudospins (one at +eps_k and one at -eps_k for each of the 24 \
radial nodes, eps_k = (k^2 + m^2)^(1/2)), integrated here by NDSolve with the method IDA, gives d_0 = 2 Delta_0 = " <>
  ToString[NumberForm[N[2 dl0, 15], 15]] <> " from its gap equation and, after the kick 1e-3, the d(t) of the Rust program for \
0 <= t <= 50 (largest |difference| / d_0, at the " <> ToString[Length[amp]] <> " stored times)", Abs[2 dl0 - d0]/d0 < 10^-12 && dev5 < 10^-6,
  "d_0 against the gap equation of WD-3 " <> sci[Abs[2 dl0 - d0]/d0] <> "; deviation " <> sci[dev5]];

out = StringJoin[
  "sh_dynamics.wls: the exact part of stage 6, part one: the dynamics in time of the Higgs superfluid (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WD and a number), the statement and, in brackets, the\n",
  "facts that decide it.  Symbolic algebra, and exact arithmetic on the numbers of the files.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
