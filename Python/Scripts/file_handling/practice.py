FILE_NAME = "fruits.txt"
import os

# create file and write 5 lines(favorite fruits)
#with open(FILE_NAME, "w") as f:
#    f.write("apple\n")
#    f.write("Orange\n")
#    f.write("pineapple\n")
#    f.write("pommegranate\n")
#    f.write("nuts\n")

# append 2 lines
#with open(FILE_NAME, "a") as f:
#    f.write("Line 1\n")
#    f.write("Line 2\n")


# Read and print all lines
#with open(FILE_NAME, "r") as f:
#    lines = f.read()
#    print(lines, end='')

try:
    os.remove(FILE_NAME)
except FileNotFoundError:
    print(f"File '{FILE_NAME}' does not exist") 


print("Finished")
