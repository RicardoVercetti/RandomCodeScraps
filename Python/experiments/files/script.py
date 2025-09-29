print("Started")

with open("text.txt", "rb") as f:
    data = f.read()
    print(len(data))
    print(" ".join(f"{b:02x}" for b in data))


