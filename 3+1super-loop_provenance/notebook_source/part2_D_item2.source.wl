(* ::Section:: *)
(*II.3 Item [2]: analytic solutions of the field equations, and the checks AS-1 to AS-10 (program t2_analytic)*)

(* ::Text:: *)
(*The program t2_analytic (t2analytic/src/main.rs) answers item [2] of Test 2, which asks for an analysis of the field equations and for analytic solutions of simple cases. It solves the mean-field (Dirac-Bogoliubov-de Gennes) equations exactly in four cases and checks every solution twice: against the continuum equations, and against the lattice operator $\mathcal H^{\rm lat}_{\rm LL}$ of chapter II.1.4, the operator that the solvers of the items [3] and [5] integrate. The solutions are therefore also test problems with known answers for those solvers. The cases are: A, free spinor charges in constant gauge potentials (checks AS-1 and AS-2); B, the uniform polar superfluid (AS-3 to AS-6); C, a domain wall of the pair field (AS-7 to AS-9); D, constant magnetic fields (AS-10). The rewriting below keeps the order of the original program, because its random numbers are drawn in that order from one generator with the seed 2026111708; the checks AS-3, AS-7 and AS-8 draw random numbers, the others draw none. Every check prints one line with PASS or FAIL. The statement is that of the original report (in the two checks with a time integration, AS-2 and AS-4, it names IDA in place of CVODE and states what is required of IDA); the numbers in the details are the results of the Wolfram Language, followed by the values of the original report.*)

(* ::Text:: *)
(***The equations.** All four cases are computed in the canonical gauge, in which the spin connection vanishes ($\Gamma^{\rm LL}_p=0$); a gauge transformation carries every solution into any other gauge. A mode has the particle part $\psi^{\rm p}_{\rm LL}$ and the hole part $\psi^{\rm h}_{\rm LL}$, each with the eight components of chapter II.1.3 (the components 1, 3, 5, 7 have isospin up, the components 2, 4, 6, 8 isospin down). The mean-field equations are*)

(* ::DisplayFormula:: *)
(*i\partial_0\psi^{\rm p}_{\rm LL}=\big(h^\nabla_{\rm LL}-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}\big)\psi^{\rm p}_{\rm LL}+\mathsf P_{\rm LL}\psi^{\rm h}_{\rm LL}*)

(* ::DisplayFormula:: *)
(*i\partial_0\psi^{\rm h}_{\rm LL}=\mathsf P_{\rm LL}^\dagger\psi^{\rm p}_{\rm LL}-\big(h^\nabla_{\rm LL}-\mathsf A_{{\rm LL},0}-\mu_{\rm LL}\big)^{*}\psi^{\rm h}_{\rm LL}*)

(* ::Text:: *)
(*with the covariant Dirac Hamiltonian $h^\nabla_{\rm LL}=-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j+\beta_{\rm LL}m_{\rm LL}$, the covariant derivative $\nabla^{\rm LL}_j=\partial_j-i\mathsf A_{{\rm LL},j}$, the gauge matrices $\mathsf A_{{\rm LL},p}=g^{\rm v}_{\rm LL}\sum_\iota A^\iota_{{\rm LL},p}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}\mathsf 1_8$ of the potentials, the chemical potential $\mu_{\rm LL}$, and the pairing block $\mathsf P_{\rm LL}=\sum_\iota d^\iota_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger$ of the order parameter $\vec d_{\rm LL}=(d^1_{\rm LL},d^2_{\rm LL},d^3_{\rm LL})$. The matrices $\alpha^j_{\rm LL}=\mathrm{diag}(-\sigma^{\rm LL}_j,\sigma^{\rm LL}_j)$ and $\beta_{\rm LL}=\gamma^0_{\rm LL}$ act on the Dirac index, the potentials on the isospin index. The operator on the right side is the Dirac-Bogoliubov-de Gennes operator $\mathcal H^{\rm DB}_{\rm LL}$; a stationary mode depends on the time through the factor $\exp(-iE^{\rm qp}_{{\rm LL},n}x^0_{\rm LL})$, and its energy $E^{\rm qp}_{{\rm LL},n}$ is an eigenvalue of $\mathcal H^{\rm DB}_{\rm LL}$. If all fields are constant in space, the modes are plane waves, $\exp(i\sum_jq^j_{\rm LL}x^j_{\rm LL})$ times a constant vector of 16 components, and the field equations are the eigenvalue problem of a $16\times16$ matrix.*)

(* ::Text:: *)
(***The lattice of the checks.** Every lattice of this chapter is a periodic chain (a ring) of $n^{\rm x}_{\rm LL}$ sites along $x^1_{\rm LL}$, with the spacing $\ell^{\rm x}_{\rm LL}$ and the Wilson coefficient $r^{\rm W}_{\rm LL}=1$. Its allowed momenta $q^1_{\rm LL}$ are the integer multiples of $2\pi/(n^{\rm x}_{\rm LL}\ell^{\rm x}_{\rm LL})$; the function `t2chainMomentum` returns the allowed momentum with a given number (counted from 0) in the interval from $-\pi/\ell^{\rm x}_{\rm LL}$ (excluded) to $\pi/\ell^{\rm x}_{\rm LL}$ (included). If the fields are the same on every site, the lattice operator maps a plane wave to a plane wave of the same momentum, as in check GS-16; the function `t2planeMatrix` applies the operator to the 16 plane waves of one momentum (one for each component of a site) and returns the $16\times16$ matrix of their images at the first site. The function `t2h8` returns the matrix $\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ of a plane wave of the continuum, as an $8\times8$ matrix (it acts as the unit matrix on the isospin index). The cell also sets the seed and the matrices that several checks use; they are exact (their entries are the integers 0, 1, $-1$ and the imaginary unit).*)

(* ::Input:: *)
(* the seed of the program t2_analytic *)
t2Seed[2026111708];
(* the couplings g^v, g^F and the mass m: the same numbers as in chapter II.2 (the original program sets them again) *)
gv = 0.7; gf = 0.45; mass = 0.4;
(* a chain has more than one site only in the direction x^1: the argument of t2epsLat and t2hLatPlane that says so *)
asChainActive = {True, False, False};
(* the places of the isospin-up and of the isospin-down components among the eight components of a site *)
asUpIndex = {1, 3, 5, 7}; asDownIndex = {2, 4, 6, 8};
(* the matrices that several checks use: the pair matrix K^3 of the third isospin direction and its Hermitian
   conjugate, the matrices alpha^1, alpha^2, alpha^3 and beta on the eight components (Dirac (x) isospin) *)
asK3 = t2kpair[3];
asK3dag = ConjugateTranspose[asK3];
asAlpha = Table[t2lift4[t2alpha[j]], {j, 3}];
asBeta = t2lift4[t2beta];
(* the allowed momentum number kq (0, 1, ..., nsites - 1) of a periodic chain of nsites sites with the spacing ell,
   in the interval (-pi/ell, pi/ell] *)
t2chainMomentum[kq_Integer, nsites_Integer, ell_] := 2. Pi If[kq <= Quotient[nsites, 2], kq, kq - nsites]/(nsites ell);
(* the 16 x 16 matrix of the lattice operator on the plane waves exp(i q x^1) e_b of a periodic chain (b = 1..16, the
   components of a site): with fields that are the same on every site, the image of a plane wave is a plane wave of
   the same momentum; its 16 components at the first site are the column b.  The 16 plane waves are the columns of
   the matrix planeWaves, to which t2apply applies the operator column by column *)
t2planeMatrix[lattice_, fields_, q_] := Module[{nsites = t2nsites[lattice], planeWaves},
   planeWaves = Table[If[Mod[row - 1, 16] + 1 == b, Exp[I q lattice["ell"] Quotient[row - 1, 16]], 0. I], {row, 16 nsites}, {b, 16}];
   t2apply[lattice, fields, planeWaves][[1 ;; 16]]];
(* h(q) = sum_j alpha^j q_j + beta m on Dirac (x) isospin (8 x 8), the plane-wave Hamiltonian of the continuum *)
t2h8[q_List, m_] := m asBeta + Sum[q[[j]] asAlpha[[j]], {j, 3}];
(* numbers in the details of the checks: two significant digits with a power of ten; a fixed number of decimals *)
asSci[x_?NumericQ] := ScientificForm[x, 2];
asSci[l_List] := Row[Riffle[Map[ScientificForm[#, 2] &, l], ", "]];
asFix[x_?NumericQ, decimals_Integer] := NumberForm[x, {20, decimals}];
(* the largest deviation of the columns cols of two lists of rows; Infinity if the lists have different dimensions *)
t2rowsDeviation[rowsHere_, rowsOriginal_, cols_] := If[Dimensions[rowsHere] === Dimensions[rowsOriginal],
   Max[Abs[rowsHere[[All, cols]] - rowsOriginal[[All, cols]]]], Infinity];

(* ::Subsection:: *)
(*Case A: free spinor charges in constant gauge potentials (checks AS-1 and AS-2)*)

(* ::Text:: *)
(***The fields.** No pair field ($\vec d_{\rm LL}=0$), and constant potentials of the fermion number and of the third isospin direction, $A^{\rm F}_{{\rm LL},p}$ and $A^3_{{\rm LL},p}$, with $A^1_{{\rm LL},p}=A^2_{{\rm LL},p}=0$. The generator $T^3_{\rm LL}$ is diagonal, so the gauge matrix $\mathsf A_{{\rm LL},p}$ is diagonal in the isospin index: it has the eigenvalue $\mathsf A^\uparrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}+\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-up components (the charge $Q_{\rm LL}=T^3_{\rm LL}=+\tfrac12$) and the eigenvalue $\mathsf A^\downarrow_{{\rm LL},p}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}-\tfrac12g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}$ on the isospin-down components ($Q_{\rm LL}=-\tfrac12$). The field strengths vanish.*)

(* ::Text:: *)
(***The solution.** Take a particle of isospin up, $\psi^{\rm p}_{\rm LL}=\exp\big(i\sum_jq^j_{\rm LL}x^j_{\rm LL}-iE^{\rm qp}_{{\rm LL},n}x^0_{\rm LL}\big)\chi^{\rm pw}_{\rm LL}$ with a constant spinor $\chi^{\rm pw}_{\rm LL}$ (a Dirac spinor times the isospin vector $(1,0)^T$), and $\psi^{\rm h}_{\rm LL}=0$. The covariant derivative gives $\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL}=i(q^j_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},j})\psi^{\rm p}_{\rm LL}$, and the field equations become the eigenvalue problem $E^{\rm qp}_{{\rm LL},n}\chi^{\rm pw}_{\rm LL}=\big(\sum_j\alpha^j_{\rm LL}(q^j_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},j})+\beta_{\rm LL}m_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},0}-\mu_{\rm LL}\big)\chi^{\rm pw}_{\rm LL}$. The Dirac matrices obey $(\alpha^j_{\rm LL})^2=\beta_{\rm LL}^2=\mathsf 1_4$, $\alpha^j_{\rm LL}\alpha^{j'}_{\rm LL}+\alpha^{j'}_{\rm LL}\alpha^j_{\rm LL}=0$ for $j\neq j'$ and $\alpha^j_{\rm LL}\beta_{\rm LL}+\beta_{\rm LL}\alpha^j_{\rm LL}=0$ (check GS-1). The matrix $\sum_j\alpha^j_{\rm LL}(q^j_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},j})+\beta_{\rm LL}m_{\rm LL}$ therefore squares to $\sum_j(q^j_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},j})^2+m_{\rm LL}^2$ times the unit matrix, and its eigenvalues are the two square roots of this number, each twice. A hole (with $\psi^{\rm p}_{\rm LL}=0$) obeys the second field equation, whose operator $-(h^\nabla_{\rm LL}-\mathsf A_{{\rm LL},0}-\mu_{\rm LL})^{*}$ contains the charges with the opposite sign. The energies of the isospin-up components are, each twice,*)

(* ::DisplayFormula:: *)
(*E^{\rm qp}_{{\rm LL},n}=\pm\Big(\sum_j(q^j_{\rm LL}-\mathsf A^\uparrow_{{\rm LL},j})^2+m_{\rm LL}^2\Big)^{1/2}-\mathsf A^\uparrow_{{\rm LL},0}-\mu_{\rm LL}\quad\text{(particles)},\qquad E^{\rm qp}_{{\rm LL},n}=\mp\Big(\sum_j(q^j_{\rm LL}+\mathsf A^\uparrow_{{\rm LL},j})^2+m_{\rm LL}^2\Big)^{1/2}+\mathsf A^\uparrow_{{\rm LL},0}+\mu_{\rm LL}\quad\text{(holes)}*)

(* ::Text:: *)
(*and those of the isospin-down components are the same with $\mathsf A^\downarrow_{{\rm LL},p}$ in place of $\mathsf A^\uparrow_{{\rm LL},p}$. On the lattice the square roots are replaced by the lattice energy $\varepsilon^{\rm lat}_{\rm LL}$ of check GS-16 at the shifted momenta. This is exact: the link matrix $\mathsf U^{\rm link}_{{\rm LL},1}=\exp(-i\ell^{\rm x}_{\rm LL}\mathsf A_{{\rm LL},1})$ multiplies the plane wave at the next site by $\exp(-i\ell^{\rm x}_{\rm LL}\mathsf A^\uparrow_{{\rm LL},1})$, which shifts the momentum in the differences of the lattice operator by exactly $\mathsf A^\uparrow_{{\rm LL},1}$.*)

(* ::Text:: *)
(***What it shows.** The space components of a potential shift the momentum (minimal coupling): the plane wave with the label $q^1_{\rm LL}$ has its lowest energy at $q^1_{\rm LL}=\mathsf A^\uparrow_{{\rm LL},1}$. The time component shifts the energy, exactly as a change of the chemical potential would. In infinite space a gauge transformation removes both shifts. On a ring the gauge function would have to be periodic, and it is periodic only if $\exp(i\mathsf A_{{\rm LL},1}n^{\rm x}_{\rm LL}\ell^{\rm x}_{\rm LL})$ is the unit matrix. This matrix is the product of the link matrices around the ring (a Wilson loop), which no gauge transformation changes; if it is not the unit matrix, the ring encloses a flux, and the energies at the allowed momenta are truly shifted (the Aharonov-Bohm effect). Check AS-1 uses a ring of 32 sites ($\ell^{\rm x}_{\rm LL}=0.5$, circumference 16, $m_{\rm LL}=0.4$, $\mu_{\rm LL}=0.2$) with the potentials $A^3_{{\rm LL},1}=0.8$ and $A^{\rm F}_{{\rm LL},1}=0.6$ in the links and $A^3_{{\rm LL},0}=0.5$ and $A^{\rm F}_{{\rm LL},0}=0.3$ in the potential matrix, so that $\mathsf A^\uparrow_{{\rm LL},1}=0.55$, $\mathsf A^\downarrow_{{\rm LL},1}=-0.01$, $\mathsf A^\uparrow_{{\rm LL},0}=0.31$ and $\mathsf A^\downarrow_{{\rm LL},0}=-0.04$. For every allowed momentum it compares the 16 eigenvalues of `t2planeMatrix` with the formula, restricts the particle block (the rows and columns 1 to 8) to each isospin component and compares its four eigenvalues with the formula, and checks that the particle block does not connect the two isospin components. Figure 3(a) shows the energies of the particle block.*)

(* ::Input:: *)
(* AS-1: the potentials Av^3_1, AF_1 (in the links), Av^3_0, AF_0 (in the potential matrix), the chemical potential;
   a chain of 32 sites with the spacing 0.5 *)
{as1Av31, as1AF1, as1Av30, as1AF0, as1Mu} = {0.8, 0.6, 0.5, 0.3, 0.2};
{as1N, as1Ell} = {32, 0.5};
as1Lat = <|"n" -> {as1N, 1, 1}, "ell" -> as1Ell, "periodic" -> {True, False, False}, "wilson" -> 1.0, "mass" -> mass, "mu" -> as1Mu|>;
as1Fields = t2trivialFields[as1N];
Do[as1Fields[["links", site, 1]] = t2link[gv, gf, {0., 0., as1Av31}, as1AF1, as1Ell];
  as1Fields[["v0", site]] = t2potential[gv, gf, {0., 0., as1Av30}, as1AF0], {site, as1N}];
(* Acal_p of the isospin component with the charge qq = +1/2 (up) or -1/2 (down): p = 1 and p = 0 *)
as1Acal1[qq_] := gf as1AF1 + qq gv as1Av31;
as1Acal0[qq_] := gf as1AF0 + qq gv as1Av30;
as1Dev = 0.;
(* the rows {q, Q_LL, E_qp} of the points of Figure 3(a), in the order of the original data file *)
as1LatticeRows = {};
Do[
  Module[{q = t2chainMomentum[kq, as1N, as1Ell], m16, ev, expect, pblock},
   m16 = t2planeMatrix[as1Lat, as1Fields, q];
   (* all 16 eigenvalues: particles +-eps_lat(q - Acal_1) - Acal_0 - mu, holes -(+-eps_lat(q + Acal_1) - Acal_0 - mu), each twice *)
   ev = First[t2eigh[m16]];
   expect = Sort[Flatten[Table[
       With[{ep = t2epsLat[{q - as1Acal1[qq], 0., 0.}, as1Ell, mass, 1.0, asChainActive],
         eh = t2epsLat[{q + as1Acal1[qq], 0., 0.}, as1Ell, mass, 1.0, asChainActive]},
        Table[{ep - as1Acal0[qq] - as1Mu, -ep - as1Acal0[qq] - as1Mu, eh + as1Acal0[qq] + as1Mu, -eh + as1Acal0[qq] + as1Mu}, {2}]],
       {qq, {0.5, -0.5}}]]];
   as1Dev = Max[as1Dev, Max[Abs[ev - expect]]];
   (* the particle block (rows and columns 1..8), restricted to each isospin component *)
   pblock = m16[[1 ;; 8, 1 ;; 8]];
   Do[
    Module[{idx = If[qq > 0, asUpIndex, asDownIndex], evq, ep, ex},
     evq = First[t2eigh[pblock[[idx, idx]]]];
     ep = t2epsLat[{q - as1Acal1[qq], 0., 0.}, as1Ell, mass, 1.0, asChainActive];
     ex = Sort[{-ep - as1Acal0[qq] - as1Mu, -ep - as1Acal0[qq] - as1Mu, ep - as1Acal0[qq] - as1Mu, ep - as1Acal0[qq] - as1Mu}];
     as1Dev = Max[as1Dev, Max[Abs[evq - ex]]];
     Do[AppendTo[as1LatticeRows, {q, qq, evq[[i]]}], {i, 4}]],
    {qq, {0.5, -0.5}}];
   (* the particle block does not mix the two isospin components (the potentials point along tau_3) *)
   as1Dev = Max[as1Dev, Max[Abs[pblock[[asUpIndex, asDownIndex]]]]]],
  {kq, 0, as1N - 1}];
(* the rows {q, Q_LL, branch, E_qp} of the curves of Figure 3(a): the formula on 401 momenta; branch 1: the positive
   square root, -1: the negative one *)
as1CurveRows = Flatten[Table[
    With[{q = -Pi/as1Ell + 2 Pi/as1Ell i/400.},
     Flatten[Table[With[{ep = t2epsLat[{q - as1Acal1[qq], 0., 0.}, as1Ell, mass, 1.0, asChainActive]},
        {{q, qq, 1, ep - as1Acal0[qq] - as1Mu}, {q, qq, -1, -ep - as1Acal0[qq] - as1Mu}}], {qq, {0.5, -0.5}}], 1]],
    {i, 0, 400}], 1];
SL31Check["AS-1", "free spinor charges in constant gauge potentials (case A): on a periodic chain of 32 sites along x_LL^1 (dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, g_v = 0.7, g_F = 0.45) with the constant potentials Av_LL^3_1 = 0.8 and AF_LL_1 = 0.6 (in the links) and Av_LL^3_0 = 0.5 and AF_LL_0 = 0.3 (in the potential matrix), and no pair field, the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum q are, for each isospin component (Q_LL = +1/2 with Acal_up_p = g_F AF_LL_p + g_v Av_LL^3_p/2, Q_LL = -1/2 with Acal_down_p = g_F AF_LL_p - g_v Av_LL^3_p/2), +-eps_lat(q - Acal_1) - Acal_0 - mu (particles) and -(+-eps_lat(q + Acal_1) - Acal_0 - mu) (holes), each twice: the potential of the space direction shifts the momentum (minimal coupling q -> q - Acal_1) and the potential of the time direction shifts the energy; the particle block does not mix the two isospin components, and its restriction to each component has the eigenvalues +-eps_lat(q - Acal_1) - Acal_0 - mu (tolerance 1e-12)",
  as1Dev < 1*^-12,
  Row[{"largest deviation ", asSci[as1Dev], "; Acal_up_1 = ", asFix[as1Acal1[0.5], 4], ", Acal_down_1 = ", asFix[as1Acal1[-0.5], 4], ", Acal_up_0 = ", asFix[as1Acal0[0.5], 4],
    ", Acal_down_0 = ", asFix[as1Acal0[-0.5], 4], " (original: 9.8e-15; 0.5500, -0.0100, 0.3100, -0.0400)"}]]

