import subprocess

for i in range(20):
    for j in range(1, 21):
        subprocess.run(["cargo", "run", f"test{j}.py"])