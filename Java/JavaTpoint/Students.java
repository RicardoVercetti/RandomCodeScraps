public class Students implements Cloneable {
    int rollNo;
    String name;

    Students(int roll, String name) {
        this.rollNo = roll;
        this.name = name;
    }

    public Object clone() throws CloneNotSupportedException {
        return super.clone();
    }

    public static void main(String[] args) {
        Students s1 = new Students(123, "Ajay");
        System.out.println("Student 1 roll : " + s1.rollNo);
        System.out.println("Student 1 name : " + s1.name);

        try {
            Students s2 = (Students) s1.clone();
            System.out.println("Student 2 roll : " + s2.rollNo);
            System.out.println("Student 2 name : " + s2.name);

        } catch (CloneNotSupportedException e) {
            e.printStackTrace();
        }
    }
}
