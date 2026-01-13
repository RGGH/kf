use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;                 // 👈 REQUIRED
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "demo-group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer
        .subscribe(&["demo-topic"])
        .expect("Subscription failed");

    println!("Listening for messages…");

    let mut stream = consumer.stream();

    while let Ok(Some(message)) = stream.try_next().await {
        if let Some(Ok(payload)) = message.payload_view::<str>() {
            println!("Received: {}", payload);
        } else {
            println!("Received empty or non-UTF8 payload");
        }
    }
}

