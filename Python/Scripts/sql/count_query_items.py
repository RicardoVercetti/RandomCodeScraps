import re

with open("filename.txt", "r", encoding="utf-8") as f:
        query = f.read()


cleaned = (
    query.replace("\n", " ")  # remove newlines
         .replace("\t", " ")  # remove tabs
         .replace("[", "")
         .replace("]", "")
         .strip()
)

# remove the leading "select" keyword if it exists
if cleaned.lower().startswith("select "):
    cleaned = cleaned[7:]

# --- CUT OFF AFTER FIRST ' from ' or ' left join ' (case insensitive) ---
# This ensures we only count the comma-separated fields in the SELECT list
cutoff_pattern = re.compile(r"\b(from|left\s+join|right\s+join|inner\s+join)\b", re.IGNORECASE)
match = cutoff_pattern.search(cleaned)
if match:
    cleaned = cleaned[:match.start()]

# --- SPLIT AND COUNT ---
items = [item.strip() for item in cleaned.split(",") if item.strip()]
print(f"Number of selected items: {len(items)}")

# optionally print them all for verification
# for i, col in enumerate(items, 1):
#     print(f"{i:3}. {col}")

print(f"last few : {"|".join(items[-5:])}")


