(* ::Package:: *)

(* ::Chapter:: *)
(*Part II. The pairing, the three methods, the ground states and the gauge bosons in the superfluid*)

(* ::Text:: *)
(*Part II treats item 6 of the problem (Sections 8 to 11 of the manuscript): which interaction binds two spinor charges into a pair of spin zero and which gauge boson mediates the pairing; the methods of Hubbard and Stratonovich (HS), Kohn and Sham (KS) and Cornwall, Jackiw and Tomboulis (CJT); the recursion of the potential until all three methods admit a superfluid; the ground states, the first excited states and the gap; and the gauge bosons in the superfluid. The checks of this part are those of the Python program `sf_part2.py`; like that program, the notebook compares its own results with the values of the Rust program `sf_part2`, which the next (closed) cell holds: they were copied from the data files of the Rust program by `wolfram/tools/make_data_cells.py`, whose comment at the top of the cell names the files and their SHA-256 checksums. The notebook reads no file.*)

(* ::Input::Closed:: *)
(* @@INCLUDE data_part2.wl@@ *)
Print["values of the Rust program sf_part2: ", Length[rustPart2], "; rows of its table of pair channels: ", Length[rustPairChannels]];

(* ::Section:: *)
(*II.1 The pair channels and the gauge bosons that mediate the pairing*)

(* ::Text:: *)
(*The spin-zero pairs are the six antisymmetric unitary matrices $\mathsf K^\iota_{\rm LL}$ (scalar, $0^+$) and $\mathsf K'^\iota_{\rm LL}=C_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ (pseudoscalar, $0^-$). The state of two spinor charges at one point in the channel $\mathsf K$ is $\lvert\mathsf K\rangle\propto\tfrac12\sum_{cc'}\mathsf K^*_{cc'}\Psi^\dagger_{{\rm LL},c}\Psi^\dagger_{{\rm LL},c'}\lvert0\rangle$, normalized, and the pair energy of a contact operator $\mathcal O$ is $E^{\rm pair}_{\rm SF}[\mathcal O;\mathsf K]=\langle\mathsf K\rvert\mathcal O\lvert\mathsf K\rangle$, per unit coupling; negative means attraction. The operators are the three terms of the potentials and the contact limit of the exchange of the four gauge bosons of GEE, $\tfrac12\sum_p\eta^{\rm LL}_{pp}:(\Psi^\dagger\mathsf A^{\rm b}_{{\rm SF},p}\Psi)^2:$ with the vertex matrices $\mathsf A^{\rm b}_{{\rm SF},p}=(\gamma^0_{\rm LL}\gamma^p_{\rm LL})\otimes\mathsf t^{\rm b}_{\rm SF}$ ($\mathsf t^{\rm b}_{\rm SF}=\tfrac12\tau^{\rm LL}_\iota$ for $W^\iota_{\rm SF}$ and $\mathsf 1_2$ for $B_{\rm SF}$). The result: the scalar term has $+1$ in the $0^+$ channels and $-1$ in the $0^-$ channels, the vector term $+4$ everywhere, the pairing term $-4$ in the $0^+$ channels, each $W^\iota_{\rm SF}$ attracts ($-1$) in the two channels with the same index and repels ($+1$) in the other four, and $B_{\rm SF}$ repels ($+4$) everywhere. In every channel*)

(* ::DisplayFormula:: *)
(*\sum_{\iota=1}^3E^{\rm pair}_{\rm SF}[W^\iota;\mathsf K]=\tfrac14E^{\rm pair}_{\rm SF}[B;\mathsf K]=+1,*)

(* ::Text:: *)
(*the isospin Casimir of a triplet pair ($\vec T_1\cdot\vec T_2=+\tfrac14$): no gauge boson attracts a spin-zero pair in the symmetric vacuum. After the breaking of the symmetry in the channel $\mathsf K^3_{\rm LL}$, the gauge field $W^3_{\rm SF}$ of the unbroken generator stays massless (the zeugon, the massless gauge boson of the superfluid; not the photon, since the DiracLike spinor has electric charge 0) and attracts the two charges of the condensed pair, which carry the opposite charges $T^3=\pm\tfrac12$. The next cell computes the 42 pair energies exactly on the 256 states of one point (with the operators of Section I.5) and compares them with the table of the Rust program.*)

