print("started... yay...")

def get_file():
    with open("/home/jehoniah/Documents/repos/RandomCodeScraps/Python/Scripts/uthkarsh_migration_scripts/res/card_accounts.txt", "r") as f:
        lines = f.readlines()
        return lines

lines = get_file()
each_lines = [line.split(",") for line in lines]
print("items per row: ", len(each_lines[0]))