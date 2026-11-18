<!-- BEGIN nchecks_algebra -->
27
<!-- END nchecks_algebra -->

<!-- BEGIN nchecks_malcev -->
9
<!-- END nchecks_malcev -->

<!-- BEGIN nchecks_manifolds -->
9
<!-- END nchecks_manifolds -->

<!-- BEGIN nchecks_superfluid -->
11
<!-- END nchecks_superfluid -->

<!-- BEGIN nchecks_renorm -->
6
<!-- END nchecks_renorm -->

<!-- BEGIN nchecks_groebner -->
12
<!-- END nchecks_groebner -->

<!-- BEGIN nchecks_symbol -->
9
<!-- END nchecks_symbol -->

<!-- BEGIN nchecks_self_contained -->
1
<!-- END nchecks_self_contained -->

<!-- BEGIN checks_table -->
| report (data/) | written by | checks passed |
|---|---|---|
| `sl_algebra_checks.txt` | `superloop algebra` | 27 of 27 |
| `sl_malcev_checks.txt` | `superloop malcev` | 9 of 9 |
| `sl_manifolds_checks.txt` | `superloop manifolds` | 9 of 9 |
| `sl_superfluid_checks.txt` | `superloop superfluid` | 11 of 11 |
| `sl_renorm_checks.txt` | `superloop renorm` | 6 of 6 |
| `sl_groebner_checks.txt` | `malcev_groebner.py` | 12 of 12 |
| `sl_symbol_checks.txt` | `check_symbols.py` | 9 of 9 |
| `sl_self_contained_checks.txt` | `check_self_contained.py` | 1 of 1 |
<!-- END checks_table -->

<!-- BEGIN nchecks_explain -->
10
<!-- END nchecks_explain -->

<!-- BEGIN nchecks_r13 -->
8
<!-- END nchecks_r13 -->

<!-- BEGIN checks_table_all -->
| report (data/) | written by | checks passed |
|---|---|---|
| `sl_algebra_checks.txt` | `superloop algebra` | 27 of 27 |
| `sl_malcev_checks.txt` | `superloop malcev` | 9 of 9 |
| `sl_manifolds_checks.txt` | `superloop manifolds` | 9 of 9 |
| `sl_superfluid_checks.txt` | `superloop superfluid` | 11 of 11 |
| `sl_renorm_checks.txt` | `superloop renorm` | 6 of 6 |
| `sl_groebner_checks.txt` | `malcev_groebner.py` | 12 of 12 |
| `sl_symbol_checks.txt` | `check_symbols.py` | 9 of 9 |
| `sl_self_contained_checks.txt` | `check_self_contained.py` | 1 of 1 |
| `explanations/data/ex_superfluid_checks.txt` | `explain_superfluid` | 10 of 10 |
| `explanations/data/ex_r13_checks.txt` | `r13_yukawa` | 8 of 8 |
| `test2/data/t2_analytic_checks.txt` | `t2_analytic` | 10 of 10 |
| `test2/data/t2_fields_checks.txt` | `t2_fields` | 16 of 16 |
| `test2/data/t2_py_examples_checks.txt` | `t2_examples_check.py` | 10 of 10 |
| `test2/data/t2_py_selftest_checks.txt` | `t2_solve.py` | 4 of 4 |
| `test2/data/t2_rs_examples_checks.txt` | `t2_examples_check.py` | 11 of 11 |
| `test2/data/t2_rs_selftest_checks.txt` | `t2_solve` | 5 of 5 |
| `test2/data/t2_sfbs_checks.txt` | `t2_sfbs` | 14 of 14 |
<!-- END checks_table_all -->

