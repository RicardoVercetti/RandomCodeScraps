def right_pad(string, length, char=' '):
    return string.ljust(length, char)

seriel_no = "16112"                 # len 5
separator_1 = "$"
CHN_1 = "1001 0000 0000 1013"         # 19 - a space between every 4 chareters
# seperator_1
embossing_name = "PRASAD M D"           # 25    (padding requred)
# seperator_1
valid_from_date = "0125"            # MMYY
# seperator_1
expiry_date = "0126"                # MMYY
# seperator_1
acct_no = "000000000684955"         # len 15
# seperator_1
cvv2 = "270"

SEGMENT_1 = seriel_no + separator_1 + CHN_1 + separator_1 + right_pad(embossing_name, 25) + separator_1 \
+ valid_from_date + separator_1 +expiry_date + separator_1 + acct_no +separator_1+cvv2

# TRACK - I

separator_2 = "\""
start_sentinel = "%"
format_charecter = "B"
CHN_2 = "1001000000001013"
common_field_1 = "^"
encoding_name = "PRASAD"            # len 25 (padding required)
common_field_2 = "/"
# common_field_1
expiry_period = "2601"              # YYMM
service_code = "101"
cvv1 = "005"
common_1 = "A"
common_20 = "00000000000000000000"
end_sentinel = "?"

SEGMENT_2 = separator_2 + start_sentinel + format_charecter + CHN_2 + common_field_1 \
+ right_pad(encoding_name, 25) + common_field_2 + common_field_1 + expiry_period \
+ service_code + cvv1 + common_1 + common_20 + end_sentinel


# TRACK - II

start_sentinel_1 = ";"
# CHN_2
common_field_3 = "="
# expiry_period
# service_code
# cvv1
common_10 = "0000000000"
# end_sentinel
common_field_4 = "+"
common_field_5 = "1"
separator_3 = "~"
iCvv = "346"
# separator_3
name = "Prasad M D"                       # len 81 (padding required)
# separator_3
address_1 = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"                      # len 45 (padding required)
# separator_3
address_2 = "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB"                      # len 45 (padding required)
# separator_3
address_3 = "CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC"                      # len 45 (padding required)
# separator_3
address_4 = "DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD"                      # len 45 (padding required)
# separator_3
address_5 = "795113"                      # len 45 (padding required)
# separator_3
phone_number = "9287410233"                   # len 20 (padding required)
# separator_3
# phone_number
# separator_3
# CHN_2
# separator_3
# acct_no
# separator_3
customer_id = "0100000551"
# separator_3
last_10_of_digit_chn = "0000001013"
# separator_3
branch_name = "APTBANK"         # len 30 (padding required)
common_field_6 = "&"

SEGMENT_3 = start_sentinel_1 + CHN_2 + common_field_3 + expiry_period + service_code \
+ cvv1 + common_10 + end_sentinel + common_field_4 + common_field_5 + separator_3 \
+ iCvv + separator_3 + right_pad(name, 81) + separator_3 + right_pad(address_1, 45) \
+ separator_3 +right_pad(address_2, 45) + separator_3 + right_pad(address_3, 45) \
+ separator_3 +right_pad(address_4, 45) + separator_3 + right_pad(address_5, 45) + separator_3 \
+ right_pad(phone_number, 20) + separator_3 + right_pad(phone_number, 20) \
+ separator_3 + CHN_2 + separator_3 + acct_no + separator_3 + customer_id \
+ separator_3 + last_10_of_digit_chn + separator_3 + right_pad(branch_name, 30) \
+ common_field_6



print(f"'{SEGMENT_1}{SEGMENT_2}{SEGMENT_3}'")
