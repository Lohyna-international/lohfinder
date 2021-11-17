use serde::{Deserialize, Serialize};

pub type Category = String;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Event {
    pub id: u64,
    pub title: String,
    pub cover: String,
    pub description: String,
    pub organizer: u64,
    pub date_created: i64,
    pub date_planning: i64,
    pub category: Category,
}

impl Event {
    pub fn from_json(serialized: &String) -> Result<Event, Box<dyn std::error::Error>> {
        match serde_json::from_str::<Event>(&serialized) {
            Ok(event) => Ok(event),
            Err(e) => Err(Box::new(e)),
        }
    }

    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match serde_json::to_string(self) {
            Ok(json) => Ok(json),
            Err(e) => Err(Box::new(e)),
        }
    }
}
