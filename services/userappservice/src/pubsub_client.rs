use super::types::*;
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
        let topics_names = vec!["app_create", "app_update", "app_get"];
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
    #[allow(dead_code)]
    pub fn clean_db(&self) -> bool {
        self.manager.reset_all().unwrap_or(false)
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

    async fn acknowledge(&self, ids: Vec<String>, name: String) {
        match self.subs.get(&name) {
            Some(s) => s.acknowledge_messages(ids).await,
            None => eprintln!("Failed to find sub for acknowledgement!"),
        }
    }

    fn parse_message_type(
        &self,
        messages: Vec<Message>,
        name: &String,
    ) -> Result<Vec<Box<dyn PubSubCallBack>>, std::io::Error> {
        match name.as_str() {
            "app_create" => Ok(messages
                .iter()
                .filter_map(|v| serde_json::from_str::<CreateAppMessage>(&v.data).ok())
                .map(|v| Box::new(v) as Box<dyn PubSubCallBack>)
                .collect()),
            "app_update" => Ok(messages
                .iter()
                .filter_map(|v| serde_json::from_str::<UpdateStatusMessage>(&v.data).ok())
                .map(|v| Box::new(v) as Box<dyn PubSubCallBack>)
                .collect()),
            "app_get" => Ok(messages
                .iter()
                .filter_map(|v| serde_json::from_str::<GetAppsMessage>(&v.data).ok())
                .map(|v| Box::new(v) as Box<dyn PubSubCallBack>)
                .collect()),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Topic not found",
            )),
        }
    }

    fn work_messages(
        &self,
        messages: Vec<(Result<Message, error::Error>, String)>,
        name: String,
    ) -> Result<(Vec<Status>, Vec<String>, String), cloud_pubsub::error::Error> {
        let ids: Vec<String> = messages.iter().map(|v| v.1.clone()).collect();
        let values: Vec<Message> = messages.into_iter().filter_map(|v| v.0.ok()).collect();
        let statuses: Vec<Status> = match self.parse_message_type(values, &name) {
            Ok(items) => items.iter().map(|v| v.handle(&self.manager)).collect(),
            Err(e) => return Err(cloud_pubsub::error::Error::IO(e)),
        };
        Ok((statuses, ids, name))
    }

    pub async fn handle_messages(&self, timeout : u8) -> Result<Vec<Status>, Box<dyn std::error::Error>> {
        let mut all_statuses = Vec::new();
        let mut futures = Vec::new();
        let mut ackns = Vec::new();
        for (_, sub) in &self.subs {
            futures.push(tokio::time::timeout(
                tokio::time::Duration::from_secs(timeout.into()),
                sub.get_messages::<Message>(),
            ));
        }
        let r: Vec<(Vec<(Result<Message, error::Error>, String)>, String)> =
            futures::future::join_all(futures)
                .await
                .into_iter()
                .zip(self.subs.keys())
                .filter_map(|(f, n)| match f {
                    Ok(v) => match v {
                        Ok(v) => Some((v, n.clone())),
                        Err(_) => None,
                    },
                    Err(_) => None,
                })
                .collect();
        r.into_iter()
            .filter_map(|(v, n)| self.work_messages(v, n).ok())
            .for_each(|(mut s, ids, name)| {
                all_statuses.append(&mut s);
                ackns.push(self.acknowledge(ids, name));
            });
        futures::future::join_all(ackns).await;
        Ok(all_statuses)
    }
}
