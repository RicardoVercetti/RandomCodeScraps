from cryptography.fernet import Fernet

key = Fernet.generate_key()
print(f"Key: {key.decode()}")

print("key type: ", type(key))

fer = Fernet(key)
data = b'A really secret data'
token = fer.encrypt(data)
print("token: ", token)

decrypted = fer.decrypt(token)
print("decrpt: ", decrypted)
print("type decrypt: ", type(decrypted))
print("in hex: " + " ".join(f'{b:02x}' for b in decrypted))