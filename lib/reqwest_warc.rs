//! reqwest_warc handles serializing reqwest's Request and Response types to WARC files using the warc library

use http::HeaderMap;
use std::{fs::OpenOptions, path};
use ulid::Ulid;
use util::cdx_url_canonical;
use warc::{Record, RecordType, WarcWriter};

use crate::util;
use anyhow::{anyhow, Result};

pub fn serialize_headers(headers: &HeaderMap) -> String {
    // Does this header serialization method support multi-line headers
    let each: Vec<String> = headers
        .into_iter()
        .map(|(k, v)| vec![k.as_str(), v.to_str().unwrap()].join(": "))
        .collect();

    each.join("\r\n")
}

// TODO: rather than a tuple with just the record and URL, maybe preserve other metadata
// Maybe return own request
pub async fn crate_warc_request(req: reqwest::Request) -> Result<Record> {
    let empty: [u8; 0] = [];
    let bd = match req.body() {
        Some(b) => b
            .as_bytes()
            .ok_or(anyhow!("Could not convert request body to bytes"))?,
        None => &empty,
    };

    let headers = req.headers().clone();

    Ok(create_warc(RecordType::Request, &headers, bd)?)
}

pub async fn crate_warc_response(res: reqwest::Response) -> Result<Record> {
    let headers = res.headers().clone();

    let body = res.text().await?;
    let bd = body.as_bytes();

    Ok(create_warc(RecordType::Response, &headers, bd)?)
}

fn create_warc(
    typ: RecordType,
    headers: &HeaderMap,
    body: &[u8],
) -> Result<Record> {
    let out_headers = serialize_headers(headers);

    // https://users.rust-lang.org/t/what-is-right-ways-to-concat-strings/3780/3
    let fin = out_headers + "\r\n\r\n";
    let before = fin.as_bytes();
    // https://stackoverflow.com/questions/40792801/best-way-to-concatenate-vectors-in-rust
    let warc_body = [before, body].concat();

    // Record ID
    let id = Ulid::new().to_string();

    let mut record = Record::default();
    record.set_warc_version("1.1");
    record.set_warc_id(id);
    record.set_warc_type(typ);
    record.replace_body(warc_body);

    Ok(record)
}

pub fn write_warc_file(url: url::Url, records: Vec<Record>) -> Result<()> {
    // WARC File ID
    let id = Ulid::new().to_string();

    // Write WARC File
    let dir = "./warc-out";
    std::fs::create_dir_all(dir)?;

    let fname = vec![
        cdx_url_canonical(url)? + "-" + id.as_str(),
        "warc".to_string(),
    ]
    .join(".");
    let file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path::Path::new(dir).join(fname))?;
    let mut writer = WarcWriter::new(file);

    for record in records {
        writer.write(&record)?;
    }
    Ok(())
}

pub async fn write_warc(
    req: reqwest::Request,
    res: reqwest::Response,
) -> Result<()> {
    if req.url() != res.url() {
        return Err(anyhow!("URLs on request and response are not equal"));
    }
    let req_url = req.url().clone();
    let records = vec![
        crate_warc_request(req).await?,
        crate_warc_response(res).await?,
    ];
    write_warc_file(req_url, records)
}
