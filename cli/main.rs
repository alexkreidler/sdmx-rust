use anyhow::{anyhow, Result};
// use std::collections::HashMap;

use sdmxblaze::{
    queries::metadata_query,
    sdmx_sources::{Accept, Sources},
};
use std::fs;

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

    for ref mut source in out_sources {
        // DATA QUERIES
        let base_url = Url::parse(format!("{}/", &source.url).as_str())?;

        let paths = ["dataflow", "all", "all", "latest"].to_vec();

        // Joining with a base url replaces the last item in path (public)
        let mut path_url = base_url.join(metadata_query(paths).as_str())?;

        let url = path_url.to_string();

        println!("{:#?}", url);

        let accepts = vec![
            "application/xml",
            "application/vnd.sdmx.structure+xml;version=2.1",
            "application/vnd.sdmx.structure+json;version=1.0.0",
            // "application/vnd.sdmx.genericmetadata+xml;version=2.1",
            // "application/vnd.sdmx.genericdata+xml;version=2.1",
            // "application/vnd.sdmx.data+json;version=1.0.0",
            // "application/vnd.sdmx.data+csv;version=1.0.0",
        ];

        for accept in accepts {
            let resp = reqwest::Client::new()
                .get(&url)
                .header("Accept", accept)
                .send()
                .await?;

            println!("{:#?}", resp);

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

            let mut out = resp.text().await?;
            // out.truncate(200);
            println!("{}", out);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn modification_sources() -> Result<()> {
        let sources = fs::read_to_string("./sources.json")?;

        let res: Sources = serde_json::from_str(&sources)?;

        let mut out_sources = res;

        let ref mut source = out_sources[1];

        println!("{:#?}", source);

        let accept = "application/vnd.sdmx.structure+xml;version=2.1";
        let mvr = StatusCode::OK;

        if source.structural_accept.as_mut().is_none() {
            source.structural_accept = Some(Accept {
                supported_accept_headers: Vec::new(),
                denied_accept_headers: Vec::new(),
            })
        }
        match mvr {
            StatusCode::OK => source
                .structural_accept
                .as_mut()
                // I was worried this unwrap might do something weird here because
                // it is described as "consuming the value" but its fine
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

        println!("{:#?}", source);

        Ok(())
    }
}
