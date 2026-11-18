#!/usr/bin/env bash
# run_convergence.sh -- PROJECT spacetimes, Revision_SuperLoop, folder test2/cvode_vs_rk4.
# The four time-dependent examples of the solver t2_solve (packet_free, cyclotron, andreev, quench) are solved again
# with CVODE at two further relative tolerances, rtol = 1e-10 and rtol = 1e-13 (atol = rtol/100), besides the default
# rtol = 1e-12 of the files test2/examples/*.t2in.  If the results at the three tolerances agree, the CVODE results are
# converged, and their difference from the results of the fixed-step methods of the first version of Test 2 (RK4 and the
# implicit midpoint rule) is the error of those methods.
# The input files data/conv/NAME_rtolR.t2in are the example files with the two lines 'rtol = R' and 'atol = R/100'
# added; the outputs are data/conv/NAME_rtolR_{summary.txt,series.csv,fields.csv,...}.
#
# Usage (from this folder, after 'cargo build --release' in test2/rust):
#     bash run_convergence.sh [path of the t2_solve binary]
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
BIN="${1:-$HERE/../rust/target/release/t2_solve}"
for name in packet_free cyclotron andreev quench; do
  for r in 1e-10 1e-13; do
    a=$(python3 -c "print('%g' % ($r / 100))")
    inp="$HERE/data/conv/${name}_rtol${r}.t2in"
    { cat "$HERE/../examples/${name}.t2in"; printf 'rtol = %s\natol = %s\n' "$r" "$a"; } > "$inp"
    echo "== $name, rtol = $r, atol = $a"
    "$BIN" "$inp" --out "$HERE/data/conv/${name}_rtol${r}" | tail -1
  done
done
