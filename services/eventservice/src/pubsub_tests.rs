use super::*;
use crate::types::{Category, Event};
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

#[test]
fn create_event_pubsub_test() {
    let (test_client, work_client) = aw!(init_client());
    let topic = test_client.topic(String::from("event_create"));
    let topic2 = test_client.topic(String::from("category_create"));
    let sub = test_client.subscribe(String::from("results"));
    let (cat1, _, _, _) = test_categories();
    let (event1, _, _) = test_events(Some(cat1.clone()), None, None);
    let message = pstypes::CreateEventMessage {
        message_id: 0,
        id: Some(event1.id),
        title: event1.title.clone(),
        cover: event1.cover.clone(),
        description: event1.description.clone(),
        organizer: event1.organizer,
        category: event1.category.clone(),
        date_created: Some(event1.date_created),
        date_planning: event1.date_planning,
    };
    let create_cat_message = pstypes::CreateCategoryMessage {
        message_id: 1,
        name: cat1.clone(),
    };
    aw!(topic2
        .publish(serde_json::to_string(&create_cat_message).expect("Failed to serialize message")))
    .expect("Failed to send message");
    let time = chrono::Utc::now();
    let res = aw!(work_client.handle_messages()).expect("Failed to handle messages");
    println!("Seconds to pull all messages : {}", (chrono::Utc::now() - time).num_seconds());
    assert!(res
        .iter()
        .inspect(|f| println!("{:?}", f))
        .any(|f| f.id == 1 && f.code == 200));
    aw!(work_client.return_results(res)).expect("Failed to return results");
    let statuses = aw!(sub.get_messages::<pstypes::Status>()).expect("Failed to get statuses");
    assert!(statuses
        .iter()
        .filter_map(|f| f.0.as_ref().ok())
        .any(|s| s.id == 1 && s.code == 200));

    aw!(topic.publish(serde_json::to_string(&message).expect("Failed to serialize message")))
        .expect("Failed to send message");
    let res = aw!(work_client.handle_messages()).expect("Failed to handle messages");
    assert!(res
        .iter()
        .inspect(|f| println!("{:?}", f))
        .any(|f| f.id == 0 && f.code == 200));
    aw!(work_client.return_results(res)).expect("Failed to return results");
    let statuses = aw!(sub.get_messages::<pstypes::Status>()).expect("Failed to get statuses");
    assert!(statuses
        .iter()
        .filter_map(|f| f.0.as_ref().ok())
        .any(|s| s.id == 0 && s.code == 200));
    work_client.clean_db();
}
