def display_profile(**kwargs):
    for single in kwargs.items():
        # print(f"{key}: {value}")
        print(f"single value: {single}")
        print(f"type: {type(single)}")
    # items = kwargs.items()
    # print(type(items))
    # print(f"values: {items}")

display_profile(name="Alan", age=23, job="Mailman", city="Madagaskar")