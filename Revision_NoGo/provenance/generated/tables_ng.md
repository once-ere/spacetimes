<!-- BEGIN nchecks_octonions -->
32
<!-- END nchecks_octonions -->

<!-- BEGIN nchecks_triality -->
15
<!-- END nchecks_triality -->

<!-- BEGIN nchecks_kaluza -->
20
<!-- END nchecks_kaluza -->

<!-- BEGIN nchecks_superalgebra -->
8
<!-- END nchecks_superalgebra -->

<!-- BEGIN nchecks_pairs4d -->
14
<!-- END nchecks_pairs4d -->

<!-- BEGIN nchecks_renorm -->
5
<!-- END nchecks_renorm -->

<!-- BEGIN nchecks_symbol -->
6
<!-- END nchecks_symbol -->

<!-- BEGIN nchecks_self_contained -->
1
<!-- END nchecks_self_contained -->

<!-- BEGIN checks_table -->
| report (data/) | command | checks passed |
|---|---|---|
| `ng_octonions_checks.txt` | `nogo octonions` | 32 of 32 |
| `ng_triality_checks.txt` | `nogo triality` | 15 of 15 |
| `ng_kaluza_checks.txt` | `nogo kaluza` | 20 of 20 |
| `ng_superalgebra_checks.txt` | `nogo superalgebra` | 8 of 8 |
| `ng_pairs4d_checks.txt` | `nogo pairs4d` | 14 of 14 |
| `ng_renorm_checks.txt` | `nogo renorm` | 5 of 5 |
| `ng_symbol_checks.txt` | `check_symbols.py` | 6 of 6 |
| `ng_self_contained_checks.txt` | `check_self_contained.py` | 1 of 1 |
<!-- END checks_table -->

