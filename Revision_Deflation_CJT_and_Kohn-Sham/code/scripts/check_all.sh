#!/usr/bin/env bash
# check_all.sh (Revision_Deflation_CJT_and_Kohn-Sham) -- one verdict for the whole folder.
#
# Runs the checks of the documents again and reads the verdict of every check report in
# data/.  Prints one line for each, and ends with the status 0 only if every one of them
# passes.  It exists because a report that fails is easy to overlook when only the last
# lines of a long output are read: nothing is to be committed unless this script ends
# with "ALL PASS".
#
# It does NOT run the program again (the commands of `deflation_cjt_ks` take about two and
# a half hours) and it does not run the two independent verifications or the mutation test
# again (about twelve minutes); it reads the reports that they wrote.  Run those first when
# a program or a data file was changed.
#
# Usage (from the folder Revision_Deflation_CJT_and_Kohn-Sham):
#     bash code/scripts/check_all.sh
set -u
cd "$(dirname "$0")/../.."
fail=0
line() { printf '%-6s %s\n' "$1" "$2"; }

# 1. checks of the documents, executed now
for s in check_numbers check_symbols check_self_contained; do
    if python3 code/scripts/$s.py > /dev/null 2>&1; then line PASS "code/scripts/$s.py"; else line FAIL "code/scripts/$s.py"; fail=1; fi
done
if python3 code/scripts/vendor_eb.py --check > /dev/null 2>&1; then line PASS "code/scripts/vendor_eb.py --check"; else line FAIL "code/scripts/vendor_eb.py --check"; fail=1; fi

# 2. the reports in data/: "N of N checks passed", "N of N deliberate errors detected", or a line [PASS]/[FAIL]
for f in data/*_checks.txt data/*_checks_independent.txt data/mutation_test_independent.txt; do
    [ -f "$f" ] || continue
    if grep -q '^\[FAIL\]' "$f"; then line FAIL "$f (contains a failed check)"; fail=1; continue; fi
    c=$(grep -m1 -oE '[0-9]+ of [0-9]+ (checks passed|deliberate errors detected)' "$f" || true)
    if [ -n "$c" ]; then
        a=${c%% of *}; b=${c#* of }; b=${b%% *}
        if [ "$a" = "$b" ]; then line PASS "$f ($c)"; else line FAIL "$f ($c)"; fail=1; fi
    elif grep -q '^\[PASS\]' "$f"; then line PASS "$f"
    else line FAIL "$f (no verdict found)"; fail=1
    fi
done

# 3. no document may contain an unresolved placeholder
if grep -l '{{[A-Z]*:' provenance/*.md manuscript/*.md manuscript/*.tex ./*.md 2>/dev/null | grep -q .; then
    line FAIL "unresolved placeholder in: $(grep -l '{{[A-Z]*:' provenance/*.md manuscript/*.md manuscript/*.tex ./*.md 2>/dev/null | tr '\n' ' ')"; fail=1
else
    line PASS "no unresolved placeholder in the built documents"
fi

if [ $fail -eq 0 ]; then echo "ALL PASS"; else echo "SOME CHECK FAILS"; fi
exit $fail
