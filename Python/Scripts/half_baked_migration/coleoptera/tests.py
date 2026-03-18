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

def range_of_headers():
    x = [s for s in range(1, 35)]       # this couldv'e been `x = list(range(1, 35))`
    print(",".join(map(str, x)))


def sets_in_one_column():
    """
    5th → card status, 
    6th → custom state, 
    8th → hold response code
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
            card_status_sets.add(line.split(",")[21])
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
        for line in cards_file:
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
    
#     insert_query = """
#     INSERT INTO public.apt_cards
# (institution_nr, branch_id, card_no, masked_card_no, seq_no, product_id, limits, barcode_no, 
# card_type, hold_rsp_code, issuance_date, activation_date, track2_value, pvv_or_pin_offset, 
# validation_data_question, validation_data, cardholder_rsp_info, discretionary_data, 
# cvv1, cvv2, icvv, dcvv, expiry_date, chip_product_code, physical_product_id, financial_type, 
# last_updated_date, last_updated_user, expiry_day, from_date, from_day, 
# trnsn_cntr, no_more_renewal, card_cancel_reason, restrict_emv_txn_without_pin, 
# restrict_onl_cntctless_txn, restrict_offline_txn, restrict_ecommerce_txn, card_status_id, 
# enable_domestic_atm_transaction, enable_domestic_pos_transaction, enable_domestic_cnp_transaction, 
# enable_domestic_contactless_transaction, enable_international_atm_transaction, 
# enable_international_pos_transaction, enable_international_cnp_transaction, 
# enable_international_contactless_transaction, insta_kit_id, insta_batch_id, pin_retry_count, 
# pin_entry_date, cvv2_retry_count, last_cvv2_try_date, inventory_id, card_identify, icvd, 
# generation_date, card_status_change_date, last_annual_fee_deducted_date, "comments")
# VALUES(0, 0, '', '', '', 0, 0, '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', '', CURRENT_TIMESTAMP, '', '', '', '', 0, '', '', false, true, false, true, 0, false, false, false, false, false, false, false, false, '', 0, 0, '', 0, '', 0, '', 0, '', '', '', '');
#     """


# insert_query_for_one_row()
# all_columns_with_null()
sets_in_one_column()