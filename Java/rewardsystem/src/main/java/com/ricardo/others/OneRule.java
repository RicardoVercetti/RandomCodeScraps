package com.ricardo.others;

import com.ricardo.models.LoyaltyProgram;
import org.jeasy.rules.annotation.Action;
import org.jeasy.rules.annotation.Condition;
import org.jeasy.rules.annotation.Rule;

@Rule
public class OneRule {
    LoyaltyProgram lp;

    public OneRule(LoyaltyProgram lp) {
        this.lp = lp;
    }

    @Condition
    public boolean checkCondition() {
        // check a certain condition and have actions based on that condition

        return false;
    }

    @Action
    public void rewardSomething() {
        // add automatic rewards and put them in the customer's bucket

        // manual rewards are picked by customer

    }
}
