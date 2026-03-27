import json
import pandas as pd
import numpy as np
from pathlib import Path
import matplotlib.pyplot as plt

RESULTS_PATH = Path("/Users/rylan/blvflag/results/sim_results.json")
OUT_DIR = Path("/Users/rylan/blvflag/results/analysis_outputs")
OUT_DIR.mkdir(exist_ok=True)

def load_data():
    with open(RESULTS_PATH, "r") as f:
        data = json.load(f)
    return pd.DataFrame(data)

def preprocess(df):
    fix_map = {
        "GOOD_FIX": 1.0,
        "PARTIAL_FIX": 0.5,
        "POOR_FIX": 0.0
    }

    df["fix_score"] = df["fix_type"].map(fix_map)
    return df


def compute_reduction(df):
    baseline = df[df["mode"] == "baseline"].groupby("script")["nav_steps"].mean()

    def reduction(row):
        if row["script"] in baseline:
            return baseline[row["script"]] - row["nav_steps"]
        return np.nan

    df["nav_reduction"] = df.apply(reduction, axis=1)

    return df


#nav steps by mode
def plot_nav_steps(df):
    plt.figure()
    df.groupby("mode")["nav_steps"].mean().plot(kind="bar")
    plt.title(" User 4 - Average Navigation Steps by Mode")
    plt.ylabel("Nav Steps")
    plt.tight_layout()
    plt.savefig(OUT_DIR / "U4_fig_nav_steps.png")
    plt.close()

#fix quality
def plot_fix_quality(df):
    plt.figure()
    df.groupby("mode")["fix_score"].mean().plot(kind="bar")
    plt.title("User 4 - Average Fix Quality by Mode")
    plt.ylabel("Fix Score")
    plt.tight_layout()
    plt.savefig(OUT_DIR / "U4_fig_fix_quality.png")
    plt.close()


# nav seteps distrubtion
def plot_boxplot(df):
    plt.figure()
    df.boxplot(column="nav_steps", by="mode")
    plt.title("User 4 - Navigation Steps Distribution")
    plt.suptitle("")
    plt.ylabel("Nav Steps")
    plt.tight_layout()
    plt.savefig(OUT_DIR / "U4_fig_nav_boxplot.png")
    plt.close()

#tradeoofs
def plot_tradeoff(df):
    plt.figure()

    for mode in df["mode"].unique():
        subset = df[df["mode"] == mode]
        plt.scatter(subset["nav_steps"], subset["fix_score"], label=mode)

    plt.xlabel("Navigation Steps")
    plt.ylabel("Fix Score")
    plt.title("User 4 - Navigation vs Fix Quality Tradeoff")
    plt.legend()
    plt.tight_layout()
    plt.savefig(OUT_DIR / "U4_fig_tradeoff.png")
    plt.close()

# improvements
def plot_improvement(df):
    baseline = df[df["mode"] == "baseline"].groupby("script")["nav_steps"].mean()

    improvements = []

    for _, row in df.iterrows():
        base = baseline.get(row["script"], None)
        if base is not None:
            improvements.append(base - row["nav_steps"])
        else:
            improvements.append(np.nan)

    df["improvement"] = improvements

    plt.figure()
    df["improvement"].dropna().hist(bins=20)
    plt.title(" User 4- Navigation Improvement (Baseline - Tool)")
    plt.xlabel("Improvement")
    plt.ylabel("Frequency")
    plt.tight_layout()
    plt.savefig(OUT_DIR / "U4_fig_improvement_hist.png")
    plt.close()

def main():
    df = load_data()
    df = preprocess(df)

    df.to_csv(OUT_DIR / "full_data.csv", index=False)

    df = compute_reduction(df)
    plot_nav_steps(df)
    plot_fix_quality(df)
    plot_boxplot(df)
    plot_tradeoff(df)
    plot_improvement(df)

    print(f"\nDONE — outputs saved to {OUT_DIR}")


if __name__ == "__main__":
    main()