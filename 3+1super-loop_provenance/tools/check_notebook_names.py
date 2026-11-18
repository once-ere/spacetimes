#!/usr/bin/env python3
"""check_notebook_names.py      (PROJECT spacetimes, notebook 3+1super-loop.nb)

PURPOSE
  "One name, one object" for the names of the PROGRAM of the notebook.  A notebook is evaluated in one
  session: a name that receives a value at the top level of an input cell (a "global" name, for example
  mass, lat or k) keeps this value in all later cells.  If a later chapter uses such a name for another
  object, or forgets to define its own variable and so reads the old value, the program is wrong
  without any message.  This script checks two rules for every chapter of the notebook:

    N1  a global name of an EARLIER chapter occurs in the code of a chapter (in any role) only if it
        is declared in the table SHARED below, with the reason why it denotes the SAME object;
    N2  a name that is global (in the chapter itself or in an earlier chapter) is not used in the
        chapter as a local name -- a variable of Module, With or Block, the variable of a loop, an
        argument of Function or Compile, or the name of a pattern such as x_ -- unless it is declared
        in the table SAME_ARGUMENT below (an argument of a function that carries the same quantity as
        the global name, for example the chemical potential mu_ passed to a function).

  The names of the library of Part II (they begin with t2), of the package of Part I (they begin with
  SL31) and the function LL are the same objects everywhere and are exempt from N1.
  Exit status 1 if a rule is broken.

HOW THE CODE IS READ
  The source of the notebook is notebook_source/3+1super-loop.source.wl; a line
  (* @@INCLUDECELLS file@@ *) inserts a chapter kept in its own file, and a line (* @@INCLUDE file@@ *)
  inside an input cell inserts a package or a file of data.  The chapters are: the main source (Part I),
  then the included files in the order of the notebook; the files of the table GROUPS form one chapter
  (chapter II.2, item [1], is kept in two files).  From every input cell the comments and the strings
  are removed; the rest is cut into tokens and the brackets are matched.
  A name is GLOBAL in a chapter if it is the target of an assignment (name = ..., name[...] := ...,
  {name1, name2} = ..., Format[name[...]] := ...) at a place where it is not a local name.  Local names
  are: the variables of Module, With, Block and DynamicModule, the variables of the loops Do, Table,
  Sum, Product and ParallelTable (the first name of an iterator {name, ...}), the arguments of Function
  and of Compile, and the names of patterns (a name directly followed by an underscore).  A file
  inserted by (* @@INCLUDE file@@ *) is a package (its names live in the context SL31` and its private
  context, and it gives no global names) or data (it gives the names that it assigns at the beginning
  of a line).

USAGE (in the top folder of the repository)
  python3 3+1super-loop_provenance/tools/check_notebook_names.py [report file]
"""
import os
import re
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SRC_DIR = os.path.join(REPO, "3+1super-loop_provenance", "notebook_source")
MAIN = "3+1super-loop.source.wl"

