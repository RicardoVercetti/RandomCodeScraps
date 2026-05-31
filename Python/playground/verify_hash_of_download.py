import hashlib
from pathlib import Path
import requests

def verify_sha256_sidecar(binary_path, sidecar_path=None) -> (bool, str, str):
    """
    Verify a binary against its .sha256 sidecar file.
    
    Args:
        binary_path: Path to the binary file
        sidecar_path: Path to the .sha256 file (optional, auto-detected)
    
    Returns:
        bool: True if hash matches, False otherwise
    """
    binary_path = Path(binary_path)
    
    # Auto-detect sidecar if not specified
    if sidecar_path is None:
        sidecar_path = binary_path.with_suffix(binary_path.suffix + '.sha256')
    else:
        sidecar_path = Path(sidecar_path)
    
    # Read expected hash from sidecar
    with open(sidecar_path, 'r') as f:
        content = f.read().strip()
    
    # Handle common formats:
    # Format 1: "hash  filename"
    # Format 2: "hash"  (just the hash)
    expected_hash = content.split()[0] if ' ' in content else content
    
    # Calculate actual hash of binary
    sha256_hash = hashlib.sha256()
    with open(binary_path, 'rb') as f:
        # Read in chunks for large files
        for chunk in iter(lambda: f.read(4096), b''):
            sha256_hash.update(chunk)
    
    actual_hash = sha256_hash.hexdigest()
    
    return (actual_hash == expected_hash, expected_hash, actual_hash)

def verify_hash(binary_path: Path, hash_path: Path):
    # Usage
    if verify_sha256_sidecar(binary_path=binary_path, sidecar_path=hash_path):
        print("✓ Hash matches - file is valid")
    else:
        print("✗ Hash mismatch - file may be corrupted or tampered")

def download_file(filename: str, url: str):
    filepath = Path(filename)
    if filepath.exists():
        raise FileExistsError(f"file with the file name: {filename} already exists, please give a new name")

    response = requests.get(url)

    with open(filepath, "wb") as dfile:
        dfile.write(response.content)

    # success action doesn't prolly need any logs or whatsoever?

def get_file_and_checksum() -> (str, str):
    filename = "uv-i686-unknown-linux-gnu.tar.gz"
    checksum_name = "uv-i686-unknown-linux-gnu.tar.gz.sha256"
    file_url = "https://releases.astral.sh/github/uv/releases/download/0.11.17/uv-i686-unknown-linux-gnu.tar.gz"
    checksum_url = "https://releases.astral.sh/github/uv/releases/download/0.11.17/uv-i686-unknown-linux-gnu.tar.gz.sha256"

    file_path = Path(filename)
    checksum_path = Path(checksum_name)

    if not file_path.exists():
        download_file(filename=filename, url=file_url)
    else:
        print(f"file: {file_path} already exists, skipping download")

    if not checksum_path.exists():
        download_file(filename=checksum_name, url=checksum_url)
    else:
        print(f"file: {checksum_path} already exists, skipping download")

    print(f"downloaded {filename} and {checksum_name}")
    return (filename, checksum_name)

def main():
    file_name, checksum_name = get_file_and_checksum()

    is_verified, expected_hash, actual_hash = verify_sha256_sidecar(file_name, checksum_name)

    if is_verified:
        print("hashes matched!")
    else:
        print("hashes didn't match...")

    print(f"expected hash: {expected_hash}")
    print(f"actual hash: {actual_hash}")

    print(f"type of expected hash: {type(expected_hash)}")
    print(f"type of actual hash: {type(actual_hash)}")

if __name__ == "__main__":
    main()
