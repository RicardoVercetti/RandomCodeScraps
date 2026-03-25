# sanity check steps
# 1. check the given length of the files based on the type of the file
# 2. check the datatype of appropriate fields
# 3. [prolly not needed] cross checking - applicable only to the card-accounts and account cusotomers


from dataclasses import dataclass
from src.sanitycheck.helpers import remove_double_quotes, is_all_digits, is_empty
from src.utils.logger_setup import logger

@dataclass
class CardProductDto:
    name: str
    bin: str

class InvalidCardProgram(Exception):
    pass

class CardProducts:
    def __init__(self):
        self.card_products: list[CardProductDto] = []

    def check_and_push(self, pan: str, product_name):
        # first the product_name have to exist. if it does, the bin have to be the same
        for product in self.card_products:
            if product.name == product_name:
                # here the bin must match, if not - there is been some kinda mistake in the file provided
                if product.bin == pan[:6]:
                    return          # all ok
                else:
                    raise InvalidCardProgram(f"Product Name matches but bin doesn't. ({product.name}, {product.bin}) != ({product_name}, {pan[:6]})")
        
        # if it reaches here, its a new product and bin
        self.card_products.append(CardProductDto(product_name, pan[:6]))

    def get_all_products(self):
        return self.card_products

@dataclass
class ProgressStatus:
    source_name: str
    total_items: int
    failed_items: int
    # errors: list[str]           # have information on line number, filename, and perhaps even the card/account/customer number as well
    card_product_list: list[CardProductDto]


# cards load file
def sanity_check_cards(filenames: list[str]) -> ProgressStatus:
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
    status = ProgressStatus("cards", None, 0, card_product_list=[])
    critical_error = False                  # if this occurs, we should'nt insert any data
    total_lines = 0
    card_products = CardProducts()

    # then call it a day
    for filename in filenames:
        lines = 0           # per file line count
        with open(filename, "r") as file:
            logger.info(f"performing cards extract sanity check on file: {filename}")
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
                    # status.errors.append(f"Invalid length: {len(oneline)} in line {lines} of file: {filename}. Lenth must be exactly {34}")
                    status.failed_items += 1
                    logger.error(f"Invalid length: {len(oneline)} in line {lines} of file: {filename}. Lenth must be exactly {34}")
                    continue

                # format data so that the double quotes can be removed
                clean_pan = remove_double_quotes(oneline[0])
                
                # 16 length logics are the only things handled, if else the bin stripping logic changes
                if len(clean_pan) != 16:
                    # status.errors.append(f"Invalid pan length: '{oneline[0]}', in line {lines} of file: {filename}")
                    status.failed_items += 1
                    logger.error(f"Invalid pan length: '{len(clean_pan)}', in line {lines} of file: {filename}")
                
                # pan should be all numeric
                if not is_all_digits(clean_pan):
                    status.failed_items += 1
                    logger.error(f"pan number must be all numeric. Got: '{clean_pan}' at line: {lines} of file: {filename}")

                # if card product name is same then bin should be same for each card
                clean_product_name = remove_double_quotes(oneline[2])
                try:
                    card_products.check_and_push(clean_pan, clean_product_name)
                except InvalidCardProgram as e:
                    # status.errors.append(f"{e}")
                    status.failed_items += 1
                    logger.error(f"{e}")

                # last updated date cannot be empty
                clean_last_updated_date = remove_double_quotes(oneline[32])
                if len(clean_last_updated_date) < 1:
                    # status.errors.append(f"last updated date is invalid at line: {lines} of file: {filename}. Value found: '{oneline[32]}'")
                    status.failed_items += 1
                    logger.error(f"last updated date is invalid at line: {lines} of file: {filename}. Value found: '{clean_last_updated_date}'")

                # last updated user cannot be empty
                clean_last_updated_user = remove_double_quotes(oneline[33])
                if len(clean_last_updated_user) < 1:
                    # status.error.append(f"last updated user is invalid at line: {lines} of file: {filename}. Value found: '{oneline[33]}'")
                    status.failed_items += 1
                    logger.error(f"last updated user is invalid at line: {lines} of file: {filename}. Value found: '{clean_last_updated_user}'")
            logger.info(f"fininshed sanity check on file: {filename}")

        total_lines += lines      
    # main fn
    status.total_items = total_lines
    status.card_product_list = card_products.get_all_products()
    return status     
                

