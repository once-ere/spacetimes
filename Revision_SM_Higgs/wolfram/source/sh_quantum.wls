#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_quantum.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The third computation of the quantum numbers under the revised hypotheses H1-H5 (provenance
   document H0): the checks of the Rust program sh_quantum repeated in EXACT arithmetic (rational
   numbers, square roots and symbols), so that the floating-point tolerances of the Rust and Python
   programs are not needed.  The identifiers are those of the Rust program with the prefix W.

   The finite transformations of the checks QN-9 are not repeated: the exact identities of the
   generators (WQN-2, WQN-6, WQN-8) imply them, because SU(2) x U(1) and SL(2,C) are connected and
   every element is a product of exponentials of generators.

   The field: twelve components, 0..7 the isodoublet Psi_LL (component 2 d + s, Dirac index d,
   isospin index s = 0 up, 1 down), 8..11 the singlet chi_SH; the chiral basis of the Dirac
   matrices.  A pair is Pi[M] = phi1^T M phi2, a particle-antiparticle bilinear B[M] = phi1^dag M
   phi2; under phi -> exp(i eps X) phi the matrix M changes by i eps L_X(M), with
   L_X(M) = X1^T M + M X2 (pairs) and -X1 M + M X2 (particle-antiparticle).

   USAGE
     wolframscript -file sh_quantum.wls  <sh_channels.csv>  <report.txt>
   The first argument is the table of channels written by the Rust program (data/sh_channels.csv),
   with which the exact table is compared; the report is written to the second argument
   (data/hck_sh_quantum_checks_wolfram.txt in the repository).
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 2, Print["usage: sh_quantum.wls <sh_channels.csv> <report.txt>"]; Exit[2]];
{channelsFile, reportFile} = args[[1 ;; 2]];
t0 = AbsoluteTime[];

