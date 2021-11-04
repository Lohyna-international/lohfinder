use chrono::{DateTime, Duration}
use chrono::prelude::*
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Event
{
    pub id : u64,
    title : String,
    cover : String,
    description : String,
    organizer : u64,
    date_created : DateTime<Utc>,
    date_planning : DateTime<Utc>,
}

impl Event {
    pub fn new() -> Event
    {
        Event {
            id : -1,
            title : "Unnamed".to_string(),
            cover : "N/A".to_string(),
            description : "Empty".to_string(),
            organizer : -1,
            date_created : Utc::now(),
            date_planning : Utc::now() + Duration::days(10)
        }
    }

    pub fn from_json(serialized : &String) -> Result<Event, Box<dyn std::error::Error>>
    {
        match serde_json::from_str<Event>(&serialized) {
            Ok(event) -> event,
            Err(e) -> Err(e)
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Category
{
    name : String
}

impl Category {
    pub fn new(name : String) -> Category {
        Category { name : name }
    }

    pub fn from_json(serialized : &String) -> Result<Category, Box<dyn std::error::Error>>
    {
        match serde_json::from_str<Category>(&serialized) {
            Ok(cat) -> cat,
            Err(e) -> Err(e)
        }
    }
}