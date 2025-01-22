package Java.Mastery;

import java.util.Arrays;
import java.util.List;

public class IterationTechniques {
    
    public static void main(String[] args) {
        List<String> names = Arrays.asList("Alice", "Bob", "Charlie");

        // with iterator
        // Iterator<String> itr = names.iterator();
        // while (itr.hasNext()) {
        //     System.out.println("--- > "+ itr.next());
        // }

        // with Stream API
        names.stream().forEach(System.out::println);
    }
}
