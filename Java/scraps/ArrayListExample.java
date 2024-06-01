

import java.util.ArrayList;
import java.util.List;

public class ArrayListExample {
    public static void main(String[] args) {
        List<String> l = new ArrayList<>();
        l.add("One");
        l.add("Two");
        l.add("Three");

        System.out.println(String.join(",", l));
    }
}
