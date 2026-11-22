#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_vaidya.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23, stage 6)

   PURPOSE
   The exact part of stage 6: the brochon (a Dirac particle of mass m) in the ingoing Vaidya
   space-time, the field of a black hole whose mass M(v) changes.  Units G_N = c = hbar = 1.  The
   identifier of a check is WV and its number; each entry names, in parentheses, the checks of the
   Rust program lg_vaidya and of the Python program python/lg_vaidya.py that test the same statements.

     WV-1  (Rust LV-1; Python YV-1) the separation: with the spin connection computed here from the
           coframe (the tetrad postulate) for an arbitrary function M(v), the separated spinor with the
           angular equations (A) and the radial system (V1), (V2) solves the Dirac equation exactly, for
           every r > 0; the same spinor with a wrong coefficient in (V1) does not (a control that the
           check can fail);
     WV-2  (Rust LV-4, LV-5; Python YV-2) the conservation of the charge (V3), exactly;
     WV-3  (Rust LV-2; Python YV-3) for a constant mass (M = 1) the substitution P = e^(-i omega v) p,
           Q = e^(-i omega v) q turns (V1), (V2) into the static equations, which have at the horizon the
           exponents 0 and 4 i omega - 1/2;
     WV-4  (Rust LV-2; Python YV-4) the static solutions of stage 6 and the solutions of stage 5 in the
           freely falling tetrad are the same field: p = (1 + W)^(-1/2) e^(i omega h) P,
           q = (1 + W)^(1/2) e^(i omega h) Q with h' = 1/(1 + W), W = (2/r)^(1/2); the opposite exponents
           fail (a control);
     WV-4b (the explanation of WV-4) with v = tau + h(r) the coframe of stage 6 is the freely falling
           coframe boosted in the plane (0, 1) with the rapidity -ln(1 + W);
     WV-5  (Rust: the excision of the black hole) the radial light rays: the vectors (dv, dr) = (0, 1)
           (the ingoing rays, the slices v = const) and (1, f/2) (the outgoing rays) are null, and (V1),
           written d_v P + (f/2) d_r P = ..., differentiates P along the outgoing rays.

   THE SYSTEMS (as in the Rust program; m_j, lambda the angular quantum numbers, lambda = +-(j + 1/2))
     metric  ds^2 = f dv^2 - 2 dv dr - r^2 dOmega^2, f = 1 - 2 M(v)/r;
     coframe e^0 = ((1 + f)/2) dv - dr, e^1 = ((1 - f)/2) dv + dr, e^2 = r dth, e^3 = r sin(th) dph;
     (A)   A' = (m_j / sin th) A - lambda B,   B' = -(m_j / sin th) B + lambda A;
     spinor  r^(-1) sin(th)^(-1/2) e^(i m_j ph) S_H (-i Q A, i P B, P A, Q B), S_H = diag(H, H),
             H = i (sigma_1 + sigma_3)/2^(1/2), P = P(v, r), Q = Q(v, r);
     (V1)  d_v P = (M/r - 1/2) d_r P - (M/(2 r^2)) P - (i lambda/(2 r) + m/2) Q;
     (V2)  d_r Q = (i lambda/r - m) P;
     (V3)  d_v (2 |P|^2) + d_r (f |P|^2 - |Q|^2) = 0;
     static (M = 1, P = e^(-i omega v) p, Q = e^(-i omega v) q):
           r (r - 2) p' = 2 i omega r^2 p - p - (i lambda r + m r^2) q,   r q' = (i lambda - m r) p;
     stage 5, freely falling (W = (2/r)^(1/2)):
           P' = [(i omega - W/(4r)) P - (i lambda/r + m) Q]/(1 - W),
           Q' = [(i lambda/r - m) P + (-i omega + W/(4r)) Q]/(1 + W).

   NOTE ON NAMES
   The coordinate symbols (v, r, th, ph) and the parameters (mB, om, lam, mj) are never used as
   iterators.  The mass of the brochon is mB, the mass function of the black hole bhM[v].

   USAGE
     wolframscript -file lg_vaidya.wls <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 1, Print["usage: lg_vaidya.wls <report.txt>"]; Exit[2]];
