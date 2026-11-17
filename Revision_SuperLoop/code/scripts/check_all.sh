#!/usr/bin/env bash
# check_all.sh (Revision_SuperLoop) -- run everything of the folder Revision_SuperLoop in order, and give one
# verdict.
#
# Steps:
#   1. the program superloop (all five commands; about one second), the Groebner-basis
#      classification (a few seconds) and the seven figures; then the program of the folder
#      explanations (explanations/code/explain-rs: the examples of the explanation of the
#      spin-zero superfluid) and its three figures, which read two data files of superloop; and
#      the program explanations/code/r13-rs (the 3+1 manifold and the renormalizability of the
#      Yukawa form) and its four figures; then the programs of the folder test2 (the answers to
#      Test 2): t2_fields (the gauge-covariant field equations of the spinor field, item [1]),
#      t2_analytic (the analytic solutions of item [2]) and the figures of items [1] and [2]
#      (test2/python/make_t2_figures.py);
#   2. two passes of: the registry of the symbols, the fragments of the documents, the checks of
#      the symbols and of self-containedness, the assembly of the Markdown documents, the build of
#      the manuscript and of the .tex/.pdf versions of the documents.  The second pass is needed
#      because the reports of the checks, which the documents print, count the built documents
#      that the first pass creates; after the second pass every printed report is current;
#   3. the verdicts: every report in data/, explanations/data/ and test2/data/ must pass, no built document may
#      contain an unresolved
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
explain() { (cd explanations && cargo run --release --quiet --manifest-path code/explain-rs/Cargo.toml); }
run "explain_superfluid (explanations)" explain
run "make_explain_figures.py (explanations)" python3 explanations/code/make_explain_figures.py
r13() { (cd explanations && cargo run --release --quiet --manifest-path code/r13-rs/Cargo.toml); }
run "r13_yukawa (explanations)" r13
run "make_r13_figures.py (explanations)" python3 explanations/code/make_r13_figures.py
t2fields() { (cd test2/rust && cargo build --release --quiet && ./target/release/t2_fields); }
run "t2_fields (test2)" t2fields
t2analytic() { (cd test2/rust && cargo build --release --quiet && ./target/release/t2_analytic); }
run "t2_analytic (test2)" t2analytic
run "make_t2_figures.py (test2)" python3 test2/python/make_t2_figures.py
# Test 2, item [3]: the Python solver (self-test, the seven examples, the table of the keys) and the checks of the examples
run "t2_solve.py self-test (test2, item 3)" python3 test2/python/t2_solve.py --selftest test2/data/t2_py_selftest_checks.txt
t2py() { for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
           python3 test2/python/t2_solve.py test2/examples/$e.t2in --out test2/data/t2_py_$e || return 1; done
         python3 test2/python/t2_solve.py --keys-md test2/data/t2_py_fragments.md; }
run "t2_solve.py, the seven examples (test2, item 3)" t2py
run "t2_examples_check.py, Python solutions (test2, item 3)" python3 test2/python/t2_examples_check.py test2/data/t2_py
# Test 2, item [4]: the Python plotter draws Figures 7 to 13 from the seven solutions
t2pyplot() { for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
               python3 test2/python/t2_plot.py test2/data/t2_py_$e --out test2/figures/fig_t2_py_$e || return 1; done; }
run "t2_plot.py, Figures 7 to 13 (test2, item 4)" t2pyplot
# Test 2, item [5]: the Rust solver (self-test, the seven examples) and the checks of its solutions with the comparison EX-8
t2rs() { (cd test2/rust && cargo build --release --quiet) || return 1
         ./test2/rust/target/release/t2_solve --selftest test2/data/t2_rs_selftest_checks.txt test2/examples || return 1
         for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
           ./test2/rust/target/release/t2_solve test2/examples/$e.t2in --out test2/data/t2_rs_$e || return 1; done; }
