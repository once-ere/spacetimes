---
title: "The stop of all running stages on 20 November 2026"
subtitle: "Provenance document `P3` of PROJECT spacetimes, Revision_SuperLoop"
author: "Prepared for Patrick Lee Nash, Ph.D."
date: "November 20, 2026"
---

# 1. What this document is

On 20 November 2026 the author gave a new task. Its first instruction was to stop all running stages and to create a
provenance of the stop. This document is that provenance. It records

* the two instructions of that day that concern the stop, verbatim (Section 3);
* every stage of the work plan, what it was meant to do, and its state at the moment of the stop (Sections 4 and 5);
* what was done to stop the stages, and what was found when the state was checked (Sections 6 and 8);
* the one stage whose finished work was delivered after the stop with the author's permission, stage E, and how it was
  verified before delivery (Section 7);
* what the stop does not change (Section 9), and the errors made during the stop (Section 10).

Every term used here is defined in Section 2. The document contains no formula.

# 2. The terms used in this document

| Term | Meaning |
|------------------|-------------------------------------------------------------------------------------------|
| PROJECT spacetimes | the author's project. Its repository is `https://github.com/once-ere/spacetimes.git`; its working folder on the author's computer is `/Users/nsh/Developer/github/spacetimes`, called **the Mac folder** below. |
| Revision_SuperLoop | the folder of the repository in which the stages of Section 4 were carried out. |
| workspace | the cloud computer on which the programs and documents of the stages were written and run. It holds a copy (a clone) of the repository. |
| commit | a recorded state of the repository, named here by the first seven hexadecimal digits of its identifier. **main** is the principal line of commits. **origin/main** is the state of main on the server of the repository, as last seen by the Mac folder. To **push** is to send the commits of the Mac folder to the server; the author does this himself. |
| bundle | a file that carries commits from the workspace to the Mac folder. There they are added with a fast-forward merge: commits are added, and nothing that is already there is changed or removed. |
| tracked file | a file whose content is recorded in the commits. |
| full check | the script `code/scripts/check_all.sh` of Revision_SuperLoop. It rebuilds every program, data file, figure and document of the folder from its sources and runs every check: 69 steps, about 21 minutes. Its last line is `ALL PASS` when every step succeeds. |
| program check | one of the numbered checks that a program of the folder prints in its report (for example `CJ-12`), each with the word PASS or FAIL. |
| stage | one part of the work plan in force before the stop; Section 4 lists them with their content. |
| task list | the list of tasks of the working session, which the author sees beside the conversation. |
| watcher | a small Wolfram Language program that runs inside the author's Wolfram application on the Mac. It executes requests that the workspace places as files in the folder `.git-transfer/mma/` of the Mac folder, and writes the answers there. It is a tool for running Wolfram code, not a stage. |
| SFBS | the superfluid boson singlet state: the state of spin zero formed by pairs of spinor charges. |
| CJT | the method of J. M. Cornwall, R. Jackiw and E. Tomboulis, "Effective action for composite operators", Phys. Rev. D 10, 2428 (1974). |
| RK4, CVODE, IDA | RK4 is the classical Runge-Kutta method of fourth order with a fixed step. CVODE and IDA are integrators with error control of the SUNDIALS library, used here in the pure Rust version of the author's repository and, for IDA, inside the Wolfram function `NDSolve`. |
| Test 2 | the author's test problem of Revision_SuperLoop: the gauge-covariant spinor field on the 3+1 manifold and the SFBS, in eleven items numbered [1] to [11]. |

# 3. The instructions of 20 November 2026

At 17:21 (Pacific time) the author gave a new task. Its first instruction reads, verbatim:

> "First stop all running stages; create a provenance."

At 17:31 the author added, verbatim:

> "it is OK to push stage E 's. completed work, if this does NOT interfere with your current task"

The rest of the new task is carried out in a new folder of the repository and is not part of this record.

# 4. The stages of the work plan, and their state at the stop

The work plan in force was that of 18 November 2026: stages A, B and C, and the six stages D to I that the author added
on 18 November at 20:17. Table 1 lists each stage, what it was meant to do, its state at the moment of the stop, and
what the stop did to it.

**Table 1.** The stages of the work plan.

| Stage | What it was meant to do | State at the stop | Action |
|-----|------------------------------|---------------------------------------|----------------|
| A | the notebook `3+1super-loop.nb` (at the top of the repository) with every time integration done by IDA instead of RK4 | complete; commit `8779db7` | none needed |
| B | the port of the programs of Test 2, items [2] to [11], into the same notebook | item [2] complete (commit `5dc688e`); items [3] to [11] not begun; a notebook chapter with the checks named GS-17 to GS-20 had been written and set aside outside the repository; it was never committed | stopped |
| C | the older programs that use integrators with a fixed step, redone with CVODE in new folders | not begun | stopped |
| D | the creation of the SFBS from the spinor field by the method CJT | complete; commit `32a4961`; pushed by the author (origin/main = `32a4961` on the Mac at 17:30 on 20 November) | none needed |
| E | the effective potential of the SFBS in the method CJT (the analogue of the "Mexican hat" potential of the complex Higgs doublet) | every part written on 19 November: the program part (program checks CJ-12 to CJ-15; the report of 19 November, 09:32, says "15 of 15 checks passed"), the new section 10 of the document of the stages D to I, the figure, the data files, the symbol registry and the records of the folder. **Not verified**: the full check had been started three times; the first two runs failed through two errors of procedure of the person doing the work, and the third run (started 19 November, 09:36:54) stopped with the workspace after 17 of its 69 steps; its log ends at 09:41. **Not committed.** | verified and delivered after the author's permission of 17:31 (Section 7) |
| F | the relative minima of this potential and the spontaneous breaking of the symmetry | a draft of its program part (four program checks, named CJ-16 to CJ-19) and a scratch calculation exist in the workspace outside the repository; the draft was never compiled into the program | stopped; the drafts are kept unchanged and are not continued |
| G | the expansion of the SFBS about the minimum (called SFBSv) | not begun | stopped |
| H | the kinetic Lagrangian of the SFBS | not begun | stopped |
| I | the masses of the Goldstone bosons from SFBSv and the kinetic Lagrangian, the diagonalization of the mass terms, the surviving symmetry group, and its relation to U(1)_em and the massless photon | not begun | stopped |