<!-- BEGIN checks_rows_tex -->
A-1 & the associator [x,y,z] = (xy)z - x(yz) of the bi-octonions B\_LL is alternating (it changes sign under every exchange of two of its arguments) on all 512 ordered triples of the units e0..e7: B\_LL is an ALTERNATIVE algebra & PASS\\
A-2 & B\_LL is NOT associative: exactly 168 of the 512 ordered triples of units have a non-zero associator (the triples of distinct imaginary units outside the 7 quaternionic triples); example [e1,e2,e4] = (e1 e2) e4 - e1 (e2 e4) = 2 e7  -- non-zero associators: 168; coefficients of [e1,e2,e4] on e0..e7: [0, 0, 0, 0, 0, 0, 0, 2] & PASS\\
A-3 & on 300 random bi-octonions x, y, z with Gaussian-integer coefficients the complex norm is multiplicative, N\_LL(xy) = N\_LL(x) N\_LL(y), and the three Moufang identities z(x(zy)) = ((zx)z)y, ((yz)x)z = y(z(xz)), (zx)(yz) = (z(xy))z hold exactly: the unit bi-octonions form a Moufang LOOP, the super-loop L\_LL & PASS\\
A-4 & for the commutator [x,y] = xy - yx of B\_LL the Jacobiator Jac(x,y,z) = [[x,y],z] + [[y,z],x] + [[z,x],y] equals six times the associator on all 512 triples of units, Jac(x,y,z) = 6 [x,y,z]; it is non-zero on 168 triples: the Jacobi identity fails exactly where associativity fails  -- non-zero Jacobiators: 168 & PASS\\
A-5 & the imaginary bi-octonions m\_LL = C (x) Im O with the commutator satisfy the linearized Malcev identity Jac(x,y,[w,z]) + Jac(w,y,[x,z]) = [Jac(x,y,z),w] + [Jac(w,y,z),x] on all 2401 4-tuples of imaginary units: m\_LL is a Malcev algebra that is not a Lie algebra & PASS\\
A-6 & for 50 random triples of 4x4 integer matrices (operators, whose product is associative) the Jacobiator of the commutator vanishes exactly; this is the step that every no-go proof takes when it treats the symmetry generators as operators & PASS\\
A-7 & left multiplication x -$>$ L\_x is not a representation of the Malcev algebra m\_LL: [L\_e1, L\_e4] - L\_[e1,e4] is a non-zero matrix & PASS\\
A-8 & Theorem R: the Jacobiators of the real basis elements e\_p, i e\_p (p = 1..7) of m\_LL span all of m\_LL (real rank 14, exact); because operators obey the Jacobi identity (A-6), every linear map from m\_LL to operators on a vector space that turns commutators into commutators maps every Jacobiator to 0 and therefore vanishes: m\_LL has no non-zero realization by operators  -- rank of the span of the Jacobiators: 14 & PASS\\
L-1 & the Lorentz part l\_LL = C (x) Im H (real basis J\_LL\^{}j = e\_j/2, K\_LL\^{}j = i e\_j/2, j = 1,2,3) is a Lie subalgebra of m\_LL isomorphic to so(3,1): [J\_LL\^{}j, J\_LL\^{}j'] = sum\_j'' eps3\_(j j' j'') J\_LL\^{}j'', [J\_LL\^{}j, K\_LL\^{}j'] = sum\_j'' eps3\_(j j' j'') K\_LL\^{}j'', [K\_LL\^{}j, K\_LL\^{}j'] = -sum\_j'' eps3\_(j j' j'') J\_LL\^{}j''; the unit bi-quaternions SL(2,C) = Spin(3,1)\^{}+ form a subgroup of the super-loop L\_LL & PASS\\
L-2 & the Killing form Killing(x,y) = tr(ad x ad y) of l\_LL (6x6, exact) is non-degenerate with signature (3 positive, 3 negative), negative on the rotations and positive on the boosts, as for so(3,1)  -- signature (3, 3, 0), Killing(e1,e1) = -16, Killing(i e1,i e1) = 16 & PASS\\
L-3 & a Lorentz element uL of l\_LL acts on a spinor charge bS e4 of s\_LL = (C (x) H) e4 by [uL, bS e4] = 2 (bS uL) e4 = -2 (W\_LL(uL) bS) e4, where W\_LL(uL) bS = -bS uL is a Lie representation (the Weyl spinor); the inner action rho\_LL(uL) = -2 W\_LL(uL) obeys [rho\_LL(uL1), rho\_LL(uL2)] = -2 rho\_LL([uL1,uL2]): it is NOT a representation (the twist) & PASS\\
L-4 & the bracket of two spinor charges lies in the Lorentz part, [bS e4, bS' e4] = bSbar bS' - bS'bar bS in l\_LL, and these brackets span all of l\_LL (rank 6): spinor charges close into Lorentz generators  -- rank 6 & PASS\\
L-5 & Jacobiators by sector: Jac(uL1, uL2, bS e4) = -6 (W\_LL([uL1,uL2]) bS) e4 = (Theta\_LL - Theta\_LL\^{}2) (W\_LL([uL1,uL2]) bS) e4 with the twist charge Theta\_LL = -2 on s\_LL (exact on all 288 triples of basis elements); Jac(s\_LL, s\_LL, s\_LL) (e.g. Jac(e4,e5,e6) = -12 e7) and Jac(l\_LL, s\_LL, s\_LL) (e.g. Jac(e1,e4,e6)) are non-zero as well & PASS\\
L-6 & with the Lie action (twist charge 1 instead of -2) and the same non-zero bracket [s\_LL, s\_LL] in l\_LL, the Jacobi identity fails on triples of spinor charges: no Lie algebra extends so(3,1) by bosonic spinor charges that close into so(3,1) (Theorem L; the classification G-1, G-5, G-10, M-3 shows that every Lie algebra of the covariant families has no Lorentz part in [s\_LL, s\_LL]) & PASS\\
L-7 & the Lorentz action by automorphisms, D\_LL(uL)(aH + bS e4) = [uL, aH] + (-bS uL) e4, and the isospin action Dv\_LL(tI)(aH + bS e4) = (tI bS) e4 (tI in Im H) are derivations of B\_LL (100 random pairs), they commute, the isospin derivations vanish on C (x) H, and the inner action factors as [uL, .] = D\_LL(uL) o Theta\_LL with the twist charge Theta\_LL = 1 on l\_LL and -2 on s\_LL & PASS\\
L-8 & the real derivations of B\_LL form a Lie algebra of real dimension 28 (the complex Lie algebra g2(C)): the linear system has rank 228 for 256 unknowns (at most 28 solutions), and the inner derivations [L\_x,L\_y] + [L\_x,R\_y] + [R\_x,R\_y] are verified derivations spanning 28 dimensions  -- rank of the system 228, explicit derivations span 28 & PASS\\
L-9 & varphi\_(gL,vI)(aH + bS e4) = gL aH gLbar + (vI bS gLbar) e4 is an automorphism of B\_LL for the unit bi-quaternions gL = 1 + i e1 + e2 (a Lorentz transformation) and vI = e3 (an isospin rotation), on 100 random pairs: (SL(2,C) x SL(2,C))/Z2 = SO(4,C) inside G2(C) & PASS\\
Q-1 & the electric charge Q\_LL = T\_LL\^{}3 = i Dv\_LL(e3/2) (the third isospin generator, an automorphism of B\_LL) is a Hermitian operator on B\_LL with the eigenvalues 0 (four times) on C (x) H (the Lorentz part and the scalar) and +1/2, -1/2 (twice each) on the spinor charges (double precision)  -- on C (x) H: +0.000000 +0.000000 +0.000000 +0.000000; on s\_LL: -0.500000 -0.500000 +0.500000 +0.500000 & PASS\\
Q-2 & the isospin Casimir T\_LL\^{}2 = sum\_iota (T\_LL\^{}iota)\^{}2 = -sum\_iota Dv\_LL(e\_iota/2)\^{}2 equals 0 on C (x) H and 3/4 = (1/2)(1/2 + 1) on the spinor charges: the spinor charges form an isospin doublet (double precision)  -- on s\_LL: +0.750000 +0.750000 +0.750000 +0.750000 & PASS\\
Q-3 & the Lorentz Casimir Cas\_LL = -sum\_j D\_LL(J\_LL\^{}j)\^{}2 (exact; 4 Cas\_LL is an integer matrix) has the value spin(spin+1) = 0 on the scalar e0, 2 (spin 1) on the Lorentz part C (x) Im H and 3/4 (spin 1/2) on the spinor charges & PASS\\
Q-4 & chirality: on every bi-octonion the boost generators act as i times the rotation generators, D\_LL(K\_LL\^{}j) = i D\_LL(J\_LL\^{}j); with respect to the complex structure of B\_LL every sector has one chirality, type (spin, 0): l\_LL = (1,0), s\_LL = (1/2,0); the complex-conjugate super-loop has the other chirality (0, spin) & PASS\\
Q-5 & spinor number: the rotation generator i D\_LL(J\_LL\^{}3) has the integer eigenvalues \{-1, 0, 0, 1\} on C (x) H and the half-odd eigenvalues \{-1/2, -1/2, 1/2, 1/2\} on the spinor charges, so the rotation by 2 pi, exp(2 pi D\_LL(J\_LL\^{}3)), is +1 on the Lorentz part and -1 on the spinor charges (double precision)  -- on C (x) H: -1.000000 +0.000000 +0.000000 +1.000000; on s\_LL: -0.500000 -0.500000 +0.500000 +0.500000 & PASS\\
Q-6 & inside the loop, conjugation by an element gL of SL(2,C) acts on a spinor charge as gL (bS e4) gLbar = (bS gL\^{}2) e4 (both bracketings agree), whereas the Lorentz automorphism acts as bS -$>$ bS gL\^{}-1: the loop's inner action turns a spinor twice as fast and backwards (twist charge -2) & PASS\\
C-1 & the unit e4 of the super-loop (e4\^{}2 = -1, e4\^{}-1 = -e4) acts on bi-quaternions by e4 aH e4\^{}-1 = aHbar (100 random aH): on SL(2,C) it is the inversion gL -$>$ gL\^{}-1, and on Minkowski space R\^{}(1,3)\_LL (Hermitian bi-quaternions) it is the parity (x\_LL\^{}0, x\_LL\^{}1, x\_LL\^{}2, x\_LL\^{}3) -$>$ (x\_LL\^{}0, -x\_LL\^{}1, -x\_LL\^{}2, -x\_LL\^{}3) & PASS\\
C-2 & the union of SL(2,C) and SL(2,C) e4 is closed under the product: (gL e4)(hL e4) = -hLbar gL, gL (hL e4) = (hL gL) e4, (gL e4) hL = (gL hLbar) e4; it is a Moufang loop (a Chein-type double of the Lorentz group) inside L\_LL, and it is not associative because SL(2,C) is not commutative & PASS\\
C-3 & inversion is not an automorphism of SL(2,C): for gL = e1, hL = e2, (gL hL)\^{}-1 = -e3 but gL\^{}-1 hL\^{}-1 = +e3; hence no operator on any vector space can implement gL -$>$ gL\^{}-1 by conjugation in a faithful representation (conjugation is multiplicative): the extension by e4 exists only as a loop & PASS\\
C-4 & the inner map x -$>$ e4 x e4\^{}-1 is not an automorphism of B\_LL (it fails on random pairs): inner maps of the loop are not symmetries of its multiplication, another place where associativity is missing & PASS\\
M-1 & F\_A with the twist c\_tw = -2 and c\_sl = 1 is exactly the commutator algebra m\_LL of the imaginary bi-octonions (all 196 basis brackets agree); it satisfies the Malcev identity on all 38416 basis 4-tuples and violates the Jacobi identity & PASS\\
M-2 & F\_A with c\_sl = 0: the twists c\_tw = 0, 1, -2 give Malcev algebras (exhaustive); c\_tw = 0 and c\_tw = 1 give Lie algebras, c\_tw = -2 does not (Carlsson's non-Lie Malcev module of sl(2)) & PASS\\
M-3 & excluded members of F\_A fail exhaustively: (c\_tw, c\_sl) = (1, 1) [Lie action and spinor charges closing into Lorentz], (0, 1), (2, 0), (-1, 0), (3, 0) violate both identities (in agreement with G-1, G-3, G-4)  -- first failing Malcev 4-tuple (positions in the basis) (0, 6, 6, 8) & PASS\\
M-4 & F\_B with the twist c\_tw = -2 and all other coefficients 0 (the abelian twisted module of the Poincare algebra) satisfies the Malcev identity on all 104976 basis 4-tuples and violates the Jacobi identity: internal spinor charges, commuting with the translations p\_LL, with the non-Lie Lorentz action  -- first failing Jacobi triple (positions in the basis) (0, 1, 10) & PASS\\
M-5 & the Lie solutions of F\_B found by G-6 are Lie algebras (exhaustive): Poincare with abelian spinor charges (c\_tw = 1), Poincare with [s\_LL, s\_LL] in p\_LL (c\_tw = 1, c\_sP = 1: bosonic spinor charges whose brackets are translations), and curved space-time with c\_PP = $|$c\_Ps$|$\^{}2 (c\_tw = 1, c\_PP = 1, c\_Ps = 1 and c\_PP = 2, c\_Ps = 1 + i): spinor charges that do not commute with the translations & PASS\\
M-6 & for c\_PP $>$ 0 the space-time part l\_LL + p\_LL (with [p\_LL, p\_LL] = c\_PP x Lorentz generator) has a Killing form of signature (4 positive, 6 negative): it is so(4,1), the de Sitter algebra; the spinor charges with [p\_LL, s\_LL] != 0 form its real 8-dimensional spinor representation  -- signature (4, 6, 0) & PASS\\
M-7 & excluded members of F\_B fail exhaustively: the bi-octonion bracket together with translations (c\_tw = -2, c\_sl = 1), the Lie action with c\_sl = 1 (with or without c\_sP = 1), and the twist together with c\_sP = 1 or with the curvature c\_PP = 1: with translations present, spinor charges never close into the Lorentz algebra (G-7, G-9)  -- first failing Malcev 4-tuple (positions in the basis) (0, 6, 10, 14) & PASS\\
M-8 & F\_C: the bi-octonion algebra m\_LL with a decoupled isospin algebra su(2) (c\_tw = -2, c\_sl = 1, c\_TT = 1, c\_Ts = 0, c\_sT = 0) is Malcev and not Lie; isospin acting on the spinor charges with the Lie Lorentz action and [s\_LL, s\_LL] = 0 (c\_tw = 1, c\_Ts = 1, c\_TT = 1) is a Lie algebra & PASS\\
M-9 & isospin cannot act on the spinor charges of m\_LL inside a Malcev algebra: adding c\_Ts = 1 (isospin acts on s\_LL), or c\_sT = 1 ([s\_LL, s\_LL] gets an isospin part), or both, to m\_LL + su(2) violates the Malcev identity (exhaustive; G-11): electric charge enters the super-loop as an outer operator (an automorphism), not as an element  -- first failing Malcev 4-tuple (positions in the basis) (0, 1, 6, 9) & PASS\\
N-1 & the Hermitian bi-octonions x = x\_LL\^{}0 + i sum\_p x\_LL\^{}p e\_p (x\^{}dag = x, real x\_LL\^{}0, x\_LL\^{}p, p = 1..7) form the 7+1 Minkowski space R\^{}(1,7)\_LL: the complex norm N\_LL is real on them, with signature (1 positive, 7 negative); the Hermitian bi-quaternions (p = 1,2,3) form R\^{}(1,3)\_LL, signature (1,3), on which SL(2,C) acts by x -$>$ gL x gL\^{}dag  -- R\^{}(1,7)\_LL: (1, 7, 0), R\^{}(1,3)\_LL: (1, 3, 0) & PASS\\
N-2 & the split octonions sit inside B\_LL as Os = H + (i H) e4: this real 8-dimensional subspace is closed under the product, the map from the split Cayley-Dickson algebra (pairs (aH, bS) of real quaternions with the sign +1) to B\_LL, (aH, bS) -$>$ aH + (i bS) e4, is an isomorphism onto Os, and N\_LL restricted to Os has signature (4,4): the 4+4 manifold is the real form Os of the super-loop's algebra; its unit elements form the pseudo-sphere S\^{}(3,4), a Moufang loop  -- signature (4, 4, 0) & PASS\\
N-3 & the space of spinor charges s\_LL (real dimension 8) carries the quadratic forms Re N\_LL and Im N\_LL (invariant under the Lorentz and isospin automorphisms, because N\_LL is multiplicative), both of signature (4,4): the base of the fibration of L\_LL over s\_LL (aH + bS e4 -$>$ bS, fibre a copy of SL(2,C) wherever N\_LL(aH) = 1 - N\_LL(bS) is not zero) is a 4+4 space  -- Re N\_LL (4, 4, 0), Im N\_LL (4, 4, 0) & PASS\\
N-4 & on the 7+1 manifold the one-parameter conjugations x -$>$ gL x gL\^{}dag by the loop elements gL = exp(theta e\_p) (rotations) and gL = exp(theta i e\_p) (boosts), p = 1..7, have the generators x -$>$ [e\_p, x] and x -$>$ i (e\_p x + x e\_p), which lie in so(1,7), and their Lie closure is all of so(1,7) (dimension 28, exact): the super-loop acts on R\^{}(1,7)\_LL through the Lorentz group SO(1,7)  -- closure dimension 28 & PASS\\
N-5 & conjugation by the loop's own Lorentz subgroup (p = 1,2,3) is not a group action on the 7+1 manifold: its six generators preserve R\^{}(1,3)\_LL and act on it as so(3,1), but their Lie closure on R\^{}(1,7)\_LL has dimension 9 = so(3,1) + so(3) (exact), the extra so(3) rotating only the four directions i e4..i e7 (there the boosts act trivially and the rotations by the twisted spinor action)  -- closure on R\^{}(1,7)\_LL: 9, restricted to R\^{}(1,3)\_LL: 6 & PASS\\
N-6 & the left multiplications generate Lie groups (their closures, exact): by imaginary bi-octonions, so(8,C) of real dimension 56 (every L\_x is complex-linear and preserves the complex norm N\_LL); by imaginary octonions on R\^{}8, so(8) (28); by imaginary split units on Os = R\^{}(4,4), so(4,4) (28; each generator preserves the split norm): any linear action of the loop is an action of these LIE groups  -- closures 56 / 28 / 28 & PASS\\
N-7 & the super-loop manifold (N\_LL(w) = 1, a complex 7-sphere of real dimension 14) is parallelized by the frame fields X\_x(w) = w x (x imaginary), which are tangent (N\_LL(w, w x) = 0); their Lie brackets are [X\_x, X\_y](w) = w [x,y] + 2 [w,x,y] (100 random w, x, y): at w = 1 this is the Malcev bracket, elsewhere the associator makes the structure functions non-constant: the manifold is parallelizable but carries no Lie group structure & PASS\\
N-8 & the 7-sphere S\^{}7 of unit octonions (the compact real form of the super-loop) is the homogeneous space Spin(7)/G2: the products L\_(e\_p) L\_(e\_p') (p $<$ p') span spin(7) (dimension 21, exact) acting on R\^{}8, the stabilizer of 1 has dimension 14 (g2) and the orbit of 1 has dimension 7  -- spin(7): 21, stabilizer: 14, orbit: 7 & PASS\\
N-9 & the split real form: inside the imaginary split octonions the elements e1, i e4, i e5 span a Lie subalgebra whose Killing form has signature (2 positive, 1 negative): sl(2,R) = so(2,1), the Lorentz algebra of 2+1 dimensions; the unit split octonions S\^{}(3,4) contain SL(2,R) = Spin(2,1): on the real 4+4 manifold the super-loop's Lorentz part is the 2+1 Lorentz group, and SO(3,1) requires the complex form  -- signature (2, 1, 0) & PASS\\
S-0 & field content from the bi-octonions: with the isomorphism C (x) H = M\_2(C), e\_j -$>$ -i sigma\_j (j = 1,2,3), the spinor charge bS, as the 2x2 matrix M\_LL(bS), gives the Weyl isodoublet psi'\_LL = eps2\^{}-1 M\_LL(bS)\^{}T, whose column 1 (isospin up) and column 2 (isospin down) are Weyl spinors; on it the Lorentz derivations are D\_LL(J\_LL\^{}j) = -i sigma\_j/2, D\_LL(K\_LL\^{}j) = +sigma\_j/2 (on each column), and the isospin generators are T\_LL\^{}iota = tau\_iota/2 (mixing the columns); the chiral Dirac matrices gamma\_LL\^{}p (p = 0..3) intertwine these spinor generators with the vector action V\_LL of the loop's conjugations on R\^{}(1,3)\_LL (all 24 relations [gamma\_LL\^{}p, D] = sum\_p' V\_LL\^{}p\_p' gamma\_LL\^{}p'), the spinor charges being the gamma\_ch = +1 block: Psi\_LL = Dirac (x) isodoublet & PASS\\
S-1 & Theorem SF1 (spin-zero channels): the antisymmetric, Lorentz-invariant pair matrices on Dirac (x) isospin form a complex space of dimension 6, spanned by Kch\_iota = C\_LL gamma\_ch (x) eps2 tau\_iota (scalar 0+) and Kch'\_iota = C\_LL (x) eps2 tau\_iota (pseudoscalar 0-), iota = 1,2,3; each is unitary and an isospin TRIPLET (Casimir 2): a spin-zero s-wave pair of spinor charges carries isospin 1 and electric charge Q\_LL = +1, 0, -1  -- real nullity 12 (= 6 complex dimensions) & PASS\\
S-2 & electric charge of the spin-zero channels (charge of the pair operator Psi\_LL\^{}T Kch Psi\_LL, which annihilates two spinor charges): Kch\_3 pairs up with down, charge 0; Kch\_1 - i Kch\_2 and Kch\_1 + i Kch\_2 pair up-up and down-down, charges +1 and -1  -- Q\_LL(Kch\_3) = 0, Q\_LL(Kch\_1 - i Kch\_2) = +1, Q\_LL(Kch\_1 + i Kch\_2) = -1 & PASS\\
S-3 & Theorem SF2 (spectra; 60 random momenta, masses, chemical potentials and order parameters dvec in C\^{}3, double precision): in the scalar channel the 8 positive BdG energies are E\^{}+(Delta\_+), E\^{}-(Delta\_+), E\^{}+(Delta\_-), E\^{}-(Delta\_-) (each twice) with E\^{}(+-)(Delta) = [(eps +- mu)\^{}2 + Delta\^{}2]\^{}(1/2), eps = (q\^{}2 + m\^{}2)\^{}(1/2), and the two gaps Delta\_(+-)\^{}2 = $|$dvec$|$\^{}2 (1 +- zeta), zeta = $|$cross(dvec, dvec\^{}*)$|$ / $|$dvec$|$\^{}2 (unitary states: zeta = 0; non-unitary: two gaps); in the pseudoscalar channel (unitary dvec) Etilde\^{}(+-) = [q\^{}2 + m\^{}2 + mu\^{}2 + Delta\^{}2 +- 2 (mu\^{}2 (q\^{}2 + m\^{}2) + m\^{}2 Delta\^{}2)\^{}(1/2)]\^{}(1/2), each four times  -- largest deviation 7.11e-15 & PASS\\
S-4 & critical coupling of the scalar channel at mu = 0 (gap equation 1/G = int d\^{}3q/(2 pi)\^{}3 (1/E\^{}+ + 1/E\^{}-), momentum cutoff Lambda): G\_crit = 2 pi\^{}2/Lambda\^{}2 = 19.7392 for m = 0, and G\_crit = 2 pi\^{}2/[Lambda (Lambda\^{}2 + m\^{}2)\^{}(1/2) - m\^{}2 ln((Lambda + (Lambda\^{}2 + m\^{}2)\^{}(1/2))/m)] for m $>$ 0 (m = 0.2: quadrature against the closed form)  -- G\_crit at m = 0: 19.739209, at m = 0.2: 21.286610 & PASS\\
S-5 & the gap of the unitary scalar state grows monotonically with the coupling (mu = 0.5, m = 0.2, 40 couplings), and at weak coupling ln Delta is linear in 1/G with the BCS slope -1/(2 nuF) = -pi\^{}2/(kF mu), nuF = kF mu/(2 pi\^{}2) the density of states of one species at the Fermi surface (ratio of the slopes within 5 \%): a Fermi surface pairs for every G $>$ 0  -- slope -43.0637, BCS value -43.0745, ratio 0.9997 & PASS\\
S-6 & Theorem SF3 (the unitary state wins): at G = 12, mu = 0.5, m = 0.2 the minimum over $|$dvec$|$ of Omega increases strictly with zeta from the unitary (polar) state zeta = 0 to the fully charged state zeta = 1 (concavity of E\^{}(+-) in Delta\^{}2): the ground state is a unitary isospin-triplet condensate  -- minimum of Omega at zeta = 0: -9.720605e-5, at zeta = 1: -4.860303e-5 & PASS\\
S-10 & which spin-zero channel condenses: at the same coupling (G = 12, mu = 0.5) the minimum of Omega in the scalar (0+) channel lies below that of the pseudoscalar (0-) channel for m = 0.2, and the two minima coincide for m = 0 (chiral symmetry maps one channel onto the other): for massive spinor charges the scalar isospin-triplet condensate is the ground state  -- m = 0.2: -9.720605e-5 (0+) and -4.685258e-5 (0-); m = 0: -1.384064e-4 and -1.384064e-4 & PASS\\
S-7 & symmetry breaking under SU(2)\_v x U(1)\_F\_LL (isospin and fermion number, 4 generators): the polar state dvec = (0,0,1) keeps the rotations about the 3-axis (stabilizer of dimension 1; the electric charge Q\_LL = T\_LL\^{}3 stays unbroken: a neutral superfluid), the charged state dvec = (1, i, 0) keeps one combination of T\_LL\^{}3 and F\_LL (stabilizer 1; its condensed pairs Psi\_LL\^{}T (Kch\_1 - i Kch\_2) Psi\_LL consist of two isospin-up spinor charges, charge Q\_LL = +1); both break 3 generators  -- polar: broken 3, stabilizer 1, pair charge 0; charged: broken 3, stabilizer 1, pair charge +1 & PASS\\
S-8 & the octonionic Cooper pair of two spinor-charge fields: (bS e4)(bS' e4) = -bS'bar bS splits into the scalar part -N\_LL(bS', bS) (Lorentz scalar AND isospin singlet: spin 0, isospin 0, charge 0: the super-loop singlet) and the imaginary part, which lies in the Lorentz part l\_LL (spin 1, isospin 0: the Malcev bracket); the scalar part is SYMMETRIC in the two fields, so it vanishes for one fermion field and needs two species (Theorem SF4) & PASS\\
S-9 & two-species singlet: for one field the isospin-singlet scalar C\_LL gamma\_ch (x) eps2 is symmetric (the pair vanishes); for two species the pair matrix Kch\_singlet = C\_LL gamma\_ch (x) eps2 (x) eps2 (spin 0, isospin 0, species singlet: the third factor acts on the species index; antisymmetric as fermions require) gives the 16 positive BdG energies E\^{}- (8 times) and E\^{}+ (8 times) with E\^{}(+-) = [(eps +- mu)\^{}2 + Delta\^{}2]\^{}(1/2): a fully gapped super-loop-singlet superfluid that breaks only the fermion number and keeps isospin, electric charge and Lorentz invariance  -- E\^{}- = 0.347098, E\^{}+ = 0.878250 & PASS\\
R-1 & mass dimensions in D + 1 space-time dimensions (D = number of space dimensions): [Psi\_LL] = D/2, [phi] = (D-1)/2, contact coupling [G] = 1 - D, Yukawa [g\_Y] = (3-D)/2, quartic [lam4] = 3 - D; D = 3: [G] = -2 (non-renormalizable), [g\_Y] = [lam4] = 0 (renormalizable); D = 7 (7+1, and 4+4, where power counting sees the total dimension 8): [G] = -6, [g\_Y] = -2, [lam4] = -4 (all non-renormalizable)  -- twice ([Psi\_LL], [phi], [G], [g\_Y], [lam4]): D = 3 (3, 2, -4, 0, 0), D = 7 (7, 6, -12, -4, -8) & PASS\\
R-2 & superficial degree of divergence deg (topological counting nloop = nint\_F + nint\_B - nvert + 1, deg = (D+1) nloop - nint\_F - 2 nint\_B, compared with the formulas on all diagrams with up to 6 vertices): contact theory deg = (D+1) - D next\_F/2 + (D-1) nvert, Yukawa-quartic theory deg = (D+1) - D next\_F/2 - (D-1) next\_B/2 + (D-3)(nvert\_Y/2 + nvert\_4); for D = 3 the contact theory's deg grows by 2 per vertex (non-renormalizable) and the Yukawa theory's does not depend on the vertices (renormalizable); for D = 7 both grow (non-renormalizable) & PASS\\
R-3 & the contact coupling must depend on the cutoff like Lambda\^{}-2: at a fixed physical gap (Delta = 0.05, mu = 0.5, m = 0.2) the bare coupling solving the gap equation obeys G Lambda\^{}2/(2 pi\^{}2) -$>$ 1 (within 2 \% at Lambda = 128): the cutoff cannot be removed with a finite number of logarithmic counterterms  -- G Lambda\^{}2/(2 pi\^{}2) at Lambda = 128: 0.99981 & PASS\\
R-4 & Yukawa completion (pair field phi coupled to Psi\_LL\^{}T Kch Psi\_LL): the one-loop bubble Ibub(pE) = int d\^{}4kE/(2 pi)\^{}4 [kE.(kE+pE) + m\^{}2]/((kE\^{}2 + m\^{}2)((kE+pE)\^{}2 + m\^{}2)) (Euclidean, four-dimensional cutoff Lambda; pE = 1, m = 0.5) has a quadratically growing part Ibub(0) (absorbed by the mass counterterm) and a logarithmic momentum dependence: d[Ibub(pE) - Ibub(0)]/d ln Lambda -$>$ -pE\^{}2/(16 pi\^{}2) (absorbed by the wave-function counterterm); no new structure is needed  -- slope -6.332440e-3, -pE\^{}2/(16 pi\^{}2) = -6.332574e-3 & PASS\\
R-5 & the renormalizable completion has finitely many couplings: the quartic polynomials of a complex isospin-triplet pair field phi (degree 2 in phi and 2 in phi\^{}*) invariant under the isospin rotations form a space of dimension exactly 2, spanned by (phi\^{}dag phi)\^{}2 and $|$phi\^{}T phi$|$\^{}2 (exact; with the Yukawa coupling and the masses this is the complete list)  -- dimension 2 & PASS\\
R-6 & invariance under the loop's linear action equals invariance under its Lie closure: the quadratic forms on the real 16-dimensional B\_LL invariant under the 14 left multiplications L\_x and those invariant under the 56-dimensional closure so(8,C) form the same 2-dimensional space (Re N\_LL and Im N\_LL, verified exactly): every counterterm allowed by the closure is allowed by the loop and conversely, so the super-loop pattern of a Lagrangian is as stable under renormalization as the closure symmetry  -- closure 56, invariant forms 2 / 2 & PASS\\
G-1 & F\_A (l\_LL + s\_LL), Jacobi identity: csl\_re and csl\_im lie in the ideal, so every LIE algebra of the family has [s\_LL, s\_LL] = 0 (no Lorentz part): bosonic spinor charges cannot close into so(3,1) (Theorem L) & PASS\\
G-2 & F\_A, Jacobi identity: Re and Im of c\_tw (c\_tw - 1) with c\_tw = ctw\_re + i ctw\_im, i.e. ctw\_re\^{}2 - ctw\_re - ctw\_im\^{}2 and ctw\_im (2 ctw\_re - 1), lie in the ideal; their only real common zeros are c\_tw = 0 and c\_tw = 1: the Lie action of so(3,1) on the spinor charges is 0 or the Weyl representation & PASS\\
G-3 & F\_A, Malcev identity: Re and Im of c\_tw (c\_tw - 1)(c\_tw + 2) with c\_tw = ctw\_re + i ctw\_im lie in the ideal; this complex cubic has only the real roots 0, 1, -2: the twist is 0, 1 (Lie) or -2 (non-Lie), and ctw\_im = 0 & PASS\\
G-4 & F\_A, Malcev identity: on every real solution with [s\_LL, s\_LL] != 0 (csl\_re\^{}2 + csl\_im\^{}2 != 0) the twist is c\_tw = -2 (ctw\_re + 2 and ctw\_im lie in the saturated ideal): the bi-octonion algebra m\_LL is the only solution with c\_sl != 0 (up to the rescaling bS -$>$ lambda\_sc bS by a complex number lambda\_sc != 0, which maps c\_sl to lambda\_sc\^{}2 c\_sl) & PASS\\
G-5 & F\_B (Poincare or (anti-)de Sitter + spinor charges), Jacobi identity: csl\_re, csl\_im, cPP\_im lie in the ideal; ctw\_im = 0 and ctw\_re (ctw\_re - 1) = 0 on real solutions (ctw\_im (4 ctw\_im\^{}2 + 1) lies in the ideal) & PASS\\
G-6 & F\_B, Jacobi identity: cPP\_re ctw\_re - (cPs\_re\^{}2 + cPs\_im\^{}2), csP (ctw\_re - 1), csP cPP\_re, csP cPs\_re, csP cPs\_im lie in the ideal: the Lie solutions are (i) Poincare with abelian spinor charges, (ii) Poincare with [s\_LL, s\_LL] in p\_LL (csP != 0, a bosonic analogue of supersymmetry), (iii) curvature cPP\_re = $|$c\_Ps$|$\^{}2 $>$= 0 with [p\_LL, s\_LL] != 0 and [s\_LL, s\_LL] = 0 & PASS\\
G-7 & F\_B, Malcev identity: csl\_re and csl\_im lie in the ideal: in the presence of translations (flat or curved) even a MALCEV algebra cannot close spinor charges into the Lorentz algebra & PASS\\
G-8 & F\_B, Malcev identity: Re and Im of c\_tw (c\_tw - 1)(c\_tw + 2) lie in the ideal (c\_tw = 0, 1 or -2 on real points), and cPs\_re (ctw\_re - 1), cPs\_im (ctw\_re - 1), csP (ctw\_re - 1), cPP\_re ctw\_re - $|$c\_Ps$|$\^{}2 lie in it: for the twist c\_tw = -2 one gets c\_Ps = 0, csP = 0, cPP\_re = 0, c\_sl = 0 & PASS\\
G-9 & F\_B, Malcev identity with the twist c\_tw = -2: the ideal contains ctw\_im, cPP\_re, cPP\_im, cPs\_re, cPs\_im, csl\_re, csl\_im, csP: the only non-Lie Malcev extension of the Poincare algebra by spinor charges in this family is the ABELIAN twisted module (spinor charges commute with each other and with the translations) & PASS\\
G-10 & F\_C (l\_LL + su(2) isospin + spinor charges), Jacobi identity: csl\_re\^{}2 + csl\_im\^{}2 lies in the ideal, so c\_sl = 0 on every real solution: every Lie algebra of the family has no Lorentz part in [s\_LL, s\_LL], even when isospin is present & PASS\\
G-11 & F\_C, Malcev identity: csl\_re $|$c\_Ts$|$\^{}2, csl\_im $|$c\_Ts$|$\^{}2, csl\_re $|$c\_sT$|$\^{}2, csl\_im $|$c\_sT$|$\^{}2 lie in the ideal: whenever the spinor charges close into the Lorentz algebra (c\_sl != 0), isospin acts trivially on them (c\_Ts = 0) and [s\_LL, s\_LL] has no isospin part (c\_sT = 0) -- isospin can join the super-loop only as an outer automorphism & PASS\\
G-12 & F\_C, Malcev identity: on real solutions with c\_sl != 0, G-11 gives $|$c\_Ts$|$\^{}2 = 0 and $|$c\_sT$|$\^{}2 = 0, hence c\_Ts = c\_sT = 0; adding cTs\_re, cTs\_im, csT\_re, csT\_im to the ideal and saturating by csl\_re\^{}2 + csl\_im\^{}2 != 0, the ideal contains ctw\_re + 2 and ctw\_im: the twist is -2 -- the spinor charges with the Lorentz bracket form the bi-octonion algebra m\_LL, and su(2) isospin decouples (it can act on m\_LL only by automorphisms) & PASS\\
LL-S1 & every symbol text is registered once (184 entries) and every plain-text name of the programs, figures and data files belongs to one entry (561 names) & PASS\\
LL-S2 & the 2 taken entries carry exactly the meaning of the MASTER registry from which they are taken; none of the 402 items of the 182 new entries is an item of either master registry (503 items compared); the 24 items that the registry of Revision\_NoGo also contains are exactly the 24 declared shared ones, each with the same object (\$SO(3)\$, \$SU(2)\$, \$U(1)\$, \$\textbackslash{}det\$, \$\textbackslash{}langle\textbackslash{} \textbackslash{}rangle\$, \$\textbackslash{}mathbb C\$, \$\textbackslash{}mathbb R\$, \$\textbackslash{}mathfrak g\_2\$, \$\textbackslash{}mathfrak\{so\}(3)\$, \$\textbackslash{}mathfrak\{so\}(3,1)\$, \$\textbackslash{}mathfrak\{so\}(4)\$, \$\textbackslash{}mathfrak\{so\}(4,4)\$, \$\textbackslash{}mathfrak\{so\}(8)\$, \$\textbackslash{}mathfrak\{so\}(8,\textbackslash{}mathbb C)\$, \$\textbackslash{}mathfrak\{su\}(2)\$, \$\textbackslash{}mathrm G\_2\$, \$\textbackslash{}mathrm\{Re\}\$, \$\textbackslash{}mathrm\{diag\}\$, \$\textbackslash{}mathrm\{tr\}\$, \$\textbackslash{}mathsf 1\_8\$, \$\textbackslash{}otimes\$, \$i\$, \$p\$, \$\{\}\^{}T\$) & PASS\\
LL-S3 & every column of the 94 data files data/sl\_*.csv, explanations/data/ex\_*.csv and test2/data/t2\_*.csv is a registered plain-text name or a descriptive word (200 different columns), and every file states its content in a line '\# content:' & PASS\\
LL-S4 & each of the 7 documents of Revision\_SuperLoop and the manuscript contains the complete table of the 184 registered symbols, so that every symbol it uses is defined in the document itself & PASS\\
LL-S5 & in the 7093 formulas of the documents, mu, Lambda, Omega, nu, Delta, zeta and varepsilon always carry the suffix LL & PASS\\
LL-S6 & in the 8218 formulas of the documents and of the new registry entries, no Latin letter except the index j, D, the imaginary unit i, the index p and the differential d stands without the suffix LL in its scripts, and the Greek letters that are symbols carry the suffix as well & PASS\\
LL-S7 & every one of the 4954 occurrences of a letter in bold, sans-serif, calligraphic, blackboard or fraktur type in the formulas of the documents is a registered symbol (69 registered styled letters) & PASS\\
LL-S8 & every one of the 3633 occurrences of a name with the suffix \_LL, of an algebra element, of a family coefficient or of a pair matrix in the 127 reports and data files (data/sl\_*, explanations/data/ex\_* and test2/data/t2\_*) is a registered plain-text name & PASS\\
LL-S9 & the 3721 formulas of the 1 built documents of the folder test2, with the explicit matrices and equations written by the programs inserted, satisfy LL-S5, LL-S6 and LL-S7 & PASS\\
LL-H1 & none of the 30 documents and document sources of Revision\_SuperLoop refers its reader to another document for an equation, a symbol or a definition & PASS\\
<!-- END checks_rows_tex -->

<!-- BEGIN nchecks_rust -->
62
<!-- END nchecks_rust -->

<!-- BEGIN nchecks_physics -->
74
<!-- END nchecks_physics -->

<!-- BEGIN assoc_nonzero -->
168
<!-- END assoc_nonzero -->

<!-- BEGIN assoc_zero -->
344
<!-- END assoc_zero -->

<!-- BEGIN jac_rank -->
14
<!-- END jac_rank -->

<!-- BEGIN killing_e1 -->
$-16$
<!-- END killing_e1 -->

<!-- BEGIN killing_ie1 -->
$16$
<!-- END killing_ie1 -->

<!-- BEGIN der_rank -->
228
<!-- END der_rank -->

<!-- BEGIN der_span -->
28
<!-- END der_span -->

<!-- BEGIN n_assoc_rows -->
28
<!-- END n_assoc_rows -->

<!-- BEGIN groebner_FA_jacobi -->
ctw_im*(4*ctw_im**2 + 1)
-ctw_im**2 + ctw_re**2 - ctw_re
ctw_im*(2*ctw_re - 1)
csl_re
csl_im
<!-- END groebner_FA_jacobi -->

<!-- BEGIN groebner_FA_malcev -->
ctw_im*(12*ctw_im**4 + 35*ctw_im**2 - 10*ctw_re + 13)
ctw_im*(12*ctw_im**2*ctw_re + 4*ctw_im**2 + 7*ctw_re - 1)
csl_im*(2*ctw_im**2*ctw_re - ctw_im**2 - 3*ctw_re - 6)
csl_im*ctw_im*(2*ctw_im**2 + 5*ctw_re + 7)
-3*ctw_im**2*ctw_re - ctw_im**2 + ctw_re**3 + ctw_re**2 - 2*ctw_re
-ctw_im*(ctw_im**2 - 3*ctw_re**2 - 2*ctw_re + 2)
csl_im*(ctw_im**2 + ctw_re**2 + 4*ctw_re + 4)
-csl_im*ctw_im + csl_re*ctw_re + 2*csl_re
csl_im*ctw_re + 2*csl_im + csl_re*ctw_im
<!-- END groebner_FA_malcev -->

<!-- BEGIN groebner_table -->
| family | identity | independent constraint polynomials | random tuples used | elements of the reduced Groebner basis |
|---|---|---|---|---|
| $\mathcal F^{\rm A}_{\rm LL}$ | Jacobi | 6 | 13 | 5 |
| $\mathcal F^{\rm A}_{\rm LL}$ | Malcev | 8 | 13 | 9 |
| $\mathcal F^{\rm B}_{\rm LL}$ | Jacobi | 19 | 15 | 19 |
| $\mathcal F^{\rm B}_{\rm LL}$ | Malcev | 85 | 20 | 20 |
| $\mathcal F^{\rm C}_{\rm LL}$ | Jacobi | 12 | 15 | 25 |
| $\mathcal F^{\rm C}_{\rm LL}$ | Malcev | 43 | 16 | 113 |
<!-- END groebner_table -->

<!-- BEGIN n_family_members -->
24
<!-- END n_family_members -->

<!-- BEGIN spectra_dev -->
$7.11\times10^{-15}$
<!-- END spectra_dev -->

<!-- BEGIN gcrit_m0 -->
$19.739209$
<!-- END gcrit_m0 -->

<!-- BEGIN gcrit_m02 -->
$21.286610$
<!-- END gcrit_m02 -->

<!-- BEGIN bcs_slope -->
$-43.0637$
<!-- END bcs_slope -->

<!-- BEGIN bcs_value -->
$-43.0745$
<!-- END bcs_value -->

<!-- BEGIN bcs_ratio -->
$0.9997$
<!-- END bcs_ratio -->

<!-- BEGIN omega_min_z0 -->
$-9.720605\times10^{-5}$
<!-- END omega_min_z0 -->

<!-- BEGIN omega_min_z1 -->
$-4.860303\times10^{-5}$
<!-- END omega_min_z1 -->

<!-- BEGIN omega_ratio_z1 -->
$0.500000$
<!-- END omega_ratio_z1 -->

<!-- BEGIN omega_scalar_m02 -->
$-9.720605\times10^{-5}$
<!-- END omega_scalar_m02 -->

<!-- BEGIN omega_pseudo_m02 -->
$-4.685258\times10^{-5}$
<!-- END omega_pseudo_m02 -->

<!-- BEGIN omega_m0 -->
$-1.384064\times10^{-4}$
<!-- END omega_m0 -->

<!-- BEGIN singlet_Em -->
$0.347098$
<!-- END singlet_Em -->

<!-- BEGIN singlet_Ep -->
$0.878250$
<!-- END singlet_Ep -->

<!-- BEGIN gap_G12 -->
$0.090417$
<!-- END gap_G12 -->

<!-- BEGIN gcrit_formula_value -->
$19.7392088022$
<!-- END gcrit_formula_value -->

<!-- BEGIN n_couplings -->
40
<!-- END n_couplings -->

<!-- BEGIN G_first -->
$6$
<!-- END G_first -->

<!-- BEGIN G_last -->
$29.4$
<!-- END G_last -->

<!-- BEGIN gap_first -->
$2.568874\times10^{-3}$
<!-- END gap_first -->

<!-- BEGIN gap_last -->
$0.688144$
<!-- END gap_last -->

<!-- BEGIN contact_running_128 -->
$0.99981$
<!-- END contact_running_128 -->

<!-- BEGIN bubble_slope -->
$-6.332440\times10^{-3}$
<!-- END bubble_slope -->

<!-- BEGIN bubble_target -->
$-6.332574\times10^{-3}$
<!-- END bubble_target -->

<!-- BEGIN bubble_ratio -->
$0.99998$
<!-- END bubble_ratio -->

<!-- BEGIN contact_running_table -->
| $\Lambda_{\rm LL}$ | $G_{\rm LL}$ | $G_{\rm LL}\Lambda_{\rm LL}^2/(2\pi^2)$ |
|---|---|---|
| 1 | 10.258986 | 0.519726 |
| 2 | 3.737871 | 0.757451 |
| 4 | 1.121104 | 0.908732 |
| 8 | 0.299414 | 0.970782 |
| 16 | 0.076436 | 0.991311 |
| 32 | 0.019228 | 0.997505 |
| 64 | 0.004816 | 0.999298 |
| 128 | 0.001205 | 0.999805 |
<!-- END contact_running_table -->

<!-- BEGIN bubble_table -->
| $\Lambda_{\rm LL}$ | $I^{\rm bub}_{\rm LL}(0)$ | $I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)$ |
|---|---|---|
| 10 | 0.62376810 | -0.01583480 |
| 20 | 2.52134855 | -0.02021827 |
| 40 | 10.11824336 | -0.02460619 |
| 80 | 40.51240394 | -0.02899523 |
| 160 | 162.09562965 | -0.03338454 |
<!-- END bubble_table -->

<!-- BEGIN data_files_table -->
| data file | content (header line of the file) | columns |
|---|---|---|
| `sl_associators.csv` | the non-zero associators [e_p1, e_p2, e_p3] = (e_p1 e_p2) e_p3 - e_p1 (e_p2 e_p3) of three units with p1 < p2 < p3; each equals coefficient x the unit result (all other orders follow by alternativity) | `unit1` `unit2` `unit3` `result` `coefficient` |
| `sl_contact_running.csv` | bare contact coupling G versus the cutoff Lambda at the fixed gap Delta = 0.05 (mu = 0.5, m = 0.2) | `Lambda` `G` `G_times_Lambda2` `G_times_Lambda2_over_2pi2` |
| `sl_malcev_families.csv` | members of the covariant families verified exhaustively: Jacobi on all basis triples, linearized Malcev on all basis 4-tuples | `family` `coefficients` `Lie` `Malcev` `failing_Malcev_4_tuples` |
| `sl_malcev_structure.csv` | non-zero commutators of the real basis elements of the Malcev algebra m_LL = C (x) Im O: [left, right] = coefficient x result (left before right in the basis order e1, e2, e3, ie1, ie2, ie3, e4..e7, ie4..ie7) | `left` `right` `result` `coefficient` |
| `sl_manifolds.csv` | manifolds that carry the Lorentz super-loop | `manifold` `real_dimension` `signature` `role` |
| `sl_multiplication_table.csv` | products of the units e0..e7 of B_LL (row: left factor, column: right factor); the bi-octonions are the complex combinations of the units | `left` `e0` `e1` `e2` `e3` `e4` `e5` `e6` `e7` |
| `sl_power_counting.csv` | superficial degree of divergence deg: theory 0 = contact (next_F = 4), theory 1 = Yukawa (next_F = 2, next_B = 0), versus the number of vertices nvert, for D = 3 and D = 7 space dimensions | `D` `theory` `nvert` `next_F` `next_B` `deg` |
| `sl_quantum_numbers.csv` | quantum numbers of the sectors of B_LL under the operators of the super-loop: Q_LL (eigenvalues with multiplicities), T_LL^2, Cas_LL, Lorentz type, spinor number exp(2 pi D_LL(J_LL^3)), twist charge Theta_LL | `sector` `real_dimension` `Q_LL` `T_LL^2` `Cas_LL` `Lorentz_type` `spinor_number` `Theta_LL` |
| `sl_sf_channels.csv` | pair channels of the spinor charges: pair matrix, spin and parity, isospin, charge Q_LL of the pair, remark | `pair_matrix` `spin_parity` `isospin` `pair_charge` `remark` |
| `sl_sf_dispersion.csv` | quasiparticle energies versus q at G = 12, mu = 0.5, m = 0.2: polar state E^-, E^+; charged state (zeta = 1) gapped branch E^-(sqrt(2) Delta) [its other branch is the normal \|eps - mu\|]; normal state \|eps - mu\|; pseudoscalar channel Etilde^-, Etilde^+ at the same Delta | `q` `E_minus_polar` `E_plus_polar` `E_minus_charged_gapped` `E_normal` `Etilde_minus` `Etilde_plus` |
| `sl_sf_gap_vs_coupling.csv` | unitary scalar (0+) isospin-triplet state: gap Delta and Omega(Delta) - Omega(0) versus the coupling G (cutoff Lambda = 1, mu = 0.5, m = 0.2); G_over_Gcrit_m0 is G divided by G_crit at mu = 0, m = 0 | `G` `G_over_Gcrit_m0` `Delta` `Omega_minus_Omega0` |
| `sl_sf_omega_zeta.csv` | minimum over \|dvec\| of Omega(dvec) - Omega(0) at fixed zeta = \|cross(dvec, dvec^*)\|/\|dvec\|^2 (G = 12, mu = 0.5, m = 0.2), the \|dvec\| of the minimum, and Omega at \|dvec\| = the gap of the polar state | `zeta` `abs_dvec_at_min` `Omega_min` `Omega_at_polar_gap` |
| `sl_sf_states.csv` | spin-zero isospin-triplet states of the superfluid of the spinor charges: order parameter dvec, zeta, dimension of the stabilizer in SU(2)_v x U(1)_F_LL, broken generators, charge of the condensed pairs, gaps | `state` `dvec` `zeta` `stabilizer_dim` `broken_generators` `pair_charge` `gaps` |
| `sl_yukawa_bubble.csv` | Euclidean bubble Ibub of the Yukawa completion versus the cutoff Lambda at pE = 1, m = 0.5; Ibub(0) grows like Lambda^2, Ibub(pE) - Ibub(0) like -(pE^2/16 pi^2) ln Lambda | `Lambda` `Ibub_0` `Ibub_pE` `Ibub_pE_minus_Ibub_0` |
<!-- END data_files_table -->

<!-- BEGIN data_files_tex -->
\texttt{sl\_associators.csv}: the non-zero associators [e\_p1, e\_p2, e\_p3] = (e\_p1 e\_p2) e\_p3 - e\_p1 (e\_p2 e\_p3) of three units with p1 $<$ p2 $<$ p3; each equals coefficient x the unit result (all other orders follow by alternativity).
\texttt{sl\_contact\_running.csv}: bare contact coupling G versus the cutoff Lambda at the fixed gap Delta = 0.05 (mu = 0.5, m = 0.2).
\texttt{sl\_malcev\_families.csv}: members of the covariant families verified exhaustively: Jacobi on all basis triples, linearized Malcev on all basis 4-tuples.
\texttt{sl\_malcev\_structure.csv}: non-zero commutators of the real basis elements of the Malcev algebra m\_LL = C (x) Im O: [left, right] = coefficient x result (left before right in the basis order e1, e2, e3, ie1, ie2, ie3, e4..e7, ie4..ie7).
\texttt{sl\_manifolds.csv}: manifolds that carry the Lorentz super-loop.
\texttt{sl\_multiplication\_table.csv}: products of the units e0..e7 of B\_LL (row: left factor, column: right factor); the bi-octonions are the complex combinations of the units.
\texttt{sl\_power\_counting.csv}: superficial degree of divergence deg: theory 0 = contact (next\_F = 4), theory 1 = Yukawa (next\_F = 2, next\_B = 0), versus the number of vertices nvert, for D = 3 and D = 7 space dimensions.
\texttt{sl\_quantum\_numbers.csv}: quantum numbers of the sectors of B\_LL under the operators of the super-loop: Q\_LL (eigenvalues with multiplicities), T\_LL\^{}2, Cas\_LL, Lorentz type, spinor number exp(2 pi D\_LL(J\_LL\^{}3)), twist charge Theta\_LL.
\texttt{sl\_sf\_channels.csv}: pair channels of the spinor charges: pair matrix, spin and parity, isospin, charge Q\_LL of the pair, remark.
\texttt{sl\_sf\_dispersion.csv}: quasiparticle energies versus q at G = 12, mu = 0.5, m = 0.2: polar state E\^{}-, E\^{}+; charged state (zeta = 1) gapped branch E\^{}-(sqrt(2) Delta) [its other branch is the normal $|$eps - mu$|$]; normal state $|$eps - mu$|$; pseudoscalar channel Etilde\^{}-, Etilde\^{}+ at the same Delta.
\texttt{sl\_sf\_gap\_vs\_coupling.csv}: unitary scalar (0+) isospin-triplet state: gap Delta and Omega(Delta) - Omega(0) versus the coupling G (cutoff Lambda = 1, mu = 0.5, m = 0.2); G\_over\_Gcrit\_m0 is G divided by G\_crit at mu = 0, m = 0.
\texttt{sl\_sf\_omega\_zeta.csv}: minimum over $|$dvec$|$ of Omega(dvec) - Omega(0) at fixed zeta = $|$cross(dvec, dvec\^{}*)$|$/$|$dvec$|$\^{}2 (G = 12, mu = 0.5, m = 0.2), the $|$dvec$|$ of the minimum, and Omega at $|$dvec$|$ = the gap of the polar state.
\texttt{sl\_sf\_states.csv}: spin-zero isospin-triplet states of the superfluid of the spinor charges: order parameter dvec, zeta, dimension of the stabilizer in SU(2)\_v x U(1)\_F\_LL, broken generators, charge of the condensed pairs, gaps.
\texttt{sl\_yukawa\_bubble.csv}: Euclidean bubble Ibub of the Yukawa completion versus the cutoff Lambda at pE = 1, m = 0.5; Ibub(0) grows like Lambda\^{}2, Ibub(pE) - Ibub(0) like -(pE\^{}2/16 pi\^{}2) ln Lambda.
\texttt{sl\_groebner\_bases.txt}: the reduced Groebner bases of the six constraint ideals.
\texttt{sl\_algebra\_checks.txt}, \texttt{sl\_malcev\_checks.txt}, \texttt{sl\_manifolds\_checks.txt}, \texttt{sl\_superfluid\_checks.txt}, \texttt{sl\_renorm\_checks.txt}, \texttt{sl\_groebner\_checks.txt}, \texttt{sl\_symbol\_checks.txt}, \texttt{sl\_self\_contained\_checks.txt}: the reports of Table~\ref{tab:checks}.
<!-- END data_files_tex -->

<!-- BEGIN n_rust_lines -->
3305
<!-- END n_rust_lines -->

<!-- BEGIN n_rust_files -->
9
<!-- END n_rust_files -->

<!-- BEGIN n_groebner_lines -->
486
<!-- END n_groebner_lines -->

<!-- BEGIN data_sha256 -->
280787dfd4c8336cb63a314a332559cdb91ab4de4aacfa9f3547ab748d1d9a22  sl_algebra_checks.txt
b0c7312ba54c997c485492d313dcba905f2e4fb09ed9cb1222382579acd5f3e3  sl_associators.csv
290e26ae38ad95f39199a10c75264cb0f326a75cb8973254aad5a0ba32da9073  sl_contact_running.csv
3239d96fd7eeafb436fea956d6ad4e73760b7f205a701b05ddfb5ebce9131008  sl_groebner_bases.txt
b4d99fc5fd7eefce05bbac017563ab864e787b80663915c5cbf50574cff318a6  sl_groebner_checks.txt
e26d9914e9d7a30eb9e75ea3e8776b9f87516ccaae225631666ad23e511e238d  sl_malcev_checks.txt
dff39dffa2f958ca1d0c2d5944393b5756db4b2816c5043334ab1af0a60d86f4  sl_malcev_families.csv
34e8d33fc5e375bcb35f4023ba732d622db74b530e651caea7e9603b6ada941a  sl_malcev_structure.csv
ab062b4a66b8a38abe410ffafdb0a95521cf30ba4e095eca6053d625627c323d  sl_manifolds.csv
13336d96de70487a043d6c5f06d5536f16269b1e19157ee445189847321ec16c  sl_manifolds_checks.txt
b1cfd3dfc2434a613909e50cc3a23ca0edfb7b16007129f6137e53c3755f53a0  sl_multiplication_table.csv
1892ba5e74d6afc965fce4dfd36b5b8d8e28f715e4e57b4b8f533c5dd42912d4  sl_power_counting.csv
4014ea2dd2f73847097be883f5ee73a6329f08e689f6e6784be3161c93cdf18f  sl_quantum_numbers.csv
4e18b556b2e26a20b3946694e192780fcafabdca0c92da4ede8835cb23960014  sl_renorm_checks.txt
0f3635dcb9d1f5156c2cb2183192a9600bcb34430f1285f28c8c7dac7a40b4f5  sl_sf_channels.csv
dea8b2c8284a70334768cc492bdd32d9d0ac7c723abcb31a1f2e95c8e3459495  sl_sf_dispersion.csv
8bd27baccf82bd3fe666951b10ee99e81b6de1d1299a1362bf774bc1fb83527b  sl_sf_gap_vs_coupling.csv
6dbce6906928d983723928c990f61e9fe9aab72c52de96dfe7bca9f2d7f6edd3  sl_sf_omega_zeta.csv
27d74793da4c0e42cbc1f0cf7a4bb0ef39f5b2e8dfc52bb7c836303bca86cb8a  sl_sf_states.csv
f6692aa0efd3b52ef97406c4f0853f0ea2163434526398653004f979f4657c62  sl_superfluid_checks.txt
171af3b33706ea76db8b74c266d31f86674d8e34ac0f3d61e244287cb7ae1508  sl_yukawa_bubble.csv
<!-- END data_sha256 -->

<!-- BEGIN figure_sha256 -->
2bc7a676537fa73fd08c8d30cc057d2055ebf53df2f813ed4e72d6cc71afe205  fig_sl_charges.pdf
ac70bdd826a0d65c59fb79b94d300393fc68015bbb4d0189fc053fe8e0802980  fig_sl_classification.pdf
aa7132242af09217d639b821c1c4acd1e79eb49085c8db55628ad08051db21bf  fig_sl_manifolds.pdf
e0f3c69f58033cc4248d9dee033dd0ebc19e2e520d0a2e243ce93f33f1725497  fig_sl_renorm.pdf
0a76d5ed9e6d22ddd2e0ab0c615562514220623621094a885f19a600e68481ed  fig_sl_superfluid.pdf
e02fc2cb28f14725fc0605e0ee87b6d85b3e99fc5fc8403722cf7ab6633a6e24  fig_sl_superloop.pdf
4f73b86dd47fe47973a6cc16e0ba68e9f50049c7510c484cacd75fbe0b52240d  fig_sl_tables.pdf
<!-- END figure_sha256 -->

<!-- BEGIN code_sha256 -->
bd886920b5c69449d0bfed2064654e91f6b6b8bcf98c75eeb2dec38a9538ed67  code/superloop-rs/src/algebra.rs
01e0867bd3f3bad05b555249fe79732fb71cdcb909fe16b9885f3dca734fcdea  code/superloop-rs/src/bioct.rs
ecc3d05e48b6a49eefe1fae96dcf340d7c54ca275d995db30ad018e7d651e2cd  code/superloop-rs/src/checks.rs
cc2abe0ad40727da94e17b2cc78891b41b564bde4ba56f460a5f7319f2ad6e78  code/superloop-rs/src/exact.rs
124f653bbc959e6d591e693dcbbb3812cd564dbd00047652029255c65ced70d9  code/superloop-rs/src/main.rs
def7dbfc144559e324d3337cd16d851fa0b0d8a17114fe81eadf830581af1a0e  code/superloop-rs/src/malcev.rs
d9f46fbb4a2bcbccfd2107fd6fbf41091e78352c1346d943a9ce6385423dba04  code/superloop-rs/src/manifolds.rs
b0097ad508f55fe34ce5d8ad72c4ac7a3d7cf7d919661ca1422d1dbc6b747d88  code/superloop-rs/src/renorm.rs
61a6fb93c9591631e820c538d003a551447945a2bafe2ae84652dcd056383765  code/superloop-rs/src/superfluid.rs
84ce114dda4a896f3a7286572de3301961e6f1d82cd208379b093187e64a4c4c  code/superloop-rs/Cargo.toml
f3abd67e3bc8b707583adcb4dd5aa5bdb5ec233237bb115dcf1d1f4163df436f  code/scripts/assemble_docs.py
fd9de7ac8794a7d0316d7c07d791807a180ce1eccfa53c19eed8f69ba4022ea1  code/scripts/build_manuscript.py
465063731c88f51a1d8af013d40f1cae7613007e63d3256e5ab26b364bd3b455  code/scripts/check_layout.py
4ba23b938888ca73c928afdbe5c03078510fe524398e1751a8f7bf76ba1725d4  code/scripts/check_self_contained.py
f3dc71f4d0583419b98d8f07fa63d846eccfa0d92d5b527edc5ea39db546c7ce  code/scripts/check_symbols.py
d915832d77bfc194020e1bbed3740c8420e309cbf54832b2e5b1439a1b4cdc27  code/scripts/make_figures.py
cf5b78e608bd8ed2c880ca46425d5b6a5daca31331ca90af1697e5885d95e621  code/scripts/malcev_groebner.py
cfdc99c1e280e6956e641694c6105e6758d2cb38a6a8054aa4c4f7da11c21eb3  code/scripts/tex2md.py
eba07ddfbe6b2d1431b3f92eb5e3a14caf1eb7f5cb9aaa72a5b7d9551c22e074  code/scripts/write_doc_tables.py
eaf249798df735ec716e6d6e061355c10ba835af2833a4b9947eec7f6ba54470  code/scripts/write_symbol_registry.py
e6f3025b7f959977a1075cda0e0c34f51e208d9e28db50bc5d4fffcef189a6ab  code/scripts/build_docs.sh
fbe2871bcdeaff8a54036e20a2c4dce52c856725df4b4f9aa324ffe65e90dc7d  code/scripts/check_all.sh
cad52ccfbd5b20a51f59cd044cc1bfb05cae4268d45581539f66d47145b7e24d  code/scripts/doc_header.tex
<!-- END code_sha256 -->

<!-- BEGIN ex_pair_matrices_table -->
| pair matrix | spin and parity | isospin | species | exchange signs of the Dirac, isospin and species factors | sign of the whole matrix | terms that survive | charge of the pair |
|-------------------------------|------|---------|------------|-------------|-------|-------|-----------|
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_1$ | 0$^+$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | mixture of $+1$ and $-1$ |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_2$ | 0$^+$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | mixture of $+1$ and $-1$ |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_3$ | 0$^+$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | 0 |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}$ | 0$^+$ | 0 (singlet) | one field | $-1$, $-1$, none | $+1$ | 0 | 0 |
| $C_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_1$ | 0$^-$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | mixture of $+1$ and $-1$ |
| $C_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_2$ | 0$^-$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | mixture of $+1$ and $-1$ |
| $C_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_3$ | 0$^-$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | 0 |
| $C_{\rm LL}\otimes \mathsf E_{\rm LL}$ | 0$^-$ | 0 (singlet) | one field | $-1$, $-1$, none | $+1$ | 0 | 0 |
| $\mathsf K^1_{\rm LL}-i\,\mathsf K^2_{\rm LL}$ | 0$^+$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 2 | $+1$ |
| $\mathsf K^3_{\rm LL}$ | 0$^+$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 4 | 0 |
| $\mathsf K^1_{\rm LL}+i\,\mathsf K^2_{\rm LL}$ | 0$^+$ | 1 (triplet) | one field | $-1$, $+1$, none | $-1$ | 2 | $-1$ |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\otimes \mathsf E_{\rm LL}=\mathsf K^{\rm s}_{\rm LL}$ | 0$^+$ | 0 (singlet) | two species, species singlet | $-1$, $-1$, $-1$ | $-1$ | 8 | 0 |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_3\otimes \mathsf E_{\rm LL}$ | 0$^+$ | 1 (triplet) | two species, species singlet | $-1$, $+1$, $-1$ | $+1$ | 0 | 0 |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\otimes \sigma^{\rm LL}_0$ | 0$^+$ | 0 (singlet) | two species, each with itself | $-1$, $-1$, $+1$ | $+1$ | 0 | 0 |
| $C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes \mathsf E_{\rm LL}\tau^{\rm LL}_3\otimes \sigma^{\rm LL}_0$ | 0$^+$ | 1 (triplet) | two species, each with itself | $-1$, $+1$, $+1$ | $-1$ | 8 | 0 |
<!-- END ex_pair_matrices_table -->

<!-- BEGIN ex_states_table -->
| example | $\vec d_{\rm LL}$ | fractions of pairs with charge $+1$, 0, $-1$ | average charge of a pair | $\zeta_{\rm LL}$ | gaps $\Delta^+_{\rm LL}$, $\Delta^-_{\rm LL}$ in units of $\lvert\vec d_{\rm LL}\rvert$ | unpaired | unbroken generator |
|------------|-----------------------|-------------|--------|-----|----------|---------|--------------------|
| polar along the third axis | $(0,0,1)$ | 0, 1, 0 | 0 | 0 | 1, 1 | none | isospin rotation about the axis (0, 0, 1): $T^3_{\rm LL}=Q_{\rm LL}$ |
| polar along the first axis | $(1,0,0)$ | 0.5, 0, 0.5 | 0 | 0 | 1, 1 | none | isospin rotation about the axis (1, 0, 0) |
| polar along (1,2,2)/3 | $(1,2,2)/3$ | 0.277778, 0.444444, 0.277778 | 0 | 0 | 1, 1 | none | isospin rotation about the axis (0.333333, 0.666667, 0.666667) |
| unitary with a phase | $\exp(i\pi/4)\,(0,0.6,0.8)$ | 0.18, 0.64, 0.18 | 0 | 0 | 1, 1 | none | isospin rotation about the axis (0, 0.6, 0.8) |
| partly non-unitary | $(1,i/2,0)/\sqrt{5/4}$ | 0.9, 0, 0.1 | 0.8 | 0.8 | 1.341641, 0.447214 | none | none |
| charged, up-up pairs | $(1,i,0)/\sqrt2$ | 1, 0, 0 | 1 | 1 | 1.414214, 0 | isospin down | $\tfrac12F_{\rm LL}-T^3_{\rm LL}$, the number of isospin-down particles |
| charged, down-down pairs | $(1,-i,0)/\sqrt2$ | 0, 0, 1 | $-1$ | 1 | 1.414214, 0 | isospin up | $\tfrac12F_{\rm LL}+T^3_{\rm LL}$, the number of isospin-up particles |
<!-- END ex_states_table -->

<!-- BEGIN ex_counting_table -->
| total spin | total isospin | species | states | antisymmetric | symmetric | allowed in an s-wave |
|-------|--------|------------------|------|----------|---------|----------|
| 0 | 0 | one kind | 1 | 0 | 1 | no |
| 0 | 1 | one kind | 3 | 3 | 0 | yes |
| 1 | 0 | one kind | 3 | 3 | 0 | yes |
| 1 | 1 | one kind | 9 | 0 | 9 | no |
| 0 | 0 | two species, 0 (singlet) | 1 | 1 | 0 | yes |
| 0 | 0 | two species, 1 (triplet) | 3 | 0 | 3 | no |
| 0 | 1 | two species, 0 (singlet) | 3 | 0 | 3 | no |
| 0 | 1 | two species, 1 (triplet) | 9 | 9 | 0 | yes |
| 1 | 0 | two species, 0 (singlet) | 3 | 0 | 3 | no |
| 1 | 0 | two species, 1 (triplet) | 9 | 9 | 0 | yes |
| 1 | 1 | two species, 0 (singlet) | 9 | 9 | 0 | yes |
| 1 | 1 | two species, 1 (triplet) | 27 | 0 | 27 | no |
<!-- END ex_counting_table -->

<!-- BEGIN ex_sha256 -->
3c4366a5d11a44c8b8dd21d05fceb379646f55db407cc41321c5501e595fe4d8  explanations/data/ex_pair_matrices.csv
f35b57b83de79a1de0b45f6f1d73c8be77ddf5ba0f13799460fb62039572687e  explanations/data/ex_r13_bubble.csv
71bc8ac7fb68d7be2ea856bb01d377bd41daf1d79d158b421542bd6bd6ed12f6  explanations/data/ex_r13_checks.txt
50cb49b9704a94ca75ae079a7ceba2500701094c050f9e568a196f53e0ef35c0  explanations/data/ex_r13_dimensions.csv
60ee44d86bc281eec7760086a5e8daeeed8bb09addd5cb5e2751e8003fa36f81  explanations/data/ex_r13_divergent_counts.csv
3798cda1fd127c65e1f28a1a2642469dc8c8e6ceb70326c10f446244a5da76ce  explanations/data/ex_r13_orbits.csv
6c90420cdc8456ee62b29f52deaf8b5effff9b090e19a101699212a916090ef9  explanations/data/ex_r13_points.csv
d159f9c35283f947a298ad9bc73174c65b6726638ca51b6238e3b62b35648415  explanations/data/ex_r13_power_counting.csv
0cd9ab8b4b09768f58ef444cb11ad65eb16a065b33946369472e26f6bc9a859d  explanations/data/ex_states.csv
68f5badc7b9182d2226e6f8d36650eb324462eed9ddfa65e039a16f3b0b2978a  explanations/data/ex_superfluid_checks.txt
77cdbabb5afa005bc90d98735c371a595858e22e79c1a8d5c9689c9a4c9361af  explanations/data/ex_two_particle_counting.csv
5f5026bbbaea06c1bf85883730514138297454ee5fd8681d1f509d66e2347c0a  explanations/figures/fig_ex_bookkeeping.pdf
f25a98e31da53ef2c337460ff305c58f17756ec4e571390ba10d7bea58f340f7  explanations/figures/fig_ex_energy.pdf
e37d43b994421650e4528f6905c3250b0e3ee073213f843f923e98957b36c06a  explanations/figures/fig_ex_pairs.pdf
b87a8b56edac29b13782c9c099366bf62012941b016959ce2a6d01f0fa3b8088  explanations/figures/fig_ex_r13_bubble.pdf
1680602d6f5ce8b915cde1b77866f4f553eb5762c8021ccbc68b3ca98595fba5  explanations/figures/fig_ex_r13_coordinates.pdf
fb97fd5bfaa95ea586d1352655b2c43e0267153d87da4a664cde7a153426d7a2  explanations/figures/fig_ex_r13_divergences.pdf
429cb476f5899835609b0b86caad227c4d13b3863554f7ef69531d93b455d3de  explanations/figures/fig_ex_r13_lorentz.pdf
9ebe88b30c37e04d72636b771d9045fe00bf8cadb355acc808ae6247601c7bf3  explanations/code/explain-rs/src/main.rs
ea0102d48fa91faba90a2f3df9090eb71be4d01d548fec47a38a296628a06e41  explanations/code/explain-rs/Cargo.toml
d54d506b7d88a51c7152057d89a12800edff59d1a83fc3c5ab1c5f67862a808f  explanations/code/make_explain_figures.py
d8a9fe9245e9b5b4d9eff79eca49fbd9d81ceaafd966face6a9b7ea47aa35da8  explanations/code/r13-rs/src/main.rs
563167b8f4074b5576af1fda712b23ac00695b3e1866377b0213eae834902a4e  explanations/code/r13-rs/Cargo.toml
d8a96a63b09fba62c1ef5900a690cd5993fc8138068aeee7f870fd69148c2952  explanations/code/make_r13_figures.py
<!-- END ex_sha256 -->

<!-- BEGIN n_explain_lines -->
977
<!-- END n_explain_lines -->

<!-- BEGIN ex_r13_points_table -->
| example | $x^0_{\rm LL}$ | $x^1_{\rm LL}$ | $x^2_{\rm LL}$ | $x^3_{\rm LL}$ | $N_{\rm LL}(\mathbf x)$ | kind |
|--------------------------------|---------|---------|---------|---------|------------|--------------------|
| the sample point | 2.000000 | 1.200000 | 1.500000 | 1.000000 | $-0.690000$ | spacelike |
| the event $(1,0,0,0)$ | 1.000000 | 0 | 0 | 0 | 1.000000 | timelike |
| after the boost along $x^1_{\rm LL}$ with the rapidity 0.8 | 1.337435 | 0.888106 | 0 | 0 | 1.000000 | timelike |
| the event $(0,1,0,0)$ | 0 | 1.000000 | 0 | 0 | $-1.000000$ | spacelike |
| after the rotation about $x^3_{\rm LL}$ by the angle $\pi/3$ | 0 | 0.500000 | 0.866025 | 0 | $-1.000000$ | spacelike |
| the point $\chi_{\rm LL}\chi_{\rm LL}^\dagger$ of $\chi_{\rm LL}=(1,0)$ | 0.500000 | 0 | 0 | 0.500000 | 0 | null (on the light cone) |
| after the boost of $\chi_{\rm LL}$ along $x^3_{\rm LL}$ with the rapidity 0.8 | 1.112770 | 0 | 0 | 1.112770 | 0 | null (on the light cone) |
<!-- END ex_r13_points_table -->

<!-- BEGIN ex_r13_dimensions_table -->
| space dimensions $D$ | space-time dimension | $[\Psi_{\rm LL}]$ | $[\phi_{\rm LL}]$ | $[G_{\rm LL}]$ | $[g^{\rm Y}_{\rm LL}]$ | $[\lambda^{(1)}_{\rm LL}]=[\lambda^{(2)}_{\rm LL}]$ | renormalizable by power counting |
|----------|----------|--------|--------|--------|--------|--------------|--------------------------|
| 3 | 4 | $\tfrac{3}{2}$ | 1 | $-2$ | 0 | 0 | contact form: no; Yukawa form: yes |
| 7 | 8 | $\tfrac{7}{2}$ | 3 | $-6$ | $-2$ | $-4$ | contact form: no; Yukawa form: no |
<!-- END ex_r13_dimensions_table -->

<!-- BEGIN ex_r13_divergent_table -->
| function | $n^{\rm ext,F}_{\rm LL}$ | $n^{\rm ext,B}_{\rm LL}$ | $\deg_{\rm LL}$ | the term of $\mathcal L^{\rm Y}_{\rm LL}$ that absorbs it |
|--------------------------|--------|--------|--------|----------------------------------------|
| self-energy of the pair field | 0 | 2 | 2 | $(m^\phi_{\rm LL})^2\lvert\phi_{\rm LL}\rvert^2$ and the kinetic term of $\phi_{\rm LL}$ |
| four pair fields | 0 | 4 | 0 | $\lambda^{(1)}_{\rm LL}$, $\lambda^{(2)}_{\rm LL}$ |
| self-energy of the fermions | 2 | 0 | 1 | $m_{\rm LL}\bar\Psi_{\rm LL}\Psi_{\rm LL}$ and the kinetic term of $\Psi_{\rm LL}$ |
| Yukawa vertex | 2 | 1 | 0 | $g^{\rm Y}_{\rm LL}$ |
<!-- END ex_r13_divergent_table -->

<!-- BEGIN ex_r13_counts_table -->
| form, dimensions | 1 | 2 | 3 | 4 | 5 | 6 |
|------------------------|----|----|----|----|----|----|
| Yukawa form, 3+1 | 1 | 4 | 4 | 4 | 4 | 4 |
| contact form, 3+1 | 1 | 2 | 3 | 4 | 4 | 5 |
| Yukawa form, 7+1 | 1 | 4 | 8 | 13 | 13 | 20 |
| contact form, 7+1 | 1 | 2 | 3 | 4 | 5 | 6 |
<!-- END ex_r13_counts_table -->

<!-- BEGIN ex_r13_bubble_table -->
| $\Lambda_{\rm LL}$ | $I^{\rm bub}_{\rm LL}(0)$ computed | $I^{\rm bub}_{\rm LL}(0)$ closed form | $d[I^{\rm bub}_{\rm LL}(p^{\rm E}_{\rm LL})-I^{\rm bub}_{\rm LL}(0)]/d\ln\Lambda_{\rm LL}$ | $I^{\rm ren}_{\rm LL}$ | change of $I^{\rm ren}_{\rm LL}$ from $\Lambda_{\rm LL}/2$ |
|------|--------------|--------------|------------------|--------------|----------------|
| 4 | $9.471253\times10^{-2}$ | $9.471253\times10^{-2}$ | $-6.233478\times10^{-3}$ | $1.560633\times10^{-3}$ | — |
| 8 | $3.964998\times10^{-1}$ | $3.964998\times10^{-1}$ | $-6.307835\times10^{-3}$ | $1.560665\times10^{-3}$ | $3.208453\times10^{-8}$ |
| 16 | $1.610164$ | $1.610164$ | $-6.326390\times10^{-3}$ | $1.560666\times10^{-3}$ | $4.968769\times10^{-10}$ |
| 32 | $6.471387$ | $6.471387$ | $-6.331028\times10^{-3}$ | $1.560666\times10^{-3}$ | $7.745994\times10^{-12}$ |
| 64 | $2.592286\times10^{1}$ | $2.592286\times10^{1}$ | $-6.332187\times10^{-3}$ | $1.560666\times10^{-3}$ | $1.200301\times10^{-13}$ |
| 128 | $1.037353\times10^{2}$ | $1.037353\times10^{2}$ | $-6.332477\times10^{-3}$ | $1.560666\times10^{-3}$ | $6.415875\times10^{-15}$ |
| 256 | $4.149918\times10^{2}$ | $4.149918\times10^{2}$ | $-6.332550\times10^{-3}$ | $1.560666\times10^{-3}$ | $2.449018\times10^{-14}$ |
| 512 | $1.660024\times10^{3}$ | $1.660024\times10^{3}$ | $-6.332568\times10^{-3}$ | $1.560666\times10^{-3}$ | $1.993965\times10^{-13}$ |
| 1024 | $6.640161\times10^{3}$ | $6.640161\times10^{3}$ | $-6.332572\times10^{-3}$ | $1.560666\times10^{-3}$ | $3.827856\times10^{-13}$ |
| 2048 | $2.656071\times10^{4}$ | $2.656071\times10^{4}$ | $-6.332574\times10^{-3}$ | $1.560666\times10^{-3}$ | $1.132425\times10^{-12}$ |
<!-- END ex_r13_bubble_table -->

<!-- BEGIN n_r13_lines -->
923
<!-- END n_r13_lines -->

<!-- BEGIN t2_sigma -->
$$\sigma^{\rm LL}_0=\begin{pmatrix}1&0\\0&1\end{pmatrix},\quad \sigma^{\rm LL}_1=\begin{pmatrix}0&1\\1&0\end{pmatrix},\quad \sigma^{\rm LL}_2=\begin{pmatrix}0&-i\\i&0\end{pmatrix},\quad \sigma^{\rm LL}_3=\begin{pmatrix}1&0\\0&-1\end{pmatrix},\quad \mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}.$$
<!-- END t2_sigma -->

<!-- BEGIN t2_gamma -->
$$\begin{aligned}\gamma^0_{\rm LL}&=\begin{pmatrix}0&0&1&0\\0&0&0&1\\1&0&0&0\\0&1&0&0\end{pmatrix},&\gamma^1_{\rm LL}&=\begin{pmatrix}0&0&0&1\\0&0&1&0\\0&-1&0&0\\-1&0&0&0\end{pmatrix},\\ \gamma^2_{\rm LL}&=\begin{pmatrix}0&0&0&-i\\0&0&i&0\\0&i&0&0\\-i&0&0&0\end{pmatrix},&\gamma^3_{\rm LL}&=\begin{pmatrix}0&0&1&0\\0&0&0&-1\\-1&0&0&0\\0&1&0&0\end{pmatrix},\\ \gamma^{\rm ch}_{\rm LL}&=\begin{pmatrix}-1&0&0&0\\0&-1&0&0\\0&0&1&0\\0&0&0&1\end{pmatrix},&\quad C_{\rm LL}&=\begin{pmatrix}0&1&0&0\\-1&0&0&0\\0&0&0&-1\\0&0&1&0\end{pmatrix}.\end{aligned}$$
<!-- END t2_gamma -->

<!-- BEGIN t2_alpha -->
$$\begin{aligned}\alpha^1_{\rm LL}&=\begin{pmatrix}0&-1&0&0\\-1&0&0&0\\0&0&0&1\\0&0&1&0\end{pmatrix},&\alpha^2_{\rm LL}&=\begin{pmatrix}0&i&0&0\\-i&0&0&0\\0&0&0&-i\\0&0&i&0\end{pmatrix},\\ \alpha^3_{\rm LL}&=\begin{pmatrix}-1&0&0&0\\0&1&0&0\\0&0&1&0\\0&0&0&-1\end{pmatrix},&\beta_{\rm LL}&=\begin{pmatrix}0&0&1&0\\0&0&0&1\\1&0&0&0\\0&1&0&0\end{pmatrix}.\end{aligned}$$
<!-- END t2_alpha -->

<!-- BEGIN t2_lorentz -->
$$\begin{aligned}M_{\rm LL}(J^1_{\rm LL})&=\begin{pmatrix}0&-\tfrac{i}{2}\\-\tfrac{i}{2}&0\end{pmatrix},&M_{\rm LL}(K^1_{\rm LL})&=\begin{pmatrix}0&\tfrac{1}{2}\\\tfrac{1}{2}&0\end{pmatrix},\\ M_{\rm LL}(J^2_{\rm LL})&=\begin{pmatrix}0&-\tfrac{1}{2}\\\tfrac{1}{2}&0\end{pmatrix},&M_{\rm LL}(K^2_{\rm LL})&=\begin{pmatrix}0&-\tfrac{i}{2}\\\tfrac{i}{2}&0\end{pmatrix},\\ M_{\rm LL}(J^3_{\rm LL})&=\begin{pmatrix}-\tfrac{i}{2}&0\\0&\tfrac{i}{2}\end{pmatrix},&M_{\rm LL}(K^3_{\rm LL})&=\begin{pmatrix}\tfrac{1}{2}&0\\0&-\tfrac{1}{2}\end{pmatrix},\end{aligned}$$

$$\begin{aligned}\mathsf s_{\rm LL}(J^1_{\rm LL})&=\begin{pmatrix}0&-\tfrac{i}{2}&0&0\\-\tfrac{i}{2}&0&0&0\\0&0&0&-\tfrac{i}{2}\\0&0&-\tfrac{i}{2}&0\end{pmatrix},&\mathsf s_{\rm LL}(K^1_{\rm LL})&=\begin{pmatrix}0&-\tfrac{1}{2}&0&0\\-\tfrac{1}{2}&0&0&0\\0&0&0&\tfrac{1}{2}\\0&0&\tfrac{1}{2}&0\end{pmatrix},\\ \mathsf s_{\rm LL}(J^2_{\rm LL})&=\begin{pmatrix}0&-\tfrac{1}{2}&0&0\\\tfrac{1}{2}&0&0&0\\0&0&0&-\tfrac{1}{2}\\0&0&\tfrac{1}{2}&0\end{pmatrix},&\mathsf s_{\rm LL}(K^2_{\rm LL})&=\begin{pmatrix}0&\tfrac{i}{2}&0&0\\-\tfrac{i}{2}&0&0&0\\0&0&0&-\tfrac{i}{2}\\0&0&\tfrac{i}{2}&0\end{pmatrix},\\ \mathsf s_{\rm LL}(J^3_{\rm LL})&=\begin{pmatrix}-\tfrac{i}{2}&0&0&0\\0&\tfrac{i}{2}&0&0\\0&0&-\tfrac{i}{2}&0\\0&0&0&\tfrac{i}{2}\end{pmatrix},&\mathsf s_{\rm LL}(K^3_{\rm LL})&=\begin{pmatrix}-\tfrac{1}{2}&0&0&0\\0&\tfrac{1}{2}&0&0\\0&0&\tfrac{1}{2}&0\\0&0&0&-\tfrac{1}{2}\end{pmatrix}.\end{aligned}$$
<!-- END t2_lorentz -->

<!-- BEGIN t2_T1 -->
$$T^1_{\rm LL}=\begin{pmatrix}0&\tfrac{1}{2}&0&0&0&0&0&0\\\tfrac{1}{2}&0&0&0&0&0&0&0\\0&0&0&\tfrac{1}{2}&0&0&0&0\\0&0&\tfrac{1}{2}&0&0&0&0&0\\0&0&0&0&0&\tfrac{1}{2}&0&0\\0&0&0&0&\tfrac{1}{2}&0&0&0\\0&0&0&0&0&0&0&\tfrac{1}{2}\\0&0&0&0&0&0&\tfrac{1}{2}&0\end{pmatrix}$$
<!-- END t2_T1 -->

<!-- BEGIN t2_K1 -->
$$\mathsf K^1_{\rm LL}=\begin{pmatrix}0&0&-1&0&0&0&0&0\\0&0&0&1&0&0&0&0\\1&0&0&0&0&0&0&0\\0&-1&0&0&0&0&0&0\\0&0&0&0&0&0&-1&0\\0&0&0&0&0&0&0&1\\0&0&0&0&1&0&0&0\\0&0&0&0&0&-1&0&0\end{pmatrix}$$
<!-- END t2_K1 -->

<!-- BEGIN t2_T2 -->
$$T^2_{\rm LL}=\begin{pmatrix}0&-\tfrac{i}{2}&0&0&0&0&0&0\\\tfrac{i}{2}&0&0&0&0&0&0&0\\0&0&0&-\tfrac{i}{2}&0&0&0&0\\0&0&\tfrac{i}{2}&0&0&0&0&0\\0&0&0&0&0&-\tfrac{i}{2}&0&0\\0&0&0&0&\tfrac{i}{2}&0&0&0\\0&0&0&0&0&0&0&-\tfrac{i}{2}\\0&0&0&0&0&0&\tfrac{i}{2}&0\end{pmatrix}$$
<!-- END t2_T2 -->

<!-- BEGIN t2_K2 -->
$$\mathsf K^2_{\rm LL}=\begin{pmatrix}0&0&-i&0&0&0&0&0\\0&0&0&-i&0&0&0&0\\i&0&0&0&0&0&0&0\\0&i&0&0&0&0&0&0\\0&0&0&0&0&0&-i&0\\0&0&0&0&0&0&0&-i\\0&0&0&0&i&0&0&0\\0&0&0&0&0&i&0&0\end{pmatrix}$$
<!-- END t2_K2 -->

<!-- BEGIN t2_T3 -->
$$T^3_{\rm LL}=\begin{pmatrix}\tfrac{1}{2}&0&0&0&0&0&0&0\\0&-\tfrac{1}{2}&0&0&0&0&0&0\\0&0&\tfrac{1}{2}&0&0&0&0&0\\0&0&0&-\tfrac{1}{2}&0&0&0&0\\0&0&0&0&\tfrac{1}{2}&0&0&0\\0&0&0&0&0&-\tfrac{1}{2}&0&0\\0&0&0&0&0&0&\tfrac{1}{2}&0\\0&0&0&0&0&0&0&-\tfrac{1}{2}\end{pmatrix}$$
<!-- END t2_T3 -->

<!-- BEGIN t2_K3 -->
$$\mathsf K^3_{\rm LL}=\begin{pmatrix}0&0&0&1&0&0&0&0\\0&0&1&0&0&0&0&0\\0&-1&0&0&0&0&0&0\\-1&0&0&0&0&0&0&0\\0&0&0&0&0&0&0&1\\0&0&0&0&0&0&1&0\\0&0&0&0&0&-1&0&0\\0&0&0&0&-1&0&0&0\end{pmatrix}$$
<!-- END t2_K3 -->

<!-- BEGIN t2_P -->
$$\mathsf P_{\rm LL}=\sigma^{\rm LL}_0\otimes\begin{pmatrix}0&0&d^{1}_{\rm LL}-i\,d^{2}_{\rm LL}&-d^{3}_{\rm LL}\\0&0&-d^{3}_{\rm LL}&-d^{1}_{\rm LL}-i\,d^{2}_{\rm LL}\\-d^{1}_{\rm LL}+i\,d^{2}_{\rm LL}&d^{3}_{\rm LL}&0&0\\d^{3}_{\rm LL}&d^{1}_{\rm LL}+i\,d^{2}_{\rm LL}&0&0\end{pmatrix}$$
<!-- END t2_P -->

<!-- BEGIN t2_Gamma4 -->
$$\begin{aligned}-M_{\rm LL}(\mathbf u^{\omega}_{{\rm LL},p})^\dagger&=\tfrac12\begin{pmatrix}-i\,\omega^{3}_{{\rm LL},p}-\varpi^{3}_{{\rm LL},p}&-i\,\omega^{1}_{{\rm LL},p}-\varpi^{1}_{{\rm LL},p}-\omega^{2}_{{\rm LL},p}+i\,\varpi^{2}_{{\rm LL},p}\\-i\,\omega^{1}_{{\rm LL},p}-\varpi^{1}_{{\rm LL},p}+\omega^{2}_{{\rm LL},p}-i\,\varpi^{2}_{{\rm LL},p}&i\,\omega^{3}_{{\rm LL},p}+\varpi^{3}_{{\rm LL},p}\end{pmatrix},\\ M_{\rm LL}(\mathbf u^{\omega}_{{\rm LL},p})&=\tfrac12\begin{pmatrix}-i\,\omega^{3}_{{\rm LL},p}+\varpi^{3}_{{\rm LL},p}&-i\,\omega^{1}_{{\rm LL},p}+\varpi^{1}_{{\rm LL},p}-\omega^{2}_{{\rm LL},p}-i\,\varpi^{2}_{{\rm LL},p}\\-i\,\omega^{1}_{{\rm LL},p}+\varpi^{1}_{{\rm LL},p}+\omega^{2}_{{\rm LL},p}+i\,\varpi^{2}_{{\rm LL},p}&i\,\omega^{3}_{{\rm LL},p}-\varpi^{3}_{{\rm LL},p}\end{pmatrix}.\end{aligned}$$
<!-- END t2_Gamma4 -->

<!-- BEGIN t2_Acal -->
$$(\mathsf A_{{\rm LL},j})_{11}=(\mathsf A_{{\rm LL},j})_{33}=(\mathsf A_{{\rm LL},j})_{55}=(\mathsf A_{{\rm LL},j})_{77}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j},$$
$$(\mathsf A_{{\rm LL},j})_{12}=(\mathsf A_{{\rm LL},j})_{34}=(\mathsf A_{{\rm LL},j})_{56}=(\mathsf A_{{\rm LL},j})_{78}=\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j},$$
$$(\mathsf A_{{\rm LL},j})_{21}=(\mathsf A_{{\rm LL},j})_{43}=(\mathsf A_{{\rm LL},j})_{65}=(\mathsf A_{{\rm LL},j})_{87}=\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j},$$
$$(\mathsf A_{{\rm LL},j})_{22}=(\mathsf A_{{\rm LL},j})_{44}=(\mathsf A_{{\rm LL},j})_{66}=(\mathsf A_{{\rm LL},j})_{88}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j},$$
<!-- END t2_Acal -->

