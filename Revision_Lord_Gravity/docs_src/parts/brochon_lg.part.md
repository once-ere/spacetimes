# B. The brochon model: the field, its symmetries, its interaction and its superfluid vacuum

This appendix states the model in which the brochon lives on the flat space-time M31 before gravitation is added, the
*brochon model*, with everything that the main text uses of it: the field and its charges (Section B.1), the gauge group
GEE and the covariant derivative (B.2), the Lagrangian and the field equations (B.3), the mean field and the three methods
by which the ground state is computed (B.4), the recursion of the interaction and the superfluid ground state (B.5), and
the breaking of the symmetry, the masses of the gauge bosons and the zeugon (B.6). Section B.7 lists its symbols. The
objects of the algebra (the bi-quaternions, M31, the spinor charges, the Dirac matrices and the pair matrices
$\mathsf K^\iota_{\rm LL}$) are those of Appendix A.

The model answers a problem that the author posed: a spinor field made of the spinor charges of the Lorentz super-loop,
with the gauge group GEE and an interaction formed after his Kohn-Sham ansatz ($V^{(0)}_{\rm SF}$ below), whose ground state
was to be found by three methods. The changes of the interaction that lead to a superfluid in all three methods (Section
B.5), the ground states, the masses of the gauge bosons and the zeugon were computed in the work on the project, by the Rust
program `sf_part2` (crate `rust/sfpart2`) and the Python program `python/sf_part2.py` of the folder `Revision_SM_Higgs`
({{GEN:p2_verdict_rust}} and {{GEN:p2_verdict_python}}). The numbers of this appendix are copied from their fragments by the
script that Appendix A names, which records the files and their SHA-256 checksums. Energies are measured in units of the
momentum cutoff $\Lambda_{\rm LL}$ (so $\Lambda_{\rm LL}=1$), and the mass of the brochon is $m_{\rm LL}=0.2$.

*In words.* The brochon model is a Dirac field with an internal index, the isospin, coupled to gauge fields of its isospin and
of its fermion number, with an attraction that binds two brochons into pairs of spin zero. In the state of lowest energy the
pairs condense, as the Cooper pairs of the electrons do in a superconductor. The condensate gives three of the four gauge
bosons a mass and leaves one massless: the zeugon. The rest of the appendix makes each of these statements precise.

## B.1 The field

**The components.** The brochon $\Psi_{\rm LL}$ is the field of the eight spinor charges of Appendix A.4: eight complex
components $\Psi_{{\rm LL},c}$, $c=1,\dots,8$, functions of the point $x$ of M31. Component $c$ has the Dirac index
$d=1,\dots,4$ and the isospin index $s=1$ (up) or $2$ (down), with $c=2(d-1)+s$; the components 1 to 4 are left-handed
($\gamma^{\rm ch}_{\rm LL}=-1$) and 5 to 8 right-handed ($\gamma^{\rm ch}_{\rm LL}=+1$). A $4\times4$ Dirac matrix $A$ acts on
$\Psi_{\rm LL}$ as the Kronecker product $A\otimes\mathsf 1_2$ and a $2\times2$ isospin matrix $t$ as $\mathsf 1_4\otimes t$; where
no confusion can arise, $A\otimes\mathsf 1_2$ is written $A$. The Dirac adjoint is
$\bar\Psi_{\rm LL}=\Psi^\dagger_{\rm LL}(\gamma^0_{\rm LL}\otimes\mathsf 1_2)$.

