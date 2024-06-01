package Methods;

import java.util.List;

import com.google.gson.Gson;
import com.google.gson.GsonBuilder;
import com.google.gson.JsonArray;
import com.google.gson.JsonObject;

import Models.ApplicationIdentifiers;

public class PrintingJson {

    public void printJson() {
        // wanna print json data in a readable format
        Gson gson = new Gson();
        List<ApplicationIdentifiers> li = CreateObjects.getApplicationIdInList();
        JsonArray jsonArray = new JsonArray();
        for (ApplicationIdentifiers id : li) {
            JsonObject obj = gson.toJsonTree(id).getAsJsonObject();
            jsonArray.add(obj);
            System.out.println(obj.toString());
        }
        // System.out.println(jsonArray.toString());

    }

    public void prettyPrinting() {
        Gson gson = new GsonBuilder().setPrettyPrinting().create();
        List<ApplicationIdentifiers> li = CreateObjects.getApplicationIdInList();
        // JsonArray jsonArray = new JsonArray();
        // for (ApplicationIdentifiers id : li) {
        //     JsonObject obj = gson.toJsonTree(id).getAsJsonObject();
        //     jsonArray.add(obj);
        //     String jsonString = gson.toJson(obj);
        // }
        JsonArray ob = gson.toJsonTree(li).getAsJsonArray();
        System.out.println(gson.toJson(ob));
        
        // String jsonString = gson.toJson(jsonArray);
        // System.out.println(jsonString);
    }
}
