use std::{fs, ops::Deref, path::Path};

use serde_json::from_str;

use crate::sdmx_sources::{Source, Sources};

use anyhow::{anyhow, Result};

/// Reads a JSON file of sources into memory
pub fn read_sources<P: AsRef<Path>>(location: P) -> Result<Sources> {
    let sources = fs::read_to_string(location)?;
    let res: Sources = from_str(&sources)?;
    Ok(res)
}

/// Filters the provided sources by sourceIDs
pub fn filter_sources(
    sources: Sources,
    sourceIDs: Vec<String>,
) -> Result<Sources> {
    let out: Vec<_> = sources
        .into_iter()
        .filter(|s| sourceIDs.contains(&s.id))
        .collect();
    Ok(out)
}

/// Gets a single source from list of sources on disk
pub fn get_source(location: String, sourceID: String) -> Result<Source> {
    let out = filter_sources(read_sources(location)?, vec![sourceID.clone()])?;
    match out.len() {
        1 => Ok(out[0].clone()),
        _ => Err(anyhow!("No source found for ID {}", sourceID)),
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
