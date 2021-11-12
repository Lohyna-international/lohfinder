use crate::main;

use super::types::{Category, Event};
use chrono;
use serde::{de::IntoDeserializer, Deserialize, Serialize};
use serde_json::value;
use sled;
use std::collections::HashSet;
use tokio::runtime::Handle;

pub struct EventManager {
    db: sled::Db,
    events_name: String,
    organizers_name: String,
    categories_name: String,
}

impl EventManager {
    pub fn new(db_path: &String) -> EventManager {
        let manager = EventManager {
            db: sled::open(db_path).unwrap(),
            events_name: "events".to_string(),
            organizers_name: "organizers".to_string(),
            categories_name: "categories".to_string(),
        };

        let events = manager.db.open_tree(&manager.events_name).unwrap();
        let orgs = manager.db.open_tree(&manager.organizers_name).unwrap();
        let cats = manager.db.open_tree(&manager.categories_name).unwrap();
        events.set_merge_operator(EventManager::_merge);
        orgs.set_merge_operator(EventManager::_merge);
        cats.set_merge_operator(EventManager::_merge);
        manager
    }

    pub fn _merge(key: &[u8], old_v: Option<&[u8]>, new_v: &[u8]) -> Option<Vec<u8>> {
        let mut old_vec = match old_v {
            Some(v) => v.to_vec(),
            None => Vec::new(),
        };
        old_vec.append(&mut new_v.to_vec());
        Some(old_vec)
    }

    pub fn _compare_by_val(
        key: Option<&String>,
        a: &Event,
        b: &Event,
    ) -> Option<std::cmp::Ordering> {
        match key {
            Some(value) => match value.as_str() {
                "title" => a.title.partial_cmp(&b.title),
                "organizer" => a.organizer.partial_cmp(&b.organizer),
                "date_created" => a.date_created.partial_cmp(&b.date_created),
                "date_planning" => a.date_planning.partial_cmp(&b.date_planning),
                &_ => a.date_planning.partial_cmp(&b.date_planning),
            },
            None => a.date_planning.partial_cmp(&b.date_planning),
        }
    }

    pub fn _remove_id(data: &[u8], id: &[u8; 8]) -> Option<Vec<u8>> {
        let mut data_vec = data.to_vec();
        match data_vec.array_chunks::<8>().position(|&e| e.eq(id)) {
            Some(index) => {
                data_vec.drain(index..index + 8);
                Some(data_vec)
            }
            None => None,
        }
    }

    pub fn _events_to_vec(data: &Vec<&[u8]>) -> Vec<Event> {
        let mut res = Vec::new();
        data.iter().for_each(
            |o| match Event::from_json(&String::from_utf8(o.to_vec()).unwrap()) {
                Ok(parsed) => res.push(parsed),
                Err(e) => eprintln!("{:?}", e),
            },
        );
        res
    }

    pub fn _ids_to_vec(data: &[u8]) -> Vec<u64> {
        data.array_chunks::<8>()
            .map(|b| u64::from_be_bytes(*b))
            .collect::<Vec<u64>>()
    }

    pub fn _reset_all(self) -> Result<bool, Box<dyn std::error::Error>> {
        self.db.drop_tree(self.events_name)?;
        self.db.drop_tree(self.organizers_name)?;
        self.db.drop_tree(self.categories_name)?;
        Ok(true)
    }

