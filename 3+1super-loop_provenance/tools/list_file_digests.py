#!/usr/bin/env python3
"""list_file_digests.py      (PROJECT spacetimes, provenance of the notebook 3+1super-loop.nb)

PURPOSE
  Keeps the two lists of SHA-256 digests of the provenance P_3+1super-loop.md true:
    * section 16 ("Files"): the table of the files of the notebook must name EVERY file of the item
      (the notebook 3+1super-loop.nb in the top folder of the repository, and every file below the
      folder 3+1super-loop_provenance, except the three versions .md, .tex and .pdf of the provenance
      itself, which cannot list their own digests), and no file that does not exist.  The script
      stops with a list of the differences if this is not so.  Then it writes the digests of these
      files, in the order of the table, into the code block that follows the table.
    * section 17 ("Sources consulted"): the code block at its end lists original programs and data
      files of Test 2; the script keeps the paths of this block and writes their present digests.
  A SHA-256 digest is a number of 64 hexadecimal digits computed from the content of a file; two files
  with the same digest have, for every practical purpose, the same content.

USAGE (in the top folder of the repository)
  python3 3+1super-loop_provenance/tools/list_file_digests.py            # check, and rewrite the two blocks
  python3 3+1super-loop_provenance/tools/list_file_digests.py --check    # check only; nothing is written
"""
import hashlib
import os
import re
import sys

DOC = "3+1super-loop_provenance/P_3+1super-loop.md"
FOLDER = "3+1super-loop_provenance"
NOTEBOOK = "3+1super-loop.nb"
OWN = {FOLDER + "/P_3+1super-loop." + ext for ext in ("md", "tex", "pdf")}


def sha256(path):
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(1 << 20), b""):
            h.update(chunk)
    return h.hexdigest()


def main():
    check_only = "--check" in sys.argv[1:]
    if not os.path.exists(DOC):
        sys.exit("run this script in the top folder of the repository")
    text = open(DOC, encoding="utf-8").read()
    a = text.index("\n# 16. Files")
    b = text.index("\n# 17. Sources consulted")
    sec16, sec17 = text[a:b], text[b:]

    # ---- section 16: the table against the files on the disk
    listed = re.findall(r"(?m)^\| `([^`]+)` \|", sec16)
    duplicates = sorted({p for p in listed if listed.count(p) > 1})
    on_disk = {NOTEBOOK}
    for root, _, files in os.walk(FOLDER):
        for name in files:
            path = os.path.join(root, name).replace(os.sep, "/")
            if path not in OWN and "/__pycache__/" not in path and not name.startswith("."):
                on_disk.add(path)
    missing_in_table = sorted(on_disk - set(listed))
    missing_on_disk = sorted(set(listed) - on_disk)
    if duplicates or missing_in_table or missing_on_disk:
        for title, items in (("listed twice in the table", duplicates), ("on the disk, not in the table", missing_in_table),
                             ("in the table, not on the disk", missing_on_disk)):
            for item in items:
                print(f"{title}: {item}")
        sys.exit("section 16 and the files differ; nothing was written")
    block16 = "\n".join(f"{sha256(p)}  {p}" for p in listed)

    # ---- section 17: the paths of its last code block, with their present digests
    blocks17 = list(re.finditer(r"```\n(.*?)\n```", sec17, flags=re.S))
    last = blocks17[-1]
    paths17 = [line.split(None, 1)[1].strip() for line in last.group(1).split("\n") if line.strip()]
    absent = [p for p in paths17 if not os.path.exists(p)]
    if absent:
        sys.exit("section 17 lists files that do not exist: " + ", ".join(absent))
    block17 = "\n".join(f"{sha256(p)}  {p}" for p in paths17)

    first16 = re.search(r"```\n(.*?)\n```", sec16, flags=re.S)
    changed16 = first16.group(1) != block16
    changed17 = last.group(1) != block17
    print(f"section 16: {len(listed)} files in the table and on the disk; digests {'differ from' if changed16 else 'equal'} the list")
    print(f"section 17: {len(paths17)} files; digests {'differ from' if changed17 else 'equal'} the list")
    if check_only:
        sys.exit(1 if (changed16 or changed17) else 0)
    new16 = sec16[:first16.start(1)] + block16 + sec16[first16.end(1):]
    new17 = sec17[:last.start(1)] + block17 + sec17[last.end(1):]
    open(DOC, "w", encoding="utf-8").write(text[:a] + new16 + new17)
    print("written: " + DOC)


if __name__ == "__main__":
    main()
