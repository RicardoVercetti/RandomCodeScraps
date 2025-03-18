package com.ricardo.models.rewards;

import com.ricardo.constants.AbstractReward;

public class CashBackReward extends AbstractReward {

    @Override
    public String getRewardId() {
        return super.getRewardHash();
    }
}
