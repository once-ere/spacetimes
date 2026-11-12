#!/usr/bin/env python3
"""
scan_notebook_symbols.py -- list the symbols that the Mathematica notebooks of the
project folder define, and find the symbols that more than one notebook defines.

Why this tool exists
--------------------
Hard Rule 5 of the task specification: the original files use the same symbol for
different objects, and the project must resolve this completely.  The MASTER provenance
(provenance/P1_master_symbols) rests on two machine-made lists:

  * provenance/generated/notebook_symbols.csv   every symbol found here, with the
    notebooks that define it;
  * the fragment `nb_collision_table` in provenance/generated/notebook_symbols.md: the
    symbols defined in two or more notebooks, with the notebooks, and whether the
    defining statements are the same text in all of them.

code/scripts/check_symbols.py then verifies that no symbol of PROJECT spacetimes
coincides with any symbol of any notebook.

What counts as "a notebook defines a symbol"
--------------------------------------------
The notebook is converted to plain text, cell by cell, with tools/nb2txt.py (no
Mathematica needed).  Typeset input appears in linear form: a subscript as  P_L , a
superscript as  T16^A , an overbar as  Over[τ,_] .  Output, Print and Message cells are
ignored.

(a) CODE (Input cells).  Comments (* ... *) and the contents of strings are removed.
    Then EVERY assignment  lhs = rhs ,  lhs := rhs ,  lhs ^= rhs ,  lhs ^:= rhs  is
    found, wherever it stands: at the start of a line, inside parentheses, inside
    Do[...], Table[...], If[...], after a semicolon, or as part of a chain a = b = c.
    The recorded symbol is the head of the left-hand side, with its typeset decoration:
        η4488 = ...            ->  η4488
        (P_L = ...) // f       ->  P_L
        Do[s4by4[h] = ..., ..] ->  s4by4
        T16^A[8] = ...         ->  T16^A
        Over[τ,_][0] = ...     ->  Over[τ,_]
        f[x_] := ...           ->  f
        {a, b} = ...           ->  a and b
    Not recorded: comparisons (==, ===, !=, <=, >=), the updates += -= *= /=, Unset
    (=.), the local variables of Module, With, Block and DynamicModule (the names
    declared in the first argument, and assignments to them inside the call), the
    limits of typeset sums and products, left-hand sides whose head is a built-in
    function (Attributes[f] = ..., Format[...] := ..., and the like) and system or
    package settings (names that begin with $).
(b) DECLARATIONS of the tensor package xAct, in code:
        DefManifold[M, dim, indices]      ->  M, and every abstract index of the list
        DefChart[ch, M, values, coords]   ->  ch, and every coordinate of the list
        DefMetric[sign, g[-a,-b], CD,...] ->  g and CD
        DefConstantSymbol[c], DefScalarFunction[f], DefTensor[T[...], ...]  ->  c, f, T
    If the list of indices or of coordinates is given by a variable, the list assigned
    to that variable in the same notebook is used.  Names that xAct derives by itself
    from these declarations (RiemannCD, epsilong44, ...) are not recorded.
(c) TEXT (Text, Item and caption cells, headings).  A formula written in the prose in
    the form "name = ..." introduces notation that a reader of the notebook meets, so
    it is recorded as well: the single symbol (with optional arguments in brackets)
    that stands immediately to the left of a single "=", unless an operator stands
    before it (then it is the end of a longer expression, not a definition).  For
    "A, B = 0 ... 7" all names of the comma-separated list are recorded.  A symbol that
    a notebook introduces ONLY in this way is marked with an asterisk in the lists.
    Limits of this rule: a product written without a blank ("κε = ...") is read as one
    name, a power with a letter as exponent ("x0^p = ...") as a decorated name, and a
    bare word of four or more lower-case letters is taken as prose and not recorded.

A notebook can define a symbol several times with different meanings; only the first
definition is used for the comparison between notebooks (a definition in code before a
definition in prose).  The MASTER provenance treats such cases by hand.

"Same text": the first defining statement of the symbol in each notebook (left-hand
side, operator and right-hand side up to the end of the assignment), with all blanks
and line breaks removed, is identical in all notebooks that define it.  Different text does not prove that the objects differ, and identical text does not
prove that they agree (the right-hand sides may contain symbols that differ); the flag
is a guide for the reader of the MASTER provenance, which treats every symbol that
matters for the project individually.

The defining statements themselves are not written to the files of the repository (the
notebooks are unpublished work of the author).  For checking by hand they can be
written to a file outside the repository with the option --dump FILE.

Usage (from the repository root)
    python3 tools/scan_notebook_symbols.py [directory with the .nb files] [--dump FILE]
The default directory is the repository root, where the author keeps the notebooks
(they are not part of the repository; provenance/inputs_sha256.txt pins them).  The
script refuses to run if a notebook is missing or its SHA-256 checksum differs.
"""
import collections
import csv
import hashlib
import json
import os
import re
import subprocess
import sys
import tempfile

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
OUT_CSV = os.path.join(ROOT, "provenance", "generated", "notebook_symbols.csv")
OUT_MD = os.path.join(ROOT, "provenance", "generated", "notebook_symbols.md")

