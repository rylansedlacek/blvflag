import subprocess

for i in range(20): # a simple script to run all test files for metric generation
    for j in range(1, 21):
        subprocess.run(["cargo", "run", f"/Users/rylan/blvflag/results/test_scripts/test{j}.py", "--explain"])