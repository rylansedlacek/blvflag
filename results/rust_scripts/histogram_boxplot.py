import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sns

# load timing log
df = pd.read_json("/Users/rylan/blvflag/tool/logs/timings.jsonl", lines=True)
df['timestamp'] = pd.to_datetime(df['timestamp'])

sns.histplot(df['duration_sec'], bins=20, kde=True)
plt.title("Runtime Distribution With --diff")
plt.xlabel("Duration (seconds)")
plt.ylabel("Count")
plt.show()

sns.boxplot(x=df['duration_sec'])
plt.title("Execution Time With --diff")
plt.ylabel("Duration (seconds)")
plt.show()
