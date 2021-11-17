use crate::data_manager::EventManager;
use super::pstypes::*;
use cloud_pubsub::*;
use std::collections::HashMap;
use std::sync::Arc;

pub struct PubSubClient {
    topics: HashMap<String, Arc<Topic>>,
    subs: HashMap<String, Subscription>,
    manager: EventManager,
}

impl PubSubClient {
    pub async fn new(
        keys: String,
        manager: EventManager,
    ) -> Result<PubSubClient, Box<dyn std::error::Error>> {
        let client = Client::new(keys).await?;
        let topics_names = vec![
            "event_create",
            "event_delete",
            "event_update",
            "event_get",
            "events",
            "categories",
            "category_create",
            "category_delete",
            "category_merge",
        ];
        let mut topics = HashMap::new();
        let mut subs = HashMap::new();
        topics_names.iter().map(|f| f.to_string()).for_each(|f| {
            let topic = Arc::new(client.topic(f.clone()));
            topics.insert(f.clone(), topic);
            let sub = client.subscribe(f.clone());
            subs.insert(f, sub);
        });
        topics.insert(
            String::from("results"),
            Arc::new(client.topic("results".to_string())),
        );
        Ok(PubSubClient {
            topics,
            subs,
            manager,
        })
    }

    pub fn clean_db(&self) -> bool {
        self.manager._reset_all().unwrap_or(false)
    }

    pub async fn return_results(
        &self,
        results: Vec<Status>,
    ) -> Result<u32, Box<dyn std::error::Error>> {
        let mut failed = 0u32;
        for status in results {
            if self
                .topics
                .get(&String::from("results"))
                .unwrap()
                .publish(serde_json::to_string(&status)?)
                .await
                .is_err()
            {
                failed += 1;
            }
        }
        Ok(failed)
    }

    async fn acknowledge(&self, ids: Vec<String>, sub: &Subscription) {
        sub.acknowledge_messages(ids).await
    }

    fn work_messages<T>(
        &self,
        messages: Vec<(Result<T, error::Error>, String)>,
    ) -> (Vec<Status>, Vec<String>)
    where
        T: PubSubCallBack,
    {
        let mut res = Vec::new();
        let mut ids = Vec::new();
        messages.iter().for_each(|(item, id)| {
            if item.is_ok() {
                res.push(item.as_ref().unwrap().handle(&self.manager));
                ids.push(id.clone());
            }
            else {
                println!("Failed to parse message : {:?}", item.as_ref().err().unwrap());
            }
        });
        (res, ids)
    }

    pub async fn handle_messages(&self) -> Result<Vec<Status>, Box<dyn std::error::Error>> {
        let mut all_statuses = Vec::new();
        for (name, sub) in &self.subs {
            println!("Scanning {}", &name);
            let (mut statuses, ids) = match name.as_str() {
                "event_create" => {
                    self.work_messages(sub.get_messages::<CreateEventMessage>().await?)
                }
                "event_delete" => {
                    self.work_messages(sub.get_messages::<DeleteEventMessage>().await?)
                }
                "event_update" => {
                    self.work_messages(sub.get_messages::<UpdateEventMessage>().await?)
                }
                "event_get" => self.work_messages(sub.get_messages::<GetEventMessage>().await?),
                "events" => self.work_messages(sub.get_messages::<GetEventsMessage>().await?),
                "categories" => self.work_messages(sub.get_messages::<GetCategoriesMessage>().await?),
                "category_create" => {
                    self.work_messages(sub.get_messages::<CreateCategoryMessage>().await?)
                }
                "category_delete" => {
                    self.work_messages(sub.get_messages::<DeleteCategoryMessage>().await?)
                }
                "category_merge" => {
                    self.work_messages(sub.get_messages::<MergeCategoriesMessage>().await?)
                }
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "Topic not found",
                    )))
                }
            };
            all_statuses.append(&mut statuses);
            self.acknowledge(ids, sub).await;
        }
        Ok(all_statuses)
    }
}
