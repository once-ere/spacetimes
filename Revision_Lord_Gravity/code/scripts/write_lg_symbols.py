#!/usr/bin/env python3
"""
write_lg_symbols.py (Revision_Lord_Gravity) -- the registry of the symbols of the folder (stage 4b of task C23) and its
machine checks.

The rule of the author (one symbol, one object, in the whole of PROJECT spacetimes) is kept in this folder by suffixes:
LL for the objects of the Lorentz super-loop and of its fields (the algebra of the bi-quaternions and bi-octonions, the
brochon and its Dirac matrices), SF for the objects of the brochon model and of its superfluid ground state, LG for the
objects that this folder introduces beyond these two theories (the gravitational field, the black holes and the problems
of the manuscript), Lord for the objects of Lord's paper, and the letters of common notation without a suffix.  This program reads the three tables of
symbols of the manuscript (Appendix D of docs_src/lord_gravity.src.md, Section A.7 of docs_src/parts/algebra_lg.part.md,
Section B.7 of docs_src/parts/brochon_lg.part.md) and the table of the remarks document (Section 19 of
docs_src/P4_remarks.src.md), compares every symbol with the registries of two other folders of the project
(Revision_SM_Higgs/provenance/symbols_sf.csv, the registry of the superfluid solution of the brochon model, and
Revision_SuperLoop/provenance/symbols_ll.csv, the registry of the super-loop work), and checks:

  LY-1  every row of the four tables has a symbol cell with at least one symbol (a formula between $ signs, or a plain
        name such as GEE or M31) and a meaning;
  LY-2  no symbol with the suffix LG occurs in the two other registries (an LG symbol names an object of this folder only),
        except the collisions declared below with their remedy;
  LY-3  a symbol text that occurs in more than one row of the three tables of the manuscript is declared below, with the
        reason (the same object listed in two tables, a shorthand defined where it is used, or a local notation of a
        part); an undeclared repetition fails the check;
  LY-4  every symbol of the table of the remarks document occurs in a table of the manuscript, except those declared below
        as symbols of the remarks document only;
  LY-5  (a count, not a pass or fail) how many symbols of the manuscript with the suffix LL or SF the other two registries
        list with the same text, and how many neither lists in that text (forms with other indices or arguments, and
        objects of the two theories that the other registries do not list); the registry document lists the meanings of
        the listed ones in the three registries, for the reader to compare;
  LY-6  every symbol without a suffix (common notation) that the tables of the manuscript share with the other two
        registries is declared below: the same object there, or a collision with its remedy;
  LY-7  every symbol of the table of the task record (Section 8 of docs_src/P0_task_record.src.md) occurs in a table of the
        manuscript or is declared below as a notation of the task record (most are earlier names of objects that the
        manuscript writes otherwise); a declared one that another registry lists is declared as the same object or as a
        collision with its remedy;
  LY-8  a symbol that the tables write with different trailing arguments, or with and without one (such as M_LG and
        M_LG(v_LG)), is declared below as one object or as two related objects.

In every check a declaration that is no longer needed fails as well, so that the report never states a declaration that is
false.  What no program can decide is whether two tables that list the same symbol text mean the same object by it; this was
compared by hand for the remarks document and the task record (one collision found, the letter c, declared under LY-6), and the
symbols shared with the other registries are printed side by side in the registry document for the reader to compare.

Output: data/lg_symbols.csv (one row per symbol text of the manuscript), data/hck_lg_symbols_checks.txt (the report),
provenance/generated/lg_symbols.md (the fragments of the registry document docs_src/P1_symbols.src.md).
Usage (from the folder Revision_Lord_Gravity):   python3 code/scripts/write_lg_symbols.py
Requires: Python 3 (standard library only).
"""
import csv
import os
import re
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
PROJECT = os.path.abspath(os.path.join(ROOT, ".."))
DATA = os.path.join(ROOT, "data")
OUT = os.path.join(ROOT, "provenance", "generated", "lg_symbols.md")

TABLES = [
    ("D", "Appendix D", os.path.join(ROOT, "docs_src", "lord_gravity.src.md"), "# D. The symbols of the main text"),
    ("A.7", "Appendix A.7", os.path.join(ROOT, "docs_src", "parts", "algebra_lg.part.md"), "## A.7"),
    ("B.7", "Appendix B.7", os.path.join(ROOT, "docs_src", "parts", "brochon_lg.part.md"), "## B.7"),
]
REMARKS = ("P4", "the remarks document", os.path.join(ROOT, "docs_src", "P4_remarks.src.md"), "# 19. The symbols of this document")
TASK = ("P0", "the task record", os.path.join(ROOT, "docs_src", "P0_task_record.src.md"), "# 8. The symbols of this document")
REGISTRIES = [
    ("SM_Higgs", os.path.join(PROJECT, "Revision_SM_Higgs", "provenance", "symbols_sf.csv")),
    ("SuperLoop", os.path.join(PROJECT, "Revision_SuperLoop", "provenance", "symbols_ll.csv")),
]

