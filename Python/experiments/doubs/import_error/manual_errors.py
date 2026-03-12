
class ManualError(Exception):
    pass

class ManualErrorPartII(Exception):
    def __init__(self, message: str):
        super().__init__(message)



try:
    raise ManualError
except Exception as e:
    print("error happened says:", e)

try:
    raise ManualErrorPartII("this is the manual error part II")
except ManualErrorPartII as e:
    print("This is manual error part II")
except Exception as e:
    print("error happened this time is:", e)