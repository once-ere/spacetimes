<!-- BEGIN nchecks_algebra -->
16
<!-- END nchecks_algebra -->

<!-- BEGIN nchecks_gauge -->
7
<!-- END nchecks_gauge -->

<!-- BEGIN nchecks_bdg -->
7
<!-- END nchecks_bdg -->

<!-- BEGIN nchecks_potential -->
13
<!-- END nchecks_potential -->

<!-- BEGIN nchecks_modes -->
12
<!-- END nchecks_modes -->

<!-- BEGIN nchecks_symbol -->
6
<!-- END nchecks_symbol -->

<!-- BEGIN nchecks_self_contained -->
1
<!-- END nchecks_self_contained -->

<!-- BEGIN checks_table -->
| report (data/) | command | checks passed |
|---|---|---|
| `r3_algebra_checks.txt` | `dark_u2 algebra` | 16 of 16 |
| `r3_gauge_checks.txt` | `dark_u2 gauge` | 7 of 7 |
| `r3_bdg_checks.txt` | `dark_u2 bdg` | 7 of 7 |
| `r3_potential_checks.txt` | `dark_u2 potential` | 13 of 13 |
| `r3_modes_checks.txt` | `dark_u2 modes` | 12 of 12 |
| `r3_symbol_checks.txt` | `check_symbols.py` | 6 of 6 |
| `r3_self_contained_checks.txt` | `check_self_contained.py` | 1 of 1 |
<!-- END checks_table -->

