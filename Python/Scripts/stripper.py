"""The text from the estimates_txt.txt file looks like this:
8d
~
~

12d
9d





7d


7d
2d
3d
2d
5d
4d
15d

This is copied from an excel sheet and pasted into a text file.
The goal is to strip the text and get the total number of days.
The text will have 'd' at the end of each line.
The text will have empty lines and other characters.
"""

total_days = 0
day_arr = []

with open("/home/jehoniah/Documents/estimates_txt.txt", "r") as file:
    lines = file.readlines()


for line in lines:
    line = line.strip()
    # print("'"+line+"'")
    if line.endswith('d') and line[:-1].isdigit():
        total_days += int(line[:-1])
        day_arr.append(line[:-1])

print("Total days:", total_days)
# print("Days array:", day_arr)
print("Total elements:", len(day_arr))

# print("e".isdigit())