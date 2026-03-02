import { Card, Account, Institution, CardAccounts } from "./entities";
import { LimitProfileDef, LimitProfile, LimitLevelProfileDef, LimitLevelProfile } from "./limits";

// treat this as synthetic database

export const limitProfilesDefsStore: LimitProfileDef[] = [];
export const limitProfilesStore: LimitProfile[] = [];
export const limitLevelProfileDefsStore: LimitLevelProfileDef[] = [];
export const limitLevelProfilesStore: LimitLevelProfile[] = [];

export const institutionsStore: Institution[] = [];
export const cardsStore: Card[] = [];
export const accountsStore: Account[] = [];
export const cardAccountsStore: CardAccounts[] = [];
