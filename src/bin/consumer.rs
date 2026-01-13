use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::ClientConfig;
use rdkafka::Message;
use futures_util::StreamExt;
use futures_util::TryStreamExt;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("group.id", "demo-group")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["orders"]).unwrap();

    println!("Listening for messages…");

    let mut stream = consumer.stream();

    while let Ok(Some(message)) = stream.try_next().await {
        let payload = match message.payload_view::<str>() {
            Some(Ok(s)) => s,
            Some(Err(e)) => {
                eprintln!("Invalid UTF8: {:?}", e);
                continue;
            }
            None => {
                eprintln!("No payload in message");
                continue;
            }
        };
        println!("Received: {}", payload);
    }
}

