package com.ricardo.service;

import java.time.LocalDateTime;
import java.util.Collections;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import com.ricardo.constants.AbstractReward;
import com.ricardo.constants.Reward;
import com.ricardo.models.Customer;
import com.ricardo.models.LoyaltyProgram;
import com.ricardo.models.LoyaltyReward;
import com.ricardo.models.RewardInstance;
import com.ricardo.utils.Sysout;

/**
 * This should take in a customer's available points, Loyalty program's rules and rewards
 * and automatically put rewards to customers based on the available points to the customer
 *
 * <ul>
     *  <li> Any rule or reward that has {@code RuleRewardStatus} set to {@code INACTIVE} have to be ignored </li>
     *  <li> The service should only pick rewards that have {@code RewardTriggerCondition} set to {@code AUTO} </li>
 * </ul>
 */
public class AutoRewardsService {
    public static void allocateAutoRewardsToCustomer(LoyaltyProgram lp, Customer customer) {
        // do the deed
        var optionalActiveAutoRewards = lp.getAllActiveAutoRewards();           // INACTIVE ones are excluded
        var bagOfRewards = lp.getBagOfRewards();
        if(optionalActiveAutoRewards.isEmpty() && bagOfRewards.isEmpty()) {
            Sysout.print("There are no active AUTO rewards and bag of rewards found for the loyalty Program : " + lp.getLoyaltyProgramName());
            return;
        }

        // here, there must be at-least one LoyaltyReward present
        var activeAutoRewards = optionalActiveAutoRewards.get();


        int availablePoints = customer.getPointsEarned();
        Sysout.print("The points available for the customer : "+customer.getName()+" is : " + availablePoints);

        // if bagOfRewards are available - give from there or find the best choice and create the Instance from the rewards
        var availableLoyaltyRewards = findAllEligibleLoyaltyRewardsForPoint(availablePoints, activeAutoRewards);
        var availableBagOfRewards = findAllEligibleBagOfRewardsForPoint(availablePoints, bagOfRewards);



        // create the RewardInstance and set it to customer and call it a day
        RewardInstance rw = null;

        if(availableLoyaltyRewards.isPresent() && availableBagOfRewards.isPresent()) {
            var combinedList = combineLists(availableLoyaltyRewards.get(), availableBagOfRewards.get());
            var oneReward = getRandomElement(combinedList);

            // could be a `Reward` object from bag of rewards or a LoyaltyReward from rewards added in LP
            if(oneReward instanceof LoyaltyReward) {
                rw = createLoyaltyRewardInstance(oneReward, rw);
            } else {    // must be a Reward instance
                rw = createBorRewardInstance(oneReward, rw);
            }

            // create reward instance

        } else if(availableBagOfRewards.isEmpty() && availableLoyaltyRewards.isPresent()) {
            var oneReward = getRandomElement(availableLoyaltyRewards.get());

            // create reward instance
            rw = createLoyaltyRewardInstance(oneReward, rw);


        } else if(availableLoyaltyRewards.isEmpty() && availableBagOfRewards.isPresent()) {
            var oneReward = getRandomElement(availableBagOfRewards.get());

            // create reward instance
            rw = createBorRewardInstance(oneReward, rw);

        } else {
            Sysout.print("No Available rewards for allocating...");
            return;
        }

//        if(rw == null) {
//            Sysout.print("Error, the RewardInstance cannot be null");
//            return;
//        }

        // assign the reward instance to customer and deduce the corresponding points
        Sysout.print("Adding reward : \n" + rw.toString());
        Sysout.print("of points : " + rw.getPointsRequired());

        // yay...!
        customer.addReward(rw);
        Sysout.print("Reward added successfully");

        customer.setPointsEarned(customer.getPointsEarned() - rw.getPointsRequired());
    }

    public static Optional<List<LoyaltyReward>> findAllEligibleLoyaltyRewardsForPoint(int point, List<LoyaltyReward> loyaltyRewards) {
        List<LoyaltyReward> eligibleRewards = loyaltyRewards.stream()
                .filter(reward -> reward.getPointsRequired() <= point)
                .collect(Collectors.toList());
        return eligibleRewards.isEmpty() ? Optional.empty() : Optional.of(eligibleRewards);
    }

    public static Optional<List<Reward>> findAllEligibleBagOfRewardsForPoint(int point, List<Reward> bagOfRewards) {
        List<Reward> eligibleBagOfRewards = bagOfRewards.stream()
                .filter(reward -> reward.getPointsRequired() <= point)
                .collect(Collectors.toList());
        return eligibleBagOfRewards.isEmpty() ? Optional.empty() : Optional.of(eligibleBagOfRewards);
    }

    public static <T> T getRandomElement(List<T> list) {
        if (list == null || list.isEmpty()) return null;
        Collections.shuffle(list);
        return list.get(0);
    }

    public static List<Object> combineLists(List<?> list1, List<?> list2) {
        return Stream.concat(list1.stream(), list2.stream())
                .collect(Collectors.toList());
    }

    public static RewardInstance createLoyaltyRewardInstance(Object oneReward, RewardInstance pointer) {
        pointer = new RewardInstance();
        pointer.setRewardId("#RW36547");   // random generated reward ID
        pointer.setRewardSource("LoyaltyReward");   // ID from bag of rewards or ID of Loyalty Reward Model
        pointer.setRewardedTime(LocalDateTime.now());
        pointer.setRewardExpiry(LocalDateTime.now().plusDays(((LoyaltyReward) oneReward).getExpiryPeriodInDays()));
        pointer.setEntityType(RewardInstance.RewardType.FROM_INSTANCE);
//                pointer.setEntity();   // this is null cuz this is not from bag of rewards
        pointer.setPointsRequired(((LoyaltyReward) oneReward).getPointsRequired());
        pointer.setExpiryPeriodInDays(((LoyaltyReward) oneReward).getExpiryPeriodInDays());
        return pointer;
    }

    public static RewardInstance createBorRewardInstance(Object oneReward, RewardInstance pointer) {
        pointer = new RewardInstance();
        pointer.setRewardId(((AbstractReward)oneReward).getRewardId());   // random generated reward ID
        pointer.setRewardSource("BOR");   // ID from bag of rewards or ID of Loyalty Reward Model
        pointer.setRewardedTime(LocalDateTime.now());
        pointer.setRewardExpiry(LocalDateTime.now().plusDays(((AbstractReward) oneReward).getExpiryPeriodInDays()));
        pointer.setEntityType(RewardInstance.RewardType.FROM_BAG);
        pointer.setEntity((Reward) oneReward);
        pointer.setPointsRequired(((AbstractReward) oneReward).getPointsRequired());
        pointer.setExpiryPeriodInDays(((AbstractReward) oneReward).getExpiryPeriodInDays());
        return pointer;
    }
}