# ---------------------------------------------------------------------------- names
# first character of a name: Latin or Greek letter, $, a letter-like symbol (double-
# struck, script, ...), or a named character that nb2txt could not translate, \[Name]
_L1 = r"A-Za-z$Ͱ-Ͽἀ-῿ℂ-ⅉ\U0001D400-\U0001D7FF"
_LN = _L1 + r"0-9`"
NAME = r"(?:[%s]|\\\[[A-Za-z]+\])(?:[%s]|\\\[[A-Za-z]+\])*" % (_L1, _LN)
# typeset decoration as nb2txt writes it: ^x, _x, ^(...), _(...), possibly repeated
ADORN = r"(?:[\^_](?:[A-Za-z0-9α-ωΑ-Ω]+|\((?:[^()]|\([^()]*\))*\)))*"
HEAD = r"(?:(?:Over|Under)\[(?:[^\[\]]|\\\[[A-Za-z]+\])*\]|%s%s)" % (NAME, ADORN)
RE_HEAD = re.compile(HEAD)
RE_NAME_ONLY = re.compile(r"^%s%s$" % (NAME, ADORN))
# a typeset exponent that is an expression, as opposed to a label such as (0), (k)
RE_LONG_EXPONENT = re.compile(r"\^\((?!(?:\d{1,2}|[A-Za-zα-ωΑ-Ω])\))")
# a member of a list of names in the prose ("A, B, C = 0 ... 7"): one or two letters, digits
RE_LIST_MEMBER = re.compile(r"^(?:[A-Za-zͰ-Ͽ]{1,2}\d*)%s$" % ADORN)
# a word of the prose (as opposed to a symbol): Latin letters only, at least two
RE_PROSE_WORD = re.compile(r"^[A-Za-z][a-z]+[.:]?$")
# a bare word of four or more lower-case Latin letters in a text cell is taken as prose
RE_BARE_WORD = re.compile(r"^[a-z]{4,}$")

# left-hand sides with these heads set a property of another symbol, or are not
# definitions of a notebook symbol at all
BUILTIN_HEADS = set("""
Attributes Options Format MakeBoxes MakeExpression N Default SyntaxInformation Evaluate
Part Dimensions Length Tr Det Transpose Inverse Dot Times Plus Power Sqrt Exp Log Sin Cos
Derivative D Dt Integrate Sum Product Table Do If Print Simplify FullSimplify Expand
ExpandAll Solve Subscript Superscript OverBar Overscript Underscript MessageName
UpValues DownValues OwnValues SubValues Unprotect Protect Clear ClearAll Set SetDelayed
Abs Re Im Conjugate ConjugateTranspose Norm Sign Min Max Mod Det Rank MatrixRank
Eigenvalues Eigenvectors Eigensystem KroneckerDelta LeviCivitaTensor Signature
IdentityMatrix DiagonalMatrix MatrixForm TraditionalForm First Last Rest Most
Out In Slot SlotSequence Function Hold HoldForm Unevaluated
Over Under UnderOver
""".split())

# the prose of a text cell: words that are certainly not symbols
PROSE_STOP = set("""
a an the is are was were be been and or not if then else so that this these those it its
with without where when while which what who whose for from to of on in at by as into
let set put take use using used define defined definition denote denotes denoted write
written call called named name names note notes case cases example examples i.e e.g
""".split())


