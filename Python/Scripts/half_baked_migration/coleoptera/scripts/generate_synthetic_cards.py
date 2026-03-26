import random
import os
from datetime import datetime, timedelta

def generate_random_date():
    """Generates a random date string in YYYY-MM-DD HH:MM:SS format."""
    start_date = datetime(2023, 1, 1)
    random_days = random.randint(0, 1000)
    random_seconds = random.randint(0, 86400)
    res_date = start_date + timedelta(days=random_days, seconds=random_seconds)
    return res_date.strftime("%Y-%m-%d %H:%M:%S")

def create_synthetic_cards(filename: str, num_lines: int):
    bins = ["222975", "222911", "353611"]
    products = {
        "222975": "utk_MCard_platinum",
        "222911": "utk_Rupay_JCB_CLS",
        "353611": "Utkarsh_JCB"
    }

    print(f"Generating {num_lines} lines in {filename}...")

    progress_interval = 1_000_000  # adjust as needed

    with open(filename, "w") as f:
        for i in range(num_lines):
            chosen_bin = random.choice(bins)
            suffix = f"{i:010d}"[-10:]
            pan = f'"{chosen_bin}{suffix}"'

            seq = '"001"'
            prod = f'"{products[chosen_bin]}"'
            def_acc = '"00"'
            status = '"1"'
            custom_state = ''
            expiry = '"3005"'

            col7_21 = ',,,,,"4","8726",,,,,"0","0","CCC"'

            issued = f'"{generate_random_date()}"'
            activated = f'"{generate_random_date()}"'
            ref = f'"USFB{random.randint(100000, 999999)}"'

            branch = '""'
            cust_id = f'"{random.randint(1000000, 9999999)}"'
            pin_len = '""'
            ext_fields = '"17DOM_CNP11Y17DOM_ATM11Y17INT_CNP11N17DOM_POS11Y17INT_ATM11N17INT_POS11N17DOM_CON11N17INT_CON11N"'
            expiry_day = '""'
            from_date = '"2508"'
            from_day = '"16"'

            last_up_date = f'"{generate_random_date()}"'
            last_up_user = '"CIF_UPDATE"'

            line = f"{pan},{seq},{prod},{def_acc},{status},{custom_state},{expiry}{col7_21},{issued},{activated},{ref},{branch},{cust_id},{pin_len},{ext_fields},{expiry_day},{from_date},{from_day},,,{last_up_date},{last_up_user}\n"
            f.write(line)

            # ✅ Progress logging
            if (i + 1) % progress_interval == 0:
                percent = ((i + 1) / num_lines) * 100
                print(f"Progress: {i+1:,}/{num_lines:,} ({percent:.2f}%)")

if __name__ == "__main__":
    target_file = "res/cards_large_test.txt"
    os.makedirs("res", exist_ok=True)
    
    # Adjust N here (e.g., 100,000 or 1,000,000)
    count = 10_000_000
    
    start_time = datetime.now()
    create_synthetic_cards(target_file, count)
    end_time = datetime.now()
    
    print(f"Done! Time taken: {end_time - start_time}")