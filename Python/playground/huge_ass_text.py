print("started...")

with open("res/stacktrace.txt", "r") as file:
    lines = file.readlines()
    for line in lines:
        arr = (line[1:-1].split(","))
        for line in arr:
            print(line)

