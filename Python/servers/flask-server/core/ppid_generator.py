import random

def generate_random_number(length: int) -> int:
    if length <= 0:
        raise ValueError("Length must be a positive integer.")
    # If length is 1, smallest is 0
    if length == 1:
        return random.randint(0, 9)
    # Ensure the number has the exact length
    start = 10**(length - 1)
    end = (10**length) - 1
    return random.randint(start, end)

def generate_ppid() -> str:
    return generate_random_number(15)