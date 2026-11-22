#!/usr/bin/env wolframscript
(* ================================================================================================
   sh_examples.wls -- the third computation of the explicit examples of the objects that the author
   named on 22 November 2026 (task record C27; Revision_SM_Higgs, PROJECT spacetimes): a Moufang loop,
   the Lorentz super-loop, a Dirac spinor, an isovector, an isodoublet, an isospin triplet, an isodoublet
   of Dirac spinors, the eight spinor charges and their spin-zero superfluid.

   It derives in exact arithmetic the general statements behind the examples (the Moufang identities and
   the multiplicative norm for general bi-octonions, the bracket of two spinor charges, the isovector
   law of the pair matrices for a general SU(2) matrix, the factor one half of the non-unitary state)
   and compares every exact value of the Rust program sh_examples with its own computation; the numbers
   of the superfluid are recomputed with 30 digits.  The identifier of a check is WEX and its number;
   each entry names the checks of the Rust program (EX) and of python/sh_examples.py (YEX) that test the
   same statements.

     WEX-1  (EX-1, YEX-1) the loop of the 16 elements: its product table against the Rust table, the
            Moufang identities on all 4096 triples, the counts of non-associative triples and
            non-commuting pairs;
     WEX-2  (EX-2, EX-3, YEX-2, YEX-3) the Moufang identities and N(x y) = N(x) N(y) for general
            bi-octonions with symbolic coefficients (so every set of bi-octonions of norm 1 closed under
            the product, the unit octonions and the Lorentz super-loop, is a Moufang loop), and the
            values of the examples of the unit octonions and of the Lorentz super-loop;
     WEX-3  (EX-4, YEX-4) the Dirac spinor: for the boost along the 3-axis with rapidity eta the spinor
            (e^(-eta/2), 0, e^(eta/2), 0) solves the Dirac equation with p = (cosh eta, 0, 0, sinh eta)
            (symbolic), and the values of the example (eta = 2 log 2);
     WEX-4  (EX-5 to EX-8, YEX-5 to YEX-7) isospin: for a general SU(2) matrix U = M(v) (v a real
            quaternion of norm 1), R(U) is orthogonal and U^T (E tau_a) U = sum_b R_ab (E tau_b)
            (symbolic); the values of the isovector, the isodoublet, the triplet and the isodoublet of
            Dirac spinors;
     WEX-5  (EX-9, YEX-8) the eight spinor charges: the bracket [b e4, b' e4] = bbar b' - b'bar b for
            general bi-quaternions b, b' (symbolic), the tables of the fields and the brackets of the Rust
            program, Jac(e4, e5, e6) = -12 e7;
     WEX-6  (EX-10, YEX-9) the superfluid: the grand potential of the state zeta = 1 at |d| = Delta/2^(1/2)
            is exactly half that of the unitary state at the gap Delta (symbolic in the integrand), the
            critical coupling in closed form, the gap and the grand potential of the vacuum at G = 30 and
            of the test point (G = 12, mu = 1/2, m = 1/5) with 30 digits, against the Rust values.

   USAGE
     wolframscript -file sh_examples.wls <sh_examples.csv> <sh_examples_loop16.csv> <sh_examples_charges.csv>
         <sh_examples_brackets.csv> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 5, Print["usage: sh_examples.wls <values> <loop16> <charges> <brackets> <report>"]; Exit[2]];
{valFile, loopFile, chFile, brFile, reportFile} = args[[1 ;; 5]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 8], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];

