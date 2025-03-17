package com.ricardo.models;

import com.ricardo.constants.RuleTargetType;

/**
 * Represents a card program in the CMS context
 */
public class Product implements RuleTargetType {
    String productName;
    String productId;

    public String getProductName() {
        return productName;
    }
    public void setProductName(String productName) {
        this.productName = productName;
    }
    public String getProductId() {
        return productId;
    }
    public void setProductId(String productId) {
        this.productId = productId;
    }
}