def pinned_notebooks():
    """[(id, file name, checksum)] of the notebooks, in the order of provenance/inputs_sha256.txt."""
    out = []
    with open(os.path.join(ROOT, "provenance", "inputs_sha256.txt"), encoding="utf-8") as f:
        for line in f:
            line = line.rstrip("\n")
            if line and not line.startswith("#"):
                digest, name = line.split("  ", 1)
                if name.endswith(".nb"):
                    out.append(("N%02d" % (len(out) + 1), name, digest))
    return out


# ---------------------------------------------------------------------------- cells
RE_CELL = re.compile(r"^#(\d+) \[([A-Za-z0-9]*)\](.*)$")
CODE_STYLES = {"Input", "Code"}
SKIP_STYLES = {"Output", "Print", "Message", "Echo"}


def cells(text):
    """[(style, body)] of the output of nb2txt.py.  Headings carry their text in the header line."""
    out, style, body = [], None, []
    for line in text.split("\n"):
        m = RE_CELL.match(line)
        if m:
            if style is not None:
                out.append((style, "\n".join(body)))
            style = m.group(2)
            body = [m.group(3)] if style not in CODE_STYLES and style not in SKIP_STYLES and style else []
        elif line.startswith("#" * 70):
            continue
        elif style is not None:
            body.append(line)
    if style is not None:
        out.append((style, "\n".join(body)))
    return out


# ---------------------------------------------------------------------------- cleaning
def strip_comments(s):
    """remove (* ... *), nested; an unterminated comment removes the rest of the cell"""
    out, i, n, depth = [], 0, len(s), 0
    while i < n:
        if s.startswith("(*", i):
            depth += 1
            i += 2
        elif depth and s.startswith("*)", i):
            depth -= 1
            i += 2
        elif depth:
            if s[i] == "\n":
                out.append("\n")
            i += 1
        else:
            out.append(s[i])
            i += 1
    return "".join(out)


def blank_strings(s):
    """replace the contents of "..." by blanks (strings do not span lines here)"""
    return re.sub(r'"(?:[^"\\\n]|\\.)*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', s)


def matching(s, i):
    """index of the bracket that closes the bracket at s[i]; -1 if there is none"""
    pairs = {"[": "]", "{": "}", "(": ")"}
    stack = []
    for k in range(i, len(s)):
        c = s[k]
        if c in pairs:
            stack.append(pairs[c])
        elif c in ")]}":
            if not stack or stack.pop() != c:
                return -1
            if not stack:
                return k
    return -1


def hide_equal_signs(s, opener_regex, group_open):
    """in every bracket group that starts where opener_regex matches (the group opens at the
    character group_open, the last character of the match), replace '=' by a placeholder"""
    out = list(s)
    for m in re.finditer(opener_regex, s):
        i = m.end() - 1
        assert s[i] == group_open
        j = matching(s, i)
        if j < 0:
            j = len(s) - 1
        for k in range(i, j + 1):
            if out[k] == "=":
                out[k] = "\x00"
    return "".join(out)


RE_SCOPE = re.compile(r"\b(?:Module|With|Block|DynamicModule)\[\s*\{")


def local_scopes(s):
    """[(start, end, names)] for every Module, With, Block, DynamicModule in the cleaned code
    s: the span of the call and the names that its first argument declares as local"""
    out = []
    for m in RE_SCOPE.finditer(s):
        call_open = s.index("[", m.start())
        call_close = matching(s, call_open)
        list_open = m.end() - 1
        list_close = matching(s, list_open)
        if list_close < 0:
            continue
        if call_close < 0:
            call_close = len(s) - 1
        names = set()
        for part in split_top(s[list_open + 1:list_close]):
            name = re.split("[\x00=]", part)[0].strip()
            if RE_NAME_ONLY.match(name):
                names.add(name)
        out.append((call_open, call_close, names))
    return out


def is_local(scopes, pos, sym):
    return any(a <= pos <= b and sym in names for a, b, names in scopes)


