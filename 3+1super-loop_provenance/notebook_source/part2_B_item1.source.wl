(* ::Section:: *)
(*II.2 Item [1]: the field equations on the 3+1 manifold, and the checks GS-1 to GS-16 (program t2_fields)*)

(* ::Text:: *)
(*The program t2_fields (t2fields/src/main.rs) builds every matrix of the field equations of the self-interacting spinor field $\Psi_{\rm LL}$ on $\mathbb R^{1,3}_{\rm LL}$ and checks every statement made about them: checks GS-1 to GS-16. The rewriting below keeps the order of the original program, because the random numbers are drawn in that order from one generator with the seed 2026111707. Every check prints one line with PASS or FAIL; the statement is that of the original report, and the numbers in the details are the Wolfram Language results, followed (where the original report gives them) by the values of the original report.*)

(* ::Text:: *)
(*The physics in brief. The covariant derivative is $\nabla^{\rm LL}_p=\partial_p+\Gamma^{\rm LL}_p-i\mathsf A_{{\rm LL},p}$, with the spin connection $\Gamma^{\rm LL}_p=\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\otimes\sigma^{\rm LL}_0$ (the Lorentz connection $\mathbf u^\omega_{{\rm LL},p}=\sum_j(\omega^j_{{\rm LL},p}J^j_{\rm LL}+\varpi^j_{{\rm LL},p}K^j_{\rm LL})$, an element of the Lorentz algebra of the super-loop, acting on the Dirac index) and the gauge matrix $\mathsf A_{{\rm LL},p}=g^{\rm v}_{\rm LL}\sum_\iota A^\iota_{{\rm LL},p}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathsf 1_8$ of the isospin group and of the fermion number. The mean-field (Dirac-Bogoliubov-de Gennes) equations couple the particle part $\psi^{\rm p}_{\rm LL}$ and the hole part $\psi^{\rm h}_{\rm LL}$ of a mode through the pairing block $\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$, where $\vec d_{\rm LL}=(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})$ is the order parameter.*)

(* ::Input:: *)
(* the seed of the program t2_fields and the tolerance of the exact relations *)
t2Seed[2026111707];
tol = 1*^-12;
(* the Frobenius norm, the name used in the Rust code *)
fro[m_] := Norm[m, "Frobenius"];
(* the four Dirac matrices of the canonical frame *)
g = Table[t2gamma[p], {p, 0, 3}];

(* ::Subsection:: *)
(*GS-1 and GS-2: the Dirac matrices and the Lorentz generators*)

