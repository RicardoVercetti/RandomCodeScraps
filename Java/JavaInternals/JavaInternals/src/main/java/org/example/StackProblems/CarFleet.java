package org.example.StackProblems;

import java.util.Arrays;
import java.util.ArrayList;
import java.util.Collections;

public class CarFleet {
    // Problem statement:
    // There are n cars traveling to the same destination on a one-lane highway.
    //
    // You are given two arrays of integers position and speed, both of length n.
    //
    //    position[i] is the position of the ith car (in miles)
    //    speed[i] is the speed of the ith car (in miles per hour)
    //
    // The destination is at position target miles.
    // A car can not pass another car ahead of it. It can only catch up to another car and then drive at the same speed as the car ahead of it.
    // A car fleet is a non-empty set of cars driving at the same position and same speed. A single car is also considered a car fleet.
    // If a car catches up to a car fleet the moment the fleet reaches the destination, then the car is considered to be part of the fleet.
    // Return the number of different car fleets that will arrive at the destination.

    // Example 1:
    // Input: target = 10, position = [1,4], speed = [3,2]
    // Output: 1
    // Explanation: The cars starting at 1 (speed 3) and 4 (speed 2) become a fleet, meeting each other at 10, the destination.

    // Example 2:
    // Input: target = 10, position = [4,1,0,7], speed = [2,2,1,1]
    // Output: 3
    // Explanation: The cars starting at 4 and 7 become a fleet at position 10. The cars starting at 1 and 0 never catch up to the car ahead of them. Thus, there are 3 car fleets that will arrive at the destination.

    // Constraints:
    //
    //    n == position.length == speed.length.
    //    1 <= n <= 1000
    //    0 < target <= 1000
    //    0 < speed[i] <= 100
    //    0 <= position[i] < target
    //    All the values of position are unique.

    public static void main(String[] args) {
        System.out.println("here goes something...");
        int[] positions = {4,1,0,7};
        int[] speed = {2,2,1,1};
        int target = 10;

        int result = nahIdBruteForceIt(positions, speed, target);
        System.out.println("result: " + result);

    }

    public static int nahIdBruteForceIt(int[] positions, int[] speeds, int target)  {
        ArrayList<CarData> list = toCardDataArrayList(positions, speeds);
        sortSpeedByPosition(list);
        System.out.println("sorted: " + list);
        // go by each position, and do the math remaining = target - current
        // timeToReach = remaining / speed
        // cluster cars by time to reach
        // if the time to reach matches, they are a cluster
        // if any car/fleet has lesser timeToReach than the ones behind, they are also a cluster

        int clusterCount = 0;
        int lastTimeToReach = -1;
        for (CarData car: list) {
            int remining = target - car.position;
            int timeToReach = remining / car.speed;

//            System.out.println("cardPos: " + car.position + ", timeToReach: " + timeToReach);

            if (lastTimeToReach != -1 && lastTimeToReach <= timeToReach) {
                lastTimeToReach = timeToReach;
            } else {
                clusterCount += 1;
                lastTimeToReach = timeToReach;
            }
        }

        return clusterCount;
    }

    public static class CarData implements Comparable<CarData> {
        Integer speed;
        Integer position;

        CarData(int position, int speed) {
            this.speed = speed;
            this.position = position;
        }

        @Override
        public int compareTo(CarData cardData) {
            return this.position - cardData.position;
        }

        @Override
        public String toString() {
            return "pos: " + this.position + ", speed: " + this.speed;
        }
    }

    public static void sortSpeedByPosition(ArrayList<CarData> cardData) {
//        CarData[] array = cardData.toArray(new CarData[cardData.size()]);     // this was weird
//        Arrays.sort(array);
        Collections.sort(cardData);
    }

    public static ArrayList<CarData> toCardDataArrayList(int[] positions, int[] speeds) {
        ArrayList<CarData> carData = new ArrayList<>();
        for (int i=0; i<positions.length; i++) {
            carData.add(new CarData(positions[i], speeds[i]));
        }

        return carData;
    }
}
