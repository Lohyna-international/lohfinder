use super::*;
use crate::{
    pstypes::CreateEventMessage,
    types::{Category, Event},
};
use chrono;
use cloud_pubsub::*;
use tokio_test;

fn test_categories() -> (Category, Category, Category, Category) {
    let cat1 = "first".to_string();
    let cat2 = "second".to_string();
    let cat3 = "third".to_string();
    let cat4 = "fourth".to_string();
    (cat1, cat2, cat3, cat4)
}

fn test_events(
    cat1: Option<Category>,
    cat2: Option<Category>,
    cat3: Option<Category>,
) -> (Event, Event, Event) {
    let (tcat1, tcat2, tcat3, _) = test_categories();
    let event1 = types::Event {
        id: 1,
        title: "Event1".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 3,
        date_created: chrono::Utc::now().timestamp(),
        date_planning: (chrono::Utc::now() + chrono::Duration::days(3)).timestamp(),
        category: cat1.unwrap_or(tcat1),
    };
    let event2 = types::Event {
        id: 2,
        title: "Event2".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 1,
        date_created: chrono::Utc::now().timestamp() + 5,
        date_planning: (chrono::Utc::now() + chrono::Duration::days(2)).timestamp(),
        category: cat2.unwrap_or(tcat2),
    };
    let event3 = types::Event {
        id: 3,
        title: "Event3".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 2,
        date_created: chrono::Utc::now().timestamp() + 10,
        date_planning: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp(),
        category: cat3.unwrap_or(tcat3),
    };
    (event1, event2, event3)
}

async fn init_client() -> (Client, pubsub_client::PubSubClient) {
    let path_to_db = "./test/db";
    let keys = "./key.json";
    let manager = EventManager::new(&path_to_db.to_string()).expect("Failed to create database");
    let client = Client::new(keys.to_string())
        .await
        .expect("Failed to create testing client");
    (
        client,
        pubsub_client::PubSubClient::new(keys.to_string(), manager)
            .await
            .expect("Failed to initialize pubsub"),
    )
}

macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

fn event_to_message(event: Event, id: u64) -> CreateEventMessage {
    pstypes::CreateEventMessage {
        message_id: id,
        id: Some(event.id),
        title: event.title.clone(),
        cover: event.cover.clone(),
        description: event.description.clone(),
        organizer: event.organizer,
        category: event.category.clone(),
        date_created: Some(event.date_created),
        date_planning: event.date_planning,
    }
}

#[test]
fn create_event_pubsub_test() {
    let (test_client, work_client) = aw!(init_client());
    let topic = test_client.topic(String::from("event_create"));
    let topic2 = test_client.topic(String::from("category_create"));
    let (cat1, _, _, _) = test_categories();
    let (event1, event2, _) = test_events(Some(cat1.clone()), Some(cat1.clone()), None);
    let message1 = event_to_message(event1, 11);
    let message2 = event_to_message(event2, 12);
    let create_cat_message = pstypes::CreateCategoryMessage {
        message_id: 10,
        name: cat1.clone(),
    };
    aw!(topic2
        .publish(serde_json::to_string(&create_cat_message).expect("Failed to serialize message")))
    .expect("Failed to send message");
    let time = chrono::Utc::now();
    let res = aw!(work_client.handle_messages()).expect("Failed to handle messages");
    println!(
        "Seconds to pull all messages : {}",
        (chrono::Utc::now() - time).num_seconds()
    );
    assert!(res
        .iter()
        .inspect(|f| println!("{:?}", f))
        .any(|f| f.id == 10 && f.code == 200));
    aw!(work_client.return_results(res)).expect("Failed to return results");

    aw!(topic.publish(serde_json::to_string(&message1).expect("Failed to serialize message")))
        .expect("Failed to send message");
    aw!(topic.publish(serde_json::to_string(&message2).expect("Failed to serialize message")))
        .expect("Failed to send message");
    let res = aw!(work_client.handle_messages()).expect("Failed to handle messages");
    assert!(res
        .iter()
        .inspect(|f| println!("{:?}", f))
        .any(|f| f.id == 11 && f.code == 200));
    assert!(res.iter().any(|f| f.id == 12 && f.code == 200));
    aw!(work_client.return_results(res)).expect("Failed to return results");
    work_client.clean_db();
}
