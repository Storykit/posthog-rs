use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Decide {
    pub distinct_id: String,
}

impl Decide {
    pub fn new(distinct_id: &str) -> Self {
        Self {
            distinct_id: distinct_id.into(),
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DecideResponse {
    pub(crate) feature_flags: HashMap<String, bool>,
}
