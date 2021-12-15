use super::types::*;
use sled;
use std::collections::HashSet;

pub fn merge(_key: &[u8], old_v: Option<&[u8]>, new_v: &[u8]) -> Option<Vec<u8>> {
    let mut old_vec = match old_v {
        Some(v) => v.to_vec(),
        None => Vec::new(),
    };
    old_vec.append(&mut new_v.to_vec());
    Some(old_vec)
}

pub struct EventManager {
    db: sled::Db,
    events_name: String,
    organizers_name: String,
    categories_name: String,
}

impl EventManager {
    pub fn new(db_path: &String) -> Result<EventManager, Box<dyn std::error::Error>> {
        let manager = EventManager {
            db: sled::open(db_path)?,
            events_name: "events".to_string(),
            organizers_name: "organizers".to_string(),
            categories_name: "categories".to_string(),
        };

        let events = manager.db.open_tree(&manager.events_name)?;
        let orgs = manager.db.open_tree(&manager.organizers_name)?;
        let cats = manager.db.open_tree(&manager.categories_name)?;
        events.set_merge_operator(merge);
        orgs.set_merge_operator(merge);
        cats.set_merge_operator(merge);
        Ok(manager)
    }

    fn _compare_by_val(
        key: Option<&EventSortKey>,
        a: &Event,
        b: &Event,
    ) -> Option<std::cmp::Ordering> {
        match key {
            Some(value) => match value {
                &EventSortKey::Title => a.title.partial_cmp(&b.title),
                &EventSortKey::Organizer => a.organizer.partial_cmp(&b.organizer),
                &EventSortKey::Created => a.date_created.partial_cmp(&b.date_created),
                &EventSortKey::Planning | &EventSortKey::Default => {
                    a.date_planning.partial_cmp(&b.date_planning)
                }
            },
            None => a.date_planning.partial_cmp(&b.date_planning),
        }
    }

    fn _remove_id(data: &[u8], id: &[u8; 8]) -> Option<Vec<u8>> {
        let mut data_vec = data.to_vec();
        match data_vec.array_chunks::<8>().position(|&e| e.eq(id)) {
            Some(index) => {
                data_vec.drain(index..index + 8);
                Some(data_vec)
            }
            None => None,
        }
    }

    fn _events_to_vec(data: &Vec<&[u8]>) -> Vec<Event> {
        let mut res = Vec::new();
        data.iter().for_each(
            |o| match Event::from_json(&String::from_utf8(o.to_vec()).unwrap()) {
                Ok(parsed) => res.push(parsed),
                Err(e) => eprintln!("{:?}", e),
            },
        );
        res
    }

    fn _ids_to_vec(data: &[u8]) -> Vec<u64> {
        data.array_chunks::<8>()
            .map(|b| u64::from_be_bytes(*b))
            .collect::<Vec<u64>>()
    }

    pub fn reset_all(&self) -> Result<bool, Box<dyn std::error::Error>> {
        self.db.drop_tree(self.events_name.clone())?;
        self.db.drop_tree(self.organizers_name.clone())?;
        self.db.drop_tree(self.categories_name.clone())?;
        Ok(true)
    }

