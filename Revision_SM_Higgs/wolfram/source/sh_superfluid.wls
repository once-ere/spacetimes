#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_superfluid.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The exact part of stage 3 of the revision of the hypotheses: the formulas with which the Rust
   program sh_superfluid compares its static response of the normal vacuum, derived here in exact
   arithmetic, and their values compared with the data of the Rust program.  The identifiers are
   those of the Rust program with the prefix W.

     WSF-2a  the pair matrices: the old triplet matrix K^3 = C gamma^ch (x) E tau_3 couples the up and
             the down components of Psi_LL through the blocks -C gamma^ch (so Psi^T K^3 Psi =
             -2 Psi_u^T C gamma^ch Psi_d), the new K_d couples the down components and chi_SH through
             the blocks C gamma^ch / 2 (so Psi^T K_d Psi = Psi_d^T C gamma^ch chi = 2 Pi^d_SH): the same
             Lorentz channel between two Dirac fields of equal mass, with the coefficient 2 in place
             of 1, hence G^crit_SH = 4 G^crit_LL;
     WSF-2b  I1(m) = int_0^1 q^2/sqrt(q^2+m^2) dq in closed form, G^crit_SH = 4 pi^2 / I1(1/5) and
             G_SH / G^crit_SH, against the Rust value;
     WSF-3   the exchange contraction of the pairing term in the scalar channel: a scalar density n of
             Psi_LL (drho = n beta/8 on Psi_LL) gives chi_SH the self-energy -(G_SH/16) n beta, and a
             scalar density n' of chi_SH (drho = n' beta/4) gives Psi_LL the self-energy
             -(G_SH/16) n' beta (x) 1_2; the scalar susceptibility in closed form; the largest eigenvalue
             of the 2 x 2 response of KS and CJT for V^(1)_SH against the Rust value;
     WSF-4   the triplet pair coupling -(1/4) G^S - G^V = 6/5 of KS and CJT and its eigenvalue
             (6/5) / G^crit_LL against the Rust value.

   USAGE
     wolframscript -file sh_superfluid.wls  <sh_sf_normal_vacuum.csv>  <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 2, Print["usage: sh_superfluid.wls <sh_sf_normal_vacuum.csv> <report.txt>"]; Exit[2]];
{nvFile, reportFile} = args[[1 ;; 2]];
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
upIdx = Range[1, 8, 2]; dnIdx = Range[2, 8, 2]; chiIdx = Range[9, 12];
k3 = KroneckerProduct[aa, eps2.sig[3]];        (* the old triplet matrix on the eight components *)
vS[s_] := KroneckerProduct[aa, eIso[s]]/2;
kS[s_] := Module[{k = ConstantArray[0, {12, 12}]}, k[[1 ;; 8, 9 ;; 12]] = vS[s]; k[[9 ;; 12, 1 ;; 8]] = -Transpose[vS[s]]; k];

(* WSF-2a: the blocks of the pair matrices *)
blockOf[m_, r_, c_] := m[[r, c]];
k12 = ArrayFlatten[{{k3, ConstantArray[0, {8, 4}]}, {ConstantArray[0, {4, 8}], ConstantArray[0, {4, 4}]}}];
ok2a = blockOf[k3, upIdx, dnIdx] == -aa && blockOf[k3, dnIdx, upIdx] == -aa &&
  blockOf[k3, upIdx, upIdx] == ConstantArray[0, {4, 4}] && blockOf[k3, dnIdx, dnIdx] == ConstantArray[0, {4, 4}] &&
  blockOf[kS[1], dnIdx, chiIdx] == aa/2 && blockOf[kS[1], chiIdx, dnIdx] == aa/2 &&
  blockOf[kS[1], upIdx, chiIdx] == ConstantArray[0, {4, 4}] && blockOf[kS[1], upIdx, upIdx] == ConstantArray[0, {4, 4}] &&
  Transpose[aa] == -aa && Transpose[kS[1]] == -kS[1] && Transpose[k3] == -k3;
