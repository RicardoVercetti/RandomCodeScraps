# sanity check steps
# 1. check the given length of the files based on the type of the file
# 2. check the datatype of appropriate fields
# 3. [prolly not needed] cross checking - applicable only to the card-accounts and account cusotomers


from dataclasses import dataclass
@dataclass
class ProgressStatus:
    source_name: str
    total_items: int
    failed_items: int
    errors: list[str]           # have information on line number, filename, and perhaps even the card/account/customer number as well

class CardProducts:
    name: str
    bin: str


# cards load file
def sanity_check_cards(filenames: list[str]):
    # things to collect:
    # 1. card program name list
    # 2. binlist

    # things to check
    # 1. pan is n..19
    # 2. seq is n3, if empty, use 000
    # 3. expiry date n4 mandatory
    # 4. if the rows are symmetrical
    # 5. last_updated_date mandatory
    # 6. last_updated_user mandatory

    # todo:
    # i. go line by line and check the above
    # ii. add the card program list to conf
    # iii. take bin no configuration for each card product
    status = ProgressStatus("cards", None, 0, errors=[])
    critical_error = False
    lines = 0
    card_products: list[card_products] = []

    # then call it a day
    for filename in filenames:
        with open(filename, "r") as file:
            for line in file:                               # note each line has a '\n' at the end of the file
                # now start checking rules by each line
                oneline = line.strip().split(",")
                lines += 1                                  # current line
                
                if critical_error:      # skipping so that we can get the line number count
                    status.failed_items += 1
                    continue

                # totally 34 items in per row of cards.txt
                if len(oneline) != 34:
                    critical_error = True
                    status.errors.append(f"Invalid length: {len(oneline)} in line {lines} of file: {filename}. Lenth must be exactly {34}")
                    continue
                
                # pan length should always be less than 19
                if len(oneline[0] > 19 or oneline[0] < 10):
                    status.errors.append(f"Invalid pan length: '{oneline[0]}', in line {lines} of file: {filename}")
                
                # lets continue tomorrow


                    
                

# accounts load file

# customer load file

# card_accounts file

# customer_accounts file

