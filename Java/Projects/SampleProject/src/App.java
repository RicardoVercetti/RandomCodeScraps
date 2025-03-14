package Java.Projects.SampleProject.src;
// import Controller.ApplicationIdentifierController;
// import Controller.JsonDataBuilder;
import Methods.PrintingJson;

public class App {
    public static void main(String[] args) throws Exception {

        // System.out.println("Hello, World!");

        // ApplicationIdentifierController cn = new ApplicationIdentifierController();
        // cn.getGsonData();

        // JsonDataBuilder jb = new JsonDataBuilder();
        // jb.convertStringToJson();
        // jb.aidReturner();


        PrintingJson pj = new PrintingJson();
        // pj.printJson();
        pj.prettyPrinting();
    }
}
