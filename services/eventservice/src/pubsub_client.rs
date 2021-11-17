use super::pstypes::*;
use crate::data_manager::EventManager;
use cloud_pubsub::*;
use futures;
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

    pub async fn handle_messages(&self) -> Result<Vec<Status>, Box<dyn std::error::Error>> {
        let mut all_statuses = Vec::new();
        let mut futures = Vec::new();
        let mut ackns = Vec::new();
        let mut names = Vec::new();
        for (name, sub) in &self.subs {
            futures.push(sub.get_messages::<Message>());
            names.push(name.clone());
        }
        for (i, name) in futures::future::join_all(futures)
            .await
            .into_iter()
            .filter_map(|f| f.ok())
            .zip(names)
        {
            let ids: Vec<String> = i.iter().map(|v| v.1.clone()).collect();
            ackns.push(self.acknowledge(ids, self.subs.get(&name).unwrap()));
            let values: Vec<Message> = i.into_iter().filter_map(|v| v.0.ok()).collect();
            if values.is_empty() {
                continue;
            }
            let mut r = match name.as_str() {
                "event_create" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<CreateEventMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "event_delete" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<DeleteEventMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "event_update" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<UpdateEventMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "event_get" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<GetEventMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "events" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<GetEventsMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "categories" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<GetCategoriesMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "category_create" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<CreateCategoryMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "category_delete" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<DeleteCategoryMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                "category_merge" => Ok(values
                    .iter()
                    .filter_map(|v| serde_json::from_str::<MergeCategoriesMessage>(&v.data).ok())
                    .map(|v| v.handle(&self.manager))
                    .collect::<Vec<Status>>()),
                _ => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Topic not found",
                ))),
            }?;
            all_statuses.append(&mut r);
        }
        futures::future::join_all(ackns).await;
        Ok(all_statuses)
    }
}
