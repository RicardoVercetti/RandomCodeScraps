package com.ricardo.models;

import com.ricardo.constants.Reward;
import com.ricardo.utils.JsonPrettyPrinter;

import java.time.LocalDateTime;

/**
 * The reward instance is the actual reward that is given to the customer for points earned.<br>
 * {@code rewardedTime} can be set on the time of allocation to the customer.<br>
 * {@code rewardExpiry} can be set based on the period mentioned for {@code LoyaltyReward} from the time of allocation
 */
public class RewardInstance implements Reward {
    String rewardId;
    String rewardSource;    // Bag Of Rewards or Loyalty Reward instance ID
    LocalDateTime rewardedTime;
    LocalDateTime rewardExpiry;
    // should add reward specific data - Like?
    RewardType entityType;
    Reward entity;
    int pointsRequired;
    int expiryPeriodInDays;

    public String getRewardId() {
        return rewardId;
    }
    public void setRewardId(String rewardId) {
        this.rewardId = rewardId;
    }
    public String getRewardSource() {
        return rewardSource;
    }
    public void setRewardSource(String rewardSource) {
        this.rewardSource = rewardSource;
    }
    public LocalDateTime getRewardedTime() {
        return rewardedTime;
    }
    public void setRewardedTime(LocalDateTime rewardedTime) {
        this.rewardedTime = rewardedTime;
    }
    public LocalDateTime getRewardExpiry() {
        return rewardExpiry;
    }
    public void setRewardExpiry(LocalDateTime rewardExpiry) {
        this.rewardExpiry = rewardExpiry;
    }
    public RewardType getEntityType() {
        return entityType;
    }
    public void setEntityType(RewardType entityType) {
        this.entityType = entityType;
    }
    public Reward getEntity() {
        return entity;
    }
    public void setEntity(Reward entity) {
        this.entity = entity;
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
        return this.expiryPeriodInDays;
    }
    @Override
    public void setExpiryPeriodInDays(int expiryPeriodInDays) {
        this.expiryPeriodInDays = expiryPeriodInDays;
    }
    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }

    public enum RewardType {
        FROM_BAG,
        FROM_INSTANCE
    }
}
