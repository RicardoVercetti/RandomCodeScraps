import pandas as pd

# Path to your Excel file
file_path = "/home/jehoniah/Downloads/UI_Bug_Tracker.xlsx"  # change this to your actual file name

# Read the Excel file
df = pd.read_excel(file_path, sheet_name=1)

# Display the column names to verify (optional)
print("Columns:", df.columns.tolist())

# Count the number of occurrences of each status
status_counts = df["Status"].value_counts()

# Print the results
print("\nNumber of items by status:")
print(status_counts)

# If you also want percentages:
print("\nPercentage breakdown:")
print(df["Status"].value_counts(normalize=True) * 100)