<!-- BEGIN checks_rows_tex -->
R3-A1 & the 8 real 16x16 matrices gamma\^{}A\_1671 satisfy gamma\^{}A gamma\^{}C + gamma\^{}C gamma\^{}A = 2 eta1671\^{}\{AC\} 1 with eta1671 = diag(-1,-1,-1,-1,+1,-1,-1,-1) (64 identities) & PASS\\
R3-A2 & the matrices are real (integer); gamma\^{}4 is symmetric and the seven gamma\^{}j (j = 0,1,2,3,5,6,7) are antisymmetric; hence beta = gamma\^{}4 and alpha\^{}j = gamma\^{}4 gamma\^{}j are real symmetric and the generators gamma\^{}A gamma\^{}C of the spin group are real: the spinor representation is real, and psi\^{}* transforms like psi & PASS\\
R3-A3 & omega\_1671 = gamma\^{}0 ... gamma\^{}7 satisfies omega\^{}2 = -1, omega\^{}T = -omega, and anticommutes with every gamma\^{}A & PASS\\
R3-A4 & C\^{}+ = gamma\^{}4 and C\^{}- = gamma\^{}4 omega are real SYMMETRIC, (C\^{}+)\^{}2 = (C\^{}-)\^{}2 = 1, and C\^{}+ gamma\^{}A = +(gamma\^{}A)\^{}T C\^{}+, C\^{}- gamma\^{}A = -(gamma\^{}A)\^{}T C\^{}- for every A & PASS\\
R3-A5 & for each of the 28 generators S = gamma\^{}A gamma\^{}C (A $<$ C) of Spin(7,1): S\^{}T C\^{}+ + C\^{}+ S = 0 and S\^{}T C\^{}- + C\^{}- S = 0, so psi\^{}T C\^{}+ psi' and psi\^{}T C\^{}- psi' are Lorentz scalars & PASS\\
R3-A6 & the bilinear forms K on the 16 spinor components that are invariant under Spin(7,1) (S\^{}T K + K S = 0 for all 28 generators) form a space of dimension exactly 2, spanned by C\^{}+ and C\^{}-: the rank of the linear system (7168 equations, 256 unknowns) is 254, and C\^{}+, C\^{}- are two independent solutions  -- rank modulo 2\^{}61-1 = 254, independent solutions = 2 & PASS\\
R3-A7 & pairs psi\^{}T X psi of ONE anticommuting field are nonzero only for antisymmetric X; of the 512 matrices C\^{}+ G\_K and C\^{}- G\_K (G\_K = gamma\^{}A1 ... gamma\^{}Ar, A1 $<$ ... $<$ Ar, the 256 ordered products over the index sets K) each is either symmetric or antisymmetric, and the four Lorentz-scalar ones (rank 0 and rank 8: C\^{}+, C\^{}-, C\^{}+ omega, C\^{}- omega) are all symmetric: a single Dirac field on the 7+1 spacetime has NO spin-zero (Lorentz-scalar) pair  -- antisymmetric C\^{}+ G\_K by rank 0..8: [0, 0, 28, 56, 0, 0, 28, 8, 0]; antisymmetric C\^{}- G\_K by rank 0..8: [0, 8, 28, 0, 0, 56, 28, 0, 0] & PASS\\
R3-A8 & on the doublet Psi\_U2 = (psi\^{}1\_U2, psi\^{}2\_U2) (32 components, doublet index outside) the matrices E (x) C\^{}+ and E (x) C\^{}- are antisymmetric and nonzero, while S (x) C\^{}+- is symmetric for the three symmetric 2x2 matrices S = 1, tau\^{}1, tau\^{}3: the spin-zero pairs Phi\^{}+-\_U2 = psi\^{}1T C\^{}+- psi\^{}2 = (1/2) Psi\^{}T (E (x) C\^{}+-) Psi exist, and they are antisymmetric in the doublet index & PASS\\
R3-A9 & (tau\^{}c)\^{}T E + E tau\^{}c = 0 for c = 1, 2, 3 (so U\^{}T E U = det(U) E for every 2x2 matrix U, and E is invariant under SU(2)), E = i tau\^{}2, and [tau\^{}a, tau\^{}b] = 2 i tau\^{}c for (a,b,c) cyclic: the pair is an SU(2)\_U2 singlet, and it carries the U(1)\_U2 charge 2 y\_U2 = 1 & PASS\\
R3-A10 & with 2P\^{}L = 1 - i omega, 2P\^{}R = 1 + i omega: (P\^{}X)\^{}T C\^{}+- P\^{}Y = 0 exactly when X differs from Y, so the spin-zero pairs join a left-handed with a left-handed and a right-handed with a right-handed field; the mass term psi-bar psi joins L with R; (P\^{}L)\^{}T C\^{}+ P\^{}L is symmetric, so a single chiral field has no spin-zero pair either, while a chiral DOUBLET has one  -- C+:LL=1 C+:LR=0 C+:RL=0 C+:RR=1 C-:LL=1 C-:LR=0 C-:RL=0 C-:RR=1 & PASS\\
R3-A11 & in the rest frame: (1+beta) C\^{}+ (1+beta) = 2(1+beta) and (1-beta) C\^{}+ (1-beta) = -2(1-beta), while (1+-beta) C\^{}- (1+-beta) = 0: C\^{}+ pairs two upper (positive-energy, rest-frame) components with each other (s-wave), C\^{}- never does (it pairs upper with lower components, which vanish in the non-relativistic limit) & PASS\\
R3-A12 & the 21 rotation generators gamma\^{}i gamma\^{}j (i $<$ j space-like) are real antisymmetric and commute with beta; their commutant has dimension exactly 4 (spanned by 1, beta, omega, beta omega) and the commutant of the rotations together with beta has dimension exactly 2 (1, beta): each 8-dimensional eigenspace of beta is an absolutely irreducible real representation of Spin(7), so the rotation-invariant bilinear form on it is unique and there is exactly ONE spin-zero state of two rest-frame fermions of different flavour  -- ranks modulo 2\^{}61-1: 252 (rotations), 254 (rotations and beta); 4 resp. 2 independent solutions exhibited & PASS\\
R3-A13 & Hamiltonian algebra: alpha\^{}j, beta real symmetric with \{alpha\^{}i, alpha\^{}j\} = 2 delta\^{}\{ij\}, \{alpha\^{}j, beta\} = 0, beta\^{}2 = 1; C\^{}+ anticommutes with every alpha\^{}j and COMMUTES with beta, C\^{}- anticommutes with every alpha\^{}j and with beta; in the 32-dimensional Nambu space the mass vertex diag(beta,-beta) ANTICOMMUTES with the C\^{}+ pairing vertex [[0,C\^{}+],[C\^{}+,0]] (mass and gap add in quadrature) but COMMUTES with the C\^{}- pairing vertex (mass and gap add or subtract); diag(alpha\^{}j, alpha\^{}j) anticommutes with all three & PASS\\
R3-A14 & each of the 8 matrices gamma\^{}A\_1671 is +- a Kronecker product f1 (x) f2 (x) f3 (x) f4 of four of the real 2x2 matrices 1, X, Z, E (exact equality of all 256 entries), so the matrices can be written down completely in eight short formulas (data/r3\_gamma\_kron.csv)  -- 0:-ZZZE 1:-ZXEX 2:+ZXEZ 3:-ZX1E 4:+X111 5:+ZZE1 6:+ZZXE 7:+ZE11 & PASS\\
R3-B1 & sum\_c tau\^{}c (x) tau\^{}c = 2 SWAP - 1 on two doublets: on the antisymmetric (singlet) state it equals -3, on the three symmetric (triplet) states +1; hence T\_1 . T\_2 = -3/4 in the singlet and +1/4 in the triplet, and the exchange of the SU(2)\_U2 bosons attracts in the singlet channel of the pair & PASS\\
R3-B2 & the symmetrized traces of 3 and of 5 Pauli matrices vanish for all 27 resp. 243 index choices (SU(2)\_U2 alone has no perturbative anomaly in 8 dimensions); for the left-handed Weyl content the U(1)\_U2 anomaly coefficients sum y\^{}5, sum y\^{}3, sum y are (1/16, 1/4, 1) for a CHIRAL doublet (anomalous: U(1)\_U2 cannot be gauged) and (0, 0, 0) for the Dirac doublet, whose right-handed members are left-handed fields of charge -y  -- chiral: sum Y\^{}5 = 2, sum Y\^{}3 = 2, sum Y = 2 (y = Y/2); Dirac: 0, 0, 0; 243 five-fold traces checked & PASS\\
R3-G1 & covariance of the covariant derivative: D'\_A Psi' = Ups D\_A Psi for 4 random field configurations, 8 directions each, at a point X0  -- largest relative deviation 3.02e-12 & PASS\\
R3-G2 & the transformed connection calW'\_A is Hermitian (it lies in the Lie algebra u(2)) and its U(1)\_U2 part is B'\_A = B\_A - (d\_A Theta\^{}0)/g'\_U2  -- Hermiticity 8.81e-13, U(1) part 2.75e-12 & PASS\\
R3-G3 & the matrix field strength d\_A calW\_C - d\_C calW\_A + i[calW\_A, calW\_C] equals g (tau.W\_AC)/2 + g' y B\_AC with W\_AC = d\_A W\_C - d\_C W\_A - g W\_A x W\_C and B\_AC = d\_A B\_C - d\_C B\_A  -- largest relative deviation 1.11e-13 & PASS\\
R3-G4 & [D\_A, D\_C] Psi = i calF\_AC Psi (second derivatives by nested differences)  -- largest relative deviation 5.44e-13 & PASS\\
R3-G5 & the field strength transforms covariantly: calF'\_AC = Ups calF\_AC Ups\^{}-1  -- largest relative deviation 3.16e-12 & PASS\\
R3-G6 & for 6 random gauge transformations Ups: Ups\^{}T E Ups = det(Ups) E and det(Ups) = exp(2 i y\_U2 Theta\^{}0): the pair Phi\^{}+\_U2 is invariant under SU(2)\_U2 and is multiplied by exp(i Theta\^{}0) under U(1)\_U2 (charge 2 y\_U2 = 1)  -- 0.0e0, 2.3e-16 & PASS\\
R3-G7 & the Yukawa potential of a vector boson of mass m in 7 space dimensions, G7(r) phi(m r) with phi(x) = exp(-x)(x\^{}2+3x+3)/3, solves the radial equation Y'' + (6/r) Y' - m\^{}2 Y = 0, and phi(0) = 1 (massless limit: the 7-dimensional Coulomb potential G7(r) = Gamma(5/2)/(4 pi\^{}(7/2) r\^{}5))  -- relative residual 1.0e-8 & PASS\\
R3-C1 & spin-zero pair K = C\^{}+: for 25 random (q, M, mu, Delta) the 32 eigenvalues of the real symmetric BdG matrix are +-E\^{}+ and +-E\^{}-, each 8-fold, with E\^{}+- = sqrt((eps +- mu)\^{}2 + Delta\^{}2), eps = sqrt(q\^{}2 + M\^{}2)  -- largest deviation 1.29e-14 & PASS\\
R3-C2 & pseudoscalar pair K = C\^{}-: the eigenvalues are +-sqrt(eps\^{}2 + mu\^{}2 + Delta\^{}2 +- 2 sqrt(mu\^{}2 eps\^{}2 + M\^{}2 Delta\^{}2)), each 8-fold  -- largest deviation 1.38e-14 & PASS\\
R3-C3 & explicit spinor solutions: with w an eigenvector of h(q) = alpha.q + beta M of eigenvalue s eps (s = +-1, 8 each), xi = s eps - mu, E = sqrt(xi\^{}2 + Delta\^{}2), u = sqrt((1 + xi/E)/2), v = sqrt((1 - xi/E)/2), the 32 vectors (u w, v beta w) and (-v w, u beta w) are orthonormal eigenvectors of the BdG matrix with eigenvalues +E and -E  -- largest residual 1.78e-15, largest deviation from orthonormality 4.44e-16 & PASS\\
R3-C4 & condensate with pair momentum Q (phase gradient), K = C\^{}+, mu = 0: the eigenvalues are +-sqrt(q\^{}2 + Q\^{}2/4 + M\^{}2 + Delta\^{}2 +- sqrt((q.Q)\^{}2 + Delta\^{}2 Q\^{}2)), each 8-fold  -- largest deviation 1.53e-14 & PASS\\
R3-C5 & chiral (left-handed) doublet: psi\^{}1\_L and the conjugate psi\^{}2*\_L, which is right-handed, span 16 states; with K = C\^{}+ and no mass the 16 eigenvalues are +-sqrt((q -+ mu)\^{}2 + Delta\^{}2), each 4-fold: the chiral doublet pairs with half the states of the Dirac doublet  -- largest deviation 9.99e-15 & PASS\\
R3-U1 & the spectrum does not depend on the phase of the pair field: Delta -$>$ Delta exp(i phi) (upper-right block times exp(i phi), lower-left block times exp(-i phi)) leaves all 32 eigenvalues unchanged (6 random phases); the effective potential depends on $|$Delta$|$ only  -- largest deviation 4.00e-15 & PASS\\
R3-C6 & the gap at the Fermi surface: for the spin-zero pair (C\^{}+) the smallest quasi-particle energy is Delta itself, reached at eps = mu; for the pseudoscalar pair (C\^{}-) it is Delta sqrt(1 - M\^{}2/mu\^{}2) = Delta q\_F/mu, which vanishes in the non-relativistic limit (M = 0.3, mu = 0.5, Delta = 0.08)  -- C\^{}+: 0.0800000000 (BdG matrix 0.0800000000); C\^{}-: 0.0640000005 (BdG matrix 0.0640000000, predicted 0.0640000000) & PASS\\
R3-P1 & R7 vacuum integrals: Int\_0\^{}1 q\^{}6/sqrt(q\^{}2+r\^{}2) dq = J\_6(r) and Int\_0\^{}1 q\^{}6 sqrt(q\^{}2+r\^{}2) dq = J\_8(r) + r\^{}2 J\_6(r) with the reduction formula J\_n = sqrt(1+r\^{}2)/n - (n-1) r\^{}2 J\_\{n-2\}/n, J\_0 = ln((1+sqrt(1+r\^{}2))/r), agree with adaptive quadrature  -- largest difference 3.6e-16 & PASS\\
R3-P2 & R8 closed forms: F(s) = Int\_0\^{}1 u\^{}3 ln(u+s) du = (1-s\^{}4)/4 ln(1+s) + s\^{}4/4 ln s - 1/16 + s/12 - s\^{}2/8 + s\^{}3/4, F'(s) = 1/3 - s/2 + s\^{}2 - s\^{}3 ln((1+s)/s), Int\_0\^{}1 u\^{}3/(u+s)\^{}2 du = 1/2 - 2s + 3 s\^{}2 ln((1+s)/s) + s\^{}3/(1+s) - s\^{}2 agree with quadrature  -- largest difference 7.8e-16 & PASS\\
R3-P3 & critical couplings of the vacuum: R7: G\^{}P\_crit Lambda\^{}6 = 3/(4 a7) = 90 pi\^{}4; R8: 3/(8 b8) = 288 pi\^{}4; the left-handed doublet needs twice the R7 value, 180 pi\^{}4  -- R7 8766.818193 = 90 pi\^{}4 = 8766.818193; R8 28053.818218 = 288 pi\^{}4 = 28053.818218; chiral 17533.636386 & PASS\\
R3-P4 & vacuum, lambda\^{}P $>$ 1: the gap equation 1/G\^{}P = 4 a7 Int q\^{}6 (1/E\^{}+ + 1/E\^{}-) has a root Delta $>$ 0 (residual below 1e-12 in units of 1/G\^{}P), Omega(Delta) $<$ Omega(0) and Delta is a minimum along the radial direction; R8 also has Delta $>$ 0 (102 values of lambda in (1, 3])  -- largest residual 2.1e-15 & PASS\\
R3-P5 & vacuum, lambda\^{}P $<$ 1: Delta = 0 is the global minimum in both regularizations (Omega $>$ 0 on a scan of 40 values of Delta up to 1): no breaking below the critical coupling & PASS\\
R3-P6 & near the critical coupling the gap follows the Landau form of the Mexican hat: Delta\^{}2 = (4/3)(1 - 1/lambda) (R7) and (2/3)(1 - 1/lambda) (R8), from Omega = c2 Delta\^{}2 + c4 Delta\^{}4 with c2 = (4/3) a7 (1/lambda - 1), c4 = a7/2 (R7); at lambda = 1.0005 the ratios of the computed gap to these forms are 1 within 1 per cent  -- R7 ratio 1.00050, R8 ratio 1.00033 & PASS\\
R3-P7 & competition in the vacuum (m = 0): the fermion term depends on M\^{}2 + Delta\^{}2 only, so the minimum is the pure pair state (M, Delta) = (0, Delta(lambda\^{}P)) if lambda\^{}P $>$ lambda\^{}S, the pure mass state (M(lambda\^{}S), 0) - which does NOT break the gauge group - if lambda\^{}S $>$ lambda\^{}P, and a whole circle M\^{}2 + Delta\^{}2 = r\^{}2 of degenerate minima if lambda\^{}S = lambda\^{}P (G\^{}P = 4 G\^{}S)  -- (1.05,1.1): M = 0.00e0, Delta = 0.3796012728; (1.1,1.05): M = 0.3796012728, Delta = 0.0e0; spread of Omega on the circle 8.0e-20 (Omega = -7.059686e-7) & PASS\\
R3-P8a & the logarithm behind the Cooper instability: for mu $>$ 0 the integral 4 a7 Int q\^{}6 (1/E\^{}+ + 1/E\^{}-) grows like 8 a7 q\_F\^{}5 mu ln(1/Delta) as Delta -$>$ 0 (computed at Delta = 1e-100 and 1e-200 with the variable xi = Delta sinh t), so the gap equation 1/G\^{}P = ... has a root for EVERY G\^{}P $>$ 0  -- largest relative deviation of the slope 1.7e-15 & PASS\\
R3-P8 & Cooper instability: at every mu $>$ 0 of the scan (60 values up to 0.6) and every lambda\^{}P in \{0.5, 0.8, 0.95, 1.05\}, also below the critical coupling of the vacuum, the gap equation has a root Delta $>$ 0; where Delta $<$ 1e-300 (weak coupling at small mu) it is not representable in double precision, and its logarithm follows from R3-P8a  -- 185 points with Delta $>$ 1e-300, 55 points with 0 $<$ Delta $<$ 1e-300 & PASS\\
R3-P9 & weak-coupling (BCS) law: ln Delta = const - 1/(8 a7 q\_F\^{}5 mu G\^{}P) at a Fermi surface; at mu = 0.4 the slope of ln Delta against 1/G\^{}P at the two smallest couplings agrees with -1/(8 a7 q\_F\^{}5 mu) within 2 per cent  -- slope -3.567227e5, predicted -3.567227e5, ratio 1.00000 & PASS\\
R3-P10 & density-driven breaking of the gauge group: for lambda\^{}S = 1.1 $>$ lambda\^{}P = 1.05 the vacuum is the massive, U(2)\_U2-symmetric state (Delta = 0); as mu grows the global minimum jumps (first-order transition) to the superfluid with Delta $>$ 0, which breaks U(2)\_U2 -$>$ SU(2)\_U2  -- Delta(mu=0) = 0, Delta(mu=0.6) = 0.560606, transition between mu = 0.190 and 0.200 & PASS\\
R3-P11 & thermodynamic consistency: the density n = 8 a7 Int q\^{}6 [(eps+mu)/E\^{}+ - (eps-mu)/E\^{}-] of the superfluid minus the density 16 a7 mu\^{}7/7 of the normal massless gas equals -d(Omega\_min - Omega\_normal)/dmu (five-point differences, lambda\^{}P = 0.8, mu = 0.2, 0.35, 0.5); the derivative of Omega\_min at fixed parameters needs no derivative of Delta because Delta is stationary  -- largest relative difference 4.8e-7 & PASS\\
R3-P12 & the left-handed (chiral) doublet: its potential at coupling G\^{}P equals that of the Dirac doublet at G\^{}P/2 (half the states), so in the vacuum it pairs only for G\^{}P $>$ 2 G\^{}P\_crit; at finite density it pairs for any attraction (lambda\^{}P = 0.8, mu = 0.4: Delta $>$ 0)  -- Delta(chiral, lambda\^{}P = 0.8, mu = 0.4) = 1.950940e-22 & PASS\\
R3-M1 & normal vacuum (Delta = 0, mu = 0, M = 0 and M = 0.3): the second-order response of the BdG states to a phase gradient equals the surface term (2 a7/7)/sqrt(1 + M\^{}2) of the cutoff, Int\_0\^{}1 q\^{}6 (6 q\^{}2/7 + M\^{}2)/eps\^{}3 dq = 1/(7 sqrt(1+M\^{}2)): the subtraction of the normal state removes exactly the artefact of the cutoff  -- largest relative difference 3.0e-15 & PASS\\
R3-M2 & Goldstone theorem: at every vacuum minimum (lambda\^{}P = 1.005 ... 1.2) the phase entry 2/G\^{}P + Pi\_22(0) of the Hessian vanishes (relative to 2/G\^{}P) to the accuracy of the quadrature, and every q has exactly 16 occupied BdG states  -- largest $|$2/G\^{}P + Pi\_22(0)$|$ G\^{}P/2 = 7.5e-15 & PASS\\
R3-M3 & the amplitude entry 2/G\^{}P + Pi\_11(0) equals d\^{}2 Omega/d Delta\^{}2 computed from the closed-form potential by central differences, and is positive (a true minimum)  -- largest relative difference 2.9e-6 & PASS\\
R3-M4 & the amplitude (sigma\_U2) mode of the massless vacuum sits exactly at the pair-breaking threshold, m\_sigma = 2 Delta: det[U'' + Pi(w)] has no zero in (0, 2 Delta), and 2/G\^{}P + Pi\_11(w) vanishes linearly as w -$>$ 2 Delta (its ratio to 1 - w/2Delta is the same at 1e-4 and 1e-6 within 1 per cent)  -- ratios ["0.99995", "0.99995", "0.99994", "0.99992", "0.99990", "0.99984"] & PASS\\
R3-M5 & vacuum stiffness: the subtracted second-order response 2(S\_n - S\_sf) equals the closed form f\_s\^{}2 = 4 a7 Int q\^{}6 (6q\^{}2/7)(1/q\^{}3 - 1/E\^{}3) dq (m = 0), and is positive (the condensate resists phase gradients: Meissner effect for B\_U2)  -- largest relative difference 2.8e-13 & PASS\\
R3-M6 & time-like stiffness: f\_t\^{}2 = 2 a7 Int q\^{}6 Delta\^{}2 (1/E+\^{}3 + 1/E-\^{}3) equals -(1/4) d\^{}2/dmu\^{}2 of the fermion energy at fixed Delta (central differences)  -- largest relative difference 1.8e-6 & PASS\\
R3-M7 & Lorentz-invariant regularization R8: f\^{}2 = 16 Delta\^{}2 Int d\^{}8p/(2pi)\^{}8 (p\^{}2 + M\^{}2 + Delta\^{}2)\^{}-2 = 8 b8 Delta\^{}2 Int\_0\^{}1 u\^{}3/(u+s)\^{}2 du (s = M\^{}2 + Delta\^{}2), one and the same for space-like and time-like gradients  -- largest relative difference 1.3e-15 & PASS\\
R3-M8 & Goldstone theorem with a bare mass m = 0 ... 0.25 (lambda\^{}P = 1.05): the phase entry of the Hessian vanishes & PASS\\
R3-M12 & the amplitude mode in the vacuum with a bare mass m (mu = 0, no scalar coupling): m\_sigma = 2 Delta exactly, as follows from 2/G\^{}P + Pi\_11(w) = c(w) (4 Delta\^{}2 - w\^{}2) (the radial and the angular directions of the plane (M, Delta) contribute (Delta/r)\^{}2 c (4 r\^{}2 - w\^{}2) and -(M/r)\^{}2 c w\^{}2, r\^{}2 = m\^{}2 + Delta\^{}2); for m $>$ 0 it lies below the pair-breaking threshold 2 sqrt(m\^{}2 + Delta\^{}2): sigma\_U2 is a stable particle  -- largest $|$m\_sigma/(2 Delta) - 1$|$ = 3.7e-13; m=0.00: m\_sigma/threshold=1.000000, m=0.05: m\_sigma/threshold=0.981859, m=0.10: m\_sigma/threshold=0.925304, m=0.15: m\_sigma/threshold=0.822450, m=0.20: m\_sigma/threshold=0.651731, m=0.25: m\_sigma/threshold=0.318082 & PASS\\
R3-M9 & Goldstone theorem at finite density (mu = 0.49 ... 0.60, lambda\^{}P = 0.5): the phase entry of the Hessian vanishes and every q has 16 occupied BdG states & PASS\\
R3-M10 & relativistic superfluid at weak coupling: (a) for Delta $<$ 0.02 the spatial stiffness equals n/(4 mu) within 1 per cent (the whole density is superfluid at zero temperature); (b) where the condensation energy varies slowly with mu (Delta d ln Delta/d mu $<$ 0.1) the Goldstone velocity squared f\_s\^{}2/f\_t\^{}2(relaxed) equals 1/7 = 1/(number of space dimensions), the sound speed of a scale-free gas in 7 space dimensions, within 1 per cent; at larger Delta d ln Delta/d mu the steep growth of the gap with mu (d ln Delta/d mu = 6/(8 a7 mu\^{}7 G\^{}P) in the BCS regime) adds to the compressibility and lowers the velocity (the deviation grows roughly like (Delta d ln Delta/d mu)\^{}2)  -- largest deviations 0.0082, 0.0030 & PASS\\
R3-M11 & the static pair interaction carried by the amplitude field, T\_sigma(0) = -[d\^{}2 Omega/d Delta\^{}2]\^{}-1, is negative (attractive) at every minimum tested; it is the zero-frequency value of the sigma\_U2 propagator -[U'' + Pi(w)]\^{}-1\_11, whose pole is the sigma\_U2 mass  -- (m=0,lambda=1.05,mu=0): T\_sigma(0)/(-G\^{}P/2) = 10.8977; (m=0,lambda=0.5,mu=0.56): T\_sigma(0)/(-G\^{}P/2) = 9.3417; (m=0.1,lambda=1.05,mu=0): T\_sigma(0)/(-G\^{}P/2) = 12.7281 & PASS\\
R3-S1 & every symbol text is registered once (90 entries) and every name used in the data files of Revision\_3 belongs to one entry (88 names) & PASS\\
R3-S2 & the 13 taken entries carry exactly the meaning of the MASTER registry (113 entries of the main project, 229 of the revision on the deflating spacetimes compared), and none of the 77 new symbols is a symbol of either master registry & PASS\\
R3-S3 & every column of the 18 data files data/r3\_*.csv is a registered name or a descriptive word (88 different columns), and every file states its meaning; 9 column names also occur as names in a master registry or as symbols of the author's notebooks (A, M, m, mu, n, name, q, rank, x) - they cannot be confused, because every data file of Revision\_3 has the prefix r3\_ and a line '\# meaning:', and the name means in it the registered symbol of Revision\_3 & PASS\\
R3-S4 & each of the 4 documents of Revision\_3 (and the manuscript) contains the complete table of the 90 registered symbols, so that every symbol it uses is defined in the document itself & PASS\\
R3-S5 & in the 1554 formulas of the documents, lambda\^{}P, lambda\^{}S, G\^{}P, G\^{}S, G\^{}crit, mu, Lambda, Omega and Delta always carry the suffix U2 (Delta also the index 1 or 2 of its real and imaginary parts) & PASS\\
R3-S6 & in the 1910 formulas of the documents and of the new registry entries, no letter that a master registry defines with another meaning stands bare: every letter except the taken indices and sets (A, C, D, K, X, j, r), the new plain symbols (i, l, q) and the differential d carries the suffix of its Revision\_3 meaning, x occurs only in the coordinate names, and the Greek letters that are master symbols carry their suffix & PASS\\
R3-H1 & none of the 22 documents and document sources of Revision\_3 refers its reader to another document for an equation, a symbol or a definition & PASS\\
<!-- END checks_rows_tex -->

