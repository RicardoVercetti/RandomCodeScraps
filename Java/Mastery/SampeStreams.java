package Java.Mastery;

import java.util.*;
// import java.util.stream.Stream;

public class SampeStreams {

//  Stream Pipeline

//  A typical pipeline has three steps:

//  Source: The data source (e.g., a collection, array, or I/O).
//  Intermediate Operations: Transform the stream (e.g., filter, map).
//  Terminal Operation: Triggers processing and produces a result (e.g., collect, forEach).
    
    public static void main(String[] args) {
        // System.out.println("Runzz...!");
        List<String> names = Arrays.asList("Alice", "Bob", "Charlie");
        names.stream()
                .filter(name -> name.startsWith("A")) // Intermediate
                .forEach(item -> System.out.println("Item : " + item));        // Terminal

        // System.out.println("The value : " + "Mike".toLowerCase());

        // Integer.sum(a, b)


        // Source
        // from collections
        // collection.stream()

        // from arrays
        // Arrays.stream(array)

        // from values
        // Stream<Integer> stream = Stream.of(1,2,3);

        // Infinite streams
        // Stream<Integer> infinite = Stream.iterate(0, n -> n + 1);


        // Intermediate Operations
        // filter: Select elements based on a condition.
        // stream.filter(x -> x > 10);

        // map: Transform each element.
        // stream.map(String::toUpperCase);

        // sorted: Sort elements.
        // stream.sorted();

        // distinct: Remove duplicates.
        // stream.distinct();

        // limit: Restrict the number of elements.
        // stream.limit(5);

        // skip: Skip the first n elements.
        // stream.skip(3);

    }
}