<!-- BEGIN t2_V0 -->
$$(\mathsf A_{{\rm LL},0})_{11}=(\mathsf A_{{\rm LL},0})_{33}=(\mathsf A_{{\rm LL},0})_{55}=(\mathsf A_{{\rm LL},0})_{77}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0},$$
$$(\mathsf A_{{\rm LL},0})_{12}=(\mathsf A_{{\rm LL},0})_{34}=(\mathsf A_{{\rm LL},0})_{56}=(\mathsf A_{{\rm LL},0})_{78}=\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0},$$
$$(\mathsf A_{{\rm LL},0})_{21}=(\mathsf A_{{\rm LL},0})_{43}=(\mathsf A_{{\rm LL},0})_{65}=(\mathsf A_{{\rm LL},0})_{87}=\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0},$$
$$(\mathsf A_{{\rm LL},0})_{22}=(\mathsf A_{{\rm LL},0})_{44}=(\mathsf A_{{\rm LL},0})_{66}=(\mathsf A_{{\rm LL},0})_{88}=g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0},$$
<!-- END t2_V0 -->

<!-- BEGIN t2_Gamma8 -->
$$(\Gamma^{\rm LL}_p)_{11}=(\Gamma^{\rm LL}_p)_{22}=-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},p}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{13}=(\Gamma^{\rm LL}_p)_{24}=-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},p}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},p}-\tfrac{1}{2}\,\omega^{2}_{{\rm LL},p}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{31}=(\Gamma^{\rm LL}_p)_{42}=-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},p}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},p}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},p}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{33}=(\Gamma^{\rm LL}_p)_{44}=\tfrac{i}{2}\,\omega^{3}_{{\rm LL},p}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{55}=(\Gamma^{\rm LL}_p)_{66}=-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},p}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{57}=(\Gamma^{\rm LL}_p)_{68}=-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},p}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},p}-\tfrac{1}{2}\,\omega^{2}_{{\rm LL},p}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{75}=(\Gamma^{\rm LL}_p)_{86}=-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},p}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},p}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},p}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},p},$$
$$(\Gamma^{\rm LL}_p)_{77}=(\Gamma^{\rm LL}_p)_{88}=\tfrac{i}{2}\,\omega^{3}_{{\rm LL},p}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},p}.$$
<!-- END t2_Gamma8 -->

