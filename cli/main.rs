use anyhow::{anyhow, Context, Result};
// use std::collections::HashMap;

use reqwest::Client;
use sdmxblaze::{
    queries::metadata_query,
    reqwest_warc::write_warc,
    sdmx_sources::{Accept, Source, Sources},
};
use std::{fs, time::Instant};

use http::StatusCode;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    // Ok(())
    let sources = fs::read_to_string("./sources.json")?;

    let res: Sources = serde_json::from_str(&sources)?;

    let mut out_sources = res;

    async fn handle_source(source: &mut Source) -> Result<&mut Source> {
        // DATA QUERIES
        let base_url = Url::parse(format!("{}/", &source.url).as_str())?;

        let paths = ["dataflow", "all", "all", "latest"].to_vec();

        // Joining with a base url replaces the last item in path (public)
        let mut path_url = base_url.join(metadata_query(paths).as_str())?;

        let url = path_url.to_string();

        println!("{:#?}", url);

        let accepts = vec![
            "application/xml",
            "application/json",
            "application/vnd.sdmx.structure+xml;version=2.1",
            "application/vnd.sdmx.structure+json;version=1.0.0",
            // "application/vnd.sdmx.genericmetadata+xml;version=2.1",
            // "application/vnd.sdmx.genericdata+xml;version=2.1",
            // "application/vnd.sdmx.data+json;version=1.0.0",
            // "application/vnd.sdmx.data+csv;version=1.0.0",
        ];

        let mut a = 0;
        for accept in accepts {
            a += 1;

            let start = Instant::now();
            let client = Client::new();
            let req = client
                .get(&url)
                .header("Accept", "application/json")
                .build()?;
            let resp = client
                .execute(req.try_clone().context("Failed to clone request")?)
                .await?;

            let duration = start.elapsed();

            // println!("{:#?}", resp);

            if source.structural_accept.as_mut().is_none() {
                source.structural_accept = Some(Accept {
                    supported_accept_headers: Vec::new(),
                    denied_accept_headers: Vec::new(),
                })
            }

            match resp.status() {
                StatusCode::OK => source
                    .structural_accept
                    .as_mut()
                    .unwrap()
                    .supported_accept_headers
                    .push(accept.into()),
                _ => source
                    .structural_accept
                    .as_mut()
                    .unwrap()
                    .denied_accept_headers
                    .push(accept.into()),
            }

            source.response_content_types.push(
                match resp.headers().get("Content-Type") {
                    Some(c) => c.to_str()?.to_string(),
                    None => "NONE".to_string(),
                },
            );

            let wr_start = Instant::now();
            write_warc(req, resp).await?;
            let wr_duration = wr_start.elapsed();

            println!("Req {}, {:?}, write dur {:?}", a, duration, wr_duration);

            // let mut out = resp.text().await?;
            // out.truncate(200);
            // println!("{}", out);
        }

        println!("{:#?}", source);
        Ok(source)
    }
    let test_sources: Vec<String> =
        ["IMF".to_string(), "WB".to_string(), "UNSD".to_string()].to_vec();

    let output: Vec<_> = out_sources
        .iter_mut()
        // .filter(|source| test_sources.contains(&source.id))
        .map(|source| handle_source(source))
        .collect();

    let mut next = Vec::new();
    for fut in output {
        let n = fut.await; //?;
        if n.is_ok() {
            next.push(n.unwrap())
        } else {
            println!("{:#?}", n);
        }
    }

    // let next = &output?;
    let fin = serde_json::to_string(&next)?;

    fs::write("./out.json", fin)?;

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn modification_sources() -> Result<()> {
        Ok(())
    }
}
