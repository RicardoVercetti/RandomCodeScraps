package com.ricardo.models;

import com.ricardo.constants.RuleTargetType;

public class Merchant implements RuleTargetType {
    String merchantName;
    String merchantId;

    public Merchant() {
    }

    public Merchant(String merchantName, String merchantId) {
        this.merchantName = merchantName;
        this.merchantId = merchantId;
    }

    public String getMerchantName() {
        return merchantName;
    }
    public void setMerchantName(String merchantName) {
        this.merchantName = merchantName;
    }
    public String getMerchantId() {
        return merchantId;
    }
    public void setMerchantId(String merchantId) {
        this.merchantId = merchantId;
    }
}
