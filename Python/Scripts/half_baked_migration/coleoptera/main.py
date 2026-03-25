"""
Main file that is the entry point of the migration CLI logic
"""

import traceback
from src.utils.logger_setup import logger
from src.utils.utils import get_and_classify_files
from src.sanitycheck.sanity_check import sanity_check_cards, sanity_check_accounts, sanity_check_for_customers

def main() -> None:
    logger.info("Hello from coleoptera!")
    logger.info("here is the first line of logging...")
    classified: dict[str, list[str]] = get_and_classify_files()
    logger.info("Classified items:")
    for index, (key, values) in enumerate(classified.items()):
        logger.info("%d %s : %s", index + 1, key, values)

        if key == "cards":
            progress = sanity_check_cards(values)
            # print(f"progress: {progress}")
            if progress.failed_items > 0:
                logger.info("there are %d errors found in the %s file", progress.failed_items, progress.source_name)
        elif key == "accounts":
            progress = sanity_check_accounts(values)

            if progress.failed_items > 0:
                logger.info("there are about %d errors found in the %s file", progress.failed_items, progress.source_name)
        elif key == "customers":
            progress = sanity_check_for_customers(values)

            if progress.failed_items > 0:
                logger.info(f"there are about {progress.failed_items} errors found in the {progress.source_name} file")
        else:
            logger.info("skipping for %s", key)


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logger.error(traceback.format_exc())
        logger.error(e)
