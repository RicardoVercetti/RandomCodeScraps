import java.util.ArrayList;
import java.util.List;

class ListOperations {
    public static void main(String[] args) {
        List<String> li = new ArrayList<>();
        for (int i=0; i<6; i++) {
            li.add("String "+i);
        }

        for (String s : li) {
            System.out.println(s);
        }
        
    }
}