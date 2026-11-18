(* ::Chapter:: *)
(*Part II. The encyclopedia of the programs of Test 2*)

(* ::Text:: *)
(*Part II rewrites, in the Wolfram Language, every program of Test 2 of the super-loop revision, with every check of the original programs. Test 2 asks eleven questions (items [1] to [11]): the field equations of a self-interacting spinor field on the $3+1$ manifold of the super-loop (item [1]), their analysis and analytic solutions (item [2]), a solver of the field equations and a plotter of its solutions (items [3] to [6], written in Python and in Rust), and the construction and the physics of the superfluid boson singlet state, SFBS (items [7] to [11]). The original programs are a library and six programs in Rust (the library t2core and the programs t2_fields, t2_analytic, t2_solve, t2_plot and t2_sfbs) and five programs in Python. Every chapter of Part II names the original files that it rewrites, and every check keeps the name that it has in the original report (GS-1 to GS-16, AS-1 to AS-10, EX-1 to EX-8, SB-1 to SB-17, and the self-tests).*)

(* ::Text:: *)
(*Names. The functions of Part II begin with t2 (for Test 2) and keep the names of the original functions, written in the style of the Wolfram Language (for example the Rust function `model::gamma_ch` is `t2gammaCh`). Indices keep the values of the physics: the space-time index $p$ runs over 0, 1, 2, 3, the space index $j$ and the isospin index $\iota$ over 1, 2, 3. Wherever a result is shown, it is labelled with the registered symbol of Part I.*)

(* ::Text:: *)
(*Components of families. A component of a registered family, for example $\gamma^0_{\rm LL}$ of the family $\gamma^p_{\rm LL}$, is a symbol of its own, made from the box of the family with the index replaced by its value (function `SL31Instance` of Part I). It has to exist before a cell in which it is typed is evaluated: the front end sends the whole cell to the kernel, which reads all of it before it evaluates the first line, and a component box that is not yet symbolized is read as an ordinary expression (for example $\gamma^0_{\rm LL}$ as the power 0 of $\gamma_{\rm LL}$, which is 1). The builder of the notebook collects every component that is typed anywhere in the notebook, and the next cell makes those that Part I has not made already; the check T2-SYM verifies that every one of them is read as one atomic symbol.*)

(* ::Input:: *)
(* the components typed in the notebook, as {TeX of the family, index -> value, ...}; the list is written here by
   the builder, which collects it from the source of the notebook *)
