package org.example;

import java.util.Arrays;
import java.util.ArrayList;
import java.util.Collections;
//import java.util.Set;
//import java.util.HashSet;
import java.util.List;


public class Main {
    // mess-ups
    // [OK] static arrays   - doesn't contain add/remove, these are implemented by dyn arrays
    // [OK] dynamic arrays  - ArrayList, List(interface)
    // [OK] add/remove from arrays

    // [OK] check duplicates
    // [OK] sort array in ascending/descending
    //          - [OK] inbuilt ascending sort
    //          - [OK] inbuilt descending sort
    //          - [OK] custom ascending sort
    //          - [OK] custom descending sort

    // things to test and find out
    // [OK] altering a static array inside a function changes the parent reference - it changes
    // [  ] java linkedList implementation
    // [OK] reverse a list - this is a separate implementation
    // [OK] what does the divide operator do exactly - `a/b` - if integers floor division, if float, the decimal value division


    // notes:
    // 1. List is an interface and ArrayList is a concrete implementation
    // 2. There are other implementations such as Vector and so on.
    // 3. remove(index) only works if the passed in data is a primitive int, Integer is from Object, it will be treated as getting by object.
    // 4. primitives are only allowed in native arrays, ArrayList or any wildcard <E> should pass in Objects(should use wrapper classes)
    // 5. native arrays are declared with `{ }` syntax
    // 6. `arr.length` on native arrays, `arr.size()` on blankets

    public static void main(String[] args) {
        System.out.println("Started...");

        String[] models = {"Televisions", "Smartphones", "Refrigerators", "Televisions", "RemoteControls", "Smartphones", "Refrigerators"};

//        int[] numbers = {2, 3, 4, 5, 6, 7};              // or could use the `int [] var = new int [10];`
//        int[] returnedNumbers = changingTheArray(numbers);
//        System.out.println("outside after changing things: " + Arrays.toString(numbers));
//        System.out.println("retuned number: " + Arrays.toString(returnedNumbers));



        System.out.println("all strings: " + Arrays.toString(models));
        System.out.println("--".repeat(80));

        // removing the duplicates
        var li = Arrays.asList(models);
//        System.out.println("li: " + li);

        var uniqueList = removeDuplicates(models).toArray();

        System.out.println("unique non duplicate list: " + Arrays.toString(uniqueList));

        // sort by ascending
        Arrays.sort(uniqueList);
        System.out.println("after ascending sort: " + Arrays.toString(uniqueList));

        // sort by descending
        Arrays.sort(uniqueList, Collections.reverseOrder());
        System.out.println("after descending sort: " + Arrays.toString(uniqueList));

    }

    public static ArrayList<String> removeDuplicates(String[] input) {
        // go by each element,
        // if this element is equal to any other element on the right side from the element, it's a duplicate
        // when it's a duplicate, add it to a set/list
        // if the current index in set/list, skip through it
        // go until the end of the array

        List<Integer> duplicateIndices = new ArrayList<>();
        ArrayList<String> inputArray = new ArrayList<>(Arrays.asList(input));

        // find indices of duplicate items
        for (int i=0; i<input.length; i++) {
            if (duplicateIndices.contains(i)) continue;
            for (int j=i+1; j<input.length; j++ ) {
                if (duplicateIndices.contains(j)) continue;

                if (input[i].equals(input[j])) {
                    duplicateIndices.add(j);
                }
            }
        }

        // remove those duplicates by index in reverse order
        for (int i=duplicateIndices.size(); i>0; i--) {
            inputArray.remove((int) duplicateIndices.get(i-1));
        }

        return inputArray;
    }

    public static int[] changingTheArray(int[] input) {
//        System.out.println("array input: " + Arrays.toString(input));

        for(int i=0; i<input.length/2; i++) {
            int a = input[i];
            input[i] = input[input.length-1-i];
            input[input.length-1-i] = a;
        }

//        System.out.println("after swap: " + Arrays.toString(input));
        return input;
    }
}