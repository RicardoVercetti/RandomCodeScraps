public class Power {
    /**
   * custom implementation of pow(a, b)
   */
  static double power(double x, double n) {
    // System.out.println("x: " + x + " | n: " + n);
     if(n==0) return 1;
     if(n<0) {
      // System.out.println("M : "+ (1/(double)x));
       return findRecursiveMult((1/x), (-1 * n));
     } 
     return findRecursiveMult(x,n);
  }

  /**
   * helper function for power(int a, int b)
   */
  static double findRecursiveMult(double val, double times) {
    // System.out.println("val : "+ val+ " | t : " + times);
    double newVal = 1.00;
    while (times > 0) {
      newVal = val * newVal;
      // System.out.println("New val : " + newVal);
      times--;
    }
    return newVal;
  }
}