(* ::Input:: *)
(* GS-1: Clifford relations {gamma^p, gamma^q} = 2 eta^pq 1_4, and the forms of gamma_ch, C, alpha^j, the frame and eps2 *)
cliff = Max[Table[fro[g[[p + 1]] . g[[q + 1]] + g[[q + 1]] . g[[p + 1]] - If[p == q, 2 t2eta[p], 0] IdentityMatrix[4]], {p, 0, 3}, {q, 0, 3}]];
gchOk = fro[t2gammaCh - DiagonalMatrix[{-1, -1, 1, 1}]] < tol;
cOk = fro[t2cconj - ArrayFlatten[{{t2eps2, 0}, {0, -t2eps2}}]] < tol;
alOk = AllTrue[Range[3], fro[t2alpha[#] - ArrayFlatten[{{-t2sigma[#], 0}, {0, t2sigma[#]}}]] < tol &];
frameOk = AllTrue[Range[0, 3], fro[t2mq[t2canonicalFrame[[# + 1]]] - t2sigma[#]] < tol &&
      fro[t2mq[t2bqConj[t2canonicalFrame[[# + 1]]]] - If[# == 0, 1, -1] t2sigma[#]] < tol &] &&
   fro[t2mq[{0, 0, -1, 0}] - t2eps2] < tol;
SL31Check["GS-1", "the Dirac matrices are built from the frame of R^(1,3)_LL; Clifford relations; gamma_ch = diag(-1,-1,1,1), C_LL = diag(eps2, -eps2), alpha^j = diag(-sigma_j, sigma_j), eps2 = M_LL(-e2)",
  cliff < tol && gchOk && cOk && alOk && frameOk, Row[{"largest deviation of the Clifford relations ", cliff, " (original: 0.0e0)"}]]

(* ::Input:: *)
(* GS-2: the Lorentz generators sgen(J^j), sgen(K^j); the 27 relations of so(3,1); the 24 intertwining relations *)
sj = Table[t2spinGen[t2jgen[j]], {j, 3}];
sk = Table[t2spinGen[t2kgen[j]], {j, 3}];
comm[a_, b_] := a . b - b . a;
genOk = And @@ Flatten[Table[With[{k = Mod[j, 3] + 1, l = Mod[j + 1, 3] + 1},
      {fro[sj[[j]] - g[[k + 1]] . g[[l + 1]]/2] < tol, fro[sk[[j]] - g[[1]] . g[[j + 1]]/2] < tol,
       fro[sj[[j]] - ArrayFlatten[{{-I/2 t2sigma[j], 0}, {0, -I/2 t2sigma[j]}}]] < tol,
       fro[sk[[j]] - ArrayFlatten[{{-1/2 t2sigma[j], 0}, {0, 1/2 t2sigma[j]}}]] < tol}], {j, 3}]];
rel = Flatten[Table[With[{lj = Sum[t2eps3[a, b, c] sj[[c]], {c, 3}], lk = Sum[t2eps3[a, b, c] sk[[c]], {c, 3}]},
      {fro[comm[sj[[a]], sj[[b]]] - lj] < tol, fro[comm[sj[[a]], sk[[b]]] - lk] < tol, fro[comm[sk[[a]], sk[[b]]] + lj] < tol}], {a, 3}, {b, 3}]];
intertw = Flatten[Table[With[{s = t2spinGen[u], v = t2vecAction[u]},
      Table[fro[comm[g[[p + 1]], s] - Sum[v[[p + 1, q + 1]] g[[q + 1]], {q, 0, 3}]] < tol, {p, 0, 3}]], {j, 3}, {u, {t2jgen[j], t2kgen[j]}}]];
SL31Check["GS-2", "the Lorentz generators of the super-loop on the Dirac spinor, sgen(J^j) = diag(-i sigma_j/2, -i sigma_j/2) = gamma^j' gamma^j''/2, sgen(K^j) = diag(-sigma_j/2, sigma_j/2) = gamma^0 gamma^j/2; the 27 relations of so(3,1); 24 intertwining relations [gamma^p, sgen(u)] = sum V(u)[p][q] gamma^q",
  genOk && (And @@ rel) && (And @@ intertw), Row[{Length[rel], " commutation relations, ", Length[intertw], " intertwining relations (original: 27 and 24)"}]]

(* ::Subsection:: *)
(*GS-3 and GS-4: finite transformations, isospin and the pair matrices*)

(* ::Text:: *)
(*GS-3 is the change of representation of the gauge group $SL(2,\mathbb C)$: for a unit bi-quaternion $\mathbf g$ the spinor matrix $\mathsf S_{\rm LL}(\mathbf g)=\mathrm{diag}\big(M_{\rm LL}(\tilde{\mathbf g}),M_{\rm LL}(\mathbf g)\big)$, with $\tilde{\mathbf g}=(\mathbf g^\dagger)^{-1}$ and so $M_{\rm LL}(\tilde{\mathbf g})=(M_{\rm LL}(\mathbf g)^\dagger)^{-1}$, satisfies $\mathsf S_{\rm LL}(\mathbf g)^{-1}\gamma^p_{\rm LL}\mathsf S_{\rm LL}(\mathbf g)=\sum_{p'}[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}\gamma^{p'}_{\rm LL}$ with the real Lorentz matrix $[\mathsf L_{\rm LL}(\mathbf g)]_{pp'}=\tfrac12{\rm Re}\,{\rm tr}\big(\sigma^{\rm LL}_pM_{\rm LL}(\mathbf g)\sigma^{\rm LL}_{p'}M_{\rm LL}(\mathbf g)^\dagger\big)$, and $\mathsf S_{\rm LL}(\mathbf g)\gamma^p_{\rm LL}[\mathbf f]\mathsf S_{\rm LL}(\mathbf g)^{-1}=\gamma^p_{\rm LL}[\tilde{\mathbf g}\mathbf f\tilde{\mathbf g}^\dagger]$ for random Hermitian frames $\mathbf f$. GS-4 checks the isospin algebra, the antisymmetry and the invariance of the pair matrices $\mathsf K^\iota_{\rm LL}$, and that $\Psi^T_{\rm LL}\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}=0$ for a commuting (c-number) spinor, for which the antisymmetric matrix gives zero.*)

(* ::Input:: *)
(* GS-3: 200 random unit bi-quaternions; for each, a random Hermitian frame (the random numbers in the order of t2_fields) *)
worst3 = 0.; worst3b = 0.;
Do[
  Module[{mg, s4, s4i, l, f, gf, mgt, f2, gf2},
   mg = t2randSL2C[];
   s4 = t2spinRep[mg];
   s4i = ArrayFlatten[{{ConjugateTranspose[mg], 0}, {0, t2inv2[mg]}}];
   worst3 = Max[worst3, fro[s4 . s4i - IdentityMatrix[4]]];
   l = t2lmat[mg];
   Do[worst3 = Max[worst3, fro[s4i . g[[p + 1]] . s4 - Sum[l[[p + 1, q + 1]] g[[q + 1]], {q, 0, 3}]]], {p, 0, 3}];
   (* the frame f_p = x0 e0 + i (x1 e1 + x2 e2 + x3 e3), real x drawn p by p *)
   f = Table[Module[{x0 = t2Unit[]}, Prepend[Table[I t2Unit[], {3}], x0]], {4}];
   gf = t2gammasFromFrame[f];
   mgt = t2inv2[ConjugateTranspose[mg]];
   f2 = Table[With[{x = t2coords[mgt . t2mq[f[[p]]] . ConjugateTranspose[mgt]]}, {x[[1]], I x[[2]], I x[[3]], I x[[4]]}], {p, 4}];
   gf2 = t2gammasFromFrame[f2];
   Do[worst3b = Max[worst3b, fro[s4 . gf[[p]] . s4i - gf2[[p]]]], {p, 4}]],
  {200}];
SL31Check["GS-3", "the gauge group SL(2,C) as a change of representation: Sspin^-1 gamma^p Sspin = sum Lmat[p][q] gamma^q and Sspin gamma^p[f] Sspin^-1 = gamma^p[gLt f gLt^dag] for 200 random gL and random Hermitian frames (tolerance 1e-10)",
  worst3 < 1*^-10 && worst3b < 1*^-10, Row[{"largest deviations ", worst3, " and ", worst3b, " (original: 5.8e-14 and 6.6e-14)"}]]

(* ::Input:: *)
(* GS-4: isospin, Lorentz invariance and isospin covariance of the pair matrices, the c-number spinor *)
t = Table[t2tiso[i], {i, 3}];
k = Table[t2kpair[i], {i, 3}];
isoOk = And @@ Flatten[Table[fro[comm[t[[a]], t[[b]]] - Sum[I t2eps3[a, b, c] t[[c]], {c, 3}]] < tol, {a, 3}, {b, 3}]] &&
   And @@ Flatten[Table[fro[comm[t[[a]], t2lift4[sj[[u]]]]] < tol && fro[comm[t[[a]], t2lift4[sk[[u]]]]] < tol, {a, 3}, {u, 3}]];
kkOk = AllTrue[k, fro[# + Transpose[#]] < tol &];
worst4 = 0.; cnum = 0.;
Do[
  Module[{mg, s8, th, u2, u8, r, psi},
   mg = t2randSL2C[];
   s8 = t2lift4[t2spinRep[mg]];
   th = Table[3 t2Unit[], {3}];
   u2 = t2su2[th]; u8 = t2lift2[u2]; r = t2adjoint[u2];
   Do[worst4 = Max[worst4, fro[Transpose[s8] . k[[a]] . s8 - k[[a]]],
      fro[Transpose[u8] . k[[a]] . u8 - Sum[r[[a, b]] k[[b]], {b, 3}]]], {a, 3}];
   psi = t2RndVec[8];
   Do[cnum = Max[cnum, Abs[psi . k[[a]] . psi]], {a, 3}]],
  {100}];
SL31Check["GS-4", "isospin T^iota obeys [T^a, T^b] = i eps T^c and commutes with the Lorentz generators; the pair matrices Kch_iota are antisymmetric, Lorentz invariant and an isospin vector (100 random gL and U); Psi^T Kch Psi = 0 for a c-number spinor (tolerance 1e-10)",
  isoOk && kkOk && worst4 < 1*^-10 && cnum < 1*^-12, Row[{"largest deviation ", worst4, "; largest |Psi^T Kch Psi| ", cnum, " (original: 5.2e-15; 3.0e-16)"}]]

(* ::Subsection:: *)
(*GS-5 and GS-6: gauge covariance and curvature of the covariant derivative*)

(* ::Text:: *)
(*Fields linear in the coordinates $x_{\rm LL}=(x^0_{\rm LL},x^1_{\rm LL},x^2_{\rm LL},x^3_{\rm LL})$ are drawn at random: the isospin potentials $A^\iota_{{\rm LL},p}$, the fermion-number potentials $A^{\rm F}_{{\rm LL},p}$ and the Lorentz connection with the rotation components $\omega^j_{{\rm LL},p}$ and the boost components $\varpi^j_{{\rm LL},p}$; the spinor field $\Psi_{\rm LL}$ is a sum of three plane waves. A gauge transformation $\mathcal U_{\rm LL}=\exp(i\theta^{\rm F}_{\rm LL})\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL}$ is applied, whose phase $\theta^{\rm F}_{\rm LL}$, isospin rotation $\mathsf U_{\rm LL}=\exp(i\sum_\iota\theta^\iota_{\rm LL}\tau^{\rm LL}_\iota/2)$ and unit bi-quaternion $\mathbf g$ (with $M_{\rm LL}(\mathbf g)=\exp M_{\rm LL}(\mathbf u)$ for a Lorentz generator $\mathbf u$) depend on $x_{\rm LL}$ through $\theta^{\rm F}_{\rm LL}$, $\theta^\iota_{\rm LL}$ and $\mathbf u$, which are linear in $x_{\rm LL}$. The transformed fields are $A^{\rm F}_{{\rm LL},p}+\partial_p\theta^{\rm F}_{\rm LL}/g^{\rm F}_{\rm LL}$, $\mathsf U_{\rm LL}\mathsf a_{{\rm LL},p}\mathsf U_{\rm LL}^{-1}-(i/g^{\rm v}_{\rm LL})(\partial_p\mathsf U_{\rm LL})\mathsf U_{\rm LL}^{-1}$ in place of $\mathsf a_{{\rm LL},p}=\sum_\iota A^\iota_{{\rm LL},p}\tau^{\rm LL}_\iota/2$, and, on the Dirac index, $\mathsf S_{\rm LL}(\mathbf g)\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})\mathsf S_{\rm LL}(\mathbf g)^{-1}-(\partial_p\mathsf S_{\rm LL}(\mathbf g))\mathsf S_{\rm LL}(\mathbf g)^{-1}$ in place of $\mathsf s_{\rm LL}(\mathbf u^\omega_{{\rm LL},p})$. GS-5 checks that the covariant derivative made of the transformed fields, applied to $\mathcal U_{\rm LL}\Psi_{\rm LL}$, equals $\mathcal U_{\rm LL}\nabla^{\rm LL}_p\Psi_{\rm LL}$ at 20 random points; the derivatives of the gauge functions are computed by central differences with the step $10^{-5}$. GS-6 checks that the commutator of two covariant derivatives is the curvature, $[\nabla^{\rm LL}_p,\nabla^{\rm LL}_{p'}]\Psi_{\rm LL}=\big(\mathsf R^{\rm LL}_{pp'}-ig^{\rm v}_{\rm LL}\sum_\iota\mathsf F^\iota_{{\rm LL},pp'}T^\iota_{\rm LL}-ig^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},pp'}\big)\Psi_{\rm LL}$, with $\mathsf R^{\rm LL}_{pp'}=\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+\Gamma^{\rm LL}_p\Gamma^{\rm LL}_{p'}-\Gamma^{\rm LL}_{p'}\Gamma^{\rm LL}_p$, $\mathsf F^\iota_{{\rm LL},pp'}=\partial_pA^\iota_{{\rm LL},p'}-\partial_{p'}A^\iota_{{\rm LL},p}+g^{\rm v}_{\rm LL}\sum_{\iota',\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}A^{\iota'}_{{\rm LL},p}A^{\iota''}_{{\rm LL},p'}$ and $\mathsf F^{\rm F}_{{\rm LL},pp'}=\partial_pA^{\rm F}_{{\rm LL},p'}-\partial_{p'}A^{\rm F}_{{\rm LL},p}$.*)

(* ::Input:: *)
gv = 0.7; gf = 0.45;
(* the fields linear in the coordinates, drawn in the order of t2_fields *)
a0 = ConstantArray[0., {3, 4}]; a1 = ConstantArray[0., {3, 4, 4}];
Do[a0[[i, p]] = t2Unit[]; Do[a1[[i, p, q]] = 0.5 t2Unit[], {q, 4}], {i, 3}, {p, 4}];
f0 = ConstantArray[0., 4]; f1 = ConstantArray[0., {4, 4}];
Do[f0[[p]] = t2Unit[]; Do[f1[[p, q]] = 0.5 t2Unit[], {q, 4}], {p, 4}];
w0 = ConstantArray[0., {6, 4}]; w1 = ConstantArray[0., {6, 4, 4}];
Do[w0[[i, p]] = 0.5 t2Unit[]; Do[w1[[i, p, q]] = 0.3 t2Unit[], {q, 4}], {i, 6}, {p, 4}];
(* three plane waves: a momentum (4 numbers) and an 8-component amplitude *)
waves = Table[Module[{qq = Table[t2Unit[], {4}]}, {qq, t2RndVec[8]}], {3}];
psiAt[x_] := Sum[wave[[2]] Exp[I wave[[1]] . x], {wave, waves}];
dpsiAt[x_, p_] := Sum[wave[[2]] I wave[[1, p + 1]] Exp[I wave[[1]] . x], {wave, waves}];
lin[c0_, sl_, x_] := c0 + sl . x;
aAt[x_, p_] := Table[lin[a0[[i, p + 1]], a1[[i, p + 1]], x], {i, 3}];
afAt[x_, p_] := lin[f0[[p + 1]], f1[[p + 1]], x];
(* the Lorentz element sum_j (om_j J^j + vp_j K^j) as a bi-quaternion *)
lorentzElem[om_, vp_] := Prepend[Table[0.5 om[[j]] + 0.5 I vp[[j]], {j, 3}], 0];
gammaAt[x_, p_] := t2spinGen[lorentzElem[Table[lin[w0[[j, p + 1]], w1[[j, p + 1]], x], {j, 3}], Table[lin[w0[[3 + j, p + 1]], w1[[3 + j, p + 1]], x], {j, 3}]]];
(* the 8 x 8 gauge matrix gv A.T + gF AF *)
gauge8[gv_, gf_, a_, af_] := gf af IdentityMatrix[8] + Sum[gv a[[i]] t2tiso[i], {i, 3}];
(* nabla_p Psi = d_p Psi + (Gamma_p (x) 1) Psi - i (gv A.T + gF AF) Psi *)
nabla[psi_, dpsi_, gam_, a_, af_] := dpsi + t2lift4[gam] . psi - I gauge8[gv, gf, a, af] . psi;
(* the gauge transformation: the phase thF(x), U(x) in SU(2), gL(x) = exp(Lorentz element(x)) *)
tf = Table[t2Unit[], {5}];
tu = Table[t2Unit[], {3}, {5}];
tl = Table[0.5 t2Unit[], {6}, {5}];
l5[cf_, x_] := cf[[1]] + cf[[2 ;; 5]] . x;
thfAt[x_] := l5[tf, x];
uAt[x_] := t2su2[Table[l5[tu[[i]], x], {i, 3}]];
mgAt[x_] := t2expm[t2mq[lorentzElem[Table[l5[tl[[j]], x], {j, 3}], Table[l5[tl[[3 + j]], x], {j, 3}]]]];
s4At[x_] := t2spinRep[mgAt[x]];
caluAt[x_] := Exp[I thfAt[x]] t2lift4[s4At[x]] . t2lift2[uAt[x]];
shift[x_, p_, d_] := ReplacePart[x, p + 1 -> x[[p + 1]] + d];

(* ::Input:: *)
(* GS-5: nabla'_p (Ucal Psi) = Ucal nabla_p Psi at 20 random points *)
h = 1*^-5; worst5 = 0.; worst5b = 0.;
Do[
  Module[{x = Table[t2Unit[], {4}], cu, psi, psit},
   cu = caluAt[x]; psi = psiAt[x]; psit = cu . psi;
   Do[
    Module[{thd, aft, u2, du, a, amat, amat2, at, s4, ds4, s4i, gam, gamt, dpsit, lhs, rhs},
     thd = (thfAt[shift[x, p, h]] - thfAt[shift[x, p, -h]])/(2 h);
     aft = afAt[x, p] + thd/gf;
     u2 = uAt[x];
     du = (uAt[shift[x, p, h]] - uAt[shift[x, p, -h]]) (0.5/h);
     a = aAt[x, p];
     amat = Sum[0.5 a[[i]] t2sigma[i], {i, 3}];
     amat2 = u2 . amat . ConjugateTranspose[u2] - (I/gv) du . ConjugateTranspose[u2];
     at = Table[Re[Tr[amat2 . t2sigma[i]]], {i, 3}];
     worst5b = Max[worst5b, fro[amat2 - ConjugateTranspose[amat2]]];
     s4 = s4At[x];
     ds4 = (s4At[shift[x, p, h]] - s4At[shift[x, p, -h]]) (0.5/h);
     s4i = ArrayFlatten[{{ConjugateTranspose[mgAt[x]], 0}, {0, t2inv2[mgAt[x]]}}];
     gam = gammaAt[x, p];
     gamt = s4 . gam . s4i - ds4 . s4i;
     dpsit = (caluAt[shift[x, p, h]] . psiAt[shift[x, p, h]] - caluAt[shift[x, p, -h]] . psiAt[shift[x, p, -h]]) (0.5/h);
     lhs = nabla[psit, dpsit, gamt, at, aft];
     rhs = cu . nabla[psi, dpsiAt[x, p], gam, a, afAt[x, p]];
     worst5 = Max[worst5, Norm[lhs - rhs]/(1 + Norm[rhs])]],
    {p, 0, 3}]],
  {20}];
SL31Check["GS-5", "gauge covariance of the covariant derivative: nabla'_p (Ucal Psi) = Ucal nabla_p Psi for Ucal(x) = exp(i thF) Sspin(gL(x)) (x) U(x), with the transformed AF, Av and Gamma (20 random points, central differences with the step 1e-5; relative tolerance 1e-7)",
  worst5 < 1*^-7 && worst5b < 1*^-9, Row[{"largest relative deviation ", worst5, " (original: 1.4e-10)"}]]

(* ::Input:: *)
(* GS-6: [nabla_p, nabla_q] Psi = (R_pq - i gv F^iota_pq T^iota - i gF F^F_pq) Psi, outer derivative by central differences *)
h6 = 1*^-4; worst6 = 0.;
Do[
  Module[{x = Table[t2Unit[], {4}], psi},
   psi = psiAt[x];
   Do[If[p == q, Continue[]];
    Module[{nq, np, npq, nqp, lhs, dgq, dgp, rr, ap, aq, fv, ff, curv, rhs},
     nq[y_] := nabla[psiAt[y], dpsiAt[y, q], gammaAt[y, q], aAt[y, q], afAt[y, q]];
     np[y_] := nabla[psiAt[y], dpsiAt[y, p], gammaAt[y, p], aAt[y, p], afAt[y, p]];
     npq = nabla[nq[x], (nq[shift[x, p, h6]] - nq[shift[x, p, -h6]]) (0.5/h6), gammaAt[x, p], aAt[x, p], afAt[x, p]];
     nqp = nabla[np[x], (np[shift[x, q, h6]] - np[shift[x, q, -h6]]) (0.5/h6), gammaAt[x, q], aAt[x, q], afAt[x, q]];
     lhs = npq - nqp;
     (* R_pq = d_p Gamma_q - d_q Gamma_p + [Gamma_p, Gamma_q]; the derivatives are exact (the fields are linear) *)
     dgq = {Table[w1[[j, q + 1, p + 1]], {j, 3}], Table[w1[[3 + j, q + 1, p + 1]], {j, 3}]};
     dgp = {Table[w1[[j, p + 1, q + 1]], {j, 3}], Table[w1[[3 + j, p + 1, q + 1]], {j, 3}]};
     rr = t2spinGen[lorentzElem @@ dgq] - t2spinGen[lorentzElem @@ dgp] + comm[gammaAt[x, p], gammaAt[x, q]];
     ap = aAt[x, p]; aq = aAt[x, q];
     fv = Table[a1[[i, q + 1, p + 1]] - a1[[i, p + 1, q + 1]] + Sum[gv t2eps3[i, k3, l3] ap[[k3]] aq[[l3]], {k3, 3}, {l3, 3}], {i, 3}];
     ff = f1[[q + 1, p + 1]] - f1[[p + 1, q + 1]];
     curv = t2lift4[rr] - I gauge8[gv, gf, fv, ff];
     rhs = curv . psi;
     worst6 = Max[worst6, Norm[lhs - rhs]/(1 + Norm[rhs])]],
    {p, 0, 3}, {q, 0, 3}]],
  {20}];
SL31Check["GS-6", "the commutator of two covariant derivatives is the curvature of the gauge group: [nabla_p, nabla_q] Psi = (Rc_pq - i g_v Fv^iota_pq T^iota - i g_F FF_pq) Psi (20 random points, all 12 ordered pairs; central differences with the step 1e-4; relative tolerance 1e-6)",
  worst6 < 1*^-6, Row[{"largest relative deviation ", worst6, " (original: 2.1e-9)"}]]

(* ::Subsection:: *)
(*GS-7 to GS-9: the super-loop form of the Dirac operator and the Dirac-Bogoliubov-de Gennes equations*)

(* ::Input:: *)
(* GS-7: with constant non-abelian gauge fields nabla_p acts on plane waves as N_p = i k_p - i (gv A_p.T + gF AF_p) *)
worst7 = 0.;
Do[
  Module[{kv = Table[t2Unit[], {4}], nps, dirac, ur, ll, sq, rhs, md, mdb, nk},
   nps = Table[Module[{a = Table[t2Unit[], {3}]}, I kv[[p + 1]] IdentityMatrix[8] - I gauge8[gv, gf, a, t2Unit[]]], {p, 0, 3}];
   dirac = Sum[t2lift4[g[[p + 1]]] . nps[[p + 1]], {p, 0, 3}];
   ur = Sum[KroneckerProduct[t2sigma[p], IdentityMatrix[2]] . nps[[p + 1, 5 ;; 8, 5 ;; 8]], {p, 0, 3}];
   ll = Sum[KroneckerProduct[If[p == 0, 1, -1] t2sigma[p], IdentityMatrix[2]] . nps[[p + 1, 1 ;; 4, 1 ;; 4]], {p, 0, 3}];
   worst7 = Max[worst7, fro[dirac[[1 ;; 4, 5 ;; 8]] - ur], fro[dirac[[5 ;; 8, 1 ;; 4]] - ll],
     fro[dirac[[1 ;; 4, 1 ;; 4]]] + fro[dirac[[5 ;; 8, 5 ;; 8]]]];
   sq = dirac . dirac;
   rhs = Sum[t2eta[p] nps[[p + 1]] . nps[[p + 1]], {p, 0, 3}] +
     Sum[0.25 t2lift4[comm[g[[p + 1]], g[[q + 1]]]] . comm[nps[[p + 1]], nps[[q + 1]]], {p, 0, 3}, {q, 0, 3}];
   worst7 = Max[worst7, fro[sq - rhs]];
   md = Sum[I kv[[p + 1]] t2sigma[p], {p, 0, 3}];
   mdb = Sum[I If[p == 0, 1, -1] kv[[p + 1]] t2sigma[p], {p, 0, 3}];
   nk = kv[[1]]^2 - kv[[2]]^2 - kv[[3]]^2 - kv[[4]]^2;
   worst7 = Max[worst7, fro[md . mdb + nk IdentityMatrix[2]]]],
  {50}];
SL31Check["GS-7", "the super-loop form of the Dirac operator: gamma^p nabla_p has the Weyl blocks M_LL(D) and M_LL(conj D) and no diagonal blocks; (gamma.nabla)^2 = eta nabla nabla + (1/4)[gamma^p, gamma^q][nabla_p, nabla_q]; for a free plane wave M_LL(D) M_LL(conj D) = -N_LL(k) sigma_0 (50 random momenta and fields; tolerance 1e-12)",
  worst7 < 1*^-12, Row[{"largest deviation ", worst7, " (original: 1.6e-15)"}]]

(* ::Input:: *)
(* GS-8: the operator H_DB(q) of a plane wave with constant gauge fields: Hermitian and particle-hole symmetric;
   the spectrum of the polar state +-E^(+-), each four times *)
mass = 0.4; mu = 0.3;
hq[q_, acal_, v0_, d_, mu_] := Module[{hp, hh, p},
   hp = mass t2lift4[t2beta] - v0 - mu IdentityMatrix[8] + Sum[t2lift4[t2alpha[j]] . (q[[j]] IdentityMatrix[8] - acal[[j]]), {j, 3}];
   hh = mass t2lift4[t2beta] - Conjugate[v0] - mu IdentityMatrix[8] - Sum[Conjugate[t2lift4[t2alpha[j]]] . (q[[j]] IdentityMatrix[8] + Conjugate[acal[[j]]]), {j, 3}];
   p = t2pairPotential[d];
   ArrayFlatten[{{hp, p}, {ConjugateTranspose[p], -hh}}]];
swap = ArrayFlatten[{{0, IdentityMatrix[8]}, {IdentityMatrix[8], 0}}];
worst9 = 0.;
Do[
  Module[{q = Table[2 t2Unit[], {3}], acal, v0, d, hm, hmq},
   acal = Table[Module[{a = Table[t2Unit[], {3}]}, gauge8[gv, gf, a, t2Unit[]]], {3}];
   v0 = Module[{a = Table[t2Unit[], {3}]}, gauge8[gv, gf, a, t2Unit[]]];
   d = t2RndVec[3];
   hm = hq[q, acal, v0, d, mu];
   worst9 = Max[worst9, fro[hm - ConjugateTranspose[hm]]];
   hmq = hq[-q, acal, v0, d, mu];
   worst9 = Max[worst9, fro[swap . Conjugate[hm] . swap + hmq]]],
  {50}];
zero3 = {ConstantArray[0, {8, 8}], ConstantArray[0, {8, 8}], ConstantArray[0, {8, 8}]};
worst9s = 0.;
Do[
  Module[{q = Table[2 t2Unit[], {3}], delta, hm, ev, eps, ep, em, expect},
   delta = 0.25 + 0.2 t2Unit[];
   hm = hq[q, zero3, ConstantArray[0, {8, 8}], {0, 0, delta}, mu];
   ev = First[t2eigh[hm]];
   eps = Sqrt[q . q + mass^2];
   ep = Sqrt[(eps + mu)^2 + delta^2]; em = Sqrt[(eps - mu)^2 + delta^2];
   expect = Sort[Join[ConstantArray[-ep, 4], ConstantArray[-em, 4], ConstantArray[em, 4], ConstantArray[ep, 4]]];
   worst9s = Max[worst9s, Max[Abs[ev - expect]]]],
  {30}];
SL31Check["GS-8", "the Dirac-Bogoliubov-de Gennes operator H_DB(q) with constant gauge fields is Hermitian and particle-hole symmetric (50 random momenta, fields and order parameters); for the polar order parameter (0, 0, Delta) its spectrum is +-E^(+-), each four times (30 random momenta; m = 0.4, mu = 0.3; tolerance 1e-10)",
  worst9 < 1*^-10 && worst9s < 1*^-10, Row[{"largest deviations ", worst9, " and ", worst9s, " (original: 0.0e0 and 6.7e-15)"}]]

(* ::Input:: *)
(* GS-9: the mode-sum form of the gap equation: sum over the positive modes of psi_p^T Kch_3 conj(psi_h) = -2 Delta (1/E^+ + 1/E^-) *)
k3 = t2kpair[3];
worstGap = 0.;
Do[
  Module[{q = Table[2 t2Unit[], {3}], delta, hm, ev, vecs, s, eps, ep, em},
   delta = 0.25 + 0.2 t2Unit[];
   hm = hq[q, zero3, ConstantArray[0, {8, 8}], {0, 0, delta}, mu];
   {ev, vecs} = t2eigh[hm];
   s = Sum[If[ev[[n]] > 0, vecs[[1 ;; 8, n]] . k3 . Conjugate[vecs[[9 ;; 16, n]]], 0], {n, 16}];
   eps = Sqrt[q . q + mass^2];
   ep = Sqrt[(eps + mu)^2 + delta^2]; em = Sqrt[(eps - mu)^2 + delta^2];
   worstGap = Max[worstGap, Abs[s + 2 delta (1/ep + 1/em)]]],
  {30}];
SL31Check["GS-9", "the self-consistency of the mean field: for the polar order parameter the sum over the eight positive-energy eigenvectors of psi_p^T Kch_3 conj(psi_h) equals -2 Delta (1/E^+ + 1/E^-), so that the mode-sum form of the mean-field equations gives the gap equation 1/G = int d^3q/(2 pi)^3 (1/E^+ + 1/E^-) (30 random momenta; tolerance 1e-10)",
  worstGap < 1*^-10, Row[{"largest deviation ", worstGap, " (original: 1.7e-15)"}]]

(* ::Subsection:: *)
(*GS-10 and GS-11: the lattice operator and the eigensolver*)

(* ::Input:: *)
(* GS-10: a 3 x 2 x 4 lattice with random links, potentials and order parameters *)
lat = <|"n" -> {3, 2, 4}, "ell" -> 0.7, "periodic" -> {True, False, True}, "wilson" -> 1.0, "mass" -> mass, "mu" -> mu|>;
ns = t2nsites[lat];
fl = t2trivialFields[ns];
Do[
  Do[fl[["links", s, j]] = Module[{a = Table[t2Unit[], {3}]}, t2link[gv, gf, a, t2Unit[], lat["ell"]]], {j, 3}];
  fl[["v0", s]] = Module[{a = Table[t2Unit[], {3}]}, t2potential[gv, gf, a, t2Unit[]]];
  fl[["d", s]] = t2RndVec[3],
  {s, ns}];
hl = t2dense[lat, fl];
herm = fro[hl - ConjugateTranspose[hl]];
n = t2dim[lat];
xm = SparseArray[Flatten[Table[{{16 s + kk, 16 s + 8 + kk} -> 1, {16 s + 8 + kk, 16 s + kk} -> 1}, {s, 0, ns - 1}, {kk, 8}], 2], {n, n}];
ph = fro[xm . Conjugate[hl] . xm + hl];
(* gauge covariance under random site transformations G(s) = exp(i th) U(s) *)
gs = {}; fl2 = fl; gbig = SparseArray[{}, {n, n}, 0. I];
Do[
  Module[{th = 3 t2Unit[], u2, gm, r, dn, g8},
   u2 = t2su2[Table[3 t2Unit[], {3}]];
   gm = Exp[I th] u2;
   AppendTo[gs, gm];
   r = t2adjoint[u2];
   dn = Table[Sum[fl[["d", s + 1, b]] r[[a, b]], {b, 3}] Exp[2 I th], {a, 3}];
   fl2[["d", s + 1]] = dn;
   g8 = t2lift2[gm];
   gbig[[16 s + 1 ;; 16 s + 8, 16 s + 1 ;; 16 s + 8]] = g8;
   gbig[[16 s + 9 ;; 16 s + 16, 16 s + 9 ;; 16 s + 16]] = Conjugate[g8]],
  {s, 0, ns - 1}];
Do[
  fl2[["v0", s + 1]] = gs[[s + 1]] . fl[["v0", s + 1]] . ConjugateTranspose[gs[[s + 1]]];
  Do[With[{sp = t2neighbour[lat, s, j, True]},
    fl2[["links", s + 1, j + 1]] = gs[[s + 1]] . fl[["links", s + 1, j + 1]] . ConjugateTranspose[If[sp === None, IdentityMatrix[2], gs[[sp + 1]]]]], {j, 0, 2}],
  {s, 0, ns - 1}];
hl2 = t2dense[lat, fl2];
gcov = fro[hl2 - gbig . hl . ConjugateTranspose[gbig]];
(* the free one-dimensional lattice: the spectrum +-eps_lat, each eight times; the doubler *)
lat1 = <|"n" -> {1, 1, 12}, "ell" -> 0.5, "periodic" -> {True, True, True}, "wilson" -> 1.0, "mass" -> mass, "mu" -> 0.|>;
ev1 = First[t2eigh[t2dense[lat1, t2trivialFields[12]]]];
expect1 = Sort[Flatten[Table[With[{e = t2epsLat[{0., 0., 2 Pi kk/(12 0.5)}, 0.5, mass, 1.0, {False, False, True}]}, Table[{e, -e}, {8}]], {kk, 0, 11}]]];
disp = Max[Abs[ev1 - expect1]];
doublerOk = Abs[t2epsLat[{0., 0., Pi/0.5}, 0.5, mass, 1.0, {False, False, True}] - (mass + 2/0.5)] < 1*^-12;
(* the spectral radius and the bound used for the time step *)
rho = Max[Abs[First[t2eigh[hl]]]];
bound = t2normBound[lat, fl];
SL31Check["GS-10", "the lattice operator H_lat on a 3 x 2 x 4 lattice (dx = 0.7, r_W = 1, periodic in x and z) with random links, potentials and order parameters is Hermitian, particle-hole symmetric and exactly gauge covariant; on a free periodic chain of 12 sites its spectrum is +-eps_lat, each eight times; the Wilson term lifts the doubler to m + 2 r_W/dx; the spectral radius stays below the bound Hbound (tolerance 1e-10)",
  herm < 1*^-10 && ph < 1*^-10 && gcov < 1*^-10 && disp < 1*^-10 && doublerOk && rho <= bound,
  Row[{"Hermiticity ", herm, ", particle-hole ", ph, ", gauge covariance ", gcov, ", dispersion ", disp, "; spectral radius ", NumberForm[rho, {6, 4}], " <= bound ", NumberForm[bound, {6, 4}],
    " (original: 0.0e0, 0.0e0, 1.8e-13, 1.4e-14; 8.4349 <= 17.5474)"}]]

(* ::Input:: *)
(* GS-11: the rewritten eigensolver on random Hermitian matrices of the dimensions 7, 64 and 200 *)
worst11 = 0.;
Do[
  Module[{a = ConstantArray[0. I, {nn, nn}], ev, v},
   Do[Module[{z = If[i == j, t2Unit[] + 0. I, t2UnitC[]]}, a[[i, j]] = z; a[[j, i]] = Conjugate[z]], {i, nn}, {j, i}];
   {ev, v} = t2eigh[a];
   worst11 = Max[worst11, fro[v . DiagonalMatrix[ev] . ConjugateTranspose[v] - a]/fro[a], fro[ConjugateTranspose[v] . v - IdentityMatrix[nn]]]],
  {nn, {7, 64, 200}}];
SL31Check["GS-11", "the Hermitian eigensolver of t2core (Householder reduction and implicit QL), rewritten above: for random Hermitian matrices of the dimensions 7, 64 and 200 the eigenvectors are orthonormal and reproduce the matrix (relative tolerance 1e-11)",
  worst11 < 1*^-11, Row[{"largest relative residual ", worst11, " (original: 6.0e-14)"}]]
