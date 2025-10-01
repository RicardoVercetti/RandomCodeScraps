print("here is something")

data = bytes.fromhex("746578740A20696E0A6C696E7578")

with open("lin_file.txt", "wb") as f:
    f.write(data)
    
print("wrote successfully!")

