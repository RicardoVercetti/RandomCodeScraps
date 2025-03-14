package com.ricardo;

import com.ricardo.constants.RuleTargetType;
import com.ricardo.constants.RuleType;
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
        rule.setPointsAllocated(20);

        // create reward #1
        LoyaltyReward reward = new LoyaltyReward();
        reward.setRewardName("Discount Reward");
        reward.setRewardId("RW1");


        // create LP #1
        LoyaltyProgram program = new LoyaltyProgram();
        program.setLoyaltyProgramName("Gold Program");
        program.setLoyaltyProgramId("GP123");

        // add rules and rewards to the loyalty program
        program.addRule(rule);
        program.addReward(reward);

        System.out.println(JsonPrettyPrinter.prettyJson(program.toString()));
    }
}