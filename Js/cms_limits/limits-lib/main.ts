import { createInstitution } from "./src/entities";
import { createLimitLevelProfile, createLimitLevelProfileDef, createLimitProfile, createLimitProfileDef } from "./src/limits";

console.log("started the main code...");
// Instructions:
// 1. limit_profile_def and limit_profile defines the limit values by an ID
// 2. limit_level_profile_def and limit_level_profile defines the order by which to consider the limit - Card, account, customer → all may have different limits, 
// limit_level_profile is configured in the card_account level. 


// main entites
const institution = createInstitution();
const institution2 = createInstitution();

// limit profile definitions
const limitProfile = createLimitProfile();
const limitProfileDef = createLimitProfileDef(limitProfile.limit_profile_id, institution.institution_id);

// limit level definitions
const limitLevel = createLimitLevelProfile({use_limits_of: "CARD"});
const LimitLevelProfile = createLimitLevelProfileDef(limitLevel.limit_level_profile_id);

// main entites
console.log(`institution: ${JSON.stringify(institution, null, 2)}`);

console.log(`limit profile: ${JSON.stringify(limitProfile, null, 2)}`);
console.log(`Limit profile def: ${JSON.stringify(limitProfileDef, null, 2)}`);
console.log(`Instititution 2: ${JSON.stringify(institution2, null, 2)}`);
