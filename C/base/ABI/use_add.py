import ctypes

lib = ctypes.CDLL("./libadd.so")    # dlopen the .so

lib.add.restype = ctypes.c_long         # return type is long
lib.add.argtypes = [ctypes.c_long, ctypes.c_long]


print(lib.add(4, 4))
# print(4+4)