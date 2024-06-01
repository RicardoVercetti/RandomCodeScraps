import java.util.Arrays;

public class StringArrays {
    public static void main(String[] args) {
        String apids = "A0000000031010,A000000003101001,A0000000031010,A000000003101001,";
        String[] strArray = apids.split(",");
        // for(String s : strArray) {
        //     System.out.println("String : "+ s);
        // }
        System.out.println((Arrays.toString(strArray)).substring(1, Arrays.toString(strArray).length()-1));
        // System.out.println(strArray);
    }
}
