use http::{HeaderMap, HeaderValue};
use reqwest::Client;

use crate::{
    queries::metadata_query, reqwest_layer::Response, reqwest_warc::write_warc,
};
use anyhow::{anyhow, Context, Result};
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

    fn extract_relevant(&self, res: Response) -> Result<String>;
}

struct DataflowStage {}

impl Stage for DataflowStage {
    fn get_url(&self, prior: String) -> Result<Url> {
        let paths = ["dataflow", "all", "all", "latest"].to_vec();
        Url::try_from(metadata_query(paths).as_str())
            .context("Failed to parse URL")
    }

    fn extract_relevant(&self, res: Response) -> Result<String> {
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
    warc_write: bool,

    // base_url: Option<String>,
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
            warc_write: true,
            // base_url: None,
            stages: vec![],
        }
    }
}

impl Crawler {
    pub async fn crawl(&self, endpoint: String) -> Result<()> {
        let base_url = Url::parse(
            endpoint.as_str()
            // self.base_url
            //     .clone()
            //     .ok_or_else(|| {
            //         anyhow!(
            //             "You must provide a base URL before starting to crawl"
            //         )
            //     })?
            //     .as_str(),
        )?;
        println!("Starting {} crawler version {}", self.name, self.version);

        let mut prior_data = "".to_string();
        for stage in &self.stages {
            println!("Starting stage {}", stage.name());

            let relative_url = stage.get_url(prior_data.clone())?;

            let req_url = base_url.join(relative_url.as_str())?;

            let res = make_request(
                req_url,
                {
                    let mut hm = HeaderMap::new();
                    hm.insert("Accept", "application/json".parse().unwrap());
                    hm.insert(
                        "User-Agent",
                        (&self.user_agent).parse().unwrap(),
                    );
                    Some(hm)
                },
                self.warc_write,
            )
            .await?;

            prior_data = stage.extract_relevant(res)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use crate::util::get_source;

    use super::*;
}