t2componentSpecs = (* @@COMPONENTS@@ *);
(* their boxes, made exactly as the builder made the boxes in the input cells *)
t2componentBoxes = Map[Apply[SL31`Build`symbolBoxes, #] &, t2componentSpecs];
(* a component that is already a symbol (made in Part I) is not symbolized again *)
t2componentNew = Select[t2componentBoxes, ! MatchQ[SL31SymbolOf[#], HoldComplete[_Symbol]] &];
Scan[SL31Symbolize, t2componentNew];
(* like the symbols of Part I (section I.6), every component prints in TraditionalForm (the form of the labels of
   graphics, from which TeXForm is made) as its box, not as its internal name; a component without a usage
   message receives one that names its family and its index values *)
Scan[Function[spec, With[{b = Apply[SL31`Build`symbolBoxes, spec]},
     Replace[SL31SymbolOf[b], HoldComplete[s_Symbol] :> (MakeBoxes[s, TraditionalForm] = b;
        If[! StringQ[MessageName[s, "usage"]], MessageName[s, "usage"] = "the component " <>
           StringRiffle[Map[#[[1]] <> " = " <> #[[2]] &, Rest[spec]], ", "] <> " of the registered family " <> First[spec]])]]],
  t2componentSpecs];
t2componentTeX = Map[Replace[SL31SymbolOf[#], HoldComplete[s_] :> ToString[TeXForm[Unevaluated[s]]]] &, t2componentBoxes];
SL31Check["T2-SYM", "every component of a registered family that is typed in an input cell of the notebook (collected from the source by the builder) is read as one atomic symbol, and its TeX is written from its printed form, without the internal name",
  AllTrue[t2componentBoxes, MatchQ[SL31SymbolOf[#], HoldComplete[_Symbol]] &] &&
   AllTrue[t2componentTeX, ! StringContainsQ[#, "UnderBracket" | "\[UnderBracket]"] &],
  Row[{Length[t2componentBoxes], " components, ", Length[t2componentNew], " of them made here; their TeX: ", StringRiffle[t2componentTeX, ", "]}]]

(* ::Text:: *)
(*Numbers. The original programs compute in double precision with random numbers from a generator with a fixed seed. Part II uses the same generator with the same seeds, and draws the random numbers in the same order, so that every random matrix, field and momentum is the same number as in the original program. Exact quantities (the Dirac matrices, the generators) are computed exactly; results that depend on rounding (the largest deviation of a check) are reported next to the value of the original report.*)

(* ::Section:: *)
(*II.1 The core library t2core: random numbers, matrices, the super-loop operators, the lattice*)

(* ::Subsection:: *)
(*II.1.1 Random numbers (t2core/src/rng.rs)*)

(* ::Text:: *)
(*The generator SplitMix64 keeps one unsigned 64-bit integer as its state. Every call adds the constant 9E3779B97F4A7C15 (hexadecimal) to the state and mixes the result with shifts, exclusive-or operations (`BitXor`) and multiplications; all arithmetic is modulo $2^{64}$, as in Rust. A uniform number in $[-1,1)$ is made from the upper 53 bits. The conversion is exact in both languages, so the Wolfram Language reproduces every random number of the original programs bit for bit. The test compares the first three integers and the first four uniform numbers for the seed 2026111707 (the seed of the program t2_fields) with the values of the same algorithm computed independently in Python 3.*)

(* ::Input:: *)
(* the state of the generator; t2Seed[s] starts a new sequence *)
t2Seed[s_Integer] := (t2rngState = Mod[s, 2^64]);
(* one step of SplitMix64: returns an integer in [0, 2^64) *)
t2NextU64[] := Module[{z},
   t2rngState = Mod[t2rngState + 16^^9E3779B97F4A7C15, 2^64];
   z = t2rngState;
   z = Mod[BitXor[z, BitShiftRight[z, 30]] 16^^BF58476D1CE4E5B9, 2^64];
   z = Mod[BitXor[z, BitShiftRight[z, 27]] 16^^94D049BB133111EB, 2^64];
   BitXor[z, BitShiftRight[z, 31]]];
(* a uniform number in [-1, 1): the upper 53 bits, as an exact rational, converted to a double *)
t2Unit[] := N[2 BitShiftRight[t2NextU64[], 11]/2^53 - 1];
(* a random complex number re + i im with re and im uniform in [-1, 1) (re is drawn first) *)
t2UnitC[] := Module[{re = t2Unit[]}, re + I t2Unit[]];
(* a list of n random complex numbers *)
t2RndVec[n_Integer] := Table[t2UnitC[], {n}];
(* the test: the first values for the seed 2026111707, computed by the same algorithm in Python 3 *)
t2Seed[2026111707];
t2first = {t2NextU64[], t2NextU64[], t2NextU64[]};
t2Seed[2026111707];
t2firstUnits = Table[t2Unit[], {4}];
SL31Check["T2-RNG", "the generator SplitMix64 gives, for the seed 2026111707, the integers and the uniform numbers of the original algorithm",
  t2first === {16253586484067153034, 11771902715344777314, 8742176756923514478} &&
   t2firstUnits === {0.7622173776706638, 0.2763122498264816, -0.05217129678912458, 0.4038650534416697},
  t2firstUnits]

(* ::Subsection:: *)
(*II.1.2 Complex numbers and matrices (t2core/src/c64.rs and cmat.rs)*)

(* ::Text:: *)
(*The Rust library defines its own complex numbers (c64.rs) and dense complex matrices (cmat.rs). The Wolfram Language has both built in: a complex number is `a + I b`; a matrix is a list of rows; `.` (Dot) is the matrix product, `ConjugateTranspose` the Hermitian conjugate, `KroneckerProduct` the Kronecker product with the same index convention as the library (every entry of the product is the product of one entry of the first matrix and one entry of the second; its row is the pair of their rows and its column the pair of their columns, the row or column of the first matrix being the slower of the two), `Tr` the trace and `Norm[m, "Frobenius"]` the Frobenius norm. Three routines of cmat.rs are rewritten here because the programs and their checks use them: the inverse of a $2\times2$ matrix by its explicit formula, the matrix exponential by scaling and squaring of the Taylor series, and the Hermitian eigensolver.*)

(* ::Input:: *)
(* the determinant and the inverse of a 2 x 2 matrix, by the explicit formulas of cmat.rs *)
t2det2[m_] := m[[1, 1]] m[[2, 2]] - m[[1, 2]] m[[2, 1]];
t2inv2[m_] := With[{d = t2det2[m]}, {{m[[2, 2]], -m[[1, 2]]}, {-m[[2, 1]], m[[1, 1]]}}/d];
(* the principal square root of a complex number, as in c64.rs: |z|^(1/2) (cos(arg z/2) + i sin(arg z/2)) *)
t2csqrt[z_] := With[{r = Sqrt[Abs[z]], th = Arg[z]/2}, r Cos[th] + I r Sin[th]];
(* the matrix exponential of cmat.rs: halve the matrix until its Frobenius norm is at most 1/4, sum
   the Taylor series (at most 29 terms, stopping when a term is smaller than 1e-18), then square the
   result as often as the matrix was halved *)
t2expm[m_] := Module[{n = Length[m], nrm = Norm[m, "Frobenius"], s = 0, scale = 1., x, term, sum},
   While[nrm scale > 0.25, scale *= 0.5; s++];
   x = scale m;
   term = IdentityMatrix[n]; sum = IdentityMatrix[n];
   Do[term = term . x/k; sum = sum + term; If[Norm[term, "Frobenius"] < 1*^-18, Break[]], {k, 1, 29}];
   Do[sum = sum . sum, {s}];
   sum];

(* ::Text:: *)
(*The Hermitian eigensolver `eigh` of cmat.rs finds all eigenvalues and eigenvectors of a complex Hermitian matrix (called `a` in the code) in two stages. (1) Householder reduction: for every column `k` a reflection (the unit matrix minus twice the projector on a complex unit vector `vv`) is chosen that sets to zero all entries of the column below the subdiagonal; applied from both sides it keeps the eigenvalues and turns `a` step by step into a tridiagonal matrix, and the product of the reflections is accumulated in the matrix `q`. The complex subdiagonal entries are then made real and non-negative by a diagonal unitary matrix of phases. (2) The implicit QL algorithm with Wilkinson shifts: plane rotations, applied to the real symmetric tridiagonal matrix, drive its off-diagonal entries to zero; the diagonal then holds the eigenvalues, and the accumulated rotations, multiplied by `q`, give the eigenvectors. The function below follows the Rust code step by step (the indices are shifted by one, because lists of the Wolfram Language begin at position 1); the vector operations of the Wolfram Language replace the inner loops.*)

(* ::Input:: *)
t2eigh[m_] := Module[{n = Length[m], a, q, vv, p, w, x, xnorm, x0, phase, alpha, vn, kk, s, d, e, ph, sub, r, zt,
    iter, mm, dd, g, sr, cr, pp, i, f, b, underflow, idx, z0, z1},
   (* stage 1: the Householder reduction of the Hermitian part of m *)
   a = N[(m + ConjugateTranspose[m])/2];
   q = N[IdentityMatrix[n]] + 0. I;
   Do[
    x = a[[k + 1 ;; n, k]];
    xnorm = Sqrt[Total[Abs[x]^2]];
    If[xnorm < 1*^-300, Continue[]];
    x0 = a[[k + 1, k]];
    phase = If[Abs[x0] > 0, x0/Abs[x0], 1.];
    alpha = -phase xnorm;
    (* v = x - alpha e1, normalized; v vanishes in the positions 1..k *)
    vv = ConstantArray[0. + 0. I, n];
    vv[[k + 1 ;; n]] = x;
    vv[[k + 1]] = vv[[k + 1]] - alpha;
    vn = Norm[vv];
    If[vn < 1*^-300, Continue[]];
    vv[[k + 1 ;; n]] = vv[[k + 1 ;; n]]/vn;
    (* H a H = a - 2 v w^dag - 2 w v^dag with p = a v, K = v^dag p, w = p - K v *)
    p = ConstantArray[0. + 0. I, n];
    p[[k ;; n]] = a[[k ;; n, k + 1 ;; n]] . vv[[k + 1 ;; n]];
    kk = Conjugate[vv[[k + 1 ;; n]]] . p[[k + 1 ;; n]];
    w = p - vv kk;
    a[[k ;; n, k ;; n]] = a[[k ;; n, k ;; n]] -
      2 (Outer[Times, vv[[k ;; n]], Conjugate[w[[k ;; n]]]] + Outer[Times, w[[k ;; n]], Conjugate[vv[[k ;; n]]]]);
    (* q <- q H *)
    s = q[[All, k + 1 ;; n]] . vv[[k + 1 ;; n]];
    q[[All, k + 1 ;; n]] = q[[All, k + 1 ;; n]] - 2 Outer[Times, s, Conjugate[vv[[k + 1 ;; n]]]],
    {k, 1, n - 2}];
   (* the tridiagonal matrix: real diagonal d, complex subdiagonal; the phases ph make it real *)
   d = Re[Diagonal[a]];
   e = ConstantArray[0., n];
   ph = ConstantArray[1. + 0. I, n];
   Do[sub = a[[i, i - 1]]; r = Abs[sub];
    ph[[i]] = If[r > 0, sub ph[[i - 1]]/r, ph[[i - 1]]];
    e[[i]] = r, {i, 2, n}];
   q = q . DiagonalMatrix[ph];
   (* stage 2: the implicit QL algorithm (tqli); zt holds the transposed rotation matrix *)
   zt = N[IdentityMatrix[n]];
   e = Append[Rest[e], 0.];
   Do[
    iter = 0;
    While[True,
     mm = l;
     While[mm < n, dd = Abs[d[[mm]]] + Abs[d[[mm + 1]]]; If[Abs[e[[mm]]] <= $MachineEpsilon dd, Break[]]; mm++];
     If[mm == l, Break[]];
     iter++; If[iter > 60, Break[]];
     g = (d[[l + 1]] - d[[l]])/(2 e[[l]]);
     r = Sqrt[g^2 + 1.];
     g = d[[mm]] - d[[l]] + e[[l]]/(g + If[g >= 0, Abs[r], -Abs[r]]);
     sr = 1.; cr = 1.; pp = 0.;
     i = mm - 1; underflow = False;
     While[i >= l,
      f = sr e[[i]]; b = cr e[[i]];
      r = Sqrt[f^2 + g^2];
      e[[i + 1]] = r;
      If[r == 0., d[[i + 1]] -= pp; e[[mm]] = 0.; underflow = True; Break[]];
      sr = f/r; cr = g/r;
      g = d[[i + 1]] - pp;
      r = (d[[i]] - g) sr + 2 cr b;
      pp = sr r;
      d[[i + 1]] = g + pp;
      g = cr r - b;
      (* rotate the rows i and i + 1 of zt *)
      z0 = zt[[i]]; z1 = zt[[i + 1]];
      zt[[i + 1]] = z0 sr + z1 cr;
      zt[[i]] = z0 cr - z1 sr;
      i--];
     If[underflow, Continue[]];
     d[[l]] -= pp; e[[l]] = g; e[[mm]] = 0.],
    {l, 1, n}];
   (* sort the eigenvalues ascending; the eigenvectors are the columns of q . Transpose[zt] *)
   idx = Ordering[d];
   {d[[idx]], (q . Transpose[zt])[[All, idx]]}];

(* ::Text:: *)
(*A first test of the rewritten routines on a random $9\times9$ Hermitian matrix and a random $3\times3$ matrix, against the built-in functions `Eigenvalues` and `MatrixExp` (the full tests of the eigensolver are the check GS-11 below).*)

(* ::Input:: *)
t2Seed[12345];
t2testH = Module[{z = Table[t2UnitC[], {9}, {9}]}, z + ConjugateTranspose[z]];
t2testM = Table[t2UnitC[], {3}, {3}];
{t2testVals, t2testVecs} = t2eigh[t2testH];
SL31Check["T2-LINALG", "the rewritten eigensolver and matrix exponential of cmat.rs agree with Eigenvalues and MatrixExp of the Wolfram Language",
  Max[Abs[t2testVals - Sort[Re[Eigenvalues[t2testH]]]]] < 1*^-12 &&
   Norm[t2testVecs . DiagonalMatrix[t2testVals] . ConjugateTranspose[t2testVecs] - t2testH, "Frobenius"] < 1*^-12 &&
   Norm[t2expm[t2testM] - MatrixExp[t2testM], "Frobenius"] < 1*^-13,
  {Max[Abs[t2testVals - Sort[Re[Eigenvalues[t2testH]]]]], Norm[t2expm[t2testM] - MatrixExp[t2testM], "Frobenius"]}]

(* ::Subsection:: *)
(*II.1.3 The super-loop operators as matrices (t2core/src/model.rs)*)

(* ::Text:: *)
(*Every matrix of the field equations is built from the matrix $M_{\rm LL}(\mathbf b)$ of a bi-quaternion. A bi-quaternion $\mathbf b=\sum_{p=0}^3\xi^{\rm LL}_p(\mathbf b)\mathbf e_p$ (four complex coefficients $\xi^{\rm LL}_p(\mathbf b)$) has the $2\times2$ matrix $M_{\rm LL}(\mathbf b)=\xi^{\rm LL}_0(\mathbf b)\sigma^{\rm LL}_0-i\sum_{j=1}^3\xi^{\rm LL}_j(\mathbf b)\sigma^{\rm LL}_j$, so that $M_{\rm LL}(\mathbf e_0)=\sigma^{\rm LL}_0$ and $M_{\rm LL}(\mathbf e_j)=-i\sigma^{\rm LL}_j$. The canonical frame of $\mathbb R^{1,3}_{\rm LL}$ is $\mathbf f_0=\mathbf e_0$, $\mathbf f_j=i\mathbf e_j$, with $M_{\rm LL}(\mathbf f_p)=\sigma^{\rm LL}_p$. The Dirac matrices of a frame are $\gamma^p_{\rm LL}[\mathbf f]=\begin{pmatrix}0&M_{\rm LL}(\mathbf f_p)\\M_{\rm LL}(\bar{\mathbf f}_p)&0\end{pmatrix}$ (the chiral basis; $\bar{\mathbf b}$ is the conjugate bi-quaternion, in which the signs of the coefficients of $\mathbf e_1,\mathbf e_2,\mathbf e_3$ change); for the canonical frame they are $\gamma^p_{\rm LL}$. The spinor field $\Psi_{\rm LL}$ has the 8 components $\Psi_{{\rm LL},1},\dots,\Psi_{{\rm LL},8}$: components 1 and 2 are the first Dirac component with isospin up and down, 3 and 4 the second, 5 and 6 the third, 7 and 8 the fourth. A $4\times4$ matrix of the Dirac index therefore acts on the eight components as its Kronecker product with $\mathsf 1_2$, and a $2\times2$ matrix of the isospin index as the Kronecker product of $\mathsf 1_4$ with it. The cell defines every function of model.rs, in the same order; the comment of each function states the formula.*)

(* ::Input:: *)
(* the Pauli matrices sigma_0 (unit), sigma_1, sigma_2, sigma_3, and eps2 = i sigma_2 *)
t2sigma[0] = {{1, 0}, {0, 1}}; t2sigma[1] = {{0, 1}, {1, 0}};
t2sigma[2] = {{0, -I}, {I, 0}}; t2sigma[3] = {{1, 0}, {0, -1}};
t2eps2 = {{0, 1}, {-1, 0}};
(* M_LL(b) of the bi-quaternion b = sum_p xi_p e_p, xi = {xi_0, xi_1, xi_2, xi_3} *)
t2mq[xi_List] := xi[[1]] t2sigma[0] - I (xi[[2]] t2sigma[1] + xi[[3]] t2sigma[2] + xi[[4]] t2sigma[3]);
(* the conjugate bi-quaternion: the signs of the coefficients of e1, e2, e3 change *)
t2bqConj[xi_List] := {xi[[1]], -xi[[2]], -xi[[3]], -xi[[4]]};
(* the canonical frame of R^(1,3)_LL: f_0 = e0, f_j = i e_j (one row per p) *)
t2canonicalFrame = {{1, 0, 0, 0}, {0, I, 0, 0}, {0, 0, I, 0}, {0, 0, 0, I}};
(* the Dirac matrices of a frame f (a list of four bi-quaternions): gamma^p[f] = [[0, M(f_p)], [M(conj f_p), 0]] *)
t2gammasFromFrame[f_] := Table[ArrayFlatten[{{0, t2mq[f[[p + 1]]]}, {t2mq[t2bqConj[f[[p + 1]]]], 0}}], {p, 0, 3}];
(* gamma_LL^p of the canonical frame (p = 0..3) *)
t2gammaList = t2gammasFromFrame[t2canonicalFrame];
t2gamma[p_Integer] := t2gammaList[[p + 1]];
(* the chirality matrix gamma_ch = i gamma^0 gamma^1 gamma^2 gamma^3; C_LL = i gamma^2 gamma^0 *)
t2gammaCh = I t2gamma[0] . t2gamma[1] . t2gamma[2] . t2gamma[3];
t2cconj = I t2gamma[2] . t2gamma[0];
(* alpha^j = gamma^0 gamma^j; beta = gamma^0 *)
t2alpha[j_Integer] := t2gamma[0] . t2gamma[j];
t2beta = t2gamma[0];
(* the coefficients of the rotation generator J_LL^j = e_j/2 and of the boost generator K_LL^j = i e_j/2 *)
t2jgen[j_Integer] := ReplacePart[{0, 0, 0, 0}, j + 1 -> 1/2];
t2kgen[j_Integer] := ReplacePart[{0, 0, 0, 0}, j + 1 -> I/2];
(* a Lorentz generator u (imaginary bi-quaternion) on the Dirac spinor: sgen(u) = diag(-M(u)^dag, M(u)) *)
t2spinGen[u_List] := With[{m = t2mq[u]}, ArrayFlatten[{{-ConjugateTranspose[m], 0}, {0, m}}]];
(* the spinor representation of a unit bi-quaternion with the matrix mg = M_LL(gL): diag((mg^dag)^-1, mg) *)
t2spinRep[mg_] := ArrayFlatten[{{t2inv2[ConjugateTranspose[mg]], 0}, {0, mg}}];
(* the coordinates x^p = tr(sigma_p X)/2 of a Hermitian 2 x 2 matrix X *)
t2coords[x_] := Table[Re[Tr[t2sigma[p] . x]]/2, {p, 0, 3}];
(* the real 4 x 4 Lorentz matrix: entry (p, q) = Re tr(sigma_p mg sigma_q mg^dag)/2 *)
t2lmat[mg_] := Transpose[Table[t2coords[mg . t2sigma[q] . ConjugateTranspose[mg]], {q, 0, 3}]];
(* the vector action V_LL(u): the real 4 x 4 matrix of M(x) -> M(u) M(x) + M(x) M(u)^dag *)
t2vecAction[u_List] := With[{mu = t2mq[u]}, Transpose[Table[t2coords[mu . t2sigma[q] + t2sigma[q] . ConjugateTranspose[mu]], {q, 0, 3}]]];
(* a Dirac matrix lifted to the 8 components: A (x) 1_2; an isospin matrix: 1_4 (x) B *)
t2lift4[a_] := KroneckerProduct[a, IdentityMatrix[2]];
t2lift2[b_] := KroneckerProduct[IdentityMatrix[4], b];
(* the isospin generators T_LL^iota = 1_4 (x) tau_iota/2 *)
t2tiso[i_Integer] := t2lift2[t2sigma[i]/2];
(* the scalar pair matrices Kch_iota = (C gamma_ch) (x) (eps2 tau_iota) and the pseudoscalar ones C (x) (eps2 tau_iota) *)
t2kpair[i_Integer] := KroneckerProduct[t2cconj . t2gammaCh, t2eps2 . t2sigma[i]];
t2kpairPs[i_Integer] := KroneckerProduct[t2cconj, t2eps2 . t2sigma[i]];
(* the pair potential P_LL = sum_iota d^iota Kch_iota^dag (8 x 8) *)
t2pairPotential[d_List] := Sum[d[[i]] ConjugateTranspose[t2kpair[i]], {i, 3}];
(* a random unit bi-quaternion as its matrix in SL(2,C): four random complex entries, accepted if
   |det| > 0.1, then divided by the square root of the determinant *)
t2randSL2C[] := Module[{m, d},
   While[True,
    m = {{t2UnitC[], t2UnitC[]}, {t2UnitC[], t2UnitC[]}};
    d = t2det2[m];
    If[Abs[d] > 0.1, Return[m (1/t2csqrt[d]), Module]]]];
(* the SU(2) matrix exp(i sum_iota th_iota tau_iota/2) *)
t2su2[th_List] := t2expm[Sum[I th[[i]]/2 t2sigma[i], {i, 3}]];
(* the rotation R(U): U tau_iota U^dag = sum_kappa tau_kappa R[kappa][iota] *)
t2adjoint[u_] := Table[Re[Tr[t2sigma[k] . u . t2sigma[i] . ConjugateTranspose[u]]]/2, {k, 3}, {i, 3}];
(* the Levi-Civita symbol for indices 1, 2, 3 and the metric diag(1, -1, -1, -1) for p = 0..3 *)
t2eps3[i_, j_, k_] := Signature[{i, j, k}];
t2eta[p_] := If[p == 0, 1, -1];
(* the bi-quaternion b with M_LL(b) = m, and the product and the conjugations of bi-quaternions *)
t2bqFrom[m_] := {(m[[1, 1]] + m[[2, 2]])/2, I (m[[1, 2]] + m[[2, 1]])/2, (m[[2, 1]] - m[[1, 2]])/2, I (m[[1, 1]] - m[[2, 2]])/2};
t2bqMul[x_, y_] := t2bqFrom[t2mq[x] . t2mq[y]];
t2bqStar[x_] := Conjugate[x];
t2bqDag[x_] := Conjugate[t2bqConj[x]];
(* the Weyl isodoublet of a spinor charge y e4: psi'(y) = eps2^-1 M_LL(y)^T (rows: Weyl index, columns: isospin) *)
t2psiPrime[y_] := t2inv2[t2eps2] . Transpose[t2mq[y]];
(* the Dirac isodoublet of two spinor charges: components 1..4 are the rows of psi'(y), 5..8 those of psi'(b) *)
t2diracFromCharges[y_, b_] := Join[Flatten[t2psiPrime[y]], Flatten[t2psiPrime[b]]];
(* the Cayley-Dickson product (a + b e4)(c + d e4) = (a c - conj(d) b) + (d a + b conj(c)) e4 *)
t2bioctMul[{a_, b_}, {cq_, d_}] := {t2bqMul[a, cq] - t2bqMul[t2bqConj[d], b], t2bqMul[d, a] + t2bqMul[b, t2bqConj[cq]]};

(* ::Text:: *)
(*The basic matrices, labelled with their registered symbols (they are exact: integers and the imaginary unit):*)

(* ::Input:: *)
Grid[{
   {LL["\\sigma^{\\rm LL}_j", "j" -> "0"], MatrixForm[t2sigma[0]], LL["\\sigma^{\\rm LL}_j", "j" -> "1"], MatrixForm[t2sigma[1]],
    LL["\\sigma^{\\rm LL}_j", "j" -> "2"], MatrixForm[t2sigma[2]], LL["\\sigma^{\\rm LL}_j", "j" -> "3"], MatrixForm[t2sigma[3]]},
   {LL["\\gamma^p_{\\rm LL}", "p" -> "0"], MatrixForm[t2gamma[0]], LL["\\gamma^p_{\\rm LL}", "p" -> "1"], MatrixForm[t2gamma[1]],
    LL["\\gamma^p_{\\rm LL}", "p" -> "2"], MatrixForm[t2gamma[2]], LL["\\gamma^p_{\\rm LL}", "p" -> "3"], MatrixForm[t2gamma[3]]},
   {LL["\\gamma^{\\rm ch}_{\\rm LL}"], MatrixForm[t2gammaCh], LL["C_{\\rm LL}"], MatrixForm[t2cconj],
    LL["\\beta_{\\rm LL}"], MatrixForm[t2beta], LL["\\mathsf E_{\\rm LL}"], MatrixForm[t2eps2]}},
  Spacings -> {1, 2}, Alignment -> Center]

(* ::Subsection:: *)
(*II.1.4 The lattice operator (t2core/src/lattice.rs)*)

(* ::Text:: *)
(*The solvers discretize space on a lattice of $n^{\rm x}_{\rm LL}\times n^{\rm y}_{\rm LL}\times n^{\rm z}_{\rm LL}$ sites with the spacing $\ell^{\rm x}_{\rm LL}$ (called ell in the code). A site $\vec s_{\rm LL}=(s^1_{\rm LL},s^2_{\rm LL},s^3_{\rm LL})$ has integer coordinates counted from 0, and the code gives it the number $s^1_{\rm LL}+n^{\rm x}_{\rm LL}(s^2_{\rm LL}+n^{\rm y}_{\rm LL}s^3_{\rm LL})$; $\vec\delta^j_{\rm LL}$ is the step to the next site in the direction $j$. On every site a mode has 16 complex components: the particle part $\psi^{\rm p}_{\rm LL}$ (8 components) and the hole part $\psi^{\rm h}_{\rm LL}$ (8). The gauge fields live on the links: the link from $\vec s_{\rm LL}$ to $\vec s_{\rm LL}+\vec\delta^j_{\rm LL}$ carries the $2\times2$ isospin matrix $\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})=\exp\big(-i\ell^{\rm x}_{\rm LL}(g^{\rm v}_{\rm LL}\mathsf a_{{\rm LL},j}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\sigma^{\rm LL}_0)\big)$ with the isospin potential $\mathsf a_{{\rm LL},j}=\sum_\iota A^\iota_{{\rm LL},j}\tau^{\rm LL}_\iota/2$. Because $(\sum_\iota A^\iota_{{\rm LL},j}\tau^{\rm LL}_\iota)^2=\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert^2\sigma^{\rm LL}_0$, with the length $\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert$ of the isospin vector $\vec A^{\rm v}_{{\rm LL},j}=(A^1_{{\rm LL},j},A^2_{{\rm LL},j},A^3_{{\rm LL},j})$, the exponential has the closed form $e^{-i\ell^{\rm x}_{\rm LL}g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}}\big(\cos(\tfrac12\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert)\sigma^{\rm LL}_0-i\sin(\tfrac12\ell^{\rm x}_{\rm LL}g^{\rm v}_{\rm LL}\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert)\sum_\iota A^\iota_{{\rm LL},j}\tau^{\rm LL}_\iota/\lvert\vec A^{\rm v}_{{\rm LL},j}\rvert\big)$, which `t2link` evaluates. The particle row of the lattice Dirac-Bogoliubov-de Gennes operator $\mathcal H^{\rm lat}_{\rm LL}$, applied to a mode $(\psi^{\rm p}_{\rm LL},\psi^{\rm h}_{\rm LL})$, is*)

(* ::DisplayFormula:: *)
(*\sum_j\Big[-\frac{i\alpha^j_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\big(\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})-\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})\big)-\frac{r^{\rm W}_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\beta_{\rm LL}\big(\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})+\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})-2\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})\big)\Big]+m_{\rm LL}\beta_{\rm LL}\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})-\big(\mathsf A_{{\rm LL},0}(\vec s_{\rm LL})+\mu_{\rm LL}\big)\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})+\mathsf P_{\rm LL}(\vec s_{\rm LL})\psi^{\rm h}_{\rm LL}(\vec s_{\rm LL}),*)

(* ::Text:: *)
(*where $\alpha^j_{\rm LL}$ and $\beta_{\rm LL}$ act on the Dirac index and the link matrices on the isospin index (code: `t2hMinusV` without the pair term, and `t2apply`). The hole row is $\mathsf P_{\rm LL}(\vec s_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})$ minus the complex conjugate of the same expression without the pair term, applied to the complex conjugate of $\psi^{\rm h}_{\rm LL}$. Here $r^{\rm W}_{\rm LL}$ is the Wilson coefficient (the term in $r^{\rm W}_{\rm LL}$ removes the doubled fermions of the lattice), $\mathsf A_{{\rm LL},0}(\vec s_{\rm LL})=g^{\rm v}_{\rm LL}\sum_\iota A^\iota_{{\rm LL},0}T^\iota_{\rm LL}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\mathsf 1_8$ is the potential of the time direction at the site and $\mathsf P_{\rm LL}(\vec s_{\rm LL})=\sum_\iota d^\iota_{\rm LL}(\vec s_{\rm LL})(\mathsf K^\iota_{\rm LL})^\dagger$ the pairing block; a direction without a neighbour (a Dirichlet boundary) contributes zero. The function `t2apply` applies the operator to a vector, or, column by column, to a matrix whose columns are vectors; `t2dense` applies it to the unit matrix and so returns the whole matrix.*)

(* ::Input:: *)
(* the closed form of the link matrix exp(-i ell (gv a.tau/2 + gf b)) *)
t2link[gv_, gf_, a_List, b_, ell_] := Module[{na = Sqrt[a . a], x, ph, cx, sx, n1, n2, n3},
   x = 0.5 ell gv na;
   ph = Cos[-ell gf b] + I Sin[-ell gf b];
   {cx, sx} = {Cos[x], Sin[x]};
   {n1, n2, n3} = If[na > 0, a/na, {0., 0., 0.}];
   ph {{cx - I sx n3, -sx n2 - I sx n1}, {sx n2 - I sx n1, cx + I sx n3}}];
(* the 2 x 2 potential matrix gv A0.tau/2 + gf A0F *)
t2potential[gv_, gf_, a0_List, a0f_] := With[{h = 0.5 gv},
   {{gf a0f + h a0[[3]], h a0[[1]] - I h a0[[2]]}, {h a0[[1]] + I h a0[[2]], gf a0f - h a0[[3]]}}];
(* a lattice: <|"n" -> {n1, n2, n3}, "ell" -> spacing, "periodic" -> {True|False, ..}, "wilson" -> r_W,
   "mass" -> m, "mu" -> mu|>; sites are numbered from 0 *)
t2nsites[lat_] := Times @@ lat["n"];
t2dim[lat_] := 16 t2nsites[lat];
t2siteOf[lat_, {ix_, iy_, iz_}] := ix + lat["n"][[1]] (iy + lat["n"][[2]] iz);
t2siteCoords[lat_, s_] := {Mod[s, lat["n"][[1]]], Mod[Quotient[s, lat["n"][[1]]], lat["n"][[2]]], Quotient[s, lat["n"][[1]] lat["n"][[2]]]};
(* the neighbour of site s in the direction j (0, 1, 2), forward or backward; None at a Dirichlet boundary
   and in a direction with a single site *)
t2neighbour[lat_, s_, j_, fwd_] := Module[{x = t2siteCoords[lat, s], n = lat["n"][[j + 1]]},
   Which[
    n == 1, None,
    fwd && x[[j + 1]] + 1 == n, If[lat["periodic"][[j + 1]], x[[j + 1]] = 0; t2siteOf[lat, x], None],
    fwd, x[[j + 1]] += 1; t2siteOf[lat, x],
    x[[j + 1]] == 0, If[lat["periodic"][[j + 1]], x[[j + 1]] = n - 1; t2siteOf[lat, x], None],
    True, x[[j + 1]] -= 1; t2siteOf[lat, x]]];
(* trivial fields: unit links, zero potential, zero order parameter *)
t2trivialFields[ns_] := <|"links" -> Table[N[{IdentityMatrix[2], IdentityMatrix[2], IdentityMatrix[2]}], {ns}],
   "v0" -> Table[{{0., 0.}, {0., 0.}}, {ns}], "d" -> Table[{0., 0., 0.}, {ns}]|>;
(* the 8 x 8 matrices used on every site: alpha^j (x) 1_2, beta (x) 1_2 and Kch_iota^dag *)
t2alpha8 = Table[N[t2lift4[t2alpha[j]]], {j, 3}];
t2beta8 = N[t2lift4[t2beta]];
t2kdag8 = Table[N[ConjugateTranspose[t2kpair[i]]], {i, 3}];
t2iso2[m_, w_] := t2lift2[m] . w;
(* the lattice Dirac operator minus (V0 + mu) on the 8-component field f (a function of the site) at site s *)
t2hMinusV[lat_, fl_, f_, s_] := Module[{ps = f[s], out, inv2l = 0.5/lat["ell"], rw, wf, wb, sp, sm},
   rw = lat["wilson"] inv2l;
   out = lat["mass"] t2beta8 . ps;
   Do[
    If[lat["n"][[j + 1]] == 1, Continue[]];
    sp = t2neighbour[lat, s, j, True]; sm = t2neighbour[lat, s, j, False];
    wf = If[sp === None, 0 ps, t2iso2[fl["links"][[s + 1, j + 1]], f[sp]]];
    wb = If[sm === None, 0 ps, t2iso2[ConjugateTranspose[fl["links"][[sm + 1, j + 1]]], f[sm]]];
    out += -I t2alpha8[[j + 1]] . ((wf - wb) inv2l) - rw t2beta8 . (wf + wb - 2 ps),
    {j, 0, 2}];
   out - t2iso2[fl["v0"][[s + 1]], ps] - lat["mu"] ps];
(* H_DB applied to w: a vector of 16 nsites components, or a matrix whose columns are such vectors *)
t2apply[lat_, fl_, w_] := Module[{part, holeConj, pmat},
   part[s_] := w[[16 s + 1 ;; 16 s + 8]];
   holeConj[s_] := Conjugate[w[[16 s + 9 ;; 16 s + 16]]];
   Join @@ Table[
     pmat = Sum[fl["d"][[s + 1, i]] t2kdag8[[i]], {i, 3}];
     Join[t2hMinusV[lat, fl, part, s] + pmat . w[[16 s + 9 ;; 16 s + 16]],
      ConjugateTranspose[pmat] . part[s] - Conjugate[t2hMinusV[lat, fl, holeConj, s]]],
     {s, 0, t2nsites[lat] - 1}]];
(* the dense matrix of H_DB: the operator applied to the unit matrix *)
t2dense[lat_, fl_] := t2apply[lat, fl, N[IdentityMatrix[t2dim[lat]]] + 0. I];
(* the bound B^H of the spectral radius of H_lat (check GS-10); the original solvers use it in the bounds of their
   checks of the energy *)
t2normBound[lat_, fl_] := Count[lat["n"], n_ /; n > 1] (1 + 2 lat["wilson"])/lat["ell"] + Abs[lat["mass"]] + Abs[lat["mu"]] +
   Max[Map[Norm[#, "Frobenius"] &, fl["v0"]]] + Max[Map[Total[Abs[#]] &, fl["d"]]];
(* the free lattice dispersion eps_lat(q)^2 = sum_j sin^2(q_j ell)/ell^2 + (m + (r/ell) sum_j (1 - cos q_j ell))^2 *)
t2epsLat[q_List, ell_, m_, r_, active_List] := Sqrt[
   Sum[If[active[[j]], Sin[q[[j]] ell]^2/ell^2, 0], {j, 3}] + (m + r/ell Sum[If[active[[j]], 1 - Cos[q[[j]] ell], 0], {j, 3}])^2];
(* the 4 x 4 matrix of the free lattice operator on the plane wave exp(i q.x) *)
t2hLatPlane[q_List, ell_, m_, r_, active_List] := Sum[If[active[[j]], t2alpha[j] Sin[q[[j]] ell]/ell, 0], {j, 3}] +
   t2beta (m + r/ell Sum[If[active[[j]], 1 - Cos[q[[j]] ell], 0], {j, 3}]);
(* the anomalous density Pi^iota(s) = sum_n psi_p,n(s)^T Kch_iota conj(psi_h,n(s)) of a set of mode functions *)
t2anomalous[ns_, modes_List] := Table[Sum[w[[16 s + 1 ;; 16 s + 8]] . N[t2kpair[i]] . Conjugate[w[[16 s + 9 ;; 16 s + 16]]], {w, modes}],
   {s, 0, ns - 1}, {i, 3}];

(* ::Text:: *)
(*The library module sym.rs is a small symbolic algebra: linear combinations of monomials (TeX strings) with exact complex coefficients (the real and the imaginary part of a coefficient are rational numbers), used by the programs to print the matrices and the component equations in TeX and to evaluate every printed formula numerically. The Wolfram Language is itself a symbolic algebra with exact Gaussian rationals, so this module needs no rewriting: in check GS-12 below the same formulas are built from symbols and evaluated in the same way. For example, the TeX of $\sigma^{\rm LL}_2$ written by `TeXForm`:*)

(* ::Input:: *)
ToString[TeXForm[MatrixForm[t2sigma[2]]]]

(* ::Subsection:: *)
(*II.1.5 The time integration (t2core/src/cvode.rs): IDA of SUNDIALS through NDSolve*)

(* ::Text:: *)
(*The equations and the integrator. The lattice equations of a Nambu mode are $i\,\partial_0w^{\rm lat}_{\rm LL}=\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$: the mode is written as one complex vector $w^{\rm lat}_{\rm LL}$ (the 16 components of $\psi^{\rm p}_{\rm LL}$ and $\psi^{\rm h}_{\rm LL}$ on every site, the sites one after the other) with $n^{\rm cpx}_{\rm LL}$ complex components $w^{\rm cmp}_{\rm LL}$. The original programs integrate them in time with CVODE, the solver of ordinary differential equations of the suite SUNDIALS (SUite of Nonlinear and DIfferential/ALgebraic equation Solvers, Lawrence Livermore National Laboratory; version 7.8.0 rewritten in Rust and called through the module cvode.rs of t2core), with the Adams-Moulton formulas of the orders 1 to 12. The first version of the original programs, and of this notebook, used the classical Runge-Kutta method of the fourth order with a fixed step, written by hand; such a method is used nowhere in the project any more. A notebook cannot call the Rust program. The Wolfram Language contains another integrator of the same suite: IDA, the solver of differential-algebraic equations, which `NDSolve` uses with the option `Method -> "IDA"`. An ordinary differential equation is the special case without algebraic equations. Part II uses IDA for every time integration.*)

(* ::Text:: *)
(*How IDA integrates, and what its tolerances mean. IDA advances the solution in internal steps with the backward differentiation formulas of the orders 1 to 5 (such a formula expresses the derivative at the end of a step by the values at the ends of this step and of the steps before it) and solves the equations of every step by Newton's method. It chooses the length $h^{\rm IDA}_{\rm LL}$ and the order of every step itself, so that an estimate of the local error, the error made in this one step, passes a test: the weighted root-mean-square norm of the estimate must be at most 1. This norm is the square root of the mean, over all real unknowns, of the squares of the estimated errors, each divided by the tolerance of its unknown; the tolerance of a real unknown is $r^{\rm tol}_{\rm LL}$ times the modulus of the unknown plus $a^{\rm tol}_{\rm LL}$, with the relative tolerance $r^{\rm tol}_{\rm LL}$ and the absolute tolerance $a^{\rm tol}_{\rm LL}$. `NDSolve` takes the two tolerances from its options: the relative tolerance is 10 to the power minus the setting of `PrecisionGoal`, the absolute tolerance 10 to the power minus the setting of `AccuracyGoal`; `t2evolveIDA` therefore sets these options to the negative decimal logarithms of $r^{\rm tol}_{\rm LL}$ and $a^{\rm tol}_{\rm LL}$. After a successful step IDA doubles the length of the step if its estimate allows that; otherwise it keeps the length or shortens it. If the error of every real unknown equals its tolerance, the norm of the error of one step is the norm of the vector of the tolerances, which by the triangle inequality is at most $r^{\rm tol}_{\rm LL}$ times the norm of the state plus $a^{\rm tol}_{\rm LL}$ times the square root of the number $2n^{\rm cpx}_{\rm LL}$ of real unknowns. This number is called $B^{\rm loc}_{\rm LL}$. The exact local error of a step, called $\delta^{\rm loc}_{\rm LL}$, is the norm of the difference between the computed state at the end of the step and the exact solution that starts from the computed state at its beginning. IDA tests its estimate, not $\delta^{\rm loc}_{\rm LL}$ itself, so the checks T2-IDA and GS-13 measure $\delta^{\rm loc}_{\rm LL}$ for every internal step and compare it with $B^{\rm loc}_{\rm LL}$.*)

(* ::Text:: *)
(*The real form. IDA computes with real numbers (`NDSolve` answers a complex equation for this method with the message NDSolveValue::mconly and gives no solution). `t2evolveIDA` therefore writes the state as the real vector of the length $2n^{\rm cpx}_{\rm LL}$ made of the real parts `u` and the imaginary parts `v` of its components, and the Hermitian matrix as `hr` + $i$ `hi` with the real symmetric matrix `hr` and the real antisymmetric matrix `hi`. Then $-i\mathcal H^{\rm lat}_{\rm LL}w^{\rm lat}_{\rm LL}$ has the real part `hi.u + hr.v` and the imaginary part `-hr.u + hi.v`, so the lattice equations are the real linear system with the matrix `m = {{hi, hr}, {-hr, hi}}` acting on the real vector (`u`, `v`). The matrix `m` is antisymmetric, and the largest length that it gives to a vector of the length 1 is $E^{\rm max}_{\rm LL}$, the largest modulus of the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ of $\mathcal H^{\rm lat}_{\rm LL}$.*)

(* ::Text:: *)
(*The states between the internal steps. `NDSolve` returns the solution as an `InterpolatingFunction`. Its grid is the list of the times at the ends of the internal steps; with every grid time it stores the value of the solution and the value of the derivative that IDA computed there, and between two neighbouring grid times it gives every real unknown by the polynomial of the third degree that has these values and derivatives at the two ends (cubic Hermite interpolation). An output time lies in general inside an internal step, so a state returned by `t2evolveIDA` is an interpolated one, and its deviation from the exact solution has three sources. (1) The deviations of the stored values at the two ends of the step from the exact solution. The polynomial weighs the two values with numbers between 0 and 1 whose sum is 1, and the two derivatives with numbers whose moduli add up to at most one quarter of $h^{\rm IDA}_{\rm LL}$; the exact derivative is `m` applied to the exact value. If the larger deviation at the two ends is at most $B^{\rm err}_{\rm LL}$, this source gives at most $\big(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}\big)B^{\rm err}_{\rm LL}$. (2) The stored derivatives are not exactly `m` applied to the stored values, because Newton's method stops at a tolerance. The norm of the difference at a grid time is called $r^{\rm der}_{\rm LL}$; it gives at most $\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}$. (3) The interpolation of the exact solution itself. The cubic Hermite polynomial of a function with a continuous fourth derivative deviates from the function, at a time inside the step, by an average of the fourth derivative over the step, taken with a weight that is nowhere negative and whose integral is the product of the squares of the distances of that time from the two ends, divided by 24. The product is at most $(h^{\rm IDA}_{\rm LL})^4/16$, and the fourth derivative of the exact solution is the fourth power of `m` applied to it, a vector of at most $(E^{\rm max}_{\rm LL})^4$ times the norm of the state. This source therefore gives at most $(h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})^4/384$ times the norm of the state. The sum of the three is the bound $B^{\rm out}_{\rm LL}$ of the deviation of a returned state, written here for a state of the norm 1:*)

(* ::DisplayFormula:: *)
(*B^{\rm out}_{\rm LL}=\big(1+\tfrac14h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL}\big)B^{\rm err}_{\rm LL}+\tfrac14h^{\rm IDA}_{\rm LL}r^{\rm der}_{\rm LL}+\frac{(h^{\rm IDA}_{\rm LL}E^{\rm max}_{\rm LL})^4}{384}*)

(* ::Text:: *)
(*The deviation at the ends of the steps. The exact evolution is unitary: it changes the norm of no vector. The deviation at the end of a step is therefore the deviation at its beginning, carried along unchanged in size, plus the local error of the step, so that the deviation after $N^{\rm st}_{\rm LL}$ steps is at most the sum of their local errors $\delta^{\rm loc}_{\rm LL}$. If every local error is at most $B^{\rm loc}_{\rm LL}$, the deviation at the end of every step is at most $B^{\rm err}_{\rm LL}=N^{\rm st}_{\rm LL}B^{\rm loc}_{\rm LL}$, the accumulated local error that the tolerances allow. (The original programs use the same number with the tolerances of CVODE, which weighs the real and the imaginary part of a complex component with the modulus of the component: there $B^{\rm loc}_{\rm LL}$ is the square root of 2 times the sum of $r^{\rm tol}_{\rm LL}$ times the norm of the state and $a^{\rm tol}_{\rm LL}$ times the square root of $n^{\rm cpx}_{\rm LL}$.) The first statement needs no condition: the sum of the measured local errors is itself a bound of the deviation at the end of every step, whether or not the single errors stay below $B^{\rm loc}_{\rm LL}$ (IDA tests its own estimate of the error, in its weighted norm, not $\delta^{\rm loc}_{\rm LL}$). The checks of chapter II.3 use this sum. The function `t2localErrorsIDA` measures $\delta^{\rm loc}_{\rm LL}$ for every internal step, and their sum: with the eigenvalues $E^{\rm qp}_{{\rm LL},n}$ and the eigenvectors of the Hermitian matrix, the exact solution that starts from a computed state multiplies the coefficient of every eigenvector by the phase factor of the angle $-E^{\rm qp}_{{\rm LL},n}h^{\rm IDA}_{\rm LL}$, with the eigenvalue $E^{\rm qp}_{{\rm LL},n}$ of this eigenvector.*)

(* ::Input:: *)
(* The time integration of i dw/dx0 = hm . w by IDA of SUNDIALS through NDSolve.
   hm: a Hermitian matrix that does not depend on the time; w0: the state at x0 = 0; touts: the output times in
   increasing order, the first of them 0; rtol, atol: the tolerances of the local error test; analysis: optionally a
   function that is applied to the InterpolatingFunction of the real form before it is discarded.
   The result is an association with the keys
     "states"       the complex states at the output times
     "steps"        the number N^st of internal steps
     "evaluations"  the number N^ev of evaluations of the right side m . y that NDSolve reports (EvaluationMonitor)
     "hmax"         the largest internal step h^IDA
     "stepAt"       for every output time after the first: the length of the internal step that contains it
     "residualAt"   for the same output times: the larger norm r^der of (stored derivative) - m . (stored value) at
                    the two ends of that step
     "hermiteAt"    for the same output times: the norm of the difference between the state given by NDSolve and the
                    cubic Hermite polynomial built here from the values and the derivatives at the two ends of the step
     "analysis"     the result of the function analysis, or None.
   A failed integration (NDSolve then prints a message) gives $Failed. *)
t2evolveIDA[hm_?MatrixQ, w0_?VectorQ, touts_List, rtol_?NumericQ, atol_?NumericQ, analysis_ : None] :=
  Module[{nc = Length[w0], m, yIDA, tIDA, sol, dsol, grid, nev = 0, ends, toComplex, hermite},
   m = Developer`ToPackedArray[N[ArrayFlatten[{{Im[hm], Re[hm]}, {-Re[hm], Im[hm]}}]]];
   sol = NDSolveValue[{yIDA'[tIDA] == m . yIDA[tIDA], yIDA[0] == N[Join[Re[w0], Im[w0]]]}, yIDA, {tIDA, 0, Max[touts]},
     Method -> "IDA", PrecisionGoal -> -Log10[rtol], AccuracyGoal -> -Log10[atol], MaxSteps -> 10^7, EvaluationMonitor :> (nev++)];
   If[Head[sol] =!= InterpolatingFunction, Return[$Failed, Module]];
   grid = Flatten[sol["Grid"]];
   If[Last[grid] < Max[touts], Return[$Failed, Module]];
   dsol = Derivative[1][sol];
   toComplex[y_] := y[[1 ;; nc]] + I y[[nc + 1 ;; 2 nc]];
   (* the two ends of the internal step that contains an output time *)
   ends = Map[Function[tout, With[{k = Min[LengthWhile[grid, # < tout &] + 1, Length[grid]]}, grid[[{k - 1, k}]]]], Rest[touts]];
   (* the cubic Hermite polynomial of the step from a to b at the time tout, from the values and derivatives at a and b;
      frac is the fraction of the step that lies before tout *)
   hermite[tout_, {a_, b_}] := With[{frac = (tout - a)/(b - a)},
     (2 frac^3 - 3 frac^2 + 1) sol[a] + (3 frac^2 - 2 frac^3) sol[b] + (b - a) ((frac^3 - 2 frac^2 + frac) dsol[a] + (frac^3 - frac^2) dsol[b])];
   <|"states" -> Map[toComplex[sol[#]] &, touts],
     "steps" -> Length[grid] - 1, "evaluations" -> nev, "hmax" -> Max[Differences[grid]],
     "stepAt" -> Map[#[[2]] - #[[1]] &, ends],
     "residualAt" -> Map[Function[ab, Max[Norm[dsol[ab[[1]]] - m . sol[ab[[1]]]], Norm[dsol[ab[[2]]] - m . sol[ab[[2]]]]]], ends],
     "hermiteAt" -> MapThread[Norm[sol[#1] - hermite[#1, #2]] &, {Rest[touts], ends}],
     "analysis" -> If[analysis === None, None, analysis[sol]]|>];
(* The exact local errors delta^loc of all internal steps of a solution of t2evolveIDA, for the Hermitian matrix with the
   eigenvalues ev and the eigenvectors in the columns of vv (as t2eigh returns them).  The states at the grid times are
   expanded in the eigenvectors (the rows of coef); the exact solution over a step of the length h multiplies the
   coefficient of the eigenvalue E by exp(-i E h).  The steps are treated in groups of 2000 to save memory.
   The result: the largest delta^loc/B^loc with B^loc = rtol (norm of the state) + atol sqrt(2 n^cpx), the largest
   delta^loc, the largest norm of a state at a grid time, and the sum of delta^loc over all internal steps. *)
t2localErrorsIDA[ev_List, vv_?MatrixQ, rtol_?NumericQ, atol_?NumericQ][sol_] :=
  Module[{grid = Flatten[sol["Grid"]], vals = sol["ValuesOnGrid"], nc = Length[ev], npts, ratioMax = 0., emax = 0., nmax = 0., total = 0., idx, wgrid, coef, err, nrm},
   npts = Length[grid];
   Do[
    idx = Range[first, Min[first + 2000, npts]];
    wgrid = vals[[idx, 1 ;; nc]] + I vals[[idx, nc + 1 ;; 2 nc]];
    coef = wgrid . Conjugate[vv];
    err = Map[Norm, Rest[coef] - Most[coef] Exp[-I Outer[Times, Differences[grid[[idx]]], ev]]];
    nrm = Map[Norm, wgrid];
    ratioMax = Max[ratioMax, Max[err/(rtol Most[nrm] + atol Sqrt[2. nc])]];
    emax = Max[emax, Max[err]];
    total += Total[err];
    nmax = Max[nmax, Max[nrm]],
    {first, 1, npts - 1, 2000}];
   <|"largestRatio" -> ratioMax, "largestLocalError" -> emax, "largestNorm" -> nmax, "sumLocalErrors" -> total|>];
(* An integration by IDA together with the measurements that its checks need (used by the checks of the later
   chapters).  The eigenvalues and eigenvectors of the Hermitian matrix hm are computed with t2eigh, the equations are
   integrated with t2evolveIDA, and the exact local error of every internal step is measured with t2localErrorsIDA.
   The result is the association of t2evolveIDA with these further keys:
     "emax"      E^max, the largest modulus of an eigenvalue of hm
     "ratio"     the largest ratio of an exact local error delta^loc to rtol (norm of the state) + atol sqrt(2 n^cpx)
     "norm"      the largest norm of a state at the end of an internal step
     "bloc"      B^loc = rtol (largest norm of a state) + atol sqrt(2 n^cpx)
     "berr"      B^err = N^st B^loc, the bound of the deviation at the end of every internal step
     "residual"  the largest r^der at the ends of the internal steps that contain the output times
     "hermite"   the largest difference between a state given by NDSolve and the cubic Hermite polynomial of its step
     "bout"      B^out = (1 + h^IDA E^max/4) B^err + h^IDA r^der/4 + (h^IDA E^max)^4/384 (largest norm), the bound
                 of the norm of the deviation of a state at an output time (for a state of any norm), which holds if
                 every delta^loc is at most B^loc ("ratio" at most 1)
     "sumloc"    the sum of the measured exact local errors delta^loc of all internal steps: because the evolution is
                 unitary, the deviation at the end of every internal step is at most this sum, whatever the ratio is
     "bsum"      the same bound as "bout" with "sumloc" in place of B^err: it holds without any condition *)
t2evolveIDAChecked[hm_?MatrixQ, w0_?VectorQ, touts_List, rtol_?NumericQ, atol_?NumericQ] :=
  Module[{evC, vvC, runC, normC, hmaxC, emaxC, blocC, berrC, residC, sumC},
   {evC, vvC} = t2eigh[hm];
   emaxC = Max[Abs[evC]];
   runC = t2evolveIDA[hm, w0, touts, rtol, atol, t2localErrorsIDA[evC, vvC, rtol, atol]];
   If[runC === $Failed, Return[$Failed, Module]];
   normC = runC[["analysis", "largestNorm"]];
   hmaxC = runC["hmax"];
   residC = Max[runC["residualAt"]];
   blocC = rtol normC + atol Sqrt[2. Length[w0]];
   berrC = runC["steps"] blocC;
   sumC = runC[["analysis", "sumLocalErrors"]];
   Join[runC, <|"emax" -> emaxC, "ratio" -> runC[["analysis", "largestRatio"]], "norm" -> normC, "bloc" -> blocC, "berr" -> berrC,
     "residual" -> residC, "hermite" -> Max[runC["hermiteAt"]],
     "bout" -> (1 + hmaxC emaxC/4) berrC + hmaxC residC/4 + (hmaxC emaxC)^4/384 normC,
     "sumloc" -> sumC, "bsum" -> (1 + hmaxC emaxC/4) sumC + hmaxC residC/4 + (hmaxC emaxC)^4/384 normC|>]];

(* ::Text:: *)
(*Check T2-IDA has three parts. Part (a) shows what the two tolerances mean, on the equation whose solution is the decaying exponential function $\exp(-x^0_{\rm LL})$ (the derivative of the unknown is minus the unknown, the initial value is 1), integrated from 0 to 25 with $r^{\rm tol}_{\rm LL}=10^{-8}$ and $a^{\rm tol}_{\rm LL}=10^{-10}$. The solution falls from 1 to about $10^{-11}$. While it is much larger than $a^{\rm tol}_{\rm LL}/r^{\rm tol}_{\rm LL}=10^{-2}$, the tolerance of a step is $r^{\rm tol}_{\rm LL}$ times the modulus of the solution; when it is much smaller, the tolerance is $a^{\rm tol}_{\rm LL}$. The check requires that the exact local error of every internal step is at most the tolerance of the step, and that in each of the two regimes the largest ratio of the local error to the tolerance exceeds 1/128. The second requirement shows that the tolerances are the stated ones and not smaller ones: by its rule for the doubling of a step, IDA at the order 5 doubles the step as soon as its estimate falls below 1/128 of the tolerance. Part (b) tests `t2evolveIDA` and `t2localErrorsIDA` on a Hermitian $2\times2$ matrix, integrated from 0 to 10 with $r^{\rm tol}_{\rm LL}=10^{-10}$ and $a^{\rm tol}_{\rm LL}=10^{-12}$ and compared at 21 output times with the solution by the matrix exponential (`MatrixExp`): every $\delta^{\rm loc}_{\rm LL}$ is at most $B^{\rm loc}_{\rm LL}$, the largest deviation at the output times is at most $B^{\rm out}_{\rm LL}$, and the states of `NDSolve` are the cubic Hermite polynomials of the stored values and derivatives (tolerance $10^{-12}$). Part (c): the function `t2evolveIDAChecked` puts the integration and these measurements together for the checks of the later chapters. It computes the eigenvalues and the eigenvectors of the matrix itself, and returns the bounds $B^{\rm loc}_{\rm LL}$, $B^{\rm err}_{\rm LL}$ and $B^{\rm out}_{\rm LL}$ for a state of any norm, the sum of the measured local errors, and the bound of the deviation at the output times that is made with this sum in place of $B^{\rm err}_{\rm LL}$. Applied to the problem of part (b) it must give the same states and the same bounds; the sum of the local errors must be at most $B^{\rm err}_{\rm LL}$ (every one of them is at most $B^{\rm loc}_{\rm LL}$), and the largest deviation must be at most the bound made with the sum.*)

(* ::Input:: *)
(* T2-IDA (a): the local errors that IDA accepts on the decaying exponential, rtol = 1e-8, atol = 1e-10 *)
{rtolDecay, atolDecay} = {1.*^-8, 1.*^-10};
solDecay = Module[{yD, tD}, NDSolveValue[{yD'[tD] == -yD[tD], yD[0] == 1.}, yD, {tD, 0, 25}, Method -> "IDA",
     PrecisionGoal -> -Log10[rtolDecay], AccuracyGoal -> -Log10[atolDecay]]];
gridDecay = Flatten[solDecay["Grid"]];
valsDecay = Flatten[solDecay["ValuesOnGrid"]];
(* the exact local error of every step: the computed value at its end minus the exact solution from the computed value
   at its beginning; and its ratio to the tolerance rtol |value| + atol of the step *)
localDecay = Abs[Rest[valsDecay] - Most[valsDecay] Exp[-Differences[gridDecay]]];
ratioDecay = localDecay/(rtolDecay Abs[Most[valsDecay]] + atolDecay);
(* the two regimes: the solution above 100 atol/rtol = 1 never holds after the start (the solution is at most 1), so the
   relative regime is taken as the solution above 10 atol/rtol = 0.1, the absolute regime as the solution below
   atol/(10 rtol) = 0.001 *)
relDecay = Max[Pick[ratioDecay, Map[# > 10 atolDecay/rtolDecay &, Abs[Most[valsDecay]]]]];
absDecay = Max[Pick[ratioDecay, Map[# < atolDecay/(10 rtolDecay) &, Abs[Most[valsDecay]]]]];
(* T2-IDA (b): t2evolveIDA on a Hermitian 2 x 2 matrix against the matrix exponential *)
hmTwo = {{0.3, 0.5 - 0.2 I}, {0.5 + 0.2 I, -0.7}};
w0Two = {0.6, 0.8 I};
toutsTwo = Range[0., 10., 0.5];
{evTwo, vvTwo} = t2eigh[hmTwo];
{rtolTwo, atolTwo} = {1.*^-10, 1.*^-12};
runTwo = t2evolveIDA[hmTwo, w0Two, toutsTwo, rtolTwo, atolTwo, t2localErrorsIDA[evTwo, vvTwo, rtolTwo, atolTwo]];
devTwo = Max[MapThread[Norm[#1 - MatrixExp[-I hmTwo #2] . w0Two] &, {runTwo["states"], toutsTwo}]];
blocTwo = rtolTwo runTwo[["analysis", "largestNorm"]] + atolTwo Sqrt[2. 2];
berrTwo = runTwo["steps"] blocTwo;
boutTwo = (1 + runTwo["hmax"] Max[Abs[evTwo]]/4) berrTwo + runTwo["hmax"] Max[runTwo["residualAt"]]/4 +
   (runTwo["hmax"] Max[Abs[evTwo]])^4/384 runTwo[["analysis", "largestNorm"]];
(* T2-IDA (c): the function t2evolveIDAChecked repeats the integration of (b) and must give the same states and bounds *)
checkedTwo = t2evolveIDAChecked[hmTwo, w0Two, toutsTwo, rtolTwo, atolTwo];
sameTwo = checkedTwo["states"] == runTwo["states"] && checkedTwo["steps"] == runTwo["steps"] && checkedTwo["bloc"] == blocTwo &&
   checkedTwo["berr"] == berrTwo && checkedTwo["bout"] == boutTwo && checkedTwo["ratio"] == runTwo[["analysis", "largestRatio"]];
sumTwo = checkedTwo["sumloc"] <= berrTwo && devTwo <= checkedTwo["bsum"] <= boutTwo;
SL31Check["T2-IDA", "the integrator IDA of SUNDIALS, used through NDSolve with Method -> \"IDA\": (a) on the decaying exponential (rtol = 1e-8, atol = 1e-10, x_LL^0 from 0 to 25) the exact local error of every internal step is at most the tolerance rtol |value| + atol of the step, and the largest ratio of the local error to this tolerance exceeds 1/128 both while the solution is above 10 atol/rtol and after it has fallen below atol/(10 rtol): the tolerances of IDA are 10^-PrecisionGoal and 10^-AccuracyGoal; (b) on a Hermitian 2 x 2 matrix (rtol = 1e-10, atol = 1e-12, 21 output times up to 10) every local error is at most budget_step = rtol (norm of the state) + atol sqrt(4), the largest deviation from the solution by MatrixExp is at most budget_out, and the states of NDSolve are the cubic Hermite polynomials of the stored values and derivatives (tolerance 1e-12); (c) the function t2evolveIDAChecked, which makes these measurements for the checks of the later chapters, gives the same states and the same bounds as (b); the sum of its measured local errors is at most budget, and the largest deviation is at most the bound made with this sum in place of budget, which is at most budget_out",
  Max[ratioDecay] <= 1 && relDecay > 1/128 && absDecay > 1/128 && runTwo[["analysis", "largestRatio"]] <= 1 && devTwo <= boutTwo && Max[runTwo["hermiteAt"]] < 1*^-12 && sameTwo && sumTwo,
  Row[{"(a) ", Length[localDecay], " steps, largest ratio of a local error to its tolerance ", NumberForm[Max[ratioDecay], {4, 3}], " (relative regime ", NumberForm[relDecay, {4, 3}],
    ", absolute regime ", NumberForm[absDecay, {4, 3}], "); (b) ", runTwo["steps"], " steps, ", runTwo["evaluations"], " evaluations, largest local_error/budget_step ",
    NumberForm[runTwo[["analysis", "largestRatio"]], {4, 3}], ", largest deviation ", ScientificForm[devTwo, 2], " <= budget_out ", ScientificForm[boutTwo, 2],
    ", Hermite polynomials within ", ScientificForm[Max[runTwo["hermiteAt"]], 2], "; (c) the same states and bounds: ", sameTwo, "; sum of the local errors ", ScientificForm[checkedTwo["sumloc"], 2], " <= budget ", ScientificForm[berrTwo, 2],
    ", largest deviation ", ScientificForm[devTwo, 2], " <= bound with the sum ", ScientificForm[checkedTwo["bsum"], 2]}]]