<!-- BEGIN checks_rows_tex -->
O-1 [O\_NG] & e0 = 1 is a two-sided unit & PASS\\
O-2 [O\_NG] & x conj(x) = N(x) e0 with N = diag(norm); signature as stated  -- N = [1, 1, 1, 1, 1, 1, 1, 1] & PASS\\
O-3 [O\_NG] & composition N(xy) = N(x) N(y), verified in its fully polarised form (linear in each of four arguments) on all 4096 quadruples of basis elements & PASS\\
O-4 [O\_NG] & alternative: the associator (x,y,z) = (xy)z - x(yz) is totally antisymmetric (all 512 basis triples) & PASS\\
O-5 [O\_NG] & NOT associative: the associator is nonzero on exactly 168 of the 512 ordered basis triples (the ordered triples of distinct imaginary units that do not lie in one quaternionic subalgebra); for example (e1, e2, e4) = (e1 e2) e4 - e1 (e2 e4) = e7 - (-e7) = 2 e7  -- nonzero triples = 168, (e1,e2,e4) = [0, 0, 0, 0, 0, 0, 0, 2] & PASS\\
O-6 [O\_NG] & Moufang identities z(x(zy)) = ((zx)z)y, x(z(yz)) = ((xz)y)z, (zx)(yz) = (z(xy))z on 2000 random integer triples (exact) & PASS\\
O-7 [O\_NG] & Jacobiator of the commutator Jac(x,y,z) = [[x,y],z] + [[y,z],x] + [[z,x],y] equals 6 (x,y,z) (all 512 basis triples) & PASS\\
O-8 [O\_NG] & the Jacobiators of all triples of imaginary basis units span all of Im (rank 7): every linear representation of the commutator algebra (Im, [,]) by matrices vanishes identically  -- rank = 7 & PASS\\
O-9 [O\_NG] & Malcev identity Jac(x, y, [x,z]) = [Jac(x,y,z), x] on 2000 random imaginary integer triples (exact) & PASS\\
O-10 [O\_NG] & derivation algebra Der = \{ delta : delta(xy) = (delta x)y + x(delta y) \} has dimension 14 (exact null space), delta e0 = 0, delta in so(N)  -- dim = 14 & PASS\\
O-11 [O\_NG] & Killing form of Der(O) is negative definite: Der(O) = compact g2 (the Lie algebra of G2 = Aut(O))  -- signature (+0, -14) & PASS\\
O-12 [O\_NG] & the left multiplications L\_e1..L\_e7 generate (under commutators) a Lie algebra of dimension 28 inside so(N): all of so(8) for O, all of so(4,4) for Os  -- dim = 28 & PASS\\
O-13 [O\_NG] & Z2 grading of the algebra = H + H l: H H in H, H (H l) in H l, (H l) H in H l, (H l)(H l) in H; H = span(e0..e3) is a quaternion subalgebra & PASS\\
O-1 [Os\_NG] & e0 = 1 is a two-sided unit & PASS\\
O-2 [Os\_NG] & x conj(x) = N(x) e0 with N = diag(norm); signature as stated  -- N = [1, 1, 1, 1, -1, -1, -1, -1] & PASS\\
O-3 [Os\_NG] & composition N(xy) = N(x) N(y), verified in its fully polarised form (linear in each of four arguments) on all 4096 quadruples of basis elements & PASS\\
O-4 [Os\_NG] & alternative: the associator (x,y,z) = (xy)z - x(yz) is totally antisymmetric (all 512 basis triples) & PASS\\
O-5 [Os\_NG] & NOT associative: the associator is nonzero on exactly 168 of the 512 ordered basis triples (the ordered triples of distinct imaginary units that do not lie in one quaternionic subalgebra); for example (e1, e2, e4) = (e1 e2) e4 - e1 (e2 e4) = e7 - (-e7) = 2 e7  -- nonzero triples = 168, (e1,e2,e4) = [0, 0, 0, 0, 0, 0, 0, 2] & PASS\\
O-6 [Os\_NG] & Moufang identities z(x(zy)) = ((zx)z)y, x(z(yz)) = ((xz)y)z, (zx)(yz) = (z(xy))z on 2000 random integer triples (exact) & PASS\\
O-7 [Os\_NG] & Jacobiator of the commutator Jac(x,y,z) = [[x,y],z] + [[y,z],x] + [[z,x],y] equals 6 (x,y,z) (all 512 basis triples) & PASS\\
O-8 [Os\_NG] & the Jacobiators of all triples of imaginary basis units span all of Im (rank 7): every linear representation of the commutator algebra (Im, [,]) by matrices vanishes identically  -- rank = 7 & PASS\\
O-9 [Os\_NG] & Malcev identity Jac(x, y, [x,z]) = [Jac(x,y,z), x] on 2000 random imaginary integer triples (exact) & PASS\\
O-10 [Os\_NG] & derivation algebra Der = \{ delta : delta(xy) = (delta x)y + x(delta y) \} has dimension 14 (exact null space), delta e0 = 0, delta in so(N)  -- dim = 14 & PASS\\
O-11 [Os\_NG] & Killing form of Der(Os) has signature (8+, 6-): Der(Os) = split real form of g2  -- signature (+8, -6) & PASS\\
O-12 [Os\_NG] & the left multiplications L\_e1..L\_e7 generate (under commutators) a Lie algebra of dimension 28 inside so(N): all of so(8) for O, all of so(4,4) for Os  -- dim = 28 & PASS\\
O-13 [Os\_NG] & Z2 grading of the algebra = H + H l: H H in H, H (H l) in H l, (H l) H in H l, (H l)(H l) in H; H = span(e0..e3) is a quaternion subalgebra & PASS\\
O-14a & the octonionic matrices Gamma71oct[A] (time $<$-$>$ 1, axes 1,2,3 $<$-$>$ i,j,k, axes 0,5,6,7 $<$-$>$ l,il,jl,kl) satisfy the Clifford relations of eta1671 & PASS\\
O-14b & the octonionic matrices Gamma44oct[A] = [[0, L\_conj(e\_A)], [L\_e\_A, 0]] satisfy the Clifford relations of eta1644 (the split norm N of Os) & PASS\\
O-14c & the 7+1 metric is the real part of the octonion square: eta1671(x,x) = Re(x\^{}2), x = x4 + x1 i + x2 j + x3 k + x0 l + x5 il + x6 jl + x7 kl & PASS\\
O-14d & the 4+4 metric is the split-octonion norm: eta1644(x,x) = N(x) = x conj(x), x = sum\_A xA e\_A & PASS\\
O-15 [1671] & an invertible intertwiner I1671 with I gamma1671[A] = Gamma\_oct[A] I exists (Clifford-group average); the 256 monomials of gamma1671 are independent, so the representation is irreducible and I is unique up to a factor  -- rank I = 16, rank of the 256 monomials = 256 & PASS\\
O-15 [1644] & an invertible intertwiner I1644 with I gamma1644[A] = Gamma\_oct[A] I exists (Clifford-group average); the 256 monomials of gamma1644 are independent, so the representation is irreducible and I is unique up to a factor  -- rank I = 16, rank of the 256 monomials = 256 & PASS\\
T-1 [O\_NG] & local triality (LT): for each of the 28 basis elements m1 of so(N) the linear system for (m2, m3) in so(N) x so(N) is consistent with a unique solution (exact) & PASS\\
T-2 [O\_NG] & T2: m1 -$>$ m2 and T3: m1 -$>$ m3 are invertible Lie-algebra automorphisms of so(N): they preserve the commutators of all 378 pairs of basis elements & PASS\\
T-3 [O\_NG] & T2 and T3 are involutions: T2\^{}2 = T3\^{}2 = 1 & PASS\\
T-4 [O\_NG] & T2 and T3 are OUTER: the simple rotation r01 has det(lambda - r01) = lambda\^{}6 (lambda\^{}2 + 1) (vector representation), its images have det(lambda - T2 r01) = det(lambda - T3 r01) = (lambda\^{}2 + 1/4)\^{}4 (half-spinor spectrum); conjugation by O(N) preserves characteristic polynomials  -- r01: 1 lambda\^{}8 + 1 lambda\^{}6; T2 r01: 1 lambda\^{}8 + 1 lambda\^{}6 + 3/8 lambda\^{}4 + 1/16 lambda\^{}2 + 1/256; T3 r01: 1 lambda\^{}8 + 1 lambda\^{}6 + 3/8 lambda\^{}4 + 1/16 lambda\^{}2 + 1/256 & PASS\\
T-5 [O\_NG] & vartheta = T2 o Ad(kappa) (kappa: x -$>$ conj(x)) has order 3 (vartheta\^{}3 = 1, vartheta != 1), Ad(kappa)\^{}2 = 1 and Ad(kappa) vartheta Ad(kappa) = vartheta\^{}-1: the automorphisms generate Sym(3) (the permutations of three objects), the outer automorphism group of so(8, C) & PASS\\
T-1 [Os\_NG] & local triality (LT): for each of the 28 basis elements m1 of so(N) the linear system for (m2, m3) in so(N) x so(N) is consistent with a unique solution (exact) & PASS\\
T-2 [Os\_NG] & T2: m1 -$>$ m2 and T3: m1 -$>$ m3 are invertible Lie-algebra automorphisms of so(N): they preserve the commutators of all 378 pairs of basis elements & PASS\\
T-3 [Os\_NG] & T2 and T3 are involutions: T2\^{}2 = T3\^{}2 = 1 & PASS\\
T-4 [Os\_NG] & T2 and T3 are OUTER: the simple rotation r01 has det(lambda - r01) = lambda\^{}6 (lambda\^{}2 + 1) (vector representation), its images have det(lambda - T2 r01) = det(lambda - T3 r01) = (lambda\^{}2 + 1/4)\^{}4 (half-spinor spectrum); conjugation by O(N) preserves characteristic polynomials  -- r01: 1 lambda\^{}8 + 1 lambda\^{}6; T2 r01: 1 lambda\^{}8 + 1 lambda\^{}6 + 3/8 lambda\^{}4 + 1/16 lambda\^{}2 + 1/256; T3 r01: 1 lambda\^{}8 + 1 lambda\^{}6 + 3/8 lambda\^{}4 + 1/16 lambda\^{}2 + 1/256 & PASS\\
T-5 [Os\_NG] & vartheta = T2 o Ad(kappa) (kappa: x -$>$ conj(x)) has order 3 (vartheta\^{}3 = 1, vartheta != 1), Ad(kappa)\^{}2 = 1 and Ad(kappa) vartheta Ad(kappa) = vartheta\^{}-1: the automorphisms generate Sym(3) (the permutations of three objects), the outer automorphism group of so(8, C) & PASS\\
T-6 [Os\_NG] & the triality images have exactly the spectrum of the half-spinor generators of the project: Sigma\^{}01\_1644 restricted to S+ and to S- has det(lambda - Sigma\^{}01\_1644) = (lambda\^{}2 + 1/4)\^{}4  -- S+: 1 lambda\^{}8 + 1 lambda\^{}6 + 3/8 lambda\^{}4 + 1/16 lambda\^{}2 + 1/256; S-: 1 lambda\^{}8 + 1 lambda\^{}6 + 3/8 lambda\^{}4 + 1/16 lambda\^{}2 + 1/256 & PASS\\
T-7a & so(7,1) has no local triality: for the boost m1 with m1 e0 = e1, m1 e1 = e0 and m1 = 0 on e2..e7 (an element of so(7,1) for the metric Re(x\^{}2)) the system (LT) is INCONSISTENT even with m2, m3 arbitrary real 8-by-8 matrices (exact elimination) & PASS\\
T-7b & commutant of so(7,1) on the real 16-dimensional spinor space is span\{1, omega1671\} with omega1671\^{}2 = -1: the spinor representation is of COMPLEX type (no real 8-dimensional half-spinors; S+ and S- are complex conjugates)  -- dim $<$= 2 (rank modulo a large prime), 1 and omega1671 commute & PASS\\
T-7c & commutant of so(4,4) on the real 16-dimensional spinor space is span\{1, omega1644\} with omega1644\^{}2 = +1: the space is S+ + S- with two REAL 8-dimensional half-spinor representations (Majorana-Weyl)  -- dim $<$= 2 (rank modulo a large prime) & PASS\\
T-7d & commutant of so(7,1) in its vector representation is the real numbers (dimension 1): the vector representation is of REAL type; an automorphism preserves the type, so no automorphism of so(7,1) maps the vector representation onto a half-spinor representation -- there is no triality for 7+1  -- dim $<$= 1 (rank modulo a large prime) & PASS\\
K-1 [7+1] & 28 = 6 + 6 + 16: the four-dimensional Lorentz generators Sigma\^{}(AB) (A, B in \{1,2,3,4\}) commute exactly with the internal generators Sigma\^{}(ab) (a, b in \{0,5,6,7\}); every mixed generator Sigma\^{}(Aa) (A in \{1,2,3,4\}, a in \{0,5,6,7\}) fails to commute with some Sigma\^{}(BC) (B, C in \{1,2,3,4\}) & PASS\\
K-2 [7+1] & internal algebra COMPACT so(4): trace form negative definite, (2 Sigma\^{}(ab))\^{}2 = -1 for all six internal pairs (imaginary spectrum, unitary on finite multiplets)  -- internal (+0, -6), squares [-1, -1, -1, -1, -1, -1]; four-dimensional Lorentz (+3, -3) & PASS\\
K-3 [7+1] & the 16 components are the tensor product of a 4-component world factor and a 4-component fibre factor: the 16 monomials of gamma\^{}1..gamma\^{}4 and the 16 monomials of omega4 gamma\^{}a (a in INT) are each linearly independent and the two sets commute; omega4 = gamma\^{}1 gamma\^{}2 gamma\^{}3 gamma\^{}4 and omega\_int = gamma\^{}0 gamma\^{}5 gamma\^{}6 gamma\^{}7 commute; sum of the squares of the six 2 Sigma\^{}(ab) as stated  -- ranks 16 and 16; omega4\^{}2 = -1, omega\_int\^{}2 = +1 & PASS\\
K-4a [7+1] & the two families coincide: every C+ gamma\^{}K equals +-C- gamma\^{}K' for exactly one K' (C- = +-C+ omega and omega gamma\^{}K = +-gamma\^{}(complement of K)); the rows with n4 = 4 of the table repeat the rows with n4 = 0 of the other family & PASS\\
K-4 [7+1] & one-field pairs psi\^{}T C(+-) gamma\^{}K psi (all 512 matrices): C+ and C- are symmetric (no one-field internal-singlet spin-zero pair); the nonzero, four-dimensionally scalar, s-wave, isotropic channels are EXACTLY C- gamma\^{}a (internal vector, 4) and C+ gamma\^{}a gamma\^{}b (internal 2-form, 6); the spin-one channels C- gamma\^{}j (j = 1,2,3) are s-wave but anisotropic  -- 10 channels: C+g[0, 5] C+g[0, 6] C+g[5, 6] C+g[0, 7] C+g[5, 7] C+g[6, 7] C-g[0] C-g[5] C-g[6] C-g[7] & PASS\\
K-10 [7+1] & the stabiliser of an internal unit vector of square eta = -1 (along the axis x0 on 7+1, along x5 on 4+4) in the Lorentz algebra has dimension 21 (so(6,1) on 7+1, so(4,3) on 4+4) and contains 12 generators that mix M(3,1) with the internal directions orthogonal to the vector  -- dim = 21, mixing = 12 & PASS\\
K-1 [4+4] & 28 = 6 + 6 + 16: the four-dimensional Lorentz generators Sigma\^{}(AB) (A, B in \{1,2,3,4\}) commute exactly with the internal generators Sigma\^{}(ab) (a, b in \{0,5,6,7\}); every mixed generator Sigma\^{}(Aa) (A in \{1,2,3,4\}, a in \{0,5,6,7\}) fails to commute with some Sigma\^{}(BC) (B, C in \{1,2,3,4\}) & PASS\\
K-2 [4+4] & internal algebra NON-COMPACT so(1,3): trace form of signature (3+, 3-); (2 Sigma\^{}(0a))\^{}2 = +1 for a = 5, 6, 7 (REAL spectrum +-1/2: no positive-definite invariant inner product on the 16 components)  -- internal (+3, -3), squares [1, 1, 1, -1, -1, -1]; four-dimensional Lorentz (+3, -3) & PASS\\
K-3 [4+4] & the 16 components are the tensor product of a 4-component world factor and a 4-component fibre factor: the 16 monomials of gamma\^{}1..gamma\^{}4 and the 16 monomials of omega4 gamma\^{}a (a in INT) are each linearly independent and the two sets commute; omega4 = gamma\^{}1 gamma\^{}2 gamma\^{}3 gamma\^{}4 and omega\_int = gamma\^{}0 gamma\^{}5 gamma\^{}6 gamma\^{}7 commute; sum of the squares of the six 2 Sigma\^{}(ab) as stated  -- ranks 16 and 16; omega4\^{}2 = -1, omega\_int\^{}2 = -1 & PASS\\
K-4a [4+4] & the two families coincide: every C+ gamma\^{}K equals +-C- gamma\^{}K' for exactly one K' (C- = +-C+ omega and omega gamma\^{}K = +-gamma\^{}(complement of K)); the rows with n4 = 4 of the table repeat the rows with n4 = 0 of the other family & PASS\\
K-4 [4+4] & one-field pairs psi\^{}T C(+-) gamma\^{}K psi (all 512 matrices): C+ and C- are symmetric (no one-field internal-singlet spin-zero pair); the nonzero, four-dimensionally scalar, s-wave, isotropic channels are EXACTLY C- gamma\^{}a (internal vector, 4) and C+ gamma\^{}a gamma\^{}b (internal 2-form, 6); the spin-one channels C- gamma\^{}j (j = 1,2,3) are s-wave but anisotropic  -- 10 channels: C+g[0, 5] C+g[0, 6] C+g[5, 6] C+g[0, 7] C+g[5, 7] C+g[6, 7] C-g[0] C-g[5] C-g[6] C-g[7] & PASS\\
K-10 [4+4] & the stabiliser of an internal unit vector of square eta = -1 (along the axis x0 on 7+1, along x5 on 4+4) in the Lorentz algebra has dimension 21 (so(6,1) on 7+1, so(4,3) on 4+4) and contains 12 generators that mix M(3,1) with the internal directions orthogonal to the vector  -- dim = 21, mixing = 12 & PASS\\
K-5a & G2 locking on 7+1: inside so(3) (rotations of x1, x2, x3 = Im H) + so(4) (rotations of x0, x5, x6, x7 = H l) the stabiliser of the associative 3-form phi(x,y,z) = $<$x, yz$>$ has dimension 6 (= so(4)\_H in G2), and so has the stabiliser of its 6 mixed components (one unit from e1, e2, e3 and two from e4, ..., e7) alone  -- dims 6 and 6; nonzero mixed components = 6 & PASS\\
K-5b & the locking is complete: the stabiliser projects ONTO the space rotations so(3) (rank 3), contains NO pure space rotation (dimension 0), and its kernel is an internal su(2) (dimension 3): every space rotation must be accompanied by an internal rotation  -- projection rank 3, pure space 0, pure internal 3 & PASS\\
K-5c & the same subalgebra from Der(O) = g2: the elements of Der(O) that preserve Im H and H l form a 6-dimensional algebra (so(4)\_H = su(2) + su(2))  -- dim = 6 & PASS\\
K-6 & 4+4 exchange: left multiplication by l (N(l) = -1) is an anti-isometry (L\^{}T N L = -N, L\^{}2 = 1) that maps M(3,1) = span(i, j, k, l) onto K(1,3) = span(1, il, jl, kl) and back; Ad(L\_l) maps so(3,1) of M onto so(1,3) of K and back (no such exchange exists on 7+1, where the two algebras are not isomorphic: check K-2) & PASS\\
K-7 [7+1] & 7+1: the Dirac conjugation matrix A8 = gamma\^{}4 satisfies A8 Sigma\^{}(AB) A8\^{}-1 = -Sigma\^{}(AB)T for all 28 generators, and the 8D-covariant charge density psi\^{}dagger A8 gamma\^{}4 psi = psi\^{}dagger psi is POSITIVE & PASS\\
K-7 [4+4] & 4+4: the Dirac conjugation matrix A8 = gamma\^{}4 gamma\^{}5 gamma\^{}6 gamma\^{}7 satisfies A8 Sigma\^{}(AB) A8\^{}-1 = -Sigma\^{}(AB)T for all 28 generators, and the 8D-covariant density matrix A8 gamma\^{}4 = gamma\^{}5 gamma\^{}6 gamma\^{}7 is traceless with square 1: the density is INDEFINITE (negative-norm states) & PASS\\
K-8 & O'Raifeartaigh: a rotation of the 8D Lorentz group in the (x1, x0) plane (cos = 3/5, sin = 4/5) leaves the 8D mass\^{}2 = 16 unchanged but changes the four-dimensional mass\^{}2 from 16 to 544/25 (exact): a finite-dimensional Lie group containing the 4D Poincare group splits 4D masses because the 4D mass spectrum of one 8D particle is continuous (the theorem's isolated-eigenvalue hypothesis fails)  -- m4\^{}2: 16 -$>$ 544/25, m8\^{}2 after = 16 & PASS\\
K-9 & Kaluza-Klein modes on a torus of radius R ((mR)\^{}2 = 1, $|$n\_a$|$ $<$= Nmax): 7+1 has m\_eff\^{}2 = m\^{}2 + $|$n$|$\^{}2/R\^{}2 $>$ 0 (no tachyon); 4+4 has m\_eff\^{}2 = m\^{}2 + (n0\^{}2 - n5\^{}2 - n6\^{}2 - n7\^{}2)/R\^{}2 and the number of tachyonic modes grows like Nmax\^{}4 (a fixed fraction of all modes)  -- 4+4 tachyon fraction at Nmax = 12: 0.8660 & PASS\\
S-1 [7+1] & C+ gamma\^{}A is symmetric for all A (a symmetric anticommutator \{Q, Q\} = Y\^{}C P\_C with Y\^{}C = 2 C+ gamma\^{}C exists); omega commutes with all Sigma\^{}(AB) and omega\^{}2 = -1: the R-charge Rch = omega generates a COMPACT U(1)\_R & PASS\\
S-2a [7+1] & graded Jacobi identity of the super-Poincare algebra J (28) + P (8) + Q (16) + Rch (1) on all 53\^{}3 triples; Rch commutes with J and P (an internal symmetry) but rotates the supercharges, and \{Q, Q\} produces the translations: spacetime and internal symmetry are linked through the odd generators (Haag-Lopuszanski-Sohnius loophole)  -- dimension 53, 148877 triples checked, 0 violations; dyadic true & PASS\\
S-2b [7+1] & the 7+1 superalgebra EXTENDED by the 128 tensorial charges Z\_K ($|$K$|$ = 0, 4, 5, 8; with P the 136 symmetric matrices C+ gamma\^{}K): graded Jacobi identity on all triples (181 generators); the Z\_K commute with P and Q but transform as Lorentz tensors (a McGlinn-type counterexample with an abelian, non-semisimple 'internal' algebra)  -- dimension 181, 5929741 triples checked, 0 violations; dyadic true & PASS\\
S-3 [7+1] & of the 128 tensorial charges, 56 (rank 5) are Rch-neutral and 72 (ranks 0, 4, 8) are rotated by Rch; the 126 charges of rank 4 and 5 are Lorentz tensors (commute with all P\_C but not with J); only Z\_() and Z\_(01234567) are Lorentz scalars  -- neutral 56, charged 72, Lorentz tensors 126 & PASS\\
S-4 [7+1] & positivity: Y\^{}4 = 2 C+ gamma\^{}4 = 2 and the seven matrices C+ gamma\^{}A (A != 4) anticommute and square to 1, so sum\_C Y\^{}C P\_C has eigenvalues 2 (P\_4 +- $|$P$|$), $|$P$|$\^{}2 = sum\_(A != 4) P\_A\^{}2: positive semi-definite exactly when P lies in the closed forward cone & PASS\\
S-1 [4+4] & C+ gamma\^{}A is symmetric for all A; omega1644 commutes with all Sigma\^{}(AB) and omega1644\^{}2 = +1: the R-charge Rch = omega1644 generates a NON-COMPACT SO(1,1)\_R (Q+ -$>$ e\^{}(-varphi) Q+, Q- -$>$ e\^{}(+varphi) Q-) & PASS\\
S-2a [4+4] & graded Jacobi identity of the super-Poincare algebra J (28) + P (8) + Q (16) + Rch (1) on all 53\^{}3 triples; Rch commutes with J and P (an internal symmetry) but rotates the supercharges, and \{Q, Q\} produces the translations: spacetime and internal symmetry are linked through the odd generators (Haag-Lopuszanski-Sohnius loophole)  -- dimension 53, 148877 triples checked, 0 violations; dyadic true & PASS\\
S-4 [4+4] & no positivity: every Y\^{}C = 2 C+ gamma\^{}C is traceless and nonzero, so sum\_C Y\^{}C P\_C has a NEGATIVE eigenvalue for every P != 0 -- there is no unitary representation with Hermitian Q and P != 0; moreover omega1644 Y\^{}C = -Y\^{}C omega1644: \{Q, Q\} pairs Q+ with Q- only & PASS\\
P-1 [7+1] & alpha\^{}1, alpha\^{}2, alpha\^{}3, beta are Hermitian, square to 1 and anticommute pairwise: the spectrum of h(q) is +-sqrt(q\^{}2 + m\^{}2) (8 times each) & PASS\\
P-1 [4+4] & alpha\^{}1, alpha\^{}2, alpha\^{}3, beta are Hermitian, square to 1 and anticommute pairwise: the spectrum of h(q) is +-sqrt(q\^{}2 + m\^{}2) (8 times each) & PASS\\
P-2 [7+1] & isotropic channels (internal vector C- gamma\^{}a, internal 2-form C+ gamma\^{}a gamma\^{}b, doublet C+): the 32 BdG eigenvalues are +-E\^{}+, +-E\^{}- (8 times each), E\^{}(+-) = sqrt((eps +- mu)\^{}2 + $|$Delta$|$\^{}2), for every momentum direction (40 random samples each)  -- max deviation 4.44e-14 & PASS\\
P-2 [4+4] & isotropic channels (internal vector C- gamma\^{}a, internal 2-form C+ gamma\^{}a gamma\^{}b, doublet C+): the 32 BdG eigenvalues are +-E\^{}+, +-E\^{}- (8 times each), E\^{}(+-) = sqrt((eps +- mu)\^{}2 + $|$Delta$|$\^{}2), for every momentum direction (40 random samples each)  -- max deviation 2.49e-14 & PASS\\
P-3a [7+1] & non-unitary internal-vector states Delta\^{}0 = $|$Delta$|$ cos(ups), Delta\^{}5 = i $|$Delta$|$ sin(ups): the BdG spectrum is +-sqrt((eps +- mu)\^{}2 + $|$Delta$|$\^{}2 (1 +- $|$sin 2 ups$|$)) (4 times each) and independent of the momentum direction; the chiral state (ups = pi/4) has gaps sqrt(2)$|$Delta$|$ and 0  -- max deviation 4.44e-14, isotropy 3.11e-14 & PASS\\
P-3b [4+4] & Wick map: for arbitrary complex pair fields the 4+4 BdG spectrum equals the 7+1 spectrum after Delta\_0 -$>$ i Delta\_0 (vector channel) and Delta\_0a -$>$ i Delta\_0a (2-form channel): the positive-metric 4+4 superfluid is the 7+1 superfluid with the internal axis x0 Wick-rotated (40 random samples each)  -- max deviations 2.80e-14, 2.22e-14 & PASS\\
P-4 & spin-one channel C- gamma\^{}1 (both spacetimes): E\_(+-)\^{}2 = eps\^{}2 + mu\^{}2 + $|$Delta$|$\^{}2 +- 2 sqrt(mu\^{}2 eps\^{}2 + $|$Delta$|$\^{}2 q\_1\^{}2) (8 times each) -- ANISOTROPIC: it differs from the isotropic formula unless q\_1 = 0  -- max deviation 4.44e-14; anisotropy 0.249 & PASS\\
P-5 & normalisation: Omega from the numerical BdG spectrum (c\_N = 1/2 one field, 1 doublet) equals the closed forms Omega = $|$Delta$|$\^{}2/G - nu int (E\^{}+ + E\^{}-) with nu = 4 (one field, 7+1 and 4+4) and nu = 8 (doublet)  -- -1.8938756076e-4 vs -1.8938756076e-4; -1.4454417882e-3 vs -1.4454417882e-3; -1.8938756076e-4 vs -1.8938756076e-4 & PASS\\
P-6 & vacuum (mu = 0): the spin-zero gap appears only above the critical coupling G\_crit = 4 pi\^{}2/(nu Lambda\^{}2) for m = 0: pi\^{}2 (one field, internal vector or 2-form) and pi\^{}2/2 (doublet); for m = 0.2 Lambda the one-field threshold is higher  -- G\_crit = 9.869604401089 (pi\^{}2 = 9.869604401089), 4.934802200545, m = 0.2: 10.643305 & PASS\\
P-7 & finite density (m = 0.2): for a coupling that is subcritical in vacuum there is no gap for mu $<$ m and a gap for every mu $>$= 0.3 on the grid (Cooper instability of the Fermi surface; just above mu = m the gap is exponentially small, below the solver floor 1e-12), and at weak coupling ln$|$Delta$|$ falls linearly in 1/G with the BCS slope -2 pi\^{}2/(nu q\_F mu)  -- slope -21.41969 vs prediction -21.53724 (ratio 0.9945) & PASS\\
P-8 & at the SAME eight-dimensional vector coupling (7+1, m = 0.2, mu = 0.5, G = 0.9 G\_crit(m)): Omega\_polar $<$ Omega\_chiral and Omega\_polar $<$ Omega\_spin-one for every $|$Delta$|$ on the grid; the minimum is the polar spin-zero state (an internal vector condensate)  -- min Omega: polar -2.505051e-3, chiral -1.252231e-3, spin one -4.971294e-4; polar gap 0.317478 & PASS\\
P-9 & amplitude mode (vacuum, m = 0, G = 1.5 G\_crit): Pi\_A(w) = (4$|$Delta$|$\^{}2 - w\^{}2) c\_sigma(w) with c\_sigma finite and positive on [0, 2$|$Delta$|$], so Pi\_A vanishes at w = 2$|$Delta$|$: m\_sigma = 2$|$Delta$|$; static check: Pi\_A(0) equals half the second derivative of Omega with respect to $|$Delta$|$ at the minimum  -- $|$Delta$|$ = 0.6753386910, Pi\_A(0) = 3.28385662e-2, Omega''/2 = 3.28385662e-2 & PASS\\
P-10 & symmetry breaking (exact stabilisers): internal vector, polar: U(1)\_F x SO(4) (7) -$>$ SO(3) (3), 4 Goldstone modes; chiral: -$>$ 2 generators, one of them LOCKS the fibre rotation Rot(0,5) (an 8D Lorentz rotation) to the fermion number F, 5 Goldstone modes; internal 2-form, self-dual polar: 7 -$>$ U(1) x SU(2) (4), 3 Goldstone modes; 4+4 with the eta-weighted interaction: U(1)\_F x SO(3) (4) -$>$ SO(2) (1), 3 Goldstone modes; the four-dimensional Lorentz group is unbroken in every case (spin-zero condensates)  -- stabiliser dims: polar 3, chiral 2 (locked true), 2-form 4, 4+4 triplet 1 & PASS\\
P-11 & the 8D-invariant interaction (G/4) eta\_AB V\^{}A\^{}dagger V\^{}B (G $>$ 0) of the vector pairs V\^{}A = psi\^{}T C- gamma\^{}A psi attracts the components with eta\_AA = -1: on 7+1 these are the internal 0,5,6,7 (spin zero, isotropic) and the space 1,2,3 (spin one, anisotropic) -- the time component 4 is repelled; on 4+4 they are 4,5,6,7: the internal 5,6,7 (spin zero, isotropic) and the time component 4 (not s-wave) -- the internal 0 and the space 1,2,3 are repelled  -- 7+1 attractive [0, 1, 2, 3, 5, 6, 7]; 4+4 attractive [4, 5, 6, 7] & PASS\\
R-1 & mass dimensions with D space dimensions (D + 1 spacetime dimensions): D = 3: [g\_gauge] = [g\_Yukawa] = [g\_quartic] = 0, [G4f] = -2; D = 7: [fermion] = 7/2, [scalar] = [gauge field] = 3, [g\_gauge] = [g\_Yukawa] = -2, [g\_quartic] = -4, [G4f] = -6 (every interaction coupling has negative mass dimension for D = 7) & PASS\\
R-2 & superficial degree of divergence deg (D space dimensions): four-fermion 4-point function deg = (nvert-1)(D-1) with nvert vertices: 2 at one loop for D = 3 (the Lambda\^{}2 of the gap equation), 6 for D = 7, growing with nvert in both; Yukawa vertex function: independent of nvert for D = 3 (renormalizable), +2 per vertex for D = 7 & PASS\\
R-3 & Jacobi's four-square theorem: r4(ell) = 8 times the sum of the divisors of ell that are not multiples of 4 (r4(0) = 1) agrees with the direct count of the integer vectors n with $|$n$|$\^{}2 = ell for ell = 0..200 & PASS\\
R-4 & Kaluza-Klein tower on the torus T\^{}4 (7+1, m R = 0.01): for Lambda $<$ 1/R the tower sum B is the zero-mode bubble f alone, logarithmic (dB/d ln Lambda = 1/(8 pi\^{}2) up to terms of the order of m\^{}2/Lambda\^{}2; at Lambda = 0.3/R within 0.5 \%); for Lambda $>$$>$ 1/R it grows like Lambda\^{}4 R\^{}4: B/(Lambda R)\^{}4 at Lambda R = 60 is within 4 \% of the continuum (eight-dimensional) value C8 = (1/8) int\_0\^{}1 u\^{}3 [ln(1 + 1/u\^{}2) - 1/(1 + u\^{}2)] du  -- slope ratio 0.99778; B/(Lambda R)\^{}4 = 1.207225e-2 vs C8 = 1.207170e-2 (ratio 1.0000) & PASS\\
R-5 & 4+4: the number of tachyonic Kaluza-Klein modes below the cutoff grows like (Lambda R)\^{}4 (doubling Lambda R from 10 to 20 multiplies it by 16 within 10 \%); the Euclidean bubble f(m\_eff\^{}2) is undefined for m\_eff\^{}2 $<$ 0  -- tachyons: 39966 at Lambda R = 10, 644274 at 20 (ratio 16.121) & PASS\\
NG-S1 & every symbol text is registered once (115 entries) and every plain-text name of the programs, figures and data files belongs to one entry (219 names) & PASS\\
NG-S2 & the 27 taken entries carry exactly the meaning of the MASTER registry from which they are taken (113 entries of the main project, 229 of the revision on the deflating spacetimes compared), and none of the 88 new symbols is a symbol of either master registry & PASS\\
NG-S3 & every column of the 19 data files data/ng\_*.csv is a registered plain-text name or a descriptive word (93 different columns), and every file states its content; 9 column names also occur as names in a master registry or as symbols of the author's notebooks (C, D, G, K, Lambda, mu, phase, q, v) - they cannot be confused, because every data file of Revision\_NoGo has the prefix ng\_ and a line '\# content:', and the name means in it the registered symbol of Revision\_NoGo & PASS\\
NG-S4 & each of the 4 documents of Revision\_NoGo (and the manuscript) contains the complete table of the 115 registered symbols, so that every symbol it uses is defined in the document itself & PASS\\
NG-S5 & in the 1566 formulas of the documents, mu, Lambda, Omega, nu, Pi\^{}A, G\^{}crit and G\^{}D always carry the suffix NG, and Delta the suffix NG or a tag 1671, 1644 & PASS\\
NG-S6 & in the 2019 formulas of the documents and of the new registry entries, no letter that a master registry defines with another meaning stands bare: every letter except the taken indices and sets (A, B, C, D, K, X, a, b, j, r, s, t), the new plain symbols (i, l, p, q) and the differential d carries the suffix NG or a tag 1671, 1644, x occurs only in the coordinate names, and the Greek letters carry their suffix & PASS\\
NG-H1 & none of the 23 documents and document sources of Revision\_NoGo refers its reader to another document for an equation, a symbol or a definition & PASS\\
<!-- END checks_rows_tex -->

