use serde::{Serialize, Deserialize};
use chrono;
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Category
{
    pub name : String
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Event
{
    pub id : u64,
    pub title : String,
    pub cover : String,
    pub description : String,
    pub organizer : u64,
    pub date_created : i64,
    pub date_planning : i64,
    pub category : Category
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
            date_created : chrono::Utc::now().timestamp(),
            date_planning : (chrono::Utc::now() + chrono::Duration::days(1)).timestamp(),
            category : Category { name : "N/A".to_string()}
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
