import os
import subprocess
from utils import current_date_time_for_filename

env = os.environ.copy()
env["PGPASSWORD"] = "postgres"      # password

## Other params
host: str = "localhost"
db: str = "New_Alka_Artha_DCMS"
port: str = "5432"
username: str = "postgres"
# password: str = "postgres"
filename: str = "Artha_backup_" + current_date_time_for_filename() + ".sql"



def backup_of_existing_db():
    # take a single .sql file of postgres DB

    result = subprocess.run(["pg_dump", "-U", username, "-h", host, "-p", port, db], text=True, capture_output=True, env=env)

    if result.returncode == 0:
        with open(f"./db/{filename}", "w") as file:
            file.write(result.stdout)
        print(f"backed up the data into {result.stderr} successfully...")
    else:
        print(f"error occured while creating the backup")
        print(result.stderr)
    

def main():
    print("Script initiated...")
    
    try:
        backup_of_existing_db()
    except FileNotFoundError as e:
        print(e)

if __name__ == "__main__":
    main()