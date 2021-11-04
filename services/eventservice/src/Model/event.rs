use sled
use types

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
            db : sled::open(dp_path).unwrap()
        }
    }

    pub fn create_event(new_event : Event) -> Result<(), Box<dyn std::error::Error>>
    {
        db.transaction(|tx_db| {
            let id = new_event.id.to_be_bytes().clone();
            let org_id = new_event.organizer.to_be_bytes().clone();
            let events = tx_db.open("events")?;
            let organizers = tx_dp.open("organizers")?;
            events.insert(id.clone() , new_event)?;
            if organizers.contains_key(&org_id)? {
                organizers.get(&org_id).append(id.clone());
            }
            else {
                organizers.insert(org_id, vec![id.clone()]);
            }
            Ok(())
        })?;
        Ok(())
    }
}