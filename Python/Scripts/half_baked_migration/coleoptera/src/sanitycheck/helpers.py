# src/sanitycheck/helpers.py

import re

def remove_double_quotes(s: str) -> str:
    if s.startswith("\"") and s.endswith("\""):
        return s[1:-1]
    return s

def is_all_digits(s: str) -> bool:
    return bool(re.match(r"^\d+$", s))

def is_empty(input: str) -> bool:
    return input == None or len(input) < 1

def main():
    pan = "\"3536110000000004\""
    cleaned_pan = remove_double_quotes(pan)
    print(f"post cleanup: '{cleaned_pan}'")

if __name__ == "__main__":
    main()
    # print(is_all_digits("3243b6"))