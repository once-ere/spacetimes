(* ::Package:: *)

(* ::Chapter:: *)
(*Part III, second section. The free energy at finite temperature: the critical temperatures and the size of the order parameter*)

(* ::Text:: *)
(*On 21 November 2026 the author asked to add to the analysis and the solution the effective potential and the free energy of each of the five computed methods $X$ (HS $(1,1)$, HS $(0,1)$, HS $(0,0)$, KS and CJT), to determine whether the order parameter is then smaller and more physically reasonable, and to document this addendum with its provenance. The effective potential of each method is the subject of Section III.L. This chapter computes the free energy at the temperature $T$ with the engine of Part II, shows that its value at $T=0$ is the effective potential, finds the critical temperatures, and answers the question (Section III.T.6). The Rust program `sf_part3` (module `thermal`) and the Python program `sf_part3.py` compute the same things; the closed cell of Section III.T.2 holds their values for the comparison.*)

(* ::Section:: *)
(*III.T.1 The free energy of a state at the temperature T*)

(* ::Text:: *)
(***Units and conditions.** The temperature $T$ is measured in units of energy (Boltzmann's constant $k_{\rm B}=1$), in the units of the project ($\Lambda_{\rm LL}=1$). The chemical potential $\mu_{\rm LL}$ of the fermion number is zero, as everywhere in the solution: the states hold as many particles as antiparticles. **The thermal states.** At the temperature $T$ a uniform quasi-free state is the thermal state of its matrix of Bogoliubov and de Gennes $\mathsf H_{\rm SF}$ (Section II.2): at every momentum its generalized density, the $16\times16$ matrix of the normal and the pair correlations, is $f(\mathsf H_{\rm SF})$, the Fermi function $f(\varepsilon)=(e^{\varepsilon/T}+1)^{-1}$ applied to the eigenvalues $\varepsilon_k$ ($k=1,\dots,16$) of $\mathsf H_{\rm SF}$, with the occupations $f_k=f(\varepsilon_k)$. At $T=0$ the Fermi function is 1 for the negative and 0 for the positive eigenvalues, and the thermal state is the ground state of Part II. **The free energy** per volume of method $X$ is*)

(* ::DisplayFormula:: *)
(*\mathcal E^X_{\rm SF}-T\,\mathcal S_{\rm SF},\qquad\mathcal S_{\rm SF}=-\frac12\int\!\frac{d^3q}{(2\pi)^3}\sum_{k=1}^{16}\big[f_k\ln f_k+(1-f_k)\ln(1-f_k)\big],*)

(* ::Text:: *)
(*with the energy per volume $\mathcal E^X_{\rm SF}$ of method $X$ (unchanged, Section II.2) and the entropy per volume $\mathcal S_{\rm SF}$ of the quasi-free state. Every mode of the doubled space of $\mathsf H_{\rm SF}$ appears twice, at $\pm\varepsilon$ with the occupations $f$ and $1-f$, hence the factor $\tfrac12$. The self-energies are the derivatives of $\mathcal E^X_{\rm SF}$ as at $T=0$, and the stationary points of the free energy are the self-consistent thermal states. In HS this is the free energy of the saddle point of the auxiliary fields; in CJT the effective action at the temperature $T$ with the truncation of the method at $T=0$; in KS the functional of N. D. Mermin (the theory of Hohenberg, Kohn and Sham at finite temperature) with the entropy of the quasi-free state of Kohn and Sham and the exchange-correlation energy of the method at $T=0$ (in the exact theory of Mermin the exchange-correlation free energy depends on the temperature; here only the states do, an approximation of the method). **The free energy of the field** $\vec\Phi$ is the smallest free energy among the thermal states with this field, measured from the normal state at the same temperature:*)

(* ::DisplayFormula:: *)
(*\mathcal F^X_{\rm SF}(\vec\Phi;T)=\min\{\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}:\ \vec\Phi^X_{\rm SF}=\vec\Phi\}-\big(\mathcal E^X_{\rm SF}-T\mathcal S_{\rm SF}\big)_{\rm normal}.*)

(* ::Text:: *)
(*It is computed point by point as the effective potential of Section III.L: a fixed pair self-energy $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$ (the applied field) is put into $\mathsf H_{\rm SF}$, the normal self-energy is made self-consistent at the temperature $T$, and the thermal state gives one point, its own field $\Phi$ and its free energy. Its minimum lies at $\lvert\vec\Phi\rvert=\Phi^X_{0,{\rm SF}}(T)$, the **order parameter at the temperature $T$**: the modulus of the field of the self-consistent superfluid at $T$. At $T=0$ the entropy vanishes and $\mathcal F^X_{\rm SF}(\vec\Phi;0)=\mathcal V^X_{\rm SF}(\vec\Phi)$: **the effective potential is the free energy at zero temperature**, and $\Phi^X_{0,{\rm SF}}(0)=\Phi^X_{0,{\rm SF}}$. The next cell defines the thermal states of the engine of Part II: `thermalNode` makes $f(\mathsf H_{\rm SF})$ and the entropy of one node of the grid, `solveSFT` the self-consistent thermal state, and `fixedPairT` the thermal state of an applied pair field.*)

(* ::Input:: *)
(* fermiSF[e, t]: the Fermi function 1/(e^(e/t) + 1) of the energy e at the temperature t (the step function at t = 0) *)
fermiSF[e_?NumericQ, t_?NumericQ] := If[t <= 0, Which[e < 0, 1., e > 0, 0., True, 0.5], 1./(Exp[Clip[e/t, {-700., 700.}]] + 1.)];
(* binEntropy[f]: the entropy -(f ln f + (1 - f) ln(1 - f)) of a mode with the occupation f (zero at f = 0 and f = 1) *)
binEntropy[f_] := If[0 < f < 1, -(f Log[f] + (1 - f) Log[1 - f]), 0.];
(* thermalNode[h, t]: the thermal state f(H) of one BdG matrix h at the temperature t:
   {C^n, C^p, the sorted eigenvalues, the sum of the entropies of the 16 modes} *)
thermalNode[h_, t_] := Module[{ev, vec, f, r},
   {ev, vec} = Eigensystem[0.5 (h + ConjugateTranspose[h])]; ev = Re[ev];
   f = Map[fermiSF[#, t] &, ev];
   r = Transpose[vec] . DiagonalMatrix[f] . Conjugate[vec];   (* R = sum_k f_k v_k v_k^dag *)
   {r[[1 ;; 8, 1 ;; 8]], -r[[1 ;; 8, 9 ;; 16]], Sort[ev], Total[Map[binEntropy, f]]}];
(* solveSFT[mo, N0, P0, t, mix, tol, maxit]: the self-consistent thermal state at the temperature t (the stationary
   point of E - t S) from the initial self-energies N0, P0: the self-energies, the energy per volume (from the normal
   vacuum), the entropy per volume, the free energy per volume and the number of iterations *)
solveSFT[mo_, n0_, p0_, t_, mix_ : 0.6, tol_ : 10^-13, maxit_ : 400000] := Module[
   {ns = n0, ps = p0, res, cn, cp, ent, rho, kap, ek, ei, n2, p2, r, it = 0, out = None},
   While[out === None,
     res = Map[thermalNode[#, t] &, bdg[mo, ns, ps]];
     cn = res[[All, 1]]; cp = res[[All, 2]];
     ent = 0.5 mo["g"]["w"] . res[[All, 4]];
     {rho, kap, ek} = localDens[mo, cn, cp];
     {ei, n2, p2} = interaction[mo["ts"], rho, kap];
     r = Max[Abs[n2 - ns], Abs[p2 - ps]];
     If[r < tol || it >= maxit,
       out = <|"n" -> n2, "p" -> p2, "e" -> ek + ei, "s" -> ent, "free" -> ek + ei - t ent, "it" -> it, "res" -> r|>,
       ns = mix n2 + (1 - mix) ns; ps = mix p2 + (1 - mix) ps;
       ns = 0.5 (ns + ConjugateTranspose[ns]); ps = 0.5 (ps - Transpose[ps]);
       it++]];
   out];
(* fixedPairT[mo, pfix, N0, t]: the thermal state of the applied pair self-energy pfix at the temperature t, with the
   normal self-energy made self-consistent: {N, the field Phi of the state (the modulus of the amplitude of its own
   pair self-energy in the channel K^3), its free energy per volume} *)
fixedPairT[mo_, pfix_, n0_, t_, tol_ : 10^-14] := Module[{ns = n0, res, cn, cp, ent, rho, kap, ek, ei, n2, p2, done = False, it = 0},
   While[! done,
     res = Map[thermalNode[#, t] &, bdg[mo, ns, pfix]];
     cn = res[[All, 1]]; cp = res[[All, 2]];
     ent = 0.5 mo["g"]["w"] . res[[All, 4]];
     {rho, kap, ek} = localDens[mo, cn, cp];
     {ei, n2, p2} = interaction[mo["ts"], rho, kap];
     If[Max[Abs[n2 - ns]] < tol || it >= 200000, done = True,
       ns = 0.7 n2 + 0.3 ns; ns = 0.5 (ns + ConjugateTranspose[ns]); it++]];
   {n2, Abs[Tr[k8[3] . p2]/8], ek + ei - t ent}];
(* phiOf[sol]: the order parameter of a self-consistent state, the modulus of the amplitude of its pair self-energy *)
phiOf[sol_] := Abs[Tr[k8[3] . sol["p"]]/8];
Print["the thermal engine is defined"];

(* ::Section:: *)
(*III.T.2 The values of the Rust program*)

(* ::Text:: *)
(*The closed cell holds the thermal results of the Rust program `sf_part3`: `rustThermal` (per method and reduced temperature $T/T^{X,{\rm c}}_{\rm SF}$: the temperature, the order parameter, the Dirac masses of the superfluid and of the normal state, the gap, the radius of convergence, the free energy at the minimum, the difference of the entropies, the residual of the gap equation, the Taylor coefficients, the order parameter of Landau's theory and the number of iterations), `rustThermalLandau` (the residuals of the Landau forms) and `rustThermalCurves` (the free-energy curves: the applied field, the field of the state and its free energy). They were copied from the data files named, with their SHA-256 checksums, in the comment at the top of the cell, by `wolfram/tools/make_data_cells.py`.*)

(* ::Input::Closed:: *)
(* @@INCLUDE data_part3_thermal.wl@@ *)
Print["rows of the Rust tables: ", Length[rustThermal], " (order parameter), ", Length[rustThermalLandau], " (Landau forms), ", Length[rustThermalCurves], " (curve points)"];

(* ::Input:: *)
(* the columns of rustThermal: method, tau, T, Phi0, Dirac mass, gap, Dirac mass of the normal state, radius,
   Phi0/radius, F at the minimum, S - S_normal, residual of the gap equation, a, b, Phi0 of Landau's theory, iterations *)
thCol = AssociationThread[{"method", "tau", "T", "Phi0", "mass", "gap", "massN", "radius", "ratio", "F", "dS", "gapeq",
     "a", "b", "phiL", "it"} -> Range[16]];
(* thRow[t, tau]: the row of the method t at the reduced temperature tau; thGet[t, tau, key]: one entry *)
thRow[t_, tau_] := First[Select[rustThermal, #[[1]] == t && Abs[#[[2]] - tau] < 10^-9 &]];
thGet[t_, tau_, key_] := thRow[t, tau][[thCol[key]]];
thTaus = Union[rustThermal[[All, 2]]];
Print["the reduced temperatures of the table: ", thTaus];

(* ::Section:: *)
(*III.T.3 The limit of zero temperature*)

(* ::Text:: *)
(*The next cell takes the ground states of $V^{(2)}_{\rm SF}$ of Part II (`groundSF`), makes them self-consistent to $10^{-13}$, and computes the thermal states at $T=0.005$, where $e^{-E^{X,{\rm gap}}_{\rm SF}/T}<10^{-25}$: the order parameter and the free energy measured from the normal state at the same temperature must be those of $T=0$.*)

(* ::Input:: *)
(* thModel[t], thGround[t]: the model of V^(2) and the ground state at T = 0 of the method t (from Part II, iterated to 1e-13) *)
thTags = {"HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"};
thName = <|"HS_1_1" -> "HS (1, 1)", "HS_0_1" -> "HS (0, 1)", "HS_0_0" -> "HS (0, 0)", "KS" -> "KS", "CJT" -> "CJT"|>;
thModel[t_] := groundSF["V2_" <> t][[1]];
thGround[t_] := thGround[t] = Module[{s = groundSF["V2_" <> t][[2]]}, solveSF[thModel[t], s["n"], s["p"], 0.6, 10^-13]];
Do[Module[{mo = thModel[t], g0 = thGround[t], low, nlow},
   low = solveSFT[mo, g0["n"], g0["p"], 0.005, 0.6, 10^-13];
   nlow = solveSFT[mo, zero8, zero8, 0.005, 0.6, 10^-13];
   check["WP3-T1 " <> t, thName[t] <> ": the thermal states at T = 0.005 reproduce the ground state of T = 0 (|change of Phi0| + relative change of the energy)",
    Abs[phiOf[low] - phiOf[g0]] + Abs[(low["free"] - nlow["free"]) - g0["e"]]/Abs[g0["e"]], 10^-11, "P3-T1 " <> t, "YP3-T1 " <> t]],
  {t, {"HS_1_1", "CJT"}}];

(* ::Section:: *)
(*III.T.4 The critical temperature*)

(* ::Text:: *)
(***The effective couplings.** A uniform scalar density $n^{\rm S}_{\rm SF}$ shifts the Dirac mass by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$, and a pair amplitude in the channel $\mathsf K^3_{\rm LL}$ produces the pair self-energy with the strength $g^{X,{\rm P}}_{\rm SF}$ (Section II.3). For a state with the Dirac mass $M^{X,{\rm D}}_{\rm SF}$ and the pair field $\Phi$ at the temperature $T$ the scalar density is*)

(* ::DisplayFormula:: *)
(*n^{\rm S}_{\rm SF}=-\frac2{\pi^2}\Big[M^{X,{\rm D}}_{\rm SF}\,I^{(1)}_{\rm SF}(E^{X,{\rm gap}}_{\rm SF},T)-m_{\rm LL}I^{(1)}_{\rm SF}(m_{\rm LL},0)\Big],\qquad I^{(1)}_{\rm SF}(M,T)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\tanh(E_M/2T)}{E_M}\,dq,*)

(* ::Text:: *)
(*with $E_M=(q^2+M^2)^{1/2}$ and the gap $E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+\Phi^2)^{1/2}$; the factor $\tanh(E/2T)=1-2f(E)$ is the thermal factor of a pair of quasi-particles, and $I^{(1)}_{\rm SF}(M,0)$ is the integral of Part II. The Dirac mass is self-consistent when $M^{X,{\rm D}}_{\rm SF}-m_{\rm LL}=g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$. **The normal state and its instability.** The normal state at $T$ has no pair field and the Dirac mass $M^{X,{\rm n}}_{\rm SF}(T)$ that solves this condition with $\Phi=0$: $m_{\rm LL}$ at $T=0$; at $T>0$ the thermally excited particles and antiparticles change the scalar density, which shifts the mass where $g^{X,{\rm S}}_{\rm SF}\ne0$. A small pair field in the normal state is amplified by the factor*)

(* ::DisplayFormula:: *)
(*\lambda^{\rm P}_{\rm SF}(T)=\frac{g^{X,{\rm P}}_{\rm SF}\,I^{(1)}_{\rm SF}\big(M^{X,{\rm n}}_{\rm SF}(T),T\big)}{\pi^2},*)

(* ::Text:: *)
(*the eigenvalue of the static pair response of the normal state: the normal state is unstable towards a superfluid when $\lambda^{\rm P}_{\rm SF}(T)>1$ (the criterion of Thouless). The **critical temperature** $T^{X,{\rm c}}_{\rm SF}$ solves $\lambda^{\rm P}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})=1$. The next cell computes $M^{X,{\rm n}}_{\rm SF}(T)$ from the scalar self-consistency alone (bisection), $\lambda^{\rm P}_{\rm SF}(T)$ with the integral on the radial nodes of the grid of Part II, and $T^{X,{\rm c}}_{\rm SF}$ by bisection; for the three methods with $g^{X,{\rm S}}_{\rm SF}\ne0$ it compares $M^{X,{\rm n}}_{\rm SF}(T^{X,{\rm c}}_{\rm SF})$ with the normal state of the thermal engine. It then shows that $\lambda^{\rm P}_{\rm SF}(T)$ does not increase from $T=0.01$ to $T=0.6$, so that $T^{X,{\rm c}}_{\rm SF}$ is the only solution there.*)

(* ::Input:: *)
(* i1Grid[mo, s, t]: I1 = int_0^Lambda q^2 tanh(E/2t)/E dq with E = (q^2 + s)^(1/2), on the radial nodes of the grid *)
i1Grid[mo_, s_, t_] := Module[{kr = mo["g"]["kr"], wr = mo["g"]["wr"], e},
   e = Sqrt[kr^2 + s];
   Total[wr kr^2 If[t > 0, Tanh[e/(2 t)], 1.]/e]];
(* thCoup[t]: the effective couplings {g^S, g^P} of the method t for V^(2) (Part II) *)
thCoup[t_] := thCoup[t] = effCouplings[Lookup[AssociationThread[shortName /@ methodsSF, methodsSF], t], Sequence @@ potentialsSF["V2"]];
(* thMassN[t, T]: the Dirac mass of the normal state at the temperature T from M - m = g^S n^S (bisection on [1e-6, 2]) *)
thMassN[t_, temp_] := Module[{mo = thModel[t], gs = thCoup[t][[1]], lo = 10.^-6, hi = 2., mid, i1m},
   If[Abs[gs] < 10^-14, Return[N[mLL], Module]];
   i1m = i1Grid[mo, N[mLL]^2, 0.];
   Do[mid = 0.5 (lo + hi);
     If[mid - mLL + gs 2/Pi^2 (mid i1Grid[mo, mid^2, temp] - mLL i1m) > 0, hi = mid, lo = mid], {80}];
   0.5 (lo + hi)];
(* thLam[t, T]: lambda^P(T) of the normal state; thTc[t]: the critical temperature by bisection on [0.01, 1] *)
thLam[t_, temp_] := thCoup[t][[2]] i1Grid[thModel[t], thMassN[t, temp]^2, temp]/Pi^2;
thTc[t_] := thTc[t] = Module[{lo = 0.01, hi = 1., mid},
   Do[mid = 0.5 (lo + hi); If[thLam[t, mid] > 1, lo = mid, hi = mid], {60}]; 0.5 (lo + hi)];
Do[Module[{tc = thTc[t], dev, extra = ""},
   dev = Abs[tc - rustPart3["p3_th_Tc_" <> t]]/rustPart3["p3_th_Tc_" <> t] + Abs[thLam[t, 0.] - rustPart3["p3_th_lam0_" <> t]];
   If[Abs[thCoup[t][[1]]] > 10^-14,
     Module[{ns = solveSFT[thModel[t], zero8, zero8, tc, 0.6, 10^-14]},
       dev += Abs[diracMass[ns] - thMassN[t, tc]];
       extra = "; the Dirac mass of the normal state of the thermal engine at T_c = " <> ToString[NumberForm[diracMass[ns], 10]] <> " = the scalar self-consistency"]];
   check["WP3-T3 " <> t, thName[t] <> ": the critical temperature T_c = " <> ToString[NumberForm[tc, 10]] <> " (bisection of lambda^P(T) = 1) = Rust (relative), and lambda^P(0) = " <> ToString[NumberForm[thLam[t, 0.], 10]] <> " = Rust" <> extra,
    dev, 10^-10, "P3-T3 " <> t, If[MemberQ[{"HS_1_1", "CJT"}, t], "YP3-T3 " <> t <> ", YP3-T9 " <> t, "YP3-T9 " <> t]]], {t, thTags}];
Do[Module[{lams = Table[thLam[t, temp], {temp, Subdivide[0.01, 0.6, 59]}], rise},
   rise = Max[0., Max[Differences[lams]]];
   check["WP3-T9 " <> t, thName[t] <> ": lambda^P(T) of the normal state does not increase on 60 temperatures from 0.01 to 0.6 (from " <> ToString[NumberForm[First[lams], 6]] <> " to " <> ToString[NumberForm[Last[lams], 6]] <> "; the largest rise), so T_c is the only solution of lambda^P(T) = 1 there",
    rise, 10^-12, "P3-T9 " <> t, "YP3-T9 " <> t]], {t, thTags}];
Grid[Prepend[Table[{thName[t], NumberForm[thCoup[t][[1]], 6], NumberForm[thCoup[t][[2]], 6], NumberForm[thLam[t, 0.], 8], NumberForm[thTc[t], 8]}, {t, thTags}],
   {"method", "g^S", "g^P", "lambda^P(0)", "T_c"}], Frame -> All, Alignment -> Right]

(* ::Section:: *)
(*III.T.5 The gap equation; the Dirac mass of the superfluid does not depend on the temperature*)

(* ::Text:: *)
(*The polar superfluid at the temperature $T$ has the pair field $\Phi^X_{0,{\rm SF}}(T)$, and its pair self-consistency is the **gap equation***)

(* ::DisplayFormula:: *)
(*1=\frac{g^{X,{\rm P}}_{\rm SF}}{\pi^2}\,I^{(1)}_{\rm SF}\Big(\big((M^{X,{\rm D}}_{\rm SF})^2+\Phi^X_{0,{\rm SF}}(T)^2\big)^{1/2},T\Big).*)

(* ::Text:: *)
(*The pair coupling alone fixes the gap at every temperature. The scalar density of the superfluid contains the same integral, which the gap equation fixes to $\pi^2/g^{X,{\rm P}}_{\rm SF}$: $n^{\rm S}_{\rm SF}=-2M^{X,{\rm D}}_{\rm SF}/g^{X,{\rm P}}_{\rm SF}+2m_{\rm LL}I^{(1)}_{\rm SF}(m_{\rm LL},0)/\pi^2$. The scalar self-consistency $M^{X,{\rm D}}_{\rm SF}-m_{\rm LL}=g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$ therefore contains no temperature: **the Dirac mass of the superfluid does not depend on the temperature** below $T^{X,{\rm c}}_{\rm SF}$, and the order parameter follows from the gap equation alone. The next cell computes, with the thermal engine, the superfluid of CJT at $0.5\,T^{\rm CJT,c}_{\rm SF}$ and $0.9\,T^{\rm CJT,c}_{\rm SF}$ (from the ground state of $T=0$) and checks its order parameter, its free energy, its gap equation and its Dirac mass; it then solves the gap equation alone for HS $(0,0)$, whose Dirac mass is $m_{\rm LL}$, at the 13 temperatures of the table.*)

(* ::Input:: *)
(* the superfluid of CJT at 0.5 T_c and 0.9 T_c with the thermal engine; thSF[tau] = {superfluid, normal state} *)
thSF = <||>;
Module[{mo = thModel["CJT"], gp = thCoup["CJT"][[2]], n0 = thGround["CJT"]["n"], p0 = thGround["CJT"]["p"], dev = 0., gapdev = 0., mdev = 0., m0 = diracMass[thGround["CJT"]]},
  Do[Module[{temp = thGet["CJT", tau, "T"], sf, nn, phi, mass},
     sf = solveSFT[mo, n0, p0, temp, 0.6, 10^-13];
     nn = solveSFT[mo, zero8, zero8, temp, 0.6, 10^-14];
     n0 = sf["n"]; p0 = sf["p"];
     thSF[tau] = {sf, nn};
     phi = phiOf[sf]; mass = diracMass[sf];
     dev = Max[dev, Abs[phi - thGet["CJT", tau, "Phi0"]], Abs[(sf["free"] - nn["free"]) - thGet["CJT", tau, "F"]]/Abs[thGet["CJT", tau, "F"]]];
     gapdev = Max[gapdev, Abs[gp i1Grid[mo, mass^2 + phi^2, temp]/Pi^2 - 1]];
     mdev = Max[mdev, Abs[mass - m0], Abs[mass - thGet["CJT", tau, "mass"]]]], {tau, {0.5, 0.9}}];
  check["WP3-T2 CJT", "CJT: the superfluid at 0.5 T_c and 0.9 T_c (thermal engine): the order parameter and the free energy = Rust (largest difference; the free energy relative), and the gap equation holds (largest |g^P I1/pi^2 - 1|; the sum of both)",
   dev + gapdev, 10^-8, "P3-T2 CJT", "YP3-T2 CJT"];
  check["WP3-T8 CJT", "CJT: the Dirac mass of the superfluid at 0.5 T_c and 0.9 T_c equals that at T = 0, " <> ToString[NumberForm[m0, 10]] <> ", and the Rust value (largest difference)",
   mdev, 10^-10, "P3-T8 CJT", "YP3-T8 CJT"]];
(* HS (0, 0): the order parameter from the gap equation alone, g^P I1(m^2 + Phi0^2, T)/pi^2 = 1 (bisection on [0, 2]) *)
thGapPhi[t_, temp_, mass_] := Module[{mo = thModel[t], gp = thCoup[t][[2]], lo = 0., hi = 2., mid},
   Do[mid = 0.5 (lo + hi); If[gp i1Grid[mo, mass^2 + mid^2, temp]/Pi^2 > 1, lo = mid, hi = mid], {100}]; 0.5 (lo + hi)];
thPhiGap00 = Table[{tau, thGapPhi["HS_0_0", thGet["HS_0_0", tau, "T"], N[mLL]]}, {tau, thTaus}];
check["WP3-T2 HS_0_0", "HS (0, 0): the order parameter of every temperature of the table from the gap equation alone (with the Dirac mass m_LL, without the self-consistent engine) = Rust (largest difference)",
  Max[Map[Abs[#[[2]] - thGet["HS_0_0", #[[1]], "Phi0"]] &, thPhiGap00]], 10^-9, "P3-T2 HS_0_0", "YP3-T4"];

(* ::Section:: *)
(*III.T.6 The size of the order parameter, Landau's theory, and the answer*)

(* ::Text:: *)
(***The radius of convergence.** The free energy is built from integrals over the momenta of functions of the quasi-particle energies $E=(q^2+M^2+\Phi_{\rm a}^2)^{1/2}$ and of their Fermi functions: the terms $-T\ln(2\cosh(E/2T))$, one for each pair of eigenvalues $\pm E$ (at the Fermi occupation $f=f(E)$ the energy $E(f-\tfrac12)$ of the pair and its term $T[f\ln f+(1-f)\ln(1-f)]$ in $-T\mathcal S_{\rm SF}$ add up to $-T\ln(2\cosh(E/2T))$), and the densities with the factor $\tanh(E/2T)/E$. With the Matsubara frequencies $\omega_n=(2n+1)\pi T$ they become sums over $n=0,1,2,\dots$,*)

(* ::DisplayFormula:: *)
(*\ln\big(2\cosh(E/2T)\big)=\ln2+\sum_{n=0}^\infty\ln\Big(1+\frac{E^2}{\omega_n^2}\Big),\qquad\frac{\tanh(E/2T)}{E}=4T\sum_{n=0}^\infty\frac{1}{\omega_n^2+E^2},*)

(* ::Text:: *)
(*of functions of $\omega_n^2+q^2+M^2+\Phi_{\rm a}^2$. Integrated over the momenta, each is singular where $\omega_n^2+M^2+\Phi_{\rm a}^2=0$ (at the end $q=0$ of the integral); the nearest of these points to the origin, for the Dirac mass of the normal state held fixed, is $\Phi_{\rm a}^2=-(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2)$, and at $T=0$ it is the point $\Phi_{\rm a}^2=-m_{\rm LL}^2$ of Section III.L.5. The series of $\mathcal F^X_{\rm SF}$ in powers of $\lvert\vec\Phi\rvert^2$ about the normal state therefore converges within a radius of the order of*)

(* ::DisplayFormula:: *)
(*r^X_{\rm SF}(T)=\big(M^{X,{\rm n}}_{\rm SF}(T)^2+\pi^2T^2\big)^{1/2},*)

(* ::Text:: *)
(*not exactly equal to it (the Dirac mass changes with the applied field where $g^{X,{\rm S}}_{\rm SF}\ne0$, and the field of the states is not the applied field). Because $r^X_{\rm SF}$ is an estimate, the answer rests also on two direct measures that do not use it. **Landau's theory** expands the free energy at the symmetric state, $\mathcal F^X_{\rm SF}=a^{X,{\rm Tay}}_{\rm SF}(T)\Phi^2+b^{X,{\rm Tay}}_{\rm SF}(T)\Phi^4+\dots$ along the polar fields, and predicts the order parameter $(-a/2b)^{1/2}$, with $a=a^{X,{\rm Tay}}_{\rm SF}(T)$ and $b=b^{X,{\rm Tay}}_{\rm SF}(T)$. The next cell computes them for CJT at $0.9\,T^{\rm CJT,c}_{\rm SF}$ from the thermal states of four small applied fields, $\Phi_{\rm a}=0.01$ to $0.04$ ($\mathcal F^X_{\rm SF}/\Phi^2$ interpolated exactly by a cubic polynomial in $\Phi^2$, whose value and slope at $\Phi^2=0$ are $a$ and $b$). The cell after it fits the **Landau form** of Section III.L ($a\Phi^2+b\Phi^4+c\Phi^6$) and the quartic form ($a\Phi^2+b\Phi^4$) to the free-energy curves of the Rust program at six temperatures, with the least squares of Section III.L.4.*)

(* ::Input:: *)
(* the Taylor coefficients a, b of the free energy of CJT at 0.9 T_c at the origin, and the order parameter of Landau's theory *)
Module[{mo = thModel["CJT"], temp = thGet["CJT", 0.9, "T"], nn = thSF[0.9][[2]], n0, xs = {}, ys = {}, c, a, b, err, errRust},
  n0 = nn["n"];
  Do[Module[{res = fixedPairT[mo, fa ConjugateTranspose[k8[3]], n0, temp]},
     n0 = res[[1]];
     AppendTo[xs, res[[2]]^2]; AppendTo[ys, (res[[3]] - nn["free"])/res[[2]]^2]], {fa, {0.01, 0.02, 0.03, 0.04}}];
  (* the cubic polynomial in the scaled variable x/xmax, whose matrix is well conditioned; then back to a, b *)
  c = LinearSolve[Table[(x/Max[xs])^k, {x, xs}, {k, 0, 3}], ys];
  {a, b} = {c[[1]], c[[2]]/Max[xs]};
  err = Sqrt[-a/(2 b)]/thGet["CJT", 0.9, "Phi0"] - 1;
  errRust = thGet["CJT", 0.9, "phiL"]/thGet["CJT", 0.9, "Phi0"] - 1;
  check["WP3-T5 CJT", "CJT at 0.9 T_c: the Taylor coefficients a = " <> ToString[NumberForm[a, 8]] <> ", b = " <> ToString[NumberForm[b, 8]] <> " of the free energy at the origin and the error " <> sci[err] <> " of the order parameter (-a/2b)^(1/2) of Landau's theory (" <> ToString[NumberForm[Abs[err]/0.1, 3]] <> " times 1 - T/T_c) = Rust (largest relative difference of a and b, difference of the error)",
   Max[Abs[a - thGet["CJT", 0.9, "a"]]/Abs[thGet["CJT", 0.9, "a"]], Abs[b - thGet["CJT", 0.9, "b"]]/Abs[thGet["CJT", 0.9, "b"]], Abs[err - errRust]], 10^-5, "P3-T5 CJT", "YP3-T5 CJT"]];

(* ::Input:: *)
(* the Landau form and the quartic form fitted to the free-energy curves of the Rust program on [0, 1.4 Phi0(T)];
   thRes[t][tau] = {residual of the Landau form, residual of the quartic form}, relative to the depth *)
thCurveTaus = {0., 0.5, 0.8, 0.9, 0.95, 0.98};
thRes = Association[Map[Function[t, t -> Association[Map[Function[tau, Module[{pts, phi0 = thGet[t, tau, "Phi0"], depth = Abs[thGet[t, tau, "F"]], sel},
        pts = Select[rustThermalCurves, #[[1]] == t && Abs[#[[2]] - tau] < 10^-9 &][[All, {4, 5}]];
        sel = Select[pts, #[[1]] <= 1.4 phi0 &];
        tau -> {evenFit[sel, 3][[2]]/depth, evenFit[sel, 2][[2]]/depth}]], thCurveTaus]]], thTags]];
(* thRustRes[t, tau]: the two residuals of the Rust program (the reduced temperatures of the data are compared within 1e-9) *)
thRustRes[t_, tau_] := First[Select[rustThermalLandau, #[[1]] == t && Abs[#[[2]] - tau] < 10^-9 &]][[3 ;; 4]];
Do[Module[{rr = thRes[t], dev, falls},
   dev = Max[Table[Max[Abs[rr[tau] - thRustRes[t, tau]]/rr[tau]], {tau, thCurveTaus}]];
   falls = rr[0.9][[2]] > rr[0.95][[2]] > rr[0.98][[2]] && rr[0.98][[1]] < 10^-4;
   check["WP3-T6 " <> t, thName[t] <> ": the largest residuals of the Landau form and of the quartic form at T/T_c = 0, 0.5, 0.8, 0.9, 0.95, 0.98, recomputed by LeastSquares from the curves of the Rust program = Rust (largest relative difference); the quartic residual falls towards T_c (" <> sci[rr[0.9][[2]]] <> ", " <> sci[rr[0.95][[2]]] <> ", " <> sci[rr[0.98][[2]]] <> " of the depth at 0.9, 0.95, 0.98 T_c) and that of the Landau form is " <> sci[rr[0.98][[1]]] <> " at 0.98 T_c: " <> If[falls, "yes", "NO"],
    dev + If[falls, 0, 1], 10^-7, "P3-T6 " <> t, "YP3-T6 " <> t];
   check["WP3-T7 " <> t, thName[t] <> ": at T = 0 the free energy is the effective potential: the residual of the Landau form of the curve at T = 0 is that of Section III.L (relative difference)",
    Abs[rr[0.][[1]] - lfResults[t]["resK3"]/Abs[lfE0[t]]]/rr[0.][[1]], 10^-8, "P3-T7 " <> t, "YP3-T7 " <> t]], {t, thTags}];

(* ::Text:: *)
(*The next cell prints the numbers of the answer: the critical temperature, the order parameter at $T=0$ and at $0.98\,T^{X,{\rm c}}_{\rm SF}$, the radius of convergence at $0.98\,T^{X,{\rm c}}_{\rm SF}$ (from the Dirac mass of the normal state computed by the notebook), the reduced temperature above which the order parameter lies inside the radius, and the residuals of the two forms at $T=0$ and at $0.98\,T^{X,{\rm c}}_{\rm SF}$ (in per cent of the depth). The two cells of the figure after it show the order parameter and the radius against $T/T^{X,{\rm c}}_{\rm SF}$ and their ratio, and the residuals of the two forms.*)

(* ::Input:: *)
(* thRadius[t, tau]: the radius (M_n(T)^2 + pi^2 T^2)^(1/2) with the Dirac mass of the normal state computed by the notebook *)
thRadius[t_, tau_] := Module[{temp = tau thTc[t]}, Sqrt[thMassN[t, temp]^2 + Pi^2 temp^2]];
(* thCross[t]: the reduced temperature above which Phi0(T) < r(T) (linear interpolation in the table of temperatures) *)
thCross[t_] := Module[{v = Table[{tau, thGet[t, tau, "Phi0"]/thRadius[t, tau]}, {tau, thTaus}], k},
   k = First[Flatten[Position[Differences[Sign[v[[All, 2]] - 1]], d_ /; d != 0]]];
   v[[k, 1]] + (v[[k, 2]] - 1) (v[[k + 1, 1]] - v[[k, 1]])/(v[[k, 2]] - v[[k + 1, 2]])];
Grid[Prepend[Table[{thName[t], NumberForm[thTc[t], 6], NumberForm[thGet[t, 0., "Phi0"], 6], NumberForm[thGet[t, 0.98, "Phi0"], 6],
     NumberForm[thRadius[t, 0.98], 6], NumberForm[thCross[t], 3], NumberForm[100 thRes[t][0.][[1]], 3], NumberForm[100 thRes[t][0.98][[1]], 3],
     NumberForm[100 thRes[t][0.][[2]], 3], NumberForm[100 thRes[t][0.98][[2]], 3]}, {t, thTags}],
   {"method", "T_c", "Phi0(0)", "Phi0(0.98 T_c)", "r(0.98 T_c)", "Phi0 < r above T/T_c =", "Landau form, T = 0 (%)", "Landau form, 0.98 T_c (%)",
    "quartic form, T = 0 (%)", "quartic form, 0.98 T_c (%)"}], Frame -> All, Alignment -> Right, BaseStyle -> {FontSize -> 8}]

(* ::Input:: *)
(* the figure, first cell: top, the order parameter Phi0(T) of the Rust table (points and lines) and the radius r(T)
   computed by the notebook (dashed), against T/T_c (KS and CJT coincide; one curve for both); bottom, the ratio
   Phi0(T)/r(T) (below 1: inside the radius of convergence); one panel above the other, so that the two fit the width
   and the height of a printed page *)
thShown = {"HS_1_1", "HS_0_1", "HS_0_0", "CJT"};
thLegend = {"HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "CJT = KS"};
thColors = {RGBColor["#2a78d6"], RGBColor["#eb6834"], RGBColor["#1baf7a"], RGBColor["#eda100"]};
Column[{
  Show[ListLinePlot[Table[Table[{tau, thGet[t, tau, "Phi0"]}, {tau, thTaus}], {t, thShown}],
     PlotStyle -> Table[Directive[thColors[[i]], AbsoluteThickness[1.4]], {i, 4}], PlotMarkers -> {Automatic, 5},
     PlotLegends -> Placed[thLegend, {Left, Top}], Frame -> True, FrameLabel -> {"T / Tc", "order parameter \[CapitalPhi]0(T)"},
     PlotRange -> {0, 0.9}, ImageSize -> {460, 260}, AspectRatio -> Full,
     PlotLabel -> "the order parameter falls to zero at Tc; dashed: the radius r(T)"],
   ListLinePlot[Table[Table[{tau, thRadius[t, tau]}, {tau, 0., 0.98, 0.02}], {t, thShown}],
     PlotStyle -> Table[Directive[thColors[[i]], AbsoluteThickness[1.], AbsoluteDashing[{4, 3}]], {i, 4}]]],
  ListLinePlot[Table[Table[{tau, thGet[t, tau, "Phi0"]/thRadius[t, tau]}, {tau, thTaus}], {t, thShown}],
   PlotStyle -> Table[Directive[thColors[[i]], AbsoluteThickness[1.4]], {i, 4}], PlotMarkers -> {Automatic, 5},
   PlotLegends -> Placed[thLegend, {Right, Top}], Frame -> True, FrameLabel -> {"T / Tc", "\[CapitalPhi]0(T) / r(T)"},
   GridLines -> {None, {1}}, ImageSize -> {460, 260}, AspectRatio -> Full, PlotLabel -> "below 1: inside the radius of convergence"]},
 Spacings -> 1.5]

(* ::Input:: *)
(* the figure, second cell: the residuals of the Landau form (solid) and of the quartic form (dotted) recomputed by the
   notebook, relative to the depth, in per cent (logarithmic scale), against T/T_c; the colors of the methods as above *)
ListLogPlot[Join[Table[Table[{tau, 100 thRes[t][tau][[1]]}, {tau, thCurveTaus}], {t, thShown}],
   Table[Table[{tau, 100 thRes[t][tau][[2]]}, {tau, thCurveTaus}], {t, thShown}]], Joined -> True,
 PlotStyle -> Join[Table[Directive[thColors[[i]], AbsoluteThickness[1.4]], {i, 4}], Table[Directive[thColors[[i]], AbsoluteThickness[1.], AbsoluteDashing[{1, 2}]], {i, 4}]],
 PlotMarkers -> {Automatic, 5}, PlotLegends -> Placed[thLegend, {Left, Bottom}], Frame -> True,
 FrameLabel -> {"T / Tc", "largest residual / depth (per cent)"}, ImageSize -> {460, 280}, AspectRatio -> Full,
 PlotLabel -> "solid: the Landau form; dotted: the quartic form"]

(* ::Text:: *)
(***Is the order parameter smaller and more physically reasonable?** Yes, near the critical temperature, and only there.*)

(* ::ItemNumbered:: *)
(***It is smaller.** $\Phi^X_{0,{\rm SF}}(T)$ keeps its value of $T=0$ up to about $0.3\,T^{X,{\rm c}}_{\rm SF}$ (the gap is much larger than these temperatures, and thermal quasi-particles are exponentially rare), then falls and vanishes continuously at $T^{X,{\rm c}}_{\rm SF}$, like $(1-T/T^{X,{\rm c}}_{\rm SF})^{1/2}$ (the table and the figure above).*)

(* ::ItemNumbered:: *)
(***It lies inside the radius of convergence.** At $T=0$ the order parameter is 1.5 to 2 times the radius, which is then the mass $m_{\rm LL}$; above the reduced temperature of the table (0.35 to 0.42) it is smaller than the radius, and near $T^{X,{\rm c}}_{\rm SF}$ much smaller. There the series of the free energy in powers of $\lvert\vec\Phi\rvert^2$ converges at the minimum, and the Landau coefficients are the Taylor coefficients of the free energy: the regime of Landau's theory, which Section III.L found absent at $T=0$.*)

(* ::ItemNumbered:: *)
(***The Landau form becomes exact.** The residual of the Landau form falls from a few tenths of a per cent of the depth at $T=0$ to about $10^{-3}$ per cent at $0.98\,T^{X,{\rm c}}_{\rm SF}$, and that of the quartic form (the form of the Higgs potential of the Standard Model and of the theory of Ginzburg and Landau) from 3 to 6 per cent to about 0.3 per cent; near $T^{X,{\rm c}}_{\rm SF}$ it falls in proportion to $1-T/T^{X,{\rm c}}_{\rm SF}$.*)

(* ::ItemNumbered:: *)
(***Landau's theory predicts the order parameter.** Near $T^{X,{\rm c}}_{\rm SF}$ the error of $(-a/2b)^{1/2}$ is a fraction (0.19 to 0.28 in the Rust table) of $1-T/T^{X,{\rm c}}_{\rm SF}$, as Landau's theory requires; the cell above computes it for CJT at $0.9\,T^{\rm CJT,c}_{\rm SF}$.*)

(* ::ItemNumbered:: *)
(***The scales.** Near $T^{X,{\rm c}}_{\rm SF}$ the order parameter is small also against the Dirac mass and the cutoff. The critical temperatures themselves are about a fifth to a quarter of the cutoff (the table above), and the states at the cutoff are occupied by 0.8 to 1.8 per cent at $0.98\,T^{X,{\rm c}}_{\rm SF}$, so the thermal physics reaches the cutoff only weakly. The ratio of the gap at $T=0$ to $T^{X,{\rm c}}_{\rm SF}$ is 1.65 to 1.74; the theory of Bardeen, Cooper and Schrieffer gives $\pi e^{-\gamma_{\rm E}}=1.764$ (with Euler's constant $\gamma_{\rm E}=0.5772$) for weak pairing at a Fermi surface. The ratios are of the same size but not equal: here there is no Fermi surface ($\mu_{\rm LL}=0$), the pairing needs a coupling above a critical one, and the gap contains the Dirac mass.*)

(* ::Section:: *)
(*III.T.7 What the free energy of these methods does not contain, and why it matters*)

(* ::Text:: *)
(***The remark.** All three methods give mean-field free energies: the thermal state is quasi-free, so the free energy contains the thermal quasi-particles of the spinor field, but neither the thermal fluctuations of the order parameter about its mean value (its amplitude, Goldstone and non-unitary modes) nor thermal gauge fields. **Why it holds.** A quasi-free state is a state whose correlations of four and more fields are fixed by its correlations of two fields (Wick's theorem); the three methods choose the best such state, and a fluctuation of the order parameter is a correlation of two pairs beyond the product of the pair densities. In HS the integral over the auxiliary fields is replaced by its saddle point, in CJT the effective action is truncated at the contractions of the contact terms in pairs, and in KS the exchange-correlation energy has no thermal part.*)

(* ::Text:: *)
(***Why it matters.** Near a continuous transition the fluctuations of the order parameter grow without bound. Their size relative to the order parameter is measured by the Ginzburg number of the transition (A. P. Levanyuk 1959, V. L. Ginzburg 1960): the mean-field free energy is accurate where $1-T/T^{X,{\rm c}}_{\rm SF}$ is much larger than the Ginzburg number, and it fails within the critical region, where it is smaller. For a free energy of the form of Ginzburg and Landau in three dimensions, $\gamma_{\rm GL}\lvert\nabla\psi_{\rm GL}\rvert^2+\alpha'_{\rm GL}(T/T_{\rm c}-1)\lvert\psi_{\rm GL}\rvert^2+\tfrac12\beta_{\rm GL}\lvert\psi_{\rm GL}\rvert^4$ (the example of Section III.L.2 with the gradient coefficient $\gamma_{\rm GL}$, the slope $\alpha'_{\rm GL}$ and the critical temperature $T_{\rm c}$), the Ginzburg number is $T_{\rm c}^2\beta_{\rm GL}^2/(\alpha'_{\rm GL}\gamma_{\rm GL}^3)$ up to a numerical factor that depends on the conventions. Within the critical region the order parameter vanishes with an exponent different from $\tfrac12$ (in three dimensions near 0.33 to 0.37 for the Ising, XY and Heisenberg universality classes), the critical temperature is in general lower than the mean-field one, and for an order parameter with two quartic invariants, as here, the fluctuations can also make the transition discontinuous (the renormalization-group theory of such fields is reviewed by A. Pelissetto and E. Vicari, 2002). A gauge field coupled to the order parameter can do the same: its fluctuations make the transition of a superconductor weakly first order (B. I. Halperin, T. C. Lubensky and S.-k. Ma, 1974), and the thermal transition of the Higgs field of the electroweak theory, of first order for small Higgs masses, ends at a critical Higgs mass, above which there is no transition but a smooth crossover (lattice simulations of K. Kajantie, M. Laine, K. Rummukainen and M. Shaposhnikov, 1996). The answer of Section III.T.6 therefore describes the superfluid near $T^{X,{\rm c}}_{\rm SF}$ only outside the critical region, and whether the transition of the full theory is continuous is not decided here. The size of the critical region needs the gradient coefficient of the field at $T^{X,{\rm c}}_{\rm SF}$, which is not computed yet (the solution computes the coefficients of the gradient terms at $T=0$ only); computing it is the next step if the critical region is to be located. At $T=0$ the methods omit the quantum fluctuations of the order parameter as well; that is a property of the methods, not the subject of this remark.*)
