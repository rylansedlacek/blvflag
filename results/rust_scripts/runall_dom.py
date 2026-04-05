#!/usr/bin/env python3

import csv
import os
import random
import subprocess
from statistics import mean

OUT_DIR = "/Users/rylan/blvflag/results/domjudge_2"
ACCEPT_SET = {"accepted", "correct"}
MYSQL_HOST = "127.0.0.1"
MYSQL_PORT = "3306"
MYSQL_DB = "domjudge_data"
MYSQL_USER = "root"
MYSQL_PASSWORD = "BooneFart56"

# controls
MAX_CYCLES = 30
MAX_STEPS_PER_CYCLE = 12
MAX_RUNS = 300
RANDOM_SEED = 42
ONLY_PROBID = None
ONLY_TEAMID = None

os.makedirs(OUT_DIR, exist_ok=True)

# runs a query and returns parsed rows - from H3
def mysql_query(sql):
    
    command = [
        "mysql",
        "-h",
        MYSQL_HOST,
        "-P",
        MYSQL_PORT,
        "-u",
        MYSQL_USER,
        f"-p{MYSQL_PASSWORD}",
        "--batch",
        "--raw",
        "--skip-column-names",
        "-e",
        sql,
        MYSQL_DB,
    ]

    result = subprocess.run(command, capture_output=True, text=True, check=False)
    if result.returncode != 0:
        raise RuntimeError("query failed")

    rows = []
    reader = csv.reader(result.stdout.splitlines(), delimiter="\t")

    for record in reader:
        if len(record) != 7:
            continue
        try:
            row = {
                "submitid": int(record[0]),
                "teamid": int(record[1]),
                "probid": int(record[2]),
                "submittime": record[3],
                "result": (record[4]).strip().lower(),
                "filename": record[5],
                "sourcecode": record[6],
            }

        except ValueError:
            continue
        rows.append(row)

    return rows


# loads grouped submission cycles by team 
def load_cycles(max_cycles, only_probid, only_teamid):
    where_parts = ["sf.filename like '%.py'"]
    if only_probid is not None:
        where_parts.append(f"s.probid = {only_probid}")
    if only_teamid is not None:
        where_parts.append(f"s.teamid = {only_teamid}")

    sql = """
select
  s.submitid,
  s.teamid,
  s.probid,
  s.submittime,
  j.result,
  sf.filename,
  convert(sf.sourcecode using utf8mb4) as sourcecode
from submission s
join judging j on s.submitid = j.submitid
join submission_file sf on s.submitid = sf.submitid
where {where_clause}
order by s.teamid, s.probid, s.submittime, s.submitid;
""".strip().format(where_clause=" and ".join(where_parts))

    rows = mysql_query(sql)
    grouped = {}

    for row in rows:
        key = f"team{row['teamid']}_prob{row['probid']}"

        if key not in grouped:
            grouped[key] = []

        grouped[key].append(row)

    cycles = {}
    count = 0
    for key, value in grouped.items():
        if count >= max_cycles:
            break
        cycles[key] = value
        count += 1
    return cycles


# counts line level differences between two code snippets - like s0 -> s1
def count_changed_lines(old_text, new_text):
    old_lines = old_text.splitlines()
    new_lines = new_text.splitlines()
    shared_count = min(len(old_lines), len(new_lines))
    changed = 0

    for index in range(shared_count):
        if old_lines[index] != new_lines[index]:
            changed += 1

    changed += abs(len(old_lines) - len(new_lines))
    return changed


# finds the next nearby index where the judging result changes - error -> fix
def find_diff_idx(sequence, start_index, limit=4):
    current_result = sequence[start_index]["result"]
    stop_index = min(len(sequence), start_index + 1 + limit)
    for index in range(start_index + 1, stop_index):
        if sequence[index]["result"] != current_result:
            return index
    return None


# chooses the next submission index based on mode - 
# baseline (same)
# diff (move a little)
# explain (move a little more)
# both (move the most)
# To simulate usefulness
def transition_index(mode, sequence, index, rng):
    if index >= len(sequence) - 1:
        return index

    next_index = index + 1
    current_row = sequence[index]
    next_row = sequence[next_index]
    repeated_error = current_row["result"] not in ACCEPT_SET and next_row["result"] == current_row["result"]
    small_patch = count_changed_lines(current_row["sourcecode"], next_row["sourcecode"]) <= 2

    if mode == "baseline":
        return next_index

    if mode == "diff":
        if repeated_error and small_patch and rng.random() < 0.55:
            jump_index = find_diff_idx(sequence, index, limit=4)
            if jump_index is not None:
                return jump_index
            return min(index + 2, len(sequence) - 1)
        return next_index

    if mode == "explain":
        if repeated_error and rng.random() < 0.50:
            return min(index + 2, len(sequence) - 1)
        if rng.random() < 0.08:
            return next_index
        return next_index

    if mode == "both":
        if repeated_error and rng.random() < 0.70:
            jump_index = find_diff_idx(sequence, index, limit=5)
            if jump_index is not None:
                return jump_index
            return min(index + 2, len(sequence) - 1)
        return next_index

    return next_index


