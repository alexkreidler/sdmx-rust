//! reqwest_warc handles serializing reqwest's Request and Response types to WARC files using the warc library

use http::HeaderMap;
use std::{fs::OpenOptions, path};
use ulid::Ulid;
use warc::{Record, RecordType, WarcWriter};

use anyhow::{anyhow, Result};

pub fn serialize_headers(headers: &HeaderMap) -> String {
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
