package Controller;

import java.util.List;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
// import com.fasterxml.jackson.databind.ObjectMapper;
import com.google.gson.JsonObject;
// import com.google.gson.JsonParser;

import Methods.CreateObjects;
import Models.ApplicationIdentifiers;

public class JsonDataBuilder {
    public void buildJson() {
        JsonObject jsonObject = new JsonObject();
        jsonObject.addProperty("name", "John Doe");
        jsonObject.addProperty("age", 30);
        jsonObject.addProperty("city", "New York");
        

        String jsonString = jsonObject.toString();
        System.out.println(jsonString);
    }

    public void convertStringToJson() {
        
        JsonObject returnData = new JsonObject();
        returnData.addProperty("status", "success");
        // returnData.add;
        // JsonObject jsonObject = (new JsonParser()).parse(data).getAsJsonObject();
        System.out.println(returnData.toString());

        // System.out.println(data);
    }

    public void stringPack() {
        // String data = "";
    }

    public void aidReturner() throws JsonProcessingException {

        JsonObject returnData = new JsonObject();
        String data =  stringOfObjects();
        returnData.addProperty("status", "success");
        returnData.addProperty("data", data);

        System.out.println("Return data : "+returnData.toString());
    }

    public String stringOfObjects() {
        // prereq
        Gson gson = new GsonBuilder().create();

        // builder
        // StringBuilder sb = new StringBuilder();

        List<ApplicationIdentifiers> li = CreateObjects.getApplicationIdInList();
        System.out.println("Individual data :");
        // sb.append("[");
        // for(ApplicationIdentifiers id : li) {
        //     String json = gson.toJson(id);
        //     System.out.println(json);
        //     sb.append(// The `json` variable in the `stringOfObjects` method is storing the JSON
        //     // representation of an `ApplicationIdentifiers` object using the Gson library's
        //     // `toJson` method. This method converts the `ApplicationIdentifiers` object `id`
        //     // into a JSON string representation.
        //     json);
        //     sb.append(",");
        // }
        // sb.append("]");

        String json = gson.toJson(li);
        System.out.println(json);

        return json;
    }
}
