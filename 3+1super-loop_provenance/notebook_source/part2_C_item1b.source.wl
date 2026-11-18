(* ::Subsection:: *)
(*GS-12: the explicit matrices and the component equations*)

(* ::Text:: *)
(*The original program prints every matrix and every component equation of the field equations in TeX, with its small symbolic algebra, and evaluates each printed expression for numbers assigned to its symbols, comparing it with the $8\times8$ and $16\times16$ matrix computation (check GS-12). Here the same expressions are built from symbols of the Wolfram Language, shown, and evaluated in the same way. The symbols: $d^\iota_{\rm LL}$ (the order parameter), $\omega^j_{{\rm LL},p}$ and $\varpi^j_{{\rm LL},p}$ (the rotation and boost parts of the Lorentz connection in the direction $p$), $A^\iota_{{\rm LL},p}$ and $A^{\rm F}_{{\rm LL},p}$ (the gauge potentials), $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ (the couplings), $\psi^{\rm p}_{{\rm LL},k}$, $\psi^{\rm h}_{{\rm LL},k}$, $\Psi_{{\rm LL},k}$ ($k=1,\dots,8$, the components), $m_{\rm LL}$, $\mu_{\rm LL}$, $G_{\rm LL}$ and $\Pi^\iota_{\rm LL}$ (the pair operators). They are ordinary symbols with formatting rules, so that they print as in the documents.*)

(* ::Input:: *)
(* the symbols of the formulas and the way they print *)
Format[dS[i_]] := Subsuperscript["d", "LL", i];
Format[dStarS[i_]] := Superscript[Subsuperscript["d", "LL", i], "*"];
Format[omS[j_, idx_]] := Subsuperscript["\[Omega]", Row[{"LL,", idx}], j];
Format[vpS[j_, idx_]] := Subsuperscript["\[CurlyPi]", Row[{"LL,", idx}], j];
Format[afS[idx_]] := Subsuperscript["A", Row[{"LL,", idx}], "F"];
Format[avS[i_, idx_]] := Subsuperscript["A", Row[{"LL,", idx}], i];
Format[gvS] := Subsuperscript["g", "LL", "v"];
Format[gfS] := Subsuperscript["g", "LL", "F"];
Format[psP[k_]] := Subsuperscript["\[Psi]", Row[{"LL,", k}], "p"];
Format[psH[k_]] := Subsuperscript["\[Psi]", Row[{"LL,", k}], "h"];
Format[psiS[k_]] := Subscript["\[CapitalPsi]", Row[{"LL,", k}]];
Format[psiStarS[k_]] := Subsuperscript["\[CapitalPsi]", Row[{"LL,", k}], "*"];
Format[piS[i_]] := Subsuperscript["\[CapitalPi]", "LL", i];
Format[mS] := Subscript["m", "LL"]; Format[muS] := Subscript["\[Mu]", "LL"]; Format[bigGS] := Subscript["G", "LL"];
Format[djS[x_]] := Row[{Subscript["\[PartialD]", "j"], x}];
Format[nabPS[j_, k_]] := Subscript[Row[{"(", Subsuperscript["\[Del]", j, "LL"], Subsuperscript["\[Psi]", "LL", "p"], ")"}], k];
Format[nabHS[j_, k_]] := Subscript[Row[{"(", Subsuperscript["\[Del]", j, "LL*"], Subsuperscript["\[Psi]", "LL", "h"], ")"}], k];
Format[nabPsiS[j_, k_]] := Subscript[Row[{"(", Subsuperscript["\[Del]", j, "LL"], Subscript["\[CapitalPsi]", "LL"], ")"}], k];
Format[nablaS[p_]] := Subsuperscript["\[Del]", p, "LL"];
(* the complex conjugate of the explicit numbers of an expression (the symbols are real) *)
conjC[e_] := e /. Complex[x_, y_] :> Complex[x, -y];

(* ::Input:: *)
(* the pair potential P_LL with symbolic d^iota: it is block diagonal with two equal 4 x 4 blocks *)
dval = {0.31 - 0.2 I, -0.17 + 0.4 I, 0.52 + 0.11 I};
pSym = Sum[dS[i] ConjugateTranspose[t2kpair[i]], {i, 3}];
pdev = Max[Abs[(pSym /. dS[i_] :> dval[[i]]) - t2pairPotential[dval]]];
pdev = Max[pdev, Max[Table[With[{kd = ConjugateTranspose[t2kpair[i]]},
       fro[kd[[1 ;; 4, 1 ;; 4]] - kd[[5 ;; 8, 5 ;; 8]]] + fro[kd[[1 ;; 4, 5 ;; 8]]] + fro[kd[[5 ;; 8, 1 ;; 4]]]], {i, 3}]]];
(* the Lorentz connection in the direction p: the 4 x 4 matrix sgen(u_p) and its two Weyl blocks *)
omv = {0.3, -0.7, 0.45}; vpv = {0.12, 0.5, -0.33};
gm = Sum[omS[j, "p"] sj[[j]] + vpS[j, "p"] sk[[j]], {j, 3}];
gnum = t2spinGen[lorentzElem[omv, vpv]];
valsG = Join[Table[omS[j, "p"] -> omv[[j]], {j, 3}], Table[vpS[j, "p"] -> vpv[[j]], {j, 3}]];
bl = Sum[omS[j, "p"] t2mq[t2jgen[j]] + vpS[j, "p"] t2mq[t2kgen[j]], {j, 3}];
bu = Sum[-omS[j, "p"] ConjugateTranspose[t2mq[t2jgen[j]]] - vpS[j, "p"] ConjugateTranspose[t2mq[t2kgen[j]]], {j, 3}];
pdev = Max[pdev, Max[Abs[(gm /. valsG) - gnum]], Max[Abs[(bu /. valsG) - gnum[[1 ;; 2, 1 ;; 2]]]],
   Max[Abs[(bl /. valsG) - gnum[[3 ;; 4, 3 ;; 4]]]], Max[Abs[gnum[[1 ;; 2, 3 ;; 4]]] + Abs[gnum[[3 ;; 4, 1 ;; 2]]]]];
Column[{
  Row[{LL["\\mathsf P_{\\rm LL}"], " = ", LL["\\sigma^{\\rm LL}_j", "j" -> "0"], " \[CircleTimes] ", MatrixForm[pSym[[1 ;; 4, 1 ;; 4]]]}],
  Row[{"\[Minus]", Superscript[Row[{LL["M_{\\rm LL}(\\mathbf b)"], "(", LL["\\mathbf u^\\omega_{{\\rm LL},p}"], ")"}], "\[Dagger]"], " = ", MatrixForm[bu]}],
  Row[{LL["M_{\\rm LL}(\\mathbf b)"], "(", LL["\\mathbf u^\\omega_{{\\rm LL},p}"], ")", " = ", MatrixForm[bl]}]}, Spacings -> 1.5]

