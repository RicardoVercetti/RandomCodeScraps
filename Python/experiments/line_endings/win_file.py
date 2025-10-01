print("write win file")

data = bytes.fromhex("746578740D0A20696E0D0A77696E")

with open("win_txt.txt", "wb") as f:
    f.write(data)
    
print("file written successfully!")
