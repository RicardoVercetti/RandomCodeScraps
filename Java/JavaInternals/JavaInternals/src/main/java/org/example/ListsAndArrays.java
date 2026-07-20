package org.example;

public class ListsAndArrays {           //  https://www.geeksforgeeks.org/java/java-collection-tutorial/
    // notes:
    // 1. dangling else ambiguity - every inner if is paired up with every nearest below else statement
    public static void main(String[] args) {
        System.out.println("started yooo....");

        int a = 2;
        int b = 3;

        if (a==2) if (b==3) System.out.println("ininside");
        else System.out.println("lA neighbourhood");
        else System.out.println("one more...");

        System.out.println("outside of any of this...");

//        int n = 0;
//        int m = 0;
//        boolean haveToRun = true;
//        while(haveToRun) {
//            if (n == 5) if (m == 5) haveToRun = false;
//            else System.out.println("not true I guess");
//            n++;
//            m++;
//            System.out.println("continuting to increment");
//        }
    }
}
