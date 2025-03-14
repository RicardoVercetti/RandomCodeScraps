package Java.JavaTpoint;
public class CallByValue {
    int a = 5;

    void change(int n) {
        // System.out.println("Value of a before change : " + a);
        a = a + n;
        // System.out.println("Value of a after change : " + a);
    }

    public static void main(String[] args) {
        System.out.println("Hey...");
        CallByValue cbv = new CallByValue();
        // int num = 5;
        System.out.println("Value of a before change : " + cbv.a);
        cbv.change(5);
        System.out.println("Value of a after change : " + cbv.a);

        // System.out.println("Change : " + cbv.a);
    }
}
