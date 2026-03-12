def args_tryout(*items):
    print(f"value: {items}")
    print(f"type: {type(items)}")
    for key, arg in enumerate(items):
        print(f"{key+1}: {arg}")

args_tryout(1234, "w4346", 4527)