run "t2_solve (Rust): self-test and the seven examples (test2, item 5)" t2rs
run "t2_examples_check.py, Rust solutions and EX-8 (test2, item 5)" python3 test2/python/t2_examples_check.py test2/data/t2_rs test2/data/t2_py
# Test 2, item [6]: the Rust plotter draws Figures 14 to 20 (SVG), converted to PDF and PNG with rsvg-convert
t2rsplot() { for e in packet_free cyclotron andreev kink_spectrum ground_state kink_ground_state quench; do
               ./test2/rust/target/release/t2_plot test2/data/t2_rs_$e --out test2/figures/fig_t2_rs_$e || return 1
               rsvg-convert -f pdf -o test2/figures/fig_t2_rs_$e.pdf test2/figures/fig_t2_rs_$e.svg || return 1
               rsvg-convert -f png -z 1.6 -o test2/figures/fig_t2_rs_$e.png test2/figures/fig_t2_rs_$e.svg || return 1; done; }
run "t2_plot (Rust), Figures 14 to 20 (test2, item 6)" t2rsplot
# Test 2, items [7] to [11]: the checks SB-* of the superfluid boson singlet state (t2_sfbs) and their figures
t2sfbs() { (cd test2/rust && cargo build --release --quiet && ./target/release/t2_sfbs); }
run "t2_sfbs (test2, items 7 to 11)" t2sfbs
run "make_t2_sfbs_figures.py (test2, items 7 to 11)" python3 test2/python/make_t2_sfbs_figures.py

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
for f in data/sl_*_checks.txt explanations/data/ex_*_checks.txt test2/data/t2_*_checks.txt; do
    if grep -q '^\[FAIL\]' "$f"; then line FAIL "$f (contains a failed check)"; fail=1; continue; fi
    c=$(grep -m1 -oE '[0-9]+ of [0-9]+ checks passed' "$f" || true)
    if [ -n "$c" ]; then
        a=${c%% of *}; b=${c#* of }; b=${b%% *}
        if [ "$a" = "$b" ]; then line PASS "$f ($c)"; else line FAIL "$f ($c)"; fail=1; fi
    else line FAIL "$f (no verdict found)"; fail=1
    fi
done

if grep -l '{{[A-Z]*:' provenance/*.md provenance/*.tex manuscript/*.md manuscript/*.tex explanations/*.md explanations/*.tex test2/*.md test2/*.tex ./README.md ./README.tex 2>/dev/null | grep -q .; then
    line FAIL "unresolved placeholder in: $(grep -l '{{[A-Z]*:' provenance/*.md provenance/*.tex manuscript/*.md manuscript/*.tex explanations/*.md explanations/*.tex test2/*.md test2/*.tex ./README.md ./README.tex 2>/dev/null | tr '\n' ' ')"; fail=1
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
for f in sorted(glob.glob("*.md") + glob.glob("provenance/*.md") + glob.glob("manuscript/*.md") + glob.glob("explanations/*.md") + glob.glob("test2/*.md")):
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
         "manuscript/The Lorentz Super-Loop of the Bi-Octonions - Non-Associative Counterexamples to the No-Go Theorems and a Spin-Zero Spinor Superfluid" \
         "explanations/The spin-zero superfluid of the spinor charges - an explanation" \
         "explanations/The 3+1 manifold of the super-loop and the renormalizability of the Yukawa form" \
         "test2/Test 2 - the gauge-covariant spinor field on the 3+1 manifold and the superfluid boson singlet state"; do
    for ext in md tex pdf; do
        [ -s "$b.$ext" ] || missing="$missing $b.$ext"
    done
done
if [ -n "$missing" ]; then line FAIL "missing document versions:$missing"; fail=1
else line PASS "every document exists as .md, .tex and .pdf"; fi

tools=$(grep -n -i -E '\b(claude|anthropic|chatgpt|openai|gemini|copilot|language model|AI[- ]generated|artificial intelligence)\b' \
        manuscript/*.tex manuscript/*.md provenance/*.md provenance/*.tex explanations/*.md explanations/*.tex README.md README.tex docs_src/*.src.md docs_src/manuscript/*.tex \
        test2/*.md test2/*.tex test2/rust/*/src/*.rs test2/rust/Cargo.toml test2/rust/*/Cargo.toml $(ls test2/python/*.py 2>/dev/null) 2>/dev/null | head -5)
if [ -n "$tools" ]; then line FAIL "a document names a tool used to prepare it: $tools"; fail=1
else line PASS "no document names a tool used to prepare it"; fi

if [ $fail -eq 0 ]; then echo "ALL PASS"; else echo "SOME CHECK FAILS"; fi
exit $fail
