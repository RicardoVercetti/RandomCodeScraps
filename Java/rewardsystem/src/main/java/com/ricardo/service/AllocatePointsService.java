package com.ricardo.service;

import com.ricardo.models.Customer;
import com.ricardo.models.LoyaltyProgram;

/**
 * Take the current transaction amount and Loyalty Program and allocate points based on {@code ACTIVE} Rules in a Loyalty Program.
 */
public class AllocatePointsService {
    public static void allocatePointsToCustomer(LoyaltyProgram lp, Customer customer, int amountOfTransaction) {
        // do the deed
    }
}
