from src.utils.logger_setup import logger
from src.utils.utils import get_and_classify_files
from src.sanitycheck.sanity_check import sanity_check_cards

def main():
    logger.info("Hello from coleoptera!")
    logger.info("here is the first line of logging...")
    classified: dict[str, list[str]] = get_and_classify_files()
    logger.info("Classified items:")
    for index, (key, values) in enumerate(classified.items()):
        logger.info(f"{index + 1}. {key} : {values}")

        if key == "cards":
            sanity_check_cards(values)
        else:
            print(f"skipping for {key}")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logger.error(e)
