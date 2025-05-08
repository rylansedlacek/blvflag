import subprocess
import time
import matplotlib.pyplot as plt # thank goodness for youtube

COMMANDS = [
    ("--diff", ["cargo", "run", "script.py" , "--diff"]), # diff
    ("--explain", ["cargo", "run", "script.py" , "--explain"]), # explain
]

results = [] # store results in list

for label, cmd in COMMANDS: # for each command
    print(f"running {label}")
    total_duration = 0.0 

    for i in range(10):
        start = time.time()
        subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE) 
        end = time.time()
        duration = end - start # calc total
        total_duration += duration

    avg_duration = total_duration / 10 # average based on number of runs
    results.append((label, avg_duration))

labels, durations = zip(*results) # and plot it
plt.bar(labels, durations)
plt.ylabel("execution time (s)") # y axis
plt.title("--diff vs --explain Runtime") # title
plt.savefig("diff_vs_explain.png")
plt.show()