# LY-3: the symbol texts that the manuscript lists in more than one row, with the reason.  "same" = the same object, listed
# in two tables (the appendices list their own symbols, so that each is complete in itself); "short" = a shorthand that a
# proof or a section defines where it uses it; "local" = a local notation of a part, declared in that part.
SAME = "the same object, listed in the table of each part that uses it"
DECLARED_REPEATS = {
    r"\mathsf E_{\rm LL}": SAME, r"\mathsf 1_2": SAME, r"\mathsf 1_4": SAME, r"\gamma^{\rm ch}_{\rm LL}": SAME,
    r"C_{\rm LL}": SAME, r"M_{\rm LL}(\mathbf b)": SAME, r"\mathbf g": SAME, r"\mathsf L_{\rm LL}(\mathbf g)": SAME,
    r"\Psi_{\rm LL}": SAME, r"m_{\rm LL}": SAME, r"\mathsf U_{\rm LL}": SAME, r"\nabla^{\rm GEE}_p": SAME,
    r"\Gamma^{\rm LL}_p": SAME, r"\mathsf W_{{\rm SF},p}": SAME, r"W^\iota_{{\rm SF},p}": SAME, r"B_{{\rm SF},p}": SAME,
    r"g^{\rm v}_{\rm LL}": SAME, r"g^{\rm F}_{\rm LL}": SAME, r"T^3_{\rm LL}": SAME, r"Q_{\rm LL}": SAME, r"\vec n": SAME,
    r"V^{(n)}_{\rm SF}": SAME, r"G^{\rm S}_{\rm SF}": SAME, r"G^{\rm V}_{\rm SF}": SAME, r"G_{\rm LL}": SAME,
    r"\Pi^\iota_{\rm LL}": SAME, r"\mathsf K^\iota_{\rm LL}": SAME, r"T^\iota_{\rm LL}": SAME,
    r"\epsilon^{\rm LL}_{\iota\iota'\iota''}": SAME, r"M^{\rm CJT}_{W,{\rm SF}}": SAME, r"\varepsilon_{\rm LL}": SAME,
    r"U(1)^{\rm Q}_{\rm LL}": SAME, r"\mathbf g_{\rm boost}": SAME, r"P_{\rm LG}": SAME, r"\alpha^{\rm Z}_{\rm LG}": SAME,
    r"u_{\rm LL}": SAME, "GEE": SAME, r"\mathsf S": r"the same object: the spinor matrix $\mathsf S_{\rm LL}(\mathbf g)$, written short in Part II",
    r"s": "the same object: the isospin index (1 up, 2 down) of the brochon",
    r"x^\mu": "the same object: the coordinates, of M31 in Appendix A and of a curved space-time in the main text",
    r"e": "the same object: the elementary electric charge (Lord's Sections 1 and 2, and the electron in Section 30.3)",
    r"m": r"a rest mass: of a Bargmann-Wigner field in Part I, and the shorthand for the mass of one particle of the pair in "
          "Sections 29.3 and 33.3, defined there",
    r"M": "shorthand for the Dirac mass of the superfluid, defined in Theorems L2 and L3 and in Section 33, the same object",
    r"D": "shorthand for the pair amplitude of the superfluid, defined in Theorems L2 and L3 and in Section 33, the same object",
    r"E": "shorthand for the gap of the superfluid, defined in Theorem L3 and in Section 33, the same object",
    r"\ell": "the orbital angular momentum number: of a level of the pair (Section 29.3) and of a spherical harmonic (Section 32)",
    r"r": r"a local notation of Part VI, declared at its beginning: there $r$ is the distance of the two quasi-particles of the "
          r"pair, and the Schwarzschild radius of their centre of mass is $r^{\rm cm}_{\rm LG}$",
    r"\theta": r"a local notation of Part VI, declared at its beginning: there $\theta$ is the angle of the relative coordinate of "
               r"the pair from the radius of the black hole",
}
# LY-6: the symbols without a suffix that the manuscript shares with the other registries
SAME_ALGEBRA = "the same object (the algebra of the bi-quaternions and the bi-octonions, as in the other registries)"
DECLARED_SHARED = {
    r"\mathsf 1_2": r"the same object (a unit matrix)", r"\mathsf 1_4": r"the same object (a unit matrix)",
    r"\mathrm{tr}": r"the same object (the trace); the manuscript adds the normalized trace $\mathrm{tr}_{\rm N}$",
    r"A_\mu": r"the same object: the electromagnetic potential (Lord's Sections 1 and 2; the photon of the Standard Model)",
    r"e": r"the same object: the elementary electric charge",
    r"\mathbf b": SAME_ALGEBRA, r"\mathbf e_p": SAME_ALGEBRA, r"\mathbf g": SAME_ALGEBRA, r"\tilde{\mathbf g}": SAME_ALGEBRA,
    r"\mathbf x": SAME_ALGEBRA, r"\mathbf f_p": SAME_ALGEBRA, r"\mathbf e_0,\dots,\mathbf e_7": SAME_ALGEBRA,
    r"\bar{\mathbf x}": SAME_ALGEBRA, r"\mathbf x^\dagger": SAME_ALGEBRA, r"[\mathbf x,\mathbf y,\mathbf z]": SAME_ALGEBRA,
    r"\mathbf u": SAME_ALGEBRA, "M31": r"the same object: the space-time of the Hermitian bi-quaternions",
    r"GEE": r"the same object: the gauge group of the brochon model", r"\nabla^{\rm GEE}_p": r"the same object: the GEE covariant derivative",
    r"SU(2)_{\rm v}": r"the same object: the isospin group", r"O(2)": r"the same object: the group of the rotations and reflections of a plane",
    r"S": r"the same object: the total spin of a state of two spin-1/2 particles",
    r"c": r"a collision: in this folder $c$ is the number of a component of the brochon (1 to 8, Appendix B.1), the same "
          r"object as in the two other registries, and also the speed of light, common notation that the units set to 1 "
          r"(remedy: each is defined where it is used; the speed of light appears only in $\hbar=c=1$ and in formulas written "
          r"in ordinary units, such as $y_{\rm LG}=G_{\rm N}M_{\rm LG}m_{\rm LL}/(\hbar c)$; the letter is also one of the frame "
          r"indices $a,b,c,d$, which mean nothing outside their formula)",
    r"G_S": r"the same object: the author's scalar coupling of his Kohn-Sham ansatz",
    r"G_V": r"the same object: the author's vector coupling of his Kohn-Sham ansatz",
    r"n": r"the same object in the table: the number density of the author's Kohn-Sham ansatz",
    r"n_S": r"the same object: the scalar density of the author's Kohn-Sham ansatz",
    r"X": r"the same object: the label of a method (HS, KS or CJT)",
    r"M": r"a collision: in Revision_SuperLoop the letter M labels a family of machine checks; in this folder $M$ is a "
          r"shorthand for the Dirac mass of the superfluid, defined in Theorems L2 and L3 and in Section 33 where it is used "
          r"(remedy: the shorthand is defined at each use)",
    r"D": r"a collision: in Revision_SuperLoop $D$ is a number of space dimensions; in this folder $D$ is a shorthand for the "
          r"pair amplitude of the superfluid, defined in Theorems L2 and L3 and in Section 33 where it is used (remedy: the "
          r"shorthand is defined at each use)",
    r"L": r"a collision: in Revision_SuperLoop the letter L labels a family of machine checks; in this folder $L$ is the "
          r"spectroscopic letter of the orbital angular momentum ($S$, $P$, $D$) of a level of the pair, defined in Section 29.3 "
          r"(remedy: the letter and its values are defined there; the values $S,P,D,\dots$ are the labels of the standard "
          r"notation of a level, such as $1\,{}^3S_1$ or the triplet S states, and not symbols of objects: as symbols, $S$ is the "
          r"total spin and $D$ the shorthand for the pair amplitude)",
    r"\alpha": r"a collision: in Revision_SM_Higgs $\alpha$ is an index over the two components of a Weyl spinor; in this folder "
               r"$\alpha$ is a shorthand for $\alpha^{\rm Z}_{\rm LG}$ in Sections 29.3 and 33.3, defined there (remedy: the "
               r"shorthand is defined at each use)",
}
# LY-2: LG symbols that occur in another registry, with the remedy (none at present)
DECLARED_LG_COLLISIONS = {}
# LY-4: symbols of the remarks document that the tables of the manuscript do not list
REMARKS_ONLY = {
    r"\hbar": "Planck's constant, common notation (set to 1 in the manuscript)",
    r"\epsilon^{\rm d}_{\rm LG}": r"a dielectric constant of the condensate, used only in remark R9",
    r"W^1": r"the massive gauge boson $W^1$, the component $\iota=1$ of $W^\iota_{{\rm SF},p}$",
    r"W^2": r"the massive gauge boson $W^2$, the component $\iota=2$ of $W^\iota_{{\rm SF},p}$",
    r"B": r"the massive gauge boson $B_{{\rm SF},p}$, written without its index",
    r"R^{\rm LG}_{\mu\nu\rho\sigma}": r"the Riemann tensor with all indices lowered; the manuscript lists the same tensor with the "
                                     r"index letters $\rho\sigma\mu\nu$, $R^{\rm LG}_{\rho\sigma\mu\nu}$",
    r"\gamma^{ab}_{\rm LL}": r"the antisymmetrized Dirac matrices; the manuscript lists them as $\gamma^{\mu\nu}_{\rm LL}$",
}
# LY-7: symbols of the table of the task record (its Section 8) that the tables of the manuscript do not list.  The task record
# is a log: its early entries (stages 5 and 6) were written before Parts IV and V of the manuscript, which chose some symbols
# anew; the entries keep their notation, and the table of the task record defines it.
TASK_ONLY = {
    r"\eta^{\rm LL}": r"the Minkowski metric written without its indices (as in Section 1 of the manuscript); the tables of the "
                     r"manuscript list it with its indices, $\eta^{\rm LL}_{\mu\nu}$",
    r"SL(2,\mathbb C)": r"a standard group, the unit bi-quaternions; the manuscript names it in its text and in the meaning of GEE",
    r"LG": r"the suffix itself, which the task record lists to explain it; not a symbol of an object",
    r"\gamma^a_{\rm LL}": r"the Dirac matrices with a frame index $a$; the tables of the manuscript list them with the index $\mu$ "
                         r"and, in Appendix A, with the index $p$",
    r"\gamma^{ab}_{\rm LL}": r"their antisymmetrized products with frame indices; the manuscript lists $\gamma^{\mu\nu}_{\rm LL}$",
    r"R^{\rm LG}_{ab\mu\nu}": r"the Riemann tensor with two frame indices; the manuscript lists $R^{{\rm LG}\,a}{}_{b\mu\nu}$",
    r"s_{\rm LG}": r"an exponent of the radial equations at a regular singular point (entries of stage 5); the manuscript writes it "
                  r"$\varsigma_{\rm LG}$ (19.1)",
    r"\mathcal A_{\rm LG}": r"the greybody factor (entries of stage 5); the manuscript writes it $\mathcal T_{\rm LG}$ (21.1)",
    r"S_{\rm LG}": r"$j_{\rm LG}-\ell_{\rm LG}$, the projection of the spin of a hydrogen-like level (entries of stage 5); not used "
                  r"in the manuscript",
    r"a^{\rm H}_{\rm LG}": r"the coefficient of the decay rate at small masses (entry E13); the manuscript writes it "
                          r"$c^{\rm H}_{\rm LG}$ (20.5)",
    r"S_{\rm H}": r"the matrix of the separated spinor (entries E7 and E15); the manuscript writes it $\mathsf S^{\rm H}_{\rm LG}$ (18.3)",
    r"H_{\rm LG}": r"its block (entries E7 and E15); the manuscript writes it $\mathsf H_{\rm LG}$ (18.3)",
    r"\sigma_1": r"a Pauli matrix (entries E7 and E15); the manuscript writes it $\sigma^{\rm LL}_1$",
    r"\sigma_3": r"a Pauli matrix (entries E7 and E15); the manuscript writes it $\sigma^{\rm LL}_3$",
    r"v": r"the advanced time of the Vaidya space-time (decision D10 and the entries of stage 6); the manuscript writes it $v_{\rm LG}$",
    r"\mathcal M_{\rm LG}(v)": r"the mass as a function of the advanced time (entries of stage 6); the manuscript writes "
                              r"$\mathcal M_{\rm LG}(v_{\rm LG})$",
    r"\mathcal P^{\rm V}_{\rm LG}(v,r)": r"a radial function in the Vaidya space-time (entries of stage 6); the manuscript writes "
                                        r"the argument $v_{\rm LG}$",
    r"\mathcal Q^{\rm V}_{\rm LG}(v,r)": r"the other radial function in the Vaidya space-time (entries of stage 6); the manuscript "
                                        r"writes the argument $v_{\rm LG}$",
    r"N^{\rm V}_{\rm LG}(v)": r"the charge on a slice of constant advanced time (entries of stage 6); the manuscript writes it "
                             r"$\mathcal N^{\rm V}_{\rm LG}(v_{\rm LG})$ (24.5)",
    r"p_n": r"the profile of the state $n$ on a slice (entry E18); the manuscript writes it $\mathcal P^{(n)}_{\rm LG}(r)$",
    r"c_{n,{\rm LG}}": r"the coefficient of the state $n$ in the decomposition of the field (entry E18); not used in the manuscript",
    r"W_{n,{\rm LG}}": r"the charge that the decomposition assigns to the state $n$ (entry E18); the manuscript writes it "
                      r"$\mathcal W_{n,{\rm LG}}$",
    r"r_*": r"the tortoise coordinate (entries of stage 5); not used in the manuscript",
    r"Z": r"the nuclear charge of a hydrogen atom before a sudden change (entry E17); the manuscript writes it $Z_{\rm H}$ (Section 26.1)",
    r"Z'": r"the nuclear charge after the change (entry E17); the manuscript writes it $Z'_{\rm H}$ (Section 26.1)",
    r"r^{\rm ion}_{\rm LG}": r"the radius at which the tidal field frees the pair in the classical estimate (entry E20); not used "
                            r"in the manuscript",
}
# LY-7: symbols of the task record only that one of the other registries lists, with the same object or a collision with
# its remedy
TASK_SHARED = {
    r"SL(2,\mathbb C)": r"the same object as in the registries of Revision_SM_Higgs and Revision_SuperLoop",
    r"v": r"a collision: in Revision_SM_Higgs $v$ is the vacuum value of the Higgs field of the Standard Model; in the task record "
          r"of this folder $v$ is the advanced time of the Vaidya space-time (remedy: the task record defines it in decision D10 "
          r"and in its table of symbols, and the manuscript writes $v_{\rm LG}$)",
}
# LY-8: symbols that the tables (of the manuscript, the remarks document and the task record) write with different trailing
# arguments, or with and without one; each such family is declared as one object (or one object and a closely related one)
DECLARED_FORMS = {
    r"E": r"two related objects: the energy $E(\vec q)$ of a quasi-particle of momentum $\vec q$ (B.18) and its smallest value, "
          r"the gap, written $E$ in Theorem L3 and in Section 33: $E=E(\vec 0)$ for the method CJT",
    r"N_{\rm LL}": r"one object and its polarization: the complex norm of a bi-octonion, $N_{\rm LL}(\mathbf x)$, of a bi-quaternion, "
                  r"$N_{\rm LL}(\mathbf b)$, and its polarized form $N_{\rm LL}(\mathbf b,\mathbf b')$",
    r"\gamma^p_{\rm LL}": r"one family: the Dirac matrices $\gamma^p_{\rm LL}[\mathbf f]$ of a frame $\mathbf f$ (9.4) and those of "
                         r"the canonical frame, $\gamma^p_{\rm LL}$",
    r"\mathbf g": r"one object: a unit bi-quaternion, and $\mathbf g(x)$ the same as a function of the point (a local transformation, (B.4))",
    r"\mathcal M_{\rm LG}": r"one object: the mass of the black hole in units of length, as a function of the advanced time "
                           r"($\mathcal M_{\rm LG}(v_{\rm LG})$, in the task record $\mathcal M_{\rm LG}(v)$) and at the start "
                           r"($\mathcal M_{\rm LG}(0)$)",
    r"\mathcal P^{\rm V}_{\rm LG}": r"one object: a radial function in the Vaidya space-time, with the argument $v_{\rm LG}$ in the "
                                   r"manuscript and $v$ in the task record",
    r"\mathcal Q^{\rm V}_{\rm LG}": r"one object: the other radial function in the Vaidya space-time, with the argument $v_{\rm LG}$ in "
                                   r"the manuscript and $v$ in the task record",
    r"\mathsf U_{\rm LL}": r"one object: a local isospin rotation, written with or without its argument $x$",
    r"\mathsf h_{\rm LL}": r"one object: the free Dirac Hamiltonian, written with or without the momentum $\vec q$",
    r"\theta^{\rm F}_{\rm LL}": r"one object: the phase of a local transformation of the fermion number, written with or without "
                               r"its argument $x$",
    r"\xi^{\rm LL}_p": r"one object: the complex coefficients of a bi-quaternion or a bi-octonion, written with or without the "
                      r"element as argument",
}


