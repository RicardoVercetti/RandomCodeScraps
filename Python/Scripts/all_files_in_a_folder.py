import os

def get_all_files_from_folder(folder_path):
    # List all files in the folder (excluding directories)
    file_names = [f for f in os.listdir(folder_path) if os.path.isfile(os.path.join(folder_path, f))]
    return file_names

def all_files_in_folder_with_format(folder_path: str, format:str) -> list[str]:
    file_names = [f for f in os.listdir(folder_path) if os.path.isfile(os.path.join(folder_path, f)) and f.endswith(format)]
    return file_names