(* ::Input:: *)
(* the gauge matrix Acal_p = gF AF_p 1 + gv A^iota_p T^iota and the 8 x 8 connection Gamma_p, with a generic index *)
aVal[i_, p_] := 0.37 i - 0.21 p + 0.05 (i - 1) p;
afVal[p_] := 0.29 - 0.13 p;
acalSym[idx_] := gfS afS[idx] IdentityMatrix[8] + Sum[gvS avS[i, idx] t2tiso[i], {i, 3}];
acalNum[p_] := gauge8[gv, gf, Table[aVal[i, p], {i, 3}], afVal[p]];
omvP[p_] := {0.21 - 0.1 p, -0.34 + 0.07 p, 0.15 + 0.11 p};
vpvP[p_] := {0.08 + 0.05 p, 0.27 - 0.09 p, -0.19 + 0.04 p};
gamSym[idx_] := Sum[omS[j, idx] t2lift4[sj[[j]]] + vpS[j, idx] t2lift4[sk[[j]]], {j, 3}];
gamNum[p_] := t2lift4[t2spinGen[lorentzElem[omvP[p], vpvP[p]]]];
(* the values of the symbols of the generic index idx, taken from the direction p *)
valsAt[idx_, p_] := Join[{gvS -> gv, gfS -> gf, afS[idx] -> afVal[p]}, Table[avS[i, idx] -> aVal[i, p], {i, 3}],
   Table[omS[j, idx] -> omvP[p][[j]], {j, 3}], Table[vpS[j, idx] -> vpvP[p][[j]], {j, 3}]];
pdev = Max[pdev, Max[Abs[(gamSym["p"] /. valsAt["p", 2]) - gamNum[2]]]];
Column[{Row[{Subscript[Row[{"(", LL["\\mathsf A_{{\\rm LL},p}", "p" -> "j"], ")"}], "11"], " = ", acalSym["j"][[1, 1]]}],
  Row[{Subscript[Row[{"(", LL["\\Gamma^{\\rm LL}_p"], ")"}], "11"], " = ", gamSym["p"][[1, 1]]}]}]

(* ::Input:: *)
(* the components of the covariant derivatives, for a plane wave exp(i q.x) and the fields of the direction j = 1 *)
qv = {0., 0.83, -0.41, 0.66};
wv = t2RndVec[16];
wp = wv[[1 ;; 8]]; whh = wv[[9 ;; 16]];
nabP[j_] := (I qv[[j + 1]] IdentityMatrix[8] + gamNum[j] - I acalNum[j]) . wp;
nabH[j_] := (I qv[[j + 1]] IdentityMatrix[8] + Conjugate[gamNum[j]] + I Conjugate[acalNum[j]]) . whh;
expP = Table[djS[psP[kk]] + Sum[gamSym["j"][[kk, l]] psP[l], {l, 8}] - I Sum[acalSym["j"][[kk, l]] psP[l], {l, 8}], {kk, 8}];
expH = Table[djS[psH[kk]] + Sum[conjC[gamSym["j"][[kk, l]]] psH[l], {l, 8}] + I Sum[conjC[acalSym["j"][[kk, l]]] psH[l], {l, 8}], {kk, 8}];
expPsi = Table[djS[psiS[kk]] + Sum[gamSym["j"][[kk, l]] psiS[l], {l, 8}] - I Sum[acalSym["j"][[kk, l]] psiS[l], {l, 8}], {kk, 8}];
valsPW = Join[valsAt["j", 1], Table[psP[l] -> wp[[l]], {l, 8}], Table[psH[l] -> whh[[l]], {l, 8}], Table[psiS[l] -> wp[[l]], {l, 8}],
   Table[djS[psP[l]] -> I qv[[2]] wp[[l]], {l, 8}], Table[djS[psH[l]] -> I qv[[2]] whh[[l]], {l, 8}], Table[djS[psiS[l]] -> I qv[[2]] wp[[l]], {l, 8}]];
eqdev2 = Max[Abs[(expP /. valsPW) - nabP[1]], Abs[(expH /. valsPW) - nabH[1]], Abs[(expPsi /. valsPW) - nabP[1]]];
Column[Table[Row[{Subscript[Row[{"(", Subsuperscript["\[Del]", "j", "LL"], Subsuperscript["\[Psi]", "LL", "p"], ")"}], kk], " = ", expP[[kk]]}], {kk, 1, 2}], Spacings -> 1]

(* ::Input:: *)
(* the sixteen equations of the Dirac-Bogoliubov-de Gennes system, written with the components *)
psPv = Table[psP[l], {l, 8}]; psHv = Table[psH[l], {l, 8}];
pDagSym = Transpose[conjC[pSym] /. dS[i_] :> dStarS[i]];
rhsP = Sum[(-I t2lift4[t2alpha[j]]) . Table[nabPS[j, kk], {kk, 8}], {j, 3}] + mS t2lift4[t2beta] . psPv - acalSym["0"] . psPv -
   I gamSym["0"] . psPv - muS psPv + pSym . psHv;
rhsH = -Sum[(I Conjugate[t2lift4[t2alpha[j]]]) . Table[nabHS[j, kk], {kk, 8}], {j, 3}] - mS t2lift4[t2beta] . psHv + conjC[acalSym["0"]] . psHv -
   I conjC[gamSym["0"]] . psHv + muS psHv + pDagSym . psPv;
valsBdG = Join[valsAt["0", 0], Table[psP[l] -> wp[[l]], {l, 8}], Table[psH[l] -> whh[[l]], {l, 8}],
   Flatten[Table[{nabPS[j, kk] -> nabP[j][[kk]], nabHS[j, kk] -> nabH[j][[kk]]}, {j, 3}, {kk, 8}]],
   {mS -> mass, muS -> mu}, Table[dS[i] -> dval[[i]], {i, 3}], Table[dStarS[i] -> Conjugate[dval[[i]]], {i, 3}]];
(* the matrix form for the same plane wave, with the connection and the potentials of every direction *)
hqG = Module[{hp, hh, p = t2pairPotential[dval]},
   hp = mass t2lift4[t2beta] - I gamNum[0] - acalNum[0] - mu IdentityMatrix[8] +
     Sum[-I t2lift4[t2alpha[j]] . (I qv[[j + 1]] IdentityMatrix[8] + gamNum[j] - I acalNum[j]), {j, 3}];
   hh = mass t2lift4[t2beta] + I Conjugate[gamNum[0]] - Conjugate[acalNum[0]] - mu IdentityMatrix[8] +
     Sum[I Conjugate[t2lift4[t2alpha[j]]] . (I qv[[j + 1]] IdentityMatrix[8] + Conjugate[gamNum[j]] + I Conjugate[acalNum[j]]), {j, 3}];
   ArrayFlatten[{{hp, p}, {ConjugateTranspose[p], -hh}}]];
hw = hqG . wv;
eqdev = Max[Abs[(rhsP /. valsBdG) - hw[[1 ;; 8]]], Abs[(rhsH /. valsBdG) - hw[[9 ;; 16]]]];
Column[{Row[{I Subscript["\[PartialD]", 0], psP[1], " = ", rhsP[[1]]}], Row[{I Subscript["\[PartialD]", 0], psH[1], " = ", rhsH[[1]]}]}, Spacings -> 1]

(* ::Input:: *)
(* the pair operators Pi^iota = Psi^T Kch_iota Psi written out (only a < b: the components anticommute) *)
prodVal[a_, b_] := 0.1 a + I (0.07 (b - 1) - 0.2);
piSym = Table[Sum[(k[[i, a, b]] - k[[i, b, a]]) psiS[a] psiS[b], {a, 8}, {b, a + 1, 8}], {i, 3}];
pidev = Max[Table[Abs[(piSym[[i]] /. psiS[a_] psiS[b_] :> prodVal[a, b]) - Sum[(k[[i, a, b]] - k[[i, b, a]]) prodVal[a, b], {a, 8}, {b, a + 1, 8}]], {i, 3}]];
Column[Table[Row[{piS[i], " = ", piSym[[i]]}], {i, 3}], Spacings -> 1]