(* ::Input:: *)
(* kPs[iota]: the matrices K'^iota_LL = C_LL (x) E_LL tau_iota of the pseudoscalar spin-zero pairs *)
kPs[iota_Integer] := KroneckerProduct[cLL, eLL . sig[[iota + 1]]];
(* pairState[k]: the normalized state of two charges at one point in the channel k: sum_ab conj(k_ab) c_a^dag c_b^dag |0>;
   the empty state |0> is the first basis state *)
pairState[k_] := Module[{op, w},
   op = Sum[If[k[[a, b]] =!= 0, Conjugate[k[[a, b]]] cdOps[[a]] . cdOps[[b]], 0], {a, 8}, {b, 8}];
   w = op . UnitVector[256, 1];
   w/Norm[w]];
(* the seven contact operators: the three terms of the potentials and the contact limit of the four gauge bosons *)
channelOps = <|"scalar" -> 1/2 quartic[beta8, beta8],
   "vector" -> 1/2 (quartic[IdentityMatrix[8], IdentityMatrix[8]] - Sum[quartic[alpha8[[j]], alpha8[[j]]], {j, 3}]),
   "pairing" -> -1/4 Sum[ConjugateTranspose[piOps[[k]]] . piOps[[k]], {k, 3}]|>;
Do[With[{t = {sig[[2]]/2, sig[[3]]/2, sig[[4]]/2, IdentityMatrix[2]}[[b]], name = {"boson W1", "boson W2", "boson W3", "boson B"}[[b]]},
   channelOps[name] = Sum[1/2 etaLL[[p + 1, p + 1]] quartic[d8[gam[0] . gam[p]] . i8[t], d8[gam[0] . gam[p]] . i8[t]], {p, 0, 3}]], {b, 4}];
(* the six channels and their states *)
channelMats = Join[Table["K^" <> ToString[i] -> kLL[i], {i, 3}], Table["K'^" <> ToString[i] -> kPs[i], {i, 3}]];
channelStates = Association[Map[#[[1]] -> pairState[#[[2]]] &, channelMats]];
pairEnergy = Association[Flatten[Table[{o, c} -> Re[Conjugate[channelStates[c]] . channelOps[o] . channelStates[c]],
     {o, Keys[channelOps]}, {c, Keys[channelStates]}]]];
(* the table of the Rust program: the operator name (first word, or "boson W1" ...) and the channel (first token) *)
rustPairEnergy = Association[Map[{If[StringStartsQ[#[[1]], "boson"], StringRiffle[Take[StringSplit[#[[1]]], 2]], First[StringSplit[#[[1]]]]],
       First[StringSplit[#[[2]]]]} -> #[[3]] &, rustPairChannels]];
check["WP2-0a", "pair-channel energies of 7 operators in 6 channels (exact algebra of one point) = the table of the Rust program",
  Max[Table[Abs[pairEnergy[k] - rustPairEnergy[k]], {k, Keys[pairEnergy]}]], 10^-12, "P2-0e", "YP2-0a"];
check["WP2-0b", "the three W together = 1/4 of B in every channel",
  Max[Table[Abs[pairEnergy[{"boson W1", c}] + pairEnergy[{"boson W2", c}] + pairEnergy[{"boson W3", c}] - 1/4 pairEnergy[{"boson B", c}]],
    {c, Keys[channelStates]}]], 10^-12, "P2-0d", "YP2-0b"];
Grid[Prepend[Table[Prepend[Table[pairEnergy[{o, c}], {c, Keys[channelStates]}], o], {o, Keys[channelOps]}],
   Prepend[Keys[channelStates], "operator"]], Frame -> All, Alignment -> Right]

(* ::Section:: *)
(*II.2 The three methods and their numerical engine*)

(* ::Text:: *)
(*All three methods approximate the state of the spinor field by a uniform quasi-free state: at every momentum $\vec q_{\rm LL}$ the correlation matrices $\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})$ and $\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL})$, and their integrals over the momenta measured from the normal vacuum,*)

(* ::DisplayFormula:: *)
(*\delta\mathsf C^{\rm n}=\int\!\frac{d^3q}{(2\pi)^3}\big[\mathsf C^{\rm n}_{\rm LL}(\vec q_{\rm LL})-\mathsf C^{\rm vac}_{\rm LL}(\vec q_{\rm LL})\big],\qquad\mathsf C^{\rm p}=\int\!\frac{d^3q}{(2\pi)^3}\,\mathsf C^{\rm p}_{\rm LL}(\vec q_{\rm LL}),*)

(* ::Text:: *)
(*with $\lvert\vec q_{\rm LL}\rvert<\Lambda_{\rm LL}$. The energy per volume is the kinetic part plus the contractions of the contact terms: a bilinear term $\nu:(\Psi^\dagger\mathsf X_{\rm SF}\Psi)^2:$ gives $\nu[w^{\rm H}_{\rm SF}\,\mathrm{tr}(\mathsf X\delta\mathsf C^{\rm n})^2-w^{\rm F}_{\rm SF}\,\mathrm{tr}(\mathsf X\tilde{\mathsf C}\mathsf X\tilde{\mathsf C})-w^{\rm P}_{\rm SF}\,\mathrm{tr}((\mathsf C^{\rm p})^*\mathsf X\mathsf C^{\rm p}\mathsf X^T)]$, and a pairing term $-\tfrac14G_{\rm LL}:(\Pi^\iota)^\dagger\Pi^\iota:$ gives $-\tfrac14G_{\rm LL}[w^{\rm P}_{\rm SF}\lvert\mathrm{tr}(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm p})^T)\rvert^2+2w^{\rm F}_{\rm SF}\,\mathrm{tr}((\mathsf K^\iota_{\rm LL})^\dagger\tilde{\mathsf C}^T\mathsf K^\iota_{\rm LL}\tilde{\mathsf C})]$, where the weights $w^{\rm H}_{\rm SF}$, $w^{\rm F}_{\rm SF}$, $w^{\rm P}_{\rm SF}$ of the direct (Hartree), exchange (Fock) and pair (Bogoliubov) contractions define the method, and $\tilde{\mathsf C}=\delta\mathsf C^{\rm n}$ except in KS, where it is the projection $\mathcal P^{\rm KS}_{\rm SF}(\delta\mathsf C^{\rm n})=\tfrac18\mathrm{tr}(\delta\mathsf C^{\rm n})\mathsf 1_8+\tfrac18\mathrm{tr}((\beta_{\rm LL}\otimes\mathsf 1_2)\delta\mathsf C^{\rm n})\beta_{\rm LL}\otimes\mathsf 1_2$ onto the two Kohn-Sham densities. **HS** decouples the shares $s^{\rm HS,S}_{\rm SF}$ and $s^{\rm HS,V}_{\rm SF}$ of the scalar and the vector term in the density channels and the rest in the pair channels: weights $(s^{\rm HS,S}_{\rm SF},0,1-s^{\rm HS,S}_{\rm SF})$, $(s^{\rm HS,V}_{\rm SF},0,1-s^{\rm HS,V}_{\rm SF})$ and $(0,0,1)$ for the pairing term; three parameter sets are computed, $(1,1)$, $(0,1)$ and $(0,0)$. **KS** and **CJT** keep all three contractions with weight 1; KS with the projection. The self-energies are the derivatives of the energy,*)

(* ::DisplayFormula:: *)
(*\mathsf N_{\rm SF}=\nu\big[2w^{\rm H}_{\rm SF}\,\mathsf X\,\mathrm{tr}(\mathsf X\delta\mathsf C^{\rm n})-2w^{\rm F}_{\rm SF}\,\mathcal P(\mathsf X\tilde{\mathsf C}\mathsf X)\big],\qquad\mathsf P_{\rm SF}=-2\nu\,w^{\rm P}_{\rm SF}\,\mathsf X\mathsf C^{\rm p}\mathsf X^T*)

(* ::Text:: *)
(*for a bilinear term, and $\mathsf N_{\rm SF}=-G_{\rm LL}w^{\rm F}_{\rm SF}\mathcal P((\mathsf K^\iota)^\dagger\tilde{\mathsf C}^T\mathsf K^\iota)$, $\mathsf P_{\rm SF}=w^{\rm P}_{\rm SF}d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$ with $d^\iota_{\rm LL}=-\tfrac12G_{\rm LL}\mathrm{tr}(\mathsf K^\iota_{\rm LL}(\mathsf C^{\rm p})^T)$ for a pairing term. The state is the ground state of the matrix of Bogoliubov and de Gennes at every momentum,*)

(* ::DisplayFormula:: *)
(*\mathsf H_{\rm SF}(\vec q_{\rm LL})=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q_{\rm LL})+\mathsf N_{\rm SF}&\mathsf P_{\rm SF}\\\mathsf P_{\rm SF}^\dagger&-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})+\mathsf N_{\rm SF})^T\end{pmatrix},*)

(* ::Text:: *)
(*whose projector onto the negative eigenvalues gives $\mathsf C^{\rm n}_{\rm LL}$ and $-\mathsf C^{\rm p}_{\rm LL}$ as its upper blocks. **The grid:** 24 Gauss-Legendre nodes of $\lvert\vec q_{\rm LL}\rvert$ on $[0,\Lambda_{\rm LL}]$ times the six directions $\pm\vec e_j$, with the weight $w_{\rm r}q^2/(2\pi^2)/6$ of $\int d^3q/(2\pi)^3$. **The iteration:** the self-energies are mixed (new times 0.6 plus old times 0.4) until they change by less than $10^{-12}$. The gap is the minimum over $0\le\lvert\vec q_{\rm LL}\rvert\le\Lambda_{\rm LL}$ of the smallest eigenvalue modulus of $\mathsf H_{\rm SF}$ (401 momenta and a golden-section refinement), and the Dirac mass is $m_{\rm LL}+\tfrac18\mathrm{tr}((\beta_{\rm LL}\otimes\mathsf 1_2)\mathsf N_{\rm SF})$. The next cell defines this engine; the cell after it checks that the self-energies are the derivatives of the energy (central differences on random states) and that the engine with the pairing term alone reproduces the analytic solution AS3 of Part I.*)

