import requests

HASH = "https://releases.astral.sh/github/uv/releases/download/0.11.17/uv-aarch64-unknown-linux-gnu.tar.gz.sha256"

response = requests.get(HASH)

with open("filename.sha256", "wb") as file:
    file.write(response.content)
    
print("Download complete!")