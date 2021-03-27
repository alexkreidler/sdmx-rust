use anyhow::{anyhow, Result};
// use std::collections::HashMap;

use sdmxblaze::{queries::metadata_query, sdmx_sources::Sources};
use std::fs;

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

    let unicef = res
        .into_iter()
        .find(|s| s.id == "UNICEF")
        .ok_or(anyhow!("No UNICEF"))?;
    println!("UNICEF URL {:#?}", unicef);
    // let () = unicef;

    // DATA QUERIES
    let base_url = Url::parse(format!("{}/", &unicef.url).as_str())?;

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

        let mut out = resp.text().await?;
        // out.truncate(200);
        println!("{}", out);
    }
    // base_url.join(paths.join("/").as_str())?;

    // path_url.set_query(Some(
    //     querystring::stringify(vec![
    //         ("format", "sdmx-2.1"),
    //         ("references", "none"),
    //     ])
    //     .as_str(),
    // ));

    Ok(())
}
