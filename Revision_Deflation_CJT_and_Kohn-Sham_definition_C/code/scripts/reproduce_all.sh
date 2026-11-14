#!/usr/bin/env bash
# reproduce_all.sh (Revision_Deflation_CJT_and_Kohn-Sham_definition_C)
#
# PURPOSE.  To prove that the data files and the reports of checks of this folder are what the
# programs of this folder produce: everything is executed again from the sources, and every file
# that a program writes into data/ is compared, byte for byte, with the file that is committed in
# the repository.
#
# WHERE TO RUN IT.  In a fresh copy of the repository (a clone), in which nothing has been changed:
#
#     git clone https://github.com/once-ere/spacetimes.git
#     cd spacetimes/Revision_Deflation_CJT_and_Kohn-Sham_definition_C
#     bash code/scripts/reproduce_all.sh
#
# The first line downloads the repository, the second goes to this folder, the third runs this
# script.  It needs a Rust toolchain (cargo) and Python 3 with numpy, scipy, sympy and mpmath.
# On a machine with two processors it takes about two hours and a quarter.
#
# WHAT IT DOES, in this order:
#   1. compiles the program and runs its unit tests (cargo test --release), builds the program;
#   2. runs the 19 commands of the program.  They are run in two chains side by side, because the
#      machine of this work has two processors: the command `families`, which takes longest, in
#      one chain and the eighteen others in the second.  No command reads a file that another
#      command of this folder writes, so that the order does not matter;
#   3. runs the two independent verification scripts, the test of those scripts with deliberate
#      errors, and the check of the four vendored modules;
#   4. compares every file in data/ with the committed file (git) and writes the result to
#      data/reproduction_checks.txt: one line for each step, "[PASS]" or "[FAIL]".
#
# The three reports of the checks of the DOCUMENTS (number_checks.txt, symbol_checks.txt,
# self_contained_checks.txt) are not written by this script; they are written when the documents
# are built (bash code/scripts/build_all_docs.sh), and they are left out of the comparison, as is
# the report of this script itself.
#
# OPTIONS (environment variables):
#   REPRO_LOG=dir   where the outputs of the commands are kept (default: /tmp/reproduce_logs)
#   REPRO_SKIP_LONG=1   leave out the three longest commands (forecast, superfluid, families);
#                       the report then says so and counts them as not executed
set -u
cd "$(dirname "$0")/../.."
FOLDER=Revision_Deflation_CJT_and_Kohn-Sham_definition_C
LOG=${REPRO_LOG:-/tmp/reproduce_logs}
mkdir -p "$LOG"
M=code/deflation-cjt-ks-c-rs/Cargo.toml
B=code/deflation-cjt-ks-c-rs/target/release/deflation_cjt_ks_c
date +%s > "$LOG/start_seconds.txt"
git rev-parse HEAD > "$LOG/commit.txt"
git status --porcelain --untracked-files=no -- . > "$LOG/status_before.txt"
: > "$LOG/steps.txt"

step() {   # step <name> <command...>: run, keep the output, record exit status and seconds
    local name=$1; shift
    local t0; t0=$(date +%s)
    "$@" > "$LOG/$name.log" 2>&1
    local rc=$?
    echo "$name $rc $(( $(date +%s) - t0 ))" >> "$LOG/steps.txt"
    return $rc
}

echo "== 1. unit tests and build (log: $LOG)"
step cargo_test  cargo test  --release --manifest-path $M
step cargo_build cargo build --release --manifest-path $M || { echo "the program does not build; see $LOG/cargo_build.log"; exit 1; }

echo "== 2. the commands of the program, in two chains"
SHORT="selfcoupling kohnsham pairing gravity matrices modes symmetry channels history quench paired cosmology matter background spectrum forms"
LONG="forecast superfluid"
if [ "${REPRO_SKIP_LONG:-0}" = "1" ]; then LONG=""; fi
(
    if [ "${REPRO_SKIP_LONG:-0}" != "1" ]; then step cmd_families $B families; fi
) &
(
    for c in $SHORT $LONG; do step cmd_$c $B $c; done
    echo "== 3. independent verifications (second chain, while the first still runs)"
    step py_verify_deflating python3 code/scripts/verify_deflating_independent.py
    step py_verify_cjt_ks    python3 code/scripts/verify_cjt_ks_independent.py
    step py_mutation_test    python3 code/scripts/mutation_test_independent.py
    step py_vendor_check     python3 code/scripts/vendor_eb.py --check
) &
wait

echo "== 4. comparison with the committed files"
python3 - "$LOG" "$FOLDER" <<'PY'
import os, re, subprocess, sys, time
log, folder = sys.argv[1], sys.argv[2]
start = int(open(os.path.join(log, "start_seconds.txt")).read())
commit = open(os.path.join(log, "commit.txt")).read().strip()
steps = [l.split() for l in open(os.path.join(log, "steps.txt")) if l.strip()]
steps = {n: (int(rc), int(sec)) for n, rc, sec in steps}
def sh(*a):
    return subprocess.run(a, capture_output=True, text=True)
