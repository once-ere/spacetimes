(* ::Package:: *)

(* ::Chapter:: *)
(*Part III, third section. The width of the critical region, and the superfluid as a Higgs sector*)

(* ::Text:: *)
(*On 21 November 2026 the author asked to find how wide the region of the fluctuations near the critical temperature is, which needs one more calculation, the gradient coefficient of the field at the critical temperature; then to determine for each of the five methods $X$ (HS $(1,1)$, HS $(0,1)$, HS $(0,0)$, KS and CJT) whether the superfluid can play the role of the Higgs particle, and whether the Higgs is really a composite particle. This chapter does both with the engines of Part II and of Section III.T, and repeats the checks of the Rust program `sf_part3` (modules `ginzburg` and `higgs`) and of the Python program `sf_part3.py`. **The answers.** The Ginzburg number is 7.6 to 11.4 (Section III.G.7): the critical region is wider than the critical temperature itself, so the mean-field description of the five methods holds nowhere near the transition. Every method realizes the Higgs mechanism and has a composite neutral scalar below the threshold of two quasi-particles, but in no method can it be the Higgs boson that is observed (Section III.G.8). Within this model the Higgs field is composite by construction.*)

(* ::Section:: *)
(*III.G.1 The free energy of Ginzburg and Landau and the Ginzburg number*)

(* ::Text:: *)
(***Units.** As in Section III.T: $k_{\rm B}=1$, the cutoff $\Lambda_{\rm LL}=1$, the chemical potential zero. **The field.** The field of the superfluid is the complex isospin vector $\vec\Phi=(\Phi^1,\Phi^2,\Phi^3)$ of the components of the pair self-energy along $(\mathsf K^\iota_{\rm LL})^\dagger$, $\Phi^\iota=\mathrm{tr}(\mathsf K^\iota_{\rm LL}\mathsf P_{\rm SF})/8$; it has $n^\Phi_{\rm SF}=6$ real components, and $\lvert\vec\Phi\rvert^2=\sum_\iota\lvert\Phi^\iota\rvert^2$. **The form of Ginzburg and Landau.** Near the critical temperature $T^{X,{\rm c}}_{\rm SF}$ the field is small. For a static field that varies slowly in space, the free energy of method $X$ to second order in the field and in its derivatives and to fourth order in the field is*)

(* ::DisplayFormula:: *)
(*\mathcal F^{X,{\rm GL}}_{\rm SF}[\vec\Phi]=\int d^3x\Big[\mathsf c^X_{\rm SF}\sum_{j=1}^3(\partial_j\vec\Phi)^\dagger\cdot\partial_j\vec\Phi+a^{X,{\rm Tay}}_{\rm SF}(T)\,\lvert\vec\Phi\rvert^2+b^{X,{\rm Tay}}_{\rm SF}(T)\,\lvert\vec\Phi\rvert^4\Big],*)

(* ::Text:: *)
(*with the Taylor coefficients $a^{X,{\rm Tay}}_{\rm SF}$, $b^{X,{\rm Tay}}_{\rm SF}$ of the free energy at the origin (Section III.T.6), $a^{X,{\rm Tay}}_{\rm SF}(T)\approx\alpha^{X,{\rm GL}}_{\rm SF}t^{\rm red}_{\rm SF}$ near $T^{X,{\rm c}}_{\rm SF}$, the reduced temperature $t^{\rm red}_{\rm SF}=(T-T^{X,{\rm c}}_{\rm SF})/T^{X,{\rm c}}_{\rm SF}$, the slope $\alpha^{X,{\rm GL}}_{\rm SF}=T^{X,{\rm c}}_{\rm SF}\,\partial_Ta^{X,{\rm Tay}}_{\rm SF}$ at $T^{X,{\rm c}}_{\rm SF}$, and the **gradient coefficient** $\mathsf c^X_{\rm SF}$ at $T^{X,{\rm c}}_{\rm SF}$. The rotations and the isospin rotations allow no other term of second order, and a term with $\vec\Phi\cdot\vec\Phi$ has the fermion number 4 and is not invariant under $U(1)^{\rm F}_{\rm LL}$. Of the two quartic invariants $\lvert\vec\Phi\rvert^4$ and $\lvert\vec\Phi\cdot\vec\Phi\rvert^2$, which are equal on the polar fields of the minimum, the form keeps the coefficient along these fields. **The Ginzburg number.** Above $T^{X,{\rm c}}_{\rm SF}$ the field fluctuates about zero with the probability density proportional to $\exp(-\mathcal F^{X,{\rm GL}}_{\rm SF}/T)$. To second order the fluctuations are Gaussian, and their free energy per volume is, up to terms analytic in $T$, $\tfrac12n^\Phi_{\rm SF}T\int\frac{d^3k}{(2\pi)^3}\ln(\mathsf c^X_{\rm SF}k^2+a^{X,{\rm Tay}}_{\rm SF})$. With $\int\frac{d^3k}{(2\pi)^3}(\mathsf ck^2+a)^{-2}=(8\pi\mathsf c^{3/2}a^{1/2})^{-1}$ its specific heat per volume, $C=-T\partial^2_T$ of the free energy, is near $T^{X,{\rm c}}_{\rm SF}$*)

(* ::DisplayFormula:: *)
(*C^{X,{\rm fl}}_{\rm SF}=\frac{n^\Phi_{\rm SF}\,(\alpha^{X,{\rm GL}}_{\rm SF})^{3/2}}{16\pi\,(\mathsf c^X_{\rm SF})^{3/2}\,(t^{\rm red}_{\rm SF})^{1/2}},\qquad\Delta C^X_{\rm SF}=\frac{(\alpha^{X,{\rm GL}}_{\rm SF})^2}{2\,b^{X,{\rm Tay}}_{\rm SF}\,T^{X,{\rm c}}_{\rm SF}},*)

(* ::Text:: *)
(*where $\Delta C^X_{\rm SF}$ is the jump of the specific heat of mean-field theory at $T^{X,{\rm c}}_{\rm SF}$ (below it the minimum of the form is $\lvert\vec\Phi\rvert^2=-a/(2b)$ with the free energy $-a^2/(4b)$ per volume). Mean-field theory holds where $C^{X,{\rm fl}}_{\rm SF}\ll\Delta C^X_{\rm SF}$. The two are equal at the **Ginzburg number** (A. P. Levanyuk 1959, V. L. Ginzburg 1960)*)

(* ::DisplayFormula:: *)
(*{\rm Gi}^{X}_{\rm SF}=\frac{(n^\Phi_{\rm SF})^2\,(b^{X,{\rm Tay}}_{\rm SF})^2\,(T^{X,{\rm c}}_{\rm SF})^2}{64\pi^2\,\alpha^{X,{\rm GL}}_{\rm SF}\,(\mathsf c^X_{\rm SF})^3}=\frac{(n^\Phi_{\rm SF})^2}{256\pi^2\big(\Delta C^X_{\rm SF}(\xi^{X,{\rm GL}}_{\rm SF})^3\big)^2},\qquad\xi^{X,{\rm GL}}_{\rm SF}=\Big(\frac{\mathsf c^X_{\rm SF}}{\alpha^{X,{\rm GL}}_{\rm SF}}\Big)^{1/2}.*)

(* ::Text:: *)
(*Mean-field theory holds for $\lvert t^{\rm red}_{\rm SF}\rvert\gg{\rm Gi}^{X}_{\rm SF}$, and the critical region, where it fails, has the width of about ${\rm Gi}^{X}_{\rm SF}T^{X,{\rm c}}_{\rm SF}$ on both sides of $T^{X,{\rm c}}_{\rm SF}$. The second form shows the meaning: the correlation length of mean-field theory is $\xi^{X,{\rm GL}}_{\rm SF}\lvert t^{\rm red}_{\rm SF}\rvert^{-1/2}$, and mean-field theory needs many degrees of freedom in a correlation volume, $\Delta C^X_{\rm SF}(\xi^{X,{\rm GL}}_{\rm SF})^3\gg1$. **A remark on the factors (what it means and why it matters).** Other forms of the criterion (the mean square fluctuation of the field in a correlation volume against the square of its mean value, or the first correction to $b^{X,{\rm Tay}}_{\rm SF}$ against $b^{X,{\rm Tay}}_{\rm SF}$) give the same combination of $\alpha$, $b$, $\mathsf c$ and $T_{\rm c}$ with other numerical factors of order one, and books on the subject differ by such factors. The formula above is the definition used here. It matters only when the number is near 1; the numbers found below are far above 1, so a factor of order one does not change the conclusion.*)

(* ::Section:: *)
(*III.G.2 The coefficient of the field of a moving pair: the static pair response*)

