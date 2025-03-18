package com.ricardo.constants;

import com.ricardo.utils.JsonPrettyPrinter;

public abstract class AbstractReward implements Reward {
    String rewardName;
    String rewardHash;
    String rewardData;
    int pointsRequired;
    int expiryPeriodInDays;


    public String getRewardName() {
        return rewardName;
    }
    public void setRewardName(String rewardName) {
        this.rewardName = rewardName;
    }
    public String getRewardHash() {
        return rewardHash;
    }
    public void setRewardHash(String rewardHash) {
        this.rewardHash = rewardHash;
    }
    public String getRewardData() {
        return rewardData;
    }
    public void setRewardData(String rewardData) {
        this.rewardData = rewardData;
    }
    @Override
    public int getPointsRequired() {
        return pointsRequired;
    }
    @Override
    public void setPointsRequired(int pointsRequired) {
        this.pointsRequired = pointsRequired;
    }
    @Override
    public int getExpiryPeriodInDays() {
        return expiryPeriodInDays;
    }
    @Override
    public void setExpiryPeriodInDays(int expiryPeriodInDays) {
        this.expiryPeriodInDays = expiryPeriodInDays;
    }

    public abstract String getRewardId();


    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }
}
