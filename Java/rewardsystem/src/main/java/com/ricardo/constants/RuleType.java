package com.ricardo.constants;

/**
 * the RuleType defines where is this rule applied,
 * MERCHANT_RULE     → on merchants available (Mer A, Mer B, etc.,)
 * PRODUCT_RULE      → on card products available (CP1, CP2)
 * MERCHANT_PRODUCT  → on the combo of merchant and product
 */
public enum RuleType {
    MERCHANT_RULE,
    PRODUCT_RULE,
    MERCHANT_PRODUCT,
    VELOCITY
}
