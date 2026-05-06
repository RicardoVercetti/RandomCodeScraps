# IDEA
# A json string could possibly have 
# 1. string, 
# 2. number, 
# 3. boolean, 
# 4. null, 
# 5.array, 
# 6.object 

# outermost could be array or object
# if array > nested arrays or an object inside
# if object > key value pairs
#   - keys must be strings
#   - values could be strings/numbers, boolean, null, array, object

from enum import Enum


class Capsules(Enum):
    ARRAY = 0
    OBJECT = 1

class JsonArray:
    def __init__(self):
        pass

class JsonObject:
    def __init__(self):
        pass

    @staticmethod
    def is_json_object(st: str):
        # trim the leading and training spaces, should start with '{' and end with '}'
        pass


class JsonObject:

    def __init__(self):
        pass
        

string_value = "{\"key\": \"value\"}"




print(string_value)




