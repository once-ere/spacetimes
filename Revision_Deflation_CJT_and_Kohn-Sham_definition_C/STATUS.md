---
title: "Status of the work in the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C"
author: "PROJECT spacetimes (author of the project: Patrick Lee Nash)"
date: "13 November 2026, 15:40 Pacific time"
---

# 1. What this folder is

On 13 November 2026 the author of the project decided that, in a new folder, the
self-interaction of the Dirac field on the $4+4$ spacetime is to be *defined* by the
covariant form, that both forms are to be recorded correctly and completely, and that
the two options are to be explained and discussed in the documents and in the
provenance files. This is that folder. It is **work in progress**: this file states
exactly what the folder contains at the time written above and what is still missing.
The folder `Revision_Deflation_CJT_and_Kohn-Sham` of the same repository, in which the
other form is the definition, is complete and is not changed by this work.

# 2. The two forms, in one page

The Dirac field $\psi$ has sixteen components. The Dirac matrices $\gamma^A$
($A=0,\dots,7$) of the $4+4$ spacetime are real $16\times16$ matrices; those of the
four space-like coordinates $x0,\dots,x3$ square to $+1$, those of the four time-like
coordinates $x4,\dots,x7$ to $-1$. Three matrices are built from them:
$C^-=\gamma^0\gamma^1\gamma^2\gamma^3$ (real, symmetric, square $1$),
$\beta=-i\gamma^4$ (Hermitian, square $1$), and $W=iC^-\gamma^4$ (Hermitian, square
$1$), so that $C^-=-W\beta$. For fields that do not depend on the three additional
times $x5,x6,x7$ (the *quantized sector*) the kinetic and mass terms are taken in the
positive form $\psi^\dagger(i\partial_4-h)\psi$, with the Hamiltonian matrix
$h=\sum_{j=0}^{3}\alpha^j(-i\partial_j)+\beta m$, $\alpha^j=-\gamma^4\gamma^j$; $W$
commutes with $h$, and the field consists of two halves, $W=+1$ and $W=-1$.

The self-interaction is $\mathcal L_{\rm self}=-\tfrac12\sum_{r=0}^{8}g_r\,I_r$ with
nine real couplings $g_r$ and nine invariants $I_r$, one for each number $r$ of
different indices in a product of Dirac matrices. The two forms differ in the
bilinears from which the invariants are built:

* **Form C (covariant form; the definition in this folder).** The bilinears are
  $\tilde\psi\,\Theta^K\psi$ with $\tilde\psi=\psi^\dagger C^-$ and
  $\Theta^K=\zeta_r\,\gamma^{A_1}\cdots\gamma^{A_r}$, where $\zeta_r=i$ for
  $r=1,2,5,6$ and $\zeta_r=1$ otherwise;
  $I_r=\sum_{|K|=r}(\tilde\psi\,\Theta_K\psi)(\tilde\psi\,\Theta^K\psi)$ with
  $\Theta_K=(\Theta^K)^{-1}$. Each $I_r$ is invariant under all transformations of
  $Spin(4,4)$.
* **Form Q (quantized-sector form; the alternative).** The bilinears are
  $\bar\psi\,\Theta^K\psi$ with $\bar\psi=\psi^\dagger\beta$ and with the matrices
  $-i\gamma^A$ ($A=0,\dots,4$), $\gamma^A$ ($A=5,6,7$) in place of $\gamma^A$, and
  $\zeta_r=i$ for $r=2,3,6,7$.

The exact relation between them (verified for all 256 index sets $K$): form C is form
Q with the matrix $W$ inserted in every bilinear,
$I_r^{\rm C}=\sum_{|K|=r}\epsilon_K\,(\psi^\dagger W\beta\,\Theta^K\psi)^2$ with the
matrices $\Theta^K$ and the signs $\epsilon_K=\pm1$ of form Q. Two particles of the
same half interact in the same way in both forms; two particles of different halves
interact with opposite signs.

# 3. What the folder contains now

