"""Utility classes that can be used for the migration script"""

def next_double_quotation(full_str: str, this_quotation: int) -> int:
    """Given a position N from a string S, return the next double quotation '"' found, if none found, returns the last position"""
    loop_location = this_quotation + 1
    while len(full_str) > loop_location:
        if full_str[loop_location] == "\"":
            return loop_location
        loop_location += 1
    return loop_location


def parse_extract_files(input_string: str) -> [str]:
    """Pass in the comma separated, each field surrounded by quotation string to get back a list of strings"""
    full_len = len(input_string)
    current_pos = 0
    separated_full_values = []

    if full_len < 1:
        return separated_full_values

    # go until finding the first '"' or ','
    while current_pos < full_len:
        this_str = input_string[current_pos]

        if this_str == ",":
            current_pos += 1
            separated_full_values.append("")
            continue
        elif this_str == "\"":      # note: this could be a single quote as well
            current_pos += 1
            end_location = next_double_quotation(full_str=input_string, this_quotation=current_pos)
            separated_full_values.append(input_string[current_pos: end_location])
            current_pos = end_location + 1
            continue
        elif this_str == " ":
            current_pos += 1
            continue
        else:
            print(f"[warn] it shouldn't enter into this location. Pos: {current_pos}")
        

        current_pos += 1
    return separated_full_values


if __name__ == "__main__":
    sample_string = '"35361102000,00754","001","1375020000000001","10","1","10",,"2025-11-10 15:57:03","SP",'
    parsed = parse_extract_files(sample_string)
    
    print(f"parsed: {parsed}")

    print("-- key & value --")
    for key, value in enumerate(parsed):
        print(f"{key}: {value}")
