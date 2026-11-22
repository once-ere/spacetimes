#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_higgs.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The exact part of stage 4 of the revision of the hypotheses, second version (the isodoublet
   Psi_LL and the two singlets chi_SH, chi'_SH), with the first version as the comparison: the
   formulas with which the Rust program sh_higgs computes the masses of the gauge bosons, rho and
   the amplitude mode, derived here in exact arithmetic from the effective Lagrangian of the pair
   fields, and the closed forms of the method HS (0, 0), in which only the pairing terms act,
   compared with the data of the Rust program.  The identifiers are those of the Rust program with
   the prefix W.

     WH-1  the custodial algebra of the effective Lagrangian Z tr((d M)^dag d M) of the bidoublet
           M_SH -> L M_SH R^T at the vacuum M_SH = phi i tau^2, with chemical potentials of the six
           generators T^a (on L) and T^a_R (on R): the susceptibilities are chi(T^a, T^b) =
           chi(T^a_R, T^b_R) = -chi(T^a, T^b_R) = Z phi^2 delta_ab; with Y = T^3_R + R/4 (R unbroken)
           the electric charge Q = T^3 + Y does not respond, the mass matrix g_a g_b chi^{ab} of
           (W^1, W^2, W^3, B) has the eigenvalues 0, g^2 Z phi^2 (twice), (g^2 + g'^2) Z phi^2, and
           rho = 1 (symbolic g, g', Z, phi);
     WH-2  the mass of the amplitude mode from the effective Lagrangian: for the two pair fields of the
           second version (Phi^d with T^3 = -1/2, Phi'^u with +1/2, moving together) and for the one of
           the first version, M^2 = V'' phi_0^2 / (4 chi^{33}) (symbolic);
     WH-3  the closed forms of HS (0, 0): the gap equation (G_SH / 4 pi^2) I1(M_eff) = 1 with
           M_eff = (m^2 + d_0^2/4)^(1/2), phi_0 = d_0 I1(M_eff) / (4 pi^2) = d_0 / G_SH, and the
           curvature of the effective potential of one pair, V''(phi_0) = 2 pi^2 d_0^2 J / (I1 (I1 -
           d_0^2 J / 4)) with J = int_0^1 q^2 (q^2 + M_eff^2)^(-3/2) dq (twice that in the second
           version, where both pairs move together), solved and evaluated to 30 digits, against the
           Rust values of both versions;
     WH-4  the conversion to GeV and the comparison with the Standard Model: Lambda_LL = v / v_SH,
           M_H = omega_H Lambda_LL, the ratio M_H / v against m_H / v = 125.13 / v, from the data of the
           Rust program (exact rational arithmetic on the numbers of the files);
     WH-5  the amplitude mode of HS (0, 0) lies at omega = d_0 exactly: with E_k^2 = k^2 + m^2 + d_0^2/4
           the amplitude and phase eigenvalues of the linear response (the closed forms of the theory of
           Bardeen, Cooper and Schrieffer, which the Python program checks against its response)
             lambda_amp(omega)   = (G_SH / 4 pi^2) int_0^1 dk k^2 (k^2 + m^2) / (E_k (E_k^2 - omega^2/4)),
             lambda_phase(omega) = (G_SH / 4 pi^2) int_0^1 dk k^2 E_k / (E_k^2 - omega^2/4)
           obey the exact identity lambda_amp(omega) - lambda_phase(0) = ((omega^2 - d_0^2)/4) S(omega),
           S(omega) = (G_SH / 4 pi^2) int_0^1 dk k^2 / (E_k (E_k^2 - omega^2/4)) > 0 for omega below
           2 M_eff = (4 m^2 + d_0^2)^(1/2); lambda_phase(0) = 1 is the gap equation, so lambda_amp = 1
           only at omega = d_0 (the integrals with 30 digits); the frequencies of the Rust program against d_0.

   USAGE
     wolframscript -file sh_higgs.wls <sh_hg_minimum.csv> <sh_hg_masses.csv> <sh_hg_values.csv> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 4, Print["usage: sh_higgs.wls <sh_hg_minimum.csv> <sh_hg_masses.csv> <sh_hg_values.csv> <report.txt>"]; Exit[2]];
{minFile, massFile, valFile, reportFile} = args[[1 ;; 4]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
sci[x_] := ToString[CForm[N[x, 2]]];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
readTable[file_] := Module[{t = Import[file, "CSV", "Numeric" -> False]}, {First[t], Rest[t]}];
col[h_, n_] := Position[h, n][[1, 1]];

(* ------------------------------------------------------------------ WH-1: the custodial algebra *)
tau = {{{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
eps = {{0, 1}, {-1, 0}};                       (* i tau^2 *)
mvac = phi eps;
(* the chemical potentials mu_a of T^a (acting on the left) and nu_a of T^a_R (acting on the right, M -> M R^T) *)
amat[mu_List, nu_List] := Sum[mu[[a]] tau[[a]]/2, {a, 3}].mvac + mvac.Transpose[Sum[nu[[a]] tau[[a]]/2, {a, 3}]];
(* ComplexExpand takes every symbol as real: the chemical potentials, phi and Z *)
energy[mu_List, nu_List] := -zz ComplexExpand[Tr[ConjugateTranspose[amat[mu, nu]].amat[mu, nu]]];
vars = Join[Array[mu, 3], Array[nu, 3]];
en = Simplify[energy[Array[mu, 3], Array[nu, 3]]];
chi6 = -Table[D[en, vars[[i]], vars[[j]]], {i, 6}, {j, 6}];
chiWant = zz phi^2 ArrayFlatten[{{IdentityMatrix[3], -IdentityMatrix[3]}, {-IdentityMatrix[3], IdentityMatrix[3]}}];
okStruct = Simplify[chi6 - chiWant] == ConstantArray[0, {6, 6}];
(* Y = T^3_R + R/4 with R unbroken (zero susceptibility): chi^{YY} = chi(T^3_R, T^3_R), chi^{3Y} = chi(T^3, T^3_R) *)
chi4 = {{chi6[[1, 1]], chi6[[1, 2]], chi6[[1, 3]], chi6[[1, 6]]}, {chi6[[2, 1]], chi6[[2, 2]], chi6[[2, 3]], chi6[[2, 6]]},
  {chi6[[3, 1]], chi6[[3, 2]], chi6[[3, 3]], chi6[[3, 6]]}, {chi6[[6, 1]], chi6[[6, 2]], chi6[[6, 3]], chi6[[6, 6]]}};
qvec = {0, 0, 1, 1};
okQ = Simplify[chi4.qvec] == {0, 0, 0, 0};
cpl = {g, g, g, gp};
m2 = Table[cpl[[a]] cpl[[b]] chi4[[a, b]], {a, 4}, {b, 4}];
evs = Simplify[Eigenvalues[m2], Assumptions -> g > 0 && gp > 0 && zz > 0 && phi > 0];
okEv = Simplify[CharacteristicPolynomial[m2, x] - x (x - g^2 zz phi^2)^2 (x - (g^2 + gp^2) zz phi^2)] === 0;
rhoSym = Simplify[(g^2 zz phi^2)/((g^2 + gp^2) zz phi^2 g^2/(g^2 + gp^2))];
check["WH-1", "the effective Lagrangian Z tr((dM)^dag dM) of the bidoublet at M = phi i tau^2: chi(T^a, T^b) = chi(T^a_R, \
T^b_R) = -chi(T^a, T^b_R) = Z phi^2 delta_ab; with Y = T^3_R + R/4 the charge Q = T^3 + Y does not respond; the mass matrix \
of (W^1, W^2, W^3, B) has the eigenvalues 0, g^2 Z phi^2 (twice), (g^2 + g'^2) Z phi^2, so M_W = g v/2 and M_Z = (g^2 + \
g'^2)^(1/2) v/2 with v^2 = 4 Z phi^2, and rho = " <> ToString[rhoSym, InputForm] <> " (symbolic)",
  okStruct && okQ && okEv && rhoSym === 1, "eigenvalues " <> ToString[evs, InputForm]];

(* ------------------------------------------------------------------ WH-2: the amplitude mode of the effective Lagrangian *)
(* second version: L = Z (|d Phi^d|^2 + |d Phi'^u|^2) - V(phi), both lengths phi0 + eta; chi^{33} from a chemical
   potential of T^3 (charges -1/2 and +1/2): E(mu) = V - Z mu^2 (1/4 + 1/4) phi0^2 *)
chi33v2 = -D[-zz mu3^2 (1/4 + 1/4) phi0^2, {mu3, 2}];
(* the amplitude: kinetic term 2 Z eta'^2, potential V'' eta^2 / 2: omega^2 = V'' / (4 Z) *)
om2v2 = vpp/(4 zz);
okV2 = Simplify[om2v2 - vpp phi0^2/(4 chi33v2)] == 0;
(* first version: L = Z |d Phi^d|^2 - V(phi); chi^{33} = 2 Z (1/4) phi0^2; amplitude: Z eta'^2 - V'' eta^2/2 *)
chi33v1 = -D[-zz mu3^2 (1/4) phi0^2, {mu3, 2}];
om2v1 = vpp/(2 zz);
okV1 = Simplify[om2v1 - vpp phi0^2/(4 chi33v1)] == 0;
check["WH-2", "the effective Lagrangian of the pair fields gives the mass of the amplitude mode M^2 = V'' phi_0^2 / (4 \
chi^{33}) in both versions: second version chi^{33} = " <> ToString[chi33v2, InputForm] <> ", M^2 = V''/(4 Z); first \
version chi^{33} = " <> ToString[chi33v1, InputForm] <> ", M^2 = V''/(2 Z) (symbolic)", okV2 && okV1, "identities"];

(* ------------------------------------------------------------------ WH-3: the closed forms of HS (0, 0) *)
i1[mm_] := (Sqrt[1 + mm^2] - mm^2 ArcSinh[1/mm])/2;
jj[mm_] := ArcSinh[1/mm] - 1/Sqrt[1 + mm^2];
okJ = Simplify[D[i1[mm], mm] + mm jj[mm], mm > 0] == 0 &&
  Max[Table[Abs[N[NIntegrate[q^2 (q^2 + mm^2)^(-3/2), {q, 0, 1}, WorkingPrecision -> 40] - jj[mm], 40]], {mm, {1/10, 1/5, 1/2, 1}}]] < 10^-30;
mLL = 1/5; gSH = 504/5;
mEff = mm /. FindRoot[gSH i1[mm]/(4 Pi^2) == 1, {mm, 9/20}, WorkingPrecision -> 40];
d0 = 2 Sqrt[mEff^2 - mLL^2];
phi0 = d0 i1[mEff]/(4 Pi^2);
jv = jj[mEff];
vppOne = 2 Pi^2 d0^2 jv/(i1[mEff] (i1[mEff] - d0^2 jv/4));
{mhdr, mrows} = readTable[minFile];
minOf[ver_, meth_] := SelectFirst[mrows, #[[col[mhdr, "version"]]] == ver && #[[col[mhdr, "method"]]] == meth &];
dev3 = Max[Flatten[Table[Module[{r = minOf[ver, "HS (0, 0)"], fac = If[ver == "second", 2, 1]},
      {Abs[num[r[[col[mhdr, "d0"]]]] - d0]/d0, Abs[num[r[[col[mhdr, "phi0"]]]] - phi0]/phi0,
       Abs[num[r[[col[mhdr, "V2_at_phi0"]]]] - fac vppOne]/(fac vppOne)}], {ver, {"second", "first"}}]]];
check["WH-3", "HS (0, 0): the gap equation (G_SH / 4 pi^2) I1(M_eff) = 1 gives M_eff = " <> ToString[NumberForm[N[mEff, 16], 16]] <>
  ", d_0 = " <> ToString[NumberForm[N[d0, 16], 16]] <> ", phi_0 = d_0 I1 / (4 pi^2) = d_0 / G_SH = " <> ToString[NumberForm[N[phi0, 16], 16]] <>
  "; dI1/dM = -M J with J = arsinh(1/M) - (1 + M^2)^(-1/2) (exact; J equals its integral to 40 digits); V''(phi_0) = 2 pi^2 \
d_0^2 J / (I1 (I1 - d_0^2 J/4)) = " <> ToString[NumberForm[N[vppOne, 16], 16]] <> " for one pair (first version) and twice \
that, " <> ToString[NumberForm[N[2 vppOne, 16], 16]] <> ", for the two pairs of the second version; against the Rust values \
(relative)", okJ && dev3 < 10^-6, "deviation " <> sci[dev3]];

(* ------------------------------------------------------------------ WH-4: GeV and the Standard Model *)
{shdr, srows} = readTable[massFile];
{vhdr, vrows} = readTable[valFile];
valOf[name_] := num[SelectFirst[vrows, #[[1]] == name &][[2]]];
vGeV = valOf["v_GeV"];
mhOverV = valOf["m_H_over_v_SM"];
okSM = Abs[mhOverV - 12513/100/vGeV] < 10^-12;
(* the rows of the methods without a computation of the linear response carry NaN in its columns; the CSV import
   turns the field NaN into Missing["NotAvailable"] *)
noRpa[r_] := With[{x = r[[col[shdr, "M_H_RPA"]]]}, MissingQ[x] || (StringQ[x] && StringContainsQ[x, "NaN"])];
dev4 = Max[Table[Module[{vsh = num[r[[col[shdr, "v_SH"]]]], lam = num[r[[col[shdr, "Lambda_GeV"]]]], mr = r[[col[shdr, "M_H_RPA"]]],
      mrg = r[[col[shdr, "M_H_RPA_GeV"]]], mrv = r[[col[shdr, "M_H_RPA_over_v_SH"]]]},
     If[noRpa[r], Abs[lam - vGeV/vsh]/lam,
      Max[Abs[lam - vGeV/vsh]/lam, Abs[num[mrg] - num[mr] lam]/num[mrg], Abs[num[mrv] - num[mr]/vsh]/num[mrv]]]], {r, srows}]];
ratios = Table[num[r[[col[shdr, "M_H_RPA_over_v_SH"]]]], {r, Select[srows, ! noRpa[#] &]}];
check["WH-4", "m_H / v = 125.13 / v = " <> ToString[NumberForm[N[mhOverV, 10], 10]] <> " (the inputs of stage 2); for every \
row of the masses, Lambda_LL = v / v_SH, M_H = omega_H Lambda_LL and M_H / v = omega_H / v_SH (relative); the amplitude modes of \
the linear response give M_H / v from " <> ToString[NumberForm[Min[ratios], 6]] <> " to " <> ToString[NumberForm[Max[ratios], 6]] <>
  ", " <> ToString[NumberForm[Min[ratios]/mhOverV, 4]] <> " to " <> ToString[NumberForm[Max[ratios]/mhOverV, 4]] <>
  " times the value of the Standard Model", okSM && dev4 < 10^-9, "deviation " <> sci[dev4]];

(* ------------------------------------------------------------------ WH-5: the amplitude mode at omega = d_0 *)
(* the identity per momentum, symbolic: e2 = k^2 + m^2, E^2 = e2 + d^2/4 *)
ek[k_, m_, d_] := Sqrt[k^2 + m^2 + d^2/4];
idOK = Simplify[(k^2 + m^2)/(ek[k, m, d] (ek[k, m, d]^2 - w^2/4)) - 1/ek[k, m, d] ==
    (w^2 - d^2)/(4 ek[k, m, d] (ek[k, m, d]^2 - w^2/4)), k > 0 && m > 0 && d > 0 && w >= 0];
(* positivity of the factor below the threshold: E^2 - w^2/4 > 0 for every k when w < 2 M_eff, that is
   w^2 < 4 m^2 + d^2 (a statement about all real numbers, decided by Resolve) *)
posOK = Resolve[ForAll[{k, m, d, w}, k >= 0 && m > 0 && d > 0 && w >= 0 && w^2 < 4 m^2 + d^2,
    k^2 + m^2 + d^2/4 - w^2/4 > 0], Reals];
(* the continuum integrals at the d_0 of WH-3 (40-digit arithmetic) *)
(* d_0 carries about 40 digits (FindRoot of WH-3), so the integrals are taken with 30; the frequency is raised to 40
   digits inside, because FindRoot passes it with the precision 30 *)
lamAmp[w_?NumericQ] := With[{ww = SetPrecision[w, 40]}, gSH/(4 Pi^2) NIntegrate[
    k^2 (k^2 + mLL^2)/(ek[k, mLL, d0] (ek[k, mLL, d0]^2 - ww^2/4)), {k, 0, 1}, WorkingPrecision -> 30, PrecisionGoal -> 25]];
lamPh[w_?NumericQ] := With[{ww = SetPrecision[w, 40]}, gSH/(4 Pi^2) NIntegrate[
    k^2 ek[k, mLL, d0]/(ek[k, mLL, d0]^2 - ww^2/4), {k, 0, 1}, WorkingPrecision -> 30, PrecisionGoal -> 25]];
dAmp = Abs[lamAmp[d0] - 1]; dPh = Abs[lamPh[0] - 1];
wFound = w /. FindRoot[lamAmp[w] == 1, {w, 7/10, 9/10}, WorkingPrecision -> 30];
thr = Sqrt[4 mLL^2 + d0^2];
(* the frequencies of the Rust program (linear response of the whole sector) against d_0 *)
tagsW = {"second_HS0_1", "second_HS0_0", "first_HS0_1", "first_HS0_0"};
dev5 = Max[Table[Abs[valOf["amp_omega_" <> t] - d0]/d0, {t, tagsW}]];
check["WH-5", "HS (0, 0): the identity (k^2 + m^2) / (E (E^2 - w^2/4)) - 1/E = (w^2 - d^2) / (4 E (E^2 - w^2/4)) with \
E^2 = k^2 + m^2 + d^2/4 holds for every momentum (symbolic) and E^2 - w^2/4 > 0 for w < 2 M_eff (symbolic), so that \
lambda_amp(w) = 1 only at w = d_0; the continuum integrals give |lambda_amp(d_0) - 1| = " <> sci[dAmp] <>
  " and |lambda_phase(0) - 1| = " <> sci[dPh] <> " (the gap equation), the root of lambda_amp = 1 is " <>
  ToString[NumberForm[N[wFound, 20], 20]] <> " against d_0 = " <> ToString[NumberForm[N[d0, 20], 20]] <>
  ", below 2 M_eff = " <> ToString[NumberForm[N[thr, 16], 16]] <> " (binding " <> ToString[NumberForm[N[thr - d0, 6], 6]] <>
  "); the amplitude modes of the Rust program (HS (0, 1) and HS (0, 0), both versions) equal d_0 (relative)",
  TrueQ[idOK] && TrueQ[posOK] && dAmp < 10^-22 && dPh < 10^-22 && Abs[wFound - d0]/d0 < 10^-22 && dev5 < 10^-9,
  "identity " <> ToString[idOK] <> ", positivity " <> ToString[posOK] <> "; Rust deviation " <> sci[dev5]];

out = StringJoin[
  "sh_higgs.wls: the exact formulas of stage 4, second version with the first as the comparison (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier (that of the Rust program with the prefix W), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic, and numbers with 30 or 40 digits where a value of the Rust program is compared.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0, 0.1]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
