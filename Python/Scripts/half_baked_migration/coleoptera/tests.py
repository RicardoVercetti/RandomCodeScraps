# print("this is a tests py")
# with open("./res/cards.txt", "r") as file:
#     for line in file:
#         oneline = line.strip().split(",")
#         # print(f"{1}, {2}, {3}: {oneline[0]}, {oneline[1]}, {oneline[2]}")
#         print(len(oneline))

from src.headers import cards_header, account_headers, customer_headers

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
            

def one_row_from_customers_with_data():
    with open("./res/customers_copy.txt", "r") as file:
        for line in file:
            one_line = line.strip().split(",")
            # print(len(one_line))
            # print(f"oneline value: '{one_line}'")
            for key, (header, value) in enumerate(zip(customer_headers, one_line)):
                print(f"{key}. {header} - {value}")
            break           # stop at one round

def all_non_nulls_from_customers():
    null_set = set()
    line_no = 0
    with open("./res/customers_copy.txt", "r") as file:
        for line in file:
            line_no += 1
            one_line = line.strip().split(",")
            for index, column in enumerate(one_line):
                if column is None or column == "" or len(column) < 1:
                    null_set.add(index)
                # if index == 39 and column is None or column == "" or len(column) < 1:
                #     print(f"last updated data in null at line: {line_no}, value: '{column}'")
    
    non_null = [i for i in range(len(customer_headers)) if i not in null_set]
    # print(f"non nulls: {non_null}")
    print("non-null values from customers.txt")
    for item in non_null:
        print(f"{item}: {customer_headers[item]}")

def atleast_one_non_null():
    import sys
    atlease_one_nn = set()
    line_no = 0
    with open("./res/customers_copy.txt", "r") as file:
        for line in file:
            line_no += 1
            for idx, value in enumerate(line.strip().split(",")):
                if value is not None or value != "" or not (len(value) < 1):
                    atlease_one_nn.add(idx)
            if len(line.strip().split(",")) != 41:
                print(f"invalid length at line: {line_no}", file=sys.stderr)
    
    print(f"atleast one non-null values:")
    # print(f"len: {len(atlease_one_nn)}")
    for idx in atlease_one_nn:
        print(f"{idx + 1}: {customer_headers[idx]}")

def is_present(item: str) -> bool:
    return item != None and len(item) > 0

def print_customer_value_with_header(one_row: list[str]):
    for header, value in zip(customer_headers, one_row):
        print(f"{header} - {value}")

def customer_data_with_mutiple_names_n_name_on_cards():
    line_nr = 0
    with open("./res/customers_copy.txt", "r") as file:
        for line in file:
            line_nr += 1
            one_line = line.strip().split(",")
            # if item 2, 3, 4, 5, 6, (any one present) && 7, 8, 9, 10, 11(any one present)
            if is_present(one_line[2]) or is_present(one_line[3]) or is_present(one_line[4]) or is_present(one_line[5]) or is_present(one_line[6]):
                print(f"one row found at line: {line_nr}")
                if  is_present(one_line[7]) or is_present(one_line[8]) or is_present(one_line[9]) or is_present(one_line[10]) or is_present(one_line[11]):
                    print(f"data found at line: {line_nr}")
                    print_customer_value_with_header(one_line)

def customer_headers_with_index():
    for index, header in enumerate(customer_headers):
        print(f"{index}: {header}")

def encoding_error_file():
    with open("./res/customers.txt", "r") as file:
        line_nr = 0
        for line in file:
            line_nr += 1
            print(f"line no: {line_nr}")

# cards
# insert_query_for_one_row()
# all_columns_with_null()
# sets_in_one_column()
# all_cards_row()

# accounts
# all_rows_for_accounts()
# all_rows_for_accounts_with_data()
# all_mandatory_fields_from_accounts()
# atleast_one_instance_from_accounts()
# any_row_that_has_empty_last_update_value()

# customers
# one_row_from_customers_with_data()
# all_non_nulls_from_customers()
# atleast_one_non_null()
# customer_data_with_mutiple_names_n_name_on_cards()
# customer_headers_with_index()

# print(len(customer_headers))


encoding_error_file()