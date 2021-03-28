use reqwest::Client;
use warc::{RawRecord, Record, RecordType, WarcWriter};

use crate::queries::metadata_query;
use anyhow::Result;
use chrono::Utc;
use std::{collections::HashMap, time::Instant};
use std::{convert::TryFrom, string::ToString};
use std::{fs::OpenOptions, path};
use ulid::Ulid;
use url::Url;

pub async fn write_response(res: reqwest::Response) -> Result<()> {
    // Serialize response
    let heads = res.headers();

    // Does this header serialization method support multi-line headers
    let headers: Vec<String> = heads
        .into_iter()
        .map(|(k, v)| vec![k.as_str(), v.to_str().unwrap()].join(": "))
        .collect();

    let out_headers = headers.join("\r\n");
    let response_serialized = res.text().await?;
    println!("{:#?}", response_serialized);
    let fin = vec![out_headers, response_serialized].join("\r\n\r\n");
    let body = fin.as_bytes();

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
    record.set_warc_type(RecordType::Response);
    record.replace_body(body);

    // {
    //     raw: RawRecord {
    //         version: "WARC/1.1".to_string(),
    //         headers: HashMap::new(),
    //         body: vec![],
    //     },
    //     record_date: Utc::now(),
    //     record_id: id,
    //     record_type: RecordType::Response,
    //     truncated_type: None,
    // };

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
