# sanity check steps
# 1. check the given length of the files based on the type of the file
# 2. check the datatype of appropriate fields
# 3. [prolly not needed] cross checking - applicable only to the card-accounts and account cusotomers


# cards load file
def sanity_check_cards(filename: str):
    # things to collect:
    # 1. card program list
    # 2. binlist
    # 3. 

    # things to check
    # 1. pan is n..19
    # 2. seq is n3, if empty, use 000
    # 3. expiry date n4
    # 4. 
    with open(str, "r") as file:
        for line in line:
            line = line.strip()
            print(line)

# accounts load file

# customer load file

# card_accounts file

# customer_accounts file

