use http::HeaderMap;
use reqwest::{Body, Client};
use warc::{RawRecord, Record, RecordType, WarcWriter};

use crate::queries::metadata_query;
use anyhow::{anyhow, Result};
use chrono::Utc;
use std::{collections::HashMap, time::Instant};
use std::{convert::TryFrom, string::ToString};
use std::{fs::OpenOptions, path};
use ulid::Ulid;
use url::Url;

fn serialize_headers(headers: &HeaderMap) -> String {
    // Does this header serialization method support multi-line headers
    let each: Vec<String> = headers
        .into_iter()
        .map(|(k, v)| vec![k.as_str(), v.to_str().unwrap()].join(": "))
        .collect();

    each.join("\r\n")
}

pub async fn write_request(req: reqwest::Request) -> anyhow::Result<()> {
    let bd = req.body().ok_or(anyhow!("No body"))?;

    bd.as_bytes();

    Ok(())
}

pub async fn write_response(res: reqwest::Response) -> Result<()> {
    // Serialize response
    let headers = res.headers().clone();
    // let response_serialized = res.text().await?;
    // let bd = res.body().ok_or_else(|| Err(anyhow!("No body")))?;

    let body = res.text().await?;
    let bd = body.as_bytes();

    write_warc(RecordType::Response, &headers, bd).await?;

    Ok(())
}

pub async fn write_warc(
    typ: RecordType,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<()> {
    let out_headers = serialize_headers(headers);

    // https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/3
    let fin = out_headers + "\r\n\r\n";
    let before = fin.as_bytes();
    // https://stackoverflow.com/questions/40792801/best-way-to-concatenate-vectors-in-rust
    let warc_body = [before, body].concat();

    // Write WARC File
    let dir = "./warc-out";
    std::fs::create_dir_all(dir)?;

    let id = Ulid::new().to_string();
    let fname = vec![id.clone(), "warc".to_string()].join(".");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path::Path::new(dir).join(fname))?;
    let mut writer = WarcWriter::new(file);

    let mut record = Record::default();
    record.set_warc_version("1.1");
    record.set_warc_id(id);
    record.set_warc_type(typ);
    record.replace_body(warc_body);

    writer.write(&record)?;

    Ok(())
}

pub async fn get_dataflows<T: ToString>(endpoint: T) -> Result<()> {
    let TESTING_APIS: Vec<&str> = vec!["IMF", "WB", "WB_WDI"];

    let paths = ["dataflow", "all", "all", "latest"].to_vec();

    let base_url = endpoint.to_string();

    // Joining with a base url replaces the last item in path (public)
    let path_url = Url::try_from(base_url.as_str())?
        .join(metadata_query(paths).as_str())?;

    let url = path_url.to_string();

    let start = Instant::now();
    let resp = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await?;

    write_response(resp).await?;

    let duration = start.elapsed();

    println!("Req {}, {:?}", base_url, duration);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn get_random_dataflow() -> Result<()> {
        get_dataflows("https://google.com/").await?;
        Ok(())
    }
}
