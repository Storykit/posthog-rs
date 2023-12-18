use std::collections::HashMap;

use async_trait::async_trait;

use crate::client::{Client, PublicClient};
use crate::decide::{Decide, DecideResponse};
use crate::errors::Error;
use crate::event::Event;
use crate::types::APIResult;

#[async_trait]
pub trait PublicAPI {
    async fn capture(&self, event: Event) -> APIResult<()>;
    async fn capture_batch(&self, events: Vec<Event>) -> APIResult<()> {
        for event in events {
            self.capture(event).await?;
        }
        Ok(())
    }

    async fn decide(&self, user_id: &str) -> APIResult<HashMap<String, bool>>;
}

#[async_trait]
impl PublicAPI for Client<PublicClient> {
    async fn capture(&self, event: Event) -> APIResult<()> {
        let _res = self
            .post_request_with_body("/capture/".into(), event)
            .await?;
        Ok(())
    }

    async fn decide(&self, user_id: &str) -> APIResult<HashMap<String, bool>> {
        let body = Decide::new(user_id);
        let url = format!("/decide?v=3");
        let res = self.post_request_with_body(url, body).await?;
        let response: DecideResponse = res
            .json::<DecideResponse>()
            .await
            .map_err(|e| Error::Serialization(e.to_string()))?;
        Ok(response.feature_flags)
    }
}
