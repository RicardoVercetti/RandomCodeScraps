package Java.Mastery;

import java.util.*;

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



    }
}
