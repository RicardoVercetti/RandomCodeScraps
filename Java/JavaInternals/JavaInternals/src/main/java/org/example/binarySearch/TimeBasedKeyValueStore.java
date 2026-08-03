package org.example.binarySearch;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;

public class TimeBasedKeyValueStore {
    // Problem statement:
    // Design a time-based key-value data structure that can store multiple values for the same key at different time stamps and retrieve the key's value at a certain timestamp.
    //
    // Implement the TimeMap class:
    //
    //    TimeMap() Initializes the object of the data structure.
    //    void set(String key, String value, int timestamp) Stores the key `key` with the value `value` at the given time timestamp.
    //    String get(String key, int timestamp) Returns a value such that set was called previously, with timestamp_prev <= timestamp. If there are multiple such values, it returns the value associated with the largest timestamp_prev. If there are no values, it returns "".
    //
    // Example 1:
    //
    // Input:
    // ["TimeMap", "set", ["alice", "happy", 1], "get", ["alice", 1], "get", ["alice", 2], "set", ["alice", "sad", 3], "get", ["alice", 3]]
    // Output:
    // [null, null, "happy", "happy", null, "sad"]

    // Explanation:
    // TimeMap timeMap = new TimeMap();
    // timeMap.set("alice", "happy", 1);  // store the key "alice" and value "happy" along with timestamp = 1.
    // timeMap.get("alice", 1);           // return "happy"
    // timeMap.get("alice", 2);           // return "happy", there is no value stored for timestamp 2, thus we return the value at timestamp 1.
    // timeMap.set("alice", "sad", 3);    // store the key "alice" and value "sad" along with timestamp = 3.
    // timeMap.get("alice", 3);           // return "sad"
    //
    // Constraints:
    //
    //    1 <= key.length, value.length <= 100
    //    key and value only include lowercase English letters and digits.
    //    0 <= timestamp <= 10^7
    //    All the timestamps of set are strictly increasing.

    // Recommended Time & Space Complexity
    // You should aim for a solution with O(1) time for set(), O(logn) time for get(), and O(m * n) space,
    // where n is the total number of values associated with a key, and m is the total number of keys.

    HashMap<String, ArrayList<ValueStore>> data;

    public TimeBasedKeyValueStore() {
        this.data = new HashMap<>();
    }

    public void set(String key, String value, int timestamp) {
        if (this.data.containsKey(key)) {
            // get and then set.
            this.data.get(key).add(new ValueStore(value, timestamp));
            return;
        }
        ArrayList<ValueStore> arraylist = new ArrayList<>();
        arraylist.add(new ValueStore(value, timestamp));
        this.data.put(key, arraylist);
    }

    public String get(String key, int timestamp) {
        // look for key
        if (!this.data.containsKey(key)) return "";

        ArrayList<ValueStore> thisList = data.get(key);
        Integer comparableTimeStamp = timestamp;

        int start = 0;
        int end = thisList.size() - 1;
        int mid = (end - start)/2 + end;
//        int last = thisList.size() - 1;

        while (start<=end) {
            ValueStore value = thisList.get(mid);
            if (value.timeStamp.equals(comparableTimeStamp)) {
                return value.value;
            } else if (start == end && end == mid && !value.timeStamp.equals(comparableTimeStamp)) {
                // try getting later value, else this value(which would be previous
                if (mid < thisList.size() - 1) return thisList.get(mid+1).value;
                return thisList.get(mid).value;
            } else if (timestamp < value.timeStamp) {
                end = mid;
            } else if (timestamp > value.timeStamp) {
                start = mid;
            }
            mid = (end - start)/2 + start;
        }

        return "-1";        // this is an impossible condition
    }

    void display() {
        for (Map.Entry<String, ArrayList<ValueStore>> value: this.data.entrySet()) {
            System.out.println(value.getKey() + ": " + value.getValue().toString());
        }
    }

    public static void main(String[] args) {
        System.out.println("here goes nothing...");
        TimeBasedKeyValueStore timeMap = new TimeBasedKeyValueStore();
        timeMap.set("alice", "happy", 1);
        System.out.println(timeMap.get("alice", 1) + ", but should return \"happy\"");        // should return "happy"
        System.out.println(timeMap.get("alice", 2) + ", should also return \"happy\"");        // should also return "happy"
        timeMap.set("alice", "sad", 3);
        System.out.println(timeMap.get("alice", 3) + ", should return \"sad\"");        // should return "sad"

//        timeMap.display();
    }

    static class ValueStore {
        String value;
        Integer timeStamp;

        public ValueStore(String value, Integer timeStamp) {
            this.value = value;
            this.timeStamp = timeStamp;
        }

        public String toString() {
            return "(" + this.value + ", " + this.timeStamp + ")";
        }
    }
}
