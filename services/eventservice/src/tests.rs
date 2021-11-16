use crate::types::{Category, Event};

use super::*;

fn test_categories() -> (Category, Category, Category, Category) {
    let cat1 = "first".to_string();
    let cat2 = "second".to_string();
    let cat3 = "third".to_string();
    let cat4 = "fourth".to_string();
    (cat1, cat2, cat3, cat4)
}

fn test_events(cat1 : Option<Category>, cat2 : Option<Category>,cat3 : Option<Category>) -> (Event,Event,Event) {
    let (tcat1, tcat2, tcat3, _) = test_categories();
    let event1 = types::Event {
        id: 1,
        title: "Event1".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 3,
        date_created: chrono::Utc::now().timestamp(),
        date_planning: (chrono::Utc::now() + chrono::Duration::days(3)).timestamp(),
        category: cat1.unwrap_or(tcat1)
    };
    let event2 = types::Event {
        id: 2,
        title: "Event2".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 1,
        date_created: chrono::Utc::now().timestamp() + 5,
        date_planning: (chrono::Utc::now() + chrono::Duration::days(2)).timestamp(),
        category: cat2.unwrap_or(tcat2)
    };
    let event3 = types::Event {
        id: 3,
        title: "Event3".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 2,
        date_created: chrono::Utc::now().timestamp() + 10,
        date_planning: (chrono::Utc::now() + chrono::Duration::days(1)).timestamp(),
        category: cat3.unwrap_or(tcat3)
    };
    (event1, event2, event3)
}

#[test]
fn merge_test() {
    assert_eq!(
        data_manager::EventManager::_merge(b"", Some(b"ab"), b"cd"),
        Some((b"abcd").to_vec())
    );
}

#[test]
fn compare_by_val_test() {
    let (event1, event2, _) = test_events(None, None, None);
    assert_eq!(
        data_manager::EventManager::_compare_by_val(None, &event1, &event2),
        Some(std::cmp::Ordering::Greater)
    );
    assert_eq!(
        data_manager::EventManager::_compare_by_val(Some(&"title".to_string()), &event1, &event2),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        data_manager::EventManager::_compare_by_val(
            Some(&"organizer".to_string()),
            &event1,
            &event2
        ),
        Some(std::cmp::Ordering::Greater)
    );
    assert_eq!(
        data_manager::EventManager::_compare_by_val(
            Some(&"date_created".to_string()),
            &event1,
            &event2
        ),
        Some(std::cmp::Ordering::Less)
    );
    assert_eq!(
        data_manager::EventManager::_compare_by_val(
            Some(&"date_planning".to_string()),
            &event1,
            &event2
        ),
        Some(std::cmp::Ordering::Greater)
    );
    assert_eq!(
        data_manager::EventManager::_compare_by_val(
            Some(&"non_exisint".to_string()),
            &event1,
            &event2
        ),
        Some(std::cmp::Ordering::Greater)
    );
}

#[test]
fn remove_id_test() {
    let mut ids = (42 as u64).to_be_bytes().to_vec();
    ids.append(&mut (43 as u64).to_be_bytes().to_vec());
    let data = &ids[..];
    assert_eq!(
        data_manager::EventManager::_remove_id(data, &(42 as u64).to_be_bytes()),
        Some((43 as u64).to_be_bytes().to_vec())
    );
    assert_eq!(
        data_manager::EventManager::_remove_id(data, &(1 as u64).to_be_bytes()),
        None
    );
}

#[test]
fn events_to_vec_test() {
    let (event1, event2, _) = test_events(None, None, None);
    assert_eq!(
        data_manager::EventManager::_events_to_vec(&vec![
            event1.to_json().unwrap().as_bytes(),
            event2.to_json().unwrap().as_bytes()
        ]),
        vec![event1, event2]
    );
}

#[test]
fn ids_to_vec_test() {
    assert_eq!(
        data_manager::EventManager::_ids_to_vec(&(42 as u64).to_be_bytes()),
        vec![42 as u64]
    );
    assert_eq!(
        data_manager::EventManager::_ids_to_vec(b"42"),
        Vec::<u64>::new()
    );
}

#[test]
fn create_event_test() {
    let manager = data_manager::EventManager::new(&"./test/createeventtestdb".to_string()).expect("Failed to create db");
    let (cat1, _, _, _) = test_categories();
    let (event1, _, _) = test_events(Some(cat1.clone()), None, None);
    assert!(manager.create_event(&event1).is_err());
    manager
        .create_category(&cat1)
        .expect("Failed to create category!");
    manager
        .create_event(&event1)
        .expect("Failed to create event");
    let event = manager.get_event(&1).expect("Failed to get event");
    assert_eq!(event, event1);
    manager._reset_all().expect("Failed to delete db");
}

#[test]
fn delete_event_test() {
    let manager = data_manager::EventManager::new(&"./test/deleventtestdb".to_string()).expect("Failed to create db");
    let (cat1, _, _, _) = test_categories();
    let (event1, event2, _) = test_events(Some(cat1.clone()), Some(cat1.clone()), None);
    manager
        .create_category(&cat1)
        .expect("Failed to create category!");
    manager
        .create_event(&event1)
        .expect("Failed to create event");
    manager
        .create_event(&event2)
        .expect("Failed to create event");
    let events = manager
        .get_events(Some(&"title".to_string()), None, None)
        .expect("Failed to get events");
    assert_eq!(events, vec![event1.clone(), event2.clone()]);
    manager
        .delete_event(&event1.id)
        .expect("Failed to delete event");
    let events = manager
        .get_events(Some(&"title".to_string()), None, None)
        .expect("Failed to get events");
    assert_eq!(events, vec![event2.clone()]);
    assert!(manager.delete_event(&event2.id).is_ok());
    manager._reset_all().expect("Failed to delete db");
}

