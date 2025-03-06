from all_files_in_a_folder import all_files_in_folder_with_format
from data_extraction_from_excel_script import extract_arrival_time_from_tsr

folder_name = '/home/jehoniah/Documents/TSRs'
list_of_files = all_files_in_folder_with_format(folder_name, ".xlsx")
list_of_time_pairs = []

for file_name in list_of_files:
    arrival_time = extract_arrival_time_from_tsr(folder_name + "/" + file_name)
    list_of_time_pairs.append((file_name, arrival_time))
    print(f"File: {file_name}, Arrival Time: {arrival_time}")
# print(len(list_of_files))