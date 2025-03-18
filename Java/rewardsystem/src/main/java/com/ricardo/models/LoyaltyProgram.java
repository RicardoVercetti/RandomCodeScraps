package com.ricardo.models;

import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;

import com.ricardo.constants.Reward;
import com.ricardo.constants.RuleRewardStatus;
import com.ricardo.utils.JsonPrettyPrinter;

/**
 * <li>This is the Main Loyalty Program object model</li>
 * <li>A loyalty Program can have many {@code LoyaltyRule} and {@code LoyaltyReward} associated with it.</li>
 * <li>Then there are {@code bagOfRewards}. This is for rewarding the customers with a finite 3rd party coupons/vouchers.</li>
 */
public class LoyaltyProgram {
    String loyaltyProgramName;
    String loyaltyProgramId;
    ArrayList<LoyaltyRule> rulesAdded = new ArrayList<>();
    ArrayList<LoyaltyReward> rewardsAdded = new ArrayList<>();
    List<Reward> bagOfRewards = new ArrayList<>();

    public String getLoyaltyProgramName() {
        return loyaltyProgramName;
    }
    public void setLoyaltyProgramName(String loyaltyProgramName) {
        this.loyaltyProgramName = loyaltyProgramName;
    }
    public String getLoyaltyProgramId() {
        return loyaltyProgramId;
    }
    public void setLoyaltyProgramId(String loyaltyProgramId) {
        this.loyaltyProgramId = loyaltyProgramId;
    }
    public ArrayList<LoyaltyRule> getRulesAdded() {
        return rulesAdded;
    }
    public void setRulesAdded(ArrayList<LoyaltyRule> rulesAdded) {
        this.rulesAdded = rulesAdded;
    }
    public ArrayList<LoyaltyReward> getRewardsAdded() {
        return rewardsAdded;
    }
    public void setRewardsAdded(ArrayList<LoyaltyReward> rewardsAdded) {
        this.rewardsAdded = rewardsAdded;
    }
    public void addRule(LoyaltyRule rule) {
        rulesAdded.add(rule);
    }
    public void addReward(LoyaltyReward reward) {
        rewardsAdded.add(reward);
    }
    public void addRewardIntoBagOfRewards(Reward reward) {
        bagOfRewards.add(reward);
    }
    public Optional<LoyaltyRule> getRuleByName(String ruleName) {
        for(LoyaltyRule rule: rulesAdded) {
            if(rule.getRuleName().equals(ruleName)) return Optional.of(rule);
        }
        return Optional.empty();
    }
    public Optional<LoyaltyRule> getRuleById(String id) {
        // the concise way
        return rulesAdded.stream()
                .filter(rule -> rule.getRuleId().equals(id))
                .findFirst();
    }
    public Optional<List<LoyaltyRule>> getAllActiveRules() {
        List<LoyaltyRule> activeRules = rulesAdded.stream()
                .filter(rule -> rule.getRuleStatus() == RuleRewardStatus.ACTIVE)
                .collect(Collectors.toList());

        return activeRules.isEmpty() ? Optional.empty() : Optional.of(activeRules);
    }
    public Optional<List<LoyaltyReward>> getAllActiveRewards() {
        List<LoyaltyReward> activeRewards = rewardsAdded.stream()
                .filter(reward -> reward.getRewardStatus() == RuleRewardStatus.ACTIVE)
                .collect(Collectors.toList());
        return activeRewards.isEmpty() ? Optional.empty() : Optional.of(activeRewards);
    }
    public Optional<List<LoyaltyReward>> getAllActiveAutoRewards() {
        List<LoyaltyReward> activeAutoRewards = rewardsAdded.stream()
                .filter(reward -> (reward.isActive() && reward.isAuto()))
                .collect(Collectors.toList());
        return activeAutoRewards.isEmpty() ? Optional.empty() : Optional.of(activeAutoRewards);
    }
    public List<Reward> getBagOfRewards() {
        return bagOfRewards;
    }
    public void setBagOfRewards(List<Reward> bagOfRewards) {
        this.bagOfRewards = bagOfRewards;
    }

    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }
}
