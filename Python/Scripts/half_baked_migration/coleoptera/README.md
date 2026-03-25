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
- [ ] For duplicated product name but different BIN; sort by bin number and let the user input the product_id from DCMS though the config.json 
- [ ] Eligible files post sanity check to be added in config.json