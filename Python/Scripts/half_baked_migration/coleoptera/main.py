"""
Main file that is the entry point of the migration CLI logic
"""

import traceback
from src.utils.logger_setup import logger
from src.utils.utils import get_and_classify_files
from src.sanitycheck.sanity_check import sanity_check_cards, sanity_check_accounts, sanity_check_for_customers, ProgressStatus

def main() -> None:
    logger.info("Hello from coleoptera!")
    logger.info("here is the first line of logging...")
    classified: dict[str, list[str]] = get_and_classify_files()


    # todo: #1 check if the config already exists, else do the config run
    
    # todo: #2 else do the migration run 

    status_lists: list[ProgressStatus] = []

    logger.info("Classified items:")
    for index, (key, values) in enumerate(classified.items()):
        logger.info("%d %s : %s", index + 1, key, values)

        if key == "cards":
            progress = sanity_check_cards(values)
            status_lists.append(progress)
        elif key == "accounts":
            progress = sanity_check_accounts(values)
            status_lists.append(progress)
        elif key == "customers":
            progress = sanity_check_for_customers(values)
            status_lists.append(progress)
        else:
            logger.info("skipping for %s", key)
    print("all finished!")


if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logger.error(traceback.format_exc())
        logger.error(e)
