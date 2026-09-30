def parse_limit_value(value_str):
    """Convert string to float, treating last two digits as decimal places. Return 0.00 if empty or invalid"""
    if not value_str or value_str.strip() == '':
        return 0.00
    try:
        # Remove any quotes and whitespace
        cleaned = value_str.strip().strip('"')
        # Convert to integer first to handle values like "11000000"
        int_val = int(cleaned)
        # Divide by 100 to treat last two digits as decimal places
        return int_val / 100.0
    except Exception as _e:
        return 0.00

value = parse_limit_value("11000000")

print(f"parsed: {value}")       # 11 000 000  110 000.0