use cloud_pubsub::*;
use std::sync::Arc;
use std::env;
use super::types::{Event, Category};

#[derive(Debug)]
struct UpdatePacket(String);

impl FromPubSubMessage for UpdatePacket {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => Ok(UpdatePacket(String::from_utf8_lossy(&bytes).into_owned())),
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[tokio::main]
async fn test() {
    let PUBSUB_HOST: String = env::var("PUBSUB_EMULATOR_HOST")
        .map(|host| format!("http://{}", host))
        .unwrap_or_else(|_| String::from("https://pubsub.googleapis.com"));
    println!("Will use Host {}", PUBSUB_HOST);
    let pubsub = Client::new("key.json".to_string()).await.expect("Failed to initialize pubsub");
    let topic = Arc::new(pubsub.topic("topic-test".to_string()));
    let sub = topic.subscribe().await.expect("Failed to subscribe");
    match topic.clone().publish("🔥").await {
        Ok(response) => {
            println!("{:?}", response);
        }
        Err(e) => eprintln!("Failed sending message {}", e),
    }

    println!("Subscribed to topic with: {}", sub.name);
    let packets = sub
        .clone()
        .get_messages::<UpdatePacket>()
        .await
        .expect("Error Checking PubSub");

    for packet in &packets {
        println!("Received: {:?}", packet);
    }

    if !packets.is_empty() {
        let acks = packets
            .into_iter()
            .map(|packet| packet.1)
            .collect::<Vec<_>>();
        sub.acknowledge_messages(acks).await;
    } else {
        println!("Cleaning up");
        drop(pubsub);
        sub.destroy().await.expect("Failed deleting subscription");
        println!("Successfully deleted subscription");
    }
}
