#!/usr/bin/env wolframscript
(* ::Package:: *)
(* ================================================================================================
   lg_annih.wls      (PROJECT spacetimes, folder Revision_Lord_Gravity, task C23, stage 7d)

   PURPOSE
   The exact checks, in the Wolfram Language, that the quasi-particles of the superfluid vacuum of the
   brochon model are the particles and antiparticles of a Dirac field of the mass of the gap, whose
   number the zeugon conserves.  The identifier of a check is WA and its number; the counterparts in
   the Rust program lg_annih (AN) and the Python program lg_annih.py (YA) in brackets.

     WA-1  (Rust AN-1; Python YA-1) P = (gamma^0 (x) 1_2)(K^3)^dag is real, antisymmetric and unitary;
           P (gamma^a)^* = gamma^a P; P (i T^3)^* = i T^3 P; the reflection F = 1_4 (x) tau_1 commutes
           with C (P F^* = F P) and turns T^3 into -T^3;
     WA-2  (Rust AN-2; Python YA-2) [i gamma d - M - D C](1 + b C) = (1 - b C)[i gamma d - E] with
           E = (M^2 + |D|^2)^(1/2) and b = -D/(E + M), on a plane wave with a symbolic spinor and a
           symbolic four-momentum (off the mass shell);
     WA-3  (Rust AN-3; Python YA-3) the matrices gamma^0 gamma^mu T^3 P are symmetric and
           P^dag (gamma^0 gamma^mu T^3) P = -(gamma^0 gamma^mu T^3)^T; in the Fock space of eight fermion
           modes (dimension 256, Jordan-Wigner, exact rational arithmetic, b = 3/7 + 2 i/5), the field
           Psi = chi + b P chi^dag has {Psi_i, Psi_j^dag} = (1 + |b|^2) delta_ij, {Psi_i, Psi_j} = 0, and
           its zeugon current is exactly (1 + |b|^2) times that of chi.

   USAGE
     wolframscript -file lg_annih.wls <report.txt>
   ================================================================================================ *)
args = Rest[$ScriptCommandLine];
If[Length[args] < 1, Print["usage: lg_annih.wls <report.txt>"]; Exit[2]];
reportFile = args[[1]];
t0 = AbsoluteTime[];
lines = {}; npass = 0; ntot = 0;
check[id_String, text_String, ok_, detail_String] := Module[{line},
  ntot++; If[TrueQ[ok], npass++];
  line = StringJoin[If[TrueQ[ok], "PASS ", "FAIL "], StringPadRight[id, 9], text, " [", detail, "]"];
  Print[line]; AppendTo[lines, line]];

