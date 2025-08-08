from enum import Enum

class PpimsResponseCodes(Enum):
    SUCCESS = "000"
    NOT_FOUND = "404"
    ERROR = "500"