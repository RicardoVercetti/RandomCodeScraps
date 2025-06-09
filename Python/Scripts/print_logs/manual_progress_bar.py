import time
#import math

def progress_bar(progress, total):
    percent = round(progress/total * 100)
    space = " "
    total_len = 30
    filler = "#"
    to_fill = round(progress/100 * total_len)
    remaining = total_len - to_fill
    filler_str = filler*to_fill
    rem_str = space * remaining
    print(f"[{filler_str}{rem_str}] {percent}%", end="\r", flush=True)

for i in range(1, 101, 5):
    progress_bar(i, 100)
    time.sleep(0.1)


print("\nDone!")