(* the matrices of the chiral basis of lord_gravity.md and of the pairing term *)
s0 = IdentityMatrix[2]; s1 = {{0, 1}, {1, 0}}; s2 = {{0, -I}, {I, 0}}; s3 = {{1, 0}, {0, -1}};
z2 = ConstantArray[0, {2, 2}];
blk[a_, b_, c_, d_] := ArrayFlatten[{{a, b}, {c, d}}];
g4 = {blk[z2, s0, s0, z2], blk[z2, s1, -s1, z2], blk[z2, s2, -s2, z2], blk[z2, s3, -s3, z2]};
gch = I g4[[1]] . g4[[2]] . g4[[3]] . g4[[4]];
el = {{0, 1}, {-1, 0}};
cll = ArrayFlatten[{{el, z2}, {z2, -el}}];
k3 = KroneckerProduct[cll . gch, el . s3];
pm = KroneckerProduct[g4[[1]], s0] . ConjugateTranspose[k3];
g8 = KroneckerProduct[#, s0] & /@ g4;
t3 = KroneckerProduct[IdentityMatrix[4], s3/2];
fl = KroneckerProduct[IdentityMatrix[4], s1];
zero8 = ConstantArray[0, {8, 8}];

(* ------------------------------------------------------------------ WA-1 *)
ok1 = AllTrue[Flatten[pm], Im[#] === 0 &] && pm + Transpose[pm] === zero8 && pm . ConjugateTranspose[pm] === IdentityMatrix[8] &&
   AllTrue[g8, pm . Conjugate[#] - # . pm === zero8 &] && pm . Conjugate[I t3] - I t3 . pm === zero8 &&
   pm . Conjugate[fl] - fl . pm === zero8 && fl . t3 . fl + t3 === zero8 && AllTrue[g8, fl . # - # . fl === zero8 &];
check["WA-1", "(Rust AN-1; Python YA-1) P is real, antisymmetric and unitary; P (gamma^a)^* = gamma^a P; P (i T^3)^* = i T^3 P; \
F = 1_4 (x) tau_1 commutes with C and with the gamma^a, and F T^3 F = -T^3 (exact)", ok1, "all identities: " <> ToString[ok1]];

(* ------------------------------------------------------------------ WA-2 *)
(* a field A e^(-i p.x) + B e^(+i p.x) as the pair {A, B}: i gamma d -> {g.p A, -g.p B}; C -> {P B^*, P A^*} *)
uvec = Table[x[k] + I y[k], {k, 8}];
pv = {p0, p1, p2, p3};
gp = g8[[1]] p0 - g8[[2]] p1 - g8[[3]] p2 - g8[[4]] p3;
dd = d Exp[I del]; ee = Sqrt[mm^2 + d^2];
conjv[v_] := ComplexExpand[Conjugate[v]];
dsl[{a_, b_}] := {gp . a, -gp . b};
cm[{a_, b_}] := {pm . conjv[b], pm . conjv[a]};
res[bb_] := Module[{chi = {uvec, ConstantArray[0, 8]}, psi, lhs, w, rhs},
   psi = chi + bb cm[chi];
   lhs = dsl[psi] - mm psi - dd cm[psi];
   w = dsl[chi] - ee chi;
   rhs = w - bb cm[w];
   Simplify[ComplexExpand[Flatten[lhs - rhs]], Assumptions -> {mm > 0, d > 0, Element[del, Reals]}]];
r2 = res[-dd/(ee + mm)];
c2 = res[dd/(ee + mm)];
ok2 = AllTrue[r2, # === 0 &] && ! AllTrue[c2, # === 0 &];
check["WA-2", "(Rust AN-2; Python YA-2) [i gamma d - M - D C](1 + b C) = (1 - b C)[i gamma d - E], b = -D/(E + M), on a plane \
wave with a symbolic spinor (16 real symbols) and four-momentum (off the mass shell), symbolic M, |D| and phase of D; with -b it \
fails (control)", ok2, "16 components vanish: " <> ToString[AllTrue[r2, # === 0 &]] <> "; control fails: " <>
   ToString[! AllTrue[c2, # === 0 &]]];

(* ------------------------------------------------------------------ WA-3 *)
gmu = Table[g8[[1]] . g8[[m]] . t3, {m, 4}];
sym = AllTrue[gmu, (# . pm) - Transpose[# . pm] === zero8 &];
nrm = AllTrue[gmu, ConjugateTranspose[pm] . # . pm + Transpose[#] === zero8 &];
(* the Fock space: c_i = Z (x) ... (x) Z (x) a (x) 1 (x) ... (x) 1 *)
aop = SparseArray[{{1, 2} -> 1}, {2, 2}]; zop = SparseArray[{{1, 1} -> 1, {2, 2} -> -1}]; one = SparseArray[IdentityMatrix[2]];
cop[i_] := KroneckerProduct @@ Table[Which[k < i, zop, k == i, aop, True, one], {k, 1, 8}];
cs = Table[cop[i], {i, 8}];
cds = ConjugateTranspose /@ cs;
bq = 3/7 + 2 I/5; nb = 1 + Abs[bq]^2;
psi = Table[cs[[i]] + bq Sum[pm[[i, j]] cds[[j]], {j, 8}], {i, 8}];
psid = ConjugateTranspose /@ psi;
id256 = SparseArray[IdentityMatrix[256]];
car1 = AllTrue[Flatten[Table[Normal[psi[[i]] . psid[[j]] + psid[[j]] . psi[[i]] - If[i == j, nb, 0] id256] === ConstantArray[0, {256, 256}],
      {i, 8}, {j, 8}]], TrueQ];
car2 = AllTrue[Flatten[Table[Normal[psi[[i]] . psi[[j]] + psi[[j]] . psi[[i]]] === ConstantArray[0, {256, 256}], {i, 8}, {j, 8}]], TrueQ];
curr = AllTrue[Table[Module[{G = gmu[[m]], jp, jc},
      jp = Sum[psid[[i]] . Sum[G[[i, j]] psi[[j]], {j, 8}], {i, 8}];
      jc = Sum[cds[[i]] . Sum[G[[i, j]] cs[[j]], {j, 8}], {i, 8}];
      Normal[jp - nb jc - (Tr[jp - nb jc]/256) id256] === ConstantArray[0, {256, 256}]], {m, 4}], TrueQ];
check["WA-3", "(Rust AN-3; Python YA-3) gamma^0 gamma^mu T^3 P is symmetric and P^dag (gamma^0 gamma^mu T^3) P = \
-(gamma^0 gamma^mu T^3)^T; in the Fock space of eight modes (exact, b = 3/7 + 2i/5) {Psi_i, Psi_j^dag} = (1 + |b|^2) delta_ij, \
{Psi_i, Psi_j} = 0 and J[Psi] = (1 + |b|^2) J[chi] up to a multiple of the identity", sym && nrm && car1 && car2 && curr,
  "symmetric " <> ToString[sym] <> ", normal part " <> ToString[nrm] <> ", anticommutators " <> ToString[car1 && car2] <>
   ", currents " <> ToString[curr]];

out = StringJoin[
  "lg_annih.wls: the exact checks of stage 7d of task C23, the quasi-particles as Dirac particles and the annihilation of the \
pair (Revision_Lord_Gravity, PROJECT spacetimes)\n",
  "Each line: PASS or FAIL, the identifier of the check in this script (WA and a number), the statement and, in brackets, the\n",
  "facts that decide it.  All checks are exact.\n\n",
  StringRiffle[lines, "\n"], "\n\nNotes:\n",
  "  Wolfram Language " <> ToString[IntegerPart[$VersionNumber]] <> "." <> ToString[Round[10 FractionalPart[$VersionNumber]]] <>
   "." <> ToString[$ReleaseNumber] <> "; run time " <> ToString[Round[AbsoluteTime[] - t0]] <> " s\n",
  If[npass == ntot, "\nVERDICT: ALL PASS (" <> ToString[npass] <> " of " <> ToString[ntot] <> ")\n",
   "\nVERDICT: FAIL (" <> ToString[npass] <> " of " <> ToString[ntot] <> " passed)\n"]];
Export[reportFile, out, "Text"];
Print[npass, " of ", ntot, " checks passed"];