(* ::Text:: *)
(***The isospin turns in a constant isospin potential (check AS-2).** Now only $A^1_{{\rm LL},0}$ differs from zero, and it is constant: $\mathsf A_{{\rm LL},0}=\tfrac12g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}\tau^{\rm LL}_1$. This matrix acts on the isospin index and commutes with $h^\nabla_{\rm LL}$, so the evolution of the particle part is a product, $\exp\big(-i(h^\nabla_{\rm LL}-\mathsf A_{{\rm LL},0}-\mu_{\rm LL})x^0_{\rm LL}\big)=\exp\big(-i(h^\nabla_{\rm LL}-\mu_{\rm LL})x^0_{\rm LL}\big)\exp(i\mathsf A_{{\rm LL},0}x^0_{\rm LL})$. Because $(\tau^{\rm LL}_1)^2=\sigma^{\rm LL}_0$, the second factor is $\exp(i\mathsf A_{{\rm LL},0}x^0_{\rm LL})=\cos(\tfrac12\omega^{\rm pr}_{\rm LL}x^0_{\rm LL})\,\sigma^{\rm LL}_0+i\sin(\tfrac12\omega^{\rm pr}_{\rm LL}x^0_{\rm LL})\,\tau^{\rm LL}_1$ with $\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}$. A particle that starts as a plane wave of positive energy with isospin up, $\chi^{\rm pw}_{\rm LL}$ times the isospin vector $(1,0)^T$, therefore has at the time $x^0_{\rm LL}$ the phase factor $\exp(-i(\varepsilon^{\rm lat}_{\rm LL}-\mu_{\rm LL})x^0_{\rm LL})$ and the isospin vector $\big(\cos(\tfrac12\omega^{\rm pr}_{\rm LL}x^0_{\rm LL}),\,i\sin(\tfrac12\omega^{\rm pr}_{\rm LL}x^0_{\rm LL})\big)^T$, and its probability of isospin up is*)

(* ::DisplayFormula:: *)
(*P^\uparrow_{\rm LL}(x^0_{\rm LL})=\cos^2\big(\tfrac12\omega^{\rm pr}_{\rm LL}x^0_{\rm LL}\big),\qquad\omega^{\rm pr}_{\rm LL}=g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}*)

(* ::Text:: *)
(*The isospin turns about the direction of the potential with the angular frequency $\omega^{\rm pr}_{\rm LL}$. The potential has zero field strength, and the rotation is exactly the gauge transformation $\mathsf U_{\rm LL}=\exp(i\mathsf A_{{\rm LL},0}x^0_{\rm LL})$ that removes it: in the gauge without the potential the isospin of the same particle stays up. The isospin direction of a single mode is therefore not gauge invariant; only its direction relative to other objects that carry isospin is. For the solvers this solution is a sharp test of the isospin structure of the potential term. Check AS-2 takes a ring of 16 sites ($\ell^{\rm x}_{\rm LL}=0.5$, $\mu_{\rm LL}=0.2$, $A^1_{{\rm LL},0}=1.5$, so $\omega^{\rm pr}_{\rm LL}=1.05$) and the plane wave of the allowed momentum number 2 with isospin up, integrates the lattice equations to the 101 output times $x^0_{\rm LL}=0,0.16,\dots,16$ (the interval between the output times is $\ell^{\rm t}_{\rm LL}=0.16$), and compares the whole state with the exact solution. The original program integrates with CVODE ($r^{\rm tol}_{\rm LL}=10^{-12}$, $a^{\rm tol}_{\rm LL}=10^{-14}$) and requires a relative deviation $\delta^{\rm rel}_{\rm LL}$ below $10^{-9}$. The notebook integrates with IDA and the same tolerances (function `t2evolveIDAChecked` of chapter II.1.5) and keeps this requirement. It requires in addition that the states at the output times are the cubic Hermite polynomials of the stored values and derivatives, and that the largest deviation is at most the bound that follows from the measured local errors: the sum of the exact local errors $\delta^{\rm loc}_{\rm LL}$ of all internal steps bounds the deviation at the end of every step (the evolution is unitary), and chapter II.1.5 carries such a bound to the output times, with this sum in place of $B^{\rm err}_{\rm LL}$. The largest ratio of a local error to $B^{\rm loc}_{\rm LL}$ is printed for information; it is not required to stay below 1, because IDA tests its own estimate of the error, in a weighted norm that averages over all real unknowns, and half of the real unknowns of this state (the hole components) are exactly zero at all times. Figure 3(b) shows the probability of isospin up.*)

(* ::Input:: *)
(* AS-2: a chain of 16 sites; the constant isospin potential Av^1_0 = 1.5 along tau_1 in the potential matrix *)
{as2N, as2Ell, as2Mu, as2Av10} = {16, 0.5, 0.2, 1.5};
as2Lat = <|"n" -> {as2N, 1, 1}, "ell" -> as2Ell, "periodic" -> {True, False, False}, "wilson" -> 1.0, "mass" -> mass, "mu" -> as2Mu|>;
as2Fields = t2trivialFields[as2N];
Do[as2Fields[["v0", site]] = t2potential[gv, gf, {as2Av10, 0., 0.}, 0.], {site, as2N}];
(* the allowed momentum number 2, and the eigenvector of the largest eigenvalue eps_lat of the 4 x 4 matrix of the
   free lattice operator on its plane wave: the Dirac spinor of chi_pw *)
as2Q = t2chainMomentum[2, as2N, as2Ell];
{as2E4, as2V4} = t2eigh[t2hLatPlane[{as2Q, 0., 0.}, as2Ell, mass, 1.0, asChainActive]];
as2Eps = as2E4[[4]];
as2Chi4 = as2V4[[All, 4]];
as2OmegaPr = gv as2Av10;
(* the exact solution at the time x0: on every site the eight particle components chi_pw (x) (cos, i sin) times the
   phase factors of the time and of the plane wave; the hole components are zero *)
as2Exact[x0_?NumericQ] := Flatten[Table[
    With[{phaseSite = Exp[-I (as2Eps - as2Mu) x0] Exp[I as2Q as2Ell site]},
     Join[Flatten[Table[{as2Chi4[[a]] phaseSite Cos[0.5 as2OmegaPr x0], as2Chi4[[a]] phaseSite I Sin[0.5 as2OmegaPr x0]}, {a, 4}]], ConstantArray[0. I, 8]]],
    {site, 0, as2N - 1}]];
(* the 101 output times 0, 0.16, ..., 16; the integration by IDA with the tolerances of the original program *)
as2Touts = 0.16 Range[0, 100];
{as2Rtol, as2Atol} = {1.*^-12, 1.*^-14};
as2H = t2dense[as2Lat, as2Fields];
as2Run = t2evolveIDAChecked[as2H, as2Exact[0.], as2Touts, as2Rtol, as2Atol];
as2ExactStates = Map[as2Exact, as2Touts];
(* the norms of the deviations of the states from the exact states, and the largest relative deviation *)
as2Deviations = If[AssociationQ[as2Run], MapThread[Norm[#1 - #2] &, {as2Run["states"], as2ExactStates}], {Infinity}];
as2Norm = Norm[as2Exact[0.]];
as2Dev = If[AssociationQ[as2Run], Max[as2Deviations/Map[Norm, as2ExactStates]], Infinity];
(* the probability of isospin up of a state: the part of its squared norm in the particle components 1, 3, 5, 7 *)
as2Pup[w_] := Total[Abs[Flatten[Partition[w, 16][[All, asUpIndex]]]]^2]/Total[Abs[w]^2];
(* the rows {x^0, P_up of the integration, P_up exact} of Figure 3(b) *)
as2PrecessionRows = If[AssociationQ[as2Run],
   Table[{as2Touts[[i]], as2Pup[as2Run[["states", i]]], Cos[0.5 as2OmegaPr as2Touts[[i]]]^2}, {i, Length[as2Touts]}], {}];
SL31Check["AS-2", "isospin precession in a constant isospin potential (case A): in the potential Av_LL^1_0 = 1.5 (along tau_1, no other field) the positive-energy lattice plane wave chi_pw (x) (isospin up) evolves as exp(-i (eps_lat - mu) x_LL^0) chi_pw (x) (cos(omega_pr x_LL^0/2), i sin(omega_pr x_LL^0/2)) with omega_pr = g_v Av_LL^1_0, so that P_up = cos^2(omega_pr x_LL^0/2); the integration of the lattice equations by IDA (SUNDIALS, through NDSolve with Method -> \"IDA\", in place of CVODE of the original program; rtol = 1e-12, atol = 1e-14; periodic chain of 16 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.2, momentum 2 pi 2/(16 dx)) agrees with this exact solution at the 101 output times x_LL^0 = 0, 0.16, ..., 16: (1) the relative deviation of the whole state stays below 1e-9, the tolerance of the original check; (2) the states at the output times are the cubic Hermite polynomials of the stored values and derivatives (tolerance 1e-12), and the largest deviation is at most the bound that follows from the measured exact local errors of the internal steps, (1 + h_IDA E_max/4) (sum of local_error) + h_IDA deriv_residual/4 + (h_IDA E_max)^4/384 (norm of the state); the numbers of steps and of evaluations of IDA and the largest ratio local_error/budget_step, with budget_step = rtol (norm of the state) + atol sqrt(512), are stated",
  AssociationQ[as2Run] && as2Dev < 1*^-9 && as2Run["hermite"] < 1*^-12 && Max[as2Deviations] <= as2Run["bsum"],
  If[AssociationQ[as2Run],
   Row[{"largest relative deviation ", asSci[as2Dev], " <= bound/(norm of the state) ", asSci[as2Run["bsum"]/as2Norm], "; (sum of local_error)/(norm of the state) ", asSci[as2Run["sumloc"]/as2Norm],
     "; largest local_error/budget_step ", asSci[as2Run["ratio"]],
     "; IDA: N_steps ", as2Run["steps"], ", N_evals ", as2Run["evaluations"], ", largest h_IDA ", asSci[as2Run["hmax"]], ", largest deriv_residual ", asSci[as2Run["residual"]],
     ", Hermite polynomials within ", asSci[as2Run["hermite"]], ", E_max ", asFix[as2Run["emax"], 4], "; omega_pr = ", asFix[as2OmegaPr, 4], ", eps_lat = ", asFix[as2Eps, 6],
     " (original program, CVODE: largest relative deviation 1.3e-11, 1013 steps, 1494 evaluations of the right side; omega_pr = 1.0500, eps_lat = 1.723884)"}],
   "the integration by IDA failed"]]

(* ::Subsection:: *)
(*Case B: the uniform polar superfluid (checks AS-3 to AS-6)*)

(* ::Text:: *)
(***The fields.** No gauge fields, and a uniform pair field along the third isospin direction, $\vec d_{\rm LL}=(0,0,\Delta_{\rm LL})$ with a real $\Delta_{\rm LL}>0$ (the polar state). Then $\mathsf P_{\rm LL}=\Delta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$ and $\mathsf P_{\rm LL}^\dagger=\Delta_{\rm LL}\mathsf K^3_{\rm LL}$. The pair matrix is $\mathsf K^3_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_3$. With $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}=\mathrm{diag}(-\mathsf E_{\rm LL},-\mathsf E_{\rm LL})$ and $\mathsf E_{\rm LL}\tau^{\rm LL}_3=-\tau^{\rm LL}_1$ it is $\mathsf K^3_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})\otimes\tau^{\rm LL}_1$: a real matrix with $(\mathsf K^3_{\rm LL})^T=(\mathsf K^3_{\rm LL})^\dagger=-\mathsf K^3_{\rm LL}$ and $(\mathsf K^3_{\rm LL})^2=-\mathsf 1_8$.*)

(* ::Text:: *)
(***The key identity.** For the matrix $\mathsf h_{\rm LL}(\vec q_{\rm LL})=\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ of a plane wave,*)

(* ::DisplayFormula:: *)
(*\mathsf K^3_{\rm LL}\,\mathsf h_{\rm LL}(\vec q_{\rm LL})=\mathsf h_{\rm LL}(-\vec q_{\rm LL})^{*}\,\mathsf K^3_{\rm LL}*)

(* ::Text:: *)
(*Proof: with $\alpha^j_{\rm LL}=\mathrm{diag}(-\sigma^{\rm LL}_j,\sigma^{\rm LL}_j)$, the Dirac factor $\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})$ of $\mathsf K^3_{\rm LL}$, and the relation $\mathsf E_{\rm LL}\sigma^{\rm LL}_j=-(\sigma^{\rm LL}_j)^{*}\mathsf E_{\rm LL}$ (for $j=1$ and $j=3$ the Pauli matrix is real and anticommutes with $\mathsf E_{\rm LL}=i\sigma^{\rm LL}_2$; for $j=2$ it is imaginary and commutes with it), one finds $\mathsf K^3_{\rm LL}\alpha^j_{\rm LL}=-(\alpha^j_{\rm LL})^{*}\mathsf K^3_{\rm LL}$. The matrix $\beta_{\rm LL}$ is real and exchanges the two Weyl blocks, whose Dirac factors in $\mathsf K^3_{\rm LL}$ are equal, so $\mathsf K^3_{\rm LL}\beta_{\rm LL}=\beta_{\rm LL}\mathsf K^3_{\rm LL}$. Inserted in $\mathsf h_{\rm LL}$, the two relations give the identity. It holds on the lattice as well, where $\sin(q^j_{\rm LL}\ell^{\rm x}_{\rm LL})/\ell^{\rm x}_{\rm LL}$ (an odd function of $q^j_{\rm LL}$) stands in place of $q^j_{\rm LL}$ and the Wilson term (an even function) is added to the mass.*)

(* ::Text:: *)
(***The Bogoliubov modes.** On plane waves the operator $\mathcal H^{\rm DB}_{\rm LL}$ is a $16\times16$ matrix with the blocks $\mathsf h_{\rm LL}(\vec q_{\rm LL})-\mu_{\rm LL}$ (upper left), $\Delta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$ (upper right), $\Delta_{\rm LL}\mathsf K^3_{\rm LL}$ (lower left) and $-(\mathsf h_{\rm LL}(-\vec q_{\rm LL})^{*}-\mu_{\rm LL})$ (lower right), as in check GS-8. Let $\chi^{\rm pw}_{\rm LL}$ be a normalized eigenvector of $\mathsf h_{\rm LL}(\vec q_{\rm LL})$ with the eigenvalue $+\varepsilon_{\rm LL}$ or $-\varepsilon_{\rm LL}$, where $\varepsilon_{\rm LL}=(q_{\rm LL}^2+m_{\rm LL}^2)^{1/2}$. By the key identity, $\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL}$ is an eigenvector of $\mathsf h_{\rm LL}(-\vec q_{\rm LL})^{*}$ with the same eigenvalue; and because $(\mathsf K^3_{\rm LL})^\dagger\mathsf K^3_{\rm LL}=\mathsf 1_8$, the operator maps the two vectors $(\chi^{\rm pw}_{\rm LL},0)$ and $(0,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL})$ to combinations of the same two vectors, with the matrix*)

(* ::DisplayFormula:: *)
(*\begin{pmatrix}\pm\varepsilon_{\rm LL}-\mu_{\rm LL}&\Delta_{\rm LL}\\\Delta_{\rm LL}&-(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})\end{pmatrix}*)

(* ::Text:: *)
(*The square of this matrix is $(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})^2+\Delta_{\rm LL}^2$ times the unit matrix. Its eigenvalues are therefore $\pm E^-_{\rm LL}$ for the upper sign (from the eigenvalue $+\varepsilon_{\rm LL}$) and $\pm E^+_{\rm LL}$ for the lower sign (from $-\varepsilon_{\rm LL}$), with $E^\pm_{\rm LL}=\big((\varepsilon_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2\big)^{1/2}$, each four times (the eigenvalue $+\varepsilon_{\rm LL}$ of the $8\times8$ matrix $\mathsf h_{\rm LL}$ has four eigenvectors, and so has $-\varepsilon_{\rm LL}$). The eigenvector of the positive eigenvalue is $\big(\cos\vartheta^{\rm B}_{\rm LL}\,\chi^{\rm pw}_{\rm LL},\ \sin\vartheta^{\rm B}_{\rm LL}\,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL}\big)$ with the Bogoliubov angle $\vartheta^{\rm B}_{\rm LL}$ given by*)

(* ::DisplayFormula:: *)
(*\cos^2\vartheta^{\rm B}_{\rm LL}=\tfrac12\Big(1+\frac{\pm\varepsilon_{\rm LL}-\mu_{\rm LL}}{E^\mp_{\rm LL}}\Big),\qquad\sin^2\vartheta^{\rm B}_{\rm LL}=\tfrac12\Big(1-\frac{\pm\varepsilon_{\rm LL}-\mu_{\rm LL}}{E^\mp_{\rm LL}}\Big)*)

(* ::Text:: *)
(*(inserted in the matrix, the first row gives $\tan\vartheta^{\rm B}_{\rm LL}=\big(E^\mp_{\rm LL}-(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})\big)/\Delta_{\rm LL}$ and the second $\tan\vartheta^{\rm B}_{\rm LL}=\Delta_{\rm LL}/\big(E^\mp_{\rm LL}+(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})\big)$, and the two agree because $(E^\mp_{\rm LL})^2-(\pm\varepsilon_{\rm LL}-\mu_{\rm LL})^2=\Delta_{\rm LL}^2$). These modes are the quasi-particles of the superfluid: superpositions of a particle and a hole with the weights $\cos^2\vartheta^{\rm B}_{\rm LL}$ and $\sin^2\vartheta^{\rm B}_{\rm LL}$. The branch $E^-_{\rm LL}$ has its minimum $\Delta_{\rm LL}$ at the Fermi momentum $k^{\rm F}_{\rm LL}$, where $\varepsilon_{\rm LL}=\mu_{\rm LL}$: $\Delta_{\rm LL}$ is the energy gap of the superfluid. The branch $E^+_{\rm LL}$ belongs to the states of negative energy (the antiparticles). Check AS-3 verifies the key identity, the eigenvectors and their energies for 50 random momenta, chemical potentials and gaps, and then the 16 eigenvalues of the lattice operator on the plane wave of every allowed momentum of a ring of 32 sites ($\ell^{\rm x}_{\rm LL}=0.5$, $m_{\rm LL}=0.4$, $\mu_{\rm LL}=0.6$, $\Delta_{\rm LL}=0.3$), where $\varepsilon^{\rm lat}_{\rm LL}$ stands in place of $\varepsilon_{\rm LL}$ (Figure 4(a)).*)

(* ::Input:: *)
(* AS-3, the continuum: 50 random momenta, chemical potentials and gaps (the random numbers in the order of t2_analytic) *)
as3Dev = 0.;
Do[
  Module[{q, chem, gap, hq8, hmqc, pair, hbdg, ev, vecs, energy},
   q = Table[2. t2Unit[], {3}];
   chem = Abs[t2Unit[]];
   gap = 0.1 + 0.9 Abs[t2Unit[]];
   hq8 = t2h8[q, mass];
   hmqc = Conjugate[t2h8[-q, mass]];
   (* the identity K^3 h(q) = conj(h(-q)) K^3 *)
   as3Dev = Max[as3Dev, fro[asK3 . hq8 - hmqc . asK3]];
   (* H_BdG(q) = [[h(q) - mu, P], [P^dag, -(conj(h(-q)) - mu)]] with P = Delta (K^3)^dag *)
   pair = t2pairPotential[{0, 0, gap}];
   hbdg = ArrayFlatten[{{hq8 - chem IdentityMatrix[8], pair}, {ConjugateTranspose[pair], -(hmqc - chem IdentityMatrix[8])}}];
   {ev, vecs} = t2eigh[hq8];
   energy = Sqrt[q . q + mass^2];
   (* the eigenvector number 8 has the eigenvalue +eps, the number 1 the eigenvalue -eps *)
   Do[
    Module[{col = colSign[[1]], sg = colSign[[2]], chi, kchi, xe, e16, cth, sth, v16, target},
     as3Dev = Max[as3Dev, Abs[ev[[col]] - sg energy]];
     chi = vecs[[All, col]];
     kchi = asK3 . chi;
     xe = sg energy - chem;
     e16 = Sqrt[xe^2 + gap^2];
     (* cos theta_B and sin theta_B *)
     cth = Sqrt[0.5 (1 + xe/e16)]; sth = Sqrt[0.5 (1 - xe/e16)];
     v16 = Join[cth chi, sth kchi];
     as3Dev = Max[as3Dev, Norm[hbdg . v16 - e16 v16]];
     target = If[sg > 0, Sqrt[(energy - chem)^2 + gap^2], Sqrt[(energy + chem)^2 + gap^2]];
     as3Dev = Max[as3Dev, Abs[e16 - target]]],
    {colSign, {{8, 1.}, {1, -1.}}}]],
  {50}];
(* the lattice: the parameters of case B (the spacing, the chemical potential, the gap), a chain of 32 sites with the
   uniform pair field (0, 0, Delta) *)
{asBEll, asBMu, asBDelta} = {0.5, 0.6, 0.3};
as3N = 32;
as3Lat = <|"n" -> {as3N, 1, 1}, "ell" -> asBEll, "periodic" -> {True, False, False}, "wilson" -> 1.0, "mass" -> mass, "mu" -> asBMu|>;
as3Fields = t2trivialFields[as3N];
Do[as3Fields[["d", site]] = {0., 0., asBDelta}, {site, as3N}];
as3DevLat = 0.;
(* the rows {q, E_qp} of the points of Figure 4(a): the positive eigenvalues, in ascending order for every momentum *)
as3LatticeRows = {};
Do[
  Module[{q = t2chainMomentum[kq, as3N, asBEll], ev, el, em, ep, ex},
   ev = First[t2eigh[t2planeMatrix[as3Lat, as3Fields, q]]];
   el = t2epsLat[{q, 0., 0.}, asBEll, mass, 1.0, asChainActive];
   em = Sqrt[(el - asBMu)^2 + asBDelta^2];
   ep = Sqrt[(el + asBMu)^2 + asBDelta^2];
   ex = Sort[Flatten[Table[{em, -em, ep, -ep}, {4}]]];
   as3DevLat = Max[as3DevLat, Max[Abs[ev - ex]]];
   Do[If[ev[[i]] > 0, AppendTo[as3LatticeRows, {q, ev[[i]]}]], {i, 16}]],
  {kq, 0, as3N - 1}];
(* the rows {q, eps_lat, E^-, E^+} of the curves of Figure 4(a), on 401 momenta *)
as3CurveRows = Table[
   With[{q = -Pi/asBEll + 2 Pi/asBEll i/400.},
    With[{el = t2epsLat[{q, 0., 0.}, asBEll, mass, 1.0, asChainActive]},
     {q, el, Sqrt[(el - asBMu)^2 + asBDelta^2], Sqrt[(el + asBMu)^2 + asBDelta^2]}]],
   {i, 0, 400}];
