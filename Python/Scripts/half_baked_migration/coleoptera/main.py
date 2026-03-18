from src.logger_setup import logging
from src.utils import get_and_classify_files

def main():
    logging.info("Hello from coleoptera!")
    logging.info("here is the first line of logging...")
    classified: dict[str, list[str]] = get_and_classify_files()
    logging.info("Classified items:")
    for index, (key, value) in enumerate(classified.items()):
        logging.info(f"{index + 1}. {key} : {value}")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logging.error(e)
