use super::*;
use crate::types::*;
use chrono;
use cloud_pubsub::*;
use tokio_test;

fn test_app(id: u64, uid: Option<u64>, eid: Option<u64>) -> Application {
    Application {
        id: id,
        user_id: uid.unwrap_or(0),
        event_id: eid.unwrap_or(0),
        status: ApplicationStatus::Created,
    }
}

#[test]
fn merge_test() {
    assert_eq!(
        data_manager::merge(b"", Some(b"ab"), b"cd"),
        Some((b"abcd").to_vec())
    );
}

#[test]
fn create_get_app_test() {
    let manager = data_manager::EventManager::new(&"./test/createapptestdb".to_string())
        .expect("Failed to create db");
    let app = test_app(0, Some(1), Some(2));
    assert!(manager.create_app(&app).is_ok());
    let apps = manager.get_for_event(2).expect("Failed to get apps");
    assert_eq!(vec![app.clone()], apps);
    let apps = manager.get_for_user(1).expect("Failed to get apps");
    assert_eq!(vec![app], apps);
    manager.reset_all().expect("Failed to delete db");
}

#[test]
fn update_app_test() {
    let manager = data_manager::EventManager::new(&"./test/updateapptestdb".to_string())
        .expect("Failed to create db");
    let mut app = test_app(0, Some(1), Some(2));
    assert!(manager.create_app(&app).is_ok());
    let apps = manager.get_for_event(2).expect("Failed to get apps");
    assert_eq!(vec![app.clone()], apps);
    app.set_status(ApplicationStatus::Approved);
    manager
        .update_status(app.id, ApplicationStatus::Approved)
        .expect("Failed to update status");
    let apps = manager.get_for_user(1).expect("Failed to get apps");
    assert_eq!(vec![app], apps);
    manager.reset_all().expect("Failed to delete db");
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
    let topic = test_client.topic(String::from("app_create"));
    let topic2 = test_client.topic(String::from("app_get"));
    let create_app_message = CreateAppMessage {
        message_id: 111,
        id: Some(0),
        user_id: 1,
        event_id: 2,
    };
    let get_app_message = GetAppsMessage {
        message_id: 222,
        id: 1,
        get_for: GetFor::User,
    };
    aw!(topic
        .publish(serde_json::to_string(&create_app_message).expect("Failed to serialize message")))
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
        .any(|f| f.id == 111 && f.code == 200));
    aw!(work_client.return_results(res)).expect("Failed to return results");

    aw!(topic2
        .publish(serde_json::to_string(&get_app_message).expect("Failed to serialize message")))
    .expect("Failed to send message");
    let res = aw!(work_client.handle_messages()).expect("Failed to handle messages");
    assert!(res
        .iter()
        .inspect(|f| println!("{:?}", f))
        .any(|f| f.id == 222 && f.code == 200));
    aw!(work_client.return_results(res)).expect("Failed to return results");
    work_client.clean_db();
}