<!-- BEGIN nchecks_rust -->
94
<!-- END nchecks_rust -->

<!-- BEGIN nchecks_rust_pass -->
94
<!-- END nchecks_rust_pass -->

<!-- BEGIN assoc_nonzero -->
168
<!-- END assoc_nonzero -->

<!-- BEGIN assoc_zero -->
344
<!-- END assoc_zero -->

<!-- BEGIN split_g2_signature -->
$8$ positive and $6$ negative eigenvalues
<!-- END split_g2_signature -->

<!-- BEGIN g2_stab_full -->
6
<!-- END g2_stab_full -->

<!-- BEGIN g2_proj_rank -->
3
<!-- END g2_proj_rank -->

<!-- BEGIN g2_pure_space -->
0
<!-- END g2_pure_space -->

<!-- BEGIN g2_pure_int -->
3
<!-- END g2_pure_int -->

<!-- BEGIN kk_tach_fraction -->
0.8660
<!-- END kk_tach_fraction -->

<!-- BEGIN susy_triples -->
148877
<!-- END susy_triples -->

<!-- BEGIN susy_triples_ext -->
5929741
<!-- END susy_triples_ext -->

<!-- BEGIN gcrit_one -->
$9.8696044011$
<!-- END gcrit_one -->

<!-- BEGIN gcrit_doublet -->
$4.9348022005$
<!-- END gcrit_doublet -->

