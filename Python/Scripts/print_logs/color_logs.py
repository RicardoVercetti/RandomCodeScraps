#print("\033[32m✓ Success!\033[0m")
#print("\033[31m✗ Error occurred\033[0m")
#print("\033[33m⚠ Warning: Check input\033[0m")
#print("\033[36mℹ Info: Running task...\033[0m")

RED = "\033[31m"
MAGNETA = "\033[35m"
RESET = "\033[0m"
ALT_FONT = "\033[3m"

def print_red(s):
    print(f"{RED}{s}{RESET}")

def print_magneta(s):
    print(f"{MAGNETA}{s}{RESET}")

#print("\033[31mThis is red\033[0m")
print_red("Here is a red text")
print_magneta("And a magneta text")
