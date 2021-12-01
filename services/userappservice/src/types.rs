use serde::{Deserialize, Serialize};
use cloud_pubsub::*;
use super::*;

#[derive(Serialize, Deserialize,Debug, PartialEq, Eq, Clone)]
pub enum ApplicationStatus {
    Created,
    Registered,
    Approved,
    Rejected,
    Postponed
}



#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Application {
    pub id : u64,
    pub user_id: u64,
    pub event_id: u64,
    pub status : ApplicationStatus
}

impl Application {
    pub fn from_json(serialized: &String) -> Result<Application, Box<dyn std::error::Error>> {
        match serde_json::from_str::<Application>(&serialized) {
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

    pub fn set_status(&mut self, status : ApplicationStatus) {
        self.status = status;
    }
}

fn format_string(inp : &str) -> String {
    serde_json::from_str::<String>(inp).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub data : String
}

impl Message {
    pub fn new(data : String) -> Message{
        Message {data}
    }
}

impl FromPubSubMessage for Message {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str::<String>(&String::from_utf8(bytes).unwrap()) {
                Ok(m) => Ok(Message::new(m)),
                Err(e) => Err(error::Error::from(e)),
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: u64,
    pub code: u16,
    pub message: String,
}

impl Status {
    pub fn ok(id: u64) -> Status {
        Status {
            id: id,
            code: 200,
            message: "Ok".to_string(),
        }
    }
    pub fn new(id: u64, code: u16, message: String) -> Status {
        Status {
            id: id,
            code: code,
            message: message,
        }
    }
}

impl FromPubSubMessage for Status {
    fn from(message: EncodedMessage) -> Result<Self, error::Error> {
        match message.decode() {
            Ok(bytes) => match serde_json::from_str::<Self>(&format_string(&String::from_utf8(bytes).unwrap())) {
                Ok(m) => Ok(m),
                Err(e) => Err(error::Error::from(e)),
            },
            Err(e) => Err(error::Error::from(e)),
        }
    }
}

pub trait PubSubCallBack {
    fn action(
        &self,
        manager: &data_manager::EventManager,
    ) -> Result<Status, Box<dyn std::error::Error>>;

    fn error_message(&self, error: Box<dyn std::error::Error>) -> String;

    fn message_id(&self) -> u64;

    fn handle(&self, manager: &data_manager::EventManager) -> Status {
        match self.action(manager) {
            Ok(s) => s,
            Err(e) => Status::new(self.message_id().clone(), 404, self.error_message(e)),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateAppMessage {
    pub message_id : u64,
    pub id : Option<u64>,
    pub user_id : u64,
    pub event_id : u64,
}

impl PubSubCallBack for CreateAppMessage {
    fn action(&self, manager: &data_manager::EventManager) -> Result<Status, Box<dyn std::error::Error>> {
        let id = self.id.unwrap_or(manager.generate_id());
        let app = Application {
            id : id,
            user_id : self.user_id,
            event_id : self.event_id,
            status : ApplicationStatus::Created
        };
        manager.create_app(&app)?;
        Ok(Status::ok(self.message_id))
    }

    fn error_message(&self, error: Box<dyn std::error::Error>) -> String {
        format!("Failed to create app : {:?}", error)
    }

    fn message_id(&self) -> u64 {
        self.message_id
    }
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStatusMessage {
    pub message_id : u64,
    pub id : u64,
    pub status : ApplicationStatus,
}

impl PubSubCallBack for UpdateStatusMessage {
    fn action(&self, manager: &data_manager::EventManager) -> Result<Status, Box<dyn std::error::Error>> {
        manager.update_status(self.id, self.status.clone())?;
        Ok(Status::ok(self.message_id))
    }

    fn error_message(&self, error: Box<dyn std::error::Error>) -> String {
        format!("Failed to update status : {:?}", error)
    }

    fn message_id(&self) -> u64 {
        self.message_id
    }
}

#[derive(Serialize, Deserialize)]
pub enum GetFor {
    User,
    Event
}

#[derive(Serialize, Deserialize)]
pub struct GetAppsMessage {
    pub message_id : u64,
    pub id : u64,
    pub get_for : GetFor,
}

#[derive(Serialize, Deserialize)]
struct AppsList {
    apps : Vec<Application>
}

impl PubSubCallBack for GetAppsMessage {
    fn action(&self, manager: &data_manager::EventManager) -> Result<Status, Box<dyn std::error::Error>> {
        let apps = match self.get_for {
            GetFor::User => manager.get_for_user(self.id),
            GetFor::Event => manager.get_for_event(self.id),
        }?;
        let applist = AppsList {apps};
        Ok(Status::new(self.message_id, 200, serde_json::to_string(&applist)?))
    }

    fn error_message(&self, error: Box<dyn std::error::Error>) -> String {
        format!("Failed to create app : {:?}", error)
    }

    fn message_id(&self) -> u64 {
        self.message_id
    }
}