def clean_code(s):
    s = blank_strings(strip_comments(s))
    s = hide_equal_signs(s, r"\b(?:Module|With|Block|DynamicModule)\[\s*\{", "{")   # local variables
    s = hide_equal_signs(s, r"\b(?:UnderOver|Under)\[", "[")                          # limits of sums
    return s


# ---------------------------------------------------------------------------- assignments
def is_assignment(s, i):
    """True if s[i] == '=' is (the last character of) an assignment operator = := ^= ^:="""
    if s[i] != "=":
        return False
    prev = s[i - 1] if i > 0 else ""
    nxt = s[i + 1] if i + 1 < len(s) else ""
    if nxt == "=" or prev in ("=", "!", "<", ">", "+", "-", "*", "/"):
        return False                      # == === != <= >= =!= += -= *= /=
    if nxt == "!" and s[i + 2:i + 3] == "=":
        return False                      # =!=
    if nxt == "." and not s[i + 2:i + 3].isdigit():
        return False                      # =.  (Unset)
    return True


def operator_start(s, i):
    """index of the first character of the assignment operator that ends at s[i]"""
    j = i
    if j > 0 and s[j - 1] == ":":
        j -= 1
    if j > 0 and s[j - 1] == "^":
        j -= 1
    return j


def lhs_of(s, op):
    """the left-hand side of the assignment whose operator starts at s[op]: back to the
    nearest delimiter (start, newline, ; , an unmatched opening bracket, another assignment)"""
    depth, k = 0, op - 1
    while k >= 0:
        c = s[k]
        if c in ")]}":
            depth += 1
        elif c in "([{":
            if depth == 0:
                break
            depth -= 1
        elif depth == 0:
            if c in ",\n":
                break
            if c == ";" and not (k > 0 and s[k - 1] == "/"):
                break
            if c == "=" and is_assignment(s, k):
                break
            if c == "&" or c == "|":
                break
        k -= 1
    return s[k + 1:op]


def rhs_end(s, i):
    """index just behind the right-hand side of the assignment whose operator ends at s[i]:
    the next ; , or newline outside brackets, or the bracket that closes the enclosing group"""
    depth, k, n = 0, i + 1, len(s)
    while k < n:
        c = s[k]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            if depth == 0:
                break
            depth -= 1
        elif depth == 0 and c in ";,\n":
            if c != "\n" or s[i + 1:k].strip():      # a line break directly after the operator is not the end
                break
        k += 1
    return k


def split_top(s, sep=","):
    """split at separators that are not inside brackets"""
    out, depth, cur = [], 0, []
    for c in s:
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        if c == sep and depth == 0:
            out.append("".join(cur))
            cur = []
        else:
            cur.append(c)
    out.append("".join(cur))
    return out


def head_of(lhs):
    """the symbol that an expression  head[args]...[args]  /; condition  defines, or None"""
    t = lhs.strip()
    if "/:" in t:                         # tag /: lhs
        t = t.split("/:", 1)[0].strip()
    m = RE_HEAD.match(t)
    if not m:
        return None
    sym, rest = m.group(0), t[m.end():]
    while rest.startswith("["):           # arguments, parts
        j = matching(rest, 0)
        if j < 0:
            return None
        rest = rest[j + 1:]
    rest = rest.strip()
    if rest and not rest.startswith("/;"):
        return None
    return sym


def symbols_of_lhs(lhs):
    t = lhs.strip()
    if t.startswith("{") and matching(t, 0) == len(t) - 1:      # {a, b} = ...
        out = []
        for part in split_top(t[1:-1]):
            h = head_of(part)
            if h:
                out.append(h)
        return out
    h = head_of(t)
    return [h] if h else []


def acceptable(sym):
    if sym.startswith(("Over[", "Under[")):          # a typeset overbar or underbar on a symbol
        return len(sym) <= 40
    base = re.match(NAME, sym)
    base = base.group(0) if base else sym
    if sym.startswith("$") or base in BUILTIN_HEADS:
        return False
    return len(sym) <= 40


def normalize(sym):
    """in the prose a numerical exponent is a power, not part of a name:  E^2 -> E"""
    return re.sub(r"\^\d+$", "", sym)


