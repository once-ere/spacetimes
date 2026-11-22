#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   sh_det.wls      (PROJECT spacetimes, folder Revision_SM_Higgs)

   PURPOSE
   The exact part of stage 7 of the revision of the hypotheses, second version: the determinant
   interaction -G^det_SH (eps_st Pi^s_SH Pi'^t_SH + h.c.) that the author chose on 22 November 2026
   for the Goldstone boson of the particle number (decision D32 (c) of the task record), and the mass
   that it gives that boson.  The algebra of the term, its mean field in the method of Hubbard and
   Stratonovich, the effective Lagrangian with the relation of Dashen (of Gell-Mann, Oakes and Renner),
   and the closed forms of the method HS (0, 0) are derived here in exact arithmetic or with 30 digits
   and compared with the data of the Rust program sh_det.  The identifier of a check is WDT and its
   number in this script; each entry names, in parentheses, the checks of the Rust program sh_det and
   of the Python program python/sh_det.py that test the same statements in floating point.

     WDT-1  (Rust DT-1b, DT-1c; Python none) the algebra of the term: det(L M R^T) = det M for L, R in
            SU(2) (the bidoublet M -> L M R^T); a rotation by the particle number multiplies every pair
            field by e^(2 i theta) and det M by e^(4 i theta), so theta = pi/4 changes its sign; and
            det M = H2^dag H1 with H1 = Phi = (Phi^u, Phi^d) and H2 = i tau^2 (Phi')^* (symbolic);
     WDT-2  (Rust DT-3b; Python YDT-4) the mean field of HS: with the pair amplitudes p of the four
            channels, the interaction energy -G sum |p|^2 - G^det (p_u p'_d - p_d p'_u + c.c.) gives the
            pair fields Delta_X = -dV/dp_X^*; at the neutral vacuum p_d = phi, p'_u = -phi the gap
            equation has the coupling G + G^det, and the linear map of the four neutral real coordinates
            has the eigenvalues G + G^det (common amplitude, relative phase) and G - G^det (relative
            amplitude, common phase); the charged pair (p_u and the conjugate of p'_d) has G + G^det and G - G^det; the
            quadratic form of the four pair operators and their adjoints has the eigenvalues G +- G^det,
            each twice (symbolic);
     WDT-3  (Rust DT-7a, DT-7b; Python YDT-3) the effective Lagrangian Z tr(dM^dag dM) + G^det (det M + c.c.)
            at M = phi e^(i theta) i tau^2: the common phase theta has the squared mass 2 G^det / Z; a
            chemical potential mu of the particle number (each pair field has the charge 2) gives the
            susceptibility chi^FF = 16 Z phi^2, and the energy of the term is E^det = -2 G^det phi^2, so
            m^2 chi^FF = -16 E^det, the relation of Dashen (symbolic);
     WDT-4  (Rust DT-6, DT-7a; Python YDT-4) HS (0, 0): the identity E/(E^2 - w^2/4) - 1/E =
            (w^2/4)/(E (E^2 - w^2/4)) (symbolic); with the gap equation (G + G^det) S_ph(0) = 1 the
            pseudo-Goldstone boson is the root of (w^2/4) K(w) = 2 G^det/(G^2 - G^det^2), K(w) =
            (1/4 pi^2) int q^2 dq/(E (E^2 - w^2/4)); with chi^FF = d_0^2 J_3/pi^2 (J_3 = int q^2/E^3, the
            closed form that the Rust check DT-4d confirms on the grid) the mass of Dashen is
            m_D^2 = 8 G^det/((G + G^det)^2 K(0)), so m^2/m_D^2 = ((G + G^det)/(G - G^det)) K(0)/K(m)
            exactly; evaluated with 30 digits for G^det/G_SH = 0.01, 0.1, 0.5 against the Rust masses;
     WDT-5  (Rust: the crossings of the scan; Python YDT-6) the continuum limit: the integrals at the
            threshold w = 2 M in closed form, int_0^1 E dq = (sqrt(1 + M^2) + M^2 arsinh(1/M))/2 and
            int_0^1 (q^2 + m^2)/E dq = I1(M) + m^2 arsinh(1/M) (symbolic); the couplings at which the
            pseudo-Goldstone boson and the triplet reach the threshold, and at which the boson reaches
            d_0/2 and d_0, solved with 30 digits, against data/sh_det_values.csv;
     WDT-6  (the columns in GeV of the Rust program; Python none) the conversion to GeV, Lambda_LL = v/v_SH,
            m_G = omega_G Lambda_LL, from the numbers of data/sh_det_states.csv in exact rational
            arithmetic.

   USAGE
     wolframscript -file sh_det.wls <sh_det_states.csv> <sh_det_values.csv> <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 3, Print["usage: sh_det.wls <sh_det_states.csv> <sh_det_values.csv> <report.txt>"]; Exit[2]];
{statesFile, valFile, reportFile} = args[[1 ;; 3]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
sci[x_] := ToString[CForm[N[x, 2]]];
num[s_String] := ToExpression[StringReplace[s, {"e" -> "*^"}]];
(* the CSV files are read as strings; a field that the importer turns into Missing (it does so for "NaN" in some versions
   of the Wolfram Language) is written back as the string "NaN", so that every field is a string *)
readTable[file_] := Module[{t = Import[file, "CSV", "Numeric" -> False] /. _Missing -> "NaN"}, {First[t], Rest[t]}];
col[h_, n_] := Position[h, n][[1, 1]];
zeroQ[x_] := TrueQ[Expand[x] === 0];
(* an identity decided symbolically, or, where Simplify does not reach the canonical form, by its value with 50 digits
   at a generic rational point (a witness; the identities of this script are between elementary functions) *)
(* the witness is evaluated with the rational coordinates of the point raised to 60 digits (fixed-precision arithmetic),
   since N with a precision goal cannot certify the value of an expression that is exactly zero *)
zeroW[x_, pt_] := TrueQ[Simplify[x] === 0] || TrueQ[Abs[x /. (pt /. (s_ -> v_) :> (s -> SetPrecision[v, 60]))] < 10^-40];

(* ------------------------------------------------------------------ WDT-1: the algebra of the term *)
tau = {{{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
eps = {{0, 1}, {-1, 0}};                         (* i tau^2 *)
mm = {{pu, qu}, {pd, qd}};                       (* the bidoublet: columns Phi = (pu, pd), Phi' = (qu, qd) *)
lm = {{la, lb}, {-lbc, lac}};                     (* an element of SU(2): det = la lac + lb lbc = 1 *)
rm = {{ra, rb}, {-rbc, rac}};
detLMR = Det[lm.mm.Transpose[rm]];
okInv = zeroQ[detLMR - (la lac + lb lbc) (ra rac + rb rbc) Det[mm]];
okF = zeroQ[Det[Exp[2 I th] mm] - Exp[4 I th] Det[mm]] && (Exp[4 I Pi/4] == -1);
(* H1 = Phi, H2 = i tau^2 times the conjugate of Phi'; the fields are complex symbols, and the conjugate of the
   conjugate of x is x *)
h1 = {pu, pd}; h2 = eps.{Conjugate[qu], Conjugate[qd]};
okH = zeroQ[Conjugate[h2].h1 /. Conjugate[Conjugate[x_]] :> x // Expand // (# - Det[mm]) &];
check["WDT-1", "the determinant of the bidoublet of the pair fields, det M = Phi^u Phi'^d - Phi^d Phi'^u, obeys \
det(L M R^T) = det L det R det M = det M for L, R in SU(2) (the custodial symmetry and SU(2)_L), is multiplied by \
e^(4 i theta) when every pair field is multiplied by e^(2 i theta) (a rotation by the particle number), so that \
theta = pi/4 changes its sign (the sign convention G^det >= 0), and equals H2^dag H1 with H1 = Phi and \
H2 = i tau^2 (Phi')^* (the term m_12^2 of two Higgs doublets)",
  okInv && okF && okH, "invariance " <> ToString[okInv] <> ", charge 4 " <> ToString[okF] <> ", two doublets " <> ToString[okH]];

(* ------------------------------------------------------------------ WDT-2: the mean field of HS *)
(* the pair amplitudes as real and imaginary parts; V = -G sum |p|^2 - Gd (pu qd - pd qu + c.c.) *)
cvars = {pu, pd, qu, qd};
vpot = -gg Sum[x Conjugate[x], {x, cvars}] - gd (pu qd - pd qu + Conjugate[pu] Conjugate[qd] - Conjugate[pd] Conjugate[qu]);
(* Delta_X = -dV/dp_X^*, with p and p^* independent *)
dvc[x_] := -D[vpot /. Conjugate[y_] :> cc[y], cc[x]];
delta = Table[dvc[x], {x, cvars}] /. cc[y_] :> Conjugate[y];
okDelta = zeroQ[delta[[1]] - (gg pu + gd Conjugate[qd])] && zeroQ[delta[[2]] - (gg pd - gd Conjugate[qu])] &&
  zeroQ[delta[[3]] - (gg qu - gd Conjugate[pd])] && zeroQ[delta[[4]] - (gg qd + gd Conjugate[pu])];
(* the neutral vacuum and its fluctuations: pd = phi + ad + i bd, qu = -(phi + a2 + i b2) *)
subN = {pu -> 0, qd -> 0, pd -> ph + ad + I bd, qu -> -(ph + a2 + I b2)};
(* ComplexExpand takes every symbol as real: phi, the fluctuations and the couplings *)
dd = ComplexExpand[delta[[2]] /. subN];              (* Delta_d *)
du = ComplexExpand[-(delta[[3]] /. subN)];           (* -Delta'_u *)
okGap = zeroQ[(dd /. {ad -> 0, bd -> 0, a2 -> 0, b2 -> 0}) - (gg + gd) ph] && zeroQ[(du /. {ad -> 0, bd -> 0, a2 -> 0, b2 -> 0}) - (gg + gd) ph];
lin = {ComplexExpand[Re[dd]], ComplexExpand[Im[dd]], ComplexExpand[Re[du]], ComplexExpand[Im[du]]} /. {ph -> 0};
jac = Table[D[lin[[i]], {ad, bd, a2, b2}[[j]]], {i, 4}, {j, 4}];
vecs = {{1, 0, 1, 0}, {0, 1, 0, -1}, {1, 0, -1, 0}, {0, 1, 0, 1}};   (* common amp, relative phase, relative amp, common phase *)
want = {gg + gd, gg + gd, gg - gd, gg - gd};
okEig = And @@ Table[zeroQ[Total[(jac.vecs[[k]] - want[[k]] vecs[[k]])^2]], {k, 4}];
(* the charged pair (p_u and the conjugate of p'_d): Delta_u and the complex conjugate of Delta'_d (the couplings are real, so the
   conjugate exchanges every amplitude with its conjugate) *)
dch = {delta[[1]], delta[[4]] /. {qd -> Conjugate[qd], Conjugate[pu] -> pu}};
jch = {{Coefficient[dch[[1]], pu], Coefficient[dch[[1]], Conjugate[qd]]}, {Coefficient[dch[[2]], pu], Coefficient[dch[[2]], Conjugate[qd]]}};
okCh = zeroQ[Det[jch - (gg - gd) IdentityMatrix[2]]] && zeroQ[Det[jch - (gg + gd) IdentityMatrix[2]]] && (jch =!= {{0, 0}, {0, 0}});
(* the quadratic form in u = (Pi^u, Pi^d, Pi'^d^dag, Pi'^u^dag): G 1 + Gd ((0,0,1,0),(0,0,0,-1),(1,0,0,0),(0,-1,0,0)) *)
gform = gg IdentityMatrix[4] + gd {{0, 0, 1, 0}, {0, 0, 0, -1}, {1, 0, 0, 0}, {0, -1, 0, 0}};
okForm = Sort[Eigenvalues[gform] /. {gg -> 7, gd -> 2}] == {5, 5, 9, 9};
check["WDT-2", "the mean field of HS: Delta_X = -dV/dp_X^* gives Delta_u = G p_u + G^det p'_d^*, Delta_d = G p_d - G^det \
p'_u^*, Delta'_u = G p'_u - G^det p_d^*, Delta'_d = G p'_d + G^det p_u^* (the antilinear coupling of the two pair \
fields); at the neutral vacuum p_d = phi, p'_u = -phi the gap equation has the coupling G + G^det; the linear map of \
the neutral coordinates has the eigenvalue G + G^det on the common amplitude and the relative phase and G - G^det on \
the relative amplitude and the common phase; the charged pair has G - G^det and G + G^det; the quadratic form of the \
four pair operators and their adjoints has the eigenvalues G +- G^det (each twice; at G = 7, G^det = 2: 5, 5, 9, 9)",
  okDelta && okGap && okEig && okCh && okForm,
  "fields " <> ToString[okDelta] <> ", gap " <> ToString[okGap] <> ", neutral eigenvectors " <> ToString[okEig] <>
   ", charged " <> ToString[okCh] <> ", quadratic form " <> ToString[okForm]];

(* ------------------------------------------------------------------ WDT-3: the effective Lagrangian and Dashen *)
mt = ph Exp[I th[t]] eps;
(* |d/dt (phi e^(i theta))|^2 = phi^2 theta'^2 for real phi and theta, times tr(eps^T eps) = 2 *)
kin = zz ph^2 th'[t]^2 Tr[Transpose[eps].eps];
(* det M + c.c. = phi^2 (e^(2 i theta) + e^(-2 i theta)) det(eps) for real phi and theta *)
pot = -gd ph^2 Det[eps] (Exp[2 I th[t]] + Exp[-2 I th[t]]) // ExpToTrig;
lag = kin - pot;
eom = D[D[lag, th'[t]], t] - D[lag, th[t]];
(* linearized: th small *)
eomLin = Normal[Series[eom /. {th[t] -> e1 x[t], th'[t] -> e1 x'[t], th''[t] -> e1 x''[t]}, {e1, 0, 1}]] /. e1 -> 1;
(* A x'' + B x = 0 oscillates with omega^2 = B / A *)
m2 = Simplify[Coefficient[eomLin, x[t]]/Coefficient[eomLin, x''[t]]];
okMass = zeroQ[Together[m2 - 2 gd/zz]];
(* the chemical potential mu of F (each pair field has the charge 2): M -> e^(2 i mu t) M; the energy of the
   kinetic term in the static rotating state is -Z tr(dM^dag dM) (the term mu^2 of -L), chi^FF = -d^2 E/d mu^2 *)
(* the grand potential of the rotating state M = phi e^(2 i mu t) eps: -Z |dM/dt|^2 = -Z (2 mu phi)^2 tr(eps^T eps) *)
eMu = -zz (2 mu ph)^2 Tr[Transpose[eps].eps];
chiFF = -D[eMu, {mu, 2}];
eDet = pot /. th[t] -> 0;
okDashen = zeroQ[Simplify[chiFF - 16 zz ph^2]] && zeroQ[Simplify[eDet + 2 gd ph^2]] && zeroQ[Simplify[m2 chiFF + 16 eDet]];
check["WDT-3", "the effective Lagrangian Z tr(dM^dag dM) + G^det (det M + c.c.) at M = phi e^(i theta) i tau^2: the kinetic \
term 2 Z phi^2 theta'^2 and the potential -2 G^det phi^2 cos(2 theta) give the common phase the squared mass \
2 G^det/Z; the chemical potential of the particle number gives chi^FF = 16 Z phi^2 and the term has the energy \
E^det = -2 G^det phi^2, so m^2 chi^FF = -16 E^det: the relation of Dashen (of Gell-Mann, Oakes and Renner) for a \
term that changes the particle number by 4",
  okMass && okDashen, "m^2 = " <> ToString[InputForm[m2]] <> ", chi^FF = " <> ToString[InputForm[chiFF]] <> ", E^det = " <> ToString[InputForm[eDet]]];

(* ------------------------------------------------------------------ WDT-4: HS (0, 0), the exact relation *)
okId = zeroQ[Together[ee/(ee^2 - w^2/4) - 1/ee - (w^2/4)/(ee (ee^2 - w^2/4))]];
{sHead, sRows} = readTable[statesFile];
{vHead, vRows} = readTable[valFile];
valOf[name_] := num[Select[vRows, #[[1]] == name &][[1, 2]]];
rowOf[meth_, r_] := Select[sRows, #[[col[sHead, "method"]]] == meth && Abs[num[#[[col[sHead, "G_det_over_G_SH"]]]] - r] < 10^-12 &][[1]];
gSH = 1008/10; mLL = 2/10;
i1[mq_] := (Sqrt[1 + mq^2] - mq^2 ArcSinh[1/mq])/2;
(* r is raised to 50 digits: when r comes from an outer root search with 25 or 30 digits, it is the exact input of
   this inner problem, and the inner root is wanted with 40 *)
d0of[r_] := With[{rr = SetPrecision[r, 50]}, d /. FindRoot[gSH (1 + rr) i1[Sqrt[mLL^2 + d^2/4]]/(4 Pi^2) == 1, {d, 1}, WorkingPrecision -> 40]];
(* the arguments are raised to 60 digits, so that a root found with 30 digits and the cancellation in
   M^2 - w^2/4 near the threshold do not lower the precision of the integrand below the working precision (40); the
   integrand is smooth for w < 2 M, with a peak of width (M^2 - w^2/4)^(1/2) near q = 0 *)
kInt[w_?NumericQ, mq_] := With[{ww = SetPrecision[w, 60], m4 = SetPrecision[mq, 60]},
   NIntegrate[q^2/(Sqrt[q^2 + m4^2] (q^2 + m4^2 - ww^2/4)), {q, 0, 1}, WorkingPrecision -> 40, PrecisionGoal -> 30,
    MaxRecursion -> 40]/(4 Pi^2)];
j3[mq_] := ArcSinh[1/mq] - 1/Sqrt[1 + mq^2];
res4 = Table[Module[{gd = r gSH, d0, me, mg, md, row, rg, rd, ratio},
    d0 = d0of[r]; me = Sqrt[mLL^2 + d0^2/4];
    (* the bracket ends 1 per cent below the threshold 2 M: the roots for 0.01, 0.1 and 0.5 lie below 0.97 times it, and
       closer to the threshold the integrand has a peak of width (M^2 - w^2/4)^(1/2) that only costs precision *)
    mg = w /. FindRoot[(w^2/4) kInt[w, me] == 2 gd/(gSH^2 - gd^2), {w, 1/1000, 2 me (1 - 1/100)}, Method -> "Brent", WorkingPrecision -> 30];
    md = Sqrt[8 gd/((gSH + gd)^2 kInt[0, me])];
    (* chi^FF = d_0^2 J_3/pi^2 and phi_0 = d_0/(G + G^det): the mass of Dashen 32 G^det phi_0^2 / chi^FF *)
    ratio = mg^2/md^2 - ((gSH + gd)/(gSH - gd)) kInt[0, me]/kInt[mg, me];
    row = rowOf["HS (0, 0)", r];
    rg = num[row[[col[sHead, "m_G_RPA"]]]]; rd = num[row[[col[sHead, "m_G_Dashen"]]]];
    {r, d0, mg, md, Abs[ratio], Abs[mg - rg]/mg, Abs[md - rd]/md, Abs[Sqrt[32 gd (d0/(gSH + gd))^2/(d0^2 j3[me]/Pi^2)] - md]/md}],
   {r, {1/100, 1/10, 1/2}}];
dev4 = Max[res4[[All, 6]]]; dev4d = Max[res4[[All, 7]]];
check["WDT-4", "HS (0, 0): the identity E/(E^2 - w^2/4) - 1/E = (w^2/4)/(E (E^2 - w^2/4)) (symbolic); the pseudo-Goldstone \
boson is the root of (w^2/4) K(w) = 2 G^det/(G^2 - G^det^2) and the mass of Dashen is m_D^2 = 8 G^det/((G + G^det)^2 K(0)) \
= 32 G^det phi_0^2/chi^FF with chi^FF = d_0^2 J_3/pi^2, so that m^2/m_D^2 = ((G + G^det)/(G - G^det)) K(0)/K(m) exactly; \
with 30 digits for G^det/G_SH = 0.01, 0.1, 0.5, against the masses of the linear response and of Dashen of the Rust \
program on its grid of 24 radial nodes (largest relative difference; the grid sums differ from the integrals by less)",
  okId && Max[res4[[All, 5]]] < 10^-20 && Max[res4[[All, 8]]] < 10^-20 && dev4 < 10^-8 && dev4d < 10^-6,
  StringRiffle[Table["ratio " <> ToString[N[x[[1]]]] <> ": d_0 " <> ToString[NumberForm[N[x[[2]], 16], 16]] <> ", m_G " <>
     ToString[NumberForm[N[x[[3]], 16], 16]] <> ", m_Dashen " <> ToString[NumberForm[N[x[[4]], 16], 16]] <> ", (m/m_D)^2 = " <>
     ToString[NumberForm[N[x[[3]]^2/x[[4]]^2, 12], 12]], {x, res4}], "; "] <> "; Rust deviation of the linear response " <>
   sci[dev4] <> " (tolerance 1e-8), of the mass of Dashen " <> sci[dev4d] <> " (tolerance 1e-6: the susceptibility of the Rust \
program comes from differences of the chemical potential)"];

(* ------------------------------------------------------------------ WDT-5: the continuum limit and the crossings *)
okE = zeroW[Integrate[Sqrt[q^2 + m0^2], {q, 0, 1}, Assumptions -> m0 > 0] - (Sqrt[1 + m0^2] + m0^2 ArcSinh[1/m0])/2, {m0 -> 37/100}];
okA = zeroW[Integrate[(q^2 + mm0^2)/Sqrt[q^2 + m0^2], {q, 0, 1}, Assumptions -> m0 > 0] - (i1[m0] + mm0^2 ArcSinh[1/m0]),
   {m0 -> 37/100, mm0 -> 21/100}];
condG[r_?NumericQ] := Module[{d0 = d0of[r], me}, me = Sqrt[mLL^2 + d0^2/4];
   gSH (1 - r) (Sqrt[1 + me^2] + me^2 ArcSinh[1/me])/(8 Pi^2) - 1];
condT[r_?NumericQ] := Module[{d0 = d0of[r], me}, me = Sqrt[mLL^2 + d0^2/4];
   gSH (1 - r) (i1[me] + mLL^2 ArcSinh[1/me])/(4 Pi^2) - 1];
sPh[w_?NumericQ, mq_] := With[{ww = SetPrecision[w, 60], m4 = SetPrecision[mq, 60]},
   NIntegrate[q^2 Sqrt[q^2 + m4^2]/(q^2 + m4^2 - ww^2/4), {q, 0, 1}, WorkingPrecision -> 40, PrecisionGoal -> 30,
    MaxRecursion -> 40]/(4 Pi^2)];
crossAt[r_?NumericQ, frac_] := Module[{d0 = d0of[r], me}, me = Sqrt[mLL^2 + d0^2/4]; gSH (1 - r) sPh[frac d0, me] - 1];
rEnd = r /. FindRoot[condG[r], {r, 3/10, 99/100}, Method -> "Brent", WorkingPrecision -> 30];
rTrip = r /. FindRoot[condT[r], {r, 1/1000, 1/2}, Method -> "Brent", WorkingPrecision -> 30];
rHalf = r /. FindRoot[crossAt[r, 1/2], {r, 1/1000, 3/10}, Method -> "Brent", WorkingPrecision -> 25];
rCross = r /. FindRoot[crossAt[r, 1], {r, 3/10, 7/10}, Method -> "Brent", WorkingPrecision -> 25];
dev5 = Max[Abs[rEnd - valOf["r_end"]]/rEnd, Abs[rTrip - valOf["r_trip_end"]]/rTrip, Abs[rHalf - valOf["r_half"]]/rHalf,
   Abs[rCross - valOf["r_cross"]]/rCross];
check["WDT-5", "the continuum limit of HS (0, 0): int_0^1 E dq = (sqrt(1 + M^2) + M^2 arsinh(1/M))/2 and int_0^1 (q^2 + m^2)/E dq \
= I1(M) + m^2 arsinh(1/M) (symbolic); the pseudo-Goldstone boson reaches the threshold 2 M where (G - G^det) times the \
first over 4 pi^2 equals 1, the triplet where (G - G^det) times the second over 4 pi^2 equals 1, and the boson reaches \
d_0/2 and d_0 where (G - G^det) S_ph(w) = 1 at those frequencies; the couplings G^det/G_SH, solved with 25 to 30 \
digits, against the Rust program (largest relative difference)",
  okE && okA && dev5 < 10^-9,
  "integrals " <> ToString[okE && okA] <> "; the boson at the threshold " <> ToString[NumberForm[N[rEnd, 15], 15]] <>
   ", the triplet " <> ToString[NumberForm[N[rTrip, 15], 15]] <> ", m_G = d_0/2 at " <> ToString[NumberForm[N[rHalf, 15], 15]] <>
   ", m_G = d_0 at " <> ToString[NumberForm[N[rCross, 15], 15]] <> "; Rust deviation " <> sci[dev5]];

(* ------------------------------------------------------------------ WDT-6: the conversion to GeV *)
dev6 = Max[Table[Module[{v = Rationalize[num[row[[col[sHead, "v_SH"]]]], 0], lam, wg, wgg, lamF},
     lam = Rationalize[valOf["v_gev"], 0]/v;
     lamF = Rationalize[num[row[[col[sHead, "Lambda_GeV"]]]], 0];
     wg = num[row[[col[sHead, "m_G_RPA"]]]]; wgg = num[row[[col[sHead, "m_G_RPA_GeV"]]]];
     Max[Abs[lam - lamF]/lam, If[NumericQ[wg] && wg > 0, Abs[Rationalize[wg, 0] lam - Rationalize[wgg, 0]]/(Rationalize[wg, 0] lam), 0]]],
    {row, Select[sRows, num[#[[col[sHead, "G_det_over_G_SH"]]]] > 0 && StringFreeQ[#[[col[sHead, "m_G_RPA"]]], "NaN"] &]}]];
check["WDT-6", "the conversion to GeV of the Rust program: Lambda_LL = v/v_SH with v = 246.22 GeV and m_G = omega_G Lambda_LL, \
recomputed from the numbers of data/sh_det_states.csv in exact rational arithmetic (largest relative difference)",
  dev6 < 10^-14, "deviation " <> sci[dev6]];

out = StringJoin[
  "sh_det.wls: the exact formulas of stage 7, the determinant interaction (Revision_SM_Higgs, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WDT and a number), the statement and, in brackets, the\n",
  "facts that decide it.  Exact arithmetic, and numbers with 25 to 40 digits where a value of the Rust program is compared.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[NumberForm[Round[AbsoluteTime[] - t0, 0.1], {Infinity, 1}]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
