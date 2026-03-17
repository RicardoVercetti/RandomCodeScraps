
def main():
    print("the main method")

    one_str = """
        here is some multi line strings
        and this would be the second line
    """

    li = one_str.split("\n");
    for key, value in enumerate(li):
        print(f"{key}: {value}")

if __name__ == "__main__":
    main()