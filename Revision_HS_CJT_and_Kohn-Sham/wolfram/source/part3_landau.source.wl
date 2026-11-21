(* ::Package:: *)

(* ::Chapter:: *)
(*Part III, first section. The Landau form of the effective potentials: what it is, how accurate it is, and why that matters*)

(* ::Text:: *)
(*Part III of the solution builds the effective Lagrangians $\mathcal L^{\rm CJT}$, $\mathcal L^{\rm HS}$, $\mathcal L^{\rm KS}$ of the field of the superfluid boson singlet state. Their potential parts were summarized by a polynomial, the Landau form, with the remark: "The Landau form is a fit; its largest residual is 0.27-0.7 % of the depth". The author asked (21 November 2026) to explain this remark clearly and completely and why it is important, to define the Landau form with examples, and to keep the answer in the documents and in this notebook. This chapter is that answer. It is the first section of Part III in the notebook; the other sections of Part III, and Part II, whose engine computes the curves used here, follow in the next versions. Until then the computed curves are taken from the Rust program `sf_part3` (the closed cell of Section III.L.3, with the SHA-256 checksums of its data files), and this chapter recomputes from them, by its own least squares, every number of the answer.*)

(* ::Section:: *)
(*III.L.1 The objects*)

(* ::Text:: *)
(***The field and its effective potential.** The field of the superfluid boson singlet state is the complex isospin vector $\vec\Phi^X_{\rm SF}=(\Phi^{X,1}_{\rm SF},\Phi^{X,2}_{\rm SF},\Phi^{X,3}_{\rm SF})$ of the amplitudes of the pair self-energy of a uniform state, $\mathsf P_{\rm SF}=\sum_\iota\Phi^{X,\iota}_{\rm SF}(\mathsf K^\iota_{\rm LL})^\dagger$, where $\mathsf K^\iota_{\rm LL}$ are the three pair matrices of Part I (`kLL[iota]` in the code) and $X$ is one of the five computed methods: HS $(1,1)$, HS $(0,1)$, HS $(0,0)$ (the method of Hubbard and Stratonovich with the shares $s^{\rm HS,S}_{\rm SF}$, $s^{\rm HS,V}_{\rm SF}$ of the scalar and the vector term decoupled in the density channels), KS (Kohn and Sham) and CJT (Cornwall, Jackiw and Tomboulis). The effective potential $\mathcal V^X_{\rm SF}(\vec\Phi)$ is the smallest energy per volume of method $X$ among the uniform states whose field is $\vec\Phi$, measured from the normal vacuum, so that $\mathcal V^X_{\rm SF}(0)=0$.*)

(* ::Text:: *)
(***How it is computed.** A fixed pair self-energy $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$, the applied field of amplitude $\Phi_{\rm a}$, is put into the matrix of Bogoliubov and de Gennes, the normal self-energy is made self-consistent, and the ground state gives one point: its own field $\vec\Phi=\Phi\vec e_3$ and its energy per volume. The Rust program computes 61 points per method, with $\Phi_{\rm a}$ from $0$ to $2\Phi^X_{0,{\rm SF}}$, each self-consistent to $10^{-13}$. Along the **polar** fields $\vec\Phi=e^{i\vartheta}\Phi\vec n$ (real phase $\vartheta$, real unit vector $\vec n$) the potential depends on $\Phi=\lvert\vec\Phi\rvert$ alone. Its minimum lies at $\Phi^X_{0,{\rm SF}}$ (the vacuum), and the **depth** $\lvert\mathcal V^X_{\rm SF}(\Phi^X_{0,{\rm SF}})\rvert$ is the energy per volume by which the superfluid lies below the normal vacuum. The second derivative $\mathcal V''_{\rm amp}$ of the polar curve at its minimum fixes the mass of the amplitude mode, $(M^{X,{\rm H}}_{\rm SF})^2=\mathcal V''_{\rm amp}/(2(Z^X_{\rm SF}+Y^X_{\rm SF}))$, where $Z^X_{\rm SF}$ and $Y^X_{\rm SF}$ are the two coefficients of the kinetic part of the effective Lagrangian.*)

(* ::Section:: *)
(*III.L.2 The Landau form: definition and examples*)

(* ::Text:: *)
(***Definition.** A Landau form of a potential is its expansion in powers of the order parameter (the field that vanishes in the symmetric state and not in the broken one) under two rules: only polynomial invariants of the symmetry group occur, and the expansion stops after the lowest orders that bound the potential from below and produce its minima. L. D. Landau introduced it for phase transitions (1937): near a continuous transition the order parameter is small, the free energy is assumed to be analytic in it, and the form is its Taylor expansion, whose coefficients (the Landau coefficients) are derivatives at the symmetric state. The quadratic coefficient changes its sign at the transition; the signs of the higher coefficients decide the order of the transition and which broken state is the minimum. Examples:*)