<!-- BEGIN nchecks_rust -->
55
<!-- END nchecks_rust -->

<!-- BEGIN nchecks_rust_pass -->
55
<!-- END nchecks_rust_pass -->

<!-- BEGIN gamma_kron_aligned -->
\gamma^{0}_{1671}&=-\mathsf Z\otimes\mathsf Z\otimes\mathsf Z\otimes\mathsf E\\
\gamma^{1}_{1671}&=-\mathsf Z\otimes\mathsf X\otimes\mathsf E\otimes\mathsf X\\
\gamma^{2}_{1671}&=\mathsf Z\otimes\mathsf X\otimes\mathsf E\otimes\mathsf Z\\
\gamma^{3}_{1671}&=-\mathsf Z\otimes\mathsf X\otimes\mathsf 1_2\otimes\mathsf E\\
\gamma^{4}_{1671}&=\mathsf X\otimes\mathsf 1_2\otimes\mathsf 1_2\otimes\mathsf 1_2\\
\gamma^{5}_{1671}&=\mathsf Z\otimes\mathsf Z\otimes\mathsf E\otimes\mathsf 1_2\\
\gamma^{6}_{1671}&=\mathsf Z\otimes\mathsf Z\otimes\mathsf X\otimes\mathsf E\\
\gamma^{7}_{1671}&=\mathsf Z\otimes\mathsf E\otimes\mathsf 1_2\otimes\mathsf 1_2
<!-- END gamma_kron_aligned -->