<!-- BEGIN gcrit_one_m02 -->
$10.643305$
<!-- END gcrit_one_m02 -->

<!-- BEGIN bcs_ratio -->
$0.9945$
<!-- END bcs_ratio -->

<!-- BEGIN omega_min_polar -->
$-2.505051$
<!-- END omega_min_polar -->

<!-- BEGIN omega_min_chiral -->
$-1.252231$
<!-- END omega_min_chiral -->

<!-- BEGIN omega_min_spin1 -->
$-0.4971294$
<!-- END omega_min_spin1 -->

<!-- BEGIN gap_polar -->
$0.317478$
<!-- END gap_polar -->

<!-- BEGIN n_omega_grid -->
60
<!-- END n_omega_grid -->

<!-- BEGIN gap_amplitude -->
$0.6753386910$
<!-- END gap_amplitude -->

<!-- BEGIN pia0 -->
$3.28385662\times10^{-2}$
<!-- END pia0 -->

<!-- BEGIN omega_second -->
$3.28385662\times10^{-2}$
<!-- END omega_second -->

<!-- BEGIN power_rows -->
fermion field & $D/2$ & $3/2$ & $7/2$\\
scalar or gauge field & $(D-1)/2$ & $1$ & $3$\\
gauge or Yukawa coupling & $(3-D)/2$ & $0$ & $-2$\\
quartic scalar coupling & $3-D$ & $0$ & $-4$\\
four-fermion coupling & $1-D$ & $-2$ & $-6$\\
<!-- END power_rows -->

