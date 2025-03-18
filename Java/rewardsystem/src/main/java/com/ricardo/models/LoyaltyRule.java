package com.ricardo.models;

import java.util.ArrayList;

import com.ricardo.constants.RuleCondition;
import com.ricardo.constants.RuleRewardStatus;
import com.ricardo.constants.RuleTargetType;
import com.ricardo.constants.RuleType;
import com.ricardo.utils.JsonPrettyPrinter;

/**
 * The {@code LoyaltyRule} defines the condition for allocating points to transactions done
 * <ul>
 *     <li>{@code RuleType} defines if the rule have to be applied when the transaction is done to a certain merchant or by a certain product</li>
 *     <li>{@code targetArray} can have the target entities in an array that the {@code RuleType} defines </li>
 *     <li>{@code RuleCondition} is the actual condition to apply this rule</li>
 *     <li>{@code ruleValue} is used for the condition defined</li>
 * </ul>
 */
public class LoyaltyRule {
    String ruleName;
    String ruleId;
    RuleType ruleType;
    ArrayList<RuleTargetType> targetArray = new ArrayList<>();
    RuleCondition ruleCondition;
    int ruleValue;
    int pointsAllocated;
    RuleRewardStatus ruleStatus;

    public String getRuleName() {
        return ruleName;
    }
    public void setRuleName(String ruleName) {
        this.ruleName = ruleName;
    }
    public String getRuleId() {
        return ruleId;
    }
    public void setRuleId(String ruleId) {
        this.ruleId = ruleId;
    }
    public RuleType getRuleType() {
        return ruleType;
    }
    public void setRuleType(RuleType ruleType) {
        this.ruleType = ruleType;
    }
    public ArrayList<RuleTargetType> getTargetArray() {
        return targetArray;
    }
    public void setTargetArray(ArrayList<RuleTargetType> targetType) {
        this.targetArray = targetType;
    }
    public void addTargetToArray(RuleTargetType target) {
        if(this.ruleType == RuleType.MERCHANT_PRODUCT) {
            this.targetArray.add(target);
            return;
        }

        if(this.ruleType == RuleType.MERCHANT_RULE && target instanceof Merchant) {
            this.targetArray.add(target);
            return;
        }

        if(this.ruleType == RuleType.PRODUCT_RULE && target instanceof  Product) {
            this.targetArray.add(target);
            return;
        }

        System.out.println("For the Rule type "+ ruleType + ", the corresponding target have to be passed");
    }
    public int getPointsAllocated() {
        return pointsAllocated;
    }
    public void setPointsAllocated(int pointsAllocated) {
        this.pointsAllocated = pointsAllocated;
    }
    public RuleCondition getRuleCondition() {
        return ruleCondition;
    }
    public void setRuleCondition(RuleCondition ruleCondition) {
        this.ruleCondition = ruleCondition;
    }
    public int getRuleValue() {
        return ruleValue;
    }
    public void setRuleValue(int ruleValue) {
        this.ruleValue = ruleValue;
    }
    public RuleRewardStatus getRuleStatus() {
        return ruleStatus;
    }
    public void setRuleStatus(RuleRewardStatus ruleStatus) {
        this.ruleStatus = ruleStatus;
    }

    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }
}
