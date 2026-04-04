#!/usr/bin/env python3

from __future__ import annotations
import argparse
import csv
import json
import os
import random
import subprocess
import tempfile
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path
from statistics import mean


OUT_DIR = Path("/Users/rylan/blvflag/results/domjudge_sim")
OUT_DIR.mkdir(parents=True, exist_ok=True)

ACCEPT_SET = {"accepted", "correct"}

@dataclass
class SubmissionRow:
    submitid: int
    teamid: int
    probid: int
    submittime: str
    result: str
    filename: str
    sourcecode: str


def parse_args() -> argparse.Namespace:
    p = argparse.ArgumentParser(description="Branching Domjudge simulator for BLVDIFF modes")
    p.add_argument("--max-cycles", type=int, default=30)
    p.add_argument("--max-steps-per-cycle", type=int, default=12)
    p.add_argument("--runs", type=int, default=300, help="Monte Carlo runs per cycle and mode")
    p.add_argument("--seed", type=int, default=42)
    p.add_argument("--only-probid", type=int, default=None)
    p.add_argument("--only-teamid", type=int, default=None)
    return p.parse_args()


def env_or_default(name: str, default: str) -> str:
    v = os.getenv(name)
    return v if v else default


def mysql_query(sql: str) -> list[SubmissionRow]:
    user = os.getenv("DOMJUDGE_DB_USER")
    password = os.getenv("DOMJUDGE_DB_PASSWORD")
    if not user or not password:
        raise RuntimeError("DOMJUDGE_DB_USER and DOMJUDGE_DB_PASSWORD are required")

    host = env_or_default("DOMJUDGE_DB_HOST", "127.0.0.1")
    port = env_or_default("DOMJUDGE_DB_PORT", "3306")
    db_name = env_or_default("DOMJUDGE_DB_NAME", "domjudge_data")

    with tempfile.NamedTemporaryFile("w", delete=False, encoding="utf-8") as tmp:
        tmp.write("[client]\n")
        tmp.write(f"user={user}\n")
        tmp.write(f"password={password}\n")
        tmp.write(f"host={host}\n")
        tmp.write(f"port={port}\n")
        tmp.write(f"database={db_name}\n")
        defaults_file = tmp.name

    cmd = [
        "mysql",
        f"--defaults-extra-file={defaults_file}",
        "--batch",
        "--raw",
        "--skip-column-names",
        "-e",
        sql,
    ]

    try:
        cp = subprocess.run(cmd, capture_output=True, text=True, check=False)
    finally:
        try:
            os.remove(defaults_file)
        except OSError:
            pass

    if cp.returncode != 0:
        raise RuntimeError(cp.stderr.strip() or "mysql query failed")

    out: list[SubmissionRow] = []
    reader = csv.reader(cp.stdout.splitlines(), delimiter="\t")
    for rec in reader:
        if len(rec) != 7:
            continue
        try:
            out.append(
                SubmissionRow(
                    submitid=int(rec[0]),
                    teamid=int(rec[1]),
                    probid=int(rec[2]),
                    submittime=rec[3],
                    result=(rec[4] or "").strip().lower(),
                    filename=rec[5],
                    sourcecode=rec[6],
                )
            )
        except ValueError:
            continue
    return out


def load_cycles(max_cycles: int, only_probid: int | None, only_teamid: int | None) -> dict[str, list[SubmissionRow]]:
    where = ["sf.filename LIKE '%.py'"]
    if only_probid is not None:
        where.append(f"s.probid = {only_probid}")
    if only_teamid is not None:
        where.append(f"s.teamid = {only_teamid}")

    sql = f"""
SELECT
  s.submitid,
  s.teamid,
  s.probid,
  s.submittime,
  j.result,
  sf.filename,
  CONVERT(sf.sourcecode USING utf8mb4) AS sourcecode
FROM submission s
JOIN judging j ON s.submitid = j.submitid
JOIN submission_file sf ON s.submitid = sf.submitid
WHERE {' AND '.join(where)}
ORDER BY s.teamid, s.probid, s.submittime, s.submitid;
""".strip()

    rows = mysql_query(sql)

    grouped: dict[str, list[SubmissionRow]] = defaultdict(list)
    for r in rows:
        key = f"team{r.teamid}_prob{r.probid}"
        grouped[key].append(r)

    # stable truncation to first N cycles
    items = list(grouped.items())[:max_cycles]
    out: dict[str, list[SubmissionRow]] = {}
    for k, v in items:
        out[k] = v
    return out


def count_changed_lines(a: str, b: str) -> int:
    a_lines = a.splitlines()
    b_lines = b.splitlines()
    m = min(len(a_lines), len(b_lines))
    changed = sum(1 for i in range(m) if a_lines[i] != b_lines[i])
    changed += abs(len(a_lines) - len(b_lines))
    return changed


def find_next_diff_verdict_idx(seq: list[SubmissionRow], i: int, limit: int = 4) -> int | None:
    curr = seq[i].result
    for j in range(i + 1, min(len(seq), i + 1 + limit)):
        if seq[j].result != curr:
            return j
    return None


