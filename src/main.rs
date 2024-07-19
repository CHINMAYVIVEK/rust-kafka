use producer::produce;

mod consumer;
mod producer;

#[tokio::main]
async fn main() {
    // println!("Hello, world!");

    // Kafka - Message Queue

    let producer = producer::create();
    produce(producer, String::from("Hello Worlds, I am testing")).await;

    consumer::start().await;
}
