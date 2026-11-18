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

(* ::Text:: *)
(*$\sum_j\Big[-\frac{i\alpha^j_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\big(\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})-\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})\big)-\frac{r^{\rm W}_{\rm LL}}{2\ell^{\rm x}_{\rm LL}}\beta_{\rm LL}\big(\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL})\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}+\vec\delta^j_{\rm LL})+\mathsf U^{\rm link}_{{\rm LL},j}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})^\dagger\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL}-\vec\delta^j_{\rm LL})-2\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})\big)\Big]+m_{\rm LL}\beta_{\rm LL}\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})-\big(\mathsf A_{{\rm LL},0}(\vec s_{\rm LL})+\mu_{\rm LL}\big)\psi^{\rm p}_{\rm LL}(\vec s_{\rm LL})+\mathsf P_{\rm LL}(\vec s_{\rm LL})\psi^{\rm h}_{\rm LL}(\vec s_{\rm LL})$,*)

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
(* the bound of the spectral radius used for the time step *)
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
(* one step of the classical Runge-Kutta method RK4 for i dw/dt = H w *)
t2rk4Step[lat_, fl_, w_, dt_] := Module[{k1, k2, k3, k4},
   k1 = -I t2apply[lat, fl, w];
   k2 = -I t2apply[lat, fl, w + 0.5 dt k1];
   k3 = -I t2apply[lat, fl, w + 0.5 dt k2];
   k4 = -I t2apply[lat, fl, w + dt k3];
   w + dt/6 (k1 + 2 k2 + 2 k3 + k4)];

(* ::Text:: *)
(*The library module sym.rs is a small symbolic algebra: linear combinations of monomials (TeX strings) with exact coefficients $a+bi$ ($a,b$ rational), used by the programs to print the matrices and the component equations in TeX and to evaluate every printed formula numerically. The Wolfram Language is itself a symbolic algebra with exact Gaussian rationals, so this module needs no rewriting: in check GS-12 below the same formulas are built from symbols and evaluated in the same way. For example, the TeX of $\sigma^{\rm LL}_2$ written by `TeXForm`:*)

(* ::Input:: *)
ToString[TeXForm[MatrixForm[t2sigma[2]]]]
