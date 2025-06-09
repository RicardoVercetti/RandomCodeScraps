import time

for i in range(0, 101, 10):
    print(f"Progress {i}%...".ljust(20), end="\r", flush=True)
    time.sleep(0.5)

print("Download completed")