<!-- BEGIN kk_slope_ratio -->
$0.99778$
<!-- END kk_slope_ratio -->

<!-- BEGIN B_over_LR4 -->
$1.207225\times10^{-2}$
<!-- END B_over_LR4 -->

<!-- BEGIN C8 -->
$1.207170\times10^{-2}$
<!-- END C8 -->

<!-- BEGIN B_ratio -->
$1.0000$
<!-- END B_ratio -->

<!-- BEGIN tach10 -->
39966
<!-- END tach10 -->

<!-- BEGIN tach20 -->
644274
<!-- END tach20 -->

<!-- BEGIN tach_ratio -->
$16.121$
<!-- END tach_ratio -->

<!-- BEGIN data_files_table -->
| data file | content (header line of the file) | columns |
|---|---|---|
| `ng_amplitude_mode.csv` | amplitude-mode kernel in vacuum, m = 0, G = 1.5 pi^2, \|Delta\| = 0.6753386910: w/(2\|Delta\|), c_sigma(w), Pi_A(w) | `w_over_2Delta` `c_sigma` `Pi_A` |
| `ng_bcs_law.csv` | weak-coupling law at m = 0.2, mu = 0.5: 1/G, \|Delta\|, ln\|Delta\|; predicted slope d ln Delta / d(1/G) = -21.5372424987 | `invG` `Delta` `lnDelta` |
| `ng_dispersion.csv` | lowest BdG branches at m = 0.2, mu = 0.5, \|Delta\| = 0.3174778196: polar E^-, chiral branches sqrt((eps-mu)^2 + 2 Delta^2) and \|eps - mu\|, spin one along the pair direction and perpendicular to it | `q` `polar` `chiral_gapped` `chiral_gapless` `spin1_parallel` `spin1_perp` |
| `ng_g2_locking.csv` | G2 locking on 7+1: dimensions of Lie algebras inside so(3) (space rotations, Im H) + so(4) (internal rotations, H l) | `row` `dimension` |
| `ng_gap_vs_coupling.csv` | vacuum (mu = 0) gap \|Delta\| (units of Lambda) versus coupling G; G_over_Gcrit = G/G_crit with G_crit = pi^2/Lambda^2 (one field, m = 0) | `G_over_Gcrit` `G` `Delta_onefield_m0` `Delta_doublet_m0` `Delta_onefield_m0.2` |
| `ng_gap_vs_mu.csv` | one-field spin-zero gap \|Delta\| versus mu at m = 0.2, G = 0.6 G_crit(m=0.2) = 6.3859829267 (subcritical in vacuum); a gap below 1e-12 is reported as 0 | `mu` `Delta` |
| `ng_goldstone.csv` | symmetry breaking patterns: phase 1 = internal vector polar, 2 = internal vector chiral, 3 = internal 2-form self-dual polar (7+1), 4 = internal triplet polar with the eta-weighted interaction (4+4); dimension of the symmetry group, dimension of the unbroken subgroup, number of broken generators | `signature` `phase` `dim_symmetry` `dim_unbroken` `broken` |
| `ng_intertwiner_1644.csv` | integer intertwiner I1644 with I gamma1644[A] = Gamma_oct[A] I (A = 0..7); rows 0..15, columns col0..col15 | `col0` `col1` `col2` `col3` `col4` `col5` `col6` `col7` `col8` `col9` `col10` `col11` `col12` `col13` `col14` `col15` |
| `ng_intertwiner_1671.csv` | integer intertwiner I1671 with I gamma1671[A] = Gamma_oct[A] I (A = 0..7); rows 0..15, columns col0..col15 | `col0` `col1` `col2` `col3` `col4` `col5` `col6` `col7` `col8` `col9` `col10` `col11` `col12` `col13` `col14` `col15` |
| `ng_kk_bubble.csv` | one-loop bubble B summed over the Kaluza-Klein tower on T^4 (R = 1, m R = 0.01) versus the cutoff Lambda, the zero-mode bubble f, and the continuum (eight-dimensional) form C8 (Lambda R)^4 with C8 = 0.012071698785 | `Lambda` `B_tower` `f_zero_mode` `C8_LambdaR4` `B_over_LambdaR4` |
| `ng_kk_tachyons.csv` | Kaluza-Klein modes n in [-Nmax, Nmax]^4 on a torus of radius R with (mR)^2 = 1: number of tachyonic modes (m_eff^2 < 0) | `Nmax` `modes` `tachyons_71` `tachyons_44` `fraction_44` |
| `ng_kk_tachyons_cutoff.csv` | 4+4 Kaluza-Klein modes with \|n\| <= Lambda R ((m R)^2 = 1): all modes, tachyonic modes, tachyons/(Lambda R)^4 | `LambdaR` `modes` `tachyons` `tachyons_over_LR4` |
| `ng_octonion_tables.csv` | multiplication tables of O_NG (varsigma = -1) and Os_NG (varsigma = +1): e_(idx_left) e_(idx_right) = sign * e_(idx_product) | `algebra` `left` `right` `product` `idx_left` `idx_right` `idx_product` `sign` |
| `ng_omega_states.csv` | Omega(\|Delta\|) - Omega(0) at m = 0.2, mu = 0.5, G = 9.5789743900 (one field, 7+1): polar spin-zero, chiral spin-zero (zeta = \|sin 2 ups\| = 1), spin-one polar | `Delta` `Omega_polar` `Omega_chiral` `Omega_spin1` |
| `ng_pair_channels.csv` | all one-field pair matrices C(+-) gamma^K (K a subset of {0..7}) on both spacetimes | `signature` `C` `K` `n4` `nt` `nint` `symmetry` `nonzero` `s_wave` `isotropic` `representation` |
| `ng_power_counting.csv` | mass dimensions with D space dimensions (D + 1 spacetime dimensions) of a fermion field, a scalar field, a gauge field, a gauge coupling, a Yukawa coupling, a quartic scalar coupling and a four-fermion contact coupling G4f | `D` `fermion` `scalar` `gauge_field` `g_gauge` `g_Yukawa` `g_quartic` `G4f` |
| `ng_superalgebra_positivity.csv` | smallest and largest eigenvalue of {Q, Q} = sum_C Y^C P_C for P = (P_4 = 1, P_1 = v, other components 0) on 7+1 and on 4+4 (numerical diagonalisation) | `signature` `v` `eig_min` `eig_max` |
| `ng_superficial_degree.csv` | superficial degree of divergence deg versus the number of vertices nvert: four-fermion 4-point function with D = 3 and D = 7 space dimensions; Yukawa vertex function (2 external fermion lines, 1 external boson line) with D = 3 and D = 7 | `nvert` `fourfermion_D3` `fourfermion_D7` `yukawa_D3` `yukawa_D7` |
| `ng_triality_charpolys.csv` | characteristic polynomials det(lambda - matrix) of the simple rotation r01 and of its images under the triality automorphisms T2, T3, vartheta, vartheta^2 (exact rational coefficients) | `algebra` `matrix` `char_poly` |
<!-- END data_files_table -->

