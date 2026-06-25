cached: dict[str, str] = dict()

def fetch_card_status_id(this_id: str) -> str:
    print(f"fetching ID for: {this_id}")
    return "a"

def get_card_status_id(input_id: str) -> str:
    if cached.get(input_id) is not None:
        print(f"::: using the cached version for: {input_id}")
        return cached.get(input_id)

    fetched_id = fetch_card_status_id(input_id)

    cached[input_id] = fetched_id
    # whenever adding new item, gotta check if the limit is reached, if yes, remove the oldest item
    return fetched_id
    
input_ids = ["11", "22", "99", "00", "11", "11", "99"]


for id in input_ids:
    fetched_id_is = get_card_status_id(id)
    print(f"fetched id for {id} is {fetched_id_is}")
