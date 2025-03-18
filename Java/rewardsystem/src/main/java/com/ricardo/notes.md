## Questions

- Q: How do you tell if points has already been rewarded/not rewarded for a specific transaction or a set of past transactions?
  - A: Don't have to, add points while doing a transaction, and don't mind about if teh past transactions has been rewarded properly or not
- Q: How do you trigger a reward addition point? during the transaction? or post transaction(like in gpay)?
  - A: During transaction is okay. Don't go too specific.
 


## Dev Notes
- The Loyalty program is already linked to a product(Card program), there is no need to have `PRODUCT_RULE` in the `RuleType`.
- `LoyaltyReward` is not an Implementation of `Reward` while `RewardInstace` and all the `bagOfRewards` are.