(* ::Input:: *)
(* the eight component equations of the exact operator equation, with independent random values for the symbols *)
nabPsiVal = Table[t2RndVec[8], {3}];
psiVal = t2RndVec[8];
psiStarVal = t2RndVec[8];
piVal = t2RndVec[3];
gVal = 1.3;
psiV = Table[psiS[l], {l, 8}];
refExact = (mass t2lift4[t2beta] - I gamNum[0] - acalNum[0] - mu IdentityMatrix[8]) . psiVal +
   Sum[(-I t2lift4[t2alpha[j]]) . nabPsiVal[[j]], {j, 3}] - Sum[(ConjugateTranspose[k[[i]]] . psiStarVal) piVal[[i]] 0.5 gVal, {i, 3}];
exactEq = Sum[(-I t2lift4[t2alpha[j]]) . Table[nabPsiS[j, kk], {kk, 8}], {j, 3}] + mS t2lift4[t2beta] . psiV - acalSym["0"] . psiV -
   I gamSym["0"] . psiV - muS psiV - bigGS/2 Sum[(ConjugateTranspose[k[[i]]] . Table[psiStarS[l], {l, 8}]) piS[i], {i, 3}];
valsExact = Join[valsAt["0", 0], Table[psiS[l] -> psiVal[[l]], {l, 8}], Table[psiStarS[l] -> psiStarVal[[l]], {l, 8}],
   Table[piS[i] -> piVal[[i]], {i, 3}], Flatten[Table[nabPsiS[j, kk] -> nabPsiVal[[j, kk]], {j, 3}, {kk, 8}]], {mS -> mass, muS -> mu, bigGS -> gVal}];
eqdev3 = Max[Abs[(exactEq /. valsExact) - refExact]];
(* the super-loop Weyl blocks M_LL(D) and M_LL(conj D) as 2 x 2 operator matrices *)
mdSym = Sum[t2mq[t2canonicalFrame[[p + 1]]] nablaS[p], {p, 0, 3}];
mdbSym = Sum[t2mq[t2bqConj[t2canonicalFrame[[p + 1]]]] nablaS[p], {p, 0, 3}];
Column[{Row[{I Subscript["\[PartialD]", 0], psiS[1], " = ", exactEq[[1]]}], Row[{LL["M_{\\rm LL}(\\mathbf b)"], "(", LL["\\mathbf D_{\\rm LL}"], ") = ", MatrixForm[mdSym], ",   ", LL["M_{\\rm LL}(\\mathbf b)"], "(", OverBar[LL["\\mathbf D_{\\rm LL}"]], ") = ", MatrixForm[mdbSym]}]}, Spacings -> 1]

(* ::Input:: *)
SL31Check["GS-12", "the printed formulas agree with the matrices that they describe: the entries of Ppair (symbolic d), of the blocks of sgen(u_p) and of Gamma_p, the expansions of the covariant derivatives, the sixteen equations of the Dirac-Bogoliubov-de Gennes system, the pair operators Pi and the eight exact equations, each evaluated for numbers assigned to its symbols and compared with the matrix computation (tolerance 1e-12)",
  pdev < 1*^-12 && eqdev < 1*^-12 && eqdev2 < 1*^-12 && pidev < 1*^-12 && eqdev3 < 1*^-12,
  Row[{"largest deviations ", pdev, ", ", eqdev, ", ", eqdev2, ", ", pidev, ", ", eqdev3, " (original: 0.0e0, 4.0e-16, 3.4e-16, 0.0e0, 5.0e-16)"}]]

(* ::Subsection:: *)
(*GS-13: the time integration by IDA, compared with the exact solution*)

(* ::Text:: *)
(*The original program checks its time integration on the lattice of GS-10 ($3\times2\times4$ sites with random links, potentials and pair fields; $n^{\rm cpx}_{\rm LL}=384$). From a random initial state $w^{\rm lat}_{\rm LL}$ of the norm 1, drawn from a generator of its own (seed 2026111713, so that the random numbers of the later checks are not shifted), it integrates the lattice equations with CVODE up to $x^0_{\rm LL}=20$, with $r^{\rm tol}_{\rm LL}=10^{-8},10^{-10},10^{-12}$ and $a^{\rm tol}_{\rm LL}=r^{\rm tol}_{\rm LL}/100$, and compares the states at the 41 output times $x^0_{\rm LL}=0,0.5,\dots,20$ with the exact solution $w^{\rm ex}_{\rm LL}$: the sum over the eigenvectors of $\mathcal H^{\rm lat}_{\rm LL}$ of the eigenvector times its scalar product with the initial state times the phase factor of the angle $-E^{\rm qp}_{{\rm LL},n}x^0_{\rm LL}$ (eigensolver `t2eigh`). The quantity compared is the relative deviation $\delta^{\rm rel}_{\rm LL}$, the norm of the difference between the computed and the exact state divided by the norm of the exact state. The notebook repeats the check with IDA in place of CVODE (functions `t2evolveIDA` and `t2localErrorsIDA` of chapter II.1.5): the same lattice, the same initial state, the same tolerances, the same output times.*)

(* ::Text:: *)
(*The check has four parts, for each of the three tolerances. (1) The exact local error $\delta^{\rm loc}_{\rm LL}$ of every internal step is at most $B^{\rm loc}_{\rm LL}=r^{\rm tol}_{\rm LL}$ times the norm of the state plus $a^{\rm tol}_{\rm LL}\sqrt{2n^{\rm cpx}_{\rm LL}}$. Because the evolution is unitary, the deviation at the end of every internal step is then at most $B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$ (chapter II.1.5). (2) The states of `NDSolve` at the output times are the cubic Hermite polynomials of the stored values and derivatives (tolerance $10^{-12}$), so that the bound $B^{\rm out}_{\rm LL}$ of chapter II.1.5 holds for them, and the largest $\delta^{\rm rel}_{\rm LL}$ is at most $B^{\rm out}_{\rm LL}$ (the exact state has the norm 1, so the relative deviation is the deviation). (3) The largest deviation decreases from $r^{\rm tol}_{\rm LL}=10^{-8}$ to $10^{-10}$ to $10^{-12}$. (4) The two requirements of the original check for $r^{\rm tol}_{\rm LL}=10^{-12}$: the largest deviation stays below $10^{-9}$, and the norm of the state, which the exact evolution conserves, changes by less than $10^{-10}$. The printed line also gives the numbers of internal steps $N^{\rm st}_{\rm LL}$ and of evaluations $N^{\rm ev}_{\rm LL}$ of the right side, the largest internal steps, and the results of CVODE from the report of the original program.*)

(* ::Input:: *)
(* GS-13: the initial state from a generator of its own (seed 2026111713).  The state of the main generator is kept
   and restored, so that the random numbers of the later checks are those of the original program *)
