use anyhow::{anyhow, Context, Result};
// use std::collections::HashMap;

use reqwest::header::HeaderMap;
use sdmxblaze::sdmx_sources::{Source, Sources};
use std::{fs, path};

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

    let paths = ["dataflow", "all", "all", "latest"];

    // Joining with a base url replaces the last item in path (public)
    let mut path_url = base_url.join(paths.join("/").as_str())?;

    path_url.set_query(Some(
        querystring::stringify(vec![
            ("format", "sdmx-2.1"),
            ("references", "none"),
        ])
        .as_str(),
    ));

    let url = path_url.to_string();

    println!("{:#?}", url);

    let client = reqwest::ClientBuilder::new().build()?;
    // client.get(url).headers(
    //     HeaderMap::
    // )
    let resp = reqwest::get(url).await?;

    // println!("{:#?}", resp);

    let mut out = resp.text().await?;
    // out.truncate(200);
    println!("{}", out);

    Ok(())
}