check["WSF-2a", "the triplet matrix K^3 couples the up and down components of Psi_LL by -C gamma^ch in both blocks \
(Psi^T K^3 Psi = -2 Psi_u^T C gamma^ch Psi_d); K_d couples the down components and chi_SH by C gamma^ch / 2 in both \
blocks (Psi^T K_d Psi = Psi_d^T C gamma^ch chi_SH): the same channel with half the coefficient, so the pairing term \
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
nv = Import[nvFile, "CSV", "Numeric" -> False];
hdr = First[nv]; rows = Rest[nv];
col[n_] := Position[hdr, n][[1, 1]];
rowOf[pot_, meth_] := SelectFirst[rows, #[[col["potential"]]] == pot && #[[col["method"]]] == meth &];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
lamRust = num[rowOf["V1_SH", "CJT"][[col["lam_higgs"]]]];
dev2b = Abs[N[lamExact, 30] - lamRust];
check["WSF-2b", "I1(m) = int_0^1 q^2 / sqrt(q^2 + m^2) dq = (sqrt(1 + m^2) - m^2 arsinh(1/m)) / 2 \
(the exact integral equals it at ten masses, to 40 digits); G^crit_SH = 4 pi^2 / I1(1/5) = " <> ToString[NumberForm[N[gcritSH, 20], 20]] <> " and G_SH / G^crit_SH = " <>
  ToString[NumberForm[N[lamExact, 16], 16]] <> " (G_SH = 504/5), against the Rust value", okI1 && dev2b < 10^-9,
  "deviation " <> sci[dev2b]];

(* WSF-3: the exchange contraction of the pairing term in the scalar channel *)
rhoPsi[n_] := ArrayFlatten[{{n KroneckerProduct[bet, IdentityMatrix[2]]/8, ConstantArray[0, {8, 4}]}, {ConstantArray[0, {4, 8}], ConstantArray[0, {4, 4}]}}];
rhoChi[n_] := ArrayFlatten[{{ConstantArray[0, {8, 8}], ConstantArray[0, {8, 4}]}, {ConstantArray[0, {4, 8}], n bet/4}}];
fock[rho_] := Sum[-g ConjugateTranspose[kS[s]].Transpose[rho].kS[s], {s, 0, 1}];
f1 = fock[rhoPsi[n]]; f2 = fock[rhoChi[n]];
ok3a = Simplify[f1[[9 ;; 12, 9 ;; 12]] - (-(g/16) n bet)] == ConstantArray[0, {4, 4}] &&
  Simplify[f1[[1 ;; 8, 1 ;; 8]]] == ConstantArray[0, {8, 8}] &&
  Simplify[f2[[1 ;; 8, 1 ;; 8]] - (-(g/16) n KroneckerProduct[bet, IdentityMatrix[2]])] == ConstantArray[0, {8, 8}] &&
  Simplify[f2[[9 ;; 12, 9 ;; 12]]] == ConstantArray[0, {4, 4}];
chiInt = Integrate[q^4/(q^2 + m^2)^(3/2), {q, 0, 1}, Assumptions -> m > 0];
chi8 = 2/Pi^2 chiInt /. m -> mLL;
chi4 = chi8/2;
gPP = 7/8 (-8) - 1/2 (4/5);                   (* (7/8) G^S - (1/2) G^V with G^S = -8, G^V = 4/5 *)
gPX = -gSH/16;
smat = {{-gPP chi8, -gPX chi4}, {-gPX chi8, 0}};
lamS = Max[Re[Eigenvalues[N[smat, 30]]]];
lamSRust = num[rowOf["V1_SH", "CJT"][[col["lam_scalar"]]]];
dev3 = Abs[lamS - lamSRust];
check["WSF-3", "the exchange contraction of the pairing term: a scalar density n of Psi_LL gives chi_SH the self-energy \
-(G_SH/16) n beta and a scalar density of chi_SH gives Psi_LL -(G_SH/16) n beta (x) 1_2 (exact); the susceptibility \
chi_S = (2/pi^2) int_0^1 q^4/(q^2 + m^2)^(3/2) dq = " <> ToString[NumberForm[N[chi8, 16], 16]] <>
  " (Psi_LL; half for chi_SH); the largest eigenvalue of the 2 x 2 scalar response of KS and CJT for V^(1)_SH = " <>
  ToString[NumberForm[lamS, 12]] <> ", against the Rust value", ok3a && dev3 < 10^-9, "deviation " <> sci[dev3]];

(* WSF-4: the triplet pair coupling *)
gTrip = -1/4 (-8) - 4/5;
lamTrip = gTrip/(Pi^2/i1closed[mLL]);
lamTripRust = num[rowOf["V1_SH", "CJT"][[col["lam_triplet"]]]];
dev4 = Abs[N[lamTrip, 30] - lamTripRust];
check["WSF-4", "the scalar triplet pair coupling of KS and CJT, -(1/4) G^S - G^V = " <> ToString[gTrip, InputForm] <>
  ", and its eigenvalue in the normal vacuum, (6/5) I1(1/5) / pi^2 = " <> ToString[NumberForm[N[lamTrip, 16], 16]] <>
  ", against the Rust value", gTrip == 6/5 && dev4 < 10^-9, "deviation " <> sci[dev4]];

out = StringJoin[
  "sh_superfluid.wls: the exact formulas of stage 3 (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier (that of the Rust program with the prefix W), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic, and numbers with 30 digits where a value of the Rust program is compared.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0, 0.1]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