# global names of earlier chapters that a later chapter (from --from on) may use: name -> the reason why it is the
# same object
SHARED = {
    "gv": "the coupling g^v_LL of the isospin potentials; the original programs t2_fields and t2_analytic both set it to 0.7",
    "gf": "the coupling g^F_LL of the fermion-number potential; both original programs set it to 0.45",
    "mass": "the mass m_LL of the spinor charges; both original programs set it to 0.4",
    "fro": "the function of chapter II.2 that returns the Frobenius norm of a matrix",
    "upright": "the function of chapter II.2 that keeps the letters of a label of a figure upright",
    "blue": "a color of the figures of the project (#2a78d6)",
    "orange": "a color of the figures of the project (#eb6834)",
    "aqua": "a color of the figures of the project (#1baf7a)",
    "grey": "a color of the figures of the project (#8a8984)",
    "muted": "the color of the texts inside the figures of the project (#5f5e5a)",
    "mSym": "the registered symbol m_LL as a label of a figure (chapter II.2)",
    "ellxSym": "the registered symbol of the lattice spacing as a label of a figure (chapter II.2)",
    "rWSym": "the registered symbol of the Wilson coefficient as a label of a figure (chapter II.2)",
    "q1Sym": "the registered symbol of the first component of the momentum as a label of a figure (chapter II.2)",
    "epsLatSym": "the registered symbol of the lattice energy as a label of a figure (chapter II.2)",
    "x0Sym": "the registered symbol of the time x^0_LL as a label of a figure (chapter II.2)",
}
# global names that a chapter may use as the name of an ARGUMENT of a function (a pattern name), because the argument
# carries the same quantity: name -> the reason
SAME_ARGUMENT = {
    "gv": "an argument of the functions gauge8 and nabla of chapter II.2: the coupling g^v_LL",
    "gf": "an argument of the function gauge8 of chapter II.2: the coupling g^F_LL",
    "mu": "an argument of the function hq of chapter II.2: the chemical potential mu_LL",
}
# files that form one chapter
GROUPS = [["part2_B_item1.source.wl", "part2_C_item1b.source.wl"]]
LIBRARY = re.compile(r"^(t2[A-Za-z0-9]*|SL31[A-Za-z0-9$]*|LL)$")

MARK = re.compile(r"^\(\* ::(\w+)(?:::\w+)?:: \*\)[ \t]*$", re.M)
INCLUDECELLS = re.compile(r"^\(\* @@INCLUDECELLS ([^@]+)@@ \*\)[ \t]*$", re.M)
INCLUDE = re.compile(r"\(\* @@INCLUDE ([^@]+)@@ \*\)")

TOKEN = re.compile(r"""
    (?P<string>"(?:[^"\\]|\\.)*")
  | (?P<name>[A-Za-z$][A-Za-z0-9$]*(?:`[A-Za-z$][A-Za-z0-9$]*)*`?)
  | (?P<number>\d+(?:\.\d*)?(?:\*\^-?\d+)?|\.\d+)
  | (?P<op>===|=!=|:=|\^:=|\^=|==|!=|<=|>=|\+=|-=|\*=|/=|->|:>|\[\[|\]\]|<\||\|>|//|/@|@@|&&|\|\||\+\+|--|.)
""", re.X | re.S)

SCOPES = {"Module", "With", "Block", "DynamicModule"}
LOOPS = {"Do", "Table", "Sum", "Product", "ParallelTable"}
WRAPPERS = {"Format", "Attributes", "Options", "N", "Default", "SyntaxInformation"}
# assignments to these forms attach a rule to a symbol that exists already (often the variable of a pattern)
NOT_A_DEFINITION = {"MakeBoxes", "MessageName"}


def strip_comments(code):
    """the code without its comments (comments may be nested); strings are kept"""
    out, i, depth, n = [], 0, 0, len(code)
    while i < n:
        if depth == 0 and code[i] == '"':
            m = re.compile(r'"(?:[^"\\]|\\.)*"', re.S).match(code, i)
            j = m.end() if m else n
            out.append(code[i:j])
            i = j
        elif code.startswith("(*", i):
            depth += 1
            i += 2
        elif depth > 0 and code.startswith("*)", i):
            depth -= 1
            i += 2
            if depth == 0:
                out.append(" ")
        elif depth > 0:
            i += 1
        else:
            out.append(code[i])
            i += 1
    return "".join(out)


def tokens(code):
    out = []
    for m in TOKEN.finditer(strip_comments(code)):
        kind = m.lastgroup
        text = m.group(0)
        if kind == "op" and text.isspace():
            continue
        out.append((kind, text))
    return out


