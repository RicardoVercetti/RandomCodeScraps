import openpyxl
# just a script that will extract the arrival time from the TSRs I've sent.

def extract_arrival_time_from_tsr(tsr_file_path):
    # Load the workbook
    wb = openpyxl.load_workbook(tsr_file_path)

    # Select the active sheet or specify a sheet by name
    sheet = wb.active  # Or use wb['SheetName'] if you want to select a specific sheet

    # To get data from a specific cell
    data = sheet['D2'].value

    return data


if __name__ == '__main__':
    print(extract_arrival_time_from_tsr('/home/jehoniah/Documents/TSRs/ActivityLog_05_03_2025.xlsx'))
