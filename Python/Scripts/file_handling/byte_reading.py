import time

with open("/home/jehoniah/Documents/temp/keystore.p12", "rb", buffering=0) as file:
    # for line in file:
    #     print(line.hex())
        # print(line)
    print(type(file))
    methods = [method for method in dir(file) if callable(getattr(file, method))]
    print(methods)

    # Problems
    # 1. should be able to pause the reading and continue from wherever we left off.
    # 2. when the reading is interrupted, have to close the open things before exiting.
    # 3. while reading from a file as buffer, should know when this is changed by some other process or means

    n = 0
    has_readable = True
    while(has_readable):
        print("iter: ", n)
        byte_array = bytearray(60)
        # print(type(byte_array))
        ret_value = file.readinto(byte_array)
        if ret_value == 0:
            has_readable = False
            break
        print(f"bytes: {byte_array[:ret_value].hex()}, ret: {ret_value}, length when striped: {len(byte_array[:ret_value])}")
        n+=1
        time.sleep(0.1)

    print("existed while loop..")

