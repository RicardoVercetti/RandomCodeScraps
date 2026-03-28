import psycopg2
from psycopg2 import sql
import traceback
# Configuration
DB_CONFIG = {
    "dbname": "test_new_restored_artha",
    "user": "postgres",
    "password": "postgres",
    "host": "localhost"
}

SAFE_TABLES = [
    "apt_account_types", "apt_api_auth", "apt_card_delivery_type", "apt_card_status",
    "apt_card_status_reason", "apt_city", "apt_country", "apt_currency",
    "apt_customer_status", "apt_customer_type", "apt_fee_service", "apt_financial_type",
    "apt_levels", "apt_limit_types", "apt_logo_codes", "apt_modules",
    "apt_notification_types", "apt_permissions", "apt_role_lvl", "apt_servicecode",
    "apt_states", "apt_system_configuration", "apt_town", "apt_transaction_def", "apt_usage_types"
]

PRESERVE_FILTERS = {
    "apt_role_module_permissions": {"col": "role_id", "vals": (1, 14)},
    "apt_roles": {"col": "role_id", "vals": (1, 14)},
    "apt_users": {"col": "user_id", "vals": (1, 14)},
}

def clean_database():
    try:
        conn = psycopg2.connect(**DB_CONFIG)
        cur = conn.cursor()

        # Get all tables as clean strings
        cur.execute("SELECT table_name FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE'")
        all_tables = [row[0] for row in cur.fetchall()]

        # print(",".join(all_tables))
        # return

        # Disable triggers/constraints for this session
        cur.execute("SET session_replication_role = 'replica';")

        # --- PASS 1: TRUNCATE UNKNOWN TABLES ---
        # We do this first so CASCADE doesn't wipe our preserved data later
        for table in all_tables:
            if table not in SAFE_TABLES and table not in PRESERVE_FILTERS:
                # print(f"type of table: {type(table)}")
                print(f"Truncating {table}...")
                cur.execute(sql.SQL("TRUNCATE TABLE {tbl} CASCADE").format(tbl=sql.Identifier(table)))

        # --- PASS 2: SURGICAL DELETE ON FILTERED TABLES ---
        for table, cfg in PRESERVE_FILTERS.items():             # todo: this is kinda not required. delete all values and manually insert the required entries
            if table in all_tables:
                print(f"Cleaning {table}: Keeping {cfg['col']} in {cfg['vals']}")
                # We use NOT IN with the values provided
                query = sql.SQL("DELETE FROM {tbl} WHERE {col} NOT IN %s").format(
                    tbl=sql.Identifier(table),
                    col=sql.Identifier(cfg['col'])
                )
                cur.execute(query, (cfg['vals'],))
                
                # Reset sequences for these tables specifically
                # Note: userz_user_id_seq is the name from your schema
                seq_query = sql.SQL("SELECT setval(pg_get_serial_sequence(%s, %s), COALESCE((SELECT MAX({col}) FROM {tbl}), 1))").format(
                    col=sql.Identifier(cfg['col']),
                    tbl=sql.Identifier(table)
                )
                cur.execute(seq_query, (table, cfg['col']))

        cur.execute("SET session_replication_role = 'origin';")
        conn.commit()
        print("\nSuccess: Database cleaned and seed data preserved.")

    except Exception as e:
        if conn: conn.rollback()
        print(f"Error: {e}")
        print(f"{traceback.format_exc()}")
    finally:
        if cur: cur.close()
        if conn: conn.close()

if __name__ == "__main__":
    if input("Type 'yes' to wipe data: ").lower() == 'yes':
        clean_database()