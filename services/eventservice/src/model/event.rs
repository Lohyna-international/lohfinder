use std::convert::TryInto;

use sled;
use chrono::{DateTime, Duration};
use chrono::prelude::*;
use serde::{Serialize, Deserialize};
use sled::transaction::TransactionalTrees;

#[derive(Serialize, Deserialize, Debug)]
pub struct Event
{
    pub id : u64,
    title : String,
    cover : String,
    description : String,
    organizer : u64,
    date_created : String,
    date_planning : String,
}

impl Event {
    pub fn new() -> Event
    {
        Event {
            id : 1,
            title : "Unnamed".to_string(),
            cover : "N/A".to_string(),
            description : "Empty".to_string(),
            organizer : 1,
            date_created : "".to_string(),
            date_planning : "".to_string()
        }
    }

    pub fn from_json(serialized : &String) -> Result<Event, Box<dyn std::error::Error>>
    {
        match serde_json::from_str::<Event>(&serialized) {
            Ok(event) => Ok(event),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>>
    {
        match serde_json::to_string(self)
        {
            Ok(json) => Ok(json),
            Err(e) => Err(Box::new(e))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category
{
    name : String
}

impl Category {
    pub fn new(name : String) -> Category {
        Category { name : name }
    }

    pub fn from_json(serialized : &String) -> Result<Category, Box<dyn std::error::Error>>
    {
        match serde_json::from_str::<Category>(&serialized) {
            Ok(cat) => Ok(cat),
            Err(e) => Err(Box::new(e))
        }
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>>
    {
        match serde_json::to_string(self)
        {
            Ok(json) => Ok(json),
            Err(e) => Err(Box::new(e))
        }
    }
}

struct EventManager
{
    db : sled::Db
}

impl EventManager
{
    pub fn new(db_path : &String) -> EventManager 
    {
        EventManager
        {
            db : sled::open(db_path).unwrap()
        }
    }

    fn remove_id(data : &[u8], id : &[u8;8]) -> Option<Vec<u8>>
    {
        let mut data_vec = data.to_vec();
        match data_vec.array_chunks::<8>().position(|&e| e.eq(id))
        {
            Some(index) => {data_vec.drain(index..index+8); Some(data_vec)},
            None => None
        }
    }

    pub fn create_event(&self, new_event : Event) -> Result<(), Box<dyn std::error::Error>>
    {
        let id = new_event.id.to_be_bytes().clone();
        let org_id = new_event.organizer.to_be_bytes().clone();
        let events = self.db.open_tree("events")?;
        events.insert(id.clone() , new_event.to_json().unwrap().as_bytes())?;
        let organizers = self.db.open_tree("organizers")?;
        if organizers.contains_key(&org_id)? {
            organizers.merge(&org_id, id)?;
        }
        else {
            organizers.insert(org_id,id.to_vec())?;
        }
        Ok(())
    }

    pub fn delete_event(&self, event_id : &String) -> Result<(), Box<dyn std::error::Error>>
    {
        let id = event_id.as_bytes();
        let events = self.db.open_tree("events")?;
        let event = Event::from_json(&String::from_utf8(events.get(id)?.unwrap().to_vec())?)?;
        let org_id = event.organizer.to_be_bytes().clone();
        events.remove(id)?;
        let organizers = self.db.open_tree("organizers")?;
        if organizers.contains_key(&org_id)? {
            let ids = organizers.get(&org_id)?.unwrap();
            let new_ids = EventManager::remove_id(&ids[..], &org_id);
            if new_ids.is_some() 
            {
                organizers.compare_and_swap(org_id, Some(ids), new_ids)??;
            }
        }
        Ok(())
    }

    pub fn update_event(&self, new_event : Event) -> Result<(), Box<dyn std::error::Error>>
    {
        let id = new_event.id.to_be_bytes().clone();
        let events = self.db.open_tree("events")?;
        events.compare_and_swap(id, None, Some(new_event.to_json().unwrap().as_bytes()));
        Ok(())
    }

    pub fn get_event(&self, event_id : &String) -> Result<Event, Box<dyn std::error::Error>>
    {
        let id = event_id.as_bytes();
        let events = self.db.open_tree("events")?;
        Event::from_json(&String::from_utf8(events.get(id)?.unwrap().to_vec())?)
    }

    pub fn get_events(&self, sort_key : Option<&String>, organizer : Option<&String>, category : Option<&String>) -> Result<Vec<Event>, Box<dyn std::error::Error>>
    {

    }
}