def scan_code(body, found, all_code):
    s = clean_code(body)
    scopes = local_scopes(s)
    for i, c in enumerate(s):
        if c == "=" and is_assignment(s, i):
            op = operator_start(s, i)
            lhs = lhs_of(s, op)
            stmt = (lhs.strip() + " " + s[op:rhs_end(s, i)]).strip()
            for sym in symbols_of_lhs(lhs):
                if acceptable(sym) and not is_local(scopes, i, sym):
                    found.setdefault(sym, ("code", stmt[:4000]))
    scan_declarations(s, found, all_code)


# ---------------------------------------------------------------------------- xAct
RE_DEF = re.compile(r"\bDef(Manifold|Chart|Metric|ConstantSymbol|ScalarFunction|Tensor|CovD)\[")
RE_VALUE_ASSIGN = r"(?:^|[;\n(\[{,])\s*%s\s*=\s*"


def list_assigned_to(s, name, depth=0):
    """the elements of the first list assigned to `name` in the code s, or None.  The value
    may be a list {a, b, ...}, or Flatten[{u, v}] or Join[u, v] of variables that are lists."""
    m = re.search(RE_VALUE_ASSIGN % re.escape(name), s)
    if not m or depth > 3:
        return None
    rest = s[m.end():]
    if rest.startswith("{"):
        j = matching(rest, 0)
        return [a.strip() for a in split_top(rest[1:j])] if j > 0 else None
    mm = re.match(r"(?:Flatten\[\s*\{|Join\[)", rest)
    if mm:
        i = mm.end() - 1
        j = matching(rest, i)
        if j < 0:
            return None
        out = []
        for part in split_top(rest[i + 1:j]):
            sub = list_assigned_to(s, part.strip(), depth + 1)
            if sub is None:
                return None
            out += sub
        return out
    return None


def scan_declarations(s, found, all_code):
    """s: the code of one cell; all_code: the code of all cells of the notebook (to look up
    a list of indices or coordinates that is given by a variable)"""
    for m in RE_DEF.finditer(s):
        kind = m.group(1)
        i = m.end() - 1
        j = matching(s, i)
        if j < 0:
            continue
        args = [a.strip() for a in split_top(s[i + 1:j])]

        def rec(sym, text):
            sym = sym.strip()
            if RE_NAME_ONLY.match(sym) and acceptable(sym):
                found.setdefault(sym, ("code", text))

        def as_list(arg):
            if arg.startswith("{") and arg.endswith("}"):
                return split_top(arg[1:-1])
            return list_assigned_to(all_code, arg) or []

        if kind == "Manifold" and len(args) >= 3:
            rec(args[0], "manifold (DefManifold)")
            for a in as_list(args[2]):
                rec(a, "abstract index of a manifold (DefManifold)")
        elif kind == "Chart" and len(args) >= 4:
            rec(args[0], "chart (DefChart)")
            values = [v.strip() for v in as_list(args[2])]
            for pos, a in enumerate(as_list(args[3])):
                a = re.sub(r"\[\s*\]$", "", a.strip())
                rec(a, "coordinate number %d of %d of a chart, index value %s (DefChart)"
                    % (pos + 1, len(values), values[pos] if pos < len(values) else "?"))
        elif kind == "Metric" and len(args) >= 3:
            rec(re.sub(r"\[.*$", "", args[1], flags=re.S), "metric of signature %s (DefMetric)" % args[0].replace(" ", ""))
            rec(args[2], "covariant derivative of a metric (DefMetric)")
        elif kind in ("ConstantSymbol", "ScalarFunction") and args:
            rec(args[0], "constant symbol (DefConstantSymbol)" if kind == "ConstantSymbol" else "scalar function (DefScalarFunction)")
        elif kind in ("Tensor", "CovD") and args:
            rec(re.sub(r"\[.*$", "", args[0], flags=re.S), "tensor (DefTensor)" if kind == "Tensor" else "covariant derivative (DefCovD)")


