import sys


file_name = sys.argv[1]

with open(file_name, "rb") as f:
    read_data = f.read()
    print(type(read_data))

     


