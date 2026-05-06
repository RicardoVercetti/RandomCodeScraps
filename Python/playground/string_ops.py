"""Self implimentations of some string functions"""

def trim(st: str) -> str:
    """removes the leading and trailing spaces"""
    
    replaced_str = ""
    for char in st:
        if char == " ":
            replaced_str += char

    if len(replaced_str) < 1:
        return st

    front_pos = None
    back_pos = None

    # first non-space char position from forward
    for pos, char in enumerate(st):
        if front_pos is None and char == " ":
            continue
        if front_pos is None and char != " ":
            front_pos = pos
        if front_pos is not None:
            break
    
    # first non-space char position from backwards
    str_len = len(st)
    for pos, char in enumerate(list(reversed(st))):
        if back_pos is None and char == " ":
            continue
        if back_pos is None and char != " ":
            back_pos = str_len - pos
        if back_pos is not None:
            break

    return st[front_pos:back_pos]
