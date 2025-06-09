import time

def progress_bar(progress, total, bar_length=30):
    percent = int((progress/total)*100)
    bar = "#" * int(bar_length*progress / total)
    space = " " * (bar_length - len(bar))
    print(f"[{bar}{space}] {percent}%", end="\r", flush=True)

for i in range(0, 101):
    progress_bar(i, 100)
    time.sleep(0.05)

print("\nDone")
