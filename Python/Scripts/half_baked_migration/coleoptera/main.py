import traceback
from src.utils.logger_setup import logger
from src.utils.utils import get_and_classify_files
from src.sanitycheck.sanity_check import (
    sanity_check_cards, 
    sanity_check_accounts, 
    sanity_check_for_customers, 
    ProgressStatus
)
# Assuming you move the config logic into a dedicated helper or src/config.py
from src.config import Config, CardProgram

def generate_phase1_config(status_lists: list[ProgressStatus], config_path: str):
    """
    Orchestrates the creation of config.json. 
    Includes all detected CardProduct/BIN pairs for manual user mapping.
    """
    conf = Config(filename=config_path)
    
    for status in status_lists:
        # Filter out corrupt files for the migration list
        clean_files = [m.filename for m in status.filemetadata if not m.is_corrupt]
        
        if status.source_name == "cards":
            conf.add_card_files(clean_files)
            
            # We add EVERY unique pair found.
            for cp in status.card_product_list:
                # Even if cp.is_dulplicated is True, we add it.
                # This allows the user to see the conflict and map ranges manually.
                conf.add_card_program(CardProgram(
                    name=cp.name,
                    bin=cp.bin,
                    id="",           
                    min_range="",    
                    max_range=""     
                ))
        
        elif status.source_name == "accounts":
            conf.accounts_file.extend(clean_files)
        elif status.source_name == "customers":
            conf.customers_file.extend(clean_files)

    conf.store_in_file()
    logger.info(f"--- Phase 1 Config generated with {len(conf.card_programs_list)} programs at: {config_path} ---")

def main() -> None:
    logger.info("Hello from coleoptera! Starting Phase 1: Analysis.")
    classified: dict[str, list[str]] = get_and_classify_files()

    status_lists: list[ProgressStatus] = []

    # 1. Run Sanity Checks
    for key, values in classified.items():
        if key == "cards":
            status_lists.append(sanity_check_cards(values))
        elif key == "accounts":
            status_lists.append(sanity_check_accounts(values))
        elif key == "customers":
            status_lists.append(sanity_check_for_customers(values))
        else:
            logger.info("Skipping analysis for %s", key)

    # 2. Call the Config Generation Function
    # This is the step that creates your JSON for manual editing
    try:
        generate_phase1_config(status_lists, "config.json")
    except Exception as e:
        logger.error(f"Failed to generate config.json: {e}")

    # 3. Final Report (The multi-line string logic we discussed)
    # ... [Insert the report building logic here] ...
    print("\nPhase 1 Complete. Check your logs and config.json.")

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logger.error(traceback.format_exc())