# simulates one mode run and records success, steps, and path
def run_mode_once(mode, sequence, max_steps, rng):
    index = 0
    visited = 0
    reached_accept = False
    submit_path = []

    while visited < max_steps and index < len(sequence):
        row = sequence[index]
        submit_path.append(row["submitid"])
        visited += 1

        if row["result"] in ACCEPT_SET:
            reached_accept = True
            break

        next_index = transition_index(mode, sequence, index, rng)
        if next_index <= index:
            next_index = index + 1
        index = min(next_index, len(sequence) - 1)

        if visited >= len(sequence) + 3:
            break

    return {
        "success": 1 if reached_accept else 0,
        "steps_to_fix": visited if reached_accept else None,
        "steps_taken": visited,
        "path": submit_path,
    }


# computes mean and confidence interval for values - user interaction sim
def boot(values, rng, n_boot=2000, alpha=0.05):
    if not values:
        return 0.0, 0.0, 0.0

    sample_means = []
    value_count = len(values)

    for _ in range(n_boot):
        sample = []
        for _ in range(value_count):
            sample.append(values[rng.randrange(value_count)])
        sample_means.append(mean(sample))

    sample_means.sort()
    low_index = int((alpha / 2) * n_boot)
    high_index = int((1 - alpha / 2) * n_boot) - 1
    return mean(values), sample_means[low_index], sample_means[high_index]


# executes all simulation modes and writes the csv for figure generation
def main():
    rng = random.Random(RANDOM_SEED)
    cycles = load_cycles(MAX_CYCLES, ONLY_PROBID, ONLY_TEAMID)
    modes = ["baseline", "diff", "explain", "both"]

    run_rows = []
    for cycle_key, sequence in cycles.items():
        for mode in modes:
            for run_id in range(1, MAX_RUNS + 1):
                result = run_mode_once(mode, sequence, MAX_STEPS_PER_CYCLE, rng)
                run_rows.append({
                    "cycle_key": cycle_key,
                    "mode": mode,
                    "run_id": run_id,
                    "success": result["success"],
                    "steps_to_fix": result["steps_to_fix"] if result["steps_to_fix"] is not None else "",
                    "steps_taken": result["steps_taken"],
                    "submit_path": "|".join(str(item) for item in result["path"]),
                })

    rows_by_mode = {}
    for mode in modes:
        rows_by_mode[mode] = []
    for row in run_rows:
        rows_by_mode[row["mode"]].append(row)

    summary = []
    for mode in modes:
        rows = rows_by_mode[mode]
        success_values = []
        steps_to_fix_values = []
        steps_taken_values = []

        for row in rows:
            success_values.append(float(row["success"]))
            if str(row["steps_to_fix"]).strip():
                steps_to_fix_values.append(float(row["steps_to_fix"]))
            steps_taken_values.append(float(row["steps_taken"]))

        success_mean, success_low, success_high = boot(success_values, rng)
        steps_to_fix_mean, steps_to_fix_low, steps_to_fix_high = boot(steps_to_fix_values, rng)
        steps_taken_mean, steps_taken_low, steps_taken_high = boot(steps_taken_values, rng)

        summary.append({
            "mode": mode,
            "n": len(rows),
            "success_rate_mean": round(success_mean, 4),
            "success_rate_ci_low": round(success_low, 4),
            "success_rate_ci_high": round(success_high, 4),
            "steps_to_fix_mean": round(steps_to_fix_mean, 4),
            "steps_to_fix_ci_low": round(steps_to_fix_low, 4),
            "steps_to_fix_ci_high": round(steps_to_fix_high, 4),
            "steps_taken_mean": round(steps_taken_mean, 4),
            "steps_taken_ci_low": round(steps_taken_low, 4),
            "steps_taken_ci_high": round(steps_taken_high, 4),
        })

    summary_csv = os.path.join(OUT_DIR, "branching_summary_by_mode.csv")
    with open(summary_csv, "w", newline="", encoding="utf-8") as file_handle:
        writer = csv.DictWriter(file_handle, fieldnames=list(summary[0].keys()))
        writer.writeheader()
        writer.writerows(summary)

    print(f"Wrote: {summary_csv}")
    print("\nSummary:")
    for row in summary:
        print(row)


if __name__ == "__main__":
    main()