# accounts load file
def sanity_check_accounts(filenames: list[str]):
    # mandatory fields
    # 0. account_id
    # 1. account_type
    # 2. curreny_code
    # 8. last_updated_date
    # 9. last_updated_user

    status = ProgressStatus("accounts", 0, 0, [])
    critical_error = False
    total_lines = 0

    for accounts_file in filenames:
        logger.info(f"started sanity check for accounts file: {accounts_file}")
        lines = 0
        with open(accounts_file, "r") as file:

            for line in file:
                oneline = line.strip().split(",")
                lines+=1

                # total length
                if len(oneline) != 10:
                    status.failed_items += 1
                    logger.error(f"invalid number of items in line: {lines} of file: {accounts_file}, expected: 10, actual: {len(oneline)}'")
                    continue            # if the length doesn't qualify, don't bother going down the line

                # account_id can't be empty
                clean_account_no = remove_double_quotes(oneline[0])
                if len(clean_account_no) < 5:
                    status.failed_items += 1
                    logger.error(f"invalid account number length at line: {lines} of file: {accounts_file}. content: {clean_account_no}")
                
                # account_type should exactly be 2 chars and should be numeric
                clean_account_type = remove_double_quotes(oneline[1])
                if len(clean_account_type) != 2:
                    status.falied_items += 1
                    logger.error(f"Invalid account type length at line: {lines} of file: {accounts_file}. expected: 2, actual: {len(clean_account_type)}")

                try:
                    num = int(clean_account_type)
                except ValueError:
                    status.failed_items += 1
                    logger.error(f"account type should be numberic in line: {lines} of file: {accounts_file}. found: {clean_account_type}")

                # currency code should be 3 characters and all numeric
                clean_currency_code = remove_double_quotes(oneline[2])
                if len(clean_currency_code) != 3:
                    status.failed_items += 1
                    logger.error(f"Invalid length of currency code in line: {lines} of file: {accounts_file}. expected: 3, actual: {len(clean_currency_code)}")

                try:
                    num = int(clean_currency_code)
                except ValueError:
                    status.failed_items += 1
                    logger.error(f"Currency code must be numeric at line: {lines} of file: {accounts_file}. found value: {clean_currency_code}")

                # last_updated_date must be atleast 19 like in → `2025-06-21 19:26:35`
                clean_last_updated_date = remove_double_quotes(oneline[8])
                if len(clean_last_updated_date) < 19:
                    status.failed_items += 1
                    logger.error(f"invalid length found in the last updated date at line: {lines} of file: {accounts_file}. expected >= 19, found: {len(clean_last_updated_date)}")

                
                # last_updated_user must be alleast 1 character
                clean_last_updated_user = remove_double_quotes(oneline[9])
                if len(clean_last_updated_user) < 1:
                    status.falied_items += 1
                    logger.error(f"Invalied last updated user date at line: {lines} of file: {accounts_file}")
        
        total_lines += lines
        logger.info(f"finised sanity check for account file: {accounts_file}")
    # main fn
    status.total_items = total_lines
    return status

# customer load file
def sanity_check_for_customers(filenames: list[str]) -> ProgressStatus:
    # all non null value
    # 0: customer_id
    # 3: c1_first_name
    # 6: c1_name_on_card
    # 21: postal_address_1
    # 23: postal_city
    # 36: vip_flag
    # 39: last_updated_date
    # 40: last_updated_user
    status = ProgressStatus("customers", 0, 0, [])
    total_items = 0
    critical_error = False

    for filename in filenames:
        logger.info(f"started sanity check for customers file: {filename}")
        line_nr = 0
        try:
            with open(filename, "r") as file:
                for line in file:
                    line_nr += 1
                    one_line = line.strip().split(",")

                    # validation1: total number of values
                    if len(one_line) != 41:
                        status.failed_items += 1
                        logger.error(f"Invalid number of items in line: {line_nr} of file: {filename}. expected: 41, got: {len(one_line)}")
                        continue

                    # validation2: customer_id must be present
                    clean_customer_id = remove_double_quotes(one_line[0])
                    if is_empty(clean_customer_id):
                        status.failed_items += 1
                        logger.error(f"customer_id is null at line: {line_nr} of file: {filename}")

                    # validation3: c1_first_name
                    clean_first_name = remove_double_quotes(one_line[3])
                    if is_empty(clean_first_name):
                        status.failed_items += 1
                        logger.error(f"mandatory first name value is not found in line: {line_nr} of file: {filename}. received: {clean_first_name}")

                    # validation4: c1_name_on_card
                    clean_name_on_card = remove_double_quotes(one_line[6])
                    if is_empty(clean_name_on_card):
                        status.failed_items += 1
                        logger.error(f"name of card value is not found in line: {line_nr} of file: {filename}, got: '{clean_name_on_card}'")

                    # validation5: postal address - this being null won't cause a problem in DCMS
                    
                    # validation6: postal_city - same as above
                    
                    # validation7: vip_flag
                    clean_vip_flag = remove_double_quotes(one_line[36])
                    if is_empty(clean_vip_flag):
                        status.failed_items += 1
                        logger.error(f"VIP flag is empty at line: {line_nr} of file: {filename}. Got: '{clean_vip_flag}'")

                    # validation8: last_updated_date
                    clean_last_updated_date = remove_double_quotes(one_line[39])
                    if is_empty(clean_last_updated_date):
                        status.failed_items += 1
                        logger.error(f"last updated date in empty at line: {line_nr} of file: {filename}. Got: '{clean_last_updated_date}'")
                    
                    # validation9: last_updated_user
                    clean_last_updated_user = remove_double_quotes(one_line[40])
                    if is_empty(clean_last_updated_user):
                        status.failed_items += 1
                        logger.error(f"last updated user is empty at line: {line_nr} of file: {filename}, Got: '{clean_last_updated_user}'")
            # end with
        except UnicodeDecodeError as err:
            logger.error(f"decode error for file: {filename} at line: {line_nr}, skipping sanity check for this file")
            status.failed_items += 1
            logger.error(err)

        total_items += line_nr
        logger.info(f"finished santiy check for customers file: {filename}")
    # end for
    status.total_items = total_items
    return status

# card_accounts file

# customer_accounts file