(* the CSV files are read as strings; a field that the importer turns into Missing is read as "NaN" *)
readTable[file_] := Module[{t = Import[file, "CSV", "Numeric" -> False] /. _Missing -> "NaN"}, {First[t], Rest[t]}];
{vHead, vRows} = readTable[valFile];
val[name_] := Select[vRows, #[[1]] == name &][[1, 2]];
(* a value of the Rust program as an expression: the units e0..e7 become ee[0]..ee[7], the generators J1..K3 become
   jj[1]..kk[3], the imaginary unit i becomes I, and parentheses become lists *)
parse[s_String] := ToExpression[StringReplace[StringReplace[StringReplace[s,
      RegularExpression["(?<![A-Za-z0-9_])i(?![A-Za-z0-9_])"] -> "I"],
     {RegularExpression["e(\\d)"] -> "ee[$1]", RegularExpression["J(\\d)"] -> "jj[$1]", RegularExpression["K(\\d)"] -> "kk[$1]"}],
    {"(" -> "{", ")" -> "}"}]];

(* ------------------------------------------------------------------ the algebra, written here from its definition *)
qmul[{a0_, a1_, a2_, a3_}, {b0_, b1_, b2_, b3_}] := {a0 b0 - a1 b1 - a2 b2 - a3 b3, a0 b1 + a1 b0 + a2 b3 - a3 b2,
   a0 b2 - a1 b3 + a2 b0 + a3 b1, a0 b3 + a1 b2 - a2 b1 + a3 b0};
qbar[{a0_, a1_, a2_, a3_}] := {a0, -a1, -a2, -a3};
omul[x_, y_] := Expand[Join[qmul[x[[1 ;; 4]], y[[1 ;; 4]]] - qmul[qbar[y[[5 ;; 8]]], x[[5 ;; 8]]],
    qmul[y[[5 ;; 8]], x[[1 ;; 4]]] + qmul[x[[5 ;; 8]], qbar[y[[1 ;; 4]]]]]];
obar[x_] := Join[{x[[1]]}, -Rest[x]];
onorm[x_] := Expand[x . x];
unit[p_] := UnitVector[8, p + 1];
oexpr[x_] := Expand[x . Table[ee[p], {p, 0, 7}]];
comm[x_, y_] := omul[x, y] - omul[y, x];
jac[x_, y_, z_] := Expand[comm[comm[x, y], z] + comm[comm[y, z], x] + comm[comm[z, x], y]];
mouf[x_, y_, z_] := {omul[z, omul[x, omul[z, y]]] - omul[omul[omul[z, x], z], y],
   omul[omul[omul[y, z], x], z] - omul[y, omul[z, omul[x, z]]],
   omul[omul[z, x], omul[y, z]] - omul[omul[z, omul[x, y]], z]};
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
eps = {{0, 1}, {-1, 0}};
mll[b_] := b[[1]] sig[[1]] - I (b[[2]] sig[[2]] + b[[3]] sig[[3]] + b[[4]] sig[[4]]);
lorentz[m_] := Table[ComplexExpand[Re[Tr[sig[[p]] . m . sig[[q]] . ConjugateTranspose[m]]/2]], {p, 4}, {q, 4}];
same[a_, b_] := TrueQ[Simplify[a - b] === 0] || TrueQ[Simplify[Flatten[{a - b}]] === ConstantArray[0, Length[Flatten[{a - b}]]]];

(* ------------------------------------------------------------------ WEX-1: the loop of 16 elements *)
elems = Flatten[Table[s unit[p], {s, {1, -1}}, {p, 0, 7}], 1];
pos[x_] := First[FirstPosition[elems, x]];
pr = Table[pos[omul[elems[[i]], elems[[j]]]], {i, 16}, {j, 16}];
{lHead, lRows} = readTable[loopFile];
tabOk = Length[lRows] == 256 && AllTrue[lRows, Expand[oexpr[omul[parse[#[[1]]] /. ee[p_] :> unit[p], parse[#[[2]]] /. ee[p_] :> unit[p]]]
       - parse[#[[3]]]] === 0 &];
mfail = 0; nonassoc = 0; noncomm = 0;
Do[If[pr[[x, y]] != pr[[y, x]], noncomm++];
  Do[If[! (pr[[z, pr[[x, pr[[z, y]]]]]] == pr[[pr[[pr[[z, x]], z]], y]] && pr[[pr[[pr[[y, z]], x]], z]] == pr[[y, pr[[z, pr[[x, z]]]]]] &&
       pr[[pr[[z, x]], pr[[y, z]]]] == pr[[pr[[z, pr[[x, y]]]], z]]), mfail++];
   If[pr[[pr[[x, y]], z]] != pr[[x, pr[[y, z]]]], nonassoc++], {z, 16}], {x, 16}, {y, 16}];
check["WEX-1", "the loop of the 16 elements +-e_p: the 256 products of the Rust table agree with this product; the Moufang \
identities hold on all 4096 triples; 1344 triples are not associative and 168 pairs do not commute",
  tabOk && mfail == 0 && nonassoc == 1344 && noncomm == 168,
  "table " <> ToString[tabOk] <> ", Moufang failures " <> ToString[mfail] <> ", non-associative " <> ToString[nonassoc] <>
   ", non-commuting " <> ToString[noncomm]];

(* ------------------------------------------------------------------ WEX-2: Moufang and the norm in general; the examples *)
xs = Array[xa, 8]; ys = Array[ya, 8]; zs = Array[za, 8];
moufGeneral = AllTrue[Flatten[mouf[xs, ys, zs]], # === 0 &];
normGeneral = Expand[onorm[omul[xs, ys]] - onorm[xs] onorm[ys]] === 0;
x = {3/5, 4/5, 0, 0, 0, 0, 0, 0}; y = {0, 0, 3/5, 0, 4/5, 0, 0, 0}; z = {1/3, 0, 0, 2/3, 0, 0, 2/3, 0};
ex2 = AllTrue[{x, y, z}, onorm[#] == 1 &] && same[parse[val["ex2_xy"]], oexpr[omul[x, y]]] &&
   same[parse[val["ex2_(xy)z"]], oexpr[omul[omul[x, y], z]]] && same[parse[val["ex2_x(yz)"]], oexpr[omul[x, omul[y, z]]]] &&
   same[parse[val["ex2_assoc_x_y_z"]], oexpr[omul[omul[x, y], z] - omul[x, omul[y, z]]]] && same[parse[val["ex2_xbar"]], oexpr[obar[x]]];
gr = {3/5, 4/5, 0, 0, 0, 0, 0, 0}; gb = {5/4, 0, 0, 3 I/4, 0, 0, 0, 0}; sv = {3/5, 0, 0, 0, 4/5, 0, 0, 0};
w = omul[gb, sv]; xsol = omul[obar[w], unit[5]];
mr = mll[gr[[1 ;; 4]]]; mb = mll[gb[[1 ;; 4]]];
ex3 = AllTrue[{gr, gb, sv, w}, onorm[#] == 1 &] && omul[w, xsol] == unit[5] && Det[mr] == 1 && Det[mb] == 1 &&
   same[parse[val["ex3_M_g_rot"]], mr] && same[parse[val["ex3_M_g_boost"]], mb] &&
   same[parse[val["ex3_L_g_boost"]], lorentz[mb]] && same[parse[val["ex3_L_g_rot"]], lorentz[mr]] &&
   same[parse[val["ex3_w"]], oexpr[w]] && same[parse[val["ex3_x_solving_wx_e5"]], oexpr[xsol]] &&
   same[parse[val["ex3_assoc_grot_gboost_s"]], oexpr[omul[omul[gr, gb], sv] - omul[gr, omul[gb, sv]]]] &&
   same[parse[val["ex3_conj_g_e4_gbar"]], oexpr[omul[omul[gb, unit[4]], obar[gb]]]] &&
   oexpr[omul[omul[gb, unit[4]], obar[gb]]] === Expand[oexpr[Join[{0, 0, 0, 0}, omul[gb, gb][[1 ;; 4]]]]] &&
   Module[{pt = {2, I, 3 I, 5 I, 0, 0, 0, 0}}, omul[omul[unit[4], pt], -unit[4]] == obar[pt]];
check["WEX-2", "the three Moufang identities and N(x y) = N(x) N(y) hold for general bi-octonions x, y, z with symbolic \
coefficients (so the unit octonions and the Lorentz super-loop, closed under the product, are Moufang loops); the values of \
the examples of EX-2 and EX-3 (the unit octonions; g_rot, g_boost, s, w, wbar e5, the matrices, the Lorentz matrices, the \
associator, Eq. (17) and the parity) agree with the Rust program exactly",
  moufGeneral && normGeneral && ex2 && ex3,
  "Moufang general " <> ToString[moufGeneral] <> ", norm general " <> ToString[normGeneral] <> ", examples " <> ToString[ex2 && ex3]];

(* ------------------------------------------------------------------ WEX-3: the Dirac spinor *)
z2 = ConstantArray[0, {2, 2}];
gam = Join[{ArrayFlatten[{{z2, sig[[1]]}, {sig[[1]], z2}}]}, Table[ArrayFlatten[{{z2, sig[[j]]}, {-sig[[j]], z2}}], {j, 2, 4}]];
gch = I gam[[1]] . gam[[2]] . gam[[3]] . gam[[4]];
slash[p_] := p[[1]] gam[[1]] - p[[2]] gam[[2]] - p[[3]] gam[[3]] - p[[4]] gam[[4]];
uEta = {Exp[-eta/2], 0, Exp[eta/2], 0};
dGeneral = Simplify[(slash[{Cosh[eta], 0, 0, Sinh[eta]}] - IdentityMatrix[4]) . uEta, eta \[Element] Reals] === {0, 0, 0, 0};
uu = uEta /. eta -> 2 Log[2]; pp = {17/8, 0, 0, 15/8};
sg = ArrayFlatten[{{Inverse[ConjugateTranspose[mb]], z2}, {z2, mb}}];
ex4 = Simplify[{Cosh[2 Log[2]], Sinh[2 Log[2]]}] == {17/8, 15/8} && (slash[pp] - IdentityMatrix[4]) . uu == {0, 0, 0, 0} &&
   sg . {1, 0, 1, 0} == uu && Conjugate[uu] . gam[[1]] . uu == 2 && Conjugate[uu] . uu == 17/4 &&
   same[parse[val["ex4_u_up"]], uu] && same[parse[val["ex4_u_down"]], sg . {0, 1, 0, 1}] && same[parse[val["ex4_gamma_ch_u_up"]], gch . uu] &&
   AllTrue[Range[4], Simplify[Inverse[sg] . gam[[#]] . sg - Sum[lorentz[mb][[#, nu]] gam[[nu]], {nu, 4}]] == ConstantArray[0, {4, 4}] &];
check["WEX-3", "the Dirac spinor: (e^(-eta/2), 0, e^(eta/2), 0) solves (gamma.p - 1) u = 0 for p = (cosh eta, 0, 0, sinh \
eta) and every real eta (symbolic); at eta = 2 log 2 it is the spinor (1/2, 0, 2, 0) of the Rust program, with p = (17/8, \
0, 0, 15/8), ubar u = 2, u^dag u = 17/4, S^-1 gamma^mu S = L^mu_nu gamma^nu, and the values of gamma^ch u and of the \
spin-down spinor", dGeneral && ex4, "general " <> ToString[dGeneral] <> ", example " <> ToString[ex4]];

(* ------------------------------------------------------------------ WEX-4: isospin *)
(* every SU(2) matrix is M(v) for a real quaternion v of norm 1; for a real v of any norm n = v.v, M(v) is n^(1/2) times
   an SU(2) matrix, R(M(v)) is n times its rotation, so R^T R = n^2 and the law of the pair matrices are polynomial
   identities in v that need no constraint *)
vv = {v0, v1, v2, v3};
uGen = mll[vv];
rOf[u_] := Table[Tr[sig[[k + 1]] . u . sig[[l + 1]] . ConjugateTranspose[u]]/2, {k, 3}, {l, 3}];
rG = ComplexExpand[rOf[uGen]];
orthoGeneral = Expand[Transpose[rG] . rG - (vv . vv)^2 IdentityMatrix[3]] === ConstantArray[0, {3, 3}];
et = Table[eps . sig[[k + 1]], {k, 3}];
pairGeneral = AllTrue[Range[3], Expand[ComplexExpand[Transpose[uGen] . et[[#]] . uGen - Sum[rG[[#, l]] et[[l]], {l, 3}]]] ===
      ConstantArray[0, {2, 2}] &];
uEx = mll[{3/5, 4/5, 0, 0}]; rEx = rOf[uEx];
nvec[v_] := Table[Conjugate[v] . sig[[k + 1]] . v, {k, 3}];
psiUp = KroneckerProduct[uu, {1, 0}] // Flatten;
d8 = KroneckerProduct[slash[pp] - IdentityMatrix[4], IdentityMatrix[2]];
ex5 = same[parse[val["ex5_U"]], uEx] && same[parse[val["ex5_R_of_U"]], rEx] && same[parse[val["ex6_U_psi"]], uEx . {1, 0}] &&
   same[parse[val["ex6_n_U_psi"]], nvec[uEx . {1, 0}]] && nvec[uEx . {1, 0}] == rEx . {0, 0, 1} &&
   Sort[Eigenvalues[Sum[MatrixPower[(KroneckerProduct[sig[[k + 1]], sig[[1]]] + KroneckerProduct[sig[[1]], sig[[k + 1]]])/2, 2], {k, 3}]]] == {0, 2, 2, 2} &&
   AllTrue[et, # == Transpose[#] &] && eps == -Transpose[eps] && same[parse[val["ex7_X_of_U"]], rEx] &&
   same[parse[val["ex7_E_tau1_minus_i_E_tau2"]], et[[1]] - I et[[2]]] && same[parse[val["ex7_E_tau1_plus_i_E_tau2"]], et[[1]] + I et[[2]]] &&
   d8 . psiUp == ConstantArray[0, 8] && same[parse[val["ex8_Psi_up"]], psiUp] &&
   same[parse[val["ex8_(1xU)Psi_up"]], KroneckerProduct[IdentityMatrix[4], uEx] . psiUp] &&
   d8 . (KroneckerProduct[IdentityMatrix[4], uEx] . psiUp) == ConstantArray[0, 8];
check["WEX-4", "isospin: for a general SU(2) matrix U = M(v) (v a real quaternion of norm 1) the matrix R(U) of U tau_a \
U^dag = sum_b R_ba tau_b is orthogonal, and U^T (E tau_a) U = sum_b R_ab (E tau_b) (symbolic, as identities in v); the values of the isovector, the \
isodoublet, the triplet (eigenvalues 0, 2, 2, 2 of T^2) and the isodoublet of Dirac spinors agree with the Rust program",
  orthoGeneral && pairGeneral && ex5, "orthogonal " <> ToString[orthoGeneral] <> ", pair law " <> ToString[pairGeneral] <>
   ", examples " <> ToString[ex5]];

(* ------------------------------------------------------------------ WEX-5: the eight spinor charges *)
bq = Array[bb, 4]; bq2 = Array[cb, 4];
brGeneral = Expand[comm[Join[{0, 0, 0, 0}, bq], Join[{0, 0, 0, 0}, bq2]] - Join[qmul[qbar[bq], bq2] - qmul[qbar[bq2], bq], {0, 0, 0, 0}]] ===
   ConstantArray[0, 8];
names = {"e4", "e5", "e6", "e7", "i e4", "i e5", "i e6", "i e7"};
bk = Table[If[k <= 4, UnitVector[4, k], I UnitVector[4, k - 4]], {k, 8}];
psiP[b_] := -eps . Transpose[mll[b]];
{cHead, cRows} = readTable[chFile];
chOk = Length[cRows] == 8 && AllTrue[Range[8], cRows[[#, 1]] == names[[#]] && same[parse[cRows[[#, 2]]], mll[bk[[#]]]] &&
       same[parse[cRows[[#, 3]]], psiP[bk[[#]]]] && same[parse[cRows[[#, 4]]], psiP[bk[[#]]][[All, 1]]] &&
       same[parse[cRows[[#, 5]]], psiP[bk[[#]]][[All, 2]]] &];
{bHead, bRows} = readTable[brFile];
sk = Table[Join[{0, 0, 0, 0}, bk[[k]]], {k, 8}];
jkExpr[v_] := Expand[Sum[2 ComplexExpand[Re[v[[j + 1]]]] jj[j] + 2 ComplexExpand[Im[v[[j + 1]]]] kk[j], {j, 3}]];
brOk = Length[bRows] == 64 && AllTrue[Range[64], Module[{k = Quotient[# - 1, 8] + 1, l = Mod[# - 1, 8] + 1, br},
       br = comm[sk[[k]], sk[[l]]];
       bRows[[#, 1]] == names[[k]] && bRows[[#, 2]] == names[[l]] && same[parse[bRows[[#, 3]]], oexpr[br]] &&
        same[parse[bRows[[#, 4]]], jkExpr[br]]] &];
jacOk = oexpr[jac[unit[4], unit[5], unit[6]]] === -12 ee[7];
check["WEX-5", "the spinor charges: [b e4, b' e4] = bbar b' - b'bar b for general bi-quaternions b, b' (symbolic, Eq. (12)); \
the tables of the fields psi' = E^-1 M(b)^T and of the 64 brackets of the Rust program agree with this computation; \
Jac(e4, e5, e6) = -12 e7", brGeneral && chOk && brOk && jacOk,
  "general " <> ToString[brGeneral] <> ", fields " <> ToString[chOk] <> ", brackets " <> ToString[brOk] <> ", Jacobiator " <> ToString[jacOk]];

(* ------------------------------------------------------------------ WEX-6: the superfluid *)
(* the integrand of the grand potential (27) for one gap g, at the momentum q, measured from the normal state *)
integrand[g_, q_, mu_, m_] := q^2 (Sqrt[(Sqrt[q^2 + m^2] + mu)^2 + g^2] + Sqrt[(Sqrt[q^2 + m^2] - mu)^2 + g^2] -
     (Sqrt[q^2 + m^2] + mu) - Abs[Sqrt[q^2 + m^2] - mu]);
omegaGaps[d2_, gaps_List, gg_, mu_, m_, q_] := d2/gg - Sum[integrand[gp, q, mu, m], {gp, gaps}]/(2 Pi^2);
(* the state zeta = 1 has the gaps 2^(1/2) |d| and 0; at |d| = Delta/2^(1/2) its integrand is half the unitary one *)
halfOk = Simplify[omegaGaps[dl^2/2, {Sqrt[2] dl/Sqrt[2], 0}, gg, mu, m, q] - omegaGaps[dl^2, {dl, dl}, gg, mu, m, q]/2,
    Assumptions -> {dl > 0, gg > 0, mu > 0, m > 0, q > 0}] === 0;
mll2 = 1/5;
i1[mq_] := (Sqrt[1 + mq^2] - mq^2 ArcSinh[1/mq])/2;
jI[mq_] := ((2 + mq^2) Sqrt[1 + mq^2] - mq^4 ArcSinh[1/mq])/8;
gcrit = N[Pi^2/i1[mll2], 30];
gc0 = N[2 Pi^2, 30];
mqv = mm /. FindRoot[i1[mm]/Pi^2 == 1/30, {mm, 1/5, 5}, Method -> "Brent", WorkingPrecision -> 40];
gapVac = Sqrt[mqv^2 - mll2^2];
omVac = gapVac^2/30 - 2/Pi^2 (jI[mqv] - jI[mll2]);
(* the test point: exact inputs; the arguments are raised to 50 digits before the integrals (precision of the integrands) *)
{gt, mut} = {12, 1/2}; qf = Sqrt[mut^2 - mll2^2];
rhsT[g_?NumericQ] := With[{g5 = SetPrecision[g, 50]},
   NIntegrate[q^2 (1/Sqrt[(Sqrt[q^2 + mll2^2] + mut)^2 + g5^2] + 1/Sqrt[(Sqrt[q^2 + mll2^2] - mut)^2 + g5^2]), {q, 0, qf, 1},
     WorkingPrecision -> 40, PrecisionGoal -> 25, MaxRecursion -> 40]/(2 Pi^2)];
gapT = g /. FindRoot[rhsT[g] == 1/gt, {g, 1/20, 1/5}, Method -> "Brent", WorkingPrecision -> 30];
omT[d_?NumericQ] := With[{d5 = SetPrecision[d, 50]}, d5^2/gt - NIntegrate[integrand[d5, q, mut, mll2] 2, {q, 0, qf, 1},
      WorkingPrecision -> 40, PrecisionGoal -> 25, MaxRecursion -> 40]/(2 Pi^2)];
omU = omT[gapT];
rel[a_, s_String] := Abs[a - ToExpression[StringReplace[s, "e" -> "*^"]]]/Abs[a];
devs = {Abs[gcrit - 21286610/10^6]/(5/10^7), Abs[gc0 - 19739209/10^6]/(5/10^7), rel[gapVac, val["ex10_vacuum_G30_gap"]]/10^-9,
   rel[omVac, val["ex10_vacuum_G30_Omega_min"]]/10^-9, rel[gapT, val["ex10_test_gap"]]/10^-9,
   rel[omU, val["ex10_test_Omega_unitary"]]/10^-9, rel[omU/2, val["ex10_test_Omega_zeta1"]]/10^-8,
   rel[gapT/Sqrt[2], val["ex10_test_d_zeta1"]]/10^-7};
check["WEX-6", "the superfluid: the grand potential (27) of the non-unitary state zeta = 1 at |d| = Delta/2^(1/2) is exactly half \
that of the unitary state at the gap Delta (symbolic, in the integrand), so its minimum is half the unitary minimum and lies \
at |d| = Delta/2^(1/2); the critical coupling pi^2/I1(m) (21.286610 for m = 1/5, 2 pi^2 = 19.739209 for m = 0); the gap and \
the grand potential of the vacuum at G = 30 and of the test point (G = 12, mu = 1/2, m = 1/5) with 30 digits, against the \
Rust values (largest deviation in units of the tolerances: 5e-7 absolute for the couplings, 1e-9 relative for the gaps \
and the unitary grand potentials, 1e-8 for the minimum of the state zeta = 1 and 1e-7 for its location, which the Rust \
program finds by golden sections, accurate to about the square root of the precision of the numbers)",
  halfOk && Max[devs] < 1, "half " <> ToString[halfOk] <> ", the deviations in units of their tolerances (the couplings at \
m = 1/5 and 0, the vacuum gap and grand potential, the gap, the unitary and the zeta = 1 grand potentials, the location of \
the zeta = 1 minimum) " <> ToString[Map[NumberForm[N[#], 2] &, devs]] <> "; gap " <> ToString[NumberForm[N[gapT, 15], 15]] <>
   ", Omega " <> ToString[CForm[N[omU, 12]]] <> ", vacuum gap " <> ToString[NumberForm[N[gapVac, 15], 15]]];

out = StringJoin[
  "sh_examples.wls: the exact derivations and the third computation of the explicit examples (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WEX and a number), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic, and 30 to 40 digits for the numbers of the superfluid.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