(* ------------------------------------------------------------------ the report *)
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
allZero[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[# == 0]] &];

(* ------------------------------------------------------------------ Pauli and Dirac matrices *)
sig[0] = IdentityMatrix[2]; sig[1] = {{0, 1}, {1, 0}}; sig[2] = {{0, -I}, {I, 0}}; sig[3] = {{1, 0}, {0, -1}};
sigbar[0] = sig[0]; sigbar[j_] := -sig[j];
z2 = ConstantArray[0, {2, 2}];
gam[p_] := ArrayFlatten[{{z2, sig[p]}, {sigbar[p], z2}}];
gch = I gam[0].gam[1].gam[2].gam[3];          (* diag(-1, -1, 1, 1) *)
cC = I gam[2].gam[0];                          (* the charge-conjugation matrix diag(E, -E) *)
d8[a_] := KroneckerProduct[a, IdentityMatrix[2]];
blk[a_, b_] := Module[{n1 = Length[a], n2 = Length[b], m},
  m = ConstantArray[0, {n1 + n2, n1 + n2}]; m[[1 ;; n1, 1 ;; n1]] = a; m[[n1 + 1 ;;, n1 + 1 ;;]] = b; m];
dirac12[a_] := blk[d8[a], a];
comm[a_, b_] := a.b - b.a; acomm[a_, b_] := a.b + b.a;
lev[a_, b_, c_] := Signature[{a, b, c}];

(* ------------------------------------------------------------------ the internal generators *)
tLL[a_] := KroneckerProduct[IdentityMatrix[4], sig[a]/2];
T[a_] := blk[tLL[a], ConstantArray[0, {4, 4}]];
FF = IdentityMatrix[12]; YY = FF/4; RR = blk[IdentityMatrix[8], -IdentityMatrix[4]];
QQ = T[3] + YY;
gens = {T[1], T[2], T[3], YY, QQ, FF, RR};
genNames = {"T1", "T2", "T3", "Y", "Q", "F", "R"};

(* WQN-1 *)
ok1 = And @@ Flatten[Table[comm[T[a], T[b]] == I Sum[lev[a, b, c] T[c], {c, 3}], {a, 3}, {b, 3}]] &&
  And @@ Flatten[Table[comm[T[a], x] == 0 IdentityMatrix[12], {a, 3}, {x, {YY, FF, RR}}]] &&
  And @@ (# == ConjugateTranspose[#] & /@ gens);
check["WQN-1", "[T^a, T^b] = i eps_abc T^c, Y, F, R commute with T^a, all generators Hermitian", ok1, "identities of exact matrices"];

(* ------------------------------------------------------------------ the Lorentz generators *)
lb = Join[Table[-I sig[j]/2, {j, 3}], Table[sig[j]/2, {j, 3}]];   (* rotations, boosts *)
sGen[mu_] := blk[-ConjugateTranspose[mu], mu];
l12 = dirac12[sGen[#]] & /@ lb;
ok2 = And @@ Flatten[Table[dirac12[sGen[comm[lb[[k]], lb[[l]]]]] == comm[l12[[k]], l12[[l]]], {k, 6}, {l, 6}]] &&
  And @@ Flatten[Table[comm[s, x] == 0 IdentityMatrix[12], {s, l12}, {x, gens}]] &&
  MatrixRank[Join[Re[Flatten[#]], Im[Flatten[#]]] & /@ l12] == 6;
check["WQN-2", "the six real Lorentz generators: S([u, u']) = [S(u), S(u')], independent over the reals, commuting with \
the internal generators", ok2, "identities of exact matrices; real rank 6"];

(* ------------------------------------------------------------------ WQN-4: the charges *)
qd = Diagonal[QQ];
ok4 = YY == FF/4 && qd == Join[Flatten[ConstantArray[{3/4, -1/4}, 4]], ConstantArray[1/4, 4]];
check["WQN-4", "Y = F/4 = 1/4; Q = 3/4 (up), -1/4 (down) on Psi_LL and 1/4 on chi_SH", ok4,
  "Q on the components 0..11: " <> ToString[qd, InputForm]];

(* ------------------------------------------------------------------ WQN-5: anomalies *)
ch12 = dirac12[gch];
traces = Join[{Tr[ch12.YY.YY.YY], Tr[ch12.YY]},
  Flatten[Table[{Tr[ch12.acomm[T[a], T[b]].YY], Tr[ch12.YY.YY.T[a]]}, {a, 3}, {b, 3}]],
  Flatten[Table[Tr[ch12.T[a].acomm[T[b], T[c]]], {a, 3}, {b, 3}, {c, 3}]]];
par12 = dirac12[gam[0]];
cas12 = Sum[T[a].T[a], {a, 3}];
nDoub = Count[Diagonal[cas12], 3/4];
ok5 = AllTrue[traces, # == 0 &] && And @@ (comm[par12, #] == 0 IdentityMatrix[12] & /@ gens) &&
  Mod[nDoub, 4] == 0 && EvenQ[nDoub/4];
check["WQN-5", "no anomaly (" <> ToString[Length[traces]] <> " traces exactly 0), vector-like fields, an even number of \
Weyl doublets", ok5, ToString[nDoub/4] <> " Weyl doublets"];

(* ------------------------------------------------------------------ bilinears *)
fieldOff = <|"Psi" -> 0, "chi" -> 8|>; fieldN = <|"Psi" -> 8, "chi" -> 4|>;
onField[x_, f_] := x[[fieldOff[f] + 1 ;; fieldOff[f] + fieldN[f], fieldOff[f] + 1 ;; fieldOff[f] + fieldN[f]]];
diracOn[f_, a_] := If[f == "Psi", d8[a], a];
leftOp[kind_, a_] := If[kind == "pair", Transpose[a], ConjugateTranspose[a]];
(* the matrix on vec(M) (row by row) of M -> left(A1) M + M A2 *)
vecOp[kind_, a1_, a2_] := KroneckerProduct[leftOp[kind, a1], IdentityMatrix[Length[a2]]] +
  KroneckerProduct[IdentityMatrix[Length[a1]], Transpose[a2]];
transpOp[n_] := Module[{p = ConstantArray[0, {n^2, n^2}]},
  Do[p[[j n + i + 1, i n + j + 1]] = 1, {i, 0, n - 1}, {j, 0, n - 1}]; p];
sectors = {{"pair", "Psi", "Psi"}, {"pair", "Psi", "chi"}, {"pair", "chi", "chi"},
  {"pa", "Psi", "Psi"}, {"pa", "Psi", "chi"}, {"pa", "chi", "chi"}};
secName[{k_, a_, b_}] := If[k == "pair", a <> " " <> b, a <> "^dag " <> b];
invOps[{kind_, f1_, f2_}, antisym_, sym_] := Module[{ops},
  ops = vecOp[kind, diracOn[f1, sGen[#]], diracOn[f2, sGen[#]]] & /@ lb;
  If[antisym, AppendTo[ops, IdentityMatrix[fieldN[f1]^2] + transpOp[fieldN[f1]]]];
  If[sym, AppendTo[ops, IdentityMatrix[fieldN[f1]^2] - transpOp[fieldN[f1]]]];
  Join @@ ops];
(* L_X on vec(M) *)
chargeOp[{kind_, f1_, f2_}, x_] := Module[{x1 = onField[x, f1], x2 = onField[x, f2]},
  KroneckerProduct[If[kind == "pair", Transpose[x1], -x1], IdentityMatrix[Length[x2]]] +
   KroneckerProduct[IdentityMatrix[Length[x1]], Transpose[x2]]];
parityOp[{kind_, f1_, f2_}] := KroneckerProduct[leftOp[kind, diracOn[f1, gam[0]]], Transpose[diracOn[f2, gam[0]]]];
(* the matrix of an operator on the span of the columns of b (b need not be orthonormal); the residual tells whether
   the span is invariant *)
restrict[op_, b_] := Module[{bt = Transpose[b], c},
  c = Inverse[ConjugateTranspose[bt].bt].ConjugateTranspose[bt].op.bt;
  {c, Simplify[op.bt - bt.c]}];

(* ------------------------------------------------------------------ WQN-6: the invariant spaces *)
want = {6, 4, 2, 8, 4, 2};
spaces = Table[NullSpace[invOps[sectors[[k]], sectors[[k, 1]] == "pair" && sectors[[k, 2]] == sectors[[k, 3]], False]],
  {k, 6}];
symSpace = NullSpace[invOps[sectors[[1]], False, True]];
dims = Length /@ spaces;
eIso[s_] := If[s == 0, {{1}, {0}}, {{0}, {1}}];
kp = Table[KroneckerProduct[cC.gch, eIso[s]], {s, 0, 1}];
km = Table[KroneckerProduct[cC, eIso[s]], {s, 0, 1}];
inSpan[v_, b_] := MatrixRank[Append[b, v]] == MatrixRank[b];
ok6 = dims == want && Length[symSpace] == 2 &&
  AllTrue[Join[kp, km], inSpan[Flatten[#], spaces[[2]]] &] && MatrixRank[Flatten /@ Join[kp, km]] == 4 &&
  AllTrue[Join[kp, km], allZero[invOps[sectors[[2]], False, False].Flatten[#]] &];
check["WQN-6", "the dimensions 6, 4, 2 (pairs Psi Psi, Psi chi, chi chi), 8, 4, 2 (particle-antiparticle) and 2 \
(symmetric Psi Psi); C gamma^ch (x) e_s, C (x) e_s are invariant and span the Psi chi pairs", ok6,
  "dimensions " <> ToString[dims] <> ", symmetric " <> ToString[Length[symSpace]]];

(* ------------------------------------------------------------------ WQN-7: parity *)
pp = parityOp[sectors[[2]]];
ok7 = And @@ (pp.Flatten[#] == Flatten[#] & /@ kp) && And @@ (pp.Flatten[#] == -Flatten[#] & /@ km);
check["WQN-7", "parity: +1 for C gamma^ch (x) e_s, -1 for C (x) e_s", ok7, "identities of exact matrices"];

(* ------------------------------------------------------------------ WQN-8: the Higgs channel *)
hb = Flatten[#]/Sqrt[Total[Abs[Flatten[#]]^2]] & /@ kp;      (* orthonormal *)
gHiggs[x_] := Table[Simplify[Conjugate[hb[[sp]]].(chargeOp[sectors[[2]], x].hb[[s]])], {s, 2}, {sp, 2}];
hg = gHiggs /@ gens;
closure8 = And @@ Flatten[Table[allZero[chargeOp[sectors[[2]], gens[[i]]].hb[[s]] - Sum[hg[[i, s, sp]] hb[[sp]],
  {sp, 2}]], {i, 7}, {s, 2}]];
ok8 = And @@ Table[hg[[a]] == sig[a]/2, {a, 3}] && hg[[4]] == IdentityMatrix[2]/2 && hg[[5]] == {{1, 0}, {0, 0}} &&
  hg[[6]] == 2 IdentityMatrix[2] && hg[[7]] == ConstantArray[0, {2, 2}] &&
  Sum[hg[[a]].hg[[a]], {a, 3}] == 3/4 IdentityMatrix[2] && closure8;
check["WQN-8", "on Phi_SH: G_{T^a} = tau^a/2, G_Y = 1/2, G_Q = diag(1, 0), G_F = 2, G_R = 0, Casimir 3/4, closure: the \
generators of the Higgs doublet", ok8, "G_Y = " <> ToString[hg[[4]], InputForm]];

(* ------------------------------------------------------------------ WQN-10: the twelve channels *)
su2Content[cas_] := Module[{vals = Sort[cas], dims = {}, u},
  u = Tally[vals];
  Do[With[{c = t[[1]], n = t[[2]], d = Sqrt[1 + 4 t[[1]]]}, dims = Join[dims, ConstantArray[d, n/d]]], {t, u}];
  StringRiffle[ToString /@ Sort[dims], " + "]];
fracStr[x_] := ToString[x, InputForm];
fracsStr[v_] := StringRiffle[fracStr /@ Reverse[Sort[v]], "; "];
channels = {}; closureAll = True;
Do[Module[{sec = sectors[[k]], b = spaces[[k]], pr, res, ev, sub},
   {pr, res} = restrict[parityOp[sec], b];
   closureAll = closureAll && allZero[res];
   Do[Module[{nsp = NullSpace[pr - sign IdentityMatrix[Length[pr]]], subb, lx, cas, yv, qv},
      subb = nsp.b;                                (* rows: the vectors of the part of parity sign *)
      lx = Table[Module[{c, r}, {c, r} = restrict[chargeOp[sec, gens[[i]]], subb]; closureAll = closureAll && allZero[r];
         c], {i, 7}];
      cas = Simplify[Eigenvalues[Sum[lx[[a]].lx[[a]], {a, 3}]]];
      yv = Simplify[Eigenvalues[lx[[4]]]]; qv = Simplify[Eigenvalues[lx[[5]]]];
      AppendTo[channels, <|"sector" -> secName[sec], "parity" -> sign, "dim" -> Length[subb],
        "su2" -> su2Content[cas], "Y" -> fracsStr[yv], "Q" -> fracsStr[qv], "qv" -> qv, "gens" -> lx[[1 ;; 4]],
        "lxQ" -> lx[[5]]|>]],
    {sign, {1, -1}}]],
  {k, 6}];
rust = Import[channelsFile, "CSV", "Numeric" -> False];   (* every field as a string: "-" would otherwise not stay "-" *)
hdr = First[rust]; rows = Rest[rust];
col[name_] := Position[hdr, name][[1, 1]];
(* the parity is written as +1 or -1 (the importer would read a lone "-" as a missing value) *)
parityOf[x_] := Which[MemberQ[{"+1", "1", 1}, x], 1, MemberQ[{"-1", -1}, x], -1, True, 0];
rustOf[sec_, par_] := SelectFirst[rows, #[[col["sector"]]] == sec && parityOf[#[[col["parity"]]]] == par &, Missing[]];
str[x_] := If[StringQ[x], x, ToString[x]];
mism = Select[channels, Module[{r = rustOf[#["sector"], #["parity"]]},
     MissingQ[r] || str[r[[col["dimension"]]]] != ToString[#["dim"]] || str[r[[col["SU2_multiplets"]]]] != #["su2"] ||
      str[r[[col["Y"]]]] != #["Y"] || str[r[[col["Q"]]]] != #["Q"]] &];
check["WQN-10", "the twelve channels: parity split, closure, SU(2) multiplets, Y and Q agree with the table of the \
Rust program", closureAll && mism === {} && Length[rows] == 12,
  If[mism === {}, "all 12 agree", "differ: " <> StringRiffle[Module[{r = rustOf[#["sector"], #["parity"]]},
      #["sector"] <> " (" <> ToString[#["parity"]] <> "): exact " <> ToString[#["dim"]] <> " / " <> #["su2"] <> " / " <> #["Y"] <>
       " / " <> #["Q"] <> ", Rust " <> If[MissingQ[r], "missing", ToString[r, InputForm]]] & /@ mism, "; "]]];
minQ = Min[Abs[Flatten[#["qv"] & /@ Select[channels, MemberQ[{"Psi Psi", "chi chi"}, #["sector"]] &]]]];
check["WQN-10e", "no pair of two Psi_LL or of two chi_SH particles is neutral", minQ == 1/2,
  "smallest absolute value of Q " <> fracStr[minQ]];

(* ------------------------------------------------------------------ WQN-11: the breaking *)
(* the real coefficients c with (sum_A c_A G_A) w = 0 for all pairs (G, w) *)
stab[conds_] := Module[{cs = Array[cc, 4], eqs},
  eqs = Flatten[Table[ComplexExpand[{Re[#], Im[#]} & /@ (Sum[cs[[a]] p[[1, a]], {a, 4}].p[[2]])], {p, conds}]];
  NullSpace[Normal[CoefficientArrays[eqs, cs][[2]]]]];
phi0 = {0, 1};
st0 = stab[{{hg[[1 ;; 4]], phi0}}];
ok11b = Length[st0] == 1 && Normalize[st0[[1]]] (Sign[Last[st0[[1]]]]) == {0, 0, 1, 1}/Sqrt[2];
(* a fixed generic vector with exact entries for each channel, and a vector of its neutral part *)
generic[n_] := Table[(j + 1) + I (2 j - 1), {j, n}];
res11 = Table[Module[{w = generic[ch["dim"]], st, nz, wn, stn, allNeutral},
    st = stab[{{hg[[1 ;; 4]], phi0}, {ch["gens"], w}}];
    allNeutral = AllTrue[ch["qv"], # == 0 &];
    nz = NullSpace[ch["lxQ"]];
    stn = If[nz === {}, None, stab[{{hg[[1 ;; 4]], phi0}, {ch["gens"], Total[MapIndexed[(#2[[1]] + 2) #1 &, nz]]}}]];
    {Length[st] == If[allNeutral, 1, 0],
     stn === None || (Length[stn] == 1 && Normalize[stn[[1]]] Sign[Last[stn[[1]]]] == {0, 0, 1, 1}/Sqrt[2])}],
   {ch, channels}];
check["WQN-11", "the stabilizer of (0, phi) is spanned by Q = T^3 + Y; a generic second condensate with a charged \
component breaks Q, a neutral one keeps exactly Q", ok11b && And @@ Flatten[res11],
  "stabilizer " <> ToString[st0, InputForm]];

(* ------------------------------------------------------------------ WQN-12: the mass matrix *)
(* the restricted matrices of the channel computation act on the coefficient vectors; for the Higgs channel the
   orthonormal hb gives G directly (WQN-8) *)
phiv = {0, v/Sqrt[2]};
ga = {g, g, g, gp};
m2 = Table[Simplify[ga[[a]] ga[[b]] ComplexExpand[Re[Conjugate[phiv].acomm[hg[[a]], hg[[b]]].phiv]],
    Assumptions -> {g > 0, gp > 0, v > 0}], {a, 4}, {b, 4}];
ev12 = Simplify[Eigenvalues[m2], Assumptions -> {g > 0, gp > 0, v > 0}];
wantEv = {0, g^2 v^2/4, g^2 v^2/4, (g^2 + gp^2) v^2/4};
nul = NullSpace[m2];
ok12 = Sort[Expand[ev12]] === Sort[Expand[wantEv]] && Length[nul] == 1 &&
  Simplify[nul[[1]] - (nul[[1, 4]]/g) {0, 0, gp, g}] == {0, 0, 0, 0};
check["WQN-12", "the mass matrix of (W^1, W^2, W^3, B) from Phi_SH = (0, v/sqrt2): eigenvalues 0, g^2 v^2/4 (twice), \
(g^2 + g'^2) v^2/4, the massless vector along (0, 0, g', g) (symbolic couplings)", ok12,
  "eigenvalues " <> ToString[ev12, InputForm]];

(* ------------------------------------------------------------------ the report *)
out = StringJoin[
  "sh_quantum.wls: the exact computation of the checks of sh_quantum (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier (that of the Rust program with the prefix W), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic: no tolerance.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0, 0.1]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