<!-- BEGIN gamma_kron_md -->
$$
\begin{aligned}
\gamma^{0}_{1671}&=-\mathsf Z\otimes\mathsf Z\otimes\mathsf Z\otimes\mathsf E\\
\gamma^{1}_{1671}&=-\mathsf Z\otimes\mathsf X\otimes\mathsf E\otimes\mathsf X\\
\gamma^{2}_{1671}&=\mathsf Z\otimes\mathsf X\otimes\mathsf E\otimes\mathsf Z\\
\gamma^{3}_{1671}&=-\mathsf Z\otimes\mathsf X\otimes\mathsf 1_2\otimes\mathsf E\\
\gamma^{4}_{1671}&=\mathsf X\otimes\mathsf 1_2\otimes\mathsf 1_2\otimes\mathsf 1_2\\
\gamma^{5}_{1671}&=\mathsf Z\otimes\mathsf Z\otimes\mathsf E\otimes\mathsf 1_2\\
\gamma^{6}_{1671}&=\mathsf Z\otimes\mathsf Z\otimes\mathsf X\otimes\mathsf E\\
\gamma^{7}_{1671}&=\mathsf Z\otimes\mathsf E\otimes\mathsf 1_2\otimes\mathsf 1_2
\end{aligned}
$$
<!-- END gamma_kron_md -->

