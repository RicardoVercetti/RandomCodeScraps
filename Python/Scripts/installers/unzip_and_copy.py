import zipfile
import shutil
import os
import sys

# Paths
zip_file = "/home/jehoniah/Desktop/lil-workspace/DCMS-V1.0.01.zip"  # Put your zip file in the same directory as the exe
extract_dir = "/home/jehoniah/Desktop/lil-workspace/extracted_temp"
destination_dir = "/home/jehoniah/Desktop/lil-workspace/destination_folder"  # Change to your destination path

def unzip_file():
    print(f"Unzipping {zip_file}...")
    with zipfile.ZipFile(zip_file, 'r') as zip_ref:
        zip_ref.extractall(extract_dir)
    print("Unzip complete.")

def copy_files():
    print(f"Copying files to {destination_dir}...")
    if not os.path.exists(destination_dir):
        os.makedirs(destination_dir)

    total_files = sum(len(files) for _, _, files in os.walk(extract_dir))
    copied = 0

    for root, dirs, files in os.walk(extract_dir):
        for file in files:
            src = os.path.join(root, file)
            rel_path = os.path.relpath(root, extract_dir)
            dest_folder = os.path.join(destination_dir, rel_path)
            os.makedirs(dest_folder, exist_ok=True)
            shutil.copy2(src, dest_folder)
            copied += 1
            sys.stdout.write(f"\rCopied {copied}/{total_files} files...")
            sys.stdout.flush()

    print("\nCopy complete.\n")

def cleanup():
    print("Cleaning up temporary files...")
    shutil.rmtree(extract_dir, ignore_errors=True)

if __name__ == "__main__":
    unzip_file()
    copy_files()
    cleanup()
    print("All done!")
    input("Press Enter to exit...")
