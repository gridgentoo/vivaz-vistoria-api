use serde::{Deserialize, Serialize};
use serde_json::Result;



#[derive(Clone, Debug, Serialize, Deserialize)]pub struct List {
    #[serde(rename = "List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) list: Option<Vec<String>>,
}