(* ::Text:: *)
(*A pair field with the momentum $q^{\rm pair}_{\rm SF}$ along $\vec e_3$, $\vec\Phi(\vec x)=\vec\Phi\,e^{iq^{\rm pair}_{\rm SF}x^3}$, has the gradient term $\mathsf c^X_{\rm SF}(q^{\rm pair}_{\rm SF})^2\lvert\vec\Phi\rvert^2$, so its coefficient of $\lvert\vec\Phi\rvert^2$ is $a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T)=a^{X,{\rm Tay}}_{\rm SF}(T)+\mathsf c^X_{\rm SF}(q^{\rm pair}_{\rm SF})^2+O((q^{\rm pair}_{\rm SF})^4)$. The state of a moving pair pairs the momenta $\vec q+\tfrac12q^{\rm pair}_{\rm SF}\vec e_3$ and $-\vec q+\tfrac12q^{\rm pair}_{\rm SF}\vec e_3$ (relative momentum $\vec q$). After the gauge transformation $\Psi\to e^{iq^{\rm pair}_{\rm SF}x^3/2}\Psi$, which makes the pair field uniform, it is the thermal state of the matrix of Bogoliubov and de Gennes (Section II.2) with $\mathsf h_{\rm LL}(\pm\vec q)$ replaced by $\mathsf h_{\rm LL}(\pm\vec q)+\mathsf T^{\rm tw}_{\rm SF}$, the **twist** $\mathsf T^{\rm tw}_{\rm SF}=\tfrac12q^{\rm pair}_{\rm SF}\,\alpha^3_{\rm LL}\otimes\mathsf 1_2$, with the normal self-energy $\mathsf N^{\rm n}_{\rm SF}$ of the normal state at $T$ (a change of the normal self-energy changes the free energy only at fourth order in the field, because the free energy is stationary in it) and with a small antisymmetric pair block $\mathsf P^{\rm a}_{\rm SF}$. To first order in $\mathsf P^{\rm a}_{\rm SF}$ the pair matrix summed over the momenta, $\mathsf C^{\rm p,tot}_{\rm SF}=\int\frac{d^3q}{(2\pi)^3}\mathsf C^{\rm p}_{\rm LL}(\vec q)$, is the **static pair response***)

(* ::DisplayFormula:: *)
(*\mathsf C^{\rm p,tot}_{\rm SF}=\mathcal X^{\rm P}_{\rm SF}[\mathsf P^{\rm a}_{\rm SF}]=-\int\frac{d^3q}{(2\pi)^3}\sum_{a,b=1}^8\frac{f(\varepsilon^{\rm A}_{{\rm SF},a})-f(\varepsilon^{\rm D}_{{\rm SF},b})}{\varepsilon^{\rm A}_{{\rm SF},a}-\varepsilon^{\rm D}_{{\rm SF},b}}\,\mathsf u_{{\rm SF},a}\mathsf u_{{\rm SF},a}^\dagger\,\mathsf P^{\rm a}_{\rm SF}\,\mathsf w_{{\rm SF},b}\mathsf w_{{\rm SF},b}^\dagger,*)

(* ::Text:: *)
(*where $\varepsilon^{\rm A}_{{\rm SF},a}$ and the normalized eigenvectors $\mathsf u_{{\rm SF},a}$ are those of the particle block $\mathsf h_{\rm LL}(\vec q)+\mathsf N^{\rm n}_{\rm SF}+\mathsf T^{\rm tw}_{\rm SF}$, $\varepsilon^{\rm D}_{{\rm SF},b}$ and $\mathsf w_{{\rm SF},b}$ those of the hole block $-(\mathsf h_{\rm LL}(-\vec q)+\mathsf N^{\rm n}_{\rm SF}+\mathsf T^{\rm tw}_{\rm SF})^T$, $f$ is the Fermi function, and the quotient is $f'(\varepsilon)$ where the two energies coincide (the first-order term of the function $f(\mathsf H_{\rm SF})$ of a Hermitian matrix in its off-diagonal block). The free energy of the state, measured from the normal state at $T$, is to second order*)

(* ::DisplayFormula:: *)
(*\mathcal F^X_{2,{\rm SF}}=\tfrac12\,\mathrm{Re}\,\mathrm{tr}\Big(\big(\mathsf P^{\rm a}_{\rm SF}-\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}]\big)\big(\mathsf C^{\rm p,tot}_{\rm SF}\big)^\dagger\Big),*)

(* ::Text:: *)
(*where $\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}]$ is the pair self-energy of method $X$ for the pair matrix $\mathsf C^{\rm p,tot}_{\rm SF}$ (Section II.2; linear in it, and it contains every pair channel of the method, not only the channels $\mathsf K^\iota_{\rm LL}$). **Derivation.** The expectation value of the pair term of the BdG Hamiltonian is $\langle V\rangle=-\mathrm{Re}\,\mathrm{tr}(\mathsf P^{\rm a}_{\rm SF}(\mathsf C^{\rm p,tot}_{\rm SF})^\dagger)$; the thermal state of the Hamiltonian with the pair term has the thermodynamic potential $\tfrac12\langle V\rangle$ below that of the normal state (second-order perturbation theory); the free energy of the method contains this potential minus $\langle V\rangle$ (the pair term is not part of the Hamiltonian of the model) plus the pair part of the interaction energy, which is quadratic in the pair matrix and equals $-\tfrac12\mathrm{Re}\,\mathrm{tr}(\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}](\mathsf C^{\rm p,tot}_{\rm SF})^\dagger)$; the sum is the formula above. The field of the state is $\Phi^3=\mathrm{tr}(\mathsf K^3_{\rm LL}\mathsf P_{\rm SF}[\mathsf C^{\rm p,tot}_{\rm SF}])/8$. The free energy of the field is the smallest $\mathcal F^X_{2,{\rm SF}}$ among the states with this field: the components of $\mathsf P^{\rm a}_{\rm SF}$ along $(\mathsf K^1_{\rm LL})^\dagger$, $(\mathsf K^2_{\rm LL})^\dagger$, $(\mathsf K^3_{\rm LL})^\dagger$ are held at $(0,0,1)$, and its components along the other 25 antisymmetric matrices are made self-consistent (a linear system of 25 complex unknowns: the stationarity of the free energy with a Lagrange multiplier for the field). Then $a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T)=\mathcal F^X_{2,{\rm SF}}/\lvert\Phi^3\rvert^2$. At $q^{\rm pair}_{\rm SF}=0$ and in the channel $\mathsf K^3_{\rm LL}$ alone this is*)

(* ::DisplayFormula:: *)
(*a^{X,{\rm Tay}}_{\rm SF}(0;T)=\frac{1-\lambda^{\rm P}_{\rm SF}(T)}{\lambda^{\rm P}_{\rm SF}(T)\,g^{X,{\rm P}}_{\rm SF}},*)

(* ::Text:: *)
(*with the eigenvalue $\lambda^{\rm P}_{\rm SF}(T)$ of Section III.T.4: the applied block $(\mathsf K^3_{\rm LL})^\dagger$ produces the field $\Phi^3=\lambda^{\rm P}_{\rm SF}(T)$, and the formula for $\mathcal F^X_{2,{\rm SF}}$ gives $\tfrac12(1-\lambda^{\rm P}_{\rm SF})\cdot2\lambda^{\rm P}_{\rm SF}/g^{X,{\rm P}}_{\rm SF}$. Hence the slope in closed form, $\alpha^{X,{\rm GL}}_{\rm SF}=-T^{X,{\rm c}}_{\rm SF}\,\partial_T\lambda^{\rm P}_{\rm SF}/(\lambda^{\rm P}_{\rm SF})^2g^{X,{\rm P}}_{\rm SF}$ at $T^{X,{\rm c}}_{\rm SF}$. **The engine.** The next cell defines the static pair response on any set of momenta with weights, the quadratic free energy of the field, the normal self-energy at $T$, and the coefficient for a twist $q\,\alpha^3_{\rm LL}\otimes\mathsf t$ with an isospin matrix $\mathsf t$: $\mathsf t=\mathsf 1_2$ moves the pair (fermion number 2) with $q^{\rm pair}_{\rm SF}=2q$; $\mathsf t=\tfrac12\tau_1$ turns the isospin direction of the field (its components of charge $\pm1$ of $T^1_{\rm LL}$ get the momenta $\pm q$), and must give the same coefficient by the isospin invariance of the form.*)

(* ::Input:: *)
(* ---- the static pair response and the quadratic free energy of a field (Section III.G.2) ---- *)
(* gzPairs: the 28 index pairs {a, b}, a < b; gzBasis: the antisymmetric basis matrices E_ab - E_ba *)
gzPairs = Flatten[Table[{a, b}, {a, 8}, {b, a + 1, 8}], 1];
gzBasis = Map[Function[pr, Table[Which[{i, j} == pr, 1., {j, i} == pr, -1., True, 0.], {i, 8}, {j, 8}]], gzPairs];
(* gzCoeffs[p]: the coefficients of an antisymmetric matrix in this basis (its entries above the diagonal);
   gzFromCoeffs[v]: the antisymmetric matrix of the coefficients v *)