<!-- BEGIN t2_nabla_components -->
$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_1&=\partial_j\psi^{\rm p}_{{\rm LL},1}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_2&=\partial_j\psi^{\rm p}_{{\rm LL},2}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_3&=\partial_j\psi^{\rm p}_{{\rm LL},3}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},1}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_4&=\partial_j\psi^{\rm p}_{{\rm LL},4}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},2}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},3}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_5&=\partial_j\psi^{\rm p}_{{\rm LL},5}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_6&=\partial_j\psi^{\rm p}_{{\rm LL},6}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_7&=\partial_j\psi^{\rm p}_{{\rm LL},7}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},5}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\psi^{\rm p}_{\rm LL})_8&=\partial_j\psi^{\rm p}_{{\rm LL},8}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},6}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},7}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm p}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_1&=\partial_j\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_2&=\partial_j\psi^{\rm h}_{{\rm LL},2}+\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_3&=\partial_j\psi^{\rm h}_{{\rm LL},3}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_4&=\partial_j\psi^{\rm h}_{{\rm LL},4}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},2}\\
&\quad -\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},3}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_5&=\partial_j\psi^{\rm h}_{{\rm LL},5}+\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_6&=\partial_j\psi^{\rm h}_{{\rm LL},6}+\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_7&=\partial_j\psi^{\rm h}_{{\rm LL},7}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{{\rm LL}*}_j\psi^{\rm h}_{\rm LL})_8&=\partial_j\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},6}\\
&\quad -\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},7}+i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\psi^{\rm h}_{{\rm LL},8}.
\end{aligned}$$
<!-- END t2_nabla_components -->

