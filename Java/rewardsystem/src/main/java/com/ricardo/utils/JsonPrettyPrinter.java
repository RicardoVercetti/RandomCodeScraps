package com.ricardo.utils;

import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.SerializationFeature;
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule;

public class JsonPrettyPrinter {
    static final ObjectMapper objectMapper;

    static {
        objectMapper = new ObjectMapper().registerModule(new JavaTimeModule());;
        objectMapper.enable(SerializationFeature.INDENT_OUTPUT);
    }

    @SuppressWarnings("SpellCheckingInspection")
    public static String jsonifyObject(Object obj) {
        try {
            return objectMapper.writeValueAsString(obj);
        } catch (JsonProcessingException e) {
            throw new RuntimeException("Error while JSONifying object", e);
        }
    }

    public static String prettyJson(String str) {
        try {
            // Parse the string into a JsonNode
            JsonNode jsonNode = objectMapper.readTree(str);

            // Convert the JsonNode into a pretty-printed JSON string
            return objectMapper.writeValueAsString(jsonNode);
        } catch (JsonProcessingException e) {
            throw new RuntimeException(e);
        }
    }
}