    pub fn generate_id(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match self.db.generate_id() {
            Ok(id) => Ok(id),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn create_event(&self, new_event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        let id = new_event.id.to_be_bytes().clone();
        let org_id = new_event.organizer.to_be_bytes().clone();
        let cat = new_event.category.name.as_bytes();
        let events = self.db.open_tree(&self.events_name)?;
        events.insert(id.clone(), new_event.to_json().unwrap().as_bytes())?;
        let organizers = self.db.open_tree(&self.organizers_name)?;
        let categories = self.db.open_tree(&self.categories_name)?;
        if organizers.contains_key(&org_id)? {
            organizers.merge(&org_id, id)?;
        } else {
            organizers.insert(org_id, id.to_vec())?;
        }
        if categories.contains_key(&cat)? {
            categories.merge(&cat, id)?;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Unknown category",
            )));
        }
        Ok(())
    }

    pub fn delete_event(&self, event_id: &u64) -> Result<(), Box<dyn std::error::Error>> {
        let id = event_id.to_be_bytes();
        let events = self.db.open_tree(&self.events_name)?;
        if !events.contains_key(&id)? {
            return Ok(());
        }
        let event = Event::from_json(&String::from_utf8(events.get(id)?.unwrap().to_vec())?)?;
        let org_id = event.organizer.to_be_bytes().clone();
        let cat = event.category.name.as_bytes();
        events.remove(id)?;
        let organizers = self.db.open_tree(&self.organizers_name)?;
        let categories = self.db.open_tree(&self.categories_name)?;
        if organizers.contains_key(&org_id)? {
            let ids = organizers.get(&org_id)?.unwrap();
            let new_ids = EventManager::_remove_id(&ids[..], &id);
            if new_ids.is_some() {
                organizers.compare_and_swap(org_id, Some(ids), new_ids)??;
            }
        }
        if categories.contains_key(&cat)? {
            let ids = categories.get(&cat)?.unwrap();
            let new_ids = EventManager::_remove_id(&ids[..], &id);
            if new_ids.is_some() {
                categories.compare_and_swap(cat, Some(ids), new_ids)??;
            }
        }
        Ok(())
    }

    pub fn update_event(&self, new_event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        let id = new_event.id.to_be_bytes().clone();
        let events = self.db.open_tree(&self.events_name)?;
        events.insert(id, new_event.to_json().unwrap().as_bytes())?;
        Ok(())
    }

    pub fn get_event(&self, event_id: &u64) -> Result<Event, Box<dyn std::error::Error>> {
        let id = event_id.to_be_bytes();
        let events = self.db.open_tree(&self.events_name)?;
        let event = events.get(id)?;
        if event.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Event not found",
            )));
        }
        Event::from_json(&String::from_utf8(event.unwrap().to_vec())?)
    }

    pub fn get_events(
        &self,
        sort_key: Option<&String>,
        organizer: Option<u64>,
        category: Option<&String>,
    ) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let events = self.db.open_tree(&self.events_name)?;
        let mut return_events = Vec::new();
        if organizer.is_none() && category.is_none() {
            events.iter().for_each(|v| {
                if v.is_ok() {
                    let (_, event) = v.unwrap();
                    let parsed = Event::from_json(&String::from_utf8(event.to_vec()).unwrap());
                    if parsed.is_ok() {
                        return_events.push(parsed.unwrap());
                    }
                }
            })
        } else {
            let mut ids: Vec<u64> = Vec::new();
            if organizer.is_some() && category.is_some() {
                let organizers = self.db.open_tree(&self.organizers_name)?;
                let org_ids = organizers.get(organizer.unwrap().to_be_bytes())?;
                let categories = self.db.open_tree(&self.categories_name)?;
                let cat_ids = categories.get(category.unwrap().as_bytes())?;
                if org_ids.is_some() && cat_ids.is_some() {
                    let o_ids: HashSet<u64> = EventManager::_ids_to_vec(&org_ids.unwrap())
                        .into_iter()
                        .collect();
                    let c_ids: HashSet<u64> = EventManager::_ids_to_vec(&cat_ids.unwrap())
                        .into_iter()
                        .collect();
                    let mut res: Vec<u64> = o_ids.intersection(&c_ids).map(|t| t.clone()).collect();
                    ids.append(&mut res);
                } else if org_ids.is_some() {
                    ids.append(&mut EventManager::_ids_to_vec(&org_ids.unwrap()));
                } else if cat_ids.is_some() {
                    ids.append(&mut EventManager::_ids_to_vec(&cat_ids.unwrap()));
                }
            } else if organizer.is_some() {
                let organizers = self.db.open_tree(&self.organizers_name)?;
                let org_ids = organizers.get(organizer.unwrap().to_be_bytes())?;
                if org_ids.is_some() {
                    ids.append(&mut EventManager::_ids_to_vec(&org_ids.unwrap()));
                }
            } else if category.is_some() {
                let categories = self.db.open_tree(&self.categories_name)?;
                let cat_ids = categories.get(category.unwrap().as_bytes())?;
                if cat_ids.is_some() {
                    ids.append(&mut EventManager::_ids_to_vec(&cat_ids.unwrap()));
                }
            }
            let events_raw = ids.into_iter().map(|v| events.get(v.to_be_bytes()));
            events_raw.for_each(|v| {
                if v.is_ok() {
                    let parsed =
                        Event::from_json(&String::from_utf8(v.unwrap().unwrap().to_vec()).unwrap());
                    if parsed.is_ok() {
                        return_events.push(parsed.unwrap());
                    }
                }
            });
        }
        return_events.sort_by(|a, b| {
            EventManager::_compare_by_val(sort_key, a, b).unwrap_or(std::cmp::Ordering::Equal)
        });
        Ok(return_events)
    }

    pub fn create_category(&self, cat: &Category) -> Result<(), Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        if !categories.contains_key(&cat.name.as_bytes())? {
            categories.insert(&cat.name.as_bytes(), b"")?;
        }
        Ok(())
    }

    pub fn get_categories(&self) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        Ok(categories
            .iter()
            .map(|f| Category::new(String::from_utf8(f.unwrap().0.to_vec()).unwrap()))
            .collect())
    }

    pub fn delete_category(&self, cat: &Category) -> Result<(), Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        if categories.contains_key(&cat.name.as_bytes())? {
            categories.remove(&cat.name.as_bytes())?;
        } else {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Category not found",
            )));
        }
        Ok(())
    }

    pub fn merge_categories(
        &self,
        cat1: &Category,
        cat2: &Category,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        let id1 = cat1.name.as_bytes();
        let id2 = cat2.name.as_bytes();
        let mut to_add: Vec<u8> = Vec::new();
        if categories.contains_key(id2)? {
            match categories.get(id2)? {
                Some(values) => {
                    to_add.append(&mut values.to_vec());
                }
                None => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Category not found",
                    )))
                }
            }
        }
        if categories.contains_key(id1)? {
            categories.merge(id1, to_add)?;
        } else {
            categories.insert(id1, to_add)?;
        }
        self.delete_category(cat2)
    }
}
