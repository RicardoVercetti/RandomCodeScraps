# logger_setup.py
import logging
from datetime import datetime
import os

os.makedirs("logs", exist_ok=True)

log_file = f"logs/script_log{datetime.now().strftime('%Y-%m-%d_%H-%M-%S')}.log"

logger = logging.getLogger()
logger.setLevel(logging.INFO)

# file handler
file_handler = logging.FileHandler(log_file)
file_handler.setFormatter(logging.Formatter(
    "%(asctime)s [%(levelname)s] %(message)s"
))


# console handler
console_handler = logging.StreamHandler()
console_handler.setFormatter(logging.Formatter(
    "%(asctime)s [%(levelname)s] %(message)s"
))
logger.addHandler(file_handler)
logger.addHandler(console_handler)

# logger.info(f"logging started at: {datetime.now()}")