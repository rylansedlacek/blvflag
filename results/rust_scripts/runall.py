import subprocess
import os
from pathlib import Path
import json
import time

SCRIPT_DIR = Path("/Users/rylan/blvflag/results/test_scripts")
HISTORY_DIR = Path("/Users/rylan/blvflag/results/sim_history")
HISTORY_DIR.mkdir(exist_ok=True)
CARGO_PROJECT_DIR = "/Users/rylan/blvflag/tool/"  
A11Y_TASKS_PATH = Path("/Users/rylan/blvflag/results/a11y_tasks.json")

NUM_RUNS = 5  # Number of simulation iterations per script

def run_blvdiff(script_path, flags=None):
    cmd = [
        "cargo", "run",
        "--manifest-path", os.path.join(CARGO_PROJECT_DIR, "Cargo.toml"),
        "--",
        str(script_path)
    ]
    if flags:
        cmd.extend(flags)
    print(f"\n[RUNNING] {' '.join(cmd)}")
    subprocess.run(cmd)

def load_a11y_tasks():
    if not A11Y_TASKS_PATH.exists():
        return {}
    with open(A11Y_TASKS_PATH, 'r') as f:
        return json.load(f)

def map_task_to_line(lines, task_sequence):
    if not task_sequence:
        return 0  # fallback to first line
    line_idx = 0
    for action in task_sequence:
        if action in ["tab", "arrow_down"]:
            line_idx += 1
        elif action == "arrow_up":
            line_idx = max(0, line_idx - 1)
        elif action == "enter":
            break

    line_idx = min(line_idx, len(lines) - 1)
    # skip blank lines
    while line_idx < len(lines) and not lines[line_idx].strip():
        line_idx += 1
    if line_idx >= len(lines):
        line_idx = len(lines) - 1
    return line_idx

def modify_script(filepath, task_sequence, iteration):
    with open(filepath, 'r') as f:
        lines = f.readlines()

    line_idx = map_task_to_line(lines, task_sequence)

    # apply a simulated fix
    lines[line_idx] = f"# SIMULATED FIX {iteration} based on task sequence\n"

    hist_path = HISTORY_DIR / f"{filepath.stem}_v{iteration}.py"
    with open(hist_path, 'w') as f:
        f.writelines(lines)

    return hist_path

def simulate_blv_behavior(script_path, task_sequence, iteration):
    print(f"\n[SIMULATION] Iteration {iteration} on {script_path.name}")

    # 1 run the original script
    run_blvdiff(script_path)

    # 2modify the script based on simulated BLV behavior
    fixed_path = modify_script(script_path, task_sequence, iteration)

    # 3 re-run modified script
    run_blvdiff(fixed_path)

    # 4 run BLVDIFF explain + diff for analysis
    run_blvdiff(script_path, flags=["--explain", "--diff"])


def main():
    scripts = sorted(SCRIPT_DIR.glob("*.py"))
    a11y_tasks = load_a11y_tasks()

    if not scripts:
        return

    for run_idx in range(1, NUM_RUNS + 1):
        print(f"\n=== SIMULATION RUN {run_idx} ===")
        for script in scripts:
            task_sequence = a11y_tasks.get(script.name, [])
            simulate_blv_behavior(script, task_sequence, run_idx)

if __name__ == "__main__":
    main()