**Anticommutation.** The rotation by $2\pi$ multiplies every spinor charge by $-1$ (Appendix A.4). By the spin-statistics
theorem the brochon is therefore a fermion field: its components are operators that anticommute at equal times,
$$\{\Psi_{{\rm LL},c}(x),\Psi^\dagger_{{\rm LL},c'}(y)\}=\delta_{cc'}\,\delta^3(\vec x-\vec y),\qquad\{\Psi_{{\rm LL},c}(x),\Psi_{{\rm LL},c'}(y)\}=0,\tag{B.1}$$
where $\vec x$ and $\vec y$ are the space parts of the two points.

**The charges.** Three groups act on the brochon: the Lorentz group $SL(2,\mathbb C)$, by the spinor matrix
$\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf 1_2$ of (9.3); the isospin rotations of Appendix A.4, which form a group $SU(2)$
written $SU(2)_{\rm v}$, by $\mathsf 1_4\otimes\mathsf U_{\rm LL}$ with $\mathsf U_{\rm LL}\in SU(2)$; and the phases
$\Psi_{\rm LL}\mapsto e^{i\theta^{\rm F}_{\rm LL}}\Psi_{\rm LL}$, which form the group $U(1)^{\rm F}_{\rm LL}$. The generators of the isospin
rotations are $T^\iota_{\rm LL}=\mathsf 1_4\otimes\tau^{\rm LL}_\iota/2$ ($\iota=1,2,3$), with $[T^1_{\rm LL},T^2_{\rm LL}]=iT^3_{\rm LL}$
and cyclically; the generator of the phases is the *fermion number* $F_{\rm LL}$, which is 1 on $\Psi_{\rm LL}$. The *charge*
$Q_{\rm LL}=T^3_{\rm LL}$ is $+\tfrac12$ on the up components and $-\tfrac12$ on the down components. It is a charge of the
group $SU(2)_{\rm v}$ of the model and not the electric charge: the brochon carries no electric charge, and nothing in the model
does.

**Plane waves and the normal vacuum.** In a box of volume $V$ with periodic boundary conditions,
$\Psi_{\rm LL}(x)=V^{-1/2}\sum_{\vec q}e^{i\vec q\cdot\vec x}\Psi_{\rm LL}(\vec q)$, and the Hamiltonian of the free field is
$\sum_{\vec q}\Psi^\dagger_{\rm LL}(\vec q)\,\mathsf h_{\rm LL}(\vec q)\,\Psi_{\rm LL}(\vec q)$ with the free Dirac Hamiltonian
$\mathsf h_{\rm LL}(\vec q)=\gamma^0_{\rm LL}(\vec\gamma_{\rm LL}\cdot\vec q+m_{\rm LL})$ of (A.13), whose eigenvalues are
$\pm\varepsilon_{\rm LL}$, $\varepsilon_{\rm LL}=(q^2+m_{\rm LL}^2)^{1/2}$, four times each. The *normal vacuum* is the state in
which every mode of negative energy is occupied and every mode of positive energy is empty (the filled Dirac sea), for the
momenta below the cutoff. The *correlation matrices* of a state are $(\mathsf C^{\rm n}_{\rm LL})_{cc'}=\langle\Psi^\dagger_{{\rm LL},c'}\Psi_{{\rm LL},c}\rangle$
(the normal one, Hermitian) and $(\mathsf C^{\rm p}_{\rm LL})_{cc'}=\langle\Psi_{{\rm LL},c}\Psi_{{\rm LL},c'}\rangle$ (the pair one,
antisymmetric), at one point or, with the argument $\vec q$, for one momentum. For the normal vacuum the pair matrix is zero
and the normal matrix is the projector onto the negative eigenvalues,
$$\mathsf C^{\rm vac}_{\rm LL}(\vec q)=\tfrac12\big(\mathsf 1_8-\mathsf h_{\rm LL}(\vec q)/\varepsilon_{\rm LL}\big).\tag{B.2}$$
The *normal order* $:\,\cdot\,:$ of a product of field operators subtracts from every contraction its value in the normal
vacuum, so that a normal-ordered product has the expectation value zero in the normal vacuum. Every energy of the model is
measured from the normal vacuum.

**Example: a plane wave of the brochon.** The spinor $u_{\rm LL}={{GENM:lgw_lw5_u}}$ of Appendix A.5 belongs to a particle of
mass 1 and momentum $P_{\rm LG}={{GENM:lgw_lw5_p}}$. The field $\Psi_{\rm LL}(x)=(u_{\rm LL}\otimes(1,0))\,e^{-iP_{\rm LG}\cdot x}$, whose
eight components are ${{GENM:lgw_lw5_psi}}$ times $e^{-iP_{\rm LG}\cdot x}$, is a brochon of mass 1 with isospin up
($Q_{\rm LL}=+\tfrac12$) in a plane wave: it solves the free field equation $(i\gamma^p_{\rm LL}\partial_p-1)\Psi_{\rm LL}=0$ (checks
LW-5c of the program `lg_worked` of this folder and YW-5b, WW-3 of its Python and Wolfram counterparts). Section 16.1 computes
its current and its energy-momentum tensor.

## B.2 The gauge group GEE and the covariant derivative

The gauge group of the model, named GEE by the author, is
$$\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}.\tag{B.3}$$
A **local GEE transformation** is a triple of functions on M31: a unit bi-quaternion $\mathbf g(x)$, an isospin rotation
$\mathsf U_{\rm LL}(x)\in SU(2)$ and a real phase $\theta^{\rm F}_{\rm LL}(x)$. It acts on the brochon by
$$\Psi_{\rm LL}\longmapsto\mathcal U_{\rm LL}\Psi_{\rm LL},\qquad\mathcal U_{\rm LL}=e^{i\theta^{\rm F}_{\rm LL}}\,\mathsf S_{\rm LL}(\mathbf g)\otimes\mathsf U_{\rm LL},\tag{B.4}$$
and on a frame by $\mathbf f_p\mapsto\tilde{\mathbf g}\,\mathbf f_p\,\tilde{\mathbf g}^\dagger$ (Section 9.2).

**The gauge fields.** Each factor of GEE has its gauge field: the *spin connection* $\Gamma^{\rm LL}_p$, for each direction $p$ a
$4\times4$ matrix of the Lorentz algebra (the gauge field of $SL(2,\mathbb C)$); three real vector fields $W^\iota_{{\rm SF},p}$,
collected in the Hermitian traceless matrix $\mathsf W_{{\rm SF},p}=\sum_\iota W^\iota_{{\rm SF},p}\tau^{\rm LL}_\iota/2$ (the gauge
field of $SU(2)_{\rm v}$); and a real vector field $B_{{\rm SF},p}$ (the gauge field of $U(1)^{\rm F}_{\rm LL}$). Their couplings are
$g^{\rm v}_{\rm LL}=0.7$ and $g^{\rm F}_{\rm LL}=0.45$, the values used in all the work on the model. The **covariant derivative**
of the brochon is
$$\nabla^{\rm GEE}_p\Psi_{\rm LL}=\big(\partial_p+\Gamma^{\rm LL}_p\otimes\mathsf 1_2+ig^{\rm v}_{\rm LL}\,\mathsf 1_4\otimes\mathsf W_{{\rm SF},p}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\big)\Psi_{\rm LL},\tag{B.5}$$
the same as (9.5). Under the local transformation (B.4) let the gauge fields change as
$$\begin{aligned}\Gamma'^{\rm LL}_p&=\mathsf S\,\Gamma^{\rm LL}_p\,\mathsf S^{-1}-(\partial_p\mathsf S)\,\mathsf S^{-1},\\\mathsf W'_{{\rm SF},p}&=\mathsf U_{\rm LL}\mathsf W_{{\rm SF},p}\mathsf U_{\rm LL}^\dagger+\frac i{g^{\rm v}_{\rm LL}}(\partial_p\mathsf U_{\rm LL})\mathsf U_{\rm LL}^\dagger,\\B'_{{\rm SF},p}&=B_{{\rm SF},p}-\frac1{g^{\rm F}_{\rm LL}}\partial_p\theta^{\rm F}_{\rm LL},\end{aligned}\tag{B.6}$$
with $\mathsf S=\mathsf S_{\rm LL}(\mathbf g(x))$. **Theorem (covariance).** $\nabla'^{\rm GEE}_p(\mathcal U_{\rm LL}\Psi_{\rm LL})=\mathcal U_{\rm LL}\nabla^{\rm GEE}_p\Psi_{\rm LL}$.
*Proof.* The three factors of $\mathcal U_{\rm LL}$ commute and act on different indices, so it suffices to check each with
its own gauge field. For the phase,
$(\partial_p+ig^{\rm F}_{\rm LL}B'_{{\rm SF},p})(e^{i\theta^{\rm F}_{\rm LL}}\Psi)=e^{i\theta^{\rm F}_{\rm LL}}(\partial_p+i\partial_p\theta^{\rm F}_{\rm LL}+ig^{\rm F}_{\rm LL}B_{{\rm SF},p}-i\partial_p\theta^{\rm F}_{\rm LL})\Psi$; for the isospin,
$(\partial_p+ig^{\rm v}_{\rm LL}\mathsf W'_{{\rm SF},p})(\mathsf U_{\rm LL}\Psi)=\mathsf U_{\rm LL}\partial_p\Psi+(\partial_p\mathsf U_{\rm LL})\Psi+ig^{\rm v}_{\rm LL}\mathsf U_{\rm LL}\mathsf W_{{\rm SF},p}\Psi-(\partial_p\mathsf U_{\rm LL})\Psi$;
the Lorentz factor is the same computation with $\Gamma'^{\rm LL}_p$. $\square$ The transformed $\mathsf W'_{{\rm SF},p}$ is again
Hermitian and traceless, because $(\partial_p\mathsf U_{\rm LL})\mathsf U_{\rm LL}^\dagger$ is anti-Hermitian
($\mathsf U_{\rm LL}\mathsf U_{\rm LL}^\dagger=\mathsf 1_2$) and traceless ($\det\mathsf U_{\rm LL}=1$). Since the Dirac matrices of
a frame change as $\gamma^p_{\rm LL}[\tilde{\mathbf g}\mathbf f\tilde{\mathbf g}^\dagger]=\mathsf S\gamma^p_{\rm LL}[\mathbf f]\mathsf S^{-1}$
(Section 9.2), the Dirac operator $\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p$ is covariant as well, and
$\bar\Psi_{\rm LL}(i\gamma^p_{\rm LL}[\mathbf f]\nabla^{\rm GEE}_p-m_{\rm LL})\Psi_{\rm LL}$ is invariant.

**Field strengths and currents.** The field strengths of the gauge fields are
$$\begin{aligned}W^\iota_{{\rm SF},pp'}&=\partial_pW^\iota_{{\rm SF},p'}-\partial_{p'}W^\iota_{{\rm SF},p}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},p}W^{\iota''}_{{\rm SF},p'},\qquad B_{{\rm SF},pp'}=\partial_pB_{{\rm SF},p'}-\partial_{p'}B_{{\rm SF},p},\\\Omega^{\rm L}_{{\rm SF},pp'}&=\partial_p\Gamma^{\rm LL}_{p'}-\partial_{p'}\Gamma^{\rm LL}_p+[\Gamma^{\rm LL}_p,\Gamma^{\rm LL}_{p'}],\end{aligned}\tag{B.7}$$
with the Levi-Civita symbol $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ of three indices; in matrix form
$\mathsf W_{{\rm SF},pp'}=\sum_\iota W^\iota_{{\rm SF},pp'}\tau^{\rm LL}_\iota/2=\partial_p\mathsf W_{{\rm SF},p'}-\partial_{p'}\mathsf W_{{\rm SF},p}+ig^{\rm v}_{\rm LL}[\mathsf W_{{\rm SF},p},\mathsf W_{{\rm SF},p'}]$.
They change covariantly, $\mathsf W'_{{\rm SF},pp'}=\mathsf U_{\rm LL}\mathsf W_{{\rm SF},pp'}\mathsf U_{\rm LL}^\dagger$, $B'_{{\rm SF},pp'}=B_{{\rm SF},pp'}$,
$\Omega'^{\rm L}_{{\rm SF},pp'}=\mathsf S\,\Omega^{\rm L}_{{\rm SF},pp'}\mathsf S^{-1}$. The **currents** of the brochon are the isospin
currents and the fermion-number current,
$$j^{\iota,p}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^p_{\rm LL}T^\iota_{\rm LL}\Psi_{\rm LL},\qquad j^{{\rm F},p}_{\rm SF}=\bar\Psi_{\rm LL}\gamma^p_{\rm LL}\Psi_{\rm LL}.\tag{B.8}$$

**Flat M31.** On M31 the spin connection carries no field strength of its own: in the canonical frame it is zero, and in a frame
obtained from the canonical one by a local transformation it is the pure gauge $\Gamma^{\rm LL}_p=-(\partial_p\mathsf S)\mathsf S^{-1}$,
whose curvature $\Omega^{\rm L}_{{\rm SF},pp'}$ vanishes. The brochon model leaves the dynamics of this connection out; it is
computed in the canonical frame, where $\Gamma^{\rm LL}_p=0$ and $\gamma^p_{\rm LL}[\mathbf f]=\gamma^p_{\rm LL}$. The main text supplies
that dynamics.

## B.3 The Lagrangian and the field equations

The Lagrangian density of the brochon model is
$$\mathcal L_{\rm SF}=\bar\Psi_{\rm LL}\big(i\gamma^p_{\rm LL}\nabla^{\rm GEE}_p-m_{\rm LL}\big)\Psi_{\rm LL}-V^{(n)}_{\rm SF}-\tfrac14\sum_\iota W^\iota_{{\rm SF},pp'}W^{\iota,pp'}_{\rm SF}-\tfrac14B_{{\rm SF},pp'}B^{pp'}_{\rm SF}\tag{B.9}$$
(indices raised with $\eta^{\rm LL}$, repeated indices summed): the Dirac part with the mass term, the self-interaction
$-V^{(n)}_{\rm SF}$, and the Yang-Mills part of the gauge fields of $SU(2)_{\rm v}$ and $U(1)^{\rm F}_{\rm LL}$. The **self-interaction**
$V^{(n)}_{\rm SF}$ is a sum of normal-ordered GEE-invariant products of four field components at one point (a *contact
interaction*); the index $n$ counts the steps of the recursion of Section B.5. Its first two forms are
$$V^{(0)}_{\rm SF}=\tfrac12G^{\rm S}_{\rm SF}:(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:+\tfrac12G^{\rm V}_{\rm SF}\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:,\qquad V^{(1)}_{\rm SF}=V^{(0)}_{\rm SF}-\tfrac14G_{\rm LL}\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:,\tag{B.10}$$
with the local pair operators $\Pi^\iota_{\rm LL}(x)=\Psi_{\rm LL}(x)^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(x)$, which remove two brochons at
one point in the spin-zero pair state $\mathsf K^\iota_{\rm LL}=C_{\rm LL}\gamma^{\rm ch}_{\rm LL}\otimes\mathsf E_{\rm LL}\tau^{\rm LL}_\iota$ of
Appendix A.6 (fermion number 2, an isospin triplet); their integral over space is the operator
$\sum_{\vec q}\Psi_{\rm LL}(\vec q)^T\mathsf K^\iota_{\rm LL}\Psi_{\rm LL}(-\vec q)$ of Appendix A.6, which removes a pair of opposite momenta. The three terms act on the scalar density $\bar\Psi\Psi$, on the number
current and on the pairs: a negative $G^{\rm S}_{\rm SF}$ is an attraction in the scalar density, a positive $G^{\rm V}_{\rm SF}$ a
repulsion of the number densities, and a positive $G_{\rm LL}$ an attraction in the spin-zero pair channel. $V^{(0)}_{\rm SF}$ is
formed after the author's Kohn-Sham ansatz, whose interaction energy per volume of uniform matter is
$\tfrac12G_Vn^2+\tfrac12G_Sn_S^2$ (number density $n$, scalar density $n_S$, his values $G_S=-8$ and $G_V=0.8$): in a uniform
state the two terms of $V^{(0)}_{\rm SF}$ reduce to these, so $G^{\rm S}_{\rm SF}=-8$ and $G^{\rm V}_{\rm SF}=0.8$. Every term of (B.9) is
GEE-invariant: $\bar\Psi\Psi$ by (B.4), the contraction of the current with itself because the current is a vector, and
$\sum_\iota(\Pi^\iota)^\dagger\Pi^\iota$ because $\Pi^\iota$ is a Lorentz scalar that rotates as a vector under isospin and changes only
by the phase $e^{2i\theta^{\rm F}_{\rm LL}}$ under $U(1)^{\rm F}_{\rm LL}$.

**The field equations.** Varying $\bar\Psi_{\rm LL}$ gives the field equation of the brochon,
$$i\gamma^p_{\rm LL}\nabla^{\rm GEE}_p\Psi_{\rm LL}-m_{\rm LL}\Psi_{\rm LL}=G^{\rm S}_{\rm SF}(\bar\Psi\Psi)\Psi+G^{\rm V}_{\rm SF}\,\eta^{\rm LL}_{pp}j^{{\rm F},p}_{\rm SF}\gamma^p_{\rm LL}\Psi-\tfrac12G_{\rm LL}\sum_\iota\gamma^0_{\rm LL}(\mathsf K^\iota_{\rm LL})^\dagger\Psi^*\,\Pi^\iota_{\rm LL}\tag{B.11}$$
($\Psi=\Psi_{\rm LL}$, $\Psi^*$ the column of the adjoint operators; as operator equations the factors stand in the order written).
The pairing term is the derivative of $-\tfrac14G_{\rm LL}(\Pi^\iota)^\dagger\Pi^\iota$ with respect to $\bar\Psi$: since
$(\Pi^\iota)^\dagger=\sum_{ab}(\mathsf K^\iota)^*_{ab}\Psi^\dagger_b\Psi^\dagger_a$, its derivative with respect to $\Psi^\dagger_e$ is
$2((\mathsf K^\iota)^\dagger\Psi^*)_e$, and $\partial/\partial\bar\Psi=\gamma^0\partial/\partial\Psi^\dagger$. The scalar term acts like a
mass that depends on the field, the vector term like a gauge potential coupled to the fermion number, and the pairing term
couples $\Psi$ to $\Psi^*$: it conserves the fermion number as a whole but not the number of particles in a given state, and it
is the term that can produce a condensate of pairs. Varying the gauge fields gives the **Yang-Mills equations**
$$\partial_pW^{\iota,pp'}_{\rm SF}-g^{\rm v}_{\rm LL}\sum_{\iota'\iota''}\epsilon^{\rm LL}_{\iota\iota'\iota''}W^{\iota'}_{{\rm SF},p}W^{\iota'',pp'}_{\rm SF}=g^{\rm v}_{\rm LL}j^{\iota,p'}_{\rm SF},\qquad\partial_pB^{pp'}_{\rm SF}=g^{\rm F}_{\rm LL}j^{{\rm F},p'}_{\rm SF}.\tag{B.12}$$
Because the contact terms commute with $F_{\rm LL}$ and $T^\iota_{\rm LL}$, the field equations conserve the currents (Noether's
theorem): $\partial_pj^{{\rm F},p}_{\rm SF}=0$, and the isospin currents are conserved covariantly.

## B.4 The mean field and the three methods

**Quasi-free states.** The field equation (B.11) is an equation for operators and cannot be solved exactly. All three methods
of the model approximate the state of the brochon by a *quasi-free state*: a state in which every expectation value of a
product of field operators is given by Wick's theorem in terms of the two correlation matrices (the ground state of any
Hamiltonian that is quadratic in the field is of this kind, the normal vacuum among them). For a contact term, Wick's theorem
gives three *contractions*: for two $8\times8$ matrices $\mathsf X$, $\mathsf Y$ and all fields at one point,
$$\big\langle{:}(\Psi^\dagger\mathsf X\Psi)(\Psi^\dagger\mathsf Y\Psi){:}\big\rangle=\mathrm{tr}(\mathsf X\,\delta\mathsf C^{\rm n})\,\mathrm{tr}(\mathsf Y\,\delta\mathsf C^{\rm n})-\mathrm{tr}(\mathsf X\,\delta\mathsf C^{\rm n}\,\mathsf Y\,\delta\mathsf C^{\rm n})-\mathrm{tr}\big((\mathsf C^{\rm p})^*\mathsf X\,\mathsf C^{\rm p}\,\mathsf Y^T\big),\tag{B.13}$$
with $\delta\mathsf C^{\rm n}=\mathsf C^{\rm n}_{\rm LL}-\mathsf C^{\rm vac}_{\rm LL}$ and $\mathsf C^{\rm p}=\mathsf C^{\rm p}_{\rm LL}$, the local matrices (the integrals
over the momenta below the cutoff). The three terms are the *direct* (Hartree) contraction, in which each bilinear is replaced
by its average; the *exchange* (Fock) contraction, in which a field of one bilinear is paired with a field of the other; and
the *pair* (Bogoliubov) contraction, in which two annihilation operators are paired with each other.

**Self-energies and the Bogoliubov-de Gennes matrix.** The energy per volume of a uniform quasi-free state is then a function
of its correlation matrices. Its derivatives are the *self-energies* of the state: the normal one $\mathsf N_{\rm SF}$ (Hermitian)
and the pair one $\mathsf P_{\rm SF}$ (antisymmetric), defined by the first variation
$\delta\mathcal E=\mathrm{tr}(\mathsf N_{\rm SF}\,\delta\mathsf C^{\rm n})-\mathrm{Re}\,\mathrm{tr}(\mathsf P_{\rm SF}(\delta\mathsf C^{\rm p})^\dagger)$. They play
the role of a potential that the brochon feels from all the others, and enter the **Bogoliubov-de Gennes matrix** of the state
at the momentum $\vec q$,
$$\mathsf H_{\rm SF}(\vec q)=\begin{pmatrix}\mathsf h_{\rm LL}(\vec q)+\mathsf N_{\rm SF}&\mathsf P_{\rm SF}\\\mathsf P_{\rm SF}^\dagger&-\big(\mathsf h_{\rm LL}(-\vec q)+\mathsf N_{\rm SF}\big)^T\end{pmatrix},\tag{B.14}$$
the matrix (A.15) with $-\mu_{\rm LL}$ replaced by the normal self-energy. A *self-consistent mean-field state* is a quasi-free
state that, at every momentum, occupies exactly the negative eigenvalues of the matrix (B.14) built with its own self-energies;
the equations that express this are its *gap equations*. The ground state is the self-consistent state of lowest energy.
The programs find the self-consistent states by iteration on a grid of momenta (24 Gauss-Legendre nodes in $|\vec q|$ times the
six directions $\pm\vec e_j$, exact for these states; the self-energies are mixed with the previous ones until they change by
less than $10^{-12}$). They start from seven different initial states (the normal vacuum, a polar and a non-unitary pair, and
raised and lowered Dirac masses) and compare the stationary states reached from them.

**The three methods** differ in which contractions of each term they keep, with weights $w^{\rm H}_{\rm SF}$ (direct),
$w^{\rm F}_{\rm SF}$ (exchange) and $w^{\rm P}_{\rm SF}$ (pair):

* **Hubbard-Stratonovich (HS).** Each four-fermion term is rewritten exactly as the exchange of an auxiliary boson field: for
  example $\exp(-i\int\tfrac12G^{\rm S}_{\rm SF}(\bar\Psi\Psi)^2)$ is, up to a constant, the Gaussian integral over a real field
  $\sigma^{\rm HS}_{\rm SF}$ of $\exp(i\int[(\sigma^{\rm HS}_{\rm SF})^2/(2G^{\rm S}_{\rm SF})-\sigma^{\rm HS}_{\rm SF}\bar\Psi\Psi])$ (complete the square), so
  that the brochon appears quadratically, coupled to the auxiliary field like a shift of its mass. The mean field replaces the
  auxiliary fields by constants. A four-fermion term can be decoupled in a density channel or in a pair channel (the Fierz
  identities rewrite it as a sum over channels); the *HS parameters* $s^{\rm HS,S}_{\rm SF}$ and $s^{\rm HS,V}_{\rm SF}$, between 0 and 1,
  are the shares of the scalar and of the vector term decoupled in the density channels. The weights are
  $(s^{\rm HS,S}_{\rm SF},0,1-s^{\rm HS,S}_{\rm SF})$ for the scalar term, $(s^{\rm HS,V}_{\rm SF},0,1-s^{\rm HS,V}_{\rm SF})$ for the vector term and
  $(0,0,1)$ for the pairing term: HS keeps no exchange contraction. "HS $(1,1)$" denotes the usual choice
  $s^{\rm HS,S}_{\rm SF}=s^{\rm HS,V}_{\rm SF}=1$.
* **Kohn-Sham (KS).** The energy of the ground state is written as a functional of the local densities (the number density,
  the scalar density and the pair amplitude) and minimized through an auxiliary system of independent brochons in local
  potentials, the Kohn-Sham potentials (a Dirac mass, a vector potential and a pair potential). All weights are 1, but the
  exchange contraction is taken in the local-density approximation: $\delta\mathsf C^{\rm n}$ in it is replaced by its projection
  $\tfrac18\mathrm{tr}(\delta\mathsf C^{\rm n})\mathsf 1_8+\tfrac18\mathrm{tr}(\beta\,\delta\mathsf C^{\rm n})\beta$ onto the matrices of the two
  densities ($\beta=\gamma^0_{\rm LL}\otimes\mathsf 1_2$).
* **Cornwall-Jackiw-Tomboulis (CJT).** The effective action of the propagator of the field, the sum of the one-loop term and of
  all two-particle-irreducible vacuum diagrams, is stationary at the exact propagator. In its two-loop approximation the diagrams
  are the three contractions of every contact term, with all weights 1 and no projection, and the exchange of one gauge boson
  (Section B.6).

**Theorem (KS and CJT for uniform states).** *For a uniform state whose $\delta\mathsf C^{\rm n}$ is a combination of $\mathsf 1_8$ and
$\beta$, KS and CJT have the same energy and the same self-energies.* *Proof.* For such a state the projection of KS acts as the
identity, and the exchange self-energies stay in the same span: $\mathsf X\delta\mathsf C^{\rm n}\mathsf X$ for $\mathsf X=\beta$,
$\mathsf 1_8$, $\alpha^j$ (with $\alpha^j\beta\alpha^j=-\beta$), and $(\mathsf K^\iota)^\dagger(\delta\mathsf C^{\rm n})^T\mathsf K^\iota$ (with
$(\mathsf K^\iota)^\dagger\mathsf K^\iota=\mathsf 1_8$ and $(\mathsf K^\iota)^\dagger\beta^T\mathsf K^\iota=\beta$). $\square$ The normal vacuum and the
polar superfluid of Section B.5 are such states.

**The effective couplings.** Two numbers decide the fate of the normal vacuum in each method $X$. A uniform scalar density
$n^{\rm S}_{\rm SF}$ shifts the Dirac mass by $g^{X,{\rm S}}_{\rm SF}n^{\rm S}_{\rm SF}$, and a pair amplitude in the polar channel
$\mathsf K^3_{\rm LL}$ produces the pair self-energy $D^X_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$ with
$D^X_{\rm SF}=-\tfrac12g^{X,{\rm P}}_{\rm SF}\mathrm{tr}(\mathsf K^3_{\rm LL}(\mathsf C^{\rm p})^T)$. Collecting the contractions of the three terms,
$$\begin{aligned}g^{{\rm HS},{\rm S}}_{\rm SF}&=s^{\rm HS,S}_{\rm SF}G^{\rm S}_{\rm SF},&g^{{\rm KS},{\rm S}}_{\rm SF}&=g^{{\rm CJT},{\rm S}}_{\rm SF}=\tfrac78G^{\rm S}_{\rm SF}-\tfrac12G^{\rm V}_{\rm SF}-\tfrac38G_{\rm LL},\\g^{{\rm HS},{\rm P}}_{\rm SF}&=G_{\rm LL}-\tfrac14(1-s^{\rm HS,S}_{\rm SF})G^{\rm S}_{\rm SF}-(1-s^{\rm HS,V}_{\rm SF})G^{\rm V}_{\rm SF},&g^{{\rm KS},{\rm P}}_{\rm SF}&=g^{{\rm CJT},{\rm P}}_{\rm SF}=G_{\rm LL}-\tfrac14G^{\rm S}_{\rm SF}-G^{\rm V}_{\rm SF}.\end{aligned}\tag{B.15}$$
In the scalar channel, the direct contraction of the scalar term gives $G^{\rm S}_{\rm SF}$ and its exchange contraction
$-\tfrac18G^{\rm S}_{\rm SF}$; the exchange contractions of the vector term give $-\tfrac12G^{\rm V}_{\rm SF}$ and those of the three pairing
terms $-\tfrac18G_{\rm LL}$ each. In the pair channel the pairing term gives $G_{\rm LL}$, and the pair contractions of the scalar and
of the vector term act like pairing terms of the strengths $-\tfrac14G^{\rm S}_{\rm SF}$ and $-G^{\rm V}_{\rm SF}$ (by the Fierz identities;
the programs compute all these numbers from the matrices of the terms).

**The stability of the normal vacuum.** A small shift $\delta M$ of the Dirac mass changes the scalar density of the normal
vacuum by $-\chi^{\rm S}_{\rm SF}\delta M$, with the *scalar susceptibility*
$$\chi^{\rm S}_{\rm SF}=\frac2{\pi^2}\int_0^{\Lambda_{\rm LL}}\frac{q^4\,dq}{(q^2+m_{\rm LL}^2)^{3/2}}={{GENM:p2_chiS}}\tag{B.16}$$
(the derivative of the scalar density $\int d^3q/(2\pi)^3\,(-4M/(q^2+M^2)^{1/2}+4m_{\rm LL}/\varepsilon_{\rm LL})$ of a free state of Dirac
mass $M$), and the shift feeds back as $\delta M=g^{X,{\rm S}}_{\rm SF}\delta n^{\rm S}_{\rm SF}$. So the normal vacuum is unstable towards a
*scalar condensate* (a Dirac mass different from $m_{\rm LL}$) when $\lambda^{\rm S}_{\rm SF}=-g^{X,{\rm S}}_{\rm SF}\chi^{\rm S}_{\rm SF}>1$. In the
pair channel the gap equation of Appendix A.6 at a vanishing gap gives the eigenvalue
$\lambda^{\rm P}_{\rm SF}=g^{X,{\rm P}}_{\rm SF}/G^{\rm crit}_{\rm LL}$ with the critical coupling (A.19),
$G^{\rm crit}_{\rm LL}=\pi^2/I^{(1)}_{\rm SF}(m_{\rm LL})={{GENM:p2_Gcrit}}$: the normal vacuum is unstable towards pairing when
$\lambda^{\rm P}_{\rm SF}>1$ (the *Thouless criterion*).

## B.5 The recursion of the interaction and the polar ground state

The author's problem asked to change the interaction step by step, together with the HS parameters and the Kohn-Sham
potentials, until all three methods give a superfluid ground state. Three steps were needed.

**Step 0: the author's $V^{(0)}_{\rm SF}$** ($G^{\rm S}_{\rm SF}=-8$, $G^{\rm V}_{\rm SF}=0.8$, no pairing term). By (B.15) in CJT,
$g^{{\rm CJT},{\rm S}}_{\rm SF}={{GENM:p2_gS_V0_CJT}}$ and $g^{{\rm CJT},{\rm P}}_{\rm SF}={{GENM:p2_gP_V0_CJT}}$, so $\lambda^{\rm S}_{\rm SF}={{GENM:p2_lamS_V0_CJT}}$
and $\lambda^{\rm P}_{\rm SF}={{GENM:p2_lamP_V0_CJT}}$: both channels are stable. Every start of every method ends in the normal vacuum:
**$V^{(0)}_{\rm SF}$ has no superfluid in any method.**

**Step 1: $V^{(1)}_{\rm SF}$** adds the pairing term with $G_{\rm LL}=22.5$, above $G^{\rm crit}_{\rm LL}$. In HS the pair channel becomes
unstable and the ground state is a polar superfluid. In KS and CJT, however, the exchange contraction of the pairing term acts in
the scalar channel as the attraction $-\tfrac38G_{\rm LL}$, and the scalar coupling becomes
$g^{{\rm CJT},{\rm S}}_{\rm SF}={{GENM:p2_t_gS_V1_CJT}}$, so $\lambda^{\rm S}_{\rm SF}={{GENM:p2_t_lamS_V1_CJT}}>1$: the ground state is a *massive
state*, a scalar condensate with the Dirac mass ${{GENM:p2_t_M0_V1_CJT}}$. With so large a mass the pair channel is no longer
critical, because the critical coupling grows with the mass: at this mass it is $\pi^2/I^{(1)}_{\rm SF}(|M|)={{GENM:p2_t_Gcrit_M0_V1}}$,
more than $g^{{\rm CJT},{\rm P}}_{\rm SF}=23.7$. **$V^{(1)}_{\rm SF}$ has a superfluid in HS but not in KS and CJT.** (In the methods that keep the
exchange contraction, an attraction in the pair channel brings with it, by the Fierz identities, an attraction in the scalar
channel.)

**Step 2: $V^{(2)}_{\rm SF}$** keeps the form of $V^{(1)}_{\rm SF}$ and the vector coupling, and chooses $G^{\rm S}_{\rm SF}$ and $G_{\rm LL}$ so that
in KS and CJT the scalar channel has the stable coupling of $V^{(0)}_{\rm SF}$ and the pair channel the supercritical coupling of
$V^{(1)}_{\rm SF}$: $g^{{\rm CJT},{\rm S}}_{\rm SF}=-7.4$ and $g^{{\rm CJT},{\rm P}}_{\rm SF}=23.7$. By (B.15) these are the two linear equations
$\tfrac78G^{\rm S}_{\rm SF}-\tfrac38G_{\rm LL}=-7.4+0.4$ and $-\tfrac14G^{\rm S}_{\rm SF}+G_{\rm LL}=23.7+0.8$, with the solution
$G^{\rm S}_{\rm SF}={{GENM:p2_V2_GS}}$ and $G_{\rm LL}={{GENM:p2_V2_G}}$:
$$V^{(2)}_{\rm SF}=\tfrac12(2.8):(\bar\Psi_{\rm LL}\Psi_{\rm LL})^2:+\tfrac12(0.8)\sum_p\eta^{\rm LL}_{pp}:(j^{{\rm F},p}_{\rm SF})^2:-\tfrac14(25.2)\sum_\iota:(\Pi^\iota_{\rm LL})^\dagger\Pi^\iota_{\rm LL}:.\tag{B.17}$$
The explicit scalar term is now repulsive; in KS and CJT the scalar attraction of the author's ansatz is supplied by the exchange
contraction of the pairing term. The normal vacuum is unstable only in the pair channel, in every method
($\lambda^{\rm P}_{\rm SF}$ from {{GEN:p2_t_lamP_V2}}; in KS and CJT $\lambda^{\rm P}_{\rm SF}={{GENM:p2_lamP_V2_CJT}}$ and
$\lambda^{\rm S}_{\rm SF}={{GENM:p2_lamS_V2_CJT}}$), and **the ground state of every method is a polar superfluid.** The recursion ends here.

**The polar ground state.** In every method the ground state of $V^{(2)}_{\rm SF}$ has the normal self-energy
$(M^{X,{\rm D}}_{\rm SF}-m_{\rm LL})\,\beta$, a change of the Dirac mass from $m_{\rm LL}$ to $M^{X,{\rm D}}_{\rm SF}$ (the number density vanishes),
and the pair self-energy $D^X_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$: the pairs are condensed in the polar channel, each joining an up
with a down brochon. The Bogoliubov-de Gennes matrix (B.14) then has, at every momentum, the eigenvalues $\pm E(\vec q)$ eight
times each, with the **quasi-particle energy**
$$E(\vec q)=\big(|\vec q|^2+(M^{X,{\rm D}}_{\rm SF})^2+|D^X_{\rm SF}|^2\big)^{1/2},\tag{B.18}$$
as for (A.15) with $\mu_{\rm LL}=0$; the **gap**, the smallest energy of a quasi-particle, is
$E^{X,{\rm gap}}_{\rm SF}=((M^{X,{\rm D}}_{\rm SF})^2+|D^X_{\rm SF}|^2)^{1/2}$, at $\vec q=0$. The pair self-consistency is the gap equation (A.18) with
the coupling $g^{X,{\rm P}}_{\rm SF}$ and the effective mass $E^{X,{\rm gap}}_{\rm SF}$,
$$1=\frac{g^{X,{\rm P}}_{\rm SF}}{\pi^2}\,I^{(1)}_{\rm SF}\big(E^{X,{\rm gap}}_{\rm SF}\big),\tag{B.19}$$
so the pair coupling alone fixes the gap: the scalar term shifts the Dirac mass, and the pair amplitude adjusts so that
$M^2+|D|^2$ stays fixed. The numbers (energies per volume measured from the normal vacuum, in units of $\Lambda_{\rm LL}^4$):

* **KS and CJT** (equal by the theorem of Section B.4): $\mathcal E^{\rm CJT}_{\rm SF}={{GENM:p2_E_V2_CJT}}$,
  $|D^{\rm CJT}_{\rm SF}|={{GENM:p2_D_V2_CJT}}$, $M^{\rm CJT,D}_{\rm SF}={{GENM:p2_M_V2_CJT}}$, $E^{\rm CJT,gap}_{\rm SF}={{GENM:p2_gap_V2_CJT}}$;
* **HS (1, 1)**: $\mathcal E^{\rm HS}_{\rm SF}={{GENM:p2_E_V2_HS_1_1}}$, $|D^{\rm HS}_{\rm SF}|={{GENM:p2_D_V2_HS_1_1}}$,
  $M^{\rm HS,D}_{\rm SF}={{GENM:p2_M_V2_HS_1_1}}$, $E^{\rm HS,gap}_{\rm SF}={{GENM:p2_gap_V2_HS_1_1}}$.

The main text uses the state of CJT. The states of non-zero non-unitarity ($\zeta_{\rm LL}>0$ of Appendix A.6) are stationary too,
but lie higher in energy in every method.

**Example: the polar condensate and its quasi-particles.** The condensate of the CJT ground state is the state whose pair
self-energy is $D^{\rm CJT}_{\rm SF}(\mathsf K^3_{\rm LL})^\dagger$ with $|D^{\rm CJT}_{\rm SF}|={{GENM:lgw_lw6_d}}$; the matrix $\mathsf K^3_{\rm LL}$ has
the non-zero entries {{GEN:ex10_k3_entries}} (rows and columns numbered from 0 to 7, the component number minus 1), so each
condensed pair joins an isospin-up with an isospin-down component of the same chirality and of opposite spin. Built with these self-energies,
the $16\times16$ matrix (B.14) has, at every momentum the program tried, the eigenvalues $\pm E(\vec q)$ of (B.18) eight times
each (check LW-6a of the program `lg_worked` of this folder, Appendix C.4). A quasi-particle at rest has the
energy $E(0)={{GENM:lgw_lw6_e_q0}}$, the gap; one with $|\vec q|=0.3$ has $E={{GENM:lgw_lw6_e_q0_3}}$, and one at the cutoff
$E={{GENM:lgw_lw6_e_q1}}$. The gap equation (B.19) holds: $I^{(1)}_{\rm SF}(E^{\rm CJT,gap}_{\rm SF})={{GENM:lgw_lw7_i1_gap}}$ by the
closed form (A.19), equal to $\pi^2/23.7={{GENM:lgw_lw7_pi2_over_gp}}$ (check LW-7b).

**The excitations.** A state with an odd fermion number contains at least one quasi-particle, so the lowest such state lies the
gap above the ground state. A state of zero total momentum and even fermion number can contain two quasi-particles of opposite
momenta; their energies form a continuum that begins at the *threshold* $2E^{\rm CJT,gap}_{\rm SF}={{GENM:p2_thr_CJT}}$. Below the
threshold lie the *collective modes*, the small oscillations of the condensate as a whole, found as the frequencies at which
the linearized mean-field equations have a self-consistent solution:

* three **Goldstone modes** of zero frequency, one for each generator of GEE that the condensate breaks (Section B.6); the
  condensate turns along them at no cost of energy;
* the **amplitude mode**, the oscillation of the size $|D|$ of the condensate (in a superconductor it is called the Higgs mode),
  with a share of the scalar density, at ${{GENM:p2_mode1_CJT}}$;
* the **non-unitary doublet**, two oscillations of the direction of the pair vector that make the condensate non-unitary, at
  ${{GENM:p2_mode2_CJT}}$; they carry the charges $Q_{\rm LL}=\pm1$.

## B.6 The breaking of the symmetry, the gauge bosons and the zeugon

**The pair field.** The condensate is described by the complex isospin vector $\vec\Phi_{\rm SF}$ of the amplitudes of the pair
self-energy, $\mathsf P_{\rm SF}=\sum_\iota\Phi^\iota_{\rm SF}(\mathsf K^\iota_{\rm LL})^\dagger$; for the ground state $\vec\Phi_{\rm SF}=D^X_{\rm SF}\vec e_3$. Under GEE it is
a Lorentz scalar, an isovector, and it takes the phase $e^{2i\theta^{\rm F}_{\rm LL}}$ (fermion number 2), so its covariant derivative is
$$\nabla^{\rm GEE}_p\vec\Phi_{\rm SF}=\partial_p\vec\Phi_{\rm SF}+2ig^{\rm F}_{\rm LL}B_{{\rm SF},p}\vec\Phi_{\rm SF}-g^{\rm v}_{\rm LL}\vec W_{{\rm SF},p}\times\vec\Phi_{\rm SF},\tag{B.20}$$
with $\vec W_{{\rm SF},p}=(W^1_{{\rm SF},p},W^2_{{\rm SF},p},W^3_{{\rm SF},p})$. At long wavelengths the condensate obeys an effective Lagrangian of
the pair field: a kinetic term built from (B.20), and a potential term, minus the energy per volume of the uniform state with a
given $\vec\Phi_{\rm SF}$. As a function of the complex amplitude of a polar field this energy has the shape of a "Mexican hat": a
maximum at $\vec\Phi_{\rm SF}=0$, the normal vacuum, and a ring of minima at $|\vec\Phi_{\rm SF}|=|D^X_{\rm SF}|$.

**The vacua and the surviving symmetry.** The energy depends only on $|\vec\Phi|^2$ and $|\vec\Phi\cdot\vec\Phi|^2$, and its minima are
the polar fields of modulus $|D|$: $|D|e^{i\phi^{\rm vac}_{\rm SF}}\vec n$ with a real phase $\phi^{\rm vac}_{\rm SF}$ and a real unit vector
$\vec n$, where $(\phi^{\rm vac}_{\rm SF},\vec n)$ and $(\phi^{\rm vac}_{\rm SF}+\pi,-\vec n)$ give the same field. GEE moves every vacuum into every other (the phase by
$U(1)^{\rm F}_{\rm LL}$, the direction by isospin rotations), so every vacuum is equivalent to $|D|\vec e_3$. The elements of GEE that
leave $|D|\vec e_3$ unchanged form the group
$$H^{\rm vac}_{\rm SF}=SL(2,\mathbb C)\times O(2),\qquad O(2)=U(1)^{\rm Q}_{\rm LL}\rtimes\{\mathsf 1,\mathsf r\},\qquad U(1)^{\rm Q}_{\rm LL}=\{e^{i\theta_{\rm r}Q_{\rm LL}}\},\tag{B.21}$$
where $e^{i\theta_{\rm r}Q_{\rm LL}}$ are the isospin rotations about $\vec e_3$ by the angle $\theta_{\rm r}$ and $\mathsf r$ is the rotation by $\pi$ about $\vec e_1$ (which
turns $\vec e_3$ into $-\vec e_3$) combined with the phase $\theta^{\rm F}_{\rm LL}=\pi/2$ (which multiplies the field by $-1$). The
symmetry is broken as
$$\mathrm{GEE}=SL(2,\mathbb C)\times SU(2)_{\rm v}\times U(1)^{\rm F}_{\rm LL}\ \longrightarrow\ SL(2,\mathbb C)\times O(2):\tag{B.22}$$
three generators are broken ($T^1_{\rm LL}$, $T^2_{\rm LL}$ and $F_{\rm LL}$, which give the three Goldstone modes), and the Lorentz group
is unbroken because the condensate has spin zero. Each pair of the condensate has $Q_{\rm LL}=+\tfrac12-\tfrac12=0$, so the
condensate is neutral under $U(1)^{\rm Q}_{\rm LL}$.

**The masses of the gauge bosons.** A constant time component $W^\iota_{{\rm SF},0}$ of a gauge field enters the Hamiltonian like a
chemical potential $\mu^\iota_{\rm SF}=-g^{\rm v}_{\rm LL}W^\iota_{{\rm SF},0}$ of the charge $T^\iota_{\rm LL}$, and $B_{{\rm SF},0}$ like a chemical potential
$\mu^{\rm F}_{\rm SF}=-g^{\rm F}_{\rm LL}B_{{\rm SF},0}$ of the fermion number. The energy of the ground state falls by
$\tfrac12\sum_{ab}\chi^{ab}_{\rm SF}\mu^a_{\rm SF}\mu^b_{\rm SF}$, with the static susceptibilities
$\chi^{ab}_{\rm SF}=\partial\langle\text{charge }a\rangle/\partial\mu^b_{\rm SF}$ ($a,b\in\{1,2,3,{\rm F}\}$), and minus this energy is the
time-time part of a mass term $\tfrac12\sum_{ab}\mathsf M_{{\rm SF},ab}\,\eta^{\rm LL}_{pp}\mathcal W^a_p\mathcal W^b_p$ of the gauge fields
$(\mathcal W^1,\mathcal W^2,\mathcal W^3,\mathcal W^{\rm F})=(W^1_{\rm SF},W^2_{\rm SF},W^3_{\rm SF},B_{\rm SF})$. Hence the **mass matrix**
$$\mathsf M_{{\rm SF},ab}=g_ag_b\,\chi^{ab}_{\rm SF},\qquad g_1=g_2=g_3=g^{\rm v}_{\rm LL},\quad g_{\rm F}=g^{\rm F}_{\rm LL}.\tag{B.23}$$
In the normal vacuum, which has a gap and no charge, every susceptibility vanishes and all four gauge bosons are massless. In
the polar superfluid the matrix is diagonal and $\chi^{33}_{\rm SF}=0$, because the condensate is neutral under $U(1)^{\rm Q}_{\rm LL}$ and
has a gap, so that a small chemical potential of $Q_{\rm LL}$ does not change it. In the CJT solution with the exchange of the
gauge bosons (below), $\chi^{11}_{\rm SF}=\chi^{22}_{\rm SF}={{GENM:p2_chi11_CJT_exchange}}$ and $\chi^{\rm FF}_{\rm SF}={{GENM:p2_chiFF_CJT_exchange}}$,
and the mass eigenstates are:

* $W^\pm_{{\rm SF},p}=(W^1_{{\rm SF},p}\mp iW^2_{{\rm SF},p})/\sqrt2$, of the charges $Q_{\rm LL}=\pm1$ and the mass
  $M^{\rm CJT}_{W,{\rm SF}}=g^{\rm v}_{\rm LL}(\chi^{11}_{\rm SF})^{1/2}={{GENM:p2_cjtex_M_W}}$;
* $Z_{{\rm SF},p}=B_{{\rm SF},p}$, of the mass $M^{\rm CJT}_{Z,{\rm SF}}=g^{\rm F}_{\rm LL}(\chi^{\rm FF}_{\rm SF})^{1/2}={{GENM:p2_cjtex_M_Z}}$;
* $A_{{\rm SF},p}=W^3_{{\rm SF},p}$, the gauge field of $U(1)^{\rm Q}_{\rm LL}$, massless: the program finds $M^{\rm CJT}_{A,{\rm SF}}={{GENM:p2_cjtex_M_A}}$,
  zero to its accuracy. It does not mix with $B_{\rm SF}$, because the condensate has $Q_{\rm LL}=0$ but the fermion number 2.

The three massive bosons have taken up the three Goldstone modes as their longitudinal polarizations (the Higgs mechanism), and
their range is about the inverse of their masses, $1/M^{\rm CJT}_{W,{\rm SF}}\approx23/\Lambda_{\rm LL}$.

**The zeugon.** The massless gauge boson $A_{{\rm SF},p}$ of the superfluid is the **zeugon**. It couples to the charge
$Q_{\rm LL}=T^3_{\rm LL}$ with the coupling $g^{\rm v}_{\rm LL}$, so it acts on the brochon (charges $\pm\tfrac12$), on $W^\pm_{\rm SF}$ (charges
$\pm1$) and on the non-unitary doublet (charges $\pm1$). It is not the photon: it couples to a charge of $SU(2)_{\rm v}$, and nothing
in the model carries electric charge. What the computation establishes about it:

1. *It is massless*, by the neutrality and the gap of the condensate, in every method.
2. *It attracts the two members of a condensed pair, but alone it does not hold them together.* The exchange of a gauge boson
   adds to the energy of a uniform state its exchange and pair contractions with the propagator of the boson. In the CJT solution
   with the exchange of all four bosons (self-consistent masses; the pair amplitude falls to ${{GENM:p2_cjtex_D}}$ and the gap to
   ${{GENM:p2_cjtex_gap}}$), the exchange energy of the zeugon is ${{GENM:p2_cjtex_exchange_energy_W3}}$, an attraction, while $W^1$, $W^2$
   and $B$ repel (${{GENM:p2_cjtex_exchange_energy_W1}}$ each and ${{GENM:p2_cjtex_exchange_energy_B}}$), with the net effect
   ${{GENM:p2_cjtex_exchange_energy}}$. In the linearized gap equation of the normal vacuum the zeugon alone has the eigenvalue
   ${{GENM:p2_thouless3}}\,(g^{\rm v}_{\rm LL})^2$, so it would pair the normal vacuum by itself only for
   $g^{\rm v}_{\rm LL}>{{GENM:p2_thouless7}}$, almost eight times the coupling of the model; with all four bosons, which are all massless in
   the normal vacuum, the eigenvalue is ${{GENM:p2_thouless6}}$: the gauge bosons repel in the symmetric vacuum and cannot create the
   condensate. At $g^{\rm v}_{\rm LL}=0.7$ the condensate is produced by the pairing term of $V^{(2)}_{\rm SF}$.
3. *Its charge conjugation is a gauge symmetry.* The surviving group is $O(2)$, not $U(1)$: its element $\mathsf r$ reverses the
   zeugon and exchanges the charged bosons,
   $$A_{{\rm SF},p}\to-A_{{\rm SF},p},\qquad W^\pm_{{\rm SF},p}\to W^\mp_{{\rm SF},p},\qquad Z_{{\rm SF},p}\to Z_{{\rm SF},p},\tag{B.24}$$
   which never holds for the photon. A $U(1)$ gauge theory with a local charge conjugation is called *Alice electrodynamics*
   (J. Kiskis 1978; A. S. Schwarz 1982; M. Bucher, H.-K. Lo and J. Preskill 1992).

**Example: the force of the zeugon.** Two brochons of the charges $Q_1$ and $Q_2$ at rest at the distance $r_{12}$ exchange the
massless zeugon as two electric charges exchange the photon: their potential energy is $(g^{\rm v}_{\rm LL})^2Q_1Q_2/(4\pi r_{12})$ (in
the Heaviside-Lorentz units of the model). For the two members of a condensed pair, $Q_1=+\tfrac12$ and $Q_2=-\tfrac12$, this is the
attraction $-\alpha^{\rm Z}_{\rm LG}/r_{12}$ with $\alpha^{\rm Z}_{\rm LG}=(g^{\rm v}_{\rm LL})^2/(16\pi)={{GENM:lgp_alpha}}$, a potential of the form of
Coulomb's.

**Which numbers the main text uses.** The quasi-particles of the main text (Section 15.4) are those of the CJT ground state of
$V^{(2)}_{\rm SF}$ without the exchange of the gauge bosons, with the gap $E^{\rm CJT,gap}_{\rm SF}={{GENM:p2_gap_V2_CJT}}$; the exchange
of the gauge bosons at the couplings of the model lowers the gap to ${{GENM:p2_cjtex_gap}}$. The masses of the gauge bosons are those of
the solution with the exchange.

**The names.** The name *zeugon*, from the Greek ζεῦγος, a yoked pair, was chosen in the work on the model, because the zeugon is the
gauge boson that attracts the two brochons of a condensed pair; the author, who asked for the massless gauge boson, left the choice
of its name to the work. The name *brochon* for the field was proposed in the same work and adopted by the author on 21 November 2026.

## B.7 The symbols of this appendix

| symbol | meaning |
|:---|:---|
| $\Psi_{\rm LL}$, $\Psi_{{\rm LL},c}$, $\bar\Psi_{\rm LL}$ | the brochon, its eight components, its Dirac adjoint |
| $d$, $s$, $c$ | the Dirac index, the isospin index (1 up, 2 down) and the component number $2(d-1)+s$ (indices of Section B.1) |
| $SU(2)_{\rm v}$, $U(1)^{\rm F}_{\rm LL}$, GEE | the isospin rotations, the phases of the fermion number, and the gauge group (B.3) |
| $T^\iota_{\rm LL}$, $F_{\rm LL}$, $Q_{\rm LL}$ | the isospin generators, the fermion number, and the charge $T^3_{\rm LL}$ |
| $\mathsf h_{\rm LL}(\vec q)$, $\varepsilon_{\rm LL}$ | the free Dirac Hamiltonian and $(q^2+m_{\rm LL}^2)^{1/2}$ |
| $\mathsf C^{\rm n}_{\rm LL}$, $\mathsf C^{\rm p}_{\rm LL}$, $\mathsf C^{\rm vac}_{\rm LL}$, $\delta\mathsf C^{\rm n}$, $\mathsf C^{\rm p}$ | the normal and the pair correlation matrix, that of the normal vacuum (B.2), and the local matrices of (B.13) |
| $\mathbf g(x)$, $\mathsf U_{\rm LL}(x)$, $\theta^{\rm F}_{\rm LL}(x)$, $\mathcal U_{\rm LL}$ | a local GEE transformation and its matrix (B.4) |
| $\Gamma^{\rm LL}_p$, $\mathsf W_{{\rm SF},p}$, $W^\iota_{{\rm SF},p}$, $B_{{\rm SF},p}$ | the gauge fields: the spin connection, the field of $SU(2)_{\rm v}$ (matrix and components), the field of $U(1)^{\rm F}_{\rm LL}$ |
| $g^{\rm v}_{\rm LL}$, $g^{\rm F}_{\rm LL}$ | the gauge couplings, 0.7 and 0.45 |
| $\nabla^{\rm GEE}_p$, $\mathsf S$ | the covariant derivative (B.5) and $\mathsf S_{\rm LL}(\mathbf g(x))$ |
| $W^\iota_{{\rm SF},pp'}$, $\mathsf W_{{\rm SF},pp'}$, $B_{{\rm SF},pp'}$, $\Omega^{\rm L}_{{\rm SF},pp'}$ | the field strengths (B.7) |
| $\epsilon^{\rm LL}_{\iota\iota'\iota''}$ | the Levi-Civita symbol of three indices |
| $j^{\iota,p}_{\rm SF}$, $j^{{\rm F},p}_{\rm SF}$ | the isospin currents and the fermion-number current (B.8) |
| $\mathcal L_{\rm SF}$, $V^{(n)}_{\rm SF}$ | the Lagrangian density (B.9) and the self-interaction of step $n$ |
| $G^{\rm S}_{\rm SF}$, $G^{\rm V}_{\rm SF}$, $G_{\rm LL}$ | the scalar, the vector and the pairing coupling |
| $G_S$, $G_V$, $n$, $n_S$ | the couplings and the densities of the author's Kohn-Sham ansatz |
| $\Pi^\iota_{\rm LL}$ | the local pair operators |
| $\mathsf X$, $\mathsf Y$ | matrices of two bilinears in (B.13) |
| $\mathcal E$, $\mathcal E^X_{\rm SF}$ | the energy per volume of a uniform state, and of the ground state of the method $X$ |
| $\mathsf N_{\rm SF}$, $\mathsf P_{\rm SF}$, $\mathsf H_{\rm SF}(\vec q)$ | the normal and the pair self-energy, and the Bogoliubov-de Gennes matrix (B.14) |
| $w^{\rm H}_{\rm SF}$, $w^{\rm F}_{\rm SF}$, $w^{\rm P}_{\rm SF}$ | the weights of the direct, the exchange and the pair contraction |
| $\sigma^{\rm HS}_{\rm SF}$, $s^{\rm HS,S}_{\rm SF}$, $s^{\rm HS,V}_{\rm SF}$ | the auxiliary scalar field of HS and the HS parameters |
| $\beta$, $\alpha^j$ | $\gamma^0_{\rm LL}\otimes\mathsf 1_2$ and $\gamma^0_{\rm LL}\gamma^j_{\rm LL}\otimes\mathsf 1_2$ |
| $X$ | a method: HS, KS or CJT |
| $g^{X,{\rm S}}_{\rm SF}$, $g^{X,{\rm P}}_{\rm SF}$ | the effective couplings of the scalar and of the pair channel (B.15) |
| $n^{\rm S}_{\rm SF}$, $\delta M$ | a uniform scalar density and a shift of the Dirac mass |
| $\chi^{\rm S}_{\rm SF}$, $\lambda^{\rm S}_{\rm SF}$, $\lambda^{\rm P}_{\rm SF}$ | the scalar susceptibility (B.16) and the eigenvalues of the static response in the two channels |
| $M^{X,{\rm D}}_{\rm SF}$, $D^X_{\rm SF}$, $E^{X,{\rm gap}}_{\rm SF}$ | the Dirac mass, the pair amplitude and the gap of the polar ground state |
| $E(\vec q)$ | the quasi-particle energy (B.18) |
| $\vec\Phi_{\rm SF}$, $\Phi^\iota_{\rm SF}$ | the pair field and its components (B.20) |
| $\phi^{\rm vac}_{\rm SF}$, $\vec n$, $\vec e_\iota$ | the phase and the isospin direction of a vacuum; the unit vectors of isospin space |
| $H^{\rm vac}_{\rm SF}$, $O(2)$, $U(1)^{\rm Q}_{\rm LL}$, $\mathsf r$ | the surviving group (B.21), its gauge part, its identity component and its charge conjugation |
| $\mu^a_{\rm SF}$, $\chi^{ab}_{\rm SF}$, $\mathsf M_{{\rm SF},ab}$, $\mathcal W^a_p$, $g_a$ | the chemical potentials of the four charges, their susceptibilities, the mass matrix (B.23), the four gauge fields and their couplings |
| $W^\pm_{{\rm SF},p}$, $Z_{{\rm SF},p}$, $A_{{\rm SF},p}$ | the massive charged bosons, the massive neutral boson and the zeugon |
| $M^{\rm CJT}_{W,{\rm SF}}$, $M^{\rm CJT}_{Z,{\rm SF}}$, $M^{\rm CJT}_{A,{\rm SF}}$ | their masses in CJT |
| $Q_1$, $Q_2$, $r_{12}$, $\alpha^{\rm Z}_{\rm LG}$ | two charges, their distance, and the strength $(g^{\rm v}_{\rm LL})^2/(16\pi)$ of the zeugon potential |
| $u_{\rm LL}$, $P_{\rm LG}$ | the spinor of Appendix A.5 and the momentum $(\tfrac{17}8,0,0,\tfrac{15}8)$ of the plane wave of the example |
