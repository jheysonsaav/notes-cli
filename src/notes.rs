use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    id: Uuid,
    datetime: DateTime<Utc>,
    title: String,
    topics: Vec<String>,
    body: String,
}

#[allow(dead_code)]
impl Note {
    pub fn new(title: &str, topics: Vec<&str>, body: &str) -> Self {
        let mut final_topics = vec![];

        for topic in topics {
            final_topics.push(String::from(topic));
        }

        Self {
            id: Uuid::new_v4(),
            datetime: Utc::now(),
            title: String::from(title),
            topics: final_topics,
            body: String::from(body),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn get_datetime(&self) -> DateTime<Utc> {
        self.datetime.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_topics(&self) -> Vec<String> {
        self.topics.clone()
    }

    pub fn get_body(&self) -> String {
        self.body.clone()
    }

    pub fn set_id(&mut self, id: &str) -> Self {
        self.id = Uuid::parse_str(id).unwrap();
        self.clone()
    }

    pub fn set_datetime(&mut self, datetime: &str) -> Self {
        self.datetime = datetime.parse().unwrap();
        self.clone()
    }

    pub fn set_title(&mut self, title: &str) -> Self {
        self.title = String::from(title);
        self.clone()
    }

    pub fn set_topics(&mut self, topics: Vec<&str>) -> Self {
        self.topics = topics.iter().map(|x| x.to_string()).collect();
        self.clone()
    }

    pub fn set_body(&mut self, body: &str) -> Self {
        self.body = String::from(body);
        self.clone()
    }
}
