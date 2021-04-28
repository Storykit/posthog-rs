use std::collections::HashMap;
use chrono::{NaiveDateTime};
use reqwest::blocking::Client as HttpClient;
use reqwest::header::CONTENT_TYPE;
use serde::{Serialize};


extern crate serde_json;

const API_ENDPOINT: &str = "https://app.posthog.com/capture/";

pub fn client<C: Into<ClientOptions>>(options: C) -> Client {
    Client {
        options: options.into(),
        client: HttpClient::new(),
    }
}

#[derive(Debug)]
pub enum Error {
    Connection(String)
}

pub struct ClientOptions {
    api_endpoint: String,
    api_key: String,
}

impl From<&str> for ClientOptions {
    fn from(api_key: &str) -> Self {
        ClientOptions {
            api_endpoint: API_ENDPOINT.to_string(),
            api_key: api_key.to_string(),
        }
    }
}

pub struct Client {
    options: ClientOptions,
    client: HttpClient,
}

impl Client {
    pub fn capture(&self, event: Event) -> Result<(), Error> {
        let inner_event = InnerEvent::new(event, self.options.api_key.clone());
        let _res = self.client.post(self.options.api_endpoint.clone())
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&inner_event).expect("unwrap here is safe"))
            .send()
            .map_err(|e| Error::Connection(e.to_string()))?;
        Ok(())
    }

    pub fn capture_batch(&self, events: Vec<Event>) -> Result<(), Error> {
        for event in events {
            self.capture(event)?;
        }
        Ok(())
    }
}

// This exists so that the client doesn't have to specify the API key over and over
#[derive(Serialize)]
struct InnerEvent {
    api_key: String,
    event: String,
    properties: Properties,
    //#[serde(serialize_with = "serialize_timestamp", skip_serializing_if = "Option::is_none")]
    timestamp: Option<NaiveDateTime>,
}

impl InnerEvent {
    fn new(event: Event, api_key: String) -> Self {
        Self {
            api_key,
            event: event.event,
            properties: event.properties,
            timestamp: event.timestamp,
        }
    }
}


pub struct Event {
    event: String,
    properties: Properties,
    timestamp: Option<NaiveDateTime>,
}

#[derive(Serialize)]
pub struct Properties {
    distinct_id: String,
    props: HashMap<String, String>,
}


#[cfg(test)]
pub mod tests {
    use super::*;
    use chrono::{Utc};

    #[test]
    fn get_client() {
        let client = crate::client(env!("POSTHOG_API_KEY"));

        let mut props = HashMap::new();
        props.insert("key1".to_string(), "value1".to_string());
        props.insert("key2".to_string(), "value2".to_string());

        let event = Event {
            event: "test".to_string(),
            properties: Properties { distinct_id: "1234".to_string(), props },
            timestamp: Some(Utc::now().naive_utc()),
        };

        let res = client.capture(event).unwrap();
    }
}