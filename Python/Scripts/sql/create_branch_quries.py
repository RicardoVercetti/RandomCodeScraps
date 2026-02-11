from datetime import datetime

BRANCH_CODE_PREFIX = "B"
BRANCH_CODE_START = 2003


class BranchCodeGenerator:
    def __init__(self, start):
        self.counter = start

    def next(self):
        code = f"{BRANCH_CODE_PREFIX}{self.counter:04d}"
        self.counter += 1
        return code


branch_code_gen = BranchCodeGenerator(BRANCH_CODE_START)


def sql_value(val):
    if val is None:
        return "NULL"
    if isinstance(val, str):
        return "'" + val.replace("'", "''") + "'"
    if isinstance(val, datetime):
        return f"'{val.strftime('%Y-%m-%d %H:%M:%S')}'"
    return str(val)


def generate_insert_apt_branch(data: dict) -> str:
    columns = []
    values = []

    for column, value in data.items():
        columns.append(column)
        values.append(sql_value(value))

    return f"""
INSERT INTO public.apt_branch (
    {", ".join(columns)}
) VALUES (
    {", ".join(values)}
);
""".strip()


# ------------------ Example Usage ------------------

def create_branch_data():
    branch_data = {
        "branch_area": "MG Road",
        "branch_building_name": "Tech Park",
        "branch_code": branch_code_gen.next(),
        "branch_landmark": "Near Metro Station",
        "branch_name": "Bangalore Main Branch",
        "city": "Bangalore",
        "country": "India",
        "created_date": datetime.now(),
        "institution_nr": 109,
        "last_updated_date": datetime.now(),
        "pincode": "560001",
        "primary_contact_email_id": "contact@bank.com",
        "primary_contact_name": "Ravi Kumar",
        "primary_contact_number": "9876543210",
        "secondary_contact_email_id": None,
        "secondary_contact_name": None,
        "secondary_contact_number": None,
        "state": "Karnataka",
        "status": "ACTIVE",
        "last_updated_user": "SYSTEM"
    }
    return branch_data



# print(generate_insert_apt_branch(branch_data))
li = []
for i in range(1000):
    li.append(generate_insert_apt_branch(create_branch_data()))

for item in li:
    print(item)
