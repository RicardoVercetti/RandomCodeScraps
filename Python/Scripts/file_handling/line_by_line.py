print("Started!")

with open("hello.txt") as f:
    for line in f:
        print(line.strip()) # .strip() removes \n

print("Finised!")
