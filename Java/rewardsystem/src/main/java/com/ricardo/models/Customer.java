package com.ricardo.models;

import com.ricardo.utils.JsonPrettyPrinter;

import java.util.ArrayList;

public class Customer {
    String name;
    String customerId;
    Product productUsed;
    int balance;
    int pointsEarned;
    ArrayList<RewardInstance> rewardsEarned = new ArrayList<>();

    public String getName() {
        return name;
    }
    public void setName(String name) {
        this.name = name;
    }
    public String getCustomerId() {
        return customerId;
    }
    public void setCustomerId(String customerId) {
        this.customerId = customerId;
    }
    public Product getProductUsed() {
        return productUsed;
    }
    public void setProductUsed(Product productUsed) {
        this.productUsed = productUsed;
    }
    public int getBalance() {
        return balance;
    }
    public void setBalance(int balance) {
        this.balance = balance;
    }
    public int getPointsEarned() {
        return pointsEarned;
    }
    public void setPointsEarned(int pointsEarned) {
        this.pointsEarned = pointsEarned;
    }
    public void addReward(RewardInstance reward) {
        this.rewardsEarned.add(reward);
    }
    public ArrayList<RewardInstance> getRewardsEarned() {
        return rewardsEarned;
    }

    @Override
    public String toString() {
        return JsonPrettyPrinter.jsonifyObject(this);
    }
}
