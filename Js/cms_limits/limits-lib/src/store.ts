import { Card, Account, Institution, CardAccounts } from "./entities";
import { LimitProfileDef, LimitProfile, LimitLevelProfileDef, LimitLevelProfile } from "./limits";

// treat this as synthetic database

export const limitProfilesDefs: LimitProfileDef[] = [];
export const limitProfiles: LimitProfile[] = [];
export const limitLevelProfileDefs: LimitLevelProfileDef[] = [];
export const limitLevelProfiles: LimitLevelProfile[] = [];

export const institutionsStore: Institution[] = [];
export const cardsStore: Card[] = [];
export const accountsStore: Account[] = [];
export const cardAccountsStore: CardAccounts[] = [];
