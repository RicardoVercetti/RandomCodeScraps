from datetime import datetime

def current_date_time_for_filename():
   now = datetime.now()
   return now.strftime("%Y%m%d_%H%M%S") 