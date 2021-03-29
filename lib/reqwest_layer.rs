use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// A simple response structure that mimics Reqwest
#[derive(Debug, Clone)]
pub struct Response {
    pub status: http::StatusCode,
    pub headers: http::HeaderMap,
    pub url: url::Url,
    pub body: Option<String>,
    pub version: http::Version,
}

impl From<reqwest::Response> for Response {
    /// The user must manually set the body
    fn from(r: reqwest::Response) -> Self {
        Response {
            status: r.status(),
            headers: r.headers().clone(),
            url: r.url().clone(),
            body: None,
            //r.text().await?,
            version: r.version(),
        }
    }
}

impl Response {
    /// The user must manually set the body
    pub async fn parse(r: reqwest::Response) -> Result<Self> {
        Ok(Response {
            status: r.status(),
            headers: r.headers().clone(),
            url: r.url().clone(),
            version: r.version(),
            body: Some(r.text().await?),
        })
    }
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// pub struct Response {
//     #[serde(with = "http_serde::status")]
//     status: http::StatusCode,

//     #[serde(with = "http_serde::header_map")]
//     headers: http::HeaderMap,

//     #[serde(with = "http_serde::uri")]
//     url: url::Url,
//     body: String,
//     version: http::Version,
// }