<!-- BEGIN t2_nabla_Psi_components -->
$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_1&=\partial_j\Psi_{{\rm LL},1}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},1}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},3}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},3}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},3}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},3}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},1}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},1}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},2}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_2&=\partial_j\Psi_{{\rm LL},2}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},2}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},4}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},4}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},4}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},1}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},1}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},2}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_3&=\partial_j\Psi_{{\rm LL},3}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},1}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},1}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},1}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},3}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},3}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},3}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},3}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_4&=\partial_j\Psi_{{\rm LL},4}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},2}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},2}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},4}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},4}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},3}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},3}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},4}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_5&=\partial_j\Psi_{{\rm LL},5}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},5}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},7}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},7}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},7}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},7}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},5}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},5}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},6}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_6&=\partial_j\Psi_{{\rm LL},6}-\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},6}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},8}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},8}-\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},8}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},5}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},5}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},6}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_7&=\partial_j\Psi_{{\rm LL},7}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},5}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},5}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},5}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},7}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},7}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},7}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},7}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
(\nabla^{\rm LL}_j\Psi_{\rm LL})_8&=\partial_j\Psi_{{\rm LL},8}-\tfrac{i}{2}\,\omega^{1}_{{\rm LL},j}\Psi_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{1}_{{\rm LL},j}\Psi_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{2}_{{\rm LL},j}\Psi_{{\rm LL},6}+\tfrac{i}{2}\,\varpi^{2}_{{\rm LL},j}\Psi_{{\rm LL},6}\\
&\quad +\tfrac{i}{2}\,\omega^{3}_{{\rm LL},j}\Psi_{{\rm LL},8}-\tfrac{1}{2}\,\varpi^{3}_{{\rm LL},j}\Psi_{{\rm LL},8}-\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},j}\Psi_{{\rm LL},7}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},j}\Psi_{{\rm LL},7}-i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}\Psi_{{\rm LL},8}\\
&\quad +\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},j}\Psi_{{\rm LL},8}.
\end{aligned}$$
<!-- END t2_nabla_Psi_components -->

