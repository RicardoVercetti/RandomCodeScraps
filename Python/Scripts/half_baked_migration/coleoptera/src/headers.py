# order is as follows
# 1. cards
# 2. accounts
# 3. customers
# 4. card-accounts
# 5. customer-accounts

# cards
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

# accounts
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

# customers
customer_headers = [
    "customer_id",
    "national_id",

    "c1_title",
    "c1_first_name",
    "c1_initials",
    "c1_last_name",
    "c1_name_on_card",

    "c2_title",
    "c2_first_name",
    "c2_initials",
    "c2_last_name",
    "c2_name_on_card",

    "c3_title",
    "c3_first_name",
    "c3_initials",
    "c3_last_name",
    "c3_name_on_card",

    "telephone_number",
    "mobile_telephone_number",
    "fax_number",
    "email_address",

    "postal_address_1",
    "postal_address_2",
    "postal_city",
    "postal_region",
    "postal_code",
    "postal_country",

    "other_address_1",
    "other_address_2",
    "other_city",
    "other_region",
    "other_postal_code",
    "other_country",

    "date_of_birth",
    "company_name",
    "preferred_language",

    "vip_flag",
    "vip_lapse_date",

    "extended_fields",

    "last_updated_date",
    "last_updated_user"
]