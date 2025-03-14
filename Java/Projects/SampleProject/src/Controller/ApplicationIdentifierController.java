package Java.Projects.SampleProject.src.Controller;

// import java.util.ArrayList;
import java.util.List;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.ObjectMapper;

import Methods.CreateObjects;
import Models.ApplicationIdentifiers;

// import 

public class ApplicationIdentifierController {

    public void getGsonData() {

        List<ApplicationIdentifiers> li = CreateObjects.getApplicationIdInList();


         ObjectMapper mapper = new ObjectMapper();
         StringBuilder jsonData = new StringBuilder();
        //  jsonData.append("{");
         try {
            // for(int i=0; i<li.size(); i++) {
            //     jsonData.append( mapper.writeValueAsString(li.get(i)));
            //     if(!(li.size()-1 == i)) jsonData.append(",");
            // }

            // for(ApplicationIdentifiers ids : li) {
            //     jsonData.append(mapper.writeValueAsString(ids));
            //     jsonData.append(",");
            // }
            // jsonData.append("}");

            jsonData.append(mapper.writeValueAsString(li));

        } catch (JsonProcessingException e) {
            e.printStackTrace();
        }
        System.out.println(jsonData.toString());
    }
    
}
