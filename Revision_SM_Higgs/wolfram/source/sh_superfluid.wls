#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_superfluid.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The exact part of stage 3 of the revision of the hypotheses, second version (the isodoublet
   Psi_LL and the two singlets chi_SH, chi'_SH; sixteen components): the formulas with which the
   Rust program sh_superfluid compares its static response of the normal vacuum and its ground
   states, derived here in exact arithmetic, and their values compared with the data of the Rust
   program.  The identifier of a check is WSF and its number in this script; each entry names, in
   parentheses, the checks of the Rust program sh_superfluid and of the Python program
   python/sh_superfluid.py that test the same statements in floating point.

     WSF-2a  (Rust SF-2a; Python YSF-2) the pair matrices: the old triplet matrix K^3 = C gamma^ch (x) E tau_3 couples the up and
             the down components of Psi_LL through the blocks -C gamma^ch; the new matrices couple the
             down components with chi_SH (K_d) and the up components with chi'_SH (K'_u) through the
             blocks C gamma^ch / 2: the same Lorentz channel between two Dirac fields of equal mass,
             with the coefficient 2 in place of 1, hence G^crit_SH = 4 G^crit_LL;
     WSF-2b  (Rust SF-2a; Python YSF-2) I1(m) = int_0^1 q^2/sqrt(q^2+m^2) dq in closed form, G^crit_SH = 4 pi^2 / I1(1/5) and
             G_SH / G^crit_SH, against the Rust values of both versions;
     WSF-3   (Rust SF-3a, SF-3b; Python YSF-3) the exchange contraction of the pairing terms in the scalar channel: a scalar density n
             of Psi_LL gives each paired singlet the self-energy -(G_SH/16) n beta, and a scalar
             density of a paired singlet gives Psi_LL -(G_SH/16) n beta (x) 1_2; the scalar
             susceptibility in closed form; the largest eigenvalue of the 3 x 3 response of CJT for
             V^(1)_SH in both versions against the Rust values;
     WSF-4   (Rust SF-4a; no Python check) the triplet pair coupling -(1/4) G^S - G^V = 6/5 of KS and CJT and its eigenvalue
             (6/5) / G^crit_LL against the Rust value;
     WSF-5   (Rust SF-6a, SF-6b; Python YSF-6, YSF-12) the ground states of HS (0, 1) and HS (0, 0), where only the pairing terms act: the gap
             equation G_SH I1(M_eff) / (4 pi^2) = 1, M_eff = (m^2 + |D|^2/4)^(1/2), solved to 30 digits;
             |D| against the singular values of the Rust program (both equal to |D| in the second
             version; |D| and 0 in the first).

   USAGE
     wolframscript -file sh_superfluid.wls  <sh_sf_normal_vacuum.csv>  <sh_sf_ground.csv>  <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: sh_superfluid.wls <sh_sf_normal_vacuum.csv> <sh_sf_ground.csv> <report.txt>"]; Exit[2]];
{nvFile, grFile, reportFile} = args[[1 ;; 3]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];