<!-- BEGIN pairs_by_rank_table -->
| rank $r$ | products $\mathsf G_K$ | $C^+_{1671}\mathsf G_K$ antisymmetric | $C^-_{1671}\mathsf G_K$ antisymmetric |
|---|---|---|---|
| 0 | 1 | 0 | 0 |
| 1 | 8 | 0 | 8 |
| 2 | 28 | 28 | 28 |
| 3 | 56 | 56 | 0 |
| 4 | 70 | 0 | 0 |
| 5 | 56 | 0 | 56 |
| 6 | 28 | 28 | 28 |
| 7 | 8 | 8 | 0 |
| 8 | 1 | 0 | 0 |
<!-- END pairs_by_rank_table -->

<!-- BEGIN num_Gcrit_R7 -->
$8766.8$
<!-- END num_Gcrit_R7 -->

<!-- BEGIN num_Gcrit_R8 -->
$28054$
<!-- END num_Gcrit_R8 -->

<!-- BEGIN num_Gcrit_chiral_R7 -->
$17534$
<!-- END num_Gcrit_chiral_R7 -->

<!-- BEGIN num_Delta_vac_R7_l102 -->
$0.16479$
<!-- END num_Delta_vac_R7_l102 -->

<!-- BEGIN num_Delta_vac_R8_l102 -->
$0.11580$
<!-- END num_Delta_vac_R8_l102 -->

<!-- BEGIN num_Omega_vac_R7_l102 -->
$-2.9991\times10^{-8}$
<!-- END num_Omega_vac_R7_l102 -->

<!-- BEGIN num_Delta_vac_R7_l105 -->
$0.26370$
<!-- END num_Delta_vac_R7_l105 -->

<!-- BEGIN num_Delta_vac_R8_l105 -->
$0.18375$
<!-- END num_Delta_vac_R8_l105 -->

<!-- BEGIN num_Omega_vac_R7_l105 -->
$-1.8331\times10^{-7}$
<!-- END num_Omega_vac_R7_l105 -->

<!-- BEGIN num_Delta_vac_R7_l110 -->
$0.37960$
<!-- END num_Delta_vac_R7_l110 -->

<!-- BEGIN num_Delta_vac_R8_l110 -->
$0.26106$
<!-- END num_Delta_vac_R8_l110 -->

<!-- BEGIN num_Omega_vac_R7_l110 -->
$-7.0597\times10^{-7}$
<!-- END num_Omega_vac_R7_l110 -->

<!-- BEGIN num_BCS_slope_fit -->
$-3.5672\times10^{5}$
<!-- END num_BCS_slope_fit -->

<!-- BEGIN num_BCS_slope_pred -->
$-3.5672\times10^{5}$
<!-- END num_BCS_slope_pred -->

<!-- BEGIN num_mu_transition -->
$0.19500$
<!-- END num_mu_transition -->

<!-- BEGIN num_Delta_spectrum_vac -->
$0.26370$
<!-- END num_Delta_spectrum_vac -->

<!-- BEGIN num_Delta_spectrum_den -->
$0.14721$
<!-- END num_Delta_spectrum_den -->

<!-- BEGIN num_vac_l105_Delta -->
$0.26370$
<!-- END num_vac_l105_Delta -->

<!-- BEGIN num_vac_l105_fs2 -->
$6.6002\times10^{-6}$
<!-- END num_vac_l105_fs2 -->

<!-- BEGIN num_vac_l105_ft2 -->
$4.9843\times10^{-6}$
<!-- END num_vac_l105_ft2 -->

<!-- BEGIN num_vac_l105_vG -->
$1.1507$
<!-- END num_vac_l105_vG -->

<!-- BEGIN num_vac_l105_Delta_R8 -->
$0.18375$
<!-- END num_vac_l105_Delta_R8 -->

<!-- BEGIN num_vac_l105_f2_R8 -->
$1.5997\times10^{-6}$
<!-- END num_vac_l105_f2_R8 -->

<!-- BEGIN num_vac_l1005_vG -->
$1.1357$
<!-- END num_vac_l1005_vG -->

<!-- BEGIN num_den_mu054_Delta -->
$0.017824$
<!-- END num_den_mu054_Delta -->

<!-- BEGIN num_den_mu054_msigma_over_thr -->
$1.0000$
<!-- END num_den_mu054_msigma_over_thr -->

