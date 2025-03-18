package com.ricardo.models;

import com.ricardo.constants.RewardCategory;
import com.ricardo.constants.RuleRewardStatus;
import com.ricardo.constants.RewardType;
import com.ricardo.constants.RewardTriggerCondition;
import com.ricardo.utils.JsonPrettyPrinter;

public class LoyaltyReward {
    String rewardName;
    String rewardId;
    RewardType rewardType;
    RewardCategory rewardCategory;
    RewardTriggerCondition triggerCondition;
    int pointsRequired;
    RuleRewardStatus rewardStatus;
    int expiryPeriodInDays;

    public String getRewardName() {
        return rewardName;
    }
    public void setRewardName(String rewardName) {
        this.rewardName = rewardName;
    }
    public String getRewardId() {
        return rewardId;
    }
    public void setRewardId(String rewardId) {
        this.rewardId = rewardId;
    }
    public RewardType getRewardType() {
        return rewardType;
    }
    public void setRewardType(RewardType rewardType) {
        this.rewardType = rewardType;
    }
    public RewardCategory getRewardCategory() {
        return rewardCategory;
    }
    public void setRewardCategory(RewardCategory rewardCategory) {
        this.rewardCategory = rewardCategory;
    }
    public RewardTriggerCondition getTriggerCondition() {
        return triggerCondition;
    }
    public void setTriggerCondition(RewardTriggerCondition triggerCondition) {
        this.triggerCondition = triggerCondition;
    }
    public int getPointsRequired() {
        return pointsRequired;
    }
    public void setPointsRequired(int pointsRequired) {
        this.pointsRequired = pointsRequired;
    }
    public RuleRewardStatus getRewardStatus() {
        return rewardStatus;
    }
    public void setRewardStatus(RuleRewardStatus rewardStatus) {
        this.rewardStatus = rewardStatus;
    }
    public boolean isActive() {
        return this.rewardStatus == RuleRewardStatus.ACTIVE;
    }
    public boolean isAuto() {
        return this.triggerCondition == RewardTriggerCondition.AUTO;
    }
    public int getExpiryPeriodInDays() {
        return expiryPeriodInDays;
    }
    public void setExpiryPeriodInDays(int expiryPeriodInDays) {
        this.expiryPeriodInDays = expiryPeriodInDays;
    }

    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }
}
