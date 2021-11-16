use crate::pstypes;

use super::types::{Category, Event};
use super::pstypes::*;
use cloud_pubsub::*;
use std::env;
use std::sync::Arc;
use std::collections::HashMap;

struct PubSubClient {
    topics : HashMap<String, Arc<Topic>>,
    subs : HashMap<String, Subscription>,
}

impl PubSubClient {
    pub async fn new(keys : String) -> Result<PubSubClient, Box<dyn std::error::Error>> {
        let client = Client::new(keys).await?;
        let topics_names = vec!["event_create", "event_delete", "event_update", "event_get", "events", "categories","category_create","category_delete", "category_merge"];
        let mut topics = HashMap::new();
        let mut subs = HashMap::new();
        topics_names.iter().map(|f| f.to_string()).for_each(|f| {
            let topic = Arc::new(client.topic(f.clone()));
            topics.insert(f, topic);
        });
        for (name, topic) in &topics {
            let sub = topic.subscribe().await.expect("Failed to subscribe");
            subs.insert(name.clone(), sub);
        }
        Ok(PubSubClient {topics : topics, subs : subs})
    }

    fn parse_message<T, U>(message : T) -> Option<U> where T : FromPubSubMessage, U : PubSubCallBack {
        
    }
}


#[tokio::main]
async fn test() {
    let PUBSUB_HOST: String = env::var("PUBSUB_EMULATOR_HOST")
        .map(|host| format!("http://{}", host))
        .unwrap_or_else(|_| String::from("https://pubsub.googleapis.com"));
    println!("Will use Host {}", PUBSUB_HOST);
    let pubsub = Client::new("key.json".to_string())
        .await
        .expect("Failed to initialize pubsub");
    let topic = Arc::new(pubsub.topic("topic-test".to_string()));
    let sub = topic.subscribe().await.expect("Failed to subscribe");
    match topic.publish("🔥").await {
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
