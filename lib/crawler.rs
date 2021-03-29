use http::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::{
    queries::metadata_query, reqwest_layer::Response, reqwest_warc::write_warc,
};
use anyhow::{Context, Result};
use std::{any::Any, time::Instant};
use std::{convert::TryFrom, string::ToString};
use url::Url;

async fn make_request(
    req_url: Url,
    headers: Option<HeaderMap<HeaderValue>>,
    warc_write: bool,
) -> Result<Response> {
    let start = Instant::now();
    let client = Client::new();
    let req = client
        .get(req_url.clone())
        .headers(headers.unwrap_or_default())
        .build()?;
    let resp = client
        .execute(req.try_clone().context("Failed to clone request")?)
        .await?;
    let duration = start.elapsed();

    let res = Response::parse(resp).await?;

    if warc_write {
        // TODO: make more performant by removing clone
        write_warc(req, res.clone()).await?;
    }

    println!("Req {:?} duration {:?}, write dur", req_url, duration);
    Ok(res)
}
// pub struct Agent {}
// impl Agent {}

trait Stage {
    fn name(&self) -> String;
    /// Get the URL relative to the base URL to request
    /// `prior` is any prior stage data relevant to this one, serialized as JSON
    fn get_url(&self, prior: String) -> Result<url::Url>;

    fn extract_relevant(&self, res: Response) -> Result<url::Url>;
}

struct DataflowStage {}

impl Stage for DataflowStage {
    fn get_url(&self, prior: String) -> Result<Url> {
        let paths = ["dataflow", "all", "all", "latest"].to_vec();
        Url::try_from(metadata_query(paths).as_str())
            .context("Failed to parse URL")
    }

    fn extract_relevant(&self, res: Response) -> Result<Url> {
        todo!()
    }

    fn name(&self) -> String {
        "dataflow".to_string()
    }
}

pub struct Crawler {
    name: String,
    version: String,
    user_agent: String,
    write_warc: bool,
    stages: Vec<Box<dyn Stage>>,
}

impl Default for Crawler {
    fn default() -> Self {
        let name = "sdmxblaze".to_string();
        let version = "1.0".to_string();
        Crawler {
            name: name.clone(),
            version: version.clone(),
            // Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)
            // Mozilla/5.0 (compatible; {}/{})
            user_agent: format!(
                "Mozilla/5.0 (compatible; {}/{})",
                name, version
            ),
            write_warc: true,
            stages: vec![],
        }
    }
}

impl Crawler {
    pub async fn crawl(&self, endpoint: String) -> Result<()> {
        println!("Starting {} crawler version {}", self.name, self.version);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::util::get_source;

    use super::*;
}
