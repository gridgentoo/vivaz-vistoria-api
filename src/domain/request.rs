use serde::{Serialize, Deserialize};
use std::collections::{BTreeMap, BTreeSet};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message{


    #[serde(rename = "List")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_list: Option<Vec<String>>,

    #[serde(skip_serializing)]
    pub(crate) ttl: usize
}