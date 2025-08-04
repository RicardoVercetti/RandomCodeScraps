import json
from colorama import init, Fore, Style

init(autoreset=True)

def pretty_print_json_colored(data, indent=0):
    spacing = '    ' * indent
    if isinstance(data, dict):
        print(f"{spacing}{Fore.BLUE}{{{Fore.RESET}")
        items = list(data.items())
        for i, (key, value) in enumerate(items):
            is_last = i == len(items) - 1
            print(f"{'    ' * (indent + 1)}{Fore.YELLOW}\"{key}\"{Fore.RESET}: ", end="")

            if isinstance(value, dict):
                pretty_print_json_colored(value, indent + 1)
            elif isinstance(value, list):
                print(f"{Fore.BLUE}[{Fore.RESET}")
                for j, item in enumerate(value):
                    pretty_print_json_colored(item, indent + 2)
                print(f"{'    ' * (indent + 1)}{Fore.BLUE}]{Fore.RESET}")
            else:
                print(color_value(value))
        print(f"{spacing}{Fore.BLUE}}}{Fore.RESET}")
    elif isinstance(data, list):
        print(f"{spacing}{Fore.BLUE}[{Fore.RESET}")
        for item in data:
            pretty_print_json_colored(item, indent + 1)
        print(f"{spacing}{Fore.BLUE}]{Fore.RESET}")
    else:
        print(spacing + color_value(data))

def color_value(val):
    if isinstance(val, str):
        return Fore.GREEN + f"\"{val}\"" + Fore.RESET
    elif isinstance(val, (int, float)):
        return Fore.CYAN + str(val) + Fore.RESET
    elif isinstance(val, bool):
        return Fore.MAGENTA + str(val).lower() + Fore.RESET
    elif val is None:
        return Fore.RED + "null" + Fore.RESET
    else:
        return str(val)
