# each '<' must have a '>'
# if there is a '/' between '<' and '>', don't expect a closing tag wih the same name
# Each tag can have any inner tags
# One tag(including opening or closing) can be in one line the rest can be in other lines
from __future__ import annotations

class Tag:
    def __init__(self, name: str):
        self.name = name
        self.children: [Tag] = []
        self.content: str = ""

    def add_children(self, tag: Tag):
        self.children.append(tag)

def find_next_closing(full_str: str, this_pos: int) -> int:
    current_pos = this_pos + 1
    # has_closing_tag = False
    while len(full_str) > current_pos:
        if full_str[current_pos] == ">":
            return current_pos
        current_pos += 1
    return current_pos

def find_first_start(full_str: str) -> int:
    for key, value in enumerate(full_str):
        if value == "<":
            return key
    return len(full_str)

def find_next_start(full_str: str, pos: int) -> int:
    current_pos = pos
    while len(full_str) > current_pos:
        if full_str[current_pos] == "<":
            return current_pos
        current_pos += 1

def is_self_closing(full_str: str, start_pos: int, end_pos: int) -> bool:
    pos_from_back = 1
    while end_pos-pos_from_back > start_pos:
        if full_str[end_pos-pos_from_back] == " " or full_str[end_pos-pos_from_back] == ">":
            pos_from_back += 1
            continue
        elif full_str[end_pos-pos_from_back] == "/":
            return True
        else:
            return False


def parse_xml(input_str: str):
    # the first item must always be a starting tag, if empty string, walk past it, if anything else(might not be right, but) also walk past it
    current_pos = 0
    full_len = len(input_str)

    # could be that everything is in one outer tag or all are sibling tags

    if input_str[current_pos] == "<":
        # find the closing section
        next_closing = find_next_closing(full_str=input_str, this_pos=current_pos)

        # check if self closed

        # if self closed, add the tag and move on

        # if not self closed, find the closing tag and process everything in between
        pass
    elif input_str[current_pos] == " ":
        # continue
        pass


def test():
    # sample_string = """
    #                 <tag>Heading </tag>
    #                 <2tag> Heading2 </2tag>
    #             """
    # # print(sample_string)
    # for st in sample_string:
    #     print(f"-{st.encode().hex()}-")

    sap = "some string before<sample1> some string after <sample2/> then the second one"

    start_pos = find_first_start(sap)
    # start_pos = 18
    pos = find_next_closing(sap, start_pos)
    first_xml = sap[start_pos: pos+1]
    print(f"start_pos: {start_pos}, end_pos: {pos}, substring: '{first_xml}'")
    print(f"first_xml isSelfClosing: {is_self_closing(sap, start_pos, pos+1)}")

    next_start = find_next_start(full_str=sap, pos=pos)
    next_end = find_next_closing(full_str=sap, this_pos=next_start)

    next_xml = sap[next_start: next_end+1]

    print(f"next_start: {next_start}, next_end: {next_end}, substring: {next_xml}")
    print(f"next_xml isSelfClosing: {is_self_closing(sap, next_start, next_end+1)}")





if __name__ == "__main__":
    test()
