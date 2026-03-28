import os
import subprocess
from dataclasses import dataclass
# from src.utils.logger_setup import logger

@dataclass
class RestoreDbData:
    host: str
    port: str
    username: str
    new_db_name: str
    backup_file_path: str

def restore_database(data: RestoreDbData):
    """Creates a new database and restores data from a .sql file."""

    env = os.environ.copy()
    if not env["PGPASSWORD"]:
        raise ValueError("password env not set!")
    
    if not os.path.exists(data.backup_file_path):
        print(f"Backup file not found: {data.backup_file_path}")
        return

    # 1. Create the new Database
    # We connect to the default 'postgres' db to execute the CREATE command
    print(f"Creating new database: {data.new_db_name}...")
    create_db_cmd = ["createdb", "-h", data.host, "-p", data.port, "-U", data.username, data.new_db_name]
    
    create_res = subprocess.run(create_db_cmd, env=env, capture_output=True, text=True)
    
    if create_res.returncode != 0:
        # If database already exists, we might want to continue or stop
        if "already exists" in create_res.stderr:
            print(f"Database {data.new_db_name} already exists. Attempting to restore anyway.")
            # print(f"{create_res.stderr}")
        else:
            print(f"Failed to create database: {create_res.stderr}")
            return

    # 2. Restore the SQL file using psql
    # We use < (stdin) redirection via the 'input' parameter in subprocess.run
    print(f"Restoring data from {data.backup_file_path} into {data.new_db_name}...")
    
    restore_cmd = ["psql", "-h", data.host, "-p", data.port, "-U", data.username, "-d", data.new_db_name]
    
    try:
        with open(data.backup_file_path, "r") as sql_file:
            # We pass the file content directly to the psql command
            restore_res = subprocess.run(
                restore_cmd, 
                env=env, 
                stdin=sql_file, 
                capture_output=True, 
                text=True
            )

        if restore_res.returncode == 0:
            print(f"✅ Successfully restored {data.new_db_name} from backup.")
        else:
            print(f"❌ Restore failed: {restore_res.stderr}")
            
    except Exception as e:
        print(f"An unexpected error occurred during restore: {e}")

def main():
    print("Restore script initiated...")
    restore_database(RestoreDbData(
        "localhost", 
        "5432", 
        "postgres", 
        "test_new_restored_artha", 
        "/home/jehoniah/Documents/repos/RandomCodeScraps/Python/Scripts/half_baked_migration/coleoptera/db/Artha_demo_backup_20260327_163724.sql"
        )
    )

if __name__ == "__main__":
    # Configuration
    env = os.environ.copy()
    env["PGPASSWORD"] = "postgres"
    main()