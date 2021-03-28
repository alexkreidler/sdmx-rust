use std::{fs, ops::Deref};

use serde_json::from_str;

use crate::sdmx_sources::{Source, Sources};

use anyhow::{anyhow, Result};

pub fn get_source<T: ToString>(sourceID: T) -> Result<Source> {
    let sources = fs::read_to_string("./sources.json")?;

    let res: Sources = from_str(&sources)?;

    let out: Vec<_> = res
        .into_iter()
        .filter(|s| s.id == sourceID.to_string())
        .collect();

    match out.len() {
        1 => Ok(out[0].clone()),
        _ => Err(anyhow!("No source found for ID {}", sourceID.to_string())),
    }
}
