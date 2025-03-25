package org.example.kafka;

import org.apache.kafka.clients.producer.*;

import java.util.Properties;

public class SimpleProducer {
    public static void main(String[] args) {
        // Set up the producer properties
        Properties props = new Properties();
        props.put("bootstrap.servers", "localhost:9092");
        props.put("key.serializer", "org.apache.kafka.common.serialization.StringSerializer");
        props.put("value.serializer", "org.apache.kafka.common.serialization.StringSerializer");

        // Create the Kafka producer
        Producer<String, String> producer = new KafkaProducer<>(props);

        // Create a producer record
        ProducerRecord<String, String> record = new ProducerRecord<>("test-topic", "key", "Hello, Kafka!");

        // Send the record
        producer.send(record, (metadata, exception) -> {
            if (exception == null) {
                System.out.printf("Message sent successfully to partition %d, offset %d%n",
                        metadata.partition(), metadata.offset());
            } else {
                exception.printStackTrace();
            }
        });

        // Close the producer
        producer.close();
    }
}