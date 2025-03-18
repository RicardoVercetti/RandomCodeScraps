package com.ricardo;

import com.ricardo.constants.*;
import com.ricardo.models.*;
import com.ricardo.models.rewards.CashBackReward;
import com.ricardo.models.rewards.CouponReward;
import com.ricardo.service.AutoRewardsService;
import com.ricardo.utils.JsonPrettyPrinter;
import com.ricardo.utils.Sysout;

import java.util.ArrayList;

public class Main {
    public static void instantiateDataForMainTesting() {
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
        rule.setRuleStatus(RuleRewardStatus.ACTIVE);

        // create reward #1
        LoyaltyReward reward1 = new LoyaltyReward();
        reward1.setRewardName("Discount Reward");
        reward1.setRewardId("RW1");
        reward1.setRewardType(RewardType.CHOICE);
        reward1.setRewardCategory(RewardCategory.MERCHANT_REWARDS);
        reward1.setTriggerCondition(RewardTriggerCondition.MANUAL);
        reward1.setPointsRequired(10);
        reward1.setRewardStatus(RuleRewardStatus.ACTIVE);

        // create reward #2
        LoyaltyReward reward2 = new LoyaltyReward();
        reward2.setRewardName("CashBack reward");
        reward2.setRewardId("RW2");
        reward2.setRewardType(RewardType.FIXED);
        reward2.setRewardCategory(RewardCategory.CASHBACK);
        reward2.setTriggerCondition(RewardTriggerCondition.AUTO);
        reward2.setPointsRequired(20);
        reward2.setRewardStatus(RuleRewardStatus.INACTIVE);

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

    public static void testWithBagOfRewards() {
        ArrayList<Reward> bagOfRewards = new ArrayList<>();

        AbstractReward cashback = new CashBackReward();
        cashback.setRewardName("Cashback reward A");
        cashback.setRewardHash("#FRGEE");
        cashback.setRewardData("Here is a fixed cashback of 2.Rs");
        cashback.setPointsRequired(10);
        bagOfRewards.add(cashback);

        AbstractReward coupon1 = new CouponReward();
        coupon1.setRewardName("Coupon reward A");
        coupon1.setRewardHash("#DJKAJO");
        coupon1.setRewardData("Here is a coupon for 10% discount on EasyBuy");
        coupon1.setPointsRequired(5);
        bagOfRewards.add(coupon1);

        AbstractReward coupon2 = new CouponReward();
        coupon2.setRewardName("Coupon reward B");
        coupon2.setRewardHash("#FEWGHRH");
        coupon2.setRewardData("Here is a coupon for 5% discount on EasyBuy");
        coupon2.setPointsRequired(15);
        bagOfRewards.add(coupon2);

        System.out.println(bagOfRewards);
    }

    public static void testAutoRewardService() {
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
        rule.setRuleStatus(RuleRewardStatus.ACTIVE);

        // create reward #1
        LoyaltyReward reward1 = new LoyaltyReward();
        reward1.setRewardName("Discount Reward");
        reward1.setRewardId("RW1");
        reward1.setRewardType(RewardType.CHOICE);
        reward1.setRewardCategory(RewardCategory.MERCHANT_REWARDS);
        reward1.setTriggerCondition(RewardTriggerCondition.MANUAL);
        reward1.setPointsRequired(10);
        reward1.setRewardStatus(RuleRewardStatus.ACTIVE);
        reward1.setExpiryPeriodInDays(20);

        // create reward #2
        LoyaltyReward reward2 = new LoyaltyReward();
        reward2.setRewardName("CashBack reward");
        reward2.setRewardId("RW2");
        reward2.setRewardType(RewardType.FIXED);
        reward2.setRewardCategory(RewardCategory.CASHBACK);
        reward2.setTriggerCondition(RewardTriggerCondition.AUTO);
        reward2.setPointsRequired(20);
        reward2.setRewardStatus(RuleRewardStatus.ACTIVE);
        reward2.setExpiryPeriodInDays(15);

        // create Loyalty Program #1
        LoyaltyProgram program = new LoyaltyProgram();
        program.setLoyaltyProgramName("Gold Program");
        program.setLoyaltyProgramId("GP123");

        // add rules and rewards to the loyalty program
        program.addRule(rule);
        program.addReward(reward1);
        program.addReward(reward2);

        // create customer
        Customer cus = new Customer();
        cus.setName("Huzain M V");
        cus.setCustomerId("#CUS123");
        cus.setBalance(1000);
        cus.setPointsEarned(30);
//        cus.setProductUsed();

        // Auto Rewards Service

        System.out.println("Customer data : \n" +cus.toString());

        AutoRewardsService.allocateAutoRewardsToCustomer(program, cus);

        Sysout.print(program.toString());
        System.out.println("Customer data : \n" +cus.toString());
    }

    public static void testCustomerData() {
        Customer cus = new Customer();
        cus.setName("Huzain M V");
        cus.setCustomerId("#CUS123");
        cus.setBalance(1000);
        cus.setPointsEarned(30);
        System.out.println("Customer data : \n" +cus.toString());
//        System.out.println("Rewards earned size: " + cus.getRewardsEarned().size());


    }

    public static void main(String[] args) {
        System.out.println(" --- Starting the reward system --- ");

//        testWithBagOfRewards();
//        instantiateDataForMainTesting();
        testAutoRewardService();
//        testCustomerData();

    }
}