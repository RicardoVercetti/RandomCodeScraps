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

- do a sanity check and verify if the format is correct for the datatype
- create a config file for the data that need to be present 