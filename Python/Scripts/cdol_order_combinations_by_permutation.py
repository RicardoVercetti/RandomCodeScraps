from itertools import permutations

variables = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k"]

# Generate all possible permutations of different lengths
# for r in range(1, len(variables) + 1):  # Length from 1 to 11
#     for perm in permutations(variables, r):
#         print("".join(perm))  # Concatenate and print

# Generate only full-length permutations
for perm in permutations(variables, len(variables)):
    print("".join(perm))  # Concatenate and print