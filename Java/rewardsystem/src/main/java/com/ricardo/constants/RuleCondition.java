package com.ricardo.constants;

/**
 * This could be a certain number of transactions done in a certain period or a certain sum of amount transaction done
 * in a certain period
 */
public enum RuleCondition {
    NO_OF_TRANSACTION,  // number of transactions done for/under the merchant/product
    AMT_OF_TRANSACTION  // amount of transactions done for/under the merchant/product
}
