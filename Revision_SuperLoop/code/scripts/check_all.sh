#!/usr/bin/env bash
# check_all.sh (Revision_SuperLoop) -- run everything of the folder Revision_SuperLoop in order, and give one
# verdict.
#
# Steps:
#   1. the program superloop (all five commands; about one second), the Groebner-basis
#      classification (a few seconds) and the seven figures;
#   2. two passes of: the registry of the symbols, the fragments of the documents, the checks of
#      the symbols and of self-containedness, the assembly of the Markdown documents, the build of
#      the manuscript and of the .tex/.pdf versions of the documents.  The second pass is needed
#      because the reports of the checks, which the documents print, count the built documents
#      that the first pass creates; after the second pass every printed report is current;
#   3. the verdicts: every report in data/ must pass, no built document may contain an unresolved
#      placeholder, every report printed in a document must be the current one, no formula may end
#      directly before a digit, every document must exist as .md, .tex and .pdf, and no document
#      may name a tool used to prepare it.
# Nothing is to be committed unless the script ends with "ALL PASS".
#
# Usage (from the folder Revision_SuperLoop):   bash code/scripts/check_all.sh
set -u
cd "$(dirname "$0")/../.."
fail=0
line() { printf '%-6s %s\n' "$1" "$2"; }
run() {  # run a step quietly; print PASS or FAIL with its name; keep its output in /tmp on failure
    local name="$1"; shift
    if "$@" > /tmp/sl_step.log 2>&1; then line PASS "$name"; else line FAIL "$name (output: /tmp/sl_step_failed.log)"; cp /tmp/sl_step.log /tmp/sl_step_failed.log; fail=1; fi
}

# 1. program and figures
run "superloop all" cargo run --release --quiet --manifest-path code/superloop-rs/Cargo.toml -- all
run "malcev_groebner.py" python3 code/scripts/malcev_groebner.py
run "make_figures.py" python3 code/scripts/make_figures.py

# 2. documents, two passes
for pass in 1 2; do
    run "pass $pass: write_symbol_registry.py" python3 code/scripts/write_symbol_registry.py
    run "pass $pass: write_doc_tables.py (before the checks)" python3 code/scripts/write_doc_tables.py
    run "pass $pass: check_symbols.py" python3 code/scripts/check_symbols.py
    run "pass $pass: check_self_contained.py" python3 code/scripts/check_self_contained.py
    run "pass $pass: write_doc_tables.py (with the reports)" python3 code/scripts/write_doc_tables.py
    run "pass $pass: assemble_docs.py" python3 code/scripts/assemble_docs.py
    run "pass $pass: build_manuscript.py" python3 code/scripts/build_manuscript.py
    run "pass $pass: build_docs.sh" bash code/scripts/build_docs.sh
done

# 3. verdicts
for f in data/sl_*_checks.txt; do
    if grep -q '^\[FAIL\]' "$f"; then line FAIL "$f (contains a failed check)"; fail=1; continue; fi
    c=$(grep -m1 -oE '[0-9]+ of [0-9]+ checks passed' "$f" || true)
    if [ -n "$c" ]; then
        a=${c%% of *}; b=${c#* of }; b=${b%% *}
        if [ "$a" = "$b" ]; then line PASS "$f ($c)"; else line FAIL "$f ($c)"; fail=1; fi
    else line FAIL "$f (no verdict found)"; fail=1
    fi
done

if grep -l '{{[A-Z]*:' provenance/*.md provenance/*.tex manuscript/*.md manuscript/*.tex ./README.md ./README.tex 2>/dev/null | grep -q .; then
    line FAIL "unresolved placeholder in: $(grep -l '{{[A-Z]*:' provenance/*.md provenance/*.tex manuscript/*.md manuscript/*.tex ./README.md ./README.tex 2>/dev/null | tr '\n' ' ')"; fail=1
else
    line PASS "no unresolved placeholder in the built documents"
fi

stale=$(python3 - <<'PY'
import glob, os, re
out = []
for src in sorted(glob.glob("docs_src/*.src.md")):
    text = open(src, encoding="utf-8").read()
    m = re.match(r"<!-- OUTPUT: (.+?) -->", text)
    if not m or not os.path.exists(m.group(1)):
        out.append("%s (not assembled)" % src)
        continue
    built = open(m.group(1), encoding="utf-8").read()
    for f in re.findall(r"\{\{FILE:([^}]+)\}\}", text):
        if not os.path.exists(f) or open(f, encoding="utf-8").read().strip() not in built:
            out.append("%s prints an outdated %s" % (m.group(1), f))
print("; ".join(out))
PY
)
if [ -n "$stale" ]; then line FAIL "$stale"; fail=1
else line PASS "every report printed in a document is the current report"; fi

bad=$(python3 - <<'PY'
import glob, re
out = []
for f in sorted(glob.glob("*.md") + glob.glob("provenance/*.md") + glob.glob("manuscript/*.md")):
    text = open(f, encoding="utf-8").read()
    text = re.sub(r"(?s)```.*?```", " ", text)
    text = re.sub(r"`[^`\n]*`", " ", text)
    text = re.sub(r"(?s)\$\$.*?\$\$", " ", text)
    for para in re.split(r"\n\s*\n", text):
        segs = re.split(r"(?<!\\)\$", para)
        for i in range(1, len(segs) - 1, 2):
            if segs[i + 1][:1].isdigit():
                out.append("%s: ...$%s$%s" % (f, segs[i][-20:], segs[i + 1][:8]))
                break
print("; ".join(out[:5]))
PY
)
if [ -n "$bad" ]; then line FAIL "a formula ends directly before a digit: $bad"; fail=1
else line PASS "no formula ends directly before a digit"; fi

missing=""
for b in README provenance/P0_task_rules_deviations_decisions provenance/P1_symbols_LL provenance/P2_calculations_code_and_checks \
         "manuscript/The Lorentz Super-Loop of the Bi-Octonions - Non-Associative Counterexamples to the No-Go Theorems and a Spin-Zero Spinor Superfluid"; do
    for ext in md tex pdf; do
        [ -s "$b.$ext" ] || missing="$missing $b.$ext"
    done
done
if [ -n "$missing" ]; then line FAIL "missing document versions:$missing"; fail=1
else line PASS "every document exists as .md, .tex and .pdf"; fi

tools=$(grep -n -i -E '\b(claude|anthropic|chatgpt|openai|gemini|copilot|language model|AI[- ]generated|artificial intelligence)\b' \
        manuscript/*.tex manuscript/*.md provenance/*.md provenance/*.tex README.md README.tex docs_src/*.src.md docs_src/manuscript/*.tex 2>/dev/null | head -5)
if [ -n "$tools" ]; then line FAIL "a document names a tool used to prepare it: $tools"; fail=1
else line PASS "no document names a tool used to prepare it"; fi

if [ $fail -eq 0 ]; then echo "ALL PASS"; else echo "SOME CHECK FAILS"; fi
exit $fail
