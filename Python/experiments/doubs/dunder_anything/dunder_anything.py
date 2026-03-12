class SampleClass:
    __some_value__ = "some string"

    value1: str = "value 1 string"
    value2: list[str] = ["str2, str22"]

class SomeOtherClass:
    name = "class name"
    value2 = 234
    _some_othername = "some other name"         # a Gentleman's agreement, dont access this from outside
    __some_value = 2436                         # name is mangled by python interpreter so that it's unaccessible


c: SomeOtherClass = SomeOtherClass()
print(c.name)
print(c.value2)
print(c._some_othername)
# print(c.__some_value)