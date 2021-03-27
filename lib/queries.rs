// use anyhow::{anyhow, Result};
// use serde::{Deserialize, Serialize};
use std::string::ToString;

/// Should follow this precise order: resource, agencyID, resourceID, version, itemID
/// The function does not currently check the order or the correctness of the values provided,
/// so it is up to the user not to create incorrect queries
pub fn metadata_query<T: ToString>(data: Vec<T>) -> String {
    data.into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("/")
}
