public class DivideWithoutOperators {

    /**
     * divide two numbers without using *,/,+ operators
     */
    static int divide(int a, int b) {
        int sign = ((a < 0) || (b < 0)) ? -1 : 1;
        int quotient = 0;
        while (a > b) {
            int rem = a - b;
            quotient++;
            a = rem;
        }
    
    return quotient*sign;
  }
}
