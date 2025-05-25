public class StringToInt {
    /**
     * String to int custom implementation like atoi in c/c++
     */
    static int atoi(String num) {
        int idx = 0;
        int sign = 1;
        int res = 0;

        // remove leading white spaces
        while (idx < num.length() && num.charAt(idx) == ' ') {
            idx++;
        }
        // System.out.println("After removing leading white spaces : '" +
        // num.substring(idx)+"'");

        // assign sign if its there
        if (num.charAt(idx) == '-' || num.charAt(idx) == '+') {
            if (num.charAt(idx) == '-') {
                sign = -1;
                idx++;
            }
        }

        // parse each item
        while (idx < num.length() && num.charAt(idx) >= '0' && num.charAt(idx) <= '9') {
            // TODO the overflow and underflow have to be checked

            res = res * 10 + (num.charAt(idx) - '0');
            idx++;
        }

        return res * sign;

    }
}