    pub fn generate_id(&self) -> Result<u64, Box<dyn std::error::Error>> {
        match self.db.generate_id() {
            Ok(id) => Ok(id),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn create_event(&self, new_event: &Event) -> Result<(), Box<dyn std::error::Error>> {
        let id = new_event.id.to_be_bytes();
        let org_id = new_event.organizer.to_be_bytes();
        let cat = new_event.category.as_bytes();
        let events = self.db.open_tree(&self.events_name)?;
        events.insert(id, new_event.to_json()?.as_bytes())?;
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
        let org_id = event.organizer.to_be_bytes();
        let cat = event.category.as_bytes();
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
        let id = new_event.id.to_be_bytes();
        let events = self.db.open_tree(&self.events_name)?;
        events.insert(id, new_event.to_json().unwrap().as_bytes())?;
        Ok(())
    }

    pub fn get_event(&self, event_id: &u64) -> Result<Event, Box<dyn std::error::Error>> {
        let id = event_id.to_be_bytes();
        let events = self.db.open_tree(&self.events_name)?;
        let event = events.get(id)?;
        match event {
            Some(e) => Event::from_json(&String::from_utf8(e.to_vec())?),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Event not found",
            ))),
        }
    }

    fn _events_for_organizer(
        &self,
        organizer: u64,
    ) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        let organizers = self.db.open_tree(&self.organizers_name)?;
        let org_ids = organizers.get(organizer.to_be_bytes())?;
        match org_ids {
            Some(ids) => Ok(EventManager::_ids_to_vec(&ids)),
            None => Ok(Vec::new()),
        }
    }

    fn _events_for_category(
        &self,
        category: &String,
    ) -> Result<Vec<u64>, Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        let cat_ids = categories.get(category.as_bytes())?;
        match cat_ids {
            Some(ids) => Ok(EventManager::_ids_to_vec(&ids)),
            None => Ok(Vec::new()),
        }
    }

    fn _get_all_events(&self) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let events = self.db.open_tree(&self.events_name)?;
        Ok(events
            .iter()
            .filter_map(|f| {
                if let Ok((_, event)) = f {
                    Event::from_json(&String::from_utf8(event.to_vec()).unwrap()).ok()
                } else {
                    None
                }
            })
            .collect())
    }

    pub fn get_events(
        &self,
        sort_key: Option<&EventSortKey>,
        organizer: Option<u64>,
        category: Option<&String>,
    ) -> Result<Vec<Event>, Box<dyn std::error::Error>> {
        let events = self.db.open_tree(&self.events_name)?;
        let mut result_events = Vec::new();
        if organizer.is_none() && category.is_none() {
            result_events.append(&mut self._get_all_events()?);
        } else {
            let o: Option<HashSet<u64>> = match organizer {
                Some(org) => Some(self._events_for_organizer(org)?.into_iter().collect()),
                None => None,
            };
            let c: Option<HashSet<u64>> = match category {
                Some(cat) => Some(self._events_for_category(cat)?.into_iter().collect()),
                None => None,
            };
            let intersect_events = match (c, o) {
                (Some(c), Some(o)) => (&c & &o),
                (None, Some(o)) => o,
                (Some(c), None) => c,
                (None, None) => HashSet::new(),
            };
            let mut result: Vec<Event> = intersect_events
                .iter()
                .filter_map(|v| {
                    if let Ok(Some(f)) = events.get(v.to_be_bytes()) {
                        return Event::from_json(&String::from_utf8(f.to_vec()).unwrap()).ok();
                    }
                    None
                })
                .collect();
            result_events.append(&mut result);
        }
        result_events.sort_by(|a, b| {
            EventManager::_compare_by_val(sort_key, a, b).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(result_events)
    }

    pub fn create_category(&self, cat: &Category) -> Result<(), Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        if !categories.contains_key(&cat.as_bytes())? {
            categories.insert(&cat.as_bytes(), b"")?;
        }
        Ok(())
    }

    pub fn get_categories(&self) -> Result<Vec<Category>, Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        Ok(categories
            .iter()
            .filter_map(|f| f.ok())
            .filter_map(|f| String::from_utf8(f.0.to_vec()).ok())
            .collect())
    }

    pub fn delete_category(&self, cat: &Category) -> Result<(), Box<dyn std::error::Error>> {
        let categories = self.db.open_tree(&self.categories_name)?;
        if categories.contains_key(&cat.as_bytes())? {
            categories.remove(&cat.as_bytes())?;
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
        let id1 = cat1.as_bytes();
        let id2 = cat2.as_bytes();
        let mut to_add: Vec<u8> = Vec::new();
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
        if categories.contains_key(id1)? {
            categories.merge(id1, to_add)?;
        } else {
            categories.insert(id1, to_add)?;
        }
        self.delete_category(cat2)
    }
}
