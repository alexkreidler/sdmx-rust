use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dataflow {
    pub resource_id: String,
    pub agency_id: String,
    pub name: String,
    pub version: Option<String>,
}
