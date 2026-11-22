#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_quantum.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The third computation of the quantum numbers under the revised hypotheses H1-H6, second version
   (the isodoublet Psi_LL and the two singlets chi_SH and chi'_SH; provenance document H0): the
   checks of the Rust program sh_quantum repeated in EXACT arithmetic (rational numbers, square
   roots and symbols), so that the floating-point tolerances of the Rust and Python programs are not
   needed.  The identifier of a check is W and the name of the family of checks of the Rust program
   to which its statements belong (and of the Python check YQN-n of the same family); a check need
   not test every statement of its family.  The Rust checks that each one tests:
     WQN-1 QN-1a to QN-1e; WQN-2 QN-2a, QN-2b; WQN-4 QN-4a to QN-4c; WQN-5 QN-5a to QN-5c;
     WQN-6 QN-6a to QN-6e (and, infinitesimally, the Lorentz invariance of QN-9e: its spaces are the
     null spaces of the Lorentz generators); WQN-7 QN-7; WQN-8 QN-8a to QN-8g; WQN-9 QN-9a to QN-9d
     (infinitesimally); WQN-10 QN-10a to QN-10c and QN-10g; WQN-10e QN-10e, QN-10f; WQN-11 QN-11b,
     QN-11d, QN-11e; WQN-11f QN-11f, QN-11g; WQN-12 QN-12b, QN-12d, QN-12f.  The families QN-3 (the
     free Hamiltonians) and the checks QN-10d, QN-11a, QN-11c, QN-12a, QN-12c, QN-12e and QN-12g have
     no counterpart here.

   The finite transformations of the checks QN-9 are replaced by their infinitesimal form (WQN-9):
   the groups are connected, so every element is a product of exponentials of generators, and the
   exact identities of the generators imply the finite ones.

   The field: sixteen components, 0..7 the isodoublet Psi_LL (component 2 d + s, Dirac index d,
   isospin index s = 0 up, 1 down), 8..11 the singlet chi_SH, 12..15 the singlet chi'_SH; the
   chiral basis of the Dirac matrices.  The pair of singlets taken as one field, "(chi, chi')",
   has the components 8..15 (component 8 + 4 a + d, a = 0 for chi_SH and 1 for chi'_SH).  A pair is
   Pi[M] = phi1^T M phi2, a particle-antiparticle bilinear B[M] = phi1^dag M phi2; under
   phi -> exp(i eps X) phi the matrix M changes by i eps L_X(M), with L_X(M) = X1^T M + M X2
   (pairs) and -X1 M + M X2 (particle-antiparticle).

   USAGE
     wolframscript -file sh_quantum.wls  <sh_channels.csv>  <sh_custodial_channels.csv>  <report.txt>
   The first two arguments are the tables of channels written by the Rust program
   (data/sh_channels.csv and data/sh_custodial_channels.csv), with which the exact tables are
   compared; the report is written to the third argument (data/hck_sh_quantum_checks_wolfram.txt
   in the repository).
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: sh_quantum.wls <sh_channels.csv> <sh_custodial_channels.csv> <report.txt>"];
  Exit[2]];
{channelsFile, custodialFile, reportFile} = args[[1 ;; 3]];
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
(* the block-diagonal matrix of the given square matrices *)
blk[ms__] := Module[{l = {ms}, n, m, o = 0},
  n = Total[Length /@ l]; m = ConstantArray[0, {n, n}];
  Do[m[[o + 1 ;; o + Length[x], o + 1 ;; o + Length[x]]] = x; o += Length[x], {x, l}]; m];
dirac16[a_] := blk[d8[a], a, a];
comm[a_, b_] := a.b - b.a; acomm[a_, b_] := a.b + b.a;
lev[a_, b_, c_] := Signature[{a, b, c}];
z16 = ConstantArray[0, {16, 16}]; z8 = ConstantArray[0, {8, 8}]; i2 = IdentityMatrix[2]; i4 = IdentityMatrix[4];

(* ------------------------------------------------------------------ the internal generators *)
T[a_] := blk[KroneckerProduct[IdentityMatrix[4], sig[a]/2], z8];       (* SU(2)_L on Psi_LL *)
TR[a_] := blk[z8, KroneckerProduct[sig[a]/2, IdentityMatrix[4]]];      (* SU(2)_R on (chi, chi') *)
FF = IdentityMatrix[16];                                               (* the particle number F_tot *)
YY = blk[IdentityMatrix[12]/4, -3 IdentityMatrix[4]/4];                (* the hypercharges of the table of H1 *)
RR = blk[IdentityMatrix[8], -IdentityMatrix[8]];                       (* the relative number *)
QQ = T[3] + YY;
gens = {T[1], T[2], T[3], YY, QQ, FF, RR, TR[3]};                      (* the order of the data files *)
gensAll = Join[gens, {TR[1], TR[2]}];
iY = 4; iQ = 5; iF = 6; iR = 7; iTR = {9, 10, 8};

(* WQN-1 *)
ok1 = And @@ Flatten[Table[comm[T[a], T[b]] == I Sum[lev[a, b, c] T[c], {c, 3}], {a, 3}, {b, 3}]] &&
  And @@ Flatten[Table[comm[TR[a], TR[b]] == I Sum[lev[a, b, c] TR[c], {c, 3}], {a, 3}, {b, 3}]] &&
  And @@ Flatten[Table[comm[T[a], TR[b]] == z16, {a, 3}, {b, 3}]] &&
  And @@ Flatten[Table[comm[x, z] == z16, {x, {T[1], T[2], T[3], YY, FF, RR, TR[3]}}, {z, {YY, FF, RR, TR[3]}}]] &&
  And @@ Flatten[Table[comm[TR[a], x] == z16, {a, 3}, {x, {FF, RR}}]] &&
  And @@ (# == ConjugateTranspose[#] & /@ gensAll) &&
  YY == TR[3] + RR/4 && QQ == T[3] + TR[3] + RR/4 && comm[YY, TR[1]] =!= z16;
check["WQN-1", "the algebra of SU(2)_L x SU(2)_R x U(1)_R x U(1)_F_tot, Hermitian generators, Y = T^3_R + R/4 \
(4.3), Q = T^3 + T^3_R + R/4, and [Y, T^1_R] != 0", ok1, "identities of exact matrices"];

(* ------------------------------------------------------------------ the Lorentz generators *)
lb = Join[Table[-I sig[j]/2, {j, 3}], Table[sig[j]/2, {j, 3}]];   (* rotations, boosts *)
sGen[mu_] := blk[-ConjugateTranspose[mu], mu];
l16 = dirac16[sGen[#]] & /@ lb;
ok2 = And @@ Flatten[Table[dirac16[sGen[comm[lb[[k]], lb[[l]]]]] == comm[l16[[k]], l16[[l]]], {k, 6}, {l, 6}]] &&
  And @@ Flatten[Table[comm[s, x] == z16, {s, l16}, {x, gensAll}]] &&
  MatrixRank[Join[Re[Flatten[#]], Im[Flatten[#]]] & /@ l16] == 6;
check["WQN-2", "the six real Lorentz generators: S([u, u']) = [S(u), S(u')], independent over the reals, commuting with \
the ten internal generators", ok2, "identities of exact matrices; real rank 6"];

(* ------------------------------------------------------------------ WQN-4: the charges *)
qd = Diagonal[QQ];
ok4 = qd == Join[Flatten[ConstantArray[{3/4, -1/4}, 4]], ConstantArray[1/4, 4], ConstantArray[-3/4, 4]] &&
  Diagonal[4 YY] == Join[ConstantArray[1, 12], ConstantArray[-3, 4]] &&
  Sum[T[a].T[a], {a, 3}] == blk[3/4 IdentityMatrix[8], z8] && Sum[TR[a].TR[a], {a, 3}] == blk[z8, 3/4 IdentityMatrix[8]];
check["WQN-4", "Q = 3/4 (up), -1/4 (down) on Psi_LL, 1/4 on chi_SH, -3/4 on chi'_SH; 4 Y = F_tot on Psi_LL and chi_SH, \
-3 F_tot on chi'_SH; the Casimir operators 3/4 of SU(2)_L on Psi_LL and of SU(2)_R on the singlets", ok4,
  "Q on the components 0..15: " <> ToString[qd, InputForm]];

(* ------------------------------------------------------------------ WQN-5: anomalies *)
ch16 = dirac16[gch];
traces = Join[{Tr[ch16.YY.YY.YY], Tr[ch16.YY]},
  Flatten[Table[{Tr[ch16.acomm[T[a], T[b]].YY], Tr[ch16.YY.YY.T[a]]}, {a, 3}, {b, 3}]],
  Flatten[Table[Tr[ch16.T[a].acomm[T[b], T[c]]], {a, 3}, {b, 3}, {c, 3}]]];
par16 = dirac16[gam[0]];
nDoub = Count[Diagonal[Sum[T[a].T[a], {a, 3}]], 3/4];
ok5 = AllTrue[traces, # == 0 &] && And @@ (comm[par16, #] == z16 & /@ gensAll) && Mod[nDoub, 4] == 0 && EvenQ[nDoub/4];
check["WQN-5", "no anomaly (" <> ToString[Length[traces]] <> " traces exactly 0), vector-like fields, an even number of \
Weyl doublets", ok5, ToString[nDoub/4] <> " Weyl doublets"];

(* ------------------------------------------------------------------ bilinears *)
fieldOff = <|"Psi" -> 0, "chi" -> 8, "chi'" -> 12, "(chi, chi')" -> 8|>;
fieldN = <|"Psi" -> 8, "chi" -> 4, "chi'" -> 4, "(chi, chi')" -> 8|>;
onField[x_, f_] := x[[fieldOff[f] + 1 ;; fieldOff[f] + fieldN[f], fieldOff[f] + 1 ;; fieldOff[f] + fieldN[f]]];
diracOn[f_, a_] := Which[f == "Psi", d8[a], f == "(chi, chi')", blk[a, a], True, a];
leftOp[kind_, a_] := If[kind == "pair", Transpose[a], ConjugateTranspose[a]];
(* the matrix on vec(M) (row by row) of M -> left(A1) M + M A2 *)
vecOp[kind_, a1_, a2_] := KroneckerProduct[leftOp[kind, a1], IdentityMatrix[Length[a2]]] +
  KroneckerProduct[IdentityMatrix[Length[a1]], Transpose[a2]];
transpOp[n_] := Module[{p = ConstantArray[0, {n^2, n^2}]},
  Do[p[[j n + i + 1, i n + j + 1]] = 1, {i, 0, n - 1}, {j, 0, n - 1}]; p];
sectors = {{"pair", "Psi", "Psi"}, {"pair", "Psi", "chi"}, {"pair", "Psi", "chi'"}, {"pair", "chi", "chi"},
  {"pair", "chi", "chi'"}, {"pair", "chi'", "chi'"},
  {"pa", "Psi", "Psi"}, {"pa", "Psi", "chi"}, {"pa", "Psi", "chi'"}, {"pa", "chi", "chi"}, {"pa", "chi", "chi'"},
  {"pa", "chi'", "chi'"}};
csectors = {{"pair", "Psi", "(chi, chi')"}, {"pair", "(chi, chi')", "(chi, chi')"}, {"pa", "Psi", "(chi, chi')"},
  {"pa", "(chi, chi')", "(chi, chi')"}};
secName[{k_, a_, b_}] := If[k == "pair", a <> " " <> b, a <> "^dag " <> b];
samePair[{k_, a_, b_}] := k == "pair" && a == b;
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
want = {6, 4, 4, 2, 2, 2, 8, 4, 4, 2, 2, 2};
spaces = Table[NullSpace[invOps[sec, samePair[sec], False]], {sec, sectors}];
symDims = Length[NullSpace[invOps[sectors[[#]], False, True]]] & /@ {1, 4, 6};
cspaces = Table[NullSpace[invOps[sec, samePair[sec], False]], {sec, csectors}];
dims = Length /@ spaces; cdims = Length /@ cspaces;
eIso[s_] := If[s == 0, {{1}, {0}}, {{0}, {1}}];
kp = Table[KroneckerProduct[cC.gch, eIso[s]], {s, 0, 1}];
km = Table[KroneckerProduct[cC, eIso[s]], {s, 0, 1}];
inSpan[v_, b_] := MatrixRank[Append[b, v]] == MatrixRank[b];
ok6 = dims == want && symDims == {2, 0, 0} && cdims == {8, 6, 8, 8} &&
  AllTrue[Join[kp, km], inSpan[Flatten[#], spaces[[2]]] && inSpan[Flatten[#], spaces[[3]]] &] &&
  MatrixRank[Flatten /@ Join[kp, km]] == 4 &&
  AllTrue[Join[kp, km], allZero[invOps[sectors[[2]], False, False].Flatten[#]] &];
check["WQN-6", "the dimensions 6, 4, 4, 2, 2, 2 (pairs Psi Psi, Psi chi, Psi chi', chi chi, chi chi', chi' chi'), 8, 4, 4, \
2, 2, 2 (particle-antiparticle), 2, 0, 0 (symmetric Psi Psi, chi chi, chi' chi'), 8, 6, 8, 8 (the pair of singlets as \
one field); C gamma^ch (x) e_s, C (x) e_s are invariant and span the pairs Psi chi and Psi chi'", ok6,
  "dimensions " <> ToString[dims] <> ", symmetric " <> ToString[symDims] <> ", custodial " <> ToString[cdims]];

(* ------------------------------------------------------------------ WQN-7: parity *)
ok7 = And @@ Flatten[Table[With[{pp = parityOp[sectors[[k]]]},
     {pp.Flatten[#] == Flatten[#] & /@ kp, pp.Flatten[#] == -Flatten[#] & /@ km}], {k, {2, 3}}]];
check["WQN-7", "parity: +1 for C gamma^ch (x) e_s, -1 for C (x) e_s, with either singlet", ok7,
  "identities of exact matrices"];

(* ------------------------------------------------------------------ WQN-8: the pair fields and the bidoublet *)
hb = Flatten[#]/Sqrt[Total[Abs[Flatten[#]]^2]] & /@ kp;      (* orthonormal *)
(* the generators G_{s s'} = <M_s', L_X(M_s)> on an orthonormal basis *)
gOn[sec_, basis_, x_] := Table[Simplify[Conjugate[basis[[sp]]].(chargeOp[sec, x].basis[[s]])], {s, Length[basis]},
   {sp, Length[basis]}];
closed[sec_, basis_, glist_] := And @@ Flatten[Table[Module[{g = gOn[sec, basis, glist[[i]]]},
      allZero[chargeOp[sec, glist[[i]]].basis[[s]] - Sum[g[[s, sp]] basis[[sp]], {sp, Length[basis]}]]],
     {i, Length[glist]}, {s, Length[basis]}]];
hg = gOn[sectors[[2]], hb, #] & /@ gens;
hg2 = gOn[sectors[[3]], hb, #] & /@ gens;
eps = {{0, 1}, {-1, 0}};
ok8a = And @@ Table[hg[[a]] == sig[a]/2 && hg2[[a]] == sig[a]/2, {a, 3}] && hg[[iY]] == i2/2 && hg2[[iY]] == -i2/2 &&
  hg[[iQ]] == {{1, 0}, {0, 0}} && hg2[[iQ]] == {{0, 0}, {0, -1}} && hg[[iF]] == 2 i2 && hg2[[iF]] == 2 i2 &&
  hg[[iR]] == z2 && hg2[[iR]] == z2 && hg[[8]] == i2/2 && hg2[[8]] == -i2/2 &&
  Sum[hg[[a]].hg[[a]], {a, 3}] == 3/4 i2 && Sum[hg2[[a]].hg2[[a]], {a, 3}] == 3/4 i2 &&
  closed[sectors[[2]], hb, gens] && closed[sectors[[3]], hb, gens];
(* the conjugate doublet i tau^2 Phi'^*: generators -(i tau^2) G^* (i tau^2)^T *)
ok8b = And @@ Table[-eps.Conjugate[hg2[[a]]].Transpose[eps] == sig[a]/2, {a, 3}] &&
  -eps.Conjugate[hg2[[iY]]].Transpose[eps] == i2/2;
(* the bidoublet: components (up chi, up chi', down chi, down chi') *)
bb = Flatten[Table[Module[{m = ConstantArray[0, {8, 8}]},
      m[[All, 4 a + 1 ;; 4 a + 4]] = kp[[s + 1]]/Sqrt[Total[Abs[Flatten[kp[[s + 1]]]]^2]]; Flatten[m]],
     {s, 0, 1}, {a, 0, 1}], 1];
bg = gOn[csectors[[1]], bb, #] & /@ gensAll;
ok8c = And @@ Table[bg[[a]] == KroneckerProduct[sig[a]/2, i2] && bg[[iTR[[a]]]] == KroneckerProduct[i2, sig[a]/2],
    {a, 3}] && bg[[iY]] == KroneckerProduct[i2, sig[3]/2] && bg[[iF]] == 2 i4 && bg[[iR]] == ConstantArray[0, {4, 4}] &&
  closed[csectors[[1]], bb, gensAll];
check["WQN-8", "on Phi_SH: tau^a/2, Y = 1/2, Q = diag(1, 0); on Phi'_SH: tau^a/2, Y = -1/2, Q = diag(0, -1); F_tot = 2, \
R = 0, T^3_R = +-1/2, Casimir 3/4, closure; i tau^2 Phi'^* has tau^a/2 and Y = 1/2; the bidoublet has (tau^a/2) (x) 1 \
and 1 (x) (tau^a/2): M_SH -> L M_SH R^T", ok8a && ok8b && ok8c,
  "G_Y = " <> ToString[hg[[iY]], InputForm] <> " and " <> ToString[hg2[[iY]], InputForm]];

(* ------------------------------------------------------------------ WQN-9: the invariants, infinitesimally *)
(* the pairing term tr(M^dag M) is invariant when the generators on the components are Hermitian; the change of
   det M under m -> m + i eps G m is i eps q det M with q = (grad det).(G m) / det: 0 for T^a, T^a_R, R and Y, 4 for
   F_tot *)
mm = {m1, m2, m3, m4}; detM = m1 m4 - m2 m3;
charge[g_] := Simplify[(D[detM, {mm}].(g.mm))/detM];
okH = And @@ (# == ConjugateTranspose[#] & /@ Join[hg, hg2, bg]);
qdet = charge /@ bg[[{1, 2, 3, iTR[[1]], iTR[[2]], iTR[[3]], iR, iY, iF}]];
check["WQN-9", "the generators on Phi_SH, Phi'_SH and M_SH are Hermitian, so the pairing term (4.2) is invariant; det M_SH \
has the charges 0 under T^a, T^a_R, R and Y and 4 under F_tot (the determinant term breaks only U(1)_F_tot)",
  okH && qdet == {0, 0, 0, 0, 0, 0, 0, 0, 4}, "charges of det M_SH " <> ToString[qdet]];

(* ------------------------------------------------------------------ WQN-10: the channels *)
su2Content[cas_] := Module[{dims = {}},
  Do[With[{d = Sqrt[1 + 4 t[[1]]]}, dims = Join[dims, ConstantArray[d, t[[2]]/d]]], {t, Tally[Sort[cas]]}];
  StringRiffle[ToString /@ Sort[dims], " + "]];
(* the multiplets of SU(2)_L x SU(2)_R: the Casimir of SU(2)_R restricted to each eigenspace of that of SU(2)_L *)
su2x2Content[cl_, cr_] := Module[{mult = {}},
  Do[Module[{bsp = Transpose[NullSpace[cl - lam IdentityMatrix[Length[cl]]]], sub, dl = Sqrt[1 + 4 lam]},
     sub = PseudoInverse[bsp].cr.bsp;
     Do[With[{dr = Sqrt[1 + 4 t[[1]]]}, mult = Join[mult, ConstantArray[{dl, dr}, t[[2]]/(dl dr)]]],
      {t, Tally[Simplify[Eigenvalues[sub]]]}]],
   {lam, DeleteDuplicates[Simplify[Eigenvalues[cl]]]}];
  StringRiffle["(" <> ToString[#[[1]]] <> "," <> ToString[#[[2]]] <> ")" & /@ Sort[mult], " + "]];
fracStr[x_] := ToString[x, InputForm];
fracsStr[v_] := StringRiffle[fracStr /@ Reverse[Sort[v]], "; "];
(* the parts of parity +1 and -1 of the space b of a sector, with the matrices of the generators glist on each *)
channelsOf[sec_, b_, glist_] := Module[{pr, res, out = {}},
  {pr, res} = restrict[parityOp[sec], b];
  closureAll = closureAll && allZero[res];
  Do[Module[{subb = NullSpace[pr - sign IdentityMatrix[Length[pr]]].b, lx},
     lx = Table[Module[{c, r}, {c, r} = restrict[chargeOp[sec, glist[[i]]], subb];
        closureAll = closureAll && allZero[r]; c], {i, Length[glist]}];
     AppendTo[out, <|"sector" -> secName[sec], "parity" -> sign, "dim" -> Length[subb], "lx" -> lx|>]],
   {sign, {1, -1}}];
  out];
closureAll = True;
channels = Flatten[Table[channelsOf[sectors[[k]], spaces[[k]], gens], {k, 12}]];
channels = Append[#, <|"su2" -> su2Content[Simplify[Eigenvalues[Sum[#["lx"][[a]].#["lx"][[a]], {a, 3}]]]],
      "Y" -> fracsStr[Simplify[Eigenvalues[#["lx"][[iY]]]]], "qv" -> Simplify[Eigenvalues[#["lx"][[iQ]]]],
      "F" -> fracsStr[Simplify[Eigenvalues[#["lx"][[iF]]]]], "R" -> fracsStr[Simplify[Eigenvalues[#["lx"][[iR]]]]],
      "T3R" -> fracsStr[Simplify[Eigenvalues[#["lx"][[8]]]]]|>] & /@ channels;
channels = Append[#, "Q" -> fracsStr[#["qv"]]] & /@ channels;
cchannels = Flatten[Table[channelsOf[csectors[[k]], cspaces[[k]], gensAll], {k, 4}]];
cchannels = Append[#, "su2x2" -> su2x2Content[Sum[#["lx"][[a]].#["lx"][[a]], {a, 3}],
       Sum[#["lx"][[iTR[[a]]]].#["lx"][[iTR[[a]]]], {a, 3}]]] & /@ cchannels;
(* every field as a string: "-" would otherwise not stay "-" *)
readTable[file_] := Module[{t = Import[file, "CSV", "Numeric" -> False]}, {First[t], Rest[t]}];
{hdr, rows} = readTable[channelsFile];
{chdr, crows} = readTable[custodialFile];
col[h_, name_] := Position[h, name][[1, 1]];
(* the parity is written as +1 or -1 (the importer would read a lone "-" as a missing value) *)
parityOf[x_] := Which[MemberQ[{"+1", "1", 1}, x], 1, MemberQ[{"-1", -1}, x], -1, True, 0];
rowOf[h_, rs_, sec_, par_] := SelectFirst[rs, #[[col[h, "sector"]]] == sec && parityOf[#[[col[h, "parity"]]]] == par &,
   Missing[]];
str[x_] := If[StringQ[x], x, ToString[x]];
mism = Select[channels, Module[{r = rowOf[hdr, rows, #["sector"], #["parity"]]},
     MissingQ[r] || str[r[[col[hdr, "dimension"]]]] != ToString[#["dim"]] ||
      str[r[[col[hdr, "SU2_multiplets"]]]] != #["su2"] ||
      AnyTrue[{"Y", "Q", "F", "R", "T3R"}, Function[k, str[r[[col[hdr, k]]]] != #[k]]]] &];
cmism = Select[cchannels, Module[{r = rowOf[chdr, crows, #["sector"], #["parity"]]},
     MissingQ[r] || str[r[[col[chdr, "dimension"]]]] != ToString[#["dim"]] ||
      str[r[[col[chdr, "SU2L_x_SU2R_multiplets"]]]] != #["su2x2"]] &];
check["WQN-10", "the 24 channels (parity split, closure, SU(2) multiplets, Y, Q, F_tot, R, T^3_R) and the 8 custodial \
channels (SU(2)_L x SU(2)_R multiplets) agree with the tables of the Rust program",
  closureAll && mism === {} && cmism === {} && Length[rows] == 24 && Length[crows] == 8,
  If[mism === {} && cmism === {}, "all 24 and all 8 agree; custodial: " <>
    StringRiffle[#["sector"] <> " " <> #["su2x2"] & /@ cchannels[[1 ;; ;; 2]], "; "],
   "differ: " <> StringRiffle[#["sector"] <> " (" <> ToString[#["parity"]] <> ")" & /@ Join[mism, cmism], "; "]]];
noNeutral = Select[channels, MemberQ[{"Psi Psi", "chi chi", "chi' chi'", "chi chi'"}, #["sector"]] &];
minQ = Min[Abs[Flatten[#["qv"] & /@ noNeutral]]];
nNeutral = Count[#["qv"], 0] & /@ Select[channels, StringFreeQ[#["sector"], "^dag"] &];
check["WQN-10e", "no pair of two particles of one field and no chi chi' pair is neutral; the pair channels have exactly \
one neutral component each in Psi chi and Psi chi' and none elsewhere", minQ == 1/2 &&
  nNeutral == {0, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0}, "smallest absolute value of Q " <> fracStr[minQ] <>
  "; neutral components of the 12 pair channels " <> ToString[nNeutral]];

(* ------------------------------------------------------------------ WQN-11: the breaking *)
(* the real coefficients c with (sum_A c_A G_A) w = 0 for all pairs (G, w) *)
stab[conds_] := Module[{n = Length[conds[[1, 1]]], cs, eqs},
  cs = Array[cc, n];
  eqs = Flatten[Table[ComplexExpand[{Re[#], Im[#]} & /@ (Sum[cs[[a]] p[[1, a]], {a, n}].p[[2]])], {p, conds}]];
  NullSpace[Normal[CoefficientArrays[eqs, cs][[2]]]]];
alongQ[st_] := Length[st] == 1 && Normalize[st[[1]]] Sign[Last[st[[1]]]] == {0, 0, 1, 1}/Sqrt[2];
phi0 = {0, 1}; phi0p = {1, 0};
st0 = stab[{{hg[[1 ;; 4]], phi0}}];
stN = stab[{{hg[[1 ;; 4]], {0, 1 + 2 I}}, {hg2[[1 ;; 4]], {3 - I, 0}}}];   (* the neutral vacuum, unequal lengths *)
(* the bidoublet with the eight generators (T^1, T^2, T^3, T^1_R, T^2_R, T^3_R, R, F_tot) *)
g8 = bg[[{1, 2, 3, iTR[[1]], iTR[[2]], iTR[[3]], iR, iF}]];
wIt = {0, 1, -1, 0};                                                        (* i tau^2 *)
st8 = stab[{{g8, wIt}}];
want8 = {{1, 0, 0, 1, 0, 0, 0, 0}, {0, 1, 0, 0, 1, 0, 0, 0}, {0, 0, 1, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}};
gauge8 = {{1, 0, 0, 0, 0, 0, 0, 0}, {0, 1, 0, 0, 0, 0, 0, 0}, {0, 0, 1, 0, 0, 0, 0, 0}, {0, 0, 0, 0, 0, 1, 1/4, 0}};
stG = stab[{{bg[[{1, 2, 3, iY}]], wIt}}];
stPh = stab[{{g8, {0, (3 + 4 I)/5, I, 0}}}];                                (* equal lengths, other phases *)
stU = stab[{{g8, {0, 2, 1 + I, 0}}}];                                       (* unequal lengths *)
ok11a = alongQ[st0] && alongQ[stN] && Length[st8] == 4 && MatrixRank[Join[st8, want8]] == 4 && alongQ[stG] &&
  MatrixRank[Join[st8, gauge8]] == 7 && MatrixRank[Join[st8, gauge8, {UnitVector[8, 8]}]] == 8 && Length[stPh] == 4 &&
  Length[stU] == 2 && MatrixRank[Join[stU, {{0, 0, 1, 0, 0, 1, 0, 0}, {0, 0, 0, 0, 0, 0, 1, 0}}]] == 2;
check["WQN-11", "the stabilizer of (0, phi) and of the neutral vacuum of H5 in SU(2)_L x U(1)_Y is spanned by Q; that of \
i tau^2 among the eight generators is spanned by T^a + T^a_R and R, three gauged generators and F_tot are broken; other \
phases with equal lengths keep four generators, unequal lengths only T^3 + T^3_R and R", ok11a,
  "stabilizer of i tau^2 " <> ToString[st8, InputForm]];
(* a fixed generic vector with exact entries for each channel, and a vector of its neutral part *)
generic[n_] := Table[(j + 1) + I (2 j - 1), {j, n}];
res11 = Table[Module[{w = generic[ch["dim"]], st, nz, stn, allNeutral, g4 = ch["lx"][[1 ;; 4]]},
    st = stab[{{hg[[1 ;; 4]], phi0}, {hg2[[1 ;; 4]], phi0p}, {g4, w}}];
    allNeutral = AllTrue[ch["qv"], # == 0 &];
    nz = NullSpace[ch["lx"][[iQ]]];
    stn = If[nz === {}, None,
      stab[{{hg[[1 ;; 4]], phi0}, {hg2[[1 ;; 4]], phi0p}, {g4, Total[MapIndexed[(#2[[1]] + 2) #1 &, nz]]}}]];
    {Length[st] == If[allNeutral, 1, 0], stn === None || alongQ[stn]}],
   {ch, channels}];
check["WQN-11f", "a generic second condensate with a charged component, added to the neutral vacuum of H5, breaks Q; \
one in the neutral part of a channel keeps exactly Q", And @@ Flatten[res11],
  ToString[Count[res11, {True, True}]] <> " of " <> ToString[Length[res11]] <> " channels as expected"];

(* ------------------------------------------------------------------ WQN-12: the mass matrix *)
phiv = {0, ph}; phivp = {php, 0};                      (* the lengths phi and phi' of the two condensates *)
ga = {g, g, g, gp};
m2 = Table[Simplify[ga[[a]] ga[[b]] ComplexExpand[Re[Conjugate[phiv].acomm[hg[[a]], hg[[b]]].phiv +
        Conjugate[phivp].acomm[hg2[[a]], hg2[[b]]].phivp]], Assumptions -> {g > 0, gp > 0, ph > 0, php > 0}],
   {a, 4}, {b, 4}];
ev12 = Simplify[Eigenvalues[m2], Assumptions -> {g > 0, gp > 0, ph > 0, php > 0}];
vv = 2 (ph^2 + php^2);                                  (* v_SH^2 *)
wantEv = {0, g^2 vv/4, g^2 vv/4, (g^2 + gp^2) vv/4};
nul = NullSpace[m2];
rho12 = Simplify[(g^2 vv/4)/((g^2 + gp^2) vv/4 g^2/(g^2 + gp^2))];
ok12 = Sort[Expand[ev12]] === Sort[Expand[wantEv]] && Length[nul] == 1 &&
  Simplify[nul[[1]] - (nul[[1, 4]]/g) {0, 0, gp, g}] == {0, 0, 0, 0} && rho12 === 1;
check["WQN-12", "the mass matrix of (W^1, W^2, W^3, B) from Phi_SH = (0, phi) and Phi'_SH = (phi', 0): eigenvalues 0, \
g^2 v_SH^2/4 (twice), (g^2 + g'^2) v_SH^2/4 with v_SH^2 = 2 (phi^2 + phi'^2) for any phi, phi', the massless vector \
along (0, 0, g', g), and rho = 1 (symbolic couplings and lengths)", ok12,
  "eigenvalues " <> ToString[ev12, InputForm]];

(* ------------------------------------------------------------------ the report *)
out = StringJoin[
  "sh_quantum.wls: the exact computation of the checks of sh_quantum, second version (Revision_SM_Higgs, PROJECT \
spacetimes)\n",
  "Each line: PASS or FAIL, the identifier (W and the family of checks of the Rust program to which its statements belong), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic: no tolerance.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
