import subprocess
import os
from pathlib import Path
import json
import shutil
import re

SCRIPT_DIR = Path("/Users/rylan/blvflag/results/test_scripts")
HISTORY_DIR = Path("/Users/rylan/blvflag/results/sim_history")
TRACE_DIR = Path("/Users/rylan/blvflag/results/a11y_traces")
RESULTS_LOG = Path("/Users/rylan/blvflag/results/sim_results.json")

HISTORY_DIR.mkdir(exist_ok=True)

CARGO_PROJECT_DIR = "/Users/rylan/blvflag/tool/"
NUM_RUNS = 1 # change as needed - 1 for now is sufficient based on 4 runs per


def extract_number(path: Path):
    nums = re.findall(r"\d+", path.stem)
    return int(nums[0]) if nums else 0


# run blv diff with flag
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


# meta data parser!
def parse_metadata(metadata_path):
    with open(metadata_path, 'r') as f:
        data = json.load(f)

    summary = data.get("summary", {})
    keystrokes = summary.get("keystrokes", {})

    key_freq = keystrokes.get("other_keys", {})

    nav_keys = ["Tab", "F6", "ArrowDown", "ArrowUp"] # all ive seen

    nav_count = sum(key_freq.get(k, 0) for k in nav_keys)
    enter_count = key_freq.get("Enter", 0)
    esc_count = key_freq.get("Esc", 0)

    total_events = summary.get("events", 1)

    return {
        "nav_steps": nav_count,
        "action_steps": enter_count,
        "abort_steps": esc_count,
        "total_events": total_events,
        "nav_ratio": nav_count / max(total_events, 1)
    }


# map the interactions - get nav steps
def map_interaction_to_line(lines, interaction_model):
    nav_depth = interaction_model["nav_steps"]
    line_idx = min(nav_depth, len(lines) - 1)

    if interaction_model["nav_ratio"] > 0.5:
        line_idx = max(0, line_idx - 2)

    while line_idx < len(lines) and not lines[line_idx].strip():
        line_idx += 1

    return min(line_idx, len(lines) - 1)


# adjust nav steps for tool interactions
def adjust_for_tool(interaction_model, mode):
    model = interaction_model.copy()

    if mode == "explain":
        model["nav_steps"] = int(model["nav_steps"] * 0.7)
    elif mode == "diff":
        model["nav_steps"] = int(model["nav_steps"] * 0.5)
    elif mode == "both":
        model["nav_steps"] = int(model["nav_steps"] * 0.4)

    return model


# modify script copy for diff flag
def modify_script(filepath, interaction_model, iteration, mode):
    with open(filepath, 'r') as f:
        lines = f.readlines()

    line_idx = map_interaction_to_line(lines, interaction_model)

    if interaction_model["nav_ratio"] < 0.3:
        fix_type = "GOOD_FIX"
    elif interaction_model["nav_ratio"] < 0.6:
        fix_type = "PARTIAL_FIX"
    else:
        fix_type = "POOR_FIX"

    lines[line_idx] = f"# {fix_type} iteration {iteration} ({mode})\n"

    hist_path = HISTORY_DIR / f"{filepath.stem}_{mode}_v{iteration}.py"

    with open(hist_path, 'w') as f:
        f.writelines(lines)

    return hist_path, fix_type

# log results in json for figure generation
def log_result(script, iteration, interaction_model, fix_type, mode):
    entry = {
        "script": script.name,
        "iteration": iteration,
        "mode": mode,
        "nav_steps": interaction_model["nav_steps"],
        "nav_ratio": interaction_model["nav_ratio"],
        "fix_type": fix_type
    }

    if RESULTS_LOG.exists():
        with open(RESULTS_LOG, "r") as f:
            data = json.load(f)
    else:
        data = []

    data.append(entry)

    with open(RESULTS_LOG, "w") as f:
        json.dump(data, f, indent=2)

# isolated fix
def prepare_working_copy(script_path, iteration, mode):
    temp_dir = HISTORY_DIR / "working"
    temp_dir.mkdir(exist_ok=True)

    dest = temp_dir / f"{script_path.stem}_iter{iteration}_{mode}.py"
    shutil.copy(script_path, dest)

    return dest

# take everything and use parsed steps to run.
def simulate_blv_behavior(script_path, metadata_path, iteration, mode):
    print(f"\n[SIMULATION] {mode} | {script_path.name} ↔ {metadata_path.name}")

    base_model = parse_metadata(metadata_path)
    adjusted_model = adjust_for_tool(base_model, mode)

    working_copy = prepare_working_copy(script_path, iteration, mode)

    fixed_path, fix_type = modify_script(
        working_copy,
        adjusted_model,
        iteration,
        mode
    )

    flags = ["--explain", "--diff"]
    if mode == "explain":
        flags = ["--explain"]
    elif mode == "diff":
        flags = ["--diff"]

    run_blvdiff(fixed_path, flags=flags)

    log_result(script_path, iteration, adjusted_model, fix_type, mode)


#cool
def main():
    scripts = sorted(SCRIPT_DIR.glob("*.py"), key=extract_number)
    metadata_files = sorted(TRACE_DIR.glob("metadata_*.json"))

    print(f"\nFound {len(scripts)} scripts")
    print(f"Found {len(metadata_files)} metadatas")

    if len(scripts) != len(metadata_files):
        print("")

    paired = list(zip(scripts, metadata_files))

    for run_idx in range(1, NUM_RUNS + 1):
        print(f"\nsim run {run_idx}")

        for script, metadata_path in paired:
            for mode in ["baseline", "explain", "diff", "both"]:
                simulate_blv_behavior(
                    script,
                    metadata_path,
                    run_idx,
                    mode
                )


if __name__ == "__main__":
    main()