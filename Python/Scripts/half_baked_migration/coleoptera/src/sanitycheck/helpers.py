def remove_double_quotes(s: str) -> str:
    if s.startswith("\"") and s.endswith("\""):
        return s[1:-1]
    return s

def main():
    pan = "\"3536110000000004\""
    cleaned_pan = remove_double_quotes(pan)
    print(f"post cleanup: '{cleaned_pan}'")

if __name__ == "__main__":
    main()