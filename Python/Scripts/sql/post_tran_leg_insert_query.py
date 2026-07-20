import csv
from datetime import datetime
from pathlib import Path

def generate_insert_statements(csv_file):
    all_queries = []
    with open(csv_file, 'r') as file:
        reader = csv.DictReader(file)
        
        for row in reader:
            # Handle NULL values
            columns = []
            values = []
            
            for col, val in row.items():
                # Clean column names - replace spaces and special characters
                clean_col = col.lower().replace(' ', '_').replace('-', '_')
                columns.append(clean_col)
                
                # Handle NULL values
                if val == '' or val == 'NULL':
                    values.append('NULL')
                else:
                    # Escape single quotes and handle date formats
                    if val.startswith('202') and (' ' in val or '-' in val):
                        # Handle date formats like "2025-12-27 09:14:32.000"
                        try:
                            # Try to parse as datetime
                            dt = datetime.strptime(val.split('.')[0], '%Y-%m-%d %H:%M:%S')
                            values.append(f"'{dt}'")
                        except:
                            values.append(f"'{val}'")
                    else:
                        # Handle numeric values
                        try:
                            float(val.replace(',', ''))
                            values.append(val)
                        except:
                            # Escape single quotes in text
                            escaped_val = val.replace("'", "''")
                            values.append(f"'{escaped_val}'")
            
            insert_sql = f"INSERT INTO apt_post_tran_leg ({', '.join(columns)}) VALUES ({', '.join(values)});"
            # print(insert_sql)
            all_queries.append(insert_sql)

    with open("all_insert_query.sql", "w") as file:
        for line in all_queries:
            file.write(line + "\n")

# Usage
current_path = Path.cwd()
print(f"current path: {current_path}")
generate_insert_statements('./pos_trans.csv')
