package com.ricardo.models;

import java.util.ArrayList;
import com.ricardo.utils.JsonPrettyPrinter;

public class LoyaltyProgram {
    String loyaltyProgramName;
    String loyaltyProgramId;
    ArrayList<LoyaltyRule> rulesAdded = new ArrayList<>();
    ArrayList<LoyaltyReward> rewardsAdded = new ArrayList<>();

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

    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }
}