<!-- BEGIN t2_bdg_equations -->
$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},1}&=i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_3+(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_3+i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_1+m_{\rm LL}\psi^{\rm p}_{{\rm LL},5}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},1}\\
&\quad +d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},3}-i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},3}-d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},2}&=i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_4+(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_4+i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_2+m_{\rm LL}\psi^{\rm p}_{{\rm LL},6}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},2}\\
&\quad -d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},3}-d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},4}-i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},3}&=i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_1-(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_1-i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_3+m_{\rm LL}\psi^{\rm p}_{{\rm LL},7}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},1}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},3}\\
&\quad -d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},1}+i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},1}+d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},4}&=i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_2-(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_2-i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_4+m_{\rm LL}\psi^{\rm p}_{{\rm LL},8}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},3}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},4}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},4}\\
&\quad +d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},1}+d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},2}+i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},5}&=-i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_7-(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_7-i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_5+m_{\rm LL}\psi^{\rm p}_{{\rm LL},1}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},5}\\
&\quad +d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},7}-i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},7}-d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},6}&=-i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_8-(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_8-i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_6+m_{\rm LL}\psi^{\rm p}_{{\rm LL},2}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},6}\\
&\quad -d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},7}-d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},8}-i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},7}&=-i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_5+(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_5+i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_7+m_{\rm LL}\psi^{\rm p}_{{\rm LL},3}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},5}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},7}\\
&\quad -d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},5}+i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},5}+d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm p}_{{\rm LL},8}&=-i\,(\nabla^{\rm LL}_1\psi^{\rm p}_{\rm LL})_6+(\nabla^{\rm LL}_2\psi^{\rm p}_{\rm LL})_6+i\,(\nabla^{\rm LL}_3\psi^{\rm p}_{\rm LL})_8+m_{\rm LL}\psi^{\rm p}_{{\rm LL},4}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},7}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm p}_{{\rm LL},8}-\mu_{\rm LL}\psi^{\rm p}_{{\rm LL},8}\\
&\quad +d^{3}_{\rm LL}\psi^{\rm h}_{{\rm LL},5}+d^{1}_{\rm LL}\psi^{\rm h}_{{\rm LL},6}+i\,d^{2}_{\rm LL}\psi^{\rm h}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},1}&=i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_3-(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_3+i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_1-m_{\rm LL}\psi^{\rm h}_{{\rm LL},5}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}\\
&\quad +\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}\\
&\quad +\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},1}\\
&\quad -(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},3}-i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},3}+(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},2}&=i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_4-(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_4+i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_2-m_{\rm LL}\psi^{\rm h}_{{\rm LL},6}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}\\
&\quad +\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},2}\\
&\quad +(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},3}+(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},4}-i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},4},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},3}&=i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_1+(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_1-i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_3-m_{\rm LL}\psi^{\rm h}_{{\rm LL},7}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}\\
&\quad +\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},1}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},3}\\
&\quad +(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},1}+i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},1}-(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},4}&=i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_2+(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_2-i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_4-m_{\rm LL}\psi^{\rm h}_{{\rm LL},8}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},3}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},2}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},4}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},4}\\
&\quad -(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},1}-(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},2}+i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},2},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},5}&=-i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_7+(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_7-i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_5-m_{\rm LL}\psi^{\rm h}_{{\rm LL},1}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}\\
&\quad +\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}\\
&\quad +\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},5}\\
&\quad -(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},7}-i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},7}+(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},6}&=-i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_8+(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_8-i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_6-m_{\rm LL}\psi^{\rm h}_{{\rm LL},2}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}\\
&\quad +\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},6}\\
&\quad +(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},7}+(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},8}-i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},8},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},7}&=-i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_5-(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_5+i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_7-m_{\rm LL}\psi^{\rm h}_{{\rm LL},3}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}\\
&\quad +\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},5}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},7}\\
&\quad +(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},5}+i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},5}-(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},6},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\psi^{\rm h}_{{\rm LL},8}&=-i\,(\nabla^{{\rm LL}*}_1\psi^{\rm h}_{\rm LL})_6-(\nabla^{{\rm LL}*}_2\psi^{\rm h}_{\rm LL})_6+i\,(\nabla^{{\rm LL}*}_3\psi^{\rm h}_{\rm LL})_8-m_{\rm LL}\psi^{\rm h}_{{\rm LL},4}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},7}+g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},6}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\psi^{\rm h}_{{\rm LL},8}+\mu_{\rm LL}\psi^{\rm h}_{{\rm LL},8}\\
&\quad -(d^{3}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},5}-(d^{1}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},6}+i\,(d^{2}_{\rm LL})^*\psi^{\rm p}_{{\rm LL},6}.
\end{aligned}$$
<!-- END t2_bdg_equations -->

