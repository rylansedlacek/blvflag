import subprocess
import os
from pathlib import Path

SCRIPT_DIR = Path("/Users/rylan/blvflag/demo/context_scripts")
CARGO_PROJECT_DIR = "/Users/rylan/blvflag/tool/"


def extract_number(path: Path):
    import re
    nums = re.findall(r"\d+", path.stem)
    return int(nums[0]) if nums else 0

def run_blvdiff(script_path, flags=None):
    cmd = [
        "cargo", "run",
        "--manifest-path", os.path.join(CARGO_PROJECT_DIR, "Cargo.toml"),
        "--",
        str(script_path)
    ]
    if flags:
        cmd.extend(flags)

    print(f"\n[running] {' '.join(cmd)}")
    subprocess.run(cmd)


def run_all_scripts(folder):
    print(f"\nrunning all scripts in: {folder}\n")

    files = sorted(
        [f for f in Path(folder).iterdir() if f.suffix == ".py"],
        key=extract_number
    )

    for f in files:
        print(f"\n>>> {f.name}")
        run_blvdiff(f)  


# ---- RUN ----
if __name__ == "__main__":
    run_all_scripts(SCRIPT_DIR)