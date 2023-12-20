//!  Posthog API Client
//!
//!  Allows for communication with posthog API (both public and private)

mod client;
mod config;
mod decide;
pub mod errors;
mod event;
mod feature_flags;
mod properties;
mod public_api;
/// Data types related to the API
pub mod types;

pub use client::{Client, ClientBuilder, PrivateClient, PublicClient};
pub use event::Event;
pub use feature_flags::FeatureFlagsAPI;
pub use public_api::PublicAPI;
pub use types::{APIResult, *};