# ---------------------------------------------------------------------------- text cells
def boundary_before(s, b):
    """True if what stands before position b+1 (blanks skipped) allows a definition to start
    there: the start of a line, one of , ; : ( [ { =, or a word of the prose.  False if a
    number, a closing bracket, an operator or another symbol stands there (then the token
    is a factor of a longer expression).  Returns (ok, index of the character found)."""
    while b >= 0 and s[b] in " \t":
        b -= 1
    if b < 0 or s[b] == "\n":
        return True, b
    if s[b] in ",;:([{=":
        return True, b
    e = b + 1
    while b >= 0 and not s[b].isspace():
        b -= 1
    return bool(RE_PROSE_WORD.match(s[b + 1:e])), e - 1


def scan_text(body, found):
    s = clean_code(body) if "(*" in body or "[" in body else body     # drafts of code in text cells
    scopes = local_scopes(s)
    for i, c in enumerate(s):
        if c != "=" or not is_assignment(s, i):
            continue
        if i + 1 < len(s) and s[i + 1] == ">":          # => in prose
            continue
        op = operator_start(s, i)
        # the token immediately to the left: a head without blanks, then bracket groups
        k = op - 1
        while k >= 0 and s[k] in " \t":
            k -= 1
        end = k + 1
        depth = 0
        while k >= 0:
            ch = s[k]
            if ch in ")]}":
                depth += 1
            elif ch in "([{":
                if depth == 0:
                    break
                depth -= 1
            elif depth == 0 and (ch.isspace() or ch in ",;:="):
                break
            k -= 1
        token = s[k + 1:end]
        sym = head_of(token)
        if not sym or RE_LONG_EXPONENT.search(sym):     # K^(-2(p1 - q1)) is a power, not a name
            continue
        ok, b = boundary_before(s, k)
        if not ok:
            continue
        names = [sym]
        # "A, B = 0 ... 7": the other names of the comma-separated list
        while b >= 0 and s[b] == "," and len(names) < 8:
            e = b
            b -= 1
            while b >= 0 and s[b] in " \t":
                b -= 1
            e2 = b + 1
            while b >= 0 and not (s[b].isspace() or s[b] in ",;:=([{"):
                b -= 1
            tok = s[b + 1:e2]
            if not RE_LIST_MEMBER.match(tok):
                break
            ok, b = boundary_before(s, b)
            if not ok:
                break
            names.append(tok)
        line_start = s.rfind("\n", 0, op) + 1
        stmt = s[max(line_start, op - 40):op + 110].split("\n")[0].strip()
        for nm in names:
            nm = normalize(nm)
            if (acceptable(nm) and nm.lower() not in PROSE_STOP and not nm.isdigit()
                    and not (RE_BARE_WORD.match(nm) and nm == token.strip())
                    and not is_local(scopes, i, nm)):
                found.setdefault(nm, ("text", stmt))


# ---------------------------------------------------------------------------- one notebook
def scan(text):
    """{symbol: (kind, first defining statement)}; kind 'code' wins over 'text'"""
    code, prose = {}, {}
    cs = cells(text)
    all_code = "\n".join(clean_code(body) for style, body in cs if style in CODE_STYLES)
    for style, body in cs:
        if style in SKIP_STYLES:
            continue
        if style in CODE_STYLES:
            scan_code(body, code, all_code)
        else:
            scan_text(body, prose)
    found = dict(prose)
    found.update(code)
    return found