def table_rows(path, start):
    with open(path, encoding="utf-8") as f:
        text = f.read()
    i = text.find(start)
    if i < 0:
        sys.exit("%s: no heading %r" % (path, start))
    rows = []
    started = False
    for line in text[i:].splitlines():
        if line.startswith("| symbol") or line.startswith("|:"):
            started = True
            continue
        if started and line.startswith("| "):
            rows.append(line)
        elif started and rows and not line.startswith("|"):
            break
    return rows


def cells(row):
    parts = [c.strip() for c in row.strip().strip("|").split(" | ")]
    return parts[0], " | ".join(parts[1:])


def symbols_of(cell):
    """the symbols of a cell: the formulas between $ signs, and the plain names (such as GEE and M31) between commas"""
    out = [s.strip() for s in re.findall(r"\$([^$]+)\$", cell) if s.strip()]
    rest = re.sub(r"\$[^$]+\$", ",", cell)
    out += [w.strip() for w in re.split(r"[,;]", rest) if re.fullmatch(r"[A-Z][A-Za-z0-9]*", w.strip())]
    return out


def registry(path):
    out = {}
    with open(path, encoding="utf-8") as f:
        lines = [l for l in f.read().splitlines() if not l.startswith("#")]
    for row in csv.DictReader(lines):
        for sy in symbols_of(row.get("symbol", "")):
            out.setdefault(sy, []).append(row.get("meaning", ""))
    return out


