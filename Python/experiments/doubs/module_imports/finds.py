def check_modules(di: dict):
    for key, value in di.items():
        print(f"{key}: {value}")

import sys
# print("before module import")

# imp = sys.modules
# print(f"type: {type(imp)}")
# print(f"value: {imp}")
# check_modules(imp)


from import_error import library

# print("library imported")

# print(f"value after import: {sys.modules}")
# print("after import")
check_modules(sys.modules)
