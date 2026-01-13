use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Create producer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .set("acks", "all")
        .create()
        .expect("Producer creation failed");

    // Build record
    let record: FutureRecord<'_, str, str> = FutureRecord::to("orders")
        .payload(r#"{"order_id":1235,"amount":39.99}"#)
        .key("order-1234");
        // headers() removed for now

    // Send asynchronously
    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Message delivered: {:?}", delivery),
        Err((err, _msg)) => eprintln!("Failed to deliver message: {:?}", err),
    }
}