rngKept13 = t2rngState;
t2Seed[2026111713];
w0IDA13 = t2RndVec[t2dim[lat]];
t2rngState = rngKept13;
w0IDA13 = w0IDA13/Norm[w0IDA13];
(* the exact solution: the eigenvalues and eigenvectors of H_lat (t2eigh: ascending eigenvalues, the eigenvectors in the
   columns) and the spectral sum *)
{e13, v13} = t2eigh[hl];
eMax13 = Max[Abs[e13]];
coef13 = ConjugateTranspose[v13] . w0IDA13;
wExact13[x0_?NumericQ] := v13 . (Exp[-I e13 x0] coef13);
touts13 = Range[0., 20., 0.5];
exact13 = wExact13 /@ touts13;
rtols13 = {1.*^-8, 1.*^-10, 1.*^-12};
(* the three integrations by IDA, atol = rtol/100, each with the measurement of the local errors of all its steps *)
{time13, runs13} = AbsoluteTiming[Table[t2evolveIDA[hl, w0IDA13, touts13, rt, rt/100, t2localErrorsIDA[e13, v13, rt, rt/100]], {rt, rtols13}]];
dev13 = Table[MapThread[Norm[#1 - #2]/Norm[#2] &, {runs13[[k, "states"]], exact13}], {k, 3}];
maxdev13 = Max /@ dev13;
normChange13 = Table[Max[Abs[Norm /@ runs13[[k, "states"]] - 1]], {k, 3}];
steps13 = runs13[[All, "steps"]];
evals13 = runs13[[All, "evaluations"]];
hmax13 = runs13[[All, "hmax"]];
ratio13 = runs13[[All, "analysis", "largestRatio"]];
norm13 = runs13[[All, "analysis", "largestNorm"]];
resid13 = Max /@ runs13[[All, "residualAt"]];
hermite13 = Max /@ runs13[[All, "hermiteAt"]];
(* B^loc = rtol (largest norm of a state) + atol sqrt(2 n^cpx); B^err = N^st B^loc; B^out as in chapter II.1.5 *)
bloc13 = Table[rtols13[[k]] norm13[[k]] + rtols13[[k]]/100 Sqrt[2. t2dim[lat]], {k, 3}];
berr13 = steps13 bloc13;
bout13 = Table[(1 + hmax13[[k]] eMax13/4) berr13[[k]] + hmax13[[k]] resid13[[k]]/4 + (hmax13[[k]] eMax13)^4/384 norm13[[k]], {k, 3}];
sci13[l_List] := Row[Riffle[Map[ScientificForm[#, 2] &, l], ", "]];
(* the numbers of internal steps of CVODE for the three tolerances, from the report of the original program *)
cvSteps13 = {1528, 3661, 3804};
SL31Check["GS-13", "time integration: the lattice equations (i times the derivative of the Nambu mode w_lat with respect to the time x_LL^0 equals H_lat w_lat) are integrated by IDA (SUNDIALS, through NDSolve with Method -> \"IDA\": backward differentiation formulas of the orders 1 to 5 with local error control and Newton's method; the tolerance of every real unknown is rtol times its modulus plus atol) in place of CVODE of the original program; on the 3 x 2 x 4 lattice of GS-10 (384 complex components), from the random initial state of the norm 1 of the original program, for each of rtol = 1e-8, 1e-10, 1e-12 (atol = rtol/100): (1) the exact local error local_error of every internal step is at most budget_step = rtol (norm of the state) + atol sqrt(768); (2) the states at the 41 output times x_LL^0 = 0, 0.5, ..., 20 are the cubic Hermite polynomials of the stored values and derivatives (tolerance 1e-12), and their largest relative deviation from the exact spectral solution w_exact is at most budget_out = (1 + h_IDA E_max/4) budget + h_IDA deriv_residual/4 + (h_IDA E_max)^4/384 with budget = N_steps budget_step; (3) the largest deviation decreases from rtol = 1e-8 to 1e-10 to 1e-12; (4) for rtol = 1e-12 it stays below 1e-9, and the norm of the state changes by less than 1e-10",
  And @@ Thread[ratio13 <= 1] && Max[hermite13] < 1*^-12 && And @@ Thread[maxdev13 <= bout13] && maxdev13[[1]] > maxdev13[[2]] > maxdev13[[3]] &&
   maxdev13[[3]] < 1*^-9 && normChange13[[3]] < 1*^-10,
  Row[{"for rtol = 1e-8, 1e-10, 1e-12: largest local_error/budget_step ", sci13[ratio13], "; largest relative deviations ", sci13[maxdev13], " <= budget_out ", sci13[bout13],
    " (budget ", sci13[berr13], "); largest norm changes ", sci13[normChange13], "; N_steps ", Row[steps13, ", "], "; N_evals ", Row[evals13, ", "], "; largest h_IDA ", sci13[hmax13],
    "; largest deriv_residual ", sci13[resid13], "; Hermite polynomials within ", sci13[hermite13], "; E_max ", NumberForm[eMax13, {6, 4}], "; computing time ", Round[time13, 0.1],
    " s (original program, CVODE: largest relative deviations 1.2e-6, 1.7e-7, 1.7e-10, budgets 2.6e-5, 6.2e-7, 6.4e-9, norm changes 5.2e-7, 9.8e-8, 5.9e-11, steps 1528, 3661, 3804)"}]]

(* ::Subsection:: *)
(*GS-14: the super-loop form of the field equations*)

(* ::Text:: *)
(*The Weyl isodoublet of a spinor charge $\mathbf y\mathbf e_4$ (with a bi-quaternion $\mathbf y$) is $\psi'_{\rm LL}(\mathbf y)=\mathsf E_{\rm LL}^{-1}M_{\rm LL}(\mathbf y)^T$, and the Dirac isodoublet $\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]$ of the two spinor charges $\mathbf b^\pm_{\rm LL}\mathbf e_4$ has the upper block $\psi'_{\rm LL}(\mathbf b^-_{\rm LL})$ (components 1 to 4) and the lower block $\psi'_{\rm LL}(\mathbf b^+_{\rm LL})$ (components 5 to 8). The check shows, with random bi-quaternions: (a) a frame matrix acting on a Weyl isodoublet is a right multiplication of the spinor charge, $M_{\rm LL}(\mathbf x)\psi'_{\rm LL}(\mathbf y)=\psi'_{\rm LL}(\mathbf y\bar{\mathbf x})$, in agreement with the Cayley-Dickson product $(\mathbf a+\mathbf b\mathbf e_4)(\mathbf c+\mathbf d\mathbf e_4)=(\mathbf a\mathbf c-\bar{\mathbf d}\mathbf b)+(\mathbf d\mathbf a+\mathbf b\bar{\mathbf c})\mathbf e_4$, which gives $(\mathbf y\mathbf e_4)\mathbf x=(\mathbf y\bar{\mathbf x})\mathbf e_4$; (b) the Dirac matrices exchange the two spinor charges and multiply them by the frame, $\gamma^p_{\rm LL}[\mathbf f]\Psi_{\rm LL}[\mathbf b^-_{\rm LL},\mathbf b^+_{\rm LL}]=\Psi_{\rm LL}[\mathbf b^+_{\rm LL}\bar{\mathbf f}_p,\mathbf b^-_{\rm LL}\mathbf f_p]$; (c) the gauge group acts by the automorphisms of the super-loop: $\mathcal U_{\rm LL}$ with $\mathsf U_{\rm LL}=M_{\rm LL}(\mathbf v)$ replaces $\mathbf b^+_{\rm LL}$ by $e^{i\theta^{\rm F}_{\rm LL}}\mathbf v\mathbf b^+_{\rm LL}\bar{\mathbf g}$ and $\mathbf b^-_{\rm LL}$ by $e^{i\theta^{\rm F}_{\rm LL}}\mathbf v\mathbf b^-_{\rm LL}\mathbf g^\dagger$ (and $\tilde{\mathbf g}=\mathbf g^*$); (d) the covariant derivative acts through the derivations of $\mathbb B_{\rm LL}$: on $\mathbf b^+_{\rm LL}$ it is $\partial_p\mathbf b^+_{\rm LL}-\mathbf b^+_{\rm LL}\mathbf u^\omega_{{\rm LL},p}+g^{\rm v}_{\rm LL}\mathbf t^{\rm A}_{{\rm LL},p}\mathbf b^+_{\rm LL}-ig^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathbf b^+_{\rm LL}$ with $\mathbf t^{\rm A}_{{\rm LL},p}=\sum_\iota A^\iota_{{\rm LL},p}\mathbf e_\iota/2$, and on $\mathbf b^-_{\rm LL}$ the same with $\mathbf u^{\omega*}_{{\rm LL},p}$ in place of $\mathbf u^\omega_{{\rm LL},p}$; (e) the Dirac operator is the right multiplication by the frame, $\sum_p\gamma^p_{\rm LL}\nabla^{\rm LL}_p\Psi_{\rm LL}=\Psi_{\rm LL}\big[\sum_p(\nabla^{\rm LL}_p\mathbf b^+_{\rm LL})\bar{\mathbf f}_p,\sum_p(\nabla^{\rm LL}_p\mathbf b^-_{\rm LL})\mathbf f_p\big]$, the right multiplications $R^{\rm LL}_{\bar{\mathbf f}_p}$ and $R^{\rm LL}_{\mathbf f_p}$ applied to the covariant derivatives of the charges.*)

(* ::Input:: *)
rbq[] := Table[t2UnitC[], {4}];
zero4 = {0, 0, 0, 0};
bqDev[x_, y_] := Max[Abs[x - y]];
devA = 0.;
Do[Module[{x = rbq[], y, lhs, ycx, prod},
   y = rbq[];
   lhs = t2mq[x] . t2psiPrime[y];
   ycx = t2bqMul[y, t2bqConj[x]];
   devA = Max[devA, fro[lhs - t2psiPrime[ycx]]];
   prod = t2bioctMul[{zero4, y}, {x, zero4}];
   devA = Max[devA, bqDev[prod[[1]], zero4], bqDev[prod[[2]], ycx]]], {50}];
devB = 0.;
Do[Module[{f, gfr, bsm, bsp, psi},
   f = If[trial == 0, t2canonicalFrame, Table[Module[{x0 = t2Unit[]}, {x0, I t2Unit[], I t2Unit[], I t2Unit[]}], {4}]];
   gfr = t2gammasFromFrame[f];
   bsm = rbq[]; bsp = rbq[];
   psi = t2diracFromCharges[bsm, bsp];
   Do[devB = Max[devB, Norm[t2lift4[gfr[[p]]] . psi - t2diracFromCharges[t2bqMul[bsp, t2bqConj[f[[p]]]], t2bqMul[bsm, f[[p]]]]]], {p, 4}]],
  {trial, 0, 19}];
devC = 0.;
Do[Module[{mg = t2randSL2C[], gq, uu, vq, ph, bsm, bsp, big, lhs, m2, p2, rhs},
   gq = t2bqFrom[mg];
   uu = t2su2[Table[3 t2Unit[], {3}]];
   vq = t2bqFrom[uu];
   ph = Exp[I 3 t2Unit[]];
   bsm = rbq[]; bsp = rbq[];
   big = ph KroneckerProduct[t2spinRep[mg], uu];
   lhs = big . t2diracFromCharges[bsm, bsp];
   m2 = t2bqMul[t2bqMul[vq, bsm], t2bqDag[gq]];
   p2 = t2bqMul[t2bqMul[vq, bsp], t2bqConj[gq]];
   rhs = ph t2diracFromCharges[m2, p2];
   devC = Max[devC, Norm[lhs - rhs], Max[Abs[Im[vq]]], bqDev[t2bqFrom[t2inv2[ConjugateTranspose[mg]]], t2bqStar[gq]]]], {50}];
devD = 0.; devE = 0.;
Do[Module[{bsm = rbq[], bsp, psi, nabPsi = {}, nbp = {}, nbm = {}, lhsE, upNew, loNew},
   bsp = rbq[];
   psi = t2diracFromCharges[bsm, bsp];
   Do[Module[{dbp = rbq[], dbm, up, a, af, dpsi, lhs, ta, cov, np, nm},
      dbm = rbq[];
      up = Module[{om = Table[t2Unit[], {3}]}, lorentzElem[om, Table[t2Unit[], {3}]]];
      a = Table[t2Unit[], {3}]; af = t2Unit[];
      dpsi = t2diracFromCharges[dbm, dbp];
      lhs = dpsi + t2lift4[t2spinGen[up]] . psi - I gauge8[gv, gf, a, af] . psi;
      (* the super-loop form: d y - y u + g_v (tA y) - i g_F AF y on the spinor charge y e4 *)
      ta = {0, 0.5 a[[1]], 0.5 a[[2]], 0.5 a[[3]]};
      cov[y_, dy_, u_] := dy - t2bqMul[y, u] + gv t2bqMul[ta, y] - I gf af y;
      np = cov[bsp, dbp, up];
      nm = cov[bsm, dbm, t2bqStar[up]];
      devD = Max[devD, Norm[lhs - t2diracFromCharges[nm, np]]];
      AppendTo[nabPsi, lhs]; AppendTo[nbp, np]; AppendTo[nbm, nm]], {p, 0, 3}];
   lhsE = Sum[t2lift4[t2gamma[p]] . nabPsi[[p + 1]], {p, 0, 3}];
   upNew = Sum[t2bqMul[nbp[[p + 1]], t2bqConj[t2canonicalFrame[[p + 1]]]], {p, 0, 3}];
   loNew = Sum[t2bqMul[nbm[[p + 1]], t2canonicalFrame[[p + 1]]], {p, 0, 3}];
   devE = Max[devE, Norm[lhsE - t2diracFromCharges[upNew, loNew]]]], {30}];
SL31Check["GS-14", "the super-loop form of the Dirac isodoublet and of its field equations: (a) a frame matrix is the right multiplication of the spinor charge; (b) gamma^p[f] Psi(bSm, bSp) = Psi(bSp conj f_p, bSm f_p); (c) the gauge group acts by the automorphisms of the super-loop; (d) the covariant derivative acts through the derivations of B_LL; (e) the Dirac operator is sum_p R_(f_p) nabla_p (tolerance 1e-12, 1e-10 for (c))",
  devA < tol && devB < tol && devC < 1*^-10 && devD < tol && devE < tol,
  Row[{"largest deviations ", devA, ", ", devB, ", ", devC, ", ", devD, ", ", devE, " (original: 3.2e-16, 4.1e-16, 3.1e-15, 8.8e-16, 2.1e-15)"}]]

(* ::Subsection:: *)
(*GS-15: the Lorentz connection is fixed by the frame*)

(* ::Input:: *)
ulist = Table[Module[{om = Table[t2Unit[], {3}]}, lorentzElem[om, Table[t2Unit[], {3}]]], {5}];
mg15[x_] := t2expm[t2mq[ulist[[5]]] + Sum[x[[p]] t2mq[ulist[[p]]], {p, 4}]];
h15 = 1*^-4;
(* the connection -(d_p Sspin) Sspin^-1; the scalar factor is applied before the matrix product
   (Dot binds more strongly than the product with a number) *)
conn15[x_, p_] := (-(0.5/h15) (t2spinRep[mg15[shift[x, p, h15]]] - t2spinRep[mg15[shift[x, p, -h15]]])) . t2spinRep[t2inv2[mg15[x]]];
gam15[x_, q_] := With[{mg = mg15[x]}, t2spinRep[mg] . t2gamma[q] . t2spinRep[t2inv2[mg]]];
dev15form = 0.; dev15const = 0.; dev15curv = 0.;
Do[Module[{x = Table[t2Unit[], {4}], gp},
   gp = Table[conn15[x, p], {p, 0, 3}];
   Do[Module[{up = gp[[p + 1, 1 ;; 2, 1 ;; 2]], lo = gp[[p + 1, 3 ;; 4, 3 ;; 4]], off},
      off = fro[gp[[p + 1, 1 ;; 2, 3 ;; 4]]] + fro[gp[[p + 1, 3 ;; 4, 1 ;; 2]]];
      dev15form = Max[dev15form, off, fro[up + ConjugateTranspose[lo]], Abs[Tr[lo]]];
      Do[
       dev15const = Max[dev15const, fro[(gam15[shift[x, p, h15], q] - gam15[shift[x, p, -h15], q]) (0.5/h15) + comm[gp[[p + 1]], gam15[x, q]]]];
       If[q != p,
        dev15curv = Max[dev15curv, fro[(conn15[shift[x, p, h15], q] - conn15[shift[x, p, -h15], q]) (0.5/h15) -
            (conn15[shift[x, q, h15], p] - conn15[shift[x, q, -h15], p]) (0.5/h15) + comm[gp[[p + 1]], gp[[q + 1]]]]]],
       {q, 0, 3}]], {p, 0, 3}]], {20}];
(* uniqueness: the real-linear map u -> ([sgen(u), gamma^q])_q has the rank 6 on the six generators *)
cols15 = Map[Function[gen, Flatten[Table[Flatten[{Re[#], Im[#]} & /@ Flatten[comm[gen, t2gamma[q]]]], {q, 0, 3}]]], Join[sj, sk]];
gram15 = N[cols15 . Transpose[cols15]];
gev15 = First[t2eigh[gram15]];
SL31Check["GS-15", "the Lorentz connection is fixed by the frame: for an x-dependent Lorentz transformation the connection -(d_p Sspin) Sspin^-1 has the form sgen(u), the Dirac matrices of the transformed frame are covariantly constant and the curvature vanishes (20 random points; tolerance 1e-6); the 24 commutators [sgen(J^j), gamma^q], [sgen(K^j), gamma^q] have the rank 6 (smallest Gram eigenvalue > 1e-6)",
  dev15form < 1*^-6 && dev15const < 1*^-6 && dev15curv < 1*^-6 && gev15[[1]] > 1*^-6,
  Row[{"largest deviations ", dev15form, ", ", dev15const, ", ", dev15curv, "; smallest Gram eigenvalue ", NumberForm[gev15[[1]], {5, 3}], " (original: 1.0e-9, 3.6e-8, 8.0e-9; 8.000)"}]]

(* ::Subsection:: *)
(*GS-16: the lattice operator on plane waves*)

(* ::Text:: *)
(*On a periodic chain of 24 sites along $x^1_{\rm LL}$ ($\ell^{\rm x}_{\rm LL}=0.5$, $m_{\rm LL}=0.4$, $\mu_{\rm LL}=0$, no gauge fields, no pairing) the lattice operator $\mathcal H^{\rm lat}_{\rm LL}$ of the solvers maps a plane wave, $\exp(iq^1_{\rm LL}x^1_{\rm LL})$ times one of the 16 unit vectors of a site, to a plane wave of the same momentum, and the $16\times16$ matrix of these images has the eigenvalues $-\varepsilon^{\rm lat}_{\rm LL}$ and $+\varepsilon^{\rm lat}_{\rm LL}$, each eight times, at all 24 allowed momenta $q^1_{\rm LL}=2\pi\kappa/(24\ell^{\rm x}_{\rm LL})$ (integers $\kappa=-11,\dots,12$), for the Wilson coefficients $r^{\rm W}_{\rm LL}=0$ and $r^{\rm W}_{\rm LL}=1$. The energy of a free particle on the lattice is $\varepsilon^{\rm lat}_{\rm LL}(\vec q_{\rm LL})=\Big(\sum_j\sin^2(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})/(\ell^{\rm x}_{\rm LL})^2+\big(m_{\rm LL}+(r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL})\sum_j(1-\cos(q^j_{\rm LL}\ell^{\rm x}_{\rm LL}))\big)^2\Big)^{1/2}$, the sums running over the directions with more than one site. At $q^1_{\rm LL}\ell^{\rm x}_{\rm LL}=\pm\pi$ the sine vanishes again: without the Wilson term ($r^{\rm W}_{\rm LL}=0$) the energy returns to $m_{\rm LL}$ there (a second, spurious particle, the doubler); with $r^{\rm W}_{\rm LL}=1$ it is lifted to $m_{\rm LL}+2r^{\rm W}_{\rm LL}/\ell^{\rm x}_{\rm LL}=4.4$. Figure 1 shows these eigenvalues with the formula and with the continuum energy $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$ (the figures stand in the subsection after the data files of the original program).*)

(* ::Input:: *)
{mFig, dxFig, nFig} = {0.4, 0.5, 24};
(* (1) the curves: the continuum energy and eps_lat for r_W = 0 and 1 on 401 momenta *)
curves = Flatten[Table[With[{q = -Pi/dxFig + 2 Pi/dxFig i/400.}, {rw, q, Sqrt[q^2 + mFig^2], t2epsLat[{q, 0., 0.}, dxFig, mFig, rw, {True, False, False}]}], {rw, {0., 1.}}, {i, 0, 400}], 1];
(* (2) the points: the largest eigenvalue of the operator on the plane waves of the 24 allowed momenta *)
dispDev = 0.;
points = Flatten[Table[
    Module[{lat1 = <|"n" -> {nFig, 1, 1}, "ell" -> dxFig, "periodic" -> {True, False, False}, "wilson" -> rw, "mass" -> mFig, "mu" -> 0.|>,
      kk, q, w, out, m16, ev, el},
     kk = If[kq <= nFig/2, kq, kq - nFig];
     q = 2 Pi kk/(nFig dxFig);
     (* the 16 plane waves, one per unit vector b, as the columns of a matrix *)
     w = Table[If[Mod[r - 1, 16] + 1 == b, Exp[I q dxFig Quotient[r - 1, 16]], 0. I], {r, 16 nFig}, {b, 16}];
     out = t2apply[lat1, t2trivialFields[nFig], w];
     m16 = out[[1 ;; 16]];
     dispDev = Max[dispDev, Max[Abs[out[[16 (nFig - 1) + 1 ;; 16 nFig]] - m16 Exp[I q dxFig (nFig - 1)]]]];
     ev = First[t2eigh[m16]];
     el = t2epsLat[{q, 0., 0.}, dxFig, mFig, rw, {True, False, False}];
     dispDev = Max[dispDev, Max[Abs[ev - Join[ConstantArray[-el, 8], ConstantArray[el, 8]]]]];
     {rw, q, ev[[16]]}],
    {rw, {0., 1.}}, {kq, 0, nFig - 1}], 1];
SL31Check["GS-16", "the lattice operator of the solvers on plane waves: on a periodic chain of 24 sites (dx = 0.5, m = 0.4) the image of a plane wave is a plane wave of the same momentum, and the 16 x 16 matrix of the images has the eigenvalues -eps_lat (eightfold) and +eps_lat (eightfold) at all 24 momenta, for r_W = 0 and 1 (tolerance 1e-12)",
  dispDev < 1*^-12, Row[{"largest deviation ", dispDev, " (original: 1.4e-14)"}]]

(* ::Subsection:: *)
(*The data files of the original program*)

(* ::Text:: *)
(*The original program t2_fields wrote the numbers of its two figures into three data files: t2_dispersion_lattice.csv (the 48 points of Figure 1: for $r^{\rm W}_{\rm LL}=0$ and $r^{\rm W}_{\rm LL}=1$ and each of the 24 momenta, the largest eigenvalue of the lattice operator), t2_dispersion_curves.csv (the 802 points of the curves of Figure 1: for $r^{\rm W}_{\rm LL}=0$ and $r^{\rm W}_{\rm LL}=1$ and each of 401 momenta, the continuum energy $\varepsilon_{\rm LL}$ and the formula of $\varepsilon^{\rm lat}_{\rm LL}$) and t2_cvode_accuracy.csv (the 41 rows of its integrations of GS-13 by CVODE: the output time $x^0_{\rm LL}$; the relative deviations $\delta^{\rm rel}_{\rm LL}$ for $r^{\rm tol}_{\rm LL}=10^{-8},10^{-10},10^{-12}$; and, for the same three tolerances, the length $h^{\rm CV}_{\rm LL}$ and the order $q^{\rm CV}_{\rm LL}$ of the last internal step of CVODE before the output time, 0 in the first row). The first two files print their numbers with 10 decimals, the third with 7 significant digits (its times with 4 decimals). The next cell contains every number of the three files, copied without change, so that the notebook stays complete. The check after it compares the numbers of Figure 1 with the lists `points` and `curves` computed above, row by row and column by column; because the files are rounded, deviations up to half a unit of the last printed decimal, $5\times10^{-11}$, are expected, and larger deviations would mean that the rewritten programs compute different numbers. The integrations by CVODE cannot be repeated in the Wolfram Language; for them the check confirms that the file has the output times of GS-13 and that its numbers show what the original check GS-13 states: the deviations are rounding errors in the first row, stay below $10^{-9}$ for $r^{\rm tol}_{\rm LL}=10^{-12}$, and their largest values decrease from $r^{\rm tol}_{\rm LL}=10^{-8}$ to $10^{-10}$ to $10^{-12}$. Figure 2 shows them together with the deviations of IDA computed in GS-13.*)

(* ::Input:: *)
(* @@INCLUDE t2_fields_figure_data.wl@@ *)

(* ::Input:: *)
(* Figure 1: the same rows in the same order, the momenta and the energies compared entry by entry; Figure 2: the times
   of the file and the largest deviations of its three integrations by CVODE *)
devPts = Max[Abs[points - t2origPoints]];
devCurves = Max[Abs[curves - t2origCurves]];
cvMax = Table[Max[t2origCvode[[All, 1 + k]]], {k, 3}];
cvTimes = Max[Abs[t2origCvode[[All, 1]] - touts13]];
SL31Check["T2-DATA-1", "the numbers of the figures agree with the data files written by the original program t2_fields: the 48 eigenvalues of the lattice operator and the 802 points of the curves of Figure 1 (tolerance 5.1e-11, half a unit of the last printed decimal); the file of the integrations by CVODE of Figure 2 has the 41 output times of GS-13 (tolerance 5.1e-5), and its relative deviations from the exact solution are below 1e-13 at x_LL^0 = 0, stay below 1e-9 for rtol = 1e-12, and their largest values decrease from rtol = 1e-8 to 1e-10 to 1e-12, as the original check GS-13 states",
  Dimensions[points] == Dimensions[t2origPoints] && Dimensions[curves] == Dimensions[t2origCurves] && Dimensions[t2origCvode] == {41, 10} &&
   devPts < 5.1*^-11 && devCurves < 5.1*^-11 && cvTimes < 5.1*^-5 && Max[t2origCvode[[1, 2 ;; 4]]] < 1*^-13 && cvMax[[3]] < 1*^-9 &&
   cvMax[[1]] > cvMax[[2]] > cvMax[[3]],
  Row[{"largest deviations ", devPts, ", ", devCurves, "; times ", cvTimes, "; largest deviations of CVODE in the file ", sci13[cvMax], " (original report: 1.2e-6, 1.7e-7, 1.7e-10)"}]]

(* ::Subsection:: *)
(*The figures of item [1]*)

(* ::Input:: *)
(* Figure 1: the energy of a free particle on the lattice; the colors of the project (blue, orange, grey) *)
blue = RGBColor["#2a78d6"]; orange = RGBColor["#eb6834"]; grey = RGBColor["#8a8984"]; muted = RGBColor["#5f5e5a"];
aqua = RGBColor["#1baf7a"]; green = RGBColor["#008300"]; red = RGBColor["#e34948"];
(* the registered symbols that appear in the labels of Figure 1 (Part I): the mass, the lattice spacing, the Wilson
   coefficient, the first component of the momentum, the two energies *)
{mSym, ellxSym, rWSym, q1Sym, epsSym, epsLatSym} = {LL["m_{\\rm LL}"], LL["\\ell^{\\rm x}_{\\rm LL}"],
   LL["r^{\\rm W}_{\\rm LL}"], LL["q^j_{\\rm LL}", "j" -> "1"], LL["\\varepsilon_{\\rm LL}"],
   LL["\\varepsilon^{\\rm lat}_{\\rm LL}"]};
(* labels of graphics are typeset in TraditionalForm, which prints every single letter in italics; upright[e] keeps the
   letters of the symbols upright, as they are printed in the cells of the notebook (for example the roman W of the
   Wilson coefficient) *)
upright[e_] := Style[e, SingleLetterItalics -> False];
curve[rw_, col_] := Select[curves, #[[1]] == rw &][[All, col]];
qAxis = Select[curves, #[[1]] == 0. &][[All, 2]] dxFig;
fig1a = Show[
   ListLinePlot[{Transpose[{qAxis, curve[0., 3]}], Transpose[{qAxis, curve[0., 4]}], Transpose[{qAxis, curve[1., 4]}]},
    PlotStyle -> {Directive[grey, Dashed, Thickness[0.004]], Directive[orange, Thickness[0.004]], Directive[blue, Thickness[0.004]]},
    PlotLegends -> Placed[LineLegend[upright /@ {Row[{"continuum, ", epsSym}], Row[{"lattice, ", epsLatSym, ", ", rWSym, " = 0"}],
        Row[{"lattice, ", epsLatSym, ", ", rWSym, " = 1"}]}], Below]],
   ListPlot[{Select[points, #[[1]] == 0. &][[All, {2, 3}]] . DiagonalMatrix[{dxFig, 1}], Select[points, #[[1]] == 1. &][[All, {2, 3}]] . DiagonalMatrix[{dxFig, 1}]},
    PlotStyle -> {orange, blue}, PlotMarkers -> {"\[EmptyCircle]", 9}],
   (* the labels are placed in empty regions of the plot (checked on the exported page); they are written in a muted
      ink, the colored curves next to them carry the identity *)
   Graphics[{muted, Dotted, Line[{{-Pi, mFig}, {Pi, mFig}}], Text[upright[Style[Row[{mSym, " = 0.4"}], 10, muted]], {-1.8, 0.56}],
     Text[upright[Style[Row[{rWSym, " = 1: doubler lifted to ", mSym, " + 2 ", rWSym, "/", ellxSym, " = 4.4"}], 10, muted]], {0, 5.05}],
     Text[upright[Style[Row[{rWSym, " = 0: doubler at \[PlusMinus]\[Pi]"}], 10, muted]], {1.6, 0.72}]}],
   PlotRange -> {{-Pi, Pi}, {0, 5.4}}, Axes -> False, Frame -> True,
   FrameLabel -> {upright[Row[{q1Sym, " ", ellxSym, " (momentum times lattice spacing)"}]], "energy"},
   FrameTicks -> {{Automatic, None}, {{{-Pi, "-\[Pi]"}, {-Pi/2, "-\[Pi]/2"}, {0, "0"}, {Pi/2, "\[Pi]/2"}, {Pi, "\[Pi]"}}, None}},
   GridLines -> Automatic, GridLinesStyle -> Directive[GrayLevel[0.9]], ImageSize -> 520,
   PlotLabel -> upright[Style[Row[{"(a) a free particle on the lattice, ", mSym, " = 0.4, ", ellxSym, " = 0.5"}], 12]]];
fig1b = ListLinePlot[Table[With[{sel = Select[curves, #[[1]] == rw && Abs[#[[2]] dxFig] <= Pi/2 + 1*^-12 &]}, Transpose[{sel[[All, 2]] dxFig, (sel[[All, 4]] - sel[[All, 3]])/sel[[All, 3]]}]], {rw, {0., 1.}}],
   PlotStyle -> {Directive[orange, Thickness[0.004]], Directive[blue, Thickness[0.004]]},
   PlotLegends -> Placed[LineLegend[upright /@ {Row[{rWSym, " = 0"}], Row[{rWSym, " = 1"}]}], {0.5, 0.15}],
   (* only the horizontal axis (deviation zero) is drawn; a vertical axis line would cross the legend *)
   Axes -> {True, False}, Frame -> True,
   FrameLabel -> upright /@ {Row[{q1Sym, " ", ellxSym}], Row[{"(", epsLatSym, " \[Minus] ", epsSym, ")/", epsSym}]},
   GridLines -> Automatic, GridLinesStyle -> Directive[GrayLevel[0.9]], ImageSize -> 420, PlotLabel -> Style["(b) the deviation from the continuum", 12]];
Labeled[Row[{fig1a, fig1b}, "  "], Style["Figure 1. The energy of a free particle on the lattice (item [1], check GS-16): the points are eigenvalues of the lattice operator of the solvers.", 11], Top]

(* ::Input:: *)
(* Figure 2: the time integration of check GS-13.  Blue, full lines, filled circles: IDA in this notebook.  Orange, dashed
   lines, open squares: CVODE in the original program (the numbers of the file t2_cvode_accuracy.csv in the data cell
   above, and the numbers of steps of its report).  The output time 0, where the deviations are rounding errors of about
   1e-15, is left out of the logarithmic plot.  The positions of texts in a logarithmic plot are natural logarithms *)
{x0Sym, rtolSym, deltaRelSym, wExSym, nstSym} = {LL["x^p_{\\rm LL}", "p" -> "0"], LL["r^{\\rm tol}_{\\rm LL}"],
   LL["\\delta^{\\rm rel}_{\\rm LL}"], LL["w^{\\rm ex}_{\\rm LL}"], LL["N^{\\rm st}_{\\rm LL}"]};
idaDev = Table[Transpose[{Rest[touts13], Rest[dev13[[k]]]}], {k, 3}];
cvDev = Table[Transpose[{Rest[t2origCvode[[All, 1]]], Rest[t2origCvode[[All, 1 + k]]]}], {k, 3}];
idaStyle = Directive[blue, Thickness[0.004]];
cvStyle = Directive[orange, Dashed, Thickness[0.004]];
(* the label of a tolerance, in the muted grey of the texts *)
tolLabel[k_] := upright[Style[Row[{rtolSym, " = ", Superscript[10, -6 - 2 k]}], 10, muted]];
tolShort[k_] := Style[Superscript[10, -6 - 2 k], 10, muted];
(* the curves end at the time 20.  The two curves of one tolerance are joined, by two thin grey lines from
   their ends, to the label of the tolerance, which stands to the right of the curves at the geometric mean of
   the two last deviations; the time axis is therefore drawn to 26.5, with tick marks up to 20 *)
endIda = Map[Last, dev13];
endCv = Table[t2origCvode[[-1, 1 + k]], {k, 3}];
labelAt = Sqrt[endIda endCv];
pairLabels = Table[{Directive[muted, Thickness[0.0015]],
     Line[{{20, Log[endIda[[k]]]}, {21, Log[labelAt[[k]]]}, {20, Log[endCv[[k]]]}}],
     Text[tolLabel[k], {21.3, Log[labelAt[[k]]]}, {-1, 0}]}, {k, 3}];
fig2a = ListLogPlot[Join[idaDev, cvDev], Joined -> True, PlotStyle -> Join[ConstantArray[idaStyle, 3], ConstantArray[cvStyle, 3]],
   PlotRange -> {{0, 26.5}, {1*^-13, 1*^-3}}, Frame -> True, FrameTicks -> {{Automatic, None}, {Range[0, 20, 5], None}},
   FrameLabel -> {upright[Row[{x0Sym, " (time)"}]], upright[Row[{deltaRelSym, " (relative deviation from ", wExSym, ")"}]]},
   GridLines -> {Range[0, 20, 5], Automatic}, GridLinesStyle -> Directive[GrayLevel[0.9]], ImageSize -> 470,
   Epilog -> pairLabels,
   PlotLabel -> Style["(a) the deviation from the exact solution", 12]];
(* the largest deviation against the number of internal steps; every point is labelled with its tolerance: the
   labels of IDA stand to the right of their points, those of CVODE to the left (the second one to the left and
   below its point, clear of the line that arrives there) *)
idaWork = Transpose[{steps13, maxdev13}];
cvWork = Transpose[{cvSteps13, cvMax}];
fig2b = ListLogLogPlot[{idaWork, cvWork}, Joined -> True, PlotStyle -> {idaStyle, cvStyle},
   PlotMarkers -> {{"\[FilledCircle]", 10}, {"\[EmptySquare]", 10}}, PlotRange -> {{400, 200000}, {1*^-12, 1*^-4}}, Frame -> True,
   FrameLabel -> {upright[Row[{nstSym, " (number of internal steps)"}]], upright[Row[{"largest ", deltaRelSym}]]},
   GridLines -> Automatic, GridLinesStyle -> Directive[GrayLevel[0.9]], ImageSize -> 470,
   Epilog -> Join[Table[Text[tolShort[k], Log[idaWork[[k]]], {-1.6, 0}], {k, 3}],
     MapThread[Text[tolShort[#1], Log[cvWork[[#1]]], #2] &, {{1, 2, 3}, {{1.6, 0}, {1.3, 1.8}, {1.6, 0}}}]],
   PlotLabel -> Style[upright[Row[{"(b) accuracy against work; the labels are ", rtolSym}]], 12]];
legend2 = LineLegend[{idaStyle, cvStyle}, {"IDA (this notebook)", "CVODE (original program t2_fields)"}, LegendLayout -> "Row"];
Labeled[Column[{Row[{fig2a, fig2b}, "  "], legend2}, Center],
  Style["Figure 2. The time integration of the lattice equations (item [1], check GS-13): IDA in this notebook and CVODE in the original program.", 11], Top]

(* ::Subsection:: *)
(*Summary of item [1]*)

(* ::Input:: *)
Grid[Prepend[Map[{#["name"], If[#["pass"], Style["PASS", Darker[Green]], Style["FAIL", Red]], #["statement"]} &,
    Select[SL31`Test`$Results, StringStartsQ[#["name"], "GS-" | "T2-"] &]], Style[#, Bold] & /@ {"check", "result", "statement"}],
  (* no fixed column widths: the front end wraps the statements at the width of the window or of the printed page *)
  Frame -> All, Alignment -> {Left, Top}]
