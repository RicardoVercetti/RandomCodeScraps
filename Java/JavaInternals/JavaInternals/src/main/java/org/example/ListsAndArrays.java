package org.example;

import java.util.ArrayList;

public class ListsAndArrays {           //  https://www.geeksforgeeks.org/java/java-collection-tutorial/
    // notes:
    // 1. dangling else ambiguity - every inner if is paired up with every nearest below else statement
    // 2. ArrayList implementation - add(by index, element), remove(index, element), get(index), indexOf(element), addAll, removeAll, size, clear + some iterators
    // 3.
    public static void main(String[] args) {
        System.out.println("started yooo....");

        ArrayList<Integer> someArray = new ArrayList<Integer>();
        for(int i=1; i<=20; i++) {
            someArray.add(i);
//            if (i%5==0) someArray.remove(Integer.valueOf(i-1));
        }

        someArray.add(1, Integer.valueOf(30));
        someArray.remove(2);


        System.out.println("array data: " + someArray);
        System.out.println("size: " + someArray.size());
    }
}