<!-- BEGIN num_den_mu054_fs2_over_n4mu -->
$1.0082$
<!-- END num_den_mu054_fs2_over_n4mu -->

<!-- BEGIN num_den_mu054_vG2 -->
$0.094247$
<!-- END num_den_mu054_vG2 -->

<!-- BEGIN num_Gcrit_R7_over_pi4 -->
$90.0000$
<!-- END num_Gcrit_R7_over_pi4 -->

<!-- BEGIN num_Gcrit_R8_over_pi4 -->
$288.000$
<!-- END num_Gcrit_R8_over_pi4 -->

<!-- BEGIN num_BCS_ratio -->
$1.00000$
<!-- END num_BCS_ratio -->

<!-- BEGIN num_vG2_times7 -->
$0.6597$
<!-- END num_vG2_times7 -->

<!-- BEGIN num_gap_ratio_l105 -->
$1.44$
<!-- END num_gap_ratio_l105 -->

<!-- BEGIN vacuum_gap_table -->
| $\lambda^P_{U2}$ | $\Delta_{U2}$ (R7) | Landau (R7) | $\Delta_{U2}$ (R8) | Landau (R8) | $\Omega^{\min}_{U2}$ (R7) |
|---|---|---|---|---|---|
| 1.02 | $0.1648$ | $0.1617$ | $0.1158$ | $0.1143$ | $-3.00\times10^{-8}$ |
| 1.04 | $0.2350$ | $0.2265$ | $0.1642$ | $0.1601$ | $-1.18\times10^{-7}$ |
| 1.06 | $0.2899$ | $0.2747$ | $0.2015$ | $0.1943$ | $-2.62\times10^{-7}$ |
| 1.10 | $0.3796$ | $0.3482$ | $0.2611$ | $0.2462$ | $-7.06\times10^{-7}$ |
| 1.20 | $0.5537$ | $0.4714$ | $0.3717$ | $0.3333$ | $-2.62\times10^{-6}$ |
| 1.50 | $0.9439$ | $0.6667$ | $0.5946$ | $0.4714$ | $-1.34\times10^{-5}$ |
| 2.00 | $1.473$ | $0.8165$ | $0.8487$ | $0.5774$ | $-4.07\times10^{-5}$ |
| 2.50 | $1.954$ | $0.8944$ | $1.044$ | $0.6325$ | $-7.38\times10^{-5}$ |
| 3.00 | $2.416$ | $0.9428$ | $1.209$ | $0.6667$ | $-1.10\times10^{-4}$ |
<!-- END vacuum_gap_table -->

<!-- BEGIN modes_vacuum_table -->
| $\lambda^P_{U2}$ | $\Delta_{U2}$ | $m_{\sigma U2}/2\Delta_{U2}$ | $(f^s_{U2})^2$ (R7) | $(f^t_{U2})^2$ (R7) | $f^s_{U2}/f^t_{U2}$ (R7) | $\Delta_{U2}$ (R8) | $f_{U2}^2$ (R8) |
|---|---|---|---|---|---|---|---|
| 1.005 | $0.08185$ | $1.000000$ | $7.250\times10^{-7}$ | $5.620\times10^{-7}$ | $1.136$ | $0.05778$ | $1.762\times10^{-7}$ |
| 1.010 | $0.1160$ | $1.000000$ | $1.434\times10^{-6}$ | $1.108\times10^{-6}$ | $1.137$ | $0.08177$ | $3.484\times10^{-7}$ |
| 1.020 | $0.1648$ | $1.000000$ | $2.807\times10^{-6}$ | $2.156\times10^{-6}$ | $1.141$ | $0.1158$ | $6.817\times10^{-7}$ |
| 1.050 | $0.2637$ | $1.000000$ | $6.600\times10^{-6}$ | $4.984\times10^{-6}$ | $1.151$ | $0.1837$ | $1.600\times10^{-6}$ |
| 1.100 | $0.3796$ | $1.000000$ | $1.202\times10^{-5}$ | $8.834\times10^{-6}$ | $1.166$ | $0.2611$ | $2.898\times10^{-6}$ |
| 1.200 | $0.5537$ | $1.000000$ | $2.032\times10^{-5}$ | $1.421\times10^{-5}$ | $1.196$ | $0.3717$ | $4.841\times10^{-6}$ |
<!-- END modes_vacuum_table -->

<!-- BEGIN num_fsft_first -->
$1.136$
<!-- END num_fsft_first -->

<!-- BEGIN num_fsft_last -->
$1.196$
<!-- END num_fsft_last -->

<!-- BEGIN num_lambda_modes_first -->
$1.005$
<!-- END num_lambda_modes_first -->

<!-- BEGIN num_lambda_modes_last -->
$1.20$
<!-- END num_lambda_modes_last -->

<!-- BEGIN modes_mass_table -->
| $m_{U2}$ | $\Delta_{U2}$ | threshold $2\sqrt{m_{U2}^2+\Delta_{U2}^2}$ | $m_{\sigma U2}$ | $m_{\sigma U2}/(2\Delta_{U2})$ | below threshold |
|---|---|---|---|---|---|
| 0 | $0.26370$ | $0.52739$ | $0.52739$ | $1.000000$ | at it |
| 0.0500 | $0.25891$ | $0.52739$ | $0.51782$ | $1.000000$ | yes |
| 0.100 | $0.24400$ | $0.52739$ | $0.48800$ | $1.000000$ | yes |
| 0.150 | $0.21688$ | $0.52739$ | $0.43375$ | $1.000000$ | yes |
| 0.200 | $0.17186$ | $0.52739$ | $0.34372$ | $1.000000$ | yes |
| 0.250 | $0.083877$ | $0.52739$ | $0.16775$ | $1.000000$ | yes |
<!-- END modes_mass_table -->

<!-- BEGIN modes_density_table -->
| $\mu_{U2}$ | $\Delta_{U2}$ | $m_{\sigma U2}/(2\Delta_{U2})$ | $(f^s_{U2})^2/(n_{U2}/4\mu_{U2})$ | $z_{U2}$ | $7(v^G_{U2})^2$ |
|---|---|---|---|---|---|
| 0.490 | $5.441\times10^{-4}$ | $1.000000$ | $1.0000$ | $0.0560$ | $0.9970$ |
| 0.500 | $0.001405$ | $1.000000$ | $1.0001$ | $0.123$ | $0.9858$ |
| 0.510 | $0.003141$ | $1.000000$ | $1.0004$ | $0.232$ | $0.9514$ |
| 0.520 | $0.006211$ | $1.000000$ | $1.0013$ | $0.389$ | $0.8771$ |
| 0.530 | $0.01103$ | $1.000000$ | $1.0036$ | $0.579$ | $0.7682$ |
| 0.540 | $0.01782$ | $1.000000$ | $1.0082$ | $0.777$ | $0.6597$ |
| 0.560 | $0.03662$ | $1.000000$ | $1.0238$ | $1.07$ | $0.5496$ |
| 0.580 | $0.05910$ | $1.000000$ | $1.0417$ | $1.15$ | $0.5724$ |
| 0.600 | $0.08176$ | $1.000000$ | $1.0567$ | $1.10$ | $0.6637$ |
<!-- END modes_density_table -->

<!-- BEGIN num_Tsigma_vac -->
$10.9$
<!-- END num_Tsigma_vac -->

<!-- BEGIN num_Tsigma_den -->
$9.34$
<!-- END num_Tsigma_den -->

