
def clean(value):
    if value is None or value == "":
        return "NULL"
    return f"'{value}'"

def parse_row(line: str):
    """
    Converts a CSV line like:
    "1195","10","356",,,,"0",,"2025-06-21 19:26:35","SP"

    into a clean Python list
    """
    parts = line.strip().split(",")

    cleaned = []
    for p in parts:
        p = p.strip()

        # remove surrounding quotes
        if p.startswith('"') and p.endswith('"'):
            p = p[1:-1]

        # convert empty to None
        if p == "":
            p = None

        cleaned.append(p)

    return cleaned

def generate_insert_accounts_query(row, institution_nr, branch_id):
    """
    row: list of values from CSV
    institution_nr: int
    branch_id: int
    """

    

    account_id = row[0]
    account_type = row[1]
    currency_code = row[2]
    hold_rsp_code = row[3]
    account_product = row[4]
    extended_fields = row[5]
    overdraft_limit = row[6]
    # row[7] = account_nickname (not used in DB)
    last_updated_date = row[8]
    last_updated_user = row[9]

    query = f"""
    INSERT INTO public.apt_accounts (
        institution_nr,
        branch_id,
        account_id,
        account_type,
        currency_code,
        hold_rsp_code,
        extended_fields,
        overdraft_limit,
        account_product,
        last_updated_date,
        last_updated_user
    ) VALUES (
        {institution_nr},
        {branch_id},
        {clean(account_id)},
        {clean(account_type)},
        {clean(currency_code)},
        {clean(hold_rsp_code)},
        {clean(extended_fields)},
        {clean(overdraft_limit)},
        {clean(account_product)},
        {clean(last_updated_date)},
        {clean(last_updated_user)}
    );
    """

    return query.strip()

def test_accounts_row():
    institution_nr = 110
    branch_id = 7061

    with open("./res/accounts.txt", "r") as accounts_file:
        for line in accounts_file:
            if not line.strip():
                continue  # skip empty lines

            row = parse_row(line)

            # safety check (optional but useful)
            if len(row) < 10:
                print(f"Skipping invalid row: {row}")
                continue

            query = generate_insert_accounts_query(row, institution_nr, branch_id)
            print(query)
            break

if __name__ == "__main__":
    test_accounts_row()