use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct Decide {
    pub distinct_id: String,
}

impl Decide {
    pub fn new<S: Into<String>>(distinct_id: S) -> Self {
        Self {
            distinct_id: distinct_id.into(),
        }
    }
}

#[derive(serde::Serialize)]
pub(crate) struct InnerDecide {
    api_key: String,
    distinct_id: String,
}

impl InnerDecide {
    pub(crate) fn new(decide: Decide, api_key: String) -> Self {
        Self {
            api_key,
            distinct_id: decide.distinct_id,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct DecideResponse {
    pub(crate) feature_flags: HashMap<String, bool>,
}