reportFile = args[[1]];
t0 = AbsoluteTime[];
lines = {}; notes = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];
zeroQ[m_] := AllTrue[Flatten[{m}], TrueQ[Simplify[#] == 0] &];

(* ------------------------------------------------------------------ the matrices (indices 1..4 = 0..3) *)
sig = {IdentityMatrix[2], {{0, 1}, {1, 0}}, {{0, -I}, {I, 0}}, {{1, 0}, {0, -1}}};
sigbar = {sig[[1]], -sig[[2]], -sig[[3]], -sig[[4]]};
gam = Table[ArrayFlatten[{{0, sig[[a]]}, {sigbar[[a]], 0}}], {a, 4}];   (* gamma^a of the chiral basis, (1.2) *)
eta = DiagonalMatrix[{1, -1, -1, -1}];
gab[a_, b_] := (gam[[a]] . gam[[b]] - gam[[b]] . gam[[a]])/2;
hH = I (sig[[2]] + sig[[4]])/Sqrt[2];
sH = ArrayFlatten[{{hH, 0}, {0, hH}}];

(* ------------------------------------------------------------------ the geometry of a coframe *)
(* E[[a, k]] = e^a_k; F[[a, k]] = e_a^k; the Christoffel symbols; the spin connection of the tetrad postulate
   om[[k, a, b]] = omega_k^a_b; gamma^k = e_a^k gamma^a; the spinor connection Omega_k = 1/4 omega_{k ab} gamma^{ab}
   (the function geometry of lg_curved.wls and lg_schw.wls) *)
geometry[E_, X_, simp_] := Module[{F, g, gi, chr, om1, gU, spc},
  F = simp[Transpose[Inverse[E]]];
  g = simp[Transpose[E] . eta . E];
  gi = simp[Transpose[F] . eta . F];
  chr = simp[Table[1/2 Sum[gi[[rr, l]] (D[g[[l, n]], X[[k]]] + D[g[[l, k]], X[[n]]] - D[g[[k, n]], X[[l]]]),
     {l, 4}], {rr, 4}, {k, 4}, {n, 4}]];
  om1 = simp[Table[Sum[E[[a, n]] (D[F[[b, n]], X[[k]]] + Sum[chr[[n, k, l]] F[[b, l]], {l, 4}]), {n, 4}],
     {k, 4}, {a, 4}, {b, 4}]];
  gU = Table[Sum[F[[a, k]] gam[[a]], {a, 4}], {k, 4}];
  spc = simp[Table[1/4 Sum[eta[[a, a]] om1[[k, a, b]] gab[a, b], {a, 4}, {b, 4}], {k, 4}]];
  <|"gam" -> gU, "spc" -> spc, "X" -> X, "g" -> g|>];
(* the Dirac operator i gamma^k (d_k + Omega_k) psi - m psi *)
dirac[geo_, psi_, mass_] := Sum[I geo["gam"][[k]] . (D[psi, geo["X"][[k]]] + geo["spc"][[k]] . psi), {k, 4}] - mass psi;
angRules = {Derivative[1][aA][th] -> mj/Sin[th] aA[th] - lam bB[th], Derivative[1][bB][th] -> -mj/Sin[th] bB[th] + lam aA[th]};

(* ------------------------------------------------------------------ WV-1: the separation *)
fV = 1 - 2 bhM[v]/r;
coframe = {{(1 + fV)/2, -1, 0, 0}, {(1 - fV)/2, 1, 0, 0}, {0, 0, r, 0}, {0, 0, 0, r Sin[th]}};
geoV = geometry[coframe, {v, r, th, ph}, Simplify[#, r > 0 && 0 < th < Pi] &];
prefV = r^-1 Sin[th]^(-1/2) Exp[I mj ph];
psiV = prefV sH . {-I qQ[v, r] aA[th], I pP[v, r] bB[th], pP[v, r] aA[th], qQ[v, r] bB[th]};
(* (V1) with the coefficient c of the term in P: c = 1/2 is the system of the Rust program; c = 1 is the control *)
radV[c_] := {Derivative[1, 0][pP][v, r] -> (bhM[v]/r - 1/2) Derivative[0, 1][pP][v, r] - c bhM[v]/r^2 pP[v, r] -
     (I lam/(2 r) + mB/2) qQ[v, r],
   Derivative[0, 1][qQ][v, r] -> (I lam/r - mB) pP[v, r]};
dirV = dirac[geoV, psiV, mB];
resV = Simplify[(dirV /. angRules /. radV[1/2])/prefV, r > 0 && 0 < th < Pi];
resC = Simplify[(dirV /. angRules /. radV[1])/prefV, r > 0 && 0 < th < Pi];
(* the metric of the coframe must be the Vaidya metric *)
gV = {{fV, -1, 0, 0}, {-1, 0, 0, 0}, {0, 0, -r^2, 0}, {0, 0, 0, -r^2 Sin[th]^2}};
check["WV-1", "(Rust LV-1; Python YV-1) the separation in the ingoing Vaidya space-time: the coframe e^0 = ((1+f)/2) dv - \
dr, e^1 = ((1-f)/2) dv + dr, e^2 = r dth, e^3 = r sin(th) dph gives the metric f dv^2 - 2 dv dr - r^2 dOmega^2 with f = \
1 - 2 M(v)/r; with the spin connection computed from it for an arbitrary function M(v), the spinor r^(-1) sin(th)^(-1/2) \
e^(i m_j ph) S_H (-i Q A, i P B, P A, Q B) solves i gamma^k (d_k + Omega_k) psi = m psi exactly when A, B solve (A) and P, Q \
solve (V1), (V2), for every r > 0 and whatever d_v Q is; with the coefficient M/r^2 instead of M/(2 r^2) in (V1) the \
residual is not zero (the control)",
  zeroQ[geoV["g"] - gV] && zeroQ[resV] && ! zeroQ[resC],
  "metric difference " <> ToString[InputForm[Simplify[geoV["g"] - gV]]] <> "; residual " <> ToString[InputForm[resV]] <>
   "; control residual (abbreviated) " <> ToString[Short[InputForm[resC], 2]]];

(* ------------------------------------------------------------------ WV-2: the charge *)
(* the conjugates pB, qB of P, Q are independent functions that obey the conjugate equations (M(v), m, lambda real) *)
radVb = {Derivative[1, 0][pB][v, r] -> (bhM[v]/r - 1/2) Derivative[0, 1][pB][v, r] - bhM[v]/(2 r^2) pB[v, r] -
     (-I lam/(2 r) + mB/2) qB[v, r],
   Derivative[0, 1][qB][v, r] -> (-I lam/r - mB) pB[v, r]};
cont = Simplify[(2 D[pP[v, r] pB[v, r], v] + D[fV pP[v, r] pB[v, r] - qQ[v, r] qB[v, r], r]) /. radV[1/2] /. radVb];
check["WV-2", "(Rust LV-4, LV-5; Python YV-2) the conservation of the charge: d_v (2 |P|^2) + d_r (f |P|^2 - |Q|^2) = 0 \
exactly for every solution of (V1), (V2), for an arbitrary function M(v) (the conjugates of P, Q are independent functions \
that solve the conjugate equations; M, m, lambda real)", zeroQ[cont], "residual " <> ToString[InputForm[cont]]];

(* ------------------------------------------------------------------ WV-3: the exponents of the static equations *)
aEF = {{(2 I om - 1/r^2)/(1 - 2/r), -(I lam/r + mB)/(1 - 2/r)}, {I lam/r - mB, 0}};
resEF = Simplify[Limit[(r - 2) aEF, r -> 2]];
evEF = Simplify[Eigenvalues[resEF]];
okEF = Length[evEF] == 2 && AnyTrue[evEF, TrueQ[Simplify[#] == 0] &] &&
   AnyTrue[evEF, TrueQ[Simplify[# - (4 I om - 1/2)] == 0] &];
(* the static equations from (V1), (V2) with P = e^(-i omega v) p(r), Q = e^(-i omega v) q(r), M = 1: the residuals r1,
   r2 of (V1), (V2), multiplied by 2 r^2 e^(i omega v) and r e^(i omega v), must equal the residuals st1, st2 of the static
   equations *)
subStat = {pP -> Function[{vv, rr}, Exp[-I om vv] pp[rr]], qQ -> Function[{vv, rr}, Exp[-I om vv] qq[rr]]};
r1V = Derivative[1, 0][pP][v, r] - ((1/r - 1/2) Derivative[0, 1][pP][v, r] - 1/(2 r^2) pP[v, r] -
     (I lam/(2 r) + mB/2) qQ[v, r]);
r2V = Derivative[0, 1][qQ][v, r] - (I lam/r - mB) pP[v, r];
st1 = r (r - 2) pp'[r] - (2 I om r^2 pp[r] - pp[r] - (I lam r + mB r^2) qq[r]);
st2 = r qq'[r] - (I lam - mB r) pp[r];
statV = Simplify[({2 r^2 Exp[I om v] r1V, r Exp[I om v] r2V} /. subStat) - {st1, st2}];
check["WV-3", "(Rust LV-2; Python YV-3) for a constant mass (M = 1) the substitution P = e^(-i omega v) p(r), Q = e^(-i \
omega v) q(r) turns (V1), (V2) into r (r - 2) p' = 2 i omega r^2 p - p - (i lambda r + m r^2) q, r q' = (i lambda - m r) p; \
the residue matrix lim (r - 2) A(r) of this system at the horizon has the eigenvalues 0 (the solution analytic at r = 2, \
which falls in) and 4 i omega - 1/2, as in stage 5", zeroQ[statV] && okEF,
  "the static reduction " <> ToString[InputForm[statV]] <> "; residue " <> ToString[InputForm[resEF]] <> ", eigenvalues " <>
   ToString[InputForm[evEF]]];

(* ------------------------------------------------------------------ WV-4: the relation to stage 5 *)
(* Outside the horizon every square root is a rational function of b = (1 + W)^(1/2): W = b^2 - 1, r = 2/W^2,
   dW/dr = -W^3/4, db/dr = (dW/dr)/(2 b).  P, Q are the values of a solution of the system of stage 5 (freely falling
   tetrad) and dP, dQ its derivatives; p = e^(i omega h) b^s P, q = e^(i omega h) b^(-s) Q with h' = 1/(1 + W) = 1/b^2, so
   that the factor e^(i omega h) contributes (i omega/b^2) p to p'.  For s = -1 both static equations must hold
   identically in b, P, Q; for s = +1 they must fail. *)
wB = bb^2 - 1; rB = 2/wB^2; dWdr = -wB^3/4; dbdr = dWdr/(2 bb);
dP5 = ((I om - wB/(4 rB)) pV - (I lam/rB + mB) qV)/(1 - wB);
dQ5 = ((I lam/rB - mB) pV + (-I om + wB/(4 rB)) qV)/(1 + wB);
relEF[s_] := Module[{p, q, dp, dq},
  p = bb^s pV; q = bb^(-s) qV;
  dp = I om/bb^2 p + s bb^(s - 1) dbdr pV + bb^s dP5;
  dq = I om/bb^2 q - s bb^(-s - 1) dbdr qV + bb^(-s) dQ5;
  Together[{rB (rB - 2) dp - (2 I om rB^2 p - p - (I lam rB + mB rB^2) q), rB dq - (I lam - mB rB) p}]];
rel4 = relEF[-1]; rel4c = relEF[1];
check["WV-4", "(Rust LV-2; Python YV-4) the static solutions of stage 6 and the stationary solutions of stage 5 are the \
same field: if (P, Q) solve the system of stage 5 in the freely falling tetrad (W = (2/r)^(1/2)), then p = (1 + W)^(-1/2) \
e^(i omega h) P, q = (1 + W)^(1/2) e^(i omega h) Q with h' = 1/(1 + W) solve the static equations of stage 6 identically \
(written as rational functions of b = (1 + W)^(1/2)); with the opposite exponents, (1 + W)^(1/2) and (1 + W)^(-1/2), they do \
not (the control).  So the coframe of stage 6 is the freely falling coframe boosted with the rapidity -ln(1 + W), v = tau + \
h(r), and the two problems have the same quasi-bound frequencies", zeroQ[rel4] && ! zeroQ[rel4c],
  "residuals " <> ToString[InputForm[Simplify[rel4]]] <> "; control (abbreviated) " <>
   ToString[Short[InputForm[Simplify[rel4c]], 2]]];
(* the boost itself: in the coordinates (tau, r) of stage 5, with dv = dtau + dr/(1 + W), the coframe of stage 6 is
   Lambda . (freely falling coframe) with Lambda the boost of rapidity chi = -Log[1 + W] in the plane (0, 1) *)
wR = Sqrt[2/r]; fR = 1 - wR^2;
cfPG = {{1, 0}, {wR, 1}};                                                (* e^0 = dtau, e^1 = dr + W dtau, on (dtau, dr) *)
cfEF = {{(1 + fR)/2, (1 + fR)/2/(1 + wR) - 1}, {(1 - fR)/2, (1 - fR)/2/(1 + wR) + 1}};    (* stage 6, dv = dtau + dr/(1+W) *)
chi = -Log[1 + wR];
boost = {{Cosh[chi], Sinh[chi]}, {Sinh[chi], Cosh[chi]}};
boostDiff = Simplify[TrigToExp[cfEF - boost . cfPG], r > 0];
check["WV-4b", "(the explanation of WV-4) with v = tau + h(r), h' = 1/(1 + W), the two radial covectors of the coframe of \
stage 6 are the boost of rapidity -ln(1 + W) of those of the freely falling coframe e^0 = dtau, e^1 = dr + W dtau, at every \
r > 0", zeroQ[boostDiff], "difference " <> ToString[InputForm[boostDiff]]];

(* ------------------------------------------------------------------ WV-5: the light rays *)
(* radial vectors (dv, dr): the length g(X, X) = f dv^2 - 2 dv dr; the radial null directions are dv = 0 and dr = f dv/2,
   since f dv^2 - 2 dv dr = dv (f dv - 2 dr) *)
nullLen[x_] := Simplify[x . gV[[1 ;; 2, 1 ;; 2]] . x];
factorRays = Simplify[fV dvv^2 - 2 dvv drr - dvv (fV dvv - 2 drr)];
transport = Simplify[-(bhM[v]/r - 1/2) - fV/2];          (* -(coefficient of d_r P in (V1)) minus f/2 *)
okRays = zeroQ[{nullLen[{0, 1}], nullLen[{1, fV/2}], factorRays, transport}];
check["WV-5", "(Rust: the excision of the black hole) the radial light rays: f dv^2 - 2 dv dr = dv (f dv - 2 dr), so the \
radial null directions are dv = 0 (the ingoing rays, which are the slices v = const) and dr/dv = f/2 (the outgoing rays); \
(V1) is d_v P + (f/2) d_r P = -(M/(2 r^2)) P - (i lambda/(2 r) + m/2) Q, a transport of P along the outgoing rays, outward \
where f > 0 and inward inside the apparent horizon r = 2 M(v); hence an inner edge of the grid inside the apparent horizon \
needs no boundary condition", okRays,
  "lengths of (0, 1) and (1, f/2): " <> ToString[InputForm[{nullLen[{0, 1}], nullLen[{1, fV/2}]}]] <>
   "; factorization " <> ToString[InputForm[factorRays]] <> "; transport " <> ToString[InputForm[transport]]];

out = StringJoin[
  "lg_vaidya.wls: the exact checks of stage 6 of task C23, the brochon in the ingoing Vaidya space-time \
(Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WV and a number), the statement and, in brackets, the\n",
  "facts that decide it.  All checks are exact (symbolic).\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  StringJoin[("  " <> # <> "\n") & /@ notes],
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
Exit[If[npass == ntot, 0, 1]];
