use std::time::Duration;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::{Client as HttpClient, Response};
use serde::{Deserialize, Serialize};

use crate::config::{API_ENDPOINT, DEFAULT_TIMEOUT};
use crate::errors::Error;
use crate::types::APIResult;

pub struct PublicBuilder;
pub struct PrivateBuilder;
pub struct NoKey;

/// Client option and configurations builder
pub struct ClientBuilder<State = NoKey> {
    api_endpoint: String,
    private_api_key: Option<String>,
    public_api_key: Option<String>,
    timeout: Duration,
    state: std::marker::PhantomData<State>,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<State> ClientBuilder<State> {
    /// API endpoint
    pub fn set_endpoint(mut self, api_endpoint: String) -> Self {
        self.api_endpoint = api_endpoint;
        self
    }

    /// Timeout of all requests
    pub fn set_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}

// As long as no key has been set, we can set either a public or private key, but once a key has
// been set we can't put in another type of key
impl ClientBuilder<NoKey> {
    pub fn set_private_api_key(self, api_key: String) -> ClientBuilder<PrivateBuilder> {
        ClientBuilder {
            api_endpoint: self.api_endpoint,
            private_api_key: Some(api_key),
            public_api_key: self.public_api_key,
            timeout: self.timeout,
            state: std::marker::PhantomData,
        }
    }

    pub fn set_public_api_key(self, api_key: String) -> ClientBuilder<PublicBuilder> {
        ClientBuilder {
            api_endpoint: self.api_endpoint,
            private_api_key: self.private_api_key,
            public_api_key: Some(api_key),
            timeout: self.timeout,
            state: std::marker::PhantomData,
        }
    }
}

impl ClientBuilder<PublicBuilder> {
    pub fn build(self) -> Result<Client<PublicClient>, Error> {
        if let Some(public_api) = self.public_api_key {
            let options = ClientOptions {
                api_endpoint: self.api_endpoint,
                api_key: public_api,
                timeout: self.timeout,
            };
            return Ok(Client::<PublicClient>::new(options));
        } else {
            return Err(Error::ClientOptionConfigError("Missing API key!".into()));
        };
    }
}

impl ClientBuilder<PrivateBuilder> {
    pub fn build(self) -> Result<Client<PrivateClient>, Error> {
        if let Some(private_api) = self.private_api_key {
            let options = ClientOptions {
                api_endpoint: self.api_endpoint,
                api_key: private_api,
                timeout: self.timeout,
            };
            return Ok(Client::<PrivateClient>::new(options));
        } else {
            return Err(Error::ClientOptionConfigError("Missing API key!".into()));
        };
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            api_endpoint: API_ENDPOINT.to_string(),
            public_api_key: None,
            private_api_key: None,
            timeout: DEFAULT_TIMEOUT,
            state: std::marker::PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct ClientOptions {
    pub api_endpoint: String,
    api_key: String,
    pub timeout: Duration,
}

pub struct PrivateClient;
pub struct PublicClient;

/// Default client towards API
pub struct Client<State> {
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
}

impl<State> Client<State> {
    /// Combine url endpoint with API base url
    fn full_url(&self, endpoint: String) -> String {
        format!("{}{endpoint}", self.options.api_endpoint.clone())
    }

    fn new(options: ClientOptions) -> Self {
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