def tree(toks):
    """nested lists: a group is ('grp', opening bracket, [items]); [[ ]] is a group of its own"""
    close = {"[": "]", "{": "}", "(": ")", "[[": "]]", "<|": "|>"}
    stack = [[]]
    opens = []
    for kind, text in toks:
        if kind == "op" and text in close:
            opens.append(text)
            stack.append([])
        elif kind == "op" and opens and text == close[opens[-1]]:
            items = stack.pop()
            stack[-1].append(("grp", opens.pop(), items))
        elif kind == "op" and text == "]]" and opens and opens[-1] == "[":
            # "]]" that closes two single brackets, as in f[g[x]]
            for _ in range(2):
                if opens and opens[-1] == "[":
                    items = stack.pop()
                    stack[-1].append(("grp", opens.pop(), items))
        else:
            stack[-1].append((kind, text))
    while len(stack) > 1:                       # unbalanced input: close what is open
        items = stack.pop()
        stack[-1].append(("grp", opens.pop() if opens else "[", items))
    return stack[0]


def split(items, sep):
    """the parts of a token list between the separator tokens"""
    parts, cur = [], []
    for it in items:
        if it[0] == "op" and it[1] in sep:
            parts.append(cur)
            cur = []
        else:
            cur.append(it)
    parts.append(cur)
    return parts


def first_names(group_items):
    """the first name of every element of a list {a, b = 1, {c, 0, 3}}: used for local variables and iterators"""
    names = []
    for part in split(group_items, {","}):
        if part and part[0][0] == "name":
            names.append(part[0][1])
        elif part and part[0][0] == "grp" and part[0][1] == "{":      # Compile: {{a, _Real, 1}, ...}
            inner = part[0][2]
            if inner and inner[0][0] == "name":
                names.append(inner[0][1])
    return names


def target_names(lhs):
    """the names that an assignment with the left side lhs defines"""
    lhs = [it for it in lhs]
    if not lhs:
        return []
    head = lhs[0]
    if head[0] == "name":
        if head[1] in NOT_A_DEFINITION:
            return []
        if head[1] in WRAPPERS and len(lhs) > 1 and lhs[1][0] == "grp":
            return target_names(lhs[1][2])
        return [head[1]]
    if head[0] == "grp" and head[1] == "{" and len(lhs) == 1:
        return first_names(head[2])
    return []


def walk(items, bound, globals_found, used, locals_found):
    """collects the global assignment targets, all names used, and all names used as local names (of scoping constructs,
    loops, functions and patterns); bound: the local names at this place"""
    for k, it in enumerate(items):                 # the name of a pattern: a name directly followed by an underscore
        if it[0] == "name" and k + 1 < len(items) and items[k + 1] == ("op", "_"):
            locals_found.add(it[1])
    # the scoping constructs and the loops
    k = 0
    while k < len(items):
        it = items[k]
        if it[0] == "name":
            used.add(it[1])
            nxt = items[k + 1] if k + 1 < len(items) else None
            if nxt and nxt[0] == "grp" and nxt[1] == "[":
                args = split(nxt[2], {","})
                local = set()
                if it[1] in SCOPES and args and args[0] and args[0][0][0] == "grp" and args[0][0][1] == "{":
                    local = set(first_names(args[0][0][2]))
                elif it[1] in LOOPS:
                    for a in args[1:]:
                        if len(a) == 1 and a[0][0] == "grp" and a[0][1] == "{":
                            inner = a[0][2]
                            if inner and inner[0][0] == "name" and len(split(inner, {","})) > 1:
                                local.add(inner[0][1])
                elif it[1] == "Function" and len(args) > 1 and args[0]:
                    if args[0][0][0] == "name" and len(args[0]) == 1:
                        local = {args[0][0][1]}
                    elif args[0][0][0] == "grp" and args[0][0][1] == "{":
                        local = set(first_names(args[0][0][2]))
                elif it[1] == "Compile" and args and args[0] and args[0][0][0] == "grp":
                    local = set(first_names(args[0][0][2]))
                locals_found |= local
                walk(nxt[2], bound | local, globals_found, used, locals_found)
                k += 2
                continue
        elif it[0] == "grp":
            walk(it[2], bound, globals_found, used, locals_found)
        k += 1
    # the assignments of this level: the left side reaches back to the previous separator
    start = 0
    for k, it in enumerate(items):
        if it[0] == "op" and it[1] in (";", ","):
            start = k + 1
        elif it[0] == "op" and it[1] in ("=", ":=", "^=", "^:="):
            for name in target_names(items[start:k]):
                if name not in bound and "`" not in name:
                    globals_found.add(name)
            start = k + 1
        elif it[0] == "op" and it[1] in ("&", "/;"):
            pass