<!-- BEGIN t2_Pi -->
$$\begin{aligned}
\Pi^1_{\rm LL}&=-2\Psi_{{\rm LL},1}\Psi_{{\rm LL},3}+2\Psi_{{\rm LL},2}\Psi_{{\rm LL},4}-2\Psi_{{\rm LL},5}\Psi_{{\rm LL},7}+2\Psi_{{\rm LL},6}\Psi_{{\rm LL},8},\\
\Pi^2_{\rm LL}&=-2i\,\Psi_{{\rm LL},1}\Psi_{{\rm LL},3}-2i\,\Psi_{{\rm LL},2}\Psi_{{\rm LL},4}-2i\,\Psi_{{\rm LL},5}\Psi_{{\rm LL},7}-2i\,\Psi_{{\rm LL},6}\Psi_{{\rm LL},8},\\
\Pi^3_{\rm LL}&=2\Psi_{{\rm LL},1}\Psi_{{\rm LL},4}+2\Psi_{{\rm LL},2}\Psi_{{\rm LL},3}+2\Psi_{{\rm LL},5}\Psi_{{\rm LL},8}+2\Psi_{{\rm LL},6}\Psi_{{\rm LL},7}.
\end{aligned}$$
<!-- END t2_Pi -->

<!-- BEGIN t2_exact_equations -->
$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},1}&=i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_3+(\nabla^{\rm LL}_2\Psi_{\rm LL})_3+i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_1+m_{\rm LL}\Psi_{{\rm LL},5}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},1}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},1}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},2}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},2}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},1}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},3}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},3}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},3}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},3}-\mu_{\rm LL}\Psi_{{\rm LL},1}\\
&\quad -\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},3}\Pi^1_{\rm LL}+\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},3}\Pi^2_{\rm LL}+\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},4}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},2}&=i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_4+(\nabla^{\rm LL}_2\Psi_{\rm LL})_4+i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_2+m_{\rm LL}\Psi_{{\rm LL},6}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},1}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},2}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},2}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},2}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},4}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},4}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},4}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},4}-\mu_{\rm LL}\Psi_{{\rm LL},2}\\
&\quad +\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},4}\Pi^1_{\rm LL}+\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},4}\Pi^2_{\rm LL}+\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},3}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},3}&=i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_1-(\nabla^{\rm LL}_2\Psi_{\rm LL})_1-i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_3+m_{\rm LL}\Psi_{{\rm LL},7}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},3}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},3}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},4}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},4}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},1}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},1}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},1}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},1}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},3}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},3}-\mu_{\rm LL}\Psi_{{\rm LL},3}\\
&\quad +\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},1}\Pi^1_{\rm LL}-\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},1}\Pi^2_{\rm LL}-\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},2}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},4}&=i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_2-(\nabla^{\rm LL}_2\Psi_{\rm LL})_2-i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_4+m_{\rm LL}\Psi_{{\rm LL},8}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},3}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},3}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},4}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},4}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},2}+\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},2}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},2}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},2}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},4}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},4}-\mu_{\rm LL}\Psi_{{\rm LL},4}\\
&\quad -\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},2}\Pi^1_{\rm LL}-\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},2}\Pi^2_{\rm LL}-\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},1}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},5}&=-i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_7-(\nabla^{\rm LL}_2\Psi_{\rm LL})_7-i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_5+m_{\rm LL}\Psi_{{\rm LL},1}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},5}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},5}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},6}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},6}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},5}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},7}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},7}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},7}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},7}-\mu_{\rm LL}\Psi_{{\rm LL},5}\\
&\quad -\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},7}\Pi^1_{\rm LL}+\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},7}\Pi^2_{\rm LL}+\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},8}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},6}&=-i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_8-(\nabla^{\rm LL}_2\Psi_{\rm LL})_8-i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_6+m_{\rm LL}\Psi_{{\rm LL},2}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},5}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},6}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},6}-\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},6}\\
&\quad -\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},8}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},8}+\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},8}-\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},8}-\mu_{\rm LL}\Psi_{{\rm LL},6}\\
&\quad +\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},8}\Pi^1_{\rm LL}+\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},8}\Pi^2_{\rm LL}+\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},7}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},7}&=-i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_5+(\nabla^{\rm LL}_2\Psi_{\rm LL})_5+i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_7+m_{\rm LL}\Psi_{{\rm LL},3}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},7}\\
&\quad -\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},7}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},8}+\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},8}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},5}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},5}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},5}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},5}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},7}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},7}-\mu_{\rm LL}\Psi_{{\rm LL},7}\\
&\quad +\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},5}\Pi^1_{\rm LL}-\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},5}\Pi^2_{\rm LL}-\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},6}\Pi^3_{\rm LL},
\end{aligned}$$

$$\begin{aligned}
i\partial_0\Psi_{{\rm LL},8}&=-i\,(\nabla^{\rm LL}_1\Psi_{\rm LL})_6+(\nabla^{\rm LL}_2\Psi_{\rm LL})_6+i\,(\nabla^{\rm LL}_3\Psi_{\rm LL})_8+m_{\rm LL}\Psi_{{\rm LL},4}-\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{1}_{{\rm LL},0}\Psi_{{\rm LL},7}\\
&\quad -\tfrac{i}{2}\,g^{\rm v}_{\rm LL}A^{2}_{{\rm LL},0}\Psi_{{\rm LL},7}-g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}\Psi_{{\rm LL},8}+\tfrac{1}{2}\,g^{\rm v}_{\rm LL}A^{3}_{{\rm LL},0}\Psi_{{\rm LL},8}-\tfrac{1}{2}\,\omega^{1}_{{\rm LL},0}\Psi_{{\rm LL},6}-\tfrac{i}{2}\,\varpi^{1}_{{\rm LL},0}\Psi_{{\rm LL},6}\\
&\quad -\tfrac{i}{2}\,\omega^{2}_{{\rm LL},0}\Psi_{{\rm LL},6}+\tfrac{1}{2}\,\varpi^{2}_{{\rm LL},0}\Psi_{{\rm LL},6}+\tfrac{1}{2}\,\omega^{3}_{{\rm LL},0}\Psi_{{\rm LL},8}+\tfrac{i}{2}\,\varpi^{3}_{{\rm LL},0}\Psi_{{\rm LL},8}-\mu_{\rm LL}\Psi_{{\rm LL},8}\\
&\quad -\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},6}\Pi^1_{\rm LL}-\tfrac{i}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},6}\Pi^2_{\rm LL}-\tfrac{1}{2}\,G_{\rm LL}\Psi^*_{{\rm LL},5}\Pi^3_{\rm LL}.
\end{aligned}$$
<!-- END t2_exact_equations -->

<!-- BEGIN t2_weyl_blocks -->
$$M_{\rm LL}(\mathbf D_{\rm LL})=\begin{pmatrix}\nabla^{\rm LL}_0+\nabla^{\rm LL}_3&\nabla^{\rm LL}_1-i\,\nabla^{\rm LL}_2\\\nabla^{\rm LL}_1+i\,\nabla^{\rm LL}_2&\nabla^{\rm LL}_0-\nabla^{\rm LL}_3\end{pmatrix},\qquad M_{\rm LL}(\bar{\mathbf D}_{\rm LL})=\begin{pmatrix}\nabla^{\rm LL}_0-\nabla^{\rm LL}_3&-\nabla^{\rm LL}_1+i\,\nabla^{\rm LL}_2\\-\nabla^{\rm LL}_1-i\,\nabla^{\rm LL}_2&\nabla^{\rm LL}_0+\nabla^{\rm LL}_3\end{pmatrix}.$$
<!-- END t2_weyl_blocks -->

