use super::types::{Category, Event};
use super::*;
use chrono;
use cloud_pubsub::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Status {
    pub code : u16,
    pub message : String
}

impl Status {
    pub fn ok() -> Status {
        Status {
            code : 200,
            message : "Ok".to_string()
        }
    }
    pub fn new(code : u16, message : String) -> Status {
        Status { code : code, message : message}
    }
}

pub trait PubSubCallBack {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>>;

    fn error_message(error : Box<dyn std::error::Error>) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct CreateEventMessage {
    pub id: Option<u64>,
    pub title: String,
    pub cover: String,
    pub description: String,
    pub organizer: u64,
    pub date_planning: i64,
    pub category: Category,
}

impl PubSubCallBack for CreateEventMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        let id = self.id.unwrap_or(manager.generate_id()?);
        let now = chrono::Utc::now().timestamp();
        let event = Event {
            id: id,
            title: self.title.clone(),
            cover: self.cover.clone(),
            description: self.description.clone(),
            organizer: self.organizer,
            date_created: now,
            date_planning: self.date_planning,
            category: self.category.clone(),
        };
        manager.create_event(&event)?;
        Ok(Status::ok())
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to create event, error : {:?}", error)}
}

impl FromPubSubMessage for CreateEventMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct DeleteEventMessage {
    pub id: u64,
}

impl PubSubCallBack for DeleteEventMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        manager.delete_event(&self.id)?;
        Ok(Status::ok())
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to delete event, error : {:?}", error)}
}

impl FromPubSubMessage for DeleteEventMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateEventMessage {
    pub id: u64,
    pub title: String,
    pub cover: String,
    pub description: String,
    pub organizer: u64,
    pub date_created : i64,
    pub date_planning: i64,
    pub category: Category,
}

impl PubSubCallBack for UpdateEventMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        let event = Event {
            id: self.id,
            title: self.title.clone(),
            cover: self.cover.clone(),
            description: self.description.clone(),
            organizer: self.organizer,
            date_created: self.date_created,
            date_planning: self.date_planning,
            category: self.category.clone(),
        };
        manager.update_event(&event)?;
        Ok(Status::ok())
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to update event, error : {:?}", error)}
}

impl FromPubSubMessage for UpdateEventMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GetEventMessage {
    pub id: u64,
}

impl PubSubCallBack for GetEventMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        let event = manager.get_event(&self.id)?;
        Ok(Status::new(200,event.to_json()?))
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to get event, error : {:?}", error)}
}

impl FromPubSubMessage for GetEventMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct EventsList {
    pub events : Vec<Event>
}

#[derive(Serialize, Deserialize)]
pub struct GetEventsMessage {
    pub sort_key: Option<String>,
    pub organizer: Option<u64>,
    pub category: Option<String>,
}

impl PubSubCallBack for GetEventsMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        let events = manager.get_events(self.sort_key.as_ref(), self.organizer, self.category.as_ref())?;
        Ok(Status::new(200,serde_json::to_string(&EventsList{events : events})?))
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to get events, error : {:?}", error)}
}

impl FromPubSubMessage for GetEventsMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateCategoryMessage {
    pub name: Category,
}

impl PubSubCallBack for CreateCategoryMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        manager.create_category(&self.name)?;
        Ok(Status::ok())
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to create category, error : {:?}", error)}
}

impl FromPubSubMessage for CreateCategoryMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteCategoryMessage {
    pub name: Category,
}

impl PubSubCallBack for DeleteCategoryMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        manager.delete_category(&self.name)?;
        Ok(Status::ok())
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to delete category, error : {:?}", error)}
}

impl FromPubSubMessage for DeleteCategoryMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct CategoriesList {
    pub categories : Vec<Category>
}

#[derive(Serialize, Deserialize)]
pub struct GetCategoriesMessage {
}

impl PubSubCallBack for GetCategoriesMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        let cats = manager.get_categories()?;
        Ok(Status::new(200, serde_json::to_string(&cats)?))
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to get categories, error : {:?}", error)}
}

impl FromPubSubMessage for GetCategoriesMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MergeCategoriesMessage {
    pub merge_into: Category,
    pub merge_from: Category
}

impl PubSubCallBack for MergeCategoriesMessage {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>> {
        manager.merge_categories(&self.merge_into, &self.merge_from)?;
        Ok(Status::ok())
    }

    fn error_message(error : Box<dyn std::error::Error>) -> String { format!("Failed to merge categories, error : {:?}", error)}
}

impl FromPubSubMessage for MergeCategoriesMessage {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str(&String::from_utf8_lossy(&bytes)) {
                Ok(m) => m,
                Err(e) => Err(error::Error::from(e))
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}