<!-- BEGIN data_files_tex -->
\texttt{ng\_amplitude\_mode.csv}: amplitude-mode kernel in vacuum, m = 0, G = 1.5 pi\^{}2, $|$Delta$|$ = 0.6753386910: w/(2$|$Delta$|$), c\_sigma(w), Pi\_A(w).
\texttt{ng\_bcs\_law.csv}: weak-coupling law at m = 0.2, mu = 0.5: 1/G, $|$Delta$|$, ln$|$Delta$|$; predicted slope d ln Delta / d(1/G) = -21.5372424987.
\texttt{ng\_dispersion.csv}: lowest BdG branches at m = 0.2, mu = 0.5, $|$Delta$|$ = 0.3174778196: polar E\^{}-, chiral branches sqrt((eps-mu)\^{}2 + 2 Delta\^{}2) and $|$eps - mu$|$, spin one along the pair direction and perpendicular to it.
\texttt{ng\_g2\_locking.csv}: G2 locking on 7+1: dimensions of Lie algebras inside so(3) (space rotations, Im H) + so(4) (internal rotations, H l).
\texttt{ng\_gap\_vs\_coupling.csv}: vacuum (mu = 0) gap $|$Delta$|$ (units of Lambda) versus coupling G; G\_over\_Gcrit = G/G\_crit with G\_crit = pi\^{}2/Lambda\^{}2 (one field, m = 0).
\texttt{ng\_gap\_vs\_mu.csv}: one-field spin-zero gap $|$Delta$|$ versus mu at m = 0.2, G = 0.6 G\_crit(m=0.2) = 6.3859829267 (subcritical in vacuum); a gap below 1e-12 is reported as 0.
\texttt{ng\_goldstone.csv}: symmetry breaking patterns: phase 1 = internal vector polar, 2 = internal vector chiral, 3 = internal 2-form self-dual polar (7+1), 4 = internal triplet polar with the eta-weighted interaction (4+4); dimension of the symmetry group, dimension of the unbroken subgroup, number of broken generators.
\texttt{ng\_intertwiner\_1644.csv}: integer intertwiner I1644 with I gamma1644[A] = Gamma\_oct[A] I (A = 0..7); rows 0..15, columns col0..col15.
\texttt{ng\_intertwiner\_1671.csv}: integer intertwiner I1671 with I gamma1671[A] = Gamma\_oct[A] I (A = 0..7); rows 0..15, columns col0..col15.
\texttt{ng\_kk\_bubble.csv}: one-loop bubble B summed over the Kaluza-Klein tower on T\^{}4 (R = 1, m R = 0.01) versus the cutoff Lambda, the zero-mode bubble f, and the continuum (eight-dimensional) form C8 (Lambda R)\^{}4 with C8 = 0.012071698785.
\texttt{ng\_kk\_tachyons.csv}: Kaluza-Klein modes n in [-Nmax, Nmax]\^{}4 on a torus of radius R with (mR)\^{}2 = 1: number of tachyonic modes (m\_eff\^{}2 $<$ 0).
\texttt{ng\_kk\_tachyons\_cutoff.csv}: 4+4 Kaluza-Klein modes with $|$n$|$ $<$= Lambda R ((m R)\^{}2 = 1): all modes, tachyonic modes, tachyons/(Lambda R)\^{}4.
\texttt{ng\_octonion\_tables.csv}: multiplication tables of O\_NG (varsigma = -1) and Os\_NG (varsigma = +1): e\_(idx\_left) e\_(idx\_right) = sign * e\_(idx\_product).
\texttt{ng\_omega\_states.csv}: Omega($|$Delta$|$) - Omega(0) at m = 0.2, mu = 0.5, G = 9.5789743900 (one field, 7+1): polar spin-zero, chiral spin-zero (zeta = $|$sin 2 ups$|$ = 1), spin-one polar.
\texttt{ng\_pair\_channels.csv}: all one-field pair matrices C(+-) gamma\^{}K (K a subset of \{0..7\}) on both spacetimes.
\texttt{ng\_power\_counting.csv}: mass dimensions with D space dimensions (D + 1 spacetime dimensions) of a fermion field, a scalar field, a gauge field, a gauge coupling, a Yukawa coupling, a quartic scalar coupling and a four-fermion contact coupling G4f.
\texttt{ng\_superalgebra\_positivity.csv}: smallest and largest eigenvalue of \{Q, Q\} = sum\_C Y\^{}C P\_C for P = (P\_4 = 1, P\_1 = v, other components 0) on 7+1 and on 4+4 (numerical diagonalisation).
\texttt{ng\_superficial\_degree.csv}: superficial degree of divergence deg versus the number of vertices nvert: four-fermion 4-point function with D = 3 and D = 7 space dimensions; Yukawa vertex function (2 external fermion lines, 1 external boson line) with D = 3 and D = 7.
\texttt{ng\_triality\_charpolys.csv}: characteristic polynomials det(lambda - matrix) of the simple rotation r01 and of its images under the triality automorphisms T2, T3, vartheta, vartheta\^{}2 (exact rational coefficients).
\texttt{ng\_octonions\_checks.txt}, \texttt{ng\_triality\_checks.txt}, \texttt{ng\_kaluza\_checks.txt}, \texttt{ng\_superalgebra\_checks.txt}, \texttt{ng\_pairs4d\_checks.txt}, \texttt{ng\_renorm\_checks.txt}, \texttt{ng\_symbol\_checks.txt}, \texttt{ng\_self\_contained\_checks.txt}: the reports of Table~\ref{tab:checks}.
<!-- END data_files_tex -->