gzCoeffs[p_] := Map[p[[#[[1]], #[[2]]]] &, gzPairs];
gzFromCoeffs[v_] := v . gzBasis;
(* gzAlpha8[j]: alpha^j_LL (x) 1_2 (numerical); gzH[q]: the free matrix h_LL(q) = (beta m_LL + alpha . q) (x) 1_2 *)
gzAlpha8 = Table[N[d8[alphaLL[j]]], {j, 3}];
gzH[q_] := N[mLL] b8 + q[[1]] gzAlpha8[[1]] + q[[2]] gzAlpha8[[2]] + q[[3]] gzAlpha8[[3]];
(* gzTwist[q, t2]: the twist q alpha^3_LL (x) t2 (t2 a 2 x 2 isospin matrix) *)
gzTwist[q_, t2_] := q N[d8[alphaLL[3]] . i8[t2]];
(* gzDF[e1, e2, t]: (f(e1) - f(e2))/(e1 - e2), and f'(e) = -f (1 - f)/t where the two energies coincide *)
gzDF[e1_, e2_, t_] := If[Abs[e1 - e2] < 10^-7, With[{f = fermiSF[0.5 (e1 + e2), t]}, -f (1 - f)/t], (fermiSF[e1, t] - fermiSF[e2, t])/(e1 - e2)];
(* gzResponse[qs, ws, ns, x, t]: the static pair response X^P of the 28 basis matrices at the temperature t on the
   momenta qs with the weights ws of int d^3q/(2 pi)^3, for the normal self-energy ns and the twist x.  At each node:
   the eigenvalues ea and eigenvectors (rows of va) of the particle block A = h(q) + N + X, ed and vd of the hole block
   D = -(h(-q) + N + X)^T; with L = conj(va) and R = vd the matrix U_A^dag (E_ab - E_ba) U_D has the entries
   L[k, a] R[l, b] - L[k, b] R[l, a]; it is multiplied entry by entry with F[k, l] = gzDF[ea_k, ed_l, t] and turned
   back with U_A = transpose(va) and U_D^dag = conj(vd) *)
gzResponse[qs_, ws_, ns_, x_, t_] := Module[{out = ConstantArray[0. I, {28, 8, 8}]},
   Do[Module[{a, d, ea, va, ed, vd, l, r, fm, ua},
      a = gzH[qs[[n]]] + ns + x;
      d = -Transpose[gzH[-qs[[n]]] + ns + x];
      {ea, va} = Eigensystem[0.5 (a + ConjugateTranspose[a])];
      {ed, vd} = Eigensystem[0.5 (d + ConjugateTranspose[d])];
      {ea, ed} = Re[{ea, ed}];
      l = Conjugate[va]; r = vd; ua = Transpose[va];
      fm = Table[gzDF[ea[[k]], ed[[m]], t], {k, 8}, {m, 8}];
      out -= ws[[n]] Map[Function[pr, ua . ((Outer[Times, l[[All, pr[[1]]]], r[[All, pr[[2]]]]] - Outer[Times, l[[All, pr[[2]]]], r[[All, pr[[1]]]]]) fm) . Conjugate[vd]], gzPairs]],
     {n, Length[qs]}];
   out];
(* gzQuad[ts, chi, relax]: the coefficient a = F_2/|Phi^3|^2 of the field for the response chi (28 matrices) of the
   contact terms ts; the components of P^a along (K^1)^dag, (K^2)^dag, (K^3)^dag are held at (0, 0, 1) and, with relax,
   the other 25 are made self-consistent: (1 - Pi_perp M) p_perp = Pi_perp M e3, with the linear map M (column j: the
   pair self-energy of the method for the pair matrix chi_j) and the projection Pi_perp that removes the components
   along the three (K^iota)^dag; returns {a, the norm of p_perp} *)
gzQuad[ts_, chi_, relax_ : True] := Module[{cols, kd, perp, amat, pp, p, kap, pm, phi, pnorm = 0.},
   cols = Transpose[Map[gzCoeffs[interaction[ts, zero8, #][[3]]] &, chi]];
   kd = Table[gzCoeffs[ConjugateTranspose[k8[i]]], {i, 3}];
   perp[v_] := v - Sum[kd[[i]] Tr[k8[i] . gzFromCoeffs[v]]/8, {i, 3}];
   p = kd[[3]];
   If[relax,
     amat = IdentityMatrix[28] - Transpose[Map[perp, Transpose[cols]]];
     pp = LinearSolve[amat, perp[cols . kd[[3]]]];
     pnorm = Norm[pp]; p = kd[[3]] + pp];
   kap = p . chi;
   pm = interaction[ts, zero8, kap][[3]];
   phi = Tr[k8[3] . pm]/8;
   {0.5 Re[Tr[(gzFromCoeffs[p] - pm) . ConjugateTranspose[kap]]]/Abs[phi]^2, pnorm}];
(* gzNormal[t, temp]: the normal self-energy of the method t at the temperature temp (thermal engine, P = 0) *)
gzNormal[t_, temp_] := gzNormal[t, temp] = solveSFT[thModel[t], zero8, zero8, temp, 0.6, 10^-14]["n"];
(* gzA[t, temp, q, t2, relax, nodes]: {a, |p_perp|} of the method t at the temperature temp for the twist q alpha^3 (x) t2,
   on the nodes {qs, ws} (default: the 24 x 6 momenta of the grid of the method) *)
gzA[t_, temp_, q_, t2_, relax_ : True, nodes_ : Automatic] := Module[{mo = thModel[t], qs, ws},
   {qs, ws} = If[nodes === Automatic, {mo["g"]["q"], mo["g"]["w"]}, nodes];
   gzQuad[mo["ts"], gzResponse[qs, ws, gzNormal[t, temp], gzTwist[q, t2], temp], relax]];
Print["the static pair response is defined (", Length[gzPairs], " basis matrices)"];

(* ::Section:: *)
(*III.G.3 The values of the Rust program*)

(* ::Text:: *)
(*The closed cell holds the results of the modules `ginzburg` and `higgs` of the Rust program: `rustGinzburg` (per method: the critical temperature, the Dirac mass of the normal state at it, the coefficients $\alpha^{X,{\rm GL}}_{\rm SF}$, $b^{X,{\rm Tay}}_{\rm SF}$, $\mathsf c^X_{\rm SF}$, the gradient coefficient of the isospin twist, of the fixed channels, of the twist of $T^3_{\rm LL}$ and of the twisted states, the two values of the extrapolation, the norms of the other channels, $a$ at $T^{X,{\rm c}}_{\rm SF}$, the Ginzburg number, the width, $\xi^{X,{\rm GL}}_{\rm SF}$ and the field at the edge of the critical region), `rustGinzburgRegions` (the two regularizations of Section III.G.5) and `rustHiggs` (the numbers of Section III.G.8, one row per method).*)

(* ::Input::Closed:: *)
(* @@INCLUDE data_part3_ginzburg.wl@@ *)
Print["rows: ", Length[rustGinzburg], " (Ginzburg), ", Length[rustGinzburgRegions], " (regions), ", Length[rustHiggs], " (Higgs)"];

(* ::Input:: *)
(* gzRust[t, key]: an entry of rustGinzburg; hgRust[t, key]: an entry of rustHiggs (one row per method) *)
gzCols = AssociationThread[{"method", "Tc", "massN", "alpha", "b", "c", "cIso", "cFixed", "cT3", "cTwisted", "c1", "c2", "perp0", "perpK", "aTc", "Gi", "width", "xi0", "PhiG"} -> Range[19]];
gzRust[t_, key_] := First[Select[rustGinzburg, #[[1]] == t &]][[gzCols[key]]];
hgCols = AssociationThread[{"method", "v", "fF", "fI", "MH", "MNU", "MW", "MZ", "MHv", "lambda", "MHMW", "MH2gap", "MNUMH", "fFfI", "MWMZ", "width", "Tcv"} -> Range[17]];
hgRust[t_, key_] := First[Select[rustHiggs, #[[1]] == t &]][[hgCols[key]]];
Print["the critical temperatures of the Rust program: ", N[Map[gzRust[#, "Tc"] &, thTags]]];

(* ::Section:: *)
(*III.G.4 The coefficient at zero pair momentum and the slope*)

(* ::Text:: *)
(*The next cell computes, for every method with its own critical temperature (Section III.T.4), the coefficient at zero pair momentum from the static pair response and checks it four ways: at $0.9\,T^{X,{\rm c}}_{\rm SF}$ it is the Taylor coefficient of the nonlinear free energy of the Rust table (Section III.T); at $T/T^{X,{\rm c}}_{\rm SF}=0.9$, $0.998$, $1.002$ it is $(1-\lambda^{\rm P}_{\rm SF})/(\lambda^{\rm P}_{\rm SF}g^{X,{\rm P}}_{\rm SF})$ with the eigenvalue of Section III.T.4; at $T^{X,{\rm c}}_{\rm SF}$ it vanishes; and no other pair channel is induced. The slope $\alpha^{X,{\rm GL}}_{\rm SF}$ is computed twice: from central differences of the coefficient at $T^{X,{\rm c}}_{\rm SF}(1\pm0.002)$ and $T^{X,{\rm c}}_{\rm SF}(1\pm0.004)$ with the Richardson extrapolation, and in closed form with the derivatives of $I^{(1)}_{\rm SF}$ on the radial nodes,*)

(* ::DisplayFormula:: *)
(*\partial_TI^{(1)}_{\rm SF}=-\frac1{2T^2}\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{\cosh^2(E_M/2T)},\qquad\partial_MI^{(1)}_{\rm SF}=\int_0^{\Lambda_{\rm LL}}q^2\frac{M}{E_M}\Big[\frac{1}{2TE_M\cosh^2(E_M/2T)}-\frac{\tanh(E_M/2T)}{E_M^2}\Big]dq,*)

(* ::Text:: *)
(*and the derivative of the Dirac mass of the normal state from the implicit function theorem applied to its scalar self-consistency $M-m_{\rm LL}=g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}(M,T)$ (Section III.T.4): $\partial_TM=g^{X,{\rm S}}_{\rm SF}\partial_Tn^{\rm S}_{\rm SF}/(1-g^{X,{\rm S}}_{\rm SF}\partial_Mn^{\rm S}_{\rm SF})$.*)

(* ::Input:: *)
(* gzI1D[mo, m, temp]: {I1, dI1/dT, dI1/dM} on the radial nodes of the grid of the model mo (temp > 0) *)
gzI1D[mo_, m_, temp_] := Module[{kr = mo["g"]["kr"], wr = mo["g"]["wr"], e, x},
   e = Sqrt[kr^2 + m^2]; x = e/(2 temp);
   {Total[wr kr^2 Tanh[x]/e], -Total[wr kr^2 Sech[x]^2]/(2 temp^2), Total[wr kr^2 (m/e) (Sech[x]^2/(2 temp e) - Tanh[x]/e^2)]}];
(* gzAlphaClosed[t]: {alpha, lambda^P(T_c), M_n(T_c)} in closed form at the critical temperature of the notebook *)
gzAlphaClosed[t_] := Module[{mo = thModel[t], tc = thTc[t], gs, gp, mn, i1, i1t, i1m, i10, dnt, dnm, dmt, lam, dlam},
   {gs, gp} = thCoup[t];
   mn = thMassN[t, tc];
   {i1, i1t, i1m} = gzI1D[mo, mn, tc];
   dnt = -(2/Pi^2) mn i1t; dnm = -(2/Pi^2) (i1 + mn i1m);
   dmt = gs dnt/(1 - gs dnm);
   lam = gp i1/Pi^2; dlam = gp (i1t + i1m dmt)/Pi^2;
   {-tc dlam/(lam^2 gp), lam, mn}];
(* gzResults[t]: the results of the method t (association), filled by the cells of this section and the next *)
gzResults = <||>;
Do[Module[{tc = thTc[t], gp = thCoup[t][[2]], t09 = thGet[t, 0.9, "T"], a09, at, worst = 0., ac, perp0, d1, d2, alFD, alCl, lamC, mnC},
   (* at 0.9 T_c, against the Taylor coefficient of the nonlinear free energy of the Rust table *)
   a09 = gzA[t, t09, 0., IdentityMatrix[2]][[1]]; at = thGet[t, 0.9, "a"];
   check["WP3-G1 " <> t, thName[t] <> ": at 0.9 T_c the coefficient a(0) of |Phi|^2 from the static pair response (" <> sci[a09] <> ") equals the Taylor coefficient a^Tay of the nonlinear free energy of Section III.T (" <> sci[at] <> ") (relative difference)",
    Abs[a09 - at]/Abs[at], 10^-7, "P3-G1 " <> t, "YP3-G1 " <> t];
   (* the closed form at three temperatures *)
   Do[Module[{temp = tau tc, a, lam},
      a = gzA[t, temp, 0., IdentityMatrix[2]][[1]]; lam = thLam[t, temp];
      worst = Max[worst, Abs[a - (1 - lam)/(lam gp)]/Abs[(1 - lam)/(lam gp)]]], {tau, {0.9, 0.998, 1.002}}];
   check["WP3-G2 " <> t, thName[t] <> ": a(0, T) = (1 - lambda^P(T))/(lambda^P(T) g^P) at T/T_c = 0.9, 0.998, 1.002, with lambda^P(T) of Section III.T.4 (largest relative difference)",
    worst, 10^-9, "P3-G2 " <> t, "YP3-G2 " <> t];
   (* the slope: central differences with the Richardson extrapolation, and the closed form *)
   d1 = (gzA[t, 1.002 tc, 0., IdentityMatrix[2]][[1]] - gzA[t, 0.998 tc, 0., IdentityMatrix[2]][[1]])/(2 0.002 tc);
   d2 = (gzA[t, 1.004 tc, 0., IdentityMatrix[2]][[1]] - gzA[t, 0.996 tc, 0., IdentityMatrix[2]][[1]])/(2 0.004 tc);
   alFD = tc (4 d1 - d2)/3;
   {alCl, lamC, mnC} = gzAlphaClosed[t];
   check["WP3-G10 " <> t, thName[t] <> ": alpha = T_c da/dT = " <> ToString[NumberForm[alFD, 10]] <> " from central differences of the static pair response (Richardson) = " <> ToString[NumberForm[alCl, 10]] <> " in closed form (lambda^P(T_c) = " <> ToString[NumberForm[lamC, 12]] <> ", M_n = " <> ToString[NumberForm[mnC, 10]] <> ") = Rust (largest relative difference)",
    Max[Abs[alFD - alCl]/alCl, Abs[alCl - gzRust[t, "alpha"]]/gzRust[t, "alpha"]], 10^-8, "P3-G10 " <> t, "YP3-G10 " <> t];
   (* at the critical temperature: a vanishes, and no other pair channel is induced *)
   {ac, perp0} = gzA[t, tc, 0., IdentityMatrix[2]];
   check["WP3-G3 " <> t, thName[t] <> ": a(0, T_c) vanishes to the accuracy of T_c (|a(0, T_c)| / alpha)", Abs[ac]/alCl, 10^-9, "P3-G3 " <> t, "YP3-G3 " <> t];
   check["WP3-G4 " <> t, thName[t] <> ": at zero pair momentum no other pair channel is induced (norm of the self-consistent components of P^a other than (K^iota)^dag)", perp0, 10^-12, "P3-G4 " <> t, "YP3-G4 " <> t];
   gzResults[t] = <|"Tc" -> tc, "alpha" -> alCl, "alphaFD" -> alFD, "a0" -> ac|>], {t, thTags}];

(* ::Section:: *)
(*III.G.5 The gradient coefficient and the regularization*)

(* ::Text:: *)
(*$\mathsf c^X_{\rm SF}$ is $[a^{X,{\rm Tay}}_{\rm SF}(q^{\rm pair}_{\rm SF};T^{X,{\rm c}}_{\rm SF})-a^{X,{\rm Tay}}_{\rm SF}(0;T^{X,{\rm c}}_{\rm SF})]/(q^{\rm pair}_{\rm SF})^2$ at $q^{\rm pair}_{\rm SF}=0.01$ and $0.02$ with the Richardson extrapolation. The six directions of the grid are exact at second order in the twist (the term of second order is a quadratic polynomial in the direction). **The regularization (a remark: what it means and why it matters).** The model has the cutoff $\lvert\vec q\rvert<\Lambda_{\rm LL}$ on the momenta of the uniform states, and for a moving pair it must be put on one of two momenta. With the cutoff on the momenta of the two charges of the pair, $\lvert\vec q\pm\tfrac12q^{\rm pair}_{\rm SF}\vec e_3\rvert<\Lambda_{\rm LL}$ (a lens of two spheres), a moving pair loses the pairs in a shell at the cutoff whose thickness is of order $\lvert q^{\rm pair}_{\rm SF}\rvert$, so that $a(q^{\rm pair}_{\rm SF})-a(0)$ is linear in $\lvert q^{\rm pair}_{\rm SF}\rvert$ and a gradient expansion does not exist. With the cutoff on the relative momentum, $\lvert\vec q\rvert<\Lambda_{\rm LL}$, the change is quadratic; this routing is used here (it is the routing of the twisted states of the coefficients of the kinetic part at $T=0$, where the uniform twist shifts the momenta inside the fixed sphere). The remaining error of the regularization is measured by the twist $q\,\alpha^3_{\rm LL}\otimes\tfrac12\tau_3$ of the unbroken generator $T^3_{\rm LL}$, which does not change the field along $\vec e_3$ and would cost nothing in a gauge-invariant theory; with the cutoff it costs the spurious coefficient $\mathsf c^{X,{\rm T3}}_{\rm SF}$, about a fifth of $\mathsf c^X_{\rm SF}$ (the Rust table). This matters for the size of the Ginzburg number, which is proportional to $1/\mathsf c^3$: changing $\mathsf c^X_{\rm SF}$ by $\pm\mathsf c^{X,{\rm T3}}_{\rm SF}$ changes it by a factor of about 2 either way (Section III.G.7), not enough to change the conclusion. The next cells compute the gradient coefficient from the phase twist and from the isospin twist, check that it does not depend on the quadrature (for CJT: 40 radial nodes, and a product rule of 144 directions), and compare the two regularizations on a product rule: 24 Gauss-Legendre nodes in $\cos\theta$ (12 on each half), 6 angles $\varphi$ and 24 radial nodes on $[0,r(\vec n)]$ in the direction $\vec n$, with $r=\Lambda_{\rm LL}$ for the sphere and $r(\vec n)=-\tfrac12q^{\rm pair}_{\rm SF}\lvert n_3\rvert+(\Lambda_{\rm LL}^2-\tfrac14(q^{\rm pair}_{\rm SF})^2(1-n_3^2))^{1/2}$ for the lens.*)

(* ::Input:: *)
(* gzGrad[t, t2, s, relax, nodes]: {c, c(0.01), c(0.02)} at T_c from the twist of the isospin matrix t2, twist strength
   q = k/s for the pair momentum k (s = 2 for the phase twist, s = 1 for the isospin twists) *)
gzGrad[t_, t2_, s_, relax_ : True, nodes_ : Automatic] := Module[{tc = thTc[t], a0, c1, c2},
   a0 = gzA[t, tc, 0., t2, relax, nodes][[1]];
   c1 = (gzA[t, tc, 0.01/s, t2, relax, nodes][[1]] - a0)/0.01^2;
   c2 = (gzA[t, tc, 0.02/s, t2, relax, nodes][[1]] - a0)/0.02^2;
   {(4 c1 - c2)/3, c1, c2}];
Do[Module[{c, c1, c2, cIso},
   {c, c1, c2} = gzGrad[t, IdentityMatrix[2], 2.];
   cIso = gzGrad[t, PauliMatrix[1]/2, 1.][[1]];
   check["WP3-G5 " <> t, thName[t] <> ": the isospin twist gives the gradient coefficient of the phase twist, c = " <> ToString[NumberForm[c, 10]] <> " (isospin invariance; relative difference)",
    Abs[cIso/c - 1], 10^-8, "P3-G5 " <> t, "YP3-G5 " <> t];
   check["WP3-G6 " <> t, thName[t] <> ": c > 0 and the extrapolation in k converges: c(0.01) = " <> ToString[NumberForm[c1, 10]] <> ", c(0.02) = " <> ToString[NumberForm[c2, 10]] <> ", extrapolated " <> ToString[NumberForm[c, 10]] <> " = Rust (relative difference; 1 is added if c <= 0 or the two values differ by more than 1e-3 of c)",
    Abs[c - gzRust[t, "c"]]/gzRust[t, "c"] + If[c > 0 && Abs[c2 - c1]/c < 10^-3, 0, 1], 10^-8, "P3-G6 " <> t, "YP3-G6 " <> t];
   gzResults[t] = Join[gzResults[t], <|"c" -> c|>]], {t, thTags}];

(* ::Input:: *)
(* gzRegion[nr, nt, nphi, radius]: the nodes {qs, ws} of a region of relative momenta with the radius radius[n] in the
   direction n: Gauss-Legendre in cos(theta) on [-1, 0] and [0, 1] (nt nodes each), nphi equal steps in phi, nr
   Gauss-Legendre nodes on [0, radius[n]]; the weights integrate int d^3q/(2 pi)^3 *)
gzRegion[nr_, nt_, nphi_, radius_] := Module[{xr, wrr, xt, wt, qs = {}, ws = {}},
   {xr, wrr} = gaussLegendre[nr]; {xt, wt} = gaussLegendre[nt];
   Do[Do[Module[{ct = 0.5 (xt[[i]] + half), wct = 0.25 wt[[i]], st, ph, n, rr, q},
       st = Sqrt[Max[1. - ct^2, 0.]];
       Do[ph = 2. Pi (iphi + 0.5)/nphi; n = {st Cos[ph], st Sin[ph], ct}; rr = radius[n];
         q = 0.5 rr (xr + 1.);
         qs = Join[qs, Map[# n &, q]];
         ws = Join[ws, 0.5 rr wrr q^2/(2. Pi^2) wct/nphi], {iphi, 0, nphi - 1}]], {i, nt}], {half, {-1., 1.}}];
   {qs, ws}];
(* the two regularizations of every method at T_c, against the Rust table: the growth of a(k) - a(0) from k = 0.005
   to 0.01 (4 for a quadratic, 2 for a linear function) and the four differences themselves *)
gzSphere = gzRegion[24, 12, 6, 1. &];
Do[Module[{tc = thTc[t], s0, ds, dl, rs, rl, rr, dev},
   s0 = gzA[t, tc, 0., IdentityMatrix[2], True, gzSphere][[1]];
   ds = Table[gzA[t, tc, k/2, IdentityMatrix[2], True, gzSphere][[1]] - s0, {k, {0.005, 0.01}}];
   (* the radius of the lens is built with the value of k inserted (With), so that it does not depend on a variable
      of the same name elsewhere *)
   dl = Table[With[{kk = k}, gzA[t, tc, kk/2, IdentityMatrix[2], True, gzRegion[24, 12, 6, Function[n, -0.5 kk Abs[n[[3]]] + Sqrt[1. - 0.25 kk^2 (1 - n[[3]]^2)]]]][[1]] - s0], {k, {0.005, 0.01}}];
   {rs, rl} = {ds[[2]]/ds[[1]], dl[[2]]/dl[[1]]};
   rr = Select[rustGinzburgRegions, #[[1]] == t &];
   dev = Max[Abs[ds - rr[[1 ;; 2, 3]]]/Abs[rr[[1 ;; 2, 3]]], Abs[dl - rr[[1 ;; 2, 4]]]/Abs[rr[[1 ;; 2, 4]]]];
   check["WP3-G9 " <> t, thName[t] <> ": with the cutoff on the momenta of the two charges a(k) - a(0) is linear in |k| (ratio " <> ToString[NumberForm[rl, 6]] <> " for k = 0.01 and 0.005), with the cutoff on the relative momentum quadratic (ratio " <> ToString[NumberForm[rs, 7]] <> "); the four differences = Rust (largest relative difference; 1 is added if a ratio is off by more than 0.02 of 2 or 1e-3 of 4)",
    dev + If[Abs[rl/2 - 1] < 0.02 && Abs[rs/4 - 1] < 10^-3, 0, 1], 10^-7, "P3-G9 " <> t, "YP3-G9 " <> t]], {t, thTags}];

(* ::Input:: *)
(* CJT: the gradient coefficient with 40 radial nodes (a model of the same contact terms on the grid of 40 x 6 momenta)
   and on the product rule of the sphere (144 directions, 24 radial nodes), against the six directions *)
Module[{tc = thTc["CJT"], ts = thModel["CJT"]["ts"], mo40, n40, a40, c40, cProd, c6 = gzResults["CJT"]["c"]},
  mo40 = makeModel[ts, 40];
  n40 = solveSFT[mo40, zero8, zero8, tc, 0.6, 10^-14]["n"];
  a40[q_] := gzQuad[ts, gzResponse[mo40["g"]["q"], mo40["g"]["w"], n40, gzTwist[q, IdentityMatrix[2]], tc]][[1]];
  c40 = Module[{a0 = a40[0.], c1, c2}, c1 = (a40[0.005] - a0)/0.01^2; c2 = (a40[0.01] - a0)/0.02^2; (4 c1 - c2)/3];
  cProd = gzGrad["CJT", IdentityMatrix[2], 2., True, gzSphere][[1]];
  check["WP3-G8", "CJT: the gradient coefficient does not depend on the quadrature: six directions and 24 radial nodes " <> ToString[NumberForm[c6, 12]] <> ", 144 directions " <> ToString[NumberForm[cProd, 12]] <> ", 40 radial nodes " <> ToString[NumberForm[c40, 12]] <> " (largest relative difference)",
   Max[Abs[cProd - c6], Abs[c40 - c6]]/c6, 10^-8, "P3-G8", "YP3-G8 CJT"]];

(* ::Section:: *)
(*III.G.6 The quartic coefficient at the critical temperature*)

(* ::Text:: *)
(*$b^{X,{\rm Tay}}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})$ comes from the nonlinear free energy of four small applied fields at $T^{X,{\rm c}}_{\rm SF}$, as the Taylor coefficients of Section III.T.6: the thermal states of the applied pair self-energies $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$, $\Phi_{\rm a}=0.01$ to $0.04$, with a self-consistent normal self-energy, and $\mathcal F/\Phi^2$ interpolated by a cubic polynomial in $\Phi^2$ (in the scaled variable $\Phi^2/\Phi^2_{\max}$, whose matrix is well conditioned). The next cell does this for CJT and compares with the Rust value; the other methods use the Rust values of the table.*)

(* ::Input:: *)
Module[{mo = thModel["CJT"], tc = thTc["CJT"], nn, n0, xs = {}, ys = {}, cf, a, b},
  nn = solveSFT[mo, zero8, zero8, tc, 0.6, 10^-14]; n0 = nn["n"];
  Do[Module[{res = fixedPairT[mo, fa ConjugateTranspose[k8[3]], n0, tc]},
     n0 = res[[1]]; AppendTo[xs, res[[2]]^2]; AppendTo[ys, (res[[3]] - nn["free"])/res[[2]]^2]], {fa, {0.01, 0.02, 0.03, 0.04}}];
  cf = LinearSolve[Table[(x/Max[xs])^k, {x, xs}, {k, 0, 3}], ys];
  {a, b} = {cf[[1]], cf[[2]]/Max[xs]};
  gzResults["CJT"] = Join[gzResults["CJT"], <|"b" -> b|>];
  check["WP3-G12", "CJT at T_c: the quartic coefficient b = " <> ToString[NumberForm[b, 10]] <> " of the free energy at the origin (four applied fields, thermal engine) is positive and = Rust " <> ToString[NumberForm[gzRust["CJT", "b"], 10]] <> " (relative difference; a(T_c) = " <> sci[a] <> ")",
   If[b > 0, Abs[b - gzRust["CJT", "b"]]/gzRust["CJT", "b"], 1.], 10^-6, "P3-G12 CJT", "YP3-G12 CJT"]];

(* ::Section:: *)
(*III.G.7 The Ginzburg number: how wide is the critical region?*)

(* ::Text:: *)
(*The next cell computes the Ginzburg number by both forms of Section III.G.1, with the slope and the gradient coefficient of the notebook and the quartic coefficient of the notebook (CJT) or of the Rust table (the other methods), and prints the table of the answer: the correlation length $\xi^{X,{\rm GL}}_{\rm SF}$ in units of $1/\Lambda_{\rm LL}$, the specific-heat jump per correlation volume $\Delta C^X_{\rm SF}(\xi^{X,{\rm GL}}_{\rm SF})^3$, the Ginzburg number, the width ${\rm Gi}^{X}_{\rm SF}T^{X,{\rm c}}_{\rm SF}$ of the critical region in temperature, and the Ginzburg number with $\mathsf c^X_{\rm SF}$ changed by $\pm\mathsf c^{X,{\rm T3}}_{\rm SF}$ of the Rust table.*)

(* ::Input:: *)
gzGi[t_] := Module[{r = gzResults[t], b = If[t === "CJT", gzResults["CJT"]["b"], gzRust[t, "b"]], gi1, dc, xi, gi2},
   gi1 = 36 b^2 r["Tc"]^2/(64 Pi^2 r["alpha"] r["c"]^3);
   dc = r["alpha"]^2/(2 b r["Tc"]); xi = Sqrt[r["c"]/r["alpha"]];
   gi2 = 36/(256 Pi^2 (dc xi^3)^2);
   {gi1, gi2, xi, dc xi^3, b}];
Do[Module[{g = gzGi[t]},
   check["WP3-G11 " <> t, thName[t] <> ": the Ginzburg number by the first form " <> ToString[NumberForm[g[[1]], 8]] <> " and by the second form " <> ToString[NumberForm[g[[2]], 8]] <> " = Rust " <> ToString[NumberForm[gzRust[t, "Gi"], 8]] <> " (largest relative difference)",
    Max[Abs[g[[1]] - gzRust[t, "Gi"]], Abs[g[[2]] - g[[1]]]]/gzRust[t, "Gi"], 10^-6, "P3-G11 " <> t, "YP3-G11 " <> t]], {t, thTags}];
Grid[Prepend[Table[Module[{g = gzGi[t], r = gzResults[t], ct3 = gzRust[t, "cT3"]},
     {thName[t], NumberForm[r["Tc"], 6], NumberForm[r["alpha"], 6], NumberForm[g[[5]], 6], NumberForm[r["c"], 6], NumberForm[g[[3]], 4], NumberForm[g[[4]], 3],
      NumberForm[g[[1]], 4], NumberForm[g[[1]] r["Tc"], 4],
      ToString[NumberForm[36 g[[5]]^2 r["Tc"]^2/(64 Pi^2 r["alpha"] (r["c"] + ct3)^3), 3]] <> " to " <> ToString[NumberForm[36 g[[5]]^2 r["Tc"]^2/(64 Pi^2 r["alpha"] (r["c"] - ct3)^3), 3]]}], {t, thTags}],
   {"method", "T_c", "alpha", "b", "c", "xi0", "Delta C xi0^3", "Gi", "Gi T_c", "Gi with c -+ c_T3"}], Frame -> All, Alignment -> Right, BaseStyle -> {FontSize -> 8}]

(* ::Text:: *)
(***How wide is the critical region?** The Ginzburg number is 7.6 to 11.4, far above 1. The condition $\lvert t^{\rm red}_{\rm SF}\rvert\gg{\rm Gi}^{X}_{\rm SF}$ of mean-field theory therefore holds at no temperature where the form of Ginzburg and Landau applies: even at $\lvert t^{\rm red}_{\rm SF}\rvert=1$ the Gaussian fluctuations alone give $C^{X,{\rm fl}}_{\rm SF}/\Delta C^X_{\rm SF}=({\rm Gi}^{X}_{\rm SF})^{1/2}\approx3$. The width ${\rm Gi}^{X}_{\rm SF}T^{X,{\rm c}}_{\rm SF}$ is larger than the critical temperature itself and of the order of the cutoff. The reason is in the table: the correlation length $\xi^{X,{\rm GL}}_{\rm SF}$ is about $1/\Lambda_{\rm LL}$, the shortest length of the model, because the pairs are tightly bound (there is no Fermi surface, and the pairing needs a coupling above the critical one), and the specific-heat jump per correlation volume is about 0.04, far below 1: a correlation volume holds much less than one degree of freedom, while mean-field theory needs many. (In a superconductor of Bardeen, Cooper and Schrieffer the correlation length is large against the distance of the electrons and the Ginzburg number, of the order of $(T_{\rm c}/E_{\rm F})^4$ with the Fermi energy $E_{\rm F}$, is tiny; here the opposite holds.) The error of the regularization does not change the conclusion (the last column). **What this means for Section III.T (a remark, and why it matters).** The statements of Section III.T.6 are statements about the mean-field free energy, and as such they are correct and checked. But the temperatures at which that free energy shows the behaviour of Landau's theory, from about $0.4$ to $0.98$ of $T^{X,{\rm c}}_{\rm SF}$, lie inside the critical region. The mean-field theory of the five methods therefore does not describe the superfluid near its transition: the critical temperatures, the exponent $\tfrac12$ of the order parameter and the continuity of the transition are not reliable. It matters because the five methods agree with each other there, and agreement among mean-field methods is no evidence when all of them omit the same fluctuations. A reliable description of the transition needs a calculation beyond the methods: for example the three-dimensional theory of a complex isospin vector with two quartic invariants and with the gauge fields on a lattice (the method used for the thermal transition of the electroweak theory by K. Kajantie, M. Laine, K. Rummukainen and M. Shaposhnikov, 1996), or the Gaussian fluctuations of the auxiliary pair field in HS (the method of P. Nozieres and S. Schmitt-Rink, 1985, for pairs bound more tightly than their distance).*)

(* ::Section:: *)
(*III.G.8 Can the superfluid play the role of the Higgs particle, and is the Higgs a composite particle?*)

(* ::Text:: *)
(***Two questions.** Whether the superfluid can play the role of the Higgs particle has two meanings. (b) Can it be the Higgs field of the Standard Model? No, in every method, and the answer needs no number: the only matter field of the model is the DiracLike spinor, which has electric charge 0 and no quantum numbers of the electroweak group $SU(2)_L\times U(1)_Y$, so every field made of it, the condensate included, is a singlet of the electroweak group. Its vacuum value leaves that group unbroken and gives no mass to the $W$ and $Z$ bosons of nature, and its amplitude mode has no coupling at tree level to them, to the quarks or to the leptons, the couplings by which the Higgs boson of nature is identified. The model contains no coupling between the two sectors; a term such as $(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}\,\vec\Phi^\dagger\cdot\vec\Phi$ would be an addition to the model, would mix the amplitude mode with the Higgs boson of nature, and would be limited by the measured couplings of that boson. (The first version of this section compared the superfluid with the Higgs boson of the Standard Model as if the two gauge groups were one; that comparison had no basis and is withdrawn.) (a) Is the superfluid the Higgs field of its own gauge group $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$, with a Higgs particle? The rest of this section answers (a) with the quantities of the new group alone; its scale is not known, and every mass is in units of $\Lambda_{\rm LL}$. **The scale of the breaking.** $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}f^{X,{\rm I}}_{\rm SF}$ with the isospin decay constant $f^{X,{\rm I}}_{\rm SF}=(2Z^X_{\rm SF})^{1/2}\Phi^X_{0,{\rm SF}}$, where $Z^X_{\rm SF}$ and $Y^X_{\rm SF}$ are the coefficients of the two invariants of the kinetic part of the field, $Z^X_{\rm SF}\lvert\partial\vec\Phi\rvert^2+Y^X_{\rm SF}\lvert\vec\Phi^\dagger\cdot\partial\vec\Phi\rvert^2/\lvert\vec\Phi\rvert^2$, and $\Phi^X_{0,{\rm SF}}$ is the field of the vacuum at $T=0$; the decay constant of the phase is $f^{X,{\rm F}}_{\rm SF}=(2(Z^X_{\rm SF}+Y^X_{\rm SF}))^{1/2}\Phi^X_{0,{\rm SF}}$, and $M^X_{Z,{\rm SF}}=2g^{\rm F}_{\rm LL}f^{X,{\rm F}}_{\rm SF}$. With the convention $M_W=gv/2$ by which the Standard Model defines its vacuum value, the scale of the breaking and the quartic coupling are*)


(* ::DisplayFormula:: *)
(*v^X_{\rm SF}=2f^{X,{\rm I}}_{\rm SF},\qquad M^X_{W,{\rm SF}}=\tfrac12g^{\rm v}_{\rm LL}v^X_{\rm SF},\qquad\lambda^{X,{\rm H}}_{\rm SF}=\frac{(M^{X,{\rm H}}_{\rm SF})^2}{2(v^X_{\rm SF})^2},*)

(* ::Text:: *)
(*with the mass $M^{X,{\rm H}}_{\rm SF}$ of the amplitude (Higgs) mode and $M^{X,{\rm NU}}_{\rm SF}$ of the non-unitary doublet from the linear response of Part II (the exact small oscillations of each method). **The width of the amplitude mode.** On the field $\vec\Phi=(\Phi^X_{0,{\rm SF}}+h/(2(Z^X_{\rm SF}+Y^X_{\rm SF}))^{1/2})\,e^{i\vartheta_0}\vec e_3$ (the amplitude field $h$ normalized canonically and the phase $\vartheta_0$), and on the same field rotated about $\vec e_1$ by $\vartheta_1$, the kinetic part contains the couplings $f^{X,{\rm F}}_{\rm SF}\,h(\partial\vartheta_0)^2$ and $((f^{X,{\rm I}}_{\rm SF})^2/f^{X,{\rm F}}_{\rm SF})\,h(\partial\vartheta_1)^2$ (and the same for $\vartheta_2$). With the canonical Goldstone fields $f^{X,{\rm F}}_{\rm SF}\vartheta_0$ and $f^{X,{\rm I}}_{\rm SF}\vartheta_k$ all three couplings are $1/f^{X,{\rm F}}_{\rm SF}$; the decay of $h$ at rest into two massless quanta of one canonical field has the amplitude $(M^{X,{\rm H}}_{\rm SF})^2/f^{X,{\rm F}}_{\rm SF}$ and the width $(M^{X,{\rm H}}_{\rm SF})^3/(32\pi(f^{X,{\rm F}}_{\rm SF})^2)$ (the phase space of two identical massless particles), and the three channels give*)

(* ::DisplayFormula:: *)
(*\Gamma^{X,{\rm H}}_{\rm SF}=\frac{3\,(M^{X,{\rm H}}_{\rm SF})^3}{32\pi\,(f^{X,{\rm F}}_{\rm SF})^2}.*)

(* ::Text:: *)
(*With the gauge fields the Goldstone fields are the longitudinal parts of the massive gauge bosons (the equivalence theorem of J. M. Cornwall, D. N. Levin and G. Tiktopoulos 1974 and of M. S. Chanowitz and M. K. Gaillard 1985), so that the same formula gives the width into $W^+W^-$ and $ZZ$ of the new group, up to corrections that the cells after the table of the Higgs sector compute. With $f^{X,{\rm F}}_{\rm SF}$ replaced by the vacuum value $v$ of the Standard Model, the same formula is the known width $3m^3/(32\pi v^2)$ of a heavy Higgs boson of mass $m$ into its longitudinal $W$ and $Z$ bosons, a check of the normalization. **A remark on the width (what it means and why it matters).** The formula uses the effective Lagrangian at the energy $M^{X,{\rm H}}_{\rm SF}$, where it is not accurate (its mass of the amplitude mode differs from the linear response by 7.0 to 17 per cent, as the cell after the table of the Higgs sector computes), so it estimates the order of magnitude only. It matters because the conclusion drawn from it, that the amplitude mode is a broad resonance and not a narrow particle, needs only the order of magnitude: the width is 2 to 4 times the mass. The next cell derives the two couplings by exact differentiation of the kinetic part (symbolic, with real symbols), and compares them with the closed forms and the Rust values.*)

(* ::Input:: *)
(* the couplings of the amplitude field to the Goldstone fields by exact differentiation of the kinetic part along a line
   gzX: the field at gzX is (gzP0 + gzAmp/gzNh) e^(i gzD0 gzX) R_1(gzD1 gzX) e_3 with the constant amplitude gzAmp and the
   gradients gzD0, gzD1 of the two angles (R_1: the rotation about e_1); gzKin is Z |dPhi|^2 + Y |Phi^dag dPhi|^2/|Phi|^2
   at gzX = 0 with the coefficients gzZz, gzYy (all symbols real) *)
gzField = (gzP0 + gzAmp/gzNh) Exp[I gzD0 gzX] {0, -Sin[gzD1 gzX], Cos[gzD1 gzX]};
gzKin = Module[{ph = gzField /. gzX -> 0, dph = D[gzField, gzX] /. gzX -> 0},
   ComplexExpand[gzZz Conjugate[dph] . dph + gzYy Abs[Conjugate[ph] . dph]^2/(Conjugate[ph] . ph)]];
(* the coefficients of gzAmp gzD0^2 and of gzAmp gzD1^2: the derivative in gzAmp at gzAmp = 0, with the canonical
   normalization gzNh = (2 (Z + Y))^(1/2) of the amplitude field *)
gzCouplings = Simplify[{Coefficient[D[gzKin /. gzD1 -> 0, gzAmp] /. gzAmp -> 0, gzD0^2], Coefficient[D[gzKin /. gzD0 -> 0, gzAmp] /. gzAmp -> 0, gzD1^2]} /. gzNh -> Sqrt[2 (gzZz + gzYy)],
   Assumptions -> {gzZz > 0, gzZz + gzYy > 0, gzP0 > 0}];
Print["the couplings of h to (d theta_0)^2 and (d theta_1)^2: ", gzCouplings, " = {f_F, f_I^2/f_F} with f_F = (2 (Z + Y))^(1/2) Phi0, f_I = (2 Z)^(1/2) Phi0"];
Do[Module[{z = rustPart3["p3_Z_" <> t], y = rustPart3["p3_Y_" <> t], p0 = rustPart3["p3_Phi0_" <> t], g, fF, fI},
   g = gzCouplings /. {gzZz -> z, gzYy -> y, gzP0 -> p0};
   fF = Sqrt[2 (z + y)] p0; fI = Sqrt[2 z] p0;
   check["WP3-H2 " <> t, thName[t] <> ": the couplings of h to (d theta_0)^2 and (d theta_1)^2 by exact differentiation of the kinetic part, " <> ToString[NumberForm[g[[1]], 8]] <> " and " <> ToString[NumberForm[g[[2]], 8]] <> ", = f_F and f_I^2/f_F and = Rust (largest relative difference)",
    Max[Abs[g[[1]]/fF - 1], Abs[g[[2]]/(fI^2/fF) - 1], Abs[g[[1]]/hgRust[t, "fF"] - 1], Abs[g[[2]]/(hgRust[t, "fI"]^2/hgRust[t, "fF"]) - 1]], 10^-10, "P3-H2 " <> t, "YP3-H2 " <> t]], {t, thTags}];

(* ::Input:: *)
(* the numbers of the Higgs sector of every method from the values of Parts II and III (the susceptibilities chi_11 and
   chi_FF with f_I = chi_11^(1/2), f_F = chi_FF^(1/2)/2, the masses of the linear response) with the gap 2 E_gap of the
   ground state computed by the notebook (Part II), against the Rust table *)
gzHiggs[t_] := Module[{c11 = rustPart3["p3_chi11_" <> t], cff = rustPart3["p3_chiFF_" <> t], mh = rustPart3["p3_MH_RPA_" <> t], mnu = rustPart3["p3_MNU_RPA_" <> t],
    gap0 = thGet[t, 0., "gap"], fI, fF, v, mw, mz},
   fI = Sqrt[c11]; fF = Sqrt[cff]/2; v = 2 fI; mw = gvLL fI; mz = 2 gfLL fF;
   <|"v" -> v, "fF" -> fF, "fI" -> fI, "MHv" -> mh/v, "lambda" -> mh^2/(2 v^2), "MHMW" -> mh/mw, "MH2gap" -> mh/(2 gap0), "MNUMH" -> mnu/mh,
     "fFfI" -> fF/fI, "MWMZ" -> mw/mz, "width" -> 3 mh^2/(32 Pi fF^2), "Tcv" -> gzRust[t, "Tc"]/v, "MH" -> mh|>];
Do[Module[{hg = gzHiggs[t], gapNb = gapSF[thGround[t]["n"], thGround[t]["p"]], dev},
   checkTrue["WP3-H1 " <> t, thName[t] <> ": the amplitude mode lies below the threshold of two quasi-particles: M_H = " <> ToString[NumberForm[hg["MH"], 7]] <> " < 2 E_gap = " <> ToString[NumberForm[2 gapNb, 7]] <> " (the gap of the ground state computed by the notebook)",
    hg["MH"] < 2 gapNb, "M_H / 2 E_gap = " <> ToString[NumberForm[hg["MH"]/(2 gapNb), 5]], "P3-H1 " <> t, "YP3-H1 " <> t];
   dev = Max[Map[Abs[hg[#] - hgRust[t, #]]/Abs[hgRust[t, #]] &, {"v", "fF", "fI", "MHv", "lambda", "MHMW", "MH2gap", "MNUMH", "fFfI", "MWMZ", "width", "Tcv"}]];
   check["WP3-H3 " <> t, thName[t] <> ": the numbers of the Higgs sector (v_SF = " <> ToString[NumberForm[hg["v"], 8]] <> ", M_H/v_SF = " <> ToString[NumberForm[hg["MHv"], 6]] <> ", lambda = " <> ToString[NumberForm[hg["lambda"], 6]] <> ", Gamma/M_H = " <> ToString[NumberForm[hg["width"], 5]] <> ") = Rust (largest relative difference)",
    dev, 10^-9, "P3-H3 " <> t, "YP3-H3 " <> t]], {t, thTags}];
Grid[Prepend[Table[Module[{hg = gzHiggs[t]}, {thName[t], NumberForm[hg["MHv"], 4], NumberForm[hg["lambda"], 4], NumberForm[hg["MHMW"], 4], NumberForm[hg["MH2gap"], 3],
       NumberForm[hg["MNUMH"], 4], NumberForm[hg["fFfI"], 4], NumberForm[hg["MWMZ"], 4], NumberForm[hg["width"], 3]}], {t, thTags}],
   {"method", "M_H/v", "lambda", "M_H/M_W", "M_H/2E_gap", "M_NU/M_H", "f_F/f_I", "M_W/M_Z", "Gamma/M_H"}], Frame -> All, Alignment -> Right, BaseStyle -> {FontSize -> 8}]

(* ::Text:: *)
(***The decay into pairs of gauge bosons (a remark: what it means and why it matters).** The width formula above is the width into $W^+W^-$ and $ZZ$ of the new group up to corrections of the relative order of $(M^X_{W,{\rm SF}}/M^{X,{\rm H}}_{\rm SF})^2$ and $(M^X_{Z,{\rm SF}}/M^{X,{\rm H}}_{\rm SF})^2$: the equivalence theorem holds at energies large against the masses of the bosons, and the decay at rest gives each boson the energy $M^{X,{\rm H}}_{\rm SF}/2$. The kinetic part couples $h$ to the gauge bosons through their mass terms: both squared masses are proportional to the square of the field of the vacuum, so the term linear in $h$ of each mass term is that mass term times $2h/f^{X,{\rm F}}_{\rm SF}$, and at tree level the width into a pair of $W$ bosons is that of the two corresponding Goldstone channels times the factor*)

(* ::DisplayFormula:: *)
(*\Big(1-\frac{4(M^X_{W,{\rm SF}})^2}{(M^{X,{\rm H}}_{\rm SF})^2}\Big)^{1/2}\Big(1-\frac{4(M^X_{W,{\rm SF}})^2}{(M^{X,{\rm H}}_{\rm SF})^2}+\frac{12(M^X_{W,{\rm SF}})^4}{(M^{X,{\rm H}}_{\rm SF})^4}\Big)*)

(* ::Text:: *)
(*(the phase space, and the sum over the polarizations of the two bosons, whose transverse parts the Goldstone fields do not have), and the same with $M^X_{Z,{\rm SF}}$ for $ZZ$. The next cell computes, for every method, the larger of the ratios $M^X_{W,{\rm SF}}/M^{X,{\rm H}}_{\rm SF}$ and $M^X_{Z,{\rm SF}}/M^{X,{\rm H}}_{\rm SF}$, the smaller of the two factors, and how much the masses of the effective Lagrangian exceed those of the linear response. The ratio is at most 0.106 (KS and CJT) and the factor at least 0.934, so the width formula overestimates the width into $W^+W^-$ and $ZZ$ of the new group by less than 7 per cent. It matters little: that is far less than the uncertainty of the effective Lagrangian at the energy of the mode (the remark above), and the conclusion, a broad resonance, does not depend on it.*)

(* ::Input:: *)
(* the decay into a pair of W or Z bosons: the larger of the ratios M_W/M_H and M_Z/M_H (gzVV["ratio"]), the tree-level
   factor (1 - 4 r)^(1/2) (1 - 4 r + 12 r^2) with r the square of that ratio (gzVV["factor"]; the smaller of the factors
   of W and Z), and the masses of the effective Lagrangian (the curvature of the potential and the kinetic part) against
   the linear response, in per cent, for the amplitude mode (gzVV["effH"]) and for the non-unitary modes (gzVV["effNU"]);
   the masses M_W = g^v f_I and M_Z = 2 g^F f_F are those of gzHiggs *)
gzVV[t_] := Module[{hg = gzHiggs[t], mw, mz, r},
   mw = gvLL hg["fI"]; mz = 2 gfLL hg["fF"]; r = (Max[mw, mz]/hg["MH"])^2;
   <|"ratio" -> Sqrt[r], "factor" -> Sqrt[1 - 4 r] (1 - 4 r + 12 r^2),
     "effH" -> 100 (rustPart3["p3_MH_" <> t]/rustPart3["p3_MH_RPA_" <> t] - 1), "effNU" -> 100 (rustPart3["p3_MNU_" <> t]/rustPart3["p3_MNU_RPA_" <> t] - 1)|>];
Grid[Prepend[Table[Module[{q = gzVV[t]}, {thName[t], NumberForm[q["ratio"], 4], NumberForm[q["factor"], 4], NumberForm[q["effH"], 3], NumberForm[q["effNU"], 3]}], {t, thTags}],
   {"method", "max(M_W, M_Z)/M_H", "tree-level factor", "M_H: L^X over response - 1 (per cent)", "M_NU: L^X over response - 1 (per cent)"}], Frame -> All, Alignment -> Right, BaseStyle -> {FontSize -> 8}]

(* ::Text:: *)
(***The answer to question (a) for each method**, judged by the properties that make a field the Higgs field of a gauge group and its amplitude mode a Higgs particle.*)

(* ::ItemNumbered:: *)
(***The Higgs mechanism.** In every method the condensate gives the masses $M^X_{W,{\rm SF}}=g^{\rm v}_{\rm LL}f^{X,{\rm I}}_{\rm SF}$ and $M^X_{Z,{\rm SF}}=2g^{\rm F}_{\rm LL}f^{X,{\rm F}}_{\rm SF}$ to the $W^\pm$ and the $Z$ of the superfluid, leaves exactly one gauge boson, the zeugon $W^3$ (not the photon), massless, and turns the three Goldstone fields into the longitudinal parts of the massive bosons: the superfluid does for the new group what a Higgs field does.*)


(* ::ItemNumbered:: *)
(***A scalar of charge 0 below the threshold.** In every method the amplitude mode (charge $Q_{\rm LL}=0$) lies below the threshold of two quasi-particles, at 0.82 to 0.90 of $2E^{X,{\rm gap}}_{\rm SF}$: a bound state of the mean-field dynamics.*)


(* ::ItemNumbered:: *)
(***Its mass against the scale of the breaking.** $M^{X,{\rm H}}_{\rm SF}/v^X_{\rm SF}$ is 4.46 to 5.69 and $\lambda^{X,{\rm H}}_{\rm SF}$ is 9.9 to 16.2: the amplitude mode is heavier than the scale of the breaking. Whether a quartic coupling of this size is compatible with the perturbative unitarity of the scattering of the longitudinal gauge bosons of this group is not decided here: the bound depends on the scalar fields and the channels of the theory, and it was not computed for this one.*)


(* ::ItemNumbered:: *)
(***Its width.** $\Gamma^{X,{\rm H}}_{\rm SF}/M^{X,{\rm H}}_{\rm SF}$ is 2.1 to 3.9: once the decays into Goldstone fields are allowed, the width exceeds the mass, and the amplitude mode is a broad resonance, not a particle.*)


(* ::ItemNumbered:: *)
(***Further scalars.** Every method has the non-unitary doublet, two scalars of the charges $Q_{\rm LL}=\pm1$ at 0.99 to 1.09 times $M^{X,{\rm H}}_{\rm SF}$.*)


(* ::ItemNumbered:: *)
(***The quantum numbers.** The condensate is an isospin vector of fermion number 2 of the vector-like $SU(2)_{\rm v}$, a group of the DiracLike spinor that is not part of the electroweak theory. The zeugon is $W^3$ (no mixing angle), the $Z$ is the gauge boson of the fermion number, and $M^X_{W,{\rm SF}}/M^X_{Z,{\rm SF}}$ depends on two independent couplings. The mass of the DiracLike spinor does not come from the condensate as a Yukawa mass: the condensate adds a pairing gap to a Dirac mass that the vector-like theory allows without it.*)


(* ::ItemNumbered:: *)
(***The thermal history.** The transition of the superfluid lies deep in its own critical region (Section III.G.7); its mean-field temperature, 1.6 to 1.8 times $v^X_{\rm SF}$, is not reliable.*)


(* ::Text:: *)
(***The verdict.** (a) In every method the superfluid is the Higgs field of $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$: it breaks the group to $U(1)^{\rm Q}_{\rm LL}$, gives masses to $W^\pm$ and $Z$, leaves the massless zeugon, and has a Higgs-like scalar of charge 0 below the threshold of two quasi-particles. That scalar is a Higgs particle only in the mean-field dynamics: once it can decay into the longitudinal gauge bosons it is a resonance broader than its mass, accompanied by the two non-unitary scalars of the charges $\pm1$ of about the same mass. The methods differ in degree, not in kind: CJT and KS have the lightest and narrowest amplitude mode, HS $(1,1)$ the heaviest and broadest. (b) In no method is the superfluid the Higgs field of the Standard Model, for the reason of structure stated at the beginning of this section. **Is the Higgs a composite particle?** In this model, yes, in every method, for the Higgs particle of the new group: the field $\vec\Phi$ is not a field of the Lagrangian, whose only matter field is the DiracLike spinor. It is the amplitude of the pair self-energy, a composite of two spinor charges; its kinetic part has no term of its own and is generated entirely by the spinor field ($Z^X_{\rm SF}$ and $Y^X_{\rm SF}$ are susceptibilities of the spinor field); and its amplitude mode is a bound state of two quasi-particles below their threshold. This is the compositeness condition of W. A. Bardeen, C. T. Hill and M. Lindner (1990): the normalization of the composite field vanishes at the cutoff, where only the spinor field exists. The same structure explains why the composite is heavy against the scale of the breaking: in the dynamics of Y. Nambu and G. Jona-Lasinio (1961) the mass of the bound state is tied to the gap of the spinor charges, while the scale of the breaking comes from a loop of the spinor field and is small against the gap. A light composite needs a large ratio of the cutoff to the gap, or a different mechanism, for example a scalar that is itself a pseudo-Goldstone boson of a larger broken symmetry (D. B. Kaplan and H. Georgi 1984). Whether the Higgs boson of nature is composite is not decided here, because the Higgs particle of this model is not that boson (answer (b)); the measured couplings of the Higgs boson of nature agree with those of an elementary Higgs field at the level of several per cent, which constrains, but does not exclude, its compositeness at a higher scale.*)