def main():
    args = [a for a in sys.argv[1:]]
    dump = None
    if "--dump" in args:
        k = args.index("--dump")
        dump = args[k + 1]
        del args[k:k + 2]
    nbdir = args[0] if args else ROOT
    books = pinned_notebooks()
    defs = {}
    with tempfile.TemporaryDirectory() as tmp:
        for nid, name, digest in books:
            path = os.path.join(nbdir, name)
            if not os.path.exists(path):
                sys.exit("missing notebook: %s (give the directory of the notebooks as argument)" % path)
            with open(path, "rb") as f:
                if hashlib.sha256(f.read()).hexdigest() != digest:
                    sys.exit("checksum of %s differs from provenance/inputs_sha256.txt" % name)
            txt = os.path.join(tmp, nid + ".txt")
            subprocess.run([sys.executable, os.path.join(ROOT, "tools", "nb2txt.py"), path, txt, "300"],
                           check=True, capture_output=True)
            with open(txt, encoding="utf-8", errors="replace") as f:
                defs[nid] = scan(f.read())
    if dump:
        with open(dump, "w", encoding="utf-8") as f:
            json.dump(defs, f, ensure_ascii=False, indent=1, sort_keys=True)
    where = collections.defaultdict(list)
    for nid, _, _ in books:
        for sym in defs[nid]:
            where[sym].append(nid)
    os.makedirs(os.path.dirname(OUT_CSV), exist_ok=True)
    rows = []
    for sym in sorted(where, key=lambda s: (-len(where[s]), s)):
        nids = where[sym]
        same = "-" if len(nids) == 1 else ("yes" if len({re.sub(r"\s+", "", defs[n][sym][1]) for n in nids}) == 1 else "no")
        shown = " ".join(n + ("*" if defs[n][sym][0] == "text" else "") for n in nids)
        rows.append((sym, len(nids), shown, same))
    with open(OUT_CSV, "w", encoding="utf-8", newline="") as f:
        f.write("# generated by: tools/scan_notebook_symbols.py\n")
        f.write("# content: the symbols defined in the notebooks of the project folder (method: header of the script)\n")
        f.write("# notebooks: " + "; ".join("%s = %s" % (nid, name) for nid, name, _ in books) + "\n")
        f.write("# columns: symbol; number of notebooks that define it; their identifiers (an asterisk: introduced "
                "only in the prose of a text cell); whether the first defining statements are the same text in all "
                "of them (yes, no; - if only one notebook defines the symbol)\n")
        w = csv.writer(f)
        w.writerow(["symbol", "n_notebooks", "notebooks", "same_text"])
        w.writerows(rows)

    # ---- fragments for the MASTER provenance
    frag = []

    def add(name, text):
        frag.append("<!-- BEGIN %s -->\n%s\n<!-- END %s -->" % (name, text.rstrip("\n"), name))

    multi = [r for r in rows if r[1] >= 2]
    add("nb_n_notebooks", str(len(books)))
    add("nb_n_symbols", str(len(rows)))
    add("nb_n_multi", str(len(multi)))
    add("nb_n_multi_same", str(sum(1 for r in multi if r[3] == "yes")))
    add("nb_n_multi_diff", str(sum(1 for r in multi if r[3] == "no")))
    t = ["| Id | Notebook | Symbols defined | of these, also defined in another notebook |", "|:--|:------------------------------|----:|----:|"]
    for nid, name, _ in books:
        t.append("| %s | `%s` | %d | %d |" % (nid, name, len(defs[nid]), sum(1 for s in defs[nid] if len(where[s]) >= 2)))
    add("nb_table", "\n".join(t))

    def cell(sym):
        # a symbol is shown as code; a vertical bar or a backquote cannot be shown in a table cell
        return "`%s`" % sym.replace("|", "∣").replace("`", "'")

    # one row for every set of notebooks; a symbol carries an asterisk if at least one of the
    # notebooks introduces it only in the prose of a text cell
    groups = collections.defaultdict(lambda: {"yes": [], "no": []})
    for sym, n, nids, same in multi:
        star = "\\*" if "*" in nids else ""
        groups[nids.replace("*", "")][same].append(cell(sym) + star)
    t = ["| Notebooks | Symbols with the same defining text | Symbols with different defining text |",
         "|:-------------|:-----------------------------------------|:-----------------------------------------|"]
    for nids in sorted(groups, key=lambda g: (-len(g.split()), g)):
        t.append("| %s | %s | %s |" % (nids, ", ".join(groups[nids]["yes"]), ", ".join(groups[nids]["no"])))
    add("nb_collision_table", "\n".join(t))
    add("nb_n_groups", str(len(groups)))
    with open(OUT_MD, "w", encoding="utf-8") as f:
        f.write("<!-- generated by tools/scan_notebook_symbols.py; do not edit -->\n\n" + "\n\n".join(frag) + "\n")
    print("notebooks: %d; symbols: %d; defined in two or more notebooks: %d (same text: %d, different text: %d)"
          % (len(books), len(rows), len(multi), sum(1 for r in multi if r[3] == "yes"), sum(1 for r in multi if r[3] == "no")))
    print("wrote %s and %s" % (os.path.relpath(OUT_CSV, ROOT), os.path.relpath(OUT_MD, ROOT)))


if __name__ == "__main__":
    main()
