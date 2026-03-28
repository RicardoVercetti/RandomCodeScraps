# Coleoptera - the mover beetle

## Scope

A half baked python scipt for migrating the card, account, customer, card_accounts, account_customers into the DCMS database.

## Steps:
1. Sanity check for all formats supported
2. create config file post sanity check
3. verify database for the default dependency data

## Formats supported
1. `cards.txt`
2. `accounts.txt`
3. `customers.txt`
4. `cardaccounts.txt`
5. `customeraccounts.txt`
6. `cardoverridelimits.txt`
7. `accountoverridelimits.txt`

## Approach

- do a static file sanity check and verify if the format is correct for the datatype
    - cards
    - accounts
    - customers
    - card-accounts
    - account-customers
- create a config file for the data that need to be present 
    - Insitiution nr
    - file lists under category
- do a database cross verification check before insertion with the data in config file
    - instituion_nr
    - card product names
    - bin nubmers used
    - product_code check befro
- Enter the loop and start insertion
    - any failed insertions will not block for remaining items
    - log only failed item details and SQL error messages

## todo:
- [ ] separation of sanity check run and migration run
- [X] For duplicated product name but different BIN; sort by bin number and let the user input the product_id from DCMS though the config.json
- [/] duplicated bins should have product codes configured. That will be cross verified with DCMS database.
- [X] Eligible files post sanity check to be added in config.json
- [/] Test run one large file with synthetic data
- [X] buffer in CLI on the file being processed per line(steps)
- [ ] number of cards per product/bin/product_code combo, just in console
- [X] database backup script
- [/] python script for database clean script
- [X] single script to spin up a test database for migation candidate(restore script w/ manual deletion)
- [ ] simplify the resource files to 2-3 card products
- [ ] do manual config.json changes
- [ ] 2nd sanity check with live DB connection
- [ ] try insertion script for cards
- [ ] try inseriton script for accounts
- [ ] try insertion script for customers
- [ ] try insertion script for customer-accounts
- [ ] try inseriton script for card-accounts