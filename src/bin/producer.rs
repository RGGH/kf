use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    let record = FutureRecord::to("demo-topic")
        .payload("hello kafka")
        .key("key1");

    producer
        .send(record, Duration::from_secs(0))
        .await
        .expect("Send failed");

    println!("Message sent");
}

