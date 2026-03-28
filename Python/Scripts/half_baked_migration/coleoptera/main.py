# main.py
import traceback
from src.utils.logger_setup import logger
from src.utils.utils import get_and_classify_files
from src.sanitycheck.sanity_check import (
    sanity_check_cards, 
    sanity_check_accounts, 
    sanity_check_for_customers, 
    ProgressStatus
)
from src.config import Config, CardProgram

def generate_phase1_config(status_lists: list[ProgressStatus], config_path: str):
    conf = Config(filename=config_path)
    
    for status in status_lists:
        # Create the list of dicts with path and line count
        clean_file_data = [
            {"path": m.filename, "lines": m.no_of_items} 
            for m in status.filemetadata if not m.is_corrupt
        ]
        
        if status.source_name == "cards":
            conf.cards_file = clean_file_data
            for cp in status.card_product_list:
                conf.add_card_program(CardProgram(
                    name=cp.name,
                    bin=cp.bin,
                    product_code=cp.product_code # Pass it here
                ))
        
        elif status.source_name == "accounts":
            conf.accounts_file = clean_file_data
            
        elif status.source_name == "customers":
            conf.customers_file = clean_file_data
            
    conf.store_in_file()

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
        logger.error(traceback.format_exc())

    list_card_products_sort_by_bin(status_lists)
    list_card_products_sort_by_name(status_lists)

    # 3. Final Report (The multi-line string logic we discussed)
    print("\nPhase 1 Complete. Check your logs and config.json.")

def list_card_products_sort_by_bin(status_lists: list[ProgressStatus]):
    """Final Report: List all Card Products sorted by BIN and Product Code."""
    print("\n" + "="*85)
    print(f"{'DETECTED CARD PRODUCTS (Sorted by BIN)':^85}")
    print("="*85)

    all_detected_products = []
    for status in status_lists:
        if status.source_name == "cards":
            all_detected_products.extend(status.card_product_list)

    # Sort by BIN first, then Product Code
    sorted_by_bin = sorted(all_detected_products, key=lambda p: (p.bin, p.product_code))

    if not sorted_by_bin:
        print("No card products detected.")
    else:
        print(f"{'BIN':<10} | {'CODE':<6} | {'PRODUCT NAME':<40} | {'CONFLICT'}")
        print("-" * 85)
        for cp in sorted_by_bin:
            conflict_tag = "⚠️ [!] DUPLICATE" if cp.is_dulplicated else "✅ OK"
            print(f"{cp.bin:<10} | {cp.product_code:<6} | {cp.name:<40} | {conflict_tag}")

    print("="*85)

def list_card_products_sort_by_name(status_lists: list[ProgressStatus]):
    """Final Report: List all Card Products sorted by Product Name."""
    print("\n" + "="*85)
    print(f"{'DETECTED CARD PRODUCTS (Sorted by NAME)':^85}")
    print("="*85)

    all_detected_products = []
    for status in status_lists:
        if status.source_name == "cards":
            all_detected_products.extend(status.card_product_list)

    # Sort by Name (case-insensitive), then BIN, then Code
    sorted_by_name = sorted(all_detected_products, key=lambda p: (p.name.lower(), p.bin, p.product_code))

    if not sorted_by_name:
        print("No card products detected.")
    else:
        print(f"{'PRODUCT NAME':<40} | {'BIN':<10} | {'CODE':<6} | {'CONFLICT'}")
        print("-" * 85)
        for cp in sorted_by_name:
            conflict_tag = "⚠️ [!] DUPLICATE" if cp.is_dulplicated else "✅ OK"
            print(f"{cp.name:<40} | {cp.bin:<10} | {cp.product_code:<6} | {conflict_tag}")

    print("="*85)

if __name__ == "__main__":
    try:
        main()
    except Exception as e:
        logger.error(traceback.format_exc())