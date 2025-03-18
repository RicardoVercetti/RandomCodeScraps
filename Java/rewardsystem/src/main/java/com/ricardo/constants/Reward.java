package com.ricardo.constants;

public interface Reward {
    int getPointsRequired();  // Abstract method instead of a constant
    void setPointsRequired(int pointsRequired);
    int getExpiryPeriodInDays();
    void setExpiryPeriodInDays(int expiryPeriodInDays);

}