<!-- BEGIN n_rust_lines -->
3667
<!-- END n_rust_lines -->

<!-- BEGIN n_rust_files -->
12
<!-- END n_rust_files -->

<!-- BEGIN data_sha256 -->
7581b21457c4b201a31d5ee1b7daad600011740b61ac998970db17b870dab7de  ng_amplitude_mode.csv
ace9c68c313eaa907704b18c1dd724d1c4eecc37132872268b0e762dbe971c92  ng_bcs_law.csv
f5e9fd9382cd77e098c567f83121d34a34d7491f2c5cf4b92f6d8c26d12dac21  ng_dispersion.csv
1d02cf60b88feb24a39e1dcbcafb196f90dc3328b93e943cbc00ee0637a606f9  ng_g2_locking.csv
60095189540f12acb94104ba536a21cad44a817e538c9349dd87ececfbb87289  ng_gap_vs_coupling.csv
21e926935ea50156616c7f368aea1aadb3ffa7ed1f70c3f96a77abfc209499d3  ng_gap_vs_mu.csv
d8202c9953c51ffc61466c1b8965336b14207b0879e2750d9c7d5a1cb117e07b  ng_goldstone.csv
a024225b3b0dc9627ce0957c8cf35e668d8fbf91bcac44932257775a7121ec41  ng_intertwiner_1644.csv
c33e2d1bf122aa44c77fa3f4a6da7cab0c823e79b24152cd469cc9ea6ccf53dd  ng_intertwiner_1671.csv
7837e931de0aca7f188d18a0e6f4f636e69f62c051e6e9a915d83544cc15077f  ng_kaluza_checks.txt
615ab0663924770a6edd51a1e03a902140a31c467d7c0b096040abb30d558381  ng_kk_bubble.csv
d9b0141fc8d8c1c13525074e87c85a31c8f6040b37e46cc3acb8fdb7fdbba128  ng_kk_tachyons.csv
b5dcf3227375edb906b602f3fbe842208c8a49a321e782715b6ccf5f87e84825  ng_kk_tachyons_cutoff.csv
96aef6d10feb1f067565468494ff3959e1c25a6727c8ef927d3a1fb0e0c69ef7  ng_octonion_tables.csv
9b5e31e6ce4f2093c55efd7c0021e1017688b493741c6c1aca4c9f83e85d42ff  ng_octonions_checks.txt
6b97f2f4cf1f46e1a816b0d5fcdbfaf5ddcca52556db43dd969a55b7c2d24f12  ng_omega_states.csv
77870b45147eeee3c8005c004cdaa8cf5324a0644bf230efff62d9a730d9c55a  ng_pair_channels.csv
e724db047e95437f7caf72129d1c4351c8b4e1c3ce67f73363c59f7e2063f1e5  ng_pairs4d_checks.txt
c473330fca82dabfe5767d609f0fa1a54f11d260330c9d313e8ddccf93733a4e  ng_power_counting.csv
0833127fee94257490f68b7c649c519ce6c7085774b3700c8b20844b7a35baf6  ng_renorm_checks.txt
4ffd95f8004a197852554cc91f987ca7f973f9eb11584212f7da602dbf632e77  ng_superalgebra_checks.txt
2f45a964a921b2a84a57a196c84837c3cac63043abc547229fd767a10b4d703b  ng_superalgebra_positivity.csv
df04bc9bedd4583261115df578d143a3a14798c69cc13a0a04ae819ceb8d0569  ng_superficial_degree.csv
423c363f20934fd7cf108526409ae86010370c3a9d687c67f1dbab85053e6014  ng_triality_charpolys.csv
701264c70b23426f38480a6604201df0706b67d123bdd8a7f1fac3322123193f  ng_triality_checks.txt
<!-- END data_sha256 -->