#[test]
fn update_event_test() {
    let manager = data_manager::EventManager::new(&"./test/updeventtestdb".to_string()).expect("Failed to create db");
    let (cat1, _, _, _) = test_categories();
    let (event1, _, _) = test_events(Some(cat1.clone()), None, None);
    let event2 = types::Event {
        id: 1,
        title: "Event2".to_string(),
        cover: "N/A".to_string(),
        description: "Empty".to_string(),
        organizer: 1,
        date_created: chrono::Utc::now().timestamp() + 5,
        date_planning: (chrono::Utc::now() + chrono::Duration::days(2)).timestamp(),
        category: cat1.clone()
    };
    manager
        .create_category(&cat1)
        .expect("Failed to create category!");
    manager
        .create_event(&event1)
        .expect("Failed to create event");
    assert_eq!(manager.get_event(&1).unwrap(), event1);
    manager
        .update_event(&event2)
        .expect("Failed to update event");
    assert_eq!(manager.get_event(&1).unwrap(), event2);
    manager._reset_all().expect("Failed to delete db");
}

#[test]
fn get_event_test() {
    let manager = data_manager::EventManager::new(&"./test/geteventtestdb".to_string()).expect("Failed to create db");
    let (cat1, _, _, _) = test_categories();
    let (event1, _, _) = test_events(Some(cat1.clone()), None, None);
    manager
        .create_category(&cat1)
        .expect("Failed to create category!");
    assert!(manager.get_event(&1).is_err());
    manager
        .create_event(&event1)
        .expect("Failed to create event");
    assert_eq!(manager.get_event(&1).unwrap(), event1);
    manager._reset_all().expect("Failed to delete db");
}

#[test]
fn get_events_test() {
    let manager = data_manager::EventManager::new(&"./test/geteventstestdb".to_string()).expect("Failed to create db");
    let (cat1, cat2, _, _) = test_categories();
    let (event1, event2, event3) = test_events(Some(cat1.clone()), Some(cat2.clone()), Some(cat1.clone()));
    manager
        .create_category(&cat1)
        .expect("Failed to create category!");
    manager
        .create_category(&cat2)
        .expect("Failed to create category!");
    manager
        .create_event(&event1)
        .expect("Failed to create event");
    manager
        .create_event(&event2)
        .expect("Failed to create event");
    manager
        .create_event(&event3)
        .expect("Failed to create event");
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), None, None)
            .unwrap(),
        vec![event1.clone(), event2.clone(), event3.clone()]
    );
    assert_eq!(
        manager
            .get_events(Some(&"date_planning".to_string()), None, None)
            .unwrap(),
        vec![event3.clone(), event2.clone(), event1.clone()]
    );
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), Some(1), None)
            .unwrap(),
        vec![event2.clone()]
    );
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), None, Some(&cat2))
            .unwrap(),
        vec![event2.clone()]
    );
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), Some(2), Some(&cat1))
            .unwrap(),
        vec![event3.clone()]
    );
    manager._reset_all().expect("Failed to delete db");
}

#[test]
fn category_get_delete_test() {
    let manager = data_manager::EventManager::new(&"./test/cattestdb".to_string()).expect("Failed to create db");
    let (cat1, cat2, cat3, cat4 ) = test_categories();
    manager
        .create_category(&cat1)
        .expect("Failed to create category");
    manager
        .create_category(&cat2)
        .expect("Failed to create category");
    manager
        .create_category(&cat3)
        .expect("Failed to create category");
    assert_eq!(
        manager.get_categories().unwrap(),
        vec![cat1.clone(), cat2.clone(), cat3.clone()]
    );
    manager
        .delete_category(&cat1)
        .expect("Failed to delete category");
    assert!(manager.delete_category(&cat4).is_err());
    assert_eq!(manager.get_categories().unwrap(), vec![cat2, cat3]);
    manager._reset_all().expect("Failed to delete db");
}

#[test]
fn merge_categories_test() {
    let manager = data_manager::EventManager::new(&"./test/mergetestdb".to_string()).expect("Failed to create db");
    let (cat1, cat2, _, _) = test_categories();
    let (event1, event2, event3) = test_events(Some(cat1.clone()), Some(cat2.clone()), Some(cat1.clone()));
    manager
        .create_category(&cat1)
        .expect("Failed to create category!");
    manager
        .create_category(&cat2)
        .expect("Failed to create category!");
    manager
        .create_event(&event1)
        .expect("Failed to create event");
    manager
        .create_event(&event2)
        .expect("Failed to create event");
    manager
        .create_event(&event3)
        .expect("Failed to create event");
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), None, Some(&cat2))
            .unwrap(),
        vec![event2.clone()]
    );
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), None, Some(&cat1))
            .unwrap(),
        vec![event1.clone(), event3.clone()]
    );
    manager
        .merge_categories(&cat1, &cat2)
        .expect("Failed to merge");
    assert_eq!(
        manager
            .get_events(Some(&"title".to_string()), None, Some(&cat1))
            .unwrap(),
        vec![event1, event2, event3]
    );
    manager._reset_all().expect("Failed to delete db");
}