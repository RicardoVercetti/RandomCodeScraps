package com.ricardo.models.rewards;

import com.ricardo.constants.AbstractReward;

public class FreebieReward extends AbstractReward {

    @Override
    public String getRewardId() {
        return super.getRewardHash();
    }
}