def transition_index(mode: str, seq: list[SubmissionRow], i: int, rng: random.Random) -> int:
    if i >= len(seq) - 1:
        return i

    # always legal next
    next_i = i + 1

    curr = seq[i]
    nxt = seq[next_i]
    repeated_error = (curr.result not in ACCEPT_SET) and (nxt.result == curr.result)
    small_patch = count_changed_lines(curr.sourcecode, nxt.sourcecode) <= 2

    if mode == "baseline":
        return next_i

    # diff helps when changes are tiny/redundant
    if mode == "diff":
        if repeated_error and small_patch and rng.random() < 0.55:
            jump = find_next_diff_verdict_idx(seq, i, limit=4)
            if jump is not None:
                return jump
            return min(i + 2, len(seq) - 1)
        return next_i

    # explain helps break repeated-error loops, but can add occasional detours
    if mode == "explain":
        if repeated_error and rng.random() < 0.50:
            return min(i + 2, len(seq) - 1)
        if rng.random() < 0.08:
            return next_i  # small overhead/noise, no skip
        return next_i

    # both gets strongest skip behavior on repeated errors
    if mode == "both":
        if repeated_error and rng.random() < 0.70:
            jump = find_next_diff_verdict_idx(seq, i, limit=5)
            if jump is not None:
                return jump
            return min(i + 2, len(seq) - 1)
        return next_i

    return next_i


def run_mode_once(mode: str, seq: list[SubmissionRow], max_steps: int, rng: random.Random) -> dict:
    i = 0
    visited = 0
    reached_accept = False
    submit_path: list[int] = []

    while visited < max_steps and i < len(seq):
        row = seq[i]
        submit_path.append(row.submitid)
        visited += 1

        if row.result in ACCEPT_SET:
            reached_accept = True
            break

        ni = transition_index(mode, seq, i, rng)
        if ni <= i:
            ni = i + 1
        i = min(ni, len(seq) - 1)

        # prevent infinite loops in degenerate conditions
        if visited >= len(seq) + 3:
            break

    return {
        "success": 1 if reached_accept else 0,
        "steps_to_fix": visited if reached_accept else None,
        "steps_taken": visited,
        "path": submit_path,
    }


def bootstrap_ci(values: list[float], rng: random.Random, n_boot: int = 2000, alpha: float = 0.05) -> tuple[float, float, float]:
    if not values:
        return 0.0, 0.0, 0.0
    n = len(values)
    means = []
    for _ in range(n_boot):
        sample = [values[rng.randrange(n)] for _ in range(n)]
        means.append(mean(sample))
    means.sort()
    lo = means[int((alpha / 2) * n_boot)]
    hi = means[int((1 - alpha / 2) * n_boot) - 1]
    return mean(values), lo, hi


def main() -> None:
    args = parse_args()
    rng = random.Random(args.seed)

    cycles = load_cycles(args.max_cycles, args.only_probid, args.only_teamid)
    modes = ["baseline", "diff", "explain", "both"]

    run_rows = []
    for cycle_key, seq in cycles.items():
        for mode in modes:
            for run_id in range(1, args.runs + 1):
                rec = run_mode_once(mode, seq, args.max_steps_per_cycle, rng)
                run_rows.append({
                    "cycle_key": cycle_key,
                    "mode": mode,
                    "run_id": run_id,
                    "success": rec["success"],
                    "steps_to_fix": rec["steps_to_fix"] if rec["steps_to_fix"] is not None else "",
                    "steps_taken": rec["steps_taken"],
                    "submit_path": "|".join(str(x) for x in rec["path"]),
                })

    raw_csv = OUT_DIR / "branching_runs.csv"
    with raw_csv.open("w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=["cycle_key", "mode", "run_id", "success", "steps_to_fix", "steps_taken", "submit_path"])
        w.writeheader()
        w.writerows(run_rows)

    # aggregate by mode
    grouped = defaultdict(list)
    for r in run_rows:
        grouped[r["mode"]].append(r)

    summary = []
    for mode in modes:
        rows = grouped[mode]
        succ = [float(r["success"]) for r in rows]
        stf = [float(r["steps_to_fix"]) for r in rows if str(r["steps_to_fix"]).strip()]
        staken = [float(r["steps_taken"]) for r in rows]

        succ_m, succ_lo, succ_hi = bootstrap_ci(succ, rng)
        stf_m, stf_lo, stf_hi = bootstrap_ci(stf, rng)
        stk_m, stk_lo, stk_hi = bootstrap_ci(staken, rng)

        summary.append({
            "mode": mode,
            "n": len(rows),
            "success_rate_mean": round(succ_m, 4),
            "success_rate_ci_low": round(succ_lo, 4),
            "success_rate_ci_high": round(succ_hi, 4),
            "steps_to_fix_mean": round(stf_m, 4),
            "steps_to_fix_ci_low": round(stf_lo, 4),
            "steps_to_fix_ci_high": round(stf_hi, 4),
            "steps_taken_mean": round(stk_m, 4),
            "steps_taken_ci_low": round(stk_lo, 4),
            "steps_taken_ci_high": round(stk_hi, 4),
        })

    summary_csv = OUT_DIR / "branching_summary_by_mode.csv"
    with summary_csv.open("w", newline="", encoding="utf-8") as f:
        w = csv.DictWriter(f, fieldnames=list(summary[0].keys()))
        w.writeheader()
        w.writerows(summary)

    print(f"Wrote: {raw_csv}")
    print(f"Wrote: {summary_csv}")
    print("\nSummary:")
    for s in summary:
        print(s)


if __name__ == "__main__":
    main()
