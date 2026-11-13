#!/usr/bin/env bash
# build_all_docs.sh (Revision_Deflation_CJT_and_Kohn-Sham_definition_C) -- build every document and check it.
#
# The documents print reports of checks, and three of those checks read the documents.  This
# script runs the steps in the order in which each one finds current input, twice where that
# is needed, and ends with the verdict of check_all.sh.  It does not run the program or the
# independent verifications (see the instructions in the README); it needs their data files.
#
# Usage (from the folder Revision_Deflation_CJT_and_Kohn-Sham_definition_C):
#     bash code/scripts/build_all_docs.sh
set -u
cd "$(dirname "$0")/../.."
step() { echo "== $*"; }

step "registry of symbols, generated tables"
python3 code/scripts/write_symbol_registry.py | tail -1
# the first pass does not require the symbol report to pass: that report is made from the documents
python3 code/scripts/write_doc_tables.py --first-pass | tail -1 || exit 1

step "assemble, typeset the manuscripts, run the checks that read the documents"
python3 code/scripts/assemble_docs.py > /dev/null || exit 1
python3 code/scripts/build_manuscripts.py 2>&1 | grep -E 'PDF|LaTeX log|problem|unresolved' || true
for s in check_numbers check_symbols check_self_contained; do python3 code/scripts/$s.py > /dev/null; done

step "second pass: the strict run, with the reports as they now are"
python3 code/scripts/write_doc_tables.py | tail -1 || exit 1
python3 code/scripts/assemble_docs.py > /dev/null || exit 1
python3 code/scripts/build_manuscripts.py 2>&1 | grep -E 'PDF|LaTeX log|problem|unresolved' || true
for s in check_numbers check_symbols check_self_contained; do python3 code/scripts/$s.py > /dev/null; done
python3 code/scripts/assemble_docs.py > /dev/null || exit 1

step "typeset the other documents"
out=$(bash code/scripts/build_docs.sh 2>&1)
echo "$out" | grep -v 'note:' | tail -4
layout=$(echo "$out" | grep -c 'LAYOUT' || true)

step "verdict"
bash code/scripts/check_all.sh | grep -v '^PASS' 
if [ "${layout:-0}" != "0" ]; then echo "FAIL   a document other than the manuscripts has a layout problem (see the output of build_docs.sh)"; exit 1; fi
