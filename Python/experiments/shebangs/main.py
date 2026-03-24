import os
import re

# with open("./res/update-macports-portfile.py", "rb") as file:
#     data = file.read()
#     print(type(data))

print(os.getcwd())

folder_path = os.getcwd() + "/Python/experiments/shebangs/res/"

files = [f for f in os.listdir(folder_path) if os.path.isfile(os.path.join(folder_path, f))]


print(files)

for file in files:
    if file == "update-macports-portfile.py": continue
    with open(os.path.join(folder_path + file), "rb") as f:
        data = f.read()

        matches = re.findall(b'\x0a', data)
        # print(f"matches: {matches}")
        # print(data.hex())
        print(len(matches))