SL31Check["AS-3", "the Bogoliubov modes of the uniform polar superfluid dvec = (0, 0, Delta) (case B): Kch_3 h(q) = conj(h(-q)) Kch_3, so that for an eigenvector chi_pw of h(q) with the energy +eps or -eps the hole vector Kch_3 chi_pw is an eigenvector of conj(h(-q)) with the same energy; Ppair = Delta Kch_3^dag couples (chi_pw, 0) only to (0, Kch_3 chi_pw), and (cos theta_B chi_pw, sin theta_B Kch_3 chi_pw) with cos^2 theta_B = (1 + (+-eps - mu)/E)/2, sin^2 theta_B = (1 - (+-eps - mu)/E)/2, E = ((+-eps - mu)^2 + Delta^2)^(1/2), is an eigenvector of H_BdG(q) with the energy E, which is E^- for the positive branch +eps and E^+ for the negative branch -eps (50 random momenta, chemical potentials and gaps, m = 0.4; tolerance 1e-12); on the lattice (periodic chain of 32 sites, dx = 0.5, r_W = 1, m = 0.4, mu = 0.6, Delta = 0.3) the 16 eigenvalues of the lattice operator on the plane wave of each allowed momentum are +-E^- and +-E^+ with eps_lat in place of eps, each four times (tolerance 1e-12)",
  as3Dev < 1*^-12 && as3DevLat < 1*^-12,
  Row[{"largest deviations ", asSci[as3Dev], " (continuum), ", asSci[as3DevLat], " (lattice) (original: 6.7e-15, 1.2e-14)"}]]

(* ::Text:: *)
(***A particle injected into the superfluid oscillates into a hole (check AS-4).** Start with the particle $(\chi^{\rm pw}_{\rm LL},0)$ of the positive energy $\varepsilon_{\rm LL}$. Its evolution stays in the plane of the two vectors $(\chi^{\rm pw}_{\rm LL},0)$ and $(0,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL})$, in which the operator is the matrix displayed above with the upper sign. The square of that matrix is $(E^-_{\rm LL})^2$ times the unit matrix, so its exponential with the factor $-ix^0_{\rm LL}$ is $\cos(E^-_{\rm LL}x^0_{\rm LL})$ times the unit matrix minus $i\sin(E^-_{\rm LL}x^0_{\rm LL})/E^-_{\rm LL}$ times the matrix itself, and the mode is*)

(* ::DisplayFormula:: *)
(*\psi^{\rm p}_{\rm LL}=\Big(\cos(E^-_{\rm LL}x^0_{\rm LL})-i\,\frac{\varepsilon_{\rm LL}-\mu_{\rm LL}}{E^-_{\rm LL}}\sin(E^-_{\rm LL}x^0_{\rm LL})\Big)\chi^{\rm pw}_{\rm LL}*)

(* ::DisplayFormula:: *)
(*\psi^{\rm h}_{\rm LL}=-i\,\frac{\Delta_{\rm LL}}{E^-_{\rm LL}}\sin(E^-_{\rm LL}x^0_{\rm LL})\,\mathsf K^3_{\rm LL}\chi^{\rm pw}_{\rm LL}*)

(* ::Text:: *)
(*(both times the plane wave). The probability of the hole is*)

(* ::DisplayFormula:: *)
(*P^{\rm h}_{\rm LL}(x^0_{\rm LL})=\Big(\frac{\Delta_{\rm LL}}{E^-_{\rm LL}}\Big)^2\sin^2(E^-_{\rm LL}x^0_{\rm LL})*)

(* ::Text:: *)
(*A bare particle is not a stationary state of the superfluid: the pair field converts it into a hole and back, with the angular frequency $2E^-_{\rm LL}$ and the largest hole probability $(\Delta_{\rm LL}/E^-_{\rm LL})^2$, which is 1 at the Fermi momentum and small far from it (the Rabi oscillation of a system with two states). Check AS-4 takes a ring of 16 sites with the parameters and the pair field of AS-3 and the positive-energy plane wave of isospin up at the allowed momentum number 1, integrates the lattice equations to the 101 output times $x^0_{\rm LL}=0,0.2,\dots,20$ ($\ell^{\rm t}_{\rm LL}=0.2$), and compares the whole state with this solution, in which $\varepsilon^{\rm lat}_{\rm LL}$ stands in place of $\varepsilon_{\rm LL}$. The integrator, its tolerances and the requirements are those of check AS-2 (here the particle components with isospin down and the hole components with isospin up are exactly zero at all times, again half of the real unknowns). Figure 4(b) shows the probability of the hole.*)

(* ::Input:: *)
(* AS-4: a chain of 16 sites with the uniform pair field (0, 0, Delta) and the parameters of case B *)
as4N = 16;
as4Lat = <|"n" -> {as4N, 1, 1}, "ell" -> asBEll, "periodic" -> {True, False, False}, "wilson" -> 1.0, "mass" -> mass, "mu" -> asBMu|>;
as4Fields = t2trivialFields[as4N];
Do[as4Fields[["d", site]] = {0., 0., asBDelta}, {site, as4N}];
(* the allowed momentum number 1; chi_pw: the Dirac spinor of the largest eigenvalue eps_lat with isospin up *)
as4Q = t2chainMomentum[1, as4N, asBEll];
{as4E4, as4V4} = t2eigh[t2hLatPlane[{as4Q, 0., 0.}, asBEll, mass, 1.0, asChainActive]];
as4Eps = as4E4[[4]];
as4Chi = Flatten[Table[{as4V4[[a, 4]], 0.}, {a, 4}]];
as4KChi = asK3 . as4Chi;
(* eps_lat - mu and the quasi-particle energy E^- *)
as4X = as4Eps - asBMu;
as4E = Sqrt[as4X^2 + asBDelta^2];
(* the exact solution at the time x0: the particle part and the hole part, times the plane wave *)
as4Exact[x0_?NumericQ] := With[{cp = Cos[as4E x0] - I (as4X/as4E) Sin[as4E x0], ch = -I (asBDelta/as4E) Sin[as4E x0]},
   Flatten[Table[With[{phaseSite = Exp[I as4Q asBEll site]}, Join[cp phaseSite as4Chi, ch phaseSite as4KChi]], {site, 0, as4N - 1}]]];