<!-- BEGIN t2_py_keys -->
| key | value | default | meaning |
|-------------|--------|--------|--------------------------------|
| `format` | text | required | the format of the file; must be 't2in 1' |
| `title` | text | (see text) | a free title of the problem |
| `task` | one of: evolve, spectrum, ground_state, quench | required | what to solve (see the guide) |
| `output` | text | (see text) | the base name of the output files, relative to the folder of the input file (default: the name of the input file without its extension) |
| `sites` | three integers | required | the numbers of lattice sites n_x n_y n_z along x^1, x^2, x^3; 1 means that the fields do not depend on that coordinate |
| `spacing` | positive number | required | the lattice spacing dx (the same in all directions) |
| `boundary` | three words, each: periodic, wall | periodic periodic periodic | the boundary condition of each direction: periodic (a ring) or wall (the field vanishes beyond the last site: Dirichlet) |
| `wilson` | number >= 0 | 1.0 | the coefficient r_W of the Wilson term (removes the doublers; 1 is standard) |
| `mass` | number | 0.4 | the mass m of the spinor charges |
| `mu` | number | 0.0 | the chemical potential mu |
| `g_v` | number | 0.7 | the isospin coupling g_v |
| `g_F` | number | 0.45 | the coupling g_F of the phase group |
| `potential_0` | four numbers | 0 0 0 0 | the constant potentials A^1_0 A^2_0 A^3_0 A^F_0 (time direction) |
| `potential_1` | four numbers | 0 0 0 0 | the constant potentials A^1_1 A^2_1 A^3_1 A^F_1 (direction x^1) |
| `potential_2` | four numbers | 0 0 0 0 | the constant potentials A^1_2 A^2_2 A^3_2 A^F_2 (direction x^2) |
| `potential_3` | four numbers | 0 0 0 0 | the constant potentials A^1_3 A^2_3 A^3_3 A^F_3 (direction x^3) |
| `magnetic` | two numbers | 0 0 | constant magnetic fields along x^3: F^F_12 (phase group) and F^3_12 (third isospin direction), in the Landau gauge A_2 = F (x^1 - centre); needs n_x > 1, n_y > 1 and a wall in direction 1 |
| `gauge_file` | text | (see text) | a file with potentials that depend on the site (added to the constant ones): one line per site, in the order of the sites, with 16 numbers A^1_p A^2_p A^3_p A^F_p for p = 0, 1, 2, 3 |
| `pulse_direction` | 0, 1, 2 or 3 | 1 | the index p (0, 1, 2 or 3) of the potential A_p that carries the pulse |
| `pulse_potential` | four numbers | 0 0 0 0 | the amplitudes A^1 A^2 A^3 A^F of a uniform pulse of the potential A_p (all zero: no pulse) |
| `pulse_time` | three numbers | 5 1 0 | the centre t_c, the duration t_w and the angular frequency w_p of the pulse: A_p(x^0) = amplitudes exp(-(x^0 - t_c)^2/(2 t_w^2)) cos(w_p (x^0 - t_c)) |
| `pair` | one of: none, uniform, kink, kink_antikink, step, random, file | none | the profile of the pair field d^iota: prescribed (evolve, spectrum) or the initial guess (ground_state, quench) |
| `pair_direction` | 1, 2 or 3 | 3 | the isospin direction iota of the pair field (1, 2 or 3) |
| `pair_amplitude` | number | 0.0 | the gap Delta (uniform, step) or Delta_inf (kinks) |
| `pair_phase` | number | 0.0 | the phase of the pair field, in radians |
| `pair_width` | positive number | 1.0 | the width ell_K of a kink or of a step |
| `pair_position` | numbers | (see text) | the positions along x^1 of the walls (kink: one value, default the centre; kink_antikink: two values, default 1/4 and 3/4 of the length; step: one value, default the centre) |
| `pair_seed` | integer | 1 | the seed of the random pair field (pair = random) |
| `pair_file` | text | (see text) | a file with the pair field (pair = file): one line per site, in the order of the sites, with d1_re d1_im d2_re d2_im d3_re d3_im |
| `time_step` | positive number | required for evolve, quench | the time step dt (evolve, quench) |
| `steps` | positive integer | required for evolve, quench | the number of time steps (evolve, quench) |
| `output_every` | positive integer | 0 | write the fields on the sites every this many steps (0: about 50 outputs); the time series is written at every step |
| `initial` | one of: packet, eigenmode, file | packet | the initial mode of the task evolve |
| `packet_center` | three numbers | (see text) | the centre of the Gaussian packet (x^1 x^2 x^3; default the centre of the lattice) |
| `packet_width` | positive number | 2.0 | the width of the packet (standard deviation of the density) |
| `packet_momentum` | three numbers | 0 0 0 | the momentum q of the packet |
| `packet_branch` | one of: positive, negative | positive | the sign of the energy +-eps_lat(q) of the free lattice operator (without mu) of the packet |
| `packet_spin` | one of: up, down | up | the spin along x^3 of the reference spinor |
| `packet_isospin` | one of: up, down | up | the isospin component |
| `packet_part` | one of: particle, hole | particle | particle part or hole part of the mode |
| `eigenmode_index` | integer | 0 | the number (ascending, from 0) of the eigenvector used as initial mode (initial = eigenmode) |
| `initial_file` | text | (see text) | a file with the initial mode (initial = file): 16 x (number of sites) lines 're im' in the order of the components |
| `spectrum_window` | two numbers | -1 1 | the energy window of the eigenvectors whose densities are written (spectrum) |
| `spectrum_max_states` | positive integer | 32 | at most this many eigenvectors of the window are written (spectrum), summed over each group of degenerate eigenvalues |
| `coupling` | positive number | required for ground_state, quench | the coupling G of the self-consistency condition (ground_state, quench) |
| `pair_constraint` | one of: none, line | none | line: the self-consistent iteration keeps the pair field a real multiple of exp(i pair_phase) along pair_direction (for walls of the pair field, which are unstable without it) |
| `mixing` | number in (0, 1] | 0.5 | the damping factor beta of the Anderson iteration (ground_state, quench) |
| `tolerance` | positive number | 1e-10 | the iteration stops when the largest change of d^iota is below this |
| `max_iterations` | positive integer | 400 | the largest number of iterations |
| `quench_coupling` | positive number | required for quench | the coupling after x^0 = 0 (quench) |
<!-- END t2_py_keys -->

<!-- BEGIN t2s_Kps -->
$$\begin{aligned}\mathsf K'^1_{\rm LL}&=\begin{pmatrix}0&0&1&0&0&0&0&0\\0&0&0&-1&0&0&0&0\\-1&0&0&0&0&0&0&0\\0&1&0&0&0&0&0&0\\0&0&0&0&0&0&-1&0\\0&0&0&0&0&0&0&1\\0&0&0&0&1&0&0&0\\0&0&0&0&0&-1&0&0\end{pmatrix},\\ \mathsf K'^2_{\rm LL}&=\begin{pmatrix}0&0&i&0&0&0&0&0\\0&0&0&i&0&0&0&0\\-i&0&0&0&0&0&0&0\\0&-i&0&0&0&0&0&0\\0&0&0&0&0&0&-i&0\\0&0&0&0&0&0&0&-i\\0&0&0&0&i&0&0&0\\0&0&0&0&0&i&0&0\end{pmatrix},\\ \mathsf K'^3_{\rm LL}&=\begin{pmatrix}0&0&0&-1&0&0&0&0\\0&0&-1&0&0&0&0&0\\0&1&0&0&0&0&0&0\\1&0&0&0&0&0&0&0\\0&0&0&0&0&0&0&1\\0&0&0&0&0&0&1&0\\0&0&0&0&0&-1&0&0\\0&0&0&0&-1&0&0&0\end{pmatrix}.\end{aligned}$$
<!-- END t2s_Kps -->

<!-- BEGIN t2s_Kcharge -->
$$\begin{aligned}\sqrt2\,\mathsf K^{(+1)}_{\rm LL}=\mathsf K^1_{\rm LL}-i\mathsf K^2_{\rm LL}&=\begin{pmatrix}0&0&-2&0&0&0&0&0\\0&0&0&0&0&0&0&0\\2&0&0&0&0&0&0&0\\0&0&0&0&0&0&0&0\\0&0&0&0&0&0&-2&0\\0&0&0&0&0&0&0&0\\0&0&0&0&2&0&0&0\\0&0&0&0&0&0&0&0\end{pmatrix},\\ \sqrt2\,\mathsf K^{(-1)}_{\rm LL}=\mathsf K^1_{\rm LL}+i\mathsf K^2_{\rm LL}&=\begin{pmatrix}0&0&0&0&0&0&0&0\\0&0&0&2&0&0&0&0\\0&0&0&0&0&0&0&0\\0&-2&0&0&0&0&0&0\\0&0&0&0&0&0&0&0\\0&0&0&0&0&0&0&2\\0&0&0&0&0&0&0&0\\0&0&0&0&0&-2&0&0\end{pmatrix},\\ \mathsf K^{(0)}_{\rm LL}=\mathsf K^3_{\rm LL}&=\begin{pmatrix}0&0&0&1&0&0&0&0\\0&0&1&0&0&0&0&0\\0&-1&0&0&0&0&0&0\\-1&0&0&0&0&0&0&0\\0&0&0&0&0&0&0&1\\0&0&0&0&0&0&1&0\\0&0&0&0&0&-1&0&0\\0&0&0&0&-1&0&0&0\end{pmatrix}.\end{aligned}$$
<!-- END t2s_Kcharge -->

<!-- BEGIN t2s_h4 -->
$$\sum_j\alpha^j_{\rm LL}q^j_{\rm LL}+\beta_{\rm LL}m_{\rm LL}=\begin{pmatrix}-q^3_{\rm LL}&-q^1_{\rm LL}+i\,q^2_{\rm LL}&m_{\rm LL}&0\\-q^1_{\rm LL}-i\,q^2_{\rm LL}&q^3_{\rm LL}&0&m_{\rm LL}\\m_{\rm LL}&0&q^3_{\rm LL}&q^1_{\rm LL}-i\,q^2_{\rm LL}\\0&m_{\rm LL}&q^1_{\rm LL}+i\,q^2_{\rm LL}&-q^3_{\rm LL}\end{pmatrix}.$$
<!-- END t2s_h4 -->

<!-- BEGIN t2s_Pi_charge -->
$$\begin{aligned}
\sqrt2\,\Pi^{(+1)}_{\rm LL}&=-4\Psi_{{\rm LL},1}\Psi_{{\rm LL},3}-4\Psi_{{\rm LL},5}\Psi_{{\rm LL},7},\\
\sqrt2\,\Pi^{(-1)}_{\rm LL}&=4\Psi_{{\rm LL},2}\Psi_{{\rm LL},4}+4\Psi_{{\rm LL},6}\Psi_{{\rm LL},8},\\
\Pi^{(0)}_{\rm LL}&=2\Psi_{{\rm LL},1}\Psi_{{\rm LL},4}+2\Psi_{{\rm LL},2}\Psi_{{\rm LL},3}+2\Psi_{{\rm LL},5}\Psi_{{\rm LL},8}+2\Psi_{{\rm LL},6}\Psi_{{\rm LL},7}.
\end{aligned}$$
<!-- END t2s_Pi_charge -->

<!-- BEGIN t2s_Avec -->
$$\mathcal A^{\rm v}_{{\rm LL},p}=\begin{pmatrix}0&-A^3_{{\rm LL},p}&A^2_{{\rm LL},p}\\A^3_{{\rm LL},p}&0&-A^1_{{\rm LL},p}\\-A^2_{{\rm LL},p}&A^1_{{\rm LL},p}&0\end{pmatrix}$$
<!-- END t2s_Avec -->

<!-- BEGIN t2s_nablaB -->
$$\begin{aligned}(\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL})^1&=\partial_pd^1_{\rm LL}-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}d^1_{\rm LL}+g^{\rm v}_{\rm LL}A^2_{{\rm LL},p}d^3_{\rm LL}-g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}d^2_{\rm LL}\\
(\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL})^2&=\partial_pd^2_{\rm LL}-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}d^2_{\rm LL}-g^{\rm v}_{\rm LL}A^1_{{\rm LL},p}d^3_{\rm LL}+g^{\rm v}_{\rm LL}A^3_{{\rm LL},p}d^1_{\rm LL}\\
(\nabla^{\rm B}_{{\rm LL},p}\vec d_{\rm LL})^3&=\partial_pd^3_{\rm LL}-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},p}d^3_{\rm LL}+g^{\rm v}_{\rm LL}A^1_{{\rm LL},p}d^2_{\rm LL}-g^{\rm v}_{\rm LL}A^2_{{\rm LL},p}d^1_{\rm LL}\end{aligned}$$
<!-- END t2s_nablaB -->

<!-- BEGIN t2s_fieldeq -->
$$\begin{aligned}&Z^{\rm t}_{\rm LL}\big[\partial_0(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^1-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^1\\
&\quad +g^{\rm v}_{\rm LL}A^2_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^3-g^{\rm v}_{\rm LL}A^3_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^2\big]\\
&\quad-Z^{\rm s}_{\rm LL}\sum_j\big[\partial_j(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^1-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^1\\
&\quad +g^{\rm v}_{\rm LL}A^2_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^3-g^{\rm v}_{\rm LL}A^3_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^2\big]\\
&\quad+\frac{\partial\Omega_{\rm LL}}{\partial(d^1_{\rm LL})^*}=0\\[4pt]
&Z^{\rm t}_{\rm LL}\big[\partial_0(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^2-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^2\\
&\quad -g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^3+g^{\rm v}_{\rm LL}A^3_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^1\big]\\
&\quad-Z^{\rm s}_{\rm LL}\sum_j\big[\partial_j(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^2-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^2\\
&\quad -g^{\rm v}_{\rm LL}A^1_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^3+g^{\rm v}_{\rm LL}A^3_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^1\big]\\
&\quad+\frac{\partial\Omega_{\rm LL}}{\partial(d^2_{\rm LL})^*}=0\\[4pt]
&Z^{\rm t}_{\rm LL}\big[\partial_0(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^3-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^3\\
&\quad +g^{\rm v}_{\rm LL}A^1_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^2-g^{\rm v}_{\rm LL}A^2_{{\rm LL},0}(\nabla^{\rm B}_{{\rm LL},0}\vec d_{\rm LL})^1\big]\\
&\quad-Z^{\rm s}_{\rm LL}\sum_j\big[\partial_j(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^3-2i\,g^{\rm F}_{\rm LL}A^{\rm F}_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^3\\
&\quad +g^{\rm v}_{\rm LL}A^1_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^2-g^{\rm v}_{\rm LL}A^2_{{\rm LL},j}(\nabla^{\rm B}_{{\rm LL},j}\vec d_{\rm LL})^1\big]\\
&\quad+\frac{\partial\Omega_{\rm LL}}{\partial(d^3_{\rm LL})^*}=0\end{aligned}$$
<!-- END t2s_fieldeq -->

<!-- BEGIN t2s_Dpair -->
$$\mathsf D_{\rm LL}=\begin{pmatrix}d^1_{\rm LL}-i\,d^2_{\rm LL}&-d^3_{\rm LL}\\-d^3_{\rm LL}&-d^1_{\rm LL}-i\,d^2_{\rm LL}\end{pmatrix}$$
<!-- END t2s_Dpair -->

<!-- BEGIN nchecks_t2_analytic -->
10
<!-- END nchecks_t2_analytic -->

<!-- BEGIN nchecks_t2_fields -->
16
<!-- END nchecks_t2_fields -->

<!-- BEGIN nchecks_t2_py_examples -->
10
<!-- END nchecks_t2_py_examples -->

<!-- BEGIN nchecks_t2_py_selftest -->
4
<!-- END nchecks_t2_py_selftest -->

<!-- BEGIN nchecks_t2_rs_examples -->
11
<!-- END nchecks_t2_rs_examples -->

<!-- BEGIN nchecks_t2_rs_selftest -->
5
<!-- END nchecks_t2_rs_selftest -->

<!-- BEGIN nchecks_t2_sfbs -->
14
<!-- END nchecks_t2_sfbs -->

<!-- BEGIN n_t2_rust_lines -->
11498
<!-- END n_t2_rust_lines -->

<!-- BEGIN n_t2_rust_files -->
17
<!-- END n_t2_rust_files -->
