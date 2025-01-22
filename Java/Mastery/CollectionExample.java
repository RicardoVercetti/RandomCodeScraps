package Java.Mastery;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Queue;
import java.util.Set;

public class CollectionExample {
    public static void main(String[] args) {
        // List Example
        List<String> list = new ArrayList<>();
        list.add("Apple");
        list.add("Banana");
        list.add("Apple"); // Allows duplicates
        System.out.println("List: " + list);

        // Set Example
        Set<String> set = new HashSet<>(list);
        System.out.println("Set: " + set); // Removes duplicates

        // Map Example
        Map<String, Integer> map = new HashMap<>();
        map.put("Apple", 2);
        map.put("Banana", 3);
        System.out.println("Map: " + map);

        // Queue Example
        Queue<String> queue = new LinkedList<>();
        queue.add("First");
        queue.add("Second");
        System.out.println("Queue: " + queue);
    }
}
