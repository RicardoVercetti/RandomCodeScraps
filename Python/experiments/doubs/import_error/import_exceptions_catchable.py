from library import some_function

# seems like sure can catch the import error
# 

if __name__ == "__main__":
    print("ran this as main")
    some_function()
    try:
        from library import some_other_function
        some_other_function()
    except ImportError as e:
        print("import of some other function failed: ", e)