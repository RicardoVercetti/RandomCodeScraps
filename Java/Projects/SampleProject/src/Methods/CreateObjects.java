package Java.Projects.SampleProject.src.Methods;
import Models.ApplicationIdentifiers;
import java.util.ArrayList;
import java.util.List;

public class CreateObjects {
    public static List<ApplicationIdentifiers> getApplicationIdInList() {

        ApplicationIdentifiers id1 = new ApplicationIdentifiers();
        id1.setApplication_identifier("A0000000031010");
        id1.setCountry("United States");
        id1.setDescription("Standard/Gold VISA credit card");
        id1.setName("VISA Debit/Credit (Classic)");
        id1.setType("EMV");
        id1.setVendor("Visa International");

        ApplicationIdentifiers id2 = new ApplicationIdentifiers();
        id2.setApplication_identifier("A0000000031010");
        id2.setCountry("United States");
        id2.setDescription("Standard/Gold VISA credit card");
        id2.setName("VISA Debit/Credit (Classic)");
        id2.setType("EMV");
        id2.setVendor("Visa International");

        List<ApplicationIdentifiers> li = new ArrayList<>();
        li.add(id1);
        li.add(id2);
        return li;
    }
}