(* ------------------------------------------------------------------ matrices *)
sig[0] = IdentityMatrix[2]; sig[1] = {{0, 1}, {1, 0}}; sig[2] = {{0, -I}, {I, 0}}; sig[3] = {{1, 0}, {0, -1}};
sigbar[0] = sig[0]; sigbar[j_] := -sig[j];
z2 = ConstantArray[0, {2, 2}];
gam[p_] := ArrayFlatten[{{z2, sig[p]}, {sigbar[p], z2}}];
gch = I gam[0].gam[1].gam[2].gam[3];
cC = I gam[2].gam[0];
bet = gam[0];
eps2 = {{0, 1}, {-1, 0}};
aa = cC.gch;                                     (* the Lorentz scalar pair matrix C gamma^ch, 4 x 4 *)
eIso[s_] := If[s == 0, {{1}, {0}}, {{0}, {1}}];
upIdx = Range[1, 8, 2]; dnIdx = Range[2, 8, 2]; chiIdx = Range[9, 12]; chi2Idx = Range[13, 16];
z44 = ConstantArray[0, {4, 4}];
k3 = KroneckerProduct[aa, eps2.sig[3]];        (* the old triplet matrix on the eight components *)
vS[s_] := KroneckerProduct[aa, eIso[s]]/2;
(* the pair matrix of Psi_LL with the singlet a (0: chi_SH, 1: chi'_SH) and the isospin component s *)
kP[a_, s_] := Module[{k = ConstantArray[0, {16, 16}], o = 8 + 4 a},
  k[[1 ;; 8, o + 1 ;; o + 4]] = vS[s]; k[[o + 1 ;; o + 4, 1 ;; 8]] = -Transpose[vS[s]]; k];

(* WSF-2a: the blocks of the pair matrices *)
blockOf[m_, r_, c_] := m[[r, c]];
ok2a = blockOf[k3, upIdx, dnIdx] == -aa && blockOf[k3, dnIdx, upIdx] == -aa &&
  blockOf[k3, upIdx, upIdx] == z44 && blockOf[k3, dnIdx, dnIdx] == z44 &&
  blockOf[kP[0, 1], dnIdx, chiIdx] == aa/2 && blockOf[kP[0, 1], chiIdx, dnIdx] == aa/2 &&
  blockOf[kP[0, 1], upIdx, chiIdx] == z44 && blockOf[kP[0, 1], dnIdx, chi2Idx] == z44 &&
  blockOf[kP[1, 0], upIdx, chi2Idx] == aa/2 && blockOf[kP[1, 0], chi2Idx, upIdx] == aa/2 &&
  blockOf[kP[1, 0], dnIdx, chi2Idx] == z44 && blockOf[kP[1, 0], upIdx, chiIdx] == z44 &&
  Transpose[aa] == -aa && Transpose[kP[0, 1]] == -kP[0, 1] && Transpose[kP[1, 0]] == -kP[1, 0] && Transpose[k3] == -k3;
check["WSF-2a", "the triplet matrix K^3 couples the up and down components of Psi_LL by -C gamma^ch in both blocks \
(Psi^T K^3 Psi = -2 Psi_u^T C gamma^ch Psi_d); K_d couples the down components and chi_SH, K'_u the up components and \
chi'_SH, by C gamma^ch / 2 in both blocks: the same channel with half the coefficient, so the pairing term \
-(G/4) |Psi^T K Psi|^2 has the critical coupling G^crit_SH = 4 G^crit_LL", ok2a, "identities of exact matrices"];

(* WSF-2b: I1, G^crit *)
i1closed[m_] := (Sqrt[1 + m^2] - m^2 ArcSinh[1/m])/2;
i1int = Integrate[q^2/Sqrt[q^2 + m^2], {q, 0, 1}, Assumptions -> m > 0];
(* the closed form against the exact integral at ten rational masses, with 40 digits *)
okI1 = Max[Table[Abs[N[i1int /. m -> mm, 40] - N[i1closed[mm], 40]], {mm, {1/50, 1/10, 1/5, 3/10, 1/2, 3/4, 1, 3/2, 2, 5}}]] < 10^-35;
sci[x_] := ToString[CForm[N[x, 2]]];
mLL = 1/5; gSH = 504/5;                      (* G_SH = 100.8 = 504/5 *)
gcritSH = 4 Pi^2/i1closed[mLL];
lamExact = gSH/gcritSH;
readTable[file_] := Module[{t = Import[file, "CSV", "Numeric" -> False]}, {First[t], Rest[t]}];
{hdr, rows} = readTable[nvFile];
{ghdr, grows} = readTable[grFile];
col[h_, n_] := Position[h, n][[1, 1]];
rowOf[pot_, meth_] := SelectFirst[rows, #[[col[hdr, "potential"]]] == pot && #[[col[hdr, "method"]]] == meth &];
grOf[ver_, meth_] := SelectFirst[grows, #[[col[ghdr, "version"]]] == ver && #[[col[ghdr, "method"]]] == meth &];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
lamRust = num[rowOf[#, "CJT"][[col[hdr, "lam_higgs"]]]] & /@ {"V1_SH", "V1_SH first version"};
dev2b = Max[Abs[N[lamExact, 30] - lamRust]];
check["WSF-2b", "I1(m) = int_0^1 q^2 / sqrt(q^2 + m^2) dq = (sqrt(1 + m^2) - m^2 arsinh(1/m)) / 2 \
(the exact integral equals it at ten masses, to 40 digits); G^crit_SH = 4 pi^2 / I1(1/5) = " <> ToString[NumberForm[N[gcritSH, 20], 20]] <> " and G_SH / G^crit_SH = " <>
  ToString[NumberForm[N[lamExact, 16], 16]] <> " (G_SH = 504/5), against the Rust values of both versions", okI1 && dev2b < 10^-9,
  "deviation " <> sci[dev2b]];

(* WSF-3: the exchange contraction of the pairing terms in the scalar channel *)
emb[x_, r_] := Module[{m = ConstantArray[0, {16, 16}]}, m[[r, r]] = x; m];
rhoPsi[n_] := emb[n KroneckerProduct[bet, IdentityMatrix[2]]/8, Range[1, 8]];
rhoChi[n_] := emb[n bet/4, chiIdx];
rhoChi2[n_] := emb[n bet/4, chi2Idx];
fock[rho_, a_] := Sum[-g ConjugateTranspose[kP[a, s]].Transpose[rho].kP[a, s], {s, 0, 1}];
fock2[rho_] := fock[rho, 0] + fock[rho, 1];
f1 = fock2[rhoPsi[n]]; f2 = fock2[rhoChi[n]]; f3 = fock2[rhoChi2[n]];
z88 = ConstantArray[0, {8, 8}];
ok3a = Simplify[f1[[chiIdx, chiIdx]] - (-(g/16) n bet)] == z44 && Simplify[f1[[chi2Idx, chi2Idx]] - (-(g/16) n bet)] == z44 &&
  Simplify[f1[[1 ;; 8, 1 ;; 8]]] == z88 &&
  Simplify[f2[[1 ;; 8, 1 ;; 8]] - (-(g/16) n KroneckerProduct[bet, IdentityMatrix[2]])] == z88 &&
  Simplify[f3[[1 ;; 8, 1 ;; 8]] - (-(g/16) n KroneckerProduct[bet, IdentityMatrix[2]])] == z88 &&
  Simplify[f2[[chiIdx, chiIdx]]] == z44 && Simplify[f3[[chi2Idx, chi2Idx]]] == z44;
chiInt = Integrate[q^4/(q^2 + m^2)^(3/2), {q, 0, 1}, Assumptions -> m > 0];
chi8 = 2/Pi^2 chiInt /. m -> mLL;
chi4 = chi8/2;
gPP = 7/8 (-8) - 1/2 (4/5);                   (* (7/8) G^S - (1/2) G^V with G^S = -8, G^V = 4/5 *)
lamOf[g2_] := Module[{gPX = -gSH/16, gPX2 = -g2/16, smat},
  smat = {{-gPP chi8, -gPX chi4, -gPX2 chi4}, {-gPX chi8, 0, 0}, {-gPX2 chi8, 0, 0}};
  Max[Re[Eigenvalues[N[smat, 30]]]]];
lamS = {lamOf[gSH], lamOf[0]};
lamSRust = num[rowOf[#, "CJT"][[col[hdr, "lam_scalar"]]]] & /@ {"V1_SH", "V1_SH first version"};
dev3 = Max[Abs[lamS - lamSRust]];
check["WSF-3", "the exchange contraction of the pairing terms: a scalar density n of Psi_LL gives each paired singlet \
the self-energy -(G_SH/16) n beta and a scalar density of a paired singlet gives Psi_LL -(G_SH/16) n beta (x) 1_2 (exact); \
the susceptibility chi_S = (2/pi^2) int_0^1 q^4/(q^2 + m^2)^(3/2) dq = " <> ToString[NumberForm[N[chi8, 16], 16]] <>
  " (Psi_LL; half for a singlet); the largest eigenvalue of the 3 x 3 scalar response of CJT for V^(1)_SH = " <>
  ToString[NumberForm[lamS[[1]], 12]] <> " (second version) and " <> ToString[NumberForm[lamS[[2]], 12]] <>
  " (first), against the Rust values", ok3a && dev3 < 10^-9, "deviation " <> sci[dev3]];

(* WSF-4: the triplet pair coupling *)
gTrip = -1/4 (-8) - 4/5;
lamTrip = gTrip/(Pi^2/i1closed[mLL]);
lamTripRust = num[rowOf["V1_SH", "CJT"][[col[hdr, "lam_triplet"]]]];
dev4 = Abs[N[lamTrip, 30] - lamTripRust];
check["WSF-4", "the scalar triplet pair coupling of KS and CJT, -(1/4) G^S - G^V = " <> ToString[gTrip, InputForm] <>
  ", and its eigenvalue in the normal vacuum, (6/5) I1(1/5) / pi^2 = " <> ToString[NumberForm[N[lamTrip, 16], 16]] <>
  ", against the Rust value", gTrip == 6/5 && dev4 < 10^-9, "deviation " <> sci[dev4]];

(* WSF-5: the gap equation of the pairing-only ground states *)
mEff = mm /. FindRoot[gSH i1closed[mm]/(4 Pi^2) == 1, {mm, 9/20}, WorkingPrecision -> 40];
dAbs = 2 Sqrt[mEff^2 - mLL^2];
dev5 = Max[Flatten[Table[Module[{r = grOf[ver, meth], s1, s2},
      s1 = num[r[[col[ghdr, "sing1"]]]]; s2 = num[r[[col[ghdr, "sing2"]]]];
      {Abs[s1 - dAbs], Abs[s2 - If[ver == "second", dAbs, 0]]}], {ver, {"second", "first"}}, {meth, {"HS (0, 1)", "HS (0, 0)"}}]]];
check["WSF-5", "HS (0, 1) and HS (0, 0), where only the pairing terms act: the gap equation G_SH I1(M_eff) / (4 pi^2) = 1 \
gives M_eff = " <> ToString[NumberForm[N[mEff, 16], 16]] <> " and |D| = 2 (M_eff^2 - m^2)^(1/2) = " <>
  ToString[NumberForm[N[dAbs, 16], 16]] <> " (30 digits); the singular values of the bidoublet of the Rust program are |D| \
and |D| in the second version and |D| and 0 in the first", dev5 < 10^-9, "deviation " <> sci[dev5]];

out = StringJoin[
  "sh_superfluid.wls: the exact formulas of stage 3, second version (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WSF and a number), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic, and numbers with 30 digits where a value of the Rust program is compared.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
