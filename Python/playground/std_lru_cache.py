from functools import lru_cache
from datetime import datetime

call_stack_count = 0

class DiffObject:
    def __init__(self, inp: str):
        self.data = "some data"
        self.inp = inp

# @lru_cache(maxsize=30)
def factorial(n: int) -> int:
    call_stack_count += 1
    return n * factorial(n-1) if n else 1

# @lru_cache(maxsize=30)
def fibonacci(n: int, diff_obj: DiffObject) -> int:
    global call_stack_count
    call_stack_count += 1
    if n <= 1:
        return n
    return fibonacci(n-1, diff_obj) + fibonacci(n-2, diff_obj)

def main():
    diff_obj = DiffObject("10")
    start_time = datetime.now()
    print(f"start time: {start_time}")

    result = fibonacci(30, diff_obj)
    print(f"result: {result}")

    end_time = datetime.now()
    print(f"end time: {end_time}")

    print(f"elapsed: {end_time - start_time}")
    print(f"call stack count: {call_stack_count}")


if __name__ == "__main__":
    main()