(* ::Input:: *)
(* ---- the constants and matrices of the engine (floating point) ---- *)
lamLL = 1.;                                   (* the momentum cutoff Lambda_LL *)
{gvLL, gfLL} = {0.7, 0.45};                   (* the gauge couplings g^v_LL, g^F_LL of the project *)
id8 = N[IdentityMatrix[8]];
b8 = N[d8[betaLL]];                           (* beta_LL (x) 1_2 *)
k8[iota_] := N[kLL[iota]];                    (* K^iota_LL, numerical *)
zero8 = ConstantArray[0. I, {8, 8}];
(* gaussLegendre[n]: the nodes and weights of the n-point Gauss-Legendre rule on [-1, 1] (40 digits, then machine numbers) *)
gaussLegendre[n_Integer] := Module[{xx, poly, dpoly, roots},
   poly = LegendreP[n, xx]; dpoly = D[poly, xx];
   roots = Sort[xx /. NSolve[poly == 0, xx, WorkingPrecision -> 40]];
   {N[roots], N[Map[2/((1 - #^2) (dpoly /. xx -> #)^2) &, roots]]}];
(* gridSF[nr]: the radial nodes kr (on [0, Lambda]) and weights wr, the momenta q (nr x 6 directions) and the weights w of
   int d^3q/(2 pi)^3; the node of radial index i and direction d is the node 6 (i - 1) + d *)
gridSF[nr_Integer] := Module[{x, wgl, kr, wr, dirs},
   {x, wgl} = gaussLegendre[nr];
   kr = 0.5 lamLL (x + 1.); wr = 0.5 lamLL wgl;
   dirs = {{1, 0, 0}, {-1, 0, 0}, {0, 1, 0}, {0, -1, 0}, {0, 0, 1}, {0, 0, -1}};
   <|"nr" -> nr, "kr" -> kr, "wr" -> wr, "dirs" -> N[dirs],
     "q" -> Flatten[Table[kr[[i]] dirs[[d]], {i, nr}, {d, 6}], 1],
     "w" -> Flatten[Table[wr[[i]] kr[[i]]^2/(2 Pi^2)/6., {i, nr}, {d, 6}]], "n" -> 6 nr|>];
(* ksProj[x]: the projection P^KS_SF onto the matrices of the two Kohn-Sham densities *)
ksProj[x_] := Tr[x]/8 id8 + Tr[b8 . x]/8 b8;
(* terms[method, gs, gv, g]: the contact terms of the potential with the couplings (G^S, G^V, G) for a method
   {"HS", sS, sV}, {"KS"} or {"CJT"}: a list of {kind, data, {wH, wF, wP}, ks}; kind "b" with data {nu, X} for
   nu :(Psi^dag X Psi)^2:, kind "p" with data {G, K} for -(G/4) :Pi^dag Pi:; ks is True for the KS projection *)
terms[method_List, gs_, gv_, g_] := Module[{ws, wv, wq, ks, t = {}},
   If[method[[1]] === "HS",
     {ws, wv, wq, ks} = {{method[[2]], 0., 1. - method[[2]]}, {method[[3]], 0., 1. - method[[3]]}, {1., 0.}, False},
     {ws, wv, wq, ks} = {{1., 1., 1.}, {1., 1., 1.}, {1., 1.}, method[[1]] === "KS"}];
   If[gs != 0, AppendTo[t, {"b", {0.5 gs, b8}, ws, ks}]];
   If[gv != 0, AppendTo[t, {"b", {0.5 gv, id8}, wv, ks}];
     Do[AppendTo[t, {"b", {-0.5 gv, N[d8[alphaLL[j]]]}, wv, ks}], {j, 3}]];
   If[g != 0, Do[AppendTo[t, {"p", {g, k8[i]}, {0., wq[[2]], wq[[1]]}, ks}], {i, 3}]];
   t];
(* interaction[ts, rho, kap]: the energy and the self-energies {e, N, P} of the contact terms for the local matrices
   rho = delta C^n and kap = C^p (the formulas of the text) *)
interaction[ts_List, rho_, kap_] := Module[{e = 0., n = zero8, p = zero8},
   Do[Module[{kind = t[[1]], dat = t[[2]], wh = t[[3, 1]], wf = t[[3, 2]], wp = t[[3, 3]], ks = t[[4]], rf},
      rf = If[ks, ksProj[rho], rho];
      If[kind === "b",
        Module[{nu = dat[[1]], x = dat[[2]], tx, nf},
          tx = Tr[x . rho];
          e += nu (wh Re[tx tx] - wf Re[Tr[x . rf . x . rf]] - wp Re[Tr[Conjugate[kap] . x . kap . Transpose[x]]]);
          nf = 2 x . rf . x;
          n += nu (wh 2 x tx - wf If[ks, ksProj[nf], nf]);
          p += -nu wp 2 x . kap . Transpose[x]],
        Module[{gg = dat[[1]], k = dat[[2]], tt, nn},
          tt = Tr[k . Transpose[kap]];
          e += -0.25 gg (wp Abs[tt]^2 + 2 wf Re[Tr[ConjugateTranspose[k] . Transpose[rf] . k . rf]]);
          nn = -gg wf ConjugateTranspose[k] . Transpose[rf] . k;
          n += If[ks, ksProj[nn], nn];
          p += wp (-0.5 gg tt) ConjugateTranspose[k]]]], {t, ts}];
   {e, n, p}];
(* makeModel[ts, nr, mu]: the free matrices h(q) - mu and h(-q) - mu, the normal vacuum C^vac(q) = (1 - h/eps)/2 and the
   node of the opposite direction of every node *)
makeModel[ts_List, nr_Integer : 24, mu_ : 0.] := Module[{g = gridSF[nr], eps},
   eps = Sqrt[Total[g["q"]^2, {2}] + mLL^2];
   <|"ts" -> ts, "g" -> g,
     "hq" -> Map[N[hLL[#, mLL]] - mu id8 &, g["q"]], "hmq" -> Map[N[hLL[-#, mLL]] - mu id8 &, g["q"]],
     "vac" -> MapThread[0.5 (id8 - N[hLL[#1, mLL]]/#2) &, {g["q"], eps}],
     "opp" -> Table[6 Quotient[i, 6] + BitXor[Mod[i, 6], 1] + 1, {i, 0, g["n"] - 1}]|>];
(* bdg[mo, N, P, nex, pex]: the BdG matrices of all nodes; nex, pex are momentum-dependent self-energies (the exchange of
   the gauge bosons, Section II.5) or None *)
bdg[mo_, ns_, ps_, nex_ : None, pex_ : None] := Module[{a, d, p},
   a = Map[# + ns &, mo["hq"]]; d = Map[# + ns &, mo["hmq"]]; p = ConstantArray[ps, mo["g"]["n"]];
   If[nex =!= None, a = a + nex; d = d + nex[[mo["opp"]]]; p = p + pex];
   MapThread[ArrayFlatten[{{#1, #3}, {ConjugateTranspose[#3], -Transpose[#2]}}] &, {a, d, p}]];
(* stateOfNode[h]: the projector onto the negative eigenvalues of one BdG matrix: {C^n, C^p, the sorted eigenvalues} *)
stateOfNode[h_] := Module[{ev, vec, occ, r},
   (* the matrix is Hermitian up to rounding; it is made exactly Hermitian, so that Eigensystem uses the Hermitian
      solver, whose eigenvectors are orthonormal also within degenerate eigenvalues *)
   {ev, vec} = Eigensystem[0.5 (h + ConjugateTranspose[h])]; ev = Re[ev];
   occ = Pick[vec, Thread[ev < 0]];
   r = Transpose[occ] . Conjugate[occ];
   {r[[1 ;; 8, 1 ;; 8]], -r[[1 ;; 8, 9 ;; 16]], Sort[ev]}];
states[hs_] := Module[{res = Map[stateOfNode, hs]}, {res[[All, 1]], res[[All, 2]], res[[All, 3]]}];
(* localDens[mo, Cn, Cp]: rho = delta C^n, kap = C^p (integrals over the momenta) and the kinetic energy *)
localDens[mo_, cn_, cp_] := Module[{w = mo["g"]["w"], d = cn - mo["vac"]},
   {w . d, w . cp, Re[w . MapThread[Tr[#1 . #2] &, {mo["hq"], d}]]}];
(* solveSF[mo, N0, P0, mix, tol, maxit, ext]: the self-consistent uniform state from the initial self-energies; ext is
   a constant external normal self-energy (a chemical potential of a charge) or None *)
solveSF[mo_, n0_, p0_, mix_ : 0.6, tol_ : 10^-12, maxit_ : 40000, ext_ : None] := Module[
   {ns = n0, ps = p0, cn, cp, ev, rho, kap, ek, ei, n2, p2, res, it = 0, out = None},
   While[out === None,
     {cn, cp, ev} = states[bdg[mo, ns, ps]];
     {rho, kap, ek} = localDens[mo, cn, cp];
     {ei, n2, p2} = interaction[mo["ts"], rho, kap];
     If[ext =!= None, ei += Re[Tr[ext . rho]]; n2 = n2 + ext];
     res = Max[Abs[n2 - ns], Abs[p2 - ps]];
     If[res < tol || it >= maxit,
       out = <|"n" -> n2, "p" -> p2, "rho" -> rho, "kap" -> kap, "e" -> ek + ei, "it" -> it, "res" -> res, "cn" -> cn, "cp" -> cp|>,
       ns = mix n2 + (1 - mix) ns; ps = mix p2 + (1 - mix) ps;
       ns = 0.5 (ns + ConjugateTranspose[ns]); ps = 0.5 (ps - Transpose[ps]);
       it++]];
   out];
(* qpSF[q, N, P]: the smallest eigenvalue modulus of the BdG matrix at the momentum q; gapSF: its minimum over the
   momenta along x^3 (401 points and a golden-section refinement) *)
qpSF[q_, ns_, ps_] := Module[{h = ArrayFlatten[{{N[hLL[q, mLL]] + ns, ps}, {ConjugateTranspose[ps], -Transpose[N[hLL[-q, mLL]] + ns]}}]},
   Min[Abs[Eigenvalues[0.5 (h + ConjugateTranspose[h])]]]];
gapSF[ns_, ps_] := Module[{xs = Subdivide[0., lamLL, 400], vals, i, a, b, c1, c2},
   vals = Map[qpSF[{0., 0., #}, ns, ps] &, xs];
   i = First[Ordering[vals, 1]];
   a = xs[[Max[i - 1, 1]]]; b = xs[[Min[i + 1, 401]]];
   Do[c1 = b - 0.618034 (b - a); c2 = a + 0.618034 (b - a);
     If[qpSF[{0., 0., c1}, ns, ps] < qpSF[{0., 0., c2}, ns, ps], b = c2, a = c1], {60}];
   Min[qpSF[{0., 0., 0.5 (a + b)}, ns, ps], vals[[i]], qpSF[{0., 0., 0.}, ns, ps]]];
(* dGap[sol] = |D| (the pair amplitude of the channel K^3), diracMass[sol] = m_LL + tr(beta N)/8 *)
dGap[sol_] := Abs[Tr[k8[3] . sol["p"]]/8];
diracMass[sol_] := mLL + Re[Tr[b8 . sol["n"]]]/8;
(* effCouplings[method, gs, gv, g]: the effective scalar and pair couplings {g^S, g^P} of a method and a potential *)
effCouplings[m_List, gs_, gv_, g_] := Module[{ts = terms[m, gs, gv, g], n, kap, p},
   n = interaction[ts, b8/8, zero8][[2]];
   kap = -Conjugate[k8[3]];
   p = interaction[ts, zero8, kap][[3]];
   {Re[Tr[b8 . n]]/8, -2 Re[(Tr[k8[3] . p]/8)/Tr[k8[3] . Transpose[kap]]]}];
(* the five methods and their names in the tables *)
methodsSF = {{"HS", 1., 1.}, {"HS", 0., 1.}, {"HS", 0., 0.}, {"KS"}, {"CJT"}};
shortName[m_List] := If[m[[1]] === "HS", "HS_" <> ToString[Round[m[[2]]]] <> "_" <> ToString[Round[m[[3]]]], m[[1]]];
Print["the engine is defined; grid of 24 radial nodes: ", gridSF[24]["n"], " momenta"];

(* ::Input:: *)
(* YP2-1: the self-energies are the derivatives of the energy: central differences on three random states per method,
   for the terms of V^(2) (G^S = 2.8, G^V = 0.8, G = 25.2) *)
dP21 = Max[Table[Module[{ts = terms[m, 2.8, 0.8, 25.2]},
     Table[Module[{a, rho, b, kap, dr, dk, h = 10.^-4, ep, em, n, p, pred},
       a = RandomVariate[NormalDistribution[], {8, 8}] + I RandomVariate[NormalDistribution[], {8, 8}]; rho = 0.05 (a + ConjugateTranspose[a]);
       b = RandomVariate[NormalDistribution[], {8, 8}] + I RandomVariate[NormalDistribution[], {8, 8}]; kap = 0.05 (b - Transpose[b]);
       a = RandomVariate[NormalDistribution[], {8, 8}] + I RandomVariate[NormalDistribution[], {8, 8}]; dr = a + ConjugateTranspose[a];
       b = RandomVariate[NormalDistribution[], {8, 8}] + I RandomVariate[NormalDistribution[], {8, 8}]; dk = b - Transpose[b];
       ep = interaction[ts, rho + h dr, kap + h dk][[1]];
       em = interaction[ts, rho - h dr, kap - h dk][[1]];
       {n, p} = Rest[interaction[ts, rho, kap]];
       pred = Re[Tr[n . dr]] - Re[Tr[p . ConjugateTranspose[dk]]];
       Abs[(ep - em)/(2 h) - pred]/(1 + Abs[pred])], {3}]], {m, methodsSF}]];
check["WP2-1", "self-energies = derivatives of the energy, V^(2), five methods, random states", dP21, 10^-8, "P2-1a", "YP2-1"];
(* YP2-2: the pairing term alone on 48 radial nodes reproduces AS3: |d^3| = 0.2094933033 *)
moAS3 = makeModel[Table[{"p", {22.5, k8[i]}, {0., 0., 1.}, False}, {i, 3}], 48];
solAS3 = solveSF[moAS3, zero8, 0.3 ConjugateTranspose[k8[3]], 1., 10^-13];
d3AS3 = Abs[-0.5 22.5 Tr[k8[3] . Transpose[solAS3["kap"]]]];
check["WP2-2", "pairing-only limit = AS3 of Part I: |d^3| = " <> ToString[NumberForm[d3AS3, 11]] <> " (" <> ToString[solAS3["it"]] <> " iterations)",
  Abs[d3AS3 - 0.2094933033], 3 10^-10, "P2-2a", "YP2-2"];

(* ::Section:: *)
(*II.3 The recursion of the potential and the ground states*)

(* ::Text:: *)
(*The task asks to change the potential until all three methods admit a superfluid. **The effective couplings.** For a uniform state whose normal self-energy is a Dirac mass and whose pair self-energy lies in the channel $\mathsf K^3_{\rm LL}$, every method acts through two numbers: the scalar coupling $g^{X,{\rm S}}_{\rm SF}$ (the shift of the Dirac mass per unit scalar density) and the pair coupling $g^{X,{\rm P}}_{\rm SF}$ (the pair self-energy per unit pair amplitude), computed by `effCouplings` from the self-energy maps of the terms. The normal vacuum is unstable towards pairing when $\lambda^{\rm P}_{\rm SF}=g^{X,{\rm P}}_{\rm SF}/G^{\rm crit}_{\rm LL}>1$, with the critical pair coupling of AS3,*)

(* ::DisplayFormula:: *)
(*G^{\rm crit}_{\rm LL}=\frac{\pi^2}{I^{(1)}_{\rm SF}(m_{\rm LL})},\qquad I^{(1)}_{\rm SF}(M)=\int_0^{\Lambda_{\rm LL}}\frac{q^2\,dq}{(q^2+M^2)^{1/2}}=\tfrac12\Big((1+M^2)^{1/2}-M^2\,\mathrm{arsinh}\frac1M\Big).*)

(* ::Text:: *)
(***The three potentials.** $V^{(0)}_{\rm SF}$ (the author's: $G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$, no pairing term) admits no superfluid in any method. $V^{(1)}_{\rm SF}$ adds the pairing term with $G_{\rm LL}=22.5$, the value of AS3: HS becomes superfluid, but in KS and CJT the exchange contraction of the pairing term acts in the scalar channel as an attraction, the scalar channel becomes unstable, and the ground state is a massive state. $V^{(2)}_{\rm SF}$ keeps the form and the vector coupling and fixes $G^{\rm S}_{\rm SF}$ and $G_{\rm LL}$ by two conditions on the CJT couplings: the scalar coupling of $V^{(0)}_{\rm SF}$ (stable) and the pair coupling of $V^{(1)}_{\rm SF}$ (supercritical). The couplings are linear in $(G^{\rm S}_{\rm SF},G_{\rm LL})$, so the conditions are two linear equations, which the next cell solves from the computed couplings; the solution is $G^{\rm S}_{\rm SF}=2.8$, $G_{\rm LL}=25.2$:*)

(* ::DisplayFormula:: *)
(*V^{(2)}_{\rm SF}=\tfrac12(2.8):(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:+\tfrac12(0.8)\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:-\tfrac14(25.2)\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:.*)

(* ::Text:: *)
(*For each of the three potentials and the five methods, the cell after it computes the effective couplings and the stationary states reached from three starts (a polar pair $0.3(\mathsf K^3_{\rm LL})^\dagger$; a Dirac mass lowered by 1; a Dirac mass raised by 0.5), keeps the one of lowest energy (the ground state), and computes its energy per volume, its pair amplitude $\lvert D^X_{\rm SF}\rvert$, its Dirac mass and its gap. The ground state of $V^{(2)}_{\rm SF}$ is a polar superfluid in every method. Each number is compared with that of the Rust program. (This cell computes 45 self-consistent states; it takes a few minutes.)*)

(* ::Input:: *)
(* the critical pair coupling pi^2/I1(m) and the coefficients of V^(2) from the two conditions on the CJT couplings *)
gCrit = Pi^2/(0.5 (Sqrt[1 + mLL^2] - mLL^2 ArcSinh[1/mLL]));
check["WP2-6b", "the critical pair coupling pi^2/I1(m) = " <> ToString[NumberForm[N[gCrit], 11]], Abs[gCrit - rustPart2["p2_Gcrit"]], 10^-10, "P2-6b", "YP2-6b"];
Module[{f = effCouplings[{"CJT"}, #1, 0.8, #2] &, s0, p0, s1, p1, s2, p2, st, pt},
  {s0, p0} = f[0., 0.]; {s1, p1} = f[1., 0.]; {s2, p2} = f[0., 1.];
  st = effCouplings[{"CJT"}, -8., 0.8, 0.][[1]] - s0;
  pt = effCouplings[{"CJT"}, -8., 0.8, 22.5][[2]] - p0;
  {gS2, gP2} = LinearSolve[{{s1 - s0, s2 - s0}, {p1 - p0, p2 - p0}}, {st, pt}]];
check["WP2-6a", "V^(2) from the conditions on the CJT effective couplings: (G^S, G) = (" <> ToString[NumberForm[gS2, 11]] <> ", " <> ToString[NumberForm[gP2, 11]] <> ")",
  Max[Abs[gS2 - rustPart2["p2_V2_GS"]], Abs[gP2 - rustPart2["p2_V2_G"]]], 10^-10, "P2-6a", "YP2-6a"];
(* the three potentials {G^S, G^V, G} *)
potentialsSF = <|"V0" -> {-8., 0.8, 0.}, "V1" -> {-8., 0.8, 22.5}, "V2" -> {gS2, 0.8, gP2}|>;

(* ::Input:: *)
(* the recursion: for every potential and method the effective couplings and the ground state (the lowest of three
   starts); groundSF[key] = {model, solution}; the deviations from the Rust values are collected in devSF *)
devSF = <|"g" -> 0., "E" -> 0., "D" -> 0., "M" -> 0., "gap" -> 0.|>;
groundSF = <||>;
groundRows = {};
Do[Module[{pot = potentialsSF[pn], key = pn <> "_" <> shortName[m], gs, gp, mo, sols, best, gap},
   {gs, gp} = effCouplings[m, Sequence @@ pot];
   devSF["g"] = Max[devSF["g"], Abs[gs - rustPart2["p2_gS_" <> key]], Abs[gp - rustPart2["p2_gP_" <> key]]];
   mo = makeModel[terms[m, Sequence @@ pot]];
   sols = {solveSF[mo, zero8, 0.3 ConjugateTranspose[k8[3]]], solveSF[mo, -1. b8, zero8], solveSF[mo, 0.5 b8, zero8]};
   best = First[SortBy[sols, #["e"] &]];
   groundSF[key] = {mo, best};
   gap = gapSF[best["n"], best["p"]];
   devSF["E"] = Max[devSF["E"], Abs[best["e"] - rustPart2["p2_E_" <> key]]];
   devSF["D"] = Max[devSF["D"], Abs[dGap[best] - rustPart2["p2_D_" <> key]]];
   devSF["M"] = Max[devSF["M"], Abs[diracMass[best] - rustPart2["p2_M_" <> key]]];
   devSF["gap"] = Max[devSF["gap"], Abs[gap - rustPart2["p2_gap_" <> key]]];
   AppendTo[groundRows, {pn, shortName[m], gs, gp, 10^4 best["e"], dGap[best], diracMass[best], gap, best["it"]}]],
  {pn, {"V0", "V1", "V2"}}, {m, methodsSF}];
check["WP2-6c", "effective couplings g_S, g_P of 3 potentials x 5 methods = Rust", devSF["g"], 10^-11, "P2-6b, P2-6c", "YP2-6c"];
check["WP2-6d", "ground-state energies of 3 potentials x 5 methods = Rust", devSF["E"], 10^-11, "P2-6d", "YP2-6d"];
check["WP2-6e", "pair amplitudes |D| of the ground states = Rust", devSF["D"], 10^-8, "P2-6d", "YP2-6e"];
check["WP2-6f", "Dirac masses of the ground states = Rust", devSF["M"], 10^-8, "P2-6d", "YP2-6f"];
check["WP2-6g", "gaps of the ground states = Rust", devSF["gap"], 10^-7, "P2-6d", "YP2-6g"];
Grid[Prepend[Map[{#[[1]], #[[2]], NumberForm[#[[3]], 6], NumberForm[#[[4]], 6], NumberForm[#[[5]], 8], NumberForm[#[[6]], 8],
      NumberForm[#[[7]], 8], NumberForm[#[[8]], 8], #[[9]]} &, groundRows],
   {"potential", "method", "g^S", "g^P", "10^4 E (per volume)", "|D|", "Dirac mass", "gap", "iterations"}],
  Frame -> All, Alignment -> Right, BaseStyle -> {FontSize -> 9}]

(* ::Section:: *)
(*II.4 The first excited states: the linear response of the superfluid*)

(* ::Text:: *)
(*A small change of the self-energies with the time dependence $e^{-i\omega t}$ is described by $N_{\rm V}=120$ complex amplitudes: the 64 entries of a change $\delta\mathsf N$ of the normal self-energy and the 28 independent entries of each of two antisymmetric changes of the pair self-energy ($\delta\mathsf P$ and the change of its adjoint). The change of the state in answer to a change of the self-energies is the response $\chi(\omega)$, a $120\times120$ matrix,*)

(* ::DisplayFormula:: *)
(*\chi(\omega)=\sum_n\Big(\frac{\mathsf A_n}{\omega-\Omega_n}-\frac{\mathsf B_n}{\omega+\Omega_n}\Big),*)

(* ::Text:: *)
(*a sum over the momenta $n$ of the grid, where $\Omega_n=2E_n$ is the energy of a pair of quasi-particles at the momentum $n$ (for these states every eigenvalue of $\mathsf H_{\rm SF}$ at a momentum is $\pm E_n$, which the cell checks) and $\mathsf A_n$, $\mathsf B_n$ are built from the eigenvectors (the creation of two quasi-particles and its reverse). The self-energies change by $\mathsf F$ times the change of the state, where $\mathsf F$ is the $120\times120$ matrix of the self-energy maps of the contact terms (`respKernel`). A collective mode of frequency $\omega$ is a solution of $\delta=\chi(\omega)\mathsf F\delta$: the matrix $\mathsf M(\omega)=\chi(\omega)\mathsf F$ has the eigenvalue 1. At $\omega=0$ the three broken generators give three Goldstone modes: $\mathsf M(0)$ has the eigenvalue 1 three times. Below the threshold $\min(2E^{X,{\rm gap}}_{\rm SF},\min_n\Omega_n)$ the modes are found where the number of real eigenvalues of $\mathsf M(\omega)$ above 1 changes: a scan of 300 frequencies, then 45 bisections for each change. The cell does this for HS $(1,1)$, KS and CJT with $V^{(2)}_{\rm SF}$, and compares the modes with the Rust program.*)

(* ::Input:: *)
(* the 28 pairs (i, j), i < j, of the antisymmetric matrices, in the order of the rows *)
pairsSF = Flatten[Table[{i, j}, {i, 8}, {j, i + 1, 8}], 1];
(* toVec[x, a, b]: the 120 amplitudes of (dN, dP, dP-bar): the 64 entries of x, then 1/2 (a_ij - a_ji) and 1/2 (b_ij - b_ji) *)
toVec[x_, a_, b_] := Join[Flatten[x], Map[0.5 (a[[#[[1]], #[[2]]]] - a[[#[[2]], #[[1]]]]) &, pairsSF], Map[0.5 (b[[#[[1]], #[[2]]]] - b[[#[[2]], #[[1]]]]) &, pairsSF]];
(* fromVec[v]: the inverse: the matrix x and the two antisymmetric matrices *)
fromVec[v_] := Module[{x = Partition[v[[1 ;; 64]], 8], a = ConstantArray[0. I, {8, 8}], b = ConstantArray[0. I, {8, 8}]},
   Do[With[{i = pairsSF[[s, 1]], j = pairsSF[[s, 2]]},
     a[[i, j]] = v[[64 + s]]; a[[j, i]] = -v[[64 + s]]; b[[i, j]] = v[[92 + s]]; b[[j, i]] = -v[[92 + s]]], {s, 28}];
   {x, a, b}];
(* respKernel[ts]: the 120 x 120 matrix F of the self-energy maps of the contact terms *)
respKernel[ts_] := Transpose[Table[Module[{e = UnitVector[120, bb], rho, dk, dkb, n, p, pb},
     {rho, dk, dkb} = fromVec[N[e]];
     n = interaction[ts, rho, zero8][[2]];
     p = interaction[ts, zero8, dk][[3]];
     pb = ConjugateTranspose[interaction[ts, zero8, ConjugateTranspose[dkb]][[3]]];
     toVec[n, p, pb]], {bb, 120}]];
(* the 120 fields of the BdG form [[dN, dP], [dPbar, -dN^T]] *)
respFields = Table[Module[{dn, dp, dpb}, {dn, dp, dpb} = fromVec[N[UnitVector[120, bb]]];
    ArrayFlatten[{{dn, dp}, {dpb, -Transpose[dn]}}]], {bb, 120}];
(* respExtract[r]: the 120 amplitudes of a 16 x 16 change of the generalized density r *)
respExtract[r_] := Module[{r12 = -r[[1 ;; 8, 9 ;; 16]], r21 = -r[[9 ;; 16, 1 ;; 8]]},
   Join[Flatten[r[[1 ;; 8, 1 ;; 8]]], Map[0.5 (r12[[#[[1]], #[[2]]]] - r12[[#[[2]], #[[1]]]]) &, pairsSF],
    Map[0.5 (r21[[#[[1]], #[[2]]]] - r21[[#[[2]], #[[1]]]]) &, pairsSF]]];
(* response[mo, N, P]: {A_n, B_n, Omega_n, the largest spread of the positive eigenvalues of a node} *)
maskA = Table[If[k > 8 && l <= 8, 1., 0.], {k, 16}, {l, 16}];   (* k empty (upper half of the sorted spectrum), l occupied *)
maskB = Transpose[maskA];
response[mo_, ns_, ps_] := Module[{hs = bdg[mo, ns, ps], w = mo["g"]["w"], res, spread, om, aa, bb},
   res = Map[Function[h, Module[{ev, vec, ord, v, vd, x},
        {ev, vec} = Eigensystem[0.5 (h + ConjugateTranspose[h])]; ev = Re[ev];
        ord = Ordering[ev]; ev = ev[[ord]]; v = Transpose[vec[[ord]]]; vd = ConjugateTranspose[v];
        x = Map[vd . # . v &, respFields];
        {ev, Map[respExtract[v . (# maskA) . vd] &, x], Map[respExtract[v . (# maskB) . vd] &, x]}]], hs];
   spread = Max[Map[Max[#[[1, 9 ;; 16]]] - Min[#[[1, 9 ;; 16]]] &, res]];
   om = Map[2 #[[1, 9]] &, res];
   aa = MapThread[Transpose[#1] #2 &, {res[[All, 2]], w}];
   bb = MapThread[Transpose[#1] #2 &, {res[[All, 3]], w}];
   {aa, bb, om, spread}];
(* mMat[resp, f, w]: M(w) = chi(w) F *)
mMat[resp_, f_, w_] := ((1/(w - resp[[3]])) . resp[[1]] - (1/(w + resp[[3]])) . resp[[2]]) . f;
(* countModes[resp, f, w]: the number of real eigenvalues of M(w) above 1 *)
countModes[resp_, f_, w_] := Count[Eigenvalues[mMat[resp, f, w]], ev_ /; Re[ev] > 1 && Abs[Im[ev]] < 10^-7 (1 + Abs[Re[ev]])];

(* ::Input:: *)
(* the Goldstone modes and the collective modes of HS (1, 1), KS and CJT with V^(2) *)
modeRows = {};
Do[Module[{key = shortName[m], mo, s, resp, f, ev0, ng, thr, wmax, ws, nprev, wprev, modes = {}, rust},
   {mo, s} = groundSF["V2_" <> key];
   s = solveSF[mo, s["n"], s["p"], 0.6, 10^-13];
   resp = response[mo, s["n"], s["p"]];
   f = respKernel[mo["ts"]];
   ev0 = Eigenvalues[mMat[resp, f, 0.]];
   ng = Count[ev0, e_ /; Abs[e - 1] < 10^-6];
   check["WP2-7a " <> key, key <> ": three Goldstone modes (the eigenvalues of M(0) equal to 1); the spectrum of every node is +-E (spread " <> sci[resp[[4]]] <> ")",
    If[resp[[4]] < 10^-9, Abs[ng - 3] + Sort[Abs[ev0 - 1]][[3]], 1.], 10^-9, "P2-7a " <> key, "YP2-7a " <> key];
   thr = 2 gapSF[s["n"], s["p"]];
   wmax = Min[thr, Min[resp[[3]]]] (1 - 10^-3);
   ws = wmax (1 - (1 - Rest[Subdivide[0., 1., 300]])^2);
   nprev = countModes[resp, f, 10^-4 wmax]; wprev = 10^-4 wmax;
   Do[Module[{nn = countModes[resp, f, w], a, b, c},
      If[nn != nprev,
       a = wprev; b = w;
       Do[c = 0.5 (a + b); If[countModes[resp, f, c] == nprev, a = c, b = c], {45}];
       AppendTo[modes, 0.5 (a + b)]];
      nprev = nn; wprev = w], {w, ws}];
   rust = Sort[Values[KeySelect[rustPart2, StringStartsQ[#, "p2_mode"] && StringEndsQ[#, "_" <> key] &]]];
   check["WP2-7b " <> key, key <> ": the collective modes below the threshold " <> ToString[NumberForm[#, 9] & /@ Sort[modes]] <> " = Rust " <> ToString[NumberForm[#, 9] & /@ rust],
    If[Length[modes] == Length[rust], Max[Abs[Sort[modes] - rust]], 1.], 10^-7, "P2-7a " <> key, "YP2-7b " <> key];
   AppendTo[modeRows, {key, ng, thr, Sort[modes]}]], {m, {{"HS", 1., 1.}, {"KS"}, {"CJT"}}}];
Grid[Prepend[Map[{#[[1]], #[[2]], NumberForm[#[[3]], 8], Row[NumberForm[#, 8] & /@ #[[4]], ", "]} &, modeRows],
   {"method", "Goldstone modes", "threshold 2 E_gap", "collective modes"}], Frame -> All, Alignment -> Left]

(* ::Section:: *)
(*II.5 The gauge bosons in the superfluid*)

(* ::Text:: *)
(***The masses from the susceptibilities.** A constant gauge field $A^a_0$ along the generator $\mathsf Q_a$ ($a=1,2,3$: $T^\iota_{\rm LL}$; $a=4$: the fermion number $\mathsf 1_8$) enters like a chemical potential $\mu^a$ of that charge. The static susceptibility $\chi^{ab}_{\rm SF}=\partial\langle\mathsf Q_a\rangle/\partial\mu^b$ of the superfluid gives the mass matrix of the gauge bosons, $(M^2)_{ab}=g_ag_b\chi^{ab}_{\rm SF}$ (the Higgs mechanism of the condensate), with $g_a=g^{\rm v}_{\rm LL}$ for the three $W^\iota_{\rm SF}$ and $g^{\rm F}_{\rm LL}$ for $B_{\rm SF}$. The next cell computes $\chi^{ab}_{\rm SF}$ by central differences of the self-consistent states with the external self-energies $\mp h\,\mathsf Q_b$, $h=2\times10^{-3}$ and $2h$, and the Richardson extrapolation. The generator $T^3_{\rm LL}$ leaves the condensate in the channel $\mathsf K^3_{\rm LL}$ unchanged, so $\chi^{33}_{\rm SF}=0$: $W^3_{\rm SF}$ stays massless (the zeugon), and there is no mixing.*)

(* ::Input:: *)
(* the four charges: T^1, T^2, T^3 (1_4 (x) tau/2) and the fermion number 1_8 *)
chargesSF = Join[Table[N[i8[sig[[k + 1]]/2]], {k, 3}], {id8}];
(* susceptibilitySF[mo, s]: the 4 x 4 static susceptibilities by central differences and Richardson *)
susceptibilitySF[mo_, s_, h_ : 2. 10^-3] := Transpose[Table[Module[{dn},
     dn = Table[Module[{r = Table[Module[{ext = -sg step chargesSF[[bb]]},
            solveSF[mo, s["n"] + ext, s["p"], 0.6, 10^-14, 40000, ext]["rho"]], {sg, {1, -1}}]},
        Table[Re[Tr[chargesSF[[a]] . r[[1]]] - Tr[chargesSF[[a]] . r[[2]]]]/(2 step), {a, 4}]], {step, {h, 2 h}}];
     (4 dn[[1]] - dn[[2]])/3], {bb, 4}]];
chiRows = {};
Do[Module[{key = shortName[m], mo, s, chi, off},
   {mo, s} = groundSF["V2_" <> key];
   chi = susceptibilitySF[mo, s];
   off = Max[Abs[chi - DiagonalMatrix[Diagonal[chi]]]];
   check["WP2-8a " <> key, key <> ": chi_11 = " <> ToString[NumberForm[chi[[1, 1]], 10]] <> ", chi_FF = " <> ToString[NumberForm[chi[[4, 4]], 10]] <> " = Rust; chi_33 = " <> sci[chi[[3, 3]]] <> " (massless zeugon), no mixing",
    Max[Abs[chi[[1, 1]] - rustPart2["p2_chi11_" <> key]], Abs[chi[[4, 4]] - rustPart2["p2_chiFF_" <> key]], Abs[chi[[3, 3]]], off], 10^-8,
    "P2-8a " <> key <> ", P2-8b " <> key, "YP2-8a " <> key];
   AppendTo[chiRows, {key, chi[[1, 1]], chi[[3, 3]], chi[[4, 4]], gvLL Sqrt[chi[[1, 1]]], gfLL Sqrt[chi[[4, 4]]]}]], {m, {{"HS", 1., 1.}, {"KS"}, {"CJT"}}}];
Grid[Prepend[Map[{#[[1]], NumberForm[#[[2]], 8], sci[#[[3]]], NumberForm[#[[4]], 8], NumberForm[#[[5]], 8], NumberForm[#[[6]], 8]} &, chiRows],
   {"method", "chi_11 = chi_22", "chi_33", "chi_FF", "M_W = g_v sqrt(chi_11)", "M_Z = g_F sqrt(chi_FF)"}], Frame -> All, Alignment -> Right]

(* ::Text:: *)
(***The exchange of the gauge bosons.** Beyond their contact limit, the four gauge bosons are exchanged between the charges with the propagator $1/(\lvert\vec q-\vec q\,'\rvert^2+M^2)$ of a boson of mass $M$. In a uniform state the exchange self-energies at the momentum $\vec q$ are integrals over $\vec q\,'$ of the correlation matrices; with the six directions of the grid the correlation matrix at a radius has a part of angular momentum 0 (the mean over the directions) and three parts of angular momentum 1, and the angular integrals reduce to the two angular averages*)

(* ::DisplayFormula:: *)
(*D_0(q,q')=\frac12\int_{-1}^1\frac{dx}{q^2+q'^2+M^2-2qq'x},\qquad D_1(q,q')=\frac12\int_{-1}^1\frac{x\,dx}{q^2+q'^2+M^2-2qq'x},*)

(* ::Text:: *)
(*whose closed forms contain $\ln((q-q')^2+M^2)$, singular at $q=q'$ for a massless boson; the radial integral subtracts this logarithm at $q'=q$ and integrates it exactly (`kernIntegral`). The next cell compares the closed forms with a numerical integration over the angle; then it computes the Thouless eigenvalues of the exchange of the zeugon $W^3_{\rm SF}$ alone and of $B_{\rm SF}$ alone in the normal vacuum (per unit squared coupling; a positive eigenvalue attracts the pair $\mathsf K^3_{\rm LL}$), by power iteration; and finally the CJT state of $V^{(2)}_{\rm SF}$ with the exchange of all four bosons at the couplings $g^{\rm v}_{\rm LL}=0.7$, $g^{\rm F}_{\rm LL}=0.45$ and the boson masses of the Rust program.*)

(* ::Input:: *)
(* kernSF[g, mass]: the radial kernels of D0, D1 in closed form: {r0, r1}, {s0, s1} with D_l = r_l + s_l ln((q - q')^2 + M^2),
   the logarithm lm, and aq = the exact integral of ln((q - q')^2 + M^2) over q' in [0, Lambda] *)
kernSF[g_, mass_] := Module[{kr = g["kr"], q, qp, a, b, lp, lm, fp},
   q = Outer[#1 &, kr, kr]; qp = Outer[#2 &, kr, kr];
   a = q^2 + qp^2 + mass^2; b = 2 q qp; lp = Log[a + b];
   lm = Table[If[mass == 0 && i == j, 0., Log[(kr[[i]] - kr[[j]])^2 + mass^2]], {i, Length[kr]}, {j, Length[kr]}];
   fp[x_] := If[mass == 0, If[x == 0, 0., x Log[x^2] - 2 x], x Log[x^2 + mass^2] - 2 x + 2 mass ArcTan[x/mass]];
   <|"r" -> {lp/(2 b), -1/b + a lp/(2 b^2)}, "s" -> {-1/(2 b), -a/(2 b^2)}, "lm" -> lm,
     "aq" -> Map[fp[lamLL - #] - fp[-#] &, kr], "g" -> g|>];
(* kernIntegral[kern, l, f]: the integral over q' of q'^2/(2 pi^2) D_l(q_i, q') f(q') for a list f of nr matrices, at all q_i,
   with the logarithm subtracted at q' = q_i and integrated exactly *)
kernIntegral[kern_, l_, f_] := Module[{g = kern["g"], kr, wr, w, nr, out, u, ui},
   kr = g["kr"]; wr = g["wr"]; nr = Length[kr]; w = wr kr^2/(2 Pi^2);
   out = kern["r"][[l + 1]] . (w f);
   Table[Module[{ui = kern["s"][[l + 1]][[i, i]] kr[[i]]^2/(2 Pi^2) f[[i]]},
      out[[i]] + Sum[wr[[j]] kern["lm"][[i, j]] (kern["s"][[l + 1]][[i, j]] kr[[j]]^2/(2 Pi^2) f[[j]] - ui), {j, nr}] + ui kern["aq"][[i]]], {i, nr}]];
(* the vertices of a boson with the isospin matrix t: {eta_pp, (gamma^0 gamma^p) (x) t} for p = 0..3 *)
verticesSF[t_] := Table[{etaLL[[p + 1, p + 1]], N[d8[gam[0] . gam[p]] . i8[t]]}, {p, 0, 3}];
(* makeBosons[g, gv, gf, masses]: the four bosons {coupling, kernel, vertices} *)
makeBosons[g_, gv_, gf_, masses_] := Table[{{gv, gv, gv, gf}[[i]], kernSF[g, masses[[i]]], verticesSF[{sig[[2]]/2, sig[[3]]/2, sig[[4]]/2, IdentityMatrix[2]}[[i]]]}, {i, 4}];
(* exchangeSF[g, bosons, dcn, cp]: the exchange self-energies at every node and the exchange energy, from the
   correlation matrices dcn = C^n - C^vac and cp at the nodes *)
exchangeSF[g_, bosons_, dcn_, cp_] := Module[{nr = g["nr"], x, y, l0r, l0k, l1r, l1k, dirs = g["dirs"], ns, ps, e},
   x = Partition[dcn, 6]; y = Partition[cp, 6];              (* [radius][direction] *)
   l0r = Map[Mean, x]; l0k = Map[Mean, y];
   l1r = Table[0.5 (x[[All, 2 j - 1]] - x[[All, 2 j]]), {j, 3}];
   l1k = Table[0.5 (y[[All, 2 j - 1]] - y[[All, 2 j]]), {j, 3}];
   ns = ConstantArray[0. I, {nr, 6, 8, 8}]; ps = ConstantArray[0. I, {nr, 6, 8, 8}];
   Do[Module[{gb = bos[[1]], kern = bos[[2]], fr, fk, i0r, i0k, i1r, i1k},
      i0r = kernIntegral[kern, 0, l0r]; i0k = kernIntegral[kern, 0, l0k];
      i1r = Table[kernIntegral[kern, 1, l1r[[j]]], {j, 3}]; i1k = Table[kernIntegral[kern, 1, l1k[[j]]], {j, 3}];
      fr = Table[i0r[[r]] + Sum[i1r[[j, r]] dirs[[d, j]], {j, 3}], {r, nr}, {d, 6}];
      fk = Table[i0k[[r]] + Sum[i1k[[j, r]] dirs[[d, j]], {j, 3}], {r, nr}, {d, 6}];
      Do[With[{eta = vt[[1]], a = vt[[2]]},
        ns -= eta gb^2 Map[a . # . a &, fr, {2}];
        ps -= eta gb^2 Map[a . # . Transpose[a] &, fk, {2}]], {vt, bos[[3]]}]], {bos, bosons}];
   ns = Flatten[ns, 1]; ps = Flatten[ps, 1];
   e = 0.5 Re[g["w"] . MapThread[Tr[#1 . #2] &, {ns, dcn}]] - 0.5 Re[g["w"] . MapThread[Total[#1 Conjugate[#2], 2] &, {ps, cp}]];
   {ns, ps, e}];

(* ::Input:: *)
(* the closed forms of D0, D1 against a numerical integration over the angle (mass 0.5, the radial nodes 6 and 12) *)
Module[{g = gridSF[24], kern, q, qp, d0, d1, lm},
  kern = kernSF[g, 0.5];
  q = g["kr"][[6]]; qp = g["kr"][[12]];
  (* the integrals with 30 digits: the two radii are given 30 digits (their machine values, extended with zeros) *)
  With[{qq = SetPrecision[q, 30], qqp = SetPrecision[qp, 30]},
   d0 = 1/2 NIntegrate[1/(qq^2 + qqp^2 + 1/4 - 2 qq qqp x), {x, -1, 1}, WorkingPrecision -> 30, PrecisionGoal -> 20];
   d1 = 1/2 NIntegrate[x/(qq^2 + qqp^2 + 1/4 - 2 qq qqp x), {x, -1, 1}, WorkingPrecision -> 30, PrecisionGoal -> 20]];
  lm = Log[(q - qp)^2 + 0.25];
  check["WP2-8b", "the angular averages D0, D1 of the Yukawa kernel in closed form = numerical integration over the angle",
   Max[Abs[kern["r"][[1]][[6, 12]] + kern["s"][[1]][[6, 12]] lm - d0], Abs[kern["r"][[2]][[6, 12]] + kern["s"][[2]][[6, 12]] lm - d1]], 10^-12,
   "P2-3a, P2-3b", "YP2-8b"]];

(* ::Input:: *)
(* the Thouless eigenvalues of the exchange of W^3 alone and of B alone in the normal vacuum (coupling 1, mass 0),
   by power iteration: the pair field P(q) -> the exchange pair self-energy of the pair correlation it induces *)
Module[{mo = makeModel[{}], g, eig, applyP, lam = <||>},
  g = mo["g"];
  eig = Map[Function[h, Module[{ev, vec, ord}, {ev, vec} = Eigensystem[0.5 (h + ConjugateTranspose[h])]; ev = Re[ev];
       ord = Ordering[ev]; {ev[[ord]], Transpose[vec[[ord]]]}]], bdg[mo, zero8, zero8]];
  (* the response of the pair correlation to a pair field: weights (f_l - f_k)/(E_l - E_k) of the transitions *)
  applyP[p_] := MapThread[Function[{e, pn}, Module[{ev = e[[1]], v = e[[2]], f, wkl, dh, x, r},
       f = Map[If[# < 0, 1., 0.] &, ev];
       wkl = Table[If[f[[l]] != f[[k]], (f[[l]] - f[[k]])/(ev[[l]] - ev[[k]]), 0.], {k, 16}, {l, 16}];
       dh = ArrayFlatten[{{zero8, pn}, {zero8, zero8}}];
       x = ConjugateTranspose[v] . dh . v;
       r = v . (x wkl) . ConjugateTranspose[v];
       -r[[1 ;; 8, 9 ;; 16]]]], {eig, p}];
  Do[Module[{name = pr[[1]], bos = {makeBosons[g, 1., 1., {0., 0., 0., 0.}][[pr[[2]]]]}, p, lold = 0., lnew = 0., it = 0, done = False},
     p = ConstantArray[ConjugateTranspose[k8[3]], g["n"]];
     While[! done && it < 400,
      Module[{kap = applyP[p], pe, num, dd},
       pe = exchangeSF[g, bos, ConstantArray[zero8, g["n"]], kap][[2]];
       num = Re[g["w"] . MapThread[Total[Conjugate[#1] #2, 2] &, {p, pe}]];
       dd = Re[g["w"] . Map[Total[Abs[#]^2, 2] &, p]];
       lnew = num/dd;
       p = pe/Sqrt[Re[g["w"] . Map[Total[Abs[#]^2, 2] &, pe]]/dd];
       If[Abs[lnew - lold] < 10^-12 && it > 3, done = True];
       lold = lnew; it++]];
     lam[name] = lnew], {pr, {{"W3", 3}, {"B", 4}}}];
  thoulessSF = lam];
check["WP2-8c", "the Thouless eigenvalues of the exchange per unit coupling squared: zeugon W^3 " <> ToString[NumberForm[thoulessSF["W3"], 10]] <> ", B " <> ToString[NumberForm[thoulessSF["B"], 10]] <> " = Rust",
  Max[Abs[thoulessSF["W3"] - rustPart2["p2_thouless3"]], Abs[thoulessSF["B"] - rustPart2["p2_thouless4"]]], 10^-9, "P2-8c, P2-8d", "YP2-8c"];

(* ::Text:: *)
(*The zeugon attracts the condensed pair (positive eigenvalue) and $B_{\rm SF}$ repels it four times as strongly. **CJT with the exchange.** The last cell of Part II makes the CJT state of $V^{(2)}_{\rm SF}$ self-consistent with the exchange of all four gauge bosons added to the contact terms: the self-energies now depend on the momentum, and the matrix of Bogoliubov and de Gennes at the momentum $\vec q$ contains the exchange self-energies at $\vec q$ (and at $-\vec q$ in its lower block). The boson masses are those of the Rust program, $M_W$ and $M_Z$ of this state and a massless zeugon; the iteration mixes one half of the new self-energies until they change by less than $10^{-11}$. The superfluid survives, and the energy agrees with the Rust value.*)

(* ::Input:: *)
(* CJT with the exchange of the four gauge bosons at g_v = 0.7, g_F = 0.45 *)
Module[{mo, s, bos, nc, pc, nex, pex, it = 0, res = 1., cn, cp, ev, rho, kap, ek, ei, n2, p2, n3, p3, eex, masses},
  {mo, s} = groundSF["V2_CJT"];
  masses = {rustPart2["p2_cjtex_M_W"], rustPart2["p2_cjtex_M_W"], 0., rustPart2["p2_cjtex_M_Z"]};
  bos = makeBosons[mo["g"], gvLL, gfLL, masses];
  nc = s["n"]; pc = s["p"];
  nex = ConstantArray[zero8, mo["g"]["n"]]; pex = ConstantArray[zero8, mo["g"]["n"]];
  While[res >= 10^-11 && it < 20000,
   {cn, cp, ev} = states[bdg[mo, nc, pc, nex, pex]];
   {rho, kap, ek} = localDens[mo, cn, cp];
   {ei, n2, p2} = interaction[mo["ts"], rho, kap];
   {n3, p3, eex} = exchangeSF[mo["g"], bos, cn - mo["vac"], cp];
   res = Max[Abs[n2 - nc], Abs[p2 - pc], Abs[n3 - nex], Abs[p3 - pex]];
   If[res >= 10^-11,
    nc = 0.5 n2 + 0.5 nc; pc = 0.5 p2 + 0.5 pc;
    nex = 0.5 n3 + 0.5 nex; pex = 0.5 p3 + 0.5 pex;
    it++]];
  cjtExchange = <|"energy" -> ek + ei + eex, "exchange" -> eex, "iterations" -> it|>;
  check["WP2-8d", "CJT with the exchange at g_v = 0.7, g_F = 0.45: energy " <> sci[ek + ei + eex] <> " = Rust (relative; " <> ToString[it] <> " iterations; exchange energy " <> sci[eex] <> ")",
   Abs[(ek + ei + eex) - rustPart2["p2_cjtex_energy"]]/Abs[rustPart2["p2_cjtex_energy"]], 10^-7, "P2-8f", "YP2-8d"]];

(* ::Text:: *)
(***A remark on the zeugon (what it means and why it matters).** After the breaking exactly one gauge boson stays massless, $W^3_{\rm SF}$ for the condensate in the channel $\mathsf K^3_{\rm LL}$ (in general $\vec n\cdot\vec W_{\rm SF}$ for a vacuum along $\vec n$), the gauge boson of the unbroken charge $Q_{\rm LL}=T^3_{\rm LL}$ of $SU(2)_{\rm v}$. It is not the photon: only the DiracLike spinor (charges $\pm\tfrac12$), the bosons $W^\pm$ and the two non-unitary scalars (charges $\pm1$) carry $Q_{\rm LL}$, and nothing in the model carries electric charge. The author, who asked for a new massless gauge boson that appears after the breaking and is not the photon, offered the solution its naming; the name is the zeugon (from the Greek zeugos, a yoked pair), because it is the only gauge boson that attracts the two charges of the condensed pair, as the cells above show. Two qualifications: its attraction alone would pair the normal vacuum only for $g^{\rm v}_{\rm LL}$ above about 5.3, almost eight times the coupling of the project, so at $g^{\rm v}_{\rm LL}=0.7$ it does not by itself hold the condensate, which the pairing term of the potential produces; and it is a prediction of the model, not an observation. Why it matters: the group that survives the breaking is $O(2)$, not $U(1)$; its second component, a rotation by $\pi$ of the isospin combined with the phase $\pi/2$ of the fermion number, is a gauge transformation that reverses the zeugon and exchanges $W^+$ and $W^-$, so the charge conjugation of the zeugon is a local symmetry, which it never is for the photon. A $U(1)$ gauge theory with a local charge conjugation is Alice electrodynamics (J. Kiskis 1978; A. S. Schwarz 1982; M. Bucher, H.-K. Lo and J. Preskill 1992): the vacua $\Phi_0e^{i\vartheta}\vec n$ and $\Phi_0e^{i(\vartheta+\pi)}(-\vec n)$ are the same field, so a half-quantum vortex of the superfluid, along which $\vec n$ turns into $-\vec n$ while the phase advances by $\pi$, is an Alice string, which reverses the charge $Q_{\rm LL}$ of every particle carried around it (U. Leonhardt and G. E. Volovik described its counterpart in a condensate of spin 1 in 2000). These strings, and the magnetic monopoles of the zeugon that the sphere of directions $\vec n$ allows, follow from the topology of the vacuum and are not computed in this notebook.*)