<!-- BEGIN figure_sha256 -->
0aec696b6932e627f8d4b62a8214221a55efde72ef5e8484e14ebb203b3e4b99  fig_ng_amplitude.pdf
fb32e887ea4b4a40d68036de4414e63d0e7b5eb784f0962d277ee755f7d9ab7c  fig_ng_channels.pdf
d88c8b9006c68f9d33cc90e9f1a8eef3b47b2642fd2ab8be019d780565d99dd1  fig_ng_density.pdf
c9a26e48a234e890eb01031828950f2fdda08b9c5511a1584372efb23ee5a598  fig_ng_gap.pdf
523d24c2b9201a12165a71eaa69dddea8c0b78fe49a88fc2f6bcfbe6525068f4  fig_ng_goldstone.pdf
356f1ee01bf39d02171cea0834d0cb22c0be500b7ed11e1e86fb00853516c05d  fig_ng_kk.pdf
443090e855ac11e883caf7ab155916e49e808f944b335eb1bdae5df448f989c8  fig_ng_locking.pdf
1f635b841ce23515e258db02d868581fde7082c527870411ce337fc33a58d053  fig_ng_manifolds.pdf
bbe9e561a46c8c1fde8fcd441fdc79bdd3a9206d4122b7f0015218fbcf26336b  fig_ng_renorm.pdf
e276b424640d3bee995dd3498ce0b2a92869cad945f4c0bf98479a6e45a3d3e7  fig_ng_states.pdf
bc4b366b8cb980b542824ee339f29b7d69dcd55c2b58f320891e9c7811531c5b  fig_ng_superalgebra.pdf
a24a4bf70a97a2577bc596a22b2abd359325cdc90f50b55dbb137c42040a6edb  fig_ng_tables.pdf
3c0fce19bda2e2c6c3ccf2f03aa416e03944a7910d0bff425545989eea74affd  fig_ng_triality.pdf
<!-- END figure_sha256 -->

<!-- BEGIN code_sha256 -->
41fba6fbce624acc26f145e11bbf50faece92062bcd209d03b902264050e1041  code/nogo-rs/src/checks.rs
e09bd5867108304af90fef73d6d97fea4b1cb720104bf260a50528fda2df2ded  code/nogo-rs/src/exact.rs
651efab2b48a6f53e4eca1e3a1e5d4731a19d1d91bd9feeceb12362382406f1a  code/nogo-rs/src/integrate.rs
f703ad2758fef4dcd9f85685ff4c317e219d0f0d6dfcfd9656082a0d49ec44b3  code/nogo-rs/src/kaluza.rs
b7084b44b1df2d4ad9408fc6d8fee2a758d24730fe18560b4bb1dd0d90f7cdb5  code/nogo-rs/src/main.rs
a5b05a0b80abe3f085f9e11233eba5d1611caf7b51ee710017e212385b6af605  code/nogo-rs/src/oct.rs
bf5347e564cb78a2b913bc5aa08e5acfdd73ba83d604b34a25a27595f9a1b873  code/nogo-rs/src/pairs4d.rs
0803714a1180262e13a3d32283b8a59b2a96439ad0891cfd96d221f7298f1d2d  code/nogo-rs/src/renorm.rs
a27a2ded2fb2da04f790ddc474feee25a802c51c8513c29102bcd6829a109fb1  code/nogo-rs/src/rsym.rs
a78d4201c2f610f6f7f5ac7a98df79c20573abe704729161de54e2feb607bce6  code/nogo-rs/src/sig.rs
4ab3b75629d85a130006bf5e9d52988c641aea2c4c0fd5fac3474921b985721c  code/nogo-rs/src/superalg.rs
350c1e082b9a9c323c2d84af9b66a38736a29b99c7e4939628a0c900760c9e5e  code/nogo-rs/src/triality.rs
8c7ece7d5d4ef2948d08fa207f46e620c648ba22842e227096aa0c41c47b97da  code/nogo-rs/Cargo.toml
a11ef5cd08c51dccaf01b1f28be4ff85b747d0c3793c2c7bed584875941f4c9a  code/scripts/assemble_docs.py
69e40372f846b3421d7e5aa439fe6e66fcfcf63966bb44a37122c6536264b222  code/scripts/build_manuscript.py
465063731c88f51a1d8af013d40f1cae7613007e63d3256e5ab26b364bd3b455  code/scripts/check_layout.py
575538fca483a4b2a62c3b345768e1f9c8a812a240be2b579a2576805178c7e6  code/scripts/check_self_contained.py
973f838f92aacdcba55859777f2cee9f3955fe14630176f70beae6982908263d  code/scripts/check_symbols.py
2c3a5bac5d4f6cfb36748115cfefc0b99d8c6f66db516efe11113a204deab143  code/scripts/make_figures.py
36b06d03bd0e13e815b050b4338c810c254e860341bbaf7996dacabb77953c55  code/scripts/tex2md.py
9ecf4785626cab2db94f02f53ea7b340bd3884243548547d6049d178c1342c34  code/scripts/write_doc_tables.py
200df9b86494286b4c4f1a48a0f226280688b7da1854f58e7cbc8a1e0b6afee2  code/scripts/write_symbol_registry.py
f67ec04162b5942359f455bb3ebbffeda87f85ef716c27801990036267cd7ce6  code/scripts/build_docs.sh
596f2269a6094d1224ca243cfcd7a4e3731bf5cac7fdfba1d05283cb3c3df728  code/scripts/check_all.sh
f7da314c7097bf0920e92cda2b3caefa9da132414dd68990252c5160a9fc8372  code/scripts/doc_header.tex
<!-- END code_sha256 -->
