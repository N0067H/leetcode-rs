"""Discover independent LeetCode Rust crates for rust-analyzer."""

import json
from pathlib import Path
import subprocess
import sys


root = Path(__file__).resolve().parent.parent
sysroot = subprocess.check_output(["rustc", "--print", "sysroot"], text=True).strip()
project = {
    "sysroot": sysroot,
    "sysroot_src": str(Path(sysroot) / "lib/rustlib/src/rust/library"),
    "crates": [
        {
            "display_name": "leetcode_" + path.stem.replace("-", "_").replace(".", "_"),
            "root_module": str(path),
            "edition": "2021",
            "deps": [],
        }
        for path in sorted(root.glob("*.rs"))
    ],
}

if "--write" in sys.argv:
    # Keep a snapshot for editors that discover rust-project.json directly.
    (root / "rust-project.json").write_text(json.dumps(project, indent=2) + "\n")
else:
    print(json.dumps({
        "kind": "finished",
        "buildfile": str(root / "rust-project.json"),
        "project": project,
    }))
