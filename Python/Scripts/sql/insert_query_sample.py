
def create_query(name: str, location: str) -> str:
    return f"insert into my_cities(name, location) values('{name}', '{location}');"

def write_in_file(content_list: list[str]):
    with open("sample_insert_query.sql", "w", encoding="utf8") as file:
        for line in content_list:
            file.writelines(f"{line}\n")

def make_name_and_location():
    name = "panama"
    location = "nowhere"
    times = 141001
    queries = []
    for _ in range(100000):
        query = create_query(f"{name}{times+1}", f"{location}{times+1}")
        times += 1
        queries.append(query)
    write_in_file(queries)
    print(f"times at the end: {times}")



if __name__ == "__main__":
    make_name_and_location()
