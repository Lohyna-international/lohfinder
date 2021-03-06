#![feature(array_chunks)]

use data_manager::EventManager;
use pubsub_client::PubSubClient;
mod data_manager;
mod pubsub_client;
mod types;

#[tokio::main]
async fn main() {
    let path_to_db = "/data/uappservice/db";
    let keys = "./key.json";
    let manager = EventManager::new(&path_to_db.to_string()).expect("Failed to create database");
    let client = PubSubClient::new(keys.to_string(), manager)
        .await
        .expect("Failed to initialize pubsub");
    println!("Init finished without errors!!!");
    loop {
        match client.handle_messages(1).await {
            Ok(status) => match client.return_results(status).await {
                Ok(0) => println!("All statuses successfully sent!"),
                Ok(n) => println!("Failed to send {} statuses", n),
                Err(e) => eprintln!("Error sending statuses : {}", e),
            },
            Err(e) => eprintln!("Failed to handle messages : {}", e),
        }
    }
}

#[cfg(test)]
mod tests;

#[test]
#[ignore]
fn clean_tests() {
    assert!(std::fs::remove_dir_all("./test").is_ok());
}
