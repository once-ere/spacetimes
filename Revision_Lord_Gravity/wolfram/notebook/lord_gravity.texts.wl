(* ::Package:: *)

(* ::Title:: *)
(*Lord's gauge theory of gravitation and the brochon: the notebook of the checks*)

(* ::Subtitle:: *)
(*PROJECT spacetimes, folder Revision_Lord_Gravity. The notebook lord_gravity.nb, prepared for Patrick Lee Nash, Ph.D.*)

(* ::Text:: *)
(***What this notebook is.** The folder `Revision_Lord_Gravity` holds a manuscript, `lord_gravity.md`, that reads E. A. Lord's paper "General relativity from gauge invariance" (Proceedings of the Cambridge Philosophical Society 69, 423 (1971)), relates his gauge group to the group $SL(2,\mathbb C)$ of the project, couples the spinor field of the project, the brochon $\Psi_{\rm LL}$, to gravitation, and solves its equations around black holes. Its statements are checked by programs in three languages: Rust, Python and the Wolfram Language. The nine scripts in the Wolfram Language make 46 checks, most of them in exact arithmetic. This notebook runs all of them, in the order of the manuscript, and explains before each check what it decides and why it matters. A student who works through it sees every algebraic identity of the manuscript being verified, and can change the inputs and watch what happens.*)

(* ::Text:: *)
(***How to use it.** Open the notebook in Wolfram (version 15 or later; it was evaluated with Wolfram 15.0.1) and choose Evaluation > Evaluate Notebook. The cells are evaluated in order; the whole notebook takes about a minute. Nothing outside the notebook is read: the three data files that some checks compare with are written into closed cells (a closed cell shows only a thin bracket; double-click the bracket to open it). Every check prints one line that begins with PASS or FAIL, and the last chapter lists all checks.*)

(* ::Text:: *)
(***How to read it.** A text cell explains, in words and formulas, what the next input cell computes. The formulas in the texts use the symbols of the manuscript; the code uses short names of its own, which the comments in the code and the texts name (for example `sig` for the four matrices $\sigma^{\rm LL}_\mu$). A symbol with the suffix LL is an object of the Lorentz super-loop and of its fields, SF an object of the brochon model and of its superfluid ground state, LG an object that this folder introduces (the gravitational field, the black holes and their problems), and Lord an object of Lord's paper in his own notation.*)

(* ::Text:: *)
(***The checks and their counterparts.** Each check has an identifier: WL for Lord's paper, WG for the curved space-time, WS for the Schwarzschild field, WV for the Vaidya field, WP, WC, WT and WA for the four computations of the pair and the black holes, WW for the worked examples. The line that a check prints names, in parentheses, the checks of the Rust program and of the Python program of the folder that test the same statement in floating point, so that a reader without a Wolfram license can follow every step with those programs (folders `rust/` and `python/` of the folder). The notebook runs the code of the scripts in `wolfram/source/` unchanged; each chapter runs in a context of its own (for example ``LGNB`Lord` ``), so that the chapters, which were written as separate programs, cannot change each other's symbols.*)

(* ::Text:: *)
(***What the notebook does not repeat.** The long numerical computations of the manuscript (the evolutions in time of Parts V and VI, which take minutes to forty minutes) are done by the Rust programs with the integrator CVODE of SUNDIALS; they are not repeated here. The notebook repeats only those numbers that a Wolfram script recomputes independently: three quasi-bound states and three greybody factors of Part IV (checks WS-7 and WS-8), found again in arithmetic of 32 digits.*)

(* ::Chapter:: *)
(*0. The register of the checks*)

(* ::Text:: *)
(*The next cell defines the functions that record and print a check. `check[id, text, ok, detail]` records a check that is decided by facts, such as an exact identity: it passes when `ok` is `True`, and `detail` states the facts. `checkDev[id, text, dev, tol]` records a numerical check: it passes when the measured deviation `dev` is at most the tolerance `tol`. Both append an association to the list ``LGNB`Test`$Results``, the register of all checks, and print the same line that the scripts print into their reports (the files `data/hck_lg_*_checks_wolfram.txt` of the folder). The function `fmt` writes a number with four significant digits in the form `2.857e-8`.*)

(* ::Input:: *)
(* the register of the checks of this notebook: one association per check, in the order of evaluation *)
LGNB`Test`$Results = {};
(* fmt[x, nd]: a real number written d.ddde-x with nd significant digits, as the reports of the scripts write it (the
   function fmt of the script lg_schw.wls, with other names for its variables, so that no common name such as x is
   created in the context Global`) *)
fmt[lgX_, lgNd_ : 4] := Module[{lgE, lgMm, lgY = N[lgX]},
  If[lgY == 0, "0", lgE = Floor[Log10[Abs[lgY]]]; lgMm = Round[Abs[lgY] 10^(lgNd - 1 - lgE)];
   If[lgMm >= 10^lgNd, lgMm = Round[lgMm/10]; lgE = lgE + 1];
   StringJoin[If[lgY < 0, "-", ""], ToString[Quotient[lgMm, 10^(lgNd - 1)]], ".",
    IntegerString[Mod[lgMm, 10^(lgNd - 1)], 10, lgNd - 1], "e", ToString[lgE]]]];
(* checkHook[]: called after every check; it does nothing here (the dry run of the code without the front end uses it
   to print the computing time of each check) *)
checkHook[] := Null;
(* check[id, text, ok, detail]: a check decided by facts; it passes when ok is True *)
check[lgId_String, lgText_String, lgOk_, lgDetail_String] := Module[{lgPass = TrueQ[lgOk], lgLine},
  lgLine = StringJoin[If[lgPass, "PASS ", "FAIL "], StringPadRight[lgId, 9], lgText, " [", lgDetail, "]"];
  AppendTo[LGNB`Test`$Results, <|"id" -> lgId, "pass" -> lgPass, "deviation" -> Missing["NotNumerical"],
     "tolerance" -> Missing["NotNumerical"], "line" -> lgLine|>];
  Print[lgLine]; checkHook[]; lgPass];
(* checkDev[id, text, dev, tol]: a numerical check; it passes when the deviation dev is at most the tolerance tol *)
checkDev[lgId_String, lgText_String, lgDev_, lgTol_] := Module[{lgPass = TrueQ[lgDev <= lgTol], lgLine},
  lgLine = StringJoin[If[lgPass, "PASS ", "FAIL "], StringPadRight[lgId, 9], "deviation ", fmt[lgDev], " (tolerance ",
    fmt[lgTol, 2], "): ", lgText];
  AppendTo[LGNB`Test`$Results, <|"id" -> lgId, "pass" -> lgPass, "deviation" -> N[lgDev], "tolerance" -> N[lgTol],
     "line" -> lgLine|>];
  Print[lgLine]; checkHook[]; lgPass];

(* ::Chapter:: *)
(*1. Lord's paper and the association (Parts I and II of the manuscript)*)

(* ::Text:: *)
(*Lord writes a four-vector $v^\mu$ as the $2\times2$ Hermitian matrix $v^\mu\sigma^{\rm LL}_\mu$, where $\sigma^{\rm LL}_0$ is the unit matrix and $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3$ are the Pauli matrices. Its determinant is the Minkowski square $v_\mu v^\mu$. His central observation is that the four matrices need not be the Pauli matrices: any four Hermitian matrices whose products obey the same rules will do, and changing from one such set to another is a change of representation. Requiring that the physics does not depend on this choice, at every point separately, is a gauge principle, and its gauge field turns out to be gravitation. This chapter checks, in exact arithmetic, the identities of his Section 1 (the algebra of the matrices) and the signs and factors of his Section 4 (the gauge transformation), where several printed formulas do not follow from his definitions; three of them are checked here.*)

(* ::Text:: *)
(*The **bar** of a $2\times2$ matrix is $\bar X=-\mathsf E_{\rm LL}X^T\mathsf E_{\rm LL}$, with $\mathsf E_{\rm LL}=\begin{pmatrix}0&1\\-1&0\end{pmatrix}$ (Lord's $\epsilon$); for an invertible matrix it is $\det X$ times $X^{-1}$. The products $\sigma^{\mu\nu}_{\rm LG}=\tfrac12(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu-\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu)$ generate the Lorentz transformations on two-component spinors, and the Dirac matrices are the $4\times4$ matrices $\gamma^\mu=\begin{pmatrix}0&\sigma^{\rm LL}_\mu\\\bar\sigma^{\rm LL}_\mu&0\end{pmatrix}$ (the Weyl form). In the code: `sig` for the four $\sigma^{\rm LL}_\mu$, `eps` for $\mathsf E_{\rm LL}$, `bar` for the bar, `smn` for $\sigma^{\mu\nu}_{\rm LG}$, `gam` and `gm` for the Dirac matrices, `g5` for Lord's $\gamma^5$, `cc` for his charge-conjugation matrix $C$. The indices 1 to 4 of the code are the indices 0 to 3 of the texts.*)

(* @@PART lg_lord.wls|Lord@@ *)

(* ::Text:: *)
(*The helper function `zeroQ[m]` decides whether every entry of a matrix (or a list of matrices) simplifies to zero.*)

(* @@PROLOGUE lg_lord.wls@@ *)

(* ::Text:: *)
(*Lord's matrices, exactly as he defines them.*)

(* @@SECTION lg_lord.wls|Lord's matrices@@ *)

(* ::Section:: *)
(*WL-1: the identities of Lord's Section 1*)

(* ::Text:: *)
(*Lord's Section 1 lists the rules that his matrices obey: the bar of $\sigma^{\rm LL}_\mu$ (1.2); the relation $\tfrac12(\sigma^{\rm LL}_\mu\bar\sigma^{\rm LL}_\nu+\sigma^{\rm LL}_\nu\bar\sigma^{\rm LL}_\mu)=\eta^{\rm LL}_{\mu\nu}\mathsf 1_2$ (1.3), which says that the matrices carry the Minkowski metric; the commutators of the generators $\sigma^{\mu\nu}_{\rm LG}$ (1.5), which are those of the Lorentz algebra; their self-duality (1.7); the Clifford relation $\tfrac12\{\gamma^\mu,\gamma^\nu\}=\eta^{\rm LL}_{\mu\nu}\mathsf 1_4$ (1.10); the properties of $\gamma^5$ and of the charge-conjugation matrix (1.11) to (1.14); and the block form of $\gamma^{\mu\nu}$ (1.15) and its duality (1.17). The cell evaluates every one of them for all index combinations (for (1.5) these are $4^4=256$ combinations), in exact arithmetic. These are the rules that every later computation uses, so a single wrong sign here would spoil everything after it.*)

(* @@SECTION lg_lord.wls|WL-1@@ *)

(* ::Section:: *)
(*WL-2 and WL-3: matrices of determinant 1 and the polarized determinant*)

(* ::Text:: *)
(*A change of representation is a $2\times2$ matrix $S_{\rm Lord}$ of determinant 1. WL-2 checks, for a matrix of determinant 1 with symbolic entries, the two rules that make the bar behave well under such a change: $\mathsf E_{\rm LL}S=(S^T)^{-1}\mathsf E_{\rm LL}$, and $\overline{S\sigma S'}=S'^{-1}\bar\sigma S^{-1}$ for any matrix $\sigma$ and a second matrix $S'$ of determinant 1; with $S'=S^\dagger$ this is Lord's (1.23).*)

(* ::Text:: *)
(*WL-3 checks the identity behind the word "Minkowski product of two matrices": for any two $2\times2$ matrices $A$ and $B$, $\tfrac12(A\bar B+B\bar A)=N_{\rm LG}(A,B)\,\mathsf 1_2$, where $N_{\rm LG}(A,B)=\tfrac12(\det(A+B)-\det A-\det B)$ is the polarized determinant. So a set of four Hermitian matrices satisfies Lord's (1.3) exactly when their polarized determinants are $\eta^{\rm LL}_{\mu\nu}$; this is how the manuscript recognizes a valid set (an orthonormal frame) by a simple computation.*)

(* @@SECTION lg_lord.wls|WL-2@@ *)

(* @@SECTION lg_lord.wls|WL-3@@ *)

(* ::Section:: *)
(*WL-4: an infinitesimal Lorentz transformation*)

(* ::Text:: *)
(*Lord's (1.8) says that the spinor matrix $L=\mathsf 1_2+\tfrac14\lambda_{\mu\nu}\sigma^{\mu\nu}_{\rm LG}$ and the Lorentz matrix $\Lambda_\mu{}^\nu=\delta_\mu^\nu+\eta^{\nu\rho}\lambda_{\rho\mu}$ belong together: transforming the matrices $\sigma^{\rm LL}_\mu$ by $L$ is the same as transforming their index by $\Lambda$. The check takes the six parameters $\lambda_{\mu\nu}$ as real symbols and verifies the relation to first order in them, where it is a linear identity.*)

(* @@SECTION lg_lord.wls|WL-4@@ *)

(* ::Section:: *)
(*WL-5: the sign of the gauge transformation (Lord's Section 4)*)

(* ::Text:: *)
(*When the change of representation $S(x)$ depends on the point, the derivative of a spinor is no longer covariant, and Lord adds an auxiliary field $\Gamma$, as in electrodynamics: $D=\partial-\Gamma$. For $D\psi$ to transform like $\psi$, the field must transform as $\Gamma'=S\Gamma S^{-1}+(\partial S)S^{-1}$. Lord prints the inhomogeneous term as $S\,\partial(S^{-1})$, which is its negative. The check takes an explicit $S(t)$ of determinant 1, a constant $\Gamma$ in the Lorentz algebra with symbolic entries, and a spinor of four symbolic functions, and shows that the corrected law makes $D\psi$ covariant and the printed one does not. Section 7 of the manuscript records this correction and those of the next two checks.*)

(* @@SECTION lg_lord.wls|WL-5@@ *)

(* ::Section:: *)
(*WL-6: the factor 2 of the connection of the vectors*)

(* ::Text:: *)
(*A spinor connection $\Gamma$ induces a connection $C$ on vectors through the commutator with the Dirac matrices: $[\Gamma,\gamma_b]$ is a combination of the $\gamma_e$, and its coefficients are $C$. In Lord's special gauge (constant Dirac matrices) with $\Gamma=\tfrac12g\,a_{bc}\gamma^{bc}$, the check computes the coefficients for six symbolic $a_{bc}$ and finds $C=2g\,a$, twice the value that Lord prints in (4.20) and (5.2).*)

(* @@SECTION lg_lord.wls|WL-6@@ *)

(* ::Section:: *)
(*WL-7: the curvature of the spinor connection*)

(* ::Text:: *)
(*The map $\lambda(W)=\tfrac14W_{ab}\gamma^{ab}$ takes an element of the Lorentz algebra (an antisymmetric $W_{ab}$) to a $4\times4$ matrix that acts on Dirac spinors. The check shows, for symbolic $W$, that it acts on the Dirac matrices as the Lorentz algebra acts on vectors, and that it preserves commutators: $[\lambda(W_1),\lambda(W_2)]=\lambda([W_1,W_2])$, a homomorphism of Lie algebras. It follows that the curvature of the spinor connection is $\tfrac14R_{ab\mu\nu}\gamma^{ab}$, where $R$ is the Riemann tensor; Lord prints $\tfrac12R\gamma$, twice this. His two factors 2 (this one and that of WL-6) compensate each other only to first order in the fields.*)

(* @@SECTION lg_lord.wls|WL-7@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*2. The brochon in a curved space-time (Part III)*)

(* ::Text:: *)
(*Part III turns Lord's reference vectors into the tetrad of a curved space-time: four orthonormal vectors $e^{\rm LG}_a{}^\mu$ at each point, from which the metric $g^{\rm LG}_{\mu\nu}=e^{{\rm LG}\,a}{}_\mu\eta^{\rm LL}_{ab}e^{{\rm LG}\,b}{}_\nu$ follows. The spin connection $\omega^{\rm LG}_{\mu\,ab}$ is fixed by requiring that the tetrad be covariantly constant (the tetrad postulate), and the spinor connection is $\Gamma^{\rm LL}_\mu=\tfrac14\omega^{\rm LG}_{\mu\,ab}\gamma^{ab}_{\rm LL}$. With it the brochon obeys the Dirac equation $i\gamma^\mu_{\rm LG}(\partial_\mu+\Gamma^{\rm LL}_\mu)\Psi_{\rm LL}=m_{\rm LL}\Psi_{\rm LL}$ in any gravitational field. This chapter checks the geometry for the space-times that the later parts use, the square of the Dirac operator, the energy-momentum tensor of the brochon, and a property of the pairing term of the superfluid.*)

(* @@PART lg_curved.wls|Curved@@ *)

(* ::Text:: *)
(*Four helper functions: `zeroQ` (every entry simplifies to zero), `finiteQ` (no infinity in an expression), `zeroE` (every entry expands to zero, which decides identities between polynomials in symbolic functions quickly), and `witnessQ` (an expression is not identically zero, shown by its value for explicit functions at one rational point; it is used for the controls, the statements that a wrong formula fails).*)

(* @@PROLOGUE lg_curved.wls@@ *)

(* ::Text:: *)
(*The Dirac matrices of the chiral basis (the Weyl form), `gam`, the metric `eta` and the products `gab` $=\gamma^{ab}$.*)

(* @@SECTION lg_curved.wls|the matrices@@ *)

(* ::Text:: *)
(*The function `geometry[E, X, simp]` takes a coframe, the matrix `E` with entries $e^{{\rm LG}\,a}{}_\mu$ in the coordinates `X`, and returns the tetrad (`F`, the inverse transpose), the metric, the Christoffel symbols, the spin connection of the tetrad postulate, the Dirac matrices $\gamma^\mu_{\rm LG}=e^{\rm LG}_a{}^\mu\gamma^a$ and the spinor connection $\tfrac14\omega_{\mu\,ab}\gamma^{ab}$. The functions `riemann`, `ricci` and `christoffel` compute the curvature in the conventions of Landau and Lifshitz; `nablaGamma` is the covariant derivative of the Dirac matrices, which must vanish, and `spinCurvDev` is the difference between the curvature of the spinor connection and $\tfrac14R_{ab\mu\nu}\gamma^{ab}$, which must vanish too. Then the assumptions $r>2M$, $M>0$, $0<\theta<\pi$ are set for `Simplify` (they hold in this chapter only).*)

(* @@SECTION lg_curved.wls|the geometry of a coframe@@ *)

(* ::Section:: *)
(*WG-1: the Schwarzschild space-time in the static tetrad*)

(* ::Text:: *)
(*The Schwarzschild black hole of the mass $M_{\rm LG}$ has the metric $f_{\rm LG}\,dt^2-f_{\rm LG}^{-1}dr^2-r^2d\Omega^2$ with $f_{\rm LG}=1-2\mathcal M_{\rm LG}/r$, where $\mathcal M_{\rm LG}=G_{\rm N}M_{\rm LG}$ is the mass as a length (units $c=1$; in the code $G_{\rm N}=1$, the mass `M` and `fS` for $f_{\rm LG}$). The static tetrad is the diagonal one, $e^0=f_{\rm LG}^{1/2}dt$, $e^1=f_{\rm LG}^{-1/2}dr$, $e^2=r\,d\theta$, $e^3=r\sin\theta\,d\varphi$. The check computes from it: the Ricci tensor (zero: the space-time is empty), the Kretschmann invariant $R_{\mu\nu\rho\sigma}R^{\mu\nu\rho\sigma}=48\mathcal M_{\rm LG}^2/r^6$ (finite at the horizon $r=2\mathcal M_{\rm LG}$, infinite at $r=0$: the horizon is not a singularity of the geometry, the centre is), the spinor connection $\Gamma^{\rm LL}_\mu$ in closed form, the covariant constancy of the Dirac matrices, and the curvature of the spinor connection, $\tfrac14R^{\rm LG}_{ab\mu\nu}\gamma^{ab}_{\rm LL}$. The last is checked for every static, spherically symmetric tetrad with an arbitrary function $h_{\rm LG}(r)$ in place of $f_{\rm LG}^{1/2}$, which avoids square roots that `Simplify` does not always remove.*)

(* @@SECTION lg_curved.wls|WG-1@@ *)

(* ::Section:: *)
(*WG-2: the freely falling observers (Painleve and Gullstrand)*)

(* ::Text:: *)
(*The static tetrad is singular at the horizon, because a static observer cannot exist there. The observers who fall freely from rest at infinity have a tetrad that is regular across the horizon: with their proper time $\tau_{\rm LG}$ as the time coordinate, $e^0=d\tau_{\rm LG}$, $e^1=dr+w_{\rm LG}\,d\tau_{\rm LG}$ with $w_{\rm LG}=(2\mathcal M_{\rm LG}/r)^{1/2}$, their speed (in the code `tT` and `vPG`). The check shows that this is the same space-time (the metric is Schwarzschild's after the change $\tau_{\rm LG}=t+\int w_{\rm LG}/f_{\rm LG}\,dr$), computes the spinor connection in closed form, and shows that the tetrad, the Christoffel symbols and the spinor connection are finite at $r=2\mathcal M_{\rm LG}$. Part IV integrates the radial equations of the brochon in this tetrad across the horizon.*)

(* @@SECTION lg_curved.wls|WG-2@@ *)

(* ::Section:: *)
(*WG-3: the Vaidya space-times (a black hole whose mass changes)*)

(* ::Text:: *)
(*If radiation falls into a black hole, its mass grows; if it radiates, its mass shrinks. The Vaidya metric describes this with a mass that depends on the advanced time $v_{\rm LG}$ (for infalling radiation) or on the retarded time $u_{\rm LG}$ (for outgoing radiation); in the code they are `vv` and `uu`, and the mass functions `mIn` and `mOut`. The check computes Einstein's tensor and finds that it is $8\pi$ times the energy-momentum tensor of a pressureless stream of radiation moving at the speed of light (null dust), with the energy density $\rho_{\rm LG}=\mathcal M_{\rm LG}'(v_{\rm LG})/(4\pi r^2)$, positive when the mass grows, or $-\mathcal M_{\rm LG}'(u_{\rm LG})/(4\pi r^2)$, positive when the mass shrinks; the scalar curvature vanishes. Part V uses the ingoing form.*)

(* @@SECTION lg_curved.wls|WG-3@@ *)

(* ::Section:: *)
(*WG-4: conformal invariance and the square of the Dirac operator*)

(* ::Text:: *)
(*In a space-time whose metric is a multiple of the flat one, $g=\Omega_{\rm LG}^2\eta^{\rm LL}$ with $\Omega_{\rm LG}=e^{\phi(x)}$ and $\phi$ an arbitrary symbolic function, the check verifies two statements for a spinor of four symbolic functions. First, the massless Dirac operator $\gamma^\mu_{\rm LG}\nabla_\mu$ (with $\nabla_\mu=\partial_\mu+\Gamma^{\rm LL}_\mu$) is conformally invariant: $\gamma^\mu_{\rm LG}\nabla_\mu(\Omega_{\rm LG}^{-3/2}\psi)=\Omega_{\rm LG}^{-5/2}\gamma^a\partial_a\psi$, so the massless brochon does not feel such a gravitational field except through a rescaling. Second, the square of the Dirac operator is $(\gamma^\mu_{\rm LG}\nabla_\mu)^2\psi=g^{\mu\nu}_{\rm LG}\nabla_\mu\nabla_\nu\psi-\tfrac14R\psi$ (the formula of Lichnerowicz), with the coefficient $\tfrac14$ of the scalar curvature $R$, not Lord's $\tfrac12$ (the control shows that $\tfrac12$ fails, and that $R$ is not zero here, so the control means something). This coefficient is remark R1 of the manuscript.*)

(* @@SECTION lg_curved.wls|WG-4@@ *)

(* ::Section:: *)
(*WG-5: the energy-momentum tensor from the variation of the tetrad*)

(* ::Text:: *)
(*In general relativity the source of gravitation is the energy-momentum tensor, defined as the response of the action to a change of the metric. For a spinor the action depends on the tetrad, not only on the metric, and the response to the tetrad has a symmetric and an antisymmetric part. The check perturbs the flat tetrad by sixteen symbolic functions, $e_a{}^\mu=\delta+\varepsilon h_{a\mu}$, varies the Dirac action to first order in $\varepsilon$ with the package `VariationalMethods`, and shows: the symmetric part is exactly the tensor $T_{\mu\nu}$ of the manuscript (16.2), for any field; the antisymmetric part is not zero for an arbitrary field, but vanishes for a solution of the Dirac equation (a superposition of two plane waves of mass 3). So on solutions the tensor is symmetric, as Einstein's equations require.*)

(* @@SECTION lg_curved.wls|WG-5@@ *)

(* ::Section:: *)
(*WG-6: the pairing term and the zeugon*)

(* ::Text:: *)
(*In the superfluid ground state of the brochon model the field equation contains a pairing term, which mixes the brochon with its complex conjugate: $\mathcal C_{\rm LG}\Psi=\mathsf P_{\rm LG}\Psi^{*}$ with the matrix $\mathsf P_{\rm LG}=(\gamma^0_{\rm LL}\otimes\mathsf 1_2)(\mathsf K^3_{\rm LL})^\dagger$. For the quasi-particles to move like Dirac particles in a gravitational field (Theorem L2), this map must commute with the Dirac matrices and with every Lorentz transformation, and with the charge of the zeugon. The check shows exactly that: $\mathsf P\,\overline{\gamma^a}=\gamma^a\mathsf P$, $\mathsf P\,\overline{\mathsf P}=-1$, $\mathsf P\,\overline{X}=X\mathsf P$ for every element $X=\tfrac14w^{\rm LG}_{ab}\gamma^{ab}$ of the Lorentz algebra, and $\mathsf P\,\overline{iT^3}=iT^3\mathsf P$ for the generator $T^3_{\rm LL}$ of the zeugon, but not for $T^1$, $T^2$ or the fermion number (the bars here are complex conjugates). This is the algebraic reason why the zeugon, the gauge boson of $T^3_{\rm LL}$, stays massless in the superfluid while the other gauge bosons do not, and why the quasi-particles carry its charge (Theorem L2).*)

(* @@SECTION lg_curved.wls|WG-6@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*3. The brochon in the Schwarzschild field (Part IV)*)

(* ::Text:: *)
(*Part IV solves the Dirac equation of a particle of the mass $m_{\rm LL}$ (one isospin component of the brochon) around a Schwarzschild black hole of the mass $\mathcal M_{\rm LG}=1$ (the horizon at $r=2$; units $G_{\rm N}=c=\hbar=1$), with $f_{\rm LG}=1-2/r$ and $w_{\rm LG}=(2/r)^{1/2}$. Because the black hole is spherical, the equation separates. The angular dependence is that of the hydrogen atom: the angular functions $\alpha_{\rm LG}(\theta)$, $\beta_{\rm LG}(\theta)$ obey (18.4), with the total angular momentum $j_{\rm LG}$, its component $m_{j,{\rm LG}}$ and the separation constant $\lambda_{\rm LG}=\pm(j_{\rm LG}+\tfrac12)$. Two radial functions remain. In the static tetrad they are $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$ and obey (18.7); in the freely falling tetrad they are $\mathcal P_{\rm LG}$, $\mathcal Q_{\rm LG}$ and obey (18.5), which is regular at the horizon:*)

(* ::DisplayFormula:: *)
(*P'=\frac{(i\omega-W/(4r))P-(i\lambda/r+m)Q}{1-W},\qquad Q'=\frac{(i\lambda/r-m)P+(-i\omega+W/(4r))Q}{1+W}.*)

(* ::Text:: *)
(*In the code: `mB` is the mass $m_{\rm LL}$, `om` the frequency $\omega^{\rm S}_{\rm LG}$, `lam` the separation constant $\lambda_{\rm LG}$, `mj` the component $m_{j,{\rm LG}}$, `aA`, `bB` the angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$, `u`, `w` the radial functions $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$ of the static tetrad (the system called (S) in the code), `pP`, `qQ` the radial functions $\mathcal P_{\rm LG}$, $\mathcal Q_{\rm LG}$ of the freely falling tetrad (the system (PG)), `fS` the function $f_{\rm LG}$, and `wPG` the speed $w_{\rm LG}$ (in WS-3 `sW`). The matrices `sH` and `hH` are the constant matrices $\mathsf S^{\rm H}_{\rm LG}$ and $\mathsf H_{\rm LG}$ of (18.3), which bring the separated spinor into its simplest form.*)

(* @@PART lg_schw.wls|Schw@@ *)

(* ::Text:: *)
(*The helper functions: `zeroQ`, `num` (a number written in the CSV files, such as `1.5e-3`, read exactly), and `readTable` (the header and the rows of a CSV file; in the notebook it reads the text of the file from a closed cell of this chapter).*)

(* @@PROLOGUE lg_schw.wls@@ *)

(* @@SECTION lg_schw.wls|the matrices@@ *)

(* ::Text:: *)
(*The function `geometry` of the previous chapter, written again (this chapter has its own context), and the Dirac operator `dirac[geo, psi, mass]` $=i\gamma^k_{\rm LG}(\partial_k+\Gamma^{\rm LL}_k)\psi-m\psi$; `angRules` are the angular equations (18.4), $\alpha_{\rm LG}'=(m_{j,{\rm LG}}/\sin\theta)\alpha_{\rm LG}-\lambda_{\rm LG}\beta_{\rm LG}$ and $\beta_{\rm LG}'=-(m_{j,{\rm LG}}/\sin\theta)\beta_{\rm LG}+\lambda_{\rm LG}\alpha_{\rm LG}$.*)

(* @@SECTION lg_schw.wls|the geometry of a coframe@@ *)

(* ::Section:: *)
(*WS-1 and WS-2: the separation in the two tetrads*)

(* ::Text:: *)
(*These checks prove that the separation is exact. The spinor is written with the angular functions $\alpha_{\rm LG}$, $\beta_{\rm LG}$ and two radial functions, it is put into the Dirac equation with the spin connection computed from the tetrad, and the angular and radial equations are used to eliminate the derivatives; the result must vanish identically. WS-1 does this in the static tetrad (outside the horizon), WS-2 in the freely falling tetrad, where the identity holds for every $r>0$: on both sides of the horizon, which is what Part IV needs to follow the field into the black hole.*)

(* @@SECTION lg_schw.wls|WS-1@@ *)

(* @@SECTION lg_schw.wls|WS-2@@ *)

(* ::Section:: *)
(*WS-3: the two descriptions are the same field*)

(* ::Text:: *)
(*Outside the horizon both systems describe the brochon, so they must be related by the change of tetrad and of time coordinate. The check shows that if $\mathcal U_{\rm LG}$, $\mathcal V_{\rm LG}$ solve (18.7), then $\mathcal P_{\rm LG}=(1-w_{\rm LG})^{-1/2}a\,e^{i\omega^{\rm S}_{\rm LG}(\tau_{\rm LG}-t)}$ and $\mathcal Q_{\rm LG}=(1+w_{\rm LG})^{-1/2}b\,e^{i\omega^{\rm S}_{\rm LG}(\tau_{\rm LG}-t)}$, with $a=(\mathcal U_{\rm LG}-i\mathcal V_{\rm LG})/2$, $b=(\mathcal U_{\rm LG}+i\mathcal V_{\rm LG})/2$ and $d(\tau_{\rm LG}-t)/dr=w_{\rm LG}/f_{\rm LG}$, solve (18.5). The square roots are handled by writing everything in terms of the two positive numbers $(1-w_{\rm LG})^{1/2}$ and $(1+w_{\rm LG})^{1/2}$ (in the code `aa` and `bb`), whose squares add up to 2, and reducing the numerators modulo that relation.*)

(* @@SECTION lg_schw.wls|WS-3@@ *)

(* ::Section:: *)
(*WS-4: how the field behaves at the horizon and at the centre*)

(* ::Text:: *)
(*Near a point where the coefficients of a linear system blow up like $1/(r-r_0)$, the solutions behave like powers $(r-r_0)^s$, and the exponents $s$ are the eigenvalues of the residue matrix (the coefficient of $1/(r-r_0)$). At the horizon the exponents of (18.5) are $0$, the solution that is smooth there and falls in, and $4i\omega^{\rm S}_{\rm LG}-\tfrac12$, a solution that oscillates infinitely often and would come out of the black hole; the physical boundary condition keeps the first. At $r=0$ both exponents are $\tfrac14$, so both solutions behave as $r^{1/4}$.*)

(* @@SECTION lg_schw.wls|WS-4@@ *)

(* ::Section:: *)
(*WS-5 and WS-9: the radial current*)

(* ::Text:: *)
(*The conserved charge of the Dirac field gives the radial current (19.2), $J_{\rm LG}=(1+w_{\rm LG})|\mathcal Q_{\rm LG}|^2-(1-w_{\rm LG})|\mathcal P_{\rm LG}|^2$, the flux into a sphere. For a real frequency it does not depend on $r$ (WS-5): what flows in at infinity flows into the horizon. For a complex frequency $\omega^{\rm S}_{\rm LG}=a+ib$ it changes as $dJ_{\rm LG}/dr=2b(|\mathcal P_{\rm LG}|^2+|\mathcal Q_{\rm LG}|^2)$ (WS-9). A quasi-bound state decays at infinity and has $J_{\rm LG}=2$ at the horizon (with the normalization $\mathcal Q_{\rm LG}(2)=1$), so integrating gives $\mathrm{Im}\,\omega^{\rm S}_{\rm LG}=-1/\mathcal N^{\rm S}_{\rm LG}$, with $\mathcal N^{\rm S}_{\rm LG}$ the integral of $|\mathcal P_{\rm LG}|^2+|\mathcal Q_{\rm LG}|^2$ outside the horizon (20.3): the decay rate of a state is fixed by how much of it lies outside the black hole. In the code the complex conjugates are independent functions `pB`, `qB` that obey the conjugate system, and `aw`, `bw` are the real and the imaginary part of the frequency.*)

(* @@SECTION lg_schw.wls|WS-5@@ *)

(* @@SECTION lg_schw.wls|WS-9@@ *)

(* ::Section:: *)
(*WS-6: the angular functions of j = 1/2*)

(* ::Text:: *)
(*For $j_{\rm LG}=\tfrac12$ the angular functions have a closed form, for example $\alpha_{\rm LG}=\sin^{1/2}\tfrac\theta2\cos^{3/2}\tfrac\theta2$ and $\beta_{\rm LG}=\lambda_{\rm LG}\sin^{3/2}\tfrac\theta2\cos^{1/2}\tfrac\theta2$ for $m_{j,{\rm LG}}=\tfrac12$ (18.6). The check verifies the four cases $\lambda_{\rm LG}=\pm1$, $m_{j,{\rm LG}}=\pm\tfrac12$ exactly, and that $\alpha_{\rm LG}$ and $\beta_{\rm LG}$ divided by $(\sin\theta)^{1/2}$ are bounded at both poles (they become $\cos\tfrac\theta2/\sqrt2$ and $\sin\tfrac\theta2/\sqrt2$ up to sign). In the code `sS` and `cC` are the sine and the cosine of $\theta/2$.*)

(* @@SECTION lg_schw.wls|WS-6@@ *)

(* ::Section:: *)
(*The numerical part: a series at the horizon and NDSolve*)

(* ::Text:: *)
(*The next two checks find quasi-bound states and greybody factors again, independently of the Rust program. The solution that is smooth at the horizon is started from its Taylor series of order 20 at $r=2+\tfrac1{20}$ (the coefficients come from a linear system that the cell builds once), and integrated with `NDSolve` in arithmetic of 32 significant digits. This is an integration in the radius, not in time: `NDSolve` uses its automatic method, which controls its error to the requested 26 digits.*)

(* @@SECTION lg_schw.wls|the numerical part@@ *)

(* ::Text:: *)
(*The data of the Rust program that the next two checks compare with: the files `data/lg_schw_qbs.csv` (the quasi-bound states) and `data/lg_schw_grey.csv` (the greybody factors), in closed cells.*)

(* @@DATA lgQbsCSV|data/lg_schw_qbs.csv@@ *)

(* @@DATA lgGreyCSV|data/lg_schw_grey.csv@@ *)

(* ::Section:: *)
(*WS-7: three quasi-bound states*)

(* ::Text:: *)
(*A quasi-bound state is a solution that is smooth at the horizon and decays at infinity; like the levels of a hydrogen atom it exists only for discrete frequencies, here complex, because the state slowly falls into the black hole. For the coupling $y_{\rm LG}=\mathcal M_{\rm LG}m_{\rm LL}=\tfrac3{10}$ (in the code `mmQ`) the check takes the states $1S_{1/2}$, $2P_{1/2}$ and $2P_{3/2}$: it integrates outward from the horizon and inward from a large radius (where it starts from the decaying solution), measures the mismatch of the two at a middle radius, and lets `FindRoot` move the frequency until the mismatch vanishes, starting $10^{-4}$ away from the Rust value. The largest difference from the Rust values, divided by $m_{\rm LL}$, must be below $10^{-11}$ (the Rust values have the accuracy of double precision).*)

(* @@SECTION lg_schw.wls|WS-7@@ *)

(* ::Section:: *)
(*WS-8: three greybody factors*)

(* ::Text:: *)
(*A wave sent towards the black hole is partly absorbed and partly scattered back; the absorbed fraction is the greybody factor. The check integrates from the horizon to $r=4000$, splits the solution there into an incoming and an outgoing wave, and divides the current into the horizon by the incoming current. The three cases are compared with the Rust values, which carry the error $3\times10^{-8}$ of the integrator CVODE; the tolerance is $10^{-7}$.*)

(* @@SECTION lg_schw.wls|WS-8@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*4. The brochon around a black hole whose mass changes (Part V)*)

(* ::Text:: *)
(*Part V puts the brochon into the ingoing Vaidya space-time, $ds^2=f_{\rm LG}\,dv^2-2\,dv\,dr-r^2d\Omega^2$ with $f_{\rm LG}=1-2\mathcal M_{\rm LG}(v_{\rm LG})/r$, the field of a black hole that grows or shrinks (Chapter 2, WG-3). It uses a coframe that is regular across the horizon, $e^0=\tfrac12(1+f_{\rm LG})\,dv-dr$, $e^1=\tfrac12(1-f_{\rm LG})\,dv+dr$, $e^2=r\,d\theta$, $e^3=r\sin\theta\,d\varphi$. The Dirac equation then reduces to two equations for two functions $\mathcal P^{\rm V}_{\rm LG}(v_{\rm LG},r)$ and $\mathcal Q^{\rm V}_{\rm LG}(v_{\rm LG},r)$: an evolution equation in the advanced time (24.2) and a constraint in the radius (24.3),*)

(* ::DisplayFormula:: *)
(*\partial_vP=\Big(\frac Mr-\frac12\Big)\partial_rP-\frac M{2r^2}P-\Big(\frac{i\lambda}{2r}+\frac m2\Big)Q,\qquad\partial_rQ=\Big(\frac{i\lambda}r-m\Big)P.*)

(* ::Text:: *)
(*In the code the advanced time is `v`, the mass function of the black hole `bhM[v]`, the mass of the brochon `mB`, and the radial functions `pP`, `qQ`.*)

(* @@PART lg_vaidya.wls|Vaidya@@ *)

(* @@PROLOGUE lg_vaidya.wls@@ *)

(* @@SECTION lg_vaidya.wls|the matrices@@ *)

(* @@SECTION lg_vaidya.wls|the geometry of a coframe@@ *)

(* ::Section:: *)
(*WV-1: the separation for an arbitrary mass function*)

(* ::Text:: *)
(*With the spin connection computed from the coframe for an arbitrary function $\mathcal M_{\rm LG}(v_{\rm LG})$, the check shows that the separated spinor (24.1) solves the Dirac equation exactly when (18.4), (24.2) and (24.3) hold, for every $r>0$. As a control it changes one coefficient of (24.2) and shows that the identity then fails, so the check is able to fail.*)

(* @@SECTION lg_vaidya.wls|WV-1@@ *)

(* ::Section:: *)
(*WV-2: the conservation of the charge*)

(* ::Text:: *)
(*The charge of the brochon on a slice of constant advanced time is $\mathcal N^{\rm V}_{\rm LG}(v_{\rm LG})$, the integral of $2|\mathcal P^{\rm V}_{\rm LG}|^2$ over $r$ (24.5). It changes only by the flux through the boundaries: $\partial_v(2|\mathcal P^{\rm V}_{\rm LG}|^2)+\partial_r(f_{\rm LG}|\mathcal P^{\rm V}_{\rm LG}|^2-|\mathcal Q^{\rm V}_{\rm LG}|^2)=0$. Part V uses this to follow how much of the field stays outside the black hole while its mass changes, and the Rust program checks it in its evolutions (checks LV-4 and LV-5).*)

(* @@SECTION lg_vaidya.wls|WV-2@@ *)

(* ::Section:: *)
(*WV-3 to WV-4b: the link to Part IV*)

(* ::Text:: *)
(*For a constant mass the equations must reduce to those of Part IV. With $\mathcal P^{\rm V}_{\rm LG}=e^{-i\omega^{\rm S}_{\rm LG}v}p^{\rm V}_{\rm LG}(r)$ and $\mathcal Q^{\rm V}_{\rm LG}=e^{-i\omega^{\rm S}_{\rm LG}v}q^{\rm V}_{\rm LG}(r)$ they become static equations whose exponents at the horizon are again $0$ and $4i\omega^{\rm S}_{\rm LG}-\tfrac12$ (WV-3). Their solutions are the freely falling solutions of Part IV: $p^{\rm V}_{\rm LG}=(1+w_{\rm LG})^{-1/2}e^{i\omega^{\rm S}_{\rm LG}h}\mathcal P_{\rm LG}$ and $q^{\rm V}_{\rm LG}=(1+w_{\rm LG})^{1/2}e^{i\omega^{\rm S}_{\rm LG}h}\mathcal Q_{\rm LG}$ with $h'=1/(1+w_{\rm LG})$ (so that $h$ is the function $\Delta_{\rm LG}=v_{\rm LG}-\tau_{\rm LG}$ of (24.7)); the opposite exponents fail, a control (WV-4). The reason (WV-4b): the coframe of Part V is the freely falling coframe boosted in the radial direction with the rapidity $\vartheta^{\rm V}_{\rm LG}=-\ln(1+w_{\rm LG})$, and a boost multiplies the two radial functions by $e^{\pm\vartheta/2}$. WV-5 shows that the radial light rays are the lines of constant $v_{\rm LG}$ (falling in) and the lines along which (24.2) differentiates $\mathcal P^{\rm V}_{\rm LG}$ (going out); the Rust program uses this to cut the inside of the black hole off its grid.*)

(* @@SECTION lg_vaidya.wls|WV-3@@ *)

(* @@SECTION lg_vaidya.wls|WV-4:@@ *)

(* @@SECTION lg_vaidya.wls|WV-5@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*5. The pair bound by the zeugon (Part VI, Section 29)*)

(* ::Text:: *)
(*Two quasi-particles of the superfluid with opposite zeugon charges $\pm\tfrac12$ attract each other through the massless zeugon, as an electron and a positron attract each other through the photon. Their bound states form an atom, the pair, with the potential $-\alpha^{\rm Z}_{\rm LG}/r$ and the strength $\alpha^{\rm Z}_{\rm LG}=(g^{\rm v}_{\rm LL})^2/(16\pi)$. To leading order its levels are those of hydrogen with the reduced mass $\mu^{\rm pair}_{\rm LG}$; at the next order, $(\alpha^{\rm Z}_{\rm LG})^4$, the motion of the particles (relativistic corrections), their spins (fine and hyperfine structure) and, for a particle with its own antiparticle, a term of virtual annihilation shift the levels. These corrections are computed from the Breit-Fermi Hamiltonian, as for positronium. In the code the units are those of the relative motion: reduced mass 1 and strength 1, so each particle has the mass `m = 2`; `rr` is the radius.*)

(* @@PART lg_pair.wls|Pair@@ *)

(* @@DATA lgPairLevelsCSV|data/lg_pair_levels.csv@@ *)

(* ::Section:: *)
(*WP-1: the levels of the pair to the order alpha^4*)

(* ::Text:: *)
(*For every state with principal number $n\le3$ the check computes the coefficient of $m(\alpha^{\rm Z}_{\rm LG})^4$ of the shift, without and with the annihilation term, from the radial functions of hydrogen (`LaguerreL`) by exact integration, and compares it with the exact rational numbers that the Python program wrote into the file `data/lg_pair_levels.csv` (in the closed cell above). The function `ints` computes the integrals of the radial function that the operators need, `level` adds the terms of the Hamiltonian, and `s12` is the expectation value of the tensor force between the two spins.*)

(* @@SECTION lg_pair.wls|WP-1@@ *)

(* ::Section:: *)
(*WP-2: the barrier of the pair in a tidal field*)

(* ::Text:: *)
(*Near a black hole the tidal field adds $-k^{\rm tid}_{\rm LG}z^2$ to the potential along the radial direction, with $k^{\rm tid}_{\rm LG}=\mu^{\rm pair}_{\rm LG}\mathcal M_{\rm LG}/r^3$ (in the code `k`, and `rg` for $\mathcal M_{\rm LG}$, `al` for $\alpha^{\rm Z}_{\rm LG}$, `mu` for $\mu^{\rm pair}_{\rm LG}$). The sum $-\alpha^{\rm Z}_{\rm LG}/z-k^{\rm tid}_{\rm LG}z^2$ has a maximum, a barrier: the check finds its top at $z_*=(\alpha^{\rm Z}_{\rm LG}/(2k^{\rm tid}_{\rm LG}))^{1/3}$ with the height $-\tfrac32(\alpha^{\rm Z}_{\rm LG})^{2/3}(2k^{\rm tid}_{\rm LG})^{1/3}$, and shows that the barrier sinks to the level $-\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2/(2n^2)$ at the radius given by $r^3=54n^6\mathcal M_{\rm LG}/((\mu^{\rm pair}_{\rm LG})^2(\alpha^{\rm Z}_{\rm LG})^4)$. This is the classical estimate of the break-up of the pair; Part VI shows (Section 32, remark R13) that along a real fall the pair breaks up much later.*)

(* @@SECTION lg_pair.wls|WP-2@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*6. A black hole that carries the charge of the zeugon (Part VI, Section 30)*)

(* ::Text:: *)
(*A black hole can swallow quasi-particles of one zeugon charge more than of the other and so acquire a charge $Q^{\rm Z}_{\rm LG}$. Its field is that of Reissner and Nordstrom, $f^{\rm RN}_{\rm LG}=1-2\mathcal M_{\rm LG}/r+(\mathcal Q^{\rm Z}_{\rm LG})^2/r^2$, with the charge measured as a length $\mathcal Q^{\rm Z}_{\rm LG}=(G_{\rm N}/4\pi)^{1/2}Q^{\rm Z}_{\rm LG}$ in the normalization of the zeugon field of the manuscript (30.1). This chapter checks the geometry, the limit that selective accretion sets to the charge, and the size of the shadow of such a black hole.*)

(* @@PART lg_charge.wls|Charge@@ *)

(* ::Section:: *)
(*WC-1: the geometry*)

(* ::Text:: *)
(*With the zeugon field $A_{{\rm SF},\mu}dx^\mu=(Q^{\rm Z}_{\rm LG}/(4\pi r))\,dt$ the metric with $f^{\rm RN}_{\rm LG}=1-2G_{\rm N}M_{\rm LG}/r+G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/(4\pi r^2)$ solves Einstein's equations with the energy-momentum tensor of the zeugon exactly; with $G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/r^2$ in place of $G_{\rm N}(Q^{\rm Z}_{\rm LG})^2/(4\pi r^2)$ (the normalization of another convention) it does not, which shows that the factor $4\pi$ matters and that the check can fail. In the code `gN` is $G_{\rm N}$, `mm` the mass and `qq` the charge.*)

(* @@SECTION lg_charge.wls|WC-1@@ *)

(* ::Section:: *)
(*WC-2: the limit of the charge by accretion*)

(* ::Text:: *)
(*A charged black hole repels particles of the same sign. A quasi-particle that starts at rest far away, with the ratio of its charge to its mass written as the length $x^{\rm acc}_{\rm LG}$ (30.2), reaches the horizon only if its radial velocity squared, $(1-x^{\rm acc}_{\rm LG}/r)^2-f^{\rm RN}_{\rm LG}$, stays positive all the way down. In units $\mathcal M_{\rm LG}=1$ (in the code `x` for $x^{\rm acc}_{\rm LG}$, `q2` for $(\mathcal Q^{\rm Z}_{\rm LG})^2$ and `W` for the velocity squared) the check shows the identities that make this simple, $r^2(dr/d\tau^{\rm qp}_{\rm LG})^2=2(1-x^{\rm acc}_{\rm LG})r+(x^{\rm acc}_{\rm LG})^2-(\mathcal Q^{\rm Z}_{\rm LG})^2$ (with $\tau^{\rm qp}_{\rm LG}$ the proper time of the quasi-particle) and $(r_{+,{\rm LG}}-x^{\rm acc}_{\rm LG})^2$ at the outer horizon $r_{+,{\rm LG}}$, and lets `Reduce` decide for six charges that the velocity squared is positive at every $r\ge r_{+,{\rm LG}}$ exactly when $0<x^{\rm acc}_{\rm LG}\le1$. Particles whose charge-to-mass length exceeds the mass of the black hole can no longer fall in, which limits the charge that accretion can build up.*)

(* @@SECTION lg_charge.wls|WC-2@@ *)

(* ::Section:: *)
(*WC-3: the shadow*)

(* ::Text:: *)
(*Light that passes close to a black hole can circle it on the photon sphere, where $d(r^2/f^{\rm RN}_{\rm LG})/dr=0$, which here is $r^2-3r+2(\mathcal Q^{\rm Z}_{\rm LG})^2=0$ (units $\mathcal M_{\rm LG}=1$). Light that comes closer falls in, so the black hole casts a shadow whose radius, as seen from far away, is $b^{\rm sh}_{\rm LG}=(r^2/f^{\rm RN}_{\rm LG})^{1/2}$ at the photon sphere $r^{\rm ph}_{\rm LG}$. The check derives $b^{\rm sh}_{\rm LG}=3\sqrt3\,(1-(\mathcal Q^{\rm Z}_{\rm LG})^2/6-7(\mathcal Q^{\rm Z}_{\rm LG})^4/216+\dots)$ for a small charge and $b^{\rm sh}_{\rm LG}=4$ for the largest charge, $\mathcal Q^{\rm Z}_{\rm LG}=\mathcal M_{\rm LG}$; the charge shrinks the shadow, which is how such a charge could be detected (Section 34).*)

(* @@SECTION lg_charge.wls|WC-3@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*7. The pair falls into a black hole (Part VI, Section 32)*)

(* ::Text:: *)
(*A pair that falls freely into a black hole is stretched along the radius and squeezed across it by the tidal field. In the atomic units of the pair (lengths in its Bohr radius $a^{\rm pair}_{\rm LG}$, energies in $\mu^{\rm pair}_{\rm LG}(\alpha^{\rm Z}_{\rm LG})^2$) its relative motion is that of a hydrogen atom in the potential $-1/r-\kappa^{\rm tid}_{\rm LG}r^2P_2(\cos\theta)$, where $P_2(x)=\tfrac12(3x^2-1)$ is the Legendre polynomial of degree 2 and $\kappa^{\rm tid}_{\rm LG}$ the tidal strength (32.2). This chapter checks the pieces of that computation that can be done exactly. In the code `kap` (WT-2) and `kk` (WT-3, WT-4) are $\kappa^{\rm tid}_{\rm LG}$, `T` is the time $T^{\rm fall}_{\rm LG}$, `mm` the mass $\mathcal M_{\rm LG}$, and `F` the strength $F^{\rm u}_{\rm LG}$ of a uniform field.*)

(* @@PART lg_tidal.wls|Tidal@@ *)

(* ::Section:: *)
(*WT-1: how the tidal field couples the angular momenta*)

(* ::Text:: *)
(*The tidal potential $r^2P_2(\cos\theta)$ couples a state of the orbital angular momentum $\ell$ to those of $\ell$ and $\ell\pm2$. The strength of the coupling is the integral of $Y_{\ell0}P_2Y_{\ell'0}$ over the sphere, a product of two 3-j symbols. The check compares the formula with 3-j symbols (`ThreeJSymbol`) with the closed forms that the Rust program uses, $\ell(\ell+1)/((2\ell-1)(2\ell+3))$ for $\ell'=\ell$ and the one for $\ell'=\ell+2$, for $\ell=0,2,\dots,30$.*)

(* @@SECTION lg_tidal.wls|WT-1@@ *)

(* ::Section:: *)
(*WT-2: the tidal field along the fall*)

(* ::Text:: *)
(*The observers of the Painleve-Gullstrand coordinates fall freely from rest far away. The check verifies that they are geodesic and that their proper time is the coordinate time; that the tidal tensor (the geodesic deviation) in their frame is $\mathrm{diag}(2\mathcal M_{\rm LG}/r^3,-\mathcal M_{\rm LG}/r^3,-\mathcal M_{\rm LG}/r^3)$, stretching along the radius and squeezing across; and that along their path $r^3=\tfrac92\mathcal M_{\rm LG}(\tau^{\rm end}_{\rm LG}-\tau_{\rm LG})^2$. In the atomic units of the pair the tidal strength is then $\kappa^{\rm tid}_{\rm LG}=2/(9(T^{\rm fall}_{\rm LG})^2)$, where $T^{\rm fall}_{\rm LG}$ is the proper time that is left before the centre would be reached: the same for every black hole. The mass of the black hole decides only when the horizon is crossed, at $T^{\rm fall}_{\rm LG}=\tfrac43\tilde{\mathcal M}_{\rm LG}$.*)

(* @@SECTION lg_tidal.wls|WT-2@@ *)

(* ::Section:: *)
(*WT-3: the energy in a weak static tidal field*)

(* ::Text:: *)
(*In a weak static field the energy of the ground state shifts by second-order perturbation theory, which the method of Dalgarno and Lewis turns into a differential equation for one function. The check verifies that $\kappa^{\rm tid}_{\rm LG}(r^3/3+r^2/2)$ solves it, and that the energy is $E^{\rm tid}_{\rm LG}=-\tfrac12-\tfrac{15}2(\kappa^{\rm tid}_{\rm LG})^2-\tfrac{153}2(\kappa^{\rm tid}_{\rm LG})^3+\dots$ (32.5); the Python program compares these coefficients with the eigenvalues of its matrices.*)

(* @@SECTION lg_tidal.wls|WT-3@@ *)

(* ::Section:: *)
(*WT-4: the rate of tunnelling in a weak field*)

(* ::Text:: *)
(*In a static tidal field the pair can tunnel through its barrier and fly apart, at the rate $\Gamma^{\rm tid}_{\rm LG}$. For a weak field the rate follows from the solution near the nucleus and the semiclassical solution under the barrier. The check first applies the method to a uniform field of the strength $F^{\rm u}_{\rm LG}$, where it must give Landau's rate $(4/F^{\rm u}_{\rm LG})\,e^{-2/(3F^{\rm u}_{\rm LG})}$ (it does), and then to the tidal field, where it gives $8(\kappa^{\rm tid}_{\rm LG})^{-1/2}\exp(-\pi/(2(2\kappa^{\rm tid}_{\rm LG})^{1/2}))/\sinh(\pi/\sqrt2)$, the formula that the manuscript compares with the computed rates.*)

(* @@SECTION lg_tidal.wls|WT-4@@ *)

(* ::Section:: *)
(*WT-5: classical motion in the growing field*)

(* ::Text:: *)
(*Along the fall the tidal strength grows as $\kappa^{\rm tid}_{\rm LG}=2/(9(T^{\rm fall}_{\rm LG})^2)$. The classical equations of the relative motion along and across the radius, $z''=2\kappa^{\rm tid}_{\rm LG}z$ and $x''=-\kappa^{\rm tid}_{\rm LG}x$, then have power-law solutions: $T^{4/3}$ and $T^{-1/3}$ along, $T^{2/3}$ and $T^{1/3}$ across (with $T=T^{\rm fall}_{\rm LG}$). The check lets `DSolve` find them and compares the solutions with given initial values with the closed forms of the Rust program, which uses them to test its integration.*)

(* @@SECTION lg_tidal.wls|WT-5@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*8. The quasi-particles are Dirac particles: annihilation (Part VI, Section 33)*)

(* ::Text:: *)
(*The pairing in the superfluid mixes particles and antiparticles of the brochon, so its quasi-particles are mixtures too, and it is not obvious which of them can annihilate. Theorem L3 of the manuscript answers this: the quasi-particle field is $\Psi_{\rm LL}=\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG}\chi_{\rm LG}$ with $b^{\rm qp}_{\rm LG}=-D/(E+M)$, where $\chi_{\rm LG}$ is an ordinary Dirac field of the mass $E=(M^2+|D|^2)^{1/2}$; here, as in the theorem, $M$, $D$ and $E$ are short for the Dirac mass $M^{\rm CJT,D}_{\rm SF}$, the pair amplitude $D^{\rm CJT}_{\rm SF}$ and the gap $E^{\rm CJT,gap}_{\rm SF}$ of the superfluid. So the quasi-particles are the particles and antiparticles of a Dirac field, and the zeugon conserves their number $N^{\rm qp}_{\rm LG}$. This chapter checks the theorem exactly, including in the Fock space of the quantized field.*)

(* @@PART lg_annih.wls|Annih@@ *)

(* ::Text:: *)
(*The matrices of the chiral basis and of the pairing term, on the eight components of the brochon (four Dirac components times two isospin components): `g8` the Dirac matrices, `pm` the matrix $\mathsf P_{\rm LG}$, `t3` the generator $T^3_{\rm LL}$, `fl` the reflection $\mathsf F^{\rm v}_{\rm LG}=\mathsf 1_4\otimes\tau^{\rm LL}_1$ that exchanges the two isospin components.*)

(* @@PROLOGUE lg_annih.wls@@ *)

(* ::Section:: *)
(*WA-1: the algebra of the pairing term*)

(* ::Text:: *)
(*The matrix $\mathsf P_{\rm LG}$ is real, antisymmetric and unitary; it commutes with the Dirac matrices and with $iT^3_{\rm LL}$ in the sense of WG-6; and the reflection $\mathsf F^{\rm v}_{\rm LG}$ commutes with the pairing map and turns $T^3_{\rm LL}$ into $-T^3_{\rm LL}$. The reflection is the element of the unbroken group $O(2)$ that the selection rules of annihilation use.*)

(* @@SECTION lg_annih.wls|WA-1@@ *)

(* ::Section:: *)
(*WA-2: Theorem L3 for a plane wave*)

(* ::Text:: *)
(*The core of Theorem L3 is the identity $[i\gamma\cdot\partial-M-D\,\mathcal C_{\rm LG}](1+b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG})=(1-b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG})[i\gamma\cdot\partial-E]$: the operator of the superfluid, applied to $\chi_{\rm LG}+b^{\rm qp}_{\rm LG}\,\mathcal C_{\rm LG}\chi_{\rm LG}$, gives the same as the Dirac operator of the mass $E$ applied to $\chi_{\rm LG}$, up to the factor on the left, which can be inverted. The check verifies it on a plane wave with a symbolic spinor (16 real symbols) and a symbolic four-momentum that need not satisfy any mass condition, with symbolic $M$, $|D|$ and phase of $D$. As a control, the wrong sign of $b^{\rm qp}_{\rm LG}$ fails.*)

(* @@SECTION lg_annih.wls|WA-2@@ *)

(* ::Section:: *)
(*WA-3: the quantized field and its zeugon current*)

(* ::Text:: *)
(*The quantized field of eight modes lives in a Fock space of dimension $2^8=256$, built with the construction of Jordan and Wigner, in exact rational arithmetic with $b^{\rm qp}_{\rm LG}=\tfrac37+\tfrac25i$. The check shows that $\Psi=\chi+b^{\rm qp}_{\rm LG}\,\mathsf P_{\rm LG}\chi^\dagger$ has the anticommutators of a free field up to the factor $1+|b^{\rm qp}_{\rm LG}|^2$, and that its zeugon current is exactly $1+|b^{\rm qp}_{\rm LG}|^2$ times that of $\chi$, up to a constant: the current contains no term that creates or destroys two quasi-particles, so the zeugon cannot make two quasi-particles of opposite charge annihilate unless they are a particle and its own antiparticle.*)

(* @@SECTION lg_annih.wls|WA-3@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*9. The worked examples (Parts II and III, Appendices A and B)*)

(* ::Text:: *)
(*The manuscript illustrates its objects with explicit examples, chosen so that every number is a simple fraction. This chapter checks them exactly. In the code: `mll[xi]` is the $2\times2$ matrix $M_{\rm LL}(\mathbf b)$ of a bi-quaternion with the coefficients `xi`, `ndet` the polarized determinant $N_{\rm LG}$, `dirac` Lord's Dirac matrices of a set of four $2\times2$ matrices, `lorentzOf` the Lorentz matrix $\mathsf L_{\rm LL}(\mathbf g)$ of a unit bi-quaternion.*)

(* @@PART lg_worked.wls|Worked@@ *)

(* @@PROLOGUE lg_worked.wls@@ *)

(* ::Section:: *)
(*WW-1: a change of representation by a boost*)

(* ::Text:: *)
(*The unit bi-quaternion $\mathbf g=\tfrac54\mathbf e_0+\tfrac34i\,\mathbf e_3$ is a boost along the third axis with the velocity $\tfrac{15}{17}$ (its Lorentz matrix has the entries $\tfrac{17}8$ and $\tfrac{15}8$, and $(\tfrac{17}8)^2-(\tfrac{15}8)^2=1$). Its matrix $S=\mathrm{diag}(\tfrac12,2)$ changes Lord's matrices into $\mathrm{diag}(\tfrac14,4)$, $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$ and $\mathrm{diag}(\tfrac14,-4)$. The check verifies, exactly, that the new set is Hermitian with the right polarized determinants, that its Dirac matrices are those of the old set conjugated by the spinor matrix of the boost, that they equal the old ones combined with the inverse Lorentz matrix, and that the new set is the frame $(\tfrac{17}8,0,0,-\tfrac{15}8)$, $(0,1,0,0)$, $(0,0,1,0)$, $(-\tfrac{15}8,0,0,\tfrac{17}8)$ of M31, whose time vector moves with the velocity $-\tfrac{15}{17}$.*)

(* @@SECTION lg_worked.wls|WW-1@@ *)

(* ::Section:: *)
(*WW-2: a set that is not a representation*)

(* ::Text:: *)
(*Lord writes that any linearly independent set of four Hermitian matrices will do; the manuscript shows that this is too much. The set $\sigma^{\rm LL}_0$, $\sigma^{\rm LL}_1$, $\sigma^{\rm LL}_2$, $\sigma^{\rm LL}_3+\tfrac12\sigma^{\rm LL}_0$ is Hermitian and linearly independent, but the polarized determinant of the first and the last is $\tfrac12$ instead of 0, and their Dirac matrices do not anticommute. So a valid set must also satisfy (1.3).*)

(* @@SECTION lg_worked.wls|WW-2@@ *)

(* ::Section:: *)
(*WW-3: a plane wave of the brochon and its energy and momentum*)

(* ::Text:: *)
(*The plane wave with the spinor $u_{\rm LL}=(\tfrac12,0,2,0)$, the momentum $P_{\rm LG}=(\tfrac{17}8,0,0,\tfrac{15}8)$ and the mass 1 (the boosted rest spinor of Appendix A) is checked with symbolic coordinates: it solves the free Dirac equation, its current is twice its momentum, its Lagrangian density vanishes (as it must on a solution), and its energy-momentum tensor (16.2) is twice the product of the momentum with itself, with the trace 2: the tensor of a stream of particles of the momentum $P_{\rm LG}$.*)

(* @@SECTION lg_worked.wls|WW-3@@ *)

(* ::Section:: *)
(*WW-4: the quasi-particle energies of the superfluid*)

(* ::Text:: *)
(*In the polar ground state of the brochon model the Bogoliubov-de Gennes matrix (a $16\times16$ matrix that couples the brochon at the momentum $\vec q$ with its conjugate at $-\vec q$) has, for symbolic real momentum, Dirac mass $M$ and pair amplitude $D$ (in the code `mM` and `dd`), a square that is $(|\vec q|^2+M^2+D^2)$ times the unit matrix, and the trace 0. Hence its eigenvalues are $\pm E(\vec q)$, eight times each, with $E(\vec q)=(|\vec q|^2+M^2+D^2)^{1/2}$ (B.18): the energy of a quasi-particle, whose smallest value, at $\vec q=0$, is the gap.*)

(* @@SECTION lg_worked.wls|WW-4@@ *)

(* ::Section:: *)
(*WW-5: the gap equation*)

(* ::Text:: *)
(*The size of the gap is fixed by a gap equation that contains the integral $I^{(1)}_{\rm SF}(M)=\int_0^1q^2(q^2+M^2)^{-1/2}dq$ (A.19, with the cutoff $\Lambda_{\rm LL}=1$). The check computes the integral with `Integrate`, compares it with the closed form of (A.19) symbolically and as numbers of 40 digits, and then puts the printed numbers of the manuscript into the gap equation of the method CJT, $g^{{\rm CJT},{\rm P}}_{\rm SF}\,I^{(1)}_{\rm SF}(E^{\rm CJT,gap}_{\rm SF})/\pi^2=1$ with $g^{{\rm CJT},{\rm P}}_{\rm SF}=23.7$, and into $\pi^2/I^{(1)}_{\rm SF}(\tfrac15)=G^{\rm crit}_{\rm LL}$: both hold to the ten printed digits. The printed numbers come from the fragment file `lg_examples.md`, in the closed cell below.*)

(* @@DATA lgExamplesText|provenance/generated/lg_examples.md@@ *)

(* @@SECTION lg_worked.wls|WW-5@@ *)

(* @@ENDPART@@ *)

(* ::Chapter:: *)
(*10. The remarks of the manuscript*)

(* ::Text:: *)
(*The manuscript qualifies several of its results: it says where a result is an approximation, a model or restricted in some way, and its remarks document explains each such remark (what it means, why it holds, why it matters). Here they are in brief, with the chapter of this notebook that bears on each.*)

(* ::Item:: *)
(*R1, the coupling to the scalar curvature: the square of the Dirac operator contains $-\tfrac14R$, not Lord's $\tfrac12$ (Chapter 2, WG-4).*)

(* ::Item:: *)
(*R2, a constant Dirac mass and pair amplitude: Theorem L2 assumes that the Dirac mass and the pair amplitude of the quasi-particle are the same everywhere; the algebra that the theorem rests on is checked in Chapter 2 (WG-6).*)

(* ::Item:: *)
(*R3, the energy of the condensed vacuum: the condensation lowers the energy of the vacuum, and gravitation responds to that as to a cosmological constant (computed by the Rust and the Python programs; not in this notebook).*)

(* ::Item:: *)
(*R4, operators, normal order and semiclassical gravitation: the interaction and the cutoff of the brochon are defined in flat space-time and have no unique form in a curved one; the source of Einstein's equations is the expectation value of the energy-momentum tensor of Chapter 2 (WG-5), which is called semiclassical gravitation.*)

(* ::Item:: *)
(*R5, the massive gauge fields: in the superfluid $W^1$, $W^2$ and $B$ are massive, and their energy-momentum tensors add to the source of gravitation; the manuscript needs only the tensor of the zeugon, which stays massless (Chapter 2, WG-6).*)

(* ::Item:: *)
(*R6, the small-mass formulas of the gravitational atom: the formulas of Dolan and Dempsey hold for small coupling only; the exact states are those of Chapter 3 (WS-7).*)

(* ::Item:: *)
(*R7, the shrinking black hole is a model: a black hole that shrinks by evaporation is described by the ingoing Vaidya space-time with an inflow of negative energy (the model of W. A. Hiscock); the radiation that leaves is not modelled (Chapter 2, WG-3; Chapter 4).*)

(* ::Item:: *)
(*R8, projections are not probabilities: the quasi-bound states of a black hole are not orthogonal, so the projections of the field on them do not add up to one (computed by the Rust program; not in this notebook).*)

(* ::Item:: *)
(*R9, the polarization of the condensate is not computed: the potential of the pair is that of the empty vacuum; the superfluid could weaken it at long distances (Chapter 5 uses the unweakened potential).*)

(* ::Item:: *)
(*R10, the estimate of the charge by the creation of pairs: Schwinger's rate of pair creation is used as an estimate of the largest charge (Chapter 6 checks the geometry and the accretion limit).*)

(* ::Item:: *)
(*R11, the ringing in the limit of large angular number: the relation between the shadow and the ringing holds for large angular number only (Chapter 6, WC-3).*)

(* ::Item:: *)
(*R12, the quadratic tidal potential and the slow relative motion: the tidal potential is the first term of an expansion in the size of the pair divided by the radius (Chapter 7, WT-2).*)

(* ::Item:: *)
(*R13, the classical estimate of the break-up is false for a real fall: the barrier of Chapter 5 (WP-2) sinks much earlier than the pair actually separates.*)

(* ::Item:: *)
(*R14, the probability of separation late in the fall depends on a numerical device: below the remaining time 1, the probability that the two quasi-particles are more than 10 Bohr radii apart depends on how the tidal potential is flattened far out (computed by the Rust program; not in this notebook).*)

(* ::Item:: *)
(*R15, the uncertainties of the cosmological parameters enter the comparison of the vacuum energy with observation (not in this notebook).*)

(* ::Item:: *)
(*R16, the conservation of the number of quasi-particles holds in the mean-field approximation (Chapter 8, WA-3).*)

(* ::Chapter:: *)
(*The list of all checks*)

(* ::Text:: *)
(*The last cells list every check of this notebook with its result, and count the checks that passed.*)

(* ::Input:: *)
(* the table of all checks: identifier, result, and for the numerical checks the deviation and the tolerance *)
Grid[Prepend[Map[{#["id"], If[#["pass"], "PASS", "FAIL"], If[NumericQ[#["deviation"]], fmt[#["deviation"]], "--"],
      If[NumericQ[#["tolerance"]], fmt[#["tolerance"], 2], "--"]} &, LGNB`Test`$Results],
   Style[#, Bold] & /@ {"check", "result", "deviation", "tolerance"}],
  Frame -> All, Alignment -> Left, BaseStyle -> {FontSize -> 9}]

(* ::Input:: *)
(* the count of the checks that passed *)
Print["checks of this notebook: ", Length[LGNB`Test`$Results], "; passed: ", Count[LGNB`Test`$Results, lgR_ /; lgR["pass"]],
  "; failed: ", Select[LGNB`Test`$Results, ! #["pass"] &][[All, "id"]]]
