import psycopg2
from cryptography.fernet import Fernet

# connect to postgres
conn = psycopg2.connect(
    host="localhost",
    database="Alka_Artha_DCMS",
    user="postgres",
    password="postgres",
    port=5432
)

cursor = conn.cursor()

cursor.execute("SELECT * from apt_institutions")

rows = cursor.fetchall()

print(f"rows: {rows}")

for institution_nr, institution_name in rows:
    print(f"{institution_nr}. {institution_name}")

print("runs...")