use crate::types::Category;

use super::*;

#[test]
fn merge_test()
{
    assert_eq!(data_manager::EventManager::_merge(b"", Some(b"ab"), b"cd"),Some((b"abcd").to_vec()));
}

#[test]
fn compare_by_val_test()
{
    let event1 = types::Event {
        id : 1,
        title : "Event1".to_string(),
        cover : "N/A".to_string(),
        description : "Empty".to_string(),
        organizer : 3,
        date_created : chrono::Utc::now().timestamp(),
        date_planning : (chrono::Utc::now() + chrono::Duration::days(2)).timestamp(),
        category : Category { name : "N/A".to_string()}
    };
    let event3 = types::Event {
        id : 3,
        title : "Event3".to_string(),
        cover : "N/A".to_string(),
        description : "Empty".to_string(),
        organizer : 2,
        date_created : chrono::Utc::now().timestamp() + 10,
        date_planning : (chrono::Utc::now() + chrono::Duration::days(1)).timestamp(),
        category : Category { name : "N/A".to_string()}
    };

    assert_eq!(data_manager::EventManager::_compare_by_val(None, &event1, &event3), Some(std::cmp::Ordering::Greater));
    assert_eq!(data_manager::EventManager::_compare_by_val(Some(&"title".to_string()), &event1, &event3), Some(std::cmp::Ordering::Less));
    assert_eq!(data_manager::EventManager::_compare_by_val(Some(&"organizer".to_string()), &event1, &event3), Some(std::cmp::Ordering::Greater));
    assert_eq!(data_manager::EventManager::_compare_by_val(Some(&"date_created".to_string()), &event1, &event3), Some(std::cmp::Ordering::Less));
    assert_eq!(data_manager::EventManager::_compare_by_val(Some(&"date_planning".to_string()), &event1, &event3), Some(std::cmp::Ordering::Greater));
    assert_eq!(data_manager::EventManager::_compare_by_val(Some(&"non_exisint".to_string()), &event1, &event3), Some(std::cmp::Ordering::Greater));
}

#[test]
fn remove_id_test()
{
    let mut ids = (42 as u64).to_be_bytes().to_vec();
    ids.append(&mut (43 as u64).to_be_bytes().to_vec());
    let data = &ids[..];
    assert_eq!(data_manager::EventManager::_remove_id(data, &(42 as u64).to_be_bytes()), Some((43 as u64).to_be_bytes().to_vec()));
    assert_eq!(data_manager::EventManager::_remove_id(data, &(1 as u64).to_be_bytes()), None);
}

#[test]
fn events_to_vec_test()
{
    let event1 = types::Event {
        id : 1,
        title : "Event1".to_string(),
        cover : "N/A".to_string(),
        description : "Empty".to_string(),
        organizer : 3,
        date_created : chrono::Utc::now().timestamp(),
        date_planning : (chrono::Utc::now() + chrono::Duration::days(2)).timestamp(),
        category : Category { name : "N/A".to_string()}
    };
    let event3 = types::Event {
        id : 3,
        title : "Event3".to_string(),
        cover : "N/A".to_string(),
        description : "Empty".to_string(),
        organizer : 2,
        date_created : chrono::Utc::now().timestamp() + 10,
        date_planning : (chrono::Utc::now() + chrono::Duration::days(1)).timestamp(),
        category : Category { name : "N/A".to_string()}
    };
    assert_eq!(data_manager::EventManager::_events_to_vec(&vec![event1.to_json().unwrap().as_bytes(), event3.to_json().unwrap().as_bytes()]), vec![event1, event3]);
}   

#[test]
fn ids_to_vec_test()
{
    assert_eq!(data_manager::EventManager::_ids_to_vec(&(42 as u64).to_be_bytes()), vec![42 as u64]);
    assert_eq!(data_manager::EventManager::_ids_to_vec(b"42"), Vec::<u64>::new());
}

#[test]
fn create_event_test()
{
    let manager = data_manager::EventManager::new(&"./db".to_string());
    let event1 = types::Event {
        id : 1,
        title : "Event1".to_string(),
        cover : "N/A".to_string(),
        description : "Empty".to_string(),
        organizer : 3,
        date_created : chrono::Utc::now().timestamp(),
        date_planning : (chrono::Utc::now() + chrono::Duration::days(2)).timestamp(),
        category : Category { name : "N/A".to_string()}
    };
    manager.create_category(&Category { name : "N/A".to_string()}).expect("Failed to create category!");
    manager.create_event(&event1).expect("Failed to create event");
    let event = manager.get_event(&1).expect("Failed to get event");
    assert_eq!(event,event1);
    manager._reset_all().expect("Failed to delete db");
}