(* the 101 output times 0, 0.2, ..., 20; the integration by IDA with the tolerances of AS-2 *)
as4Touts = 0.2 Range[0, 100];
as4H = t2dense[as4Lat, as4Fields];
as4Run = t2evolveIDAChecked[as4H, as4Exact[0.], as4Touts, as2Rtol, as2Atol];
as4ExactStates = Map[as4Exact, as4Touts];
as4Deviations = If[AssociationQ[as4Run], MapThread[Norm[#1 - #2] &, {as4Run["states"], as4ExactStates}], {Infinity}];
as4Norm = Norm[as4Exact[0.]];
as4Dev = If[AssociationQ[as4Run], Max[as4Deviations/Map[Norm, as4ExactStates]], Infinity];
(* the probability of the hole of a state: the part of its squared norm in the hole components 9..16 of every site *)
as4Phole[w_] := Total[Abs[Flatten[Partition[w, 16][[All, 9 ;; 16]]]]^2]/Total[Abs[w]^2];
(* the rows {x^0, P_hole of the integration, P_hole exact} of Figure 4(b) *)
as4RabiRows = If[AssociationQ[as4Run],
   Table[{as4Touts[[i]], as4Phole[as4Run[["states", i]]], (asBDelta/as4E)^2 Sin[as4E as4Touts[[i]]]^2}, {i, Length[as4Touts]}], {}];
SL31Check["AS-4", "the particle-hole (Rabi) oscillation of a particle injected into the uniform polar superfluid (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with the pair field dvec = (0, 0, 0.3), the positive-energy lattice plane wave chi_pw of isospin up at the momentum 2 pi/(16 dx) evolves as (cos(E^- x_LL^0) - i ((eps_lat - mu)/E^-) sin(E^- x_LL^0)) (chi_pw, 0) - i (Delta/E^-) sin(E^- x_LL^0) (0, Kch_3 chi_pw), with E^- = ((eps_lat - mu)^2 + Delta^2)^(1/2): the probability of the hole is P_hole = (Delta/E^-)^2 sin^2(E^- x_LL^0); the integration of the lattice equations by IDA (SUNDIALS, through NDSolve with Method -> \"IDA\", in place of CVODE of the original program; rtol = 1e-12, atol = 1e-14) agrees with this exact solution at the 101 output times x_LL^0 = 0, 0.2, ..., 20: (1) the relative deviation of the whole state stays below 1e-9, the tolerance of the original check; (2) the states at the output times are the cubic Hermite polynomials of the stored values and derivatives (tolerance 1e-12), and the largest deviation is at most the bound that follows from the measured exact local errors of the internal steps, (1 + h_IDA E_max/4) (sum of local_error) + h_IDA deriv_residual/4 + (h_IDA E_max)^4/384 (norm of the state); the numbers of steps and of evaluations of IDA and the largest ratio local_error/budget_step, with budget_step = rtol (norm of the state) + atol sqrt(512), are stated",
  AssociationQ[as4Run] && as4Dev < 1*^-9 && as4Run["hermite"] < 1*^-12 && Max[as4Deviations] <= as4Run["bsum"],
  If[AssociationQ[as4Run],
   Row[{"largest relative deviation ", asSci[as4Dev], " <= bound/(norm of the state) ", asSci[as4Run["bsum"]/as4Norm], "; (sum of local_error)/(norm of the state) ", asSci[as4Run["sumloc"]/as4Norm],
     "; largest local_error/budget_step ", asSci[as4Run["ratio"]],
     "; IDA: N_steps ", as4Run["steps"], ", N_evals ", as4Run["evaluations"], ", largest h_IDA ", asSci[as4Run["hmax"]], ", largest deriv_residual ", asSci[as4Run["residual"]],
     ", Hermite polynomials within ", asSci[as4Run["hermite"]], ", E_max ", asFix[as4Run["emax"], 4], "; eps_lat = ", asFix[as4Eps, 6], ", E^- = ", asFix[as4E, 6],
     ", largest P_hole = (Delta/E^-)^2 = ", asFix[(asBDelta/as4E)^2, 6],
     " (original program, CVODE: largest relative deviation 4.7e-10, 486 steps, 963 evaluations of the right side; eps_lat = 0.943799, E^- = 0.456287, largest P_hole = 0.432281)"}],
   "the integration by IDA failed"]]

(* ::Text:: *)
(***The gap equation in closed form (check AS-5).** The pair field of a stationary state must be the one that its own modes produce. For the uniform polar state this self-consistency condition is the gap equation: $1/G_{\rm LL}$ equals the integral of $(1/E^+_{\rm LL}+1/E^-_{\rm LL})\,d^3q_{\rm LL}/(2\pi)^3$ over the momenta with $q_{\rm LL}<\Lambda_{\rm LL}$ (check GS-9), with the coupling $G_{\rm LL}$ of the contact interaction and the momentum cutoff $\Lambda_{\rm LL}$. For massless spinor charges ($m_{\rm LL}=0$, $\varepsilon_{\rm LL}=q_{\rm LL}$) the integral can be done. The integrand depends only on $q_{\rm LL}$, so $d^3q_{\rm LL}/(2\pi)^3$ becomes $q_{\rm LL}^2\,dq_{\rm LL}/(2\pi^2)$, and one needs antiderivatives of $q_{\rm LL}^2/E^\pm_{\rm LL}$. They are $\tfrac12(q_{\rm LL}\mp3\mu_{\rm LL})E^\pm_{\rm LL}+(\mu_{\rm LL}^2-\tfrac12\Delta_{\rm LL}^2)\,\mathrm{arsinh}\big((q_{\rm LL}\pm\mu_{\rm LL})/\Delta_{\rm LL}\big)$, as differentiation shows: the derivative with respect to $q_{\rm LL}$, multiplied by $E^\pm_{\rm LL}$, is $\tfrac12(E^\pm_{\rm LL})^2+\tfrac12(q_{\rm LL}\mp3\mu_{\rm LL})(q_{\rm LL}\pm\mu_{\rm LL})+\mu_{\rm LL}^2-\tfrac12\Delta_{\rm LL}^2=q_{\rm LL}^2$. At $q_{\rm LL}=0$ the two antiderivatives cancel each other (there $E^+_{\rm LL}=E^-_{\rm LL}$, and the function arsinh is odd), so the integral is the sum of the two antiderivatives at $q_{\rm LL}=\Lambda_{\rm LL}$, divided by $2\pi^2$. The gap equation is therefore (the sum has two terms, one with the upper signs and one with the lower signs)*)

(* ::DisplayFormula:: *)
(*\frac1{G_{\rm LL}}=\frac1{4\pi^2}\sum_{\pm}\Big[(\Lambda_{\rm LL}\mp3\mu_{\rm LL})\,E^\pm_{\rm LL}(\Lambda_{\rm LL})+(2\mu_{\rm LL}^2-\Delta_{\rm LL}^2)\,\mathrm{arsinh}\frac{\Lambda_{\rm LL}\pm\mu_{\rm LL}}{\Delta_{\rm LL}}\Big]*)

(* ::Text:: *)
(*with $E^\pm_{\rm LL}(\Lambda_{\rm LL})=\big((\Lambda_{\rm LL}\pm\mu_{\rm LL})^2+\Delta_{\rm LL}^2\big)^{1/2}$. At $\mu_{\rm LL}=0$ the right side is $\big(\Lambda_{\rm LL}(\Lambda_{\rm LL}^2+\Delta_{\rm LL}^2)^{1/2}-\Delta_{\rm LL}^2\,\mathrm{arsinh}(\Lambda_{\rm LL}/\Delta_{\rm LL})\big)/(2\pi^2)$, which decreases from $\Lambda_{\rm LL}^2/(2\pi^2)$ as $\Delta_{\rm LL}$ grows: at $\mu_{\rm LL}=0$ a gap exists only above the critical coupling $G^{\rm crit}_{\rm LL}=2\pi^2/\Lambda_{\rm LL}^2$ (below it the vacuum stays normal, above it the vacuum itself condenses). For $\mu_{\rm LL}>0$ the arsinh terms grow like $\ln(1/\Delta_{\rm LL})$ as $\Delta_{\rm LL}\to0$, with the coefficient $2\mu_{\rm LL}^2$ (the Fermi surface), so a solution exists for every attractive coupling. With $E^\pm_{\rm LL}(\Lambda_{\rm LL})\to\Lambda_{\rm LL}\pm\mu_{\rm LL}$ and $\mathrm{arsinh}\big((\Lambda_{\rm LL}\pm\mu_{\rm LL})/\Delta_{\rm LL}\big)\to\ln\big(2(\Lambda_{\rm LL}\pm\mu_{\rm LL})/\Delta_{\rm LL}\big)$ the gap equation becomes $1/G_{\rm LL}=\big(\Lambda_{\rm LL}^2-3\mu_{\rm LL}^2+\mu_{\rm LL}^2\ln(4(\Lambda_{\rm LL}^2-\mu_{\rm LL}^2)/\Delta_{\rm LL}^2)\big)/(2\pi^2)$, whose solution is the weak-coupling gap of the theory of Bardeen, Cooper and Schrieffer (BCS),*)

(* ::DisplayFormula:: *)
(*\Delta_{\rm LL}=2(\Lambda_{\rm LL}^2-\mu_{\rm LL}^2)^{1/2}\exp(-\tfrac32)\exp\Big(-\frac{\pi^2}{\mu_{\rm LL}^2}\Big(\frac1{G_{\rm LL}}-\frac1{G^{\rm crit}_{\rm LL}}\Big)\Big)*)

(* ::Text:: *)
(*which is exponentially small in $1/G_{\rm LL}$. The terms neglected in this limit are of the order $\Delta_{\rm LL}^2\ln(\Lambda_{\rm LL}/\Delta_{\rm LL})$; carried through the solution, they give the BCS form a relative error of the size $(\Delta_{\rm LL}^2/(2\mu_{\rm LL}^2))\ln(\Lambda_{\rm LL}/\Delta_{\rm LL})$. Check AS-5 has four parts, all with $\Lambda_{\rm LL}=1$: (1) the closed form against a numerical quadrature of the integral, for six pairs of $\mu_{\rm LL}$ and $\Delta_{\rm LL}$; (2) the critical coupling; (3) the gaps that the closed form gives by bisection, inserted in the integral computed by quadrature, for the three chemical potentials $\mu_{\rm LL}=0,0.3,0.6$ and the four couplings $G_{\rm LL}/G^{\rm crit}_{\rm LL}=0.6,1.2,1.8,2.4$ (of these twelve cases eleven have a gap; at $\mu_{\rm LL}=0$ and $0.6\,G^{\rm crit}_{\rm LL}$ there is none); (4) the BCS form at $\mu_{\rm LL}=0.3$, whose relative deviation must be smaller than twice the size of the leading correction. The quadrature is the Gauss-Legendre rule with 32 nodes on every panel of a division of the interval from 0 to $\Lambda_{\rm LL}$. The function `t2gaussLegendre` computes the nodes, which are the zeros of the Legendre polynomial of the degree 32, by Newton's method, and their weights; with them the rule is exact for every polynomial up to the degree 63. The panels are 64 equal ones, refined towards the Fermi momentum $k^{\rm F}_{\rm LL}=\mu_{\rm LL}$, where $1/E^-_{\rm LL}$ has its sharp maximum $1/\Delta_{\rm LL}$ (function `t2gapIntegralQuad`). Figure 4(c) shows the gap as a function of the coupling.*)

(* ::Input:: *)
(* the Gauss-Legendre nodes and weights on [-1, 1]: Newton's method on the Legendre polynomial of the degree nodes,
   which is computed with its three-term recurrence (leg0 is the polynomial of the degree j + 1, leg1 of the degree j) *)
t2gaussLegendre[nodes_Integer] := Module[{nodeX = ConstantArray[0., nodes], nodeW = ConstantArray[0., nodes], z, zOld, leg0, leg1, leg2, deriv},
   Do[
    z = Cos[Pi (i + 0.75)/(nodes + 0.5)];
    Do[
     leg0 = 1.; leg1 = 0.;
     Do[leg2 = leg1; leg1 = leg0; leg0 = ((2. j + 1.) z leg1 - j leg2)/(j + 1.), {j, 0, nodes - 1}];
     deriv = nodes (z leg0 - leg1)/(z z - 1.);
     zOld = z;
     z = zOld - leg0/deriv;
     If[Abs[z - zOld] < 1.*^-15, Break[]],
     {100}];
    (* the derivative at the converged node, for the weight *)
    leg0 = 1.; leg1 = 0.;
    Do[leg2 = leg1; leg1 = leg0; leg0 = ((2. j + 1.) z leg1 - j leg2)/(j + 1.), {j, 0, nodes - 1}];
    deriv = nodes (z leg0 - leg1)/(z z - 1.);
    nodeX[[i + 1]] = -z;
    nodeW[[i + 1]] = 2./((1. - z z) deriv deriv),
    {i, 0, nodes - 1}];
   {nodeX, nodeW}];
(* the right side of the gap equation for massless spinor charges in closed form (the displayed formula): lam is the
   cutoff, chem the chemical potential, gap the gap *)
t2gapIntegralClosed[lam_, chem_, gap_] := With[{ep = Sqrt[(lam + chem)^2 + gap^2], em = Sqrt[(lam - chem)^2 + gap^2]},
   ((lam - 3. chem) ep + (lam + 3. chem) em + (2. chem chem - gap gap) (ArcSinh[(lam + chem)/gap] + ArcSinh[(lam - chem)/gap]))/(4. Pi Pi)];
(* the same integral for the mass m by composite Gauss-Legendre quadrature in q, with panels refined geometrically
   towards the Fermi momentum, where 1/E^- has its sharp maximum 1/gap; gl is the pair {nodes, weights} *)
t2gapIntegralQuad[lam_, chem_, gap_, m_, gl_List] := Module[{integrand, kf, pts, width, total = 0., mid, half},
   integrand[q_] := With[{energy = Sqrt[q q + m m]},
     q q (1./Sqrt[(energy + chem)^2 + gap gap] + 1./Sqrt[(energy - chem)^2 + gap gap])/(2. Pi Pi)];
   kf = If[chem > m, Sqrt[chem chem - m m], -1.];
   pts = {0., lam};
   If[kf > 0. && kf < lam,
    AppendTo[pts, kf];
    width = Max[gap, 1.*^-6]/4.;
    While[width < lam,
     If[kf - width > 0., AppendTo[pts, kf - width]];
     If[kf + width < lam, AppendTo[pts, kf + width]];
     width *= 2.]];
   Do[AppendTo[pts, lam i/64.], {i, 1, 63}];
   pts = Sort[pts];
   (* a point closer than 1e-14 to the last point kept is left out *)
   pts = Fold[If[Abs[#2 - Last[#1]] < 1.*^-14, #1, Append[#1, #2]] &, {First[pts]}, Rest[pts]];
   Do[
    mid = 0.5 (pts[[panel]] + pts[[panel + 1]]); half = 0.5 (pts[[panel + 1]] - pts[[panel]]);
    Do[total += gl[[2, node]] half integrand[mid + half gl[[1, node]]], {node, Length[gl[[1]]]}],
    {panel, Length[pts] - 1}];
   total];
(* the gap that solves 1/coupling = t2gapIntegralClosed[lam, chem, gap], by bisection between 1e-12 and 20 (the right
   side decreases with the gap); 0 if there is no solution above 1e-12 *)
t2solveGap[coupling_, lam_, chem_] := Module[{target = 1./coupling, lo = 1.*^-12, hi = 20., mid},
   If[t2gapIntegralClosed[lam, chem, lo] < target, Return[0., Module]];
   Do[mid = 0.5 (lo + hi); If[t2gapIntegralClosed[lam, chem, mid] > target, lo = mid, hi = mid], {200}];
   0.5 (lo + hi)];

(* ::Input:: *)
(* AS-5 (1): the closed form against the quadrature for six pairs (mu, Delta), cutoff 1 *)
as5GL = t2gaussLegendre[32];
as5Lam = 1.;
as5DevClosed = 0.;
Do[With[{closed = t2gapIntegralClosed[as5Lam, pairB[[1]], pairB[[2]]], quad = t2gapIntegralQuad[as5Lam, pairB[[1]], pairB[[2]], 0., as5GL]},
   as5DevClosed = Max[as5DevClosed, Abs[closed - quad]/Abs[quad]]],
  {pairB, {{0., 0.3}, {0.5, 0.2}, {0.8, 0.05}, {0.3, 1.0}, {0.95, 0.01}, {0., 2.0}}}];
(* (2) the critical coupling of massless charges at mu = 0: 1/G_crit = Lambda^2/(2 pi^2) *)
as5Gcrit = 2. Pi Pi/(as5Lam as5Lam);
as5GcritDev = Abs[t2gapIntegralClosed[as5Lam, 0., 1.*^-9] as5Gcrit - 1.];
(* (3) the gap as a function of the coupling for mu = 0, 0.3, 0.6: the rows {mu, G/G_crit, Delta, source} of Figure
   4(c).  Source 0: 281 couplings G/G_crit = 0.20, 0.21, ..., 3.00, the gap from the closed form by bisection;
   source 1: the couplings 0.6, 1.2, 1.8, 2.4 at which the gap is inserted in the integral computed by quadrature (only
   where there is a gap).  The ratio G/G_crit is computed exactly and then rounded, so that the ratio 1 is exactly 1 *)
as5GapRows = {};
as5DevRoot = 0.;
as5Roots = 0;
Do[
  Do[With[{ratioG = N[(20 + i)/100]}, AppendTo[as5GapRows, {chem, ratioG, t2solveGap[ratioG as5Gcrit, as5Lam, chem], 0}]], {i, 0, 280}];
  Do[With[{gap = t2solveGap[ratioG as5Gcrit, as5Lam, chem]},
    If[gap > 0.,
     as5Roots++;
     as5DevRoot = Max[as5DevRoot, Abs[t2gapIntegralQuad[as5Lam, chem, gap, 0., as5GL] ratioG as5Gcrit - 1.]];
     AppendTo[as5GapRows, {chem, ratioG, gap, 1}]]],
   {ratioG, {0.6, 1.2, 1.8, 2.4}}],
  {chem, {0., 0.3, 0.6}}];
as5GapAt15 = t2solveGap[1.5 as5Gcrit, as5Lam, 0.];
(* (4) the weak-coupling (BCS) limit at mu = 0.3: for G/G_crit = 0.3, 0.4, 0.5 the list {gap, relative deviation of
   the gap from the BCS form, size (gap^2/(2 mu^2)) ln(1/gap) of the leading correction} *)
as5Bcs = Table[
   Module[{chem = 0.3, coupling = ratioG as5Gcrit, gap, bcs},
    gap = t2solveGap[coupling, as5Lam, chem];
    bcs = 2. Sqrt[as5Lam as5Lam - chem chem] Exp[-1.5] Exp[-(Pi Pi/(chem chem)) (1./coupling - 1./as5Gcrit)];
    {gap, gap/bcs - 1., gap gap/(2. chem chem) Log[1./gap]}],
   {ratioG, {0.3, 0.4, 0.5}}];
as5BcsOk = AllTrue[as5Bcs, Abs[#[[2]]] < 2. #[[3]] &];
SL31Check["AS-5", "the gap equation in closed form for massless spinor charges (case B): int_(q<Lambda) d^3q/(2 pi)^3 (1/E^+ + 1/E^-) with E^(+-) = ((q +- mu)^2 + Delta^2)^(1/2) equals (1/(4 pi^2)) [(Lambda - 3 mu) E^+(Lambda) + (Lambda + 3 mu) E^-(Lambda) + (2 mu^2 - Delta^2) (asinh((Lambda + mu)/Delta) + asinh((Lambda - mu)/Delta))] for six pairs (mu, Delta) (relative deviation from composite Gauss-Legendre quadrature with 32 nodes per panel, tolerance 1e-10); its limit Delta -> 0 at mu = 0 is Lambda^2/(2 pi^2), so that G_crit = 2 pi^2/Lambda^2 (tolerance 1e-8); the gaps Delta(G) found from the closed form by bisection satisfy G int(...) = 1 with the integral computed by quadrature (the 11 of the 12 cases mu = 0, 0.3, 0.6 with G/G_crit = 0.6, 1.2, 1.8, 2.4 that have a gap, tolerance 1e-9); and for mu > 0 and weak coupling (G < G_crit) the gap approaches the BCS form Delta = 2 (Lambda^2 - mu^2)^(1/2) exp(-3/2) exp(-(pi^2/mu^2) (1/G - 1/G_crit)), the relative deviation being smaller than twice the size (Delta^2/(2 mu^2)) ln(1/Delta) of the leading correction (mu = 0.3, G/G_crit = 0.3, 0.4, 0.5); Lambda = 1",
  as5DevClosed < 1*^-10 && as5GcritDev < 1*^-8 && as5DevRoot < 1*^-9 && as5Roots == 11 && as5BcsOk,
  Row[{"largest deviations ", asSci[as5DevClosed], ", ", asSci[as5GcritDev], ", ", asSci[as5DevRoot], " (", as5Roots, " gaps); Delta at G = 1.5 G_crit, mu = 0: ", asFix[as5GapAt15, 6],
    "; weak coupling at mu = 0.3, G/G_crit = 0.3, 0.4, 0.5: Delta = ",
    Row[Riffle[Map[Row[{ScientificForm[#[[1]], 4], " (", asSci[#[[2]]], ", scale ", asSci[#[[3]]], ")"}] &, as5Bcs], ", "]],
    " (original: 1.8e-15, 0.0e0, 2.2e-15 (11 gaps); 0.675339; 9.985e-7 (-7.6e-11, scale 7.7e-11), 1.023e-4 (-5.3e-7, scale 5.3e-7), 1.646e-3 (-9.5e-5, scale 9.6e-5))"}]]

(* ::Text:: *)
(***The uniform state is an exact self-consistent solution on the lattice (check AS-6).** The self-consistency condition of the mean field is $d^\iota_{\rm LL}(x_{\rm LL})=-\tfrac12G_{\rm LL}\sum(\psi^{\rm p}_{{\rm LL},n})^T\mathsf K^\iota_{\rm LL}(\psi^{\rm h}_{{\rm LL},n})^{*}$, where the sum runs over the modes with $E^{\rm qp}_{{\rm LL},n}>0$ and every mode is normalized so that the integral of $\lvert\psi^{\rm p}_{{\rm LL},n}\rvert^2+\lvert\psi^{\rm h}_{{\rm LL},n}\rvert^2$ over space is 1. On a chain the eigenvectors of the lattice operator are unit vectors, and the modes with this normalization are their components divided by $(\ell^{\rm x}_{\rm LL})^{1/2}$. For the uniform polar state the eight positive-energy eigenvectors of one momentum contribute together $-2\Delta_{\rm LL}(1/E^+_{\rm LL}+1/E^-_{\rm LL})$ to the sum of the third isospin direction (check GS-9), divided by $n^{\rm x}_{\rm LL}$ because a unit vector spreads its norm over the sites of the chain, and nothing to the directions 1 and 2. The sum over the modes at every site is therefore $-2\Delta_{\rm LL}(n^{\rm x}_{\rm LL})^{-1}\sum(1/E^+_{\rm LL}+1/E^-_{\rm LL})$ for $\iota=3$, the last sum running over the allowed momenta with $\varepsilon^{\rm lat}_{\rm LL}$ in $E^\pm_{\rm LL}$, and 0 for $\iota=1,2$. The self-consistency condition then returns the same pair field $(0,0,\Delta_{\rm LL})$ on every site exactly when*)

(* ::DisplayFormula:: *)
(*\frac1{G_{\rm LL}}=\frac1{n^{\rm x}_{\rm LL}\ell^{\rm x}_{\rm LL}}\sum_{q^1_{\rm LL}}\Big(\frac1{E^+_{\rm LL}}+\frac1{E^-_{\rm LL}}\Big)*)

(* ::Text:: *)
(*the gap equation of the chain: the sum over the allowed momenta, divided by the circumference of the ring, stands in place of the integral over $q^1_{\rm LL}$ divided by $2\pi$. Check AS-6 computes the sum over the modes with the routine of the solvers (`t2anomalous`) from the 128 positive-energy eigenvectors of the chain of 16 sites of check AS-4, compares it with the formula, and verifies that the self-consistency condition with the coupling of the displayed equation gives back $\Delta_{\rm LL}$ on every site: a fixed point that the self-consistent solvers of the items [3] and [5] must reproduce.*)

(* ::Input:: *)
(* AS-6: the chain and the pair field of AS-4 (the original program builds the same chain a second time); the
   eigenvectors of its positive eigenvalues are the modes *)
{as6Ev, as6Vec} = t2eigh[as4H];
as6Modes = Table[as6Vec[[All, col]], {col, Select[Range[Length[as6Ev]], as6Ev[[#]] > 0 &]}];
(* the sum over the modes of psi_p^T Kch_iota conj(psi_h) at every site, by the routine of the solvers *)
as6Pi = t2anomalous[as4N, as6Modes];
(* the sum over the 16 allowed momenta of 1/E^- + 1/E^+ (eps_lat in place of eps) *)
as6Sum = Sum[With[{el = t2epsLat[{t2chainMomentum[kq, as4N, asBEll], 0., 0.}, asBEll, mass, 1.0, asChainActive]},
    1./Sqrt[(el - asBMu)^2 + asBDelta^2] + 1./Sqrt[(el + asBMu)^2 + asBDelta^2]], {kq, 0, as4N - 1}];
as6PiExpect = -2. asBDelta as6Sum/as4N;
(* the coupling of the lattice gap equation 1/G = (1/(N dx)) sum_q (1/E^+ + 1/E^-) *)
as6G = 1./(as6Sum/(as4N asBEll));
as6DevSum = Max[Abs[as6Pi[[All, 1]]], Abs[as6Pi[[All, 2]]], Abs[as6Pi[[All, 3]] - as6PiExpect]];
(* the new pair field d^3 = -(G/2) Pi^3 with Pi^3 = (mode sum)/dx *)
as6DevNew = Max[Abs[as6Pi[[All, 3]] (-0.5 as6G/asBEll) - asBDelta]];
SL31Check["AS-6", "the uniform polar state is an exact self-consistent solution of the lattice mean-field equations (case B): on a periodic chain of 16 sites (dx = 0.5, r_W = 1, m = 0.4, mu = 0.6) with dvec = (0, 0, 0.3), the sum over the 128 positive-energy eigenvectors of the lattice operator (unit vectors, with the particle part psi_p and the hole part psi_h) of psi_p^T Kch_iota conj(psi_h), computed at every site by the routine of the solvers, is 0 for iota = 1, 2 and -2 Delta (1/16) sum_q (1/E^+ + 1/E^-) (eps_lat in place of eps, the 16 allowed momenta) for iota = 3 (tolerance 1e-12); with Pi_iota = dx^(-1) times this sum (the continuum normalization of a chain) and the coupling G given by the lattice gap equation 1/G = (1/(16 dx)) sum_q (1/E^+ + 1/E^-), the self-consistency condition d_iota = -(G/2) Pi_iota gives back dvec = (0, 0, Delta) on every site (tolerance 1e-12): a fixed point that the self-consistent solvers must reproduce",
  as6DevSum < 1*^-12 && as6DevNew < 1*^-12 && Length[as6Modes] == 8 as4N,
  Row[{Length[as6Modes], " positive modes; largest deviations ", asSci[as6DevSum], " (mode sum), ", asSci[as6DevNew], " (new dvec); G = ", asFix[as6G, 6],
    " (original: 128 positive modes; 2.6e-15, 1.1e-15; G = 0.432159)"}]]

(* ::Subsection:: *)
(*Case C: a domain wall of the pair field (checks AS-7 to AS-9)*)

(* ::Text:: *)
(***The fields.** No gauge fields, $\mu_{\rm LL}=0$, and a real pair field along the third isospin direction that depends on $x^1_{\rm LL}$ and changes its sign: $\vec d_{\rm LL}=(0,0,\Delta_{\rm LL}(x^1_{\rm LL}))$ with $\Delta_{\rm LL}(x^1_{\rm LL})=\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$. Two superfluid domains with opposite signs of the order parameter meet at a wall of the width $\ell^{\rm K}_{\rm LL}$ (a kink); $\Delta^\infty_{\rm LL}$ is the gap far from the wall. The fields do not depend on $x^2_{\rm LL}$ and $x^3_{\rm LL}$, and the modes are plane waves $\exp(i(q^2_{\rm LL}x^2_{\rm LL}+q^3_{\rm LL}x^3_{\rm LL}))$ along the wall; first $q^2_{\rm LL}=q^3_{\rm LL}=0$.*)

(* ::Text:: *)
(***Four facts about the matrices.** With $\mathsf K^3_{\rm LL}=\mathrm{diag}(\mathsf E_{\rm LL},\mathsf E_{\rm LL})\otimes\tau^{\rm LL}_1$ and $\alpha^1_{\rm LL}=\mathrm{diag}(-\sigma^{\rm LL}_1,\sigma^{\rm LL}_1)$: (1) $\big((\mathsf K^3_{\rm LL})^\dagger\big)^2=-\mathsf 1_8$; (2) $\alpha^1_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}=-(\mathsf K^3_{\rm LL})^\dagger$, because $\sigma^{\rm LL}_1\mathsf E_{\rm LL}\sigma^{\rm LL}_1=-\mathsf E_{\rm LL}$; (3) $\beta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}=-(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\beta_{\rm LL}$, because $\beta_{\rm LL}$ commutes with $(\mathsf K^3_{\rm LL})^\dagger$ and anticommutes with $\alpha^1_{\rm LL}$; (4) $\alpha^1_{\rm LL}$ and $\beta_{\rm LL}$ are real.*)

(* ::Text:: *)
(***The bound states.** Let the profile $\varphi^{\rm K}_{\rm LL}$ solve $d\varphi^{\rm K}_{\rm LL}/dx^1_{\rm LL}=-\Delta_{\rm LL}\varphi^{\rm K}_{\rm LL}$; for the kink this is $\varphi^{\rm K}_{\rm LL}(x^1_{\rm LL})=\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}}$. Let $\chi^{\rm K}_{\rm LL}$ be a constant spinor of eight components with $\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=\pm\chi^{\rm K}_{\rm LL}$. Then the mode*)

(* ::DisplayFormula:: *)
(*\psi^{\rm p}_{\rm LL}=\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL},\qquad\psi^{\rm h}_{\rm LL}=i(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\varphi^{\rm K}_{\rm LL}\chi^{\rm K}_{\rm LL},\qquad E^{\rm qp}_{{\rm LL},n}=\pm m_{\rm LL}*)

(* ::Text:: *)
(*is a stationary solution of the field equations. Proof. The particle row is the sum of three terms. Because $d\varphi^{\rm K}_{\rm LL}/dx^1_{\rm LL}=-\Delta_{\rm LL}\varphi^{\rm K}_{\rm LL}$, the first term is $-i\alpha^1_{\rm LL}\partial_1\psi^{\rm p}_{\rm LL}=i\Delta_{\rm LL}\varphi^{\rm K}_{\rm LL}\alpha^1_{\rm LL}\chi^{\rm K}_{\rm LL}$. The second is $\beta_{\rm LL}m_{\rm LL}\psi^{\rm p}_{\rm LL}=\pm m_{\rm LL}\psi^{\rm p}_{\rm LL}$. The third is $\Delta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\psi^{\rm h}_{\rm LL}=i\Delta_{\rm LL}\varphi^{\rm K}_{\rm LL}\big((\mathsf K^3_{\rm LL})^\dagger\big)^2\alpha^1_{\rm LL}\chi^{\rm K}_{\rm LL}$, which by fact (1) is minus the first. The particle row is therefore $\pm m_{\rm LL}\psi^{\rm p}_{\rm LL}$. The hole row, with $\mathsf P_{\rm LL}^\dagger=\Delta_{\rm LL}\mathsf K^3_{\rm LL}=-\Delta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger$ and the real matrices of fact (4), is $\Delta_{\rm LL}\mathsf K^3_{\rm LL}\psi^{\rm p}_{\rm LL}-(i\alpha^1_{\rm LL}\partial_1+\beta_{\rm LL}m_{\rm LL})\psi^{\rm h}_{\rm LL}$, again three terms. The first is $\Delta_{\rm LL}\mathsf K^3_{\rm LL}\psi^{\rm p}_{\rm LL}=-\Delta_{\rm LL}\varphi^{\rm K}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\chi^{\rm K}_{\rm LL}$. The second is $-i\alpha^1_{\rm LL}\partial_1\psi^{\rm h}_{\rm LL}=-\Delta_{\rm LL}\varphi^{\rm K}_{\rm LL}\alpha^1_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\chi^{\rm K}_{\rm LL}$, which by fact (2) is minus the first. The third is $-\beta_{\rm LL}m_{\rm LL}\psi^{\rm h}_{\rm LL}=-im_{\rm LL}\varphi^{\rm K}_{\rm LL}\beta_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\chi^{\rm K}_{\rm LL}$, which by fact (3) is $im_{\rm LL}\varphi^{\rm K}_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\alpha^1_{\rm LL}\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=\pm m_{\rm LL}\psi^{\rm h}_{\rm LL}$. The hole row is therefore $\pm m_{\rm LL}\psi^{\rm h}_{\rm LL}$, and the proof is complete.*)

(* ::Text:: *)
(*The profile falls off like $\exp(-\Delta^\infty_{\rm LL}\lvert x^1_{\rm LL}\rvert)$ on both sides of the wall, because $\Delta_{\rm LL}$ has opposite signs on the two sides: the state is bound to the wall. The spinors with $\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=+\chi^{\rm K}_{\rm LL}$ form a space of four dimensions, and so do those with $\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=-\chi^{\rm K}_{\rm LL}$: every wall carries four bound states of the energy $+m_{\rm LL}$ and four of the energy $-m_{\rm LL}$. Their energy does not depend on the shape of the profile, only on its change of sign (the mechanism of Jackiw and Rebbi, here for the pair field of the spinor charges). For an antikink, $\Delta_{\rm LL}=-\Delta^\infty_{\rm LL}\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$, the bound states have the same profile and the same energies, and the hole part has the opposite sign. Check AS-7 verifies the four facts, and the sixteen component equations of the mode at 50 random points with random spinors $\chi^{\rm K}_{\rm LL}$ (a random vector projected on the eigenspace of $\beta_{\rm LL}$ with the eigenvalue $+1$ or $-1$), for $\Delta^\infty_{\rm LL}=1$, $\ell^{\rm K}_{\rm LL}=1$ and $m_{\rm LL}=0.4$.*)

(* ::Input:: *)
(* AS-7: the identities of the 8 x 8 matrices (asK3dag is (K^3)^dag, asAlpha[[1]] is alpha^1, asBeta is beta) *)
as7IdDev = fro[asK3dag . asK3dag + IdentityMatrix[8]] + fro[ConjugateTranspose[asK3dag] + asK3dag] +
   fro[asAlpha[[1]] . asK3dag . asAlpha[[1]] + asK3dag] + fro[asBeta . asK3dag . asAlpha[[1]] + asK3dag . asAlpha[[1]] . asBeta] +
   fro[asAlpha[[1]] - Conjugate[asAlpha[[1]]]] + fro[asBeta - Conjugate[asBeta]];
(* the kink of case C: the gap far from the wall and the width of the wall *)
{asCDinf, asCEllK} = {1.0, 1.0};
as7Dev = 0.;
Do[
  Module[{x = 4. t2Unit[], gap, phi, dphi, y, sg, chi, eta, u, du, v, dv, rowP, rowH},
   gap = asCDinf Tanh[x/asCEllK];
   phi = Cosh[x/asCEllK]^(-asCDinf asCEllK);
   (* the derivative of the profile: phi_K' = -Delta phi_K *)
   dphi = -gap phi;
   (* chi_K: a random vector projected on the eigenspace beta = +1 (even trials) or -1 (odd trials) *)
   y = t2RndVec[8];
   sg = If[EvenQ[trial], 1., -1.];
   chi = 0.5 (y + sg asBeta . y);
   (* i (K^3)^dag alpha^1 chi_K *)
   eta = I asK3dag . asAlpha[[1]] . chi;
   u = phi chi; du = dphi chi; v = phi eta; dv = dphi eta;
   (* the particle row -i alpha^1 psi_p' + m beta psi_p + Delta (K^3)^dag psi_h, minus the energy times psi_p *)
   rowP = -I asAlpha[[1]] . du + mass asBeta . u + gap asK3dag . v - sg mass u;
   (* the hole row Delta K^3 psi_p - (i conj(alpha^1) psi_h' + m conj(beta) psi_h), minus the energy times psi_h *)
   rowH = gap ConjugateTranspose[asK3dag] . u - I Conjugate[asAlpha[[1]]] . dv - mass Conjugate[asBeta] . v - sg mass v;
   as7Dev = Max[as7Dev, Max[Abs[rowP]], Max[Abs[rowH]]]],
  {trial, 0, 49}];
SL31Check["AS-7", "the bound states of a domain wall (kink) of the pair field (case C), in the continuum: the 8 x 8 matrices (Dirac (x) isospin) satisfy (Kch_3^dag)^2 = -1, (Kch_3^dag)^dag = -Kch_3^dag, alpha_LL^1 Kch_3^dag alpha_LL^1 = -Kch_3^dag and beta_LL Kch_3^dag alpha_LL^1 = -Kch_3^dag alpha_LL^1 beta_LL, and alpha_LL^1 and beta_LL are real (tolerance 1e-13); for the kink dvec = (0, 0, Delta(x_LL^1)) with Delta(x_LL^1) = Delta_inf tanh(x_LL^1/ell_K) (Delta_inf = 1, ell_K = 1) and mu = 0, the mode psi_p = phi_K chi_K, psi_h = i Kch_3^dag alpha_LL^1 phi_K chi_K with phi_K = cosh(x_LL^1/ell_K)^(-Delta_inf ell_K) (so that d phi_K/d x_LL^1 = -Delta phi_K) and a constant chi_K with beta_LL chi_K = +chi_K or -chi_K satisfies the sixteen component equations with the energy +m or -m at 50 random points (m = 0.4, random chi_K; tolerance 1e-12)",
  as7IdDev < 1*^-13 && as7Dev < 1*^-12,
  Row[{"deviations of the identities ", asSci[as7IdDev], ", of the equations ", asSci[as7Dev], " (original: 0.0e0, 8.3e-17)"}]]

(* ::Text:: *)
(***The square of the operator, and all bound states (check AS-8).** With the plane wave $\exp(i(q^2_{\rm LL}x^2_{\rm LL}+q^3_{\rm LL}x^3_{\rm LL}))$ along the wall, the particle block of $\mathcal H^{\rm DB}_{\rm LL}$ is the operator $-i\alpha^1_{\rm LL}\partial_1+\alpha^2_{\rm LL}q^2_{\rm LL}+\alpha^3_{\rm LL}q^3_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$, and the hole block is minus the complex conjugate of the same operator with $-q^2_{\rm LL}$ and $-q^3_{\rm LL}$ in place of $q^2_{\rm LL}$ and $q^3_{\rm LL}$. By the relation $\alpha^j_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger=-(\mathsf K^3_{\rm LL})^\dagger(\alpha^j_{\rm LL})^{*}$ for $j=1,2,3$ (the key identity of case B in another form), the particle block times $(\mathsf K^3_{\rm LL})^\dagger$ equals $(\mathsf K^3_{\rm LL})^\dagger$ times the hole block with the opposite sign. In the off-diagonal blocks of the square of $\mathcal H^{\rm DB}_{\rm LL}$ everything therefore cancels except the derivative of $\Delta_{\rm LL}$, which arises when $\partial_1$ acts on the factor $\Delta_{\rm LL}$; the diagonal blocks are the squares of the particle block and of the hole block plus $\Delta_{\rm LL}^2(\mathsf K^3_{\rm LL})^\dagger\mathsf K^3_{\rm LL}=\Delta_{\rm LL}^2\mathsf 1_8$. The result is*)

(* ::DisplayFormula:: *)
(*(\mathcal H^{\rm DB}_{\rm LL})^2=\Big(-\frac{d^2}{d(x^1_{\rm LL})^2}+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2+m_{\rm LL}^2+\Delta_{\rm LL}^2\Big)\mathsf 1_{16}+\frac{d\Delta_{\rm LL}}{dx^1_{\rm LL}}\,\mathsf Q^{\rm K}_{\rm LL}*)

(* ::Text:: *)
(*with the constant matrix*)

(* ::DisplayFormula:: *)
(*\mathsf Q^{\rm K}_{\rm LL}=\begin{pmatrix}0&-i\alpha^1_{\rm LL}(\mathsf K^3_{\rm LL})^\dagger\\-i\alpha^1_{\rm LL}\mathsf K^3_{\rm LL}&0\end{pmatrix}*)

(* ::Text:: *)
(*The constant matrix $\mathsf Q^{\rm K}_{\rm LL}$ is Hermitian, its square is the unit matrix, and its trace is 0: it has the eigenvalue $-1$ on a subspace of eight dimensions and the eigenvalue $+1$ on another. On these two subspaces the square of the operator is $m_{\rm LL}^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2$ plus one of the two scalar operators $-d^2/d(x^1_{\rm LL})^2+\Delta_{\rm LL}^2-d\Delta_{\rm LL}/dx^1_{\rm LL}$ (eigenvalue $-1$) and $-d^2/d(x^1_{\rm LL})^2+\Delta_{\rm LL}^2+d\Delta_{\rm LL}/dx^1_{\rm LL}$ (eigenvalue $+1$). The first is the product $(-d/dx^1_{\rm LL}+\Delta_{\rm LL})(d/dx^1_{\rm LL}+\Delta_{\rm LL})$, and the second is the product of the same two first-order operators in the other order. Two such products have the same eigenvalues different from zero (if one of them maps a function to a multiple of itself, the first-order factor applied to that function gives an eigenfunction of the other product with the same eigenvalue). They differ only in the eigenvalue zero: $d/dx^1_{\rm LL}+\Delta_{\rm LL}$ maps the normalizable profile $\varphi^{\rm K}_{\rm LL}$ to zero, while the functions that $-d/dx^1_{\rm LL}+\Delta_{\rm LL}$ maps to zero are the multiples of $1/\varphi^{\rm K}_{\rm LL}$, which grow on both sides of the wall. The sector with the eigenvalue $-1$ therefore has exactly one bound state more than the sector with $+1$: the state of the display before this one, with the squared energy $m_{\rm LL}^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2$, the energy of a free particle of the mass $m_{\rm LL}$ that moves in the two dimensions of the wall.*)

(* ::Text:: *)
(*For the kink, $\Delta_{\rm LL}^2\mp d\Delta_{\rm LL}/dx^1_{\rm LL}=(\Delta^\infty_{\rm LL})^2-\Delta^\infty_{\rm LL}(\Delta^\infty_{\rm LL}\pm1/\ell^{\rm K}_{\rm LL})\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-2}$, because $\tanh^2=1-\cosh^{-2}$ and the derivative of $\tanh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})$ is $\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-2}/\ell^{\rm K}_{\rm LL}$. The operator of the sector $-1$ is therefore $(\Delta^\infty_{\rm LL})^2$ plus the potential well $-d^2/d(x^1_{\rm LL})^2-\Delta^\infty_{\rm LL}(\Delta^\infty_{\rm LL}+1/\ell^{\rm K}_{\rm LL})\cosh(x^1_{\rm LL}/\ell^{\rm K}_{\rm LL})^{-2}$, a well of the form of Poeschl and Teller with the parameter $\Delta^\infty_{\rm LL}$; and the operator of the sector $+1$ is $(\Delta^\infty_{\rm LL})^2$ plus the well of the same form with the parameter $\Delta^\infty_{\rm LL}-1/\ell^{\rm K}_{\rm LL}$. By the argument of the two products, the well with the parameter $\Delta^\infty_{\rm LL}$ has one bound state more than the well with the parameter $\Delta^\infty_{\rm LL}-1/\ell^{\rm K}_{\rm LL}$, namely $\varphi^{\rm K}_{\rm LL}$ with the eigenvalue $-(\Delta^\infty_{\rm LL})^2$ of the well, and its other bound states are those of the well with the smaller parameter. Applied again and again, as long as the parameter stays positive, this gives the eigenvalues $-(\Delta^\infty_{\rm LL}-k^{\rm K}_{\rm LL}/\ell^{\rm K}_{\rm LL})^2$ of the well with the parameter $\Delta^\infty_{\rm LL}$, for the integers $k^{\rm K}_{\rm LL}$ with $0\le k^{\rm K}_{\rm LL}<\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$, and those with $1\le k^{\rm K}_{\rm LL}<\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}$ for the sector $+1$. The bound states of the wall therefore have the energies*)

(* ::DisplayFormula:: *)
(*E^{\rm qp}_{{\rm LL},n}=\pm\Big(m_{\rm LL}^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2+\frac{2k^{\rm K}_{\rm LL}\Delta^\infty_{\rm LL}}{\ell^{\rm K}_{\rm LL}}-\frac{(k^{\rm K}_{\rm LL})^2}{(\ell^{\rm K}_{\rm LL})^2}\Big)^{1/2},\qquad k^{\rm K}_{\rm LL}=0,1,2,\dots,\quad k^{\rm K}_{\rm LL}<\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}*)

(* ::Text:: *)
(*four times each sign for $k^{\rm K}_{\rm LL}=0$ (the sector $-1$ only) and eight times each sign for $k^{\rm K}_{\rm LL}\ge1$ (both sectors); the states that are not bound begin at $\big(m_{\rm LL}^2+(q^2_{\rm LL})^2+(q^3_{\rm LL})^2+(\Delta^\infty_{\rm LL})^2\big)^{1/2}$. In particular, for $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}\le1$ the states with $k^{\rm K}_{\rm LL}=0$ are the only bound states. Check AS-8 verifies the properties of $\mathsf Q^{\rm K}_{\rm LL}$ and the relation between $\alpha^j_{\rm LL}$ and $(\mathsf K^3_{\rm LL})^\dagger$; the formula for the square, by applying the operator twice to a Gaussian function of $x^1_{\rm LL}$ times a random vector of 16 components, with exact derivatives, for 50 random kinks, points and momenta; and the bound states of the two scalar operators for $\Delta^\infty_{\rm LL}\ell^{\rm K}_{\rm LL}=1$ and $2.5$ (one and none, and three and two bound states).*)

(* ::Text:: *)
(*The bound states of the scalar operators are computed by second-order finite differences on the interval from $-20$ to $20$ with 4000 and with 8001 inner points. The error of such differences is proportional to the square of the spacing, so four thirds of the finer result minus one third of the coarser one removes its leading part (Richardson extrapolation). The finite-difference operator is a symmetric tridiagonal matrix. The number of its eigenvalues below a given number is the number of negative pivots in the elimination of the matrix minus that number times the unit matrix (a Sturm sequence; function `t2sturmCount`, which is compiled with `Compile` because the check passes through some fourteen million matrix rows), and bisection on this count gives every eigenvalue below the threshold $(\Delta^\infty_{\rm LL})^2$ (function `t2sturmLowest`).*)

(* ::Input:: *)
(* the number of eigenvalues below the number below of the real symmetric tridiagonal matrix with the diagonal diag and
   the constant off-diagonal off: the number of negative pivots of the elimination of the matrix minus below times the
   unit matrix (a Sturm sequence).  A pivot that is exactly zero is replaced by -1e-300, as in the original program *)
t2sturmCount = Compile[{{diag, _Real, 1}, {off, _Real}, {below, _Real}},
   Module[{count = 0, pivot = 1.},
    Do[
     pivot = (diag[[i]] - below) - If[i == 1, 0., off off/pivot];
     If[pivot == 0., pivot = -1.*^-300];
     If[pivot < 0., count++],
     {i, Length[diag]}];
    count]];
(* the eigenvalues number 0, 1, ... (ascending) below upper of the tridiagonal matrix, each by 200 steps of bisection
   between lower and upper *)
t2sturmLowest[diag_, off_, lower_, upper_] := Table[
   Module[{lo = lower, hi = upper, mid},
    Do[mid = 0.5 (lo + hi); If[t2sturmCount[diag, off, mid] > number, hi = mid, lo = mid], {200}];
    0.5 (lo + hi)],
   {number, 0, t2sturmCount[diag, off, upper] - 1}];
(* the bound-state eigenvalues (below the threshold dinf^2) of the scalar operator -d^2/dx^2 + Delta^2 + sgn Delta' with
   Delta = dinf tanh(x/ellk), by second-order finite differences on (-xmax, xmax) with Dirichlet ends, extrapolated from
   npts and 2 npts + 1 inner points *)
t2ptBoundStates[dinf_, ellk_, sgn_, xmax_, npts_Integer] := Module[{solve, coarse, fine, len},
   solve[np_] := Module[{width = 2. xmax/(np + 1.), diag},
     diag = Table[With[{x = -xmax + i width}, 2./(width width) + (dinf Tanh[x/ellk])^2 + sgn dinf/ellk/Cosh[x/ellk]^2], {i, 1, np}];
     t2sturmLowest[diag, -1./(width width), Min[diag] - 4./(width width), dinf dinf]];
   coarse = solve[npts]; fine = solve[2 npts + 1];
   len = Min[Length[coarse], Length[fine]];
   (4. fine[[1 ;; len]] - coarse[[1 ;; len]])/3.];

(* ::Input:: *)
(* AS-8: the matrix Qmat_K = [[0, -i alpha^1 (K^3)^dag], [-i alpha^1 K^3, 0]]: Hermitian, its square the unit matrix, trace 0 *)
as8Qk = ArrayFlatten[{{0, -I asAlpha[[1]] . asK3dag}, {-I asAlpha[[1]] . asK3, 0}}];
as8QkDev = fro[as8Qk - ConjugateTranspose[as8Qk]] + fro[as8Qk . as8Qk - IdentityMatrix[16]] + Abs[Tr[as8Qk]];
(* the relation alpha^j (K^3)^dag = -(K^3)^dag conj(alpha^j) (j = 1, 2, 3), which removes the cross terms *)
as8RelDev = Max[Table[fro[asAlpha[[j]] . asK3dag + asK3dag . Conjugate[asAlpha[[j]]]], {j, 3}]];
(* the square of the operator, applied to v = (Gaussian) times a random vector, at 50 random points *)
as8DevSq = 0.;
Do[
  Module[{dinfR, ellR, q2, q3, x, x0, sw, gs0, gs1, gs2, gap, dgap, wv16, wPart, wHole, hp0, hh0, ma1, pa1, hvp, hvp1, hvh, hvh1, h2p, h2h, scal, qw},
   dinfR = 0.5 + 1.5 Abs[t2Unit[]];
   ellR = 0.5 + 1.5 Abs[t2Unit[]];
   q2 = t2Unit[]; q3 = t2Unit[];
   x = 3. t2Unit[];
   x0 = t2Unit[]; sw = 0.5 + Abs[t2Unit[]];
   (* the Gaussian exp(-(x - x0)^2/(2 sw^2)) and its first and second derivative at x *)
   gs0 = Exp[-(x - x0)^2/(2. sw sw)];
   gs1 = -(x - x0)/(sw sw) gs0;
   gs2 = ((x - x0)^2/sw^4 - 1./(sw sw)) gs0;
   (* the kink and its derivative at x *)
   gap = dinfR Tanh[x/ellR];
   dgap = dinfR/ellR/Cosh[x/ellR]^2;
   wv16 = t2RndVec[16];
   wPart = wv16[[1 ;; 8]]; wHole = wv16[[9 ;; 16]];
   (* the parts of the particle and of the hole operator that do not depend on x:
      alpha^2 q^2 + alpha^3 q^3 + beta m, and conj(-alpha^2 q^2 - alpha^3 q^3 + beta m) *)
   hp0 = q2 asAlpha[[2]] + q3 asAlpha[[3]] + mass asBeta;
   hh0 = -q2 Conjugate[asAlpha[[2]]] - q3 Conjugate[asAlpha[[3]]] + mass Conjugate[asBeta];
   (* -i alpha^1 and conj(-i alpha^1) = i conj(alpha^1) *)
   ma1 = -I asAlpha[[1]];
   pa1 = I Conjugate[asAlpha[[1]]];
   (* H v and its derivative, from the Gaussian and the kink and their derivatives *)
   hvp = gs1 ma1 . wPart + gs0 hp0 . wPart + gap gs0 asK3dag . wHole;
   hvp1 = gs2 ma1 . wPart + gs1 hp0 . wPart + (dgap gs0 + gap gs1) asK3dag . wHole;
   hvh = gap gs0 asK3 . wPart - gs1 pa1 . wHole - gs0 hh0 . wHole;
   hvh1 = (dgap gs0 + gap gs1) asK3 . wPart - gs2 pa1 . wHole - gs1 hh0 . wHole;
   (* H (H v) *)
   h2p = ma1 . hvp1 + hp0 . hvp + gap asK3dag . hvh;
   h2h = gap asK3 . hvp - pa1 . hvh1 - hh0 . hvh;
   (* the formula *)
   scal = -gs2 + (q2 q2 + q3 q3 + mass mass + gap gap) gs0;
   qw = as8Qk . wv16;
   as8DevSq = Max[as8DevSq, Max[Abs[Join[h2p, h2h] - (scal wv16 + dgap gs0 qw)]]]],
  {50}];
(* the bound states of the two scalar operators for Delta_inf ell_K = 1 and 2.5 (ell_K = 1): for the eigenvalue -1 of
   Qmat_K the well parameter nu is Delta_inf ell_K, for +1 it is Delta_inf ell_K - 1; the expected eigenvalues are
   Delta_inf^2 - (nu - k)^2/ell_K^2 for k = 0, 1, ... < nu *)
as8DevPT = 0.;
as8CountOk = True;
as8Counts = {};
Do[
  Do[
   Module[{dinfT = kink[[1]], ellT = kink[[2]], sgn = sector[[1]], nu = kink[[1]] kink[[2]] + sector[[2]], found, expect = {}, level = 0.},
    found = t2ptBoundStates[dinfT, ellT, sgn, 20., 4000];
    While[level < nu - 1.*^-12, AppendTo[expect, dinfT dinfT - (nu - level)^2/(ellT ellT)]; level += 1.];
    If[Length[found] != Length[expect], as8CountOk = False];
    Do[as8DevPT = Max[as8DevPT, Abs[found[[i]] - expect[[i]]]], {i, Min[Length[found], Length[expect]]}];
    AppendTo[as8Counts, {dinfT ellT, sgn, Length[found]}]],
   {sector, {{-1., 0.}, {1., -1.}}}],
  {kink, {{1., 1.}, {2.5, 1.}}}];
SL31Check["AS-8", "the square of the operator and the complete spectrum of the bound states of a wall (case C): with mu = 0, no gauge fields, dvec = (0, 0, Delta(x_LL^1)) real and the plane wave exp(i (q^2 x_LL^2 + q^3 x_LL^3)) along the wall, H_DB^2 = (-d^2/d(x_LL^1)^2 + (q^2)^2 + (q^3)^2 + m^2 + Delta^2) 1_16 + (d Delta/d x_LL^1) Qmat_K with the constant matrix Qmat_K = [[0, -i alpha_LL^1 Kch_3^dag], [-i alpha_LL^1 Kch_3, 0]], which is Hermitian, squares to the unit matrix and has the trace 0 (tolerance 1e-13), because alpha_LL^j Kch_3^dag = -Kch_3^dag conj(alpha_LL^j) for j = 1, 2, 3 (tolerance 1e-13); the identity is checked by applying H_DB twice to a Gaussian times a random 16-component vector, with exact derivatives, for 50 random kinks Delta = Delta_inf tanh(x_LL^1/ell_K), points and momenta (m = 0.4; tolerance 1e-11); the bound states of the two scalar operators -d^2/d(x_LL^1)^2 + Delta^2 - d Delta/d x_LL^1 (the eigenvalue -1 of Qmat_K) and -d^2/d(x_LL^1)^2 + Delta^2 + d Delta/d x_LL^1 (the eigenvalue +1) are those of Poeschl-Teller wells, Delta_inf^2 - (Delta_inf - k_K/ell_K)^2 for the integers k_K with 0 <= k_K < Delta_inf ell_K (eigenvalue -1) and 1 <= k_K < Delta_inf ell_K (eigenvalue +1): finite differences with 4000 and 8001 points on (-20, 20), extrapolated, give exactly these bound states, and no others below the threshold Delta_inf^2, for Delta_inf ell_K = 1 and 2.5 (ell_K = 1; tolerance 1e-6); hence the bound states of the wall have the energies +-(m^2 + (q^2)^2 + (q^3)^2 + 2 k_K Delta_inf/ell_K - k_K^2/ell_K^2)^(1/2)",
  as8QkDev < 1*^-13 && as8RelDev < 1*^-13 && as8DevSq < 1*^-11 && as8DevPT < 1*^-6 && as8CountOk,
  Row[{"deviations: Qmat_K ", asSci[as8QkDev], ", relation ", asSci[as8RelDev], ", square ", asSci[as8DevSq], ", bound states ", asSci[as8DevPT], "; ",
    Row[Riffle[Map[Row[{"Delta_inf ell_K = ", asFix[#[[1]], 1], ", Qmat_K = ", If[#[[2]] > 0, "+1", "-1"], ": ", #[[3]], If[#[[3]] == 1, " bound state", " bound states"]}] &, as8Counts], "; "]],
    " (original: 0.0e0, 0.0e0, 9.9e-16, 9.2e-9; 1, 0, 3 and 2 bound states)"}]]

(* ::Text:: *)
(***On the lattice: sixteen bound states, and the first-order effect of the Wilson term (check AS-9).** A periodic lattice needs a second wall. The check uses a ring of the length 24 with a kink at $x^1_{\rm LL}=6$ and an antikink at $x^1_{\rm LL}=18$, $\Delta_{\rm LL}(x^1_{\rm LL})=\tanh(x^1_{\rm LL}-6)\tanh(18-x^1_{\rm LL})$ ($\Delta^\infty_{\rm LL}=\ell^{\rm K}_{\rm LL}=1$, so only the states with $k^{\rm K}_{\rm LL}=0$ are bound), with $m_{\rm LL}=0.4$ and $\mu_{\rm LL}=0$. On 60 sites ($\ell^{\rm x}_{\rm LL}=0.4$) the lattice operator is a matrix of 960 rows, which `t2eigh` diagonalizes. It has exactly 16 eigenvalues inside 0.8 times the threshold $(m_{\rm LL}^2+(\Delta^\infty_{\rm LL})^2)^{1/2}$: eight near $+m_{\rm LL}$ and eight near $-m_{\rm LL}$, four for each wall and sign, and the density of the eight positive ones sits on the two walls with the profile $(\varphi^{\rm K}_{\rm LL})^2$. They lie slightly above $m_{\rm LL}$ in magnitude, because of the Wilson term. To see by how much, the analytic bound state of the kink is put on the lattice, with the spinor $\chi^{\rm K}_{\rm LL}$ whose components 1 and 5 are $1/\sqrt2$ and whose other components are zero ($\beta_{\rm LL}\chi^{\rm K}_{\rm LL}=\chi^{\rm K}_{\rm LL}$, because $\beta_{\rm LL}$ exchanges the first and the third Dirac component, which with isospin up are the components 1 and 5), and its Rayleigh quotient $E^{\rm R}_{\rm LL}$ is computed: the scalar product of the state with $\mathcal H^{\rm lat}_{\rm LL}$ applied to it, divided by the scalar product of the state with itself. In $\mathcal H^{\rm lat}_{\rm LL}-m_{\rm LL}$ applied to the state the mass terms cancel as in the proof above; the terms with $\alpha^1_{\rm LL}$ and with the pair field contribute nothing to the Rayleigh quotient, because $(\chi^{\rm K}_{\rm LL})^\dagger\alpha^1_{\rm LL}\chi^{\rm K}_{\rm LL}=0$ for an eigenvector of $\beta_{\rm LL}$ (which anticommutes with $\alpha^1_{\rm LL}$); the Wilson terms of the particle part and of the hole part add. After a summation by parts on the ring,*)

(* ::DisplayFormula:: *)
(*E^{\rm R}_{\rm LL}=m_{\rm LL}+\frac{r^{\rm W}_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\,\frac{\sum\big(\varphi^{\rm K}_{\rm LL}((s^1_{\rm LL}+1)\ell^{\rm x}_{\rm LL})-\varphi^{\rm K}_{\rm LL}(s^1_{\rm LL}\ell^{\rm x}_{\rm LL})\big)^2}{\sum\varphi^{\rm K}_{\rm LL}(s^1_{\rm LL}\ell^{\rm x}_{\rm LL})^2}*)

(* ::Text:: *)
(*where both sums run over the sites $s^1_{\rm LL}$ of the ring. For a small spacing the sums become integrals, and $E^{\rm R}_{\rm LL}$ tends to $m_{\rm LL}$ plus $\tfrac12r^{\rm W}_{\rm LL}\ell^{\rm x}_{\rm LL}$ times the ratio of the integral of $(d\varphi^{\rm K}_{\rm LL}/dx^1_{\rm LL})^2$ to the integral of $(\varphi^{\rm K}_{\rm LL})^2$. For $\varphi^{\rm K}_{\rm LL}=1/\cosh(x^1_{\rm LL})$ the two integrals are $2/3$ and $2$, so that $E^{\rm R}_{\rm LL}\to m_{\rm LL}+r^{\rm W}_{\rm LL}\ell^{\rm x}_{\rm LL}/6$: the Wilson term shifts the bound states at first order in the lattice spacing. This is the price for removing the doublers (check GS-16). The check verifies the displayed formula on rings with $\ell^{\rm x}_{\rm LL}=0.4,0.2,0.1,0.05$, the approach of $(E^{\rm R}_{\rm LL}-m_{\rm LL})/(r^{\rm W}_{\rm LL}\ell^{\rm x}_{\rm LL})$ to $1/6$ (with a difference that falls by a factor near 4 at every halving of the spacing), and the decrease of the relative residual of the state (the norm of $\mathcal H^{\rm lat}_{\rm LL}$ applied to the state minus $E^{\rm R}_{\rm LL}$ times the state, divided by the norm of the state).*)

(* ::Text:: *)
(*Finally the check compares the eigenvalues and the densities of the diagonalization at $\ell^{\rm x}_{\rm LL}=0.4$ with the analytic state at second order in the residual of that state, by two theorems of perturbation theory. Take a normalized trial state. Its Rayleigh quotient is its scalar product with the operator applied to it, and its relative residual is the norm of the operator applied to the state minus the Rayleigh quotient times the state. Written in the eigenvectors of the operator, the state has components whose squared moduli are weights with the sum 1; with these weights the mean of the eigenvalues is the Rayleigh quotient, and the mean of the squared differences between the eigenvalues and the Rayleigh quotient is the square of the relative residual. Now let a group of eigenvalues be given, let the Rayleigh quotient lie between the nearest eigenvalue below the group and the nearest one above it, and call the distance the smallest difference between the Rayleigh quotient and an eigenvalue outside the group.*)

(* ::Text:: *)
(***The theorem of Kato and Temple, for a group.** An eigenvalue of the group is not larger than the Rayleigh quotient plus the square of the relative residual divided by the distance, and an eigenvalue of the group is not smaller than the Rayleigh quotient minus the same amount. Proof of the first half: suppose that every eigenvalue of the group exceeded the Rayleigh quotient by more than this amount. For every eigenvalue of the operator form the product of two differences: the eigenvalue minus the nearest eigenvalue below the group, and the eigenvalue minus the Rayleigh quotient minus the amount. For an eigenvalue at or below that neighbour both differences are negative or zero, and for every other eigenvalue both are positive, so no product is negative. But the mean of the products with the weights is the square of the relative residual, minus the amount times the difference between the Rayleigh quotient and the neighbour (the terms of first order in the differences from the Rayleigh quotient have the mean zero); this is zero or negative, because that difference is at least the distance. A mean of numbers that are not negative can be zero or negative only if every number with a weight is zero, that is, if the state lies in the eigenvectors of the neighbour; then its Rayleigh quotient would be the neighbour and not a number above it. This contradiction proves the first half; the second half follows in the same way with the nearest eigenvalue above the group.*)

(* ::Text:: *)
(***The theorem of Davis and Kahan, for a group.** The part of the trial state outside the space of the eigenvectors of the group has at most the norm of the relative residual divided by the distance. Proof: the operator minus the Rayleigh quotient multiplies every component of that part by a number of magnitude at least the distance, and the image of that part is a part of the image of the whole state, whose norm is the relative residual.*)

(* ::Text:: *)
(*Here the group consists of the eight positive bound states, and the trial state is the analytic bound state on the ring of 60 sites. The check verifies that $E^{\rm R}_{\rm LL}$ lies between the neighbours of the group. The moduli of the 16 eigenvalues inside 0.8 times the threshold differ among themselves only by rounding errors (the check prints this spread, the largest minus the smallest modulus), so that by the first theorem all of them lie within the square of the relative residual divided by the distance, plus the spread, of $E^{\rm R}_{\rm LL}$; the check verifies it. It verifies the second theorem directly as well: the norm of the part of the analytic state outside the space of the eight eigenvectors is computed from the scalar products of the state with these eigenvectors. For the densities the check makes a requirement on the scale of the second theorem: the summed density of the eight eigenvectors and the analytic density of the two walls, each normalized to the sum 1 over the sites, are compared by their Bhattacharyya coefficient (the sum over the sites of the square root of their product, which is 1 for equal densities and smaller otherwise), and 1 minus this coefficient must be smaller than half the square of the relative residual divided by the distance (the leading term of 1 minus the cosine of the angle whose sine is that ratio). This is not a consequence of the theorem, because the analytic state of the check is one state of one wall. Figure 5 shows the pair field, the densities, the spectrum and the convergence.*)

(* ::Input:: *)
(* AS-9: a ring of the length 24 with a kink at x^1 = 6 and an antikink at x^1 = 18 *)
as9Ring = 24.;
{as9Xk1, as9Xk2} = {6., 18.};
as9Profile[x_] := asCDinf Tanh[(x - as9Xk1)/asCEllK] Tanh[(as9Xk2 - x)/asCEllK];
(* the distance of two points of the ring, and the profile phi_K of the bound state of a wall at the point wall *)
as9Distance[x_, wall_] := With[{d = Mod[x - wall, as9Ring]}, Min[d, as9Ring - d]];
as9Phi[x_, wall_] := Cosh[as9Distance[x, wall]/asCEllK]^(-asCDinf asCEllK);
(* (1) the dense spectrum at dx = 0.4 (60 sites, a matrix of 960 rows) *)
as9N = 60;
as9Ell = as9Ring/as9N;
as9Lat = <|"n" -> {as9N, 1, 1}, "ell" -> as9Ell, "periodic" -> {True, False, False}, "wilson" -> 1.0, "mass" -> mass, "mu" -> 0.|>;
as9Fields = t2trivialFields[as9N];
Do[as9Fields[["d", site]] = {0., 0., as9Profile[(site - 1) as9Ell]}, {site, as9N}];
as9H = t2dense[as9Lat, as9Fields];
{as9Ev, as9Vec} = t2eigh[as9H];
as9Threshold = Sqrt[mass mass + asCDinf asCDinf];
(* the places of the eigenvalues inside 0.8 times the threshold, and of the positive ones among them *)
as9Inside = Select[Range[Length[as9Ev]], Abs[as9Ev[[#]]] < 0.8 as9Threshold &];
as9Plus = Select[as9Inside, as9Ev[[#]] > 0 &];
(* the rows {index, E_qp} of Figure 5(c): the index counts from 0 *)
as9SpectrumRows = Table[{i - 1, as9Ev[[i]]}, {i, Length[as9Ev]}];
(* the summed density of the eight eigenvectors near +m on every site, and the analytic density of the bound states of
   the two walls, phi_K(kink)^2 + phi_K(antikink)^2 *)
as9Density = Total[Table[Map[Total, Partition[Abs[as9Vec[[All, col]]]^2, 16]], {col, as9Plus}]];
as9DensityExact[x_] := as9Phi[x, as9Xk1]^2 + as9Phi[x, as9Xk2]^2;
as9NormExact = Sum[as9DensityExact[site as9Ell], {site, 0, as9N - 1}];
as9NormLattice = Total[as9Density];
(* the rows {x^1, Delta, density of the lattice, analytic density} of Figure 5(a), (b): both densities normalized to the
   integral 1; and the Bhattacharyya coefficient of the two normalized densities *)
as9ProfileRows = Table[With[{x = site as9Ell},
    {x, as9Profile[x], as9Density[[site + 1]]/as9NormLattice/as9Ell, as9DensityExact[x]/as9NormExact/as9Ell}], {site, 0, as9N - 1}];
as9BC = Sum[Sqrt[(as9Density[[site + 1]]/as9NormLattice) (as9DensityExact[site as9Ell]/as9NormExact)], {site, 0, as9N - 1}];
(* (2) the Rayleigh quotient of the analytic bound state of the kink at x^1 = 6 on rings with dx = 0.4, 0.2, 0.1, 0.05:
   chi_K has the components 1 and 5 equal to 1/sqrt 2 (beta chi_K = chi_K, energy +m) *)
as9Chi = ReplacePart[ConstantArray[0., 8], {1 -> 1/Sqrt[2.], 5 -> 1/Sqrt[2.]}];
as9BetaDev = Norm[asBeta . as9Chi - as9Chi];
as9Eta = I asK3dag . asAlpha[[1]] . as9Chi;
(* for every ring the list {dx, E_R, relative residual, deviation of E_R from the lattice formula, outside part};
   the outside part, computed for the ring of 60 sites only, is the norm of the part of the normalized analytic
   state outside the space of the eight eigenvectors near +m: the square root of 1 minus the sum of its squared
   scalar products with them *)
as9Convergence = Table[
   Module[{ellS = as9Ring/nsites, latS, fieldsS, phis, w, hState, den, er, s1, s0},
    latS = <|"n" -> {nsites, 1, 1}, "ell" -> ellS, "periodic" -> {True, False, False}, "wilson" -> 1.0, "mass" -> mass, "mu" -> 0.|>;
    fieldsS = t2trivialFields[nsites];
    Do[fieldsS[["d", site]] = {0., 0., as9Profile[(site - 1) ellS]}, {site, nsites}];
    phis = Table[as9Phi[site ellS, as9Xk1], {site, 0, nsites - 1}];
    w = Flatten[Table[Join[phis[[site]] as9Chi, phis[[site]] as9Eta], {site, nsites}]];
    hState = t2apply[latS, fieldsS, w];
    den = Total[Abs[w]^2];
    er = Re[Conjugate[w] . hState]/den;
    (* the exact lattice formula E_R = m + (r_W/(2 dx)) sum (phi_K(s+1) - phi_K(s))^2 / sum phi_K(s)^2 *)
    s1 = Total[(RotateLeft[phis] - phis)^2]; s0 = Total[phis^2];
    {ellS, er, Norm[hState - er w]/Sqrt[den], Abs[er - (mass + 1./(2. ellS) s1/s0)],
     If[nsites == as9N, Sqrt[Max[0., 1. - Total[Abs[ConjugateTranspose[as9Vec[[All, as9Plus]]] . w]^2]/den]], 0.]}],
   {nsites, {60, 120, 240, 480}}];
as9DevFormula = Max[as9Convergence[[All, 4]]];
(* the rows {dx, E_R, relative residual} of Figure 5(d) *)
as9ConvergenceRows = as9Convergence[[All, 1 ;; 3]];
(* the shift E_R - m divided by r_W dx tends to 1/6, with an error of second order in dx *)
as9Ratios = (as9Convergence[[All, 2]] - mass)/as9Convergence[[All, 1]];
as9Errors = 1./6. - as9Ratios;
as9FirstOrder = Abs[as9Ratios[[4]] - 1./6.] < 0.01/6. && AllTrue[Partition[as9Errors, 2, 1], 3.5 < #[[1]]/#[[2]] < 4.5 &];
as9ResidualFalls = AllTrue[Partition[as9Convergence[[All, 3]], 2, 1], #[[2]] < 0.6 #[[1]] &];
(* the two theorems at dx = 0.4: the Rayleigh quotient, the relative residual r, the distance delta from E_R to the
   nearest eigenvalue outside the group of the eight positive bound states *)
as9ER = as9Convergence[[1, 2]];
as9Residual = as9Convergence[[1, 3]];
as9Delta = Min[Abs[as9Ev[[Complement[Range[Length[as9Ev]], as9Plus]]] - as9ER]];
(* E_R lies between the neighbours of the group (the eigenvalues are in ascending order, and the group is a run of them) *)
as9Between = as9Plus === Range[First[as9Plus], Last[as9Plus]] && First[as9Plus] > 1 && Last[as9Plus] < Length[as9Ev] &&
   as9Ev[[First[as9Plus] - 1]] < as9ER < as9Ev[[Last[as9Plus] + 1]];
(* Kato and Temple: the 16 moduli lie within r^2/delta + spread of E_R; spread is the largest minus the smallest modulus *)
as9Moduli = Abs[as9Ev[[as9Inside]]];
as9Spread = Max[as9Moduli] - Min[as9Moduli];
as9WorstBound = Max[Abs[as9Moduli - as9ER]];
as9TolE = as9Residual as9Residual/as9Delta + as9Spread;
(* Davis and Kahan: the part of the analytic state outside the eight eigenvectors has at most the norm r/delta *)
as9Outside = as9Convergence[[1, 5]];
as9TolOut = as9Residual/as9Delta;
(* the requirement for the densities *)
as9TolBC = 0.5 (as9Residual/as9Delta)^2;
SL31Check["AS-9", "the bound states of the domain wall on the lattice (case C): on a ring of length 24 (60 sites, dx = 0.4, r_W = 1, m = 0.4, mu = 0) with the kink-antikink profile Delta(x_LL^1) = tanh(x_LL^1 - 6) tanh(18 - x_LL^1), the lattice operator has exactly 16 eigenvalues inside 0.8 times the continuum threshold (m^2 + Delta_inf^2)^(1/2), eight positive and eight negative (four per wall and sign); with the relative residual r of the analytic bound state at dx = 0.4 and the distance delta from its Rayleigh quotient E_R to the nearest eigenvalue outside the cluster of the eight positive bound states, which lies between the neighbours of the cluster: (1) the moduli of the 16 eigenvalues lie within r^2/delta + spread of E_R, where spread is the largest minus the smallest of the 16 moduli (the theorem of Kato and Temple: one eigenvalue of the cluster lies at most r^2/delta above E_R and one at most r^2/delta below); (2) the part of the normalized analytic state outside the space of the eight eigenvectors near +m has at most the norm r/delta (the theorem of Davis and Kahan); (3) the summed density of the eight states near +m agrees with the analytic density of the bound states of the two walls at second order in r, 1 - (Bhattacharyya coefficient of the two normalized densities) < (1/2) (r/delta)^2 (a requirement of the check on the scale of (2), not a theorem; the Bhattacharyya coefficient is the sum over the sites of the square root of the product of the two densities, each normalized to the sum 1); for the analytic bound state of the kink (chi_K with the components 1 and 5 equal to 1/sqrt 2 and the others zero, beta_LL chi_K = chi_K) on rings with dx = 0.4, 0.2, 0.1, 0.05 the Rayleigh quotient E_R equals the exact lattice formula m + (r_W/(2 dx)) sum (phi_K(s+1) - phi_K(s))^2 / sum phi_K(s)^2 (tolerance 1e-12), (E_R - m)/(r_W dx) tends to 1/6 (within 1 per cent at dx = 0.05, the difference decreasing by a factor between 3.5 and 4.5 at each halving of dx: the Wilson term shifts the bound state at first order in dx, by r_W dx/6, with a correction of order dx^3), and the relative residual decreases at least by the factor 0.6 at each halving of dx",
  Length[as9Inside] == 16 && Length[as9Plus] == 8 && as9Between && as9WorstBound <= as9TolE && as9Outside <= as9TolOut && 1. - as9BC < as9TolBC && as9DevFormula < 1*^-12 && as9FirstOrder && as9ResidualFalls &&
   as9BetaDev < 1*^-15,
  Row[{Length[as9Inside], " bound states (", Length[as9Plus], " positive); largest | |E| - E_R | = ", asFix[as9WorstBound, 5], " (bound r^2/delta + spread = ", asFix[as9TolE, 5], ", delta = ",
    asFix[as9Delta, 4], ", spread = ", asSci[as9Spread], "); part of the analytic state outside the eight eigenvectors ", asFix[as9Outside, 4], " (bound r/delta = ",
    asFix[as9TolOut, 4], "); 1 - Bhattacharyya coefficient = ", ScientificForm[1. - as9BC, 3], " (required below ", ScientificForm[as9TolBC, 3], "); E_R = ",
    Row[Riffle[Map[asFix[#, 6] &, as9Convergence[[All, 2]]], ", "]], "; (E_R - m)/dx = ", Row[Riffle[Map[asFix[#, 5] &, as9Ratios], ", "]], "; residuals ",
    Row[Riffle[Map[ScientificForm[#, 3] &, as9Convergence[[All, 3]]], ", "]], "; deviation from the lattice formula ", asSci[as9DevFormula],
    " (original: 16 bound states (8 positive); 0.00312 (bound 0.02260, delta = 0.6139, spread = 3.8e-14); 0.0841 (bound 0.1919); 1.12e-3 (required below 1.84e-2); E_R = 0.465443, 0.433178, 0.416647, 0.408331; (E_R - m)/dx = 0.16361, 0.16589, 0.16647, 0.16662; residuals 1.18e-1, 5.94e-2, 2.98e-2, 1.49e-2; 3.8e-15)"}]]

(* ::Subsection:: *)
(*Case D: Landau levels in constant magnetic fields (check AS-10)*)

(* ::Text:: *)
(***The fields.** No pair field, $\mu_{\rm LL}=0$, and constant magnetic fields along $x^3_{\rm LL}$ of the fermion number and of the third isospin direction: the field strengths $\mathsf F^{\rm F}_{{\rm LL},12}$ and $\mathsf F^3_{{\rm LL},12}$, in the Landau gauge $A^{\rm F}_{{\rm LL},2}=\mathsf F^{\rm F}_{{\rm LL},12}x^1_{\rm LL}$, $A^3_{{\rm LL},2}=\mathsf F^3_{{\rm LL},12}x^1_{\rm LL}$ (all other potentials are zero). On the isospin-up components the potential is $\mathsf A^\uparrow_{{\rm LL},2}=B^\uparrow_{\rm LL}x^1_{\rm LL}$ and on the isospin-down components $\mathsf A^\downarrow_{{\rm LL},2}=B^\downarrow_{\rm LL}x^1_{\rm LL}$, with $B^\uparrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}+\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}$ and $B^\downarrow_{\rm LL}=g^{\rm F}_{\rm LL}\mathsf F^{\rm F}_{{\rm LL},12}-\tfrac12g^{\rm v}_{\rm LL}\mathsf F^3_{{\rm LL},12}$: the two isospin components feel different magnetic fields.*)

(* ::Text:: *)
(***The solution.** Square the covariant Dirac Hamiltonian $h^\nabla_{\rm LL}=-i\sum_j\alpha^j_{\rm LL}\nabla^{\rm LL}_j+\beta_{\rm LL}m_{\rm LL}$. The terms with $\alpha^j_{\rm LL}\beta_{\rm LL}$ cancel, the terms with twice the same $\alpha^j_{\rm LL}$ give $\sum_j(-i\nabla^{\rm LL}_j)^2$, and the terms with two different matrices combine into commutators of covariant derivatives, which are the field strengths (check GS-6), multiplied by $\alpha^j_{\rm LL}\alpha^{j'}_{\rm LL}=-2\,\mathsf s_{\rm LL}(J^{j''}_{\rm LL})$ for the cyclic triples $(j,j',j'')$ (check GS-2). Here only the field strengths with the indices 12 differ from zero. On the isospin-up components, and on $\exp(i(q^2_{\rm LL}x^2_{\rm LL}+q^3_{\rm LL}x^3_{\rm LL}))$ times a function of $x^1_{\rm LL}$, the square is*)

(* ::DisplayFormula:: *)
(*(h^\nabla_{\rm LL})^2=m_{\rm LL}^2+(q^3_{\rm LL})^2+\Big(-\frac{d^2}{d(x^1_{\rm LL})^2}+(B^\uparrow_{\rm LL})^2\big(x^1_{\rm LL}-q^2_{\rm LL}/B^\uparrow_{\rm LL}\big)^2\Big)-2B^\uparrow_{\rm LL}\,i\,\mathsf s_{\rm LL}(J^3_{\rm LL})*)

(* ::Text:: *)
(*where $i\,\mathsf s_{\rm LL}(J^3_{\rm LL})=\tfrac12\mathrm{diag}(\sigma^{\rm LL}_3,\sigma^{\rm LL}_3)$ is the spin of the spinor charge along $x^3_{\rm LL}$. The large bracket is a harmonic oscillator centred at $x^1_{\rm LL}=q^2_{\rm LL}/B^\uparrow_{\rm LL}$, with the eigenvalues $\lvert B^\uparrow_{\rm LL}\rvert$ times $1,3,5,\dots$; the spin has the eigenvalues $\pm\tfrac12$ and commutes with the bracket. The sum of the oscillator eigenvalue and of the spin term $\mp B^\uparrow_{\rm LL}$ is $2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert$ with the number $n^{\rm L}_{\rm LL}=0,1,2,\dots$ of the Landau level: the level 0 arises only from the lowest oscillator state with the spin parallel to the field, and every level $n^{\rm L}_{\rm LL}\ge1$ from two combinations (the oscillator state with the number $n^{\rm L}_{\rm LL}$ and the parallel spin, and the oscillator state with the number $n^{\rm L}_{\rm LL}-1$ and the antiparallel spin). The energies are therefore*)

(* ::DisplayFormula:: *)
(*E^{\rm qp}_{{\rm LL},n}=\pm\big(m_{\rm LL}^2+(q^3_{\rm LL})^2+2n^{\rm L}_{\rm LL}\lvert B^\uparrow_{\rm LL}\rvert\big)^{1/2},\qquad n^{\rm L}_{\rm LL}=0,1,2,\dots*)

(* ::Text:: *)
(*for every $q^2_{\rm LL}$ (the position of the centre: the levels are degenerate), and the same with $B^\downarrow_{\rm LL}$ for isospin down. At $q^3_{\rm LL}=0$ the level 0 has the energies $+m_{\rm LL}$ and $-m_{\rm LL}$ once each, and every level $n^{\rm L}_{\rm LL}\ge1$ has each of its two energies twice. The level 0 does not depend on the field: the energy of the spin cancels the zero-point energy of the oscillator exactly, because the magnetic moment of a Dirac particle has the gyromagnetic factor 2, for the charge of the fermion number and for the isospin charge alike. Check AS-10 diagonalizes $h^\nabla_{\rm LL}$ at $q^2_{\rm LL}=q^3_{\rm LL}=0$, which for one isospin component is the $4\times4$ matrix operator $\alpha^1_{\rm LL}(-i\,d/dx^1_{\rm LL})-\alpha^2_{\rm LL}B^\uparrow_{\rm LL}x^1_{\rm LL}+\beta_{\rm LL}m_{\rm LL}$ (and the same with $B^\downarrow_{\rm LL}$), in a basis of 120 oscillator states of the frequency $\lvert B^\uparrow_{\rm LL}\rvert$ (or $\lvert B^\downarrow_{\rm LL}\rvert$). The fields are $\mathsf F^{\rm F}_{{\rm LL},12}=1.2$ and $\mathsf F^3_{{\rm LL},12}=1.0$, so that $B^\uparrow_{\rm LL}=0.89$ and $B^\downarrow_{\rm LL}=0.19$. In the oscillator basis the position and the momentum are matrices whose only entries stand next to the diagonal (the comments of the code state them). The exact eigenstates of the level $n^{\rm L}_{\rm LL}$ contain only the oscillator states with the numbers $n^{\rm L}_{\rm LL}-1$ and $n^{\rm L}_{\rm LL}$, so the basis, whose states have the numbers 0 to 119, represents the levels 0 to 119 exactly (the level 0 contains only the oscillator state 0). It has no state with the number 120: the oscillator state 119 with the antiparallel spin has lost its partner, and the truncated matrix has, instead of the level 120, two states with the energies $+m_{\rm LL}$ and $-m_{\rm LL}$ that live on the oscillator state 119. These two states are artefacts of the truncation. The check discards every eigenvector with a weight above $10^{-8}$ on the ten highest oscillator states (the numbers 110 to 119): for each isospin component these are the two artefacts and the forty states of the levels 110 to 119, which the check does not need. It then compares the remaining energies with the formula for the levels 0 to 10, with their multiplicities, and requires that no other energy lies below the level 10. Figure 6 shows the levels.*)

(* ::Input:: *)
(* AS-10: the field strengths FF_12 (fermion number) and Fv^3_12 (third isospin direction); 120 oscillator states *)
{as10FF12, as10Fv12} = {1.2, 1.0};
as10Nosc = 120;
as10Dev = 0.;
as10CountOk = True;
(* the numbers of discarded eigenvectors: all; those whose energy is that of a Landau level 110 to 119; those with the
   energy +m or -m (the artefacts of the truncation) *)
{as10Discarded, as10DiscardedLevels, as10DiscardedAtMass} = {0, 0, 0};
(* the rows {Q_LL, n_L, E_qp, source} of Figure 6; source 1: the diagonalization, 0: the formula *)
as10LandauRows = {};
Do[
  Module[{field = gf as10FF12 + qq gv as10Fv12, fieldAbs, roots, xmat, pmat, hLandau, ev, vecs, weightTop, kept, dropped},
   (* the magnetic field B_up (qq = +1/2) or B_down (qq = -1/2) that the isospin component feels *)
   fieldAbs = Abs[field];
   (* the oscillator basis of the frequency |B|, states 0..119: with the lowering operator a (entries sqrt(1), sqrt(2),
      ..., sqrt(119) above the diagonal) the position is x^1 = (a + a^dag)/sqrt(2|B|) and the momentum is
      p_1 = i sqrt(|B|/2) (a^dag - a) *)
   roots = Sqrt[N[Range[as10Nosc - 1]]];
   xmat = DiagonalMatrix[roots/Sqrt[2. fieldAbs], 1] + DiagonalMatrix[roots/Sqrt[2. fieldAbs], -1];
   pmat = DiagonalMatrix[I Sqrt[fieldAbs/2.] roots, -1] + DiagonalMatrix[-I Sqrt[fieldAbs/2.] roots, 1];
   (* h = alpha^1 p_1 + alpha^2 (-B x^1) + beta m: the Dirac index is the slower one, the oscillator number the faster *)
   hLandau = KroneckerProduct[t2alpha[1], pmat] + KroneckerProduct[t2alpha[2], -field xmat] + mass KroneckerProduct[t2beta, IdentityMatrix[as10Nosc]];
   {ev, vecs} = t2eigh[hLandau];
   (* the weight of every eigenvector on the ten highest oscillator states (of all four Dirac components) *)
   weightTop = Table[Total[Abs[Flatten[Partition[vecs[[All, col]], as10Nosc][[All, as10Nosc - 9 ;; as10Nosc]]]]^2], {col, Length[ev]}];
   kept = Pick[ev, Map[# < 1.*^-8 &, weightTop]];
   dropped = Pick[ev, Map[! (# < 1.*^-8) &, weightTop]];
   as10Discarded += Length[dropped];
   as10DiscardedLevels += Count[dropped, energy_ /; Min[Abs[Abs[energy] - Table[Sqrt[mass mass + 2. level fieldAbs], {level, as10Nosc - 10, as10Nosc - 1}]]] < 1.*^-8];
   as10DiscardedAtMass += Count[dropped, energy_ /; Abs[Abs[energy] - mass] < 1.*^-8];
   Do[
    Module[{mine = Sort[Abs[Select[kept, # sg > 0 &]]], expect},
     (* the level 0 once, the levels 1 to 10 twice *)
     expect = Prepend[Flatten[Table[ConstantArray[Sqrt[mass mass + 2. level fieldAbs], 2], {level, 1, 10}]], mass];
     If[Length[mine] <= Length[expect],
      as10CountOk = False,
      as10Dev = Max[as10Dev, Max[Abs[mine[[1 ;; Length[expect]]] - expect]]];
      (* the next kept energy lies above the level 10 *)
      If[mine[[Length[expect] + 1]] < Last[expect] + 1.*^-9, as10CountOk = False];
      Do[AppendTo[as10LandauRows, {qq, If[i == 1, 0, Quotient[i, 2]], sg mine[[i]], 1}], {i, Length[expect]}];
      Do[AppendTo[as10LandauRows, {qq, level, sg Sqrt[mass mass + 2. level fieldAbs], 0}], {level, 0, 10}]]],
    {sg, {1., -1.}}]],
  {qq, {0.5, -0.5}}];
{as10Bup, as10Bdown} = {gf as10FF12 + 0.5 gv as10Fv12, gf as10FF12 - 0.5 gv as10Fv12};
SL31Check["AS-10", "Landau levels (case D): for one isospin component (Q_LL = +1/2 or -1/2) in the constant magnetic fields FF_LL_12 = 1.2 of U(1)_F and Fv_LL^3_12 = 1.0 of the third isospin direction (Landau gauge AF_LL_2 = FF_LL_12 x_LL^1, Av_LL^3_2 = Fv_LL^3_12 x_LL^1; g_v = 0.7, g_F = 0.45), the component feels the magnetic field B_up = g_F FF_LL_12 + g_v Fv_LL^3_12/2 or B_down = g_F FF_LL_12 - g_v Fv_LL^3_12/2; the Dirac Hamiltonian alpha_LL^1 p_1 + alpha_LL^2 (p_2 - B_up x_LL^1) + beta_LL m at p_2 = p_3 = 0 (and the same with B_down), with the momentum operators p_j = -i d_j, diagonalized in a basis of 120 oscillator states of the frequency |B_up| (or |B_down|) (the states with a weight above 1e-8 on the ten highest oscillator states are discarded: for each isospin component they are the 40 states of the levels 110 to 119, which the check does not use, and the two artefacts of the truncation, which have the energies +m and -m and live on the highest oscillator state), has the energies +m and -m once each and +-(m^2 + 2 n_L |B_up|)^(1/2) (or with |B_down|) twice each for n_L = 1 to 10, and no other energy below the level 10 (m = 0.4; tolerance 1e-10)",
  as10Dev < 1*^-10 && as10CountOk && as10Discarded == 84 && as10DiscardedLevels == 80 && as10DiscardedAtMass == 4,
  Row[{"largest deviation ", asSci[as10Dev], "; B_up = ", asFix[as10Bup, 3], ", B_down = ", asFix[as10Bdown, 3], "; ", as10Discarded, " discarded states, of them ", as10DiscardedLevels,
    " at the levels 110 to 119 and ", as10DiscardedAtMass, " artefacts at +-m (original: 1.2e-14; B_up = 0.890, B_down = 0.190; 84 discarded states, of them 80 at the levels 110 to 119 and 4 artefacts at +-m)"}]]

(* ::Subsection:: *)
(*The data files of the original program*)

(* ::Text:: *)
(*The original program t2_analytic wrote the numbers of its figures into eleven data files. The next cell contains every number of these files, copied without change of a digit. It is a closed cell: the notebook evaluates it like every other cell, but shows only its thin cell bracket, and a printed copy of the notebook leaves it out, so that the notebook stays complete without many pages of numbers. To read the numbers, select the bracket of the cell and choose Cell > Cell Properties > Open. Its output, the next cell that is shown, is the list of the numbers of rows of the eleven lists. Two changes of form were made in the copy: a number with an exponent is written in the notation of the Wolfram Language, and a column of words is coded by numbers (the branch positive or negative of the curves of case A as $1$ or $-1$; the source of a value as 0 for a closed form or a formula and 1 for a quadrature or a numerical diagonalization).*)

(* ::Text:: *)
(*The eleven lists, with their files and columns: `t2origACurves` (t2_A_dispersion_curves.csv: momentum, $Q_{\rm LL}$, branch, energy; the curves of Figure 3(a)); `t2origALattice` (t2_A_dispersion_lattice.csv: momentum, $Q_{\rm LL}$, eigenvalue; the points of Figure 3(a)); `t2origAPrecession` (t2_A_precession.csv: time, $P^\uparrow_{\rm LL}$ of the integration by CVODE, $P^\uparrow_{\rm LL}$ exact; Figure 3(b)); `t2origBCurves` (t2_B_bogoliubov_curves.csv: momentum, $\varepsilon^{\rm lat}_{\rm LL}$, $E^-_{\rm LL}$, $E^+_{\rm LL}$; the curves of Figure 4(a)); `t2origBLattice` (t2_B_bogoliubov_lattice.csv: momentum, positive eigenvalue; the points of Figure 4(a)); `t2origBGap` (t2_B_gap.csv: $\mu_{\rm LL}$, $G_{\rm LL}/G^{\rm crit}_{\rm LL}$, $\Delta_{\rm LL}$, source; Figure 4(c)); `t2origBRabi` (t2_B_rabi.csv: time, $P^{\rm h}_{\rm LL}$ of the integration by CVODE, $P^{\rm h}_{\rm LL}$ exact; Figure 4(b)); `t2origCConvergence` (t2_C_kink_convergence.csv: $\ell^{\rm x}_{\rm LL}$, $E^{\rm R}_{\rm LL}$, relative residual; Figure 5(d)); `t2origCProfile` (t2_C_kink_profile.csv: $x^1_{\rm LL}$, $\Delta_{\rm LL}$, density of the lattice eigenvectors, analytic density; Figure 5(a) and (b)); `t2origCSpectrum` (t2_C_kink_spectrum.csv: number of the eigenvalue counted from 0, eigenvalue; Figure 5(c)); `t2origDLandau` (t2_D_landau.csv: $Q_{\rm LL}$, $n^{\rm L}_{\rm LL}$, energy, source; Figure 6). The files print energies, momenta, positions, probabilities and densities with 10 decimals, the times with 4 decimals, the ratio $G_{\rm LL}/G^{\rm crit}_{\rm LL}$ with 5 decimals and the relative residual with 7 significant digits.*)

(* ::Input::Closed:: *)
(* @@INCLUDE t2_analytic_data.wl@@ *)

(* ::Text:: *)
(*Check T2-DATA-2 compares every list, row by row and column by column, with the rows that this chapter computed (`as1CurveRows`, `as1LatticeRows`, `as2PrecessionRows`, `as3CurveRows`, `as3LatticeRows`, `as5GapRows`, `as4RabiRows`, `as9ConvergenceRows`, `as9ProfileRows`, `as9SpectrumRows`, `as10LandauRows`). A column that the file prints with 10 decimals may deviate by half a unit of the last printed decimal, $5\times10^{-11}$ (the check allows $5.1\times10^{-11}$); the times may deviate by $5.1\times10^{-5}$, the ratio of the couplings by $5.1\times10^{-6}$, and the relative residual by the fraction $5.1\times10^{-7}$ of itself. The charges, the branches, the numbers of the levels and of the eigenvalues, and the sources must be equal. A larger deviation would mean that the rewritten program computes different numbers. Two columns hold results of the integration by CVODE, which the notebook replaces by IDA: the probabilities $P^\uparrow_{\rm LL}$ and $P^{\rm h}_{\rm LL}$ of the integrated states. They need not agree to the last decimal, but within the accuracy of the two integrations. If a computed state has the relative deviation $\delta^{\rm rel}_{\rm LL}$ from the exact state, the two states, each divided by its norm, differ by at most $2\delta^{\rm rel}_{\rm LL}$ (the norms of the two states differ by at most $\delta^{\rm rel}_{\rm LL}$ times the norm of the exact state, and so do the states). A probability is the square of the norm of the part of the normalized state in the isospin-up (or the hole) components; these norms are at most 1 and differ by at most $2\delta^{\rm rel}_{\rm LL}$, so the probabilities differ by at most $4\delta^{\rm rel}_{\rm LL}$. The probability of IDA and that of CVODE therefore differ by at most 4 times the sum of their two largest relative deviations, plus the rounding of the file. The largest relative deviation of IDA is the one measured in check AS-2 or AS-4; that of CVODE is taken from the original report, $1.3\times10^{-11}$ and $4.7\times10^{-10}$, which are printed with two digits, so that the bounds $1.35\times10^{-11}$ and $4.75\times10^{-10}$ are used.*)

(* ::Input:: *)
(* two lists of rows have the same dimensions and equal columns cols (charges, branches, numbers of levels, sources) *)
t2rowsEqual[rowsHere_, rowsOriginal_, cols_] := Dimensions[rowsHere] === Dimensions[rowsOriginal] &&
   TrueQ[Max[Abs[rowsHere[[All, cols]] - rowsOriginal[[All, cols]]]] == 0];
(* the columns printed with 10 decimals: the largest deviation in every file *)
asDataDev10 = {t2rowsDeviation[as1CurveRows, t2origACurves, {1, 4}], t2rowsDeviation[as1LatticeRows, t2origALattice, {1, 3}],
   t2rowsDeviation[as2PrecessionRows, t2origAPrecession, {3}], t2rowsDeviation[as3CurveRows, t2origBCurves, {1, 2, 3, 4}],
   t2rowsDeviation[as3LatticeRows, t2origBLattice, {1, 2}], t2rowsDeviation[as5GapRows, t2origBGap, {3}],
   t2rowsDeviation[as4RabiRows, t2origBRabi, {3}], t2rowsDeviation[as9ConvergenceRows, t2origCConvergence, {1, 2}],
   t2rowsDeviation[as9ProfileRows, t2origCProfile, {1, 2, 3, 4}], t2rowsDeviation[as9SpectrumRows, t2origCSpectrum, {2}],
   t2rowsDeviation[as10LandauRows, t2origDLandau, {3}]};
(* the times (4 decimals), the ratio of the couplings (5 decimals), the relative residual (7 significant digits) *)
asDataDevTimes = Max[t2rowsDeviation[as2PrecessionRows, t2origAPrecession, {1}], t2rowsDeviation[as4RabiRows, t2origBRabi, {1}]];
asDataDevRatio = t2rowsDeviation[as5GapRows, t2origBGap, {2}];
asDataDevResidual = If[Dimensions[as9ConvergenceRows] === Dimensions[t2origCConvergence],
   Max[Abs[as9ConvergenceRows[[All, 3]]/t2origCConvergence[[All, 3]] - 1.]], Infinity];
(* the columns that must be equal *)
asDataEqual = t2rowsEqual[as1CurveRows, t2origACurves, {2, 3}] && t2rowsEqual[as1LatticeRows, t2origALattice, {2}] &&
   t2rowsEqual[as5GapRows, t2origBGap, {1, 4}] && t2rowsEqual[as9SpectrumRows, t2origCSpectrum, {1}] && t2rowsEqual[as10LandauRows, t2origDLandau, {1, 2, 4}];
(* the probabilities of the integrated states: IDA here, CVODE in the files; the bounds 4 (delta_IDA + delta_CVODE) + 5.1e-11 *)
asDataDevPup = t2rowsDeviation[as2PrecessionRows, t2origAPrecession, {2}];
asDataDevPhole = t2rowsDeviation[as4RabiRows, t2origBRabi, {2}];
asDataBoundPup = 4. (as2Dev + 1.35*^-11) + 5.1*^-11;
asDataBoundPhole = 4. (as4Dev + 4.75*^-10) + 5.1*^-11;
(* the number of rows of the eleven lists of the data cell *)
asDataRows = Total[Map[Length, {t2origACurves, t2origALattice, t2origAPrecession, t2origBCurves, t2origBLattice, t2origBGap, t2origBRabi, t2origCConvergence,
     t2origCProfile, t2origCSpectrum, t2origDLandau}]];
SL31Check["T2-DATA-2", "the numbers of the figures of item [2] agree with the eleven data files written by the original program t2_analytic (t2_A_dispersion_curves, t2_A_dispersion_lattice, t2_A_precession, t2_B_bogoliubov_curves, t2_B_bogoliubov_lattice, t2_B_gap, t2_B_rabi, t2_C_kink_convergence, t2_C_kink_profile, t2_C_kink_spectrum, t2_D_landau; 4725 rows): every column printed with 10 decimals within 5.1e-11, half a unit of the last printed decimal; the times within 5.1e-5, G/G_crit within 5.1e-6, the relative residual within the fraction 5.1e-7; the charges, branches, numbers of levels and of eigenvalues and the sources are equal; the probabilities P_up and P_hole of the integrated states, computed here with IDA and in the files with CVODE, differ by at most 4 (max_deviation of IDA + max_deviation of CVODE) + 5.1e-11, the bound that follows from the relative deviations of the two integrations from the exact solution",
  asDataRows == 4725 && Max[asDataDev10] < 5.1*^-11 && asDataDevTimes < 5.1*^-5 && asDataDevRatio < 5.1*^-6 && asDataDevResidual < 5.1*^-7 && asDataEqual &&
   asDataDevPup <= asDataBoundPup && asDataDevPhole <= asDataBoundPhole,
  Row[{asDataRows, " rows; largest deviations of the columns with 10 decimals, file by file: ", asSci[asDataDev10], "; times ", asSci[asDataDevTimes], "; G/G_crit ", asSci[asDataDevRatio],
    "; relative residual ", asSci[asDataDevResidual], "; equal columns: ", asDataEqual, "; P_up of IDA and of CVODE differ by at most ", asSci[asDataDevPup], " (bound ",
    asSci[asDataBoundPup], "), P_hole by at most ", asSci[asDataDevPhole], " (bound ", asSci[asDataBoundPhole], ")"}]]

(* ::Subsection:: *)
(*The figures of item [2]*)

(* ::Text:: *)
(*The Figures 3 to 6 are the figures of the original program, drawn here from the rows that this chapter computed (the comments of the cells name the lists). The panels of a figure stand one below the other, each with its legend, so that no figure is wider than the printed page; the Figures 4 and 5 take two cells each. A curve is a formula or an exact solution; a point is a result of a computation on the lattice (an eigenvalue, a state of an integration, a density). In the two panels with a time integration, Figure 3(b) and Figure 4(b), the points alternate between the integration by IDA in this notebook and the integration by CVODE in the original program, whose numbers stand in the data cell above.*)

(* ::Input:: *)
(* the registered symbols of the labels of the Figures 3 to 6 (Part I); the symbols mSym, ellxSym, rWSym, q1Sym,
   epsLatSym and x0Sym are those of the figures of chapter II.2 *)
{eqpSym, qChargeSym, acalUpSym, acalDownSym, pUpSym, omegaPrSym, av10Sym} = {LL["E^{\\rm qp}_{{\\rm LL},n}"], LL["Q_{\\rm LL}"],
   LL["\\mathsf A^\\uparrow_{{\\rm LL},p}", "p" -> "1"], LL["\\mathsf A^\\downarrow_{{\\rm LL},p}", "p" -> "1"], LL["P^\\uparrow_{\\rm LL}"],
   LL["\\omega^{\\rm pr}_{\\rm LL}"], LL["A^\\iota_{{\\rm LL},p}", "\\iota" -> "1", "p" -> "0"]};
{ePolarMinusSym, ePolarPlusSym, muSym, deltaSym, pHoleSym, gCouplingSym, gCritSym, lambdaSym} = {LL["E^\\pm_{\\rm LL}", "\\pm" -> "-"],
   LL["E^\\pm_{\\rm LL}", "\\pm" -> "+"], LL["\\mu_{\\rm LL}"], LL["\\Delta_{\\rm LL}"], LL["P^{\\rm h}_{\\rm LL}"], LL["G_{\\rm LL}"],
   LL["G^{\\rm crit}_{\\rm LL}"], LL["\\Lambda_{\\rm LL}"]};
{x1Sym, phiKSym, eRSym, nLSym, bUpSym, bDownSym} = {LL["x^p_{\\rm LL}", "p" -> "1"], LL["\\varphi^{\\rm K}_{\\rm LL}"], LL["E^{\\rm R}_{\\rm LL}"],
   LL["n^{\\rm L}_{\\rm LL}"], LL["B^\\uparrow_{\\rm LL}"], LL["B^\\downarrow_{\\rm LL}"]};
(* the style of a curve, of a dashed reference curve, and the common options of the panels *)
asLine[color_] := Directive[color, Thickness[0.004]];
asDashed[color_] := Directive[color, Dashed, Thickness[0.004]];
asPanel = {Axes -> False, Frame -> True, GridLines -> Automatic, GridLinesStyle -> Directive[GrayLevel[0.9]], LabelStyle -> Directive[FontSize -> 11]};
(* the size of a panel: its width is smaller than the width of the printed page, and two panels with their legends
   fit on one page one below the other *)
asSize = {ImageSize -> 480, AspectRatio -> 0.55};
(* a text inside a panel, in the muted grey of the texts; a title of a panel *)
asNote[e_] := upright[Style[e, 10, muted]];
asTitle[e_] := upright[Style[e, 12]];
(* an entry of a legend for points: the marker of the plot in its color, and its label; a row of such entries *)
asPointKey[marker_String, color_, label_] := Row[{Style[marker, color, 13], " ", upright[Style[label, 10]]}];
asKeys[entries__] := Row[{entries}, Spacer[16]];
(* a figure: the caption, and below it the panels and their legends, one below the other *)
asFigure[caption_String, parts__] := Labeled[Column[{parts}, Center, Spacings -> 0.8], Style[caption, 11], Top];

(* ::Input:: *)
(* Figure 3(a): the energies of the particle block in constant potentials.  Curves: the formula (rows as1CurveRows);
   circles: the eigenvalues of the lattice operator (rows as1LatticeRows); blue: isospin up, orange: isospin down.
   The dotted lines mark the momenta Acal_up_1 and Acal_down_1 at which the bands have their lowest energy *)
fig3aCurves = Flatten[Table[Map[{#[[1]] as1Ell, #[[4]]} &, Select[as1CurveRows, #[[2]] == qq && #[[3]] == branch &]], {qq, {0.5, -0.5}}, {branch, {1, -1}}], 1];
fig3aPoints = Table[Map[{#[[1]] as1Ell, #[[3]]} &, Select[as1LatticeRows, #[[2]] == qq &]], {qq, {0.5, -0.5}}];
fig3a = Show[
   ListLinePlot[fig3aCurves, PlotStyle -> {asLine[blue], asLine[blue], asLine[orange], asLine[orange]}],
   ListPlot[fig3aPoints, PlotStyle -> {blue, orange}, PlotMarkers -> {"\[EmptyCircle]", 9}],
   Graphics[{muted, Dotted, Line[{{as1Acal1[0.5] as1Ell, -3.4}, {as1Acal1[0.5] as1Ell, 3.4}}], Line[{{as1Acal1[-0.5] as1Ell, -3.4}, {as1Acal1[-0.5] as1Ell, 3.4}}],
     Text[asNote[Row[{q1Sym, " = ", acalUpSym}]], {as1Acal1[0.5] as1Ell + 0.05, 2.9}, {-1, 0}],
     Text[asNote[Row[{q1Sym, " = ", acalDownSym}]], {as1Acal1[-0.5] as1Ell - 0.05, 2.9}, {1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {{-Pi/2, Pi/2}, {-3.4, 3.4}},
   FrameLabel -> {upright[Row[{q1Sym, " ", ellxSym, " (momentum times lattice spacing)"}]], upright[Row[{eqpSym, " (particle block)"}]]},
   FrameTicks -> {{Automatic, None}, {{{-Pi/2, "-\[Pi]/2"}, {-Pi/4, "-\[Pi]/4"}, {0, "0"}, {Pi/4, "\[Pi]/4"}, {Pi/2, "\[Pi]/2"}}, None}},
   Sequence @@ asSize, PlotLabel -> asTitle["(a) constant potentials: momentum and energy shifted"]];
fig3aLegend = Column[{LineLegend[{asLine[blue], asLine[orange]},
     Map[upright, {Row[{qChargeSym, " = +1/2 (isospin up), formula"}], Row[{qChargeSym, " = \[Minus]1/2 (isospin down), formula"}]}], LegendLayout -> "Row"],
    asKeys[asPointKey["\[EmptyCircle]", blue, "isospin up, lattice operator"], asPointKey["\[EmptyCircle]", orange, "isospin down, lattice operator"]]}, Center];
(* Figure 3(b): the probability of isospin up.  Grey line: the exact solution; filled blue circles: IDA in this notebook
   (the outputs number 0, 4, 8, ...); open orange squares: CVODE in the original program (the outputs number 2, 6, 10,
   ..., from the data cell).  The two sets of points alternate, so that neither covers the other *)
fig3bExact = Table[{x0, Cos[0.5 as2OmegaPr x0]^2}, {x0, 0., 16., 0.04}];
fig3bIda = as2PrecessionRows[[1 ;; -1 ;; 4, {1, 2}]];
fig3bCvode = t2origAPrecession[[3 ;; -1 ;; 4, {1, 2}]];
fig3b = Show[
   ListLinePlot[fig3bExact, PlotStyle -> asLine[grey]],
   ListPlot[{fig3bIda, fig3bCvode}, PlotStyle -> {blue, orange}, PlotMarkers -> {{"\[FilledCircle]", 9}, {"\[EmptySquare]", 9}}],
   Graphics[{muted, Dotted, Line[{{2 Pi/as2OmegaPr, -0.05}, {2 Pi/as2OmegaPr, 1.1}}],
     Text[asNote[Row[{"period 2\[Pi]/", omegaPrSym}]], {2 Pi/as2OmegaPr + 0.25, 1.05}, {-1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {{0, 16}, {-0.05, 1.1}},
   FrameLabel -> {upright[Row[{x0Sym, " (time)"}]], upright[Row[{pUpSym, " (probability of isospin up)"}]]},
   Sequence @@ asSize, PlotLabel -> asTitle[Row[{"(b) isospin precession, ", av10Sym, " = 1.5"}]]];
fig3bLegend = Column[{LineLegend[{asLine[grey]}, {upright[Row[{"exact: ", Superscript["cos", 2], "(", omegaPrSym, " ", x0Sym, "/2)"}]]}],
    asKeys[asPointKey["\[FilledCircle]", blue, "IDA (this notebook)"], asPointKey["\[EmptySquare]", orange, "CVODE (original program)"]]}, Center];
asFigure["Figure 3. Case A, free spinor charges in constant gauge potentials (item [2], checks AS-1 and AS-2).", fig3a, fig3aLegend, fig3b, fig3bLegend]

(* ::Input:: *)
(* Figure 4(a): the positive energies of the uniform polar superfluid on the lattice.  Curves: the formulas with eps_lat
   (rows as3CurveRows); circles: the eigenvalues of the lattice operator (rows as3LatticeRows) *)
fig4aX = as3CurveRows[[All, 1]] asBEll;
(* the Fermi momentum of the lattice, where E^- has its minimum Delta (the left one of the two minima is marked) *)
fig4aFermi = First[MinimalBy[Select[as3CurveRows, #[[1]] < 0 &], #[[3]] &]][[1]] asBEll;
fig4a = Show[
   ListLinePlot[{Transpose[{fig4aX, Abs[as3CurveRows[[All, 2]] - asBMu]}], Transpose[{fig4aX, as3CurveRows[[All, 3]]}], Transpose[{fig4aX, as3CurveRows[[All, 4]]}]},
    PlotStyle -> {asDashed[grey], asLine[blue], asLine[orange]}],
   ListPlot[Map[{#[[1]] asBEll, #[[2]]} &, as3LatticeRows], PlotStyle -> muted, PlotMarkers -> {"\[EmptyCircle]", 9}],
   Graphics[{muted, Thickness[0.002], Arrowheads[0.025], Arrow[{{-0.72, 0.3}, {fig4aFermi - 0.03, asBDelta}}],
     Text[asNote[Column[{Row[{"gap ", deltaSym, " at the Fermi momentum"}], Row[{"(where ", epsLatSym, " = ", muSym, ")"}]}, Spacings -> 0.2]], {-1.52, 0.3}, {-1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {{-Pi/2, Pi/2}, {0, 3.2}},
   FrameLabel -> {upright[Row[{q1Sym, " ", ellxSym}]], upright[Row[{eqpSym, " > 0"}]]},
   FrameTicks -> {{Automatic, None}, {{{-Pi/2, "-\[Pi]/2"}, {-Pi/4, "-\[Pi]/4"}, {0, "0"}, {Pi/4, "\[Pi]/4"}, {Pi/2, "\[Pi]/2"}}, None}},
   Sequence @@ asSize, PlotLabel -> asTitle["(a) Bogoliubov energies on the lattice"]];
fig4aLegend = Column[{LineLegend[{asLine[blue], asLine[orange], asDashed[grey]},
     Map[upright, {Row[{ePolarMinusSym, " (formula)"}], Row[{ePolarPlusSym, " (formula)"}], Row[{"normal state, |", epsLatSym, " \[Minus] ", muSym, "|"}]}], LegendLayout -> "Row"],
    asKeys[asPointKey["\[EmptyCircle]", muted, "lattice operator"]]}, Center];
(* Figure 4(b): the probability of the hole; the points as in Figure 3(b) *)
fig4bExact = Table[{x0, (asBDelta/as4E)^2 Sin[as4E x0]^2}, {x0, 0., 20., 0.05}];
fig4bIda = as4RabiRows[[1 ;; -1 ;; 4, {1, 2}]];
fig4bCvode = t2origBRabi[[3 ;; -1 ;; 4, {1, 2}]];
fig4b = Show[
   ListLinePlot[fig4bExact, PlotStyle -> asLine[grey]],
   ListPlot[{fig4bIda, fig4bCvode}, PlotStyle -> {blue, orange}, PlotMarkers -> {{"\[FilledCircle]", 9}, {"\[EmptySquare]", 9}}],
   Graphics[{muted, Dotted, Line[{{0, (asBDelta/as4E)^2}, {20, (asBDelta/as4E)^2}}],
     Text[asNote[Row[{"largest value (", deltaSym, "/", ePolarMinusSym, Superscript[")", 2]}]], {0.4, (asBDelta/as4E)^2 + 0.025}, {-1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {{0, 20}, {-0.02, 0.55}},
   FrameLabel -> {upright[Row[{x0Sym, " (time)"}]], upright[Row[{pHoleSym, " (probability of the hole)"}]]},
   Sequence @@ asSize, PlotLabel -> asTitle["(b) particle-hole oscillation of an injected particle"]];
fig4bLegend = Column[{LineLegend[{asLine[grey]}, {upright[Row[{"exact: (", deltaSym, "/", ePolarMinusSym, Superscript[")", 2], " ", Superscript["sin", 2], "(", ePolarMinusSym, " ", x0Sym, ")"}]]}],
    asKeys[asPointKey["\[FilledCircle]", blue, "IDA (this notebook)"], asPointKey["\[EmptySquare]", orange, "CVODE (original program)"]]}, Center];
asFigure["Figure 4, panels (a) and (b). Case B, the uniform polar superfluid (item [2], checks AS-3 and AS-4).", fig4a, fig4aLegend, fig4b, fig4bLegend]

(* ::Input:: *)
(* Figure 4(c): the gap of massless charges as a function of the coupling, for three chemical potentials.  Curves: the
   closed form solved by bisection; squares: the gaps that were inserted in the integral computed by quadrature *)
fig4cLines = Table[Map[{#[[2]], #[[3]]} &, Select[as5GapRows, #[[1]] == chem && #[[4]] == 0 &]], {chem, {0., 0.3, 0.6}}];
fig4cQuad = Table[Map[{#[[2]], #[[3]]} &, Select[as5GapRows, #[[1]] == chem && #[[4]] == 1 &]], {chem, {0., 0.3, 0.6}}];
fig4c = Show[
   ListLinePlot[fig4cLines, PlotStyle -> {asLine[blue], asLine[orange], asLine[aqua]}],
   ListPlot[fig4cQuad, PlotStyle -> {blue, orange, aqua}, PlotMarkers -> {"\[EmptySquare]", 10}],
   Graphics[{muted, Dotted, Line[{{1, 0}, {1, 1.2}}], Text[asNote[Row[{gCouplingSym, " = ", gCritSym}]], {1.03, 1.1}, {-1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {{0.2, 3.0}, {0, 1.2}},
   FrameLabel -> {upright[Row[{gCouplingSym, "/", gCritSym, " (", mSym, " = 0, ", lambdaSym, " = 1)"}]], upright[deltaSym]},
   Sequence @@ asSize, PlotLabel -> asTitle["(c) the gap equation in closed form"]];
fig4cLegend = Column[{LineLegend[{asLine[blue], asLine[orange], asLine[aqua]},
     Map[upright, {Row[{muSym, " = 0"}], Row[{muSym, " = 0.3"}], Row[{muSym, " = 0.6"}]}], LegendLayout -> "Row"],
    asKeys[asPointKey["\[EmptySquare]", muted, "gap checked by quadrature"]]}, Center];
asFigure["Figure 4, panel (c). Case B, the gap of the uniform polar superfluid of massless spinor charges (item [2], check AS-5).", fig4c, fig4cLegend]

(* ::Input:: *)
(* Figure 5(a): the pair field of the ring with a kink at x^1 = 6 and an antikink at x^1 = 18 *)
fig5a = Show[
   ListLinePlot[Table[{x, as9Profile[x]}, {x, 0., 24., 0.05}], PlotStyle -> asLine[blue]],
   Graphics[{muted, Line[{{0, 0}, {24, 0}}], Dotted, Line[{{as9Xk1, -1.15}, {as9Xk1, 1.15}}], Line[{{as9Xk2, -1.15}, {as9Xk2, 1.15}}],
     Text[asNote["kink"], {as9Xk1 + 0.4, -0.75}, {-1, 0}], Text[asNote["antikink"], {as9Xk2 + 0.4, 0.3}, {-1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {{0, 24}, {-1.15, 1.15}}, FrameLabel -> {upright[x1Sym], upright[deltaSym]},
   Sequence @@ asSize, PlotLabel -> asTitle["(a) the pair field on the ring"]];
(* Figure 5(b): the densities, both normalized to the integral 1.  Line: the analytic density of the bound states of the
   two walls at the 60 sites; circles: the summed density of the eight lattice eigenvectors near +m *)
fig5b = Show[
   ListLinePlot[as9ProfileRows[[All, {1, 4}]], PlotStyle -> asLine[blue]],
   ListPlot[as9ProfileRows[[All, {1, 3}]], PlotStyle -> orange, PlotMarkers -> {"\[EmptyCircle]", 8}],
   Sequence @@ asPanel, PlotRange -> {{0, 24}, {-0.01, 0.27}}, FrameLabel -> {upright[x1Sym], "density (integral 1)"},
   Sequence @@ asSize, PlotLabel -> asTitle["(b) the bound states sit on the walls"]];
fig5bLegend = Column[{LineLegend[{asLine[blue]}, {upright[Row[{"analytic, proportional to (", phiKSym, Superscript[")", 2], " of both walls"}]]}],
    asKeys[asPointKey["\[EmptyCircle]", orange, Row[{"lattice, the eight states near +", mSym}]]]}, Center];
asFigure["Figure 5, panels (a) and (b). Case C, a domain wall of the pair field on a ring of the length 24 (item [2], check AS-9).", fig5a, fig5b, fig5bLegend]

(* ::Input:: *)
(* Figure 5(c): the eigenvalues of the ring with a modulus below 2, against their number; the 16 bound states (the
   places as9Inside) are drawn larger and in orange *)
fig5cBound = as9SpectrumRows[[as9Inside]];
fig5cContinuum = Select[Delete[as9SpectrumRows, Map[List, as9Inside]], Abs[#[[2]]] < 2. &];
fig5cRange = {Min[fig5cContinuum[[All, 1]]] - 8, Max[fig5cContinuum[[All, 1]]] + 8};
fig5c = Show[
   ListPlot[{fig5cContinuum, fig5cBound}, PlotStyle -> {grey, orange}, PlotMarkers -> {{"\[FilledCircle]", 4}, {"\[FilledCircle]", 8}}],
   Graphics[{muted, Dashed, Line[{{fig5cRange[[1]], mass}, {fig5cRange[[2]], mass}}], Line[{{fig5cRange[[1]], -mass}, {fig5cRange[[2]], -mass}}],
     Text[asNote[Row[{"+", mSym}]], {fig5cRange[[1]] + 12, mass + 0.13}, {-1, 0}], Text[asNote[Row[{"\[Minus]", mSym}]], {fig5cRange[[1]] + 12, -mass - 0.13}, {-1, 0}]}],
   Sequence @@ asPanel, PlotRange -> {fig5cRange, {-2., 2.}}, FrameLabel -> {"number of the eigenvalue (ascending)", upright[eqpSym]},
   Sequence @@ asSize, PlotLabel -> asTitle[Row[{"(c) the spectrum of the ring near zero, ", ellxSym, " = 0.4"}]]];
fig5cLegend = asKeys[asPointKey["\[FilledCircle]", grey, "states that are not bound"], asPointKey["\[FilledCircle]", orange, "the 16 bound states"]];
(* Figure 5(d): the convergence of the analytic bound state with the lattice spacing (logarithmic axes): the shift of its
   Rayleigh quotient, the first-order shift r_W dx/6, and its relative residual *)
fig5dSpacing = as9ConvergenceRows[[All, 1]];
fig5d = Show[
   ListLogLogPlot[{Transpose[{fig5dSpacing, as9ConvergenceRows[[All, 2]] - mass}], Transpose[{fig5dSpacing, as9ConvergenceRows[[All, 3]]}]}, Joined -> True,
    PlotStyle -> {asLine[blue], asLine[orange]}, PlotMarkers -> {{"\[FilledCircle]", 9}, {"\[FilledSquare]", 9}}, PlotRange -> {{0.04, 0.5}, {0.006, 0.2}},
    Axes -> False, Frame -> True, GridLines -> Automatic, GridLinesStyle -> Directive[GrayLevel[0.9]],
    FrameLabel -> {upright[Row[{ellxSym, " (lattice spacing)"}]], "shift of the energy; residual"},
    LabelStyle -> Directive[FontSize -> 11], Sequence @@ asSize, PlotLabel -> asTitle["(d) convergence of the analytic bound state"]],
   ListLogLogPlot[Transpose[{fig5dSpacing, fig5dSpacing/6.}], Joined -> True, PlotStyle -> asDashed[grey]]];
fig5dLegend = LineLegend[{asLine[blue], asDashed[grey], asLine[orange]},
   Map[upright, {Row[{eRSym, " \[Minus] ", mSym}], Row[{rWSym, " ", ellxSym, "/6 (first order)"}], "relative residual"}], LegendLayout -> "Row"];
asFigure["Figure 5, panels (c) and (d). Case C, the spectrum of the ring and the convergence of the analytic bound state (item [2], check AS-9).", fig5c, fig5cLegend, fig5d, fig5dLegend]

(* ::Input:: *)
(* Figure 6: the Landau levels of the two isospin components.  Lines: the formula; circles: the eigenvalues of the
   diagonalization in the oscillator basis (every level from 1 on is a pair of equal eigenvalues) *)
fig6Series[qq_, source_, sg_] := Map[{#[[2]], #[[3]]} &, Select[as10LandauRows, #[[1]] == qq && #[[4]] == source && sg #[[3]] > 0 &]];
fig6 = Show[
   ListLinePlot[{fig6Series[0.5, 0, 1], fig6Series[0.5, 0, -1], fig6Series[-0.5, 0, 1], fig6Series[-0.5, 0, -1]},
    PlotStyle -> {asLine[blue], asLine[blue], asLine[orange], asLine[orange]}],
   ListPlot[{Join[fig6Series[0.5, 1, 1], fig6Series[0.5, 1, -1]], Join[fig6Series[-0.5, 1, 1], fig6Series[-0.5, 1, -1]]}, PlotStyle -> {blue, orange},
    PlotMarkers -> {"\[EmptyCircle]", 10}],
   Graphics[{muted, Line[{{-0.3, 0}, {10.3, 0}}]}],
   Sequence @@ asPanel, PlotRange -> {{-0.3, 10.3}, {-4.7, 4.7}}, FrameTicks -> {{Automatic, None}, {Range[0, 10], None}},
   FrameLabel -> {upright[Row[{nLSym, " (number of the Landau level)"}]], upright[eqpSym]},
   ImageSize -> 560, PlotLabel -> asTitle["Landau levels: each level twice, except the level 0 (once)"]];
fig6Legend = Column[{LineLegend[{asLine[blue], asLine[orange]},
     Map[upright, {Row[{qChargeSym, " = +1/2, ", bUpSym, " = 0.89, formula"}], Row[{qChargeSym, " = \[Minus]1/2, ", bDownSym, " = 0.19, formula"}]}], LegendLayout -> "Row"],
    asKeys[asPointKey["\[EmptyCircle]", blue, "isospin up, oscillator basis"], asPointKey["\[EmptyCircle]", orange, "isospin down, oscillator basis"]]}, Center];
asFigure["Figure 6. Case D, the Landau levels of the two isospin components (item [2], check AS-10).", fig6, fig6Legend]

(* ::Subsection:: *)
(*Summary of item [2]*)

(* ::Input:: *)
Grid[Prepend[Map[{#["name"], If[#["pass"], Style["PASS", Darker[Green]], Style["FAIL", Red]], #["statement"]} &,
    Select[SL31`Test`$Results, StringStartsQ[#["name"], "AS-"] || #["name"] === "T2-DATA-2" &]], Style[#, Bold] & /@ {"check", "result", "statement"}],
  Frame -> All, Alignment -> {Left, Top}]
