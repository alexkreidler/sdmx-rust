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

pub fn cdx_url_canonical(u: url::Url) -> Result<String> {
    let mut domains: Vec<_> = u
        .host_str()
        .ok_or(anyhow!("No host string for url {:?}", u))?
        .split(".")
        .collect();
    domains.reverse();
    println!("{:#?}", domains);
    // TODO: how to deal with query params that are important for the page?
    // Not specified in any CDX specs, probably need to look at example Wayback CDX repsponses
    let out = domains.join(",") + ")"; // + u.path();
                                       // Path was causing huge issues, can't really have files with / in them

    println!("{:#?}", out);
    Ok(out)
}
