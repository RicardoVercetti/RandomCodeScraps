import os
from src.logger_setup import logging


# load all files in the res folder
folder_path: str = "./res"

def get_all_files_from_res() -> list[str]:
    """Gets all the files in the folder ./res and returns a list of all files"""
    files: list[str] = []
    for filename in os.listdir(folder_path):
        files.append(filename)
        # fullpath = os.path.join(folder_path, filename)
        # if os.path.isfile(fullpath) and filename.endswith(".txt"):
        #     print(fullpath)
    return files

def classify_files(file_names: list[str], folder_path: str) -> dict[str, list[str]]:
    classified: dict[str, list[str]] = {}
    for filename in file_names:
        if filename.startswith("cards"):
            # if exists in dict, push into the list
            if classified.get("cards") is not None:
                classified.get("cards").append(os.path.join(folder_path, filename))
                continue
            classified.setdefault("cards", [os.path.join(folder_path, filename)])

        elif filename.startswith("customers"):
            if classified.get("customers") is not None:
                classified.get("customer").append(os.path.join(folder_path, filename))
                continue
            classified.setdefault("customers", [os.path.join(folder_path, filename)])

        elif filename.startswith("accounts"):
            if classified.get("accounts") is not None:
                classified.get("accounts").append(os.path.join(folder_path, filename))
                continue
            classified.setdefault("accounts", [os.path.join(folder_path, filename)])

        elif filename.startswith("cardaccounts"):
            if classified.get("cardaccounts") is not None:
                classified.get("cardaccounts").append(os.path.join(folder_path, filename))
                continue
            classified.setdefault("cardaccounts", [os.path.join(folder_path, filename)])

        elif filename.startswith("customeraccounts"):
            if classified.get("customeraccounts") is not None:
                classified.get("customeraccounts").append(os.path.join(folder_path, filename))
                continue
            classified.setdefault("customeraccounts", [os.path.join(folder_path, filename)])
        
        else:
            logging.info(f"{filename} is ignored as its not classified")

    return classified

def get_and_classify_files() -> dict[str, list[str]]:
    all_files = get_all_files_from_res()
    return classify_files(all_files, os.getcwd() + "/res")