(* ::Item:: *)
(*A uniaxial ferromagnet: $F_{\rm mag}=a_{\rm mag}m_{\rm mag}^2+b_{\rm mag}m_{\rm mag}^4$ for the magnetization $m_{\rm mag}$ (the symmetry $m_{\rm mag}\to-m_{\rm mag}$), with $a_{\rm mag}$ proportional to the temperature minus the Curie temperature and $b_{\rm mag}>0$; below the Curie temperature the minima are $m_{\rm mag}=\pm(-a_{\rm mag}/(2b_{\rm mag}))^{1/2}$.*)

(* ::Item:: *)
(*The theory of superconductivity of V. L. Ginzburg and L. D. Landau (1950): $\alpha_{\rm GL}\lvert\psi_{\rm GL}\rvert^2+\tfrac12\beta_{\rm GL}\lvert\psi_{\rm GL}\rvert^4$ for the complex pair wave function $\psi_{\rm GL}$ (the symmetry $U(1)$), with gradient terms and the magnetic field added. L. P. Gor'kov (1959) derived it from the theory of Bardeen, Cooper and Schrieffer near the critical temperature, where $\lvert\psi_{\rm GL}\rvert$ is small; there the coefficients are true Taylor coefficients.*)

(* ::Item:: *)
(*The Higgs potential of the Standard Model, $-(\mu^{\rm SM})^2(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}+\lambda^{\rm SM}((\Phi^{\rm SM})^\dagger\Phi^{\rm SM})^2$: the doublet $\Phi^{\rm SM}$ has the single invariant $(\Phi^{\rm SM})^\dagger\Phi^{\rm SM}$ of degree 2, and this Landau form is exact at tree level, not a truncation.*)

(* ::Item:: *)
(*The Bose-Einstein condensates of spin 1 (T.-L. Ho, 1998; T. Ohmi and K. Machida, 1998), the closest analogue of $\vec\Phi$: their order parameter is a complex vector $\vec\varpi$, and their energy per volume $\tfrac12c_{\rm n}\lvert\vec\varpi\rvert^4+\tfrac12c_{\rm s}(\lvert\vec\varpi\rvert^4-\lvert\vec\varpi\cdot\vec\varpi\rvert^2)$ contains the same two quartic invariants as $\vec\Phi$, the second term being the square of the spin. $c_{\rm s}>0$ (sodium-23) favors the polar state, $c_{\rm s}<0$ (rubidium-87) the ferromagnetic state $\vec\varpi\propto(1,i,0)$.*)

(* ::Item:: *)
(*The ferroelectric barium titanate (A. F. Devonshire, 1949): $\alpha_{\rm D}P_{\rm D}^2+\beta_{\rm D}P_{\rm D}^4+\gamma_{\rm D}P_{\rm D}^6$ for the polarization $P_{\rm D}$, with $\beta_{\rm D}<0$ and $\gamma_{\rm D}>0$: the sixth power is required, and the transition is of first order.*)

(* ::Item:: *)
(*Superfluid helium-3 (A. J. Leggett, 1975): one quadratic and five quartic invariants of a complex $3\times3$ matrix, whose five coefficients decide between the phases A and B. The nematic liquid crystals (P. G. de Gennes, 1971): the traceless symmetric tensor of the nematic order admits an invariant of the third degree, so the transition is of first order.*)

(* ::Text:: *)
(*The name has nothing to do with the Landau levels of a charge in a magnetic field (Section I.6 of this notebook, AS2): the same physicist, another subject. **The Landau form of $\vec\Phi$.** The symmetry is $SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}$. The independent invariants up to the fourth degree are $\lvert\vec\Phi\rvert^2$, $\lvert\vec\Phi\rvert^4$ and $\lvert\vec\Phi\cdot\vec\Phi\rvert^2$, and $\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2=\lvert\vec\Phi\times\vec\Phi^*\rvert^2$ vanishes exactly on the polar fields. The Landau form used by the solution is*)

(* ::DisplayFormula:: *)
(*\mathcal V^X_{\rm SF}\approx a^X_{\rm SF}\lvert\vec\Phi\rvert^2+b^X_{\rm SF}\lvert\vec\Phi\rvert^4+c^X_{\rm SF}\lvert\vec\Phi\rvert^6+u^X_{\rm SF}\big(\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2\big),*)

