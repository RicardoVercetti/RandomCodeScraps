package Java.Projects.SampleProject.src.Methods;

// import com.fasterxml.jackson.databind.ObjectMapper;
import com.google.gson.JsonElement;
// import com.google.gson.JsonObject;
import com.google.gson.JsonParser;

public class JsonElementPack {
    public static void getJsonElement(String data) {
        // String data to JsonElement conversion
        JsonElement obj = new JsonParser().parse(data);
        
        System.out.println("Data object : " + obj);
    }
}