def base_of(sy):
    """the symbol with a trailing argument, such as (x) or [f], removed (for check LY-8)"""
    return re.sub(r"(\([^()]*\)|\[[^\[\]]*\])$", "", sy)


def has_suffix(sy, suf):
    return ("{\\rm %s}" % suf) in sy or ("\\rm %s}" % suf) in sy or (",\\rm %s" % suf) in sy or ("{\\rm %s," % suf) in sy


def main():
    checks = []
    manuscript = {}   # symbol text -> list of (table, meaning)
    nrows = 0
    bad = []
    for tag, name, path, start in TABLES:
        rows = table_rows(path, start)
        if not rows:
            sys.exit("%s: no rows" % name)
        for r in rows:
            nrows += 1
            sym_cell, meaning = cells(r)
            syms = symbols_of(sym_cell)
            if not syms or not meaning:
                bad.append("%s: %s" % (tag, r[:80]))
            for sy in syms:
                manuscript.setdefault(sy, []).append((tag, meaning))
    rem_rows = table_rows(REMARKS[2], REMARKS[3])
    remarks = {}
    for r in rem_rows:
        sym_cell, meaning = cells(r)
        syms = symbols_of(sym_cell)
        if not syms or not meaning:
            bad.append("P4: %s" % r[:80])
        for sy in syms:
            remarks.setdefault(sy, []).append(meaning)
    checks.append(("LY-1", not bad,
                   "every row of the four tables (%d rows of the manuscript in Appendix D, A.7 and B.7, and %d of the remarks "
                   "document) has a symbol cell with at least one symbol and a meaning: %d symbol texts in the manuscript%s"
                   % (nrows, len(rem_rows), len(manuscript), "" if not bad else "; rows without: " + "; ".join(bad))))
    regs = {name: registry(path) for name, path in REGISTRIES}
    # LY-2
    lg = [sy for sy in manuscript if has_suffix(sy, "LG")]
    coll = [(sy, n) for sy in lg for n in regs if sy in regs[n] and sy not in DECLARED_LG_COLLISIONS]
    checks.append(("LY-2", not coll,
                   "none of the %d symbol texts with the suffix LG occurs in the registries of Revision_SM_Higgs (%d symbol texts) "
                   "and Revision_SuperLoop (%d)%s" % (len(lg), len(regs["SM_Higgs"]), len(regs["SuperLoop"]),
                                                      "" if not coll else "; collisions: " + ", ".join("%s (%s)" % c for c in coll))))
    # LY-3
    rep = {sy: v for sy, v in manuscript.items() if len(v) > 1}
    undeclared = sorted(sy for sy in rep if sy not in DECLARED_REPEATS)
    unused = sorted(sy for sy in DECLARED_REPEATS if sy not in rep)
    checks.append(("LY-3", not undeclared and not unused,
                   "of the %d symbol texts that the tables of the manuscript list in more than one row, every one is declared with "
                   "its reason (%d declared)%s%s" % (len(rep), len(DECLARED_REPEATS),
                                                     "" if not undeclared else "; undeclared: " + ", ".join("$%s$" % u for u in undeclared),
                                                     "" if not unused else "; declared but no longer repeated: " + ", ".join("$%s$" % u for u in unused))))
    # LY-4
    missing = sorted(sy for sy in remarks if sy not in manuscript and sy not in REMARKS_ONLY)
    unused4 = sorted(sy for sy in REMARKS_ONLY if sy in manuscript or sy not in remarks)
    checks.append(("LY-4", not missing and not unused4,
                   "every one of the %d symbol texts of the remarks document occurs in a table of the manuscript, except the %d "
                   "declared symbols of the remarks document only; whether a shared symbol names the same object in both was "
                   "reviewed by hand (one collision, $c$, declared under LY-6)%s%s"
                   % (len(remarks), len(REMARKS_ONLY),
                      "" if not missing else "; missing: " + ", ".join("$%s$" % m for m in missing),
                      "" if not unused4 else "; declared but listed by the manuscript or not used: " + ", ".join("$%s$" % u for u in unused4))))
    # LY-5 (count)
    llsf = [sy for sy in manuscript if has_suffix(sy, "LL") or has_suffix(sy, "SF")]
    in_sh = [sy for sy in llsf if sy in regs["SM_Higgs"]]
    in_sl = [sy for sy in llsf if sy in regs["SuperLoop"]]
    in_both = [sy for sy in in_sh if sy in in_sl]
    in_either = sorted(set(in_sh) | set(in_sl))
    in_neither = [sy for sy in llsf if sy not in in_either]
    note5 = ("of the %d symbol texts of the manuscript with the suffix LL or SF, %d occur with the same text in the registry of "
             "Revision_SM_Higgs and %d in that of Revision_SuperLoop (%d in both, %d in at least one); the other %d are forms or "
             "objects that neither registry lists in that text; the registry document lists the meanings of the %d in the three registries"
             % (len(llsf), len(in_sh), len(in_sl), len(in_both), len(in_either), len(in_neither), len(in_either)))
    # LY-6
    nosuf = [sy for sy in manuscript if not any(has_suffix(sy, x) for x in ("LG", "LL", "SF", "SH")) and "Lord" not in sy]
    shared = [sy for sy in nosuf if any(sy in regs[n] for n in regs)]
    undecl6 = sorted(sy for sy in shared if sy not in DECLARED_SHARED)
    unused6 = sorted(sy for sy in DECLARED_SHARED if sy not in shared)
    checks.append(("LY-6", not undecl6 and not unused6,
                   "every one of the %d symbol texts without a suffix that the tables of the manuscript share with the two other "
                   "registries is declared as the same object or as a collision with its remedy (%d declared, %d collisions)%s"
                   % (len(shared), len(DECLARED_SHARED), sum(1 for v in DECLARED_SHARED.values() if v.startswith("a collision")),
                      ("" if not undecl6 else "; undeclared: " + ", ".join("$%s$" % u for u in undecl6))
                      + ("" if not unused6 else "; declared but not shared: " + ", ".join("$%s$" % u for u in unused6)))))
    # LY-7: the table of the task record
    task_rows = table_rows(TASK[2], TASK[3])
    task = {}
    bad7 = []
    for r in task_rows:
        sym_cell, meaning = cells(r)
        syms = symbols_of(sym_cell)
        if not syms or not meaning:
            bad7.append(r[:80])
        for sy in syms:
            task.setdefault(sy, []).append(meaning)
    missing7 = sorted(sy for sy in task if sy not in manuscript and sy not in TASK_ONLY)
    unused7 = sorted(sy for sy in TASK_ONLY if sy in manuscript or sy not in task)
    task_shared = sorted(sy for sy in TASK_ONLY if any(sy in regs[n] for n in regs))
    undecl7 = sorted(sy for sy in task_shared if sy not in TASK_SHARED)
    unused7s = sorted(sy for sy in TASK_SHARED if sy not in task_shared)
    n_task_in_ms = sum(1 for sy in task if sy in manuscript)
    checks.append(("LY-7", not (bad7 or missing7 or unused7 or undecl7 or unused7s),
                   "every one of the %d symbol texts of the table of the task record (%d rows) occurs in a table of the "
                   "manuscript (%d; whether each names the same object in both was reviewed by hand, with no difference found), or "
                   "is declared as a notation of the task record (%d declared, most of them earlier names of objects that the "
                   "manuscript writes otherwise); of these, the %d that another registry lists are declared as the same object or "
                   "as a collision with its remedy%s"
                   % (len(task), len(task_rows), n_task_in_ms, len(TASK_ONLY), len(task_shared),
                      "".join(x for x in [
                          "" if not bad7 else "; rows without a symbol or a meaning: " + "; ".join(bad7),
                          "" if not missing7 else "; undeclared: " + ", ".join("$%s$" % u for u in missing7),
                          "" if not unused7 else "; declared but listed by the manuscript or not used: " + ", ".join("$%s$" % u for u in unused7),
                          "" if not undecl7 else "; shared but undeclared: " + ", ".join("$%s$" % u for u in undecl7),
                          "" if not unused7s else "; declared as shared but not shared: " + ", ".join("$%s$" % u for u in unused7s)]))))
    # LY-8: one letter with different trailing arguments
    every = {}
    for src in (manuscript, remarks, task):
        for sy in src:
            every.setdefault(base_of(sy), set()).add(sy)
    forms = {b: sorted(v) for b, v in every.items() if len(v) > 1}
    undecl8 = sorted(b for b in forms if b not in DECLARED_FORMS)
    unused8 = sorted(b for b in DECLARED_FORMS if b not in forms)
    checks.append(("LY-8", not undecl8 and not unused8,
                   "of the symbols of the three tables (the manuscript, the remarks document, the task record), %d are written with "
                   "different trailing arguments or with and without one; every one is declared as one object or as two related "
                   "objects (%d declared)%s%s"
                   % (len(forms), len(DECLARED_FORMS),
                      "" if not undecl8 else "; undeclared: " + ", ".join("$%s$ (%s)" % (b, ", ".join("$%s$" % x for x in forms[b])) for b in undecl8),
                      "" if not unused8 else "; declared but not found: " + ", ".join("$%s$" % u for u in unused8))))
    # the data file and the report
    os.makedirs(DATA, exist_ok=True)
    with open(os.path.join(DATA, "lg_symbols.csv"), "w", newline="", encoding="utf-8") as f:
        w = csv.writer(f, lineterminator="\n")
        w.writerow(["symbol", "tables", "meaning_in_this_folder", "meaning_in_Revision_SM_Higgs", "meaning_in_Revision_SuperLoop"])
        for sy in sorted(manuscript, key=lambda x: x.lower()):
            w.writerow([sy if re.fullmatch(r"[A-Z][A-Za-z0-9]*", sy) else "$%s$" % sy, " ".join(sorted({t for t, _ in manuscript[sy]})), manuscript[sy][0][1],
                        regs["SM_Higgs"].get(sy, [""])[0], regs["SuperLoop"].get(sy, [""])[0]])
    npass = sum(1 for _, ok, _ in checks if ok)
    lines = ["PROJECT spacetimes, Revision_Lord_Gravity: the registry of the symbols and its checks (stage 4b of task C23; "
             "code/scripts/write_lg_symbols.py)", ""]
    for cid, ok, text in checks:
        lines.append("%s %-5s %s" % ("PASS" if ok else "FAIL", cid, text))
    lines.append("NOTE  LY-5  " + note5)
    lines.append("")
    lines.append("Declared repetitions (LY-3):")
    for sy in sorted(DECLARED_REPEATS):
        lines.append("  $%s$: %s" % (sy, DECLARED_REPEATS[sy]))
    lines.append("Symbols without a suffix shared with the other registries (LY-6):")
    for sy in sorted(DECLARED_SHARED):
        lines.append("  $%s$: %s" % (sy, DECLARED_SHARED[sy]))
    lines.append("Symbols of the remarks document only (LY-4):")
    for sy in sorted(REMARKS_ONLY):
        lines.append("  $%s$: %s" % (sy, REMARKS_ONLY[sy]))
    lines.append("Notations of the task record (LY-7):")
    for sy in sorted(TASK_ONLY):
        lines.append("  $%s$: %s" % (sy, TASK_ONLY[sy]))
    lines.append("Notations of the task record that another registry lists (LY-7):")
    for sy in sorted(TASK_SHARED):
        lines.append("  $%s$: %s" % (sy, TASK_SHARED[sy]))
    lines.append("Symbols written with different arguments (LY-8):")
    for b in sorted(DECLARED_FORMS):
        lines.append("  $%s$ (%s): %s" % (b, ", ".join("$%s$" % x for x in forms[b]), DECLARED_FORMS[b]))
    lines.append("")
    lines.append("VERDICT: %s (%d of %d)" % ("ALL PASS" if npass == len(checks) else "FAIL", npass, len(checks)))
    with open(os.path.join(DATA, "hck_lg_symbols_checks.txt"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
    # the fragments: the list of the symbols of the manuscript with the suffix LL or SF that another registry lists, with their
    # meanings in the three registries, the full registry, and the counts
    frag = {}
    # a list rather than a table: the meanings of the other registries contain long formulas (matrices, traces) that do not
    # fit into the narrow cells of a table of four columns
    tab = []
    for sy in sorted(in_either, key=lambda x: (x.lower().lstrip("\\"), x)):
        tab.append("* $%s$" % sy)
        tab.append("    * in this folder (%s): %s" % (", ".join(sorted({t for t, _ in manuscript[sy]})), manuscript[sy][0][1]))
        for n, label in (("SM_Higgs", "Revision_SM_Higgs"), ("SuperLoop", "Revision_SuperLoop")):
            tab.append("    * in %s: %s" % (label, regs[n][sy][0] if sy in regs[n] else "-- (not listed)"))
    frag["lgy_shared_table"] = "\n".join(tab)
    # the full registry: every symbol text of the manuscript, its meaning, the tables that list it, and the other registries
    # that list the same text
    # the dashes of the separator line set the relative widths of the columns in the PDF (19, 56, 10 and 15 per cent)
    full = ["| symbol | meaning in this folder | listed in | the same text in |",
            "|:---------|:-----------------------------|:-----|:--------|"]
    for sy in sorted(manuscript, key=lambda x: (x.lower().lstrip("\\"), x)):
        shown = sy if re.fullmatch(r"[A-Z][A-Za-z0-9]*", sy) else "$%s$" % sy
        where = ", ".join(sorted({t for t, _ in manuscript[sy]}))
        other = ", ".join(n for n in ("SM_Higgs", "SuperLoop") if sy in regs[n]) or "--"
        full.append("| %s | %s | %s | %s |" % (shown, manuscript[sy][0][1].replace("|", "\\|"), where, other))
    frag["lgy_full_table"] = "\n".join(full)
    frag["lgy_n_symbols"] = str(len(manuscript))
    frag["lgy_n_lg"] = str(len(lg))
    frag["lgy_n_llsf"] = str(len(llsf))
    frag["lgy_n_in_sh"] = str(len(in_sh))
    frag["lgy_n_in_sl"] = str(len(in_sl))
    frag["lgy_n_in_both"] = str(len(in_both))
    frag["lgy_n_in_either"] = str(len(in_either))
    frag["lgy_n_in_neither"] = str(len(in_neither))
    frag["lgy_n_repeats"] = str(len(rep))
    frag["lgy_n_task"] = str(len(task))
    frag["lgy_n_task_only"] = str(len(TASK_ONLY))
    frag["lgy_n_forms"] = str(len(forms))
    colls = sorted([sy for sy, v in DECLARED_SHARED.items() if v.startswith("a collision")], key=lambda x: x.lower().lstrip("\\"))
    frag["lgy_n_collisions"] = str(len(colls))
    frag["lgy_collisions"] = ", ".join("$%s$" % c for c in colls[:-1]) + " and $%s$" % colls[-1]
    frag["lgy_verdict"] = "%s (%d of %d)" % ("ALL PASS" if npass == len(checks) else "FAIL", npass, len(checks))
    with open(OUT, "w", encoding="utf-8") as f:
        f.write("<!-- generated by code/scripts/write_lg_symbols.py from the tables of symbols of the documents; do not edit -->\n")
        for k in sorted(frag):
            f.write("<!-- BEGIN %s -->\n%s\n<!-- END %s -->\n" % (k, frag[k], k))
    print("write_lg_symbols: %d of %d checks pass; %d symbols, %d LG, %d LL/SF (%d in SM_Higgs, %d in SuperLoop)"
          % (npass, len(checks), len(manuscript), len(lg), len(llsf), len(in_sh), len(in_sl)))
    if npass != len(checks):
        for cid, ok, text in checks:
            if not ok:
                print("FAIL", cid, text[:2000])


if __name__ == "__main__":
    main()
