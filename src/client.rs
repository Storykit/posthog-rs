use std::time::Duration;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as HttpClient, Response};
use serde::{Deserialize, Serialize};

use crate::config::{API_ENDPOINT, DEFAULT_TIMEOUT};
use crate::errors::Error;
use crate::types::APIResult;

/// Client option and configurations builder
pub struct ClientOptionsBuilder {
    api_endpoint: String,
    api_key: Option<String>,
    timeout: Duration,
}

impl ClientOptionsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// API endpoint
    pub fn set_endpoint(mut self, api_endpoint: String) -> Self {
        self.api_endpoint = api_endpoint;
        self
    }

    /// API key - Note that this does not differentiate between public and private API and requests
    /// with a public API key towards the Private API will fail
    pub fn set_api_key(mut self, api_key: String) -> Self {
        self.api_key = Some(api_key);
        self
    }

    /// Timeout of all requests
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn build(self) -> Result<ClientOptions, Error> {
        if let Some(api_key) = self.api_key {
            Ok(ClientOptions {
                api_endpoint: self.api_endpoint,
                api_key,
                timeout: self.timeout,
            })
        } else {
            Err(Error::ClientOptionConfigError("Missing API key!".into()))
        }
    }
}

impl Default for ClientOptionsBuilder {
    fn default() -> Self {
        Self {
            api_endpoint: API_ENDPOINT.to_string(),
            api_key: None,
            timeout: DEFAULT_TIMEOUT,
        }
    }
}

#[derive(Clone)]
pub struct ClientOptions {
    pub api_endpoint: String,
    api_key: String,
    pub timeout: Duration,
}

pub struct PrivateClient {}
pub struct PublicClient {}

/// Default client towards API
pub struct Client<State = PublicClient> {
    options: ClientOptions,
    client: HttpClient,
    state: std::marker::PhantomData<State>,
}

impl Client<PrivateClient> {
    /// Run get request towards API
    pub(crate) async fn get_request(&self, endpoint: String) -> APIResult<Response> {
        self.client
            .get(self.full_url(endpoint))
            .header(CONTENT_TYPE, "application/json")
            .header(AUTHORIZATION, format!("Bearer {}", self.options.api_key))
            .send()
            .await
            .map_err(|e| Error::Connection(e.to_string()))
    }

    pub fn public(&self) -> Client<PublicClient> {
        Client {
            options: self.options.clone(),
            client: self.client.clone(),
            state: std::marker::PhantomData,
        }
    }
}

impl Client<PublicClient> {
    /// Run post request towards API
    pub(crate) async fn post_request_with_body<B>(
        &self,
        endpoint: String,
        body: B,
    ) -> APIResult<Response>
    where
        B: Sized + serde::Serialize,
    {
        let body = Body {
            inner: body,
            api_key: self.options.api_key.clone(),
        };
        self.client
            .post(self.full_url(endpoint))
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&body).map_err(|e| Error::Serialization(e.to_string()))?)
            .send()
            .await
            .map_err(|e| Error::Connection(e.to_string()))
    }

    pub fn private(&self) -> Client<PrivateClient> {
        Client {
            options: self.options.clone(),
            client: self.client.clone(),
            state: std::marker::PhantomData,
        }
    }
}

impl<State> Client<State> {
    /// Combine url endpoint with API base url
    fn full_url(&self, endpoint: String) -> String {
        format!("{}{endpoint}", self.options.api_endpoint.clone())
    }
}

impl Client {
    /// Create new API Client
    pub fn new(options: ClientOptions) -> Self {
        let client = HttpClient::builder()
            .timeout(options.timeout)
            .build()
            .expect("Failed to create underlying HTTPClient"); // Unwrap here is as safe as `HttpClient::new`
        Self {
            options,
            client,
            state: std::marker::PhantomData,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Body<T: Serialize> {
    api_key: String,
    #[serde(flatten)]
    inner: T,
}
