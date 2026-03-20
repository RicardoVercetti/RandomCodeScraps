# print("this is a tests py")
# with open("./res/cards.txt", "r") as file:
#     for line in file:
#         oneline = line.strip().split(",")
#         # print(f"{1}, {2}, {3}: {oneline[0]}, {oneline[1]}, {oneline[2]}")
#         print(len(oneline))

cards_header = [
    "pan",
    "seqence_no",
    "card_program",
    "dafault_account_type",
    "card_status",
    "custom_state",
    "expiry_date",
    "hold_response_code",
    "track_2_value",
    "track_2_offset",
    "secure_pin_len",
    "secure_pin_offset_or_PVV",
    "insecure_pin_offset_or_PVV",
    "validation_data_question",
    "validation_data",
    "cardholder_response_information",
    "mailer_destination",
    "company_card",
    "descritionary_data",
    "date_issued",
    "date_activated",
    "issuer_reference",
    "branch_code",
    "customer_id",
    "insecure_pin_length_or_PVKI",
    "extended_fields",
    "expiry_day",
    "from_date",
    "from_day",
    "contactless_discretionary_data",
    "dynamic_cvv_key_index",
    "batch_number",
    "last_updated_date",
    "last_updated_user"
]

account_headers = [
    "account_id",
    "account_type",
    "currency_code",
    "hold_response_code",
    "account_product",
    "extended_fields",
    "overdraft_limit",
    "account_nickname",
    "last_updated_date",
    "last_updated_user"
]

def range_of_headers():
    x = [s for s in range(1, 35)]       # this couldv'e been `x = list(range(1, 35))`
    print(",".join(map(str, x)))


def sets_in_one_column():
    """
    5th → card status, 
    6th → custom state, 
    7th → hold response code
    8th → hold response code
    11th → secure pin offset
    21st → issuer reference
    23rd → branch code
    """
    card_status_sets = set()
    with open("./res/cards.txt", "r") as file:
        line_no = 1
        for line in file:
            if line_no == 1:        # skip the manually inserted number in the first line
                line_no+=1
                continue
            card_status_sets.add(line.split(",")[11])
    print(f"all in set: {card_status_sets}")

def all_columns_with_null():
    null_column_no = set()
    # null_columns: dict[int, int] = dict()           # dict[int, int] = [position, count_of_null]
    total_rows = 0
    with open("./res/cards.txt", "r") as file:
        for line in file:
            total_rows += 1
            for key, value in enumerate(line.split(",")):
                if len(value) < 1 or value.lower() == "null":
                    # if null_columns.get(key) is not None:
                    #     null_columns[key] = null_columns.get(key) + 1
                    # null_columns[key] = 1     # when adding, add in index no
                    null_column_no.add(key)

    # print(f"all keys column: {null_column_no}")
    print("\n".join(f"{item+1}: {cards_header[item]}" for item in null_column_no))
    # print("\n".join(f"{cards_header[pos]}: {count}" for pos, count in null_columns.items()))

    print(f"items that are mandatory: ")
    print("\n".join(f"{key + 1}: {header_name}" for key, header_name in enumerate(cards_header) if key not in null_column_no))

def all_cards_row():
    for key, value in enumerate(cards_header):
        print(f"{key}: {value}")

def insert_query_for_one_row():
    one_line = ""
    with open("./res/cards.txt", "r") as cards_file:
        num = 1;
        for line in cards_file:
            if num == 1:
                num += 1
                continue
            one_line = line
            break
    # print(f"oneline: {one_line}")
    for index, (head, value) in enumerate(zip(cards_header, one_line.split(","))):
        print(f"{index}: {head} - {value}")

    # prerequisite data
    # 1. insitution_nr
    # 2. branch_id {optional}
    # 3. card product
    # 4. card transaction enable disable values from card product
    # 5. 
    
    # madeup data
    # 1. masked card no
    
def all_rows_for_accounts():
    for index, row_name in enumerate(account_headers):
        print(f"{index}: {row_name}")

def all_rows_for_accounts_with_data():
    all_rows = []
    with open("./res/accounts.txt","r") as file:
        for line in file:
            one_line = line.strip().split(",")
            all_rows.append("\n".join(f"{idx}. {name}: {value}" for idx, (name, value) in enumerate(zip(account_headers, one_line))))

    print(all_rows[0])

def all_mandatory_fields_from_accounts():
    null_value_set = set()

    with open("./res/accounts.txt", "r") as f:
        for line in f:
            oneline = line.strip().split(",")
            for index, item in enumerate(oneline):
                if item == None or len(item) < 1:
                    null_value_set.add(index)       # index values starting from 0 are added
                # else:
                #     if index == 0:
                #         print(f"value at index 0: {item}")
    
    # non_null_headers = []
    # for index, header in enumerate(account_headers):
    #     if index not in null_value_set:
    #         non_null_headers.append(header)

    print("All mandate values are:")
    print("\n".join(f"{idx}: {header}" for idx, header in enumerate(account_headers) if idx not in null_value_set))

def atleast_one_instance_from_accounts():
    atleast_one = set()

    with open("./res/accounts.txt", "r") as file:
        for line in file:
            oneline = line.strip().split(",")
            for idx, item in enumerate(oneline):
                if item != None and len(item) > 1:
                    atleast_one.add(idx)

    # currated_headers = [header for idx, header in enumerate(account_headers) if idx in atleast_one]
    print("alteast one non nul columns:")
    print("\n".join(f"{index + 1}. {header}" for index, header in enumerate(account_headers) if index in atleast_one))

def any_row_that_has_empty_last_update_value():
    with open("./res/accounts.txt", "r") as file:
        for idx, line in enumerate(file):
            oneline = line.strip().split(",")
            print(f"lastupdated user: : {oneline[9]}")
            if oneline[9] == None or len(oneline[9]) < 1:
                print(f"invalid last_updated_user at line: {idx+1}, data: {",".join(oneline)}")
            


# insert_query_for_one_row()
# all_columns_with_null()
# sets_in_one_column()
# all_cards_row()
# all_rows_for_accounts()
# all_rows_for_accounts_with_data()
all_mandatory_fields_from_accounts()
atleast_one_instance_from_accounts()
# any_row_that_has_empty_last_update_value()