# 5. Stages that the author had paused earlier

These were not running at the stop; they remain as the author left them.

**Table 2.** Stages paused or stopped earlier by the author.

| When | What | State |
|-------------------|--------------------------------------------------------------|------------------------------|
| 12 November, about 15:20 | the main project (the 4+4 and 7+1 spacetimes): the independent verification pass, the correction of the statement about fermions bound to a vortex line, a planned provenance, the final check | paused by the author; not running since |
| 15 November, 12:36 | all stages of the main project then running | paused by the author; not running since |
| 15 November, 13:25 | the folders `Revision_2` and `Revision_2b` | stopped by the author's choice; they stay as they are |

# 6. What was done to stop the stages

1. **Processes in the workspace.** At 17:29 on 20 November no process of any stage ran in the workspace: it had been
   restarted, and the list of its processes contained none of the programs of the stages. There was nothing to end.
2. **Processes on the Mac.** No program of a stage runs on the Mac; the stages ran in the workspace. The watcher is not a
   stage and was not touched. It is needed for the new task, for which the author asked that the Wolfram application on
   his computer be used.
3. **The task list.** Every open task of the stages was marked "STOPPED 2026-11-20".
4. **The drafts.** The drafts of stage F and the set-aside notebook chapter of stage B were left where they were,
   unchanged, outside the repository. Nothing was deleted.
5. **No further work** is done on stages B, C, F, G, H and I, unless the author asks for it. The only work done after the
   stop on a stage of Table 1 is the verification and delivery of the finished work of stage E (Section 7), which the
   author permitted at 17:31 under the condition that it not interfere with the new task: it ran in the background while
   the new task began.

# 7. Stage E: verification and delivery

Stage E is the only stage of Table 1 on which work was done after the stop, and that work was limited to verifying and
delivering what had been finished before the stop. Nothing of its content was changed.

1. **The permission.** At 17:31 the author permitted the delivery of the finished work of stage E, if it did not interfere
   with the new task. It did not: the verification ran by itself in the background of the workspace while the new task
   began.
2. **The verification.** The full check of Revision_SuperLoop was run once more from the start, on 20 November from 17:33:13
   to 17:52:42. Every one of its 69 steps succeeded; its last line is `ALL PASS`. The report of the program of stage E says
   "15 of 15 checks passed" (the program checks CJ-1 to CJ-15; CJ-12 to CJ-15 belong to stage E).
3. **The commit.** The files of stage E, as the full check rebuilt them from their sources, were committed unchanged as
   commit `0cefb61` ("Test 2, stage E: the effective potential of the superfluid boson singlet state by the method of
   Cornwall, Jackiw and Tomboulis"): 110 files.
4. **What stage E contains.** A new Section 10 of the document of the stages D to I of Test 2 (its file name: "The superfluid
   boson singlet state by the method of Cornwall, Jackiw and Tomboulis", .md, .tex, .pdf), with four theorems, two tables
   and one figure; the four program checks CJ-12 to CJ-15 of the program `t2_cjt`; two data files and one figure; six
   entries of the symbol registry of the folder; and the records of the folder (its task record, symbol file, record of the
   calculations and README) brought up to date. The other files of the commit are those that the full check rebuilds every
   time (documents and figures whose content did not change).
5. **The transfer.** Commit `0cefb61` and the commit of this document are carried to the Mac folder together; the author
   pushes them.

# 8. The state of the repository at the stop

The repository on the Mac was examined at 17:30 on 20 November.

* main = origin/main = `32a4961`: the server and the Mac folder held the same commits.
* No tracked file was changed or missing.
* `git fsck`, the integrity check of git, found no missing and no damaged object.
* It reported twelve leftover temporary files of git (34 MiB in total) in `.git/objects/pack/`, and a leftover lock file
  `.git/objects/maintenance.lock`. They were left by earlier git operations that could not finish removing their own
  temporary files, because the programs of the workspace are not allowed to delete files in the Mac folder. They do not
  affect the commits, the merges or the pushes. They were not removed.

These checks show that the content recorded in the repository is intact. They do not judge the quality of that content.

After the stop, commit `0cefb61` (stage E) and the commit of this document are carried to the Mac folder in one bundle and
added there by a fast-forward merge, which changes nothing that is already there; the author then pushes them to the server.

# 9. What the stop does not change

* Every commit up to `32a4961` stays as it is; nothing was rewritten or removed.
* The folders of earlier revisions are not changed.
* The new task of 20 November is carried out in a new folder of the repository, `Revision_HS_CJT_and_Kohn-Sham`. It
  changes nothing in Revision_SuperLoop.

# 10. Errors made during the stop

| Error | What went wrong | How it was found | Correction |
|----------|---------------------------------------------|----------------------|-------------------------------------|
| E1 | The first draft of Table 2 of this document named another document as the record of an earlier pause. The author's rule forbids sending the reader to another document. | reading the draft against the author's rules before the document was built | the reference was removed; Table 2 states the facts itself |
