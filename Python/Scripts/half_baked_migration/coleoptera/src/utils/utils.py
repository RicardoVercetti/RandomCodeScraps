import os
from src.utils.logger_setup import logger # Use the logger instance you created

# load all files in the res folder
FOLDER_PATH: str = "./res"

def get_all_files_from_res() -> list[str]:
    """Gets all the files in the folder ./res and returns a list of all files"""
    if not os.path.exists(FOLDER_PATH):
        logger.error(f"Directory {FOLDER_PATH} does not exist.")
        return []
    return [f for f in os.listdir(FOLDER_PATH) if os.path.isfile(os.path.join(FOLDER_PATH, f))]

def classify_files(file_names: list[str], absolute_folder_path: str) -> dict[str, list[str]]:
    classified: dict[str, list[str]] = {}
    
    # Mapping of prefixes to their dictionary keys
    # Order matters: check longer/specific strings first
    prefixes = {
        "cardaccounts": "card_accounts",
        "customeraccounts": "customer_accounts",
        "cards": "cards",
        "customers": "customers",
        "accounts": "accounts"
    }

    for filename in file_names:
        matched = False
        full_path = os.path.join(absolute_folder_path, filename)
        
        for prefix, key in prefixes.items():
            if filename.startswith(prefix):
                if key not in classified:
                    classified[key] = []
                classified[key].append(full_path)
                matched = True
                break # Stop checking other prefixes once matched
        
        if not matched:
            logger.info(f"{filename} ignored (not classified)")

    return classified

def get_and_classify_files() -> dict[str, list[str]]:
    all_files = get_all_files_from_res()
    # Using os.path.abspath to ensure the path is solid
    return classify_files(all_files, os.path.abspath(FOLDER_PATH))

if __name__ == "__main__":
    print("Classified files test:")
    print(get_and_classify_files())