| item | state |
|:---|:---|
| `code/deflation-cjt-ks-c-rs` | the program `deflation_cjt_ks_c` (Rust). It is the program of the folder with the other definition plus: the module `halves.rs` (uniform matter of the two halves of the field in both forms, the effective potential of Cornwall, Jackiw and Tomboulis for two halves, the vacuum with the filled states of negative energy, the functional with the Vorton-Scalar); the command `forms` (files `cmd_forms.rs`, `cmd_forms_b.rs`); extensions of `selfint.rs` (exact checks 44-S13 and S12 for all three models), of `pair_contact.rs` and `cmd_pairing.rs` (both forms in the pair along the hidden axis and in the pairs along $x5,x6,x7$), and of `cmd_gravity.rs` (matter of both forms as a source). It compiles; the unit tests of the new modules pass. |
| `code/scripts` | the scripts of the other folder with the names of this folder; not yet adapted to the new results. |
| `data` | only files that the program of this folder has written and whose command has finished are committed. At the time written above that is the command `superfluid`; the other commands are being executed. |
| documents (two manuscripts, the document on the Dirac matrices, the provenance files, README) | **not yet written for this folder**; they follow, each in the three formats .md, .tex and .pdf. |

# 4. Exact results that are already established

They come from the command `selfcoupling` of the program of this folder (36 of 36
checks passed in a first run; integer arithmetic).

1. *The same four numbers in both forms.* For uniform matter whose two halves are
   filled differently, with the number densities $n_\pm$ and the scalar densities
   $s_\pm$ of the halves, the interaction energy is
   $\tfrac12 a_V(n_+^2+n_-^2)+\tfrac12 a_S(s_+^2+s_-^2)\pm[\,b_V\,n_+n_-+b_S\,s_+s_-]$,
   upper sign for form Q and lower sign for form C, with
   $a_S=(7g_{0\rm eff}-5g_{1\rm eff}-3g_{7\rm eff})/8$,
   $a_V=(-g_{0\rm eff}+11g_{1\rm eff}-3g_{7\rm eff})/8$,
   $b_S=(8g_{0\rm eff}-3g_{1\rm eff}-5g_{7\rm eff}-g_{8\rm eff})/8$,
   $b_V=(11g_{1\rm eff}-3g_{7\rm eff}+g_{8\rm eff})/8$. Here
   $g_{0\rm eff},g_{1\rm eff},g_{7\rm eff},g_{8\rm eff}$ are the four independent
   combinations of the nine couplings (scalar, vector, axial vector, pseudoscalar):
   $g_{0\rm eff}=g_0-3g_2-4g_3-5g_4-4g_5-g_6$, $g_{1\rm eff}=g_1-g_2-g_5+g_6$,
   $g_{7\rm eff}=g_7-g_2-g_3+g_6$, $g_{8\rm eff}=g_8-g_2+4g_3-5g_4+4g_5-3g_6$.
2. *The internal symmetry.* Fields that do not depend on $x0,x5,x6,x7$ have, without
   self-interaction, an internal symmetry with eight generators: the six products
   $\gamma^a\gamma^b$ of two of the matrices $\gamma^0,\gamma^5,\gamma^6,\gamma^7$,
   the unit matrix, and a matrix $P_E$ proportional to
   $\gamma^0\gamma^5\gamma^6\gamma^7$. In form Q the self-interaction is invariant
   under the six products for all couplings, and under $P_E$ if and only if
   $g_{1\rm eff}=g_{7\rm eff}=0$. In form C it is invariant for all couplings only
   under the three products $\gamma^5\gamma^6,\gamma^5\gamma^7,\gamma^6\gamma^7$ (the
   rotations of the three additional times, which are symmetries of the spacetime);
   under the three unitary transformations generated by $\gamma^0\gamma^t$
   ($t=5,6,7$) and under $P_E$ it is invariant only if all four effective couplings
   vanish. *In form C every self-interaction that does not vanish breaks the internal
   symmetry down to the rotations of $x5,x6,x7$.*
3. *Pairs.* The pair along the hidden axis, created by $\psi^TC^-\gamma^0\psi$,
   consists of one particle of each half; the pairs along $x5,x6,x7$ consist of two
   particles of the same half. In form C the contact self-interaction therefore acts
   in the pair along the hidden axis with the sign opposite to that of form Q, and in
   the pairs along $x5,x6,x7$ with the same sign (unit test of `pair_contact.rs`, all
   momenta in ordinary space).

# 5. What is still missing

1. The data files of all commands, written by the program of this folder (in
   progress).
2. The independent verification of the new results by a second program in another
   language, and the test of that verification with deliberate errors.
3. The figures.
4. The documents: both manuscripts, the document on the Dirac matrices with both
   forms written out in full, the provenance files, among them one that records both
   options completely and discusses them, and the README; each as .md, .tex and .pdf.

Each of these items is committed and pushed when it is finished.
