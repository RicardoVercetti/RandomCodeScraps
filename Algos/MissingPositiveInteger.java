import java.util.Arrays;

public class MissingPositiveInteger {
    public static int findIt(int[] arr) {
    System.out.println("Arr : " + Arrays.toString(arr));
    int[] sorted = arr;
    
    Arrays.sort(sorted);
    // System.out.println("Sorted : " + Arrays.toString(sorted));
    for(int i=0; i<sorted.length-1; i++) {
      // System.out.println("i :" + i + " | plus 1: " + (i+1));
      if(sorted[i] <0) continue;
      if(sorted[i+1] - sorted[i] >= 2) {
        // System.out.println("ith : "+sorted[i]+ " ip1: "+ sorted[i+1]);
        return sorted[i]+1;
      }
    }
    return sorted[sorted.length-1]+1;
  }
}