(* ::Text:: *)
(*the polynomial $\mathcal V_{\rm L}(\Phi)=a^X_{\rm SF}\Phi^2+b^X_{\rm SF}\Phi^4+c^X_{\rm SF}\Phi^6$ along the polar fields. The coefficient $u^X_{\rm SF}$ is not fitted: it comes from the computed second derivative in a non-unitary direction at the minimum, and it is positive in every method (the sodium case of the condensates of spin 1). The next cell checks, for random vectors, that $\lvert\vec\Phi\rvert^4-\lvert\vec\Phi\cdot\vec\Phi\rvert^2=\lvert\vec\Phi\times\vec\Phi^*\rvert^2$, and that it vanishes on polar fields.*)

(* ::Input:: *)
(* the identity |Phi|^4 - |Phi.Phi|^2 = |Phi x Phi^*|^2 for complex 3-vectors, exactly, with symbolic components *)
lfVec = Array[lfRe[#] + I lfIm[#] &, 3];   (* Phi = lfRe + i lfIm with real components lfRe[k], lfIm[k] *)
lfNorm2 = ComplexExpand[Conjugate[lfVec] . lfVec];
lfIdentity = Simplify[ComplexExpand[lfNorm2^2 - Abs[lfVec . lfVec]^2 - Conjugate[Cross[lfVec, Conjugate[lfVec]]] . Cross[lfVec, Conjugate[lfVec]]]];
(* on a polar field Phi = e^(i theta) r n with a real unit vector n the term vanishes *)
lfPolar = Simplify[ComplexExpand[With[{v = Exp[I lfTh] lfR {Sin[lfT1] Cos[lfT2], Sin[lfT1] Sin[lfT2], Cos[lfT1]}},
      (Conjugate[v] . v)^2 - Abs[v . v]^2]], Assumptions -> Element[{lfTh, lfR, lfT1, lfT2}, Reals]];
Print["|Phi|^4 - |Phi.Phi|^2 - |Phi x Phi*|^2 = ", lfIdentity, ";  on a polar field |Phi|^4 - |Phi.Phi|^2 = ", lfPolar];

(* ::Section:: *)
(*III.L.3 The computed curves of the Rust program*)

(* ::Text:: *)
(*The closed cell holds the values of Part III of the Rust program `sf_part3` (`rustPart3`) and its five computed polar curves (`rustEffPot[tag]`: for each point the applied field $\Phi_{\rm a}$, the field $\Phi$ of the state, the energy per volume $\mathcal V^X_{\rm SF}$ and the Dirac mass of the state). They were copied from the data files named, with their SHA-256 checksums, in the comment at the top of the cell, by `wolfram/tools/make_data_cells.py`.*)

(* ::Input::Closed:: *)
(* @@INCLUDE data_part3.wl@@ *)
Print["values of the Rust program sf_part3: ", Length[rustPart3], "; points of the five polar curves: ", Length /@ (rustEffPot /@ {"HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"})];

(* ::Input:: *)
(* lfTags: the five methods (the tags of the data) and their printed names *)
lfTags = {"HS_1_1", "HS_0_1", "HS_0_0", "KS", "CJT"};
lfName = <|"HS_1_1" -> "HS (1, 1)", "HS_0_1" -> "HS (0, 1)", "HS_0_0" -> "HS (0, 0)", "KS" -> "KS", "CJT" -> "CJT"|>;
(* lfPhi0[t]: the vacuum field Phi^X_0,SF; lfE0[t]: the energy per volume of the minimum (minus the depth) *)
lfPhi0[t_] := rustPart3["p3_Phi0_" <> t];
lfE0[t_] := rustPart3["p3_E0_" <> t];
(* lfCurve[t]: the computed polar curve as pairs {Phi, V}; lfFitted[t, top]: its points with Phi <= top Phi0 *)
lfCurve[t_] := rustEffPot[t][[All, {2, 3}]];
lfFitted[t_, top_] := Select[lfCurve[t], #[[1]] <= top lfPhi0[t] (1 + 10^-12) &];
Grid[Prepend[Table[{lfName[t], lfPhi0[t], lfE0[t], Length[lfFitted[t, 1.4]], Max[lfCurve[t][[All, 1]]]/lfPhi0[t]}, {t, lfTags}],
   {"method", "Phi0", "minimum of V (minus the depth)", "points with Phi <= 1.4 Phi0", "end of the curve / Phi0"}], Frame -> All]

(* ::Section:: *)
(*III.L.4 The fit and its residual*)

(* ::Text:: *)
(***"The Landau form is a fit."** The coefficients $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$ are not derived from the theory: they minimize the sum of the squares of the residuals $\mathcal V_{\rm L}(\Phi_i)-\mathcal V^X_{\rm SF}(\Phi_i)$ over the computed points $\Phi_i\le1.4\Phi^X_{0,{\rm SF}}$. **"Its largest residual is 0.27-0.7 % of the depth."** The largest of these residuals, divided by the depth, lies between these two numbers for the five methods. The next cell defines the least squares (the function `LeastSquares` of the Wolfram Language on the columns $t^k$, $t=(\Phi/\Phi_{\max})^2$, which keeps the fits with many powers well conditioned) and computes, for each method: the minimum $\Phi_{\rm L}$ of the fitted polynomial, its value and its second derivative there; the largest residual of the fits $\sum_{k=1}^Kc_k\Phi^{2k}$ with $K=2$ (the quartic form of the Standard Model) to $K=6$ (powers to $\Phi^{12}$); the number of sign changes of the residual of the Landau form; and the Landau form fitted on three intervals, $[0,\Phi^X_{0,{\rm SF}}]$, $[0,1.4\Phi^X_{0,{\rm SF}}]$ and the whole curve. Every number is compared with that of the Rust program, which computes it with Householder reflections; the Python program computes it a third time with the singular-value decomposition.*)

(* ::Input:: *)
(* evenFit[pts, kk]: the least-squares fit V = sum_{k=1..kk} c_k Phi^(2k) to the points {Phi, V}; returns the
   coefficients {c_1, ..., c_kk}, the largest residual and the list of the residuals (fit minus computed value) *)
evenFit[pts_, kk_] := Module[{x = pts[[All, 1]], y = pts[[All, 2]], xm, a, d, c, r},
   xm = Max[x];
   a = Table[(xi/xm)^(2 k), {xi, x}, {k, kk}];        (* the columns t^k with t = (Phi/Phi_max)^2 *)
   d = LeastSquares[a, y];
   c = d/xm^(2 Range[kk]);                            (* back to the coefficients of Phi^(2k) *)
   r = Table[Sum[c[[k]] xi^(2 k), {k, kk}], {xi, x}] - y;
   {c, Max[Abs[r]], r}];
(* landauMin[{a, b, c}]: the minimum Phi_L > 0 of a Phi^2 + b Phi^4 + c Phi^6 (the positive root s = Phi^2 of
   a + 2 b s + 3 c s^2 = 0), the value of the polynomial there and its second derivative 2 a + 12 b Phi^2 + 30 c Phi^4 *)
landauMin[{a_, b_, c_}] := Module[{s = (-2 b + Sqrt[4 b^2 - 12 a c])/(6 c)}, {Sqrt[s], a s + b s^2 + c s^3, 2 a + 12 b s + 30 c s^2}];
(* lfResults[t]: all numbers of the accuracy of the Landau form for the method t *)
lfResults = Association[Map[Function[t, Module[{pts = lfFitted[t, 1.4], fits, mn, r6, big, signs, top3, rng},
      fits = Table[evenFit[pts, kk], {kk, 2, 6}];        (* K = 2, ..., 6; K = 3 is the Landau form *)
      mn = landauMin[fits[[2, 1]]];
      r6 = fits[[2, 3]];
      big = Select[r6, Abs[#] > 10^-3 Max[Abs[r6]] &];   (* residuals below 1e-3 of the largest are not counted *)
      signs = Count[Differences[Sign[big]], d_ /; d != 0];
      top3 = Max[lfCurve[t][[All, 1]]]/lfPhi0[t];
      rng = Table[Module[{f = evenFit[lfFitted[t, top], 3]}, Join[{top}, f[[1]], {f[[2]]}]], {top, {1.0, 1.4, top3}}];
      t -> Join[<|"a" -> fits[[2, 1, 1]], "b" -> fits[[2, 1, 2]], "c" -> fits[[2, 1, 3]],
         "phifit" -> mn[[1]], "depthfit" -> mn[[2]], "curvfit" -> mn[[3]], "signs" -> signs,
         "resK2" -> fits[[1, 2]], "resK3" -> fits[[2, 2]], "resK4" -> fits[[3, 2]], "resK5" -> fits[[4, 2]], "resK6" -> fits[[5, 2]],
         "cK4" -> fits[[3, 1, 3]],
         "residuals" -> Transpose[{pts[[All, 1]]/lfPhi0[t], fits[[1, 3]]/Abs[lfE0[t]], r6/Abs[lfE0[t]], fits[[3, 3]]/Abs[lfE0[t]]}]|>,
        Association[Flatten[Table[{"rng" <> ToString[i] <> "_top" -> rng[[i, 1]], "rng" <> ToString[i] <> "_a" -> rng[[i, 2]],
            "rng" <> ToString[i] <> "_b" -> rng[[i, 3]], "rng" <> ToString[i] <> "_c" -> rng[[i, 4]], "rng" <> ToString[i] <> "_res" -> rng[[i, 5]]}, {i, 3}]]]]]],
    lfTags]];
(* the comparison with the Rust program: the largest relative difference of all numbers, plus the difference of the
   counts of the sign changes *)
lfKeys = {"a", "b", "c", "phifit", "depthfit", "curvfit", "resK2", "resK3", "resK4", "resK5", "resK6", "cK4",
   "rng1_top", "rng1_a", "rng1_b", "rng1_c", "rng1_res", "rng2_top", "rng2_a", "rng2_b", "rng2_c", "rng2_res",
   "rng3_top", "rng3_a", "rng3_b", "rng3_c", "rng3_res"};
Do[With[{r = lfResults[t]},
   check["WP3-11 " <> t, lfName[t] <> ": the accuracy of the Landau form recomputed by LeastSquares (the coefficients, the minimum, depth and curvature of the polynomial, the residuals of the fits with Phi^2 .. Phi^(2K), K = 2 .. 6, the fits on three intervals, the sign changes) = Rust (largest relative difference)",
    Max[Map[Abs[r[#] - rustPart3["p3_landau_" <> # <> "_" <> t]]/Abs[rustPart3["p3_landau_" <> # <> "_" <> t]] &, lfKeys]] +
     Abs[r["signs"] - rustPart3["p3_landau_signs_" <> t]], 10^-7, "P3-11 " <> t, "YP3-11 " <> t]], {t, lfTags}];

(* ::Text:: *)
(*The next cell prints the numbers in the form of a table (in per cent where marked): what the polynomial gets wrong at its own minimum (the position, the depth, the curvature and the resulting mass of the amplitude mode), the largest residuals of the fits with more or fewer powers, the sign changes, and the coefficient $c^X_{\rm SF}$ fitted on the three intervals. The figure after it shows the residuals (top) and the decrease of the largest residual with the number of powers (bottom).*)

(* ::Input:: *)
(* the table of the accuracy of the Landau form; pct[x]: x in per cent with three significant digits *)
pct[x_] := NumberForm[100. x, 3];
lfRows = {
   {"minimum of the polynomial: Phi_L/Phi0 - 1 (per cent)", pct[lfResults[#]["phifit"]/lfPhi0[#] - 1] &},
   {"depth of the polynomial / depth - 1 (per cent)", pct[lfResults[#]["depthfit"]/lfE0[#] - 1] &},
   {"curvature at its minimum / V''amp - 1 (per cent)", pct[lfResults[#]["curvfit"]/rustPart3["p3_V2amp_" <> #] - 1] &},
   {"resulting M^H / M^H - 1 (per cent)", pct[Sqrt[lfResults[#]["curvfit"]/rustPart3["p3_V2amp_" <> #]] - 1] &},
   {"largest residual / depth, powers to Phi^4 (per cent)", pct[lfResults[#]["resK2"]/Abs[lfE0[#]]] &},
   {"the same, powers to Phi^6: the Landau form (per cent)", pct[lfResults[#]["resK3"]/Abs[lfE0[#]]] &},
   {"the same, powers to Phi^8 (per cent)", pct[lfResults[#]["resK4"]/Abs[lfE0[#]]] &},
   {"the same, powers to Phi^10 (per cent)", pct[lfResults[#]["resK5"]/Abs[lfE0[#]]] &},
   {"the same, powers to Phi^12 (per cent)", pct[lfResults[#]["resK6"]/Abs[lfE0[#]]] &},
   {"sign changes of the residual of the Landau form", lfResults[#]["signs"] &},
   {"10^3 c fitted on [0, Phi0]", NumberForm[1000 lfResults[#]["rng1_c"], 4] &},
   {"10^3 c fitted on [0, 1.4 Phi0]", NumberForm[1000 lfResults[#]["rng2_c"], 4] &},
   {"10^3 c fitted on the whole curve", NumberForm[1000 lfResults[#]["rng3_c"], 4] &},
   {"10^3 c of the fit with powers to Phi^8", NumberForm[1000 lfResults[#]["cK4"], 4] &}};
Grid[Prepend[Map[Function[row, Prepend[Map[row[[2]], lfTags], row[[1]]]], lfRows], Prepend[lfName /@ lfTags, "quantity"]],
  Frame -> All, Alignment -> Left, BaseStyle -> {FontSize -> 9}]

(* ::Input:: *)
(* the figure: top, the residual of the Landau form along the fitted interval relative to the depth (KS and CJT
   coincide along the polar fields, so one curve is drawn for both); bottom, the largest residual of the fits with
   the powers Phi^2 .. Phi^(2K) against 2K (logarithmic scale); one panel above the other, so that both fit the
   width of a printed page *)
lfColors = {RGBColor["#2a78d6"], RGBColor["#eb6834"], RGBColor["#1baf7a"], RGBColor["#eda100"]};
lfShown = {"HS_1_1", "HS_0_1", "HS_0_0", "CJT"};
lfLegend = {"HS (1, 1)", "HS (0, 1)", "HS (0, 0)", "CJT = KS"};
Column[{
  ListLinePlot[Table[With[{r = lfResults[t]["residuals"]}, Transpose[{r[[All, 1]], 100 r[[All, 3]]}]], {t, lfShown}],
   PlotStyle -> Table[Directive[lfColors[[i]], AbsoluteThickness[1.4], AbsoluteDashing[{{}, {6, 3}, {6, 2, 1, 2}, {1, 2}}[[i]]]], {i, 4}],
   PlotMarkers -> {Automatic, 4}, Frame -> True, FrameLabel -> {"\[CapitalPhi] / \[CapitalPhi]0 (polar fields)", "residual of the Landau form / depth (per cent)"},
   PlotLegends -> Placed[lfLegend, {Left, Bottom}], GridLines -> {{1.0}, {0}}, ImageSize -> 460,
   PlotLabel -> "the fit oscillates about the computed curve"],
  ListLogPlot[Table[Table[{2 k, 100 lfResults[t]["resK" <> ToString[k]]/Abs[lfE0[t]]}, {k, 2, 6}], {t, lfShown}],
   Joined -> True, PlotStyle -> Table[Directive[lfColors[[i]], AbsoluteThickness[1.2]], {i, 4}], PlotMarkers -> {Automatic, 7},
   Frame -> True, FrameLabel -> {"highest power 2K of \[CapitalPhi] in the fit", "largest residual / depth (per cent)"},
   PlotLegends -> Placed[lfLegend, {Left, Bottom}], GridLines -> {{6}, None}, ImageSize -> 460,
   PlotLabel -> "more powers, smaller residual: a truncation"]}, Spacings -> 1.5]

(* ::Section:: *)
(*III.L.5 Why no polynomial can be exact: the cause*)

(* ::Text:: *)
(***The pair field enters the quasi-particle energies like a mass.** For the applied pair self-energy $\Phi_{\rm a}(\mathsf K^3_{\rm LL})^\dagger$ and a normal self-energy that is a Dirac mass, $(M^{X,{\rm D}}_{\rm SF}-m_{\rm LL})\beta_{\rm LL}\otimes\mathsf 1_2$, the eigenvalues of the matrix of Bogoliubov and de Gennes at the momentum $\vec q_{\rm LL}$ are $\pm E$, each eight times, with*)

(* ::DisplayFormula:: *)
(*E=\big(\lvert\vec q_{\rm LL}\rvert^2+(M^{X,{\rm D}}_{\rm SF})^2+\Phi_{\rm a}^2\big)^{1/2}.*)

(* ::Text:: *)
(*The next cell checks this for 20 random momenta, masses and fields with the matrices of Part I (`hLL`, `kLL`, `betaLL`); along the whole polar curve the normal self-energy is such a Dirac mass (the Rust check P3-9 and the Python check YP3-9).*)

(* ::Input:: *)
(* the BdG matrix of the eight components with the Dirac mass M (normal self-energy (M - m) beta (x) 1_2) and the applied
   pair self-energy fa (K^3)^dag, at the momentum q; its sorted eigenvalues against -E (eight times) and +E (eight times) *)
lfBdG[q_, mass_, fa_] := Module[{n = (mass - mLL) d8[betaLL], p = fa ConjugateTranspose[kLL[3]], a, d},
   a = hLL[q, mLL] + n; d = hLL[-q, mLL] + n;
   N[ArrayFlatten[{{a, p}, {ConjugateTranspose[p], -Transpose[d]}}]]];
SeedRandom[20261121];
lfQpDev = Max[Table[Module[{q = RandomReal[{-1, 1}, 3], mass = RandomReal[{0.1, 0.5}], fa = RandomReal[{0, 0.6}], ev, e},
     ev = Sort[Re[Eigenvalues[lfBdG[q, mass, fa]]]];
     e = Sqrt[q . q + mass^2 + fa^2];
     Max[Abs[ev - Join[ConstantArray[-e, 8], ConstantArray[e, 8]]]]], {20}]];
check["WP3-8", "the pair field enters the quasi-particle energies like a mass: the BdG matrix with N = (M - m) beta (x) 1_2 and P = Phi_a (K^3)^dag has the eigenvalues -+(q^2 + M^2 + Phi_a^2)^(1/2), eight times each (20 random q, M, Phi_a)", lfQpDev, 10^-13, "P3-8", "YP3-8"];

(* ::Text:: *)
(***A logarithm.** The energy per volume is an integral over the momenta $q<\Lambda_{\rm LL}$ of functions of $(q^2+s)^{1/2}$ with $s=(M^{X,{\rm D}}_{\rm SF})^2+\Phi_{\rm a}^2$. The next cell integrates the simplest of them exactly and expands the result about $s=0$: it contains the term $\tfrac1{16}s^2\ln s$, which is not analytic at $s=0$, that is, at $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$. Hence $\mathcal V^X_{\rm SF}$ is not a polynomial in $\lvert\vec\Phi\rvert^2$, and a series of powers of $\lvert\vec\Phi\rvert^2$ about the origin can converge only within a radius of the order of the Dirac mass ($m_{\rm LL}=0.2$ at the origin), while $\Phi^X_{0,{\rm SF}}=0.31$ to $0.40$: the superfluid is far from the regime of Landau's theory, where the order parameter is small and the Landau coefficients are Taylor coefficients.*)

(* ::Input:: *)
(* the integral of q^2 (q^2 + s)^(1/2) from 0 to the cutoff lam, exactly, and its expansion about s = 0 *)
(* lfq: the modulus of the momentum, lfs: the quantity M^2 + Phi_a^2, lfLam: the cutoff *)
lfInt = Integrate[lfq^2 Sqrt[lfq^2 + lfs], {lfq, 0, lfLam}, Assumptions -> lfs > 0 && lfLam > 0];
lfClosed = lfLam (2 lfLam^2 + lfs) Sqrt[lfLam^2 + lfs]/8 - lfs^2/8 Log[(lfLam + Sqrt[lfLam^2 + lfs])/Sqrt[lfs]];
Print["the integral minus the closed form: ", FullSimplify[lfInt - lfClosed, Assumptions -> lfs > 0 && lfLam > 0]];
lfSeries = Expand[Normal[Series[lfClosed /. lfLam -> 1, {lfs, 0, 2}]]];
Print["the closed form about s = 0 (cutoff 1), to the order s^2: ", lfSeries];
Print["its term with the logarithm: ", Coefficient[lfSeries, Log[lfs]], " Log[s]"];

(* ::Text:: *)
(***The Taylor coefficient at the origin.** The coefficient $a^{X,{\rm Tay}}_{\rm SF}$ of $\lvert\vec\Phi\rvert^2$ of the Taylor series at the origin is the limit of $\mathcal V^X_{\rm SF}/\Phi^2$ at $\Phi\to0$. The Rust and the Python program compute it from three states with small applied fields (0.004, 0.008, 0.016) and the Richardson elimination of the term proportional to $\Phi^2$. The next cell estimates it from the first six points of the computed curve ($\Phi<0.1$, well inside the radius of convergence) by fitting $\mathcal V^X_{\rm SF}/\Phi^2=a_0+a_1\Phi^2+a_2\Phi^4$; the truncation of this local series limits the agreement to about $3\times10^{-7}$. The fitted $a^X_{\rm SF}$ of the Landau form differs from $a^{X,{\rm Tay}}_{\rm SF}$ by 0.7 to 2 per cent: it is a property of the fit.*)

(* ::Input:: *)
(* lfTaylor[t]: a^Tay estimated from the points 2 to 7 of the curve (the first point is Phi = 0) *)
lfTaylor[t_] := Module[{p = lfCurve[t][[2 ;; 7]]}, LeastSquares[Table[{1, x^2, x^4}, {x, p[[All, 1]]}], p[[All, 2]]/p[[All, 1]]^2][[1]]];
Do[check["WP3-10 " <> t, lfName[t] <> ": the coefficient of Phi^2 of the Taylor series at the origin from the first six points of the curve = the Rust value a^Tay = " <> ToString[rustPart3["p3_a_taylor_" <> t]] <> " (relative)",
   Abs[lfTaylor[t] - rustPart3["p3_a_taylor_" <> t]]/Abs[rustPart3["p3_a_taylor_" <> t]], 10^-6, "P3-10 " <> t, "YP3-10 " <> t], {t, {"HS_1_1", "CJT"}}];
Grid[Prepend[Table[{lfName[t], NumberForm[lfTaylor[t], 10], NumberForm[rustPart3["p3_a_taylor_" <> t], 10], NumberForm[lfResults[t]["a"], 10], NumberForm[100 (lfResults[t]["a"]/rustPart3["p3_a_taylor_" <> t] - 1), 3]}, {t, lfTags}],
   {"method", "a^Tay from the curve", "a^Tay of the Rust program", "a of the Landau form", "a/a^Tay - 1 (per cent)"}], Frame -> All]

(* ::Section:: *)
(*III.L.6 Why it matters*)

(* ::ItemNumbered:: *)
(***It enters no other result.** The effective Lagrangians of the solution contain the computed function $\mathcal V^X_{\rm SF}$, not the Landau form, and no other result uses $a^X_{\rm SF}$, $b^X_{\rm SF}$, $c^X_{\rm SF}$: the mass of the amplitude mode uses the curvature $\mathcal V''_{\rm amp}$ of the computed curve, $u^X_{\rm SF}$ and the mass of the non-unitary modes use the computed curvature in the non-unitary direction, and the masses of the gauge bosons and the dynamics use $\Phi^X_{0,{\rm SF}}$, $Z^X_{\rm SF}$, $Y^X_{\rm SF}$ and the susceptibilities.*)

(* ::ItemNumbered:: *)
(***It matters to whoever replaces $\mathcal V^X_{\rm SF}$ by the Landau form**, as in a model of the type of Ginzburg and Landau or of Higgs: the minimum of the polynomial is 0.04 to 0.11 per cent too small, its depth 0.12 to 0.31 per cent too large, its curvature at its minimum 0.86 to 2.3 per cent too large, and the mass of the amplitude mode 0.43 to 1.1 per cent too large (the table above). A derivative enlarges the relative error of the function. Beyond the fitted interval the error grows: fitted on the whole computed curve, the form misses by 1.2 to 1.7 per cent of the depth.*)

(* ::ItemNumbered:: *)
(***The coefficients are not constants of the theory.** $a^X_{\rm SF}$ differs from the Taylor coefficient at the origin, and $b^X_{\rm SF}$, $c^X_{\rm SF}$ change with the fitted interval and with the number of powers (the last four rows of the table). No physical meaning may be given to $b^X_{\rm SF}$ or $c^X_{\rm SF}$ alone. The physical quantities are those that the computed function defines: $\Phi^X_{0,{\rm SF}}$, the depth, the curvatures at the minimum, and $a^{X,{\rm Tay}}_{\rm SF}$.*)

(* ::ItemNumbered:: *)
(***The qualitative results do not depend on the fit.** That $\mathcal V^X_{\rm SF}$ vanishes at the origin, is negative near it, has its minimum at $\Phi^X_{0,{\rm SF}}$, depends only on the two invariants, and has a stable polar minimum ($u^X_{\rm SF}>0$) are properties of the computed function, checked on it directly; the sign of $a^X_{\rm SF}$ is that of $a^{X,{\rm Tay}}_{\rm SF}$ (negative: the normal vacuum is unstable).*)

(* ::ItemNumbered:: *)
(***The comparison with the Standard Model.** The field has a Mexican hat with the symmetry structure of the Higgs sector and of the polar phase of a condensate of spin 1, but its potential is not the quartic potential of the Standard Model: the quartic form misses $\mathcal V^X_{\rm SF}$ by 2.8 to 5.9 per cent of the depth, about ten times more than the Landau form with the sixth power.*)

(* ::Text:: *)
(***Corrections of the first answers.** The first answers to the author contained five statements that were not exact; the numbers of this chapter are the corrected ones. (1) The residual changes its sign three times, not five. (2) The fit called a fit on $[0,2\Phi_0]$ used all 61 points of the curve, whose fields reach 1.52 to 1.64 times $\Phi^X_{0,{\rm SF}}$; the applied field, not the field of the states, reaches $2\Phi^X_{0,{\rm SF}}$. (3) The range of the end of the curve is 1.52 to 1.64 (HS $(0,0)$), not 1.52 to 1.61. (4) The singularity lies at $\Phi_{\rm a}^2=-(M^{X,{\rm D}}_{\rm SF})^2$ in the applied field; in the field of the states the radius of convergence is of the order of the Dirac mass, not equal to it. (5) The errors of the curvature quoted first, 1.1 to 2.7 per cent, are those of the polynomial at the minimum of the computed curve; at the polynomial's own minimum, where a user evaluates it, they are 0.86 to 2.3 per cent, as the table shows.*)
