package com.ricardo;

import com.ricardo.constants.*;
import com.ricardo.models.LoyaltyProgram;
import com.ricardo.models.LoyaltyReward;
import com.ricardo.models.LoyaltyRule;
import com.ricardo.models.Merchant;
import com.ricardo.utils.JsonPrettyPrinter;

public class Main {
//    public static void instantiateData() {
//
//    }

    public static void main(String[] args) {
        System.out.println(" --- Starting the reward system --- ");

        // create merchant
        Merchant m1 = new Merchant("Merchant A", "MER123");

        // Create rule #1
        LoyaltyRule rule = new LoyaltyRule();
        rule.setRuleName("Rule One");
        rule.setRuleId("R1");
        rule.setRuleType(RuleType.MERCHANT_RULE);
        rule.addTargetToArray(m1);
        rule.setRuleCondition(RuleCondition.AMT_OF_TRANSACTION);
        rule.setRuleValue(2000);
        rule.setPointsAllocated(20);

        // create reward #1
        LoyaltyReward reward1 = new LoyaltyReward();
        reward1.setRewardName("Discount Reward");
        reward1.setRewardId("RW1");
        reward1.setRewardType(RewardType.CHOICE);
        reward1.setRewardCategory(RewardCategory.MERCHANT_REWARDS);
        reward1.setTriggerCondition(RewardTriggerCondition.MANUAL);
        reward1.setPointsRequired(10);

        // create reward #2
        LoyaltyReward reward2 = new LoyaltyReward();
        reward2.setRewardName("CashBack reward");
        reward2.setRewardId("RW2");
        reward2.setRewardType(RewardType.FIXED);
        reward2.setRewardCategory(RewardCategory.CASHBACK);
        reward2.setTriggerCondition(RewardTriggerCondition.AUTO);
        reward2.setPointsRequired(20);

        // create Loyalty Program #1
        LoyaltyProgram program = new LoyaltyProgram();
        program.setLoyaltyProgramName("Gold Program");
        program.setLoyaltyProgramId("GP123");

        // add rules and rewards to the loyalty program
        program.addRule(rule);
        program.addReward(reward1);
        program.addReward(reward2);

        System.out.println(JsonPrettyPrinter.prettyJson(program.toString()));
    }
}