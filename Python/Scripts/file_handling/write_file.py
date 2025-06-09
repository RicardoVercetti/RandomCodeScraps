print("Runs")

# 'w' creates if doesn't exist, overwrites if exists

with open("hello.txt", "w") as f:
    f.write("Hello from Termux!\n")
    f.write("Practice Python file handling.\n")

print("Finished")
