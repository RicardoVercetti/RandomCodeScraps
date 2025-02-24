from itertools import permutations
from hsm_connection import FutureX, current_time
import re

# variables = []
variables = [
    "000000000085",  # AMOUNT
    "000000000000",  # AMOUNT_OTHER
    "0356",          # TERMINAL_COUNTRY_CODE
    "0880048800",    # TERMINAL_VERIFICATION_RESULT
    "0356",          # TRANSACTION_CURRENCY_CODE
    "250220",        # TRANSACTION_DATE
    "01",            # TRANSACTION_TYPE
    "533748FF",      # UNPREDICTABLE_NUMBER
    "3800",          # APPLICATION_INTERCHANGE_PROFILE
    "002E",          # ATC
    "0105200003400000"  # ISSUER_APPLICATION_DATA
]



# start
print(current_time())

def make_command_data(conca):
    return f"AOEMVA;FS0;KM1;KP5C2D605E4990BFA518FF3162D215BA4C;KQ1001000000001013;KR01;KS002E;KT{conca};BO3884A577D1210698;NP0;"

def getConcat():
    return "00000000008500000000000003560880048800035625022001533748FF3800002E0105200003400000"

def strip_response(resp):
    return resp[10:13]

def main():
    client = FutureX()
    # client.log_all = True
    i=0
    for perm in permutations(variables, len(variables)):
        print(f"\rProgress: {i} iters", end="", flush=True)
        val = "".join(perm)
        # Send a message
        # client.send(make_command_data(getConcat()))
        client.send(make_command_data(val))
        
        # Optionally, receive a response
        response = client.receive()
        # if response:
        #     print(f"Striped : '{strip_response(response)}'")

        # if(i>10):
        #     break
        i+=1
        # print("just response :" , response)
        response = re.sub(r"[\x00-\x1F]", "", response)

        if not response.startswith("[AOEMVA;BB"):
            print(f"\nSomething wrong with the response string! DEBUG: '{repr(response)}'")
            # print("\nSomething wrong with the response string!")

        if(response == "BBY"):
            print("Match found!")
            print("val:", val)
    

    # Close the connection
    client.close()



main()