def input_cells(text):
    marks = list(MARK.finditer(text))
    for k, m in enumerate(marks):
        end = marks[k + 1].start() if k + 1 < len(marks) else len(text)
        if m.group(1) == "Input":
            yield text[m.end():end]


def data_file_globals(path):
    """the names that a data file assigns at the beginning of a line (a package file gives none)"""
    text = open(path, encoding="utf-8").read()
    if "BeginPackage[" in text:
        return set()
    return set(re.findall(r"(?m)^([A-Za-z$][A-Za-z0-9$]*) = ", strip_comments(text)))


def analyse(text):
    """(global names, all names used, names used as local names) of the input cells of a chapter"""
    globals_found, used, locals_found = set(), set(), set()
    for cell in input_cells(text):
        for inc in INCLUDE.findall(cell):
            globals_found |= data_file_globals(os.path.join(SRC_DIR, inc))
        walk(tree(tokens(cell)), set(), globals_found, used, locals_found)
    return globals_found, used, locals_found


def main():
    report_path = sys.argv[1] if len(sys.argv) > 1 else None
    main_text = open(os.path.join(SRC_DIR, MAIN), encoding="utf-8").read()
    chapters = [(MAIN + " (Part I)", INCLUDECELLS.sub("", main_text))]
    for name in INCLUDECELLS.findall(main_text):
        text = open(os.path.join(SRC_DIR, name), encoding="utf-8").read()
        group = next((g for g in GROUPS if name in g), None)
        if group and name != group[0]:
            chapters[-1] = (chapters[-1][0] + " + " + name, chapters[-1][1] + "\n" + text)
        else:
            chapters.append((name, text))
    earlier = {}                                 # global name -> the chapter that defined it first
    errors = 0
    lines = ["PROJECT spacetimes, notebook 3+1super-loop.nb: the names of the program, chapter by chapter",
             "rule N1: a global name of an earlier chapter is used only if it is declared as shared (the same object)",
             "rule N2: a global name is not used as a local name, except as a declared argument of a function",
             "(the names of the library, which begin with t2 or SL31, and the function LL are exempt from N1)", ""]
    for name, text in chapters:
        g, used, local = analyse(text)
        reused = sorted(n for n in used if n in earlier and not LIBRARY.match(n))
        clash = sorted(n for n in local if (n in earlier or n in g))
        lines.append("chapter %s: %d global names defined, %d names used, %d local names" % (name, len(g), len(used), len(local)))
        for n in reused:
            if n in SHARED:
                lines.append("    N1 shared: %s (first defined in %s) -- %s" % (n, earlier[n], SHARED[n]))
            else:
                lines.append("    N1 ERROR: %s is a global name of %s and is used here without a declaration" % (n, earlier[n]))
                errors += 1
        for n in clash:
            if n in SAME_ARGUMENT:
                lines.append("    N2 the same quantity as an argument: %s -- %s" % (n, SAME_ARGUMENT[n]))
            else:
                lines.append("    N2 ERROR: %s is a global name (of %s) and is used here as a local name" % (n, earlier.get(n, "this chapter")))
                errors += 1
        for n in g:
            earlier.setdefault(n, name)
    lines.append("")
    lines.append("%d chapters, %d global names, %d errors" % (len(chapters), len(earlier), errors))
    report = "\n".join(lines) + "\n"
    if report_path:
        with open(report_path, "w", encoding="utf-8") as f:
            f.write(report)
    print(report, end="")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