<!-- BEGIN num_Tsigma_mass -->
$12.7$
<!-- END num_Tsigma_mass -->

<!-- BEGIN num_transition_M0 -->
$0.379601$
<!-- END num_transition_M0 -->

<!-- BEGIN num_transition_Dend -->
$0.56061$
<!-- END num_transition_Dend -->

<!-- BEGIN data_files_table -->
| data file | meaning (header line of the file) | columns |
|---|---|---|
| `r3_bcs_slope.csv` | weak-coupling (BCS) law at mu = 0.4, m = 0: ln Delta versus 1/G^P; predicted slope -1/(8 a7 q_F^5 mu) with q_F = mu | `lambda` `inv_G` `Delta` `lnDelta` |
| `r3_competition.csv` | vacuum Omega_R7(M, Delta) on a grid for (lambda^S, lambda^P) = (1.05, 1.1) [case 0], (1.1, 1.1) [case 1], (1.1, 1.05) [case 2], m = 0 | `case` `M` `Delta` `Omega` |
| `r3_density_scan.csv` | pair field, its log10 (below 1e-300 from the Cooper logarithm), number density and Omega_min - Omega_normal versus mu (m = 0, no scalar coupling) for lambda^P = 0.5, 0.8, 0.95, 1.05 (R7) | `mu` `D050` `lgD050` `n050` `O050` `D080` `lgD080` `n080` `O080` `D095` `lgD095` `n095` `O095` `D105` `lgD105` `n105` `O105` |
| `r3_fermi_surface_gap.csv` | quasi-particle energies near the Fermi surface for M = 0.3, mu = 0.5, Delta = 0.08 (units of the cutoff): normal \|eps - mu\|, spin-zero pair (C^+) E^-, pseudoscalar pair (C^-) smaller branch | `q` `normal` `E_Cplus` `E_Cminus` |
| `r3_gamma_kron.csv` | gamma^A_1671 = sign * f1 (x) f2 (x) f3 (x) f4 with the real 2x2 matrices 1, X = [[0,1],[1,0]], Z = [[1,0],[0,-1]], E = [[0,1],[-1,0]] | `A` `sign` `f1` `f2` `f3` `f4` |
| `r3_hat_2d.csv` | vacuum Omega_R7 as a function of the real and imaginary parts (Delta_1, Delta_2) of the pair field, lambda^P = 1.05, m = 0 | `Delta1` `Delta2` `Omega` |
| `r3_hat_radial.csv` | vacuum Omega_R7(Delta) - Omega_R7(0) (units Lambda^8) for lambda^P = 0.95, 1.0, 1.02, 1.05, 1.1 (m = 0, no scalar coupling) | `Delta` `l095` `l100` `l102` `l105` `l110` |
| `r3_modes_density.csv` | finite density (m = 0, lambda^P = 0.5, no scalar coupling, R7): Delta, threshold 2 Delta, sigma mass and kind, f_s^2, f_t^2 at fixed Delta, f_t^2 relaxed = (1/4) dn/dmu, n, n/(4 mu), f_s^2/(n/(4 mu)), Goldstone velocity squared f_s^2/f_t^2(relaxed), d ln Delta/d mu along the minimum, and Delta d ln Delta/d mu | `mu` `Delta` `threshold` `msigma` `kind` `fs2` `ft2_fixed` `ft2_relaxed` `n` `n_over_4mu` `fs2_over_n4mu` `vG2` `dlnDelta_dmu` `Delta_dlnDelta_dmu` |
| `r3_modes_mass.csv` | vacuum with a bare mass m (lambda^P = 1.05, mu = 0, no scalar coupling): pair field, threshold 2 sqrt(m^2 + Delta^2), sigma mass, kind (0 = bound state below threshold, 1 = at threshold) | `m` `Delta` `threshold` `msigma` `kind` |
| `r3_modes_vacuum.csv` | vacuum (mu = 0, m = 0): pair field, sigma mass in units of 2 Delta, spatial and time-like stiffness f_s^2, f_t^2 (units Lambda^6) in R7 and their ratio (the Goldstone velocity if U(1)_U2 were global), and the R8 pair field and its single (Lorentz-invariant) stiffness f^2 | `lambda` `Delta` `msigma_over_2Delta` `fs2` `ft2` `vG_R7` `Delta_R8` `f2_R8` |
| `r3_pairs_by_rank.csv` | number of ordered products G_K = gamma^A1 ... gamma^Ar (A1 < ... < Ar) of rank r for which C^+ G_K and C^- G_K are antisymmetric (a pair of one field exists) or symmetric (it vanishes) | `rank` `products` `antisym_Cplus` `antisym_Cminus` `sym_Cplus` `sym_Cminus` |
| `r3_phase_map.csv` | pair field at the minimum on the plane of lambda^P and mu (m = 0, no scalar coupling, R7) | `lambda` `mu` `Delta` |
| `r3_spectrum.csv` | quasi-particle energies (m = 0): vacuum normal q and vacuum superfluid sqrt(q^2+Delta^2) at lambda^P = 1.05; mu = 0.4, lambda^P = 0.8: normal \|q-mu\|, q+mu and superfluid E^-, E^+ (self-consistent Delta) | `q` `vac_normal` `vac_sf` `den_normal_minus` `den_normal_plus` `den_sf_minus` `den_sf_plus` |
| `r3_static_potential.csv` | static potential between two heavy members of the doublet, in units of g_U2^2 G7(r), G7(r) = Gamma(5/2)/(4 pi^(7/2) r^5), for (g'_U2/g_U2)^2 = 1, y_U2 = 1/2; x = m_B r | `x` `phi` `singlet_after` `triplet_after` `singlet_before` `triplet_before` |
| `r3_summary_modes.csv` | key numbers of the command `modes` (units Lambda_U2 = 1) | `name` `value` `meaning` |
| `r3_summary_potential.csv` | key numbers of the command `potential` (units Lambda_U2 = 1) | `name` `value` `meaning` |
| `r3_transition.csv` | density-driven breaking: lambda^S = 1.1, lambda^P = 1.05, m = 0 (R7): global minimum (M, Delta, Omega), the U(2)-symmetric branch (Delta = 0: M, Omega) and the density at the minimum, versus mu | `mu` `M` `Delta` `Omega` `M_sym` `Omega_sym` `n` |
| `r3_vacuum_gap.csv` | vacuum (mu = 0, m = 0, no scalar coupling): pair field at the minimum versus lambda^P = G^P/G^P_crit in R7 and R8, the Landau (small-gap) forms sqrt(4/3 (1-1/lambda)) and sqrt(2/3 (1-1/lambda)), the left-handed doublet (R7, same G^P_crit of R7 in lambda), and the minimum value of Omega | `lambda` `Delta_R7` `Delta_R8` `Landau_R7` `Landau_R8` `Delta_chiral_R7` `Omega_R7` `Omega_R8` |
<!-- END data_files_table -->

<!-- BEGIN n_rust_lines -->
2683
<!-- END n_rust_lines -->

<!-- BEGIN n_rust_files -->
9
<!-- END n_rust_files -->

<!-- BEGIN data_sha256 -->
af005f79587417c66507d8d6561bfe58322cb49107bdfff8ae15bc50d25f4db4  r3_algebra_checks.txt
8a84d1618db38ca3d3a36b75cc5108b41666505637475a6878b7d78e8cbf48ea  r3_bcs_slope.csv
7e543021c9cdeff80fb5a2b8fb68f1c244447adf64ddcdd84d73b057ab65c30d  r3_bdg_checks.txt
d01f084921f28bb10c1f3b0343b30a6b3656280a4a01abb0dba5fe27cc81c115  r3_competition.csv
6ee22e4473033582d03f4ea2d9e2476965aa31a6d7df6c7b9ae3a881417943d5  r3_density_scan.csv
4e3e21945ac9dd7856d93ee1367949e06420106738ed55985bda59dff4eab901  r3_fermi_surface_gap.csv
1bb9a53bafc9a05a956ed8a388bc6c3d88adad40dc54fbdc06acf7cd1544ccc1  r3_gamma_kron.csv
a6860c3679eef34d9e6b234b781ce231342ce904633023057f2ce8fb25e19f7c  r3_gauge_checks.txt
7dd499a9cb0e52c3ac59f4c0106cf926a5a41f149dadf00b0bf89f740d285e27  r3_hat_2d.csv
fdb482ebb92bac5ff055398fefca4ab609d01b31301922f36a30a7657c2b5ab0  r3_hat_radial.csv
52be8940d91a5681ebce32561355edd1ba5f9ee2c2ec724dc1277ea78b277653  r3_modes_checks.txt
db72768d2f9c8319e7b5788a5ccb9602373543213cb2676eab52b31d80442385  r3_modes_density.csv
a2531b0a350807ad0f60ce475c05dc23e0b802e5d67b162dbb0afa42827d549b  r3_modes_mass.csv
ef7a37878a48b5e2f6a6c8242e2d658547c1822170b5941327980168f690970f  r3_modes_vacuum.csv
40a2be87ab67e07130b35bea5ac80e90a675b8806cb09984fd1da6804b04d3b9  r3_pairs_by_rank.csv
04a8faa22a6476110e3843387fd8deb68e9e28c53d9191bb0a4fa0ed6abefc9f  r3_phase_map.csv
5d863e8866113e7dc5263f13c73197fe20c01c4498b090acd3d4b18b78251b7a  r3_potential_checks.txt
693e64475de2ef4d38c27c5229fee9901daa8d9aeacaa7f3c6675615ae5c7003  r3_self_contained_checks.txt
038f5aea5d4e3299c6187da715525215629f59f6efa25a7977bd98b76d9ee827  r3_spectrum.csv
59869edf7548a25f05aadab58dbba0ea0784cba1e24d6efc90a5d0afc91f3add  r3_static_potential.csv
16947430fdd4e3d49fa8e56e03afd80001742c2f6d38868038f3eb11049d5b60  r3_summary_modes.csv
6303920b9f5e2b4eb8bc6d395d764f00f2351a32059c44b8baf95fcbfa9f3165  r3_summary_potential.csv
a521ce986f27ccf4d345093f146a7e14f7c2df301751074f4054edae0ad396d9  r3_symbol_checks.txt
ef8e1845cc4c15a7739100ec0ae14bc31e3eb6e686cb086c901d641e895edf2c  r3_transition.csv
a9fbf1142cd50b9cd0c64bf2187971a8f9024007b6423014f6679d0ffa3dc74e  r3_vacuum_gap.csv
<!-- END data_sha256 -->

<!-- BEGIN figure_sha256 -->
2e1335c1a027d789818c8e65628e08e447c12d23cf92815d90726ee0f20b32f9  fig_r3_competition.pdf
39c7ddbd4422f5df4a9387ca7947f529aa7851605dcdbd3ae2fc0c805063551c  fig_r3_density.pdf
855b133ba987423d91b40ae38f9ab39722fe113c2a35aaffa782953fe8e3c57c  fig_r3_gap.pdf
b23679da4e6797d4e73e18111735969383a88bdb952db4c4600868c1a3155c1d  fig_r3_hat.pdf
3b3ba2ee52aab7f762e190830eeb43d7eacdbc1bceed0130a9716a554da120cc  fig_r3_modes.pdf
3660c4c44a93b18186913f5fd1d5e450beb10b39ff215b3ef0bab75e08d89504  fig_r3_phase.pdf
c21ddef72f72185ec05c7763ae897f656d41a2305523b812c39676ae672e31fc  fig_r3_spectrum.pdf
8b1fbe0ff043d430a87f41f938b0cada8fd5beb3f6ec2c438a108589d36a46fe  fig_r3_static.pdf
1242352556da0901d24387415d172ea4055653ce6446dcc1104cf4efb46b5e2a  fig_r3_transition.pdf
<!-- END figure_sha256 -->

<!-- BEGIN code_sha256 -->
ebce0cae8681e0f7f1924267829e07bf8a06a8cc5e0301f9b5e507c90fec8aa7  code/dark-u2-rs/src/algebra.rs
644562f6722a71eee2f0416e9c8038e5576b2cd84fa0a29bbc0c4708b0ec0313  code/dark-u2-rs/src/bdg.rs
343f72da16ef057dbf19bdf1c9738f99ca31eaa27abfa83ecfde22b3a100ba75  code/dark-u2-rs/src/checks.rs
c97b952760ba341e764cf30f4cde279fe3854bbae21cf2bf5e73223924708da3  code/dark-u2-rs/src/gauge.rs
651efab2b48a6f53e4eca1e3a1e5d4731a19d1d91bd9feeceb12362382406f1a  code/dark-u2-rs/src/integrate.rs
1e88fc6952360824bd6a491f78816d4aa1434b174a9752ac6fcb83ba6f02c3bd  code/dark-u2-rs/src/main.rs
ab6a4bad6c5140de37d7ab0e0b6557a436a4de0816198c7bbb2a99200edb50b8  code/dark-u2-rs/src/modes.rs
384aec14273a35c63398ff89bc1850abff3ce04af2d8c3b23903811e13d025f6  code/dark-u2-rs/src/potential.rs
9f19d5afaeb8cff6454da5c3a098fc74c8957498f51b0bfa2e503f31bbfb9191  code/dark-u2-rs/src/rsym.rs
0a6bee389a7858f831b41a2f67601aec7c8445aac568dafabee4e8394a89e15c  code/scripts/assemble_docs.py
27004018376dccf914fe48f18f8af84f8f57cad412370580a6d78d4be07005ea  code/scripts/build_docs.sh
cd4544027125a16d32f5dcd86183fe070f1738b786a6a346c143a6ba237b9293  code/scripts/build_manuscript.py
f927120ebaf6390160ac371fcaf68e9f059012df5a6700a071521adb13bdafbe  code/scripts/check_all.sh
465063731c88f51a1d8af013d40f1cae7613007e63d3256e5ab26b364bd3b455  code/scripts/check_layout.py
fd4648f2688fbf6b31e875c2b0828b6ee157ad605b75fe1956e87274489bdffa  code/scripts/check_self_contained.py
0ecce2f9c6b06bb02f7c1e0e132f5b6eb7fec9f98cc55300c91ec37bcec6e6b7  code/scripts/check_symbols.py
ac40294235d8cff5d08bd6145121bbd12f4f584f22999841a699ec0a2a988c08  code/scripts/make_figures.py
60ded4a97e357119ec32d67e04526dc59ec228677c1ca4a1980cae92a4b93073  code/scripts/tex2md.py
75fd9d766f0e6e24c4afd4f2877d3450ca03c68c66df6a58d7b2730c8a28cf97  code/scripts/write_doc_tables.py
c826a214995e506e647843e68e1d31df9d475aea278ca1a411953eaa08c74169  code/scripts/write_symbol_registry.py
<!-- END code_sha256 -->