lines, ok_all = [], True
def rec(ok, ident, text):
    global ok_all
    ok_all = ok_all and ok
    lines.append("[%s] %s  %s" % ("PASS" if ok else "FAIL", ident, text))

# RP1: the starting point
dirty = open(os.path.join(log, "status_before.txt")).read().strip()
rec(dirty == "", "RP1", "no tracked file of the folder differed from the commit %s when the script started%s"
    % (commit[:7], "" if dirty == "" else " -- NOT SO: " + dirty.replace("\n", "; ")[:300]))

# RP2: unit tests
t = open(os.path.join(log, "cargo_test.log"), errors="replace").read()
passed = sum(int(x) for x in re.findall(r"test result: ok\. (\d+) passed", t))
failed = sum(int(x) for x in re.findall(r"(\d+) failed", t))
rc, sec = steps.get("cargo_test", (1, 0))
rec(rc == 0 and failed == 0 and passed > 0, "RP2", "cargo test --release: %d unit tests passed, %d failed (%d s)" % (passed, failed, sec))

# RP3: each command of the program
commands = "selfcoupling kohnsham pairing gravity matrices modes symmetry channels history quench paired cosmology matter background spectrum forms forecast superfluid families".split()
executed = 0
for c in commands:
    if "cmd_" + c not in steps:
        rec(False, "RP3-" + c, "command %s: NOT EXECUTED" % c)
        continue
    rc, sec = steps["cmd_" + c]
    out = open(os.path.join(log, "cmd_%s.log" % c), errors="replace").read()
    verdicts = re.findall(r"(\d+) of (\d+) checks passed", out)
    good = rc == 0 and len(verdicts) > 0 and all(a == b for a, b in verdicts)
    executed += 1
    rec(good, "RP3-" + c, "command %s: exit status %d, %s (%d s)"
        % (c, rc, "; ".join("%s of %s checks passed" % v for v in verdicts) if verdicts else "no verdict in its output", sec))

# RP4: the independent scripts
for name, what in (("py_verify_deflating", "verify_deflating_independent.py"), ("py_verify_cjt_ks", "verify_cjt_ks_independent.py"),
                   ("py_mutation_test", "mutation_test_independent.py"), ("py_vendor_check", "vendor_eb.py --check")):
    if name not in steps:
        rec(False, "RP4-" + name[3:], "%s: NOT EXECUTED" % what)
        continue
    rc, sec = steps[name]
    out = open(os.path.join(log, name + ".log"), errors="replace").read()
    v = re.findall(r"(\d+) of (\d+) (checks passed|deliberate errors detected)", out)
    good = rc == 0 and all(a == b for a, b, _ in v)
    rec(good, "RP4-" + name[3:], "%s: exit status %d%s (%d s)" % (what, rc, "".join(", %s of %s %s" % x for x in v[-1:]), sec))

# RP5: every file of data/ against the committed file
not_by_this_script = {"number_checks.txt", "symbol_checks.txt", "self_contained_checks.txt", "reproduction_checks.txt"}
tracked = [p for p in sh("git", "ls-files", "data").stdout.split("\n") if p]
identical, different, not_rewritten = [], [], []
for p in tracked:
    if os.path.basename(p) in not_by_this_script:
        continue
    if sh("git", "diff", "--quiet", "--", p).returncode == 0:
        identical.append(p)
    else:
        different.append(p)
    if os.path.getmtime(p) < start:
        not_rewritten.append(p)
untracked = [p for p in sh("git", "ls-files", "--others", "--exclude-standard", "data").stdout.split("\n") if p and os.path.basename(p) not in not_by_this_script]
n = len(identical) + len(different)
rec(len(not_rewritten) == 0, "RP5a", "every one of the %d files of data/ that the programs write was written again by this run%s"
    % (n, "" if not not_rewritten else " -- NOT WRITTEN AGAIN: " + ", ".join(not_rewritten)[:1500]))
rec(len(different) == 0, "RP5b", "%d of these %d files are identical, byte for byte, to the committed files%s"
    % (len(identical), n, "" if not different else " -- DIFFERENT: " + ", ".join(different)[:1500]))
rec(len(untracked) == 0, "RP5c", "the run wrote no file into data/ that is not in the repository%s"
    % ("" if not untracked else " -- NEW FILES: " + ", ".join(untracked)[:1500]))

total = int(time.time()) - start
head = ["PROJECT spacetimes, %s: everything executed again from the sources and compared with the repository" % folder,
        "generated by: code/scripts/reproduce_all.sh",
        "commit %s; %d of the 19 commands of the program executed; total time %d s (%d min) on a machine with %s processors"
        % (commit, executed, total, total // 60, os.cpu_count()),
        "%d of %d checks passed" % (sum(1 for l in lines if l.startswith("[PASS]")), len(lines)), ""]
open("data/reproduction_checks.txt", "w", encoding="utf-8").write("\n".join(head + lines) + "\n")
print("\n".join(head + lines))
sys.exit(0 if ok_all else 1)
PY
