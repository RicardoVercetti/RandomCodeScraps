import { Account, Card, CardAccounts, createCardProduct, createInstitution } from "./src/entities";
import { createLimitLevelProfile, createLimitLevelProfileDef, createLimitProfile, createLimitProfileDef } from "./src/limits";
import { accountsStore, cardAccountsStore, cardsStore } from "./src/store";

console.log("started the main code...");
// Instructions:
// 1. limit_profile_def and limit_profile defines the limit values by an ID
// 2. limit_level_profile_def and limit_level_profile defines the order by which to consider the limit - Card, account, customer → all may have different limits, 
// limit_level_profile is configured in the card_account level. 


// main entities
const institution = createInstitution();
const institution2 = createInstitution();

// limit profile definitions
const limitProfile = createLimitProfile();
const limitProfileDef = createLimitProfileDef(limitProfile.limit_profile_id, institution.institution_id);

// limit level definitions
const limitLevel = createLimitLevelProfile({use_limits_of: "CARD"});
const limitLevelProfile = createLimitLevelProfileDef(limitLevel.limit_level_profile_id);

const cardProduct1 = createCardProduct("123456", limitProfile.limit_profile_id);

// main entities
console.log(`institution: ${JSON.stringify(institution, null, 2)}`);

console.log(`limit profile: ${JSON.stringify(limitProfile, null, 2)}`);
console.log(`Limit profile def: ${JSON.stringify(limitProfileDef, null, 2)}`);
console.log(`Institution 2: ${JSON.stringify(institution2, null, 2)}`);
console.log(`Limit level profile: ${JSON.stringify(limitLevelProfile, null, 2)}`);


// case to prove: when there are multiple limits configured at different level, pick the one thats configured by a limit level profile
// when a card is linked to a customer - that's when the limit checking comes into play

// Institution, Card Product, Card, Account, Customer - all may have limit profile ID values
// CardAccount - will have limit level profile id saying which one to take
// when the 


const card1 = new Card(institution, cardProduct1);
const card2 = new Card(institution, cardProduct1);
const card3 = new Card(institution, cardProduct1);
// console.log(`card1: ${JSON.stringify(card1, null, 2)}`);
// console.log(`card2: ${JSON.stringify(card2, null, 2)}`);
// console.log(`card3: ${JSON.stringify(card3, null, 2)}`);

const account1 = new Account(institution);
const account2 = new Account(institution);

const cardAccount1 = new CardAccounts(card1, account1);
const cardAccount2 = new CardAccounts(card2, account2);

console.log(`all cards: ${JSON.stringify(cardsStore, null, 2)}`);
console.log(`all accounts: ${JSON.stringify(accountsStore, null, 2)}`);
console.log(`all card-accounts: ${JSON.stringify(cardAccountsStore, null, 2)}`);
