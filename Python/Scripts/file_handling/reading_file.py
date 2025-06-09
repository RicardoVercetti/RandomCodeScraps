print("Started!")

with open("hello.txt", "r") as f:
    content = f.read()
    print(f"Type: {type(content)}")
    print(content)


print("Finished!")
