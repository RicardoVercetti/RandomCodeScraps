# src/utils/logger_setup.py
import logging
from datetime import datetime
import os

# ANSI Escape Codes for Terminal Colors
RED = "\033[31m"
RESET = "\033[0m"
BOLD = "\033[1m"

class ColorConsoleFormatter(logging.Formatter):
    """Custom formatter to inject ANSI colors into the console output."""
    
    FORMAT = "%(asctime)s [%(levelname)s] %(message)s"

    def format(self, record):
        log_fmt = self.FORMAT
        
        # If the level is ERROR or CRITICAL, wrap the whole line in red
        if record.levelno >= logging.ERROR:
            log_fmt = f"{RED}{BOLD}{self.FORMAT}{RESET}"
        elif record.levelno == logging.WARNING:
            log_fmt = f"\033[33m{self.FORMAT}{RESET}" # Yellow for warnings
            
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)

# Setup directories
os.makedirs("logs", exist_ok=True)
log_file = f"logs/script_log_{datetime.now().strftime('%Y-%m-%d_%H-%M-%S')}.log"

logger = logging.getLogger()
logger.setLevel(logging.INFO)

# --- File Handler (Plain Text - No Colors) ---
file_handler = logging.FileHandler(log_file)
file_handler.setFormatter(logging.Formatter(
    "%(asctime)s [%(levelname)s] %(message)s"
))

# --- Console Handler (Colored Output) ---
console_handler = logging.StreamHandler()
console_handler.setFormatter(ColorConsoleFormatter())

logger.addHandler(file_handler)
logger.addHandler(console_handler)