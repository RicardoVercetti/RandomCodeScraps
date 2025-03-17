package com.ricardo.others;

import org.jeasy.rules.annotation.Action;
import org.jeasy.rules.annotation.Condition;
import org.jeasy.rules.annotation.Rule;

@Rule(name = "Spending Rule", description = "Give points based on spending amount")
public class SpendingRule {

    private double amountSpent;
    private int points;

    public SpendingRule(double amountSpent) {
        this.amountSpent = amountSpent;
    }

    @Condition
    public boolean checkCondition() {
        return amountSpent >= 100; // Apply rule if spending is >= 100
    }

    @Action
    public void addPoints() {
        if (amountSpent >= 500) {
            points = 50;
        } else if (amountSpent >= 100) {
            points = 10;
        }
        System.out.println("Customer earned " + points + " points!");
    }
}
