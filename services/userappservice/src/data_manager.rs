use super::types::*;
use sled;

pub fn merge(key: &[u8], old_v: Option<&[u8]>, new_v: &[u8]) -> Option<Vec<u8>> {
    let mut old_vec = match old_v {
        Some(v) => v.to_vec(),
        None => Vec::new(),
    };
    old_vec.append(&mut new_v.to_vec());
    Some(old_vec)
}

pub struct EventManager {
    db: sled::Db,
    all_apps: String,
    users_apps: String,
    events_apps: String,
}

impl EventManager {
    pub fn new(db_path: &String) -> Result<EventManager, Box<dyn std::error::Error>> {
        let manager = EventManager {
            db: sled::open(db_path)?,
            all_apps: "apps".to_string(),
            users_apps: "userapps".to_string(),
            events_apps: "eventapps".to_string(),
        };
        let apps = manager.db.open_tree(&manager.all_apps)?;
        let users = manager.db.open_tree(&manager.users_apps)?;
        let events = manager.db.open_tree(&manager.events_apps)?;
        apps.set_merge_operator(merge);
        users.set_merge_operator(merge);
        events.set_merge_operator(merge);
        Ok(manager)
    }

    pub fn generate_id(&self) -> u64 {
        self.db.generate_id().unwrap_or(42)
    }

    fn _ids_to_vec(data: &[u8]) -> Vec<u64> {
        data.array_chunks::<8>()
            .map(|b| u64::from_be_bytes(*b))
            .collect::<Vec<u64>>()
    }

    pub fn reset_all(&self) -> Result<bool, Box<dyn std::error::Error>> {
        self.db.drop_tree(self.all_apps.clone())?;
        self.db.drop_tree(self.users_apps.clone())?;
        self.db.drop_tree(self.events_apps.clone())?;
        Ok(true)
    }

    pub fn create_app(&self, new_app: &Application) -> Result<(), Box<dyn std::error::Error>> {
        let id = new_app.id.to_be_bytes();
        let user_id = new_app.user_id.to_be_bytes();
        let event_id = new_app.event_id.to_be_bytes();
        let apps = self.db.open_tree(&self.all_apps)?;
        if apps.contains_key(&id)? {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Application already created",
            )));
        }
        apps.insert(&id, new_app.to_json()?.as_bytes());
        let users = self.db.open_tree(&self.users_apps)?;
        let events = self.db.open_tree(&self.events_apps)?;
        if users.contains_key(&user_id)? {
            users.merge(&user_id, id)?;
        } else {
            users.insert(user_id, id.to_vec())?;
        }
        if events.contains_key(&event_id)? {
            events.merge(&event_id, id)?;
        } else {
            events.insert(event_id, id.to_vec())?;
        }
        Ok(())
    }

    pub fn update_status(
        &self,
        id: u64,
        status: ApplicationStatus,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = id.to_be_bytes();
        let apps = self.db.open_tree(&self.all_apps)?;
        if !apps.contains_key(&id)? {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "App not found!",
            )));
        }
        match apps.get(&id)? {
            Some(old_app) => {
                let mut app = Application::from_json(&String::from_utf8(old_app.to_vec())?)?;
                app.set_status(status);
                apps.compare_and_swap(&id, Some(old_app), Some(app.to_json()?.as_bytes()))??;
                Ok(())
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "App not found!",
            ))),
        }
    }

    fn get_apps_for(
        &self,
        tree: sled::Tree,
        id: &[u8],
    ) -> Result<Vec<Application>, Box<dyn std::error::Error>> {
        let ids = match tree.get(&id)? {
            Some(i) => EventManager::_ids_to_vec(&i),
            None => Vec::new(),
        };
        let apps = self.db.open_tree(&self.all_apps)?;
        let res = ids
            .iter()
            .filter_map(|id| apps.get(&id.to_be_bytes()).ok())
            .filter_map(|a| {
                a.and_then(|raw_app| {
                    Application::from_json(&String::from_utf8(raw_app.to_vec()).unwrap()).ok()
                })
            })
            .collect();
        Ok(res)
    }

    pub fn get_for_user(
        &self,
        user_id: u64,
    ) -> Result<Vec<Application>, Box<dyn std::error::Error>> {
        let id = user_id.to_be_bytes();
        let users = self.db.open_tree(&self.users_apps)?;
        self.get_apps_for(users, &id)
    }

    pub fn get_for_event(
        &self,
        event_id: u64,
    ) -> Result<Vec<Application>, Box<dyn std::error::Error>> {
        let id = event_id.to_be_bytes();
        let events = self.db.open_tree(&self.events_apps)?;
        self.get_apps_for(events, &id)
    }
}
